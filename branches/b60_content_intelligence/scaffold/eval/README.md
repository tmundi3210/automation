# eval/ — leakage-controlled frozen-metric backtest (grounded in `eval_heavy`)

`harness.py` turns "test until it's good" into a real, terminating test:

- a sealed **`PREREG`** (one construct, one OEC, two baselines, a beat-the-human threshold,
  bootstrap settings, `max_cycles`) — hashed so it can't be edited post-hoc.
- **leakage controls**: keep only strictly **post-cutoff** items; the holdout carries realized
  outcomes the system never saw (**blind**).
- the primary OEC is the **medium-tier bet** with a **bootstrap CI** — it counts only if the CI
  **excludes zero**. Two baselines: naive base rate + human-curated.
- the improve-loop **terminates** (threshold OR `max_cycles`) with a **termination certificate**.

`_fixtures/holdout.frozen.jsonl` is the frozen synthetic holdout; `_fixtures/make_holdout.py`
regenerates it deterministically (hash-seeded, no RNG). **Hook point:** swap in a real
realized-outcome ledger + a confirmed knowledge cutoff before trusting the verdict.

Current result: medium-tier bet **confirmed** (CI excludes zero) but the system **does not beat
the human-curated baseline** → **NO-GO**. The harness refuses to overclaim.

```bash
python3 eval/_fixtures/make_holdout.py   # (re)build the frozen holdout
python3 eval/harness.py                  # the leakage-controlled verdict + certificate
```
