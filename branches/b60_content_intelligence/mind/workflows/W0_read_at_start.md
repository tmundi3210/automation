# W0_read_at_start — READ-AT-START — boot & decision-lossless retrieval

> Boot the mind from disk and load exactly the memory slice this turn needs — no more, no less. Owner specialist: `mind_read_at_start`. Position in the ONE MIND loop: **step 0 of 7**, runs first, before PERCEIVE/RECALL/PLAN/ACT/REFLECT/CONSOLIDATE. Single-write-owner of: nothing persistent of substance — it stages the **LOADED manifest** for the diary, and is the only phase allowed to add a **missing-record stub pointer row** to `mind/memory/INDEX.md`'s HOT POINTERS map. It never rewrites the INDEX NOW block, counts, or consolidation-due lines.

## Purpose

The mind has no in-RAM continuity between turns. Everything it "is" lives on disk under `mind/memory/`. READ-AT-START is the cold-boot that turns a directory of files back into a situated agent: it reconstructs identity, active goal, resume point, and any interrupted work, then loads a **decision-lossless** slice of memory for the rest of the loop to reason over.

Two failure shapes define the job, and both are equally bad:

- **Under-loading** — a downstream phase needs a belief, episode, prediction, or schema that READ-AT-START did not load, so it either stalls or guesses. The slice must be *decision-lossless*: every decision a later phase makes must be reconstructable from what this phase pulled.
- **Over-loading** — dumping the whole store. That blows the token budget, buries the signal, and defeats the entire point of an indexed memory. The slice must stay within roughly **6,000 tokens**.

The discipline that resolves the tension is the **RETRIEVAL RULE** plus one hard rule of honesty: if the active goal needs a record the INDEX does not list, that record **does not exist yet**. READ-AT-START names the gap (a stub pointer) but never fabricates the record's contents. The mind would rather know it has a hole than believe a thing it made up.

This phase binds nothing, decides nothing about direction, and executes no step. It restores state and hands a clean, sourced, budgeted slice to PERCEIVE.

## Inputs (memory read) and Outputs (memory written)

All paths are relative to the active mind root. In the seeded run that root is `branches/b60_content_intelligence/mind/memory/`.

**Inputs — READS (in this order):**

1. `mind/memory/INDEX.md` — read **in full**, always, first. The map of everything the mind knows. Source of: NOW block (identity, active goal, phase/step left off, last_run outcome, top open question, consolidation-due dates), TIERS counts, HOT POINTERS map, the RETRIEVAL RULE text.
2. `mind/memory/kernel/cursor.json` — the resume point: `env_id`, `turn`, `active_step_id`, `phase`, `plan_hash`, `goal`, `note`, `updated`.
3. `mind/memory/kernel/blackboard/<active-env>/` — interrupted-run scratch for the active environment (e.g. `kernel/blackboard/tech_builders/`). In the seeded run this directory is empty except `.gitkeep` (clean close).
4. `mind/memory/working/*` — working scratch (focus slots, in-flight notes). Always staged when present. If the directory is absent, treat working scratch as empty and note it (see edge cases).
5. `mind/memory/goals/plan.md` — only the **open step row** of the active goal's plan, to cross-check `active_step_id` and `plan_hash`.
6. `mind/memory/kernel/schedule.jsonl` — every wake-up row; compare each `due` to now.
7. Then, per the RETRIEVAL RULE, the HOT-POINTER bodies whose id ∈ the active goal's `touches`, and ≤2 topic-matched semantic bodies. For G-12 those resolve to:
   - `mind/memory/semantic/world/ai-agents.md`
   - `mind/memory/semantic/audiences/builders.md`
   - `mind/memory/procedural/playbooks/hype-adjudication.md`
   - `mind/memory/schemas/SCH_geo_link_react_brief.md`

**Outputs — WRITES:**

- **Nothing persistent of substance.** The phase's product is the **LOADED manifest** — the exact slice pulled, by id and absolute file path, with token estimates and qualifying reasons — staged in working memory for the **consolidate** phase to commit as the diary's `LOADED:` line in `mind/memory/episodic/journal/2026-06-28.md`. READ-AT-START does **not** write the journal itself.
- **Conditionally:** exactly one **stub pointer row** appended to the HOT POINTERS map in `mind/memory/INDEX.md`, and only when the active goal needs a record whose id is not already listed there. Format and constraints in the Protocol below.

That is the complete write surface. Every other file under `mind/memory/` is read-only for this phase.

## Protocol

### Step 0 — Resolve the mind root and confirm the INDEX exists

Determine the active mind root (the directory containing `INDEX.md` and `kernel/`). If `INDEX.md` is missing or unreadable, do not proceed and do not bootstrap from other files — **escalate** (see Failure modes). The INDEX is the authoritative map; without it there is no trustworthy way to know what to load.

### Step 1 — Read INDEX.md in full and parse the NOW block

Read the entire file (it is small by design — about 30 lines in the seeded run). Parse and pin these fields from the header and `## NOW` block:

- **run id** and **active env** from the header line (`run:R0631 · env:tech_builders`).
- **active goal** id and quoted title (`goal: G-12 "weekly brief: AI-agent hype vs organic (tech_builders)"`).
- **goal type** annotation (`(agency)`) — note it; it is informational for PLAN, not a filter here.
- **phase / step left off** (`phase: consolidate · step 6/6 done`).
- **last_run** line: prior run id, date, outcome, and delta (`R0630 2026-06-27 → outcome:EMITTED · Δsemantic:+1 world`).
- **top open question** (`open_q: OQ-07 ...` → `attention/questions.md`).
- **TRANSFER note** if present (`this env (tech_builders) was TRANSFERRED from punjab_diaspora via SCH_geo_link_react_brief (TRX_0007)`). Record it; do not act on it (TRANSFER mode is RECALL's job).
- From `## TIERS`: the per-tier counts and last-consolidated dates (informational; used to sanity-check the schedule and detect bloat flags).
- From `## CONSOLIDATION DUE`: the `next pass` dates and the `backlog`/`bloat` flags.

The active goal's `touches` list is **not** in the NOW block — it lives on the goal's stack block in `goals/goals.md`. Resolve it in Step 4. Do not guess it from the HOT POINTERS list; read it from the goal.

### Step 2 — Read cursor.json and the working scratch; restore the resume point

Read `kernel/cursor.json` and pin: `env_id`, `turn`, `active_step_id`, `phase`, `plan_hash`, `goal`, `note`, `updated`.

Read the working scratch:

- `kernel/blackboard/<env_id>/` for the active env (e.g. `kernel/blackboard/tech_builders/`). Any non-`.gitkeep` content here is interrupted-run state to restore.
- `working/*` if present.

Decide **interrupted vs clean**:

- **Clean close** when `active_step_id` is `DONE` (or the plan's open step set is empty) and the blackboard for the env holds no in-flight step state. This means the previous turn finished its loop and committed.
- **Interrupted** when `active_step_id` names a live step and/or the blackboard holds partial step output. The slice must then additionally cover what is needed to resume that exact step.

**Cross-check cursor.json against the INDEX NOW block.** Two fields can legitimately differ and must not be treated as conflicts:

- The cursor's `phase` is the **entry phase for the next turn** (where the loop resumes), while the INDEX NOW `phase` records the **last completed phase of the prior turn**. In the seeded run cursor `phase:sense` (next turn enters at sense/perceive) and INDEX `phase:consolidate · step 6/6 done` (prior turn finished consolidate) are **consistent**, not contradictory.

Treat as a genuine **inconsistency** (set the inconsistency flag, do not proceed to load on guessed values) when any of these disagree:

- cursor `goal` ≠ INDEX NOW goal id.
- cursor `plan_hash` ≠ the `plan_hash` in `goals/plan.md` header.
- cursor `active_step_id` names a step id that does not exist in `goals/plan.md`.
- cursor `env_id` ≠ INDEX header env.

On any such disagreement, escalate per Failure modes rather than picking a side.

### Step 3 — Read schedule.jsonl and detect the turn mandate

Read every row of `kernel/schedule.jsonl`. For each, compare `due` to **now** (the current run timestamp).

- A wake-up is **due** when `due ≤ now`.
- If one or more are due, the **earliest due** wake-up IS this turn's work. Capture its `kind` (`consolidate` | `recheck` | `recalibrate`), `env_id` target, and `action`, and set the **turn mandate** = "execute due wake-up `<id>`". This mandate is flagged for the PLAN phase; it overrides free-goal work.
- If a due wake-up targets a specific `env_id` other than the active env, record that the turn may need to bind a different environment — but do **not** bind it (PERCEIVE owns binding). Just flag the target env on the mandate.
- If multiple are due, the mandate is the earliest; note the others as also-due so PLAN can batch or queue them.
- If none are due, set **turn mandate = FREE** (free-goal work on the active goal), and record the next upcoming wake-up id and date for context.

Use `now = 2026-06-28T14:02Z` from the run. In the seeded schedule the earliest future row is `wk_021` (daily light consolidate, due `2026-06-29T03:00Z`) — after now — so **nothing is due** and the turn is FREE.

### Step 4 — Resolve goal.touches and apply the RETRIEVAL RULE

Open `goals/goals.md`, find the ACTIVE goal block (status `ACTIVE`, top of the in-use stack), and read its `touches:` line. For G-12 that is `touches: W-ai-agents, A-builders, P-hype-adjud, SCH-geo-link`.

Now stage the slice in three tiers, in this exact order:

1. **Always:** `INDEX.md` (already read) + everything under `working/*` + the focus slots. These are non-negotiable and load regardless of goal.
2. **Goal-touched pointers:** for each id in `goal.touches`, find its row in the INDEX HOT POINTERS map, resolve the file path, and load that body. For G-12:
   - `W-ai-agents` → `semantic/world/ai-agents.md` (conf .68)
   - `A-builders` → `semantic/audiences/builders.md` (conf .80)
   - `P-hype-adjud` → `procedural/playbooks/hype-adjudication.md` (win 7/9)
   - `SCH-geo-link` → `schemas/SCH_geo_link_react_brief.md`
   - (`G-12` itself points at `goals/goals.md`, already read in this step — do not double-count its tokens.)
3. **Topic-matched semantic, ≤2:** add at most two semantic bodies whose topic matches the goal's subject, and only if not already loaded above. For G-12 the topic is "AI-agent hype vs organic for builders," which topic-matches `semantic/world/ai-agents.md` and `semantic/audiences/builders.md` — the **same two bodies the first two pointers already reference**. So the topic-match adds zero new bodies and the count stays at two semantic bodies. Do not reach for a third (e.g. do not pull `semantic/glossary.md` unless an abbreviation in a loaded body is load-bearing and unresolved).

Do **not** load, in this phase: episodes (`experience/episodes.jsonl`), outcomes, full belief audit trails, reflections, calibration internals, or any superseded record. Those are RECALL's to pull on demand. READ-AT-START loads the *current* slice, not the *history*.

### Step 5 — Track tokens and enforce the ~6k budget

Maintain a running token estimate as you stage. Estimate per item (rough character/4 heuristic is fine) and keep a tally. Target ceiling: **~6,000 tokens** for the whole staged slice.

If the tally would exceed the budget, drop the **lowest-value optional item first**, in this priority (drop earliest):

1. a topic-matched semantic body that is not also a goal-touched pointer,
2. then the *weaker* of two topic matches (lower conf / fewer runs-seen),
3. then a goal-touched pointer that is least connected to the open step.

Never drop INDEX, `working/*`, or focus to fit the budget — if even those plus the mandatory pointers exceed budget, that is a **budget-overflow escalation** (see Failure modes), not a silent truncation.

Record every drop in a **not-loaded list** with a one-line reason (e.g. "dropped glossary.md — no unresolved abbr in loaded bodies; budget"). The not-loaded list is part of the manifest so downstream phases know what was deliberately excluded versus simply absent.

In the seeded run the four pointer bodies plus INDEX plus focus total well under 6k, so nothing is dropped and the not-loaded list is empty.

### Step 6 — Handle missing records (no-fabrication rule)

For each record the active goal demonstrably needs whose id is **not** in the INDEX HOT POINTERS map:

- Treat it as **non-existent**. The mind does not know this thing yet.
- Append **exactly one stub pointer row** per missing id to the INDEX HOT POINTERS map, in the established row format:

  ```
  <ID>  → <intended/file/path.md>  · <one-line why this should exist>  · conf:unknown
  ```

- **Never** author the body file at that path. **Never** write guessed beliefs, claims, or contents anywhere.
- Note the named gap in the manifest so a later phase (PLAN may schedule its acquisition; CONSOLIDATE may author it) can fill it honestly.

Do not modify any existing pointer row, the NOW block, the TIERS counts, the RETRIEVAL RULE text, or the CONSOLIDATION DUE block. The only edit permitted to `INDEX.md` in this phase is appending stub pointer rows to the HOT POINTERS map.

In the seeded run every id in G-12's touches (`W-ai-agents`, `A-builders`, `P-hype-adjud`, `SCH-geo-link`) is already present in the map, so **no stub is created** and `INDEX.md` is not modified at all.

### Step 7 — Compile the LOADED manifest

Assemble the manifest that the consolidate phase will render as the diary `LOADED:` line. It contains:

- **loaded items:** each by id and absolute file path, with qualifying reason (mandatory | goal-touched pointer | topic-match | resume-needed) and token estimate. Include conf where the record carries one.
- **not-loaded list:** deliberately excluded items with one-line reasons (budget drops, redundant topic matches).
- **created stubs:** any new pointer rows (id → path · why · conf:unknown), or "none."
- **turn mandate:** FREE or the due wake-up id+kind+target.
- **resume point:** restored `{env_id, turn, active_step_id, phase, plan_hash, goal}` and the interrupted/clean verdict.
- **total token estimate** for the loaded slice.

The compact diary form mirrors the existing seeded `LOADED:` line so consolidate can drop it in directly:

```
LOADED : INDEX, focus, W-ai-agents(.68), A-builders(.80), P-hype-adjud, SCH-geo-link, OQ-07.
```

### Step 8 — Hand off, stop at the boundary

Emit to PERCEIVE: the restored resume point, the turn mandate, the within-budget loaded slice, and the staged LOADED manifest. Then **stop**. Do not bind the environment, do not set the focus line, do not recall episodes or beliefs for reasoning, do not plan steps, do not execute, do not score. Those are other phases' jobs and other phases' files.

## Decision rules & edge cases

- **Cursor phase vs INDEX phase look like they conflict.** They usually do not — cursor `phase` is the next-turn entry point, INDEX `phase` is the last-completed phase. Only flag a conflict on goal-id, plan_hash, step-existence, or env-id mismatch (Step 2). Do not block the turn over the phase-field difference alone.
- **`working/` directory is absent.** Treat working scratch as empty, record "working: none (dir absent)" in the manifest, and continue. The focus slots referenced in INDEX TIERS (`working: focus(3)`) are still expected; if focus is referenced but unrecoverable, flag it as a soft inconsistency for PERCEIVE (which owns the focus line) rather than escalating.
- **Blackboard for the env has partial step output but cursor says `DONE`.** This is an interrupted/cursor disagreement. Prefer the blackboard evidence (something was mid-flight), set interrupted=true, load what the live step needs, and flag the cursor staleness for escalation review. Do not silently discard the scratch.
- **A goal-touched pointer's body file is missing on disk** (the pointer row exists but the file does not). Do not fabricate. Record "pointer present, body MISSING" in the manifest, set the inconsistency flag, and escalate if the missing body is required to satisfy the goal. A pointer with conf set but no body is a corruption signal, not a missing-record-to-stub case.
- **A needed record's id is genuinely absent from the map.** Create exactly one stub row (Step 6). One stub per missing id, no body, conf:unknown. Naming the gap is success; filling it from imagination is failure.
- **Budget exceeded even after dropping all optional items.** Do not truncate mandatory items. Escalate as budget-overflow (the goal cannot be served decision-losslessly within ~6k — likely the goal is too broad or a body has bloated past consolidation thresholds).
- **Two equally-strong topic matches and room for only one.** Prefer the body with higher conf and more `runs-seen`; on a tie prefer the one linked from a goal-touched pointer. Record the loser in the not-loaded list.
- **A due wake-up targets a different env than the active env.** Flag the target env on the mandate; do not bind it. PERCEIVE binds environments.
- **Multiple wake-ups due at once.** Earliest is the mandate; the rest are recorded as also-due for PLAN to queue.
- **INDEX TIERS counts disagree with what is on disk** (e.g. INDEX says `semantic: world(1)` but two world files exist). Record the count drift as a soft flag in the manifest for CONSOLIDATE to reconcile; do not edit the counts (not this phase's write surface) and do not let the drift block loading.

## Worked example (run R0631 / goal G-12)

Boot at `now = 2026-06-28T14:02Z`, mind root `branches/b60_content_intelligence/mind/memory/`.

**Step 1 — INDEX in full.** Header gives `run:R0631 · env:tech_builders`. NOW gives active goal **G-12** "weekly brief: AI-agent hype vs organic (tech_builders)" (agency); phase `consolidate · step 6/6 done`; last_run **R0630** 2026-06-27 outcome **EMITTED** (Δsemantic +1 world); open_q **OQ-07** "are the 2 origins funded by one operator?" → `attention/questions.md`; TRANSFER note: env `tech_builders` came from `punjab_diaspora` via `SCH_geo_link_react_brief (TRX_0007)`. CONSOLIDATION DUE: next light 2026-06-29, weekly deep 2026-07-05, recalibrate 2026-07-12; backlog 0/7, bloat OK.

**Step 2 — cursor + scratch.** `cursor.json`: `turn:631, active_step_id:DONE, phase:sense, plan_hash:a1b2c3, goal:G-12`. Blackboard `kernel/blackboard/tech_builders/` holds only `.gitkeep`; `working/` absent. Verdict: **clean close** (`active_step_id:DONE`, no in-flight scratch). Cross-check: cursor `phase:sense` (next-turn entry) vs INDEX `consolidate · step 6/6 done` (last completed) — **consistent**, not a conflict. cursor `goal:G-12` = INDEX goal ✓; cursor `plan_hash:a1b2c3` = `goals/plan.md` header `plan_hash:a1b2c3` ✓; `active_step_id:DONE` is consistent with the plan having all six steps S1–S6 at DONE ✓. No inconsistency flag.

**Step 3 — schedule.** Rows: `wk_021` daily light consolidate due 2026-06-29T03:00Z; `wk_022` weekly deep 2026-07-05T03:00Z; `wk_023` recalibrate 2026-07-12T03:00Z; `wk_024` recheck tech_builders (re-run G-12, resolve P-118) due 2026-07-05T09:00Z; `wk_025` recheck P-118 re-spike due 2026-07-12T09:00Z. Every `due` is **after** now (earliest is `wk_021` on 2026-06-29). **Nothing due → turn mandate = FREE** (free-goal work on G-12). Note next upcoming: `wk_021` 2026-06-29.

**Step 4 — touches + RETRIEVAL RULE.** From `goals/goals.md`, G-12 `touches: W-ai-agents, A-builders, P-hype-adjud, SCH-geo-link`. Stage:
- mandatory: INDEX + focus (working absent → focus from INDEX TIERS `focus(3)`).
- goal-touched pointers → `semantic/world/ai-agents.md` (B1 adoption rising .68; B2 manufactured-hype .74; B3 aleatory DO-NOT-CHASE; C1 unverified → OQ-07), `semantic/audiences/builders.md` (A1 signal>volume .80; A2 framing .72; A3 saves/shares .68), `procedural/playbooks/hype-adjudication.md` (≥N-origin gate, N=3, win 7/9), `schemas/SCH_geo_link_react_brief.md` (tech_builders binding, conf .71).
- topic-match ≤2 → `world/ai-agents.md` and `audiences/builders.md`, already loaded as pointers → **0 new bodies**, count stays at two semantic. No third body pulled.

**Step 5 — budget.** Four pointer bodies (~30–40 lines each) + INDEX (~30 lines) + focus ≈ well under 6k. No drops; not-loaded list empty.

**Step 6 — missing records.** All four touched ids are present in the HOT POINTERS map. **No stub created. `INDEX.md` not modified.** (For contrast: had G-12 also touched, say, a `W-funding-trace` record to answer OQ-07's "one operator?" question and that id were absent, this phase would append exactly `W-funding-trace → semantic/world/funding-trace.md · ownership/funding provenance behind agent-hype origins · conf:unknown` and write no body — leaving the actual trace for ACT/RECALL to gather and CONSOLIDATE to author.)

**Step 7 — manifest.** Loaded: INDEX; focus(3); `semantic/world/ai-agents.md`(.68); `semantic/audiences/builders.md`(.80); `procedural/playbooks/hype-adjudication.md`(win 7/9); `schemas/SCH_geo_link_react_brief.md`; plus the open-question handle OQ-07. Not-loaded: none. Stubs: none. Mandate: FREE. Resume: `{tech_builders, 631, DONE, next-phase sense, a1b2c3, G-12}`, clean. Staged diary line:

```
LOADED : INDEX, focus, W-ai-agents(.68), A-builders(.80), P-hype-adjud, SCH-geo-link, OQ-07.
```

This matches the `LOADED:` line already recorded for R0631 in `episodic/journal/2026-06-28.md`, confirming the slice is decision-lossless for the run that followed.

**Note on records NOT loaded and why it is still lossless.** R0631 went on to refute prediction **P-114** (predicted ≥3 independent origins; deduped to 2 → refuted, Brier 0.42), reinforce belief **B2** (manufactured-hype .70→.74), open prediction **P-118** (re-spike within 14d, p=0.35, due 2026-07-12), and log candidate lesson **L-23** ("≥3 platforms agreeing ≠ ≥3 origins — dedup BEFORE counting," seen 1×). READ-AT-START did **not** load `calibration/predictions.jsonl`, `procedural/lessons.md`, or `experience/episodes.jsonl` — correctly. Those are written/read by ACT, REFLECT, and RECALL respectively. The slice this phase loaded (the ≥N-origin gate in the playbook, B2's current conf, the schema binding, OQ-07) is exactly what PERCEIVE/RECALL/PLAN needed to *reach* those operations; the operations themselves pull their own records on demand. Loading P-114 or L-23 here would have been over-loading.

**Step 8 — handoff.** Emit resume point + mandate FREE + the loaded slice + staged manifest to PERCEIVE; stop. No env bound, no focus line set, no episodes recalled, no plan touched.

## Failure modes & escalation

ESCALATE to a human (halt the turn, emit the flag, do not guess) when:

- **INDEX.md missing, unreadable, or schema-version mismatch** (`schema:mem/1.0` absent or unknown). The map is gone; there is no safe basis to load. Do not reconstruct it from other tiers.
- **cursor.json points at a non-existent step or plan_hash.** `active_step_id` names a step absent from `goals/plan.md`, or `plan_hash` disagrees with the plan header. The resume point is corrupt; resuming on a guess risks redoing or skipping committed work.
- **INDEX ↔ cursor goal/env disagreement.** cursor `goal`/`env_id` differs from the INDEX NOW/header. Two different "active" states; pick neither.
- **Goal-touched pointer present but body file missing on disk**, and that body is required to serve the goal. A dangling pointer is corruption, not a stub case — do not fabricate a body.
- **Budget overflow on mandatory items.** INDEX + working + the required goal-touched pointers alone exceed ~6k. The goal cannot be served decision-losslessly within budget — flag for goal-scoping or consolidation (a body has likely bloated past its consolidation threshold).
- **Blackboard scratch contradicts a `DONE` cursor** in a way that cannot be reconciled (partial output for a step the cursor claims finished). Restore conservatively, flag the staleness, and escalate if the contradiction blocks a clean resume.
- **schedule.jsonl unparseable or a due row malformed** (missing `due`/`kind`/`env_id`). Cannot determine the mandate; do not assume FREE.

For soft drift that does **not** block loading (TIERS counts vs disk, missing `working/` with focus referenced), record a flag in the manifest for the owning phase (CONSOLIDATE for counts, PERCEIVE for focus) and continue — do not escalate, do not self-repair another phase's files.

## Handoff

**Next workflow:** PERCEIVE (step 1 of the loop) runs immediately after.

State READ-AT-START must leave behind for PERCEIVE to pick up:

- **Restored resume point:** `{env_id, turn, active_step_id, phase, plan_hash, goal}` plus the interrupted/clean verdict — so PERCEIVE knows whether it is opening a fresh turn or resuming mid-step.
- **Turn mandate:** FREE (free-goal work on the active goal) or the due wake-up (`id`, `kind`, target `env_id`, `action`) — so PERCEIVE and PLAN route the turn correctly. In R0631 this is FREE.
- **The loaded slice, within budget:** every staged body by id and path, available in working memory for RECALL/PLAN to read without re-fetching.
- **The staged LOADED manifest:** loaded items + not-loaded list + created stubs + token total — held for CONSOLIDATE to commit as the `LOADED:` line in `episodic/journal/2026-06-28.md`. READ-AT-START does not write the journal; it stages the line.
- **Any flags:** inconsistency, soft drift, or escalation conditions, attached to the handoff.

What PERCEIVE owns next and READ-AT-START must NOT have done: binding the environment (`kernel/environments.jsonl`) and setting the turn's focus line. READ-AT-START stops exactly at that boundary.

## Single-write-owner contract

**MAY write (the entire write surface of this phase):**

- `mind/memory/INDEX.md` — **only** to append a missing-record **stub pointer row** to the HOT POINTERS map, in the format `<ID> → <intended/path.md> · <one-line why> · conf:unknown`, one row per genuinely-absent needed id. No body is ever authored alongside it.
- The **LOADED manifest** in working memory — staged, not persisted by this phase; the consolidate phase commits it to the journal.

**MUST NEVER touch:**

- The INDEX **NOW block**, **TIERS counts**, **RETRIEVAL RULE** text, or **CONSOLIDATION DUE** block — those belong to CONSOLIDATE (which rewrites the NOW block and counts at end of turn).
- `kernel/cursor.json`, `kernel/checkpoint.jsonl`, `kernel/turn_log.jsonl`, `kernel/schedule.jsonl`, `kernel/environments.jsonl`, `kernel/blackboard/*` — read-only here; the kernel and later phases own writes.
- `goals/*` (goals, direction, plan, subgoals) — PLAN's.
- `attention/*` (questions, surprises, explore_log, inquiry_budget) — PERCEIVE/REFLECT's.
- `calibration/predictions.jsonl`, `recalibration_map.json`, `calibration_report.md` — ACT/REFLECT's.
- `experience/*` (episodes, outcomes, _ledger, reflections) — REFLECT's.
- `schemas/*` bodies, `semantic/*` bodies, `procedural/*` bodies — RECALL reads them; CONSOLIDATE authors/folds them. READ-AT-START only *reads* the slice it loads and only *registers a stub pointer* for an absent one.
- `episodic/journal/2026-06-28.md` and `episodic/timeline.md` — READ-AT-START stages the `LOADED:` line; CONSOLIDATE writes it.
- `_meta/*` (retention, consolidation_log) — CONSOLIDATE's.

The single inviolable rule across this entire phase: **load what the turn needs, name what is missing, fabricate nothing.** The only persistent mark READ-AT-START is permitted to leave is a stub that says "this should exist and does not yet" — never a guessed belief, never a body file, never another faculty's record.
