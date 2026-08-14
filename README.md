# automation — orchestrated knowledge-base & specialist factory

An orchestrated pipeline that turns the **Expert-System JSON KB Generator** schema
(`schema/kb_generator_v1.4.1.txt`) into a repeatable factory:

1. **Phase A** builds a foundational *Knowledge Searcher* KB — a machine-facing map of
   where/how authoritative knowledge is located, retrieved, evaluated, and synthesized.
2. **Phase B** takes any raw idea, neutralizes it, converts it to an academic taxonomy
   (grounded by Phase A), generates grouped dense KBs, and distills a **specialist** per KB.

## How it runs
- **One orchestrator** (a Claude session) holds the plan and enforces gates. It does **not**
  do heavy generation in-context.
- **Helper sub-agents** each run one schema job, write JSON to a file, and return a compact
  report. This keeps the orchestrator lean and lets work fan out.
- A **deterministic gate** (`validators/kb_validator.py` + `validators/metrics.py`) checks
  every output for quality (schema/reference/formula integrity) and quantity (density-mode
  counts, token size) before it is accepted.

## Layout
- `PLAN.md` — master plan + live orchestration state (**start here**).
- `schema/` — the generator prompt + the machine-facing pipeline directive.
- `taxonomy/` — the Knowledge Searcher meta-domain seed.
- `validators/` — the quality/quantity gate (Python stdlib only).
- `prompts/` — helper job specs (calibration + generic generation).
- `calibration/`, `knowledge_base/`, `specialists/` — generated artifacts.

## Validator quick use
```
python3 validators/kb_validator.py path/to/KB.json --mode dense --report out.validation.json
python3 validators/metrics.py      path/to/KB.json --label run1 --report out.metrics.json
```

## Status
Scaffold complete. Calibration sweep (to fix the optimal batch size) is the next step and
is gated on user go-ahead — see `PLAN.md` §7–8.
