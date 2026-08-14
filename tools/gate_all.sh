#!/usr/bin/env bash
# tools/gate_all.sh — the single gate entrypoint for the EXCHANGE protocol (v3).
# Every agent runs this before push; the integrator re-runs it on fetched refs.
# bash + python3 stdlib only. Edited ONLY by the integrator; its sha256 is
# printed (FPR lines) so all parties can confirm they ran identical checks.
#
# Usage:
#   tools/gate_all.sh --task <staging-dir>   # gate one task delivery (builders, pre-push)
#   tools/gate_all.sh --repo                 # full integration sweep (integrator, pre-merge + post-wire)
#   tools/gate_all.sh --fingerprint          # print environment + gate fingerprints only
#
# Exit 0 iff every gate passed; one "FAIL:" line per failure; final line is
# "GATES: PASS ..." or "GATES: FAIL ...".
GATE_VERSION="3.0"
set -u
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
FAIL=0
note() { printf '%s\n' "$*"; }
fail() { FAIL=1; printf 'FAIL: %s\n' "$*"; }

fingerprint() {
  note "gate_all version: $GATE_VERSION"
  note "python: $(python3 --version 2>&1)"
  note "HEAD: $(git rev-parse HEAD 2>/dev/null || echo no-git)"
  sha256sum tools/gate_all.sh tools/check_scope.sh branches/_forge/kb_forge.py \
    validators/kb_validator.py validators/metrics.py validators/specialist_validator.py \
    2>/dev/null | sed 's/^/FPR: /'
}

json_parse_all() { # <dir> — every .json under dir must parse
  local f
  while IFS= read -r -d '' f; do
    python3 -c 'import json,sys; json.load(open(sys.argv[1]))' "$f" 2>/dev/null \
      || fail "invalid JSON: $f"
  done < <(find "$1" -type f -name '*.json' -print0)
}

gate_task() { # <staging-dir>
  local DIR="${1%/}"
  [ -d "$DIR" ] || { fail "no such staging dir: $DIR"; return; }
  local TMP; TMP="$(mktemp -d)"
  local f kb out n_spec=0 n_kb=0 n_sp=0
  json_parse_all "$DIR"
  # gate 1: every spec re-forges byte-identically to its committed KB
  while IFS= read -r -d '' f; do
    n_spec=$((n_spec+1))
    kb="${f%.spec.json}.kb.json"
    out="$TMP/reforged.$n_spec.json"
    if ! python3 branches/_forge/kb_forge.py "$f" -o "$out" --quiet; then
      fail "forge error: $f"; continue
    fi
    if [ ! -f "$kb" ]; then fail "spec without committed KB: $f (expected $kb)"; continue; fi
    cmp -s "$out" "$kb" || fail "committed KB is not the forge output of its spec (hand-edited?): $kb"
  done < <(find "$DIR" -type f -name '*.spec.json' -print0)
  # gate 2: every KB passes dense validation
  while IFS= read -r -d '' f; do
    n_kb=$((n_kb+1))
    python3 validators/kb_validator.py "$f" --mode dense --quiet || fail "kb_validator --mode dense: $f"
  done < <(find "$DIR" -type f -name '*.kb.json' -print0)
  # gate 3: every specialist passes
  while IFS= read -r -d '' f; do
    n_sp=$((n_sp+1))
    python3 validators/specialist_validator.py "$f" --quiet || fail "specialist_validator: $f"
  done < <(find "$DIR" -type f -name '*.specialist.json' -print0)
  rm -rf "$TMP"
  [ $((n_spec + n_kb + n_sp)) -gt 0 ] || fail "staging dir has no gateable artifacts: $DIR"
  note "task gate: $n_spec specs reforged, $n_kb KBs dense-validated, $n_sp specialists validated in $DIR"
}

gate_repo() {
  local TMP; TMP="$(mktemp -d)"
  local f n=0
  # gate 1: every wired KB passes dense validation
  for f in knowledge_base/knowledge_searcher/*.kb.json; do
    python3 validators/kb_validator.py "$f" --mode dense --quiet || fail "kb_validator --mode dense: $f"
    n=$((n+1))
  done
  note "repo gate: $n wired KBs dense-validated"
  # gate 2: every wired specialist passes
  n=0
  for f in specialists/*.specialist.json; do
    python3 validators/specialist_validator.py "$f" --quiet || fail "specialist_validator: $f"
    n=$((n+1))
  done
  note "repo gate: $n wired specialists validated"
  # gate 3: drift — ROUTER and INDEX must equal a fresh deterministic rebuild
  { python3 tools/build_router.py --spec-dir specialists --out "$TMP/router.json" >/dev/null 2>&1 \
      && cmp -s "$TMP/router.json" specialists/ROUTER.json; } \
    || fail "specialists/ROUTER.json drifted from rebuild (rerun tools/build_router.py)"
  { python3 tools/build_index.py --kb-dir knowledge_base/knowledge_searcher --out "$TMP/index.json" >/dev/null 2>&1 \
      && cmp -s "$TMP/index.json" knowledge_base/knowledge_searcher/INDEX.json; } \
    || fail "knowledge_base/knowledge_searcher/INDEX.json drifted from rebuild (rerun tools/build_index.py)"
  # gate 4: ledger sanity — AGENTS.json + tasks.json consistent; active task scopes pairwise disjoint
  python3 - <<'PY' || fail "EXCHANGE ledger inconsistent (see message above)"
import json, sys
A = json.load(open("EXCHANGE/AGENTS.json"))
T = json.load(open("EXCHANGE/tasks.json"))
def die(m): print("ledger:", m); sys.exit(1)
if A["protocol_version"] != T["protocol_version"]:
    die("protocol_version mismatch between AGENTS.json and tasks.json")
states, seen = set(T["states"]), set()
ACTIVE = {"assigned", "in_progress", "revise", "delivered"}
active = []
for t in T["tasks"]:
    if t["id"] in seen: die(f"duplicate task id {t['id']}")
    seen.add(t["id"])
    if t["state"] not in states: die(f"{t['id']}: unknown state {t['state']!r}")
    if t["agent"] not in A["agents"]: die(f"{t['id']}: unknown agent {t['agent']!r}")
    if not (isinstance(t["write_scope"], list) and t["write_scope"]): die(f"{t['id']}: empty write_scope")
    if t["state"] in ACTIVE: active.append(t)
def prefixes(t):  # non-EXCHANGE scope globs reduced to dir prefixes
    return [p.rstrip("*") for p in t["write_scope"] if not p.startswith("EXCHANGE/")]
for i, a in enumerate(active):
    for b in active[i+1:]:
        for pa in prefixes(a):
            for pb in prefixes(b):
                if pa.startswith(pb) or pb.startswith(pa):
                    die(f"active tasks {a['id']} and {b['id']} have overlapping write scopes: {pa!r} vs {pb!r}")
PY
  rm -rf "$TMP"
}

MODE="${1:-}"
fingerprint
case "$MODE" in
  --fingerprint) exit 0 ;;
  --task) gate_task "${2:?usage: gate_all.sh --task <staging-dir>}" ;;
  --repo) gate_repo ;;
  *) note "usage: tools/gate_all.sh --task <staging-dir> | --repo | --fingerprint"; exit 2 ;;
esac
SELF_FPR="$(sha256sum tools/gate_all.sh | cut -c1-12)"
if [ "$FAIL" -eq 0 ]; then note "GATES: PASS gate_all=$SELF_FPR"; else note "GATES: FAIL gate_all=$SELF_FPR"; fi
exit "$FAIL"
