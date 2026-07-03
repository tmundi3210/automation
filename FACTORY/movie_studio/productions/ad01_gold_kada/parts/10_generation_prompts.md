# Part 10 — Generation Prompts & Model Routing (Higgsfield)

### `ad01_gold_kada` · Instagram Reel · 9:16 · 6 clips / 22 s · Classic-Luxurious

**Authored by:** Higgsfield Prompting & Model-Ops specialist.
**Ground truth:** every model id / parameter / duration / aspect-ratio / media-role below is quoted verbatim from the live-enumerated surface `research/higgsfield_surface.md` (30 image / 28 video / 5 audio, `has_more:false`, read **2026-07-03**), **not** from memory of any other tool. Aligned to `parts/00_master_plan.md` (6 shots, durations 4/4/4/3/4/3 s, VO-driven, quiet-luxury, product carried by the owner's photo).
**Account state pinned:** 1,010 credits, `plus` plan. **Per-model credit cost is UNKNOWN on the read surface → every model×resolution×duration cell is `get_cost:true`-preflighted before any spend.**

> **Evidence tags used below:** `[FACT-surface]` = read off the 2026-07-03 MCP enumeration · `[FACT-guide]` = fetched vendor prompt guide (Veo/Sora/Imagen) via the KBs · `[ESTIMATE]` = authored craft applying a sourced pattern · `[UNKNOWN]` = named unknown with its resolution path.

---

## 0. Binding compliance banner (carried from master plan §Compliance)

- **No invented authenticity facts.** Karat/purity, hallmark/BIS, weight, price, brand/city, guarantees are **[OWNER-SLOT]** — never generated, never spoken/shown unless the owner supplies the TRUE value. **No fabricated hallmark, karat stamp, or maker's-mark is ever generated in any macro.**
- **Presenter = fictional AI brand model** showing craftsmanship (aspirational). Never "a verified happy customer," never a real person's likeness. (The likeness lane is closed; nothing here evokes a specific real person.)
- **AI-content disclosure:** publish with Meta's **"AI info" label ON** and an honest, non-embellishing caption.
- **Exact product = owner's uploaded photo (image reference).** The connector cannot read a chat attachment → the owner uploads the kada photo through the **Higgsfield media-upload widget at run time** (`media_upload`/`media_import_url` → `media_confirm`). Flagged in the runbook.
- **Chain = OWNER-SLOT.** No chain photo exists → no chain shot is generated. Do **not** invent a chain design.

---

## 1. Global generation settings (all six shots)

| Setting | Value | Source |
|---|---|---|
| Aspect ratio | **9:16** (fixed BEFORE model — ratio sets silently veto picks) | `[FACT-surface]` |
| Still quality | **4K** where the model exposes it (`resolution:4k` / Seedream `quality:high`) | `[FACT-surface]` |
| Video resolution | **1080p** (Seedance/Kling `4k`/`pro` where budget-probe allows; Seedance 4k/1080p require `mode:std`) | `[FACT-surface]` |
| Native clip audio | **OFF** — reel is muted-autoplay-first + VO-driven. Set `generate_audio:false` (Seedance), `sound:off` (Kling), no `audio_references` (Wan). VO added in post via `seed_audio`/`text2speech_v2`. | `[FACT-surface]` |
| Music | **Not on this surface** — `generate_audio` tool is speech-only; `sonilo_music` is game-pipeline-only. Score = **[OWNER-SLOT: licensed track added in edit].** | `[FACT-surface]` |
| Call envelope | `count` 1–4/call; media `value` = `media_id` or a prior `job_id`, **never a URL**; unsupported durations **silently clamp** → every slot band-checked. | `[FACT-surface]` |
| Cost discipline | `get_cost:true` zero-spend probe on every new model×resolution×duration cell before any `count>1` batch. | `[FACT-surface]` |

**On-screen text (hook line, lower-third owner-slots, CTA card) is NEVER model-generated — it is composited in the editor** inside Reels-safe zones (master plan §4/§7).

---

## 2. (b) CHARACTER-CREATION STEP — standing up the male persona + the product

Two persistence mechanisms exist on the surface; **never mix them in one call** `[FACT-surface]`.

### 2.1 The presenter — trained **Soul** AND a **Reference Element** (build both)

| Lane | What it is | Models it works on | Role in this ad |
|---|---|---|---|
| **Trained Soul** (`show_characters` → train: name + **5–20 images**, ~10 min, non-blocking) | High-fidelity trained identity, bound by `soul_id`, **ONE `soul_id` per generation** | **ONLY `soul_2` and `soul_cinematic`** `[FACT-surface]` | **Identity ANCHOR.** Generates the canonical presenter hero stills (solo, no product macro). Highest face fidelity. |
| **Reference Element** (`show_reference_elements` → instant, no training) | Reusable id embedded as `<<<element_id>>>` inside the prompt text; **multiple tokens per prompt allowed**; **NOT usable on the Soul models** `[FACT-surface]` | Element-compatible **image**: `cinematic_studio_2_5`, `seedream_v4_5`, `seedream_v5_lite`, `nano_banana_2`, `gpt_image_2`; **video**: `seedance_2_0`, `kling3_0` (per surface parallel list) `[FACT-surface]` | **Identity CARRIER for combined shots.** Lets the same face ride on non-Soul models and in video, and coexist in-frame with the product image-reference. |

**Build order:** train the **Soul** first → approve its face/wardrobe on a couple of solo stills → create a **presenter Reference Element `<<<presenter>>>`** from those approved renders so identity is portable to the element-compatible image + video models. `[ESTIMATE — the Element-vs-Soul fidelity gap is UNKNOWN; price it with one early A/B before batching.]`

### 2.2 The product — the owner's kada photo (image reference), optionally an Element

- The **exact bracelet is carried by the owner's uploaded photo as an image reference** (role `image` on `nano_banana_pro`/`cinematic_studio_2_5`; role `image_references` on `seedream_v4_5`/`seedance_2_0`). **Owner uploads at run time** (`media_upload` → `media_confirm`) — flagged in the runbook. `[FACT-surface]`
- **Optional but recommended:** promote that same uploaded photo to a **product Reference Element `<<<kada>>>`** so the exact piece is reusable across every element-compatible shot without re-uploading. `[ESTIMATE]`
- **Never describe the engraving geometry by text alone** — the temple/Greek-key motif and wheat-grain cross-hatch are the exact-fidelity risk; they must come from the photo/Element.

### 2.3 Which shot uses which surface

| Shot | Persona present? | Product in frame? | Identity surface | Product surface |
|---|---|---|---|---|
| 1 Hook (wrist/cuff) | minimal (sleeve/forearm) | yes, hero | none needed (or `<<<presenter>>>` for sleeve continuity) | owner photo image-ref / `<<<kada>>>` |
| 2 Reveal (presenter + kada on wrist) | **yes, face** | yes | **`<<<presenter>>>` Element** (Soul still is the identity source + a solo alternate) | owner photo image-ref |
| 3 Band macro | no | yes, hero macro | none | owner photo image-ref / `<<<kada>>>` |
| 4 Terminal macro | no | yes, hero macro | none | owner photo image-ref / `<<<kada>>>` |
| 5 Authenticity (presenter + kada on stand) | **yes, face** | yes | **`<<<presenter>>>` Element** | owner photo image-ref |
| 6 CTA (product at rest) | minimal/optional | yes, hero | optional `<<<presenter>>>` | owner photo image-ref / `<<<kada>>>` |

> **Why the combined shots (2, 5) use the Element, not the Soul:** a Soul model takes **one** image input and **cannot** carry an Element token, so it can't hold the presenter's `soul_id` **and** the exact-product photo in one generation. The Element route (`<<<presenter>>>` token in the prompt **+** owner product photo as the image reference) is the surface-designated way to put both in one frame. The Soul remains the identity **source** and the alternate for solo-persona framings. `[FACT-surface]`

---

## 3. (a) MODEL PICK PER SHOT — start-image model + video model

**Routing rules applied:** (i) ratio 9:16 fixed first; (ii) duration band-checked against each model's enumerated set (no reliance on silent clamping); (iii) **all six clips route to a both-frames model** so the bracketed seams have real end frames (master plan §4); (iv) product macros favor a model that can *also* hold the product by image-reference during motion. Every pick names a **primary + one alternate** per the owner's "try different, keep what wins."

### 3.1 Start-frame IMAGE models

| Shot | Primary (params) | Alternate | One-line justification |
|---|---|---|---|
| 1, 3, 4, 6 (product-forward, no face) | **`nano_banana_pro`** · `resolution:4k`, `9:16`, owner photo in role `image` | `seedream_v4_5` · `quality:high` (≤~6K), `image_references` | Ultimate-quality photoreal + confirmed 9:16 + 4K macro detail for metal micro-reflections; Seedream alt gives precise product transformation from the reference. `[FACT-surface]` |
| 2, 5 (presenter + product in one frame) | **`cinematic_studio_2_5`** · `resolution:4k`, `9:16`, `<<<presenter>>>` in prompt + owner photo in role `image` | `seedream_v4_5` (element-compatible, ≤~6K) · or **Soul** `soul_cinematic` (`soul_id`, solo, composite product) | Element-compatible + cinematic 4K + confirmed 9:16 → carries face Element and exact-product reference together; Soul alt is the max-fidelity solo-face fallback. `[FACT-surface]` |

*Ratio note:* `seedream_v4_5`'s enumerated set is "8 incl. 21:9" — confirm 9:16 is present at lock time; if not, keep it start-only where 9:16 is verified. `[UNKNOWN → read schema at lock]`

### 3.2 VIDEO models (i2v from the approved start frame; both-frames for bracketed seams)

| Shot | s | Seam (in/out) | Primary video model (params) | Alternate | Justification |
|---|---|---|---|---|---|
| **1 Hook** | 4.0 | cold-open / **whip-pan out** | **`seedance_2_0`** · `mode:std`, `1080p`, `9:16`, `generate_audio:false`, `start_image`+`end_image`(+`image_references`=kada) | `kling3_0` · `mode:pro`, `sound:off`, start+end | Both-frames **and** the only model that also holds the product by `image_references` during the forearm turn; 4 s ∈ band (4–15). `[FACT-surface]` |
| **2 Reveal** | 4.0 | **whip-pan in** / **match-cut on gold** | **`seedance_2_0`** · same, `<<<presenter>>>` in prompt for identity hold, end frame = kada-dominant | `kling3_0` (start+end) | "Reference-driven… consistent identity"; both frames let the end frame hand the gold to the macro. `[FACT-surface]` |
| **3 Band macro** | 4.0 | **match cut** / **rack-focus out** | **`seedance_2_0`** · `mode:std`, `1080p`/`4k`, `image_references`=kada to lock the exact engraving through the travel | `kling3_0` (start+end) | Image-references carry the exact wheat-grain + rails during the lateral move; both frames bracket the seam. `[FACT-surface]` |
| **4 Terminal macro** | 3.0 | **invisible cut** / **whip-pan out** | **`wan2_7`** · `1080p`, `9:16`, `start_image`+`end_image` | `kling3_0` (start+end, 3–15 s) | **3 s is below Seedance's min (4) → Seedance would clamp.** Wan 2.7 (2–15 s, the only native 2–3 s both-frames model) + "character-consistent"; exact motif carried by the start frame. `[FACT-surface]` |
| **5 Authenticity** | 4.0 | **whip-pan in** / **match/clean cut** | **`seedance_2_0`** · `<<<presenter>>>`, both frames, `generate_audio:false` | `kling3_0` (start+end) | Both frames + Element identity for the settle + single approving nod; 4 s ∈ band. `[FACT-surface]` |
| **6 CTA** | 3.0 | **clean cut in** / end (soft open loop) | **`wan2_7`** · `1080p`, `9:16`, start+end | `kling3_0` (start+end, 3 s) | 3 s → same Seedance-clamp reason as shot 4; near-static hold of product at rest, subtle specular glint only. `[FACT-surface]` |

**Deliberately NOT used:** `minimax_hailuo` (great facial emotion, but durations are **6 or 10 s only** — cannot serve any 3–4 s slot) `[FACT-surface]`; `veo3_1` / `veo3` / `kling2_6` / `grok_video*` (start-only → can't bracket a fixed end frame) `[FACT-surface]`; `marketing_studio_video` (min 12 s) `[FACT-surface]`.

---

## 4. (c) EXACT PROMPTS PER SHOT

**Frozen blocks (reuse VERBATIM — paraphrase drift is the main text-side cause of identity/grade drift `[FACT-guide]`):**

- **PRESENTER block** (persona shots only): *"a man in his late 40s, warm mid-brown-to-tan skin with natural skin texture — visible pores, subtle fine lines, realistic uneven tone, slight natural shine — neat salt-and-pepper hair, close groomed stubble, calm quietly-authoritative expression; unbranded well-cut deep-navy jacket over a fine open-collar cream shirt, no logos, no monograms."*
- **PRODUCT block** (scaffold only — EXACT geometry comes from the reference photo, never text): *"a men's high-polish warm yellow-gold kada (open cuff bangle), ~8–10 mm band, fine engraved cross-hatch wheat-grain texture, two bright polished rails top and bottom, openwork temple / Greek-key motif panel near the open terminals — exact form carried by the uploaded reference photo."*
- **PERSONA palette (shots 2, 5):** warm amber, cream, deep navy, slate-blue, gold.
- **PRODUCT palette (shots 1, 3, 4, 6):** warm 22k gold, honeyed amber, espresso-brown shadow, soft cream highlight, near-black negative space.
- **Anti-plastic constants (every prompt):** real optics + directional side light (NOT flat ring light) + visible metal micro-reflections + subtle film grain + gold color accuracy. **Banned words:** flawless, perfect, smooth, airbrushed, beauty, ring light, ethereal glow, max saturation, hyperdetailed, 8K, masterpiece. `[FACT-guide]`

**Grammar notes:** IMAGE prompts use the Imagen/Seedream-class **"A photo of…"** photographic trigger, front-loaded (subject → action → critical style → context → detail), **30–80 words** `[FACT-guide]`. VIDEO prompts follow the **image-to-video motion-only rule** — the start frame already supplies subject/scene, so the clip prompt describes **almost only motion: one camera move + one beat-counted action**; re-describing the still reduces motion `[FACT-guide]`. No Veo timestamps / MiniMax brackets are used (those families aren't routed here). A `Negative:` line is included only for the Kling alternate (Kling supports a dedicated negative field ≤2,500 chars); Seedance/Wan get positive-only phrasing.

---

### SHOT 1 — HOOK · `ad01_s1_hook` · 4.0 s

**START IMAGE** — `nano_banana_pro` (`resolution:4k`, `9:16`; owner kada photo in role `image`):
```
A photo of a man's forearm turned toward the lens in warm low-key light, a high-polish warm yellow-gold kada catching a single diamond-cut flare across the band, tailored unbranded dark sleeve, relaxed hand softly out of frame. Macro 90mm, f/2.8, shallow depth of field, warm key from camera left with a cool edge from the right, visible metal micro-reflections and true gold color, subtle film grain. Palette: warm gold, honeyed amber, espresso-brown, cream, near-black.
```
*(Product = uploaded reference photo, image role; exact engraving from the photo, not the text.)*

**VIDEO** — `seedance_2_0` (`mode:std`, `1080p`, `9:16`, `generate_audio:false`, `start_image`=this still, `end_image`=motion-blurred whip frame, `image_references`=kada photo):
```
Slow forearm rotation toward the lens over a 4-count; the gold band turns and a diamond-cut specular flare travels across it; lens static, then a fast whip-pan blur begins on the final beat. No hands manipulating, fingers stay soft-out of frame. Keep the warm-key / cool-edge lighting and gold accuracy constant.
```
> HARD-SHOT: hands → keep fingers out; on-screen "Real gold. Real craft." is a **composited overlay in edit**, not generated.

---

### SHOT 2 — PRODUCT REVEAL / PERSONA · `ad01_s2_reveal` · 4.0 s

**START IMAGE** — `cinematic_studio_2_5` (`resolution:4k`, `9:16`; `<<<presenter>>>` in prompt + owner kada photo in role `image`):
```
A photo of <<<presenter>>>, chest-up, looking down calmly at a men's warm yellow-gold kada on his wrist and lifting it slightly to the light. [PRESENTER block]. Warm-neutral cinematic grade, soft window key from camera left, faint cool fill from the right, 50mm, f/2.0, natural skin texture, quiet-luxury styling. Palette: warm amber, cream, deep navy, slate-blue, gold. Exact bracelet from the reference photo.
```

**VIDEO** — `seedance_2_0` (both frames; `<<<presenter>>>` for identity hold; `end_image` = kada-dominant frame to hand off to the macro):
```
Slow push-in from chest-up toward the wrist over 4 seconds; the man's gaze settles on the kada, a small calm approving breath, wrist held steady, no fidget; motion ends framed tight on the gold band. Mouth relaxed near-neutral (no dialogue). Hold identity, wardrobe and grade constant.
```
> HARD-SHOT: hands near face/wrist → static resting pose; VO-driven, no lip-sync lock.

---

### SHOT 3 — QUALITY MACRO (BAND) · `ad01_s3_band_macro` · 4.0 s

**START IMAGE** — `nano_banana_pro` (`resolution:4k`, `9:16`; owner kada photo in role `image`):
```
A photo of an extreme macro of a men's warm yellow-gold kada band filling the frame, fine engraved cross-hatch wheat-grain texture throwing a shimmering diamond-cut sparkle, two bright polished rails top and bottom. Macro 100mm, f/4, precise focusing, controlled directional side light with one specular roll, visible metal micro-reflections, accurate warm gold, subtle grain, no hands, no stamps or hallmarks. Palette: warm gold, honeyed amber, espresso-brown, cream, near-black.
```

**VIDEO** — `seedance_2_0` (`mode:std`, `1080p`/`4k`, both frames, `image_references`=kada photo to lock the engraving):
```
Slow lateral tracking along the band over 4 seconds, a single specular highlight rolling across the wheat-grain cross-hatch and the two polished rails; shallow depth of field; ends continuing toward the terminal for a rack-focus hand-off. No hands, no text, no stamps. Keep the exact engraving, rails and gold color constant.
```
> HARD-SHOT: **no fabricated hallmark / karat stamp** — plain textured band only.

---

### SHOT 4 — QUALITY MACRO (TERMINALS) · `ad01_s4_terminal_macro` · 3.0 s

**START IMAGE** — `nano_banana_pro` (`resolution:4k`, `9:16`; owner kada photo in role `image`):
```
A photo of an extreme macro of the terminal openwork temple / Greek-key motif panel of a men's warm yellow-gold kada, the open cuff end reading clearly, deep warm negative space behind. Macro 100mm, f/4, precise focusing, controlled directional light, visible metal micro-reflections, accurate gold, subtle grain, motif crisp and symmetric, no hands. Palette: warm gold, honeyed amber, espresso-brown, cream, near-black.
```
*(Fine geometric motif is the top garble risk → carried by the reference photo; worst-artifact gate on the still — any garbled/asymmetric motif = re-roll, not ship.)*

**VIDEO** — `wan2_7` (`1080p`, `9:16`, `start_image`=this still, `end_image`=whip-blur frame):
```
Rack focus pulling from soft foreground onto the terminal openwork motif over 3 seconds, the geometric panel resolving to sharp; a whip-pan blur begins on the last beat to hand back to the presenter. No hands, no text. Hold the motif geometry and gold color exactly.
```
> **3 s → `wan2_7`, not Seedance** (Seedance min 4 would clamp). Alt `kling3_0` (native 3 s, start+end).

---

### SHOT 5 — AUTHENTICITY · `ad01_s5_authenticity` · 4.0 s

**START IMAGE** — `cinematic_studio_2_5` (`resolution:4k`, `9:16`; `<<<presenter>>>` + owner kada photo in role `image`):
```
A photo of <<<presenter>>>, medium shot, standing calmly beside a men's warm yellow-gold kada resting on a soft-lit honed-marble stand. [PRESENTER block]. Warm cinematic grade, soft key from camera left, cool rim from behind, 50mm, f/2.2, natural skin texture, generous quiet negative space in the lower third for a composited caption. Palette: warm amber, cream, deep navy, slate-blue, gold. Exact bracelet from the reference photo. No on-image text.
```

**VIDEO** — `seedance_2_0` (both frames, `<<<presenter>>>`, `generate_audio:false`):
```
Slow push-in and settle over 4 seconds; the man gives a single calm approving nod toward the kada on the stand, then holds still; wrist and hands relaxed. No dialogue lip-sync. Keep identity, wardrobe, grade and the empty lower-third constant for the overlay.
```
> HARD-SHOT: authenticity implication → the **[OWNER-SLOT]** purity/hallmark lower-third is **composited only with TRUE values**; if the owner has no hallmark, the text omits it. The clip generates **no** stamp or certificate.

---

### SHOT 6 — CTA · `ad01_s6_cta` · 3.0 s

**START IMAGE** — `nano_banana_pro` (`resolution:4k`, `9:16`; owner kada photo in role `image`):
```
A photo of a men's warm yellow-gold kada at rest on a soft-lit honed-marble surface, clean and premium, generous quiet negative space around it for a composited CTA card, kept clear of the bottom Reels UI band and top-right corner. Macro 85mm, f/4, controlled directional light, visible metal micro-reflections, accurate gold, subtle grain. Palette: warm gold, honeyed amber, espresso-brown, cream, near-black. No on-image text.
```

**VIDEO** — `wan2_7` (`1080p`, `9:16`, start+end near-identical for a near-static hold):
```
Near-static hold for 3 seconds, a slow subtle specular glint drifting across the gold band, the piece perfectly still on the surface; no camera move beyond a faint settle. Keep the negative space and gold color constant.
```
> HARD-SHOT: **all CTA text + handle are composited graphics in the editor**, never generated. `[OWNER-SLOT: @handle]`, optional `[OWNER-SLOT: purchase link URL]`.

---

## 5. (d) GENERATION RUNBOOK — ordered Higgsfield calls (once the connector is live)

> Legend: **[OWNER]** = a step the owner must do (upload). **[PROBE]** = `get_cost:true` first, then spend. All media `value`s are `media_id` (from `media_confirm`) or a prior `job_id` — never a URL.

**Phase A — assets & personas**
1. **[OWNER] Upload the kada photo** via the media-upload widget → `media_upload`/`media_import_url` → `media_confirm` → capture `media_id`. *(Blocking for all six shots.)*
2. *(Chain: none. No chain photo → chain shots stay OWNER-SLOT and are not generated.)*
3. **Train the presenter Soul** — `show_characters` train, name + 5–20 consistent reference images (~10 min, non-blocking) → capture `soul_id`. **[PROBE training cost — UNKNOWN.]**
4. **[PROBE]** Generate 1–2 solo presenter hero stills on `soul_cinematic` (`soul_id`, 9:16, quality 2k) to approve face/wardrobe.
5. **Create the presenter Reference Element `<<<presenter>>>`** from the approved renders (`show_reference_elements` create). *(Optional: also create product Element `<<<kada>>>` from step 1's photo.)*
6. **Early A/B [PROBE]:** one combined still via Soul-then-composite vs the Element route, to price the Element-vs-Soul fidelity gap `[UNKNOWN]` before batching.

**Phase B — per-shot START IMAGES** (`generate_image`, 9:16, 4K, `count` 2–4 per shot; `[PROBE]` each new model×resolution cell once)
7. Shot 1 → `nano_banana_pro` + kada `media_id` (role `image`).
8. Shot 2 → `cinematic_studio_2_5`, `<<<presenter>>>` in prompt + kada `media_id`.
9. Shot 3 → `nano_banana_pro` + kada `media_id`.
10. Shot 4 → `nano_banana_pro` + kada `media_id`. **Worst-artifact gate on the motif** — garbled/asymmetric = re-roll.
11. Shot 5 → `cinematic_studio_2_5`, `<<<presenter>>>` + kada `media_id`.
12. Shot 6 → `nano_banana_pro` + kada `media_id`.
    → Curate one winning start frame per shot; where a seam needs a distinct **end frame**, generate/curate it now (e.g. shot-2 kada-dominant end; shot-1/4 whip-blur end) matched in palette/lens/grade.

**Phase C — per-shot VIDEO** (`generate_video`, i2v motion-only, both-frames, `generate_audio:false`/`sound:off`; `[PROBE]` each model×resolution×duration cell)
13. Shot 1 → `seedance_2_0` (4 s, `mode:std`, 1080p) · start+end+`image_references`=kada.
14. Shot 2 → `seedance_2_0` (4 s) · start+end, `<<<presenter>>>`.
15. Shot 3 → `seedance_2_0` (4 s, `mode:std`) · start+end+`image_references`=kada.
16. Shot 4 → `wan2_7` (3 s, 1080p) · start+end. *(Not Seedance — 3 s would clamp.)*
17. Shot 5 → `seedance_2_0` (4 s) · start+end, `<<<presenter>>>`.
18. Shot 6 → `wan2_7` (3 s, 1080p) · start+end.
    → Review each end frame before it seeds the next link (frame chaining via `job_id`); re-roll any clip failing the realism/worst-artifact gate. **Escalate to the realism reviewer after two failed one-axis rounds** (change exactly one axis per round, N≥2 samples).

**Phase D — finish**
19. **End-frame chaining check:** confirm each bracketed seam (whip-pan / match-cut) reads as designed; regenerate a mismatched link if needed.
20. **Upscale:** stills → `topaz_image` / `bytedance_image_upscale`; clips → `topaz_video` / `bytedance_video_upscale`; run `video_deflicker` on any AI-flicker.
21. **VO:** `seed_audio` or `text2speech_v2` for the master-plan lines — **only the [OWNER-SLOT] claims the owner has filled with TRUE values**; empty slot → line cut.
22. **Edit (outside Higgsfield):** stitch 6 clips, composite the hook line / lower-third owner-slots / CTA card in Reels-safe zones, add **[OWNER-SLOT: licensed music]**, export **watermark-free**.
23. **Publish:** Meta **"AI info" label ON** + honest caption; no authenticity claim beyond owner-verified slots.

---

## 6. Owner-slots & unknowns to clear before spend

**Blocking owner inputs:** kada photo upload (step 1); true values for purity/hallmark/weight/price/brand/city/guarantee (only the filled ones are spoken/shown); `@handle` + optional purchase URL; licensed music track; (chain photo only if a chain shot is ever wanted).

**Named UNKNOWNs carried (resolution path):** per-model credit cost → `get_cost:true` before each cell; Soul training cost → probe; Element-vs-Soul fidelity gap → early A/B (step 6); `seedream_v4_5` 9:16 presence → read schema at lock; `nano_banana_*` Element-id routing collision → prefer explicitly element-listed `cinematic_studio_2_5`/`seedream_v4_5`/`nano_banana_2` for Element shots; seed exposure per model → read live schema at run. Re-enumerate the catalog if any call contradicts this 2026-07-03 snapshot.
