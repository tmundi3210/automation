# Bounded-Autonomy AI Intelligence System — Planning Package

Operator spec: `SPEC.md` (verbatim). STOP CONDITION honored: read-only
discovery only; NO project files modified; NO implementation until the
operator approves this package.

## Deliverable map (SPEC numbering)

| # | Deliverable | File | Status |
|---|---|---|---|
| 1 | Capability matrix | `CAPABILITY_MATRIX.md` | done; [U] cells close via TASK-015/016 |
| 2 | Project registry | `PROJECT_REGISTRY.yaml` | done (14 projects, git-date-verified) |
| 3 | Topic ontology | `TOPIC_ONTOLOGY.yaml` | done (21 topics, evidence bars, exclusions) |
| 4 | Source registry | `SOURCE_REGISTRY.yaml` | done (30 rows; unverified rows auto-disabled) |
| 5 | System architecture | `SYSTEM_ARCHITECTURE.md` | done |
| 6 | Data schemas | `DATA_SCHEMAS.md` | done (8 JSON-schema blocks + state machine) |
| 7 | Memory architecture | `MEMORY_ARCHITECTURE.md` | done (8 layers) |
| 8 | Evaluation registry | `EVALUATION_REGISTRY.yaml` | done (9 active, 15 planned) |
| 9 | Security policy | `SECURITY_POLICY.md` | done (11 control tables, quarantine flow) |
| 10 | Threat model | `THREAT_MODEL.md` | done (5 boundaries, 6 attacker stories) |
| 11 | Autonomy policy | `AUTONOMY_POLICY.yaml` | done (L0-L5; L5 never autonomous) |
| 12 | Cost budget | `COST_BUDGET.yaml` | done (invocations + wall-minutes) |
| 13 | Implementation phases | `IMPLEMENTATION_PHASES.md` | done (A-H, operator checkpoints) |
| 14 | Operations runbook | `OPERATIONS_RUNBOOK.md` | done (digests, kill switch, playbooks) |
| 15 | Code skeleton | in `SYSTEM_ARCHITECTURE.md` (documented only — no code, per stop condition) | done |
| 16 | Review objects | `reviews/review_claude.json` done; `reviews/review_grok.json` + `reviews/review_codex.json` arrive via TASK-015/016 | partial |
| 17 | Assumptions & risks | `ASSUMPTIONS_AND_RISKS.md` | done (living; finalizes with 15/16) |

## How it was produced

Hub (Claude Code) verified its own environment with live tests; four
specialist-injected author agents wrote docs 3-14 grounded in SPEC + the
repo's operating evidence (EXCHANGE protocol, scoring, polling, research
digest); a read-only inventory agent produced the project data. Grok and
Codex were tasked (TASK-015/016) to self-verify their environments — nothing
about their capabilities is assumed — and to return independent
different-family reviews. Case studies (Hermes Agent, OpenClaw) are
pending exact-project resolution by the scout; their registry rows are
`pending_resolution` with null URLs rather than guessed links.

## Operator decision requested

APPROVE / APPROVE_WITH_CHANGES / REJECT (reply forms in
`OPERATIONS_RUNBOOK.md`). Implementation Phase A starts only on approval.
