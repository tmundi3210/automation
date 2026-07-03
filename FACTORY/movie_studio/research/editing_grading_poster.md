# Editing, Color Grading & Cover/Poster Selection for Stitched 3–6 s Reels

Scope: the timeline side of the owner's stitched-reel pipeline — pacing/sequencing/cut craft, post-side transition execution (paired with the cinematography file's shooting side), color grading incl. cross-AI-clip matching, cover/poster selection, a shot-analysis rubric for the director loop, the start-image/end-image frame-matching workflow, and mobile/vertical tool-form realities.

## 0. Sourcing note (read first)

- The egress proxy 403-blocked EVERY direct WebFetch attempted in this session, including official surfaces: `help.instagram.com` (Reels specs page), `creators.instagram.com` (trial-reels announcement), `higgsfield.ai/blog`, plus adobe.com, studiobinder.com, hootsuite.com, premiumbeat.com, nofilmschool.com. [FACT — observed 403s in this session]
- Therefore this file uses a downgraded tag, **[FACT/excerpt]** = the claim was returned verbatim/near-verbatim in search-engine excerpts of the cited source, but the source page itself could not be fetched. Treat as one notch below [FACT]. [ESTIMATE] = professional craft consensus or inference, basis + confidence stated. [UNKNOWN] = named blocker.
- Higgsfield-side capabilities (which models expose start/end-frame, native resolutions, etc.) are owned by the MCP-surface research file, not this one; where this file depends on them it says so and tags [UNKNOWN here].

## 1. Stitched-reel editing craft

### 1.1 Pacing math

- Feature-film baseline: average shot length (ASL) in commercial cinema fell from ~8–11 s (pre-1960) to ~4–6 s in recent decades — Bordwell's "intensified continuity"; Barry Salt measured a drop from ~11 s to ~7 s over 1958–1975. [FACT/excerpt — Bordwell "Intensified Continuity" PDF + Cinemetrics discussions, sources 1–2]
- Reels run faster than cinema: the owner's own format (3/4/5/6 s clips stitched) sits at roughly cinema-trailer/music-video cutting rate. With 3–6 s clips, a 15 s reel = 3–5 shots; a 30 s reel = 6–9 shots. [FACT — arithmetic on the owner's stated clip lengths]
- Retention economics: up to ~50% of viewers drop off within the first 3 seconds; watch-time (total seconds, % completed, rewatches) is treated as the dominant ranking signal; creator-analytics guides claim reels with 3-second hold above ~60% strongly outperform those below ~40%. [FACT/excerpt on the 3-second-drop-off and watch-time emphasis, sources 3–4; the "5–10x reach" multiplier is a third-party analytics claim — treat as [ESTIMATE], medium-low confidence, not a Meta-published number]
- Practical pacing rule for this pipeline [ESTIMATE — derived from the above, high confidence]: put the single strongest image FIRST (no logo/intro frame); front-load a pattern interrupt inside clip 1; a complete short reel with high retention beats a longer reel with low retention ("a 10 s reel at 80% retention beats a 60 s reel at 30%" — creator-guide heuristic, [FACT/excerpt], source 4).
- Shot-length modulation inside the reel [ESTIMATE — standard editorial craft, high confidence]: don't cut at a metronome. Hold the hero/luxury reveal longest (4–6 s), use 2–3 s connective shots between reveals; accelerating clip lengths toward a drop builds tension, decelerating after it gives "arrival."

### 1.2 Cut-on-beat practice

- Beat-sync = aligning cuts/transitions/visual changes to beats of the track; when a cut lands on a drum hit or bass drop the edit reads as intentional and rhythmic, and audio+visual are processed as one coherent stream. [FACT/excerpt — beat-sync editing guides, source 5]
- Do NOT cut on every beat — that reads frantic; cut major shot changes on downbeats/accents, let minor motion (speed ramps, camera lands) carry off-beats. [FACT/excerpt — same guides; also standard music-video craft]
- Mechanics: mark beats first (tap-markers in CapCut/VN, waveform spikes in Premiere/Resolve, or automatic beat detection), then conform cuts to markers — audio locked before picture. [FACT/excerpt — source 5]
- A frequently repeated claim that "Facebook found videos with strong audio-visual sync see up to 40% higher completion" circulates in these guides; the original Meta study could not be located or fetched. [UNKNOWN provenance — do not cite the 40% as fact]

### 1.3 Sequencing logic (shot-size rhythm)

- Film grammar baseline: coverage is planned wide → medium → close; wide establishes geography, medium carries action/relationship, close-up delivers emotion; alternating sizes creates the scene's rhythm — quick cuts between close-ups and action build intensity, steadier wides/mediums calm. [FACT/excerpt — shot-grammar guides, sources 6–7]
- Two adjacent shots should differ meaningfully in size or angle (the classic "30-degree/two-step" avoidance of jump cuts) [ESTIMATE — canonical continuity craft, high confidence]. For a 5-shot luxury reel a proven skeleton [ESTIMATE, medium-high confidence]: (1) close/detail hook (watch clasp, steam off espresso, heel on marble), (2) wide establishing the place, (3) medium of persona in motion, (4) second detail or reaction close-up, (5) wide or loop-back shot that lands the caption/CTA.
- Walter Murch's Rule of Six — the priority order for any cut: Emotion 51%, Story 23%, Rhythm 10%, Eye-trace 7%, Two-dimensional plane of screen 5%, Three-dimensional space of action 4%; sacrifice from the bottom up, never give up emotion. [FACT/excerpt — Murch, *In the Blink of an Eye*, as summarized by StudioBinder/NoFilmSchool, sources 8–9]
- Eye-trace applied to reels [ESTIMATE — direct application of Murch #4, high confidence]: because viewers can't rescan a 3 s shot, place the next shot's subject where the previous shot left the viewer's eye (e.g., persona exits frame right at eye-level → next clip's car enters at that same screen position).

### 1.4 J-cuts / L-cuts (sound bridging)

- J-cut: audio of the NEXT clip starts before its picture (audio leads). L-cut: audio of the CURRENT clip continues over the next clip's picture (audio trails). Both are "split edits"/sound bridges; named for the clip shape on the timeline. [FACT/excerpt — Adobe/Epidemic Sound/StudioBinder glossaries, sources 10–12]
- Function: staggering the audio and picture change makes the transition smoother and more naturalistic than a straight cut. [FACT/excerpt — same sources]
- Reels application [ESTIMATE — high confidence]: let the SFX of the incoming scene (waves, engine, champagne pour) start 8–15 frames before its picture (J-cut) — the ear pre-sells the location change, so the eye accepts the hard cut; conversely trail the music swell of a hero shot over the first frames of the next (L-cut) to glue a montage. Since AI-generated clips arrive without production audio, ALL sound bridging is constructed in post from the soundtrack/SFX layer — this is a pure timeline decision in this pipeline. [FACT — structural property of the AI pipeline]

### 1.5 Speed ramping

- Speed ramping / time remapping = gradually varying playback speed inside one clip via a keyframed speed curve; typical use: normal speed → extreme slow-mo at the action peak → accelerate out, giving the moment maximum visual weight. [FACT/excerpt — Adobe/Riverside/Kapwing guides, sources 13–14]
- Smoothness depends on source frame rate: more captured frames = smoother slow-mo. [FACT/excerpt — source 13] For AI clips this maps to generated fps: slowing a 24/25/30 fps AI clip below ~50–60% needs optical-flow frame interpolation and can expose AI morphing artifacts. [ESTIMATE — inference from interpolation mechanics, medium confidence; verify per-model fps on the MCP surface — [UNKNOWN here]]
- Ramp INTO cuts [ESTIMATE — craft, high confidence]: a short speed-up in the last 5–10 frames of the outgoing clip plus a matching decelerating start on the incoming clip mimics whip energy and hides the seam; pairs with §2.1.

### 1.6 Loop construction

- Seamless loop = the last frame flows into the first so viewers don't register the restart; loops raise rewatches and total watch time, which the ranking system rewards. [FACT/excerpt — loop-editing guides, source 15]
- Frame-matching is the core technique: line up first frame against last frame, zoom 200–400% to check pixel-level continuity, trim by single frames until they match; fallback: duplicate the first ~1 s at the tail and hide the joint with a 0.5–1 s cross-dissolve. [FACT/excerpt — source 15]
- AI-native loop trick: with start/end-frame conditioned generation, submitting the SAME image as both start and end frame produces a natural loop (documented by Kling: "use the same image for start and end to auto-create a smooth loop"). [FACT/excerpt — Kling/Higgsfield-blog excerpts, sources 16–17; availability of this mode inside the connected Higgsfield MCP = [UNKNOWN here], owned by the MCP-surface file]
- Loop the SENTENCE, not just the pixels [ESTIMATE — craft, medium-high confidence]: the caption/voice line should also read continuously across the restart (end mid-motion, not on a closed gesture).

## 2. Transition execution in post (timeline side; shooting side owned by cinematography file)

### 2.1 Matching motion vectors

- The seam between two clips disappears when motion direction, speed, and screen-position of the dominant moving element match across the cut; whip transitions "work best when you whip the pan in the same direction … the same direction of movement in the following shot," and matching color/brightness at the cut point further hides it. [FACT/excerpt — whip-pan guides, sources 18–19]
- Timeline checklist per transition [ESTIMATE — operationalization, high confidence]: (a) same vector (L→R out = L→R in), (b) comparable pixel velocity at the cut (adjust with speed ramp §1.5), (c) subject occupies matching screen region (nudge with scale/position keyframes — safe within ~110% on 1080×1920 before softness shows), (d) luminance/hue at the two boundary frames within a close range (fix with a per-clip trim grade §3.5).

### 2.2 Whip blur bridging

- Post recipe: cut the outgoing clip inside its blur-out and the incoming clip inside its blur-in and butt them together; the matched motion blur disguises the edit point, selling one continuous camera move between unrelated locations. [FACT/excerpt — sources 18–19]
- When the AI clips lack real blur (AI whips often render too clean), add 2–4 frames of directional blur + a subtle 5–10% zoom ramp on both sides of the cut in the editor. [ESTIMATE — standard editor-side whip-faking craft, medium-high confidence]

### 2.3 Masked wipes (foreground wipes)

- A foreground/masked wipe uses a passing object's or person's silhouette to wipe the frame and reveal the next scene; executed by keyframing a mask along the occluder's edge, with the second clip on the track beneath — organic, non-jarring scene change. [FACT/excerpt — wipe-transition guides + Premiere mask-wipe tutorials, sources 20–21]
- AI-pipeline application [ESTIMATE — high confidence]: prompt/shoot clips where something crosses full-frame (pillar, car, waiter's tray, persona walking past lens — the cinematography file owns getting these) and let the editor cut inside the 100%-occluded frames — when the frame is fully covered, no mask keyframing is even needed (it degenerates to an invisible cut).

### 2.4 Flash frames & invisible cuts

- Invisible cut: make the outgoing clip's last frame and the incoming clip's first frame identical — commonly by both going to full black (lens covered, wall pass, dark doorway) so the cut point cannot be located. [FACT/excerpt — source 19]
- Flash frame (white/overexposed 1–3-frame insert at the cut, often with a camera-flash SFX) is the high-energy inverse: it masks the discontinuity and adds paparazzi/red-carpet connotation that suits the luxury genre; keep to ≤2 per reel or it reads as strobing. [ESTIMATE — established music-video/reel craft; no single citable spec; medium confidence. Accessibility note: rapid full-frame flashing can trigger photosensitivity — keep flashes sparse and short, [ESTIMATE, high confidence]]
- Transition inventory for the director's shot-plan (timeline side): hard cut on beat (default, cheapest, most "cinema"), J/L sound bridge, whip bridge, masked wipe, invisible-cut via black/occlusion, flash frame, speed-ramped cut, match-cut on shape/motion (glass rim → wheel rim). Match cuts join two shots via matching composition/action and are the most "premium"-reading transition when the two frames are designed for it (§6). [FACT/excerpt on match-cut definition — source 22; inventory composition itself [ESTIMATE]]

## 3. Color grading

### 3.1 Log vs Rec.709 (concepts)

- Log encodings preserve more tonal information in shadows/midtones/highlights than display-referred Rec.709; log footage looks flat/desaturated and is a grading canvas; Rec.709 is the standard SDR delivery color space. [FACT/excerpt — log/Rec.709 explainers, sources 23–24]
- Workflow order (industry-standard): (1) technical normalization first — log→Rec.709 conversion LUT or color-space transform so decisions are made on an accurate image; (2) primary corrections (exposure, white balance, contrast); (3) secondaries (skin isolation, masks/windows); (4) creative look/LUT LAST in the chain. [FACT/excerpt — sources 23–24]
- AI-pipeline reality: Higgsfield/AI video models deliver display-referred (Rec.709-ish) files, not log [ESTIMATE — typical of consumer AI video delivery, medium confidence; exact delivery specs = [UNKNOWN here], MCP-surface file]. So this pipeline's "normalization" step is not log conversion but per-clip balancing to a hero reference (§3.5) — same chain logic, different first step.

### 3.2 Teal-orange and its luxury variants

- Teal-orange: orange and teal are complements with maximal exposure-value contrast among complementary pairs; skin tones sit in the orange range, so pushing shadows/backgrounds teal makes subjects pop; the look dominates 2000s+ blockbusters and partly replicates golden-hour (warm light vs blue sky). [FACT/excerpt — Adobe/PetaPixel explainers, sources 25–26]
- Luxury variants (the classic-luxurious brief should NOT run full blockbuster teal-orange):
  - **Warm-neutral filmic / "old money"**: warm, muted, timeless palette — cream, beige, camel, navy, sage, taupe, tobacco; nothing neon; warm practical light (~2700–3000 K look) reads richer than cool light. [FACT/excerpt — old-money-aesthetic style guides, source 27; palette mapping to grade controls is [ESTIMATE]]
  - Grade translation [ESTIMATE — colorist craft, medium-high confidence]: keep skin warm-natural, drift shadows only slightly cool (a whisper of teal, not a wall), desaturate global chroma ~10–20% below default, roll off (soften/mute) highlights rather than letting whites clip — muted highlights are the strongest single "expensive film" cue; gentle S-curve with lifted-but-not-milky blacks.
  - Fitness-accent posts can push slightly more contrast/saturation than the core grade, but stay inside the same palette family so the grid remains coherent (§4.2). [ESTIMATE — brand-consistency logic, high confidence]

### 3.3 Skin-tone protection

- Skin tones are the reference the audience calibrates "real" against; in vectorscope terms skin hue clusters along the skin-tone indicator line, and colorists refine skin on the vectorscope after balancing luminance (waveform) and white balance (RGB parade). [FACT/excerpt — Resolve shot-matching guides, source 28]
- Practice for this pipeline [ESTIMATE — standard colorist craft, high confidence]: qualify/mask skin before applying the creative look, or apply the look and then key skin back toward its corrected hue/saturation; never let a teal shadow push or heavy desaturation land on faces — gray or cyan-tinged skin is an instant "AI/cheap filter" tell, which directly violates the owner's #1 "must not look AI" requirement. Both personas' skin tones must land on the SAME target values across every clip and every post — skin consistency IS character consistency at the grade layer.

### 3.4 LUT concept

- A LUT (Look-Up Table) is a stored transform mapping input color values to output values; two roles: technical LUTs (e.g., log→Rec.709 normalization, applied first) and creative LUTs (looks, applied last, intensity-controllable). [FACT/excerpt — sources 23–24]
- Pipeline use [ESTIMATE — high confidence]: bake the approved "house luxury look" (§3.2) as ONE creative LUT/preset applied to every clip AFTER per-clip balancing; never use a LUT as a substitute for per-clip correction (LUTs are dumb transforms — applied to unbalanced clips they amplify mismatch).

### 3.5 Matching grade ACROSS AI clips — the real problem here

- Named problem: clips from different generation batches (and different models) arrive with slightly different color temperature, contrast, and saturation even when content is consistent; a unified color grade applied in post is "the single most effective way to make disparate clips feel like they belong to the same video." [FACT/excerpt — AI-video consistency guides, sources 29–30]
- AI adds intra-clip failure modes classic footage doesn't have: temporal drift (colors/details wandering within a clip) and flicker; mitigations cited: generate short 3–5 s segments (already the owner's format), keep lighting prompts simple and consistent ("3–5 lighting adjectives"; e.g., "soft, diffused, consistent morning light" is more stable than a complex description), and use a master reference image via image-to-video to anchor light direction/specularity. [FACT/excerpt — sources 29–31; note these are vendor/practitioner guides, not peer-reviewed]
- Shot-matching procedure (adapted from professional Resolve practice, [FACT/excerpt] on the base method — source 28; adaptation [ESTIMATE, high confidence]):
  1. Normalize all clips into one working space/timeline color management.
  2. Declare a **hero clip** per reel (best skin + best exposure).
  3. Match every other clip to the hero: luminance on the waveform → white balance on the RGB parade → skin on the vectorscope; automated "shot match to this clip" is a starting point only, always refine by eye+scopes.
  4. Group clips (per-scene or per-location groups) so shared corrections propagate.
  5. Apply the house creative LUT (§3.4) globally on top.
  6. QC pass: step through every cut boundary; the two frames either side of each cut must sit within a tight luminance/hue window (this doubles as the §2.1(d) transition check).
- Cross-MODEL matching (Higgsfield exposes multiple video models): expect systematic per-model color signatures; maintain a small per-model trim preset (offset/temp/sat nudge toward the house target) learned from the first batches, applied before the hero-match step. [ESTIMATE — process design grounded in the per-model-signature observation above, medium confidence; actual per-model signatures = [UNKNOWN until generation runs]]

### 3.6 IG compression-safe grading

- Instagram re-encodes uploads; compression on already-compressed sources produces blockiness and banding; practitioner guidance: deliver 1080-wide (upscaling beyond gives no benefit and raises compression risk), ~3,500–5,000 kbps cited for Reels-length content, Rec.709/SDR delivery, enable "Upload at highest quality," upload from the app rather than third-party re-encoders. [FACT/excerpt — compression-fix guides, source 32; the exact bitrate window is third-party advice, not Meta-published — hold as [ESTIMATE, medium confidence]]
- Banding mechanics: 8-bit = 256 levels/channel, insufficient for smooth gradients (skies, luxury-interior walls, vignettes); social delivery is 8-bit 4:2:0, so banding risk is structural. Countermeasures with citable basis: grade/render in higher bit depth as long as possible; add fine dither/film grain (~1–2%) to break up smooth areas; use a deband filter (e.g., Resolve's Deband) on affected regions. [FACT/excerpt — banding/debanding guides, sources 33–34]
- Crushed-black protection [ESTIMATE — colorist craft + codec behavior, medium-high confidence]: don't grade shadow detail right at black — heavy encoders crush near-black macroblocks first; keep intentional shadow detail a few code values above 0 and check the graded reel ON A PHONE after an actual test upload, not just in the NLE. Fine film grain also helps the encoder hold shadow texture, but very heavy grain wastes bitrate and can smear under re-encode — keep it subtle.
- Bonus alignment: subtle grain simultaneously fights banding AND is a "shot on film, not AI" cue — one knob serving the owner's #1 requirement and delivery QC. [ESTIMATE — inference, medium-high confidence]

## 4. Poster / cover selection

### 4.1 What makes a strong reel cover

- Face presence: thumbnails with faces out-click faceless ones; close-up faces with eye contact perform best; expressive emotion (surprise/excitement/curiosity) beats neutral (one aggregator claims ~25% higher CTR for expressive vs neutral). [FACT/excerpt — thumbnail-psychology roundups incl. Search Engine Journal, sources 35–36; the 25% figure is an industry-aggregator number, hold as [ESTIMATE, medium-low confidence]]
- Readability at thumbnail size: big central elements; text 3–5 words max (fine-at-fullsize sentences die at ~150 px grid width); bold fonts, never thin/script; high contrast that survives both dark and light app modes. [FACT/excerpt — reel-cover guides, sources 37–38]
- Crop safety: covers are cropped differently in feed, profile grid, and Explore; keep the hook imagery and any text inside the CENTER SQUARE of the 1080×1920 cover so every crop keeps it. [FACT/excerpt — sources 37–38]
- Custom uploaded covers vs picked video frames: guides consistently favor a designed/selected deliberate cover over a random autoframe because it states what the reel is about; IG supports both (frame-scrub or upload from camera roll). [FACT/excerpt — source 37]
- Curiosity mechanics [ESTIMATE — synthesis of thumbnail psychology, medium-high confidence]: the cover should pose an unresolved visual question (mid-action, mid-reveal, gaze off-frame toward something unseen) rather than summarize the reel; in the luxury genre the "question" is usually access ("where is this? who lives like this?").

### 4.2 Grid coherence for a luxury account

- Consistent cover templates/design language across reels make the profile grid cohesive and the brand recognizable — the standard mechanism is a repeatable cover template (same type placement, same grade, same margin logic). [FACT/excerpt — source 37]
- Luxury-specific translation [ESTIMATE — brand-design craft, medium-high confidence]: coherence carries the "classic luxurious" positioning harder than any single cover — one house grade (§3.2) on all cover frames, one serif/typographic system, generous negative space, alternating persona/place/detail covers in a deliberate cadence; avoid loud text-heavy covers that read as clickbait — in this genre restraint IS the click driver. The male and female personas should be instantly distinguishable at grid glance but share the identical grade and template.

### 4.3 A/B practice

- Native mechanism: **Trial Reels** (announced Dec 10, 2024) show a reel to NON-followers first; after ~24 h you get views/likes/comments/shares to judge it, then share to followers manually or auto-share on performance; available to public accounts ≥1,000 followers; Instagram-reported stats: 40% of creators who try trials post more reels, and 80% of those see increased non-follower reach; Mosseri framing: experiment "without worrying how followers might react." [FACT/excerpt — official announcement is on creators.instagram.com but that page 403-blocked; figures reproduced across multiple secondary writeups, sources 39–40 — hold specifics one notch down]
- Trial reels test the whole reel (hook+content), not the cover in isolation; there is no native organic cover-only A/B. Cover iteration is therefore observational: hold everything else constant across similar posts, vary one cover variable (face vs detail, text vs none), read profile-grid-sourced views in analytics. [ESTIMATE — inference from feature scope, medium-high confidence; exact analytics breakdowns available per-account = [UNKNOWN until the account exists]]

## 5. Shot/image analysis rubric (for the director feedback loop)

Scoring card the director loop applies to every generated still/clip frame before it enters the timeline. Weights [ESTIMATE — designed here from the cited craft; calibrate against real batches]. Score each 0–2 (0 fail, 1 acceptable, 2 strong); gate at total ≥12/18 AND no 0 in starred rows.

| # | Axis | What to check | Grounding |
|---|------|---------------|-----------|
| 1* | Exposure | Skin at correct level on waveform; highlights rolled, not clipped; blacks not crushed (§3.6) | scopes practice [FACT/excerpt, source 28] |
| 2 | Composition | Subject on thirds intersections or deliberate center; leading lines resolve INTO the subject; horizon level; headroom/looking-room correct | classical composition [ESTIMATE — canonical craft, high confidence] |
| 3 | Color harmony | Frame palette inside the house luxury palette (§3.2); no stray saturated accent hue; skin on the skin-tone line | vectorscope practice [FACT/excerpt, source 28] |
| 4* | Sharpness distribution | Sharpest point = intended subject (usually eyes); background softer than subject; NO uniform everything-sharp look (an AI tell) and no AI mush zones (hands, text, patterns) | AI-artifact QC [FACT/excerpt that uniform/warped detail reads AI, source 31; distribution rule [ESTIMATE]] |
| 5 | Subject separation | Subject pops via tonal contrast, hue contrast (warm skin vs cooler field — the teal-orange mechanism §3.2), or depth of field | complementary-separation mechanism [FACT/excerpt, sources 25–26] |
| 6* | Physics/realism | Shadows agree with one light direction; reflections/specular highlights plausible; motion blur consistent with motion; cloth/hair/liquid behave | owner requirement #1; artifact taxonomy owned by the prompting file [UNKNOWN here — cross-file] |
| 7 | Emotional read | The frame communicates ONE clear feeling; expression readable at thumbnail size | Murch criterion 1 [FACT/excerpt, source 8]; thumbnail emotion [FACT/excerpt, source 35] |
| 8 | Continuity fit | Matches neighbors: persona identity/wardrobe, light direction, grade family, motion vector at the cut boundary (§2.1) | shot-matching practice [FACT/excerpt, source 28] |
| 9 | Genre fit | Reads classic-luxurious (restraint, materials, muted highlights) not gauche/new-money loud | genre codes owned by the luxury-codes file; palette basis [FACT/excerpt, source 27] |

Rows 1, 4, 6 starred: a zero on any = automatic regenerate, because each is an irrecoverable-in-post failure (clipped data, AI-tell texture, broken physics). [ESTIMATE — triage design, high confidence]

## 6. Start-image + end-image workflow (frame-matching feeding the transition plan)

- Capability basis: start/end-frame-conditioned generation ("Start & End Frames" on Kling, "Keyframes" on Runway Gen-3 — up to 3 images on Turbo, "Keyframes" on Luma) takes two uploaded images and generates the motion between them; same-image-both-ends yields a loop. [FACT/excerpt — Kling docs/fal/Runway help excerpts + a Higgsfield blog post "Kling is King: Start & End Frames" exists on higgsfield.ai (fetch blocked), sources 16–17, 41; which Higgsfield MCP models expose this parameter = [UNKNOWN here], MCP-surface file owns it]
- Why this is the transition machine of the whole pipeline [ESTIMATE — design synthesis, high confidence]: the reel is planned as a CHAIN of stills first. For clips i and i+1, clip i's END image and clip i+1's START image are designed as a PAIR according to the chosen transition from §2's inventory. The editor then inherits cuts that already match.
- Frame-pairing rules per transition type [ESTIMATE — operationalization of §2 + match-cut craft, medium-high confidence]:
  1. **Hard cut / match cut**: pair frames matched on dominant shape and screen position (round glass → round wheel; persona silhouette same size/position in both) with a deliberate size jump (close→wide) so it doesn't read as a jump cut.
  2. **Whip bridge**: end image = subject exiting with strong lateral vector; start image = matching vector entering; blur added or generated at both boundaries.
  3. **Masked wipe / invisible cut**: end image = frame fully or nearly occluded (dark doorway, passing object); start image = same occlusion state in the new scene.
  4. **Loop**: reel's last clip's end image ≈ first clip's start image (§1.6).
  5. All pairs: same house grade target, same skin values, luminance of the two frames within a narrow window (§3.5 QC) — pairs failing the §5 rubric on either frame are regenerated BEFORE video generation, which is the cheap place to fail. [FACT — cost ordering: stills are cheaper than video on credit-based platforms as a category; exact Higgsfield credit prices [UNKNOWN here]]
- Constraint to respect: end-frame conditioning controls the boundary, not the path — mid-clip motion can still wander/morph; keep clips 3–5 s (already the format) and prompt motion simply, per the consistency guidance in §3.5. [FACT/excerpt on short-segment stability — source 29]

## 7. Tool-form notes: mobile-first vertical realities

- Canvas: Reels are 9:16, 1080×1920. [FACT/excerpt — universally consistent across specs guides, sources 42–43; official help.instagram.com page 403-blocked]
- Safe zones (UI overlay regions on 1080×1920): commonly published figures — keep ~108–220 px at top clear (camera/audio UI), ~320 px at bottom (caption, audio attribution, profile/follow), ~60 px left, ~110–120 px right (like/comment/share/save stack); i.e., a usable center of roughly 900×1400–1500 px. The right-side button stack and bottom-left caption zone are the two most-covered regions. [FACT/excerpt — multiple 2025–2026 safe-zone guides converge on these ranges but disagree in exact pixels (e.g., 108 vs 220 top), sources 42–44 — treat exact values as [ESTIMATE, medium confidence]; no fetchable official Meta pixel spec found → [UNKNOWN: Meta's own current safe-zone pixel numbers]]
- Practical layout rules [ESTIMATE — direct application, high confidence]: captions/quote text lives in the center band (upper-middle third), never bottom 320 px or right 120 px; the persona's face should sit in the middle 50% vertically so neither feed crop (4:5 in feed) nor grid crop (center square/4:5) beheads it; the cover's text must survive the center-square crop (§4.1).
- Mobile editing reality [ESTIMATE — tooling landscape, medium confidence]: beat-marking and quick assembly are strongest in mobile-first editors (CapCut/VN tap-markers [FACT/excerpt — source 5]); scope-based shot matching (§3.5) needs a desktop grader (Resolve free tier includes scopes, groups, shot-match). A hybrid is the realistic pipeline: assemble/beat-lock mobile-or-desktop → grade/match in Resolve → export master → upload from the IG app with "highest quality" on (§3.6).
- Watch context: significant reel consumption is sound-off at first touch; open with text-readable, motion-legible imagery, never audio-dependent hooks. [FACT/excerpt — creator-guidance summaries, source 4; exact sound-off share numbers vary by study and none was fetchable → no number claimed]

## Cross-file handoffs

- Shooting-side transition requirements (what to generate so §2 has material) → cinematography file.
- AI artifact taxonomy behind rubric row 6 → prompting/QC file. Persona/wardrobe consistency behind rubric row 8 → character file.
- Which Higgsfield models expose start/end-frame, fps, delivery specs, credit costs → MCP-surface file. Posting cadence/analytics loop around §4.3 → media-manager file. Music/beat source for §1.2 → soundtrack file.

## Sources

All URLs below were surfaced via WebSearch; direct WebFetch of every attempted page returned HTTP 403 through the session proxy (noted in §0), so citations rest on search-result excerpts of these pages.

1. Bordwell, "Intensified Continuity" (PDF mirror) — https://cinecdoque.wordpress.com/wp-content/uploads/2015/03/bordwell-intensified-continuity.pdf
2. Cutting-rates study, Wide Screen journal — https://widescreenjournal.org/wp-content/uploads/2022/08/formatted-cutting-rates.pdf
3. Inro, "Instagram Reels First 3 Seconds Hook" — https://www.inro.social/blog/instagram-reels-3-second-hook-leads
4. OpusClip, "Instagram Reels Hook Formulas That Drive 3-Second Holds" — https://www.opus.pro/blog/instagram-reels-hook-formulas
5. Bitcut, "Beat Sync Video Editing" — https://bitcut.app/blog/beat-sync-video-editing
6. Cadrage, "The 7 most common shot sizes" — https://www.cadrage.app/the-7-most-common-shot-sizes-in-filmmaking/
7. StudioBinder, "50+ Types of Camera Shots" — https://www.studiobinder.com/blog/ultimate-guide-to-camera-shots/
8. StudioBinder, "The Rule of Six — Walter Murch" — https://www.studiobinder.com/blog/walter-murch-rule-of-six/
9. No Film School, "6 'Rules' for Good Cutting According to Walter Murch" — https://nofilmschool.com/2016/11/6-rules-good-cutting-according-oscar-winning-editor-walter-murch
10. Adobe, "What is an L cut and J cut in film?" — https://www.adobe.com/creativecloud/video/post-production/cuts-in-film/l-and-j-cut.html
11. Epidemic Sound, "J-cuts vs. L-cuts" — https://www.epidemicsound.com/blog/j-cuts-and-l-cuts/
12. StudioBinder, "What is a Sound Bridge in Film" — https://www.studiobinder.com/blog/what-is-a-sound-bridge-definition/
13. Riverside, "Speed Ramping: What It Is & Practical Tips" — https://riverside.com/video-editor/video-editing-glossary/speed-ramping
14. Adobe, "Create action speed ramps with Premiere" — https://www.adobe.com/creativecloud/video/hub/guides/premiere-pro-speed-ramp.html
15. TechPorn, "Trimming Tricks to Make Seamless Loop Videos" — https://www.techporn.ph/trimming-tricks-to-make-seamless-loop-videos/
16. Kling AI, "Start and End Frames" (quickstart) — https://kling.ai/quickstart/ai-video-start-end-frames
17. Higgsfield blog, "Kling is King: Start & End Frames" — https://higgsfield.ai/blog/Kling-Start-End-Frames
18. No Film School, "Follow These 5 Steps to Create a Perfect Whip Pan Transition" — https://nofilmschool.com/how-to-do-a-whip-pan
19. DRIFF, "In-Camera Transitions: The Whip Cut" — https://www.driff.ca/diy-film-school/lesson1-the-whip-cut
20. StudioBinder, "What is a Wipe Transition in Film" — https://www.studiobinder.com/blog/what-is-a-wipe-transition-in-film/
21. 4K Shooters, "Slick Walk By Transition in Premiere Pro" — https://www.4kshooters.net/2017/06/28/thats-how-you-can-create-a-slick-walk-by-transition-in-adobe-premiere-pro-cc/
22. Backstage, "Match Cut: Video Editing Technique, Explained" — https://www.backstage.com/magazine/article/what-is-a-match-cut-75670/
23. Pixflow, "Differences Between RAW, LOG, and Rec 709" — https://pixflow.net/blog/difference-between-raw-log-and-rec-709-camera-footage/
24. AAA Presets, "Guide to Grading LOG Footage in Premiere Pro" — https://aaapresets.com/blogs/premiere-pro-color-grading-guide-pro-cinematic-workflow/unlocking-cinematic-potential-your-ultimate-guide-to-grading-log-footage-in-premiere-pro-s-log-c-log-v-log-in-2025
25. Adobe, "Why you should use orange and teal color grading" — https://www.adobe.com/creativecloud/video/hub/features/why-use-orange-teal-grading.html
26. PetaPixel, "What is the 'Orange & Teal Look'" — https://petapixel.com/2017/02/23/orange-teal-look-popular-hollywood/
27. Lovau/Vibesabi old-money palette guides — https://www.vibesabi.com/old-money-aesthetic/ ; https://lovau.com/blogs/news/old-money-aesthetic-for-men-classic-style-that-commands-respect-1
28. Colourist, "Shot Matching in DaVinci Resolve — Step by Step" — https://colourgrade.agency/learn-the-craft/shot-matching-in-davinci-resolve-step-by-step
29. PyxelJam, "Consistent Style Across Multiple AI-Generated Video Clips" — https://pyxeljam.com/how-to-achieve-consistent-style-across-multiple-ai-generated-video-clips/
30. Kling AI blog, "All You Need to Know about Drift in AI-generated Video" — https://kling.ai/blog/fix-ai-video-drift-consistency-guide
31. Genra, "Why Your AI Videos Look Fake: 7 Fixes" — https://genra.ai/blog/why-ai-videos-look-fake-how-to-fix
32. Watermarkly, "Why Instagram Ruins Photo & Video Quality" — https://watermarkly.com/blog/why-instagram-ruins-quality/
33. PremiumBeat, "Methods for Debanding Your Footage in DaVinci Resolve" — https://www.premiumbeat.com/blog/debanding-footage-davinci-resolve/
34. KTC, "Color Banding and Bit Depth Explained" — https://us.ktcplay.com/blogs/support-tips/color-banding-bit-depth-explained
35. Search Engine Journal, "Do Faces Help YouTube Thumbnails? What The Data Says" — https://www.searchenginejournal.com/do-faces-help-youtube-thumbnails-heres-what-the-data-says/563944/
36. ThumbnailTest, "Face in YouTube Thumbnail" — https://thumbnailtest.com/guides/face-in-youtube-thumbnail/
37. Hootsuite, "The ultimate Instagram Reels cover guide" — https://blog.hootsuite.com/instagram-reels-cover/
38. BrandForge, "Instagram Reels Cover Design" — https://brandforge.me/blog/instagram-reels-cover-design
39. Instagram for Creators (official; fetch 403-blocked), "Trial reels" — https://creators.instagram.com/blog/instagram-trial-reels
40. Metricool, "Instagram Trial Reels" — https://metricool.com/instagram-trial-reels/
41. Runway Help, "Creating with Keyframes on Gen-3" — https://help.runwayml.com/hc/en-us/articles/34170748696595-Creating-with-Keyframes-on-Gen-3
42. Kreatli, "Instagram Reels Safe Zone Guide" (fetch 403-blocked) — https://kreatli.com/guides/instagram-reels-safe-zone
43. CampaignSwift, "Instagram Safe Zone Sizes Guide 2026" — https://campaignswift.com/blog/instagram-safe-zone-sizes
44. Minta, "Instagram Safe Zone: Guidelines & Best Practices" — https://www.minta.ai/blog-post/instagram-safe-zone
