## The Instagram / TikTok Poll (ready to launch)

_Analyst: `jewel_poll` specialist. Scope: the exact paid IG/TikTok poll the owner can run to turn "which jewelry do 18–25 US women like?" into a **ranked, confidence-tagged candidate list** that seeds the six-SKU launch — the instrument, the incentive built to minimize self-selection bias, its US legal envelope, the 18–25 paid targeting, the minimum sample for a usable signal, and the exact read-out into a preference ranking. This section produces a **preference SIGNAL among responders**, never measured demand, a purchase rate, or a market size — those are the `jewel_market` section's job, and the final SKU pick is `jewel_assortment`'s._

**Honesty contract (binds every number below).** Every quantitative claim carries one tag: **[FACT-source]** (a cited public/legal/statistical fact), **[ESTIMATE-basis]** (a number I derived, method shown), **[METHOD]** (a recipe to measure a number not yet obtained), or **[UNKNOWN]** (an honest gap). **A poll % is labeled a [SIGNAL], never demand.** No response count, incentive lift, delivered-audience composition, CPM, cost-per-response, or preference share is asserted as a measured actual for a poll that has not run — those are all [METHOD]/[UNKNOWN] until a live campaign measures them. Platform ad-product facts are dated **[FACT-as-of 2026-07]** and must be re-verified in Ads Manager before the build.

---

### 0. Decision first — what this poll must decide (and what would make it worth nothing)

The specialist's non-negotiable rule: **name the decision before choosing any instrument** ([FACT-method], DECISION_FIRST_FRAMING). The launch decision here is:

> **Of the six launch SKUs, how should they be split across category and finish?** — i.e. produce a **ranking** of the five categories (earrings / necklaces / rings / bracelets / anklets), then, inside the top 1–2 categories, a ranking of **style** (e.g. studs / hoops / huggies / drops) and a **finish** read (rhodium silver-tone vs gold-tone) and a rough **price band**.

Because the decision is a **ranking**, the core instrument **cannot** be a folder of independent "do you like earrings?" yes/no polls — on a friendly audience every category scores "yes," yielding no order ([FACT], FORCED_TRADEOFF_RANKING). A ranking needs a **forced trade-off**. The simple like-poll still has one legitimate job — a cheap **screen** to drop a dead category — so the design below runs **both**: a single-choice screen *and* a forced-trade-off follow-up. If a proposed poll question would change no launch action, it is cut.

---

### 1. Poll A — the single-choice category screen (the "simple poll" the owner asked for)

This is the coarse, high-response top-of-funnel item. It **screens** interest; it does **not** rank ([FACT], SINGLE_CHOICE_LIMITS).

**Exact question:**
> **"Which piece do you reach for most?"**
> ◻ Earrings ◻ Necklaces ◻ Rings ◻ Bracelets ◻ Anklets *(◻ None of these)*

Design notes, all [FACT-method] from the instrument KB:
- **MECE options** (OPTION_WORDING_MECE): the five categories are mutually exclusive, at one taxonomic level (don't mix "earrings" with "gold hoops"), plus an explicit **opt-out** ("None of these") so a non-preferrer isn't forced into a false pick.
- **Order effect** (OPTION_ORDER_EFFECTS): position biases choice (primacy on a visual list). Native stickers **cannot randomize order**, so **counterbalance** by running the item on ≥2 creatives with the category order swapped, or move it to a form that randomizes.
- **What Poll A CAN tell you:** a coarse top-of-mind affect read and a fast, cheap engagement signal to **kill a near-dead category**. **What it CANNOT:** rank the categories against each other, measure intensity/intent/willingness-to-pay, say *which* earring, or size the market ([SIGNAL] only).

Because a native IG poll sticker is exactly **2 options** and an IG quiz is up to **4** (§4), the 5-way item ships either as an **off-platform form** (preferred, see Poll B) or as a **paired-comparison funnel** of 2-option stickers.

---

### 2. Poll B — the forced-trade-off follow-up (this is what actually ranks the six SKUs)

Two deliverable routes; pick per how decisive the read must be (PLATFORM_SEQUENCING_DESIGN, [METHOD]).

**Route 1 — native paired-comparison funnel (approximate ranking, more reach).** A chain of 2-option stickers/voting add-ons that makes categories compete:
- Round 1: *Earrings vs Necklaces* → Round 2: *winner vs Rings* → *winner vs Bracelets* → *winner vs Anklets*, plus a couple of cross-pairs to check consistency. Each tap is a forced choice, so it yields a real order. **Cost:** each stage re-selects a different self-selected sub-audience (a bias handed to §6), and pair order can tilt the result — so rotate which item is shown first.

**Route 2 — off-platform MaxDiff (rigorous ranking, cleaner signal — recommended for the category rank).** Drive the ad to a linked form (Typeform / Google Form / Sawtooth) hosting a **MaxDiff / best-worst** task ([FACT-Louviere & Woodworth], MAXDIFF_BEST_WORST): show balanced subsets of 4–5 items and ask **best** and **worst** in each; across the block design this returns ranked scores on a common scale with far less acquiescence than rating each item. This is the strongest lightweight ranker and the one that most cleanly feeds the six-SKU pick. **Cost:** multiple screens + a link-out funnel that lowers completion (raises cost-per-response, §7).

**Exact MaxDiff item (repeat over ~5–6 screens, rotating the set):**
> **"Of these, which would you MOST want and which LEAST?"**
> [ Gold hoops · Silver-tone (rhodium) studs · Pendant necklace · Stackable rings · Chain bracelet ] — tap one **Most**, one **Least**.
> (Rotate items so each appears an equal number of times across screens.)

**Attribute follow-ups (only inside the winning category — don't ask before you have a winner):**
- **Finish (the brand's core question):** a clean paired choice — **"Which finish?" ◻ Bright silver-tone (rhodium-plated) ◻ Warm gold-tone (gold-plated)**. The two options must be **correctly defined per the FTC Jewelry Guides, 16 CFR Part 23** and named neutrally; the poll may test *preference between correctly-defined finishes* but must **not** assert a finish claim as demand ([FACT], routed to `jewel_materials`).
- **Style within category:** e.g. *studs / hoops / huggies / drops* (4-option quiz or paired funnel).
- **Price band (willingness-to-pay) — never inferable from a like-poll** ([FACT], WTP_INSTRUMENTS). Use a proper WTP instrument on the off-platform form: **Gabor-Granger** ("At $X, how likely are you to buy? …" repeated at $19 / $29 / $39 / $49) or **Van Westendorp** four-question price-sensitivity. A liking poll carries **zero** price information; WTP is a separate, explicit instrument.

---

### 3. Pretest before you spend

Cognitively pretest the wording on 5–10 people in the target cohort (read-aloud, "what does this question mean to you?") to catch ambiguous options and leading phrasing (PILOT_COGNITIVE_PRETEST, [FACT-method]). Cheap insurance against spending reach on a broken item.

---

### 4. Native platform affordances — re-verified live (2026-07)

The KB's dated affordances were re-checked against current platform/ad docs; the paid picture is **richer than "stickers are organic-only"** — verify again in Ads Manager on the build day:

| Surface | What it supports | Tag |
|---|---|---|
| IG Story **poll** sticker | exactly **2** options (binary) | [FACT-as-of 2026-07, socialrails / Meta] |
| IG **quiz** sticker | up to **4** options (one keyed "correct" — built for trivia, not neutral preference) | [FACT-as-of, socialrails] |
| IG **polling sticker in Stories *Ads*** (Ads Manager) | **paid** poll sticker is now a supported Stories-ad add-on under objectives incl. Traffic / Leads / Conversions | [FACT-as-of, Meta Business Help / Sprinklr] |
| IG native polls in **feed/Reels comments** | up to **4** answer options (added 2025) | [FACT-as-of, socialrails] |
| IG emoji-slider / question stickers | one 0–100 scale / open text | [FACT-as-of] |
| TikTok organic in-app poll sticker | **removed ~2022**; a comment-poll test was pulled in June 2026 | [FACT-as-of, TikTok/benly] |
| TikTok **Voting Sticker interactive add-on** (Ads Manager) | **paid** add-on: a voting topic + **2** options on a video ad | [FACT-as-of, TikTok Ads Help] |

**Consequence:** both platforms now offer a **paid 2-option interactive vote** (IG polling-sticker Stories ad; TikTok Voting Sticker), which is perfect for **Poll A screens and the paired-comparison funnel (Route 1)**. Neither can host a native **MaxDiff/conjoint/WTP** — anything richer than 2–4 options **must go off-platform** (Route 2). Re-verify these exact limits before launch; they change often.

---

### 5. Reaching 18–25 US women on paid (targeting is an inferred proxy)

Targeting levers on Meta/TikTok Ads Manager (AUDIENCE_DEFINITION_18_25, [METHOD-targeting]): **age 18–24/25, gender = women, geo = US**, interest/behavior affinities (fashion, jewelry, specific styles), **lookalike/similar** audiences seeded from the owner's existing pixel/customer list, and **custom audiences** (site visitors, engagers — the store is already live, so this seed exists).

Two binding caveats ([FACT]):
- **Platform age/interest data is inferred and self-reported, not verified** — "women 18–25 interested in jewelry" is a *proxy* for the cohort, not a census. Do not read the targeting spec as proof of who actually answered.
- **The optimizer, not you, curates the responder slice** (PLATFORM_TARGETING_MECHANICS): the delivery algorithm narrows to whoever is cheapest to get the chosen event from, skewing the responding sample in ways the targeting spec doesn't show.
- **No protected-class (e.g. ethnicity) targeting** — off-limits on Meta/TikTok and out of scope; any cultural read comes from the store's own first-party signal + Census/ACS base rates, never a demographic purchase-rate table (which does not exist).

**Objective ↔ response-unit match** (CAMPAIGN_OBJECTIVE_CHOICE, [METHOD]): a cheap **engagement** objective is fine for the coarse Poll A screen; the **tie-breaking MaxDiff/attribute polls that must be rankable with confidence** should optimize for **Traffic (link clicks) or Leads/Conversions** — the completed off-platform answer is the cleanest vote, at higher cost per response.

---

### 6. The incentive — built to MINIMIZE self-selection bias, and its US legal envelope

The owner's instinct ("offer something free to attract them to answer") is sound for **lift** but is the single biggest **bias** risk: a broadcast free-jewelry giveaway recruits **freebie-hunters and giveaway-farm accounts** whose demographics/intent may not match the 18–25 buyer, so you'd rank preferences on the **wrong population** ([FACT-mechanism], INCENTIVE_AUDIENCE_MISMATCH). Bigger n does **not** fix this — self-selection is a systematic offset that does not average out ([FACT], SELF_SELECTION_BIAS / TOTAL_SURVEY_ERROR). A large incentivized sample is a *more precisely wrong* number, not a better one.

**Incentive design levers to keep the lift without wrecking the signal** ([METHOD], INCENTIVE_DESIGN_TRADEOFF):
1. **Relevant prize, not cash/gift-card** — give away **one of your own jewelry pieces**. A jewelry prize filters toward jewelry-interested respondents; generic cash maximizes freebie-hunters.
2. **Single random draw**, not "everyone who answers gets $X" — draws attract fewer straightliners.
3. **Decouple the prize from the answer** — entry does **not** depend on *which* option is chosen, and **say so** ("your answer doesn't affect your odds"). Removes the demand-characteristic pull toward a "winning" answer.
4. **Gate the entry after the question** — collect the preference **first**, offer entry **after**, so the answer is given before the prize is salient.
5. **Modest prize value** — a smaller prize lifts less but draws fewer bots/farms (ties to fraud control).
6. **Measure the bias, don't assume it away** — run an **incentivized-vs-not A/B split**; if the two arms rank categories differently, the incentive is distorting the read. The realized lift and bias of each lever are **[UNKNOWN]** until this A/B measures them.

**Fraud/bot controls** (treat entry fraud as the *predictable* cost of a prize, [FACT], FRAUD_BOT_CONTROL): attention/qualification checks, **deduplication** on account/email/device, **US + 18+ eligibility and geo screening**, rate-limiting / one-entry-per-person, and — critically — **separate the sweepstakes ENTRY act from the PREFERENCE answer**, counting preferences only from verified on-target respondents. Residual fraud after controls is [UNKNOWN], estimate it, don't assume zero.

#### The legal envelope — [FACT], route each to counsel / the live guideline (this is the law, not legal advice for your specific filing)

- **US sweepstakes / lottery test** ([FACT-US promotion law], SWEEPSTAKES_LAW): a promotion combining **consideration + chance + prize** is an illegal private lottery. Fix it by removing **consideration**: a clear **"No Purchase Necessary"** plus a **free, equal-dignity Alternative Method of Entry (AMOE)** giving non-buyers the **same odds** (e.g. a free web/mail-in entry). Publish **Official Rules** stating sponsor, eligibility (US, 18+, exclusions), start/end dates, the free AMOE, odds, prize description + **approximate retail value (ARV)**, and winner selection/notification.
- **State registration + bond** ([FACT-source: Klein Moynihan Turco / RAVEN5, 2026]): **New York and Florida** require **registration + a surety bond** when the **aggregate prize pool exceeds $5,000** (NY: file ≥30 days before; FL: register with the Dept. of Agriculture & Consumer Services, file ≥7 business days before, fines up to $1,000 for late filing). **Rhode Island** requires **registration/notice for retail promotions with prizes over $500** (no bond). **Practical guardrail: keep the total prize ARV modest (e.g. ≤ $500) and you stay under the NY/FL $5,000 bond triggers *and* the RI retail threshold** — a single mid-tier plated piece easily fits, which also happens to be the *lower-bias* choice (lever 5).
- **FTC material-connection disclosure** ([FACT-16 CFR Part 255, Endorsement Guides, rev. June 2023], FTC_ENDORSEMENT_DISCLOSURE): the giveaway tied to responding/entering is a **material connection** that must be disclosed **clearly and conspicuously, in plain language** ("sweepstakes entry," "#ad") in the **same place as the message** — not buried in a hashtag block, and **the advertiser (you) is responsible**; the duty does not shift to the user. A native paid-partnership label alone may not suffice.
- **Privacy / consent / ethics** ([FACT], PRIVACY_CONSENT_ETHICS): **honor the prize** on the stated terms; **data-minimize** (collect only what the draw needs); collecting emails brings **CCPA/CPRA** notice-at-collection + opt-out/deletion duties and **CAN-SPAM** (clear opt-out, honest headers) for any follow-up marketing; **age-gate to 18+** and US-eligible — if reach could capture **under-13s, COPPA** bars collecting their data without verifiable parental consent. Don't disguise the ad as organic or imply everyone wins.

**Escalation flags (per the specialist):** any prize pool with consideration+chance+prize lacking a No-Purchase-Necessary AMOE, any pool over $5,000 in NY/FL (or $500 retail in RI), any missing FTC disclosure, or any reach to under-18s → **stop and route to counsel + the live Meta/TikTok promotion guidelines** before launch.

---

### 7. How big a sample — and what it costs to reach it

**Margin of error** ([FACT-statistics], MoE = z·√(p(1−p)/n), 95%, worst-case p=0.5):

| Usable n (per option cell) | Approx. MoE |
|---|---|
| ~100 | ±~10% |
| ~384 | ±5% |
| ~1,000 | ±~3% |

**The per-cell trap (load-bearing):** a 5-category poll **splits n across cells** — each option's *own* count sets its precision, not the total. A 5-way split of n=500 leaves ~100 per option (±~10%). **To read the category rank with usable per-option precision (~±8–10%), target ~100–150 usable responses *per category* → ~500–750 usable responses total** ([METHOD], SAMPLE_SIZE_MIN_SIGNAL). The MaxDiff needs enough respondents that each item appears in enough subsets — plan the same **~500+ usable** order of magnitude.

**The decisive caveat:** this formula bounds **only random sampling error** on a random sample. It does **not** bound self-selection, platform-coverage, or incentive bias. A tight ±3% MoE on an incentivized self-selected sample is **precision without accuracy**. n is a **floor to read a signal, never a fix for bias.**

**Cost-per-response** ([METHOD], COST_PER_RESPONSE_MODEL — formula is real, all live inputs [UNKNOWN] until a run):
> **CPR = spend / usable responses = CPM / (usable responses per 1,000 impressions)**, where usable-responses-per-1,000 = 1,000 × CTR/engagement-rate × completion-rate × usable-fraction (after bot/junk filtering).

This makes a high CPR **diagnosable**: it's either a high-CPM (audience) problem, a low-CTR (creative/hook) problem, a low-completion (funnel-friction) problem, or a high-junk (incentive/bot) problem — each with a different fix.

**Illustrative budget (every input flagged — do NOT quote as a quote):**
- Benchmark CPM ranges (cited, not your measured cost): **IG ~$6.70 average; TikTok ~$3.50–$7, generally 20–40% cheaper than Meta** [FACT-source: WordStream / Lebesgue / AdRoll benchmarks, 2026] — actual auction cost is [UNKNOWN] until you run.
- Assume, purely to show the arithmetic: CPM = **$7** [FACT-benchmark], link-CTR = **1%** [ESTIMATE-basis: display-ad benchmark], off-platform completion = **40%** [ESTIMATE-basis: link-then-form funnel]. Then usable responses/1,000 impressions = 1,000 × 0.01 × 0.40 = **4**, so **CPR ≈ $7 / 4 ≈ $1.75** [ESTIMATE, illustrative].
- For ~500–750 usable responses: **~$875–$1,310** [ESTIMATE-basis: CPR × required-n]; reading the whole poll at n≈384 (±5% overall) ≈ **~$670** [ESTIMATE]. **A cheap engagement-objective screen (Poll A) can run for far less; the clean MaxDiff read costs more per response — that is the CAMPAIGN_OBJECTIVE trade-off, not a defect.**
- **Feasibility gate:** if the budget to reach a decisive per-cell n exceeds what the owner will spend, the honest output is **"the poll cannot answer that question at that granularity"** — simplify to fewer options or accept a coarser read (SAMPLE_SIZE_BUDGET).

---

### 8. Reading the responses into a preference ranking (the read-out that feeds the 6 SKUs)

This is the exact procedure that converts raw taps into the ranked candidate list handed downstream.

1. **Define the response unit *before* counting** ([METHOD], ENGAGEMENT_TO_RESPONSE_MAP): a **response = an option choice** — a completed form answer, an option-specific link click (earring-ad click vs necklace-ad click), or a categorized vote. An impression, a 3-second view, a scroll-past, or an undifferentiated like/follow is **not** a vote. Record the **usable fraction** after removing bots/dupes/junk.
2. **Normalize to a RATE, not a raw count** ([METHOD], SIGNAL_CONSTRUCTION_RANKING): rank options by **responses per impression (or per dollar)**, so an option that merely got more budget/impressions can't win on exposure. Compute each option's **share of normalized preference with a confidence interval sized by its usable n**.
3. **Confound control** ([METHOD], CONFOUND_CONTROL): hold creative/format/placement constant across options and **rotate** them, so category preference isn't confounded with creative appeal (the specific risk of the A/B-creative vehicle). In a paired funnel, rotate which item shows first.
4. **Test separability** ([FACT/METHOD], STAT_CONFIDENCE_TEST): two-proportion z-test (or a simple **CI-overlap** check) on the normalized shares; the minimum detectable difference is set by the per-cell n. With five options competing, apply a **multiple-comparison caution** (Bonferroni-style) so one option doesn't look like a winner by chance.
5. **Apply the weak-signal rule — refuse to over-read noise** ([FACT], WEAK_SIGNAL_THRESHOLD). Do **not** turn the result into a ranking if **any** of: top options' CIs **overlap**; usable n is **below** the minimum; the gap is **inside the bias band**; or CPR was so high the budget couldn't buy a decisive n. When weak → **collect more, simplify the question, or fall back** to the market read and existing-store sell-through, and **label the poll inconclusive.** Forcing a decision on a weak signal is worse than admitting it didn't resolve.
6. **Stated ≠ revealed** ([FACT], STATED_VS_REVEALED): a poll captures **stated/engaged** preference under hypothetical, incentivized conditions. A poll winner is a **directional hypothesis to validate against real sell-through**, not a proven seller.
7. **Hand off, reliability-weighted** ([METHOD], SIGNAL_FUSION_HANDOFF): deliver to `jewel_assortment` a **ranked candidate list**, each item tagged with its normalized share, CI, usable n, separability verdict, stated-vs-revealed status, and legal-clearance/bias status. Weight it against sibling signals by reliability: **revealed store sell-through > validated poll > stated poll > inferred market estimate.** **This specialist stops before the final six-SKU pick — that is `jewel_assortment`'s.**

**One-line output shape handed downstream:**
> `Rank: [Earrings 0.31±0.06 (n=142) | Necklaces 0.27±0.06 (n=138) | …]; Earrings vs Necklaces: CIs overlap → NOT separable (tie); finish read: gold-tone > silver-tone, separable; legal: AMOE+rules cleared, ARV ≤$500 (no NY/FL/RI trigger); bias: incentivized arm agreed with control.` *(illustrative structure, not measured values.)*

---

### What only your poll / private data can answer

Genuine gaps in this domain — no public source closes them honestly; this poll plus your own store analytics are what fill them:

- **Your audience's actual category *ranking*** (earrings vs necklaces vs rings vs bracelets vs anklets) — public data ranks the *market*, not *your* 18–25 women. This is the MaxDiff's core output. [poll — Route 2]
- **Rhodium silver-tone vs gold-tone finish preference** for your buyer — the brand's defining question; a paired-choice + first-week sell-through read, never a public fact. [poll + private data]
- **Which style within the winning category** (studs/hoops/huggies/drops) wins. [poll]
- **Willingness-to-pay / the right price band** for your six SKUs — requires a Gabor-Granger/Van Westendorp instrument; a like-poll yields **zero** price information. [poll — WTP instrument → `jewel_economics`]
- **The realized incentive lift and its bias direction** — measurable only by the incentivized-vs-not A/B split; [UNKNOWN] until run.
- **Live CPM / CTR / completion / cost-per-response** for your account and creative — [UNKNOWN] until a campaign measures them; the formula and diagnostic are given, the numbers are not invented.
- **Who actually responded** (true delivered-audience composition vs the targeting spec) — an inferred proxy; confirm against platform audience insights + your store's first-party analytics.
- **Any ethnicity/cultural read** — no public per-group purchase rate exists or may be invented, and protected-class targeting is barred; only your privacy-safe first-party signal + ACS trade-area base rates can speak to it, bounded by the ecological fallacy.

_All figures above carry their tags. Every poll % is a responder **[SIGNAL]**, never demand; platform ad affordances are dated **[FACT-as-of 2026-07]** to re-verify in Ads Manager before the build; the incentive is designed inside the FTC/sweepstakes/privacy envelope, with prize ARV kept modest to stay under the NY/FL $5,000 bond and RI $500 retail thresholds; and the final six-SKU pick is left to `jewel_assortment`._
