# W1_perceive — PERCEIVE — bind environment & surface the top question

> Bind this turn to the one `active:true` environment in `kernel/environments.jsonl`, load only that record's `context_slice` + `constraints`, surface the single highest value-of-information open question from `attention/questions.md`, screen it for false presupposition, and emit one focus line — nothing answered, recalled, planned, or executed. Owner specialist: `mind_perceive` (the PERCEIVE faculty of ONE MIND, owned by Attention, weighted erotetic). Position in the loop: step 1 of 6 (PERCEIVE → RECALL → PLAN → ACT → REFLECT → CONSOLIDATE). Single-write-owner files: the one focus line in working scratch (`working/focus` for the bound env, e.g. `kernel/blackboard/tech_builders/focus`), and — only when a surprise has changed value-of-information inputs — re-rank/flag edits inside `attention/questions.md` (Attention owns `attention/*`).

## Purpose

PERCEIVE exists to make the rest of the turn HONEST about two things it would otherwise assume: *which world am I in*, and *what is the one question worth asking right now*. Everything downstream (RECALL pulling analogous episodes, PLAN setting a falsifiable step, ACT gathering evidence, REFLECT scoring a prediction, CONSOLIDATE writing memory) inherits the frame this phase binds. If PERCEIVE binds the wrong environment, every later phase loads the wrong context slice and the diary's LOADED line lies. If PERCEIVE surfaces a loaded question (one with a false presupposition), ACT wastes the turn answering something that was never true.

Concretely, PERCEIVE does five things and stops:

1. **Resolve** the single active environment. Exactly one record in `kernel/environments.jsonl` must carry `active:true`. In run R0631 that is `tech_builders` (domain `ai-agents/dev-tools`); `punjab_diaspora` is `active:false` and is ignored entirely.
2. **Bind** that environment's `domain`, `audience`, `platform`, `context_slice`, and `constraints` as the frame for the whole turn, and note the transfer provenance (`tech_builders` was transferred from `punjab_diaspora` via `SCH_geo_link_react_brief`, committed as `TRX_0007`) so downstream phases know the env is borrowed, not native.
3. **Surface** the highest value-of-information open question from `attention/questions.md`, where value-of-information = goal-impact × uncertainty ÷ cost.
4. **Screen** that question (and every candidate it outranks or is outranked by) against `attention/surprises.md` and established memory for a false or never-measured presupposition; FLAG and park any loaded question instead of answering it.
5. **Emit** one focus line into working scratch and hand the bound frame + top question to RECALL.

PERCEIVE never answers the question, never gathers evidence, never recalls episodes, never sets direction, never picks a plan step, never executes, never scores a prediction. Those are ACT / RECALL / PLAN / REFLECT work. When the active environment is ambiguous, or the top surviving question has no statable answerhood condition, or every queued question is a flagged false-presupposition, PERCEIVE halts and escalates rather than improvising a focus.

## Inputs (memory read) and Outputs (memory written)

All paths are relative to the mind memory root `branches/b60_content_intelligence/mind/memory/`.

**Inputs — READ (in this order):**

- `kernel/environments.jsonl` — the environment registry. Select the one record with `active:true`. (R0631: `tech_builders` active; `punjab_diaspora` `active:false`.)
- The active record's `context_slice` and `constraints` fields ONLY — loaded inline from the same JSONL line. (R0631: `context_slice` = "transferred from punjab_diaspora via SCH_geo_link_react_brief; medium-tier re-measured per niche"; `constraints` = `["safe-lanes-only","compliance-gate-mandatory","ToS:x-tech"]`.)
- `attention/questions.md` — the value-of-information-ranked question queue. Read the VoI column, status column, type, presupposition, answerhood-cond, source.
- `attention/surprises.md` — read ONLY the lines cited as a question's `source` (provenance confirmation). (R0631: Q-OQ-07's source is S-009; glance at S-009 to confirm the AI-agent wave deduped to 2 origins, not the ≥3 of refuted prediction P-114.)
- `INDEX.md` NOW-block — read to confirm the active goal (`G-12 "weekly brief: AI-agent hype vs organic (tech_builders)"`) and that the env transfer provenance matches.
- `kernel/cursor.json` — read to confirm `env_id` and `goal` agree with the active environment, and that the prior run consolidated (`note: "R0631 complete and consolidated"`).

**Do NOT read** in this phase (those belong to RECALL or later): `experience/episodes.jsonl`, `experience/outcomes.jsonl`, `experience/reflections/*`, `semantic/world/*`, `semantic/audiences/*`, `semantic/glossary.md`, `procedural/playbooks/*`, `procedural/lessons.md`, `goals/plan.md`, `goals/subgoals.md`, `calibration/*`, `schemas/abstractions.md`. (Exception: you may glance at `semantic/world/ai-agents.md` ONLY to confirm whether a candidate question's presupposition was ever *measured* during the loaded-question screen — never to load beliefs as turn context. Reading a belief's existence is a screen; importing it as evidence is RECALL's job.)

**Outputs — WRITE (single-write-owner, and only these):**

- A one-line focus statement into working scratch for the bound env — `kernel/blackboard/tech_builders/focus` (the working/focus slot named in INDEX's TIERS line `working: focus(3) · blackboard/tech_builders/`). This line is yours and only yours.
- WHEN JUSTIFIED ONLY: a re-rank and/or flag update inside `attention/questions.md` (Attention owns `attention/*`). Justified means a surprise has changed a value-of-information input, or a presupposition screen newly flips a presupposition to FALSE. You append/adjust the VoI value and reorder rows, or set `status` to `flagged` and `presupposition` to FALSE with a `→ rewrite` route. You never delete a row and never silently change another row's wording, id, type, answerhood-cond, or source.

**Never write** (these belong to PLAN / ACT / REFLECT / CONSOLIDATE or other faculties): `goals/*`, `kernel/cursor.json`, `kernel/checkpoint.jsonl`, `kernel/turn_log.jsonl`, `kernel/environments.jsonl`, `kernel/schedule.jsonl`, `episodic/*`, `experience/*`, `semantic/*`, `procedural/*`, `calibration/*`, `schemas/*`, `_meta/*`, `INDEX.md`.

## Protocol

Run every step every turn. Do not skip a step because the previous turn's answer "looks the same" — the active environment and the queue can change between turns.

### Step 1 — Resolve the active environment

1.1 Open `kernel/environments.jsonl`. Parse each line as one environment record.

1.2 Filter to records where `active == true`. Count them.

- **Exactly one active** → proceed with that record. In R0631 the records are `punjab_diaspora` (`active:false`) and `tech_builders` (`active:true`); the unique active record is `tech_builders`.
- **Zero active** → HALT. Do not pick the most recent, the first, or the highest-confidence env. Escalate (see Failure modes). A turn with no bound world cannot honestly load a context slice.
- **Two or more active** → HALT and escalate. Two active environments means the registry is in a state the kernel forbids; guessing would silently bind the turn to one world while another claims to be live.

1.3 Record the resolved `env_id` (R0631: `tech_builders`). This identifier is the key for the working/focus slot and for cross-checks in Step 2.

### Step 2 — Bind the environment as the turn frame

2.1 From the active record ONLY, load these fields as the frame: `domain`, `audience`, `platform`, `context_slice`, `constraints`. For R0631:

- `domain`: `ai-agents/dev-tools`
- `audience`: `early-stage builders + devs, global`
- `platform`: `youtube+reddit+x-tech`
- `context_slice`: `transferred from punjab_diaspora via SCH_geo_link_react_brief; medium-tier re-measured per niche`
- `constraints`: `safe-lanes-only`, `compliance-gate-mandatory`, `ToS:x-tech`

2.2 Treat these as the binding constraints for the ENTIRE turn. They are not advisory. `safe-lanes-only` and `compliance-gate-mandatory` mean ACT and the eventual emission must clear the compliance gate; `ToS:x-tech` means platform `x-tech` carries its own terms-of-service envelope. PERCEIVE does not enforce these (it does not act), but it carries them into the frame so PLAN/ACT inherit them and so the diary's LOADED line is honest about what was bound.

2.3 Read the transfer provenance out of `context_slice`: `tech_builders` was transferred FROM `punjab_diaspora` via schema `SCH_geo_link_react_brief`. Cross-check against `INDEX.md` NOW-block (`note: this env (tech_builders) was TRANSFERRED from punjab_diaspora via SCH_geo_link_react_brief (TRX_0007)`). Record `TRX_0007` as the transfer id so downstream phases know the env is BORROWED — its bindings (anchor, culture_codes, platform_set) were re-bound per `SCH_geo_link_react_brief`, and "medium-tier is the opportunity" is a HYPOTHESIS re-measured per domain, not an inherited fact.

2.4 Cross-check the bound `env_id` against `kernel/cursor.json`. Confirm `cursor.env_id == "tech_builders"` and `cursor.goal == "G-12"`. Confirm the prior run consolidated (`cursor.note` contains "complete and consolidated"; R0631: `"R0631 complete and consolidated; next turn picks up wk_024/wk_025 or a new goal"`). If the cursor's `env_id` disagrees with the resolved active env, HALT and escalate — a disagreement means the previous turn's CONSOLIDATE and the environment registry are out of sync, and binding either one blindly would be a guess.

2.5 Cross-check the active goal against `INDEX.md` NOW-block: `goal: G-12 "weekly brief: AI-agent hype vs organic (tech_builders)"`. The frame now consists of {env_id, domain, audience, platform, context_slice, constraints, transfer provenance, active goal}. This is the bound frame handed to RECALL.

### Step 3 — Surface the top open question by value-of-information

3.1 Open `attention/questions.md`. Each row carries: `QID | status | type | question | answerhood-cond | presupposition | VoI | source`.

3.2 Compute/confirm value-of-information for each row: **VoI = goal-impact × uncertainty ÷ cost**. The queue stores the VoI value per row; you confirm it is current (Step 6 may re-rank if a surprise changed an input). For R0631 the stored values are Q-OQ-07 = 0.81, Q-018 = 0.64, Q-019 = 0.58, Q-016 = (none; flagged).

3.3 Keep only rows whose `status` is `active` or `open`. Drop `flagged` rows from the candidate-for-surfacing set (they remain in the file; they are simply not eligible to become the focus). In R0631, eligible = {Q-OQ-07 (active), Q-018 (open), Q-019 (open)}; Q-016 is `flagged` and not eligible.

3.4 Sort the eligible set by VoI descending. The leader is the highest-VoI eligible row: R0631 → Q-OQ-07 at VoI 0.81.

3.5 The leader is a *candidate*, not yet the surfaced question. It must survive Step 4 (typing + answerhood) and Step 5 (loaded-question screen) before it is surfaced. If the leader fails either screen, demote it per the rules below and re-test the next-highest eligible row.

### Step 4 — Erotetic typing and answerhood condition

4.1 Confirm the candidate carries an explicit **type**: `polar` (yes/no), `wh` (which/who/where/what), or `why` (cause). R0631 Q-OQ-07 type = `polar`.

4.2 Confirm the candidate carries an explicit **presupposition** — the thing that must be true for the question to even make sense. R0631 Q-OQ-07 presupposition = "2 distinct origins exist (CHECK)".

4.3 Confirm the candidate carries an explicit **answerhood condition** — the statable thing that would count as having answered it. R0631 Q-OQ-07 answerhood-cond = "a deduped funding/ownership trace links or separates them". A polar question's answerhood condition must name what evidence flips the yes/no; a wh question's must name what a complete enumeration looks like; a why question's must name what counts as the cause.

4.4 **Answerhood gate:** a question with no statable answerhood condition is NOT yet a question. Set its `status` to flagged with route `→ repair` (it needs an answerhood condition written before it can be surfaced) and do NOT surface it. Re-test the next-highest eligible row. (R0631: every eligible row has an answerhood condition, so none is repaired here. Q-016, already flagged, carries answerhood-cond `—` precisely because a false-presupposition question has no statable answer.)

### Step 5 — Loaded-question screen (false-presupposition flagging)

5.1 For EACH candidate (the leader and any row you might fall through to), evaluate its presupposition against what is actually ESTABLISHED in memory — not against what the surface implies. The check is binary on three states:

- **TRUE** (the presupposition was measured and holds) → presupposition passes; the question is answerable in principle.
- **FALSE / never-measured** (the presupposition is contradicted by, or was never measured in, established memory) → FLAG. Set `status: flagged`, mark `presup FALSE`, route `→ rewrite`. NEVER answer a loaded question. Demote it out of the eligible set and re-test the next row.
- **CHECK** (the presupposition is plausible but not yet verified for THIS node) → surface the question but carry the CHECK forward so a later phase (ACT) verifies the presupposition before treating its conclusion as load-bearing. Do not resolve the CHECK yourself.

5.2 Apply to R0631:

- **Q-016** ("Have we stopped over-counting bot comments as real?") presupposes that bot ground-truth was measured. Established memory says it never was. So Q-016's presupposition is FALSE; it stays `flagged`, `presup FALSE: bot ground-truth was never measured (FALSE_PRESUP → rewrite, don't answer)`, and is SKIPPED — never answered, never surfaced.
- **Q-OQ-07's own presupposition** ("2 distinct origins exist") is marked **CHECK**, not TRUE. The dedup in S-009 found 2 *apparent* independent origins, but whether they are genuinely 2 distinct origins or 1 operator wearing two masks is exactly what the question interrogates. So surface Q-OQ-07 AND carry the CHECK forward: ACT must verify the 2-origins presupposition (via the deduped funding/ownership trace) rather than assume it. If ACT finds the "2 origins" are 1 operator, belief B2 ("public AI-agent hype is mostly MANUFACTURED", conf .74) would shift toward "coincidence/common-method", which is why the question is the highest-VoI item on the queue.

5.3 The screen draws provenance from `attention/surprises.md` for any candidate whose `source` cites a surprise. Q-OQ-07's `source` is `surprise S-009`. Glance at S-009 to confirm: observation = "the AI-agent cross-platform wave deduped to only 2 origins, not the 4+ the surface implied"; violated-prediction = P-114 ("expected ≥3 independent origins for a wave this size"); magnitude 0.78; spawned Q-OQ-07. This confirms the question's provenance is real and that P-114 was refuted (Brier 0.42) — the surprise that legitimately raised this question's VoI to the top of the queue.

5.4 A candidate that passes typing (Step 4), has a stated answerhood condition, and whose presupposition is TRUE or CHECK (not FALSE) is the **surfaced question**. R0631 → Q-OQ-07 (polar, VoI 0.81, presupposition CHECK, answerhood = deduped funding/ownership trace).

### Step 6 — Re-rank the queue (only when a surprise changed a value-of-information input)

6.1 Re-ranking is permitted because Attention owns `attention/*`. It is JUSTIFIED only when a logged surprise has changed a goal-impact, uncertainty, or cost input feeding VoI for one or more rows, or when Step 5 newly flipped a presupposition to FALSE (which removes a row from eligibility and may reorder the rest).

6.2 When re-ranking: recompute VoI for affected rows, write the new value into the `VoI` column, and reorder rows by VoI descending. PRESERVE every row's `QID`, `type`, `presupposition`, `answerhood-cond`, and `source` verbatim. NEVER delete a row (a question, even a flagged one, is audit history). NEVER silently change another row's wording.

6.3 In R0631 the surprise S-009 had ALREADY been folded into the queue by the prior turn's CONSOLIDATE (Q-OQ-07 sits at 0.81 driven by S-009's magnitude 0.78). No new surprise has landed this turn, so the order is unchanged and no write to `attention/questions.md` is needed. PERCEIVE leaves the queue exactly as found.

6.4 If you DO write, the write is confined to `attention/questions.md`. You do not touch any other file to record the re-rank.

### Step 7 — Emit the focus line

7.1 Compose ONE line and write it to the working/focus slot for the bound env (`kernel/blackboard/tech_builders/focus`). The line is the entire output payload of this phase. Format (fields separated by ` · `):

`PERCEIVE <run> · env <env_id> · top-Q <QID> (<type>, VoI <value>, presup <TRUE|CHECK>) · flagged <QID>(<reason>) · focus: <one-line restatement of the surfaced question>`

7.2 For R0631 the focus line is:

`PERCEIVE R0631 · env tech_builders · top-Q Q-OQ-07 (polar, VoI .81, presup CHECK) · flagged Q-016 (false-presup) · focus: is the 2-origin AI-agent wave one operator or two?`

7.3 The line records: the run, the bound env, the surfaced question with its type/VoI/presupposition-state, any flagged false-presupposition question that was parked (so the audit trail shows what was deliberately NOT answered), and a one-line natural-language restatement of the focus. This is the honest "what I am asking now" that the diary's later LOADED line will reference.

### Step 8 — Stop and hand off

8.1 Stop. Do not answer, recall, plan, or act. Hand the bound env frame + surfaced top question + focus line to RECALL.

8.2 The carried-forward CHECK on Q-OQ-07's presupposition travels with the handoff so PLAN/ACT know to verify "2 distinct origins exist" before treating the manufactured-hype conclusion as established.

## Decision rules & edge cases

- **Zero active environments** → HALT, escalate. No improvising a frame from cursor or INDEX.
- **Two-plus active environments** → HALT, escalate. The registry violates the one-world invariant; binding either is a guess.
- **Active env disagrees with `kernel/cursor.json`** → HALT, escalate. The previous CONSOLIDATE and the registry are out of sync.
- **Active env's goal disagrees with `INDEX.md` NOW-block** → prefer the active env record + cursor agreement; if all three (env, cursor, INDEX) cannot be reconciled, escalate. In R0631 all three agree on `tech_builders` / `G-12`.
- **Top-VoI eligible question has no answerhood condition** → flag `→ repair`, demote, fall through to next eligible row. If you reach the bottom of the eligible set with nothing repaired, escalate (no surfaceable question).
- **Every queued question is flagged false-presupposition** → HALT, escalate. There is no honest focus to emit; surfacing a flagged question would be answering a loaded question by the back door.
- **Top question's presupposition is FALSE** → flag `→ rewrite`, demote, take the next-highest eligible row. Q-016 demonstrates this: its presupposition (bot ground-truth measured) is false, so it is parked, not surfaced, regardless of any VoI it might otherwise carry.
- **Top question's presupposition is CHECK** → surface it and carry the CHECK forward; do NOT resolve the CHECK (that is ACT's verification work). Q-OQ-07 demonstrates this.
- **Tie in VoI between two eligible questions** → break the tie by lower cost first, then by higher magnitude of the spawning surprise (`attention/surprises.md`), then by earlier QID. Record the tie-break in the focus line if it changed the surfaced question.
- **A surprise landed this turn that changes VoI inputs** → re-rank `attention/questions.md` (Step 6), preserving every row's id/type/presupposition/answerhood-cond/source; never delete a row. If no surprise landed, leave the queue untouched (R0631 case).
- **Inquiry-budget interaction** → `attention/inquiry_budget.md` sets `explore_ratio: 0.20` (turns_this_cycle 12, explore_turns_spent 2). PERCEIVE does not spend the budget (it does not explore), but if the surfaced top question is an EXPLORE-type probe and the explore ratio is already exhausted for the cycle, note that in the focus line so PLAN knows the budget posture; PERCEIVE still surfaces the highest-VoI question — it does not silently swap to an exploit question to dodge the budget. In R0631 Q-OQ-07 is an exploit-leaning verification of B2, consistent with the policy-note "keep exploiting the honesty-adjudication edge while it's confirmed."
- **Context-slice budget** → INDEX's RETRIEVAL RULE caps load at ~6k tokens. PERCEIVE's reads (one env record's slice, the question queue, cited surprise lines, NOW-block, cursor) sit well under this. If the env record or queue is anomalously large, read only `context_slice` + `constraints` from the env line and the columns of `attention/questions.md` — do not pull the full semantic/episodic tiers to "understand" a question; that is RECALL's budgeted job.
- **A needed record is absent** → per INDEX's retrieval rule, "if a needed record isn't listed here, it doesn't exist yet — create it, don't hallucinate it." But CREATION of memory records is not PERCEIVE's write authority. If the active env record is missing entirely, or `attention/questions.md` is empty, escalate rather than fabricate an env or a question.

## Worked example (run R0631 / goal G-12)

**Setup.** Run R0631, goal `G-12` ("ship a weekly hype-vs-organic brief in the `tech_builders` environment"), prior turn consolidated.

**Step 1 — Resolve.** `kernel/environments.jsonl` has two records. `punjab_diaspora` is `active:false` (the original beachhead) → ignored. `tech_builders` is `active:true` → the unique active env. Resolved `env_id = tech_builders`.

**Step 2 — Bind.** Frame loaded from the `tech_builders` record only: domain `ai-agents/dev-tools`, audience `early-stage builders + devs, global`, platform `youtube+reddit+x-tech`, context_slice "transferred from punjab_diaspora via SCH_geo_link_react_brief; medium-tier re-measured per niche", constraints `safe-lanes-only` + `compliance-gate-mandatory` + `ToS:x-tech`. Transfer provenance: env BORROWED from `punjab_diaspora` via `SCH_geo_link_react_brief`, committed as `TRX_0007` (per `schemas/transfer_log.md`: probe lift +0.23, 95% CI [0.06, 0.39] excludes 0 → COMMIT, schema.confidence[tech_builders]=0.71; ablation dropped all punjab bindings and lift held +0.19 → ACQUIRED here, not just borrowed prior). Cross-check: `kernel/cursor.json` has `env_id:"tech_builders"`, `goal:"G-12"`, note "R0631 complete and consolidated". `INDEX.md` NOW-block confirms goal `G-12` and the transfer note. All three agree → frame is honest.

**Step 3 — Surface by VoI.** `attention/questions.md` eligible rows (status active/open): Q-OQ-07 (VoI 0.81), Q-018 (0.64), Q-019 (0.58). Q-016 is `flagged` → not eligible. Sort descending → leader = Q-OQ-07 at 0.81 ("Are the 2 'independent' origins of the AI-agent spike actually funded by one operator?").

**Step 4 — Typing + answerhood.** Q-OQ-07 is `polar`, presupposition "2 distinct origins exist (CHECK)", answerhood-cond "a deduped funding/ownership trace links or separates them". All three present → passes the answerhood gate.

**Step 5 — Loaded-question screen.**
- Q-016 ("have we stopped over-counting bot comments?") presupposes bot ground-truth was measured; it never was → presupposition FALSE → stays flagged, route `→ rewrite`, SKIPPED, never answered.
- Q-OQ-07's presupposition ("2 distinct origins exist") is CHECK: the S-009 dedup found 2 *apparent* origins, but the question is precisely whether those 2 collapse to 1 operator. Surface Q-OQ-07 AND carry the CHECK forward — ACT must verify the 2-origins presupposition via the funding/ownership trace, not assume it.
- Provenance confirm: Q-OQ-07's source S-009 in `attention/surprises.md` — observation "the cross-platform wave deduped to only 2 origins, not the 4+ the surface implied", violated-prediction P-114 ("≥3 independent origins for a wave this size", `calibration/predictions.jsonl` status `refuted`, Brier 0.42), magnitude 0.78. The refutation of P-114 is the surprise that legitimately put Q-OQ-07 at the top. Relevant belief context (existence-check only, not loaded as evidence): `semantic/world/ai-agents.md` B2 "public AI-agent hype is mostly MANUFACTURED" conf .74 (raised .70→.74 this run per the journal), and conditional-known C1 "if search-interest is independent of platform signal → B2 strengthens, status UNVERIFIED → Q-OQ-07". If ACT later finds the 2 origins are 1 operator, B2's basis shifts from "manufactured" toward "coincidence/common-method" — which is exactly the stake Q-OQ-07 measures.

**Step 6 — Re-rank?** S-009 was already folded into the queue by the prior CONSOLIDATE (Q-OQ-07 at 0.81 reflects magnitude 0.78). No new surprise landed this turn → order unchanged → no write to `attention/questions.md`. Candidate lesson `L-23` ("≥3 platforms agreeing ≠ ≥3 origins — dedup BEFORE counting", `procedural/lessons.md`, seen 1× in e0631, needs 1 more independent confirm) is noted as the procedural backdrop for why dedup-before-counting matters here, but PERCEIVE does not read or promote it — that is CONSOLIDATE's authority.

**Step 7 — Emit.** Write to `kernel/blackboard/tech_builders/focus`:

`PERCEIVE R0631 · env tech_builders · top-Q Q-OQ-07 (polar, VoI .81, presup CHECK) · flagged Q-016 (false-presup) · focus: is the 2-origin AI-agent wave one operator or two?`

**Step 8 — Handoff.** Stop. Hand to RECALL the bound `tech_builders` frame + surfaced Q-OQ-07 + the CHECK on its presupposition + the focus line. PERCEIVE answers nothing, recalls no episode, sets no direction, picks no step, scores no prediction.

## Failure modes & escalation

- **Ambiguous environment (zero or two-plus `active:true`)** → ESCALATE to a human. The one-world invariant is broken; any guess silently binds the turn to a world the registry does not uniquely endorse.
- **Env / cursor / INDEX disagreement that cannot be reconciled** → ESCALATE. Binding an env that the cursor or INDEX contradicts produces a dishonest LOADED line and corrupts every downstream phase.
- **Surfaced top question has no answerhood condition and none of the fall-through rows do either** → ESCALATE. There is nothing the loop can be RIGHT-or-WRONG about; PLAN cannot derive a falsifiable criterion from an unanswerable question.
- **Every queued question is flagged false-presupposition** → ESCALATE. Surfacing a flagged question would be answering a loaded question by the back door; emitting no focus at all stalls the loop. A human must seed a screened question or repair a presupposition.
- **`attention/questions.md` empty, or active env record missing** → ESCALATE. PERCEIVE has no authority to author an environment or a question; fabricating either violates the single-write-owner contract and the "create it, don't hallucinate it" rule (and creation here belongs to other faculties, not PERCEIVE).
- **Tempted to answer / recall / plan / act** → this is the defining failure of the phase. If the only sensible next move is to ANSWER, that is the signal to HAND OFF to ACT, not to do it here. If the question needs structurally-similar past work, hand off to RECALL. If it needs a falsifiable success criterion or step, hand off to PLAN. PERCEIVE that answers its own question has destroyed the separation that keeps the loop honest.
- **Re-rank that deletes or rewrites a foreign row** → contract violation. Re-ranking may only recompute VoI and reorder; ids, types, presuppositions, answerhood-conditions, and sources are immutable to PERCEIVE, and no row is ever deleted.

Escalation = stop the turn, write nothing to working/focus, leave `attention/questions.md` untouched, and surface the specific anomaly (which invariant broke, which records disagree) to a human. Do not advance the cursor or checkpoint — those are CONSOLIDATE's, and an escalated turn did not complete.

## Handoff

**Runs next:** RECALL (step 2 of the loop, `W2_recall`).

**State PERCEIVE must leave behind for RECALL:**

- One focus line written to the working/focus slot for the bound env (`kernel/blackboard/tech_builders/focus`), in the Step 7 format, naming the surfaced question, its type, its VoI, and its presupposition state (CHECK in R0631).
- The bound env frame available to RECALL: env_id `tech_builders`, domain `ai-agents/dev-tools`, audience, platform, `context_slice`, `constraints`, transfer provenance (`SCH_geo_link_react_brief` / `TRX_0007`), and active goal `G-12`.
- The surfaced top question (R0631: Q-OQ-07, polar) with its answerhood condition and its carried-forward CHECK on the "2 distinct origins" presupposition.
- Any flag/re-rank edits to `attention/questions.md` committed (R0631: none — queue unchanged).

RECALL then pulls the structurally-similar episodes, beliefs, playbooks, and semantic records the surfaced question needs (e.g. `procedural/playbooks/hype-adjudication.md` for the ≥N-origin gate, `semantic/world/ai-agents.md` B2, the e0631 reflection) under its own ~6k-token budget. PERCEIVE deliberately did NOT pull any of those — that is RECALL's job, and doing it here would have blurred the phase boundary.

## Single-write-owner contract

**PERCEIVE MAY write — and only these:**

- The single focus line in the working/focus slot for the bound env: `kernel/blackboard/tech_builders/focus` (one line, owned exclusively by PERCEIVE).
- `attention/questions.md` — re-rank (recompute VoI, reorder) and/or flag updates (set `status: flagged`, `presup FALSE`, route `→ rewrite` or `→ repair`), ONLY when justified by a surprise that changed a VoI input or a presupposition that newly screens FALSE. Attention owns `attention/*`. Every row's `QID`, `type`, `presupposition`, `answerhood-cond`, and `source` is preserved verbatim; no row is ever deleted; no foreign row's wording is silently changed.

**PERCEIVE MUST NEVER touch:**

- `goals/goals.md`, `goals/direction.md`, `goals/plan.md`, `goals/subgoals.md` (PLAN / goal faculties).
- `kernel/cursor.json`, `kernel/checkpoint.jsonl`, `kernel/turn_log.jsonl`, `kernel/environments.jsonl`, `kernel/schedule.jsonl` (advancing the cursor, appending the checkpoint, and scheduling are CONSOLIDATE's / the kernel's job).
- `episodic/*` (journal, timeline — CONSOLIDATE writes the diary).
- `experience/*` (episodes, outcomes, reflections, ledger — ACT / REFLECT / CONSOLIDATE).
- `semantic/*` (world, audiences, glossary — CONSOLIDATE on belief Δ).
- `procedural/*` (playbooks, lessons — CONSOLIDATE on lesson promotion; L-23 stays a candidate until CONSOLIDATE promotes it).
- `calibration/*` (predictions, recalibration_map, report — REFLECT scores, CONSOLIDATE recalibrates).
- `schemas/*` (schema definitions, abstractions, transfer_log — schema/transfer faculties).
- `_meta/*`, `INDEX.md` (CONSOLIDATE / meta).

The contract is the phase boundary made literal: PERCEIVE surfaces and frames the question; it never answers it, gathers evidence, recalls an episode, sets direction, picks a step, executes, or scores a prediction. A loaded question is flagged and parked here, never resolved here.
