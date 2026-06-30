# Knitting Machines — Owner Deep-Dive (T3)

**For:** the owner of a small, AI-designed D2C woollen/acrylic knitwear brand sourcing in Ludhiana and selling in California / Turlock.
**Question this answers:** What machines actually make our sweaters? Should a micro-brand **buy used, buy new, or subcontract knitting** — and at what cost? Is it smarter to **design our own machine or buy-and-program** an existing one? Where do **reverse-engineering / own-parts / AI** genuinely help, and where are they hype? And what is the **programmer-skill bottleneck** that everything hinges on?

**How to read the tags (binding honesty discipline):**
- **[FACT]** = cited public source.
- **[ESTIMATE]** = a reasoned estimate — method + basis + confidence stated; carries the literal word *estimate*. **Every price, throughput, and lead-time band below is an [ESTIMATE]**, never fake-precise.
- **[UNKNOWN]** = we genuinely don't know, and we are not going to fake it. Coverage is a **floor, not a ceiling.**

> **The one sentence to remember.** *For a micro D2C capsule brand the knitting machine is almost never the thing to buy and never the thing to build — the binding constraints are the **CAD/flat-knit programmer** and the **linker**, not the iron. So the default is: **subcontract knit + link + finish into the Ludhiana job-work mesh, rent/contract the scarce programmer skill, and point AI at the program/maintenance/QC layers** — buying used iron only when one style runs at proven volume, and building your own machine never.* `[ESTIMATE: judgment, HIGH — two independent analysts reached this separately]`

---

## 1. What machines actually make our sweaters

A finished fully-fashioned (FF) sweater passes through three machine families plus finishing. Know which does what, because the **make-vs-buy boundary** sits at a different place for each.

**1. The knitting machine — the flat-bed (V-bed).** Two opposed needle beds knit flat panels back-and-forth and shape them by increasing/decreasing stitches (this is what makes a panel *fully-fashioned* — knit to final shape, minimal waste). This is the core sweater technology our whole chain centres on. The computerised versions come from two historical names — **Shima Seiki (Japan)** and **Stoll (Germany)** — plus a fast-rising **Chinese tier led by Cixing** (which also owns Switzerland's Steiger). `[FACT — vendor identities; SCOPE_GLOSSARY §3]`
   - *Sub-family worth flagging:* **whole-garment machines** (Shima **WHOLEGARMENT®**, Stoll **knit&wear®**) knit the **entire seamless garment in one piece** — no panels, **no linking at all**. Strategically huge because it kills the linker bottleneck (below), but it is the most expensive class with the steepest programming. A later/volume decision, never a launch one.
2. **Circular knitting** — knits a continuous jersey *tube* fast for later cut-&-sew. High throughput, cheap per metre, but no shaping → more cut waste, more casual product. Not our primary path for shaped sweaters, but relevant if a style is cut-&-sew jersey.
3. **The linker — the seaming machine, and the throughput bottleneck.** After FF panels are knit, a **linking machine** joins them edge-to-edge **loop-to-loop** for the flat, near-invisible, stretchy seam that *defines* a quality FF sweater. This is the **eyesight-intensive, hand-skilled, piece-rate step that sets a small line's daily capacity** — not the knitting. T1 already flagged skilled linkers as the hardest role to re-hire after the lean season. `[FACT — T1 Ludhiana DEEPDIVE §4]`
4. **Finishing machines** — steam press / boarding / washing / softening that turn a harsh off-machine panel into retail product (SCOPE_GLOSSARY §4).

**The load-bearing point:** of these, the **linker — not the knitting machine — caps how many sweaters a small line ships per day**, and the **knitting machine's value lives in its program, not its metal** (§5). Hold both facts; they drive every decision below.

---

## 2. Buy used vs buy new vs subcontract — the straight answer with cost bands

**The decision, in priority order, for a micro FF-sweater line:**

| Phase | What you do for knitting | Capital | Why |
|---|---|---|---|
| **Phase 0 — launch / capsule** | **Subcontract** knit + link + finish into the Ludhiana job-work mesh. Optionally buy **one cheap used hand-flat (~$200–1,500)** for in-house sampling/iteration only. Spend instead on a **knit-CAD seat + a contracted programmer + sampling.** | Low tens-of-thousands USD `[ESTIMATE, MED]` | The agglomeration *is* the infrastructure — the one Ludhiana moat that helps a tiny brand (T1 §3). Small-batch/iteration fit is Ludhiana's genuine edge for our use case (T1 §6). Capital stays free for design + CAC. |
| **Phase 1 — one style proven at volume** | **Buy 1–2 USED machines — Chinese (Cixing-class) first.** Add a CAD seat, spares, and linkers to match output. | ~$15k–60k per machine landed `[ESTIMATE, LOW–MED — condition/model-specific; get quotes]` | Utilisation finally justifies the machine + programmer + linker payroll. **Used, not new**, because the used pool is deep and (post-Stoll exit, §4) deepening. |
| **New machine** | Only **Shima** (premium, with India support) or **Chinese new** (Cixing, cost tier). | Shima FF new ~$60k–140k; Chinese new ~$8k–25k; whole-garment ~$120k–250k+ `[ESTIMATE, LOW–MED; precise new price UNKNOWN — vendors quote per config]` | A small brand rarely needs new; new is a volume/quality-benchmark purchase, not a founding move. **Stoll new-build is off the table (§4) → Shima or Chinese only.** |

### Consolidated price bands — all [ESTIMATE], get landed quotes before committing a cost model
*(Two analysts published partly-overlapping bands; where they diverged we kept the **wider range and the lower confidence** rather than splitting into false precision.)*

| Machine class | New (per machine) | Used / reconditioned | Confidence |
|---|---|---|---|
| Hand-flat / domestic (sampling) | ~$500–3,000 | ~$200–1,500 | MED |
| Power-flat (semi-auto) | ~$3,000–10,000 | ~$1,000–5,000 | MED |
| **Chinese computerised flat (Cixing-class)** | ~$8,000–40,000+ | **~$4,000–15,000** | MED |
| **Shima / Stoll computerised flat (standard FF)** | ~$60,000–140,000 | ~$8,000–60,000+ | LOW–MED |
| Whole-garment (Shima SWG-N2 / MACH2S) | ~$120,000–250,000+ | ~$20,000–90,000 | LOW–MED |
| Linking (rotary, hand/electric) | ~$1,500–8,000 | ~$500–4,000 | MED |
| Automatic Italian linker (Complett/Exacta-class) | ~$15,000–40,000+ | ~$5,000–20,000 | LOW |
| Finishing (steam press / boarding / wash) per unit | ~$2,000–20,000 | ~$1,000–8,000 | LOW–MED |

> **Over-confidence warning (explicit).** Tidy single figures like "a ~$15k used Chinese machine" are **decision-framing illustrations, not quotes.** Used capital-goods price is condition/model/gauge-specific; the honest band is **wide and LOW–MED confidence.** **Get real landed quotes (machine + freight + duty + install) before any cost model commits.** This is the #1 input gap into T4.

**Import cost on the machine itself (into India):** knitting machines = **HS heading 8447**, carrying **~7.5% BCD (+ ~0.75% surcharge) then 18% IGST** on assessable value. `[FACT — HSN 8447, 18% IGST; ESTIMATE on exact BCD sub-heading, confirm at purchase]` Plus hidden costs: install, power/compressed-air infrastructure, spares, and — for any ex-Stoll buy — the parts-via-Obertshausen service question (§4).

**Throughput — order-of-magnitude planning bands only `[ESTIMATE, LOW–MED]`:** a computerised flat knits ~one sweater's panels in ~20–45 min (~12–25 garments'-worth of panels/machine/shift); a linker does ~one sweater's joins in ~10–25 min (~20–45 sweaters/linker/shift) — **which is exactly why linking, not knitting, sets the daily ceiling.** Whole-garment is slower per garment (~6–15/machine/shift) **but deletes the entire linking step**, which is where its strategic value lives.

**Why subcontract wins at capsule scale:** even a "tiny" owned line is a **mid-six-figure + skilled-staffing commitment before a single garment sells** — illustratively ~2–4 knitters + 2–4 linkers + finishing + ≥1 programmer for ~30–60 sweaters/day `[ESTIMATE: design illustration, LOW–MED — must be rebuilt bottom-up against a real Ludhiana unit in T4]`. A deliberately tiny, iterating capsule brand usually won't hit the utilisation that justifies that. Buy-used becomes correct only when a **specific repeating program at sustained volume** fills the machine, the programmer, and the linkers.

---

## 3. Design-our-own vs buy-and-program — the straight answer

**Buy-and-program. Decisively. Building your own machine is the single highest-confidence "do not" in this entire segment.** `[ESTIMATE: engineering + business judgment, HIGH]`

A computerised V-bed machine's *defining* subsystems — the **needle bed, the needles, the cam systems, the electronic needle-selection, the controller firmware, and the knit-CAD software** — are all very hard, decades-deep, and exactly the parts you cannot skip or fake. **You can build the easy 20% (frame, take-down rollers, guards) and still not have a knitting machine.** The precision wall is real: gauge pitch and bed flatness held to a fraction of millimetres over a 1–1.7 m bed, repeatable after thermal cycling and millions of needle strokes.

Two facts settle it:
- **Even Shima and Stoll buy their needles from Groz-Beckert** `[FACT — groz-beckert.com]`. A micro-brand machining its own needles is choosing to lose to a 170-year metallurgy specialist at the one thing it does best.
- **Every serious open/academic effort lives on the *software* layer, not the iron.** OpenKnit proved a hobby machine is *possible* but nowhere near commercial gauge/speed/reliability/FF-shaping; the serious research (CMU Textiles Lab, the **knitout** format, knit compilers, 2023 ACM TOG) is **all compilers and formats, not metal** `[FACT]`. The smart money builds the program, not the machine.

**Build cost ≈ $2M–10M+ over 3–7 years for a likely-inferior result** `[ESTIMATE, HIGH on direction]` — versus a sub-$20k used Chinese machine that already works. For a brand whose entire edge is **product + AI design + export-legitimacy**, that is a catastrophic misallocation. The decision is not close.

---

## 4. The Stoll exit — the one market-structure fact that changes the buy decision

**[FACT — multiple independent sources, Oct 2025: karlmayer.com, knittingindustry.com, Textile World, wtin.com]:** KARL MAYER (which had acquired Stoll in 2020) decided in early 2025 to refocus on warp knitting / technical textiles and **discontinue the STOLL flat-knitting machine business**. The Reutlingen (Germany) plant **closed 31 Oct 2025**; China production ended Dec 2025; ~280 jobs lost; no investor found. **Spare parts and service continue only via the Obertshausen central warehouse.**

**What this means for us:**
- **New Stoll is effectively end-of-line.** New-build choices narrow to **Shima Seiki (premium, India support) + Chinese (Cixing-led, cost tier).**
- **The installed base becomes a used/service market → the used pool *deepens*** (lots of ex-Stoll iron will circulate) — which **reinforces buy-used-or-subcontract.**
- **For any ex-Stoll used purchase, parts-and-service availability is now a real diligence item** (Obertshausen-only); factor it into the buy.

India itself remains an **importer / assembler / dealer market, not a builder** of computerised flat machines (Ludhiana dealers — e.g. Kniton Traders, King International — handle Shima/Stoll/Cixing plus parts and repair). `[FACT dealer existence; ESTIMATE "primarily importer," HIGH — no major Indian computerised-flat OEM found]`

---

## 5. Where reverse-engineering, own-parts, and AI help — and where they don't

### Reverse-engineering / own-parts / 3D-printing: **periphery and maintenance ONLY**
- **Do ✅:** jigs, fixtures, alignment gauges, non-critical spares (yarn-guide eyelets, carrier brackets, guards, cone holders, tensioner add-ons), take-down tweaks, and **reverse-engineering for *maintenance*** — learning cam timing and sensor behaviour to keep cheap used iron running, 3D-scanning worn obsolete brackets to replace them. This saves **real money on uptime**, which matters precisely because we'd be running cheap used machines.
- **Don't ❌:** needles, needle beds, cam systems, electronic needle-selection, controller firmware, knit-CAD cloning. The core kinematic chain (bed → needles → cams → selection) destroys money and time, and cloning selection/firmware adds **IP/legal exposure.**
- **The rule:** periphery yes, **core kinematic chain never.** `[ESTIMATE: engineering judgment, HIGH]`

### AI: real leverage on software/data/vision, hype on the iron
- **Real — do this ✅:**
  1. **Generative design → knit program** (the bridge to T8). Vendor stacks already do design → simulation → virtual-sampling `[FACT]`; AI assist drafts stitch structures, shaping, and auto-grading on top. **This is the single highest-value AI use here, because it offsets the scarce-programmer bottleneck** (§6).
  2. **CAD-programming assist** as a **programmer *multiplier*, not a replacement.**
  3. **Predictive maintenance** on used iron — high value *because* we run cheap used machines.
  4. **Vision defect detection** — the backbone of the T5 quality work.
- **Hype / overstated ⚠️❌:**
  - "AI designs and builds the machine" — **No.** AI does not solve metrology, cam profiles, or needle-selection; that's physics and manufacturing, not generation. ❌
  - "Generative AI → finished garment, no programmer" — **overstated.** AI drafts; a human programmer plus real physical sampling closes the loop. ⚠️
  - "AI replaces the linker / QC entirely" — vision QC *assists*; it does not yet replace the skilled hand seam. And cited vision-QC accuracy (~91–96%) is **dataset-specific [FACT for those papers], not a guarantee on our acrylic/blend knits** — T5 must validate on real samples. ⚠️

**The honest frame:** AI relaxes the programmer constraint and improves uptime and QC. It does not fabricate iron, and its design→machine-program *autonomy on our specific fabrics* is **[UNKNOWN]** — something T8 must **test, not assume.**

---

## 6. The programmer-skill bottleneck — the thing the whole strategy hinges on

**The binding constraint is human skill, not hardware** — and it has two faces:

1. **The CAD/flat-knit programmer** is the scarce, winnable, **machine-specific** skill that turns a design into a machine program. They author every needle action, stitch structure, FF shaping, carrier move, racking sequence, take-down setting — and, for whole-garment, the full 3D integral build. The toolchains are vendor-locked: **Shima SDS-ONE APEX / APEXFiz** vs **Stoll M1plus / knitelligence** — *a Shima program is not a Stoll program.* This is a **harder constraint than acquiring the machine**, and it is the direct continuation of T1's flagged scarcity (Ludhiana DEEPDIVE §4/§6, Open Q#8: CAD/flat-knit programmers are scarce relative to operators of older hand/power-flat machines).
2. **The linker** is the throughput-limiting hand skill (§1) — the role T1 names as the hardest to re-hire after the lean season.

**Why this matters strategically for an *AI-design* brand specifically:** our entire pipeline assumes someone can translate an AI-generated design into a runnable machine program. If that programmer is scarce and expensive locally, the design pipeline stalls at the exact hand-off it depends on. That is why **AI design→program assist is the highest-value AI investment in this segment** — it is aimed squarely at relaxing the one bottleneck that can choke the brand. But it is a **multiplier on a programmer, not a substitute for one** — at least until T8 proves otherwise on our fabrics, which today is **[UNKNOWN].**

**Local availability and cost of a programmer vs ordinary operators is [ESTIMATE, MED] / on-the-ground confirmation needed** — and it directly decides how much of the program layer is AI-assisted versus hired. This is the hinge: get it wrong and the cheapest used machine in Ludhiana is still dead iron.

---

## Bottom line for the owner

1. **Subcontract, don't own, don't build — at capsule scale.** Knit + link + finish via the Ludhiana job-work mesh; own only design + brand + (optionally) one cheap used hand-flat for sampling. The agglomeration is the infrastructure, and it suits exactly the small, fast, varied runs an AI-design brand lives on.
2. **Buy used only when one style hits proven volume — Chinese (Cixing) first.** Used iron, not new, because the used pool is deep and (post-Stoll) deepening. **New is rarely the move; Stoll new-build is off the table (2025 exit) → Shima or Chinese.** Whole-garment (no-linking) is a later volume trade study, not a launch choice.
3. **Building your own machine: never.** ~$2M–10M+ over 3–7 years for a likely-inferior result vs a sub-$20k used machine. Needle bed/needles/cams/selection/firmware/knit-CAD are all very-hard and supplier-defended (Groz-Beckert makes the needles even for Shima and Stoll). **Reverse-engineer and 3D-print only the periphery, and only for maintenance — never the core kinematic chain.**
4. **The constraint is the programmer and the linker, not the iron.** Rent/contract the scarce CAD/flat-knit programmer; don't own the linking bottleneck early. This is where the money and the risk actually sit.
5. **Point AI at software, data, and vision — not the metal.** Design→program assist (offsets the scarce programmer; bridges to T8), predictive maintenance on used machines, vision defect detection (T5). "AI builds the machine" is hype; "AI replaces the programmer/linker" is overstated.
6. **Before any cost model commits: get real landed used-machine quotes** (machine + freight + ~7.5% BCD + 18% IGST under HSN 8447 + install). The bands here are wide and LOW–MED confidence by design.

---

## Open questions — what only quotes and on-the-ground work can close

1. **Real landed used-machine quotes** (Chinese vs Shima/Stoll, by gauge/condition, + freight/duty/install into India) — the #1 input gap; bands here are wide [ESTIMATE, LOW–MED] → **T4 cost model.**
2. **Job-work rates for knitting and linking per piece** — the machine-side analogue of T1's #1 missing number (the sweater piece-rate); decides buy-vs-subcontract economics → **T4.**
3. **Local availability and cost of a CAD/flat-knit programmer vs operators** — [ESTIMATE, MED] / T1 Open Q#8; decides AI-assisted vs hired → **T8.**
4. **AI design→machine-program autonomy on our acrylic/blend fabrics** — [UNKNOWN]; T8 must test, not assume.
5. **Vision-QC accuracy on our specific knits** — cited 91–96% is dataset-specific [FACT, not transferable] → **T5 validates.**
6. **Whole-garment micro-economics** — the no-linking upside vs highest capital + steepest programming; net [UNKNOWN] at micro scale → **T4 trade study.**
7. **Exact India BCD sub-heading rate + 2025–26 budget changes** — ~7.5% BCD + 18% IGST working assumption [ESTIMATE, MED]; confirm at purchase.
8. **Ex-Stoll used service/parts cost and availability post-Reutlingen-closure** — only "service via Obertshausen" is [FACT]; affects whether ex-Stoll iron is a smart used buy → 2026 on-the-ground.

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/machines/DEEPDIVE.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `SCOPE_GLOSSARY.md`; `ludhiana/DEEPDIVE.md`; `machines/reconciled.md`
- **Delivered (owner-facing):** (1) what machines make our sweaters — flat-bed/V-bed knitting (Shima/Stoll/Cixing) + whole-garment sub-family, circular, the linker as bottleneck, finishing; (2) buy-used vs new vs subcontract with consolidated [ESTIMATE] cost bands, throughput bands, HSN 8447 import cost, and the mid-six-figure owned-line reality → subcontract at capsule scale, buy used (Chinese first) at proven volume, new rarely; (3) the blunt design-our-own vs buy-and-program verdict — buy-and-program, building is the highest-confidence "never" (~$2M–10M+/3–7 yr, Groz-Beckert needles, software-only research); (4) the Stoll 2025 exit as the market-structure fact that reinforces buy-used; (5) where reverse-engineering/own-parts/AI help (periphery + maintenance + design→program/predictive-maintenance/vision) and where they're hype (the iron); (6) the programmer-skill bottleneck (CAD/flat-knit programmer + linker) as the hinge of the whole strategy.
- **Honesty discipline:** every price/throughput/lead-time tagged [ESTIMATE] with confidence; single-figure price anchors explicitly downgraded to wide bands; Stoll closure / HSN 8447 / Groz-Beckert / vision-QC accuracy carried as [FACT] with the dataset-specific caveat; programmer availability, AI design→program autonomy, and whole-garment micro-economics held at [ESTIMATE]/[UNKNOWN] as a floor; no "build cheap" myth entertained — it is the thing this segment most confidently kills.
- **Bottom line:** subcontract → buy-used (never build); the programmer and linker are the constraints, not the machine; reverse-engineer only to maintain; aim AI at program/maintenance/QC; get landed used-machine quotes before T4 commits a cost model.
