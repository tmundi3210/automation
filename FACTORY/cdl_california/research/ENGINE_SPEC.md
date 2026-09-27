# Learning-engine spec (FSRS-6, BKT, diagnosis, planner, readiness, gamification invariants)

_Research sweep 2026-09-27 (read-only agents; web access was search-index only — dmv.ca.gov, youtube.com, app stores blocked from the build container). Tags: [FACT src] / [ESTIMATE basis] / [UNKNOWN]. Source agent: `pedagogy-engine`. Kept verbatim for provenance; corrections from the plan review are in `../BUILD_PLAN.md` and `REVIEW_FINDINGS.md`._

# CA CDL Learning Engine: Spec Pulled from Existing Repo Research

All repo paths are relative to `/home/user/automation/FACTORY/study_system/`. `ARC:` means a member of the uploaded zip archive.

## 0. Differences between CA CDL and the repo's prior work (read first)

| # | Difference | Impact |
|---|---|---|
| D1 | Every CA CDL test item has **3 options** [FACT ARC:00-START-HERE.md:18, GK-01:197]. The repo assumed 4-option items with a guessing floor c≈0.25 (`research/learner_model_and_adaptivity.md:49`, `FIX_SPEC_BKT_EVIDENCE.md:15`). | For 3-option MCQ, guess probability c = P(G) = **1/3**. For True/False, c = 0.5 [by definition]. |
| D2 | The repo's BKT guard caps a fitted P(G) below 0.3 (`learner_model...:23`). A 3-option chance level of 0.333 already breaks that cap. | The cap applies only to *fitted* parameters. Treat format guess rates as structural constants and control inflation by tempering the update (§2). |
| D3 | The authoring rule says avoid NOT/EXCEPT stems (`research/question_types_and_answering.md:148`). The DMV test "often flips the wording… NOT… EXCEPT… True or False" [FACT ARC:00-START-HERE.md:26]. | Teach every fact in positive form. The practice bank must still include a flipped-stem variant for each fact. Trap-wording becomes its own diagnosis cause. |
| D4 | The sched specialist forbids any pass probability (`app_plan/base44_plan/handoff/specialists/sched.specialist.json` validation_checklist[8]). The user wants a readiness score. | Show it as an [ESTIMATE] calibrated against mock exams. The hard "Ready" gate is fresh mock scores (§5). |
| D5 | The app's planner reserves 14 days before the exam as "last day to learn" (`app_plan/APP_ANALYSIS.md:114`). The CA pack's syllabus is **8 days** [ARC:00-START-HERE.md:31-50]. | Scale the cutoff to the time available (§4). |
| D6 | The answer key follows the handbook. There is a published "handbook vs elsewhere" table (90 days vs 180, 500 ft vs 300, 9.84 in vs 10¾, 20–45 psi vs 25–40, …) [ARC:00-START-HERE.md:61-73]. | Add a new error cause: **wrong-source** (§3). |
| D7 | Failure rates quoted in the pack: 39.5% fail General Knowledge (GK) on the first try, 43.9% fail Combination; "most misses are exact numbers and trap wording" [ARC:00-START-HERE.md:27, citing "a DMV study"]. | Number-confusion and trap-wording are the causes worth the most. |
| D8 | Test format: 3 tries per test, touchscreen, immediate wrong-answer feedback, a skip option, no time limit [ARC:00-START-HERE.md:21-25]. | Build a mock-exam mode that matches this UI. Never use timers in scored practice. |

## 1. FSRS-6 (checked against the web)

**Source:** py-fsrs `fsrs/scheduler.py` (fetched), the awesome-fsrs wiki "The-Algorithm" (fetched), and the repo copy `research/spaced_repetition_and_anki.md:82-91`. All three agree.

**Default weights w0..w20** [FACT, identical in all three sources]:
`[0.212, 1.2931, 2.3065, 8.2956, 6.4133, 0.8334, 3.0194, 0.001, 1.8722, 0.1666, 0.796, 1.4835, 0.0614, 0.2629, 1.6483, 0.6014, 1.8729, 0.5425, 0.0912, 0.0658, 0.1542]`

**Parameter bounds** [FACT py-fsrs]:
- Lower: `(0.001×4, 1.0, 0.001, 0.001, 0.001, 0.0, 0.0, 0.001, 0.001, 0.001, 0.001, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.1)`
- Upper: `(INIT_S_MAX×4, 10, 4, 4, 0.75, 4.5, 0.8, 3.5, 5, 0.25, 0.9, 4, 1, 6, 2, 2, 0.8, 0.8)`
- `STABILITY_MIN = 0.001`; difficulty D is clamped to [1, 10].

**Scheduler defaults** [FACT py-fsrs]: desired_retention 0.9, learning steps (1 min, 10 min), relearning step (10 min), maximum_interval 36500, fuzzing on.

**Formulas** (G is the grade: 1 = Again, 2 = Hard, 3 = Good, 4 = Easy):

| Quantity | Formula |
|---|---|
| Curve constant | `factor = 0.9^(−1/w20) − 1` (≈0.98035 at the defaults) |
| Retrievability (chance of recall) after t days | `R(t,S) = (1 + factor·t/S)^(−w20)` |
| Interval for target retention r | `I(r,S) = round((S/factor)·(r^(−1/w20) − 1))`, clamped to [1, max]. `I(0.9,S) = S` |
| Initial stability | `S0 = w[G−1]`: 0.212 / 1.2931 / 2.3065 / 8.2956 days |
| Initial difficulty | `D0(G) = w4 − e^{w5(G−1)} + 1`, clamped: 6.41 / 5.11 / 2.12 / 1 (unclamped Easy = −4.77) |
| Next difficulty | `ΔD = −w6(G−3)`; `D' = D + ΔD·(10−D)/9`; `D'' = w7·D0(4)_unclamped + (1−w7)·D'`; clamp to [1, 10] |
| Stability after a correct answer (G ≥ 2) | `S' = S·(1 + e^{w8}·(11−D)·S^{−w9}·(e^{w10(1−R)} − 1)·[w15 if G=2]·[w16 if G=4])` |
| Stability after a lapse (G = 1) | `S' = min( w11·D^{−w12}·((S+1)^{w13} − 1)·e^{w14(1−R)} , S / e^{w17·w18} )` |
| Same-day review (<1 day since last review) | `S' = S·e^{w17(G−3+w18)}·S^{−w19}`; for G ≥ 2 the growth factor is `max(…, 1)` |
| Elapsed days | `max(0, whole days since last review)` |

- **Fuzz** [FACT py-fsrs]: added noise of 15% for intervals in [2.5, 7) days, 10% in [7, 20), 5% for 20+.
- **Worked check with defaults:** a first answer of Good gives S = 2.31 days. A review at R = 0.9 then graded Good gives S = 11.9. A lapse at that point gives S = 0.62 [computed].
- **JS implementation:** `ts-fsrs` 5.4.2, MIT license [FACT registry.npmjs.org/ts-fsrs/latest]. The FSRS-6 move happened in ts-fsrs 5.x; loading an old 19-weight vector pads w19 = 0 and w20 = 0.5 [FACT via search snippet]. Use it client-side.
- **Review log:** log every review from day one as `(ts_ms, item_id, grade 1-4, interval, last_interval, duration_ms ≤ 60000, type 0 learn / 1 review / 2 relearn / 3 cram)` (`research/spaced_repetition_and_anki.md:129,167`).
- **Per-user tuning:** refit the weights only after roughly 1000 reviews (`research/learner_model_and_adaptivity.md:137`). A CDL learner will usually never get there, so assume the defaults for the whole course [ESTIMATE].

**Grade mapping for MCQ** [ESTIMATE; FSRS gives no MCQ rule]:

Inputs are correct/incorrect, confidence (Sure / Unsure / Guess, or an "I don't know" button) and response time normalised per word, `rt_n = rt_ms / words(stem + options)`, compared with the learner's rolling median `m`.

| Outcome | FSRS grade |
|---|---|
| Wrong, or "I don't know" | 1 (Again) |
| Correct + Guess | 1. No retrieval happened, and chance is 33%. Log `guess_correct = true`. |
| Correct + Unsure, or rt_n > 2m | 2 (Hard) |
| Correct + Sure, 0.5m ≤ rt_n ≤ 2m | 3 (Good) |
| Correct + Sure, rt_n < 0.5m, previous grade ≥ 3 | 4 (Easy), **only for typed or numeric recall**. MCQ tops out at Good, because 3-option recognition inflates the signal. |

Additional rules for this mapping:
- **High-confidence wrong answer:** grade 1 and set `hypercorrect = true`. Show strong feedback, then schedule a retest at about 7 days. High-confidence errors are corrected more readily after feedback (Butterfield & Metcalfe 2001, G = .36 [FACT via search snippet]), but they tend to come back after a week (Butler et al., Psychon Bull Rev, "high-confidence errors return" [FACT title via search]).
- **Rating-honesty check:** flag cases where Easy came with a long latency or a later lapse (`research/learner_model_and_adaptivity.md:126`).
- **Response time:** used only for grading. It must never become a timer in scored practice (D8, and `app_plan/base44_plan/handoff/GAMIFICATION_PLAN.md:77` INV-8).
- **Lesson-embedded checks:** questions asked right after teaching are logged as type 0 (learn) and fall under the same-day formula.

## 2. BKT per skill, and Elo

**BKT (Bayesian Knowledge Tracing)** (`research/learner_model_and_adaptivity.md:15-23`, `FIX_SPEC_BKT_EVIDENCE.md:13-30`):

| Parameter | Value | Tag |
|---|---|---|
| P(L0), prior mastery | 0.3 (app default, `GAMIFICATION_PLAN.md:163`). Better: set it from an adaptive placement quiz, since car-license holders already know some GK. | ESTIMATE |
| P(T), learning rate | ≤ 0.15 (`FIX_SPEC:20`) | ESTIMATE |
| P(S), slip | 0.05–0.10; up to 0.15 for LLM-graded free text (`FIX_SPEC:23-25`) | ESTIMATE |
| P(G), guess, by format | 3-option MCQ 1/3; NOT/EXCEPT 3-option 1/3; T/F 0.5; typed number or term 0.05 (`FIX_SPEC:16`) | FACT (arithmetic) / ESTIMATE (typed) |
| Evidence weight w_fmt | MCQ 0.3; T/F 0.2; typed 1.0 (`FIX_SPEC:18`) | ESTIMATE |

Update steps:
1. Likelihood ratio: `LR_correct = (1−S)/G`; `LR_wrong = S/(1−G)`.
2. Tempered update in odds space: `odds' = odds · LR^{w_fmt·e}`.
3. Easy-item damping applies to correct answers only: `e = clamp((1−p̂)/0.15, 0.25, 1)`, where p̂ is Elo's predicted chance of a correct answer. Misses are never damped (`FIX_SPEC:29`).
4. Learning transition: `P(L) ← P(L|obs) + (1 − P(L|obs))·P(T)`.

Worked check [computed]: at P(L) = 0.2, one correct 3-option MCQ moves it to about 0.25. That meets the acceptance limit of Δ ≤ +0.07 (`FIX_SPEC:44`).

**Elo** (`research/learner_model_and_adaptivity.md:36-38`, adjusted with the 3PL guessing floor from `:46`):
- Predicted correctness: `p = c + (1−c)·σ(θ − d)`, with c = 1/3 for 3-option items.
- Updates: `θ += K·w_fmt·(y − p)` and `d −= K·(y − p)`.
- Step size: K = 0.4 [FACT, literature value]. The app currently uses 0.3 (`FIX_SPEC:8`). Decay it as `K = 0.4/(1 + 0.05·n)` [ESTIMATE].
- Seed d higher for number and trap items (D7) [ESTIMATE].

**Uncertainty:** use a labelled Glicko-like stand-in. `sd² = sd0²/(1 + n_eff) + σ_idle²·days_idle`, so uncertainty grows when a skill goes unpractised. The current app bug decays it ×0.97 no matter what happens (`TESTING_PLAN.md:80`). Glicko's exact equations are [UNKNOWN] in the repo; do not hard-code them (`research/learner_model_and_adaptivity.md:41`).

**Skill thresholds:**
- Weakness: P(L) < 0.5.
- Prerequisite unblock / "on track": ≥ 0.6 (`TESTING_PLAN.md:99`).
- "Learned": ≥ 0.95 (`GAMIFICATION_PLAN.md:166`).
- Target success rate on new material: 80–85%, kept separate from desired_retention (`research/learner_model_and_adaptivity.md:106-107`).

## 3. Weakness diagnosis (runs client-side)

**Logged per attempt:** `{item, concepts[], format, stem_polarity (pos | neg | TF), numeric_key, chosen_opt, opt_tag, correct, conf, rt_ms, R_at_attempt, P(L), sd}`.

**Distractor tags** are required on every wrong option: `neighbor_number | round_number | sibling_rule_number | alt_source | trap_named | misconception:<id> | plausible_generic`. The source pack already requires wrong options to be "neighbouring numbers, or the common mistake named in Exam traps — never invented" [ARC:00-START-HERE.md:151]. Deriving the tags is therefore mechanical.

**Uncertainty gate first:** if `sd > LOW_SD` (0.7, `TESTING_PLAN.md:100`) or the concept has fewer than 3 attempts, send it to a probe (2–3 varied items). Do not label a cause yet. Confidence must be computed from the evidence count. The app's current hard-coded 0.4 is misleading (`app_plan/APP_ANALYSIS.md:92`).

**Rules, evaluated in order.** The first cause that fires wins; every fired rule is stored as evidence. The base order follows `research/learner_model_and_adaptivity.md:75-85` with three CDL causes added.

| Code | Cause | Detection rule [ESTIMATE thresholds] |
|---|---|---|
| T | Trap wording | Negative stem missed while the chosen option is a *true* statement, and ≥ 2/3 of recent positive-stem items on the same concept are correct. Or `opt_tag = trap_named`. Or `rt_n < 0.4m`, meaning the question was answered faster than it can be read. |
| W | Wrong source | `opt_tag = alt_source` (federal rule or car handbook value). |
| N | Number confusion | `numeric_key` and the chosen tag is a number-type tag. Two or more such misses in the same number family (e.g. every "500 ft" rule) adds the pair to the confusion matrix M[i][j]. |
| E1 | Slip | ≥ 3 of the last 4 correct, P(L) ≥ 0.8, answer marked Sure or normal RT, and the distractor tag is generic. |
| E4 | Retrieval decay | The item had a prior success and `R_at_attempt < 0.8`, or elapsed time > S. |
| E3 | Missing prerequisite | A prerequisite concept has P(L) < 0.5 or has not been studied, confirmed by probing its prerequisites. Walk down to the deepest failing base (`research/learner_model_and_adaptivity.md:81`). |
| E2 | Misconception | The same `misconception:<id>` is chosen on ≥ 2 distinct items. Or `hypercorrect` fires twice. |

Remediation reuses existing lesson parts: Learn-it snippet, Numbers table, Exam traps and handbook page [ARC:00-START-HERE.md:7].

| Cause | Re-teach | Contrast card | Targeted quiz | Effect on scheduling |
|---|---|---|---|---|
| T | Highlight the NOT/EXCEPT keyword; "read twice" | Same fact asked positive, negative and T/F | Flip drill ×3 | Add the flipped variants |
| W | Row from the "Handbook way" table | Handbook value vs elsewhere, with page | 2 MCQs using the alt_source distractor | Retest in about 3 days |
| N | Number-family table plus a memory hook (e.g. "one pound above", GK-01:36) | Neighbouring numbers side by side | Typed numeric cloze (strong evidence), then interleave the confused pair | Interleave pairs (`research/learner_model_and_adaptivity.md:113-114`) |
| E1 | None | — | 1 confirmation probe in 1–2 days | None |
| E4 | None | — | FSRS relearning | FSRS handles it. Chronic lapses → mnemonic or rewrite. |
| E3 | Prerequisite Learn-it snippet | — | 1–2 items per prerequisite | Block **new** cards on the dependent concept; reviews are never blocked (`app_plan/FEATURE_PLAN.md:74`) |
| E2 | Worked explanation | Right model vs wrong model | 2–3 distractor-targeted MCQs, retest at +7 days | Add new cards; keep the old ones |

**Review list:**
- One row per (concept, cause), upserted. The current app creates duplicate rows (`app_plan/APP_ANALYSIS.md:75`).
- Priority = `stakes(test weight High=2 / Med=1) × (1 − P(L)) × (1 + [R < 0.9])` [ESTIMATE].
- Lifecycle: `open → probing → resolved`.
- A row auto-resolves only when all of these hold: probes correct on ≥ 2 distinct days, ≥ 1 of them a non-MCQ (typed) answer where one exists, P(L) ≥ 0.6, and sd low (`TESTING_PLAN.md:79,101-102`). Manual resolve is only a snooze.

## 4. Exam-date planner, daily load and calendar fill

**Inputs:**
- License class → required tests. GK for all classes; Air Brakes if the vehicle has air brakes; Combination for Class A; endorsements P and S (20 questions, 16 to pass), T/N/H [ARC:part-10:14,109; 00-START-HERE:13-19].
- Exam date per test, minutes per day, off-days.
- Lesson time estimates [ARC:00-START-HERE.md:31-49].

**Algorithm** [ESTIMATE shape, following the sched back-planning method]:
1. `days_avail = days to exam − off-days − 1 buffer day`.
2. `cutoff = exam − max(2, ceil(0.2·days_avail))` days. No new lessons after this date.
3. Spread the remaining lesson minutes across the days before the cutoff, in pack order: GK-01…14, then CV-01…04 [ARC:00-START-HERE.md:52].
4. Forecast review load by simulating FSRS due dates × the learner's measured seconds per card.
5. Retention aimed at exam day: if `now + I(r,S) > t_exam` and `R(t_exam − now, S) < 0.9`, schedule a final review within 1–3 days of the exam.
6. Full mock exams at about 60% of the timeline and on the last 2 days.
7. Feasibility: go if every day ≤ 0.85 × budget, tight if ≤ 1.0, otherwise no-go with suggestions (move the date, add minutes). Labels come from `app_plan/APP_ANALYSIS.md:23`.

**Daily queue order:** overdue high-stakes reviews → review-list probes and remediation → new lesson segment → interleaved mixed practice. Trim to budget by postponing mastered-easy items first (`research/learner_model_and_adaptivity.md:127`).

**Calendar fill:**
- `pct = floor(done/planned × 20) × 5`, giving 21 states. The tick shows if and only if done == planned (`research/gamification_visuals_multimodal.md:43`).
- Weight tasks by minutes [ESTIMATE; this is an open build decision, `research/gamification_visuals_multimodal.md:44`].
- A day with nothing planned renders a distinct "rest" state, not a full green fill (`GAMIFICATION_PLAN.md:182`).
- Rest vs missed is decided by the actual due count that day, not by whether a plan row exists (`:234`).
- Future days render as an amber-dashed forecast and never show a tick (`:149,239`).
- Compute day keys in local time (UTC bug noted at `app_plan/FEATURE_PLAN.md:38`).

**Resume:** keep the session cursor, pending queue and review log in IndexedDB, with JSON export/import [ESTIMATE].

## 5. Mastery gates and readiness

**Lesson gate (non-punitive):**
- Unlock the next lesson's *new* material when the lesson's practice test is ≥ 90% [ARC:00-START-HERE.md:59] and every concept in the lesson has P(L) ≥ 0.6.
- A retake uses new items (the pack's generator rule, ARC:151), not the same key.
- Placement quiz can skip ahead. Unlocking is only a navigation convenience; reviews are never blocked (`GAMIFICATION_PLAN.md:55`).

**Readiness estimate** [ESTIMATE, labelled as an estimate, D4]:
- Draw a blueprint-weighted sample of n items (GK n = 50, k = 40; CV 20/16; Air Brakes 25/20).
- Per item: `p_i = R_i(t_exam) + (1 − R_i)·(1/3)`. For unstudied items use the Elo p.
- `P(pass) = P(Σ Bernoulli(p_i) ≥ k)`, computed exactly by a Poisson-binomial DP in O(n²) (fast client-side).
- The official item distribution is [UNKNOWN]. Use the pack's High/Medium weights as a proxy.

Binomial reference at uniform accuracy p [computed]:

| p | GK ≥ 40/50 | CV ≥ 16/20 | Air Brakes ≥ 20/25 |
|---|---|---|---|
| 0.80 | 0.584 | 0.630 | 0.617 |
| 0.85 | 0.880 | 0.830 | 0.838 |
| 0.90 | **0.991** | 0.957 | 0.967 |

This is the quantitative reason for the pack's 90% target.

- **Calibration:** after each fresh mock, record the Brier score of predicted vs observed. If calibration is poor, show "uncertain" (`TESTING_PLAN.md:65`).
- **Display bands:** <50% Not yet · 50–85% Borderline · 85–97% Likely.
- **Hard "Ready" gate:** 2 consecutive fresh full mocks ≥ 90% (GK practice set 93 questions and CV set 68 at ≥ 90%, ARC:156-157) **and** the estimate ≥ 0.97.

## 6. Content type → teaching mode → interaction → Bloom level

Bloom/rung ladder from `research/flashcards_and_book_decomposition.md:318-342`; image-vs-text rule from `:132-139`.

| CDL content type | Examples | Teaching mode | Interactions (learn → test) | Bloom level |
|---|---|---|---|---|
| Exact numbers | 26,001 lb, 20–45 psi, 500 ft, 180/14 days | Number-family table, memory hook | Typed numeric cloze → sort values to rules → 3-option MCQ with neighbour distractors; timed drill allowed | Remember |
| Terms / codes | Endorsements H N P S T X, restrictions L/Z, CLP | Glossary cards | Matching → reversed card only if the answer is unique | Remember |
| Legal rules / conditions | Disqualifications, out-of-service orders, CA-specific laws | Decision table or if-then flowchart | Sort scenarios into penalty buckets → scenario MCQ; "handbook way" contrast | Understand / Apply |
| Systems / mechanisms | Air brakes, tractor protection valve, spring brakes, trailer ABS | Interactive SVG schematic with gauges and animated air flow | Label via image occlusion → "what if there's a leak" manipulation → cause-effect MCQ, hotspot | Understand |
| Procedures / sequences | Coupling/uncoupling (CV-03), vehicle inspection (GK-04), air brake check | Step-by-step simulation (state machine; wrong step shows its consequence) | Drag steps into order → spot the error → overlapping cloze (`research/flashcards_and_book_decomposition.md:163`) | Apply |
| Spatial / visual | Off-tracking, space management, mirrors, railroad crossings | Top-down 2D animation | Drag to position, trailer path tracer, stopping-distance builder | Understand / Apply |
| Judgement situations | Skids, emergencies, night/fog, mountain grades | Branching scenario plus optional YouTube | "Best next action" MCQ | Apply / Analyze |
| Traps (all types) | NOT/EXCEPT, T/F flips | Flip drill | Positive → negative → T/F variants | Understand |

- **Final exam format:** everything converges to 3-option single-best-answer MCQ with flipped variants.
- **YouTube:** only IDs checked by a human; no generated IDs [UNKNOWN until verified].
- **Media:** visuals only for spatial content (media specialist's cost rule, `specialists/media.specialist.json` decision_procedure[1]).
- **Skills-test preparation** (pre-trip inspection spoken aloud): typed or voice recall checklist.

## 7. Topic-specialised gamification

**Binding invariants** (`GAMIFICATION_PLAN.md:67-77`):
- Games read the learner model and never write to it.
- No XP, volume or speed rewards.
- A rest day never breaks a streak.
- Timed data is quarantined.
- A timeout is not a wrong answer (`:27`).

Per-topic mechanics [ESTIMATE designs, each tied to real mastery]:

| Topic | Mechanic |
|---|---|
| Air brakes | "Gauge keeper": run the handbook brake check on the simulator; completing it correctly earns a PASS stamp. |
| Coupling | "Coupling puzzle" state machine; a wrong step animates its consequence (informational only, no lives). |
| Numbers | "Number-family collection" lights up at P(L) ≥ 0.95. This is the only place the water-drop timed drill is allowed (remember-level only, `:23-24`). The response window is `6000 + 3×1800 = 11,400 ms`, minimum 4000 ms (`:247`), and the timer can be turned off. |
| Inspection | "Spot the defect" scene; tracks which defect categories have been cleared. |
| Laws | "Case file" sorting. |
| Traps | "Trap hunter": collection of trap types cleared. |
| Mocks | Test-day rehearsal that copies the real touchscreen and skip behaviour (D8). |
| Course level | License-class path / concept map. "Learned" is shown dimmer ("seen") once its cards' stability drops, never revoked (`:177`). Day box = effort; mastery meter = "n of N assessed" (`:214`). |

- **Streak:** freeze tokens are spent only on a missed day that had due work; always show a "days active in last 30" metric (`:148`, `research/gamification_visuals_multimodal.md:150-153`).
- **Rejected:** XP, badges-for-prizes, leaderboards, hearts/lives, variable-reward loops, fake head-start progress, avatars/pets, speed-scored quizzes (`GAMIFICATION_PLAN.md:49`).
- **Accessibility:** support reduced motion; never rely on colour alone (`:33`).

## 8. 1–10 self-scoring rubric (reuses TESTING_PLAN)

Score = 10 × weighted average of normalised axes. Honesty is weighted above speed (`TESTING_PLAN.md:145`). **Pass is 8.**

| Axis (weight) | Metric |
|---|---|
| Content fidelity (0.20) | Answer keys match the handbook; the numeric audit passes 100% (the ARC:part-10:359 pattern); coverage recall/precision against the lessons (`TESTING_PLAN.md:196-203`) |
| Item quality (0.15) | qcraft bands good/weak/reject, 0 rejects (`:211-219`) |
| Learner-model honesty (0.20) | Synthetic learners fast-and-steady / struggles-on-prereqs / cramper-who-forgets / lucky-guesser (`:35-39`). Lucky-guesser P(L) < 0.4 over 20 items; premature resolution ≤ 0.05 (`FIX_SPEC:45-46`) |
| Calibration (0.10) | Brier score, reliability curve (`TESTING_PLAN.md:65`) |
| Planner (0.10) | Minutes within budget; backlog flat (`:69`) |
| Gamification invariants (0.10) | Zero game writes to the model, rest-day checks (`GAMIFICATION_PLAN.md:135`) |
| Beginner end-to-end UX + accessibility + i18n (0.15) | Scripted beginner walkthroughs with screenshots; reduced motion; Arabic RTL. The handbook's language list is English, Arabic, Chinese, Punjabi, Russian, Spanish [ARC:00-START-HERE.md:24] |

**Hard fails** (any one caps the score at 5): a key that contradicts the handbook; an invented URL or video ID; a game writing to BKT/Elo/FSRS; an FSRS regression (identical grade streams produce different schedules, `FIX_SPEC:50`). Tune thresholds with a fractional-factorial run of about 16 configurations (`TESTING_PLAN.md:151-158`).

Sources:
- [awesome-fsrs wiki: The Algorithm](https://github.com/open-spaced-repetition/awesome-fsrs/wiki/The-Algorithm)
- [py-fsrs scheduler.py](https://raw.githubusercontent.com/open-spaced-repetition/py-fsrs/main/fsrs/scheduler.py)
- [ts-fsrs on npm](https://registry.npmjs.org/ts-fsrs/latest)
- [GeodeMd issue on ts-fsrs 5.x / FSRS-6](https://github.com/T1Fleming/GeodeMd/issues/50)
- [Hypercorrection persists but high-confidence errors return (Springer)](https://link.springer.com/article/10.3758/s13423-011-0173-y)
- [Butterfield & Metcalfe (ResearchGate)](https://www.researchgate.net/publication/11641193_Errors_Committed_with_High_Confidence_Are_Hypercorrected)
- [driving-tests.org CA CDL page](https://driving-tests.org/california/ca-cdl-general-knowledge-test-2/) (dmv.ca.gov is blocked by the egress proxy; test format taken from the archive instead)
