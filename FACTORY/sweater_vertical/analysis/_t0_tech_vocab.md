# T0 — Technical Vocabulary (Textile / Knitwear)

Shared technical glossary for the sweater vertical. Every downstream specialist (T1 Ludhiana → T9
logistics) must use these terms identically. Scope: fibre, yarn count, ply, knitting gauge, knitting
families, construction (fully-fashioned vs cut-&-sew), seaming, stitch types, fabric weight, finishing,
and QC/evenness. Each entry = one precise line + **why it matters to a sweater business**.

**Honesty tags.** Physical/definitional facts (e.g. "Tex = g/1000 m") are inherent definitions, not
money/capacity/regulation/behaviour claims, so they carry no tag — they are true by definition of the
unit. Where a *business* claim creeps in (typical pricing, capacity, what buyers do), it is tagged
`[FACT]` (public source), `[ESTIMATE]` (method+basis+confidence, literal word "estimate"), or
`[UNKNOWN]`. Conversion constants below are dimensional identities and are stated plainly.

---

## 1. Fibre types and sweater-relevant properties

A sweater's hand, warmth, price, pilling, and care label are decided at the fibre line before a single
stitch is knit. Two axes dominate: **staple length** (longer staple → fewer protruding fibre ends →
smoother, stronger, less pill) and **fibre fineness** (micron for animal fibre, denier-per-filament for
synthetic → finer = softer hand, more drape).

- **Acrylic** — Synthetic (polyacrylonitrile) staple fibre; the workhorse of mass-market sweaters.
  Cheap, bulky, takes bright dyes, machine-washable, moth-proof; but pills, holds static, low breathability,
  weak when wet-abraded, melts rather than chars. *Why it matters:* it is the default cost floor for a D2C
  sweater — most sub-$40 retail knitwear is acrylic or acrylic-rich. Ludhiana is fundamentally an
  **acrylic + acrylic-blend** cluster, so this is the base case fibre for our supply chain. `[ESTIMATE:
  industry-lore, high confidence]`
- **Wool (generic sheep's wool)** — Natural protein/keratin fibre; crimped, elastic, warm even when damp,
  naturally flame-resistant and odour-resistant. Coarser grades (>27 micron) prickle on skin. *Why it
  matters:* the warmth/resilience benchmark; "wool-rich" is a premium signal but micron grade decides
  whether it's next-to-skin comfortable.
- **Merino** — Fine wool from Merino sheep, typically ~17–24 micron (sub-19.5 = "fine/superfine"); soft,
  low prickle, good moisture management, premium positioning. *Why it matters:* the sweet spot for a
  premium-but-wearable D2C knit; "merino" is a marketable word that justifies a higher price band.
- **Lambswool** — Wool from a sheep's first shearing (~7 months), softer and finer than later clips; not a
  separate species, a first-clip grade. *Why it matters:* lets a brand say "soft wool" without paying
  merino prices.
- **Cashmere** — Fine downy undercoat of the cashmere goat, ~14–19 micron; extremely soft, light, warm,
  low-yield (≈150–200 g usable down per goat/year), expensive, pills more than wool because of short fine
  fibres. *Why it matters:* the luxury anchor; tiny fibre content (even 5–10% in a blend) raises perceived
  value, but pure cashmere demands careful finishing and a care story or it pills and disappoints.
- **Cotton** — Natural cellulosic staple; breathable, hypoallergenic, machine-washable, no prickle, but
  heavy, low elasticity/recovery (bags out), poor warmth-to-weight, slow to dry. *Why it matters:* the
  go-to for warm-climate / spring-summer knitwear and the California market where heavy wool sells fewer
  months a year; mercerised cotton (see §9) upgrades sheen and strength.
- **Viscose / Modal (regenerated cellulose, "rayon")** — Cellulose chemically regenerated into filament;
  silky drape, high sheen, very soft, takes dye deeply; but weak when wet, low recovery, shrinks/creases.
  Modal is a stronger, finer second-generation viscose. *Why it matters:* added to blends for drape and
  softness and to cut cost vs animal fibre; flags a "fluid", less structured knit.
- **Nylon (polyamide)** — Strong, abrasion-resistant, elastic synthetic; rarely the main fibre, usually a
  reinforcing minority (cuffs, high-wear, and to add durability to soft blends). *Why it matters:* a few
  percent nylon in a cashmere/lambswool blend dramatically improves pill and abrasion resistance — a
  common quality lever. `[ESTIMATE: industry-lore, high confidence]`
- **Blends** — Two or more fibres spun together to trade off cost, hand, and performance. Common sweater
  blends: acrylic/wool (warmth at a price), wool/nylon (durability), cotton/acrylic (washability +
  softness), cashmere/wool or cashmere/nylon (luxury hand, controlled cost & pilling), wool/viscose
  (drape). *Why it matters:* the blend ratio on the care label is the single biggest cost-and-positioning
  decision in the whole garment; "70/30" vs "50/50" can move landed cost meaningfully. The exact ratio is
  a margin lever the brand controls.

**Cross-cutting fibre concepts**

- **Micron** — Diameter of an animal fibre in micrometres; lower = finer/softer/pricier. The grading axis
  for wool/cashmere. *Why it matters:* it, not the word "merino," determines next-to-skin comfort and
  price.
- **Staple length** — Length of an individual fibre; longer staple (e.g. long-staple cotton, longer wool)
  → smoother, stronger, lower-pill yarn. *Why it matters:* a cheap way buyers and QC predict pilling
  before wear-testing.
- **Pilling** — Tangled balls of broken/loose surface fibres; worse with short, fine, low-twist, loosely
  knit fibres (acrylic, cashmere). *Why it matters:* the #1 post-purchase complaint and return driver for
  knitwear; designed-out via fibre choice, twist, tighter gauge, and anti-pill finishing.

---

## 2. Yarn count systems (yarn fineness / linear density)

Yarn count expresses how fine or coarse a yarn is. Two opposite logics:

- **Direct systems** (mass per fixed length): **bigger number = thicker/heavier yarn.** Tex, Decitex,
  Denier.
- **Indirect systems** (length per fixed mass): **bigger number = finer yarn.** Ne (cotton), Nm (metric),
  worsted. This inversion is the #1 source of cross-team confusion, so it is fixed here.

Definitions:

- **Ne (English cotton count)** — Number of 840-yard hanks per 1 pound. Ne 30 = thirty 840-yd hanks weigh
  one pound. Indirect → higher Ne = finer. *Why it matters:* the dominant count for cotton and for much of
  the South-Asian spun-yarn trade; Ludhiana quotes acrylic/cotton spun yarns in Ne and in "2/30"-style
  folded notation (§3). `[ESTIMATE: industry-lore, high confidence]`
- **Nm (metric count)** — Number of metres per 1 gram (i.e. metres in 1 g; equivalently km per kg).
  Nm 48 = 48 m weighs 1 g. Indirect → higher Nm = finer. *Why it matters:* the dominant count for
  wool/worsted and woollen-spun knitting yarns; sweater yarn specs from European mills and worsted yarn
  are usually in Nm.
- **Worsted count (NeK)** — Number of 560-yard hanks per pound; a wool-system indirect count. *Why it
  matters:* appears on legacy/British wool specs; convert to Nm to compare.
- **Tex** — Mass in grams of 1000 m of yarn. Direct → higher Tex = thicker. *Why it matters:* the SI-style
  count; engineering/QC and Uster reports often use Tex/Decitex, so it's the neutral pivot for conversion.
- **Decitex (dtex)** — Mass in grams of 10,000 m (= 10 × Tex). Used for fine filament. *Why it matters:*
  standard for filament viscose/nylon/polyester used in blends.
- **Denier (den)** — Mass in grams of 9000 m. Direct → higher denier = thicker. *Why it matters:* the
  filament/synthetic count (the "denier" on nylon, the filament inputs to blends); ≈ 9 × Tex.

### Conversion intuition (memorise the pivots)

Use **Tex as the pivot** — everything is one step away:

- Tex = 1000 / Nm  ⇔  Nm = 1000 / Tex
- Tex = 590.5 / Ne  ⇔  Ne = 590.5 / Tex
- Denier = 9 × Tex  ;  Decitex = 10 × Tex
- **Ne ↔ Nm: Nm = 1.693 × Ne**  (so Ne = Nm / 1.693). Mnemonic: *metric numbers are bigger; multiply Ne by ~1.69.*
- **Ne ↔ Denier: Denier ≈ 5315 / Ne.**

Worked sanity checks (round numbers):
- Ne 30 → Nm ≈ 50.8 → Tex ≈ 19.7 → ≈ 177 denier.
- Nm 48 → Ne ≈ 28.4 → Tex ≈ 20.8.
- 150 denier filament → Tex ≈ 16.7 → Nm ≈ 60.

**Rule of thumb for the team:** a *finer* yarn (high Ne/Nm, low Tex/denier) → finer achievable gauge,
lighter garment, softer hand, more yardage per kg (so more knitting time per kg). A *coarser* yarn → bulky
low-gauge sweater. Count and gauge (§4) must be matched.

---

## 3. Ply / folded yarn

- **Singles (1-ply)** — One continuous strand straight off the spinning frame; cheaper, can be weaker, can
  skew/torque a flat-knit panel because of unbalanced twist. *Why it matters:* fine-gauge fashion knits
  sometimes run singles for lightness, but balance/skew is a QC risk.
- **Ply / folded yarn** — Two or more singles twisted together; balances twist, boosts strength, evenness,
  and roundness. *Why it matters:* most knitwear yarn is 2-ply (folded) for durability and clean stitch
  definition.
- **Folded-count notation (e.g. 2/30, 2/48 Nm)** — "2/30" = two singles of 30s plied together. **Resultant
  count of a 2-fold yarn ≈ the singles count ÷ number of plies**, so 2/30 ≈ a 15s-equivalent thickness
  (two 30s ends side by side). Read order varies by region (worsted often writes count/ply, e.g. 30/2);
  always confirm which number is the singles count. *Why it matters:* the gauge a mill can knit is driven
  by the **resultant** thickness, not the singles number; misreading 2/30 vs 1/30 is a doubling error in
  fabric weight. The "ends" a knitter feeds (e.g. "3 ends of 2/48") further multiply the effective
  thickness — the standard lever for hitting a target gauge with whatever count is in stock.
- **Twist (TPI / TPM, twist per inch/metre) and twist direction (S vs Z)** — Amount and handedness of
  twist; more twist = stronger, leaner, lower-pill, but harsher hand; balanced ply pairs opposing twists.
  *Why it matters:* a twist/pilling/softness trade-off the spinner controls; under-twisted soft yarn
  pills, over-twisted yarn feels wiry and can bias the fabric.

---

## 4. Knitting GAUGE / GG (machine gauge)

- **GG (gauge)** — Number of needles per inch on the machine bed; the master variable for fabric fineness.
  Higher GG = more, finer needles → finer, denser, lighter fabric needing finer yarn; lower GG = fewer,
  thicker needles → bulkier, heavier fabric from thicker yarn. `[FACT — definition; Wikipedia "Gauge
  (knitting)", diamondknitland.com]` *Why it matters:* GG must match yarn count (§2) and dictates machine
  selection, knit time per garment, and the look/weight/price tier of the product.

Practical GG → product map (industry convention; garment weights are order-of-magnitude `[ESTIMATE:
industry-lore, medium-high confidence]`):

- **3GG** — Very coarse / "chunky." Thick, heavy, highly insulating winter sweaters; bold cables, bulky
  yarn (low Nm, e.g. ~1/4–2/14 Nm class). Garment ~700 g–1 kg+. *Matters:* high yarn cost per piece but
  fewer pieces sold off-season; signals "statement winter knit."
- **5GG** — Coarse. Chunky-to-medium winter sweaters, still bulky. ~500–800 g.
- **7GG** — Medium; **the commercial default.** Standard-weight pullovers/cardigans sold in most retail —
  "7gg" on a label = ordinary sweater. ~350–550 g. *Matters:* this is the volume sweet spot and likely the
  default for a mainstream D2C piece. `[FACT — mysweaterfactory.com; ritacashmere.com]`
- **9–10GG** — Medium-fine; lighter pullovers, transitional-season knits. ~280–400 g.
- **12GG** — Fine; lightweight fine-gauge everyday knitwear — fine merino, polo necks, summer-weight,
  layering. ~200–350 g. *Matters:* the right tier for the warm California market and "elevated basics."
- **14GG / 16GG / 18GG** — Very fine, near-jersey; fine-merino, silk-blend, T-shirt-like knits. ~150–280 g.
  *Matters:* premium fine-gauge look but needs fine high-count yarn and slower knitting (more cost/piece).

Rough yarn-to-gauge pairing rule `[ESTIMATE: industry-lore, medium confidence]`: convert the resultant
yarn count to Nm; a finer gauge wants a higher Nm (finer) yarn, and the knitter tunes the **number of ends
fed** to fill the needle. E.g. fine 12–14GG often runs higher-count fine yarns; coarse 3–5GG runs heavy
low-count yarn or many ends.

Related machine-fineness notes:
- **E (E-gauge)** — Same concept expressed as needles per inch with an "E" (E7 = 7GG); common on European
  (Stoll) machines. *Matters:* spec sheets mix "7GG" and "E7" — same thing.
- **3.5.7 / multi-gauge ("transfer/needle-out" coarse looks on a finer bed)** — A finer machine knitting
  with selected needles out to mimic a coarser gauge. *Matters:* lets one machine fake several gauges,
  affecting make-vs-buy and machine utilisation.

---

## 5. Knitting families (machine technologies)

- **Flat-bed (V-bed) knitting** — Two opposed needle beds in a "V"; knits flat panels back and forth,
  can transfer stitches → ribs, shaping, structure. Computerised flat machines = **Shima Seiki** (Japan),
  **Stoll** (Germany, now KARL MAYER STOLL). *Why it matters:* the core technology of sweater making and
  of **fully-fashioned** production (§6); the machine class our whole production chain centres on.
- **Fully-fashioned / fashioning** — Flat-bed capability to **increase/decrease stitches** so each panel is
  knit to final shape with shaped selvedges (fashioning marks), not cut. *Why it matters:* the quality and
  yarn-efficiency hallmark of better sweaters (see §6).
- **Whole-garment / integral knitting (WholeGarment® Shima, knit&wear® Stoll)** — A flat machine knits the
  **entire seamless garment in one piece** — no panels, no linking. *Why it matters:* near-zero cut waste
  and no/low labour for seaming, but expensive machines, slower cycle, and a steep programming skill — a
  strategic make-decision and a candidate "AI/automation" frontier for the venture.
- **Circular knitting** — Cylinder of needles knits a continuous **tube of jersey-type fabric** at high
  speed; output is yardage (later cut & sewn), not shaped panels. *Why it matters:* the basis of cut-&-sew
  knit (sweatshirts, fine jersey); high throughput, low per-metre cost, but no fashioning → more cut waste
  and a different (often more casual) product.
- **Warp knitting (tricot/raschel)** — Many parallel yarns looped **along** the fabric length; fast, stable,
  runs lace/mesh/technical fabrics. *Why it matters:* mostly outside classic sweaters (trims, technical,
  lace panels); know it exists so it isn't confused with weft/flat knitting.
- **(Hand-frame / domestic flat knitting)** — Manual or semi-auto flat machines. *Why it matters:* small
  artisanal capacity and sampling; relevant to a tiny-line make-vs-buy and prototyping.

Orientation note: flat-bed, circular, and warp differ in **how the loop is formed and the fabric grows**.
Flat & circular are **weft** knitting (one yarn forms a course across); warp uses many yarns down the
wales. Sweaters = overwhelmingly weft/flat.

---

## 6. Construction: fully-fashioned vs cut-&-sew

- **Fully-fashioned (FF)** — Each panel (front, back, sleeves) is **knit to its final shape** on a flat-bed
  with stitch increases/decreases; panels are then **linked** (§7) edge-to-edge. *Why it matters:* premium
  construction — minimal yarn waste, clean shaped edges, no raw cut edges to fray, better drape and fit;
  the word "fully-fashioned" is a quality and price justifier on the brand side.
- **Cut-&-sew (C&S)** — Knit a flat/circular **blank of fabric**, then **cut** garment shapes out and
  **sew** them with overlock/coverstitch like woven garments. *Why it matters:* faster and cheaper for
  fine jersey and high volume, but **more material waste**, cut edges that can curl/fray, and a more casual
  result; the cost-vs-quality counterpoint to FF in every make decision.
- **Integral / seamless (recap)** — Whole-garment knitting (§5) eliminates both cutting and most seaming.
  *Why it matters:* the third option that changes the labour and waste maths entirely.
- **Fashioning marks** — The small diagonal stitch marks left where a panel was shaped (decreased). *Why it
  matters:* a visible authenticity cue that a sweater is genuinely fully-fashioned, not cut.

---

## 7. Seaming / joining (linking, looping, mock-linking)

- **Linking** — Joining two knitted edges **loop-to-loop** on a **linking machine** (a circular dial of
  points, often "X-gauge" cups/points), producing a flat, near-invisible, stretchy seam aligned stitch-for-
  stitch. *Why it matters:* the defining finishing step of quality fully-fashioned sweaters; "fully linked"
  / "hand-linked" is a quality claim, and linking is a skilled, labour-intensive, throughput-limiting
  operation (a real cost and capacity bottleneck). `[ESTIMATE: industry-lore, high confidence]`
- **Looping** — Synonym/relative of linking for attaching trims (collars, plackets) loop-to-loop; "looping"
  and "linking" are often used interchangeably by region. *Why it matters:* know that one operation/term
  family covers loop-faithful joins.
- **Mock-linking (mock linking)** — A linking-machine or overlock seam that **imitates** a true linked look
  without full loop-to-loop registration; faster/cheaper, slightly less clean. *Why it matters:* a common
  cost-down that buyers/QC must distinguish from true linking when auditing quality claims.
- **Overlock / cup-seam / coverstitch** — Standard sewn seams used in cut-&-sew and on lower-cost knits;
  bulkier and less stretchy than linking. *Why it matters:* the cheaper seaming class; presence signals
  C&S or value construction.

---

## 8. Stitch types / knit structures

- **Jersey / plain / single knit (stockinette)** — Basic single-face knit; smooth "V" face, looped back;
  light, curls at edges, can ladder/run. *Why it matters:* the lightweight base structure (fine-gauge tops,
  cut-&-sew bodies); cheapest yarn-wise but needs edge finishing.
- **Rib — 1×1 and 2×2** — Alternating face/back wales giving vertical ridges and strong widthwise stretch &
  recovery; 1×1 (finer) and 2×2 (broader, bouncier). *Why it matters:* the standard for cuffs, hems,
  collars (holds shape, hugs body); rib quality = fit and recovery, a key QC point.
- **Links-links (purl) / links-and-links** — Alternates knit and purl **along the wale** (rows of face and
  back), giving a reversible, textured, lengthwise-stretchy fabric; needs a purl/links-capable machine.
  *Why it matters:* texture and reversibility for design; flags a machine-capability requirement.
- **Cable** — Wales crossed over each other (stitch transfer) to make rope-like raised twists. *Why it
  matters:* classic heavy-knit design value (esp. 3–7GG), but uses more yarn and machine time → cost driver.
- **Intarsia** — Blocks of different-coloured yarn knit in defined areas with **no floats** on the back
  (each colour worked only in its zone). *Why it matters:* clean colour-blocking/logos with no yarn waste on
  the reverse, but slower/skilled and machine-dependent — a design-vs-cost lever.
- **Jacquard** — Multi-colour pattern where unused colours **float/are carried** across the back (single or
  bird's-eye/backed jacquard to control floats). *Why it matters:* rich all-over patterns/Fair-Isle looks;
  heavier (extra yarn carried) and back-float management is a quality issue.
- **Pointelle** — Decorative open/eyelet holes made by stitch transfer; lacy, lightweight. *Why it matters:*
  feminine/spring fine-gauge styling — relevant for warm-climate (California) lighter knits.
- **Tuck & miss (float) stitches** — Held (tucked) or skipped (missed) loops creating texture/colour
  effects; the building blocks of many of the above. *Why it matters:* texture without extra yarn colours;
  affects width, weight, and stretch.
- **Plating** — Two yarns fed together so one shows on the face, another on the back (e.g. soft face,
  strong back). *Why it matters:* a way to get a luxe face cheaply or add stretch (elastane plated in).

---

## 9. Fabric weight & finishing

**Weight**

- **GSM (grams per square metre)** — Mass of one square metre of the fabric; the universal fabric-weight
  metric. *Why it matters:* the objective spec for "how heavy/warm" a knit is, set on the tech pack and
  checked in QC; pairs with GG and yarn count to define the product. For shaped sweaters, weight is often
  also tracked as **grams per garment** at a given size. `[ESTIMATE: industry-lore, high confidence]`
- **Stitch density (courses/wales per inch or cm)** — How many loops per unit length/width; tighter density
  = heavier, more stable, lower-pill fabric. *Why it matters:* a QC dial that, with GSM, catches a too-loose
  (cheap, saggy) or too-tight (stiff, yarn-hungry) knit.
- **Loop/stitch length (yarn consumed per stitch)** — Length of yarn in one loop; the deepest control of
  weight, tightness, and yarn cost. *Why it matters:* the parameter the programmer tunes to hit target GSM
  and yarn cost simultaneously.

**Finishing terms**

- **Milling / fulling** — Controlled wet/heat/agitation that felts and **closes up** a wool knit for a
  fuller, softer, warmer, more cohesive hand. *Why it matters:* turns an open raw-knit into a finished
  premium hand; over-milling shrinks/felts irreversibly → a QC risk.
- **Anti-pill finish** — Chemical/enzyme (e.g. cellulase bio-polishing on cellulosics) or mechanical
  treatments, plus low-pill fibre/twist choices, to reduce pilling. *Why it matters:* directly attacks the
  top return driver; an "anti-pill" claim must survive wash-test grading (see §10 pilling test).
- **Mercerised (mercerisation)** — Caustic-soda treatment of cotton under tension giving lustre, strength,
  better dye uptake. *Why it matters:* upgrades cotton knitwear to a sheeny, premium "mercerised cotton"
  positioning — relevant for the warm-market cotton line.
- **Sanforised / compacted (shrink control)** — Mechanical pre-shrinking/compacting so the finished garment
  meets a residual-shrinkage limit. *Why it matters:* keeps the garment within size tolerance after the
  customer washes it — a returns and care-label compliance issue (term is woven-origin but the
  shrink-control concept applies to knits via compacting/relaxation).
- **Scouring / washing / softening** — Removing spinning oils/dirt and adding softeners/silicone for hand.
  *Why it matters:* raw-off-machine knits feel harsh; finishing is what makes the sample feel like the
  retail product.
- **Steaming / pressing / blocking / Kier-relax** — Heat-set the knit to final dimensions and relax stresses
  so panels/garments hold shape and measurements. *Why it matters:* final dimensional accuracy and the
  "finished" look; skipped/rushed pressing shows up as measurement and appearance defects.
- **Dyeing stages** — **Fibre/stock-dyed**, **yarn/package-dyed**, **piece/garment-dyed**; later-stage
  dyeing = faster response to trend but more colour-consistency risk. *Why it matters:* a lead-time vs
  colour-fastness trade-off and a key colour-QC point (lot-to-lot shade matching).

---

## 10. QC / yarn evenness & defect terms

Yarn quality is graded against the **USTER® STATISTICS** benchmark percentiles (e.g. "25% USTER" = better
than 75% of world production). Lower irregularity = cleaner knitting, fewer stops, fewer fabric defects.

- **Yarn evenness / irregularity** — How uniform the yarn's mass is along its length; measured optically or
  capacitively on an evenness tester. *Why it matters:* uneven yarn → streaky fabric, knitting stops, and
  visible defects → scrap and downtime, a direct cost.
- **CV% (Coefficient of Variation of mass)** — The standard statistical measure of mass irregularity (std
  dev / mean of mass per unit length, as a %); the headline evenness number (also written CVm). Lower =
  more even. `[FACT — textilesbar.com; standard textile metrology]` *Why it matters:* the single number
  buyers anchor on when grading yarn lots; ties directly to fabric appearance.
- **U% (unevenness percentage)** — Older mean-deviation measure of the same irregularity; CV% ≈ 1.25 × U%
  for a normal distribution. *Why it matters:* legacy specs quote U%; convert to compare with CV%.
- **IPI (Imperfection Index)** — Sum of **thin places + thick places + neps per 1000 m** at standard
  sensitivities; the count of point defects. *Why it matters:* the headline "cleanliness" number alongside
  CV%; high IPI predicts visible fabric faults and knitting stops. `[FACT — textilelearner.net;
  iejrd.com]`
- **Thin place** — A spot where yarn cross-section drops ≥ ~50% below the mean (standard −50% sensitivity).
  *Why it matters:* weak point that can break in knitting and shows as a light streak.
- **Thick place** — A spot ≥ ~50% above the mean (+50%). *Why it matters:* shows as a heavy/dark fault in
  fabric.
- **Nep** — A small tightly tangled knot of fibre, ≥ ~200% of mean cross-section (+200%). *Why it matters:*
  visible specks in the knit, worse with short/immature fibre and rough processing — a common reject cause.
- **Hairiness (H / S3)** — Amount of protruding fibre ends on the yarn surface. *Why it matters:* affects
  pilling, hand, and shed/lint; too high → pilling and dusty knitting, too low → flat/harsh look.
- **Tenacity / elongation (single-yarn strength)** — Breaking force (cN/tex) and stretch at break. *Why it
  matters:* low strength → yarn breaks in knitting (downtime) and weak seams; a core acceptance test.
- **Count CV / count variation** — Lot-to-lot and within-lot variation of the yarn count itself. *Why it
  matters:* count drift shifts GSM and shade between batches — a consistency defect across production runs.
- **USTER STATISTICS percentile (e.g. 5%/25%/50% USTER)** — Where a yarn sits versus global production for
  each parameter. *Why it matters:* the common language for specifying yarn grade in a PO ("must meet 25%
  USTER on CV% and IPI"). `[FACT — Uster Technologies methodology]`
- **Pilling grade (1–5, e.g. Martindale / ICI box / random tumble)** — Lab wear-test rating of pill
  formation (5 = best/no pill, 1 = severe). *Why it matters:* the objective test behind any "anti-pill"
  claim; a sourcing acceptance gate and the link from fibre/finish choices to real-world returns.
- **Fabric inspection / 4-point system** — Standard for grading finished-fabric defects per 100 yd². *Why
  it matters:* the cut-&-sew-side analogue of yarn QC; sets accept/reject on fabric lots.
- **Bursting strength / dimensional stability (shrinkage %) / spirality** — Knit-fabric tests: pressure to
  rupture; size change after wash; twisting of a tube/garment (often from unbalanced singles twist). *Why
  it matters:* these are the finished-garment acceptance tests that govern returns, sizing complaints, and
  the "twisted side seam" defect.

---

## Cross-team usage rules (binding for downstream specialists)

1. **Indirect vs direct counts:** never compare a Ne/Nm number with a Tex/denier number without converting;
   remember higher Ne/Nm = finer, higher Tex/denier = thicker.
2. **Pivot through Tex** for any count conversion; carry **Nm = 1.693 × Ne** for the Ne↔Nm hop.
3. **Folded notation 2/30** means two 30s singles ≈ resultant 15s — always confirm singles count and ends
   fed before judging fabric weight.
4. **GG must match yarn count**; quote both when specifying a fabric, plus target **GSM / g-per-garment**.
5. **Fully-fashioned + linked** ≠ **cut-&-sew + overlocked** — keep construction and seaming terms precise,
   they carry the quality/price story.
6. **Grade yarn against USTER percentiles on CV% + IPI**, and **grade pilling 1–5 by wash test** — those are
   the numbers a PO and a QC reject decision actually turn on.

---

## Sources (web-grounded items)

- Gauge (knitting), Wikipedia — definition of GG as needles per inch.
- diamondknitland.com; ritacashmere.com; mysweaterfactory.com — GG→garment/weight conventions (3/5/7/12GG).
- maruyasu-fil.com (KNIT MAGAZINE) — yarn-count-to-gauge guidance (folded counts → gauge).
- testextextile.com; textileschool.com; textilesbar.com — Ne/Nm/Tex/denier definitions & conversions.
- textilelearner.net; iejrd.com; textilesbar.com — IPI, CV%/U%, thin/thick/nep sensitivities; USTER.
- Uster Technologies (USTER STATISTICS) methodology — percentile grading concept.

Conversion constants (Tex = 1000/Nm = 590.5/Ne; denier = 9×Tex; Nm = 1.693×Ne) are dimensional identities,
stated directly rather than tagged. Garment-weight-by-gauge figures are `[ESTIMATE: industry-lore]`
order-of-magnitude bands, not vendor quotes.
