# POLL_KIT.md — the ready-to-run buyer-preference poll (copy-paste, launch-ready)

_Owner-facing. The storefront is already live. This is the **one instrument that collects the single
thing no public data supplies**: which jewelry category and which finish **your** ~18–25 US women
prefer, and roughly what they'll pay. Everything here is lifted from and kept consistent with the
`jewel_poll` specialist (kb1 instrument_design, kb2 incentive_sampling_bias, kb3 paid_reach_and_signal)
and Section 6 of the MARKET_ANALYSIS_REPORT. Copy the boxed text verbatim into IG/TikTok and your form._

**Honesty contract (binds every number in this kit).** Every load-bearing claim carries one tag:
**[FACT]** / **[FACT-source]** (a cited public/legal/statistical fact or standards constant),
**[ESTIMATE]** (a figure derived with the method shown — not a measured actual),
**[METHOD]** (a recipe to obtain a number not yet measured), **[UNKNOWN]** (an honest gap), and
**[SIGNAL]** (a poll % — a preference read among self-selected responders, **never** demand). No
response count, incentive lift, delivered-audience composition, CPM, cost-per-response, category rank,
finish share, or price is asserted as a measured actual for a poll that has not run. Platform ad-product
facts are dated **[FACT-as-of 2026-07]** and must be re-verified in Ads Manager before you build.

---

## 1. What this poll answers — and what it does NOT

**The decision it exists to inform** [FACT, DECISION_FIRST_FRAMING]: _of your six launch SKUs, how do
they split across **category** and **finish**?_ Because that is a **ranking**, the core instrument is a
**forced trade-off**, not a folder of "do you like earrings?" yes/no polls (a friendly audience says
"yes" to everything, which yields no order).

**What it DOES give you** — the one thing no public source can supply for **your** buyer:

- **Your audience's category preference** — a ranking of earrings / necklaces / rings / bracelets /
  anklets among the women who actually respond. Public data ranks *the market*, not *your* 18–25 women.
  [SIGNAL — the MaxDiff's core output]
- **Rhodium silver-tone vs gold-tone finish preference** — the brand's defining question. A paired
  choice; validate later against first-week sell-through. [SIGNAL]
- **Style within the winning category** (studs / hoops / huggies / drops). [SIGNAL]
- **Price acceptance / willingness-to-pay** — via a proper WTP instrument (Gabor-Granger), because a
  liking poll carries **zero** price information. [SIGNAL → routes to `jewel_economics`]

**What it does NOT do — do not over-read it:**

- It is a **preference [SIGNAL] among self-selected responders, not a sales forecast, not demand, not a
  market size, not a purchase rate, not a finish-share of the population.** A poll % is never demand.
  [FACT, SINGLE_CHOICE_LIMITS / HONESTY_LEDGER]
- **Poll A (single-choice) screens; it does not rank.** It can kill a near-dead category and give a
  cheap top-of-mind read; it cannot order categories against each other, measure intensity/intent, price
  anything, or say *which* earring. [FACT]
- It measures **stated** preference under hypothetical, incentivized conditions. A poll winner is a
  **directional hypothesis to validate against real sell-through** (stated ≠ revealed). [FACT,
  STATED_VS_REVEALED]
- The margin-of-error math bounds **only random sampling error** — it does **not** fix self-selection,
  platform-coverage, or incentive bias. A tight ±3% on an incentivized self-selected sample is
  **precision without accuracy.** [FACT, TOTAL_SURVEY_ERROR]
- **It does not pick the six SKUs.** That is `jewel_assortment`'s job (see §6). This kit stops at a
  confidence-tagged ranked candidate list plus its bias and legal-clearance status.

---

## 2. The instruments — verbatim and launch-ready

Copy the boxed text exactly. Run **both** a cheap single-choice screen (Poll A) **and** the forced
trade-off (Poll B) — they do different jobs.

### Poll A — single-choice category screen (the coarse top-of-funnel screen)

> **"Which piece do you reach for most?"**
> ◻ Earrings ◻ Necklaces ◻ Rings ◻ Bracelets ◻ Anklets *(◻ None of these)*

Design notes [FACT-method, kb1]:
- **MECE options at one taxonomic level** — don't mix "earrings" with "gold hoops" — plus an explicit
  **opt-out** ("None of these") so a non-preferrer isn't forced into a false pick.
- **Order effect:** native stickers can't randomize option order, so **counterbalance** — run the item on
  **≥2 creatives with the category order swapped**, or host it on a form that randomizes.
- **Feasibility:** a native IG poll sticker is exactly **2** options and an IG quiz is up to **4**
  [FACT-as-of 2026-07], so this 5-way item ships **either off-platform** (preferred) **or as a
  paired-comparison funnel** (Route 1 below). Poll A **screens**; it does **not** rank.

### Poll B — the forced trade-off that actually ranks

**Recommended: off-platform MaxDiff / best-worst** (Typeform / Google Form / Sawtooth). Show a balanced
subset of items; ask best **and** worst; rotate the set. Repeat over **~5–6 screens**, rotating so each
item appears an equal number of times. [FACT-Louviere & Woodworth, MAXDIFF_BEST_WORST]

> **"Of these, which would you MOST want and which LEAST?"**
> [ Gold hoops · Silver-tone (rhodium) studs · Pendant necklace · Stackable rings · Chain bracelet ]
> — tap one **Most**, one **Least**.
> _(Rotate items so each appears an equal number of times across screens.)_

**Item pool** (the five buyable subtypes, one per category, finish embedded so the finish read is
seeded): Gold hoops (earrings) · Silver-tone rhodium studs (earrings) · Pendant necklace · Stackable
rings · Chain bracelet. Add an anklet item on the rotation if you want anklets ranked
(_e.g._ "Anklet chain").

**Alternative: native paired-comparison funnel** (Route 1 — more reach, approximate ranking) [METHOD]:
2-option votes — *Earrings vs Necklaces* → winner vs Rings → vs Bracelets → vs Anklets, plus a couple of
cross-pairs to check consistency. Rotate which item shows first. Both platforms now support a **paid
2-option interactive vote** (IG polling-sticker Stories ad; TikTok Voting Sticker) — perfect for this
funnel — but **re-verify these limits in Ads Manager on build day** [FACT-as-of 2026-07]. Anything richer
than 2–4 options must go off-platform.

### Finish question (the brand's core question) — a clean paired choice

> **"Which finish?"** ◻ Bright silver-tone (rhodium-plated) ◻ Warm gold-tone (gold-plated)

Both options are named neutrally and must be **defined correctly per the FTC Jewelry Guides, 16 CFR Part
23** [FACT]. The poll may test *preference between correctly-defined finishes*; it must **not** assert a
finish claim as demand (that's `jewel_materials`).

### Style within the winning category (only after you have a winner — don't ask before)

> **"Which [earrings] do you wear most?"** ◻ Studs ◻ Hoops ◻ Huggies ◻ Drops

(4-option quiz or paired funnel; swap in the winning category's style set.)

### Price / willingness-to-pay — **method: Gabor-Granger** (off-platform form only)

A like-poll carries **zero** price information, so WTP is a separate, explicit instrument [FACT,
WTP_INSTRUMENTS]. Ask buy-likelihood at ascending price points:

> **"At $X, how likely are you to buy this?"**
> ◻ Very likely ◻ Somewhat likely ◻ Not likely
> — repeat the question at **$19 / $29 / $39 / $49**.

_(Gabor-Granger method: the buy-likelihood curve across the four price points gives the price band.
Alternatively run a Van Westendorp four-question price-sensitivity battery. The price bands are a
**method to obtain** the WTP number — the figures themselves are [UNKNOWN] until the poll runs.)_

**Pretest first** [FACT-method, PILOT_COGNITIVE_PRETEST]: read the wording aloud to **5–10 women in the
cohort** ("what does this question mean to you?") to catch ambiguity before you spend reach. Cheap
insurance.

---

## 3. Incentive design — built to MINIMIZE bias

The owner's instinct ("offer something free to attract answers") is right for **lift** but is the single
biggest **bias** risk: a broadcast free-jewelry giveaway recruits **freebie-hunters and giveaway-farm
accounts** whose demographics/intent may not match the 18–25 buyer — so you'd rank preferences on the
**wrong population** [FACT-mechanism, INCENTIVE_AUDIENCE_MISMATCH]. **Bigger n does not fix this** —
self-selection is a systematic offset that does not average out; a large incentivized sample is a *more
precisely wrong* number, not a better one [FACT, SELF_SELECTION_BIAS].

**The six bias-minimizing levers** [METHOD, INCENTIVE_DESIGN_TRADEOFF]:

1. **Give away one of your own jewelry pieces**, not cash or a gift card — a jewelry prize filters toward
   jewelry-interested respondents; generic cash maximizes freebie-hunters.
2. **Single random draw**, not "everyone who answers gets $X" — draws attract fewer straightliners.
3. **Decouple the prize from the answer, and say so** — entry does not depend on *which* option you pick.
   Removes the demand-characteristic pull toward a "winning" answer.
4. **Collect the preference first, offer entry after** — the answer is given before the prize is salient.
5. **Modest prize value** — a smaller prize lifts less but draws fewer bots/farms (and keeps you under
   the legal triggers — see §5).
6. **Measure the bias — don't assume it away** — run an **incentivized-vs-not A/B split**; if the two
   arms rank categories differently, the incentive is distorting the read. Realized lift and bias are
   **[UNKNOWN]** until this A/B measures them.

**Fraud / bot controls** [FACT, FRAUD_BOT_CONTROL] — treat entry fraud as the *predictable* cost of a
prize: attention/qualification checks; **deduplication** on account/email/device; **US + 18+ eligibility
and geo screening**; rate-limiting (one entry per person); and — critically — **separate the sweepstakes
ENTRY act from the PREFERENCE answer**, counting preferences only from verified on-target respondents.
Residual fraud after controls is [UNKNOWN] — estimate it, don't assume zero.

### Exact incentive copy line (paste after the question, before the entry field)

> **"Thanks for your answer! Want in on the giveaway? Enter to win one of our pieces — one winner, chosen
> at random. No purchase necessary. Your answer doesn't affect your odds. US, 18+. See Official Rules."**

_(This line does the work of levers 1–4 at once: names the relevant jewelry prize, a single random draw,
decouples odds from the answer, and gates entry after the preference is captured — with the No-Purchase-
Necessary and eligibility hooks the legal envelope requires.)_

---

## 4. Platform mechanics — running it as a paid ad poll to US women ~18–25

**Targeting** [METHOD, AUDIENCE_DEFINITION_18_25]: age **18–24/25**, gender **women**, geo **US**, plus
interest/behavior affinities (fashion, jewelry) and **lookalike/custom audiences** seeded from your
existing pixel/customer list — the store is already live, so this seed exists. Two binding caveats
[FACT]:
- Platform age/interest data is an **inferred proxy, not a census** — don't read the targeting spec as
  proof of who actually answered.
- **The optimizer, not you, curates the responder slice** — delivery narrows to whoever is cheapest to
  get the event from, skewing the responding sample in ways the spec doesn't show.
- **No protected-class (e.g. ethnicity) targeting** — off-limits on Meta/TikTok and out of scope.

**Native sticker vs off-platform form — the trade-off:**

| Route | Carries | Pro | Con |
|---|---|---|---|
| **Native paid vote** (IG polling-sticker Stories ad; TikTok Voting Sticker) [FACT-as-of 2026-07] | 2 options only | Cheapest reach, in-feed, low friction → high response | Can host **only** Poll A screens + the paired-comparison funnel; **cannot** host MaxDiff / WTP |
| **Off-platform form** (Typeform / Google Form / Sawtooth), driven by a link/lead ad | Unlimited — MaxDiff, WTP, style | The **cleanest, rankable vote**; randomizes order; the recommended ranker | Link-out lowers completion → higher cost-per-response |

**Objective ↔ response-unit match** [METHOD, CAMPAIGN_OBJECTIVE_CHOICE]: a cheap **Engagement** objective
is fine for the coarse Poll A screen; the **MaxDiff/attribute polls that must be rankable with confidence**
should optimize for **Traffic (link clicks) or Leads/Conversions** — the completed off-platform answer is
the cleanest vote, at higher cost per response.

**Minimum usable-N target (the per-cell trap is load-bearing)** [METHOD, SAMPLE_SIZE_MIN_SIGNAL]: a
5-category poll **splits n across cells** — each option's own count sets its precision, not the total.

| Usable n (per option cell) | Approx. MoE (95%, p=0.5) |
|---|---|
| ~100 | ±~10% |
| ~384 (total) | ±5% overall |
| ~1,000 | ±~3% |

To read the category rank at usable per-option precision (~±8–10%), target **~100–150 usable responses
per category → ~500–750 usable responses total** [METHOD]. The MaxDiff needs enough respondents that each
item appears in enough subsets — plan the same **~500+ usable** order of magnitude.

**Roughly what reach that needs** (illustrative scaffolding — every input is a placeholder you replace
with your own live numbers, **NOT a quote and NOT a benchmark to trust**): CPMs vary widely by platform,
audience and season and MUST be pulled live from Ads Manager before you budget. As an *illustrative
placeholder only*, take CPM ≈ **$7** [ESTIMATE, illustrative — pull live], link-CTR **1%** [ESTIMATE,
illustrative], off-platform completion **40%** [ESTIMATE, link-then-form funnel] → ~**4 usable responses
per 1,000 impressions** → **CPR ≈ $1.75** [ESTIMATE, illustrative]. That would imply **~125k–190k
impressions** for 500–750 usable responses, at **~$875–$1,310** in spend [ESTIMATE, illustrative]; a
coarser n≈384 read ≈ **~$670** [ESTIMATE, illustrative]. **Every CPM/CTR/completion/CPR figure here is
[UNKNOWN] until your own campaign measures it — treat the whole calculation as scaffolding for your
Ads-Manager pull, not a cost you can quote.**

**READ-OUT rule — the weak-signal / margin-of-error caveat (do NOT over-read a thin result)** [FACT,
WEAK_SIGNAL_THRESHOLD]:
1. Define a **response = an option choice** before counting; an impression, view, scroll-past, or
   undifferentiated like/follow is **not** a vote. Record the **usable fraction** after removing
   bots/dupes/junk.
2. Normalize to a **rate** (responses per impression / per dollar), **not a raw count**, so an option
   that merely got more budget can't win on exposure.
3. **Rotate creative/format/placement** to control confounds so category preference isn't confounded with
   creative appeal.
4. Test **separability** (CI-overlap check or two-proportion z-test, with a multiple-comparison caution
   across 5 options).
5. **Apply the weak-signal rule — refuse to rank if any of:** top options' CIs **overlap**; usable n is
   **below** the minimum; the gap is **inside the bias band**; or CPR was so high the budget couldn't buy
   a decisive n. When weak → **collect more, simplify the question, or fall back** to the market read +
   store sell-through, and **label the poll inconclusive.** Forcing a decision on a weak signal is worse
   than admitting it didn't resolve.
6. **Feasibility gate:** if the budget to reach a decisive per-cell n exceeds what you'll spend, the
   honest output is _"the poll can't answer that at that granularity"_ — simplify to fewer options or
   accept a coarser read.

**One-line output shape handed downstream** _(illustrative structure, NOT measured values)_:
> `Rank: [Earrings 0.31±0.06 (n=142) | Necklaces 0.27±0.06 (n=138) | …]; Earrings vs Necklaces: CIs`
> `overlap → NOT separable (tie); finish read: gold-tone > silver-tone, separable; legal: AMOE+rules`
> `cleared, ARV ≤$500 (no NY/FL/RI trigger); bias: incentivized arm agreed with control.`

---

## 5. Legal quick-check box (sweepstakes essentials)

**[FACT-source: general US sweepstakes / promotion law]** — this is the law, **not** legal advice for
your specific filing. **Confirm each item with counsel and the live Meta/TikTok promotion guidelines
before launch.** No statute number below is invented; where a specific citation isn't grounded it is left
general on purpose.

- [ ] **No illegal lottery.** A promotion combining **consideration + chance + prize** is an illegal
      private lottery. Remove *consideration*: post a clear **"No Purchase Necessary"** and offer a free,
      **equal-dignity Alternative Method of Entry (AMOE)** giving non-buyers the **same odds** (e.g. a
      free web/mail-in entry). [FACT-US promotion law]
- [ ] **Official Rules published** — sponsor, eligibility (US, 18+, exclusions), start/end dates, the free
      AMOE, odds, prize description + **approximate retail value (ARV)**, winner selection/notification.
      [FACT]
- [ ] **Keep ARV under the state triggers.** **NY and FL require registration + a surety bond when the
      aggregate prize pool exceeds $5,000; RI requires registration/notice for retail prizes over $500**
      [FACT-source]. **Keep total prize ARV modest (≤ ~$500)** — a single mid-tier plated piece fits
      easily — and you stay under the NY/FL bond+registration triggers **and** the RI retail threshold.
      This is also the lower-bias choice (§3, lever 5). Confirm current thresholds with counsel.
- [ ] **Single random draw**, decoupled from the answer (§3). One entry per verified person.
- [ ] **FTC material-connection disclosure** — the giveaway tied to responding/entering is a material
      connection; disclose **clearly and conspicuously, in plain language, in the same place as the
      message** (not buried in a hashtag block). The advertiser (you) is responsible. [FACT-source: FTC
      Endorsement Guides, 16 CFR Part 255]
- [ ] **Eligibility / age gate** — **US, 18+.** Data-minimize (collect only what the draw needs). Email
      collection brings **CCPA/CPRA** notice-at-collection + opt-out/deletion and **CAN-SPAM** (clear
      opt-out, honest headers) duties; if reach could capture **under-13s, COPPA** bars collecting their
      data without verifiable parental consent. [FACT]
- [ ] **Platform promotion rules** — comply with the live Meta/Instagram and TikTok promotion guidelines
      (release, no inaccurate tagging). Re-verify on build day. [FACT-as-of 2026-07]

**Escalation flags — stop and route to counsel if any fires:** any prize pool with
consideration+chance+prize lacking a No-Purchase-Necessary AMOE; any pool over **$5,000** in NY/FL (or
**$500** retail in RI); any missing FTC disclosure; or any reach to under-18s.

---

## 6. Handoff — where the results go

The read-out flows into **`jewel_assortment`**, the convergence node that locks the six SKUs, via the
**ORCHESTRATION.md feedback loop** (`ORCHESTRATION.md` §2 pipeline + §4 nested loops):

1. **Into `jewel_assortment` kb1 (`kb1_signal_synthesis_ranking`)** — the confidence-tagged ranked
   candidate list (category rank + finish read + style + WTP band), each item tagged with its normalized
   share, CI, usable n, separability verdict, stated-vs-revealed status, and legal-clearance/bias status.
   kb1 **fuses** this poll [SIGNAL] with the market read and the owner's own store sell-through,
   on the single **revealed > stated > proxy** reliability ladder that `REALTIME_DATA_COLLECTION.md`
   also uses: **owner's own store sell-through (revealed, first-party) > a corroborated revealed
   web/marketplace proxy (bestseller rank + review-velocity, ≥2 independent sources) > a validated poll
   read > a stated poll vote > an uncorroborated inferred market estimate.** (Only the *uncorroborated*
   web inference sits at the bottom; a *corroborated* revealed marketplace proxy outranks a stated poll
   vote — the two docs agree on this.)
2. **Into `jewel_assortment` kb2 (`kb2_launch_set_of_six`)** — the fused ranking sets the **depth
   category** (the winner gets the depth slots) and the **dominant plating bath** (the finish winner sets
   the primary bath; the loser becomes the accent or is cut). kb2 picks the **launch set of six** subject
   to the constraint prune (contribution-margin floor, MOQ × COGS × six, plating-batch count, aggregate
   capital ceiling).
3. **Through the feedback loop** — poll returns landing after the first pass re-enter via the **outer
   loop** (`kb3_test_iterate_protocol`): _new poll data past the minimum-signal n → re-fuse in
   `kb1_signal_synthesis_ranking` → re-rank → the six SKUs re-lock._ Per ORCHESTRATION invariants, the
   ranked six is **trusted only after `jewel_economics` re-gates it** on margin/CAC.

**This kit stops before the final six-SKU pick — that is `jewel_assortment`'s.** If the signal is weak or
inconclusive (§4 read-out rule), hand it off **labeled inconclusive** and let `jewel_assortment` fall
back to the market prior + store data rather than force a rank on noise.

---

_Every figure above carries its tag. Every poll % is a responder **[SIGNAL]**, never demand. Platform ad
affordances are dated **[FACT-as-of 2026-07]** to re-verify in Ads Manager before the build. The incentive
is designed inside the FTC/sweepstakes/privacy envelope, with prize ARV kept ≤ ~$500 to stay under the
NY/FL $5,000 bond and RI $500 retail triggers. The final six-SKU pick is left to `jewel_assortment`._
