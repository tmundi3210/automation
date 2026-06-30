# T3 — Knitting-Machine Landscape & Economics (Analyst A)

**Role:** machine-landscape & economics analyst. Subject = *the machines that make sweaters*, what they cost, where they come from, and what a micro-brand should actually buy / rent / subcontract.

**For:** the same small AI-designed D2C woollen/acrylic knitwear brand (Ludhiana sourcing → California/Turlock retail). This connects directly to the Ludhiana dive's flagged scarcity: **CAD/flat-knit programmers** as the skill that turns a design into a machine program (§6).

**Honesty tags (binding, carried from SCOPE_GLOSSARY):** **[FACT]** cited public source · **[ESTIMATE]** method+basis+confidence, literal word *estimate* · **[UNKNOWN]** valid, frequent answer. Price/MOQ/lead-time bands are almost always **[ESTIMATE]** — never a fake-precise number as fact. Coverage is a floor.

> **The one sentence to remember.** *For a tiny D2C capsule brand, the knitting machine is almost never the thing to buy — the binding constraint is the CAD/flat-knit programmer and the linker, not the iron; so the default posture is subcontract the knitting/linking to the Ludhiana agglomeration and rent the scarce human skill, buying a machine only if/when one style runs at volume.* `[ESTIMATE: judgment, HIGH — follows directly from T1 §3 agglomeration + §6 programmer/linker bottleneck]`

---

## 1. Machine families for sweaters — what each is for, and the gauge ranges

A sweater is made of (a) **knitting** the fabric/panels, (b) **joining** them (linking/seaming), and (c) **finishing**. Different machine families own each step. Use the §3 glossary terms exactly (flat-bed/V-bed, WHOLEGARMENT/knit&wear, circular, linking, GG/E-gauge).

### 1a. Computerised flat-bed (V-bed) — the core sweater machine
- **What it is:** two opposed needle beds knit flat panels back-and-forth; computer-controlled needle selection, carriage (cam) systems, yarn carriers, stitch transfer and **racking** for ribs, cables, intarsia, jacquard, and **fully-fashioned (FF)** shaping (knit-to-shape panels with fashioning marks). The core technology the whole chain centres on per glossary §3. `[FACT — definitional; Shima Seiki / Stoll product pages]`
- **What it makes:** the front/back/sleeve panels of a classic FF sweater; rib trims; collars; can also knit "blanks" for cut-&-sew.
- **Systems/feeders:** machines are described by number of **knitting systems (cam systems)** — 1, 2, 3, 4 — more systems = more rows knit per carriage pass = higher throughput (and higher price). A "3-system" machine is a common commercial workhorse. `[FACT — vendor/marketplace spec convention]`
- **Gauge range:** typically **E3 (≈3GG) up to E18 (18GG)** across the product family; coarse 3–5GG for chunky knits, **7GG the commercial pullover default**, 12GG the fine "elevated basics" tier for warm California, 14–18GG near-jersey fine. Multi-gauge / needle-out (3.5.7) lets one bed fake coarser gauges. `[FACT — Stoll CMS 503 ki L quoted E5–E18 / E2.5.2–8.2; glossary §3 GG→product map]`

### 1b. WHOLEGARMENT® / knit&wear® — integral seamless machines
- **What it is:** a flat-bed evolution (4-bed or 2-bed-plus-loop-transfer) that knits the **entire garment in one piece** — body + sleeves + collar — coming off the machine essentially finished, **no panels, no linking**. Shima Seiki trademark = **WHOLEGARMENT®**; Stoll's = **knit&wear®**. `[FACT — Shima Seiki / Stoll product pages]`
- **Why it matters / why it's hard:** near-zero cut waste and **no seaming labour** (kills the linker bottleneck) — but machines are **expensive, slower per garment**, and the **programming is much harder** (a full 3D garment program vs a flat panel). A strategic make-decision and an AI/automation frontier. `[ESTIMATE: industry-lore, HIGH — consistent across vendor messaging + glossary §3]`
- **Gauge / models:** Shima all-needle WHOLEGARMENT typically **8–16GG**, with "half-gauge" technique to widen the effective range and gauge-less knitting on the **MACH2®S** flagship (multiple gauges in one garment). The **SWG-N2** series is the compact/entry WHOLEGARMENT class. Used-market listings show e.g. **M153xs 15GG (2019)** and **M153x 18GG (2010)**. `[FACT — Shima Seiki SWG-N2 / MACH2S pages; made-in-china used listings]`

### 1c. Hand-flat & power-flat — the legacy/entry tier
- **Hand-flat (hand-frame / domestic flat):** manual carriage, operator-driven; cheap, slow, for **sampling, artisanal small runs, and prototyping**. Glossary §3 "hand-frame / domestic flat." `[FACT — glossary]`
- **Power-flat (semi-automatic):** motorised carriage, mechanical/cam pattern control, limited automatic shaping; the workhorse of older/low-cost Ludhiana shops before computerised flats. **Far cheaper than computerised flat, but limited patterning and shaping, more operator skill per piece, lower repeatability.** `[ESTIMATE: industry-lore, MED-HIGH]`
- **Relevance to us:** T1 flagged that **operators of older hand-/power-flat machines are plentiful while CAD/computerised-flat programmers are scarce** — so the cheap-machine tier exists but does *not* serve an AI-design pipeline that needs programmable, repeatable, fine-gauge fashion knits. `[FACT — T1 DEEPDIVE §4/§6]`

### 1d. Circular — jersey bodies for cut-&-sew (mostly adjacent to sweaters)
- **What it is:** a rotating cylinder of needles knits a continuous **jersey tube** at high speed for later **cut-&-sew (C&S)**. High throughput, low per-metre cost; **no fashioning → more cut waste, more casual product** (tees, sweatshirts, fine jersey "sweater-knit" tops). Glossary §3. `[FACT — glossary]`
- **Relevance to us:** only if the line includes jersey/sweatshirt styles; a *classic FF sweater* is **not** a circular product. Worth knowing so it isn't confused with flat/weft knitting and so the C&S-vs-FF cost counterpoint is correct in T4. `[FACT — glossary §3 construction]`

### 1e. Linking machines — the quality seam (the real bottleneck)
- **What it is:** a rotary **point-dial** machine (a circular crown of points) that joins two knitted edges **loop-to-loop**, producing the flat, near-invisible, stretchy, stitch-aligned seam that defines FF sweaters ("fully-linked" / "hand-linked" = a quality claim). The operator hangs each loop on a point by eye → eyesight-intensive, slow, piece-rate, **throughput-limiting**. Glossary §3 calls it a "real cost + capacity bottleneck." `[FACT — glossary §3; T1 §4 names linking as *the* skill bottleneck]`
- **Gauge match:** the linker's **points-per-inch (PPI)** must roughly match the knit gauge (e.g. a ~7PPI linker for ~7GG knitwear; finer knits need finer-point linkers). Hand linkers ~7PPI handle up to ~4-ply yarn. `[FACT — Hague linker specs; standard practice]`
- **Brands/tiers:** hand/electric rotary linkers (e.g. **Hague** in the UK; numerous Italian and Chinese makers) at the low end; automatic/computerised linkers and **"automatic linking + Italian (Complett/Exacta-class) overlock-linkers"** at the high end. *(Specific Complett/Exacta model/price = [UNKNOWN] — not found in public listings.)* `[FACT brand existence; ESTIMATE tiering, MED]`
- **Mock-linking vs true linking:** glossary §3 distinguishes true loop-to-loop linking from **mock-linking** (imitates the look, faster/cheaper, slightly less clean) and from **overlock/cup-seam/coverstitch** (sewn seams that signal C&S/value construction). A QC + cost-down distinction T5 must police.

### 1f. Finishing machines (post-knit — full depth in T6)
Steaming/pressing/blocking units (set dimensions), **boarding** (sweater shape-setting), washing/scouring/softening lines (anti-pill, silicone hand), and inspection tables. Mostly outside T3's machine economics; flagged here so the chain is complete. `[FACT — glossary §4]`

**Family → role summary**

| Family | Makes | Gauge band | Linking needed? | Capital tier |
|---|---|---|---|---|
| Computerised flat-bed (V-bed) | FF panels, ribs, jacquard/intarsia, blanks | E3–E18 | **Yes** (separate step) | High |
| WHOLEGARMENT / knit&wear | whole seamless garment | ~8–16GG (+half-gauge) | **No** (integral) | Highest |
| Power-flat (semi-auto) | simpler panels, low-cost | ~3–12GG | Yes | Low-Med |
| Hand-flat / domestic | samples, artisanal | varies | Yes | Lowest |
| Circular | jersey tube for C&S | (cut-&-sew) | No (sewn) | Med (volume) |
| Linking | the loop-to-loop seam | match knit PPI | — | Low-Med |

---

## 2. Where the machines are made — and by whom

- **Japan — Shima Seiki** (Wakayama). The global standard for computerised flat + the originator of **WHOLEGARMENT®**. Full vertical: machines + **SDS-ONE APEX design system (CAD/CAM)** + the programming software. Has an **India subsidiary/network** (Shima Seiki India) and dealer presence in Ludhiana/Tirupur. `[FACT — Shima Seiki company/network pages; Ludhiana dealer listings e.g. Kniton Traders, King International]`
- **Germany — Stoll**, now **STOLL by KARL MAYER** (KARL MAYER acquired Stoll in 2020). The other historic computerised-flat leader (CMS / ADF lines; **knit&wear®**). **MAJOR STRUCTURAL CHANGE [FACT]:** KARL MAYER decided in early 2025 to refocus on warp knitting/technical textiles and **discontinue the STOLL flat-knitting machine business** — the **Reutlingen (Germany) production site closed 31 Oct 2025** and **China production ended Dec 2025**; ~280 Reutlingen jobs lost, no investor found, **spare-parts/service continuing via Obertshausen central warehouse.** This means **new Stoll flat machines are effectively end-of-line; the installed base becomes a used/service market.** `[FACT — karlmayer.com; knittingindustry.com; Textile World; wtin.com — multiple independent sources, Oct 2025]`
- **China — clones / domestic brands (the volume tier).** A large ecosystem of computerised-flat makers: **Cixing (Ningbo Cixing)** — the dominant Chinese brand, which also **acquired the Swiss Steiger** flat-knit brand; plus **Changhua, Flying Tiger, Giant Star, Longxing/Lonati-class, Jinlong, Tongda, Fengfan, Lianxing, Beworth, Qihangxing** and many more. These target lower price points and now cover most gauges incl. WHOLEGARMENT-style "integral" machines. `[FACT — made-in-china / Alibaba / industry listings; Cixing–Steiger ownership widely reported]`
- **Italy/Switzerland/others (niche):** **Steiger** (Swiss, now Cixing-owned), Italian **Complett** (linkers), and various linking/finishing specialists. `[FACT — brand existence]`
- **India's role:** **primarily an importer/assembler/dealer market, not a builder of computerised flat machines.** Ludhiana hosts dealers/importers (e.g. Kniton Traders est. 2008, King International) of Shima/Stoll/Cixing machines plus parts and pattern-maker/repair services; India makes some **accessory and lower-tech machinery and parts**, and **circular/hosiery machinery** exists domestically, but the high-end computerised flat + WHOLEGARMENT machines are **imported (Japan/China/EU-used).** `[FACT — Ludhiana dealer listings; ESTIMATE on "primarily importer," HIGH — no major Indian computerised-flat OEM found]`

**Strategic read:** with **Stoll's new-build business closing (2025)**, the realistic new-machine choices narrow to **Shima Seiki (premium/standard, with India support)** and **Chinese brands led by Cixing (cost tier)** — and the **used market deepens** (lots of ex-Stoll iron will circulate, supported only by spare-parts service). For a micro-brand this *reinforces* the buy-used-or-subcontract logic. `[ESTIMATE: judgment, MED-HIGH]`

---

## 3. Cost reality — new vs used, throughput, line sizing, import

### 3a. Price bands (all [ESTIMATE] unless tagged — vendors don't publish list prices)
**Method:** triangulating (i) public used-marketplace listing ranges (Machinio/Wotol/Exapro/made-in-china), (ii) Indian B2B listings (IndiaMART/TradeIndia show Shima units quoted around **₹2 lakh ≈ US$2.4k for old/basic and rising steeply for newer**), (iii) industry-lore on new computerised-flat pricing. **Confidence MED on bands, LOW on any single point. Never quote as [FACT].**

| Machine class | New (per machine) | Used / reconditioned | Basis |
|---|---|---|---|
| **Hand-flat / domestic flat** | ~$500–3,000 | ~$200–1,500 | low-tech, widely traded `[ESTIMATE, MED]` |
| **Power-flat (semi-auto)** | ~$3,000–10,000 | ~$1,000–5,000 | China new-build common `[ESTIMATE, MED]` |
| **Chinese computerised flat (Cixing-class, 3-system)** | ~$8,000–25,000 | ~$4,000–12,000 | made-in-china new listings ~$7k–9.5k for entry; higher for wide/fine `[ESTIMATE, MED — anchored to made-in-china listings $7,000–9,500]` |
| **Shima/Stoll computerised flat (standard FF)** | ~$60,000–140,000 | ~$8,000–45,000 (age/gauge/condition) | premium new; deep used market; Indian listing shows old units ~₹2 lakh `[ESTIMATE, MED-LOW on new; MED on used]` |
| **WHOLEGARMENT / knit&wear (Shima SWG-N2 / MACH2S; Stoll knit&wear)** | ~$120,000–250,000+ | ~$20,000–90,000 | newest, most expensive; thin used market `[ESTIMATE, LOW-MED]` |
| **Linking machine (rotary, hand/electric)** | ~$1,500–8,000 | ~$500–4,000 | Hague-class + Chinese; automatic linkers higher `[ESTIMATE, MED]` |
| **Automatic / Italian linker (Complett/Exacta-class)** | ~$15,000–40,000+ | ~$5,000–20,000 | high-end; specific quotes [UNKNOWN] `[ESTIMATE, LOW]` |
| **Finishing (steam press / boarding / wash) per unit** | ~$2,000–20,000 | ~$1,000–8,000 | wide range by type `[ESTIMATE, LOW-MED]` |

> **Why new Shima/Stoll prices are [ESTIMATE, LOW-MED]:** vendors quote on configuration (gauge, working width, # systems, options) and **do not publish list prices**; the only public "prices" are used-marketplace and Indian-listing figures, which are non-comparable across condition. The **$60k–140k new band** is industry-lore-level, not a quote. **The precise new price is [UNKNOWN].**

### 3b. Throughput (panels / garments per shift) — order-of-magnitude only
**Method:** industry-lore + arithmetic; depends massively on gauge, garment size, stitch complexity (cables/intarsia slow it), # of knitting systems, and machine speed. **Confidence LOW-MED; treat as planning bands, harden in T4 against a real unit.**
- **Computerised flat, FF panels:** a mid-gauge (7–12GG) machine knits roughly **one adult sweater's set of panels in ~20–45 min** → ~**12–25 garments' worth of panels per machine per 8-hr shift** (fewer for heavy cables/intarsia/fine gauge, more for simple light knits). `[ESTIMATE: industry-lore, LOW-MED]`
- **WHOLEGARMENT:** **slower per garment** (whole 3D piece) — order **~6–15 garments/machine/shift** — but **eliminates the entire linking step**, so the *system* throughput comparison must include linking labour. `[ESTIMATE: industry-lore, LOW-MED]`
- **Linking (the bottleneck):** a skilled linker seams roughly **one sweater's joins in ~10–25 min** → ~**20–45 sweaters/linker/shift** depending on gauge/complexity — which is **why linking, not knitting, usually sets a small line's daily capacity** and why WHOLEGARMENT's "no-link" property is strategically large. `[ESTIMATE: industry-lore, LOW-MED; consistent with T1 §4 "linking is the bottleneck"]`
- **Power-flat / hand-flat:** materially lower and more operator-variable. `[ESTIMATE, LOW]`

### 3c. How many machines a tiny line needs
**Method:** arithmetic from §3b bands for a hypothetical micro-line targeting **~30–60 finished FF sweaters/day** (a small-capsule scale). **[ESTIMATE: design illustration, LOW-MED.]**
- **Knitting:** ~**2–4 computerised flat machines** (3-system, mixed 7GG + 12GG) to cover panels at that rate with style variety. `[ESTIMATE]`
- **Linking:** ~**2–4 linkers + linkers-operators** — linking, not knitting, is what you must staff up to match output. `[ESTIMATE]`
- **Finishing:** ~**1 steam-press/boarding station + 1 inspection table** (plus outsourced wash/dye). `[ESTIMATE]`
- **CAD/programming:** **≥1 flat-knit programmer** (the scarce skill — see §4); often **shared/contracted** rather than full-time at micro scale.
- **Reality check:** even this "tiny" owned line is a **mid-six-figure capital + skilled-staffing commitment** before a single garment sells — which is exactly why subcontracting dominates at capsule scale (§5). `[ESTIMATE: judgment, MED-HIGH]`

### 3d. Import / lead-time / duty (into India, for the machine itself)
- **Indian import classification:** knitting machines = **HS/HSN heading 8447**. **GST/IGST 18%** on import. `[FACT — CBIC/HSN 8447; cleartax/vakilsearch]`
- **Basic Customs Duty (BCD):** textile machinery under 8447 is generally in the **~7.5% BCD** band (plus ~10% Social Welfare Surcharge on the BCD ≈ +0.75%, plus 18% IGST on assessable value+BCD). **Exact line-item BCD can vary by sub-heading and by current-year budget notifications → [ESTIMATE, MED; confirm exact rate at purchase].** `[ESTIMATE — multiple India customs-duty aggregators cite ~7.5% BCD for 8447; precise sub-heading rate UNKNOWN]`
- **Lead time:** new Shima/Cixing machines are typically **made-to-order / configured → weeks-to-months to build + ship (ocean from Japan/China)**; used machines ship faster but need reconditioning/setup. **[ESTIMATE: industry-lore, LOW-MED; specific lead times UNKNOWN.]**
- **Hidden costs:** install, power/compressed-air infrastructure, spares, and **the now-critical service question for ex-Stoll machines** (parts via Obertshausen only) — factor service availability into any used-Stoll purchase. `[FACT — Stoll service-via-Obertshausen; ESTIMATE on cost impact]`

---

## 4. The people side — CAD/flat-knit programmers (the scarce skill)

This is the explicit bridge from the Ludhiana dive (T1 §4, §6, Open Q#8): *"CAD/programmers for computerised flat-bed machines (Shima/Stoll-class) are scarce relative to operators of older hand-/power-flat machines… our design pipeline assumes someone can translate a design into a machine program."* `[FACT — T1 DEEPDIVE]`

- **What the role is:** a flat-knit programmer takes a design/tech-pack and authors a **machine program** on the vendor CAD/CAM (**Shima SDS-ONE APEX**; Stoll **M1plus / knitelligence**) — defining every needle action, stitch structure, shaping (FF increase/decrease), yarn-carrier moves, racking, take-down, and (for WHOLEGARMENT) the full 3D integral build. It is **the actual translation from "a design" to "a knitted object,"** and it is **machine-specific** (a Shima program ≠ a Stoll program). `[FACT — vendor CAD systems; definitional]`
- **Why it's scarce and decisive for us:** the AI-design pipeline (T8) outputs images/specs; **someone still has to program the machine.** WHOLEGARMENT raises the programming bar sharply. So the programmer is a **harder constraint than the machine** for a micro-brand — and T1 flags real availability of programmers vs operators as **[ESTIMATE, MED] / Open Q#8 (on-the-ground confirmation needed).** `[FACT — T1; ESTIMATE on scarcity magnitude]`
- **Implication for the AI angle:** the AI frontier here is **AI-assisted/auto-generation of the knit program** (image/spec → machine code), which would relax exactly this bottleneck — flagged as a frontier, **feasibility/maturity [UNKNOWN] in T3, revisited in T8.** `[ESTIMATE: judgment, MED]`

---

## 5. What a micro-brand actually buys vs rents vs subcontracts

**Default = subcontract, because the agglomeration is the infrastructure (T1 §3 moat #5 — the one moat that helps us).** `[FACT — T1 DEEPDIVE §3/§6]`

| Step | Buy? | Rent / contract-skill? | Subcontract (job-work)? | Recommendation |
|---|---|---|---|---|
| **Knitting (panels)** | only at proven style volume | rent machine-time at a job-knitter | **default** — pay per-panel/per-piece to Ludhiana knit shops | **Subcontract** at capsule scale `[ESTIMATE, HIGH]` |
| **CAD/programming** | — | **rent/contract a programmer** (the scarce skill) | bundled into the knitter's service | **Contract the skill**; verify availability (T1 Open Q#8) `[ESTIMATE, HIGH]` |
| **Linking** | only at volume | — | **default** — the cluster's linkers | **Subcontract** (it's the bottleneck — don't own it early) `[ESTIMATE, HIGH]` |
| **Finishing/wash** | no | — | **subcontract** (and *never own a dyehouse* — T1 §5b NGT/CETP risk) | **Subcontract** `[FACT-anchored — T1]` |
| **Sampling/proto** | maybe **1 hand-flat** | — | or use the subcontractor's | **Optional cheap buy** for fast in-house iteration `[ESTIMATE, MED]` |

- **Buy** makes sense only when a **single style runs at sustained volume** (utilisation justifies the machine + programmer + linker payroll) — which a deliberately tiny, iterating capsule brand (glossary §8) usually won't hit early. `[ESTIMATE: judgment, MED-HIGH]`
- **Rent/contract the scarce human (programmer)** rather than the iron — the program is the value, the machine is a commodity-ish input you can rent via job-work.
- **Subcontract** knitting + linking + finishing into the walking-distance mesh — exactly the small-batch/iteration fit T1 §6 identified as Ludhiana's genuine edge for *our* use case.
- **The make-vs-buy boundary** (glossary cross-rule #9) for T3 therefore lands at: **own design + brand + (optionally) a sampling hand-flat; rent the programmer; buy CMT/job-work for knit+link+finish.** This is the machine-side expression of the venture's recurring central lever. `[ESTIMATE: design-framing, HIGH it's the right default for a micro-brand]`

---

## 6. Open questions (floor, not ceiling — for T3 reconcile / T4 / T8)
1. **Exact new-machine list prices (Shima, Cixing) by config — [UNKNOWN];** vendors quote on configuration. Bands here are [ESTIMATE]. *(Vendor quote needed.)*
2. **Real used-Stoll service/parts cost & availability post-Reutlingen-closure — [UNKNOWN];** only "service via Obertshausen" is [FACT]. Affects whether ex-Stoll iron is a smart used buy. *(2026 on-the-ground.)*
3. **Actual per-shift throughput on a *specific* Ludhiana machine/garment — [UNKNOWN];** §3b are order-of-magnitude bands. *(Floor unit data → T4.)*
4. **Job-work rates for knitting & linking per piece — [UNKNOWN];** the machine-side analogue of T1's #1 missing number (the sweater piece-rate). Decides buy-vs-subcontract economics. *(On-the-ground → T4.)*
5. **Local availability of Shima/Stoll-class programmers vs operators — [ESTIMATE, MED] / T1 Open Q#8;** decides whether the AI→program pipeline is staffable. *(On-the-ground → T8.)*
6. **Maturity of AI-assisted knit-program generation (image/spec → machine code) — [UNKNOWN];** the frontier that would relax the programmer bottleneck. *(→ T8.)*
7. **Exact India BCD sub-heading rate for the specific machine + any 2025–26 budget changes — [ESTIMATE, MED];** ~7.5% BCD + 18% IGST is the working assumption. *(Confirm at purchase.)*

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/machines/analyst_a_landscape.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `SCOPE_GLOSSARY.md`; `ludhiana/DEEPDIVE.md`
- **Delivered:** (1) machine families for sweaters — computerised flat-bed/V-bed, WHOLEGARMENT/knit&wear, hand-flat/power-flat, circular, **linking (the quality seam + bottleneck)**, finishing — each with role + gauge band (E3–E18; 7GG default, 12GG fine-CA, WG 8–16GG); (2) **where made** — Japan/Shima (standard + WHOLEGARMENT, India support), Germany/Stoll **[FACT: flat-knit business discontinued, Reutlingen closed 31 Oct 2025, China prod ended Dec 2025 — new-build effectively end-of-line]**, China/Cixing-led cost tier (+ Steiger acquired), India = importer/dealer not builder; (3) **cost reality** — new-vs-used price bands per class [ESTIMATE, MED/LOW with method], throughput bands (linking sets capacity), tiny-line sizing (~2–4 knitters + 2–4 linkers + finishing for ~30–60 sweaters/day = mid-six-figure commitment), India import (HSN 8447, ~7.5% BCD + 18% IGST); (4) **people side** — CAD/flat-knit programmer (Shima SDS-ONE APEX / Stoll M1plus) as *the* scarce skill that turns design→machine program, the bridge from T1 §4/§6/Q#8; plus the **buy-vs-rent-vs-subcontract** table landing on *subcontract knit+link+finish, rent the programmer, own only design+brand+optional sampling hand-flat.*
- **Honesty discipline:** every price/throughput/lead-time tagged [ESTIMATE] with method+confidence (never fake-precise [FACT]); Stoll closure, HSN 8447, Cixing–Steiger, gauge bands, Reutlingen dates carried as [FACT] with sources; programmer-scarcity carried as [ESTIMATE]/T1 flag; 7 open questions left explicitly [UNKNOWN] as a floor.
- **Bottom line:** the machine is rarely the micro-brand's binding constraint — the **programmer and the linker** are; default posture = **subcontract into the Ludhiana mesh, rent the scarce CAD skill, buy iron only at proven style volume**, while noting the 2025 Stoll exit narrows new-build to **Shima + Chinese (Cixing) and deepens the used market.**
