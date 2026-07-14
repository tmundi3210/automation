# ASSUMPTIONS_AND_RISKS — deliverable 17 (living document)

Status: DRAFT — finalized after TASK-015 (Grok self-verification) and
TASK-016 (Codex self-verification + gate audit) integrate.

## Unresolved assumptions (blocking full confidence, not blocking review)

| # | Assumption | Why unresolved | Resolution path |
|---|---|---|---|
| A1 | ~~Grok X-search~~ RESOLVED (TASK-015): native session tools, live-tested; NOT a feed API | — | done |
| A2 | ~~Codex CLI web-search~~ RESOLVED (TASK-016): `--search` exists but is BLOCKED — configured model `gpt-5.6-terra` rejected by build 0.140.0. Treat as unavailable; new assumption: operator may want to fix the model pin | — | operator decision |
| A3 | Grok and Codex gate state may COLLIDE in `~/.exchange-gate/` — still open; TWO duplicate-ACK incidents are consistent with it | Codex could not inspect `$HOME` from its sandbox | OPERATOR: run `bash incoming/task-016__capability-codex/gate_audit.sh` on the Mac, apply its ACTION lines, paste output |
| A4 | ~~Hermes Agent / OpenClaw resolution~~ RESOLVED (hub WebSearch, 2026-07-13): NousResearch/hermes-agent and openclaw/openclaw — see CASE_STUDIES.md | — | done |
| A5 | Plan/rate limits for all three subscriptions | Not visible from configs inspected so far | Self-verification tasks; else operator-supplied |
| A6 | Only ONE authorized repo (`tmundi3210/automation`) | Hub's GitHub scope is this repo; repo-list tool unavailable in this session; other local projects on the operator's Mac are invisible to the hub | Operator confirms project list; builders can inventory local paths in a later read-only task if authorized |
| A7 | ~~X API ingestion~~ RESOLVED-DIRECTIONALLY (TASK-015): interactive-only confirmed; X cadence is scout-tick-driven (SOURCE_REGISTRY amendment v1_1) unless operator authorizes an X API | — | done |
| A8 | Podcast/audio ingestion needs a transcript source | No component has been verified to transcribe audio | Prefer show-notes/transcript feeds (Tier C is discovery-only anyway) |

## Accepted residual risks (recorded, operator-acknowledged)

| # | Risk | Acceptance basis |
|---|---|---|
| R1 | Both builders push as the OWNER GitHub account (distinct git identities + trailers only) | OPERATOR WAIVER-001 — explicit operator decision; compensating controls: author pin, ancestry check, message-commit purity, scope gate |
| R2 | Builders run on a personal Mac with real credentials present | Least-privilege flags (`acceptEdits` + deny rules, `workspace-write` sandbox) + standing orders in root-owned 444 files; not equivalent to isolated CI |
| R3 | Hub sessions are ephemeral; anything unpushed is lost on container reclaim | Mitigated by push-early discipline; accepted for planning work |
| R4 | MCP servers can disconnect mid-session (observed) | Degradation ladder: fall back to WebSearch/repo data; retry next session |
| R5 | LLM compliance decays on long prohibition lists (arXiv:2605.28639) | Policies written affirmative-first; hard denials enforced OUTSIDE the model (gates, sandboxes, scopes) |

## Known gaps deferred to implementation phases

- No vector index exists yet (memory layer 3) — Phase C decision.
- No dead-letter queue — currently a failed task just re-runs on wake;
  Phase B formalizes.
- Framework-analysis library (SPEC Phase 6) has ZERO entries at planning
  time — first two arrive with TASK-015 case studies.
- The affirmative-first rewrite of both builders' local standing-orders
  files is SCHEDULED but not yet executed (pre-existing backlog item).
- Social-media generation/scheduling (a SPEC topic) has no verified
  execution surface in any component — planning covers ingestion/research
  side only until a posting tool is authorized.

## R6 (added 2026-07-14, operator-confirmed)

Operator runs third-party agent gateways (Hermes Agent `ai.hermes.gateway`,
OpenClaw `ai.openclaw.gateway`) as the same macOS user that holds the owner
GitHub token, `~/.codex/auth.json`, builder state dirs, and the automation
clone. Hermes' loop was the source of the recurring Terminal windows
(driving local codex sessions). OpenClaw's community skill templates are a
known supply-chain surface (CASE_STUDIES.md), now co-resident with builder
credentials. Operator-accepted for now; recommended mitigation: run
gateways under a separate macOS user account, and deny their configs read
access to `~/.exchange-gate*`, `~/.codex/`, `~/.grok/`, and the clone.
Also observed: `com.mundi.PremiumCapsule.auto-deploy` failing (status 78) —
operator-owned, outside this system's scope, flagged for operator review.
