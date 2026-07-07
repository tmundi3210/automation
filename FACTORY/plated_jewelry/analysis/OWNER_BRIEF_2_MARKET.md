# OWNER_BRIEF_2_MARKET.md — the owner's market/business brief, saved as-is (2026-07-07)

_The first owner brief (`OWNER_BRIEF.md`, 2026-07-02) was supply-side: plate steel wire, build a
DIY sputtering/electroplating machine, process chemistry, cost-of-making, houseware extension. This
second brief pivots to the **demand / business / product-selection front-end** of the same venture.
Saved verbatim first (speech-to-text stream-of-thought, artifacts intact), then unpacked with
confidence tags. Nothing in §2 overwrites §1. Prior product decisions still bind: **Custom / headless
storefront (7 engineering specialists) + Rhodiumgold-specific grounding** (owner's answers, 2026-07-07)._

## 1. The owner's message (verbatim, unedited)

> So now make specialist through academic fields, subfields, topics, subtopics, whatever suits. Like
> go highly specialized terms. Do not go into generalized, which you know. Just collect the most
> important information and make the specialist as directed by the pipeline or how you made the
> previous one. I want, so my project is, I want to make a business model and starting from business
> model then however it starts like step by step. So I want to see what's the demand for jewelries in
> USA, which type, which ethnicity, how many people buy, what's the, what brands do they buy, what
> shapes are common, is it whole gold or what's the percentage of gold-plated one? Rhodium plating I
> think so, which is used to do silver, which makes silver shiny and keep it shiny, doesn't, so prevent
> the, prevent it from unshine, something. And what's the price, where do they, which factories does
> the gold plating? That's not important which factory does it, so it's just an overview like how much
> it costs them to make them, like what's the base material for the jewelries. How to decrease the The
> price without decreasing the quality and how to make it high-quality stuff. So, the main thing I want
> to do right now is want to look at the most sold items, which I should start with. Like, I have, I
> will start with only six items. I have website running. So, now, for that, collect all data which is
> available, mindful of the old stuff. And I also plan to do a poll, Instagram poll, TikTok poll through
> ad to see which type of jewelry, like girls like. I think so it's girls, mostly girls buy it, like from
> young girls, from 18 to 25 or whatever. So I will create specific poll for them, like simple one, like,
> do you like earrings or whatever the category is. So I need to create poll. And then maybe in the poll
> I need to tell them that get free or something to attract them to select one. Then, so I want to do
> market analysis, whatever is required before starting the business. So create all the specialists which
> require in this field, and which shape of that jewelry I need. So I need specialist for that too. So now
> run my idea, however it is, yield best results. I think so, use the dense KBs for neutralizing the idea
> and doing all the stuff. I don't know, just, you decide.

## 2. Clause-by-clause unpacking (interpretation layer — tagged)

### 2a. METHOD instructions (high confidence — restate the standing FACTORY method)
1. **Make specialists through academic fields → subfields → topics → subtopics → highly specialized
   terms; do NOT stay generalized.** Collect only the most important information. Build them "as directed
   by the pipeline / how you made the previous one" (= the FACTORY spec→forge→dense-gate pipeline; the
   `sweater_vertical` / `movie_studio` precedent). [FACT — explicit]
2. **Use the dense KBs to "neutralize the idea" and do the work** — i.e. run the business idea *through*
   knowledge-injected specialists grounded in dense, gated KBs, as in the standing method. [FACT — explicit]
3. **"Run my idea, however it is, yield best results… you decide."** — owner delegates the specific
   decomposition and execution to the factory. [FACT — explicit delegation]

### 2b. The BUSINESS ASK (what the specialists must let an agent answer)
- **Business model first, then step by step.** Start from the business model and build outward. [FACT]
- **US jewelry market demand:** which *types* sell, *which ethnicity / cultural market* buys, *how many*
  buy (market sizing), *which brands* they buy, *what shapes* are common, and the **solid-gold vs
  gold-plated share** (%). [FACT — explicit question bank]
- **Rhodium plating** — owner's (roughly correct) understanding: rhodium is plated over **silver** to
  make it bright/white and **keep it from tarnishing** ("prevent it from unshine"). [FACT — the intent;
  the materials specialist carries the precise, correct science: rhodium is a hard, bright, tarnish-
  resistant platinum-group metal commonly plated over sterling silver and white gold.]
- **Price & cost overview:** typical retail *price*; an *overview of what it costs to make* (not which
  specific factory — "that's not important"); the **base material** of the jewelry. [FACT]
- **Cost-down without quality-down** + **how to make high-quality**. [FACT — explicit; = value engineering]
- **"The main thing right now": pick the most-sold items to START with — only SIX items.** Owner already
  has a **website running**; use available data, "mindful of the old stuff." [FACT — top priority]
- **Poll plan:** run an **Instagram / TikTok poll via paid ad** to learn which jewelry *type* the audience
  likes; audience is **mostly young women ~18–25**; keep it **simple** ("do you like earrings or [category]");
  possibly offer a **free item / incentive** to lift response. Owner wants to **create the poll**. [FACT]
- **Market analysis — "whatever is required before starting the business."** [FACT]
- **"Which shape of jewelry I need — I need a specialist for that too."** [FACT — explicit specialist ask]

### 2c. HONESTY BOUNDARY (binds every specialist and KB built from this brief)
The owner asks for numbers a factory **must not invent**: how many people buy, what % is gold-plated,
which ethnicity buys, which brands, what price. **No market statistic, demographic percentage, brand
claim, or price is fabricated.** The specialists encode **methods, terminology, classification, and
decision procedures** — *how to measure* each of these — plus **genuine, well-established domain facts**
(materials science; US FTC Jewelry Guides, 16 CFR Part 23, defining "gold-plated", "vermeil", "gold-filled",
"rhodium", "sterling"; consumer-research methodology; pricing theory). Every claim is tagged
`[FACT]` (with basis), `[ESTIMATE]` (with method + confidence), `[METHOD]` (a way to obtain the number),
or `[UNKNOWN]`. The actual market numbers are produced later — by the **polls** and **public-data pulls**
the specialists direct — never guessed here.

### 2d. Deliverables ladder (this brief → P4-market series)
- **P4a (this + BUILD_PLAN_MARKET.md):** brief saved as-is; the 6-specialist market roster and its
  per-KB decomposition authored. → commit.
- **P4b:** forge the 6 specialists (18 dense, gated KBs + 6 specialist specs) via the FACTORY pipeline. → commit.
- **P4c:** verify ALL GREEN (every KB `kb_validator --mode dense`; every specialist `specialist_validator`). → commit.
- **P4d:** report + map each specialist to the owner's question bank; then (owner-gated) run the idea
  *through* the specialists → the poll instrument, the demand read, the 6-SKU launch set, the cost/price model.

_Honesty rules bind throughout. No invented supplier names, prices, market shares, demographic figures,
or brand claims anywhere. Real materials-science and regulatory facts (FTC 16 CFR 23; EN1811 nickel
release; sterling = 92.5% Ag) are carried as [FACT] with basis._
