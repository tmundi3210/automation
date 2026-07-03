# Character Consistency & Wardrobe System — two personas (1 M + 1 F), classic-luxurious, hundreds of generations

Scope: character-bible design vocabulary, identity-consistency TECHNIQUES across AI tools (Higgsfield technique analysis included; full platform enumeration is owned by `higgsfield_surface.md`), drift detection + QC, capsule-wardrobe pre-builds, hairstyle/grooming codes, outfit-combination grammar, and duo staging. Honesty tags throughout. Blocked fetches noted: `docs.midjourney.com` and `higgsfield.ai/blog` returned HTTP 403 through the egress proxy — claims from those pages are carried via search excerpts of the official pages and tagged accordingly.

---

## 1. Character bible design (the identity anchor text)

**Principle.** A character bible is a fixed, canonical descriptor block pasted verbatim into every prompt. Community-converged guidance across consistency guides: aim for roughly **50–100 words covering age, build, hair, face, eyes, outfit, signature item** — too short and the model fills gaps differently each time; too long and parts get ignored [FACT — consistent across multiple guides, e.g. ToonyStory prompt-template guide and prompting.systems consistency guide, sources 22–23; this is practitioner consensus, not vendor doc, so treat the exact word band as [ESTIMATE], high confidence on the "fixed verbatim block" principle].

Two hard rules from the same corpus [FACT — sources 22–23]:
1. **Identical phrasing forever.** "Shoulder-length dark brown hair" must never become "brunette" — generators treat synonyms as different visual tokens.
2. **Fixed token position.** Keep the character block at the very start of the prompt; shifting its position changes the weight the model assigns to it.

**Descriptor vocabulary that generators actually respect** [ESTIMATE — synthesis of the guides above + prompting_image_video.md findings; medium-high confidence]. Generators respond to coarse, high-signal categorical attributes, not millimetre geometry. Effective axes:
- **Age**: a specific number or tight band ("34 years old") beats "adult"; ages wobble ±5–10 years without it (see §3).
- **Face geometry**: face shape (oval / square / heart / oblong), jawline (chiseled, squared, soft), cheekbones (high, prominent, subtle), chin (cleft, pointed, rounded), nose (straight, aquiline, slightly upturned), forehead (high, average).
- **Eyes**: color with modifier ("dark hazel", "grey-blue") + shape (almond, hooded, deep-set, upturned) + brow character (straight dark brows, softly arched).
- **Skin**: tone + undertone in plain language ("warm olive", "fair with cool undertone", "sun-kissed light tan") — undertone words survive lighting changes better than bare tone words [ESTIMATE — practitioner logic, medium confidence].
- **Hair**: length + texture + specific color + style + part side ("collarbone-length, softly waved, dark espresso brown, side-parted on the left") — all four slots, always.
- **Build**: somatotype-style words (lean athletic, broad-shouldered mesomorphic, slender, toned hourglass) + approximate height cue via proportion ("tall, long-limbed").
- **Distinguishing marks**: a single mole/freckle pattern is a powerful anchor for HUMAN QC but generators frequently drop or migrate marks — treat marks as a drift TEST SIGNAL, not a reliability guarantee [ESTIMATE — widely reported in consistency guides, medium confidence].

**Backstory as prompt anchor.** A 1–2 line persona logline ("A 34-year-old European heir who splits time between a Lake Como villa and a Mayfair townhouse; understated, athletic, never hurried") does two jobs: (a) it steers the model's DEFAULT fills for wardrobe, posture, setting and grading toward the genre, because context words carry strong visual priors; (b) it keeps human prompt-authors consistent across hundreds of shots. It does NOT stabilize face geometry — identity must be carried by reference conditioning (§2), the bible only stabilizes everything around the face [ESTIMATE — mechanism inference from how text conditioning works + guide consensus; high confidence on the division of labor].

**Worked persona-bible TEMPLATES (authored [ESTIMATE] — appearance decisions belong to the owner; these are proposals in the correct format, not facts):**
- **Male persona (working name "A")**: "34-year-old man, Mediterranean-European features, oval-square face, chiseled jawline, high cheekbones, straight nose, deep-set dark hazel eyes, straight dark brows, warm olive skin, short dark brown hair side-parted left with light natural sheen, clean-shaven, lean athletic broad-shouldered build, 6'1"."
- **Female persona (working name "B")**: "29-year-old woman, Southern-European features, heart-shaped face, defined cheekbones, softly pointed chin, almond dark-brown eyes, softly arched brows, straight elegant nose, sun-kissed light-olive skin, collarbone-length softly waved dark espresso hair centre-parted, slender toned build, 5'9"."

---

## 2. Consistency techniques across AI tools (weakest → strongest)

### 2.1 Prompt-only (character-sheet text)
Free and universal, but weakest: every generation re-samples the face from text priors. Sufficient for background extras; never sufficient for a recurring persona [ESTIMATE — unanimous practitioner consensus, high confidence].

### 2.2 Seed pinning — what it can and cannot do
- Seeds anchor the initial noise/composition, **not identity or style**; Midjourney explicitly produces *similar, not identical* results from the same seed, and seed behavior is not stable across model versions (a V5 seed does not reproduce on V6) [FACT — Aituts and Midjourney-seed guides, sources 20–21].
- In Stable-Diffusion-class systems, same seed + same prompt diverges when model version, scheduler/sampler settings, or image size/aspect ratio change; changing resolution alone re-composes the image [FACT — A1111 discussion + unimatrixz troubleshooting, sources 19–20].
- **Verdict**: seed pinning is a *reproducibility/debugging* tool (re-run a near-miss with one variable changed), NOT an identity system. On the Higgsfield MCP surface no user-settable seed parameter was found on the generate schemas (see higgsfield_surface.md §f) [FACT — absence read off the live surface by the parallel agent], so seed pinning is largely unavailable there anyway.

### 2.3 Reference-image conditioning (per-generation, no training)
- **Midjourney `--cref`** (V6): recreates a character's face/hair/clothes from a reference URL; official docs say it works best on Midjourney-generated characters and that "images of real people typically won't look exactly like them" [FACT — search excerpt of official doc; direct fetch 403-blocked, sources 1–2].
- **Midjourney `--oref` + `--ow`** (V7 replacement): puts a referenced character/object into new images; weight range 1–1000, default 100, practical guidance ≤400 to avoid instability; costs ~2× GPU time [FACT on ranges/default/cost — search excerpts of the official doc and updates page, sources 2–3; direct fetch 403-blocked → carried at excerpt confidence].
- **Runway Gen-4 References**: one to **three** reference images generate new stills preserving the character across lighting/locations; single-ref+text = flexible exploration, multi-ref = tighter control; reference inputs capped around 720p-class resolution [FACT — Runway help-center and Academy pages, sources 4–5].
- **Kling**: (a) **Elements** — upload 1–4 images, tag the subjects, describe interactions; the designated multi-subject consistency path for image-to-video; (b) **custom Face Model** — trained from ~10–30 guided videos, then invoked as a face reference for stable identity across angles and multi-shot scenes [FACT — Kling quickstart page + secondary reporting, sources 6–8].
- **Higgsfield Elements** (technique class, not enumeration): instant no-training references embedded as `<<<element_id>>>` placeholders in the prompt; multiple placeholders per prompt allowed → the surface-designated path for shots containing BOTH personas; not usable with the Soul models [FACT — live MCP surface, per higgsfield_surface.md §c, source 24].
- **Video-side reference roles on Higgsfield**: `seedance_2_0` accepts `image_references` for "consistent identity" and is the surface's named default for identity video; `wan2_7` self-describes character-consistent video [FACT — live MCP surface via higgsfield_surface.md, source 24].
- **Technique property**: reference conditioning "reminds" the model of the face at every step by injecting encoded features — this is the structural fix for drift; prompting alone cannot enforce identity at the model level [FACT — stated across the drift literature, sources 12–14].

### 2.4 Trained identity models (LoRA / platform fine-tunes) — strongest identity, highest setup cost
- **Open-ecosystem LoRA**: community-converged dataset spec for a face/character LoRA: ~**15–40 high-quality images** (20–25 commonly cited as the sweet spot; 30–50 for SDXL), varied angles (front / 45° / profile), varied lighting and expressions, face occupying ~40–60% of frame; below ~15 images the model struggles, above ~30 without proportional diversity it overfits ("25 good images beat 75 inconsistent ones"); Flux-style trainers want full-sentence captions [FACT that these are the published community recommendations, sources 9–11; the numbers themselves are practitioner heuristics → [ESTIMATE], medium-high confidence].
- **Overfit failure mode**: an overfit LoRA locks pose/lighting/outfit along with the face — the persona stops taking direction [ESTIMATE — standard LoRA-training caveat, high confidence].
- **Higgsfield Souls** = the platform's *managed* equivalent of this class: train from 5–20 reference images, ~10 min, then a `soul_id` conditions `soul_2`/`soul_cinematic` image generations. Hard constraint: **one soul_id per generation** → a Soul can never place both personas in one frame; and Souls work only with the two Soul models [FACT — live MCP surface via higgsfield_surface.md §c, source 24]. Whether Elements lose identity fidelity vs Souls is an open platform question [UNKNOWN — flagged on the surface file; testable only by generating].
- **Bootstrapping trick specific to this surface**: `soul_cast` (text-only "consistent cinematic character identity" model) can generate a persona SHEET first; its outputs (job IDs) are valid Soul-training inputs → design the face in soul_cast, curate 5–20 frames, train the Soul from them. [FACT that the input plumbing allows it (job IDs accepted as training inputs, per higgsfield_surface.md); the workflow itself is authored craft [ESTIMATE], high confidence it is mechanically valid.]

### 2.5 Face-swap post-pass — the rescue tool, with a realism bill
- The dominant open face-swap model (InsightFace **inswapper**) processes faces at **128×128** regardless of source resolution [FACT — InsightFace repo README + FaceSwapLab FAQ, sources 15–16].
- Consequence: swapped faces come back soft/flat, skin micro-texture is lost, and the mismatch is conspicuous on high-res output; standard remediation is a face-restoration pass (GFPGAN/CodeFormer) plus mask blending — which itself imparts a recognizable "restored-face" plastic smoothness [FACT on the pipeline and the resolution loss, sources 15–16; the "plastic" characterization is [ESTIMATE], high confidence, and it directly collides with the owner's #1 requirement (must not look AI)].
- Per-frame video swapping adds temporal flicker risk [ESTIMATE — standard practitioner caveat, medium-high confidence].
- **Verdict**: face-swap is a *last-resort rescue* for an otherwise-perfect shot whose face drifted — never the pipeline's default identity mechanism.

### 2.6 Recommended technique stack for this program [ESTIMATE — authored synthesis of §2.1–2.5 + surface constraints; medium-high confidence]
1. Design each face in `soul_cast` sheets → curate → **train one Soul per persona** (solo hero stills via soul_2/soul_cinematic).
2. **Create one Element per persona** (plus wardrobe/prop Elements) for (a) any two-persona frame and (b) every non-Soul model, including video.
3. Video identity: carry it in **pixels, not text** — generate the identity-perfect START frame as a still (Soul/Element), then feed it as `start_image`; chain clip N's end frame as clip N+1's start frame (mechanically supported: `generate_video` media accepts prior job IDs [FACT — higgsfield_surface.md §b]).
4. The character-bible text block STILL rides along in every prompt (it stabilizes wardrobe/age/grooming even when a reference carries the face).
5. Face-swap only as rescue, followed by the anti-AI QC gate (realism_antiai.md).

---

## 3. Identity drift: how it shows up, how to catch it

**Manifestations** [FACT — consistent across the drift literature, sources 12–14]:
- **Face geometry wobble**: subtle changes to facial features and proportions between shots even under an unchanged description; face drift outruns body drift, especially in long or multi-shot videos.
- **Lighting-induced identity swap**: a drastic lighting/color change can make the model produce "a different face that still matches the prompt" (soft daylight vs neon is the cited example).
- **Root cause**: frames/shots are sampled probabilistically without a persistent character state; coherence is optimized within a shot, not across shots.

**Practical drift taxonomy for QC** [ESTIMATE — authored from the above + guide corpus; high confidence as a checklist, not as measurements]: interocular spacing / eye size, jaw width, nose bridge, **age wobble** (persona reads 25 in one shot, 45 in the next), **skin-tone shift** across lighting setups, hair part flipping sides / length jumps, eyebrow thickness, tooth shape in smiles, ear shape (almost never text-anchored), mole/freckle disappearance or migration, hand/ring continuity, body proportion drift (slower).

**Automated detection**: the standard mechanism is a **face-recognition embedding + cosine similarity vs a canonical embedding** (ArcFace-class models; cosine similarity is the predominant matcher; the accept threshold is deployment-specific and set at the FAR/FRR balance point) [FACT — face-verification literature and implementations, sources 17–18]. No universal threshold exists for AI-generated faces [UNKNOWN — no published calibration found for synthetic-persona QC]; therefore: build the persona's canonical embedding from the Soul-training set, score every kept generation, and **calibrate the program's own reject threshold empirically** on the first few dozen accepted/rejected shots [ESTIMATE — sound engineering transfer of the verification method, high confidence]. On-surface hook: `video_analysis_create` exists for shot analysis in the feedback loop [FACT — tool present on the live surface, per higgsfield_surface.md §f]; whether it can score identity similarity is [UNKNOWN].

**Consistency QC checklist (run per generation batch)** [ESTIMATE — authored craft]:
1. Same face at 100% zoom side-by-side with the canonical turnaround sheet? 2. Age within band? 3. Skin tone consistent with the LIT scene (not with a different person)? 4. Hair: length, texture, color, part side, finish — all five? 5. Eyes: color + shape? 6. Jaw/cheekbone silhouette in 3/4 view? 7. Marks present and in place? 8. Grooming state (beard level / makeup level) matches the scene card? 9. Wardrobe module rendered faithfully (fabric, color, closure details)? 10. Embedding similarity above calibrated threshold? 11. Both personas verified independently in duo shots? 12. Continuity vs the adjacent clips in the stitched reel (jewelry, watch, bag, nail color).

---

## 4. Wardrobe system: capsule pre-builds for a classic-luxurious two-persona program

### 4.1 Capsule theory (the source pattern)
- The term **capsule wardrobe** was coined by **Susie Faux**, owner of the London boutique "Wardrobe," in the **1970s**: a small set of essential, timeless pieces supplemented by seasonal items [FACT — The Good Trade / capsule-history sources, sources 25–26].
- **Donna Karan** mainstreamed it in **1985** with "**Seven Easy Pieces**" — bodysuit, skirt, tailored jacket, dress, something leather, white shirt, cashmere sweater — interchangeable pieces taking a woman from office to evening [FACT — same sources].
- Modern quiet-luxury capsule guidance: ~**10–12 cohesive base pieces** where *every piece works with every other* and the palette is designed for cross-combination [FACT that this is the published guidance, sources 27, 30; the count is editorial → [ESTIMATE]].
- **Transfer to AI production** [ESTIMATE — authored, high confidence]: a "pre-built outfit" = a **named wardrobe module**: a fixed descriptor paragraph (garments, fabrics, colors, closures, accessories) reused verbatim — optionally ALSO materialized as a Higgsfield `prop`/`character` Element so the garment is reference-anchored, not just text-anchored (Element categories `character|environment|prop` confirmed on the surface [FACT — higgsfield_surface.md §c]). Modules recombine safely because palette cohesion is designed in up front — exactly the owner's "pre-build some clothes and use them in combination, not strictly."

### 4.2 Classic-luxurious MENSWEAR codes (old-money / quiet luxury)
[FACT — Wikipedia "Quiet luxury" + old-money style guides, sources 28–31]:
- **Philosophy**: understated elegance; "an almost aggressive absence of branding" — no logo larger than a thumbnail; the point is to look like you don't *need* branding.
- **Tailoring is the pillar**: even basics read bespoke; fit does the signaling that logos would.
- **Fabric signifiers**: cashmere, merino wool, linen, silk, tweed, high-grade cotton; no synthetics.
- **Canonical pieces**: navy blazer, crisp Oxford shirts, structured polos, tailored trousers/chinos, wool overcoat/trench, leather loafers, minimalist watch.
- **Palette**: navy, beige/camel, white, forest green, burgundy — engineered for effortless mixing.
- **Reference houses** (aesthetic reference ONLY — never claim or depict brand identity in generated content): Loro Piana, Brunello Cucinelli, Zegna, The Row; Savile Row heritage (Anderson & Sheppard, Huntsman), John Lobb, Turnbull & Asser [FACT that these are the named quiet-luxury/old-money reference brands, sources 29–31].
- **Black tie, correctly** (gala scenes must survive expert eyes) [FACT — Debrett's + black-tie references, sources 32–33]: black wool (barathea) dinner jacket, silk **peaked lapels or shawl collar**, covered buttons, **no vents**, single- or double-breasted; black trousers with a single braid down each leg; soft-collared white/cream shirt; **self-tied** bow tie. Getting these details into the prompt is a cheap realism win — default AI "tuxedos" hallucinate notch lapels and vents [ESTIMATE on the failure mode — medium confidence, practitioner observation].

### 4.3 Classic-luxurious WOMENSWEAR codes
[FACT — quiet-luxury womenswear guides + resort-wear editorial, sources 30, 34–35]:
- **Philosophy**: quality, clean silhouettes, no visible logos, timeless over trendy.
- **Silhouettes**: straight-cut blazer (black/camel), high-rise wide-leg trousers, crisp white shirt, midi dresses with movement and softness rather than tightness.
- **Resort register**: white/ivory midi dresses in cotton poplin or linen blends, delicate straps, open backs, softly structured waists — polished by day, transitions to evening.
- **Evening register**: well-tailored solid-color gown as the formal statement; Debrett's: evening dress, long or at least not very short [FACT — source 32].
- **Palette**: ivory, camel, navy, soft pastels; natural fabrics (cashmere, silk, wool, linen, fine cotton).

### 4.4 Fitness-crossover looks ("fitness also sometimes")
[ESTIMATE — authored to genre, medium-high confidence]: keep fitness inside the luxury codes — heritage-athletic, not gym-fluorescent: tennis whites, riding/polo styling, rowing-club knits, monochrome logo-free technical sets (charcoal/ivory/deep green), cashmere-blend zip layers, clean court sneakers or leather trainers; male physique read through fitted (not compression-shiny) fabrics; female through elegant matched sets and high-neck one-pieces at pool/yacht scenes. This preserves genre consistency while serving the psychologist/appeal brief handled in the parallel psychology file.

### 4.5 Persona palettes + module pre-builds [ESTIMATE — authored proposals for owner sign-off]
- **Persona A (male) palette**: navy, charcoal, camel, ivory, forest green; metals: brushed steel/white gold; leather: dark brown.
- **Persona B (female) palette**: ivory, cream, champagne, camel, black, powder blue; metals: yellow gold/pearl; leather: tan.
- **Male modules**: M1 Boardroom (charcoal suit, white poplin shirt, no tie, brown oxfords) · M2 Riviera (ivory linen shirt, camel pleated trousers, loafers no socks) · M3 Yacht (navy knit polo, white trousers, deck-ready loafers) · M4 Gala (Debrett's-correct black tie, §4.2) · M5 Airport (camel overcoat, cream rollneck, dark denim-free wool trousers, leather weekender) · M6 Gym-heritage (charcoal technical set, court sneakers) · M7 Countryside (tweed jacket, olive chinos, suede boots) · M8 Evening-casual (navy unstructured blazer, ecru tee, tailored trousers).
- **Female modules**: F1 Boardroom (camel straight blazer, ivory silk blouse, wide-leg trousers) · F2 Riviera (white linen midi dress, raffia tote, gold sandals) · F3 Yacht (navy-and-white knit set or high-neck ivory swimsuit + silk sarong) · F4 Gala (solid champagne silk column gown, minimal jewelry) · F5 Airport (cream cashmere co-ord, camel wrap coat, tan leather tote) · F6 Gym-heritage (ivory/charcoal matched set, sleek low bun) · F7 Countryside (cream cableknit, tailored riding-style trousers, leather boots) · F8 Evening-casual (black slip midi + straight blazer).
- Each module ships as: (a) fixed descriptor paragraph, (b) optional garment Element(s), (c) formality band 1–5, (d) allowed scenes (§6).

---

## 5. Hairstyle & grooming codes + continuity

**Male old-money grooming canon** [FACT that these are the genre's named styles — old-money hairstyle guides, sources 36–37]: classic **side part** (1920s–30s lineage, Cary Grant/Fred Astaire), **slicked back**, **Ivy League** — neat, structured, "every strand in place." Persona A proposal: side part, left, light natural sheen, clean-shaven [ESTIMATE — authored].

**Female canon** [FACT — same sources]: polished **soft waves**, **chignon**, **low bun**, sleek bob — polished but not overdone. Persona B proposal: soft waves down (default), low chignon (gala), sleek low bun (gym/yacht wind) [ESTIMATE — authored].

**Continuity rules in prompts** [ESTIMATE — authored craft, high confidence]:
1. Hair is a five-slot lock in the bible: length + texture + color + part side + finish; never drop a slot.
2. Grooming states are NAMED and enumerated (A: "clean-shaven" only, or a fixed stubble grade — stubble LENGTH wobbles badly between generations, so pick one and QC it; B: makeup level "polished-natural" default, "evening glam" for gala only).
3. Environment-consistent hair physics: yacht/wind scenes use the bun/slick variants rather than fighting the generator to keep loose hair tidy — pre-authorizing a "wind variant" per persona prevents ad-hoc hair changes that read as drift.
4. Hairstyle changes are VERSIONED events (persona bible v1.1), never per-shot improvisation — an Instagram audience tracks hair harder than jawlines [ESTIMATE — editorial judgment, medium confidence].

---

## 6. Outfit-combination grammar (authored combinatorics — [ESTIMATE], craft, for owner tuning)

**Formality bands**: 1 gym/beach · 2 resort-day · 3 smart-casual/travel · 4 dinner/cocktail · 5 gala/black-tie.

**Scene dress codes** (module compatibility matrix):
| Scene | Band | Male modules | Female modules |
|---|---|---|---|
| Yacht deck (day) | 2 | M3, M2 | F3, F2 |
| Gala / red carpet | 5 | M4 | F4 |
| Gym / training | 1 | M6 | F6 |
| Airport / private terminal | 3 | M5 | F5 |
| Riviera café / resort | 2 | M2 | F2 |
| Boardroom / city | 4 | M1 | F1 |
| Gallery / dinner | 4 | M8 | F8 |
| Countryside / estate | 3 | M7 | F7 |
| Luxury-car night drive | 3–4 | M8, M1 | F8 |

**Combination rules** [ESTIMATE — authored from quiet-luxury editorial logic, sources 28–31]:
1. **One statement maximum** per outfit (a bold coat OR a bold accessory, never both) — quiet luxury reads through restraint.
2. **Palette adjacency**: pieces combine only within the persona palette (§4.5); cross-persona coordination in duo shots = complementary, not matching ("his navy, her ivory"), never identical colors head-to-toe.
3. **Band adjacency**: pieces may cross-combine within ±1 formality band (M2 trousers under an M8 blazer = fine; M6 sneakers with M4 = never).
4. **Fabric season lock**: linen/poplin modules only in sun scenes; tweed/cashmere-overcoat only in cool scenes — fabric-season mismatch is an instant AI tell.
5. **Accessory continuity set**: each persona has ONE watch, ONE default jewelry set, ONE bag family, fixed in the bible; accessories are the highest-visibility continuity objects across a stitched reel.
6. Flexibility clause (the owner's own): modules are defaults, not law — off-matrix combinations are allowed when a scene demands it, but they must be logged as new named modules if reused.

---

## 7. Persona-pair dynamics on screen (duo staging)

**Craft base from couples-photography posing** [FACT that these are the published techniques — PetaPixel / ShootDotEdit / posing guides, sources 38–39]:
- **Seated poses equalize height** and read naturally intimate (bench, steps, yacht gunwale).
- **Slope/step staging**: shorter partner uphill/on a step to manage the height gap; taller partner widens stance to drop height.
- **Connection points carry the shot**: hands, lean-in, forehead-close "nearly" moments — attention goes to the connection, not the geometry.
- **Walking pose**: one leads, hand held back to the other, look-back over shoulder.
- **Slightly elevated camera angle** flatters two-person compositions.

**Translation to AI prompting + this platform** [ESTIMATE — authored synthesis, medium-high confidence]:
1. Duo shots CANNOT use two Souls (one soul_id per generation [FACT — higgsfield_surface.md §c]) → all duo frames go through **two character Elements in one prompt** (multi-placeholder), or through a start-frame that already contains both verified personas.
2. Prompt staging language that maps to the craft above: "seated together at the stern, her hand on his forearm, both looking toward the horizon, camera slightly above eye level" — specify WHO is where, ONE connection point, and a shared eyeline (both-at-camera reads promotional; both-at-horizon or at-each-other reads cinematic).
3. **Duo grammar for the genre** [ESTIMATE — authored]: default blocking = offset not symmetric (one persona half-a-step behind or angled 30°); physical contact minimal-and-deliberate (hand on arm, not embrace) to match the classic-luxurious register; each persona also carries SOLO shots in every location so the pair reads as two individuals with a shared world, not a locked unit — this also halves identity-consistency risk per shot.
4. QC both identities independently in every duo frame (§3 checklist item 11); duo frames drift more because reference strength is split across two subjects [ESTIMATE — inference from multi-subject conditioning behavior, medium confidence; magnitude untested → [UNKNOWN] until run phase].

---

## Named UNKNOWNs (for the run phase)
1. [UNKNOWN] Element-vs-Soul identity-fidelity gap on Higgsfield (surface says Souls are "identity-faithful"; magnitude only testable by generating).
2. [UNKNOWN] Whether two character Elements in one prompt degrade each identity, and by how much.
3. [UNKNOWN] A calibrated embedding-similarity accept threshold for these two synthetic personas (must be measured on the first accepted/rejected batches).
4. [UNKNOWN] Whether `video_analysis_create` can score identity similarity or only general shot content.
5. [UNKNOWN] How faithfully garment Elements reproduce fabric/closure details vs text-only wardrobe modules.
6. [UNKNOWN] Exact Midjourney/Runway/Kling doc details beyond search excerpts where official pages were 403-blocked (docs.midjourney.com, higgsfield.ai/blog) — excerpt-level confidence only.

## Sources
1. Midjourney Character Reference (official doc; 403-blocked direct, via search excerpt): https://docs.midjourney.com/hc/en-us/articles/32162917505293-Character-Reference
2. Midjourney Omni Reference (official doc; 403-blocked direct, via search excerpt): https://docs.midjourney.com/hc/en-us/articles/36285124473997-Omni-Reference
3. Midjourney updates — Omni-Reference --oref: https://updates.midjourney.com/omni-reference-oref/
4. Runway help — Creating with Gen-4 Image References: https://help.runwayml.com/hc/en-us/articles/40042718905875-Creating-with-Gen-4-Image-References
5. Runway Academy — Gen-4 References: https://academy.runwayml.com/tutorial/gen-4-references
6. Kling AI quickstart — Elements (video character consistency): https://app.klingai.com/global/quickstart/ai-video-character-consistency
7. Kling custom Face Model how-to (Pollo AI): https://pollo.ai/hub/how-to-use-kling-ai-face-model
8. Kling Custom Model feature report (AIbase): https://news.aibase.com/news/13089
9. Character LoRA dataset guide (RunComfy): https://www.runcomfy.com/trainer/ai-toolkit/z-image-character-lora-dataset-guide
10. Train Stable Diffusion LoRA guide: https://sanj.dev/post/train-stable-diffusion-lora-self-portraits/
11. LoRA training parameters — human character (HF forums): https://discuss.huggingface.co/t/perfect-lora-training-parameters-human-character/147211
12. Magic Hour — AI video consistency / character drift: https://magichour.ai/blog/ai-video-consistency-character-face-tools
13. Kling blog — drift in AI-generated video: https://kling.ai/blog/fix-ai-video-drift-consistency-guide
14. DZone — Gen AI video identity drift and hallucination: https://dzone.com/articles/gen-ai-video-approach-to-identity-drift-and-hallucination
15. InsightFace inswapper README (128×128): https://github.com/deepinsight/insightface/blob/master/examples/in_swapper/README.md
16. FaceSwapLab FAQ (resolution/quality limits, GFPGAN post): https://glucauze.github.io/sd-webui-faceswaplab/faq/
17. ArcFace vs CosFace (cosine-similarity matching): https://didit.me/blog/arcface-vs-cosface-deep-dive-into-face-matching-algorithms/
18. ArcFace for Disguised Face Recognition (thresholding at FAR/FRR): https://openaccess.thecvf.com/content_ICCVW_2019/papers/DFW/Deng_ArcFace_for_Disguised_Face_Recognition_ICCVW_2019_paper.pdf
19. A1111 discussion — same seed, different image: https://github.com/AUTOMATIC1111/stable-diffusion-webui/discussions/7578
20. Unimatrixz — seed troubleshooting: https://unimatrixz.com/blog/troubleshooting-stable-diffusion-why-your-seed-is-not-generating-the-same-image/
21. Aituts — Midjourney seeds explained (similar, not identical): https://aituts.com/midjourney-seeds/
22. ToonyStory — consistent-character prompt templates: https://toonystory.com/ai-character-consistency/prompt-templates
23. prompting.systems — creating consistent characters in AI art: https://prompting.systems/blog/creating-consistent-characters-in-ai-art
24. Internal, live-surface primary source: /home/user/automation/FACTORY/movie_studio/research/higgsfield_surface.md (Higgsfield MCP read-only enumeration, 2026-07-03)
25. The Good Trade — What is a capsule wardrobe (Faux/Karan history): https://www.thegoodtrade.com/features/what-is-a-capsule-wardrobe/
26. BUST — history of capsule wardrobes: https://bust.com/style/198114-vogue-donna-karan-french-style-capsule-wardrobes.html
27. Salsa — quiet luxury minimalist capsule (10–12 pieces): https://www.salsajeans.com/en/quiet-luxury-and-old-money-aesthetic:-how-to-build-a-minimalist-capsule-wardrobe_582.html?idb=86
28. Wikipedia — Quiet luxury: https://en.wikipedia.org/wiki/Quiet_luxury
29. The VOU — old money brands / codes: https://thevou.com/blog/old-money-fashion-brands-men/
30. AGLAIA — old money style guide: https://www.aglaiamagazine.com/old-money-style-guide-to-mastering-quiet-luxury/
31. Luxury Columnist — quiet luxury brands: https://luxurycolumnist.com/quiet-luxury-brands/
32. Debrett's — Deconstructing dress codes: https://debretts.com/deconstructing-dress-codes/
33. Wikipedia — Black tie: https://en.wikipedia.org/wiki/Black_tie
34. Le Magazine Éclat — luxury resort outfits: https://lemagazineeclat.com/luxury-resort-outfits-for-summer-that-look-effortlessly-expensive/
35. Who What Wear — luxury capsule wardrobe: https://www.whowhatwear.com/luxury-capsule-wardrobe
36. Vieux Riche — old money hairstyles (men & women): https://thevieuxriche.com/blogs/articles/old-money-hairstyles-men-women
37. The VOU — old money hairstyle guide: https://thevou.com/blog/old-money-hairstyle-guide/
38. PetaPixel — photographing couples with height difference: https://petapixel.com/2021/08/12/how-to-photograph-couples-with-height-difference/
39. ShootDotEdit — posing height-difference couples: https://shootdotedit.com/blogs/news/posing-height-difference-couples
40. Higgsfield blog — tools for consistent AI characters (403-blocked; title/existence only, no content claims used): https://higgsfield.ai/blog/tools-for-consistent-ai-characters
