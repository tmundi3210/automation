# Quality Control & the "Thread-Checker" — Owner Deep-Dive (T5)

**For:** the owner of a small, AI-designed D2C acrylic/blend knitwear brand sourcing in Ludhiana and selling in California / Turlock.
**Question this answers:** What QC should a micro knitwear brand *actually* run — the yarn spec you put in a contract, the incoming inspection you do, the finished-garment tests you pay for? Is the owner's idea of a **device that grades thread-to-thread, records it, and sends a code back to the supplier** real, buildable, worth it — or premature? And where does **AI vision QC** genuinely earn its keep versus where is it hype?

**How to read the tags (binding honesty discipline):**
- **[FACT]** = cited public source / published standard.
- **[ESTIMATE]** = a reasoned estimate — method + basis + confidence stated; carries the literal word *estimate*. **Every price band below is an [ESTIMATE]**, never fake-precise.
- **[UNKNOWN]** = we genuinely don't know and won't fake it. Coverage is a **floor, not a ceiling.**

> **The one sentence to remember.** *The QC that grades yarn thread-to-thread already exists — it is six-figure, Uster-class capital that lives on your **supplier's** floor, so you **specify it in the purchase order and never own it**; the only QC a micro-brand realistically owns is a **human inspector checking incoming garments against an AQL plan**, plus a small **third-party lab spot-check budget** (~$30–250/lot); the owner's "thread-checker" is best understood as **a contract with a QR stamp, not a machine** — build only the cheap (~$0–5k) **grade-code-back-to-the-supplier loop**, and treat AI vision as a promising-but-unvalidated assistant, never a replacement for the inspector.* `[ESTIMATE: judgment, HIGH — two independent analysts reached this separately]`

---

## 1. The mental model: three places QC happens, only one of which you own

A sweater's quality is decided in three layers, and the brand's leverage is completely different at each. Confusing them is the single most expensive QC mistake a small brand makes.

| Layer | Where it happens | Who owns the machine | Your move |
|---|---|---|---|
| **Yarn quality** (evenness, imperfections, strength) | Spinner's lab + winder | **Spinner** (Uster-class capital) | **Demand it in the PO**; never own it |
| **Fabric / knit quality** (defects in the knitted panel) | Knitter / CMT floor | **Knitter** (inspection frame) | **Demand it**; explore cheap vision *later* |
| **Finished-garment quality** (measurements, seams, finish, labels) | Your incoming-goods table | **You** (a person + a tape + a light) | **OWN this** — it's your real QC |

The whole strategy below follows from this table: **sense-and-grade is the supplier's; you demand it. Finished-garment AQL is yours; you own it. Lab confirmation tests you buy.** Nothing about a 3-article capsule justifies owning a tester or building a device.

---

## 2. The contractual yarn spec — your QC starts in the PO, not on a machine

You don't test yarn evenness yourself. You **write the number into the purchase order** and make the spinner prove they hit it. This is the cheapest, highest-leverage QC you do, and it costs nothing but precision.

**Yarn is graded against USTER® STATISTICS** — a global percentile benchmark. "25% USTER" means the yarn is better than 75% of world production on that parameter; lower percentile = better. `[FACT — Uster Technologies methodology]`

**What to put in the PO (the numeric spec):**
- **CVm% (mass evenness)** and **IPI (Imperfection Index = thin −50% / thick +50% / neps +200% per 1000 m)** — demand **≤ 25% USTER** on both for a mainstream-to-premium acrylic/blend sweater. **Do not pay for 5% USTER on mass acrylic** — it's money for a fineness the customer never feels. `[FACT — parameter definitions, uster.com; ESTIMATE-MED for the 25% target being the right tier for our product]`
- **Hairiness (H/S3)** — relevant because too-high hairiness drives pilling (your #1 return risk); state a ceiling.
- **Tenacity / elongation** — minimum breaking strength so the yarn doesn't break in knitting (downtime) or in seams.
- **Count + count-CV** — the actual yarn count (Ne/Nm) plus a tolerance, so GSM and shade don't drift between batches.
- **Fibre composition** — the exact blend ratio (e.g. "70/30 acrylic/wool"). This double-duties as a **customs-duty-misdeclaration guard** — the care label and the HTS line must match what's actually in the yarn.

**The instrument behind these numbers (so you can talk to the spinner credibly):**
- **USTER® TESTER 6** — the global off-line lab evenness tester (capacitive + optical), measuring exactly CVm%, IPI, hairiness, count-CV, tenacity, graded against USTER STATISTICS. **Price: six-figure-USD class; exact list price [UNKNOWN]** (Uster doesn't publish), reconciled band **~$80k–400k+ [ESTIMATE, LOW confidence]**. You will never own one. `[FACT — uster.com TESTER 6 datasheets 2021–22]`
- **USTER® QUANTUM (4.0)** — the **in-line yarn clearer** on the spinner's winder: it checks the yarn **thread-to-thread, 100% inline, and records every fault**, cutting and re-splicing defects automatically, logging to the cloud. **This is literally "check the yarn thread-to-thread and record it," already productised — and it lives on the spinner's machine.** `[FACT — uster.com QUANTUM; Textile World 2021]`

> **Why this matters for the thread-checker idea:** the owner's instinct ("grade every bit of thread, record it") is *correct and already shipping* — as Uster Quantum. The realisation to internalise: **you don't build the part that senses and grades; you demand the spinner already runs it and hands you the certificate.**

**Honest caveat — can your spinner even produce this cert?** Large integrated groups (Vardhman, Nahar, Sportking) self-test and can hand you a USTER report. A **target Ludhiana micro-spinner may not own a tester at all** — in which case the cert comes from a third-party lab run *you* pay for, not the spinner. `[ESTIMATE, MED-HIGH that micro-spinners can't self-test — Open-Q #5]`

---

## 3. Incoming AQL inspection — the one QC you actually own

When the garments arrive, **a human inspects a statistical sample against a written defect standard.** This is near-zero capital — an inspector, a light box, a tape measure, and a written standard — and it is the brand's genuine, owned quality gate.

**AQL (Acceptable Quality Limit)** is the standard statistical sampling plan (ISO 2859-1 / ANSI-ASQ Z1.4): you pull a sample sized to the lot, count defects, and **accept or reject the whole batch** on the count. `[FACT — standard apparel QC]`

**Apparel default acceptance levels — use these:**
- **Critical 0** (zero tolerance — anything unsafe/unsellable)
- **Major 2.5** (defects a customer would return for)
- **Minor 4.0** (cosmetic flaws)
`[FACT — Tetra/QIMA/ASQ apparel defaults]`

**What the inspector checks (and the standards behind each):**
- **Measurements** against the tech pack, within tolerance (catches bad grading, rushed pressing).
- **Seam/linking quality** — is it truly **linked** (flat, loop-to-loop) or cheaper **mock-linked / overlocked**? (SCOPE_GLOSSARY §3 — this carries the quality-price story.)
- **Visual defects** — holes, dropped stitches, barré (streaks), shade mismatch, loose ends, float problems on jacquard backs.
- **Labels & care** — fibre composition matches the yarn cert and the duty declaration; care label present and correct.

**This step closes the loop the yarn spec opened:** the numbers you demanded in the PO (§2) and the lab tests you buy (§4) only have teeth if you actually **sample, grade, and reject** on arrival. QC without a willingness to reject a lot is theatre.

---

## 4. Finished-garment lab tests — the QC you buy (don't own)

Some acceptance tests need a lab. You **don't own the equipment** — you send samples to a third-party lab, per lot or per new style/colour, and write the pass thresholds into the PO so the supplier shares the risk.

| Test | What it catches | Threshold to demand |
|---|---|---|
| **Pilling grade (ICI Pilling Box, ISO 12945-1, scale 1–5)** | The #1 post-purchase return driver | **≥ 3–4 after ≥1 home wash** `[FACT — ISO 12945-1]` |
| **Dimensional stability / shrinkage %** | Garment changes size after customer washes it | Within tech-pack tolerance |
| **Spirality** | "Twisted side seam" (often from unbalanced singles twist) | Within tolerance |
| **Colour fastness (wash/rub/light)** | Colour bleeds or fades | Standard grades |
| **Fibre composition** | Mislabelled blend → returns *and* duty penalty | Matches label + HTS |
| **Yarn evenness spot-check (a discrete Uster run)** | Confirms the spinner's claimed CVm%/IPI | ≤ 25% USTER as specified |

**Cost of this recurring QC:** **~$30–250 per lot [ESTIMATE, MED]** depending on how many tests and which lab; a **discrete per-sample Uster evenness run near Ludhiana is [UNKNOWN]** and needs a real quote. **This is your actual recurring QC cost line — it feeds the T4 cost model.** `[FACT for the test standards; ESTIMATE for the cost band — compliancegate / India textile-lab norms]`

**Reality check on scale:** your yarn input is roughly **$1–2.50/garment**. A $30–250/lot spot-check is proportionate; a six-figure tester is not. The economics alone settle the own-vs-demand question.

---

## 5. The "thread-checker" — honest verdict

The owner's idea: **a device that grades thread-to-thread, records the grade, and sends a code back to the supplier.** It's a genuinely good instinct. Here's the honest decomposition — because the idea is really *four different things*, and they have opposite verdicts.

**The four layers:**
- **L1 — sense the yarn** (read evenness/faults along the thread)
- **L2 — sense the knit** (read defects in the knitted fabric)
- **L3 — grade** (turn the readings into a benchmark-anchored verdict)
- **L4 — code back to the supplier** (send the named spinner a grade/scorecard per lot)

**Verdict by layer:**

| Layer | Does it already exist? | Build it? | Why |
|---|---|---|---|
| **L1 sense yarn** | **Yes — Uster Quantum (in-line) + Tester 6 (lab)** | **No** | Reinvents a decades-deep metrology moat; six-figure capital vs a ~$1–2.50/garment input |
| **L2 sense knit** | Partly — auto fabric-inspection frames + emerging vision | **No (explore later)** | See §6 — vision is cheap but **unvalidated on our fabrics** |
| **L3 grade** | **Yes — USTER STATISTICS percentiles, ICI 1–5, 4-point, AQL** | **No — cite, don't invent** | The scales are pre-written global standards; a DIY grade has no credibility |
| **L4 code-back loop** | **No — nobody sells this turnkey** | **YES — the one ownable sliver** | It's software glue on a contract; ~$0–5k |

**Build-vs-buy, stated plainly:**
- **L1/L2/L3 — BUY (via the PO).** The sensing-and-grading device the owner imagines already exists as supplier-owned, six-figure, benchmark-anchored capital. **A home-built rig can *demo* a reading but cannot *credibly grade*** against USTER STATISTICS — that's Uster's whole moat (Uster is to yarn evenness what Groz-Beckert is to needles). Don't build it. Don't own it. **Demand it.** `[ESTIMATE — engineering + economic judgment, HIGH; converges with the machines DEEPDIVE]`
- **L4 — BUILD (cheaply).** The genuinely novel, ownable piece is the **accountability loop**: log each lot's grade to a scorecard, emit a **grade code / QR back to the named spinner** ("here's how lot #47 scored on CVm%, IPI, pilling"). Nobody sells this as a product because it's *your* contract instrument, not a machine. **Cost ~$0–5k [ESTIMATE, MED-HIGH]** — a spreadsheet/app plus a QR stamp.

**Is the L4 loop worth it? Cheap yes, powerful maybe.** It is worth building because it's nearly free, and it's a real negotiation/selection instrument — over a few lots you can see which spinners are chronically out-of-spec, then re-price or drop them. **But its leverage is conditional, not guaranteed.** It only has teeth if:
1. the **numeric PO spec (§2) exists** to grade against,
2. you will **actually reject lots**, and
3. **your order is big enough that the spinner cares** — a micro-capsule's returned code may simply be ignored. **[UNKNOWN — Open-Q #7]**

> **The reframe that resolves the whole idea:** the thread-checker isn't a gadget you invent — **it's the part you already buy (sensing+grading, via the PO) plus a cheap part you bolt on (the code-back loop).** Premature to build the sensor; smart to build the loop *on top of a numeric PO spec and a willingness to reject.*

**Don't confuse two different QR loops:** the owner's idea is **buyer → supplier** ("here's your grade on this lot" — accountability). That's distinct from consumer-facing **brand → consumer** traceability/digital-product-passport QR (TextileGenesis; EU ESPR DPP basic ~2027 / full ~2028–29). The consumer DPP is a **separate, forward-facing, later-if-marketing-wants-it** option — not the supplier-accountability loop. `[FACT — DPP regulatory trend]`

---

## 6. AI vision QC — where it genuinely helps vs where it's hype

**The honest one-liner:** computer-vision knit-defect detection is **the one brand-cheap QC frontier** — it's software + a camera + lighting, not heavy iron — **but the accuracy numbers you'll be quoted are not proven on our fabrics, and even when they work, vision *assists* the human inspector, it doesn't replace him.**

**Where AI vision genuinely helps `[ESTIMATE, MED]`:**
- **L2 fabric/knit defect detection** — flagging holes, dropped stitches, and streaks on a knitted panel from a camera feed. Cheap hardware; the real value is consistency (a camera doesn't get tired at hour 8).
- As a **screening layer** that triages panels for the human linker/inspector — not as the final accept/reject authority.

**Where it's hype — the binding caveat (keep this loud, not in a footnote):**
- Vendors and papers cite **91–96% / >95% defect-detection accuracy** — but those numbers come from **specific datasets (TILDA, woven and jacquard fabrics)**, **NOT** from our **acrylic/blend stockinette and rib knits**. Loop texture, fabric stretch, barré, and heather/melange colour noise are real confounders. **Accuracy on OUR fabrics is [UNKNOWN].** `[FACT that the numbers are reported; ESTIMATE-HIGH that they do not transfer untested]`
- **Treat any "~95%" claim as marketing until validated on our own pre-production samples (PPS swatches, from T8 sampling).** This is the single make-or-break input for any brand-side vision or "thread-checker" claim.
- **The true cost of vision QC is the data, not the camera** — building a **labelled dataset of *our* defects** and validating a model is the expensive part, and its cost/lead-time for a micro-brand is **[UNKNOWN — Open-Q #4].**

**Where AI vision is NOT the answer:** yarn evenness grading (Uster already does it better and is benchmark-anchored), and final garment accept/reject (a skilled human reading seam/linking quality and shade is still the standard for a small premium line).

---

## 7. The sequence — cheapest-first, what to do and when

1. **NOW (≈ $0 capital):** Write the **numeric PO spec** (§2) → **demand the spinner's USTER cert** (or a third-party lab cert if they're a micro-spinner) → **AQL-sample incoming garments** against a written standard (Critical 0 / Major 2.5 / Minor 4.0) → **buy a per-lot third-party lab spot-check** (~$30–250/lot) for pilling/shrinkage/fastness/composition/evenness → **log every lot to a scorecard that emits a grade code/QR back to the named spinner** (the L4 loop, ~$0–5k).
2. **LATER, only at proven repeat volume on a style:** explore a **used auto fabric-inspection rig (~$3k entry → $82k+ knit-specialised [ESTIMATE/listings, MED])** or a **vision screening layer — but only AFTER validating accuracy on our own samples.**
3. **NEVER:** own a USTER Tester 6, build a yarn clearer, or build the sensing-and-grading "thread-checker" device. Disproportionate to the input and reinvents a metrology moat.

---

## 8. Bottom line for the owner

1. **Sense-and-grade QC is your supplier's job — demand it via the PO, never own it.** The machine that grades thread-to-thread already exists (Uster Quantum / Tester 6) and is six-figure capital on the spinner's floor. Your QC *starts* as a number in the purchase order. `[FACT for the equipment class; ESTIMATE-HIGH for the posture]`
2. **The only QC you actually own is a human inspecting incoming garments under an AQL plan** (Critical 0 / Major 2.5 / Minor 4.0) — near-zero capital — **plus a ~$30–250/lot third-party lab spot-check budget** for pilling (≥3–4 post-wash), shrinkage, spirality, fastness, and fibre composition. That lab budget is your real recurring QC cost line (→ T4).
3. **The "thread-checker" is a contract with a QR stamp, not a machine.** Don't build the sensor (premature, reinvents Uster, disproportionate to a ~$1–2.50/garment input). **Build only the cheap L4 grade→code→named-spinner loop (~$0–5k)** — on top of the numeric PO and a real willingness to reject lots. Its leverage is **conditional** on your order being big enough for the spinner to care — it may be ignored. **[UNKNOWN]**
4. **AI vision is the one brand-cheap frontier, but unproven on our fabrics.** Cited 91–96% accuracies are dataset-specific (woven/jacquard) and **NOT validated on our acrylic/blend knits — [UNKNOWN].** Validate on real samples before believing, and even then it **assists** the human inspector, it doesn't replace him.
5. **Consumer traceability QR (digital product passport) is a separate, later, marketing-driven option** — not the supplier-accountability loop. Don't conflate the two.

---

## 9. Open questions (only on-the-ground/quotes can close these — floor, not ceiling)

1. **Exact USTER TESTER 6 list/used price** — **[UNKNOWN]**; reconciled band **$80k–400k+ [ESTIMATE, LOW]**. Blocks own-vs-rent math, but **the conclusion (don't own) is insensitive to the number.** (→ inherited from yarn Open-Q #6.)
2. **Per-lot third-party lab spot-check cost near Ludhiana** (and a discrete per-sample Uster run) — **~$30–250/lot [ESTIMATE, MED]; per-sample Uster run [UNKNOWN].** The actual recurring QC cost. **→ T4 cost model.**
3. **Knit-defect vision accuracy on OUR acrylic/blend constructions** — **[UNKNOWN]**; vendor/paper >95% is dataset-specific. Needs a labelled dataset of our defects + validation. **The make-or-break input for any brand-side vision/thread-checker claim.** **→ machines DEEPDIVE Open-Q #5 / T8 sampling.**
4. **Cost & lead-time of building a labelled-defect dataset + model** for a micro-brand — **[UNKNOWN]**; the true cost of vision QC is the data, not the camera.
5. **Can target Ludhiana micro-spinners supply a usable USTER cert at all** (vs only integrated Vardhman/Nahar/Sportking)? — **[ESTIMATE, MED-HIGH they can't self-test]**; decides whether the loop runs on supplier-cert or our own paid lab spot-check. **→ T2/T4.**
6. **Realistic 3rd-party AQL inspection man-day rate in Ludhiana** — **[ESTIMATE ~$100–350/day, MED]**; confirm with a quote.
7. **Do spinners actually *act* on a returned grade code?** — **[UNKNOWN]**; the L4 loop's leverage depends on our order being big enough to matter; a tiny capsule's code-back may be ignored. **→ T1/T4 relationship reality.**
8. **Quantified pilling-grade × construction curves for our exact knits** — **[UNKNOWN until sampling]** (shared with yarn DEEPDIVE Open-Q #11). **→ T8/T5.**

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/quality_checker/DEEPDIVE.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `SCOPE_GLOSSARY.md`; `yarn/DEEPDIVE.md` (referenced for ~$1–2.50/garment yarn input + Open-Q cross-links); `machines/DEEPDIVE.md` (style + linker-bottleneck + "don't build the iron" convergence); `quality_checker/reconciled.md` (primary source).
- **Role:** final owner-facing writer for T5 — turned the reconciled QC picture into a decision-ready deep-dive.
- **Structure:** (1) three-layer own/demand/buy mental model → (2) contractual yarn spec (the PO *is* the QC) → (3) incoming AQL (the one owned gate) → (4) finished-garment lab tests (bought) → (5) honest 4-layer verdict on the thread-checker (don't build the sensor, build the cheap code-back loop) → (6) AI vision — genuine help vs hype, with the dataset-transfer caveat kept loud → (7) cheapest-first sequence → (8) Bottom line → (9) 8 open questions.
- **Honesty discipline:** every price/accuracy tagged [ESTIMATE]+confidence, [FACT]+source, or [UNKNOWN]; six-figure tester price held as a wide [ESTIMATE, LOW] with the note that the don't-own conclusion is insensitive to it; vision accuracy on our fabrics held [UNKNOWN] as the headline caveat, not a footnote; L4-loop leverage stated as conditional, not guaranteed; the thread-checker neither dismissed nor hyped — decomposed into a don't-build sensor + a do-build cheap loop.
- **Bottom line:** sense-and-grade QC is the supplier's (demand via PO, never own); the brand owns only human AQL garment inspection + a ~$30–250/lot lab spot-check budget; the thread-checker is a contract with a QR stamp — build only the cheap L4 grade→code→supplier loop on a numeric PO spine; AI vision is the lone brand-cheap frontier but its accuracy on our knits is [UNKNOWN] and must be validated before belief.
