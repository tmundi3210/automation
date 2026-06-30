# Yarn — Analyst B: Technical-Quality (T2)

**Role:** yarn technical-quality analyst. **Question this answers:** *What makes sweater yarn good or bad, how do those determinants show up in the finished garment, and how do we spec / grade / test it so a Ludhiana supplier can actually meet the bar?*

**For:** the owner of a small, AI-designed D2C knitwear brand. Default fibre case = **acrylic + acrylic-blend** (Ludhiana's reality per T1), with merino/cotton lines as premium/warm-market options. Connects upstream to T1 (Ludhiana cluster), downstream to **T5 (the "thread-checker" QC device)** and **T6 (finishing)**.

**Honesty tags (binding, travel downstream):** **[FACT]** = cited public source. **[ESTIMATE]** = method + basis + confidence + literal word *estimate*. **[UNKNOWN]** = genuinely not known, not faked. Price/MOQ/lead-time bands are **[ESTIMATE]** unless a live quote is cited. Untagged statements are dimensional identities or definitional (true by definition).

> **The one sentence to remember.** *Sweater-yarn quality is decided at the spinning frame by five dials — staple length, twist, ply, fibre fineness, and blend ratio — and 80% of what the customer feels (softness, warmth, strength, drape) and 100% of the #1 return driver (pilling) trace back to those dials plus the gauge it's knit at; so the only quality lever a buy-yarn brand truly controls is **writing a count-and-USTER-graded spec and enforcing it with a wash-tested pilling gate.***

---

## 1. The five quality determinants — and how each shows up in the finished sweater

Yarn quality is not one number. It is five mostly-independent dials set at spinning, each mapping to a specific thing the customer feels. Get the mapping wrong and you "fix" the wrong dial.

| Dial | What it is | Set higher → | Shows up in the sweater as |
|---|---|---|---|
| **Staple length** | length of one fibre (mm), or "continuous filament" for synthetics | longer = fewer fibre ends per length | **less pilling, more strength, smoother hand** (the cheapest pilling predictor) |
| **Twist (TPI / TPM, twist multiplier K)** | turns of twist per unit length; K normalises twist to count | more twist = tighter fibre bond | **stronger, leaner, lower-pill, but harsher/wirier**; too little = weak + pills |
| **Ply / folding** | number of singles twisted together (2-ply, "2/30") | more plies = balanced, rounder | **strength, evenness, clean stitch definition, no skew/spirality** |
| **Fibre fineness (micron / denier-per-filament)** | diameter of one fibre | finer = softer | **softer next-to-skin hand, better drape — but *more* pilling and lower strength** |
| **Blend ratio** | % of each fibre spun together | — (a trade, not a "higher") | **the cost/hand/performance/care-label/duty position of the whole garment** |

### 1a. Staple length
- **Longer staple → fewer free fibre ends on the yarn surface → less shedding, less pilling, higher strength, smoother hand.** [FACT — ScienceDirect "Pilling Tendency": short staple is listed among factors that *increase* pilling tendency.] For synthetic staple this is a direct spec lever: specifying **38 mm or 51 mm cut length instead of 32 mm** "dramatically reduces the number of fibre ends available to shed and form pills." [FACT — neolianda.com polyester pilling guide; the same physics applies to acrylic staple.]
- **In the sweater:** short-staple yarn (or short, immature cotton; recycled/shoddy wool from Panipat — see T1 §6) is the first thing that pills and the first thing that breaks during knitting (downtime). Continuous-**filament** synthetics essentially don't pill from shed ends because there are no staple ends — but staple acrylic, the Ludhiana default, does.
- **Spec implication:** for staple fibres, **name a minimum staple length** (or fibre grade) in the PO, not just the count. This is also a cheap pre-wear QC proxy (SCOPE_GLOSSARY §1). `[ESTIMATE: industry-lore, high confidence]`

### 1b. Twist (TPI/TPM and twist multiplier)
- **Twist creates a strength-vs-softness trade-off with an optimum.** As twist rises, fibre-to-fibre friction binds fibres tighter → tensile strength climbs **up to an optimal twist multiplier, then falls** (over-twist makes the yarn snarl/torque and actually weakens it). [FACT — meeraind.com; glyarn.com; textile twist-multiplier theory: "maximum strength of a yarn is obtained for a definite value of K".]
- **Twist multiplier K** normalises twist to count so the *same* character is held across thicknesses (TPI alone is meaningless without the count). Practical bands: **K ≈ 3.0 → soft, pliable hand** (knitwear-typical); **K ≈ 6.0 → hard, stiff, "twist-lively" yarn** (weaving/sewing-thread territory). [FACT — textile twist-factor practice, glyarn.com.] Wool *woollen-spun* singles for loft run very low, **~7 TPI**, deliberately, to keep air/loft. [FACT — twist-mechanics sources.]
- **In the sweater:** **under-twisted yarn pills and breaks; over-twisted yarn feels wiry, biases the fabric, and causes spirality** ("twisted side-seam" defect — SCOPE_GLOSSARY §5). Tighter twist is a primary *anti-pill* lever: "tighter twist improves resistance to surface abrasion and reduces pilling, as fibres are less likely to work loose." [FACT — textile twist sources.] But you cannot just crank twist — softness is the whole point of a sweater, so twist is balanced *against* hand, not maximised.
- **Spec implication:** specify **TPM (or TPI) and direction (S/Z)**, or equivalently the **twist multiplier**, and require **balanced ply** (opposing twists) to kill spirality. This is the dial most often left blank in a naive PO. `[ESTIMATE: industry-lore, high confidence]`

### 1c. Ply / folding
- **Folding two singles with the opposing twist balances torque, rounds the yarn, raises strength and evenness, and sharpens stitch definition.** Most durable knitwear yarn is **2-ply** for exactly this. [FACT — folding theory; SCOPE_GLOSSARY §2.]
- **Folded-count notation is a trap that drives fabric-weight errors:** "**2/30**" = two 30s singles plied, **resultant ≈ singles ÷ plies ≈ 15s-equivalent** thickness; misreading **2/30 as 1/30 doubles the implied fabric weight.** And the *ends fed* into the knitting machine further multiply effective thickness. (SCOPE_GLOSSARY §2 + cross-team rule 3.) Untagged — dimensional.
- **In the sweater:** singles (1-ply) are lighter and cheaper and used for fine fashion knits, **but can skew/torque a flat panel** (a QC reject) and pill more. Ply = durability + clean wales + no spirality.
- **Spec implication:** **always confirm singles count, ply, and ends fed before judging the fabric weight** — a 2/30 vs 1/30 misread is a doubling error.

### 1d. Fibre fineness (micron / denier-per-filament) — the counter-intuitive dial
- **Finer = softer, but finer also pills MORE and is weaker.** Finer fibre means more fibre ends exposed per yarn cross-section and softer, more-mobile fibres that "tangle more easily into balls." [FACT — neolianda.com / ScienceDirect pilling.] This is the central tension of premium knitwear: **the softness you pay for (low micron) is the same property that makes the garment pill** — which is why cashmere (14–19 µm) pills *more* than coarser wool (SCOPE_GLOSSARY §1) and must be defended with twist, ply, blend, and finishing.
- **Animal-fibre micron → comfort/price map** [FACT — Woolmark; merino-grade sources]:
  - **<22 µm = the "itch threshold"**: below ~22 µm fibres bend rather than poke, so they read as soft; above ~25 µm most people feel prickle. Generic/coarse wool 28–32 µm prickles.
  - **Merino ~17–24 µm**; **superfine 17.5–18.5 µm**; each **−1 µm ≈ +15–20% perceived softness** [FACT — merino-grade sources, vendor-stated] and a real price step.
  - **Cashmere ~14–19 µm** (luxury anchor); **lambswool** = first-shearing grade, softer than later clips.
- **Synthetic analogue = denier-per-filament (dpf).** Lower dpf = softer/more delicate; higher dpf = stronger/more durable + lower pill. Acrylic upgrades are sold exactly this way: moving from **1.0 dpf to ~2.5 dpf anti-pilling acrylic** is reported to cut customer complaints 30–50% / return rates ~45%. [FACT — estakoyarns.com, szoneierfabrics, vendor-reported; treat the % as vendor case-study, directionally high-confidence, magnitude **[ESTIMATE: vendor-sourced, medium]**.]
- **In the sweater:** fineness sets next-to-skin comfort and drape — and the brand's whole price tier — but trades *against* durability and pilling.
- **Spec implication:** for the **warm California market** a finer, softer hand sells; for acrylic, **specify an anti-pill / higher-dpf grade** rather than the cheapest 1D acrylic. Don't say "merino" — say a **micron ceiling** (e.g. "≤19.5 µm"); "merino" is marketing, micron is the spec. `[ESTIMATE: judgment, high confidence]`

### 1e. Blend ratio — the margin/positioning dial the brand controls
- **The ratio on the care label is the single biggest cost-and-positioning decision** (SCOPE_GLOSSARY §1). "70/30" vs "50/50" moves cost, hand, warmth, care, **and US import duty** (fibre changes the HTS line — wool 6110.11 ~16% ≈ cotton 6110.20.20 ~16.5%, while acrylic/MMF 6110.30.30 ~32% is the outlier; SCOPE_GLOSSARY §6).
- **The highest-leverage blend trick: a few % nylon (polyamide).** Adding **a small % nylon** to a cashmere/lambswool/merino yarn "sharply improves pill + abrasion resistance" — a standard, cheap quality lever (e.g. classic 90/10 or 95/5 wool-or-cashmere/nylon). [FACT — SCOPE_GLOSSARY §1; widely-used industry construction.] This is the cleanest way to keep a soft (= pilling-prone) fibre while taming its #1 weakness.
- **In the sweater:** the blend decides cost, perceived value (even 5–10% cashmere lifts a label), warmth-to-weight, care label, and pilling defence simultaneously.
- **Spec implication:** treat blend ratio as a margin lever set by the brand and **written into the PO with a tolerance** (e.g. "70% acrylic / 30% wool ± 3% by mass, verified by fibre-composition test"), because blend drift is both a cost cheat and a duty-misdeclaration risk.

**Determinant → garment-property summary (what the customer actually feels):**
- **Hand (softness):** fineness (↑ fine = ↑ soft) + twist (↑ twist = ↓ soft) + finishing.
- **Warmth:** loft/bulk (low twist, high-bulk acrylic, milled wool) + fibre (wool/acrylic > cotton warmth-to-weight) + blend.
- **Strength / durability:** staple length + twist (to optimum) + ply.
- **Drape:** fineness + low-ish twist + fibre (viscose/modal drape; cotton bags out, low recovery).
- **Pilling (the failure mode):** *all five dials at once* → §2.

---

## 2. PILLING — the #1 failure mode, and how yarn + finishing control it

**Why it dominates:** pilling is the **#1 post-purchase complaint and return driver** for knitwear (SCOPE_GLOSSARY §1). A pill is a tangled ball of broken/loose surface fibres that (a) work loose, (b) tangle, (c) stay anchored by a few un-broken fibres instead of shedding. Control any of those three and you control pilling.

### 2a. What causes it (the mnemonic: SHORT, FINE, LOW-TWIST, LOOSE)
Factors that **increase** pilling tendency [FACT — ScienceDirect "Pilling Tendency"; neolianda.com]:
- **Short staple length** (more free fibre ends to shed).
- **Fine fibre / low denier** (more ends per cross-section, softer fibres tangle easily) — the cruel irony that *softness causes pilling*.
- **Low yarn twist** (fibres not bound, work loose easily).
- **Loose / open yarn and fabric construction; coarse gauge** (low stitch density gives fibres room to migrate and ball).
- **High yarn hairiness** (protruding ends are pre-positioned to pill).
- **Open-end / rotor spun** > ring spun for pilling; **round fibre cross-section**; long wet-processing; napping.

Factors that **reduce** pilling [FACT — same sources]: **longer staple, higher twist, ring/compact/air-jet spun, fibre crimp, singeing, heat-setting, steaming, shearing, anti-pill chemical/enzyme finish.** **Compact / air-jet spinning can cut fibre shedding by up to ~30%** vs conventional ring spinning [FACT — neolianda.com, vendor-stated; magnitude **[ESTIMATE: vendor-sourced, medium]**].

### 2b. The yarn-choice levers (designed out *before* a stitch is knit)
1. **Longer staple** (or filament where the look allows).
2. **Higher twist / higher twist multiplier** — within the hand budget.
3. **Ply** (2-ply binds better than singles).
4. **Anti-pill / higher-dpf fibre grade** — for acrylic this is the single biggest lever (the "2.5D anti-pilling acrylic" upgrade, §1d).
5. **A few % nylon in the blend** for soft fibres (§1e).
6. **Lower hairiness** (a measurable USTER parameter, §3) — compact-spun yarn is lower-hairiness.

### 2c. The construction lever (T3/T8 own this, flagged here)
- **Tighter gauge + higher stitch density (more courses/wales per inch, shorter loop length) → fewer pills**, because fibres have less room to migrate and ball. A **12GG fine knit** (the right tier for warm-California "elevated basics" per SCOPE_GLOSSARY §3) is inherently lower-pilling than a loose 5GG chunky in the same yarn. Loose gauge is a pilling *cause*; the design team must not undercut a low-pill yarn with a slack knit.

### 2d. The finishing lever (→ T6)
Anti-pill finishing **does not replace** good yarn — it's the last line, and any "anti-pill" claim must survive the wash-test pilling grade (§3, §2e):
- **Singeing / gas singeing:** fabric/yarn passed quickly over an open flame to **burn off protruding surface fibres** before they become fuzz. [FACT — slideshare singeing/biopolishing.] Classic for cotton; relevant where the look tolerates it.
- **Cellulase enzyme "bio-polishing"** (cellulosics — cotton, viscose/modal; **not** acrylic/wool, which cellulase can't digest): enzymes **selectively hydrolyse the microfibrils/fuzz protruding from the yarn surface** so they snap off cleanly → smoother surface, **permanently** lower pilling tendency, softer hand, brighter colour. Typical knitwear parameters: **0.5–1.5% enzyme on fabric weight, pH 4.5–5.5, 45–55 °C, 30–45 min.** [FACT — cellulase.bio; shreebschemicals; researchgate enzyme-wash studies.] **Permanent**, unlike softeners that wash out. Trade-off: over-treatment **loses fabric weight/strength** (it's eating fibre) — a real QC limit.
- **Heat-setting / steaming / shearing:** stabilise and trim the surface (§ finishing, T6).
- **Mechanical softeners / silicone:** improve *hand* but **wash out** — do not durably fix pilling (don't let a soft off-machine feel be mistaken for an anti-pill result).

### 2e. The objective gate (the only thing that makes "anti-pill" real)
Pilling is graded **1–5** (5 = no change/no pill; 1 = very severe) by a standardised wear test [FACT — ISO 12945 family; begoodtex; testextextile]:
- **For knit sweaters the preferred method is the ICI Pilling Box (ISO 12945-1)** — random tumbling, no applied pressure; the Martindale figure-8 method (ISO 12945-2) is the woven-oriented method. [FACT — textiletrainer; testextextile.]
- **Acceptance bands [FACT — retail-QC convention, begoodtex/knitwear.io]:** most retail brands require **min grade 3–4**; **premium wool specifies grade 4+.** **[ESTIMATE: sourcing judgment, high confidence]** for our brand: **target ICI ≥ 3–4 after at least one home-wash cycle** (post-wash, because washing loosens fibres and *drops* the grade — the post-wash test is what predicts real returns).
- **This is the link from fibre/finish choices to real-world returns**, and the natural accept/reject gate the T5 checker concept plugs into (§6).

---

## 3. Yarn evenness & defect metrics — what a buyer can actually test or demand

Yarn is graded against **USTER® STATISTICS** percentiles — the global benchmark where **"25% USTER" = the yarn is better than 75% of world production** for that parameter. Lower irregularity → cleaner knitting, fewer machine stops, fewer fabric defects → directly lower scrap and downtime. [FACT — Uster Technologies methodology; SCOPE_GLOSSARY §5.]

| Metric | What it measures | Why a sweater buyer cares |
|---|---|---|
| **CV% (CVm)** | std-dev/mean of mass per unit length (%); the headline evenness number | uneven yarn → streaky/barré fabric, knitting stops; **the number buyers anchor on when grading a lot** [FACT — textilesbar; standard metrology] |
| **U%** | older mean-deviation evenness; **CV% ≈ 1.25 × U%** | convert legacy U% specs to CV% to compare (untagged identity) |
| **IPI (Imperfection Index)** | **thin(−50%) + thick(+50%) + neps(+200%) per 1000 m** at standard sensitivities | the point-defect "cleanliness" number alongside CV%; **high IPI predicts visible faults + knitting stops** [FACT — textilelearner; sarpublication PDF] |
| **Thin place** | cross-section ≥ ~50% below mean | weak point → breaks in knitting; shows as a **light streak** |
| **Thick place** | ≥ ~50% above mean | shows as a **heavy/dark fault** |
| **Nep** | tight fibre knot ≥ ~200% of mean (≥~200 µm) | **visible specks**, worse with short/immature fibre — a common reject cause |
| **Hairiness (H / S3)** | protruding fibre ends on the surface | **too high → pilling + dusty/linty knitting; too low → flat/harsh look** — directly ties to §2 |
| **Tenacity / elongation** | breaking force (cN/tex) + stretch at break | **low strength → breaks in knitting (downtime) + weak seams** — a core acceptance test |
| **Count CV / count variation** | lot-to-lot + within-lot variation of the count itself | **count drift shifts GSM + shade between batches** — the cross-run consistency defect that ruins a re-order |

**Sensitivities are standardised** (thin −50%, thick +50%, nep +200%), so "IPI" is comparable across labs *if the sensitivity settings are stated* — always state them in the PO. [FACT — Uster standard settings.]

**What a buyer can actually do (escalating cost/leverage):**
1. **Demand the spinner's USTER test report** per lot (cheapest — they already test). Anchor on **CV%, IPI, hairiness, tenacity, count CV.**
2. **Write the PO in USTER-percentile language**: e.g. "must meet **25% USTER on CV% and IPI**" — the standard, enforceable PO phrasing. [FACT — Uster PO convention.]
3. **Independent lab test** a sample (third-party textile lab) when the relationship is new or stakes are high.
4. **Pilling wash-test (ICI, §2e)** on a knitted swatch — the test that actually predicts returns; the spinner's USTER report does *not* cover finished-garment pilling.
5. **Finished-knit tests** (T5/T6 boundary): GSM, stitch density, bursting strength, **dimensional stability/shrinkage %**, **spirality** (catches unbalanced singles twist), **fibre-composition test** (verifies blend ratio), **colour-fastness/shade match.**

**Honest limits for our scale [ESTIMATE: judgment, medium-high]:** a micro-brand will *not* own a USTER tester (a USTER Tester is a 6-figure-USD class instrument **[ESTIMATE: industry knowledge, medium — exact list price [UNKNOWN]]**). The realistic posture is **demand the report + percentile clause + third-party spot-check + the ICI pilling gate** — i.e. *contractual* QC, not in-house metrology. This is exactly the gap the **T5 "thread-checker"** idea targets (§6).

---

## 4. How to write a yarn spec / acceptance criteria a Ludhiana supplier can meet

A spec that a fragmented MSME cluster (T1: ~10–12k units, ~70% micro, relationship/cash-driven mandi) can actually hit is **count-anchored, USTER-graded, tolerance-bounded, and tied to named test methods** — not vibes. Template below; fill brackets per style.

**Yarn Spec Sheet (PO attachment) — fields:**
1. **Fibre & blend:** e.g. "70% acrylic (anti-pill grade, ≥2.5 dpf) / 30% combed wool (≤21 µm), **± 3% by mass**, verified by fibre-composition test." (Name micron ceiling, *not* "merino"; name dpf/anti-pill grade for acrylic.)
2. **Count:** in the system the mandi quotes — **Ne or Nm with folded notation**, e.g. "**2/30 Nm**" — *and* the Tex equivalent in brackets to stop cross-team errors (Tex = 1000/Nm = 590.5/Ne; Nm = 1.693×Ne). State **singles count, ply, and ends fed.**
3. **Twist:** **TPM (or TPI) + direction (S/Z)** or the **twist multiplier K**, and "**balanced ply**" to bound spirality.
4. **Staple length:** minimum staple (mm) or fibre grade — the pilling/strength floor.
5. **Evenness & defects (USTER-percentile clause):** "**≤ 25% USTER on CVm% and IPI**" + state sensitivities (thin −50/thick +50/nep +200); cap **hairiness** band; min **tenacity (cN/tex)** + elongation; **count CV** ceiling for re-order consistency. *Require the per-lot USTER report.*
6. **Pilling acceptance:** "knitted swatch in target gauge must reach **ICI Pilling Box (ISO 12945-1) grade ≥ 3–4 after 1 home-wash cycle.**" (The gate that makes any "anti-pill" claim real.)
7. **Colour:** shade vs approved standard, **lot-to-lot ΔE tolerance**, dye method (stock/package/garment — lead-time vs fastness trade, SCOPE_GLOSSARY §4); **colour-fastness** (wash/light/rub) minima.
8. **Gauge it must knit at:** state the target **GG and GSM/grams-per-garment** so count↔gauge match is contractual (cross-team rule 4).
9. **Sampling/acceptance plan:** **AQL** sampling on the lot; per-cone/per-lot USTER report; reject rule (e.g. "any lot failing the percentile clause or the pilling gate is rejected; re-test at supplier cost").
10. **Commercials (carry to T2 commercial analyst / T9):** Incoterm (year+place), MOQ per count/shade/lot **[ESTIMATE/UNKNOWN — T2 commercial]**, lead time, price basis.

**Can a Ludhiana supplier meet it?** [ESTIMATE: judgment, medium-high confidence]
- **USTER reporting + percentile clauses + count/twist/ply specs:** **yes** — the large integrated spinners (Vardhman, Nahar, Sportking; T1 §2) run lab QC and export-house processes and quote in Ne/Nm folded notation natively. A micro spinner may not self-test; then push to a USTER-equipped spinner or accept third-party testing.
- **The friction points** [ESTIMATE: industry-lore, medium]: (a) **MOQ** — a USTER-graded, anti-pill, specific-blend lot is a *made-to-order* run; small-batch iteration fights MOQ (the T1/T2 open question). (b) **Count/shade consistency across re-orders** (count CV) in a relationship-driven mandi. (c) **Honoring micron/dpf/anti-pill grade** rather than substituting cheaper fibre — which is exactly why the **fibre-composition test + pilling gate** must be contractual, not trust-based.
- **Tolerances must be realistic:** demanding **5% USTER** (top-tier export yarn) on a mainstream acrylic lot will either be unquotable or priced as premium. **Match the percentile to the price tier** — 25% USTER is a sane mainstream bar; 5% is luxury. `[ESTIMATE: industry-lore, medium-high]`

---

## 5. Yarn-stage treatments relevant to "best quality"

Treatments that happen *at the yarn/early stage* (garment finishing is T6; these are the ones that change the yarn's quality before/at knitting):

- **Singeing / gassing** (cotton, blends): pass over flame to burn off surface hairs → smoother, lower-hairiness, **lower-pilling**, more lustrous yarn. "Gassed" mercerised cotton is a recognised premium cotton grade. [FACT — singeing sources; common cotton-yarn practice.] Relevant to a cotton/warm-market line; **not** for acrylic (melts) or where loft/fuzz is the desired look.
- **Mercerising** (cotton only): treat under **tension** with concentrated **NaOH (~18–25%)** for <~4 min, then wash out → permanent **lustre, +tensile strength/elongation (∝ caustic concentration), and +20–40% dye uptake**; tension prevents the ~¼ shrinkage that would otherwise occur. [FACT — Wikipedia Mercerisation; textilesbar; sapub optimisation study.] This is the upgrade that turns plain cotton into premium **"mercerised cotton"** — directly relevant to the **warm California line** where cotton sells more months than heavy wool (SCOPE_GLOSSARY §1, §4). Can be done at yarn or fabric stage.
- **Steaming / conditioning / heat-setting:** relaxes spinning stresses and **sets twist**, reducing snarling/liveliness and **spirality** in the finished panel, and stabilising the yarn for clean knitting. Heat-setting also helps lock low-pill structure in synthetics. [FACT — heat-setting/steaming listed among pilling-reducers; twist-setting practice.] Relevant to controlling the "twisted side-seam" defect at source.
- **Scouring / oil removal:** removes spinning oils so dye and hand are true (more a wet-processing step; T6).
- **Compact / air-jet spinning** (a spinning *method*, not a post-treatment, but a quality choice at yarn): lower hairiness, **up to ~30% less shedding** → lower pilling and cleaner knitting. [FACT — neolianda.com, vendor magnitude **[ESTIMATE: medium]**.] Worth specifying ("compact-spun") where pilling is the priority and budget allows.

**Net:** for the **acrylic default**, the yarn-stage quality moves are **anti-pill/higher-dpf grade + longer staple + adequate twist + ply + (optionally) compact-spun + heat-set** — *not* mercerising/singeing (those are cotton tools). For a **cotton warm-market line**, **mercerising (+ gassing/singeing)** is the premium upgrade. For **wool/merino**, **micron ceiling + a few % nylon + twist + milling/finishing** (T6).

---

## 6. Connections to T5 (the "thread-checker") and T6 (finishing)

**To T5 — the QC / thread-checker device.** The owner's idea (MASTER_PLAN T5) is a device that *grades thread-to-thread, records it, and sends a code back to the supplier.* This analysis defines **what that device would have to grade and against what bar**:
- **The gradeable yarn parameters are exactly §3's USTER-class set:** **CV%/CVm, IPI (thin/thick/neps), hairiness, count CV, tenacity** — these are what "thread-to-thread quality" *means* technically, and they're what a percentile clause is written against.
- **Build-vs-buy reality for the checker** [ESTIMATE: judgment, medium — full feasibility is T5's job]: optical/capacitive evenness testing at full USTER fidelity is a heavy, expensive instrument; a *cheaper* checker plausibly targets the **visible/point-defect subset** (thick/thin/neps/hairiness via line-scan camera or photodiode) and the **pilling-grade** end via computer-vision on a wash-tested swatch — i.e. grade the things that drive *visible* faults and returns, not full metrology.
- **The "code back to the supplier" loop maps onto §4's percentile clause + AQL reject rule** — the device's output is precisely the accept/reject signal and the per-lot grade the spec already calls for. So T5 should be designed to **emit the USTER-percentile + pilling-grade verdict the PO demands**, closing the loop the micro-brand can't close with its own lab (§3 limits).

**To T6 — finishing.** Pilling control is **shared** between yarn (this file, §2b) and finishing (T6, §2d): **singeing, cellulase bio-polishing, heat-setting, shearing, anti-pill chemistry** are T6's levers, but they **only work on top of a sound yarn** and **must be validated by the same ICI pilling gate (§2e).** T6 also owns **mercerising at fabric stage, milling/fulling (wool), softening, sanforising/compacting (shrink control), and dyeing-stage choice** — several of which (mercerise, milling) are quality *upgrades* that this file specs the yarn to be *capable* of receiving. The hand-off rule: **yarn spec sets the floor (fibre/staple/twist/ply/evenness); finishing lifts hand and surface; the pilling grade + dimensional-stability test are the joint acceptance gates** both stages answer to.

---

## Bottom line for the owner

1. **Five dials decide everything:** staple length, twist (TPM/K), ply, fineness (micron/dpf), blend ratio — set at spinning, felt as hand/warmth/strength/drape, and *jointly* as pilling.
2. **Pilling is the war.** It's the #1 return driver, caused by **short + fine + low-twist + loose-gauge + hairy** yarn. For the **acrylic default**, the single biggest lever is **anti-pill / higher-dpf acrylic grade** (vendor-reported 30–50% fewer complaints); for **soft wool/cashmere**, **a few % nylon + twist + finishing**; never let a **loose gauge** (T3/T8) undo a low-pill yarn.
3. **Grade with numbers, not adjectives.** Anchor lots on **CV% + IPI + hairiness + tenacity + count CV** against **USTER percentiles** (mainstream = 25% USTER; luxury = 5%), and gate every "anti-pill" claim on **ICI Pilling Box grade ≥ 3–4, post-wash.**
4. **The spec is the only quality lever a buy-yarn brand controls.** Write a **count-anchored (Ne/Nm folded + Tex), twist-and-ply-stated, micron/dpf-named, USTER-percentile-claused, pilling-gated, fibre-composition-verified, AQL-sampled** spec — and make it *contractual*, because a fragmented mandi rewards substitution.
5. **Match the bar to the tier and the supplier.** Large integrated spinners can hit USTER clauses and quote folded counts natively; micro-spinners can't self-test (third-party or push to a USTER spinner). Don't demand 5% USTER on a mainstream lot — it's unquotable or luxury-priced.
6. **Yarn-stage treatments are fibre-specific:** acrylic → anti-pill grade + twist + heat-set + (compact-spun); cotton → **mercerise + gas-singe**; wool → micron + nylon + milling. **Don't apply cotton tools to acrylic.**
7. **The checker (T5) grades §3's parameters; finishing (T6) shares the pilling fight** — both answer to the **same pilling + dimensional-stability gates** this spec defines.

---

## Open questions (handed downstream)

1. **Actual MOQ / price / lead-time for a USTER-graded, anti-pill, specific-blend made-to-order lot in Ludhiana — [UNKNOWN]; → T2 commercial analyst / T9.** This determines whether spec-driven small-batch iteration is economically real.
2. **Real availability of compact-spun / anti-pill acrylic grades from Ludhiana spinners vs having to import the fibre — [UNKNOWN]; → T2.**
3. **Exact USTER-tester / lab-test cost and whether a third-party textile lab near Ludhiana is accessible to a micro-buyer — [UNKNOWN]; → T2/T5.**
4. **Feasibility/cost of a cheaper-than-USTER "thread-checker" that grades the point-defect + pilling subset — [UNKNOWN by design here]; → T5** (this file defines the parameters it must grade and the percentile/pilling bar it must emit).
5. **Whether the default fibre is acrylic, acrylic-blend, or a cotton/merino premium line — a brand/positioning decision (T7/T8) that flips which yarn-stage treatments (mercerise vs anti-pill grade) apply.**
6. **Quantified pilling-grade vs blend-ratio / twist / gauge curves for our exact constructions — [UNKNOWN until sampling]; needs PPS swatch testing (T8/T5).**

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/yarn/analyst_b_quality.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `FACTORY/sweater_vertical/analysis/SCOPE_GLOSSARY.md`; `FACTORY/sweater_vertical/analysis/ludhiana/DEEPDIVE.md`
- **Delivered (coverage = floor):** (1) five quality determinants — staple/twist/ply/fineness/blend — each mapped to hand/warmth/strength/drape, with the counter-intuitive "softness causes pilling" tension made explicit; (2) PILLING deep-dive — causes (short/fine/low-twist/loose/hairy), yarn-choice levers (anti-pill dpf, twist, ply, +nylon), construction lever (tight gauge), finishing levers (singe/cellulase/heat-set), and the **ICI ISO 12945-1 grade ≥3–4 post-wash** acceptance gate; (3) evenness/defect metrics — CV%/U%/IPI/thin/thick/nep/hairiness/tenacity/count-CV, USTER percentiles, and what a micro-buyer can actually test/demand (report + percentile clause + 3rd-party + pilling gate, *not* in-house metrology); (4) a fillable **yarn spec / acceptance template** (count-anchored, USTER-claused, pilling-gated, fibre-comp-verified, AQL) + a realistic read on whether a Ludhiana supplier can meet it (large spinners yes, micro no, match percentile to tier); (5) yarn-stage treatments — **singeing/gassing, mercerising (cotton, NaOH 18–25% under tension, +20–40% dye uptake), steaming/heat-set/conditioning, compact-spun** — flagged as fibre-specific; (6) explicit T5 (checker grades the §3 USTER set + emits the percentile/pilling verdict) and T6 (shared pilling fight, joint gates) connections.
- **Honesty discipline:** evenness/pilling/mercerise/twist physics tagged **[FACT]** with public sources; vendor-reported magnitudes (30–50% complaint drop, ~30% shed reduction, −1µm≈+15–20% softness) tagged **[ESTIMATE: vendor-sourced]** with confidence, never as hard fact; tester cost, MOQ/price/lead-time, fibre availability, and checker feasibility held at **[UNKNOWN]** and handed to T2/T5/T9; spec-meetability and tier-matching tagged **[ESTIMATE: judgment]** with confidence.
- **Bottom line:** quality is five spinning dials; pilling is the war; the only lever a buy-yarn brand owns is a numeric, USTER-graded, pilling-gated, contractual spec matched to price tier and supplier capability.

### Sources (anchored)
- Pilling causes/reducers: ScienceDirect "Pilling Tendency"; neolianda.com polyester-pilling guide; estakoyarns.com & szoneierfabrics acrylic anti-pill (vendor); cellulase.bio & shreebschemicals (bio-polishing); slideshare singeing/biopolishing.
- Twist/twist-multiplier: meeraind.com; glyarn.com; textile twist-factor practice (K≈3 soft / K≈6 hard; ~7 TPI woollen singles).
- Fineness/micron: Woolmark; merino-grade sources (≤22µm itch threshold, −1µm≈+15–20% softness; merino 17–24, superfine 17.5–18.5, cashmere 14–19).
- Evenness/USTER: textilesbar (USTER report, CV%); textilelearner & sarpublication (IPI = thin−50+thick+50+neps+200/1000m); Uster Technologies USTER STATISTICS percentile methodology.
- Pilling test: ISO 12945-1 (ICI box, knits) / 12945-2 (Martindale, wovens) — begoodtex, testextextile, textiletrainer, knitwear.io (retail min grade 3–4, premium wool 4+).
- Mercerising: Wikipedia Mercerisation; textilesbar; sapub mercerisation-optimisation study (NaOH 18–25% under tension; +20–40% dye uptake; strength ∝ caustic conc.).
