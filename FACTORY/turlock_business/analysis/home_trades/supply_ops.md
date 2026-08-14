# Home & Contractor Trades — Supply Chain & Operations

**Sector:** Independent Home & Contractor Trades, Turlock CA (Stanislaus County)
**Scope:** 22 named independent businesses across 6 sub-categories — Electrical, Flooring, HVAC, Janitorial/Cleaning, Landscaping, Plumbing (see `businesses.json`)
**Author lens:** Supply & Operations analyst
**Honesty:** Per-business revenue/margin is not public. Every cost %, margin, and dollar figure below is a **labeled benchmark estimate** with method + public source + confidence word and the literal word "estimate". Tags: `[FACT]`, `[ESTIMATE]`, `[UNKNOWN]`.

---

## 1. How material/inputs flow into this sector

Unlike storefront retail, this sector's defining capital is **trucks, tools, and licenses — not floor inventory.** Five of the six sub-categories are **mobile, dispatch-based service operations**: material is bought *per job* from local branch wholesalers, loaded onto a service truck, and consumed on a customer site the same day or week. There is almost no held finished-goods inventory and no storefront requirement. The structural supply pattern is:

1. **Branch-wholesaler just-in-time (electrical, HVAC, plumbing, landscaping/irrigation, janitorial):** the contractor draws from a local "supply house" branch (Ferguson, Hajoca, SiteOne, Graybar, WAXIE, etc.) on a trade-credit account, picking up or having delivered the exact pipe/wire/fittings/condensers/sprinkler heads needed for booked work. Lead time is **same-day to next-day for stocked SKUs**; special-order equipment runs longer. `[FACT]`
2. **Showroom-reseller + install (flooring):** the one sub-category with a true storefront/inventory model. Arrow Floor Covering and Turlock Floor Covering operate **showrooms** that sample manufacturer goods (Shaw, Mohawk, Mannington, etc.), sell to homeowners/contractors, then dispatch install crews. Material is largely **special-order cut-to-job** with manufacturer lead times, not deep warehoused stock. `[FACT]`
3. **Equipment + consumables (janitorial):** chemicals, paper/can-liners, and floor machines bought from sanitary-supply distributors; the "product" sold is recurring labor, so COGS is low and the supply chain is thin. `[FACT]`

The two **C-side regulated mechanical trades** (HVAC, plumbing) and electrical carry the highest **per-job COGS exposure** because a single condenser, water heater, or panel is a large equipment purchase; landscaping installs and flooring carry material exposure on hardscape/product; janitorial and pure-service repair calls are the most **labor-dominant, lowest-COGS** models.

---

## 2. Suppliers & inputs by sub-category

| Sub-category (named businesses) | Typical suppliers / inputs `[FACT]` | Flow & lead time | Inventory model | Equipment/tooling capex |
|---|---|---|---|---|
| **Electrical** — Applied Electrical (AEI), Frayer Electric | Branch supply houses: **Graybar, Rexel/Platt, CED (Consolidated Electrical), Ferguson Electrical, Border States**; gear from Square D/Schneider, Eaton, Siemens; wire/conduit, panels, EV chargers, breakers `[FACT]` | Stocked items same/next-day on trade account; switchgear & transformers special-order **weeks–months** `[ESTIMATE]` (supply-chain norm, med confidence) | JIT to job; truck stock of common devices | **Med:** service vans, hand/power tools, testers, bender; minimal shop |
| **HVAC** — Horizon Heating & Air, Saunders Air (since 1948), DeHart (sec.), Turlock Plumbing H&A (sec.) | Equipment distributors: **Ferguson HVAC, Watsco/Carrier Enterprise, Johnstone Supply, Lennox (LennoxPros), Trane/American Standard branches**; condensers, furnaces, coils, refrigerant, ducting, thermostats `[FACT]` | Stocked parts same/next-day; full systems often **dealer-stocked or 1–2 wk**; seasonal pre-buy ahead of summer `[ESTIMATE]` (med confidence) | JIT to install; some pre-buy of common tonnages | **Med-high:** vans, recovery machines, gauges, vacuum pumps, EPA-spec tooling |
| **Plumbing** — Johnson Plumbing (since 1960), Mainline, PRO Plumbers, Turlock Plumbing Co, DeHart, Turlock Plumbing H&A | **Hajoca, Ferguson Plumbing, PACE Supply, Morrison Supply**; fixtures Kohler/Moen/Pfister, water heaters (Rheem/Bradford White), PEX/copper/ABS, valves, water-filtration `[FACT]` | Stocked fittings same-day pickup; water heaters/fixtures next-day; tankless & specialty special-order `[ESTIMATE]` (med confidence) | JIT; truck stock of fittings + 1–2 common water heaters | **Med:** vans, drain machines/cameras, press tools, jetters |
| **Flooring** — Arrow Floor Covering, Turlock Floor Covering | Mill/distributor channel: **Shaw, Mohawk, Mannington, Engineered Floors**; regional flooring distributors (e.g., Sacramento/Yuba City wholesalers); pad, adhesive, trim, LVP/tile `[FACT]` | Showroom samples → **special-order cut-to-job, ~1–3 wk** `[ESTIMATE]` (industry norm, med confidence) | **Showroom + special-order** (only true storefront model) | Low-med: showroom buildout, vans, install tools (saws, stretchers) |
| **Landscaping** — Capote, Diaz & Flores, JNG, Lopez, Turlock Landscaping, Valley Nursery Landscape, Visionary (most-crowded, 7) | Irrigation/landscape supply: **SiteOne, Ewing, Horizon Distributors** (Hunter, Rain Bird, Toro); local nurseries/sod farms for plant material; rock/aggregate/mulch yards; concrete (Capote) `[FACT]` | Irrigation/hardgoods same/next-day; **plant material & sod seasonal/JIT**; bulk material by yard `[FACT]` | Mostly JIT per job; minimal yard stock | Low-med: trucks/trailers, mowers, skid steer, hand tools |
| **Janitorial/Cleaning** — ASAP Janitorial (30+ yr), Barbara's Janitorial, Ultimate Janitorial | Sanitary-supply distributors: **WAXIE (Envoy/Western states), Imperial Dade, CleanSource, Uline**; chemicals, can liners, paper, floor pads, vacuums/auto-scrubbers `[FACT]` | Consumables stocked/next-day delivery; standing replenishment to contracts `[FACT]` | Low: rolling consumable stock + equipment | **Low:** vacuums, buffers/auto-scrubbers, vehicles; no storefront |

---

## 3. Operating model & labor

**Dominant labor model = licensed owner-operator + W-2 field crew(s).** `[ESTIMATE]` Most independents here run 2–12 field staff dispatched from trucks (low confidence; method: typical single-license regional trade-contractor staffing). There is **no booth/chair-rental model** in this sector (that is a personal-care pattern); labor here is employed crews under a license-holder. Distinct variants:

- **Licensed-pro gated trades (electrical, HVAC, plumbing):** the business is legally bound to a CSLB license holder / Responsible Managing Officer; journeyman/apprentice crews work under that license. Skilled labor is a **hard supply constraint**, not just a cost line. AEI (LIC# 885461) and Frayer (C-10 #1101008) carry posted license numbers; Saunders (1948), Johnson (1960) are multi-generation family licenses — long-tenured, hard-to-replicate operations. `[FACT]`
- **Recurring-contract crews (janitorial, landscape maintenance):** ASAP, Barbara's, Ultimate (janitorial) and the maintenance side of the 7 landscapers run **route-based recurring labor** — the core "production" is crew-hours on standing accounts (offices, ag facilities, HOAs). This is the most predictable revenue but the thinnest COGS and most labor-percentage-sensitive. `[ESTIMATE]` (med confidence)
- **Project/install crews (flooring, landscape install, water-heater/panel/system swaps):** material-plus-labor jobs with higher COGS and longer durations; Capote (landscape + concrete) and the flooring showrooms sit here.
- **Mobile dispatch + 24/7 emergency premium (plumbing/HVAC):** PRO Plumbers and Turlock Plumbing Co advertise 24/7 emergency across Stanislaus County `[FACT]` — after-hours/emergency service-and-repair carries the highest margins in the sector (see §5).

**Seasonality / daypart:**
- **HVAC:** strongly seasonal — **summer cooling peak (Central Valley 100°F+) and shoulder heating** drive emergency repair surge; off-season is maintenance/install. `[FACT]`
- **Landscaping:** **spring–fall install + mow season**, winter slows to cleanup/dormant pruning; tied to housing stock and ag facility grounds. `[FACT]`
- **Plumbing/Electrical:** less seasonal, demand tied to housing stock age, remodels, and emergencies (slab leaks, panel upgrades, EV chargers). `[ESTIMATE]` (med confidence)
- **Janitorial:** non-seasonal recurring nights/weekends (after-hours commercial cleaning); demand tied to commercial occupancy and ag-facility sanitation. `[FACT]`-style.
- **Flooring:** tied to remodel/real-estate cycles and Q4/spring home-improvement.

---

## 4. Regulatory / licensing inputs that gate operations

These are the **operational gates** — a business cannot legally bid covered work without them. All `[FACT]` from CSLB / CA statute:

- **CSLB license** (Contractors State License Board) required for any project where labor+materials ≥ **$1,000** (the "$1k rule"). Relevant Class-C specialty classifications for this sector:
  - **C-10 Electrical** (AEI, Frayer)
  - **C-20 Warm-Air Heating, Ventilating & Air-Conditioning** (Horizon, Saunders, HVAC side of DeHart/Turlock Plumbing H&A)
  - **C-36 Plumbing** (Johnson, Mainline, PRO, Turlock Plumbing Co, DeHart)
  - **C-27 Landscaping** (the 7 landscapers; note Capote's concrete may invoke **C-8 Concrete**)
  - **C-15 Flooring & Floor Covering** (Arrow, Turlock Floor Covering)
  - Janitorial/cleaning is **generally not CSLB-gated** unless doing covered construction work; it operates on a city **business license** + liability/janitorial bond instead.
- **$25,000 contractor bond** on file with CSLB for all licensees; **LLCs carry an additional $100,000 worker bond.** `[FACT]`
- **Workers' compensation:** required for any contractor with employees. **Critical for this sector — C-20 HVAC cannot claim a WC exemption regardless of employee status** (alongside C-8, C-22, C-39, C-61/D-49). Broader SB 216 push toward WC-for-all-licensees was deferred from 1/1/2026 to **1/1/2028** by SB 1455. `[FACT]`
- **General liability insurance** (CSLB-recommended; often required by GCs/commercial clients/HOAs).
- **EPA Section 608 refrigerant certification** for HVAC techs handling refrigerant; refrigerant-transition/A2L rules affect equipment purchasing. `[FACT]` (HVAC-specific)
- **Permits & inspection:** electrical panel upgrades, water heaters, repipes, HVAC changeouts, and irrigation/backflow typically require **City of Turlock / Stanislaus County building permits**; backflow-prevention testing is a recurring landscape/plumbing gate. `[ESTIMATE]` (high confidence — standard CA municipal practice)
- **Cal/OSHA, DOT** (for larger service vehicles), and chemical/SDS compliance (janitorial, landscape pesticide — **CA DPR / county Ag Commissioner license** for chemical applicators; Diaz & Flores' "minimal-chemical" approach reduces this exposure). `[FACT]`

---

## 5. Cost structure (labeled estimates)

All figures below are **labeled benchmark estimates** — industry medians applied to the sub-category, **not** measured figures for any named Turlock business. No dollar revenue is asserted for any named business.

| Sub-category | Gross margin (est.) | Labor as % of revenue (est.) | COGS/material exposure | Net margin (est.) | Method + source + confidence |
|---|---|---|---|---|---|
| **Plumbing / Electrical service & repair** | **35–55% gross** | ~25–35% | Low–med (fittings/devices) | **10–20% net** | Estimate. Method: apply specialty-trade service/repair benchmarks. Source: ServiceTitan / Projul / EdgeStrat contractor-margin guides 2026. **Med confidence.** |
| **HVAC** | **20–35% gross** (install) / higher on repair | ~25–35% | **High** (condensers/furnaces) | **5–12% net** | Estimate. Method: specialty-trade install vs. service split. Source: Projul / Siana 2026 benchmarks. **Med confidence.** |
| **Flooring (showroom + install)** | **25–40% gross** | ~15–25% (install crews) | **High** (product is special-order) | **5–12% net** | Estimate. Method: flooring-retail-plus-install margin norm. Source: contractor-margin-by-trade guides 2026. **Low–med confidence.** |
| **Landscaping — maintenance** | **45–55% gross** | **40–50%** (labor-dominant) | Low (2–6% material) | **10–20% net** | Estimate. Method: lawn/landscape maintenance benchmarks. Source: Aspire / TurfBooks / RealGreen 2026. **Med confidence.** |
| **Landscaping — install/hardscape** | **35–50% gross** | ~15–25% | **20–30% material** | **10–15% net** | Estimate. Same sources; install carries higher material exposure. **Med confidence.** |
| **Janitorial/Cleaning** | **40–55% gross** (commercial) | **50–60%** (labor is the product) | **Very low** (chemicals/paper) | **10–25% net** | Estimate. Method: commercial-cleaning labor-line benchmarks. Source: TheJanitorialStore / Housecall Pro / Jobber 2026. **Med confidence.** |

**Cross-cutting cost drivers (all sub-categories):**
- **Labor burden** adds **~25–40% on top of base wages** (employer FICA 7.65%, FUTA/SUTA, workers' comp, benefits, PTO). `[ESTIMATE]` (Source: contractor labor-burden benchmarks 2026, high confidence). **Workers' comp rates are higher for roofing/HVAC-class risk** and are a mandatory, non-trivial line for C-20.
- **No-rent / mobile capex profile:** these are **truck-and-tool businesses, not storefront-rent businesses** (flooring showrooms are the exception). Capex is **vans + specialized tools** (drain cameras, recovery machines, auto-scrubbers, skid steers) rather than retail lease. `[FACT]`
- **Insurance + bond + license** are fixed annual gates: $25k bond premium, GL premium, WC (rate-driven), and CSLB renewal (~$450 renewal/license cycle range). `[ESTIMATE]` (low–med confidence; exact premiums vary by carrier/risk).
- **Fuel + vehicle maintenance** is a meaningful variable cost for mobile dispatch and route-based crews. `[ESTIMATE]`
- **Emergency / after-hours premium** (24/7 plumbing/HVAC) is the sector's highest-margin work — repair service margins (35–55% gross) materially exceed install margins. `[ESTIMATE]` (med confidence).

---

## 6. Owner-relevant notes (cleaning & landscaping flagged in decision memo)

- **Landscaping is the most-crowded sub-category (7 named independents)** — Capote, Diaz & Flores, JNG, Lopez (30+ yr), Turlock Landscaping, Valley Nursery Landscape, Visionary. Supply chain is **commoditized and shared** (SiteOne/Ewing/Horizon serve them all), so competitive edge is **labor efficiency, recurring-maintenance route density, and differentiation** (Diaz & Flores' minimal-chemical/whole-systems angle; Capote's concrete cross-sell; Visionary's tree care). Maintenance-route revenue is recurring but **labor-percentage-sensitive (40–50% est.)** — the operational risk is labor cost/availability, not material supply. `[ESTIMATE]`
- **Janitorial (3 named: ASAP, Barbara's, Ultimate)** is the **lowest-capex, lowest-COGS, highest-recurring** entry point — no CSLB gate, thin supply chain (WAXIE/Imperial Dade), and labor *is* the product (50–60% of revenue est.). Barriers to entry are low (favors new entrants) but so is differentiation; the moat is **contract retention with commercial/ag-facility accounts**, not supply or capex. `[ESTIMATE]`
- **The licensed mechanical trades (electrical, HVAC, plumbing)** have the **strongest moats** — CSLB licensing, skilled-labor scarcity, bond/WC gates, and multi-generation incumbents (Saunders 1948, Johnson 1960) raise the entry bar well above cleaning/landscaping. `[FACT]`/`[ESTIMATE]`

---

## Sources

- CSLB / California contractor licensing, bond, WC (SB 216 / SB 1455): cslb.ca.gov; thecontractormatrix.com; eclipseinsurance.com; glacierpointinsurance.com
- Electrical distribution: graybar.com; ewweb.com (Top-100); servicetitan.com
- HVAC distribution: ferguson.com; watsco.com (Carrier Enterprise); johnstonesupply.com; lennoxpros.com; achrnews.com
- Plumbing distribution: hajoca.com; ferguson.com; pacesupply.com
- Flooring: shawfloors.com; mohawkflooring.com; manningtonfloors; regional flooring distributors
- Landscape/irrigation: siteone.com; ewing (ewingoutdoorsupply); horizononline.com
- Janitorial: waxie.com; imperialdade.com; cleansource
- Cost/margin benchmarks: servicetitan.com; projul.com; edgestratfinance.com; sianamarketing.com (trades); youraspire.com; turfbooks.com; realgreen.com (landscape); thejanitorialstore.com; housecallpro.com; getjobber.com (cleaning)

*Per-business revenue/margin is not public; all economics above are labeled industry-benchmark estimates, not measured figures for any named Turlock business.*
