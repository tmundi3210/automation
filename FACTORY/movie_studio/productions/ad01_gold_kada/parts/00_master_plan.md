# Master Plan — `ad01_gold_kada`
### Instagram Reel Ad · Men's Yellow-Gold Kada · Genre: Classic-Luxurious (house genre)

**Format contract:** 9:16 vertical, muted-autoplay-first, built from 3–6 s clips stitched into one reel (owner's own format, honored).
**Register:** quiet luxury — texture, cut, craft, and controlled light over loud branding. Gold/brass reads as the *product*, not as flash. Restraint is the signal.
**Media class:** AI-generated photoreal video for a **real commercial product**. Binding honesty + Meta AI-disclosure discipline applies to every claim (see §Compliance and §Owner-Slots).

> **Ground-truth product (OBSERVED from owner's photo, not invented):** men's yellow-gold **KADA** (open cuff bangle), high-polish warm 22k-look gold, band ~8–10 mm wide. Main band carries a fine engraved **cross-hatch / wheat-grain texture** throwing a diamond-cut sparkle, framed by **two polished rails** top and bottom. Near the **terminals** (open ends) sits a decorative **openwork / engraved geometric motif panel** (temple / Greek-key style). Solid, masculine, classic.
> **The exact bracelet is carried by IMAGE REFERENCE (the owner's photo), never by text description alone.** Remote generation requires the owner to upload that photo through the **Higgsfield media-upload widget at run time** — the connector cannot read a chat attachment.
> **Chain:** the brief references a matching gold chain but supplied **no chain photo**. Any chain shot is an **OWNER-SLOT** (needs a reference image first). Do **not** invent a specific chain design. This plan keeps the chain out of the shot list until a reference exists.

---

## 1. Final Length & Rationale

**LENGTH: 22 seconds — 6 clips.**

| Clip | Beat | Seconds |
|---|---|---|
| 1 | Hook | 4.0 |
| 2 | Product reveal | 4.0 |
| 3 | Quality praise (band) | 4.0 |
| 4 | Quality praise (terminals) | 3.0 |
| 5 | Authenticity praise | 4.0 |
| 6 | CTA | 3.0 |
| | **Total** | **22.0** |

**Rationale (story KB):**
- **Clip = beat (CLIP_BEAT_MAPPING).** In the stitched-clip format the clip boundary *is* the beat boundary, so clip count budgets the story. The five ad beats (hook → reveal → quality → authenticity → CTA) plus a splittable quality beat map cleanly to 6 clips. Fewer than 5 clips can't carry all five beats with a distinct hook; more than 7 dilutes a single-product ad.
- **~22 s sits in the sweet spot** of the requested 15–30 s window: long enough for two product-macro hero beats (the craftsmanship *is* the sell) yet short enough to hold retention. Modulated durations, not a metronome — hero/macro beats get 4 s, the connective terminal macro and the CTA get 3 s.
- **Front-load the strongest image (HOOK_FIRST_TWO_SECONDS / RETENTION_CLIFF_EVIDENCE).** Instagram head Mosseri publicly frames the **first ~3 s** as where retention is won or lost, and says early-second retention is consistent until viewers make a sub-2-second swipe decision. So clip 1 opens on the single most arresting frame — the gold catching a diamond-cut flare — with **motion in frame one**, **no logo intro, no slow build**.
- **Hook↔payoff contract (VISUAL_PAYOFF_DELIVERY).** The hook promises *desirable, verifiable gold you can own*; the CTA payoff delivers *how to get it*. Written as one contract across both ends.
- **Ending mode: open loop (soft), not hard loop.** This is an ad optimizing for the **click/CTA**, not invisible rewatch (ENDING_MODE_SELECTION) — the reel ends on the offer, not a return to frame one.

---

## 2. Speech Approach

**DECISION: VOICEOVER-DRIVEN.** A classic-luxurious male **brand presenter** is shown in cutaways (mid-shots, wrist/forearm, an approving glance); the **product macros carry the hero beats**; the **VO carries the words**. No sustained tight talking-head lip-sync.

**Why:** tight talking-head lip-sync is a known AI-realism risk (mouth/teeth/phoneme sync is a frequent tell). Decoupling voice from a locked-on mouth lets us cut to product macros exactly when the claims land, and keeps the presenter in flattering, low-risk framings. It also serves the **silent-legibility gate** (SILENT_LEGIBILITY_GATE): feed video autoplays muted, so the story must read with sound off — the product hero shots + on-screen text carry meaning; the VO is the bonus for the minority who unmute, never a dependency.

**Alternative the owner may request:** a **talking-presenter** cut where the model speaks the lines to camera. Available on request, but it re-opens the lip-sync realism risk and would route extra iteration budget to the realism gate. Not the default.

**Muted-operability requirement:** every spoken claim is mirrored by an on-screen text beat or an unmistakable visual, so the ad works fully on mute.

---

## 3. Beat Sheet

| # | Beat | Seconds | On screen (muted-legible) | VO carries |
|---|---|---|---|---|
| **1** | **Hook** | 0.0–4.0 | Aspiration snapshot: presenter's forearm rotates to camera in warm low light; the kada throws a **diamond-cut flare** across the band. Motion in frame one. On-screen text hook (small, serif): *"Real gold. Real craft."* | Opening promise line |
| **2** | **Product reveal** | 4.0–8.0 | Cutaway to the classic-luxurious male, calm, looking down at the kada on his wrist / lifting it slightly to the light. Establishes the persona's world. | Names the piece: a men's gold kada |
| **3** | **Quality praise — band** | 8.0–12.0 | Extreme macro traveling along the main band: **cross-hatch wheat-grain texture**, **diamond-cut sparkle**, the **two polished rails**. No hands. | Praise of the cut/finish/craftsmanship |
| **4** | **Quality praise — terminals** | 12.0–15.0 | Rack-focus/continuation macro to the **terminal openwork temple/Greek-key motif panel**. The detail that says *classic, made-to-last*. | The signature motif, solidity |
| **5** | **Authenticity praise** | 15.0–19.0 | Presenter mid-shot; the kada rests on a soft-lit surface / stand beside him; he gives a single approving nod. On-screen text beat shows the **[OWNER-SLOT]** authenticity fact(s) as a clean lower-third. | Owner-verified authenticity claim (slotted) |
| **6** | **CTA** | 19.0–22.0 | Clean CTA card over the product at rest: **"Link on screen · in bio to buy"** + handle. Composited graphic text, not generated. | Call to action |

---

## 4. Shot List (6 shots) — transition-first

Seams are planned **before** the clips: N clips → 5 seams. **Whip pans and match cuts** dominate because their blur/graphic-continuity tolerate AI mismatch, and because the gold band is a natural match-cut anchor across the macro seams. **All six clips route to a both-frames end-frame-eligible model** (seams are bracketed) — never a start-only model on a fixed end frame. **Per-model credit cost is UNKNOWN on the read surface → every model×resolution×duration cell is `get_cost:true`-preflighted before any spend.**

Duration-band note: 3 s clips (4, 6) route to **`wan2_7`** (the only native 2–3 s model); 4 s clips route to a both-frames 4 s-capable model (e.g. `seedance1_5`, min 4 s). Confirm all against the live catalog at lock time.

---

**SHOT 1 — HOOK**
- **shot_id:** `ad01_s1_hook`
- **intent:** arrest the swipe; promise real, desirable gold in <2 s
- **shot_size:** tight — forearm/wrist, product-forward (persona present via cuff)
- **camera_move:** slow forearm rotation toward lens (motion in frame one); lens static
- **seconds:** 4.0
- **transition_in:** cold open on the flare frame (no build)
- **transition_out:** **whip-pan out** (forgiving seam)
- **on screen:** warm low-key light; tailored unbranded dark sleeve; kada rotates and throws a diamond-cut sparkle; on-screen serif text "Real gold. Real craft." in upper-safe zone
- **HARD-SHOT flags:** ⚠ *hands.* **Mitigation:** relaxed hand at frame edge, no finger dexterity/manipulation — just a slow forearm turn; keep fingers loosely out of frame. ⚠ *legible on-screen text.* **Mitigation:** text is a **composited overlay added in edit**, never generated in-clip.

**SHOT 2 — PRODUCT REVEAL / PERSONA**
- **shot_id:** `ad01_s2_reveal`
- **intent:** establish the classic-luxurious male presenter and his world
- **shot_size:** medium (chest-up)
- **camera_move:** slow push-in
- **seconds:** 4.0
- **transition_in:** **whip-pan in** (matches clip 1 out — same-direction blur)
- **transition_out:** **match cut on the gold** — push toward wrist; end frame dominated by the kada, handing off to the macro
- **on screen:** presenter looking down at the kada on his wrist, calm approving expression; warm-neutral grade, deep anchor tone in wardrobe; quiet-luxury styling, no visible logos
- **HARD-SHOT flags:** ⚠ *hands near face/wrist.* **Mitigation:** static, resting pose — no dexterous manipulation; wrist held, not fidgeted. Presenter mouth relaxed/near-neutral (VO-driven, no lip-sync lock).

**SHOT 3 — QUALITY MACRO (BAND)**
- **shot_id:** `ad01_s3_band_macro`
- **intent:** hero the craftsmanship — cross-hatch texture, diamond-cut sparkle, polished rails
- **shot_size:** extreme macro
- **camera_move:** slow lateral travel along the band (one move)
- **seconds:** 4.0
- **transition_in:** **match cut** — incoming frame is the same gold band that ended clip 2 (graphic continuity)
- **transition_out:** **invisible cut / rack focus** continuing along the band toward the terminal
- **on screen:** the band fills frame — engraved wheat-grain cross-hatch throwing shimmer, the two bright polished rails top and bottom; shallow DoF; controlled soft key with one specular roll to animate the sparkle
- **HARD-SHOT flags:** ⚠ *legible text risk* if any hallmark/stamp is fabricated. **Mitigation:** **do NOT generate any hallmark, karat stamp, or maker's mark** — keep macro on the plain textured band only; authenticity marks are owner facts, shown only if the owner supplies a real reference (see Owner-Slots). No hands in frame.

**SHOT 4 — QUALITY MACRO (TERMINALS)**
- **shot_id:** `ad01_s4_terminal_macro`
- **intent:** show the signature openwork temple/Greek-key motif — the "made-to-last" detail
- **shot_size:** extreme macro
- **camera_move:** rack focus settling on the terminal motif panel (continuation of clip 3's travel)
- **seconds:** 3.0
- **transition_in:** **invisible cut** from clip 3 (near-identical low-detail gold frame, focus pull)
- **transition_out:** **whip-pan out** back to the presenter
- **on screen:** the terminal openwork geometric panel in sharp focus, warm gold, deep negative space; the open cuff end reads clearly
- **HARD-SHOT flags:** ⚠ *fine geometric detail can garble.* **Mitigation:** this is the primary reason the exact piece must be carried by the **owner's reference photo**; worst-artifact gate on the still — any garbled/asymmetric motif forces a re-roll, not a ship. No hands.

**SHOT 5 — AUTHENTICITY**
- **shot_id:** `ad01_s5_authenticity`
- **intent:** land the owner-verified authenticity claim, premium and calm
- **shot_size:** medium
- **camera_move:** slow push-in / settle
- **seconds:** 4.0
- **transition_in:** **whip-pan in** (matches clip 4 out)
- **transition_out:** **soft match cut / clean cut** to the CTA card
- **on screen:** presenter beside the kada resting on a soft-lit stand or honed-marble surface; single approving nod; **on-screen lower-third text renders the [OWNER-SLOT] authenticity fact(s)** (e.g. purity/hallmark) — composited, only populated with TRUE owner values
- **HARD-SHOT flags:** ⚠ *authenticity implication.* **Mitigation:** if the owner supplies **no** hallmark/certification, this beat's text shows only claims the owner can stand behind (e.g. "owner-verified") and the ad **must not imply a hallmark that doesn't exist**. ⚠ *legible text* → composited overlay, not generated.

**SHOT 6 — CTA**
- **shot_id:** `ad01_s6_cta`
- **intent:** clean, unmissable call to action
- **shot_size:** medium/product card
- **camera_move:** static hold (let the card read)
- **seconds:** 3.0
- **transition_in:** **clean cut** from clip 5
- **transition_out:** end (soft open loop — ends on the offer)
- **on screen:** the kada at rest (on wrist relaxed or on stand); **CTA card: "Link on screen · in bio to buy"** + **[OWNER-SLOT: @handle]**; text held in Reels-safe zone
- **HARD-SHOT flags:** ⚠ *legible text is the whole shot.* **Mitigation:** CTA text and handle are **composited graphics in the editor**, never model-generated. Keep out of the bottom UI band and top-right safe zones.

---

## 5. VO Script

Tight, premium, unhurried. **Every factual/authenticity claim is an `[OWNER-SLOT]` the seller fills with a TRUE value.** If a slot is empty, the line is cut — never guessed.

- **Clip 1 (Hook):** "This is what real gold looks like."
- **Clip 2 (Reveal):** "A men's gold kada — solid, classic, made to be worn every day."
- **Clip 3 (Band):** "Look at the work. Hand-finished cross-hatch, diamond-cut to catch the light, framed in polished gold."
- **Clip 4 (Terminals):** "And the temple motif at the ends — the detail that tells you it was built to last."
- **Clip 5 (Authenticity):** "`[OWNER-SLOT: purity claim — e.g. 22k / stated karat]`, `[OWNER-SLOT: hallmark/certification — e.g. BIS-hallmarked — ONLY if true]`, authenticity verified by `[OWNER-SLOT: seller/brand name]`."
- **Clip 6 (CTA):** "Yours from `[OWNER-SLOT: seller/brand name]`. Link on screen, or in our bio, to buy."

> **Script honesty rule:** karat/purity, hallmark/BIS, weight, price, brand/city, and any guarantee are **not spoken unless the owner supplies the true value**. The presenter is framed as a **brand presenter showing craftsmanship (aspirational)** — never as "a verified happy customer" or a real testimonial.

---

## 6. Persona Brief (for the character department)

A **fictional AI brand presenter**: a classic-luxurious man, **early-40s to 50s**, warm mid-brown-to-tan skin, neat salt-and-pepper hair and close, groomed stubble, calm and quietly authoritative — old-money composure, never flashy. Wardrobe is **quiet luxury**: an unbranded well-cut navy or charcoal jacket over a fine open-collar shirt, no visible monograms or logos, one restrained metallic note (the gold kada is the only "flex"). He reads as someone who *owns* nice things, not someone selling them — understated, trustworthy, at ease. Lighting is warm, soft, controlled (no harsh direct key, no mixed color temps). He must be built with **consistent face + wardrobe** so all his cutaways read as one man in one world, and he is **an original AI persona — not a real person and not posing as a real customer**.

---

## 7. CTA Plan

- **Where:** on-screen **CTA card in clip 6** (composited graphic) **+ "in bio"** as the durable second path (link stickers/website field in the profile).
- **Copy:** *"Link on screen · in bio to buy"* + **[OWNER-SLOT: @handle]**. If a swipe-up/link sticker or profile link URL exists, that is **[OWNER-SLOT: purchase link URL]**.
- **Timing:** CTA card enters at **~19.0 s** and holds to **22.0 s** (full 3 s, unhurried read). A subtle text echo ("in bio") may ride the last 1 s of clip 5 to pre-load it.
- **Safe zones (Reels):** keep all CTA text out of the **bottom ~250 px** (caption / like / share UI) and the **top-right** (menu). Center-lower-third placement, generous negative space, serif or clean sans, ≤2 tones — quiet-luxury typography, no gradient/ornament.
- **Muted operability:** the CTA is fully legible with sound off (it is on-screen text, not a spoken-only line).
- **Clean export:** render **watermark-free** — leftover third-party/tool watermarks suppress Reels recommendation.
- **AI disclosure + caption:** publish with Meta's **"AI info" label toggled ON** (photoreal generated video is in the mandatory-disclosure class) and an **honest caption** written as a searchable sentence (caption is the discovery/SEO surface). Caption must not add authenticity claims beyond the owner-verified slots.

---

## 8. Owner-Slots — facts/assets the owner MUST supply before generation

**Authenticity / commercial facts (spoken + on-screen only if TRUE):**
- [ ] **Purity / karat** (e.g. 22k) — exact stated value
- [ ] **Hallmark / certification** (e.g. BIS hallmark) — *only if the piece actually carries one; if none, the ad omits it entirely*
- [ ] **Weight** (grams) — if it will be stated
- [ ] **Price** — if it will be stated
- [ ] **Seller / brand name**
- [ ] **City / location** — if stated
- [ ] **Guarantee / return / buy-back policy** — exact terms, if stated

**Distribution:**
- [ ] **Instagram handle (@)** for the CTA card and "in bio"
- [ ] **Purchase link URL** (bio link / product page), if a link path exists

**Reference images (blocking for faithful generation):**
- [ ] **The owner's KADA photo, uploaded via the Higgsfield media-upload widget at run time** — the exact bracelet is carried by this image reference; the connector cannot read a chat attachment. Blocking for clips 1–6.
- [ ] **A gold-chain reference photo** — *required before any chain shot can be generated.* No chain appears in this plan until a reference exists; do not invent a chain design.

**Consent / framing:**
- [ ] Confirm the presenter is presented as a **brand model showing craftsmanship**, not as a customer testimonial (already enforced in script/persona).

---

### Compliance summary (binding)
- No invented authenticity facts — karat, hallmark, weight, price, brand, city, guarantees are owner-slots.
- Presenter = original AI brand model, never a real person or a "verified happy customer."
- Publish with Meta **AI-info label ON** + honest, non-embellishing caption.
- Exact product carried by **owner's uploaded reference photo**; no fabricated hallmarks/stamps in macros; no invented chain.
- Per-model credit cost is UNKNOWN → `get_cost:true` preflight every cell before spend; lock the shot list against the eight lock conditions before any video call.
