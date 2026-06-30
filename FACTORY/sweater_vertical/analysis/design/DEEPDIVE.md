# AI Design Specialist — Owner Deep-Dive (T8)

**For:** the owner of the AI-designed D2C knitwear brand selling the **3-article California line** — **A1** cotton-blend 12GG 12-month core (margin/LTV engine) · **A2** fine-acrylic 7–12GG value/youth driver (acquisition front door) · **A3** thin merino/lambswool Nov–Feb premium capsule (halo / price-anchor) — seeded in **Turlock**, scaled to the **ship-to-California youth-leaning social shopper** (`brand_market/DEEPDIVE.md`).

**Four questions this answers:** (1) *What to design* — silhouette / stitch / colour / detail per article, and **why the constraints, not a moodboard, pick it.** (2) *The sizing + grading math* that must be exact — US size set, the four knit measurements, a per-size grade rule, and how knit gauge + shrinkage bend the spec, calibrated by the Turlock pop-ups. (3) *Trend ingestion → drop calendar* — how to read runway/retail/social for **this** colour-starved brand and turn it into a release clock. (4) *The Flux generative method* the owner runs himself — a reusable eight-slot prompt template, ready-to-run A1/A2/A3 example prompts, and the **honest bridge from a pretty image to a producible spec** that hands off to T2 (yarn) and T3 (programmer).

**How to read the tags (binding honesty discipline, per `SCOPE_GLOSSARY`):**
- **[FACT]** = cited public source / true-by-definition.
- **[ESTIMATE]** = reasoned, carries method + basis + confidence + the literal word *estimate*.
- **[UNKNOWN]** = genuinely not known; a valid, frequent answer. Coverage here is a **floor, not a ceiling**.
- **[SNAPSHOT]** = true at a point in time but **moving** (model versions, API params, live trends) — re-confirm before it drives spend.

> **The one sentence to remember.** *Design this capsule from **three fixed constraints, not a moodboard** — the **2–4-colour dye-lot ceiling** (`yarn §3b`), the **12GG warm-California weight** (`yarn §1c`), and the **anti-pill + linker-throughput reality** (`yarn §4`, `machines §1`) — yielding a **gender-neutral, soft-boxy, texture-over-colour** capsule whose looks are carried by **stitch architecture (rib / half-Milano / cable / pointelle), not by colour the budget can't pay for**; size it on a **linear finished-garment grade (chest +2"/size, body +1.5", sleeve +0.75", hem +2") built on a washed, relaxed swatch to a post-wash target**, and **let the Turlock pop-ups, not a chart, set the size curve**; then use **Flux as an idea + content engine, not a knit compiler** — prompt it as an **eight-slot fashion-photo brief encoding those same constraints**, run it **guidance-distilled (CFG ~3.0–3.8, ~24–32 steps, seed-locked one-variable iteration)**, and treat every render as a **look + content brief a human must translate into a tech pack** — because the **four things that make a knit buildable (yarn count, true GG/density, stitch-count grade, finished measurements/shrinkage) are exactly the four things a Flux pixel cannot specify** and must be filled from `yarn` + the sizing math + a physical PPS swatch.* `[ESTIMATE: synthesis, HIGH]`

---

## 1. The design judgement: three constraints decide the line before any aesthetics

The single most important judgement in this whole file: **the design is not generated from taste — it is generated from upstream constraints, and the 2026 trend happens to reward the same choices.** Four upstream facts cap the option space; the design lives inside them, and the rare alignment is that fashion points the same way. `[ESTIMATE: synthesis of upstream files, HIGH]`

| # | Constraint (source) | What it forbids / forces | The design consequence |
|---|---|---|---|
| 1 | **Colour budget ≤ 2–4 launch colours** — dye-lot MOQ ~150–500 kg/colour (`yarn §3b`) | can't fund multi-colour Fair-Isle / jacquard / 6-colour print | the line is **colour-starved, not style-starved** → interest comes from **stitch + silhouette, not colour-count** |
| 2 | **12GG warm-CA weight, ~200–350 g** (`yarn §1c`) | no chunky 3–5GG hero knits; warm climate sells heavy wool only ~Nov–Feb | fine-gauge **"elevated-basic" canvas**; fake substance with **surface stitch** (cable / half-Milano), not bulky yarn |
| 3 | **Linking is the throughput bottleneck** (`machines §1`) | every extra FF panel / colour-block join is linker minutes = capacity cost | favour **fewer, cleaner seams**; reserve fashioning marks as an A3 authenticity cue, not everywhere |
| 4 | **Anti-pill is a brand-ending gate on acrylic A2** (`yarn §4`, `brand_market §3`) | no long floats / brushed-fuzzy surfaces on acrylic — they pill fastest; "one pilling video is brand-ending" | A2 runs **smooth, tight, conservative stitch** + a contractual anti-pill spec |

**The design rule that falls out (the spine of the whole line):** **texture-over-colour, silhouette-over-print, single-colour body with at most a contrast rib/tip.** This is *also* the 2026 youth direction (boxy/oversized + rib/cable revival + pigment-return after greige), so constraint and fashion align — a coincidence worth banking, not relying on. `[FACT/ESTIMATE blend, HIGH]`

**The consumer the rule serves (carried from T7):** youth-leaning, value-quality California shopper discovering on TikTok FYP / IG Reels / Pinterest, plus the ~46%-Hispanic, CSU-Stanislaus Turlock seed (`brand_market §1`, `[FACT — demographics]`). That dictates three further design moves: **gender-neutral/unisex cuts** (widen the addressable buyer per MOQ-expensive SKU), **slightly-oversized/layering-friendly fits** (match both the 2026 silhouette and the warm climate, where a sweater is worn open, not as sole insulation), and **surfaces that photograph well on a phone** (the product *is* the content). The literal payoff Analyst B adds: **the act of AI-generating the design is itself the TikTok/IG asset** — the "AI-designed, made by hands" story pillar (`brand_market §3`).

---

## 2. The articles, designed (the fibre/role ladder — not three styles)

### 2a. Silhouette → stitch → colour → detail (shared spine)
- **Core fit block — "relaxed straight / soft-boxy":** dropped or soft-set shoulder, straight-to-slightly-tapered body, **~5–6" ease** (top of the standard 4–6" sweater layering band, `[FACT]`), graded XS–XXL, serving the unisex core. **Two silhouette variants off the one block** (SKU discipline): (a) crew/mock pullover (the volume seller, simplest to link); (b) cardigan or quarter-zip (layering). **No third base shape at launch.** Core at **true length**; crop reserved for the A2 youth drop. Avoid heavy shawl-collar / chunky funnel necks — they read alpine-heritage, which T7 bans ("clean/light/Californian, never alpine").
- **Stitch palette mapped to gauge/colour/machine** (`SCOPE_GLOSSARY §3`): **USE freely (single-colour, texture = interest)** — jersey, **rib 1×1/2×2** (cuffs/hems/collars *and* as a full-body hero), **half-Milano/Milano/half-cardigan** (fake density at a fine gauge — the warm-climate "substantial" trick), **cable** (texture-value but uses more yarn + machine time → price as premium/A3), **links-links/purl + pointelle** (lacy, lightweight, warm-CA spring). **USE sparingly (colour-cheap)** — **plating** (luxe face / plate elastane; ~zero colour cost), **tipping/contrast rib** (two-tone from the 2–4 budget). **AVOID/reserve (colour-hungry or pill-risky)** — **jacquard** (multiple colours + back-floats that pill on acrylic) and **intarsia** (slow/skilled; viable only as a single hero graphic on one premium drop). `[ESTIMATE: HIGH that colourwork is mostly out on this budget]`
- **Colour — 2 evergreen anchors + 1–2 rotating accents:** anchors = **warm off-white/cream + a mid neutral** (oat/greige/soft charcoal) — read "clean Californian," and **2026 Pantone "Cloud Dancer" (soft-white/putty)** makes the anchor on-trend, not a compromise `[FACT — Pantone 2026]`. Accents (≤2 at a time, rotate per drop) = **eggplant/plum, marigold/butter, teal, pillar-box red** (AW26 runway knit colours, `[FACT]`); **icy blues/purples + dusty pastels** the softer Gen-Z read `[ESTIMATE: MED]`. **Hero-colour discipline:** prove a hero on sell-through *before* paying a small-batch surcharge or a second dye lot; launch ~2 anchors + 1 accent. Colour names + any slogan-knit **must clear a Spanish reading** (mandatory, ~46% Hispanic, T7).
- **Details (cheap differentiators):** clean self-finished ribbed hems/cuffs/collars (recovery = perceived quality); **fashioning marks** as FF-authenticity on A3; a woven "from Turlock" care/origin tab; tonal/contrast-tipped rib; **minimal/no external logo** (quiet-luxury-adjacent *and* dodges the TM / right-of-publicity exposure T7 flags). One quarter-zip max; avoid hardware.

### 2b. The three articles
| | **A1 — cotton-blend core** | **A2 — fine-acrylic value/youth** | **A3 — merino/lambswool premium** |
|---|---|---|---|
| **Role** | margin + LTV engine | acquisition front door (youth) | halo / press / price-anchor |
| **Silhouette** | relaxed-straight unisex crew + 1 cardigan; **true length** | soft-boxy, **slightly cropped** option; trend-forward | refined relaxed crew/mock; cleanest fit |
| **Stitch** | jersey + full **rib**; **half-Milano** for substance; clean self-finish | **rib** + simple abrasion-surviving texture; **plating**; **no floats** | **cable** + fine **links-links**; **fashioning marks**; optional single **intarsia** hero |
| **Colour** | 2 anchors (cream + neutral) | anchors + the **trend accent** (youth colour lives here) | 1 anchor + 1 elevated tonal (eggplant/charcoal) |
| **Price ladder** (positioning, T9 sets floors) | **~$70–95** | **~$45–65** | **~$120–180** |
| **Design risk to watch** | don't over-elaborate the margin engine | **pilling video = brand-ending** → conservative stitch + anti-pill spec | cable/intarsia adds yarn + linker cost → price for it |

**Architecture rule (binding, carried from T7):** loud youth/hype energy at the **front** (A2, acquisition); the durable margin thing (A1) **behind** it. *If the A2 youth capsule visually becomes the house brand, margin and longevity erode.* Design A2 loud, A1 evergreen.

**Why exactly 3 — a fibre/role ladder, not a style count.** Each article occupies a distinct *economic role* the others can't fill: **A1** margin/LTV engine (cotton, longest sell-window ~Sept–April, best repeat), **A2** cheap front door (acrylic, bought for UGC/reach not contribution, highest return), **A3** halo/price-anchor (merino, press + makes A1 read as value). Drop any one and you lose a *function*. Three is also the **MOQ-survivable maximum**: 3 styles × ~3 colours × 6 sizes = **54 SKUs** already, each carrying MOQ + inventory math (`SCOPE_GLOSSARY §8`); a 4th breaks the dye-lot/working-capital ceiling, a 2nd forfeits a role. The capsule **cross-merchandises** (shared anchors + block + grade → bundles toward the AOV >~$90 CAC-headroom target, shares dye lots). `[ESTIMATE: synthesis, HIGH]`

---

## 3. The sizing + grading math (the part that must be exact)

### 3a. Finished-garment, not body — the #1 error
The **tech-pack spec is FINISHED-GARMENT (flat / half-) measurements, NOT body measurements.** Body chest + ease = garment chest; sweaters carry **~4–6" of layering ease**, our soft-boxy fit at the **upper end (~5–6")**. Grade the *garment*, then sanity-check against body charts. Confusing the two ships a sweater a full size off. Size set: **XS · S · M · L · XL · XXL (6)**, unisex **men's-derived block** (broader/straighter), published with a "size down for fitted / true-to-size for oversized" note. `[ESTIMATE: unisex-block convention, MED-HIGH]`

### 3b. The four knit measurements + M base (FINISHED, inches)
The four points knit QC and fit turn on, plus working secondaries. `[ESTIMATE: from public US sweater charts + 5–6" ease, MED-HIGH]`

| Measurement (finished) | M base | Note |
|---|---|---|
| **Chest** (½ flat; ×2 = full) | **42"** full (21" flat) | body chest ~38" M + ~4–5" ease → 42–43"; soft-boxy = top of band `[FACT — M body 38–40", 4–6" ease]` |
| **Body length** (HPS → hem) | **28"** | matches public M ~28"; crop variant −2" `[FACT]` |
| **Sleeve length** (shoulder → cuff) | **35.5"** | M ~35–35.5"; drop-shoulder shortens the set sleeve `[FACT]` |
| **Bottom hem width** (½) | **20"** flat | slightly under chest for a clean straight body; ribbed for recovery |

*Secondaries at M:* shoulder ~18.5", armhole depth ~10", cuff rib ~3.5", hem rib ~2.5–3" `[ESTIMATE: MED]`.

### 3c. The grade rule (the load-bearing math)
A **linear (even) grade** across XS–XXL — one increment per measurement, applied uniformly. Industry-standard knit increments `[FACT — chest 1.5–2"/size, body 1–1.5", sleeve 0.5–1"]`:

| Measurement | **Grade / size** | XS → XXL run (full / finished) |
|---|---|---|
| **Chest (full)** | **+2"** | **38 · 40 · 42 · 44 · 46 · 48** (cross-checked vs published US charts `[FACT]`) |
| **Body length** | **+1.5"** | 25.5 → 33 |
| **Sleeve length** | **+0.75"** | 33.25 → 37 |
| **Hem width (full)** | **+2"** | tracks chest to hold the straight silhouette |
| **Shoulder** | **+0.5"** | keeps proportion `[ESTIMATE: MED]` |

**Launch linear; let pop-up returns prove whether a non-linear break is needed at XS/XXL** — don't pre-engineer complexity the data hasn't justified. `[ESTIMATE: HIGH that linear-first is correct]`

### 3d. The knit-specific interaction (gauge ↔ grade ↔ shrinkage — the bit unique to knitwear)
A sweater is **knit to a stitch/row count, not cut to a pattern**, so the inch-grade must be **translated into whole wales/courses at production density** before it is programmable (`machines §6` — the programmer's job, the scarce bottleneck). The chain:
1. **Gauge sets the inch↔stitch conversion.** Illustrative `[ESTIMATE: arithmetic]`: at ~9 wales/inch finished, **+2" chest = +18 wales per panel side.** A 2" grade is **only valid if it lands on an integer number of wales** at the production density — round each graded dimension to the nearest achievable wale/course count.
2. **Loop length / stitch density is the lever** the programmer tunes to hit GSM *and* the dimension simultaneously (`SCOPE_GLOSSARY §4`). Spec **finished dimensions AFTER relaxation/steaming, measured on a washed swatch** — off-machine fabric is under tension and reads narrow/long.
3. **Shrinkage is a tolerance, not an afterthought.** **Build to the POST-WASH target** (knit slightly larger so it lands on-spec after relaxation), with **≤3–5% residual shrinkage** written into the PO `[ESTIMATE: MED-HIGH]`. **Per-fibre, the grade is shared but the knit-larger allowance is not:** **cotton (A1)** shrinks most + bags out (rib hems hard; consider compacting/sanforising), **merino (A3)** mills/relaxes more, **acrylic (A2)** is dimensionally stable but heat-sensitive (can't hard-press). Require **balanced ply to kill spirality** (the twisted-side-seam defect, `yarn §4a`).
4. **Tolerance every point** (e.g. chest ±0.5", length ±0.75", sleeve ±0.5") — out-of-tolerance = reject. **Without tolerances the grade rule is decorative.**

### 3e. Pop-ups = the fitting lab (the highest-value design input the brand owns)
The Turlock pop-ups are not just sales — they are a **fitting lab and the T7 size-curve link** (`brand_market §1/§4` flags the size curve as `[UNKNOWN until pop-ups run]`):
- **Capture the size curve:** record, per fitted customer, the size bought (+ ideally measured chest). The **distribution of sizes sold = the production size curve** (the XS:S:M:L:XL:XXL ratio to make). A wrong curve = dead stock in the tails + stock-outs in the middle = markdown.
- **Validate grade + ease on the *local* body population** — the Central-Valley, ~46%-Hispanic distribution **may not match the national US chart** `[ESTIMATE: MED]`; the linear grade and 5–6" ease are *launch hypotheses* the try-on validates or breaks.
- **Buy down returns:** online runs ~24% vs ~6% in-store and **size is the #1 apparel return reason** (`brand_market §4`); every offline fitting calibrates the online size guide.
- **Loop every drop:** pop-up curve → adjust grade/ease + production ratio → next drop. This is an **iteration protocol, not a one-time calibration.**

---

## 4. Trend ingestion → drop calendar (filter, don't follow)

### 4a. The three-filter funnel
The brand is colour-starved, fine-gauge, anti-pill-constrained and youth-fronted, so it **cannot chase runway literally.** Ingest through three filters; **only signals that survive all three AND fit the four constraints (§1) enter the line.** `[ESTIMATE: method, MED-HIGH]`
1. **Macro (slow, 12–18 mo) — WGSN/Coloro/Pantone + fashion-week knit** → direction: silhouette (boxy/oversized), texture (rib/cable revival), palette (pigment-return; Cloud Dancer anchor; eggplant/marigold/teal/red accents). Picks the 1–2 rotating accents and confirms the texture-over-colour bet.
2. **Meso (weeks–months) — Everlane/Quince/J.Crew/Massimo Dutti knitwear + Depop/thrift youth signal** → the "is it shipping, not just on a runway" check against the premium tier the brand prices against.
3. **Micro (real-time, days) — TikTok FYP / IG Reels / Pinterest** for *this* consumer → where the brand both **reads** and **publishes** (the AI-design process is itself the content).

A trend that needs 6 colours or a brushed-pill surface is **read, noted, and declined.**

### 4b. The Central-Valley-youth hype gap — owned here, refused honestly
`brand_market` Open Q#13 flags this as the open question owned by no upstream file. **This file owns the design half and states what is / isn't known:**
- **`[UNKNOWN]` (refused, not fabricated):** the *specific* live micro-trends/slang/sounds/local hype objects of Turlock/Central-Valley youth in mid-2026 — public trend reports skew coastal/national.
- **`[ESTIMATE: MED] channels:** TikTok FYP dominant, IG Reels, Pinterest, Depop/thrift, the CSU-Stanislaus social graph, bilingual/Hispanic cultural channels.
- **Close it with an instrument, not a guess:** a lightweight **listening loop** (saved knit + Central-Valley + CSU + Hispanic-creator watch) **+ the pop-ups as trend recon** (the same in-person channel that fixes the size curve) **+ own-channel sell-through by colour/style** as the truest local signal. **Pin this before a content calendar or hype-adjacent name ships** (the T7 instruction).

### 4c. The drop calendar (sell-window × production clock × trend clock)
Reconciles the **California sell-window** (light/transitional ~Sept–April, true heavy wool only Nov–Feb), the **Ludhiana production clock** (yarn-in → ex-factory ~6–12 wks; **don't commit into the Jul–Oct cluster peak**, `yarn §3c`) + ocean (T9), and the **trend clock** (accents picked 1–2 quarters ahead). `[ESTIMATE: structure MED-HIGH; dates MED]`

| Drop | Lands (CA) | Lead article | Order yarn by | Notes |
|---|---|---|---|---|
| **D1 — Fall core** | **Aug–Sep** | **A1** cotton core (+ A2 youth) | ~**May–Jun** | the LTV engine launches the season; longest sell-window. Order ahead of the cluster peak. |
| **D2 — Holiday / premium** | **Oct–Nov** | **A3** merino/lambswool capsule | ~**Jul–Aug** (pre-peak) | the thin true-cold window; halo/press/gifting; cable + premium colour. |
| **D3 — Youth / hype accent** | **Jan–Feb** | **A2** acrylic trend-accent re-colour | ~**Oct–Nov** | post-holiday youth churn; the rotating trend colour; fastest, most disposable read. |
| **D4 — Spring transitional** | **Mar–Apr** | **A1**-light (pointelle/jersey) | ~**Dec–Jan** | warm-CA spring fine-gauge; lighter stitch + palette. |

**Drop discipline:** small, timed, sometimes-limited releases manufacture scarcity/urgency (the hype-D2C tactic, `SCOPE_GLOSSARY §8`). **Re-confirm the trend accent at each order-by date** (the accent is the most perishable input); core/silhouette/grade are stable across drops.

---

## 5. The Flux generative method (the owner runs this himself)

### 5a. What Flux is in this pipeline — the framing that prevents the expensive mistake
Flux sits **upstream of the programmer**, at the *front* of `machines §5`'s "generative design → simulation → virtual-sampling" chain — the **idea/look-generation step, not the program.**
- **It IS:** a near-zero-marginal-cost **visual-ideation + mood + content engine** — and the *act of generating is the marketing* (the "AI-designed, made by hands" story pillar, literally).
- **It IS NOT:** a CAD/knit programmer, gauge calculator, grading engine, or any guarantee the rendered knit is knittable. **A Flux image has no stitch count, no GG, no yarn count, no measurement, no shrinkage behaviour — it will happily render an impossible knit.**
- **The brand-specific stake:** mistaking a Flux image for a spec ships an un-producible tech pack to Ludhiana and burns a **~2–5 wk sampling cycle + dye-lot money** (`yarn §3c`). **Flux de-risks the *idea*; it does not de-risk the *make*.**

### 5b. The eight-slot prompt skeleton (each slot pulls upstream vocabulary)
Flux is **guidance-distilled** → it prefers **descriptive prose over keyword-salad**, and **ignores true negative prompts on the dev/schnell path** (steer with positive phrasing, §5f). Order matters: lead with the garment, end with render controls. The **four constraints (§1) are baked into the slot choices** so generations stay producible-adjacent. `[FACT — BFL/HF + practitioner guides; SNAPSHOT on behaviour across versions]`

| # | Slot | What goes here | Source |
|---|---|---|---|
| 1 | **Garment + silhouette/fit** | crew/mock/cardigan/quarter-zip; soft-boxy / relaxed-straight / slightly-cropped; dropped shoulder; ~5–6" ease look | §2a |
| 2 | **Knit construction + stitch** | FF look, *visible stitch*; 2×2 rib full-body, half-Milano, cable, links-links, pointelle, jersey, plating; fashioning marks on A3 | `SCOPE_GLOSSARY §3` |
| 3 | **Gauge look** | "fine 12-gauge, tight even stitches, lightweight" — gauge as *visible stitch size*, never a number Flux understands | `yarn §1c` |
| 4 | **Colour + fibre/blend hand** | the 2–4-colour palette; fibre *appearance* ("matte soft cotton-blend," "lustrous fine merino," "smooth matte acrylic") — Flux renders hand/sheen, not fibre identity | §2a / `yarn §1b` |
| 5 | **Styling + model / no-model** | gender-neutral model worn open/layered/sleeves pushed, or flat-lay, or tech-flat; California-casual | §2a |
| 6 | **Shot type** | on-model / flat-lay / tech-flat — **pick ONE** (§5c) | §5c |
| 7 | **Photography + lighting** | soft natural daylight, clean studio, **warm Californian light, neutral backdrop — never alpine** (state it or Flux gives you Aspen) | `brand_market §3` |
| 8 | **Render controls** | positive-phrased quality + defect avoidance: "sharp focus, even knit stitch, correct hands with five fingers, plain unbranded garment, no logos, no text" | §5f |

### 5c. Shot type by job (run all three, at different stages)
- **On-model lookbook** → concept selection + content (most sell-y; but hides construction, hands are the #1 defect source, fit is *implied*).
- **Flat-lay / ghost-mannequin** → read actual garment shape/proportion/stitch/colour for spec hand-off (proportions visual, still no measurements).
- **Tech-flat / CAD-style front+back** → the closest Flux gets to a spec drawing — but an **illustration, not CAD**; **treat as a sketch to redraw properly.**
**Workflow:** on-model to *choose a direction* (+ make content) → then flat-lay + tech-flat of the chosen direction to *read it for hand-off*. Appealing shot first, buildable shot second.

### 5d. Parameters (all `[SNAPSHOT]` — re-confirm on the owner's live endpoint/version)
- **Guidance (distilled CFG) ~3.0–3.8** (~3.5 balance; higher → tighter adherence/flatter texture, lower → natural texture/looser brief; tech-flats can push 3.5–5) — the key "did it follow my stitch/colour brief" dial.
- **Steps ~24–32** (diminishing returns past ~32; low end for sweeps, high for finals).
- **Dimensions multiples of 32, ~768–1024/side** — portrait **832×1216** on-model, square **1024×1024** flat-lay, landscape for capsule group shots.
- **Seed = fix to iterate one variable, sweep to explore.**
- **Model tier:** schnell for cheap ideation sweeps → dev for the working look → pro for the hero/content final. `[FACT — Flux guides; SNAPSHOT on exact numbers]`

### 5e. The iteration loop (a disciplined sweep, not random re-rolls)
1. **Broad explore** (cheap tier): the §5b skeleton at **5–10 seeds, same prompt** → contact sheet → pick 1–2 directions.
2. **Lock the seed, sweep ONE slot** — stitch (rib → half-Milano → cable), then colour, then fit — "change one element at a time, keep seed fixed, so you can attribute the change." `[FACT — Flux best practice]`
3. **Tune guidance for fidelity** (ignoring the brief → nudge up ~3.5→4.2; plasticky → nudge down ~3.5→3.0).
4. **Finalize on the good tier** (dev/pro, steps ~32) for the hero + content asset.
5. **Re-render the chosen direction as flat-lay + tech-flat** for hand-off.

**Logging discipline (non-negotiable):** save **prompt + seed + guidance + steps + model + dims** with every keeper, or "you've made art, not a process."

### 5f. "Negative prompts" don't work on Flux-dev — steer with positive phrasing
Because the distilled dev/schnell path ignores true negatives, every avoidance is a **positive description of the correct state** `[FACT — guidance-distilled; SNAPSHOT]`:

| Defect | DON'T (ignored) | DO (works) |
|---|---|---|
| **Distorted hands / extra fingers** | "no deformed hands" | "**accurate hands, five separated fingers, relaxed at sides or in pockets**" — or sidestep: **hands in pockets / crop out / flat-lay.** Cheapest hand fix = no hands in frame. |
| **Fake logos / garbled text** | "no logos, no text" | "**plain unbranded garment, smooth fabric, no text, no logos, no labels, no graphics**" (also dodges the TM/right-of-publicity exposure T7 bans) |
| **Impossible / non-knittable knit** | (can't be negative-prompted — a *knowledge* gap, not a render flag) | constrain positively to real structures ("regular 2×2 rib," "standard evenly-repeated cable") **and catch the impossible ones in human spec review (§6), not the prompt** |

Advanced `true_cfg_scale>1` + a real negative prompt exists but ~doubles compute and is rarely needed if positive phrasing + shot choice are disciplined. `[FACT — true_cfg path; ESTIMATE usually unnecessary, MED]`

### 5g. Capsule consistency is engineered, not given
Flux has **no garment memory across calls.** Make A1/A2/A3 read as one line by: **reusing identical slots 5–8** (styling + shot + lighting + render controls) verbatim, varying only 1–4; forcing **shared cream/oat/charcoal anchor tokens** (mirrors the real shared-dye-lot economics, `yarn §3b`); holding a **seed family**; and *optionally* feeding a hero image via **img2img/IP-Adapter/Redux/"context"** *if the endpoint exposes it* (`[SNAPSHOT/UNKNOWN]` per endpoint — verify first). **Honest limit:** consistency is a **brand-look goal (achievable), not a spec-identity guarantee (not achievable)** — spec identity comes from the tech pack, not the image set.

---

## 6. The reusable template + worked A1/A2/A3 prompts (ready to run today)

### 6a. The fill-in-the-blanks TEMPLATE (copy, fill eight slots, run)
```
[SHOT TYPE] of a [SILHOUETTE/FIT] [GARMENT TYPE] sweater,
[KNIT CONSTRUCTION + STITCH STRUCTURE], [SELF-FINISH detail],
[GAUGE LOOK — fine/medium, stitch-density cue, weight cue],
in [COLOUR — body] [+ contrast tip/rib colour], a [FIBRE/BLEND HAND — matte/lustrous/soft] hand,
[STYLING — on a gender-neutral model worn ___ / laid flat / tech-flat front+back],
[PHOTOGRAPHY + LIGHTING preset — clean/light/Californian, never alpine],
sharp focus, even consistent knit-stitch gauge, realistic [fibre] texture,
[HANDS — five separated fingers relaxed / hands in pockets / no hands in frame],
plain unbranded garment, no logos, no text, no graphics.

— Params: model=[schnell→dev→pro]; guidance≈[3.0–3.8]; steps≈[24–32];
  dimensions=[832×1216 on-model / 1024×1024 flat-lay]; seed=[fix to iterate / sweep to explore].
```
*Slot order = §5b. Negatives are positive-phrased = §5f. The four constraints (≤4 colours / fine-gauge / few-seam / anti-pill) are baked into the choices = §1.*

**Three saved lighting presets (brand-locked, pin slot 7):**
- **E-comm clean:** "clean light-grey seamless studio backdrop, soft even diffused lighting, true-to-life colour, 85mm, sharp product focus."
- **California editorial:** "soft warm natural daylight, airy minimal setting, light wood / off-white tones, gentle shadows, 50mm editorial, relaxed Californian mood."
- **Flat-lay catalogue:** "top-down flat-lay on a clean neutral linen surface, even soft daylight, no harsh shadows, true colour."
- **NEVER:** "cozy cabin, fireplace, snow, alpine, chunky wool, autumn forest" — the cliché Flux defaults to that violates the brand read.

### 6b. A1 — cotton-blend 12GG core (margin engine — *evergreen, clean, not over-elaborated*)
**On-model (concept + content):**
> *"Full-length on-model lookbook photograph of a **relaxed-straight gender-neutral crew-neck pullover sweater**, **fully-fashioned fine-gauge knit with an allover soft 2×2 rib body and clean self-finished ribbed cuffs, hem and neck**, **fine 12-gauge tight even stitches, lightweight elevated-basic weight**, in **warm oat-cream with a subtle tonal contrast-tipped cuff**, a **soft matte cotton-blend hand**, **worn open and relaxed by a calm gender-neutral model, hands in pockets**, **soft warm natural Californian daylight, airy minimal off-white setting, 50mm editorial, true-to-life colour**, sharp focus, even consistent knit-stitch gauge, realistic soft cotton texture, **hands in pockets**, plain unbranded garment, no logos, no text, no graphics."*
> — `model=dev; guidance≈3.3; steps≈30; 832×1216; seed=FIX once chosen.`

**Tech-flat (spec hand-off):**
> *"Clean flat technical fashion drawing (tech-flat) of the same relaxed-straight crew-neck pullover, **front and back views**, allover 2×2 rib, ribbed crew neck/cuffs/hem, dropped shoulder, no model, flat vector line style, white background, no colour fill, no text."*
> — `model=dev; guidance≈4.0; steps≈28; 1024×1024; seed=reuse.`

*Spec note (the §6 fill):* A1 = cotton-rich blend, fine resultant count at **12GG**, ~200–350 g, **built to post-wash target** (cotton shrinks most → rib hems hard + consider compacting, §3d). The render shows *intent*; count/gauge/grade come from `yarn §1b` + the PPS swatch.

### 6c. A2 — fine-acrylic 7–12GG value/youth (front door — *loud-but-cheap, anti-pill-conservative*)
**On-model (the youth/hype content asset):**
> *"Editorial on-model lookbook photograph of a **soft-boxy slightly-cropped gender-neutral mock-neck pullover sweater**, **fully-fashioned fine-gauge knit with a crisp tight stitch and a clean 2×2 ribbed mock-neck, cuffs and cropped hem, smooth surface with no brushing and no long floats**, **fine 7–12-gauge even stitches, lightweight**, in **a bold trend accent — deep eggplant — with a cream contrast-tipped cuff**, a **smooth matte acrylic hand**, **worn by a youthful gender-neutral model, sleeves pushed up, hands relaxed at sides**, **clean light-grey seamless studio backdrop, soft even diffused lighting, 85mm, true colour**, sharp focus, even tight consistent knit-stitch gauge, **smooth crisp surface (not fuzzy, no pilling)**, accurate hands with five separated fingers, plain unbranded garment, no logos, no text, no graphics."*
> — `model=dev; guidance≈3.5; steps≈30; 832×1216; seed=FIX once chosen.`

*Spec note (the brand-critical trap):* A2 is the **pill kill-zone** — the prompt **forces a smooth crisp tight surface and forbids brushed/floated looks** because "one pilling video is brand-ending" (`brand_market §3`) and acrylic pills worst (`yarn §4`). **If a prettier *fuzzy* render appears, reject it for A2 no matter how good it looks** — anti-pill grade + tight gauge are the real defence (`yarn §4b`), not the image. The eggplant accent is the rotating trend colour.

### 6d. A3 — thin merino/lambswool Nov–Feb premium (halo / price-anchor — *cable, fashioning marks, premium colour*)
**On-model (the press/halo hero):**
> *"Premium editorial on-model photograph of a **refined relaxed gender-neutral crew-neck pullover sweater**, **fully-fashioned fine-gauge knit with a clean vertical cable pattern down the body and fine links-links purl texture, with visible fashioning marks at the raglan shaping and clean linked seams**, **fine 12-gauge even stitches, lightweight refined weight**, in **deep charcoal with a single tonal anchor**, a **soft lustrous fine-merino hand**, **worn by a poised gender-neutral model, hands relaxed, minimal styling**, **soft warm natural daylight, light wood and off-white tones, gentle shadows, 50mm editorial, true-to-life colour**, sharp focus, even consistent knit-stitch gauge, **realistic soft lustrous merino wool texture, neatly conserved evenly-repeated cable stitches**, accurate relaxed hands with five separated fingers, plain unbranded garment, no logos, no text, no graphics."*
> — `model=pro; guidance≈3.4; steps≈32; 832×1216; seed=FIX once chosen.`

*Spec note (the impossible-knit trap):* A3 carries **cable + fashioning marks** — the two most likely **"impossible knit"** offenders (cables must conserve stitches; fashioning marks must sit at real decrease lines). The prompt asks for *"neatly conserved evenly-repeated cable stitches,"* but **the spec reviewer + programmer must verify the cable is knittable and price in the extra yarn + linker time** (`SCOPE_GLOSSARY §3`). A3 = merino/lambswool 2/48 Nm-class at **12GG**, ~250–400 g, milled/relaxed (`yarn §1b`); even 5–10% cashmere or a few % nylon lifts hand/pill.

### 6e. Capsule-consistency pass (run after the three are chosen)
Reuse the **identical lighting + backdrop + model description + render-control text** across all three (vary only garment/stitch/gauge/colour, §5g), hold the **same seed family**, force the **shared cream/oat/charcoal anchors**, then render the **group/flat-lay capsule shot** (landscape AR) for the lookbook.

---

## 7. The bridge: Flux image → producible spec (the seam where design hands off to T2 + T3)

This is the load-bearing junction of the whole deliverable. **A Flux image carries real, usable design intent — but not a single buildability number.**

**What a Flux image CAN specify (approved direction, usable):** silhouette/proportion *direction*, stitch/texture *intent*, colour *story*/mood, detailing *intent*, styling/content direction. `[ESTIMATE: HIGH]`

**What a Flux image CANNOT specify — the four buildability inputs, each filled elsewhere** `[ESTIMATE: HIGH; load-bearing]`:

| Buildability input | Why Flux can't give it | Where it comes from → **whose job** |
|---|---|---|
| **1. Yarn count + fibre + blend ratio** (resultant Ne/Nm, ply, ends-fed) | renders apparent hand/sheen, not a count or fibre | `yarn §1b/§1c` quadruple + a live mandi/importer quote → **T2 (yarn)** |
| **2. True gauge (GG) + stitch density** (wales × courses/inch) | "fineness" is a look, not needles/inch — Flux has no GG | `SCOPE_GLOSSARY §3` + §3d + **measured on a physical PPS swatch** → **T2 + T3** |
| **3. Panel shaping + stitch-count grade** (FF inc/dec, courses, XS–XXL grade *in stitches*) | draws a shape, doesn't conserve stitches or know the grade rule; **renders impossible shaping** | §3c/§3d grade rule **translated into whole wales/courses by the CAD programmer** (`machines §6`) → **T3 (programmer)** |
| **4. Finished measurements + tolerances + shrinkage allowance** | no inches, ease, ±tol, or post-wash behaviour in a pixel | §3b/§3d finished spec, 5–6" ease, ≤3–5% residual shrink, per-fibre, ±tolerances → **T8 → tech pack** |

**Plus three things Flux actively gets WRONG (human catches at review, before money moves):**
- **Impossible knits** — non-stitch-conserving cables, ribs that change wale count, "seamless" joins no linker makes (A3's cable + fashioning marks are the top offenders).
- **Un-payable colourwork** — a 6-colour jacquard the 2–4-colour budget can't fund (`yarn §3b`).
- **Pill-prone acrylic surfaces** — a gorgeous brushed-fuzzy A2 render is a return machine; **down-rate it for A2 no matter how pretty.**

**The 8-step translation workflow (Flux image → tech pack → knittable garment):**
1. Choose direction from the **on-model** render (bank as content).
2. **Re-render flat-lay + tech-flat** of that direction.
3. **Human review vs the 4 constraints + 3 traps** — kill impossible/un-payable/pill-prone features *now*.
4. **Map intent onto the upstream specs** — fibre×count×gauge×weight quadruple (`yarn §1b`), stitch from `SCOPE_GLOSSARY §3`, finished-measurement base + grade (§3).
5. **CAD/flat-knit programmer authors the program** — needle actions, FF shaping, the **stitch-count grade** (human + machine-specific Shima APEX / Stoll M1plus, never the image, `machines §6`).
6. **Physical PPS swatch + sample = ground truth** — measures real gauge/density/shrinkage, confirms **ICI pilling ≥3–4 post-wash** (`yarn §4`). The swatch, not the render, is truth.
7. **Lock the tech pack** — measurements + tolerances + count/shade + GSM + stitch + trims + AQL (`SCOPE_GLOSSARY §7`).
8. **Pop-up fit calibration sets the size curve** (§3e) — the render never touches this; body data does.

**The one-line rule:** *the Flux image gets you from blank page → approved look + content; the tech pack + PPS swatch + programmer get you from approved look → knittable garment. Never skip the second half because the first half was cheap and pretty.*

**Four honest limits (binding):** **AI design → knit-program autonomy on our fabrics is `[UNKNOWN]` — test, don't assume** (`machines` Open Q#4); until proven, a human programmer + PPS swatch always close the loop. **Screen colour ≠ dye shade** (monitor RGB is mood, not a Pantone/lab-dip with lot-to-lot ΔE tolerance, `yarn §4d`). **Photogenic ≠ producible ≠ profitable** (anti-pill + T9 economics overrule the image every time). **No legal shortcut in the pixels** — AI renders/marketing carry the same CA §3344 / §3344.1 (70-yr) / Lanham §43(a) / Class-25 TM exposure (`brand_market §5`), so garments stay **plain/unbranded** and names **original/coined**, slogan-knit **Spanish-cleared**.

---

## Bottom line for the owner

1. **Design from 3 constraints, not a moodboard** — the 2–4-colour dye-lot ceiling, the 12GG warm-CA weight, and anti-pill + linker throughput → a **gender-neutral, soft-boxy, texture-over-colour** capsule that 2026 trend happens to reward. Carry looks with **stitch architecture (rib/half-Milano/cable/pointelle), not colour**; jacquard/intarsia mostly OUT (one intarsia hero, one premium drop, max). `[FACT/ESTIMATE blend, HIGH]`
2. **Per article:** A1 evergreen/clean (margin engine), A2 loud-but-cheap + anti-pill-conservative (front door — **one pilling video is brand-ending**), A3 cable/FF-marks/premium colour (halo/price-anchor). **Keep the loud youth front from becoming the house brand.** Exactly 3 = a **fibre/role ladder**, the smallest set covering all economic roles inside the colour + MOQ ceiling (54 SKUs already).
3. **Sizing:** XS–XXL, unisex men's block, **FINISHED-garment specs** (not body), ~5–6" ease, **linear grade (chest +2"/body +1.5"/sleeve +0.75"/hem +2")**, full-chest 38→48. **Translate to whole wales/courses at production density; build to the post-wash target on a washed swatch** with ≤3–5% residual shrink (per-fibre); **tolerance every point** or the grade is decorative. **Pop-ups = the fitting lab** that sets the production size curve on the local body and attacks the ~24% return rate — trust the local curve over the national chart.
4. **Trend ingestion = a 3-filter funnel**; only signals surviving all three AND the 4 constraints enter the line. The **Central-Valley-youth hype specifics are `[UNKNOWN]` (refused)**, channels are `[ESTIMATE]`; **close with a listening loop + pop-up recon + sell-through before a content calendar ships.** Drop calendar D1 Fall core / D2 Holiday premium / D3 Youth accent / D4 Spring transitional, ordered ahead of the Jul–Oct Ludhiana peak.
5. **Flux is an idea + content engine, not a knit compiler.** Prompt it as an **eight-slot fashion-photo brief encoding the same 4 constraints**, run **guidance-distilled (CFG ~3.0–3.8, ~24–32 steps, seed-locked one-variable iteration, log every keeper)**, avoid defects by **positive phrasing** (Flux-dev ignores true negatives), engineer capsule consistency (shared scaffold/palette/seed-family). **Use the ready-to-run template + A1/A2/A3 prompts in §6 today.** All params `[SNAPSHOT]` — re-confirm on the live endpoint.
6. **The producibility gap is the whole game:** a Flux pixel **cannot** specify the **four buildability inputs** (yarn count, true GG/density, stitch-count grade, finished measurements/shrinkage) and actively renders **impossible knits, un-payable colourwork, and pill-prone acrylic.** Bridge via choose on-model → flat-lay+tech-flat → human review vs 4 constraints + 3 traps → map intent onto **`yarn` (T2)** + sizing → **CAD programmer (T3) authors the program** → **PPS swatch = ground truth** → lock tech pack → pop-ups set the size curve.
7. **Central uncertainty:** **AI design → knit-program autonomy on our fabrics is `[UNKNOWN]` — test, don't assume.** Until proven, a human programmer + PPS swatch always close the loop. And: screen colour ≠ dye shade; **photogenic ≠ producible ≠ profitable**; **no legal shortcut in the pixels.**

---

## Open questions (floor, not ceiling)

1. **Local size curve** (XS:S:M:L:XL:XXL ratio for the ~46%-Hispanic Central-Valley body) — `[UNKNOWN until pop-ups]`; the production-quantity input (§3e; T7 link).
2. **Whether the national US sweater chart fits the local body distribution** / whether a non-linear grade break is needed at XS/XXL — `[UNKNOWN]`; pop-up returns decide.
3. **Specific 2026 Central-Valley-youth hype signals** (slang/sounds/objects) that should bias styling/colour — `[UNKNOWN]`, refused; closed by the listening loop + pop-up recon (`brand_market` Open Q#13).
4. **Production stitch density (wales × courses/inch) at our exact 12GG construction** — `[UNKNOWN until PPS sampling]`; converts both the inch-grade and any render's look into knittable stitch counts (`yarn` Open Q#11).
5. **Per-article residual shrinkage % + real post-wash hand** for our cotton/acrylic/merino constructions — `[UNKNOWN until PPS swatch]`; sets the knit-larger allowance.
6. **AI design → knit-program autonomy on our fabrics** (does the pipeline ever produce a runnable, correctly-graded program, or does Flux stay strictly upstream of the programmer?) — `[UNKNOWN]`; **T8 must test, not assume** (`machines` Open Q#4).
7. **Whether image-reference conditioning (img2img/IP-Adapter/Redux/"context") is available on the owner's specific Flux endpoint** — the capsule-consistency lever — `[SNAPSHOT/UNKNOWN]`; verify per endpoint.
8. **Exact live FLUX param ranges (guidance/steps/dims) for the owner's model version** — `[SNAPSHOT]`; starting values are dev-consensus, re-confirm.
9. **Real per-article return rate by size** for our constructions — `[UNKNOWN until selling]`; ~24% carried, gap robust (`brand_market` Open Q#5).
10. **Whether tech-flat renders are accurate enough to annotate directly or must be redrawn in proper CAD** — `[ESTIMATE: redraw, MED-HIGH]`.
11. **Whether a cheap Flux-tier ideation sweep + good-tier finals genuinely lowers cost vs all-pro** at the owner's volume — `[ESTIMATE: MED]`.

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/design/DEEPDIVE.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `SCOPE_GLOSSARY.md`; `FACTORY/turlock_business/sectors/food_dining/` (structural template); `design/reconciled.md`; `design/analyst_a_design_sizing.md`; `design/analyst_b_flux_prompting.md`; `brand_market/DEEPDIVE.md`; `yarn/DEEPDIVE.md`; `machines/DEEPDIVE.md`.
- **Delivered (owner-facing):** (1) the **design judgement** — the 3 (×4) upstream constraints that pre-decide the space, the texture-over-colour rule, the A1/A2/A3 fibre/role ladder with the loud-front/evergreen-behind architecture rule, and why exactly 3; (2) the **sizing + grading math** — XS–XXL finished-garment specs with 5–6" ease, the 4 knit measurements + M base, the linear grade rule (chest +2"/body +1.5"/sleeve +0.75"/hem +2", full-chest 38→48), the gauge↔grade↔shrinkage interaction (whole-wales translation, post-wash target, per-fibre allowance, tolerance every point), and the pop-up fitting-lab size-curve loop; (3) the **trend-to-drop pipeline** — the 3-filter funnel, the owned/refused Central-Valley-youth hype gap + its listening-loop closure, and the 4-drop calendar reconciling sell-window × production clock × trend clock; (4) the **Flux method** — what Flux is/isn't, the 8-slot skeleton, shot-type-by-job, guidance-distilled params + seed-locked iteration, positive-phrased defect avoidance, engineered capsule consistency, **a reusable fill-in template + ready-to-run A1/A2/A3 on-model + tech-flat prompts with params**; (5) the **image→producible-spec bridge** — the 4 buildability inputs Flux can't give mapped to who fills them (T2 yarn / T3 programmer / PPS swatch / tech pack), the 3 traps Flux gets wrong, and the 8-step translation workflow.
- **Honesty discipline preserved:** sizing/grade increments + US charts + 2026 colour/silhouette carried as `[FACT]`; all trend reads, channel inferences, ease/density/shrinkage allowances, price positioning, and Flux params tagged `[ESTIMATE]`/`[SNAPSHOT]`; the Central-Valley-youth hype specifics **refused as `[UNKNOWN]`, not fabricated**; the central `[UNKNOWN]` (AI→knit-program autonomy) kept as "test, don't assume"; correct US duties + legal exposure (right-of-publicity/Lanham/Class-25 applying to AI renders) kept off the line; 11 open questions as a floor.
- **Bottom line:** design from the constraints (which 2026 rewards), carry looks with stitch not colour, size on a linear finished-garment grade calibrated by pop-ups not a chart, prompt Flux as a constrained 8-slot brief, and **never confuse the cheap pretty image with the spec** — the four buildability inputs and the impossible-knit/pilling/colourwork traps are filled and caught by `yarn` (T2) + the sizing math + the CAD programmer (T3) + the PPS swatch, not by the pixels.
