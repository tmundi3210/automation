# SPECIALIST_MAP.md — which existing specialist governs each part of the Base44 study app

**What this document is.** A planning-only coverage map. It answers: "for every surface (screen or engine) of the live Base44 study app, which of the 9 already-built specialists is the authority?" — and then explains how to use the existing loop doctrine (sysloops + SELF_LOOP.md) to analyze the app *over time*, and where your wished-for "Base44 managing agent" fits. Nothing here builds anything.

**Terms, defined once:**
- **Specialist** — a gated, evidence-grounded expert prompt (`*.specialist.json`) with its own knowledge bases (KBs). Nine exist for the study system. [FACT — SPECIALIST_RECON]
- **App surface** — one functional area of the app (a page, or a backend engine behind a page).
- **Loop** — a recurring analyze→decide→act cycle. Two distinct kinds exist in your docs: the **nightly app loop** (sysloops: the app improving its own study plan) and the **artifact loop** (SELF_LOOP.md: auditing the specialist files themselves). Conflating them is an error. [FACT — both docs read in recon]
- **Honesty tags** — [FACT] verified in a named file/source; [ESTIMATE] inference with stated basis; [UNKNOWN] not verifiable here; [METHOD] how to find out; [SIGNAL] weak indicator.

Sources: SPECIALIST_RECON.md, APP_RECON.md, KG_RECON.md, EXTERNAL_RECON.md (all read in full or skimmed as directed, 2026-07-08).

---

## 1. Coverage map: app surface → governing specialist(s)

Every row is [FACT] — derived from the specialist mandates (SPECIALIST_RECON) matched against the actual app code (APP_RECON). "Governs" means: when a design or correctness question arises on that surface, that specialist's spec + KBs are the authority to consult.

| App surface (what's in the app today) | Governing specialist(s) | Why (one line) |
|---|---|---|
| **Planner** — `Planner.jsx` + `planExam` function: exam-date back-planning, budget share across INBDE/TOEFL, feasibility go/tight/no_go | **sched** | Its `kb_timeline_planning` owns deadline back-planning and feasibility verdicts; it also flags that the app's ×2.3 interval ladder is a heuristic that diverges from real FSRS simulation [FACT — APP_RECON risk 12]. |
| **Scheduling / SRS core** — `submitReview` FSRS-6 via ts-fsrs, desired-retention knob, revlog, due queue | **sched** | Spaced-repetition state machines, the retention-vs-workload dial, and revlog-shape logging are its core mandate. |
| **Learner model** — BKT + Elo per concept (`LearnerState`, `ItemElo`), MasteryBars, weakness list, E1–E4 diagnosis | **learner** (+ **sysloops** for the nightly replay step) | learner owns BKT/Elo dual rating, the uncertainty gate, and the E1–E4 differential diagnosis the pipeline implements; sysloops owns when/how it re-runs in batch. |
| **Review & grading** — `Review.jsx`, rating buttons, `gradeTextAnswer` AI grading, chosen-option capture | **sched** (interval outcome) + **learner** (mastery update) + **qcraft** (answering UX, MCQ semantics) + **voice** (rubric-grading contract) | Each owns one seam of a single review event; the seams are explicitly declared in their specs [FACT — cross-seam sanity in SPECIALIST_RECON]. |
| **Library / cards / content pipeline** — `LibraryPage`, `CardForm`, `generateCards` LLM authoring, Concept/ConceptEdge data | **contenteng** (card & edge-table quality) + **qcraft** (item-writing rules, misconception tags, metadata schema) | contenteng owns book→units→terms→claims→edges→cards and deck lint; qcraft owns defensible one-best-answer authoring — the `generateCards` prompt already encodes both traditions [FACT — APP_RECON §4d]. |
| **Dashboard / gamification** — StatsRow, DayBoxCalendar (white box → green 5% fill → tick), streak + slack tokens | **engage** | The day-box render spec in the app is engage's owner-brief-as-law contract verbatim (`floor(done/planned*20)`, tick only at 100%); streak-freeze ("slack tokens") matches its mandated forgiveness mechanic. |
| **Voice capture** — `VoiceRecorder` → `VoiceQueue` (queued, never drained in-repo) | **voice** | Record-now/analyze-later queue, STT engine as a deferred config flag, LLM rubric-grades content only — the app implements exactly its capture half; the worker half is absent [FACT — APP_RECON §4d]. |
| **Content domains** — INBDE FK×CC taxonomy (`Concept.inbde_fk`, FK6 Pathology labels); TOEFL track | **dental** (INBDE blueprint & pathway) + **toefl** (format-era, ladder, stage gates) | The app's FK/CC axes are the JCNDE blueprint dental owns; TOEFL course structure and band semantics are toefl's mandate. |
| **System loops** — `runPipeline` (calibration → diagnosis → plan → snapshots → self-metrics), SystemMetrics, CalibrationBin | **sysloops** | The pipeline's stage order is sysloops' nightly-analysis blueprint almost line-for-line; sysloops also owns its biggest defect — it is manual-only, no scheduler exists anywhere [FACT — APP_RECON §3b, grep-verified]. |
| **Anki interop** — Settings stubs `anki_mode`/`anki_endpoint` (no export code exists) | **sched** | The only external integration any existing specialist owns is Anki import/export. |

**Known holes in this map** [FACT — SPECIALIST_RECON gap list]: knowledge-graph *visualization*, color/design system beyond the day-box, external integrations beyond Anki, media/3D generation, document ingestion below TOC level, onboarding/explainability UI, LLM model-ops, and agent *runtime* engineering have **no governing specialist**. Those are exactly what SPECIALIST_LIST.md §C proposes (names only).

---

## 2. USE FOR WITH-TIME ANALYSIS — running the loop doctrine over the live app

"With-time analysis" = watching whether the app actually gets better as it accumulates data, instead of trusting a one-day snapshot. The doctrine for this already exists; nothing new needs inventing.

### 2a. The nightly analysis loop (sysloops governs)

[FACT — sysloops spec + APP_RECON] The app already contains ~70% of sysloops' blueprint as `runPipeline` (calibration bins with Brier/log-loss, E1–E4 weakness rows, tomorrow's plan, mastery snapshots, self-metrics). What the doctrine says must change to make it a real with-time loop:

1. **Make it actually nightly.** Today the "nightly" pipeline runs only when you press a Dashboard button; no cron/schedule exists anywhere in the repo [FACT — APP_RECON §3b]. Base44 backend functions can run on schedules ("Automations") [FACT-source: docs.base44.com backend-functions overview, via search snippet — verify in-app]. Wiring `runPipeline` to a schedule is the single highest-leverage change the doctrine demands.
2. **Read trends, never points.** Every sysloops metric is a dated append-only snapshot judged as a multi-day trend (e.g., calibration drift over ≥3 days), never a single day [FACT — sysloops invariant]. The app already keys by `run_date` and delete-then-recreates idempotently — the storage shape is right; only the *reading* discipline is missing.
3. **Snapshot + threshold + named consumer, or delete.** sysloops' metric-governance rule: a metric may exist only if it has (a) a stored daily snapshot, (b) a numeric threshold, and (c) a named consumer — a person, page, or code path that acts on it. [FACT — sysloops invariant] Applied to the app today: `CalibrationBin`, `SystemMetrics`, and `ItemElo` are computed but consumed by **no UI and no code** [FACT — APP_RECON §3d, grep-verified]. Under doctrine each must either gain a consumer (a chart, an alert, a re-fit trigger) or be deleted. This one rule is your with-time audit checklist.
4. **Close the two half-open loops.** (a) Calibration is measured but never corrects anything — the FSRS w-vector re-fit is delegated to an off-repo "home-lab Worker" that doesn't exist [FACT — APP_RECON risk 2]; sysloops' gated-re-fit rule (~1000+ reviews AND 2–4 weeks since last fit, revert on degradation — labeled design defaults [ESTIMATE]) is the ready spec for it. (b) Weakness rows carry "block new cards until prereq ≥ 0.6" as *prose no code enforces* [FACT — APP_RECON §4a]; sysloops + learner define the enforcement seam.
5. **Degraded mode.** If the pipeline is unhealthy, ship a plain FSRS-due-reviews-only plan rather than nothing — "a valid degraded plan always beats a rich broken one" [FACT — sysloops].
6. **Honesty carry-over:** no observed run of this loop exists yet; every threshold above is a labeled design default [ESTIMATE], to be hardened only by real nightly runs.

### 2b. The competitor benchmark loop (sysloops governs)

[FACT — sysloops `kb_competitor_loops`] A recurring scan (scheduled, plus event-driven when a competitor changes or one of our own self-metrics surfaces a documented fault) of competitor feature pages, feeding an **adopt / adapt / reject ledger**: feature existence is [FACT] from vendor docs; prices/user-counts are [ESTIMATE]/[UNKNOWN]; mechanisms are copied for evidenced function, never brand. EXTERNAL_RECON §8 (Anki/FSRS, INBDE Bootcamp, UWorld, Osmosis, RemNote, Obsidian graph view, Duolingo, Brilliant) is the ready-made **first scan input** for that ledger — it is raw material, not decisions.

### 2c. The artifact loop (SELF_LOOP.md governs)

Distinct from the app loop: SELF_LOOP.md audits the **specialist files themselves** on a cadence. Per specialist per round, four *fresh context windows*: R1 adversarial questioner → R2 answerer (arguing only from the spec/KBs, honesty-tagged) → R3 judge (gap/contradiction/stale/overclaim/…, disposition apply_now|propose|watch) → R4 synthesizer; max 2 rounds with an honest NOT-CONVERGED exit; only mechanical, gate-green `apply_now` fixes self-apply; an independent re-gate (`bash FACTORY/build.sh <dir>`) always follows. [FACT — SELF_LOOP.md via recon] Use it whenever the app's behavior reveals a specialist's claim was stale or wrong: the finding routes to the artifact loop, not to ad-hoc edits.

### 2d. The owner's "Base44 managing agent" — how the wish maps onto doctrine

Your wish: an in-app agent that manages the system. The doctrine already defines its **duties and constraints**; what it does not define is any runtime, and **no agent is built now** — this is a mapping, not a build. [ESTIMATE — design reading of sysloops + SELF_LOOP.md; medium-high confidence, per recon]

The correct architecture under house doctrine is an **orchestrator that injects a fresh agent per loop role** — separate LLM invocations with disjoint inputs for each role below — never one long-lived chat agent that plays every part (fresh-context-per-role is a non-negotiable SELF_LOOP invariant):

| Agent role (fresh context each) | Doctrine source | Duty |
|---|---|---|
| Nightly-run steward | sysloops | Trigger/verify the pipeline, enforce step order + idempotency, declare go/no-go, fall back to degraded mode so a valid plan always ships. |
| Diagnostician | sysloops | Govern metrics under snapshot+threshold+named-consumer; run the diagnosis-efficacy meta-loop (did mastery recover within K days of a fix? chronically failing fix class → adjust the E1–E4 thresholds themselves); treat a chronically ~60%-done plan as a bad *plan* to shrink, never learner blame. |
| Router / escalator | all 9 specs | Every specialist declares boundaries + defer/flag/human_review triggers — collectively a ready routing table: which "brain" a question belongs to, and what must go to *you* (burnout signals, cloud-STT consent, owner-unresolved UI points, the deferred STT engine choice). |
| Periodic spec auditor | SELF_LOOP.md | Run R1–R4 against the specialist prompts — but bound hard: only mechanical apply_now self-applies; everything else lands in a propose-backlog for you; independent re-gate follows; agent self-reports are never acceptance. |
| Benchmark scanner | sysloops | Scheduled + event-driven competitor re-scans feeding the adopt/adapt/reject ledger. |

**What the doctrine does NOT provide** [FACT — absent from all recon'd files]: Base44 agent-runtime specifics (their `base44/agents/` JSONC configs are documented only via search snippets [FACT-source: docs.base44.com — verify in-app]), LLM/prompt/cost ops for the agents themselves, or app-level telemetry. That runtime gap is assigned to a *proposed* specialist in SPECIALIST_LIST.md §C — proposed by name only, not created.

---

## 3. COMPETITORS / GAMIFICATION — who answers those questions

- **"Should we add streaks / leagues / XP / badges? Is our day-box right?"** → **engage** (exists). Its `kb_gamification_mechanics` gates every mechanic against documented hazards: informational feedback only, no points/prize economy (overjustification risk), no fabricated progress, every streak ships with a freeze/repair plus a long-horizon metric one miss can't zero, and no mechanic may make the SRS backlog punitive. [FACT — engage spec] The app's existing streak-with-slack-tokens already conforms; e.g., a Duolingo-style *league* would have to pass engage's gates before adoption.
- **"What do Anki, Bootcamp, UWorld, Osmosis, RemNote, Obsidian, Duolingo, Brilliant do, and what should we copy?"** → **sysloops** (exists), `kb_competitor_loops`: the recurring scan → adopt/adapt/reject ledger process (§2b above), with EXTERNAL_RECON §8 as the first scan's evidence. Mechanisms are adopted for evidenced function with honesty tags, never because a brand does it.
- No new specialist is needed for either question. [FACT — both mandates cover them]

---

*Planning document only. Nothing in this file creates, modifies, or authorizes building any specialist, agent, or app code.*
