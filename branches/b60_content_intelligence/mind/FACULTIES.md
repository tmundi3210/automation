# FACULTIES — the seven parts of the mind

Each faculty was designed by the repo specialist that owns its discipline (run prompt-only). Each
**owns** a slice of `memory/` (single write-owner — no two faculties write the same file), runs at a
defined seam of the unified loop, and is driven by a short prompt snippet the kernel injects. This is
the reference; the kernel (`KERNEL.md`) wires them into one turn.

---

## 1. Kernel — runtime (`orch`)
**Owns:** `memory/kernel/` — `cursor.json`, `turn_log.jsonl`, `checkpoint.jsonl`, `schedule.jsonl`,
`environments.jsonl`, `blackboard/<env>/`.
**Does:** runs each turn as five beats (sense→recall→decide→act→record) that *begin by reading memory
and end by writing it*; decides **series vs parallel** per step; **checkpoints** after every completed
step so a paused/crashed run resumes exactly; fires **scheduled** wake-ups (recheck/consolidate/
recalibrate); binds each turn to one **environment** from the registry.
**Series/parallel rule:** parallel a step's substeps *iff* they're independent AND don't write each
other's blackboard blocks AND fit the budget; else series; **default series**; merge parallel results
with a reducer that keeps the higher-provenance branch on conflict and drops timed-out branches with a
`partial` note.
**Closes:** the runtime substrate, multi-environment, the loop that makes it one persistent agent.

## 2. Agency — goals & direction (`planner_heavy`)
**Owns:** `memory/goals/` — `goals.md` (the goal stack), `direction.md` (committed direction +
rejected alternatives), `plan.md` (the step ledger, each step series/parallel), `subgoals.md` (spawn log).
**Does:** turns a fuzzy intent into a **typed goal with a falsifiable success criterion** *once*, then
commits and executes; **PLAN→ACT→MONITOR→REPLAN** where replan fires **only on a named defeater**
(stale / off-track / precondition-broke / new-info / budget-low) — so it neither thrashes nor blindly
persists; **spawns its own subgoals** ("I lack X → push subgoal: get X", pop when met); a goal exits
only as **DONE** (criterion met), **ABANDONED** (defeater voids it), or **ESCALATED** (to a human).
**Closes:** agency. *(Top goals are still human-seeded; subgoals are self-generated.)*

## 3. Attention — curiosity & inquiry (`erotetic_heavy`)
**Owns:** `memory/attention/` — `questions.md` (ranked open-question queue), `surprises.md`,
`explore_log.md`, `inquiry_budget.md`.
**Does:** keeps a queue of typed open questions, each with a **presupposition** and an **answerhood
condition** (what would resolve it), ranked by **value-of-information** = goal-impact × uncertainty ÷
cost; spawns questions from **gaps, surprises, and failed predictions**; flags **loaded questions**
(false presupposition) instead of answering them; runs a budgeted **explore-vs-exploit** split (a
minority of turns sample under-explored areas → feeds generalization); hands the top question to agency
as a subgoal with its answerhood condition as the done-test.
**Closes:** directed attention; feeds agency (concrete subgoals) and generalization (exploration).

## 4. Calibration — belief & confidence (`epistemics_heavy`)
**Owns:** `memory/calibration/` — `predictions.jsonl` (the ledger), `recalibration_map.json` (the
active confidence correction), `calibration_report.md`. *Writes confidence deltas into `semantic/`.*
**Does:** every falsifiable claim is a **logged bet** `{prediction, confidence, due_date, basis}`,
sealed before the outcome; at outcome time it's scored (confirmed/refuted + Brier); on a schedule the
mind **bins predictions by confidence band** and checks whether its 70%s come true ~70% of the time,
then rewrites the **recalibration map** ("when you feel 80%, say 70%") that the kernel injects into
every future confidence. **Honesty guard:** no guess travels as a fact; confidence + basis are
mandatory; irreducible (aleatory) uncertainty has a floor it's never optimized below.
**Closes:** calibration — now adaptive and auditable instead of one-shot frozen.

## 5. Learning — experience → lessons (`eval_heavy`)
**Owns:** `memory/experience/` — `episodes.jsonl` (claims sealed pre-outcome), `outcomes.jsonl`,
`reflections/<ep>.md`, `_ledger.jsonl`. *Promotes survivors into `procedural/`.*
**Does:** after each episode, **reflect** (what happened / worked / didn't / why) and distil a
candidate **lesson** = TRIGGER → MOVE → EVIDENCE; the **"did it actually work?"** check compares
claimed vs realized vs a **baseline** (not vs zero), counting correlated episodes as **one cluster**;
**promote** a lesson to a trusted playbook only after ≥k independent confirms whose CI excludes zero;
**retire** it when refuted or a regime change breaks it. This is what makes the improve-loop's inputs
change cycle-to-cycle — the missing experience→lesson→reuse arc.
**Closes:** learning from experience — deliberately slow, so growth is earned not asserted.

## 6. Generalization — transfer & the mind loop (`reason`)
**Owns:** `memory/schemas/` — `<schema_id>.md` (a domain-general reasoning skeleton with open slots +
invariants), `abstractions.md` (the lift ledger: general vs specific bindings), `transfer_log.md` (the
cheap-probe go/no-go).
**Does:** on a **new domain**, retrieve the most **structurally** analogous past episodes (similar
*move*, not similar topic) → **lift** each into a general schema + the old domain's specific bindings →
**re-bind** the slots to the new domain → **cheap probe** on a tiny frozen holdout (commit only if lift
CI excludes 0 and ECE<0.05) → **ablate** every borrowed binding to separate *acquired skill* from
*borrowed prior*. Also defines the **unified loop** (perceive→recall→plan→act→reflect→consolidate) that
makes the faculties one agent.
**Closes:** generalization (measured, not asserted) + the Newell "one mind" loop.

## 7. Memory — the substrate (`signal_heavy` + `epistemics` + `orch`)
**Owns:** `memory/INDEX.md`, `memory/episodic/` (the diary + timeline + archive), `memory/semantic/`
(beliefs about world/audiences + glossary), `memory/procedural/` (playbooks + lessons), `memory/_meta/`.
**Does:** four **tiers** — working (this run's scratch), episodic (the dated **diary**), semantic
(consolidated calibrated beliefs), procedural (skills) — with an **INDEX read first every turn**; each
turn pulls only a **decision-lossless slice** (never the whole store); a scheduled **consolidation
"sleep"** folds episodes into beliefs & skills, archives the raw, and keeps the live store small.
Every record is typed, dated, provenance-stamped, and confidence-tagged — so the mind can always answer
*"do I actually know this, or did I assume it, and how sure am I?"*
**Closes:** persistent continuity — the keystone every other faculty stands on.

---

### How they compose (single-agent invariants)
One objective, one diary, one schedule, one shared memory; **single write-owner per block** so
faculties can't corrupt each other; the loop is **closed** (reflect/consolidate feed the next recall),
so a lesson learned in one domain changes how the next is approached. That closure — plus the shared
external memory — is what makes it *one mind*, not a bag of tools.
