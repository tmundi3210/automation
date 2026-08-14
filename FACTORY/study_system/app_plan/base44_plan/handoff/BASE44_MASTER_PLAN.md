# BASE44_MASTER_PLAN.md — paste this FIRST into the Base44 planner agent

**Standing directive (binding).** Base44's **planner agent** uses THIS file plus the specialist consultation table below as its top-level context. Base44's **build agent** takes ONE slice at a time from `BASE44_SLICES.md`, in order (Slice 5 may be pulled forward — see the slice map). Every slice ends with a **Base44 agent directive** block naming the specialist JSON file(s) to load as design authority: paste the named JSON(s) in full into the planning context, design strictly by their `role` + `decision_procedure`, treat their `escalation_triggers` as stop-and-ask rules, use their `validation_checklist` as that slice's acceptance test, and never act outside their `boundaries`. `BASE44_LOOPS.md` is the binding loops doctrine (load with Slice 8 and any pipeline change). `BASE44_APP_SPEC.json` is the machine-readable index of the same plan.

**Honesty contract (binding on all output).** Tags [FACT] / [FACT-source: URL] / [ESTIMATE] / [METHOD] / [UNKNOWN] / [SIGNAL] are load-bearing and must be preserved in any derived plan or UI copy where marked. Vendor/API facts come ONLY from `recon/EXTERNAL_RECON.md`; app-behavior facts ONLY from `recon/APP_RECON.md`. Never invent endpoints, model names, prices, or citations. An [UNKNOWN] is resolved by running its [METHOD] or asking the owner — never by guessing.

---

## 1. Current state (from APP_RECON, 2026-07-08)

1. A single-user, dark-themed React 18/Vite app exported from Base44 (app id `6a4d7164497f2266c1d3ac55`); pages Dashboard / Review / Library / Learner Model / Planner / Settings talk to 20 Base44 entities [FACT — entity directory listing, verified 2026-07-08] via `@base44/sdk` plus five Deno backend functions [FACT — APP_RECON §1].
2. Scheduling core is FSRS-6 via `ts-fsrs@5.2.1`; learner model is per-concept BKT + two-sided Elo; a manually-triggered "nightly" pipeline does recall calibration, E1–E4 weakness diagnosis, tomorrow's plan, mastery snapshots and self-metrics [FACT — APP_RECON §1].
3. Exactly one loop is genuinely closed (review → FSRS → due queue); BKT/Elo/ItemElo/CalibrationBin/SystemMetrics are display-only or consumed by nothing; weakness prescriptions are advice strings, not actions [FACT — APP_RECON §3d].
4. No scheduler of any kind exists — "nightly" runs only on the Dashboard button; VoiceQueue has no consumer; the FSRS optimizer re-fit was delegated to an off-repo "home-lab Worker" absent from the codebase [FACT — APP_RECON §3b, §4d, risk 2].
5. Structural defects shaping every slice: UTC day-keys vs local calendar (risk 5), silent 500-row query caps (risk 3), review queue ignores `new_per_day` (risk 7), Weakness duplicates + rerun-wiped resolutions (risk 8), three racing DailyPlan writers (risk 6) [FACT — APP_RECON].

## 2. Target architecture

After Slices 0–8 the app is: a **scheduled, idempotent, timezone-correct nightly Automation** (single plan writer, append-only snapshots, full pagination, degraded mode) feeding a **next-day-only detailed plan** with enforced new-card gating and evidence-based weakness auto-resolution; every surface **explains itself honestly** via a four-rung disclosure ladder and a Guide page; every model signal (BKT, Elo, calibration) has a **named consumer** or is deleted; external effects (Anki push, Calendar events, WhatsApp nudges) ride an **outbox + Secrets** reliability spine; coding agents attach through a **local MCP bridge** (read-only by default, journaled writes) and the app's LLM calls route through an **exactly-two-model Ollama Cloud picker** with loud fallback and cost visibility; concepts live in a **typed, provenance-carrying knowledge graph** with semantic zoom and abstaining hover definitions; card media passes an **earns-its-cost verdict and a human clinical gate**; and a **managing agent** stewards the loops — proposing, never silently applying — behind owner approval gates.

## 3. SPECIALIST CONSULTATION TABLE (all 14 — Base44 must load the named file(s) before designing in that domain)

Repo root: `/home/user/automation/`. Directive blocks cite repo-relative paths; absolute paths below. All 14 files verified on disk 2026-07-08 [FACT — directory listing].

| Code | Absolute path | Governs | Load when |
|---|---|---|---|
| sched | /home/user/automation/FACTORY/study_system/specialists/sched/sched.specialist.json | FSRS/spaced-repetition scheduling math, desired-retention dial, queue/plan assembly, revlog design, Anki interop doctrine, exam feasibility go/tight/no_go, refusal of invented projections | Slices 2, 3; any change to due semantics, plan budgets, or the retention dial |
| learner | /home/user/automation/FACTORY/study_system/specialists/learner/learner.specialist.json | BKT/Elo mastery estimation, uncertainty-gated weakness flagging, E1–E4 diagnosis + prescribed fixes, gating thresholds, challenge-band calibration | Slice 2; any mastery/diagnosis/threshold question anywhere |
| qcraft | /home/user/automation/FACTORY/study_system/specialists/qcraft/qcraft.specialist.json | Item quality, one-best-answer authoring canon, what qualifies as a probe item, psychometric bands | Slice 2 (probe items), Slice 4 (generated-card quality) |
| dental | /home/user/automation/FACTORY/study_system/specialists/dental/dental.specialist.json | INBDE identity, 10-FK × 56-CC blueprint taxonomy, FK/CC wording | Slice 1 (glossary), Slice 6 (FK grouping for roles) |
| toefl | /home/user/automation/FACTORY/study_system/specialists/toefl/toefl.specialist.json | TOEFL 2026 format truth, section/band semantics | Slice 1 (TOEFL glossary entries); any TOEFL-specific surface |
| contenteng | /home/user/automation/FACTORY/study_system/specialists/contenteng/contenteng.specialist.json | Books → units → terms → claims → edges → cards decomposition, card lint, coverage+honesty ship gate | Slice 4 (ingest); edge-triple semantics feeding Slice 6 |
| engage | /home/user/automation/FACTORY/study_system/specialists/engage/engage.specialist.json | Day-box calendar render law (owner brief = binding), streak/slack semantics, effort-vs-mastery separation, non-punitive gamification, nudge wording | Slices 1, 2, 4; anything rendered on the calendar or worded as motivation |
| voice | /home/user/automation/FACTORY/study_system/specialists/voice/voice.specialist.json | Voice capture queue state machine, transcriber deferral (local default, cloud consent-gated), rubric grading, human-confirmed candidates | Slice 7 (voice notes); any VoiceQueue consumer |
| sysloops | /home/user/automation/FACTORY/study_system/specialists/sysloops/sysloops.specialist.json | The nightly pipeline contract (idempotency, append-only, degraded mode, snapshot+threshold+consumer metrics), benchmark ledger, agent duties/doctrine | Slice 8 (+ BASE44_LOOPS.md + FACTORY/SELF_LOOP.md); Slice 2 pipeline changes; Slice 5 agent-exposure rules |
| kgraph | /home/user/automation/FACTORY/study_system/specialists/kgraph/kgraph.specialist.json | Graph schema/meaning: node/edge typing, integrity invariants, cycle-free prereq DAG, provenance/quarantine, terminology linking, gating semantics, msi import contract | Slice 6 §1; any ConceptEdge write path anywhere |
| viz | /home/user/automation/FACTORY/study_system/specialists/viz/viz.specialist.json | Graph drawing: semantic-zoom LOD ladder, channel-to-meaning color system (perceptual, CVD-safe, never color alone), layout, interaction, measured build-vs-adopt | Slice 6 §2; analytics charts beyond engage's day-box |
| extint | /home/user/automation/FACTORY/study_system/specialists/extint/extint.specialist.json | Everything crossing the app boundary: Base44 platform ops (Automations, Secrets, backend functions, agents runtime), AnkiConnect, Google OAuth/Calendar, WAHA, MCP, Ollama Cloud, Tripo transport, LLM model-ops (cost/drift/fallback), outbox/retry/idempotency | Slices 0, 3, 4, 5; transport in 7; runtime in 8 |
| media | /home/user/automation/FACTORY/study_system/specialists/media/media.specialist.json | When media earns its cost (skip/2D/3D verdicts), image roles + prompt discipline, clinical-check gate, Tripo mesh QC, web budget, provenance contract | Slice 7; any request to attach media to a card |
| uxguide | /home/user/automation/FACTORY/study_system/specialists/uxguide/uxguide.specialist.json | Explainability: four-rung disclosure ladder, Guide/glossary, WHY surfaces, honest wording rules, hover/tap accessibility contract | Slice 1; any user-facing explanatory copy in any slice |

Reused doctrine (document, not a specialist): `/home/user/automation/FACTORY/SELF_LOOP.md` — the bounded fresh-context audit protocol; loaded with sysloops for Slice 8's auditor duty [FACT — file exists].

## 4. Slice map with dependencies

```
Slice 0 Foundations (extint): scheduler, day-key fix, pagination, new_per_day, Secrets, outbox
  ├─► Slice 1 Explainability (uxguide) — zero hard deps; may run parallel to 0
  ├─► Slice 2 Planner dynamics (sched+learner+engage) — needs 0 (day-key, pagination, schedule)
  │     ├─► Slice 3 Anki + Calendar (extint, sched consult) — needs 0 (Secrets, outbox) + 2 (plan to announce)
  │     ├─► Slice 4 Export / ingest / WhatsApp (extint, contenteng) — needs 0; export benefits from all later entities (dynamic enumeration)
  │     ├─► Slice 5 MCP bridge + Ollama picker (extint) — OWNER'S TOP PRIORITY; needs only 2's stable data model — MAY RUN RIGHT AFTER 2, parallel to 3/4
  │     └─► Slice 6 Knowledge graph (kgraph+viz) — needs 0 (pagination); hands the gate spec to 2's consumers; 2's manual "A is base of B" UI must call 6's writeEdge once it exists
  ├─► Slice 7 Media/3D (media, extint transport, voice) — needs 5 (bridge) + 0 (Secrets, outbox)
  └─► Slice 8 Managing agent + loops (sysloops + SELF_LOOP) — LAST: supervises everything the earlier slices created; hard-needs 0's schedule
```

One numbering everywhere: these slice numbers are used identically in `BASE44_SLICES.md`, `BASE44_LOOPS.md` and `BASE44_APP_SPEC.json`. (FEATURE_PLAN phase → slice: P1→1, P2→2, P3→3, P4→4, P5→5, P6→6, P7→7, P8→8; Slice 0 extracts the platform prerequisites those phases shared.)

## 5. Cross-cutting facts (no slice may contradict these)

1. **No scheduler exists.** Everything labeled "nightly" runs only on the Dashboard button [FACT — APP_RECON §3b, grep-verified]. Base44 Automations can schedule backend functions [FACT-source: docs.base44.com backend-functions overview, via snippet — verify in-app]; Slice 0 wires it, Slice 8 hardens it. Until then no slice may assume a scheduled run.
2. **UTC/local day bug.** Day-keys are UTC `toISOString().slice(0,10)` while the calendar draws local days [FACT — APP_RECON risk 5]; for a west-of-UTC user, evening reviews land on "tomorrow." Slice 0 fixes it with a Settings `timezone` (IANA) field and one shared day-key rule; every date any slice shows uses that convention.
3. **Silent 500-row query caps** in pipeline and dashboard queries [FACT — APP_RECON risk 3]. At the planned 2000+ cards, stats, exports and graphs silently under-count. Slice 0 replaces every capped read with pagination-to-exhaustion; rows-scanned is recorded so truncation is observable.
4. **Weakness lifecycle flaws.** Duplicate open rows accumulate across days; same-day pipeline reruns wipe "resolved" marks; `probing`/`probe_card_ids` are write-only [FACT — APP_RECON risk 8, §4a]. Slice 2 fixes the lifecycle (one open row per (concept, cause), upsert, evidence-based auto-resolve, snooze); Slice 8's append-only `WeaknessEvent` rows make resolutions rerun-proof.
5. **Secrets pattern.** Every credential is a per-app Secret read server-side in a backend function [FACT-source: docs.base44.com/Integrations/Using-integrations, via snippet]; never in client code, never in the two-way-synced repo.

## 6. Loops overview (doctrine detail in BASE44_LOOPS.md)

- **Already closed (keep):** review → Revlog/CardState (FSRS) → due queue → next review [FACT — APP_RECON §3d].
- **Nightly analysis loop (Slice 0 schedules, Slice 2 extends, Slice 8 hardens):** one idempotent run per owner-local night — ingest/reconcile → retention calibration → gated re-fit decision → skill replay + degeneracy guard → weakness diagnosis → card-quality pass → tomorrow's plan → mastery snapshots → self-metrics + go/no-go — with a degraded mode that always ships a plain FSRS-due-only plan rather than a rich broken one.
- **Self-metrics rule:** every kept metric = snapshot + threshold + named consumer, or it is deleted. (CalibrationBin and SystemMetrics are computed today and consumed by nothing [FACT — APP_RECON §3d].)
- **Closed-loop fixes:** BKT → new-card gate; Elo → p≈0.85 acquisition/probe selection; elo_theta_sd (fixed to grow with inactivity) → well-measured-weakness rule; CalibrationBin trend → re-fit gate + health flag; error_tags persisted onto Revlog → E2 evidence (E-letter binding assumed enum order [ESTIMATE] until the Slice 1 code read confirms it). Plan-regression at three levels (concept gate re-applies; chronic <60% adherence → plan-shrink proposal; post-re-fit calibration degradation → auto-revert `w`).
- **Managing agent (Slice 8):** five duties — nightly steward, diagnostician (trend-not-point), router (the 14 specialists' escalation triggers are the routing table), bounded SELF_LOOP spec auditor (proposals only, max 2 rounds, honest NOT-CONVERGED), quarterly benchmark scanner (adopt/adapt/reject ledger). Shadow mode first; every write journaled; proposals never self-apply.

## 7. Safety posture

- **Secrets:** per-app Base44 Secrets only (slot names in BASE44_APP_SPEC.json); UI shows configured/not-configured, never values; rotate-on-exposure fires `security_alert`; the two-way GitHub sync means a secret in the repo is a leak [FACT-source: docs.base44.com GitHub-sync pages].
- **Approval gates (vocabulary):** `credential_add` (owner personally adds/replaces any secret) · `external_send` (first send to any new external surface + wording/trial review) · `spend` (anything costing money) · `host_public` (anything internet-reachable, e.g. WAHA exposure) · `data_export` (user data leaving the app as files or to a third-party API, incl. images to Tripo) · `security_alert` (suspected exposure → rotate + notify) · `schedule_enable` (owner enables any recurring Automation) · `write_unlock` (each MCP/agent write capability, one at a time) · plus per-slice sign-offs listed in each slice. Every external action in every slice names its gate.
- **Untrusted content:** ingested documents and LLM outputs are proposals, never direct writes (owner approves ingested cards; voice-note candidates need confirmation; LLM/unsourced graph edges enter quarantined and never gate); generated images/3D are `unverified` until the human clinical gate passes — no unverified asset ever attaches to a card; agent/MCP writes are journaled append-only and read-only by default; a retired/missing LLM model stops calls and asks the owner — never a silent substitute; no silent empty/fabricated LLM result may enter learning data.
- **Known risk signals:** WAHA automates WhatsApp Web unofficially — account-ban risk, opt-in only, off by default [SIGNAL]; two scheduling authorities after Anki push — pushed notes are labeled one-way-managed.

---

**Standing directive, restated:** planner agent = this file + the specialist table; build agent = one slice at a time from `BASE44_SLICES.md`, obeying each slice's directive block (load the named specialist JSONs, honor escalation triggers as stop-and-ask, use validation checklists as acceptance, stay inside boundaries), with `BASE44_LOOPS.md` binding for anything touching loops and `BASE44_APP_SPEC.json` as the machine index.

```
Base44 agent directive
> Before planning ANY slice, load the specialist file(s) named in that slice's directive block from the table above (paste the full JSON into your planning context). Design strictly by each specialist's role + decision_procedure; treat escalation_triggers as stop-and-ask rules and validation_checklists as acceptance tests. Do not act outside any specialist's boundaries. Preserve every [FACT]/[ESTIMATE]/[UNKNOWN] tag; resolve [UNKNOWN]s only via their [METHOD] or the owner — never by invention.
```
