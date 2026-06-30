# Turlock Home & Contractor Trades — Sector Deep-Dive

**The definitive synthesis of the 22-business independent Home & Contractor Trades universe in Turlock, CA (Stanislaus County).**

This document merges three upstream analyses — supply-chain & operations (`supply_ops.md`), market & saturation economics (`market_saturation.md`), and the reconciliation brief (`reconciled.md`) — against the source dataset (`businesses.json`, count = 22) into a single sector reference.

**Compiled:** 2026-06-30 · Role: Writer (synthesis).

---

## Financial & Honesty Charter (binding for this document)

This deep-dive obeys the following rules without exception:

- **Per-business dollar revenue is NOT public** for these privately held independents. **No dollar figure in this document is stated as an actual reported figure for any named business.**
- Every dollar figure tied to a *sub-category* is a **labeled benchmark estimate** — a published industry benchmark (revenue per truck / per technician / per employee / per crew) scaled by an **observable size signal** (years in business, residential-vs-commercial mix, showroom presence, fleet/crew language) — explicitly tagged **[ESTIMATE]** with a **method, source, and confidence level**.
- **City / county / state figures** (population, income, demographics, licensing rules, bond and fee amounts) are **cited public facts** tagged **[FACT]**, or are explicitly labeled derivations/estimates.
- **Supplier-to-named-business links are representative inference**, not confirmed accounts, unless a cited source states the relationship.
- Where a *high gross* revenue band exists, it is paired with the *thin net margin* caveat so high revenue is never mistaken for high profit.
- Tags used throughout: **[FACT]** (cited public fact), **[ESTIMATE]** (labeled benchmark estimate), **[UNKNOWN]** (gap we cannot responsibly fill).
- **Coverage is reported as a FLOOR, not a ceiling.** The catalog of 22 captures *identifiable, web-present independents*; the cash-basis informal tail is real and uncounted.
- No fabrication. Where a number is uncertain, it is flagged as such.

---

## Table of Contents

1. [Sector overview & the named businesses](#1-sector-overview--the-named-businesses)
2. [Why this many businesses, not more](#2-why-this-many-businesses-not-more)
3. [Business models & unit economics per sub-category](#3-business-models--unit-economics-per-sub-category)
4. [Supply chain & operations](#4-supply-chain--operations)
5. [Competitive structure — crowded vs. white-space](#5-competitive-structure--crowded-vs-white-space)
6. [Risks, unknowns & coverage caveats](#6-risks-unknowns--coverage-caveats)

---

## 1. Sector overview & the named businesses

Turlock's identifiable independent home-and-trades universe is **22 firms across six sub-categories** [FACT, verified against `businesses.json`]. Unlike storefront retail or food service, this is fundamentally a **truck-and-tools sector**: the defining capital is service vans, specialized tools, and — for the licensed trades — a contractor's license, *not* floor inventory or a customer-facing storefront. Five of the six sub-categories are mobile, dispatch-based service operations; flooring is the lone storefront/showroom exception [FACT].

The single most important structural fact about this sector is that it runs on **two distinct economic engines**, and almost every downstream conclusion flows from that split:

- **Engine A — licensed, event-driven mechanical trades** (electrical, HVAC, plumbing). These carry real moats: a CSLB license tied to a Responsible Managing Officer, bonds, workers' compensation, EPA refrigerant certification (HVAC), and — the binding constraint — skilled journeyman/apprentice labor that cannot be conjured quickly. Demand is largely non-discretionary and event-driven (a dead water heater, a failed AC in a 100°F July, a tripping panel).
- **Engine B — easy-entry, labor-is-the-product recurring services** (janitorial/cleaning, landscape maintenance). These have near-zero licensing barriers, thin supply chains, and low capex; the "product" sold is recurring crew-hours on standing accounts. Defensibility comes from contract retention, not from any moat in supply or capital.

This division — moat strength on one side, saturation on the other — is the spine of the entire analysis.

### The 22 businesses by sub-category

**Counts (primary category) [FACT, from `businesses.json`]:** Landscaping **7** · Plumbing **6** · Janitorial/Cleaning **3** · HVAC **2** · Electrical **2** · Flooring **2** = **22**.

> **Reconciliation note on the plumbing count.** The upstream `market_saturation.md` analysis listed plumbing as **5**; the catalog and `supply_ops.md` both show **6 plumbing-primary firms**. The reconciliation brief confirms **6 is correct** — `market_saturation` appears to have netted out DeHart (which is plumbing-primary with HVAC as a *secondary* line) and treated it as HVAC. The sector density figure (3.0/10k, computed on the total of 22) is unaffected; only the per-sub-category split is corrected here to 6.

> **Functional vs. headline HVAC supply.** The raw HVAC primary count is 2, but two plumbing-primary firms (DeHart; Turlock Plumbing Heating & Air) carry HVAC as a secondary line. Functional HVAC capacity is therefore closer to **~4** [ESTIMATE, from catalog secondary-category tags] — the "thin" label applies only to the headline primary count.

| Sub-category (n) | Named businesses | Notable signal |
|---|---|---|
| **Electrical (2)** | Applied Electrical Services (AEI; LIC# 885461); Frayer Electric Inc. (C-10 #1101008) | Both licensed C-10; residential + commercial |
| **Flooring (2)** | Arrow Floor Covering, Inc.; Turlock Floor Covering | Only storefront/showroom model in the sector |
| **HVAC (2 primary)** | Horizon Heating & Air Conditioning (20+ yrs); Saunders Air Conditioning & Heating (since **1948**) | Multi-generation incumbents; replacement-heavy valley demand |
| **Janitorial/Cleaning (3)** | ASAP Janitorial Service (30+ yrs); Barbara's Janitorial Services (15 yrs); Ultimate Janitorial Services | Commercial-contract vs. residential/move-out split |
| **Landscaping (7)** | Capote Landscape (+ concrete); Diaz & Flores Landscape (minimal-chemical); JNG Landscape & Maintenance; Lopez Landscaping (30+ yrs); Turlock Landscaping; Valley Nursery Landscape (20 yrs); Visionary Landscape (tree care) | Most-crowded sub-category |
| **Plumbing (6)** | Johnson Plumbing (since **1960**); Mainline Plumbing (28+ yrs); PRO Plumbers (24/7); Turlock Plumbing Co (24/7); DeHart Plumbing, Heating & Air (+ HVAC); Turlock Plumbing Heating & Air (+ HVAC) | Two carry 24/7 emergency; two cross-sell HVAC |

**Who they are, in one read.** The sector skews toward **long-tenured, owner-operated incumbents**: Saunders (1948), Johnson (1960), Lopez (30+ yr), ASAP (30+ yr), Mainline (28+ yr), Valley Nursery (20 yr). Several carry posted CSLB license numbers (AEI, Frayer), and several advertise 24/7 emergency dispatch across Stanislaus County (PRO, Turlock Plumbing Co). The "newest-feeling" segment is the easy-entry tail (landscaping/janitorial), where the catalog is also the least complete (see §6). The dominant labor model across the whole sector is **licensed owner-operator + W-2 field crew(s)** of roughly 2–12 staff [ESTIMATE; method: typical single-license regional trade-contractor staffing; low confidence] — there is no booth/chair-rental model here.

---

## 2. Why this many businesses, not more

The clean answer: **Turlock has roughly this many independent home-trades firms because a hard demand ceiling and a ~15-mile leak to Modesto cap the total, while a two-speed entry barrier sorts where those firms land.** Crucially, "this many" is partly a *measurement* artifact — the true operating count is higher than 22 because the informal tail is invisible. Let's put labeled numbers on each link.

### 2a. The demand base (who buys, and how much)

The sector sells to three overlapping buyers [FACT, per `market_saturation.md`]:

1. **Homeowners** — ~73k residents [FACT: U.S. Census est. 72,919 mid-2025] in roughly **23–25k households** [ESTIMATE: 73k ÷ ~2.9 persons/household; medium confidence — a model-derived figure, not a count], median household income **~$83k** [FACT: $82,995 in 2024]. An aging housing stock (Turlock's core built 1950s–1990s) drives recurring repair/replacement demand.
2. **Light-commercial / ag-facility operators** — Turlock sits in a dense dairy, nut-processing, and food-packing belt. Janitorial, HVAC, and electrical work for offices, schools, churches, medical, and ag/food facilities is the recurring-revenue backbone (multi-year contracts).
3. **Property managers / multi-family** — apartment turns drive move-out cleaning, flooring replacement, and plumbing.

Demand splits sharply by discretion: plumbing/HVAC/electrical are **non-discretionary, event-driven** (low price sensitivity at the moment of failure); janitorial and landscape maintenance are **recurring-contract** (price-competitive on renewal); flooring and landscape install/design are **discretionary and rate-sensitive** (they soften when interest rates and remodel budgets tighten). Unlike auto-body or medical sectors, most of this work is **out-of-pocket** (homeowner cash, financing, HELOC), so demand is more income/rate-sensitive than insurance-shielded sectors [FACT/ESTIMATE per `market_saturation.md`].

**The demand ceiling.** The base is capped by population (~73k city, plus a larger rural-south-Stanislaus trade-area draw that is left **[UNKNOWN]** in magnitude), housing-unit count, and commercial-property count — all slow-growing. CSU Stanislaus adds ~8,400 undergrad / ~9,700 total [FACT: fall 2024] but those are **renters, not high-ticket trade buyers**. There is **no tourism and no large daytime in-commuter base** to inflate trade volume beyond residents + local commercial.

### 2b. Saturation read — businesses per 10k vs. benchmark

> **Sector density [ESTIMATE; method = catalog count ÷ population × 10,000]: 22 firms ÷ 73,000 residents × 10,000 ≈ 3.0 independent home-trades firms per 10,000 residents.**

This is explicitly a **FLOOR**, not a true total — it captures only identifiable independents with a web presence. Once you add solo handymen, cash-basis/informal operators (significant here given the labor pool, see §2d), and Modesto-based firms that service Turlock, **adjusted true density is plausibly 5–8 per 10k** [ESTIMATE; low confidence].

**Benchmark.** Nationally, home-services/specialty-trade contractors run on the order of **15–25+ establishments per 10k residents** when all trades and informal operators are counted [ESTIMATE; low–medium confidence; derived from national establishment counts (the U.S. has 120k+ flooring firms alone, and several hundred thousand each in plumbing, electrical, HVAC, landscaping) ÷ U.S. population]. So Turlock's catalog-visible 3.0/10k is **not "saturated on paper"** — but that gap is **mostly a measurement artifact** (informal operators are invisible) layered on **genuine leakage to Modesto**, not evidence of a wide-open market.

### 2c. Leakage to Modesto — the dominant cap

Modesto (~218k residents) sits **15 miles north** and hosts the regional concentration of larger trade firms, supply houses, national/franchise players (Roto-Rooter-class plumbing/drain brands, regional HVAC chains), and **big-box installed services** (Home Depot/Lowe's flooring, water heaters, HVAC) [FACT]. High-ticket and specialty jobs leak north, thinning the *Turlock-headquartered* count. Additional leak channels: lead aggregators (Angi/Thumbtack/Yelp) that commoditize price, online/DIY product purchasing that pulls margin out of flooring and plumbing supply, and the informal/uncataloged operators *within* Turlock that pull the low end away from named firms.

### 2d. The two-speed entry barrier — what sorts the internal shape

The same demand base produces a **crowded** easy-entry segment and a **thin** licensed segment because of a two-speed barrier:

- **Licensed trades gated hard.** CSLB licensing (C-10 electrical, C-20 HVAC, C-36 plumbing, C-15 flooring, C-27 landscaping), a **$25,000 contractor bond** [FACT], workers' comp, and — the binding constraint — **skilled-labor scarcity** filter out casual entrants in electrical, plumbing, HVAC, and flooring. This is *why* those counts are thin and concentrated.
- **Janitorial and landscape barely gated.** Janitorial is generally **not CSLB-gated** (it runs on a city business license + liability/janitorial bond), and landscape maintenance entry is near-trivial (truck-and-mower capex). Low barriers + a labor-rich community = many small operators. This is *why* those are the crowded sub-categories.

**Community-driven demand (the ceiling-shaper, not ceiling-raiser).** Turlock's demographics support more small operators per capita than a generic 73k town:
- **Mexican/Hispanic (~46% of population)** [FACT/ESTIMATE — community/Census-derived; directionally sound but not Census-precise] — a large, labor-rich entrepreneurial base feeding the crowded landscaping and janitorial segments (note the Lopez, Diaz & Flores, Capote names). Spanish-language referral channels lower customer-acquisition cost and support more sub-scale firms than headcount alone implies.
- **Assyrian (~20,000, roughly a quarter of the city); Portuguese/Azorean (~7%); Punjabi/Sikh** [FACT/ESTIMATE — community-sourced; Assyrian ancestry is not a clean Census field, so treat as a community estimate]. These tight, high-trust communities sustain referral-based, in-language trade firms and ag/dairy-facility commercial accounts (Portuguese-Azorean dairy ties; Punjabi ag/trucking; Assyrian commercial property). Community trust lets niche operators survive on word-of-mouth — supporting more small firms per capita **while simultaneously enlarging the uncataloged informal tail.**

### 2e. The synthesized causal chain

**Net read:** the count is held down primarily by **demand ceiling + Modesto leakage**; its *internal shape* (crowded easy-entry vs. thin licensed) is set by the **licensing/skilled-labor barrier**; and the headline 22 is a **visible floor**, with true operating density (~5–8/10k) higher once the cash-basis tail is counted. The clearest unfilled local headroom is **independent electrical capacity** — thin precisely because the moat that protects it also suppresses new entry (a thin count plus rising solar/EV/panel demand = genuine white-space, not a contradiction).

---

## 3. Business models & unit economics per sub-category

> **Every band below is an [ESTIMATE], not actual.** Method: published industry benchmark (revenue per truck / per technician / per employee) × an *inferred* unit count read from public signals (years in business, residential-vs-commercial language, showroom presence). Unit counts are themselves **[UNKNOWN]** for every firm, so bands are wide on purpose. Central Valley wage and ticket levels run modestly below coastal CA, so true local figures likely sit at the **lower-middle** of each band [ESTIMATE; low–medium confidence]. No dollar figure is asserted for any named business.

### Cost-structure backbone (labeled estimates)

Sources: ServiceTitan / Projul / EdgeStrat / Siana contractor-margin guides 2026 (trades); Aspire / TurfBooks / RealGreen 2026 (landscape); TheJanitorialStore / Housecall Pro / Jobber 2026 (cleaning).

| Sub-category | Gross margin [EST] | Labor as % of revenue [EST] | COGS/material exposure | Net margin [EST] | Confidence |
|---|---|---|---|---|---|
| **Plumbing / Electrical — service & repair** | 35–55% | ~25–35% | Low–med (fittings/devices) | **10–20% net** | Medium |
| **HVAC** | 20–35% (install) / higher on repair | ~25–35% | **High** (condensers/furnaces) | **5–12% net** | Medium |
| **Flooring (showroom + install)** | 25–40% | ~15–25% (install crews) | **High** (product is special-order) | **5–12% net** | Low–med |
| **Landscaping — maintenance** | 45–55% | **40–50%** (labor-dominant) | Low (2–6% material) | **10–20% net** | Medium |
| **Landscaping — install/hardscape** | 35–50% | ~15–25% | 20–30% material | **10–15% net** | Medium |
| **Janitorial/Cleaning** | 40–55% (commercial) | **50–60%** (labor IS the product) | Very low (chemicals/paper) | **10–25% net** | Medium |

**Cross-cutting cost drivers [ESTIMATE unless tagged]:** labor burden adds **~25–40% on top of base wages** (employer FICA 7.65%, FUTA/SUTA, workers' comp, benefits, PTO) [ESTIMATE; high confidence] — and workers' comp rates are *higher* for HVAC-class risk, a mandatory non-trivial line for C-20. Capex is a **no-rent / mobile profile** [FACT] (vans + specialized tools, not retail lease — flooring showrooms excepted). Fixed annual gates: $25k bond premium, GL premium, WC (rate-driven), CSLB renewal (~$450/cycle range) [ESTIMATE; low–med]. **Emergency / after-hours repair is the sector's highest-margin work** — repair gross (35–55%) materially exceeds install gross [ESTIMATE; medium].

### Per-firm revenue bands (labeled estimates)

> Built from national per-truck / per-tech / per-employee productivity benchmarks × the observable size signal. **None is a stated fact about any named business.**

| Sub-category (n) | Size signal used | Benchmark unit & source | Per-firm revenue band [ESTIMATE] | Confidence |
|---|---|---|---|---|
| **Plumbing (6)** | 1–6 trucks; mix of solo (PRO, Turlock Plumbing Co) and multi-decade firms (Johnson 1960, Mainline 28+ yr) | $350k–$550k revenue/truck (Level/Pineido 2026) | Solo/owner-op: **$300k–$500k**; established multi-truck: **$800k–$2.5M** | Medium |
| **HVAC (2 primary + 2 secondary)** | Saunders since 1948, Horizon 20+ yr; replacement-heavy hot valley | $400k–$650k revenue/truck (Level 2026) | Established multi-truck: **$1M–$3M**; smaller: **$400k–$700k** | Medium |
| **Electrical (2)** | AEI and Frayer; residential + commercial, licensed C-10 | Journeyman target $180k–$250k rev/electrician (NECA); 4.5–5× burdened cost on service | 1–3 electricians: **$300k–$600k**; small commercial-capable: **$600k–$1.5M** | Medium |
| **Flooring (2)** | Both showroom + install (Arrow, Turlock Floor Covering) | ~$563k avg gross/installer-firm (franchise data); showroom adds ~$60k fixed lease | Showroom + install: **$500k–$1.5M** | Medium |
| **Landscaping (7)** | Maintenance + install; 20–30+ yr firms (Lopez, Valley Nursery) to smaller crews | ~$107k rev/employee (Aspire); multi-crew $500k–$2M | Solo/single-crew: **$80k–$250k**; multi-crew: **$400k–$1.5M** | Medium |
| **Janitorial/Cleaning (3)** | Commercial+residential; ASAP 30+ yr, recurring contracts | Small firm $75k–$200k; multi-contract $250k–$1M+ | Owner-op residential-lean (Barbara's): **$75k–$200k**; commercial-contract (ASAP, Ultimate): **$250k–$900k** | Medium |

### Labor model by model archetype

The sector resolves into four labor archetypes [ESTIMATE except where tagged]:

- **Licensed-pro gated trades** (electrical, HVAC, plumbing): the business is legally bound to a CSLB license holder / RMO; journeyman/apprentice crews work under that license. Skilled labor is a **hard supply constraint**, not just a cost line [FACT — license-tied operations like AEI #885461, Frayer #1101008, Saunders 1948, Johnson 1960].
- **Recurring-contract route crews** (janitorial; landscape maintenance): the core "production" is crew-hours on standing accounts (offices, ag facilities, HOAs). Most predictable revenue, thinnest COGS, most labor-percentage-sensitive (50–60% janitorial; 40–50% landscape maintenance) [ESTIMATE; medium].
- **Project/install crews** (flooring; landscape install; water-heater/panel/system swaps): material-plus-labor jobs, higher COGS, longer durations (Capote's concrete cross-sell; the flooring showrooms).
- **Mobile dispatch + 24/7 emergency premium** (plumbing/HVAC): PRO Plumbers and Turlock Plumbing Co advertise 24/7 across Stanislaus County [FACT]; after-hours repair carries the sector's highest margins.

**Seasonality.** HVAC is strongly seasonal (summer 100°F+ cooling peak drives emergency surge) [FACT]; landscaping is spring–fall install + mow, winter cleanup/dormant pruning [FACT]; plumbing/electrical are less seasonal, tied to housing-stock age, remodels, and emergencies [ESTIMATE; medium]; janitorial is non-seasonal recurring nights/weekends [FACT]; flooring tracks remodel/real-estate cycles.

---

## 4. Supply chain & operations

> **Supplier links are representative inference**, not confirmed accounts. Named distributors are the realistic, verified-to-serve-this-geography supply universe; the link from a specific named firm to a specific supplier is inference unless a cited source says otherwise.

### The structural supply pattern

This sector's defining capital is **trucks, tools, and licenses — not floor inventory** [FACT]. There is almost no held finished-goods inventory and no storefront requirement (flooring excepted). Material flows three ways:

1. **Branch-wholesaler just-in-time** (electrical, HVAC, plumbing, landscaping/irrigation, janitorial): the contractor draws from a local "supply house" branch on a **trade-credit account**, picking up or having delivered the exact pipe/wire/fittings/condensers/sprinkler heads for booked work. Lead time is **same-day to next-day for stocked SKUs**; special-order equipment (switchgear, transformers, tankless, full HVAC systems) runs **weeks to months** [FACT for stocked lead time; ESTIMATE/medium for special-order].
2. **Showroom-reseller + install** (flooring — the one true storefront/inventory model): Arrow and Turlock Floor Covering operate showrooms sampling manufacturer goods (Shaw, Mohawk, Mannington), then dispatch install crews. Material is largely **special-order cut-to-job (~1–3 wk)**, not deep warehoused stock [FACT/ESTIMATE].
3. **Equipment + consumables** (janitorial): chemicals, paper/can-liners, floor machines from sanitary-supply distributors; the product sold is recurring labor, so COGS is low and the supply chain is thin [FACT].

### Suppliers & inputs by sub-category

| Sub-category | Representative supply nodes [FACT — representative universe] | Equipment/tooling capex [EST] |
|---|---|---|
| **Electrical** | Graybar, Rexel/Platt, CED, Ferguson Electrical, Border States; gear from Square D/Schneider, Eaton, Siemens; EV chargers, panels, breakers | **Med:** vans, hand/power tools, testers, bender; minimal shop |
| **HVAC** | Ferguson HVAC, Watsco/Carrier Enterprise, Johnstone Supply, LennoxPros, Trane/American Standard; condensers, furnaces, coils, refrigerant, ducting | **Med–high:** vans, recovery machines, gauges, vacuum pumps, EPA-spec tooling |
| **Plumbing** | Hajoca, Ferguson Plumbing, PACE Supply, Morrison Supply; Kohler/Moen/Pfister, Rheem/Bradford White heaters, PEX/copper/ABS | **Med:** vans, drain machines/cameras, press tools, jetters |
| **Flooring** | Shaw, Mohawk, Mannington, Engineered Floors; regional flooring distributors; pad, adhesive, trim, LVP/tile | **Low–med:** showroom buildout, vans, install tools (saws, stretchers) |
| **Landscaping** | SiteOne, Ewing, Horizon Distributors (Hunter, Rain Bird, Toro); local nurseries/sod, rock/aggregate/mulch yards; concrete (Capote) | **Low–med:** trucks/trailers, mowers, skid steer, hand tools |
| **Janitorial** | WAXIE, Imperial Dade, CleanSource, Uline; chemicals, can liners, paper, floor pads, vacuums/auto-scrubbers | **Low:** vacuums, buffers/auto-scrubbers, vehicles; no storefront |

**A competitive insight the market-lens alone missed:** the supply houses are **shared across all local firms** — SiteOne/Ewing/Horizon serve every landscaper; Ferguson/Hajoca serve every plumber. Commoditized, shared inputs mean **there is no supply-side moat** in the crowded segments. In landscaping and janitorial this directly enables the race-to-the-bottom pricing dynamic, because no operator can differentiate on input cost. The licensed trades face higher *per-job COGS exposure* (a single condenser, water heater, or panel is a large equipment purchase) but their moat lives in licensing and skilled labor, not supply.

### Regulatory / licensing gates (operational, all [FACT] from CSLB / CA statute)

A business cannot legally bid covered work without these — they are the operational gates:

- **CSLB license** required for any project where labor + materials ≥ **$1,000** (the "$1k rule"). Relevant Class-C classifications: **C-10 Electrical** (AEI, Frayer); **C-20 HVAC** (Horizon, Saunders, and the HVAC side of DeHart / Turlock Plumbing H&A); **C-36 Plumbing** (Johnson, Mainline, PRO, Turlock Plumbing Co, DeHart); **C-27 Landscaping** (the 7 landscapers; Capote's concrete may invoke **C-8 Concrete**); **C-15 Flooring** (Arrow, Turlock Floor Covering). Janitorial is generally **not CSLB-gated** — city business license + liability/janitorial bond instead.
- **$25,000 contractor bond** on file for all licensees; **LLCs carry an additional $100,000 worker bond.**
- **Workers' compensation** required for any contractor with employees. **C-20 HVAC cannot claim a WC exemption regardless of employee status.** The broader SB 216 push toward WC-for-all-licensees was **deferred from 1/1/2026 to 1/1/2028 by SB 1455** — a near-term cost-shock that will raise the entry bar further for the smallest operators (a regulatory-timing item the market analysis omitted).
- **EPA Section 608 refrigerant certification** for HVAC techs; refrigerant-transition/A2L rules affect equipment purchasing.
- **Permits & inspection:** panel upgrades, water heaters, repipes, HVAC changeouts, and irrigation/backflow typically require City of Turlock / Stanislaus County building permits; backflow testing is a recurring landscape/plumbing gate [ESTIMATE; high confidence — standard CA municipal practice].
- **Cal/OSHA, DOT** (larger service vehicles), and **CA DPR / county Ag Commissioner license** for chemical applicators (landscape pesticide; Diaz & Flores' minimal-chemical approach reduces this exposure).

---

## 5. Competitive structure — crowded vs. white-space

### Crowded (low barrier, fragmented)

- **Landscaping (7) — the single most-crowded sub-category.** Easy entry (no high-barrier license relative to demand, truck-and-mower capex), a large Hispanic labor/entrepreneur base, and overlap with the unlicensed informal tail. Supply is commoditized and shared, so competition is on **price and relationships**; commodity maintenance is a race to the bottom. Differentiation comes only via design/install, irrigation, lighting, or eco positioning (Diaz & Flores' minimal-chemical/whole-systems angle; Capote's concrete cross-sell; Visionary's tree care). The operational risk is **labor cost/availability (40–50% of revenue), not material supply** [ESTIMATE].
- **Janitorial/Cleaning (3 cataloged, larger informal tail).** Recurring commercial contracts are defensible *once won*, but entry is trivial and the informal residential-cleaning market is deep. Competition splits between **commercial-contract players** (ASAP, Ultimate — stickier, higher revenue) and **residential/move-out operators** (Barbara's — lower revenue, higher churn). The moat is **contract retention**, not supply or capex.

### Concentrated / thin (licensed-trade moat)

- **Plumbing (6)** — moderately served; licensing + 24/7 emergency capability + decades of brand (Johnson since 1960) create defensible positions. A new entrant fights established names but it is not a commodity scrum like landscaping.
- **HVAC (2 primary, +2 plumbing cross-sellers)** — **thin on headcount but functionally better-served (~4)** than the headline suggests; replacement-driven, weather-amplified, high-ticket. Defensible via brand longevity (Saunders 1948).
- **Electrical (2)** — **the thinnest licensed trade and the clearest white-space on count.** Only two cataloged C-10 firms for a 73k city with growing solar/EV/panel-upgrade demand. Likely supplemented by Modesto firms and solar installers, but **local independent electrical capacity looks under-supplied** [ESTIMATE].
- **Flooring (2)** — thin; only two showroom/install firms, with big-box (Home Depot/Lowe's installed flooring) and Modesto retailers absorbing much demand.

### What an entrant should know

**White-space candidates (where demand and operations both permit):**
1. **Local electrical capacity — especially solar/EV/panel upgrades.** The single best-supported headroom: thin count + rising non-discretionary demand, with the moat (licensing, skilled labor) high enough to protect a well-run entrant.
2. **Commercial HVAC for the ag/food-facility belt** — recurring, high-ticket, defensible.
3. **Premium/specialized landscape** (water-wise/drought design, lighting) — the only defensible position *above* the commodity-mowing floor.

**Succession white-space (an entry path neither upstream file fully developed).** Multi-generation incumbents — Saunders (1948), Johnson (1960), Lopez (30+ yr), ASAP (30+ yr) — imply **owner-age and succession risk**, and therefore a potential **acquisition entry path** rather than a green-field build. For a buyer, acquiring an established license-tied book of business sidesteps the skilled-labor and brand-tenure barriers that otherwise gate the licensed trades [ESTIMATE — inferred from business-age signals].

**The go/no-go rule.** Don't add a ninth commodity-mowing landscaper or a fourth residential cleaner into the price-competitive easy-entry tail. The genuinely under-served local opportunity sits in the **licensed trades — above all electrical** — where Modesto leakage and licensing barriers have kept the local count thin, and where the moat that suppresses entry also protects whoever clears it.

---

## 6. Risks, unknowns & coverage caveats

1. **Per-business revenue is not public and is never stated as fact here.** Every dollar figure is a sub-category benchmark × an observable size signal, labeled [ESTIMATE] with a confidence level. The most defensible per-business read is the §3 per-sub-category band, not any point figure.

2. **Coverage is a FLOOR, not a ceiling.** The catalog of 22 captures *identifiable, web-present independents*. The cash-basis, unlicensed, solo informal tail — especially in landscaping and house-cleaning, and especially given the labor-rich community — is real and uncounted. True operating density is plausibly **5–8/10k vs. the 3.0/10k catalog figure** [ESTIMATE; low confidence]. Read every "thin" sub-category as "thin *among cataloged firms*."

3. **Trade-area population is [UNKNOWN] above the city figure.** Demand is capped at ~73k city residents in the analysis, but the firms also draw from rural south Stanislaus County. The true addressable base is somewhere above 73k and is not quantified — which means the saturation read is, if anything, conservative on the demand side.

4. **The plumbing count was corrected (5 → 6).** `market_saturation.md` stated plumbing as 5; the catalog shows 6 plumbing-primary firms (DeHart is plumbing-primary with HVAC secondary). Corrected throughout this document. Density (3.0/10k on 22 total) is unaffected.

5. **Demographic percentages are community-sourced, not Census-precise.** The Assyrian (~20,000) and Mexican/Hispanic (~46%) figures are directionally sound community/Wikipedia-derived estimates — Assyrian ancestry in particular is not a clean Census field — and are tagged [FACT/ESTIMATE, community-sourced] rather than bare [FACT].

6. **Supplier attributions are representative inference.** Turlock independents do not publish vendor lists. Named distributors (Ferguson, Hajoca, SiteOne, Ewing, WAXIE, etc.) are the realistic, verified-to-serve-this-geography supply universe; the link from a specific named firm to a specific supplier is inference unless a cited source states it.

7. **Unit counts driving the revenue bands are [UNKNOWN].** Trucks/crews/employees per firm are inferred from public signals (years in business, residential-vs-commercial language, showroom presence), so bands are wide on purpose. Central Valley wage/ticket levels run modestly below coastal CA, so true local figures likely sit at the lower-middle of each band [ESTIMATE; low–medium].

8. **Near-term regulatory cost-shock.** The SB 216 → SB 1455 deferral pushes workers-comp-for-all-licensees to **1/1/2028** [FACT]. When it lands it raises the fixed-cost floor on the smallest licensed operators, which would tighten the licensed-trade count further — a moving part not yet priced into the current saturation read.

9. **Static snapshot.** This is a 2026-06-30 view. Solar/EV/panel demand is rising, an aging housing stock keeps replacement demand steady, and Modesto's gravitational pull is an estimate of cross-shopping behavior, not a measured leakage figure. The reads will move over time.

---

*All revenue/margin/market-size figures herein are labeled benchmark estimates (industry benchmark × observable size signal), not actual reported figures for any named business. City/government figures are cited public facts or labeled derivations. No per-business dollar revenue is stated as fact. Supplier-to-named-business links are representative inference unless a cited source states the relationship. Coverage is reported as a floor. Sources are carried from `supply_ops.md`, `market_saturation.md`, `reconciled.md`, and the source dataset `businesses.json` — including CSLB (cslb.ca.gov; SB 216/SB 1455), distributor sources (graybar, ferguson, hajoca, siteone, ewing, waxie, etc.), and cost/market benchmarks (servicetitan, projul, level, pineido, aspire, jobber, U.S. Census QuickFacts, KQED, DataUSA).*
