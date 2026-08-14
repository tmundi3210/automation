#!/usr/bin/env python3
"""Generate the qeval__answer_quality content spec (B10) for kb_forge.py.
Compact authoring: node() applies sane defaults so only domain content + base
metric magnitudes are specified per node. Domain: scoring questions and answers
on a multi-dimensional rubric within a single evaluation session."""
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
        "academic_fields": ["epistemology", "computational_linguistics", "evaluation_methodology"],
        "subfields": subfields or ["question_science", "answerhood_theory", "rubric_scoring"],
        "specialists": specialists or ["evaluation_methodologist"],
        "contradictors": contradictors or ["single_score_advocate"],
        "inputs": inputs or ["question text", "answer text", "question under discussion"],
        "outputs": outputs or ["per-dimension score", "evidence label"],
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

# ---- foundations: well-formedness and presupposition soundness ----
N.append(node("WELLFORMED","question_well_formedness",
  "Assess whether the question is well-formed: syntactically intelligible, semantically determinate, and carrying a satisfiable presupposition set, so that a true answer can exist at all (Belnap & Steel's answerhood preconditions).",
  "foundations", [], ["CQ_01"],
  b(0.9,0.78,0.82,0.55,0.82,0.7,0.55,0.7,0.42, 0.8,0.82, 0.9,0.7,0.16,0.5,[0.2,0.5],"question grammar or answerhood preconditions revised"),
  ["syntactic intelligibility","semantic determinacy","satisfiable presupposition set"],
  ["scoring the answer","relevance to the QUD"],
  [pro("A well-formedness gate filters questions that admit no true answer before any answer scoring is attempted","'have you stopped overclaiming?' is flagged as carrying a false presupposition")],
  [con("Strict well-formedness can reject loosely phrased but genuinely answerable questions","a colloquial 'why even bother?' rejected though its intent is recoverable")],
  ["ambiguous question scored as if determinate","loaded question's false presupposition passes unflagged"],
  ["question has a determinate reading and a satisfiable presupposition set"],
  ["answerhood precondition set revised","new ill-formed question class observed"],
  specialists=["question_theorist","linguist"]))

N.append(node("PRESUP_SOUNDNESS","presupposition_soundness",
  "Evaluate the soundness of the presuppositions the question and answer rely on: are the taken-for-granted propositions true and accepted in the discourse, distinguishing genuine information gaps from loaded or complex questions.",
  "foundations", ["WELLFORMED"], ["CQ_01","CQ_02"],
  b(0.84,0.72,0.72,0.6,0.82,0.72,0.55,0.66,0.5, 0.76,0.8, 0.84,0.66,0.18,0.5,[0.22,0.54],"presupposition accommodation policy changes"),
  ["truth of taken-for-granted propositions","loaded/complex question detection","presupposition accommodation"],
  ["overall answer aggregation","comparative ranking"],
  [pro("Separating a false presupposition from a real gap stops the evaluator from rewarding an answer that accepts a false premise","an answer that 'corrects the question' scores higher than one that answers the loaded form")],
  [con("Presupposition analysis is context-sensitive; accommodation norms differ by discourse and can over-flag","a domain term presupposed as shared is treated as a loaded premise")],
  ["false presupposition accommodated silently","legitimate shared premise mis-flagged as loaded"],
  ["presupposition truth and acceptance are explicitly checked before crediting the answer"],
  ["accommodation policy changes","a loaded question slipped through to scoring"],
  specialists=["pragmatics_specialist","question_theorist"]))

N.append(node("QUESTION_QUALITY","overall_question_quality_score",
  "Produce an overall question-quality score combining well-formedness, presupposition soundness, specificity, and answerability, so questions can be compared and a poor question is not allowed to inflate or deflate the answer's evaluation.",
  "foundations", ["WELLFORMED","PRESUP_SOUNDNESS"], ["CQ_02","CQ_03"],
  b(0.86,0.78,0.78,0.6,0.8,0.78,0.58,0.66,0.52, 0.76,0.8, 0.86,0.66,0.2,0.52,[0.22,0.56],"question-quality rubric weights change"),
  ["question specificity","answerability","aggregate question score"],
  ["answer informativeness","evidence tiering"],
  [pro("Scoring the question itself prevents a vague prompt from being silently blamed on the answerer","a one-word ambiguous prompt is marked low-quality, not the under-specified answer to it")],
  [con("A single question-quality number hides which dimension failed and can mislead downstream comparison","two questions tie on the aggregate but fail for opposite reasons")],
  ["poor question score conflated with poor answer","question quality scored on a hidden, non-reproducible rubric"],
  ["question score decomposes into named sub-scores with reproducible weights"],
  ["question rubric weights revised","question/answer blame attribution disputed"],
  specialists=["evaluation_methodologist","question_theorist"]))

# ---- relevance, scope, granularity, directness ----
N.append(node("RELEVANCE","relevance_to_question_under_discussion",
  "Judge the answer's relevance to the question under discussion (QUD): does it address the proposition the question requests, in the sense of Grice's maxim of relation and IR topical relevance, rather than an adjacent or different question.",
  "relevance", ["QUESTION_QUALITY"], ["CQ_03","CQ_04"],
  b(0.9,0.82,0.82,0.62,0.84,0.82,0.58,0.64,0.55, 0.74,0.78, 0.9,0.64,0.2,0.55,[0.24,0.58],"QUD model or relevance criterion changes"),
  ["topical relevance to the QUD","Gricean relation","on-question vs off-question"],
  ["informativeness magnitude","calibration"],
  [pro("Anchoring relevance to an explicit QUD makes 'off-topic' a checkable property rather than a reviewer impression","an eloquent answer to a neighbouring question scores low on relevance to the asked QUD")],
  [con("QUD identification is itself interpretive; a mis-identified QUD makes every relevance judgment wrong","reading the QUD too narrowly penalizes a correctly broad answer")],
  ["answer scored relevant to a mis-identified QUD","topical overlap mistaken for genuine relevance"],
  ["the QUD is stated explicitly and the answer is judged against it, not a paraphrase"],
  ["QUD model revised","relevance disputed between reviewers"],
  specialists=["pragmatics_specialist","ir_relevance_specialist"]))

N.append(node("SCOPE_MATCH","scope_match_question_answer",
  "Check that the answer's scope matches the question's scope: neither narrower (leaving requested parts unanswered) nor broader (answering more than asked), so breadth is appropriate to the QUD.",
  "relevance", ["RELEVANCE"], ["CQ_04","CQ_05"],
  b(0.8,0.72,0.72,0.6,0.78,0.74,0.55,0.64,0.5, 0.76,0.78, 0.8,0.64,0.2,0.5,[0.24,0.56],"scope-matching policy changes"),
  ["breadth alignment","under-scope detection","over-scope detection"],
  ["granularity of detail","aggregation"],
  [pro("Explicit scope matching separates 'answered too little' from 'answered too much', which a single relevance score conflates","a question about one region answered for all regions is flagged as over-scope")],
  [con("Some questions legitimately invite scope expansion; rigid scope matching penalizes helpful context","a 'what is X?' answer that adds a crucial caveat is dinged as over-scope")],
  ["partial answer scored as complete","scope creep rewarded as thoroughness"],
  ["answer breadth is mapped to the question's requested breadth with under/over-scope flags"],
  ["scope policy changes","scope dispute between reviewers"]))

N.append(node("GRANULARITY_MATCH","granularity_match",
  "Check that the answer's granularity (level of detail/abstraction) matches what the question requests: a yes/no question should not require a treatise, and a how-question should not be answered with a one-word label.",
  "relevance", ["RELEVANCE"], ["CQ_05"],
  b(0.76,0.68,0.7,0.58,0.74,0.7,0.5,0.66,0.46, 0.78,0.78, 0.76,0.66,0.2,0.46,[0.22,0.52],"granularity-matching policy changes"),
  ["level-of-detail alignment","abstraction-level match","verbosity vs terseness fit"],
  ["scope breadth","evidence tiering"],
  [pro("Granularity matching distinguishes an appropriately concise answer from an evasively shallow one","a one-line answer to a yes/no question is correct, not penalized as thin")],
  [con("Granularity preferences vary by audience; a fixed target mis-serves experts and novices alike","an expert audience finds the 'right' granularity patronizing")],
  ["concise correct answer penalized as shallow","verbose padding rewarded as depth"],
  ["answer detail level is compared to the question's requested granularity"],
  ["granularity policy changes","audience model changes"]))

N.append(node("DIRECTNESS","directness_and_responsiveness",
  "Assess how directly the answer responds to the question: does it lead with the requested proposition (a direct answer in Belnap & Steel's sense) or bury/withhold it behind preamble, qualification, or redirection.",
  "relevance", ["RELEVANCE"], ["CQ_06"],
  b(0.82,0.74,0.78,0.58,0.8,0.76,0.55,0.62,0.58, 0.74,0.78, 0.82,0.62,0.22,0.52,[0.24,0.6],"directness rubric or hedging policy changes"),
  ["lead-with-answer detection","direct vs indirect answer","responsiveness"],
  ["honest hedging magnitude","deflection detection"],
  [pro("Measuring directness catches answers that technically contain the information but withhold it behind evasion","the answer is in the last sentence after three paragraphs of hedging, flagged as low-directness")],
  [con("Maximal directness can punish answers whose honest uncertainty genuinely requires qualification first","a correctly cautious 'it depends, because...' reads as indirect")],
  ["evasive answer scored as responsive","necessary qualification mistaken for evasion"],
  ["the requested proposition is locatable and its position/prominence in the answer is scored"],
  ["directness rubric changes","directness vs hedging dispute observed"],
  specialists=["pragmatics_specialist","evaluation_methodologist"]))

# ---- informativeness, adequacy, coverage ----
N.append(node("INFORMATIVENESS","answer_informativeness_specificity",
  "Score the answer's informativeness and specificity: how much it reduces the questioner's uncertainty, per Grice's maxim of quantity (as informative as required, not more), favouring specific commitments over vacuous generality.",
  "content", ["RELEVANCE"], ["CQ_07","CQ_08"],
  b(0.88,0.82,0.84,0.64,0.82,0.8,0.58,0.62,0.58, 0.72,0.76, 0.88,0.62,0.22,0.55,[0.26,0.62],"informativeness measure or quantity-maxim policy changes"),
  ["uncertainty reduction","specificity of commitments","Gricean quantity"],
  ["calibration of confidence","comparative ranking"],
  [pro("Rewarding specific, uncertainty-reducing content separates a substantive answer from a fluent non-answer","'about 40%' is more informative than 'a significant portion', other things equal")],
  [con("Maximizing informativeness pushes toward over-commitment beyond what the evidence warrants","a precise number is asserted where only a range is supported")],
  ["vacuous generality scored as informative","spurious precision rewarded over warranted hedging"],
  ["informativeness is scored as uncertainty reduction relative to what the question asked"],
  ["quantity-maxim policy changes","informativeness vs calibration tension observed"],
  specialists=["information_theorist","evaluation_methodologist"]))

N.append(node("ANSWER_ADEQUACY","answer_adequacy_completeness",
  "Evaluate adequacy: whether the answer fully resolves the question it addresses, completing the proposition the question requested rather than partially or tangentially touching it (answerhood completeness).",
  "content", ["INFORMATIVENESS","SCOPE_MATCH"], ["CQ_08","CQ_09"],
  b(0.86,0.8,0.82,0.64,0.82,0.8,0.58,0.62,0.55, 0.74,0.78, 0.86,0.62,0.22,0.55,[0.26,0.62],"adequacy criterion changes"),
  ["completeness vs the question","partial-answer detection","resolution of the requested proposition"],
  ["sub-question coverage","aggregation policy"],
  [pro("Adequacy makes 'fully answered' a checkable property distinct from 'said something relevant'","a partial answer that resolves one of two requested facts is marked adequate-partial, not adequate")],
  [con("Adequacy can demand more than a hard question admits, penalizing honest partial answers to open problems","an answer to a genuinely unresolved question is dinged for incompleteness")],
  ["partial answer scored as fully adequate","open-problem answer over-penalized for incompleteness"],
  ["the answer is judged complete or partial against the explicit requested proposition"],
  ["adequacy criterion changes","adequacy dispute on an open question"],
  specialists=["question_theorist","evaluation_methodologist"]))

N.append(node("ANSWER_COVERAGE","sub_question_coverage",
  "Decompose a complex/multi-part question into its sub-questions and verify coverage: each sub-question is mapped to at least one part of the answer, so a multi-part question is not scored on a single salient part.",
  "content", ["ANSWER_ADEQUACY"], ["CQ_09","CQ_10"],
  b(0.82,0.76,0.78,0.66,0.8,0.8,0.58,0.6,0.55, 0.74,0.78, 0.82,0.6,0.22,0.54,[0.26,0.62],"sub-question decomposition policy changes"),
  ["sub-question decomposition","sub-question-to-answer mapping","coverage closure"],
  ["per-dimension scoring","ranking"],
  [pro("Sub-question coverage catches the common failure where a multi-part question is answered only in its easy part","a 'what and why' question answered only on 'what' is flagged as 50% coverage")],
  [con("Sub-question decomposition is interpretive; over-splitting invents obligations the asker did not intend","a single intent is split into three pedantic sub-questions")],
  ["multi-part question scored on one part","over-decomposition fabricates uncovered sub-questions"],
  ["each identified sub-question maps to a covering answer span or an explicit gap"],
  ["decomposition policy changes","a multi-part question mis-scored on one part"],
  specialists=["question_theorist","evaluation_methodologist"]))

# ---- warrant, evidence, falsifiability, consistency ----
N.append(node("WARRANT_EVAL","evidential_warrant_evaluation",
  "Evaluate the evidential warrant linking the answer's claims to its grounds, in Toulmin's sense: is there a stated or recoverable warrant that licenses moving from the cited grounds to the asserted claim, with backing where needed.",
  "evidence", ["RELEVANCE"], ["CQ_11","CQ_12"],
  b(0.9,0.82,0.8,0.7,0.86,0.84,0.62,0.6,0.6, 0.7,0.74, 0.9,0.6,0.24,0.58,[0.28,0.66],"warrant model or backing requirement changes"),
  ["grounds-to-claim warrant","backing presence","rebuttal acknowledgement"],
  ["confidence calibration magnitude","aggregation"],
  [pro("A Toulmin warrant check exposes claims that assert a conclusion without licensing the inference from their grounds","an answer cites a correlation as grounds for a causal claim with no warrant, flagged")],
  [con("Demanding explicit warrant penalizes correct answers in domains where the inference is conventionally tacit","a textbook fact is dinged for not re-deriving its warrant")],
  ["unwarranted leap from grounds to claim accepted","tacit but valid warrant treated as missing"],
  ["each load-bearing claim has a recoverable warrant linking it to stated grounds"],
  ["warrant model changes","an unwarranted claim was credited"],
  specialists=["argumentation_specialist","epistemologist"]))

N.append(node("EVIDENCE_TIER","evidence_tier_of_answer",
  "Assign an evidence tier to the answer's support: from unsourced assertion, through cited secondary source, to primary/observed/experimental evidence, so the strength of grounds is explicit and comparable across answers.",
  "evidence", ["WARRANT_EVAL"], ["CQ_12","CQ_13"],
  b(0.84,0.76,0.74,0.64,0.82,0.78,0.6,0.62,0.5, 0.74,0.78, 0.84,0.62,0.2,0.55,[0.24,0.58],"evidence-tier taxonomy changes"),
  ["evidence-strength tiering","source-type classification","unsourced-assertion flagging"],
  ["warrant adequacy","calibration"],
  [pro("An explicit evidence tier makes 'better supported' comparable across answers instead of an impression","an answer citing a primary measurement outranks one asserting the same claim unsourced")],
  [con("Tier labels can be gamed by citing weak sources that look authoritative","a citation to an irrelevant but prestigious source inflates the tier")],
  ["unsourced assertion tiered as evidenced","prestige source mistaken for relevant evidence"],
  ["the answer's support is mapped to an explicit, ordered evidence tier"],
  ["evidence taxonomy changes","a mis-tiered citation observed"],
  specialists=["evidence_governance_specialist","epistemologist"]))

N.append(node("FALSIFIABLE_CHECK","falsifiability_checkability",
  "Determine whether the answer's claims are falsifiable/checkable: is there a stated or implied way to confirm or refute them, distinguishing testable commitments from unfalsifiable, self-sealing, or purely rhetorical statements.",
  "evidence", ["WARRANT_EVAL"], ["CQ_13"],
  b(0.8,0.72,0.72,0.62,0.8,0.74,0.55,0.62,0.48, 0.76,0.78, 0.8,0.62,0.2,0.5,[0.24,0.56],"falsifiability criterion changes"),
  ["testability of claims","confirm/refute path","self-sealing-statement detection"],
  ["evidence tier","aggregation"],
  [pro("A falsifiability check separates a checkable commitment from an unfalsifiable hedge that cannot be wrong","'it may or may not rain' is flagged as unfalsifiable, not credited as cautious")],
  [con("Some valid normative or definitional answers are legitimately not empirically falsifiable","a definitional answer is wrongly penalized for non-testability")],
  ["unfalsifiable hedge scored as a safe answer","valid definitional claim penalized as untestable"],
  ["each empirical claim has a stated or implied confirm/refute path"],
  ["falsifiability criterion changes","an unfalsifiable claim was credited as cautious"],
  specialists=["philosopher_of_science","evaluation_methodologist"]))

N.append(node("CONSISTENCY","internal_consistency",
  "Check the answer's internal consistency: it must not assert mutually contradictory propositions, and its stated confidence, scope, and conclusions must cohere across the whole response.",
  "evidence", ["INFORMATIVENESS"], ["CQ_14"],
  b(0.82,0.74,0.74,0.62,0.82,0.76,0.58,0.64,0.52, 0.76,0.78, 0.82,0.64,0.2,0.5,[0.22,0.54],"consistency-checking method changes"),
  ["contradiction detection","scope/conclusion coherence","confidence coherence"],
  ["calibration vs evidence","ranking"],
  [pro("Internal-consistency checking catches answers that hedge then assert, or contradict themselves across sections","an answer says 'unknown' then later asserts the unknown value, flagged")],
  [con("Apparent contradictions can be context-shifted distinctions that surface checking would over-flag","two claims true under different stated conditions read as a contradiction")],
  ["self-contradiction passes unflagged","legitimate context-shift mis-flagged as contradiction"],
  ["the answer is free of unresolved internal contradictions across claims, scope, and confidence"],
  ["consistency method changes","a self-contradictory answer was credited"]))

# ---- calibration, hedging, overclaim, deflection ----
N.append(node("CALIBRATION","confidence_calibration_vs_evidence",
  "Evaluate calibration: whether the answer's expressed confidence matches the strength of its evidence and warrant, rewarding stated confidence that tracks reality (Tetlock-style calibration) and penalizing both over- and under-confidence.",
  "honesty", ["WARRANT_EVAL","EVIDENCE_TIER","CONSISTENCY"], ["CQ_14","CQ_15"],
  b(0.9,0.82,0.82,0.72,0.88,0.85,0.65,0.58,0.62, 0.68,0.74, 0.9,0.58,0.26,0.6,[0.3,0.68],"calibration scoring rule changes"),
  ["confidence-vs-evidence match","over/under-confidence detection","proper-scoring intuition"],
  ["informativeness magnitude","comparative ranking"],
  [pro("Calibration scoring makes confidence accountable to evidence, the way proper scoring rules reward honest probabilities","a 'probably' backed by strong evidence outscores a 'certainly' backed by weak grounds")],
  [con("Calibration pressure can push answers toward bland mid-confidence that is hard to mark wrong","every answer hedges to 'maybe' to dodge a calibration penalty")],
  ["confident wrong answer rewarded for fluency","honest hedging penalized as low confidence"],
  ["expressed confidence is scored against evidence strength, penalizing over- and under-confidence"],
  ["calibration rule changes","systematic over/under-confidence observed in scored answers"],
  specialists=["forecasting_calibration_specialist","epistemologist"]))

N.append(node("HEDGING_EVAL","appropriate_hedging_evaluation",
  "Evaluate hedging quality: whether qualifiers, ranges, and uncertainty markers are used where and only where the evidence warrants them, distinguishing honest, informative hedging from both evasive over-hedging and reckless under-hedging.",
  "honesty", ["CALIBRATION","DIRECTNESS"], ["CQ_15","CQ_16"],
  b(0.82,0.74,0.78,0.66,0.82,0.8,0.58,0.6,0.6, 0.72,0.76, 0.82,0.6,0.24,0.56,[0.28,0.64],"hedging policy changes"),
  ["warranted-qualifier detection","over-hedging detection","under-hedging detection"],
  ["overclaim detection","aggregation"],
  [pro("Scoring hedging quality rewards a qualifier that carries real information and penalizes one that only dodges commitment","'between 30 and 50%' informs; 'some unknown amount' merely hedges")],
  [con("Hedging norms are audience- and stakes-dependent; a fixed standard mis-serves high- and low-stakes contexts","cautious clinical phrasing reads as evasive in a casual context")],
  ["evasive over-hedging scored as appropriate caution","reckless under-hedging scored as confidence"],
  ["each qualifier is judged warranted, over-hedged, or under-hedged against the evidence"],
  ["hedging policy changes","hedging vs directness dispute observed"],
  specialists=["pragmatics_specialist","forecasting_calibration_specialist"]))

N.append(node("OVERCLAIM_DETECT","overclaim_undersupport_detection",
  "Detect overclaim and under-support: assertions whose strength exceeds their warrant and evidence tier, including spurious precision, unsupported universals, and causal claims from correlational grounds.",
  "honesty", ["CALIBRATION","WARRANT_EVAL"], ["CQ_16","CQ_17"],
  b(0.88,0.8,0.8,0.7,0.88,0.84,0.62,0.58,0.64, 0.68,0.74, 0.88,0.58,0.26,0.6,[0.3,0.68],"overclaim taxonomy changes"),
  ["claim-strength vs warrant gap","spurious-precision detection","unsupported-universal detection"],
  ["deflection detection","ranking"],
  [pro("Overclaim detection penalizes the confident-but-unsupported answer that informativeness scoring alone would reward","'always' is flagged where the grounds support only 'often')")],
  [con("Aggressive overclaim flags can suppress legitimately strong claims that do have adequate warrant","a well-supported strong claim is dinged as overclaim by an over-sensitive flag")],
  ["overclaim rewarded as confident expertise","well-warranted strong claim suppressed as overclaim"],
  ["each claim's asserted strength is checked against its warrant and evidence tier"],
  ["overclaim taxonomy changes","an overclaim was credited as strong evidence"],
  specialists=["argumentation_specialist","epistemologist"]))

N.append(node("DEFLECTION_DETECT","deflection_nonanswer_detection",
  "Detect deflection and non-answers: responses that change the question, answer a different (easier) question, restate the question, or fill space without committing to the requested proposition.",
  "honesty", ["DIRECTNESS","RELEVANCE"], ["CQ_06","CQ_17"],
  b(0.86,0.8,0.82,0.66,0.86,0.82,0.6,0.6,0.62, 0.7,0.74, 0.86,0.6,0.24,0.58,[0.28,0.66],"deflection taxonomy changes"),
  ["question-switching detection","non-commitment detection","restatement-as-answer detection"],
  ["aggregation","comparative ranking"],
  [pro("Deflection detection catches the fluent non-answer that scores well on tone but never commits to the requested proposition","a polished response that answers an adjacent question is flagged as a non-answer")],
  [con("A genuine 'I cannot answer, and here is why' is a legitimate response that crude deflection flags over-penalize","an honest, well-reasoned refusal is mis-flagged as evasion")],
  ["fluent non-answer scored as a real answer","honest justified refusal mis-flagged as deflection"],
  ["the response is checked for whether it commits to the requested proposition or deflects"],
  ["deflection taxonomy changes","a non-answer was scored as an answer"],
  specialists=["pragmatics_specialist","evaluation_methodologist"]))

# ---- aggregation, rubric, ranking ----
N.append(node("MULTIDIM_RUBRIC","multidimensional_scoring_rubric",
  "Define the multi-dimensional rubric: the named, anchored dimensions (relevance, informativeness, adequacy, warrant, calibration, honesty) with level descriptors, so each answer receives a transparent per-dimension profile, not a single opaque score.",
  "scoring", ["RELEVANCE","INFORMATIVENESS","ANSWER_ADEQUACY","WARRANT_EVAL","CALIBRATION","DEFLECTION_DETECT"], ["CQ_18","CQ_19"],
  b(0.9,0.84,0.84,0.66,0.84,0.86,0.66,0.62,0.55, 0.74,0.78, 0.9,0.62,0.2,0.56,[0.22,0.56],"rubric dimensions or anchors change"),
  ["named scoring dimensions","level-anchored descriptors","per-answer score profile"],
  ["aggregation weights","comparative ranking mechanics"],
  [pro("An anchored multi-dimensional rubric makes scores reproducible and reveals which dimension drove a verdict","two answers tie on total but the rubric shows one failed warrant and the other failed relevance")],
  [con("More dimensions raise annotation cost and inter-rater variance, and tempt double-counting correlated dimensions","relevance and adequacy partly overlap, inflating a low score on both")],
  ["dimensions overlap and double-count","unanchored levels yield non-reproducible scores"],
  ["each dimension has level-anchored descriptors and produces an independent sub-score"],
  ["rubric dimensions or anchors revised","inter-rater disagreement on a dimension rises"],
  specialists=["evaluation_methodologist","psychometrician"]))

N.append(node("RUBRIC_AGGREGATE","aggregation_policy_across_dimensions",
  "Specify the aggregation policy that combines per-dimension sub-scores into a comparable overall score, including weights, any gating/veto dimensions, and how missing or not-applicable dimensions are handled, while preserving the underlying profile.",
  "scoring", ["MULTIDIM_RUBRIC","QUESTION_QUALITY"], ["CQ_19","CQ_20"],
  b(0.88,0.82,0.82,0.7,0.86,0.84,0.66,0.6,0.6, 0.72,0.76, 0.88,0.6,0.22,0.58,[0.26,0.62],"aggregation weights or gating policy changes"),
  ["dimension weighting","gating/veto dimensions","missing-dimension handling"],
  ["per-dimension scoring rules","question-quality scoring internals"],
  [pro("A stated aggregation policy makes the overall score reproducible and lets a critical dimension veto rather than be averaged away","a deflection veto caps the total regardless of fluent informativeness")],
  [con("Any aggregation discards information; a single number can hide a profile that two answers do not actually share","two answers reach 0.7 by averaging opposite strengths and weaknesses")],
  ["critical failure averaged away into a passing total","aggregate reported without the profile it came from"],
  ["aggregation weights and any veto rules are explicit; the overall score ships with its profile"],
  ["aggregation policy changes","an averaged total masked a critical failure"],
  specialists=["evaluation_methodologist","decision_analyst"]))

N.append(node("COMPARATIVE_RANK","comparative_answer_ranking",
  "Rank candidate answers to the same question against each other using the rubric profiles and aggregation, producing a consistent, transitive ordering that is robust to irrelevant alternatives where possible.",
  "scoring", ["RUBRIC_AGGREGATE"], ["CQ_20"],
  b(0.82,0.78,0.8,0.7,0.82,0.8,0.62,0.6,0.55, 0.72,0.76, 0.82,0.6,0.24,0.55,[0.28,0.62],"ranking method or transitivity policy changes"),
  ["pairwise/listwise ranking","transitivity check","tie-breaking by dimension"],
  ["single-answer absolute scoring","rubric definition"],
  [pro("Comparative ranking is often more reliable than absolute scores because raters compare more consistently than they calibrate absolutely","reviewers agree A beats B even when they disagree on A's absolute score")],
  [con("Pairwise preferences can be intransitive and order-sensitive, producing cycles or unstable rankings","A>B, B>C, C>A cycle from inconsistent pairwise judgments")],
  ["intransitive ranking cycle produced","ranking sensitive to the set of candidates compared"],
  ["the ranking is checked for transitivity and ties are broken by named dimensions"],
  ["ranking method changes","a ranking cycle or instability observed"],
  specialists=["evaluation_methodologist","decision_analyst"]))

CQ = [
 ("CQ_01","Is the question well-formed and are its presuppositions sound, so that a true answer can exist?",["nodes","glossary"],"WELLFORMED and PRESUP_SOUNDNESS establish answerhood preconditions and presupposition truth",["WELLFORMED","PRESUP_SOUNDNESS"]),
 ("CQ_02","How is overall question quality scored and kept separate from answer quality?",["nodes"],"QUESTION_QUALITY aggregates well-formedness, presupposition soundness, and answerability",["QUESTION_QUALITY","PRESUP_SOUNDNESS"]),
 ("CQ_03","How is the answer's relevance to the question under discussion judged?",["nodes","conflict_axes"],"RELEVANCE anchors topical relevance to an explicit QUD; QUESTION_QUALITY gates it",["RELEVANCE","QUESTION_QUALITY"]),
 ("CQ_04","How is scope match between question and answer checked?",["nodes"],"SCOPE_MATCH detects under- and over-scope against the requested breadth",["SCOPE_MATCH","RELEVANCE"]),
 ("CQ_05","How is granularity match between question and answer checked?",["nodes"],"GRANULARITY_MATCH compares the answer's detail level to the requested granularity",["GRANULARITY_MATCH","SCOPE_MATCH"]),
 ("CQ_06","How are directness and deflection/non-answers distinguished?",["nodes","workflow"],"DIRECTNESS scores responsiveness; DEFLECTION_DETECT flags non-answers",["DIRECTNESS","DEFLECTION_DETECT"]),
 ("CQ_07","How is the answer's informativeness and specificity scored?",["nodes"],"INFORMATIVENESS scores uncertainty reduction per the quantity maxim",["INFORMATIVENESS"]),
 ("CQ_08","How is answer adequacy/completeness against the question evaluated?",["nodes"],"ANSWER_ADEQUACY judges completion of the requested proposition; INFORMATIVENESS feeds it",["ANSWER_ADEQUACY","INFORMATIVENESS"]),
 ("CQ_09","How is coverage of a multi-part question's sub-questions verified?",["nodes"],"ANSWER_COVERAGE maps each sub-question to a covering span or gap",["ANSWER_COVERAGE","ANSWER_ADEQUACY"]),
 ("CQ_10","How is a complex question decomposed for coverage scoring?",["nodes","workflow"],"ANSWER_COVERAGE decomposes the question and checks coverage closure",["ANSWER_COVERAGE"]),
 ("CQ_11","How is the evidential warrant linking grounds to claims evaluated?",["nodes"],"WARRANT_EVAL checks the Toulmin warrant and backing for load-bearing claims",["WARRANT_EVAL"]),
 ("CQ_12","How is the answer's evidence tier assigned and used?",["nodes"],"EVIDENCE_TIER assigns an ordered evidence-strength tier; WARRANT_EVAL precedes it",["EVIDENCE_TIER","WARRANT_EVAL"]),
 ("CQ_13","How is an answer's falsifiability/checkability determined?",["nodes"],"FALSIFIABLE_CHECK identifies confirm/refute paths and self-sealing statements",["FALSIFIABLE_CHECK","EVIDENCE_TIER"]),
 ("CQ_14","How are internal consistency and confidence calibration evaluated?",["nodes","conflict_axes"],"CONSISTENCY detects contradictions; CALIBRATION scores confidence vs evidence",["CONSISTENCY","CALIBRATION"]),
 ("CQ_15","How is appropriate hedging distinguished from over- and under-hedging?",["nodes"],"HEDGING_EVAL judges qualifiers against evidence; CALIBRATION grounds it",["HEDGING_EVAL","CALIBRATION"]),
 ("CQ_16","How are overclaim and under-support detected?",["nodes"],"OVERCLAIM_DETECT compares asserted strength to warrant and evidence tier",["OVERCLAIM_DETECT","HEDGING_EVAL"]),
 ("CQ_17","How are deflection and fluent non-answers separated from honest refusals?",["nodes","edge_cases"],"DEFLECTION_DETECT and OVERCLAIM_DETECT separate evasion and overreach from honest answers",["DEFLECTION_DETECT","OVERCLAIM_DETECT"]),
 ("CQ_18","What multi-dimensional rubric produces a transparent per-answer profile?",["nodes"],"MULTIDIM_RUBRIC defines anchored dimensions yielding independent sub-scores",["MULTIDIM_RUBRIC"]),
 ("CQ_19","How are per-dimension scores aggregated into a comparable overall score?",["nodes","conflict_axes"],"RUBRIC_AGGREGATE specifies weights, gating, and missing-dimension handling over MULTIDIM_RUBRIC",["RUBRIC_AGGREGATE","MULTIDIM_RUBRIC"]),
 ("CQ_20","How are candidate answers ranked comparably and consistently?",["nodes","workflow"],"COMPARATIVE_RANK orders answers from rubric profiles with a transitivity check",["COMPARATIVE_RANK","RUBRIC_AGGREGATE"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

# consolidate node->CQ references (dense target 10-14; we use 20 nodes mapping onto 20 CQs but cap CQ count)
# NOTE: brief caps competency_questions at 14. Reduce to 14 CQs and remap.
CQ = [
 ("CQ_01","Is the question well-formed and are its presuppositions sound, so a true answer can exist?",["nodes","glossary"],"WELLFORMED and PRESUP_SOUNDNESS establish answerhood preconditions and presupposition truth",["WELLFORMED","PRESUP_SOUNDNESS"]),
 ("CQ_02","How is overall question quality scored and kept separate from answer quality?",["nodes"],"QUESTION_QUALITY aggregates well-formedness, presupposition soundness, and answerability",["QUESTION_QUALITY","PRESUP_SOUNDNESS"]),
 ("CQ_03","How is the answer's relevance to the question under discussion judged?",["nodes","conflict_axes"],"RELEVANCE anchors topical relevance to an explicit QUD, gated by question quality",["RELEVANCE","QUESTION_QUALITY"]),
 ("CQ_04","How are scope match and granularity match between question and answer checked?",["nodes"],"SCOPE_MATCH and GRANULARITY_MATCH check requested breadth and detail level",["SCOPE_MATCH","GRANULARITY_MATCH"]),
 ("CQ_05","How are directness and deflection/non-answers distinguished?",["nodes","workflow"],"DIRECTNESS scores responsiveness; DEFLECTION_DETECT flags non-answers",["DIRECTNESS","DEFLECTION_DETECT"]),
 ("CQ_06","How are the answer's informativeness and specificity scored?",["nodes"],"INFORMATIVENESS scores uncertainty reduction per the Gricean quantity maxim",["INFORMATIVENESS"]),
 ("CQ_07","How are answer adequacy and multi-part coverage evaluated?",["nodes"],"ANSWER_ADEQUACY judges completion; ANSWER_COVERAGE maps each sub-question",["ANSWER_ADEQUACY","ANSWER_COVERAGE"]),
 ("CQ_08","How is the evidential warrant linking grounds to claims evaluated?",["nodes"],"WARRANT_EVAL checks Toulmin warrant and backing for load-bearing claims",["WARRANT_EVAL"]),
 ("CQ_09","How are the answer's evidence tier and falsifiability assigned?",["nodes"],"EVIDENCE_TIER tiers support strength; FALSIFIABLE_CHECK finds confirm/refute paths",["EVIDENCE_TIER","FALSIFIABLE_CHECK"]),
 ("CQ_10","How are internal consistency and confidence calibration evaluated?",["nodes","conflict_axes"],"CONSISTENCY detects contradictions; CALIBRATION scores confidence vs evidence",["CONSISTENCY","CALIBRATION"]),
 ("CQ_11","How is appropriate hedging distinguished from over- and under-hedging?",["nodes"],"HEDGING_EVAL judges qualifiers against the evidence the answer cites",["HEDGING_EVAL"]),
 ("CQ_12","How are overclaim, under-support, and deflection detected?",["nodes","edge_cases"],"OVERCLAIM_DETECT and DEFLECTION_DETECT separate overreach and evasion from honest answers",["OVERCLAIM_DETECT","DEFLECTION_DETECT"]),
 ("CQ_13","What multi-dimensional rubric and aggregation produce comparable scores?",["nodes","conflict_axes"],"MULTIDIM_RUBRIC defines anchored dimensions; RUBRIC_AGGREGATE combines them",["MULTIDIM_RUBRIC","RUBRIC_AGGREGATE"]),
 ("CQ_14","How are candidate answers ranked comparably and consistently?",["nodes","workflow"],"COMPARATIVE_RANK orders answers from rubric profiles with a transitivity check",["COMPARATIVE_RANK","RUBRIC_AGGREGATE"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

CQ_MAP = {
 "WELLFORMED":["CQ_01"], "PRESUP_SOUNDNESS":["CQ_01","CQ_02"], "QUESTION_QUALITY":["CQ_02","CQ_03"],
 "RELEVANCE":["CQ_03","CQ_05"], "SCOPE_MATCH":["CQ_04"], "GRANULARITY_MATCH":["CQ_04"],
 "DIRECTNESS":["CQ_05"], "INFORMATIVENESS":["CQ_06"], "ANSWER_ADEQUACY":["CQ_07"],
 "ANSWER_COVERAGE":["CQ_07"], "WARRANT_EVAL":["CQ_08"], "EVIDENCE_TIER":["CQ_09"],
 "FALSIFIABLE_CHECK":["CQ_09"], "CONSISTENCY":["CQ_10"], "CALIBRATION":["CQ_10","CQ_11"],
 "HEDGING_EVAL":["CQ_11"], "OVERCLAIM_DETECT":["CQ_12"], "DEFLECTION_DETECT":["CQ_05","CQ_12"],
 "MULTIDIM_RUBRIC":["CQ_13"], "RUBRIC_AGGREGATE":["CQ_13","CQ_14"], "COMPARATIVE_RANK":["CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

GL = [
 ("question_under_discussion","the proposition the current question requests an answer to, against which relevance and adequacy are judged",["QUD","conversational_topic"],["rhetorical_question"],["RELEVANCE","SCOPE_MATCH","ANSWER_ADEQUACY"]),
 ("direct_answer","a response that supplies exactly the proposition the question requests, in Belnap & Steel's answerhood sense",["responsive_answer"],["partial_answer","corrective_reply"],["DIRECTNESS","ANSWER_ADEQUACY"]),
 ("presupposition","a proposition taken for granted by a question or answer that must hold for it to be felicitous",["taken_for_granted_premise"],["assertion","entailment"],["PRESUP_SOUNDNESS","WELLFORMED"]),
 ("warrant","the inference license, in Toulmin's terms, that moves from stated grounds to the asserted claim",["inference_license"],["claim","grounds"],["WARRANT_EVAL","OVERCLAIM_DETECT"]),
 ("evidence_tier","an ordered label for the strength of support behind a claim, from unsourced assertion to primary evidence",["support_strength_level"],["citation_count"],["EVIDENCE_TIER","FALSIFIABLE_CHECK"]),
 ("calibration","the degree to which expressed confidence matches the actual likelihood the claim is correct given its evidence",["confidence_accuracy"],["fluency","assertiveness"],["CALIBRATION","HEDGING_EVAL"]),
 ("overclaim","an assertion whose strength exceeds what its warrant and evidence tier support",["over_assertion","unsupported_strength"],["warranted_strong_claim"],["OVERCLAIM_DETECT","WARRANT_EVAL"]),
 ("deflection","a response that avoids the requested proposition by switching, restating, or padding the question",["evasion","non_answer"],["honest_refusal"],["DEFLECTION_DETECT","DIRECTNESS"]),
 ("multidimensional_rubric","a scoring instrument with named, level-anchored dimensions yielding a per-answer profile",["scoring_rubric","evaluation_matrix"],["single_overall_score"],["MULTIDIM_RUBRIC","RUBRIC_AGGREGATE"]),
 ("aggregation_policy","the rule combining per-dimension sub-scores into one comparable score, including weights and vetoes",["scoring_aggregation"],["raw_dimension_profile"],["RUBRIC_AGGREGATE","COMPARATIVE_RANK"]),
]
GLS=[{"term":t,"definition":d,"synonyms":s,"not_same_as":ns,"used_by_nodes":u} for (t,d,s,ns,u) in GL]

# ---- edges ----
E=[]
def dep(f,t,rs,cc=0.82,erc=0.28,cp=0.14,why="",ben="",rk="",ex=""):
    E.append({"from":f,"to":t,"edge_type":"dependency","relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{t} depends on {f}","benefit_of_coupling":ben or "ordered prerequisite",
              "risk_of_conflict":rk or "downstream rework if upstream changes","example":ex or f"{f} scored before {t}"})
def conf(f,t,rs,st,rule,why,cp=0.5,erc=0.5,cc=0.6):
    E.append({"from":f,"to":t,"edge_type":"conflict","relation_strength":rs,"signed_tension":st,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "resolution_rule":rule,"why_related":why,"benefit_of_coupling":"tension surfaced and resolved by rule",
              "risk_of_conflict":"unmanaged tension degrades evaluation quality","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated behavior",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies)
dep("WELLFORMED","PRESUP_SOUNDNESS",0.84)
dep("PRESUP_SOUNDNESS","QUESTION_QUALITY",0.8)
dep("QUESTION_QUALITY","RELEVANCE",0.84)
dep("RELEVANCE","SCOPE_MATCH",0.8)
dep("RELEVANCE","DIRECTNESS",0.8)
dep("RELEVANCE","INFORMATIVENESS",0.82)
dep("RELEVANCE","WARRANT_EVAL",0.8)
dep("INFORMATIVENESS","ANSWER_ADEQUACY",0.82)
dep("ANSWER_ADEQUACY","ANSWER_COVERAGE",0.8)
dep("WARRANT_EVAL","EVIDENCE_TIER",0.82)
dep("WARRANT_EVAL","FALSIFIABLE_CHECK",0.78)
dep("INFORMATIVENESS","CONSISTENCY",0.76)
dep("WARRANT_EVAL","CALIBRATION",0.84)
dep("EVIDENCE_TIER","CALIBRATION",0.8)
dep("CONSISTENCY","CALIBRATION",0.76)
dep("CALIBRATION","HEDGING_EVAL",0.8)
dep("CALIBRATION","OVERCLAIM_DETECT",0.82)
dep("WARRANT_EVAL","OVERCLAIM_DETECT",0.78)
dep("DIRECTNESS","DEFLECTION_DETECT",0.8)
dep("RELEVANCE","DEFLECTION_DETECT",0.76)
dep("CALIBRATION","MULTIDIM_RUBRIC",0.76)
dep("DEFLECTION_DETECT","MULTIDIM_RUBRIC",0.72)
dep("MULTIDIM_RUBRIC","RUBRIC_AGGREGATE",0.84)
dep("QUESTION_QUALITY","RUBRIC_AGGREGATE",0.72)
dep("RUBRIC_AGGREGATE","COMPARATIVE_RANK",0.82)
# cross-cutting non-dependency edges
rel("ANSWER_COVERAGE","RUBRIC_AGGREGATE","constraint",0.74,why="sub-question coverage is an obligation the aggregation must reflect, not average away")
rel("GRANULARITY_MATCH","ANSWER_ADEQUACY","constraint",0.72,why="granularity mismatch bounds how adequate an otherwise complete answer can be")
rel("HEDGING_EVAL","OVERCLAIM_DETECT","similarity",0.72,why="hedging quality and overclaim are dual views of confidence-vs-evidence fit")
rel("OVERCLAIM_DETECT","DEFLECTION_DETECT","similarity",0.7,why="overclaim and deflection are the two honesty failure poles the rubric must both catch")
rel("FALSIFIABLE_CHECK","CALIBRATION","causal",0.72,why="unfalsifiable claims cannot be calibrated, so falsifiability feeds calibration scoring")
rel("EVIDENCE_TIER","OVERCLAIM_DETECT","causal",0.74,why="a low evidence tier under a strong claim is the signature of overclaim")
rel("PRESUP_SOUNDNESS","DEFLECTION_DETECT","feedback",0.7,why="an answer that exploits a false presupposition is a form of deflection to flag back")
rel("COMPARATIVE_RANK","MULTIDIM_RUBRIC","feedback",0.7,why="ranking disagreements feed back to refine ambiguous rubric anchors")

# conflict edges (negative signed_tension + resolution_rule): real tensions
conf("INFORMATIVENESS","CALIBRATION",0.74,-0.6,
  "cap informativeness credit at the warranted evidence tier: reward specificity only up to the point the evidence supports; beyond that, calibration and overclaim penalties dominate",
  "maximizing informativeness/specificity pushes toward claims stronger than the evidence warrants, which calibration penalizes")
conf("DIRECTNESS","HEDGING_EVAL",0.7,-0.55,
  "require the direct answer to lead, but do not penalize qualifiers that carry warranted, decision-relevant uncertainty; only evasive over-hedging reduces directness",
  "demanding a direct lead-with-the-answer response conflicts with honest hedging that genuine uncertainty requires")
conf("RUBRIC_AGGREGATE","MULTIDIM_RUBRIC",0.7,-0.5,
  "always ship the per-dimension profile alongside the aggregate, and let critical dimensions veto rather than average, so the single score never replaces the transparent profile",
  "collapsing the multi-dimensional profile into one aggregate score trades transparency for comparability")
conf("WARRANT_EVAL","ANSWER_ADEQUACY",0.68,-0.5,
  "for hard or open questions, score warrant and adequacy on a curve relative to what is knowable: a well-warranted partial answer outranks an unwarranted complete-sounding one",
  "a strict warrant requirement can make hard but legitimately answerable questions look inadequate, penalizing honest partial answers")

CA=[
 {"name":"informativeness_vs_calibration","description":"Rewarding specific, uncertainty-reducing answers pushes toward commitments stronger than the evidence warrants; calibration and overclaim detection pull the other way.","poles":["max_informativeness","evidence_warranted_confidence"],"resolution_hint":"credit specificity only up to the warranted evidence tier","tension_score":0.74,"affected_nodes":["INFORMATIVENESS","CALIBRATION","OVERCLAIM_DETECT"]},
 {"name":"directness_vs_honest_hedging","description":"Demanding a direct lead-with-the-answer response competes with the qualification honest uncertainty requires.","poles":["lead_with_answer","warranted_qualification"],"resolution_hint":"penalize only evasive over-hedging, not warranted decision-relevant uncertainty","tension_score":0.7,"affected_nodes":["DIRECTNESS","HEDGING_EVAL","DEFLECTION_DETECT"]},
 {"name":"single_aggregate_vs_multidim_transparency","description":"A single comparable score is convenient but hides which dimension drove the verdict; the full profile is transparent but harder to rank.","poles":["single_score","full_profile"],"resolution_hint":"ship the profile with the aggregate; use vetoes not averaging for critical dimensions","tension_score":0.7,"affected_nodes":["RUBRIC_AGGREGATE","MULTIDIM_RUBRIC","COMPARATIVE_RANK"]},
 {"name":"strict_warrant_vs_answering_hard_questions","description":"A strict warrant/evidence requirement penalizes honest partial answers to genuinely hard or open questions.","poles":["strict_warrant","credit_honest_partial"],"resolution_hint":"score warrant and adequacy on a curve relative to what is knowable","tension_score":0.68,"affected_nodes":["WARRANT_EVAL","ANSWER_ADEQUACY","EVIDENCE_TIER"]},
 {"name":"scope_breadth_vs_focus","description":"Broad answers cover more of a multi-part question but risk over-scope and dilution; focused answers are crisp but may under-cover.","poles":["broad_coverage","focused_directness"],"resolution_hint":"match scope to the QUD's requested breadth; reward coverage only of asked parts","tension_score":0.62,"affected_nodes":["SCOPE_MATCH","ANSWER_COVERAGE","DIRECTNESS"]},
 {"name":"specificity_vs_falsifiability_safety","description":"More specific claims are more informative but also more falsifiable and thus riskier; vague claims are safe but uninformative.","poles":["specific_falsifiable","safe_vague"],"resolution_hint":"reward specific falsifiable claims while pricing in their error risk via calibration","tension_score":0.6,"affected_nodes":["INFORMATIVENESS","FALSIFIABLE_CHECK","CALIBRATION"]},
 {"name":"relevance_strictness_vs_helpful_context","description":"Strict QUD relevance penalizes added context; lenient relevance rewards tangents that pad the answer.","poles":["strict_qud_relevance","helpful_context"],"resolution_hint":"credit context only when it materially serves the QUD; flag the rest as over-scope","tension_score":0.6,"affected_nodes":["RELEVANCE","SCOPE_MATCH","INFORMATIVENESS"]},
 {"name":"granularity_for_experts_vs_novices","description":"The right level of detail differs by audience, so a fixed granularity target mis-serves some readers.","poles":["expert_granularity","novice_granularity"],"resolution_hint":"score granularity against the question's stated or inferred audience, not a fixed level","tension_score":0.55,"affected_nodes":["GRANULARITY_MATCH","SCOPE_MATCH","ANSWER_ADEQUACY"]},
 {"name":"deflection_flagging_vs_honest_refusal","description":"Aggressive deflection detection risks penalizing a legitimate, well-reasoned 'I cannot answer, and here is why'.","poles":["flag_all_nonanswers","credit_honest_refusal"],"resolution_hint":"distinguish a justified refusal with reasons from an evasive non-answer","tension_score":0.6,"affected_nodes":["DEFLECTION_DETECT","DIRECTNESS","WARRANT_EVAL"]},
]

EC=[
 {"description":"A loaded question with a false presupposition is answered on its own terms, rewarding an answer that accepts the false premise.","trigger":"presupposition soundness not checked before scoring the answer","affected_nodes":["PRESUP_SOUNDNESS","WELLFORMED","DEFLECTION_DETECT"],"mitigation":"flag the false presupposition; credit answers that correct rather than accept it","severity":"high"},
 {"description":"A fluent, confident answer to a neighbouring question scores high on tone but never addresses the actual QUD.","trigger":"relevance judged against a paraphrased rather than explicit QUD","affected_nodes":["RELEVANCE","DEFLECTION_DETECT"],"mitigation":"state the QUD explicitly and score relevance and deflection against it","severity":"high"},
 {"description":"An answer asserts spurious precision (a sharp number) where the evidence supports only a range, and informativeness scoring rewards it.","trigger":"informativeness credited without checking warrant and evidence tier","affected_nodes":["INFORMATIVENESS","CALIBRATION","OVERCLAIM_DETECT"],"mitigation":"cap informativeness at the warranted tier; apply an overclaim penalty","severity":"high"},
 {"description":"A multi-part question is answered only in its easy part, and the salient part dominates the score.","trigger":"sub-question coverage not computed for a complex question","affected_nodes":["ANSWER_COVERAGE","ANSWER_ADEQUACY","SCOPE_MATCH"],"mitigation":"decompose into sub-questions and require each to map to a covering span or gap","severity":"high"},
 {"description":"An answer hedges everything to 'maybe' to dodge a calibration penalty, sacrificing all informativeness.","trigger":"calibration penalty applied without rewarding warranted specificity","affected_nodes":["CALIBRATION","HEDGING_EVAL","INFORMATIVENESS"],"mitigation":"reward hedging only when warranted; penalize evasive over-hedging via directness","severity":"medium"},
 {"description":"A causal claim is asserted from purely correlational grounds with no warrant, but reads as authoritative.","trigger":"warrant from grounds to claim not evaluated","affected_nodes":["WARRANT_EVAL","OVERCLAIM_DETECT","EVIDENCE_TIER"],"mitigation":"require a recoverable warrant; flag correlation-to-causation leaps as overclaim","severity":"high"},
 {"description":"An unfalsifiable, self-sealing statement is scored as a safe, cautious answer.","trigger":"falsifiability not checked, so a non-committal claim looks prudent","affected_nodes":["FALSIFIABLE_CHECK","CALIBRATION","DEFLECTION_DETECT"],"mitigation":"flag self-sealing statements as unfalsifiable, not as warranted caution","severity":"medium"},
 {"description":"The aggregate score averages a critical deflection failure away into a passing total.","trigger":"aggregation averages a veto-worthy dimension instead of gating on it","affected_nodes":["RUBRIC_AGGREGATE","DEFLECTION_DETECT","MULTIDIM_RUBRIC"],"mitigation":"let critical dimensions veto the total; always ship the profile with the score","severity":"high"},
 {"description":"Pairwise answer comparisons produce an intransitive cycle (A>B>C>A), so no consistent ranking exists.","trigger":"ranking taken from inconsistent pairwise judgments without a transitivity check","affected_nodes":["COMPARATIVE_RANK","RUBRIC_AGGREGATE"],"mitigation":"check transitivity; resolve cycles by aggregating dimension sub-scores","severity":"medium"},
 {"description":"An honest, well-reasoned refusal ('I cannot answer because the data does not exist') is mis-flagged as deflection.","trigger":"deflection detection ignores justified refusals with stated reasons","affected_nodes":["DEFLECTION_DETECT","WARRANT_EVAL","DIRECTNESS"],"mitigation":"credit a refusal that gives warranted reasons; reserve deflection for evasion","severity":"medium"},
 {"description":"An answer correct at expert granularity is penalized as too terse for a novice question, or vice versa.","trigger":"granularity scored against a fixed level instead of the question's audience","affected_nodes":["GRANULARITY_MATCH","SCOPE_MATCH","ANSWER_ADEQUACY"],"mitigation":"score granularity against the question's stated or inferred audience","severity":"low"},
 {"description":"An answer contradicts itself across sections (declares 'unknown' then asserts the value), yet scores well on each part in isolation.","trigger":"internal consistency not checked across the whole response","affected_nodes":["CONSISTENCY","CALIBRATION","INFORMATIVENESS"],"mitigation":"run a whole-response contradiction check before crediting any claim","severity":"medium"},
]

WF=[
 {"action":"check_question_wellformedness","node_ref":"WELLFORMED","description":"Verify the question is intelligible, determinate, and carries satisfiable presuppositions before any answer scoring.","artifact":"wellformedness_report","gate":"question has a determinate reading and satisfiable presuppositions"},
 {"action":"check_presuppositions","node_ref":"PRESUP_SOUNDNESS","description":"Evaluate the truth and acceptance of the question's and answer's presuppositions; flag loaded/complex questions.","artifact":"presupposition_report","gate":"presupposition truth explicitly checked"},
 {"action":"score_question_quality","node_ref":"QUESTION_QUALITY","description":"Aggregate well-formedness, presupposition soundness, specificity, and answerability into a question-quality score with named sub-scores.","artifact":"question_quality_profile","gate":"question score decomposes into reproducible sub-scores"},
 {"action":"judge_relevance","node_ref":"RELEVANCE","description":"State the QUD explicitly and judge the answer's topical relevance to it.","artifact":"relevance_judgment","gate":"QUD stated; answer judged against it"},
 {"action":"check_scope_and_granularity","node_ref":"SCOPE_MATCH","description":"Map answer breadth and detail level to the question's requested scope and granularity, flagging under/over.","artifact":"scope_granularity_report","gate":"under/over-scope and granularity flags computed"},
 {"action":"score_directness","node_ref":"DIRECTNESS","description":"Locate the requested proposition and score how directly and prominently the answer supplies it.","artifact":"directness_score","gate":"requested proposition located and its prominence scored"},
 {"action":"score_informativeness_and_adequacy","node_ref":"INFORMATIVENESS","description":"Score uncertainty reduction and specificity, then adequacy and sub-question coverage.","artifact":"content_score_profile","gate":"informativeness, adequacy, and coverage scored"},
 {"action":"evaluate_warrant_and_evidence","node_ref":"WARRANT_EVAL","description":"Check the warrant from grounds to claims, assign an evidence tier, and test falsifiability.","artifact":"evidence_profile","gate":"each load-bearing claim has a warrant and evidence tier"},
 {"action":"score_calibration_and_honesty","node_ref":"CALIBRATION","description":"Score confidence calibration, hedging quality, overclaim, and deflection against the evidence.","artifact":"honesty_profile","gate":"confidence scored vs evidence; overclaim and deflection flagged"},
 {"action":"apply_multidim_rubric","node_ref":"MULTIDIM_RUBRIC","description":"Record the per-dimension, level-anchored sub-scores into a transparent answer profile.","artifact":"rubric_profile","gate":"each dimension has an independent anchored sub-score"},
 {"action":"aggregate_score","node_ref":"RUBRIC_AGGREGATE","description":"Combine sub-scores with explicit weights and vetoes, shipping the profile alongside the aggregate.","artifact":"aggregate_score_with_profile","gate":"weights/vetoes explicit; profile attached"},
 {"action":"rank_candidates","node_ref":"COMPARATIVE_RANK","description":"Rank candidate answers from their rubric profiles and check the ordering for transitivity.","artifact":"answer_ranking","gate":"ranking is transitive; ties broken by named dimensions"},
]

DR=[
 {"rule":"WELLFORMED and PRESUP_SOUNDNESS must pass before any answer dimension is scored","rationale":"scoring an answer to an ill-formed or loaded question conflates question and answer defects","trigger":"answer scoring begins on an unchecked question","action":"block answer scoring until the question is well-formed and its presuppositions checked"},
 {"rule":"RELEVANCE to an explicit QUD must be established before informativeness, adequacy, or warrant are credited","rationale":"informative, well-warranted content about the wrong question is still a non-answer","trigger":"content dimensions scored without a stated QUD","action":"require an explicit QUD and an on-question relevance judgment first"},
 {"rule":"INFORMATIVENESS credit must be capped at the warranted evidence tier","rationale":"rewarding specificity beyond the evidence incentivizes overclaim","trigger":"a precise claim scored informative above its evidence tier","action":"cap informativeness at the warranted tier and apply the overclaim penalty"},
 {"rule":"DEFLECTION_DETECT must distinguish a justified refusal from an evasive non-answer","rationale":"penalizing honest 'cannot answer, here is why' responses rewards confident guessing","trigger":"a refusal with stated reasons flagged as deflection","action":"credit warranted refusals; reserve the deflection flag for evasion"},
 {"rule":"RUBRIC_AGGREGATE must ship the per-dimension profile and let critical dimensions veto","rationale":"averaging hides a critical failure behind a passing total","trigger":"an aggregate reported without its profile or with a critical failure averaged away","action":"attach the profile and gate on veto dimensions"},
 {"rule":"WARRANT_EVAL and ANSWER_ADEQUACY must be scored relative to what is knowable for hard questions","rationale":"a strict absolute standard penalizes honest partial answers to open problems","trigger":"an open-question answer marked inadequate against a closed-question standard","action":"score warrant and adequacy on a knowability-relative curve"},
 {"rule":"CONSISTENCY must be checked across the whole response before any claim is credited","rationale":"a self-contradictory answer can score well on each part in isolation","trigger":"per-part scoring without a whole-response contradiction check","action":"run a whole-response consistency check first"},
 {"rule":"COMPARATIVE_RANK must verify transitivity before publishing an ordering","rationale":"intransitive pairwise preferences yield an undefined ranking","trigger":"a ranking derived from inconsistent pairwise judgments","action":"check transitivity and resolve cycles via dimension sub-scores"},
]

ARR=[
 {"rule":"Do not score answer dimensions before the question is well-formed and its presuppositions checked; question defects otherwise contaminate answer scores","prevents":"re-scoring every answer after discovering the question was loaded"},
 {"rule":"Do not judge relevance against a paraphrased QUD; fixing a mis-identified QUD invalidates all dependent scores","prevents":"re-running the full rubric after the QUD is corrected"},
 {"rule":"Do not credit informativeness above the warranted evidence tier; retracting inflated scores after an overclaim is found is costly","prevents":"reworking informativeness and calibration scores after an overclaim audit"},
 {"rule":"Do not collapse the rubric to a single number without retaining the profile; reconstructing dimension scores later is expensive","prevents":"re-annotating answers to recover a discarded per-dimension profile"},
 {"rule":"Do not finalize calibration before falsifiability is checked; an unfalsifiable claim cannot be calibrated","prevents":"re-doing calibration after discovering a claim was self-sealing"},
 {"rule":"Do not flag deflection without checking for a justified refusal; reversing a wrong deflection flag means re-scoring honesty","prevents":"honesty re-scoring after a refusal was mis-flagged as evasion"},
 {"rule":"Do not rank answers before checking transitivity; an intransitive ranking must be discarded and rebuilt","prevents":"republishing a ranking after a cycle is detected"},
 {"rule":"Do not aggregate before sub-question coverage is computed for complex questions; a missing sub-question surfaces as a late coverage gap","prevents":"re-aggregating after a multi-part question is found under-covered"},
]

IP=[
 {"trigger":"a loaded or ill-formed question reaches answer scoring unflagged","action":"tighten WELLFORMED and PRESUP_SOUNDNESS gates and re-run question-quality scoring","nodes":["WELLFORMED","PRESUP_SOUNDNESS","QUESTION_QUALITY"],"priority":"critical"},
 {"trigger":"a fluent non-answer to a neighbouring question scores high","action":"make the QUD explicit in RELEVANCE and strengthen DEFLECTION_DETECT","nodes":["RELEVANCE","DEFLECTION_DETECT"],"priority":"high"},
 {"trigger":"a confident but under-supported answer outscores a warranted hedged one","action":"cap INFORMATIVENESS at the evidence tier and strengthen CALIBRATION and OVERCLAIM_DETECT","nodes":["INFORMATIVENESS","CALIBRATION","OVERCLAIM_DETECT"],"priority":"high"},
 {"trigger":"a multi-part question is scored on only its easy part","action":"enforce sub-question decomposition in ANSWER_COVERAGE and gate ANSWER_ADEQUACY on it","nodes":["ANSWER_COVERAGE","ANSWER_ADEQUACY"],"priority":"high"},
 {"trigger":"a critical failure is averaged into a passing aggregate","action":"add a veto for the critical dimension in RUBRIC_AGGREGATE and attach the MULTIDIM_RUBRIC profile","nodes":["RUBRIC_AGGREGATE","MULTIDIM_RUBRIC"],"priority":"high"},
 {"trigger":"an honest justified refusal is mis-flagged as deflection","action":"refine DEFLECTION_DETECT to credit warranted refusals using WARRANT_EVAL","nodes":["DEFLECTION_DETECT","WARRANT_EVAL"],"priority":"medium"},
 {"trigger":"a comparative ranking produces an intransitive cycle","action":"add a transitivity check in COMPARATIVE_RANK and resolve via RUBRIC_AGGREGATE sub-scores","nodes":["COMPARATIVE_RANK","RUBRIC_AGGREGATE"],"priority":"medium"},
]

spec = {
 "domain":"qeval__answer_quality",
 "domain_label":"Question & Answer Quality Evaluation (Question Science subdomain)",
 "purpose":"session_bounded_method_for_scoring_questions_and_answers_on_a_multidimensional_rubric_covering_wellformedness_informativeness_relevance_to_the_question_under_discussion_adequacy_and_coverage_evidential_warrant_calibration_and_detection_of_deflection_and_overclaim_to_yield_transparent_comparable_per_dimension_scores",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "evaluation is neutral and informational, scoring quality rather than enforcing any policy",
   "a single evaluation session has access to the question, the answer(s), and the question under discussion",
   "scores are relative and comparable within a session, not absolute ground-truth quality",
 ],
 "exclusions":[
   "factual verification of claims against external ground truth (delegated to a fact-checking KB)",
   "safety, policy, or content-moderation judgments beyond honesty/overclaim",
   "model-internal or training-time evaluation; this is response-level scoring",
   "automated answer generation or repair; this KB only scores",
 ],
 "source_description":"heuristic prior estimates for question-and-answer quality evaluation work units, informed by Gricean pragmatics, erotetic/answerhood theory, argumentation theory, IR relevance, and calibration research; no supplied dataset",
 "source_citation":"Grice 1975 Logic and Conversation (maxims of quantity, quality, relation, manner); Belnap & Steel 1976 The Logic of Questions and Answers (answerhood); Toulmin 1958 The Uses of Argument (grounds/warrant/backing); Tetlock & Gardner 2015 Superforecasting (calibration); van Rijsbergen 1979 Information Retrieval (topical relevance); Roberts 2012 Information Structure: Towards an Integrated Formal Theory of Pragmatics (question under discussion)",
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
 "priority_rationale":"WELLFORMED, PRESUP_SOUNDNESS, and QUESTION_QUALITY gate the question; RELEVANCE anchors every answer dimension; content (informativeness/adequacy/coverage) and evidence (warrant/tier/falsifiability/consistency) feed honesty (calibration/hedging/overclaim/deflection); the multi-dimensional rubric, aggregation, and ranking close the method last.",
 "eval_objective":"verify_wellformedness_relevance_informativeness_adequacy_warrant_calibration_and_deflection_overclaim_detection_yield_transparent_comparable_scores_in_qeval__answer_quality_kb",
}

out_dir = "branches/b10_question_compiler/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "qeval__answer_quality.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
