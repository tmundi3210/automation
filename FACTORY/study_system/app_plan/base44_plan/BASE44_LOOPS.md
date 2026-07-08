# BASE44_LOOPS.md — the standalone loops doctrine (nightly Automation, managing agent, closed-loop fixes, SELF_LOOP audit)

**Place in the package:** this is the binding loops doctrine behind **Slice 8** of `BASE44_SLICES.md` (and the pipeline changes Slice 2 makes). Paste it into Base44's planning context whenever a slice touches the nightly pipeline, the managing agent, or any self-metric. Authored operating as the **sysloops** specialist.

Honesty posture, stated first per sysloops doctrine: **no observed run of this system exists** — every threshold, cadence, band and window below is a labeled design default [ESTIMATE], never a measured operating point. Competitor evidence is WebSearch-verified only [FACT — EXTERNAL_RECON method note]. App-behavior facts cite APP_RECON; vendor facts cite EXTERNAL_RECON. Nothing here is implementation — it is the contract the Base44 builder designs to.

---

## 1. Nightly analysis as a real scheduled Automation

Today "nightly" is a Dashboard button; no cron/scheduled config exists anywhere [FACT — APP_RECON §3b]. Base44 Automations can run backend functions on schedules or DB events [FACT-source: docs.base44.com backend-functions overview, via snippet — verify in-app]. This slice converts `runPipeline` into a scheduled Automation, keeping the button as a manual override that is safe to press at any time (idempotency below makes that free).

**Job contract.** One batch job per owner-local night over four inputs: the day's Revlog rows, late AI-graded voice answers (joined back to their original attempt by revlog id, never counted as new attempts), the concept graph, and yesterday's DailyPlan. Fixed cadence split: all steps nightly except the FSRS re-fit, which is weekly-to-monthly behind its gate.

**Timezone-correct day keys.** All day keys today are UTC `toISOString().slice(0,10)` while the calendar draws local days [FACT — APP_RECON risk 5]. Rule: add a `timezone` (IANA) field to Settings; every day key in Revlog aggregation, DailyPlan, Weakness dating, MasteryHistory and streak math is the owner-local calendar date; the Automation fires ~02:00 owner-local [ESTIMATE — after the study day closes]. Whether Base44 schedules support a timezone selector is [UNKNOWN] — [METHOD]: open workspace → Automations and check; fallback is a UTC schedule offset to land at 02:00 local.

**Idempotent, append-only.** Each run gets `run_id = (local day_key, attempt_n)`. All outputs are append-only dated snapshots tagged with run_id; readers take the latest run_id per day_key. The current delete-then-recreate pattern is abolished — it is what wipes same-day "resolved" marks [FACT — APP_RECON risk 8]. A crash mid-run re-runs to the same end state; a step that cannot be made idempotent is not shipped (sysloops boundary). The silent 500-row query caps in pipeline reads [FACT — APP_RECON risk 3] are replaced with full pagination in the same stroke — calibration over a truncated revlog is a false trend.

**Step order** (each step reads only reconciled inputs + prior snapshots; failure of a step skips only its dependents):
1. **Ingest/reconcile** — day's revlogs + late voice grades into one de-duplicated concept-tagged stream, identical on re-run.
2. **Retention calibration** — bin FSRS predicted-R (deciles) vs observed recall; Brier + log-loss (log-loss is the FSRS optimizer's own objective [FACT — sysloops KB]); append a dated CalibrationBin snapshot; act only on the multi-day trend, never one night's bins.
3. **Gated re-fit decision** — run/skip/request: re-fit only at ~1000+ total reviews AND ~2–4 weeks since last fit [ESTIMATE — labeled defaults]. Honest limit: no FSRS optimizer exists in this repo — it was delegated to an off-repo worker, and `Settings.w_optimized` is never written [FACT — APP_RECON §3d, risk 2]. Whether an optimizer can run inside a Base44 Deno function is [UNKNOWN] — [METHOD]: test the optimizer package in a throwaway backend function; until then this step emits a **re-fit request proposal** (owner-gated), and after any applied fit, reverts to prior `w` automatically if post-fit calibration degrades.
4. **Skill replay + degeneracy guard** — re-apply late-joined grades (by id) to BKT/Elo, down-weight MCQ corrects for the ~25% guessing floor; cap P(G) < ~0.3 and P(S) ≤ ~0.1–0.3, watch Elo θ/b divergence, freeze+revert a breaching fit. Fix the uncertainty direction: `elo_theta_sd` currently only decays ×0.97 toward a floor regardless of outcome, so stated uncertainty becomes overconfident [FACT — APP_RECON §3d]; replace with upward decay for concepts untouched > N days. The exact Glicko rating-deviation equations are [UNKNOWN] (egress-blocked source) — use a plainly-labeled linear proxy; never hard-code Glicko from memory (sysloops escalation trigger).
5. **Weakness diagnosis + confusion signals** — E1–E4 differential on well-measured weak concepts only (low θ OR low P(L), AND low uncertainty); persist `gradeTextAnswer.error_tags` onto Revlog first — today they are displayed and dropped, so free-text misconceptions never reach the diagnoser [FACT — APP_RECON risk 10]; high-confusion concept pairs route to adjacent-but-not-identical interleaving in step 7. Probes capped at ~3/day [ESTIMATE] so diagnosis never crowds out learning.
6. **Card-quality pass** — leeches, ItemElo outliers vs siblings, extreme predicted-vs-actual gaps → set `Card.quality_flag` (exists, never written today [FACT — APP_RECON §2]) and queue for rewrite; never lower mastery to explain a bad card.
7. **Tomorrow's plan** — due reviews + capped probes + new-material quota at p ≈ 0.85 (Elo-matched), interleaved, trimmed to `daily_minutes` by postponing mastered-easy first and never weak-concept or acquisition items; the ~85% acquisition dial stays separate from the `desired_retention` ~0.9 review dial.
8. **Mastery snapshots** — FK/CC aggregates appended to MasteryHistory (feeds the trend chart, its only data source [FACT — APP_RECON §3b]).
9. **Self-metrics + go/no-go** — scorecard below; declare the run valid or **degraded**.

**Degraded mode.** If ingest fails, guards breach, or the voice-grading backlog is stale/high: skip steps 4–6, ship a plain FSRS-due-only plan, mark the PipelineRun row `degraded` with the failed step named, and surface it on the Agent Activity page. A valid degraded plan always beats a rich broken one.

**Self-metrics: snapshot + threshold + named consumer, or delete.** CalibrationBin and SystemMetrics are computed today and consumed by nothing [FACT — APP_RECON §3d]. Every kept metric gets all three columns; a metric nothing consumes is deleted:

| Metric (daily snapshot) | Threshold [ESTIMATE defaults] | Named consumer |
|---|---|---|
| Calibration Brier/log-loss trend | worsening ≥7-day trend | step-3 re-fit gate + Dashboard health flag |
| Schedule adherence (done/planned) | < ~60% over rolling 7 days | plan-shrink rule (§3) — bad PLAN, not bad learner |
| Pipeline health (last-run age, step failures, ungraded voice count) | run age > 26h; backlog > ~20 | degraded-mode switch + Agent Activity flag |
| Skill-model sanity (P(G)/P(S) caps, Elo divergence) | any cap breached | step-4 freeze+revert |
| Challenge calibration (rolling 2-week success) | > ~92% coasting / < ~70% frustrated | challenge-band recalibration in step 7 |
| Diagnosis efficacy (θ/P(L) recovery within K ≈ 10 days of a fix) | a fix class chronically failing | E1–E4 threshold-adjust **proposal** (owner-gated) |
| Kill list | — | `SystemMetrics.llm_cost_usd`/`stt_cost_usd` are never set [FACT — APP_RECON §2]: wire to real cost sources via the Slice 5 model picker or delete the fields |

**Owner approval gate (this part):** the owner enables the schedule in the Base44 workspace themself and approves the 02:00-local run time; the manual button stays until two clean scheduled weeks pass.

```
Base44 agent directive
> Load FACTORY/study_system/specialists/sysloops/sysloops.specialist.json (paste the full JSON into your planning context). Design strictly by its role + decision_procedure; treat its escalation_triggers as stop-and-ask rules and its validation_checklist as this slice's acceptance test. Do not act outside its boundaries.
```

---

## 2. The managing agent

Runtime: a Base44 agent defined as a JSONC config in `base44/agents/`, synced via the Base44 CLI [FACT-source: docs.base44.com AI-agents pages, via snippet — verify in-app]. Agent runtime limits, allowed triggers and model/cost controls are [UNKNOWN] — [METHOD]: build a one-step throwaway automation/agent in the workspace first and observe (FEATURE_PLAN P8). Five duties, from sysloops doctrine:

1. **Nightly steward** — ensures the §1 Automation ran; retries once on failure; enters degraded mode; writes an append-only AgentActionJournal row for every action it takes.
2. **Diagnostician** — reads the scorecard each morning; on a threshold breach, packages the evidence (metric, snapshot trend, threshold) into a flag or proposal; never acts on a single-day snapshot (trend-not-point rule).
3. **Router** — routes each anomaly/question to the specialist whose boundaries own it, using the specialists' own escalation triggers as the routing table. All 14 specialist JSONs exist on disk under `FACTORY/study_system/specialists/` [FACT — directory listing 2026-07-08]: scheduling/due/retention → sched; mastery/BKT/E1–E4 thresholds → learner; item wording → qcraft; decomposition/ingest → contenteng; streak/day-box semantics → engage; FK/CC content → dental; TOEFL → toefl; voice grading → voice; integrations/auth/scheduling infra/LLM-ops → extint; graph schema → kgraph; graph drawing → viz; media value → media; guide wording → uxguide; loop design → sysloops. Where a sysloops escalation trigger says "defer to owning specialist," the agent loads that specialist's JSON and re-asks there.
4. **Bounded spec auditor** — runs the SELF_LOOP protocol per specialist on a cadence (~monthly, one specialist per run [ESTIMATE]): four **fresh context windows** — R1 questioner (adversary, does not answer) → R2 answerer (IS the specialist; answers only from its spec/KBs with [FACT]/[ESTIMATE]/[UNKNOWN]; gaps stay gaps) → R3 judge (classifies gap|contradiction|stale|overclaim|untagged|scope|ok, disposition apply_now|propose|watch) → dry-stop or one more round, **max 2**; still-hot at cap = flagged **NOT-CONVERGED**, never silently truncated → R4 synthesizer (LOOP_REPORT + feedback). Honest scope limit: inside the app the auditor produces **proposals only** — the finalizer that applies `apply_now` items and the independent re-gate live in the repo toolchain, not the app, and agent self-reports are never acceptance [FACT — SELF_LOOP invariants 3–4].
5. **Benchmark scanner** — re-runs the competitor scan quarterly [ESTIMATE] plus event triggers: a competitor flagship/price change, a new entrant, or one of the documented faults surfacing in our OWN self-metrics (the loop-that-improves-the-loop path). Each finding lands as a BenchmarkLedger row: function → exemplar → what-to-copy + modification → evidence tag → verdict adopt/adapt/reject. Honesty: feature existence [FACT] from vendor docs only, prices [ESTIMATE]/[UNKNOWN], vendor claims marked as such, no user count/revenue/pass rate ever asserted. Seed rows from EXTERNAL_RECON §8 (e.g. Anki desired-retention dial = adopt, already in app; Duolingo streak-freeze = adapt, bound to completed due reviews — the app's `streak_slack_tokens` already does this [FACT — APP_RECON §4b]; UWorld peer percentile = reject, single-user system has no peers).

**Autonomous (mechanical, journaled) vs owner-gated:**

| Agent may do alone | Needs the owner gate |
|---|---|
| trigger/retry the nightly run; enter degraded mode | first enablement of the schedule; go-live of each duty (two-week shadow, steward first, auditor last [FEATURE_PLAN P8 gate]) |
| append journal rows, flags, trend reports, proposals | applying ANY threshold/parameter change (incl. E1–E4, challenge band, FSRS `w`) |
| dedupe/upsert weakness rows per §3 rules | any spec/KB file change (repo finalizer + independent re-gate) |
| revert `w` to prior on post-fit calibration degradation (safety default, journaled + flagged) | adopting a BenchmarkLedger verdict into the build backlog |
| read-only benchmark research within its budget | any external send (WhatsApp/email/calendar event), any new credential/Secret, any spend above the owner-set LLM budget cap, any export of user data off-app |

```
Base44 agent directive
> Load FACTORY/study_system/specialists/sysloops/sysloops.specialist.json AND FACTORY/SELF_LOOP.md (paste both in full into your planning context). Design the agent's duties by the specialist's role + decision_procedure; implement the audit duty exactly per SELF_LOOP's fresh-context roles, bounded rounds and NOT-CONVERGED honesty; treat sysloops escalation_triggers as stop-and-ask rules and its validation_checklist as acceptance. The agent proposes; only the owner gate applies.
```

---

## 3. The closed-loop fixes

**Weakness lifecycle** (defects: duplicates accumulate across days; same-day reruns wipe resolved marks; `probing`/`probe_card_ids` write-only; prereq blocking is prose [FACT — APP_RECON risk 8, §4a]):
- **Dedupe:** exactly one open row per (concept, cause), upserted across days with `first_seen`/`last_seen`; delete-recreate abolished.
- **Probe follow-through:** `probe_card_ids` are actually enqueued into the next plan (within the ~3/day cap); status transitions open → probing when scheduled — the enum finally earns its keep.
- **Evidence-based auto-resolve** replaces Mark-Resolved: a weakness closes when concept mastery is back above threshold AND its probe cards were answered correctly on ≥2 distinct local days [ESTIMATE — trend-not-snapshot rule; harden with real runs]. Resolution is an appended event row, so reruns cannot wipe it. The button becomes a snooze ("dismiss until date") that hides but cannot fake mastery.

**BKT/Elo/calibration finally consumed** (all are display-only or unread today [FACT — APP_RECON §3d, risk 9]) — each signal with its named consumer:
- `bkt_p_mastery` → the new-card **gate** in queue assembly (weak concept gets no NEW cards; reviews never blocked, per the non-punitive contract) and the weakness flag rule.
- `elo_theta` + `ItemElo.elo_b` → acquisition selection at p ≈ 0.85 and Elo-matched probe picking in plan build (step 7).
- `elo_theta_sd` (fixed to decay upward) → the well-measured-weakness rule: no weakness is flagged on an under-sampled concept — it gets probes instead.
- CalibrationBin trend → the re-fit gate (step 3) and the Dashboard/Agent health flag.
- `error_tags` (persisted onto Revlog per §1 step 5) → the misconception evidence stream feeding E2 diagnosis and interleaving (E-letter binding assumed enum order [ESTIMATE] until the Slice 1 code read confirms it).

**Plan-regression rule**, three levels: (a) concept level — the new-card gate re-applies automatically when a base concept's mastery falls back below threshold (regression is allowed, matching Slice 2's gating); (b) plan level — chronic adherence < ~60% over ≥7 days [ESTIMATE] triggers a shrink proposal (`planned_units`/`new_per_day` down): a chronically undone plan is a bad PLAN to shrink, never a bad learner; (c) model level — post-re-fit calibration degradation auto-reverts `w` to prior (journaled, flagged).

```
Base44 agent directive
> Load FACTORY/study_system/specialists/sysloops/sysloops.specialist.json (paste the full JSON into your planning context). Design strictly by its role + decision_procedure — especially the dominance rules (card-fault over learner-weakness, budget over probes, degraded over broken); treat its escalation_triggers as stop-and-ask rules and its validation_checklist as this slice's acceptance test. Do not act outside its boundaries.
```

---

## 4. Slice 8 build content + loop acceptance checks

**Entities:** `AgentActionJournal` (append-only: timestamp, duty, action, evidence ref); `Proposal` (source duty, evidence, status pending/approved/rejected, owner decision date); `BenchmarkLedger` (four columns + verdict + evidence tag); `WeaknessEvent` (append-only resolution/snooze/probe events); PipelineRun extended with run_id, local day_key, per-step status, degraded flag. **Screens:** one "Agent Activity" page — what ran, what it found, what awaits approval, with approve/reject on proposals; the manual pipeline button kept as override. **Behaviors:** the §1 schedule; a shadow-mode flag (agent proposes only); an owner-set LLM budget cap consuming Slice 5's cost visibility.

**Loop acceptance checks** (this section's gate; derived from the sysloops validation_checklist):
1. Pipeline runs on schedule with no button press; dated PipelineRun rows carry owner-local day keys.
2. Running the pipeline twice in one night yields the identical end state: no duplicate Weakness/MasteryHistory rows; resolved/snoozed marks intact.
3. An evening review by a west-of-UTC user lands on today's day box, not tomorrow's.
4. Every kept self-metric shows snapshot + threshold + consumer on the Agent Activity page; `llm_cost_usd`/`stt_cost_usd` are wired or gone.
5. A forced step failure produces a degraded run that still ships an FSRS-due-only plan, flagged with the failed step named.
6. The same (concept, cause) missed on 3 days = one open Weakness with updated `last_seen`.
7. Probe cards appear in the next plan (≤ cap); a weakness moves open → probing → resolved on the 2-distinct-day evidence; Mark-Resolved is gone, snooze exists.
8. A weak concept receives no new cards; the gate re-applies on regression; due reviews are never blocked.
9. Below-gate nights log "re-fit skipped (gate)"; a simulated post-fit calibration degradation auto-reverts `w`.
10. In shadow mode the agent writes nothing outside journal + proposals; every agent write has a journal row.
11. An audit run yields a LOOP_REPORT-style proposal with round count and, where applicable, an explicit NOT-CONVERGED flag; nothing self-applies.
12. Every BenchmarkLedger row carries all four columns + verdict + evidence tag; no unlabeled price/user-count anywhere.
13. Pipeline reads paginate fully — verified at 2000+ cards (no 500-row truncation).

**Owner approval gates (this part):** schedule enablement; two-week shadow then per-duty go-live; budget cap; every proposal application; any external send/credential/export as tabled in §2.

```
Base44 agent directive
> Load FACTORY/study_system/specialists/sysloops/sysloops.specialist.json AND FACTORY/SELF_LOOP.md (paste both in full into your planning context). Build slice 8 strictly by the specialist's role + decision_procedure and SELF_LOOP's invariants (fresh contexts, bounded rounds, apply_now-only-with-independent-re-gate, self-reports are never acceptance). Treat the escalation_triggers as stop-and-ask rules and the acceptance checks above plus the validation_checklist as this slice's gate. Do not act outside the specialist's boundaries.
```
