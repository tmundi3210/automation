# APP_ANALYSIS — Your Base44 Study App, read from its own code

_Audience: you, the owner (non-engineer power user). Plain language; terms defined on first use._
_Every load-bearing claim is tagged: **[FACT]** = read directly in your code (file:line cited from the recon) · **[ESTIMATE]** (basis + confidence) = my inference · **[UNKNOWN]** = not knowable from the code + a **[METHOD]** to find out · **[SIGNAL]** = weak hint · **[METHOD]** = how to verify._
_Source of truth for the code citations: `app_plan_recon/APP_RECON.md` (read in full) and `KG_RECON.md` (blueprint codes). PLANNING DOCUMENT — no app code was changed._

---

## §1 What this app is, and a day of using it

[FACT] It is a **single-user, dark-themed study app** built on the Base44 platform (a low-code app host), written in React, that helps you prepare for two exams — **INBDE** (the US dental board exam) and **TOEFL** (English proficiency). The sidebar literally says "INBDE · TOEFL 2026" (`Layout.jsx:20`). It combines three ideas: (1) **spaced repetition** — an algorithm decides when to show each flashcard again; (2) a **learner model** — a running estimate of how well you know each concept; and (3) a **nightly analysis** — a batch job that looks at the day's mistakes and writes a diagnosis plus tomorrow's plan.

**Key terms, once:**
- **FSRS-6** [FACT `submitReview/entry.ts:2`] = "Free Spaced Repetition Scheduler," the modern algorithm (same family Anki uses) that, given your grade on a card and how long it's been, computes the next date to show it. This is the app's scheduling engine.
- **BKT** (Bayesian Knowledge Tracing) [FACT `submitReview/entry.ts:106-111`] = a per-concept "probability you've mastered this" number that ticks up on right answers, down on wrong ones.
- **Elo** [FACT `submitReview/entry.ts:112-137`] = the chess-rating idea applied two ways: your **skill (θ, "theta")** per concept and each **card's difficulty (b)**; they update against each other after every answer.

**A day of use, end to end:**
1. **Open the Dashboard (`/`).** It shows four stats (Due Reviews, New Cards, Today's %, Streak), a calendar of day-boxes that fill green as you complete work, and a list of open weaknesses. [FACT] Merely loading this page **creates or updates today's plan** (`DailyPlan`) (`Dashboard.jsx:32-42`).
2. **Go to Review (`/review`).** The app pulls the cards due now (up to 30, earliest-due first, `Review.jsx:30`), shows one at a time. For multiple-choice you pick an option; for free-text you can type or **speak** your answer. You may ask the AI to suggest a grade ("Grade with AI"), but **you still press the final 1–4 button yourself** (`Review.jsx:144, 56-57`). Each press calls `submitReview`, which reschedules the card, updates your mastery numbers, and ticks the day's progress up by one.
3. **Add material in Library (`/library`).** You either hand-write a card (`CardForm`) or ask the AI to generate a batch from a topic (`GenerateCardsDialog` → `generateCards`). New cards become due immediately.
4. **Check Learner Model (`/learner`).** Bar charts of your mastery per exam-blueprint area, a trend line over time, and the "Open Weaknesses (E1–E4)" list with a **Mark Resolved** button.
5. **Plan on Planner (`/planner`).** Enter an exam date and how many cards you must learn; it estimates whether the timeline is feasible (go / tight / no-go) and a recommended new-cards-per-day.
6. **End of day — press "Run nightly analysis" on the Dashboard.** [FACT — this is the crux] Despite the name "nightly," **nothing runs on its own.** The only trigger for the analysis is you clicking that button (`Dashboard.jsx:72-82`; no cron/scheduler exists anywhere — grep-verified, `APP_RECON §3b`). That click runs `runPipeline`, which grades the day's calibration, diagnoses weaknesses, builds tomorrow's plan, and snapshots mastery.

[FACT] Voice answers are recorded and dropped into a `VoiceQueue` marked "queued," waiting for an "overnight worker" to transcribe and grade them — **but that worker does not exist in this codebase** (`VoiceRecorder.jsx:5`; grep `VoiceQueue` in functions → zero). As shipped, voice answers sit in "queued" forever.

---

## §2 Architecture map

**Pages (what you click) and what each touches:**

| Page (route) | Reads / writes (entities = database tables) | Backend function it calls |
|---|---|---|
| Dashboard `/` | reads CardState, DailyPlan, Weakness, Settings, VoiceQueue, Concept; **writes DailyPlan on load** (`Dashboard.jsx:32-42`) | `runPipeline` (the "Run nightly analysis" button) |
| Review `/review` | reads due CardState, Card, ItemOption; writes VoiceQueue rows | `submitReview` (per grade), `gradeTextAnswer` (AI grade) |
| Library `/library` | reads/writes Deck, Card, CardState; CardForm writes Concept/Card/ItemOption/CardState | `generateCards` |
| Learner Model `/learner` | reads LearnerState, Concept, Weakness, MasteryHistory; **writes Weakness.status** (`:55-58`) | none |
| Planner `/planner` | reads ExamPlan, Settings | `planExam` |
| Settings `/settings` | reads/writes Settings | none |

**Backend functions (Deno/TypeScript on Base44) and what they write:**

| Function | When it runs | Writes |
|---|---|---|
| `submitReview` | every 1–4 grade press | Revlog, CardState (FSRS reschedule), LearnerState (BKT+Elo), ItemElo, DailyPlan progress |
| `runPipeline` | **only the Dashboard button** | PipelineRun, CalibrationBin, Weakness, DailyPlan (tomorrow), MasteryHistory, SystemMetrics |
| `planExam` | Planner button | ExamPlan (one per exam) |
| `generateCards` | Library dialog | Concept, Card, ItemOption, CardState |
| `gradeTextAnswer` | Review "Grade with AI" | **nothing** — pure AI call, returns a suggested score/rating |

**Dead schema — tables/fields the code never uses** [FACT `APP_RECON:34`]: the `Exam` entity (zero references), `VoiceQueue` (no consumer), `DailyPlan.per_exam_alloc_json`, `Card.quality_flag`, `SystemMetrics.llm_cost_usd`/`stt_cost_usd`, `Concept.inbde_cc` (the fine 1–56 code). These are "promises" in the schema with no behavior behind them.

---

## §3 The loops it has today, and the with-time verdict

A "loop" = something is measured, then that measurement changes future behavior. Loops that feed back are self-correcting; measurements that go nowhere just drift.

**CLOSED — genuinely self-correcting:**
- **Review → schedule → due queue → review.** [FACT] Every grade re-fits the card's memory state via FSRS (`submitReview/entry.ts:77-90`) and the queue automatically re-orders by due date (`Review.jsx:30`). This is standard FSRS and **stays healthy with time** with no maintenance.
- **Review → day progress → day-box fill / streak.** [FACT] `submitReview/entry.ts:140-150` bumps `done_units` and the green fill. Real, but purely motivational.

**HALF-OPEN — computed, shown, but never acted on:**
- **Review → BKT/Elo mastery → display only.** [FACT] Nothing reads `bkt_p_mastery`, `elo_theta`, or `elo_b` to pick or schedule cards — the queue is due-order only (`Review.jsx:30`; grep-verified `APP_RECON:66`). Your mastery numbers decorate charts; they do not steer what you study.

**OPEN — measured nightly, consumed by nobody:**
- **CalibrationBin + SystemMetrics** (how well the app's predictions matched reality; Brier/log-loss scores) are written by `runPipeline` but **read by no page and no code** (grep `CalibrationBin|SystemMetrics|ItemElo|PipelineRun` in `src/` → zero, `APP_RECON:67`). The re-fitting step that would close this ("re-optimize FSRS from your history") is delegated to the off-repo worker; `submitReview` is *ready* to use `Settings.w_optimized` (`:22-24`) but nothing ever writes it.
- **Diagnosis without treatment.** [FACT] Weakness rows carry a `prescribed_fix` sentence and `probe_card_ids`, but no code schedules a probe, blocks a card, or moves status to "probing" (`APP_RECON:68`). The prescriptions are advice strings, not actions.

**What self-corrects vs. what drifts with time:**
- **Self-corrects:** each card's FSRS state (every grade); today's `planned_units` (Dashboard recomputes on load, `Dashboard.jsx:35-42`).
- **Drifts if you never press the button:** weaknesses, the mastery trend chart (needs ≥2 pipeline run-dates, `MasteryTrendChart.jsx:53-56`), calibration, self-metrics, and tomorrow's plan — all stale, because **there is no scheduled execution anywhere** [FACT, grep-verified `APP_RECON:72`].
- **Drifts structurally even if you do press it:** BKT parameters are never re-fit; the uncertainty bands (`elo_theta_sd`) shrink toward 0.3 no matter how surprising your answers are (`submitReview/entry.ts:122`), so the model grows quietly overconfident; calibration error is measured but never corrects anything; and old weaknesses are only cleared for the *same* run-date (`runPipeline/entry.ts:53`), so a persistent weakness spawns duplicate open rows day after day.

**Verdict:** [FACT-grounded] One genuinely closed loop (FSRS scheduling — good forever). One well-instrumented but **unclosed** meta-loop (calibration → re-fit; diagnosis → remediation). With time, the flashcard core stays excellent; the "adaptive brain" stays merely descriptive and silently goes stale without your manual click.

---

## §4 Your questions, answered from the code

### (a) "why do the open weaknesses (E1–E4) exist" — and the confusing phrases

**Why they exist / where they come from:** [FACT] Only `runPipeline` creates Weakness rows (`runPipeline/entry.ts:108-115`), from *today's missed cards*, grouped by concept. "E1–E4" (heading `LearnerModel.jsx:75`) is a **differential diagnosis** — like a doctor listing four possible causes for one symptom. The four causes are the enum values in `Weakness.jsonc`: [ESTIMATE, high — code never binds the letters explicitly] **E1 = slip, E2 = misconception, E3 = missing_prereq, E4 = retrieval_decay.** For each concept you missed, the pipeline runs down a rule ladder and picks the first cause that fits (`runPipeline/entry.ts:92-106`).

> Owner's phrase: **"capillary misconceptions, confidence 40%, undersampled probe"**

[FACT + ESTIMATE] Read this as three separate things stuck together:
- **"capillary"** = the **concept's name** (the subject matter — a card about capillaries). [ESTIMATE, high] This is your data, not a code term; the app just prints the concept it diagnosed.
- **"misconception"** = the diagnosed **cause**. Here it is the **fallback** cause: when none of the specific rules fire, the pipeline defaults to `misconception` at **confidence 0.4** with the fix text *"Under-sampled — probe with 2-3 varied items on this concept to firm the diagnosis."* (`runPipeline/entry.ts:105`).
- **"confidence 40%"** = [FACT] `WeaknessList.jsx:32` prints `confidence × 100%`. Crucially, this number is a **hardcoded constant of whichever rule fired (0.4–0.8), NOT a real computed probability.** 40% is simply the fallback rule's stamped-in value — it means "I don't have enough of your data to be sure," not "40% likely."
- **"undersampled probe"** = that same fallback fix text. "Under-sampled" = the app saw too few attempts on this concept to diagnose confidently, so it *recommends* probing. [FACT] But no probe is ever scheduled — the text is advice only.

> Owner's phrase: **"2–3 multivariate items … to form the diagnosis"**

[FACT] This is the *same* fallback fix string ("probe with 2-3 varied items," `runPipeline/entry.ts:105`). Plain meaning: *"I'm not sure what's wrong yet; give me 2–3 more varied questions on this concept and I'll be able to tell."* ("multivariate" ≈ "varied.") [FACT] Nothing in the app actually generates or schedules those 2–3 items — you would have to study the concept again yourself and re-run the pipeline.

> Owner's phrase: **"missing prerequisite … block new cards until prerequisite mastered"**

[FACT] This is the **E3 = missing_prereq** rule (`runPipeline/entry.ts:92-106`). Trigger: you missed a concept that has a **prerequisite** concept (linked via a `ConceptEdge` of type `prereq`) whose mastery is below 0.5. Diagnosis: you're failing the harder concept because you haven't mastered the thing it depends on. Fix text: *"Schedule the prerequisite concept first; block new cards on this concept until prereq mastery ≥ 0.6."* [FACT — important] **This blocking is text only. No code blocks anything** (grep-verified `APP_RECON:85`). The app tells you to block new cards; it never does.

> Owner's request: **exactly what "Mark Resolved" does, and what should happen automatically**

[FACT] The **entire** effect of Mark Resolved is one line: `Weakness.update(w.id, { status: "resolved" })` then reload (`LearnerModel.jsx:55-58`). Downstream: the row vanishes from the two open-status lists (Dashboard `:20`, LearnerModel `:34`). **No card is rescheduled, no mastery number changes, nothing is re-studied.** Two footguns [FACT `APP_RECON:92`]: (1) if you miss that concept again on a later day, the next pipeline run creates a fresh open weakness; (2) re-running today's pipeline **wipes your same-day "resolved" mark** (it deletes today's weakness rows and re-diagnoses).

**What *should* happen automatically instead** [ESTIMATE — my proposal, no code written]: Resolving a weakness should be a *consequence*, not a *button*. When a weakness is diagnosed, the app should enqueue the prescribed action it already names — for a misconception, schedule a same-misconception contrast probe; for missing_prereq, actually raise the prerequisite's priority and gate the dependent concept's new cards; for retrieval_decay, trust FSRS; for a slip, one confirm-probe. The weakness should then carry status `open → probing → resolved`, and it should **auto-resolve only when the evidence recovers** — e.g. the concept's mastery (θ or BKT) climbs back above the diagnosis threshold and stays there across the next K pipeline runs — with duplicates deduplicated across days by concept+cause rather than re-created each run. Manual "Mark Resolved" would remain only as an override. This is exactly the "close the remediation loop" gap in §3; it is greenfield work, not a tweak.

### (b) The Dashboard/Planner numbers: INBDE, "new 24," date, budget share, total cards

[FACT — clarification] Most of these live on the **Planner** result card, not the Dashboard (`Planner.jsx:82-96`), one card per exam:
- **"INBDE"** = the exam code (a plan card exists per exam; the other is TOEFL). The header is `exam_code` + `exam_date`.
- **"new 24" (i.e., "New / day")** = **`rec_new_per_day`, the Planner's recommended number of brand-new cards to introduce per day** to finish on time (`Planner.jsx`, from `planExam`). It is a *daily allowance*, **not** a deck size. [ESTIMATE, high] "new 24" = "learn 24 new cards a day." On the **Dashboard**, the separate "New Cards" stat is `min(new-card due count, Settings.new_per_day ?? 10)` (`Dashboard.jsx:25-27`) — same idea, your per-day new-card cap.
- **date** = on the Dashboard header it's just today's date (`new Date().toDateString()`, `Dashboard.jsx:94`); on a Planner card it's `exam_date`, and "Last day to learn" = exam_date minus a 14-day headroom (`planExam/entry.ts:22,63`).
- **"Budget share"** = [FACT] that exam's **slice of your shared daily study-minutes budget**, split between exams **in proportion to how many cards each has** (`daily_minutes × total_cards / totalCardsAll`, `planExam/entry.ts:25,69`). If INBDE has more cards than TOEFL, INBDE gets the bigger minutes share.
- **"Total cards"** = [FACT] the number **you typed** as "cards to learn" for that exam (the Planner form defaults to 2000 INBDE / 800 TOEFL, `Planner.jsx:14-16`). It is your target, not a count of cards that exist in the app.
- The little chart under a Planner card is the projected daily-minutes workload curve (`projected_curve_json`). [FACT] The Dashboard itself has **no** chart; the mastery **trend** chart is on Learner Model.
- **"Streak"** [FACT `Dashboard.jsx:44-60`] = consecutive days you met `day_done_threshold` (default 0.8 of your plan), with `streak_slack_tokens` (default 2) auto-spent to forgive missed days.
- **Day-boxes** [FACT `DayBoxCalendar.jsx`] = each box's green fill = `daybox_state × 5%` (a 0–20 scale, tick at 100%), today gets a green border. **Clicking a day does nothing** — day boxes have no click handler (`APP_RECON:96`).

### (c) "FK6 pathology" and its sibling codes

[FACT] **FK = Foundation Knowledge area; the INBDE blueprint is a matrix of 10 FK areas × 56 Clinical Content (CC) areas** (`KG_RECON §3`; source jcnde.ada.org/inbde). Every INBDE item integrates ≥1 FK area applied within a CC area — that's the "Integrated" in INBDE. So these are **official exam-blueprint axes, not free tags you invented.**
- **"FK6 Pathology"** = [FACT `LearnerModel.jsx:8`, hardcoded label] Foundation Knowledge area 6 = **"General and disease-specific pathology to assess patient risk"** (`KG_RECON:126`). The bar labeled FK6 is your **average mastery across all your concepts tagged `inbde_fk = 6`.**
- **Sibling FK codes** [FACT `KG_RECON:122-130`]: FK1 molecular/cellular biology, FK2 physics & chemistry of normal/abnormal biology, FK3 physics & chemistry of materials/technologies, FK4 genetic/congenital/developmental disease, FK5 immune & host defense, **FK6 pathology**, FK7 microbiology, FK8 pharmacology, FK9 behavioral science/ethics/law, FK10 research methods & informatics. [ESTIMATE-high wording — exact phrasing pending the JCNDE PDF, see below.]
- **The CC (Clinical Content) side** [FACT] is grouped into 3 sections the app labels: **DTP** = Diagnosis & Treatment Planning, **OHM** = Oral Health Management, **PP** = Practice & Profession (`LearnerModel.jsx:9`). Your mastery bars aggregate to these 10 FK + 3 CC axes — the same axes JCNDE uses on its remediation report.
- [UNKNOWN] The full verbatim list of all **56 CC areas** and the official per-FK/per-CC weightings are **not in your repo** — JCNDE publishes them as figure-images and the pages returned HTTP-403 this session. [METHOD] Pull the **INBDE Item Development Guide** + **Model Domain of Dentistry** PDFs (jcnde.ada.org; UF mirror `dental.ufl.edu/.../INBDE_Item_Development_Guide.pdf`) from an unblocked network. [FACT] A finer `Concept.inbde_cc` (1–56) field exists in your schema but is used **nowhere** in the code.

### (d) How cards work today

- **Hand-authoring (CardForm)** [FACT `CardForm.jsx`]: you type a prompt, expected answer, choose MCQ or free-text, a Bloom level, an optional INBDE "Patient Box" vignette, and one concept tag (find-or-create by exam + name). For MCQ you enter 4 options, mark the correct one, and can tag each *wrong* option with a **`misconception_tag`** (a short label naming the mistake). Saving creates the Card + options + a CardState due immediately.
- **AI generation (generateCards)** [FACT `generateCards/entry.ts`]: you give a topic, a count (1–15), and optional grounding notes; the backend calls Base44's built-in LLM (`Core.InvokeLLM` — **no model name is chosen anywhere**, `APP_RECON:126`) with a prompt encoding good-card rules (Wozniak minimum-information, NBME item-writing, a misconception_tag required on every wrong option, original non-copyrighted items). It creates one Concept for the topic plus the cards. [SIGNAL] The dialog never sends the `inbde_fk`/`inbde_cc_section` params the function can accept, so AI cards land unmapped to the blueprint.
- **Grading (gradeTextAnswer)** [FACT `gradeTextAnswer/entry.ts`; `Review.jsx:56-57,144`]: for free-text, the AI returns a suggested 0–1 score, a 1–4 rating, a rationale, and `error_tags`. It is **advisory** — it writes nothing; **you press the final 1–4 button.** [SIGNAL — important gap] `error_tags` are shown but **never saved**, so free-text/voice misconceptions never reach the weakness diagnoser, which only sees MCQ `chosen_option_id`.
- **Voice (VoiceRecorder → VoiceQueue)** [FACT `VoiceRecorder.jsx`]: records a webm clip, uploads it, and creates a `VoiceQueue` row marked "queued." The Dashboard counts it toward today's plan and says "N voice answers queued for overnight transcription + grading." [FACT] **No code transcribes or grades them.** As shipped, they stay queued forever.

---

## §5 What is ABSENT today (verified-absence list = your feature runway)

Each is grep-verified empty in the code (`APP_RECON §5`). This is the honest gap list to build against.

1. **No real scheduler.** [FACT] Everything labeled "nightly" is a manual Dashboard button; no cron/automation exists (Base44 config is site-build only, `APP_RECON:135`). Any recurring behavior (auto weakness refresh, calendar sync, Anki sync, voice draining) needs a scheduler first.
2. **No Anki export/import.** [FACT] Only Settings *labels* mention ".apkg export" and "AnkiConnect"; **no export code exists** (`APP_RECON:119`).
3. **No Google Calendar / reminders** [FACT — zero hits].
4. **No WhatsApp** [FACT — zero hits].
5. **No MCP server** [FACT — zero hits].
6. **No knowledge-graph view, and no way to create edges.** [FACT] `Concept` and `ConceptEdge` tables exist and the pipeline *reads* prereq edges once (`runPipeline/entry.ts:84-89`), but **no page renders a graph, no graph library is installed, and no code ever creates a ConceptEdge** (`APP_RECON:123`). Your book→units→edges method (`KG_RECON §2`) is fully specified in the FACTORY assets but **not built into this app.**
7. **No document/PDF/folder ingestion.** [FACT] The only upload is the single voice clip (`APP_RECON:125`).
8. **No user-selectable LLM / Ollama.** [FACT] Both AI calls are `Core.InvokeLLM` with no model parameter; the only engine setting is `stt_engine`, itself a "deferred flag" with no consumer (`APP_RECON:126`).
9. **No hover tooltips / onboarding / guide layer** on app components (only chart tooltips exist) (`APP_RECON:127`).
10. **No day-detail view** — clicking a calendar day does nothing (`APP_RECON:128`).
11. **No review-frequency setting.** [FACT] The closest tweakable knob is the `desired_retention` slider (0.70–0.97) — the single FSRS workload dial. Also tweakable: `daily_minutes`, `new_per_day`, `w_source`, `stt_engine`, Anki mode, streak slack, burnout threshold. [FACT] `burnout_threshold` is stored but **enforced nowhere** (`APP_RECON:129`).
12. **The learner model steers nothing.** [FACT] BKT/Elo/ItemElo/CalibrationBin feed no scheduling or item-selection decision. An adaptive queue or Elo-matched probing is greenfield (`APP_RECON:143`).
13. **The remediation loop is open.** [FACT] Weakness prescriptions ("probe," "block new cards until prereq mastered") are prose; nothing enforces them, duplicates accumulate across days, and same-day reruns wipe "resolved" marks (`APP_RECON:142`). See §4(a) for the auto-resolve proposal.

[METHOD to convert this runway into specialists/features under house rules — not done here]: name the gap in an L2 build plan, run L1 research, then the MAKE_A_SPECIALIST pipeline. The nine existing study_system specialists (sched, qcraft, dental, toefl, contenteng, learner, engage, voice, sysloops) already cover most surfaces; the uncovered ones per `SPECIALIST_RECON` are: knowledge-graph visualization, design-system/color, platform integrations beyond Anki, media/3D generation, document ingestion, UX explainability/onboarding, LLM/model-ops, agent runtime, and app telemetry.
