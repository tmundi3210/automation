# RETENTION & CONSOLIDATION POLICY · v1

SCHEDULE (registered in kernel/schedule.jsonl)
- daily light pass (03:00Z): fold diary BELIEF Δ into semantic/; promote lessons seen ≥2×; refresh INDEX.
- weekly deep pass: archive consolidated journal days → episodic/_archive/<period>.md; collapse duplicate beliefs; round-trip check.
- weekly recalibrate: recompute confidence bands → rewrite calibration/recalibration_map.json.
- trigger override: run a pass early if unconsolidated runs ≥ 7 OR any tier exceeds its size bound.

KEEP / ARCHIVE / DROP (rate-distortion — keep what changes a future decision)
- KEEP forever: beliefs (with provenance), trusted playbooks, the prediction ledger, thresholds, the timeline spine.
- ARCHIVE (out of hot path, recoverable): raw journal days older than the audit window; superseded beliefs.
- DROP: working scratch after a run; pure restatement that changes the narrative but no decision.

INVARIANTS
- aleatory (DO-NOT-CHASE) items are never re-opened for "learning".
- a belief is never deleted, only SUPERSEDED (auditable revision).
- the INDEX must stay honest: anything not pointered is effectively forgotten until consolidation re-surfaces it.
