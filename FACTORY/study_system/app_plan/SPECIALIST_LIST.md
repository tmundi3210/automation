# SPECIALIST_LIST.md — the complete specialist roster: existing, reused doctrine, and proposed

**What this document is.** The deliverable list you asked for: everything that already exists (nothing to create), the doctrine that gets reused as-is, and the *smallest honest set* of NEW specialists that would be needed to cover the app domains nothing currently governs. Section C is **names + scope only** — no specialist, KB, or code is created by this document.

Honesty tags: [FACT] verified in a named file/source · [ESTIMATE] inference with basis+confidence · [UNKNOWN] not verifiable here · [METHOD] how to obtain. Sources: SPECIALIST_RECON.md, APP_RECON.md, KG_RECON.md, EXTERNAL_RECON.md (2026-07-08).

---

## A. EXISTING — the 9 built, gated specialists (nothing to create)

[FACT — all 9 read from `/home/user/automation/FACTORY/study_system/specialists/*/`; 24 KBs total, per BUILD_PLAN.md]

1. **sched** (`sched_sr_planner`) — spaced-repetition scheduling (SM-2/Anki/FSRS-6), the desired-retention workload dial, revlog design, Anki interop, and exam-deadline back-planning with feasibility go/no-go.
2. **qcraft** (`qcraft_item_strategist`) — question craft for INBDE + TOEFL: item classification, defensible one-best-answer authoring to NBME/JCNDE canon, psychometric quality bands, and format-keyed answering/pacing playbooks.
3. **dental** (`inbde_fmg_advisor`) — INBDE exam identity, the 10-FK × 56-CC blueprint as content taxonomy, and the foreign-trained-dentist licensure pathway end to end.
4. **toefl** (`toefl_from_zero_coach`) — TOEFL 2026-format truth, task→skill→drill mapping, and the from-zero CEFR/vocabulary ladder with mock-based stage gates.
5. **contenteng** (`contenteng_card_engineer`) — books → units → terms → claims → relation edges → cards: the decomposition pipeline, card-quality lint, and the coverage+honesty ship gate. (Also holds the knowledge-graph *data* method: the edge table.) 
6. **learner** (`learner_model_adaptivity_strategist`) — per-concept mastery (BKT + Elo), uncertainty-gated weakness flagging, the E1–E4 diagnosis with prescribed fixes, and ~85%-band challenge calibration with a burnout guard.
7. **engage** (`engage_progress_visual_architect`) — the day-box calendar render spec (your brief as binding law), effort-vs-mastery separation, and hazard-gated gamification (streak freezes, no points economy).
8. **voice** (`voice_capture_grader`) — voice answers: record-now/analyze-later queue, one TranscriberInterface (local default, cloud deferred), LLM rubric-grading of content, audited write-back to the learner model.
9. **sysloops** (`sysloops_feedback_engineer`) — the two self-improvement loops: the idempotent nightly analysis pipeline (calibration → gated re-fit → diagnosis → plan → self-metrics → go/no-go) and the recurring competitor benchmark scan with an adopt/adapt/reject ledger.

## B. EXISTING-REUSED DOCTRINE (documents, not specialists — reused as-is)

1. **SELF_LOOP.md** [FACT — canonical at `FACTORY/sweater_vertical/specialists/BRAIN/SELF_LOOP.md`, parameterized per vertical by `FACTORY/SELF_LOOP.md`] — the artifact-audit loop: per specialist, four fresh-context roles (adversarial questioner → answerer → judge → synthesizer), bounded rounds with an honest NOT-CONVERGED exit, only mechanical gate-green fixes self-apply, independent re-gate before commit. Reused to audit both the existing 9 and any future specialists — and it is the role-injection pattern the future "managing agent" orchestrator must follow.
2. **sysloops loop structure** [FACT — sysloops spec] — the nightly-pipeline step order, idempotency/append-only rule, snapshot+threshold+named-consumer metric governance, degraded mode, and the competitor-ledger process. Reused as the governing blueprint for making the app's existing `runPipeline` a real scheduled loop (see SPECIALIST_MAP.md §2).

## C. PROPOSED NEW SPECIALISTS — names + scope only (NOTHING built here)

Proposal rule applied: propose only where **no** existing specialist covers the domain [FACT — each gap grep/read-verified in recon], merge aggressively for the smallest honest set, and keep each boundary domain-coherent like the existing roster. Each entry: name → 1-line scope → the academic field(s) whose fields→topics→subtopics enumeration would ground its dense KBs (the house KB-building method) → which of your asks it serves.

### C1. `kgraph` — knowledge-graph architect
- **Scope:** turn contenteng's edge table into a real, typed knowledge graph — node/edge schema, hub-node and prerequisite structure, when to graduate from flat-spine to full graph, plus medical/dental terminology linking (recognizing a term in card text, normalizing synonyms, attaching the definition that a hover would show).
- **Grounding fields:** knowledge representation & ontology engineering; learning sciences of graphs — concept mapping (Novak & Cañas 2008 [FACT — already cited in contenteng KBs]) and Knowledge Space Theory (Doignon & Falmagne 1985 [FACT — same]); biomedical informatics — terminology systems & entity linking (controlled vocabularies; exact source vocabularies to use are a research-time decision [METHOD: L1 research pass]).
- **Serves your asks:** the color-coded knowledge graph's *data and meaning* layer; medical terminology linking/hovers; the "books → fundamental units → graph" vision beyond what contenteng deliberately defers. (contenteng stops at the edge table and explicitly defers full KST [FACT — KG_RECON §2a]; nobody owns terminology linking [FACT — gap list].)

### C2. `viz` — visual-encoding & graph-view cartographer
- **Scope:** how everything is *drawn*: interactive graph rendering with semantic zoom (more detail as you zoom, honest abstraction as you zoom out), the color system that encodes meaning (e.g., high-yield intensity, mastery state) perceptually and colorblind-safely, and the analytics charts (calibration trends, mastery over time) beyond engage's day-box.
- **Grounding fields:** information visualization & graphical perception (Bertin's visual variables; Cleveland–McGill-tradition perception studies [ESTIMATE — general domain knowledge; exact works and editions to be source-verified during this specialist's L1 research pass]); cartographic generalization & level-of-detail (the map-making science behind semantic zoom); color science & color psychology (perceptual color spaces, palette design); graph drawing (layout algorithms, edge bundling).
- **Serves your asks:** the color-coded KG view with semantic zoom; high-yield intensity coding; charts for the pipeline's currently consumer-less metrics. (engage explicitly routes out exact styling; no specialist owns graph rendering or charts [FACT — gap list 1, 2, 12]; no graph library even exists in the app [FACT — APP_RECON §5.5].)

### C3. `extint` — external-integration & platform-ops engineer
- **Scope:** everything that crosses the app's boundary to another service: AnkiConnect (localhost-only bridge — a cloud app can never reach it directly [FACT — EXTERNAL_RECON]), Google Calendar/Tasks OAuth, WAHA WhatsApp messaging (optional, ban-risk flagged [SIGNAL]), MCP servers, Ollama Cloud endpoints/keys, Base44 platform ops (Deno backend functions, scheduled Automations, per-app Secrets, `base44/agents/` runtime wiring), plus LLM model-ops for all in-app LLM calls (model selection, cost tracking, evaluation, drift, fallback).
- **Grounding fields:** web API engineering & authorization (REST, OAuth 2.0, webhooks, secrets handling); distributed-systems reliability (idempotency, retries, queues — extending sysloops' rules across the network boundary); ML/LLM operations (model evaluation, cost & drift monitoring); plus vendor-doc KBs (Base44, AnkiConnect, Ollama, WAHA, Tripo, Google APIs — grounded in official docs, [ESTIMATE]/[UNKNOWN]-tagged where docs were only snippet-verified, per EXTERNAL_RECON).
- **Serves your asks:** Anki live sync, calendar/WhatsApp reminders, MCP servers, Ollama Cloud, and the *runtime* half of the managing agent (sysloops covers its duties/doctrine; nobody covers implementing it on Base44 [FACT — gap list 3, 7, 8]). Also retires sched's phantom route to a "sync/infra specialist" that doesn't exist [FACT — SPECIALIST_RECON].

### C4. `media` — instructional media & 3D flashcard producer
- **Scope:** generating card media pedagogically: diagrams/images for image-occlusion cards, and the image → Tripo → GLB → in-browser `<model-viewer>` pipeline for 3D anatomy flashcards — including when 3D actually helps learning vs when it's decoration.
- **Grounding fields:** multimedia learning science (Mayer's cognitive theory of multimedia learning and its design principles [ESTIMATE — general domain knowledge; exact citations to be source-verified during this specialist's L1 research pass]); anatomy education research (evidence on 3D models vs 2D for spatial anatomy); 3D asset pipelines for the web (glTF/GLB, PBR texturing, model conversion — grounded in Tripo/model-viewer official docs [FACT-source: EXTERNAL_RECON §7]).
- **Serves your asks:** 3D anatomy flashcards and generated card imagery. (Image occlusion *consumes* images in contenteng; nobody owns *producing* media [FACT — gap list 4].)

### C5. `uxguide` — explainability & onboarding designer
- **Scope:** making the app self-explaining: the tooltip ladder (hover hints → deeper "what is this number" explanations), the guide page, onboarding flow, and "why" surfaces (why this card now, why this weakness was diagnosed, why the plan shrank) — turning the specialists' internal reasoning into honest user-facing explanations.
- **Grounding fields:** human–computer interaction (progressive disclosure, onboarding patterns, tooltip/help-system design); explainable AI (explanation interfaces for algorithmic decisions like schedulers and diagnosers); instructional design / technical communication (plain-language explanation of technical state).
- **Serves your asks:** hover tooltips everywhere, the guide page, and understanding screens like "Open Weaknesses (E1–E4)" without reading code. (The app today has zero tooltips or guide layer [FACT — APP_RECON §5.9]; voice's audit trail and sysloops' onboarding-cliff rule touch this but nobody owns it [FACT — gap list 6].)

### Gaps noted but deliberately NOT proposed as specialists now
- **Document ingestion / OCR below TOC level** — real gap [FACT — contenteng boundary], but no current owner ask forces it; revisit when you actually feed PDFs in. 
- **App telemetry / performance engineering, multi-user security** — single-user app; premature [FACT — gap list 9, 11].
- **Subject-matter truth certification** — partially mitigated by dental/toefl + source-tracing rules; a per-claim clinical-truth certifier is out of proportion for now [FACT — gap list 10].
- **Agent doctrine** — NOT a gap: sysloops + SELF_LOOP.md already cover the managing agent's duties and constraints; only its Base44 runtime wiring is uncovered, and that folds into `extint` (C3).

Five proposed specialists is the honest minimum: each merges the recon's twelve raw gaps along real field boundaries (data-meaning / drawing / boundary-crossing / media-making / explaining), mirroring how the existing nine divide their world. [ESTIMATE — design judgment; medium-high confidence]

[METHOD] If approved, each would be created via the house pipeline only: name in the L2 build plan → L1 research → MAKE_A_SPECIALIST Path B (spec → kb_forge → kb_validator dense → distill → specialist_validator → `build.sh` ALL GREEN) — explicitly not done here.

---

**None of these are created yet — creation is a separate, owner-gated step.**
