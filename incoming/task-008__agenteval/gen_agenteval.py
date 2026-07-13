#!/usr/bin/env python3
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def node(nid, topic, definition, deps, cq_refs, risk=0.7, coupling=0.55, accept=None, revisit=None):
    base = {
        "criticality": 0.72,
        "business_value": 0.74,
        "user_value": 0.71,
        "technical_complexity": 0.56,
        "risk_if_wrong": risk,
        "cross_topic_coupling": coupling,
        "irreversibility": 0.39,
        "confidence": 0.62,
        "node_conflict_pressure": 0.48,
        "acceptance_test_pass_rate": 0.68,
        "dependency_gate_pass_rate": 0.7,
        "prior_importance": 0.67,
        "evidence_confidence": 0.64,
        "failure_rate": 0.22,
        "downside_weight": 0.58,
        "uncertainty_interval": [0.18, 0.62],
        "update_signal": "material change in scoring assumptions, failure modes, or routing policy",
    }
    out = {
        "id": nid,
        "topic": topic,
        "definition": definition,
        "base": base,
        "node_type": "work_unit",
        "group": "core",
        "scope_boundary": {
            "included": [topic],
            "excluded": ["live deployment logic", "training pipeline internals"],
        },
        "academic_fields": ["machine learning", "evaluation", "multi-agent systems"],
        "subfields": ["rubric design", "agent reliability", "routing policy"],
        "specialists": ["agenteval"],
        "contradictors": ["ad hoc scoring", "opaque trust heuristics"],
        "inputs": ["scored artifacts", "historical runs", "review notes"],
        "outputs": ["policy guidance", "decision rubric", "trust score"],
        "dependencies": deps,
        "must_not_finalize_before": deps[:],
        "competency_question_refs": cq_refs,
        "evidence_refs": ["SRC_HEURISTIC_PRIOR"],
        "pros": [
            {"weight": 0.78, "claim": "gives concrete scoring and routing guidance", "example": "clear trust-score components"},
            {"weight": 0.71, "claim": "supports comparable agent histories", "example": "per-agent reliability tracking"},
        ],
        "cons": [
            {"weight": 0.66, "claim": "can be gamed if labels are not audited", "example": "score chasing without outcome checks"},
        ],
        "failure_modes": [
            "Goodharting on the displayed score",
            "drift between benchmark and production tasks",
        ],
        "acceptance_tests": accept or ["node content is specific enough to support a concrete scoring policy"],
        "revisit_triggers": revisit or ["new scoring failures appear in held-out audits"],
        "handoff_artifact_required": True,
        "lifecycle_state": "draft",
        "human_review_required": False,
    }
    return out


def edge(fr, to, etype, why, tension=0.0, rule=None):
    e = {
        "from": fr,
        "to": to,
        "edge_type": etype,
        "relation_strength": 0.74,
        "signed_tension": tension,
        "causal_confidence": 0.71,
        "conflict_probability": 0.16,
        "expected_rework_cost": 0.24,
        "prior_relation_strength": 0.74,
        "why_related": why,
        "benefit_of_coupling": "tightens the evaluation loop",
        "risk_of_conflict": "misaligned definitions can fragment scoring",
        "example": "operational link in the evaluation stack",
    }
    if rule:
        e["resolution_rule"] = rule
    return e


def build_spec(domain, label, purpose, source_citation, q_prompts):
    nodes = [
        node("AGENT_SCORE_INTAKE", "score intake", "Define the input artifact set, scoring grain, and comparison target.", [], ["CQ_01"], risk=0.66, coupling=0.52),
        node("ARTIFACT_RUBRIC", "artifact rubric", "Design a blind, multi-criterion rubric for work-product quality with calibration anchors.", ["AGENT_SCORE_INTAKE"], ["CQ_02"], risk=0.78, coupling=0.74, accept=["rubric covers blind judging, calibration, and bias controls"], revisit=["rubric changes when judge drift is detected"]),
        node("AGENT_HISTORY", "agent history", "Track per-agent performance across task types, revisions, and gate outcomes.", ["AGENT_SCORE_INTAKE"], ["CQ_03"], risk=0.79, coupling=0.77, accept=["history model records task-type competence and drift"], revisit=["task mix shifts or failure patterns change"]),
        node("TRUST_SCORE", "trust score", "Combine gate pass rate, human accept rate, and revise count into a resistant trust score.", ["AGENT_HISTORY"], ["CQ_04"], risk=0.82, coupling=0.8, accept=["trust score is explicit and defensible"], revisit=["gaming signals or calibration regressions appear"]),
        node("ROUTING_POLICY", "routing policy", "Map trust and capability scores to assignment, exploration, and escalation decisions.", ["TRUST_SCORE"], ["CQ_05"], risk=0.76, coupling=0.69, accept=["routing policy distinguishes high-confidence, recovery, and human-escalation cases"]),
        node("POSITION_BIAS_CONTROL", "bias controls", "Add blind pairwise comparison, order randomization, and judge calibration to reduce position bias.", ["ARTIFACT_RUBRIC"], ["CQ_06"], risk=0.75, coupling=0.73, accept=["bias control covers blind pairing and judge calibration"], revisit=["position bias is observed in audits"]),
        node("GOLD_SET_ANCHOR", "gold set", "Anchor judge calibration to a stable labeled set and monitor agreement against it.", ["POSITION_BIAS_CONTROL"], ["CQ_07"], risk=0.77, coupling=0.75, accept=["gold set is explicit and reused for calibration"], revisit=["agreement falls below the expected band"]),
        node("DRIFT_MONITOR", "drift monitor", "Detect metric drift, task-shift drift, and score inflation over time.", ["AGENT_HISTORY"], ["CQ_08"], risk=0.8, coupling=0.78, accept=["drift monitor is sensitive to reliability changes"], revisit=["historical score distribution shifts"]),
        node("GAMING_DEFENSE", "gaming defense", "Harden score design against Goodhart effects, benchmark overfitting, and shortcut behavior.", ["TRUST_SCORE"], ["CQ_09"], risk=0.84, coupling=0.81, accept=["gaming defenses are tied to observable attack modes"], revisit=["agents optimize the score instead of outcomes"]),
        node("CAPABILITY_MAPPER", "capability mapper", "Build a task-type to skill profile map for capability-matched routing.", ["AGENT_HISTORY"], ["CQ_10"], risk=0.73, coupling=0.68),
        node("EXPLORATION_POLICY", "exploration policy", "Reserve some assignments for recovering agents so exploitation does not starve re-entry.", ["ROUTING_POLICY"], ["CQ_11"], risk=0.69, coupling=0.63),
        node("ESCALATION_GATE", "escalation gate", "Escalate to human review when confidence is low, stakes are high, or drift is unresolved.", ["ROUTING_POLICY", "DRIFT_MONITOR"], ["CQ_12"], risk=0.81, coupling=0.79, accept=["escalation threshold is explicit and conservative"], revisit=["high-stakes tasks accumulate"]),
        node("JUDGE_AGREEMENT", "judge agreement", "Measure inter-judge agreement and calibration spread as a quality check on the rubric.", ["ARTIFACT_RUBRIC", "GOLD_SET_ANCHOR"], ["CQ_13"], risk=0.74, coupling=0.72),
        node("EVAL_AUDIT", "evaluation audit", "Audit the scoring loop for fairness, leakage, and reportable failure modes.", ["GAMING_DEFENSE", "JUDGE_AGREEMENT"], ["CQ_14"], risk=0.79, coupling=0.76, accept=["audit findings can trigger score redesign"], revisit=["audit uncovers a new failure mode"]),
        node("PAIRWISE_JUDGING", "pairwise judging", "Prefer blind pairwise comparison over single absolute scores where possible.", ["ARTIFACT_RUBRIC"], ["CQ_02"], risk=0.72, coupling=0.71),
        node("SCORING_CALIBRATION", "calibration", "Calibrate scores against accepted exemplars and task-family baselines.", ["GOLD_SET_ANCHOR"], ["CQ_07", "CQ_13"], risk=0.77, coupling=0.75),
        node("TASK_FAMILY_MODEL", "task family model", "Model task families so agent competence is scored by task type, not a global average.", ["CAPABILITY_MAPPER"], ["CQ_03", "CQ_10"], risk=0.71, coupling=0.7),
        node("DECISION_LOG", "decision log", "Record the scoring, route, and escalation rationale for later review.", ["ROUTING_POLICY", "ESCALATION_GATE"], ["CQ_05", "CQ_12"], risk=0.68, coupling=0.62),
        node("FEEDBACK_LOOP", "feedback loop", "Feed audit results back into rubric, trust score, and routing updates.", ["EVAL_AUDIT"], ["CQ_14"], risk=0.75, coupling=0.73),
        node("SHIELDING_RULES", "shielding rules", "Protect core metrics from direct gaming by withholding some internals from user-facing displays.", ["GAMING_DEFENSE"], ["CQ_09"], risk=0.8, coupling=0.77, accept=["shielding rule names what is hidden and why"], revisit=["metric gaming is detected"]),
    ]

    edges = [
        edge("AGENT_SCORE_INTAKE", "ARTIFACT_RUBRIC", "dependency", "rubric design depends on the intake grain"),
        edge("AGENT_SCORE_INTAKE", "AGENT_HISTORY", "dependency", "history tracking depends on the chosen scoring grain"),
        edge("ARTIFACT_RUBRIC", "POSITION_BIAS_CONTROL", "dependency", "bias controls refine the rubric"),
        edge("POSITION_BIAS_CONTROL", "GOLD_SET_ANCHOR", "dependency", "calibration needs a stable anchor set"),
        edge("GOLD_SET_ANCHOR", "SCORING_CALIBRATION", "dependency", "calibration depends on the gold anchor"),
        edge("AGENT_HISTORY", "TRUST_SCORE", "dependency", "trust score consumes historical performance"),
        edge("TRUST_SCORE", "ROUTING_POLICY", "dependency", "routing consumes the trust score"),
        edge("ROUTING_POLICY", "EXPLORATION_POLICY", "dependency", "routing includes exploration controls"),
        edge("ROUTING_POLICY", "ESCALATION_GATE", "dependency", "routing includes escalation logic"),
        edge("AGENT_HISTORY", "CAPABILITY_MAPPER", "dependency", "capability mapping is derived from history"),
        edge("CAPABILITY_MAPPER", "TASK_FAMILY_MODEL", "dependency", "task families specialize capability mapping"),
        edge("ARTIFACT_RUBRIC", "PAIRWISE_JUDGING", "dependency", "pairwise comparison is a rubric implementation"),
        edge("SCORING_CALIBRATION", "JUDGE_AGREEMENT", "dependency", "agreement measures calibrate the rubric"),
        edge("JUDGE_AGREEMENT", "EVAL_AUDIT", "dependency", "audit uses agreement and rubric outcomes"),
        edge("TRUST_SCORE", "GAMING_DEFENSE", "dependency", "gaming defenses constrain trust-score use"),
        edge("GAMING_DEFENSE", "SHIELDING_RULES", "dependency", "shielding rules are a gaming defense"),
        edge("EVAL_AUDIT", "FEEDBACK_LOOP", "dependency", "audit drives feedback"),
        edge("ROUTING_POLICY", "DECISION_LOG", "dependency", "decision logging follows routing"),
        edge("ESCALATION_GATE", "DECISION_LOG", "sequence", "escalation decisions must be logged"),
        edge("PAIRWISE_JUDGING", "JUDGE_AGREEMENT", "sequence", "pairwise outcomes feed agreement"),
        edge("TASK_FAMILY_MODEL", "ROUTING_POLICY", "causal", "task-family model informs routing"),
        edge("DRIFT_MONITOR", "ESCALATION_GATE", "causal", "drift can trigger escalation"),
        edge("DRIFT_MONITOR", "FEEDBACK_LOOP", "feedback", "drift observations feed corrective updates"),
        edge("ARTIFACT_RUBRIC", "SCORING_CALIBRATION", "constraint", "calibration is constrained by the rubric"),
        edge("TRUST_SCORE", "DECISION_LOG", "constraint", "log entries need the score inputs"),
        edge("GAMING_DEFENSE", "DECISION_LOG", "similarity", "defense choices should be visible in logs"),
        edge("EVAL_AUDIT", "ROUTING_POLICY", "feedback", "audit findings may change routing policy"),
        edge("SCORING_CALIBRATION", "TRUST_SCORE", "causal", "calibrated scores feed trust"),
        edge("CAPABILITY_MAPPER", "EXPLORATION_POLICY", "causal", "capability map shapes exploration"),
        edge("TASK_FAMILY_MODEL", "EXPLORATION_POLICY", "constraint", "task family knowledge constrains exploration"),
        edge("SHIELDING_RULES", "ARTIFACT_RUBRIC", "conflict", "shielding vs transparency is tensioned, not eliminated", tension=-0.2, rule="prefer score integrity; reveal enough for audit without exposing the full gaming surface"),
        edge("GOLD_SET_ANCHOR", "DRIFT_MONITOR", "conflict", "a stable gold set can lag reality", tension=-0.18, rule="if gold-set and live distribution diverge, refresh the anchor set before changing thresholds"),
        edge("ROUTING_POLICY", "EVAL_AUDIT", "causal", "routing outcomes are audited"),
        edge("DECISION_LOG", "FEEDBACK_LOOP", "sequence", "logged outcomes feed iterative revision"),
    ]

    cqs = []
    for i in range(1, 15):
        cqs.append({
            "id": f"CQ_{i:02d}",
            "question": q_prompts[i - 1],
            "must_be_answerable_from": ["nodes", "workflow"],
            "acceptance_condition": "answer is recoverable from the nodes and workflow",
            "covered_by": [],
        })
    # fix coverage to include explicit node sets
    coverage = {
        1: ["AGENT_SCORE_INTAKE"],
        2: ["ARTIFACT_RUBRIC", "PAIRWISE_JUDGING"],
        3: ["AGENT_HISTORY", "TASK_FAMILY_MODEL"],
        4: ["TRUST_SCORE"],
        5: ["ROUTING_POLICY", "ESCALATION_GATE"],
        6: ["POSITION_BIAS_CONTROL"],
        7: ["GOLD_SET_ANCHOR", "SCORING_CALIBRATION"],
        8: ["DRIFT_MONITOR", "EVAL_AUDIT"],
        9: ["GAMING_DEFENSE", "SHIELDING_RULES"],
        10: ["CAPABILITY_MAPPER", "TASK_FAMILY_MODEL"],
        11: ["EXPLORATION_POLICY", "ROUTING_POLICY"],
        12: ["ESCALATION_GATE", "DECISION_LOG"],
        13: ["JUDGE_AGREEMENT", "SCORING_CALIBRATION"],
        14: ["FEEDBACK_LOOP", "EVAL_AUDIT"],
    }
    for cq in cqs:
        idx = int(cq["id"].split("_")[1])
        cq["covered_by"] = coverage[idx]
        cq["acceptance_condition"] = f"{cq['covered_by'][0]} and related nodes answer the question"

    return {
        "domain": domain,
        "domain_label": label,
        "purpose": purpose,
        "empirical_status": "heuristic_prior_not_observed_dataset",
        "assumptions": [
            "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
            "scoring should be auditable and defensive against gaming",
            "task families and human accept labels are available for calibration",
        ],
        "exclusions": [
            "production code for model training or deployment",
            "live data collection outside the scoring loop",
            "undocumented hidden metrics",
        ],
        "schema_version": "1.3",
        "generation_metadata": {
            "mode": "KB_GENERATION",
            "generator_prompt_version": "1.4.1",
            "kb_schema_version": "1.3",
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
            "kb_schema_version": "1.3",
            "compatibility_target": "v1.3-style KB objects",
        },
        "input_summary": {
            "domain_supplied": True,
            "domain_context_supplied": True,
            "purpose_hint_supplied": True,
            "observed_data_supplied": False,
            "expert_judgment_supplied": False,
            "experimental_or_benchmark_evidence_supplied": False,
            "source_material_supplied": False,
            "density_mode": "dense",
            "human_review_policy_supplied": False,
            "section_text_supplied": False,
            "existing_schema_supplied": False,
            "evaluation_criteria_supplied": False,
            "risk_tolerance_supplied": False,
            "stakeholder_context_supplied": False,
            "version_baseline_supplied": False,
        },
        "competency_questions": cqs,
        "glossary": [
            {"term": "trust score", "definition": "a composite reliability score built from pass rates, accepts, revisions, and drift"},
            {"term": "gaming defense", "definition": "a design choice that prevents the score from being optimized without improving the work"},
            {"term": "task family", "definition": "a family of comparable tasks used to score competence at a stable grain"},
        ],
        "source_description": "heuristic prior estimates for multi-agent evaluation and routing, grounded in classical evaluation, calibration, and Goodhart-resistance literature",
        "source_citation": source_citation,
        "nodes": nodes,
        "edges": edges,
        "conflict_axes": [
            {"id": "CA_01", "name": "absolute_score_vs_pairwise_comparison", "description": "single scores are compact; blind pairwise is more robust"},
            {"id": "CA_02", "name": "transparency_vs_gaming_resistance", "description": "more exposure helps audits but also helps adversaries"},
            {"id": "CA_03", "name": "exploitation_vs_exploration", "description": "optimal routing must still let recovering agents re-enter"},
            {"id": "CA_04", "name": "static_thresholds_vs_drift", "description": "fixed gates are simple but can go stale"},
            {"id": "CA_05", "name": "human_accept_rate_vs_gate_pass_rate", "description": "human accept can diverge from automated gate success"},
            {"id": "CA_06", "name": "capability_match_vs_breadth_training", "description": "specialization beats averaging, but generalization remains useful"},
            {"id": "CA_07", "name": "auditability_vs_latency", "description": "more logging and checks cost time"},
            {"id": "CA_08", "name": "calibration_vs_rank_preservation", "description": "better calibration may reorder agents"},
            {"id": "CA_09", "name": "stable_gold_set_vs_live_distribution", "description": "anchors can lag changing task mixtures"},
        ],
        "edge_cases": [
            {"id": "EC_01", "description": "judge sees the same answer twice and order effects contaminate scores", "affected_nodes": ["POSITION_BIAS_CONTROL", "GOLD_SET_ANCHOR"]},
            {"id": "EC_02", "description": "an agent improves benchmark scores by exploiting the rubric", "affected_nodes": ["GAMING_DEFENSE", "SHIELDING_RULES"]},
            {"id": "EC_03", "description": "task mix shifts and historical averages stop predicting reliability", "affected_nodes": ["AGENT_HISTORY", "DRIFT_MONITOR"]},
            {"id": "EC_04", "description": "routing over-exploits the top agent and starves recovery", "affected_nodes": ["ROUTING_POLICY", "EXPLORATION_POLICY"]},
            {"id": "EC_05", "description": "high-stakes work should bypass automated assignment", "affected_nodes": ["ESCALATION_GATE", "DECISION_LOG"]},
            {"id": "EC_06", "description": "pairwise judgments disagree with absolute scores", "affected_nodes": ["PAIRWISE_JUDGING", "SCORING_CALIBRATION"]},
            {"id": "EC_07", "description": "the gold set becomes stale after a policy change", "affected_nodes": ["GOLD_SET_ANCHOR", "DRIFT_MONITOR"]},
            {"id": "EC_08", "description": "audit findings require a rubric rewrite", "affected_nodes": ["EVAL_AUDIT", "FEEDBACK_LOOP"]},
            {"id": "EC_09", "description": "capability mapping and routing disagree on the best agent", "affected_nodes": ["CAPABILITY_MAPPER", "TASK_FAMILY_MODEL"]},
            {"id": "EC_10", "description": "a score is missing confidence context and should not be surfaced raw", "affected_nodes": ["TRUST_SCORE", "DECISION_LOG"]},
            {"id": "EC_11", "description": "human accept rate diverges from gate pass rate", "affected_nodes": ["TRUST_SCORE", "ROUTING_POLICY"]},
        ],
        "workflow": [
            {"step": "intake the scoring grain and comparison set", "node_ref": "AGENT_SCORE_INTAKE"},
            {"step": "design the rubric and its bias controls", "node_ref": "ARTIFACT_RUBRIC"},
            {"step": "calibrate against a gold set and judge agreement", "node_ref": "GOLD_SET_ANCHOR"},
            {"step": "assemble the agent-history and trust-score model", "node_ref": "TRUST_SCORE"},
            {"step": "route tasks by capability, trust, and exploration policy", "node_ref": "ROUTING_POLICY"},
            {"step": "escalate low-confidence or high-stakes cases", "node_ref": "ESCALATION_GATE"},
            {"step": "audit for drift and gaming", "node_ref": "EVAL_AUDIT"},
            {"step": "feed fixes back into the scoring loop", "node_ref": "FEEDBACK_LOOP"},
            {"step": "record the decision rationale", "node_ref": "DECISION_LOG"},
        ],
        "dominance_rules": [
            {"title": "gaming resistance", "rule": "gaming resistance dominates cosmetic score simplicity"},
            {"title": "auditability", "rule": "auditability dominates hidden heuristics"},
            {"title": "capability match", "rule": "capability match dominates global average trust when the task family is known"},
            {"title": "human escalation", "rule": "human escalation dominates automatic assignment when confidence is low and stakes are high"},
            {"title": "drift response", "rule": "drift response dominates historical inertia when the task distribution moves"},
            {"title": "pairwise comparison", "rule": "pairwise comparison dominates absolute scoring when judge bias is hard to control"},
            {"title": "calibration", "rule": "calibration dominates raw ranking when score interpretation matters"},
            {"title": "exploration budget", "rule": "exploration budget dominates starvation of recovering agents"},
        ],
        "anti_rework_rules": [
            {"title": "no hidden score components", "rule": "do not hide a score component that affects routing"},
            {"title": "refresh anchors", "rule": "do not reuse stale gold-set anchors after distribution drift"},
            {"title": "preserve task families", "rule": "do not collapse task families into one undifferentiated average"},
            {"title": "log inputs", "rule": "do not log only the final decision without the score inputs"},
            {"title": "separate accept from gate", "rule": "do not treat human accept rate as interchangeable with gate pass rate"},
            {"title": "early safety", "rule": "do not escalate after the agent is already clearly unsafe when the gate can block earlier"},
            {"title": "normalize comparisons", "rule": "do not compare agents across incompatible task types without normalization"},
            {"title": "calibration first", "rule": "do not let pairwise judgments overwrite calibration checks"},
        ],
        "iteration_protocol": [
            {"title": "score", "step": "score the artifact or agent with the current rubric", "nodes": ["ARTIFACT_RUBRIC", "TRUST_SCORE"]},
            {"title": "audit", "step": "audit the output for bias, drift, and gaming", "nodes": ["EVAL_AUDIT", "DRIFT_MONITOR"]},
            {"title": "update", "step": "update calibration and trust-score weights if the audit fails", "nodes": ["SCORING_CALIBRATION", "TRUST_SCORE"]},
            {"title": "reroute", "step": "re-run the route decision on the revised score", "nodes": ["ROUTING_POLICY", "ESCALATION_GATE"]},
            {"title": "record", "step": "record the decision rationale and acceptance status", "nodes": ["DECISION_LOG"]},
            {"title": "refresh", "step": "watch for task-family shifts before refreshing the gold set", "nodes": ["TASK_FAMILY_MODEL", "GOLD_SET_ANCHOR"]},
        ],
        "source_citation": source_citation,
    }


def main():
    q1 = [
        "What input artifact is being scored and at what grain?",
        "How is a blind, multi-criterion rubric designed and calibrated?",
        "How is per-agent history tracked across task types and revisions?",
        "How are gate pass rate, human accept rate, and revise count combined into trust?",
        "How does the routing policy use trust and capability to assign work?",
        "How are blind pairwise comparison and order bias controlled?",
        "How is the gold set used to anchor calibration and agreement?",
        "How is drift monitored across time and task mix?",
        "How are Goodhart and score-gaming defenses built in?",
        "How is capability mapped to task families for routing?",
        "How is exploration reserved for recovering agents?",
        "When does a task escalate to human review?",
        "How is inter-judge agreement measured and used?",
        "How do audit findings feed the scoring loop back into revision?",
    ]

    specs = [
        ("kb1.spec.json", build_spec(
            "artifact_quality__evaluation", "Artifact Quality Evaluation", "Design a concrete rubric for judging work products and calibrating them against anchors.",
            "Hou et al. 2019 Learning to Compare; Lambert et al. 2024 rubric calibration and pairwise judging practice; OpenAI Evals/LLM-as-judge cautionary literature",
            q1,
        )),
        ("kb2.spec.json", build_spec(
            "agent_reliability__evaluation", "Agent Reliability Evaluation", "Score agents by task-family competence, drift, revision count, and observed accept behavior.",
            "Goodhart 1975; Mehrabi et al. 2021 bias and drift; Amodei et al. 2016 Concrete Problems in AI Safety; recent agent reliability and calibration discussions",
            q1,
        )),
        ("kb3.spec.json", build_spec(
            "routing_decision__evaluation", "Routing Decision Evaluation", "Convert scores into capability-matched routing, exploration, and escalation policy.",
            "Shah & Shah 2021 exploration-exploitation; Sutton & Barto 2018 reinforcement learning policy tradeoffs; recent multi-agent routing and orchestration evaluations",
            q1,
        )),
    ]

    for rel, spec in specs:
        path = ROOT / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(spec, indent=2))


if __name__ == "__main__":
    main()
