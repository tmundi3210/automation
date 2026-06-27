# B60 — the idea, built forward (constructive panel spec)

The same 10-specialist panel re-run in **constructive build-forward mode**: each specialist was
asked to *help make the idea real* and contribute the concrete design for its piece. The result is
a single buildable spec — the idea fleshed out into a system you could actually start building,
with a v1 MVP cut and a phased roadmap. Raw per-specialist designs: `demo/build_forward_findings.md`.
(The critical read is `PANEL.md`; the seed walk-through is `RUN.md`. This doc is the *how-to-build*.)

## The system at a glance (controller spine, from `orch`)

```
ingest ─► salience ─► entity_link ─► signal{authenticity ∥ hype} ─► dense_summary ─► creative_brief ─► compliance_gate ─► emit ─►(outbox)─► downstream render/publish session
            │             │                    │                          │               │                  │
     attention map   2-3 linked        calibrated auth_p +          decision-lossless   gen_brief        path-to-yes
     + tier verdict   entity scene     genuine-hype + brief         machine brief       artifact         clearance + signed
                                                                                       (in-band contract)  in-band token
                                              ▲                                                                  │
                                              └──────────────── eval harness (OFFLINE loop) ◄────────────────────┘
                                                     frozen metric · leakage-controlled holdout · max_cycles
```

Thin control plane: typed blackboard (one write-owner per field), rule-based router, per-node
circuit breakers, token-budget accountant, trace emitter. **`compliance_gate` is the sole
compile-time predecessor of `emit`** (edge-permission matrix; static check every deploy). The
cross-session handoff is an **outbox**: the gate writes an immutable, schema-validated, sanitized
`EmitPacket` to a durable queue with an idempotency key; the downstream session *pulls* it (no
in-process call across the trust boundary). The improve-loop runs **offline only** — it never
mutates the live controller mid-request.

## Stage-by-stage build

**1. Data layer (`ir`).** Durable-first tiered ingestion: **Tier 0** official APIs (YouTube Data
v3, Reddit OAuth; X Basic/Pro budget-gated), **Tier 1** sanctioned feeds (RSS/Atom, Google News
RSS, sitemaps, channel feeds — zero-auth), **Tier 2** trends (Google Trends, YouTube trending,
r/popular, Wikipedia pageviews), **Tier 3** datasets (GDELT, CC-NEWS, HF). Instagram = owned/business
Graph API only (v1: profile metadata, not a comment firehose). Per-source rate-limit governor;
freshness tiers + TTL caching; cross-post dedup (URL-canonicalize → SimHash/MinHash → embedding ANN
→ canonical `event_id`); append-only raw store with `{source, fetch_ts, license}`.

**2. Salience + linking (`salience_heavy`).** `salience.score(node)` = weighted geometric mean of
four scaled public signals (Trends, follower counts, news/GDELT volume, Wikipedia pageviews), dual
raw/normalized track, every score provenance-stamped. `region→niche→top-5` expander over a closed
single-parent hierarchy, strictly-decreasing-salience frontier, `depth≤4 / breadth≤8 / top_k=5` +
long-tail cutoff (finite, deterministic rebuild). **Opportunity = under_served × reachable ×
addressable_value**, with the "medium-tier wins" claim *reported per niche, not assumed*. Links:
canonical entity resolution → typed-tag join (`PLACE:punjab ≠ TOPIC:punjab`) → relatedness only to
*rank* → boolean geo-relevance predicate to *license* → arity hard-enforced to {2,3} with triad closure.

**3. Reasoning (`reason`).** The linking step is **abductive generate-and-score**: sample N=12–20
candidate links (constrained JSON `{target, relation, rationale}`), score with a *separate*
PRM-style verifier on relevance / non-obviousness / evidential support, keep top 2-3. Authenticity/
hype = claim-extraction + NLI entailment against retrieved evidence (`authenticity` = fraction
grounded; `hype` = fraction unsupported-but-superlative). Go/no-go = a small fitted, temperature-
scaled logistic over link-score + authenticity + hype + coherence, with a tuned abstention
threshold. Confidence is a **measured calibration property (ECE<0.05)**, not a self-report.

**4. Signal + dense brief (`signal_heavy`).** Calibrated `P(inauthentic|public signals)` + interval +
provenance (never boolean) from five feature families (account, templated-text, timing, network,
engagement-mismatch), Platt→isotonic against a versioned labeled CIB benchmark. Genuine-hype =
provenance-dedup to distinct origins, common-method discount, `≥N`-independent-origins gate
*independent of volume*. The **dense brief** is one pipe/KV line with a `schema_hint` paid once and
`v:obs|v:derived|v:proxied` tags — decision-lossless over a fixed field set, validated by a
round-trip + coverage audit.

**5. Generation kit (`creative_heavy`).** A closed, versioned `gen_brief` artifact: required image
block + required story block (setup/turn/punchline) + optional audio block, plus an **in-band
contract block** (`disclosure_slot`, `usage_restrictions`, assume/guarantee, `derivative_risk_flag`)
and a provenance block (every field cites a `source_fact_id`; ungrounded subjects rejected). The
2-3 entity beat: one `contrast_axis`, one benign-violation joke, rule-of-three, meme format only
where it cuts comprehension cost. Non-empty `disclosure_slot` is a hard precondition (serialization
fails closed). This stage **briefs only — never renders or publishes**.

**6. Path-to-yes clearance (`compliance_heavy`, enabler).** A per-figure **Clearance Record** on
the frozen `(item, figure, use)` tuple → disposition `allow / constrain / block / human-review`.
**Green lanes that maximize what ships:** own-persona / original composite (fastest), scope-matched
consented use (+ voice-clone authorization), parody-with-visible-cues (the default when consent is
absent, instead of depicting fact). A **disclosure binder** signs a C2PA manifest and emits an
in-band token bound to a named downstream slot; a handoff verifier re-checks token survival.
Hard blocks stay hard (minors/protected persons, fabricated wrongdoing, unlicensed-source
republication); no identity use is ever auto-approved. *(Screening heuristic, not legal advice.)*

**7. Ethical resonance (`psych`, enabler).** A **4-driver scene generator** (identity-fit,
belonging, emotion, surprise) requiring self-relevance + one positive high-arousal emotion before
publish; lead with shared in-group experience, not out-group contrast. A **safe-lanes whitelist**
(own persona / consented / signposted parody / shared-experience) gates eligible subjects *before*
framing. An **honest sentiment read** weighted by reach + saves/shares (not comment volume),
correcting vocal-minority bias.

**8. Proof harness (`eval_heavy`).** A sealed pre-registration: one construct ("diaspora-anxiety
resonance"), one OEC (blind dual-rater resonance minus a guardrail penalty), two baselines (naive +
human-curated), threshold = beat the human baseline by δ. The improve-loop **terminates**
(threshold OR non-renewable `max_cycles`, with a termination certificate). The holdout is
**strictly post-cutoff + blind**, scored against a frozen realized-outcome ledger, with both leakage
channels controlled, survivorship + base-rate correction, and CIs. The medium-tier claim is a
**falsifiable bet**: it counts only if its deflated, clustered, out-of-sample CI excludes zero.

## The growth objective the whole thing optimizes (`marketing`)

The system is not optimizing views — it optimizes **net new owned subscribers/week**, primary KPI
**follower→owned-subscriber conversion**. Funnel: discovery (a 2-3-link derivative) → profile →
**owned capture** (newsletter / WhatsApp / Telegram, single consented opt-in) → activation →
retention → referral. Beachhead = Punjab/India + diaspora, Punjabi/Hinglish; right-to-win = *speed +
cultural fluency in an under-served niche* (no moat from the format itself, so the moat is the owned
list + cadence). "Medium-tier is the opportunity" becomes a **measurable rule**:
`attention-opportunity = topical demand ÷ existing saturation`, ship only above a threshold, track
yield-per-subject. This is the objective `eval` freezes its metric against and `salience` ranks toward.

## The v1 MVP (smallest honest, end-to-end slice)

A single-region (Punjab + diaspora), single-platform, single-owned-list pipeline:

1. **Data:** Tier-0 YouTube+Reddit + Tier-1 RSS + Tier-2 Trends/Wikipedia, URL/SimHash dedup. *(No paid/grey dependency.)*
2. **Salience:** 4-signal score, one diaspora bridge set, top-5 expander, opportunity score, predicate-gated **arity-2** links.
3. **Reasoning:** zero-shot CoT proposer + zero-shot LLM verifier + off-the-shelf NLI; temperature-scaled logistic on a few hundred labels.
4. **Signal:** account+templated-text+timing features, Platt calibration, exact-hash dedup, fixed N; single-line brief + round-trip audit.
5. **Generation:** one locked scene, image+story blocks, mandatory disclosure_slot + negative_constraints, schema-parse check before emit.
6. **Compliance:** green-lane router (own-persona / consented / parody-with-cues), hard blocks, one C2PA token bound to one slot, counsel queue for the rest.
7. **Resonance:** manual 4-driver checklist + safe-lane whitelist + share/save-weighted sentiment.
8. **Eval:** one construct, frozen OEC, two baselines, `max_cycles`, ~30–50 post-cutoff blind pairs, base-rate lift with bootstrap CI.
9. **Controller:** linear DAG, rule router, parallel signal fan-out, gate-before-emit, outbox handoff, offline improve-loop.
10. **Growth:** one beachhead, one owned list, manual subject-scoring, weekly conversion-KPI review.

**The honest MVP target is not virality — it is one end-to-end run that (a) ships a clearance-passing
brief about a safe-lane subject and (b) produces one leakage-controlled eval verdict.** Prove the
loop closes before optimizing the content.

## Build sequence (phased)

1. **Substrate** — `ir` Tier 0/1/2 ingestion + dedup + provenance store; the `orch` linear DAG skeleton + blackboard + outbox.
2. **Understanding** — `salience` map + `signal` brief + `reason` link-scorer over real ingested data; produce the dense brief.
3. **Generation + gates** — `creative` gen_brief schema; `compliance` green-lane router + disclosure token; `psych` safe-lanes + driver checklist.
4. **Proof** — `eval` pre-registration + post-cutoff holdout + terminating loop; run the medium-tier claim as a real experiment; wire `marketing`'s conversion KPI as the guardrail metric.
5. **Iterate** — only after the loop closes: the "later" upgrades each specialist named (X firehose, isotonic calibration, fine-tuned verifier, multi-variant briefs, multi-niche expansion).

## Make-it-real checklist (consolidated must-haves)

- **Frozen configs** — salience signal-weights + tier bands; the brief field-set/entropy-floor contract; the eval pre-registration hash. *(Reproducibility.)*
- **A typed-tag entity registry + geo-relevance predicate table** — without it, links aren't licensable.
- **A versioned labeled CIB benchmark + a labeled link/authenticity validation set** — without them, the signal estimator and the reasoning go/no-go are uncalibrated.
- **A real realized-outcome ledger + a confirmed knowledge cutoff** — without them, the eval measures memorization, not skill (returns NO-GO).
- **A named, addressable downstream disclosure slot + a standing counsel-escalation channel** — without them, compliance fails closed at the handoff.
- **A consent-compliant owned-audience capture + per-link source-of-truth** — without it, the growth funnel can't be measured or scaled.
- **A per-platform ToS compliance register** — X and Instagram are the legal pinch points; gate them behind human review.

## Bottom line

Run constructively through its own panel, the idea resolves into a **concrete, buildable system with
a defined MVP** — and the safety pieces became *part of the build*, not a wall around it:
`compliance` is a path-to-yes with green lanes, `psych` is an ethical-resonance playbook with safe
lanes. The fastest way to make it real is the v1 slice above: prove one end-to-end run that ships a
clearance-passing brief about a safe-lane subject and returns one honest eval verdict — then optimize.
