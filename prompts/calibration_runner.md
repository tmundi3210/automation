# Calibration Runner — group-size discovery (Phase A.3)

GOAL: empirically find the largest batch of taxonomy "domains" that one dense-mode
KB_GENERATION run can produce **without quality degradation or output truncation**.
The answer becomes the fixed GROUP_SIZE for all later mass generation (Phase A.4 / Phase B).

The orchestrator (main window) NEVER runs generation itself. It spawns helper
sub-agents, one per run, each given the job template below. Helpers write the KB
JSON to a file and return ONLY a compact report so orchestrator context stays lean.

## Protocol (run by orchestrator)
1. RUN_1  : 1 domain  (e.g. `information_retrieval_and_search_systems`).
2. RUN_2  : a different single domain.
3. RUN_1+2: both domains together as one combined DOMAIN input.
4. Compare RUN_1 & RUN_2 (individually) vs RUN_1+2 (combined) on the metrics below.
5. If combined shows NO degradation, escalate to RUN_4 (4 domains) and re-check.
6. Stop escalating at the first batch size where degradation OR truncation appears.
   GROUP_SIZE = largest batch that still passed.

## Degradation signals (quality dimension — from kb_validator.py)
- validator overall_status flips pass -> fail/pass_with_warnings
- count.* checks drop below dense-mode targets (under-population under batch pressure)
- formula.consistency mismatch count rises
- reference-integrity failures appear (endpoints / priority_order / evidence_refs)
- placeholders leak
- truncation: output not valid JSON / last char != "}" / abrupt cutoff

## Capacity dimension (quantity — from metrics.py)
- est_tokens per run; watch for approach to the model's single-response output ceiling
- per-domain est_tokens in combined run vs solo run (compression-under-batch effect)

## Helper job template (inject verbatim, fill {{...}})
```
You are a HELPER executing one KB_GENERATION run for a calibration experiment.
Do NOT chat. Do the steps and return only the final compact JSON report.

1. Read the generator prompt at: schema/kb_generator_v1.4.1.txt
2. Treat the following as the user input to that prompt:
     MODE: KB_GENERATION
     DENSITY_MODE: dense
     DOMAIN: {{DOMAIN_OR_COMBINED_DOMAINS}}
   Honor this pipeline directive while generating: "losslessly compressed,
   token-efficient, information-dense, fully detailed, machine-facing; optimized
   for model parsing over human readability."
3. Produce the KB strictly per the schema (valid JSON only, first char '{' last '}').
4. Write the KB JSON to: {{OUTPUT_PATH}}   (raw JSON only, no markdown fences)
5. Run: python3 validators/kb_validator.py {{OUTPUT_PATH}} --mode dense --report {{VALIDATION_REPORT_PATH}} --quiet
6. Run: python3 validators/metrics.py {{OUTPUT_PATH}} --label "{{RUN_LABEL}}" --report {{METRICS_REPORT_PATH}}
7. If kb_validator exit code != 0, attempt ONE self-repair pass on the KB file, then re-run step 5.
8. Return ONLY this compact JSON (no prose):
   {
     "run_label": "{{RUN_LABEL}}",
     "output_path": "{{OUTPUT_PATH}}",
     "validator_overall_status": "<from validation report>",
     "validator_failed_check_ids": [...],
     "est_tokens": <from metrics>,
     "counts": {...from metrics...},
     "truncated": <true|false>,
     "self_repair_attempted": <true|false>,
     "notes": "<=200 chars"
   }
```

## Orchestrator decision record (write to calibration/results/DECISION.json)
After the sweep, record: chosen GROUP_SIZE, the evidence (per-run reports),
the degradation point, and the recommended dense-mode reduce-first order if any
run approached the output ceiling.
