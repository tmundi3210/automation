# Unit Economics — DEEPDIVE (T9): Landed Cost, Duty Reality & the Contribution-after-CAC Viability Verdict

**For:** the owner of the AI-designed D2C knitwear brand (Ludhiana yarn → manufacture → 3-article line sold D2C in California / Turlock).
**What this file is:** the owner-facing synthesis of the T9 unit-economics work — the **per-article landed-cost stack**, the **duty reality** (especially the **32% MMF duty on acrylic** that flips A2's attractiveness), the **de-minimis current state**, and the **contribution-margin-after-CAC P&L** with a **blunt go / no-go verdict**. This is the segment that owns whether the venture is viable. It hands a clear go/no-go-conditions read to **T10** (the decision brain).
**Built from:** `unit_economics/reconciled.md` (the merge of Analyst A — landed cost; Analyst B — D2C P&L), inheriting T4 (mfg economics), T2 (yarn), T1 (Ludhiana).

**The 3 articles (from T8 line plan):** **A1** = cotton-blend 12-month core; **A2** = fine-acrylic value; **A3** = merino/lambswool premium capsule.

**Honesty tags (binding, travel with every claim):** **[FACT]** = cited public source / true-by-definition. **[ESTIMATE, conf]** = method + basis + confidence, carries the literal word *estimate*. **[UNKNOWN]** = genuinely not known; a valid, frequent answer (coverage is a floor, not a ceiling). **Untagged** = arithmetic identity. **[DOWNGRADED]** = a claim weakened from the strength one analyst gave it.

---

> ## The one paragraph to remember
> *Gross margin on all three articles looks fantastic (80–86% over landed cost) and **it is a trap**. The number that decides survival is **contribution margin after CAC**, and on that number the venture's own stated "value driver" is its **biggest loss-maker**. Two policy facts dominate the cost side: the **32% US duty on acrylic (A2)** — roughly double cotton's 16.5% and wool's 16% **[FACT — USITC]** — which **erases acrylic's factory-gate cost lead at the border**, and the **death of the <$800 de-minimis exemption on Aug 29, 2025 [FACT, dated]**, which kills the "ship each order direct from Ludhiana duty-free" model and forces a **bulk-importer → California 3PL → domestic-ship** operation. On the demand side, a realistic **2025–26 new-brand apparel CAC of $45–70 [ESTIMATE]** is the same order of magnitude as the entire gross margin — so a **$55 acrylic sweater loses money on the first order**, a **$75 cotton core only works at CAC < ~$38 with repeat purchase**, and **only the $145 merino premium and 2-unit bundles (AOV ~$120–150) reliably pay for their own customer.** The verdict: **viable as a small premium/community brand sold off cold paid social; NOT viable as a value/performance-marketing brand.** The single most dangerous move the upstream plan invites is **leading with A2.***

---

## 1. The landed-cost stack — how a Ludhiana sweater becomes a California COGS number

Landed cost = the true per-unit cost at our US receiving door; it becomes **COGS-to-stock**. It takes T4's **ex-factory** band and stacks the India→California import chain on top:

```
EX-FACTORY (T4)  →  FOB Nhava Sheva / Mundra (+ Ludhiana→port inland haulage, export clearance)
   →  ocean (or air) freight + marine insurance
   →  US DUTY = MFN ad-valorem (by HTS/fibre)  +  India surcharge (Section 122 / IEEPA layer)
   →  MPF (0.3464%, capped $651.50)  +  HMF (0.125%, ocean only)
   →  customs broker entry + bond + ISF + Importer-of-Record setup (amortised per unit)
   →  drayage / last-mile to the CA 3PL
   =  LANDED COST per garment  →  COGS-to-stock
```

**Duty is applied to the ex-factory (≈FOB transaction) value, not the freight-inflated value** — US apparel entries use transaction value, with international freight generally excluded if separately stated. **[FACT — US customs valuation].** This matters: duty scales with fibre and with cost, not with how we ship.

### 1a. The per-article landed stack (central case: bulk ocean, ~10% India surcharge)

| Line | A1 cotton core | A2 acrylic value | A3 merino premium |
|---|---|---|---|
| **Ex-factory (T4)** `[ESTIMATE]` | $4–9 | $3.5–8 | $6–22+ |
| **MFN duty rate** `[FACT — USITC]` | **16.5%** (6110.20.20) | **32%** (6110.30.30) | **16%** (6110.11.00) |
| **+ India surcharge (central)** `[ESTIMATE, MED]` | +10% | +10% | +10% |
| **= total duty %** | **~26.5%** | **~42%** | **~26%** |
| duty $ (on ex-factory) | ~$1.06–2.39 | ~$1.47–3.36 | ~$1.56–5.72+ |
| freight + MPF/HMF + broker/bond/drayage (bulk) `[ESTIMATE]` | ~$0.8–2.5 | ~$0.8–2.5 | ~$0.9–2.8 |
| **LANDED band / garment** | **≈ $6–14** | **≈ $6–14** | **≈ $9–31+** |
| **central planning point** `[DOWNGRADED — illustration, not a quote]` | **~$11** | **~$11** | **~$20** |

`[ESTIMATE, LOW-MED — A1/A2; LOW — A3]`. The bands are wide for two honest reasons, not sloppiness: **(a) T4's ex-factory band is itself blocked by the [UNKNOWN] knit+LINK job-work rate** — the dominant uncertainty inside the whole stack — and **(b) the India surcharge is volatile** (§3). The central ~$11/$11/$20 points are **band-centre illustrations**, not measured COGS; treat them as planning anchors, not quotes.

### 1b. The freight line is a volume curve, not a number

Per-unit inbound freight **falls in steps as volume rises** — there is no single right figure:

| Mode | Rate band (India → LA/Oakland, 2026) `[FACT — rate bands]` | Per garment | When |
|---|---|---|---|
| **Ocean FCL (40ft)** | ~$3,500–7,800 / container (~5,000–10,000 sweaters) | **~$0.5–1.5** | the right answer at maturity/bulk |
| **Ocean LCL** | ~$30–180 / CBM + ~$150–300 fixed CFS/doc/ISF fees | **~$1–6** + fixed drag | the first small bulk runs |
| **Air (consolidated)** | ~$5–8/kg (a sweater ~0.3–0.5 kg) | **~$1.5–4** | samples / urgent restock only |
| **Air express courier** | ~$8–12/kg | **~$3–6** + parcel fees | samples / PPS only — never bulk |

**Plan launch at ~$1.5–3.5/unit (first LCL run); model maturity at ~$0.5–1.5/unit (FCL).** `[ESTIMATE, MED — reconciled as a curve, not a point]`. The per-order air-from-India route (the dead de-minimis model) is **~$3–6/garment of freight alone before any duty** — one more reason it is uneconomic.

### 1c. Bulk beats per-order on the fee lines too (independent of duty)

- **MPF (FY2026):** 0.3464% of entered value, **min $33.58 / max $651.50 per formal entry.** **[FACT — CBP].** The **$651.50 cap** is the whole story: a $50k bulk entry hits the cap → **~$0.13–0.65/garment** at 1,000–5,000 pieces; a single ship-direct parcel pays the **$33.58 floor on one garment** → murderous per unit.
- **HMF:** 0.125% of cargo value, **ocean only, no cap.** **[FACT].** ~$0.006 on a $5 sweater — trivial; air is HMF-exempt.
- **Broker entry ~$150–400/entry + ISF ~$35–75; continuous bond ~$400–1,200/yr** (beats single-entry above ~3 shipments/yr) **or single-entry ~$75–275; IOR setup ~$50–150 one-time.** **[FACT — 2026 broker reportage].** Amortised over a bulk entry: **~$0.10–0.80/garment**; dominant per-unit **only** if you do many tiny entries (i.e. the ship-direct model again).

**The fee structure *alone* — before a cent of duty — rewards bulk and punishes per-order.** This is what makes the de-minimis death (§4) a clean model-flip rather than a marginal cost.

---

## 2. The duty reality — the 32% MMF wall that flips A2

**This is the headline of the whole landed analysis.** At the factory gate the line ranks A2 (acrylic) cheapest → A1 (cotton) → A3 (merino) dearest. **Duty inverts the cheap one into the most-punished one:**

| Article | HTS line | MFN base duty | What it does |
|---|---|---|---|
| **A2 fine-acrylic value** | 6110.30.30 (man-made fibre) | **32%** | **[FACT — USITC; CBP ruling N272437]** — the highest, ~2× the others |
| **A1 cotton-blend core** | 6110.20.20 (cotton, nesoi) | **16.5%** | **[FACT — USITC]** — the structurally friendliest |
| **A3 merino/lambswool premium** | 6110.11.00 (wool) | **16%** | **[FACT — USITC]** — lowest rate, but on the highest value |

**The mechanism, in dollars:** on a $6 ex-factory acrylic sweater, MMF duty alone is **~$1.92**, versus **~$0.99** on a same-priced cotton garment. Acrylic's ~$0.5–1 ex-factory saving over cotton is **almost exactly eaten by the extra ~$1 of duty** — so **landed, A1 and A2 sit in the same ~$6–14 band, and acrylic's entire reason to exist (it's cheaper) does not survive the US border.** **[FACT on the three rates; arithmetic on the inversion — both analysts converge].**

> **Re-ranking verdict:** factory-gate **A2 < A1 < A3** becomes, landed, **A1 ≈ A2 < A3**, with **cotton (A1) the cost-and-duty-friendly anchor** and **acrylic (A2) no longer meaningfully cheaper than cotton.** This is a finding T4 (factory-only) and T2 (yarn-only) structurally could not see. `[ESTIMATE, MED-HIGH on the re-rank; FACT on the duty spread driving it]`

### 2a. Two upstream corrections this layer pushes back

1. **The glossary/T4 "cotton ≈7%, wool ~16%" figure is WRONG for sweaters.** Sweaters of cotton are **16.5%** (6110.20.20), not 7% — the 7% is a different 6110 sub-line. **Both analysts independently caught this. [FACT — correction].** Every upstream margin model using ~7% cotton duty must be corrected up; the venture's true cotton duty is more than double what the plan assumed.
2. **India gets no FTA preference on heading 6110** — the "Free" column lists Australia, Korea, Israel, etc., not India. **[FACT — USITC special-rate column].** There is no tariff shortcut by virtue of sourcing in India.

### 2b. The chief-weight duty cliff — a blend ratio is a tariff switch

Classification is by the **fibre of chief weight by mass.** A 55% cotton / 45% acrylic A1 garment is "cotton" (16.5%); flip to **55% acrylic and the same sweater becomes MMF at 32% — its duty nearly doubles.** **[FACT — chief-weight rule].** As **Importer of Record the brand owns this liability** — a supplier quietly under-blending to cut yarn cost can push a "cotton-blend" past 50% MMF and leave the brand mis-declaring duty. **Write blend % + tolerance + a fibre-composition / duty-classification check into every PO.** `[FACT — chief-weight rule + IOR liability; ESTIMATE — drift risk]`

---

## 3. The India surcharge — a volatile band on top of MFN (the most movable input)

Since 2025 the US layers a **country-specific surcharge additively on top of MFN** for India: **total duty ≈ MFN base + India surcharge**, both on entered value. **[FACT — additive stacking].** It is **fibre-blind** — it hits all three articles equally, so it does **not** change the A1-vs-A2 ranking (the 32% MMF base already did that), but it **raises the whole stack** and makes the de-minimis loss more painful.

**The timeline (read the dates — this is why it's a band, not a point):**

| Date | India apparel surcharge | All-in on a cotton sweater | Tag |
|---|---|---|---|
| Apr 2, 2025 | 25% reciprocal | ~41.5% | [FACT] |
| **Aug 27, 2025** | **+25% penal → 50% peak** | **~66.5% (peak)** | [FACT — realised] |
| ~Feb 6, 2026 | interim deal cuts to 18% | ~34.5% | [FACT — interim] |
| **Feb 20–24, 2026** | SCOTUS strikes IEEPA basis → **~10% Section 122** | **~26.5%** | [FACT — ruling; ESTIMATE on date precision] |
| **mid-2026 (now)** | **~10% Section 122, under appeal, sunsets ~Jul 24, 2026** | **~26.5% cotton / ~42% acrylic / ~26% wool** | **[FACT current ~10%; UNKNOWN forward]** |

**The honest read:** the **current ~10% and the realised 50% spike are durable [FACT]; the exact transition dates carry only MED confidence [DOWNGRADED from fully-FACT]; the forward level is a genuine [UNKNOWN].** Two live forward risks both land on the launch window:
- The **Section 122 baseline statutorily sunsets ~Jul 2026** (Section 122 caps at 150 days — inherently temporary).
- The **USTR opened a Section 301 investigation on India (Mar 2026)** — a **permanent, sector-targeted tariff could replace the lapsing baseline.** Outcome **[UNKNOWN].**

> **Model it as a 0–15% band (central ~10%) PLUS an explicit 25–50% spike scenario** — because the spike already happened once and held for ~5 months. **At the Aug-2025 50% peak the same garments landed at A1 ~66.5% / A2 ~82% / A3 ~66% total duty — an acrylic sweater's duty alone nearly equalled its ex-factory cost.** **The venture must survive a return to 25–50%, not just today's 10%.** `[FACT that the spike happened; ESTIMATE, MED that it could recur]`. **Re-confirm the rate at every PO; never harden a single forward number into a forecast.**

---

## 4. De-minimis is dead — the model-flip (the most important structural finding)

**What de-minimis was, and why the original concept leaned on it.** Section 321 (19 USC 1321) historically let shipments ≤ $800 enter duty-free with minimal data, once per person per day. **[FACT].** That single rule was the magic lever the "knit in Ludhiana, ship each order direct, hold no US inventory" concept silently assumed: every order is one parcel well under $800 → **0% duty, no broker, no formal entry.**

**What happened.** **Executive Order 14324 (signed Jul 30, 2025) suspended duty-free de-minimis for ALL countries — effective 12:01 a.m. EDT Aug 29, 2025, India included.** **[FACT, dated — EO 14324; Federal Register 2025-16802; CBP fact sheet].**

**Current state (binding):** **there is NO de-minimis duty-free exemption for shipments from India.** A direct parcel from Ludhiana must now be entered and **pays full MFN duty (16–32% by fibre) + the India surcharge** — with no $800 shield.

> **The model-flip (a structural finding, not a cost tweak):**

| Model | Pre-Aug-2025 (de-minimis alive) | Post-Aug-2025 (now) |
|---|---|---|
| **Ship-direct per order from India** | 0% duty, no broker — the magic lever | **Full duty + surcharge on every parcel**, brokered entry per parcel, MPF floor per parcel, slow air freight per unit — **economically punished** |
| **Bulk import → US 3PL → domestic ship** | no shield benefit at bulk | **The new default** — duty paid once, MPF capped at $651.50, freight amortised, cheap/fast domestic last-mile |

**The venture must be a bulk Importer of Record → California (Turlock) 3PL → domestic ship.** This changes the operating model, the working-capital profile (you now pre-commit to bulk inventory — §6), and where the brand carries risk. **The zero-inventory ship-direct idea the concept assumed no longer exists.** `[FACT on the rule change; ESTIMATE, HIGH on the model-flip]`

---

## 5. The contribution-after-CAC P&L — where gross margin lies and the venture actually lives or dies

Landed cost is **COGS-to-stock, not the viability number.** Per the binding glossary rule #8, **contribution margin after CAC decides D2C survival.** Gross margin over landed COGS is **healthy and misleading** — A1/A2 at MSRP $55–75 on ~$11 landed = **80–85% gross**; A3 at $145 on ~$20 = **86% gross**. Then subtract the four costs gross margin ignores.

### 5a. The four costs below gross margin (all benchmarks `[ESTIMATE]` unless cited)

| Cost line | Benchmark | Basis |
|---|---|---|
| **Payment processing** | **2.9% + $0.30** per order | `[FACT — Shopify Payments 2026]`; refunds don't return the fee |
| **Returns reserve** | **~22% of orders** (range 20–30%) + **~$10–12/return** + ~20% dead-stock | `[ESTIMATE, MED-HIGH]` — fit-driven; a fine-gauge fit-sensitive sweater is high-return by category |
| **Ship-to-customer (outbound)** | **~$7–8/order** (free-ship is table stakes) | `[ESTIMATE, MED]` |
| **CAC (blended, paid social)** | **$45–70** new 2025–26 cohort (+~35% CPM penalty for a no-name brand; worst-quartile $187, best $42) | `[ESTIMATE, MED]` — **the governing term** |

**Why CAC governs:** every other line is single-digit dollars or a small %. **CAC is $45–70 on a $55–145 sale — the same order of magnitude as the entire gross margin.** This is the glossary's whole point made literal.

### 5b. Contribution after CAC, per single-unit order (r = 22% returns)

| Article | MSRP | Landed | Gross % | Contribution **pre-CAC** | Post-CAC @ **$25** | @ **$45** | @ **$70** |
|---|---|---|---|---|---|---|---|
| **A2 acrylic value** | **$55** | ~$11 | 80% | **$22.74** (41%) | **−$2.26** | **−$22.26** | **−$47.26** |
| **A1 cotton core** | **$75** | ~$11 | 85% | **$37.76** (50%) | **+$12.76** (17%) | **−$7.24** | **−$32.24** |
| **A3 merino premium** | **$145** | ~$20 | 86% | **$82.92** (57%) | **+$57.92** (40%) | **+$37.92** (26%) | **+$12.92** (9%) |

`[ESTIMATE — arithmetic on the §5a benchmarks; MED on method, LOW-MED on absolute levels because landed COGS and CAC are both bands; single-analyst (B), un-cross-examined — see §7]`.

**Read this as the core finding:**
- **A2 (the stated "value driver") never makes money once CAC is realistic.** Pre-CAC contribution is only **$22.74**, so even a $25 CAC puts it underwater; at the realistic $45–70 it **loses $22–47 per order.** **A2 is a loss leader, not a value driver** — squeezed by a low price ceiling *and* the 32% duty.
- **A1 (cotton core) is marginal and CAC-fragile** — clears a $25 CAC (+$12.76) but **breaks even at CAC ≈ $38** and goes negative at the realistic $45+. Survives only if CAC is held low **and** customers repeat.
- **A3 (merino premium) is the only robust earner** — positive even at a $70 CAC. **The premium price, not the premium fibre, pays for the customer.**

### 5c. The CAC ceiling and the LTV:CAC > 3 survival bar

**CAC ceiling (max CAC before the first order loses money):** A2 ≈ **$23** · A1 ≈ **$38** · A3 ≈ **$83** · 2-unit bundle (AOV $130) ≈ **$69**. Against a realistic $45–70 CAC: **A2 fails, A1 fails-to-marginal, A3 and bundles pass.**

The real bar is the glossary's **LTV:CAC > ~3.** Allowable CAC = (pre-CAC contribution × repeat orders) / 3:

| Article | repeats 1.0 | 1.5 | 2.5 |
|---|---|---|---|
| A2 $55 | CAC < **$8** | < $11 | < $19 |
| A1 $75 | CAC < **$13** | < $19 | < $31 |
| A3 $145 | CAC < **$28** | < $41 | < **$69** |
| bundle $130 | CAC < $23 | < $34 | < **$57** |

`[ESTIMATE — repeat rate is [UNKNOWN] for a brand that doesn't exist]`. **At 3:1, A2 is essentially impossible** (needs CAC < $8–19, unreachable in paid social); **A1 needs strong repeat (2.5×) just to tolerate $31; only A3 + bundles clear 3:1 at realistic CAC**, and only with some repeat.

### 5d. The AOV lever — the one move that makes the line close

CAC and outbound shipping are charged **per order, not per unit** — so **bundling amortises one CAC and one ship over a bigger basket:**

| Configuration | AOV | Pre-CAC | Post-CAC @ $25 | @ $45 | @ $70 |
|---|---|---|---|---|---|
| **2-unit bundle** (2 × $65 avg, ~$11 landed each) | **$130** | **$68.56** | **+$43.56** (34%) | **+$23.56** (18%) | **−$1.44** |

A **$130 AOV survives CAC up to ~$69** and clears 3:1 at ~$57 with modest repeat — versus a single $55 A2 that dies at $23. **The required AOV to make the model robust is ~$120–150 (≈ 2 units or 1 premium unit) — exactly the A3 price point or a deliberate 2-piece bundle.** This is the central pricing instruction back to T7/T8: **sell the line in baskets ≥ ~$120, never as single $55 sweaters.**

### 5e. Break-even volume — a knife-edge on the two softest inputs

Fixed monthly nut for a lean micro-brand `[ESTIMATE, MED]`: Shopify+apps (~$190) + 3PL minimum + minimal brand/owner draw → model **$1,500 (bootstrap) / $3,000 (modest) / $6,000 (with salary)**.

| Fixed/mo | @ $10 contrib/order | @ $20 | @ $30 |
|---|---|---|---|
| **$1,500** | 150 | 75 | 50 |
| **$3,000** | 300 | 150 | 100 |
| **$6,000** | 600 | 300 | 200 |

**At a realistic ~$20/order (premium-led, bundle-heavy, CAC ~$45) on a $3,000 nut, break-even ≈ 150 orders/mo (~$20–25k revenue/mo)** — achievable but not trivial for a no-name brand, and it **doubles to 300+ orders/mo the moment CAC drifts to $70 or contribution falls to $10.** Break-even is sensitive to exactly the two softest inputs (CAC and contribution) — **a knife-edge, not a comfortable margin.** `[ESTIMATE]`

---

## 6. The unmodelled killer — the working-capital trap

The §5 P&L is **per-order**. A brand can be **profitable on every order and still die on cash timing.** The cash-conversion trap (carried from T4 §4) is **not modelled per-order and is a separate kill-risk:** dye-lot MOQ forces **500+ units/colour of cash out**; ocean lead time + sell-through + returns settlement mean **cash is gone for months before it returns.** This is the classic profitable-on-paper D2C death. **Per-order viability is necessary, not sufficient** — T10 must weigh the cash cycle alongside the contribution math. `[ESTIMATE, MED-HIGH that this is the dominant non-P&L kill-risk]`

---

## 7. Honesty ledger — what is solid, what is soft, what is single-sourced

**Solid [FACT] (cited, load-bearing):** the three MFN duty rates (cotton 16.5% / acrylic 32% / wool 16%); the 7%-cotton upstream error correction; de-minimis death dated Aug 29, 2025; the additive-surcharge mechanism + current ~10%; the realised 50% Aug-2025 spike; MPF $33.58–$651.50 + HMF 0.125%; chief-weight rule + IOR liability; Shopify 2.9%+$0.30; right-of-publicity/Lanham legal exposure (§9).

**Soft / volatile [ESTIMATE or UNKNOWN]:** the forward India surcharge level (genuine [UNKNOWN]); all landed bands (inherit T4's [UNKNOWN] knit+LINK rate — the dominant internal uncertainty); exact 10-digit HTS suffix per article; freight per unit (a volume curve, not a point); fixed monthly nut; A3 premium yarn landed cost.

**The critical single-source flag for T10:** **the entire CAC / returns / contribution / break-even spine is Analyst B's, un-cross-examined** — Analyst A computed landed COGS and explicitly stopped there. These benchmarks are simultaneously the **governing** terms (CAC is the same magnitude as the whole gross margin) **and the softest** (all [ESTIMATE], all needing a live ad-account test). **The viability verdict rests on a single analyst's CAC band that one $1–2k ad test would harden or collapse.** `[ESTIMATE — B only; flagged single-source]`. **[DOWNGRADED]:** the single-point landed figures ($11/$11/$20) and the per-order dollar results read more precise than the evidence supports — they are **band-centre illustrations**, robust in *direction*, soft in *level*.

---

## 8. The blunt viability verdict — go / no-go conditions for T10

### Where the math FAILS (state this first, loudly)
1. **The line plan as written loses money.** A "value-led" launch leading with the **$55 A2 acrylic** at realistic CAC is **structurally unprofitable per order (−$2 to −$47).** The article the upstream plan calls the "value driver" **drains cash fastest.** Lead with A2 on paid social and the brand bleeds.
2. **A1 cotton at $75 is marginal and CAC-fragile** — needs CAC < $38 *and* repeat. New-cohort 2025–26 apparel CAC ($45–70) sits **above** its ceiling.
3. **The 32% acrylic duty + dead de-minimis + volatile India surcharge** can swing landed COGS 50%+; a **301-driven permanent tariff replacing the lapsing Section 122 baseline is a live kill-risk dated to ~July 2026 — exactly the launch window.**
4. **Returns are category-brutal and fit-driven** (22–30%); a returns-spike season alone can erase a thin contribution.
5. **The working-capital trap (§6) can bankrupt a per-order-profitable brand** — unmodelled, and the most likely real-world death.
6. **LTV:CAC > 3 fails for two of three articles** at realistic CAC; only A3/bundles clear it, and only with repeat an unproven brand can't assume.

### Where the math CLOSES (the narrow viable configuration)
The brand is viable **only if the line is inverted from the upstream plan:**
- **Lead with A3 premium ($145) + 2-unit bundles (AOV ~$120–150)** — the only configs with positive contribution at CAC $45–70 and the only ones clearing 3:1.
- **Hold blended CAC under ~$35** — which for a no-name brand means **acquisition cannot come mostly from cold paid social** (new cohorts pay $45–70+); it must lean **organic / community / affiliate / owner-audience** (≤ ~35–40% paid). **This is a go-to-market precondition, not a nicety — and it is the single most consequential lever in the whole stack.**
- **Use A2 (acrylic) deliberately as a low-margin basket-filler inside bundles, never a standalone hero** — its landed cost is *not* meaningfully below cotton, and it loses money alone at any realistic CAC.
- **Use A1 (cotton) as the margin-friendly core** — low-duty (~16.5%, ≈ wool; acrylic the 32% outlier), but needs repeat + a held-down blended CAC.
- **Keep fixed cost bootstrap-lean (~$1,500–3,000/mo)** so break-even is ~75–150 orders/mo, reachable.
- **Operate as a bulk Importer of Record → CA 3PL → domestic ship;** continuous bond above ~3 entries/yr; don't launch into the Jul–Oct Ludhiana cluster peak (carried from T1/T4).

### The explicit go / no-go conditions for T10
| Condition | Threshold to GO | Status now |
|---|---|---|
| **Blended CAC** | **< ~$35** (organic/community-led) | **[UNKNOWN]** — needs a live ad test; benchmark says $45–70 cold |
| **Line lead** | **A3 premium + bundles, AOV ~$120–150** | controllable design choice (T7/T8) |
| **A2 role** | **bundle-filler only, never standalone hero** | controllable |
| **Operating model** | **bulk IOR → CA 3PL → domestic ship** | controllable; forced by dead de-minimis |
| **India surcharge at import** | **survivable at central ~10%, stress-test 25–50%** | **[UNKNOWN] forward**; re-confirm per PO |
| **Repeat-purchase rate** | **enough to clear LTV:CAC > 3 on A1/bundles** | **[UNKNOWN]** for a non-existent brand |
| **Fixed nut** | **~$1,500–3,000/mo** | controllable |
| **Working capital** | **funded through the months-long cash-out cycle** | **[ESTIMATE]** — the unmodelled killer (§6) |

**GO if** the brand can be built premium-led, bundle-driven, and **acquire customers off cold paid social at blended CAC < ~$35**, funded through the working-capital cycle. **NO-GO (as currently planned) if** it launches as an **affordable-acrylic, value-led, paid-social brand** — that configuration loses money on the first order and is structurally unviable.

---

## 9. Legal flags (binding — do NOT wave through)
- **Right of publicity + Lanham false-endorsement (HIGH risk).** No real athlete/celebrity name, voice, signature, photo, or likeness without a license — California protects living *and deceased* persons; damages include the infringer's profits. **[FACT — Cal. Civ. Code §3344/§3344.1; Lanham Act §43(a)].** T7's "athlete/event/meme-driven naming" must clear marks before printing.
- **Duty-misclassification / blend-drift exposure.** Fibre composition sets the HTS line and duty; a supplier under-blending makes the brand (IOR) mis-declare. **Write blend % ± tolerance + fibre-composition verification into every PO.** `[FACT — duty lines; ESTIMATE — drift risk]`

---

## Bottom line for the owner

1. **Gross margin (80–86%) is a vanity number; contribution-after-CAC is the truth — and it kills the value article.** A2 acrylic at $55 loses money on the first order at any realistic CAC; A1 cotton at $75 is marginal and CAC-fragile; **only A3 merino at $145 and 2-unit bundles reliably pay for a customer.**
2. **The 32% MMF duty on acrylic flips the line.** Landed, A1 (cotton) ≈ A2 (acrylic) < A3 (merino) — acrylic's factory-gate cost lead is erased at the border, and **cotton becomes the friendly anchor.** **[FACT on rates].** The upstream **7%-cotton duty figure is wrong; it's 16.5%** — correct it everywhere.
3. **CAC ($45–70 for a new 2025–26 cohort) is the governing term** — the same order of magnitude as the whole gross margin. The brand only closes if **blended CAC < ~$35**, which means **organic/community/affiliate acquisition, not cold paid social.**
4. **Invert the line plan:** lead premium + bundles to push **AOV to ~$120–150**; A2 is a basket-filler, never a hero. **The required AOV is the single most important pricing instruction.**
5. **De-minimis is dead (Aug 29, 2025); every unit is dutiable.** Operate as a **bulk Importer of Record → CA 3PL → domestic ship**, not zero-inventory ship-direct. The **India surcharge (~10% now, 50% peak history, lapses ~Jul 2026 with a 301-replacement risk on the launch window)** can swing all-in duty to **26–57%** — stress-test it.
6. **Break-even is a knife-edge:** ~75–150 orders/mo at a lean $1,500–3,000 nut **if** contribution holds ~$20/order; it doubles the moment CAC drifts to $70.
7. **The working-capital trap is the unmodelled killer** — a per-order-profitable brand can still die on the months-long cash-out of dye-lot MOQ + ocean lead time + returns settlement. Per-order viability is necessary, not sufficient.
8. **The whole verdict rests on a single un-cross-examined CAC benchmark.** One $1–2k live ad test is the **highest-leverage thing the owner can do** — it collapses the biggest uncertainty in the entire venture.

**The honest one-liner:** *Viable as a small **premium/community** brand led by A3 + bundles at blended CAC < ~$35; **not** viable as an **affordable-acrylic, value-led, cold-paid-social** brand. The single most dangerous mistake the upstream plan invites is **leading with A2** — the article that, after the 32% duty and CAC, cannot pay for itself.*

---

## Open questions (the floor T10 inherits — not a ceiling)
1. **Live blended CAC from a real test ad account** — **[UNKNOWN]**; the single governing variable; benchmarks ($45–70) are un-cross-examined bands one $1–2k test collapses. *The highest-leverage data acquisition in the whole T9 layer.*
2. **The India surcharge level at actual import date** — **[UNKNOWN] forward**; Section 122 ~Jul-2026 sunset + Section 301 outcome pending; re-confirm before committing inventory.
3. **The T4 [UNKNOWN] knit+LINK job-work rate** — the dominant uncertainty *inside* landed COGS; one quote run halves the ex-factory band and tightens every number here.
4. **Real return rate for THIS fit/fabric** — **[ESTIMATE, 20–30%]**; fine-gauge fit-sensitivity could push higher; needs first-cohort data.
5. **Repeat-purchase rate / true LTV** — **[UNKNOWN]** for a non-existent brand; decides whether A1 ever clears LTV:CAC > 3.
6. **Live forwarder quote: FCL/LCL India→LA-Oakland + drayage + 3PL receiving** — **[ESTIMATE]** bands only; tightens freight from the §1b curve to a point.
7. **Exact 10-digit HTS suffix per article** once the tech pack fixes fibre %/value — **[UNKNOWN]**; sets the precise duty line (esp. the A3 wool value-break and the A1/A2 chief-weight call).
8. **Actual fixed monthly nut** (3PL minimums, app stack, owner draw) — **[ESTIMATE, $1,500–6,000]**; sets break-even order count.
9. **A3 premium yarn landed cost** — inherits T2/T4's **[ESTIMATE, LOW]** merino price; A3 carries the line, so its COGS needs an importer quote before it is load-bearing.
10. **Whether any India apparel line gets carve-outs** in a finalized US-India deal — **[UNKNOWN]**; could lower the surcharge for textiles specifically.
11. **Continuous bond vs single-entry break-even** at the brand's real drop cadence — **[ESTIMATE, MED]**; continuous bond wins above ~3 entries/yr.
12. **Inverted-duty / GST drag on acrylic (A2)** — a cash-cycle cost, magnitude **[UNKNOWN]**; named, not a per-unit COGS add (carried from T4).
13. **Working-capital funding** — can the owner fund the months-long cash-out cycle (§6) — the unmodelled kill-risk that a clean per-order P&L hides.

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/unit_economics/DEEPDIVE.md`
- **Read:** `unit_economics/reconciled.md`; `unit_economics/analyst_a_landed_cost.md`; `unit_economics/analyst_b_d2c_pnl.md`; `MASTER_PLAN.md`; `SCOPE_GLOSSARY.md` (with `mfg_economics/DEEPDIVE.md`, `yarn/DEEPDIVE.md`, `ludhiana/DEEPDIVE.md` inherited via the analyst/reconciled chain).
- **Delivered (role coverage):** (1) the **per-article landed-cost stack** with the full freight/MPF/HMF/broker/bond/IOR breakdown and bands A1 ≈ $6–14 / A2 ≈ $6–14 / A3 ≈ $9–31+; (2) the **duty reality** — the 32% MMF acrylic wall as the headline, the A2→A1≈A2 ranking inversion, the 7%-cotton error correction, the chief-weight duty cliff; (3) the **de-minimis current state** (dead Aug 29 2025) and its model-flip to bulk-import→CA-3PL→domestic-ship; (4) the volatile India surcharge band (~10% central / 50% peak history / [UNKNOWN] forward / 301-replacement risk on the launch window); (5) the full **contribution-after-CAC P&L** — MSRP ladder, the four below-gross costs, per-order contribution per article, CAC ceilings, LTV:CAC>3, the AOV lever, break-even volume; (6) a **blunt go/no-go verdict** with an explicit conditions table for T10; (7) legal flags; (8) Bottom line + 13 open questions.
- **Honesty discipline:** all duty rates + de-minimis date + MPF/HMF held [FACT]; surcharge forward level [UNKNOWN], history (incl. 50% spike) [FACT]; landed bands [ESTIMATE] inheriting T4's [UNKNOWN] knit+LINK width; single-point landed figures and per-order dollars [DOWNGRADED] to band-centre illustrations; the CAC/returns/contribution spine flagged **single-analyst (B), un-cross-examined** — governing yet softest; surcharge dates carried with MED confidence; 7%-cotton error corrected; right-of-publicity/Lanham legal risk kept HIGH, not waved; working-capital trap named as the unmodelled killer; no fabricated forward tariff.
- **Hand-off to T10:** GO only in a premium-led, bundle-driven, blended-CAC-under-~$35, lean-fixed-cost, bulk-import config funded through the cash cycle; NO-GO as an affordable-acrylic, value-led, cold-paid-social brand. The decisive [UNKNOWN] is live CAC — one $1–2k ad test collapses it.
