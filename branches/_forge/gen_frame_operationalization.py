#!/usr/bin/env python3
"""Generate the frame__operationalization content spec (B10 question-compiler KB)
for kb_forge.py. Compact authoring: node() applies sane defaults so only domain
content + base metric magnitudes are specified per node.

Domain: turning a vague topic into an answerable, well-scoped question — testing
answerability, structuring with PICO/FINER, mapping construct->indicator->measure,
decomposing into a MECE sub-question DAG, surfacing assumptions, and neutralizing
loaded framing. Neutral/informational; framing for clarity and testability, NOT
persuasion."""
import json, os

SRC = "SRC_HEURISTIC_PRIOR"


def node(id, topic, definition, group, deps, cqs, base,
         scope_in, scope_out, pros, cons, fmodes, accept, revisit,
         inputs=None, outputs=None, specialists=None, contradictors=None,
         subfields=None):
    b = dict(base)
    return {
        "id": id, "topic": topic, "definition": definition, "group": group,
        "node_type": "work_unit",
        "academic_fields": ["question_science", "research_methodology"],
        "subfields": subfields or ["question_framing", "operationalization"],
        "specialists": specialists or ["research_methodologist"],
        "contradictors": contradictors or ["premature_answer_advocate"],
        "inputs": inputs or ["vague topic statement", "asker intent"],
        "outputs": outputs or ["scoped question artifact"],
        "dependencies": deps, "must_not_finalize_before": [],
        "competency_question_refs": cqs, "evidence_refs": [SRC],
        "base": b,
        "scope_boundary": {"included": scope_in, "excluded": scope_out},
        "pros": pros, "cons": cons, "failure_modes": fmodes,
        "acceptance_tests": accept, "revisit_triggers": revisit,
        "handoff_artifact_required": True, "lifecycle_state": "draft",
    }


def b(C, BV, UV, TC, R, X, IR, CF, NCP, AT, DG, PI, EC, FR, DW, ui, sig):
    return {"criticality": C, "business_value": BV, "user_value": UV, "technical_complexity": TC,
            "risk_if_wrong": R, "cross_topic_coupling": X, "irreversibility": IR, "confidence": CF,
            "node_conflict_pressure": NCP, "acceptance_test_pass_rate": AT, "dependency_gate_pass_rate": DG,
            "prior_importance": PI, "evidence_confidence": EC, "failure_rate": FR, "downside_weight": DW,
            "uncertainty_interval": ui, "update_signal": sig}


def pro(claim, ex, w=0.78): return {"weight": w, "claim": claim, "example": ex}
def con(claim, ex, w=0.6): return {"weight": w, "claim": claim, "example": ex}


N = []

N.append(node("TOPIC_INTAKE", "vague_topic_and_intent_capture",
  "Capture the raw, vague topic together with the asker's intent, decision context, and what a useful answer would change, before any structuring is attempted.",
  "foundations", [], ["CQ_01"],
  b(0.9, 0.8, 0.85, 0.45, 0.8, 0.78, 0.55, 0.7, 0.4, 0.78, 0.82, 0.9, 0.7, 0.16, 0.5, [0.2, 0.5], "the asker's intent, decision context, or what-would-change criterion is restated or revised"),
  ["raw topic text", "asker intent and decision context", "what a useful answer would change"],
  ["answer production", "evidence gathering"],
  [pro("Capturing intent and decision context up front anchors every later scoping choice to a real purpose", "topic 'is remote work good?' is paired with intent 'decide our 2027 office policy'")],
  [con("Over-eliciting context can stall a quick lookup that needed no framing at all", "spending an hour on intent for 'what is the capital of France?'")],
  ["intent left implicit so the question drifts from the asker's real need", "decision context conflated with the answer itself"],
  ["the captured record names the topic, the asker's intent, and what a useful answer would change"],
  ["asker's stated purpose changes", "a new decision deadline or stakeholder appears"],
  specialists=["research_methodologist", "requirements_analyst"]))

N.append(node("ANSWERABILITY", "in_principle_answerability_test",
  "Test whether the topic can be turned into a question answerable in principle: does an answer exist, could evidence or reasoning ever settle it, and is it empirical, conceptual, normative, or unanswerable as posed.",
  "foundations", ["TOPIC_INTAKE"], ["CQ_01", "CQ_02"],
  b(0.9, 0.78, 0.8, 0.6, 0.86, 0.74, 0.55, 0.66, 0.45, 0.74, 0.8, 0.88, 0.66, 0.18, 0.55, [0.22, 0.55], "the class of question (empirical/conceptual/normative) or the evidence that could settle it is reassessed"),
  ["existence-of-answer check", "question-class classification", "settleability-by-evidence check"],
  ["the substantive answer", "study design"],
  [pro("Classifying answerability early stops effort on pseudo-questions and routes each class to the right method", "splitting 'is X better?' into an empirical 'does X reduce defects?' and a normative 'should we prefer fewer defects?'")],
  [con("A premature 'unanswerable' verdict can discard a question that is merely under-specified, not unanswerable", "rejecting 'is the system fair?' instead of operationalizing 'fair'")],
  ["a malformed or self-contradictory question is treated as answerable", "an empirical question is answered with pure definition"],
  ["the question is assigned a class and a statement of what evidence or reasoning could in principle settle it"],
  ["the question class is reclassified", "a previously unanswerable item becomes operationalizable"],
  specialists=["research_methodologist", "epistemologist"]))

N.append(node("PICO_FRAME", "pico_peco_empirical_structuring",
  "Structure an empirical question into explicit components — Population/Problem, Intervention or Exposure, Comparator, Outcome (PICO/PECO) — so the question names exactly who, what change, against what baseline, measured on what outcome.",
  "framing", ["ANSWERABILITY"], ["CQ_03"],
  b(0.85, 0.76, 0.78, 0.62, 0.78, 0.78, 0.55, 0.66, 0.5, 0.76, 0.78, 0.84, 0.66, 0.2, 0.5, [0.22, 0.54], "the population, comparator, or outcome of interest changes"),
  ["population/problem identification", "intervention or exposure naming", "comparator and outcome specification"],
  ["effect estimation", "data collection"],
  [pro("Naming a comparator forces the question to be about a contrast, not a vague property", "'does pair programming help?' becomes 'for backend teams, does pairing vs solo coding change defect rate?'")],
  [con("Forcing PICO onto a non-comparative or exploratory question manufactures a contrast that does not belong", "shoehorning 'what are users' frustrations?' into intervention/comparator slots")],
  ["the comparator is left implicit so an effect size is uninterpretable", "outcome named so broadly it cannot be measured"],
  ["each empirical question states an explicit population, contrast, and named outcome"],
  ["the comparator baseline changes", "the outcome of interest is replaced"],
  specialists=["research_methodologist", "evidence_synthesis_analyst"],
  subfields=["evidence_based_practice", "pico_framing"]))

N.append(node("FINER_CRITERIA", "finer_question_screen",
  "Screen a candidate question against FINER — Feasible, Interesting, Novel, Ethical, Relevant — to decide whether it is worth and safe to pursue before resources are committed.",
  "framing", ["ANSWERABILITY"], ["CQ_03", "CQ_04"],
  b(0.78, 0.8, 0.72, 0.55, 0.76, 0.7, 0.55, 0.68, 0.45, 0.78, 0.78, 0.8, 0.68, 0.18, 0.48, [0.2, 0.5], "feasibility, ethics, or relevance assumptions about the question change"),
  ["feasibility check (data, time, access)", "novelty and relevance screen", "ethical admissibility screen"],
  ["statistical analysis", "the answer itself"],
  [pro("An ethics-and-feasibility screen kills unanswerable-in-practice or harmful questions before they consume effort", "dropping a question whose only data source would breach consent")],
  [con("FINER's 'novel' and 'interesting' criteria are subjective and can suppress a worthwhile replication", "rejecting a needed confirmation study as 'not novel'")],
  ["an infeasible question is pursued and stalls for lack of data", "a relevant question is dropped only for failing 'novel'"],
  ["the question passes a feasibility, ethics, and relevance screen, or is explicitly revised to pass"],
  ["data access or feasibility changes", "an ethical concern is raised about the question"],
  specialists=["research_methodologist", "ethics_reviewer"],
  subfields=["clinical_research_design", "feasibility_screening"]))

N.append(node("CONSTRUCT_DEF", "construct_precise_definition",
  "Define each construct the question depends on precisely: state what the concept includes and excludes, its boundary cases, and the theory of the concept, so the question is not about an undefined word.",
  "operationalization", ["PICO_FRAME"], ["CQ_05"],
  b(0.9, 0.78, 0.78, 0.66, 0.86, 0.82, 0.62, 0.6, 0.55, 0.72, 0.76, 0.9, 0.6, 0.22, 0.58, [0.26, 0.62], "the conceptual definition, inclusion/exclusion boundary, or theory of a construct changes"),
  ["intensional definition", "inclusion/exclusion boundary", "boundary and edge cases of the concept"],
  ["choosing the measure", "data collection"],
  [pro("A precise construct definition is the anchor of construct validity: it fixes what the later indicator must capture", "defining 'engagement' as sustained voluntary attention, excluding mere clicks")],
  [con("Long definitional debate can over-theorize a construct that a single agreed indicator would have settled", "weeks defining 'productivity' for a question a throughput count would answer")],
  ["the construct is defined by its measure (operationism) instead of independently", "key boundary cases left unclassified"],
  ["each construct has an intensional definition with explicit inclusions, exclusions, and boundary cases"],
  ["the conceptual definition is contested", "a boundary case reveals the definition is incomplete"],
  specialists=["research_methodologist", "measurement_theorist"],
  subfields=["construct_validity", "conceptualization"]))

N.append(node("INDICATOR_MAP", "construct_to_indicator_mapping",
  "Map each defined construct to one or more observable indicators — the empirical signs that stand in for the concept — and state why each indicator is a valid sign of that construct.",
  "operationalization", ["CONSTRUCT_DEF"], ["CQ_05", "CQ_06"],
  b(0.86, 0.76, 0.76, 0.68, 0.84, 0.8, 0.6, 0.6, 0.55, 0.72, 0.76, 0.86, 0.6, 0.22, 0.55, [0.26, 0.6], "the chosen indicator's validity as a sign of the construct is questioned"),
  ["indicator selection", "construct-to-indicator validity argument", "single vs multiple indicator decision"],
  ["measure binding", "scoring rules"],
  [pro("Naming the indicator separately from the construct exposes the inferential leap that construct validity rests on", "construct 'team health' indicated by retention, voluntary-effort, and survey trust")],
  [con("A single convenient indicator can capture only part of a rich construct (construct under-representation)", "using lines of code as the sole indicator of 'developer output'")],
  ["an indicator is chosen for convenience, not validity (surrogate trap)", "indicator captures a different construct than intended"],
  ["each construct maps to at least one indicator with a stated validity rationale"],
  ["the indicator is shown to track a different construct", "construct under-representation is detected"],
  specialists=["research_methodologist", "measurement_theorist"],
  subfields=["construct_validity", "indicator_design"]))

N.append(node("MEASURE_BIND", "indicator_to_measure_binding",
  "Bind each indicator to a concrete measure or metric: the instrument, scale, unit, and scoring rule that produces a value, so the question becomes quantifiable or codable.",
  "operationalization", ["INDICATOR_MAP"], ["CQ_06", "CQ_07"],
  b(0.84, 0.76, 0.76, 0.7, 0.82, 0.78, 0.62, 0.62, 0.5, 0.74, 0.78, 0.84, 0.62, 0.22, 0.52, [0.24, 0.56], "the instrument, scale, unit, or scoring rule for a measure changes"),
  ["instrument and scale choice", "unit and scoring rule", "measurement-level (nominal/ordinal/interval) decision"],
  ["the analysis model", "running the study"],
  [pro("Binding a concrete unit and scoring rule makes the question testable and the answer comparable across cases", "'trust' indicator bound to a validated 7-item Likert scale scored 1-7")],
  [con("A ready-made instrument can drag in its own construct, displacing the one you defined (jingle/jangle)", "adopting an 'engagement' survey that actually measures satisfaction")],
  ["the measure has unstated units so values are not comparable", "scoring rule introduces a construct-irrelevant bias"],
  ["each indicator has a measure with a defined instrument, unit, and scoring rule"],
  ["a validated instrument's construct is found to mismatch the defined construct", "the measurement level forces an invalid analysis"],
  specialists=["research_methodologist", "psychometrician"],
  subfields=["measurement", "operational_definition"]))

N.append(node("SCOPE_BOUND", "inclusion_exclusion_scoping",
  "Set the question's boundary: the inclusion and exclusion criteria over population, time window, setting, and unit of analysis, so what counts as in-scope and out-of-scope is explicit.",
  "scoping", ["PICO_FRAME"], ["CQ_08"],
  b(0.85, 0.76, 0.78, 0.6, 0.82, 0.8, 0.66, 0.64, 0.55, 0.76, 0.78, 0.85, 0.64, 0.2, 0.55, [0.22, 0.54], "the inclusion/exclusion boundary, time window, or unit of analysis changes"),
  ["inclusion and exclusion criteria", "time window and setting bounds", "unit-of-analysis fixing"],
  ["sub-question generation", "answer synthesis"],
  [pro("Explicit inclusion/exclusion criteria make the answer's domain of validity legible and prevent silent scope creep", "scoping 'remote work effects' to knowledge workers, 2020-2025, individual-level outcomes")],
  [con("Narrow scoping for tractability can exclude exactly the cases that would generalize the answer", "studying only one company so the result cannot transfer")],
  ["unbounded scope makes the question unanswerable in practice", "boundary excludes the cases the asker actually cared about"],
  ["the question states inclusion and exclusion criteria over population, time, setting, and unit of analysis"],
  ["scope is found too narrow to serve the asker's intent", "an out-of-scope case turns out to be decision-relevant"],
  specialists=["research_methodologist", "scoping_analyst"]))

N.append(node("ISSUE_TREE", "mece_issue_tree",
  "Build a MECE issue tree (Minto pyramid): break the top question into branches that are mutually exclusive and collectively exhaustive, so the decomposition has no gaps and no overlaps.",
  "decomposition", ["SCOPE_BOUND"], ["CQ_09"],
  b(0.86, 0.78, 0.76, 0.68, 0.82, 0.85, 0.6, 0.6, 0.6, 0.72, 0.76, 0.86, 0.6, 0.22, 0.55, [0.26, 0.62], "the decomposition principle, branch set, or MECE partition of the question changes"),
  ["top-down question breakdown", "MECE branch partition", "gap-and-overlap audit"],
  ["sub-question sequencing", "answering branches"],
  [pro("A MECE partition guarantees the sub-questions cover the parent with no double-counting and no missing branch", "splitting 'why did revenue fall?' into price, volume, and mix — exhaustive and exclusive")],
  [con("Strict MECE can force an artificial partition on a domain whose real drivers genuinely overlap", "forcing exclusive buckets on causes that interact and reinforce")],
  ["overlapping branches double-count an effect", "a missing branch leaves part of the question unaddressed"],
  ["the issue tree's branches are collectively exhaustive of the parent and mutually exclusive"],
  ["a gap or overlap is found in the partition", "the decomposition principle is reconsidered"],
  specialists=["research_methodologist", "structured_problem_solver"],
  subfields=["issue_trees", "mece_decomposition"]))

N.append(node("SUBQ_DAG", "sub_question_dependency_dag",
  "Turn the issue tree into a sub-question DAG: order sub-questions by their answer-dependencies so questions whose answers feed others come first, and the structure stays acyclic.",
  "decomposition", ["ISSUE_TREE"], ["CQ_09", "CQ_10"],
  b(0.84, 0.76, 0.74, 0.7, 0.8, 0.82, 0.6, 0.6, 0.58, 0.72, 0.76, 0.84, 0.6, 0.22, 0.52, [0.26, 0.6], "the answer-dependency relations among sub-questions change"),
  ["answer-dependency edges", "acyclicity check", "topological ordering of sub-questions"],
  ["prioritization weighting", "answer production"],
  [pro("Encoding answer-dependencies as a DAG reveals which sub-questions are prerequisites and can be answered first", "'what is the baseline?' must be answered before 'did the intervention move it?'")],
  [con("Over-specifying dependencies can serialize sub-questions that were actually independent and parallelizable", "forcing an order on two unrelated branches")],
  ["a circular dependency makes the order ill-defined", "a hidden prerequisite is missed so a sub-question is answered too early"],
  ["the sub-question graph is acyclic and every dependency edge reflects a real answer prerequisite"],
  ["a dependency cycle appears", "an assumed prerequisite is found to be spurious"],
  specialists=["research_methodologist", "decomposition_engineer"],
  subfields=["question_decomposition", "dependency_modeling"]))

N.append(node("PRIORITIZE_SUBQ", "sub_question_prioritization",
  "Prioritize sub-questions by expected value of an answer against feasibility and the asker's decision, so limited effort is spent on the sub-questions that most change the decision.",
  "decomposition", ["SUBQ_DAG"], ["CQ_10"],
  b(0.8, 0.82, 0.74, 0.6, 0.76, 0.74, 0.55, 0.64, 0.5, 0.76, 0.76, 0.82, 0.64, 0.2, 0.5, [0.22, 0.52], "the asker's decision weighting or the value/feasibility of a sub-question changes"),
  ["value-of-answer estimation", "feasibility weighting", "decision-relevance ranking"],
  ["answering", "construct definition"],
  [pro("Ranking by value-of-information focuses effort on the sub-questions whose answers would actually change the decision", "answering the pivotal cost driver before low-leverage detail questions")],
  [con("Value-of-information estimates are themselves uncertain and can deprioritize a sub-question that proves pivotal", "skipping a 'minor' branch that later dominates the result")],
  ["effort spent on a sub-question whose answer changes nothing", "a pivotal sub-question deprioritized on a wrong value estimate"],
  ["sub-questions are ranked by a stated value-and-feasibility rationale tied to the asker's decision"],
  ["the decision weighting changes", "a deprioritized sub-question is found to be pivotal"]))

N.append(node("GRAIN_LEVEL", "granularity_and_abstraction_level",
  "Choose the granularity and abstraction level the question operates at: how fine the population, time, and outcome resolution should be, matched to the decision and the available evidence.",
  "scoping", ["SCOPE_BOUND"], ["CQ_08", "CQ_11"],
  b(0.78, 0.72, 0.72, 0.62, 0.74, 0.76, 0.55, 0.64, 0.5, 0.74, 0.76, 0.78, 0.64, 0.2, 0.48, [0.24, 0.54], "the required resolution of population, time, or outcome changes"),
  ["resolution of population/time/outcome", "abstraction-level selection", "aggregation vs disaggregation choice"],
  ["measure binding", "answering"],
  [pro("Matching grain to the decision avoids asking a question finer or coarser than the evidence or decision needs", "asking quarterly not daily churn when the decision is an annual budget")],
  [con("A too-coarse grain can hide the very heterogeneity that would answer the question (ecological fallacy)", "asking national averages when the effect is regional")],
  ["grain too fine for available evidence makes the question unanswerable", "grain too coarse hides decision-relevant variation"],
  ["the question's granularity is fixed and justified against the decision and available evidence"],
  ["evidence resolution changes", "aggregation is found to mask a decision-relevant pattern"]))

N.append(node("AMBIGUITY_RESOLVE", "term_disambiguation",
  "Disambiguate every load-bearing term in the question: resolve polysemy, vague quantifiers, and pronoun/scope ambiguity so the question has exactly one intended reading.",
  "clarification", ["CONSTRUCT_DEF"], ["CQ_12"],
  b(0.82, 0.72, 0.76, 0.58, 0.78, 0.78, 0.55, 0.66, 0.55, 0.76, 0.78, 0.82, 0.66, 0.2, 0.5, [0.2, 0.5], "a term is found to admit more than one intended reading"),
  ["polysemy resolution", "vague-quantifier tightening", "scope and pronoun disambiguation"],
  ["construct theory", "measurement"],
  [pro("Fixing a single intended reading per term prevents two readers from answering two different questions", "replacing 'better' with 'lower 90-day defect rate' so 'better' has one meaning")],
  [con("Aggressive disambiguation can prematurely collapse a productive ambiguity worth keeping open early", "fixing 'success' before the asker has decided which success matters")],
  ["a term silently carries two readings so answers conflict", "a vague quantifier ('most', 'often') left unquantified"],
  ["every load-bearing term has exactly one stated intended reading"],
  ["a term is shown to be read two ways", "a quantifier needs a concrete threshold"],
  specialists=["research_methodologist", "linguist"],
  subfields=["semantics", "disambiguation"]))

N.append(node("ASSUMPTION_SURFACE", "hidden_assumption_surfacing",
  "Surface the hidden assumptions the question rests on: presuppositions, causal premises, and background conditions taken for granted, and mark which must hold for the question to be well-posed.",
  "critique", ["AMBIGUITY_RESOLVE", "ANSWERABILITY"], ["CQ_12", "CQ_13"],
  b(0.86, 0.76, 0.78, 0.62, 0.86, 0.82, 0.6, 0.58, 0.6, 0.7, 0.74, 0.86, 0.58, 0.24, 0.58, [0.28, 0.64], "a presupposition or background condition the question relies on is challenged"),
  ["presupposition extraction", "causal-premise listing", "load-bearing assumption flagging"],
  ["answering", "study execution"],
  [pro("Listing presuppositions exposes loaded questions whose very framing assumes a disputed claim", "'why has quality declined?' presupposes quality declined — surfaced as an assumption to test")],
  [con("Surfacing every conceivable assumption can bury the few load-bearing ones in noise", "a 40-item assumption list nobody can act on")],
  ["a false presupposition makes the question unanswerable (complex-question fallacy)", "a load-bearing assumption left unstated and untested"],
  ["the question's presuppositions are listed and the load-bearing ones are flagged for checking"],
  ["a presupposition is challenged or falsified", "a new background condition is identified"],
  specialists=["research_methodologist", "critical_thinking_analyst"],
  subfields=["presupposition_analysis", "assumption_auditing"]))

N.append(node("FRAMING_EFFECT", "framing_effect_detection",
  "Detect framing effects and loaded wording in the question: gain/loss framing, leading phrasing, presupposed conclusions, and emotionally charged terms that would steer the answer.",
  "critique", ["ASSUMPTION_SURFACE"], ["CQ_13", "CQ_14"],
  b(0.84, 0.74, 0.78, 0.6, 0.85, 0.78, 0.55, 0.58, 0.62, 0.7, 0.74, 0.84, 0.58, 0.24, 0.56, [0.28, 0.62], "the wording is found to carry a gain/loss frame, leading phrasing, or charged term"),
  ["gain/loss frame detection", "leading-phrasing detection", "charged-term and presupposed-conclusion detection"],
  ["rewriting the question", "answering"],
  [pro("Detecting gain/loss framing flags that the same question worded two ways can elicit opposite answers", "'90% survival' vs '10% mortality' shown to shift judgments though logically identical")],
  [con("Hunting for framing in genuinely neutral wording can over-correct and strip useful, decision-relevant emphasis", "flattening a legitimate risk warning into bland phrasing")],
  ["a leading question presupposes its own answer", "a gain/loss frame biases the elicited response"],
  ["the question is scanned and any gain/loss frame, leading phrasing, or charged term is flagged"],
  ["a new framing pattern is detected", "wording is found to steer answers"],
  specialists=["research_methodologist", "behavioral_scientist"],
  subfields=["framing_effects", "cognitive_bias"]))

N.append(node("NEUTRALIZE", "loaded_framing_neutralization",
  "Neutralize detected loaded or leading framing: rewrite to symmetric, presupposition-free, affect-neutral wording while preserving the decision-relevant content, for clarity and testability, not persuasion.",
  "critique", ["FRAMING_EFFECT"], ["CQ_14"],
  b(0.82, 0.74, 0.8, 0.6, 0.82, 0.76, 0.6, 0.6, 0.6, 0.74, 0.76, 0.82, 0.6, 0.22, 0.55, [0.26, 0.6], "the neutral-rewording standard or the line between neutrality and decision-relevant emphasis changes"),
  ["symmetric rewording", "presupposition removal", "affect-neutral phrasing"],
  ["persuasion", "answering"],
  [pro("Rewriting to symmetric, presupposition-free wording makes the question elicit a judgment about the world, not the phrasing", "rewriting 'don't you agree X is failing?' to 'is X meeting target Y?'")],
  [con("Pure neutrality can drop emphasis that a decision genuinely needs, making the question bland and less actionable", "removing all risk salience from a safety-critical question")],
  ["neutralization changes the question's meaning, not just its tone", "residual framing survives the rewrite"],
  ["the reworded question is symmetric and presupposition-free while preserving decision-relevant content"],
  ["a residual frame is found after rewrite", "a stakeholder argues required emphasis was lost"],
  specialists=["research_methodologist", "neutral_phrasing_editor"],
  subfields=["framing_effects", "neutral_phrasing"]))

N.append(node("HYPOTHESIS_FORM", "testable_hypothesis_formulation",
  "Formulate the question's candidate answers as testable hypotheses: a directional or null statement of the expected relation, stated so that evidence could support or contradict it.",
  "testability", ["MEASURE_BIND", "SUBQ_DAG"], ["CQ_07", "CQ_15"],
  b(0.84, 0.76, 0.74, 0.66, 0.82, 0.78, 0.6, 0.62, 0.55, 0.74, 0.78, 0.84, 0.62, 0.22, 0.52, [0.24, 0.56], "the hypothesized relation, direction, or null is revised"),
  ["null and alternative statement", "directional relation specification", "predicted-observation derivation"],
  ["statistical testing", "data collection"],
  [pro("Casting the question as a hypothesis names in advance what observation would confirm or disconfirm it", "'pairing lowers defect rate vs solo' with a null of no difference")],
  [con("Premature hypothesis fixing can bias an exploratory question toward confirming a favored relation", "stating a directional hypothesis before any exploratory look at the data")],
  ["a hypothesis is stated so vaguely no observation could bear on it", "the null is omitted so confirmation is unfalsifiable"],
  ["each empirical sub-question has a hypothesis with a stated null and a predicted observation"],
  ["the hypothesized direction is revised", "exploratory findings reframe the relation"],
  specialists=["research_methodologist", "statistician"],
  subfields=["hypothesis_testing", "study_design"]))

N.append(node("FALSIFIABILITY", "operational_falsifiability_check",
  "Ensure the question's hypotheses are operationally falsifiable: there exists a possible, specified observation that would count as refuting evidence, given the bound measures and scope.",
  "testability", ["HYPOTHESIS_FORM", "MEASURE_BIND"], ["CQ_15", "CQ_16"],
  b(0.88, 0.76, 0.76, 0.64, 0.88, 0.8, 0.62, 0.6, 0.58, 0.72, 0.76, 0.88, 0.6, 0.24, 0.6, [0.28, 0.64], "the set of observations that would count as refuting evidence changes"),
  ["refuting-observation specification", "unfalsifiable-claim detection", "operational testability check"],
  ["running the test", "interpretation"],
  [pro("Requiring a specified refuting observation (Popper) screens out unfalsifiable questions dressed as empirical", "stating exactly what defect-rate change would refute 'pairing helps'")],
  [con("A strict falsifiability bar can reject legitimate questions that are only probabilistically or indirectly testable", "dismissing a question answerable by accumulation of indirect evidence")],
  ["a hypothesis is compatible with every possible outcome (unfalsifiable)", "the refuting observation is unobservable in practice"],
  ["each hypothesis names at least one possible, observable result that would refute it within the bound scope"],
  ["a hypothesis is shown to exclude no observation", "the specified refuter turns out unobservable"],
  specialists=["research_methodologist", "philosopher_of_science"],
  subfields=["falsifiability", "demarcation"]))

N.append(node("ANSWER_TYPE_SPEC", "expected_answer_type_specification",
  "Specify the expected answer's type and shape: whether the answer is a number, an interval, a ranked list, a yes/no with confidence, or a causal claim, plus its units and the form of acceptable evidence.",
  "testability", ["FALSIFIABILITY", "PRIORITIZE_SUBQ"], ["CQ_16", "CQ_17"],
  b(0.8, 0.78, 0.8, 0.58, 0.78, 0.74, 0.58, 0.66, 0.5, 0.78, 0.78, 0.8, 0.66, 0.18, 0.48, [0.2, 0.5], "the expected answer type, units, or acceptable evidence form changes"),
  ["answer-type declaration", "answer units and shape", "acceptable-evidence form"],
  ["producing the answer", "evidence gathering"],
  [pro("Declaring the answer's type and units up front makes 'a good answer' a checkable target before any work", "specifying the answer as a point estimate with a 95% interval in defects-per-KLOC")],
  [con("Fixing the answer type too rigidly can preclude a more informative answer shape the evidence supports", "demanding a yes/no when the evidence supports a nuanced conditional")],
  ["an answer is produced in a shape the asker cannot use", "answer units left unspecified so the result is ambiguous"],
  ["the question declares the expected answer's type, units, and the form of acceptable evidence"],
  ["the asker needs a different answer shape", "the evidence supports a richer answer type"],
  specialists=["research_methodologist", "decision_analyst"],
  subfields=["answer_specification", "decision_analysis"]))

N.append(node("REFRAME_LOOP", "iterative_reframing_loop",
  "Iteratively reframe the question as new information arrives: when an answer, an assumption check, or a feasibility finding changes the picture, revise scope, constructs, or wording and re-run the affected checks.",
  "control", ["ANSWER_TYPE_SPEC", "NEUTRALIZE", "ASSUMPTION_SURFACE"], ["CQ_17", "CQ_18"],
  b(0.86, 0.76, 0.8, 0.62, 0.84, 0.85, 0.55, 0.58, 0.6, 0.72, 0.74, 0.86, 0.58, 0.24, 0.56, [0.28, 0.64], "new evidence, a falsified assumption, or a feasibility finding warrants reframing the question"),
  ["change-driven reframing", "re-running affected checks", "version and rationale tracking"],
  ["unbounded re-asking", "answer production"],
  [pro("A bounded reframing loop lets the question track reality without restarting framing from scratch each time", "a falsified presupposition triggers a scope edit and a re-check of only the affected sub-questions")],
  [con("An unbounded reframing loop can chase a moving target and never let the question stabilize enough to answer", "endlessly re-scoping so no answer is ever attempted")],
  ["reframing churns without converging on a stable question", "a change is made without re-running the checks it invalidates"],
  ["reframing converges to a stable, re-validated question or halts with an explicit open-question rationale"],
  ["new evidence invalidates the current framing", "a feasibility finding forces a scope change"],
  specialists=["research_methodologist", "iteration_lead"],
  subfields=["iterative_inquiry", "question_versioning"]))


CQ = [
 ("CQ_01", "How is a vague topic captured with its intent, and is the resulting question answerable in principle and assigned a class (empirical/conceptual/normative)?", ["nodes", "glossary"], "TOPIC_INTAKE captures topic and intent; ANSWERABILITY tests in-principle answerability and assigns the question class", ["TOPIC_INTAKE", "ANSWERABILITY"]),
 ("CQ_02", "How is an empirical question structured into PICO/PECO components and screened for feasibility, ethics, novelty, and relevance?", ["nodes"], "PICO_FRAME structures population/intervention/comparator/outcome; FINER_CRITERIA screens it before commitment", ["PICO_FRAME", "FINER_CRITERIA"]),
 ("CQ_03", "How is a construct defined precisely and mapped to observable indicators?", ["nodes", "glossary"], "CONSTRUCT_DEF fixes inclusions/exclusions independent of measure; INDICATOR_MAP maps it to valid indicators", ["CONSTRUCT_DEF", "INDICATOR_MAP"]),
 ("CQ_04", "How does an indicator become a concrete measure with a unit and scoring rule, and a hypothesis with named units?", ["nodes"], "MEASURE_BIND binds each indicator to an instrument, unit, and scoring rule; HYPOTHESIS_FORM states the testable relation", ["MEASURE_BIND", "HYPOTHESIS_FORM"]),
 ("CQ_05", "How is the question's scope and granularity bounded by inclusion/exclusion criteria and resolution?", ["nodes"], "SCOPE_BOUND sets inclusion/exclusion and time window; GRAIN_LEVEL fixes the abstraction level", ["SCOPE_BOUND", "GRAIN_LEVEL"]),
 ("CQ_06", "How is the question decomposed into a MECE issue tree and an acyclic, dependency-ordered sub-question DAG?", ["nodes", "edges"], "ISSUE_TREE builds the MECE partition; SUBQ_DAG orders sub-questions by answer-dependencies and stays acyclic", ["ISSUE_TREE", "SUBQ_DAG"]),
 ("CQ_07", "How are sub-questions prioritized so effort lands where it most changes the decision?", ["nodes", "workflow"], "PRIORITIZE_SUBQ ranks sub-questions by value-of-answer against feasibility and the asker's decision", ["PRIORITIZE_SUBQ", "SUBQ_DAG"]),
 ("CQ_08", "How are ambiguous terms disambiguated and hidden assumptions surfaced?", ["nodes", "glossary"], "AMBIGUITY_RESOLVE fixes a single reading per term; ASSUMPTION_SURFACE lists and flags load-bearing presuppositions", ["AMBIGUITY_RESOLVE", "ASSUMPTION_SURFACE"]),
 ("CQ_09", "How are loaded presuppositions and framing effects detected in the wording?", ["nodes", "conflict_axes"], "ASSUMPTION_SURFACE flags presuppositions; FRAMING_EFFECT detects gain/loss and leading framing", ["ASSUMPTION_SURFACE", "FRAMING_EFFECT"]),
 ("CQ_10", "How is loaded or leading framing neutralized without losing decision-relevant content?", ["nodes", "workflow"], "FRAMING_EFFECT detects the frame; NEUTRALIZE rewrites to symmetric, presupposition-free wording while preserving decision-relevant content", ["FRAMING_EFFECT", "NEUTRALIZE"]),
 ("CQ_11", "How is a hypothesis formed and shown to be operationally falsifiable?", ["nodes"], "HYPOTHESIS_FORM states null/alternative; FALSIFIABILITY names a possible refuting observation within the bound scope", ["HYPOTHESIS_FORM", "FALSIFIABILITY"]),
 ("CQ_12", "How is the expected answer's type, shape, units, and acceptable evidence specified?", ["nodes"], "FALSIFIABILITY fixes refuting observations; ANSWER_TYPE_SPEC declares the answer type, units, and acceptable evidence", ["FALSIFIABILITY", "ANSWER_TYPE_SPEC"]),
 ("CQ_13", "How does the question reframe iteratively as new information arrives, and when does it stabilize?", ["nodes", "iteration_protocol"], "ANSWER_TYPE_SPEC sets the target; REFRAME_LOOP revises and re-validates only on material change until the question is stable", ["ANSWER_TYPE_SPEC", "REFRAME_LOOP"]),
 ("CQ_14", "How is the final scoped question verified as answerable, neutral, falsifiable, and decision-relevant?", ["nodes", "workflow"], "REFRAME_LOOP closes the loop only when the question is re-validated against ANSWERABILITY and the framing checks", ["REFRAME_LOOP", "ANSWERABILITY"]),
]
CQS = [{"id": i, "question": q, "must_be_answerable_from": m, "acceptance_condition": a, "covered_by": c} for (i, q, m, a, c) in CQ]

# consolidate node->CQ references onto the 14-CQ set; every CQ covered by >=1 node, each node refs 1-2
CQ_MAP = {
 "TOPIC_INTAKE": ["CQ_01"], "ANSWERABILITY": ["CQ_01", "CQ_14"], "PICO_FRAME": ["CQ_02"],
 "FINER_CRITERIA": ["CQ_02"], "CONSTRUCT_DEF": ["CQ_03"], "INDICATOR_MAP": ["CQ_03"],
 "MEASURE_BIND": ["CQ_04"], "SCOPE_BOUND": ["CQ_05"], "ISSUE_TREE": ["CQ_06"],
 "SUBQ_DAG": ["CQ_06", "CQ_07"], "PRIORITIZE_SUBQ": ["CQ_07"], "GRAIN_LEVEL": ["CQ_05"],
 "AMBIGUITY_RESOLVE": ["CQ_08"], "ASSUMPTION_SURFACE": ["CQ_08", "CQ_09"], "FRAMING_EFFECT": ["CQ_09", "CQ_10"],
 "NEUTRALIZE": ["CQ_10"], "HYPOTHESIS_FORM": ["CQ_04", "CQ_11"], "FALSIFIABILITY": ["CQ_11", "CQ_12"],
 "ANSWER_TYPE_SPEC": ["CQ_12", "CQ_13"], "REFRAME_LOOP": ["CQ_13", "CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

GL = [
 ("answerability", "the property that a question could in principle be settled by evidence or reasoning, given an existing answer", ["settleability"], ["unanswerable_question"], ["ANSWERABILITY", "TOPIC_INTAKE"]),
 ("construct", "an abstract concept (e.g. trust, productivity) that a question depends on and that must be defined before it can be measured", ["latent_concept", "theoretical_construct"], ["indicator"], ["CONSTRUCT_DEF", "INDICATOR_MAP"]),
 ("indicator", "an observable sign taken to stand in for an unobservable construct", ["proxy", "observable_sign"], ["measure"], ["INDICATOR_MAP", "MEASURE_BIND"]),
 ("measure", "the concrete instrument, scale, unit, and scoring rule that turns an indicator into a value", ["metric", "operationalization"], ["construct"], ["MEASURE_BIND", "HYPOTHESIS_FORM"]),
 ("pico", "a structuring of an empirical question into Population, Intervention/Exposure, Comparator, and Outcome", ["peco"], ["finer"], ["PICO_FRAME"]),
 ("finer", "a screen of a question for being Feasible, Interesting, Novel, Ethical, and Relevant", ["finer_screen"], ["pico"], ["FINER_CRITERIA"]),
 ("mece", "a partition that is Mutually Exclusive and Collectively Exhaustive, with no overlap and no gap", ["mutually_exclusive_collectively_exhaustive"], ["overlapping_partition"], ["ISSUE_TREE", "SUBQ_DAG"]),
 ("framing_effect", "a shift in elicited judgments caused by logically equivalent but differently worded presentations of a question", ["wording_effect", "loss_gain_framing"], ["neutral_framing"], ["FRAMING_EFFECT", "NEUTRALIZE"]),
 ("presupposition", "a claim a question takes for granted, such that the question is ill-posed if the claim is false", ["loaded_premise", "complex_question"], ["explicit_hypothesis"], ["ASSUMPTION_SURFACE", "FRAMING_EFFECT"]),
 ("falsifiability", "the property that a hypothesis excludes some possible observation, so an observation could refute it", ["refutability", "testability"], ["unfalsifiable_claim"], ["FALSIFIABILITY", "HYPOTHESIS_FORM"]),
]
GLS = [{"term": t, "definition": d, "synonyms": s, "not_same_as": ns, "used_by_nodes": u} for (t, d, s, ns, u) in GL]

# ---- edges ----
E = []


def dep(f, t, rs, cc=0.82, erc=0.28, cp=0.14, why="", ben="", rk="", ex=""):
    E.append({"from": f, "to": t, "edge_type": "dependency", "relation_strength": rs, "signed_tension": 0.0,
              "causal_confidence": cc, "conflict_probability": cp, "expected_rework_cost": erc,
              "why_related": why or f"{t} depends on {f}", "benefit_of_coupling": ben or "ordered prerequisite",
              "risk_of_conflict": rk or "downstream rework if upstream changes", "example": ex or f"{f} finalized before {t}"})


def conf(f, t, rs, st, rule, why, cp=0.5, erc=0.5, cc=0.6):
    E.append({"from": f, "to": t, "edge_type": "conflict", "relation_strength": rs, "signed_tension": st,
              "causal_confidence": cc, "conflict_probability": cp, "expected_rework_cost": erc,
              "resolution_rule": rule, "why_related": why, "benefit_of_coupling": "tension surfaced and resolved by rule",
              "risk_of_conflict": "unmanaged tension degrades question quality", "example": "see resolution_rule"})


def rel(f, t, et, rs, cc=0.7, cp=0.2, erc=0.3, why=""):
    E.append({"from": f, "to": t, "edge_type": et, "relation_strength": rs, "signed_tension": 0.0,
              "causal_confidence": cc, "conflict_probability": cp, "expected_rework_cost": erc,
              "why_related": why or f"{f} {et} {t}", "benefit_of_coupling": "coordinated behavior",
              "risk_of_conflict": "inconsistency if uncoordinated", "example": f"{f}/{t} {et} relation"})


# dependency edges (acyclic, mirror node.dependencies)
dep("TOPIC_INTAKE", "ANSWERABILITY", 0.9)
dep("ANSWERABILITY", "PICO_FRAME", 0.82)
dep("ANSWERABILITY", "FINER_CRITERIA", 0.8)
dep("PICO_FRAME", "CONSTRUCT_DEF", 0.84)
dep("CONSTRUCT_DEF", "INDICATOR_MAP", 0.86)
dep("INDICATOR_MAP", "MEASURE_BIND", 0.85)
dep("PICO_FRAME", "SCOPE_BOUND", 0.8)
dep("SCOPE_BOUND", "ISSUE_TREE", 0.82)
dep("SCOPE_BOUND", "GRAIN_LEVEL", 0.78)
dep("ISSUE_TREE", "SUBQ_DAG", 0.85)
dep("SUBQ_DAG", "PRIORITIZE_SUBQ", 0.8)
dep("CONSTRUCT_DEF", "AMBIGUITY_RESOLVE", 0.78)
dep("AMBIGUITY_RESOLVE", "ASSUMPTION_SURFACE", 0.8)
dep("ANSWERABILITY", "ASSUMPTION_SURFACE", 0.74)
dep("ASSUMPTION_SURFACE", "FRAMING_EFFECT", 0.82)
dep("FRAMING_EFFECT", "NEUTRALIZE", 0.84)
dep("MEASURE_BIND", "HYPOTHESIS_FORM", 0.84)
dep("SUBQ_DAG", "HYPOTHESIS_FORM", 0.74)
dep("HYPOTHESIS_FORM", "FALSIFIABILITY", 0.86)
dep("MEASURE_BIND", "FALSIFIABILITY", 0.76)
dep("FALSIFIABILITY", "ANSWER_TYPE_SPEC", 0.82)
dep("PRIORITIZE_SUBQ", "ANSWER_TYPE_SPEC", 0.72)
dep("ANSWER_TYPE_SPEC", "REFRAME_LOOP", 0.82)
dep("NEUTRALIZE", "REFRAME_LOOP", 0.74)
dep("ASSUMPTION_SURFACE", "REFRAME_LOOP", 0.72)

# cross-cutting non-dependency edges (no cycle risk)
rel("GRAIN_LEVEL", "MEASURE_BIND", "constraint", 0.74, why="the chosen granularity constrains which measures and units are admissible")
rel("FINER_CRITERIA", "SCOPE_BOUND", "constraint", 0.72, why="feasibility findings tighten the inclusion/exclusion scope")
rel("AMBIGUITY_RESOLVE", "ISSUE_TREE", "causal", 0.72, why="single intended readings of terms determine clean MECE branch boundaries")
rel("REFRAME_LOOP", "ANSWERABILITY", "feedback", 0.78, why="reframing re-runs the answerability test on the revised question")
rel("REFRAME_LOOP", "CONSTRUCT_DEF", "feedback", 0.74, why="reframing can revise a construct definition when an assumption is falsified")
rel("FALSIFIABILITY", "ASSUMPTION_SURFACE", "feedback", 0.7, why="an unfalsifiable hypothesis often traces to an unstated presupposition to surface")
rel("PRIORITIZE_SUBQ", "FINER_CRITERIA", "similarity", 0.7, why="sub-question prioritization and FINER both weigh value against feasibility")
rel("NEUTRALIZE", "ANSWER_TYPE_SPEC", "similarity", 0.7, why="both fix the question's final form so the answer target is unambiguous")

# conflict edges (negative signed_tension + resolution_rule) — real tensions
conf("SCOPE_BOUND", "ISSUE_TREE", 0.7, -0.55,
  "decompose only within the bounded scope; when a MECE branch needs out-of-scope cases, widen scope explicitly via REFRAME_LOOP rather than letting the tree pull scope open silently",
  "precise narrow scoping conflicts with collectively-exhaustive coverage in the issue tree")
conf("MEASURE_BIND", "CONSTRUCT_DEF", 0.7, -0.55,
  "let the construct definition lead: bind a measure only after the construct is defined, and if a ready instrument implies a different construct, revise the definition deliberately rather than letting the metric redefine the concept",
  "early operationalization (picking a convenient measure) conflicts with construct validity")
conf("ISSUE_TREE", "PICO_FRAME", 0.66, -0.45,
  "treat MECE as a target, not a dogma: when real drivers overlap, allow labelled overlap with explicit double-count handling rather than forcing an artificial exclusive partition",
  "strict MECE exclusivity conflicts with natural overlap among real-world drivers of an outcome")
conf("NEUTRALIZE", "PRIORITIZE_SUBQ", 0.66, -0.5,
  "neutralize loaded affect and presupposition while preserving decision-relevant emphasis the asker needs; remove framing that steers the answer, keep salience that the decision legitimately requires",
  "neutral framing conflicts with the decision-relevant emphasis that prioritization wants to preserve")

CA = [
 {"name": "narrow_scope_vs_coverage", "description": "Precise narrow scoping makes the question tractable and its validity domain clear, but can exclude cases needed for a collectively exhaustive answer.", "poles": ["precise_narrow_scope", "exhaustive_coverage"], "resolution_hint": "decompose within scope; widen scope explicitly via the reframing loop when a branch demands it", "tension_score": 0.7, "affected_nodes": ["SCOPE_BOUND", "ISSUE_TREE", "GRAIN_LEVEL"]},
 {"name": "early_operationalization_vs_construct_validity", "description": "Binding a convenient measure early speeds testability but risks letting the metric define the construct, harming construct validity.", "poles": ["early_operationalization", "construct_validity"], "resolution_hint": "define the construct first; revise it deliberately if an instrument implies a different concept", "tension_score": 0.75, "affected_nodes": ["MEASURE_BIND", "CONSTRUCT_DEF", "INDICATOR_MAP"]},
 {"name": "mece_strictness_vs_natural_overlap", "description": "Strict MECE partitions guarantee no gaps or double-counting, but real drivers often overlap and resist exclusive buckets.", "poles": ["strict_mece", "labelled_overlap"], "resolution_hint": "target MECE; allow explicit labelled overlap with double-count handling when drivers truly interact", "tension_score": 0.68, "affected_nodes": ["ISSUE_TREE", "SUBQ_DAG", "PICO_FRAME"]},
 {"name": "neutral_framing_vs_decision_emphasis", "description": "Neutralizing loaded wording protects against framing effects, but pure neutrality can strip emphasis a decision legitimately needs.", "poles": ["neutral_framing", "decision_relevant_emphasis"], "resolution_hint": "remove steering affect and presupposition; preserve salience the decision genuinely requires", "tension_score": 0.66, "affected_nodes": ["NEUTRALIZE", "FRAMING_EFFECT", "PRIORITIZE_SUBQ"]},
 {"name": "answerability_strictness_vs_underspecified_value", "description": "A strict answerability/falsifiability bar screens out pseudo-questions but can reject a valuable question that is merely under-specified.", "poles": ["strict_answerability", "preserve_underspecified"], "resolution_hint": "before rejecting, attempt operationalization; reject only after it provably cannot be made answerable", "tension_score": 0.65, "affected_nodes": ["ANSWERABILITY", "FALSIFIABILITY", "CONSTRUCT_DEF"]},
 {"name": "early_disambiguation_vs_productive_ambiguity", "description": "Fixing one reading per term prevents divergent answers, but premature disambiguation can collapse an ambiguity worth keeping open early.", "poles": ["early_disambiguation", "open_ambiguity"], "resolution_hint": "disambiguate load-bearing terms; defer fixing terms whose right reading depends on findings", "tension_score": 0.6, "affected_nodes": ["AMBIGUITY_RESOLVE", "CONSTRUCT_DEF", "REFRAME_LOOP"]},
 {"name": "single_indicator_simplicity_vs_construct_coverage", "description": "A single indicator is cheap and clear but can under-represent a rich construct; multiple indicators cover more but cost more.", "poles": ["single_indicator", "multi_indicator_coverage"], "resolution_hint": "use multiple indicators when construct under-representation is plausible; otherwise prefer the simplest valid one", "tension_score": 0.58, "affected_nodes": ["INDICATOR_MAP", "MEASURE_BIND", "CONSTRUCT_DEF"]},
 {"name": "fine_grain_vs_evidence_availability", "description": "Fine granularity reveals heterogeneity but may exceed available evidence; coarse grain is supportable but can hide decision-relevant variation.", "poles": ["fine_grain", "coarse_supportable_grain"], "resolution_hint": "match grain to both the decision and the evidence; disaggregate only where data supports it", "tension_score": 0.6, "affected_nodes": ["GRAIN_LEVEL", "SCOPE_BOUND", "MEASURE_BIND"]},
 {"name": "stable_question_vs_continuous_reframing", "description": "Reframing keeps the question true to new information, but unbounded reframing prevents the stability needed to actually answer it.", "poles": ["stabilize_to_answer", "continuous_reframing"], "resolution_hint": "reframe only on material change; bound the loop and re-run only the checks a change invalidates", "tension_score": 0.62, "affected_nodes": ["REFRAME_LOOP", "ANSWER_TYPE_SPEC", "ANSWERABILITY"]},
]

EC = [
 {"description": "A normative or definitional question is treated as empirical and answered with data it cannot settle.", "trigger": "question class not assigned before structuring", "affected_nodes": ["ANSWERABILITY", "PICO_FRAME"], "mitigation": "classify the question (empirical/conceptual/normative) first and route each class to its method", "severity": "high"},
 {"description": "A construct is defined only by its chosen measure, so construct validity collapses (operationism).", "trigger": "measure bound before an independent construct definition exists", "affected_nodes": ["CONSTRUCT_DEF", "MEASURE_BIND", "INDICATOR_MAP"], "mitigation": "define the construct independently; treat the measure as a fallible indicator of it", "severity": "high"},
 {"description": "A single convenient indicator is used for a rich construct, under-representing it (surrogate trap).", "trigger": "one easy indicator chosen for a multi-faceted construct", "affected_nodes": ["INDICATOR_MAP", "MEASURE_BIND"], "mitigation": "use multiple indicators and state each one's validity rationale", "severity": "medium"},
 {"description": "The PICO comparator is left implicit, so any reported effect size is uninterpretable.", "trigger": "intervention named without a baseline comparator", "affected_nodes": ["PICO_FRAME", "HYPOTHESIS_FORM"], "mitigation": "force an explicit comparator into every empirical question", "severity": "high"},
 {"description": "A loaded question presupposes a disputed claim, so it is ill-posed (complex-question fallacy).", "trigger": "presupposition not extracted before framing is accepted", "affected_nodes": ["ASSUMPTION_SURFACE", "FRAMING_EFFECT"], "mitigation": "list presuppositions and convert load-bearing ones into testable sub-questions", "severity": "high"},
 {"description": "Gain/loss or leading wording steers the elicited answer though the question is logically identical.", "trigger": "framing effect not detected before the question is used", "affected_nodes": ["FRAMING_EFFECT", "NEUTRALIZE"], "mitigation": "detect the frame and rewrite to symmetric, presupposition-free wording", "severity": "medium"},
 {"description": "Neutralizing the wording silently changes the question's meaning, not just its tone.", "trigger": "rewrite alters decision-relevant content while removing affect", "affected_nodes": ["NEUTRALIZE", "ANSWER_TYPE_SPEC"], "mitigation": "verify the reworded question preserves the original decision-relevant content", "severity": "medium"},
 {"description": "The sub-question graph contains a circular answer-dependency, so no valid order exists.", "trigger": "answer-dependency edges form a cycle", "affected_nodes": ["SUBQ_DAG", "ISSUE_TREE"], "mitigation": "detect cycles; break them by splitting the mutually-dependent sub-questions", "severity": "high"},
 {"description": "A MECE partition forces exclusive buckets on drivers that genuinely overlap, distorting the answer.", "trigger": "strict exclusivity imposed on interacting drivers", "affected_nodes": ["ISSUE_TREE", "PICO_FRAME"], "mitigation": "allow labelled overlap with explicit double-count handling", "severity": "low"},
 {"description": "A hypothesis is compatible with every possible outcome, so it is unfalsifiable.", "trigger": "no refuting observation specified", "affected_nodes": ["FALSIFIABILITY", "HYPOTHESIS_FORM"], "mitigation": "require at least one possible, observable result that would refute the hypothesis", "severity": "high"},
 {"description": "The scope is left unbounded, making the question impossible to answer in practice.", "trigger": "no inclusion/exclusion or time window set", "affected_nodes": ["SCOPE_BOUND", "GRAIN_LEVEL"], "mitigation": "set explicit inclusion/exclusion criteria, a time window, and a unit of analysis", "severity": "high"},
 {"description": "The reframing loop churns endlessly so the question never stabilizes enough to be answered.", "trigger": "reframing triggered on non-material change without a bound", "affected_nodes": ["REFRAME_LOOP", "ANSWER_TYPE_SPEC"], "mitigation": "reframe only on material change; bound the loop and re-run only invalidated checks", "severity": "medium"},
]

WF = [
 {"action": "capture_topic_and_intent", "node_ref": "TOPIC_INTAKE", "description": "Record the raw topic, the asker's intent and decision context, and what a useful answer would change.", "artifact": "topic_intent_record", "gate": "topic, intent, and what-would-change are captured"},
 {"action": "test_answerability", "node_ref": "ANSWERABILITY", "description": "Assign a question class and state what evidence or reasoning could in principle settle it.", "artifact": "answerability_verdict", "gate": "question class assigned and settleability stated"},
 {"action": "structure_with_pico", "node_ref": "PICO_FRAME", "description": "Structure an empirical question into population, intervention/exposure, comparator, and outcome.", "artifact": "pico_record", "gate": "an explicit comparator and named outcome are present"},
 {"action": "screen_with_finer", "node_ref": "FINER_CRITERIA", "description": "Screen the question for feasibility, ethics, novelty, and relevance before committing resources.", "artifact": "finer_screen_result", "gate": "question passes feasibility, ethics, and relevance or is revised"},
 {"action": "define_construct", "node_ref": "CONSTRUCT_DEF", "description": "Define each construct with explicit inclusions, exclusions, and boundary cases, independent of its measure.", "artifact": "construct_definitions", "gate": "each construct has an intensional definition"},
 {"action": "map_and_bind_measures", "node_ref": "MEASURE_BIND", "description": "Map constructs to valid indicators and bind each indicator to an instrument, unit, and scoring rule.", "artifact": "measurement_plan", "gate": "every indicator has a measure with a defined unit and scoring rule"},
 {"action": "bound_scope_and_grain", "node_ref": "SCOPE_BOUND", "description": "Set inclusion/exclusion criteria, time window, unit of analysis, and the granularity level.", "artifact": "scope_specification", "gate": "inclusion/exclusion criteria and granularity are explicit"},
 {"action": "decompose_to_subquestions", "node_ref": "ISSUE_TREE", "description": "Build a MECE issue tree and convert it into an acyclic, dependency-ordered sub-question DAG.", "artifact": "subquestion_dag", "gate": "branches are MECE and the sub-question graph is acyclic"},
 {"action": "prioritize_subquestions", "node_ref": "PRIORITIZE_SUBQ", "description": "Rank sub-questions by value-of-answer against feasibility and the asker's decision.", "artifact": "prioritized_subquestions", "gate": "sub-questions ranked by a stated value-and-feasibility rationale"},
 {"action": "surface_and_neutralize_framing", "node_ref": "NEUTRALIZE", "description": "Disambiguate terms, surface presuppositions, detect framing effects, and rewrite to neutral wording.", "artifact": "neutralized_question", "gate": "no leading frame or load-bearing presupposition remains unhandled"},
 {"action": "form_and_test_hypotheses", "node_ref": "FALSIFIABILITY", "description": "Formulate testable hypotheses with a null and confirm each names a possible refuting observation.", "artifact": "falsifiable_hypotheses", "gate": "each hypothesis names at least one possible refuting observation"},
 {"action": "specify_answer_and_reframe", "node_ref": "ANSWER_TYPE_SPEC", "description": "Declare the expected answer type, units, and acceptable evidence, then reframe on material change until stable.", "artifact": "scoped_question_spec", "gate": "answer type and units declared; the question is re-validated and stable"},
]

DR = [
 {"rule": "TOPIC_INTAKE intent and ANSWERABILITY class must be fixed before PICO_FRAME structures the question", "rationale": "structure derived from the wrong question class causes full rework", "trigger": "PICO structuring begins without a captured intent and assigned question class", "action": "block structuring until intent and question class are recorded"},
 {"rule": "CONSTRUCT_DEF must precede MEASURE_BIND; a measure may not define its construct", "rationale": "letting the metric define the concept destroys construct validity", "trigger": "a measure is bound before an independent construct definition exists", "action": "block measure binding until the construct is defined independently"},
 {"rule": "ASSUMPTION_SURFACE and FRAMING_EFFECT must run before the question is finalized", "rationale": "loaded presuppositions and framing effects silently bias the answer", "trigger": "a question is finalized without a presupposition and framing scan", "action": "require the assumption and framing scan before finalization"},
 {"rule": "SUBQ_DAG must be acyclic before PRIORITIZE_SUBQ assigns an order", "rationale": "a cyclic dependency has no valid answer order to prioritize", "trigger": "prioritization runs on a graph with a dependency cycle", "action": "reject and route to break the cycle in SUBQ_DAG"},
 {"rule": "HYPOTHESIS_FORM hypotheses must pass FALSIFIABILITY before ANSWER_TYPE_SPEC", "rationale": "an unfalsifiable hypothesis cannot have a meaningful answer type", "trigger": "answer-type specification on an unfalsifiable hypothesis", "action": "block until a refuting observation is specified"},
 {"rule": "SCOPE_BOUND inclusion/exclusion must be set before ISSUE_TREE decomposition", "rationale": "decomposing an unbounded question yields a tree that pulls scope open silently", "trigger": "issue-tree construction with no bounded scope", "action": "require explicit inclusion/exclusion before decomposition"},
 {"rule": "NEUTRALIZE must preserve decision-relevant content while removing steering framing", "rationale": "over-neutralization can strip emphasis the decision needs", "trigger": "a rewrite removes salience the asker's decision requires", "action": "verify decision-relevant content is preserved against TOPIC_INTAKE intent"},
 {"rule": "REFRAME_LOOP may re-open framing only on material change in evidence or assumptions", "rationale": "unbounded reframing prevents the question from ever stabilizing", "trigger": "reframing triggered by a non-material change", "action": "require a material-change justification and bound the loop"},
 {"rule": "GRAIN_LEVEL must be consistent with available evidence before MEASURE_BIND", "rationale": "a grain finer than the evidence makes the question unanswerable in practice", "trigger": "a granularity is chosen finer than the evidence can support", "action": "coarsen the grain or flag the evidence gap before binding measures"},
]

ARR = [
 {"rule": "Do not structure with PICO before the question class is assigned; a class change invalidates the structure", "prevents": "re-structuring after discovering the question was normative, not empirical"},
 {"rule": "Do not bind a measure before the construct is defined; a definition change forces re-measurement", "prevents": "re-selecting instruments after the construct is redefined"},
 {"rule": "Do not finalize wording before surfacing presuppositions; a late-found loaded premise re-opens framing", "prevents": "re-framing the whole question after a presupposition is challenged"},
 {"rule": "Do not prioritize sub-questions before the DAG is acyclic; a cycle invalidates any order", "prevents": "re-ordering all sub-questions after a dependency cycle surfaces"},
 {"rule": "Do not specify an answer type before checking falsifiability; an unfalsifiable hypothesis has no answerable shape", "prevents": "re-deriving the answer spec after the hypothesis is found untestable"},
 {"rule": "Do not decompose before scope is bounded; an unbounded tree must be rebuilt once scope is set", "prevents": "rebuilding the issue tree after scope is finally fixed"},
 {"rule": "Do not over-neutralize without checking intent; stripped emphasis must be restored later", "prevents": "re-inserting decision-relevant salience after a too-bland rewrite"},
 {"rule": "Do not reframe on non-material change; churn forces repeated re-validation of unchanged checks", "prevents": "re-running framing checks that no change actually invalidated"},
]

IP = [
 {"trigger": "a question is found to be unanswerable or mis-classified as posed", "action": "re-run ANSWERABILITY to reassign the class and route to operationalization or rejection", "nodes": ["ANSWERABILITY", "TOPIC_INTAKE"], "priority": "critical"},
 {"trigger": "an instrument is found to measure a different construct than the one defined", "action": "revise CONSTRUCT_DEF or re-select the indicator in INDICATOR_MAP before re-binding the measure", "nodes": ["CONSTRUCT_DEF", "INDICATOR_MAP", "MEASURE_BIND"], "priority": "high"},
 {"trigger": "a load-bearing presupposition is challenged or falsified", "action": "re-run ASSUMPTION_SURFACE and FRAMING_EFFECT and reframe the affected sub-questions", "nodes": ["ASSUMPTION_SURFACE", "FRAMING_EFFECT", "REFRAME_LOOP"], "priority": "high"},
 {"trigger": "a dependency cycle appears among sub-questions", "action": "break the cycle in SUBQ_DAG and re-run PRIORITIZE_SUBQ", "nodes": ["SUBQ_DAG", "PRIORITIZE_SUBQ"], "priority": "high"},
 {"trigger": "a hypothesis is shown to exclude no possible observation", "action": "tighten HYPOTHESIS_FORM and re-check FALSIFIABILITY before specifying the answer type", "nodes": ["HYPOTHESIS_FORM", "FALSIFIABILITY", "ANSWER_TYPE_SPEC"], "priority": "high"},
 {"trigger": "scope is found too narrow or too coarse for the asker's decision", "action": "revise SCOPE_BOUND and GRAIN_LEVEL and re-decompose the affected branches", "nodes": ["SCOPE_BOUND", "GRAIN_LEVEL", "ISSUE_TREE"], "priority": "medium"},
 {"trigger": "a stakeholder argues neutralization stripped required emphasis", "action": "re-run NEUTRALIZE against TOPIC_INTAKE intent to restore decision-relevant salience", "nodes": ["NEUTRALIZE", "PRIORITIZE_SUBQ", "TOPIC_INTAKE"], "priority": "medium"},
]

spec = {
 "domain": "frame__operationalization",
 "domain_label": "Question Framing & Operationalization (Question Science subdomain)",
 "purpose": "session_bounded_method_for_turning_a_vague_topic_into_an_answerable_well_scoped_question_by_testing_answerability_structuring_with_pico_and_finer_mapping_construct_to_indicator_to_measure_decomposing_into_a_mece_sub_question_dag_surfacing_hidden_assumptions_and_neutralizing_loaded_framing_for_clarity_and_testability_not_persuasion",
 "assumptions": [
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "scope is the framing-and-operationalization method itself; answering the question and study execution are delegated to downstream stages",
   "the asker's intent and decision context can be elicited well enough to anchor scoping",
   "framing is for clarity and testability, not persuasion: neutrality and decision-relevant emphasis are both goals",
 ],
 "exclusions": [
   "producing the substantive answer or running the study (delegated to downstream answering stages)",
   "statistical analysis and effect estimation (delegated to analysis KBs)",
   "evidence gathering and source retrieval (delegated to retrieval stages)",
   "persuasive or rhetorical framing intended to steer rather than clarify",
 ],
 "source_description": "heuristic prior estimates for question-framing and operationalization work units, informed by the question-science, evidence-based-practice, structured-problem-solving, measurement-theory, and philosophy-of-science literatures; no supplied dataset",
 "source_citation": "Cooke 1880 The New Chemistry / Richardson, Wilson, Nishikawa & Hayward 1995 (PICO, ACP Journal Club); Hulley, Cummings, Browner, Grady & Newman, Designing Clinical Research (FINER criteria); Minto, The Pyramid Principle (MECE and issue trees); Tversky & Kahneman 1981 The Framing of Decisions and the Psychology of Choice (framing effects); Popper 1959 The Logic of Scientific Discovery (falsifiability); Cronbach & Meehl 1955 Construct Validity in Psychological Tests",
 "competency_questions": CQS,
 "glossary": GLS,
 "nodes": N,
 "edges": E,
 "conflict_axes": CA,
 "edge_cases": EC,
 "workflow": WF,
 "dominance_rules": DR,
 "anti_rework_rules": ARR,
 "iteration_protocol": IP,
 "priority_rationale": "TOPIC_INTAKE/ANSWERABILITY anchor the method; PICO/FINER and construct->indicator->measure operationalize; scope, issue tree and sub-question DAG decompose; assumption surfacing, framing detection and neutralization protect neutrality; hypothesis/falsifiability/answer-type close testability; the reframing loop keeps the question current.",
 "eval_objective": "verify_answerability_testing_operationalization_mece_decomposition_assumption_surfacing_framing_neutralization_and_falsifiability_of_frame__operationalization_kb",
}

out_dir = "branches/b10_question_compiler/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "frame__operationalization.spec.json")
open(path, "w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes", len(N), "edges", len(E), "CA", len(CA), "EC", len(EC), "WF", len(WF), "CQ", len(CQS), "DR", len(DR), "ARR", len(ARR), "IP", len(IP))
