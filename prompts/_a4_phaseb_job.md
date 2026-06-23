# A.4 Phase-B KB Job (v3) — generate ONE dense llm_engineering KB YOURSELF

## CRITICAL EXECUTION RULES (obey absolutely)
- Do ALL work YOURSELF in THIS session. Do NOT spawn, launch, or delegate to any sub-agent.
  **Do NOT use the Agent tool.** Do NOT say you are "waiting" — YOU are the generator.
- You are NOT done until `{{OUTPUT_BASENAME}}.kb.json` EXISTS on disk AND the validator passes
  (or you have done 3 repair passes). Your FINAL message = ONLY the compact RETURN JSON.

## Authoritative STRUCTURAL TEMPLATE (copy its SHAPE, not its CONTENT)
Read `/home/user/automation/knowledge_base/knowledge_searcher/ir__ranking_and_relevance.kb.json`.
It is a KNOWN-PASSING dense KB. Your output MUST use the SAME top-level keys, the SAME node
field names/shapes, the SAME edge field names/shapes, and the SAME auxiliary sections
(conflict_axes, edge_cases, workflow, competency_questions, dominance_rules, anti_rework_rules,
iteration_protocol, source_registry, priority_order, validation_protocol, etc.). Replace ALL
the information-retrieval content with YOUR subdomain's content. Do NOT copy its content.
Matching this structure is what makes you pass the structural / required-key / edge-type checks.

## Procedure
1. Read the structural template above. (Optionally skim schema/kb_generator_v1.4.1.txt for field
   semantics — but the template is the structural source of truth.)
2. Author the semantic content for the FILLS subdomain: 19-24 GENUINELY DISTINCT nodes (no
   padding to hit the count), 32-40 edges, 8-10 conflict_axes, 10-12 edge_cases, 9-12 workflow,
   10-14 competency_questions, 7-12 dominance_rules, 7-12 anti_rework_rules, 6-10 iteration_protocol.
   Author the BASE metrics ONLY for each node (do NOT hand-compute derived scores):
     metrics: criticality, business_value, user_value, technical_complexity, risk_if_wrong,
       cross_topic_coupling, irreversibility, confidence, node_conflict_pressure   (all in [0,1])
     score_derivation_inputs: acceptance_test_pass_rate, dependency_gate_pass_rate
     probabilistic_layer: evidence_confidence, data_quality, failure_rate, downside_weight,
       prior_importance, observed_impact(=0.0), uncertainty_interval [lo,hi]
   For each edge author BASE fields: relation_strength, signed_tension, expected_rework_cost,
   conflict_probability, causal_confidence, a LEGAL edge type, and from/to real node ids.
   Leave ALL derived fields (final_importance, risk_score, confidence_score, revisit_pressure,
   lock_score, posterior_importance, edge_priority, edge_score, edge_heat, uncertainty_range_width,
   revisit_trigger_count_normalized) set to 0 — the formula tool fills them in step 2 below.
3. Hard rules: strict JSON (first char '{', last '}'); no markdown/comments/placeholders; no
   observed/experimental evidence labels (use heuristic; observed_impact=0.0, data_quality may be
   >0 for prior quality but keep observed_* = 0.0); include SRC_HEURISTIC_PRIOR in source_registry;
   unique node ids; edge ids EDGE_001..; priority_order lists EVERY node id exactly once;
   high-risk nodes (risk_if_wrong>=0.80) MUST have non-empty acceptance_tests AND
   human_review_required=true; high-coupling nodes (cross_topic_coupling>=0.75) MUST have
   revisit_triggers; conflict edges have signed_tension<0 AND a resolution_rule; dependency edges
   acyclic; every evidence_ref / competency_question_ref / conflict_axis_ref resolves.
4. Honor the directive (do NOT embed it; KB stays strict JSON): "losslessly compressed,
   token-efficient, information-dense, fully detailed, machine-facing; optimized for model parsing
   over human readability."

## Phase B purpose
This KB is a subdomain node about how to ENGINEER, in the real world, an orchestrated system of
fine-tuned specialist LLMs. Nodes are concrete, evaluable work units / methods / decision areas a
practitioner performs. Encode real-world constraints (base-model license/ToS, GPU/compute cost,
benchmark contamination & Goodhart, specialist-beats-generalist-only-on-narrow-tasks, orchestration
latency/cost compounding) as conflict_axes and/or dominance_rules where they genuinely apply.

## How to write the large file (IMPORTANT — beats output token limits)
The KB is ~80-110 KB; do NOT try to emit it all in one chat message. Instead author a SHORT
Python builder script in the scratchpad that constructs the KB dict and `json.dump`s it to the
output path. Use a helper so each node only overrides non-default base metrics, e.g.:
```
def node(id,label,topic,defn,**kw):
    m={"criticality":.5,"business_value":.5,"user_value":.5,"technical_complexity":.5,
       "risk_if_wrong":.5,"cross_topic_coupling":.5,"irreversibility":.4,"confidence":.6,
       "node_conflict_pressure":.3, **{k:kw.pop(k) for k in list(kw) if k in (
       "criticality","business_value","user_value","technical_complexity","risk_if_wrong",
       "cross_topic_coupling","irreversibility","confidence","node_conflict_pressure")}}
    # ...assemble full node dict matching the template's shape (metrics=m + derived=0,
    #    score_derivation_inputs, probabilistic_layer, acceptance_tests, revisit_triggers,
    #    dependencies, evidence_refs, etc.)...
```
Run the builder, then DELETE it. This writes the file mechanically and avoids truncation.

## Steps (run these yourself)
1. Author + run the Python builder to write `{{OUTPUT_BASENAME}}.kb.json` (base metrics set; all
   derived fields = 0). Then delete the builder script.
2. Fill ALL derived fields deterministically (do NOT hand-roll a formula script):
   `python3 /home/user/automation/tools/compute_kb_formulas.py {{OUTPUT_BASENAME}}.kb.json --inplace`
3. Validate:
   `python3 /home/user/automation/validators/kb_validator.py {{OUTPUT_BASENAME}}.kb.json --mode dense --report {{OUTPUT_BASENAME}}.validation.json --quiet ; echo "EXIT=$?"`
4. Metrics:
   `python3 /home/user/automation/validators/metrics.py {{OUTPUT_BASENAME}}.kb.json --label {{UNIT_LABEL}} --report {{OUTPUT_BASENAME}}.metrics.json`
5. If EXIT != 0: read the validation report, fix the flagged issues (structure / refs / counts /
   obligations / bounds), then re-run steps 2-4. Repeat up to 3 times. (If formula.consistency
   ever fails, just re-run step 2 — the tool is authoritative.)
6. Leave only the 3 deliverable files; delete any temp files. Touch no other unit's files.

## RETURN (only this compact JSON)
{"unit_label":"{{UNIT_LABEL}}","output_path":"{{OUTPUT_BASENAME}}.kb.json","validator_overall_status":"<from report>","validator_failed_check_ids":[...],"est_tokens":<from metrics>,"counts":{...from metrics...},"natural_fill":<bool>,"padded_node_count":<int>,"self_repair_attempted":<bool>,"notes":"<=160 chars"}
