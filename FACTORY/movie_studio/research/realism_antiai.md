# Realism / Anti-AI-Look — Artifact Taxonomy, Human-Detection Evidence, Physics QC, Mitigation Map

Scope: the owner-ranked #1 requirement ("it should not look AI — natural, realistic, physics and all") turned into (a) a grounded taxonomy of image and video tells, (b) what humans actually notice per published studies, (c) a luxury-content physics checklist, (d) a per-shot QC rubric, (e) a mitigation map, (f) the compression/small-screen factor.

**Grounding note (honest):** the egress proxy 403-blocked EVERY direct WebFetch attempted (arxiv.org/html and /abs, journals.sagepub.com, pmc.ncbi.nlm.nih.gov, gijn.org, insight.kellogg.northwestern.edu, caniphish.com; plus curl CONNECT-tunnel 403). Grounding therefore comes from two live surfaces: (1) WebSearch result excerpts of the named papers/guides, and (2) Hugging Face `paper_search` MCP abstracts (full abstracts read off the live tool surface). Where a number comes from a search excerpt of the primary paper rather than the fetched paper itself, it is tagged **[FACT — via search excerpt]**. Nothing below is invented.

---

## 1. Taxonomy of AI IMAGE tells (still frames, posters, start/end frames)

Organized by the categories the forensics literature actually uses. The 2026 comprehensive review of AI-image detection (arXiv 2502.15176) gives an artifact taxonomy including: improper fur/hair direction flows, unrealistic eye reflections, anatomically impossible joint configurations, biological asymmetry errors, exaggerated characteristic features (e.g., overly large eyes), unnatural repetition/tiling patterns, and illogical light sources / incorrect shadow geometry [FACT — via search excerpt of arXiv 2502.15176, S1].

### 1a. Anatomy
- **Hands/fingers**: wrong finger count, merged fingers, thumb on wrong side, knuckles bending in unnatural directions [FACT — verification guides, S8, S10]. Caveat carried honestly: by 2025 leading image models (search excerpt names Midjourney, DALL-E) render hands far better, so hands are a *weakening* still-image tell [FACT — via search excerpt, S10] — though they remain a strong VIDEO tell (§2).
- **Eyes**: unrealistic/mismatched corneal reflections — the two eyes should show the same environment; AI eyes often don't [FACT — review taxonomy, S1]. Overly large or exaggerated eyes flagged as a taxonomy category [FACT, S1].
- **Teeth/ears**: covered in the review under biological-asymmetry and exaggerated-feature error classes (asymmetric paired features, irregular dentition) [FACT that asymmetry/feature classes exist in the taxonomy, S1; the specific teeth/ears instantiations are [ESTIMATE — standard instantiations of those classes in practitioner guides, high confidence]].
- **Joints**: anatomically impossible joint configurations — named review category [FACT, S1].

### 1b. Texture
- **Skin over-smoothing**: waxy, poreless, uniformly lit skin; forensic detectors exploit micro-texture statistics (Local Binary Patterns, GLCM homogeneity/entropy) precisely because generated skin deviates from natural micro-roughness [FACT — via search excerpt of S1].
- **Frequency fingerprints**: generators leave model-dependent fingerprints in local frequency bands from learned upsampling filters and noise injection — invisible to eyes, visible to detectors [FACT, S1]. Relevant later: compression destroys these (§7).
- **Repetition/tiling**: perfect repeating texture (fabric weave, foliage, crowd faces) is "highly indicative of synthetic generation" [FACT — via search excerpt, S1].
- **Hair/fur direction**: improper flow fields — strands that ignore gravity or merge into skin [FACT — review category, S1].
- **Fabric weave**: weave that changes scale mid-garment or tiles perfectly falls under the repetition + texture-irregularity categories [ESTIMATE — direct application of S1 categories to garments, high confidence].

### 1c. Text and logos
- Garbled lettering, near-English words, mixed alphabets on signs/books/shirts — historically the most reliable single tell [FACT — via search excerpts, S8, S10]. Caveat: newer models can produce clean typography, so absence of garble is not proof of real, but presence of garble is near-proof of AI [FACT — via search excerpt, S10]. For a luxury operation: brand logos (car marques, fashion houses) are high-risk because viewers know exactly what a Rolls-Royce grille badge or interlocking-C logo looks like [ESTIMATE — reasoning from the text-tell evidence, high confidence].

### 1d. Geometry: shadows, reflections, perspective
- **Shadows**: in a single-light scene (sun), all shadows must point away from that source; AI images frequently show shadows in multiple directions — a documented forensic check (Amped Authenticate shadow tool) [FACT — via search excerpt, S8, S9].
- **Reflections**: for a flat reflective surface, lines connecting each object point to its reflection must converge to a single vanishing point; AI reflections break this — reflections show wrong angle, missing elements, or phantom objects [FACT — Amped Authenticate reflection technique, S9; guide excerpts, S10].
- **Perspective**: illogical light sources / incorrect shadow geometry is a named review category [FACT, S1]; converging-lines/vanishing-point errors are the practitioner extension [ESTIMATE — high confidence].

### 1e. Background coherence and object count
- Melted/incoherent background objects, half-formed people, chairs with wrong leg counts — semantic incoherence is a top-level artifact domain in the video benchmark taxonomy (§2) and applies to stills [FACT that the category exists, S2; instantiation [ESTIMATE — high confidence]].
- **Object count drift**: too many/too few of countable things (fingers, wheels' spokes, wine glasses on a table) — an instance of the review's repetition/asymmetry classes [ESTIMATE — basis: taxonomy classes in S1 + guide excerpts; high confidence]. No fetched source gave a count-error frequency statistic — [UNKNOWN: published base rates per artifact type; the blocked Artifact-Bench full text likely has per-type frequencies].

---

## 2. Taxonomy of AI VIDEO tells

Anchor: **Artifact-Bench** (arXiv 2605.18984, May 2026) — a three-level hierarchical taxonomy of realism artifacts (top-level artifact domains → mid-level failure families → **30 fine-grained artifact types**), covering photorealistic, animated, and CG-style video; abstract names the top-level domains as temporal inconsistencies, structural distortions, and semantic incoherence [FACT — abstract read off HF paper_search, S2]. The full 30-type list sits in the paper body, which was proxy-blocked — [UNKNOWN: the exact 30 labels; the three domains and tier structure are FACT].

### 2a. Temporal artifacts
- **Flicker**: frame-to-frame luminance/texture instability; StreamingT2V explicitly added a refinement stage "to suppress temporal flickering and repetitive self-similarity," i.e., builders acknowledge the artifact [FACT — via search excerpt, S3]; ATSS detects AI video via anomalous temporal self-similarity [FACT — paper exists, S3].
- **Texture boiling**: faint "boiling" on skin and fabric plus unnaturally uniform frame cadence — named in detection guides [FACT — via search excerpts, S11, S12].
- **Morphing**: fingers deform during motion; held objects blend into hands/body because the model can't segment person from prop [FACT — via search excerpts, S11].
- **Identity drift**: character's eye color, jacket style, face geometry changing across frames/shots — industry-named phenomenon ("identity drift") [FACT — via search excerpts, S12, S13]. Directly threatens the two-persona consistency requirement.
- **Unstable edges / drifting details** around hands, hair, text [FACT — via search excerpt, S11].

### 2b. Physics violations (the owner's explicit concern)
- **Second-order motion (acceleration)**: D3 (arXiv 2508.00701) shows a *fundamental divergence in second-order feature distributions* (Newtonian second-order central differences) between real and AI video — i.e., AI motion has wrong accelerations even when positions look fine; training-free detection on 40 dataset subsets, +10.39 pp mAP over prior best on GenVideo [FACT — abstract, S3].
- **Probability-flow violations**: NSG-VD (arXiv 2510.08073) detects AI video via deviations from natural spatiotemporal dynamics (Normalized Spatiotemporal Gradient), +16.00 pp recall over SOTA baselines [FACT — abstract, S3].
- **Benchmarked physics failure rates**: VideoPhy (arXiv 2406.03520): best model tested (CogVideoX-5B) produced videos adhering to both caption and physical laws in only **39.6%** of instances; solid–fluid and fluid–fluid interactions specifically curated as failure-prone [FACT — abstract, S4]. VideoPhy-2 (arXiv 2503.06800): best model only **22% joint performance** on the hard subset; models "particularly struggle with conservation laws like mass and momentum" [FACT — abstract, S4]. PhyGenBench (arXiv 2410.05363, ICML 2025): 160 prompts across 27 physical laws in 4 domains (optics, mechanics, thermal, material properties); current models fail broadly, and *neither scaling nor prompt engineering fully fixes dynamic scenarios* [FACT — abstract, S5].
- **Concrete violation types** (guides + benchmark categories): gravity ignored, momentum discontinuities (foot lands, body keeps gliding), **foot sliding / missing friction at contact points** ("watch feet, wheels, and contact points") [FACT — via search excerpt, S11]; cloth/hair that doesn't carry inertia; water that doesn't conserve volume [conservation-law struggle is FACT per VideoPhy-2, S4; per-material instantiation ESTIMATE, high confidence].
- **Complex articulated action**: Sora launched with the stated limitation that it "generates unrealistic physics and struggles with complex actions over long durations"; viral AI gymnastics clips show limbs interchanging and heads reattaching [FACT — via search excerpts, S14]. Eating/drinking and dexterous hand-object manipulation are documented open problems (dedicated research lines: ManiVideo, hand-object interaction generation) [FACT — papers exist, S14].

### 2c. Detector-vs-human note
Artifact-Bench evaluated 19 leading multimodal LLMs on artifact spotting: many were near or below random in challenging settings, and MLLM judgments significantly misalign with human perceptual preference [FACT — abstract, S2]. Consequence for the pipeline: an automated VLM-based QC critic is useful but cannot be the only gate; a human (or at minimum the ordered rubric in §5 applied deliberately) must sit in the feedback loop.

---

## 3. What human-detection studies actually show (numbers, honest)

- **Meta-analysis (Diel et al. 2024, Computers in Human Behavior: AI, 56 papers, 86,155 participants)**: overall human deepfake-detection accuracy **55.54%** (95% CI 48.87–62.10) — CI crosses chance. By modality: video **57.31%** [47.80–66.57], images **53.16%** [42.12–64.64], audio **62.08%** [38.23–83.18], text 52.00% [37.42–65.88] [FACT — via search excerpt of the published meta-analysis, S6].
- **Groh et al., PNAS 2022 (15,016 participants)**: ~66% accuracy identifying deepfake videos of faces; rises to 73% when participants saw a model's prediction before finalizing [FACT — via search excerpt, S7].
- **Nightingale & Farid, PNAS 2022**: with training and trial-by-trial feedback, average accuracy on real-vs-GAN faces reached only **59.0%** (95% CI 57.7–60.4); synthetic faces were rated MORE trustworthy than real ones (4.82 vs 4.48 on a 7-point scale) [FACT — via search excerpt of the PNAS record, S15]. The widely reported no-training baseline is 48.2% (below chance) [ESTIMATE — figure widely reported for Experiment 1; primary PDF proxy-blocked, so carried at high-but-not-FACT confidence].
- **Miller et al. 2023, Psychological Science ("AI Hyperrealism")**: White AI (StyleGAN2) faces judged human **69.5%** of the time vs **52.2%** for real White human faces; the most-confident participants made the MOST errors (Dunning–Kruger pattern); hyperrealism driven by AI faces being more average, more attractive, more familiar, less memorable [FACT — via search excerpts of the paper and its summaries, S16].
- **Trainability**: a 2025 Frontiers in AI study trained humans on synthetic-face artifacts with measurable improvement; best individual performers reach ~80%; super-recognizers beat average performers by ~15 percentage points on AI-vs-real face discrimination [FACT — via search excerpts, S16, S17].

**Operational readings (synthesis, tagged [ESTIMATE — reasoned from the numbers above, high confidence unless noted):**
1. Static AI *faces* already pass human inspection — face realism is a mostly-won battle; do not over-spend QC there.
2. Humans are barely above chance overall, BUT the studies test one-shot judgments of content people were *asked* to judge. An Instagram account posts repeatedly: a follower sees dozens of clips, and one blatant tell (garbled logo, six fingers, sliding feet) flips the account's perceived authenticity permanently. QC must therefore be per-shot and worst-artifact-driven, not average-quality-driven.
3. Video gives viewers more evidence than stills (motion physics); the physics benchmarks (22–39.6% pass rates) show motion is where current models objectively fail — so the scarce QC attention should go to MOTION tells first, texture last.
4. Audience confidence is miscalibrated (Dunning–Kruger, S16) — comment-section "this is AI" accusations will happen even on clean content; the defense is consistency over many posts, not any single perfect clip.

---

## 4. Physics-plausibility checklist for LUXURY content specifically

Each item = what must be true physically; violation = tell. Physical laws stated are elementary optics/mechanics; their status as *documented AI failure domains* is grounded where cited. PhyGenBench's four failure domains — optics, mechanics, thermal, material properties — cover every row below [FACT — S5].

| Asset | Must be true | Common AI failure | Grounding |
|---|---|---|---|
| **Car paint/body reflections** | Environment reflections slide across curved paint as camera/car moves; reflected scene matches the visible scene | "Baked" static reflections; reflections of objects not in the scene | Reflection-consistency forensics [FACT, S9]; motion application [ESTIMATE, high conf.] |
| **Wheel rotation** | Rolling without slipping: wheel angular speed matches ground speed; contact patch stays planted | Wheels sliding (rotation too slow/absent), spoke count drifting; note: stroboscopic "wagon-wheel" backward-spin IS real-footage-plausible at some shutter/frame-rate combos, so don't over-flag it | Foot/wheel/contact-point sliding named in guides [FACT — via excerpt, S11]; rolling condition + wagon-wheel caveat [ESTIMATE — standard mechanics/cinematography, high conf.] |
| **Glass & liquid** | Refraction: background shifts/inverts through a filled glass; pour streams narrow as they fall (continuity); level rises as poured; carbonation rises | Liquid volume not conserved, pour with no level change, glass with no refraction | Solid–fluid / fluid–fluid interactions are curated VideoPhy failure classes; conservation of mass a named worst-area [FACT, S4] |
| **Fabric drape** | Cloth carries inertia — swings after motion stops; silk vs wool drape differently; weave scale constant | Cloth glued to body, weave tiling/boiling, drape ignoring wind/motion | Material properties = PhyGenBench domain [FACT, S5]; texture tiling [FACT, S1]; boiling [FACT, S11] |
| **Jewelry sparkle** | Specular glints move as light/camera/wrist moves; a diamond's fire changes color with angle | Static painted-on sparkle; glints inconsistent with the scene's single key light | Optics domain [FACT, S5]; instantiation [ESTIMATE, high conf.] |
| **Pool/ocean water** | Sun glints align with sun position; caustics dance; splashes conserve volume and fall ballistically; ripples disperse outward and decay | Water texture boiling, splash appearing from nothing, ripples without cause | Fluid classes fail in VideoPhy [FACT, S4]; specifics [ESTIMATE, high conf.] |
| **Golden-hour shadows** | ONE sun ⇒ all shadows parallel, long, pointing away from it; warm key + cool sky fill; shadow softness consistent | Multiple shadow directions in one frame — a *documented* forensic check | [FACT — Amped shadow-consistency technique, S8, S9] |
| **Human motion (fitness accents)** | Accelerations Newtonian; feet plant without sliding; mass shifts before a lift | Foot slide, momentum discontinuity, impossible joint angles | Second-order divergence [FACT, S3]; foot sliding [FACT — via excerpt, S11]; gymnastics-class failures [FACT, S14] |

Luxury-specific severity note [ESTIMATE — audience reasoning, moderate-high confidence]: the luxury audience contains genre experts (car people know wheel/stance/badge details; watch/jewelry people know how light plays on a bezel). Genre-expert scrutiny raises the effective detection rate above the ~55% general-population baseline for exactly these objects — so car badges, wheels, watch faces, and logo-bearing goods deserve disproportionate QC.

---

## 5. QC scoring rubric — per shot, ordered by human-noticeability

Ordering rationale (honest): no fetched study ranks cue noticeability directly; order is an editorial synthesis [ESTIMATE — basis: prevalence and emphasis across forensic guides (S8–S11), the physics-benchmark failure severity (S4, S5), and the §3 finding that faces pass while motion fails; confidence moderate-high]. Score each item 0 = clean, 1 = visible on inspection, 2 = visible at a glance. **Gate: any single 2 ⇒ re-roll; total ≥ 4 ⇒ re-roll; total 2–3 ⇒ fixable in post only if the offending region is croppable/gradeable (§6).**

1. **Text/logos/badges** — any garbled or off-model lettering, marque badge, watch face. Most damning because binary-checkable by any viewer [FACT that text is a top guide tell, S8, S10].
2. **Hands in motion** — finger count, morphing, held-object blending [FACT, S11].
3. **Contact physics** — foot slide, wheel slip, objects hovering, splash-from-nothing [FACT — S11 (contact points), S4 (conservation failures)].
4. **Identity lock** — persona's face/eye-color/hair/wardrobe identical to the reference sheet, within-shot and across shots [FACT — identity drift is a named failure, S12, S13].
5. **Object permanence & count** — items appearing/vanishing between frames; glass count on the table stable [ESTIMATE — instance of temporal-inconsistency domain S2, high conf.].
6. **Shadow/reflection geometry** — one shadow direction; reflections match scene [FACT — S8, S9].
7. **Motion quality** — accelerations natural, no floaty slow-drift look, cloth/hair inertia present [FACT — second-order divergence S3; floatiness instantiation ESTIMATE].
8. **Texture stability** — skin/fabric boiling, edge shimmer [FACT, S11].
9. **Background coherence** — melted extras, impossible furniture, tiling foliage [FACT — semantic-incoherence domain S2; S1 tiling].
10. **Skin micro-texture** — waxy over-smoothing (lowest priority: largely invisible after IG compression, §7) [FACT that detectors need micro-texture S1 + compression destroys it S18].

Apply at three checkpoints: start-frame image (items 1,4,5,6,9,10), raw video (all), post-edit reel (items 1–4 at final resolution on a phone screen).

---

## 6. Mitigation map — which fix for which tell

- **Fixable by PROMPT**: physics-negative prompting works measurably — PhysVid's local physics conditioning + *negative physics prompts* ("descriptions of locally relevant law violations" to steer away from them) improved VideoPhy physical-commonsense scores ~33% over baseline generators [FACT — abstract, S19]; describing lighting singularly ("low golden sun camera-left, long parallel shadows") constrains shadow geometry [ESTIMATE, moderate]; avoiding text ("no visible signage, logo-free styling") eliminates tell #1 at the prompt level [ESTIMATE, high]. HARD LIMIT carried honestly: PhyGenBench found prompt engineering insufficient for dynamic physics scenarios [FACT, S5] — prompts reduce, never guarantee.
- **Fixable by MODEL CHOICE / MODE CHOICE**: physics adherence varies hugely across models (VideoPhy/VideoPhy-2 spreads; best ≠ average) [FACT, S4] — the owner's own directive ("try different, which gives better result") is the right policy; A/B per shot-type and keep a per-model failure ledger. Image-to-video from a QC-approved start frame (owner's start/end-frame workflow) locks anatomy, wardrobe, and identity at t=0, converting image-QC wins into video wins and shrinking identity-drift exposure [ESTIMATE — mechanism reasoning, high confidence; which Higgsfield modes support first/last-frame conditioning is the MCP-surface research file's question, [UNKNOWN] here].
- **Fixable by RE-ROLL**: generation is stochastic; per-seed artifacts (a bad hand, one flicker burst) differ run to run — re-roll is the cheapest fix for any single rubric-2 [ESTIMATE — standard generative practice, high confidence]. Budget rule: QC BEFORE spending on upscale/post.
- **Fixable in POST**: crop/reframe to excise a bad hand or melted background edge; punch-in hides border artifacts; color grade + film-grain overlay adds the sensor noise and character whose absence reads synthetic (and masks waxy skin) [ESTIMATE — grain restores the high-frequency statistics detectors say generated images lack (S1-adjacent reasoning), moderate-high]; speed ramps shorten the window in which physics can be judged and motion-blur transitions (whip pans) mask cuts placed exactly where drift was building [ESTIMATE, moderate-high]; cut length itself: the owner's 3–6 s stitched format is intrinsically protective — temporal artifacts accumulate with duration (Sora's stated long-duration weakness, S14), so short shots cut before drift compounds [FACT for duration-dependence, S14; format-as-mitigation framing ESTIMATE].
- **Fixable only by SHOT DESIGN — the known-hard shot list (avoid or shoot around)**:
  1. Eating/drinking to the mouth [FACT — documented failure, S14]
  2. Dexterous hand-object manipulation (pouring, typing, instruments, jewelry clasping) [FACT — open research problem, S14]
  3. Athletic/gymnastic full-body action; fast martial/dance choreography [FACT, S14]
  4. Walking/running with feet prominent in frame (foot-slide exposure) [FACT — contact-point tell, S11]
  5. Legible text: signage, menus, watch faces, badge close-ups [FACT — text tell, S8, S10]
  6. Mirrors/large reflective surfaces showing the subject [FACT — reflection forensics, S9]
  7. Crowds and background extras (semantic-incoherence exposure) [ESTIMATE — from S1/S2 categories, high conf.]
  8. Hero fluid shots: champagne pours, splashes, held long [FACT — fluid classes fail, S4]
  Workarounds: imply rather than show (glass already full, lifted toward lips off-frame; feet out of frame; wheels in motion-blur; reflections defocused via shallow DOF) [ESTIMATE — direct design consequence, high confidence].

---

## 7. Platform/context factor — what IG compression and phone screens hide (and don't)

- Social platforms recompress aggressively; the deepfake-forensics literature is explicit that this "launders low-level forensic cues": block effects obscure artifacts, textural information is lost, and detectors trained on clean data degrade badly on platform-compressed video [FACT — via search excerpts of compressed-deepfake detection papers, S18].
- Therefore compression HIDES: generator frequency fingerprints and upsampling signatures [FACT — these are the low-level cues named in S1/S18], skin micro-texture waxiness, faint boiling/shimmer, film-grain absence [ESTIMATE — these are high-frequency phenomena of exactly the kind compression removes, high confidence].
- Compression does NOT hide: garbled logos, wrong finger counts, foot sliding, shadow-direction errors, identity drift, object count changes — these are semantic/geometric, survive any bitrate, and are the cues human raters use anyway [ESTIMATE — reasoned from what DCT-style compression removes (high-frequency detail) vs preserves (structure/motion), high confidence; consistent with humans scoring 57.31% on video (S6) largely via semantic cues].
- Small vertical phone screens (9:16, ~6-inch) further shrink fine-detail visibility; a shot that fails rubric item 10 but passes items 1–6 will look clean in-feed [ESTIMATE — display-size reasoning, moderate-high; no fetched study quantified screen-size effects — [UNKNOWN: quantified small-screen detection deltas]].
- Honest flip side: compression is not a license to relax — it also *adds* blocky artifacts that can make borderline AI texture look worse, and Meta applies "AI info" labeling (disclosure obligations are the media-manager file's scope, not a visual tell) [ESTIMATE on the artifact-amplification point, moderate; labeling existence FACT-adjacent but sourced in the media-manager research file, not here].

**Net QC doctrine**: optimize for the post-compression phone-screen viewing condition — final QC pass on an actual phone at feed resolution — and spend the rubric budget top-down (text → hands → contact physics → identity), because those are what both the physics benchmarks and human raters converge on.

---

## Sources

1. Methods and Trends in Detecting AI-Generated Images: A Comprehensive Review — https://arxiv.org/html/2502.15176v2 (direct fetch proxy-blocked; grounded via search excerpts; also published as https://www.sciencedirect.com/science/article/pii/S1574013726000171)
2. Artifact-Bench: Evaluating MLLMs on Detecting and Assessing the Artifacts of AI-Generated Videos — https://hf.co/papers/2605.18984 (full abstract read via Hugging Face paper_search; arxiv HTML proxy-blocked)
3. D3: Training-Free AI-Generated Video Detection Using Second-Order Features — https://hf.co/papers/2508.00701 ; NSG-VD, Physics-Driven Spatiotemporal Modeling — https://hf.co/papers/2510.08073 ; ATSS — https://arxiv.org/html/2604.04029
4. VideoPhy: Evaluating Physical Commonsense for Video Generation — https://hf.co/papers/2406.03520 ; VideoPhy-2 — https://hf.co/papers/2503.06800 (abstracts via HF paper_search)
5. PhyGenBench / Towards World Simulator (ICML 2025) — https://hf.co/papers/2410.05363 ; code: https://github.com/OpenGVLab/PhyGenBench
6. Diel et al., Human performance in detecting deepfakes: systematic review and meta-analysis of 56 papers — https://www.sciencedirect.com/science/article/pii/S2451958824001714 (numbers via search excerpts; PDF mirror: http://macdorman.com/kfm/writings/pubs/Diel-2024-Human-Performance-Detecting-Deepfakes-Meta-Analysis.pdf)
7. Groh et al., Deepfake detection by human crowds, machines, and machine-informed crowds, PNAS 2022 — https://www.pnas.org/doi/10.1073/pnas.2110013119 (via search excerpts)
8. How To Reveal AI-generated Images by Checking Shadows and Reflections in Amped Authenticate — https://blog.ampedsoftware.com/2023/10/11/how-to-reveal-ai-generated-images-by-checking-shadows-and-reflections-in-amped-authenticate
9. Forensic Focus mirror of the Amped shadows/reflections technique — https://www.forensicfocus.com/articles/how-to-reveal-ai-generated-images-by-checking-shadows-and-reflections-in-amped-authenticate/
10. How to Spot AI-Generated Images: 10 Telltale Signs — https://whichoneis.ai/blog/how-to-spot-ai-generated-images ; Reporter's Guide to Detecting AI-Generated Content (GIJN) — https://gijn.org/resource/guide-detecting-ai-generated-content/ (GIJN fetch proxy-blocked; via search excerpts)
11. 10 Techniques To Spot AI-Generated Videos — https://caniphish.com/blog/how-to-spot-ai-videos (fetch proxy-blocked; via search excerpts) ; Runway Video: How to Spot AI Motion and Artifacts Fast — https://detectvideo.ai/runway-video/
12. All You Need to Know about Drift in AI-generated Video — https://kling.ai/blog/fix-ai-video-drift-consistency-guide
13. Gen AI Video: Identity Drift and Hallucination — https://dzone.com/articles/gen-ai-video-approach-to-identity-drift-and-hallucination
14. Sora limitations / AI gymnastics coverage — https://www.digitalcameraworld.com/tech/software/horrifying-yet-hilarious-viral-gymnastics-video-illustrates-why-videographers-shouldnt-fear-losing-jobs-to-ai-yet ; https://newatlas.com/technology/ai-gymnastics/ ; ManiVideo — https://arxiv.org/pdf/2412.16212 ; hand-object interaction generation — https://arxiv.org/pdf/2512.01677
15. Nightingale & Farid, AI-synthesized faces are indistinguishable from real faces and more trustworthy, PNAS 2022 — https://www.pnas.org/doi/10.1073/pnas.2120481119 (numbers via search excerpts; PMC mirror https://pmc.ncbi.nlm.nih.gov/articles/PMC8872790/ also proxy-blocked)
16. Miller et al., AI Hyperrealism, Psychological Science 2023 — https://journals.sagepub.com/doi/10.1177/09567976231207095 (proxy-blocked; numbers via search excerpts; open PDF record: https://discovery.ucl.ac.uk/id/eprint/10181988/) ; Psychology Today summary — https://www.psychologytoday.com/us/blog/urban-survival/202311/people-now-see-ai-generated-faces-as-more-real-than-human-ones
17. Training humans for synthetic face image detection, Frontiers in AI 2025 — https://www.frontiersin.org/journals/artificial-intelligence/articles/10.3389/frai.2025.1568267/full ; super-recognizer result — https://bpspsychub.onlinelibrary.wiley.com/doi/10.1111/bjop.70063
18. Compression laundering of forensic cues: Pay Less Attention to Deceptive Artifacts (OSN-compressed deepfakes) — https://arxiv.org/html/2506.20548v1 ; Social Network Compression Emulation — https://arxiv.org/html/2508.08765v1 ; Multi-domain awareness for compressed deepfake videos — https://www.sciencedirect.com/science/article/abs/pii/S107731422400153X
19. PhysVid: Physics Aware Local Conditioning for Generative Video Models — https://hf.co/papers/2603.26285 (abstract via HF paper_search)
