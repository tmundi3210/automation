# Part 70 — Realism / Anti-AI QC Checklist

### `ad01_gold_kada` · Men's Yellow-Gold Kada · 22 s / 6-clip 9:16 Reel

**Author:** Realism & Anti-AI QC reviewer of record (the hard reviewer). This gate is **binding**: a re-roll or redesign verdict here is not overruled by a convenience/cost argument from any other department.

**What this document is:** a per-shot QC checklist specific to *this* ad, ordered by human-noticeability, with the PASS thresholds and the re-roll / post-fix / redesign decision per defect, plus the escalation rule that stops the ad from shipping.

**Evidence-tier discipline (carried into every verdict below):** claims are tagged **FACT** (fetched abstract / named review taxonomy, some via search excerpt), **ESTIMATE** (editorial synthesis or reasoned instantiation — includes the noticeability order, the 0/1/2 scale, and the re-roll thresholds), or **UNKNOWN** (held open with a named resolution path). None is silently upgraded.

**Scope boundaries (this gate does NOT decide):** which Higgsfield model/mode/parameters to call (→ model-ops specialist); whether the AI-disclosure label / caption obligation is met (→ media/platform specialist — noted here only as a ship-blocker to route); whether the persona reference sheet is correctly authored (→ character specialist — this gate only *diffs against* it). The likeness lane is closed: the presenter is a **fictional AI brand model**, and all "identity" checks below concern that fictional persona's self-consistency, never concealment of a real person.

---

## 0. The evidence-based priority flip (why this checklist is ordered the way it is)

Human one-shot detection of AI **faces/skin** hovers near chance (Diel et al. 2024: 55.54% overall, 57.31% video; Nightingale & Farid: 59.0% even trained, and synthetic faces rated *more* trustworthy than real, 4.82 vs 4.48) [FACT via search excerpt]. Where models **objectively fail** is physics/motion/text (VideoPhy best 39.6%; VideoPhy-2 best 22% on its hard subset, worst on conservation of mass/momentum; PhyGenBench: neither scaling nor prompting alone fixes dynamic physics) [FACT — abstracts].

**Consequence for this ad:** QC budget goes to the gold's **specular/physics behavior, any legible text, hands, and identity lock** first — and to skin micro-texture *last*. This ad is almost entirely **product macro + a calm presenter**; it deliberately has no walking, no eating, no fluid pours, no crowds. That is a low-risk shot design. The residual risk is concentrated in four places, in noticeability order: **legible text (CTA/hallmark)**, **the gold's color + micro-reflection + diamond-cut sparkle reading as real specular**, **hands whenever the kada is held**, and **presenter identity lock across the 6 clips**.

An automated MLLM/VLM critic may pre-filter but is **never** the sole gate (Artifact-Bench: many of 19 leading MLLMs near/below random, misaligned with human perception) [FACT]. A human applies rubric items 1–4 on every shot before ship.

---

## 1. Scoring gate (applies to every shot below)

Score each rubric item **0 = clean · 1 = visible on inspection · 2 = visible at a glance** [ESTIMATE — editorial operationalization; no fetched study specifies this exact scale].

Apply the gate in this order — **the override is checked BEFORE any total is summed:**

1. **Any single item = 2 → RE-ROLL.** Non-negotiable, regardless of how clean everything else is. One glance-visible tell can permanently flip an account's perceived authenticity. This also triggers **human review** (escalation rule below).
2. **Running total ≥ 4 → RE-ROLL**, even with no single 2.
3. **Total 2–3 with no single 2 → fixable in POST *only if*** the offending region is genuinely croppable, gradeable, or speed-rampable, AND the tell is a **texture-class** item (item 8 texture stability, or item 10 skin micro-texture). Post is **never** allowed to mask a structural/geometric tell (text, hands, contact physics, identity, shadow/reflection geometry).
4. **Total 0–1 → PASS.**

**Three checkpoints per shot** (a shot must clear all three):
- **Start-frame image** — score items **1, 4, 5, 6, 9, 10**. A start-frame failure **blocks** video generation (image-to-video inherits and compounds the flaw).
- **Raw generated video** — score **all ten** items + the luxury-physics sub-checks.
- **Post-edited final, re-checked on an actual phone at feed resolution** — items **1–4 minimum**. Compression hides frequency fingerprints, skin waxiness, and faint boiling/shimmer; it does **NOT** hide garbled text, wrong finger counts, foot-slide, shadow-direction errors, identity drift, or object-count changes [FACT via search excerpts]. Do not approve on a desktop raw-export view alone.

**The ten rubric items, in this ad's noticeability order:** (1) text/logos/badges · (2) hands in motion · (3) contact physics · (4) identity lock · (5) object permanence/count · (6) shadow/reflection geometry · (7) motion quality · (8) texture stability · (9) background coherence · (10) skin micro-texture. Order is an **ESTIMATE** (editorial synthesis, not a fetched ranking).

---

## 2. The ad's four high-noticeability risk areas (read these first)

### A. LEGIBLE TEXT — highest-priority tell in this ad
Garbled/near-English lettering is historically the single most reliable AI tell, checkable by any viewer in under a second [FACT via search excerpt]. This ad has three text exposures, and the rule for all three is the same: **text is generated in POST, never in the model.**

- **CTA card (Shot 6)** — the whole shot is text. **Never generate the CTA copy, handle, or link inside the clip.** The model generates only the kada-at-rest plate; the CTA card, `[OWNER-SLOT: @handle]`, and `[OWNER-SLOT: purchase link URL]` are **composited graphics in the editor.** Any legible text baked into the generated frame = automatic item-1 = 2 → re-roll the plate clean.
- **Authenticity lower-third (Shot 5)** — the `[OWNER-SLOT]` purity/hallmark line is composited overlay text, **only populated with TRUE owner values.** If the owner supplies no hallmark, the line is cut — the ad must **not imply a hallmark that does not exist** (this is a compliance ship-blocker, not just a realism one).
- **Hallmark / karat stamp on the metal (Shots 3–4 macros)** — **do NOT generate any hallmark, BIS mark, karat stamp, or maker's mark on the band.** These are luxury micro-typography — the exact class the model garbles, and a fabricated authenticity mark is *also* a compliance violation. Keep the macro on the plain textured band and terminal motif only. A generated stamp = item 1 = 2 → **re-roll**, and if it recurs, **redesign** the framing to exclude the stamp zone. A real stamp appears only if the owner supplies a reference and only as owner-verified fact.

> **PASS threshold (text):** zero legible generated glyphs anywhere in any clip. All intended text is a post overlay. **Fix routing:** generated text → re-roll to a clean plate (post cannot "fix" garbled generated text, only re-shoot around it); missing/false owner claim → cut the line (compliance).

### B. GOLD COLOR + METAL MICRO-REFLECTION + DIAMOND-CUT SPARKLE — the product *is* the sell
This is the hero of clips 1, 3, 4, and it draws **genre-expert scrutiny**: jewelry buyers know exactly how light plays on polished gold and on a diamond-cut face, so their effective detection rate sits above the general ~55% baseline [ESTIMATE — audience reasoning, moderate-high confidence]. Three sub-checks, in order:

1. **Gold COLOR accuracy.** Warm 22k-look yellow gold must read as *metal*, not as brass, mustard paint, or an orange-plastic HDR glow. Check: the band shows a **warm-to-cool tonal gradient across its curvature** (the polished rails catch a near-white hot specular; the recessed cross-hatch stays deeper amber). A flat, single-hue "gold sticker" look = item 8 (texture stability) or item 6 (reflection geometry) fail. The owner's reference photo is the ground-truth color target — grade the final toward it, not toward a generic "luxury gold."
2. **Metal micro-reflection believability.** Polished gold is a mirror at grazing angles: it must **reflect the environment** (the warm key, the cool fill, dark surroundings) and those reflections must **track continuously** as the wrist rotates (Shot 1) or the camera travels (Shot 3). Failure modes to flag: reflections that sit *painted-on* and don't move with the surface; a specular hotspot inconsistent with the scene's **single key light** (see §C golden-hour/single-light rule); "boiling" shimmer on the metal between frames (item 8). Optics is a named PhyGenBench failure domain [FACT]; the metal-reflection instantiation applies it with the single-light forensic check [ESTIMATE — high confidence].
3. **Diamond-cut SPARKLE as real specular, not painted-on.** The cross-hatch wheat-grain must throw **specular glints that MOVE as light/camera/wrist move** — sparkle is a view-dependent event, not a texture. **Common AI failure: static, painted-on sparkle that stays put frame-to-frame, or glints that fire inconsistently with the key light** [FACT — jewelry sparkle physics node]. Check on the *raw video*, not the still: scrub frame-by-frame and confirm individual glints **appear, travel across the facets, and extinguish** as the specular roll passes (Shot 3's "one specular roll to animate the sparkle" is exactly this test). If the sparkle is frozen or twinkles randomly without tracking the light, that is item 6 = 2 → re-roll.

> **PASS threshold (gold):** color matches the owner-reference warm gold within a believable metal gamut; reflections track the surface continuously with no boiling; at least one clean specular roll shows sparkle *moving as specular*. **Fix routing:** color-only miss with no motion/geometry fault → **POST grade** (allowed, texture-class). Static/painted-on sparkle or reflections that don't track → **re-roll** (structural, post cannot add real specular motion). Persistent failure across re-rolls → **redesign** the move (a slower, single controlled specular roll on a locked axis is easier for the model than a complex multi-light rotation; route the *reason* to model-ops, not the model choice itself).

### C. HANDS / FINGERS — known-hard; mitigated by design, not by iteration
Hands are a **weakening still tell but a strong video tell** — motion re-exposes finger-count and morphing errors frame to frame [FACT via search excerpt]. Dexterous hand-object manipulation (clasping/turning jewelry in-fingers) is a **documented open research problem** on the known-hard avoid list — it is **not** something to iterate on after the fact [FACT]. This ad holds the kada in Shots 1 and 2. Mitigation is structural and set at scripting time:

- **Controlled, non-dexterous holds only.** Shot 1 is a **slow forearm rotation** toward the lens — no fingers manipulating the piece, fingers loosely out of frame. Shot 2 is a **static resting pose** — wrist held, not fidgeted; no clasping, no turning the kada in-hand.
- **Wrist-wear framing.** The kada is worn on the wrist / forearm, so the hand need not interact with it dexterously at all — this is the single biggest risk reducer. Frame the cuff on the wrist; keep the hand relaxed and preferably partially out of frame.
- **Macro cutaways instead of manipulation.** Any "showing the detail" is done via the **product macros (Shots 3–4)** — camera moving over a resting/worn piece — **never** by the presenter picking it up and rotating it in his fingers. If a beat ever seems to need in-hand manipulation, that is a **redesign** trigger, not a re-roll trigger.

> **PASS threshold (hands):** correct finger count and no finger/knuckle morphing across every frame the hand is visible; no held-object blending into the hand. **Fix routing:** a single-frame finger glitch in an otherwise clean clip → **POST** (crop/reframe/speed-ramp past it, *if* the region is genuinely excisable) — but a *morphing* hand or wrong finger count that persists across frames is **structural → re-roll**; any need for in-fingers manipulation → **redesign** to a wrist-wear or macro-cutaway. Repeated hand failure on a non-manipulation hold → escalate as a candidate to formally shoot the hand out of frame entirely.

### D. PRESENTER IDENTITY LOCK across the 6 clips
"Identity drift" (face geometry, eye color, hair, wardrobe changing across shots without cause) is an industry-named phenomenon [FACT via search excerpts]. The presenter appears in Shots 2, 5, 6 (and his cuffed forearm in 1). He must read as **one man in one world** across all of them.

- **Diff every appearance against the master persona reference sheet** — not merely against the adjacent shot (drift accumulates gradually). Check: face geometry, skin tone (warm mid-brown-to-tan), salt-and-pepper hair + groomed stubble, the quiet-luxury navy/charcoal wardrobe, and the single metallic note being *this* kada.
- **Strongest structural mitigation: lock identity at a QC-approved start frame** and generate image-to-video from it, so anatomy/wardrobe/identity are fixed at t=0 before drift-prone motion begins [ESTIMATE — high confidence]. (*Which Higgsfield mode supports start-frame conditioning is out of this gate's scope → model-ops; UNKNOWN here.*)

> **PASS threshold (identity):** every presenter shot matches the reference sheet on face, hair, skin tone, and wardrobe; the kada is visually the same piece in every appearance. **Fix routing:** drift → **re-roll from the locked start frame** (structural; post cannot fix a changed face). If the reference sheet itself is ambiguous/underspecified → **escalate to the character specialist** (this gate diffs against the sheet, it does not author it).

---

## 3. Per-shot QC checklist

Each shot lists its **priority items** (the ones that actually carry risk for that shot — score these hardest), the shot-specific pass note, and the fix routing. Score the full ten-item rubric on raw video regardless; the priority list is where the noticeability budget goes.

### SHOT 1 — HOOK (`ad01_s1_hook`) · forearm rotation + diamond-cut flare · 4.0 s
**Priority items:** 1 (composited text) · 2 (hand) · 6 (sparkle/reflection geometry) · 8 (metal texture stability).
- **Gold/sparkle:** this is the flare frame — the whole hook rests on the diamond-cut sparkle reading as **moving specular** during the forearm rotation (§B.3). Scrub for a glint that travels and extinguishes; a frozen/painted flare kills the hook.
- **Hand:** slow forearm turn only, fingers loose/out of frame, **no manipulation** (§C). Watch the wrist-edge for finger morphing across the rotation.
- **Text:** "Real gold. Real craft." is a **post overlay** in the upper-safe zone — never generated.
- **Reflection geometry:** the moving gold must reflect the warm low-key environment consistently with one key light.
- **PASS:** clean specular roll + no finger morph + no generated glyphs. **Fix:** static sparkle or reflection-not-tracking → **re-roll**; single-frame edge-finger glitch → **post** speed-ramp/crop if excisable; any in-hand manipulation creeping in → **redesign** to pure forearm turn.

### SHOT 2 — PRODUCT REVEAL / PERSONA (`ad01_s2_reveal`) · presenter chest-up, push-in · 4.0 s
**Priority items:** 4 (identity) · 2 (resting hand/wrist) · 6 (gold on wrist) · 10 (skin, lightly).
- **Identity:** first full look at the presenter — set the identity anchor and diff against the reference sheet (§D). This is the frame to lock and reuse for 5 and 6.
- **Hand/wrist:** static resting pose, wrist held not fidgeted, **no dexterous manipulation** (§C). Mouth relaxed/near-neutral — **VO-driven, no lip-sync lock** (see §4).
- **Gold:** the kada on the wrist still needs believable metal reflection as the push-in changes angle.
- **Skin:** item 10 is lowest priority and compression-masked — a glance only; do not spend budget polishing it.
- **PASS:** identity matches sheet + resting hand clean + kada reads as metal. **Fix:** identity drift → **re-roll from locked start frame**; hand fidget/morph → **re-roll**; skin waxiness alone → **pass/light post grade** (texture-class, compression will mask it).

### SHOT 3 — QUALITY MACRO, BAND (`ad01_s3_band_macro`) · extreme macro, lateral travel · 4.0 s
**Priority items:** 1 (NO generated hallmark/stamp) · 6 (sparkle as specular) · 8 (texture stability/boiling) · 5 (motif/rail continuity).
- **Text:** **do NOT generate any hallmark, karat stamp, or maker's mark** (§A). Keep the macro on the plain cross-hatch band and the two polished rails only. A generated stamp = item 1 = 2 → re-roll/redesign.
- **Gold/sparkle:** the hero craftsmanship beat. The lateral travel + one specular roll must make the wheat-grain **sparkle move as specular** (§B.3) and the two rails hold a clean continuous hot highlight. Watch for metal **boiling** (item 8) between frames.
- **Object permanence:** the cross-hatch pattern and rail spacing must stay **consistent** along the travel — no pattern that mutates, tiles, or changes pitch mid-move (item 5 / repetition-tiling).
- **No hands in frame.**
- **PASS:** no generated stamp + sparkle moves as specular + rails/pattern continuous + no boiling. **Fix:** color-only miss → **post grade**; static sparkle / boiling / mutating pattern → **re-roll**; recurring garble of the fine texture → **redesign** (slower move, shallower travel, or carry the exact texture harder via the owner reference image — route the model/prompt reason to model-ops).

### SHOT 4 — QUALITY MACRO, TERMINALS (`ad01_s4_terminal_macro`) · extreme macro, rack focus on motif · 3.0 s
**Priority items:** 5 (motif fidelity/symmetry — the highest risk in this shot) · 1 (no generated stamp) · 6 (gold) · 8 (texture).
- **Motif fidelity:** the terminal openwork temple/Greek-key panel is **fine geometric detail — the class most prone to garble.** Check the still hard: the geometric repeat must be **regular and symmetric**, the openwork negative space clean, the open cuff end reading correctly. **Any garbled, asymmetric, or melted motif = item 5 = 2 → re-roll, not ship.** This is the primary reason the exact piece must be carried by the **owner's reference photo** (uploaded via the Higgsfield media-upload widget at run time — the connector cannot read a chat attachment).
- **Text:** still no generated stamps in the terminal zone.
- **PASS:** motif geometry regular, symmetric, and matching the owner reference; open end reads clearly. **Fix:** garbled/asymmetric motif → **re-roll**; if it will not resolve across re-rolls → **redesign** (shorter rack, softer focus settle, or hold the motif slightly defocused so the eye reads "detail" without the model having to render every element — last resort, since the motif is a selling point).

### SHOT 5 — AUTHENTICITY (`ad01_s5_authenticity`) · presenter medium + resting kada + lower-third · 4.0 s
**Priority items:** 4 (identity continuity from Shot 2) · 1 (composited lower-third, TRUE values only) · 6 (gold on stand) · 3 (kada resting physics).
- **Identity:** diff against the reference sheet AND against Shot 2 — same man, same wardrobe, same kada (§D).
- **Text:** the `[OWNER-SLOT]` authenticity lower-third is a **composited overlay populated only with TRUE owner values.** No hallmark implied if none exists (compliance ship-blocker).
- **Resting physics (item 3):** the kada resting on the soft-lit stand/surface must sit with **believable contact and weight** — a solid cuff casts a **contact shadow** and does not float or hover. Check the shadow anchors the piece to the surface.
- **PASS:** identity continuous + only-true composited text + kada rests with a real contact shadow. **Fix:** identity drift → **re-roll from locked frame**; floating/no-contact-shadow → **re-roll** (contact physics is structural, never post); false/absent owner claim → **cut the line** (compliance).

### SHOT 6 — CTA (`ad01_s6_cta`) · kada at rest + CTA card · static hold · 3.0 s
**Priority items:** 1 (the entire shot is text — all composited) · 6 (gold at rest) · 3 (resting physics).
- **Text:** the CTA card, `[OWNER-SLOT: @handle]`, and any link are **100% composited graphics in the editor.** The model generates only the clean kada-at-rest plate. **Zero legible generated glyphs** in the plate. Keep composited text out of the bottom ~250 px Reels UI band and the top-right menu safe zone.
- **Gold/physics:** the resting kada still needs believable metal + a contact shadow (no float).
- **PASS:** clean generated plate with no glyphs + CTA legible and in-safe-zone + kada rests believably. **Fix:** any generated text in the plate → **re-roll the plate clean** (post overlays the real CTA); float → **re-roll**.

---

## 4. Talking-model / lip-sync (conditional — only if the owner requests the talking-presenter cut)
The default is **VO-driven with no sustained talking-head lip-sync** — the plan decouples voice from a locked mouth precisely because tight lip-sync (mouth/teeth/phoneme alignment) is a frequent AI tell. **In the default cut, there is no lip-sync item to gate** — keep the presenter's mouth relaxed/near-neutral in Shots 2, 5, 6 and cut to product macros as claims land.

**If the owner requests the talking cut,** add a lip-sync sub-check to the presenter shots:
- **Phoneme alignment:** mouth shapes must match the audible consonants/vowels; watch for mouth motion continuing after audio stops, or a "mushy" mid-viseme that never closes on b/p/m.
- **Teeth stability:** teeth must not multiply, merge, or shimmer between frames (item 8 / exaggerated-feature + asymmetry classes).
- **Jaw/lip coherence:** no rubber-banding of the lip line or jaw geometry.
- **Scoring:** treat a glance-visible lip-sync break as **item 2 or 7 = 2 → re-roll.** Lip-sync is **not reliably post-fixable** and repeated failure is a **redesign** trigger (return to the VO-driven default — the plan's stated fallback). Route any "which model/mode does better lip-sync" question to **model-ops**, not this gate.

---

## 5. Chain shot — blocked, do not generate
The brief references a matching gold chain but supplied **no chain photo.** Any chain shot is an **OWNER-SLOT**: it cannot be generated faithfully without a reference image, and inventing a specific chain design is out of bounds. **This gate will re-roll/redesign to zero any chain that appears without an owner reference.** No chain is in the 6-shot plan; keep it out until a reference exists.

---

## 6. Decision summary — defect → disposition

| Defect class | Where it bites in this ad | Default disposition |
|---|---|---|
| Generated legible text / garbled glyph | Shots 1, 5, 6 overlays; any stamp in 3, 4 | **Re-roll to clean plate** (text is post-only); false owner claim → **cut** |
| Static / painted-on diamond-cut sparkle | Shots 1, 3 | **Re-roll** (structural; post cannot add real specular) |
| Reflections not tracking the metal surface | Shots 1, 2, 3, 6 | **Re-roll** |
| Gold **color** off (no motion/geometry fault) | any gold shot | **Post grade** toward owner-reference gold (texture-class, allowed) |
| Garbled / asymmetric terminal motif | Shot 4 | **Re-roll**; if unresolved → **redesign** (defocus/shorter rack) |
| Finger morph / wrong finger count in motion | Shots 1, 2 | **Re-roll**; single excisable frame → **post**; needs manipulation → **redesign** to wrist-wear/macro |
| Any in-fingers dexterous manipulation of the kada | any presenter shot | **Redesign** (never iterate — known-hard) |
| Presenter identity drift | Shots 2, 5, 6 | **Re-roll from locked start frame**; ambiguous sheet → **escalate to character specialist** |
| Kada floating / no contact shadow | Shots 5, 6 | **Re-roll** (contact physics, never post) |
| Metal boiling / texture shimmer | Shots 1, 3 | **Post** only if croppable/gradeable AND texture-class; else **re-roll** |
| Skin waxiness / micro-texture | Shot 2 | **Pass / light post** — compression masks it; do not overspend |
| Lip-sync break (talking cut only) | Shots 2, 5, 6 | **Re-roll**; repeated → **redesign** back to VO default |
| Chain with no owner reference | anywhere | **Redesign to remove** |

---

## 7. Escalation rule — what stops the ad from shipping

The reel does **not** ship until every one of the following holds. Any unmet item is a hard stop (re-roll / redesign / escalate to the owning specialist), not a judgment call:

1. **No single rubric item scores 2 on any of the 6 shots**, verified by a human on the **post-compression, phone-screen, feed-resolution** asset — not a desktop raw export. Any single 2 = mandatory human-reviewed re-roll (the any-single-2 override).
2. **Zero legible generated glyphs** anywhere; all text (hook line, authenticity lower-third, CTA, handle, link) is a **post overlay populated only with TRUE owner values.** No generated hallmark/karat/maker's stamp on the metal.
3. **The terminal motif (Shot 4) is clean, symmetric, and matches the owner's reference photo** — the reference photo has actually been uploaded via the Higgsfield media-upload widget at run time.
4. **Presenter identity is continuous** across Shots 2/5/6 against the master reference sheet.
5. **The diamond-cut sparkle reads as moving specular** (verified on raw video, frame-scrubbed) in at least the hero macro (Shot 3) and the hook (Shot 1).
6. **No fabricated authenticity claim** — spoken or on-screen. If an owner-slot (karat, hallmark, weight, price, brand, city, guarantee) is empty, the corresponding line/overlay is cut. *(This is a compliance obligation owned by the media/platform specialist; this gate flags it as a ship-blocker and routes it — it does not adjudicate policy.)*
7. **AI-content disclosure is handled** — Meta "AI info" label ON + honest, non-embellishing caption. *(Owned by the media/platform specialist; noted here only as a ship-blocker to route — not decided by this gate.)*

**Binding-verdict note:** items 1–5 are this realism gate's own authority and cannot be overruled by a cost/convenience argument. Items 6–7 are outside this gate's scope but are listed because they *also* stop the ship — they are routed to their owning specialists, not resolved here. If any shot has cycled through re-roll → model-swap → prompt-fix without clearing, it escalates to **shot redesign** (imply rather than show); if a redesign still cannot carry the beat, the beat is cut before a flawed shot ships.
