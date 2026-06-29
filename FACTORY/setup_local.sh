#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# setup_local.sh — clone this repo locally and self-test the specialist factory.
#
# Everything here is Python *stdlib only* — no pip install, no network at run
# time, no GPU, no API keys. If you can run `python3 --version` you can run the
# whole factory.
#
# Usage:
#   # A) you do NOT have the repo yet — clone + verify in one shot:
#   bash setup_local.sh --clone
#
#   # B) you already have the repo — run this from inside it to verify:
#   bash FACTORY/setup_local.sh
#
# Options:
#   --clone            git clone the repo first, then verify inside it
#   --repo  <url>      override the clone URL   (default: the GitHub HTTPS URL below)
#   --branch <name>    branch to check out      (default: the repo's default branch)
#   --dir   <path>     clone target directory   (default: ./automation)
# ---------------------------------------------------------------------------
set -euo pipefail

REPO_URL="https://github.com/tmundi3210/automation.git"
BRANCH=""          # empty => use the remote's default branch (where the code lives)
CLONE=0
DIR="automation"

while [ $# -gt 0 ]; do
  case "$1" in
    --clone)  CLONE=1 ;;
    --repo)   REPO_URL="$2"; shift ;;
    --branch) BRANCH="$2"; shift ;;
    --dir)    DIR="$2"; shift ;;
    *) echo "unknown arg: $1" >&2; exit 2 ;;
  esac
  shift
done

say() { printf '\n\033[1m== %s\033[0m\n' "$1"; }

# --- 0. python present? -----------------------------------------------------
say "0. checking python3 (stdlib only — nothing to pip install)"
if ! command -v python3 >/dev/null 2>&1; then
  echo "ERROR: python3 not found. Install Python 3.8+ and re-run." >&2
  exit 1
fi
python3 --version

# --- 1. get the repo --------------------------------------------------------
if [ "$CLONE" -eq 1 ]; then
  say "1. cloning $REPO_URL (branch: ${BRANCH:-<default>}) -> $DIR"
  if [ -d "$DIR/.git" ]; then
    echo "(already cloned at $DIR — pulling latest)"
    git -C "$DIR" pull
  elif [ -n "$BRANCH" ]; then
    git clone --branch "$BRANCH" "$REPO_URL" "$DIR"
  else
    git clone "$REPO_URL" "$DIR"          # checks out the remote default branch
  fi
  cd "$DIR"
else
  # find the repo root from wherever we were invoked
  ROOT="$(git rev-parse --show-toplevel 2>/dev/null || true)"
  if [ -z "$ROOT" ]; then
    echo "ERROR: not inside a git repo and --clone not given." >&2
    echo "       run:  bash setup_local.sh --clone" >&2
    exit 1
  fi
  cd "$ROOT"
  say "1. using existing checkout at $ROOT"
fi

# sanity: are the factory pieces here?
for f in validators/kb_validator.py validators/specialist_validator.py \
         branches/_forge/kb_forge.py dist/prompt_template.json; do
  [ -f "$f" ] || { echo "ERROR: expected file missing: $f (wrong repo/branch?)" >&2; exit 1; }
done

# --- 2. self-test the three gates on real artifacts -------------------------
say "2. self-test: gate one existing KB (dense mode)"
KB="branches/b60_content_intelligence/kb/geo__attention_topology.kb.json"
python3 validators/kb_validator.py "$KB" --mode dense --quiet && echo "PASS  kb gate: $KB"

say "3. self-test: gate one existing specialist"
SPEC="branches/b60_content_intelligence/salience_heavy.specialist.heavy.json"
python3 validators/specialist_validator.py "$SPEC" --quiet && echo "PASS  specialist gate: $SPEC"

say "4. self-test: rebuild a KB from its compact spec with the forge, then gate it"
SP="branches/b60_content_intelligence/kb/_src/geo__attention_topology.spec.json"
OUT="$(mktemp -d)/rebuilt.kb.json"
python3 branches/_forge/kb_forge.py "$SP" -o "$OUT" --quiet \
  && python3 validators/kb_validator.py "$OUT" --mode dense --quiet \
  && echo "PASS  forge round-trip: spec -> kb -> dense gate"
rm -f "$OUT"

# --- 5. done ----------------------------------------------------------------
say "5. ALL GREEN — the factory works on your machine."
cat <<'EOF'

Next:
  • Read   FACTORY/MAKE_A_SPECIALIST.md   (the full how-to: KB -> gate -> specialist -> use it)
  • Author a new specialist the deterministic way:
      cp FACTORY/kb_spec.template.json my_topic.spec.json     # fill in 3 of these (3 KBs)
      python3 branches/_forge/kb_forge.py my_topic.spec.json -o my_topic.kb.json
      python3 validators/kb_validator.py my_topic.kb.json --mode dense
      cp FACTORY/specialist.template.json my.specialist.json  # ground it in the 3 KBs
      python3 validators/specialist_validator.py my.specialist.json
  • Or author with an AI: paste FACTORY/GENERATOR_PROMPT.md into a capable model.
  • Use a finished specialist as a prompt: see "How to USE it" in MAKE_A_SPECIALIST.md.
EOF
