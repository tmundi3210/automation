# APP_RECON — Base44 Study App (INBDE/TOEFL adaptive study engine)

All file paths are relative to `/tmp/claude-0/-home-user-automation/90e26d01-e32a-5b75-aa3c-38fff390c680/scratchpad/base44_app`. Every claim below is [FACT] read directly from those files unless tagged otherwise. Line numbers cite the file as read on 2026-07-08.

---

## 1. WHAT IT IS

[FACT] A single-user, dark-themed React 18 / Vite app exported from Base44 (app id `6a4d7164497f2266c1d3ac55`, `base44/.app.jsonc:2`) that is a spaced-repetition + adaptive-diagnosis study engine for two exams: INBDE (dental boards) and TOEFL (`base44/entities/Exam.jsonc` enum `["INBDE","TOEFL"]`; sidebar caption "INBDE · TOEFL 2026", `src/components/Layout.jsx:20`). Frontend pages (Dashboard, Review, Library, Learner Model, Planner, Settings — routes in `src/App.jsx:45-52`) talk to Base44 entities directly via `@base44/sdk` (`src/api/base44Client.js`) and to five Deno backend functions (`base44/functions/*/entry.ts`). The scheduling core is FSRS-6 via `ts-fsrs@5.2.1` (`submitReview/entry.ts:2`); the learner model is per-concept Bayesian Knowledge Tracing (BKT) plus a two-sided Elo (learner θ per concept, item difficulty b per card) (`submitReview/entry.ts:92-138`); a manually-triggered "nightly" pipeline does recall calibration, E1–E4 weakness diagnosis, tomorrow's plan, mastery snapshots, and system self-metrics (`runPipeline/entry.ts`). Cards are created by hand or by an LLM (`generateCards`), free-text answers can be AI-graded (`gradeTextAnswer`), and voice answers are recorded to a `VoiceQueue` for an off-repo "home-lab Worker" that does not exist in this codebase.

---

## 2. ARCHITECTURE MAP

| Page (route) | Components used | Entities read/written from page | Backend functions invoked |
|---|---|---|---|
| `Dashboard.jsx` (`/`) | `StatsRow`, `DayBoxCalendar`, `WeaknessList` | reads CardState, DailyPlan, Weakness, Settings, VoiceQueue, Concept; **creates/updates DailyPlan on load** (`Dashboard.jsx:32-42`) | `runPipeline` (button "Run nightly analysis", `Dashboard.jsx:72-82`) |
| `Review.jsx` (`/review`) | `ReviewCard`, `VoiceRecorder` | reads CardState (due ≤ now, 30), Card, ItemOption; VoiceRecorder creates VoiceQueue rows | `submitReview` (each rating, `:51`), `gradeTextAnswer` (`:76`) |
| `LibraryPage.jsx` (`/library`) | `CardForm`, `GenerateCardsDialog` | reads/creates Deck; reads/deletes Card + CardState; CardForm creates Concept/Card/ItemOption/CardState | `generateCards` (`GenerateCardsDialog.jsx:20`) |
| `LearnerModel.jsx` (`/learner`) | `MasteryTrendChart`, `MasteryBars` ×4, `WeaknessList` | reads LearnerState, Concept, Weakness, MasteryHistory; **updates Weakness.status** (`:55-58`) | none |
| `Planner.jsx` (`/planner`) | inline Recharts LineChart | reads ExamPlan, Settings | `planExam` (`:37`) |
| `AppSettings.jsx` (`/settings`) | form fields | reads/creates/updates Settings | none |

**Backend functions → entities they write:**

| Function | Trigger | Writes |
|---|---|---|
| `submitReview` | Review page, per rating | Revlog (create), CardState (upsert), LearnerState (upsert per concept), ItemElo (upsert), DailyPlan (done_units/daybox_state) |
| `runPipeline` | Dashboard button only | PipelineRun, CalibrationBin (delete+create), Weakness (delete+create for run_date), DailyPlan (tomorrow), MasteryHistory (delete+bulkCreate), SystemMetrics (delete+create) |
| `planExam` | Planner button | ExamPlan (upsert per exam_code) |
| `generateCards` | Library dialog | Concept (find-or-create), Card, ItemOption, CardState |
| `gradeTextAnswer` | Review "Grade with AI" | **nothing** — pure LLM call, returns score/rating/rationale/error_tags |

[FACT] Entities defined but written/read by nothing in this repo: `Exam` (grep `entities.Exam` in src+functions → zero hits), `VoiceQueue` has no consumer function (grep `VoiceQueue` in `base44/functions/` → zero), `DailyPlan.per_exam_alloc_json` never written, `Card.quality_flag`, `SystemMetrics.llm_cost_usd/stt_cost_usd` never set, `Concept.inbde_cc` (1–56) never referenced outside the entity file.

---

## 3. LOOPS IT ALREADY HAS

### 3a. submitReview — the per-review loop (runs on every rating click)
[FACT] `submitReview/entry.ts` updates, in order:
1. **Revlog** — created with full FSRS reconstruction fields (stability/difficulty before, retrievability_at_review, w_version, desired_retention_at_review, chosen_option_id, correct, grade_source) (`:58-75`).
2. **CardState** — upserted with FSRS-6 next schedule from `ts-fsrs` `scheduler.repeat(fsrsCard, now)[rating]`, honoring `Settings.desired_retention` and `Settings.w_optimized` when `w_source==='optimized'` (`:17-25, 77-90`).
3. **LearnerState** — per concept on the card: BKT posterior update (slip capped at 0.10, guess at 0.30, `:106-111`) and Elo θ update with K=0.3 against item difficulty b (`:112-126`); `elo_theta_sd` decays ×0.97 to a 0.3 floor regardless of outcome (`:122`).
4. **ItemElo** — item difficulty b nudged with K=0.15 against mean concept θ (`:128-137`).
5. **DailyPlan** — today's `done_units`+1 and `daybox_state = floor(done/planned*20)` (`:140-150`).

[FACT] It does **not** touch CalibrationBin or MasteryHistory — those belong to runPipeline.

### 3b. runPipeline — the batch analysis loop (manual only)
[FACT] Despite the comment "Nightly analysis pipeline" (`runPipeline/entry.ts:3`), the only trigger in the repo is the Dashboard button (`Dashboard.jsx:72-82,97-99`). [FACT] No cron/scheduled-task config exists: grep `cron|schedule|trigger|nightly` across `base44/`, `vite.config.js`, `package.json` matches only code comments and FSRS field names; `base44/config.jsonc` contains only site build commands. [SIGNAL] The header comment says a "home-lab Worker owns STT + FSRS optimizer re-fit" (`:3-5`) — that worker is outside this repo; stage numbering even skips from "Stage 1" to "Stage 3" (`:26,52`), consistent with a missing externally-owned stage 2.

Stages (idempotent, keyed by `run_date`, delete-then-recreate):
- **Stage 1 Calibration** (`:26-50`): bins the day's Revlogs (≤500, `:22-24`) by predicted retrievability decile vs actual `correct`; writes CalibrationBin rows with Brier/logloss.
- **Stage 3 E1–E4 diagnosis** (`:52-118`): from today's misses, builds per-concept evidence and creates Weakness rows (see §4a).
- **Stage 4 Tomorrow's plan** (`:120-134`): counts CardState due by tomorrow, caps new at `Settings.new_per_day`, upserts tomorrow's DailyPlan `planned_units`.
- **Stage 5 Mastery snapshot** (`:136-164`): averages `LearnerState.bkt_p_mastery` per FK area (`fk|N`) and CC section (`cc|DTP/OHM/PP`), bulk-writes MasteryHistory — this is the sole data source of the MasteryTrendChart.
- **Stage 6 Self-metrics** (`:166-179`): SystemMetrics with mean Brier/logloss, schedule adherence (done/planned of today's DailyPlan), minutes studied.

### 3c. planExam — feasibility planner (manual only)
[FACT] `planExam/entry.ts` runs a heuristic forward simulation: an interval "ladder" approximating FSRS growth (`interval *= 2.3 * clamp(log(dr)/log(0.9), 0.5, 2.5)`, `:11-18`), splits the shared daily minute budget across exams pro-rata by `total_cards` (`:25`), binary-searches the max sustainable new/day within that budget (`:49-54`), classifies feasibility go/tight/no_go (85% band, `:57-59`), and **upserts one ExamPlan per exam_code** (`:76-78`) including `projected_curve_json` (downsampled daily-minutes curve, `:74`). [FACT] It does **not** write DailyPlan; DailyPlan rows come from three writers: Dashboard load (`Dashboard.jsx:32-42`), submitReview (`:140-150`), and runPipeline stage 4.

### 3d. Verdict — which loops are truly closed, and what drifts
- **CLOSED (self-correcting):** review → Revlog/CardState (FSRS) → due queue → next review. Each rating re-fits the card's memory state; the queue (`Review.jsx:30`, due ≤ now sorted by due) automatically reflects it. This will stay good with time — it is standard FSRS.
- **CLOSED (small):** review → DailyPlan.done_units → DayBox fill/streak → (motivational only).
- **HALF-OPEN:** review → BKT/Elo LearnerState/ItemElo → **display only**. [FACT] Nothing reads `bkt_p_mastery`, `elo_theta`, or `elo_b` for scheduling or item selection — the review queue is due-order only (`Review.jsx:30`); mastery feeds MasteryBars/MasteryHistory and the pipeline's diagnosis thresholds, never the scheduler.
- **OPEN (measured, never fed back):** CalibrationBin + SystemMetrics are computed nightly but consumed by no UI and no code (grep `CalibrationBin|SystemMetrics|ItemElo|PipelineRun` in `src/` → zero hits). The FSRS w-vector re-fit that would close this loop is delegated to the off-repo Worker; `submitReview` is *ready* to consume `Settings.w_optimized` (`:22-24`) but nothing in the repo ever writes it.
- **OPEN (diagnosis without treatment):** Weakness rows carry `prescribed_fix` text and `probe_card_ids`, but no code schedules probes, blocks new cards, or transitions status to `probing` (grep `probing|probe` in `src/` and functions → only the creation site). The prescriptions are advice strings, not actions.

**What drifts / what self-corrects over time:**
- Self-corrects: per-card FSRS state (every rating), DailyPlan planned_units (Dashboard recomputes on load, `Dashboard.jsx:35-42`).
- Drifts if the user never presses the button: Weaknesses, MasteryHistory (trend chart needs ≥2 run dates, `MasteryTrendChart.jsx:53-56`), CalibrationBin, SystemMetrics, tomorrow's plan — all stale, because there is **no scheduled execution anywhere** [FACT, grep-verified above].
- Drifts structurally: BKT parameters (pT/pS/pG) are never re-fit; `elo_theta_sd`/`elo_b_sd` decay monotonically toward 0.3 no matter how surprising outcomes are (`submitReview/entry.ts:122,134`), so stated uncertainty becomes overconfident; calibration error is measured but never corrects desired_retention or w; Weakness rows from prior days are only deleted for the *same* run_date (`runPipeline/entry.ts:53`), so a persistent weakness accumulates duplicate open rows across days.

**Verdict:** it has one genuine closed feedback loop (FSRS review scheduling) and a well-instrumented but *unclosed* meta-loop (calibration → parameter re-fit; diagnosis → remediation). WITH TIME the SRS core stays good; the adaptive/diagnostic layer stays merely descriptive and will silently stale without the manual pipeline click.

---

## 4. UI SEMANTICS THE OWNER ASKED ABOUT

### 4a. LearnerModel "Open Weaknesses (E1–E4 diagnosed)"
- **Where a Weakness comes from:** [FACT] only `runPipeline` creates them (`runPipeline/entry.ts:108-115`); no other writer exists except LearnerModel's status update. The heading "E1–E4" (`LearnerModel.jsx:75`) maps to the four `cause` enum values in `Weakness.jsonc`: **slip, misconception, missing_prereq, retrieval_decay** — the code comment calls this "E1–E4 differential diagnosis on today's misses" (`runPipeline/entry.ts:52`). [ESTIMATE, high] E1=slip, E2=misconception, E3=missing_prereq, E4=retrieval_decay by enum order; the code never binds the letters explicitly.
- **Decision rules** (`runPipeline/entry.ts:92-106`), evaluated per concept over the day's missed cards:
  1. Same wrong-option `misconception_tag` chosen ≥2× → `misconception`, confidence 0.8, fix: "Targeted correction + contrast pair for misconception \"<tag>\"; probe with a same-misconception item."
  2. Else any prereq concept (via `ConceptEdge type:'prereq'`) with `bkt_p_mastery < 0.5` → `missing_prereq`, confidence 0.7, fix: **"Schedule the prerequisite concept first; block new cards on this concept until prereq mastery ≥ 0.6."** [FACT] This blocking is *text only* — no code enforces it anywhere (grep-verified).
  3. Else any miss with retrievability < 0.7 and mastery ≥ 0.5 → `retrieval_decay`, confidence 0.7, fix: "Pure FSRS reschedule…".
  4. Else mastery ≥ 0.6 and exactly one miss → `slip`, confidence 0.6, fix default "One confirm probe; do not reschedule aggressively."
  5. Fallback → `misconception`, confidence 0.4, fix: **"Under-sampled — probe with 2-3 varied items on this concept to firm the diagnosis."** (`:105`) — this is the owner's "undersampled probe / 2-3 multivariate items to form the diagnosis" string.
- **Misconception label:** comes from `ItemOption.misconception_tag` — a snake_case label attached to each *wrong* MCQ option at authoring time (manual `CardForm.jsx:74`, or LLM-generated with default `'unspecified'`, `generateCards/entry.ts:94`), captured at review time via `chosen_option_id` (`Review.jsx:55`, `runPipeline/entry.ts:60-63`).
- **"confidence 80%":** [FACT] `WeaknessList.jsx:32` renders `Math.round(w.confidence*100)%`. It is the hardcoded heuristic constant of whichever rule fired (0.4–0.8) — not a computed posterior probability.
- **probe_card_ids:** first 3 missed card ids (`runPipeline/entry.ts:112`) — stored, never consumed by any code.
- **"Mark Resolved" exact mutation:** [FACT] `LearnerModel.jsx:55-58`: `Weakness.update(w.id, { status: "resolved" })` then reload. That is the *entire* effect — no CardState, LearnerState, or scheduling change. "What happens next": the row disappears from both open-status queries (`Dashboard.jsx:20`, `LearnerModel.jsx:34` filter `status:'open'`). If the same concept is missed again on a later day, the next pipeline run creates a fresh open Weakness (it only deletes rows whose `detected_run_date` equals that run's date, `runPipeline/entry.ts:53` — which also means re-running today's pipeline wipes a same-day "resolved" mark and re-diagnoses).

### 4b. Dashboard StatsRow + DayBoxCalendar + charts
- **StatsRow** (`Dashboard.jsx:107-114`): "Due Reviews" = CardState rows due ≤ now with state ≠ new; "New Cards" = `min(new-state due count, Settings.new_per_day ?? 10)` (`:25-27`) — so **"new 24" style figures are a per-day new-card allowance/count, not a deck size**; "Today" = `done_units/planned_units` % of today's DailyPlan; "Streak" = consecutive days meeting `day_done_threshold` (default 0.8) with `streak_slack_tokens` (default 2) auto-spent on missed days (`:44-60`). The header date is just `new Date().toDateString()` (`:94`).
- **DayBoxCalendar** (`components/dashboard/DayBoxCalendar.jsx`): month/week grid of white boxes; each box's green fill = `DailyPlan.daybox_state × 5%` (0..20 scale, owner-spec comment `:4` "white empty box → green download-bar fill (5% increments, 0..20) → tick at 100%"); check-mark at state 20; today gets a green border; future days dimmed. [FACT] Day boxes have **no click handler** — the only `onClick` in the file is the week/month toggle (`:63`).
- **"INBDE", "budget share", "total cards", date:** these live on the **Planner** result card (`Planner.jsx:82-96`), one card per ExamPlan: header = `exam_code` + `exam_date`; "New / day" = `rec_new_per_day` (planner's recommended new cards/day — the likely referent of "new 24"); "Last day to learn" = exam_date − 14 headroom days (`planExam/entry.ts:22,63`); **"Budget share" = that exam's slice of the shared daily minute budget, prorated by card count** (`daily_minutes = round(budgetSec/60)` where `budgetSec = daily_minutes*60*(exam.total_cards/totalCardsAll)`, `planExam/entry.ts:25,69`); **"Total cards" = the user-entered number of cards to learn for that exam** (Planner form defaults 2000 INBDE / 800 TOEFL, `Planner.jsx:14-16`). The chart under it is the projected daily-minutes load curve from `projected_curve_json`.
- Other charts: Dashboard itself has no Recharts chart; the trend chart is on LearnerModel (§4c).

### 4c. "FK6 pathology", MasteryBars, MasteryTrendChart, CalibrationBin, ItemElo
- **FK/CC codes are INBDE blueprint axes, not free tags:** [FACT] `Concept.inbde_fk` is an integer 1–10 ("Foundation Knowledge" area) and `inbde_cc_section` ∈ {DTP, OHM, PP} ("Clinical Content" sections) (`Concept.jsonc`). Display names are hardcoded: `FK_LABELS = {…6:"FK6 Pathology"…}` and `CC_LABELS = {DTP:"Diagnosis & Tx Planning", OHM:"Oral Health Management", PP:"Practice & Profession"}` (`LearnerModel.jsx:8-9`, duplicated in `MasteryTrendChart.jsx:5-6`). The page subtitle states the intent: "BKT mastery per concept · Elo challenge calibration · aggregated to the INBDE remediation-report axes (10 FK + 3 CC sections)" (`LearnerModel.jsx:63`). A finer `inbde_cc` 1–56 field exists on the entity but is used nowhere.
- **MasteryBars** (`components/learner/MasteryBars.jsx`): mean of `LearnerState.bkt_p_mastery` across the concepts in each group (grouping in `LearnerModel.jsx:11-25`), sorted weakest-first; bar color green ≥60%, yellow ≥40%, else red; the `· N` suffix is concept count. The 4th panel "Weakest Concepts" is the 12 lowest individual concepts.
- **MasteryTrendChart:** one line per FK area or CC section over `MasteryHistory` rows (written only by pipeline stage 5); shows a placeholder until ≥2 distinct run dates exist (`MasteryTrendChart.jsx:53-56`).
- **CalibrationBin semantics:** per run_date and predicted-retrievability decile: n, actual recall rate, Brier, logloss (`CalibrationBin.jsonc`; computed `runPipeline/entry.ts:26-50`). [FACT] Rendered by **no page** (grep `CalibrationBin` in `src/` → zero).
- **ItemElo semantics:** per-card difficulty `elo_b` (± `elo_b_sd`), updated after each review against the mean concept θ (`submitReview/entry.ts:128-137`); seeded from `Card.difficulty_seed`. [FACT] Rendered by no page and used by no selector (grep → zero in `src/`).

### 4d. Card creation flow
- **CardForm** (`components/library/CardForm.jsx`): manual authoring — prompt, expected answer, mcq/free_text, Bloom level, optional INBDE "Patient Box" vignette, one concept tag (find-or-create `Concept` by exam_code+name, `:24-27`); for MCQ, 4 options with radio-selected correct answer and per-wrong-option `misconception_tag` inputs (`:69-77`); creates Card + ItemOptions + a CardState due immediately (`:41`).
- **GenerateCardsDialog → generateCards:** dialog collects topic, count (1–15), optional grounding notes (`GenerateCardsDialog.jsx:34-36`) and invokes the backend fn. `generateCards/entry.ts` calls **`base44.integrations.Core.InvokeLLM`** (the Base44 platform's built-in LLM integration — no model name or selection anywhere) with a prompt encoding the Wozniak minimum-information principle + NBME item-writing rules, requiring a misconception_tag on every wrong MCQ option and original (non-copyrighted) items (`:13-25`), with a strict response JSON schema (`:26-57`). It find-or-creates one Concept named after the topic (optionally stamped with `inbde_fk`/`inbde_cc_section` params, `:60-71` — note the UI dialog never sends those two params), then creates Cards (`source_ref:'ai_generated'`), ItemOptions (wrong options default tag `'unspecified'`), and CardStates due immediately (`:73-100`).
- **gradeTextAnswer:** LLM grader (also `Core.InvokeLLM`) returning `score` 0–1, FSRS `rating` 1–4 with an explicit rubric, `rationale`, `error_tags` (`gradeTextAnswer/entry.ts:11-30`); writes nothing. In `Review.jsx` the result is displayed ("AI suggests rating: N", `:144`); the **user still clicks the final 1–4 button**; the AI's contribution to the stored record is `correct` (from `aiGrade.correct`) and `grade_source:'llm'` (`Review.jsx:56-57`). [SIGNAL] `error_tags` are displayed but never persisted — free-text misconceptions never reach the Weakness diagnoser (which only sees MCQ `chosen_option_id`).
- **VoiceRecorder / VoiceQueue:** MediaRecorder → webm blob → `base44.integrations.Core.UploadPrivateFile` → `VoiceQueue.create({status:'queued'})` (`VoiceRecorder.jsx:16-28`). Dashboard counts queued items into today's planned units and shows "N voice answers queued for overnight transcription + grading" (`Dashboard.jsx:22,28,118-120`). [FACT] **No code in this repo transcribes or grades them** (grep `VoiceQueue` in `base44/functions/` → zero); the comment delegates to "the nightly Worker" (`VoiceRecorder.jsx:5`). VoiceQueue rows would sit in `queued` forever as shipped.

---

## 5. WHAT IS ABSENT (grep-verified)

Each item: [FACT] based on the cited empty search over `src/` + `base44/` (and `package.json` where relevant).

1. **AnkiConnect / Anki export — settings stub only.** `grep -rni "anki"` hits only the `Settings` entity fields (`anki_mode`, `anki_endpoint`) and the AppSettings form (`AppSettings.jsx:83-92`, option labels ".apkg export (genanki, via Worker)" / "AnkiConnect live"). `grep -rniE "apkg|genanki|createObjectURL|saveAs|FileSaver|writeFile"` → only that same option label. **No export code exists.**
2. **Google Calendar:** `grep -rniE "google.?calendar|gcal|googleapis"` → zero hits.
3. **WhatsApp / WAHA:** `grep -rniE "whatsapp|waha"` → zero hits.
4. **MCP server:** `grep -rniE "\bmcp\b|modelcontextprotocol"` (src, base44, package.json) → zero hits.
5. **Knowledge-graph view:** `Concept` and `ConceptEdge` entities **do exist** (`base44/entities/Concept.jsonc`, `ConceptEdge.jsonc` with prereq/related/part_of edge types), and ConceptEdge is *read* once, in the pipeline's prereq check (`runPipeline/entry.ts:84-89`). But `grep -rn "ConceptEdge" src/` → zero, and no graph library is installed (`grep -rniE "cytoscape|vis-network|react-flow|reactflow|d3-force|force-graph|sigma"` in src + package.json → zero). **No page renders any graph, and no code ever creates a ConceptEdge row.**
6. **Local file save/export of cards/notes/graphs:** no download/object-URL/file-writer code anywhere (grep above, item 1).
7. **Upload folder / document ingestion:** the only upload is the single voice-answer `UploadPrivateFile` in `VoiceRecorder.jsx:21`. No document/PDF/folder ingestion path exists (`grep -rn "Upload" src/ base44/functions/` → that one hit).
8. **Ollama / user-selectable LLM:** `grep -rniE "ollama|gpt-4|claude|model"`-style searches → zero model names; both LLM calls are `base44.integrations.Core.InvokeLLM` with no model parameter (`generateCards/entry.ts:13`, `gradeTextAnswer/entry.ts:11`). The only engine choice in Settings is `stt_engine` (local_whisper/cloud_openai/cloud_azure) — itself a "deferred config flag" per its own hint (`AppSettings.jsx:61`) with no consumer.
9. **Hover tooltips / guide layer:** the shadcn `ui/tooltip.jsx` primitive exists but is imported only by the unused `ui/sidebar.jsx` (`grep -rn "ui/tooltip" src/` → 1 hit in sidebar.jsx). The only `<Tooltip>` elements are Recharts chart tooltips (`Planner.jsx:107`, `MasteryTrendChart.jsx:63`). No `title=` hover hints on app components, no onboarding/guide layer of any kind.
10. **Dashboard date-click day-detail view:** `DayBox` has no onClick; the sole onClick in `DayBoxCalendar.jsx` is the week/month toggle (`:63`). Clicking a day does nothing.
11. **Adjustable review-frequency setting:** no "review frequency"/interval-modifier setting exists (`grep -rniE "frequency|interval"` in AppSettings + Settings.jsonc → zero). **What IS tweakable today** (`AppSettings.jsx` + `Settings.jsonc`): `desired_retention` slider 0.70–0.97 (the single FSRS workload knob — this is the closest thing to review frequency), `daily_minutes`, `new_per_day`, `w_source` (default vs optimized), `stt_engine`, `anki_mode`/`anki_endpoint`, `streak_slack_tokens`, `burnout_threshold`. Entity-only fields with no UI: `w_optimized`, `w_version`, `day_done_threshold` (used by Dashboard streak math). [FACT] `burnout_threshold` is stored but enforced nowhere (grep `burnout` → Settings UI only).

---

## 6. RISKS / DEBT (fragile for planned extensions)

1. **No scheduler exists.** Everything named "nightly" is a Dashboard button. Any extension that assumes recurring analysis (weakness refresh, calendar sync, Anki sync, VoiceQueue draining) needs a real cron/trigger first — Base44 config has none (`base44/config.jsonc` is site-build only).
2. **Phantom worker dependency.** Comments in three places (`runPipeline/entry.ts:3-5`, `VoiceRecorder.jsx:5`, `AppSettings.jsx:51,61,86`) delegate STT, FSRS w-optimizer re-fit, and .apkg export to a "home-lab Worker" that is not in the repo. VoiceQueue rows and `w_source:'optimized'` are dead ends as shipped.
3. **Hard 500-row query caps with silent truncation:** revlogs/day (`runPipeline/entry.ts:23`), concepts and learner states (`:138-139`), due CardStates (`:125`, `Dashboard.jsx:18`), MasteryHistory chart rows (`MasteryTrendChart.jsx:14`). At the owner's own planned scale (2000+ cards) mature-deck due counts and revlog volume will cross these; stats and diagnosis silently under-count. [ESTIMATE, medium-high given Planner default 2000 INBDE cards.]
4. **N+1 query patterns** in `runPipeline` (per-miss `Card.get`, per-concept `LearnerState.filter`, per-edge prereq lookups) and `submitReview` (per-concept filter+update) — latency and rate-limit risk grows linearly with data. [METHOD to confirm limits: load-test against a Base44 dev backend.]
5. **UTC/local timezone mismatch.** All day keys are `toISOString().slice(0,10)` (UTC) while the calendar builds local `Date`s (`DayBoxCalendar.jsx:34-51`, `Dashboard.jsx:16,48-58`); for a US-Pacific user, evening reviews land on "tomorrow's" plan and DayBox day labels can shift. [ESTIMATE, high confidence for negative-UTC-offset users.]
6. **Dashboard writes on read:** `load()` creates/updates today's DailyPlan (`Dashboard.jsx:32-42`), racing with `submitReview`'s DailyPlan update — last-writer-wins on `planned_units`/`daybox_state`.
7. **Review queue ignores the new-card budget:** `Review.jsx:30` takes the first 30 due CardStates in due order, mixing new and review cards with no `new_per_day` enforcement (grep `new_per_day` in Review.jsx → zero) — the Dashboard's "New Cards" allowance is cosmetic.
8. **Weakness lifecycle debt:** duplicates accumulate across days (delete is same-run_date-only, `runPipeline/entry.ts:53`); same-day rerun wipes "resolved" marks; `probing` status and `probe_card_ids` are write-only; `missing_prereq` blocking is prose. Any "close the remediation loop" extension must define dedup + enforcement.
9. **Learner model is display-only.** BKT/Elo/ItemElo/CalibrationBin feed no scheduling or item-selection decision; wiring them in (adaptive queue, Elo-matched probes) is greenfield, not a tweak.
10. **LLM trust surface:** `submitReview` accepts client-supplied `correct`/`rating` unvalidated; `gradeTextAnswer.error_tags` are dropped (never persisted), so free-text/voice misconceptions can never reach the diagnoser without a schema change (e.g., persisting tags on Revlog).
11. **Unused-entity drift:** `Exam`, `Concept.inbde_cc`, `Card.kind` variants (`skill_drill`, `adaptive_module_sim`), `Card.quality_flag`, `DailyPlan.per_exam_alloc_json`, `SystemMetrics.*_cost_usd` are schema promises with no code behind them — extensions should either implement or prune them to avoid false affordances.
12. **planExam is a heuristic, not FSRS:** the ×2.3 interval ladder with clamped retention scaling (`planExam/entry.ts:11-18`) will diverge from actual FSRS-simulated load; feasibility verdicts ("go/tight/no_go") inherit that error. [METHOD: replace with ts-fsrs simulation over real CardStates to validate.]
