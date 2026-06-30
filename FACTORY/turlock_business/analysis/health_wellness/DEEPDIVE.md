# Turlock Health & Wellness — Sector Deep-Dive

**The definitive synthesis of the independent Health & Wellness universe in Turlock, CA (Stanislaus County).**

This document merges three upstream analyses — supply-chain & operations (`supply_ops.md`), market & saturation economics (`market_saturation.md`), and the reconciliation brief (`reconciled.md`) — against the source dataset (`businesses.json`) into a single decision-grade sector reference.

**Compiled:** 2026-06-30 · Role: Writer (synthesis).

---

## Financial & Honesty Charter (binding for this document)

This deep-dive obeys the following rules without exception:

- **Per-business revenue, margin, payer mix, lease rate, and staffing are NOT public** for these privately held independents. **No dollar figure in this document is stated as an actual reported figure for any named business.** [UNKNOWN at the named-entity level]
- Every dollar figure tied to a *sub-category* is a **labeled benchmark estimate** — a published industry benchmark (revenue per provider / per practice / per studio, overhead %, COGS %) scaled by an **observable size signal** (provider count where inferable, format) and a **Central-Valley reimbursement/cost-of-living haircut** — explicitly tagged **[ESTIMATE]** with a method, a source, and a confidence word.
- **City / county / state figures** (population, income, demographics, license and renewal fees, the 2026 Medicare conversion factor) are **cited public facts** tagged **[FACT]**, or are explicitly labeled derivations/estimates.
- **Supplier-to-named-business links are representative inference** — the named distributors are the realistic, verified-to-serve-this-geography supply universe; the link from a specific named practice to a specific supplier is inference unless a cited source states the relationship.
- Where a *high gross* revenue band exists, it is paired with its *thin net / overhead* caveat so high revenue is never mistaken for high profit.
- **Coverage is reported as a FLOOR, not a ceiling.** The catalog is "at least this many verified independents," never "exactly all of them."
- Tags used throughout: **[FACT]** = sourced public data · **[ESTIMATE]** = modeled benchmark · **[UNKNOWN]** = not determinable from public data.

---

## Table of Contents

1. [Sector overview & the named businesses](#1-sector-overview--the-named-businesses)
2. [Why this many, not more — the saturation argument](#2-why-this-many-not-more--the-saturation-argument)
3. [Business models & unit economics per sub-category](#3-business-models--unit-economics-per-sub-category)
4. [Supply chain & operations](#4-supply-chain--operations)
5. [Competitive structure — crowded vs. white-space](#5-competitive-structure--crowded-vs-white-space)
6. [Risks, unknowns & coverage caveats](#6-risks-unknowns--coverage-caveats)

---

## 1. Sector overview & the named businesses

Turlock's independent Health & Wellness sector resolves to a verified **floor of 26 owner-operated, licensed businesses** across six sub-categories. [FACT — `businesses.json`, count = 26] This is a floor, not a census: the catalog deliberately **excludes chains, DSOs, hospital-employed providers, and corporate optical**, and rests on per-business source confidence that ranges from high to medium. The true population of independents could be modestly higher; it is very unlikely to be lower. [UNKNOWN — exact universe]

**The six sub-categories and their counts (by primary category):**

| Sub-category | Count | Named businesses |
|---|---|---|
| **Dental** | 7 | American Family Dentistry · Hawkeye Family Dental · Healthy Smile Dental · Melgosa Dental · Turlock Family Dentistry · Turlock Smiles Dentistry · Valley Dental Turlock |
| **Chiropractic** | 6 | Atkinson · Beech · Crawford Chiropractic Center · LOR · Mas Vida · Turlock Auto Accident Injury (Dr. Greg Jones, DC) |
| **Optometry** | 6 | Avak's · Generations Family · Monte Vista · Plett Family · Turlock Eyecare (since 1950) · Turlock Family Vision |
| **Physical Therapy** | 3 | Altruistic PT · Elite PT & Fitness · Tower PT |
| **Independent Gym/Fitness Studio** | 3 | Barre Defined · Curl Fitness · Workout Studio by KB |
| **Yoga/Wellness Studio** | 1 | I Am Yoga Wellness Studio |
| **Total** | **26** | |

**The two economic engines.** The defining structural fact of this sector — agreed across both upstream memos — is that it is not one market but two. [FACT]

- **Insurance-reimbursed clinical** (Dental, Optometry, Physical Therapy, plus the personal-injury/auto slice of Chiropractic). Revenue is gated by payer contracts — Delta Dental, VSP/EyeMed, Medicare, commercial PPOs, and very heavily in Stanislaus County **Medi-Cal / Denti-Cal**. Operations revolve around billing, prior authorization, and a physical-goods supply chain (crowns, frames, lenses, modalities). These formats are recurring and the most recession-resilient. [FACT]
- **Cash / membership wellness** (independent gyms, yoga, barre, and the cash/maintenance slice of Chiropractic). Revenue is membership, class-pack, or cash-pay with almost no payer billing; the "supply chain" is capital equipment + facility + instructor labor rather than consumables. These are semi-discretionary — the first line cut in a downturn. [FACT]

**The two dual-category straddles.** Two businesses sit in two buckets at once, which slightly inflates any strict per-sub-category tally and must be handled explicitly. [FACT]
- **Elite Physical Therapy & Fitness** straddles PT (insurance-reimbursed) + fitness (cash), so it carries *both* a DME/modality supply chain and a gym-equipment capex line.
- **I Am Yoga Wellness Studio** lists Pilates and Yoga Sculpt alongside Vinyasa/Ashtanga/Hatha/Restorative/Qi Gong, adding light equipment to an otherwise prop-only input base, and overlapping the fitness-studio format.

So "26 across six categories" is the honest headline, but the reader should note that two of the 26 are economically bi-modal.

**Who anchors the demand.** The resident base is **~72,500 residents** [FACT — California Demographics / Census QuickFacts 2024], **median household income ~$82,995** [FACT — same], **median age ~34.9** [FACT]. Three overlays shape what the sector can carry: **CSU Stanislaus (~8,455 undergrad / ~9,700 total)** [FACT — US News / DataUSA 2024], a younger, vision-and-wellness-skewed cohort; a large **county ag and warehouse workforce + retiree base** that drives musculoskeletal (chiropractic, PT) volume above a white-collar town's baseline; and **deep ethnic-community density** — California's largest Assyrian community (~20,000, ~25% of the city) [FACT — KQED], a Portuguese/Azorean cohort (~7.3% ancestry) [FACT — Statistical Atlas], and Punjabi/Sikh and Mexican (Hispanic ~46%) communities [FACT — California Demographics]. That last overlay is the sector's most distinctive demand force, and it does real work in §2.

---

## 2. Why this many, not more — the saturation argument

The single best-supported answer is that **26 is an equilibrium**: four ceilings hold the count down, and one floor (community fragmentation) keeps it from collapsing toward chain dominance. [ESTIMATE — synthesized verdict; med confidence] This is an editorial conclusion, not sourced data, and is labeled accordingly — including the upstream characterization of the sector as "structurally healthy," which is a judgment, not a measured figure.

### 2.1 The saturation math (businesses per 10k)

Method: catalog count ÷ 7.25 (= 72,500 / 10,000). The benchmark column is the U.S. **practitioner** ratio converted to per-10k. A critical honesty caveat governs every row: **the benchmark counts *providers*, while the catalog counts *independent businesses*** — each business may hold 1–3 providers, and the catalog excludes chains/DSOs/hospital-employed providers. So the catalog ratio *should* sit **below** the practitioner benchmark even in a fully-served market; the gap is the leakage-and-consolidation signal, not proof of undersupply. [FACT — method]

| Sub-category | Catalog count | Per 10k (city pop) | US benchmark (providers/10k) | Read |
|---|---|---|---|---|
| Chiropractic | 6 | 0.83 | ~2.4 (23.7/100k) [FACT] | **Healthy, not saturated.** Cash/auto model + ag/bilingual demand supports the count. Room for 1–2 *differentiated* entrants. |
| Dental | 7 | 0.97 | ~6.0 (59.5/100k) [FACT] | Ratio far below provider benchmark because **chains/DSOs + hospital + Modesto absorb the rest.** Most *contested*, not undersupplied. |
| Optometry | 6 | 0.83 | ~1.3 (12–14/100k) [FACT] | **Crowded for independents.** 6 independent ODs in a 72k town is dense; chain/online optical competes hard. Near ceiling. |
| Physical Therapy | 3 | 0.41 | n/a (referral-gated) | **Under-built / white-space.** Referral- and payer-gated; Modesto hospital PT leaks demand out. Room to grow. |
| Independent Gym/Fitness | 3 | 0.41 | n/a (format-driven) | Thin because **national chains own the value tier.** Independents survive only as differentiated boutiques. |
| Yoga/Wellness | 1 | 0.14 | n/a | **Thinnest sub-category.** Demand-ceiling + leakage-to-fitness-studios story. 1–2 more *could* fit on the CSU + community base. |
| **Sector total** | **26** | **3.6** | — | Consistent with a mid-density, leakage-exposed secondary market. |

The 3.6/10k aggregate is the headline number: **mid-density, not collapsed and not saturated.** [ESTIMATE — derived from FACT inputs; high confidence in the arithmetic, med in the interpretation]

### 2.2 The demand ceiling — four caps

**Cap 1 (biggest): leakage to Modesto.** ~15 minutes north, Memorial/Doctors hospital systems, specialist optometry/ophthalmology, large multi-provider dental groups, and chain optical pull the higher-acuity and price-shopping demand out of Turlock. Both upstream memos rank this the dominant external cap; it is the single biggest reason the catalog ratio sits below the provider benchmark. [FACT — competitive backdrop; the magnitude of leakage is ESTIMATE]

**Cap 2: the payer-mix cap.** Stanislaus is a high-Medi-Cal county and lost ~17,000 enrollees in the 2024 unwinding. [FACT — DHCS via Stocktonia 2024] Medi-Cal dental (Denti-Cal) reimburses well below commercial PPO and carries heavier admin, which **caps how many dental/clinical independents can subsist on the low-income majority** and pushes them toward PPO/cash share. Both memos independently identify this as the *binding* clinical constraint — reimbursement, not input supply, is what compresses margin. [FACT]

**Cap 3: the discretionary-wallet cap.** A median-$82,995 town with a high Medi-Cal share has a limited discretionary wallet, which holds yoga and boutique-fitness counts thin **regardless of their low entry capex.** The constraint on these formats is demand/utilization, not the cost of opening. [ESTIMATE — med confidence]

**Cap 4: the supplier + reimbursement margin squeeze.** Distributor concentration (the dental "big three"; EssilorLuxottica/ABB in optical) and the 2026 Medicare PT conversion factor (~$33.40, net ~-1% after RVU changes) thin the per-unit economics that would otherwise fund additional entrants. [FACT — conversion factor; ESTIMATE — net margin effect] Licensing barriers (DC Board, Dental Board, Board of Optometry, PT Board) further raise the entry threshold and keep new-entrant supply orderly. [FACT]

### 2.3 The demand floor — the community expander

The counter-force that keeps the count from falling toward chain dominance is **ethnic-community trust/language fragmentation plus family-dynasty loyalty.** [FACT — demographic base; ESTIMATE — its effect on provider count] Assyrian, Portuguese/Azorean, and Punjabi/Hispanic trust-and-language pools, layered on multigenerational practice loyalty, fragment the market into **parallel demand pools** that each support their own independent. The clearest visible evidence is **Mas Vida Chiropractic's** explicit bilingual (English/Spanish) positioning and the several family-named, multigenerational practices (Generations Family Optometry, Plett Family, the family-dentistry cluster). This is the documented "supports a format a generic 73k town wouldn't" effect — it raises the sustainable provider count *above* the raw per-capita benchmark. [ESTIMATE — med confidence]

**Net verdict.** Leakage (Cap 1) and the payer/wallet caps (Caps 2–3) keep the count from growing; the supplier/reimbursement squeeze (Cap 4) thins the economics; community fragmentation (the floor) keeps it from shrinking. The result sits at a healthy-mid **3.6/10k**. The genuine room to add is narrow and *conditional*: **differentiated, referral-driven PT; a second yoga/wellness studio; and niche bilingual/PI chiropractic.** Optometry and independent dental are effectively at their contested ceiling and compete on differentiation, not headcount. [ESTIMATE — synthesized; med confidence]

---

## 3. Business models & unit economics per sub-category

> **Every band below is an estimate, not actual.** Method = published national/industry benchmark (ADA Health Policy Institute / dental CPA surveys; optometry GPO/COGS surveys; APTA/WebPT PT economics; boutique-fitness and yoga operating models) × an observable size signal × a **~0.80–0.90 Central-Valley cost-of-living / reimbursement haircut** vs. national/coastal averages. Spread within each band is driven by provider count, payer/cash mix, eyewear/procedure capture, and how much demand a clinic recaptures from Modesto leakage. Confidence is the analyst's confidence in the band as a *planning estimate* — never a claim about any named business. [UNKNOWN at the named-entity level]

| Sub-category (count) | Per-business revenue band — *estimate, not actual* | Cost structure / margin pairing | Labor model | Confidence |
|---|---|---|---|---|
| **Dental (7)** | **$600k–$1.0M gross/yr** per single-dentist office; multi-provider offices (e.g., those advertising oral surgery) scale toward/above the top | Overhead **~55–65% of collections**; sub-$750k-collection solo practices run **70–80%**. Components: staff payroll 25–28%, lab fees 6–8%, supplies 5–6%, rent/facility 6–7%. Denti-Cal mix compresses margin. **High gross ≠ high net.** | W-2 licensed DDS/RDH + support staff (hygienists, front-desk/billing); labor is the single largest cost line; some add associate doctors | Med |
| **Optometry (6)** | **$500k–$900k revenue/yr** per single-OD practice | Optical **COGS ~27–35% of revenue** (frames/lenses/contacts + shipping); buying-group membership pulls COGS toward the low end. **Eyewear capture rate is the margin engine** (the exam is not). | W-2 OD + opticians + support; solo OD income benchmark ~$212k [FACT — Review of Optometry 2023] | Med |
| **Physical Therapy (3)** | **$150k–$300k revenue/yr per PT; ~$300k–$700k per clinic** | Payroll ~70% of fixed overhead / 40–50% of total expenses; rent ~5–10%; admin (billing/marketing/software) ~10–15%; supply COGS ~15%. Revenue **~$75–$150/visit** ($110–$140 billing 3–4 codes/session); rule of thumb a treating PT bills 3–4× salary; **~200–400 visits/month to break even.** 2026 Medicare cut is a per-visit headwind. | W-2 licensed PTs + aides + front-desk/billing; 1–2 PTs/clinic | Med |
| **Chiropractic (6)** | **$150k–$450k gross/yr per solo DC**; auto-injury/lien clinics run higher and lumpier (settlement-timed) | Low consumable COGS; cost dominated by **owner labor + rent + professional liability + EHR ($300–$800/mo).** Cash/maintenance models carry minimal billing overhead; PI/auto trades higher per-case value for lien/collections risk and slower cash conversion. | Predominantly **owner-operator DC**, often solo or +1 chiropractic assistant + billing; PI/auto (Dr. Jones) carries heavier lien admin | Low–Med |
| **Independent Gym/Fitness (3)** | **$120k–$500k revenue/yr per studio**; owner take ~$50k–$120k benchmark | Near-zero COGS. **Payroll is the largest line, lease #2**; lease + equipment ≈ 60–70% of startup cost. Equipment leasing converts capex to opex. **Utilization (members/sqft, class fill) is the margin lever.** | Owner-operator + **per-class-paid instructors, frequently 1099** (AB-5 exposure); instructor pay scales with class volume | Low–Med |
| **Yoga/Wellness (1)** | **$100k–$350k revenue/yr per studio** (61% of US yoga studios earn <$500k/yr [FACT]) | **Rent ~20–30% of revenue; instructor pay ~$25–$75/class (≈30%+ of top-line);** target ~30–50% gross margin per class. Unlimited memberships ~$100–$200/mo. Broad format menu spreads fixed cost across more class slots. | Owner-operator + per-class instructors (1099/AB-5 exposure) | Med |

**The cross-cutting unit-economics story:**
- **Capex tiers (high → low): dental ≈ optometry > PT > chiropractic ≈ gym > yoga.** [ESTIMATE] Dental and optometry are the capital-heavy, COGS-heavy formats; chiro-cash, gym, and yoga are near-zero-COGS, capex-plus-labor-plus-rent businesses.
- **The binding constraint differs by engine.** For the clinical engine it is **reimbursement** (Denti-Cal rates, the 2026 Medicare PT cut), not inputs. For the cash engine it is **utilization** (fill rate, members per square foot) and the **labor-classification model** (W-2 vs. 1099 under AB-5).
- **Two different PT anchors must not be quoted interchangeably.** The per-visit benchmark (~$75–$150/visit) and the per-PT-per-year benchmark ($150k–$300k) are reconcilable but distinct; quoting one as if it were the other is an error. Both are [ESTIMATE].

---

## 4. Supply chain & operations

> **Supplier links are representative inference**, not confirmed contracts. The named distributors and buying groups are the realistic, verified-to-serve-this-geography supply universe; the link from a specific named Turlock practice to a specific supplier is inference unless a cited source states it. [FACT — supplier universe; inference — specific links]

### 4.1 Inputs, suppliers & capex by sub-category

| Sub-category | Primary suppliers / buying groups | What flows in | Capex (initial build/equipment) |
|---|---|---|---|
| **Dental** | **Henry Schein, Patterson, Benco** ("big three," ~85% of distributor sales; ~41/34/10% share) for consumables; **dental labs (e.g., Glidewell)** for restorations; implant/CAD-CAM OEMs [FACT] | Gloves, anesthetic, composites, impression/scan materials, sterilization; lab-fabricated crowns/bridges/dentures | Chairs/units, digital pano/CBCT X-ray, autoclave, intraoral scanners — **high six-figure build-out** [ESTIMATE] |
| **Optometry** | **EssilorLuxottica** (frames + lenses), **ABB Optical** (contacts, ~2/3 of ECPs); buying groups **Vision Source, PECAA, IDOC, EPON** for vendor discounts [FACT] | Frame inventory, finished/edged lenses, soft & GP contacts, diagnostic consumables | Phoropter, auto-refractor, OCT/fundus camera, edger, exam lanes [ESTIMATE] |
| **Chiropractic** | **ScripHessco** (largest reconditioned-table distributor), **MeyerDC** (est. 1948); supplement/nutraceutical vendors; EHR (ChiroTouch, ClinicMind) [FACT] | Adjusting tables, e-stim/ultrasound modalities, table paper, supports/braces, resale supplements | Adjusting tables ($1k–$5k each), modalities, optional digital X-ray ($40k–$80k for insurance/PI practices) [ESTIMATE] |
| **Physical Therapy** | DME/rehab distributors (MeyerPT, Performance Health/TheraBand, BTE); EHR/billing (WebPT) [FACT] | Exercise equipment, modalities (ultrasound, e-stim), tables, bands/weights, low-cost consumables | Treatment tables, gym/exercise floor, modalities; **$50k–$150k initial equipment** [ESTIMATE] |
| **Independent Gym/Fitness** | Equipment OEMs **Life Fitness, Precor, Technogym, Rogue**; financing/leasing programs [FACT] | Racks, cardio machines, free weights, barre/Pilates apparatus, flooring | **$8k–$10k (small studio) to $100k+ (full commercial floor)**; often leased to spread capex [ESTIMATE] |
| **Yoga/Wellness** | Prop vendors (mats, blocks, straps, bolsters), apparel wholesalers; class software (Mindbody, Mariana Tek, WellnessLiving) [FACT] | Props, retail apparel, cleaning supplies, studio fit-out (flooring, sound, optional heat) | **Lowest capex in sector** — flooring, mirrors, sound, optional heating [ESTIMATE] |

**Supplier concentration is the dental/optometry story, and it is a margin risk the demand view alone misses.** The dental big three and EssilorLuxottica/ABB give national distributors strong pricing power over small Turlock independents; the main counterweight is **buying-group membership** (Vision Source/PECAA/IDOC for optical; GPOs for dental). Cash/membership formats avoid this entirely — their "supply chain" is durable capex bought once, with near-zero perishable inventory. [FACT]

**Inventory models** vary by engine: dental consumables run near-JIT with free-ship thresholds (~$200–$300/order) while lab cases take 5–15 business days; optical lens jobs cut to Rx (~3–7 days) with contacts drop-shipped; chiropractic/PT equipment is durable capex with low consumable burn; gym/yoga carry essentially no perishable inventory. [FACT]

### 4.2 Operating cadence & labor

**Labor** is the dominant cost line in both engines but structured differently: clinical formats run **W-2 licensed professionals + support staff**; chiropractic is **predominantly owner-operator**; gym/yoga/barre run **owner-operator + per-class-paid instructors, frequently 1099** — which is precisely where AB-5/Dynamex misclassification risk bites. [FACT]

**Dayparts and seasonality** differ by engine. Clinical (dental/optometry/PT/chiro) runs weekday daytime + some early-evening/Saturday blocks, with optometry seeing a **Q4/January bump** as flex-spending and vision benefits reset plus a back-to-school CSU spike. Fitness/yoga run the classic **early-morning + evening dayparts with a January "resolution" surge** and a summer dip; membership smooths the cash flow. [ESTIMATE]

### 4.3 Regulatory & licensing gates

All clinical providers are gated by **California Department of Consumer Affairs healing-arts boards**; the concrete gates — often omitted from a pure demand analysis — are the real entry mechanics. [FACT]

- **Board of Chiropractic Examiners (BCE):** annual DC renewal, **24 CE hours/yr** (incl. 2 hrs Ethics & Law); X-ray practice adds RHB radiologic-equipment registration. [FACT]
- **Dental Board of California:** DDS/RDH licensure, X-ray/CBCT registration, **Denti-Cal/Medi-Cal provider enrollment required to bill (separate from licensure)**, OSHA/Cal-OSHA infection control, biohazard waste handling. [FACT]
- **California State Board of Optometry:** exam app fee **$275**, biennial renewal **$500**; glaucoma/TPA certifications expand billable medical scope. [FACT]
- **Physical Therapy Board of California (PTBC):** app fee **$300**, initial license **$150**, **$200 renewal**; CA allows direct access with limits. [FACT]

**Cross-cutting:** city of Turlock business license; professional liability/malpractice (~$1,200/mo benchmark for solo clinical [ESTIMATE]); general liability for studios (slip/fall, equipment); ADA facility compliance; and for fitness/yoga, **AB-5 (Dynamex/ABC test) worker-classification exposure** on 1099 instructors. CSLB is generally **not** a gating input — it governs the contractors doing the build-out, not the practice. [FACT]

---

## 5. Competitive structure — crowded vs. white-space

**Crowded (compete on differentiation, not count):**

- **Optometry (0.83/10k independents).** Dense for a 72k town; the real fight is **product-margin defense against Costco/online eyewear** (Warby Parker, 1-800-Contacts, Costco optical) that bleed the eyewear margin funding the practice. Differentiation = medical eye care, disease management, specialty contact-lens fitting — which several catalog practices (Monte Vista, Avak's) already lead with. Survivable only via eyewear-capture + buying-group cost control. [ESTIMATE — read; FACT — competitive backdrop]
- **Independent dental (0.97/10k).** Most-contested sub-category, under pressure from DSOs/chains and Modesto multi-provider groups. Independents win on **relationship/longevity** — several advertise 20+ years (American Family Dentistry) and use family/cosmetic/Invisalign upsell to escape Denti-Cal-rate commoditization. [ESTIMATE — read]
- **Chiropractic (0.83/10k).** Moderately crowded but **segmented by model** — wellness-cash (Atkinson, Beech), affordability-positioned (Crawford), bilingual (Mas Vida), and auto-injury/lien (Dr. Jones) — and that segmentation is exactly what keeps six clinics viable. [ESTIMATE — read]

**White-space (room to add / under-served), conditional on differentiation:**

- **Physical Therapy (0.41/10k).** The clearest organic-growth lane — under-built, with demand leaking to Modesto hospital PT. But growth must come from **physician-referral relationships and visit volume, not from rate** (the 2026 Medicare cut squeezes per-visit). Ortho/sports/post-surgical is the opening. [ESTIMATE — read]
- **Yoga/Wellness (0.14/10k, single studio).** Thinnest sub-category; **1–2 more could fit** on the CSU + young-professional + Assyrian/Portuguese base — *but only if a new studio recaptures demand that gyms selling yoga/Pilates (Curl Fitness, Workout Studio by KB, I Am Yoga itself) currently bleed off.* The cap is a demand ceiling, not an entry-cost barrier. [ESTIMATE — read]
- **Independent fitness (0.41/10k).** White-space **only for differentiated formats** — the value tier is owned by national chains (Planet Fitness / In-Shape / Crunch type). Boutiques like Barre Defined survive on niche, not price. [ESTIMATE — read]

**Where demand leaks out (ranked):** (1) Modesto hospital systems & specialist groups — higher-acuity dental/optometry/PT; (2) chain optical + online eyewear/contacts — optometry product margin; (3) national gym chains — the value/commodity fitness tier; (4) DSOs/dental chains — price-shopping and Medi-Cal-skewed dental volume. [FACT — leakage channels; ESTIMATE — ranking]

**What an entrant should know.** The defensible moves are narrow and conditional: a **differentiated, referral-driven PT clinic**; a **second yoga/wellness studio** that captures the CSU and community base gyms currently leak; or a **niche bilingual or PI chiropractic** practice. Do **not** open a generic seventh chiropractic clinic, a seventh independent optometry, or an eighth independent general-dental office competing on Denti-Cal volume — those compete on differentiation against an effectively-ceilinged count. Every clinical entrant must also clear the licensing/Denti-Cal-enrollment gate, and every studio entrant must resolve the AB-5 W-2-vs-1099 instructor question before scaling. [ESTIMATE — go/no-go; FACT — gates]

---

## 6. Risks, unknowns & coverage caveats

1. **Per-business revenue is not public and is never stated as fact here.** Every dollar figure is a sub-category benchmark scaled by a Valley haircut, labeled [ESTIMATE]. The most defensible per-business read is the §3 band, not a sector average — and even those bands carry only low–medium confidence. [UNKNOWN at the named-entity level]

2. **Coverage is a FLOOR, not a ceiling.** The 26 are verified independents with chains/DSOs/hospital-employed providers excluded; per-business source confidence ranges high to medium (e.g., Turlock Smiles Dentistry, Avak's, Curl Fitness sit at medium). The real independent universe could be modestly larger. Existence and sub-category are reliable; finer operational detail (provider counts, exact payer mix, lease rates) is not independently verified. [UNKNOWN]

3. **The two dual-category straddles distort strict counts.** Elite PT & Fitness (PT + fitness) and I Am Yoga (yoga + Pilates/fitness) each sit in two buckets. The "26 across six categories" framing is honest at the headline level but bi-modal underneath for these two. Saturation reads are unaffected. [FACT]

4. **Reimbursement is the live, moving risk on the clinical side.** Denti-Cal rates, the 2024 Medi-Cal unwinding (~17,000 county enrollees lost) [FACT], and the **2026 Medicare PT conversion factor (~$33.40, net ~-1%)** [FACT] move margins more than any input cost. These are dated facts in a moving system. [FACT — current figures; the forward trajectory is UNKNOWN]

5. **Supplier concentration and AB-5 are structural risks the demand view alone misses.** The dental big three and EssilorLuxottica/ABB hold pricing power over small independents (mitigated only by buying groups); AB-5/Dynamex worker-classification exposure sits on exactly the 1099-instructor gym/yoga formats the saturation read calls thinnest. [FACT]

6. **Saturation reads are estimates of competitive density, not measured leakage.** The Modesto ceiling is an estimate of cross-shopping behavior, and the per-10k benchmarks compare *providers* to *businesses* — an intentional apples-to-oranges that must be read as a directional signal, not a precise gap. [ESTIMATE]

7. **"Structurally healthy" is a judgment, not data.** The upstream conclusion that the sector is structurally healthy, and that Turlock sustains more independents per capita than a generic same-size town, is well-supported on the *expander* (community-fragmentation) claim but remains an editorial [ESTIMATE], not a sourced figure. [ESTIMATE]

8. **Static snapshot.** This is a 2026-06-30 view. CSU enrollment is rebounding, the Medicare PT cut is newly in effect, and Medi-Cal enrollment is in flux — trajectories that will move the saturation reads over time. [FACT — as-of date]

**The single highest-leverage confirmation:** primary outreach (operator interviews / vendor invoices) plus county provider-enrollment and CDTFA/Turlock Finance receipts would resolve per-business revenue, payer mix, and the true independent universe at once — collapsing the lowest-confidence numbers in §1 and §3 simultaneously. [UNKNOWN — until obtained]

---

*All revenue/margin/market-size figures herein are labeled benchmark estimates (industry benchmark × observable size signal × Central-Valley haircut), not actual reported figures for any named business. City/county/state figures (population, income, demographics, license fees, the 2026 Medicare conversion factor) are cited public facts or labeled derivations. Supplier-to-named-business links are representative inference unless a cited source states the relationship. Coverage is reported as a floor. Sources are carried from `supply_ops.md`, `market_saturation.md`, `reconciled.md`, and the source dataset `businesses.json`.*

### Sources (carried from upstream)

- **Demographics & demand:** California Demographics / Census QuickFacts; US News / DataUSA (CSU Stanislaus); Stocktonia (Medi-Cal unwinding); KQED (Assyrian community); Statistical Atlas (ancestry).
- **Dental:** plus.tgpo.io (distributor share); zenone / netsuite / overjet (ADA HPI overhead); ADA 2024 Survey via Overjet/Curve; dental.dhcs.ca.gov / cda.org (Denti-Cal).
- **Optometry:** idoc / abboptical / visionsource / pipgpo (suppliers, buying groups, COGS); Vertical IQ; Review of Optometry 2023; optometry.ca.gov.
- **Chiropractic:** scriphessco / meyerdc; calchiro / chiro.ca.gov; chirotouch (startup costs).
- **Physical Therapy:** webpt / coremedicalgroup / financialmodelslab; ptbc.ca.gov; 2026 Medicare conversion factor.
- **Fitness & yoga:** financialmodelslab / roguefitness; MMCG / WellnessLiving 2024; marianatek.
- **Workforce ratios:** CDC/NCHS (dentists); FCLB / Springer (chiropractors).
