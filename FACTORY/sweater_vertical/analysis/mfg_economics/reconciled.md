# Manufacturing Economics — Reconciled (T4)

**For:** the writer who builds `mfg_economics/DEEPDIVE.md`, and the KB authors after.
**What this is:** the balanced merge of the two T4 inputs — **the per-garment cost model** and **the make-vs-buy decision** — into one picture: agreed facts (tagged), the decision logic, and the contested/uncertain points, with over-confident claims downgraded and every honesty tag preserved.

**How to read the tags (binding, inherited from T0/T1/T2/T3):**
- **[FACT]** = cited public source / true-by-definition identity.
- **[ESTIMATE, confidence]** = reasoned estimate; method + basis + confidence stated; carries the literal word *estimate*.
- **[UNKNOWN]** = genuinely not known; not faked. Coverage is a **floor, not a ceiling.**

---

## 0. A process-level honesty note that frames this whole reconciliation (read first)

**ERRATUM (post-build):** at reconcile time Analyst B's memo had **timed out** and was absent — hence the asymmetry described below. `analyst_b_make_vs_buy.md` was **subsequently written** in the build wave, so the make-vs-buy verdict is now a **tested A/B conclusion** (B1/B2/B3 trade study + flip-conditions, folded into KB2). Read every "B absent / un-cross-examined / reconstructed-only" note below as **superseded**.

**At reconcile time the two named inputs were not symmetric.** Only **Analyst A — the per-garment cost model** (`analyst_a_cost_model.md`) had been written; **Analyst B's make-vs-buy memo had timed out and did not yet exist** at reconcile time (since written — see Erratum). So *this reconcile* was **not** the usual two-independent-analyst merge; it is:

- **Analyst A's cost model**, taken on its own terms, plus
- **the make-vs-buy decision logic reconstructed from the upstream deep-dives that already carry it** — T3 Machines §1–6 (subcontract-vs-buy-vs-build, the linker/programmer bottlenecks), T1 Ludhiana §3/§6 (the six moats, compliant-core, where-better), T2 Yarn §2/§5 (buy-local-vs-import, never-own-a-dyehouse), and SCOPE_GLOSSARY §7 (the make-vs-buy lever as the venture's recurring central decision).

**Why this is still a sound reconciliation, not a fabrication:** the make-vs-buy *conclusion* for this venture is already **convergent and high-confidence across three upstream segments** (subcontract the knit/link/finish mesh, own only design+brand+US-import, never build a machine, never own a dyehouse). So the "B-side" view is real and well-sourced — it simply lives in the upstream files rather than in a dedicated `analyst_b` file. The honest cost of B's absence is **not** a missing answer; it is a **missing independent challenge to A's cost bands and to the make-vs-buy boundary** — i.e., less adversarial pressure than a normal merge. I flag that explicitly as the reconciliation's own [UNKNOWN], and I downgrade confidence accordingly wherever a claim rests on a single source. **The writer/critic should treat the make-vs-buy section here as reconstructed-consensus, not as a cross-examined two-analyst result.**

---

## 1. Agreed facts and framings (tagged) — the load-bearing anchors both the cost model and the upstream make-vs-buy reasoning rest on

These are not contested between A and the upstream make-vs-buy reasoning; they are the shared substrate.

- **The model boundary is ex-factory conversion cost, not landed cost and not a P&L.** The stack is `full yarn + knit job-work + LINK job-work + finish + trims/QC/pack → subtotal + compliance loading + overhead/job-worker margin = ex-factory cost per garment`. Duty/freight/insurance/broker live in **T9**; CAC/returns/contribution margin are the **D2C survival test (T7/T9)**. `[FACT — boundary definition; per T0 cross-team rule #8 and Analyst A §0]`
- **Yarn sets the *level*; the [UNKNOWN] job-work rate sets the *uncertainty*.** Yarn ≈ **50–60% of the knit-garment stack** (anchor: Vardhman FY20 raw material = 54% of revenue `[FACT, T1 §5a]`); labour (knit/link/finish/check) ≈ **15–25%** `[ESTIMATE, MED, T1 §5b]`. So the stack's magnitude is well-anchored, but its precision is blocked by the lines we cannot price. `[FACT on the anchor; ESTIMATE, MED on the percentage split]`
- **Yarn cost per garment is small and well-anchored for the value/core fibres: ~$1–2.50 of fibre per sweater** at ~300–500 g/garment. `[ESTIMATE, MED — carried from T2 §3a]` Premium-fibre rows (merino/lambswool/cashmere) are **order-of-magnitude only** and must not enter a margin model without an importer quote. `[ESTIMATE, LOW, T2 §3a]`
- **The make-vs-buy boundary is the venture's recurring central lever** — own spinning/knitting vs buy yarn/CMT vs full-package FOB. `[ESTIMATE: design-framing, HIGH it's the key lever; GLOSSARY cross-team rule #9]`
- **The linker — not the knitting machine — caps a small line's daily output**, and it is the hardest skill to re-hire after the lean season. `[ESTIMATE, MED-HIGH; T3 §1, T1 §4]` This is why linking is both *the* throughput bottleneck **and** (per Analyst A) one of the two unpriced cost lines.
- **The compliance loading is ~15%+ (EPF 12% + ESI 3.25%) on on-roll cash wages** — real, and the largest single *legal* cost disadvantage vs an off-roll competitor. `[ESTIMATE→arithmetic from statutory rates, HIGH on the ~15%; T1 §5b]`
- **Do not own a dyehouse, and do not build a machine.** Dyeing is the highest-regulatory-risk node (all three Ludhiana CETPs non-compliant, crore-scale CPCB penalties, NGT review `[FACT, T1 §5b]`); building a flat-knit machine is ~$2M–10M+ over 3–7 years for a likely-inferior result vs a sub-$20k used Chinese machine `[ESTIMATE, HIGH on direction; T3 §3]`. Both are the highest-confidence "do-nots" in the venture.
- **HSN/duty constants** (for whichever side of make-vs-buy imports goods or a machine): garments heading **6110** (cotton 6110.20.20 ≈16.5%, wool 6110.11 ~16%, acrylic/MMF 6110.30.30 ~32%); knitting machines **HSN 8447** (~7.5% BCD + 18% IGST into India). `[FACT — USITC; HSN 8447 — T2/T3]`

---

## 2. The decision logic — make-vs-buy, reconciled into one boundary

**The single reconciled recommendation** (convergent across T1/T2/T3; cost model consistent with it):

> **Buy, don't make — at capsule scale.** Own only **design + brand + US import (importer of record)**. **Subcontract knit + link + finish into the Ludhiana job-work mesh**, **buy yarn in-cluster on bought-dyed stock shades** (import only premium tops), and **rent/contract the scarce CAD/flat-knit programmer**. Graduate toward owning a step **only when a specific repeating program runs at proven, sustained volume**. **Never** own a dyehouse; **never** build a machine. `[ESTIMATE: judgment, HIGH on the capsule-scale verdict — three upstream analysts converged; T1 §3/§6, T2 §5, T3 §1–6]`

**The make-vs-buy ladder, stage by stage (where the boundary sits and why):**

| Stage | Default posture | Make only when | Why buy at launch |
|---|---|---|---|
| **Spinning / yarn** | **Buy** (market yarn, in-cluster) | never, at this venture's scale | Integrated groups have captive spinning margin we can't match; yarn is a commodity input we spec, not make (T1 §3, T2 §2). |
| **Dyeing** | **Buy** (bought-dyed stock shades; job-work dye only for a proven hero colour) | **never own** — outsource always | Highest regulatory-risk node; NGT/CETP closure exposure wildly out of proportion to a 3-article line (T1 §5b, T2 §5). |
| **Knitting** | **Subcontract** the FF panels | one style at proven volume → buy **used** (Chinese/Cixing first) | Agglomeration *is* the infrastructure; small-batch iteration is Ludhiana's genuine edge; a tiny owned line is a mid-six-figure + skilled-staffing commitment before a garment sells (T3 §2). |
| **Linking** | **Subcontract** | last to in-source — it's the bottleneck *and* the scarce skill | Linker sets the daily ceiling; owning it early imports the throughput risk and the hardest-to-hire skill (T3 §1, T1 §4). |
| **Finishing** | **Subcontract** (optionally one cheap used press for sampling) | when volume justifies | Anti-pill/press/board are job-work steps the mesh already supplies (T2 §4, T3 §1). |
| **Design + brand + US import** | **OWN** | — always in-house | This is the venture's entire edge and where downstream margin is captured (T1 §6, GLOSSARY §8). |

**The reasoning that makes "buy" correct here — and the honest counter:**
- **Pro-buy (the consensus):** capital stays free for design + CAC; the cluster's walking-distance subcontracting mesh suits the small, fast, varied runs an AI-design brand lives on; the venture's margin is captured downstream in US D2C, so the factory gate is **not** the whole game; and a compliant entrant can never win on factory cost anyway (T1's central seam: incumbents' model is the paper-vs-real-cost gap). `[ESTIMATE: judgment, HIGH]`
- **The counter that must stay honest (and is *under-pressed* given B's absence):** subcontracting means **QC is fragmented across many micro-vendors** (T1 §3 agglomeration is flexible *but* QC-fragmented), the brand **carries no captive margin to absorb shocks**, and the **per-style fixed costs (sampling/PPS, tech-pack, lab dips, AQL) amortise over a tiny MOQ → they pull per-garment cost up materially at low volume** (Analyst A Open Q#7). The "buy" verdict is right for *launch*, but its cost penalty at micro-volume is real and **not yet quantified.**

---

## 3. The cost model, reconciled — what survives, what gets downgraded

Analyst A's stack is methodologically sound and honestly tagged. Reconciliation keeps its structure and **explicitly preserves its self-flagged softness**; B's absence means **no second analyst stress-tested A's bands**, so I hold them at A's stated confidence and add the caveat that they are un-cross-examined.

### 3a. What holds (agreed / well-supported)
- **The 10-line cost-line taxonomy** (yarn, knit job-work, link job-work, finishing, trims/labels/pack, QC, EPF/ESI loading, overhead/margin, friction/speed-money, inverted-duty drag). The discipline of **naming lines 9–10 but NOT adding them** — friction is episodic/amortised-to-near-zero per garment at capsule volume, inverted-duty drag is a **working-capital cost (T9 cash-cycle), not a per-unit COGS line** — is correct and inherited cleanly from T1 §5b/§5c. `[ESTIMATE, sound; tags preserved]`
- **The conversion cross-check is shape-only, not a quote.** The published FF-knitwear study (knitting ~$0.74/pc @9/day, linking ~$0.45/pc @15/day, finishing chain) confirms the **shape** (linking ≈ 60% of knitting cost per piece; the labour-heavy seam) and is consistent with T3's "linker sets the ceiling." A's refusal to collapse it to a Ludhiana point — because the *productivity* (pc/operator-day) the rate multiplies is [UNKNOWN] for Ludhiana — is the right call. `[FACT — that study's figures; ESTIMATE, LOW that it transfers as a level]`
- **Compliance loading applied only to the labour-bearing portion of conversion, not the whole stack.** In a subcontract model the ~15%+ mostly lands on the *job-worker*, not the brand; the brand carries it only on labour it puts on its own roll (sampling, in-house finishing/QC). Correct, and consistent with T1 §5b. `[ESTIMATE, HIGH on rate; UNKNOWN on the base it applies to]`

### 3b. The illustrative ex-factory bands — carried, with confidence preserved (and one band tightened in label only)
All three are **labelled illustrations, not quotes**; assumptions live in Analyst A §3.

| Article | Ex-factory band (US$/garment) | Reconciled confidence | Note |
|---|---|---|---|
| **A1 — cotton-blend 12-month core (12GG)** | **≈ $4–9** | `[ESTIMATE, LOW-MED]` | blocked by the [UNKNOWN] knit/link rate |
| **A2 — fine-acrylic value driver (7–12GG)** | **≈ $3.5–8** | `[ESTIMATE, LOW-MED]` | the **cost floor** of the three; best MOQ-flex; margin lives or dies on the anti-pill finish (ICI ≥3–4 post-wash, T2 §4) — a cheap line that, if cut, returns as the #1 return driver |
| **A3 — merino/lambswool premium capsule (7–12GG)** | **≈ $6–22+** | `[ESTIMATE, LOW]` | **least reliable** — yarn-dominated and order-of-magnitude; wool duty (~16%) ≈ cotton (~16.5%), so its *landed* premium is widened by high ex-factory yarn cost, not a duty gap; acrylic/MMF ~32% is the duty outlier |

### 3c. What gets DOWNGRADED (over-confident or thinly-sourced — flagged for the writer/critic)
- **The conversion band "US$1.5–4.0/garment" for a mid-complexity FF pullover** is Analyst A's **single softest number**, and it is soft *because* the knit + link lines inside it are [UNKNOWN]. **Downgrade any temptation to treat the midpoint as a planning figure** — it is a width, not a point. `[DOWNGRADED — ESTIMATE, LOW-MED; rests on a wage-ratio adjustment off a non-Ludhiana study × an [UNKNOWN] Ludhiana productivity]`
- **The "₹12/kg knitting" and "USD 20–100+/dozen" IndiaMART/trade fragments** are loose sanity-checks only — A correctly calls them "consistent, not confirmatory." **Do not let these harden into a quotable per-piece rate.** `[DOWNGRADED — ESTIMATE, LOW; trade-listing fragments, not a real per-FF-sweater linking quote]`
- **The whole set of ex-factory bands ($4–9 / $3.5–8 / $6–22+)** carries an *extra* downgrade this reconciliation must add that A could not: **no independent second analyst challenged them.** They are internally honest but **un-cross-examined**. Treat them as A's good-faith illustration, not as a reconciled two-source consensus. `[DOWNGRADED for process — single-analyst, B-side absent]`
- **The "yarn = 50–60%" split**, while load-bearing, rests on **one anchor (Vardhman FY20 = 54%) + industry norms** — already MED in T1, and it should not be cited as a measured cluster average. `[ESTIMATE, MED — preserved at T1's confidence, not promoted]`

---

## 4. Contested / uncertain points — the open ledger (floor, not ceiling)

**The binding blocker (both the cost model and the make-vs-buy economics turn on it):**
1. **Ludhiana per-piece knit + LINK job-work rate** (₹/piece at 7GG/12GG, 300/400 g) — **[UNKNOWN]; THE blocker.** No reliable public figure exists (confirmed across T1 Open Q#1, T3 Open Q#2, and A's fresh search). Lines 2–3 are ~**25–45% of the conversion subtotal** and are the lines we cannot price → the ex-factory bands are **illustrations, not a model.** **One on-the-ground quote run** ("₹/piece to knit FF panels and ₹/piece to link, at 7GG and 12GG, for 300 g and 400 g pullovers") from 2–3 job-workers collapses these to [ESTIMATE, MED] and tightens every band by roughly half. **The single highest-leverage data acquisition in the segment** — and it *also* settles the buy-vs-own-knitting boundary, because it's the number make-vs-buy compares against. `[UNKNOWN; unblocks on one quote run]`

**The structurally-real-but-not-per-unit-additive (named, held [UNKNOWN], correctly NOT added):**
2. **Friction / "speed-money" line** — episodic/amortised-to-near-zero per garment at capsule volume; a T1 §5c approval-gate cost, **mechanism not accusation, every amount [UNKNOWN] by design.** Named, not added. `[UNKNOWN by design]`
3. **Inverted-duty / GST working-capital drag on acrylic (A2)** — a **cash-conversion-cycle cost (T9), not a per-garment COGS line**; magnitude [UNKNOWN]. Flagged, not added. `[UNKNOWN magnitude]`

**The inputs that move the bands but aren't the blocker:**
4. **Real small-run garment/CMT MOQ at a named unit (~100–300 pcs)** — sets **fixed-cost amortisation per garment**; the lever that makes micro-volume genuinely more expensive per piece than the bands imply. `[ESTIMATE, MED, T2]`
5. **Per-style fixed costs** (sampling/PPS, tech-pack, lab dips, AQL inspection) amortised over MOQ — **pulls small-run per-garment cost up materially at low volume.** `[ESTIMATE, LOW]`
6. **Live 2025–26 yarn ₹/kg per fibre/count** — `[ESTIMATE]` bands on stale points (T2 §3a); premium rows need an importer quote.
7. **Landed used-machine quotes** (only relevant the moment make-vs-buy tips toward owning knitting — Chinese/Cixing first; ex-Stoll parts-via-Obertshausen diligence) — wide [ESTIMATE, LOW-MED] bands; the #1 input gap into any owned-line cost model. `[ESTIMATE, LOW-MED, T3]`
8. **Compliance-premium *base*** (how much labour the brand carries on-roll vs the job-worker) — sets what the ~15%+ actually multiplies. `[UNKNOWN]`
9. **PSPCL industrial power tariff + the job-worker's power share folded into the rate** — `[UNKNOWN]`, T1 Open Q#4.
10. **CAD/flat-knit programmer cost/availability** — `[ESTIMATE, MED]`; the make-vs-buy hinge on the *skill* side (T3 §6) — if owning knitting, this is a payroll line; if subcontracting, it's a contracted-rate [UNKNOWN].

**The reconciliation's own added uncertainty (process):**
11. **No independent challenge to A's cost bands or to the make-vs-buy boundary** — B's make-vs-buy memo was never written. The make-vs-buy *conclusion* is robust (three upstream segments converged), but the **adversarial pressure a normal A/B merge provides is absent here.** The neutralize critic downstream should treat §2's boundary and §3's bands as **reconstructed-consensus / single-analyst**, and press them harder than a two-source result would warrant. `[UNKNOWN — missing second-analyst challenge]`

---

## Bottom line (for the writer)

1. **Two halves, one of them reconstructed.** The cost model is Analyst A's (sound, honestly tagged). The make-vs-buy half is **reconstructed from T1/T2/T3** because `analyst_b_make_vs_buy.md` was never written — flag this so the critic presses it.
2. **Make-vs-buy verdict (HIGH-confidence, convergent): buy, don't make.** Own design+brand+US-import; subcontract knit+link+finish; buy yarn in-cluster (bought-dyed); rent the programmer; never own a dyehouse; never build a machine. In-source a step only at proven volume, linking last.
3. **Cost model: ex-factory ≠ landed ≠ P&L; contribution after CAC decides viability (T7/T9), not this number.**
4. **Yarn sets the level (well-anchored, ~$1–2.50/garment value/core), the [UNKNOWN] knit/LINK per-piece rate sets the uncertainty AND the make-vs-buy comparison.** It's the same missing number on both sides.
5. **Illustrative ex-factory bands — labelled, un-cross-examined:** A1 ≈$4–9, A2 ≈$3.5–8, A3 ≈$6–22+. A2 is the floor; A3 is yarn-dominated and least reliable.
6. **One quote run unblocks everything.** ₹/piece knit + ₹/piece link at 7GG/12GG, 300/400 g, from 2–3 job-workers → converts the illustration into a model and settles the buy-vs-own-knitting boundary in one stroke.
7. **Don't add what you can't price.** Friction/speed-money (mechanism, never accusation) and inverted-duty drag (T9 cash-cycle) are named and held [UNKNOWN].

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/mfg_economics/reconciled.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `SCOPE_GLOSSARY.md`; `ludhiana/DEEPDIVE.md`; `yarn/DEEPDIVE.md`; `machines/DEEPDIVE.md`; `mfg_economics/analyst_a_cost_model.md`
- **Input status (corrected post-build):** `mfg_economics/analyst_b_make_vs_buy.md` was absent at reconcile time but was **subsequently written** in the build wave. This reconcile merged Analyst A's cost model with make-vs-buy logic reconstructed from T1 §3/§6, T2 §2/§5, T3 §1–6, GLOSSARY §7/§9; the build wave then **cross-examined** that verdict with the real analyst-B (tested A/B — see Erratum). The bands stay wide because the **[UNKNOWN] knit+LINK rate** blocks precision, not for lack of review.
- **Delivered:** (1) a process note that B's make-vs-buy file is absent and what that costs (lost adversarial pressure, not a lost answer); (2) agreed/tagged anchors (boundary, yarn=50–60%, linker bottleneck, ~15%+ compliance, never-dye/never-build, HSN/duty constants); (3) the reconciled make-vs-buy boundary as a stage-by-stage ladder with the honest counter; (4) the cost model reconciled — what holds, what is downgraded ($1.5–4 conversion band, trade-fragment rates, the un-cross-examined ex-factory bands, the single-anchor 54% split); (5) the open ledger with the [UNKNOWN] knit/LINK rate as THE blocker and the process-[UNKNOWN] of B's absence.
- **Honesty discipline:** Ludhiana knit/link per-piece rate held [UNKNOWN] as the #1 blocker (never fabricated); A's bands carried at A's stated confidence with an *added* process-downgrade for B's absence; over-confident single figures (conversion midpoint, trade fragments) downgraded to widths; friction/speed-money kept as mechanism-not-accusation with amounts [UNKNOWN]; inverted-duty drag named as a T9 cash-cycle cost, not a per-unit add; premium-fibre (A3) row kept order-of-magnitude/LOW; the missing-second-analyst gap flagged as a first-class [UNKNOWN] for the neutralize critic.
- **Bottom line:** buy-don't-make at capsule scale (convergent, HIGH); yarn sets the level and the [UNKNOWN] knit/LINK rate sets the uncertainty on *both* the cost model and the make-vs-buy comparison; ex-factory bands A1≈$4–9 / A2≈$3.5–8 / A3≈$6–22+ are labelled, un-cross-examined illustrations; one job-worker quote run unblocks the whole segment.
</content>
</invoke>
