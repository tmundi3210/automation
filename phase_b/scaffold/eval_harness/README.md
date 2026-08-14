# eval_harness — test sets + scorers + LLM-as-judge + score tracking

Grounded in the 'eval' specialist and `IDEA_NEUTRALIZED.md` ("scoring system; pull real
benchmark questions; test yourself"). This is also the `evaluate()` that
`train/rollback_loop.py` calls in production.

## Files
- `harness.py` — per-`task_type` scorers + LLM-as-judge + contamination flag + score
  history.
- `testset.example.jsonl` — six items spanning `numeric` / `choice` / `open`, including
  one with a deliberate reference leak to exercise the contamination guard.

## Run
```bash
python3 phase_b/scaffold/eval_harness/harness.py --system oracle   # validates the scorers (~1.0 on exact)
python3 phase_b/scaffold/eval_harness/harness.py --system mock --k 2   # MockBackend under test (realistic)
python3 phase_b/scaffold/eval_harness/harness.py phase_b/sources/_my_testset.jsonl   # your own set
```

## Scorers (per `task_type`)
- **numeric** — tool-grounded number parse + tolerance (exact work is *computed*, not
  judged — mirrors the math boundary).
- **choice** — normalized substring match to the reference.
- **open** — LLM-as-judge (`backend.score` with a rubric), graded "impartial; ignore
  order/verbosity" to blunt position/self-preference bias (eval specialist caveat).
- **pass@k / metric-at-k** — `--k` samples per item; an item passes if any sample does.

## Trust vs score
The **contamination flag** marks items whose reference leaks into the prompt (non-choice
only); flagged items are reported for *trust*, not silently dropped from the score —
the operator decides. Every run appends `{overall, by_type}` to `_scores_history.jsonl`
so a fine-tune's trajectory is trackable across runs (feeds the training monitor).

## Hook points
- `--system oracle` proves the scorers are correct (echoes the reference → exact ~1.0).
- Swap the mock judge/system for `ApiBackend`/`LocalBackend`, or pass the orchestration
  graph as the system under test, to evaluate the real composed pipeline.
