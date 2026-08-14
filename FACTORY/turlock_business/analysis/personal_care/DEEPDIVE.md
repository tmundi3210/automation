# Turlock Personal Care — Sector Deep-Dive

**The definitive synthesis of the 25-business independent Personal Care universe in Turlock, CA (Stanislaus County).**

This document merges three upstream analyses — supply-chain & operations (`supply_ops.md`), market & saturation economics (`market_saturation.md`), and the reconciliation brief (`reconciled.md`) — against the source dataset (`businesses.json`, count = 25) into a single decision-grade sector reference.

**Compiled:** 2026-06-30 · Role: Writer (synthesis).

---

## Financial & Honesty Charter (binding for this document)

This deep-dive obeys the following rules without exception:

- **Per-business dollar revenue is NOT public** for privately held independents. **No dollar figure in this document is stated as an actual reported figure for any named business.**
- Every dollar figure tied to a *sub-category* is a **labeled benchmark [ESTIMATE]** — a published industry benchmark (revenue per chair / per station / per provider / per artist) scaled by an **observable size signal** (chair count, station count, provider count, format) — explicitly tagged with method, source, confidence, and the word "estimate."
- **City / county / state figures** (population, income, demographics, license requirements) are **cited public facts** tagged **[FACT]**, or are explicitly labeled derivations/estimates.
- **Supplier-to-named-business links are representative inference**, not confirmed contracts, unless a cited source states the relationship (a few are explicitly self-disclosed and are noted as such).
- Where a *high gross* revenue band exists, it is paired with the *thin net / cost-structure* caveat so high revenue is never mistaken for high profit.
- **Coverage is reported as a FLOOR, not a ceiling** — the catalog deliberately excludes chains, mall/Walmart counters, and below-threshold solo/home/booth operators, so true on-the-ground density is **higher** than every count shown here.
- Tags used throughout: **[FACT]** (cited public fact), **[ESTIMATE]** (labeled benchmark), **[UNKNOWN]** (acknowledged gap). No fabrication.

---

## Table of Contents

1. [Sector overview & the named businesses](#1-sector-overview--the-named-businesses)
2. [Why this many businesses, not more](#2-why-this-many-businesses-not-more)
3. [Business models & unit economics per sub-category](#3-business-models--unit-economics-per-sub-category)
4. [Supply chain & operations](#4-supply-chain--operations)
5. [Competitive structure — crowded vs white-space](#5-competitive-structure--crowded-vs-white-space)
6. [Risks, unknowns & coverage caveats](#6-risks-unknowns--coverage-caveats)

---

## 1. Sector overview & the named businesses

Turlock sustains a cataloged **25 independent Personal Care businesses** — a count that is best read as a **floor**, because chains (Great Clips / Sport Clips / Supercuts-type), Walmart and mall nail counters, and below-threshold solo/home/Instagram-booked operators are deliberately excluded. The sector is not one market but **two structurally different sub-sectors** that happen to share a storefront aesthetic: a **low-capex, distributor-fed, recurring-essential** group (barber, hair, nail, day spa, tattoo) and a **medical-procurement, discretionary/aspirational, high-capex** group (med spa). Every upstream analysis converges on this fault line — supply-ops frames it as beauty-distribution vs. medical procurement; the market memo frames it as recurring-essential vs. discretionary demand; they are the same split seen from two sides.

**The market (public facts).** Turlock's resident base is approximately **72,500 people** [FACT — California-Demographics / Census QuickFacts, 2024], median household income **~$83,000** [FACT], median age **34.9** [FACT]. The population is **46% Hispanic / 41% White (non-Hispanic) / ~6% Asian** [FACT]. Overlaid on the resident census are roughly **8,400 CSU Stanislaus undergraduates** (~9,300 total enrollment, Fall 2024) [FACT — csustan.edu IEA / US News], a term-time population that disproportionately consumes nails, brows/lashes, fades, and first tattoos. The city also carries unusually deep heritage communities — **Assyrian** (~one-quarter of the city, 4th-highest Assyrian-ancestry share in the U.S.) [FACT — Wikipedia/AASA], **Portuguese/Azorean** (~7% ancestry) [FACT], a near-majority **Mexican** base [FACT], and a **Punjabi/Sikh** community — each generating wedding / quinceañera / festival event-beauty demand that a generic 72k town would not carry.

**The 25 businesses by sub-category (primary category as spine).** All 25 are verified independents. Three carry a secondary Spa/Day Spa tag (Bliss, Salon Two Zero Nine, Organic Nails), which is why some upstream spa rows read higher than the single standalone count.

### Barber Shop (5)

| Business | Confidence | Note |
|---|---|---|
| **Barber Club Turlock** | high | Men's haircuts and grooming. |
| **Get Faded Barbershop** | high | Appointment-based fades; runs a **Square** booking site (POS [FACT]). |
| **Godspeed Barbers Turlock** | high | Appointment-based men's cuts; **Square** site (POS [FACT]). |
| **Jack's Barbershop** | med | Walk-in and appointment; Instagram-led. |
| **The Fade Room** | med | Fades and men's grooming; Instagram-led. |

### Men's Grooming (1)

| Business | Confidence | Note |
|---|---|---|
| **Noblemens Grooming** | high | Men's cuts/grooming with guy-friendly stylists and **retail grooming product** (pomade/beard oil) — a higher-margin attach. Secondary: Barber Shop. |

### Hair Salon (3, full-service; excludes solo booth renters)

| Business | Confidence | Note |
|---|---|---|
| **Bliss Salon and Day Spa** | high | Hair, esthetics, lash, facials, waxing; described as "one of the largest salons in the valley." Secondary: Spa/Day Spa. |
| **Inspirations Salon** | high | Full-service cuts, color, highlights. |
| **Salon Two Zero Nine, Inc.** | high | **Independent stylists + private suites**, custom color, extensions, head-spa, esthetics, lash — a named booth-rental / suite model. Secondary: Spa/Day Spa. |

### Nail Salon (7)

| Business | Confidence | Note |
|---|---|---|
| **Amazing Nails & Spa** | high | Manicures, pedicures. |
| **Cathy Nails** | high | Nail services. |
| **Cong's Nails and Spa** | high | Manicures, pedicures, threading. |
| **Infinity Nail Spa** | high | Shellac/gel, acrylic, nail art, body waxing. |
| **Jennifer Nail & Spa** | med | Nail services. |
| **KV Nails & Spa** | high | Manicures, pedicures, spa services. |
| **Organic Nails and Spa** | high | **SNS dipping powder** (a named match to the dip-powder supply channel), spa pedicure, waxing, facial, lash, permanent makeup. Secondary: Spa/Day Spa. |

### Med Spa / Aesthetics (3)

| Business | Confidence | Note |
|---|---|---|
| **Advance Med Spa** | med | Botox, chemical peels, microneedling, laser lipo; part of Advance Urgent Care and Med Spa (built-in medical structure). |
| **Aesthetic Lab** | high | Botox, fillers, laser hair removal, microneedling, facials, body contouring, hair restoration. |
| **Face&Beyond Aesthetic Lounge** | high | **Multi-manufacturer injectables** (Botox, Dysport, Jeuveau, Daxxify, Xeomin), fillers, laser, microneedling, medical-grade skincare — a named match to the multi-brand injectable supply chain. |

### Spa / Day Spa (1 standalone)

| Business | Confidence | Note |
|---|---|---|
| **The Spa Turlock** | high | Massage, waxing, facials, retail beauty products. (Plus spa-secondary at Bliss, Salon Two Zero Nine, Organic Nails.) |

### Tattoo / Piercing Studio (5)

| Business | Confidence | Note |
|---|---|---|
| **Colossal Tattoo** | high | Walk-ins welcome. |
| **Main Street Tattoo** | high | Custom tattoos, walk-ins welcome. |
| **Moezart Tattoo** | med | Tattoo studio. |
| **Rebelucio Tattoo** | med | Tattoo artist/studio. |
| **Third Eye Tattoo** | med | Downtown Turlock studio. |

**Category roll-up:** Barber 5 · Men's Grooming 1 · Hair Salon 3 · Nail Salon 7 · Med Spa 3 · Spa/Day Spa 1 · Tattoo/Piercing 5 = **25** (with three spa secondaries). Counts agree across all three upstream memos and the source dataset.

---

## 2. Why this many businesses, not more

The single best-supported answer is **demand-governed, not supply-governed**: a fixed local demand pool supports a finite number of recurring-essential chairs, and that **demand ceiling — not supply-chain friction — is the primary cap**. Supply chain and capex explain the *mix* and *margins* of who exists; the *count* is set by how many residents need a recurring cut/fill, minus what leaks away, gated at the top tier by licensing. The forces below stack into the equilibrium of ~25.

### 2.1 Saturation read — businesses per 10k (a FLOOR)

**Method:** cataloged independent count ÷ (population / 10,000). Population 72,500 → divisor **7.25**. These are independents only; chains, mall/Walmart counters, and home/booth-rent solo operators below the catalog threshold are **not** counted, so true density is **higher** than every figure shown. The per-10k numbers are therefore a floor, and where they read "thin," that is frequently a counting artifact, not slack demand.

| Sub-category | Count | Per 10k | US benchmark per 10k | Read |
|---|---|---|---|---|
| **Nail Salon** | 7 | **0.97** | **~3.4 / 10k** (≈113k–118k US salons ÷ ~335M) [ESTIMATE — Poidata/IBIS, med confidence] | Below national density *on independents-only*; with chain/mall/solo counters likely at-par. **Locally crowded** in lived terms. |
| **Barber Shop** (+grooming hybrid) | 5 (+1) | **0.69–0.83** | no clean benchmark | Crowded for a 72k town; CSU + heavy Hispanic male base supports it. |
| **Tattoo/Piercing** | 5 | **0.69** | **~0.35 / 10k** (~11,600 US studios ÷ 335M) [ESTIMATE — ResearchAndMarkets 2024, med confidence] | **~2× national density — notable oversupply signal.** |
| **Hair Salon** | 3 | **0.41** | densest US personal-care format | **Undercounted** — booth-rent stylists inside Bliss/Salon 209/suites are not separate rows; real density far higher. |
| **Med Spa/Aesthetics** | 3 | **0.41** | growing, no stable per-capita benchmark | **White-space-ish / under-built** — healthy for 72k; gated by entry barrier. |
| **Day Spa (standalone)** | 1 | **0.14** | — | Thin as standalone; mostly bundled into salons. |
| **Sector total** | **25** | **3.45** | — | Moderate aggregate density; concentration in nail/barber/tattoo. |

**A critical honesty correction (carried from reconciliation):** the nail "0.97 vs ~3.4" gap must **not** be cited as evidence of headroom. The ~3.4 benchmark counts *all* salons — chains, mall/Walmart counters, solo booth operators — which the catalog explicitly excludes. The defensible verdict is that nail is **locally crowded in lived competitive terms** (low differentiation, walk-in-substitutable, price-posted) even while the independents-only ratio reads thin. The thinness is a counting method artifact, not demand slack.

### 2.2 The demand ceiling (the primary cap)

Personal Care splits into two demand rhythms, and the count is anchored by the first:

- **Recurring-essential cadence** — barber (men, every 2–4 weeks), nail fills (every 2–3 weeks), basic women's cut/color (every 6–10 weeks). Sticky, habit-driven, recession-resilient. A $25–$40 men's cut or a $35–$60 mani-pedi is a small-ticket habit, not a deferrable luxury. [Cadence and price points are industry-pattern observations, ticket figures ESTIMATE — Booksy/joinblvd, med confidence.]
- **Discretionary / aspirational cadence** — med spa injectables (every 3–4 months), day-spa massage/facials, lash extensions, tattoos. Higher ticket ($150–$700+), income-elastic, first to be cut when budgets tighten.

A town of **72,500 residents at ~$83k median income** [FACT] supports a finite number of recurring-cut/fill chairs. Nail and barber sit at or near **local saturation**: adding an 8th nail salon mostly **reshuffles share** rather than growing the pie. Turlock's income sits above the Stanislaus County average but well below coastal-CA metros, so the town supports **volume formats** (value nail/barber) more readily than premium-only formats — which is exactly the observed mix.

### 2.3 Leakage to Modesto (trims discretionary spend)

Modesto (~15 miles north, ~218k people) is the regional retail/medical gravity center. **Higher-ticket and specialty demand — premium med spa, plastics-adjacent aesthetics, luxury spa days, specialty tattoo artists — leaks north.** Turlock keeps the recurring-essential, convenience-driven spend and loses the "make-a-day-of-it" discretionary spend. This is **[ESTIMATE — inferred from Modesto's size/retail role; no published Turlock-specific leakage study found]**, and the **magnitude is [UNKNOWN] — the single largest unmeasured variable in this analysis.**

### 2.4 Online / DTC and at-home substitution (caps per-visit spend)

Grooming products, at-home gel/dip kits, clipper/DIY culture, and at-home skincare cannibalize the **product-attach and low-end-service margin**, capping per-visit spend and the number of chairs a fixed population can support. [Substitution effect is directional/qualitative; magnitude [UNKNOWN].]

### 2.5 Licensing barriers (decisive only for the high-capex tier)

CA Board of Barbering & Cosmetology (BarberCosmo) licensure gates barbers, cosmetologists, estheticians, and manicurists — a real but **routine** gate that does not by itself cap the count. The **binding** barrier is med spa: a **physician medical-director oversight** structure plus **$50k–$150k device capex** [ESTIMATE — vendor/industry pricing, moderate confidence] is "the single biggest entry barrier" and explains why only **~3** exist despite attractive category economics. Crucially (per reconciliation), med spa "white-space" should be read as **"high-barrier, under-built"** — not "easy to enter." The opportunity exists *because* the barrier suppresses entrants, not because demand is unmet-and-easy.

### 2.6 The lone expander — ethnic-community event demand (lifts the count above a generic 72k town)

Turlock's deep heritage communities create **wedding / quinceañera / festival / event-driven beauty demand** — formal updos, bridal, brow/lash, henna-adjacent services, group bookings — that **sustains more salon/nail/event capacity than population alone would predict.** This is the one structural feature pushing the count *modestly above* what a generic 72k town would carry, and it is a **defensible, under-formalized niche** national chains cannot serve.

**Net synthesis:** demand ceiling caps the count; Modesto leakage and online/at-home substitution trim it; licensing gates only the high-capex med-spa tier; ethnic-event demand is the lone offsetting lift. **~25 is the equilibrium** where the recurring-essential floor is filled, nail/barber/tattoo run at-or-above local saturation, and med spa / premium spa remain under-built behind a capital-and-licensure wall.

---

## 3. Business models & unit economics per sub-category

> **Every band below is an [ESTIMATE], not actual.** Method = published industry benchmark × an observable Turlock size signal (chairs/stations/providers/artists), haircut for a sub-metro market where Turlock pricing and volume run below US-average metros. Confidence reflects how reliable the benchmark and the size-signal mapping are. **No figure is a claim about any named business.** Booth/chair rental shifts cost lines onto renters, so owner-level and operator-level economics diverge — flagged per row.

### 3.1 The labor model is the sector's defining variable

- **Booth/chair-rental dominant in barber and hair [FACT].** Owners rent stations to independently-licensed pros who keep service revenue, issue a 1099 for rent, carry their own insurance, and book their own clients. Salon Two Zero Nine explicitly advertises "independent stylists" and "private suites" — a named booth-rental/suite model. This converts the **owner's revenue to predictable rent** rather than service margin, and shifts COGS, scheduling, and client risk onto the renter.
- **Owner-operator / small W-2 crews** dominate single-chair shops and walk-in tattoo studios.
- **Med spa** requires **medical-director oversight** — a physician (MD/DO) owns/supervises the medical acts; injectors are RNs/NPs/PAs under delegation. A structural labor/cost input unique to the three med spas.
- **The live 2025 nail shock [FACT]:** CA's temporary AB 5 booth-rental carve-out for **manicurists expired Jan 1, 2025** — nail techs must now be **W-2 employees** (hair/skin/barber pros retain the Borello carve-out). See §3.4.

### 3.2 Revenue bands by sub-category (labeled estimates)

| Sub-category (count) | Size signal | National per-unit benchmark (source) | Turlock-adjusted band/yr — *estimate, not actual* | Confidence |
|---|---|---|---|---|
| **Barber Shop (5)** | # chairs (typ. 3–6) | $60k–$100k rev/chair well-run; weak shops <$50k/chair (joinblvd/Booksy 2024) | **$90k–$280k**; small 2-chair walk-in lower | Medium |
| **Men's Grooming (1)** | chairs + product attach | barber per-chair + higher ticket | **$130k–$300k** | Low–Med |
| **Hair Salon (3)** | # stylist stations (booth-rent) | ~$245k avg US salon; ~$76k rev/employee (trafft/joinblvd 2024) | **House $150k–$500k**; booth renters individually **~$40k–$90k take** | Medium |
| **Nail Salon (7)** | # mani/pedi stations (typ. 6–12) | US avg $250k–$460k/location; ~$34 rev/capita (Statista/Kentley 2024) | **$150k–$420k**; high-volume strip-center toward top | Medium |
| **Med Spa/Aesthetics (3)** | # injector-providers (1–3) | $300k–$600k rev/provider; **~$1.4M avg single-location** [ESTIMATE — AmSpa/joinblvd national, NOT Turlock] | **$350k–$1.2M**; injectable-led | Low–Med (wide variance) |
| **Day Spa (1)** | # treatment rooms/tables | massage/facial room economics | **$120k–$350k** | Low |
| **Tattoo/Piercing (5)** | # artists (1–5) | 1–2 artist $60k–$150k; 3–5 artist $150k–$350k (Vagaro/ResearchAndMarkets 2024); avg artist ~$52k | **$60k–$300k**; most local 1–3 artists → lower band | Medium |

**Two honesty flags (carried from reconciliation):** (1) the **med-spa "~$1.4M avg/location, 20–25% margin"** is an **AmSpa/joinblvd *national* benchmark [ESTIMATE], not a Turlock fact** — it must never be written as a bare assertion. (2) The **nail "~$34 revenue/capita"** is a **national Statista [ESTIMATE]** applied to a value-pricing market this very analysis haircuts; keep it explicitly tagged. Neither figure attributes revenue to a named business, so the core rule holds — but both are national-benchmark-as-estimate, never local-actual.

**Caveat on cash:** nail and tattoo are the most cash-intensive sub-categories; reported revenue likely **understates true throughput**, so those bands are conservative. The cash-gap magnitude is **[UNKNOWN]**.

### 3.3 Cost structure by sub-category (labeled estimates)

> All figures are **industry-benchmark [ESTIMATE]s** — method/source named, confidence stated. **None is a statement about a specific Turlock business's actuals.** Booth-rental shifts these line items onto renters.

**Hair salon (full-service, operator basis)** [ESTIMATE — Professional Beauty Association / Vagaro / industry P&L, moderate confidence]:
- Labor (wages + commission 30–40% + payroll tax): **~40–55% of revenue** (largest line).
- Rent / occupancy: **~6–12%**. Product COGS (color, care): **~5–12%**.
- Net margin: **~8–15% typical; 20%+ top performers** (US avg ~8% estimate, PBA-cited).
- *Booth-rental owners invert this:* revenue ≈ rent collected; labor/COGS borne by renters.

**Barber** [ESTIMATE — salon benchmarks + chair-rental structure, low-moderate confidence]: lower COGS than hair (minimal product); under chair-rental the **owner's economics are rent-driven**, with per-chair rent commonly **$150–$350/week** (regional pro-forum benchmark, low confidence — local figure [UNKNOWN]). Men's grooming layers a **higher-margin retail product attach** (Noblemens) on the same base.

**Nail salon (post-2025, W-2 basis)** [ESTIMATE — industry + new-law impact, low-moderate confidence]: historically thin margins on low ticket prices; see §3.4 for the W-2 squeeze. Magnitude for any named salon is [UNKNOWN].

**Med spa** [ESTIMATE — aesthetics-industry benchmarks, low-moderate confidence]: higher gross margins on injectables, but **product COGS is real, lot-tracked, and largely passed to price**; dominant cost drivers are **medical-director/injector labor, device lease/depreciation ($1.5k–$4k/mo/device estimate), and malpractice/medical insurance**. **Capital intensity, not COGS, is the gating economic** — high gross does NOT mean easy net.

**Tattoo** [ESTIMATE — body-art shop norms, low confidence]: predominantly **commission-split or booth-rent** with artists; consumable COGS per session is small; shop economics are rent/split-driven. Local splits [UNKNOWN].

**Day spa** [ESTIMATE]: low per-room capex and JIT consumables; economics driven by treatment-room utilization; thin as a standalone segment.

### 3.4 The compounded nail-margin squeeze (the live 2025 story)

This is the sector's most important current operating dynamic, and it is a **synthesis neither upstream memo states alone**. Two forces **stack** on the seven nail salons:

1. **Structural cost increase (supply-ops):** the forced **W-2 transition adds ~10–15%+ to effective labor cost** [ESTIMATE — industry + new-law impact, low-moderate confidence] versus the prior rent model — payroll tax + workers' comp + admin + PAGA-misclassification exposure — and disproportionately affects the Vietnamese-immigrant labor base (per CalMatters/UCLA reporting).
2. **Weak pricing power (market):** price-posted, walk-in-substitutable, low-differentiation competition with little ability to pass cost through.

**Combined = compounded margin compression in 2025.** A structural labor-cost increase landing on top of already-weak pricing power is the correct, decision-grade read — and it is the single biggest operating-model disruptor in the sector right now.

---

## 4. Supply chain & operations

> **Supplier links are representative inference**, not confirmed contracts, except where a named business self-discloses (Organic Nails' SNS dipping; Face&Beyond's multi-brand injectables) — those are noted as named matches.

The defining feature of the non-medical sub-sector is a **low-capex, distributor-fed, consumables-light, just-in-time** input model. Storefronts are small, product turns are predictable, and a handful of national beauty distributors with CA warehouses (plus Amazon/direct-to-pro e-commerce) deliver in 1–5 days. Med spa is the exception on every axis — medical procurement, cold-chain, six-figure capex.

### 4.1 Inputs → suppliers by sub-category

| Sub-category | Key named suppliers/distributors [FACT] | Core consumables | Inventory model | Equipment capex [ESTIMATE] | Lead time |
|---|---|---|---|---|---|
| **Barber / Men's Grooming** | CosmoProf, SalonCentric, BuyBarber, Vip Barber, ChopMyHair, WCK, Palms Fashion (Wahl/Andis/Oster/BabylissPro/JRL) | Blades, oil, capes, Barbicide, retail pomade | Minimal JIT | **~$1,500–$4,000/chair** | 1–5 days |
| **Hair Salon** | SalonCentric, CosmoProf (Redken/Wella/Schwarzkopf/Olaplex) | Color, developer, care lines, extensions, lash | Low–moderate (color bar) | **~$1,500–$3,000/station** + wash plumbing | 1–5 days |
| **Nail Salon** | DTK (San Jose), ND Nails, Cali/Cal Nail, Beauty Zone (Westminster), C8/LA Nail (OPI/DND/SNS/Kiara Sky/Apres) | Monomer, acrylic/dip powder, gel, tips, files | Broad-SKU JIT | **Pedi chairs ~$1,500–$3,500 ea** | 1–4 days |
| **Med Spa** | McKesson, Pipeline Medical, aesthetics-specialty (Allergan/Galderma/Evolus/Revance/Merz) | Botox/fillers (cold-chain, lot-tracked, short-dated), needles, topicals | FIFO, short-dated, opening-order minimums | **Devices $50k–$150k buy / $1.5k–$4k/mo lease** | varies; cold-chain |
| **Tattoo / Piercing** | Tattoo-specialty distributors; mandated pre-sterile needles + commercial inks only | Single-use cartridges/needles, ink, barrier film, sharps disposal | Modest single-use buffer | **~$2,000–$6,000/station** + shared autoclave | 1–7 days |
| **Spa / Day Spa** | SalonCentric, CosmoProf, skincare-specialty | Wax, skincare, massage oils, retail product | Low JIT | **~$1,000–$3,000/room** | 1–5 days |

*(All capex figures: [ESTIMATE — vendor/industry pricing, low-to-moderate confidence].)*

**Three supply-chain features worth isolating:**

- **The nail channel is a distinct CA Vietnamese-American wholesale ecosystem** — DTK (San Jose), ND, Cali/Cal, Beauty Zone (Little Saigon), C8/LA Nail — separate from the CosmoProf/SalonCentric mainstream and enabling fast regional JIT reorder of a broad slow-turn color/powder SKU long-tail plus fast-turn staples (monomer, tips, files).
- **Med spa is medical procurement, not beauty distribution** — injectables are cold-chain, lot-tracked, short-dated, FIFO, with manufacturer opening-order minimums [ESTIMATE — Allergan new-account opening orders ~$2,000–$5,000; basic launch inventory/equipment ~$10,000–$25,000, vendor/industry pricing, moderate confidence].
- **Tattoo supply is regulatorily constrained** — CA's Safe Body Art Act mandates pre-sterilized single-use needles/cartridges and commercially manufactured inks only, with a steam autoclave required for any reusable instruments [FACT].

### 4.2 The capex spectrum (the entry-cost story in one line)

Barber, nail, tattoo, and spa are **low-capex** (~$1,500–$6,000 per station/chair); **med spa is 10–50× higher** ($50k–$150k devices), which is why **leasing-not-buying** is the common cash-flow workaround there. This single fact governs the whole sector's competitive structure: low capex enables the tattoo oversupply the market observes, while high capex + licensure suppresses med-spa entry into white-space.

### 4.3 Operating cadence

- Barber/hair/nail peak **evenings, weekends, and pre-event seasons** (holidays, prom, weddings); barbers skew a weekend/biweekly cadence. [ESTIMATE — industry pattern, moderate confidence]
- Med spa and tattoo are **appointment-led** (some walk-in: Colossal, Main Street, Jack's); med spa sees gifting/event seasonality.
- All sub-categories are **cash-heavy / tip-heavy**, with Square/Booksy/Vagaro POS common (Get Faded and Godspeed run Square sites [FACT]).

### 4.4 Regulatory & licensing gates

| Gate | Requirement | Tag |
|---|---|---|
| **CA BarberCosmo** | Individual licenses (displayed) **and** an establishment license; sanitation/plumbing minimums (toilets, hot/cold running water, handwashing) before opening; booth renters each licensed, establishment verifies currency. Gates barber, hair, manicuring, esthetics. | [FACT] |
| **Worker classification** | Hair/skin/barber retain the **Borello carve-out** (booth rental permitted: separate business, own insurance, written agreement, 1099). **Manicurists lost the carve-out 1/1/2025 → must be W-2.** Retroactive withholding/workers'-comp + PAGA exposure for non-compliance. | [FACT] |
| **Med spa medical licensure** | **Physician medical-director oversight**; injectables are FDA-regulated prescription products via authorized medical distributors; medical malpractice + corporate-practice-of-medicine structure required. | [FACT] |
| **Safe Body Art Act + Stanislaus County Env. Health** | County **Body Art Facility health permit**, registered practitioners, pre-sterilized single-use needles, commercial inks only, steam autoclave, infection-control plan, plan-check, self-inspection. | [FACT] |
| **General** | City of Turlock business license; CA seller's permit (retail resale); general + professional liability insurance. **No CSLB input** applies to ongoing operations (only one-time tenant-improvement construction). | [FACT/ESTIMATE] |

---

## 5. Competitive structure — crowded vs white-space

**The big picture:** Turlock Personal Care is a **moderate-density, recurring-essential market** that retains habit-driven cut/fill/grooming spend, loses discretionary high-ticket spend to Modesto and online, and shows genuine **local oversupply in nail and tattoo** against national benchmarks while leaving **med spa and premium spa** as the clearest white-space — with **ethnic-community event demand** the one structural feature letting a 72k town carry more beauty capacity than size alone would justify.

### 5.1 Crowded (share-shuffle, not pie-growth)

- **Nail (7)** — the most contested sub-category. Heavily Vietnamese-immigrant-labor, price-competitive, low differentiation, walk-in substitutable. Competes against itself plus mall/Walmart-adjacent counters and chains. Margin pressure and weak pricing power, now compounded by the 2025 W-2 mandate (§3.4). **An entrant adds a chair to a saturated room.**
- **Barber (5 + grooming hybrid)** — saturated for the town size. Differentiation is brand/vibe (appointment-app booths like the Square-site shops) and the CSU/young-male fade niche. Competes with **national chains** (Sport Clips / Great Clips / Supercuts-type) on the value end.
- **Tattoo (5)** — **~2× national per-capita density** [ESTIMATE], a clear saturation flag. Low per-station capex *enables* the oversupply. Competes on artist reputation/portfolio (Instagram-driven), with **top-end specialty work leaking to Modesto/Bay Area** artists.

### 5.2 White-space / under-built

- **Med spa (3)** — favorable national economics, **high entry barrier (medical oversight + six-figure device capex)**, and clear leakage of premium aesthetics to Modesto = room for a **well-capitalized operator with physician oversight** to recapture spend. Read this as "high-barrier, under-built," **not** "easy to enter."
- **Premium day spa / wellness (1 standalone)** — the "destination spa day" is the format most leaking north; a credible local destination concept could recapture it.
- **Event/cultural beauty** — Assyrian / Portuguese / Hispanic / Punjabi wedding-and-formal demand (bridal teams, group bookings, on-site) is **defensible and under-formalized**, and national chains structurally cannot serve it.

### 5.3 Who pulls demand away

- **Regional (Modesto):** the dominant leakage vector for high-ticket aesthetics, specialty tattoo, luxury spa. Magnitude [UNKNOWN].
- **National chains:** value-end barber/nail/hair compress the low end on price and convenience.
- **Online/DTC + at-home:** grooming products, at-home gel/lash kits, clipper/DIY culture cap per-visit and product-attach revenue.
- **Solo/booth-rent & home operators (gray market):** below-catalog independents (Instagram-booked stylists, home nail/lash techs) fragment share — a real competitor class drawing on the **same CosmoProf/DTK distributor channels** but **[UNKNOWN] in count** (neither memo sizes it; see §6).

### 5.4 What an entrant should know (the go/no-go rule)

- **Do not add an 8th nail salon, a 6th barber, or a 6th tattoo studio** — these are at or above local saturation; an entrant reshuffles share into a margin-compressed (nail) or reputation-gated (tattoo) field.
- **Defensible headroom** is **med spa** (only if you can clear the physician-oversight + six-figure-capex wall and out-compete Modesto on convenience/trust), **premium destination spa** (recapture the day-spa spend leaking north), and **event/cultural beauty** (a format no chain can serve).
- **The cost gate the demand view alone misses:** med spa is gated by **capital + licensure**, not demand; nail is gated by the **2025 W-2 cost step-up**; tattoo's low capex is exactly why it is oversupplied. "White space" is demand-true but **capital-and-licensure-gated** — the binding constraint at the top tier is the entry wall, not the input cost.

---

## 6. Risks, unknowns & coverage caveats

1. **Per-business revenue is not public and is never stated as fact here.** Every dollar figure is a sub-category benchmark × an observable size signal, labeled estimate with a confidence level. The most defensible per-business read is the §3.2 band, never a point figure for a named shop.

2. **Coverage is a FLOOR, not a ceiling.** The catalog excludes chains, mall/Walmart counters, and below-threshold solo/home/Instagram operators. True on-the-ground density is **higher** than every per-10k figure. Where a ratio reads "thin" (nail 0.97 vs ~3.4), that is a **counting-method artifact, not demand headroom** — do not misread it as room to grow.

3. **Modesto leakage magnitude is the largest unmeasured variable [UNKNOWN].** Both upstream memos leave it unquantified; no published Turlock-specific leakage study was found. It is directionally certain (Modesto is 3× the size, 15 miles north) but the dollar magnitude is an estimate-by-inference only.

4. **The gray-market / below-catalog operator class is unsized [UNKNOWN].** Solo/home/Instagram stylists and nail/lash techs are a real competitor class drawing on the same distributor channels, but neither memo counts them or models their distributor draw. This is the most material missing supply input.

5. **Med-spa demand depth is asserted favorable but never sized [UNKNOWN].** How much of the 72k actually buys injectables at ~$83k median income is unquantified; the "white-space" read rests on national category economics + leakage logic, not a measured local injectable-buyer count.

6. **National-benchmark-as-fact drift (corrected).** Two market-memo phrasings drifted toward stated fact — the med-spa "~$1.4M avg/location, 20–25% margin" and the nail "~$34 revenue/capita." Both are **national [ESTIMATE]s, not Turlock facts**, and are tagged as such throughout (§3.2). Lower severity than local-actual-as-fact (neither names a business), but worth flagging.

7. **The 2025 nail W-2 squeeze is a live, compounding cost shock** whose per-business magnitude is [UNKNOWN]. The ~10–15%+ effective-labor-cost step-up is an industry estimate; how each of the seven named salons absorbs it (price increase, hour cuts, owner-absorption, informal non-compliance) is unobservable from public data.

8. **Supplier attributions are representative inference.** Turlock independents do not publish vendor lists. Named distributors are the realistic, verified-to-serve-this-geography supply universe; only **Organic Nails** (SNS dipping → dip-powder channel) and **Face&Beyond** (named multi-brand injectables → medical channel) have self-disclosed input links.

9. **Source confidence varies by business.** Several rows rest on aggregator/social listings (Yelp/Instagram/Facebook) at med confidence (Jack's, The Fade Room, Moezart, Rebelucio, Third Eye, Advance Med Spa, Jennifer Nail). Existence and sub-category are reliable; finer operational detail (chair/station counts, exact menus) is not independently verified — so the size signals feeding §3.2 bands are themselves estimates.

10. **Booth-rental inverts owner economics.** For booth-rent barber/hair, "house revenue" ≈ rent collected, not service margin; the §3.2 hair "house" band and the per-renter take band describe two different things and must not be summed.

11. **Static snapshot (2026-06-30).** CSU enrollment is rebounding (not booming), the nail W-2 mandate is freshly in effect, and med spa is a nationally compounding category — trajectories that will move the saturation reads over time.

---

*All revenue, margin, and market-size figures herein are labeled benchmark estimates (industry benchmark × observable size signal), not actual reported figures. City/county/state figures are cited public facts [FACT] or labeled derivations/estimates. No per-business dollar revenue is stated as fact. Supplier-to-named-business links are representative inference unless a cited source states the relationship. Coverage is a floor, not a ceiling. Sources are carried from `supply_ops.md`, `market_saturation.md`, `reconciled.md`, and the source dataset `businesses.json`.*

### Sources (carried from upstream memos)

- **Demographics:** [California-Demographics — Turlock](https://www.california-demographics.com/turlock-demographics) · [Census QuickFacts — Turlock](https://www.census.gov/quickfacts/fact/table/turlockcitycalifornia/PST045224) · [CSU Stanislaus IEA](https://www.csustan.edu/iea/institutional-data) · [Wikipedia — Turlock](https://en.wikipedia.org/wiki/Turlock,_California) · [AASA — Central Valley Assyrian community](https://assyrianaid.org/central-valley/) · [Statistical Atlas — Turlock Ancestry](https://statisticalatlas.com/place/California/Turlock/Ancestry)
- **Nail wholesale:** [DTK](https://dtknailsupply.com/) · [ND Nails](https://ndnailsupply.com/) · [Cali Nail](https://calnailsupply.com/) · [Beauty Zone](https://www.beautyzonesupply.com/) · [C8](https://c8nailsupply.com/) · [LA Nail Supplies](https://www.lanailsupplies.com/)
- **Barber wholesale:** [BuyBarber](https://buybarber.com/collections/wahl) · [Vip Barber](https://www.vipbarbersupply.com/) · [ChopMyHair](https://chopmyhair.com/) · [WCK](https://wckbarbersupply.com/) · [Palms Fashion](https://palmsfashioninc.com/)
- **Salon distribution:** [SalonCentric](https://www.saloncentric.com/) · [CosmoProf](https://www.cosmoprofbeauty.com/)
- **Med spa procurement/capex:** [McKesson Allergan/Botox](https://mms.mckesson.com/product/693836/Allergan-Botox-00023391950) · [Pipeline Medical](https://pipelinemedical.com/blog/top-10-medical-spa-supply-companies-in-the-us-and-globally-2/) · [Empire Medical Training](https://www.empiremedicaltraining.com/business-workshops/med-spa-equipment-vendors/)
- **Licensing / labor law:** [CA BarberCosmo establishment FAQs](https://www.barbercosmo.ca.gov/licensees/establish_faqs.pdf) · [Act & Regs](https://www.barbercosmo.ca.gov/laws_regs/act_regs.pdf) · [PBFC — booth rental](https://www.beautyfederation.org/labor-law/2025/cracking-down-on-booth-rental) · [PBFC — manicurists no longer booth renters](https://www.beautyfederation.org/advocacy/2024/ca-manicurists-may-no-longer-be-booth-renters) · [NAILS Magazine](https://www.nailsmag.com/1091731/california-nail-techs-may-no-longer-be-booth-renters) · [CalMatters](https://calmatters.org/politics/2025/07/workers-nail-salons-labor-bill/) · [Hackler Flynn — AB5 manicurists](https://www.hacklerflynnlaw.com/blog/2025/april/how-ab5-affects-manicurists2/)
- **Body art:** [Stanislaus County Safe Body Art Program](https://www.stancounty.com/er/environmentalhealth/safe-body-art-program.shtm)
- **Revenue/cost benchmarks:** [Vagaro salon margins](https://www.vagaro.com/learn/average-hair-salon-profit-margins) · [JoinBlvd salon profit](https://www.joinblvd.com/blog/hair-salon-profit-margin) · [Sharpsheets](https://sharpsheets.io/blog/how-profitable-is-a-hair-salon/) · [Booksy — barber profitability](https://biz.booksy.com/en-us/blog/is-being-a-barber-profitable) · [Trafft — hair salon statistics](https://trafft.com/hair-salon-statistics/) · [Kentley — nail salons](https://www.kentleyinsights.com/nail-salons-industry-market-research-report/) · [Statista — nail salons](https://www.statista.com/topics/4624/nail-salons-in-the-us/) · [Poidata — US nail salon count](https://www.poidata.io/report/nail-salon/united-states) · [JoinBlvd — med spa revenue](https://www.joinblvd.com/blog/average-medical-spa-revenue) · [AmSpa — 2024 Medical Spa State of the Industry](https://www.americanmedspa.org/news/2024-medical-spa-state-of-the-industry-executive-report-recap/) · [ResearchAndMarkets — US Tattoo Studios 2024](https://www.businesswire.com/news/home/20240627329377/en/U.S.-Tattoo-Studios-Tattoo-Removal-Services-Market-Analysis-2024) · [Vagaro — tattoo market](https://www.vagaro.com/learn/tattoo-statistics-business) · [IBISWorld — Hair & Nail Salons](https://www.ibisworld.com/united-states/number-of-businesses/hair-nail-salons/1718/)
