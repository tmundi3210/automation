# W6_consolidate — CONSOLIDATE — write memory, checkpoint, and the scheduled "sleep"

> Close the turn by WRITING everything to durable memory and re-arming the next wake-up. Owner specialist: `mind_consolidate`. Loop position: step 6 of 6, after PERCEIVE → RECALL → PLAN → ACT → REFLECT have run; their output (scored predictions, the candidate belief move, the candidate lesson, the working blackboard) is this phase's input. Single-write-owner files: `episodic/journal/<today>.md`, `episodic/timeline.md`, `kernel/checkpoint.jsonl`, `kernel/cursor.json`, `INDEX.md`, `kernel/schedule.jsonl` — and, ONLY inside a due sleep pass: `semantic/*`, `procedural/lessons.md`, `procedural/playbooks/*`, `episodic/_archive/*`, `calibration/recalibration_map.json`, `_meta/consolidation_log.md`.

## Purpose

CONSOLIDATE is the diarist, checkpointer, and sleeper. It is a scribe and scheduler, not a thinker. Its job is to make the turn *durable and resumable*: a future turn — same session or days later — must be able to reconstruct every decision from the consolidated store alone. It does four things every turn (diary, timeline, checkpoint+cursor, INDEX), then re-arms the schedule. On the subset of turns where a `consolidate` or `recalibrate` wake-up in `kernel/schedule.jsonl` is due *now*, it additionally runs that "sleep" pass: it folds the diary's proposed belief moves into `semantic/`, promotes lessons seen at least twice into `procedural/`, archives aged journal days, rewrites the recalibration map if a band is miscalibrated, and appends a pass record to `_meta/consolidation_log.md`.

The hard separation that defines this phase: on a **normal turn** the diary's `BELIEF Δ` and `LESSON?` lines are **proposals only** — CONSOLIDATE writes them into the journal but does **not** touch `semantic/` or `procedural/`. Only the scheduled sleep applies them. This keeps belief revision deliberate, batched, and auditable instead of happening silently inside every turn.

CONSOLIDATE never invents a finding, belief, prediction, or lesson. It transcribes what REFLECT produced and applies what the calibration ledger already scored. It does not make the goal-lifecycle call (DONE/ABANDONED/ESCALATED belongs to agency/goals), and it does not make the safety/compliance call (the gate at `../GATE_STEP2.md` is owned elsewhere); it only records the gate's GREEN/RED result.

## Inputs (memory read) and Outputs (memory written)

Reads are read-mostly — open them to orient, do not modify them here (they belong to sibling faculties).

**Reads — orientation (every turn):**
- `branches/b60_content_intelligence/mind/memory/INDEX.md` — NOW block (run, env, goal, phase, step k/n, last_run outcome, open_q), TIERS counts, HOT POINTERS, CONSOLIDATION DUE block.
- `branches/b60_content_intelligence/mind/memory/kernel/cursor.json` — where the turn left off (`env_id`, `turn`, `active_step_id`, `phase`, `plan_hash`, `goal`).
- `branches/b60_content_intelligence/mind/memory/kernel/checkpoint.jsonl` — last line's `next_step` (the step to resolve done) and `plan_hash`.
- `branches/b60_content_intelligence/mind/memory/kernel/schedule.jsonl` — every future wake-up; decides whether a `consolidate`/`recalibrate` pass is due now.
- `branches/b60_content_intelligence/mind/memory/_meta/retention.md` — the keep/archive/drop policy and the consolidation invariants.
- `branches/b60_content_intelligence/mind/memory/goals/plan.md` — to locate the just-finished step row and read its `plan_hash`.

**Reads — REFLECT/RECALL handoff (every turn):**
- REFLECT's output for this turn: the scored predictions, the candidate `BELIEF Δ`, the candidate `LESSON?`, the run OUTCOME and the recorded gate result. In the seeded run this is mirrored in `branches/b60_content_intelligence/mind/memory/experience/reflections/e0631.md`.
- RECALL's recorded LOADED slice (what was pulled at the start of this turn) — copied verbatim into the diary's LOADED line. In the seeded run this is the `recalled` array of the `kernel/turn_log.jsonl` line for turn 631.
- `branches/b60_content_intelligence/mind/memory/calibration/predictions.jsonl` — read-only here, to copy the scored/open prediction ids and outcomes into PROV (P-114 refuted, P-118 open). CONSOLIDATE never writes this file; the calibration faculty owns it.

**Reads — sleep pass only (when a consolidate/recalibrate wake-up is due):**
- `branches/b60_content_intelligence/mind/memory/semantic/world/ai-agents.md`, `semantic/audiences/builders.md`, `semantic/glossary.md` — the records the diary's belief deltas fold into.
- `branches/b60_content_intelligence/mind/memory/procedural/lessons.md` — the candidate/trusted lesson ledger.
- `branches/b60_content_intelligence/mind/memory/calibration/recalibration_map.json` — the current bands and version, for a recalibrate pass.

**Writes — single-write-owner, EVERY turn:**
- `branches/b60_content_intelligence/mind/memory/episodic/journal/2026-06-28.md` (today's file) — append ONE run block with all ten mandatory lines.
- `branches/b60_content_intelligence/mind/memory/episodic/timeline.md` — append one pipe-delimited line.
- `branches/b60_content_intelligence/mind/memory/kernel/checkpoint.jsonl` — append one done line.
- `branches/b60_content_intelligence/mind/memory/kernel/cursor.json` — rewrite in place to advance to next step or DONE.
- `branches/b60_content_intelligence/mind/memory/INDEX.md` — rewrite in place (NOW, TIERS, HOT POINTERS, CONSOLIDATION DUE).
- `branches/b60_content_intelligence/mind/memory/kernel/schedule.jsonl` — append/refresh the next wake-up(s).

**Writes — single-write-owner, SLEEP PASS ONLY:**
- `branches/b60_content_intelligence/mind/memory/semantic/world/ai-agents.md` and `semantic/audiences/builders.md` (and `semantic/glossary.md` if a term is consolidated) — fold belief deltas.
- `branches/b60_content_intelligence/mind/memory/procedural/lessons.md` — promote lessons seen ≥2×; on promotion also `procedural/playbooks/<id>.md`.
- `branches/b60_content_intelligence/mind/memory/episodic/_archive/<period>.md` — archive aged journal days on a weekly-deep pass.
- `branches/b60_content_intelligence/mind/memory/calibration/recalibration_map.json` — rewrite ONLY if a band is miscalibrated by the scored ledger.
- `branches/b60_content_intelligence/mind/memory/_meta/consolidation_log.md` — append one structured pass record.

**One-step write order, also the toggle in the agency ledger:**
- `branches/b60_content_intelligence/mind/memory/goals/plan.md` — toggle the finished step's `status` field to DONE. This is the one cell CONSOLIDATE may touch in agency's file; it never rewrites the ledger structure, rankings, re-plan triggers, or any other field.

## Protocol

Run these steps in order. Every turn does steps 1–6 and step 11. Steps 7–10 run only inside a due sleep pass.

### 1. Orient: fix run, env, goal, finished step, plan_hash, and whether a sleep is due

Read `kernel/cursor.json`, the NOW and CONSOLIDATION DUE blocks of `INDEX.md`, `kernel/schedule.jsonl`, and the **last line** of `kernel/checkpoint.jsonl`. Extract:
- `run` (e.g. R0631), `env_id` (e.g. tech_builders), `goal` (e.g. G-12), `plan_hash` (e.g. a1b2c3).
- the finished `step_id` — it is the `next_step` of the last checkpoint line *before this turn*, i.e. the step ACT/REFLECT just completed (S6 in the seeded run).
- the resolved next step — read it from `goals/plan.md` dependency chain (the step whose `dep` is the finished step), or `DONE` if the finished step was the terminal step.

Then check the schedule for a **due** sleep: a row with `kind` in {`consolidate`, `recalibrate`} whose `due` timestamp is ≤ now (treat the turn's wall-clock `ts` as now). A `recheck` row is NOT a sleep — it schedules a future *goal* turn, not a consolidation pass, so it never triggers steps 7–10. Record the verdict as a single boolean: **sleep-due = yes/no** and, if yes, the matching wake-up id and sub-kind (daily-light vs weekly-deep for consolidate).

Also pull from the REFLECT handoff: the scored predictions (ids + outcome + brier), the candidate `BELIEF Δ`, the candidate `LESSON?` (with its seen-count), the run OUTCOME, and the gate result (GREEN/RED). Pull the RECALL LOADED slice verbatim — do not paraphrase it.

If any orientation read is missing or malformed, follow Decision rules & edge cases below before writing.

### 2. Write the diary entry (append ONE run block, all ten mandatory lines)

Append to `episodic/journal/<today>.md` (today = `2026-06-28.md` for the seeded run) exactly one `## RUN <run>` block under the day header. Every one of the ten lines MUST appear, in this order. Write the literal word `none` on any line that is empty — never omit a line, never collapse two lines into one.

The ten mandatory lines:
1. `SET-OUT` — the turn's intent in one line, transcribed from PLAN's focus/direction.
2. `LOADED` — the RECALL slice **verbatim**: the INDEX plus the exact pointer ids pulled, with their confidences where RECALL recorded them. This line is the audit trail for retrieval; do not summarize or reorder it.
3. `DID` — the actions ACT took, in sequence.
4. `FOUND` — the finding, transcribed from REFLECT. Never author a new finding here.
5. `BELIEF Δ` — the candidate belief move as a **PROPOSAL**: name the record, the belief id, the old→new confidence, and why (confirmed/refuted by which prediction). Mark unchanged beliefs explicitly. This line does not modify `semantic/`; the sleep applies it.
6. `PREDICT` — the scored prediction(s) plus any new open prediction: id, the scored outcome (refuted/confirmed) with its post-score state, and the new open prediction with its probability and due date.
7. `ASSUMED` — the conditional-knowns relied on this turn (e.g. C1), each with its revisit condition.
8. `LESSON?` — the candidate lesson as a **PROPOSAL**: id, one-line rule, and the independent seen-count (e.g. 1×). Promotion is the sleep's job, not this line's.
9. `OUTCOME` — the run result (EMITTED/HELD/etc.), the gate result recorded (not decided) here, and the next watch item / next wake-up.
10. `PROV` — provenance: source ids, model, thresholds with version, the prediction ledger ids touched, and the schema id used.

Formatting rule: keep each line on a single physical line prefixed by its label and a colon, matching the existing block in `episodic/journal/2026-06-28.md`. The block is **append-only** — never edit a prior run's block in the same file.

### 3. Append one timeline line (the spine)

Append exactly one pipe-delimited row to `episodic/timeline.md` under the existing header, in the column order `date | run | goal | outcome | Δsemantic`. The `Δsemantic` cell is the *proposed* delta described compactly (belief move + open prediction + candidate lesson). One run = one line; never rewrite a prior line. This file is the cheapest reconstruction index, so the outcome and delta must be legible at a glance.

### 4. Toggle the finished step done in `goals/plan.md`, then checkpoint and advance the cursor

Three writes, in this order:
1. **plan.md toggle.** In `goals/plan.md`, change only the `status` cell of the finished step's row to `DONE`. Touch nothing else — not the desc, dep, mode, ranking, or re-plan triggers. If the step is already `DONE` (re-run/idempotency), leave it; do not double-toggle.
2. **checkpoint append.** Append one line to `kernel/checkpoint.jsonl` recording `step_id`, `env_id`, `status:"done"`, the resolved `next_step` (a step id or `DONE`), `plan_hash`, and `ts`. This file is the crash-safe source of truth for resume; it is append-only and must never be rewritten.
3. **cursor advance.** Rewrite `kernel/cursor.json` in place so `active_step_id` becomes the resolved next step (or `DONE` if the plan is complete), set `phase` to the next phase the resumer should enter, bump/keep `turn`, keep `plan_hash` and `goal`, and write a `note` describing what the next turn picks up and `updated` to the turn ts. The cursor is the single resume pointer — it must agree with the last checkpoint line.

### 5. Rewrite `INDEX.md` in place (NOW · TIERS · HOT POINTERS · CONSOLIDATION DUE)

Rewrite the four blocks so the INDEX stays honest — anything not pointered is effectively forgotten until a later consolidation re-surfaces it.
- **Header** — bump `updated`, set `run`, `env`.
- **NOW** — goal line, `phase`/`step k/n done`, `last_run` (the run just finished with its outcome and Δsemantic), `open_q` (the top open question id, e.g. OQ-07), and the transfer/provenance note if still relevant.
- **TIERS** — counts per tier and the last-consolidated date per consolidating tier. On a normal turn the semantic/procedural last-consolidated dates do **not** move (no fold happened); they move only after a sleep pass (step 9).
- **HOT POINTERS** — id → file · one-line why · confidence. Add a pointer for any record created this turn (e.g. a new semantic record created empty-but-structured). Drop a pointer for any record archived this turn. On a normal turn confidences shown here reflect the *consolidated* store, not the proposal — do not bump a pointer's confidence from a proposal that the sleep has not yet applied.
- **CONSOLIDATION DUE** — next pass dates (daily-light, weekly-deep, recalibrate) read from the schedule, the backlog count of unconsolidated runs vs threshold 7, and bloat status (OK / a named tier over its bound). Increment backlog by 1 on a normal turn; on a sleep pass set it as step 9 dictates.

### 6. Branch on the schedule

Re-read the sleep-due verdict from step 1.
- **If NO consolidate/recalibrate wake-up is due now:** the turn is a normal turn. Proceed to step 11 (re-arm). **Do not** edit `semantic/` or `procedural/`. The `BELIEF Δ` and `LESSON?` lines remain proposals carried in the journal for the next sleep to apply. Skip steps 7–10.
- **If a `consolidate` wake-up IS due now:** run steps 7 (always) and 8-skip, then 9–10. Daily-light runs the fold + lesson check; weekly-deep additionally runs archive + round-trip.
- **If a `recalibrate` wake-up IS due now:** run step 8, then 9–10. (A turn may have both a consolidate and a recalibrate row due; run 7 then 8 then 9–10.)

### 7. Consolidate pass — fold belief deltas, check lesson promotion, (weekly) archive + round-trip

For **each unconsolidated diary `BELIEF Δ`** since the last fold (the backlog), apply it to its matching `semantic/world/*` or `semantic/audiences/*` record:
- **Move confidence by the calibration OUTCOME, never by restatement.** A belief gains/loses confidence because a prediction that tested it was scored, not because the diary mentioned it again. Re-stating an already-folded belief with no new scored outcome moves nothing.
- **Bump runs-seen and extend provenance** — append the run id to the belief's provenance list and update the record's `runs-seen` and `updated` header.
- **Supersede, never delete, a retired belief.** Move a retired claim to the SUPERSEDED block with a strike-through and a `retired <run>: <reason>` note. The auditable-revision invariant in `_meta/retention.md` forbids deletion.
- **Flag aleatory items DO-NOT-CHASE** so they are never re-opened for learning. An aleatory (irreducible-noise) belief keeps its `—` confidence and its DO-NOT-CHASE tag; never schedule a recheck against it.
- **Promote a conditional-known to a belief only if its condition was verified** this period; otherwise leave it UNVERIFIED and pointed at its open question.

For **lesson promotion**: a `LESSON?` candidate promotes into `procedural/lessons.md` TRUSTED only when it has been **seen ≥2× independently** (two distinct episodes, not two mentions of one episode). On promotion *and* once the lesson has a confidence interval that excludes zero, write a `procedural/playbooks/<id>.md` entry with TRIGGER / MOVE / EVIDENCE / SCOPE-FALSIFIES. A candidate at 1× stays on probation — record `promoted: none`.

For a **weekly-deep** pass only:
- **Archive** journal days older than the audit window into `episodic/_archive/<period>.md`, leaving the timeline spine and the consolidated beliefs in the hot path. Drop a HOT POINTER for anything moved out of the hot path; keep a recoverable archive pointer.
- **Round-trip check.** Ask: could the next run reconstruct every decision from the consolidated store alone (INDEX + semantic + procedural + timeline + ledger), without the archived raw days? If anything must-preserve would be lost, keep it (lift it into semantic/procedural or the timeline before archiving). Collapse duplicate beliefs that say the same thing into one with merged provenance.

### 8. Recalibrate pass — rewrite the map only if a band is miscalibrated

On a `recalibrate` wake-up: recompute the confidence bands from the **scored** prediction ledger over the policy window. Rewrite `calibration/recalibration_map.json` **only if** a band is miscalibrated by the scored ledger (e.g. the 80–90 bands show systematic overconfidence). On a rewrite, bump the version (e.g. `recal-v2` → `recal-v3`), update `fit_ts`, `n_scored`, `brier`, and the `shrink_rule`, and set the next refit due date. If every band is within tolerance, **leave the file unchanged** and record "no change" in the pass log — do not bump the version for a no-op.

### 9. Close the sleep — log the pass and refresh the INDEX

Append one structured record to `_meta/consolidation_log.md` (append-only) with: timestamp, sub-kind (daily-light / weekly-deep / recalibrate), `runs folded`, `merged` (each belief move applied), `promoted` (lessons promoted, or none), `archived` (journal days moved, or none), `dropped` (working scratch / pure restatements), and `backlog reset to 0`. Then refresh `INDEX.md`: update the TIERS last-consolidated dates to today, set HOT POINTER confidences to the now-applied values, and reset the CONSOLIDATION DUE backlog to 0 with the next pass dates.

### 10. (covered above — recalibrate logging folds into step 9's pass record)

A recalibrate pass writes its own line to the consolidation log: version old→new (or "unchanged"), `n_scored`, new brier, and which band moved. Same append-only file.

### 11. Re-arm `kernel/schedule.jsonl` — leave no orphaned or duplicate rows

Always the last action of the turn. Roll each **consumed** recurring wake-up forward by its recurrence and append/refresh the next occurrence:
- nightly `consolidate` (daily-light) → next nightly `due`.
- weekly `consolidate` (weekly-deep + archive) → next weekly `due`.
- weekly `recalibrate` → next weekly `due`.
Add any `recheck` a due prediction implies — when a prediction's `due_date` is set, schedule a `recheck` wake-up for that date so a future turn scores it. Remove duplicates and orphans: never leave two rows for the same occurrence, and never leave a row whose goal/prediction no longer exists. On a normal turn where no sleep fired, you still re-arm only if the nightly consolidate row is missing or stale; otherwise leave the schedule as is. End the turn.

## Decision rules & edge cases

- **Empty mandatory line.** Any of the ten diary lines with no content is written as the literal `none`. Never omit the line, never merge it upward.
- **Proposal vs application.** On a normal turn, `BELIEF Δ` and `LESSON?` are proposals; `semantic/` and `procedural/` are untouched and the INDEX shows pre-fold confidences. Only a due sleep applies them. If you find yourself editing `semantic/` outside a sleep, stop — that is a contract violation.
- **No scored outcome → no belief move.** During a fold, if a belief's `BELIEF Δ` has no backing scored prediction (restatement only), do not move its confidence; bump provenance/runs-seen at most. Confidence tracks calibration, not mention frequency.
- **Lesson at 1×.** A candidate seen once stays on probation; promotion requires ≥2 independent confirms. `promoted: none`.
- **Missing record the INDEX does not list.** When a write you owe needs a file not in the INDEX, create it **empty-but-structured** (correct header + empty sections) and add a HOT POINTER for it. Never fabricate its prior contents.
- **Cursor / checkpoint disagreement.** The last checkpoint line is the source of truth for the finished step and next_step; reconcile the cursor to it. If they cannot be reconciled, do not advance — escalate (see Failure modes).
- **Plan already DONE / re-run idempotency.** If the finished step is already `DONE` and a matching checkpoint line already exists for this turn, do not double-write; the turn is already consolidated. Re-arm only.
- **Backlog ≥ 7 or a tier over its bound.** `_meta/retention.md` allows running a consolidation pass early when unconsolidated runs reach the threshold (7) or a tier exceeds its size bound. If the backlog hits 7 on a normal turn and no sleep is due, append an early `consolidate` wake-up due now and run it this turn rather than letting bloat grow.
- **Gate RED.** Record the RED result in OUTCOME and PROV exactly as REFLECT reported it; still write the diary, timeline, checkpoint, cursor, INDEX. Do not route around the gate, do not emit, do not flip RED to GREEN. The gate decision is owned at `../GATE_STEP2.md`.
- **Goal lifecycle.** CONSOLIDATE never writes DONE/ABANDONED/ESCALATED into `goals/goals.md` or `goals/direction.md`. It records the step-level `status:done` in `plan.md` and the run outcome in the diary/timeline; the goal-level verdict is agency's call on a later turn.
- **Aleatory items.** Never schedule a recheck against a DO-NOT-CHASE belief and never "learn" from its run-to-run noise.
- **Recalibrate no-op.** If no band is miscalibrated, leave `recalibration_map.json` byte-for-byte unchanged and log "unchanged"; do not bump the version.
- **Conflicting belief deltas in the backlog.** If two unconsolidated diary entries propose opposite moves on the same belief, fold them in run order and let the later scored outcome win; record both in the log's `merged` line so the revision stays auditable.

## Worked example (run R0631 / goal G-12)

Context at turn start (read in step 1): `kernel/cursor.json` is at the step that ACT/REFLECT just finished; the last `kernel/checkpoint.jsonl` line before this turn resolved `next_step:S6`; `goals/plan.md` row S6 is the terminal step ("reflect + consolidate"); `plan_hash:a1b2c3`; run R0631; env tech_builders (TRANSFERRED from punjab_diaspora via SCH_geo_link_react_brief, TRX_0007); goal G-12 "weekly brief: AI-agent hype vs organic". REFLECT handoff (mirrored in `experience/reflections/e0631.md`): finding = a 4-platform surface deduped to only 2 independent origins, 2 < N=3 → MANUFACTURED; prediction P-114 (≥3 independent origins, conf .65, band 70) REFUTED with brier 0.42; new prediction P-118 (re-spike within 14d) opened at p=0.35, band 50, due 2026-07-12; belief move proposal B2 .70→.74 confirmed; candidate lesson L-23 seen 1×; OUTCOME EMITTED brief b0631, gate GREEN. RECALL LOADED slice (from the turn_log `recalled` array): INDEX, focus, W-ai-agents(.68), A-builders(.80), P-hype-adjud, SCH-geo-link, OQ-07.

**Step 2 — diary.** Append the `## RUN R0631` block to `episodic/journal/2026-06-28.md` with all ten lines, copying the LOADED slice verbatim. The mandatory lines resolve to: SET-OUT = produce the weekly tech_builders brief and decide genuine vs manufactured (≥N origins); LOADED = the verbatim slice above; DID = collected the rising item, read reactions across youtube/reddit/x-tech in parallel, provenance-deduped origins; FOUND = surface looked like a 4-platform wave, deduped to 2 independent origins, 2 < N=3 → MANUFACTURED, one "cross-platform" wave was a single reposted seed; BELIEF Δ = W-ai-agents B2 .70→.74 confirmed (proposal), B1 unchanged; PREDICT = P-114 refuted, P-118 open p=0.35 due 2026-07-12; ASSUMED = C1 (search-interest treated as semi-independent of platform signal, revisit if it correlates); LESSON? = L-23 "≥3 platforms agreeing ≠ ≥3 origins — dedup BEFORE counting" seen 1×; OUTCOME = EMITTED b0631 (gate GREEN), next watch OQ-07, re-run G-12 in 7d (wk_024); PROV = sources s1..s14, model opus-4-8, thresholds N=3@v2, ledger P-114(refuted)/P-118(open), schema SCH_geo_link_react_brief@tech. No empty lines here, so no `none` is needed this turn.

**Step 3 — timeline.** Append to `episodic/timeline.md`: `2026-06-28 | R0631 | G-12 | EMITTED (manufactured-hype call) | B2 .70→.74; P-118 open; L-23 candidate`.

**Step 4 — plan/checkpoint/cursor.** Toggle `goals/plan.md` row S6 `status` to DONE (only that cell). Append to `kernel/checkpoint.jsonl`: `{"step_id":"S6","env_id":"tech_builders","status":"done","next_step":"DONE","plan_hash":"a1b2c3","ts":"2026-06-28T14:02Z"}`. Rewrite `kernel/cursor.json` to `active_step_id:"DONE"`, with a note that R0631 is complete and consolidated and the next turn picks up wk_024/wk_025 or a new goal.

**Step 5 — INDEX.** Rewrite NOW (goal G-12, phase consolidate step 6/6 done, last_run R0631 EMITTED Δsemantic +1 world proposed, open_q OQ-07, transfer note), TIERS counts, HOT POINTERS (W-ai-agents, A-builders, P-hype-adjud, SCH-geo-link, G-12 — confidences at the *pre-fold* values until a sleep applies them), and CONSOLIDATION DUE.

**Step 6 — branch.** The schedule's `consolidate` rows wk_021 (daily-light, due 2026-06-29T03:00Z) and wk_022 (weekly-deep, due 2026-07-05) and recalibrate wk_023 (due 2026-07-12) are all in the future relative to this turn's 14:02Z. **No sleep is due now.** So B2's move and L-23 stay proposals; `semantic/world/ai-agents.md` and `procedural/lessons.md` are NOT edited this turn. (The seeded store reflects a daily-light pass that ran at 14:02Z and folded R0631 — that is the *next* turn's sleep producing B2 at .74 in `semantic/world/ai-agents.md`, L-23 staying at 1× in `procedural/lessons.md` with `promoted: none`, and the `2026-06-28T14:02Z · daily-light` record in `_meta/consolidation_log.md`. On the sleep turn, steps 7 and 9 produce exactly that: fold B2 .70→.74 moved by P-114's scored outcome, append R0631 to B2 provenance, leave L-23 on probation, log runs-folded R0631 / merged B2 / promoted none / archived none / dropped working scratch / backlog reset to 0.)

**Step 8 — would the recalibrate run?** Not on this turn (wk_023 is 2026-07-12). When it does, recompute bands from the scored ledger; P-114's brier 0.42 in band 70 plus the prior `note` ("mild overconfidence in the 80-90 bands") decides whether `recalibration_map.json` is rewritten from `recal-v2` to `recal-v3` or left unchanged.

**Step 11 — re-arm.** Confirm wk_021/wk_022/wk_023 are present and not stale; ensure the recheck wk_024 (re-run G-12 in 7d) and wk_025 (P-118 due 2026-07-12) exist so P-118 gets scored. P-118's `due_date` 2026-07-12 is exactly the recheck wk_025 implies. No duplicates, no orphans. End the turn.

## Failure modes & escalation

- **Missing or malformed orientation read** (cursor, last checkpoint, or INDEX NOW unreadable) → do not guess the finished step or next_step. Write the diary/timeline from the REFLECT handoff if available, but do not advance the cursor on a fabricated step. ESCALATE to a human with the exact file and the conflict.
- **Checkpoint ⇄ cursor disagreement that cannot be reconciled** → the resume pointer would be wrong. Halt the advance, leave the last good checkpoint untouched, ESCALATE.
- **A belief move with no backing scored prediction proposed for a fold** → refuse to move confidence; log it as restatement-only. If REFLECT insists on a move with no ledger backing, that is a thinking-vs-scribe boundary breach — record it and ESCALATE rather than inventing calibration.
- **Lesson promotion requested at <2× or without a CI excluding zero** → keep on probation; do not write the playbook. Promoting prematurely corrupts `procedural/`.
- **Gate result ambiguous or absent** → do not assume GREEN. Record the run as HELD pending gate, ESCALATE; never emit on an unrecorded gate.
- **Round-trip check fails on a weekly-deep pass** (a decision cannot be reconstructed from the consolidated store) → do not archive the day; lift the must-preserve content into semantic/procedural/timeline first. If it cannot be lifted without inventing content, ESCALATE.
- **Schedule would be left with orphans/duplicates and cannot be cleaned deterministically** → re-arm conservatively (keep the recurring rows, drop nothing you are unsure about) and flag the ambiguity for a human.
- **Backlog far over threshold with no sleep due and an early pass cannot be authorized** → the store is bloating and beliefs are going stale; ESCALATE so a human can authorize an out-of-band consolidation.
- **Asked to write a sibling-owned file** (`goals/goals.md`, `goals/direction.md`, `attention/*`, `calibration/predictions.jsonl`, `experience/*`, `schemas/*`) → refuse; that is a contract violation. Record the request and ESCALATE.

## Handoff

CONSOLIDATE ends the turn. The next workflow to run is the **next turn's PERCEIVE** (W1), driven by whatever this phase left in `kernel/cursor.json` and `kernel/schedule.jsonl`.

State this phase must leave behind for the next turn to resume cleanly:
- `kernel/cursor.json` points at the resolved next step (or `DONE`) with a `note` saying what to pick up — for R0631, that the run is complete and consolidated and the next turn picks up wk_024/wk_025 or a new goal.
- `kernel/checkpoint.jsonl` has the appended done line agreeing with the cursor (crash-safe resume truth).
- `kernel/schedule.jsonl` has the re-armed wake-ups: nightly consolidate (wk_021), weekly deep (wk_022), weekly recalibrate (wk_023), and the rechecks the open predictions imply (wk_024 re-run G-12, wk_025 P-118 due) — no orphans, no duplicates. If a `consolidate`/`recalibrate` row is due on a future turn, that turn's CONSOLIDATE will run the sleep (steps 7–10) and apply the proposals this turn deferred.
- `INDEX.md` is honest: NOW, TIERS, HOT POINTERS, and CONSOLIDATION DUE all reflect the post-turn state; any record created this turn is pointered, any archived record is dropped from HOT POINTERS.
- `episodic/journal/<today>.md` and `episodic/timeline.md` carry the full, append-only record of the turn so the next run can reconstruct it.

If a sleep ran, it additionally leaves: applied beliefs in `semantic/*`, promoted lessons in `procedural/*` (or probation unchanged), archived days in `episodic/_archive/*`, a possibly-rewritten `calibration/recalibration_map.json`, a fresh `_meta/consolidation_log.md` record, and a backlog reset to 0 in the INDEX.

## Single-write-owner contract

**MAY write, every turn (single-write-owner):**
- `episodic/journal/<today>.md` (append one run block, all ten lines)
- `episodic/timeline.md` (append one line)
- `kernel/checkpoint.jsonl` (append one done line)
- `kernel/cursor.json` (rewrite to advance)
- `INDEX.md` (rewrite the four blocks)
- `kernel/schedule.jsonl` (append/refresh next wake-ups)

**MAY write, only inside a due sleep pass:**
- `semantic/world/*`, `semantic/audiences/*`, `semantic/glossary.md` (fold belief deltas)
- `procedural/lessons.md`, and `procedural/playbooks/*` on promotion
- `episodic/_archive/*` (weekly-deep archive)
- `calibration/recalibration_map.json` (recalibrate pass, only if miscalibrated)
- `_meta/consolidation_log.md` (append one pass record)

**MAY touch exactly one cell elsewhere:**
- `goals/plan.md` — toggle the finished step's `status` to DONE only. Never the ledger structure, ranking, or any other field.

**MUST NEVER write (sibling-owned):**
- `goals/goals.md`, `goals/direction.md`, `goals/subgoals.md` — agency owns the goal lifecycle and direction.
- `attention/*` (questions, surprises, explore_log, inquiry_budget) — the attention faculty owns these.
- `calibration/predictions.jsonl`, `calibration/calibration_report.md` — the calibration faculty owns the ledger and report; CONSOLIDATE only reads them.
- `experience/*` (episodes, outcomes, _ledger, reflections) — the experience/learning faculty owns these.
- `schemas/*` — the transfer/abstraction faculty owns schemas.
- `../GATE_STEP2.md` and the gate decision — owned elsewhere; CONSOLIDATE records GREEN/RED, never sets it.
- Outside a due sleep: `semantic/*` and `procedural/*` are off-limits — the `BELIEF Δ` and `LESSON?` lines only propose; the scheduled sleep applies them.

When a write CONSOLIDATE owes needs a file the INDEX does not list, create it empty-but-structured and pointer it; never fabricate prior contents.
