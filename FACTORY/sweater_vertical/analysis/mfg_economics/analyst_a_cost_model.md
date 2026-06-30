# Per-Garment Cost Model — Analyst A (T4)

**For:** the owner of a small, AI-designed D2C knitwear brand sourcing in Ludhiana, selling in California/Turlock.
**Question this answers:** What does **one sweater** actually cost ex-factory, built bottom-up across the **three articles** (A1 cotton-blend core, A2 fine-acrylic value, A3 merino/lambswool premium), with every line tagged honestly — and which single missing number stops this from being a real model rather than an illustration?

**How to read the tags (binding honesty discipline, inherited from T0/T1/T2/T3):**
- **[FACT]** = cited public source / true-by-definition identity.
- **[ESTIMATE, confidence]** = reasoned estimate; method + basis + confidence stated; carries the literal word *estimate*. **Every cost band below is an [ESTIMATE].**
- **[UNKNOWN]** = genuinely not known; we do not fake it. Coverage is a **floor, not a ceiling.**

> **The one sentence to remember.** *This is an **ex-factory conversion cost** model (yarn → knit → link → finish → pack, plus the ~15%+ compliance loading and overhead) — **not** a landed cost and **not** a P&L; and it is **blocked from being precise by one missing input: the Ludhiana per-piece job-work rate for knitting and linking [UNKNOWN]**, the #1 open number carried from T1 (sweater piece-rate) and T3 (knit/link job-work rate). Everything below is an honest **illustrative band** built on a labelled external cross-check, never a quote.*

---

## 0. Scope, boundary, and method (read before the numbers)

**What this model is.** A bottom-up **ex-factory cost stack** for one finished, packed, fully-fashioned (FF) sweater, assembled via the **Ludhiana job-work mesh** (subcontract knit + link + finish — the T3 recommended posture for a capsule brand, not an owned line). The stack is:

```
full yarn cost  +  knitting job-work  +  LINKING job-work  +  finishing  +  trims/labels/packing
  → subtotal (direct conversion)
  +  EPF/ESI compliance loading (~15%+ on the on-roll labour portion it touches)
  +  factory overhead / margin to the job-worker
  =  EX-FACTORY COST per garment  (the number that feeds FOB, then T9 landed cost)
```

**What this model is NOT** (and must not be read as):
- **Not landed cost** — ocean freight, US duty (cotton 6110.20.20 ≈16.5%, wool 6110.11 ~16%, acrylic/MMF 6110.30.30 ~32% [FACT — USITC]), insurance, broker, drayage all live in **T9**.
- **Not a P&L** — CAC, returns, payment fees, AOV, contribution margin are the **D2C survival test (T7/T9)**, not here. Per T0: *contribution margin after CAC, not factory cost, decides viability.*
- **Not a quote** — no live mandi/job-worker quote stands behind any number; bands only.

**Method.** Three independent anchors triangulated:
1. **Yarn:** carried directly from **T2 §3a** — `~$1–2.50 of fibre per garment` for value/core fibres, premium rows order-of-magnitude only. This is the *full yarn cost band* T2 already hardened (yarn ≈50–60% of the knit-garment cost stack, anchored on Vardhman FY20 raw-material = 54% of revenue [FACT, T1 §5a]).
2. **Conversion (knit/link/finish):** an **external published FF-knitwear costing study** used as an **order-of-magnitude cross-check only** — knitting ~US$0.74/pc at 9 pc/operator-day, linking ~US$0.45/pc at 15 pc/operator-day, plus a finishing/QC/pack breakdown (`[FACT — that study's figures]`, but **non-Ludhiana, non-transferable as a quote** → I use it only to bound the *shape* of the stack). `[ESTIMATE: external-anchor, MED on shape, LOW on level for Ludhiana]`
3. **Loadings:** the **~15%+ EPF (12%) + ESI (3.25%) compliance premium** carried verbatim from **T1 §5b** `[ESTIMATE→arithmetic from statutory rates, HIGH on the ~15%]`, applied **only to the on-roll labour portion** the brand actually carries (most job-work labour sits with the subcontractor; see §4 caveat).

**The boundary fact that dominates everything (T1 §5a, T2 §1 one-liner):** **yarn is ~50–60% of the stack and labour ~15–25%** — so the model's *level* is set by yarn, but its *uncertainty* is set by the [UNKNOWN] job-work rate, because that's the line we cannot price.

---

## 1. The line items, defined and tagged (one table, before per-article numbers)

| # | Cost line | What it is | Driver | Tag |
|---|---|---|---|---|
| 1 | **Full yarn cost** | Dyed yarn delivered to the knitter (not raw fibre): fibre + spinning + dyeing + spinner margin | g/garment × ₹/kg (T2 §3a) | `[ESTIMATE, MED core/value; LOW premium]` |
| 2 | **Knitting job-work** | Per-piece charge to knit the FF panels on a flat-bed (or whole-garment) | gauge, GG, structure, g/garment, machine time | **[UNKNOWN] Ludhiana per-piece rate** — bounded only by external cross-check |
| 3 | **LINKING job-work** | Per-piece hand seam joining panels loop-to-loop — *the capacity-setting step* (T3 §1, T1 §4) | linker skill, seam length, structure | **[UNKNOWN] Ludhiana per-piece rate** — the throughput bottleneck *and* an unpriced line |
| 4 | **Finishing** | Wash/scour, soften, anti-pill, steam/press/board, mend | g/garment, fibre, anti-pill requirement (T2 §4) | `[ESTIMATE, LOW-MED]` |
| 5 | **Trims / labels / packing** | Care+brand+size labels, polybag, hangtag, carton share | SKU complexity, brand spec | `[ESTIMATE, MED]` |
| 6 | **QC / checking** | In-line + final AQL checking, mend allowance | defect rate, AQL plan (T0 §5) | `[ESTIMATE, LOW-MED]` |
| 7 | **EPF/ESI compliance loading** | 12% EPF + 3.25% ESI ≈ **15%+** on *on-roll* cash wages (T1 §5b) | how much labour the brand carries on-roll vs job-worker carries | `[ESTIMATE, HIGH on rate; UNKNOWN base it applies to]` |
| 8 | **Overhead / job-worker margin** | The subcontractor's power, rent, supervision, profit folded into the job-work price | already partly inside #2–#4 | `[ESTIMATE, LOW-MED]` |
| 9 | **Friction / "speed-money" line** | Planned discretionary-gate cost (T1 §5c) — *named, not arbitrage* | episodic; per-garment amortisation tiny at micro scale | **[UNKNOWN] by design** |
| 10 | **Inverted-duty / GST working-capital drag** | MMF/acrylic input tax > output tax → cash trapped in slow refunds (T1 §5b) | acrylic share, refund lag | **[UNKNOWN] magnitude** — a *cash-cycle* cost, not a per-unit COGS line; flagged, not added |

**Two structural honesty notes.**
- Lines **2 and 3 are the [UNKNOWN] that blocks the model.** Carried explicitly from **T1 Open Q#1** ("per-piece/per-dozen sweater piece-rates — the single highest-value missing number; no reliable public figure exists; do not let anyone fabricate one") and **T3 Open Q#2** ("job-work rates for knitting and linking per piece — the machine-side analogue → T4"). I do not fabricate them. I bound them with an external cross-check and label the result.
- Lines **9 and 10 are real but not per-unit-additive** here — friction is episodic/amortised-to-near-zero per garment at capsule volume, and inverted-duty drag is a **working-capital** cost (it shows up in T9's cash-conversion-cycle math, not as a clean ₹/garment line). I name them so they aren't forgotten, and hold both at [UNKNOWN] rather than inventing a number.

---

## 2. The conversion-cost cross-check (external anchor, labelled — not a Ludhiana quote)

A published fully-fashioned-knitwear costing study gives this per-piece conversion shape `[FACT — that study's stated figures; ESTIMATE that it transfers to Ludhiana, LOW on level]`:

| Step | Study figure | Productivity basis |
|---|---|---|
| Knitting | ~US$0.74 / pc | 9 pc / operator-day |
| **Linking** | ~US$0.45 / pc | 15 pc / operator-day |
| Finishing chain (iron + QC + pack + wash + label, ~5 micro-steps) | ~US$0.7–1.0 / pc total (≈1 unit each) | per-piece |

**How I use it (and how I refuse to mis-use it):**
- It confirms the **shape**: linking ≈ 60% of knitting cost per piece and is genuinely the labour-heavy seam — consistent with T3's "linker sets the daily ceiling." `[ESTIMATE: cross-check, MED]`
- **Ludhiana labour is materially cheaper** than the study's locus: Punjab statutory floor ~₹438/day unskilled (~US$5.1/day) [FACT, T1 §4] vs the higher-wage origin implied by the study. A first-order wage-ratio adjustment pulls the **Ludhiana conversion cost *below* the external figures** — but the *productivity* (pc/operator-day) is what the rate actually multiplies, and that is **[UNKNOWN]** for a Ludhiana unit. **So I cannot collapse this to a point.** I carry a band.
- **Resulting Ludhiana conversion band [ESTIMATE, LOW-MED]:** knit + link + finish + check ≈ **US$1.5–4.0 per garment** for a mid-complexity FF pullover, widening up for cables/intarsia/jacquard and fine 12GG (slower → costlier, T0 §3) and down for plain 7GG jersey. **This band is the single softest part of the model and it is soft *because* lines 2–3 are [UNKNOWN].**

Cross-check sanity vs Ludhiana fragments found: a quoted "₹12/kg knitting" fragment and "USD 20–100+/dozen" sweater job-work ranges both sit loosely inside the US$1.5–4.0/garment conversion band once yarn weight and dozen-math are applied — consistent, not confirmatory. `[ESTIMATE, LOW]`

---

## 3. The per-article cost stacks (illustrative bands — assumptions stated, never facts)

**Shared assumptions (stated, so the band is reproducible, not authoritative):**
- FF construction, subcontracted into the Ludhiana mesh; bought-dyed stock-shade yarn (T2 §5 launch default).
- Garment weights from T2 §1b; yarn ₹/kg from T2 §3a (`[ESTIMATE]` bands on stale points — **refuse as current; get live quotes**).
- USD/INR ≈ ₹83/$ for translation `[ESTIMATE, MED — rate moves]`.
- Conversion band from §2 (`[ESTIMATE, LOW-MED]`, resting on an **[UNKNOWN]** Ludhiana per-piece rate).
- Compliance loading applied as a **+10–15% uplift on the conversion subtotal only** (proxy for the on-roll labour the brand or a compliant job-worker carries; T1 §5b) — **not** on yarn. Magnitude of the *base* it applies to is **[UNKNOWN]**; I uplift the labour-bearing portion, not the whole stack.
- Trims/labels/pack + QC ≈ **US$0.8–1.8/garment** `[ESTIMATE, MED]`.

### A1 — Cotton-blend 12-month core (12GG, ~200–350 g)
| Line | Band (US$) | Tag |
|---|---|---|
| Full yarn (cotton-blend, ~0.25–0.35 kg @ ~$3.0–4.0/kg) | **$0.9–1.6** | `[ESTIMATE, MED]` |
| Knitting job-work (12GG, fine → slower) | **$0.8–2.0** | **[UNKNOWN] rate → band only** |
| **Linking job-work** | **$0.5–1.5** | **[UNKNOWN] rate → band only** |
| Finishing (scour/soften/press; cotton may add mercerise option) | **$0.4–1.0** | `[ESTIMATE, LOW-MED]` |
| Trims/labels/pack + QC | **$0.8–1.8** | `[ESTIMATE, MED]` |
| Conversion subtotal (knit+link+finish+trim+QC) | ~$2.5–6.3 | — |
| + Compliance loading (10–15% on labour-bearing portion) | +$0.2–0.7 | `[ESTIMATE, HIGH rate]` |
| + Overhead/job-worker margin (folded, residual) | +$0.3–0.8 | `[ESTIMATE, LOW-MED]` |
| **Illustrative ex-factory band** | **≈ $4–9 / garment** | `[ESTIMATE, LOW-MED — blocked by [UNKNOWN] job-work rate]` |

### A2 — Fine-acrylic value driver (7–12GG, ~250–450 g)
| Line | Band (US$) | Tag |
|---|---|---|
| Full yarn (acrylic/blend, ~0.3–0.45 kg @ ~$3.1–4.6/kg) | **$0.9–2.1** | `[ESTIMATE, MED]` |
| Knitting job-work (7–12GG) | **$0.7–1.8** | **[UNKNOWN] rate → band only** |
| **Linking job-work** | **$0.5–1.4** | **[UNKNOWN] rate → band only** |
| Finishing (**anti-pill is mandatory here** — the A2-specific cost, T2 §4) | **$0.4–1.1** | `[ESTIMATE, LOW-MED]` |
| Trims/labels/pack + QC | **$0.8–1.8** | `[ESTIMATE, MED]` |
| Conversion subtotal | ~$2.4–6.1 | — |
| + Compliance loading | +$0.2–0.7 | `[ESTIMATE, HIGH rate]` |
| + Overhead/margin | +$0.3–0.8 | `[ESTIMATE, LOW-MED]` |
| **Illustrative ex-factory band** | **≈ $3.5–8 / garment** | `[ESTIMATE, LOW-MED — blocked by [UNKNOWN] job-work rate]` |

*A2 is the cost floor of the three (Ludhiana's native fibre, best MOQ-flex), but its margin lives or dies on the anti-pill finish surviving ICI ≥3–4 post-wash (T2) — a cheap line that, if cut, returns as the #1 return driver.*

### A3 — Merino/lambswool premium capsule (7–12GG, ~250–400 g)
| Line | Band (US$) | Tag |
|---|---|---|
| Full yarn (merino/lambswool, ~0.3–0.4 kg @ ~$6–34/kg — **wide**) | **$2–13+** | `[ESTIMATE, LOW — premium rows order-of-magnitude, T2]` |
| Knitting job-work (fine, careful handling) | **$0.8–2.0** | **[UNKNOWN] rate → band only** |
| **Linking job-work** (premium care) | **$0.6–1.6** | **[UNKNOWN] rate → band only** |
| Finishing (milling/softening; gentle handling) | **$0.5–1.3** | `[ESTIMATE, LOW-MED]` |
| Trims/labels/pack + QC (premium hangtag/care story) | **$1.0–2.2** | `[ESTIMATE, MED]` |
| Conversion subtotal | ~$2.9–7.1 | — |
| + Compliance loading | +$0.2–0.8 | `[ESTIMATE, HIGH rate]` |
| + Overhead/margin | +$0.4–1.0 | `[ESTIMATE, LOW-MED]` |
| **Illustrative ex-factory band** | **≈ $6–22+ / garment** | `[ESTIMATE, LOW — yarn-dominated and order-of-magnitude]` |

*A3's band is dominated by yarn and is the **least reliable** of the three — the premium fibre price is order-of-magnitude only (T2 §3a, single stale anchor) and needs an importer quote before any margin use. Wool duty (~16%, T2 §1d) is comparable to cotton (~16.5%), not lower, so A3's *landed* premium is widened by its high ex-factory yarn cost rather than by any duty advantage — while the acrylic article carries the highest duty (MMF ~32%).*

---

## 4. Why this is an illustration and not a model — the binding [UNKNOWN]

**The blocker, stated plainly:** the **Ludhiana per-piece job-work rate for knitting and (especially) LINKING is [UNKNOWN].** No reliable public figure exists (confirmed across T1 Open Q#1, T3 Open Q#2, and fresh search — IndiaMART/trade listings show "knitting job work" vendors and a loose "₹12/kg" and "USD/dozen" fragments, but **no quotable per-piece FF-sweater linking rate**). Because lines 2–3 are ~**25–45% of the conversion subtotal** and are the lines I cannot price, the ex-factory bands above are **honest illustrations, not a costed model.**

**What unblocks it (one cheap action):** a single on-the-ground quote run — *"₹/piece to knit FF panels and ₹/piece to link, at 7GG and 12GG, for a 300 g and 400 g pullover"* — from 2–3 Ludhiana job-workers collapses lines 2–3 from [UNKNOWN] to [ESTIMATE, MED] and tightens every band above by roughly half. **This is the single highest-leverage data acquisition in the whole cost segment.**

**Two more honesty caveats on the loadings:**
- The **~15%+ EPF/ESI premium is real but mostly lands on the *job-worker*, not the brand**, in a subcontract model — the brand carries it directly only on whatever labour it puts on its own roll (sampling, in-house finishing/QC). So I apply it to the *labour-bearing portion of conversion*, not the whole stack. The **competitive-gap magnitude** (vs an off-roll competitor who sheds it) stays **[UNKNOWN]** per T1 §5b — it's a *price-competition* disadvantage, not a clean per-unit add.
- **Inverted-duty / GST drag and friction/speed-money are named, not added** — they are working-capital and episodic costs (T9 / T1 §5c), and inventing a ₹/garment figure for either would be the exact fabrication the honesty discipline forbids.

---

## Bottom line for the owner

1. **Ex-factory ≠ landed ≠ P&L.** This stack ends at the factory gate; duty/freight/CAC are T9/T7. Per T0, **contribution after CAC — not this number — decides viability.**
2. **Yarn sets the level, the [UNKNOWN] linking/knitting rate sets the uncertainty.** Yarn is ~50–60% and well-anchored ($1–2.50 core/value fibre); the conversion lines are 25–45% and **unpriced** because the Ludhiana per-piece job-work rate is **[UNKNOWN]** — the #1 missing number carried from T1/T3.
3. **Illustrative ex-factory bands (assumptions in §3, never facts):** **A1 cotton-core ≈ $4–9**, **A2 acrylic-value ≈ $3.5–8**, **A3 merino/lambswool premium ≈ $6–22+** per garment. `[ESTIMATE, LOW-MED for A1/A2; LOW for A3]`
4. **A2 is the cost floor; A3 is yarn-dominated and least reliable.** A2's anti-pill finish is a cheap mandatory line; A3's premium-fibre price is order-of-magnitude only — get an importer quote.
5. **One quote run unblocks the model.** Ask 2–3 job-workers for ₹/piece knit + ₹/piece link at 7GG/12GG and 300/400 g; that single action converts this illustration into a real bottom-up model.
6. **Don't add what you can't price.** Friction/speed-money and inverted-duty drag are named and held [UNKNOWN] — they're cash-cycle/episodic costs for T9, not per-garment COGS lines.

---

## Open questions — carried forward (floor, not ceiling)

1. **Ludhiana per-piece knit + LINK job-work rate (₹/piece at 7GG/12GG, 300/400 g) — [UNKNOWN]; THE blocker.** Collapses on one on-the-ground quote run. (T1 Open Q#1 / T3 Open Q#2.)
2. **Real small-run garment/CMT MOQ at a named unit (~100–300 pcs) — [ESTIMATE, MED, T2].** Sets fixed-cost amortisation per garment.
3. **Live 2025–26 yarn ₹/kg per fibre/count — [ESTIMATE] bands on stale points (T2 §3a).** Needed before any margin model; premium rows need an importer quote.
4. **PSPCL industrial power tariff + the job-worker's power share folded into the rate — [UNKNOWN] (T1 Open Q#4).**
5. **Inverted-duty/GST working-capital drag on acrylic (A2) — magnitude [UNKNOWN]; a T9 cash-cycle cost, flagged here.**
6. **Compliance-premium *base* (how much labour the brand carries on-roll vs job-worker) — [UNKNOWN]; sets what the ~15%+ actually multiplies.**
7. **Per-style fixed costs (sampling/PPS, tech-pack, lab dips, AQL inspection) amortised over MOQ — [ESTIMATE, LOW]; pulls small-run per-garment cost up materially at low volume.**

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/mfg_economics/analyst_a_cost_model.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `SCOPE_GLOSSARY.md`; `ludhiana/DEEPDIVE.md`; `yarn/DEEPDIVE.md`; `machines/DEEPDIVE.md`
- **Web (labelled cross-check only, non-Ludhiana):** published FF-knitwear costing study (knitting ~$0.74/pc @9/day, linking ~$0.45/pc @15/day, finishing chain) used to bound the *shape* of conversion cost, **not** as a Ludhiana quote; IndiaMART/trade fragments ("₹12/kg knitting", "USD 20–100+/dozen") as loose sanity only.
- **Delivered:** (1) scope/boundary — ex-factory conversion stack, explicitly not landed/not P&L; (2) a 10-line tagged cost-line table with lines 2–3 (knit/LINK) flagged **[UNKNOWN]** and lines 9–10 (friction, inverted-duty) named-but-not-added; (3) external conversion cross-check labelled and refused as a quote; (4) three per-article illustrative ex-factory bands (**A1 ≈$4–9, A2 ≈$3.5–8, A3 ≈$6–22+**) with all assumptions stated; (5) the explicit statement that the **[UNKNOWN] Ludhiana per-piece job-work rate blocks a precise model**, plus the one quote-run that unblocks it.
- **Honesty discipline:** Ludhiana knit/link per-piece rate held **[UNKNOWN]** and named as the #1 blocker (never fabricated); every cost band tagged `[ESTIMATE]` with confidence; premium-fibre (A3) row carried as order-of-magnitude/LOW; compliance loading applied only to the labour-bearing portion with its base flagged [UNKNOWN]; friction/speed-money and inverted-duty drag named but held [UNKNOWN] rather than invented; external study used as shape-only cross-check, explicitly not transferable as a Ludhiana quote; bands stated as illustrations, never as facts.
- **Bottom line:** yarn sets the level and is well-anchored; the [UNKNOWN] per-piece knit/link rate sets the uncertainty and blocks precision; A1≈$4–9 / A2≈$3.5–8 / A3≈$6–22+ ex-factory as labelled illustrations; one job-worker quote run converts this into a real model.
