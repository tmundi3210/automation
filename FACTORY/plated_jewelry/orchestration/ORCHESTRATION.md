# ORCHESTRATION.md — plated_jewelry MARKET pipeline (6 specialists + feedback loop)

Master playbook for running the six `plated_jewelry` MARKET specialists as one orchestrated
market-analysis pipeline **with a feedback loop**. Lifts the four-role self-improvement loop from
`FACTORY/SELF_LOOP.md` and the loop-engineering invariants from
`study_system/specialists/sysloops/sysloops.specialist.json` (the "loop one"), and binds them to the
demand/business roster in `plated_jewelry/analysis/BUILD_PLAN_MARKET.md`. It does **not** copy
sysloops' study-system content — it reuses its loop *structure* (idempotent append-only,
snapshot+threshold+named-consumer, honesty-label-beats-convenient-number, degraded-mode fallback,
adopt/adapt/reject ledger, own-metric-fault-surfacing trigger).

The six specialists live under `plated_jewelry/specialists/<dir>/<specialist_id>.specialist.json`,
each grounded in exactly 3 dense KBs. Every KB filename below is real.

---

## 1. Purpose & the one rule

**Purpose.** Turn the owner's venture question — *which plated-jewelry SKUs to launch, made of what,
in which forms, at what price, to whom, on what evidence* — into a ranked, tagged, gated 6-SKU
decision-of-record, produced by six domain specialists run as a dependency pipeline and then
hardened by an adversarial feedback loop.

**The one rule (non-negotiable).**

> **One fresh agent per specialist. That agent answers ONLY from that specialist's `.specialist.json`
> spec plus its 3 grounded KBs. Gaps stay gaps.**

- No specialist reasons outside its grounding. `jewel_materials` does not opine on demand; `jewel_market`
  does not price a SKU; `jewel_poll` does not rank the assortment. Cross-domain questions are **escalated
  along the DAG**, never answered off-paper (each spec already carries a "crosses into another
  specialist's lane → defer" escalation trigger — honor it).
- Fresh context per role is what makes the audit ≠ the model agreeing with itself. A role that has
  already seen another role's output is contaminated and must not be reused.
- A gap is a first-class output. If the grounding cannot answer, the specialist returns `[UNKNOWN]` or a
  `[METHOD]` to obtain the answer — **never** a fabricated figure to fill the hole.

---

## 2. The dependency DAG (data flow)

Four **foundation** specialists run first, fully in parallel — none consumes another's output. They
answer, respectively: *what sells* (demand), *what is makeable at quality/cost* (materials), *what is
producible/returnable* (form), and *the cost/price/viability envelope* (economics). `jewel_poll` then
designs the instrument to collect the **missing preference signal** the foundations could only name as a
`[METHOD]`/`[UNKNOWN]`. `jewel_assortment` is the **convergence node**: it fuses every upstream output
plus the fresh buyer signal into the ranked 6-SKU set. `jewel_economics` is the **gate** every candidate
must clear before the set is trusted.

```
                        FOUNDATIONS  (run first, in parallel — no inter-dependency)
  ┌───────────────────────────┬───────────────────────────┬───────────────────────────┬───────────────────────────┐
  │  jewel_market             │  jewel_materials          │  jewel_form               │  jewel_economics          │
  │  (US demand read)         │  (makeable @ quality/cost)│  (producible / returnable)│  (cost / price / viability │
  │  kb1_category_and_finish_ │  kb1_substrates_base_     │  kb1_category_construction│   ENVELOPE)               │
  │     demand                │     metals                │     _taxonomy             │  kb1_cogs_and_landed      │
  │  kb2_consumer_            │  kb2_finishes_rhodium_gold│  kb2_shape_motif_style_   │  kb2_pricing_and_margin   │
  │     segmentation_method   │  kb3_quality_cost_value_  │     systems               │  kb3_viability_cac_ltv    │
  │  kb3_trend_and_competitor_│     engineering           │  kb3_sizing_fit_returns   │                           │
  │     mapping               │                           │                           │                           │
  └────────────┬──────────────┴─────────────┬─────────────┴─────────────┬─────────────┴────────────┬──────────────┘
               │ demand read +              │ substrate/finish/QC       │ form taxonomy +          │ cost+price+CAC
               │ segmentation METHOD +      │ envelope + value-eng      │ producibility +          │ envelope + margin
               │ trend/competitor map       │ cost-down levers          │ sizing/returns risk      │ floor + WTP ceiling
               │                            │                           │                          │
               │              names the MISSING preference signal (a [METHOD]/[UNKNOWN], not a guess)
               ▼                            ▼                           ▼                          │
        ┌──────────────────────────────────────────────────────────────────────┐                  │
        │  jewel_poll  (designs the instrument for the missing buyer signal)     │                  │
        │  kb1_instrument_design · kb2_incentive_sampling_bias ·                 │                  │
        │  kb3_paid_reach_and_signal                                             │                  │
        │  → OUTPUT: a runnable poll instrument + a confidence-tagged            │                  │
        │    conversion of responses into a rankable [SIGNAL] (owner collects)   │                  │
        └───────────────────────────────┬──────────────────────────────────────┘                  │
                                         │  fresh preference [SIGNAL]                               │
                                         │  (poll results AND/OR real-time public data)             │
                                         ▼                                                          │
        ┌──────────────────────────────────────────────────────────────────────┐                  │
        │  jewel_assortment  ── CONVERGENCE NODE ──                              │                  │
        │  kb1_signal_synthesis_ranking  (fuse market read + poll + own store    │                  │
        │     history, reliability-weighted, ABC/Pareto, uncertainty-tagged)     │                  │
        │  kb2_launch_set_of_six  (category coverage vs depth, good-better-best  │                  │
        │     ladder, finish mix, cohesion; prune by MOQ/plating-run/capital)    │                  │
        │  kb3_test_iterate_protocol  (launch→measure→keep/cut/expand; OUTER loop)│                 │
        │  → OUTPUT: ranked candidates → the proposed 6-SKU set                  │                  │
        └───────────────────────────────┬──────────────────────────────────────┘                  │
                                         │  each of the 6 candidates                                │
                                         ▼                                                          │
        ┌──────────────────────────────────────────────────────────────────────┐  ◄───────────────┘
        │  jewel_economics  ── THE GATE ──  (re-invoked per candidate)           │
        │  landed COGS → contribution-margin floor → WTP ceiling → CAC/LTV/      │
        │  payback go/no-go.  A SKU that fails the gate is DROPPED or RE-PRICED; │
        │  the 6-SKU set is trusted ONLY after economics re-gates it.            │
        └───────────────────────────────┬──────────────────────────────────────┘
                                         │  gated 6-SKU decision-of-record
                                         ▼
                          DECISION_OF_RECORD.md  (the locked six)
```

Edges are **data hand-offs**, not shared context: the assortment agent reads the foundations' *written
outputs*, it does not inherit their windows. The economics gate is drawn twice on purpose — once as a
foundation (it sets the envelope before anything is ranked) and once as the terminal gate (it re-clears
each concrete candidate). Both are the same specialist, freshly invoked.

---

## 3. Run modes (options)

Three modes. Modes **B** and **C** are the two ways to obtain the fresh buyer signal the foundations
can only name; they may run **together** (poll signal + live public signal fused in
`kb1_signal_synthesis_ranking`).

### Mode A — One-pass analysis
Run each of the six specialists **once** (foundations in parallel → `jewel_poll` → `jewel_assortment`
synthesizes → `jewel_economics` gates) → emit a report + the proposed 6-SKU set. Fast, honest, and
signal-thin: the assortment leans on the public-data market read and the owner's existing store history,
with preference signal marked `[UNKNOWN]` where no poll/live data was collected. Use to produce the first
decision-of-record and to expose exactly which claims are riding on an un-measured assumption.

### Mode B — Poll-first
Run `jewel_poll` to produce the **instrument** (question type per decision — single-choice vs
paired-comparison vs MaxDiff/conjoint/WTP — inside native IG/TikTok affordances, incentive kept inside
FTC/sweepstakes/platform law, paid-reach plan at a computed cost-per-response). The **owner collects
responses** out-of-band. `jewel_poll` then converts engagement into a confidence-tagged rankable
`[SIGNAL]`, which feeds `jewel_assortment` `kb1_signal_synthesis_ranking`. Poll percentages are
preference signal **only** — never read as market size, purchase rate, or demographic rate (a standing
`jewel_poll` escalation).

### Mode C — Real-time-data lane
Run the public-data collection described in the companion `REALTIME_DATA_COLLECTION.md` (marketplace
best-seller ranks, search-volume trends, public-storefront competitor/price-tier scans, social newness).
Feed the **tagged evidence** into `jewel_market` `kb3_trend_and_competitor_mapping` (as trend/competitor
input) and into `jewel_assortment` `kb1_signal_synthesis_ranking` (as a demand `[SIGNAL]` of stated
reliability/lag/bias). Every collected datum carries its source tag; a single-signal-family read (e.g.
one viral spike, no marketplace/search confirmation) is flagged, not acted on.

> **B + C together** is the recommended posture for a real launch: the poll gives *stated preference*,
> the live lane gives *revealed/behavioral* signal, and `jewel_assortment` fuses them reliability-weighted
> so neither a single boosted poll nor a single trending SKU dominates.

---

## 4. THE MARKET-ANALYSIS FEEDBACK LOOP (core deliverable)

Two nested loops. The **inner loop** audits and hardens each specialist's *read* before it is trusted
(adapted from the sysloops/SELF_LOOP four-role loop). The **outer loop** re-enters real-world outcomes so
the locked six can be re-ranked and RE-LOCKED.

### 4a. Inner loop — four fresh roles, per specialist, per round

Each role is a **fresh context window** reading only its named inputs. Bound to **2 rounds**.

**R1 — Questioner (adversary; does not answer).**
Reads the target specialist's spec + its 3 KBs (and, for `jewel_assortment`/`jewel_economics`, the
upstream written outputs it consumes). Aims every question at the **6–10 highest `risk_if_wrong`
claims** the specialist carries. For this vertical the load-bearing joints are:
1. **category-to-buy** — which worn-location category the launch leads with (`jewel_market` kb1 ×
   `jewel_form` kb1 × `jewel_assortment` kb2).
2. **finish preference** — gold-plate vs vermeil vs rhodium-over-silver as a *buyer* preference vs a
   *makeability/cost* fact (`jewel_market` kb1 × `jewel_materials` kb2 × the poll `[SIGNAL]`).
3. **price acceptance** — the WTP ceiling vs the contribution-margin floor (`jewel_economics` kb2 ×
   `jewel_poll` WTP instrument).
4. **CAC assumption** — the paid-social CAC feeding go/no-go (`jewel_economics` kb3).
5. **finish/label legality** — any vermeil/gold-filled/gold-plated/rhodium label vs the FTC 16 CFR
   Part 23 micron/karat threshold (`jewel_materials` kb2, `jewel_economics` kb1).
6. **signal reliability** — the weight `jewel_assortment` puts on the owner's thin/stale/survivorship
   store history vs fresh poll/live signal (`jewel_assortment` kb1).
7. **sizing→returns** — return risk from sizing-driven categories (`jewel_form` kb3).
8. **segmentation as method-not-rate** — any cultural/ethnic/age cut that risks becoming a per-group
   purchase rate (`jewel_market` kb2).
R1 asks *where is the number sourced, where is the gap, where is the tag missing, where does one spec
contradict another* — it never proposes answers.

**R2 — Answerer (IS the specialist).**
A fresh window that **is** the target specialist — answers each R1 question **only** from that spec + its
3 KBs, and **tags every load-bearing claim** with the honesty scheme (§5 HONESTY DISCIPLINE). A claim it
cannot ground is returned as `[UNKNOWN]` or a `[METHOD]` to measure it. It does not defend; it answers or
declares the gap.

**R3 — Judge (fresh; classifies + disposes).**
Reads R1 + R2 + the cited spec/KB. For each answer emits a verdict from:
`gap | contradiction | stale | overclaim | untagged | fabricated-number | scope | ok`, **citing the exact
spec/KB location** (`<specialist_id>` / `<kbN_filename>` / node id or CQ id). Sets a **disposition**:
- `apply_now` — high-confidence, in-scope, mechanical, and the artifact stays green under the re-gate.
- `propose` — anything needing judgement, new grounding, or a fact not yet in hand (backlog).
- `watch` — real but not yet actionable; re-check next round / next outer cycle.
Sets `dry = true` when the round surfaced no new high-severity finding. A `fabricated-number` verdict is
**always** downgraded to `propose` — a fix that would need an invented figure is never applied (§5).

**Bounded rounds / dry-stop / NOT-CONVERGED.** If R3 sets `dry`, stop. Else run **one** more round
(max 2). A specialist still producing high-severity findings at the cap is flagged **NOT-CONVERGED** in
its `LOOP_REPORT` — the cap is a cost bound, never a convergence claim. Never silently truncate; declare
dropped coverage.

**R4 — Synthesizer (fresh).**
Rolls the surviving verdicts into (a) an updated *read* for the specialist and (b) a `feedback.json`
(the apply_now/propose/watch ledger). The **finalizer applies only `apply_now` items**; then the
**orchestrator independently re-runs the gate** — `bash FACTORY/build.sh
FACTORY/plated_jewelry/specialists/<dir>` → "ALL GREEN" — before any commit. Agent self-reports are
never acceptance.

### 4b. Outer loop — real-world outcomes re-lock the six

Real-world outcomes re-enter `jewel_assortment` `kb3_test_iterate_protocol` → re-rank → the 6 SKUs are
**RE-LOCKED** (with a fresh `jewel_economics` re-gate). The outcome streams:
- **poll returns** (Mode B responses landing after the first pass),
- **the live store's bestseller / sell-through / return-rate data** ("mindful of the old stuff"),
- **real-time public signals** (Mode C lane: trend/competitor drift),
- **a live ad's realized CAC** (against the `jewel_economics` kb3 assumption).

**Revisit cadence + event triggers** (re-run the outer loop when any fires):
- **New poll data** arrives past the minimum-signal *n* → re-fuse in `kb1_signal_synthesis_ranking`.
- **A finish or price signal flips** (e.g. rhodium-over-silver preference overtakes gold-plate; the WTP
  ceiling moves) → re-rank affected SKUs, re-gate on margin.
- **A competitor / trend shift** (new entrant, a competitor flagship/price change, a fad decays) →
  refresh `jewel_market` kb3 + the Mode C evidence.
- **CAC breaches the economics gate** (realized CAC crosses the `kb3_viability_cac_ltv` payback
  threshold) → immediate re-gate; a SKU whose contribution can no longer fund its CAC is dropped or
  re-priced, and the six are re-locked.
- **Own-metric-fault-surfacing** (lifted from sysloops): if one of the loop's own tracked market
  metrics (return rate, sell-through, realized-vs-assumed CAC, realized-vs-assumed AOV) chronically
  misses, the trigger fires a re-run *and* a review of the assortment/economics thresholds themselves —
  the loop that improves the loop.

Absent any trigger, revisit on the `kb3_test_iterate_protocol` measurement-window cadence — never draw a
keep/cut/expand verdict *inside* a SKU's window or on an under-powered sample.

---

## 5. Invariants carried from the factory

1. **Fresh context per role.** Every R1/R2/R3/R4 and every specialist invocation is a new window reading
   only its named inputs. Reuse = contamination.
2. **Honesty label beats convenient number.** A labeled `[UNKNOWN]`/`[METHOD]` always beats a fabricated
   figure. Never invent a market size, unit count, solid-vs-plated %, demographic/ethnicity/gender rate,
   brand/competitor metric, or price as if measured (full contract below).
3. **A fix that needs a fabricated fact is downgraded to `propose`, never applied.** R3's
   `fabricated-number` verdict can never become `apply_now`; it becomes a named gap on the backlog.
4. **Assortment output is trusted only after economics re-gates it.** `jewel_assortment`'s ranked six is
   a *proposal* until `jewel_economics` clears every candidate on margin floor + WTP ceiling + CAC/LTV
   payload. No un-gated set is written to the decision-of-record.
5. **No silent truncation.** A specialist still hot at the round cap → **NOT-CONVERGED**. Dropped
   coverage, un-collected signal, and un-run lanes are declared explicitly, not omitted.

Supporting invariants inherited from the sysloops "loop one": all loop writes are **idempotent,
append-only, dated snapshots joined by id** (never in-place mutation of a prior read); every kept metric
carries a **snapshot + threshold + named consumer** or it is deleted as noise; a valid **degraded-mode**
plan beats a rich broken one; competitor/market evidence copied for **evidenced function, not brand
popularity**, and recorded in an **adopt/adapt/reject ledger** where no verdict rests on an unlabeled
fact.

---

## 6. What each round WRITES (append-only, idempotent, dated)

All under `plated_jewelry/analysis/self_loop/<slug>/` (default `slug` = the specialist file basename:
`jewel_market`, `jewel_materials`, `jewel_form`, `jewel_poll`, `jewel_assortment`, `jewel_economics`).
Every write is an **append-only dated snapshot joined by id** — re-running a round reproduces the same
state and never double-counts. Nothing is mutated in place.

Per specialist, per round:
- **`roundN_questions.md`** — R1's adversarial question set, each tagged to the load-bearing joint it
  targets.
- **`roundN_answers.md`** — R2's answers, every load-bearing claim carrying its `[FACT]/[FACT-source]/
  [ESTIMATE]/[METHOD]/[UNKNOWN]/[SIGNAL]` tag.
- **`roundN_judgment.json`** — R3's verdicts: `{claim_id, verdict, cited_location, disposition, dry}`
  per claim.
- **`LOOP_REPORT.md`** — R4's per-specialist rollup: what was applied, what was proposed, what is watched,
  and the **NOT-CONVERGED** flag if still hot at the cap.
- **`feedback.json`** — the machine ledger: `apply_now[] / propose[] / watch[]`, each item id'd, dated,
  and (for competitor/mechanism items) carrying an adopt/adapt/reject verdict with its evidence tag.

Vertical rollups under `plated_jewelry/analysis/self_loop/`:
- **`SELF_LOOP_SUMMARY.md`**, **`APPLIED.md`**, **`PROPOSED_BACKLOG.md`** — the three cross-specialist
  rollups (mirrors the SELF_LOOP slot binding).

The running product, at `plated_jewelry/orchestration/DECISION_OF_RECORD.md`:
- **The running 6-SKU decision-of-record** — the current locked six, each SKU stamped with: its fused
  signal + reliability weight, its finish (with FTC label check), its landed COGS → price → contribution
  margin, its CAC/LTV/payback verdict, and the date + trigger of the last re-lock. Superseded versions
  are appended, never overwritten — the decision has a dated history.

**Snapshot + threshold + named-consumer rule (from sysloops).** Every metric the loop tracks — return
rate, sell-through, realized CAC, realized AOV, poll-signal *n*, margin — must carry a **stored dated
snapshot**, a **threshold**, and a **named consumer** (the specific specialist/KB or trigger that acts on
it: e.g. realized-CAC → `jewel_economics` kb3 gate; return-rate → `jewel_form` kb3 + `jewel_assortment`
kb3). A metric lacking any of the three is deleted rather than kept as noise.

---

## HONESTY DISCIPLINE (binding, non-negotiable)

Every load-bearing claim produced anywhere in this pipeline carries exactly one tag:
`[FACT]` (true by definition / regulatory) · `[FACT-source]` (cited public source) · `[ESTIMATE]`
(method + basis + confidence, carrying the literal word *estimate*) · `[METHOD]` (the procedure to obtain
a number not in hand) · `[UNKNOWN]` (not verifiable this pass) · `[SIGNAL]` (a poll/live-data preference
reading of stated reliability, never a population rate).

**NEVER** invent a market size, sales/unit number, percentage, demographic/ethnicity/gender purchase
rate, brand claim, competitor metric, or price as if measured. A number you cannot source is a `[METHOD]`
to obtain it or an `[UNKNOWN]` — **never** a fabricated figure. Real, verifiable domain facts are
encouraged and tagged `[FACT]`/`[FACT-source]` with basis: FTC 16 CFR Part 23 finish definitions (vermeil
= ≥10k gold, ≥2.5 µm over sterling; sterling = 92.5% Ag), rhodium as a tarnish-resistant platinum-group
metal, EN1811 nickel-release limit, MaxDiff/conjoint/self-selection-bias method facts, keystone-markup
and contribution-margin definitions. This mirrors the standing FACTORY rule the whole vertical was built
under (`BUILD_PLAN_MARKET.md` honesty contract) — the loop enforces it, it does not relax it.
