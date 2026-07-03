# Part 50 — Edit, Grade & Cover — `ad01_gold_kada`
### Post-production spec for the 22 s / 6-clip men's gold kada Reel

**Scope contract.** This part takes the six QC'd clips the shot list (`00_master_plan.md §4`) hands off and turns them into one published, graded, cover-ready Reel. It owns the timeline (assembly, pacing, transitions), the grade (one house look, skin + gold protected, cross-model drift matched), the on-screen text / CTA treatment, and cover/poster selection. It does **not** re-author the story, the shot list, the prompts, the model routing, or the caption/posting decision — those live in their own parts. Where a claim rests on an unfetchable platform number it is tagged `[FACT/excerpt]`, colorist/editorial craft is `[ESTIMATE]`, and named blockers are `[UNKNOWN]`.

**Alignment lock to the master plan.** 22.0 s, six clips, durations `4 / 4 / 4 / 3 / 4 / 3`. Ending = **soft open loop** (ends on the offer, not a return to frame one). Seams already specified: whip-out→whip-in (1→2), match-cut-on-the-gold (2→3), invisible-cut/rack-focus (3→4), whip (4→5), clean cut (5→6). All on-screen text is **composited in the editor, never generated in-clip**. This part conforms to those decisions; it does not relitigate them.

> **Binding compliance carried into post (from the brief):** publish with Meta's **AI-info label ON**; caption honest and non-embellishing; **no fabricated hallmark / karat stamp / maker's mark anywhere in the macros or the grade**; every authenticity fact is an `[OWNER-SLOT]` populated only with a TRUE value; presenter is a **fictional AI brand model**, never "a verified happy customer"; the exact bracelet is carried by the **owner's uploaded reference photo** (Higgsfield media-upload widget at run time — the connector cannot read a chat attachment); **no invented chain** appears. The cross-model gold-matching problem below is the technical heart of this part.

---

## 1. Assembly Order & Pacing

### 1.1 Timeline (locked to the beat sheet)

| Clip | Beat | On-screen dur | Source model band | Role in energy curve |
|---|---|---|---|---|
| 1 | Hook — forearm rotate, diamond-cut flare | 4.0 s | 4 s both-frames (`seedance1_5`-class) | **Open on strongest image, no intro frame** |
| 2 | Reveal — presenter + kada | 4.0 s | 4 s both-frames | establish persona/world |
| 3 | Band macro — cross-hatch, rails, sparkle | 4.0 s | 4 s both-frames | **hero hold (longest dwell)** |
| 4 | Terminal macro — temple/Greek-key motif | 3.0 s | `wan2_7` (native 2–3 s) | connective, accelerate |
| 5 | Authenticity — presenter + lower-third | 4.0 s | 4 s both-frames | settle / claim lands |
| 6 | CTA card | 3.0 s | `wan2_7` | arrival, hold the offer |

Model bands are the master plan's provisional routing; **which model actually fills each cell is the prompting/model-ops specialist's call and is `get_cost:true`-preflighted before spend** — it matters here only because clips **1/2/3/5 come off one model and 4/6 off another**, which *is* the cross-model color-drift problem graded in §2.4.

### 1.2 Pacing rules applied

- **Hook front-loading `[FACT/excerpt]`.** Up to ~50% of viewers drop in the first 3 s; watch-time is the dominant ranking signal. Clip 1 opens **cold on the flare frame with motion in frame one — no logo, no title card, no slow build**. The single strongest image is spent as the opener; it reads with sound off (silent-legibility floor). The `5–10×` reach multiplier sometimes attached to this is a third-party `[ESTIMATE]`, not a Meta number — do not cite it as fact.
- **Shot-length modulation, not a metronome `[ESTIMATE]`.** Even though every source clip is a similar 3–6 s, the *edit* decides dwell. The **band macro (clip 3) is the hero hold at the full 4 s**; the connective terminal macro and the CTA run 3 s. Energy **accelerates into the macro pair (3→4)** — the craftsmanship reveal is the structural peak — then **decelerates into the authenticity settle and CTA arrival**. This is a single-product ad, so the "peak" is the product hero, not a music drop.
- **Clip count is a ceiling, honored.** Six clips is the plan's deliberate budget (five ad beats + a split quality beat). Do **not** pad. If a clip fails QC and can't be re-rolled in budget, the reel ships at fewer beats rather than with a weak filler shot.
- **Cut-on-beat discipline `[FACT/excerpt]`.** Because the clips carry **no production audio**, the track and VO are laid first; **mark beats first** (waveform spikes / tap-markers), then conform the six cut points to **downbeats/accents only — never every beat**. With only five seams this means: land the **match-cut into the band macro** and the **cut to the CTA card** on the two strongest accents. The circulated "Facebook 40% higher completion for synced audio" figure is `[UNKNOWN provenance]` — never stated as fact.
- **Emotion over rhythm when they conflict (Murch, Rule of Six) `[FACT/excerpt]`.** Rhythm is 10% against emotion's 51%. If a beat-locked cut would clip the gold flare or the presenter's approving beat mid-gesture, **slip the cut a frame or two toward the nearest strong beat** rather than sacrifice the moment.

### 1.3 Sound bridging (built entirely in post)

AI clips arrive silent, so **all** bridging is a timeline construction over the VO + music + SFX layers `[FACT/excerpt]`:
- **L-cut** the VO across the macro seams — let clip 3's "…diamond-cut to catch the light" **trail over** the first frames of clip 4 so the two macros read as one continuous craft observation, not two cuts.
- **J-cut** the CTA: let a soft music resolve / the "in bio" text-echo audio **lead** clip 6's picture by ~8–15 frames so the offer feels arrived-at, not abrupt.
- VO is decoupled from any locked mouth (the plan's voiceover-driven decision) — this is what lets the words land exactly on the macro cuts.

### 1.4 Ending mode — soft open loop (NOT a hard loop)

The plan optimizes for the **CTA click**, not invisible rewatch. So this reel **ends on the offer** — it does **not** frame-match its last frame back to its first (no 200–400% zoom-and-trim loop, no same-image-both-ends generation trick). A decelerating loop-return and a rising CTA cannot both be maximized; the CTA wins. Practically: clip 6 holds the product-at-rest + CTA card to the full 3 s and ends clean. Any "loop-y" softness is limited to the product being visually at rest so a replay isn't jarring — but the **energy and the CTA read through the last frame**.

### 1.5 Tooling

Mobile editor for beat-marking and the rough assembly only; **hand off to a desktop NLE for every frame-accurate step** — the match-cut frame trims, the composited text keyframes, and (critically) the scopes-based grade in §2, which mobile tools can't do reliably `[ESTIMATE]`.

---

## 2. Color Grade

### 2.1 The two anchors for this product: skin AND gold

Standard house discipline protects **skin** as the non-negotiable realism anchor. This ad adds a **second protected element — the product gold** — because the whole sell is the gold looking *real warm gold (24k-plated)*, not *orange plastic*. Both are qualified out of the creative-look push and held to saved targets. Everything else (wardrobe, backgrounds, surfaces) takes the look.

### 2.2 Chain order (fixed, never reordered) `[FACT/excerpt]`

**(1) Normalize → (2) Primaries → (3) Secondaries → (4) Creative look last.** Each stage assumes an accurate image from the one before; a look applied before balancing bakes per-clip mismatch into the style, and a secondary keyed on an unbalanced image grabs the wrong pixels.

- **Normalize — reframed for AI delivery `[ESTIMATE]`.** These clips are assumed **display-referred Rec.709-ish, not log** (exact Higgsfield delivery spec — color space / bit depth / fps — is `[UNKNOWN here]`, owned by the MCP-surface file; do not apply a log-conversion LUT to an already-display-referred clip). So "normalization" here means **per-clip balancing to a declared hero reference**, not a log transform.
- **Primaries** — exposure, white balance, contrast, per clip, read on scopes (§2.3).
- **Secondaries** — isolate skin; **isolate the gold** (the product-specific move, §2.5); any window work.
- **Creative look last** — the warm-neutral house LUT (§2.6), baked as ONE creative LUT applied *after* balancing.

### 2.3 Scopes discipline `[FACT/excerpt]`

Never match by eye across subtly-drifting AI clips (adapted eyes normalize drift). Read three instruments in a fixed question order: **luminance on the waveform → white balance on the RGB parade → skin hue on the vectorscope skin-tone line.** For this product add a fourth read: **the gold's hue angle on the vectorscope** — it should sit on a consistent warm-yellow vector across every clip, never swinging toward the orange/red side (the "cheap plastic gold" tell) or toward green.

### 2.4 The real problem — cross-model gold drift, and how it's matched

The seams put **two different generation models in one reel** (clips 1/2/3/5 vs 4/6). Different models carry **systematic but a-priori `[UNKNOWN]` color signatures** — the same physical gold will render at a slightly different temperature, saturation and specular character on each. Uncorrected, the terminal macro (clip 4, model B) will not match the band macro (clip 3, model A) that it invisibly cuts from — and a mismatched gold across a match cut is exactly what makes an ad read as stitched-together fakery.

Procedure:
1. **Declare one hero clip.** Choose it on **best skin + best-and-truest gold + best exposure together** — the presenter reveal (clip 2) or authenticity (clip 5) is the natural hero because it carries skin *and* gold in the same frame. Sample the **gold target** from the cleanest product macro and the **skin target** from the hero.
2. **Per-model trim preset, applied BEFORE hero matching `[ESTIMATE]`.** Learn a small temperature/saturation/offset nudge for **each model** from its first graded batch, so model-B clips (4, 6) are pulled toward the house baseline before any per-clip match. These presets are *learned from real batches, never asserted from memory* — the signatures are `[UNKNOWN]` until generation runs.
3. **Match every clip to the hero** in waveform→parade→vectorscope order.
4. **Hold the gold hue with a dedicated secondary qualifier** keyed on the product gold, pinned to the sampled true-gold vector, on **every** clip regardless of source model. This is what makes the match-cut gold (clip 2→3) and the invisible-cut gold (clip 3→4) read as one continuous bracelet.

### 2.5 Keeping the gold warm without going orange or plastic

- **Warm ≠ orange.** The house look is warm (~2700–3000 K practical) `[ESTIMATE]`, but push warmth on the *scene*, not into the gold's own hue. The gold's yellow vector is **held by qualifier**, so global warming doesn't slide it into orange.
- **Protect the specular, don't clip it.** The product hero *is* the diamond-cut sparkle — the specular roll across the cross-hatch. The house rule is **highlights rolled off, never clipped** (muted, non-clipping highlights are the single strongest "expensive film" cue) `[ESTIMATE]`. But a hard-clipped gold specular reads as blown white plastic. So: **roll the specular highlights to a bright warm-gold shoulder that retains hue and micro-detail** — the sparkle stays a *gold* sparkle, not a white hole. This is the most important single grade move for this product.
- **"Plastic" is a texture failure, not just a color one.** If the band macro reads plastic after grading, that is a **starred physics/realism-axis problem** and belongs to stills-stage regeneration + the realism reviewer, **not** a grade patch. Do not try to grade plastic out.
- **Keep the teal-shadow separation a whisper.** The warm-skin-vs-cooler-field mechanism is used at a whisper, never a wall `[FACT/excerpt mechanism; ESTIMATE dose]`. Cool shadows must **never touch the gold or the skin** — a cool cast on gold instantly kills the gold's warmth.
- **Global chroma ~10–20% below default, gentle S-curve, lifted-but-not-milky blacks** `[ESTIMATE]` — quiet-luxury restraint, matching the plan's register. Gold and skin are re-saturated back to target *after* the global desat so they stay rich while the world stays muted.

### 2.6 One house LUT, baked last

The finished warm-neutral look is baked as **one creative LUT applied after per-clip balancing** — never before (a LUT is a dumb transform that amplifies unbalanced mismatch). Every clip, the CTA card background, and the cover all share this one look so the grid stays coherent (§4).

### 2.7 Delivery — compression-safe for Instagram `[FACT/excerpt + ESTIMATE]`

- **Banding is structural** on 8-bit 4:2:0 social delivery, and this reel is full of **exactly the surfaces that band** — smooth warm gold gradients, out-of-focus bokeh, soft-lit stand/marble backgrounds. Counter with **higher-bit-depth grading, ~1–2% fine dither/grain, and a deband filter**. The grain doubles as a **shot-on-film-not-AI cue**; keep it subtle (heavy grain wastes bitrate and smears under re-encode).
- **Crushed blacks:** the plan's warm low-key hook and deep-anchor wardrobe live near black; heavy encoders crush near-black macroblocks first. Keep intentional shadow detail **a few code values above 0**.
- **Verify on an actual phone test upload**, not the NLE preview — the gold specular, the banding on the gold gradients, and the black detail all behave differently after Instagram's re-encode.
- Deliver **1080-wide, Rec.709/SDR, "Upload at highest quality" ON, watermark-free** (leftover tool watermarks suppress recommendation). The `~3,500–5,000 kbps` window is a third-party `[ESTIMATE]`, not a Meta spec.

---

## 3. On-Screen Text & CTA Treatment

All text is **composited in the editor over the graded footage — never model-generated** (generated legible text is a known AI tell and, for hallmarks/claims, a compliance hazard).

### 3.1 Reels safe zones (compose to the conservative union) `[ESTIMATE; official Meta spec UNKNOWN]`

9:16, 1080×1920. Keep text clear of the UI-covered bands: **~108–220 px top** (camera/audio), **~320 px bottom** (caption, audio attribution, profile/follow), **~60 px left**, **~110–120 px right** (like/comment/share/save stack). Usable center ≈ **900 × 1400–1500 px**. The bottom caption band and the right button stack are the two most-covered regions — **the link/handle must never sit there or the UI hides it.** Where guides disagree on exact pixels, compose to the **tightest union**.

### 3.2 Per-beat text

| Beat | Text | Placement | Hold |
|---|---|---|---|
| Clip 1 hook | *"24K Gold Plating. Real Craft."* | **upper-safe center band** (clear of top UI) | full 4 s, fades with the whip-out |
| Clip 5 authenticity | `[OWNER-SLOT]` fact(s) as a clean **lower-third** — only TRUE owner values; if no hallmark, no hallmark text | center-lower-third, above the ~320 px bottom band | ~4 s |
| Clip 6 CTA | **"Link on screen · in bio to buy"** + `[OWNER-SLOT: @handle]` (+ `[OWNER-SLOT: purchase link URL]` if one exists) | **center / center-lower-third**, generous negative space, clear of bottom + top-right | enters ~19.0 s, **holds the full 3 s** for an unhurried read |

A subtle **"in bio" echo** may ride the last ~1 s of clip 5 to pre-load the CTA.

### 3.3 Typography & contrast (quiet-luxury)

- **Font:** one restrained serif (or a clean high-weight sans), **bold/medium weight — never thin or script** (thin type dies at grid/thumbnail size and reads cheap).
- **≤ 2 tones, no gradient, no ornament, no drop-shadow clutter.** A single soft scrim behind the CTA text is allowed if contrast needs it.
- **Contrast must survive both dark and light app modes** and must survive over warm gold — cream/off-white text on the deep-anchor wardrobe or a subtle scrim, not gold-on-gold.
- The text is part of the **silent-legibility floor**: every spoken claim is mirrored on-screen so the ad works fully on mute.

### 3.4 AI disclosure (not optional)

This is photoreal AI media in a commercial ad: publish with the **AI-info label ON**. The disclosure is a publish-time toggle, not an on-screen graphic, but it is part of this deliverable's checklist and is flagged here so it is not dropped.

---

## 4. Cover / Poster Selection

### 4.1 The frame

Default to a **selected real frame from the reel** (keeps the grid feed-native and honest — the cover must never promise footage the reel lacks) `[FACT/excerpt]`. **Never accept the autoframe default.** Two viable sources, in order:

1. **The hook flare frame (clip 1)** — the forearm/kada throwing the diamond-cut sparkle in warm low light. This is the strongest thumbnail: it *is* the product, it poses the luxury **curiosity/access question** ("this gold — whose? from where?"), and it is unmistakably a jewelry frame at a glance.
2. **The band macro (clip 3)** — pure craftsmanship, cross-hatch + rails filling frame. Strongest for a **grid that leans product-detail**; weaker on face-driven click but very strong on "this account is about *this* gold."

For a **face-forward variant** (faces out-click faceless covers, close-up eye contact best `[FACT/excerpt]`), the presenter-with-kada reveal frame is the option — but the expression stays **poised intrigue, not open-mouthed shock** (shock reads clickbait and breaks the classic-luxury register). The `~25%` expressive-vs-neutral CTR figure is an aggregator `[ESTIMATE]`, never quoted as a platform stat.

### 4.2 Cover QC — it is judged cropped and small, not full-frame `[FACT/excerpt]`

- **Center-square survival:** covers crop differently in feed, grid and Explore. The hero gold (and any text) must sit **inside the center square** of the 1080×1920; if a face is used, keep it in the **middle 50% vertically** so no crop beheads it.
- **~150 px thumbnail death-test:** render the candidate at grid width on **both dark and light** backgrounds. The gold sparkle must still pop and any text (≤ 3–5 bold words) must read in under a second. A cover that only works at full size is a failed cover.
- **Runs the same nine-axis rubric** as any frame (total ≥ 12/18, no zero on a starred axis: exposure / sharpness-distribution / physics-realism). A garbled terminal motif or plastic gold **fails the cover**, same as it fails the shot.

### 4.3 Grid coherence for a jewelry account `[FACT/excerpt + ESTIMATE]`

The profile grid must read as **one brand** the moment a buyer lands on it:
- **Same house LUT** on every cover (the §2.6 warm-neutral gold look) — the grid's warmth is the brand signature.
- **One repeatable template:** same type placement, same margin logic, same single serif, generous negative space. Restraint *is* the click driver in this genre.
- **Deliberate cadence** across posts: hero product-flare / craft-macro / presenter, not three near-identical macros in a row.
- The gold is the recurring visual key — every tile should have the **true warm gold** (not the orange/plastic drift), which is precisely why the cross-model gold match in §2.4 protects grid coherence, not just this one reel.

### 4.4 Iteration is observational, not a cover-only A/B `[FACT/excerpt]`

Instagram **Trial Reels test the whole reel to non-followers, not the cover alone** (public accounts, 1,000+ followers; the `40%` / `80%` figures are held one notch below platform-confirmed). So cover iteration is **observational**: hold everything else constant, vary **one** cover variable (flare-frame vs face-frame vs macro-frame), read grid-sourced views. That decision — and posting cadence — belongs to the **media-manager specialist**; this part delivers the crop-safe, thumbnail-legible, grid-coherent cover candidate(s) ready for it.

---

## 5. Handoff Checklist

- [ ] Six clips assembled `4/4/4/3/4/3`, five seams conformed to downbeats; **cold open on the flare, no intro frame**.
- [ ] VO L-cut across the macro seam, J-cut into the CTA; ends on a **soft open loop** (offer through the last frame, no hard loop).
- [ ] Grade chain run in order; **skin AND gold** qualified out of the look and held to saved targets.
- [ ] **Per-model trim presets** applied to the model-B clips (4, 6) *before* hero matching; every clip matched to the hero on scopes; **gold hue pinned across all six clips** so the match-cut/invisible-cut gold is continuous.
- [ ] Gold specular **rolled to a warm-gold shoulder, not clipped white**; cool shadows kept off gold and skin; any "plastic" read escalated to regeneration + realism reviewer, **not** grade-patched.
- [ ] Banding/crushed-black countermeasures + **phone test upload** passed; watermark-free 1080 Rec.709/SDR.
- [ ] All text composited, in safe zones (link/handle **out of** the bottom + right-stack bands); ≤ 3–5 bold words, contrast survives dark+light; CTA holds full 3 s.
- [ ] **No fabricated hallmark/stamp** anywhere; authenticity `[OWNER-SLOT]`s show only TRUE values (no hallmark → no hallmark text); no invented chain in frame.
- [ ] Cover = selected real frame, center-square + ~150 px + rubric passed, grid-coherent under the house LUT; autoframe **not** used.
- [ ] Publish-time: **AI-info label ON**, honest caption (media-manager owns the caption text); presenter framed as brand model, not testimonial.

**Owner-blocking before any of this ships:** the owner's **KADA reference photo uploaded via the Higgsfield media-upload widget at run time** (carries the exact bracelet), and the authenticity/distribution `[OWNER-SLOT]` values for the lower-third and CTA. No chain shot until a chain reference exists.
