# T5 Quality-Checker / Thread-Checker — Reconciled

**Role: reconciler.** Merge Analyst A (QC-tech landscape — maps the commercial QC stack: who owns what, cost bands, own-vs-demand) and Analyst B (thread-checker feasibility — decomposes the owner's device idea, build-vs-buy, worth-it verdict) into one balanced picture. Below: **agreed facts (tagged)**, the **decision logic** both converge on, **contested/uncertain points** with over-confident claims downgraded, and a unified **open-questions** list. Honesty tags travel downstream: **[FACT]** = cited public source; **[ESTIMATE]** = method+basis+confidence + literal word *estimate*; **[UNKNOWN]** = genuinely not known, stated not faked. Coverage is a **floor**.

> **The one sentence to remember.** *The owner's "thread-checker" decomposes into sensing (yarn L1 + knit L2), grading (L3), and a code-back-to-supplier loop (L4); both analysts agree the sensing-and-grading layers already exist as supplier-owned, six-figure, Uster-class capital that a micro-brand **specifies in the PO and never builds or owns** — so the brand's only real owned QC is **human AQL garment inspection + a third-party lab spot-check budget**, and the only genuinely novel, ownable sliver is the **cheap (~$0–5k) buyer→supplier per-lot grade-code loop**, which is **worthless without the upstream numeric PO spec and the willingness to reject lots**, and whose computer-vision component carries an unresolved, binding caveat: cited 91–96% defect accuracies are dataset-specific (TILDA/woven/jacquard) and **NOT proven transferable to our acrylic/blend knits** — validate on real samples before believing.*

---

## 1. Agreed facts (both analysts, independently — high confidence)

These are the load-bearing claims where A and B **converge**; convergence from two separate framings raises confidence.

1. **The QC tech that matters is the supplier's, not the brand's.** Uster lab testers, in-line yarn clearers, and auto fabric-inspection frames are six-figure / line-integrated capital on **spinner / knitter / CMT** floors. The micro-brand **demands them via the PO; never owns them.** `[FACT for the equipment class; ESTIMATE-HIGH for the own-vs-demand conclusion]` — A §1/§9, B §2/§4.
2. **USTER® TESTER 6** = the global benchmark **off-line lab evenness** tester: capacitive (mass) + optical sensors, measuring **CVm%, IPI (thin −50/thick +50/neps +200 per 1000 m), hairiness, count/count-CV, tenacity** (via companion). Graded against **USTER STATISTICS percentiles**. `[FACT — uster.com datasheets 2021–22]` — A §2, B §2a.
3. **USTER® QUANTUM (4.0)** = the **in-line yarn clearer** on the winder: capacitive + optical + foreign-matter (incl. PP) classification, **100% inline**, auto-splice, cloud to Uster Quality Expert. This is *literally* "check the yarn thread-to-thread and record it," already productised, and it lives on the **spinner's** winder. `[FACT — uster.com QUANTUM; Textile World 2021]` — A §3, B §2a.
4. **The grading scales are pre-written — cite, don't invent:** USTER STATISTICS percentiles (5/25/50% USTER); ICI Pilling Box ISO 12945-1 (1–5); 4-point fabric system (ASTM D5430, per 100 yd²); AQL (ISO 2859-1 / ANSI-ASQ Z1.4). `[FACT]` — A §2/§5/§8, B §2b/§3.
5. **The brand's only realistically owned QC = human visual inspection of incoming garments under an AQL plan.** Near-zero capital (inspector + light box + tape + written defect standard). Apparel AQL defaults: **Critical 0 / Major 2.5 / Minor 4.0.** `[FACT — Tetra/QIMA/ASQ]` — A §4b/§5, B §4 (implicit in the contractual-QC posture).
6. **Everything else the brand "does" is bought, not owned:** third-party lab spot-checks for evenness, ICI pilling (gate **≥3–4 after ≥1 home wash**), shrinkage/spirality/fastness, fibre-composition (also a duty-misdeclaration guard). `[FACT — test standards; ESTIMATE for per-test cost]` — A §8, B §4.
7. **Computer-vision knit-defect detection is the one plausible brand-cheap frontier** (software + camera + lighting, not heavy iron), BUT the cited accuracies are **dataset-specific and not proven on our fabrics** (see §3.1 — the central contested/uncertain point). `[FACT that papers/vendors report the numbers; ESTIMATE/UNKNOWN on transfer]` — A §6, B §2b.
8. **Don't build the sensing device.** Matching a graded, repeatable, benchmark-anchored verdict against USTER STATISTICS is a decades-deep specialist moat (Uster is to evenness metrology what Groz-Beckert is to needles); a DIY rig can *demo* but not credibly *grade*. Disproportionate to a **~$1–2.50/garment** yarn input. `[ESTIMATE — engineering+economic judgment, HIGH; converges with machines DEEPDIVE]` — A §6/§7, B §4/§5.

**Net agreed picture:** sense-and-grade = supplier-owned, demanded-not-owned; AQL visual = the brand's one owned gate; lab tests = bought; vision = explore but unvalidated; the device = don't build.

---

## 2. The decision logic (the reasoning chain both share, stated once)

The two files are the **same argument from two entry points** — A from "map the QC tech and ask whose floor it's on," B from "decompose the owner's device and ask what's novel." They land on one decision tree:

1. **Whose floor is the machine on?** → if supplier's (Uster Tester, Quantum clearer, auto fabric-inspection frame) → **DEMAND via PO clause**, never own. (A's organizing principle = B's L1/L2 "already exists, don't reinvent.")
2. **In-line or off-line?** → in-line (100%, on-process) is *always* the maker's and is **demanded**; off-line (sampled, after the fact) splits into **OWN (AQL human visual)** + **BUY (lab spot-checks)**. (A §7.)
3. **For the one ownable QC (AQL):** spec the numbers in the PO → AQL-sample the delivered goods against them (Critical 0 / Major 2.5 / Minor 4.0). This **closes the loop the Uster/pilling/fabric specs open.**
4. **The owner's "thread-checker" idea** = four layers (B §1): **L1** sense yarn, **L2** sense knit, **L3** grade, **L4** code-back-to-supplier. The valuable-*feeling* part (L1/L2 device) is the part NOT to build; the cheap part (**L4** scorecard→grade-code→named spinner) is the **only ownable sliver** — and it is *software glue on a contractual spine*, worth **~$0–5k [ESTIMATE]**, **worthless without the §4d numeric PO and the will to reject lots.**
5. **Sequencing (cheapest-first):** *Now* — write numeric PO spec → demand spinner's USTER cert → AQL sampling → third-party lab spot-check per lot → log to a scorecard that emits a grade code/QR back to the named spinner (capital ≈ $0–5k). *Later, only if proven repeat volume* — a used vision fabric-inspection rig, **only after validating accuracy on our acrylic/blends.** *Never* — own a Tester 6 / build a clearer / build a yarn-sensing device.

**This logic is robust:** it survives the two largest [UNKNOWN]s (exact Uster price; per-lot lab cost) because the conclusion (don't own; demand + spot-check + cheap loop) is *insensitive* to their values.

---

## 3. Contested / uncertain points (over-confident claims downgraded)

The analysts barely disagree on direction; the friction is in **confidence calibration and a few diverging numbers.** Reconciled stance below.

### 3.1 Computer-vision defect accuracy — the one genuinely binding uncertainty (both flag it; keep it loud)
- A: cites papers at **91.3% / 95.36% / 96.5% / ~99%** on TILDA + custom **woven/jacquard** sets; B: cites a vendor knit-rig at **>95%** at ~120 m/min. **Both explicitly downgrade these as dataset-/vendor-specific and NOT transferable to our acrylic/blend stockinette/rib knits** (loop texture, stretch, barré, heather/melange colour noise as confounders).
- **Reconciled:** accuracy-on-our-fabrics = **[UNKNOWN]**. The published/vendor numbers are **[FACT only that they are reported]**, **[ESTIMATE-HIGH that they do not transfer untested]**. Treat any "~95%" as marketing until validated on **our PPS swatches** (T8 sampling). This is the single make-or-break input for any brand-side vision/"thread-checker" claim. **Neither analyst over-claims here — both correctly hold it open; reconciler endorses keeping it as the headline caveat, not a footnote.**

### 3.2 USTER TESTER 6 price band — A and B disagree by ~2×; downgrade to a wide [ESTIMATE]
- A: **~$80k–200k+ [ESTIMATE, LOW]**. B: **~$150k–400k+ class [ESTIMATE, LOW]**. Both stress **exact list price is [UNKNOWN]** (Uster doesn't publish).
- **Reconciled:** state as **six-figure-USD class, exact list price [UNKNOWN]**, with a deliberately wide reconciled band of **~$80k–400k+ [ESTIMATE, LOW confidence]** spanning both. **Do not present either narrow band as firm.** Crucially — and both agree — **the conclusion (don't own) is insensitive to this number**: any six-figure tester is wildly disproportionate to a ~$1–2.50/garment yarn input.

### 3.3 Per-lot third-party lab spot-check cost — A and B quote different bands
- A: **~$80–250+ per fabric/colour test panel** `[FACT — compliancegate order-of-magnitude]`, with a single discrete Uster run held as **[UNKNOWN, ~tens of dollars/sample, LOW]**. B: **~$30–150 per sample per test-suite [ESTIMATE, MED — India textile-lab norms]**.
- **Reconciled:** these are measuring slightly different units (A = a multi-test compliance *panel*; B = a per-sample *suite* near Ludhiana). State the recurring spot-check cost as **~$30–250 per lot [ESTIMATE, MED]** depending on test count and lab, with the **per-sample Uster evenness run near Ludhiana still [UNKNOWN]** (needs a real quote; inherited from yarn DEEPDIVE Open-Q#6). This is the **actual recurring QC cost** → feeds T4 cost model.

### 3.4 The L4 loop's *value* — B asserts it; reconciler adds B's own downgrade explicitly
- B (§5) makes a strong "WORTH IT" case for the code-back loop as a negotiation/selection instrument (drop/re-price chronically out-of-spec spinners). **But B itself flags the counter-risk (Open-Q#5): a micro-brand's order may be too small for a spinner to care, so the returned code may simply be ignored — [UNKNOWN].**
- **Reconciled:** the loop is **cheap and worth building (~$0–5k)**, but its **leverage is contingent, not guaranteed** — it has teeth only if (a) the §4d numeric PO exists to grade against, (b) the brand will actually reject lots, and (c) the order is large enough to matter to the spinner. Downgrade "WORTH IT" to **"cheap, worth building, leverage conditional on order size + contractual spine"** [ESTIMATE, MED]. The loop is *a contract with a QR stamp, not a machine.*

### 3.5 Auto fabric-inspection rig price — minor band difference, reconcile wide
- A: entry/SME **~$2.5k–15k**, vision-enabled **$10k+**. B: **~$3k entry → ~$82k+ knit-specialised**.
- **Reconciled:** **~$3k entry → $82k+ knit-specialised [ESTIMATE/FACT-listings, MED]** (B's range subsumes A's). Irrelevant to a 3-article capsule regardless — **don't own for a capsule**; revisit only at proven repeat volume *after* §3.1 validation.

### 3.6 Direction of the QR/traceability loop — B's clarifying distinction (no conflict, worth preserving)
- B (§2c): consumer DPP/QR (TextileGenesis, US Cotton Trust Protocol, EU ESPR DPP basic 2027 / full ~2028–29) is **brand→consumer** ("here's the journey"); the owner's idea is **buyer→supplier** ("here's your grade on this lot"). **The buyer→supplier per-lot QC scorecard direction is the piece nobody sells turnkey.** `[FACT for the DPP trend]`
- **Reconciled:** keep the distinction — it prevents conflating the owner's accountability loop with consumer marketing traceability. Consumer DPP/QR is a **separate, forward-facing, later-if-marketing-wants-it** option; not the supplier-accountability loop.

---

## 4. Unified own-vs-demand table (merged deliverable)

| QC capability | Tech | Own / Demand / Buy | Reconciled cost to *own* [ESTIMATE] | Confidence |
|---|---|---|---|---|
| Yarn evenness CVm%/IPI/hairiness/tenacity | Uster Tester 6 | **DEMAND** (PO: ≤25% USTER on CVm%+IPI; never 5% on mass acrylic) + buy spot-check | **$80k–400k+ class, exact [UNKNOWN]** → don't own | LOW (price), HIGH (don't own) |
| In-line yarn fault clearing | Uster Quantum 4.0 | **DEMAND** (spinner process control) | n/a — never brand-side | HIGH |
| Roll/fabric 4-point inspection | Auto inspection frame | **DEMAND** (CMT's); explore used rig later | $3k → $82k+ knit-specialised → don't own for capsule | MED |
| **Finished-garment AQL inspection** | **human + AQL table** | **OWN** (Critical 0 / Major 2.5 / Minor 4.0) | **~$0 capital** — *the brand's real QC* | HIGH |
| Pilling / shrink / spirality / fastness / fibre-comp | ICI box, lab tests | **BUY** (3rd-party) + DEMAND in PO | **~$30–250/lot** → buy, don't own | MED |
| Knit-defect computer vision (L2) | CNN/YOLO + camera | **EXPLORE brand-side, VALIDATE FIRST** | hardware cheap; **real cost = labelled dataset + validation; accuracy on our knits [UNKNOWN]** | LOW (transfer) |
| **L4 grade→code→supplier loop** | scorecard + QR/code | **OWN — the one novel sliver** | **~$0–5k software**; worthless without §4d PO + will to reject | MED-HIGH (cost), MED (leverage) |

---

## 5. Reconciled bottom line

1. **Sense-and-grade QC is the supplier's** (Uster Tester / Quantum clearer / auto fabric-inspection) — **demand via the PO, never own.** Both analysts agree; convergence raises confidence. `[FACT for class; ESTIMATE-HIGH for posture]`
2. **The brand's only owned QC = human AQL garment inspection** (Critical 0 / Major 2.5 / Minor 4.0) + a **~$30–250/lot third-party lab spot-check budget** (evenness, ICI pilling ≥3–4 post-wash, shrinkage/spirality/fastness, fibre-composition). Near-zero capital.
3. **The owner's thread-checker = a contract with a QR stamp, not a machine.** Don't build/own the sensing layer (premature, reinvents Uster, disproportionate to a ~$1–2.50/garment input). Build only the **L4 grade→code→named-spinner loop (~$0–5k)** — the lone ownable sliver — **on top of the §4d numeric PO and a willingness to reject lots.** Its leverage is **conditional** on order size mattering to the spinner (it may be ignored — [UNKNOWN]).
4. **Computer-vision knit-defect detection is the only brand-cheap frontier**, but cited **91–96% / >95% accuracies are dataset-/vendor-specific and NOT proven on our acrylic/blend knits** — **[UNKNOWN], validate on real PPS samples before believing, and even then it assists rather than replaces the human AQL inspector or the skilled linker.**
5. **Consumer DPP/QR traceability** (TextileGenesis; EU ESPR 2027/29) is a **separate, forward-facing** option — not the supplier-accountability loop. Adopt later if marketing wants the story.

---

## 6. Open questions (merged + de-duplicated; floor, not ceiling)

1. **Exact USTER TESTER 6 list/used price** — **[UNKNOWN]**; reconciled band **$80k–400k+ [ESTIMATE, LOW]**; blocks own-vs-rent math but **conclusion (don't own) is insensitive.** (A#1, B#1; yarn Open-Q#6.)
2. **Per-lot third-party lab spot-check cost near Ludhiana** (and a discrete per-sample Uster run) — **~$30–250/lot [ESTIMATE, MED]; per-sample Uster run [UNKNOWN]**; the actual recurring QC cost. → **T4 cost model.** (A#1, B#2; yarn Open-Q#6.)
3. **Knit-defect vision accuracy on OUR acrylic/blend constructions** — **[UNKNOWN]**; vendor/paper >95% is dataset-specific; needs a labelled dataset of our defects + validation. **The make-or-break input for any brand-side vision/thread-checker claim.** → **machines DEEPDIVE Open-Q#5 / T8 sampling.** (A#3/#4, B#3.)
4. **Cost/lead-time of a labelled-defect dataset + model build** for a micro-brand — **[UNKNOWN]**; the true cost of vision QC is the data, not the camera. (A#4.)
5. **Can target Ludhiana micro-spinners supply a usable USTER cert at all** (vs only integrated Vardhman/Nahar/Sportking) — **[ESTIMATE, MED-HIGH they can't self-test]**; decides whether the loop runs on supplier-cert or our own lab spot-check. → **T2/T4.** (A#5, B#4.)
6. **Realistic 3rd-party AQL inspection man-day rate in Ludhiana** — **[ESTIMATE ~$100–350/day, MED]**; confirm. (A#6.)
7. **Do spinners actually *act* on a returned grade code?** — **[UNKNOWN]**; the loop's leverage depends on our order being big enough to matter; a tiny capsule's code-back may be ignored. → **T1/T4 relationship reality.** (B#5.)
8. **Quantified pilling-grade × construction curves for our exact knits** — **[UNKNOWN until sampling]** (shared with yarn DEEPDIVE Open-Q#11). → **T8/T5.** (B#6.)

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/quality_checker/reconciled.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `SCOPE_GLOSSARY.md`; `yarn/DEEPDIVE.md`; `machines/DEEPDIVE.md`; `quality_checker/analyst_a_qc_tech.md`; `quality_checker/analyst_b_thread_checker.md`.
- **Method:** merged two analysts into (1) **agreed facts** (8 convergent, tagged), (2) the **single shared decision logic** (whose-floor → in-line/off-line → AQL-own/lab-buy → 4-layer thread-checker → cheapest-first sequence), (3) **contested/uncertain points** with over-confident claims downgraded — vision accuracy held [UNKNOWN] as the binding caveat; Tester 6 price reconciled to a wide [ESTIMATE] band spanning A's $80–200k+ and B's $150–400k+; lab spot-check reconciled to ~$30–250/lot; L4-loop value downgraded from "WORTH IT" to "cheap/worth-building/leverage-conditional"; (4) a **merged own-vs-demand table** and (5) a **de-duplicated 8-item open-questions** list cross-linked to yarn Open-Q#6/#11 and machines Open-Q#5.
- **Reconciliation calls (where I adjudicated):** the two files agree on direction; I (a) widened both price bands rather than pick a side, (b) kept the vision-accuracy [UNKNOWN] as the headline not a footnote, (c) made explicit B's own counter-risk that the code-back loop may be ignored, downgrading its "WORTH IT" to conditional, (d) preserved B's buyer→supplier vs brand→consumer traceability distinction, (e) flagged that the dominant conclusion (demand+spot-check+cheap loop, don't own) is *insensitive* to the two largest [UNKNOWN]s.
- **Honesty discipline:** every price/throughput/accuracy tagged [ESTIMATE]+confidence or [FACT]+source or [UNKNOWN]; no narrow price presented as firm; no magic-device endorsement; coverage stated as a floor with 8 carried open questions.
- **Bottom line:** the thread-checker is a contract with a QR stamp, not a machine — sense-and-grade is supplier-owned (demand via PO, never own); the brand owns only human AQL inspection + a lab spot-check budget; build only the cheap L4 grade→code→supplier loop on the numeric PO spine; computer-vision is the lone brand-cheap frontier but its accuracy on our knits is [UNKNOWN] and must be validated before belief.
