# W3_plan — PLAN — direction-then-execute (set direction once, then commit)

> One PLAN seam per turn: set a falsifiable direction once, emit a terminating step ledger, then hand ACT the next ready step and stop deliberating until a named defeater fires. Owner specialist: `mind_plan` (planner_heavy slice of Agency). Loop position: step 3 of 7, strictly after RECALL surfaces the focus and hot pointers, strictly before ACT executes a single step. Single-write-owner of exactly three files: `goals/direction.md`, `goals/plan.md`, `goals/subgoals.md`.

## Purpose

PLAN converts an ACTIVE goal into a committed direction and a runnable, provably-terminating ledger, then defends that commitment against thrash. The faculty exists to solve two opposite failure modes at once:

- **Under-commitment / replanning churn:** an LLM that re-weighs alternatives every turn never ships. PLAN forbids re-deliberation once a direction is committed. The only thing that reopens deliberation is a *named* defeater.
- **Over-commitment / blind persistence:** an LLM that never re-plans rides a falsified assumption into the ground. PLAN keeps a small, explicit defeater list and re-plans the moment one fires — by *appending* a new committed direction, never overwriting the audit trail.

The phase decides **direction** (which one bet, with the rejected alternatives recorded) and **decomposition** (an ordered SERIES/PARALLEL ledger with a budget and a strictly-decreasing ranking that proves the plan halts). It does not perceive, recall, execute, predict, adjudicate, score, or run the compliance gate. When the right move is outside PLAN's mandate, PLAN hands it off.

On the seeded run (R0631, env `tech_builders`, goal **G-12** "weekly brief: AI-agent hype vs organic"), PLAN's output is the committed direction block for G-12, the 6-step ledger with `plan_hash:a1b2c3`, and the subgoal audit that spawned G-07 (acquire tech_builders competence) and later spawned OQ-07 (handed to Attention). PLAN did **not** write belief B2, prediction P-114, lesson L-23, or the compliance verdict — those belong to other faculties.

## Inputs (memory read) and Outputs (memory written)

All paths are relative to `branches/b60_content_intelligence/mind/memory/`.

**READ at entry (only these, all read-only):**

- `goals/goals.md` — the goal stack; resolve the top-of-stack `status:ACTIVE` block. On the seeded run that is **G-12** (parent G-01, depth 1), with its `intent`, `success_criterion`, `fuel: 3/12`, and `touches: W-ai-agents, A-builders, P-hype-adjud, SCH-geo-link`.
- `goals/direction.md` — is there already a committed-direction heading whose id matches the active goal? This selects the SET-DIRECTION-ONCE vs EXECUTE-DON'T-REPLAN branch.
- `goals/plan.md` — the current step ledger: `max_steps` budget, `plan_hash`, and each step's `dep`/`mode`/`status`/`desc`.
- `goals/subgoals.md` — the append-only spawn/pop audit, so PLAN never re-spawns a precondition already open or already popped.
- `INDEX.md` — the map of what exists. **Rule: if a record PLAN would rely on is not listed in INDEX.md, it does not exist yet.** Treat it as a missing precondition (spawn a subgoal); never hallucinate its contents.
- The RECALL-loaded focus line and HOT POINTERS (read-only, never written by PLAN): `W-ai-agents` → `semantic/world/ai-agents.md` (beliefs B1 adoption-rising .68, **B2 manufactured-hype .74**, B3 spike-noise aleatory), `A-builders` → `semantic/audiences/builders.md` (A1 signal-over-volume .80, A2 framing .72), `P-hype-adjud` → `procedural/playbooks/hype-adjudication.md` (the ≥N-origin gate, N=3), `SCH-geo-link` → `schemas/SCH_geo_link_react_brief.md` (the re-bound `tech_builders` binding committed via TRX_0007). PLAN reads these so direction rests on current beliefs, not invention.

**WRITE (single-write-owner — only these three):**

- `goals/direction.md` — committed direction (`direction` + `because` + `rejected:` list + `success_criterion` + `review_after`). **Append-only per commit:** every replan appends a new dated block; prior blocks stay for audit.
- `goals/plan.md` — the step ledger (ordered table: `step | dep | mode | status | desc`) plus a `max_steps` budget, a strictly-decreasing `ranking` line, and the `re-plan triggers` list, stamped with a `plan_hash`.
- `goals/subgoals.md` — append-only `SPAWN`/`POP` lines for precondition subgoals.

**NEVER touched by PLAN:** `goals/goals.md` goal-status (goal-stack owner), and anything under `attention/`, `calibration/`, `experience/`, `schemas/`, `semantic/`, `procedural/`, `episodic/`, `_meta/`, `kernel/`.

## Protocol

Run this every turn at the PLAN seam. Steps 1–2 always run; then exactly one of branch A (SET-DIRECTION-ONCE) or branch B (EXECUTE-DON'T-REPLAN) runs; step 7 (precondition sweep) and step 8 (handoff) always close the seam.

### 1. Resolve the active goal

Read `goals/goals.md`. Find the top-of-stack block with `status:ACTIVE` (file order is stack order; the topmost ACTIVE block wins). Capture: `id`, `intent`, `success_criterion`, `parent`, `fuel`/`max_steps` budget, and `touches`.

- On the seeded run this resolves to **G-12** (parent G-01, `fuel: 3/12`, touches `W-ai-agents, A-builders, P-hype-adjud, SCH-geo-link`).
- **If no block is ACTIVE:** PLAN has nothing to plan. Stop and defer to Agency's goal-stack owner. Do not invent a goal.
- If two blocks both read ACTIVE (a corruption), do not guess — treat as a structural fault and ESCALATE (see Failure modes).

### 2. Branch on committed direction

Scan `goals/direction.md` for a `## <goal-id> — committed` heading matching the active id.

- **No matching heading** → branch A, SET-DIRECTION-ONCE (steps 3–5).
- **A matching heading exists** → branch B, EXECUTE-DON'T-REPLAN (step 6). Do **not** regenerate the ledger.

On the seeded run the heading `## G-12 — committed 2026-06-14` exists, so the steady-state turn takes branch B. Branch A is shown below as it ran on 2026-06-14 when G-12 was first activated.

### 3. SET-DIRECTION-ONCE — (a) write a falsifiable success_criterion

The criterion must be **metric + threshold + horizon**, never an activity. "Write briefs" is an activity and is rejected. The committed criterion for G-12 is:

> 1 clearance-passing brief/week whose hype call is later confirmed (≥N independent origins), beats human-curated baseline over 6 weeks.

Decomposed: metric = clearance-passing briefs whose hype call is later confirmed; threshold = ≥1/week AND beats the human-curated baseline; horizon = 6 weeks. This is falsifiable — if six weeks pass and the briefs do not beat baseline, or hype calls are not later confirmed at ≥N origins, the direction has failed and a defeater fires.

Checklist before accepting a criterion:
- Does it name a measurable metric? (not "improve", "engage", "cover")
- Does it carry a numeric or comparative threshold?
- Does it carry a horizon (a date or a count of cycles)?
- Is it falsifiable — is there an observation that would prove it failed?

If any answer is no, rewrite until all four hold.

### 3. SET-DIRECTION-ONCE — (b) pick exactly ONE direction, record WHY, log rejected alternatives

Pick one direction and write a `because` grounding it in RECALL-loaded beliefs, not invented facts. For G-12:

- **direction:** weekly, pick ONE rising-but-not-saturated AI-agent topic, adjudicate hype honestly (≥N independent origins), ship a brief that is RIGHT about hype rather than loud about it.
- **because:** builders reward signal over noise — grounded in A1 (`semantic/audiences/builders.md`, .80) and B2 (`semantic/world/ai-agents.md`, manufactured-hype .74). The right-to-win is honest adjudication, not volume.
- **rejected:** `[chase-the-biggest-trend: over-saturated, no moat | post-daily-volume: dilutes the honesty signal, burns trust]`. Each rejected alternative carries the reason it lost. Logging the losers is mandatory — it is what lets a future defeater check ("new info flips which direction is best") compare against the alternatives instead of re-deriving them.

Every claim in the `because` must trace to a pointer RECALL loaded. If the direction needs a belief that is not in INDEX.md, that is a missing precondition (go to step 7), not a license to assert it.

### 3. SET-DIRECTION-ONCE — (c) emit the terminating step ledger to `goals/plan.md`

Emit an ordered table. Each step carries: a `dep` (the step it waits on, or `-`), a `mode` tag (`SERIES` or `PARALLEL`), a `status` (`open`/`DONE`/`blocked`), and a one-line `desc`. The header carries a `max_steps` budget; the footer carries the ranking and the re-plan triggers. The committed G-12 ledger:

```
# PLAN for G-12  (term-budget: max_steps=8)  plan_hash:a1b2c3

| step | dep | mode     | status | desc |
|------|-----|----------|--------|------|
| S1   | -   | SERIES   | open   | collect rising AI-agent items (medium-tier band) |
| S2   | S1  | PARALLEL | open   | read reactions across youtube/reddit/x-tech (independent fan-out) |
| S3   | S2  | SERIES   | open   | apply hype-adjudication playbook: provenance-dedup → ≥N-origin gate |
| S4   | S3  | SERIES   | open   | draft brief via SCH_geo_link_react_brief re-bound to tech_builders |
| S5   | S4  | SERIES   | open   | compliance gate (sole predecessor of emit) → GREEN → emit |
| S6   | S5  | SERIES   | open   | reflect + consolidate (belief Δ, prediction, lesson) |

ranking (what strictly decreases each cycle): open steps remaining (6→0)
re-plan triggers: ledger-exhausted | step-blocked | metric off-track ×K | assumption-broke | budget-low
```

**Termination proof.** The ranking function is `open-steps-remaining`, a non-negative integer that strictly decreases every cycle (each cycle marks one ready step DONE). It starts at 6 and is bounded below by 0, so the plan provably halts in ≤6 cycles, and `max_steps=8` caps any replan-driven growth. A ledger without a strictly-decreasing ranking is rejected — emit nothing and treat as a fault.

**Compliance-gate ordering invariant.** The compliance-gate step (S5) MUST be the sole `dep` of any emit step. PLAN does not run the gate and does not decide clearance — PLAN only guarantees the ledger topology forces every emit to depend on the gate, so ACT cannot emit before the gate is GREEN. On G-12, S5 bundles gate→GREEN→emit and is the sole predecessor of consolidation S6.

### 3. SET-DIRECTION-ONCE — (d) TRANSFER guard: re-bind-then-CHEAP-PROBE first

If RECALL flagged TRANSFER (a new domain re-binding a schema), the **first committed step must be a re-bind-then-CHEAP-PROBE step, not the full run.** The full run commits only after Generalization's probe lift CI excludes zero.

On the seeded run, `tech_builders` was TRANSFERRED from `punjab_diaspora` via `SCH_geo_link_react_brief`. PLAN did not bolt the full weekly-brief run onto an unproven schema. Instead it spawned the precondition subgoal G-07 ("acquire competence in tech_builders, success = committed schema with probe lift CI excluding 0"). The cheap probe ran (12 held-out items, metric `link_validity@anchor`, pre-registered), returned lift +0.23 with 95% CI **[0.06, 0.39]** excluding 0, ablation holding at +0.19 with all punjab bindings dropped — recorded as **TRX_0007** in `schemas/transfer_log.md`. Only on that PASS did PLAN pop G-07 and let the full G-12 ledger (S4 "draft via SCH re-bound to tech_builders") proceed. Had the CI included 0, the no-go would have blocked S4 and forced a replan.

### 3. SET-DIRECTION-ONCE — (e) COMMIT and stop deliberating

Append the direction block to `goals/direction.md` with a `review_after` (for G-12: `2026-07-05 (weekly) and on defeater`), stamp the ledger's `plan_hash` (G-12: `a1b2c3`), and **stop weighing alternatives.** Once committed, the rejected list is frozen unless a defeater reopens it. Do not re-score `chase-the-biggest-trend` next turn "just to be sure" — that is the churn the phase exists to prevent.

### 6. EXECUTE-DON'T-REPLAN — hand ACT the next ready step

This is the steady-state branch (a committed direction already exists). **Do not regenerate the ledger.** Scan `goals/plan.md` top-to-bottom for the first step where ALL of:

1. `status` is `open`, AND
2. every `dep` step is `DONE`, AND
3. the step is not `blocked`.

Hand exactly **that one step** to ACT and end the seam. Hand off one step, not a batch — even PARALLEL fan-out is one ledger step (S2) that ACT expands internally into independent reads.

If **no step qualifies** (all DONE, or all remaining are blocked with no unblocked sibling), that absence is itself the `ledger-exhausted` / `all-blocked` defeater — proceed to step 6-defeater.

On a steady-state G-12 turn after S1–S2 are DONE, the first qualifying step is S3 (open, dep S2 DONE, not blocked); PLAN hands S3 to ACT and stops. It does not re-examine S1, does not re-open direction, does not re-rank.

### 6-defeater. DEFEATER CHECK — re-plan ONLY if a named defeater fired

Before any replan, confirm that exactly one of these *named* defeaters actually fired, and name it explicitly:

| defeater | fires when | seeded-run example |
|---|---|---|
| `ledger-exhausted` | no open step remains and the criterion is not met | would fire if S6 done but baseline not yet beaten at week 6 |
| `step-blocked` | a step is blocked and has no unblocked sibling to fall to | S4 blocked if TRX_0007 had returned no-go |
| `metric-off-track ×K` | the criterion metric misses K cycles running (K from the goal block; default K=2 for a weekly P1) | 2 consecutive weeks failing to beat the human-curated baseline |
| `assumption-broke` | a CONDITIONAL-KNOWN the plan depended on was falsified | C1 in `world/ai-agents.md` ("if search-interest independent of platform signal") proven false would undercut B2 and the honesty direction |
| `new-info-flips-best-direction` | fresh evidence makes a *rejected* alternative now dominate | builders demonstrably reward volume over signal (would refute A1) |
| `budget-low` | `fuel` or `max_steps` near zero | G-12 `fuel: 3/12` approaching 0 |

**No named defeater → no replan. Persist on the current ledger.** "I feel uncertain" is not a defeater. A replan **appends** a fresh committed-direction block to `goals/direction.md` (the prior block stays for audit) and emits a new ledger with a **new** `plan_hash` — never edits `a1b2c3` in place. Increment cannot reuse the old hash; auditors must be able to diff the two directions.

### 7. SERIES vs PARALLEL (applies whenever step 3 fans a step out)

Tag a step `PARALLEL` only if ALL hold:
1. the substeps are mutually independent (no data dependency), AND
2. none writes another's `kernel/blackboard/` block, kernel block, or `<env>` block (disjoint write targets), AND
3. they fit the budget (`max_steps`/`fuel`).

Otherwise `SERIES`. **Default to SERIES when unsure.** On G-12, S2 is PARALLEL because the youtube/reddit/x-tech reads are independent and each writes its own merge block under `kernel/blackboard/tech_builders/`; everything downstream of the dedup (S3→S6) is SERIES because each consumes the prior step's output.

### 8. PRECONDITION → SUBGOAL sweep

The moment setting direction or readying a step surfaces something PLAN lacks ("I lack competence in this domain", "I lack the origin count to draft honestly"), append a `SPAWN` line to `goals/subgoals.md` pushing the subgoal above the active goal with an explicit answerhood/done-test. POP it when the test is met. Two distinct shapes on the seeded run:

```
SPAWN G-07 from G-12 — open precondition "no competence in tech_builders domain" — push above G-12 (2026-06-24)
POP   G-07 — criterion met (TRX_0007 committed: probe lift CI [0.06,0.39] excludes 0) — resume G-12 (2026-06-26)
SPAWN OQ-07 (as question, handed to attention) from S3 of G-12 — "are the 2 origins one operator?" (2026-06-28)
```

- A **competence/precondition gap** spawns a real subgoal on the stack (G-07) that PLAN pops on its done-test.
- An **open question** (OQ-07) is spawned as a question and handed to Attention — PLAN does the spawning; choosing the question's value-of-information ranking belongs to Attention (`attention/questions.md`, where Q-OQ-07 sits at VoI 0.81). Never re-spawn anything already `SPAWN`-ed-open or already `POP`-ed in the log; scan first.

## Decision rules & edge cases

- **Missing record (not in INDEX.md):** treat as a non-existent precondition. Spawn a subgoal (step 8); never hallucinate the record's contents. Example: if `SCH-geo-link` had not been in INDEX, PLAN would have spawned "acquire/transfer the schema" rather than asserting a binding.
- **Ambiguous which step is "first ready":** ledger file order breaks ties — scan top-to-bottom, take the first qualifying row. Do not optimize ordering at execute time; ordering was fixed at commit.
- **Two qualifying steps that are independent:** they should already have been one PARALLEL ledger step if they were meant to run together. If they are genuinely separate SERIES steps both ready, hand the earlier (top-most) one; the next turn picks up the other.
- **Direction exists but the criterion looks wrong:** a wrong criterion is not a free edit. PLAN does not silently rewrite a committed direction. If new info shows the criterion is unfalsifiable or off, that is the `new-info-flips-best-direction` or `assumption-broke` defeater — append a corrected direction block, never overwrite.
- **Conflicting beliefs in RECALL pointers:** PLAN does not adjudicate belief confidence (Learning/Calibration own that). Plan to the belief as loaded; if the conflict is load-bearing for direction, spawn a question to Attention and proceed on the higher-confidence belief, noting the dependency so `assumption-broke` can fire later.
- **Budget low (`fuel`/`max_steps` near zero):** this is the `budget-low` defeater. Re-plan toward a cheaper direction or escalate; do not silently exceed `max_steps=8`.
- **Replan loop risk:** if defeaters fire on consecutive turns (>2 replans in one cycle without a step completing), stop appending directions and ESCALATE — the goal is likely mis-specified upstream.
- **TRANSFER without a probe result yet:** never commit the full run. First step is re-bind-then-CHEAP-PROBE; the full-run steps stay `open` and dep-blocked on the probe's PASS.
- **Compliance/clearance:** PLAN never decides it. PLAN only guarantees the gate step is the sole `dep` of every emit step. A STOP at the gate is ACT's call (ACT marks the step `blocked`); PLAN reacts to that block via the `step-blocked` defeater.

## Worked example (run R0631 / goal G-12)

**Turn entry (cursor):** `kernel/cursor.json` shows `turn:631, goal:G-12, plan_hash:a1b2c3, phase` handed from RECALL. RECALL loaded the focus and the hot pointers (W-ai-agents, A-builders, P-hype-adjud, SCH-geo-link) listed in `G-12.touches`.

**Step 1 — resolve goal.** Top-of-stack ACTIVE = **G-12** (parent G-01, `fuel: 3/12`, criterion "1 clearance-passing brief/week … beats human-curated baseline over 6 weeks").

**Step 2 — branch.** `goals/direction.md` already holds `## G-12 — committed 2026-06-14` → branch B (EXECUTE-DON'T-REPLAN). The SET-DIRECTION-ONCE work below is what PLAN did on 2026-06-14, shown for completeness.

**SET-DIRECTION-ONCE (as run on 2026-06-14):**
- Criterion = metric+threshold+horizon (the falsifiable line above), grounded in **A1** (.80) and **B2** (manufactured-hype, then .70, now **.74** after R0631).
- Direction = honest ≥N-origin adjudication; `because` = builders reward signal over noise.
- Rejected = `chase-the-biggest-trend` (over-saturated, no moat) and `post-daily-volume` (dilutes the honesty signal).
- Ledger emitted with `max_steps=8`, `plan_hash:a1b2c3`, ranking `open-steps-remaining 6→0`.
- **TRANSFER guard:** because `tech_builders` was transferred from `punjab_diaspora`, PLAN spawned **G-07** (competence subgoal) before the full run. The cheap probe (`TRX_0007`) returned lift +0.23, CI **[0.06, 0.39]** excluding 0, ablation +0.19 → COMMIT, `schema.confidence[tech_builders]=0.71`. PLAN popped G-07 on 2026-06-26 and let S4 (draft via the re-bound schema) proceed.

**Branch B execution across R0631:** PLAN walked the ledger one ready step per cycle — S1 (collect) → S2 (PARALLEL fan-out across youtube/reddit/x-tech, independent reads to disjoint blackboard merge blocks) → S3 (apply `P-hype-adjud`: provenance-dedup → ≥N-origin gate, N=3) → S4 (draft via `SCH_geo_link_react_brief` tech binding) → S5 (compliance gate, GREEN, emit) → S6 (reflect+consolidate). At each cycle PLAN handed ACT exactly the first open step with deps DONE and did not re-deliberate.

**Defeater that did NOT fire a replan.** At S3, prediction **P-114** ("a cross-platform wave this size has ≥3 independent origins", conf .65) was **refuted** — the surface 4-platform wave deduped to **2 origins** (one reposted seed), logged as surprise **S-009** (magnitude .78). This is Calibration's refutation, not PLAN's. It did **not** fire a PLAN defeater: the ledger was not exhausted, no step was blocked, the criterion metric was not off-track K times, and no *direction-bearing* assumption broke — in fact the result **confirmed** B2 (manufactured-hype) and the honest-adjudication direction. So PLAN correctly **persisted** on `a1b2c3` rather than appending a new direction. What it *did* do: at step 8 it spawned **OQ-07** ("are the 2 origins one operator?") as a question handed to Attention (now Q-OQ-07 at VoI 0.81 in `attention/questions.md`), because the brief's honesty needed the funding trace.

**What PLAN did NOT write.** Belief **B2** moving .70→.74 (Learning, `semantic/world/ai-agents.md`); prediction **P-114** refutation and **P-118** ("re-spikes within 14 days", conf .35, open) (Calibration, `predictions.jsonl`); candidate lesson **L-23** ("≥3 platforms agreeing ≠ ≥3 origins — dedup BEFORE counting", seen 1×) (Learning, `procedural/lessons.md`); the compliance GREEN at S5 (ACT). PLAN touched only `direction.md`, `plan.md`, `subgoals.md`.

## Failure modes & escalation

- **Re-deliberating a committed direction** (re-scoring rejected alternatives every turn) → churn, no ship. Guard: branch B forbids regenerating the ledger absent a named defeater.
- **Persisting through a broken assumption** (ignoring a falsified CONDITIONAL-KNOWN like C1) → riding a dead direction. Guard: the `assumption-broke` defeater must be checked against the pointers RECALL loaded.
- **Overwriting a prior direction or `plan_hash`** → destroyed audit trail. Hard rule: replans APPEND with a new hash; old blocks are immutable.
- **Emitting a ledger without a strictly-decreasing ranking** → no termination proof. Reject and re-emit.
- **Emit step not gated** (an emit step whose sole `dep` is not the compliance step) → unsafe-publish topology. Reject the ledger before commit.
- **Hallucinating a record not in INDEX.md** → fabricated plan basis. Hard rule: not in INDEX ⇒ spawn a subgoal.
- **Writing a sibling faculty's file** (a belief, a prediction, a goal-status flip, a clearance verdict) → boundary breach. Hard rule: PLAN writes only its three files.

**ESCALATE to a human when:** (1) the goal block is structurally invalid (two ACTIVE goals, missing `success_criterion`, or a criterion that cannot be made falsifiable); (2) defeaters fire on >2 consecutive turns in one cycle without a step completing (likely mis-specified goal upstream); (3) `budget-low` fires with no cheaper direction available and the criterion still unmet; (4) the compliance-gate step would have to be removed or bypassed to make the ledger runnable. Escalation means stop, name the condition, and hand back to Agency's goal-stack owner / a human — never silently relax a constraint.

## Handoff

PLAN ends by handing **ACT** (the next workflow, step 4 of the loop) exactly one artifact: the chosen next-ready ledger step (on a steady G-12 turn, e.g. S3), or an escalation. The state PLAN must leave behind for ACT and the rest of the loop:

- `goals/direction.md` — one committed direction block for the active goal (appended, never overwritten), with `success_criterion`, `rejected:`, and `review_after`.
- `goals/plan.md` — a ledger with a stamped `plan_hash`, every step carrying `dep`/`mode`/`status`/`desc`, a `max_steps` budget, a strictly-decreasing `ranking`, the `re-plan triggers` list, and the emit step gated solely on the compliance step.
- `goals/subgoals.md` — any new `SPAWN`/`POP` lines appended this turn, with done-tests.
- The single next-ready step identified for ACT, or a named escalation.

ACT executes that one step (including running the compliance gate and any playbook), then REFLECT/Learning scores outcomes and writes beliefs/lessons, and CONSOLIDATE updates tiers — none of which PLAN performs. The next PLAN seam re-enters at step 1 and, finding the direction already committed, takes branch B again unless a defeater has fired.

## Single-write-owner contract

**PLAN (`mind_plan`) MAY write exactly these three files, and no others:**

- `goals/direction.md` (committed direction + rejected alternatives; append-only per commit)
- `goals/plan.md` (the step ledger; each step SERIES/PARALLEL + `max_steps` budget + ranking + `plan_hash`)
- `goals/subgoals.md` (append-only spawn/pop audit)

**PLAN MUST NEVER touch:**

- `goals/goals.md` goal `status` (the goal-stack owner flips ACTIVE/DONE/STANDING)
- `attention/*` (questions, surprises, explore_log, inquiry_budget — Attention owns VoI and the queue; PLAN only *spawns* a question for Attention to rank)
- `calibration/*` (predictions.jsonl, recalibration_map, report — Calibration owns bets like P-114/P-118)
- `experience/*`, `episodic/*` (REFLECT/Learning own episodes, outcomes, reflections)
- `schemas/*` (Generalization owns SCH_geo_link_react_brief, transfer_log, abstractions; PLAN reads the binding, never edits it)
- `semantic/*` (Learning owns world/ai-agents beliefs B1–B3, audiences/builders A1–A3, glossary)
- `procedural/*` (Learning owns playbooks and lessons L-23)
- `kernel/*`, `_meta/*` (the kernel/scheduler and consolidation own these)

PLAN never executes a step, never makes a calibrated prediction, never adjudicates hype or runs a playbook, never scores an outcome or writes a lesson, and never makes the compliance/safety/clearance decision. When the right move is outside PLAN, PLAN hands it off rather than reaching across the boundary.
