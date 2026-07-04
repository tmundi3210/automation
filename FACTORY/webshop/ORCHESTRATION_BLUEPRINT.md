# ORCHESTRATION BLUEPRINT — Building an Online Shopping Website (E-Commerce Store)

## 0. Operating Model (how the whole thing is wrapped)

This build is run as **one committed direction** with **many mini-turns**, not one monolith and not a series of direction-resetting loops. The **MIND BRAIN** wraps the entire project as a hybrid cognitive cycle: direction is SET ONCE at the top (W3 writes a falsifiable `success_criterion` + a SERIES/PARALLEL step ledger to `goals/plan.md`), then the kernel runs **exactly one turn per invocation**, and each turn ACTs on **exactly one step** of that ledger as a full `W0 READ → W1 PERCEIVE → W2 RECALL → W3 PLAN(take-next-step) → W4 ACT → W4b GATE → W5 REFLECT → W6 CONSOLIDATE` mini-cycle. Every phase below IS one (or a few) such turns against the shared `mind/memory/` store, so learning from phase N is carried to N+1 through `episodic/`, `kernel/cursor.json`, and `kernel/checkpoint.jsonl` — never re-derived. Inside the brain, the specialists are slotted like this:

- **`orch` is the conductor** — it designs the specialist execution graph per phase, decides SET (parallel/blackboard) vs SEQUENCE, sets termination policy, and gates each hand-off. It does not decide *what is safe to ship* — that is W4b.
- **`loops` governs every iterate-until-good loop** — it frames the loop structure BEFORE any symptom fix, classifies polarity/stability, and issues the **DECISION** `break | throttle | damp | rearchitect` plus a principled stopping tolerance / termination cap for each loop's exit condition.
- **`eval` is the gate function** — it produces the pass/fail acceptance verdict (eval cards, CI regression gates, safety gates, statistical CIs) that each phase gate consumes. `eval` gates releases; it does not build.
- **`argue` and `reason` resolve decisions** — `argue` adjudicates competing options to a justified, defeasible verdict with disclosed defeaters (e.g. hosted-vs-custom); `reason` selects/gates any test-time-compute or output-verification sub-pipeline and verifies correctness claims.
- **`method` gates rigor/ethics** of anything measured (CRO experiments, A/B tests, cut-scores), **`kr` formalizes the catalog ontology**, **`intent` sits at the front of every user-facing turn**, and the domain roster does the concrete building.
- **W4b THE GATE** is the frozen, fail-closed compliance verdict (`STOP / YELLOW / GREEN`) run AFTER act and BEFORE reflect — `security` and `commerce_law` findings feed it, and it can hard-STOP any phase's shippable output regardless of build progress. ACT drafts + flags safety facts; W4b alone decides.

Ordering is enforced by the ledger's dependencies: **viability before build, data model before backend, backend contract before frontend wiring, IA/UX before visual polish, security + law review every phase, and aesthetics gated LAST so it can never block function.**

---

## 1. Phase-by-Phase Plan

Notation: **SET** = parallel substeps on `kernel/blackboard/<env>/_merge/<step>` with a reducer keeping higher-provenance branches. **SEQUENCE** = dependency-ordered. **LOOP** = iterate-until-exit, framed and terminated by `loops`. **GATE** = W4b + `eval` acceptance that must pass to advance. "Consolidate writes" = the mandatory W6 diary lines appended to `episodic/journal/<today>.md` and the INDEX/kernel updates.

---

### Phase 0 — Intake / Brief → Spec

- **Goal:** Convert a raw founder brief into a falsifiable spec + committed direction. This is where W3 **SETS DIRECTION ONCE**.
- **Specialists:** `intent` (front of pipeline — models WHO the founder is + extracts WHAT: intent + slots) → then a **SET**: `business` (unit-economics envelope: target CAC/LTV, contribution margin, price band), `marketing` (acquisition/market frame + KPI targets), `commerce_law` *(to-be-forged)* (jurisdictions, data-residency, tax-registration scope as hard constraints).
- **Order:** `intent` runs SEQUENCE-first (it emits the canonical handoff envelope); the three framing specialists run as a SET, reduced onto the blackboard.
- **Loop:** `intent`'s **clarifying LOOP** — iterate clarifying questions until calibrated confidence ≥ stake-tier threshold; **exit** at friction budget (default max 2 turns, +1 if `risk_if_wrong>0.80 AND intent_confidence<0.90`) or zero-information-gain (`IG<0.02`); OOD `>0.85` escalates immediately.
- **GATE (Spec Gate):** `argue` audits the brief-to-spec reconstruction for charity/faithfulness; W4b verdict must be GREEN/YELLOW. **Acceptance:** one falsifiable `success_criterion` written (metric + threshold + horizon, e.g. "≥2.0% checkout conversion at 3-month horizon on live traffic"), unit-economics envelope signed off by `business`, legal jurisdiction scope enumerated, no unresolved OOD.
- **Consolidate writes:** `goals/direction.md` (committed direction + rejected alternatives), `goals/plan.md` (the full SERIES/PARALLEL ledger for Phases 1–7 under a `max_steps` budget), first diary entry with SET-OUT/PREDICT/ASSUMED, INDEX NOW block seeded with identity + goal + phase.

---

### Phase 1 — Strategy & the Hosted-vs-Custom DECISION

- **Goal:** Choose the platform architecture (hosted SaaS / headless / custom) and lock the build strategy — the pivotal irreversible decision.
- **Specialists:** **SET** to gather evidence: `storefront_eng` *(forge)* (platform capability matrix), `business` (TCO/contribution-margin impact of each option), `security` (attack-surface + PCI scope per option), `commerce_law` (compliance burden per option), `marketing` (SEO/perf ceiling per option). Then **`argue`** adjudicates to a verdict; **`select`** applies if any open-weights model/vendor choice is embedded (Pareto: cheapest option meeting the non-negotiable quality floor + latency SLA).
- **Order:** evidence SET → `argue` SEQUENCE (adjudication is a single reducer step).
- **Loop:** none — this is a one-shot **DECISION** node.
- **GATE (Strategy Gate):** `argue` emits a justified defeasible verdict with **disclosed surviving defeaters, calibrated confidence/interval, and revision conditions**; because it is irreversible/high-stakes it is **escalated to human_review** before hard-lock (W4b compliance-gated/irreversible → ESCALATE). **Acceptance:** platform chosen, TCO within `business` envelope, PCI-DSS scope bounded by `security`, revision conditions recorded.
- **Consolidate writes:** decision record + rejected alternatives into diary BELIEF Δ; a `loops`-relevant note that PCI scope is now a fixed assumption; cursor advanced.

---

### Phase 2 — Data & Domain Modeling

- **Goal:** A decidable, validated catalog/commerce data model before any backend code.
- **Specialists:** **SEQUENCE** — `kr` (formal ontology: product/SKU/variant/inventory/order/customer as OWL/RDF + SHACL shapes, competency questions + test oracles) with `infosci` (taxonomy/metadata/browse facets) and `catalog_merch` *(forge)* (PIM structure, variant axes, promotion model) as a supporting SET feeding `kr`; `payments_tax` *(forge)* and `commerce_law` consulted for tax-jurisdiction + PII fields.
- **Loop:** `kr`'s internal **classify-until-consistent** loop — reason over TBox until no unsatisfiable classes; **exit** when consistency clean + all CQs answered + SHACL conformance passes.
- **GATE (Model Gate = `kr` release gate):** *"consistency clean, pitfall (OOPS!) scan clean, CQ coverage executed, quality assessed"* — **do not lock before acceptance tests + dependency gates pass** (`DR_QA_OVER_LOCK`). **Acceptance:** every competency question answered by a SPARQL/OBDA query, SHACL validation report conformant, IRI/identity (UNA) policy fixed, PII/tax fields flagged for `commerce_law`.
- **Consolidate writes:** the locked schema as a hot pointer in INDEX; BELIEF Δ on identity policy; diary FOUND lines on any modeling surprises → `attention/surprises.md`.

---

### Phase 3 — Backend Build LOOP → "Working Software" Gate  *(HEADLINE LOOP #1)*

- **Goal:** Software that actually runs — API + data layer + auth + payments/OMS wiring, provably.
- **Specialists (SET where independent, on the blackboard):** `appdev` (backend/API/ORM/migrations/auth/identity — **owner**), `payments_tax` (gateway integration, multi-currency, sales-tax calc, PCI controls), `fulfillment_oms` *(forge)* (order state machine, inventory sync, returns/RMA), with `money` (payments theory) and `ir` (search-ranking machinery for on-site search) as companions.
- **The loop (framed by `loops`):**
  1. `appdev` + peers **ACT**: build/extend the API against the Phase-2 contract (W4 drafts, flags safety facts).
  2. **Gate function fires:** `security` runs SAST/DAST/access-control review; `eval` runs the CI regression + contract test suite; `reason` verifies any correctness-critical claim.
  3. **`loops` DECIDES** the intervention on the feedback signal: `break` (stop/fix a divergent build), `throttle`, `damp`, or `rearchitect` — and issues the fix/proceed verdict.
- **Exact exit condition (`loops` stopping rule):** iterate the build→gate→decide map until it reaches a **fixed point** — *all* CI/contract tests green, `security` finds **zero unresolved high/critical** (SQLi/XSS/CSRF/broken-access/session), payment sandbox transactions succeed idempotently, and OMS state transitions are conserved — with termination **bounded by an explicit `max_steps` cap**; if the cap is hit without fixed point, a named defeater ("ledger exhausted / step blocked") fires and W3 re-plans or ESCALATES.
- **GATE ("Working Software" Gate):** W4b verdict **GREEN required** — `security` clearance + `eval` CI-pass + `payments_tax` PCI-scope controls in place. **Acceptance:** endpoints serve real requests, migrations apply clean, auth works, a test order flows end-to-end (cart→pay→order→fulfillment record), zero unresolved critical vulns.
- **Consolidate writes:** each loop iteration = one turn's diary entry; scored predictions in `calibration/predictions.jsonl` (e.g. "N=3 fixes to close the access-control gap"); lessons only as `TRIGGER → MOVE → EVIDENCE(CI)` and only if they beat baseline; checkpoint per completed step so a restart resumes mid-loop.

---

### Phase 4 — Storefront / UI Build

- **Goal:** A working storefront wired to the now-frozen backend contract — function before beauty.
- **Specialists:** **SEQUENCE gated by IA-first** — `store_ux` *(forge)* + `infosci` first establish information architecture + buy-path flows (product page → cart → checkout), THEN `storefront_eng` builds the frontend + cart/checkout engineering against the backend API, `catalog_merch` wires search/recs/merchandising, `tool` supplies any RAG/agent-memory for on-site search/support widgets, `customer_cx` *(forge)* stubs reviews/trust surfaces. `marketing` consulted for SEO-friendly rendering.
- **Order invariant:** **backend contract (Phase 3) before frontend wiring; IA/UX before any visual styling.**
- **Loop:** `tool`'s **ReAct loop** only where an agentic search/support component exists (iterate reason→act→observe until task complete or `max_iterations`); otherwise SEQUENCE build steps.
- **GATE (Functional UI Gate):** `eval` + `store_ux` verify the buy-path works: every route renders, cart persists, checkout completes against the real backend, `store_ux` WCAG smoke pass (keyboard/nav basics). **Acceptance:** a human can complete a purchase end-to-end in the UI; no visual polish required yet.
- **Consolidate writes:** IA + component map as hot pointers; diary DID/FOUND; any conversion-risk hypotheses parked in `attention/questions.md` for Phase 5.

---

### Phase 5 — Aesthetics & Conversion Polish LOOP → "Aesthetic Working UI" Gate  *(HEADLINE LOOP #2)*

- **Goal:** A visually aesthetic, accessible, conversion-ready UI — layered ON TOP of the already-working storefront so it can never block function.
- **Specialists (SET, iterating together):** `design` (the aesthetic authority — color/type/composition/design system — **owner**), `store_ux` (buy-path conversion UX + WCAG), `psych` (conversion/trust psychology, **ethical-influence-only**), `creator` (product photography/video/grade/QC), with `marketing` (CRO instrumentation) and `customer_cx` (reviews/trust surfaces) consulting.
- **The loop (framed by `loops`, measured by `method`, gated by `eval`):** `design`+`store_ux`+`psych`+`creator` iterate the visual/interaction/media pass → `eval` scores it against a **four-part rubric** → `loops` decides continue/stop.
- **Exact exit condition:** iterate until the composite gate passes AND marginal improvement per iteration falls below the `loops`-set tolerance (or `max_steps` cap): 
  - **Core Web Vitals** — LCP/CLS/INP within Google "good" thresholds (`marketing` measures),
  - **WCAG** — AA conformance (`store_ux`),
  - **Aesthetic rubric** — `design`'s design-system scorecard met,
  - **Conversion** — `eval` confirms a statistically significant lift over the Phase-4 **baseline** (not vs zero), with `method` gating experiment rigor/ethics (no dark patterns; `psych` ethical-influence constraint enforced).
- **GATE ("Aesthetic Working UI" Gate):** all four sub-gates GREEN + W4b (no manipulative/deceptive pattern; accessibility compliance). **Acceptance:** the four thresholds above, each with a CI, over a valid experiment.
- **Consolidate writes:** scored A/B predictions; a surprise (rubric passed but conversion didn't move) becomes a high-priority WHY-question in `attention/surprises.md`; winning design-system tokens promoted toward `semantic/` only at scheduled sleep-consolidation.

---

### Phase 6 — Trust / Growth / Legal & Launch Gate

- **Goal:** Everything required to ship legally, safely, and acquisition-ready.
- **Specialists (SET):** `commerce_law` (GDPR/CCPA, consumer-protection/returns law, terms, tax registration — **owner**), `security` (final appsec + data-protection audit, TLS), `payments_tax` (fraud + final PCI attestation), `customer_cx` (support/CRM, returns comms), `comm` (customer-service comms templates), `marketing` (technical SEO, analytics/consent-mode, launch acquisition), `business` (final margin check).
- **Order:** parallel audits → single W4b verdict.
- **Loop:** none — this is the **fail-closed launch gate**.
- **GATE (Launch Gate = W4b, fail-closed):** verdict must be **GREEN**; any `commerce_law` or `security` STOP halts launch regardless of build progress. Irreversible → **ESCALATE to human sign-off** with the signed disclosure token + outbox packet. **Acceptance:** privacy policy + terms + returns policy live and lawful, consent/cookie handling correct, tax registration where nexus exists, zero critical vulns, fraud controls active, analytics + SEO live.
- **Consolidate writes:** Clearance Record + disclosure token; launch decision + assumptions to diary; INDEX phase → "live".

---

### Phase 7 — Continuous Run / CRO Loop

- **Goal:** Keep it running and compounding — observability, reliability, and continuous conversion optimization.
- **Specialists:** `orch` (observability traces/dashboards, conflict scores, human-escalation queue), `loops` (reliability control: retry/backoff, rate-limit, circuit-breaker, idempotency, blast-radius/damping), `marketing` + `method` (ongoing CRO experiments, rigor-gated), `eval` (continuous-monitoring + Goodhart guard on any judge/reward signal), `customer_cx` (retention/loyalty), `security` (ongoing DAST/patch), `commerce_law` (regulatory drift).
- **Loop:** the **continuous CRO/monitoring loop** — each experiment is its own W0→W6 mini-turn; `method` gates rigor/ethics, `eval` gates significance, `loops` supplies reliability controllers with explicit convergence + stopping rules. **Exit:** never terminates; re-plans only when a **named defeater** fires (metric off-track K times, assumption broke, new info flips direction, budget low) — otherwise the top-level direction holds.
- **GATE:** each CRO change re-enters the Phase-5 four-part gate + W4b before ship; regressions fail CI (`eval`) and roll back.
- **Consolidate writes:** scheduled **sleep-consolidation** wake-ups (`kernel/schedule.jsonl`) fold accumulated BELIEF Δs into `semantic/`, promote lessons seen ≥2× into `procedural/`, archive old journals, and rewrite `calibration/recalibration_map.json` if miscalibrated.

---

## 2. The Two Headline Loops (stated precisely)

**LOOP #1 — Backend Build Loop (Phase 3)**
```
appdev/payments_tax/fulfillment_oms  ──ACT(build)──►  [ security SAST/DAST/access + eval CI/contract + reason correctness ]
        ▲                                                          │
        └──────────── loops DECIDES: break│throttle│damp│rearchitect (fix) ◄─┘
```
- **Framing rule (`loops`):** frame the loop structure before fixing any symptom; model the build→gate cycle as a map to a fixed point.
- **EXIT:** fixed point reached — all CI/contract tests green **AND** zero unresolved high/critical security findings **AND** idempotent payment sandbox success **AND** conserved OMS state transitions; termination **bounded by an explicit `max_steps` cap** (undecidable-in-general → capped). Cap hit without fixed point ⇒ defeater ⇒ W3 re-plan/ESCALATE. Passing ⇒ **"Working Software" gate GREEN**.

**LOOP #2 — Aesthetics & Conversion Loop (Phase 5)**
```
design + store_ux + psych + creator  ──ACT(polish)──►  eval scores 4-part rubric
        ▲                                                     │
        └──── loops: continue while Δ > tolerance ◄───────────┘   method gates experiment rigor/ethics
```
- **EXIT:** ALL four sub-gates pass — **Core Web Vitals** (LCP/CLS/INP "good") **AND** **WCAG AA** **AND** **`design` aesthetic-rubric score met** **AND** **conversion lift over Phase-4 baseline is statistically significant (CI excludes zero)** — AND marginal per-iteration improvement < `loops` tolerance (or `max_steps` reached). `psych` constrained to ethical influence; `method`/W4b block any dark pattern. Passing ⇒ **"Aesthetic Working UI" gate GREEN**.

---

## 3. Whole-Flow Pipeline Diagram

```
                          ┌─────────────────────────────────────────────────────────────┐
   MIND BRAIN wraps all ─►│  W0 READ ▸ W1 PERCEIVE ▸ W2 RECALL ▸ W3 PLAN ▸ W4 ACT ▸       │
   (one turn per step)    │  W4b GATE ▸ W5 REFLECT ▸ W6 CONSOLIDATE   (orch conducts)     │
                          └─────────────────────────────────────────────────────────────┘

 P0 Intake      P1 Strategy        P2 Data          P3 Backend BUILD LOOP        P4 UI Build
 Brief→Spec     hosted vs custom   kr model                                      (contract-wired)
 ┌───────┐      ┌────────┐         ┌───────┐        ┌──────────────────┐          ┌────────┐
 │intent │─────►│ argue  │────────►│  kr   │───────►│ appdev/pay/oms   │◄──┐      │store_ux│
 │+SET   │      │DECIDE  │         │+SHACL │        │  build           │   │      │storefr │
 └───────┘      └───┬────┘         └───┬───┘        └────────┬─────────┘   │      └───┬────┘
     │              ◇ human            ◇ kr release          ▼             │          │
     ◇ Spec        (irreversible)      (consistent+CQ)  ◇ sec+eval+reason  │          ◇ Functional
     │              │                   │              (fix?)─loops────────┘          │  UI gate
     ▼              ▼                   ▼                   ▼ pass                     ▼
  [GREEN]        [ESCALATE]          [GREEN]          ◇ WORKING SOFTWARE ═GREEN═►  [GREEN]──►

 P5 Aesthetics & Conversion LOOP        P6 Launch (fail-closed)      P7 Continuous Run
 ┌───────────────────────────┐         ┌────────────────────┐        ┌──────────────────┐
 │ design+store_ux+psych+     │◄──┐     │ commerce_law +     │        │ orch obs + loops │
 │ creator  polish           │   │     │ security + pay_tax │        │ reliability +    │◄─┐
 └──────────┬────────────────┘   │     │ + marketing        │        │ marketing/method │  │
            ▼ eval 4-part rubric │     └─────────┬──────────┘        │ CRO experiments  │  │
    ◇ CWV+WCAG+aesthetic+conv    │               ◇ W4b STOP/GREEN    └────────┬─────────┘  │
      (Δ>tol? loops continue)────┘               (human sign-off)             ◇ ship?───────┘
            ▼ all pass                            ▼ GREEN                       ▼
   ◇ AESTHETIC WORKING UI ═GREEN═►             [ LAUNCH ]  ─────────►   (loops forever; defeater→replan)

   Legend:  ◇ = gate (W4b + eval)   ═►/──► = advance   ◄──┐ back-arrow = LOOP
            security + commerce_law REVIEW EVERY PHASE (feed W4b);  aesthetics gated LAST.
```

---

## 4. Specialist × Phase RACI

**O** = Owner · **R** = Reviewer (feeds the gate) · **C** = Consulted · blank = not engaged. Meta specialists span all phases per the operating model.

| Specialist | P0 Intake | P1 Strategy | P2 Data | P3 Backend | P4 UI | P5 Aesthetics | P6 Launch | P7 Run |
|---|---|---|---|---|---|---|---|---|
| **mind brain** | O(dir) | O | O | O | O | O | O | O |
| **orch** | R | R | R | R | R | R | R | O |
| **loops** | | C | C | O(loop) | C | O(loop) | | O |
| **eval** | R | R | R | R(gate) | R(gate) | R(gate) | R | R |
| **argue** | R | O(decide) | | C | | C | | C |
| **reason** | | C | | R | C | | | C |
| **method** | | C | | | | R(rigor) | | R |
| **intent** | O | | | | C | C | | C |
| **kr** | | | O | C | | | | |
| **appdev** | | C | C | O | R | | R | C |
| **security** | R | R | R | R(gate) | R | R | R(gate) | R |
| **payments_tax** | | R | C | O | R | | R | C |
| **fulfillment_oms** | | | C | O | R | | C | C |
| **storefront_eng** | | O(evid) | | C | O | R | | C |
| **catalog_merch** | | C | O | C | O | R | | C |
| **store_ux** | | | | | O(IA) | O | | R |
| **customer_cx** | | | | | C | R | R | O |
| **design** | | | | | C | O | | C |
| **creator** | | | | | | O | | C |
| **marketing** | R | R | | | C | R(CRO) | R | O |
| **business** | R | R | | C | | C | R | C |
| **commerce_law** | R | R | C | C | | R | O(gate) | R |
| **money / ir / infosci / psych / comm / tool / select** | infosci/select C@P1–2 | select C | infosci C, tool C | money C, ir C | ir/infosci/tool C | psych R, comm C | comm C | psych/comm C |

---

## 5. Ordering Invariants (non-negotiable dependencies)

1. **Viability before build** — Phase 0/1 `business` + `argue` must clear before any code (no build against negative unit economics).
2. **Data model before backend** — `kr`'s release gate (Phase 2) is a hard dependency of Phase 3; backend never precedes a consistent, CQ-covered schema.
3. **Backend contract before frontend wiring** — Phase 4 wires against the frozen Phase-3 API; no UI wiring against an unstable contract.
4. **IA/UX before visual polish** — `store_ux`/`infosci` fix information architecture and buy-path flows (Phase 4) before `design`/`creator` apply any visual layer (Phase 5).
5. **Security + law review EVERY phase** — `security` and `commerce_law` feed W4b at every gate; W4b can hard-STOP any shippable output at any phase.
6. **Aesthetics gated LAST** — the aesthetics/conversion loop is layered on an already-working storefront so beauty can never block function; its gate is the last capability gate before launch.
7. **Direction set once; re-plan only on a named defeater** — W3 commits one direction at Phase 0; later turns take the next unblocked step and re-plan only if ledger-exhausted / step-blocked / metric off-track K× / assumption broke / info flips direction / budget low.
8. **W4 drafts, W4b decides** — no specialist may soften, re-implement, or route around the compliance gate.

---

## 6. How To Actually Run This

- **Forge the 7 commerce-native specialists FIRST.** They are referenced throughout but do not exist yet — build them before the ledger reaches the phases that own them: `storefront_eng` (needed by P1/P4), `catalog_merch` (P2/P4), `payments_tax` (P3), `fulfillment_oms` (P3), `store_ux` (P4/P5), `customer_cx` (P6/P7), `commerce_law` (P0/P6). Each should be forged with an explicit `loop_or_gate` contract mirroring the roster style so `orch` can slot and gate it.
- **`mind/memory/` is the single source of cross-phase truth.** All phase state lives there — `goals/plan.md` (the ledger), `kernel/cursor.json` + `kernel/checkpoint.jsonl` (resume point), `episodic/journal/*` (what happened), `calibration/predictions.jsonl` (bets scored), and the hot pointers/INDEX NOW block. There is no code runtime: the LLM IS the runtime, reading at W0 and writing at W6 every turn. Never edit `semantic/` or `procedural/` live — only PROPOSE via BELIEF Δ / LESSON? lines; those bake in only at scheduled sleep-consolidation.
- **Run one turn per invocation.** The kernel ACTs on exactly one ledger step, passes W4b, reflects vs baseline, checkpoints, and advances the cursor — then stops. Restart resumes from the last checkpoint's next step. The build ends only as **DONE** (success_criterion met), **ABANDONED** (defeater with no repair), or **ESCALATED** (irreversible/compliance-gated step or exhausted budget).
