# orch — weighted any-to-any DAG orchestration

Grounded in `phase_b/specialists/orch.specialist.json` and `IDEA_NEUTRALIZED.md`
("any-to-any, weighted, generate by token budget" + "a summarizer model when many
outputs exceed an input").

## Files
- `router.py` — need → specialist(s). Faithful reproduction of `ROUTER.json`'s
  `selection_procedure`: primary score = `route_when` membership + `domain_label` ×2;
  a repeated query word does not double-count; a close second within 1 point is merged
  (capped at the top few so an ambiguous tie composes leaders rather than the whole
  table). `purpose`-overlap is a **tie-break only** — it reorders equal-primary
  candidates by relevance without changing the documented primary scoring.
- `graph.py` — generic weighted-DAG engine + a default graph builder:
  `source(need) → each selected specialist → summarizer → aggregator`, plus an
  any-to-any skip edge `source → aggregator`. Edge weights default to routing scores.

## Run
```bash
python3 phase_b/scaffold/orch/router.py            # routing table for sample needs
python3 phase_b/scaffold/orch/graph.py "your need" # end-to-end DAG (MockBackend)
```

## Behaviors implemented
- **Weighted aggregation** — upstream outputs ordered/labeled by edge weight; higher
  weight = more prominent in the synthesis context.
- **Summarizer node** — compresses combined upstream context to an input-token budget
  when a fan-in overflows it (bounds inter-agent context); passthrough otherwise.
- **Token-budget cap / value-saturation** — the aggregator includes contributors by
  descending weight until the output cap is hit, then drops the rest and records what
  was dropped (diminishing-returns stop, not a fixed length).
- **DAG guarantee** — cycles are rejected at run time.

## Known limitation (documented, not patched)
Routing is bag-of-words over `route_when`, so generic needs can tie (e.g. the
`math`↔`reason` exact-computation overlap, or "judge" diluted by "real/questions/
answers"). This is the committed ROUTER's actual behavior; `multi_specialist_policy`
composes the tied leaders and `fallback_policy` covers zero-score needs. The scaffold
reproduces this faithfully rather than hand-tuning the signal sets.

## Hook points
Swap `backends.MockBackend` for `LocalBackend` (a served fine-tuned checkpoint from the
`train/` rollback loop) or `ApiBackend`. The graph engine is unchanged by the swap.
