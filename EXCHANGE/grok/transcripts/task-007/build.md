# TASK-007 ctxeng build transcript (grok)

## Task
Build `ctxeng` specialist (LLM context engineering) via factory Path B.
Assignee: grok. Branch: grok/task-007-ctxeng.
Scope: incoming/task-007__ctxeng/**

## Method
Authored three dense Path-B subdomain specs (not production forge/validators edits):
1. kb1 — ctx_budget_compaction (token ledger, digests, eviction, rot probes, multi-agent share)
2. kb2 — ctx_isolation_freshness (trust tiers, quarantine, negation rebound, positive framing, lateral injection)
3. kb3 — ctx_retrieval_grounding (lazy retrieve, provenance, stale/wrong-version, authority conflicts, no fabricated locators)

Forged with `branches/_forge/kb_forge.py`; dense-validated with `validators/kb_validator.py --mode dense`.
Specialist `ctxeng.specialist.json` distilled across three KBs; `validators/specialist_validator.py` pass.

## Content bar notes (culinary round-2 lessons applied)
- Node-specific pros/cons/failure_modes with differentiated weights and numeric anchors (util thresholds 80–90%, tier labels, p90 tool sizes, fidelity floors).
- Negation/ironic-rebound treated as first-class nodes with positive-framing mitigations.
- CQs tied to covering nodes; mid-band counts (21 nodes, 37–40 edges per KB).
- DAG dependency fix: removed MULTI_AGENT_SHARE↔DIGEST_HIERARCHY cycle on kb1 before ship.

## Gates
```
tools/gate_all.sh --task incoming/task-007__ctxeng
GATES: PASS gate_all=bd8210c23eee
```

## Deliverables under incoming/task-007__ctxeng/
- kb{1,2,3}.spec.json + kb{1,2,3}.kb.json + kb{1,2,3}.validation.json
- ctxeng.specialist.json + ctxeng.specialist.validation.json

## Out of scope (honored)
Did not modify validators/**, branches/_forge/**, schema/**, tools/**, dist/**, specialists/ROUTER.json, EXCHANGE/claude/**.
