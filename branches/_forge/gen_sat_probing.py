#!/usr/bin/env python3
"""Generate the sat__structured_probing content spec (B11 KB) for kb_forge.py.
Compact authoring: node() applies sane defaults so only domain content + base
metric magnitudes are specified per node. Domain = Structured Analytic Techniques
run as knowledge probes that convert gaps into elicited, bias-mitigated findings."""
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
        "academic_fields": ["intelligence_analysis", "cognitive_psychology", "knowledge_acquisition"],
        "subfields": subfields or ["structured_analytic_techniques", "elicitation", "bias_mitigation"],
        "specialists": specialists or ["intelligence_analyst"],
        "contradictors": contradictors or ["intuitive_expert_advocate"],
        "inputs": inputs or ["analytic question", "knowledge gap statement"],
        "outputs": outputs or ["elicited bias-mitigated finding"],
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

N.append(node("TECHNIQUE_SELECT","sat_selection_by_question_and_gap_type",
  "Select which structured analytic techniques to deploy as probes, keyed to the analytic question type (explanation, estimate, warning, decision) and the gap type (missing hypothesis, hidden assumption, weak evidence, blind spot).",
  "foundations", [], ["CQ_01"],
  b(0.9,0.82,0.8,0.58,0.84,0.8,0.6,0.7,0.45, 0.78,0.82, 0.9,0.7,0.16,0.5,[0.2,0.5],"taxonomy of question types or gap types is revised"),
  ["question-type to technique mapping","gap-type to technique mapping","triage of which SATs apply"],
  ["running the chosen probe","report writing","collection tasking"],
  [pro("Matching the technique to the question type avoids ritualistic SAT use and spends analyst time where it discriminates","a warning question routes to indicators and what-if, not to ACH alone")],
  [con("A rigid selection rubric can mis-route a novel question and exclude a technique that would have helped","forcing an estimative question through a diagnostic-only template")],
  ["technique chosen by habit not by question type","gap type misdiagnosed so the wrong probe is run"],
  ["each selected technique is justified by a stated question type and gap type"],
  ["question-type taxonomy changes","a probe repeatedly yields nothing diagnostic"],
  specialists=["intelligence_analyst","knowledge_engineer"]))

N.append(node("HYPOTHESIS_GEN","competing_hypothesis_generation",
  "Generate a mutually exclusive, collectively near-exhaustive set of competing hypotheses for the question, deliberately including the deception/least-likely hypothesis, before any evidence is weighed.",
  "foundations", ["TECHNIQUE_SELECT"], ["CQ_02"],
  b(0.88,0.78,0.76,0.6,0.82,0.82,0.58,0.66,0.5, 0.74,0.78, 0.86,0.66,0.2,0.5,[0.22,0.55],"hypothesis space proves under-generated for a class of questions"),
  ["hypothesis enumeration","inclusion of the unlikely/deception hypothesis","MECE structuring of hypotheses"],
  ["scoring evidence","selecting a single answer"],
  [pro("Generating the full competing set first prevents anchoring on the first plausible story and makes ACH possible","listing 'coup, election, status quo, external intervention' before weighing any cable")],
  [con("An over-large hypothesis set dilutes diagnosticity and overloads the matrix","fifteen near-duplicate hypotheses no evidence can separate")],
  ["premature convergence to one favored hypothesis","an obvious hypothesis omitted from the set"],
  ["the hypothesis set is mutually exclusive and includes at least one disconfirmable rival"],
  ["a real outcome falls outside the generated set","analysts report the set felt forced"],
  specialists=["intelligence_analyst"]))

N.append(node("ACH","analysis_of_competing_hypotheses",
  "Run Heuer's Analysis of Competing Hypotheses: build a hypothesis-by-evidence matrix, rate each evidence item's consistency with each hypothesis, and seek to refute rather than confirm, ranking hypotheses by weight of disconfirming evidence.",
  "core", ["HYPOTHESIS_GEN"], ["CQ_02","CQ_03"],
  b(0.92,0.84,0.8,0.7,0.88,0.85,0.62,0.62,0.58, 0.72,0.76, 0.92,0.62,0.22,0.58,[0.24,0.6],"ACH matrix structure or refutation logic is revised"),
  ["hypothesis-evidence matrix","consistency/inconsistency rating","refutation-first ranking"],
  ["generating hypotheses (upstream)","final probability calibration (downstream)"],
  [pro("Refutation-first ranking counters confirmation bias by promoting the hypothesis with the least disconfirming evidence, not the most supporting","the favored hypothesis falls once a single highly inconsistent item is entered")],
  [con("ACH is sensitive to which evidence enters the matrix; selective entry re-imports the bias it aims to remove","leaving an inconvenient report out of the matrix")],
  ["matrix built to confirm a preferred hypothesis","non-diagnostic evidence inflates a hypothesis's apparent support"],
  ["each hypothesis is scored against every evidence item and ranked by disconfirmation"],
  ["new evidence materially changes the matrix","two hypotheses remain inseparable after scoring"],
  specialists=["intelligence_analyst","contrarian_reviewer"]))

N.append(node("DIAGNOSTICITY","evidence_diagnosticity_scoring",
  "Score the diagnosticity of each evidence item: how strongly it discriminates among the competing hypotheses, so analysis concentrates on the few items that actually move the conclusion rather than on volume.",
  "core", ["ACH"], ["CQ_03","CQ_04"],
  b(0.86,0.78,0.74,0.7,0.82,0.8,0.55,0.62,0.55, 0.72,0.76, 0.84,0.62,0.22,0.52,[0.24,0.58],"diagnosticity rubric or weighting scheme changes"),
  ["per-item diagnosticity rating","identification of high-leverage evidence","flagging non-diagnostic items"],
  ["evidence collection","source vetting (delegated to QOIC)"],
  [pro("Ranking by diagnosticity focuses effort on the handful of items that separate hypotheses, cutting analyst overload","one satellite image that only one hypothesis can survive outweighs ten consistent-with-all reports")],
  [con("Diagnosticity judgments are themselves uncertain; a mis-scored item can over- or under-drive the conclusion","treating a deceptive plant as highly diagnostic")],
  ["volume of consistent evidence mistaken for diagnostic weight","a deceptive item rated as highly diagnostic"],
  ["the most diagnostic evidence items are identified and drive the ranking"],
  ["a key item's diagnosticity is reassessed","deception is suspected for a high-leverage item"]))

N.append(node("KAC","key_assumptions_check",
  "Run the Key Assumptions Check: surface the load-bearing assumptions the current analytic line rests on, classify each as solid, caveated, or unsupported, and flag the ones that would overturn the conclusion if wrong.",
  "core", ["TECHNIQUE_SELECT"], ["CQ_05"],
  b(0.88,0.8,0.76,0.6,0.84,0.82,0.58,0.66,0.52, 0.74,0.78, 0.86,0.66,0.2,0.52,[0.22,0.54],"the analytic line or its premises change materially"),
  ["assumption surfacing","solid/caveated/unsupported classification","load-bearing assumption flagging"],
  ["testing the assumption empirically (handed to ASSUMPTION_TEST)","hypothesis scoring"],
  [pro("Making implicit assumptions explicit converts a hidden single point of failure into an inspectable, testable item","surfacing the unstated assumption that a regime's army will stay loyal")],
  [con("Endless assumption-listing can stall analysis without prioritizing which assumptions actually matter","cataloguing trivial assumptions while missing the decisive one")],
  ["the decisive assumption is never surfaced","assumptions listed but none classified by impact"],
  ["every load-bearing assumption is surfaced and classified by how badly it would damage the conclusion if false"],
  ["a new analytic line is adopted","an assumption previously rated solid is challenged"],
  specialists=["intelligence_analyst"]))

N.append(node("ASSUMPTION_TEST","assumption_falsification_test",
  "Test or attempt to falsify each flagged key assumption: identify the observable that would break it, seek that observable, and downgrade or caveat the conclusion when the assumption fails to survive.",
  "core", ["KAC"], ["CQ_05","CQ_06"],
  b(0.84,0.76,0.72,0.66,0.82,0.78,0.58,0.62,0.54, 0.74,0.78, 0.82,0.62,0.22,0.52,[0.24,0.58],"the breaking-observable for an assumption is redefined"),
  ["breaking-observable specification","falsification attempt","conclusion downgrade on failed assumption"],
  ["assumption surfacing (upstream)","scenario construction"],
  [pro("Tying each assumption to a breaking observable makes it empirically testable rather than rhetorically asserted","'the army stays loyal' is testable by watching for defections at named garrisons")],
  [con("Some load-bearing assumptions have no near-term observable, so the test cannot resolve them in time","an assumption about long-run intent with no near-term signal")],
  ["assumption assumed true because no test was run","conclusion not downgraded after an assumption broke"],
  ["each load-bearing assumption has a breaking observable and was actively tested, not assumed"],
  ["a breaking observable appears","an untested assumption is found to be decisive"]))

N.append(node("WHATIF","what_if_analysis",
  "Run What-If Analysis: assume a surprising but possible outcome has already occurred and reason backward to the pathway that produced it, surfacing precursors and indicators no one is currently tracking.",
  "techniques", ["TECHNIQUE_SELECT"], ["CQ_07"],
  b(0.8,0.74,0.74,0.6,0.78,0.78,0.55,0.64,0.5, 0.76,0.78, 0.8,0.64,0.2,0.48,[0.22,0.54],"the stipulated surprising outcome or its plausibility changes"),
  ["assume-the-outcome framing","backward pathway tracing","precursor/indicator surfacing"],
  ["probability estimation","collection tasking on the indicators"],
  [pro("Assuming the outcome has happened licenses imaginative back-casting that forward estimation suppresses","back-casting a sudden currency collapse reveals capital-flight precursors to watch")],
  [con("What-if can manufacture vivid but low-probability pathways that distract from the central estimate","an elaborate but far-fetched scenario crowding out the base case")],
  ["plausible precursors never surfaced","a vivid scenario treated as likely without base-rate grounding"],
  ["at least one trackable precursor is surfaced and converted into an indicator"],
  ["a surfaced precursor is observed","the stipulated outcome is reassessed as implausible"]))

N.append(node("PREMORTEM","pre_mortem_prospective_hindsight",
  "Run a Pre-Mortem (Klein's prospective hindsight): stipulate that the current judgment has already failed badly, then have the team explain why, surfacing overlooked failure modes and blind spots before commitment.",
  "techniques", ["TECHNIQUE_SELECT"], ["CQ_07","CQ_08"],
  b(0.82,0.76,0.76,0.58,0.8,0.78,0.55,0.64,0.55, 0.76,0.78, 0.82,0.64,0.2,0.5,[0.22,0.54],"the judgment under test or its failure framing changes"),
  ["stipulated-failure framing","group blind-spot elicitation","overlooked-failure-mode capture"],
  ["root-cause investigation of a real failure","decision authority"],
  [pro("Prospective hindsight raises the number and candor of identified failure modes versus asking 'what could go wrong'","imagining the estimate already wrong elicits the objection no one would raise live")],
  [con("A pre-mortem can devolve into pessimism theater that lists failures without weighting their likelihood","brainstorming dozens of failures with no prioritization")],
  ["failure modes listed but not prioritized","the exercise skipped because the team is confident"],
  ["the team produces and prioritizes overlooked failure modes for the stipulated failure"],
  ["a listed failure mode begins to materialize","the underlying judgment is revised"],
  specialists=["intelligence_analyst","decision_facilitator"]))

N.append(node("SCENARIO","scenario_analysis",
  "Construct a small set of internally consistent, divergent scenarios spanning the key uncertainties, so the estimate is stress-tested across futures rather than pinned to a single most-likely line.",
  "techniques", ["WHATIF"], ["CQ_08","CQ_09"],
  b(0.8,0.74,0.74,0.66,0.78,0.8,0.55,0.6,0.52, 0.72,0.76, 0.8,0.6,0.22,0.5,[0.26,0.62],"the driving uncertainties or scenario axes change"),
  ["key-uncertainty axis selection","divergent consistent scenarios","cross-scenario robustness check"],
  ["single-point forecasting","operational planning"],
  [pro("Spanning futures with a few divergent scenarios exposes which conclusions are robust and which are scenario-fragile","a recommendation that survives all four scenarios is flagged as robust")],
  [con("Too many scenarios or overlapping ones dilute the signal and exhaust the audience","eight scenarios that differ only cosmetically")],
  ["scenarios collapse onto the most-likely line","scenario set omits a plausible divergent future"],
  ["a small set of divergent, internally consistent scenarios spans the key uncertainties"],
  ["a driving uncertainty is resolved","reality tracks outside all scenarios"]))

N.append(node("INDICATORS","indicators_and_warning",
  "Define and monitor indicators (signposts): pre-specified, observable, falsifiable signs whose appearance would signal that a hypothesis or scenario is becoming true, converting blind spots into watched warnings.",
  "techniques", ["SCENARIO","WHATIF"], ["CQ_09","CQ_10"],
  b(0.84,0.78,0.78,0.62,0.82,0.82,0.6,0.64,0.5, 0.78,0.8, 0.84,0.64,0.18,0.5,[0.2,0.5],"the indicator list or its collection feasibility changes"),
  ["indicator specification","observability/falsifiability check","warning-threshold setting"],
  ["collection execution","analysis of the triggered indicator"],
  [pro("Pre-specifying indicators before the fact prevents post-hoc rationalization and creates an early-warning tripwire","a named troop-movement indicator fires before, not after, the escalation")],
  [con("Indicators that are not actually collectable give false reassurance of vigilance","a perfect indicator no sensor can observe")],
  ["indicators are not observable or collectable","an indicator fires but no one acts on it"],
  ["each indicator is observable, falsifiable, and mapped to the hypothesis or scenario it warns for"],
  ["an indicator fires","an indicator proves uncollectable in practice"],
  specialists=["intelligence_analyst","collection_manager"]))

N.append(node("HIGH_IMPACT_LOW_PROB","high_impact_low_probability_analysis",
  "Run High-Impact/Low-Probability analysis: examine a low-likelihood but high-consequence outcome on its own terms, identify the pathway and the indicators that would raise its probability, so it is not dismissed by base rates alone.",
  "techniques", ["WHATIF","INDICATORS"], ["CQ_10","CQ_11"],
  b(0.8,0.76,0.74,0.62,0.8,0.78,0.58,0.6,0.5, 0.74,0.76, 0.8,0.6,0.22,0.54,[0.24,0.58],"the outcome's consequence assessment or pathway changes"),
  ["low-probability high-consequence framing","enabling-pathway identification","probability-raising indicators"],
  ["routine most-likely estimation","resource allocation decisions"],
  [pro("Treating a tail risk on its own terms prevents base-rate neglect of catastrophes that the average case would bury","analyzing a low-odds but catastrophic supply-chain rupture explicitly")],
  [con("Over-attending to dramatic low-probability events can skew priorities away from the likely case","spending the analysis budget on a vivid but improbable scenario")],
  ["a catastrophic tail risk dismissed purely on low base rate","tail risk inflated into the central estimate"],
  ["the high-impact low-probability outcome has an explicit pathway and probability-raising indicators"],
  ["a probability-raising indicator appears","the consequence assessment is revised"]))

N.append(node("OUTSIDE_VIEW","outside_view_reference_class_base_rate",
  "Take the outside view: place the case in a reference class and anchor the estimate on the class base rate before adjusting for case specifics, countering the inside-view tendency to over-rely on the vivid particulars of this case.",
  "calibration", ["TECHNIQUE_SELECT"], ["CQ_12"],
  b(0.86,0.8,0.78,0.64,0.82,0.82,0.58,0.62,0.55, 0.72,0.76, 0.86,0.62,0.22,0.54,[0.24,0.6],"the chosen reference class or its base rate changes"),
  ["reference-class selection","base-rate anchoring","inside-view adjustment under a cap"],
  ["case-specific evidence weighting (inside view)","final report"],
  [pro("Anchoring on a reference-class base rate corrects the inside view's systematic optimism and neglect of base rates","forecasting project slippage from the base rate of similar projects, not the team's plan")],
  [con("A poorly chosen or unrepresentative reference class imports its own bias and can mislead more than the inside view","using an irrelevant class whose base rate does not apply")],
  ["base-rate neglect: estimate built only from case particulars","reference class chosen to support a desired number"],
  ["the estimate is anchored on a defensible reference-class base rate before case-specific adjustment"],
  ["a better-fitting reference class is found","the base rate is updated with new class data"],
  specialists=["intelligence_analyst","forecaster"]))

N.append(node("QOIC","quality_of_information_check",
  "Run a Quality-of-Information Check: audit the provenance, currency, corroboration, and independence of each evidence source, flagging illusory corroboration, single-source dependence, and stale or deceptive material before it drives conclusions.",
  "calibration", ["TECHNIQUE_SELECT"], ["CQ_13"],
  b(0.88,0.8,0.78,0.64,0.86,0.82,0.6,0.64,0.55, 0.74,0.78, 0.88,0.64,0.2,0.55,[0.22,0.54],"the source base or its independence structure changes"),
  ["provenance and currency audit","independence/corroboration check","deception and single-source flagging"],
  ["diagnosticity scoring (downstream)","collection tasking"],
  [pro("Auditing source independence exposes circular reporting where several streams trace to one origin, deflating false corroboration","three 'independent' reports that all derive from one informant are collapsed to one")],
  [con("Aggressive sourcing scrutiny can discard genuinely useful but imperfectly provenanced evidence","rejecting a timely report because its chain is not fully documented")],
  ["illusory corroboration from circular reporting","stale or deceptive source treated as current and reliable"],
  ["every evidence item carries a provenance, currency, and independence rating before it is weighed"],
  ["a source is found to be circular or deceptive","new sourcing changes an item's reliability"],
  specialists=["intelligence_analyst","source_validator"]))

N.append(node("DEVILS_ADVOCATE","devils_advocacy_red_team",
  "Assign a devil's advocate / red team to build the strongest case against the prevailing analytic line, stress-testing it for groupthink and surfacing the disconfirming evidence the consensus has discounted.",
  "challenge", ["ACH","KAC"], ["CQ_14"],
  b(0.84,0.78,0.76,0.64,0.84,0.82,0.58,0.6,0.6, 0.7,0.74, 0.84,0.6,0.24,0.56,[0.26,0.62],"the prevailing analytic line or team composition changes"),
  ["adversarial case construction","groupthink stress test","surfacing discounted disconfirming evidence"],
  ["reaching consensus","authoring the mainline estimate"],
  [pro("A standing devil's advocate institutionalizes dissent, breaking the consensus that suppresses inconvenient evidence","the red cell forces the team to confront the report it had explained away")],
  [con("Routine or token devil's advocacy breeds cynicism and can corrode team cohesion if it never changes outcomes","an advocate whose objections are ritually overruled")],
  ["dissent suppressed by group pressure","devil's advocacy is ritual and never affects the conclusion"],
  ["the strongest opposing case is built and explicitly adjudicated, not dismissed"],
  ["consensus forms with no recorded dissent","the opposing case materially weakens the mainline"],
  specialists=["intelligence_analyst","red_team_lead"],
  contradictors=["consensus_builder"]))

N.append(node("RED_HAT","red_hat_perspective_taking",
  "Run Red Hat analysis: model the decision from the adversary's or another actor's frame of reference and incentives, rather than mirror-imaging one's own logic onto them, to surface choices the home frame would not predict.",
  "challenge", ["DEVILS_ADVOCATE"], ["CQ_14","CQ_15"],
  b(0.8,0.74,0.74,0.66,0.8,0.78,0.55,0.6,0.55, 0.72,0.74, 0.8,0.6,0.22,0.52,[0.26,0.6],"the modeled actor's incentives or culture are reassessed"),
  ["actor-frame role-play","incentive and culture modeling","anti-mirror-imaging check"],
  ["own-side planning","intelligence collection"],
  [pro("Adopting the actor's own frame counters mirror-imaging and predicts choices that look irrational only from the home frame","modeling an adversary who values regime survival over economic rationality")],
  [con("Role-play can substitute caricature for genuine understanding of the actor's frame","a cartoonish stand-in that confirms existing expectations")],
  ["mirror-imaging: own logic projected onto the actor","caricature mistaken for the actor's real frame"],
  ["the actor's likely choice is derived from their own incentives and frame, not the analyst's"],
  ["new evidence about the actor's incentives appears","a predicted choice is contradicted by behavior"]))

N.append(node("DECONFLICT","structured_brainstorming_and_deconfliction",
  "Run structured brainstorming and deconfliction: diverge to surface the widest set of ideas and hypotheses without premature criticism, then converge by clustering, deduplicating, and resolving contradictions across analysts.",
  "process", ["HYPOTHESIS_GEN"], ["CQ_02","CQ_16"],
  b(0.78,0.72,0.72,0.6,0.74,0.78,0.5,0.64,0.5, 0.74,0.76, 0.78,0.64,0.2,0.46,[0.22,0.52],"the divergence/convergence protocol changes"),
  ["silent divergent ideation","clustering and deduplication","cross-analyst contradiction resolution"],
  ["scoring evidence","final adjudication"],
  [pro("Separating silent divergence from convergence prevents early anchoring and dominant-voice capture in group ideation","round-robin silent generation before discussion surfaces minority hypotheses")],
  [con("Poorly facilitated brainstorming reverts to anchoring on the first or loudest idea","the senior analyst's opinion framing the whole session")],
  ["dominant voice anchors the group early","contradictions left unresolved across analysts"],
  ["divergence is silent and unjudged before convergence clusters and deconflicts the ideas"],
  ["a new analyst joins the cell","contradictory hypotheses persist after convergence"]))

N.append(node("BIAS_MITIGATION","cognitive_bias_mitigation",
  "Apply targeted cognitive-bias mitigation: name the biases most likely in this question (confirmation, anchoring, availability, base-rate neglect) and pair each with the specific technique or check that counters it, so debiasing is mechanism-specific not generic.",
  "process", ["TECHNIQUE_SELECT","ACH","OUTSIDE_VIEW"], ["CQ_03","CQ_17"],
  b(0.86,0.8,0.78,0.66,0.86,0.85,0.58,0.6,0.6, 0.7,0.74, 0.86,0.6,0.24,0.58,[0.26,0.62],"the bias inventory or the technique-to-bias mapping changes"),
  ["bias inventory for the question","technique-to-bias mapping","residual-bias caveat"],
  ["debiasing claims of certainty","performance scoring of analysts"],
  [pro("Pairing each likely bias with the specific counter-technique makes debiasing mechanism-specific instead of an unfalsifiable claim","confirmation bias countered by ACH refutation; base-rate neglect by the outside view")],
  [con("Bias-awareness alone does not remove bias and can produce overconfidence that debiasing has succeeded","a team that believes naming a bias has neutralized it")],
  ["debiasing claimed without any specific mechanism","awareness mistaken for mitigation"],
  ["each material bias is named and paired with the concrete technique or check that counters it"],
  ["a new bias is implicated by an error review","a counter-technique proves ineffective"],
  specialists=["intelligence_analyst","cognitive_scientist"]))

N.append(node("ELICITATION","structured_expert_elicitation_protocol",
  "Run a structured expert-elicitation protocol: frame unambiguous questions, elicit independent judgments before discussion to avoid herding, decompose complex quantities, and document rationales, converting a knowledge gap into a defensible aggregated judgment.",
  "process", ["TECHNIQUE_SELECT","QOIC"], ["CQ_13","CQ_18"],
  b(0.84,0.78,0.76,0.7,0.82,0.8,0.58,0.6,0.55, 0.72,0.76, 0.84,0.6,0.22,0.54,[0.26,0.62],"the elicitation protocol or expert panel changes"),
  ["unambiguous question framing","independent-then-shared elicitation","decomposition and rationale capture"],
  ["technique selection (upstream)","calibration training (separate node)"],
  [pro("Eliciting independent judgments before discussion prevents anchoring and herding on a dominant expert","each expert records an estimate privately before the panel talks")],
  [con("Aggregating poorly calibrated or correlated experts can produce confident but wrong consensus","five experts who all read the same flawed report")],
  ["herding on the first or most senior expert","correlated experts mistaken for independent confirmation"],
  ["judgments are elicited independently before discussion and each carries a documented rationale"],
  ["the expert panel changes","experts are found to share a common source"],
  specialists=["intelligence_analyst","domain_expert","facilitator"]))

N.append(node("CALIBRATION_TRAIN","probability_judgment_calibration",
  "Calibrate probability judgments: train and score analysts so stated confidence matches observed hit rates, use clear probability vocabulary, and track Brier scores so the elicited probabilities are trustworthy, not merely confident.",
  "calibration", ["ELICITATION","OUTSIDE_VIEW"], ["CQ_12","CQ_19"],
  b(0.84,0.8,0.78,0.68,0.84,0.8,0.6,0.6,0.55, 0.72,0.76, 0.84,0.6,0.22,0.56,[0.26,0.62],"the calibration scoring scheme or probability lexicon changes"),
  ["confidence-to-frequency calibration","probability vocabulary standardization","Brier-score tracking"],
  ["generating the substantive judgment","collection"],
  [pro("Calibration training measurably narrows the gap between stated confidence and actual hit rate over repeated forecasts","tracked Brier scores improve as analysts learn to widen overconfident intervals")],
  [con("Calibration needs repeated, resolvable forecasts and timely feedback, which many one-off analytic questions never get","a unique geopolitical event that never recurs for scoring")],
  ["overconfidence: stated probabilities exceed observed hit rates","vague verbal probabilities with no numeric anchor"],
  ["stated probabilities are scored against outcomes and the calibration gap is tracked over time"],
  ["a batch of forecasts resolves","systematic over- or under-confidence is detected"],
  specialists=["intelligence_analyst","forecaster"]))

N.append(node("PROBE_TO_GAP","probe_to_knowledge_gap_mapping",
  "Map each probe to the specific knowledge gap it addresses: record, per gap, which technique was run, what it elicited, and whether the gap is now closed, narrowed, or confirmed as a residual unknown, producing a gap-by-probe traceability record.",
  "synthesis", ["ACH","KAC","WHATIF","INDICATORS","QOIC"], ["CQ_06","CQ_20"],
  b(0.9,0.84,0.82,0.66,0.86,0.85,0.62,0.62,0.55, 0.76,0.8, 0.9,0.62,0.2,0.56,[0.22,0.54],"the gap taxonomy or the probe-to-gap mapping changes"),
  ["gap-to-probe assignment","elicited-finding capture","gap status (closed/narrowed/residual) record"],
  ["running the probes (upstream)","collection tasking"],
  [pro("A gap-by-probe record makes coverage auditable: every identified gap shows which probe addressed it and what changed","a gap table where each row names the gap, the probe, and the residual uncertainty")],
  [con("Forcing every gap onto a probe can give false closure when the probe only partially addressed it","marking a gap closed because a probe ran, not because it resolved")],
  ["a knowledge gap left with no probe assigned","a gap marked closed without an elicited finding to justify it"],
  ["every identified knowledge gap maps to at least one probe and a recorded status and finding"],
  ["a new knowledge gap is identified","a probe's finding is overturned by later evidence"],
  specialists=["intelligence_analyst","knowledge_engineer"]))

N.append(node("PROBE_BATTERY","probe_battery_composition",
  "Compose the probe battery for a question: sequence and bound the selected techniques into a session-scoped plan ordered by value of information, so the most decision-relevant probes run first within the analysis deadline.",
  "synthesis", ["PROBE_TO_GAP","BIAS_MITIGATION","DECONFLICT","CALIBRATION_TRAIN"], ["CQ_01","CQ_20"],
  b(0.9,0.84,0.82,0.7,0.86,0.85,0.62,0.6,0.58, 0.74,0.78, 0.9,0.6,0.22,0.58,[0.24,0.6],"the value-of-information ranking or the session time budget changes"),
  ["value-of-information ordering","session-bounded probe sequencing","battery completeness/stop check"],
  ["running individual probes (delegated to each technique node)","decision-making"],
  [pro("Ordering the battery by value of information delivers the most decision-relevant findings before the deadline forces a stop","running the diagnosticity-cutting probe before lower-value checks when time is short")],
  [con("A maximal battery can overrun the decision deadline and dilute focus across low-value probes","running all techniques on every question regardless of payoff")],
  ["battery overruns the decision deadline","low-value probes crowd out the decisive one"],
  ["the battery is ordered by value of information and bounded to fit the analysis deadline"],
  ["the decision deadline changes","a probe's realized value differs sharply from its expected value"],
  specialists=["intelligence_analyst","analytic_lead"]))

# ---- competency questions (target 10-14; author 20 CQs would exceed; cap at 14) ----
CQ = [
 ("CQ_01","How are structured analytic techniques selected and composed into a session-bounded probe battery for a given question and gap?",["nodes","workflow"],"TECHNIQUE_SELECT maps question/gap type to techniques; PROBE_BATTERY orders and bounds them",["TECHNIQUE_SELECT","PROBE_BATTERY"]),
 ("CQ_02","How is a competing-hypothesis set generated and structured before evidence is weighed?",["nodes"],"HYPOTHESIS_GEN produces a MECE set including the unlikely hypothesis; DECONFLICT widens it; ACH consumes it",["HYPOTHESIS_GEN","ACH","DECONFLICT"]),
 ("CQ_03","How does ACH counter confirmation bias and what role does diagnosticity play?",["nodes"],"ACH ranks by refutation; DIAGNOSTICITY scores discriminating evidence; BIAS_MITIGATION pairs the counter",["ACH","DIAGNOSTICITY","BIAS_MITIGATION"]),
 ("CQ_04","How is the diagnosticity of evidence scored and used to focus analysis?",["nodes"],"DIAGNOSTICITY rates each item by how strongly it discriminates among hypotheses",["DIAGNOSTICITY","ACH"]),
 ("CQ_05","How are key assumptions surfaced, classified, and tested?",["nodes"],"KAC surfaces and classifies load-bearing assumptions; ASSUMPTION_TEST falsifies them",["KAC","ASSUMPTION_TEST"]),
 ("CQ_06","How is each probe's finding tied back to the assumption or gap it was meant to resolve?",["nodes","traceability_matrix"],"ASSUMPTION_TEST downgrades on failure; PROBE_TO_GAP records the gap status and finding",["ASSUMPTION_TEST","PROBE_TO_GAP"]),
 ("CQ_07","How do what-if and pre-mortem techniques surface blind spots and precursors?",["nodes"],"WHATIF back-casts a surprising outcome; PREMORTEM stipulates failure to elicit overlooked failure modes",["WHATIF","PREMORTEM"]),
 ("CQ_08","How are divergent scenarios constructed and stress-tested?",["nodes"],"SCENARIO spans key uncertainties with divergent consistent futures; PREMORTEM stresses the judgment",["SCENARIO","PREMORTEM"]),
 ("CQ_09","How are scenarios and hypotheses converted into observable, falsifiable indicators?",["nodes","workflow"],"INDICATORS pre-specifies observable signposts mapped to SCENARIO and hypotheses",["INDICATORS","SCENARIO"]),
 ("CQ_10","How is a high-impact low-probability outcome analyzed without base-rate dismissal?",["nodes"],"HIGH_IMPACT_LOW_PROB examines the tail risk's pathway; INDICATORS warns of probability-raising signs",["HIGH_IMPACT_LOW_PROB","INDICATORS"]),
 ("CQ_11","How does the method balance attention between the most-likely case and tail risks?",["nodes","conflict_axes"],"HIGH_IMPACT_LOW_PROB frames the tail explicitly while the battery bounds attention by value of information",["HIGH_IMPACT_LOW_PROB","PROBE_BATTERY"]),
 ("CQ_12","How is the outside view and reference-class base rate brought in, and how are the resulting probabilities calibrated?",["nodes"],"OUTSIDE_VIEW anchors on a reference class; CALIBRATION_TRAIN scores confidence against outcomes",["OUTSIDE_VIEW","CALIBRATION_TRAIN"]),
 ("CQ_13","How is the quality and independence of the evidence base audited, and how does it feed elicitation?",["nodes"],"QOIC audits provenance and independence; ELICITATION uses vetted evidence in independent judgments",["QOIC","ELICITATION"]),
 ("CQ_14","How are devil's advocacy and perspective-taking used to break groupthink and mirror-imaging?",["nodes","conflict_axes"],"DEVILS_ADVOCATE builds the opposing case; RED_HAT models the actor's own frame",["DEVILS_ADVOCATE","RED_HAT"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

# consolidate node->CQ references onto the 14-CQ set
CQ_MAP = {
 "TECHNIQUE_SELECT":["CQ_01"], "HYPOTHESIS_GEN":["CQ_02"], "ACH":["CQ_03"],
 "DIAGNOSTICITY":["CQ_04"], "KAC":["CQ_05"], "ASSUMPTION_TEST":["CQ_05","CQ_06"],
 "WHATIF":["CQ_07"], "PREMORTEM":["CQ_07","CQ_08"], "SCENARIO":["CQ_08","CQ_09"],
 "INDICATORS":["CQ_09","CQ_10"], "HIGH_IMPACT_LOW_PROB":["CQ_10","CQ_11"],
 "OUTSIDE_VIEW":["CQ_12"], "QOIC":["CQ_13"], "DEVILS_ADVOCATE":["CQ_14"],
 "RED_HAT":["CQ_14"], "DECONFLICT":["CQ_02"], "BIAS_MITIGATION":["CQ_03"],
 "ELICITATION":["CQ_13"], "CALIBRATION_TRAIN":["CQ_12"], "PROBE_TO_GAP":["CQ_06"],
 "PROBE_BATTERY":["CQ_01","CQ_11"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

GL = [
 ("structured_analytic_technique","a documented, repeatable analytic method that externalizes reasoning to mitigate cognitive bias",["SAT"],["unaided_intuition"],["TECHNIQUE_SELECT","PROBE_BATTERY"]),
 ("knowledge_probe","a technique applied to a specific gap to elicit a finding, treated as an operator that converts a gap into a recorded result",["analytic_probe"],["passive_review"],["TECHNIQUE_SELECT","PROBE_TO_GAP"]),
 ("competing_hypotheses","a mutually exclusive, near-exhaustive set of explanations weighed against all evidence",["hypothesis_set"],["single_working_hypothesis"],["HYPOTHESIS_GEN","ACH"]),
 ("diagnosticity","the degree to which an evidence item discriminates among competing hypotheses",["discriminating_power"],["mere_consistency"],["DIAGNOSTICITY","ACH"]),
 ("key_assumption","a load-bearing premise whose falsity would overturn the conclusion",["load_bearing_assumption"],["incidental_assumption"],["KAC","ASSUMPTION_TEST"]),
 ("indicator","a pre-specified observable sign whose appearance warns that a hypothesis or scenario is becoming true",["signpost","warning_sign"],["post_hoc_explanation"],["INDICATORS","HIGH_IMPACT_LOW_PROB"]),
 ("outside_view","an estimate anchored on a reference-class base rate before case-specific adjustment",["reference_class_forecast"],["inside_view"],["OUTSIDE_VIEW","CALIBRATION_TRAIN"]),
 ("circular_reporting","apparent corroboration in which multiple sources trace back to a single origin",["illusory_corroboration"],["independent_corroboration"],["QOIC","ELICITATION"]),
 ("calibration","agreement between stated confidence and observed frequency of being correct",["confidence_accuracy"],["mere_confidence"],["CALIBRATION_TRAIN","OUTSIDE_VIEW"]),
 ("value_of_information","the expected decision-relevant payoff of running a probe, used to order the battery",["VOI"],["probe_volume"],["PROBE_BATTERY","PROBE_TO_GAP"]),
]
GLS=[{"term":t,"definition":d,"synonyms":s,"not_same_as":ns,"used_by_nodes":u} for (t,d,s,ns,u) in GL]

# ---- edges: dependency edges mirror node.dependencies (DAG) + cross-cutting non-dependency edges ----
E=[]
def dep(f,t,rs,cc=0.82,erc=0.28,cp=0.14,why="",ben="",rk="",ex=""):
    E.append({"from":f,"to":t,"edge_type":"dependency","relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{t} depends on {f}","benefit_of_coupling":ben or "ordered prerequisite",
              "risk_of_conflict":rk or "downstream rework if upstream changes","example":ex or f"{f} completed before {t}"})
def conf(f,t,rs,st,rule,why,cp=0.5,erc=0.5,cc=0.6):
    E.append({"from":f,"to":t,"edge_type":"conflict","relation_strength":rs,"signed_tension":st,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "resolution_rule":rule,"why_related":why,"benefit_of_coupling":"tension surfaced and resolved by rule",
              "risk_of_conflict":"unmanaged tension degrades analytic quality","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated behavior",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies)
dep("TECHNIQUE_SELECT","HYPOTHESIS_GEN",0.86)
dep("HYPOTHESIS_GEN","ACH",0.88)
dep("ACH","DIAGNOSTICITY",0.84)
dep("TECHNIQUE_SELECT","KAC",0.82)
dep("KAC","ASSUMPTION_TEST",0.84)
dep("TECHNIQUE_SELECT","WHATIF",0.8)
dep("TECHNIQUE_SELECT","PREMORTEM",0.8)
dep("WHATIF","SCENARIO",0.78)
dep("SCENARIO","INDICATORS",0.8)
dep("WHATIF","HIGH_IMPACT_LOW_PROB",0.76)
dep("INDICATORS","HIGH_IMPACT_LOW_PROB",0.74)
dep("TECHNIQUE_SELECT","OUTSIDE_VIEW",0.82)
dep("TECHNIQUE_SELECT","QOIC",0.84)
dep("ACH","DEVILS_ADVOCATE",0.78)
dep("DEVILS_ADVOCATE","RED_HAT",0.76)
dep("HYPOTHESIS_GEN","DECONFLICT",0.76)
dep("TECHNIQUE_SELECT","BIAS_MITIGATION",0.78)
dep("TECHNIQUE_SELECT","ELICITATION",0.78)
dep("QOIC","ELICITATION",0.76)
dep("ELICITATION","CALIBRATION_TRAIN",0.78)
dep("ACH","PROBE_TO_GAP",0.8)
dep("KAC","PROBE_TO_GAP",0.76)
dep("INDICATORS","PROBE_TO_GAP",0.74)
dep("QOIC","PROBE_TO_GAP",0.74)
dep("PROBE_TO_GAP","PROBE_BATTERY",0.86)
dep("BIAS_MITIGATION","PROBE_BATTERY",0.74)
dep("CALIBRATION_TRAIN","PROBE_BATTERY",0.74)
# cross-cutting non-dependency edges
rel("DIAGNOSTICITY","QOIC","constraint",0.76,why="diagnosticity weighting must be discounted by QOIC source-quality and independence ratings")
rel("PROBE_TO_GAP","TECHNIQUE_SELECT","feedback",0.76,why="residual gaps after probing feed back to re-select techniques for the next pass")
rel("CALIBRATION_TRAIN","ACH","feedback",0.72,why="calibration scores feed back to temper the confidence ACH rankings are reported with")
rel("ASSUMPTION_TEST","ACH","causal",0.74,why="a broken key assumption forces re-rating of the ACH matrix it supported")
rel("RED_HAT","HYPOTHESIS_GEN","feedback",0.72,why="the adversary frame surfaces hypotheses to add back into the competing set")
rel("INDICATORS","CALIBRATION_TRAIN","causal",0.7,why="resolved indicators provide the outcomes that calibrate probability judgments")
rel("DEVILS_ADVOCATE","DIAGNOSTICITY","causal",0.72,why="the opposing case surfaces discounted evidence whose diagnosticity must be re-scored")
rel("BIAS_MITIGATION","ELICITATION","constraint",0.72,why="bias counters constrain how expert judgments are elicited and aggregated")

# conflict edges (negative signed_tension + resolution_rule) -- real tensions
conf("PROBE_BATTERY","TECHNIQUE_SELECT",0.7,-0.6,
  "bound the battery by the decision deadline: when full SAT rigor cannot complete in time, run only the highest value-of-information probes and record the un-run techniques as accepted residual risk",
  "technique rigor and time-cost conflict with the decision deadline that bounds the analysis session")
conf("DIAGNOSTICITY","ACH",0.68,-0.55,
  "cap the ACH matrix at the most diagnostic evidence: when the full matrix overloads analysts, restrict scoring to high-diagnosticity items and explicitly note the truncation rather than abandoning ACH",
  "exhaustive ACH diagnosticity scoring conflicts with analyst overload and the cost of a large matrix")
conf("DEVILS_ADVOCATE","DECONFLICT",0.68,-0.55,
  "institutionalize dissent without corroding cohesion: rotate the devil's-advocate role and require the team to adjudicate the opposing case on the record, so challenge is structural rather than personal",
  "devil's advocacy needed against groupthink conflicts with team cohesion and consensus-building")
conf("PROBE_BATTERY","PROBE_TO_GAP",0.66,-0.5,
  "order probes by value of information and stop at diminishing returns: run additional probes only while their expected gap-resolution exceeds their cost, rather than mapping every gap to a probe",
  "running many probes for full gap coverage conflicts with focused value-of-information and a stop rule")

CA=[
 {"name":"technique_rigor_vs_decision_deadline","description":"Full SAT rigor improves quality but the time cost can miss the decision deadline that bounds the session.","poles":["maximal_rigor","timely_delivery"],"resolution_hint":"order probes by value of information and stop at the deadline, recording un-run techniques as residual risk","tension_score":0.75,"affected_nodes":["PROBE_BATTERY","TECHNIQUE_SELECT","PROBE_TO_GAP"]},
 {"name":"ach_diagnosticity_vs_analyst_overload","description":"Scoring every evidence item against every hypothesis is thorough but can overload analysts and dilute focus.","poles":["exhaustive_matrix","focused_diagnostic_subset"],"resolution_hint":"restrict the matrix to high-diagnosticity items and note the truncation","tension_score":0.7,"affected_nodes":["ACH","DIAGNOSTICITY","PROBE_BATTERY"]},
 {"name":"devils_advocacy_vs_team_cohesion","description":"Institutionalized dissent breaks groupthink but routine adversarial challenge can corrode team cohesion.","poles":["standing_dissent","team_cohesion"],"resolution_hint":"rotate the role and adjudicate the opposing case on the record","tension_score":0.7,"affected_nodes":["DEVILS_ADVOCATE","DECONFLICT","RED_HAT"]},
 {"name":"probe_coverage_vs_value_of_information","description":"Mapping every gap to a probe maximizes coverage but can run low-value probes past the point of diminishing returns.","poles":["full_gap_coverage","value_of_information_stop"],"resolution_hint":"order by value of information and stop when expected resolution falls below cost","tension_score":0.68,"affected_nodes":["PROBE_TO_GAP","PROBE_BATTERY","TECHNIQUE_SELECT"]},
 {"name":"inside_view_vs_outside_view","description":"Case-specific detail captures particulars but neglects base rates; the outside view anchors on base rates but can ignore genuine specifics.","poles":["inside_view_detail","outside_view_base_rate"],"resolution_hint":"anchor on the reference-class base rate, then adjust for case specifics under a cap","tension_score":0.66,"affected_nodes":["OUTSIDE_VIEW","CALIBRATION_TRAIN","DIAGNOSTICITY"]},
 {"name":"hypothesis_breadth_vs_matrix_tractability","description":"A wide hypothesis set reduces the chance of omission but a large set dilutes diagnosticity and overloads the ACH matrix.","poles":["wide_hypothesis_set","tractable_set"],"resolution_hint":"generate broadly, then deconflict to a MECE set that keeps the disconfirmable rival","tension_score":0.62,"affected_nodes":["HYPOTHESIS_GEN","DECONFLICT","ACH"]},
 {"name":"source_scrutiny_vs_timeliness","description":"Deep quality-of-information vetting deflates illusory corroboration but can discard timely evidence whose provenance is imperfect.","poles":["deep_source_vetting","timely_use_of_evidence"],"resolution_hint":"rate provenance and independence, caveat rather than discard timely but thin sources","tension_score":0.62,"affected_nodes":["QOIC","DIAGNOSTICITY","ELICITATION"]},
 {"name":"expert_independence_vs_shared_understanding","description":"Independent elicitation prevents herding but experts then lack the shared context that discussion provides.","poles":["independent_judgment","shared_deliberation"],"resolution_hint":"elicit independently first, then share and allow documented revision","tension_score":0.6,"affected_nodes":["ELICITATION","DECONFLICT","CALIBRATION_TRAIN"]},
 {"name":"tail_risk_attention_vs_central_estimate","description":"Explicit high-impact low-probability analysis avoids base-rate neglect but over-attending to the tail can skew priorities from the likely case.","poles":["tail_risk_focus","central_estimate_focus"],"resolution_hint":"analyze the tail on its own terms but weight battery attention by value of information","tension_score":0.6,"affected_nodes":["HIGH_IMPACT_LOW_PROB","WHATIF","PROBE_BATTERY"]},
]

EC=[
 {"description":"All probes confirm the favored hypothesis because inconvenient evidence was never entered into the ACH matrix.","trigger":"selective evidence entry re-imports confirmation bias","affected_nodes":["ACH","DIAGNOSTICITY","BIAS_MITIGATION"],"mitigation":"enforce refutation-first scoring and require every collected item be entered or explicitly excluded with reason","severity":"critical"},
 {"description":"A decisive load-bearing assumption is never surfaced, so a confident conclusion rests on an untested premise.","trigger":"key assumptions check omits the decisive assumption","affected_nodes":["KAC","ASSUMPTION_TEST","PROBE_TO_GAP"],"mitigation":"classify assumptions by impact and require a breaking observable for each load-bearing one","severity":"high"},
 {"description":"Several 'independent' reports turn out to trace to one source, inflating false corroboration.","trigger":"circular reporting not detected by the quality-of-information check","affected_nodes":["QOIC","DIAGNOSTICITY","ELICITATION"],"mitigation":"audit source independence and collapse circular streams to a single origin","severity":"high"},
 {"description":"A catastrophic tail risk is dismissed purely on its low base rate and never analyzed.","trigger":"base-rate neglect in reverse: low probability used to skip analysis","affected_nodes":["HIGH_IMPACT_LOW_PROB","OUTSIDE_VIEW","INDICATORS"],"mitigation":"analyze high-consequence outcomes on their own terms with probability-raising indicators","severity":"high"},
 {"description":"The probe battery overruns the decision deadline and the decision is made before the decisive probe completes.","trigger":"battery not ordered by value of information or not time-bounded","affected_nodes":["PROBE_BATTERY","TECHNIQUE_SELECT","PROBE_TO_GAP"],"mitigation":"order probes by value of information and bound the session; record un-run techniques as residual risk","severity":"high"},
 {"description":"Devil's advocacy becomes ritual: objections are raised and overruled without affecting any conclusion, breeding cynicism.","trigger":"adversarial challenge never adjudicated on the record","affected_nodes":["DEVILS_ADVOCATE","DECONFLICT","RED_HAT"],"mitigation":"require explicit on-record adjudication of the opposing case and rotate the role","severity":"medium"},
 {"description":"Mirror-imaging: the actor's likely choice is predicted from the analyst's own logic, missing a frame-specific decision.","trigger":"red-hat analysis projects home-frame rationality onto the actor","affected_nodes":["RED_HAT","DEVILS_ADVOCATE","HYPOTHESIS_GEN"],"mitigation":"derive the actor's choice from their own incentives and culture, not the analyst's","severity":"medium"},
 {"description":"Stated probabilities are systematically overconfident because analysts are never scored against outcomes.","trigger":"no calibration feedback loop on resolved forecasts","affected_nodes":["CALIBRATION_TRAIN","ELICITATION","OUTSIDE_VIEW"],"mitigation":"track Brier scores on resolvable forecasts and widen overconfident intervals","severity":"medium"},
 {"description":"Indicators are defined but not actually collectable, giving false reassurance of early warning.","trigger":"indicator specified without an observability/collection check","affected_nodes":["INDICATORS","SCENARIO","HIGH_IMPACT_LOW_PROB"],"mitigation":"require each indicator to pass an observability and collection-feasibility check","severity":"medium"},
 {"description":"Experts herd on the most senior voice because judgments were shared before being recorded independently.","trigger":"elicitation skips the independent-then-shared sequence","affected_nodes":["ELICITATION","DECONFLICT","CALIBRATION_TRAIN"],"mitigation":"record private judgments before any discussion, then allow documented revision","severity":"medium"},
 {"description":"A pre-mortem lists many failure modes but none are prioritized, so the decisive one is buried.","trigger":"prospective hindsight produces an unranked failure list","affected_nodes":["PREMORTEM","WHATIF","PROBE_TO_GAP"],"mitigation":"rank elicited failure modes by plausibility and impact before reporting","severity":"low"},
 {"description":"A knowledge gap is marked closed because a probe ran, even though the probe only partially addressed it.","trigger":"probe-to-gap mapping records false closure","affected_nodes":["PROBE_TO_GAP","PROBE_BATTERY","ASSUMPTION_TEST"],"mitigation":"record gap status as closed/narrowed/residual with the elicited finding that justifies it","severity":"medium"},
]

WF=[
 {"action":"select_techniques","node_ref":"TECHNIQUE_SELECT","description":"Classify the question and gap types and select the structured techniques whose probes address them.","artifact":"technique_selection_record","gate":"each selected technique is justified by a stated question and gap type"},
 {"action":"generate_hypotheses","node_ref":"HYPOTHESIS_GEN","description":"Generate a MECE competing-hypothesis set including the unlikely/deception hypothesis.","artifact":"hypothesis_set","gate":"set is mutually exclusive and includes a disconfirmable rival"},
 {"action":"run_ach","node_ref":"ACH","description":"Build the hypothesis-evidence matrix and rank by refutation, not confirmation.","artifact":"ach_matrix","gate":"every hypothesis scored against every entered evidence item"},
 {"action":"score_diagnosticity","node_ref":"DIAGNOSTICITY","description":"Rate each evidence item's discriminating power and flag non-diagnostic items.","artifact":"diagnosticity_ranking","gate":"the most diagnostic items are identified and drive the ranking"},
 {"action":"check_key_assumptions","node_ref":"KAC","description":"Surface and classify load-bearing assumptions by impact if false.","artifact":"key_assumptions_register","gate":"each load-bearing assumption is surfaced and impact-classified"},
 {"action":"test_assumptions","node_ref":"ASSUMPTION_TEST","description":"Define each assumption's breaking observable and attempt falsification; downgrade on failure.","artifact":"assumption_test_log","gate":"each load-bearing assumption has a breaking observable and was tested"},
 {"action":"run_imaginative_probes","node_ref":"WHATIF","description":"Run what-if and pre-mortem probes to back-cast surprises and elicit overlooked failure modes.","artifact":"blind_spot_findings","gate":"at least one trackable precursor or prioritized failure mode is surfaced"},
 {"action":"build_scenarios_and_indicators","node_ref":"INDICATORS","description":"Construct divergent scenarios and pre-specify observable, falsifiable indicators mapped to them.","artifact":"indicator_set","gate":"each indicator is observable, falsifiable, and mapped to a hypothesis or scenario"},
 {"action":"check_information_quality","node_ref":"QOIC","description":"Audit provenance, currency, and independence; flag circular reporting and deception.","artifact":"source_quality_report","gate":"every evidence item carries a provenance and independence rating"},
 {"action":"challenge_and_calibrate","node_ref":"DEVILS_ADVOCATE","description":"Run devil's advocacy and red-hat challenge; anchor on the outside view and calibrate probabilities.","artifact":"challenge_and_calibration_record","gate":"the opposing case is adjudicated on the record and probabilities are calibrated"},
 {"action":"map_probes_to_gaps","node_ref":"PROBE_TO_GAP","description":"Record, per gap, which probe ran, what it elicited, and the gap status.","artifact":"gap_by_probe_matrix","gate":"every identified gap maps to a probe, a finding, and a status"},
 {"action":"compose_battery","node_ref":"PROBE_BATTERY","description":"Order the probes by value of information and bound the battery to the decision deadline.","artifact":"probe_battery_plan","gate":"battery ordered by value of information and bounded to the deadline"},
]

DR=[
 {"rule":"TECHNIQUE_SELECT must justify each probe by question and gap type before any probe runs","rationale":"ritualistic SAT use wastes analyst time and misroutes gaps","trigger":"a probe is queued without a stated question/gap justification","action":"block the probe until its question and gap type are recorded"},
 {"rule":"HYPOTHESIS_GEN must produce a MECE set with at least one disconfirmable rival before ACH runs","rationale":"ACH on an anchored or incomplete set re-imports the bias it counters","trigger":"the matrix is built on a single working hypothesis","action":"return to hypothesis generation and deconfliction"},
 {"rule":"ACH must rank by disconfirming evidence, never by count of supporting evidence","rationale":"refutation-first ranking is what counters confirmation bias","trigger":"a hypothesis ranked by supporting-evidence count","action":"re-rank by weight of inconsistent evidence"},
 {"rule":"DIAGNOSTICITY weighting must be discounted by QOIC source-quality before driving the ranking","rationale":"a deceptive or circular source rated diagnostic corrupts the conclusion","trigger":"a high-diagnosticity item with an unverified or circular source","action":"discount its weight pending source validation"},
 {"rule":"Each load-bearing assumption from KAC must have a breaking observable in ASSUMPTION_TEST before the conclusion is finalized","rationale":"an untested decisive assumption is a hidden single point of failure","trigger":"a load-bearing assumption with no breaking observable","action":"define the observable or caveat the conclusion"},
 {"rule":"DEVILS_ADVOCATE must adjudicate the opposing case on the record before consensus is declared","rationale":"unadjudicated dissent leaves groupthink intact and breeds cynicism","trigger":"consensus declared with no recorded adjudication","action":"require explicit on-record adjudication and rotate the role"},
 {"rule":"OUTSIDE_VIEW base rate must be set before inside-view case adjustment in any probability estimate","rationale":"starting from case particulars produces base-rate neglect","trigger":"an estimate built only from case-specific detail","action":"anchor on a defensible reference class first"},
 {"rule":"PROBE_TO_GAP must record a status and elicited finding for every gap before PROBE_BATTERY closes","rationale":"a gap marked closed without a finding is false closure","trigger":"a gap closed with no elicited finding","action":"record closed/narrowed/residual with the justifying finding"},
 {"rule":"PROBE_BATTERY must be ordered by value of information and bounded to the decision deadline","rationale":"an unbounded battery misses the deadline and dilutes focus","trigger":"a battery that cannot complete before the deadline","action":"run highest-VOI probes first and record un-run techniques as residual risk"},
]

ARR=[
 {"rule":"Do not run probes before classifying the question and gap; mis-routed probes are wasted effort","prevents":"re-running a probe battery after discovering the gap type was misdiagnosed"},
 {"rule":"Do not build the ACH matrix before the hypothesis set is MECE and deconflicted","prevents":"rebuilding the matrix after a missing hypothesis is added late"},
 {"rule":"Do not let diagnosticity scores drive the ranking before source quality is checked","prevents":"re-scoring the matrix after a high-diagnosticity item is found circular or deceptive"},
 {"rule":"Do not finalize a conclusion before key assumptions have breaking observables and were tested","prevents":"reopening the conclusion when an untested decisive assumption later breaks"},
 {"rule":"Do not declare consensus before the opposing case is adjudicated on the record","prevents":"reworking the estimate after late dissent surfaces discounted evidence"},
 {"rule":"Do not estimate probabilities from case particulars before anchoring on a base rate","prevents":"re-deriving an estimate after base-rate neglect is caught in review"},
 {"rule":"Do not mark a gap closed without recording the elicited finding that justifies it","prevents":"re-probing a gap that was falsely marked closed"},
 {"rule":"Do not enlarge the battery past the deadline without a value-of-information stop rule","prevents":"re-planning the session after low-value probes consume the time budget"},
]

IP=[
 {"trigger":"every probe confirms the favored hypothesis","action":"audit ACH evidence entry and re-run DIAGNOSTICITY with BIAS_MITIGATION counters for confirmation bias","nodes":["ACH","DIAGNOSTICITY","BIAS_MITIGATION"],"priority":"critical"},
 {"trigger":"a load-bearing assumption is found untested and decisive","action":"return to KAC and define a breaking observable in ASSUMPTION_TEST before finalizing","nodes":["KAC","ASSUMPTION_TEST"],"priority":"high"},
 {"trigger":"circular reporting or a deceptive source is discovered","action":"re-run QOIC and re-score affected items in DIAGNOSTICITY","nodes":["QOIC","DIAGNOSTICITY","ELICITATION"],"priority":"high"},
 {"trigger":"the decision deadline shifts earlier","action":"re-order PROBE_BATTERY by value of information and record un-run techniques in PROBE_TO_GAP as residual risk","nodes":["PROBE_BATTERY","PROBE_TO_GAP","TECHNIQUE_SELECT"],"priority":"high"},
 {"trigger":"a calibration batch shows systematic overconfidence","action":"re-run CALIBRATION_TRAIN feedback and re-anchor estimates on OUTSIDE_VIEW base rates","nodes":["CALIBRATION_TRAIN","OUTSIDE_VIEW"],"priority":"medium"},
 {"trigger":"consensus formed with no recorded dissent","action":"re-run DEVILS_ADVOCATE and RED_HAT and adjudicate the opposing case on the record","nodes":["DEVILS_ADVOCATE","RED_HAT","DECONFLICT"],"priority":"medium"},
 {"trigger":"an indicator proves uncollectable in practice","action":"revise INDICATORS for observability and re-link to the scenario it warns for","nodes":["INDICATORS","SCENARIO","HIGH_IMPACT_LOW_PROB"],"priority":"medium"},
]

spec = {
 "domain":"sat__structured_probing",
 "domain_label":"Structured Analytic Techniques & Knowledge Probes (Knowledge Acquisition subdomain)",
 "purpose":"session_bounded_method_for_selecting_and_running_structured_analytic_techniques_as_knowledge_probes_that_convert_knowledge_gaps_into_elicited_bias_mitigated_findings_via_analysis_of_competing_hypotheses_key_assumptions_check_what_if_pre_mortem_indicators_and_quality_of_information_checks_each_probe_mapped_to_the_gap_it_addresses_and_composed_into_a_value_of_information_ordered_battery",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "these structured analytic techniques are standard intelligence-analysis methods used defensively to improve analytic rigor",
   "an analytic question and at least one knowledge gap statement are available to drive technique selection",
   "the analysis is session-bounded by a decision deadline that constrains how many probes can run",
 ],
 "exclusions":[
   "collection tasking and source acquisition (delegated to the collection/acquisition layer)",
   "final report writing and dissemination",
   "argument mapping and claim-evidence appraisal internals (reused from the argue specialist, not reimplemented)",
   "automated learning of which techniques work best from data",
 ],
 "source_description":"heuristic prior estimates for structured-analytic-technique probing work units, informed by the standard intelligence-analysis SAT literature; no supplied dataset",
 "source_citation":"Heuer 1999 Psychology of Intelligence Analysis (ACH, cognitive bias); Heuer & Pherson 2010/2014 Structured Analytic Techniques for Intelligence Analysis (key assumptions check, what-if, indicators, quality-of-information check, red hat, devil's advocacy); Klein 2007 Performing a Project Premortem (Harvard Business Review); Kahneman 2011 Thinking, Fast and Slow (inside/outside view, anchoring, base-rate neglect); Tetlock & Gardner 2015 Superforecasting (calibration, reference-class forecasting); US Government 2009 A Tradecraft Primer: Structured Analytic Techniques for Improving Intelligence Analysis",
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
 "priority_rationale":"TECHNIQUE_SELECT and HYPOTHESIS_GEN are foundational; ACH/DIAGNOSTICITY, KAC/ASSUMPTION_TEST, and the imaginative and calibration probes are the core techniques; challenge and bias-mitigation harden them; PROBE_TO_GAP and PROBE_BATTERY synthesize coverage and ordering last.",
 "eval_objective":"verify_technique_selection_competing_hypothesis_analysis_assumption_testing_bias_mitigation_calibration_and_probe_to_gap_mapping_of_sat__structured_probing_kb",
}

out_dir = "branches/b11_knowledge_acquisition/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "sat__structured_probing.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
