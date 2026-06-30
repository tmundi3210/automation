# Manufacturing Economics — Owner Deep-Dive (T4)

**For:** the owner of a small, AI-designed D2C knitwear brand sourcing in Ludhiana and selling in California / Turlock.
**Question this answers:** What does **one sweater actually cost ex-factory**, article by article — and how honest is that number? At our launch scale, do we **make or buy**? What's the real **minimum batch and the working-capital trap** behind a profitable-looking unit cost? And how do these unit economics **scale with volume** as they feed T9's full landed-cost P&L?

**How to read the tags (binding honesty discipline, inherited from T0/T1/T2/T3):**
- **[FACT]** = cited public source / true-by-definition identity.
- **[ESTIMATE, confidence]** = reasoned estimate; method + basis + confidence stated; carries the literal word *estimate*. **Every cost band below is an [ESTIMATE].**
- **[UNKNOWN]** = genuinely not known; we do not fake it. Coverage is a **floor, not a ceiling.**
- **[ESTIMATE: discussion/industry-lore]** = off-paper material — described as **mechanism, never a named-party accusation**; every rupee amount in it is **[UNKNOWN] by design**.

> **The one sentence to remember.** *This is an **ex-factory conversion cost** model, not a landed cost and not a P&L — and it is **honest but imprecise by exactly one input: the Ludhiana per-piece knit + LINK job-work rate, which is [UNKNOWN]**; yarn sets the *level* of the stack (well-anchored at ~$1–2.50 of fibre per value/core sweater) while that one missing rate sets the *uncertainty* — and it is the same number that decides the make-vs-buy boundary, so **one on-the-ground quote run unblocks the whole segment at once.***

> **A process caveat the writer must surface (don't bury it).** This segment was built from **only one of its two intended analysts**: the per-garment cost model (`analyst_a_cost_model.md`) exists; the independent make-vs-buy memo (`analyst_b_make_vs_buy.md`) **was never written** `[FACT — file absent]`. The make-vs-buy verdict below is therefore **reconstructed-consensus from three upstream deep-dives (T1/T2/T3) that already converged on it** — robust as a conclusion, but it never received an independent adversarial challenge inside T4. Treat §2's boundary and §3's bands as **single-analyst / reconstructed**, and press them harder than a normal two-source result. `[UNKNOWN — missing second-analyst challenge]`

---

## 0. The boundary — what this number is, and the four numbers it is NOT

Getting the boundary wrong is the most expensive mistake in the whole segment, so it comes first. This model computes **ex-factory conversion cost per garment**:

```
full (dyed) yarn  +  knit job-work  +  LINK job-work  +  finishing  +  trims/labels/pack
   → direct-conversion subtotal
   +  EPF/ESI compliance loading (~15%+, on the on-roll labour portion only)
   +  factory overhead / job-worker margin
   =  EX-FACTORY COST per garment  →  feeds FOB, then T9 landed cost
```

It is **NOT**, and must never be read as:
1. **Not landed cost.** Ocean freight, US duty (cotton 6110.20 ≈7% / wool 6110.11 ~16% `[FACT — USITC]`), insurance, broker, drayage, financing all live in **T9**.
2. **Not COGS-to-stock.** Add the above first.
3. **Not a P&L.** CAC, returns, payment fees, AOV, contribution margin are the **D2C survival test (T7/T9)** — and per T0 cross-team rule #8, **contribution margin after CAC, not factory cost, decides viability.** A great ex-factory number can still die on CAC.
4. **Not a quote.** No live mandi or job-worker quote stands behind any figure here. **Bands only**, by honest necessity (§4).

**Why this matters to the owner:** the factory gate is the *cheapest, least-decisive* part of the journey. The temptation is to over-optimise it; the discipline is to cost it honestly, then hand the real margin fight to T7/T9. This is the same lesson the cluster's own winners learned — they make the money in **downstream brand retail, not at the loom** (T1 §3/§4).

---

## 1. What sets the level vs what sets the uncertainty — the two-number frame

The entire segment reduces to two facts pulling in opposite directions:

- **Yarn sets the LEVEL — and it's well-anchored.** Yarn is **~50–60% of the knit-garment stack** (anchor: Vardhman FY20 raw material = **54% of revenue** `[FACT, T1 §5a]`; held at T1's MED confidence, *not* promoted to a measured cluster average). In garment terms that's **~$1–2.50 of fibre per value/core sweater** at ~300–500 g `[ESTIMATE, MED — carried from T2 §3a]`. So the *magnitude* of the stack is reliable.
- **The job-work rate sets the UNCERTAINTY — and it's [UNKNOWN].** Labour (knit/link/finish/check) is **~15–25%** of the stack `[ESTIMATE, MED, T1 §5b]`, and the **knit + LINK per-piece rate inside it has no reliable public figure** (confirmed across T1 Open Q#1, T3 Open Q#2, and Analyst A's fresh search). Those two lines are **~25–45% of the conversion subtotal** — the part we cannot price. So the *precision* of the stack is blocked at exactly one input.

**The owner takeaway:** don't waste effort hunting cheaper yarn (it's small, anchored, and not the constraint). Spend the effort getting **one number** — the ₹/piece to knit and to link — because it both *halves the width of every band below* and *settles the buy-vs-own-knitting decision* in one stroke. It is the single highest-leverage data acquisition in the segment.

---

## 2. Make-vs-buy at capsule scale — the verdict, the ladder, and the honest counter

**The verdict (HIGH confidence, convergent across T1/T2/T3; the cost model is consistent with it):**

> **Buy, don't make — at capsule scale.** Own only **design + brand + US import (importer of record)**. **Subcontract knit + link + finish into the Ludhiana job-work mesh**, **buy yarn in-cluster on bought-dyed stock shades** (import only premium merino tops/cashmere), and **rent/contract the scarce CAD/flat-knit programmer**. Graduate toward owning a step **only when one specific repeating program runs at proven, sustained volume** — and **linking last**, because it's both the throughput bottleneck and the hardest skill to re-hire. **Never** own a dyehouse; **never** build a machine. `[ESTIMATE: judgment, HIGH — three upstream analysts converged; T1 §3/§6, T2 §5, T3 §1–6]`

**The make-vs-buy ladder, stage by stage — where the boundary sits and why:**

| Stage | Default posture | Make only when | Why buy at launch |
|---|---|---|---|
| **Spinning / yarn** | **Buy** (market yarn, in-cluster) | never, at this scale | Integrated groups have captive-spinning margin we can't match; yarn is a commodity input we **spec**, not make (T1 §3, T2 §2). |
| **Dyeing** | **Buy** (bought-dyed stock shades; job-work dye only for a *proven* hero colour) | **never own** | Highest-regulatory-risk node — all 3 Ludhiana CETPs non-compliant, crore-scale CPCB penalties, NGT review, closure exposure wildly out of proportion to a 3-article line `[FACT, T1 §5b; T2 §5]`. |
| **Knitting** | **Subcontract** the FF panels | one style at proven volume → buy **used** (Chinese/Cixing first) | The agglomeration *is* the infrastructure; small-batch iteration is Ludhiana's genuine edge; a tiny owned line is a **mid-six-figure + skilled-staffing commitment before a garment sells** (T3 §2). |
| **Linking** | **Subcontract** | **last** to in-source | The linker sets the daily ceiling; owning it early imports both the throughput risk *and* the hardest-to-hire skill (T3 §1, T1 §4). |
| **Finishing** | **Subcontract** (optionally one cheap used press for sampling) | when volume justifies | Anti-pill / press / board are job-work steps the mesh already supplies (T2 §4, T3 §1). |
| **Design + brand + US import** | **OWN** | — always | This is the venture's entire edge and where downstream margin is captured (T1 §6, GLOSSARY §8). |

**Why "buy" is correct here (the consensus):** capital stays free for design + CAC; the cluster's walking-distance subcontracting mesh suits the **small, fast, varied runs an AI-design brand lives on**; the venture's margin is captured downstream in US D2C, so the factory gate isn't the whole game; and **a compliant entrant can never win on factory cost anyway** — the incumbents' model is precisely the paper-cost-vs-real-cost gap a compliant newcomer can't match (T1's central seam). `[ESTIMATE: judgment, HIGH]`

**The honest counter (under-pressed, because Analyst B never wrote it — keep it visible):**
- **QC fragments across many micro-vendors.** Agglomeration is flexible *but* QC-fragmented (T1 §3); a garment assembled across several tiny job-workers has more hand-off seams where quality slips. This is exactly the gap T5's "thread-checker" is meant to close cheaply.
- **No captive margin to absorb shocks.** A buy-everything brand carries no factory margin cushion against a yarn spike, a bad season, or a returns wave.
- **Per-style fixed costs amortise over a tiny MOQ → they pull per-garment cost up materially at low volume** (this is §3c's volume story, and Analyst A Open Q#7). The "buy" verdict is right for *launch*, but its **cost penalty at micro-volume is real and not yet quantified.** `[ESTIMATE, LOW]`

**Net:** buy at launch (high confidence); graduate to owning a step only on proven, repeating volume; linking last; dyehouse and machine-building never. The verdict is sound; its micro-volume cost penalty (§3c) is the part that still needs a real MOQ to quantify.

---

## 3. The per-garment cost stack, article by article

### 3a. The 10 cost lines — and the honest structural notes

| # | Cost line | What it is | Tag |
|---|---|---|---|
| 1 | **Full yarn** | Dyed yarn delivered to the knitter (fibre + spin + dye + spinner margin) | `[ESTIMATE, MED core/value; LOW premium]` |
| 2 | **Knitting job-work** | Per-piece charge to knit the FF panels | **[UNKNOWN] Ludhiana per-piece rate** |
| 3 | **LINKING job-work** | Per-piece hand seam, loop-to-loop — *the capacity-setting step* | **[UNKNOWN] Ludhiana per-piece rate** |
| 4 | **Finishing** | Wash/scour, soften, **anti-pill**, steam/press/board, mend | `[ESTIMATE, LOW-MED]` |
| 5 | **Trims / labels / pack** | Care+brand+size labels, polybag, hangtag, carton share | `[ESTIMATE, MED]` |
| 6 | **QC / checking** | In-line + final AQL checking, mend allowance | `[ESTIMATE, LOW-MED]` |
| 7 | **EPF/ESI compliance loading** | 12% EPF + 3.25% ESI ≈ **15%+** on *on-roll* cash wages | `[ESTIMATE, HIGH on rate; UNKNOWN on the base]` |
| 8 | **Overhead / job-worker margin** | Subcontractor's power, rent, supervision, profit folded into the job price | `[ESTIMATE, LOW-MED]` |
| 9 | **Friction / "speed-money"** | Planned discretionary-gate cost (T1 §5c) — *named, not arbitrage* | **[UNKNOWN] by design — NOT added** |
| 10 | **Inverted-duty / GST drag** | Acrylic input tax > output tax → cash trapped in slow refunds | **[UNKNOWN] magnitude — a cash-cycle cost (T9), NOT a per-unit add** |

**Three structural honesty notes (binding):**
- **Lines 2–3 are the [UNKNOWN] that blocks precision.** Carried verbatim from T1 Open Q#1 ("the single highest-value missing number; no reliable public figure exists; do not let anyone fabricate one") and T3 Open Q#2. We do not invent them; we **bound** them with a labelled external cross-check (§3b) and call the result an illustration.
- **Lines 9–10 are real but NOT per-unit-additive — correctly named, not added.** Friction/speed-money is **episodic and amortises to near-zero per garment at capsule volume** (and is described as mechanism, never as a named-party act — every amount [UNKNOWN] by design). Inverted-duty/GST drag is a **working-capital cost** that shows up in T9's cash-conversion-cycle math, *not* as a clean ₹/garment COGS line. Inventing a per-garment figure for either would be the exact fabrication the honesty discipline forbids. They live in §4 (cash) and T9, not in the stack.
- **The ~15%+ compliance loading mostly lands on the *job-worker*, not the brand.** In a subcontract model the brand carries EPF/ESI directly only on labour it puts on its *own* roll (sampling, in-house finishing/QC). So it is applied to the **labour-bearing portion of conversion only**, never the whole stack — and the competitive-gap magnitude vs an off-roll competitor stays **[UNKNOWN]** (T1 §5b). It's a price-competition disadvantage, not a clean per-unit add.

### 3b. The conversion cross-check (external anchor — shape only, NOT a Ludhiana quote)

A published fully-fashioned-knitwear costing study gives this per-piece conversion *shape* `[FACT — that study's figures; ESTIMATE that it transfers to Ludhiana, LOW on level]`:

| Step | Study figure | Productivity basis |
|---|---|---|
| Knitting | ~US$0.74 / pc | 9 pc / operator-day |
| **Linking** | ~US$0.45 / pc | 15 pc / operator-day |
| Finishing chain (iron + QC + pack + wash + label) | ~US$0.7–1.0 / pc total | per-piece |

**How we use it — and how we refuse to mis-use it:** it confirms the **shape** (linking ≈ 60% of knitting cost per piece; the labour-heavy seam; consistent with T3's "linker sets the ceiling"). Ludhiana labour is materially cheaper (Punjab floor ~₹438/day ≈ US$5.1 `[FACT, T1 §4]`), which pulls the *level* down — **but the productivity (pc/operator-day) the rate multiplies is [UNKNOWN] for Ludhiana**, so it cannot collapse to a point. Resulting **Ludhiana conversion band: ~US$1.5–4.0/garment** for a mid-complexity FF pullover — wider up for cables/intarsia/jacquard and fine 12GG (slower → costlier), down for plain 7GG jersey. **This is the single softest part of the model, and it is soft *because* lines 2–3 are [UNKNOWN].** `[DOWNGRADED — ESTIMATE, LOW-MED]` Loose Ludhiana trade fragments ("₹12/kg knitting", "USD 20–100+/dozen") sit inside this band — **consistent, not confirmatory; do not harden them into a quotable rate.** `[DOWNGRADED — ESTIMATE, LOW]`

### 3c. The three per-article ex-factory bands (labelled illustrations — assumptions stated, never facts)

*Shared assumptions (so the band is reproducible, not authoritative): FF construction subcontracted into the mesh; bought-dyed stock-shade yarn; weights from T2 §1b; yarn ₹/kg from T2 §3a (`[ESTIMATE]` bands on stale points — **refuse as current, get live quotes**); USD/INR ≈ ₹83/$ `[ESTIMATE, MED]`; conversion from §3b; compliance loading as +10–15% on the labour-bearing portion only; trims+QC ≈ $0.8–1.8.*

| Article | Yarn | Knit + Link (both [UNKNOWN] rate) | Finish + trims + QC | **Illustrative ex-factory band** | Confidence |
|---|---|---|---|---|---|
| **A1 — cotton-blend 12-month core (12GG, ~200–350 g)** | $0.9–1.6 | $1.3–3.5 | $1.2–2.8 | **≈ $4–9 / garment** | `[ESTIMATE, LOW-MED — blocked by [UNKNOWN] rate]` |
| **A2 — fine-acrylic value driver (7–12GG, ~250–450 g)** | $0.9–2.1 | $1.2–3.2 | $1.2–2.9 | **≈ $3.5–8 / garment** | `[ESTIMATE, LOW-MED — blocked by [UNKNOWN] rate]` |
| **A3 — merino/lambswool premium capsule (7–12GG, ~250–400 g)** | $2–13+ | $1.4–3.6 | $1.5–3.5 | **≈ $6–22+ / garment** | `[ESTIMATE, LOW — yarn-dominated, order-of-magnitude]` |

**Reading the three articles:**
- **A2 (fine acrylic) is the cost FLOOR** — Ludhiana's native fibre, best MOQ-flex, best lead-time. But its margin **lives or dies on the anti-pill finish surviving ICI Pilling-Box grade ≥3–4 after a home wash** (T2 §4): a cheap line (line 4) that, if cut, comes back as the **#1 return driver** and destroys the contribution margin downstream. Cheapest to make, easiest to ruin.
- **A1 (cotton-blend core) is the everyday seller** and carries the **lowest US duty** (~7% vs ~16% wool), so its *landed* advantage widens again in T9 — the article whose ex-factory and landed stories agree.
- **A3 (merino/lambswool premium) is the LEAST reliable band** — it's **yarn-dominated and order-of-magnitude** (the premium fibre price is a single stale anchor; **needs an importer quote before any margin use**), and the **duty asymmetry (~16% wool) hits it again in T9**, so its *landed* premium is wider still. Treat A3's number as a placeholder until an importer quotes the tops.

**The whole set carries one extra downgrade Analyst A could not add:** **no independent second analyst stress-tested these bands** (Analyst B absent). They are internally honest but **un-cross-examined** — A's good-faith illustration, not a reconciled two-source consensus. `[DOWNGRADED for process]`

---

## 4. Minimum batch + working-capital reality — where a profitable unit cost goes to die

A clean ex-factory number is a trap if you ignore **what you must buy before a single sweater sells** and **how long your cash is gone.** Two binding constraints, both from upstream, both larger than the unit cost itself at our scale:

**(a) The minimum batch is set by DYE-LOT MOQ, not by the garment.** The binding number is **~150–500 kg of dyed yarn per colour** (dye-house economic MOQ ~500–2,000 kg/colour; **+20–50% small-batch surcharge** below it) `[ESTIMATE; floor MED-HIGH, exact kg at a named dyer UNKNOWN — T2 §3b]`. At ~300–400 g/sweater, **200 kg ≈ 500–650 sweaters of ONE colour.** So:
- The launch line is **colour-starved, not style-starved** — plan only **~2–4 launch colours** across all three articles, share dye lots across styles, and reserve a *proven hero colour* before paying surcharges or graduating to job-work dyeing.
- **Garment/CMT MOQ per style-colour is ~100–300 pcs** at small Ludhiana units `[ESTIMATE, MED — T2]`. This is the smaller constraint; the **dye lot is what forces you to commit cash for 500+ pieces of a colour** whether you have demand for them or not.

**(b) Per-style fixed costs amortise over that tiny MOQ — and that's what makes micro-volume genuinely more expensive per piece than §3c's bands imply.** Sampling/PPS (proto→fit→PP), tech-pack, lab dips, AQL inspection are **per-style costs spread over the run** `[ESTIMATE, LOW]`. Over a 150-piece first run they can add a **meaningful uplift per garment**; over a 1,000-piece run they nearly vanish. **This is the single biggest reason the §3c bands understate true launch cost** — they are the *variable* conversion stack and do not yet carry amortised setup. (Quantifying this needs the real MOQ at a named unit — Open Q below.)

**(c) The working-capital trap is the killer the unit cost hides.** This is where a "profitable" sweater bankrupts a brand:
- **Cash leaves months before it returns.** Yarn is bought, dyed, knit, linked, finished, then shipped India→US ocean, then sold, then (D2C) often collected after returns settle. **Yarn-in → goods-ready ex-factory alone is ~6–12 weeks** `[ESTIMATE, MED — T2 §3c]`; ocean + sell-through + returns add many more (T9). The whole cash-conversion cycle is a **long, front-loaded cash-out.**
- **The seasonal calendar amplifies it.** The product is winter-weighted; committing a launch drop into the **Jul–Oct cluster peak** stretches lead time *and* working capital exactly when you can least afford it (T1 §4, T2 §3c). **Don't launch into the peak.**
- **The inverted-duty/GST drag adds to it (cash, not COGS).** Acrylic (MMF) inputs are taxed higher than the output, **trapping working capital in slow refunds** `[FACT — T1 §5b]`; this bites A2 specifically. Magnitude is **[UNKNOWN]** and it is a **T9 cash-cycle line, not a per-garment COGS add** — named here, quantified there.

**The owner's real first cheque is not "unit cost × pieces."** It is **(dyed-yarn MOQ for 2–4 colours) + (per-style setup for 3 articles) + (knit/link/finish on 500–1,500 pieces) + months of that cash being gone before revenue.** A sweater that costs $5 ex-factory still means **committing on the order of a few lakh rupees of yarn + setup before the first sale** `[ESTIMATE, LOW-MED — exact figure blocked by the [UNKNOWN] dye MOQ and job-work rate]`. **This, not the unit cost, is the number that sizes the launch and the cash runway.**

---

## 5. How unit economics scale with volume — feeding T9's P&L

The same garment has three different costs depending on volume, and the curve is the bridge into T9:

| Volume tier | What's true about per-garment cost | Make-vs-buy posture |
|---|---|---|
| **Micro (launch capsule, ~150–650 pcs/colour)** | **Highest per piece.** Variable stack from §3c **+ small-batch dye surcharge (+20–50%/colour) + per-style setup spread thin.** Real cost sits **above** the §3c bands. | **Buy everything** (subcontract knit/link/finish, bought-dyed yarn). High confidence. |
| **Small-batch repeat (a hero style/colour proven, low-thousands)** | **Falls toward the §3c bands.** Setup amortises; dye lots hit economic MOQ (surcharge gone); job-workers give better repeat rates. | Still **buy**, but begin pricing a **used Chinese knitting machine** for the *one* repeating program (T3 Phase 1). |
| **Sustained volume (one program filling a machine + linkers + a programmer)** | **Lowest per piece**, but only if utilisation is genuinely full. Owning knitting can beat subcontracting **once a specific repeating program runs at proven, sustained volume** — the make-vs-buy comparison the [UNKNOWN] job-work rate decides. | **Selectively make** (knitting first, **linking last**); never dye, never build a machine. |

**Three scaling rules to carry into T9:**
1. **Volume cuts cost through two channels, not one:** it (a) amortises per-style setup, and (b) clears the dye-lot small-batch surcharge. Both are **fixed/threshold effects**, so the per-garment cost **drops in steps, not smoothly** — the first big step is crossing the ~500 kg/colour economic dye MOQ.
2. **The make-vs-buy crossover IS the [UNKNOWN] job-work rate.** Owning knitting beats subcontracting only when (owned per-piece cost at your utilisation) < (subcontract per-piece rate). **We can't locate that crossover until we know the subcontract rate** — the same missing number as §1/§4. So volume strategy and the blocker are the *same* question.
3. **Ex-factory → landed multiplies, it doesn't add a little.** T9 stacks freight + duty (~7% cotton / ~16% wool) + insurance + broker + drayage + financing on top, and **duty is a percentage, so it scales with the ex-factory level** — which is why A3's premium and wool duty compound, and why A1's low-duty cotton core is the structurally friendliest article into the US.

**The clean hand-off to T9:** take the §3c bands as the *ex-factory input*, apply the §5 volume-tier adjustment (surcharge + setup at the brand's actual launch MOQ), then run the landed stack and the CAC/contribution test. **T9 owns whether the venture makes money; T4 owns only the honest factory-gate number and the cash it ties up.**

---

## Bottom line for the owner

1. **Ex-factory ≠ landed ≠ COGS-to-stock ≠ P&L.** This stack ends at the factory gate. Per T0, **contribution margin after CAC — not this number — decides viability** (T7/T9). Don't over-optimise the loom; the margin fight is downstream.
2. **Buy, don't make — at capsule scale (HIGH confidence, convergent T1/T2/T3).** Own design + brand + US import; subcontract knit + link + finish; buy yarn in-cluster on bought-dyed stock shades; rent the programmer. In-source a step **only at proven, repeating volume, linking last.** **Never own a dyehouse; never build a machine.**
3. **Yarn sets the level (well-anchored, ~$1–2.50/garment value/core); the [UNKNOWN] knit + LINK per-piece rate sets the uncertainty — and the same missing number decides make-vs-buy.** Get it and you halve every band *and* settle the crossover.
4. **Illustrative ex-factory bands (labelled, un-cross-examined): A1 cotton core ≈ $4–9, A2 acrylic value ≈ $3.5–8, A3 merino/lambswool premium ≈ $6–22+.** A2 is the **floor** (but its anti-pill finish is a cheap mandatory line that, if cut, returns as the #1 return driver); A3 is **yarn-dominated and least reliable** (needs an importer quote; wool duty widens its landed gap again).
5. **The real launch cheque is the dye-lot MOQ + per-style setup + months of tied-up cash — not the unit cost.** Plan **2–4 colours, not many styles**; expect a small-batch dye surcharge and thin setup amortisation to push true micro-volume cost **above** the §3c bands; **don't launch into the Jul–Oct peak**; treat inverted-duty drag as a **T9 cash-cycle cost, not a COGS add.**
6. **Unit cost falls in steps with volume** (clearing the dye economic MOQ, then amortising setup, then — only at sustained single-program volume — owning knitting). **One job-worker quote run converts this whole illustration into a real model and locates the make-vs-buy crossover.** It is the highest-leverage action in the segment.
7. **Don't add what you can't price.** Friction/speed-money (mechanism, never accusation) and inverted-duty drag are **named and held [UNKNOWN]** — episodic and cash-cycle costs for T9, not per-garment COGS lines.

---

## Open questions — what only on-the-ground quotes can close (floor, not ceiling)

1. **Ludhiana per-piece knit + LINK job-work rate** (₹/piece at 7GG and 12GG, for 300 g and 400 g pullovers) — **[UNKNOWN]; THE blocker.** Lines 2–3 are ~25–45% of the conversion subtotal; one quote run from 2–3 job-workers collapses them to [ESTIMATE, MED], tightens every band by ~half, **and locates the make-vs-buy crossover.** Single highest-leverage data acquisition in the segment. *(On-the-ground only — never fabricate; T1 Open Q#1, T3 Open Q#2.)*
2. **Real small-run garment/CMT MOQ at a named unit (~100–300 pcs)** — `[ESTIMATE, MED]`; sets per-garment fixed-cost amortisation, hence the true micro-volume uplift over §3c.
3. **True dye-lot MOQ + small-batch surcharge at a *named* Ludhiana dyer** — the number that sizes the **launch colour count and the first cheque** (`[ESTIMATE]`, exact kg [UNKNOWN] — T2 Open Q#2).
4. **Per-style fixed costs** (sampling/PPS, tech-pack, lab dips, AQL) amortised over MOQ — `[ESTIMATE, LOW]`; quantifies how much micro-volume sits **above** the §3c bands.
5. **Live 2025–26 yarn ₹/kg per fibre/count** — `[ESTIMATE]` bands on stale points (T2 §3a); **premium A3 rows need an importer quote** before any margin use.
6. **Inverted-duty / GST working-capital drag on acrylic (A2)** — magnitude **[UNKNOWN]**; a T9 cash-cycle cost, flagged here, quantified there.
7. **Compliance-premium *base*** — how much labour the brand carries on-roll vs the job-worker — **[UNKNOWN]**; sets what the ~15%+ actually multiplies.
8. **PSPCL industrial power tariff + the job-worker's power share folded into the rate** — **[UNKNOWN]** (T1 Open Q#4).
9. **CAD/flat-knit programmer cost/availability** — `[ESTIMATE, MED]`; a payroll line if owning knitting, a contracted-rate [UNKNOWN] if subcontracting (T3 §6).
10. **Landed used-machine quotes** (Chinese/Cixing first; ex-Stoll parts-via-Obertshausen diligence) — wide `[ESTIMATE, LOW-MED]`; the #1 input gap **the moment** make-vs-buy tips toward owning knitting (T3 Open Q#1).
11. **The friction/"speed-money" magnitude** — **[UNKNOWN] by design**, mechanism not accusation; will only ever be [ESTIMATE: discussion], never a quotable per-garment number (T1 §5c).
12. **No independent challenge to these bands or the make-vs-buy boundary (process [UNKNOWN]).** `analyst_b_make_vs_buy.md` was never written; the verdict is reconstructed-consensus from T1/T2/T3 — robust, but the adversarial pressure of a real A/B merge is absent. The neutralize critic should press §2 and §3c harder than a two-source result. `[UNKNOWN — missing second-analyst challenge]`

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/mfg_economics/DEEPDIVE.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `SCOPE_GLOSSARY.md`; `ludhiana/DEEPDIVE.md`; `yarn/DEEPDIVE.md`; `machines/DEEPDIVE.md`; `mfg_economics/reconciled.md`; `mfg_economics/analyst_a_cost_model.md`
- **Could NOT read (named input absent):** `mfg_economics/analyst_b_make_vs_buy.md` — **does not exist on disk** `[FACT — file absent]`; the make-vs-buy half is therefore reconstructed-consensus from T1/T2/T3 and flagged as a first-class process [UNKNOWN].
- **Delivered (owner-facing):** (1) the boundary — ex-factory conversion cost, explicitly not landed/COGS/P&L; (2) the two-number frame — yarn sets the level (anchored), the [UNKNOWN] knit/LINK rate sets the uncertainty; (3) make-vs-buy verdict + stage-by-stage ladder + the under-pressed honest counter; (4) the per-article cost stack — 10 tagged lines, external shape-only cross-check, and the three illustrative ex-factory bands (A1≈$4–9 / A2≈$3.5–8 / A3≈$6–22+); (5) minimum-batch + working-capital reality (dye-lot MOQ → 2–4 colours, per-style setup, the months-long cash trap, seasonal peak, inverted-duty drag) — the real first cheque, not the unit cost; (6) how unit economics scale in steps with volume and hand off to T9's P&L; (7) Bottom line + a 12-item open-questions ledger.
- **Honesty discipline:** Ludhiana knit/LINK per-piece rate held **[UNKNOWN]** as THE blocker (never fabricated); all cost bands `[ESTIMATE]` with confidence; conversion midpoint and trade fragments **[DOWNGRADED]** to widths; A3 premium row kept order-of-magnitude/LOW; compliance loading applied only to the labour-bearing portion with its base **[UNKNOWN]**; friction/speed-money kept mechanism-not-accusation with amounts **[UNKNOWN]**; inverted-duty drag named as a T9 cash-cycle cost, not a per-unit add; the missing-second-analyst gap flagged as a first-class process **[UNKNOWN]** for the neutralize critic.
- **Bottom line:** buy-don't-make at capsule scale (convergent, HIGH); yarn sets the level and the [UNKNOWN] knit/LINK rate sets the uncertainty on both the cost stack and the make-vs-buy crossover; ex-factory bands A1≈$4–9 / A2≈$3.5–8 / A3≈$6–22+ are labelled, un-cross-examined illustrations; the real launch constraint is dye-lot MOQ + per-style setup + a months-long working-capital trap, not the unit cost; one job-worker quote run unblocks the whole segment and feeds T9's full P&L.
</content>
</invoke>
