# ARCH_REVIEW — inbound architecture recommendation vs this system

Input: `import/ARCH_RECOMMENDATION_INBOUND.md` (operator-pasted external AI
analysis, 2026-07-14; third-party-project claims UNVERIFIED — see its header).
Reviewer: hub (integrator), grounded in the operating repo (EXCHANGE v3.3,
POLLING v1.1, scores.json, gates) and the bounded_autonomy package.
Status: ASSESSMENT ONLY. The bounded_autonomy operator decision
(APPROVE / APPROVE_WITH_CHANGES / REJECT) is still pending; nothing here is
implemented. STOP CONDITION stands.

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
| 1 | No free-form three-way chat; controller passes structured artifacts (task/result packets) | Already our doctrine: TASK blocks with ASSIGNEE/write-scope, per-task branches, `VERDICT:` tokens, DELIVERED-SHA stamps, tasks.json ledger, DATA_SCHEMAS.md (8 JSON-schema blocks incl. task/result/evidence). Builders never prompt each other; everything flows through the integrator and git | **ALREADY** — convergent. Their packet fields `maximum_cost_usd` and explicit `forbidden_paths` are worth merging into our task schema (we have write-scope gates and global COST_BUDGET, but no per-task cost ceiling) |
| 2 | Deterministic (non-LLM) controller service owns state, routing, budgets, permissions | Today the hub-LLM is the controller; hard denials already live OUTSIDE the model (scope gates, launchd, branch protection, sandboxes — SECURITY_POLICY doctrine). A thin deterministic service executing our existing schemas (state machine in DATA_SCHEMAS.md) is the natural Phase C/D evolution; the LLM hub keeps judgment (verdicts), the service keeps mechanics (state, budgets, retries) | **ADOPT (phased)** — genuinely new emphasis; slot as an IMPLEMENTATION_PHASES item, not a rewrite |
| 3 | Interface ladder: SDK/app-server > ACP > MCP > headless CLI > TUI | We run headless CLI + launchd wrappers — the doc itself calls this the "good initial common denominator". Verified caveats: `grok agent` rejected `--cwd` (TASK-014); `grok agent stdio` ACP is an UNVERIFIED claim; codex app-server exists on 0.140.0 (`codex app-server`, operator help paste) but is marked experimental | **ADOPT-LATER** — upgrade path per lane after the current lane is stable (TASK-018 still open); verify each interface on the installed builds first, exactly as TASK-014/015/016 did for the CLI shapes |
| 4 | Worktree/container isolation per agent per task; controller integrates | Per-agent clones + per-task branches + `check_scope.sh`/`gate_all.sh`; single-writer integrator does all merges (`--no-ff`); shared-working-tree topology explicitly REJECTED in plans/local_orchestration/PLAN.md §2 for the same clobber-race reason. Container-level isolation beyond git: not present (R2 accepted residual) | **ALREADY** (git layer); container isolation remains a recorded gap, unchanged by this doc |
| 5 | Grok split: xAI API + X Search (read-only evidence lane) vs Grok Build for coding | TASK-015 verified grok's X tools are interactive session tools, NOT a pollable feed → our X cadence is scout-tick-driven (SOURCE_REGISTRY `x_sources_v1_1`). An xAI API lane would upgrade that to scheduled X intelligence. Research-lane-never-writes already matches THREAT_MODEL (ingested content untrusted; VERIFIED-state gate) | **ADOPT (operator decision)** — needs an xAI API key + cost ceiling; net-new capability, consistent with our rules |
| 6 | Initial roles: grok scout/challenger, claude architect/critic, codex implementer; re-route on evidence after 20–50 tasks | scores.json trust lanes are already evidence-based and already match these roles (grok research-curation 9.0, ops 9.5; codex implementer with recorded process shortfalls; hub integrator/critic). Their metrics list (accepted_patch_rate, regression_rate, human_edit_distance, unsupported_claim_rate…) is finer-grained than ours | **ALREADY**; merge their metric names into EVALUATION_REGISTRY planned rows |
| 7 | Three modes: single agent / worker+reviewer / independent parallel attempts, triggered by risk not habit | Our task flow is mode 1/2 today (builder + integrator review; independent reviews used in the planning package). Mode 3 (blind parallel attempts) used ad hoc (review objects), not formalized. Cost caution matches COST_BUDGET | **ADAPT** — write the three modes + trigger rule into AUTONOMY_POLICY as routing policy |
| 8 | OpenClaw as outer gateway (channels, schedules, approvals) | **CONFLICT with R6** (ASSUMPTIONS_AND_RISKS): OpenClaw + Hermes gateways run as the same macOS user as the owner GitHub token, `~/.codex/auth.json`, builder state dirs; OpenClaw community skills are a known supply-chain surface (CASE_STUDIES.md); Hermes' gateway was the Terminal-window incident. Promoting OpenClaw to the control plane means the flagged surface would carry approval flows | **CONFLICT — operator decision.** Precondition if adopted: isolate first (separate macOS user, no read access to `~/.exchange-gate*`, `~/.codex/`, `~/.grok/`, or the clone token), pin/audit versions, and keep approvals verifiable outside it. Until then our standing recommendation holds |
| 9 | Hermes later, as memory/skills layer only; never two master orchestrators | Matches our stance (R6 containment; MEMORY_ARCHITECTURE owns memory design; Hermes/OpenClaw are Phase-6 case studies, not components) | **ALREADY** (deferred) |
| 10 | Study/borrow Ultraswarm, Deliberation, Sub-Agents Skills, Fusion, ACPX | All five are unverified third-party claims from this container (egress blocked). Their *pattern* (runner owns state, workers replaceable) matches verdict #2 without adopting their code. The doc itself warns against handing machine access to an unreviewed third-party orchestrator | **VERIFY-FIRST** — candidate scout task: resolve each repo, license, activity, security posture; case-study rows like Hermes/OpenClaw, nothing depended on |
| 11 | Shared AGENTS.md instruction file; provider files import it | Partial fit, one security correction: repo instruction files are task INPUTS here, never system prompts — elevating a repo-tracked file into an agent's system prompt is the exact attack our playbook SECURITY FIX closed (standing orders live in local root-owned 444 files). A shared repo AGENTS.md for project facts/conventions is fine within that boundary | **ADAPT WITH CAUTION** — adopt the single-source project-facts file; keep the standing-orders/system-prompt boundary as is |
| 12 | SQLite state store; no vector DB until retrieval is specified | We use tasks.json + git (append-only, auditable); MEMORY_ARCHITECTURE already defers the vector index to a Phase C decision with the same rationale | **ALREADY**; SQLite becomes relevant only alongside verdict #2's controller |
| 13 | 12-step intelligence workflow (scan → evidence → plan → gate → implement → review → evaluate → approve → merge → learn) | Maps ~1:1 onto SYSTEM_ARCHITECTURE + the evidence state machine (external content can never trigger execution past VERIFIED; L5 never autonomous). Their steps 9–10 (draft PR + channel delivery) are concrete additions for Phase D/E | **ALREADY convergent**; fold steps 9–10 into IMPLEMENTATION_PHASES |
| 14 | "What not to build" list | Every item is already a standing rule here: no shared checkout (§2 of local PLAN), no always-approve (TASK-014 moved grok to `acceptEdits`+deny), research lanes have no secrets, self-approval barred (integrator-only verdicts, nobody edits gates), majority-agreement ≠ proof (adversarial verify), transcripts ≠ memory (schema'd artifacts), no auto-exec of ingested commands (quarantine flow) | **ALREADY** — useful as an external confirmation of the package's rules |

## What this changes right now

Nothing operational. Three concrete edits are RECOMMENDED to the pending
package (to apply as part of APPROVE_WITH_CHANGES if the operator wishes):

1. **DATA_SCHEMAS.md**: add `max_cost_usd` (per-task ceiling) and explicit
   `forbidden_paths` to the task packet schema; add their result-packet
   metric fields to EVALUATION_REGISTRY planned rows.
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
  caught three DELIVERED-SHA fabrications.
- We do not adopt any third-party orchestrator sight-unseen; the doc
  agrees, but its tool table reads more confident than its own caveats.
