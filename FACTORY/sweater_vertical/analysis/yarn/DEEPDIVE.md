# Yarn & Fibre — Owner Deep-Dive (T2)

**For:** the owner of a small, AI-designed D2C knitwear brand buying yarn in/through Ludhiana and selling sweaters in warm-ish **California / Turlock** (hot summers, a narrow true-cold window).
**Question this answers:** Which fibres and counts for our **3-article California line**, and why? Where do we buy the yarn (Ludhiana base vs imports), and what are the realistic **MOQ / price / lead-time** bands? How do we spec quality so we don't get **pilling and returns**? And do we buy **dyed yarn or outsource the dye**?

**How to read the tags (binding honesty discipline — same as Ludhiana/T1):**
- **[FACT]** = cited public source / true-by-definition unit identity.
- **[ESTIMATE]** = a reasoned estimate with method + basis + confidence stated; carries the literal word *estimate*.
- **[UNKNOWN]** = we genuinely don't know and won't fake it. Coverage here is a **floor, not a ceiling**.
- **Every price / MOQ / lead-time band in §3 is `[ESTIMATE]`** — bands stand, the underlying point-anchors are stale. Do not feed a point into a margin model without a live quote.

> **The one sentence to remember.** *Yarn price is the part of this that's small and well-anchored (~$1–2.50 of fibre per sweater) — so it is **not** what governs the line; what governs the line is the **dye-lot MOQ per colour (≈150–500 kg ⇒ only ~2–4 launch colours)** on the commercial side and a **contractual anti-pill spec gated at ICI Pilling-Box grade ≥3–4 after a home wash** on the quality side. Buy the everyday fibres in-cluster on bought-dyed stock shades, import only the premium tops, and never own a dyehouse.*

---

## 1. Which fibres and counts for our 3-article California line — and why

### 1a. The climate inverts Ludhiana's instinct
Turlock / Central Valley California sells **light, mid, and transitional knits most of the year (≈Sept–April)** and true heavy wool only **Nov/Dec–Feb**. `[ESTIMATE: climate-reasoning; direction HIGH, month-bands MED]` That single fact flips the product weighting away from Ludhiana's winter-woollen default: our **core must be lightweight fine-gauge**, and **wool/merino is a thin seasonal premium capsule, not the base.** Ludhiana sells us its *product know-how*, not its product *mix* — we deliberately don't copy the mix.

### 1b. The three-article line (fibre × count × gauge × weight — all four chosen together)
The line is built from the fibre tier ladder both analysts independently arrived at (climate+margin from one side, quality from the other) `[ESTIMATE: design-framing, MED-HIGH]`. **Choose the quadruple, never a vague "midweight"** — fibre, resultant count, gauge (GG), and grams-per-garment must agree (GLOSSARY cross-team rule #4):

| Article | Fibre | Gauge / count | Weight band | Role / why |
|---|---|---|---|---|
| **A1 — the 12-month core** | **Cotton-blend** (cotton-rich, e.g. cotton/acrylic or cotton/modal) | **12GG**, fine (high-Ne cotton / fine resultant) | ~200–350 g | The everyday seller. Breathable, no prickle, washable; carries the warm-climate calendar where heavy wool sells few months. Lowest US duty (cotton 6110.20 ≈7%). `[FACT — duty line, USITC]` |
| **A2 — the value driver** | **Fine acrylic / acrylic-rich blend** | **7–12GG** (2/30, 2/32 Nm-class; finer to 12GG) | ~250–450 g | Ludhiana's base fibre = best cost + MOQ-flex + lead-time. Bright-dyeing, machine-wash, moth-proof. **Pills** — so this article lives or dies on the anti-pill spec (§3). |
| **A3 — the seasonal premium capsule** | **Merino or lambswool** (thin Nov–Feb drop; optional cashmere-blend halo) | **7–12GG** (merino 2/48 Nm-class) | ~250–400 g | The price-justifying, margin-lifting halo. Merino "fine/superfine" (≤19.5 µm) is the soft-but-wearable sweet spot; lambswool is the below-merino soft-wool grade. Even 5–10% cashmere in a blend lifts perceived value. Demoted to a thin capsule by the climate. |

**Why these and not others:** acrylic is the cost floor and Ludhiana's native strength; cotton fits the warm market and the calendar; merino/lambswool is the premium word that justifies price *without* committing the base to heavy wool the climate can't sell. Viscose/modal and a few % **nylon** are **blend ingredients**, not standalone articles — nylon especially is the cheapest pilling/abrasion lever in a soft wool or cashmere blend (a few % sharply improves both). `[ESTIMATE: industry-lore, HIGH]`

### 1c. Count → gauge, stated cleanly (with the one caveat)
- **7GG** ≈ resultant **Nm 6–8** (2/30, 2/24) → the commercial-default mid pullover (~350–550 g). `[FACT — GLOSSARY §3]`
- **12GG** = our core fine tier (2/48 Nm-class for merino; high-Ne cotton) → the warm-CA "elevated basics" weight. `[FACT — GLOSSARY §3]`
- **Caveat, carried from the reconciler:** do **not** quote "2/48 Nm = 12GG" as a rule. On *resultant* count, 2/48 Nm (~24 Nm) is barely finer than a 2/30 Ne cotton (~25 Nm) — the move to a true 12GG hand comes from **finer singles + fewer ends fed + a tighter knit, confirmed at sampling**, not from the folded-count label. `[ESTIMATE: MED]` Misreading folded notation (2/30 vs 1/30 is a doubling error in fabric weight) is the classic spec mistake — confirm **singles count + ply + ends fed** before judging weight.

### 1d. The blend ratio is the brand's #1 controllable lever
The ratio on the care label moves **cost, hand, warmth, care, perceived value, and US import duty simultaneously** (wool 6110.11 ~16% vs cotton 6110.20 ~7%). `[FACT — duty lines]` It's also a cheat-and-duty-risk surface, so it must be **written into the PO with a tolerance** — e.g. *"70/30 ± 3% by mass, verified by fibre-composition test."* Blend drift is both a quiet cost-down by the supplier and a duty-misdeclaration exposure for us as importer of record. `[ESTIMATE: industry-lore, HIGH]`

---

## 2. Where to buy — Ludhiana base vs imports

**The split is the same logic both analysts reached: buy local, import only where the quality ceiling justifies the cost/MOQ/lead-time penalty.** `[ESTIMATE: HIGH on the split-logic; FACT on the player roster + import origins]`

**In-cluster (Ludhiana / domestic) — the default:**
- **Acrylic, acrylic-rich blend, melange, lambswool, cotton-spun, blended-spun.** Cost, MOQ-flex, shade cards/restock, and lead-time all favour buying inside the cluster.
- The integrated spinners — **Vardhman, Sportking, Nahar, Oswal** — carry count availability, shade cards, restock, and (importantly for §3) **in-house lab QC** that can natively hit USTER-percentile clauses and folded-count specs. `[FACT — player roster]`
- **Micro-spinners** flex MOQ further but **can't self-test** → for anything quality-gated, push the order to a USTER-equipped spinner or budget third-party testing.

**Imported — only the premium ceiling:**
- **Merino / superfine wool tops** = imported **AU/NZ**; warehousing importers (RNB-class) hold stock incl. in Ludhiana, so you don't have to import raw fibre yourself. `[FACT — import origins]`
- **Cashmere** = imported; use as a blend **%**, not 100%, to lift the halo article without the luxury price or the heavier pilling/finishing burden of pure cashmere.
- **Fine filament viscose/modal and fine nylon** = imported and blended in for drape (viscose/modal) and for pill/abrasion resistance (nylon).

**Rule of thumb:** if it's the A1 cotton-core or A2 acrylic-value article, it's local. If it's the A3 merino/cashmere capsule, part of it is imported (or bought from a local house that has already imported the tops).

---

## 3. The realistic MOQ / price / lead-time bands `[ESTIMATE]`

All bands below are **`[ESTIMATE]`** with method = triangulated public reportage + the "yarn ≈50–60% of cost" anchor inherited from T1. **The bands stand; the point-anchors are stale** (2018 merino fibre, 2022 acrylic, 2024 cotton) — refuse the points as current, get live importer/mandi quotes before any margin model.

### 3a. Price (₹/kg ex-mill/mandi, ex-GST, mid-2020s)
| Yarn | `[ESTIMATE]` ₹/kg | ≈ USD/kg | Confidence | Note |
|---|---|---|---|---|
| Acrylic spun (2/30–2/32 Nm) | **~₹260–330** | ~$3.1–4.0 | MED | anchor: Fibre2Fashion Ludhiana 2/32 Nm ~₹280–290 (**2022 point STALE → band**) |
| Acrylic-rich blend | ~₹280–380 | ~$3.4–4.6 | MED→LOW | acrylic band + blend premium |
| Cotton spun (Ne 30) | ~₹260–320 | ~$3.0–3.8 | MED | Tirupur Ne30 combed ~₹268–276 (**Sept-2024**); FOB-India ~$2.6–2.8/kg (2025) |
| Lambswool / wool-blend spun | ~₹500–1,100 | ~$6–13 | **LOW** | no clean public point — band only |
| Merino spun (fine, 2/48 Nm) | ~₹1,400–2,800 | ~$17–34 | **LOW** | 2018 imported-merino *fibre* ₹1,253/kg = stale floor for fibre alone |
| Cashmere / cashmere-blend | ~₹6,000–20,000+ (100%) | ~$70–240+ | **LOW (order-of-mag)** | luxury, low-yield; use as blend % |

**Translation to the garment:** at ~300–500 g/sweater, value/core yarn is **~$1–2.50 of fibre per garment**. This is *small and well-anchored* — which is exactly why **yarn price is not the binding constraint.** The premium rows (lambswool/merino/cashmere) are **order-of-magnitude only** and must not enter a margin model without an importer quote. `[DOWNGRADED — single stale anchor or none]`

### 3b. MOQ — **the binding commercial constraint** (three different MOQs, don't conflate them)
- **Greige / undyed yarn:** small — **tens of kg** for a relationship buyer, up to ~a carton/quintal; lower still mill-direct. `[ESTIMATE, MED]`
- **Dyed yarn per shade — THE number that sizes the brand:** floor **~150–500 kg/colour**; dye-house economic MOQ **~500–2,000 kg/colour**; **+20–50% small-batch surcharge** below it. ⇒ at ~300–400 g/sweater, **200 kg ≈ 500–650 sweaters of one colour ⇒ a micro-capsule affords only ~2–4 colours** without surcharge or dead stock. `[ESTIMATE; few-hundred-kg-per-colour floor MED-HIGH; exact kg at a named dyer UNKNOWN]`
- **Garment / CMT MOQ per style-colour:** **~100–300 pcs** at small Ludhiana units (the job-work mesh flexes below long-run Bangladesh/Vietnam). `[ESTIMATE, MED]`

> **The design constraint that falls out of this:** the line plan is **colour-starved, not style-starved.** Plan ~2–4 launch colours across the 3 articles, share dye lots across styles where possible, and reserve a *proven hero colour* before paying small-batch surcharges or graduating to job-work dyeing. The conclusion is **not sensitive** to the exact floor within 150–500 kg, so it survives the uncertainty.

### 3c. Lead time (ex-factory; ocean India→US is separate, carried to T9)
- In-stock greige: **days.** **Dyed-to-shade: ~2–4 weeks** (lab dips + dye-lot scheduling).
- Sampling (proto→fit→PP): **~2–5 weeks** (fully-fashioned **linking is skill-bound** — the linker bottleneck inherited from T1 §4).
- Bulk knit→link→finish (small run): **~3–6 weeks**, longer in the **Jul–Oct cluster peak** — **don't commit a launch drop into the peak.**
- **Yarn-in → goods-ready ex-factory: ~6–12 weeks** for a small dyed-shade order. Add ocean (T9) for landed. `[ESTIMATE, MED; seasonal-contention direction MED-HIGH]`

---

## 4. How to spec quality so we don't get pilling / returns

**Pilling is the #1 post-purchase complaint and the main return driver**, and acrylic (our A2 value article) is the worst offender. We defend against it with **numbers in the PO, not adjectives** — and we accept it isn't real until it passes a wash-test gate.

### 4a. The five spinning dials (set each to the article's tier)
Quality is five mostly-independent dials, each mapping to one thing the customer feels:
1. **Staple length** — longer → less pill, stronger, smoother. *Name a minimum staple (mm) or fibre grade in the PO, not just the count.*
2. **Twist** (TPM/TPI + S/Z, or twist-multiplier K) — the strength-vs-softness optimum; under-twist pills and breaks, over-twist feels wiry and causes spirality (the twisted-side-seam defect). Knitwear-soft ≈ K~3; **require "balanced ply."** *The dial most often left blank in a naive PO.*
3. **Ply** — 2-ply binds better than singles (singles skew/torque a flat panel — a QC reject). *Confirm singles count + ply + ends fed* (2/30 vs 1/30 is a doubling error).
4. **Fibre fineness** (micron / denier-per-filament) — **the counter-intuitive dial: finer = softer but pills MORE and is weaker.** *Spec a micron ceiling (e.g. "≤19.5 µm") not the word "merino"; for acrylic spec an anti-pill / higher-dpf grade (~2.5 dpf), not the cheapest 1D.*
5. **Blend ratio ± tolerance** — the margin/positioning/duty lever (§1d); a few % nylon in a soft wool/cashmere blend is the cheapest pill+abrasion improvement.

### 4b. Pilling defence, in order of control
Causes (mnemonic): **short, fine, low-twist, loose-gauge, hairy** yarn. Levers, most-controllable first:
longer staple → higher twist → ply → **anti-pill / higher-dpf grade (the single biggest lever for acrylic)** → a few % nylon (for soft wool/cashmere) → lower hairiness (compact / air-jet spun) → **tight gauge + high stitch density** (don't let a loose knit undo a low-pill yarn — carried to T3/T8) → finishing (singe / cellulase bio-polish / heat-set — T6).
*Vendor case studies claim anti-pill grades cut returns ~30–50% and compact spinning cuts shedding ~30% — keep the **direction** (these levers materially help, HIGH), **distrust the exact %** (single-vendor, not independent trials). Don't promise a customer a number.* `[DOWNGRADED — direction HIGH, magnitude LOW-MED]`

### 4c. Gate with numbers, not adjectives — and gate as a contract, not a lab
We are a micro-brand, so the posture is **contractual QC, not in-house metrology** (a USTER tester is 6-figure-USD class; exact list price [UNKNOWN]). The gates:
- **Grade yarn lots against USTER percentiles on CV% + IPI** — state the thin (−50%) / thick (+50%) / nep (+200%) sensitivities; add hairiness band, min tenacity, count-CV ceiling. **Demand 25% USTER on mainstream acrylic/cotton — NOT 5%** (5% USTER on a mainstream lot is unquotable or luxury-priced). `[ESTIMATE: industry-lore, MED-HIGH]`
- **Gate every "anti-pill" claim on ICI Pilling Box (ISO 12945-1) grade ≥ 3–4 after ≥1 home-wash cycle.** Post-wash matters because washing drops the grade, and the post-wash grade is what predicts returns.
- **For micro-spinners who can't self-test:** push to a USTER-equipped spinner (Vardhman/Nahar/Sportking) or budget a third-party lab spot-check per lot.

### 4d. The enforceable spec sheet (attach to every yarn PO)
> fibre + blend (micron ceiling / anti-pill dpf, ±% by mass, fibre-composition verified) · count (Ne/Nm folded + Tex in brackets; singles / ply / ends-fed) · twist (TPM + S/Z or K, "balanced ply") · min staple · USTER clause (≤25% USTER on CVm% + IPI + sensitivities; hairiness band; min tenacity; count-CV ceiling) · pilling acceptance (ICI ≥3–4 post-wash) · colour (shade std, lot-to-lot ΔE, fastness minima) · the GG + GSM / g-per-garment it must knit at · AQL sampling + reject rule · commercials (Incoterm, MOQ, lead time, price — to T9).

This contractual loop is exactly what the **T5 "thread-checker"** is meant to close cheaply (the device that grades the point-defect + pilling subset and sends a code back to the supplier).

---

## 5. Dyed-yarn vs outsource-dye — the call

**Call: launch on bought-dyed stock shades; never own a dyehouse; graduate to greige + job-work dye only for a proven hero colour.** `[ESTIMATE: judgment; "no captive dyehouse" HIGH]`

**Why no dyehouse (inherited, non-negotiable from T1 §5b/§6):** all three Ludhiana CETPs were found non-compliant, CPCB levied crore-scale penalties, the NGT is reviewing the cluster, and dyeing is the **highest-regulatory-risk node in the entire chain** with real closure exposure. A small new entrant owning that node is taking a risk wildly out of proportion to a 3-article line. `[FACT — enforcement; ESTIMATE: judgment, HIGH]`

**The two options, and when each wins:**
- **(A) Buy dyed yarn (stock shades) = the launch default.** Simplest, lowest risk, fastest; the dye-lot risk and the NGT/closure risk sit on the supplier. Cost: less colour control, you're limited to the spinner's shade card, and you carry lot-to-lot shade-match risk (mitigated by the ΔE / fastness clauses in §4d). **This is where we start.**
- **(B) Greige + outsource (job-work) dyeing = graduate to it, selectively.** Buy undyed yarn and pay a job-work dyer per the conversion charge. Wins only once a **specific colour is a proven hero** worth the dye-lot MOQ (§3b) and the extra lead time and shade-consistency management. It buys faster trend response and exact colour, at more shade risk — the standard later-stage-dye trade-off.

Either way the **dye-lot MOQ per shade (≈150–500 kg ⇒ 2–4 colours)** is the binding number, not the dye method.

---

## Bottom line for the owner

1. **Three articles, climate-first:** **A1 cotton-blend 12-month core (12GG)**, **A2 fine-acrylic value driver (7–12GG)**, **A3 thin merino/lambswool premium capsule (Nov–Feb)**. This *inverts* Ludhiana's winter-woollen default because Turlock sells light/transitional knits most of the year. **[ESTIMATE: design-framing, MED-HIGH]**
2. **Yarn price is not your problem** — it's ~$1–2.50 of fibre per sweater and well-anchored. **Two things actually bind:** the **dye-lot MOQ (~150–500 kg/colour ⇒ only ~2–4 launch colours)** and a **contractual anti-pill spec (ICI ≥3–4 post-wash).** Build the line plan colour-starved, not style-starved.
3. **Buy local, import only the premium:** acrylic/blend/cotton/lambswool from in-cluster spinners (Vardhman/Sportking/Nahar/Oswal) on **bought-dyed stock shades**; import only merino tops/cashmere (or buy from a local house that already imported them).
4. **Spec quality as a contract, not a lab:** five spinning dials (staple, twist, ply, fineness, blend ratio) written into a **count-anchored, 25%-USTER-claused, ICI-pilling-gated, fibre-composition-verified, AQL-sampled** PO. **25% USTER is the mainstream bar — never demand 5% on a mass acrylic lot.** This is the gap the T5 thread-checker targets.
5. **Dye decision:** launch on **bought-dyed stock shades**, **never own a dyehouse** (NGT/closure risk), graduate to greige + job-work dye only for a proven hero colour.
6. **Watch the stale numbers and the calendar:** every price/MOQ/lead-time is an `[ESTIMATE]` band on stale points — get live importer + mandi quotes before any margin model; and don't commit a launch drop into the **Jul–Oct cluster peak.**

---

## Open questions — what only on-the-ground sourcing can close

1. **Current 2025–26 spot price per fibre/count (₹/kg) — `[UNKNOWN]` precise** (bands only; mandi/subscription data hardens). → T4 cost model.
2. **True dye-lot MOQ at a *named* Ludhiana job-work dyer — `[UNKNOWN]`; the single number that sets the brand's colour count.**
3. **Premium spun-yarn prices (lambswool / merino / cashmere) in India — `[ESTIMATE, LOW]`; needs an importer quote** before any margin use.
4. **Real small-run garment / CMT MOQ at a named unit — `[ESTIMATE, MED]`.** → T4.
5. **Availability of compact-spun / anti-pill acrylic grades from Ludhiana spinners vs having to import the fibre — `[UNKNOWN]`.**
6. **USTER-tester / third-party lab cost and accessibility near Ludhiana to a micro-buyer — `[UNKNOWN]`.** → T2/T5.
7. **Inverted-duty / GST working-capital drag on MMF (acrylic) yarn — flagged, not quantified;** specifically taxes our acrylic-leaning base. → T4/T9.
8. **12GG fine-gauge capacity abundance in-cluster (vs 7GG default), incl. CAD/flat-knit programmer scarcity — `[UNKNOWN]`.** → T3/T8.
9. **Feasibility/cost of a cheaper-than-USTER "thread-checker" grading the point-defect + pilling subset — `[UNKNOWN by design]`;** parameters defined (CV%/IPI/hairiness/count-CV/tenacity + pilling/percentile verdict). → T5.
10. **Default-fibre / positioning call (acrylic vs cotton vs merino premium) — a brand decision (T7/T8)** that flips which yarn-stage treatments apply (anti-pill grade vs mercerise / gas-singe).
11. **Quantified pilling-grade × blend/twist/gauge curves for our exact constructions — `[UNKNOWN until sampling]`;** needs PPS swatch testing. → T8/T5.

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/yarn/DEEPDIVE.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `FACTORY/sweater_vertical/analysis/SCOPE_GLOSSARY.md`; `FACTORY/sweater_vertical/analysis/ludhiana/DEEPDIVE.md`; `FACTORY/sweater_vertical/analysis/yarn/reconciled.md`
- **Delivered (owner-facing):** (1) the **3-article line** — A1 cotton-blend 12GG core, A2 fine-acrylic value, A3 merino/lambswool seasonal capsule — with the climate-inversion reasoning, the fibre×count×gauge×weight quadruple, the count→gauge map with the "don't quote 2/48 Nm=12GG" caveat, and the blend-ratio/duty lever; (2) **where to buy** — local for acrylic/blend/cotton/lambswool, import only merino tops/cashmere; (3) **MOQ/price/lead-time bands** all `[ESTIMATE]` — yarn ~$1–2.50/garment, **dye-lot MOQ ~150–500 kg/colour ⇒ 2–4 colours = the binding constraint**, ~6–12 wks yarn-in→ex-factory; (4) **quality spec** — five spinning dials, pilling defence order, contractual 25%-USTER + ICI-≥3–4-post-wash gate, the enforceable PO sheet; (5) **dyed-vs-outsource** — launch on bought-dyed stock shades, never own a dyehouse, graduate to job-work dye only for a hero colour.
- **Honesty discipline:** every price/MOQ/lead-time held as `[ESTIMATE]` with confidence; premium-fibre price rows + vendor pilling %s + the "2/48 Nm=12GG" exemplar carried as `[DOWNGRADED]`; stale 2018/2022/2024 anchors flagged (bands kept, points refused as current); count conversions left untagged as identities; no fabricated precise figures; coverage stated as a floor with 11 carried open questions for on-the-ground sourcing.
- **Bottom line:** climate-first 3-article line; yarn price isn't the constraint — dye-lot MOQ (2–4 colours) and a contractual anti-pill spec are; buy local + import only the premium; spec quality as a numeric contract (25% USTER, ICI ≥3–4 post-wash); launch on bought-dyed stock shades and never own a dyehouse.
