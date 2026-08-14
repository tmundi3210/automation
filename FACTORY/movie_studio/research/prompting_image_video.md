# Prompt Craft for Photoreal AI Image & Video Generation

Scope: how to write prompts for photoreal image and video generation — anatomy, camera/lighting language, image-to-video, start/end-frame, photorealism vs the "plastic AI look", negative prompts, iteration protocol, style/character reference, failure modes — grounded in official model docs where reachable, with luxury-lifestyle worked examples. Written 2026-07-03 for the movie_studio vertical (owner priority #1: "it should not look AI").

## 0. Grounding note (what was reachable)

- [FACT] Direct fetches to `ai.google.dev`, `docs.cloud.google.com`, `help.runwayml.com`, `docs.midjourney.com`, `docs.bfl.ai`, `docs.byteplus.com`, `kling.ai/blog`, `cookbook.openai.com`, `higgsfield.ai`, and `web.archive.org` were 403-blocked by the egress proxy in this session.
- [FACT] Successfully fetched in full: Google Cloud official blog "Ultimate prompting guide for Veo 3.1" [S1]; the OpenAI Cookbook Sora 2 prompting guide via its GitHub raw source [S4]; a GitHub mirror of Google's official Imagen prompt guide [S9]; two community prompting repos [S2][S11].
- Content from the blocked official pages (Runway, Midjourney, BFL/FLUX, Kling, MiniMax, Seedream, Higgsfield) is carried below via search-result excerpts that quote those pages. Tag used: **[FACT — official page content via search excerpt; direct fetch blocked]**. Treat these as one notch below fully-fetched [FACT]; re-verify exact wording from an unblocked network before hard-coding into a gated KB.

## 1. Anatomy of a strong IMAGE prompt

### 1.1 Core skeleton (converges across vendors)
- [FACT — mirror of official Google Imagen guide, S9] Imagen's stated anatomy: **Subject** ("the object, person, animal, or scenery") + **Context and background** + **Style**. Photographic style is triggered by starting the prompt "A photo of…". Quality modifiers: general ("high-quality, beautiful, stylized"), photo-specific ("4K, HDR, Studio Photo").
- [FACT — official docs.bfl.ai content via search excerpt, S8] FLUX.2: framework **Subject + Action + Style + Context**; "word order matters — FLUX.2 pays more attention to what comes first," priority order: main subject → key action → critical style → essential context → secondary details. Length bands: 10–30 words for exploration, **30–80 words "ideal for most projects"**, 80+ for complex scenes.
- [FACT — official Midjourney docs content via search excerpt, S5] Midjourney: "A text prompt is the text description of what you want to see. Avoid making long lists or detailed instructions; these can confuse the process." Describe what you DO want; any word in the prompt is treated as something to generate.
- [FACT — Seedream official/BytePlus-derived guidance via search excerpts, S10] Seedream 4.x: structure `[Subject] + [Action/Pose] + [Environment/Setting] + [Style] + [Technical Details] (+ [Text Content])`; 30–100 words; subject first, then style/context, then finishing details; highly responsive to lighting cues ("golden hour lighting," "dramatic side lighting," "soft diffused light," "moody low-key lighting").
- [ESTIMATE — synthesis of the above, high confidence] Working canonical image-prompt template for this studio:
  `SUBJECT (who/what, physical specifics) → ACTION/POSE → ENVIRONMENT (place, time of day, weather) → LIGHTING (source, direction, quality, color) → CAMERA (shot size, angle, lens mm, aperture, film/sensor) → COMPOSITION (rule of thirds, foreground/background) → STYLE/GRADE (editorial, palette 3–5 named colors) → QUALITY tokens.`

### 1.2 Camera/lens/film vocabulary (image)
- [FACT — S9, mirror of official Imagen guide] Photography modifier categories with official example keywords:
  - Camera proximity: "close up", "taken from far away"
  - Camera position: "aerial", "from below"
  - Lighting: "natural", "dramatic", "warm", "cold"
  - Camera settings: "motion blur", "soft focus", "bokeh", "portrait"
  - Lens types: "35mm", "50mm", "fisheye", "wide angle", "macro"
  - Film types: "black and white", "polaroid"
- [FACT — S9] Imagen's photorealism-by-use-case table (official): **Portraits** → prime/zoom lens, 24–35mm, "black and white film, film noir, depth of field, duotone"; **Objects/food/plants** → macro 60–105mm, "high detail, precise focusing, controlled lighting"; **Sports/wildlife (motion)** → telephoto zoom 100–400mm, "fast shutter speed, action or movement tracking"; **Astronomical/landscape (wide)** → wide-angle 10–24mm, "long exposure times, sharp focus, smooth water or clouds".
- [FACT — S8 via search excerpt] FLUX understands natural-language camera/film specs, e.g. official examples: "captured on expired Kodak Ektachrome 64 slide film cross-processed from 1987 with a 35mm spherical lens at f/5.6"; "shot with 85mm lens at f/2.8, shallow depth of field".
- [ESTIMATE — craft synthesis, high confidence] For luxury portraiture use 50–105mm at f/1.8–f/4 (flattering compression + subject separation); for cars and architecture 24–35mm at f/5.6–f/8 (context + sharpness); name a real film stock or "full-frame mirrorless" to anchor color science.

## 2. Anatomy of a strong VIDEO prompt

### 2.1 Per-model official formulas
- [FACT — fetched official Google Cloud blog, S1] Veo 3.1 five-part formula: **[Cinematography] + [Subject] + [Action] + [Context] + [Style & Ambiance]**. Camera movement vocabulary: dolly shot, tracking shot, crane shot, aerial view, slow pan, POV shot. Composition: wide shot, close-up, extreme close-up, low angle, two-shot. Lens/focus: shallow depth of field, wide-angle lens, soft focus, macro lens, deep focus.
- [FACT — S1] Veo audio syntax: dialogue in quotation marks (`A woman says, 'We have to leave now.'`); `SFX: thunder cracks in the distance`; `Ambient noise: the quiet hum of a starship bridge`.
- [FACT — S1] Veo timestamp prompting for multi-shot pacing inside one generation: `[00:00-00:02] Medium shot… [00:02-00:04] Reverse shot… SFX: …` etc.
- [FACT — Kling official prompt-guide content via search excerpts, S6] Kling text-to-video formula: **Subject + Subject Movement + Scene + (Camera Language + Lighting + Atmosphere)**. API caps prompt AND negative prompt at 2,500 characters each. Replace vague words like "cinematic" with explicit camera terms such as "slow dolly-in".
- [FACT — official platform.minimax.io content via search excerpt, S7] MiniMax/Hailuo Director-capable models (I2V-01-Director, Hailuo-02, 2.3, 2.3-Fast) accept 15 bracketed camera commands: `[Truck left/right]`, `[Pan left/right]`, `[Push in]/[Pull out]`, `[Pedestal up/down]`, `[Tilt up/down]`, `[Zoom in/out]`, `[Shake]`, `[Tracking shot]`, `[Static shot]`. Multiple commands in one bracket act simultaneously (max ~3 recommended); sequential brackets apply in order; natural language works but explicit commands are more accurate.
- [FACT — fetched OpenAI Cookbook via GitHub raw, S4] Sora 2: "Think of prompting like briefing a cinematographer who has never seen your storyboard." Recommended blocks: prose scene description (characters, costumes, scenery, weather) → cinematography (framing, mood) → action beats as bullets → dialogue block. "Detailed prompts give you control and consistency, while lighter prompts open space for creative outcomes." "Using the same prompt multiple times will lead to different results – this is a feature, not a bug."
- [FACT — S4] Sora 2 motion rule: **"Each shot should have one clear camera move and one clear subject action."** Actions in counts/beats: weak "actor walks across the room" vs strong "actor takes four steps to the window, pauses, and pulls the curtain in the final second."
- [FACT — official Runway help-center content via search excerpts, S3] Runway Gen-4: the input image "establishes the visual starting point… allowing you to focus on describing the desired motion."
- [ESTIMATE — community synthesis of Google guidance, medium confidence, S13] Optimal Veo prompt length ≈ 3–6 sentences / 100–150 words; both over-compressed one-liners and rambling paragraphs underperform.

### 2.2 Temporal phrasing & duration control
- [FACT — S1] Veo 3.1 supports explicit `[mm:ss-mm:ss]` segment prompting — the strongest documented duration-control device.
- [FACT — S4] Sora: match dialogue length to clip length ("keep lines concise… so the timing can match your clip length"); describe action in beats/counts to fill the clip deterministically.
- [ESTIMATE — craft, high confidence] For the owner's 3–6 s stitched-reel format: write ONE camera move + ONE action per clip; phrase timing as beats ("in the final second", "holds for two beats"); never ask a 5 s clip for more than one micro-event — multi-event asks are the top cause of rushed, physics-breaking motion.

## 3. Image-to-video prompting (the start frame is the contract)

- [FACT — official Runway guide content via search excerpt, S3] "Effective image to video prompts focus almost exclusively on motion. Rather than describing elements present in the image, use your prompt to describe the motion of the scene." Official wrong/right pair: ❌ "The tall man with black hair wearing a blue business suit and red tie reaches out his hand for a handshake" → ✅ "The man extends his arm to shake hands, then nods politely." Re-describing image content in detail "can lead to reduced motion or unexpected results."
- [FACT — S6 via search excerpt] Kling image-to-video: drop the Subject and Scene blocks the image already supplies; lead with **Subject Movement + Camera Language**.
- [FACT — S4] Sora image input: "The model uses the image as an anchor for the first frame, while your text prompt defines what happens next." Image must match target video resolution (JPEG/PNG/WebP).
- [ESTIMATE — synthesis, high confidence] Studio rule: in i2v mode the prompt budget is ~90% motion (subject motion + camera move + tempo), ~10% atmosphere continuation (light behavior, ambient sound where supported). Anything visual you want in the clip should be IN the start image, not re-stated in words.

## 4. Start-frame + end-frame interpolation

- [FACT — S1] Veo 3.1 "first and last frame": "Generate a natural video transition between a provided start image and end image, complete with audio." Also "ingredients to video": reference images of scene/character/object/style keep a consistent aesthetic across shots.
- [FACT — vendor/partner guides via search excerpts, S12] Kling start/end-frame mode interpolates between the two supplied compositions. Practice guidance from those guides: keep the two frames consistent in color palette, tone and setting for smooth transitions; a light prompt often suffices, adding detail only for exact camera/action control; a detailed prompt should narrate start scene → action/camera → arrival at end scene; some surfaces let you reference the frames as `@image1` / `@image2`; if the transition feels off, make the frames closer in style; too much motion → remove extra camera directions; shorter clips interpolate more cleanly, dramatic jumps cause flicker.
- [ESTIMATE — craft, high confidence] For stitched reels: generate the NEXT clip's start frame as a near-match of the previous clip's final frame (same lens, light, grade) — this is the cheapest "camera trick" for invisible cuts; alternatively use deliberate end→start mismatches as whip-pan/match-cut transitions.
- [UNKNOWN] Which exact Higgsfield MCP video models expose end-frame conditioning and under what parameter names — must be read off the live MCP surface (`models_explore`) by the platform-surface research file; not invented here.

## 5. Photorealism patterns — and what causes the "plastic AI look"

### 5.1 Wording that pushes toward natural
- [FACT — multiple concordant practitioner guides via search, S14] Why AI skin looks plastic: models are trained on heavily retouched, beauty-filtered photos, so over-smooth skin is the learned default; real skin has pores, fine lines and color shifts that scatter light unevenly — precisely what the default erases.
- [FACT — S14] Positive-wording fixes (concordant across guides): "natural skin texture with visible pores, subtle fine lines, realistic uneven skin tone, slight natural shine"; demand imperfections (freckles, natural color variation, minor blemishes); specify real-photography optics ("85mm lens", "f/1.8", "natural window lighting", side/back light); prefer **directional side light over flat ring light** so micro-shadows read as texture; subtle film grain restores realism.
- [FACT — S9] Official Imagen route to photorealism: lead with "A photo of…" + the use-case lens/film table in §1.2.
- [FACT — S4] Sora: concrete light anchors beat adjectives — "Soft window light with warm lamp fill and a cool edge from the hallway" vs weak "brightly lit room"; naming 3–5 palette colors stabilizes grade across shots.
- [ESTIMATE — synthesis, high confidence] Physically-plausible-light heuristic: every prompt names (a) the light SOURCE(s), (b) DIRECTION, (c) QUALITY (hard/soft), (d) COLOR temperature contrast. Believable motion heuristic: verbs with mass ("settles", "sways", "the coat drags in the wind") outperform abstract verbs ("moves dynamically").

### 5.2 Wording that CAUSES the plastic look
- [FACT — S14, concordant] "flawless skin", "perfect", "smooth skin", "beauty", "airbrushed" push toward the retouched training mode; flat frontal "ring light" wording kills texture.
- [ESTIMATE — practitioner consensus, medium-high confidence] Stacked hype tokens ("ultra-realistic, 8K, masterpiece, hyperdetailed, trending") on modern natural-language models (FLUX.2, Imagen 4, Seedream 4) add little and can drag toward CG-render aesthetics; the modern replacements are concrete optics + named film stock + light physics. (Legacy SD-class models did respond to quality-token stacking — model-class dependent.)
- [ESTIMATE — craft, high confidence] Also avoid: golden-glow-everywhere ("ethereal glow"), max saturation asks, "perfect symmetry" for faces, and describing people as "beautiful/stunning/perfect" instead of physically (age, build, skin, grooming).

## 6. Negative-prompt practice (where supported)

- [FACT — S5, official Midjourney docs via search excerpt] Midjourney: never write "no cake" in the prompt body (cake will appear); use `--no thing1, thing2`. Caveat from official docs: `--no modern clothing` parses as "no modern" AND "no clothing" — can even trigger moderation; keep `--no` items single-concept. `--no` ≈ multi-prompt weight −0.5.
- [FACT — S1 + S13] Veo negative prompts: list unwanted elements as nouns/descriptions, NOT instructive language — official example: describe "a desolate landscape with no buildings or roads" style exclusions as content words; don't write "no"/"don't show" instructions inside the main prompt.
- [FACT — S6 via search excerpt] Kling supports a dedicated negative prompt field (2,500-char cap).
- [FACT — S8 via search excerpt] **FLUX.2 does not support negative prompts** — "focus on describing what you want, not what you don't want."
- [FACT — S2, community Veo guide] Widely-used exclusion list for clean output: "subtitles, captions, watermark, text overlays, cartoon effects, poor lighting, blurry faces, distorted hands, compression artifacts". [ESTIMATE — medium confidence that this exact list is optimal; it is community craft, not official.]
- [ESTIMATE — synthesis, high confidence] Studio default negative block (for models with a negative field): `plastic skin, waxy, airbrushed, over-smoothed, doll-like, 3d render, cgi, beauty filter, extra fingers, warped hands, morphing, flicker, watermark, subtitles, text overlay, oversaturated` — pruned per model since long negatives can suppress wanted detail.

## 7. Prompt iteration protocol

- [FACT — S4, official] Sora: "Editing is for nudging, not gambling. Use it to make controlled changes – one at a time," naming the change: "same shot, switch to 85 mm", "same lighting, new palette: teal, sand, rust." Keep what works; don't rewrite the whole prompt when a result is close.
- [FACT — S4] Same prompt ⇒ different results by design (stochastic sampling), so A/B judgments need multiple samples per prompt variant, not one-vs-one.
- [FACT — S9 domain knowledge + vendor API surfaces generally] Most image APIs expose a `seed` parameter; fixing the seed while changing one prompt axis isolates that axis's effect. [UNKNOWN] Whether the Higgsfield MCP `generate_image`/`generate_video` tools expose seed control per model — must be read off the live tool schemas; do not assume.
- [ESTIMATE — synthesis, high confidence] Studio loop: (1) lock a baseline prompt from the templates here; (2) generate N≥2 samples; (3) diagnose against the failure-mode taxonomy (§9); (4) change EXACTLY ONE axis (lens, light direction, action verb, palette, camera move) and state the change; (5) keep a prompt-change log so wins are reproducible; (6) promote the winning phrasing into the persona/genre style block. This is the concrete form of the owner's required feedback loop.

## 8. Style-reference and character-reference prompting

- [FACT — official Midjourney docs + changelog via search excerpts, S15] Midjourney V7 **Omni-Reference** (`--oref <image>` + `--ow 0–1000`, default 100): "put THIS specific thing into my new image" — successor to V6 `--cref`. Low `--ow` 25–100 = loose influence; 200–400 = balanced (most used); 600–1000 = near-copy of face/object/style. Costs 2× GPU; incompatible with Vary Region/Pan/Zoom. Combine `--oref` (identity) with `--sref` (aesthetic) to keep one character across restyled scenes.
- [FACT — S1] Veo 3.1 "ingredients to video": reference images for scene, character, object or style hold a consistent aesthetic across shots (with audio).
- [FACT — S11, community Higgsfield skill repo, fetched] Higgsfield-ecosystem craft: **Soul ID** provides character consistency; hard rule = separate the identity block (unchanging Soul ID / character anchor) from shot-specific motion+camera text; Cinema Studio ships preset color palettes and camera movesets; an "Elements" system layers reference elements. [ESTIMATE — medium confidence: community documentation of the platform, not official docs; verify names/params against the live MCP surface (`show_characters`, `show_reference_elements`, `models_explore`).]
- [FACT — higgsfield.ai marketing pages via search excerpts, S16] Higgsfield Soul is positioned as a high-realism photo model with 20+ curated presets and internal prompt enhancement; marketing claims "photorealistic skin, fabric, and lighting that avoid plastic textures". [Marketing claim — treat quality level as unverified until credit-spending trials.]
- [ESTIMATE — synthesis, high confidence] Persona-consistency prompt pattern: fixed CHARACTER BLOCK (verbatim, reused every shot: name-tag, age, build, skin, hair, signature grooming) + fixed WARDROBE BLOCK (from the pre-built capsule) + variable SHOT BLOCK (action, camera, light). Never paraphrase the character block between shots — paraphrase drift is the main text-side cause of identity drift.

## 9. Failure modes per generator class + prompt-level mitigations

- [FACT — arXiv survey of AI-generated video evaluation via search excerpt, S17] Six failure classes: technical (blur/low-res), dynamic (no meaningful motion), physical (behavior violating physical common sense), consistency (identity/appearance changes mid-clip), quality (distorted structures/details), alignment (output deviates from the prompt).
- [FACT — S17/S14 concordant] Hands merge/extra fingers, especially when overlapping objects; models mimic the look of motion without mass/gravity/momentum; weak object permanence turns background shifts into "melting"; upscaling/sharpening amplifies temporal noise into flicker.
- Prompt-level mitigations, per class [ESTIMATE — synthesis of §§2–6 sources, high confidence]:
  - dynamic-error (frozen clip): add an explicit camera command + one concrete subject action with a beat count (Runway/Kling i2v guidance).
  - physical-error: fewer simultaneous events; verbs with mass; avoid liquids/glass-pouring, dense crowds, fast hand-object interaction in hero shots.
  - consistency-error: reference-image conditioning (§8), fixed character block, shorter clips.
  - hands: frame to keep hands out of macro focus (pockets, behind back, holding large simple objects), or plan a compositing fix.
  - alignment-error: front-load the must-have element (FLUX word-order rule generalizes); one-axis iteration to find which clause is being dropped.
- [ESTIMATE — medium confidence] Class differences: image models fail mostly on anatomy/text/logos; i2v fails mostly on motion amount and identity drift; t2v adds composition lottery on top — which is why the studio pipeline should be image-first (curate the frame) then i2v, matching the owner's start/end-frame idea.

## 10. Worked example prompts — luxury-lifestyle genre

All prompts in this section are **authored examples** ([ESTIMATE] craft applying the sourced patterns above — not sourced claims). Persona names are placeholders.

**E1 — IMAGE, male persona portrait (Imagen/FLUX/Seedream style, §1)**
"A photo of a man in his early 30s, athletic build, short dark textured hair, light stubble, natural skin texture with visible pores and subtle fine lines, wearing an unstructured navy wool blazer over a cream knit polo. He leans against the stone balustrade of a Lake Como terrace at golden hour, warm low sun from camera left, cool open-sky fill from the right. Shot on 85mm lens at f/2.0, shallow depth of field, waist-up, eye level, Kodak Portra 400 color palette: warm amber, cream, deep navy, slate blue."

**E2 — IMAGE, female persona portrait**
"A photo of a woman in her late 20s, elegant posture, warm medium skin tone with realistic texture and natural shine, dark hair in a low chignon, wearing an ivory silk slip dress and vintage gold earrings. She sits at a marble hotel-bar counter in Paris at night, soft tungsten lamp glow from the right, faint cool window light behind her. 50mm lens at f/1.8, medium close-up, slight side angle, gentle film grain, palette: ivory, brass, espresso, midnight blue."

**E3 — IMAGE, luxury car environmental**
"A photo of a black 1960s grand-touring coupe parked on a wet cobblestone street in Milan at blue hour, streetlamp reflections streaking across the paint, warm boutique windows glowing in the background. Wide shot, 35mm lens at f/5.6, low angle from across the street, long exposure feel on the wet stone, deep focus, palette: charcoal, amber, teal-blue dusk."

**E4 — IMAGE, luxury place establishing frame (built to be a start frame)**
"A photo of an infinity pool edge meeting the Aegean Sea at sunrise, white Cycladic architecture, a linen-draped breakfast table with silver coffee service in the foreground right, no people. Wide-angle 24mm at f/8, sharp focus front to back, soft pink-gold morning light from the horizon, palette: white, sand, aqua, pale rose."

**E5 — I2V, from E1's frame (Runway/Kling i2v style: motion-only, §3)**
"The man turns his head slowly toward the lake and smiles slightly; a breeze moves his blazer lapel. Slow dolly-in from medium to medium close-up over the full clip. Warm sunlight flickers softly on the water behind him. Ambient noise: distant boat engine, water lapping."

**E6 — I2V, MiniMax Director syntax (bracket commands, §2.1)**
"[Push in] The woman lifts the espresso cup, takes one unhurried sip, and sets it down in the final second, [Static shot] her earrings catching the lamp light."

**E7 — T2V, Veo five-part with audio (§2.1)**
"Tracking shot, low angle alongside the front wheel: a dark-green vintage convertible glides along a coastal cliff road at golden hour, a man in sunglasses and a woman with a silk headscarf inside, hair moving in the wind. Mediterranean cliffs and sea haze in the background. Warm backlit sun flare, shallow depth of field on the chrome details, shot as if on 35mm film, slightly grainy. SFX: smooth engine hum, wind. Ambient noise: distant gulls."

**E8 — Start+end frame transition (stitch trick, §4)**
Start frame: E4 (empty pool terrace). End frame: same terrace, the female persona seated at the table mid-laugh. Prompt: "Camera holds nearly static with a very slow push-in; morning light brightens gradually; she enters from frame right, sits, and lifts the coffee cup as the shot settles." (Frames match in palette/lens/grade per §4 guidance.)

**E9 — Veo timestamped micro-story (multi-beat single generation, §2.2)**
"[00:00-00:02] Wide shot, marble hotel lobby, the man walks toward camera, tailored charcoal suit. [00:02-00:05] Medium tracking shot beside him, revolving door light sweeping across his face. SFX: leather soles on marble. [00:05-00:08] Close-up: he checks a dress watch, slight smile, elevator doors open off-screen casting gold light. Ambient noise: quiet lobby murmur, distant piano."

**E10 — Fitness accent shot (owner's "fitness sometimes")**
"A photo of the man mid-pull-up on a rooftop calisthenics bar at dawn, city skyline soft in the background, athletic long-sleeve top, visible effort in the forearms, natural skin sheen of light sweat. 35mm lens at f/4, low three-quarter angle, cool blue morning ambient with a warm rim of first sun from behind, fast-shutter crispness, palette: slate, steel blue, warm gold rim."

**E11 — Negative block usage (paired with E2 on a model with a negative field, §6)**
Negative: "plastic skin, waxy, airbrushed, beauty filter, doll-like, 3d render, oversaturated, extra fingers, warped hands, watermark, text overlay."
Midjourney form: append `--no watermark --no text` (single concepts only, per the official parsing caveat).

**E12 — Cover/poster frame (thumbnail-optimized still)**
"A photo of the two personas back-to-back in evening wear on a candlelit palazzo staircase, both looking toward camera, strong single key light from upper left, dark vignette edges, space at the top third of frame left visually quiet for a text overlay. 50mm at f/2.8, three-quarter length, rich contrast grade, palette: black, candle gold, oxblood, ivory."

## 11. Open unknowns for downstream files

- [UNKNOWN] Exact Higgsfield MCP parameter surface (seed, negative-prompt field, end-frame support, Soul ID/character params per model) — must be enumerated from the live MCP tools, not this file.
- [UNKNOWN] Current Runway Gen-4.5 and Kling 3.0 full official keyword lists (help pages 403-blocked; only excerpt-level content captured).
- [UNKNOWN] Whether Higgsfield's internal prompt enhancement rewrites user prompts before hitting the underlying models (would change how literally these templates transfer) — test empirically in the L5 run phase.

## Sources

1. Google Cloud official blog — "Ultimate prompting guide for Veo 3.1": https://cloud.google.com/blog/products/ai-machine-learning/ultimate-prompting-guide-for-veo-3-1 (fetched)
2. snubroot / Veo-3-Prompting-Guide (community): https://raw.githubusercontent.com/snubroot/Veo-3-Prompting-Guide/main/README.md (fetched)
3. Runway official help center — Gen-4 Video Prompting Guide: https://help.runwayml.com/hc/en-us/articles/39789879462419-Gen-4-Video-Prompting-Guide and Image to Video Prompting Guide: https://help.runwayml.com/hc/en-us/articles/48324313115155-Image-to-Video-Prompting-Guide (403-blocked; content via search excerpts)
4. OpenAI Cookbook — Sora 2 Prompting Guide: https://raw.githubusercontent.com/openai/openai-cookbook/main/examples/sora/sora2_prompting_guide.ipynb (fetched; canonical page https://cookbook.openai.com/examples/sora/sora2_prompting_guide blocked)
5. Midjourney official docs — Prompt Basics: https://docs.midjourney.com/hc/en-us/articles/32023408776205-Prompt-Basics and "No" parameter: https://docs.midjourney.com/hc/en-us/articles/32173351982093-No (403-blocked; content via search excerpts)
6. Kling official prompt guide: https://kling.ai/blog/kling-ai-prompt-guide (403-blocked; formula via concordant search excerpts incl. https://www.atlascloud.ai/blog/guides/kling-ai-video-prompt-guide , https://www.veed.io/learn/kling-ai-prompting-guide )
7. MiniMax official API docs — image-to-video / Director camera commands: https://platform.minimax.io/docs/api-reference/video-generation-i2v (content via search excerpt)
8. Black Forest Labs official docs — FLUX.2 prompting guide: https://docs.bfl.ai/guides/prompting_guide_flux2 (403-blocked; content via search excerpts)
9. Mirror of Google's official Imagen prompt guide: https://raw.githubusercontent.com/llegomark/google-imagen-3/main/imagen_prompt_guide.md (fetched; canonical https://ai.google.dev/gemini-api/docs/imagen blocked)
10. Seedream 4.x prompt guides: official BytePlus page https://docs.byteplus.com/en/docs/ModelArk/1829186 (403-blocked); excerpts via https://fal.ai/learn/devs/seedream-v4-5-prompt-guide and https://www.veed.io/learn/seedream-4-prompting-guide
11. OSideMedia / higgsfield-ai-prompt-skill (community): https://raw.githubusercontent.com/OSideMedia/higgsfield-ai-prompt-skill/main/README.md (fetched)
12. Kling start/end frame practice: https://tonaai.io/blog/kling-3-start-end-frame-tutorial , https://higgsfield.ai/blog/Kling-Start-End-Frames (blocked; excerpt), https://learn.rundiffusion.com/introducing-kling-o1-start-end-frame-precision-control-for-video-sequences/
13. Veo prompt-length community guidance: https://ltx.io/blog/veo-prompt-guide , https://www.visla.us/blog/guides/how-to-prompt-veo-3-and-veo-3-1/
14. Plastic-skin fixes (concordant practitioner guides): https://www.wearview.co/blog/fix-ai-skin-texture , https://morphic.com/resources/how-to/fix-ai-generated-skin-realistic , https://andyhtu.com/fixing-plastic-ai-skin/ , https://www.magnific.com/blog/how-to-fix-plastic-skin-ai-skin-enhancer/
15. Midjourney Omni Reference: official docs https://docs.midjourney.com/hc/en-us/articles/36285124473997-Omni-Reference (blocked; excerpts) + changelog https://updates.midjourney.com/omni-reference-oref/ + https://www.imaginepro.ai/blog/2025/7/midjourney-omni-reference-guide
16. Higgsfield Soul pages (marketing): https://higgsfield.ai/soul , https://higgsfield.ai/soul-intro (blocked; excerpts) + https://kolbo.ai/blog/higgsfield-suite-100-camera-presets
17. AI-generated video failure taxonomy: "A Survey of AI-Generated Video Evaluation" https://arxiv.org/pdf/2410.19884 (excerpt) + https://genra.ai/blog/why-ai-videos-look-fake-how-to-fix , https://www.nemovideo.com/blog/why-ai-videos-look-fake-how-to-fix
