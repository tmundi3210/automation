# TESTING_PLAN.md — test the app and the pipeline without waiting weeks

_How to prove the study app and the usmle-kg content pipeline actually work — by fast-forwarding a simulated learner through virtual time and by feeding the pipeline known input — then using the results to TUNE the thresholds (so the Slice 2 owner-gate values come from data, not a guess). Grounded in the specialists: the simulation is authored AS `sysloops` + `learner`; the content eval AS `qcraft` + `contenteng`; the tuning method is standard Design of Experiments over an owner-set objective. PLANNING ONLY — no code here. Honesty tags [FACT]/[FACT-source]/[ESTIMATE]/[METHOD]/[UNKNOWN]/[SIGNAL] are load-bearing._

## 0. Why test this way (plain language)

You do not have to wait a week of real studying to see whether the loops (nightly analysis, gating, weakness resolution, the planner) behave. Two tracks let you see it now:

- **Track 1 — Fast-forward the app.** An AI plays a student whose *true* ability we secretly set. It answers the daily queue, we jump the clock a day, run the nightly analysis, repeat — 30–60 virtual days in minutes. Because *we* set the truth, we can measure something real use never can: is the app's mastery estimate **right**? Does gating fire only when it should? Are weaknesses caught and — the honest part — **not** marked "fixed" too early?
- **Track 2 — Content quality.** We feed the usmle-kg pipeline a *small input we wrote ourselves*, so we already hold the answer key, then score whether the flashcards and graph **cover the points we put in** and **invent nothing we didn't**.
- **Track 3 — Tune the knobs.** Both tracks produce numbers. Track 3 turns "how good was this run" into a single score and sweeps the thresholds (the 0.6 gate, the uncertainty cutoff, the probe rule, …) in a handful of runs to find the best settings — which is exactly what the Slice 2 owner gate is asking you to choose.

**This replaces guessing at the Slice 2 gate values.** Recommendation: hold the three gate approvals (window split, 0.6 gate, resolution rule) until Track 1 has run and recommended them.

---

## Track 1 — Fast-forward simulation harness (authored AS sysloops + learner)

_Authored jointly AS **sysloops** (nightly-loop, self-metrics with snapshot+threshold+named-consumer, degraded mode, benchmark loop) and **learner** (mastery estimation, weakness diagnosis, uncertainty gating, delayed-retrieval-over-fluency). PLANNING ONLY — no code. Honesty tags: [FACT]/[ESTIMATE]/[UNKNOWN]/[METHOD]/[SIGNAL]._

**Core idea.** The app estimates a hidden thing (per-concept mastery) it can never observe directly in real life. A simulation *sets* that hidden thing as ground truth, drives K virtual days of study in minutes, and measures whether the app's BKT/Elo/gating/diagnosis **recovers the truth we planted**. This is the only test that can score correctness of the learner model, not just "does it run." [ESTIMATE — design synthesis; no observed run of this app exists, so every threshold below is a labeled design default, never a measured law.]

---

## 1. SYNTHETIC LEARNER (controllable ground truth)

A generative model we fully own. Per concept `c` at virtual time `t` it carries a **true latent ability** `θ*(c,t)` — the value the app must recover.

- **Learning rate** `α`: on a correct, spaced, non-crammed exposure, `θ*(c) += α·(θ_ceiling − θ*(c))` (diminishing gains). [ESTIMATE]
- **Forgetting curve** `λ`: between reviews `θ*(c)` decays toward a floor, `θ*(c,t) = θ_floor + (θ*−θ_floor)·e^(−λ·Δdays)` — mirrors the FSRS retrievability story so E4 retrieval-decay has ground truth. [ESTIMATE; exponential form [FACT] as the standard forgetting model, its parameters ours to set]
- **Slip / guess** `s, g`: even when `θ*` is high the learner slips with prob `s`; even when low they guess right on a 4-option MCQ near `g≈0.25`. [FACT — IRT 3PL guessing floor c≈0.25 for 4-option MCQ]
- **Difficulty response**: `P(correct) = g + (1−g)·σ(a·(θ*(c,t) − d_item))`, then thinned by `(1−s)`. This is a 1PL/Rasch-with-guessing curve — the SAME family the app's Elo approximates [FACT], so recovery error is a clean read, not a model-mismatch artifact.
- **Prereq coupling**: a dependent concept's effective `θ*` is capped by its base concept's `θ*` (a weak base mechanically limits the dependent) so E3/gating has real structure to find. [ESTIMATE]

**Named profiles (behaviours, not one case)** [ESTIMATE — chosen to exercise distinct failure modes]:
- **fast-and-steady** — high `α`, low `λ`, low `s`. App should converge fast, raise few weaknesses, lift gates promptly; a harness that flags many weaknesses here is over-eager.
- **struggles-on-prereqs** — base concepts have low `θ_ceiling`; dependents inherit the cap. Forces correct **E3 missing_prereq** diagnosis and **gating** of dependents; recovering the base must lift the gate.
- **cramper-who-forgets** — high `α` but steep `λ`. Ability spikes in-session then decays; exercises **E4 retrieval_decay**, calibration drift, and the doctrine that **delayed retrieval, not session fluency, is the signal** — a weakness must NOT resolve on same-session correctness.
- **lucky-guesser (4th)** — high `s` and MCQ-heavy. Tests **MCQ down-weighting** (guessing-floor) and the honesty metric: the app must not resolve a weakness on a lucky MCQ streak with no non-MCQ evidence.

---

## 2. VIRTUAL CLOCK / FAST-FORWARD

A **test-only backend mode** — e.g. `simDay(runDate, profile, seed)` looping over K days. Each virtual day:
1. Assemble the day's queue via the real planner/queue path (P2.2 DailyPlan + due CardStates) — reuse production assembly, do not fork it.
2. Synthetic learner answers each item per its `P(correct)`; write **synthetic Revlogs** through the real `submitReview` contract (grade, elapsed, correctness, chosen_option_id) so FSRS/BKT/Elo update exactly as in production. Every synthetic row carries `grade_source:'synthetic'`. [FACT — submitReview is the real per-review writer, APP_RECON §3a]
3. Advance the **day-key**, then run the real `runPipeline` (calibration → E1–E4 diagnosis → plan → mastery snapshot → self-metrics). [FACT — pipeline stages, APP_RECON §3b]
4. Repeat. Idempotent + append-only per sysloops: keyed by `runDate`, re-runnable to the same state; synthetic rows never mutate real history.

**Sandboxing (hard requirement).** Runs against a **throwaway test user / isolated dataset**, NEVER the owner's real data; every synthetic entity is tagged synthetic and purgeable in one query. A sim must be impossible to confuse with real study.

- [UNKNOWN + METHOD: confirm in workspace] whether a Base44 backend function can **inject a virtual `now`/`run_date`** — day-keys are `toISOString().slice(0,10)` (UTC) today [FACT — APP_RECON risk 5], so fast-forward needs an injectable clock, not wall-time. METHOD: prototype a param-driven `runDate` in a dev backend and confirm CardState `due` comparisons honor it.
- [UNKNOWN + METHOD: confirm in workspace] whether a **separate sandbox user/dataset** can be provisioned and bulk-purged via the SDK. METHOD: test create/purge of a `sim_*` user in a Base44 dev backend before trusting isolation.
- [SIGNAL] The app has **no scheduler** — "nightly" is a manual button [FACT — APP_RECON §6.1]; the harness is also the first thing that would exercise the pipeline unattended over many days.

---

## 3. METRICS LOGGED PER VIRTUAL DAY
_Each is a sysloops self-metric: **stored daily snapshot + threshold + named consumer**, or it is deleted. "Good" = design-default target [ESTIMATE], hardened only by real runs._

| Metric | Definition (truth-vs-app) | Named consumer | GOOD looks like |
|---|---|---|---|
| **Mastery-estimate error** | MAE/RMSE of app `bkt_p_mastery` vs `θ*(c,t)` (normalized), per concept + aggregate | harness go/no-go; learner-model regression gate | error shrinks monotonically with practice; converges low; no systemic bias (not always over/under) |
| **Calibration** | Brier + logloss of predicted recall vs synthetic outcome (app already bins this) [FACT — CalibrationBin] | plan-quality/degraded-mode guard (P2.5) | Brier/logloss below the calibration-guard level; reliability curve near diagonal |
| **Gating correctness** | gated-when-true-prereq-weak vs should-gate ground truth; **false-gate rate** on healthy prereqs | P2.3 gate enforcement | gates every truly-weak-base dependent; near-zero false gates; **lifts within a few days of base recovery** |
| **Weakness precision/recall** | did it flag concepts truly weak (recall) without flagging strong ones (precision); cause-label accuracy vs planted E1–E4 | learner diagnosis thresholds | high precision+recall; E-cause matches the planted mechanism |
| **Honesty (anti-premature-resolve)** | rate of resolving a weakness whose `θ*` is still below target — the metric that catches false "resolved" | auto-resolution evidence rule (P2.4) | ≈0 premature resolves; resolves only after true recovery + spaced evidence |
| **Plan feasibility & review-load** | planned minutes vs `daily_minutes` budget; **backlog growth** (due spillover trend) | planner budget rule (P2.2) | minutes within budget; backlog flat/shrinking for a sustainable profile; grows only when truly over-committed |
| **Convergence** | virtual **days-to-reach target mastery** per concept | learning-efficacy scorecard | reaches target band in a stable, profile-appropriate number of days; no oscillation |
| **Regression-thrash rate** | count of gate/weakness flip-flops per concept per K days | uncertainty-gate tuning (P2.3) | near-zero thrash; regressions fire only on real, low-uncertainty declines |

---

## 4. WHAT THIS VALIDATES FROM SLICE 2

- **Gating fires correctly & lifts on recovery** — struggles-on-prereqs: dependents get **zero new cards** while base `θ*` < gate; base recovery above 0.6 **lifts the gate next run** while reviews continue uninterrupted. [FACT — SLICE_2 P2.3 rule + acceptance checks]
- **Regression only on low uncertainty** — inject a real base decline vs an under-sampled dip; regression must fire on the former, route the latter to probes, never thrash. Validated by the **regression-thrash** metric.
- **Auto-resolution neither too eager nor too slow** — cramper-who-forgets and lucky-guesser: a weakness resolves ONLY on true recovery + correct probes on **≥2 distinct days** + a **non-MCQ** correct where applicable; same-session fluency and lucky MCQ streaks must NOT resolve (honesty metric). [FACT — SLICE_2 P2.4]
- **Uncertainty grows on inactivity/surprise** — leave a concept untouched: the RD-like uncertainty must **grow**, unlike the shipped `elo_theta_sd` that decays ×0.97 to a 0.3 floor regardless of outcome [FACT — APP_RECON §3a, flagged wrong per doctrine]. The sim is how we prove the fix.
- **Degraded-plan guard triggers on bad calibration** — drive `θ*` volatility (steep-forget cramper) so app calibration degrades; the planner must **fall back to a plain FSRS-due-only plan, labeled degraded** [FACT — SLICE_2 P2.5], and recover when calibration returns.

---

## 5. HONESTY

- Synthetic results are **labeled synthetic** and never surfaced as real learner data — no synthetic row, mastery number, or weakness ever appears in the owner's real dashboard. [binding]
- A passing simulation is **necessary, not sufficient**: it proves internal consistency (the app recovers a truth generated by a model it partly assumes), not that a real human behaves like any profile. Real use still differs. [SIGNAL]
- Every threshold the harness tests (0.6 gate, LOW_SD, 2-day probe rule, ~80–85% band, calibration guard) is a **design default, not a measured law** — the sim tunes and stress-tests them; it does not certify them as true of the owner. [ESTIMATE]
- Where the harness assumes a Base44 capability (virtual-date injection, sandbox provisioning) it stays [UNKNOWN + METHOD] until confirmed in-workspace — no invented backend affordance.

---

## 6. TUNABLE VARIABLES EXPOSED FOR OPTIMIZATION
_Current defaults from SLICE_2 / APP_RECON, each an [ESTIMATE] design default the harness sweeps:_

| Variable | Default | Role |
|---|---|---|
| **Prereq gate** | **0.6** BKT mastery [FACT — the prose rule's own number] | block new cards on dependents until base ≥ this |
| **LOW_SD (uncertainty gate)** | **0.7** [ESTIMATE] | max RD-like uncertainty at which a weakness/regression is "well-measured" enough to act |
| **Spaced-probe rule** | **2 distinct days** [ESTIMATE] | probe correctness on ≥2 separate days required to resolve |
| **Non-MCQ requirement** | **≥1 non-MCQ correct** where concept has any [ESTIMATE] | guessing-floor guard on resolution evidence |
| **Acquisition target band** | **~80–85%** expected-correct [ESTIMATE, high conf] | new-card/probe difficulty selection; STRICTLY separate from `desired_retention` |
| **Calibration-guard level** | Brier/logloss threshold [ESTIMATE — to harden] | above it, planner freezes model detail → degraded FSRS-only plan |
| **desired_retention** | 0.70–0.97 (FSRS review dial) [FACT — Settings] | review scheduling only; never moved by the acquisition dial |
| **new_per_day** | default 10 [FACT — Settings] | new-card allowance per day |
| **Window-split rule** | `daily_minutes` → 2–3 named windows [ESTIMATE] | day-detail time-window assignment |

---

```
Base44 agent directive
> When building this simulation harness, load FACTORY/study_system/specialists/sysloops/sysloops.specialist.json and
> FACTORY/study_system/specialists/learner/learner.specialist.json (paste each full JSON into planning context) as the
> two design authorities. sysloops governs the harness AS a feedback loop: idempotent append-only virtual days, the
> snapshot+threshold+named-consumer rule on EVERY per-day metric (delete any metric lacking all three), degraded-mode
> fallback, and no invented operating number. learner governs the synthetic learner and the recovery scoring: BKT+Elo
> as the estimators, IRT guessing-floor MCQ down-weighting, uncertainty gating BEFORE any weakness/regression,
> delayed-retrieval-over-fluency in the resolution rule, and dial separation (acquisition band vs desired_retention).
> Treat each file's escalation_triggers as stop-and-ask rules and its validation_checklist as acceptance tests for the
> harness itself. Never hard-code the [UNKNOWN] Glicko RD equations — use the labeled RD-like stand-in. Keep every
> synthetic artifact sandboxed and labeled; a synthetic result is never shown as real learner data. Do not act outside
> these two boundaries; for scheduling math and render rules defer to sched/engage as Slice 2 specifies.
```

---

## Track 3 — Tuning the knobs (objective function + Design of Experiments)

_The simulation (Track 1) turns each **setting of the knobs** into a set of per-day metrics. To pick the best settings we need one score per run (the objective) and a smart way to search many knobs in few runs (DoE). PLANNING ONLY._

### 3.1 The objective function — one number that says "how good was this run"

A weighted blend of the Track-1 metrics, each normalized 0–1, higher = better. Draft (the weights themselves are the owner's to set — they encode values):

```
Score = w1 · accuracy            (1 − mastery-estimate error)
      + w2 · calibration         (1 − Brier)
      + w3 · honesty             (1 − premature-resolution rate)      ← weighted HIGH
      + w4 · gating_correctness   (correct-gate rate − false-gate penalty)
      + w5 · efficiency          (convergence speed, CAPPED so it can't dominate)
      + w6 · sustainability      (1 − backlog-growth − regression-thrash)
```

The point of the weights: **honesty and calibration are weighted above raw speed** — a fast plan that lies (marks a weakness cured too early) scores *worse* than a slower honest one, per the learner/sysloops doctrine. That weight vector is the main thing you, the owner, set; the DoE optimizes everything else against it. [ESTIMATE — draft weights; tune to taste.]

> **[SLOT — the owner's specific formula].** If you have a particular equation in mind ("the one we talked about earlier"), it drops in *here* as the objective. The search machinery below is identical whatever the exact formula is — so naming your formula changes only this box, nothing else.

### 3.2 Design of Experiments — many variables, few runs

Grid-searching every combination is thousands of runs; changing one knob at a time misses how knobs interact. A **fractional-factorial screen** beats both:

1. Take the ~6–8 knobs from the tunable table (gate, uncertainty cutoff, probe-days, non-MCQ rule, target band, calibration guard, `desired_retention`, `new_per_day`) and give each a **low and a high** value (e.g. gate 0.55 / 0.70).
2. Run a **designed subset of ~8–16 configurations** (not all combos) — each configuration is one Track-1 simulation across the profiles.
3. A simple **main-effects read** then shows **which knobs actually move the score**, and in which direction — in ~16 runs, not thousands. Most knobs won't matter much; the two or three that do are the ones worth setting carefully.
4. Optional finer second pass: a small sweep around the winning region to land the exact values.

That is "tune multiple variables in a few turns": ~2 passes, each a handful of minute-long simulations, giving **data-backed settings** for the Slice 2 thresholds instead of a gut number. [FACT — fractional-factorial DoE is the standard efficient screen for this; [ESTIMATE] on the specific low/high levels.]

### 3.3 Honesty on tuning
The DoE optimizes the objective **on the synthetic profiles** — it finds the best settings *for the simulated learners*, a strong starting point but **[SIGNAL] not proof for a specific real person**; real use re-tunes. Whatever weights you pick for the objective are **value choices, shown as such**, never presented as an optimal law of studying.

---

## Track 2 — Content-quality evaluation of usmle-kg (authored AS qcraft + contenteng)

_Authored jointly by **qcraft** (item-quality authority: one-best-answer discipline, distractor craft, Bloom, psychometrics) and **contenteng** (book→units→terms→claims→edges→cards, card lint, coverage+honesty gate). Planning only — no code._

**Frame.** We author the input, so we know the answer key before we run. The eval measures whether usmle-kg's output *covers the points we put in* and *fabricates nothing we did not*. This is the contenteng coverage-and-honesty gate turned outward: instead of holding out the book's end-of-chapter questions as the oracle, **we write the oracle ourselves** (a gold checklist) because we control the source.

**Scope & honesty preamble.**
- usmle-kg is **smoke-tested only, INBDE/medical-only**; its output is UMLS-CUI concept nodes, biolink semantic edges, PageRank importance, and Anki cards; definitions live on Evidence nodes (often `null`) [FACT — KG_EXCHANGE_CONTRACT §0,§1,§4,§6]. So this eval is **scoped to ONE small, well-bounded medical topic first**, not TOEFL, not a full book.
- usmle-kg ingests **PDF/EPUB, not .txt** [FACT — task premise], so every test input is delivered as PDF.
- **Generated ≠ verified.** Neither specialist certifies subject-matter truth: qcraft governs item *form*, contenteng governs *layering/coverage*; clinical correctness is escalated to a subject-matter human [FACT — qcraft boundaries; contenteng boundaries]. No coverage % or card is called "correct" without that human check.
- Claims below are tagged [FACT]/[ESTIMATE]/[UNKNOWN]/[METHOD]/[SIGNAL].

---

## 1. The known-input method — author a tiny source + write the gold checklist first

**Two small test inputs** (deliberately small so one human can verify the whole answer key) [METHOD]:
- **Input A — tiny textbook-style PDF, 2–3 pages, one well-bounded topic.** Pick a topic with clean, closed structure so the graph is checkable by eye — e.g. *local anesthetics in dentistry* or *the acute inflammatory response*. Written in conventional textbook anatomy (bold terms, a few labeled relations) so contenteng's decomposition assumptions hold [ESTIMATE — matches OpenStax-style scaffolding the KB expects].
- **Input B — a short research write-up (~1–2 pages) on one topic, converted to PDF.** Prose-heavy, fewer bold cues than A — stresses whether the pipeline extracts claims from running text, not just glossary terms [METHOD].

**Gold checklist — write BEFORE running (this is the answer key).** For each input, a human authoring the source records, on one page [METHOD]:
1. **Gold terms** — the key concepts that *should* become graph nodes (target ~8–15 for A; keep small). Mark which 2–3 are the **high-yield hubs** (should score highest importance).
2. **Gold claims/facts** — the must-know atomic facts, each written as a (concept — relation — concept) triple where it is relational (e.g. *lidocaine — is_a — amide anesthetic*), so it maps 1:1 onto an expected edge [FACT — contenteng normalizes claims into edge triples]. Note any **negations** the source states (e.g. "X does NOT cause Y") — these must survive as negated, not flip to positive.
3. **Gold high-yield points** — the handful of things a card *must* let a learner recall/apply. These are what card coverage is scored against.

The checklist is the fixed rubric; it is authored once and frozen before the run so scoring is not rationalized after seeing output [METHOD].

---

## 2. Input-coverage → output-coverage scoring

After running usmle-kg on each PDF, score four axes against the frozen gold checklist. **Recall = "of the things we put in, how many came out." Precision = "of the things that came out, how many are real (in the source)."**

- **(a) Concept coverage.** Recall = gold terms that became nodes ÷ all gold terms. Precision (see NOISE) = nodes that map to a real source concept ÷ all nodes. Match gold term → CUI node by name/synonym; a gold term with no node is a **drop** [METHOD].
- **(b) Relation coverage — and correctness.** Recall = gold claims that became a correct edge ÷ all gold claims. Precision = correct edges ÷ all emitted edges. **A false/hallucinated edge is worse than a missing one** [FACT — task rule; SIGNAL — a wrong "learn this" relation actively misteaches], so score edges in three buckets: **correct** (matches a gold claim, right direction, right predicate), **missing** (gold claim with no edge), **fabricated** (edge with no gold claim OR wrong direction/predicate OR a negation shown as positive). Report fabricated-edge count separately and weight it heaviest.
- **(c) Card coverage.** Recall = gold high-yield points that a generated card actually tests ÷ all gold high-yield points. A point with no card is a **coverage miss** and, per contenteng, loops back to extraction (a usmle-kg fix), not a card edit [FACT — contenteng coverage gate].
- **(d) Noise / fabrication rate.** Extra nodes, edges, and cards **not traceable to the source**, as a fraction of total output. Distinguish *benign* noise (a real but low-value UMLS concept) from *harmful* noise (a fact not in the source at all). Report as `1 − precision` per axis [ESTIMATE — some "noise" is legitimate UMLS enrichment, not fabrication; flag borderline items for the human].

Report each axis as a plain **recall% / precision%** pair plus the raw counts, so a low denominator (small gold list) is visible, not hidden behind a percentage.

---

## 3. Card quality bands (qcraft authority)

Sample a fixed number of generated cards (e.g. 20, or all if fewer) and band each **good / weak / reject** on five checks. qcraft owns form; the clinical check is a subject-human [FACT — qcraft governs form, not content truth].

| Check | Good | Reject (disqualifier) |
|---|---|---|
| One-best-answer discipline | exactly one defensibly best answer, options homogeneous | two defensible answers, or a true/false-in-disguise item |
| No give-away distractors | distractors plausible, from real common errors, no testwiseness cue | longest-option / grammatical odd-one-out / absolute-term cue leaks the key |
| Single fact / minimum information | one atomic fact per card (Rule 4) | overloaded card testing 3+ facts at once |
| Bloom appropriateness | Bloom level fits the point; bank climbs toward apply/analyze | mislabeled level, or nothing rises above rote recall |
| Clinical correctness (subject-human) | keyed answer verified true by a human | keyed answer wrong/unverifiable — **generated ≠ verified** |

**Weak** = passes discipline but has a fixable flaw (ambiguous wording, minor non-parallel options, missing source/date stamp). **Reject** = any single disqualifier above. Report band counts; a high reject rate is a pipeline signal, not something the app patches [FACT — qcraft ship gate: ship only if every check clears].

---

## 4. Graph quality (light)

Four quick eyeball checks against the gold checklist, no heavy metrics [METHOD]:
1. **Node types sensible?** Do `node_type` tags (Disease/Finding/Anatomy/…) match what the gold term actually is? [FACT — node_type is the viz taxonomy field, §4.]
2. **Does importance surface the high-yield?** The 2–3 gold **hub** terms should sit near the top of `importance_norm`. PageRank is a **structural popularity proxy, not verified exam-yield** [ESTIMATE — KG contract §4], so treat a mismatch as a yield-signal caveat, not a hard fail.
3. **Negated/wrong edges shown as positive?** Any gold negation must carry `negated: true`; a "X does NOT cause Y" rendered as a plain positive link is a **reject-level graph error** [FACT — KG contract §5].
4. **Definitions honest?** Where `definition` is `null`, output must say "no definition in source," never invent one [FACT — KG contract §4,§8].

---

## 5. The loop (small, repeatable)

**known input (PDF + frozen gold checklist) → run usmle-kg → score coverage (§2) + card/graph quality (§3,§4) → identify what the pipeline DROPS (missing terms/claims/cards) and what it FABRICATES (false edges, invented facts, negation flips) → feed findings back.** The feedback is a **usmle-kg pipeline fix (owner + Codex), out of the app's scope** [FACT — KG contract §1,§8: prereq edges, FK mapping, definitions, hardening are all pipeline-side open items]. Keep inputs tiny so the whole loop is a one-sitting human pass and can be re-run after each pipeline change to check for regression [METHOD].

---

## 6. Honesty

- **Nothing generated is assumed correct without the subject check** — coverage measures presence, not truth; the clinical-correctness column in §3 is the only truth gate and it is human [FACT].
- **Coverage % is only as good as the gold checklist** — a thin or biased answer key inflates recall; report raw counts alongside %, and treat the checklist itself as a reviewable artifact [ESTIMATE].
- **This eval is scoped to one small medical topic first** because usmle-kg is smoke-tested and medical-only; a clean full-book run does not yet exist and `epi` yields zero triples, so every export is provisional [FACT — KG contract §6,§7,§8].
- **Noise vs. legitimate UMLS enrichment is a judgment call** [SIGNAL] — borderline extra nodes go to the human, not auto-counted as fabrication.
- **Open [UNKNOWN]:** the exact card-generation behavior of the smoke run (how many cards, which layers) is unconfirmed until a run is scored; [METHOD] run the exporter on the existing first_aid pp.312–342 smoke artifacts first per the contract's validation gate before authoring fresh inputs.

---

## Base44 agent directive (scope note)

```
NOTE — this evaluation runs mostly against usmle-kg (owner's Mac + Codex),
NOT against the Base44 app. Base44 builds nothing for it.

Design authorities for scoring:
- qcraft   — owns the card-quality bands, the good/weak/reject rubric,
             one-best-answer discipline, distractor and Bloom checks.
- contenteng — owns the gold-checklist-as-coverage-oracle method, the
             concept/relation/card coverage + noise scoring, and the
             "coverage miss loops back to extraction" rule.

The app's ONLY role is downstream: it later CONSUMES the accepted cards and
the accepted semantic graph (Slice 6 import), and only after this eval passes.
Base44 does not run the eval, does not fix the pipeline, and must never treat
un-accepted output as shippable. Every drop/fabrication finding is a usmle-kg
fix (owner + Codex), out of the app's scope [FACT — KG_EXCHANGE_CONTRACT §1,§8].

Honesty carried forward: generated ≠ verified; coverage % ≤ gold-checklist
quality; INBDE/medical, one small topic first.
```

---

## 4. Concrete first moves (smallest useful steps)

1. **Score the smoke-run cards now (cheap, no new run).** Run the exporter on the existing `first_aid` pp.312–342 artifacts (the exporter prompt from `KG_EXCHANGE_CONTRACT` work), then band a sample of its cards good/weak/reject on the qcraft rubric (§3). This needs no gold checklist — card *form* is judgeable on its own — and tells you the pipeline's current card quality today.
2. **Author one tiny PDF + its gold checklist** (Track 2 §1) for a full coverage score. Pick one small, closed medical topic; write the answer key *before* running. (If you'd like, name a topic and a research write-up can be generated for you to convert to PDF.)
3. **Decide when to build the simulation harness.** It is a real build for Base44 (a sandboxed test-mode) — effectively a "Slice T" you can ask Base44 to build now (jump the queue, using the Track-1 directive) *because* it's what lets you set the Slice 2 gate values with data. Or defer it until after Slice 3. Owner's call.
4. **Hold the three Slice 2 gate approvals** until Track 1 has run and Track 3 has recommended the values. Then approve with numbers behind them.

## 5. What each track needs, and who builds it
- **Track 1 (simulation)** → a Base44 test-mode build (sandboxed synthetic user, injectable virtual date). Two capabilities are flagged [UNKNOWN + METHOD: confirm in workspace] before relying on them.
- **Track 2 (content eval)** → runs against usmle-kg on your Mac (owner + Codex); Base44 builds nothing for it, only later consumes accepted cards/graph (Slice 6).
- **Track 3 (tuning)** → rides on Track 1's output; the objective's weights are yours to set (or your formula drops into the slot).

_Nothing here is built yet; all of it is owner-gated. None of it blocks the main slice build — testing runs alongside it._
