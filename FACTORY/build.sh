#!/usr/bin/env bash
# build.sh — ONE command to forge + gate a whole specialist package.
#
# This is the "easy way": you hand-author the specs (that part must stay
# model-reasoned — the gates do NOT judge prose), then run ONE command and it
# does every forge + gate step in the right order and prints a clean summary.
#
# Usage:
#   bash FACTORY/build.sh <dir>
#
# where <dir> contains files you hand-authored:
#   - one or more   *.spec.json        (compact KB content-specs)
#   - exactly one   *.specialist.json  (the specialist, grounded to the KBs)
#
# What it does, in order — and NOTHING else:
#   1. forge  each  *.spec.json  ->  *.kb.json   (math only; your prose passes through verbatim)
#   2. gate   each  *.kb.json     with  validators/kb_validator.py --mode dense
#   3. check  the specialist's grounded_in_kbs actually point at the forged KBs
#   4. gate   the   *.specialist.json  with  validators/specialist_validator.py
#
# It NEVER writes prose. If a gate fails, fix the spec BY HAND and re-run.
# Tip: make grounded_in_kbs ABSOLUTE paths so the specialist gate passes from
# any working directory.

set -u

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO="$(cd "$SCRIPT_DIR/.." && pwd)"

PY="python3"; command -v python3 >/dev/null 2>&1 || PY="python"

DIR="${1:-}"
if [ -z "$DIR" ] || [ ! -d "$DIR" ]; then
  echo "usage: bash FACTORY/build.sh <dir-with-specs>" >&2
  exit 2
fi
DIR="$(cd "$DIR" && pwd)"

FORGE="$REPO/branches/_forge/kb_forge.py"
KBGATE="$REPO/validators/kb_validator.py"
SPGATE="$REPO/validators/specialist_validator.py"

green="OK"; red="FAIL"
fail=0
forged=""   # space-separated list of forged kb.json basenames

echo "== 1) forge + gate KBs in: $DIR =="
found_spec=0
for spec in "$DIR"/*.spec.json; do
  [ -e "$spec" ] || continue
  found_spec=1
  base="$(basename "$spec" .spec.json)"
  kb="$DIR/$base.kb.json"
  echo "-- $base"
  if "$PY" "$FORGE" "$spec" -o "$kb" --quiet; then
    echo "   forge  $green"
    forged="$forged $base.kb.json"
  else
    echo "   forge  $red"; fail=1; continue
  fi
  if "$PY" "$KBGATE" "$kb" --mode dense --quiet; then
    echo "   gate   $green   $base.kb.json"
  else
    echo "   gate   $red    re-run: $PY validators/kb_validator.py \"$kb\" --mode dense"
    fail=1
  fi
done
if [ "$found_spec" -eq 0 ]; then
  echo "no *.spec.json KB specs found in $DIR" >&2
  exit 2
fi

echo "== 2) gate specialist =="
found_sp=0
for sp in "$DIR"/*.specialist.json; do
  [ -e "$sp" ] || continue
  found_sp=1
  echo "-- $(basename "$sp")"

  # grounding advisory: warn if grounded_in_kbs don't resolve or don't point at
  # the KBs we just forged (the gate only checks the paths EXIST, not that they
  # are the right / fresh KBs).
  "$PY" - "$sp" "$DIR" $forged <<'PYEOF'
import json, os, sys
sp, d = sys.argv[1], sys.argv[2]
forged = set(sys.argv[3:])
try:
    g = json.load(open(sp)).get("grounded_in_kbs", [])
except Exception as e:
    print(f"   grounding ?  could not read spec: {e}"); sys.exit(0)
for p in g:
    exists = isinstance(p, str) and (os.path.exists(p) or os.path.exists(os.path.join(d, p)))
    is_forged = os.path.basename(p) in forged if isinstance(p, str) else False
    if not exists:
        print(f"   grounding FAIL  path does not resolve: {p}")
    elif not is_forged:
        print(f"   grounding WARN  resolves but is not one of the KBs forged here: {p}")
    else:
        print(f"   grounding OK    {os.path.basename(p)}")
PYEOF

  if "$PY" "$SPGATE" "$sp" --quiet; then
    echo "   gate   $green"
  else
    echo "   gate   $red    re-run: $PY validators/specialist_validator.py \"$sp\""
    fail=1
  fi
done
if [ "$found_sp" -eq 0 ]; then
  echo "no *.specialist.json found in $DIR" >&2
  exit 2
fi

echo
if [ "$fail" -eq 0 ]; then
  echo "ALL GREEN — every KB and the specialist passed every deterministic gate."
  echo "REMINDER: gates check STRUCTURE + MATH, never prose quality."
  echo "Eyeball the node definitions for templated near-duplicates before you trust it."
else
  echo "SOME GATES FAILED — fix the spec/metric BY HAND and re-run."
  echo "Never 'fix' a gate by writing a script that generates prose."
fi
exit "$fail"
