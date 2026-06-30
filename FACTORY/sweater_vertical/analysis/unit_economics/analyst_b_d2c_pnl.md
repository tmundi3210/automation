# D2C P&L, Pricing & Viability — Analyst B (T9 / unit_economics)

**Role:** the skeptic's contribution-margin-after-CAC model for the brand. This file answers the glossary's stated viability test (cross-team rule #8): **contribution margin after CAC, not gross margin, decides whether a micro D2C knitwear brand selling India-made sweaters into California survives.** It takes Analyst A's ex-factory bands (T4), bridges them to **landed COGS**, then builds the **D2C cost stack below gross margin**, computes **contribution after CAC per order**, the **AOV needed**, **break-even volume**, and the **honest verdict — including the configuration where the math does NOT close.**

**How to read the tags (binding, inherited T0–T4):** **[FACT]** = cited public source / true-by-definition. **[ESTIMATE, confidence]** = method + basis + confidence, carries the literal word *estimate*. **[UNKNOWN]** = not reliably known; a valid, frequent answer (coverage is a floor). Every CAC, conversion, return-rate, freight-allocation, and fixed-cost number here is an **[ESTIMATE]** unless a [FACT] source is cited. Duty rates are [FACT] (USITC), the 2025–26 surcharge layer is volatile and tagged.

> **The one sentence to remember.** *Gross margin is healthy on all three articles (≈75–86% over landed COGS), but contribution margin **after CAC** is where the line dies: at a realistic 2025–26 new-cohort apparel CAC of ~$45–70, **A2 (the $55 acrylic value driver) loses money on the very first order**, **A1 (the $75 cotton core) only works at CAC <~$38 and needs repeat purchase to clear LTV:CAC>3**, and **A3 (the $145 premium) is the only article that reliably pays for its own customer** — so the venture's stated "value driver" is its **biggest loss leader**, and the brand only closes by **inverting the line: lead with premium + bundles to lift AOV, treat A2 as a margin-dilutive volume bait, and hold blended CAC under ~$35.***

---

## 0. The boundary — what this number is, and the four it is NOT

This is the **contribution-margin-after-CAC P&L** — the honest per-order profit that must cover fixed cost. It sits **on top of** T4 (ex-factory) and a T9 landed bridge (built here, §1, because no separate T9 landed file exists yet). It is:

- **NOT ex-factory cost.** That is T4 (A1≈$4–9, A2≈$3.5–8, A3≈$6–22+ / garment, all `[ESTIMATE]`, blocked by the [UNKNOWN] knit+LINK job-work rate).
- **NOT gross margin alone.** Gross margin (revenue − landed COGS) looks great and is **a trap** — it ignores the four costs that actually eat D2C: payment, returns, ship-to-customer, and CAC.
- **NOT a forecast.** It is a **labelled arithmetic model** plugging benchmark `[ESTIMATE]` inputs into the glossary's contribution-margin identity. Real inputs need the T4 quote run **and** a live ad-account CAC.
- **NOT the working-capital model.** The cash-conversion trap (yarn-out months before revenue, dye-lot MOQ, ocean lead time, returns settling) is T4 §4 / the T9 cash cycle — a separate kill-risk that a profitable-per-order P&L can still hide. Flagged in §6, not modelled per-order.

**Critical correction carried into this file:** the glossary/T4 quote sweater duty as **"cotton ≈7%, wool ~16%."** The **7% is wrong for sweaters** — that is a different 6110 sub-line. The correct **MFN sweater rates** are below (§1), and they are materially higher, which moves landed COGS up versus what upstream assumed.

---

## 1. The landed-COGS bridge (ex-factory → US door) — corrected duty + the 2025–26 surcharge shock

Contribution starts from **landed COGS**, not ex-factory. Three things stack on T4's ex-factory band: **duty** (a % of customs value, so it scales with cost and with fibre), **freight/insurance/broker/drayage** (a per-unit allocation, brutal at micro-volume), and the **2025–26 tariff-surcharge layer** (new, volatile, and not in the glossary).

### 1a. Duty by article — the [FACT] correction

| Article | HTS line | **MFN duty** | Source |
|---|---|---|---|
| **A1 cotton-blend core** | 6110.20.20 (sweaters, of cotton, nesoi) | **16.5%** | `[FACT — USITC HTS / UNIS]` |
| **A2 fine-acrylic value** | 6110.30.30 (sweaters, of man-made fibres, nesoi) | **32%** | `[FACT — USITC HTS / faqs.org rulings]` |
| **A3 merino/lambswool premium** | 6110.11.00 (sweaters, of wool) | **16%** | `[FACT — USITC HTS / UNIS]` |

**The duty story flips the cost ranking.** Acrylic is the cheapest fibre ex-factory but carries **the highest US duty (32%)** — nearly double cotton's and wool's. So the "value" article is **penalised twice**: thin price ceiling *and* the heaviest tariff. India gets **no FTA preference** on 6110 (the "Free" column lists Australia, Korea, Israel, etc. — not India) `[FACT — USITC special-rate column]`.

### 1b. The 2025–26 surcharge layer — [ESTIMATE/volatile], the single biggest external risk to the model

The MFN rate is no longer the whole duty. As of mid-2026 the India-origin apparel surcharge picture is **unstable and changed three times in ~12 months**:
- IEEPA "reciprocal" tariff on India was **26% (Apr 2025) → cut to 18% (Nov 2025 interim deal) → struck down by the Supreme Court (≈Feb 2026, IEEPA held not to authorise tariffs)** `[FACT — White House actions; SCOTUS ruling, per trade-press reporting; confidence MED on exact dates]`.
- Replaced by a **Section 122 ~10% baseline, effective ≈Feb 2026, scheduled to lapse ≈late-July 2026** `[ESTIMATE/FACT-reported, MED — Section 122 caps at 150 days, so it is inherently temporary]`.
- **USTR opened Section 301 investigations on India (Mar 2026)** — a permanent, sector-targeted tariff could replace the lapsing baseline `[FACT — reported; outcome UNKNOWN]`.

**Net:** model a **base case with +10% surcharge** and a **stress case with +25%** (a plausible 301 outcome), and treat the **all-in India apparel duty as 26–57% depending on fibre and surcharge** — not the ~7–16% the glossary assumed. This is **not a footnote; it is a first-order driver of landed COGS** and the thing most likely to move between writing and launch.

### 1c. The de-minimis change — a [FACT] that closes one escape hatch

The **$800 de-minimis duty-free threshold was suspended for all countries (EO signed 30 Jul 2025, effective 29 Aug 2025)** `[FACT — White House EO; EY; NPR]`. **Consequence for this brand:** you **cannot** dodge duty by drop-shipping single sweaters direct from India to each US customer duty-free. Every unit is dutiable whether imported in a bulk container or as a parcel. The compliant path is **bulk-import to a US 3PL (Turlock/CA), then ship domestically** — which is what the model below assumes, and which adds the per-unit freight allocation in §1d.

### 1d. Per-unit logistics allocation `[ESTIMATE]`

Bulk small-LCL ocean India→CA + insurance + broker + drayage to a 3PL, allocated per garment: **~$1.5–3.5/unit** at a few-hundred-unit launch run (`[ESTIMATE, MED]`; falls with volume, spikes toward **$5+/unit if forced to air-freight a tiny urgent capsule** `[ESTIMATE, LOW-MED]`). This is the *inbound* leg; *outbound* ship-to-customer is a separate D2C line (§2).

### 1e. Landed COGS per article (base = exf midpoint + correct MFN duty + 10% Sec122 + $2.5 inbound freight)

| Article | T4 exf band | exf mid | Landed (MFN only) | **Landed (+10% Sec122) — BASE** | Landed (exf-hi + 25% surcharge + air) — STRESS |
|---|---|---|---|---|---|
| **A1 cotton** | $4–9 | $6.5 | $10.07 | **≈ $11** | ≈ $16–17 |
| **A2 acrylic** | $3.5–8 | $5.8 | $10.09 | **≈ $11** | ≈ $16–17 |
| **A3 merino** | $6–22+ | $14 | $18.74 | **≈ $20** | ≈ $33+ |

`[ESTIMATE, LOW-MED — inherits T4's [UNKNOWN] knit+LINK rate (the dominant uncertainty) AND the volatile surcharge; bands not points]`. **Note A2's landed cost ≈ A1's despite acrylic being cheaper ex-factory** — the 32% duty erases acrylic's fibre-cost advantage at the US door. That single fact reshapes the whole pricing argument.

---

## 2. The D2C cost stack BELOW gross margin — the four costs that actually decide it

Gross margin over landed COGS is **healthy and misleading**: A1/A2 at MSRP $55–75 on ~$11 landed = **80–85% gross**; A3 at $145 on ~$20 = **86% gross**. Now subtract what gross margin ignores:

| Cost line | Benchmark `[ESTIMATE]` unless noted | Basis / source |
|---|---|---|
| **Payment processing** | **2.9% + $0.30** per order | `[FACT — Shopify Payments Basic plan, 2026]`. +0.6–2% if PayPal/Stripe gateway. Refunds usually do **not** return the fee → cost sits on gross sales. |
| **Returns reserve** | **~22% of orders returned** (model), realistic range **20–30%** | `[ESTIMATE, MED-HIGH — Capital One Shopping 24.5% all-retail 2025; apparel 20–40%; clothing-specific studies ~26%; online apparel 22% vs 6% in-store]`. Fit/size = up to 70% of apparel returns → a fine-gauge fit-sensitive sweater is **high-return by category**. |
| → returns sub-cost: **inbound return shipping + processing** | **~$10–12 per returned unit** | `[ESTIMATE, MED]` prepaid label + restocking labour. |
| → returns sub-cost: **markdown/dead-stock loss** | **~20% of returned units unsellable** → full COGS lost | `[ESTIMATE, LOW-MED]` |
| **Ship-to-customer (outbound)** | **~$7–8/order subsidised** (free-ship is table stakes in apparel) | `[ESTIMATE, MED]` ground parcel for a ~400 g sweater + packaging. |
| **Platform / app fees** | Shopify **~$39/mo + ~$100–150/mo apps** (reviews, email, returns portal) | `[FACT base plan; ESTIMATE app stack]` — a **fixed** cost (→ §5), small per-order at volume. |
| **CAC (blended, paid social)** | **$45–75** established apparel DTC; **+~35% CPM penalty for a NEW 2025–26 cohort**; worst-quartile **$187**; best-quartile **$42** | `[ESTIMATE, MED — Meta apparel CPM ~$9–23; DTC fashion CAC $45–75 and rising ~15%/yr; new-cohort penalty; 35-brand range $42–187]`. **A brand-new India-made knitwear brand with no relevance history sits in the WORSE half → model CAC $45–70, not $25.** |

**Why CAC is the governing term:** every other line is single-digit dollars or a small %. CAC is **$45–70 on a $55–145 sale** — it is the same order of magnitude as the entire gross margin. This is the glossary's whole point made literal.

---

## 3. Contribution margin after CAC, per order — the model

Per single-unit order: `contribution_post_CAC = MSRP·(1−r) − landed·(1−r) − landed·r·0.20[dead] − (2.9%·MSRP+$0.30)[pay] − $7[ship] − r·$10[return-logistics] − CAC`, with r = 22%.

### 3a. MSRP ladder + base-case results

| Article | **MSRP** | Landed | Gross margin | Contribution **pre-CAC** | Post-CAC @ **$25** | Post-CAC @ **$45** | Post-CAC @ **$70** |
|---|---|---|---|---|---|---|---|
| **A2 acrylic value** | **$55** | $11 | 80% | **$22.74** (41%) | **−$2.26** | **−$22.26** | **−$47.26** |
| **A1 cotton core** | **$75** | $11 | 85% | **$37.76** (50%) | **+$12.76** (17%) | **−$7.24** | **−$32.24** |
| **A3 merino premium** | **$145** | $20 | 86% | **$82.92** (57%) | **+$57.92** (40%) | **+$37.92** (26%) | **+$12.92** (9%) |

`[ESTIMATE — arithmetic on the §2 benchmark inputs; confidence MED on method, LOW-MED on absolute levels because landed COGS and CAC are both bands]`.

**Read this table as the core finding:**
- **A2 (the stated "value driver") never makes money once CAC is realistic.** Its pre-CAC contribution is only **$22.74**, so even a **$25** CAC puts it underwater. At the realistic new-cohort CAC of $45–70 it loses **$22–47 per order**. **A2 is a loss leader, not a value driver.** The 32% acrylic duty + low price ceiling is the squeeze.
- **A1 (cotton core) is marginal.** It clears a **$25** CAC with $12.76 (17%), but **breaks even at CAC ≈ $38** and is **negative at the realistic $45+**. It only survives if CAC is held low **and** customers repeat (§3c).
- **A3 (merino premium) is the only robust earner** — positive even at a **$70** CAC. The premium price, not the premium fibre, is what pays for the customer.

### 3b. CAC ceiling per article (the max CAC before the first order loses money)

| Article | **CAC ceiling (single purchase)** |
|---|---|
| A2 $55 | **≈ $23** |
| A1 $75 | **≈ $38** |
| A3 $145 | **≈ $83** |
| 2-unit bundle (AOV $130) | **≈ $69** |

Against a realistic **$45–70** new-cohort CAC: **A2 fails, A1 fails-to-marginal, A3 and bundles pass.**

### 3c. The LTV:CAC > 3 survival test (glossary rule, the real bar)

A single-order contribution is not the bar; the glossary's bar is **LTV:CAC > ~3**. Allowable CAC = (pre-CAC contribution × expected repeat orders) / 3:

| Article | repeats 1.0 | repeats 1.5 | repeats 2.5 |
|---|---|---|---|
| A2 $55 | CAC < **$8** | < $11 | < $19 |
| A1 $75 | CAC < **$13** | < $19 | < $31 |
| A3 $145 | CAC < **$28** | < $41 | < **$69** |
| bundle $130 | CAC < $23 | < $34 | < **$57** |

`[ESTIMATE — repeat rate is [UNKNOWN] for a brand that doesn't exist; apparel repeat is typically modest]`. **At a 3:1 bar, A2 is essentially impossible** (needs CAC <$8–19, unreachable in paid social). **A1 needs strong repeat (2.5×) just to tolerate a $31 CAC.** **Only A3 + bundles clear 3:1 at realistic CAC**, and only with some repeat.

---

## 4. The AOV lever — the one move that makes the line close

Single-unit A2/A1 orders can't carry CAC; **the fix is AOV**, because **CAC and outbound shipping are charged per ORDER, not per unit.** Bundle two units and you amortise one CAC and one ship over a bigger basket:

| Configuration | AOV | Contribution pre-CAC | Post-CAC @ $25 | @ $45 | @ $70 |
|---|---|---|---|---|---|
| **2-unit bundle** (2 × $65 avg, $11 landed each) | **$130** | **$68.56** | **+$43.56** (34%) | **+$23.56** (18%) | **−$1.44** |

`[ESTIMATE — same method, one CAC/one ship across 2 units]`. A **$130 AOV survives CAC up to ~$69** and clears 3:1 at ~$57 with modest repeat — versus a single $55 A2 that dies at $23. **The required AOV to make the model robust is roughly $120–150** (≈2 units or 1 premium unit), which is **exactly the A3 price point or a deliberate 2-piece bundle.** This is the central design instruction to T7/T8: **the line must be sold in baskets ≥ ~$120, not as single $55 sweaters.**

---

## 5. Break-even volume — fixed nut vs contribution per order

Fixed monthly cost for a lean micro-brand `[ESTIMATE, MED]`: Shopify+apps (~$190) + a US 3PL minimum + minimal brand/design/owner draw + overhead → model **$1,500 (bootstrap) / $3,000 (modest) / $6,000 (with any salary)** per month.

| Fixed/mo | @ $10 contrib/order | @ $20 | @ $30 |
|---|---|---|---|
| **$1,500** | 150 orders | 75 | 50 |
| **$3,000** | 300 | 150 | 100 |
| **$6,000** | 600 | 300 | 200 |

`[ESTIMATE]`. **Interpretation:** at a realistic **blended contribution of ~$20/order** (a premium-led, bundle-heavy mix at CAC ~$45) and a **$3,000** lean nut, break-even is **~150 orders/month (~$20–25k revenue/mo)**. That is achievable but **not trivial for a no-name brand** — and it **collapses to needing 300+ orders/mo the moment CAC drifts to $70 or contribution falls to $10**, which is the realistic risk. Break-even is **sensitive to exactly the two softest inputs (CAC and contribution)**, so it is a knife-edge, not a comfortable margin.

---

## 6. The skeptic's verdict — where it does NOT close, and where it does

### Where the math FAILS (state this first, loudly):

1. **The stated line plan as written loses money.** A "value-led" launch leading with the **$55 A2 acrylic** at realistic CAC is **structurally unprofitable per order** (−$2 to −$47). The article the upstream plan calls the "value driver" is the article that **drains cash fastest**. If the owner launches A2-first on paid social, the brand bleeds.
2. **A1 at $75 is marginal and CAC-fragile.** It needs CAC <$38 *and* repeat purchase. New-cohort 2025–26 apparel CAC ($45–70) sits **above** its ceiling. A1-led also fails unless CAC is suppressed below benchmark.
3. **The 32% acrylic duty + suspended de-minimis + volatile India surcharge** can swing landed COGS 50%+ (stress column: A2 landed $11→$16–17). A 301-driven permanent tariff replacing the lapsing Section 122 baseline is a **live kill-risk dated to ~July 2026**, exactly the launch window.
4. **Returns are category-brutal and fit-driven.** A fine-gauge fit-sensitive sweater sold sight-unseen runs **22–30% returns**; each return costs ~$10–12 + 20% dead-stock. A returns-spike season alone can erase a thin contribution.
5. **The working-capital trap is unmodelled and can bankrupt a per-order-profitable brand** (T4 §4): dye-lot MOQ forces 500+ units/colour of cash out, ocean lead time + sell-through + returns settlement mean **cash is gone for months before it returns** — the classic profitable-on-paper D2C death.
6. **LTV:CAC > 3 fails for two of three articles** at realistic CAC. Only A3/bundles clear it, and only with repeat that an unproven brand cannot assume.

### Where the math CLOSES (the narrow viable configuration):

The brand is viable **only if the line is inverted from the upstream plan**:
- **Lead with A3 premium ($145) and 2-unit bundles ($120–150 AOV)**, not single A2 sweaters. These are the only configs with positive contribution at CAC $45–70 and the only ones clearing 3:1.
- **Hold blended CAC under ~$35** — which for a no-name brand means **CAC cannot come mostly from cold paid social** (where new cohorts pay $45–70+). It must lean on **organic/community/affiliate/owner-audience** (35–40% paid max), or the model doesn't close. This is a **go-to-market precondition, not a nicety.**
- **Use A2 deliberately as a low-margin volume/AOV filler inside bundles**, never as a standalone hero — it adds basket size without being asked to carry a CAC alone.
- **Keep fixed cost bootstrap-lean (~$1,500–3,000/mo)** so break-even is ~75–150 orders/mo, reachable.
- **Price A3 at $145+ and defend the premium** with the merino/fully-fashioned/anti-pill story — the premium price is the only thing in the whole model that comfortably pays for a customer.

### The honest bottom line

**For a micro D2C knitwear brand selling India-made sweaters into California, the math does NOT close in the "affordable value sweater on paid social" configuration — that is a money-loser, primarily because CAC ($45–70) and the 32% acrylic duty together exceed what a $55 sweater can carry.** It **does** close in a **narrow premium-led, high-AOV, low-paid-CAC configuration** (A3 + bundles, AOV ~$130, blended CAC <$35, lean fixed cost). The venture is **viable as a small premium/community brand, not as a value/performance-marketing brand.** The single most dangerous mistake the upstream plan invites is **leading with A2** — the article that, after CAC and duty, **cannot pay for itself.**

---

## 7. Legal / honesty flags (binding — do NOT wave through)

- **Right of publicity + Lanham Act false-endorsement (HIGH risk if T7 names a real athlete/celebrity).** California protects the unauthorized commercial use of a living *or deceased* person's name, voice, signature, photo, or likeness; damages include the infringer's profits `[FACT — Cal. Civ. Code §3344/§3344.1; The Fashion Law]`. A real athlete's name on a sweater = **right-of-publicity + potential federal false-endorsement exposure** `[FACT — Lanham Act §43(a)]`. **Do not let T7's "athlete/event/meme-driven naming" use a real person's name/likeness without a license.** Memes/events can carry their own trademark risk; clear marks before printing.
- **Duty-misclassification / blend-drift exposure.** Fibre composition sets the HTS line and duty (cotton 16.5% / MMF 32% / wool 16%); a supplier under-blending to cut cost can make the brand (importer of record) **mis-declare duty** — write blend ± tolerance into the PO with fibre-composition verification (carried from T2 §1d). `[FACT — duty lines; ESTIMATE — drift risk]`
- **The 7%-cotton-sweater figure in the glossary/T4 is wrong** and must not be used in any margin model — sweaters of cotton are **16.5%** (6110.20.20), not 7%. Corrected here `[FACT]`.
- **Tariff numbers are dated and volatile.** Every surcharge figure (IEEPA/Section 122/301) is reported, moving, and tagged `[ESTIMATE/volatile]`; re-confirm at the actual import date — do not harden any single rate into a forecast.

---

## Bottom line for the owner

1. **Gross margin (80–86%) is a vanity number; contribution-after-CAC is the truth — and it kills the value article.** A2 acrylic at $55 loses money on the first order at any realistic CAC; A1 cotton at $75 is marginal and CAC-fragile; **only A3 merino at $145 and 2-unit bundles reliably pay for a customer.**
2. **CAC ($45–70 for a new 2025–26 apparel cohort) is the governing term** — the same order of magnitude as the whole gross margin. The brand only closes if **blended CAC is held under ~$35**, which means **organic/community/affiliate-led acquisition, not cold paid social.**
3. **Invert the line plan:** lead premium + bundles to push **AOV to ~$120–150**; use A2 as a margin-dilutive basket-filler, never a standalone hero. **The required AOV is the single most important pricing instruction.**
4. **Landed COGS is higher and more volatile than upstream assumed:** correct sweater duty is **cotton 16.5% / acrylic 32% / wool 16%** (not ~7%), **de-minimis is gone** (every unit dutiable; bulk-import to a CA 3PL), and the **India surcharge layer (Section 122 ~10% lapsing ~Jul 2026, possible 301 replacement)** can swing all-in duty to **26–57%** — a launch-window kill-risk.
5. **Break-even is a knife-edge:** ~75–150 orders/mo at a lean $1,500–3,000 nut **if** contribution holds ~$20/order; it doubles the moment CAC drifts to $70 or contribution falls to $10.
6. **The working-capital trap (T4 §4) is the unmodelled killer** — a per-order-profitable brand can still die on the months-long cash-out of dye-lot MOQ + ocean lead time + returns settlement. Per-order viability is necessary, not sufficient.
7. **Legal:** no real athlete/celebrity name without a license (right of publicity + Lanham false-endorsement); write blend tolerance into POs (duty-misclassification exposure). Flagged, not waved.

---

## Open questions — what only live data closes (floor, not ceiling)

1. **Live blended CAC from a real test ad account** — `[UNKNOWN]`; the governing variable. Benchmarks ($45–70 new cohort) are bands; one $1–2k test spend collapses them.
2. **Real return rate for THIS fit/fabric** — `[ESTIMATE, 20–30%]`; fine-gauge fit-sensitivity could push higher. Needs first-cohort data.
3. **Repeat-purchase rate / true LTV** — `[UNKNOWN]` for a non-existent brand; sets whether A1 ever clears 3:1.
4. **The T4 [UNKNOWN] knit+LINK job-work rate** — still the dominant uncertainty *inside* landed COGS; one quote run halves the ex-factory band and tightens every number here.
5. **The India apparel surcharge at the actual import date** — `[UNKNOWN/volatile]`; Section 122 lapses ~Jul 2026, 301 outcome pending. Re-confirm before committing inventory.
6. **Real per-unit inbound freight at the brand's actual launch volume/mode** — `[ESTIMATE, $1.5–5]`; LCL vs air swings it.
7. **Actual fixed monthly nut** (3PL minimums, app stack, any draw) — `[ESTIMATE, $1,500–6,000]`; sets break-even order count.
8. **A3 premium yarn landed cost** — inherits T2/T4's `[ESTIMATE, LOW]` order-of-magnitude merino price; the premium article carries the model, so its COGS needs an importer quote before it's load-bearing.

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/unit_economics/analyst_b_d2c_pnl.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `SCOPE_GLOSSARY.md`; `mfg_economics/DEEPDIVE.md`; `yarn/DEEPDIVE.md`; `ludhiana/DEEPDIVE.md`
- **Web-grounded [FACT/ESTIMATE]:** USITC HTS sweater duty (6110.20.20 cotton 16.5%, 6110.30.30 MMF 32%, 6110.11 wool 16%); de-minimis suspension (EO 30 Jul 2025 → eff 29 Aug 2025, all countries); India surcharge sequence (IEEPA 26%→18%→SCOTUS-struck; Section 122 ~10% to ~Jul 2026; USTR 301 Mar 2026) [volatile]; Shopify Payments 2.9%+$0.30; apparel return rates (Capital One 24.5% all-retail 2025; apparel 20–40%, ~26% clothing-specific); DTC apparel CAC $45–75 + new-cohort +35% CPM, worst-quartile $187; Meta apparel CPM ~$9–23; right of publicity (Cal. Civ. Code §3344) + Lanham §43(a) false endorsement.
- **Delivered (role coverage):** (1) MSRP ladder (A2 $55 / A1 $75 / A3 $145) + gross margin over landed COGS (80–86%); (2) the full D2C cost stack below gross — payment, returns reserve, ship-to-customer, CAC, platform fees, all benchmarked; (3) contribution-after-CAC per order per article + CAC ceilings + LTV:CAC test + the AOV needed (~$120–150 / 2-unit bundle); (4) break-even volume table + the honest verdict showing **explicitly the configuration where it FAILS (A2-led / single-unit / cold paid social) vs where it CLOSES (premium-led, high-AOV, low blended CAC, lean fixed).**
- **Honesty discipline:** all CAC/return/conversion/freight/fixed-cost inputs tagged `[ESTIMATE]` with method+basis+confidence; duty rates `[FACT — USITC]`; the glossary's 7%-cotton figure flagged as wrong and corrected; the 2025–26 surcharge layer tagged `[ESTIMATE/volatile]` and called a dated kill-risk; landed COGS carries T4's `[UNKNOWN]` knit+LINK rate as the dominant internal uncertainty; working-capital trap named as an unmodelled killer; right-of-publicity / Lanham legal risk flagged HIGH and explicitly NOT waved through; 8 open questions carried as a floor.
- **Bottom line:** the math does **not** close for a value/paid-social config — A2 acrylic at $55 loses money on the first order (low ceiling + 32% duty + $45–70 CAC); it **does** close only in a narrow premium-led, bundle-driven (AOV ~$130), low-blended-CAC (<$35), lean-fixed-cost config where A3 carries the line; **invert the upstream plan, lead premium, never lead with A2, and acquire customers off cold paid social.**
