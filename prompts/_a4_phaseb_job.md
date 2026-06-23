# A.4 Phase-B KB Job (HARDENED) — generate ONE dense llm_engineering KB YOURSELF

## CRITICAL EXECUTION RULES (read first, obey absolutely)
- You MUST do all the work YOURSELF in THIS session. Do NOT spawn, launch, or delegate to
  any sub-agent or background task. **Do NOT use the Agent tool.**
- Do NOT say you are "waiting" for anything. There is no background job — YOU are the generator.
- You are NOT done until the file `{{OUTPUT_BASENAME}}.kb.json` EXISTS on disk AND you have
  run the validator on it.
- Your FINAL message must be ONLY the compact RETURN JSON below — nothing else, no prose.

## Role
You are the generator. Working dir: /home/user/automation. You produce one dense,
subdomain-grain KB for the Phase B "llm_engineering" set, gate it, and return the report.

## Procedure
1. Read `/home/user/automation/schema/kb_generator_v1.4.1.txt` IN FULL and treat it as your
   system instructions. MODE=KB_GENERATION, DENSITY_MODE=dense.
2. Author the semantic content + base metric estimates for the FILLS subdomain. Then write a
   SMALL PYTHON SCRIPT that loads your draft JSON and recomputes ALL derived fields exactly
   from the schema formulas, rounds to 2 decimals, and rewrites the JSON:
   final_importance, risk_score, confidence_score, revisit_pressure, lock_score,
   posterior_importance(=PI*EC when no observed data), edge_priority, edge_score, edge_heat,
   uncertainty_range_width(=upper-lower), revisit_trigger_count_normalized(=min(1,count/5)).
   normalize(x)=min(1,max(0,x)).
3. Honor this directive while generating (do NOT embed it; KB stays strict JSON):
   "losslessly compressed, token-efficient, information-dense, fully detailed, machine-facing;
   optimized for model parsing over human readability."

## Phase B purpose (use THIS, not any knowledge-searcher purpose)
This KB is a subdomain node in an "LLM engineering" knowledge base about how to ENGINEER, in
the real world, an orchestrated system of fine-tuned specialist language models. Nodes are
concrete, evaluable work units / methods / decision areas a practitioner performs in this
subdomain. CRITICAL: encode real-world execution constraints — base-model license/ToS limits,
GPU/compute cost, benchmark contamination & Goodhart effects, the limit that small fine-tuned
specialists beat generalists only on narrow well-specified tasks, and orchestration
latency/cost compounding — as conflict_axes and/or dominance_rules where they genuinely apply.

## Hard requirements
- Strict JSON only: first non-whitespace char '{', last '}'. No markdown, comments, placeholders.
- No observed/experimentally_validated labels: use heuristic; observed_impact / data_quality /
  observed_co_occurrence = 0.0; posterior_importance = PI*EC.
- Dense counts: 19-24 nodes, 32-40 edges, 8-10 conflict_axes, 10-12 edge_cases, 9-12 workflow,
  10-14 competency_questions, 7-12 dominance_rules, 7-12 anti_rework_rules, 6-10 iteration_protocol.
- Only GENUINELY DISTINCT nodes; do NOT pad with near-duplicates.
- Reference integrity: unique node IDs; edge IDs EDGE_001...; every edge/dependency/
  must_not_finalize_before references a real node id; priority_order lists every node once;
  evidence_refs resolve to source_registry (include SRC_HEURISTIC_PRIOR); competency_question_refs
  resolve; dependency edges acyclic; conflict edges signed_tension<0 with a resolution_rule;
  high-risk nodes (risk_if_wrong>=0.80) have acceptance_tests + human_review_required=true;
  high-coupling nodes (cross_topic_coupling>=0.75) have revisit_triggers.

## Steps (run these shell commands yourself)
1. Write raw JSON to `{{OUTPUT_BASENAME}}.kb.json`
2. `python3 /home/user/automation/validators/kb_validator.py {{OUTPUT_BASENAME}}.kb.json --mode dense --report {{OUTPUT_BASENAME}}.validation.json --quiet ; echo "EXIT=$?"`
3. `python3 /home/user/automation/validators/metrics.py {{OUTPUT_BASENAME}}.kb.json --label {{UNIT_LABEL}} --report {{OUTPUT_BASENAME}}.metrics.json`
4. If EXIT != 0: read the validation report, do ONE self-repair pass (fix via the formula
   script + reference fixes), re-run steps 2-3.
5. Delete any temp scripts. Leave only the three deliverable files. Touch no other unit's files.

## RETURN (your only final output)
{"unit_label":"{{UNIT_LABEL}}","output_path":"{{OUTPUT_BASENAME}}.kb.json","validator_overall_status":"<from report>","validator_failed_check_ids":[...],"est_tokens":<from metrics>,"counts":{...from metrics...},"natural_fill":<bool>,"padded_node_count":<int>,"truncated":<bool>,"self_repair_attempted":<bool>,"notes":"<=160 chars"}
