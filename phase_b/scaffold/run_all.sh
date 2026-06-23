#!/usr/bin/env bash
# Offline smoke test for the Phase B Track-2 scaffold. Runs every subsystem on the
# mock backend (no GPU, no network, no keys) and reports pass/fail. Exit 0 iff all pass.
set -u
cd "$(dirname "$0")/../.." || exit 2   # repo root
PY=python3
pass=0; fail=0
run() {  # run <label> <cmd...>
  local label="$1"; shift
  if "$@" >/tmp/_scaffold_smoke.out 2>&1; then
    echo "  PASS  $label"; pass=$((pass+1))
  else
    echo "  FAIL  $label"; sed 's/^/        /' /tmp/_scaffold_smoke.out | tail -5; fail=$((fail+1))
  fi
}

echo "Phase B Track-2 scaffold — offline smoke test"
run "foundation: common"        $PY phase_b/scaffold/common.py
run "foundation: backends"      $PY phase_b/scaffold/backends.py
run "orch: router"              $PY phase_b/scaffold/orch/router.py
run "orch: graph"               $PY phase_b/scaffold/orch/graph.py "fine-tune with LoRA, roll back on stall"
run "ingest: chunker"           $PY phase_b/scaffold/ingest/pdf_to_chunks.py --out /tmp/_smoke_chunks.jsonl
run "ingest: distiller+gate"    $PY phase_b/scaffold/ingest/chunk_to_kb.py --unit ftune__smoke
run "train: rollback loop"      $PY phase_b/scaffold/train/rollback_loop.py --quiet
run "eval: oracle (scorers)"    $PY phase_b/scaffold/eval_harness/harness.py --system oracle
run "eval: mock (system)"       $PY phase_b/scaffold/eval_harness/harness.py --system mock
run "select: per-domain map"    $PY phase_b/scaffold/select/select.py --map
run "render: dense->human"      $PY phase_b/scaffold/render/dense_to_human.py phase_b/specialists/eval.specialist.json
run "render: human->dense"      $PY phase_b/scaffold/render/human_to_dense.py

echo "-----------------------------------------------"
echo "TOTAL: $pass passed, $fail failed"
[ "$fail" -eq 0 ]
