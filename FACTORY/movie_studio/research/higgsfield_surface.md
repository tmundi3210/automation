# Higgsfield MCP Surface — Live Enumeration (Platform Ground Truth)

Scope: read-only enumeration of the connected Higgsfield MCP (models, presets, workflows, character/element surfaces, audio reality, credits) as of 2026-07-03; primary source for every platform-capability claim in the movie_studio pipeline.

**Provenance.** Everything below tagged [FACT] with no URL was read directly off the live MCP surface on 2026-07-03 via: `models_explore action:list` for `type:image` (30 models, `has_more:false`), `type:video` (28 models, `has_more:false`), `type:audio` (5 models, `has_more:false`); `models_explore action:get` on 15 priority models (get returns the identical schema as list — no additional hidden constraints surfaced); `presets_show` (49 presets); `get_workflow_instructions` with no argument (catalog mode); `show_characters action:list`; `show_reference_elements action:list`; `balance`; plus the loaded JSON schemas of `generate_image`, `generate_video`, `generate_audio`, `motion_control`, `show_characters`, `show_reference_elements`. Zero generate/create/upload calls were made; zero credits spent. Web cross-checks cited inline where used.

**Account state.** [FACT] Credits: 1,010. Subscription plan: `plus`. (live MCP surface, `balance`, 2026-07-03)

---

## (a) Full model catalog

### a.1 Image models (30 — complete list, `has_more:false`) [FACT, live MCP surface 2026-07-03]

| id | name | provider | description (verbatim) | key params (options; default) | media inputs | aspect ratios |
|---|---|---|---|---|---|---|
| `nano_banana_2` | Nano Banana 2 | Google | Fast, next-gen high-quality images | resolution (1k/2k/4k; 1k) | image refs (role `image`) | 1:1,3:2,2:3,4:3,3:4,4:5,5:4,9:16,16:9,21:9 |
| `nano_banana_pro` | Nano Banana Pro | Google | Ultimate quality, text and diagrams | resolution (1k/2k/4k; 1k) | image refs (role `image`) | same 10 as above |
| `nano_banana_2_lite` | Nano Banana 2 Lite | Google | Lite next-gen high-quality images | resolution (1k only); thinking (MINIMAL/HIGH; HIGH) | image_references | auto + same 10 |
| `nano_banana` | Nano Banana | Google | Realistic images, budget-friendly | — | image_references | same 10 |
| `nano_banana_2_shots` | Nano Banana Pro (sic) | (blank) | (blank) | — | image_references | auto + same 10 |
| `soul_2` | Higgsfield Soul 2.0 | Higgsfield | Realistic UGC, fashion editorial and character generation | quality (1.5k/2k; 2k); **soul_id** (optional) | image ×1 max (role `image`) | 1:1,16:9,9:16,4:3,3:4,3:2,2:3 |
| `soul_v2` | Higgsfield Soul 2.0 | Higgsfield | duplicate id of `soul_2`, identical schema | same | same | same |
| `soul_cinematic` | Soul Cinema | Higgsfield | Cinema-grade stills and concept art | quality (1.5k/2k; 2k); **soul_id** (optional) | image ×1 max | soul_2 set + 21:9 |
| `soul_cast` | Soul Cast | Higgsfield | Consistent cinematic character identity | budget (number 10–500; 50) | none (text-only) | 16:9 only |
| `soul_location` | Soul Location | Higgsfield | Environment and location generation | — | none | 1:1,4:3,3:4,16:9,9:16,3:2,2:3,21:9,9:21 |
| `cinematic_studio_2_5` | Cinema Studio Image 2.5 | Higgsfield | Cinematic stills, up to 4K resolution | resolution (1k/2k/4k; 1k) | image refs (role `image`) | 10 incl. 21:9 |
| `seedream_v4_5` | Seedream 4.5 | Bytedance | 4K output, precise control, transformations | quality (basic=up to 4K / high=up to ~6K; basic) | image_references | 8 incl. 21:9 |
| `seedream_v5_lite` | Seedream 5.0 lite | Bytedance | Visual reasoning, instruction-based editing | quality (basic/high; basic) | image refs (role `image`) | 1:1,16:9,9:16,4:3,3:4 |
| `gpt_image_2` | GPT Image 2 | OpenAI | Next-gen GPT Image, 1k/2k/4k + quality tiers | resolution (1k/2k/4k; 1k); quality (low/med/high; low) | image refs | 7 ratios |
| `openai_hazel` | OpenAI Hazel | OpenAI | Powerful editing, best text rendering | quality (low/med/high; medium) | image_references | 1:1,3:2,2:3,auto |
| `flux_2` | FLUX.2 | Black Forest Labs | Variants (pro/flex/max), precise prompt adherence | resolution (1k/2k; 1k); variant (pro/flex/max; pro) | image_references | 1:1,4:3,3:4,16:9,9:16 |
| `flux_kontext` | Flux Kontext | Black Forest Labs | Context-aware editing and style transfer | — | image_references | 1:1,4:3,3:4,16:9,9:16 |
| `kling_omni_image` | Kling O1 Image | Kling | Versatile photorealistic generation | resolution (1k/2k; 1k) | image_references | 9 incl. auto, 21:9 |
| `grok_image` | Grok Image | xAI | Expressive, high-contrast generation/editing | resolution (1k/2k; 1k); mode (std/quality; std) | image_references | 10 incl. 1:2, 2:1 |
| `recraft_v4_1` | Recraft V4.1 | Recraft | Vector/utility/product variants | resolution (1k/2k); model_type (standard/vector/utility/utility_vector); colors[] hex; background_color | none | 9 ratios |
| `z_image` | Z Image | Tongyi-MAI | Super fast, stylized text-to-image | — | none | 1:1,4:3,3:4,16:9,9:16 |
| `image_auto` | Auto | Higgsfield | Auto-selects the best image model | — | image refs | 1:1,4:3,3:4,16:9,9:16 |
| `marketing_studio_image` | Marketing Studio Image | Higgsfield | One-click product image ads | resolution (1k/2k/4k; 1k) | image refs | auto + 10 |
| `ms_image` | DTC Ads | Higgsfield | Brand-kit-aware ad images | **style_id REQUIRED** (via show_marketing_studio); brand_kit_id; resolution; quality; batch_size 1–20; product_ids ≤4 | image ×14 max | 15 incl. 27:16, 9:8 |
| `autosprite` | AutoSprite Animation | Higgsfield | Character image → game sprite sheet | kind (idle/walk/run/… 26 presets); video_tier (turbo/pro/max); frame_count 2–64; frame_size 32–512; remove_bg; with_sound; is_humanoid | image ×1 required | — |
| `image_background_remover` | Image Background Remover | — | utility | — | image_references | — |
| `outpaint` | Outpaint | — | utility (expand/uncrop) | folder_id | image_references | auto + 10 |
| `topaz_image` | Topaz | — | upscale/enhance | output_width/height required; face_enhancement + strength/creativity 0–1; variant (Standard V2/Low Res V2/CGI/High Fidelity V2/Text Refine); sharpen/denoise 0–1 | image_references | — |
| `topaz_image_generative` | Topaz (generative) | — | generative upscale | variant (Standard MAX/Redefine/Recovery/Recovery V2; Redefine); creativity 1–6; texture 1–5; autoprompt; face_enhancement | image_references | — |
| `bytedance_image_upscale` | Bytedance Image Upscale | — | upscale | resolution (2k/4k; 4k) | image_references | — |

### a.2 Video models (28 — complete list, `has_more:false`) [FACT, live MCP surface 2026-07-03]

| id | name | provider | description (verbatim) | duration | key params | media roles | aspect ratios |
|---|---|---|---|---|---|---|---|
| `cinematic_studio_3_0` | Cinema Studio Video 3.0 | Higgsfield | Most advanced cinema-grade model | 4–15 s range | resolution (480p/720p/1080p/4k; 720p); genre (auto/action/horror/comedy/noir/drama/epic; auto); generate_audio (bool; false) | `image`, `start_image`, `end_image` | auto,21:9,16:9,4:3,1:1,3:4,9:16 |
| `cinematic_studio_video_v2` | Cinema Studio Video | Higgsfield | Refined cinematic camera and color, genre control | 3–12 s range | genre (auto/action/horror/comedy/western/suspense/intimate/spectacle; auto); mode (pro/std; std); sound (on/off; on); **speedramp** (auto/custom/linear/slowmo/speedup/impact; auto); multi_shots (bool); multi_shot_mode (auto/custom); cfg_scale 0–1 (0.5); preset_id | `image`, `start_image`, `end_image` | 1:1,4:3,3:4,16:9,9:16 |
| `cinematic_studio_video` | Cinema Studio Video (v1) | Higgsfield | Solid cinematic, dramatic compositions | 5 or 10 s | slow_motion (bool); sound (bool; true) | `image`, `start_image`, `end_image` | 1:1,4:3,3:4,16:9,9:16 |
| `seedance_2_0` | Seedance 2.0 | Bytedance | Reference-driven video… consistent identity, multi-SKU | 4–15 s (5) | resolution (480p/720p/1080p/4k; 720p; 4k/1080p need mode=std); mode (std/fast); bitrate_mode (standard/high); genre (auto…epic); generate_audio (bool; true) | `start_image`, `end_image`, `image_references`, `video_references`, `audio_references` | auto,16:9,9:16,4:3,3:4,1:1,21:9 |
| `seedance_2_0_mini` | Seedance 2.0 Mini | Bytedance | Fast budget Seedance 2.0 variant | 4–15 s (5) | resolution (480p/720p only); bitrate_mode; genre; generate_audio (true) | same 5 roles as seedance_2_0 | same |
| `seedance1_5` | Seedance 1.5 Pro | Bytedance | Reliable motion, improved quality | 4/8/12 s (4) | resolution (480p/720p/1080p; 720p); generate_audio (true) | `start_image`, `end_image` | auto,16:9,9:16,4:3,3:4,1:1,21:9 |
| `kling3_0` | Kling v3.0 | Kling | Multi-shot, audio sync, motion transfer | 3–15 s (5) | mode (std/pro/4k; std); sound (on/off; on) | `start_image`, `end_image` | 16:9,9:16,1:1 |
| `kling3_0_turbo` | Kling 3.0 Turbo | Kling | Fast text-to-video, single start-frame animation | 3–15 s (5) | resolution (720p/1080p; 720p) | `start_image` only | 16:9,9:16,1:1 |
| `kling2_6` | Kling 2.6 Video | Kling | Cinematic motion, advanced physics | 5 or 10 s | sound (bool; true) | `start_image` only | 16:9,9:16,1:1 |
| `minimax_hailuo` | Minimax Hailuo | Hailuo | Natural physics, facial emotion, multiple variants | 6 or 10 s | variant (minimax/minimax-fast/minimax-2.3/minimax-2.3-fast; minimax-2.3); resolution (512/768/1080; 768; **512 incompatible with end_image and unsupported by 2.3 variants**) | `start_image`, `end_image` | (none listed) |
| `veo3_1` | Google Veo 3.1 | Google | Ultra-realistic, top-tier cinematic quality | 4/6/8 s (8) | quality (basic/high/ultra; basic); variant (veo-3-1-preview best / veo-3-1-fast; fast) | `start_image` only | 16:9,9:16 |
| `veo3_1_lite` | Google Veo 3.1 Lite | Google | Fast, affordable, budget batch clips | 4/6/8 s (8) | generate_audio (bool; **false**, costs extra) | `start_image`, `end_image` | 16:9,9:16,auto |
| `veo3` | Google Veo 3 | Google | Reliable cinematic, broad creative range | — | variant (veo-3-preview/veo-3-fast; fast) | `start_image` only | 16:9,9:16 |
| `wan2_7` | Wan 2.7 | Wan | Synchronized audio, character-consistent video | 2–15 s (5) | resolution (720p/1080p; 720p) | `start_image`, `end_image`, `audio_references` | 16:9,9:16,1:1,4:3,3:4 |
| `wan2_6` | Wan 2.6 Video | Wan | Open-weight, stylized, experimental creative | 5/10/15 s (5) | quality (720p/1080p; 720p) | `image_references`, `video_references`, `audio_references` | 16:9,9:16,1:1 |
| `grok_video_v15` | Grok Video 1.5 | xAI | Image-to-video preview, cinematic, native audio direction | 2–15 s (5) | resolution (480p/720p; 720p) | `start_image` only | (none listed) |
| `grok_video` | Grok Video | xAI | Text and image-to-video, audio support | 1–15 s (5) | — | `start_image` only | 16:9,9:16,1:1 |
| `gemini_omni` | Gemini Omni Flash | Google | Reference-driven video with native audio | 4–10 s (8) | resolution (720p only) | `image_references`, `video_references` | 16:9,9:16 |
| `higgsfield_preset` | Higgsfield Preset | Higgsfield | Preset-routed image-to-video using presets_show | — | **preset_id REQUIRED** | image ×1 required | 16:9,9:16,1:1 |
| `marketing_studio_video` | Marketing Studio | Higgsfield | One-click product ads, TikTok/Reels ready | 12–15 s | resolution (480p/720p/1080p; 720p); generate_audio (true); mode (preset slug); avatar_ids ≤1; product_ids; hook_id; setting_id; ad_reference_id (mutually exclusive with hook/setting) | avatars; `image`,`start_image`,`end_image` | auto,21:9,16:9,4:3,1:1,3:4,9:16 |
| `clipify` | Personal Clipper | Higgsfield | YouTube video → subtitled clips | segment 2–60 s | urls (exactly 1 YouTube URL); clips_num 1–20 (10); clip_aspect (9:16/1:1/16:9); subtitle font/case/position/highlight; track_face_crop | none | — |
| `explainer_video` | Explainer Video | — | assembles narrated blocks | — | width/height required; items[] required; subtitles optional | none | — |
| `topaz_video` | Topaz | — | video enhance/upscale | — | resolution (1080p/2160p); frame_interpolation; frame_rate (30) | `input_video` | auto + 6 |
| `bytedance_video_upscale` | Bytedance Video Upscale | — | upscale | — | fps 24–60 (24); resolution (1080p/2k/4k; 2k); preset (common/aigc/short_series/ugc/old_film); model_version (standard/pro) | `video_references` | — |
| `video_upscale` | Video Upscale | — | utility | — | duration; folder_id | `input_video` | — |
| `video_deflicker` | Video Deflicker | — | utility (AI-flicker fix — relevant to "not look AI" QC) | — | duration; folder_id | `input_video` | — |
| `video_background_remover` / `sam_3_video` | Remove Background | — | segmentation utilities | — | apply_mask; frames_count | `video_references` | — |
| `llm_text` | LLM Generation | — | text/LLM utility surfaced under video | — | model required; user/system_prompt; reasoning_effort | `input_images` | — |

### a.3 Audio models (5 — complete list, `has_more:false`) [FACT, live MCP surface 2026-07-03]

| id | name | provider | what it is | key params |
|---|---|---|---|---|
| `seed_audio` | Seed Audio 1.0 | ByteDance | Text-to-speech / text-to-audio synthesis; optional voice clone from audio reference or image cue | format (wav/mp3/pcm/ogg_opus; wav); sample_rate (8k–48k; 24000); speech_rate −50…100; loudness_rate −50…100; pitch_rate −12…12; voice_type (preset/element) + voice_id |
| `text2speech_v2` | Text to Speech V2 | Higgsfield | TTS with selectable engine | variant REQUIRED (elevenlabs/minimax/seed_speech/vibe_voice/cozy_voice); voice_type + voice_id REQUIRED |
| `inworld_text_to_speech` | Inworld TTS | FAL | TTS, **"Game pipeline only"** | voice required (≈110 named voices, en/zh/nl/fr/de/it/ja/ko/pl/pt/es/ru/hi/he/ar) |
| `sonilo_music` | Sonilo Music | FAL | Text-to-music, **"Game pipeline only"** | duration (s) required |
| `mirelo_text_to_audio` | Mirelo Text to Audio | FAL | Text-to-SFX, **"Game pipeline only"** | duration (s) required |

---

## (b) Start-image PLUS end-image video conditioning (owner's explicit first/last-frame workflow)

All tags [FACT], read from each model's declared `medias[].roles` on the live MCP surface, 2026-07-03.

**Support BOTH `start_image` AND `end_image`:**
- `cinematic_studio_3_0` (4–15 s, up to 4K, genre control) [FACT]
- `cinematic_studio_video_v2` (3–12 s, speedramp + multi-shot + cfg_scale) [FACT]
- `cinematic_studio_video` (v1; 5/10 s) [FACT]
- `seedance_2_0` (4–15 s, up to 4K, plus image/video/audio references simultaneously) [FACT]
- `seedance_2_0_mini` (4–15 s, ≤720p budget) [FACT]
- `seedance1_5` (4/8/12 s) [FACT]
- `kling3_0` (3–15 s, std/pro/4k) [FACT]
- `minimax_hailuo` (6/10 s; constraint: resolution 512 incompatible with end_image) [FACT]
- `veo3_1_lite` (4/6/8 s; audio off by default) [FACT]
- `wan2_7` (2–15 s; also takes audio_references) [FACT]
- `marketing_studio_video` (ad pipeline, 12–15 s) [FACT]

**Start-image ONLY (no end-frame):** `kling2_6`, `kling3_0_turbo`, `veo3`, `veo3_1`, `grok_video`, `grok_video_v15` [FACT]. **Neither (reference-driven instead):** `gemini_omni`, `wan2_6` [FACT]. **Single `image` only:** `higgsfield_preset` [FACT].

Cross-check: Higgsfield's own blog documents the start/end-frame workflow as a first-class storytelling pattern ("upload specific Start and End frames and Cinema Studio will handle the tweens"; Kling start & end frames guides) [FACT — higgsfield.ai blog, sources 2–4]. The exact per-model claim set above comes from the MCP surface, which is authoritative for what this account can call.

Practical note for the 3–6 s stitched-reel format: every both-frames model above accepts durations in the 3–6 s band except `seedance1_5` (min 4), `minimax_hailuo` (min 6), `marketing_studio_video` (min 12) [FACT, duration fields above]. Chaining clip N's end_image as clip N+1's start_image is structurally supported by `generate_video.medias[].value` accepting a prior generation's `job_id` [FACT, generate_video schema].

## (c) Character-consistency surfaces → two-persona (1 M + 1 F) mapping

Two distinct persistence mechanisms exist [FACT, live tool schemas 2026-07-03]:

1. **Soul Characters** (`show_characters`): reusable *trained* identity models. Train needs `name` + 5–20 reference images, takes ~10 min, non-blocking. Types: `soul` (legacy), `soul_2`, `soul_cinematic`. Constraints printed on the surface: a trained Soul is usable ONLY with `text2image_soul_v2` (= model `soul_2`) and `soul_cinema_studio` (= model `soul_cinematic`); **ONE soul_id per generation** — a two-persona shot cannot use two Souls in one image. Training inputs: media_id UUIDs from media_confirm, completed image-job IDs, or https URLs. Current workspace state: `{"items":[],"next_cursor":null}` — no characters exist yet [FACT].
2. **Reference Elements** (`show_reference_elements`): reusable characters/environments/props per workspace. Create is instant (single or few images, no training). Usage: embed `<<<element_id>>>` placeholders inside the `prompt` of `generate_image`/`generate_video`; multiple placeholders per prompt are allowed → this is the surface-designated path for shots containing BOTH personas [FACT, tool description]. Supported models named on the surface — image: `nano_banana_2`, `nano_banana_flash`, `gpt_image_2`, `seedream_v4_5`, `seedream_v5_lite`, `cinematic_studio_2_5`; video (per the show_characters description's parallel list): Cinema Studio Video 2 / 3.0, Seedance 2.0, Kling 3.0 [FACT, tool descriptions; note the video list in show_reference_elements itself was truncated in transmission — [UNKNOWN] whether more video models are element-compatible]. Elements are NOT usable with Soul V2 / Soul Cinema [FACT]. Current workspace state: empty list [FACT].

Additional consistency levers on the surface [FACT]: `soul_cast` (text-only "consistent cinematic character identity" image model, 16:9, budget param 10–500) — a persona-sheet generator; `seedance_2_0`/`seedance_2_0_mini` accept `image_references` for identity-consistent video ("consistent identity" in its own description); `wan2_7` self-describes "character-consistent video"; `generate_video` guidance names `seedance_2_0` as the default "for identity".

**Two-persona program mapping [ESTIMATE, basis: the constraints above; medium-high confidence]:** train one Soul per persona (male, female) for solo hero stills via `soul_2`/`soul_cinematic`; ALSO create one Element per persona (plus wardrobe/prop/location Elements — the category enum is `character|environment|prop`) so that (i) two-person shots and (ii) non-Soul models (Nano Banana Pro, Seedream 4.5, Cinema Studio, Seedance 2.0, Kling 3.0 video) can hold identity. Pre-built wardrobe = `prop`/`character` Elements combined via multiple `<<<element_id>>>` placeholders.

## (d) Motion-control and camera-movement surfaces (owner's "camera tricks")

- **`motion_control` tool** [FACT, schema]: Kling 3.0 Motion Control — animates a character image with the motion AND camera movement of a reference video ("recast, puppeteer, motion transfer"). Params: `image_id` + `motion_video_id` (confirmed media_id or job_id), `resolution` 720p/1080p, `scene_control` image|video (background source). No prompt used. This is the strongest camera-trick replication lever on the surface: film/record a reference camera move once, reuse it on generated personas.
- **`cinematic_studio_video_v2` params** [FACT]: `speedramp` (auto/custom/linear/slowmo/speedup/impact) = native speed-ramp transitions; `multi_shots` + `multi_shot_mode` (auto/custom multi_prompt) = in-model shot splitting; `cfg_scale` 0–1; `genre` including suspense/intimate/spectacle. `cinematic_studio_3_0` and Seedance add `genre` (action/horror/comedy/noir/drama/epic).
- **`higgsfield_preset` + `presets_show`** [FACT]: 49 presets returned; they are choreographed image-to-video templates (id + name + description + preview mp4), i.e. packaged camera+scenario moves, NOT a bare camera-move list. Luxury/influencer-relevant presets (verbatim names): **2000'S PAPARAZZI** (Y2K celebrity exits luxury hotel past flashing cameras), **CANDID PAPARAZZI** (airport paparazzi), **RED CARPET** (live broadcast, handheld, poses for photographers), **RACE WINNER** (cinematic night race montage, slow-motion helmet-off), **DRIFT RACING** (Tokyo night, low angles, 35mm grain), **RACE TRACK** (selfie walk, cars blasting past, camera shake), **SUMMER HAZE** (lomo home-movie, light leaks, 6 hazy shots), **TUSCAN YOGA** (sunlit Tuscan landscape — fitness accent), **ORBITAL PRESENCE** (orbital wides → intimate close-ups), **BLUE DEPTH**, **ENDING FAIRY**, **FAN MEETING**, **BASEBALL GAME**, **FINAL SERVE**, **GOLF MAJOR**. Camera-trick-shaped presets: **Earth zoom in** / **Earth zoom out**, **CGI BREAKDOWN**, **3D RENDER** (orbit + fast zooms), **FOOTBALL INVADER** (one continuous telephoto take), **Still world**, **Superfast flight**, plus ~14 game/fantasy/horror-styled ones (NIGHTLINE, FREE FALL, NEON CITY, SOUL FIGHTER, IN THE DARK, RED THREAD, EXIT THE DREAM, DRAGON FANTASY, APEX HUNTER, ZOMBIE DANCE, KUNG FU HIT, STORM GIANT, ANDROID ASSEMBLE, OFFICE CCTV, NIGHT VISION, DROWN IN MUSIC) and 13 "superhero-gen" presets carrying only name+preview (Casual Monster Slayer, Wrestle, Magic Spell, Animal chase, Arena Zero, Disintegration, Sword and Sorcery, Face Punch, Animal ride, Me and pet transformation, Wrestle…) [FACT — names/descriptions verbatim from presets_show].
- **No standalone "camera move" parameter** (dolly/pan/orbit as enum) exists on any generate_video model schema — camera language must be carried in the prompt text, via presets, via cinematic_studio genre/speedramp, or via motion_control reference video [FACT — absence read across all 28 video model schemas]. Web cross-check: Higgsfield markets Cinema Studio camera/lens/motion control as prompt-and-UI-level workflow, consistent with this [FACT — sources 2, 5].
- Adjacent editing utilities relevant to stitching: `reframe` (change video aspect), `upscale_video`, `video_deflicker`, `remove_background`/`sam_3_video` (mask), `topaz_video` (frame interpolation param) [FACT — tool list + schemas].

## (e) Audio / music generation for scene-mood scoring — CRITICAL LIMITATION

- [FACT, generate_audio tool description, live surface 2026-07-03]: "This tool only generates speech: it cannot generate music or sound effects for general use, and **there is no standalone music/SFX model here — decline general music or sound-effect requests** rather than substituting a speech model. The models sonilo_music (music), mirelo_text_to_audio (sound effects) and inworld_text_to_speech (voice) exist ONLY for the game-generation pipeline and must not be used for standalone audio."
- Consequence for the owner's Hans-Zimmer-grade scoring ask: on THIS surface, scene-mood music can only arrive as (i) **native audio inside video generations** — models with audio flags: `cinematic_studio_3_0` (generate_audio, default false), `cinematic_studio_video_v2`/`cinematic_studio_video` (sound on/off), `seedance_2_0`/`mini`/`seedance1_5` (generate_audio, default true), `kling3_0`/`kling2_6` (sound), `veo3_1_lite` (generate_audio, default false, extra cost), `wan2_7`/`wan2_6` (audio_references + synchronized audio), `grok_video_v15` ("native audio direction"), `gemini_omni` (native audio), `veo3`/`veo3_1` (tag "audio") [FACT per schema]; or (ii) **audio_references steering** — `seedance_2_0`, `seedance_2_0_mini`, `wan2_7`, `wan2_6` accept an `audio_references` media role, i.e. an externally sourced/licensed music track can condition the generation [FACT]; or (iii) external licensed music added in post, outside this MCP [ESTIMATE — inference from (i)+(ii), high confidence].
- Speech/voice is fully served: `seed_audio` (TTS + voice clone from audio reference; rate/pitch/loudness controls), `text2speech_v2` (engine choice: elevenlabs/minimax/seed_speech/vibe_voice/cozy_voice), plus `create_voice`, `list_voices`, `voice_change`, `dubbing` tools on the server [FACT — tool list; schemas of the latter not loaded].
- "3D sound"/spatial audio: no parameter anywhere on the surface mentions spatial/binaural/3D audio [FACT — absence across all schemas read]. [UNKNOWN] whether any native-audio video model renders spatially convincing sound; only testable in the run phase.

## (f) Parameter cheat-sheet — priority models

All [FACT, live MCP surface 2026-07-03]:

- **`nano_banana_pro` / `nano_banana_2`** (photoreal stills, 4K): resolution 1k|2k|4k (1k). Image-to-image via role `image`. 10 aspect ratios incl. 21:9 and 4:5 (IG feed) and 9:16 (reel cover).
- **`soul_2`** (persona hero stills, UGC/fashion): quality 1.5k|2k (2k); soul_id ← trained Soul; max ONE reference image; no 21:9.
- **`soul_cinematic`** (cinema stills/concept): quality 1.5k|2k (2k); soul_id; adds 21:9.
- **`seedream_v4_5`** (precise edits/transformations): quality basic (≤4K) | high (≤6K); multi-image `image_references`.
- **`cinematic_studio_2_5`** (cinematic stills ≤4K): resolution 1k|2k|4k; element-compatible.
- **`soul_cast`** (text-only consistent character sheets): budget 10–500 (50); 16:9 only.
- **`soul_location`** (environments): no params; widest ratio set incl. 9:21.
- **`cinematic_studio_3_0`** (hero video): duration 4–15 s; resolution 480p–4k (720p); genre auto|action|horror|comedy|noir|drama|epic; generate_audio false-by-default; start+end frames; auto/21:9…9:16.
- **`cinematic_studio_video_v2`** (transition workhorse): duration 3–12 s; genre incl. suspense|intimate|spectacle; mode pro|std; speedramp auto|custom|linear|slowmo|speedup|impact; multi_shots bool; cfg_scale 0–1 (0.5); preset_id; start+end frames.
- **`seedance_2_0`** (identity + reference video): duration 4–15 s; resolution ≤4k (4k/1080p require mode=std); mode std|fast; bitrate standard|high; genre; generate_audio default true; roles start_image+end_image+image_references+video_references+audio_references (the only model with all five).
- **`kling3_0`** (multi-shot/motion transfer): duration 3–15 s; mode std|pro|4k; sound on|off; start+end frames; 16:9|9:16|1:1.
- **`minimax_hailuo`** (physics/facial emotion): variant minimax|minimax-fast|minimax-2.3|minimax-2.3-fast (2.3); duration 6|10; resolution 512|768|1080 (768) — 512 breaks end_image and 2.3 variants.
- **`veo3_1`** (ultra-real, start-only): duration 4|6|8 (8); quality basic|high|ultra; variant preview|fast (fast).
- **`veo3_1_lite`** (budget A/B batches with end-frame): duration 4|6|8; generate_audio default false (extra cost).
- **`wan2_7`** (audio-synced, character-consistent): duration 2–15 s (only model reaching 2–3 s natively); resolution 720p|1080p; start+end+audio_references.
- **`seed_audio`** (VO): format wav|mp3|pcm|ogg_opus; sample_rate up to 48000 (24000); speech/loudness −50…100; pitch −12…12; voice preset|element.
- **generate_* envelope** [FACT, schemas]: `count` 1–4 per call (image/video; audio fixed 1); `get_cost:true` returns credit cost WITHOUT submitting (the sanctioned zero-spend price probe for the run phase); media `value` = media_id or prior job_id, never URL; unsupported durations are clamped to nearest allowed; `generate_video` guidance defaults: seedance_2_0 for identity → kling3_0 for multi-shot/audio/motion transfer → kling3_0_turbo for fast single-start-frame.

**Workflows catalog** [FACT, get_workflow_instructions no-arg, 2026-07-03]: exactly ONE workflow listed: `video-explainer` v1.1 — "narrated, NON-photoreal animated explainer or story video… NOT for: photoreal film or scripted scene (cinematic-flow); product/brand ad (tv-ad); UGC/talking-head (ugc-*); motion-design typography (*MD-flow); AI podcast (podcast-flow); single clip, no narration (video-generation)." The NOT-for text names six other flow identifiers that are absent from the catalog → the photoreal movie pipeline must be orchestrated directly through generate_image/generate_video, not a bundled workflow. [UNKNOWN] whether cinematic-flow/tv-ad/ugc-*/podcast-flow exist behind other plans or later versions.

**Other surfaces present but not enumerated (names only, [FACT] from tool list):** show_generations, show_medias, job_display, media_upload/media_import_url/media_confirm, upscale_image/upscale_video, outpaint_image, reframe, remove_background, generate_3d, virality_predictor, video_analysis_create/status/jobs (shot/image analysis for the feedback loop), shorts_studio_*, personal_clipper_*, dubbing, create_voice/list_voices/voice_change, show_marketing_studio(_generations), transactions, list_workspaces/select_workspace, sync_agents, website/game tools.

## (g) Named UNKNOWNs

1. [UNKNOWN] **Credit cost per generation per model** — not exposed by models_explore, presets_show, or balance. The surface's own mechanism is `get_cost:true` on a generate_* call (returns cost without submitting), but generate_* calls were prohibited this phase; run-phase must preflight each model×resolution×duration cell.
2. [UNKNOWN] **Rate limits / concurrency caps / queue times** — nothing on any read surface states them (only `count` ≤ 4 per call is visible).
3. [UNKNOWN] **Soul training cost in credits and exact training-image quality requirements** beyond "5–20 images, ~10 min".
4. [UNKNOWN] **Whether Elements degrade identity fidelity vs Souls** — surface says Souls are "identity-faithful" and Elements are instant references; magnitude of the fidelity gap is only testable by generation.
5. [UNKNOWN] **Model-id naming collision**: the Elements tool maps `nano_banana_2` → "(Nano Banana Pro)" and `nano_banana_flash` → "(Nano Banana 2)", while the catalog has `nano_banana_2`="Nano Banana 2" and `nano_banana_pro`="Nano Banana Pro"; `nano_banana_flash` is absent from the catalog; `nano_banana_2_shots` carries the display name "Nano Banana Pro" with blank provider. Which id the Elements pipeline actually routes to is unresolved on the surface.
6. [UNKNOWN] **Web-vs-MCP catalog gaps**: higgsfield.ai marketing pages reference "Sora" and "Cinema Studio 3.5" (sources 4, 6); neither appears in this account's MCP catalog (`has_more:false` on all three lists). Treat the MCP catalog as authoritative for what is callable here.
7. [UNKNOWN] **`preset_id` interoperability** — `cinematic_studio_video_v2` exposes a `preset_id` param and `higgsfield_preset` requires one; whether all 49 presets_show ids are valid for both routes is not stated.
8. [UNKNOWN] **Spatial/"3D" audio capability** of native-audio video models (see §e).
9. [UNKNOWN] **Plan-dependence of the catalog** — whether the `plus` plan hides models/features visible on `ultra` (pricing widget tabs exist but per-plan feature matrices were not read; show_plans_and_credits is a checkout widget, not a capability matrix).

---

## Sources

1. Live Higgsfield MCP surface, read-only enumeration, 2026-07-03 (this session): `models_explore` list image/video/audio + 15 gets; `presets_show`; `get_workflow_instructions` (catalog); `show_characters` list; `show_reference_elements` list; `balance`; JSON schemas of `generate_image`, `generate_video`, `generate_audio`, `motion_control`, `show_characters`, `show_reference_elements`, `show_plans_and_credits`.
2. https://higgsfield.ai/blog/cinema-studio-guide — Cinema Studio start/end-frame "tweens" workflow (via WebSearch excerpt).
3. https://higgsfield.ai/blog/Storytelling-with-Start-End-Frames-by-Higgsfield — start/end-frame storytelling pattern (via WebSearch excerpt).
4. https://higgsfield.ai/blog/Kling-Start-End-Frames and https://higgsfield.ai/blog/A-Guide-to-Kling-Turbo-Start-End-Frame — Kling start & end frames on Higgsfield (via WebSearch excerpt).
5. https://www.revolutioninai.com/2025/12/higgsfield-cinema-studio.html — third-party description of Cinema Studio camera/lens/motion control (via WebSearch excerpt).
6. https://higgsfield.ai/ai-video — marketing model roster page naming "Sora, Kling, Veo, Seedance & More" (via WebSearch result title; page itself not fetched → claim held at [UNKNOWN] strength in §g.6).
