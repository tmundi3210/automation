# SPECIALIST_RECON.md — roster archive of the 9 study_system specialists + loop doctrine + app-surface map + gaps

_Recon for the Base44 study-app planning task. Sources read in full: `/home/user/automation/FACTORY/study_system/specialists/{contenteng,dental,engage,learner,qcraft,sched,sysloops,toefl,voice}/*.specialist.json`, `/home/user/automation/FACTORY/study_system/BUILD_PLAN.md`, `/home/user/automation/FACTORY/SELF_LOOP.md`. Every card below is [FACT — read from the named specialist file] unless tagged otherwise. This file creates nothing; it only records what exists. PLANNING ONLY._

Roster shape [FACT — BUILD_PLAN.md]: 9 specialists, 24 KBs (six specialists carry 3 KBs; engage, voice, sysloops carry 2 — the repo-precedent minimum). All KB paths below are absolute in each spec's `grounded_in_kbs`.

---

## 1. sched — `sched_sr_planner`
**Mandate:** Scheduling & spaced repetition: choose/tune/implement SM-2 / classic-Anki / FSRS-6 schedulers as an explicit `(grade, elapsed) -> (next state, next interval)` function, ship Anki interop, and back-plan deadline timelines to a feasibility go/no-go.
**KBs:** `kb_sr_algorithms.kb.json`, `kb_scheduler_design.kb.json`, `kb_timeline_planning.kb.json`.
**Strongest invariants / escalation triggers:**
- Elapsed time is a MANDATORY scheduler input; grade vocab normalized to 1–4 internally, SM-2's 0–5 reconciled ONLY at explicit import boundaries (silent equating = refused category error).
- Desired retention is the single workload knob on a U-shaped curve; never set below CMRR; warn >0.90, overwhelming >0.97.
- Revlog-shape logging from day one (timestamp, item, grade 1–4, interval, last_interval, duration≤60000ms, type); fitted FSRS w-vector [UNKNOWN] until history exists — published FSRS-6 defaults until then; fuzz is mandatory and non-disable-able.
- Never quotes days-to-mastery formulas, pass probabilities, or exam-day recall percentages (defer → simulation/measurement); live-Anki behavior is [UNKNOWN] (Anki not installed); canonical supermemo.com SM-2 page unreachable → mirror-cross-match caveat carried.
- Routes out: card authoring (contenteng), sync-server/SQLite internals ("sync/infra specialist" — NOTE: no such specialist exists in this roster), subject pedagogy.
**App features it can reason about:** the review/scheduling engine, retention/workload settings UI, review logging schema, backlog draining, daily new-card caps, exam-countdown planner, multi-exam budget split, .apkg/AnkiConnect/genanki import-export, load forecasting inputs.

## 2. qcraft — `qcraft_item_strategist`
**Mandate:** Question craft & answering for INBDE + TOEFL iBT (2026 + legacy): classify items exam→format→stimulus, author defensible one-best-answer items to NBME/JCNDE canon with Bloom/blueprint metadata and CTT/IRT calibration, and drive format-keyed answering/pacing playbooks.
**KBs:** `kb_item_typology.kb.json`, `kb_item_writing.kb.json`, `kb_answering_strategy.kb.json`.
**Strongest invariants / escalation triggers:**
- Classify exam-then-format-then-stimulus BEFORE any tactic; NEVER apply a format's tactic to an exam that lacks it (no select-3-of-6 handling on INBDE; no legacy integrated/essay templates on live 2026 TOEFL).
- Authoring ship-gate: cover-the-options passes, homogeneous flaw-free options, common-error distractors, no EXCEPT/NOT lead-ins, both flaw families purged.
- Psychometric bands (p 0.30–0.70, r_pb ≥0.20, D) are published authoring TARGETS, not this bank's measured statistics, until a real response matrix exists; negative r_pb or a dead distractor → quarantine the item.
- Does NOT certify subject-matter content truth (defer to subject authority); emits irt_b to the scheduler but does not own difficulty-to-ability matching; per-item time budgets always [ESTIMATE].
**App features it can reason about:** question bank data model & per-item metadata schema (exam, blueprint axes, format, bloom_level, stimulus, learning_objective_id, source_ref, p_value/r_pb/irt_b), item-authoring/review pipeline, item quarantine, answering-mode UX (stem-first, predict-then-eliminate, flag-and-move), Patient-Box/case rendering semantics, per-item timers, TOEFL productive-task note templates.

## 3. dental — `inbde_fmg_advisor`
**Mandate:** INBDE content & foreign-trained (FMG) US dental licensure pathway: exam identity/format/scoring, ECE+DENTPIN eligibility, the 10-FK × 56-CC blueprint study map, official resource shelf, and the CAAPID→CODA→ADEX/DLOSCE→state-board end-to-end path.
**KBs:** `kb_exam_and_pathway.kb.json`, `kb_foundation_knowledge.kb.json`, `kb_clinical_content.kb.json`.
**Strongest invariants / escalation triggers:**
- INBDE is knowledge-only; passing ≠ license; three independent gates resolved per state board via the ADA state map (never a national checklist).
- The full verbatim 56-CC enumeration is a named [UNKNOWN] blocker (JCNDE PDFs render as figure images; fetches 403'd) — partial per-group themes only, marked incomplete; no per-FK/CC item weight ever invented (coverage-based allocation).
- Fees / retake intervals / pass rates are [ESTIMATE]/[UNKNOWN] with verify-on-live-page flags; 49–99 scale with pass 75 (raised June 2024) is [FACT].
- Resource shelf is existence-only — no accuracy/quality/endorsement claims; fail-report FK/CC diagnostic feedback is the one official personal weighting signal.
**App features it can reason about:** the dental course/syllabus tree (FK×CC blueprint as the content taxonomy), study-plan prioritization by CC-group weights, remediation mapping from diagnostic feedback, milestone/pathway tracker (eligibility→exam→CAAPID→clinical→license), resource-library shelf for the dental track.

## 4. toefl — `toefl_from_zero_coach`
**Mandate:** TOEFL content from zero: fix the format era (2026 adaptive 1–6 bands vs 2023-25 legacy 0–120), map each 2026 task type → underlying skill → drillable practice form, and sequence the CEFR/vocabulary A1→target ladder with mock-based stage gates.
**KBs:** `kb_exam_structure.kb.json`, `kb_section_skills.kb.json`, `kb_from_zero_ladder.kb.json`.
**Strongest invariants / escalation triggers:**
- Format era is fixed FIRST on every question; any 0–120 / 4-Speaking-task / ~700-word-passage material is flagged legacy before its task list is trusted.
- Scoring = average of four 1.0–6.0 sections rounded to a half band; band 4.5≈90 is the ONLY ETS-firm conversion point; full CEFR cell table [UNKNOWN].
- NO study-hours-per-band figure is ever quoted (none exists from ETS); progress gated on demonstrated proficiency: vocabulary anchor met (Milton & Alexiou 2009 — the only citable vocab source) AND a full adaptive official-mock band read holding.
- Exact 2026 per-module item counts, section order, rubric weightings, fees, cutoffs: [ESTIMATE]/[UNKNOWN], deferred to ETS sources; targets are destination requirements, never exam pass marks.
**App features it can reason about:** the TOEFL course structure (task inventory R1–R3/L1–L4/S1–S2/W1–W3 as content types), per-task drill design, placement diagnostic → ladder placement, stage-gate progression logic, mock-test spine & readiness go/no-go, legacy-material flagging in a content library, band/CEFR display semantics.

## 5. contenteng — `contenteng_card_engineer`
**Mandate:** Content engineering, books → cards: decompose textbooks into units→terms→atomic claims→relation edges→multi-step items on the five-rung ladder mapped to revised Bloom, compile to Anki note types under Wozniak's 20 rules, lint against the bad-card taxonomy, gate on coverage+honesty.
**KBs:** `kb_book_decomposition.kb.json`, `kb_flashcard_quality.kb.json`, `kb_knowledge_layering.kb.json`.
**Strongest invariants / escalation triggers:**
- Comprehension gate: no card is minted from un-understood text; card questions capped at ~4 not-yet-chunked elements (Miller/Cowan chunk budget).
- Minimum-information atomicity governs the data layer; edge (relation) cards emit only after BOTH endpoint term-cards exist; inference chains capped at 2–3 steps, worked-solution-first.
- Every card must trace to a sourced claim; volatile facts date-stamped; an unsourced/undated-volatile card is BLOCKED, not shipped.
- Coverage oracle = the book's own end-of-chapter questions (held out, never carded); a Remember-only deck is no-go — must climb to Apply/Analyze; coverage misses loop back to extraction, not cosmetic edits.
- Stops at the card: scheduling (sched), OCR/PDF ingestion below TOC level, .apkg plumbing, and subject-matter truth are all out of scope; flat-spine + edge-table is the reversible default (full KST graph only if adaptive selection demands it).
**App features it can reason about:** the library/content-ingestion pipeline (book→deck), card/note data model & note-type choice (basic/cloze/image-occlusion/edge/case), deck lint & quality gates, the concept/prerequisite edge table (the app's knowledge-graph DATA), knowledge-layer progression, per-card source-trace display.

## 6. learner — `learner_model_adaptivity_strategist`
**Mandate:** Learner model & adaptivity for ONE user: per-concept mastery via BKT + Elo dual rating (IRT as frame), uncertainty-gated weakness flagging, the E1–E4 differential diagnosis (slip/misconception/missing-prerequisite/retrieval-decay) with prescribed fixes, and ~85%-band challenge calibration with a burnout guard.
**KBs:** `kb_knowledge_tracing.kb.json`, `kb_weakness_diagnosis.kb.json`, `kb_challenge_calibration.kb.json`.
**Strongest invariants / escalation triggers:**
- Uncertainty gate dominates: a weakness is diagnosed ONLY when (low P(L) or theta) AND low RD-style uncertainty; under-sampled concepts route to probes, never diagnosis.
- The acquisition-difficulty dial (~80–85% correct on new material) is NEVER conflated with the FSRS desired_retention dial (0.8–0.97); BKT mastery and FSRS recall are kept as two side-by-side estimates (BKT assumes no forgetting).
- A correct 4-option MCQ is down-weighted vs free recall (IRT guessing floor c≈0.25).
- Exact Glicko RD equations are [UNKNOWN] (glicko.net egress-blocked) — labeled RD-like stand-in only, never hard-coded; DKT/population estimators are explicitly NOT built for one user.
- Frustration/abandonment/burnout signals → cap challenge and escalate to human_review, never optimize against a distressed learner.
**App features it can reason about:** the mastery model & per-concept scores, weakness records ({concept, cause, evidence, fix}), probe queue, confusion matrix → interleaving policy, next-item selection, coasting detection & load shedding, success-band controller, rating-honesty checks — i.e. the app's adaptive brain behind "what should I study next and why am I weak here".

## 7. engage — `engage_progress_visual_architect`
**Mandate:** Engagement, gamification & progress visuals: turn the owner's calendar day-box brief (white box + day number, green download-bar fill, tick only at true 100%, 5% increments, week/month views, separate course-level metric) into an enforceable render spec, and gate streaks/mechanics against documented hazards.
**KBs:** `kb_progress_visuals.kb.json`, `kb_gamification_mechanics.kb.json` (2 KBs).
**Strongest invariants / escalation triggers:**
- Owner brief is BINDING verbatim product truth to render, never redesign; the three owner-unresolved points (rounding direction — floor recommended [ESTIMATE], "one in it" day-number reading, past-day freeze-vs-gray [UNKNOWN]) stay explicit build-time decisions.
- `pct = floor((tasks_done/tasks_total)*20)*5`, 21 fill states, display-100 ⇔ tick ⇔ done==total; the model is the single source of truth — numeral, fill and tick can never disagree; tasks_total==0 is an explicit empty case.
- Day-box EFFORT vs course MASTERY: disjoint inputs, distinct labels, NEVER merged into one number.
- Informational feedback only: no points/prizes/currency economy (overjustification), no fabricated/illusionary progress; any streak ships with a pre-equipped freeze/repair + a secondary long-horizon metric a single miss cannot zero.
- Non-punitive scheduler contract: no engagement mechanic may change what the SRS surfaces or make the backlog punitive; exact styling (hex/px/animation) is routed OUT of scope.
**App features it can reason about:** the dashboard/calendar (day-box grid, week+month views), progress bars & tick semantics, course-progress display, streak/freeze mechanics, future-load forecast layer, habit-loop cueing (notification-as-cue framing), engagement go/no-go gates.

## 8. voice — `voice_capture_grader`
**Mandate:** Multimodal capture & voice answering: spoken answers in any language, saved-and-queued → nightly batch transcribe (one TranscriberInterface; LocalWhisper default, CloudSTT deferred by owner) → LLM rubric-grades CONTENT → write-back to learner model with audio+transcript+grade audit linkage; same path reused for voice notes.
**KBs:** `kb_capture_stt.kb.json`, `kb_queue_grading.kb.json` (2 KBs).
**Strongest invariants / escalation triggers:**
- Local-vs-cloud engine is owner-DEFERRED: a config flag behind one interface, never hard-decided; local default (audio never leaves the machine); cloud only behind explicit consent + fetched vendor retention terms (only OpenAI's terms citable; others [UNKNOWN]).
- Async record-now/analyse-later: 16 kHz mono file per answer + SQLite metadata row; queue state machine queued→transcribed→graded | failed(reason) — nothing silently dropped; no real-time ASR, no message broker at single-user scale.
- Grade CONTENT from the transcript, never pronunciation/fluency (except explicit TOEFL-speaking constructs); SpeechRater precedent cited, exact human/machine weighting [UNKNOWN].
- NEVER run speaker identification / voice embeddings (Article 9 biometric threshold); no cloud STT language count or per-language WER ever invented (Whisper = exactly 100 tokenizer languages [FACT], coverage ≠ uniform accuracy).
- Voice-note/flashcard CANDIDATES require human confirmation before entering the knowledge base.
**App features it can reason about:** the voice-answer recording UX contract, transcription queue & nightly worker, LLM grading output shape ({score, subscores, error notes, misconception tags}), learner-model write-back seam, audio-linked "why was I marked wrong" audit surface, voice-notes capture, STT privacy/consent posture.

## 9. sysloops — `sysloops_feedback_engineer`
**Mandate:** System feedback loops & nightly analysis: design/order/gate/audit the two loops that make the system improve ITSELF — the idempotent nightly batch pipeline (calibration → gated re-fit → skill replay → diagnosis → card-quality → challenge band → schedule → self-metrics → go/no-go) and the recurring competitor benchmark scan feeding an adopt/adapt/reject design ledger.
**KBs:** `kb_nightly_analysis.kb.json`, `kb_competitor_loops.kb.json` (2 KBs).
**Strongest invariants / escalation triggers:**
- Idempotency/append-only dominates convenience: every write is a dated snapshot joined by id; a step that can't be made idempotent is not shipped; late voice grades join their original attempt by id, never counted as new.
- Gated FSRS re-fit: ~1000+ reviews AND ~2–4 weeks since last fit [ESTIMATE — labeled design defaults], revert on post-fit degradation; calibration read as a multi-day TREND, never a single snapshot.
- Improve the system, never blame the learner: bad cards → rewrite queue without lowering mastery; a chronically ~60%-done plan is a bad PLAN to shrink; degraded mode (FSRS-due-reviews-only) always ships a valid plan when the pipeline is unhealthy.
- Self-metric governance: every metric needs a stored daily snapshot + a threshold + a named consumer (action/flag) or it is DELETED; the diagnosis-efficacy meta-loop adjusts the E1–E4 thresholds themselves when a fix class chronically fails.
- Competitor honesty: feature existence [FACT] from vendor docs, prices [ESTIMATE]/[UNKNOWN], no user count/revenue/pass rate asserted except marked vendor claims; mechanisms copied for evidenced function, never brand.
**App features it can reason about:** the nightly job/cron architecture, the app's self-monitoring scorecards, calibration dashboards (predicted-vs-observed retention), plan-of-record assembly, degraded-mode behavior, competitor-driven feature backlog (adopt/adapt/reject ledger), the loop-improves-loop trigger wiring.

---

## Loop doctrine: what SELF_LOOP.md + sysloops offer

Two DISTINCT loops exist in the docs — one improves the specialist ARTIFACTS, one runs the LIVE APP nightly. Conflating them would be an error. [FACT — both files read]

**SELF_LOOP.md (FACTORY-level, artifact-audit loop).** Canonical protocol lives at `FACTORY/sweater_vertical/specialists/BRAIN/SELF_LOOP.md` (+ `self_improvement_loop.json` machine form); `FACTORY/SELF_LOOP.md` only parameterizes it per vertical via slot binding: target `*.specialist.json` files, slug map, optional scope doc, a rewritten 6–10-item "load-bearing joints" list (highest risk_if_wrong claims — R1 aims there), re-gate command (`bash FACTORY/build.sh <dir>`), output dir `<vertical>/analysis/self_loop/<slug>/`, three rollups, residue greps. Per specialist per round: four fresh context windows — R1 questioner (adversary) → R2 answerer (IS the specialist, spec/KBs only, FACT/ESTIMATE/UNKNOWN) → R3 judge (gap|contradiction|stale|overclaim|untagged|scope|ok; disposition apply_now|propose|watch; dry flag) → dry-stop or max 2 rounds (still-hot = NOT-CONVERGED, never silently truncated) → R4 synthesizer (LOOP_REPORT.md + feedback.json). Finalizer applies ONLY apply_now; orchestrator independently re-runs the gate before commit. Non-reparameterizable invariants: fresh context per role; honesty discipline factory-invariant; apply_now = high-confidence + in-scope + mechanical + stays ALL GREEN; agent self-reports are never acceptance; bounded rounds with an honest exit. Generation-level defects feed forward to the gate-failure lane (PLAN.md §3f), not the artifact loop.

**(a) With-time performance analysis of a live app.** The sysloops nightly-analysis KB is a ready blueprint [FACT — sysloops spec]: a once-per-night idempotent batch over four inputs (day's revlog, late AI-graded voice answers, concept graph, yesterday's plan) with a fixed step order ingest/reconcile → retention calibration (predicted-R deciles vs observed, Brier + log-loss — log-loss being the FSRS optimizer's own objective [FACT]) → gated FSRS re-fit → BKT+Elo replay with degeneracy guard → confusion matrix → E1–E4 weakness diagnosis (capped probes) → card-quality flags → challenge-band recalibration → budget-bounded interleaved schedule → self-metrics scorecard → append-only reports → go/no-go with degraded mode. "With-time" is achieved structurally: every metric is a DATED APPEND-ONLY SNAPSHOT read as a multi-day trend, never a point; seven self-scorecards (prediction calibration, skill-model sanity, card quality, schedule adherence, challenge calibration, diagnosis efficacy, pipeline health) each require snapshot+threshold+named-consumer or deletion. Critical honesty carry: NO observed run of this system exists — every threshold/cadence/band is a labeled design default [ESTIMATE], hardened only by real runs; the Glicko RD equations are [UNKNOWN] until fetched.

**(b) An in-app managing agent (the owner's Base44 agent).** [ESTIMATE — design reading of the two docs; medium-high confidence] The doctrine supplies the agent's duties and constraints but NOT its runtime:
1. **Nightly-run steward** (from sysloops): trigger/verify the pipeline, enforce the step order and idempotency, declare go/no-go, and fall back to degraded FSRS-only mode so a valid plan always ships — "a valid degraded plan always beats a rich broken one".
2. **Diagnostician** (from sysloops): govern the self-metrics under snapshot+threshold+named-consumer; run the diagnosis-efficacy meta-loop (did theta/P(L) recover within K days of a fix; chronically failing fix class → adjust the diagnosis thresholds themselves — the loop that improves the loop); treat plan under-completion as a plan defect to shrink, never learner blame.
3. **Router/escalator**: every specialist declares boundaries + defer/flag/human_review triggers — collectively a ready-made routing table for a managing agent deciding which "brain" a question or anomaly belongs to, and what must go to the human owner (e.g. owner-unresolved UI points, burnout signals, cloud-STT consent, the deferred STT engine choice).
4. **Periodic spec auditor** (from SELF_LOOP.md): the agent can run the R1–R4 fresh-context loop against the specialist prompts it embeds — but the invariants bind it hard: fresh context per role (separate LLM invocations with disjoint inputs), only apply_now self-applies, an independent re-gate follows, and NOT-CONVERGED is flagged rather than truncated. An autonomous agent may therefore self-apply only mechanical, gate-green fixes; everything else goes to a propose backlog for the human.
5. **Benchmark scanner** (from sysloops): scheduled + event-driven competitor re-scans (competitor change, new entrant, or one of the documented faults surfacing in OUR OWN self-metrics) feeding the adopt/adapt/reject ledger.
**What the doctrine does NOT provide** [FACT — absent from all files read]: any Base44 platform/API specifics, agent runtime or orchestration framework, LLM prompt/model/cost ops for the grader or the agent itself, or web-app telemetry instrumentation (page analytics, error tracking, session metrics beyond the study revlog). Those are gaps (below).

---

## Preliminary map: app surface → covering specialist

| App surface | Covering specialist(s) | Coverage note |
|---|---|---|
| Planner (exam-date back-planning, daily load, multi-exam split, feasibility go/no-go) | **sched** (kb_timeline_planning) | Strong; time budget as binding constraint; refuses days-to-mastery/pass-probability numbers |
| Review engine / SRS (intervals, retention knob, revlog, fuzz, backlog) | **sched** | Strong; live-Anki behavior [UNKNOWN] |
| Learner model (mastery, weakness diagnosis, next-item, challenge band) | **learner** (+ **sysloops** for the nightly replay) | Strong; single-user only; Glicko equations [UNKNOWN] |
| Review queue assembly / tomorrow's plan | **sysloops** (composition) + **learner** (selection) + **sched** (due reviews) | Seams explicitly declared in all three specs |
| Library / cards / content pipeline (book→deck, note types, deck lint, edge table) | **contenteng** | Strong; stops at the card; OCR/PDF ingestion below TOC level out of scope |
| Question bank / item authoring / answering UX / item metadata & IRT | **qcraft** | Strong; content truth not certified |
| Dashboard / calendar day-box / progress bars / streaks / gamification | **engage** | Strong on semantics & guardrails; exact styling (hex/px/animation) explicitly NOT owned |
| Course progress (mastery %) display | **engage** (display) consuming **learner** (numerator) | Effort-vs-mastery never merged |
| Voice capture / transcription / spoken-answer grading / voice notes | **voice** | Strong; engine choice owner-deferred; speaker-ID forbidden |
| Content domains: dental syllabus & pathway | **dental** | 56-CC enumeration [UNKNOWN]; blueprint-as-taxonomy usable |
| Content domains: TOEFL syllabus & ladder | **toefl** | Format-era gating; stage gates by demonstrated proficiency |
| Nightly analysis / self-metrics / calibration / degraded mode | **sysloops** | Strong; all thresholds labeled design defaults [ESTIMATE] |
| Competitor-driven feature backlog | **sysloops** (kb_competitor_loops) | adopt/adapt/reject ledger; WebSearch-grade evidence only |
| Anki import/export integration | **sched** | Only external integration any specialist owns |

Cross-seam sanity [FACT]: the specs cross-reference each other coherently (sched→contenteng for authoring; contenteng→sched for intervals; learner consumes contenteng's concept graph and FSRS's R/D/S; voice writes back to learner's mastery/weakness map; engage consumes learner's mastery numerator and holds the non-punitive contract over sched; sysloops schedules/invokes all of them without re-deriving their internals).

---

## Gaps — domains NO existing specialist covers (named only; nothing created)

1. **Knowledge-graph visualization.** contenteng owns the edge-table DATA and learner consumes the prerequisite graph, but no specialist owns graph rendering, layout, or interactive exploration UI. [FACT — absent]
2. **Color semantics / visual design system.** engage fixes the white/green/tick semantics and explicitly routes out "exact hex greens, pixel sizes, fonts, animation curves"; nobody owns the design system, theming, or color-meaning conventions beyond the day-box. [FACT — engage boundaries]
3. **External integrations & platform ops (beyond Anki).** No specialist covers Base44 platform specifics, auth/accounts, push-notification delivery infrastructure (engage names the notification only as a habit CUE), calendar sync, backups/export, or third-party APIs. sched's Anki interop is the sole owned integration; sched itself routes sync-server/storage questions to a "sync/infra specialist" that does not exist in the roster — a phantom route. [FACT]
4. **Media / image / 3D generation.** Image occlusion consumes images (contenteng) but no one owns generating diagrams/figures/media for cards, or any 3D/animation content. [FACT — absent]
5. **Document ingestion / OCR / PDF parsing.** Explicitly out of scope for contenteng ("below the level of parsing the TOC"); no owner. [FACT — contenteng boundaries]
6. **UX explainability & onboarding.** voice provides audio-level audit linkage ("why was I marked wrong") and sysloops carries an "onboarding cliff" fault rule, but no specialist owns explanation surfaces (why this card now, why this plan), onboarding flows, or general UI/UX. [FACT — absent]
7. **LLM/model-ops.** The LLM rubric grader (voice), LLM-graded answers (learner/sysloops), and note-structuring LLM are all CONSUMED; nobody owns model selection, prompt engineering, evaluation, drift monitoring, or cost control for those LLM calls — nor for the managing agent itself. [FACT — absent]
8. **Agent runtime / orchestration engineering.** SELF_LOOP.md + sysloops define loop PROTOCOL and duties (see doctrine section) but no specialist owns implementing an in-app agent: tool wiring, scheduling infra, context management, Base44 agent APIs. [FACT — absent]
9. **Web-app telemetry & performance engineering.** sysloops' self-metrics cover the LEARNING system (calibration, schedule adherence, pipeline health); nobody covers app-level telemetry (page performance, client errors, session analytics) or software performance/reliability engineering. [FACT — absent]
10. **Subject-matter truth certification.** qcraft, contenteng, and voice all explicitly route "is this answer/claim actually true" to a subject-matter authority; dental/toefl cover exam structure and study maps, not item-level clinical/linguistic truth certification. Partially mitigated, not owned. [FACT — multiple boundaries]
11. **Multi-user, security & broader privacy.** Everything is deliberately single-user; voice owns the audio-privacy posture only. Accounts, permissions, data security at the app layer: no owner. [FACT — absent]
12. **Charts/analytics visualization beyond the day-box.** sysloops produces calibration tables and scorecards; engage owns only the day-box/course-bar/forecast. Rendering trend charts (e.g. predicted-vs-observed retention over time) has no owner. [FACT — absent]

[METHOD] to close any gap under house rules: name it in the L2 build plan, run L1 research, then the MAKE_A_SPECIALIST Path B pipeline (spec → kb_forge → kb_validator dense → distill → specialist_validator → build.sh ALL GREEN) — explicitly NOT done here per task constraints.
