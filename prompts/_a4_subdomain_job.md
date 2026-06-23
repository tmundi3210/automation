# A.4 Subdomain KB Job — shared helper instructions (Knowledge Searcher build)

You are an A.4 HELPER sub-agent. Working dir is /home/user/automation. You generate ONE
dense, subdomain-grain KB for the Knowledge Searcher set, gate it, and return ONLY the
final compact JSON report (section RETURN). Do NOT chat or explain.

The orchestrator gives you a FILLS block with: UNIT_LABEL, PARENT_DOMAIN, SUBDOMAIN,
SCOPE_HINT, KEY_BOOKS, OUTPUT_BASENAME.

## Procedure
1. Read the generator prompt /home/user/automation/schema/kb_generator_v1.4.1.txt IN FULL and treat it as your system instructions. MODE=KB_GENERATION, DENSITY_MODE=dense.
2. Run it with this input:
   - MODE: KB_GENERATION
   - DENSITY_MODE: dense
   - DOMAIN: {{SUBDOMAIN}} (subdomain of {{PARENT_DOMAIN}})
   - DOMAIN_CONTEXT: {{SCOPE_HINT}} . key_books: {{KEY_BOOKS}}
   - PURPOSE_HINT: this KB is a subdomain node in a "knowledge searcher" meta-KB about WHERE and HOW to find, retrieve, evaluate, and synthesize authoritative knowledge. Nodes are concrete, evaluable work units / methods / decision areas of this subdomain.
3. Honor this pipeline directive while generating (do NOT embed it; the KB must stay strict JSON): "losslessly compressed, token-efficient, information-dense, fully detailed, machine-facing; optimized for model parsing over human readability."

## Hard requirements
- Strict JSON only: first non-whitespace char '{', last '}'. No markdown, no comments, no placeholders.
- No observed/experimentally_validated labels (no observed data): use heuristic; observed_impact / data_quality / observed_co_occurrence = 0.0; posterior_importance = PI*EC.
- Dense counts: 19-24 nodes, 32-40 edges, 8-10 conflict_axes, 10-12 edge_cases, 9-12 workflow, 10-14 competency_questions, 7-12 dominance_rules, 7-12 anti_rework_rules, 6-10 iteration_protocol.
- Generate only GENUINELY DISTINCT nodes; do NOT pad with near-duplicates to reach the count.
- FORMULA CONSISTENCY IS STRICT (validator tol 0.02). Author the semantic content + base metric estimates, then write a SMALL PYTHON SCRIPT that loads your draft JSON and computes ALL derived fields exactly from the schema formulas: final_importance, risk_score, confidence_score, revisit_pressure, lock_score, posterior_importance(=PI*EC), edge_priority, edge_score, edge_heat, uncertainty_range_width(=upper-lower), revisit_trigger_count_normalized(=min(1,count/5)); round to 2 decimals; rewrite the JSON. normalize(x)=min(1,max(0,x)).
- Reference integrity: unique node IDs; edge IDs EDGE_001...; every edge from/to + every dependency/must_not_finalize_before references a real node id; priority_order lists every node exactly once; evidence_refs resolve to source_registry (include SRC_HEURISTIC_PRIOR); competency_question_refs resolve; dependency edges acyclic; conflict edges signed_tension<0 with a resolution_rule; high-risk nodes (risk_if_wrong>=0.80) have acceptance_tests and human_review_required=true; high-coupling nodes (cross_topic_coupling>=0.75) have revisit_triggers.

## Steps
1. Write the KB (raw JSON, no fences) to: {{OUTPUT_BASENAME}}.kb.json
2. Run: python3 /home/user/automation/validators/kb_validator.py {{OUTPUT_BASENAME}}.kb.json --mode dense --report {{OUTPUT_BASENAME}}.validation.json --quiet ; echo "EXIT=$?"
3. Run: python3 /home/user/automation/validators/metrics.py {{OUTPUT_BASENAME}}.kb.json --label {{UNIT_LABEL}} --report {{OUTPUT_BASENAME}}.metrics.json
4. If EXIT != 0: read the validation report, do ONE self-repair pass (prefer fixing via your formula script + reference fixes), re-run steps 2-3.
5. Delete any temporary scripts you created; leave only the three deliverable files. Do NOT touch any other unit's files.

## RETURN (only this compact JSON, no other text)
{"unit_label":"{{UNIT_LABEL}}","output_path":"{{OUTPUT_BASENAME}}.kb.json","validator_overall_status":"<from validation report>","validator_failed_check_ids":[...],"est_tokens":<from metrics>,"counts":{...from metrics...},"natural_fill":<true|false: >=19 genuinely distinct nodes with no padding>,"padded_node_count":<int>,"truncated":<bool>,"self_repair_attempted":<bool>,"notes":"<=160 chars"}
