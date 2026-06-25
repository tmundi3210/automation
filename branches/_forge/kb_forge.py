#!/usr/bin/env python3
"""
kb_forge.py — deterministic dense-KB builder for the branches/ heavy specialists.

Authors write a COMPACT content spec (prose + base metric inputs only); this builder
computes every DERIVED field exactly as validators/kb_validator.py expects, assigns
all ids, builds priority_order, auto-enforces high-risk / high-coupling obligations,
fills the static schema blocks, and emits a full schema_version 1.3 dense KB.

Because all formulas, label maps, ref-bearing ids and priority_order are produced
here (never hand-typed), a spec that builds clean passes the gate's mechanical checks
by construction. The author is responsible only for CONTENT correctness:
  - 19-24 nodes, 32-40 edges, 8-10 conflict_axes, 10-12 edge_cases,
    9-12 workflow steps, 10-14 competency_questions,
    7-12 dominance_rules, 7-12 anti_rework_rules, 6-10 iteration_protocol items
  - node.dependencies + dependency edges must stay acyclic (a DAG / layering)
  - every dependency / must_not_finalize_before / edge from-to / cq ref / workflow
    node_ref / edge_case node ref points at a real node id / CQ id
  - conflict edges supply a resolution_rule (builder forces signed_tension < 0)
  - high-risk nodes (risk_if_wrong >= 0.80) supply acceptance_tests (builder forces
    human_review_required = true); high-coupling nodes (cross_topic_coupling >= 0.75)
    supply revisit_triggers.

Usage:
  python3 kb_forge.py spec.json -o out.kb.json [--quiet]
Exit 0 = built, 1 = spec error (with a precise message). Stdlib only.

The build is a pure function of the spec (no time / randomness) so it is reproducible.
"""
import argparse
import json
import re
import sys

SCHEMA_VERSION = "1.3"
SRC_ID = "SRC_HEURISTIC_PRIOR"
LEGAL_EDGE_TYPES = {"dependency", "constraint", "conflict", "causal", "sequence", "feedback", "similarity"}

# base metric input keys an author must supply per node (all in [0,1] unless noted)
NODE_BASE_METRICS = [
    "criticality", "business_value", "user_value", "technical_complexity",
    "risk_if_wrong", "cross_topic_coupling", "irreversibility", "confidence",
    "node_conflict_pressure",
]
NODE_BASE_SDI = ["acceptance_test_pass_rate", "dependency_gate_pass_rate"]
NODE_BASE_PROB = ["prior_importance", "evidence_confidence", "failure_rate", "downside_weight"]


def die(msg):
    print(f"kb_forge: SPEC ERROR: {msg}", file=sys.stderr)
    sys.exit(1)


def norm(x):
    return min(1.0, max(0.0, x))


def r2(x):
    return round(x + 1e-9, 2)


def req(d, key, where):
    if key not in d:
        die(f"{where}: missing required field '{key}'")
    return d[key]


def num01(v, where, field):
    if not isinstance(v, (int, float)):
        die(f"{where}: field '{field}' must be a number, got {type(v).__name__}")
    if not (0.0 <= v <= 1.0):
        die(f"{where}: field '{field}'={v} out of [0,1]")
    return float(v)


def build_node(n, idx, cq_ids):
    where = f"node[{idx}] id={n.get('id')!r}"
    nid = req(n, "id", where)
    if not re.match(r"^[A-Z][A-Z0-9_]*$", nid):
        die(f"{where}: id must be UPPER_SNAKE (matched against node-id refs)")
    base = req(n, "base", where)

    m = {k: num01(base.get(k), where, k) for k in NODE_BASE_METRICS}
    sdi_in = {k: num01(base.get(k), where, k) for k in NODE_BASE_SDI}
    prob_in = {k: num01(base.get(k), where, k) for k in NODE_BASE_PROB}

    ui = req(base, "uncertainty_interval", where)
    if not (isinstance(ui, list) and len(ui) == 2 and all(isinstance(x, (int, float)) for x in ui)
            and 0.0 <= ui[0] <= ui[1] <= 1.0):
        die(f"{where}: uncertainty_interval must be [lo,hi] with 0<=lo<=hi<=1, got {ui}")
    UR = r2(ui[1] - ui[0])

    revisit = list(n.get("revisit_triggers", []))
    accept = list(n.get("acceptance_tests", []))
    RT = r2(min(1.0, len(revisit) / 5.0))

    # obligations
    if m["risk_if_wrong"] >= 0.80 and not accept:
        die(f"{where}: risk_if_wrong>=0.80 requires non-empty acceptance_tests")
    if m["cross_topic_coupling"] >= 0.75 and not revisit:
        die(f"{where}: cross_topic_coupling>=0.75 requires non-empty revisit_triggers")
    human_review = bool(n.get("human_review_required", False)) or m["risk_if_wrong"] >= 0.80

    C, BV, UV, TC = m["criticality"], m["business_value"], m["user_value"], m["technical_complexity"]
    R, X, IR, CF = m["risk_if_wrong"], m["cross_topic_coupling"], m["irreversibility"], m["confidence"]
    NCP = m["node_conflict_pressure"]
    AT, DG = sdi_in["acceptance_test_pass_rate"], sdi_in["dependency_gate_pass_rate"]
    EC, FR, DW, PI = (prob_in["evidence_confidence"], prob_in["failure_rate"],
                      prob_in["downside_weight"], prob_in["prior_importance"])
    DQ = 0.0  # no observed data
    OI = 0.0

    final_importance = r2(norm(0.18*C + 0.12*BV + 0.12*UV + 0.14*TC + 0.16*R + 0.12*X + 0.10*IR + 0.06*CF))
    risk_score = r2(norm(0.32*R + 0.24*IR + 0.18*X + 0.16*TC + 0.10*(1-CF)))
    confidence_score = r2(norm(0.45*EC + 0.25*DQ + 0.15*AT + 0.15*DG))
    revisit_pressure = r2(norm(0.30*(1-CF) + 0.25*UR + 0.20*RT + 0.15*NCP + 0.10*FR))
    lock_score = r2(norm(0.30*AT + 0.25*DG + 0.20*CF + 0.15*(1-R) + 0.10*(1-IR)))
    posterior = r2(norm(PI*EC))

    metrics = dict(m)
    metrics.update(risk_score=risk_score, confidence_score=confidence_score,
                   revisit_pressure=revisit_pressure, lock_score=lock_score,
                   final_importance=final_importance)
    metric_labels = {k: "heuristic" for k in metrics}

    sdi = {"acceptance_test_pass_rate": AT, "dependency_gate_pass_rate": DG,
           "uncertainty_range_width": UR, "revisit_trigger_count_normalized": RT}
    sdi_labels = {k: "heuristic" for k in sdi}

    cq_refs = list(n.get("competency_question_refs", []))
    for c in cq_refs:
        if c not in cq_ids:
            die(f"{where}: competency_question_refs -> unknown CQ id {c!r}")

    prob = {
        "prior_importance": PI, "evidence_confidence": EC, "observed_impact": OI,
        "data_quality": DQ, "failure_rate": FR, "downside_weight": DW,
        "uncertainty_interval": [ui[0], ui[1]], "posterior_importance": posterior,
        "update_signal": base.get("update_signal", "material change in this work unit's assumptions or evidence"),
        "value_labels": {k: "heuristic" for k in
                         ["prior_importance", "evidence_confidence", "observed_impact", "data_quality",
                          "failure_rate", "downside_weight", "uncertainty_interval", "posterior_importance"]},
    }

    def labeled(items, where2):
        out = []
        for it in items:
            out.append({
                "weight": num01(it.get("weight", 0.7), where2, "weight"),
                "evidence_label": "heuristic",
                "evidence_refs": [SRC_ID],
                "claim": req(it, "claim", where2),
                "example": it.get("example", ""),
            })
        return out

    return {
        "id": nid,
        "topic": req(n, "topic", where),
        "definition": req(n, "definition", where),
        "node_type": n.get("node_type", "work_unit"),
        "group": n.get("group", "core"),
        "scope_boundary": {
            "included": list(n.get("scope_boundary", {}).get("included", [])),
            "excluded": list(n.get("scope_boundary", {}).get("excluded", [])),
        },
        "academic_fields": list(n.get("academic_fields", [])),
        "subfields": list(n.get("subfields", [])),
        "specialists": list(n.get("specialists", [])),
        "contradictors": list(n.get("contradictors", [])),
        "inputs": list(n.get("inputs", [])),
        "outputs": list(n.get("outputs", [])),
        "dependencies": list(n.get("dependencies", [])),
        "must_not_finalize_before": list(n.get("must_not_finalize_before", [])),
        "competency_question_refs": cq_refs,
        "evidence_refs": list(n.get("evidence_refs", [SRC_ID])) or [SRC_ID],
        "metrics": metrics,
        "score_derivation_inputs": sdi,
        "score_derivation_input_labels": sdi_labels,
        "metric_labels": metric_labels,
        "probabilistic_layer": prob,
        "pros": labeled(n.get("pros", []), where + ".pros"),
        "cons": labeled(n.get("cons", []), where + ".cons"),
        "failure_modes": list(n.get("failure_modes", [])),
        "acceptance_tests": accept,
        "revisit_triggers": revisit,
        "handoff_artifact_required": bool(n.get("handoff_artifact_required", True)),
        "lifecycle_state": n.get("lifecycle_state", "draft"),
        "human_review_required": human_review,
    }


def build_edge(e, idx, node_ids):
    where = f"edge[{idx}] {e.get('from')}->{e.get('to')}"
    fr, to = req(e, "from", where), req(e, "to", where)
    if fr not in node_ids:
        die(f"{where}: 'from' {fr!r} is not a node id")
    if to not in node_ids:
        die(f"{where}: 'to' {to!r} is not a node id")
    et = req(e, "edge_type", where)
    if et not in LEGAL_EDGE_TYPES:
        die(f"{where}: edge_type {et!r} not in {sorted(LEGAL_EDGE_TYPES)}")
    RS = num01(e.get("relation_strength", 0.7), where, "relation_strength")
    ST = e.get("signed_tension", -0.05 if et == "conflict" else 0.0)
    if not isinstance(ST, (int, float)) or not (-1.0 <= ST <= 1.0):
        die(f"{where}: signed_tension must be in [-1,1]")
    CC = num01(e.get("causal_confidence", 0.7), where, "causal_confidence")
    CP = num01(e.get("conflict_probability", 0.15), where, "conflict_probability")
    ERC = num01(e.get("expected_rework_cost", 0.25), where, "expected_rework_cost")
    resolution_rule = (e.get("resolution_rule") or "").strip()
    if et == "conflict":
        if ST >= 0:
            ST = -abs(ST) if ST != 0 else -0.3        # force negative signed tension
        if not resolution_rule:
            die(f"{where}: conflict edge requires a non-empty resolution_rule")

    ep = r2(norm(RS * (0.40*abs(ST) + 0.30*ERC + 0.20*CP + 0.10*CC)))
    eh = r2(norm(0.50*norm(RS * (0.40*abs(ST) + 0.30*ERC + 0.20*CP + 0.10*CC)) + 0.20*RS + 0.15*CP + 0.15*ERC))

    edge = {
        "id": f"EDGE_{idx:03d}",
        "from": fr, "to": to, "edge_type": et,
        "relation_strength": RS, "signed_tension": round(ST, 3),
        "edge_score": ep, "edge_heat": eh,
        "prior_relation_strength": num01(e.get("prior_relation_strength", RS), where, "prior_relation_strength"),
        "observed_co_occurrence": 0.0,
        "causal_confidence": CC, "conflict_probability": CP, "expected_rework_cost": ERC,
        "edge_priority": ep,
        "edge_metric_labels": {k: "heuristic" for k in
                               ["relation_strength", "signed_tension", "edge_score", "edge_heat",
                                "edge_priority", "causal_confidence", "conflict_probability", "expected_rework_cost"]},
        "evidence_refs": list(e.get("evidence_refs", [SRC_ID])) or [SRC_ID],
        "traceability": [],
        "why_related": req(e, "why_related", where),
        "benefit_of_coupling": e.get("benefit_of_coupling", ""),
        "risk_of_conflict": e.get("risk_of_conflict", ""),
        "example": e.get("example", ""),
    }
    if resolution_rule:
        edge["resolution_rule"] = resolution_rule
    return edge


def auto_id(items, prefix, width=2):
    for i, it in enumerate(items, 1):
        it.setdefault("id", f"{prefix}_{i:0{width}d}")
    return items


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("spec")
    ap.add_argument("-o", "--out", required=True)
    ap.add_argument("--quiet", action="store_true")
    a = ap.parse_args()

    try:
        spec = json.loads(open(a.spec, encoding="utf-8").read())
    except OSError as e:
        die(f"cannot read spec: {e}")
    except json.JSONDecodeError as e:
        die(f"spec is not valid JSON: {e}")

    domain = req(spec, "domain", "spec")
    domain_label = req(spec, "domain_label", "spec")
    purpose = req(spec, "purpose", "spec")

    cqs = req(spec, "competency_questions", "spec")
    cq_ids = {c.get("id") for c in cqs}
    if None in cq_ids:
        die("every competency_question needs an id")

    nodes_in = req(spec, "nodes", "spec")
    node_ids = [n.get("id") for n in nodes_in]
    if len(set(node_ids)) != len(node_ids):
        die(f"duplicate node ids: {sorted({x for x in node_ids if node_ids.count(x) > 1})}")
    node_id_set = set(node_ids)

    nodes = [build_node(n, i, cq_ids) for i, n in enumerate(nodes_in)]

    # validate node->node refs now that the full set is known
    for n in nodes:
        for ref in n["dependencies"] + n["must_not_finalize_before"]:
            if ref not in node_id_set:
                die(f"node {n['id']}: dependency/must_not_finalize ref {ref!r} is not a node id")

    edges_in = req(spec, "edges", "spec")
    edges = [build_edge(e, i + 1, node_id_set) for i, e in enumerate(edges_in)]

    # priority order: by final_importance desc, stable on input order
    order = sorted(nodes, key=lambda n: (-n["metrics"]["final_importance"], node_ids.index(n["id"])))
    priority_order = [n["id"] for n in order]

    # workflow node ref check
    workflow = auto_id(list(spec.get("workflow", [])), "WF")
    for i, s in enumerate(workflow, 1):
        s.setdefault("step", i)
        nr = s.get("node_ref")
        if nr is not None and nr not in node_id_set:
            die(f"workflow step {s.get('id')}: node_ref {nr!r} is not a node id")

    # edge_cases node ref check
    edge_cases = auto_id(list(spec.get("edge_cases", [])), "EC")
    for c in edge_cases:
        for ref in c.get("affected_nodes", []):
            if ref not in node_id_set:
                die(f"edge_case {c.get('id')}: affected node {ref!r} is not a node id")

    conflict_axes = auto_id(list(spec.get("conflict_axes", [])), "CA")
    dominance_rules = auto_id(list(spec.get("dominance_rules", [])), "DR")
    for i, d in enumerate(dominance_rules):
        d.setdefault("priority", 1 + (i % 3))
    anti_rework = auto_id(list(spec.get("anti_rework_rules", [])), "ARR")
    iteration = auto_id(list(spec.get("iteration_protocol", [])), "IP")
    for it in iteration:
        for ref in it.get("nodes", []):
            if ref not in node_id_set:
                die(f"iteration_protocol {it.get('id')}: node {ref!r} is not a node id")

    # traceability matrix: auto-built (validator requires the key; contents are advisory)
    def steps_for(nid):
        return [s.get("id") for s in workflow if s.get("node_ref") == nid]
    def axes_for(nid):
        return [c.get("id") for c in conflict_axes if nid in c.get("affected_nodes", [])]
    trace = []
    for n in nodes:
        trace.append({
            "node_id": n["id"],
            "workflow_steps": steps_for(n["id"]),
            "competency_questions": n["competency_question_refs"],
            "conflict_axes": axes_for(n["id"]),
            "dominance_rules": [],
            "anti_rework_rules": [],
        })

    kb = {
        "domain": domain,
        "domain_label": domain_label,
        "purpose": purpose,
        "empirical_status": "heuristic_prior_not_observed_dataset",
        "assumptions": list(spec.get("assumptions", [
            "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
        ])),
        "exclusions": list(spec.get("exclusions", [])),
        "schema_version": SCHEMA_VERSION,
        "generation_metadata": {
            "mode": "KB_GENERATION",
            "generator_prompt_version": "1.4.1",
            "kb_schema_version": SCHEMA_VERSION,
            "compatibility_target": "v1.3-style KB objects",
            "density_mode_selected": "dense",
            "source_grounding_status": "not_available",
            "calculation_mode": "heuristic_estimate",
        },
        "schema_contract": {
            "schema_language": "json_schema_2020_12_compatible",
            "additional_properties_allowed": False,
            "external_validation_required": True,
            "structured_outputs_recommended": True,
            "duplicate_key_detection_required": True,
            "strict_json_required": True,
            "enum_values_closed": True,
            "kb_schema_version": SCHEMA_VERSION,
            "compatibility_target": "v1.3-style KB objects",
        },
        "input_summary": {
            "domain_supplied": True, "domain_context_supplied": True, "purpose_hint_supplied": True,
            "observed_data_supplied": False, "expert_judgment_supplied": False,
            "experimental_or_benchmark_evidence_supplied": False, "source_material_supplied": False,
            "density_mode": "dense", "human_review_policy_supplied": False, "section_text_supplied": False,
            "existing_schema_supplied": False, "evaluation_criteria_supplied": False,
            "risk_tolerance_supplied": False, "stakeholder_context_supplied": False,
            "version_baseline_supplied": False,
        },
        "competency_questions": cqs,
        "glossary": list(spec.get("glossary", [])),
        "lens_coverage": spec.get("lens_coverage", {
            "knowledge_representation": ["glossary"],
            "expert_systems": ["dominance_rules", "iteration_protocol"],
            "graph_modeling": ["edges", "conflict_axes"],
            "mathematical_scoring": ["math_model"],
            "probabilistic_modeling": ["nodes"],
            "risk_analysis": ["edge_cases"],
            "workflow_design": ["workflow"],
            "validation_and_qa": ["validation_protocol"],
            "conflict_management": ["conflict_axes", "dominance_rules"],
            "evidence_governance": ["source_registry", "evidence_label_policy"],
            "prompt_engineering": ["schema_contract"],
            "project_and_product_evaluation": ["priority_order", "priority_rationale"],
            "ai_task_orchestration": ["iteration_protocol", "workflow"],
            "software_schema_engineering": ["schema_contract"],
            "change_control": ["change_control"],
            "traceability": ["traceability_matrix"],
        }),
        "evidence_label_policy": {
            "allowed_labels": ["heuristic", "expert_estimate", "observed", "experimentally_validated"],
            "default_label": "heuristic",
            "observed_requires": "OBSERVED_DATA",
            "experimentally_validated_requires": "EXPERIMENTAL_OR_BENCHMARK_EVIDENCE",
            "observed_impact_when_no_data": 0.0,
            "data_quality_when_no_data": 0.0,
            "observed_co_occurrence_when_no_data": 0.0,
        },
        "source_registry": [{
            "source_id": SRC_ID,
            "source_type": "heuristic_prior",
            "label_eligibility": ["heuristic"],
            "description": spec.get("source_description",
                                    f"heuristic prior estimates for {domain} work units in absence of supplied datasets"),
            "citation_or_locator": spec.get("source_citation", "heuristic prior; no external dataset supplied"),
            "supports": ["heuristic scores", "heuristic weights", "heuristic risk and confidence estimates"],
            "does_not_support": ["observed metrics", "experimentally validated claims", "benchmark numbers"],
            "freshness_status": "not_applicable",
            "provenance": {"entity": "generated_artifact", "activity": "estimation",
                           "agent": "model", "timestamp_available": False},
            "grounding_limits": ["not derived from measured data", "relative not absolute",
                                 "domain-general heuristic priors"],
        }],
        "math_model": {
            "normalization_definitions": {"normalize": "normalize(x)=min(1,max(0,x))"},
            "variable_definitions": {
                "C": "criticality", "BV": "business_value", "UV": "user_value", "TC": "technical_complexity",
                "R": "risk_if_wrong", "X": "cross_topic_coupling", "IR": "irreversibility", "CF": "confidence",
                "NCP": "node_conflict_pressure", "PI": "prior_importance", "EC": "evidence_confidence",
                "OI": "observed_impact", "DQ": "data_quality", "FR": "failure_rate", "DW": "downside_weight",
                "UR": "uncertainty_range_width", "RT": "revisit_trigger_count_normalized",
                "AT": "acceptance_test_pass_rate", "DG": "dependency_gate_pass_rate",
                "RS": "relation_strength", "ST": "signed_tension", "ES": "edge_score", "EH": "edge_heat",
                "EP": "edge_priority", "PR": "prior_relation_strength", "OC": "observed_co_occurrence",
                "CC": "causal_confidence", "CP": "conflict_probability", "ERC": "expected_rework_cost",
            },
            "derived_variable_definitions": {
                "UR": "uncertainty_interval[1]-uncertainty_interval[0]",
                "RT": "min(1,count(revisit_triggers)/5)",
                "AT": "predicted_acceptance_test_pass_rate", "DG": "predicted_dependency_gate_pass_rate",
            },
            "node_score_formula": "final_importance=normalize(0.18*C+0.12*BV+0.12*UV+0.14*TC+0.16*R+0.12*X+0.10*IR+0.06*CF)",
            "risk_score_formula": "risk_score=normalize(0.32*R+0.24*IR+0.18*X+0.16*TC+0.10*(1-CF))",
            "confidence_score_formula": "confidence_score=normalize(0.45*EC+0.25*DQ+0.15*AT+0.15*DG)",
            "revisit_pressure_formula": "revisit_pressure=normalize(0.30*(1-CF)+0.25*UR+0.20*RT+0.15*NCP+0.10*FR)",
            "lock_score_formula": "lock_score=normalize(0.30*AT+0.25*DG+0.20*CF+0.15*(1-R)+0.10*(1-IR))",
            "posterior_importance_formula_when_no_observed_data": "posterior_importance=normalize(PI*EC)",
            "posterior_importance_formula_when_observed_data_exists": "posterior_importance=normalize(PI*EC+OI*DQ-FR*DW)",
            "edge_score_formula": "edge_score=edge_priority",
            "edge_priority_formula": "edge_priority=normalize(RS*(0.40*abs(ST)+0.30*ERC+0.20*CP+0.10*CC))",
            "edge_heat_formula": "edge_heat=normalize(0.50*EP+0.20*RS+0.15*CP+0.15*ERC)",
        },
        "nodes": nodes,
        "edges": edges,
        "conflict_axes": conflict_axes,
        "edge_cases": edge_cases,
        "workflow": workflow,
        "priority_order": priority_order,
        "priority_rationale": spec.get("priority_rationale",
                                       "ordered by computed final_importance; foundational work units precede dependents"),
        "dominance_rules": dominance_rules,
        "anti_rework_rules": anti_rework,
        "iteration_protocol": iteration,
        "validation_protocol": {
            "required_checks": [
                "all node ids unique", "all edge ids unique EDGE_XXX format",
                "all from/to in edges reference valid node ids",
                "all competency_question_refs resolve to valid CQ ids",
                "all evidence_refs resolve to valid source_registry ids",
                "all priority_order entries are valid node ids with no duplicates",
                "high_risk_nodes (risk_if_wrong>=0.80) have non-empty acceptance_tests and human_review_required=true",
                "high_coupling_nodes (cross_topic_coupling>=0.75) have non-empty revisit_triggers",
                "conflict edges have signed_tension<0 and resolution_rule present",
                "dependency edges form no cycles", "observed_impact=0.0 for all nodes (no observed data)",
                "SRC_HEURISTIC_PRIOR in source_registry",
                "19-24 nodes, 32-40 edges, 8-10 conflict_axes, 10-12 edge_cases, 9-12 workflow steps",
                "schema_version=1.3",
            ],
            "post_formula_checks": [
                "all derived fields non-zero after compute run",
                "final_importance in [0,1] for all nodes",
                "risk_score, confidence_score, revisit_pressure, lock_score in [0,1]",
                "posterior_importance in [0,1]", "edge_priority, edge_score, edge_heat in [0,1]",
            ],
        },
        "validation_report": {
            "status": "pending", "last_validated": None, "validator_version": None,
            "failed_checks": [], "passed_checks": [],
            "notes": "built by kb_forge.py; run validators/kb_validator.py --mode dense to certify",
        },
        "evaluation_suite": spec.get("evaluation_suite", {
            "eval_objective": spec.get("eval_objective", f"verify_core_properties_of_{domain}_kb"),
            "test_cases": spec.get("test_cases", [
                {"id": "EVAL_CASE_001", "case_type": "minimal_domain",
                 "input_summary": "smallest meaningful instance in this domain",
                 "expected_properties": ["highest-priority node engaged first", "obligations enforced"]},
            ]),
            "metrics": spec.get("eval_metrics", ["coverage", "consistency", "obligation_adherence"]),
            "regression_policy": "re-run evaluation suite after any node addition or formula change",
            "minimum_pass_conditions": ["all required checks pass", "no formula mismatches"],
        }),
        "traceability_matrix": trace,
        "change_control": {
            "version": "1.0.0",
            "created": spec.get("created", "2026-06-25"),
            "change_log": [],
            "review_required_for": ["node addition", "edge type change", "dominance_rule change",
                                    "scope boundary change"],
        },
    }

    open(a.out, "w", encoding="utf-8").write(json.dumps(kb, indent=2))
    if not a.quiet:
        counts = {k: len(kb[k]) for k in ["nodes", "edges", "conflict_axes", "edge_cases", "workflow",
                                          "competency_questions", "dominance_rules", "anti_rework_rules",
                                          "iteration_protocol"]}
        print(f"kb_forge: built {a.out}")
        print(f"  counts: {counts}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
