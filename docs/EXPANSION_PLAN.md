# Knowledge Searcher — Complete-Set Expansion Plan

> **Goal (user, 2026-06-23):** "Build a complete set." Take the proven Phase-A
> Knowledge-Searcher machinery (taxonomy → 3 dense gated KBs per domain → 1 distilled
> specialist → ROUTER + INDEX) and extend it from the current **10 foundational
> specialists** out across the applied fields the user named — **starting with Security**
> (explicitly "the main one"), with **Loops/systems** the other flagged "main."
>
> This doc is the architecture step. It (a) consolidates the brain-dump into clean,
> de-duplicated **domains**, (b) scopes each, (c) sets **boundaries**, and (d) fixes a
> **build order**. Each domain is then built by the *same* gated pipeline as the first 10.
> Nothing here is final — correct any decomposition before its batch is built.

---

## 0. The existing 10 (do not rebuild — compose with these)

`method` (methodology/epistemology) · `infosci` (knowledge organization) · `ir`
(retrieval/search) · `biblio` (bibliometrics) · `evsynth` (evidence synthesis) · `kr`
(knowledge representation) · `scholcomm` (source provenance/trust) · `libarch`
(library/archival/data curation) · `quant` (statistics & causal inference) · `argue`
(argumentation & critical reasoning).

These are the **reasoning/knowledge-finding core**. The new domains below are **applied
verticals** that *consume* the core (e.g. every new domain's "is this claim true?" routes
through `argue`/`scholcomm`/`quant`; every "find the best source" routes through `ir`/`libarch`).

---

## 1. Scoping decisions (how the brain-dump was turned into domains)

1. **Consolidated**, not literal. ~30 raw phrases → **18 domains**, each = exactly 3 subdomains
   (the pipeline's unit). Related items merged (e.g. journalism + "history-taking" → a news
   subdomain; "speaking" → communication; "what is money" → a money/banking domain).
2. **Scoped narrow where the user did.** Math is **not** "all math" — only the applied math the
   user named (finance, business, design, planning, architecting). `appmath` is the
   deterministic/applied complement to the existing `quant` (statistics/causal).
3. **De-duplicated against the existing 10.** New domains reference the core rather than
   re-deriving it. Notes per domain below.
4. **Each domain ships the full gate:** 3 dense KBs at **35/35** (`kb_validator --mode dense`)
   + 1 specialist at **13/13** (`specialist_validator`) + ROUTER/INDEX regen. No padding.

---

## 2. The 18-domain map

> Code = specialist id. ✦ = user flagged "main" (built first).

| # | code | Domain | 3 subdomains | de-dup / note |
|---|------|--------|--------------|---------------|
| 1 ✦ | `security` | **Security Engineering & Offense-Aware Defense** | network_and_wireless_security · web_application_security · offensive_techniques_and_defensive_detection | new. **Defensive framing — see §4.** |
| 2 ✦ | `loops` | **Systems Thinking, Feedback Loops & Control** | feedback_loops_and_system_dynamics · control_theory_and_stability · computational_and_learning_loops | new. "use math for this" → control theory + dynamics. |
| 3 | `appmath` | **Applied Math (finance, business, design, planning)** | financial_and_business_math · geometry_and_spatial_math · optimization_and_planning_math | deterministic/applied complement to `quant` (statistics). |
| 4 | `appdev` | **Software & App Engineering (Python-centric)** | python_ecosystems_and_packaging · mobile_and_desktop_apps · server_and_backend_systems | iPhone/Mac/Windows/Linux-server paths. |
| 5 | `business` | **Business Strategy & Operations** | strategy_and_competitive_analysis · operations_and_management · entrepreneurship_and_models | new. |
| 6 | `markets` | **Financial Markets & Investing** | securities_and_fundamental_analysis · market_mechanics_and_trading · portfolio_and_risk_management | educational, **not** advice (§4). |
| 7 | `money` | **Money, Banking & Macro-Finance** | monetary_systems_and_central_banking · banking_credit_and_payments · macroeconomics_and_inflation | "what is money." |
| 8 | `uslaw` | **US Law, Finance & Regulation (location-specific)** | us_legal_system_and_contracts · us_financial_and_banking_regulation · us_tax_and_business_compliance | the "specific location" specialist; **not legal advice** (§4). |
| 9 | `worldmodel` | **Geopolitics, News & Information Integrity** | geopolitics_and_international_relations · news_media_systems_and_journalism · misinformation_detection_and_credibility_scoring | fake-news **scoring**; composes `argue`+`scholcomm`. |
| 10 | `comm` | **Professional Communication & Speaking** | written_and_professional_communication · negotiation_and_persuasion · public_speaking_and_delivery | "speaking" lives here. |
| 11 | `psych` | **Applied Psychology** | cognitive_and_behavioral_psychology · social_and_persuasion_psychology · motivation_and_decision_psychology | **informational, not clinical** (§4). |
| 12 | `health` | **Fitness, Nutrition & Health Behavior** | exercise_science_and_training · nutrition_and_diet · sleep_recovery_and_habits | **informational, not medical advice** (§4). |
| 13 | `aicomp` | **Computing & How LLMs Work (foundations)** | computing_and_systems_fundamentals · how_llms_work · ai_in_practice_and_limits | literacy layer; Phase-B = the *engineering/build* of LLMs. |
| 14 | `maker` | **Hardware, Microcontrollers & Self-Hosting** | electronics_and_microcontrollers · home_servers_and_self_hosting · physical_computing_and_automation | Arduino/servers/automation. |
| 15 | `ling` | **Linguistics** | structural_linguistics · sociolinguistics_and_pragmatics · applied_and_computational_linguistics | new. |
| 16 | `marketing` | **Marketing & Growth** | brand_and_marketing_strategy · digital_and_performance_marketing · market_research_and_analytics | new. |
| 17 | `creator` | **Creator Craft & Content Production** | content_and_audience_strategy · visual_production_lighting_and_camera · creator_automation_and_distribution | influencer / lighting / camera / automation. |
| 18 | `design` | **Design: Fashion, Architecture & Principles** | fashion_and_apparel_design · architecture_and_built_environment · design_principles_and_history | clothes (men+women), architecture, history of design. |

**Result when complete:** 10 → **28 specialists**, 30 → **84 dense KBs**.

---

## 3. Build order (batches; each batch = commit + ROUTER/INDEX regen)

- **Batch 1 (now):** `security` ✦ + `loops` ✦  — the two "main" domains.
- **Batch 2 — technical core:** `appmath` · `appdev` · `aicomp`.
- **Batch 3 — money/business cluster:** `business` · `markets` · `money` · `uslaw`.
- **Batch 4 — world & people:** `worldmodel` · `comm` · `psych`.
- **Batch 5 — influence & make:** `marketing` · `creator` · `maker`.
- **Batch 6 — design/lang/health:** `design` · `ling` · `health`.

Each batch is independently committed and pushed; the set is always in a valid, gated state.

> **Pending "basic" cleanup (optional, no new authoring):** split `libarch` into
> `reference/discovery` vs `archival/data-curation` specialists by re-distilling its 3
> existing KBs. Cheap; queued behind the new verticals unless prioritized.

---

## 4. Boundaries (non-negotiable; enforced in the KBs themselves)

These extend the project's existing boundary discipline (`phase_b/IDEA_NEUTRALIZED.md §5`).

1. **`security` = offense-aware DEFENSE.** KBs encode adversary techniques (MITRE ATT&CK-style)
   **for the purpose of detection, alerting, hardening, and incident response** — i.e.
   understand how intrusions happen *so you can prevent and alarm them* (the user's stated
   intent). Scope: defensive security, **authorized** testing, and education. **Not** in scope:
   operational exploit/malware playbooks for unauthorized use, target selection, or evasion for
   wrongdoing. Each security KB states this scope in its purpose.
2. **`psych` = informational & adaptive, not clinical.** No diagnosis, no treatment. Mirrors the
   project's cognition-modelling boundary.
3. **`health` = general fitness/nutrition literacy, not medical advice.** Flags "consult a
   professional" for clinical conditions.
4. **`uslaw` = legal/regulatory literacy, not legal advice.** Jurisdiction-tagged,
   currency-flagged (statutes/regs change); composes `scholcomm` for source authority.
5. **`markets` / `money` / `business` = financial *education*, not personalized advice.** Every
   volatile figure is as-of-dated; no buy/sell recommendations.

---

## 5. Status

| Batch | Domains | KBs gated | Specialists | ROUTER/INDEX |
|---|---|---|---|---|
| 1 ✅ | security, loops | 6/6 @ 35/35 | security, loops (13/13) | → 12 specialists / 36 KBs |
| 2 ✅ | appmath, appdev, aicomp | 9/9 @ 35/35 | appmath, appdev, aicomp (13/13) | → 15 specialists / 45 KBs |
| 3 ✅ | business, markets, money, uslaw | 12/12 @ 35/35 | business, markets, money, uslaw (13/13) | → 19 specialists / 57 KBs |
| 4 | worldmodel, comm, psych | in progress | pending | pending |
| 5–6 | (5 remaining) | queued | queued | queued |

**Set totals after Batch 3:** 19 specialists · 57 dense KBs · 1,188 nodes · 2,210 edges ·
793 competency-questions · INDEX `all_pass: True`.

Updated as batches land. The pipeline, gate, and tools are unchanged from the first 10 —
this is breadth on a proven machine, not new machinery.
