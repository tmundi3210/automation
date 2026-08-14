# ADR — B60 v1 scaffold: what's proven, what's a bet, what to build vs buy

This records the load-bearing decisions in the v1 scaffold so a future builder knows what
is settled, what is still a bet, and where the real risk lives. It mirrors the panel's
critical read (`../PANEL.md`) — the scaffold is the *constructive* build, but the panel's
warnings are encoded as guardrails here, not waved away.

## 1. The pipeline is plumbing; the eval is the proof. (proven shape / unproven claims)
The staged DAG (ingest→…→gate→emit) is a sound, boring shape — `orch` confirmed it. What is
NOT proven is any claim that the *content* works: "medium-tier is the opportunity", "the
links are good", "the brief resonates". Those live or die in `eval/`. **Decision:** keep the
spine simple and put all the skepticism in the eval harness, with a frozen pre-registration
and a CI that must exclude zero. The scaffold deliberately returns **NO-GO** on synthetic
data rather than flatter itself.

## 2. Compliance is a compile-time predecessor of emit, not a runtime step. (safety)
The single biggest failure mode (`orch`, `compliance`) is the controller routing around the
gate under load. **Decision:** `emit`'s predecessor set is a static check (`assert_emit_guarded`)
that runs every invocation; the handoff is an outbox packet a downstream session pulls, so
there is no in-process path from `creative` to `emit` that skips `compliance_gate`.

## 3. Everything is calibrated or boolean-gated, never an un-grounded score. (honesty)
`signal` returns a **Platt-calibrated probability**, not a boolean "bot/not-bot". `reason`/
`link` gate on a **boolean geo predicate** before a relatedness score may license a join.
`eval` reports **bootstrap CIs**, not point estimates. **Decision:** any number that changes a
decision is either calibrated against a labeled set or gated by an explicit predicate — the
"category error" findings (lossless, boolean authenticity, multi-platform = corroboration)
are designed out.

## 4. Build vs buy
| Concern | v1 decision | why |
|---|---|---|
| Ingestion | **Build** thin tiered fetchers behind `load_raw()` (mock now) | durable-first, no paid/grey dependency; the access wall is the real risk |
| Dedup | **Build** (URL canon + SimHash) | trivial, deterministic, no service needed |
| Calibration | **Build** Platt now, **buy/borrow** isotonic + a real CIB benchmark later | Platt on a tiny set is enough to prove the shape |
| Proposer/verifier/NLI/judge | **Buy** (hosted or local model via `ApiBackend`/`LocalBackend`) | this is the one place a real model earns its keep |
| C2PA signing | **Buy** a real signer at the hook point | don't roll your own provenance crypto |
| Realized-outcome ledger | **Build**, carefully, and freeze it | without it the eval measures memorization, not skill |

## 5. What is still a bet (speculative)
- That live Tier-0/1/2 ingestion gives enough **recall + freshness** on cross-platform
  reactions without paid APIs or scraping (`ir`'s binding constraint — unproven here).
- That the **medium-tier opportunity** band generalizes beyond the synthetic holdout (the
  scaffold confirms it *only* on planted data; real data is the test).
- That the **owned-audience funnel** (marketing's KPI) actually converts — not modeled in v1.
- That green-lane content (own-persona / parody-with-cues) is **commercially sufficient** —
  the gate maximizes what ships, but whether that set is worth shipping is a market question.

## 6. The two boundaries (carried from the design docs)
- **Briefs, not media.** `creative` emits a brief for a downstream renderer; it never produces
  or publishes. The disclosure slot + negative constraints travel in-band and fail closed.
- **Enablers, not vetoes-removed.** `compliance` and `psych` are path-to-yes enablers with
  green/safe lanes — but the hard blocks (minor, real-person + political + voice, fabricated
  wrongdoing) stay hard and are never auto-approved. Screening heuristic, not legal advice.
