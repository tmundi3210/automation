# W5_reflect — REFLECT — did it ACTUALLY work? (vs a baseline) & distil a lesson

> The verdict beat of the ONE MIND turn: score what ACT sealed against its frozen baseline (never vs zero), collapse correlated episodes into one cluster, settle every due bet, and distil at most one lesson — or NO-LESSON. Owner specialist: `mind_reflect` (the Learning faculty, eval_heavy). Position: step 5 of 6 in the loop, after ACT seals the episode and before CONSOLIDATE ("sleep") applies proposals. Single-write-owner of `experience/*`: `experience/outcomes.jsonl`, `experience/reflections/<ep>.md`, `experience/_ledger.jsonl`, plus the per-prediction outcome stamps in `calibration/predictions.jsonl` and one append to `attention/surprises.md` for surprise→WHY handoff.

## Purpose

ACT produced a claim and sealed an episode; SENSE/THINK earlier registered bets. REFLECT is the only beat that asks whether the work *actually* paid out and turns that answer into durable, leakage-controlled memory. It exists to keep the mind honest against three failure pressures:

1. **Vs-zero inflation.** A naive read scores realized output against doing nothing, which manufactures wins. REFLECT forces every effect to be measured as `realized − frozen_baseline`. In run R0631 the baseline is a human "AI agents are blowing up!" trend post (`baseline_used: human_curated_trend_post`), so a brief that correctly calls the spike MANUFACTURED only "wins" to the extent it beats *that* post, not to the extent it beats silence.
2. **Pseudo-replication.** Two episodes that share a method/source family are one piece of evidence, not two. REFLECT collapses correlated episodes into a single cluster before counting confirms, so a lesson cannot be promoted on what is really n=1.
3. **Leakage / post-hoc baselines.** If the baseline or the pre-registration was not frozen *before* the outcome was visible, the comparison is contaminated and no valid effect exists. REFLECT re-freezes first and refuses to invent a baseline after the fact.

The deliverable is a resolved outcome row, a human-readable reflection, a ledger verdict, scored predictions, and (only when earned) a candidate or CONFIRM lesson — all as *proposals*. REFLECT never edits the semantic or procedural tiers itself; the scheduled consolidation "sleep" applies what REFLECT proposes.

## Inputs (memory read) and Outputs (memory written)

All paths are relative to `branches/b60_content_intelligence/mind/memory/`.

**READS (in this fixed order):**

| Order | Path | Why |
|---|---|---|
| 1 | `experience/episodes.jsonl` | The active sealed episode `e0631`: its `claim`, frozen `baseline_used` (`human_curated_trend_post`), `prereg_hash` (`7f2a9c`), `status:OPEN`. This is the contract being adjudicated. |
| 2 | `calibration/predictions.jsonl` | Every bet; REFLECT scores only those with `due_date <= today` (`P-114` due 2026-06-28). `P-118` (due 2026-07-12) is read but left untouched. |
| 3 | `attention/surprises.md` | So a surprise already queued is not double-logged (`S-009` from `P-114` is already present). |
| 4 | `experience/_ledger.jsonl` | Existing verdict rows and lesson clusters — to know what is already confirmed and how clusters are defined (`L-hype-adjud` already has an `e0625` CONFIRM in cluster `agent-frameworks`). |
| 5 | `experience/outcomes.jsonl` | Already-resolved episodes — to avoid re-resolving and to keep `ep` linkage unique. |
| 6 | `schemas/transfer_log.md` | Only when the step ran a transferred schema; supplies the pre-registered ablation result (`TRX_0007`: lift holds +0.19 after dropping all punjab bindings). |

**Supporting reads (context only, never written by this phase):** `INDEX.md` (the map), `goals/goals.md` (G-12 success criterion), `semantic/world/ai-agents.md` (beliefs B1/B2/B3, to know B3 is DO-NOT-CHASE), `procedural/playbooks/hype-adjudication.md` (the move being adjudicated), `calibration/recalibration_map.json` (the active band map, read-only — used to interpret a prediction's `conf_band`, never rewritten).

**WRITES (single-write-owner):**

| Path | Operation | Content shape |
|---|---|---|
| `experience/outcomes.jsonl` | append one row keyed by `ep` | `{ep, resolved_ts, verdict, claimed, realized, baseline, effect_vs_baseline, ci, cluster, cluster_n, scored_predictions:[...], transfer_label, note}` |
| `experience/reflections/<ep>.md` | write one file (`reflections/e0631.md`) | six fixed lines: WHAT-HAPPENED / WHAT-WORKED / WHAT-DIDNT / WHY / DID-IT-WORK?(vs baseline, with cluster n) / PROPOSED |
| `experience/_ledger.jsonl` | append verdict row(s) | `{lesson, ep, result, delta, cluster, note}` |
| `calibration/predictions.jsonl` | in-place stamp of due rows only | set `status`, `outcome`, `scored_ts`, `brier` on each due prediction |
| `attention/surprises.md` | append rows only | one surprise row + the WHY/POLAR question id it spawns, for each refuted bet not already queued |

**MUST NEVER TOUCH:** `semantic/*`, `procedural/*`, `calibration/recalibration_map.json`, `calibration/calibration_report.md`, `attention/questions.md` (beyond the surprise→question handoff that Attention owns the answering of), `goals/*`, `kernel/*`, the compliance gate, and the episode/claim that ACT sealed.

## Protocol

Run all eight steps in this fixed order, every time. Do not reorder, do not skip. Each step states its rule, its threshold, and its output format.

### Step 1 — RE-FREEZE (validity precondition)

Open `experience/episodes.jsonl` and select the row for the active `ep` (here `e0631`). Read `baseline_used` and `prereg_hash`.

Confirm three things, all of which must hold:

- **Baseline exists.** `baseline_used` is non-null and names a concrete comparison object (`human_curated_trend_post`), not "none" and not "vs zero".
- **Baseline was sealed before the outcome.** The episode `ts` (when ACT sealed it, `2026-06-28T13:40Z`) precedes the moment the realized outcome became visible. The baseline string must be byte-identical to what ACT wrote — no edit after sealing.
- **Prereg hash matches.** Recompute/verify `prereg_hash` (`7f2a9c`) against the frozen claim + baseline + metric. A mismatch means the contract was altered after sealing.

**If any check fails** (baseline missing, post-hoc, edited after ACT, or hash mismatch): do not invent a baseline and do not fall back to a vs-zero comparison. Append exactly one row to `experience/outcomes.jsonl`:

```
{"ep":"<ep>","resolved_ts":"<ts>","verdict":"OUTCOME:INVALID(no-prereg-baseline)","note":"baseline absent/post-hoc/edited; refusing vs-zero; escalated"}
```

Then **ESCALATE to a human** and STOP. Do not score predictions, do not write a lesson, do not write a reflection that claims an effect.

**If all checks pass:** record `RE-FREEZE: OK (baseline=human_curated_trend_post, prereg=7f2a9c, sealed 2026-06-28T13:40Z)` and proceed.

### Step 2 — MEASURE realized vs claimed vs baseline

Compute the **baseline-relative effect**:

```
effect_vs_baseline = realized − frozen_baseline      (NOT realized − 0)
```

Write all three values explicitly so the comparison is auditable: `claimed`, `realized`, `baseline`. In R0631:

- **claimed** — "the AI-agent spike is MANUFACTURED (fails the ≥N-origin gate), so a brief calling it organic would be wrong."
- **baseline** — the human trend post asserts the spike is organic ("blowing up").
- **realized** — provenance-dedup found 2 independent origins; the ≥3-origin gate fails; the spike is manufactured.

So the claim is **directionally right exactly where the baseline is wrong**: mine says manufactured, the human post implies organic, and the evidence sides with manufactured.

**Threshold for what "MEASURE" may output:** a directional read is *not* a beaten-baseline number. Unless a numeric effect with a confidence interval is available (e.g. a held-out metric like the schema probe's `link_validity@anchor`), label the result **DIRECTIONAL** and carry it forward as such. Do not convert "right vs wrong direction" into a delta. The numeric `delta +0.06` used later for the CONFIRM comes from the playbook's measured win-margin on independent clusters (`procedural/playbooks/hype-adjudication.md`: +0.06–0.08 vs human-curated trend posts, CI excludes 0), not from this single episode's direction.

Record: `MEASURE: claimed=manufactured · realized=2 origins<3 ⇒ manufactured · baseline=organic ⇒ effect=DIRECTIONAL-right (no per-episode numeric CI).`

### Step 3 — CLUSTER (collapse correlated episodes)

Read `experience/_ledger.jsonl`. **Independence is by origin/method/source family, not by date.** Two episodes that ran the *same move on the same source family* are one cluster and count once.

Build clusters for the lesson under test:

- `e0631` ran the hype-adjudication move (≥N-origin gate) on the agent-runtime topic.
- `e0625` ran the *same* hype-adjudication move on the agent-frameworks topic family.

These share method (`L-hype-adjud`) and source family (agent tooling), so for the lesson `L-hype-adjud` they would be one correlated body of evidence — but the ledger records them under distinct independent clusters (`agent-frameworks` for `e0625`, `agent-runtime` for `e0631`) because the runtime topic is a *separate independent origin family* from the frameworks topic. The rule that bites: **never count `e0631` as a second independent win in the same cluster as `e0625`.** Set `cluster_n` from the number of *distinct independent clusters*, not the number of episode rows.

Output: `CLUSTER: L-hype-adjud independent clusters = {agent-frameworks(e0625), agent-runtime(e0631)} ⇒ cluster_n=2 (distinct origin families).`

If two episodes had shared the *same* origin family, they would collapse to `cluster_n=1` and a lesson could not be promoted from them alone.

### Step 4 — SCORE DUE BETS

In `calibration/predictions.jsonl`, for every prediction with `due_date <= today` (2026-06-28), set four fields in place: `status` (`confirmed` | `refuted`), `outcome` (one-line note), `scored_ts`, and `brier`. Leave all `due_date > today` predictions completely untouched.

**Brier score** for a single binary bet: `brier = (p − o)²`, where `p` is the stated `confidence` and `o` is the realized outcome (1 if the predicted event happened, 0 if not).

Due bets in R0631:

- **P-114** — claim "≥3 independent origins", `confidence 0.65`, `due_date 2026-06-28`. Realized: deduped to 2 origins (one reposted seed), so the predicted event did **not** occur → `o=0`. `brier = (0.65 − 0)² = 0.4225 ≈ 0.42`. Stamp:
  - `status: refuted`
  - `outcome: "deduped to 2 origins (one reposted seed); see surprises S-009"`
  - `scored_ts: 2026-06-28T13:52Z`, `brier: 0.42`
- **P-118** — re-spike by `2026-07-12`. `due_date > today` → **leave open** (`status:open`, `outcome:null`, `brier:null`). This resolves at the scheduled `wk_025` recheck, not here.

**Boundary:** REFLECT stamps the per-prediction outcome the kernel routed to it. It does **not** recompute confidence bands, does **not** rewrite `calibration/recalibration_map.json`, and does **not** touch `calibration/calibration_report.md`. Those are Calibration's files and refit on the `wk_023` recalibrate schedule.

Output: `SCORE: P-114 refuted brier=0.42 · P-118 left OPEN (due 2026-07-12).`

### Step 5 — TRANSFER ABLATION (only if a transferred schema ran)

If the step ran a schema transferred from another domain, read `schemas/transfer_log.md` for the relevant transaction. Here the env `tech_builders` ran `SCH_geo_link_react_brief`, committed via `TRX_0007` (borrowed from `punjab_diaspora`).

The ablation rule: **drop ALL borrowed bindings and re-read the lift.**

- If the lift **survives** the ablation → label **ACQUIRED-here** (the competence is real in this domain, not just a borrowed prior).
- If the lift **collapses to baseline** → label **borrowed_prior** (the apparent win was carried by the source domain's bindings and does not generalize here).

`TRX_0007` already records the pre-registered ablation: dropping every punjab binding leaves lift `+0.19` (down from `+0.23` cold-start lift), CI from the probe `[0.06, 0.39]` excludes 0. The lift holds → **ACQUIRED-here**. Record that finding in the outcome row (`transfer_label: ACQUIRED-here`).

**Do not re-run the probe.** Generalization owns the probe; REFLECT only *classifies* the already-measured ablation. If `transfer_log.md` contains no ablation result for the schema that ran, do not fabricate one — set `transfer_label: UNTESTED` and flag it for the probe owner.

Output: `ABLATE: SCH-geo-link/TRX_0007 lift +0.19 post-ablation ⇒ ACQUIRED-here (no re-run).`

### Step 6 — SURPRISE → WHY-QUESTION

A **surprise** is an observation that violated a logged prediction. For each *refuted* bet from Step 4, check whether the violation is already queued in `attention/surprises.md`.

- `P-114`'s refutation already produced **S-009** ("the cross-platform wave deduped to only 2 origins, not the 4+ the surface implied"), which already spawned `Q-OQ-07`. **Do not double-log S-009.**
- Append the *follow-on* WHY-question that the refutation surfaces but does not answer: a new high-priority row asking **why one reposted seed read as four independent origins across platforms.** Format matches the existing log header (`SID | date | observation | violated-prediction | magnitude | → spawned`). Append only to `attention/surprises.md`; the kernel grants Learning exactly this one append for the surprise→WHY handoff.

**Never answer the question here.** If the question carries a false presupposition (e.g. it assumes 2 distinct origins exist when ownership is untraced — the live concern behind `Q-OQ-07`), flag it as `FALSE_PRESUP → rewrite, don't answer` and hand it off; Attention's `questions.md` is where it gets ranked and resolved, not here.

Output: `SURPRISE: S-009 already logged → appended follow-on WHY (reposted-seed-as-multi-origin), high priority.`

### Step 7 — DECIDE LESSON (lesson or NO-LESSON)

Write a candidate lesson **only** in the form:

```
TRIGGER → MOVE → EVIDENCE(with a CI)
```

and **only if** the baseline-relative effect's CI **excludes zero across independent clusters** (`cluster_n` from Step 3). Otherwise write **NO-LESSON** and state why.

Promotion thresholds, all required:

1. Effect is measured **vs the frozen baseline**, not vs zero.
2. The CI **excludes 0**.
3. The supporting evidence spans **≥2 independent clusters** (a single confirm never promotes).
4. The variance being explained is **epistemic (reducible)**, not aleatory. Aleatory noise is DO-NOT-CHASE — `B3` ("spike magnitude is noisy run-to-run") in `semantic/world/ai-agents.md` is explicitly marked irreducible; never distil a lesson that tries to optimize it away.

Applied to R0631:

- **`L-hype-adjud`** — already TRUSTED (playbook win 7/9, +0.06–0.08 vs human-curated baseline, CI excludes 0). This run adds an **independent** cluster (`agent-runtime`) with `delta +0.06`. Append a **CONFIRM** ledger row. It meets all four thresholds, so the cluster grows.
- **`L-23`** — "≥3 platforms agreeing ≠ ≥3 origins; dedup BEFORE counting." Born from this run's refutation. Seen **1×** (this is its first independent sighting). It fails threshold 3 (needs ≥2 independent clusters). Keep it as **CANDIDATE (seen 1×; needs 1 more independent confirm)**. **Do NOT promote it** to `procedural/lessons.md` — that is the consolidation sleep's job, and only after a second independent confirm.

If neither condition were met, the output would be `PROPOSED: NO-LESSON — effect directional only, CI not available across ≥2 independent clusters.`

Output: `LESSON: CONFIRM L-hype-adjud (cluster agent-runtime, Δ+0.06) · L-23 stays CANDIDATE (1×).`

### Step 8 — WRITE BACK (single-write-owner)

Perform all writes, in this order, then stop:

1. **`experience/outcomes.jsonl`** — append one resolved row keyed by `ep`. For R0631, because `P-118` is still open the *episode-level* outcome that gates the publishable G-12 success criterion remains OPEN until `wk_025`; record what is settled now and mark the episode resolution date. Shape:
   ```
   {"ep":"e0631","resolved_ts":"2026-06-28T14:01Z","verdict":"DIRECTIONAL-right vs baseline; episode resolves wk_025","claimed":"manufactured","realized":"2 origins<3 ⇒ manufactured","baseline":"human_curated_trend_post (organic)","effect_vs_baseline":"directional","cluster":"agent-runtime","cluster_n":2,"scored_predictions":["P-114:refuted:0.42"],"transfer_label":"ACQUIRED-here","note":"P-118 open to 2026-07-12; CONFIRM L-hype-adjud; L-23 candidate 1x"}
   ```
2. **`experience/reflections/e0631.md`** — write the six fixed lines:
   - `WHAT-HAPPENED`: surface looked like a 4-platform wave; provenance-dedup → 2 independent origins.
   - `WHAT-WORKED`: the hype-adjudication playbook caught it; the brief called it MANUFACTURED, not organic.
   - `WHAT-DIDNT`: pre-dedup prediction P-114 (≥3 origins) was REFUTED; over-trusted surface volume.
   - `WHY`: "many platforms agreeing" was one reposted seed, not many origins (common-method artifact).
   - `DID-IT-WORK?` (vs baseline, with cluster n): right where the human "blowing up" baseline is wrong; but this is one episode of a 2-cluster CONFIRM, not yet a per-episode beaten-baseline number.
   - `PROPOSED`: CONFIRM L-hype-adjud (cluster agent-runtime); candidate L-23 (dedup before counting) seen 1×, needs 1 more independent confirm.
3. **`experience/_ledger.jsonl`** — append the verdict row(s):
   ```
   {"lesson":"L-hype-adjud","ep":"e0631","result":"CONFIRM","delta":0.06,"cluster":"agent-runtime"}
   {"lesson":"L-23","ep":"e0631","result":"CANDIDATE","delta":null,"cluster":"agent-runtime","note":">=3 platforms agreeing != >=3 origins; dedup BEFORE counting (seen 1x; needs 1 more independent confirm)"}
   ```
4. **`calibration/predictions.jsonl`** — the in-place stamps from Step 4 (P-114 only).
5. **`attention/surprises.md`** — the follow-on WHY append from Step 6.

Leave `semantic/` and `procedural/` untouched — every lesson is a **proposal** the consolidation sleep applies, never an in-place edit by REFLECT.

## Decision rules & edge cases

- **Baseline missing / post-hoc / edited → `OUTCOME:INVALID(no-prereg-baseline)` + escalate.** Never substitute a vs-zero comparison and never reconstruct a "what the baseline probably was." This is the single hardest boundary; when in doubt, invalidate and escalate.
- **Prereg hash mismatch.** Treat exactly like a broken baseline: INVALID + escalate. The frozen `7f2a9c` must verify against the claim+baseline+metric.
- **`cluster_n == 1`.** A single independent cluster can never promote a lesson and can never gate a publishable output as a "beaten-baseline" claim. If the verdict would gate publication and `cluster_n == 1`, **stop and escalate** rather than auto-proceed.
- **CI includes zero (or no CI available).** Write `NO-LESSON` (or keep CANDIDATE) — never a confirmed lesson. A directional read with no numeric CI is `NO-LESSON` at the lesson step even if the direction is right.
- **Correlated episodes presented as independent.** Always re-derive independence by origin/method/source family from `_ledger.jsonl`, never by date. If two due episodes share a family, collapse before counting.
- **Prediction not yet due (`due_date > today`).** Read-only. Never stamp, never Brier, never close. `P-118` waits for `wk_025`.
- **Surprise already queued.** Check `attention/surprises.md` first; append only the *new* follow-on question, never a duplicate of `S-009`.
- **False-presupposition question.** Flag `FALSE_PRESUP → rewrite, don't answer` and hand to Attention (as `Q-016` already shows in `questions.md`); REFLECT never answers and never rewrites the question itself.
- **Aleatory variance.** If the apparent "miss" is irreducible run-to-run noise (`B3` DO-NOT-CHASE), do not distil a lesson against it and do not log it as a surprise that demands a WHY; note it and move on.
- **Transfer with no ablation result.** Do not fabricate or re-run; set `transfer_label: UNTESTED` and flag the probe owner (Generalization).
- **Budget / scope limit.** REFLECT scores only the routed due predictions and resolves only the active sealed episode. It does not sweep the whole prediction file for old unscored bets unless the kernel routed them; out-of-scope rows are left for their own due dates.
- **Conflict between reflection prose and the ledger.** The structured rows (`outcomes.jsonl`, `_ledger.jsonl`, the prediction stamps) are authoritative; the reflection `.md` is the human-readable mirror and must be made consistent with the rows, not the reverse.

## Worked example (run R0631 / goal G-12)

Environment `tech_builders` (transferred from `punjab_diaspora` via `SCH_geo_link_react_brief` / `TRX_0007`). Goal **G-12** — "ship a weekly hype-vs-organic brief whose hype call is later confirmed, beats the human-curated baseline over 6 weeks." ACT sealed episode **e0631** with the claim that the AI-agent spike is MANUFACTURED, baseline `human_curated_trend_post`, prereg `7f2a9c`.

1. **RE-FREEZE.** `episodes.jsonl[e0631]` shows `baseline_used: human_curated_trend_post` and `prereg_hash: 7f2a9c`, sealed `2026-06-28T13:40Z` before the dedup outcome was visible. Hash verifies. → `RE-FREEZE: OK`. (Had the baseline been blank, the row would read `OUTCOME:INVALID(no-prereg-baseline)` and the run would have escalated here.)

2. **MEASURE.** claimed = manufactured; realized = provenance-dedup yields **2 independent origins** (one reposted seed), failing the ≥3-origin gate → manufactured; baseline (human post) implies organic. The claim is **directionally right where the baseline is wrong**, but there is no per-episode numeric CI, so this stays **DIRECTIONAL**. This is consistent with belief **B2** in `semantic/world/ai-agents.md` ("public AI-agent hype is mostly MANUFACTURED"), which the upstream beats moved from .70 toward its current **.74** — but REFLECT does not edit B2; that confidence move is consolidation's, and REFLECT only proposes the CONFIRM that justifies it.

3. **CLUSTER.** `_ledger.jsonl` shows `L-hype-adjud` already CONFIRMED on `e0625` in cluster `agent-frameworks`. `e0631` is the *agent-runtime* family — a distinct independent origin family. So `L-hype-adjud` now spans **2 independent clusters** (`cluster_n=2`); `e0631` is **not** a second win inside `e0625`'s cluster.

4. **SCORE DUE BETS.** `P-114` ("≥3 independent origins", conf 0.65, due today) is **REFUTED** — the surface 4-platform wave deduped to 2 origins. `brier = (0.65−0)² = 0.42`. Stamp `status:refuted`, `outcome:"deduped to 2 origins (one reposted seed); see surprises S-009"`, `scored_ts:2026-06-28T13:52Z`, `brier:0.42`. `P-118` (re-spike by 2026-07-12) stays **OPEN** — left for `wk_025`. `recalibration_map.json` and `calibration_report.md` untouched.

5. **ABLATE.** The step ran `SCH_geo_link_react_brief`. `transfer_log.md / TRX_0007`: dropping *all* punjab bindings leaves lift **+0.19** (probe CI `[0.06, 0.39]` excludes 0). Lift holds → **ACQUIRED-here**, not borrowed prior. Recorded; probe **not** re-run.

6. **SURPRISE → WHY.** `P-114`'s refutation already produced **S-009** → `Q-OQ-07` in `surprises.md`/`questions.md`. Not double-logged. Append the follow-on high-priority WHY to `surprises.md`: *"why did one reposted seed read as 4 independent origins across platforms?"* Not answered here. Note that `Q-OQ-07` itself carries a presupposition under check ("2 distinct origins exist") — handed to Attention, not resolved by REFLECT.

7. **DECIDE LESSON.** `L-hype-adjud` meets all four thresholds (vs baseline, CI excludes 0 per the playbook's 7/9 record, ≥2 independent clusters, epistemic) → append **CONFIRM** with `delta 0.06`, cluster `agent-runtime`. `L-23` ("≥3 platforms agreeing ≠ ≥3 origins; dedup BEFORE counting") is seen **1×** → stays **CANDIDATE**, **not promoted**.

8. **WRITE BACK.** Append the resolved row to `outcomes.jsonl` (keyed `e0631`, episode-level outcome OPEN until `wk_025` because `P-118` is open); write `reflections/e0631.md` with the six lines (matching the seeded reflection); append the two `_ledger.jsonl` rows; stamp `P-114` in `predictions.jsonl`; append the follow-on WHY to `surprises.md`. `semantic/world/ai-agents.md` (B2 .74) and `procedural/lessons.md` are left untouched — the CONFIRM and the L-23 candidate are proposals the next consolidation pass applies.

Net state after REFLECT in R0631: one refuted bet scored, one open bet preserved, one transfer classified ACQUIRED-here, one lesson CONFIRMED into a 2-cluster body of evidence, one lesson held at candidate, one new WHY-question queued — and zero edits to any tier REFLECT does not own.

## Failure modes & escalation

- **Manufactured baseline.** The defining failure of this phase: filling in a baseline after the outcome, or silently comparing vs zero, to produce a "win." Prevention: Step 1 runs first and is fail-closed. Detection: any `effect_vs_baseline` computed without a verified `baseline_used` + matching `prereg_hash`. **Escalate** with `OUTCOME:INVALID(no-prereg-baseline)`.
- **Pseudo-replication / double-counting.** Counting `e0631` and `e0625` as two independent wins for the same cluster, inflating `cluster_n`. Prevention: Step 3 derives independence by origin/method family. If you cannot establish that two episodes are independent, treat them as one cluster.
- **Premature promotion.** Promoting `L-23` on a single confirm, or promoting any lesson whose CI includes zero. Prevention: Step 7's four thresholds; a candidate seen 1× stays candidate.
- **Chasing aleatory noise.** Distilling a lesson against `B3` run-to-run spike-magnitude noise. Prevention: check the variance type in `semantic/world/ai-agents.md`; DO-NOT-CHASE is hard.
- **Cross-faculty overwrite.** Editing `recalibration_map.json`, `calibration_report.md`, `attention/questions.md` beyond the surprise append, `semantic/*`, `procedural/*`, goals, or the kernel. Prevention: the single-write-owner contract below; any write outside it is a violation, not a judgment call.
- **Answering instead of handing off.** Resolving a surprise's WHY-question, or answering a false-presupposition question, inside REFLECT. Prevention: Step 6 appends only; Attention resolves.
- **Re-running the probe or re-planning.** REFLECT never re-runs ACT, never re-runs the transfer probe, never sets goal status, never touches the compliance gate.

**ESCALATE to a human (stop, do not auto-proceed) when:** (a) the prereg/baseline is broken, post-hoc, or edited; (b) the only available cluster is `cluster_n == 1` *and* the verdict would gate a publishable output (the G-12 brief); (c) a refutation implies the *playbook itself* is now wrong (the playbook's win-rate CI would collapse to include 0 — its own falsification condition); (d) two routed records conflict irreconcilably (e.g. the episode's frozen baseline disagrees with the transfer log's pre-registered metric). In all four, write what is known, mark it for review, and hand to a human rather than auto-resolving.

## Handoff

REFLECT runs as step 5 of the loop. Its consumer is the scheduled **CONSOLIDATE ("sleep")** pass — the daily light pass `wk_021` (due 2026-06-29T03:00Z) and the weekly deep pass `wk_022` (due 2026-07-05). Consolidation is what *applies* REFLECT's proposals:

- It reads the `_ledger.jsonl` CONFIRM/CANDIDATE rows and, only when a candidate reaches its second independent confirm, promotes it into `procedural/lessons.md` and updates `procedural/playbooks/hype-adjudication.md`.
- It reads the `outcomes.jsonl` rows and the CONFIRM for `L-hype-adjud` and may move belief **B2** in `semantic/world/ai-agents.md` (consolidation owns that confidence edit; REFLECT only justified it).
- It does **not** re-score predictions; that is REFLECT's stamp, already final for `P-114`.

The **state REFLECT must leave behind** for consolidation to act safely:

- `experience/outcomes.jsonl`: one resolved row per resolved episode, keyed by `ep`, with the transfer label and scored-prediction list. Episode `e0631` left with episode-level outcome OPEN until `wk_025` because `P-118` is open.
- `experience/_ledger.jsonl`: verdict rows whose `result` ∈ {CONFIRM, CANDIDATE, REFUTE} and whose `cluster` reflects *independent* origin families.
- `experience/reflections/<ep>.md`: the six-line human reflection, consistent with the rows.
- `calibration/predictions.jsonl`: every due bet stamped; every not-yet-due bet untouched and still `open`.
- `attention/surprises.md`: each refutation's surprise present exactly once, each spawning a WHY-question id for Attention to rank.

Separately, the open bet `P-118` is owned by the kernel schedule: `wk_024` (2026-07-05, re-run G-12) and `wk_025` (2026-07-12, resolve P-118). REFLECT leaves `P-118` open so those rechecks can score it; the `wk_023` recalibrate (2026-07-12) then refits the bands using the Brier scores REFLECT stamped — including `P-114`'s 0.42.

## Single-write-owner contract

**MAY write (and is the sole owner of):**

- `experience/outcomes.jsonl` — append resolved rows, keyed by `ep`.
- `experience/reflections/<ep>.md` — write the six-line reflection (e.g. `reflections/e0631.md`).
- `experience/_ledger.jsonl` — append verdict rows `{lesson, ep, result, delta, cluster, note}`.
- `calibration/predictions.jsonl` — stamp `status`/`outcome`/`scored_ts`/`brier` on **due** predictions only.
- `attention/surprises.md` — append surprise/WHY-question rows only (the one handoff the kernel grants Learning).

**MUST NEVER write or modify:**

- `semantic/*` (including `semantic/world/ai-agents.md` and belief B2) — proposals only; consolidation applies.
- `procedural/*` (including `procedural/lessons.md` and `procedural/playbooks/hype-adjudication.md`) — proposals only; no in-place promotion of L-23.
- `calibration/recalibration_map.json` and `calibration/calibration_report.md` — Calibration's files.
- `attention/questions.md` — Attention owns ranking/answering; REFLECT only appends to `surprises.md`.
- `goals/*`, `kernel/*` (cursor, schedule, checkpoint, turn_log, environments), the compliance gate, and the sealed episode/claim from ACT.

**Invariants:** never manufacture a baseline after the fact; never compare vs zero; never count correlated episodes twice; never promote a lesson on a single confirm or on a CI that includes zero; never chase aleatory noise (B3 DO-NOT-CHASE); never re-plan, re-run ACT, re-run the transfer probe, set goal status, or touch the gate. When the prereg is broken, `cluster_n == 1`, or the verdict would gate a publishable output, STOP and escalate.
