# app_plan/ — planning package for the Base44 study app ("incrediblesmartplanpulse")

_The owner uploaded the app exported from Base44 (built earlier from the `study_system` specialists) and asked for a **planning-only** pass: analyze how it works, map it to the existing specialists, list (NOT create) the new specialists it would need, and lay out the features — Anki/Calendar/WhatsApp integrations, MCP + Ollama Cloud, local export, the knowledge-graph view, 3D flashcards, and the managing agent — as a step-by-step plan. **Nothing is built by this package: no code, no new specialist, no agent.** Every next step is owner-gated._

## Reading order

| File | What it answers |
|---|---|
| `APP_ANALYSIS.md` | What the app is and how it works; the loops it already has and the with-time verdict; **your questions answered from code** (E1–E4 weaknesses, "Mark Resolved", confidence %, FK6, the Dashboard/Planner numbers, how cards work); the verified list of what is absent today. |
| `SPECIALIST_MAP.md` | Which existing specialist governs each app surface; how to run the app's **with-time analysis** using sysloops + SELF_LOOP doctrine; how the wished "Base44 managing agent" maps onto that doctrine; where competitor/gamification questions get answered. |
| `SPECIALIST_LIST.md` | **The list you asked for**: A) the 9 existing specialists (nothing to create), B) reused loop doctrine, C) the 5 proposed NEW specialists — names + scope + grounding fields only (`kgraph`, `viz`, `extint`, `media`, `uxguide`) — **none created**. |
| `FEATURE_PLAN.md` | The plan, cropped into 8 independently-shippable phases with dependencies, specialists to consult, integration prerequisites ([FACT-source]-verified), risks, and an owner approval gate per phase. |
| `BASE44_CHANGE_PROMPT.md` | Your dictated change requests to the Base44 builder, rewritten clean and paste-ready (near-term UI changes only; ambiguous dictation kept as dictated and marked). |
| `KG_PIPELINE.md` | The book → fundamental-units → knowledge-graph plan: your separate `tmundi3210/msi` repo (the likely "USMLE KG" project), the decomposition method already encoded in `contenteng`, which books to run (sourced only), how the graph feeds the app's existing `Concept`/`ConceptEdge` entities, and the cleaned semantic-zoom/color-coding visualization spec. |
| `recon/` | The evidence files the plan cites (`APP_RECON`, `SPECIALIST_RECON`, `KG_RECON`, `EXTERNAL_RECON`) — kept so every `[FACT — …]` reference stays resolvable. |

## The honesty contract (binds this package)
Every load-bearing claim is tagged `[FACT] / [FACT-source: URL] / [ESTIMATE] / [METHOD] / [UNKNOWN] / [SIGNAL]`. App-behavior claims are code-verified against the uploaded zip; external API claims come only from official docs recorded in `recon/EXTERNAL_RECON.md`; anything unverifiable is `[UNKNOWN]` with a `[METHOD]` to obtain it. No statistics, endpoints, book titles, or product features are invented.

## Status
Planning complete; adversarially audited (2 findings, both fixed: two literature citations retagged `[ESTIMATE]`, the E1–E4 letter-mapping caveat added to the Base44 prompt). **Awaiting owner direction** on: which phase to start, and whether to approve building the 5 proposed specialists (`SPECIALIST_LIST.md §C`).
