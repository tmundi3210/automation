# MANUAL GENERATION GUIDE — `ad01_gold_kada` (do it yourself in the Higgsfield UI)

Copy-paste prompts for the whole ad. **Brand: rhodiumgold · @rhodiumgold · product = 24K GOLD PLATING (gold-plated, never say "solid gold").** Cut = 7 short clips → stitch to ~24 s, 9:16.

## Global settings (every generation)
- **Aspect ratio: 9:16** (set this FIRST — it silently removes some models).
- **Stills:** 4K where offered. **Video:** 1080p.
- **Native audio: OFF** on every video clip (we add voice + music in edit).
- **Banned words** in every prompt (they cause the plastic AI look): *flawless, perfect, smooth, airbrushed, beauty, ring light, ethereal glow, max saturation, hyperdetailed, 8K, masterpiece.*

## Assets to have ready
1. **KADA photo** — upload YOUR real bracelet photo; use it as the **image reference** on every kada shot (keeps the exact engraving).
2. **Presenter A** — already generated (job `4e74d9c4…`, seed 246906). Make it a **Character / Reference Element** (or train a Soul from it + a few extra angles) so his face is identical in shots 2 & 6.
3. **Chain A** — already generated (job `295bf33f…`). Use as the **chain reference** in shots 2 & 5.

---

# THE 7 SHOTS
Each shot = generate a START IMAGE, then feed it to a VIDEO model (image-to-video). Video prompts describe **motion only** — don't re-describe the picture, it kills the motion.

## SHOT 1 — HOOK (4 s) · kada on wrist, gold flare
**Start image** — model `nano_banana_pro`, 4K, attach your **kada photo** (image reference):
```
A photo of a man's forearm turned toward the lens in warm low-key light, a high-polish warm yellow-gold kada catching a single diamond-cut flare across the band, tailored unbranded dark sleeve, relaxed hand softly out of frame. Macro 90mm, f/2.8, shallow depth of field, warm key from camera left with a cool edge from the right, visible metal micro-reflections and true gold color, subtle film grain. Palette: warm gold, honeyed amber, espresso-brown, cream, near-black.
```
**Video** — model `seedance_2_0` (1080p, mode std), start image = above, attach kada photo as reference, audio OFF:
```
Slow forearm rotation toward the lens over a 4-count; the gold band turns and a diamond-cut specular flare travels across it; lens static, then a fast whip-pan blur begins on the final beat. Fingers stay soft, out of frame. Keep the warm-key / cool-edge lighting and gold color constant.
```

## SHOT 2 — REVEAL (4 s) · presenter wearing kada + chain
**Start image** — model `cinematic_studio_2_5` (or any model that takes 2 references), 4K, attach **Presenter A** (character) + **kada photo** + **Chain A**:
```
A photo of the presenter, chest-up, looking down calmly at a warm yellow-gold kada on his wrist and lifting it slightly to the light, a matching warm yellow-gold Cuban-curb chain at his collar. A man in his late 40s, warm mid-brown-to-tan skin with natural texture — visible pores, fine lines, realistic uneven tone — neat salt-and-pepper hair, close groomed stubble, calm quietly-authoritative expression; unbranded deep-navy jacket over a fine open-collar cream shirt, no logos. Soft window key from camera left, faint cool fill right, 50mm f/2.0, natural skin texture. Palette: warm amber, cream, deep navy, slate-blue, gold. Exact bracelet + chain from the reference images.
```
**Video** — model `seedance_2_0`, start image = above, keep the presenter reference, audio OFF:
```
Slow push-in from chest-up toward the wrist over 4 seconds; his gaze settles on the kada, a small calm approving breath, wrist steady, no fidget; motion ends framed tight on the gold. Mouth relaxed, near-neutral, no dialogue. Hold identity, wardrobe and grade constant.
```

## SHOT 3 — BAND MACRO (4 s) · kada texture
**Start image** — model `nano_banana_pro`, 4K, attach **kada photo**:
```
A photo of an extreme macro of a men's warm yellow-gold kada band filling the frame, fine engraved cross-hatch wheat-grain texture throwing a shimmering diamond-cut sparkle, two bright polished rails top and bottom. Macro 100mm, f/4, controlled directional side light with one specular roll, visible metal micro-reflections, accurate warm gold, subtle grain, no hands, no stamps or hallmarks. Palette: warm gold, honeyed amber, espresso-brown, cream, near-black.
```
**Video** — model `seedance_2_0` (1080p), start image = above, attach kada photo, audio OFF:
```
Slow lateral tracking along the band over 4 seconds, a single specular highlight rolling across the wheat-grain cross-hatch and the two polished rails; shallow depth of field; ends continuing toward the terminal for a rack-focus hand-off. No hands, no text. Keep the exact engraving, rails and gold color constant.
```

## SHOT 4 — TERMINAL MACRO (3 s) · the temple/Greek-key motif
**Start image** — model `nano_banana_pro`, 4K, attach **kada photo** (reject any garbled/asymmetric motif — regenerate):
```
A photo of an extreme macro of the terminal openwork temple / Greek-key motif panel of a men's warm yellow-gold kada, the open cuff end reading clearly, deep warm negative space behind. Macro 100mm, f/4, controlled directional light, visible metal micro-reflections, accurate gold, subtle grain, motif crisp and symmetric, no hands. Palette: warm gold, honeyed amber, espresso-brown, cream, near-black.
```
**Video** — model `wan2_7` (1080p — use this, NOT Seedance, because 3 s is below Seedance's minimum), start+end, audio OFF:
```
Rack focus pulling from soft foreground onto the terminal openwork motif over 3 seconds, the geometric panel resolving to sharp; a whip-pan blur begins on the last beat. No hands, no text. Hold the motif geometry and gold color exactly.
```

## SHOT 5 — CHAIN HERO MACRO (3 s) · the matching chain  [NEW]
**Start image** — model `nano_banana_pro`, 4K, attach **Chain A** (use it as the reference so it stays the same chain):
```
A photo of an extreme macro of a men's warm yellow-gold Cuban-curb chain, substantial solid links with crisp beveled edges, high-polish gold, arranged in a gentle S-curve on a matte charcoal gradient surface. Macro 100mm, f/4, controlled directional side light with one specular roll across the links, visible metal micro-reflections, accurate warm gold, subtle grain, no hands, no text. Palette: warm gold, honeyed amber, espresso-brown, cream, near-black.
```
**Video** — model `wan2_7` (1080p, 3 s), start+end, audio OFF:
```
Slow lateral drift along the chain over 3 seconds, a single specular highlight travelling link to link; shallow depth of field; the piece perfectly still otherwise. No hands, no text. Keep the link shape and gold color constant.
```

## SHOT 6 — AUTHENTICITY (4 s) · presenter beside the piece
**Start image** — model `cinematic_studio_2_5`, 4K, attach **Presenter A** + **kada photo** (leave empty lower third for a text card):
```
A photo of the presenter, medium shot, standing calmly beside a warm yellow-gold kada resting on a soft-lit honed-marble stand. A man in his late 40s, warm tan skin with natural texture, salt-and-pepper hair, groomed stubble, calm authoritative expression; unbranded deep-navy jacket, cream open-collar shirt, no logos. Warm cinematic grade, soft key camera left, cool rim behind, 50mm f/2.2, natural skin texture, generous quiet negative space in the lower third. Palette: warm amber, cream, deep navy, slate-blue, gold. Exact bracelet from the reference photo. No on-image text.
```
**Video** — model `seedance_2_0`, start image = above, keep presenter reference, audio OFF:
```
Slow push-in and settle over 4 seconds; the man gives a single calm approving nod toward the kada on the stand, then holds still; hands relaxed. No dialogue lip-sync. Keep identity, wardrobe, grade and the empty lower-third constant.
```

## SHOT 7 — CTA (3 s) · product at rest
**Start image** — model `nano_banana_pro`, 4K, attach **kada photo** (leave space for the CTA card):
```
A photo of a men's warm yellow-gold kada at rest on a soft-lit honed-marble surface, clean and premium, generous quiet negative space around it for a text card, kept clear of the bottom and top-right corners. Macro 85mm, f/4, controlled directional light, visible metal micro-reflections, accurate gold, subtle grain. Palette: warm gold, honeyed amber, espresso-brown, cream, near-black. No on-image text.
```
**Video** — model `wan2_7` (1080p, 3 s), start+end near-identical, audio OFF:
```
Near-static hold for 3 seconds, a slow subtle specular glint drifting across the gold band, the piece perfectly still; no camera move beyond a faint settle. Keep the negative space and gold color constant.
```

---

# VOICEOVER SCRIPT (record or TTS — warm, calm, unhurried)
1. (Hook) **"This is what twenty-four-karat gold plating looks like."**
2. (Reveal) **"A men's gold-plated kada — classic, made to be worn every day."**
3. (Band) **"Look at the work — hand-finished cross-hatch, diamond-cut to catch the light."**
4. (Terminals) **"And the temple motif at the ends — the detail that tells you it's built to last."**
5. (Chain) **"With a matching chain — same twenty-four-karat gold plating, same finish."**
6. (Authenticity) **"Twenty-four-karat gold plating, crafted by rhodiumgold."**
7. (CTA) **"Yours from rhodiumgold. Link on screen, or in our bio, to buy."**

# ON-SCREEN TEXT (add in your editor — NOT generated in the video)
- Hook overlay (upper third): **24K Gold Plating · Real Craft**
- Authenticity lower-third: **24K Gold Plating · rhodiumgold**   *(no hallmark/BIS — it's plated)*
- CTA card (shot 7, centred, clear of bottom UI): **rhodiumgold** / **@rhodiumgold** / **Link in bio to buy**

# HONESTY RULES (do not break — it's a real ad)
- Say **"gold plating / gold-plated / 24k gold plating"** — **never** "solid gold", "pure gold", "real gold", or a karat purity like "22k/24k gold".
- **No hallmark / BIS / "certified" / "guaranteed pure"** language — a plated piece has no gold-purity hallmark.
- Turn ON Instagram's **AI-content label** when you post (it's an AI-made video).
- Don't fake scarcity ("only 2 left") unless it's literally true.

# EDIT & POST
Stitch the 7 clips in order → add the VO + a licensed music track (Meta Sound Collection is free for Instagram) → add the text overlays above → export **watermark-free**, 9:16 → post with the AI label ON.
