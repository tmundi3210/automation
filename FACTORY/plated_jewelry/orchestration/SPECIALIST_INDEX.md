# SPECIALIST_INDEX — Plated-Jewelry Vertical

Quick-reference card per specialist plus the single-agent invocation protocol. Six specialists cover the whole launch question set for a DTC plated fashion/costume jewelry venture aimed at ~18-25 US women. Each specialist is grounded in exactly three dense KBs and operates under one binding honesty contract.

**HONESTY DISCIPLINE (binding, non-negotiable, applies to every specialist and to any answer built from them).** Every load-bearing claim carries a tag:
- `[FACT]` / `[FACT-source]` — an established domain fact or cited public/regulatory source, stated with its basis.
- `[ESTIMATE]` — a derived number stated with its method, sampling frame, and confidence (uses the literal word *estimate*).
- `[METHOD]` — a recipe/instrument to measure a number that has not been pulled yet.
- `[UNKNOWN]` — an honest gap where nothing is measurable yet.
- `[SIGNAL]` — a directional preference read (e.g. a poll %) that is explicitly **not** measured demand.

NEVER invent a market size, unit-sales count, solid-vs-plated percentage, demographic/ethnicity purchase rate, gender split, brand claim, competitor share/metric, or a price/CAC/WTP as if measured. A number you cannot source is a `[METHOD]` to obtain it or an `[UNKNOWN]` — never a fabricated figure. Gaps stay gaps. On an escalation trigger the agent defers/flags/routes to human review instead of inventing.

---

## HOW TO RUN ONE SPECIALIST (single agent)

The unit of work is **one fresh agent = one specialist**. You do not blend specialists inside a single agent, and you do not let an agent answer outside its spec. The recipe:

1. **Spawn ONE fresh agent** with no prior context bleed. It gets exactly one operating spec.
2. **Put the FULL text of that `*.specialist.json` into the agent's prompt** verbatim as its operating spec. The `_directive` says these files are "loaded verbatim into the prompt template" — do that, do not summarize it.
3. **Tell the agent to READ its `grounded_in_kbs` files** (list the absolute paths — they are given per-card below). The agent answers only from its spec + those KBs.
4. **Give it the owner's question.** It answers ONLY from that spec + those KBs. Every claim is tagged `[FACT]`/`[ESTIMATE]`/`[METHOD]`/`[UNKNOWN]`/`[SIGNAL]`. Gaps stay gaps.
5. **On any `escalation_trigger`** in its spec, the agent takes the trigger's `action` (`defer` / `flag` / `human_review`) and routes to the named sibling specialist or authority — it does not invent a number to fill the gap.
6. **Scope discipline:** if the question belongs to a sibling specialist, the agent hands off with tags intact rather than answering out of lane. Use the lookup table at the bottom to pick the right specialist(s) before spawning.

### Copy-paste prompt skeleton

```
You are a single specialist agent. Operate STRICTLY as the specialist defined
by the spec below. Do not act outside it.

=== YOUR OPERATING SPEC (verbatim) ===
{paste the FULL text of {specialist_path} here — the entire *.specialist.json}
=== END SPEC ===

FIRST, read your grounded knowledge bases and answer only from them + the spec:
{kb_paths}   # the specialist's grounded_in_kbs, one absolute path per line

THE OWNER'S QUESTION:
{owner_question}

RULES (binding):
- Answer ONLY from the spec above and the KB files you just read. No outside
  numbers, no memory-sourced market data.
- Tag every load-bearing claim [FACT]/[FACT-source] (with basis),
  [ESTIMATE] (method + frame + confidence), [METHOD] (how to measure it),
  [UNKNOWN] (honest gap), or [SIGNAL] (a directional preference read, not demand).
- NEVER fabricate a market size, unit count, solid-vs-plated %, demographic/
  ethnicity/gender purchase rate, competitor brand-as-fact, price, CAC, WTP, or
  return rate. If asked for one and it is not measurable from your KBs, return
  the [METHOD] to measure it or an honest [UNKNOWN]. Gaps stay gaps.
- If the question hits one of your escalation_triggers, take that trigger's
  action (defer / flag / human_review) and name the sibling specialist or
  authority to route to — do not answer out of scope.
- Deliver a blunt, decisive, ranked answer, but never claim more certainty than
  your frame, dataset, or signal agreement supports.
```

Placeholders: `{specialist_path}` = the one `*.specialist.json` for the chosen specialist; `{kb_paths}` = that file's `grounded_in_kbs` list (reproduced on each card); `{owner_question}` = the owner's ask.

---

## SPECIALIST CARDS

### S1 — `jewel_market`
**Purpose:** the US fashion/costume demand-and-consumer analyst — reads which plated-jewelry types sell, who the ~18-25 US buyer is, how a segment/category sizes, how solid-vs-plated share splits, the categories-not-brands competitor/price-tier landscape, and whether a trend is durable — every number a `[METHOD]`, `[ESTIMATE]`, `[UNKNOWN]`, or cited `[FACT]`, never a fabricated market figure.

**Spec file:** `specialists/market/jewel_market.specialist.json`
**Grounded KBs:**
- `/home/user/automation/FACTORY/plated_jewelry/specialists/market/kb1_category_and_finish_demand.kb.json`
- `/home/user/automation/FACTORY/plated_jewelry/specialists/market/kb2_consumer_segmentation_method.kb.json`
- `/home/user/automation/FACTORY/plated_jewelry/specialists/market/kb3_trend_and_competitor_mapping.kb.json`

**Ask it when…**
- The question is which jewelry types/subtypes sell or how to size category demand from public proxies (best-seller rank, search volume, assortment share) over a documented frame.
- You need the solid-gold-vs-gold-plated share estimated without inventing a national percentage.
- You need to know who the buyer is — sizing the 18-25 cohort, testing the "mostly girls" gender-skew prior, or segmenting by income/occasion via the right public dataset (Census denominator, BLS CE propensity).
- The question touches the cultural/ethnic buyer cut (bounded by the ecological fallacy and no-invented-rate rule) or "how many people buy" via population × prevalence × frequency × spend.
- You need a categories-not-brands competitor map, a listed-price tier ladder, whitespace, or a durable-vs-fad trend verdict.

**Key escalation/boundary:** If an answer would state a market size, unit count, solid-vs-plated %, demographic/ethnicity/gender purchase rate, competitor share, or price as a *measured actual* → **flag** (no such measured number exists; give the `[METHOD]` or an honest `[UNKNOWN]`). If the cultural/ethnic cut would assert a per-group purchase rate or target a protected class → **human_review**.

**Sample competency questions (verbatim):**
- "How is category demand sized from public data (best-seller rank, search volume, assortment share) over a documented frame without a proprietary sales dataset?"
- "How do you size the 18-25 cohort and test the owner's 'mostly girls' gender-skew hypothesis rather than assuming it?"
- "How do you segment by cultural/ethnic market without stereotyping or inventing per-ethnicity purchase rates, and how is the ecological fallacy respected?"

---

### S2 — `jewel_materials`
**Purpose:** the materials/finish/quality-cost expert — decides what a plated piece is MADE of (brass, sterling 925, 316L, copper, zinc/pewter), specifies and legally labels the finish (gold electroplate / vermeil / gold-filled under FTC 16 CFR Part 23, and rhodium over sterling/white gold with correct Ag₂S-tarnish and PGM science), grades it against a durability acceptance battery, and value-engineers a lower price without crossing a quality gate.

**Spec file:** `specialists/materials/jewel_materials.specialist.json`
**Grounded KBs:**
- `/home/user/automation/FACTORY/plated_jewelry/specialists/materials/kb1_substrates_base_metals.kb.json`
- `/home/user/automation/FACTORY/plated_jewelry/specialists/materials/kb2_finishes_rhodium_gold.kb.json`
- `/home/user/automation/FACTORY/plated_jewelry/specialists/materials/kb3_quality_cost_value_engineering.kb.json`

**Ask it when…**
- The question is which base metal to use for a price/quality/skin-safety target (target-dependent matrix, ordinal cost rank, no invented per-piece price).
- The question is "why does silver tarnish and how does rhodium prevent it," why white gold is rhodium-plated, or how thick the rhodium should be.
- The question is is-it-hypoallergenic / nickel-free vs nickel-safe / safe for pierced ears (EN 1811 0.5 / 0.2 µg/cm²/week, after-wear EN 12472 retest).
- The question is which gold finish, how thick, and what it may legally be called as vermeil/gold-plated/gold-filled.
- The question is what makes a piece high-quality (acceptance battery: thickness, adhesion, salt-spray, wear, porosity, nickel) or how to cut COGS without cutting graded quality.

**Key escalation/boundary:** A cost-down that deletes a barrier underplate or thins gold below the porosity/wear floor on an active base metal → **flag** as a gate-crossing quality cut, not value engineering. A finish name to be printed before its FTC threshold is verified by XRF/karat/base-metal check → **flag** (confirm against accredited measurement / counsel).

**Sample competency questions (verbatim):**
- "Why does sterling silver actually tarnish (Ag2S), and by what mechanism does rhodium prevent it rather than merely making it shiny (kb2 CQ_02)?"
- "Why do nickel and 'hypoallergenic' matter specifically for 18-25 skin-contact wear, and what is the nickel-free vs nickel-safe distinction (kb1 CQ_03)?"
- "Which value-engineering levers lower price without lowering graded quality, and how are false economies that cross a quality gate or trade real for perceived quality avoided (kb3 CQ_10, CQ_11, CQ_12)?"

---

### S3 — `jewel_form`
**Purpose:** the form-selection expert — owns FORM not DEMAND. Classifies a piece on the three-axis frame (worn-location category × construction method × plating/forming constraint), scores producibility at owner scale (DIY electroplating, no casting foundry, no automated setting), picks shapes/motifs/style families that photograph and are testable while staying IP/culture-clear, and chooses constructions/sizes that structurally minimize sizing-driven returns.

**Spec file:** `specialists/form/jewel_form.specialist.json`
**Grounded KBs:**
- `/home/user/automation/FACTORY/plated_jewelry/specialists/form/kb1_category_construction_taxonomy.kb.json`
- `/home/user/automation/FACTORY/plated_jewelry/specialists/form/kb2_shape_motif_style_systems.kb.json`
- `/home/user/automation/FACTORY/plated_jewelry/specialists/form/kb3_sizing_fit_returns.kb.json`

**Ask it when…**
- The question is which type/shape/style/size to make, or which categories/constructions are producible at owner scale (stamped studs, pendant-on-bought-chain, huggie hoops screen simplest; cast/stone-set rings, tennis bracelets, rigid cuffs hardest).
- The question is which silhouette/motif/style family/finish to render for thumbnail and on-body legibility.
- A motif needs cultural or IP/trademark clearance before it is committed.
- The question is how to MEASURE whether a shape converts (view→add→purchase funnel, holdout/waitlist, confound controls) rather than asserting a best-seller.
- The question is the sizing/fit configuration — ring/necklace/hoop/bangle standards, why plated rings are exchanged not resized, stocked size range, and sizing-return-risk ranking.

**Key escalation/boundary:** A religious/sacred/culturally-loaded or house-signature/trademarked motif → **human_review** (binding pre-design gate; "AI-designed" launders neither). A demand for a market size / share / demographic rate / competitor fact / price / return rate as measured → **defer** (owns FORM not DEMAND; route to S1/S4/S6 as `[METHOD]`/`[UNKNOWN]`).

**Sample competency questions (verbatim):**
- "How do you MEASURE whether a shape converts (funnel + holdout/waitlist + confound controls) and read social/trend signals honestly rather than inventing a best-seller? (kb2 CQ_04, CQ_06)"
- "Why can a plated ring not be resized, which constructions remove the sizing dimension, and which categories carry no fit dimension at all? (kb3 CQ_07, CQ_08, CQ_09)"
- "What IP/trademark/design-right and cultural exposures does a motif or style choice create, and how are the two binding gates cleared? (kb2 CQ_11)"

---

### S4 — `jewel_poll`
**Purpose:** the social-audience preference-polling methodologist — designs a paid IG/TikTok poll of ~18-25 US women to RANK categories/attributes: picks the instrument from the decision (single-choice vs paired-comparison vs MaxDiff/conjoint vs WTP), budgets total-survey-error of an incentivized self-selected sample, keeps the giveaway inside US FTC/sweepstakes/platform law, computes cost-per-response, and converts engagement into a confidence-tagged rankable `[SIGNAL]` — never measured demand.

**Spec file:** `specialists/poll/jewel_poll.specialist.json`
**Grounded KBs:**
- `/home/user/automation/FACTORY/plated_jewelry/specialists/poll/kb1_instrument_design.kb.json`
- `/home/user/automation/FACTORY/plated_jewelry/specialists/poll/kb2_incentive_sampling_bias.kb.json`
- `/home/user/automation/FACTORY/plated_jewelry/specialists/poll/kb3_paid_reach_and_signal.kb.json`

**Ask it when…**
- The question is which poll/survey instrument to run, or what a simple "do you like earrings?" like-poll can and cannot conclude (it screens a category but cannot rank, price, or size).
- The question is how a giveaway/incentive biases WHO answers (freebie-hunter mismatch) and how to keep it legal (No-Purchase-Necessary + AMOE, FTC 16 CFR Part 255 disclosure, NY/FL registration+bond over $5,000, RI registration/notice over $500 — both grounded in `kb2_incentive_sampling_bias` as [FACT-state law], confirm with counsel; COPPA/18+ gate).
- The question is minimum sample / margin of error (MoE = z·√(p(1-p)/n); n≈384 → ±5%, n≈100 → ±10%) or cost-per-response and budget-to-decisive-n.
- The question is how to target the 18-25 cohort on paid and which creative can carry the poll given native stickers are organic-only.
- The question is reading a noisy poll — response-rate normalization, separability, and when a lead is too weak/biased to act on.

**Key escalation/boundary:** A stakeholder wanting to read a poll % as demand / market size / purchase rate / solid-vs-plated share / demographic buy-rate → **flag** (a poll yields a `[SIGNAL]`, not a market number — that is S1's job). A ranking resting on a within-margin lead, overlapping CIs, or sub-minimum n → **defer** (collect-more or triangulate).

**Sample competency questions (verbatim):**
- "What can a simple 'do you like earrings?' single-choice IG poll genuinely tell you, and what can it not (ranking, intensity, intent, WTP, market size, which product)?"
- "How does a 'get something free' incentive bias responses, and why does it change WHO answers (freebie-hunter audience mismatch) as much as HOW they answer?"
- "What is the minimum sample to read any signal and how wide is a small poll's margin of error (MoE = z*sqrt(p(1-p)/n), n~384=+/-5%, n~100=~+/-10%)?"

---

### S5 — `jewel_assortment`
**Purpose:** the launch-assortment/SKU-selection expert — fuses the three demand signals (S1 public-data market read, S4 paid poll, the owner's own on-site/bestseller history "mindful of the old stuff") into one reliability-weighted, ABC/Pareto-classified ranked candidate list; assembles a coherent launch set of exactly **six** SKUs trading coverage vs depth against a price ladder, finish mix, cohesion, MOQ, plating-batching, capital, and a contribution-margin floor; and defines the post-launch sell-through/return-quality/margin test with a keep/cut/expand rule.

**Spec file:** `specialists/assortment/jewel_assortment.specialist.json`
**Grounded KBs:**
- `/home/user/automation/FACTORY/plated_jewelry/specialists/assortment/kb1_signal_synthesis_ranking.kb.json`
- `/home/user/automation/FACTORY/plated_jewelry/specialists/assortment/kb2_launch_set_of_six.kb.json`
- `/home/user/automation/FACTORY/plated_jewelry/specialists/assortment/kb3_test_iterate_protocol.kb.json`

**Ask it when…**
- The question is how to weight/fuse the three demand signals into one uncertainty-tagged ranked candidate list without over-fitting the owner's stale store history.
- The question is which six SKUs to launch and how coverage, depth, price ladder, and finish compete for the six fixed slots.
- The question is how MOQ, plating-run batching, launch capital, and the contribution-margin floor prune the balanced set to a buildable one.
- The question is the post-launch test — sell-through, return/quality rate, contribution margin — and the keep/cut/expand verdict gated on a measurement window and sample-sufficiency.
- The question is refresh cadence, SKU-bloat (one-in-one-out), and the cannibalization check.

**Key escalation/boundary:** A rank resting primarily on thin/stale/survivorship store history or a single sub-threshold signal → **flag** (park `[UNKNOWN]`, feed the next poll wave; store data alone never ranks). A keep/cut/expand verdict drawn inside the measurement window or on an unconfounded raw sell-through read → **flag** (route to hold-and-tweak).

**Sample competency questions (verbatim):**
- "How is the owner's existing store data used 'mindful of the old stuff' with recency decay and shrinkage so the launch is not over-fit to a small, stale, survivorship-biased history?"
- "Why exactly six SKUs, and how is the six-slot budget treated as a scarce resource in which coverage, depth, a price tier and a finish all compete for the same slots?"
- "What is the keep/cut/expand rule, how does it handle winners, losers and the ambiguous middle, and how are marketing, price and seasonality confounds separated from true product performance?"

---

### S6 — `jewel_economics`
**Purpose:** the unit-economics and pricing expert — builds honest per-piece landed COGS (base metal + gold/rhodium mass from geometry × thickness × density + findings + labor + packaging, yield-adjusted, through the correct jewelry HTS duty band + freight/fees), turns it into a defensible retail price bounded by a contribution-margin floor and a measured-WTP ceiling in a real competitor band, and runs the paid-social viability arithmetic (CAC, AOV, contribution-per-order, first-order economics, LTV:CAC, payback, break-even, kill-criteria) to a cross-examined go/no-go.

**Spec file:** `specialists/economics/jewel_economics.specialist.json`
**Grounded KBs:**
- `/home/user/automation/FACTORY/plated_jewelry/specialists/economics/kb1_cogs_and_landed.kb.json`
- `/home/user/automation/FACTORY/plated_jewelry/specialists/economics/kb2_pricing_and_margin.kb.json`
- `/home/user/automation/FACTORY/plated_jewelry/specialists/economics/kb3_viability_cac_ltv.kb.json`

**Ask it when…**
- The question is per-piece cost, or how plating thickness / base-metal choice moves COGS (2.5µm heavy plate ≈ 14× the gold of a 0.175µm flash; substrate flips HTS 7117 ~11% vs 7113 ~5% — both [FACT-HTSUS], carried inline from the spec's tags).
- The question is the MOQ / gold-rhodium-inventory working-capital trap that can kill a per-unit-profitable brand.
- The question is what to charge — contribution margin (not gross), keystone as a floor check only, measured WTP ceiling, competitor band, charm price.
- The question is discount vs value-engineering reprice, or realized-vs-list margin net of returns.
- The question is whether the launch makes money — CAC, first-order economics, LTV:CAC, payback, break-even units/ROAS, and the qualified go/no-go with pre-committed kill-criteria.

**Key escalation/boundary:** The verdict/price turning on live gold/rhodium spot or a forward country tariff surcharge → **defer** (pulled at cost time, `[UNKNOWN]`-forward, never hardened). A go/no-go resting on a CAC/AOV/repeat/return number with no live reading → **defer** (softest, most decision-critical `[UNKNOWN]`s; low confidence until the $-capped live-ad test).

**Sample competency questions (verbatim):**
- "How is the gold/rhodium consumed per piece computed from the item's geometry, plating thickness and metal density?"
- "What is keystone markup versus contribution margin, and why is a cost-multiple alone not a defensible price for fashion jewelry?"
- "How is CAC via paid IG/TikTok/Meta measured rather than guessed, why must marginal paid CAC (not blended or platform-reported) govern scaling, and does the launch make money on the first order?"

---

## WHICH SPECIALIST FOR WHICH OWNER QUESTION

Maps the owner's original asks to the specialist(s) that OWN each. The **owner** column is the primary specialist to spawn; the **supporting** column names siblings that contribute a tagged input or receive the handoff.

| Owner's original ask | Owns it | Supporting / handoff |
|---|---|---|
| **US demand** — how big is the market, who buys, how many buy | **S1 jewel_market** | S4 (poll as a `[SIGNAL]` input); S5 (consumes the read) |
| **Which types sell** — which categories/subtypes are in demand | **S1 jewel_market** (demand read) | S4 (poll ranks candidates); S5 (fuses signals & picks) |
| **Rhodium science** — why silver tarnishes, why/how rhodium prevents it, why white gold is rhodium-plated | **S2 jewel_materials** | S1 (positions rhodium tier as `[FACT]`, defers process to S2) |
| **Base material** — what should it be made of, is it skin-safe/hypoallergenic | **S2 jewel_materials** | S3 (wear-zone/nickel by form); S6 (substrate → duty band & cost) |
| **Price & cost-to-make** — per-piece COGS and what to charge | **S6 jewel_economics** | S2 (materials spec feeds COGS); S1 (competitor price band) |
| **Cost-down without quality loss** — lower price, hold quality | **S2 jewel_materials** (graded value-engineering) **+ S6 jewel_economics** (discount-vs-VE reprice, realized margin) | S5 (margin-floor gate on the launch set) |
| **Which 6 items** — the final launch set of exactly six SKUs | **S5 jewel_assortment** | S1 + S4 (demand signals); S3 (producibility/sizing); S6 (margin floor / price bands) |
| **Which shapes** — silhouette/motif/style/size selection & producibility | **S3 jewel_form** | S2 (plating/nickel constraints); S4 (measure conversion); S5 (feeds the set) |
| **The poll** — design/run the IG/TikTok preference poll & read it | **S4 jewel_poll** | S1 (market read to triangulate); S5 (receives the ranked candidate list) |

**Routing notes:**
- Anything that would require a *fabricated* market size, share, demographic/ethnicity rate, competitor metric, or measured price/CAC is answered as a `[METHOD]` or `[UNKNOWN]` by whichever specialist is asked — no specialist manufactures it.
- The natural pipeline order is **S1 + S3 + S2 → S4 → S5 → S6** (or S6 in parallel for cost/pricing), with S5 as the synthesis point for the six-SKU pick and S6 as the go/no-go gate.
- "Cost-down without quality loss" is the one ask that genuinely spans two owners: S2 re-engineers the material spec while holding the acceptance gate; S6 decides whether the lower price is a margin-sacrificing discount or a permanent VE reprice. Spawn both.
