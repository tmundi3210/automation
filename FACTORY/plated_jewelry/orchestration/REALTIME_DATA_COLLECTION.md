# REALTIME_DATA_COLLECTION.md — Method spec for the real-time public-data lane (S1-RT)

_Orchestration-layer METHOD spec. Machine + operator facing. This document specifies HOW a single general-purpose data-collection agent (WebSearch/WebFetch) gathers publicly observable demand signal for the plated fashion-jewelry launch. It does NOT contain gathered data and it invents no numbers. Every load-bearing claim in any output this lane produces carries a tag: `[FACT]` (established, unsourced-but-true domain fact), `[FACT-source]` (a specific fetched public page/rank/count with a URL + date), `[ESTIMATE]` (a derived number that MUST name its method), `[METHOD]` (a recipe to obtain a number not yet pulled), `[SIGNAL]` (an engagement/interest proxy, not a purchase), `[UNKNOWN]` (an honest gap — used whenever a source is not fetchable). This mirrors the standing FACTORY honesty rule the whole vertical was built under: **a number you cannot source is a `[METHOD]` to obtain it or an `[UNKNOWN]`, never a fabricated figure.**_

**Lane ID:** `S1-RT` (real-time public-data collector) — an optional alternative/complement to the paid poll lane (`jewel_poll` / S4).
**Consumes:** live public web (shopping sites, social, search-interest tools, news/trade press).
**Feeds:** `jewel_market` kb3 (`trend_and_competitor_mapping`) as raw tagged trend signal; then `jewel_assortment` kb1 (`signal_synthesis_ranking`) where this lane's evidence becomes one of the three ranked signals (it is the S1 "public-data market read" leg of that specialist's SIGNAL_INVENTORY).
**Owner brief context:** plated fashion/costume-tier DTC brand, US, women ~18-25, launching 6 SKUs, website already live. Goal of this lane: find WHICH categories / shapes / finishes show the strongest **independent** demand signal from publicly observable data.

---

## 1. What this lane IS / IS NOT

**IS:** a **publicly-observable-signal collector**. It runs read-only queries against public shopping, social, search-interest and editorial surfaces, tags each observation, buckets it to the shared taxonomy, and emits a **TAGGED EVIDENCE TABLE** with a confidence per row. It answers exactly one question:

> *"Which categories / shapes / finishes / price-bands show the strongest INDEPENDENT demand signal in publicly observable data — corroborated across ≥2 unrelated sources?"*

**IS NOT:**
- **NOT a sales oracle.** It never emits a unit-sales count, market size, revenue figure, plated-vs-solid share, or purchase rate as a measured actual. Public rank and public review-count are `[FACT-source]`; the underlying units sold are `[ESTIMATE]` (with a named method) or `[UNKNOWN]`.
- **NOT the poll.** It is *complementary* to `jewel_poll` (S4) and shares the same downstream consumer. The poll yields a **stated** preference `[SIGNAL]` from a paid, self-selected social audience; this lane yields **revealed / behavioral proxies** from the open web (bestseller ranks, review velocity, sold-out states). Neither is demand; both are triangulation inputs. On the revealed>stated>proxy reliability ladder used by `jewel_assortment`, a marketplace bestseller rank is a *revealed-behavior proxy* and outranks a stated poll vote — but only when independently corroborated (see §4).
- **NOT the market specialist.** It does not size the market, segment the buyer, or name competitors as verified rivals. It hands raw tagged observations to `jewel_market` kb3, which does the interpretation under its own honesty contract.
- **NOT a scraper that guesses.** If a source is blocked by robots.txt, paywalled, JS-gated, or otherwise not fetchable, the row is `[UNKNOWN]` — never back-filled from memory or "typical" values.

**Output contract:** the deliverable is the evidence table of §5. One row = one observation from one URL on one date. Interpretation happens downstream. This lane's job is *honest, tagged, deduped, triangulated collection.*

---

## 2. SOURCE CATALOG

Grouped by signal family. Each row: **source → what's observable → what signal it proxies → honesty tag(s)**. All "as-of" platform mechanics are `[FACT-as-of]` and MUST be re-verified live before a run — platforms change surfaces, remove features, and gate access without notice.

### 2A. Retail / marketplace SALES-PROXY signals (revealed-behavior proxies — the strongest family, but still proxies)

| Source | What's observable | What it proxies | Honesty tag |
|---|---|---|---|
| **Amazon Best Sellers** (Jewelry > subcats: Earrings, Necklaces, Rings, Bracelets, Anklets; Fashion vs Fine nodes) | Ordinal rank 1..100 per subcat, updated hourly | Relative sales velocity *within that node right now* | rank = `[FACT-source]`; implied units = `[ESTIMATE via rank-to-velocity method]` / `[UNKNOWN]` |
| **Amazon Movers & Shakers** (same jewelry nodes) | Biggest 24h rank *gainers* | Acceleration / emerging demand (leading signal) | rank movement = `[FACT-source]`; cause = `[UNKNOWN]` |
| **Etsy "Bestseller" badge** + **"Popular now"** shelves | Which listings carry the badge; what's surfaced as popular | Sustained sales velocity for that shop/listing (Etsy awards on rolling sales) | badge present = `[FACT-source]`; volume = `[ESTIMATE]`/`[UNKNOWN]` |
| **"Frequently bought together" / "Customers also bought" / "Customers who viewed also viewed"** (Amazon, marketplace PDPs) | Co-purchase / co-view adjacency graph | Which forms bundle/stack together (cross-sell structure) | adjacency = `[FACT-source]`; strength = `[SIGNAL]` |
| **TikTok Shop** top/trending products + **Instagram Shop** featured/trending | Products surfaced as trending; "X sold" counters where shown | Social-commerce velocity for the young-women cohort | surfaced-as-trending = `[FACT-source]`; "sold" counters = `[FACT-source]` for the number shown, `[ESTIMATE]` for true units (platform-defined, often lifetime/rounded) |
| **Google Shopping** popularity / "popular products" / best-of surfaces | Which products/queries Google merges as popular | Cross-merchant purchase interest | `[FACT-source]` for surfacing; `[SIGNAL]` for magnitude |
| **Restock cadence & sold-out states** (any storefront/marketplace) | "Sold out", "only N left", "back in stock", waitlist, repeated restocks of the same SKU | Demand outrunning supply (a strong revealed proxy — nobody restocks a dead SKU) | state observed on date = `[FACT-source]`; inferred demand = `[ESTIMATE]`/`[SIGNAL]` |

> **Rank-and-velocity caveat (binding for this whole group):** a **bestseller RANK is `[FACT-source]`** (you read it off a public page). The **underlying unit sales are `[ESTIMATE]` via an explicitly named method or `[UNKNOWN]`.** Rank and review-velocity are a **PROXY, not a count.** Rank is ordinal (rank 1 vs rank 2 says nothing about the *gap*), node-relative (rank within "Anklets" ≠ comparable to rank within "Necklaces"), and velocity-weighted/recency-biased. Never convert a rank to a unit number without a stated method, and label the result `[ESTIMATE]`.

### 2B. Search / interest signals (intent proxies — relative, never absolute)

| Source | What's observable | What it proxies | Honesty tag |
|---|---|---|---|
| **Google Trends** | Relative interest index (0-100), rising/breakout queries, geo/time cuts | Relative, seasonally-shaped search *intent* — NOT volume | `[FACT-source]` for the index value + window; `[FACT]` that the index is **relative not absolute** |
| **Pinterest Trends / Pinterest Predicts** | Trending search terms, YoY growth callouts, annual forecast | Forward-looking aesthetic/planning intent (Pinterest leads purchase for jewelry/fashion) | trend listed = `[FACT-source]`; forecast = `[FACT-source, opinion/forecast]` |
| **Autocomplete / "related queries" / "People also search for"** (Google, marketplace search bars) | Query completions and adjacencies | Salience and the language real buyers use (motif/finish/shape vocabulary) | `[FACT-source]` for the suggestions; `[SIGNAL]` for demand |
| **Keyword volume tools** (any public/free tier: Google Keyword Planner ranges, etc.) | Search-volume *bands*, competition, seasonality | Absolute-ish interest scale (banded) | band = `[FACT-source]` (tool-reported); exact volume = `[ESTIMATE]` (tools model, not measure) |

> **Interest caveat:** Google Trends is **relative** (indexed to its own max in the chosen window/geo) — two separate charts are NOT comparable unless queried in one multi-term request. Search intent ≠ purchase; it leads purchase and is seasonally distorted. Read as ordinal shape, not a unit count.

### 2C. Social listening (ENGAGEMENT proxies — vanity-metric caveat applies)

| Source | What's observable | What it proxies | Honesty tag |
|---|---|---|---|
| **TikTok / Instagram hashtags** | View counts, post counts, growth rate per hashtag (e.g. a shape/motif/finish tag) | Attention/reach for a look — NOT demand | count = `[FACT-source]`; demand = `[SIGNAL]` |
| **TikTok / Instagram sounds & trends** | Which sound/format a style rides, its spread | Virality vehicle & lifecycle stage | `[FACT-source]` for the mechanic; `[SIGNAL]` |
| **Reddit** (r/jewelry, r/femalefashionadvice, r/TikTokFashion, etc.) | Threads, upvotes, recurring questions/asks, "where do I buy X" | Unincentivized organic interest & pain points (higher trust than paid social) | thread/upvotes = `[FACT-source]`; representativeness = `[SIGNAL]` |
| **Comment sentiment & recurring asks** (on top posts/PDPs/videos) | What people repeatedly ask for, complain about, tag friends on | Latent demand, objections (sizing, tarnish, price) | pattern = `[SIGNAL]`; any rate = `[ESTIMATE via coded-sample method]` |
| **Creator / UGC velocity** | How many distinct creators feature a form over time, follower spread | Breadth of organic adoption (breadth beats one viral spike) | count = `[FACT-source]`; demand = `[SIGNAL]` |

> **Vanity-metric caveat (binding for this group):** views, likes, hashtag counts and follower numbers are **`[SIGNAL]`, never purchase.** They are algorithmically amplified, sponsorable, bot-inflatable, and count *reach* not *revenue*. A hashtag with billions of views can convert to near-zero sales. Social is the **earliest** family and the **most fad-inflated and manipulable** — it is a lead indicator to be *confirmed* by the sales-proxy family, never actioned alone (see §4 triangulation).

### 2D. Reviews as a purchase proxy (an observable lower-bound on units)

| Source | What's observable | What it proxies | Honesty tag |
|---|---|---|---|
| **Review COUNT** on a listing (Amazon/Etsy/marketplace/DTC) | Cumulative # of reviews | A **lower-bound** proxy for cumulative units sold (only a fraction of buyers review) | count = `[FACT-source]`; implied units = `[ESTIMATE]` (needs a review-rate assumption) / `[UNKNOWN]` |
| **Review VELOCITY** (new reviews / unit time) | Count now vs count on a prior fetch date; dated reviews | Current sales *rate* (more robust than cumulative count for "selling NOW") | velocity = `[ESTIMATE via two-date-delta METHOD]`; each dated review = `[FACT-source]` |

> **Review method (state it every time):** cumulative review count is a censored, biased-low proxy — review-submission rates vary by category, price and platform, so **never assert units = reviews × k with an invented k.** Prefer **velocity**: `Δreviews ÷ Δdays` between two dated fetches, which cancels the unknown listing-age and partly the unknown review-rate. Normalize velocity per-listing then compare across the *same* platform only (cross-platform review cultures differ). Output review-derived unit figures as `[ESTIMATE]` naming the method and the assumption, or `[UNKNOWN]`.

### 2E. News / articles / trend reports (context — opinion/forecast, not measurement)

| Source | What's observable | What it proxies | Honesty tag |
|---|---|---|---|
| **Trade press** (WWD, Vogue Business, Business of Fashion, retail trend forecasters/agencies) | Named trends, season calls, category commentary | Editorial/industry narrative & forward forecast | claim made = `[FACT-source]`; the trend call itself = `[FACT-source, opinion/forecast]` |
| **Retailer / brand press releases & earnings color** | Stated "top category", launch news, sell-out claims | Company's own (marketing-shaped) demand claim | statement = `[FACT-source]`; truth of the claim = `[UNKNOWN]` (self-reported, unaudited) |
| **Season trend roundups** (magazine/blog "jewelry trends 2026") | Consensus of forms/finishes being pushed | Supply-side narrative & merchandiser intent | `[FACT-source, opinion/forecast]` |

> **Editorial caveat:** a trend article is **opinion/forecast**, tag as such. It is `[FACT-source]` that *the claim was made* by that outlet on that date — it is **not** evidence the trend is real demand. Editorial is a lagging/narrative family used for context and to name candidate motifs to go verify against the sales-proxy family, never as a demand measurement.

---

## 3. WHICH DATA TO ANALYZE for "most sold / most frequently bought"

A signal → interpretation table. For each: what it **does** tell you and what it **does NOT.** All buckets map to the shared taxonomy (§4 step 5).

| Signal | What it tells you | What it does NOT tell you | Tag of the raw datum |
|---|---|---|---|
| **Bestseller rank** (Amazon/Etsy node) | Relative velocity ordering *within one node, right now* — which subtype is hot | Absolute units; cross-node comparison; profit; whether it's plated vs solid | `[FACT-source]` |
| **Rank movement** (Movers & Shakers) | *Acceleration* — what's emerging vs decaying (leading signal) | Whether the spike is durable or a fad; the base level it moved from | `[FACT-source]` |
| **"Frequently bought together" adjacency** | Which forms are **bought/worn as a set** (stacking, matching sets → cross-sell & bundle structure) | Standalone demand magnitude; causal direction | `[FACT-source]` (adjacency), `[SIGNAL]` (strength) |
| **Review count** | Cumulative **lower-bound** on units for that listing; longevity | Current rate (old listings accumulate); true units (review-rate unknown) | `[FACT-source]` → `[ESTIMATE]` for units |
| **Review velocity** (Δ/time) | Current **sales rate** proxy — "selling now" | Absolute units; margin; returns | `[ESTIMATE via delta METHOD]` |
| **Sold-out / restock frequency** | Demand **outrunning supply** — a strong revealed signal; repeated restocks = durable seller | The size of demand; could also be thin inventory/ops, not high demand | `[FACT-source]` (state), `[ESTIMATE]` (demand) |
| **Search interest trend** (Trends/Pinterest) | Direction & seasonality of **intent**; rising vs falling; buyer vocabulary | Volume in units; that intent converts; absolute cross-term comparison | `[FACT-source]` (relative) |
| **Social engagement velocity** (hashtag/UGC growth) | **Breadth & speed** of attention; lifecycle stage; earliest lead | Purchases (vanity metric); authenticity (bot/sponsor inflation) | `[SIGNAL]` |
| **Price-band clustering of top sellers** | Where the winning price points sit (mass/entry vs mid-fashion) — tier the launch should hit | Realized transaction price (listed ≠ realized: discounts/promos/bundles); margin | listed price = `[FACT-source]`; realized = `[UNKNOWN]` |

**Reading rule:** convert each signal to an *ordinal* read at the **subtype** grain (huggie hoops, paperclip chain, tennis bracket, signet ring — never the flat top-category), because a hot subtype hides inside a flat category average. Roll up to category only after the subtype read.

---

## 4. ANALYSIS METHOD — the pipeline the collection agent runs

```
QUERY PLAN → COLLECT (per source) → NORMALIZE (recency-weight + dedupe) →
TRIANGULATE (≥2 unrelated sources) → BUCKET (category/shape/finish/price-band) →
EMIT tagged evidence table (confidence/row) → HANDOFF (jewel_market.kb3 → jewel_assortment.kb1)
```

**Step 0 — Frame & taxonomy lock.** Fix the buckets before collecting so every observation lands somewhere:
- **Category:** earrings / necklaces / rings / bracelets / anklets (`jewel_market` kb1 taxonomy; shared with `jewel_form`).
- **Shape/motif:** the buyable subtype (studs, huggie/hoop, drop; paperclip/rope/tennis/pendant chain; signet/band/stacking ring; cuff/chain/tennis bracelet; etc.).
- **Finish:** the FTC-anchored ladder — gold-plated / gold electroplate / gold-filled / vermeil / **rhodium-plated sterling silver** / stainless-steel-tone / base. Finish wording on listings is normalized onto the FTC 16 CFR Part 23 definitions `[FACT]`; vague terms ("gold-tone", "gold-dipped") are tagged `[UNKNOWN]`-quality, never counted as an FTC class.
- **Price-band:** entry / mid-fashion / premium-fashion (from LISTED prices; realized-price gap flagged `[UNKNOWN]`).

**Step 1 — Query plan.** For each category × candidate shape × candidate finish, pre-write the queries per source family (bestseller node URLs, Trends terms, hashtag lookups, Reddit searches, trade-press searches). Hold the query list so the run is reproducible and re-runnable (§6). Cover the young-women US framing in query terms.

**Step 2 — Collect per source.** WebFetch/WebSearch each planned query. For every datum capture: `source, url, observation (verbatim value), date-fetched`. If a fetch fails (robots/paywall/JS-gate/egress block) → write the row as `[UNKNOWN]` with the reason; **do not** substitute a remembered or "typical" value. Respect robots.txt and platform ToS; read-only, no login-walled scraping.

**Step 3 — Normalize.**
- **Recency-weight:** weight fresher observations higher; stamp every row with its date; down-weight anything whose date can't be established. Prefer *velocity/movement* signals over *cumulative* ones for a "selling now" read.
- **Dedupe:** collapse the **same product/shape appearing across multiple sources** into one entity so a single item featured on Amazon + TikTok + a blog isn't triple-counted as three independent signals. Dedup key = normalized {shape + finish + brand/listing identity}. Crucially, dedupe **before** triangulation so corroboration counts only *genuinely unrelated* sources.

**Step 4 — TRIANGULATE (the core gate).** A signal is labeled **"strong"** only if **independently corroborated by ≥2 *unrelated* sources** (unrelated = different signal families or different platforms not sharing an origin; a blog quoting Amazon is NOT a second source). Encode the corroboration count per bucket. One viral TikTok alone → `[SIGNAL]`, confidence low, "single-family, unconfirmed." Bestseller rank + rising search + restock frequency all pointing to huggie hoops → high confidence. Disagreement across families is **diagnostic, not averaged** — record the split, don't split the difference into a false middle.

**Step 5 — Bucket.** Map every surviving observation to its {category, shape, finish, price-band} bucket (Step 0). This is the grain `jewel_market` (kb3) and `jewel_form` consume.

**Step 6 — Emit the tagged evidence table** (schema in §5) with a **confidence per row** driven by: (a) source reliability (revealed sales-proxy > search intent > social engagement > editorial), (b) recency, (c) corroboration count from Step 4, (d) directness-to-purchase.

**Step 7 — Handoff** (see §5 handoff block).

### Binding guards (apply throughout)
- **VANITY-METRIC guard:** social views/likes/hashtags/followers are `[SIGNAL]` (reach), never purchase. Never let a social number alone elevate a bucket to "strong."
- **SURVIVORSHIP guard:** bestseller lists, restocks, and top-of-search show *what already exists and is already stocked/merchandised*. Absence of a shape from a bestseller list is **not** evidence of no demand — it may be unstocked/never-tried. A never-listed shape's "zero" is absence, not rejection. Sampled storefronts are survivors; flag the frame.
- **RECENCY guard:** trends decay; stamp dates, recency-weight, and prefer movement/velocity over cumulative stock metrics. A rank read once is a snapshot, not a trend — a trend needs ≥2 dated reads.
- **THE TAG RULE (non-negotiable):** **public rank + review counts are `[FACT-source]`; any inferred unit volume is `[ESTIMATE]` with a stated method; social buzz is `[SIGNAL]`.** No bucket ever carries a fabricated unit count, market size, %, share, demographic/ethnicity rate, competitor metric, or realized price. Unavailable → `[METHOD]` or `[UNKNOWN]`.

---

## 5. THE COLLECTION-AGENT PROTOCOL (copy-paste)

> Inject the block below **verbatim** into ONE general-purpose agent equipped with WebSearch + WebFetch to run the S1-RT lane.

```
ROLE: You are the S1-RT real-time public-data collection agent for a plated fashion-jewelry
DTC launch (US, women ~18-25, 6 SKUs, site already live). You gather PUBLICLY OBSERVABLE
demand signal and emit a TAGGED EVIDENCE TABLE. You do NOT gather sales figures, you do NOT
invent numbers, you do NOT pick SKUs, you do NOT size the market.

HONESTY CONTRACT (binding): tag every load-bearing datum:
  [FACT]        established true domain fact, no URL needed
  [FACT-source] a specific fetched public page value — MUST carry url + date
  [ESTIMATE]    a derived number — MUST name the method + assumption
  [METHOD]      a recipe to get a number you could not fetch
  [SIGNAL]      an engagement/interest proxy (NOT a purchase)
  [UNKNOWN]     an honest gap — use this whenever a source is not fetchable
Rules: bestseller RANK and review COUNT are [FACT-source]; any inferred UNIT VOLUME is
[ESTIMATE] with a stated method; social views/likes/hashtags are [SIGNAL], never purchase.
NEVER assert a market size, unit count, %, plated-vs-solid share, demographic/ethnicity rate,
competitor metric, or realized price as measured. If a source is blocked by robots.txt, a
paywall, a login wall, JS-gating, or an egress limit → mark that row [UNKNOWN] with the reason.
NEVER back-fill a blocked value from memory or "typical" numbers. Respect robots.txt/ToS;
read-only; no login-walled access.

TAXONOMY (bucket every observation to these):
  category  = earrings | necklaces | rings | bracelets | anklets
  shape     = the buyable subtype (studs, huggie/hoop, drop; paperclip/rope/tennis/pendant;
              signet/band/stacking; cuff/chain/tennis; etc.)
  finish    = gold-plated | gold-electroplate | gold-filled | vermeil |
              rhodium-plated-sterling | steel-tone | base  (normalize listing wording onto
              FTC 16 CFR Part 23; vague "gold-tone"/"gold-dipped" → [UNKNOWN]-quality, don't
              count as an FTC class)
  price_band= entry | mid-fashion | premium-fashion  (from LISTED price; realized = [UNKNOWN])

PIPELINE:
  1. QUERY PLAN: for each category × candidate shape × candidate finish, list queries per
     source family (see SOURCES). Keep the list — the run must be reproducible.
  2. COLLECT: WebFetch/WebSearch each. Capture verbatim value + url + date. Blocked → [UNKNOWN].
  3. NORMALIZE: date-stamp + recency-weight everything; prefer velocity/movement over
     cumulative. DEDUPE the same product/shape across sources BEFORE triangulating.
  4. TRIANGULATE: a bucket is "strong" only if ≥2 UNRELATED sources corroborate (different
     family or unrelated platform; a blog quoting Amazon is not a 2nd source). Record the
     corroboration count. Single-source → confidence low. Disagreement → record the split,
     do NOT average.
  5. BUCKET every surviving observation to {category, shape, finish, price_band}.
  6. EMIT the evidence table (schema below), one row per observation, confidence per row.
  7. Apply guards: VANITY-METRIC (social=[SIGNAL] only), SURVIVORSHIP (absence≠no-demand;
     bestseller lists show only what's already stocked), RECENCY (trend needs ≥2 dated reads).

SOURCES (re-verify each platform's surfaces live; mechanics drift):
  SALES-PROXY: Amazon Best Sellers + Movers&Shakers (Jewelry subcats); Etsy Bestseller badge
    / Popular-now; "frequently bought together" / "customers also bought"; TikTok Shop &
    Instagram Shop trending + "X sold" counters; Google Shopping popular products; sold-out /
    restock / "only N left" states.
  SEARCH/INTENT: Google Trends (RELATIVE not absolute); Pinterest Trends/Predicts; autocomplete
    + related queries; keyword-volume tools (banded [ESTIMATE]).
  SOCIAL: TikTok/Instagram hashtag view+post counts & growth; sounds/formats; Reddit
    (r/jewelry, r/femalefashionadvice, r/TikTokFashion) threads/upvotes/recurring asks; comment
    sentiment & repeated asks; creator/UGC velocity.  ← all [SIGNAL], vanity-metric caveat.
  REVIEWS: review COUNT (=[FACT-source], lower-bound units); review VELOCITY = Δreviews÷Δdays
    between two dated fetches ([ESTIMATE], name the assumption; never units=reviews×invented-k).
  NEWS/EDITORIAL: WWD / Vogue Business / BoF / trend forecasters; retailer press releases;
    season "jewelry trends" roundups.  ← [FACT-source] that the claim was made;
    the trend call itself = [FACT-source, opinion/forecast]; self-reported sell-outs = [UNKNOWN].

OUTPUT SCHEMA (markdown table AND a JSON array; one object per observation):
  {
    "source":        "<platform/outlet>",
    "url":           "<fetched url>",              // "" and honesty_tag=[UNKNOWN] if unfetchable
    "date":          "YYYY-MM-DD",                 // date fetched
    "observation":   "<verbatim value, e.g. 'Huggie hoops = Amazon Earrings BSR #3'>",
    "signal_type":   "bestseller_rank | rank_movement | co_purchase_adjacency |
                      review_count | review_velocity | soldout_restock | search_interest |
                      social_engagement | price_band | editorial",
    "category":      "earrings|necklaces|rings|bracelets|anklets|UNKNOWN",
    "shape":         "<subtype or UNKNOWN>",
    "finish":        "<FTC-normalized finish or UNKNOWN>",
    "price_band":    "entry|mid-fashion|premium-fashion|UNKNOWN",
    "honesty_tag":   "[FACT]|[FACT-source]|[ESTIMATE]|[METHOD]|[SIGNAL]|[UNKNOWN]",
    "corroboration": <int, # of unrelated sources agreeing on this bucket>,
    "confidence":    "low|med|high",               // driven by reliability+recency+corroboration
    "method_note":   "<for [ESTIMATE]: the method+assumption; for [UNKNOWN]: the block reason>"
  }

HANDOFF: deliver the table to jewel_market.kb3 (trend_and_competitor_mapping) as raw tagged
trend signal, which reads it under the 5-signal-family lag/bias ledger and requires ≥2-3
independent families to agree before a category read is actionable. jewel_market's output then
becomes the S1 "public-data market read" leg fed to jewel_assortment.kb1 (signal_synthesis_
ranking), where it is reliability-weighted against the S4 poll signal and the owner's store
history into the ranked candidate list. Carry every tag intact across the handoff. Assert
nothing jewel_market or jewel_assortment must derive under their own honesty contracts.
```

---

## 6. Cadence & feedback

**Re-run cadence — event-triggered (not a fixed clock; re-run when a trigger fires):**
- **Pre-launch baseline** `[METHOD]`: full sweep before launch to establish the initial bucket ranking and set the two-date anchor for all velocity/movement signals (velocity needs a prior read).
- **Post-poll cross-check** `[METHOD]`: re-run right after the `jewel_poll` (S4) wave to cross-check *stated* poll preference against *revealed* web proxies. Agreement raises confidence; a poll-vs-web split is a flag to `jewel_assortment`, not something to average away.
- **Seasonal shift** `[METHOD]`: re-run at the jewelry gift cycle inflections (Q4 holiday, Valentine's, Mother's Day, spring/summer) since search-interest and social families are strongly seasonal; recency-weight against the seasonal baseline.
- **Competitor launch / trend break** `[METHOD]`: re-run when a Movers & Shakers spike, a breakout Trends query, or trade-press signals a new entrant or motif — to catch emerging demand while it's still a lead signal.
- **Do not re-run faster than the signal changes or than downstream can act** — matches the `jewel_assortment` REFRESH_CADENCE rule (don't collect faster than you can measure or fund). Each re-run reuses the held query plan (§4 Step 1) so results are comparable across runs (that comparability *is* the velocity/movement signal).

**Feedback loop (through ORCHESTRATION.md):** findings loop as follows —
`S1-RT evidence table → jewel_market.kb3 (interpret as trend signal, lag/bias ledger, durable-vs-fad) → jewel_assortment.kb1 (reliability-weighted synthesis with S4 poll + store history → ranked candidate list) → 6-SKU launch → post-launch sell-through / return / margin (jewel_assortment.kb3)`. Post-launch **revealed sell-through** is the higher standard against which this lane's earlier proxies are scored: buckets this lane called "strong" that then sold-through **validate** the proxy weighting; buckets it missed or over-called **re-weight** the source-reliability priors for the next run (`FEEDBACK_TO_DEMAND`). The orchestration feedback loop treats S1-RT as one continuously-refreshable input whose *own* reliability is audited against realized sales over cycles — the lane gets more trustworthy (or is down-weighted) as its past calls are checked against what actually sold. Every loop hop carries the honesty tags intact; no proxy is ever promoted to a measured sales fact.

---

_End S1-RT method spec. This lane collects and tags; it never fabricates. Rank + review-count = `[FACT-source]`; inferred units = `[ESTIMATE]` with a method; social buzz = `[SIGNAL]`; anything unfetchable = `[UNKNOWN]`._
