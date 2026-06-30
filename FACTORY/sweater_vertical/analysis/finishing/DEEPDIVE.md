# Finishing & Post-Treatment — Owner Deep-Dive (T6)

**For:** the owner of a small, AI-designed D2C knitwear brand making sweaters in/through Ludhiana and selling in warm-ish **California / Turlock** (hot summers, a narrow true-cold window).
**Question this answers:** Once the yarn is bought and the panels are knit, **what finishing do we do to each of our three articles** to reach best quality and *control pilling*? **What do we outsource** (and why we own no wet processing / no dyeing)? **What does it cost?** And **what acceptance checks** decide pass/fail — closing the pilling/returns loop T2 opened and handing the garment-side QC to T5?

**How to read the tags (binding honesty discipline — same as Ludhiana/T1 and Yarn/T2):**
- **[FACT]** = cited public source / a true-by-definition process identity (e.g. "milling is wool-only").
- **[ESTIMATE]** = a reasoned estimate with method + basis + confidence stated; carries the literal word *estimate*. **Every cost / temperature / time / dose band below is `[ESTIMATE]`.**
- **[UNKNOWN]** = we genuinely don't know and won't fake it. Coverage here is a **floor, not a ceiling**.

> **The one sentence to remember.** *Finishing is the **second** line of pilling defence, never the first — the yarn PO sets the floor (T2), and finishing is **fibre-route-specific** (there is no universal "anti-pill finish"): on **cotton (A1)** it earns a *real* extra grade (gas-singe + cellulase bio-polish), on **acrylic (A2)** it buys only a *fraction* of a grade (the grade was won or lost at the yarn PO), and on **wool (A3)** it's a buy-the-craft milling/boarding job at ICI ≥4. The single most expensive spec lie is promising that finishing will "fix" a cheap yarn — great showroom sample, return wave one wash later. So: **outsource 100% of the wet/regulated steps (never own a dyehouse or any effluent node), and own only the two-PO recipe + the finished-garment post-wash test gate** — the garment-side half of the per-lot scorecard the T5 thread-checker returns to the named supplier.*

---

## 1. The thesis: floor upstream, finish to the ceiling, gate the garment

Three things govern everything in this deep-dive, and all three are inherited and extended, not invented here:

1. **Finishing is the second line of pilling defence, not the first.** T2 set the floor with the five spinning dials (staple, twist, ply, fineness, blend ratio) and the **ICI Pilling-Box ≥3–4 post-wash** contractual gate. Finishing then either delivers a *real* extra grade (cotton) or only a *cosmetic fraction* (acrylic). What finishing genuinely **owns for every article** is the retail **"hand"** (how the sweater feels in the customer's hands) and the **home-wash dimensional experience** (whether it holds its size after the customer washes it). `[ESTIMATE: process-reasoning, HIGH]`

2. **There is no universal "anti-pill finish."** The route is dictated by **fibre chemistry**, not a generic line spec. The four real anti-pill routes (§3) each attach to a specific fibre; writing "wash-and-anti-pill" generically across cotton/acrylic/wool is a category error that produces either a no-op or damage. `[ESTIMATE: industry-lore, HIGH]`

3. **We own the SPEC + the TEST GATE, not the machines** — "buy the process, own the contract." Identical to T2's yarn posture ("contractual QC, not in-house metrology") and T1's dyeing posture ("never own a dyehouse"). Finishing inherits both and **never contradicts them.** `[ESTIMATE: judgment, HIGH — T1 §5b/§6, T2 §4c/§5]`

**The governing fault, stated plainly:** a recipe that promises finishing will "fix" a cheap yarn is the **single most common and most expensive spec lie** in knitwear — the showroom sample feels premium (because it's freshly softened and pressed), then pills and bags out one home wash later, and the returns land on us. Finishing's *price* is small (§5); its *quality gate* is decisive (§6) — exactly the same shape as the yarn conclusion in T2.

---

## 2. The finishing chain — what each step does

The post-knit sequence, with the steps that don't apply to every garment marked. `[AGREED process spine; FACT for the step identities, ESTIMATE for "not every garment uses every step"]`

```
knit panels → linking/assembly → scour/wash → (WOOL ONLY) mill/full → anti-pill route (per fibre, §3)
   → soften → extract/dry → steam-press / board / block → relax & shrink-control → final QC gate (§6)
```

Dyeing is **not** a single step in this chain — it's a stage-of-the-chain *choice* (fibre / yarn / piece / garment), handled in §7.

| Step | What it does for the garment | Honesty tag |
|---|---|---|
| **Scour / wash** | The first wet step. Strips spinning oils, knit lubricants (paraffin/wax), sizing and lint so later finishes deposit *evenly* and the loops relax. **Under-scouring → patchy softener uptake + shade unevenness** — a finish applied over residual oil goes on blotchy. | `[FACT — wet-processing sequence]`; the "patchy uptake" consequence `[ESTIMATE: process-reasoning, HIGH]` |
| **Milling / fulling — WOOL (A3) ONLY** | Controlled wet + heat + agitation that felts and **closes up** a woollen knit → fuller, softer, warmer, hairier hand. **It is the same physics as accidental shrink-felting, deliberately stopped at a target.** Worsted/merino is milled lightly or not at all. **Irreversible if over-done — the single most destructive finishing failure.** Gate it on **dimensions + a felt/area-shrink limit + hand**, never "to the clock." Time is the dial and it is *short* (minutes). | `[FACT — woolmark/woolwise; GLOSSARY §4]`; the ~39–41 °C / 8–12 min window is `[ESTIMATE: industry-lore, single-source patent CN101385578A, MED — order-of-magnitude only]` |
| **Anti-pill (per fibre)** | Four real routes, fibre-matched. **Not universal.** | see §3 |
| **Softening** | Silicone (amino- or hydrophilic) or fatty-cationic deposit gives the "expensive" 3-second hand and lowers wear abrasion. **But it can MASK a harsh yarn** — a heavily-softened sample feels great in the showroom and harsh after one wash. | `[FACT — silicone softeners, USPTO 5102930]`; the masking trap + amino-silicone hydrophobicity/yellowing `[ESTIMATE: industry-lore/judgment, MED-HIGH]` |
| **Steam-press / board / block** | Heat + moisture (± a garment-shaped form) sets final dimensions and relaxes knit stress. **This is where the tech-pack measurements are actually hit.** Steam-under-tension lowers later relaxation shrinkage; rushed pressing shows as twisted seams, asymmetric panels, wavy hems. **Critical for the fully-fashioned "premium" read** on A3. | `[FACT — steam-under-tension, researchgate; GLOSSARY §4]`; the FF-boarding-premium link `[ESTIMATE, MED-HIGH]` |
| **Relax & shrink-control** | Relaxation shrinkage = stored knit stress released the first time the fabric is wetted. **Common to all fibres, non-reversible.** Spend it in the *factory* (wet-relax / tumble / kier-relax) not at the customer's washing machine; then compact ("sanforise") to a residual-shrink limit and steam-set to lock it. | `[FACT — relaxation-shrinkage, sciencedirect/researchgate; GLOSSARY §4]`; the ±3–5% residual limit `[ESTIMATE: typical retail tolerance, MED — confirm per buyer]` |
| **Mercerising — A1 cotton, optional up-position** | Caustic-soda-under-tension on cotton → lustre, strength, dye uptake. A **yarn/fabric-stage buy-in**, not a finisher step; pairs with gassing ("gassed & mercerised" = the standard premium-cotton combo). **Optional up-positioning, not base.** | `[FACT — GLOSSARY §4]` |

**The one structural fact that shapes the whole contract:** the recipe spans **two purchase orders**, not one. **Singe and mercerise happen at the yarn supplier; the wet + mechanical steps (scour / bio-polish / mill / soften / press / relax) happen at the garment knitter/finisher.** Both POs must carry the *same* post-wash ICI + shrinkage gate — or a step skipped at one vendor quietly passes the test at the other. (Developed in §4 and §6.) `[ESTIMATE: design-framing, HIGH]`

---

## 3. Anti-pill — the load-bearing section (four routes, each tied to a fibre)

Pilling = tangled balls of broken/loose **surface** fibres; it's the #1 post-purchase complaint and the main return driver (GLOSSARY §1, §5). T2 set the upstream defence (the five spinning dials, ICI ≥3–4 post-wash). Finishing is the **second line**, and there are exactly four real routes — **each attached to a fibre, none universal:**

| Route | How it works | Serves | Net effect | Tag |
|---|---|---|---|---|
| **Singeing / gassing** (yarn-stage) | Yarn passes through a gas flame; protruding surface fuzz burns off → low-hairiness, lustrous yarn with **fewer pill anchors** | **Cotton (A1)** + cotton-rich/viscose blends | Cuts surface fibre dramatically → "virtually eliminates" cotton pilling | `[FACT — textiletradebuddy/iosrjournals]`; the "~26% air-drag" figure is single-source `[ESTIMATE, decorative/order-of-magnitude]` |
| **Bio-polish / cellulase** (fabric or garment wet step) | A cellulase enzyme hydrolyses protruding cellulosic micro-fibrils; light abrasion snaps them off → clean, low-fuzz, softer fabric; **wash-durable, non-greasy** | **Cotton (A1) / cellulosics ONLY** — does **nothing** on acrylic or wool | Significant *durable* pill reduction + softer hand. **Must inactivate the enzyme (alkali/heat) afterwards** or it keeps eating cellulose → strength loss | `[FACT — sciencedirect/shreebschemicals]` |
| **Chemical / silicone anti-pill** (resin/silicone bonding) | A polymer/resin/silicone anchors surface fibres so they can't pull free and tangle | **Acrylic (A2) primary**; wool/cotton adjunct | Lifts grade **but** can stiffen hand + risk fastness/feel; buys only a *fraction* of a grade | `[ESTIMATE: industry-lore, MED-HIGH]` |
| **Low-pill fibre choice** (upstream — **NOT a finish**) | Higher-dpf / anti-pill acrylic grade; longer staple; a few % nylon in soft-wool blends; tighter twist + tighter gauge | All — set in the **yarn PO** | **The biggest single lever. Finishing cannot fully fix a high-pill yarn.** | `[ESTIMATE: industry-lore, HIGH — T2 §4b]` |

**Per-fibre resolution — the practical call for each of our articles:**

- **A1 cotton-blend core** — has a **real, ownable, verifiable** finishing route: **gas-singe the yarn + cellulase bio-polish the garment**, with a **mandatory retained-strength cap.** Bio-polish removes fibre (that's how it de-fuzzes), so it costs tensile strength — the spec must name **both** the target hand **and** a *minimum retained bursting/tensile strength*; over-treat and the sweater is weak, thin, and returnable. This is the article where finishing earns its keep. `[FACT — strength-loss risk]`

- **A2 acrylic value driver — the hard case.** Acrylic is **thermoplastic** (a flame beads/melts it → can't be cleanly singed) and **non-cellulosic** (cellulase does nothing). So **most of A2's pilling defence is in the yarn PO** — anti-pill / high-dpf grade, higher twist, 2-ply, tight gauge — and finishing adds only a **chemical/silicone top-up worth a fraction of a grade.** The blunt rule: **A2's ICI ≥3–4 gate is won or lost at the yarn PO, not the finishing line.** If we're ever forced onto cheap acrylic, the fix is to **re-open the yarn spec**, not to pile on chemical anti-pill. `[ESTIMATE: process-reasoning/judgment, MED-HIGH]`

- **A3 merino/lambswool premium** — chemical anti-pill is a minor adjunct; the real levers are **a few-% nylon upstream** (a *yarn* decision — the cheapest pill + abrasion lever, and the reason A3 can hit ICI ≥4 with a soft hand) plus the **milling + boarding craft** at the finishing stage. `[ESTIMATE — T2 §4]`

**The test method (binding `[FACT]`):** grade pilling on the **ISO 12945-1 box method** (the pilling-box) for our soft/whole-garment sweater knits — **not** ISO 12945-2 (Martindale), which suits flatter/denser fabric. Run ~2,000 revolutions as a baseline, grade **1 (severe) – 5 (none) to the nearest half-grade, after ≥1 home wash** (washing *drops* the grade — the post-wash grade is what predicts returns). Retail bar **≥3–4**; premium wool **≥4**. **Spec the method, not just the number** — "ICI ≥3.5" means nothing without "ISO 12945-1, post-1-wash."

---

## 4. The three recipes — one per article

`[ESTIMATE: synthesis, MED-HIGH; per-vendor Ludhiana availability/cost UNKNOWN — see §5 and open questions]`

| Decision | **A1 — Cotton-blend core (12GG)** | **A2 — Fine-acrylic value (7–12GG)** | **A3 — Merino/lambswool premium (Nov–Feb)** |
|---|---|---|---|
| **Where the grade is won** | **At finishing** (singe + bio-polish — a real route) | **At the yarn PO** (finishing buys only a fraction) | **At finishing** (mill + board) **+ nylon% upstream** |
| **Anti-pill primary lever** | Cellulase bio-polish (garment) | Anti-pill / high-dpf **yarn** grade | Few-% nylon + low-pill wool grade |
| **Anti-pill finishing top-up** | Gas-singe (yarn-stage) | Silicone/chemical (the *only* option) | Chemical adjunct (minor) |
| **The ONE dial that decides pass/fail** | Bio-polish **strength cap** | (decided upstream) softener-stiffness ceiling | Milling **time** (irreversible) |
| **Softener chemistry** | **Hydrophilic** silicone (keep it breathable) | Standard / anti-pill silicone | Amino-silicone (luxe hand OK) |
| **Mill / full?** | No | No | **Yes** — woollen-spun only; light/none on worsted merino |
| **Shape-set** | Steam-press + light board | Steam-press, **under-set** (excess heat glazes acrylic) | **Board / form-press — critical** for the FF premium read |
| **Shrink-control** | Wet-relax + compact (≤ ±3–5%) | Relax + compact | Wet-relax + felt-relax — **critical** |
| **ICI gate (finished, post-wash)** | **≥3–4** | **≥3–4 (won at the yarn PO)** | **≥4** |
| **Make-vs-outsource** | Outsource all wet; own spec + gate | Outsource all wet; own spec + gate | Outsource (specialist wool finisher); own spec + gate |
| **Per-garment finishing cost band** | `[ESTIMATE]` ~$0.30–0.90 | `[ESTIMATE]` ~$0.25–0.70 | `[ESTIMATE]` ~$0.60–1.80 |

**Mandatory vs optional:** the recipes above are the **mandatory** cores. **Mercerised + gassed cotton yarn (A1)** is **optional premium up-positioning bought in *on the yarn*** — not a finisher step, and not part of the base.

**One-line verdict per article:**
- **A1** = the **highest return-on-finishing** article — finishing earns the entire home-wash promise plus a real (if unmeasured) anti-pill uplift. **Spec it fully.**
- **A2** = **win it upstream, finish light, don't pretend finishing rescues a bad yarn.** The finishing line's job here is hand + measurements, not grade.
- **A3** = a **buy-the-craft** article whose risk is **in-cluster wool-finishing skill/capacity**, not the recipe — milling is high-skill, low-volume, possibly seasonal (open question 3).

---

## 5. Make-vs-OUTSOURCE — outsource 100% of wet, own only the spec + gate

**The call: outsource every wet / skilled / regulated finishing step; the brand owns only (a) the recipe in the two-PO tech-pack and (b) the finished-garment post-wash test gate.** `[ESTIMATE: judgment, HIGH — inherited T1 §5b/§6, T2 §4c/§5]` This is the **same posture** as T2 on yarn (contractual QC, not in-house metrology) and T1 on dyeing (never own a dyehouse). Nothing here contradicts them.

**Why outsource every wet step — not just the dyeing:**
- **Regulatory / effluent.** Wet processing makes effluent. The cluster's three CETPs were found non-compliant, CPCB levied crore-scale penalties, and the NGT is reviewing the cluster — owning **any** wet node imports closure risk wildly out of proportion to a 3-article line. `[FACT — T1 §5b]`
- **Capital + skill.** Singe ranges, strength-controlled enzyme baths, milling machines, boarding forms, and compactors are equipment-heavy and skill-bound; a micro-line can't utilise or staff them. `[ESTIMATE, HIGH]`
- **Volume.** At micro-capsule volume, in-house finishing kit sits idle. Outsourcing converts a fixed-cost machine into a **per-garment job-work charge** that scales with our (small) orders. `[ESTIMATE, HIGH]`
- **Agglomeration tailwind.** Ludhiana's walking-distance knit/dye/link/finish job-work mesh is built for renting exactly these steps — this is the one T1 moat that actually *helps* a tiny non-integrated brand. `[FACT — T1 §3 moat #5]`

**The one thing we DO own — and it's not a machine:** the **two-PO tech-pack recipe** (yarn PO: singe / mercerise / anti-pill grade / nylon%; finishing PO: scour / bio-polish / mill / soften / press / relax) **+ the finished-garment acceptance gate (§6).** This is the cheap, leverage-dense half of the make-vs-buy split — and it's the **same loop the T5 thread-checker closes**: a per-lot scorecard with a grade-code returned to the *named* supplier. Our leverage is **contractual and relational** (a supplier that accumulates out-of-spec codes), **not a clever device.** `[ESTIMATE: design-framing, HIGH]`

---

## 6. The finishing acceptance gate — eight checks on the finished garment

Every check runs on the **FINISHED garment** (after the *same* home-wash cycle, where the check is wash-sensitive) and feeds the **T5 scorecard-and-code** returned to the named supplier. This **extends T2's yarn-lot gate onto the garment**: testing only the yarn lets a skipped finish or a loose knit quietly fail; testing only the garment lets a bad yarn pass one lab dip. You need both ends. `[ESTIMATE: design-framing, HIGH — T2 §4c/§4d]`

| # | Check | Method / standard | Accept | Verifies which dial | Article |
|---|---|---|---|---|---|
| 1 | **Pilling (post-wash)** | ICI Box / **ISO 12945-1** (not -2), 1–5 to ½-grade, after ≥1 wash | A1/A2 **≥3–4**; A3 **≥4** | the per-fibre anti-pill route (master gate) | All |
| 2 | **Dimensional stability / shrinkage %** | Measure to tech-pack after the **same** wash as #1 | within **±3–5%** (confirm per buyer) | compact residual-shrink limit | All |
| 3 | **Spirality / twisted side-seam** | Visual + skew after wash | within tolerance | balanced ply (upstream T2 dial 2) — *exposed here* | All (esp. singles) |
| 4 | **Retained bursting/tensile strength** | After bio-polish vs un-polished baseline | **≥ stated minimum** | bio-polish strength cap | **A1 (mandatory)** |
| 5 | **Hand / softener durability** | Hand **after wash**, not in the showroom | survives ≥1 wash | hydrophilic softener / anti-masking | All (esp. A1/A3) |
| 6 | **Felt / area-shrink limit (milling gate)** | Area-shrink + hand vs target | within milling limit | milling **time** (irreversible) | **A3 (mandatory)** |
| 7 | **Shade match ΔE + colour fastness** | ΔE lot-to-lot; wash/rub/light minima | ΔE + fastness met | dye-stage choice (§7) | All |
| 8 | **Workmanship / appearance** | **AQL** (+ 4-point for any C&S panels) | buyer AQL met | press / board + assembly | All |

Checks **#1 (pilling), #2 (shrinkage), #7 (shade)** are exactly the finished-garment subset the **T5 device/loop** is meant to verify cheaply and code back to the supplier — so the finishing gate is **not a separate system**; it's the **garment-side half** of the per-lot QC scorecard T2 opened on the yarn side. **The two POs must carry the same gate** (§2), so a step skipped at the yarn vendor can't pass the test at the finishing vendor and vice-versa.

---

## 7. Dyeing stage — yarn vs piece vs garment

Dyeing is a **lead-time-vs-colour-risk trade**, with the *regulatory* boundary already fixed by T1 (**never own a dyehouse — outsource the node**) and the *commercial* boundary by T2 (**launch on bought-dyed stock shades**; dye-lot MOQ ~150–500 kg/colour ⇒ only ~2–4 launch colours).

| Option | Colour added | Strengths | Risks | Fit for us |
|---|---|---|---|---|
| **Fibre / stock / top-dyed** | loose fibre, pre-spin | deepest/most even penetration; best fastness; heathers/melange | longest lead; earliest commit; no agility | melange/marl (a Ludhiana strength) — A2/A3 marl |
| **Yarn / package-dyed** | on the cone, pre-knit | excellent consistency + fastness; enables stripe/jacquard/intarsia; durable | earlier commit; **lot-to-lot ΔE risk** → needs a ΔE clause; longer lead | **DEFAULT** — T2 launch on bought-dyed; the only route for colour patterning |
| **Piece-dyed** | knitted blank, pre-cut | cheapest; standard lead; dye to demand | solids only; piece unevenness; less penetration | the **C&S value path** — off our FF quality story |
| **Garment-dyed** | finished garment | **fastest agility**; soft vintage hand | **highest risk: shifts final measurements (dye-bath shrinkage)**; splotching; seam/trim must co-dye; weakest fastness | **later hero-style drop only**, NOT the launch default |

**Our call (inherited + extended) `[ESTIMATE: synthesis, MED-HIGH; regulatory/commercial anchors FACT from T1/T2]`:** **launch yarn/package-dyed (bought-dyed stock shades)** for consistency + colour-patterning; **reserve garment-dye for a later, proven hero style** (its agility + vintage hand earn the measurement/fastness risk only once the style is proven); **never piece-dye-and-cut as the core** (it's off our fully-fashioned quality story); and **whichever stage, the dye node is outsourced** (T1, non-negotiable). Carry the cross-cutting QC into T5: **lot-to-lot ΔE + wash/rub/light fastness minima in the PO at whatever dye stage** — shade drift is a real defect a tiny brand can't eat (= check #7).

---

## Bottom line for the owner

1. **Floor upstream, finish to the ceiling, gate the garment.** Finishing is the **second** line of pilling defence and is **fibre-route-specific — there is no universal anti-pill finish.** Don't ever let a recipe (or a sample) promise that finishing will fix a cheap yarn; that's the most expensive lie in knitwear.
2. **Per article:** **A1 cotton** has a *real, ownable* route — **gas-singe (yarn) + cellulase bio-polish (garment), strength-capped** — the highest return-on-finishing article, spec it fully. **A2 acrylic**'s ICI grade is **won or lost at the yarn PO**; finishing buys only a chemical/silicone *fraction* — if forced onto cheap acrylic, re-open the yarn spec, don't pile on anti-pill. **A3 wool** is a **buy-the-craft** milling + boarding + felt-relax job at **ICI ≥4** with a few-% nylon upstream.
3. **Milling is wool-only and irreversible** — high-skill, low-volume, push it to a specialist; **never write "wash-and-mill" generically** across cotton/acrylic; gate it on a **felt/area-shrink limit, not a clock.**
4. **Hand = silicone softening, but softener can MASK a harsh yarn** — spec a **wash-durable** softener, test the hand *after* a wash (not in the showroom), and use **hydrophilic** silicone on the "breathable" cotton (A1), amino-silicone on wool (A3).
5. **Boarding + relax/compact deliver the measurements and the home-wash promise** — gate dimensional-stability/shrinkage after the *same* home wash as the pilling test (**±3–5% working target, confirm per buyer**); spirality (the twisted-seam defect) is the balanced-ply sister fault exposed here.
6. **Dyeing is a fastness-vs-agility trade** — launch **bought-dyed stock shades** (yarn/package-dyed) for consistency + patterning; reserve **garment-dye for a later hero style**; **never own the dye node** (T1 NGT/closure risk); carry **ΔE + fastness** in the PO at whatever stage.
7. **Make-vs-buy resolves to buy the process, own the spec + gate** — **outsource 100% of wet/regulated finishing**, own only the **two-PO recipe** (yarn PO: singe / mercerise / anti-pill grade / nylon%; finishing PO: scour / bio-polish / mill / soften / press / relax) **+ the 8-check finished-garment post-wash gate** — the **garment-side half of the per-lot scorecard-and-code the T5 thread-checker returns** to the named supplier. **Leverage is contractual, not a device.**
8. **Finishing's *price* is not the constraint — its *quality gate* is.** ~$0.25–1.80/garment `[ESTIMATE]` is small and well-anchored as a *minority slice*; but the **per-garment job-work rate is a `[UNKNOWN]` point that blocks the T4 bottom-up model** until a live Ludhiana quote (sister to T1's missing per-piece linking rate and T2's missing dye-lot MOQ).

---

## Open questions — what only on-the-ground / live quotes can close

These are the things desk research **cannot** close. Floor, not ceiling. (Carried to T4 / T5 / T7-8 / on-the-ground.)

1. **Per-garment finishing job-work rate at a *named* Ludhiana finisher — `[UNKNOWN]`; BLOCKS the T4 model.** The bands in §4/§5 (~$0.25–1.80) are **triangulated, not measured** — the *ordering* A3 > A1 > A2 is sound, the *absolute numbers* are placeholders that must NOT be fed as points into T4 without a live quote. **Highest-priority blocker** (sister to T1's per-piece linking rate, T2's dye-lot MOQ). *(On-the-ground only.)*
2. **Gas-singe + cellulase bio-polish availability / MOQ / price as local Ludhiana job-work — `[UNKNOWN]`;** determines whether A1's *real* anti-pill route exists in-cluster or must be designed around (e.g. buying pre-singed/mercerised yarn and skipping local bio-polish).
3. **Wool-milling job-work capacity + felt-control reliability in-cluster — `[UNKNOWN]`;** A3 is low-volume, so milling capacity may be thin or seasonal, and felt-control skill is the whole game.
4. **Quantified ICI-grade *uplift* per route on our exact constructions — `[UNKNOWN until PPS swatch testing]`.** Keep the **directions HIGH** (finishing adds *some* grade on cotton, a *fraction* on acrylic) but the **magnitudes LOW** — "half a grade" is a convenient round number, not a measurement. → T5/T8 swatch trials.
5. **Acrylic chemical anti-pill — how much grade a top-up actually buys on A2 — `[UNKNOWN]`** (direction: small). Confirms whether A2's finishing top-up is worth specifying at all.
6. **Compacting residual-shrink tolerance the US D2C customer/buyer actually requires (±3–5%? tighter for a tumble-dry household?) — confirm per buyer.** A tumble-dry US customer may force a tighter limit than the working target. → T7/T9.
7. **Garment-dye shrinkage / measurement shift on our 12GG fine knits, IF a hero-style garment-dye drop is ever run — `[UNKNOWN]`; needs a sample dye trial** before committing.
8. **Care-label × softener interaction (silicone vs a marketed "breathable" cotton story) — a brand decision** that picks the softener chemistry. → T7/T8.

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/finishing/DEEPDIVE.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `FACTORY/sweater_vertical/analysis/SCOPE_GLOSSARY.md`; `FACTORY/sweater_vertical/analysis/yarn/DEEPDIVE.md`; `FACTORY/sweater_vertical/analysis/ludhiana/DEEPDIVE.md`; `FACTORY/sweater_vertical/analysis/finishing/reconciled.md`.
- **Delivered (owner-facing):** (1) the **thesis** — finishing is the *second* line of pilling defence, fibre-route-specific, owns the hand + the home-wash promise, "buy the process / own the spec+gate"; (2) the **finishing chain** with per-step facts and the milling/softener/relax traps, plus the **two-PO** structural fact; (3) the **four anti-pill routes** fibre-matched (singe→A1, bio-polish→A1, chemical→A2, low-pill fibre→upstream) with the per-article resolution and the ISO 12945-1 post-wash method; (4) **three recipes** as a decision table + one-line verdict per article (A1 highest-return / A2 won-upstream / A3 buy-the-craft); (5) the **outsource-100%-of-wet** boundary (no dyehouse / no effluent node, inherited T1/T2) with the one ownable sliver = the two-PO recipe + gate; (6) the **8-check finished-garment acceptance gate** = the garment-side half of the T5 scorecard-and-code; (7) the **dye-stage** trade (launch yarn/package-dyed, reserve garment-dye for a hero, never own the node).
- **Honesty discipline:** every cost / temperature / time band held as `[ESTIMATE]` with confidence (per-garment finishing rate carried as `[UNKNOWN]` and named as the T4 blocker; ICI-uplift magnitudes downgraded to LOW/direction-HIGH; milling temp/time and "~26% air-drag" flagged single-source/order-of-magnitude); process identities (milling = wool-only, cellulase = cellulosics-only, relaxation shrinkage = all-fibre) kept as `[FACT]`; inherited T1 (never own a dyehouse / outsource wet) and T2 (bought-dyed launch, ICI ≥3–4 post-wash, contractual-not-lab QC) boundaries respected and **extended**, never contradicted; the pilling/returns loop from T2 closed and the garment-side QC handed to T5; coverage stated as a floor with 8 carried open questions.
- **Bottom line:** floor upstream / finish to the ceiling / gate the garment; A1 has a real ownable route, A2 is decided at the yarn PO, A3 is a craft job at ICI ≥4; **outsource 100% of wet/regulated steps, own only the two-PO recipe + the finished-garment post-wash gate**; finishing's price (~$0.25–1.80/garment `[ESTIMATE]`, point `[UNKNOWN]`, blocks T4) is not the constraint — the **quality gate** is.
