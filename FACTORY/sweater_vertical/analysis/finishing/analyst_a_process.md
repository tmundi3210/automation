# Finishing-Process Landscape — Analyst A (T6, process angle)

**Role.** Map the post-knit finishing steps for sweaters and what each *does* for hand / quality / care, then tie the anti-pill routes back to the **T2 yarn contract spec (ICI Pilling-Box grade ≥3–4 after a home wash)**. This is the process map; Analyst B takes the economics/risk angle and the reconciler merges.

**Honesty tags (binding, same discipline as T1/T2):**
- **[FACT]** = cited public source / true-by-definition process identity.
- **[ESTIMATE]** = reasoned, with method + basis + confidence + the literal word *estimate*. All cost/throughput/temperature/dose bands are `[ESTIMATE]`.
- **[UNKNOWN]** = genuinely not known; not faked. Coverage is a **floor, not a ceiling**.

> **The one sentence to remember.** *Finishing cannot rescue a bad knit, but it is where ~half a grade of ICI pilling, the entire retail "hand", and the customer's wash-shrinkage experience are actually delivered — and because the brand should never own the highest-risk wet node (dyeing, per T1), the binding question is which of these steps run **in-line at the job-work knitter/dyer** vs which the brand must **gate by contract + a post-wash test** exactly like the T2 yarn spec.*

The chain (logical order, not every garment uses every step):
**knit panels → linking/assembly → scour/wash → (wool only) mill/full → enzyme/anti-pill route → soften → extract/dry → steam-press / board / block → relax & shrink-control → final QC.** Dyeing is a *stage-of-the-chain choice* (fibre / yarn / piece / garment), discussed in §9, not a single step.

---

## 1. Scour / wash (the reset that makes a knit a product)

**What it is.** The first wet step after assembly. Removes **spinning oils, knitting lubricants (paraffin/wax applied at the knitting machine), sizing, dirt, and loose fibre/lint** so the fabric wets out evenly and softeners/finishes can deposit. `[FACT — standard wet-processing sequence; woolwise.com wool wet-finishing notes]`

**What it does for hand/quality.** A sweater straight off the machine is harsh, oily-handed, and dimensionally "tight" (full of knitting stress). Scouring is the reset: it relaxes the loops, removes the greasy hand, and is the precondition for every later finish depositing evenly. Skipping or under-scouring shows up as **patchy softener uptake and shade unevenness** later. `[ESTIMATE: process-reasoning, HIGH]`

**Conditions (indicative).** Mild alkaline or neutral detergent bath, ~40–60 °C for synthetics/acrylic, hotter for cotton; gentle for wool to avoid felting (see §2). `[ESTIMATE: industry-lore, MED]`

**Fibre split for our 3 articles (T2):**
- **A2 acrylic** — light scour; main job is removing knit oil. Low felting/shrink risk.
- **A1 cotton-blend** — scour (sometimes combined with the bio-polish/dye bath to save a cycle — see §4). `[FACT — combined biopolish+dye cited, infinitabiotech/clustercollaboration]`
- **A3 wool/merino** — scour is the *gateway to milling* and the **highest-control step**: too much mechanical action here already starts irreversible felting.

---

## 2. Milling / fulling — WOOL ONLY (the deliberate, controlled felt)

**What it is.** Controlled **wet + heat + mechanical agitation** that makes wool's scaled fibres migrate and interlock, **felting the surface and closing up the fabric** → a fuller, denser, softer, warmer, hairier "milled" hand. The same physics as accidental shrink-felting, run on purpose and stopped at a target. `[FACT — woolmark.com woollen finishing; woolwise.com wet finishing]`

**What it does.** Consolidates the structure (yarns swell and "burst", fibres protrude), giving woollen-spun knits their characteristic soft hairy surface and improved cover/warmth. Applies to **woollen-spun** A3 product; worsted-spun merino is milled lightly or not at all (milling a clean worsted face muddies the stitch definition). `[FACT — milling consolidates woollens; ESTIMATE: worsted caveat, MED-HIGH]`

**The QC risk (carry to T5).** **Over-milling shrinks and felts irreversibly** — it is the single most destructive finishing failure for wool, and it is one-way: you cannot un-felt. Milling must be gated on **dimensions + hand + a felt/area-shrinkage limit**, not run "to the clock". `[FACT — GLOSSARY §4 "over-milling shrinks/felts irreversibly"; ESTIMATE: gating prescription, HIGH]`

**Indicative conditions.** Mild soap/lubricant bath, ~38–42 °C, timed in minutes (a patent example cites ~39–41 °C, 8–12 min for a wool sweater) — **time is the dial, and it is short**. `[ESTIMATE: industry-lore from patent example CN101385578A, MED — single source, treat as order-of-magnitude]`

**Relevance to us.** A3 is a *thin Nov–Feb capsule* (T2), so milling is a **low-volume, high-skill, high-risk** step we'd push to a wool-experienced job-worker, not learn in-house. For A1/A2 (cotton/acrylic) **milling does not apply at all** — a common spec error is to write "wash-and-mill" generically across a line; only the wool article mills.

---

## 3. Anti-pill routes — the load-bearing section (ties to the T2 ICI spec)

Pilling = tangled balls of broken/loose **surface fibres** (GLOSSARY §1). T2 already set the defence *upstream* in the yarn (staple, twist, ply, fineness, anti-pill grade) and established the **contractual gate: ICI Pilling Box (ISO 12945-1) grade ≥ 3–4 after ≥1 home-wash cycle.** Finishing is the *second* line of defence and is **fibre-route-specific** — there is no universal "anti-pill finish".

### 3a. The four real routes (matched to fibre)

| Route | Mechanism | Fibre it serves | Net effect on ICI grade |
|---|---|---|---|
| **Singeing / gassing** (yarn-stage, sometimes fabric) | Yarn run through a **gas flame** at speed; burns off protruding fuzz → smooth, low-hairiness, lustrous yarn | **Cotton** (A1) and cotton-rich blends; viscose blends | Fewer surface hairs → fewer pill anchors. "Singeing virtually eliminates pilling in the finished fabric"; removes surface fibre, cuts air-drag ~26% `[FACT — textiletradebuddy.com; iosrjournals viscose study]` |
| **Bio-polishing / cellulase enzyme** (fabric/garment wet stage) | **Cellulase** hydrolyses protruding cellulosic micro-fibrils; light abrasion then breaks them off → clean, low-fuzz, softer surface | **Cotton** (A1) and other cellulosics ONLY — does nothing on acrylic/wool | Significant, *wash-proof, non-greasy* pill reduction + softer hand `[FACT — sciencedirect biopolishing; shreebschemicals]` |
| **Chemical anti-pill / resin / silicone bonding** | Polymer/resin or silicone deposits **anchor surface fibres** so they don't pull free; sometimes "anti-pill" softener | Acrylic (A2), wool; cotton as adjunct | Lifts grade but can stiffen hand and risks fastness/feel trade-off `[ESTIMATE: industry-lore, MED-HIGH]` |
| **Low-pill fibre choice** (upstream, T2 — not a finish) | Higher-dpf / anti-pill acrylic grade; longer staple; nylon % in soft-wool blends; tighter twist/gauge | All — set in the yarn PO | The biggest single lever; **finishing cannot fully fix a high-pill yarn** `[ESTIMATE: industry-lore, HIGH; T2 §4b]` |

### 3b. Cellulase bio-polish — process detail (our A1 cotton core)

The route that matters most for the **12-month cotton core (A1)**, because acrylic can't be enzyme-polished and wool isn't cellulosic:
- **Dose ~0.5–1.5% cellulase on fabric weight, pH ~4.5–5.5, ~45–55 °C, ~30–45 min**, then **inactivate** the enzyme with a brief alkali wash (pH >9) or heat — *mandatory*, because live cellulase keeps eating cellulose and causes **strength loss** if not killed. `[FACT — process window, sciencedirect / shreebschemicals / diutestudents]`
- **Trade-off to spec:** bio-polishing removes fibre = a controlled, small **tensile/weight loss** (target the pill reduction, cap the strength loss). Over-treatment = a weak, thin garment. So the PO/tech-pack must state the **target hand AND a minimum retained bursting/tensile strength** (T5 acceptance test). `[FACT — strength-loss risk; ESTIMATE: dual-spec prescription, HIGH]`
- **Cost/throughput sweetener:** **bio-polish + dye can run in one bath**, cited at ~25% cost saving and ~180→100 min cycle compression — relevant because A1 is bought **dyed** (T2), so the dyer can fold the polish into the dye cycle. `[FACT — clustercollaboration/infinitabiotech; treat the % as vendor-cited, direction MED-HIGH, exact % LOW]`

### 3c. Acrylic (A2) is the hard case — and it's mostly an upstream fix

Acrylic **can't be singed cleanly** (thermoplastic — flame melts/beads it, not burns clean) and **can't be enzyme-polished** (not cellulosic). Its pilling defence is therefore **~80% upstream in the yarn** (anti-pill/high-dpf grade, higher twist, ply, tighter gauge — T2 §4b) plus a **chemical/silicone anti-pill softener** at finishing as a modest top-up. `[ESTIMATE: process-reasoning, MED-HIGH]`
- **Direct consequence for the contract:** for A2 the **ICI ≥3–4-post-wash gate is won at the yarn PO, not the finishing line.** This is exactly why T2 said A2 "lives or dies on the anti-pill spec." Finishing buys maybe a fraction of a grade; it cannot rescue a cheap 1-D acrylic. `[ESTIMATE: judgment, MED-HIGH]`

### 3d. How the test gate actually works (the contract mechanism, from T2)

- **ISO 12945-1 (ICI Pilling Box):** four specimens tumbled in a cork-lined box (commonly ~2,000 revolutions baseline for soft/wholegarment knits), graded **1 (severe) – 5 (none)** vs photographic standards, averaged to the nearest **half-grade**. `[FACT — textiletrainer.com; knitwear.io]`
- **Why "after a home wash":** washing loosens fibre structure and **drops the grade** — the post-wash grade is the one that predicts returns. `[FACT — washing decreases pilling grade]`
- **Retail bar:** most major brands require **≥3–4**; premium wool often **≥4**. Our T2 spec (≥3–4 post-wash) sits exactly on the mainstream bar — correct for A1/A2, and we should push **≥4** on the A3 wool/merino premium capsule. `[FACT — knitwear.io / begoodtex bands; ESTIMATE: A3 ≥4 prescription, MED-HIGH]`
- **ISO 12945-1 vs -2:** **-1 (box)** is the right method for **loose/soft/wholegarment sweater knits**; **-2 (Martindale)** suits flatter/denser fabric. Spec the *method* in the PO, not just a number. `[FACT — knitwear.io; begoodtex]`

> **Anti-pill closure:** the T2 yarn ICI clause and the T6 finishing route are **one continuous contract**, not two. The PO must read: *yarn anti-pill grade (upstream) + the specified finishing route per fibre (singe/bio-polish/chemical) + the same ICI ≥3–4 (≥4 for wool) post-wash gate on the FINISHED garment.* Testing only the yarn lets a loose knit or a skipped finish quietly fail; testing only the garment lets a bad yarn pass one lab dip and fail in the field.

---

## 4. Softening (the retail "hand")

**What it is.** Deposit of **softeners — most commonly silicone (amino-silicone) emulsions**, also fatty/cationic softeners — in the final wet bath, to lubricate fibre-on-fibre and deliver the smooth, "expensive" hand the customer feels in the first 3 seconds. `[FACT — silicone softening agents, USPTO 5102930; woolwise notes]`

**What it does / trade-offs.**
- **Amino-silicone** = the smoothest, most durable, "luxe" hand; can be **hydrophobic** (reduces moisture pick-up/absorbency — bad if A1 cotton is sold as "breathable") and can **yellow** whites at high heat. `[ESTIMATE: industry-lore, MED-HIGH]`
- **Hydrophilic silicone / fatty softeners** = softer-but-still-absorbent; less slick.
- Softening is partly **cosmetic and partly real** (it also lowers sew/wear abrasion), but it can **mask** a harsh yarn rather than fix it — a QC trap: a heavily-softened sample feels great in the showroom and harsh after one wash. `[ESTIMATE: judgment, MED-HIGH]`

**Spec implication.** Softener type interacts with the care story: don't silicone-coat a cotton article you market as moisture-wicking; do specify a **wash-durable softener** so the showroom hand survives to the customer. `[ESTIMATE: prescription, MED-HIGH]`

---

## 5. Steam-press / boarding / blocking (set the shape and the measurements)

**What it is.** **Heat + moisture (steam) ± tension/form** applied to set the garment to its final dimensions and relax knitting stresses:
- **Steam pressing** — flat steam press / steam tunnel; relaxes loops, sets hand, removes creases.
- **Boarding / form-pressing** — garment pulled over a **heated metal/board form** in the target size and steam-set; the classic sweater-finishing step that fixes **measurements and symmetry**. `[ESTIMATE: industry-lore, HIGH]`
- **Blocking** — pinning/laying to measured dimensions (more artisanal / sampling-scale).

**What it does for quality.** This is where **the tech-pack measurements are actually hit** and panels/symmetry are locked. Steam under slight tension/permanent-set **reduces later relaxation shrinkage and hygral expansion** (the garment "remembers" the set dimension). `[FACT — researchgate: steaming under tension lowers relaxation shrinkage & hygral expansion in plain knit wool]`

**The QC failure mode (carry to T5).** **Rushed/insufficient pressing → measurement defects + appearance defects** (twisted side-seams, asymmetric panels, wavy hems) — GLOSSARY §4 flags "rushed pressing shows as measurement + appearance defects." For **fully-fashioned linked** sweaters (our quality story), boarding is what makes the linked seam lie flat and the garment read "premium." `[FACT — GLOSSARY §4; ESTIMATE: FF-boarding link, MED-HIGH]`

---

## 6. Relaxation & shrink control (so the customer's wash doesn't ruin it)

**The core fact.** A knit straight off the machine is full of stored stress. **Relaxation shrinkage** = stresses released on first wetting; it is **common to all fibres/constructions and non-reversible**, and if it happens *at the customer's house instead of in the factory*, that's a returned, undersized garment. `[FACT — sciencedirect / researchgate relaxation-shrinkage]`

**The control levers:**
1. **Wet-relax / tumble-relax / kier-relax** — wet the fabric/garment and let it move to its low-stress dimension **before** measuring and pressing, so the shrinkage is *spent in the factory*. `[FACT — relaxing treatment aims for minimum-stress, dimensionally-stable state; USPTO 4773133]`
2. **Compacting / "sanforising" for knits** — mechanical pre-shrinking to a **residual-shrinkage limit** (e.g. ≤ ±3–5%) so the garment stays in size tolerance after the customer washes. `[FACT — GLOSSARY §4; ESTIMATE: the ±3–5% number, MED — typical retail tolerance, confirm per buyer]`
3. **Steam-set under tension** (§5) — locks the relaxed dimension in.

**Why it matters double for us.** Our care label will say machine-washable (acrylic/cotton core for a US D2C customer who *will* tumble-dry), so **dimensional stability after a home wash IS the product promise.** The acceptance test is **dimensional-stability / shrinkage %** (GLOSSARY §5), gated like pilling: *measure shrinkage after the same home-wash cycle used for the ICI test.* Spirality (twisted side-seam from unbalanced singles twist) is the sister defect — fixed upstream by **balanced ply** (T2 §4a dial 2) and exposed here. `[FACT — GLOSSARY §5; ESTIMATE: combined-test prescription, HIGH]`

---

## 7. Mercerising (A1 cotton premium lever — adjacent finish)

Briefly, because T2/GLOSSARY flag it as the cotton upgrade: **caustic-soda treatment of cotton under tension → lustre, strength, better dye uptake** ("mercerised cotton" as a premium claim). `[FACT — GLOSSARY §4]` It's a **yarn/fabric-stage** treatment, usually bought *in* (mercerised cotton yarn) rather than run by a sweater finisher, and pairs naturally with **gassed/singed** cotton ("gassed & mercerised" is the standard premium-cotton combo). `[FACT — chromatic-rayon/shsamsun on gassed+mercerised]` For A1 it's an **optional up-positioning**, not a base requirement.

---

## 8. The fibre × finishing matrix (which steps each article actually gets)

`[ESTIMATE: process-mapping synthesis of §§1–7, MED-HIGH — the routing is sound; per-vendor availability in Ludhiana is UNKNOWN, carried as an open question]`

| Step | A1 Cotton-blend core (12GG) | A2 Fine-acrylic value (7–12GG) | A3 Merino/lambswool capsule |
|---|---|---|---|
| Scour/wash | Yes | Light (de-oil) | Yes — gentle (felt risk) |
| **Mill/full** | **No** | **No** | **Yes** (woollen-spun; light/none for worsted) |
| Singe/gas | **Yes** (yarn-stage, premium) | No (melts) | No |
| Bio-polish (cellulase) | **Yes** (key anti-pill) | **No** (not cellulosic) | No (not cellulosic) |
| Chemical/silicone anti-pill | Adjunct | **Primary finishing anti-pill** | Adjunct |
| Soften (silicone) | Yes (hydrophilic — keep breathable) | Yes | Yes (luxe hand) |
| Mercerise | Optional premium | No | No |
| Steam-press / board | Yes | Yes | **Yes — critical** (premium FF hand) |
| Relax / shrink-control | Yes | Yes | **Yes — critical** (felt + relax) |
| **ICI gate (post-wash)** | **≥3–4** | **≥3–4 (won at yarn PO)** | **≥4** |

**Reading of the matrix:** A1's anti-pill is a *real finishing route* (singe + bio-polish) we can spec and verify; A2's anti-pill is *upstream yarn* with only a chemical top-up at finishing; A3's whole quality story is *milling + boarding + relax*, the highest-skill/highest-risk route, on the lowest volume.

---

## 9. Dyeing options — yarn-dyed vs piece-dyed vs garment-dyed (quality/risk trade-offs)

This is the **stage at which colour enters**, and it's a lead-time-vs-colour-risk trade (GLOSSARY §4 "later-stage = faster trend response but more colour-consistency risk"). T1 already fixed the **regulatory** boundary: **never own a dyehouse** (CETP non-compliance, NGT, closure risk); T2 fixed the **commercial** boundary: **launch on bought-dyed stock shades**; dye-lot MOQ (~150–500 kg/colour ⇒ 2–4 colours) is the binding number. This section adds the **quality/process** trade-off between *where in the chain* you dye.

| Option | When colour is added | Strengths | Risks / weaknesses | Fit for us |
|---|---|---|---|---|
| **Fibre / stock / top-dyed** | Loose fibre before spinning | Deepest, most even penetration; best **fastness**; enables heathers/melange | Longest lead time; commit colour earliest; no trend agility | The melange/heather look (Ludhiana strength); A2/A3 marl effects |
| **Yarn / package-dyed** | On the cone, before knitting | **Excellent colour consistency + fastness**; enables stripes/jacquard/intarsia (multi-colour-in-one-garment); durable, "dimensional" colour | Earlier colour commitment; **lot-to-lot ΔE risk across dye lots** (the "two skeins, different lot" problem) → manage with ΔE clause; longer lead than piece/garment | **Default for us** — T2 says launch on **bought-dyed** (= yarn/package-dyed) stock shades; only way to do colour patterning |
| **Piece-dyed** | Knitted fabric/blank, before cut | Cheapest, standard lead time; dye to demand | Solid colours only; **dye unevenness across a piece**; less penetration than yarn | Relevant for **cut-&-sew** value product, less for our fully-fashioned line |
| **Garment-dyed** | Finished sewn/linked garment | **Fastest trend/colour response** (dye to order, late colour commitment → less colour inventory risk); soft "vintage", lived-in hand popular on knits | **Highest risk:** can **shift final measurements (shrinkage in the dye bath)**; uneven splotching if not agitated; seam/trim/thread must all dye compatibly; weakest fastness/penetration | A **later-stage option for a proven hero style** (mirrors T2's "graduate to job-work dye for a hero colour"); NOT the launch default |

**The trade-off, stated cleanly `[ESTIMATE: synthesis, MED-HIGH; player/regulatory anchors FACT from T1/T2]`:**
- **Earlier dyeing (fibre/yarn) = better fastness + penetration + colour consistency, worse agility + earlier cash/colour commitment.**
- **Later dyeing (piece/garment) = better agility + less colour inventory risk + soft hand, worse shade consistency + measurement/shrinkage risk + fastness.**
- **Our call (inherited + extended):** **launch yarn/package-dyed (bought-dyed stock shades)** — best consistency for a brand that can't absorb shade complaints, and the only route that gives the marl/stripe/intarsia looks; **reserve garment-dyeing for a later hero-style drop** where its agility + vintage hand earn their measurement/fastness risk; **never piece-dye-and-cut as the core** (that's the C&S value path, off our FF quality story). And **whichever stage, the dye node is outsourced** — the risk stays on the supplier (T1).
- **The cross-cutting QC point (carry to T5):** **lot-to-lot shade match (ΔE) + colour fastness (wash/rub/light) minima** must be in the PO at *whatever* dye stage, because shade drift between batches is a real defect that a tiny brand cannot eat. T2 §4d already put ΔE + fastness in the yarn PO; garment-dye would move that gate to the finished garment. `[FACT — fastness/ΔE as standard QC; T2 §4d]`

---

## 10. What's in-line vs what we gate by contract (the make-vs-buy of finishing)

Mirrors the venture's central make-vs-buy lever, applied to finishing:
- **Runs in-line at the job-work knitter/finisher/dyer (we buy it):** scour, mill (wool specialist), singe (yarn supplier), bio-polish (often folded into dye), soften, dye — all wet/skilled/equipment-heavy nodes, especially the **regulated dye node we must NOT own (T1)**.
- **The brand owns the SPEC + the TEST GATE, not the machines:** exactly like T2's yarn posture — **contractual QC, not in-house metrology.** The brand writes the finishing route per article into the tech-pack and **gates the finished garment** on: ICI ≥3–4 post-wash (≥4 wool), dimensional-stability/shrinkage % after the same wash, spirality, retained strength (after bio-polish), ΔE shade-match + fastness, and AQL workmanship.
- **This is the T5 "thread-checker" hand-off:** the device/closed-loop T5 designs is meant to cheaply verify the **finished-garment subset** of these (pilling + shrinkage + shade) and send a code back to the supplier — closing the same contractual loop on finishing that T2 opened on yarn. `[ESTIMATE: design-framing, consistent with T2 §4c / MASTER_PLAN T5]`

---

## Bottom line (Analyst A)

1. **Finishing is the second line of pilling defence, not the first** — for **A2 acrylic the ICI ≥3–4 gate is won at the yarn PO** (acrylic can't be singed or enzyme-polished); for **A1 cotton it's a real, specifiable route (singe + cellulase bio-polish, with a retained-strength cap)**; for **A3 wool it's chemical anti-pill + a few-% nylon upstream**. One continuous contract: yarn grade + finishing route + the same post-wash ICI test on the **finished garment**.
2. **Milling is wool-only and irreversible** — a high-skill, low-volume, push-to-specialist step; never write "wash-and-mill" generically across cotton/acrylic.
3. **Hand = softening (silicone) but it can mask a harsh yarn** — spec a wash-durable softener and don't hydrophobe-coat a "breathable" cotton article.
4. **Boarding + relax/shrink-control are where measurements and the customer's wash experience are delivered** — gate dimensional-stability/shrinkage % after the *same* home wash as the ICI test; spirality is the balanced-ply sister defect.
5. **Dye stage is a fastness-vs-agility trade:** launch **yarn/package-dyed (bought-dyed stock shades)** for consistency + patterning; reserve **garment-dye for a later hero style** (agility + vintage hand vs measurement/fastness risk); never own the dye node (T1); carry **ΔE + fastness** in the PO at whatever stage.
6. **The brand owns the spec + the test gate, not the wet machines** — finishing make-vs-buy resolves to *buy the process, own the contract*, exactly like T2's yarn posture, and it's the same loop the T5 thread-checker closes.

---

## Open questions (floor, not ceiling) — for the reconciler / T5 / on-the-ground

1. **Availability of gas-singeing and cellulase bio-polishing as job-work services in/near Ludhiana, and at what MOQ/price — `[UNKNOWN]`.** Determines whether A1's anti-pill route is real locally or must be designed around.
2. **Anti-pill / high-dpf acrylic finishing-stage options for A2 beyond the yarn — `[UNKNOWN]`;** how much grade a chemical top-up actually buys on acrylic (direction known: small).
3. **Per-garment finishing cost bands (scour/soften/press/relax) at a Ludhiana job-worker — `[UNKNOWN]`;** blocks the T4 cost model (sister to the missing per-piece linking rate from T1).
4. **Quantified ICI-grade uplift from each finishing route on our exact constructions — `[UNKNOWN until PPS swatch testing]`** (T5/T8); keep route *directions* HIGH, magnitudes LOW.
5. **Garment-dye shrinkage/measurement shift on our 12GG fine knits — `[UNKNOWN]`;** needs a sample dye trial before any hero-style garment-dye drop.
6. **Wool-milling job-work capacity + felt-control reliability in-cluster — `[UNKNOWN]`;** A3 is low-volume so capacity may be thin/seasonal.
7. **Care-label + softener interaction (silicone vs marketed "breathable" cotton) — a brand decision (T7/T8)** that picks the softener chemistry.

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/finishing/analyst_a_process.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `FACTORY/sweater_vertical/analysis/SCOPE_GLOSSARY.md`; `FACTORY/sweater_vertical/analysis/yarn/DEEPDIVE.md`; `FACTORY/sweater_vertical/analysis/ludhiana/DEEPDIVE.md`
- **Delivered (process angle):** the full post-knit finishing map for sweaters — (1) scour/wash, (2) wool-only milling/fulling with the irreversible-felt risk, (3) the four anti-pill routes (singe/gas, cellulase bio-polish, chemical/silicone, low-pill fibre) matched per-fibre to A1/A2/A3 and tied back to the **T2 ICI ≥3–4 post-wash gate** as one continuous yarn+finishing contract, (4) silicone softening + masking trap, (5) steam-press/boarding/blocking for measurements & FF hand, (6) relaxation & compacting/shrink-control for the home-wash promise, (7) mercerising as A1 premium lever, (8) a fibre×finishing matrix, (9) the **yarn/piece/garment-dye fastness-vs-agility trade-off** with our launch-yarn-dyed / reserve-garment-dye-for-a-hero / never-own-the-dye-node call, (10) the in-line-vs-contract make-vs-buy resolving to *buy the process, own the spec+test gate* (the T5 hand-off).
- **Honesty discipline:** every temperature/dose/cost/throughput band tagged `[ESTIMATE]` with basis+confidence; process identities and cited test/standard facts `[FACT]`; single-source patent/vendor figures (milling temp, 25% cost saving, ~26% air-drag) flagged as order-of-magnitude / vendor-cited; Ludhiana job-work availability + per-garment finishing cost + ICI uplift magnitudes held `[UNKNOWN]` and listed as blocking open questions; no fabricated precision; inherited T1 (no dyehouse) and T2 (bought-dyed, ICI ≥3–4) boundaries respected and extended, not contradicted.
- **Bottom line:** finishing is the second (not first) line of pilling defence and is fibre-route-specific; milling is wool-only and irreversible; hand comes from silicone but can mask; boarding+relax deliver the measurements and the home-wash promise; dyeing is a fastness-vs-agility trade resolved as launch-yarn-dyed; and the brand should **buy the wet process but own the spec + the same post-wash test gate** that T2 opened on yarn — the loop T5 closes.

### Sources (this analyst's web anchors)
- ICI Pilling Box / ISO 12945-1 method, retail ≥3–4 bar, post-wash grade drop, -1 vs -2: textiletrainer.com, knitwear.io, begoodtex.com
- Cellulase bio-polishing mechanism/process window/strength-loss/combined-with-dye: sciencedirect.com (biopolishing overview), shreebschemicals.com, infinitabiotech.com, clustercollaboration.eu, diutestudents.blogspot.com
- Yarn singeing/gassing (anti-pill, hairiness, ~26% air-drag, gassed+mercerised): textiletradebuddy.com, iosrjournals.org, chromatic-rayon.com/shsamsun, en.wikipedia.org "Gassing (textile process)"
- Wool milling/fulling + softening (woollen finishing, fulling consolidates, silicone softeners, ~39–41 °C example): woolmark.com, woolwise.com, USPTO 5102930, CN101385578A
- Steaming/relaxation/dimensional stability (steam-under-tension lowers relaxation shrinkage; minimum-stress relaxing treatment): researchgate.net, sciencedirect.com, USPTO 4773133/4858288
- Dyeing stage trade-offs (yarn/piece/garment/top, consistency vs agility, garment-dye measurement shift): fanterco.com, gooten.com, knomadyarn.com, eysan.com.tw
