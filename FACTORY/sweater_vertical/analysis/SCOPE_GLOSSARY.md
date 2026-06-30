# SCOPE_GLOSSARY.md — Sweater Vertical Shared Vocabulary

**Read this first.** This is the single shared vocabulary for the entire sweater vertical. Every specialist (T1 Ludhiana → T11 brain) must use these terms identically. Built from `_t0_reconciled.md` (the master merge of the technical + business vocab); that file is the working reference, this is the clean human artifact.

**Honesty tags (binding, travel downstream with every claim).**
- **[FACT]** — cited public source; treat as a citeable anchor.
- **[ESTIMATE]** — carries method + basis + confidence, uses the literal word "estimate"; treat as a hypothesis to harden or refute. Variants: **[ESTIMATE: industry-lore, <confidence>]** = trade-craft convention; **[ESTIMATE: discussion/industry-lore]** = off-paper mechanism described **structurally, never as a named-party accusation**.
- **[UNKNOWN]** — not reliably known; a valid, frequent answer. Coverage is a **floor, not a ceiling**.
- **Untagged** — dimensional identities (unit definitions, count conversions like `Tex = g/1000 m`) are true by definition, carry no tag.

---

## Scope statement

This venture builds the **knowledge brain** (not yet the operating business) for a vertically-aware knitwear company running the full value chain: **yarn sourced in Ludhiana, India** → **flat-bed/circular knitting machines** → **manufacture, quality control, and finishing** → an **AI-designed direct-to-consumer knitwear brand selling in California/Turlock, USA**. The base case is an **acrylic + acrylic-blend** cluster (Ludhiana's reality) producing mainstream-to-premium sweaters, with the recurring central decision being the **make-vs-buy boundary** — own spinning/knitting vs buy yarn/CMT capacity vs purchase full-package FOB goods — and the recurring viability test being **contribution margin after CAC**, not factory cost. The brain is a set of grounded specialists, each with dense knowledge bases, later wired into one routed decision engine that judges whether the venture is viable, in what configuration, and where the money and the kill-risks actually are.

---

## Section 1 — Fibres

A sweater's hand, warmth, price, pilling, and care label are decided at the fibre line before a stitch is knit. Two axes dominate: **staple length** (longer → less pill, stronger, smoother) and **fineness** (micron for animal, denier-per-filament for synthetic → finer = softer).

- **Acrylic** — Synthetic (polyacrylonitrile) staple; cheap, bulky, bright-dyeing, machine-washable, moth-proof, but pills, holds static, weak wet-abraded, melts (the cost floor for mass-market knitwear, and **Ludhiana's base fibre** — our default case). `[ESTIMATE: industry-lore, high confidence]`
- **Wool (generic sheep's wool)** — Natural keratin fibre; crimped, elastic, warm when damp, flame/odour-resistant; coarse grades (>27 micron) prickle (the warmth benchmark, but **micron** — not the word "wool" — decides next-to-skin comfort).
- **Merino** — Fine wool, ~17–24 micron (sub-19.5 = "fine/superfine"); soft, low-prickle (the premium-but-wearable sweet spot and a price-justifying marketing word).
- **Lambswool** — First-shearing wool (~7 months), softer/finer than later clips; a grade not a species (lets a brand say "soft wool" below merino prices).
- **Cashmere** — Fine goat undercoat, ~14–19 micron; extremely soft/light/warm, low-yield, expensive, **pills more** than wool (the luxury anchor — even 5–10% in a blend lifts perceived value, but needs finishing + a care story).
- **Cotton** — Natural cellulosic staple; breathable, washable, no prickle, but heavy, low recovery (bags out), poor warmth-to-weight (the go-to for the **warm California market** where heavy wool sells fewer months; mercerising upgrades it).
- **Viscose / Modal (regenerated cellulose, "rayon")** — Silky drape, high sheen, deep dye uptake, but weak wet, creases; Modal = stronger/finer 2nd-gen (added for drape and to cut animal-fibre cost; signals a fluid knit).
- **Nylon (polyamide)** — Strong, abrasion-resistant, elastic; rarely the main fibre (a few % in a cashmere/lambswool blend sharply improves pill + abrasion resistance — a common quality lever). `[ESTIMATE: industry-lore, high confidence]`
- **Blends** — Two+ fibres spun together to trade cost/hand/performance (acrylic/wool, wool/nylon, cotton/acrylic, cashmere/nylon, wool/viscose). The blend **ratio** on the care label is the single biggest cost-and-positioning decision — "70/30" vs "50/50" is a margin lever the brand controls.
- **Micron** — Animal-fibre diameter (µm); lower = finer/softer/pricier (the grading axis that, not "merino," sets comfort and price).
- **Staple length** — Length of one fibre; longer → smoother, stronger, lower-pill yarn (a cheap pre-wear pilling predictor for QC).
- **Pilling** — Tangled balls of broken/loose surface fibres; worse with short, fine, low-twist, loosely-knit fibres (the #1 post-purchase complaint and return driver; designed out via fibre, twist, gauge, and anti-pill finish; quantified by the §5 pilling grade).

---

## Section 2 — Yarn metrics (count, ply, twist)

Two opposite count logics: **direct** (mass per fixed length → bigger = thicker: Tex, Decitex, Denier) and **indirect** (length per fixed mass → bigger = finer: Ne, Nm, worsted). This inversion is the #1 cross-team confusion.

- **Ne (English cotton count)** — # of 840-yd hanks per pound; indirect (higher = finer). Dominant for cotton + South-Asian spun yarn; Ludhiana quotes Ne and folded "2/30" notation (the count you'll hear most in the mandi). `[ESTIMATE: industry-lore, high confidence]`
- **Nm (metric count)** — Metres per gram (= km/kg); indirect (higher = finer). Dominant for wool/worsted; European-mill sweater specs are usually Nm.
- **Worsted count (NeK)** — # of 560-yd hanks per pound; wool-system indirect count (legacy British wool specs — convert to Nm to compare).
- **Tex** — Grams per 1000 m; direct (higher = thicker). The neutral **pivot** for all conversion; QC/Uster reports use Tex/Decitex.
- **Decitex (dtex)** — Grams per 10,000 m (= 10×Tex). Standard for fine filament viscose/nylon/polyester in blends.
- **Denier (den)** — Grams per 9000 m; direct (higher = thicker). The filament/synthetic count (≈ 9×Tex).
- **Count conversions (untagged dimensional identities)** — `Tex = 1000/Nm = 590.5/Ne`; `Denier = 9×Tex`; `Decitex = 10×Tex`; **`Nm = 1.693×Ne`** (mnemonic: metric numbers run bigger); `Denier ≈ 5315/Ne`. The exact bridges that stop cross-team count errors (sanity: Ne 30 → Nm ≈ 50.8 → Tex ≈ 19.7 → ≈177 den).
- **Singles (1-ply)** — One strand off the frame; cheaper, weaker, can skew/torque a flat panel (skew is a QC risk; fine fashion knits run singles for lightness).
- **Ply / folded yarn** — Two+ singles twisted together; balances twist, boosts strength/evenness/roundness (most knitwear yarn is 2-ply for durability + clean stitch definition).
- **Folded-count notation (2/30, 2/48 Nm)** — "2/30" = two 30s singles plied; **resultant ≈ singles ÷ plies** (2/30 ≈ 15s-equivalent), and the **ends fed** further multiply effective thickness (the standard lever to hit a target gauge with in-stock count; misreading 2/30 vs 1/30 is a doubling error in fabric weight).
- **Twist (TPI/TPM; S vs Z direction)** — Amount + handedness of twist; more = stronger, leaner, lower-pill but harsher; balanced ply pairs opposing twists (under-twist pills, over-twist feels wiry and biases the fabric).

---

## Section 3 — Knitting & construction

### Gauge
- **GG (gauge)** — Needles per inch on the bed; the master variable for fineness (higher GG → finer, denser, lighter fabric needing finer yarn). Must match yarn count; dictates machine, knit time, and price tier. `[FACT — definition; Wikipedia "Gauge (knitting)"]`
- **E (E-gauge)** — Same concept written "E" (E7 = 7GG), common on European/Stoll machines (spec sheets mix "7GG" and "E7" — identical).
- **Multi-gauge / 3.5.7 (needle-out)** — A finer machine knitting with needles out to fake a coarser gauge (one machine fakes several gauges — affects make-vs-buy + utilisation).
- **GG → product map** — 3GG very coarse/chunky (~700 g–1 kg+); 5GG coarse (~500–800 g); **7GG medium = the commercial default** retail pullover (~350–550 g) `[FACT — mysweaterfactory.com; ritacashmere.com]`; 9–10GG medium-fine (~280–400 g); **12GG fine, the right tier for warm California "elevated basics"** (~200–350 g); 14/16/18GG very fine/near-jersey (~150–280 g, needs fine high-count yarn, slower → costlier). Weights are order-of-magnitude `[ESTIMATE: industry-lore, medium-high confidence]`.

### Machine families
- **Flat-bed (V-bed) knitting** — Two opposed needle beds; knits flat panels back-and-forth, transfers stitches (ribs, shaping). Computerised flat = **Shima Seiki** (Japan), **Stoll** (Germany / KARL MAYER STOLL) — the core sweater + **fully-fashioned** technology our chain centres on.
- **Whole-garment / integral (Shima WholeGarment®, Stoll knit&wear®)** — Knits the **entire seamless garment in one piece**, no panels/linking; near-zero cut waste + no seaming labour, but expensive, slower, steep programming (a strategic make-decision and AI/automation frontier).
- **Circular knitting** — Cylinder knits a continuous **jersey tube** fast for later cut-&-sew; high throughput, low per-metre cost, but no fashioning → more cut waste, more casual product.
- **Warp knitting (tricot/raschel)** — Parallel yarns looped along the length; fast, stable, lace/mesh/technical (mostly outside sweaters — know it so it isn't confused with weft/flat).
- **Hand-frame / domestic flat** — Manual/semi-auto flat machines for small artisanal capacity + sampling (relevant to tiny-line make-vs-buy and prototyping).

### Construction
- **Fully-fashioned (FF)** — Each panel knit to final shape via stitch increase/decrease, then **linked** edge-to-edge; minimal waste, clean shaped edges, better drape/fit (a price justifier).
- **Cut-&-sew (C&S)** — Knit a blank, then cut + sew like wovens; faster/cheaper for fine jersey + volume but **more waste**, curling/fraying edges, more casual (the cost-vs-quality counterpoint to FF in every make decision).
- **Fashioning marks** — Small diagonal stitch marks where a panel was decreased; a visible authenticity cue that a sweater is genuinely FF, not cut.

### Stitch structures
- **Jersey / plain / single knit (stockinette)** — Single-face, smooth "V" face; light, curls at edges, can ladder (the cheap lightweight base; needs edge finishing).
- **Rib (1×1, 2×2)** — Alternating face/back wales → vertical ridges + widthwise stretch/recovery (cuffs/hems/collars; rib quality = fit + recovery, a key QC point).
- **Links-links (purl)** — Alternates knit/purl along the wale → reversible, textured; needs a purl-capable machine (flags a machine-capability requirement).
- **Cable** — Wales crossed via stitch transfer → raised rope twists; classic heavy-knit value but uses more yarn + machine time (a cost driver).
- **Intarsia** — Colour blocks with **no floats** on the back; clean colour-blocking/logos, no reverse waste, but slower/skilled + machine-dependent.
- **Jacquard** — Multi-colour pattern with unused colours **floated/carried** on the back; rich Fair-Isle looks but heavier + back-float management is a quality issue.
- **Pointelle** — Decorative open/eyelet holes by stitch transfer; lacy, lightweight (feminine/spring fine-gauge styling for warm-climate California knits).
- **Tuck & miss (float) stitches** — Held/skipped loops for texture/colour without extra colours; the building blocks of the above (affect width, weight, stretch).
- **Plating** — Two yarns fed so one shows face, one back (soft face/strong back, or elastane-plated for stretch); a cheap way to a luxe face or added stretch.

### Seaming
- **Linking** — Joining edges **loop-to-loop** on a linking machine for a flat, near-invisible, stretchy stitch-aligned seam; the defining quality step of FF sweaters ("fully-linked"/"hand-linked" is a quality claim), but skilled, labour-intensive, **throughput-limiting** — a real cost + capacity bottleneck. `[ESTIMATE: industry-lore, high confidence]`
- **Looping** — Synonym/relative for attaching trims loop-to-loop ("looping" and "linking" used interchangeably by region).
- **Mock-linking** — A seam that **imitates** a linked look without true loop-to-loop registration; faster/cheaper, slightly less clean (a cost-down buyers/QC must distinguish from true linking).
- **Overlock / cup-seam / coverstitch** — Standard sewn seams for C&S/value knits; bulkier, less stretchy than linking (their presence signals C&S/value construction).

---

## Section 4 — Finishing

Post-knit treatments — "after yarn/garment, what reaches best quality." (Full T6 deep-dive expands these.)

- **GSM (grams per square metre)** — Mass of one m² of fabric; the universal weight metric set on the tech pack and checked in QC, pairing with GG + count to define the product (for shaped sweaters also tracked as **grams per garment** at a size). `[ESTIMATE: industry-lore, high confidence]`
- **Stitch density (courses/wales per inch or cm)** — Loops per unit length/width; tighter = heavier, more stable, lower-pill (with GSM, catches saggy or stiff/yarn-hungry knit).
- **Loop/stitch length (yarn per stitch)** — Yarn in one loop; the deepest control of weight, tightness, and yarn cost (the parameter the programmer tunes to hit GSM + yarn cost simultaneously).
- **Milling / fulling** — Controlled wet/heat/agitation that felts and **closes up** a wool knit → fuller, softer, warmer; over-milling shrinks/felts irreversibly (a QC risk).
- **Anti-pill finish** — Chemical/enzyme (e.g. cellulase bio-polishing) or mechanical treatment plus low-pill fibre/twist; attacks the top return driver (any "anti-pill" claim must survive the §5 pilling-grade wash test).
- **Mercerised (mercerisation)** — Caustic-soda treatment of cotton under tension → lustre, strength, dye uptake (upgrades cotton to premium "mercerised cotton" for the warm-market line).
- **Sanforised / compacted (shrink control)** — Mechanical pre-shrinking to a residual-shrinkage limit; keeps the garment in size tolerance after the customer washes it (returns + care-label compliance; woven-origin term applied to knits via compacting/relaxation).
- **Scouring / washing / softening** — Remove spinning oils/dirt, add softeners/silicone for hand (makes a harsh off-machine knit feel like retail product).
- **Steaming / pressing / blocking / Kier-relax** — Heat-set to final dimensions and relax stresses so panels hold shape + measurements (rushed pressing shows as measurement + appearance defects).
- **Dyeing stages** — **Fibre/stock-dyed**, **yarn/package-dyed**, **piece/garment-dyed**; later-stage = faster trend response but more colour-consistency risk (a lead-time vs colour-fastness trade-off and a lot-to-lot shade-match QC point).

---

## Section 5 — QC (yarn evenness, defects, fabric & garment tests)

Yarn is graded against the **USTER® STATISTICS** benchmark percentiles ("25% USTER" = better than 75% of world production); lower irregularity = cleaner knitting, fewer stops/defects.

- **Yarn evenness / irregularity** — Uniformity of yarn mass along its length (optical/capacitive tester); uneven yarn → streaky fabric, knitting stops, defects → scrap + downtime (direct cost).
- **CV% (CVm, Coefficient of Variation of mass)** — Std dev / mean of mass per unit length (%); the headline evenness number, lower = more even (what buyers anchor on when grading lots). `[FACT — textilesbar.com; standard textile metrology]`
- **U% (unevenness %)** — Older mean-deviation measure; **CV% ≈ 1.25 × U%** (normal distribution) — convert legacy U% specs to compare.
- **IPI (Imperfection Index)** — **Thin + thick places + neps per 1000 m** at standard sensitivities; the point-defect/"cleanliness" number alongside CV% (high IPI predicts visible faults + knitting stops). `[FACT — textilelearner.net; iejrd.com]`
- **Thin place** — Cross-section ≥ ~50% below mean (−50%); a weak point that breaks in knitting, shows as a light streak.
- **Thick place** — ≥ ~50% above mean (+50%); shows as a heavy/dark fault.
- **Nep** — Tight tangled fibre knot ≥ ~200% of mean (+200%); visible specks, worse with short/immature fibre (a common reject cause).
- **Hairiness (H / S3)** — Protruding fibre ends on the yarn surface; affects pilling, hand, shed/lint (too high → pilling + dusty knitting; too low → flat/harsh look).
- **Tenacity / elongation** — Breaking force (cN/tex) + stretch at break; low strength → breaks in knitting (downtime) + weak seams (a core acceptance test).
- **Count CV / count variation** — Lot-to-lot + within-lot variation of the count itself; count drift shifts GSM + shade between batches (a cross-run consistency defect).
- **USTER percentile (5%/25%/50% USTER)** — Where a yarn sits vs global production per parameter; the common PO language ("must meet 25% USTER on CV% and IPI"). `[FACT — Uster Technologies methodology]`
- **Pilling grade (1–5; Martindale / ICI box / random tumble)** — Lab wear-test rating (5 = no pill, 1 = severe); the objective test behind any "anti-pill" claim and a sourcing acceptance gate (the link from fibre/finish choices to real-world returns).
- **Fabric inspection / 4-point system** — Standard grading of finished-fabric defects per 100 yd²; the C&S-side analogue of yarn QC (sets accept/reject on fabric lots).
- **Bursting strength / dimensional stability (shrinkage %) / spirality** — Knit tests: rupture pressure; size change after wash; twisting (often from unbalanced singles twist) — the finished-garment acceptance tests governing returns, sizing complaints, and the "twisted side seam" defect.
- **AQL (Acceptable Quality Limit) sampling** — Statistical inspection plan setting how many defects trigger batch rejection; the standard buyer/third-party inspection gate on export orders. `[FACT — standard apparel QC; detailed in T5]`

---

## Section 6 — Trade & sourcing (Incoterms, duty, rebates, documents)

Incoterms® 2020 = 11 ICC rules splitting cost/risk between seller and buyer; every PO must name one with year + place (e.g. "FOB Nhava Sheva, Incoterms 2020"). Definitions `[FACT — ICC / trade.gov]`.

- **EXW (Ex Works)** — Seller just makes goods available at its premises; **buyer bears all cost/risk** from the gate (cheapest quote, but we arrange Ludhiana→port→US — max control, max hassle).
- **FOB (Free On Board)** — Seller delivers export-cleared **onto the vessel**; risk/cost passes there (the common apparel quote and baseline for landed-cost math — we own ocean freight + insurance + US import).
- **CIF (Cost, Insurance, Freight)** — Seller pays freight + insurance **to destination port** but **risk still passes at origin** on loading, with minimum-cover insurance (convenient early on).
- **DDP (Delivered Duty Paid)** — Seller bears everything incl. US clearance + duty; lowest-effort, highest price (rarely offered by small Indian units for US duty).
- **FCA / DAP / CFR** — FCA = handed to carrier export-cleared; CFR = CIF minus insurance; DAP = delivered, duty unpaid (know them to compare quotes apples-to-apples).
- **MOQ (Minimum Order Quantity)** — Smallest run a supplier accepts (per style/colour/yarn lot); the gatekeeper for a small D2C line (high MOQs force capsule sizing or job-work). `[FACT concept]`; specific MOQs `[UNKNOWN until T2]`.
- **Lead time** — Calendar time from PO (or yarn-in) to goods-ready/delivered; drives drop calendar + cash tied up (ocean India→US adds weeks — T9). `[FACT concept]`
- **Landed cost** — True per-unit cost to our US door = FOB + freight + insurance + duty + broker + drayage + financing; the number that sets margin (the whole T9 model computes it). `[FACT definition]`
- **HS code / HTS code** — HS = 6-digit global code; **HTS** = US 10-digit extension setting the duty rate. Sweaters/pullovers/cardigans = **heading 6110**; mis-classification = penalties or overpaid duty. `[FACT — 6110 = sweaters/pullovers, USITC]`
- **Duty / tariff** — Ad-valorem import tax; cotton 6110.20 ≈ **7%**, wool 6110.11 ~**16%** — fibre choice changes the duty. `[FACT — USITC/Flexport/UNIS]`. **Any 2025–26 US surcharges / Section-301-type add-ons are `[UNKNOWN]` here → confirm in T9.**
- **Duty drawback (DBK)** — Indian rebate of customs duty on imported inputs used in an exported good; lowers exporter cost (can lower our FOB if passed through). `[FACT — DGFT/CBIC]`
- **RoDTEP / RoSCTL** — Indian export rebate schemes; for apparel (HS ch. 61/62/63) **RoSCTL** applies and **RoDTEP is not double-claimed**, RoSCTL extended through **31 Mar 2026** (affects how cheaply a Ludhiana exporter can quote). `[FACT — AEPC/DGFT]`; post-2026 continuation `[UNKNOWN]`.
- **LC / Letter of Credit** — Bank guarantee the seller is paid on presenting matching docs; de-risks first deals with an unknown supplier but costs bank fees + ties working capital.
- **Customs broker / IOR (Importer of Record)** — Licensed US agent filing the entry; **IOR** = legally liable party for duty/compliance (usually the brand). Broker fee is a real landed-cost line.
- **Merchant exporter / buying house** — Intermediary aggregating factory output + exporting under its name; an alternative to factory-direct that adds margin but eases compliance for a first-timer. `[ESTIMATE: industry-lore]`

---

## Section 7 — Manufacturing & business models

- **CMT (Cut–Make–Trim)** — Factory paid only for labour to cut/sew/link/trim; **buyer supplies the yarn/fabric** (lets a thin brand own materials + design + IP and rent only stitching capacity; lowers capital). `[FACT industry term]`
- **Job-work** — Indian term for subcontracted processing (knit/link/dye) where the principal owns the material and pays a conversion charge; the connective tissue of the Ludhiana cluster — how MSMEs share specialised steps. `[FACT/industry usage]`
- **FOB manufacturing / full-package (FPP)** — Factory sources its own yarn and delivers finished, packed goods at an FOB price; simplest for us (one price, one vendor) but we lose material control + transparency. `[FACT industry term]`
- **Vertical integration** — One firm owns multiple consecutive stages (spinning→knitting→finishing→export); big Ludhiana groups do this, a small brand mimics it only via tight contracts, not ownership.
- **Captive spinning** — A knitter/garmenter runs its own spinning to control yarn count, shade, and cost in-house; explains why integrated groups undercut yarn-buyers (the T2/T4 cost gap).
- **Captive power** — Self-generated electricity (diesel/gas/solar/cogen) to dodge unreliable/costly grid; a real Ludhiana cost + uptime + ESG factor (T1/T4). `[FACT concept; magnitudes → T1]`
- **Make-vs-buy** — Produce a step in-house vs purchase it; **the venture's recurring central lever** — a single Ludhiana manufacturer-cum-exporter can span spinning→export, whereas a thin venture buys yarn or CMT capacity and owns only design + brand + US import. Which boundary we sit at is the key decision (T4/T9). `[ESTIMATE: design-framing, high confidence it's the key lever]`
- **Sampling / pre-production sample (PPS)** — Approval garments (proto → fit → PP) signed off before bulk; gates quality before committing yarn money (a lead-time + cost item often underestimated). `[FACT]`
- **Tech pack** — The spec sheet: measurements, yarn count/shade, GSM, stitch, trims, tolerances, packing; the hand-off from design (T8) to factory (precision here = fewer QC failures). `[FACT]`

---

## Section 8 — Brand / D2C

Definitions `[FACT — standard retail/finance]`. Any *numbers* are `[UNKNOWN]` until T7/T9 model them.

- **D2C retail** — Selling the finished garment directly to the end consumer online/locally, no wholesale middleman; the brand (our venture) in California/Turlock (economics in T7/T9). `[FACT definition]`
- **SKU (Stock-Keeping Unit)** — One uniquely sellable variant = style × colour × size; a 3-style capsule in 4 sizes × 3 colours = 36 SKUs → drives MOQ pain + inventory math.
- **Line plan** — The planned matrix of styles/colours/sizes/prices for a season; a T8 deliverable tying design to MOQ, margin, drop calendar.
- **Capsule** — A small, cohesive mix-and-match set; the venture's deliberately tiny launch unit (e.g. youth capsule) to keep MOQ + risk low.
- **Drop** — A timed, often limited release (vs always-in-stock); a hype-friendly D2C tactic fitting a California/meme brand (T7) — manufactures scarcity + urgency.
- **MSRP** — Manufacturer's Suggested Retail Price = the consumer sticker; top of the pricing ladder.
- **COGS (Cost of Goods Sold)** — Direct cost of the unit sold = landed cost + direct fulfilment-to-stock; the denominator of margin, built from the T9 landed-cost model.
- **Gross margin** — (Revenue − COGS) / Revenue, %; headline product profitability before marketing/ops.
- **Contribution margin** — Price − **all** variable costs (COGS + payment fees + ship-to-customer + returns + variable CAC); the honest per-order profit that must cover fixed costs — **the metric that actually decides D2C viability.**
- **CAC (Customer Acquisition Cost)** — Marketing/sales spend ÷ new customers; for paid-ads D2C this often eats the whole margin (the make-or-break number — T7).
- **AOV (Average Order Value)** — Revenue ÷ orders; higher AOV (bundles, capsule) offsets CAC + shipping.
- **Sell-through** — Units sold ÷ units received over a period; low sell-through = trapped cash + markdowns.
- **LTV (Lifetime Value)** — Total contribution margin a customer yields over their lifetime; **LTV:CAC > ~3** is the D2C survival test. `[FACT/industry rule-of-thumb]`
- **Markdown / clearance** — Price cut to move unsold inventory; the penalty for over-ordering against MOQ, directly attacking gross margin.
- **Working capital / cash conversion cycle** — Cash tied up between paying for yarn and getting paid by customers; long ocean lead time + upfront MOQ = a cash trap that can kill a profitable-on-paper brand.

> **Worked illustration (labelled, not a forecast)** `[ESTIMATE: arithmetic illustration; method = plug assumed values into the identities above; placeholder inputs, not researched]`: MSRP $80, landed COGS $20, payment+ship+returns $10, blended CAC $25 → gross margin 75% but **contribution after CAC ≈ $25/order (~31%)** — showing why CAC, not factory cost, usually governs survival. Real inputs come from T9.

---

## Section 9 — Ludhiana context

- **"Hosiery"** — In Ludhiana usage, **"hosiery" = the knitwear industry broadly** (sweaters, tees, thermals) — *not* just socks; a "hosiery unit" is a knitwear maker. `[FACT — local usage; "winter hosiery"]`
- **"Manchester of India"** — Common epithet for Ludhiana as a textile/knitwear hub; signals scale + heritage (brand-story framing) and cluster dominance. `[FACT — popular usage; cluster docs]`
- **Cluster / agglomeration** — A dense geographic concentration of firms in one trade sharing labour, suppliers, know-how; Ludhiana's core advantage is spinners, knitters, dyers, machine dealers, job-workers all within reach → low coordination cost. `[FACT — cluster economics; Sameeeksha]`
- **MSME (Micro/Small/Medium Enterprise)** — Indian statutory size class; the cluster is **~14,000 MSMEs (~70% micro / 20% small / 10% medium)** → explains fragmentation, low per-unit MOQ flexibility, reliance on job-work. `[FACT — Sameeeksha cluster profile]`
- **Job-work ecosystem** — The web of subcontracting letting micro-units specialise in one step; a brand can assemble a full garment across many tiny vendors — flexible but QC-fragmented (T5). `[ESTIMATE: industry-lore, high confidence]`
- **Yarn market / mandi** — The physical/relational marketplace where yarn is traded by count/shade/lot; where pricing + MOQ are actually negotiated (T2) — relationship- and cash-driven. `[ESTIMATE: industry-lore]`
- **Off-paper economy (mechanism)** — Portions of activity (cash sales, informal labour, under-/over-invoicing, "facilitation") outside formal books; the owner explicitly wants this captured, described as a **structural incentive, never a named-party act.** How it is *generally reported* to operate in fragmented clusters: thin margins + high competition + cash-heavy local trade + fragmented micro-units create incentives for informal cash transactions, informal/contract labour, and discretionary interaction with inspectors where enforcement is patchy. Stated as mechanism + a hypothesis to probe in T1; we do **not** assert any specific person or company does any of it. `[ESTIMATE: discussion/industry-lore; confidence MEDIUM that such mechanisms exist in clusters generally, LOW/UNKNOWN on magnitude in Ludhiana specifically — revisited, never fabricated, in T1]`

---

## Cross-team usage rules (binding for all downstream specialists)

1. **Indirect vs direct counts:** never compare Ne/Nm with Tex/denier without converting; higher Ne/Nm = finer, higher Tex/denier = thicker.
2. **Pivot through Tex** for any conversion; carry **Nm = 1.693 × Ne** for the Ne↔Nm hop.
3. **Folded notation 2/30** = two 30s ≈ resultant 15s — confirm singles count + ends fed before judging fabric weight.
4. **GG must match yarn count** — quote both when specifying a fabric, plus target **GSM / g-per-garment**.
5. **Fully-fashioned + linked ≠ cut-&-sew + overlocked** — keep construction + seaming terms precise; they carry the quality/price story.
6. **Grade yarn against USTER percentiles on CV% + IPI; grade pilling 1–5 by wash test** — the numbers a PO and a QC reject decision actually turn on.
7. **Every PO names an Incoterm** (year + place) and an **HTS line**; fibre choice changes the **duty**.
8. **Contribution margin after CAC**, not gross margin, is the D2C viability test (LTV:CAC > ~3).
9. **Make-vs-buy boundary** (own spinning/knitting vs buy yarn/CMT vs full-package FOB) is the venture's recurring central lever.
10. **Honesty tags travel with claims** downstream; treat every `[FACT]` as a citeable anchor and every `[ESTIMATE]` as a hypothesis to harden or refute; off-paper material stays **mechanism, never accusation**.

---

## Sources (anchored items)

**Technical** — Gauge (knitting), Wikipedia (GG = needles/inch); diamondknitland.com, ritacashmere.com, mysweaterfactory.com (GG→garment/weight, 3/5/7/12GG); maruyasu-fil.com KNIT MAGAZINE (count→gauge); testextextile.com, textileschool.com, textilesbar.com (Ne/Nm/Tex/denier defs + conversions); textilelearner.net, iejrd.com, textilesbar.com (IPI, CV%/U%, thin/thick/nep sensitivities); Uster Technologies USTER STATISTICS (percentile grading).

**Business / chain** — Sameeeksha Ludhiana knitwear cluster profile (~14,000 MSMEs, ≈70/20/10 micro/small/medium): https://sameeeksha.org/index.php?option=com_content&view=article&id=139&Itemid=502 ; ScienceDirect / World Development (early-1990s Ludhiana >80% of India's woollen-knitwear firms): https://www.sciencedirect.com/science/article/abs/pii/S0305750X99000790 ; USITC HTS + Flexport (heading 6110; cotton 6110.20 ≈7%, wool 6110.11 ~16%): https://hts.usitc.gov/search?query=6110 , https://www.flexport.com/data/hs-code/6110-sweaters-pullovers-sweatshirts-waistcoats-vests-and-similar-articles-knitted-or-crocheted/ ; trade.gov "Know Your Incoterms": https://www.trade.gov/know-your-incoterms ; DGFT/CBIC + AEPC FAQ (DBK; RoSCTL applies, RoDTEP not double-claimed, extended to 31 Mar 2026): https://content.dgft.gov.in/Website/EPS.pdf , https://www.aepcindia.com/system/files/FAQ%20on%20RoSCTL.pdf

**Untagged constants** (dimensional identities): Tex = 1000/Nm = 590.5/Ne; denier = 9×Tex; decitex = 10×Tex; Nm = 1.693×Ne; denier ≈ 5315/Ne; CV% ≈ 1.25×U%. Garment-weight-by-gauge figures are `[ESTIMATE: industry-lore]` order-of-magnitude bands, not vendor quotes.

---

## Coverage statement (floor, not ceiling)

This glossary covers the **shared technical + business terms** needed for T1–T11 to interoperate. It deliberately does **not** quote firm-level revenues, specific MOQs/prices, or current-year tariff surcharges — those are `[UNKNOWN]` here by design and belong to T1 (Ludhiana), T2 (yarn), T9 (logistics/unit economics). Behaviour is tagged `[ESTIMATE: discussion/industry-lore]` and framed as mechanism, never accusation.
