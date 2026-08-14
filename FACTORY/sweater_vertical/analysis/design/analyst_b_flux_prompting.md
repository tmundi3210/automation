# T8 Design — Analyst B (AI Generative-Design + Flux-Prompting Methodology)

**For:** the owner, who will run image generation himself (Flux API, key in hand) for the AI-designed D2C knit capsule — **A1 cotton-blend 12GG 12-month core · A2 fine-acrylic 7–12GG value/youth driver · A3 thin merino/lambswool Nov–Feb premium capsule** (per `brand_market/DEEPDIVE.md`, `yarn/DEEPDIVE.md`). Analyst A (`design/analyst_a_design_sizing.md`) owns *what to design* (silhouette/stitch/colour/grade math/trend/line-plan). **This file owns *how to make Flux render it* — ready-to-run prompts + the method — and the honest gap between a pretty image and a knit program.**

**Question this answers (four parts):** (1) How to **structure a Flux prompt for a knitwear design** — garment, knit construction/stitch, gauge look, colour/blend, silhouette/fit, styling, on-model vs flat-lay vs tech-flat, photography/lighting, and "negative" handling. (2) **Parameters + the iteration loop** — seeds, aspect ratio, guidance, steps; how to iterate a chosen direction into variations and hold consistency across a capsule. (3) **The gap between a generated image and a PRODUCIBLE design** — what Flux can and cannot specify, and how to translate a chosen image into a real spec the knitter/CAD-programmer (T3) + yarn (T2) can execute. (4) A reusable **prompt TEMPLATE** + worked example prompts for A1/A2/A3.

**How to read the tags (binding honesty discipline, per `SCOPE_GLOSSARY`):**
- **[FACT]** = cited public source / true-by-definition.
- **[ESTIMATE]** = reasoned, carries method + basis + confidence + the literal word *estimate*.
- **[UNKNOWN]** = genuinely not known; a valid, frequent answer. Coverage is a **floor, not a ceiling**.
- **[SNAPSHOT]** = true at a point in time but **moving** (model versions, API params) — re-confirm before it drives spend.

> **The one sentence to remember.** *Flux is an **idea engine, not a knit compiler** — it renders a *photograph of a sweater that does not exist and never knit a single stitch*, so prompt it like a **fashion photo brief** (garment → construction/stitch → gauge-look → colour → silhouette/fit → styling → shot-type → lighting), run it **guidance-distilled** (distilled-CFG ~3.0–3.8 for photoreal, ~24–32 steps, fixed seed to lock a look then sweep seeds for variants), **steer away from defects with POSITIVE phrasing not a negative prompt** (Flux-dev ignores true negatives), and treat every chosen image as a **mood/spec brief that a human must translate into a tech pack** — because the four things that actually make the garment producible (resultant yarn count, true GG, panel shaping/stitch-count grade, finished measurements) are **exactly the four things a Flux pixel cannot specify** and must be filled in from `yarn/DEEPDIVE.md` + `design/analyst_a` + a physical PPS swatch.* `[ESTIMATE: synthesis, HIGH]`

---

## 0. What Flux is, in this pipeline (the framing that prevents the expensive mistake)

`machines/DEEPDIVE.md §5` already drew the hard line and this file inherits it: **"Generative AI → finished garment, no programmer" is overstated; AI drafts, a human programmer + physical sampling close the loop**, and *design→knit-program autonomy on our fabrics is `[UNKNOWN]` — T8 must test, not assume.* Flux sits **upstream of that line**. Its job is the *front* of `machines §5`'s "generative design → simulation → virtual-sampling" chain — the **idea/look generation** step, not the program. Concretely:

- **What Flux IS here:** a near-zero-marginal-cost **visual ideation + mood/concept + content engine** that turns a text brief into a photoreal "lookbook shot" of a sweater. It is the literal embodiment of the brand's **"AI-designed, made by hands" story pillar** (`brand_market §3`) — and the *process of generating* is itself the TikTok/IG-native content (`brand_market §3`, `analyst_a §1b`). Two payoffs in one tool: (a) it drafts the look, (b) the draft-making *is* marketing.
- **What Flux IS NOT:** a CAD/knit programmer, a gauge calculator, a grading engine, a fabric simulator, or a guarantee that the rendered knit is *physically knittable*. It will happily render an **impossible knit** (a cable that doesn't conserve stitches, a rib that changes wale count mid-panel, a "seamless" join no linker could make). **A Flux image has no stitch count, no GG, no yarn count, no measurement, no shrinkage behaviour** — those live in `yarn/DEEPDIVE.md` and `analyst_a §2`, and are added by a human afterward (§3).
- **Why this matters for *this* brand specifically:** the whole venture's edge is *product + AI-design + export-legitimacy* (`machines §3`). If the owner mistakes a Flux image for a spec, he ships an un-producible tech pack to Ludhiana and burns a sampling cycle (~2–5 wks, `yarn §3c`) and dye-lot money on a knit that can't be made or that pills. **Flux de-risks the *idea*; it does not de-risk the *make*.** Keep the two ledgers separate.

---

## 1. How to structure a Flux prompt for a knitwear design

### 1a. The model reality that shapes everything (read before writing a single prompt)
Flux (Black Forest Labs FLUX.1, `dev`/`pro`/`schnell` tiers) is a **guidance-distilled** flow model. Three consequences drive the whole method `[FACT — BFL/HF docs + practitioner guides, see Sources; SNAPSHOT: model behaviour moves across versions]`:
1. **It prefers natural-language, descriptive prose over keyword-salad.** Comma-stacked SD-style tag soup ("sweater, knit, 8k, masterpiece, trending") works worse than a coherent sentence-y brief that reads like a **photographer's shot description**. Write *scenes and garments*, not tags. `[FACT — Flux prompt guides]`
2. **It does not take a true negative prompt** on the distilled `dev`/`schnell` path (CFG≈1, no negative). You steer **away** from defects by **positively describing the correct state** ("five clearly separated fingers, hands relaxed at sides," "plain unbranded garment, no text") — see §1g. True classifier-free guidance (a real negative prompt) is only available where the pipeline exposes `true_cfg_scale > 1` **with** a negative prompt, which costs ~2× compute and is an advanced/optional path. **Default: positive-phrasing negatives.** `[FACT — Flux is guidance-distilled; true_cfg workaround]`
3. **Its handling of hands and fine fabric texture is comparatively strong** but not perfect — detailed positive description of the texture and the hands is what gets you there. `[FACT — Flux guides; ESTIMATE: still verify every render, HIGH]`

### 1b. The eight-slot knitwear prompt skeleton (the spine — fill every slot, in this order)
Order matters: lead with the **subject/garment**, end with **render controls**. Each slot maps to a vocabulary the *upstream files already fixed*, so the prompt is constrained, not freestyled. `[ESTIMATE: method, HIGH]`

| # | Slot | What goes here | Source of the vocabulary |
|---|---|---|---|
| 1 | **Garment type + silhouette/fit** | crew/mock pullover, cardigan, quarter-zip; soft-boxy / relaxed-straight / slightly-cropped; dropped-shoulder; ~5–6" ease look | `analyst_a §1c` (silhouette block) |
| 2 | **Knit construction + stitch structure** | fully-fashioned look, *visible knit stitch*; the specific stitch: 2×2 rib full-body, half-Milano, cable, links-links/purl, pointelle, jersey, plating; fashioning marks on A3 | `SCOPE_GLOSSARY §3`, `analyst_a §1d` |
| 3 | **Gauge look (the fineness cue)** | "fine-gauge 12GG fine knit, tight even stitches, lightweight" vs "medium 7GG" — gauge is rendered as *visible stitch size/density*, never as a number Flux understands | `SCOPE_GLOSSARY §3`, `yarn §1c` |
| 4 | **Colour + fibre/blend hand** | the 2–4-colour palette (cream/oat/charcoal anchor + one accent); fibre *appearance* ("soft matte cotton-blend," "lustrous fine merino," "matte acrylic, smooth") — Flux renders *hand/sheen*, not fibre identity | `yarn §1b/§3b`, `analyst_a §1e` |
| 5 | **Styling + model/no-model** | gender-neutral model, styling (worn open, layered, sleeves pushed), or flat-lay, or tech-flat; California-casual context | `analyst_a §1c`, `brand_market §3` |
| 6 | **Shot type** | **on-model lookbook** vs **flat-lay** vs **tech-flat/CAD-style front+back** (§1e) — pick ONE per render | §1e below |
| 7 | **Photography + lighting style** | soft natural daylight, clean studio, warm Californian light, neutral seamless backdrop, 50–85mm look, editorial/e-comm | `brand_market §3` ("clean/light/Californian, never alpine") |
| 8 | **Render controls (positive-phrased quality + defect-avoidance)** | "sharp focus, accurate even knit stitches, realistic wool texture, correct hands with five fingers, plain unbranded garment, no logos, no text" | §1g |

**The brand-story constraint that pins slot 7:** every render must read **clean / light / Californian, never alpine/heritage-wool** (`brand_market §3`). That single rule kills the default "cozy cabin / snow / fireplace" knitwear cliché Flux reaches for — so *state the Californian light and neutral backdrop explicitly* or the model will give you Aspen.

### 1c. The four design constraints, encoded into the prompt (so generations stay producible-adjacent)
`analyst_a §1a` fixes four upstream constraints; encode each so Flux doesn't render something the line can't pay for `[ESTIMATE: HIGH]`:
- **≤2–4 colours** → prompt **single-colour-body with at most a contrast rib/tip**; never prompt multi-colour Fair-Isle/jacquard florals (the dye-lot budget can't fund them, `yarn §3b`). "Solid oat-cream body, tonal ribbed cuffs."
- **12GG warm-CA weight** → prompt **fine-gauge, lightweight, fine even stitches**; explicitly *not* "chunky/bulky/heavy 3GG." Fake substance with *surface stitch* ("cable texture at a fine gauge," "dense half-Milano face"), not bulky yarn — exactly `analyst_a §1d`'s trick.
- **Linker bottleneck** → prefer **clean, few-seam silhouettes**; don't prompt elaborate multi-panel colour-block joins (each is linker minutes, `machines §1`).
- **Anti-pill on acrylic A2** → for A2, prompt **smooth tight stitch, no long floats, no brushed/fuzzy surface** (brushed/floated surfaces pill fastest, `yarn §4`; "a single pilling video is brand-ending," `brand_market §3`). A2's renders should look *crisp*, not *cozy-fuzzy*.

### 1d. Worked anatomy of one prompt (annotated)
> *"Editorial e-commerce photograph of a **gender-neutral soft-boxy crew-neck pullover sweater** [1], **fully-fashioned fine-gauge knit with an allover 2×2 rib texture and clean self-finished ribbed cuffs and hem** [2], **fine 12-gauge tight even stitches, lightweight elevated-basic weight** [3], in **soft warm oat-cream, a matte soft cotton-blend hand** [4], **worn by a relaxed gender-neutral model, sleeves slightly pushed up, styled minimally** [5], **full-length on-model lookbook shot** [6], **soft natural Californian daylight, clean light-grey seamless studio backdrop, 85mm editorial look, true-to-life colour** [7], **sharp focus, realistic even knit-stitch texture, accurate relaxed hands with five fingers, plain unbranded garment with no logos and no text** [8]."*

Every bracket is a slot from §1b; every adjective traces to an upstream file. **This is the unit of work** — not a moodboard, a *constrained brief*.

### 1e. On-model vs flat-lay vs tech-flat — pick the shot for the *job* (run all three at different stages)
The three shot types are **not interchangeable**; each serves a different stage of the pipeline `[ESTIMATE: method, HIGH]`:

| Shot type | Prompt cue | Use it for | Limit |
|---|---|---|---|
| **On-model lookbook** | "worn by a model, full-length editorial photo, studio/outdoor CA light" | **Concept selection + content** (the TikTok/IG asset, the brand-story render). Most flattering, most *sell-y*. | Hides construction detail; model pose/hands are the #1 defect source (§1g); fit is *implied*, not measured. |
| **Flat-lay / ghost-mannequin** | "flat-lay product photo of the sweater laid flat on a neutral surface, top-down, even lighting" or "invisible-mannequin product shot" | **Reading the actual garment shape, proportions, stitch, colour** for the spec hand-off. Cleaner for translating to a tech pack. | Still no measurements; proportions are *visual*, not dimensional. |
| **Tech-flat / CAD-style** | "flat technical fashion sketch / tech-flat line drawing of the sweater, front and back, clean vector style, no model, white background" | **The closest Flux gets to a spec drawing** — front+back schematic the tech pack can annotate. | Flux's tech-flats are *illustrations*, not CAD; not dimensionally accurate, not a knit program; treat as a *sketch to redraw properly* (§3). |

**The workflow:** **on-model** to *choose a direction* (and to make content), then **flat-lay + tech-flat** of the chosen direction to *read the garment for spec hand-off*. Generate the appealing shot first, the buildable shot second.

### 1f. Photography / lighting presets (reusable, brand-locked)
Three saved lighting blocks, each enforcing `brand_market §3`'s "clean/light/Californian" `[ESTIMATE: MED-HIGH]`:
- **E-comm clean:** "clean light-grey seamless studio backdrop, soft even diffused lighting, true-to-life colour, 85mm, sharp product focus."
- **California editorial:** "soft warm natural daylight, airy minimal setting, light wood / off-white tones, gentle shadows, 50mm editorial, relaxed Californian mood."
- **Flat-lay catalogue:** "top-down flat-lay on a clean neutral linen surface, even soft daylight, no harsh shadows, true colour."
Never: "cozy cabin, fireplace, snow, alpine, chunky wool, autumn forest" — the cliché Flux defaults to that violates the brand read.

### 1g. "Negative" prompts on Flux — how to actually avoid distorted hands, fake logos, impossible knits
Because Flux-dev **ignores a true negative prompt** (§1a), every avoidance is a **positive description of the correct state** `[FACT — guidance-distilled]`. The three brand-critical defect classes and their positive fixes:

| Defect to avoid | DON'T (true negative — Flux-dev ignores it) | DO (positive phrasing that works) |
|---|---|---|
| **Distorted hands / extra fingers** | "no deformed hands, no extra fingers" | "**accurate hands with five separated fingers, hands relaxed at sides or in pockets**" — or sidestep entirely: **crop the hands out / hands in pockets / flat-lay with no model.** The cheapest hand fix is *no hands in frame.* |
| **Fake logos / invented brand text / garbled lettering** | "no logos, no text" *(partially works as positive)* | "**plain unbranded garment, smooth fabric with no text, no logos, no labels, no graphics**" — Flux invents garbled text on knitwear; explicitly call the garment *plain/unbranded*. (Also dodges the TM/right-of-publicity exposure `brand_market §5` bans.) |
| **Impossible / non-knittable knit** | (can't be negative-prompted away — it's a *knowledge* gap, not a render flag) | **Constrain positively to real structures** from `SCOPE_GLOSSARY §3` ("regular 2×2 rib," "standard 6-stitch cable, evenly repeated," "fully-fashioned raglan with clean linked seams") — and **catch the impossible ones in human review (§3), not in the prompt.** Flux *will* render physically-impossible knits convincingly; the prompt cannot fully prevent it, the *spec reviewer* must. |

Other useful positive-quality anchors: "even consistent stitch gauge across the whole garment," "symmetric sleeves of equal length," "realistic seam and rib alignment." `[ESTIMATE: MED-HIGH]`

**The advanced option (only if defects persist):** if the API/pipeline exposes `true_cfg_scale`, set it `>1` and supply a real negative prompt ("deformed hands, extra fingers, text, logo, blurry, distorted knit") — but this ~doubles inference cost and is rarely needed if the positive phrasing + shot choice (hands out of frame) is disciplined. `[FACT — true_cfg path; ESTIMATE: usually unnecessary, MED]`

---

## 2. Parameters + the iteration loop

### 2a. The core parameters (starting values — all `[SNAPSHOT]`, confirm against the live API)
All values below are **practitioner-consensus starting points for FLUX.1-dev**, not magic numbers; **the literal numbers move across model versions and endpoints — treat as `[SNAPSHOT]` and re-confirm.** `[FACT — Flux prompt guides, see Sources; SNAPSHOT]`

| Parameter | Starting value | Why / how to use |
|---|---|---|
| **Guidance (distilled CFG)** | **~3.0–3.8** for photoreal lookbook/e-comm; ~3.5 default balance. Higher (→5) = tighter prompt adherence but flatter/less natural texture; lower (→2) = more natural fabric but looser to the brief. For tech-flats/sketches you can push 3.5–5. | The single most useful dial for "did it follow my stitch/colour brief vs invent its own." `[FACT — dev guidance 1.5–5 range, ~3.5 balance]` |
| **Steps (inference)** | **~24–32** | More steps = finer texture/cleaner stitches up to a point; diminishing returns past ~32. Use the low end for fast ideation sweeps, high end for the chosen final. `[FACT]` |
| **Aspect ratio / dimensions** | dimensions = **multiples of 32**, ~768–1024 per side. **Portrait (e.g. 832×1216, ~2:3 or 3:4)** for on-model; **square (1024×1024)** for flat-lay/product; **landscape** only for capsule/group shots. | Match AR to subject: vertical for people, square for products (§1e). `[FACT — dims multiple of 32, 256–1440 range on dev API]` |
| **Seed** | integer; **fix it to test, sweep it to explore** | **The consistency lever (§2c).** Fixed seed + same prompt = reproducible image; fixed seed + one changed word = isolate that word's effect; new seed + same prompt = a different "take" of the same brief. `[FACT]` |
| **Model tier** | **schnell** for cheap fast ideation sweeps; **dev** for the working look; **pro** for the final hero/content render | Cost/quality ladder; do the 50-image exploration on the cheap tier, the 3 finalists on the good one. `[ESTIMATE: MED-HIGH]` |

### 2b. The iteration loop (the disciplined sweep, not random re-rolls)
The cardinal rule from every Flux guide: **change ONE element at a time and keep the seed fixed so you can attribute the change.** `[FACT — Flux best practice]` The loop `[ESTIMATE: method, HIGH]`:

1. **Broad explore (cheap tier, varying seeds):** run the §1b skeleton for one article with **5–10 different seeds** at the *same* prompt → a contact sheet of distinct "takes." Pick the 1–2 directions that read right.
2. **Lock the seed, sweep one slot:** fix the best seed; now change **only the stitch** (rib → half-Milano → cable) across renders, then **only the colour**, then **only the fit** (boxy → cropped). Each sweep isolates one design variable — this is how you *design*, not gamble.
3. **Tune guidance for fidelity:** if it's ignoring your stitch/colour brief, nudge guidance up (~3.5→4.2); if the texture looks plasticky/flat, nudge down (~3.5→3.0). One change at a time.
4. **Finalize on the good tier:** take the chosen seed+prompt to **dev/pro**, steps ~32, for the hero render (and the content asset).
5. **Generate the buildable shots:** re-render the *chosen direction* as **flat-lay + tech-flat** (§1e) for the spec hand-off (§3).

**Logging discipline (non-negotiable):** save **prompt + seed + guidance + steps + model + dimensions** with every kept image. Without the seed+params you cannot reproduce or iterate a winner — you've made art, not a process. A simple sheet (or the API's metadata) is enough. `[ESTIMATE: HIGH]`

### 2c. Consistency across a capsule (the hard part — how to make A1/A2/A3 look like one brand)
A capsule must read as *one line*, not three unrelated images. Flux has **no built-in character/garment memory** across calls, so consistency is engineered `[ESTIMATE: method, MED-HIGH; SNAPSHOT — feature support moves]`:
- **Shared prompt scaffold:** reuse the *same* slots 5–8 (styling + shot-type + lighting + render-controls) verbatim across all three articles; vary only slots 1–4 (garment/stitch/gauge/colour). Same light, same backdrop, same model description = same brand world.
- **Shared palette tokens:** force the **2 evergreen anchors** (cream + oat/charcoal, `analyst_a §1e`) as literal colour words reused across articles so the capsule shares its base colours (which also mirrors the real **shared-dye-lot** economics, `yarn §3b`).
- **Seed family:** keep the *same seed* (or a tight seed range) across the three article prompts when you want maximally similar framing/model/light; this is the cheapest consistency lever. `[ESTIMATE: MED]`
- **Reference-conditioning tools (optional, version-dependent):** if the endpoint exposes **image-to-image / IP-Adapter / FLUX Redux / structural reference / "context" conditioning**, feed a chosen hero image as a style/structure reference so subsequent renders inherit its look. **Availability and exact names are `[SNAPSHOT]`/`[UNKNOWN]` per endpoint — verify before relying on it.** `[ESTIMATE: these features exist in the Flux ecosystem, HIGH; their availability on the owner's specific endpoint, UNKNOWN]`
- **The honest limit:** even with all of the above, Flux consistency is *approximate* — the same sweater rendered twice differs in stitch count, exact proportion, and detail. **For a capsule, consistency is a brand-look goal (achievable) not a spec-identity guarantee (not achievable).** The spec identity comes from the tech pack (§3), not the image set.

---

## 3. The gap between a generated image and a PRODUCIBLE design (the section that prevents a wasted sampling cycle)

This is the core honesty deliverable: **what a Flux image CAN and CANNOT specify, and how to translate a chosen image into a real spec** the yarn buyer (T2), knitter, and CAD-programmer (T3) can execute.

### 3a. What a Flux image CAN specify (legitimately usable inputs)
A chosen render *does* carry real, usable design intent — treat these as **approved direction** `[ESTIMATE: HIGH]`:
- **Silhouette & proportion direction** — boxy vs fitted, crop vs length, neckline, sleeve style. (Visual, not dimensional — but a valid *target* for the fit block, `analyst_a §1c/§2`.)
- **Stitch / texture *intent*** — "we want an allover 2×2 rib," "a cabled yoke," "a pointelle spring look." (The *idea* of the stitch, `SCOPE_GLOSSARY §3`.)
- **Colour *story* / palette mood** — which 2–4 colours, which is the body and which the contrast tip. (The mood; the *actual dye shade* is a Pantone/lab-dip decision, not a screen colour, §3c.)
- **Detailing intent** — contrast-tip rib, fashioning-mark look, self-finished hems, no-logo aesthetic (`analyst_a §1f`).
- **Styling / content direction** — how it's worn, the brand photo world (the marketing payoff).

### 3b. What a Flux image CANNOT specify (the four producibility gaps — must be filled by humans/files)
These are **exactly the four things that make a knit buildable**, and **none of them is in a Flux pixel** `[ESTIMATE: HIGH; this is the load-bearing point of the file]`:

| Producibility input | Why Flux can't give it | Where it actually comes from |
|---|---|---|
| **1. Yarn count + fibre + blend ratio (resultant Ne/Nm, ply, ends-fed)** | Flux renders *apparent hand/sheen*, not a count or a fibre. "Looks like soft merino" ≠ a 2/48 Nm spec. | `yarn/DEEPDIVE §1b/§1c` (fibre×count×gauge×weight quadruple) + a live mandi/importer quote (`yarn §3`). |
| **2. True gauge (GG) + stitch density (wales×courses/inch)** | The image's "fineness" is a *look*, not needles-per-inch. Flux has no GG. | `SCOPE_GLOSSARY §3` + `analyst_a §2d` + **measured on a physical PPS swatch** (`yarn` Open Q#11, `analyst_a` Open Q#4). |
| **3. Panel shaping + stitch-count grade (FF increases/decreases, courses for length, the XS–XXL grade in *stitches*)** | Flux draws a shape, not a knittable stitch geometry; it does not conserve stitches or know the grade rule. **It will render impossible shaping.** | `analyst_a §2c/§2d` grade rule **translated into whole wales/courses by the CAD-programmer** (`machines §6`). This is the programmer's job, the scarce bottleneck. |
| **4. Finished measurements + tolerances + shrinkage allowance** | No image has inches, ease, ±tolerance, or post-wash shrink behaviour. | `analyst_a §2b/§2d` (finished-garment spec, 5–6" ease, ≤3–5% residual shrink, ±tolerances) — built to a **post-wash target on a washed swatch**, per fibre. |

**Plus three things Flux actively gets WRONG and a human must catch:**
- **Impossible knits** — non-stitch-conserving cables, ribs that change wale count, "seamless" joins no linker makes. **Caught in spec review, not the prompt (§1g).**
- **Un-payable colourwork** — Flux happily renders 6-colour jacquard the 2–4-colour dye budget can't fund (`yarn §3b`). Reject at review.
- **Pill-prone surfaces on acrylic** — a gorgeous brushed-fuzzy A2 render is a **return machine** (`yarn §4`); the look must be down-rated for A2 even if it's the prettiest image.

### 3c. The translation workflow — Flux image → tech pack (the hand-off, step by step)
The bridge from a chosen render to something Ludhiana can knit `[ESTIMATE: method, HIGH]`:
1. **Choose the direction from the on-model render** (and bank it as content).
2. **Re-render flat-lay + tech-flat** of that direction (§1e) to read the garment cleanly.
3. **Human design review against the 4 constraints + the 3 "Flux-gets-wrong" traps** (§3b) — kill impossible/un-payable/pill-prone features *now*, before money moves.
4. **Map the image's *intent* onto the upstream specs:** pick the fibre×count×gauge×weight quadruple (`yarn §1b`), the stitch from `SCOPE_GLOSSARY §3`, the finished-measurement base + grade (`analyst_a §2`).
5. **Hand the *intent + the upstream numbers* to the CAD/flat-knit programmer** (`machines §6`), who authors the actual needle actions, FF shaping, and the **stitch-count grade** — *this is where the design becomes a program, and it is human+machine-specific (Shima APEX vs Stoll M1plus), never the image.*
6. **Physical PPS swatch + sample** (`yarn §3c`, proto→fit→PP) measures the *real* gauge/density/shrinkage and confirms the look survives a wash (ICI pilling ≥3–4 post-wash, `yarn §4`). **The swatch, not the render, is ground truth.**
7. **Lock the tech pack** (`SCOPE_GLOSSARY §7`): measurements + tolerances + yarn count/shade + GSM + stitch + trims + AQL — the precise hand-off `brand_market`/`yarn` both depend on.
8. **Pop-up fit calibration** sets the **size curve** (`analyst_a §2e`) — the render never touches this; the body data does.

**The one-line rule for the owner:** *the Flux image gets you from "blank page" to "approved look + content"; the tech pack + PPS swatch + programmer get you from "approved look" to "knittable garment." Never skip the second half because the first half was cheap and pretty.* `[ESTIMATE: HIGH]`

### 3d. Honest limits, stated plainly (binding)
- **Flux designs *ideas*, it does not produce a knit program.** (Inherited and reaffirmed from `machines §5`: AI drafts; programmer + sampling close the loop; autonomy on our fabrics `[UNKNOWN]`.)
- **A Flux render is a *hypothesis about a look*, not a *guarantee of a make*** — its producibility is `[UNKNOWN]` until the PPS swatch proves it.
- **Screen colour ≠ dye shade.** Monitor RGB is not a Pantone/lab-dip; the actual shade is a yarn/dye decision with lot-to-lot ΔE tolerance (`yarn §4d`), not a pixel.
- **Photogenic ≠ producible ≠ profitable.** The prettiest A2 render may be the most pill-prone and highest-return; the line's economics (T9) and the anti-pill gate (`yarn §4`) overrule the image every time.
- **No legal shortcut in the pixels.** Do not prompt real athlete/celebrity/team/event names or their likenesses into renders or "inspired-by" content — the right-of-publicity / Lanham §43(a) / Class-25 TM stack (`brand_market §5`: §3344 + §3344.1 70-yr + false-endorsement + registered marks) applies to **AI-generated images and marketing just as to any other use**, and a generated face/name that *reads as* a real person is exposure. Keep garments **plain/unbranded** (§1g) and names **original/coined** (`brand_market §5b`). `[FACT/ESTIMATE blend, HIGH on athlete=no]`

---

## 4. The reusable prompt TEMPLATE + worked examples (A1 / A2 / A3)

### 4a. The fill-in-the-blanks TEMPLATE (copy, fill the eight slots, run)
```
[SHOT TYPE] of a [SILHOUETTE/FIT] [GARMENT TYPE] sweater,
[KNIT CONSTRUCTION + STITCH STRUCTURE], [SELF-FINISH detail],
[GAUGE LOOK — fine/medium, stitch density cue, weight cue],
in [COLOUR — body] [+ contrast tip/rib colour], a [FIBRE/BLEND HAND — matte/lustrous/soft] hand,
[STYLING — on a gender-neutral model worn ___ / laid flat / tech-flat front+back],
[PHOTOGRAPHY + LIGHTING preset — §1f],
sharp focus, even consistent knit-stitch gauge, realistic [fibre] texture,
[HANDS handling — five separated fingers relaxed / hands in pockets / no hands in frame],
plain unbranded garment, no logos, no text, no graphics.

— Params: model=[schnell→dev→pro]; guidance≈[3.0–3.8]; steps≈[24–32];
  dimensions=[832×1216 on-model / 1024×1024 flat-lay]; seed=[fix to iterate / sweep to explore].
```
**Slot order = §1b. Negatives are positive-phrased = §1g. Constraints (≤4 colours / fine-gauge / few-seam / anti-pill) are baked into the choices = §1c.**

### 4b. A1 — cotton-blend 12GG 12-month core (margin/LTV engine — *evergreen, clean, not over-elaborated*)
**On-model (concept + content):**
> *"Full-length on-model lookbook photograph of a **relaxed-straight gender-neutral crew-neck pullover sweater**, **fully-fashioned fine-gauge knit with an allover soft 2×2 rib body and clean self-finished ribbed cuffs, hem and neck**, **fine 12-gauge tight even stitches, lightweight elevated-basic weight**, in **warm oat-cream with a subtle tonal contrast-tipped cuff**, a **soft matte cotton-blend hand**, **worn open and relaxed by a calm gender-neutral model, hands in pockets**, **soft warm natural Californian daylight, airy minimal off-white setting, 50mm editorial, true-to-life colour**, sharp focus, even consistent knit-stitch gauge, realistic soft cotton texture, **hands in pockets**, plain unbranded garment, no logos, no text, no graphics."*
> — `model=dev; guidance≈3.3; steps≈30; 832×1216; seed=FIX once chosen.`

**Tech-flat (spec hand-off):**
> *"Clean flat technical fashion drawing (tech-flat) of the same relaxed-straight crew-neck pullover, **front and back views**, allover 2×2 rib, ribbed crew neck/cuffs/hem, dropped shoulder, no model, flat vector line style, white background, no colour fill, no text."*
> — `model=dev; guidance≈4.0; steps≈28; 1024×1024; seed=reuse.`

*Spec note (the §3b fill):* A1 = cotton-rich blend, fine resultant count at **12GG**, ~200–350 g, built to post-wash target (cotton shrinks most → rib hems hard + consider compacting, `analyst_a §2d`). The render shows *intent*; the count/gauge/grade come from `yarn §1b` + the PPS swatch.

### 4c. A2 — fine-acrylic 7–12GG value/youth driver (acquisition front door — *loud-but-cheap, anti-pill-conservative*)
**On-model (the youth/hype content asset):**
> *"Editorial on-model lookbook photograph of a **soft-boxy slightly-cropped gender-neutral mock-neck pullover sweater**, **fully-fashioned fine-gauge knit with a crisp tight stitch and a clean 2×2 ribbed mock-neck, cuffs and cropped hem, smooth surface with no brushing and no long floats**, **fine 7–12-gauge even stitches, lightweight**, in **a bold trend accent — deep eggplant — with a cream contrast-tipped cuff**, a **smooth matte acrylic hand**, **worn by a youthful gender-neutral model, sleeves pushed up, hands relaxed at sides**, **clean light-grey seamless studio backdrop, soft even diffused lighting, 85mm, true colour**, sharp focus, even tight consistent knit-stitch gauge, **smooth crisp surface (not fuzzy, no pilling)**, accurate hands with five separated fingers, plain unbranded garment, no logos, no text, no graphics."*
> — `model=dev; guidance≈3.5; steps≈30; 832×1216; seed=FIX once chosen.`

*Spec note (the §3b fill + the brand-critical trap):* A2 is the **pill kill-zone** — the prompt **forces a smooth crisp tight surface and forbids brushed/floated looks** because "a single pilling video is brand-ending" (`brand_market §3`) and acrylic pills worst (`yarn §4`). If a prettier *fuzzy* render appears, **reject it for A2** no matter how good it looks. Anti-pill / higher-dpf grade + tight gauge are the real defence (`yarn §4b`), not the image. The eggplant accent is the rotating trend colour (`analyst_a §1e`).

### 4d. A3 — thin merino/lambswool Nov–Feb premium capsule (halo / price-anchor — *cable, fashioning marks, premium colour*)
**On-model (the press/halo hero):**
> *"Premium editorial on-model photograph of a **refined relaxed gender-neutral crew-neck pullover sweater**, **fully-fashioned fine-gauge knit with a clean vertical cable pattern down the body and fine links-links purl texture, with visible fashioning marks at the raglan shaping and clean linked seams**, **fine 12-gauge even stitches, lightweight refined weight**, in **deep charcoal with a single tonal anchor**, a **soft lustrous fine-merino hand**, **worn by a poised gender-neutral model, hands relaxed, minimal styling**, **soft warm natural daylight, light wood and off-white tones, gentle shadows, 50mm editorial, true-to-life colour**, sharp focus, even consistent knit-stitch gauge, **realistic soft lustrous merino wool texture, neatly conserved evenly-repeated cable stitches**, accurate relaxed hands with five separated fingers, plain unbranded garment, no logos, no text, no graphics."*
> — `model=pro; guidance≈3.4; steps≈32; 832×1216; seed=FIX once chosen.`

*Spec note (the §3b fill + the §1g trap):* A3 carries **cable + fashioning marks** — the two most likely **"impossible knit"** offenders (cables must conserve stitches; fashioning marks must sit at real decrease lines). The prompt asks for *"neatly conserved evenly-repeated cable stitches,"* but **the spec reviewer + programmer must verify the cable is knittable and price in the extra yarn + linker time** (`SCOPE_GLOSSARY §3`, `analyst_a §1g`). A3 = merino/lambswool 2/48 Nm-class at **12GG**, ~250–400 g, milled/relaxed (`yarn §1b`); even 5–10% cashmere or a few % nylon lifts hand/pill (`yarn §1b/§4`).

### 4e. Capsule-consistency pass (run after the three are chosen)
To make A1/A2/A3 read as one line: **reuse the identical lighting + backdrop + model-description + render-control text** across all three (vary only garment/stitch/gauge/colour, §2c), hold the **same seed family**, and force the **shared cream/oat/charcoal anchors** so the capsule shares its base palette (mirroring real shared dye lots, `yarn §3b`). Then render the **group/flat-lay capsule shot** (landscape AR) for the lookbook. `[ESTIMATE: method, MED-HIGH]`

---

## Bottom line for the owner

1. **Flux is an idea + content engine, not a knit compiler.** It renders a photo of a sweater that was never knit. Use it to go from blank page → approved look → TikTok/IG content (the "AI-designed" story pillar is literally this) — **never mistake the image for a spec.** `[ESTIMATE: HIGH]`
2. **Prompt it like a fashion photo brief in eight ordered slots** — garment/fit → construction/stitch → gauge-look → colour/hand → styling → shot-type → lighting → positive-phrased quality. Every adjective traces to an upstream file; the four design constraints (≤4 colours / fine-gauge / few-seam / anti-pill) are baked into the choices, and the brand rule "clean/light/Californian, never alpine" is stated explicitly or Flux gives you Aspen.
3. **Run it guidance-distilled:** distilled-CFG **~3.0–3.8** photoreal, **~24–32 steps**, dims multiples of 32 (portrait on-model / square flat-lay), **fix the seed to iterate one variable at a time, sweep the seed to explore.** Log prompt+seed+params with every keeper or you've made art, not a process. *(All numbers `[SNAPSHOT]` — re-confirm on the live API.)*
4. **"Negative prompts" don't work on Flux-dev** — steer away from defects with **positive phrasing** ("five separated fingers," "plain unbranded garment, no text," "smooth crisp surface no pilling"), or sidestep (hands in pockets / flat-lay). **Impossible knits can't be prompted away — the spec reviewer catches them.**
5. **Consistency across the capsule is engineered, not given:** shared lighting/backdrop/model/render text, shared palette tokens, a seed family, and (if the endpoint exposes it) image-reference conditioning — but capsule consistency is a *brand-look* goal, **not** a spec-identity guarantee.
6. **The producibility gap is the whole game:** a Flux pixel **cannot** specify the **four things that make a knit buildable** — yarn count, true GG/density, panel-shaping stitch-count grade, finished measurements/shrinkage — and it actively renders **impossible knits, un-payable colourwork, and pill-prone acrylic surfaces.** Translate via: choose on-model → re-render flat-lay+tech-flat → human review vs the 4 constraints + 3 traps → map intent onto `yarn`+`analyst_a` specs → **CAD programmer authors the program** → **PPS swatch is ground truth** → lock tech pack → pop-ups set the size curve.
7. **Three honest limits:** screen colour ≠ dye shade; photogenic ≠ producible ≠ profitable (anti-pill + T9 economics overrule the image); and **no legal shortcut in the pixels** — AI renders/marketing carry the same right-of-publicity/Lanham/TM exposure (`brand_market §5`), so garments stay plain/unbranded and names stay original/coined.
8. **The template + A1/A2/A3 worked prompts (§4) are ready to run today** — copy, fill eight slots, set the params, sweep seeds; A2's prompt is deliberately anti-pill-conservative (reject the fuzzy render), A3's flags the cable/fashioning-mark impossible-knit trap for the reviewer.

---

## Open questions (floor, not ceiling)

1. **Whether image-reference conditioning (img2img / IP-Adapter / Redux / "context") is available on the owner's specific Flux endpoint** — the capsule-consistency lever — `[SNAPSHOT/UNKNOWN]`; verify per endpoint before relying on it.
2. **The exact live FLUX param ranges (guidance/steps/dims) for the model version the owner runs** — `[SNAPSHOT]`; starting values here are dev-consensus, re-confirm on the API.
3. **AI design→knit-program autonomy on our acrylic/cotton/merino fabrics** — does the generative pipeline ever produce a *runnable, correctly-graded* program, or does Flux stay strictly upstream of the programmer? — `[UNKNOWN]` (`machines` Open Q#4, `analyst_a` Open Q#8); **T8 must test, not assume.**
4. **Production stitch density (wales×courses/inch) at our exact 12GG construction** — needed to convert any chosen render's *look* into knittable stitch counts — `[UNKNOWN until PPS sampling]` (`analyst_a` Open Q#4, `yarn` Open Q#11).
5. **Per-article residual shrinkage + real post-wash hand** vs the render's implied look — `[UNKNOWN until PPS swatch]`; the render can't predict it.
6. **Whether a cheap Flux-tier ideation sweep + good-tier finals genuinely lowers cost vs all-pro** for this owner's volume — `[ESTIMATE: MED]`; depends on his actual usage and pricing at run time.
7. **The live Central-Valley-youth hype signal that should bias the *styling/colour* of renders** — `[UNKNOWN]`, owned by the listening loop (`analyst_a §3b`, `brand_market` Open Q#13); pin before a content calendar of renders ships.
8. **Whether tech-flat renders are accurate enough to annotate directly, or must always be redrawn in proper CAD** — `[ESTIMATE: redraw, MED-HIGH]`; Flux tech-flats are illustrations, not dimensioned schematics.

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/design/analyst_b_flux_prompting.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `SCOPE_GLOSSARY.md`; `FACTORY/turlock_business/sectors/food_dining/` (structural template); `brand_market/DEEPDIVE.md`; `yarn/DEEPDIVE.md`; `machines/DEEPDIVE.md`; `design/analyst_a_design_sizing.md` (counterpart — design/sizing/trend/line-plan, not re-derived here).
- **Web-researched (cited):** FLUX.1 prompt structure + parameters (guidance ~3–3.8/steps ~24–32/dims mult-of-32/seed discipline; subject→style→composition→lighting→mood ordering); Flux is **guidance-distilled → no true negative prompt** (positive-phrasing workaround; `true_cfg_scale>1` advanced path); strong-but-verify hands/fabric handling. All numbers carried as `[SNAPSHOT]`.
- **Delivered (owner-facing, the four parts of the brief):** (1) **prompt structure** — the eight-slot knitwear skeleton mapped to upstream vocab, the four constraints encoded, on-model vs flat-lay vs tech-flat by job, brand-locked lighting presets, and **positive-phrased defect avoidance** (distorted hands / fake logos / impossible knits); (2) **parameters + iteration** — dev starting values (all `[SNAPSHOT]`), the one-variable-at-a-time seed-locked loop, logging discipline, and **capsule consistency** (shared scaffold/palette/seed-family + optional reference-conditioning, with the honest "brand-look not spec-identity" limit); (3) **the producibility gap** — what Flux CAN (silhouette/stitch-intent/colour-story/detail/styling) vs CANNOT specify (the **four buildability inputs: yarn count, true GG/density, stitch-count grade, finished measurements/shrinkage**) plus the three things it gets actively wrong (impossible knits, un-payable colourwork, pill-prone acrylic), and the 8-step image→tech-pack translation workflow ending at **PPS swatch = ground truth**; (4) a **reusable fill-in template + ready-to-run A1/A2/A3 worked prompts** (on-model + tech-flat), each with params and a spec-note.
- **Honesty discipline:** Flux's limits stated plainly and bindingly — **designs ideas, does not produce a knit program** (reaffirms `machines §5`); all param numbers `[SNAPSHOT]`; producibility of any render `[UNKNOWN]` until PPS; screen-colour≠dye-shade; photogenic≠producible≠profitable; **no legal shortcut in the pixels** — AI renders/marketing carry the same CA §3344 / §3344.1 (70-yr) / Lanham §43(a) / Class-25 TM exposure (`brand_market §5`), garments kept plain/unbranded and names original/coined; correct US sweater duties carried where relevant (cotton 6110.20.20 ~16.5% / wool 6110.11 ~16% / acrylic-MMF 6110.30.30 ~32%, no cotton-~7% error); 8 open questions as a floor.
- **Bottom line:** prompt Flux as a constrained eight-slot fashion-photo brief, run it guidance-distilled with seed-locked one-variable iteration, avoid defects by positive phrasing, engineer capsule consistency — and **never confuse the cheap pretty image with the spec**: the four buildability inputs and the impossible-knit/pilling/colourwork traps are filled and caught by `yarn`+`analyst_a`+the CAD programmer+the PPS swatch, not by the pixels.

## Sources
- FLUX.1 prompt structure + parameters (subject/style/composition/lighting/mood; guidance ~3.5, steps 24–32, dims multiple-of-32, seed discipline): https://skywork.ai/blog/flux-prompting-ultimate-guide-flux1-dev-schnell/ ; https://www.giz.ai/flux-1-prompt-guide/ ; https://deepinfra.com/blog/flux1-dev-guide ; https://fal.ai/learn/tools/how-to-use-flux
- FLUX.1-dev model card (dev guidance 1.5–5, dims 256–1440 mult-of-32): https://build.nvidia.com/black-forest-labs/flux_1-dev/modelcard ; https://huggingface.co/docs/diffusers/main/en/api/pipelines/flux
- Flux is guidance-distilled → no true negative prompt; positive-phrasing workaround; `true_cfg_scale>1` + negative-prompt advanced path: https://andreaskuhr.com/en/flux-ai-guide.html ; https://huggingface.co/docs/diffusers/api/pipelines/flux ; https://civitai.com/articles/10087/revolutionizing-cfg-unlocking-flux-distilled-models-with-advanced-guidance-algorithms
