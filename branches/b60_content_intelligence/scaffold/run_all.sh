#!/usr/bin/env bash
# Offline smoke test for the B60 v1 content-intelligence scaffold. Runs every subsystem
# on the deterministic mock backend (no GPU, no network, no keys) and reports pass/fail.
# Exit 0 iff all pass.
set -u
cd "$(dirname "$0")" || exit 2          # scaffold/ is the import root
PY=python3
pass=0; fail=0
run() {  # run <label> <cmd...>
  local label="$1"; shift
  if "$@" >/tmp/_b60_smoke.out 2>&1; then
    echo "  PASS  $label"; pass=$((pass+1))
  else
    echo "  FAIL  $label"; sed 's/^/        /' /tmp/_b60_smoke.out | tail -6; fail=$((fail+1))
  fi
}

echo "B60 v1 scaffold — offline smoke test"
run "foundation: common (schema+specialists)" $PY common.py
run "foundation: backends (mock)"             $PY backends.py
run "ingest: dedup (url+simhash)"             $PY ingest/dedup.py
run "ingest: build_nodes (understanding)"     $PY ingest/build_nodes.py
run "salience: scoring (4-signal+opp)"        $PY salience/scoring.py
run "salience: expander (region->niche->5)"   $PY salience/expander.py
run "link: entity_link (geo+abductive)"       $PY link/entity_link.py
run "reaction: authenticity (platt)"          $PY reaction/authenticity.py
run "reaction: hype (independent-origins)"    $PY reaction/hype.py
run "reaction: dense_brief (lossless audit)"  $PY reaction/dense_brief.py
run "creative: gen_brief (disclosure-or-die)" $PY creative/gen_brief.py
run "compliance: clearance (green-lane+token)" $PY compliance/clearance.py
run "psych: resonance (4-driver+safe-lane)"   $PY psych/resonance.py
run "orch: blackboard (single-writer+outbox)" $PY orch/blackboard.py
run "orch: controller (DAG spine -> emit)"    $PY orch/controller.py
run "eval: harness (leakage-controlled)"      $PY eval/harness.py
run "MVP: run_pipeline (a:brief + b:verdict)" $PY run_pipeline.py

echo "-----------------------------------------------"
echo "TOTAL: $pass passed, $fail failed"
[ "$fail" -eq 0 ]
