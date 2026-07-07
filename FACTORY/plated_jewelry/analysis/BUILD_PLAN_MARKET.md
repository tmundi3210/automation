# BUILD_PLAN_MARKET.md — P4 market/business specialist roster (2026-07-07)

Source: `OWNER_BRIEF_2_MARKET.md`. This plan turns the owner's demand/business front-end ask into a
roster of **6 FACTORY specialists**, each grounded in **3 dense, gated KBs** (18 KBs total), built on the
standing pipeline (`spec.json → kb_forge.py → kb_validator --mode dense → specialist.json →
specialist_validator`, all wrapped by `bash FACTORY/build.sh <dir>` → "ALL GREEN").

These 6 are the **demand/business/product axis**. They are complementary to — not a replacement for —
the existing **supply/manufacturing axis** already in this vertical (`analysis/taxonomy/`, `processes/`).
The materials + economics specialists *draw on* those existing files as source knowledge.

## Honesty contract (binds every KB and specialist below)
- **No fabricated market data.** No invented market sizes, unit counts, gold-plated-vs-solid percentages,
  demographic/ethnicity purchase rates, brand names presented as owner's competitors, or prices stated as
  measured. Where the owner asks "how many / what % / which ethnicity / which brands / what price," the KB
  encodes the **method to measure it** (`[METHOD]`), the public data source to pull, or an honest
  `[UNKNOWN]` — never a guess dressed as fact.
- **Real domain facts are encouraged, tagged `[FACT]` with basis.** Examples that are genuinely true and
  should anchor the KBs: US FTC Jewelry Guides 16 CFR Part 23 (definitions of "gold-plated," "gold-filled,"
  "vermeil" = ≥10k gold ≥2.5µm over sterling, "rhodium," "sterling silver" = 92.5% Ag); rhodium is a
  platinum-group metal, hard, bright-white, tarnish-resistant, commonly plated over sterling silver / white
  gold; nickel-release limit EN1811 (0.5 µg/cm²/week for prolonged skin contact); MaxDiff / conjoint /
  self-selection-bias in survey methodology; keystone markup and contribution-margin definitions.
- **Every non-fact tagged** `[ESTIMATE]` (method + confidence) or `[UNKNOWN]`. Same discipline as the
  `sweater_vertical` landed-cost KB (the structural exemplar).
- Specialists are **method + decision-procedure engines**, not databases of answers.

## Dense-mode gate bands (every KB must land in-band)
nodes 19–24 · edges 32–40 · conflict_axes 8–10 · edge_cases 10–12 · workflow 9–12 ·
competency_questions 10–14 · dominance_rules 7–12 · anti_rework_rules 7–12 · iteration_protocol 6–10.
Forge rules: node id UPPER_SNAKE unique; base metrics ∈ [0,1]; `risk_if_wrong ≥ 0.80` ⇒ non-empty
`acceptance_tests`; `cross_topic_coupling ≥ 0.75` ⇒ non-empty `revisit_triggers`; conflict edge ⇒
`resolution_rule`; all refs point at real node/CQ ids; dependencies form a DAG (no cycles).

## Structural exemplars for the authoring agents to clone
- KB spec: `FACTORY/sweater_vertical/specialists/unit_economics/kb1_landed_cost.spec.json`
- Specialist spec: `FACTORY/sweater_vertical/specialists/unit_economics/unit_economics.specialist.json`
- Templates: `FACTORY/kb_spec.template.json`, `FACTORY/specialist.template.json`

---

## S1 · `market` — specialist_id `jewel_market`
**US Fashion/Costume Jewelry Demand & Consumer Analysis.** Owns: which types sell, who buys, market
sizing method, brand-landscape mapping, price-tier structure, solid-vs-plated share — all as measurement
method + real category/finish definitions.

- **kb1_category_and_finish_demand** — category demand structure (earrings/necklaces/rings/bracelets/
  anklets) and finish/tier segmentation (fine vs fashion/costume; solid-gold vs gold-plated/vermeil/
  gold-filled vs rhodium-plated silver vs stainless/steel). CQs pin: how to size category demand from
  public data (marketplace best-seller ranks, search volume, retail assortment share) `[METHOD]`; the FTC
  finish definitions `[FACT]`; how to *estimate* solid-vs-plated share without inventing it `[METHOD]`;
  why fashion/costume is the owner's tier. Excludes: consumer demographics (kb2), trend/brand mapping (kb3).
- **kb2_consumer_segmentation_method** — how to segment the US buyer: age cohort (Gen Z / 18–25 focus),
  gender skew, cultural/ethnic-market segmentation *as a research method with honest caveats* (no invented
  rates), income, occasion/gifting. CQs pin: which public datasets (Census, BLS CE Survey, platform
  audience insights) inform each cut `[METHOD]`; how to avoid stereotyping when segmenting by ethnicity
  `[FACT — method ethics]`; how to size "how many buy" via prevalence × frequency `[METHOD]`. Excludes:
  category structure (kb1), poll instruments (S4).
- **kb3_trend_and_competitor_mapping** — reading trend signals (search trends, social/marketplace signals,
  retail newness) and mapping the brand + price-tier competitive landscape; separating durable demand from
  fad. CQs pin: signal sources and their lag/bias `[METHOD]`; how to build a competitor/price-tier map from
  public storefronts `[METHOD]`; durable-vs-fad discriminators `[FACT — method]`.

## S2 · `materials` — specialist_id `jewel_materials`
**Plated-Jewelry Materials, Rhodium/Gold Finishes & Quality-Cost Engineering.** Owns: base material,
rhodium-over-silver, gold finishes, quality grading, and cost-down-without-quality-down. *Draws on existing*
`analysis/taxonomy/{substrate-forming,electro-plating,pvd-sputtering,qc-durability}.json` and
`processes/*.json`.

- **kb1_substrates_base_metals** — base-metal substrates (brass, sterling 925, stainless 316L, copper,
  zinc/pewter alloys): mechanical/appearance properties, skin-safety & nickel release (EN1811 `[FACT]`),
  tarnish tendency, relative cost. CQs pin: which base metal for which price/quality target `[FACT/METHOD]`;
  why nickel & hypoallergenic matter for 18–25 skin-contact wear `[FACT]`.
- **kb2_finishes_rhodium_gold** — finish systems: gold plating / vermeil / gold-filled with the FTC 16 CFR
  23 definitions `[FACT]`; **rhodium plating over sterling silver / white gold** — why (bright white, hard,
  tarnish/anti-oxidation), thickness in microns, barrier/strike layers, e-coat/topcoat. CQs pin: the exact
  FTC thresholds `[FACT]`; the correct science of why silver tarnishes and rhodium prevents it `[FACT]`;
  plating-thickness classes vs durability `[FACT/ESTIMATE]`. This KB directly corrects/sharpens the owner's
  rhodium understanding.
- **kb3_quality_cost_value_engineering** — quality grading + durability tests (adhesion, tarnish/salt-spray,
  wear/abrasion, nickel spot test), cost-vs-quality tradeoff curve, and **value engineering** ("lower price
  without lowering quality": base-metal choice, plating-thickness tuning, barrier layers, finish selection,
  batch economics). CQs pin: which acceptance tests gate "high-quality" `[FACT/METHOD]`; which levers cut
  cost without cutting perceived/real quality `[METHOD]`.

## S3 · `form` — specialist_id `jewel_form`
**Jewelry Form, Shape & Style Selection.** Owns the owner's explicit "which shape do I need" ask.

- **kb1_category_construction_taxonomy** — category & construction taxonomy: studs/hoops/drops; chains/
  pendants/chokers; rings/bands; bracelets/bangles/cuffs; anklets — with construction methods and how each
  maps to the plating/forming constraints from the supply-side vertical. CQs pin: the full taxonomy `[FACT]`;
  which categories are simplest to produce at the owner's scale `[METHOD]`.
- **kb2_shape_motif_style_systems** — shapes/motifs/silhouettes and style families (minimalist, statement,
  Y2K/coastal/etc. as a *descriptive* style taxonomy), which shapes photograph and convert well on-site/
  social, versatility/stackability. CQs pin: shape→conversion reasoning `[METHOD]`; style-family taxonomy
  `[FACT/ESTIMATE]`; how to keep shape choices producible.
- **kb3_sizing_fit_returns** — sizing & fit standards (ring sizing systems, chain/necklace lengths, hoop
  diameters & post gauges), fit tolerance, and the sizing→returns/exchange link. CQs pin: standard size
  systems `[FACT]`; which categories minimize sizing-driven returns `[METHOD]`.

## S4 · `poll` — specialist_id `jewel_poll`
**Social-Audience Preference Polling (IG/TikTok).** Owns the owner's poll plan.

- **kb1_instrument_design** — poll/survey instrument design: question types, single-choice vs MaxDiff/
  ranked-preference, option wording, avoiding leading/loaded questions; what a simple "do you like
  earrings?" category poll can and cannot tell you. CQs pin: instrument choice per decision `[FACT/METHOD]`;
  the limits of a 1-question IG poll `[FACT]`.
- **kb2_incentive_sampling_bias** — incentive/giveaway design and its **bias effect**, self-selection &
  platform sampling bias, sample size / representativeness, reading noisy poll data honestly, and the
  **FTC/platform rules on giveaways, sweepstakes & disclosure** `[FACT]`. CQs pin: how a "get something free"
  incentive skews responses `[FACT]`; minimum-signal sample-size reasoning `[METHOD]`; the legal/platform
  guardrails for a giveaway `[FACT]`.
- **kb3_paid_reach_and_signal** — running polls via **paid IG/TikTok ads**: audience targeting, campaign
  objective, poll creative, cost-per-response, translating clicks/engagement into a preference signal, and
  sequencing polls to feed the assortment decision (S5). CQs pin: targeting the 18–25 audience `[METHOD]`;
  turning ad responses into a rankable signal `[METHOD]`; when the signal is too weak to act on `[FACT]`.

## S5 · `assortment` — specialist_id `jewel_assortment`
**Launch Assortment & SKU Selection.** Owns "pick the 6 items to start with."

- **kb1_signal_synthesis_ranking** — synthesizing demand signals (S1 market read + S4 poll + the owner's
  existing on-site/bestseller data, "mindful of the old stuff") into one ranked candidate list; ABC/Pareto;
  hero-SKU vs long-tail. CQs pin: how to weight & combine signals of different reliability `[METHOD]`; how to
  use existing store data without over-fitting to it `[METHOD]`.
- **kb2_launch_set_of_six** — constructing the coherent **launch set of 6**: category coverage vs depth,
  price-point spread, collection cohesion, cross-sell/stacking, and the production constraints (MOQ, plating
  runs, capital) inherited from the supply-side + economics specialists. CQs pin: what makes a balanced 6-SKU
  launch `[METHOD]`; how constraints prune the candidate list `[METHOD]`.
- **kb3_test_iterate_protocol** — test-and-iterate: launch → measure sell-through / return rate / margin →
  cut/keep/expand; refresh cadence; avoiding SKU bloat. CQs pin: the keep/cut decision rule `[METHOD]`; the
  measurement window before deciding `[FACT/METHOD]`.

## S6 · `economics` — specialist_id `jewel_economics`
**DTC Plated-Jewelry Unit Economics & Pricing.** Owns "business model, cost, price, step by step."

- **kb1_cogs_and_landed** — COGS build-up for a plated piece (base metal + gold/rhodium plating consumption
  + findings + labor + packaging), landed cost (jewelry HTS/duty band, freight), MOQ & working-capital
  reality. *Draws on* the supply-side cost material. CQs pin: the per-piece cost stack `[METHOD]`; how plating
  thickness & base metal move COGS `[FACT/METHOD]`; the working-capital trap of inventory `[FACT]`.
- **kb2_pricing_and_margin** — pricing: keystone/markup `[FACT]`, perceived-value pricing for fashion
  jewelry, price-band placement, contribution margin, discount/promo effect, and the link back to value
  engineering (S2) for "lower price without lowering quality." CQs pin: how to set a defensible retail price
  `[METHOD]`; contribution-margin math `[FACT]`; when a lower price is a value-engineering job not a discount.
- **kb3_viability_cac_ltv** — DTC viability: CAC via paid social, AOV, repeat/LTV for accessories, payback
  period, break-even units, launch-budget sequencing, and the go/no-go verdict logic. CQs pin: the
  CAC:LTV/payback test `[FACT/METHOD]`; break-even from the cost + price stack `[METHOD]`; the honest
  kill-criteria `[METHOD]`.

---

## Build order & wiring
1. Author + self-gate all 18 KB specs (parallel; each agent forges + runs `kb_validator --mode dense`
   until its KB passes).
2. Author + self-gate the 6 specialist specs (each grounds in its 3 KBs by relative path, run
   `specialist_validator`).
3. `bash FACTORY/build.sh FACTORY/plated_jewelry/specialists/<dir>` per specialist → confirm ALL GREEN.
4. Commit + push. Then P4d report + owner-question map.

_Downstream (owner-gated, after the roster is green): run the venture idea **through** these specialists —
producing the actual poll instrument, the demand read from public data, the ranked 6-SKU launch set, and
the cost/price model — with every number the specialists then generate carrying its [FACT]/[ESTIMATE]/
[METHOD]/[UNKNOWN] tag._
