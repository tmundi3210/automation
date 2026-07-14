# ARCH_REVIEW — inbound architecture recommendation vs this system

Input: `import/ARCH_RECOMMENDATION_INBOUND.md` (operator-pasted external AI
analysis, 2026-07-14; third-party-project claims UNVERIFIED — see its header).
Reviewer: hub (integrator), grounded in the operating repo (EXCHANGE v3.3,
POLLING v1.1, scores.json, gates) and the bounded_autonomy package.
Status: ASSESSMENT ONLY. The bounded_autonomy operator decision
(APPROVE / APPROVE_WITH_CHANGES / REJECT) is still pending; nothing here is
implemented. STOP CONDITION stands.
Second-pass verification (agenteval advisor, 2026-07-14): REVISE → all 11
findings applied in place, including corrections to this review's own
overstatements (enforcement strength, DATA_SCHEMAS contents, R6 quote,
controller-authority reading, verdict #7 flipped ADAPT→ALREADY). Rows tag
cited mechanisms (OPERATING) vs (PLANNED-in-package) where they differ.

## Headline

The inbound document **largely converges on what this repo already does or
has already planned** — structured artifacts instead of free-form chat,
isolation per agent, evidence-based routing, human approval gates, research
lanes without write access. Where it goes further, three items are genuinely
worth folding in (controller service, interface upgrade path, xAI-API X
lane) and one recommendation **directly conflicts with our recorded risk
R6** (OpenClaw as outer gateway). Point-by-point below.

## Gap map

| # | Inbound recommendation | Where this system stands (evidence) | Verdict |
|---|---|---|---|
| 1 | No free-form three-way chat; controller passes structured artifacts (task/result packets) | Already our doctrine: TASK blocks with ASSIGNEE/write-scope, per-task branches, `VERDICT:` tokens, DELIVERED-SHA stamps, tasks.json ledger (all OPERATING); DATA_SCHEMAS.md adds 8 schema blocks — evidence_record, source_record, claim, project_mapping, experiment_manifest, review_object, decision_log, state_machine (PLANNED-in-package; it has NO task/result-packet schema today). Builders never prompt each other; everything flows through the integrator and git | **ALREADY** — convergent at the artifact level. Their explicit `forbidden_paths` and per-task cost ceiling are worth adopting — see recommended edit 1 |
| 2 | Deterministic (non-LLM) controller service owns state, routing, budgets, permissions — including deciding when output is ready/rejected/escalated (§3), final integration (§6), and post-approval merges (§11) | Today the hub-LLM is the controller; hard denials that live OUTSIDE the model today: scope gates, launchd, sandboxes, deny rules + tip-pinning tripwire (branch protection is BYPASSABLE under WAIVER-001 — the owner PAT can push anywhere; THREAT_MODEL R-1). Read plainly, the inbound doc transfers readiness/rejection/integration/merge AUTHORITY to the controller — more than mechanics. Hub position, on the record: transfer state-keeping, budgets, retries, and merge EXECUTION to a deterministic service in the target state; keep acceptance judgment (does the work meet intent) as an LLM review function whose verdict feeds the controller's state machine. Whether to go further is the operator's architecture call | **ADOPT (phased, scoped as stated)** — slot as an IMPLEMENTATION_PHASES item; the authority split above is an explicit open decision, not settled here |
| 3 | Interface ladder: SDK/app-server > ACP > MCP > headless CLI > TUI | We run headless CLI + launchd wrappers — the doc itself calls this the "good initial common denominator". Verified caveats: `grok agent` rejected `--cwd` (TASK-014); `grok agent stdio` ACP is an UNVERIFIED claim. Codex app-server: the operator's terminal paste of `codex help` (2026-07-14, this hub session — recorded here as the evidence) lists `app-server  [experimental] Run the app server or related tooling` and `remote-control  [experimental]`; experimental flag = verify before depending, exactly the TASK-01x pattern | **ADOPT-LATER** — upgrade path per lane after the current lane is stable (TASK-018 still open); verify each interface on the installed builds first, exactly as TASK-014/015/016 did for the CLI shapes |
| 4 | Worktree/container isolation per agent per task; controller integrates | Per-agent clones + per-task branches + `check_scope.sh`/`gate_all.sh`; single-writer integrator does all merges (`--no-ff`); shared-working-tree topology explicitly REJECTED in plans/local_orchestration/PLAN.md §2 for the same clobber-race reason. Container-level isolation beyond git: not present (R2 accepted residual) | **ALREADY** (git layer); container isolation remains a recorded gap, unchanged by this doc |
| 5 | Grok split: xAI API + X Search (read-only evidence lane) vs Grok Build for coding | TASK-015 verified grok's X tools are interactive session tools, NOT a pollable feed → our X cadence is scout-tick-driven (SOURCE_REGISTRY `x_sources_v1_1`) (OPERATING). An xAI API lane would upgrade that to scheduled X intelligence. Research-lane-never-writes matches THREAT_MODEL's untrusted-ingestion rule and the VERIFIED-state gate (PLANNED-in-package) | **ADOPT (operator decision)** — needs an xAI API key + cost ceiling; net-new capability, consistent with our rules |
| 6 | Initial roles: grok scout/challenger, claude architect/critic, codex implementer; re-route on evidence after 20–50 tasks | The roles match our routing-table PRIORS, with observed evidence consistent so far but thin: scores.json marks its lanes low-confidence at n=1–2 (grok research-curation 9.0, ops 9.5; codex has recorded process shortfalls and no observed implementer-lane score yet); OBSERVED overrides priors only at ≥3 tasks/lane — the same "re-route on your own evidence" rule the doc proposes. Their metrics list (accepted_patch_rate, regression_rate, human_edit_distance, unsupported_claim_rate…) is finer-grained than ours | **ALREADY (as priors + rule)**; merge their metric names into EVALUATION_REGISTRY planned rows |
| 7 | Three modes: single agent / worker+reviewer / independent parallel attempts, triggered by risk not habit | Already formalized: ORCHESTRATION §5 pattern (b) is blind-parallel + judge, and the §5b RISK×SIZE minimum-pattern table forces pattern (b) for HIGH-risk / ambiguous-spec / revise-capped work — i.e., modes triggered by risk, not habit (OPERATING). Cost caution matches COST_BUDGET. Genuinely new: 3-way (vs 2-way) parallelism, and cross-referencing the mode table from AUTONOMY_POLICY | **ALREADY** (corrected from ADAPT by second-pass review) |
| 8 | OpenClaw as outer gateway (channels, schedules, approvals) | **CONFLICT with R6** (ASSUMPTIONS_AND_RISKS): OpenClaw + Hermes gateways run as the same macOS user as the owner GitHub token, `~/.codex/auth.json`, builder state dirs; OpenClaw community skills are a known supply-chain surface (CASE_STUDIES.md); Hermes' gateway was the Terminal-window incident. Promoting OpenClaw to the control plane means the flagged surface would carry approval flows | **CONFLICT — operator decision.** Precondition if adopted — R6's mitigation verbatim: "run gateways under a separate macOS user account, and deny their configs read access to `~/.exchange-gate*`, `~/.codex/`, `~/.grok/`, and the clone". This review ADDS two further conditions beyond R6: pin/audit gateway versions, and keep approval records verifiable outside the gateway. Until met, our standing recommendation holds. **RESOLVED 2026-07-14 (DECISION.md): operator ruled OpenClaw OUT — REJECTED as a component; case study only** |
| 9 | Hermes later, as memory/skills layer only; never two master orchestrators | Matches our stance (R6 containment; MEMORY_ARCHITECTURE owns memory design; Hermes/OpenClaw are Phase-6 case studies, not components) | **ALREADY** (deferred) |
| 10 | Study/borrow Ultraswarm, Deliberation, Sub-Agents Skills, Fusion, ACPX | All five are unverified third-party claims from this container (egress blocked). Their *pattern* (runner owns state, workers replaceable) matches verdict #2 without adopting their code. The doc itself warns against handing machine access to an unreviewed third-party orchestrator | **VERIFY-FIRST** — candidate scout task: resolve each repo, license, activity, security posture; case-study rows like Hermes/OpenClaw, nothing depended on |
| 11 | Shared AGENTS.md instruction file; provider files import it | Partial fit, one security correction: repo instruction files are task INPUTS here, never system prompts — elevating a repo-tracked file into an agent's system prompt is the exact attack our playbook SECURITY FIX closed (standing orders live in local root-owned 444 files). A shared repo AGENTS.md for project facts/conventions is fine within that boundary | **ADAPT WITH CAUTION** — adopt the single-source project-facts file; keep the standing-orders/system-prompt boundary as is |
| 12 | SQLite state store; no vector DB until retrieval is specified | We use tasks.json + git (append-only, auditable); MEMORY_ARCHITECTURE already defers the vector index to a Phase C decision with the same rationale | **ALREADY**; SQLite becomes relevant only alongside verdict #2's controller |
| 13 | 12-step intelligence workflow (scan → evidence → plan → gate → implement → review → evaluate → approve → merge → learn) | Maps ~1:1 onto SYSTEM_ARCHITECTURE + the evidence state machine (external content can never trigger execution past VERIFIED; L5 never autonomous) — both PLANNED-in-package. Step 9 (report + draft PR + approval request) already IS Phase F of IMPLEMENTATION_PHASES / state-machine row 11 / AUTONOMY L4. Only step 10 (channel delivery) is new — and it routes through OpenClaw, so it inherits verdict #8's R6 precondition | **ALREADY convergent**; step 10 folds into IMPLEMENTATION_PHASES chained to #8 |
| 14 | "What not to build" list | Every item is a standing rule here — OPERATING: no shared checkout (§2 of local PLAN), no always-approve (TASK-014 moved grok to `acceptEdits`+deny), self-approval barred (integrator-only verdicts, nobody edits gates), majority-agreement ≠ proof (adversarial verify in practice); PLANNED-in-package: research lanes have no secrets (SECURITY_POLICY), transcripts ≠ memory (MEMORY_ARCHITECTURE schema'd artifacts), no auto-exec of ingested commands (quarantine flow) | **ALREADY (rules operating; enforcement docs partly pending package)** — useful external confirmation |

## What this changes right now

Nothing operational. Three concrete edits are RECOMMENDED to the pending
package (to apply as part of APPROVE_WITH_CHANGES if the operator wishes):

1. **DATA_SCHEMAS.md**: add a NEW task-packet schema (or amend ORCHESTRATION
   §4's TASK block) carrying explicit `forbidden_paths` and a per-task
   ceiling denominated in `wall_minutes`/`invocations` — NOT USD, which
   COST_BUDGET.yaml explicitly rules out for flat-rate plans
   (`wall_minutes_each_max` is the existing precedent); add their
   result-packet metric fields to EVALUATION_REGISTRY planned rows.
2. **IMPLEMENTATION_PHASES.md**: add Phase C/D items — thin deterministic
   controller service executing the existing schemas; per-lane interface
   upgrades (codex app-server, grok ACP, Claude Agent SDK) each gated on a
   TASK-01x-style capability verification on the installed builds.
3. **New operator decisions logged**: (a) xAI API X-search lane (key + cost
   ceiling); (b) OpenClaw-as-gateway — blocked on the R6 isolation
   precondition; (c) scout verification task for the five third-party
   projects named above.

## Honest differences kept

- We keep an LLM hub for judgment even in the target state; the doc's
  controller replaces the hub's MECHANICS, not its review function.
- We keep GitHub (not SQLite) as the audit spine until a controller
  exists; git history is our replay/authenticity layer and it has already
  caught three DELIVERED-SHA mismatches.
- We do not adopt any third-party orchestrator sight-unseen; the doc
  agrees, but its tool table reads more confident than its own caveats.
