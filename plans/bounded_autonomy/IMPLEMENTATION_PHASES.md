# IMPLEMENTATION_PHASES — build order for the bounded-autonomy system

Planning deliverable #13 (SPEC.md PLANNING DELIVERABLES).
**STATUS UPDATE 2026-07-14 (DECISION.md): the operator ruled
APPROVE_WITH_CHANGES — the STOP CONDITION is lifted and Phase A is OPEN**
(TASK-019 grok, TASK-020 codex). Changes bound in: OpenClaw and Hermes are
EXCLUDED as components (case studies only); channel delivery uses first-party
channels; a thin deterministic controller service (executing these phases'
schemas) and per-lane interface upgrades (Claude Agent SDK, codex app-server,
grok ACP — each gated on a TASK-01x-style capability verification on the
installed builds) are added to Phases C/D scope per ARCH_REVIEW. Per-phase
operator checkpoints below remain binding.

Sizes are S / M / L (the EXCHANGE task-size scale: roughly ≤1h / ≤4h / ≤24h of
builder effort), not dates. Builder assignments follow the evidence-based trust
lanes (EXCHANGE/ORCHESTRATION.md §1): **Codex = code build + mechanical
verification; Grok = content, sources, high-volume drafting; Claude = wiring,
integration, judging, final review.** Observed trust overrides these priors
after ≥3 tasks per lane (EXCHANGE/SCORING.md §2).

## Global rules (apply to every phase)

- **STOP rule (universal):** nothing merges without operator approval
  (SPEC governance #7). Each phase ends with an **operator checkpoint**: a
  short demo + the exit-criteria evidence; the operator's explicit approval is
  the only thing that opens the next phase.
- All build work happens in per-task branches/worktrees with scoped writes;
  deterministic gates run before any review; author ≠ reviewer ≠ final judge.
- Every phase produces its own audit trail (task IDs, delivered SHAs, verdict
  messages) in the existing EXCHANGE machinery.
- Budget: all phase work is metered against COST_BUDGET.yaml; a phase that
  would exceed its component's ceiling pauses at a checkpoint rather than
  degrading review quality.
- Dependency order is binding: a phase must not start before its entry
  criteria hold, even if a later phase looks more valuable (sequence by
  dependency, not by shiny-ness).

## Phase order rationale

Control plane before data (A), read-only before analysis (B), storage before
scoring (C), scoring before recommendations (D), sandboxed experiments before
any PR (E→F), visibility before scale (G), and self-tuning proposals last (H)
— each phase makes the next one's failure modes observable and reversible.

---

## Phase A — Control-plane skeleton + capability matrix finalization

**Size:** M

**Builds:** deterministic scheduler/state/permission/budget/log skeleton under
`orchestrator/control_plane/` + `orchestrator/config/` (per the SPEC code
skeleton); finalized CAPABILITY_MATRIX.md with every claim verified by config,
official docs, or a harmless non-mutating test — including the **UNVERIFIED
plan limits** flagged in COST_BUDGET.yaml (verify actual Claude Code / Grok CLI
/ Codex CLI subscription limits here, or record them as still-unverified with
the test that failed to establish them).

**Who:** Codex builds the skeleton code; Grok drafts the capability probes and
runs the harmless verification commands on the Mac side; Claude integrates,
reviews, and signs the matrix.

**Entry criteria:** operator has approved the full planning package (STOP
CONDITION); PROJECT_REGISTRY.yaml scope agreed.

**Exit criteria (verifiable):**
- `orchestrator/control_plane/` runs a no-op schedule tick end-to-end: reads
  config, writes a run record with run_id + trace_id, appends to the audit log,
  exits 0 — with zero model invocations.
- Budget counters increment and the 70/85/95/100% degradation rungs fire
  correctly against synthetic counter values (unit-tested).
- CAPABILITY_MATRIX.md has a verification-evidence line per cell; unverifiable
  cells say UNVERIFIED, not a guess.
- Kill switch works: the pause mechanism in OPERATIONS_RUNBOOK.md stops the
  skeleton's tick and the resume restarts it (demonstrated live).

**What can go wrong:** capability claims copied from marketing docs instead of
verified (mitigate: evidence line per cell, Claude spot-checks); control plane
accidentally given model-call ability "for convenience" (mitigate: skeleton is
shell/python stdlib only — a model call in the control plane is a review
reject); scope creep into building adapters early.

**Operator checkpoint:** demo the no-op tick + kill switch; operator approves
the capability matrix as the factual baseline. **STOP:** no Phase B until then.

---

## Phase B — Source registry + gated ingestion (read-only)

**Size:** M

**Builds:** SOURCE_REGISTRY.yaml populated per SPEC PHASE 3 (tiers A/B/C, all
required per-source fields); `orchestrator/adapters/` pollers for the initial
source set (RSS, GitHub releases, papers, X lists) that are strictly
**read-only**: fetch → normalize → content-hash → store raw. No model analysis
yet. Polling cadences per SPEC PHASE 3 and COST_BUDGET.yaml `scout_ticks`;
conditional HTTP (ETag/If-Modified-Since) from day one.

**Who:** Grok curates the seed source list and per-source metadata (its
research-curation lane, proven in TASK-013); Codex builds the pollers and the
hash gate; Claude reviews and wires into the control plane.

**Entry criteria:** Phase A approved; capability matrix confirms each adapter's
access method actually exists.

**Exit criteria (verifiable):**
- 48h of unattended polling with zero model invocations, zero writes outside
  the ingestion store, and a poll log line for every tick (including idle —
  the TASK-014 lesson: silence must be diagnosable).
- Duplicate fetch of unchanged content produces a hash match and no new record.
- Every stored item carries source_id, timestamps, raw-content reference, and
  content hash (PHASE 4 evidence-record fields that exist at this stage).
- Rate-limit behavior demonstrated against one throttling source (backoff with
  jitter, no retry storm).

**What can go wrong:** an adapter quietly gains write or shell scope (mitigate:
adapters run sandboxed, read-only credentials, scope gate on every task);
prompt-injection payloads enter the store (expected and fine — content is DATA;
the injection boundary is enforced at consumption, Phase C+); a poller dies
silently (mitigate: per-tick log line + dead-letter capture from day one).

**Operator checkpoint:** review the 48h log, the registry, and a sample of
stored items. **STOP:** no model analysis of ingested content until approved.

---

## Phase C — Evidence store + scoring

**Size:** L

**Builds:** the PHASE 4 pipeline states NORMALIZED → DEDUPLICATED → VERIFIED →
SCORED as real code (`orchestrator/ingestion/`, `normalization/`,
`verification/`, `scoring/`); full evidence records (all PHASE 4 fields);
immutable raw store + structured store (PHASE 5 layers 1-2); scoring formulas
adapted from EXCHANGE/SCORING.md (novelty, relevance, evidence-quality,
reproducibility, confidence). First **model-invoking** stage — the HASH_GATE
hard rule from COST_BUDGET.yaml is enforced in code here.

**Who:** Codex builds pipeline + stores; Claude designs and reviews scoring
math (with the agenteval specialist's formulas as the base) and does the
verification-lane analysis; Grok supplies volume test content and runs
cross-checks.

**Entry criteria:** Phase B approved; ≥1 week of real ingested data to test
against; DATA_SCHEMAS.md fields frozen for the evidence record.

**Exit criteria (verifiable):**
- Replaying the same day's raw intake twice yields byte-identical evidence
  records (idempotent, hash-gated: second pass spends zero model invocations).
- Duplicate rate reaching analysis < 5%; each analyzed item logs its
  invocation against the `ingestion_analysis` stage budget.
- Model-generated summaries are stored as derived data linked to — never
  replacing — the raw evidence (SPEC PHASE 4: summaries are not primary
  evidence).
- Scores carry components, not just totals (auditable per SCORING §4), and a
  seeded set of known-junk + known-good items ranks correctly.

**What can go wrong:** score inflation / Goodhart (mitigate: component-wise
scores, periodic re-judge canary); model analysis triggered on unchanged
content (mitigate: budget counter assertion tests); fabricated evidence in
summaries (mitigate: every claim links to its raw source; spot fact-checks in
review); cost blowout on the backlog (mitigate: per-tier daily caps, ladder).

**Operator checkpoint:** review scored sample + spend report (invocations per
analyzed item). **STOP:** scores inform nothing downstream until approved.

---

## Phase D — Project mapping + digests

**Size:** M

**Builds:** `orchestrator/project_mapping/` — mapping scored evidence to
PROJECT_REGISTRY.yaml projects via TOPIC_ONTOLOGY.yaml (including exclusion
topics); the **daily morning digest** (≤4 bullets per project — format in
OPERATIONS_RUNBOOK.md), weekly synthesis, and monthly source-quality report
(SPEC PHASE 3 cadence).

**Who:** Grok drafts digest templates and topic-mapping content; Codex builds
the mapping/render code; Claude reviews mapping correctness and owns final
digest quality (factual long-form is Claude's lane).

**Entry criteria:** Phase C approved; PROJECT_REGISTRY.yaml and
TOPIC_ONTOLOGY.yaml operator-approved.

**Exit criteria (verifiable):**
- One week of daily digests delivered on schedule, each ≤4 bullets per project,
  every bullet tracing to an evidence_id (click-through works).
- Items in a project's exclusion topics provably never appear in its digest
  (test fixture).
- Useful-alert precision baseline measured: operator thumbs-up/down per bullet
  for the week, recorded (PHASE 10 metric).
- Weekly synthesis + monthly source report render from real data.

**What can go wrong:** digest bloat (mitigate: hard 4-bullet cap in the
renderer, not the prompt); relevance drift flooding the operator (mitigate:
precision metric + threshold tuning is a Phase H proposal, operator-applied);
cross-project leakage (mitigate: project-scoped retrieval per SPEC PHASE 5).

**Operator checkpoint:** a week of digests + precision numbers. **STOP:** no
candidate is promoted toward experiments until approved.

---

## Phase E — Experiment harness (isolated worktrees)

**Size:** L

**Builds:** `orchestrator/experiments/` + `evaluations/` implementing SPEC
PHASE 8: baseline recording; version pinning (model, prompt, deps, tools);
disposable worktree/container per experiment with secrets and network stripped;
success/failure/regression thresholds declared before the run; variance
estimation via repeats; full command/environment/output capture; auto-reject on
threshold miss. EXPERIMENT_CANDIDATE → EXPERIMENTED → ACCEPTED/REJECTED states.

**Who:** Codex builds and runs the harness (code + mechanical verification
lane); Claude reviews isolation and threshold design; Grok scaffolds experiment
manifests for volume.

**Entry criteria:** Phase D approved; at least one operator-approved
experiment candidate exists; SECURITY_POLICY.md sandbox rules approved
(governance #6: no external code outside an isolated sandbox).

**Exit criteria (verifiable):**
- A benign known-good candidate and a known-bad candidate both run end-to-end:
  the good one passes thresholds, the bad one auto-rejects, both leave complete
  experiment records.
- Isolation proven by test: an experiment attempting to read a secret path,
  reach the network, or write outside its worktree fails and the attempt is
  logged.
- Worktrees are disposable: post-run teardown leaves no residue; reruns
  reproduce results within declared variance.
- Every experiment logs against the `experiments` stage budget (≤1 concurrent,
  wall-minute caps).

**What can go wrong:** sandbox escape via test code from ingested sources
(mitigate: isolation tests above are gate, not docs; deny-by-default network);
benchmark gaming — the harness reporting success it didn't earn (mitigate:
thresholds and baselines recorded BEFORE the run by the control plane, results
recomputed by a different agent than the builder, per no-self-certification);
nondeterminism masking regressions (mitigate: mandatory repeat runs +
variance).

**Operator checkpoint:** review both experiment records + isolation test
results. **STOP:** no experiment output touches any project repo until Phase F
exists and the specific PR is approved.

---

## Phase F — Draft-PR flow + review objects

**Size:** M

**Builds:** PR_READY → HUMAN_APPROVED states: successful experiments become
clean diffs + experiment manifests + **draft** pull requests on non-protected
branches (SPEC governance #5, #7, #8); the PHASE 7 structured review object
(decision / confidence / verified_claims / unverified_claims / concerns /
required_changes / evidence_ids / recommended_next_state) produced
independently by each non-author agent; operator approve/reject reply forms
wired (OPERATIONS_RUNBOOK.md).

**Who:** Codex builds the PR mechanics; Claude is final reviewer/judge and
writes the integration review; Grok cross-verifies as the different-family
check.

**Entry criteria:** Phase E approved; target projects' protected paths and
branches encoded in PROJECT_REGISTRY.yaml; branch protections verified.

**Exit criteria (verifiable):**
- One end-to-end candidate: evidence → experiment → draft PR containing diff,
  manifest, baseline comparison, and both review objects — and it sits at
  DRAFT until the operator acts. A merge attempt by any agent fails
  (permission-level, not prompt-level).
- REJECT path works: an operator `REJECT` reply moves the candidate to
  REJECTED with the reason in the decision log; the branch is preserved,
  nothing retries automatically.
- Review objects validate against the PHASE 7 schema; author ≠ reviewer
  enforced mechanically.

**What can go wrong:** an agent merges "helpfully" (mitigate: no merge
permission exists for agent credentials — enforced outside the model; verified
by the failed-merge test); PR spam burning operator attention (mitigate: PRs
gated on experiment success + daily review-object budget); out-of-scope diffs
(mitigate: zero-token scope gate rejects mechanically before any review).

**Operator checkpoint:** walk the full trail of the one candidate; approve the
flow itself. **STOP:** the flow handles real candidates only after this
approval, and each individual PR still requires its own operator approval
forever.

---

## Phase G — Observability dashboard

**Size:** M

**Builds:** `orchestrator/observability/` + `dashboard/`: health dashboard
(PHASE 10) rendering source-health, budget spend vs ceilings, pipeline-state
counts, dead-letter queue depth, model-quality and project-impact metrics, the
ten PHASE 10 system metrics, run/trace ID search. STATUS.md-style plain-language
mirror; failure alerts wired into the digest.

**Who:** Codex builds; Grok drafts the plain-language surfaces; Claude reviews
metric definitions (quant discipline: every metric has a stated denominator,
window, and data source — no vibes-only numbers).

**Entry criteria:** Phase F approved (all pipeline stages emit events worth
observing). May overlap Phase F build once Phase E is approved, at operator
discretion — it reads events, it does not gate them.

**Exit criteria (verifiable):**
- Dashboard answers, from stored data alone: what ran today, what it spent,
  what failed, what awaits the operator — each drill-down ending at a run_id /
  evidence_id / task_id.
- A synthetic injected failure (killed poller) surfaces as an alert within one
  digest cycle.
- Metric spot-audit: three dashboard numbers recomputed by hand from the
  ledgers match.

**What can go wrong:** metrics that mislead (mitigate: definitions
review + hand recomputation audit); dashboard becoming a write path (mitigate:
strictly read-only over the ledgers); alert fatigue (mitigate: alert budget —
digest-first, page-never at this scale).

**Operator checkpoint:** operator uses the dashboard for a week alongside raw
files; approves it as the daily surface. **STOP:** Phase H untouched until then.

---

## Phase H — Bounded self-evolution proposals

**Size:** M

**Builds:** SPEC PHASE 11, proposal-only: the system generates
`PROPOSAL-NNN` records for adding/removing sources, polling-frequency changes,
topic-weight changes, source-score updates, model-routing changes, evaluation
dataset updates, and non-security prompt improvements — each with before/after
evidence and expected benefit. Proposals land in the weekly digest for operator
decision. **Applying** any proposal is an operator action; the forbidden list
(security boundaries, permissions, secrets, budget ceilings, approval rules,
protected branches, deployment permissions, audit logging, and the
self-evolution restrictions themselves) is enforced by schema: a proposal
touching those files/keys is rejected by the control plane at creation time.

**Who:** Grok drafts proposals (volume lane); Claude verifies the evidence
behind each and judges; Codex builds the proposal schema/validator.

**Entry criteria:** Phases A-G approved; ≥1 month of metrics history so
proposals can cite real before-data.

**Exit criteria (verifiable):**
- Three real proposals produced with evidence; at least one operator-applied
  and its predicted benefit checked against measured outcome after a week.
- Forbidden-scope test: a synthetic proposal targeting a budget ceiling or a
  security file is mechanically rejected and logged.
- No code path exists from proposal record to config change without the
  operator reply forms.

**What can go wrong:** evolution pressure toward metric gaming (mitigate:
predicted-vs-measured follow-up is mandatory and scored; the self_report_gap
principle from SCORING applied to the system itself); proposal spam (mitigate:
weekly batch, capped count); laundering a forbidden change inside an allowed
one (mitigate: schema-level file/key denylist + operator diff review).

**Operator checkpoint:** review the proposal→outcome loop. This checkpoint
recurs monthly thereafter — Phase H never "completes" into autonomy; it stays
proposal-only by governance rule 9.

---

## Summary table

| Phase | Deliverable | Size | Primary builder | Ends with |
|---|---|---|---|---|
| A | Control-plane skeleton + capability matrix | M | Codex (+Grok probes) | Operator checkpoint |
| B | Source registry + read-only gated ingestion | M | Codex code, Grok sources | Operator checkpoint |
| C | Evidence store + scoring | L | Codex (+Claude scoring) | Operator checkpoint |
| D | Project mapping + digests | M | Codex code, Grok content | Operator checkpoint |
| E | Experiment harness (isolated worktrees) | L | Codex | Operator checkpoint |
| F | Draft-PR flow + review objects | M | Codex (+Claude judge) | Operator checkpoint |
| G | Observability dashboard | M | Codex (+Grok surfaces) | Operator checkpoint |
| H | Bounded self-evolution (proposal-only) | M | Grok (+Codex schema) | Recurring monthly checkpoint |

Claude (hub) integrates, reviews, and judges in every phase; it is the single
writer of shared state throughout. Nothing in any phase merges without
operator approval — that rule has no exceptions and no sunset.
