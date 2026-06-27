# reaction/ — cross-platform reaction reading (grounded in `signal_heavy`)

Fills the schema `signal` block + finalizes `hype` + writes the `machine_summary`.
(Named `reaction/` to avoid colliding with the stdlib `signal` module; it implements the
"signal" stage.)

- `authenticity.py` — **calibrated** `P(inauthentic | public signals)` (never a boolean) from
  five feature families → a hand-weighted logit → **Platt scaling** fitted on a versioned
  labeled CIB benchmark. Aggregate sentiment is **organic-weighted** (a bot-laden platform is
  down-weighted), so "what real people think" isn't dominated by a campaign. Plus manipulation-
  signal flags + `consensus_vs_division`.
- `hype.py` — **genuine-hype gate**: dedup origins to **distinct independent origins**, discount
  common-method single-source amplification, require ≥N independent origins *independent of
  volume*, and emit `authenticity_adjusted_hype = raw × (1 − bot_risk)` + the final tier.
- `dense_brief.py` — the one-line pipe/KV `machine_summary` + `schema_hint` + a **round-trip
  coverage audit**: decision-lossless over a fixed field set or it fails closed.
- `_fixtures/cib_benchmark.json` — the labeled set the Platt fit calibrates against (hook point:
  replace with a real, larger, re-labeled benchmark + isotonic recalibration).

```bash
python3 reaction/authenticity.py   # calibrated bot risk + organic-weighted sentiment
python3 reaction/hype.py           # raw → adjusted hype, independent-origins gate
python3 reaction/dense_brief.py    # the lossless dense line + audit
```
