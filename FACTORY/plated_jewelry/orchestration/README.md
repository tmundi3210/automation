# Plated-Jewelry Market-Analysis Pack — README / manifest

_A self-contained bundle of the 6 gated **market/business specialists** for a rhodium/gold-**plated** fashion-jewelry DTC venture (US, young women ~18–25, launching 6 SKUs, website already live), plus everything needed to **orchestrate** them, **collect real buyer data** (poll and/or public web signals), and run the whole thing as a **feedback loop**. Nothing here invents market numbers — every load-bearing claim is tagged `[FACT]/[FACT-source]/[ESTIMATE]/[METHOD]/[UNKNOWN]/[SIGNAL]`._

> **No new specialist was created for this pack.** It packages the 6 existing gated market specialists and the existing FACTORY feedback-loop doctrine (`SELF_LOOP.md` + the `sysloops` loop specialist) into an orchestration + usage layer. See `reference/`.

---

## 1. What's in the box

```
plated_jewelry_market_pack/
├── README.md                      ← you are here (manifest + quick start)
├── ORCHESTRATION.md               ← how to run the 6 specialists together + the market-analysis FEEDBACK LOOP
├── SPECIALIST_INDEX.md            ← one card per specialist + the single-agent invocation recipe
├── REALTIME_DATA_COLLECTION.md    ← OPTION: collect real-time public data (shopping ranks, frequently-bought, social/comments, news) → most-sold signals
├── POLL_KIT.md                    ← OPTION: the ready-to-run IG/TikTok poll to collect real buyer preference
│
├── specialists/                   ← the 6 gated specialists, each = 1 *.specialist.json + 3 *.spec.json + 3 *.kb.json
│   ├── market/        jewel_market
│   ├── materials/     jewel_materials
│   ├── form/          jewel_form
│   ├── poll/          jewel_poll
│   ├── assortment/    jewel_assortment   (convergence node — ranks → the 6 SKUs)
│   └── economics/     jewel_economics    (gate — every candidate must clear it)
│
├── report/                        ← the pre-launch market-analysis report already produced from these specialists
│   ├── MARKET_ANALYSIS_REPORT.md
│   └── report_sections/           (market, materials, form, poll, assortment, economics)
│
└── reference/                     ← the plan + the loop doctrine this pack reuses (NOT new specialists)
    ├── BUILD_PLAN_MARKET.md       (roster + per-KB decomposition + honesty contract)
    ├── OWNER_BRIEF_2_MARKET.md    (the owner's brief, tagged)
    ├── SELF_LOOP.md               (the parameterizable self-improvement loop)
    └── sysloops.specialist.json   (the existing feedback-loop engineer — LOOP STRUCTURE reused, content not)
```

---

## 2. The one rule

**One fresh agent per specialist.** Each agent is given the full text of that specialist's `*.specialist.json` as its operating spec, told to read that specialist's 3 grounded `*.kb.json` files, and answers the owner's question **only** from that spec + those KBs — tagging every claim, letting gaps stay gaps, deferring on its own escalation triggers instead of inventing. No specialist reasons outside its grounding. The exact copy-paste recipe is in `SPECIALIST_INDEX.md §1`.

## 3. Quick start (pick a path)

- **Just read the answer** → open `report/MARKET_ANALYSIS_REPORT.md`. It already ran these specialists once.
- **Get real buyer data before locking the 6 SKUs** → run **either or both**:
  - `POLL_KIT.md` — launch the IG/TikTok poll (audience preference + price acceptance).
  - `REALTIME_DATA_COLLECTION.md` — run the public-web signal lane (which products are most-sold / frequently-bought, from shopping ranks, reviews, social talk, news).
- **Run the full pipeline yourself** → follow `ORCHESTRATION.md`: foundations in parallel (market, materials, form, economics) → poll/real-time data for the fresh signal → `jewel_assortment` synthesizes and ranks → `jewel_economics` gates → the 6 SKUs.
- **Keep it honest and current** → run the **market-analysis feedback loop** in `ORCHESTRATION.md §4`: adversarial questioner → specialist answerer → judge → synthesizer, bounded rounds, then real outcomes (poll, store sales, live-ad CAC, fresh public signals) re-enter `jewel_assortment.kb3_test_iterate_protocol` and the 6 SKUs are re-locked.

## 4. Honesty contract (binds the whole pack)

Every load-bearing claim carries a tag: `[FACT]` / `[FACT-source]` (cited public source) / `[ESTIMATE]` (method + basis + confidence) / `[METHOD]` (a way to obtain the number) / `[UNKNOWN]` / `[SIGNAL]` (an engagement/preference proxy, not a sale). **No market size, sales number, percentage, demographic/ethnicity rate, brand claim, competitor metric, or price is ever stated as measured fact when it isn't.** The real numbers come from the poll and the public-data pulls these tools direct — never guessed. This is the same rule the whole vertical was built and gated under.
