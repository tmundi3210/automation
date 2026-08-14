#!/usr/bin/env python3
"""Generate the kumap__uncertainty_taxonomy content spec (B11 KB) for kb_forge.py.
Compact authoring: node() applies sane defaults so only domain content + base
metric magnitudes are specified per node. Domain: session-bounded method for
mapping what is known vs unknown about a question — typing knowledge cells
(known-known .. unknown-unknown), distinguishing aleatory vs epistemic
uncertainty, surfacing assumptions as conditional knowns, identifying and
prioritizing knowledge gaps by value-of-information, and actively probing for
unknown-unknowns. Neutral / epistemic stance."""
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
        "academic_fields": ["epistemology", "decision_analysis", "knowledge_acquisition"],
        "subfields": subfields or ["uncertainty_taxonomy", "knowledge_mapping"],
        "specialists": specialists or ["knowledge_engineer"],
        "contradictors": contradictors or ["single_point_estimate_advocate"],
        "inputs": inputs or ["the question under study", "available evidence and claims"],
        "outputs": outputs or ["typed knowledge/uncertainty annotations"],
        "dependencies": deps, "must_not_finalize_before": [],
        "competency_question_refs": cqs, "evidence_refs": [SRC],
        "base": b,
        "scope_boundary": {"included": scope_in, "excluded": scope_out},
        "pros": pros, "cons": cons, "failure_modes": fmodes,
        "acceptance_tests": accept, "revisit_triggers": revisit,
        "handoff_artifact_required": True, "lifecycle_state": "draft",
    }

def b(C,BV,UV,TC,R,X,IR,CF,NCP,AT,DG,PI,EC,FR,DW,ui,sig):
    return {"criticality":C,"business_value":BV,"user_value":UV,"technical_complexity":TC,
            "risk_if_wrong":R,"cross_topic_coupling":X,"irreversibility":IR,"confidence":CF,
            "node_conflict_pressure":NCP,"acceptance_test_pass_rate":AT,"dependency_gate_pass_rate":DG,
            "prior_importance":PI,"evidence_confidence":EC,"failure_rate":FR,"downside_weight":DW,
            "uncertainty_interval":ui,"update_signal":sig}

def pro(claim, ex, w=0.78): return {"weight":w,"claim":claim,"example":ex}
def con(claim, ex, w=0.6): return {"weight":w,"claim":claim,"example":ex}

N = []

# ---- FOUNDATIONS: the four knowledge cells (Johari-style 2x2 over awareness x possession) ----
N.append(node("KNOWN_KNOWN","consciously_held_justified_knowledge",
  "Type a claim as a known-known: a proposition the inquirer is aware of holding AND can justify with cited evidence, so it can be used directly without further acquisition.",
  "cells", [], ["CQ_01"],
  b(0.9,0.78,0.82,0.5,0.8,0.7,0.55,0.74,0.42, 0.8,0.82, 0.88,0.72,0.16,0.5,[0.18,0.46],"justification standard or evidence threshold for 'known' changes"),
  ["awareness + justification test for a claim","tagging a claim usable-as-is"],
  ["measuring how reducible the residual uncertainty is","downstream evidence weighting"],
  [pro("A justified known-known is the only cell whose content may be consumed without acquisition, so typing it cleanly prevents wasted re-research","'service SLA is 99.9%' cited from the signed contract is usable now")],
  [con("Overclaiming known-known status converts an assumption into a false certainty that propagates silently","treating a vendor's marketing number as a justified known")],
  ["unjustified belief mislabeled known-known","stale known not re-checked against new evidence"],
  ["each known-known carries an explicit justification and evidence reference"],
  ["justification standard revised","a known-known is contradicted by new evidence"],
  specialists=["knowledge_engineer","epistemologist"]))

N.append(node("KNOWN_UNKNOWN","identified_acknowledged_gaps",
  "Type a question as a known-unknown: a gap the inquirer is consciously aware of and can name, making it an addressable target for acquisition or estimation.",
  "cells", ["KNOWN_KNOWN"], ["CQ_01","CQ_02"],
  b(0.88,0.76,0.78,0.55,0.78,0.74,0.5,0.66,0.5, 0.78,0.8, 0.86,0.66,0.18,0.5,[0.2,0.5],"the catalog of named open questions changes"),
  ["naming a gap as an explicit question","attaching an acquisition or estimation handle"],
  ["prioritizing among gaps","detecting gaps not yet named"],
  [pro("A named known-unknown is the unit acquisition can act on: you can plan a query, an experiment, or an elicitation for it","'we don't know peak QPS' becomes a load-test task")],
  [con("A growing list of known-unknowns can create an illusion of completeness that hides the unknown-unknowns beyond it","an exhaustive risk register that still misses the novel failure")],
  ["gap stated too vaguely to be acquirable","known-unknown left without an acquisition handle"],
  ["every known-unknown is phrased as an answerable question with an acquisition or estimation route"],
  ["new open question surfaced","a known-unknown is resolved or reframed"],
  specialists=["knowledge_engineer","requirements_analyst"]))

N.append(node("UNKNOWN_KNOWN","tacit_unsurfaced_knowledge",
  "Type and surface unknown-knowns: knowledge the inquirer or team implicitly holds (tacit skill, embedded assumption, institutional memory) but has not made explicit, so it is unavailable for justification or audit.",
  "cells", ["KNOWN_KNOWN"], ["CQ_03"],
  b(0.82,0.72,0.74,0.62,0.78,0.78,0.55,0.6,0.55, 0.74,0.76, 0.82,0.6,0.22,0.5,[0.24,0.58],"elicitation method or tacit-knowledge inventory changes"),
  ["eliciting implicit/tacit knowledge into explicit claims","detecting embedded assumptions"],
  ["irreducibly stochastic uncertainty","quantitative gap prioritization"],
  [pro("Surfacing an unknown-known converts hidden tacit knowledge into an auditable, sharable known-known and removes a class of silent assumptions","an engineer's unstated 'the cache is always warm' written down and tested")],
  [con("Aggressive elicitation can over-formalize tacit skill that resists articulation, producing brittle or false explicit rules","forcing a heuristic feel into a rigid checklist that breaks on edge cases")],
  ["tacit assumption never surfaced and silently relied upon","elicited tacit rule mis-states the actual skill"],
  ["material tacit assumptions are elicited into explicit, testable claims"],
  ["new participant or system brings unsurfaced tacit knowledge","an elicited rule is found to misrepresent practice"],
  specialists=["knowledge_engineer","domain_expert_interviewer"]))

N.append(node("UNKNOWN_UNKNOWN","unrecognized_ignorance",
  "Type and bound unknown-unknowns: ignorance the inquirer is not even aware of (questions not asked, failure modes not imagined), which cannot be listed directly and must be probed for indirectly.",
  "cells", ["KNOWN_UNKNOWN"], ["CQ_03","CQ_04"],
  b(0.9,0.82,0.78,0.74,0.9,0.85,0.66,0.5,0.62, 0.66,0.7, 0.9,0.5,0.3,0.62,[0.34,0.74],"a previously unimagined failure or question class is discovered"),
  ["bounding the space of unimagined questions","routing to active probing (DISCONFIRMATION)"],
  ["enumerating the unknown-unknowns directly (impossible)","measurement-only uncertainty"],
  [pro("Naming the unknown-unknown cell, even though its contents cannot be listed, licenses humility margins and active probing instead of false closure","reserving a 'surprise budget' rather than claiming the risk list is complete")],
  [con("Unbounded fear of unknown-unknowns can paralyze decision-making or justify infinite search","refusing to ship until every conceivable surprise is ruled out")],
  ["false closure: treating the known map as the whole territory","unbounded search chasing hypothetical surprises"],
  ["the map records an explicit residual-ignorance margin and at least one active probe for unknown-unknowns"],
  ["a realized surprise reveals a missed question class","scope or environment changes materially"],
  specialists=["knowledge_engineer","red_team_analyst"],
  contradictors=["completeness_optimist"]))

# ---- UNCERTAINTY NATURE: aleatory vs epistemic and the Walker-style taxonomy ----
N.append(node("ALEATORY","irreducible_stochastic_uncertainty",
  "Classify uncertainty as aleatory: variability inherent in the phenomenon (stochastic, chance-driven) that more knowledge about the system cannot reduce, only characterize as a distribution.",
  "nature", ["KNOWN_UNKNOWN"], ["CQ_05"],
  b(0.84,0.74,0.72,0.66,0.8,0.74,0.5,0.62,0.52, 0.76,0.78, 0.84,0.62,0.2,0.52,[0.22,0.54],"the system boundary that fixes what counts as inherent variability changes"),
  ["labeling uncertainty as irreducible-by-knowledge","characterizing it as a frequency/distribution"],
  ["reducible knowledge gaps","linguistic ambiguity"],
  [pro("Tagging uncertainty aleatory tells acquisition to stop seeking facts and instead estimate a distribution, saving wasted reduction effort","dice-roll variability is modeled, not 'researched away'")],
  [con("The aleatory/epistemic split is model-relative: what looks irreducible may be epistemic under a finer model, so a hard label can entrench avoidable ignorance","calling weather 'random' when a better model would reduce it")],
  ["epistemic gap mislabeled aleatory (gives up reducible knowledge)","aleatory variability quantified with a spuriously tight distribution"],
  ["each aleatory tag states the system boundary under which the variability is treated as irreducible"],
  ["system boundary or model resolution changes","apparent variability is reduced by a new model"],
  specialists=["statistician","risk_analyst"]))

N.append(node("EPISTEMIC","reducible_by_knowledge_uncertainty",
  "Classify uncertainty as epistemic: it stems from lack of knowledge about a fixed-but-unknown quantity and can in principle be reduced by acquiring evidence, marking it a candidate for the gap pipeline.",
  "nature", ["KNOWN_UNKNOWN"], ["CQ_05","CQ_06"],
  b(0.86,0.78,0.76,0.64,0.82,0.78,0.5,0.62,0.55, 0.76,0.78, 0.86,0.62,0.2,0.52,[0.22,0.54],"new evidence channels make a previously irreducible quantity reducible"),
  ["labeling uncertainty as reducible-by-evidence","routing it into gap identification"],
  ["inherent stochastic variability","downstream evidence-label assignment"],
  [pro("Marking uncertainty epistemic identifies exactly where acquisition can buy down uncertainty, so value-of-information analysis applies","an unknown constant becomes a measurable target")],
  [con("Treating deep or aleatory uncertainty as merely epistemic breeds overconfidence: it promises reduction that evidence cannot actually deliver","a 'just measure it' stance toward a genuinely deep uncertainty")],
  ["aleatory or deep uncertainty mislabeled epistemic","epistemic reduction assumed cheaper than it is"],
  ["each epistemic tag names a concrete evidence channel that could reduce it"],
  ["a reduction effort fails to move the uncertainty","the quantity is reclassified as aleatory or deep"],
  specialists=["statistician","knowledge_engineer"]))

N.append(node("MODEL_UNC","structural_model_uncertainty",
  "Identify model/structural uncertainty (Walker et al.): uncertainty about the form of the model or framing itself — wrong variables, wrong relationships, wrong boundaries — distinct from uncertainty in its inputs.",
  "nature", ["EPISTEMIC"], ["CQ_07"],
  b(0.86,0.78,0.7,0.72,0.85,0.82,0.6,0.56,0.6, 0.72,0.74, 0.86,0.56,0.24,0.58,[0.28,0.66],"the modeling framework or its scenario set is revised"),
  ["framing/structure uncertainty","alternative-model and scenario enumeration"],
  ["parameter-level error","ambiguity in terms"],
  [pro("Separating structural from parametric uncertainty prevents the classic error of tuning parameters of a wrong model and calling it reduced","exploring rival causal structures rather than refitting one")],
  [con("Structural uncertainty resists quantification: enumerating alternative model forms is open-ended and can shade into deep uncertainty","you cannot list all wrong framings of a novel system")],
  ["parameter tuning masquerades as structural validation","alternative model forms never enumerated"],
  ["at least one rival model structure or framing is articulated and compared"],
  ["model framework changes","a rival structure better explains the data"],
  specialists=["modeler","decision_analyst"]))

N.append(node("MEASUREMENT_UNC","measurement_and_parameter_uncertainty",
  "Identify measurement/parameter uncertainty (Walker et al.): uncertainty in the values of inputs and parameters given a fixed model structure — sampling error, instrument error, estimate spread.",
  "nature", ["EPISTEMIC"], ["CQ_07"],
  b(0.8,0.72,0.7,0.6,0.74,0.7,0.45,0.68,0.45, 0.8,0.8, 0.8,0.68,0.18,0.46,[0.2,0.48],"measurement instrument, sampling design, or parameter source changes"),
  ["input/parameter value uncertainty","sampling and instrument error"],
  ["wrong-model uncertainty","irreducible variability framing"],
  [pro("Parameter uncertainty is the most tractable kind: it has well-developed estimation and interval methods, so it can often be reduced cheaply","a confidence interval on a measured rate from more samples")],
  [con("Reducing parameter uncertainty within a wrong model gives false precision about the wrong thing","tighter intervals on a parameter of a misspecified model")],
  ["parameter precision reported without model-validity caveat","sampling bias treated as random error"],
  ["each measurement-uncertainty estimate states its source (sampling/instrument) and an interval, not a point"],
  ["instrument or sampling design changes","a parameter estimate is revised by new data"],
  specialists=["statistician","metrologist"]))

N.append(node("AMBIGUITY_UNC","linguistic_and_ambiguity_uncertainty",
  "Identify ambiguity/linguistic uncertainty: imprecision arising from the question or terms themselves — vagueness, context-dependence, underspecification — which is dissolved by definition, not by evidence.",
  "nature", ["KNOWN_UNKNOWN"], ["CQ_08"],
  b(0.78,0.7,0.74,0.56,0.74,0.72,0.45,0.66,0.48, 0.78,0.78, 0.78,0.66,0.18,0.44,[0.2,0.48],"term definitions or the question framing change"),
  ["detecting vague/underspecified terms","disambiguation by definition or operationalization"],
  ["empirical fact-gaps","stochastic variability"],
  [pro("Catching ambiguity first prevents pseudo-gaps: an 'unknown' that is really an undefined term is dissolved by definition, not costly acquisition","'is it fast?' resolved by defining a latency target, not by measurement")],
  [con("Over-operationalizing can prematurely narrow a genuinely open concept and discard relevant meanings","pinning 'quality' to one metric and losing the rest")],
  ["ambiguity mis-routed to empirical acquisition","operational definition silently narrows the question"],
  ["each ambiguity-tagged item is resolved by an explicit definition or operationalization before acquisition is planned"],
  ["term definitions revised","a defined term is found to mismatch stakeholder intent"],
  specialists=["requirements_analyst","linguist"]))

N.append(node("DEEP_UNCERTAINTY","deep_knightian_uncertainty",
  "Identify deep (Knightian / level-4+) uncertainty: situations where parties cannot agree on the probability distribution or even the full set of outcomes, so standard probabilistic reduction does not apply and decisions need robustness, not optimization.",
  "nature", ["EPISTEMIC","UNKNOWN_UNKNOWN"], ["CQ_06"],
  b(0.9,0.82,0.78,0.78,0.9,0.85,0.66,0.46,0.66, 0.64,0.7, 0.9,0.46,0.3,0.66,[0.36,0.78],"the agreed outcome set or distribution becomes contested or unknown"),
  ["flagging non-probabilistic uncertainty","routing to robustness/scenario methods (RDM)"],
  ["well-characterized risk with an agreed distribution","precise interval estimation"],
  [pro("Naming deep uncertainty honestly switches the method from optimizing a known distribution to seeking decisions robust across many futures (RAND/Lempert RDM)","planning that performs acceptably across scenarios rather than betting on one forecast")],
  [con("Declaring deep uncertainty too readily abandons tractable estimation and can excuse analytical laziness","calling a measurable quantity 'deeply uncertain' to avoid the work")],
  ["deep uncertainty forced into a single point estimate (overconfidence)","tractable uncertainty abandoned as 'deep' to avoid analysis"],
  ["deep-uncertainty items are handled by robustness/scenario analysis, not a single probability estimate"],
  ["outcome set or distribution becomes contested","a robust option set is invalidated by a new scenario"],
  specialists=["decision_analyst","scenario_planner"],
  contradictors=["expected_value_maximalist"]))

# ---- ANNOTATION LAYER: assumptions, confidence, typing of ignorance ----
N.append(node("ASSUMPTION_REGISTRY","assumptions_as_conditional_knowns",
  "Maintain a registry of assumptions: unverified propositions the inquiry depends on, recorded as conditional knowns (true-for-now, pending check) with the conclusions that rest on each, so dependence is explicit and revisitable.",
  "annotation", ["KNOWN_UNKNOWN","UNKNOWN_KNOWN"], ["CQ_09"],
  b(0.88,0.8,0.78,0.6,0.85,0.82,0.6,0.62,0.58, 0.76,0.78, 0.88,0.62,0.2,0.56,[0.24,0.58],"an assumption is invalidated or a new load-bearing assumption is added"),
  ["recording assumptions as conditional knowns","linking each assumption to dependent conclusions"],
  ["verified facts (known-knowns)","gap prioritization mechanics"],
  [pro("Treating assumptions as flagged conditional knowns keeps inferences usable while making every dependence auditable and falsifiable","tagging 'assume traffic doubles' so all sizing rests visibly on it")],
  [con("A bloated assumption registry can hide which assumptions are actually load-bearing for the decision","hundreds of logged assumptions, none ranked by impact")],
  ["load-bearing assumption left unrecorded","assumption recorded but not linked to what depends on it"],
  ["every load-bearing assumption is registered as a conditional known with its dependent conclusions and a check route"],
  ["an assumption is contradicted","a new conclusion adds a hidden dependence"],
  specialists=["analyst","auditor"]))

N.append(node("CONFIDENCE_STATE","typed_confidence_per_claim",
  "Attach a typed confidence state to each claim: a calibrated, evidence-labeled qualifier (e.g. justified / provisional / speculative with an interval) that travels with the claim for downstream use.",
  "annotation", ["KNOWN_KNOWN","ASSUMPTION_REGISTRY"], ["CQ_10"],
  b(0.86,0.78,0.78,0.62,0.82,0.8,0.55,0.62,0.55, 0.78,0.8, 0.86,0.62,0.2,0.54,[0.22,0.54],"the confidence vocabulary or calibration standard changes"),
  ["assigning calibrated confidence per claim","attaching an evidence label and interval"],
  ["binary believe/disbelieve tagging","legality gating for downstream consumers"],
  [pro("A typed, calibrated confidence state lets downstream reasoning weight claims correctly instead of treating all knowns as equal","a 'provisional, 0.4-0.7' tag stops a weak claim being used as a hard fact")],
  [con("Numeric confidence invites false precision — a fabricated 0.73 can look more authoritative than an honest 'roughly even'","attaching three-decimal probabilities to subjective hunches")],
  ["false precision in confidence numbers","confidence assigned without calibration or evidence basis"],
  ["each claim carries a calibrated confidence state with an evidence label and an interval, not a bare point"],
  ["calibration check fails","confidence vocabulary is revised"],
  specialists=["forecaster","calibration_specialist"],
  contradictors=["false_precision_critic"]))

N.append(node("IGNORANCE_TYPING","taxonomy_of_ignorance",
  "Apply the full ignorance taxonomy to each item: assign its knowledge cell, its uncertainty nature (aleatory/epistemic/ambiguity/model/measurement/deep), and whether it is recognized — giving every item a complete type before it is acted on.",
  "annotation", ["UNKNOWN_KNOWN","UNKNOWN_UNKNOWN","ALEATORY","EPISTEMIC","AMBIGUITY_UNC"], ["CQ_03","CQ_11"],
  b(0.9,0.82,0.78,0.72,0.86,0.85,0.6,0.58,0.6, 0.72,0.74, 0.9,0.58,0.22,0.58,[0.28,0.66],"the taxonomy categories or their decision rules change"),
  ["assigning a complete type (cell + nature + recognition)","reconciling overlapping categories"],
  ["the downstream acquisition action itself","numeric scoring of value-of-information"],
  [pro("A complete type per item makes the right treatment derivable: define-vs-measure-vs-distribution-vs-robustness follows from the type","ambiguity -> define; epistemic -> acquire; aleatory -> distribution; deep -> robustness")],
  [con("Forcing every item into one box ignores that real items are blends, and category boundaries are themselves uncertain","an item that is part-ambiguous, part-empirical typed as only one")],
  ["items left untyped and acted on by default","blended uncertainty forced into a single category"],
  ["every mapped item carries a cell type, a nature type, and a recognition flag"],
  ["taxonomy categories revised","an item's type is found to mislead its treatment"],
  specialists=["knowledge_engineer","epistemologist"]))

# ---- GAP PIPELINE: identify, prioritize, frontier ----
N.append(node("GAP_IDENTIFY","identify_knowledge_gaps",
  "Identify knowledge gaps: enumerate the known-unknowns and reducible-epistemic items that bear on the question, deduplicated against existing knowns and ambiguity-only pseudo-gaps.",
  "gap_pipeline", ["KNOWN_UNKNOWN","EPISTEMIC","IGNORANCE_TYPING"], ["CQ_02","CQ_12"],
  b(0.86,0.8,0.8,0.62,0.8,0.78,0.5,0.64,0.55, 0.78,0.8, 0.86,0.64,0.2,0.5,[0.22,0.54],"the question scope or the set of relevant gaps changes"),
  ["enumerating decision-relevant gaps","deduplicating against knowns and pseudo-gaps"],
  ["ranking gaps","probing for unrecognized gaps"],
  [pro("A deduplicated gap list scoped to the decision keeps acquisition focused on what actually matters, not every conceivable unknown","dropping a gap that, once defined, was already a known")],
  [con("Gap identification can never be provably exhaustive, so a complete-looking list can mask the unknown-unknowns beyond it","a tidy gap list that omits the failure no one imagined")],
  ["pseudo-gap (really ambiguity) listed as empirical gap","duplicate of an existing known listed as a gap"],
  ["each identified gap is decision-relevant, distinct from existing knowns, and not merely an undefined term"],
  ["question scope changes","a listed gap is found to duplicate a known or an ambiguity"],
  specialists=["analyst","knowledge_engineer"]))

N.append(node("GAP_PRIORITIZE","prioritize_gaps_by_value_of_information",
  "Prioritize identified gaps by value of information: rank each gap by how much resolving it would change the decision, weighed against the cost and feasibility of acquiring it, so scarce effort buys the most decision-relevance.",
  "gap_pipeline", ["GAP_IDENTIFY","CONFIDENCE_STATE"], ["CQ_12"],
  b(0.88,0.82,0.8,0.68,0.84,0.8,0.55,0.6,0.58, 0.76,0.78, 0.88,0.6,0.22,0.56,[0.26,0.6],"the decision, its payoffs, or acquisition costs change"),
  ["value-of-information ranking of gaps","cost/feasibility weighting"],
  ["the acquisition execution itself","exhaustive enumeration of every gap"],
  [pro("Value-of-information ranking stops the team from researching a low-impact unknown while a decision-flipping gap stays open","prioritizing the gap that would change go/no-go over a cosmetic detail")],
  [con("VoI estimates depend on a decision model that may itself be wrong, so the ranking inherits that model's blind spots","ranking under a payoff model that omits the real driver")],
  ["effort spent on low-VoI gaps while high-VoI gaps stay open","VoI computed from a mis-specified decision model"],
  ["gaps are ranked by decision-impact against acquisition cost, with the top gaps justified"],
  ["the decision or its payoffs change","a resolved gap fails to move the decision as ranked"],
  specialists=["decision_analyst","economist"]))

N.append(node("BOUNDARY_OF_KNOWLEDGE","map_the_epistemic_frontier",
  "Map the boundary of knowledge: render the frontier between what is known and what is not as an explicit artifact, marking which cells are populated, which gaps are open, and where the recognized map ends.",
  "gap_pipeline", ["IGNORANCE_TYPING","GAP_IDENTIFY"], ["CQ_11"],
  b(0.84,0.78,0.8,0.62,0.78,0.82,0.55,0.62,0.52, 0.76,0.78, 0.84,0.62,0.2,0.52,[0.24,0.56],"the frontier shifts as knowns are added or gaps are opened"),
  ["rendering the known/unknown frontier as an artifact","marking the edge of the recognized map"],
  ["the active probing beyond the frontier","numeric gap scoring"],
  [pro("An explicit frontier artifact communicates at a glance what is settled, what is open, and where confidence ends, aligning a team's epistemic state","a one-page map showing knowns, open gaps, and a 'here be dragons' margin")],
  [con("Any drawn boundary implies the far side is empty, subtly encouraging the false-closure error the map is meant to fight","a crisp frontier read as 'nothing important beyond here'")],
  ["frontier drawn as if it were the edge of all relevant truth","boundary not updated as knowledge shifts"],
  ["the frontier artifact distinguishes populated cells, open gaps, and an explicit residual-ignorance margin"],
  ["a known or gap changes the frontier","the boundary is treated as complete"],
  specialists=["knowledge_engineer","information_designer"]))

N.append(node("PROXY_KNOWN","proxy_surrogate_knowledge",
  "Establish proxy/surrogate knowledge for inaccessible facts: when a target quantity cannot be directly known, identify a correlated observable to stand in for it, recording the proxy's bias and validity conditions.",
  "gap_pipeline", ["EPISTEMIC","GAP_IDENTIFY"], ["CQ_13"],
  b(0.8,0.74,0.74,0.66,0.8,0.74,0.5,0.6,0.5, 0.74,0.76, 0.8,0.6,0.24,0.52,[0.26,0.6],"the proxy's correlation to the target changes or breaks"),
  ["selecting a correlated observable as a stand-in","recording proxy bias and validity conditions"],
  ["direct measurement of the target","value-of-information ranking"],
  [pro("A well-characterized proxy makes an otherwise inaccessible quantity partially knowable, with its bias stated so consumers can discount it","using deployment frequency as a proxy for hard-to-measure team throughput")],
  [con("Proxies drift: a surrogate valid yesterday can decouple from the target and quietly mislead (Goodhart-style)","a metric that becomes a target and stops tracking the thing it proxied")],
  ["proxy treated as the target itself","proxy correlation assumed stable without re-check"],
  ["each proxy records its target, its known bias, and the conditions under which it remains valid"],
  ["the proxy-target correlation weakens","the proxy becomes an optimization target"],
  specialists=["statistician","measurement_designer"]))

# ---- ACTIVE EPISTEMICS: probing, propagation, maintenance ----
N.append(node("DISCONFIRMATION","active_search_for_unknown_unknowns",
  "Actively search to convert unknown-unknowns into known-unknowns: deliberately seek disconfirming evidence, run premortems and red-teams, and probe outside the current frame to surface questions not yet asked.",
  "active", ["UNKNOWN_UNKNOWN","BOUNDARY_OF_KNOWLEDGE"], ["CQ_04"],
  b(0.88,0.82,0.8,0.74,0.9,0.85,0.6,0.5,0.64, 0.66,0.7, 0.9,0.5,0.3,0.64,[0.34,0.74],"a probe surfaces a new question class or the frame is found too narrow"),
  ["disconfirmation-seeking and premortem/red-team probes","reframing to reveal unasked questions"],
  ["confirming existing knowns","exhaustive enumeration of all surprises"],
  [pro("Active disconfirmation is the only operation that can move items out of the unknown-unknown cell, turning surprises into named gaps before they bite","a premortem surfacing a failure mode the plan never considered")],
  [con("The search for unknown-unknowns is unbounded and can consume arbitrary effort with diminishing returns","red-teaming a low-stakes decision until the budget is gone")],
  ["confirmation bias makes the probe only seek supporting evidence","unbounded probing with no stopping rule"],
  ["at least one disconfirmation-seeking probe is run with an explicit stopping rule proportional to stakes"],
  ["a realized surprise reveals the frame was too narrow","stakes change enough to revisit probe depth"],
  specialists=["red_team_analyst","forecaster"],
  contradictors=["confirmation_bias_skeptic"]))

N.append(node("UNC_PROPAGATION","propagate_uncertainty_through_inferences",
  "Propagate uncertainty through inferences: combine the confidence states and assumptions feeding a conclusion so the conclusion inherits an honest, traceable uncertainty rather than a spuriously confident one.",
  "active", ["CONFIDENCE_STATE","ASSUMPTION_REGISTRY","MEASUREMENT_UNC"], ["CQ_10"],
  b(0.86,0.78,0.76,0.74,0.85,0.85,0.58,0.56,0.6, 0.72,0.74, 0.86,0.56,0.24,0.58,[0.28,0.66],"the inference chain or any input confidence state changes"),
  ["combining input uncertainties into a conclusion","tracing which inputs dominate the result's uncertainty"],
  ["assigning the input confidences themselves","downstream legality gating"],
  [pro("Honest propagation prevents the laundering of uncertainty: a conclusion drawn from shaky inputs cannot present as more certain than its weakest load-bearing input","a result inheriting the wide interval of the assumption it rests on")],
  [con("Naive independence assumptions in propagation can drastically understate combined uncertainty when inputs are correlated","multiplying correlated errors as if independent")],
  ["uncertainty laundered: conclusion firmer than its inputs","correlated inputs propagated as independent"],
  ["each conclusion's stated uncertainty is no tighter than its dominant load-bearing input and shows that input"],
  ["an input confidence state changes","a hidden correlation among inputs is discovered"],
  specialists=["statistician","analyst"]))

N.append(node("MAP_MAINTENANCE","maintain_the_living_known_unknown_map",
  "Maintain the living known/unknown map across the session: re-type items as evidence arrives, retire resolved gaps, register new ones, and keep the boundary, assumptions, and confidence states current and consistent.",
  "active", ["BOUNDARY_OF_KNOWLEDGE","ASSUMPTION_REGISTRY","CONFIDENCE_STATE"], ["CQ_11"],
  b(0.84,0.78,0.8,0.6,0.8,0.85,0.55,0.62,0.55, 0.78,0.8, 0.84,0.62,0.2,0.52,[0.22,0.54],"the map drifts from the current evidence or a re-typing is overdue"),
  ["re-typing items on new evidence","retiring resolved gaps and registering new ones"],
  ["initial typing of items","the inference propagation step"],
  [pro("A maintained living map keeps the team's epistemic state honest over a session instead of decaying into a stale snapshot","a resolved gap retired and its dependent conclusions upgraded the same session")],
  [con("Continuous re-typing has overhead and can churn: over-frequent updates destabilize the shared map","re-scoring confidence on every minor signal until nothing settles")],
  ["map decays into a stale snapshot","update churn destabilizes the shared epistemic state"],
  ["the map reflects the latest evidence: resolved gaps retired, new gaps and re-typings applied, consistency preserved"],
  ["new evidence arrives","a re-typing is overdue or the map contradicts itself"],
  specialists=["knowledge_engineer","analyst"]))

N.append(node("CONFIDENCE_LEGALITY","confidence_label_legality_interface",
  "Define the legality interface for downstream evidence use: rules stating which confidence labels and uncertainty types are permitted to support which downstream claims, so weakly-grounded knowns cannot be consumed as if certain.",
  "active", ["CONFIDENCE_STATE","UNC_PROPAGATION","IGNORANCE_TYPING"], ["CQ_10","CQ_14"],
  b(0.86,0.8,0.78,0.66,0.88,0.82,0.62,0.6,0.58, 0.76,0.78, 0.86,0.6,0.22,0.6,[0.26,0.6],"the downstream label-eligibility policy or evidence governance rules change"),
  ["mapping confidence labels to permitted downstream uses","gating consumption of weak knowns"],
  ["the calibration of confidence itself","the propagation arithmetic"],
  [pro("A legality interface is the gate that stops a speculative or assumption-grade claim from being consumed downstream as an experimentally-validated fact","barring a 'heuristic' label from supporting a safety-critical decision")],
  [con("Over-strict legality rules can starve downstream reasoning of usable inputs, stalling decisions that could tolerate provisional evidence","rejecting all provisional knowledge so no decision can proceed")],
  ["weak claim consumed downstream as certain","legality so strict that usable provisional evidence is barred"],
  ["each downstream use is checked against the legality of its inputs' confidence labels and uncertainty types"],
  ["evidence governance policy changes","a weak claim is found to have been consumed as certain"],
  specialists=["evidence_governance_lead","auditor"]))

# ---- Competency questions (14) ----
CQ = [
 ("CQ_01","How is a claim typed across the four knowledge cells (known-known, known-unknown, unknown-known, unknown-unknown), and what does each require?",["nodes","glossary"],"KNOWN_KNOWN needs awareness plus justification, KNOWN_UNKNOWN is a named gap, UNKNOWN_KNOWN is surfaced tacit knowledge, UNKNOWN_UNKNOWN is bounded ignorance",["KNOWN_KNOWN","KNOWN_UNKNOWN","UNKNOWN_KNOWN"]),
 ("CQ_02","How are decision-relevant knowledge gaps identified and deduplicated against existing knowns and pseudo-gaps?",["nodes","workflow"],"KNOWN_UNKNOWN names gaps; GAP_IDENTIFY enumerates and deduplicates them",["KNOWN_UNKNOWN","GAP_IDENTIFY"]),
 ("CQ_03","How are unknown-knowns surfaced and how is the full ignorance taxonomy applied to each item?",["nodes"],"UNKNOWN_KNOWN elicits tacit knowledge; IGNORANCE_TYPING assigns each item a complete type",["UNKNOWN_KNOWN","IGNORANCE_TYPING"]),
 ("CQ_04","How are unknown-unknowns bounded and actively probed for to reduce false closure?",["nodes","workflow"],"UNKNOWN_UNKNOWN bounds the cell; DISCONFIRMATION actively probes to convert them into named gaps",["UNKNOWN_UNKNOWN","DISCONFIRMATION"]),
 ("CQ_05","How is uncertainty classified as aleatory versus epistemic, and why does the distinction change treatment?",["nodes"],"ALEATORY marks irreducible variability handled as a distribution; EPISTEMIC marks reducible-by-evidence uncertainty routed to acquisition",["ALEATORY","EPISTEMIC"]),
 ("CQ_06","How is deep (Knightian) uncertainty distinguished from ordinary epistemic uncertainty and handled once recognized?",["nodes","conflict_axes"],"DEEP_UNCERTAINTY flags non-probabilistic uncertainty distinct from reducible EPISTEMIC and routes it to robustness/scenario methods",["DEEP_UNCERTAINTY","EPISTEMIC"]),
 ("CQ_07","How are structural/model and measurement/parameter uncertainties separated and why does the order matter?",["nodes"],"MODEL_UNC isolates framing/structure uncertainty; MEASUREMENT_UNC isolates input/parameter uncertainty, valid only within a validated structure",["MODEL_UNC","MEASUREMENT_UNC"]),
 ("CQ_08","How is ambiguity/linguistic uncertainty detected and dissolved without costly empirical acquisition?",["nodes"],"AMBIGUITY_UNC resolves vague terms by definition rather than evidence before a gap is listed",["AMBIGUITY_UNC"]),
 ("CQ_09","How are assumptions recorded as conditional knowns so dependence on them is explicit and revisitable?",["nodes"],"ASSUMPTION_REGISTRY records assumptions as conditional knowns linked to dependent conclusions",["ASSUMPTION_REGISTRY"]),
 ("CQ_10","How is confidence typed per claim, propagated through inferences, and gated for downstream use?",["nodes","evidence_governance"],"CONFIDENCE_STATE types confidence; UNC_PROPAGATION carries it honestly; CONFIDENCE_LEGALITY gates downstream use by label",["CONFIDENCE_STATE","UNC_PROPAGATION","CONFIDENCE_LEGALITY"]),
 ("CQ_11","What does a complete type for an item consist of, and how is the frontier between known and unknown rendered and kept current?",["nodes","iteration_protocol"],"IGNORANCE_TYPING assigns cell+nature+recognition; BOUNDARY_OF_KNOWLEDGE renders the frontier and MAP_MAINTENANCE keeps it living",["IGNORANCE_TYPING","BOUNDARY_OF_KNOWLEDGE","MAP_MAINTENANCE"]),
 ("CQ_12","How are gaps prioritized for acquisition by value of information against cost?",["nodes","workflow"],"GAP_IDENTIFY enumerates gaps and GAP_PRIORITIZE ranks them by decision-impact relative to acquisition cost and feasibility",["GAP_IDENTIFY","GAP_PRIORITIZE"]),
 ("CQ_13","How is an inaccessible target quantity approached via proxy/surrogate knowledge without being misled by drift?",["nodes"],"PROXY_KNOWN selects a correlated observable and records its bias and validity conditions",["PROXY_KNOWN"]),
 ("CQ_14","What rules govern whether a given confidence label may support a downstream claim?",["nodes","evidence_governance"],"CONFIDENCE_LEGALITY maps confidence labels and uncertainty types to permitted downstream uses",["CONFIDENCE_LEGALITY"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

GL = [
 ("known_known","a proposition the inquirer is aware of holding and can justify with evidence, usable without further acquisition",["justified_known"],["assumption","unknown_known"],["KNOWN_KNOWN","CONFIDENCE_STATE"]),
 ("known_unknown","a gap the inquirer is consciously aware of and can name as an answerable question",["recognized_gap","open_question"],["unknown_unknown"],["KNOWN_UNKNOWN","GAP_IDENTIFY"]),
 ("unknown_known","tacit or implicit knowledge held but not made explicit, hence unavailable for audit",["tacit_knowledge","embedded_assumption"],["known_known"],["UNKNOWN_KNOWN","ASSUMPTION_REGISTRY"]),
 ("unknown_unknown","ignorance the inquirer is not aware of: questions not asked and failure modes not imagined",["unrecognized_ignorance"],["known_unknown"],["UNKNOWN_UNKNOWN","DISCONFIRMATION"]),
 ("aleatory_uncertainty","variability inherent in the phenomenon that more knowledge cannot reduce, only characterize as a distribution",["irreducible_uncertainty","stochastic_variability"],["epistemic_uncertainty"],["ALEATORY"]),
 ("epistemic_uncertainty","uncertainty from lack of knowledge about a fixed quantity, reducible in principle by acquiring evidence",["reducible_uncertainty"],["aleatory_uncertainty"],["EPISTEMIC","GAP_IDENTIFY"]),
 ("deep_uncertainty","a situation where the probability distribution or even the outcome set is unknown or contested, requiring robustness not optimization",["knightian_uncertainty","level_4_uncertainty"],["risk_with_known_distribution"],["DEEP_UNCERTAINTY"]),
 ("value_of_information","the expected improvement in a decision from resolving a particular uncertainty, net of acquisition cost",["voi","decision_relevance_of_a_gap"],["raw_curiosity"],["GAP_PRIORITIZE"]),
 ("conditional_known","an assumption treated as true-for-now and recorded with the conclusions that depend on it",["working_assumption"],["verified_fact"],["ASSUMPTION_REGISTRY"]),
 ("confidence_state","a calibrated, evidence-labeled qualifier (with interval) attached to a claim and carried downstream",["calibrated_confidence","epistemic_tag"],["binary_belief"],["CONFIDENCE_STATE","CONFIDENCE_LEGALITY"]),
 ("proxy_known","a correlated observable used to stand in for an inaccessible target quantity, recorded with its bias",["surrogate","indicator"],["direct_measurement"],["PROXY_KNOWN"]),
 ("epistemic_frontier","the rendered boundary between populated knowledge cells and open or unrecognized regions",["boundary_of_knowledge","known_unknown_map"],["complete_knowledge"],["BOUNDARY_OF_KNOWLEDGE","MAP_MAINTENANCE"]),
]
GLS=[{"term":t,"definition":d,"synonyms":s,"not_same_as":ns,"used_by_nodes":u} for (t,d,s,ns,u) in GL]

# ---- edges: dependency edges mirror node.dependencies (DAG) + cross-cutting non-dependency edges ----
E=[]
def dep(f,t,rs,cc=0.82,erc=0.28,cp=0.14,why="",ben="",rk="",ex=""):
    E.append({"from":f,"to":t,"edge_type":"dependency","relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{t} depends on {f}","benefit_of_coupling":ben or "ordered prerequisite",
              "risk_of_conflict":rk or "downstream rework if upstream changes","example":ex or f"{f} finalized before {t}"})
def conf(f,t,rs,st,rule,why,cp=0.5,erc=0.5,cc=0.6):
    E.append({"from":f,"to":t,"edge_type":"conflict","relation_strength":rs,"signed_tension":st,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "resolution_rule":rule,"why_related":why,"benefit_of_coupling":"tension surfaced and resolved by rule",
              "risk_of_conflict":"unmanaged tension degrades the epistemic map","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated behavior",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies)
dep("KNOWN_KNOWN","KNOWN_UNKNOWN",0.86)
dep("KNOWN_KNOWN","UNKNOWN_KNOWN",0.8)
dep("KNOWN_UNKNOWN","UNKNOWN_UNKNOWN",0.82)
dep("KNOWN_UNKNOWN","EPISTEMIC",0.8)
dep("KNOWN_UNKNOWN","AMBIGUITY_UNC",0.74)
dep("EPISTEMIC","MODEL_UNC",0.8)
dep("EPISTEMIC","MEASUREMENT_UNC",0.78)
dep("EPISTEMIC","DEEP_UNCERTAINTY",0.78)
dep("KNOWN_UNKNOWN","ASSUMPTION_REGISTRY",0.8)
dep("KNOWN_KNOWN","CONFIDENCE_STATE",0.82)
dep("ASSUMPTION_REGISTRY","CONFIDENCE_STATE",0.78)
dep("UNKNOWN_KNOWN","IGNORANCE_TYPING",0.78)
dep("EPISTEMIC","IGNORANCE_TYPING",0.78)
dep("KNOWN_UNKNOWN","GAP_IDENTIFY",0.82)
dep("IGNORANCE_TYPING","GAP_IDENTIFY",0.78)
dep("GAP_IDENTIFY","GAP_PRIORITIZE",0.84)
dep("IGNORANCE_TYPING","BOUNDARY_OF_KNOWLEDGE",0.78)
dep("GAP_IDENTIFY","PROXY_KNOWN",0.74)
dep("UNKNOWN_UNKNOWN","DISCONFIRMATION",0.82)
dep("BOUNDARY_OF_KNOWLEDGE","DISCONFIRMATION",0.78)
dep("CONFIDENCE_STATE","UNC_PROPAGATION",0.82)
dep("ASSUMPTION_REGISTRY","UNC_PROPAGATION",0.78)
dep("BOUNDARY_OF_KNOWLEDGE","MAP_MAINTENANCE",0.8)
dep("CONFIDENCE_STATE","CONFIDENCE_LEGALITY",0.82)
dep("UNC_PROPAGATION","CONFIDENCE_LEGALITY",0.78)

# cross-cutting non-dependency edges
rel("DISCONFIRMATION","KNOWN_UNKNOWN","feedback",0.82,why="probing converts an unknown-unknown into a named known-unknown, feeding the gap pipeline")
rel("MAP_MAINTENANCE","IGNORANCE_TYPING","feedback",0.78,why="new evidence triggers re-typing of items on the living map")
rel("PROXY_KNOWN","CONFIDENCE_STATE","constraint",0.74,why="a proxy's known bias constrains the confidence state of any claim it supports")
rel("AMBIGUITY_UNC","GAP_IDENTIFY","constraint",0.74,why="ambiguity must be dissolved by definition before an item counts as an empirical gap")
rel("DEEP_UNCERTAINTY","GAP_PRIORITIZE","causal",0.72,why="deep uncertainty caps the value-of-information of acquisition since evidence may not reduce it")
rel("MODEL_UNC","UNC_PROPAGATION","causal",0.72,why="structural uncertainty widens the propagated uncertainty of any model-derived conclusion")
rel("GAP_PRIORITIZE","MAP_MAINTENANCE","sequence",0.72,why="resolved high-priority gaps are retired from the living map after acquisition")

# conflict edges (negative signed_tension + resolution_rule) -- 4 real tensions
conf("GAP_IDENTIFY","GAP_PRIORITIZE",0.7,-0.6,
  "cap mapping effort at the point where the marginal gap's expected value-of-information falls below its identification cost; prioritize within the bounded set rather than seeking exhaustive enumeration",
  "exhaustive gap-mapping conflicts with the analysis cost of finding and typing every conceivable gap")
conf("EPISTEMIC","DEEP_UNCERTAINTY",0.7,-0.6,
  "treat uncertainty as epistemic (reducible by evidence) only while reduction efforts measurably move it; once they stall or parties cannot agree on a distribution, reclassify as deep and switch to robustness methods",
  "treating deep uncertainty as merely reducible-epistemic breeds overconfidence and false promises of reduction")
conf("DISCONFIRMATION","UNKNOWN_UNKNOWN",0.68,-0.55,
  "bound the search for unknown-unknowns with a stopping rule proportional to the decision's stakes; require each probe to be a structured premortem/red-team rather than open-ended speculation",
  "surfacing unknown-unknowns is valuable but the search is unbounded and can consume arbitrary effort")
conf("CONFIDENCE_STATE","CONFIDENCE_LEGALITY",0.66,-0.5,
  "express confidence at the granularity calibration supports (intervals or ordinal bands, not invented decimals); legality gates on bands so coarse-but-honest states are usable while false-precise ones are rejected",
  "precise numeric confidence states risk false precision yet downstream legality wants discriminating labels")

CA=[
 {"name":"exhaustive_gap_mapping_vs_analysis_cost","description":"Mapping every gap improves coverage but the search and typing cost is unbounded; stopping early risks missing a decision-relevant gap.","poles":["exhaustive_mapping","bounded_analysis"],"resolution_hint":"stop when marginal gap VoI falls below identification cost; prioritize within the bounded set","tension_score":0.72,"affected_nodes":["GAP_IDENTIFY","GAP_PRIORITIZE","BOUNDARY_OF_KNOWLEDGE"]},
 {"name":"reducible_treatment_vs_honest_deep_uncertainty","description":"Treating uncertainty as reducible-epistemic enables action but, applied to deep uncertainty, manufactures overconfidence; honest deep-uncertainty handling forgoes tidy point estimates.","poles":["reducible_optimization","robust_deep_handling"],"resolution_hint":"reclassify as deep once reduction stalls or distributions are contested; switch to robustness","tension_score":0.75,"affected_nodes":["EPISTEMIC","DEEP_UNCERTAINTY","ALEATORY"]},
 {"name":"surface_unknown_unknowns_vs_unbounded_search","description":"Probing for unknown-unknowns reduces surprise risk but the search has no natural bound and can consume arbitrary effort.","poles":["aggressive_probing","bounded_search"],"resolution_hint":"stopping rule proportional to stakes; structured premortem/red-team over open speculation","tension_score":0.7,"affected_nodes":["DISCONFIRMATION","UNKNOWN_UNKNOWN","BOUNDARY_OF_KNOWLEDGE"]},
 {"name":"fine_grained_confidence_vs_false_precision","description":"Fine-grained confidence states discriminate claims usefully but invite fabricated precision beyond what calibration supports.","poles":["fine_grained_confidence","calibrated_coarseness"],"resolution_hint":"express at the granularity calibration supports; gate legality on honest bands","tension_score":0.66,"affected_nodes":["CONFIDENCE_STATE","CONFIDENCE_LEGALITY","UNC_PROPAGATION"]},
 {"name":"aleatory_vs_epistemic_boundary","description":"The split between irreducible and reducible uncertainty is model-relative; a hard label can either waste reduction effort or entrench avoidable ignorance.","poles":["fixed_aleatory_label","model_relative_reassessment"],"resolution_hint":"record the system boundary under which a label holds; revisit when the model resolution changes","tension_score":0.65,"affected_nodes":["ALEATORY","EPISTEMIC","MODEL_UNC"]},
 {"name":"surface_tacit_vs_over_formalization","description":"Eliciting unknown-knowns removes silent assumptions but over-formalizing tacit skill produces brittle false rules.","poles":["aggressive_elicitation","preserve_tacit"],"resolution_hint":"elicit load-bearing tacit assumptions; test elicited rules against practice before relying on them","tension_score":0.6,"affected_nodes":["UNKNOWN_KNOWN","ASSUMPTION_REGISTRY","IGNORANCE_TYPING"]},
 {"name":"frontier_clarity_vs_false_closure","description":"A crisp boundary artifact aligns the team but implies the far side is empty, encouraging the false-closure error.","poles":["crisp_frontier","explicit_residual_margin"],"resolution_hint":"always render an explicit residual-ignorance margin beyond the drawn frontier","tension_score":0.62,"affected_nodes":["BOUNDARY_OF_KNOWLEDGE","UNKNOWN_UNKNOWN","MAP_MAINTENANCE"]},
 {"name":"proxy_usefulness_vs_proxy_drift","description":"A proxy makes an inaccessible quantity partially knowable but can decouple from its target (Goodhart) and mislead.","poles":["use_proxy","require_direct"],"resolution_hint":"record proxy bias and validity conditions; re-check correlation and watch for target-capture","tension_score":0.6,"affected_nodes":["PROXY_KNOWN","CONFIDENCE_STATE","GAP_IDENTIFY"]},
 {"name":"map_freshness_vs_update_churn","description":"Continuous re-typing keeps the map honest but over-frequent updates churn and destabilize the shared epistemic state.","poles":["continuous_update","stable_snapshot"],"resolution_hint":"re-type on material evidence only; batch minor signals to avoid churn","tension_score":0.58,"affected_nodes":["MAP_MAINTENANCE","CONFIDENCE_STATE","BOUNDARY_OF_KNOWLEDGE"]},
]

EC=[
 {"description":"An assumption is recorded but used downstream as if it were a justified known-known, so conclusions inherit false certainty.","trigger":"a conditional known consumed without checking its confidence label","affected_nodes":["ASSUMPTION_REGISTRY","CONFIDENCE_STATE","CONFIDENCE_LEGALITY"],"mitigation":"legality interface bars assumption-grade labels from supporting certainty-grade claims","severity":"high"},
 {"description":"A genuinely deep uncertainty is forced into a single point estimate, producing overconfident planning.","trigger":"deep uncertainty mislabeled as reducible-epistemic","affected_nodes":["DEEP_UNCERTAINTY","EPISTEMIC"],"mitigation":"reclassify as deep when reduction stalls or distributions are contested; use robustness methods","severity":"critical"},
 {"description":"The known/unknown map looks complete, so the team treats the recognized frontier as the whole territory and is blindsided by a surprise.","trigger":"no residual-ignorance margin or active probe recorded","affected_nodes":["UNKNOWN_UNKNOWN","BOUNDARY_OF_KNOWLEDGE","DISCONFIRMATION"],"mitigation":"always record a residual margin and run at least one disconfirmation probe","severity":"high"},
 {"description":"An item that is really an undefined term is routed to costly empirical acquisition instead of being defined.","trigger":"ambiguity not screened before gap identification","affected_nodes":["AMBIGUITY_UNC","GAP_IDENTIFY"],"mitigation":"screen and dissolve ambiguity by definition before listing a gap as empirical","severity":"medium"},
 {"description":"Effort is spent acquiring a low-impact unknown while a decision-flipping gap stays open.","trigger":"gaps not ranked by value of information","affected_nodes":["GAP_PRIORITIZE","GAP_IDENTIFY"],"mitigation":"rank gaps by decision-impact against cost; address top-VoI gaps first","severity":"high"},
 {"description":"A proxy that once tracked its target decouples (Goodhart) and silently misleads downstream reasoning.","trigger":"proxy correlation assumed stable; proxy becomes an optimization target","affected_nodes":["PROXY_KNOWN","CONFIDENCE_STATE"],"mitigation":"record validity conditions; re-check correlation; flag target-capture","severity":"medium"},
 {"description":"A conclusion is presented as firmer than its weakest load-bearing input because uncertainty was not propagated.","trigger":"inference drawn without combining input confidence states","affected_nodes":["UNC_PROPAGATION","CONFIDENCE_STATE"],"mitigation":"propagate so a conclusion is no tighter than its dominant input; show that input","severity":"high"},
 {"description":"Correlated inputs are propagated as independent, drastically understating combined uncertainty.","trigger":"independence assumed among correlated inputs","affected_nodes":["UNC_PROPAGATION","MEASUREMENT_UNC"],"mitigation":"check for correlation among inputs before combining; widen the interval when found","severity":"medium"},
 {"description":"Parameter intervals are tightened inside a structurally wrong model, giving false precision about the wrong thing.","trigger":"measurement uncertainty reduced without validating model structure","affected_nodes":["MEASUREMENT_UNC","MODEL_UNC"],"mitigation":"validate structure (rival models) before trusting tightened parameter intervals","severity":"medium"},
 {"description":"Tacit knowledge stays unsurfaced and a silent assumption drives the analysis unchallenged.","trigger":"unknown-known elicitation skipped","affected_nodes":["UNKNOWN_KNOWN","ASSUMPTION_REGISTRY"],"mitigation":"run elicitation on load-bearing items; register surfaced assumptions","severity":"high"},
 {"description":"The living map decays into a stale snapshot, so resolved gaps still show open and new evidence is ignored.","trigger":"map not maintained as evidence arrives","affected_nodes":["MAP_MAINTENANCE","BOUNDARY_OF_KNOWLEDGE"],"mitigation":"re-type on material evidence; retire resolved gaps; register new ones","severity":"medium"},
 {"description":"A speculative claim is consumed downstream as an experimentally-validated fact because no legality gate exists.","trigger":"downstream use not checked against input confidence labels","affected_nodes":["CONFIDENCE_LEGALITY","CONFIDENCE_STATE"],"mitigation":"enforce the legality interface mapping labels to permitted downstream uses","severity":"high"},
]

WF=[
 {"action":"separate_known_from_unknown","node_ref":"KNOWN_KNOWN","description":"For the question, separate justified known-knowns from everything else and record each known's justification.","artifact":"known_known_register","gate":"every known-known carries a justification and evidence reference"},
 {"action":"name_the_gaps","node_ref":"KNOWN_UNKNOWN","description":"Name each recognized gap as an answerable question with an acquisition or estimation handle.","artifact":"known_unknown_list","gate":"each gap is phrased as an answerable question"},
 {"action":"elicit_tacit_knowledge","node_ref":"UNKNOWN_KNOWN","description":"Elicit unknown-knowns: surface tacit assumptions and institutional memory into explicit testable claims.","artifact":"surfaced_assumptions","gate":"material tacit assumptions are made explicit"},
 {"action":"classify_uncertainty_nature","node_ref":"EPISTEMIC","description":"Classify each uncertain item as aleatory, epistemic, ambiguity, model, measurement, or deep.","artifact":"uncertainty_nature_tags","gate":"each item has a nature tag with its reduction route"},
 {"action":"screen_ambiguity","node_ref":"AMBIGUITY_UNC","description":"Screen for ambiguity and dissolve undefined terms by definition before any empirical acquisition is planned.","artifact":"definitions_register","gate":"no pseudo-gap (undefined term) remains on the empirical list"},
 {"action":"register_assumptions","node_ref":"ASSUMPTION_REGISTRY","description":"Record load-bearing assumptions as conditional knowns linked to the conclusions that depend on them.","artifact":"assumption_registry","gate":"every load-bearing assumption is registered with its dependents"},
 {"action":"type_confidence","node_ref":"CONFIDENCE_STATE","description":"Attach a calibrated confidence state with an evidence label and interval to each claim.","artifact":"confidence_annotations","gate":"each claim carries a calibrated confidence state, not a bare point"},
 {"action":"type_each_item","node_ref":"IGNORANCE_TYPING","description":"Assign every item a complete type: knowledge cell, uncertainty nature, and recognition flag.","artifact":"typed_item_map","gate":"every mapped item is fully typed"},
 {"action":"identify_and_prioritize_gaps","node_ref":"GAP_PRIORITIZE","description":"Enumerate decision-relevant gaps and rank them by value of information against acquisition cost.","artifact":"prioritized_gap_queue","gate":"top gaps ranked by VoI with justification"},
 {"action":"render_the_frontier","node_ref":"BOUNDARY_OF_KNOWLEDGE","description":"Render the known/unknown frontier with populated cells, open gaps, and an explicit residual-ignorance margin.","artifact":"frontier_map","gate":"frontier shows a residual-ignorance margin"},
 {"action":"probe_for_unknown_unknowns","node_ref":"DISCONFIRMATION","description":"Run a stakes-proportional premortem/red-team probe seeking disconfirming evidence and unasked questions.","artifact":"disconfirmation_findings","gate":"at least one bounded probe run with a stopping rule"},
 {"action":"propagate_and_gate","node_ref":"CONFIDENCE_LEGALITY","description":"Propagate uncertainty into conclusions and gate downstream use by confidence-label legality; then maintain the living map.","artifact":"gated_conclusions","gate":"no conclusion firmer than its inputs; downstream uses pass legality"},
]

DR=[
 {"rule":"AMBIGUITY_UNC must screen and dissolve undefined terms before GAP_IDENTIFY lists an item as an empirical gap","rationale":"an undefined term is dissolved by definition, not by costly acquisition","trigger":"a vague term is queued for empirical acquisition","action":"route to definition/operationalization before acquisition"},
 {"rule":"DEEP_UNCERTAINTY must override EPISTEMIC treatment once reduction efforts stall or the distribution is contested","rationale":"treating deep uncertainty as reducible breeds overconfidence","trigger":"a reduction effort fails to move the uncertainty","action":"reclassify as deep and switch to robustness/scenario handling"},
 {"rule":"CONFIDENCE_LEGALITY must gate any downstream use of a claim by the legality of its confidence label","rationale":"weak knowns consumed as certain corrupt downstream conclusions","trigger":"a claim is consumed downstream","action":"block uses whose inputs' labels are not legal for that use"},
 {"rule":"UNC_PROPAGATION must run before a conclusion is published, and the conclusion's interval must be no tighter than its dominant input","rationale":"un-propagated uncertainty launders shaky inputs into firm-looking conclusions","trigger":"a conclusion is drawn from multiple uncertain inputs","action":"propagate and clamp the conclusion to its dominant input's uncertainty"},
 {"rule":"BOUNDARY_OF_KNOWLEDGE must render an explicit residual-ignorance margin and at least one active probe before the map is treated as decision-ready","rationale":"a frontier with no margin invites the false-closure error","trigger":"the map is presented as complete","action":"require a residual margin and a DISCONFIRMATION probe"},
 {"rule":"ASSUMPTION_REGISTRY must record every load-bearing assumption with its dependent conclusions before those conclusions are finalized","rationale":"unrecorded load-bearing assumptions make conclusions silently fragile","trigger":"a conclusion rests on an unverified proposition","action":"register the assumption and link its dependents"},
 {"rule":"GAP_PRIORITIZE ranking by value of information governs which gaps GAP_IDENTIFY queues for acquisition","rationale":"acquisition effort is scarce and must buy decision-relevance","trigger":"acquisition is scheduled for a gap","action":"require the gap to clear a VoI-vs-cost threshold"},
 {"rule":"PROXY_KNOWN may stand in for an inaccessible target only with its bias and validity conditions recorded","rationale":"an uncharacterized proxy silently misleads downstream consumers","trigger":"a proxy is used for a target quantity","action":"require recorded bias and validity conditions before consumption"},
 {"rule":"IGNORANCE_TYPING must assign a complete type to every item before it is acted on","rationale":"untyped items get the wrong default treatment","trigger":"an item is queued for action without a type","action":"block action until cell, nature, and recognition are assigned"},
]

ARR=[
 {"rule":"Do not consume an assumption as a justified known; verify or label it first, or all dependent conclusions need rework when it breaks","prevents":"unwinding a conclusion chain after a silent assumption is invalidated"},
 {"rule":"Do not list an undefined term as an empirical gap; define it first, or you research a pseudo-gap and re-scope later","prevents":"wasted acquisition on a question that was only an ambiguity"},
 {"rule":"Do not force deep uncertainty into a point estimate; switching to robustness after an overconfident plan fails is costly","prevents":"replanning after an overconfident single-distribution bet fails"},
 {"rule":"Do not treat the recognized frontier as complete; discovering a missed question class late forces a full re-map","prevents":"emergency re-mapping after an unanticipated surprise"},
 {"rule":"Do not acquire low-VoI gaps before high-VoI ones; reordering after effort is spent wastes the acquisition budget","prevents":"redoing acquisition once a decision-flipping gap surfaces late"},
 {"rule":"Do not publish a conclusion without propagating uncertainty; retrofitting honest intervals after the fact invalidates downstream use","prevents":"recalling conclusions consumed downstream as falsely certain"},
 {"rule":"Do not rely on a proxy without recording its bias and validity; untangling a drifted proxy's influence later is expensive","prevents":"tracing and correcting decisions built on a decoupled proxy"},
 {"rule":"Do not let the map go stale; reconciling a long-unmaintained map with reality costs more than incremental re-typing","prevents":"a large reconciliation pass after the map diverges from evidence"},
]

IP=[
 {"trigger":"a realized surprise reveals a missed question class","action":"run DISCONFIRMATION probes and re-bound UNKNOWN_UNKNOWN, then re-render the frontier in BOUNDARY_OF_KNOWLEDGE","nodes":["DISCONFIRMATION","UNKNOWN_UNKNOWN","BOUNDARY_OF_KNOWLEDGE"],"priority":"critical"},
 {"trigger":"an uncertainty reduction effort stalls without moving the estimate","action":"reclassify the item from EPISTEMIC to DEEP_UNCERTAINTY and switch to robustness handling","nodes":["EPISTEMIC","DEEP_UNCERTAINTY"],"priority":"high"},
 {"trigger":"a weak claim is found to have been consumed downstream as certain","action":"tighten CONFIDENCE_LEGALITY rules and re-run UNC_PROPAGATION on affected conclusions","nodes":["CONFIDENCE_LEGALITY","UNC_PROPAGATION","CONFIDENCE_STATE"],"priority":"high"},
 {"trigger":"a load-bearing assumption is invalidated","action":"update ASSUMPTION_REGISTRY and re-type and re-confidence every dependent conclusion","nodes":["ASSUMPTION_REGISTRY","CONFIDENCE_STATE","MAP_MAINTENANCE"],"priority":"high"},
 {"trigger":"an empirical gap turns out to be an undefined term","action":"move it from GAP_IDENTIFY to AMBIGUITY_UNC and dissolve it by definition","nodes":["GAP_IDENTIFY","AMBIGUITY_UNC"],"priority":"medium"},
 {"trigger":"a proxy decouples from its target","action":"flag PROXY_KNOWN, discount the affected confidence states, and re-open the underlying gap","nodes":["PROXY_KNOWN","CONFIDENCE_STATE","GAP_IDENTIFY"],"priority":"medium"},
 {"trigger":"the decision or its payoffs change","action":"re-run GAP_PRIORITIZE value-of-information ranking and reorder the acquisition queue","nodes":["GAP_PRIORITIZE","GAP_IDENTIFY"],"priority":"medium"},
 {"trigger":"the living map drifts from current evidence","action":"run MAP_MAINTENANCE to re-type items, retire resolved gaps, and restore consistency","nodes":["MAP_MAINTENANCE","BOUNDARY_OF_KNOWLEDGE","IGNORANCE_TYPING"],"priority":"medium"},
]

spec = {
 "domain":"kumap__uncertainty_taxonomy",
 "domain_label":"Known/Unknown Mapping & Uncertainty Taxonomy (Knowledge Acquisition subdomain)",
 "purpose":"session_bounded_method_for_mapping_what_is_known_versus_unknown_about_a_question_by_typing_knowledge_cells_from_known_known_through_unknown_unknown_distinguishing_aleatory_from_epistemic_uncertainty_separating_model_measurement_ambiguity_and_deep_uncertainty_surfacing_assumptions_as_conditional_knowns_identifying_and_prioritizing_knowledge_gaps_by_value_of_information_and_actively_probing_for_unknown_unknowns_to_keep_a_living_calibrated_known_unknown_map",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "the stance is neutral and epistemic: the method types and maps uncertainty, it does not adjudicate the truth of domain claims",
   "scope is bounded to a single question or decision within a session; cross-session knowledge management is delegated to sibling KBs",
   "downstream evidence weighting and acquisition execution are consumers of this map, not part of it",
 ],
 "exclusions":[
   "executing the acquisition itself (queries, experiments, elicitation runs) — delegated to the acquisition pipeline",
   "domain-specific truth adjudication of any particular claim",
   "long-horizon organizational knowledge management and archival",
   "formal probability calculus and statistical estimation internals (assumed available as tools)",
 ],
 "source_description":"heuristic prior estimates for known/unknown mapping and uncertainty-taxonomy work units, informed by the Johari window, the risk/uncertainty distinction, the Walker et al. uncertainty taxonomy, and deep-uncertainty/robust-decision literature; no supplied dataset",
 "source_citation":"Luft & Ingham 1955 (Johari window); Knight 1921 Risk, Uncertainty and Profit (risk vs uncertainty); Walker, Harremoes, Rotmans, van der Sluijs, van Asselt, Janssen & Krayer von Krauss 2003 'Defining Uncertainty: A Conceptual Basis for Uncertainty Management in Model-Based Decision Support'; Lempert, Popper & Bankes 2003 (RAND) Shaping the Next One Hundred Years: robust decision making under deep uncertainty; Walker, Lempert & Kwakkel 2013 'Deep Uncertainty'; Taleb 2007 The Black Swan (unknown-unknowns, used with caution); Howard 1966 (value of information); Funtowicz & Ravetz 1990 (NUSAP / uncertainty in science)",
 "competency_questions":CQS,
 "glossary":GLS,
 "nodes":N,
 "edges":E,
 "conflict_axes":CA,
 "edge_cases":EC,
 "workflow":WF,
 "dominance_rules":DR,
 "anti_rework_rules":ARR,
 "iteration_protocol":IP,
 "priority_rationale":"the four knowledge cells (KNOWN_KNOWN..UNKNOWN_UNKNOWN) are foundational; uncertainty-nature typing and the annotation layer (assumptions, confidence, ignorance typing) build on them; the gap pipeline (identify, prioritize, frontier, proxy) and active epistemics (disconfirmation, propagation, maintenance, legality) close the method.",
 "eval_objective":"verify_cell_typing_aleatory_epistemic_separation_assumption_and_confidence_annotation_value_of_information_gap_prioritization_and_unknown_unknown_probing_of_kumap__uncertainty_taxonomy_kb",
}

out_dir = "branches/b11_knowledge_acquisition/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "kumap__uncertainty_taxonomy.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
