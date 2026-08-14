# salience/ — salience + opportunity + MAP expander (grounded in `salience_heavy`)

- `scoring.py` — `score()` = weighted **geometric mean** of four log-scaled public signals
  (Trends, followers, news volume, Wikipedia pageviews) + **opportunity = under_served ×
  reachable × addressable_value** + a provisional hype tier. `CONFIG` is **frozen** (weights,
  caps, tier bands) for reproducibility. The "medium-tier is the opportunity" claim is *computed
  per node*, never assumed — the owned persona lands MEDIUM_OPPORTUNITY and outranks the
  OVER_SATURATED real artist on opportunity.
- `expander.py` — bounded **region → niche → top-5** frontier (depth≤4, breadth≤8, top_k=5,
  long-tail cutoff), strictly-decreasing, a pure function of the scored set (deterministic rebuild).

```bash
python3 salience/scoring.py      # per-node salience/opportunity/tier
python3 salience/expander.py     # the niche frontiers
```
