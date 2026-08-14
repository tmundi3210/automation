# base44_plan/ — the specialist-directed Base44 build plan for the study app

**What this package is.** The detailed, paste-ready build plan for evolving the Base44 study app (INBDE/TOEFL adaptive study engine, app id `6a4d7164497f2266c1d3ac55`) through nine slices (0–8). It was assembled from six specialist-authored plan sections, grounded exclusively in `../recon/APP_RECON.md` (app behavior) and `../recon/EXTERNAL_RECON.md` (vendor facts), consistent with `../FEATURE_PLAN.md` (the 8-phase runway). It contains **no code** — Base44's own plan+build agents do the implementation; this package tells them exactly what to build, in what order, under which design authority, and behind which owner approval gates.

## Files and paste order

1. **`BASE44_MASTER_PLAN.md`** — paste FIRST into Base44's planner agent. Current state, target architecture, the 14-specialist consultation table, the slice map with dependencies, cross-cutting facts, loops overview, safety posture, and the standing directive.
2. **`BASE44_SLICES.md`** — the build agent takes ONE slice at a time, in order (Slice 5 — MCP + Ollama, the owner's top priority — may run right after Slice 2). Each slice: goal → what to build → acceptance checks → owner approval gate → its Base44 agent directive block.
3. **`BASE44_LOOPS.md`** — the standalone loops doctrine (nightly Automation contract, managing agent duties, closed-loop fixes, SELF_LOOP audit pattern). Load alongside Slice 8 and any pipeline change.
4. **`BASE44_APP_SPEC.json`** — the machine-readable index: entities + per-slice deltas, screens, settings controls, backend jobs, integration slots + secret names, approval-gate enum, loops with cadence/consumer, and the full specialist registry. Valid strict JSON.

## The one rule

Every slice ends with a fenced **"Base44 agent directive"** block naming the specialist JSON file(s) on disk (under `/home/user/automation/FACTORY/study_system/specialists/`) that are the **design authority** for that slice. Base44's agent must paste the named JSON(s) in full into its planning context and then: design strictly by the specialist's `role` + `decision_procedure`; treat its `escalation_triggers` as stop-and-ask rules (when one fires, stop and ask the owner or run the named [METHOD] — do not improvise); use its `validation_checklist` as the slice's acceptance test; and never act outside its `boundaries`. A slice built without loading its specialist(s) is out of contract.

## Honesty contract

Claims carry tags: **[FACT]** (verified in a named file) · **[FACT-source: URL]** (verified external source) · **[ESTIMATE]** (inference, basis stated) · **[METHOD]** (how to find out) · **[UNKNOWN]** (not verifiable now) · **[SIGNAL]** (weak indicator). These tags are load-bearing and survive into derived plans and, where marked, into user-facing copy. Vendor/API facts come only from `../recon/EXTERNAL_RECON.md`; app-behavior facts only from `../recon/APP_RECON.md`. No endpoint, model name, price, or citation may be invented; an [UNKNOWN] is resolved by its [METHOD] or by asking the owner — never by guessing. Design defaults (thresholds, cadences, budgets) are labeled [ESTIMATE] until real runs harden them.

## Approval gates summary

Gate vocabulary (defined in the master plan §7; machine enum in the JSON spec): `credential_add` · `external_send` · `spend` · `host_public` · `data_export` · `security_alert` · `schedule_enable` · `write_unlock` — every external action in every slice names its gate. Per-slice owner gates:

| Slice | Owner must approve |
|---|---|
| 0 | schedule enablement (daily Automation); any credential slot filled |
| 1 | Guide tone + depth ("this is how I want the app to talk to me"); FK/CC glossary wording before it is canon |
| 2 | time-window split rule; gating threshold (default 0.6); auto-resolution evidence rule before Mark-Resolved is removed |
| 3 | Google OAuth consent (personally); first calendar event + trial-week review; one-way Anki push direction before first `addNotes` |
| 4 | export round-trip inspection; first 20 ingested cards by hand before bulk; WhatsApp explicit ban-risk opt-in + WAHA credential/exposure/spend gates |
| 5 | OLLAMA_API_KEY + MCP_BRIDGE_TOKEN; one read-only Codex session before any write unlock; each write capability one at a time; the initial 2 active models; spend for paid tiers |
| 6 | G1 msi repo read + schema mapping; G2 first import sample; G3 color legend / role vocabulary / LOD budgets on ~50 real concepts; G4 any external vocabulary (license-confirmed first) |
| 7 | TRIPO_API_KEY + credit budget + per-week cap; image egress to Tripo; first 5 3D cards side-by-side vs 2D before bulk; cloud STT consent (if ever) |
| 8 | schedule + run time; two-week shadow, then per-duty go-live (steward first, auditor last); LLM budget cap; every proposal application |
