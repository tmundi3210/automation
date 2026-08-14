# T5 QC-Tech Landscape — Analyst A

**Role.** Map the quality-control *technology* for knit yarn + sweaters, end-to-end: yarn-evenness lab testing (Uster-class), online yarn monitoring on winders (Uster Quantum-class clearers), fabric/garment inspection, computer-vision knit-defect detection, in-line vs off-line QC, and AQL sampling. State what is **commercially available**, rough **cost bands [ESTIMATE]**, and — the load-bearing question for this venture — **what a micro D2C brand can actually run in-house vs what it can only demand contractually** (per the T2 yarn-PO spec). This is the technology substrate the T5 "thread-checker" device idea sits on; it does **not** decide the device's feasibility (that's the reconcile/DEEPDIVE step) but it bounds it.

**Honesty tags (binding, travel downstream):** **[FACT]** = cited public source. **[ESTIMATE]** = method + basis + confidence + literal word *estimate*. **[UNKNOWN]** = genuinely not known, stated not faked. Coverage is a **floor, not a ceiling**. Every money/throughput/accuracy band below is **[ESTIMATE]** unless a source is named.

> **The one sentence to remember.** *Almost all of the QC technology that matters here — Uster lab testers (CVm%/IPI), Quantum-class yarn clearers, automatic fabric-inspection machines — lives on the **supplier's** factory floor, is six-figure-to-line-cost capital, and is therefore something a micro-brand **specifies and demands contractually (USTER-percentile clause + ICI pilling gate + AQL on the export PO)**, not something it owns; the only QC the brand itself realistically runs is **human visual inspection of incoming garments under an AQL plan, plus a small budget for third-party lab spot-checks** — and computer-vision knit-defect detection, whose cited 91–96% accuracies are **dataset-specific and not transferable to our acrylic/blend knits**, is the one place a cheap brand-side tool *could* eventually live, but only after validation on our own samples.*

---

## 1. The QC stack, mapped to where in the chain it sits

Quality is checked at four stages; each has its own dominant technology, and each sits on a **different party's** premises. The single most important framing for a micro-brand is **whose floor the machine is on**, because that decides own-vs-demand.

| Stage | What's checked | Dominant tech | Whose floor | Brand posture |
|---|---|---|---|---|
| **Fibre in** | foreign fibre, trash, maturity, micron | bale testing (Uster HVI/AFIS-class) | spinner | demand (irrelevant to us directly) |
| **Yarn — lab (off-line)** | CVm%, IPI, hairiness, tenacity, count-CV | **Uster Tester 6** (capacitive+optical) | spinner / 3rd-party lab | **demand + budget spot-check** |
| **Yarn — in-line (on the winder)** | thick/thin/nep faults, foreign fibre, count drift, splices | **Uster Quantum-class clearer** | spinner's winding dept | demand (it's the spinner's process control) |
| **Fabric / panel** | holes, drop stitch, barré, streaks, soil, needle lines | **4-point fabric-inspection machine** + human, increasingly **vision** | knitter / CMT unit | demand + visual on receipt |
| **Garment (finished)** | seam/linking quality, measurements, shade, pilling, defects | **human visual under AQL**; lab pilling/dim-stability tests | CMT unit / **the brand** | **OWN this one** |

**Load-bearing point #1:** every box except the last is six-figure or line-integrated capital on a **supplier's** floor. The micro-brand's *own* QC is the bottom row — visual garment inspection under an AQL plan — plus paying a lab occasionally. Everything above the bottom row is enforced through the **PO clause**, exactly as T2 §4 already framed it ("contractual QC, not in-house metrology").

---

## 2. Yarn evenness — the Uster Tester (off-line lab)

**What it is.** The **USTER® TESTER 6** is the global-standard laboratory instrument for staple-yarn evenness. It is a **capacitive** mass-evenness sensor (Sensor CS) plus an **optical** sensor module; runs a yarn package past the sensor at **10–800 m/min** depending on mode, measurement range ~**1 tex to 12 ktex**. `[FACT — uster.com Tester 6 product page + technical datasheet, Nov 2021]`

**What it measures (the numbers a PO turns on — all match GLOSSARY §5):**
- **CVm%** — coefficient of variation of mass per unit length; the headline evenness number, lower = more even. `[FACT — uster.com; textilesbar.com]`
- **U%** — older mean-deviation measure; **CV% ≈ 1.25 × U%**. `[FACT — textile metrology]`
- **IPI (Imperfection Index)** = **thin (−50%) + thick (+50%) + neps (+200%) per 1000 m** at standard sensitivities; the point-defect/"cleanliness" number. (Sensitivity thresholds are user-settable; −50/+50/+200 is the classic reporting set; some reports add thin −40%.) `[FACT — uster.com; textilelearner.net]`
- **Hairiness (H, and S3 protruding ends)** — surface fibre, drives pilling/hand/lint. `[FACT]`
- Plus **count/count-CV**, and via the companion strength tester, **tenacity (cN/tex) + elongation**.

**The grading frame:** a lot is graded against **USTER® STATISTICS** percentiles — "**25% USTER**" = better than 75% of world production for that parameter. This is the literal PO language T2 §4d mandates ("≤25% USTER on CVm% + IPI"), and crucially **why demanding 5% USTER on a mainstream acrylic lot is wrong** — 5% is luxury-tier and either unquotable or priced out for a mass acrylic. `[FACT — Uster Technologies methodology; ESTIMATE: industry-lore, MED-HIGH on the "don't demand 5%" call]`

**Cost band [ESTIMATE].** Uster does not publish list price. A new Tester 6 is **six-figure USD capital** — **~$80k–200k+ [ESTIMATE, LOW confidence; method = capital-goods class reasoning + the T2 "6-figure-USD class, list price UNKNOWN" anchor; no public quote]**. Add the companion strength tester, an AFIS/HVI for fibre, and a conditioned lab room and a full Uster lab is **well into mid-six figures**. **A micro-brand never buys this.** `[FACT — list price not public]; [ESTIMATE — band]`

**What the brand actually does instead:** (a) **demand** the spinner's own Uster report per lot (integrated Ludhiana spinners — Vardhman/Nahar/Sportking — have in-house Uster labs per T2 §2); (b) for micro-spinners who can't self-test, **budget a third-party lab spot-check per lot**. Third-party *physical* textile testing (per fabric/colour/test) runs roughly **~$80–250+ per fabric/colour for a test panel** `[FACT — compliancegate.com order-of-magnitude for textile lab tests]`; a single Uster evenness run as a discrete service is **[UNKNOWN — band roughly tens of dollars per sample [ESTIMATE, LOW], confirm with a named lab]**. **Cost + accessibility of a Uster run to a micro-buyer near Ludhiana is [UNKNOWN]** (carried open from T2 Open-Q#6).

---

## 3. Online yarn monitoring — Uster Quantum-class clearers (in-line)

**What it is.** A **yarn clearer** sits on each spindle of the **automatic winder** (the machine that rewinds ring-spun cops onto packages), inspecting **100% of the yarn** as it runs past and **cutting out** faults, then auto-splicing. The current generation is **USTER® QUANTUM 4.0**, which combines **capacitive + optical** sensors in one head. `[FACT — uster.com Quantum 4.0; Textile World 2021; Morton Knit]`

**What it does (process control, not lab grading):**
- Removes **thick/thin places and neps** that exceed set channels (so they never reach the knitter).
- **Foreign-fibre & foreign-matter classification** — separates dark/light foreign fibre, vegetable matter, and **polypropylene (PP)**; "swarm clearing" and FD/TCC classes. `[FACT — uster.com Quantum 4.0]`
- Catches **count/count-CV drift** and splice defects in-line; connects to **Uster Quality Expert** for plant-wide data. `[FACT]`

**Why it matters to us but isn't ours.** This is the spinner's **in-line** defence — it is *why* a 25%-USTER lot arrives clean. It is **line-integrated capital on the spinner's winding floor**, priced per spindle/installation; **no public price** (`[UNKNOWN]`; **[ESTIMATE]** order-of-magnitude **hundreds–low-thousands USD per winding position**, LOW confidence, method = industrial-sensor class reasoning). **The micro-brand neither buys nor sees it** — it shows up only indirectly as the cleanliness of the yarn we contracted for. The honest statement: *Quantum-class clearing is the in-line counterpart to the Uster-Tester lab grade; both are the supplier's, and both are demanded via the same one-line PO clause.*

---

## 4. Fabric & garment inspection (knitter / CMT floor — and the brand's own floor)

### 4a. The 4-point fabric system
The **4-point system** (ASTM D5430) grades finished-fabric defects per 100 yd²: a defect scores **1–4 demerit points** by size, points are summed, and a points-per-100-yd² threshold sets **accept/reject** on the lot. This is the **C&S-side** fabric gate (GLOSSARY §5). `[FACT — ASTM D5430; standard apparel QC]` For **fully-fashioned** sweaters the analogue is **panel + finished-garment inspection** (linking quality, measurements, fashioning, shade), not roll-yardage points — important because our base case is FF, not cut jersey.

**Automatic fabric-inspection machines** exist (SUNTECH, Demas, etc.): a motorised inspection-and-rolling frame, increasingly with a camera bar applying the 4-point grade automatically. **Cost band [ESTIMATE]: entry/SME ~$2,500–15,000; vision-enabled high-throughput $10k+** `[ESTIMATE, MED — vendor/aggregator listings: SUNTECH, made-in-china, accio.com]`. This is **knitter/dyer-floor** capital — relevant to us only as something a CMT partner may or may not have. The brand does **not** buy a roll-inspection frame for a 3-article capsule.

### 4b. What the brand actually runs: human visual inspection under AQL
The bottom row of §1 is the **only** QC stage the brand owns. On receipt of finished garments it inspects a **sample** (not 100%) against a defect standard. This is cheap, human, and entirely within a micro-brand's reach — a trained inspector, a light table, a tape measure, a shade-match light box, and a written defect classification. **This is the realistic brand-side QC.**

---

## 5. AQL sampling — the brand's actual acceptance gate

**What it is.** **AQL (Acceptable Quality Limit)** is the statistical inspection plan that decides, from a **sample**, whether to **accept or reject a whole lot** — codified in **ISO 2859-1 / ANSI-ASQ Z1.4**. You pick a lot size and inspection level → a code letter → a sample size → **accept/reject numbers** at a chosen AQL. `[FACT — ISO 2859-1 / ANSI-ASQ Z1.4; Tetra/QIMA/ASQ]`

**The apparel defaults (this is the gate to write into our PO and run on receipt):**
- **Critical defects: AQL 0** (zero tolerance — safety/illegal).
- **Major defects: AQL 2.5** — the worldwide importer default (defects that drive returns: open seams, broken linking, wrong measurements, large stains). `[FACT — Tetra; QIMA; standard apparel QC]`
- **Minor defects: AQL 4.0** — cosmetic (loose thread ends, faint marks most customers accept). `[FACT]`

**Why AQL is the right tool for a micro-brand:** it gives a **defensible, contractual, sample-based** accept/reject without inspecting every piece — exactly matching a brand that buys a few-hundred-piece run (T2 §3b CMT MOQ ~100–300 pcs). It is the gate that closes the loop the Uster/pilling/fabric specs open: *spec the quality numbers in the PO, then AQL-sample the delivered goods against them.* A third-party inspection firm (QIMA/Intertek-class) can run the AQL inspection at the CMT unit for a per-man-day fee if the brand can't send someone — **[ESTIMATE] ~$100–350/man-day, MED, method = published 3rd-party inspection rate ranges**.

---

## 6. Computer-vision knit-defect detection — available, promising, and **not yet trustworthy on our fabrics**

**Status: commercially real for woven/large mills, research-grade for knit-specific, and explicitly unvalidated on acrylic/blend sweater knits.**

**What the literature claims (carry the honest caveat):** recent deep-learning fabric-defect work reports **~90–99% accuracy** — e.g. dual-branch CNN+GNN **91.3%** accuracy / 92.8% recall / 2.3% FPR; ResNet50+cuckoo-search **95.36%**; multiscale CNN **96.5%** on the **TILDA** dataset; jacquard-CNN ~99%; YOLOv8 ensembles ~90% across six classes. `[FACT — these specific papers report these specific numbers: mdpi.com Electronics 2024/2025; ACM ICCA 2024; ResearchGate 2024-25; Wiley Coloration Tech]`

**The binding caveat (this is the whole point — inherited verbatim from machines/DEEPDIVE §5 and T2/T3 open-Qs):** these accuracies are **dataset-specific**. They are measured on **specific public datasets (TILDA, custom woven sets) under controlled lighting on specific fabric types** — overwhelmingly **woven or jacquard**, not stockinette/rib acrylic-blend sweater knits, which have their own confounders (loop texture, stretch, barré, gauge variation, heathered/melange colour noise). **The cited 91–96% is NOT a guarantee, and NOT transferable to our knits.** `[FACT that the papers report it; ESTIMATE-HIGH that it doesn't transfer untested]`. The papers themselves flag the limiters: **need for large annotated datasets** and **sensitivity to fabric scale/type variation**. **Accuracy on our specific acrylic/blend knits is [UNKNOWN] — T5 must validate on real samples, never assume** (T3 Open-Q#5).

**Cost/feasibility for a micro-brand [ESTIMATE].** A vision-QC capability is **software + camera + lighting + a labelled dataset**, not heavy iron — in principle the *cheapest* QC tech to put on the **brand's own floor**. But the real cost is the **labelled dataset of our defects** and the validation, not the camera. Off-the-shelf vision-inspection on a fabric frame is bundled into the §4a machine price; a bespoke knit model is an R&D project. **This is the one place the T5 "thread-checker" idea could plausibly live brand-side** — but only as a *defect-flagging assist*, validated on our samples, **not** a replacement for the human AQL inspector or the skilled linker (machines/DEEPDIVE §5: "vision QC assists; it does not yet replace the skilled hand seam").

---

## 7. In-line vs off-line QC — the distinction that decides who owns what

- **In-line (on-process, 100%):** the **Quantum clearer** on the winder; an **auto fabric-inspection camera bar** on the rolling frame; vision on a knitting machine. Catches faults **as they're made**, on the **maker's** floor. **Always the supplier's** — we never run in-line QC because we don't run the process.
- **Off-line (sampled, after the fact):** the **Uster Tester** lab run; **AQL** garment inspection; lab **pilling/dimensional-stability** tests. Done on a **sample** **after** production. **This is the brand's entire realistic QC surface** — specifically the **AQL garment inspection** (own) and **lab spot-checks** (buy).

**The clean rule:** *In-line QC is demanded; off-line sampled QC is partly owned (AQL visual) and partly bought (lab). A micro-brand owns no in-line QC and no lab metrology — it owns a clipboard, an AQL table, and a spot-check budget.*

---

## 8. Lab tests behind the PO clauses (what the brand *demands* but doesn't run)

These are the named tests the T2 PO spec references; the brand requires results, a lab (supplier's or third-party) runs them:
- **Pilling — ICI Pilling Box (ISO 12945-1)** grade 1–5 (5=no pill), gated **≥3–4 after ≥1 home wash** (T2 §4c). Also Martindale (ISO 12945-2) / random-tumble. The objective test behind any "anti-pill" claim. `[FACT — ISO 12945; GLOSSARY §5]`
- **Dimensional stability / shrinkage %** after wash; **spirality** (twisted side-seam from unbalanced singles twist); **bursting strength** (knit rupture). `[FACT — standard knit tests]`
- **Fibre-composition test** to verify the **blend ratio ± tolerance** (T2 §1d) — both a quality and a **duty-misdeclaration** guard.
- **Colour: shade std + lot-to-lot ΔE + fastness minima** (wash/light/rub). `[FACT — standard]`

None of these requires brand-owned equipment; all are **contract clauses + a sample sent to a lab**.

---

## 9. What a micro-brand can run vs only demand — the deliverable table

| QC capability | Tech | Own or demand? | Brand cost to *own* [ESTIMATE] |
|---|---|---|---|
| Yarn evenness CVm%/IPI/hairiness | Uster Tester 6 | **DEMAND** (PO: ≤25% USTER) + budget 3rd-party spot-check | own = $80k–200k+ → **don't own** |
| In-line yarn fault clearing | Uster Quantum 4.0 clearer | **DEMAND** (it's spinner process control) | n/a — never brand-side |
| Yarn strength/tenacity | Uster strength tester | DEMAND + spot-check | don't own |
| Roll/fabric 4-point | Auto inspection frame | DEMAND (CMT's) | $2.5k–15k → don't own for a capsule |
| **Finished-garment AQL inspection** | **human + AQL table** | **OWN** | **~$0 capital** (inspector + light box + tape) — *the brand's real QC* |
| Pilling / shrink / spirality / fastness | ICI box, lab tests | DEMAND + 3rd-party lab | ~$80–250+/test panel → buy, don't own |
| Knit-defect computer vision | CNN/YOLO + camera | **explore brand-side, validate first** | software/camera cheap; **labelled dataset + validation is the real cost; accuracy on our knits [UNKNOWN]** |

---

## Bottom line (Analyst A)

1. **The QC technology that matters is the supplier's, not the brand's.** Uster Tester (CVm%/IPI/hairiness/tenacity, ~$80k–200k+ [ESTIMATE]), Quantum-class in-line clearers, and auto fabric-inspection frames all sit on spinner/knitter floors. The micro-brand **specifies them in the PO** (≤25% USTER on CVm%+IPI; never 5% on mass acrylic) and **never buys them**.
2. **The brand's actual, ownable QC is one thing: human visual inspection of incoming garments under an AQL plan** (Major 2.5 / Minor 4.0 / Critical 0), plus a **third-party lab spot-check budget** (~$80–250+/panel) for Uster runs, ICI pilling (≥3–4 post-wash), shrinkage/spirality/fastness, and fibre-composition. Near-zero capital.
3. **In-line QC is demanded; off-line sampled QC is owned (AQL visual) or bought (lab).** A micro-brand owns no in-line QC and no metrology — it owns an AQL table and a spot-check budget.
4. **Computer-vision knit-defect detection is the one plausible brand-side tech frontier** (cheap hardware), but the cited **91–96% accuracies are dataset-specific (TILDA/woven/jacquard) and explicitly NOT transferable to our acrylic/blend sweater knits** — accuracy on our fabrics is **[UNKNOWN], must be validated on real samples, and even then assists rather than replaces** the human AQL inspector or the skilled linker. This is exactly where the T5 "thread-checker" idea has to prove itself or stay an idea.

---

## Open questions (floor, not ceiling — carried to reconcile / T5 DEEPDIVE)
1. **Real list/used price of an Uster Tester 6** and **per-sample 3rd-party Uster run cost near Ludhiana** — [UNKNOWN]; bands only (T2 Open-Q#6).
2. **Per-winding-position cost of Quantum-class clearing** — [UNKNOWN]; only relevant as supplier context.
3. **Knit-defect vision accuracy on OUR acrylic/blend constructions** — [UNKNOWN]; needs a labelled dataset of our defects + validation (T3 Open-Q#5). The make-or-break input for any brand-side vision/"thread-checker" claim.
4. **Cost/lead-time of a labelled-defect dataset + model build** for a micro-brand — [UNKNOWN]; the true cost of vision QC is the data, not the camera.
5. **Whether target Ludhiana CMT/knitter partners already run auto fabric inspection / have Uster reports** — [UNKNOWN]; decides how much we demand vs spot-check.
6. **Realistic AQL inspection man-day rate at a named 3rd-party in Ludhiana** — [ESTIMATE ~$100–350/day, MED]; confirm.
7. **Feasibility of the T5 "thread-checker"** (a cheaper-than-Uster device grading the point-defect + pilling subset and coding a verdict back to the supplier) — **[UNKNOWN by design]**; this analyst bounds it: the yarn-grading half duplicates Uster/Quantum capital the supplier already owns (hard to undercut), while the **defect-vision half is the only genuinely brand-cheap, brand-novel piece — and it's gated on the unvalidated-accuracy problem above.**

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/quality_checker/analyst_a_qc_tech.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `SCOPE_GLOSSARY.md`; `yarn/DEEPDIVE.md`; `machines/DEEPDIVE.md`
- **Web-researched [FACT-anchored]:** Uster Tester 6 (uster.com product page + Nov-2021 datasheet: capacitive+optical, 10–800 m/min, 1 tex–12 ktex, CVm%/IPI/hairiness); Uster Quantum 4.0 clearer (uster.com / Textile World 2021: capacitive+optical, foreign-fibre/PP classification, 100% in-line); CV vision-defect accuracies (mdpi.com 2024-25, ACM ICCA 2024, ResearchGate, Wiley — 91.3%/95.36%/96.5%/~99%, TILDA + custom datasets, with stated annotation/scale limiters); AQL (ISO 2859-1 / ANSI-ASQ Z1.4; Major 2.5 / Minor 4.0 / Critical 0 apparel defaults — Tetra/QIMA/ASQ); 4-point system (ASTM D5430); auto fabric-inspection machine cost band $2.5k–15k (SUNTECH/made-in-china/accio); textile lab test ~$80–250+/panel (compliancegate).
- **Delivered:** (1) the four-stage QC stack mapped by *whose floor* the tech sits on; (2) Uster Tester — what CVm%/IPI/hairiness/tenacity it measures, the USTER-percentile grade, ~$80k–200k+ [ESTIMATE] = demand-not-own; (3) Quantum-class in-line clearers = supplier process control, never brand-side; (4) 4-point + auto fabric inspection ($2.5k–15k, knitter-floor) vs the brand's own human visual inspection; (5) **AQL as the brand's actual acceptance gate** (2.5/4.0/0); (6) computer-vision knit-defect detection with the **binding dataset-specific / not-transferable-to-our-knits caveat**; (7) in-line (demand) vs off-line (own AQL / buy lab) split; (8) the lab tests behind the PO clauses; (9) the **own-vs-demand deliverable table** a micro-brand can act on.
- **Honesty discipline:** every price/throughput/accuracy tagged [ESTIMATE] with method+confidence or [FACT] with named source; Uster + Quantum + per-position prices held as [UNKNOWN]/wide-LOW bands (not public); the 91–96% vision accuracy carried as [FACT-for-those-papers] but **explicitly non-transferable to our acrylic/blend knits, accuracy-on-our-fabrics [UNKNOWN]**; no magic-device over-promise — the thread-checker is bounded, not endorsed; coverage stated as a floor with 7 carried open questions.
- **Bottom line:** the QC tech that matters is the supplier's (Uster/Quantum/fabric-inspection — demand via the PO, never own); the brand's only real owned QC is **human AQL garment inspection + a lab spot-check budget**; computer-vision knit-defect detection is the lone brand-cheap frontier but its cited accuracy is dataset-specific and unproven on our knits — validate before believing.
