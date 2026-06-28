# W4_act — ACT — execute one step as a calibrated BET (safety gate FROZEN)

> Execute the ONE step PLAN already selected, as a calibrated bet: fire matching playbooks, draft the artifact, seal every falsifiable claim into `calibration/predictions.jsonl`, merge any parallel substeps in `kernel/blackboard/<env>/_merge/<step>`, and DEFER every safety/ethics/legal decision to the frozen `../GATE_STEP2.md`. Owner specialist: `mind_act`. Position in the loop: phase 4 of 6 (PERCEIVE → RECALL → PLAN → **ACT** → REFLECT → CONSOLIDATE). Single-write-owner files: `calibration/predictions.jsonl` (Calibration owns) and `kernel/blackboard/<env>/_merge/<step>` (Kernel owns).

## Purpose

ACT is the one beat of the turn where the mind actually *does* something in the world-model. Everything before it has narrowed the cone: PERCEIVE bound the turn to one environment, RECALL pulled the relevant beliefs/episodes/playbooks, and PLAN already chose the single step to run (the first unblocked open step in `goals/plan.md` whose dependencies are `DONE`) and tagged it `SERIES` or `PARALLEL`. ACT executes that step and only that step.

The defining stance of ACT is **calibrated betting**: nothing leaves this phase as a bare assertion. Every falsifiable claim is first written as a sealed prediction with a confidence that has passed through the active recalibration correction, so REFLECT can later score it honestly against reality. ACT also owns the mechanics of fanning a `PARALLEL` step out to substep blocks and reducing them back to one provenance-ranked result, and it owns the transfer cheap-probe go/no-go when the step is a schema re-bind into a new domain.

What ACT deliberately does NOT own: it does not pick the step, set direction, re-plan, or re-budget (Agency/PLAN); it does not rank questions (Attention); it does not score outcomes, fold belief deltas, or write lessons (REFLECT); it does not advance the cursor, append the diary, or checkpoint (CONSOLIDATE); and it does not make the safety/compliance/ethics/legal decision (that is FROZEN and owned by `../GATE_STEP2.md`). ACT executes, seals bets, merges substeps, defers the safety call, and hands back.

## Inputs (memory read) and Outputs (memory written)

All paths are relative to the run's memory root `branches/b60_content_intelligence/mind/memory/`. Read only the decision-lossless slice the `INDEX.md` retrieval rule points to; if a record you need is not in `INDEX.md`, it does not exist yet — spawn a subgoal to create it, never invent its contents.

**Inputs — read-only for ACT:**

- `INDEX.md` — the map; obey its RETRIEVAL RULE (load INDEX + working/* always, + pointers whose id is in `goal.touches`, + at most 2 semantic by topic-match, budget ~6k tokens). Confirms NOW = G-12, env `tech_builders`.
- `kernel/environments.jsonl` — bind to the single record with `"active":true`; here `tech_builders` (transferred from `punjab_diaspora`). Read its `constraints` (`safe-lanes-only`, `compliance-gate-mandatory`, `ToS:x-tech`) and `platform` (`youtube+reddit+x-tech`).
- `goals/plan.md` — the ONE step PLAN selected and its mode (`SERIES`/`PARALLEL`). ACT does not choose; it reads the row.
- `goals/goals.md`, `goals/subgoals.md` — the active goal block (G-12) and the subgoal audit trail, read for the step's success_criterion and `touches`.
- `procedural/playbooks/*` (e.g. `procedural/playbooks/hype-adjudication.md`) and `procedural/lessons.md` — fire every playbook whose TRIGGER matches the move; apply TRUSTED lessons.
- `calibration/recalibration_map.json` — the active correction (`recal-v2`); apply its shrink rule to every stated confidence before the claim travels.
- `schemas/SCH_geo_link_react_brief.md`, `schemas/abstractions.md`, `schemas/transfer_log.md` — read when the step is a draft-via-schema or a transfer re-bind.
- `kernel/blackboard/<env>/` — prior scratch for this environment (`kernel/blackboard/tech_builders/`), including any earlier `_merge/` blocks.
- `semantic/world/ai-agents.md`, `semantic/audiences/builders.md`, `semantic/glossary.md` — domain beliefs (B1, B2, B3) and audience codes the artifact draws on.
- `../GATE_STEP2.md` — the FROZEN compliance gate prompt, run verbatim on any publishable output. (One directory above the workflow doc folder; ACT reads and obeys it, never rewrites it.)

**Outputs — single-write-owner for ACT (append-only; never overwrite another faculty's lines):**

- `calibration/predictions.jsonl` — one sealed BET per falsifiable claim, written BEFORE the claim travels. Calibration owns this file.
- `kernel/blackboard/<env>/_merge/<step>` — for a `PARALLEL` step: each independent substep's own block, plus the reducer's merged output. Kernel owns the blackboard. For env `tech_builders`, step `S2`, this is `kernel/blackboard/tech_builders/_merge/S2/`.

ACT writes nothing else. It does not touch `plan.md`, `goals.md`, `experience/outcomes.jsonl`, `semantic/`, `procedural/`, `kernel/cursor.json`, `kernel/checkpoint.jsonl`, `episodic/`, or `attention/`. Any change ACT wants in those files is requested by handing a signal to the owning faculty (e.g. a subgoal request to Agency), not by writing the file.

## Protocol

Run these steps in order. Stop forward motion and jump to step 8 the moment a precondition is missing.

### 1. BIND + LOAD

1.1 Open `kernel/environments.jsonl`. Select the single record with `"active":true`. For this run that is `tech_builders` (`"context_slice":"transferred from punjab_diaspora via SCH_geo_link_react_brief"`). Hold its `constraints` and `platform` in working set — they bound the artifact and they decide whether the gate is mandatory. Here `constraints` includes `compliance-gate-mandatory`, so any publishable output WILL be routed through the gate at step 6.

1.2 Open `goals/plan.md`. Read the first row whose `status` is `OPEN` and whose `dep` step is `DONE` — that is the step PLAN handed you. Record its `step` id, `mode` (`SERIES` or `PARALLEL`), and `desc`. Do NOT re-derive which step to run; if every row is `DONE`, there is no step to act on — return a "no open step" hand-off to PLAN and halt this phase.

1.3 Load only the INDEX-pointed slice: `INDEX.md` plus working/blackboard, plus pointers whose id is in the active goal's `touches` list, plus at most two semantic records by topic-match, under the ~6k-token budget. For G-12, `touches = {W-ai-agents, A-builders, P-hype-adjud, SCH-geo-link}`, so you pull `semantic/world/ai-agents.md`, `semantic/audiences/builders.md`, `procedural/playbooks/hype-adjudication.md`, and `schemas/SCH_geo_link_react_brief.md`.

1.4 If a record the step needs is absent from `INDEX.md`, treat it as nonexistent (do not fabricate its contents) and go to step 8 to spawn a subgoal that creates it.

### 2. MATCH PLAYBOOKS

2.1 Read `procedural/playbooks/*` and `procedural/lessons.md`. For each playbook, compare its `TRIGGER` line to the move the step performs. Fire every playbook whose TRIGGER fits; apply TRUSTED lessons; treat CANDIDATE lessons (on probation, e.g. `L-23`) as advisory, not binding.

2.2 When the move is genuine-vs-manufactured adjudication, `procedural/playbooks/hype-adjudication.md` fires (TRIGGER: "a topic looks like it's blowing up across multiple platforms; you must decide genuine vs manufactured"). Apply its MOVE exactly:
   - Collect reactions but **count ORIGINS, not platforms.**
   - **Provenance-dedup:** trace each platform's wave back to its seed; a reposted or single-campaign seed is ONE origin, regardless of how many platforms carried it.
   - **≥N-origin gate, N=3, independent of volume:** fewer than 3 independent origins after dedup → MANUFACTURED. Volume never overrides this gate.
   - **Emit `p(inflated)` as a calibrated probability, never a boolean,** with the deduped origin count as the explicit `basis`.

2.3 If two fired playbooks disagree, prefer the one with the higher win-rate / TRUSTED status and note the conflict in the hand-off for REFLECT to weigh. `hype-adjudication.md` is `win:7/9 · status:TRUSTED`.

### 3. EXECUTE

Pick exactly one of 3A / 3B based on the step's nature, then run 3C (fan-out) only if PLAN tagged the step `PARALLEL`.

**3A. Transfer re-bind (cheap-probe gate).** If the step is binding a schema's open slots to a new domain (or running a freshly re-bound schema for the first time in this domain):
   - Re-bind only the schema's `open_slots` to this domain; never inherit the old envelope. For `SCH_geo_link_react_brief` into `tech_builders` the committed bindings are: `anchor = a shared tool/release event`, `culture_codes = build-in-public/VC/founder`, `platform_set = +x-tech,+HN −regional-IG`.
   - Run the re-bound schema on a **tiny FROZEN probe set** with a **pre-registered metric**. Commit only if the lift's **95% CI excludes zero.** This is the `TRX_0007` pattern: probe `n=12` held-out tech items, metric `link_validity@anchor`, lift `+0.23` over cold-start, 95% CI `[0.06, 0.39]` (excludes 0) → COMMIT, with an ablation dropping all punjab bindings still holding `+0.19` (acquired here, not just borrowed). Record the probe result for the transfer log owner (Schemas/Consolidate), not by editing it yourself.
   - If the CI includes zero, do NOT commit: revise the analog (re-bind differently) or cold-start, and hand that decision back to PLAN/Agency.

**3B. Draft via the brain (substantive artifact).** Otherwise produce the artifact the step calls for — the brief, the analysis, the decision — using the brain drafting step. Bind the schema skeleton's slots to the active domain and audience: `find {{entities}} sharing {{anchor}} salient to {{audience}}; read {{platform_set}} reactions → p(inflated) via the ≥N-origin gate; draft image+audio+story using {{culture_codes}}; emit SAFETY FACTS → gate`. Carry confidence + basis on every claim the draft makes; no claim ships bare.

**3C. Fan-out (PARALLEL only).** Fan out if and ONLY IF all of these hold: PLAN tagged the step `PARALLEL`, the substeps are genuinely independent, they do NOT write each other's blackboard blocks, and they fit the budget. Default `SERIES` when unsure; never fabricate a parallel branch's result.
   - Each parallel substep writes its OWN block under `kernel/blackboard/<env>/_merge/<step>/` (one file or keyed block per substep). For G-12's `S2` ("read reactions across youtube/reddit/x-tech"), that is `kernel/blackboard/tech_builders/_merge/S2/youtube`, `.../reddit`, `.../x-tech`.
   - Then run the **reducer**: combine the substep blocks into one merged block in the same `_merge/<step>` directory. On conflict, **keep the higher-provenance branch** (a primary/seed observation outranks a downstream repost). **Drop any timed-out branch** and stamp the merged result with a `partial` note naming the dropped branch so REFLECT knows coverage was incomplete. Never invent a branch's output to fill a gap — a missing branch is `partial`, not imagined.

### 4. SEAL BETS

For every falsifiable claim the step produces, BEFORE it travels (before it enters the artifact that goes to the gate, before it is handed back):

4.1 Take your felt confidence as a probability.

4.2 Apply `calibration/recalibration_map.json` (`recal-v2`): shrink rule `felt 80 → say 72`, `felt 90 → say 78`; the `50–70` band is well-calibrated and passes through unchanged; then clamp to `floor 0.05`, `cap 0.90`. Interpolate within the documented bands; never state more confidence than the map allows.

4.3 Append ONE line to `calibration/predictions.jsonl` with a stable id and these fields:
```json
{"id":"P-NNN","ts":"<UTC>","claim":"<the falsifiable claim>","prediction":"<the operational, checkable form>","confidence":<post-shrink p>,"conf_band":<band>,"due_date":"<when it can be scored>","basis":"<the evidence the bet rests on>","node_id":"<the observed node>","status":"open","outcome":null,"scored_ts":null,"brier":null}
```
The bet is **sealed pre-outcome**: `status:"open"`, `outcome:null`, `scored_ts:null`, `brier:null`. ACT never scores its own bets — REFLECT fills `outcome`/`brier` later. Example shapes already in the file: `P-114` (sealed at `confidence 0.65`, `conf_band 70`, basis "surface volume across 4 platforms; NOT yet deduped") and `P-118` (sealed at `confidence 0.35`, `conf_band 50`, `due_date 2026-07-12`, basis "manufactured single-origin waves usually fade; 35% it returns organically").

4.4 Never ship a guess as a fact. If a claim is falsifiable, it gets a sealed bet first. If it is not falsifiable (a definition, a restatement of a constraint), it does not need a bet but also must not be dressed up as a prediction.

### 5. (reserved within EXECUTE/SEAL) — see 3 and 4

The transfer cheap-probe (3A) and bet sealing (4) together cover "run before you trust." No separate action; this anchor exists so the protocol numbering matches the decision procedure's seal-then-gate order.

### 6. GATE

6.1 Route ANY publishable output through `../GATE_STEP2.md` **verbatim**, together with the brain's drafted artifact and its SAFETY FACTS list. "Publishable" = anything that would be rendered, voiced, posted, or emitted outside the mind. The active env `tech_builders` carries `compliance-gate-mandatory`, so this is non-optional here.

6.2 The gate returns exactly one verdict block: `VERDICT: GREEN/YELLOW/STOP`, `REASON`, `REQUIRED EDITS`, `DISCLOSURE LINE`, `CLEAR TO RENDER: yes/no`.
   - On **STOP**: mark the step `blocked` in the hand-off and **HALT**. Do not soften, re-implement, route around, or re-judge the gate. The step does not emit.
   - On **YELLOW**: proceed only as the gate allows — apply every REQUIRED EDIT and attach the DISCLOSURE LINE before the output travels. Re-run the gate on the edited output if the edits are material.
   - On **GREEN**: proceed; attach the DISCLOSURE LINE the gate specifies.

6.3 Only when `CLEAR TO RENDER: yes` does the output finalize. See the SAFETY GATE — FROZEN callout below; ACT never owns this decision.

### 7. WRITE-OWNER WRITES, then HAND BACK

7.1 Confirm ACT wrote only to its two owned targets: the sealed bets in `calibration/predictions.jsonl` and the substep blocks + reducer output in `kernel/blackboard/<env>/_merge/<step>`. Both are append-only; you did not overwrite another faculty's lines.

7.2 Hand back to REFLECT/Consolidate: the gate-cleared output (or the `blocked` marker on STOP), the merged `_merge/<step>` result with any `partial` notes, and the list of sealed-bet ids. Do NOT score outcomes, fold beliefs, write lessons, edit `semantic/` or `procedural/`, advance `kernel/cursor.json`, or append `episodic/journal/<today>.md` or `kernel/checkpoint.jsonl` — those are other faculties' beats.

### 8. SUBGOAL ON MISSING PRECONDITION

The moment you hit "I lack X" (a needed record absent from `INDEX.md`, a probe set that does not exist, a binding that cannot be resolved, a constraint you cannot evaluate): stop forward motion on the bet, and signal Agency to push a subgoal in `goals.md` to get X, stated with its **answerhood condition** (the checkable thing that means X now exists). Pop back and resume the step only when that condition is met. ACT does not write `goals.md` itself; it raises the subgoal request and waits. This mirrors how `G-07` was spawned from `G-12` ("no competence in tech_builders domain") and popped when `TRX_0007` committed.

## Decision rules & edge cases

- **No open step in `plan.md`.** Every row `DONE` → nothing to act on. Return "no open step" to PLAN; do not invent work.
- **Step's dependency not actually `DONE`.** If the chosen row's `dep` is still `OPEN`/`blocked`, that is a PLAN inconsistency. Do not run it; hand the inconsistency back to PLAN rather than acting on a step with unmet deps.
- **Missing record (not in `INDEX.md`).** Treat as nonexistent. Go to step 8 (spawn subgoal). Never hallucinate the record's contents to keep moving.
- **`SERIES` vs `PARALLEL` ambiguity.** Default `SERIES`. Fan out only when independence, non-interference (no shared blackboard block), and budget all hold. When unsure, run serial.
- **Parallel branch times out.** Drop it; keep the branches that returned; stamp the reducer output `partial` and name the dropped branch. Never fabricate the missing branch's result.
- **Parallel branches conflict.** Keep the higher-provenance branch (seed/primary over repost/downstream). If provenance ties, keep both and flag the unresolved conflict for REFLECT; do not silently pick one.
- **Playbooks conflict.** Prefer the TRUSTED / higher win-rate playbook; record the conflict in the hand-off.
- **Volume looks overwhelming but origins are few.** The ≥N-origin gate is independent of volume. Fewer than 3 independent origins after dedup → MANUFACTURED, no matter how loud. Report `p(inflated)` with the origin count as basis.
- **Felt confidence above the cap or below the floor.** Clamp to `cap 0.90` / `floor 0.05` after applying the shrink rule. Never seal a bet outside `[0.05, 0.90]`.
- **A claim is not falsifiable.** It needs no bet, but it also may not be presented as a prediction. State it as a definition/constraint, with its source.
- **Transfer probe CI includes zero.** Do NOT commit the re-bind. Revise the analog or cold-start and hand that back to PLAN; do not run the full schema on an uncommitted binding.
- **Budget low mid-step.** Finish sealing any bet already in flight (so no claim travels unsealed), write what you have to the owned files, and hand back with a budget-low flag for PLAN's re-plan trigger. Never leave a traveled claim without its sealed bet.
- **Gate returns STOP.** Mark `blocked`, HALT. This overrides every other rule in this section — no edge case justifies routing around a STOP.

## Worked example (run R0631 / goal G-12)

**Context.** `INDEX.md` NOW = G-12 "weekly brief: AI-agent hype vs organic (tech_builders)", env `tech_builders` (TRANSFERRED from `punjab_diaspora`). The plan `goals/plan.md` lists S1..S6. This trace walks ACT executing the analytical heart of the brief (the S2→S3→S4 work) for run R0631.

1. **BIND + LOAD.** Bind to `tech_builders` (the `"active":true` record; `constraints` include `compliance-gate-mandatory` and `ToS:x-tech`; `platform = youtube+reddit+x-tech`). Read the open step from `plan.md`. Load the INDEX slice for `touches = {W-ai-agents, A-builders, P-hype-adjud, SCH-geo-link}`: `semantic/world/ai-agents.md` (carries belief **B2** "public AI-agent hype is mostly MANUFACTURED", conf `.74`), `semantic/audiences/builders.md`, `procedural/playbooks/hype-adjudication.md`, `schemas/SCH_geo_link_react_brief.md`.

2. **MATCH PLAYBOOKS.** The move is genuine-vs-manufactured adjudication, so `hype-adjudication.md` (TRUSTED, win 7/9) fires. The candidate lesson **L-23** ("≥3 platforms agreeing ≠ ≥3 origins — dedup BEFORE counting", seen 1×, e0631) is advisory here and reinforces the playbook's dedup-before-count step.

3. **EXECUTE — parallel reaction read (S2), then adjudicate (S3).** `S2` is tagged `PARALLEL` and the platform reads are independent, so fan out: each of youtube / reddit / x-tech writes its own block under `kernel/blackboard/tech_builders/_merge/S2/`. The reducer merges them, keeping the higher-provenance branch on overlap. Applying the playbook's provenance-dedup at `S3`: the cross-platform wave looks huge across 4 surfaces, but tracing each wave to its seed collapses it to **2 independent origins** (one platform's wave was a reposted seed). The ≥N-origin gate (N=3) is independent of volume → **2 < 3 → MANUFACTURED.** Emit `p(inflated)` high, with basis "2 deduped independent origins" — a probability, never a boolean.

4. **SEAL BETS.** Before any of this travels, two falsifiable claims are sealed into `calibration/predictions.jsonl`:
   - **P-114** "a cross-platform wave this size has ≥3 independent origins" — sealed pre-dedup at `confidence 0.65`, `conf_band 70` (the 50–70 band passes through `recal-v2` unchanged), `due_date 2026-06-28`, basis "surface volume across 4 platforms; NOT yet deduped", `node_id ai-agent-spike-0628`, `status:open`. (REFLECT later scored P-114 **refuted** — deduped to 2 origins, one reposted seed; brier 0.42. ACT only sealed it; it did not score it.)
   - **P-118** "this AI-agent topic re-spikes within 14 days" → operationalized as "a second organic spike (≥2 independent origins) by 2026-07-12" — felt ~0.45, shrunk via `recal-v2` and sealed at `confidence 0.35`, `conf_band 50`, `due_date 2026-07-12`, basis "manufactured single-origin waves usually fade; 35% it returns organically", `status:open`.

5. **TRANSFER (already committed, referenced at S4).** The brief is drafted via `SCH_geo_link_react_brief` re-bound to `tech_builders`. That re-bind was cheap-probed and committed earlier as **TRX_0007** (probe n=12, metric `link_validity@anchor`, lift `+0.23`, 95% CI `[0.06, 0.39]` excludes 0, ablation holds `+0.19`, ECE `0.04`, `schema.confidence[tech_builders]=0.71`). Because the binding is already committed, ACT runs the draft directly rather than re-probing; if it were the first run of a fresh binding, ACT would run the frozen probe first and commit only on a CI excluding zero.

6. **GATE (S5).** The drafted brief + its SAFETY FACTS are routed through `../GATE_STEP2.md` verbatim. The topic is a public AI-agent trend with no real identifiable person depicted, so the gate returns **GREEN** with a disclosure line; `CLEAR TO RENDER: yes`. The brief proceeds to emit. (Had it depicted a real founder + their voice/likeness in a political frame, the gate would STOP and ACT would mark the step `blocked` and HALT.)

7. **HAND BACK.** ACT hands REFLECT/Consolidate: the gate-cleared brief, the merged `kernel/blackboard/tech_builders/_merge/S2/` result, and sealed-bet ids `P-114`, `P-118`. ACT did NOT move belief **B2** from `.70`→`.74` (REFLECT/consolidation folded that after scoring), did NOT mark **P-114** refuted (REFLECT did), did NOT log candidate lesson **L-23** (REFLECT proposed it), and did NOT touch `schemas/transfer_log.md` (the schema/consolidation owner records TRX rows). ACT executed S2–S4, sealed the bets, merged the parallel substeps, deferred the safety call, and handed back.

## SAFETY GATE — FROZEN

> **SAFETY GATE — FROZEN. `mind_act` does NOT make the safety / compliance / ethics / legal decision.** That step is FROZEN and owned elsewhere. ACT calls `../GATE_STEP2.md` **exactly as written**, passes it the brain's drafted output plus the SAFETY FACTS, and **obeys its single verdict** — full stop. ACT must never re-implement, soften, re-judge, or route around the gate.

What the verdicts mean for ACT:

- **STOP** — the gate refused (a minor/protected person; a real identifiable person + political/election + their voice/likeness; fabricated wrongdoing / fake quotes / defamation; private-leaked or copyrighted material). ACT marks the step `blocked` in the hand-off and **HALTS**. Nothing renders, voices, or emits. ACT does not negotiate the STOP or look for a workaround.
- **YELLOW** — allowed with REQUIRED EDITS (public policy/topic made factual-only with disclosure; clearly-labeled parody with no voice clone; a real person from an unclear source held for a license/consent check). ACT applies every REQUIRED EDIT and the DISCLOSURE LINE, re-running the gate if the edits are material, and proceeds only as the gate allows.
- **GREEN** — your own original character/persona or a consented in-scope use. ACT proceeds and attaches the gate's DISCLOSURE LINE.

Only when the gate returns `CLEAR TO RENDER: yes` does any output finalize. This is a fail-closed, path-to-yes gate: it hard-stops the genuinely risky combinations and otherwise finds the safe way to ship. It is **not owned by this specialist** precisely so that execution pressure (a deadline, a hot topic, a calibrated bet ACT is confident in) can never bend the safety call — the separation is the safeguard. ACT's role and its escalation trigger both state this deferral explicitly: any safety/ethics/legal judgment is the gate's, and on STOP, ACT halts.

## Failure modes & escalation

**Failure modes that make this phase fail:**

- **Unsealed claim travels.** A falsifiable claim reaches the artifact or the hand-off without a sealed line in `calibration/predictions.jsonl`. This breaks calibration accountability downstream. Mitigation: seal before travel, always (step 4 precedes step 6 and 7).
- **Confidence not corrected.** Stating a felt 80% as 80% instead of the `recal-v2`-shrunk 72%. Mitigation: every confidence passes through `recalibration_map.json` and the `[0.05, 0.90]` clamp.
- **Counting platforms instead of origins.** Letting volume override the ≥N-origin gate, e.g. calling a 4-platform / 2-origin wave organic. Mitigation: provenance-dedup to seeds before counting; gate is volume-independent.
- **Fabricated parallel branch.** Inventing a timed-out branch's result instead of marking the merge `partial`. Mitigation: drop-and-stamp, never fill.
- **Transfer committed without a clean probe.** Running the full re-bound schema when the probe's 95% CI includes zero. Mitigation: cheap-probe gate (3A) blocks the commit.
- **Writing outside ownership.** Editing `plan.md`, `semantic/`, `procedural/`, `cursor.json`, `checkpoint.jsonl`, or scoring a bet. Mitigation: the single-write-owner contract below; hand back instead of writing.
- **Routing around the gate.** Emitting after a STOP, or softening REQUIRED EDITS. This is the most serious failure. Mitigation: the FROZEN gate deferral; STOP halts unconditionally.

**When to ESCALATE to a human** (a goal can only end DONE / ABANDONED / ESCALATED):

- The gate returns **STOP** on the only viable form of a required deliverable and there is no compliant repair — escalate the deliverable, do not route around the gate.
- A borderline / genuine-STOP case that the screening heuristic flags as needing qualified legal judgment (the gate itself notes a real STOP/borderline should go to a lawyer).
- A precondition subgoal cannot be satisfied and there is no analog or cold-start path (transfer probe keeps failing, needed record cannot be created), blocking the step indefinitely.
- Repeated gate STOPs on the same goal indicate the goal's direction is structurally non-compliant — escalate to a human to re-scope (this is an Agency/human call, surfaced by ACT's repeated `blocked` hand-offs).

## Handoff

**Next workflow:** REFLECT (phase 5 of 6), then CONSOLIDATE (phase 6).

**State ACT must leave behind for REFLECT:**

- The gate-cleared output (or, on STOP, the step marked `blocked` with the gate's REASON), ready for REFLECT to compare claimed vs realized vs baseline.
- The list of sealed-bet ids written this turn (e.g. `P-114`, `P-118`) with `status:open`, so REFLECT can score any that are due (it scored P-114 refuted, brier 0.42) and leave the rest open.
- The merged `kernel/blackboard/<env>/_merge/<step>` result, including any `partial` note naming dropped branches, so REFLECT knows coverage.
- Any playbook-conflict or provenance-tie flags raised during execution.
- Any subgoal request raised at step 8 (handed to Agency), with its answerhood condition.

**What ACT must NOT have done (left for the downstream phases):** scored outcomes, folded belief deltas (B2 `.70`→`.74` is REFLECT/consolidation's), written candidate lessons (L-23 is REFLECT's proposal), edited `semantic/` or `procedural/`, advanced `kernel/cursor.json`, or appended `episodic/journal/<today>.md` / `kernel/checkpoint.jsonl` (CONSOLIDATE's). REFLECT reads ACT's sealed bets and gate-cleared output; CONSOLIDATE later closes the turn by writing the diary, checkpoint, cursor advance, and INDEX update, and applies the proposed belief/lesson deltas at the scheduled consolidation pass.

## Single-write-owner contract

**ACT may write — and only these (append-only, never overwriting another faculty's lines):**

- `calibration/predictions.jsonl` — one sealed BET per falsifiable claim, written before the claim travels, with `status:"open"` and null outcome/score fields. **Calibration owns this file.**
- `kernel/blackboard/<env>/_merge/<step>` — for a `PARALLEL` step, each independent substep's block and the reducer's merged output (for this run, under `kernel/blackboard/tech_builders/_merge/<step>/`). **Kernel owns the blackboard.**

**ACT must never touch:**

- `goals/plan.md`, `goals/goals.md`, `goals/direction.md`, `goals/subgoals.md` (Agency/PLAN — ACT raises subgoal requests, does not write them).
- `attention/*` (Attention ranks questions and logs surprises).
- `experience/outcomes.jsonl`, `experience/_ledger.jsonl`, the `outcome`/`scored_ts`/`brier` fields of any prediction (REFLECT scores).
- `semantic/*` and `procedural/*` (consolidation applies belief/lesson deltas).
- `schemas/transfer_log.md`, `schemas/SCH_*`, `schemas/abstractions.md` (the schema/consolidation owner records TRX rows and schema edits; ACT runs the probe and reports the result, it does not write the log).
- `kernel/cursor.json`, `kernel/checkpoint.jsonl`, `episodic/journal/<today>.md`, `episodic/timeline.md`, `INDEX.md` (CONSOLIDATE advances the cursor, checkpoints, writes the diary, and updates INDEX).

Any change ACT wants in a file it does not own is expressed as a hand-off signal to the owning faculty, not as a write.
