#!/usr/bin/env bash
# tools/check_scope.sh — conflict-free-by-construction enforcement (protocol v3).
# Verifies every path changed on a ref lies inside the writing agent's allowed
# scope: the agent's static_write_scope from EXCHANGE/AGENTS.json, plus the
# write_scope of the named task in EXCHANGE/tasks.json (which must be assigned
# to that agent and in a writable state: assigned | in_progress | revise).
# bash + python3 stdlib only. Edited ONLY by the integrator.
#
# Usage:
#   tools/check_scope.sh --agent <id> [--task TASK-NNN] [--base <ref>] [--head <ref>]
# Defaults: base = origin/<integration_branch from AGENTS.json>, head = HEAD.
#
# Builders run it pre-push on their own branch; the integrator re-runs it on
# the fetched branch before merging, e.g.:
#   tools/check_scope.sh --agent grok --task TASK-005 --head origin/grok/task-005-rearm
set -u
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
AGENT="" TASK="" BASE="" HEAD="HEAD"
while [ $# -gt 0 ]; do
  case "$1" in
    --agent) AGENT="${2:?}"; shift 2 ;;
    --task)  TASK="${2:?}";  shift 2 ;;
    --base)  BASE="${2:?}";  shift 2 ;;
    --head)  HEAD="${2:?}";  shift 2 ;;
    *) echo "unknown arg: $1"; exit 2 ;;
  esac
done
[ -n "$AGENT" ] || { echo "usage: tools/check_scope.sh --agent <id> [--task TASK-NNN] [--base <ref>] [--head <ref>]"; exit 2; }
if [ -z "$BASE" ]; then
  BASE="origin/$(python3 -c 'import json; print(json.load(open("EXCHANGE/AGENTS.json"))["integration_branch"])')" || exit 2
fi
MB="$(git merge-base "$BASE" "$HEAD" 2>/dev/null)" || { echo "FAIL: no merge-base between $BASE and $HEAD"; echo "SCOPE: FAIL"; exit 1; }
CHANGED="$(git diff --name-only "$MB" "$HEAD")"
export CHANGED
python3 - "$AGENT" "$TASK" <<'PY'
import json, sys, os, fnmatch
agent_id, task_id = sys.argv[1], sys.argv[2]
A = json.load(open("EXCHANGE/AGENTS.json"))
T = json.load(open("EXCHANGE/tasks.json"))
ag = A["agents"].get(agent_id)
if ag is None: sys.exit(f"FAIL: unknown agent: {agent_id}")
allowed = list(ag.get("static_write_scope", []))
if task_id:
    t = next((t for t in T["tasks"] if t["id"] == task_id), None)
    if t is None: sys.exit(f"FAIL: unknown task: {task_id}")
    if t["agent"] != agent_id: sys.exit(f"FAIL: {task_id} is assigned to {t['agent']}, not {agent_id}")
    if t["state"] not in ("assigned", "in_progress", "revise"):
        sys.exit(f"FAIL: {task_id} is not in a writable state (state={t['state']})")
    allowed += t["write_scope"]
# fnmatch's '*' is not path-aware (crosses '/'), so '**' normalizes to '*'
pats = [p.replace("**", "*") for p in allowed]
paths = [l.strip() for l in os.environ.get("CHANGED", "").splitlines() if l.strip()]
bad = [p for p in paths if not any(fnmatch.fnmatch(p, pat) for pat in pats)]
print(f"scope check: {len(paths)} changed path(s), agent={agent_id}, task={task_id or '-'}, allowed={allowed}")
for p in bad:
    print(f"FAIL: out-of-scope write: {p}")
sys.exit(1 if bad else 0)
PY
rc=$?
if [ $rc -eq 0 ]; then echo "SCOPE: PASS"; else echo "SCOPE: FAIL"; fi
exit $rc
