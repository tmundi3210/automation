# Generic KB Generation Job (Phase A.4 mass generation + Phase B)

Reusable job spec the orchestrator injects into a helper sub-agent to generate one
group's KB after GROUP_SIZE is fixed by calibration. Same lean-context contract:
helper writes files, returns a compact report.

## Inputs the orchestrator fills
- {{GROUP_ID}}            : stable id for this group of taxonomy domains
- {{DOMAIN_INPUT}}        : the combined DOMAIN text for the group (<= GROUP_SIZE domains)
- {{DENSITY_MODE}}        : dense (default) | standard | compact (low-value groups)
- {{OUTPUT_PATH}}         : knowledge_base/<set>/<GROUP_ID>.kb.json
- {{VALIDATION_REPORT_PATH}} : knowledge_base/<set>/<GROUP_ID>.validation.json
- {{METRICS_REPORT_PATH}} : knowledge_base/<set>/<GROUP_ID>.metrics.json
- {{ADAPTER_SCHEMA_PATH}} : optional EXISTING_SCHEMA (Phase B domain-adapter), else empty

## Helper instructions (inject verbatim)
```
You are a HELPER executing one KB_GENERATION run. Do NOT chat. Return only the final report.

1. Read generator prompt: schema/kb_generator_v1.4.1.txt
2. User input to that prompt:
     MODE: KB_GENERATION
     DENSITY_MODE: {{DENSITY_MODE}}
     DOMAIN: {{DOMAIN_INPUT}}
     {{#ADAPTER_SCHEMA_PATH}}EXISTING_SCHEMA: <contents of {{ADAPTER_SCHEMA_PATH}}>{{/ADAPTER_SCHEMA_PATH}}
   Honor pipeline directive: "losslessly compressed, token-efficient, information-dense,
   fully detailed, machine-facing; optimized for model parsing over human readability."
3. Emit KB as strict JSON only and write raw JSON to {{OUTPUT_PATH}}.
4. Validate: python3 validators/kb_validator.py {{OUTPUT_PATH}} --mode {{DENSITY_MODE}} --report {{VALIDATION_REPORT_PATH}} --quiet
5. Metrics:  python3 validators/metrics.py {{OUTPUT_PATH}} --label "{{GROUP_ID}}" --report {{METRICS_REPORT_PATH}}
6. If validator exit != 0: one self-repair pass, re-validate. If still failing, return status "needs_orchestrator".
7. Return ONLY compact JSON:
   {"group_id":"{{GROUP_ID}}","output_path":"{{OUTPUT_PATH}}","status":"<validator overall_status>",
    "failed_check_ids":[...],"est_tokens":<n>,"counts":{...},"needs_orchestrator":<bool>,"notes":"<=200 chars"}
```

## Phase B addendum — specialist creation
After a group's KB passes the gate, the orchestrator spawns a helper to derive a
SPECIALIST from that KB:
- input: the validated group KB(s) covering a coherent domain
- output: specialists/<DOMAIN>.specialist.json — a system-prompt + capability/escalation
  spec grounded ONLY in the KB (nodes/workflow/dominance_rules/validation_protocol),
  carrying the pipeline directive in its `_directive` field.
