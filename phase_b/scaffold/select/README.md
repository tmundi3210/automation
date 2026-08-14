# select — base-model selection under license + capability constraints

Grounded in the 'select' specialist and `IDEA_NEUTRALIZED.md` ("select base model per
specialist: data, restriction[license], bench, context, size, tool support" + the
quality/cost/latency objective dial).

## Files
- `model_sheet.json` — **TEMPLATE** candidate rows + per-domain requirement profiles +
  objective weightings. The tiers are illustrative, not live data.
- `select.py` — hard-filter + soft-score selector.

## Run
```bash
python3 phase_b/scaffold/select/select.py --map        # one pick per domain (all 12 specialists)
python3 phase_b/scaffold/select/select.py --dim code --min-context 64000 \
        --require-tool-use --licenses permissive --objective quality
```

## How it picks
1. **Hard filters** — license class ∈ allowed, `context_tokens ≥ min`, `params ≤ max`,
   tool-use if required. A candidate failing any is removed.
2. **Soft score** — `objective` dial weights the primary capability tier against
   `rel_cost` and a size penalty (`quality` / `balanced` / `cost`). The dial visibly
   changes the winner (e.g. for code, `cost` favours a cheap general model, `quality`
   favours the code-specialized one).
3. `--map` applies each domain's profile (derived from that specialist's emphasis) and
   reports pick + runner-up for all 12 specialists.

## Honesty (important)
Model benchmarks, license terms, context windows, and prices are **volatile
frontier-not-in-books data** (IDEA_NEUTRALIZED §7). `model_sheet.json` is a worked
example, **not** a current fact table — every field carries a `verify` note. The durable
deliverable is the *selection logic*; before any real decision, refresh the rows from the
live model card / leaderboard / license. `frontier-proprietary-api` rows are flagged as
non-tunable (use as teacher/judge or for nodes that don't need a custom fine-tune).
