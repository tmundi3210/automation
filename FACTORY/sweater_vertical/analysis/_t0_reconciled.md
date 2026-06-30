# T0 — Reconciled Master Vocabulary (Sweater Vertical)

**What this is.** The single, deduped, internally-consistent vocabulary for the whole sweater vertical,
merged from `_t0_tech_vocab.md` (technical/textile) and `_t0_chain_framing.md` (business/value-chain).
Every downstream specialist (T1 Ludhiana → T11 brain) uses these terms identically. This file supersedes
the two source files as the reference; it is the basis for the human-readable `SCOPE_GLOSSARY.md`.

**Honesty tags (binding).** `[FACT]` = cited public source. `[ESTIMATE]` = method + basis + confidence,
uses the literal word "estimate". `[UNKNOWN]` = not reliably known. `[ESTIMATE: industry-lore]` =
trade-craft convention. `[ESTIMATE: discussion/industry-lore]` = off-paper mechanism described
structurally, **never** as a named-party accusation. **Dimensional identities** (unit definitions, count
conversions like `Tex = g/1000 m`) carry **no tag** — they are true by definition, not money/capacity/
behaviour claims. Coverage is a **floor, not a ceiling**: `[UNKNOWN]` is a valid, frequent answer.

**Reconciliation notes (conflicts found & resolved).**
1. *Overlap, not conflict — fibres/counts/FF-vs-C&S/linking/GSM.* Both files touched these; the technical
   file owns the definitions, the business file owned the chain/ownership framing. Merged with the
   technical definition as canonical and the business "who does it / why it's a lever" folded in. No
   contradictions.
2. *Self-reference fixed.* Both source files pointed at a sibling `SCOPE_GLOSSARY.md` as "the other half."
   That split is now collapsed into THIS file; `SCOPE_GLOSSARY.md` is the downstream human artifact built
   *from* here, not a peer to cross-reference. Resolved: one master, no circular pointer.
3. *Duty rate stated once.* Cotton 6110.20 ≈7% / wool 6110.11 ~16% appears once (Trade & sourcing §6),
   tagged `[FACT]` with the 2025–26 surcharge caveat as `[UNKNOWN]` → deferred to T9. No double-quote.
4. *"Captive power" appeared in two business sections* (model terms + Ludhiana context) — deduped to one
   entry in Manufacturing & business models §7, cross-linked from Ludhiana context §9.
5. *Honesty-tag style harmonised.* Technical file used `[ESTIMATE: industry-lore, <confidence>]`; business
   file used `[ESTIMATE]`/`[ESTIMATE: discussion/industry-lore]`. Unified to the tag legend above;
   confidence carried where the source stated it.

---

## SECTION 1 — FIBRES

A sweater's hand, warmth, price, pilling, and care label are decided at the fibre line before a stitch is
knit. Two axes dominate: **staple length** (longer → fewer protruding ends → smoother, stronger, less
pill) and **fibre fineness** (micron for animal, denier-per-filament for synthetic → finer = softer,
more drape).

- **Acrylic** — Synthetic (polyacrylonitrile) staple; the workhorse of mass-market sweaters. Cheap,
  bulky, takes bright dyes, machine-washable, moth-proof; but pills, holds static, low breathability,
  weak when wet-abraded, melts rather than chars. *Matters:* the cost floor for D2C knitwear (most
  sub-$40 retail is acrylic/acrylic-rich); **Ludhiana is fundamentally an acrylic + acrylic-blend
  cluster**, so this is our base-case fibre. `[ESTIMATE: industry-lore, high confidence]`
- **Wool (generic sheep's wool)** — Natural protein/keratin fibre; crimped, elastic, warm even when damp,
  naturally flame- and odour-resistant. Coarse grades (>27 micron) prickle. *Matters:* the
  warmth/resilience benchmark; "wool-rich" is a premium signal but **micron** (not the word "wool")
  decides next-to-skin comfort.
- **Merino** — Fine wool from Merino sheep, ~17–24 micron (sub-19.5 = "fine/superfine"); soft, low
  prickle, good moisture management. *Matters:* the sweet spot for premium-but-wearable D2C; a marketable
  word that justifies a higher price band.
- **Lambswool** — First-shearing wool (~7 months); softer/finer than later clips — a first-clip grade,
  not a species. *Matters:* lets a brand say "soft wool" without merino prices.
- **Cashmere** — Fine downy undercoat of the cashmere goat, ~14–19 micron; extremely soft, light, warm,
  low-yield (≈150–200 g usable down per goat/year), expensive, **pills more** than wool (short fine
  fibres). *Matters:* the luxury anchor; even 5–10% in a blend raises perceived value, but needs careful
  finishing + a care story or it disappoints.
- **Cotton** — Natural cellulosic staple; breathable, hypoallergenic, washable, no prickle, but heavy,
  low elasticity/recovery (bags out), poor warmth-to-weight, slow to dry. *Matters:* the go-to for
  warm-climate / spring-summer knitwear and the **California market** (heavy wool sells fewer months/yr);
  mercerised cotton (§4 Finishing) upgrades sheen + strength.
- **Viscose / Modal (regenerated cellulose, "rayon")** — Cellulose regenerated into filament; silky
  drape, high sheen, very soft, deep dye uptake; but weak when wet, low recovery, creases. Modal = a
  stronger, finer second-generation viscose. *Matters:* added to blends for drape/softness and to cut
  animal-fibre cost; flags a "fluid", less-structured knit.
- **Nylon (polyamide)** — Strong, abrasion-resistant, elastic synthetic; rarely the main fibre, usually a
  reinforcing minority. *Matters:* a few % nylon in a cashmere/lambswool blend sharply improves pill +
  abrasion resistance — a common quality lever. `[ESTIMATE: industry-lore, high confidence]`
- **Blends** — Two+ fibres spun together to trade cost/hand/performance. Common: acrylic/wool (warmth at
  a price), wool/nylon (durability), cotton/acrylic (washability+softness), cashmere/wool or
  cashmere/nylon (luxury hand, controlled cost+pilling), wool/viscose (drape). *Matters:* the blend
  **ratio** on the care label is the single biggest cost-and-positioning decision in the garment; "70/30"
  vs "50/50" moves landed cost meaningfully — a margin lever the brand controls.

**Cross-cutting fibre concepts**
- **Micron** — Animal-fibre diameter in µm; lower = finer/softer/pricier. The grading axis for
  wool/cashmere; it, not "merino," sets comfort and price.
- **Staple length** — Length of one fibre; longer (long-staple cotton, longer wool) → smoother, stronger,
  lower-pill yarn. A cheap pre-wear predictor of pilling for buyers/QC.
- **Pilling** — Tangled balls of broken/loose surface fibres; worse with short, fine, low-twist, loosely
  knit fibres (acrylic, cashmere). The #1 post-purchase complaint / return driver; designed out via fibre
  choice, twist, tighter gauge, and anti-pill finishing. (Quantified by the §5 pilling grade.)

---

## SECTION 2 — YARN METRICS (count, ply, twist)

### 2.1 Count systems (linear density)
Two opposite logics: **Direct** (mass per fixed length → bigger number = thicker): Tex, Decitex, Denier.
**Indirect** (length per fixed mass → bigger number = finer): Ne, Nm, worsted. This inversion is the #1
cross-team confusion and is fixed here.

- **Ne (English cotton count)** — # of 840-yd hanks per pound. Indirect → higher = finer. Dominant for
  cotton and much South-Asian spun yarn; Ludhiana quotes acrylic/cotton in Ne and in folded "2/30"
  notation (§2.2). `[ESTIMATE: industry-lore, high confidence]` (count *system* usage)
- **Nm (metric count)** — Metres per gram (= km per kg). Indirect → higher = finer. Dominant for
  wool/worsted and woollen-spun knitting yarns; European-mill sweater specs are usually Nm.
- **Worsted count (NeK)** — # of 560-yd hanks per pound; wool-system indirect count. Legacy/British wool
  specs — convert to Nm to compare.
- **Tex** — Grams per 1000 m. Direct → higher = thicker. The SI-style, neutral **pivot** for conversion;
  engineering/QC and Uster reports use Tex/Decitex.
- **Decitex (dtex)** — Grams per 10,000 m (= 10 × Tex). Standard for fine filament viscose/nylon/
  polyester in blends.
- **Denier (den)** — Grams per 9000 m. Direct → higher = thicker. The filament/synthetic count (≈ 9 × Tex).

**Conversion (pivot through Tex):**
- Tex = 1000 / Nm  ⇔  Nm = 1000 / Tex
- Tex = 590.5 / Ne  ⇔  Ne = 590.5 / Tex
- Denier = 9 × Tex ; Decitex = 10 × Tex
- **Ne ↔ Nm: Nm = 1.693 × Ne** (Ne = Nm / 1.693). Mnemonic: metric numbers are bigger — multiply Ne by ~1.69.
- **Ne ↔ Denier: Denier ≈ 5315 / Ne.**

Sanity checks: Ne 30 → Nm ≈ 50.8 → Tex ≈ 19.7 → ≈177 den. Nm 48 → Ne ≈ 28.4 → Tex ≈ 20.8. 150 den
→ Tex ≈ 16.7 → Nm ≈ 60. (All dimensional identities — untagged.)

**Rule of thumb:** finer yarn (high Ne/Nm, low Tex/den) → finer achievable gauge, lighter garment, softer
hand, more yardage per kg (so more knit time/kg). Coarser → bulky low-gauge sweater. Count and gauge (§3)
must be matched.

### 2.2 Ply / folded yarn
- **Singles (1-ply)** — One strand off the spinning frame; cheaper, can be weaker, can skew/torque a
  flat panel (unbalanced twist). Fine fashion knits sometimes run singles for lightness — skew is a QC risk.
- **Ply / folded yarn** — Two+ singles twisted together; balances twist, boosts strength, evenness,
  roundness. Most knitwear yarn is 2-ply for durability + clean stitch definition.
- **Folded-count notation (2/30, 2/48 Nm)** — "2/30" = two 30s singles plied. **Resultant ≈ singles count
  ÷ plies**, so 2/30 ≈ 15s-equivalent thickness. Read order varies by region (worsted often writes 30/2);
  always confirm which number is the singles count. The **ends** fed (e.g. "3 ends of 2/48") further
  multiply effective thickness — the standard lever for hitting a target gauge with in-stock count.
  Misreading 2/30 vs 1/30 is a doubling error in fabric weight.
- **Twist (TPI / TPM; direction S vs Z)** — Amount + handedness of twist; more twist = stronger, leaner,
  lower-pill, but harsher hand; balanced ply pairs opposing twists. Under-twisted soft yarn pills;
  over-twisted feels wiry and biases the fabric.

---

## SECTION 3 — KNITTING & CONSTRUCTION

### 3.1 Gauge (GG)
- **GG (gauge)** — Needles per inch on the machine bed; the master variable for fabric fineness. Higher GG
  = more, finer needles → finer, denser, lighter fabric needing finer yarn; lower GG = bulkier from
  thicker yarn. `[FACT — definition; Wikipedia "Gauge (knitting)", diamondknitland.com]` GG must match
  yarn count (§2) and dictates machine selection, knit time/garment, and the look/weight/price tier.
- **E (E-gauge)** — Same concept written with "E" (E7 = 7GG); common on European/Stoll machines. Spec
  sheets mix "7GG" and "E7" — identical.
- **Multi-gauge / 3.5.7 (needle-out coarse looks)** — A finer machine knitting with selected needles out
  to mimic a coarser gauge; lets one machine fake several gauges → affects make-vs-buy + utilisation.

**GG → product map** (industry convention; weights are order-of-magnitude `[ESTIMATE: industry-lore,
medium-high confidence]`):
- **3GG** — Very coarse/"chunky"; heavy statement winter knits, bold cables, bulky low-Nm yarn. ~700 g–1 kg+.
- **5GG** — Coarse; chunky-to-medium winter. ~500–800 g.
- **7GG** — Medium; **the commercial default** — ordinary retail pullovers/cardigans. ~350–550 g. The
  volume sweet spot, likely default for a mainstream D2C piece. `[FACT — mysweaterfactory.com; ritacashmere.com]`
- **9–10GG** — Medium-fine; lighter pullovers, transitional knits. ~280–400 g.
- **12GG** — Fine; lightweight everyday knitwear (fine merino, polos, summer-weight, layering). ~200–350 g.
  The right tier for warm California + "elevated basics."
- **14/16/18GG** — Very fine, near-jersey; fine-merino/silk-blend T-shirt-like knits. ~150–280 g. Premium
  look but needs fine high-count yarn and slower knitting (more cost/piece).

*Yarn-to-gauge pairing* `[ESTIMATE: industry-lore, medium confidence]`: convert resultant count to Nm; a
finer gauge wants a higher-Nm yarn, and the knitter tunes the **number of ends fed** to fill the needle.

### 3.2 Knitting families (machine technologies)
- **Flat-bed (V-bed) knitting** — Two opposed needle beds in a "V"; knits flat panels back-and-forth, can
  transfer stitches → ribs, shaping, structure. Computerised flat = **Shima Seiki** (Japan), **Stoll**
  (Germany, now KARL MAYER STOLL). The core sweater + **fully-fashioned** technology; the machine class
  our production chain centres on.
- **Whole-garment / integral (WholeGarment® Shima, knit&wear® Stoll)** — A flat machine knits the **entire
  seamless garment in one piece** — no panels, no linking. Near-zero cut waste + no/low seaming labour,
  but expensive machines, slower cycle, steep programming skill — a strategic make-decision and an
  AI/automation frontier for the venture.
- **Circular knitting** — Cylinder of needles knits a continuous **jersey tube** at high speed; output is
  yardage (later cut & sewn), not shaped panels. Basis of cut-&-sew knit; high throughput, low per-metre
  cost, but no fashioning → more cut waste, more casual product.
- **Warp knitting (tricot/raschel)** — Many parallel yarns looped **along** the length; fast, stable,
  lace/mesh/technical. Mostly outside classic sweaters (trims/technical) — know it so it isn't confused
  with weft/flat.
- **Hand-frame / domestic flat knitting** — Manual/semi-auto flat machines; small artisanal capacity +
  sampling — relevant to tiny-line make-vs-buy and prototyping.

*Orientation:* flat & circular are **weft** knitting (one yarn forms a course across); warp uses many
yarns down the wales. Sweaters = overwhelmingly weft/flat.

### 3.3 Construction: fully-fashioned vs cut-&-sew
- **Fully-fashioned (FF)** — Each panel knit to final shape on flat-bed via stitch increase/decrease, then
  **linked** (§3.5) edge-to-edge. Premium: minimal yarn waste, clean shaped edges, no fraying raw edges,
  better drape/fit; "fully-fashioned" is a price justifier.
- **Cut-&-sew (C&S)** — Knit a flat/circular **blank**, then **cut** shapes and **sew** (overlock/
  coverstitch) like wovens. Faster/cheaper for fine jersey + volume, but **more waste**, curling/fraying
  cut edges, more casual — the cost-vs-quality counterpoint to FF in every make decision.
- **Integral / seamless (recap)** — Whole-garment knitting (§3.2) eliminates cutting + most seaming; the
  third option that rewrites the labour/waste maths.
- **Fashioning marks** — Small diagonal stitch marks where a panel was decreased; a visible authenticity
  cue that a sweater is genuinely FF, not cut.

### 3.4 Stitch types / knit structures
- **Jersey / plain / single knit (stockinette)** — Single-face knit; smooth "V" face, looped back; light,
  curls at edges, can ladder. The lightweight base (fine-gauge tops, C&S bodies); cheapest yarn-wise but
  needs edge finishing.
- **Rib (1×1, 2×2)** — Alternating face/back wales → vertical ridges + strong widthwise stretch/recovery.
  Standard for cuffs/hems/collars; rib quality = fit + recovery, a key QC point.
- **Links-links (purl)** — Alternates knit/purl **along the wale** → reversible, textured, lengthwise-
  stretchy; needs a purl/links-capable machine — flags a machine-capability requirement.
- **Cable** — Wales crossed via stitch transfer → rope-like raised twists. Classic heavy-knit value
  (esp. 3–7GG) but uses more yarn + machine time → cost driver.
- **Intarsia** — Blocks of different-coloured yarn with **no floats** on the back (each colour worked only
  in its zone). Clean colour-blocking/logos, no reverse waste, but slower/skilled + machine-dependent.
- **Jacquard** — Multi-colour pattern where unused colours **float/are carried** across the back (single
  or bird's-eye/backed). Rich all-over/Fair-Isle looks; heavier (carried yarn) + back-float management is
  a quality issue.
- **Pointelle** — Decorative open/eyelet holes by stitch transfer; lacy, lightweight — feminine/spring
  fine-gauge styling, relevant for warm-climate California lighter knits.
- **Tuck & miss (float) stitches** — Held (tucked) or skipped (missed) loops → texture/colour effects; the
  building blocks of the above. Texture without extra colours; affects width, weight, stretch.
- **Plating** — Two yarns fed together so one shows on face, the other on back (soft face / strong back,
  or elastane plated for stretch). A way to get a luxe face cheaply or add stretch.

### 3.5 Seaming / joining
- **Linking** — Joining two knitted edges **loop-to-loop** on a **linking machine** (circular dial of
  points, "X-gauge" cups), giving a flat, near-invisible, stretchy seam aligned stitch-for-stitch. The
  defining finishing step of quality FF sweaters; "fully-linked"/"hand-linked" is a quality claim;
  skilled, labour-intensive, **throughput-limiting** — a real cost + capacity bottleneck. `[ESTIMATE:
  industry-lore, high confidence]`
- **Looping** — Synonym/relative for attaching trims (collars, plackets) loop-to-loop; "looping" and
  "linking" used interchangeably by region.
- **Mock-linking** — A linking-machine/overlock seam that **imitates** a true linked look without full
  loop-to-loop registration; faster/cheaper, slightly less clean — a cost-down buyers/QC must distinguish
  from true linking.
- **Overlock / cup-seam / coverstitch** — Standard sewn seams for C&S and lower-cost knits; bulkier, less
  stretchy than linking. The cheaper seaming class — presence signals C&S/value construction.

---

## SECTION 4 — FINISHING

(Post-knit treatments; the full T6 deep-dive expands these. "After yarn/garment, what reaches best quality.")

- **GSM (grams per square metre)** — Mass of one m² of fabric; the universal weight metric. The objective
  spec for "how heavy/warm," set on the tech pack, checked in QC; pairs with GG + count to define the
  product. For shaped sweaters also tracked as **grams per garment** at a size. `[ESTIMATE: industry-lore,
  high confidence]` (Listed here as the weight anchor; also a QC dial in §5.)
- **Stitch density (courses/wales per inch or cm)** — Loops per unit length/width; tighter = heavier, more
  stable, lower-pill. With GSM, catches too-loose (saggy) or too-tight (stiff, yarn-hungry) knit.
- **Loop/stitch length (yarn per stitch)** — Yarn in one loop; the deepest control of weight, tightness,
  yarn cost — the parameter the programmer tunes to hit GSM + yarn cost simultaneously.
- **Milling / fulling** — Controlled wet/heat/agitation that felts and **closes up** a wool knit → fuller,
  softer, warmer hand. Over-milling shrinks/felts irreversibly → a QC risk.
- **Anti-pill finish** — Chemical/enzyme (e.g. cellulase bio-polishing on cellulosics) or mechanical
  treatment, plus low-pill fibre/twist choices. Attacks the top return driver; an "anti-pill" claim must
  survive wash-test grading (§5 pilling grade).
- **Mercerised (mercerisation)** — Caustic-soda treatment of cotton under tension → lustre, strength,
  better dye uptake. Upgrades cotton to premium "mercerised cotton" — relevant to the warm-market line.
- **Sanforised / compacted (shrink control)** — Mechanical pre-shrinking/compacting to meet a residual-
  shrinkage limit; keeps the garment in size tolerance after the customer washes it (returns + care-label
  compliance). Term is woven-origin but the concept applies to knits via compacting/relaxation.
- **Scouring / washing / softening** — Remove spinning oils/dirt, add softeners/silicone for hand. Raw
  off-machine knits feel harsh; finishing makes the sample feel like retail product.
- **Steaming / pressing / blocking / Kier-relax** — Heat-set to final dimensions, relax stresses so
  panels/garments hold shape + measurements. Skipped/rushed pressing shows as measurement + appearance
  defects.
- **Dyeing stages** — **Fibre/stock-dyed**, **yarn/package-dyed**, **piece/garment-dyed**; later-stage =
  faster trend response but more colour-consistency risk. A lead-time vs colour-fastness trade-off and a
  lot-to-lot shade-matching QC point.

---

## SECTION 5 — QC (yarn evenness, defects, fabric & garment tests)

Yarn quality is graded against the **USTER® STATISTICS** benchmark percentiles (e.g. "25% USTER" = better
than 75% of world production). Lower irregularity = cleaner knitting, fewer stops, fewer defects.

- **Yarn evenness / irregularity** — Uniformity of yarn mass along its length (optical or capacitive
  tester). Uneven yarn → streaky fabric, knitting stops, visible defects → scrap + downtime (direct cost).
- **CV% (Coefficient of Variation of mass; CVm)** — Std statistical measure of mass irregularity (std dev
  / mean of mass per unit length, %); the headline evenness number, lower = more even. `[FACT —
  textilesbar.com; standard textile metrology]` The number buyers anchor on when grading lots.
- **U% (unevenness %)** — Older mean-deviation measure; **CV% ≈ 1.25 × U%** (normal distribution). Legacy
  specs quote U% — convert to compare.
- **IPI (Imperfection Index)** — **Thin places + thick places + neps per 1000 m** at standard
  sensitivities; the point-defect count + the headline "cleanliness" number alongside CV%; high IPI
  predicts visible faults + knitting stops. `[FACT — textilelearner.net; iejrd.com]`
- **Thin place** — Cross-section ≥ ~50% below mean (−50% sensitivity); weak point that can break in
  knitting, shows as a light streak.
- **Thick place** — ≥ ~50% above mean (+50%); shows as a heavy/dark fault.
- **Nep** — Tight tangled knot of fibre, ≥ ~200% of mean (+200%); visible specks, worse with short/immature
  fibre + rough processing — a common reject cause.
- **Hairiness (H / S3)** — Protruding fibre ends on the yarn surface; affects pilling, hand, shed/lint.
  Too high → pilling + dusty knitting; too low → flat/harsh look.
- **Tenacity / elongation (single-yarn strength)** — Breaking force (cN/tex) + stretch at break. Low
  strength → breaks in knitting (downtime) + weak seams; a core acceptance test.
- **Count CV / count variation** — Lot-to-lot + within-lot variation of the count itself; count drift
  shifts GSM + shade between batches — a consistency defect across runs.
- **USTER STATISTICS percentile (5%/25%/50% USTER)** — Where a yarn sits vs global production per
  parameter; the common PO language ("must meet 25% USTER on CV% and IPI"). `[FACT — Uster Technologies
  methodology]`
- **Pilling grade (1–5; Martindale / ICI box / random tumble)** — Lab wear-test rating (5 = no pill, 1 =
  severe). The objective test behind any "anti-pill" claim; a sourcing acceptance gate and the link from
  fibre/finish choices to real-world returns.
- **Fabric inspection / 4-point system** — Standard for grading finished-fabric defects per 100 yd²; the
  C&S-side analogue of yarn QC; sets accept/reject on fabric lots.
- **Bursting strength / dimensional stability (shrinkage %) / spirality** — Knit tests: pressure to
  rupture; size change after wash; twisting of a tube/garment (often from unbalanced singles twist). The
  finished-garment acceptance tests governing returns, sizing complaints, and the "twisted side seam"
  defect.
- **AQL (Acceptable Quality Limit) sampling** — Statistical inspection plan setting how many defects in a
  sampled batch trigger rejection; the standard buyer/third-party inspection gate on export orders.
  `[FACT — standard apparel QC; detailed in T5]`

---

## SECTION 6 — TRADE & SOURCING (Incoterms, duty, rebates, documents)

Incoterms® 2020 = 11 ICC rules splitting cost/risk between seller and buyer. Definitions `[FACT]`
(ICC / trade.gov). Every PO must name one with year + place (e.g. "FOB Nhava Sheva, Incoterms 2020").

- **EXW (Ex Works)** — Seller just makes goods available at its premises; **buyer bears all cost/risk** from
  the gate. Cheapest quote but we arrange Ludhiana→port→US; max control, max hassle.
- **FOB (Free On Board)** — Seller delivers, export-cleared, **onto the vessel**; risk/cost passes there.
  The common apparel quote; we own ocean freight + insurance + US import. Baseline for landed-cost math.
- **CIF (Cost, Insurance, Freight)** — Seller pays freight + insurance **to destination port**, but **risk
  still passes at origin** on loading; insurance is minimum-cover. Convenient early on.
- **DDP (Delivered Duty Paid)** — Seller bears everything incl. US clearance + duty. Lowest-effort, highest
  price; rarely offered by small Indian units for US duty.
- **FCA / DAP / CFR** — FCA = handed to carrier export-cleared; CFR = CIF minus insurance; DAP = delivered,
  duty unpaid. Know them to compare quotes apples-to-apples.
- **MOQ (Minimum Order Quantity)** — Smallest run a supplier accepts (per style/colour/yarn lot). Gatekeeper
  for a small D2C line; high yarn/knit MOQs force capsule sizing (T8) or job-work. `[FACT concept]`;
  specific MOQs `[UNKNOWN until T2]`.
- **Lead time** — Calendar time from PO (or yarn-in) to goods-ready / delivered. Drives drop calendar +
  cash tied up; ocean India→US adds weeks (T9). `[FACT concept]`
- **Landed cost** — True per-unit cost to our US door: FOB + freight + insurance + duty + broker + drayage
  + financing. The number that sets margin; the whole T9 model computes it. `[FACT definition]`
- **HS code / HTS code** — HS = 6-digit global code; **HTS** = US 10-digit extension setting the duty rate.
  Sweaters/pullovers/cardigans = **heading 6110**. Mis-classification = penalties or overpaid duty.
  `[FACT — 6110 = sweaters/pullovers, USITC]`
- **Duty / tariff** — Ad-valorem tax on import. Cotton 6110.20 ≈ **7%**; wool 6110.11 ~**16%** range —
  fibre choice changes the duty. `[FACT — 6110.20 general ≈7%, 6110.11 ~16%, USITC/Flexport/UNIS]`. **Any
  2025–26 US surcharges / Section-301-type add-ons are `[UNKNOWN]` here → confirm in T9.**
- **Duty drawback (DBK)** — Indian rebate of customs duty paid on imported inputs used in an exported good;
  lowers exporter cost → can lower our FOB if passed through. `[FACT — DGFT/CBIC]`
- **RoDTEP / RoSCTL** — Indian export rebate schemes. For apparel (HS ch. 61/62/63) **RoSCTL** applies and
  **RoDTEP is not double-claimed** on the same goods; RoSCTL extended through **31 Mar 2026**. Affects how
  cheaply a Ludhiana exporter can quote. `[FACT — AEPC/DGFT]`; post-2026 continuation `[UNKNOWN]`.
- **LC / Letter of Credit** — Bank guarantee that the seller is paid on presenting docs matching LC terms.
  De-risks first deals with an unknown supplier; costs bank fees + ties working capital.
- **Customs broker / IOR (Importer of Record)** — Licensed US agent filing the entry; **IOR** = legally
  liable party for duty/compliance (usually the brand). Broker fee is a real landed-cost line.
- **Merchant exporter / buying house** — Intermediary aggregating factory output + exporting under its
  name; an alternative to factory-direct — adds margin but eases compliance for a first-timer.
  `[ESTIMATE: industry-lore]`

---

## SECTION 7 — MANUFACTURING & BUSINESS MODELS

- **CMT (Cut–Make–Trim)** — Factory paid only for labour to cut, sew/link, trim; **buyer supplies the
  fabric/yarn**. Lets a thin brand own materials + design and rent only stitching capacity; lowers capital,
  keeps IP. `[FACT industry term]`
- **Job-work** — Indian term for subcontracted processing (knit/link/dye) where the principal owns the
  material and pays a conversion charge. The connective tissue of the Ludhiana cluster — how MSMEs share
  specialised steps. `[FACT/industry usage]`
- **FOB manufacturing / full-package (FPP)** — Factory sources its own yarn and delivers finished, packed
  goods at an FOB price. Simplest for us (one price, one vendor) but we lose material control +
  transparency. `[FACT industry term]`
- **Vertical integration** — One firm owns multiple consecutive stages (spinning→knitting→finishing→
  export). Big Ludhiana groups do this; a small brand mimics it only via tight contracts, not ownership.
- **Captive spinning** — A knitter/garmenter runs its own spinning to control yarn count, shade, cost
  in-house. Explains why integrated groups undercut yarn-buyers; relevant to the T2/T4 cost gap.
- **Captive power** — Self-generated electricity (diesel/gas/solar/cogen) to dodge unreliable/costly grid.
  A real Ludhiana cost + uptime factor; affects unit cost + ESG story (T1/T4). *(Deduped: also referenced
  from Ludhiana context §9.)* `[FACT concept; magnitudes → T1]`
- **Make-vs-buy** — Produce a step in-house vs purchase it; the recurring lever across the venture (own a
  machine? own spinning? own knitting?). **Central framing:** a single Ludhiana **manufacturer-cum-exporter**
  can span spinning→export, whereas a thin venture buys yarn or CMT capacity and owns only design + brand +
  US import — which boundary we sit at is the key decision (T4/T9). `[ESTIMATE: design-framing, high
  confidence it's the key lever]`
- **Sampling / pre-production sample (PPS)** — Approval garments (proto → fit → PP) signed off before bulk;
  gates quality before committing yarn money; a lead-time + cost item often underestimated. `[FACT]`
- **Tech pack** — The spec sheet: measurements, yarn count/shade, GSM, stitch, trims, tolerances, packing.
  The hand-off from design (T8) to factory; precision here = fewer QC failures. `[FACT]`

---

## SECTION 8 — BRAND / D2C

Definitions `[FACT]` (standard retail/finance). Any *numbers* are `[UNKNOWN]` until T7/T9 model them.

- **D2C retail** — Selling the finished garment directly to the end consumer online/locally, no wholesale
  middleman. The brand (our venture) in California/Turlock; economics in T7/T9. `[FACT definition]`
- **SKU (Stock-Keeping Unit)** — One uniquely sellable variant = style × colour × size. A 3-style capsule
  in 4 sizes × 3 colours = 36 SKUs → drives MOQ pain + inventory math.
- **Line plan** — The planned matrix of styles/colours/sizes/prices for a season; a T8 deliverable tying
  design to MOQ, margin, drop calendar.
- **Capsule** — A small, cohesive mix-and-match set; the venture's deliberately tiny launch unit (e.g.
  youth capsule) to keep MOQ + risk low.
- **Drop** — A timed, often limited release (vs always-in-stock); a hype-friendly D2C tactic fitting a
  California/meme brand (T7) — manufactures scarcity + urgency.
- **MSRP** — Manufacturer's Suggested Retail Price = the consumer sticker; top of the pricing ladder.
- **COGS (Cost of Goods Sold)** — Direct cost of the unit sold = landed cost + direct fulfilment-to-stock;
  the denominator of margin, built from the T9 landed-cost model.
- **Gross margin** — (Revenue − COGS) / Revenue, %. Headline product profitability before marketing/ops.
- **Contribution margin** — Price − **all** variable costs (COGS + payment fees + ship-to-customer +
  returns + variable CAC); the honest per-order profit that must cover fixed costs — the metric that
  actually decides D2C viability.
- **CAC (Customer Acquisition Cost)** — Marketing/sales spend ÷ new customers. For paid-ads D2C this often
  eats the whole margin — the make-or-break number (T7).
- **AOV (Average Order Value)** — Revenue ÷ orders; higher AOV (bundles, capsule) offsets CAC + shipping.
- **Sell-through** — Units sold ÷ units received over a period; low sell-through = trapped cash + markdowns.
- **LTV (Lifetime Value)** — Total contribution margin a customer yields over their lifetime; **LTV:CAC**
  (>~3 healthy) is the D2C survival test. `[FACT/industry rule-of-thumb]`
- **Markdown / clearance** — Price cut to move unsold inventory; the penalty for over-ordering against MOQ,
  directly attacks gross margin.
- **Working capital / cash conversion cycle** — Cash tied up between paying for yarn and getting paid by
  customers. Long ocean lead time + upfront MOQ = a cash trap that can kill a profitable-on-paper brand.

**Worked illustration (labelled, not a forecast).** `[ESTIMATE: arithmetic illustration, method = plug
assumed values into the identities above, confidence n/a — placeholder inputs, not researched]`: if MSRP
= $80, landed COGS = $20, payment+ship+returns = $10, blended CAC = $25, then gross margin = 75% but
**contribution after CAC ≈ $25/order (~31%)** — showing why CAC, not factory cost, usually governs
survival. Real inputs come from T9.

---

## SECTION 9 — LUDHIANA CONTEXT

- **"Hosiery"** — In Ludhiana usage, **"hosiery" = the knitwear industry broadly** (sweaters, tees,
  thermals) — *not* just socks. A "hosiery unit" is a knitwear maker. `[FACT — local usage; "winter
  hosiery"]`
- **"Manchester of India"** — Common epithet for Ludhiana as a textile/knitwear hub; signals scale +
  heritage (brand-story framing) and that the cluster is dominant. `[FACT — popular usage; cluster docs]`
- **Cluster / agglomeration** — A dense geographic concentration of firms in one trade sharing labour,
  suppliers, know-how. Ludhiana's core advantage: spinners, knitters, dyers, machine dealers, job-workers
  all within reach → low coordination cost. `[FACT — cluster economics; Sameeeksha]`
- **MSME (Micro/Small/Medium Enterprise)** — Indian statutory size class. The cluster is **~14,000 MSMEs,
  ~70% micro / 20% small / 10% medium** → explains fragmentation, low per-unit MOQ flexibility, reliance
  on job-work. `[FACT — Sameeeksha cluster profile]`
- **Job-work ecosystem** — The web of subcontracting letting micro-units specialise in one step; a brand
  can assemble a full garment across many tiny vendors — flexible but QC-fragmented (T5). `[ESTIMATE:
  industry-lore, high confidence]`
- **Yarn market / mandi** — The physical/relational marketplace where yarn is traded by count/shade/lot;
  where pricing + MOQ are actually negotiated (T2) — relationship- and cash-driven. `[ESTIMATE:
  industry-lore]`
- **Captive power** — (full entry §7) Self-generation to beat grid unreliability/cost; a recurring Ludhiana
  cost line + resilience factor.
- **Off-paper economy (mechanism)** — Portions of activity (cash sales, informal labour, under-/over-
  invoicing, "facilitation") outside formal books. The owner explicitly wants this captured. Described as
  a **structural incentive, never a named-party act.** `[ESTIMATE: discussion/industry-lore]`

**Off-paper mechanism — how it is *generally reported* to operate in fragmented industrial clusters
(structural, not an accusation about any named firm):** thin margins + high competition + cash-heavy
local trade + fragmented micro-units create incentives for informal cash transactions, informal/contract
labour, and discretionary interaction with inspectors/regulators where enforcement is patchy. Stated as a
*mechanism* and a hypothesis to probe in T1; we do **not** assert any specific person or company does any
of it. `[ESTIMATE: discussion/industry-lore; confidence MEDIUM that such mechanisms exist in clusters
generally, LOW/UNKNOWN on magnitude in Ludhiana specifically — revisited, never fabricated, in T1]`

---

## CROSS-TEAM USAGE RULES (binding for all downstream specialists)

1. **Indirect vs direct counts:** never compare Ne/Nm with Tex/denier without converting; higher Ne/Nm =
   finer, higher Tex/denier = thicker.
2. **Pivot through Tex** for any conversion; carry **Nm = 1.693 × Ne** for the Ne↔Nm hop.
3. **Folded notation 2/30** = two 30s ≈ resultant 15s — confirm singles count + ends fed before judging
   fabric weight.
4. **GG must match yarn count**; quote both when specifying a fabric, plus target **GSM / g-per-garment**.
5. **Fully-fashioned + linked ≠ cut-&-sew + overlocked** — keep construction + seaming terms precise; they
   carry the quality/price story.
6. **Grade yarn against USTER percentiles on CV% + IPI**; **grade pilling 1–5 by wash test** — those are
   the numbers a PO and a QC reject decision actually turn on.
7. **Every PO names an Incoterm** (year + place) and an **HTS line**; fibre choice changes the **duty**.
8. **Contribution margin after CAC**, not gross margin, is the D2C viability test (LTV:CAC > ~3).
9. **Make-vs-buy boundary** (own spinning/knitting vs buy yarn/CMT vs full-package FOB) is the venture's
   recurring central lever.
10. **Honesty tags travel with claims** downstream; treat every `[FACT]` as a citeable anchor and every
    `[ESTIMATE]` as a hypothesis to harden or refute; off-paper material stays **mechanism, never
    accusation**.

---

## SOURCES (anchored items)

**Technical**
- Gauge (knitting), Wikipedia — GG = needles per inch.
- diamondknitland.com; ritacashmere.com; mysweaterfactory.com — GG→garment/weight conventions (3/5/7/12GG).
- maruyasu-fil.com (KNIT MAGAZINE) — yarn-count-to-gauge guidance (folded counts → gauge).
- testextextile.com; textileschool.com; textilesbar.com — Ne/Nm/Tex/denier definitions & conversions.
- textilelearner.net; iejrd.com; textilesbar.com — IPI, CV%/U%, thin/thick/nep sensitivities; USTER.
- Uster Technologies (USTER STATISTICS) — percentile grading methodology.

**Business / chain**
- Sameeeksha Ludhiana knitwear cluster profile — ~14,000 MSMEs, ≈70% micro / 20% small / 10% medium.
  https://sameeeksha.org/index.php?option=com_content&view=article&id=139&Itemid=502
- ScienceDirect / World Development — early-1990s Ludhiana >80% of India's woollen-knitwear firms, >90% of
  woollen/acrylic knitwear output. https://www.sciencedirect.com/science/article/abs/pii/S0305750X99000790
- USITC HTS; Flexport — heading 6110 = sweaters/pullovers; cotton 6110.20 ≈7%, wool 6110.11 ~16%.
  https://hts.usitc.gov/search?query=6110 ;
  https://www.flexport.com/data/hs-code/6110-sweaters-pullovers-sweatshirts-waistcoats-vests-and-similar-articles-knitted-or-crocheted/
- trade.gov "Know Your Incoterms" — EXW/FOB/CIF/DDP cost-risk split. https://www.trade.gov/know-your-incoterms
- DGFT/CBIC; AEPC FAQ — DBK; RoSCTL applies for apparel (RoDTEP not double-claimed), extended to 31 Mar 2026.
  https://content.dgft.gov.in/Website/EPS.pdf ; https://www.aepcindia.com/system/files/FAQ%20on%20RoSCTL.pdf

**Untagged constants** (dimensional identities, stated plainly, not tagged): Tex = 1000/Nm = 590.5/Ne;
denier = 9×Tex; decitex = 10×Tex; Nm = 1.693×Ne; denier ≈ 5315/Ne; CV% ≈ 1.25×U%. Garment-weight-by-gauge
figures are `[ESTIMATE: industry-lore]` order-of-magnitude bands, not vendor quotes.

---

## COVERAGE STATEMENT (floor, not ceiling)

This master vocabulary covers the **shared technical + business terms** needed for T1–T11 to interoperate.
It deliberately does **not** quote firm-level revenues, specific MOQs/prices, or current-year tariff
surcharges — those are `[UNKNOWN]` here by design and belong to T1 (Ludhiana), T2 (yarn), T9 (logistics/
unit economics). Behaviour is tagged `[ESTIMATE: discussion/industry-lore]` and framed as mechanism.
Downstream agents treat every `[FACT]` as a citeable anchor and every `[ESTIMATE]` as a hypothesis to
harden or refute.
