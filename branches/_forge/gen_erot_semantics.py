#!/usr/bin/env python3
"""Generate the erot__question_semantics content spec (B10 KB) for kb_forge.py.
Compact authoring: node() applies sane defaults so only domain content + base
metric magnitudes are specified per node. Domain = erotetic logic & question
semantics (the logical analysis of questions, their presuppositions, and their
answerhood conditions)."""
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
        "academic_fields": ["philosophical_logic", "formal_semantics", "linguistics"],
        "subfields": subfields or ["erotetic_logic", "question_semantics"],
        "specialists": specialists or ["erotetic_logician"],
        "contradictors": contradictors or ["pragmatic_answer_relevance_advocate"],
        "inputs": inputs or ["natural-language question", "context / common ground"],
        "outputs": outputs or ["logical analysis of the question"],
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

# ---------- foundations: question typology ----------
N.append(node("QTYPE_POLAR","polar_yes_no_question_type",
  "Classify a question as polar (yes/no): a question whose logical form is a single proposition p offered for affirmation or denial, so its core answer space is the two-cell set {p, not-p} (optionally with 'unknown').",
  "typology", [], ["CQ_01"],
  b(0.86,0.74,0.78,0.5,0.78,0.7,0.55,0.74,0.4, 0.8,0.82, 0.86,0.72,0.16,0.48,[0.18,0.46],"polar-question grammar or yes/no answer-space convention is revised"),
  ["recognition of yes/no interrogative form","the underlying proposition p","the {p, not-p} answer space"],
  ["wh-variable extraction","explanation/why analysis"],
  [pro("Polar typing fixes a minimal, decidable answer space {p, not-p}, making answerhood mechanically checkable","'Is the door closed?' has direct answers 'yes (closed)' / 'no (not closed)'")],
  [con("Surface yes/no form can mask an alternative or biased question, mis-typing it as plain polar","'Did you stop lying?' is polar in form but carries a loaded presupposition")],
  ["treating a tag/bias question as neutral polar","collapsing 'unknown' into a forced yes/no"],
  ["the underlying proposition p is extracted and the answer space is exactly {p, not-p}(+unknown)"],
  ["yes/no convention changes","a polar surface masks alternatives or bias"],
  specialists=["erotetic_logician","semanticist"]))

N.append(node("QTYPE_WH","wh_constituent_question_type",
  "Classify a question as wh/constituent: a question built from an open sentence with one or more wh-variables (who/what/where/when/which) whose answers instantiate those variables; its answer space is the set of instances satisfying the matrix.",
  "typology", [], ["CQ_01","CQ_02"],
  b(0.88,0.76,0.8,0.6,0.8,0.74,0.55,0.7,0.45, 0.78,0.8, 0.88,0.7,0.18,0.5,[0.2,0.5],"wh-variable inventory or constituent-question schema changes"),
  ["wh-variable identification","the open sentence / question matrix","sortal restriction on the variable"],
  ["yes/no proposition extraction","why-question explanation space"],
  [pro("Representing a wh-question as a lambda-abstract over its matrix makes the set of instantiating answers explicit","'Who came?' = lambda-x.came(x); answers are the people who came")],
  [con("Multiple or nested wh-variables (pair-list, functional readings) explode the answer space and resist a single schema","'Which student read which book?' yields pair-list answers")],
  ["ignoring the sortal restriction on the wh-variable","conflating single-wh and multiple-wh readings"],
  ["each wh-variable and its sortal restriction is identified and the answer matrix is well-formed"],
  ["wh-variable inventory changes","a multiple/functional wh reading is observed"],
  specialists=["erotetic_logician","semanticist"]))

N.append(node("QTYPE_ALT","alternative_question_type",
  "Classify a question as alternative (disjunctive): a question presenting a closed list of mutually-exclusive, jointly-relevant options ('Tea or coffee?'), whose answers select exactly one (or a stated subset) of the listed alternatives.",
  "typology", [], ["CQ_01","CQ_02"],
  b(0.8,0.7,0.72,0.56,0.76,0.72,0.55,0.7,0.45, 0.78,0.8, 0.8,0.7,0.18,0.48,[0.2,0.5],"the enumerated alternative set or exclusivity convention changes"),
  ["the explicit list of alternatives","mutual-exclusivity / exhaustiveness presumption","selection-of-one answer form"],
  ["open wh enumeration","yes/no reduction"],
  [pro("An alternative question fixes a finite closed option set, so its admissible answers are exactly the listed alternatives","'Stairs or elevator?' admits 'stairs' / 'elevator', not 'yes'")],
  [con("Disjunctive surface is ambiguous between a polar reading ('do you want either?') and a true alternative reading","'Tea or coffee?' may be answered 'yes' under a polar intonation")],
  ["mis-reading an alternative question as polar","an alternative list that is not jointly exhaustive of the relevant space"],
  ["the closed alternative list is enumerated and exactly-one-of selection is the answer form"],
  ["alternative set is revised","polar/alternative ambiguity is detected"],
  specialists=["erotetic_logician","semanticist"]))

N.append(node("QTYPE_WHY","explanation_why_question_type",
  "Classify a question as a why/explanation question: a question requesting an explanans for a presupposed explanandum (a fact taken as holding), whose answers are explanatory accounts relative to a contrast class, not mere instances.",
  "typology", [], ["CQ_02","CQ_03"],
  b(0.82,0.72,0.74,0.66,0.82,0.78,0.58,0.62,0.5, 0.72,0.76, 0.82,0.62,0.22,0.54,[0.26,0.6],"explanation model or contrast-class convention changes"),
  ["the presupposed explanandum","the contrast class ('why P rather than Q')","explanatory-relevance relation"],
  ["mere instance enumeration","truth-value request"],
  [pro("Modeling a why-question with an explicit contrast class (van Fraassen) makes 'a good answer' relative and checkable","'Why did the bridge fail?' = rather than stand, asks for the failure-relevant cause")],
  [con("Explanatory relevance is theory-laden; what counts as an adequate explanans is not fixed by logical form alone","causal vs functional vs intentional explanations all 'answer' the same why")],
  ["answering a why-question with a restatement of the explanandum","ignoring the contrast class so the explanans is irrelevant"],
  ["the explanandum and its contrast class are identified and answers are explanatory, not instantial"],
  ["explanation model changes","contrast class is mis-identified"],
  specialists=["erotetic_logician","philosopher_of_science"],
  contradictors=["explanation_pragmatics_skeptic"]))

# ---------- presupposition layer ----------
N.append(node("PRESUPPOSITION","question_presupposition_extraction",
  "Extract the presuppositions of a question: the propositions that must be true for the question to have any (true) direct answer at all, including the existential/uniqueness presuppositions carried by its wh-variables and definite terms.",
  "presupposition", ["QTYPE_WH","QTYPE_POLAR","QTYPE_ALT"], ["CQ_03","CQ_04"],
  b(0.9,0.8,0.8,0.66,0.86,0.82,0.62,0.62,0.55, 0.72,0.76, 0.9,0.62,0.22,0.58,[0.26,0.62],"presupposition-projection theory or trigger inventory changes"),
  ["existential presupposition of wh-questions","uniqueness presupposition of singular which/the","factivity of embedded answers"],
  ["truth of the answer itself","conversational implicature"],
  [pro("A question's presupposition set is the entailment shared by all its direct answers (Belnap & Steel), making it computable from the answer space","'Who solved it?' presupposes someone solved it")],
  [con("Some presuppositions are defeasible or context-sensitive, so a fixed extraction can over- or under-generate","'When did you arrive?' presupposes arrival, cancellable by 'if you arrived'")],
  ["missing a uniqueness presupposition on a singular wh","treating a cancellable implicature as a hard presupposition"],
  ["the question's presupposition set is extracted and equals the common entailment of its direct answers"],
  ["presupposition theory changes","a defeasible presupposition is mis-classified as hard"],
  specialists=["erotetic_logician","semanticist"]))

N.append(node("FALSE_PRESUP","false_or_loaded_presupposition_detection",
  "Detect when a question carries a presupposition that is false or contestable in the context (a loaded or complex question) and FLAG it rather than answering: a question with a false presupposition has no true direct answer and must be rejected/corrected, not answered yes or no.",
  "presupposition", ["PRESUPPOSITION"], ["CQ_04","CQ_05"],
  b(0.92,0.84,0.86,0.64,0.9,0.82,0.68,0.6,0.62, 0.7,0.74, 0.92,0.6,0.24,0.66,[0.28,0.64],"false-presupposition policy or contestation threshold changes"),
  ["checking each presupposition against the common ground","the flag/reject/correct response","loaded-question (complex question) recognition"],
  ["choosing a direct answer","ranking admissible answers"],
  [pro("Flagging a false presupposition blocks the trap of a forced yes/no that concedes the false proposition (the classic loaded question)","'Have you stopped cheating?' is flagged, not answered, when cheating never occurred")],
  [con("Over-flagging treats every contestable background as a false presupposition, refusing answerable questions","flagging 'which exit did you take?' when an exit is in fact common ground")],
  ["answering a loaded question and thereby conceding its false presupposition","silently repairing instead of surfacing the false presupposition"],
  ["any question with a presupposition false-in-context is flagged with the offending presupposition, not answered"],
  ["false-presupposition policy changes","a loaded question is answered instead of flagged","over-flagging of answerable questions observed"],
  specialists=["erotetic_logician","argumentation_analyst"],
  contradictors=["charitable_repair_advocate"]))

# ---------- answerhood layer ----------
N.append(node("ANSWERHOOD","answerhood_condition_definition",
  "Define answerhood: the conditions under which a proposition counts as an answer to the question at all (resolving the question's issue), independent of whether it is true; this fixes what objects the question's semantics ranges over.",
  "answerhood", ["PRESUPPOSITION"], ["CQ_05","CQ_06"],
  b(0.9,0.8,0.8,0.68,0.86,0.84,0.6,0.62,0.55, 0.72,0.76, 0.9,0.62,0.2,0.58,[0.24,0.6],"answerhood criterion or resolvedness definition changes"),
  ["resolvedness / aboutness criterion","relevance to the question's issue","answer-vs-non-answer boundary"],
  ["truth evaluation of the answer","pragmatic helpfulness ranking"],
  [pro("A precise answerhood relation (Hamblin: a question denotes its set of possible answers) makes the question's meaning the set it partitions","Hamblin's postulate: knowing the meaning of a question is knowing what counts as an answer")],
  [con("Multiple answerhood notions (resolving vs partial vs relevant) compete and a single choice under-serves some uses","a partially-informative reply may 'answer' pragmatically but not resolve")],
  ["admitting an off-issue proposition as an answer","equating answerhood with truth"],
  ["the answerhood relation is fixed so that exactly the issue-resolving propositions qualify as answers"],
  ["answerhood criterion changes","a partial/relevant-answer notion must be added"],
  specialists=["erotetic_logician","semanticist"]))

N.append(node("DIRECT_ANSWER","direct_answer_set_construction",
  "Construct the set of direct answers: the canonical, minimal propositions that each fully and exactly resolve the question (Belnap & Steel's nuclear answers), one per cell of the question's issue, forming the semantic backbone of the question.",
  "answerhood", ["ANSWERHOOD"], ["CQ_06","CQ_07"],
  b(0.88,0.78,0.78,0.66,0.84,0.8,0.6,0.64,0.52, 0.74,0.78, 0.88,0.64,0.2,0.56,[0.22,0.56],"direct-answer canonicalization rule changes"),
  ["enumeration of direct (nuclear) answers","one canonical answer per issue cell","minimality / exactness of each direct answer"],
  ["partial answers","corrective (presupposition-denying) replies"],
  [pro("A fixed direct-answer set gives the question a determinate denotation against which every reply is measured","'Is it raining?' has direct answers {it is raining, it is not raining}")],
  [con("For open wh-questions the direct-answer set can be infinite, requiring a schema rather than an enumeration","'Which integer?' has unbounded direct answers")],
  ["omitting a direct answer (incomplete answer space)","admitting a non-minimal/over-complete proposition as 'direct'"],
  ["each direct answer is canonical and minimal, and the set covers every cell of the question's issue"],
  ["canonicalization rule changes","an infinite direct-answer set must be schematized"],
  specialists=["erotetic_logician"]))

N.append(node("COMPLETE_ANSWER","complete_vs_partial_answer_distinction",
  "Distinguish complete from partial answers and the mention-some vs mention-all reading: a complete answer settles the whole issue (mention-all / strongly exhaustive), a partial answer settles part of it, and mention-some questions are satisfied by any single true witness.",
  "answerhood", ["DIRECT_ANSWER"], ["CQ_07","CQ_08"],
  b(0.84,0.74,0.78,0.7,0.82,0.78,0.55,0.6,0.58, 0.72,0.76, 0.84,0.6,0.22,0.54,[0.26,0.62],"exhaustivity convention (mention-some vs mention-all) changes"),
  ["mention-all (strongly exhaustive) reading","mention-some reading","partial-answer partial order"],
  ["truth verification","explanation adequacy"],
  [pro("Separating mention-some from mention-all sets the right adequacy bar per question and prevents demanding exhaustivity where one witness suffices","'Where can I buy coffee?' (mention-some) is answered by one cafe")],
  [con("The mention-some/mention-all reading is often underdetermined by surface form, so the wrong adequacy bar can be applied","'Who can chair?' is ambiguous between one eligible person and all of them")],
  ["demanding mention-all where mention-some suffices","accepting a partial answer as if it were complete"],
  ["the question's exhaustivity reading (some/all) is fixed and the completeness bar matches it"],
  ["exhaustivity convention changes","a mention-some/all reading is mis-assigned"],
  specialists=["erotetic_logician","semanticist"]))

# ---------- partition / set semantics ----------
N.append(node("PARTITION_SEM","partition_semantics_of_questions",
  "Model a question as a partition of logical space (Groenendijk & Stokhof): the question's meaning is the equivalence relation on possible worlds that groups worlds giving the same complete true answer; each block is one strongly-exhaustive answer.",
  "semantics", ["ANSWERHOOD","COMPLETE_ANSWER"], ["CQ_08","CQ_09"],
  b(0.86,0.76,0.74,0.78,0.82,0.84,0.62,0.58,0.55, 0.7,0.74, 0.86,0.58,0.24,0.58,[0.3,0.66],"partition vs alternative (Hamblin) semantics choice changes"),
  ["worlds-to-answer equivalence relation","exhaustive answer = partition block","entailment-as-partition-refinement"],
  ["intensional answer ranking","speech-act force"],
  [pro("Partition semantics makes question entailment a refinement relation between partitions, giving a clean algebra of questions","a finer partition (more wh-variables) entails a coarser one")],
  [con("Strict partitions force strong exhaustivity and a complete cover, which is intractable for large/infinite world spaces and excludes mention-some","partitioning over an infinite domain is not finitely representable")],
  ["assuming a finite partition where the space is infinite","forcing mention-some questions into an exhaustive partition"],
  ["the question induces a well-defined partition (equivalence relation) whose blocks are the exhaustive answers"],
  ["partition/alternative semantics choice changes","an infinite or mention-some case breaks the partition assumption"],
  specialists=["formal_semanticist","erotetic_logician"],
  contradictors=["inquisitive_semantics_advocate"]))

# ---------- closed vs open admissibility ----------
N.append(node("ADMISSIBLE_SET","admissible_answer_set_for_closed_types",
  "Compute the admissible answer set for closed question types (polar and alternative): the small, finite, exhaustively enumerable set of permitted answers, against which any reply is checked for membership; non-members are inadmissible by construction.",
  "admissibility", ["DIRECT_ANSWER","QTYPE_POLAR","QTYPE_ALT"], ["CQ_09","CQ_10"],
  b(0.88,0.78,0.82,0.6,0.84,0.8,0.62,0.66,0.55, 0.78,0.82, 0.88,0.66,0.18,0.56,[0.2,0.5],"closed-type admissible-set construction rule changes"),
  ["finite enumeration of permitted answers","membership test for a reply","closed-type exhaustiveness guarantee"],
  ["open-type type-conformance","evidential warrant"],
  [pro("For closed types the admissible set is finite and decidable, so answerhood is a simple membership check","a polar question admits exactly {yes/p, no/not-p, unknown}")],
  [con("A rigid admissible set rejects an informative reply that resolves the issue in non-listed words","'half-open' truthfully answers 'Is the door closed?' but is not in {yes,no}")],
  ["a reply outside the admissible set silently accepted","admissible set not jointly exhaustive of the issue"],
  ["the closed-type admissible set is finite, exhaustive, and every reply is decided by membership"],
  ["closed-type construction rule changes","an informative non-listed reply must be accommodated"],
  specialists=["erotetic_logician"]))

N.append(node("TYPE_CONFORMANCE","answer_type_conformance_for_open_types",
  "Check type-conformance for open question types (wh, why): a reply must instantiate the wh-variable with an object of the correct sort (or supply an explanans of the right category for why), since open types have no finite admissible list, only a type/sort constraint.",
  "admissibility", ["DIRECT_ANSWER","QTYPE_WH","QTYPE_WHY"], ["CQ_10","CQ_11"],
  b(0.86,0.76,0.8,0.64,0.84,0.8,0.6,0.64,0.55, 0.76,0.8, 0.86,0.64,0.2,0.56,[0.22,0.54],"sortal-typing or category-matching rule for answers changes"),
  ["sortal match of the answer to the wh-variable","explanans category match for why-questions","rejection of category-mismatched replies"],
  ["finite admissible enumeration","truth of the reply"],
  [pro("Type-conformance replaces an impossible enumeration with a checkable sortal constraint for open questions","'Who came?' rejects 'at noon' (a time, not a person) as type-nonconforming")],
  [con("Sortal boundaries are fuzzy; coercion and metonymy let a type-mismatched surface answer be acceptable","'Who is the 9:15?' answered by a train number via metonymy")],
  ["accepting a category-mismatched answer","over-strict sorting that rejects a coerced but valid answer"],
  ["every accepted open-type answer instantiates the wh-variable/explanans with the correct sort or category"],
  ["sortal-typing rule changes","a metonymic/coerced answer challenges the sort check"],
  specialists=["erotetic_logician","semanticist"]))

N.append(node("WARRANT_REQ","mandatory_evidential_warrant_for_open_answers",
  "Require a mandatory evidential warrant for open-type answers: because an open answer asserts a contingent instance (who/what/why), it must come with or be defeasible by grounds; a type-conforming but unwarranted answer is admissible-in-form yet not acceptable-as-asserted.",
  "admissibility", ["TYPE_CONFORMANCE"], ["CQ_11","CQ_12"],
  b(0.84,0.76,0.78,0.68,0.86,0.78,0.6,0.6,0.58, 0.72,0.76, 0.84,0.6,0.24,0.6,[0.28,0.64],"evidential-warrant requirement or defeasibility policy changes"),
  ["warrant attachment to the asserted instance","defeasibility check","form-vs-assertion acceptability split"],
  ["closed-type membership","logical answerhood (form only)"],
  [pro("Demanding warrant for open answers blocks confident-but-groundless instantiations of who/what/why","'Smith did it' is type-conforming but rejected without evidence linking Smith")],
  [con("A blanket warrant requirement can over-burden genuinely a-priori or definitional answers that need no empirical grounds","'Who is the bachelor's spouse?' needs no evidence: it is unwarranted by definition")],
  ["accepting a groundless instantiation","demanding empirical warrant for a definitional answer"],
  ["every open-type answer either carries a warrant or is explicitly marked defeasible/ungrounded"],
  ["warrant policy changes","a definitional answer is wrongly required to carry empirical warrant"],
  specialists=["erotetic_logician","epistemologist"],
  contradictors=["logical_answerhood_purist"]))

# ---------- inter-question logic ----------
N.append(node("QENTAILMENT","question_entailment_and_erotetic_implication",
  "Define entailment between questions: question Q1 erotetically implies Q2 when every complete answer to Q1 yields a complete answer to Q2 (Q1's partition refines Q2's), capturing when answering one settles another.",
  "inter_question", ["PARTITION_SEM"], ["CQ_09","CQ_13"],
  b(0.82,0.72,0.7,0.78,0.82,0.82,0.6,0.58,0.55, 0.7,0.74, 0.82,0.58,0.24,0.56,[0.3,0.66],"erotetic-implication definition or refinement criterion changes"),
  ["answer-transmission criterion","partition-refinement between questions","question-to-question entailment"],
  ["declarative-to-question evocation","discourse role"],
  [pro("Question entailment lets a compiler reduce a question to an already-answered finer one, avoiding redundant inquiry","answering 'who and when?' settles 'who?'")],
  [con("Refinement-based entailment is strict (mention-all); it misses pragmatic reductions that hold only mention-some","a mention-some 'where to eat' is not entailed by the exhaustive list under strict refinement")],
  ["claiming entailment where exhaustivity assumptions differ","ignoring presupposition mismatch between the two questions"],
  ["Q1 implies Q2 iff every complete answer to Q1 determines a complete answer to Q2 with compatible presuppositions"],
  ["implication definition changes","a mention-some reduction is needed but refinement is strict"],
  specialists=["erotetic_logician"]))

N.append(node("EROTETIC_INFERENCE","inferential_erotetic_logic_question_evocation",
  "Model erotetic inference (Wisniewski): how a question is evoked by a set of declaratives or arises from another question plus declaratives, with criteria of soundness (the evoked question is sound and its answers are relevant) for question-generating inferences.",
  "inter_question", ["QENTAILMENT","PRESUPPOSITION"], ["CQ_13","CQ_14"],
  b(0.8,0.72,0.7,0.8,0.82,0.82,0.58,0.56,0.55, 0.7,0.74, 0.8,0.56,0.26,0.56,[0.32,0.68],"evocation/erotetic-implication soundness criteria change"),
  ["evocation of a question by declaratives","erotetic implication (question from question + declaratives)","soundness of the question-generating step"],
  ["answer truth-evaluation","speech-act force"],
  [pro("Inferential erotetic logic gives a calculus for which questions legitimately arise, driving principled follow-up inquiry","from 'exactly one suspect lied' evoke 'which suspect lied?'")],
  [con("Evocation criteria can license many sound questions, needing a relevance/utility filter to be operational","a sound but useless question is evoked alongside the relevant one")],
  ["evoking a question whose presupposition the premises do not secure","missing the relevance condition on the evoked question"],
  ["each evoked/implied question is sound w.r.t. its premises and its presupposition is entailed by them"],
  ["soundness criteria change","an irrelevant sound question must be filtered"],
  specialists=["erotetic_logician"],
  contradictors=["pragmatic_relevance_advocate"]))

# ---------- discourse layer ----------
N.append(node("QUD","question_under_discussion_discourse_role",
  "Place the question in the discourse as a Question Under Discussion (Roberts): the current QUD constrains relevance, accommodation, and which answers are felicitous, and structures the discourse as a stack/tree of questions and their moves.",
  "discourse", ["ANSWERHOOD"], ["CQ_06","CQ_15"],
  b(0.82,0.74,0.78,0.64,0.78,0.82,0.55,0.62,0.55, 0.72,0.76, 0.82,0.62,0.22,0.5,[0.26,0.6],"QUD relevance/accommodation model changes"),
  ["current-QUD identification","relevance of a move to the QUD","QUD stack/tree structure"],
  ["logical answer enumeration","truth verification"],
  [pro("The QUD framework makes relevance and information-structure (focus) a function of the active question, organizing discourse coherence","a focus pattern is licensed by the QUD it answers")],
  [con("Identifying the operative QUD is itself inferential and can be contested, so QUD-relative judgments inherit that uncertainty","accommodating a QUD that the speaker did not intend mis-reads the discourse")],
  ["assuming a single QUD where several are open","accommodating an unintended QUD"],
  ["the operative QUD is identified and each move is judged relevant relative to it"],
  ["QUD model changes","a contested/multiple QUD is detected"],
  specialists=["formal_pragmaticist","erotetic_logician"]))

N.append(node("SUBQUESTION","subquestion_and_question_subordination",
  "Represent subordination: a question Q' is a sub-question of Q when answering Q' is a step toward answering Q (Q' is entailed by or refines part of Q under the QUD), yielding the question's decomposition into a strategy of inquiry.",
  "discourse", ["QENTAILMENT","QUD"], ["CQ_15","CQ_16"],
  b(0.8,0.72,0.74,0.7,0.8,0.8,0.55,0.6,0.55, 0.72,0.76, 0.8,0.6,0.22,0.52,[0.26,0.6],"subordination criterion or strategy-of-inquiry model changes"),
  ["sub-question relation","decomposition of a question into a strategy","ordering of sub-questions"],
  ["full partition computation","answer warrant"],
  [pro("Subordination decomposes a hard question into answerable sub-questions, structuring inquiry as a tree","'who won and by how much?' subordinates 'who won?' then 'by how much?'")],
  [con("A wrong subordination can decompose into sub-questions whose answers do not recombine into a complete answer to the parent","answering two sub-questions may still leave the parent's issue open")],
  ["sub-questions whose answers do not compose to the parent answer","circular subordination between questions"],
  ["each sub-question is entailed by or refines the parent and their answers compose to a parent answer"],
  ["subordination criterion changes","non-composing sub-questions are detected"],
  specialists=["erotetic_logician","discourse_analyst"]))

# ---------- embedding / projection ----------
N.append(node("PRESUP_PROJECTION","presupposition_projection_in_embedded_questions",
  "Handle presupposition projection: determine which presuppositions of an embedded question survive (project) to the matrix and which are filtered, when a question is embedded under predicates ('knows whether', 'wonders who') or operators.",
  "embedding", ["PRESUPPOSITION","ANSWERHOOD"], ["CQ_04","CQ_17"],
  b(0.82,0.72,0.72,0.76,0.84,0.82,0.58,0.56,0.55, 0.7,0.74, 0.82,0.56,0.26,0.58,[0.32,0.68],"projection theory or embedding-predicate inventory changes"),
  ["projection of the embedded presupposition","filtering by the embedding predicate","factive vs non-factive embedding effects"],
  ["matrix speech-act force","answer truth"],
  [pro("Tracking projection prevents wrongly attributing an embedded question's presupposition to the speaker of the matrix","'Bob wonders who solved it' does not commit the matrix speaker to someone solving it")],
  [con("Projection behavior varies by predicate and is theory-dependent, so a single projection rule mis-predicts some embeddings","'knows who' (factive) projects existence; 'wonders who' may not")],
  ["projecting a presupposition that the predicate filters","failing to project a factive presupposition"],
  ["the embedded presuppositions that project vs are filtered are correctly assigned per embedding predicate"],
  ["projection theory changes","a new embedding predicate's behavior is observed"],
  specialists=["formal_semanticist","erotetic_logician"]))

N.append(node("NEGATIVE_QUESTION","negative_and_biased_question_bias",
  "Analyze negative/biased questions: questions whose form (high vs low negation, tag questions, 'really') signals a prior epistemic bias toward a particular answer, distinguishing the encoded bias from the neutral propositional content.",
  "embedding", ["QTYPE_POLAR","PRESUPPOSITION"], ["CQ_05","CQ_18"],
  b(0.78,0.7,0.74,0.68,0.8,0.76,0.55,0.6,0.55, 0.72,0.76, 0.78,0.6,0.24,0.54,[0.28,0.62],"bias taxonomy or negation-placement analysis changes"),
  ["encoded epistemic bias","high vs low negation distinction","separation of bias from propositional content"],
  ["neutral polar answer space","explanation request"],
  [pro("Separating encoded bias from content prevents the bias from being read as a hard presupposition while still recording the speaker's lean","'Isn't the door closed?' biases toward 'closed' without presupposing it")],
  [con("The boundary between a strong bias and a genuine (loaded) presupposition is gradient and easy to over-read","'Didn't you cheat?' may be biased, or may presuppose cheating, depending on context")],
  ["reading bias as a hard presupposition","ignoring bias and treating a biased question as neutral"],
  ["the question's encoded bias is recorded separately from its neutral propositional content and answer space"],
  ["bias taxonomy changes","bias is conflated with a hard presupposition"],
  specialists=["formal_pragmaticist","erotetic_logician"]))

N.append(node("EMBEDDED_Q","embedded_indirect_question_semantics",
  "Give the semantics of embedded/indirect questions: how an interrogative complement ('knows who came', 'asked whether p') composes with its embedding predicate, including the exhaustivity (weak/strong) the predicate imposes on the embedded answer.",
  "embedding", ["PRESUP_PROJECTION","COMPLETE_ANSWER","TYPE_CONFORMANCE"], ["CQ_17","CQ_18"],
  b(0.82,0.74,0.74,0.78,0.82,0.82,0.6,0.56,0.55, 0.7,0.74, 0.82,0.56,0.26,0.58,[0.32,0.68],"embedding-predicate semantics or exhaustivity selection changes"),
  ["interrogative-complement composition","predicate-selected exhaustivity (weak/strong)","responsive vs rogative predicate handling"],
  ["matrix-clause force","standalone answer ranking"],
  [pro("Compositional embedded-question semantics predicts whether 'knows who' demands strong or weak exhaustivity, fixing the truth conditions of the matrix","'knows who came' (strongly exhaustive) is false if Bob is missed")],
  [con("Predicates vary in the exhaustivity they select and some are ambiguous, so a uniform rule mis-predicts truth conditions","'agree on who' may take a weaker exhaustivity than 'know who'")],
  ["assigning the wrong exhaustivity to the embedding predicate","treating a rogative predicate as responsive (or vice versa)"],
  ["the interrogative complement composes with the predicate at the exhaustivity the predicate selects"],
  ["embedding-predicate semantics changes","a predicate's selected exhaustivity is mis-assigned"],
  specialists=["formal_semanticist","erotetic_logician"]))

# ===================== competency questions (14) =====================
CQ = [
 ("CQ_01","How is a question's logical type (polar, alternative, wh) recognized and what answer space does each fix?",["nodes","glossary"],"QTYPE_POLAR, QTYPE_ALT and QTYPE_WH classify the type and fix its answer space",["QTYPE_POLAR","QTYPE_WH","QTYPE_ALT"]),
 ("CQ_02","How are wh, alternative, and why questions distinguished and what do their answers instantiate (instances, options, or explanantia)?",["nodes"],"QTYPE_WH/QTYPE_ALT instantiate variables/options; QTYPE_WHY requests an explanans over a contrast class",["QTYPE_WH","QTYPE_ALT","QTYPE_WHY"]),
 ("CQ_03","What does a question presuppose and how are existential/uniqueness and explanandum presuppositions extracted?",["nodes"],"PRESUPPOSITION extracts the common entailment of direct answers; QTYPE_WHY fixes the explanandum",["PRESUPPOSITION","QTYPE_WHY"]),
 ("CQ_04","How is a false or loaded presupposition detected and why is such a question flagged rather than answered, and how is encoded bias separated from a hard presupposition?",["nodes","edge_cases"],"FALSE_PRESUP flags a presupposition false-in-context; NEGATIVE_QUESTION separates bias from presupposition",["FALSE_PRESUP","NEGATIVE_QUESTION"]),
 ("CQ_05","What is the answerhood condition: what counts as an answer, and how does the QUD constrain felicitous answers?",["nodes"],"ANSWERHOOD fixes the resolving-proposition criterion; QUD constrains relevance",["ANSWERHOOD","QUD"]),
 ("CQ_06","How is the set of direct answers constructed and what makes each direct answer canonical and minimal?",["nodes"],"DIRECT_ANSWER builds the canonical minimal answers covering each issue cell",["DIRECT_ANSWER"]),
 ("CQ_07","How are complete vs partial and mention-some vs mention-all answers distinguished and the right adequacy bar set?",["nodes","conflict_axes"],"COMPLETE_ANSWER sets the exhaustivity reading and the complete/partial boundary",["COMPLETE_ANSWER"]),
 ("CQ_08","How does partition semantics model a question and tie exhaustive answers to blocks of logical space?",["nodes"],"PARTITION_SEM models the question as an equivalence relation whose blocks are exhaustive answers",["PARTITION_SEM"]),
 ("CQ_09","How is the admissible answer set computed for closed question types and decided by membership?",["nodes"],"ADMISSIBLE_SET enumerates permitted answers for polar/alternative and decides replies by membership",["ADMISSIBLE_SET"]),
 ("CQ_10","How is type-conformance checked for open-type answers and when is a reply category-mismatched?",["nodes"],"TYPE_CONFORMANCE checks the sortal/category match of an open-type answer to the wh-variable or explanans",["TYPE_CONFORMANCE"]),
 ("CQ_11","Why must open-type answers carry a mandatory evidential warrant and when is warrant waived?",["nodes"],"WARRANT_REQ mandates warrant or an explicit defeasibility marker for open-type answers",["WARRANT_REQ"]),
 ("CQ_12","How is entailment between questions defined and how are questions soundly evoked by declaratives?",["nodes","iteration_protocol"],"QENTAILMENT defines refinement-based erotetic implication; EROTETIC_INFERENCE states evocation soundness",["QENTAILMENT","EROTETIC_INFERENCE"]),
 ("CQ_13","How is the question situated in discourse and decomposed into sub-questions whose answers compose?",["nodes"],"QUD situates the question; SUBQUESTION decomposes it so sub-answers compose to a parent answer",["QUD","SUBQUESTION"]),
 ("CQ_14","How do presuppositions project and how do interrogative complements compose under embedding predicates at the selected exhaustivity?",["nodes"],"PRESUP_PROJECTION handles projection/filtering; EMBEDDED_Q composes the complement at the predicate-selected exhaustivity",["PRESUP_PROJECTION","EMBEDDED_Q"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

# consolidate node->CQ refs (each node 1-2 refs; every CQ covered)
CQ_MAP = {
 "QTYPE_POLAR":["CQ_01"], "QTYPE_WH":["CQ_01","CQ_02"], "QTYPE_ALT":["CQ_01","CQ_02"],
 "QTYPE_WHY":["CQ_02","CQ_03"], "PRESUPPOSITION":["CQ_03","CQ_04"], "FALSE_PRESUP":["CQ_04"],
 "ANSWERHOOD":["CQ_05"], "DIRECT_ANSWER":["CQ_06"], "COMPLETE_ANSWER":["CQ_07"],
 "PARTITION_SEM":["CQ_08"], "ADMISSIBLE_SET":["CQ_09"], "TYPE_CONFORMANCE":["CQ_10"],
 "WARRANT_REQ":["CQ_11"], "QENTAILMENT":["CQ_12"], "EROTETIC_INFERENCE":["CQ_12"],
 "QUD":["CQ_05","CQ_13"], "SUBQUESTION":["CQ_13"], "PRESUP_PROJECTION":["CQ_14"],
 "NEGATIVE_QUESTION":["CQ_04"], "EMBEDDED_Q":["CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

# ===================== glossary =====================
GL = [
 ("direct_answer","a canonical minimal proposition that fully and exactly resolves the question (a nuclear answer)",["nuclear_answer","canonical_answer"],["partial_answer"],["DIRECT_ANSWER","ANSWERHOOD"]),
 ("presupposition_of_a_question","a proposition entailed by every direct answer; it must hold for the question to have a true answer",["question_presupposition"],["conversational_implicature"],["PRESUPPOSITION","FALSE_PRESUP"]),
 ("loaded_question","a question carrying a false or contestable presupposition, so no plain answer is true (a complex question)",["complex_question","trick_question"],["neutral_question"],["FALSE_PRESUP","NEGATIVE_QUESTION"]),
 ("strongly_exhaustive_answer","a mention-all answer that settles, for every element, whether it falls under the question",["mention_all_answer"],["mention_some_answer"],["COMPLETE_ANSWER","PARTITION_SEM"]),
 ("mention_some_answer","an answer giving at least one true witness, sufficient for a mention-some question",["existential_answer"],["strongly_exhaustive_answer"],["COMPLETE_ANSWER"]),
 ("question_partition","the equivalence relation on worlds that groups worlds sharing the same complete true answer",["answer_partition"],["alternative_set"],["PARTITION_SEM","QENTAILMENT"]),
 ("erotetic_implication","the relation by which complete answers to one question yield complete answers to another",["question_entailment"],["material_implication"],["QENTAILMENT","EROTETIC_INFERENCE"]),
 ("question_under_discussion","the question currently at issue in a discourse that governs relevance and accommodation",["QUD"],["rhetorical_question"],["QUD","SUBQUESTION"]),
 ("evocation","the relation by which a set of declaratives gives rise to (soundly poses) a question",["question_evocation"],["assertion"],["EROTETIC_INFERENCE"]),
 ("type_conformance","an answer's instantiation of the wh-variable (or explanans) with an object of the correct sort/category",["sortal_match"],["truth_of_answer"],["TYPE_CONFORMANCE","EMBEDDED_Q"]),
]
GLS=[{"term":t,"definition":d,"synonyms":s,"not_same_as":ns,"used_by_nodes":u} for (t,d,s,ns,u) in GL]

# ===================== edges =====================
E=[]
def dep(f,t,rs,cc=0.82,erc=0.28,cp=0.14,why="",ben="",rk="",ex=""):
    E.append({"from":f,"to":t,"edge_type":"dependency","relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{t} depends on {f}","benefit_of_coupling":ben or "ordered prerequisite",
              "risk_of_conflict":rk or "downstream rework if upstream analysis changes","example":ex or f"{f} fixed before {t}"})
def conf(f,t,rs,st,rule,why,cp=0.5,erc=0.5,cc=0.6):
    E.append({"from":f,"to":t,"edge_type":"conflict","relation_strength":rs,"signed_tension":st,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "resolution_rule":rule,"why_related":why,"benefit_of_coupling":"tension surfaced and resolved by rule",
              "risk_of_conflict":"unmanaged tension degrades the analysis","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated analysis",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies)
dep("QTYPE_WH","PRESUPPOSITION",0.86)
dep("QTYPE_POLAR","PRESUPPOSITION",0.8)
dep("QTYPE_ALT","PRESUPPOSITION",0.8)
dep("PRESUPPOSITION","FALSE_PRESUP",0.9)
dep("PRESUPPOSITION","ANSWERHOOD",0.84)
dep("ANSWERHOOD","DIRECT_ANSWER",0.86)
dep("DIRECT_ANSWER","COMPLETE_ANSWER",0.84)
dep("ANSWERHOOD","PARTITION_SEM",0.8)
dep("COMPLETE_ANSWER","PARTITION_SEM",0.82)
dep("DIRECT_ANSWER","ADMISSIBLE_SET",0.84)
dep("QTYPE_POLAR","ADMISSIBLE_SET",0.78)
dep("QTYPE_ALT","ADMISSIBLE_SET",0.78)
dep("DIRECT_ANSWER","TYPE_CONFORMANCE",0.82)
dep("QTYPE_WH","TYPE_CONFORMANCE",0.8)
dep("QTYPE_WHY","TYPE_CONFORMANCE",0.78)
dep("TYPE_CONFORMANCE","WARRANT_REQ",0.84)
dep("PARTITION_SEM","QENTAILMENT",0.84)
dep("QENTAILMENT","EROTETIC_INFERENCE",0.8)
dep("PRESUPPOSITION","EROTETIC_INFERENCE",0.74)
dep("ANSWERHOOD","QUD",0.8)
dep("QENTAILMENT","SUBQUESTION",0.8)
dep("QUD","SUBQUESTION",0.78)
dep("PRESUPPOSITION","PRESUP_PROJECTION",0.82)
dep("QTYPE_POLAR","NEGATIVE_QUESTION",0.8)
dep("PRESUPPOSITION","NEGATIVE_QUESTION",0.76)
dep("PRESUP_PROJECTION","EMBEDDED_Q",0.82)
dep("TYPE_CONFORMANCE","EMBEDDED_Q",0.74)
dep("QTYPE_WHY","PRESUPPOSITION",0.74)

# cross-cutting non-dependency edges (no cycle risk)
rel("FALSE_PRESUP","ANSWERHOOD","constraint",0.78,why="a false presupposition voids answerhood: no direct answer is true, so answering is blocked")
rel("QUD","FALSE_PRESUP","feedback",0.72,why="the QUD/common ground supplies the context against which a presupposition is judged false")
rel("WARRANT_REQ","EROTETIC_INFERENCE","feedback",0.7,why="unwarranted answers re-evoke a follow-up question seeking grounds")
rel("NEGATIVE_QUESTION","ADMISSIBLE_SET","constraint",0.7,why="encoded bias must not narrow the neutral admissible answer set")
rel("SUBQUESTION","QUD","feedback",0.72,why="resolving a sub-question pops the QUD stack and advances the parent question")
rel("PARTITION_SEM","COMPLETE_ANSWER","similarity",0.72,why="a partition block just is a strongly-exhaustive complete answer")
rel("EMBEDDED_Q","PRESUPPOSITION","similarity",0.7,why="embedded questions inherit and re-project the matrix-question presuppositions")

# conflict edges (4): real tensions, negative signed_tension + resolution_rule
conf("COMPLETE_ANSWER","ANSWERHOOD",0.72,-0.6,
  "select the exhaustivity bar from the question's reading and the embedding predicate: demand mention-all only when the form or predicate is strongly exhaustive; otherwise accept a mention-some witness as a full answer",
  "mention-some adequacy conflicts with mention-all (strongly exhaustive) adequacy for the same surface question")
conf("ADMISSIBLE_SET","TYPE_CONFORMANCE",0.72,-0.55,
  "route by question type: closed types (polar/alternative) are decided by strict admissible-set membership; open types (wh/why) are decided by sortal type-conformance plus warrant, never by enumeration",
  "the strict finite admissible-set test for closed types conflicts with the open-ended type-conformance test for open types")
conf("FALSE_PRESUP","DIRECT_ANSWER",0.74,-0.62,
  "if any presupposition is false-in-context, suppress the direct-answer selection and emit a presupposition-denying corrective (flag); only when all presuppositions hold may a direct answer be returned",
  "answering a question (returning a direct answer) conflicts with flagging it when its presupposition is false")
conf("PARTITION_SEM","ADMISSIBLE_SET",0.7,-0.5,
  "use the full partition only when the world space is finite and exhaustivity is required; for tractability and for closed/mention-some cases, operate over the finite admissible set or a coarsening instead of the full partition",
  "partition completeness (a total exhaustive cover of logical space) conflicts with the tractability of a small finite admissible set")

# ===================== conflict_axes (9) =====================
CA=[
 {"name":"mention_some_vs_mention_all_adequacy","description":"Mention-some accepts one true witness while mention-all demands a strongly exhaustive answer; the same surface question can take either reading.","poles":["mention_some","mention_all"],"resolution_hint":"read exhaustivity off the question form and the embedding predicate before setting the bar","tension_score":0.72,"affected_nodes":["COMPLETE_ANSWER","ANSWERHOOD","EMBEDDED_Q"]},
 {"name":"strict_admissible_set_vs_open_type_flexibility","description":"Closed types decide answerhood by finite membership; open types have no finite list and need sortal type-conformance.","poles":["strict_admissible_membership","open_type_conformance"],"resolution_hint":"route closed types to admissible-set membership and open types to type-conformance plus warrant","tension_score":0.7,"affected_nodes":["ADMISSIBLE_SET","TYPE_CONFORMANCE","WARRANT_REQ"]},
 {"name":"answering_vs_flagging_false_presupposition","description":"A loaded question invites a direct answer that concedes a false presupposition; flagging refuses to answer until the presupposition is corrected.","poles":["answer_directly","flag_and_correct"],"resolution_hint":"check presuppositions first; flag and correct if any is false-in-context, otherwise answer","tension_score":0.75,"affected_nodes":["FALSE_PRESUP","DIRECT_ANSWER","PRESUPPOSITION"]},
 {"name":"partition_completeness_vs_tractability","description":"A complete partition of logical space is semantically clean but infeasible over large or infinite world spaces.","poles":["full_partition","finite_admissible_set"],"resolution_hint":"use the partition for finite exhaustive cases; coarsen to a finite admissible set otherwise","tension_score":0.68,"affected_nodes":["PARTITION_SEM","ADMISSIBLE_SET","QENTAILMENT"]},
 {"name":"hard_presupposition_vs_encoded_bias","description":"A negative/biased question may merely lean toward an answer or may actually presuppose it; the boundary is gradient.","poles":["hard_presupposition","mere_bias"],"resolution_hint":"treat as bias unless the proposition is entailed by every direct answer in context","tension_score":0.65,"affected_nodes":["NEGATIVE_QUESTION","PRESUPPOSITION","FALSE_PRESUP"]},
 {"name":"logical_answerhood_vs_evidential_acceptability","description":"A reply can satisfy logical answerhood (right form/type) yet be unacceptable as an unwarranted assertion.","poles":["form_only_answerhood","warranted_assertion"],"resolution_hint":"separate admissibility-in-form from acceptability-as-asserted; require warrant for the latter on open types","tension_score":0.66,"affected_nodes":["WARRANT_REQ","TYPE_CONFORMANCE","ANSWERHOOD"]},
 {"name":"strict_refinement_vs_pragmatic_reduction","description":"Erotetic implication via partition refinement is strict (mention-all) and misses pragmatic mention-some reductions between questions.","poles":["strict_refinement","pragmatic_reduction"],"resolution_hint":"use refinement for guaranteed reductions; allow pragmatic reductions only under a stated exhaustivity reading","tension_score":0.62,"affected_nodes":["QENTAILMENT","EROTETIC_INFERENCE","SUBQUESTION"]},
 {"name":"projection_default_vs_predicate_specific","description":"A uniform presupposition-projection default mis-predicts embeddings that are predicate-specific (factive vs non-factive).","poles":["uniform_projection","predicate_specific_projection"],"resolution_hint":"apply a default but override per embedding predicate's lexical projection profile","tension_score":0.6,"affected_nodes":["PRESUP_PROJECTION","EMBEDDED_Q","PRESUPPOSITION"]},
 {"name":"single_qud_vs_multiple_open_questions","description":"Assuming one operative QUD simplifies relevance judgments but mis-reads discourses with several open questions.","poles":["single_qud","multiple_quds"],"resolution_hint":"identify the QUD stack/tree; judge relevance against the topmost compatible question","tension_score":0.58,"affected_nodes":["QUD","SUBQUESTION","ANSWERHOOD"]},
]

# ===================== edge_cases (12) =====================
EC=[
 {"description":"A loaded question forces a yes/no whose either answer concedes a false presupposition.","trigger":"a presupposition is false-in-context but the question is polar in form","affected_nodes":["FALSE_PRESUP","PRESUPPOSITION","QTYPE_POLAR"],"mitigation":"flag the false presupposition and emit a corrective instead of yes/no","severity":"critical"},
 {"description":"A wh-question with no instances ('which unicorn?') has an unsatisfied existential presupposition, so no direct answer is true.","trigger":"the existential presupposition of a wh-question fails","affected_nodes":["PRESUPPOSITION","DIRECT_ANSWER","ANSWERHOOD"],"mitigation":"detect the failed existential presupposition and report 'no such instance' rather than an empty answer","severity":"high"},
 {"description":"A disjunctive surface ('tea or coffee?') is answered 'yes', revealing a polar/alternative reading mismatch.","trigger":"alternative question intoned or read as polar","affected_nodes":["QTYPE_ALT","QTYPE_POLAR","ADMISSIBLE_SET"],"mitigation":"disambiguate by intonation/context before fixing the admissible set","severity":"medium"},
 {"description":"A mention-some question ('where can I get coffee?') is wrongly held to a mention-all standard, rejecting a correct single witness.","trigger":"strong-exhaustivity bar applied to a mention-some reading","affected_nodes":["COMPLETE_ANSWER","ANSWERHOOD"],"mitigation":"set the exhaustivity bar from the reading; accept one witness for mention-some","severity":"medium"},
 {"description":"An open wh-question's direct-answer set is infinite ('which real number?'), so enumeration is impossible.","trigger":"the wh-variable ranges over an infinite domain","affected_nodes":["DIRECT_ANSWER","TYPE_CONFORMANCE","PARTITION_SEM"],"mitigation":"schematize the answer set and check sortal type-conformance instead of enumerating","severity":"high"},
 {"description":"A type-conforming answer ('Smith did it') is asserted with no grounds, satisfying form but not acceptability.","trigger":"open-type answer lacks any evidential warrant","affected_nodes":["WARRANT_REQ","TYPE_CONFORMANCE"],"mitigation":"require a warrant or an explicit defeasible/ungrounded marker before acceptance","severity":"high"},
 {"description":"A category-mismatched reply ('at noon' to 'who came?') is mistaken for an answer.","trigger":"reply instantiates the wh-variable with the wrong sort","affected_nodes":["TYPE_CONFORMANCE","QTYPE_WH"],"mitigation":"reject sort-mismatched replies unless a coercion/metonymy is licensed","severity":"medium"},
 {"description":"A presupposition of an embedded question is wrongly attributed to the matrix speaker.","trigger":"non-projecting embedded presupposition treated as projecting","affected_nodes":["PRESUP_PROJECTION","EMBEDDED_Q","PRESUPPOSITION"],"mitigation":"apply the embedding predicate's projection profile (e.g. factive 'know' vs 'wonder')","severity":"high"},
 {"description":"A biased question ('isn't the door closed?') is read as presupposing 'closed' rather than merely leaning toward it.","trigger":"encoded bias conflated with a hard presupposition","affected_nodes":["NEGATIVE_QUESTION","PRESUPPOSITION","FALSE_PRESUP"],"mitigation":"record bias separately; treat as presupposition only if entailed by every direct answer","severity":"medium"},
 {"description":"An embedding predicate's selected exhaustivity is mis-assigned, giving wrong truth conditions for 'knows who'.","trigger":"weak exhaustivity applied where the predicate selects strong (or vice versa)","affected_nodes":["EMBEDDED_Q","COMPLETE_ANSWER"],"mitigation":"assign exhaustivity from the predicate's lexical profile (responsive/rogative, weak/strong)","severity":"high"},
 {"description":"A claimed question entailment fails because the two questions carry incompatible presuppositions.","trigger":"refinement asserted without checking presupposition compatibility","affected_nodes":["QENTAILMENT","EROTETIC_INFERENCE","PRESUPPOSITION"],"mitigation":"require presupposition compatibility as a side condition on erotetic implication","severity":"medium"},
 {"description":"Sub-questions are answered but their answers do not recombine into a complete answer to the parent question.","trigger":"a decomposition whose sub-answers do not compose","affected_nodes":["SUBQUESTION","QUD","QENTAILMENT"],"mitigation":"require the sub-question set to jointly entail a parent answer before decomposing","severity":"medium"},
]

# ===================== workflow (12) =====================
WF=[
 {"action":"type_the_question","node_ref":"QTYPE_WH","description":"Classify the question as polar, alternative, wh, or why and fix the corresponding answer space.","artifact":"question_type_record","gate":"a single logical type and its answer space are assigned"},
 {"action":"extract_presuppositions","node_ref":"PRESUPPOSITION","description":"Extract the presupposition set as the common entailment of the direct answers, including existential/uniqueness presuppositions.","artifact":"presupposition_set","gate":"every direct answer entails the listed presuppositions"},
 {"action":"check_false_presupposition","node_ref":"FALSE_PRESUP","description":"Test each presupposition against the common ground; if any is false-in-context, flag and stop rather than answer.","artifact":"presupposition_verdict","gate":"no presupposition is false-in-context, or the question is flagged"},
 {"action":"define_answerhood","node_ref":"ANSWERHOOD","description":"Fix the answerhood relation: which propositions resolve the question's issue.","artifact":"answerhood_spec","gate":"the issue-resolving criterion is stated"},
 {"action":"construct_direct_answers","node_ref":"DIRECT_ANSWER","description":"Build the canonical minimal direct-answer set covering each cell of the issue.","artifact":"direct_answer_set","gate":"the direct answers cover the issue and are minimal"},
 {"action":"set_exhaustivity_reading","node_ref":"COMPLETE_ANSWER","description":"Determine the mention-some vs mention-all reading and the complete/partial boundary.","artifact":"exhaustivity_reading","gate":"the exhaustivity bar matches the question reading"},
 {"action":"build_partition_or_admissible_set","node_ref":"PARTITION_SEM","description":"Model the question as a partition for finite exhaustive cases, or coarsen to a finite admissible set for closed/tractable cases.","artifact":"semantic_model","gate":"a partition or finite admissible set is well-defined"},
 {"action":"decide_closed_type_answers","node_ref":"ADMISSIBLE_SET","description":"For polar/alternative questions, decide answers by membership in the finite admissible set.","artifact":"admissible_answer_set","gate":"each reply is decided by admissible-set membership"},
 {"action":"check_open_type_conformance","node_ref":"TYPE_CONFORMANCE","description":"For wh/why questions, check sortal/category conformance of the reply to the wh-variable or explanans.","artifact":"type_conformance_report","gate":"accepted answers are sort/category conforming"},
 {"action":"require_warrant","node_ref":"WARRANT_REQ","description":"Require an evidential warrant (or explicit defeasibility marker) for each open-type answer.","artifact":"warrant_record","gate":"every open answer carries warrant or a defeasibility marker"},
 {"action":"resolve_embedding_and_projection","node_ref":"EMBEDDED_Q","description":"For embedded questions, project/filter presuppositions and compose the complement at the predicate-selected exhaustivity.","artifact":"embedding_analysis","gate":"projection and exhaustivity follow the embedding predicate"},
 {"action":"situate_in_discourse","node_ref":"QUD","description":"Place the question as the QUD and, if needed, decompose it into sub-questions whose answers compose.","artifact":"qud_strategy","gate":"the QUD is identified and any sub-questions compose to a parent answer"},
]

# ===================== dominance_rules (9) =====================
DR=[
 {"rule":"FALSE_PRESUP must run and clear before DIRECT_ANSWER may return any answer","rationale":"a question with a false presupposition has no true direct answer; answering concedes the false proposition","trigger":"answer selection begins with an unchecked or failed presupposition","action":"block answering; flag the false presupposition and emit a corrective"},
 {"rule":"The question's logical type (QTYPE_*) must be fixed before answerhood and admissibility are computed","rationale":"the answer space and admissibility test are determined by the question type","trigger":"answerhood computed before the type is assigned","action":"assign the question type first"},
 {"rule":"Closed types are decided by ADMISSIBLE_SET membership; open types by TYPE_CONFORMANCE, never by enumeration","rationale":"open types have no finite admissible list, only a sortal constraint","trigger":"an open wh-question is forced into a finite admissible enumeration","action":"route open types to type-conformance plus warrant"},
 {"rule":"The exhaustivity reading (mention-some vs mention-all) must be fixed before completeness is judged","rationale":"the completeness bar depends on the reading and the embedding predicate","trigger":"a completeness judgment without a fixed exhaustivity reading","action":"set the reading from form/predicate before judging completeness"},
 {"rule":"Open-type answers require a warrant before acceptance-as-asserted","rationale":"a type-conforming answer can still be a groundless assertion","trigger":"an open answer accepted with no warrant or defeasibility marker","action":"require warrant or mark the answer defeasible"},
 {"rule":"Erotetic implication (QENTAILMENT) requires presupposition compatibility as a side condition","rationale":"refinement alone can relate questions whose presuppositions clash","trigger":"implication asserted between presupposition-incompatible questions","action":"add presupposition compatibility to the implication check"},
 {"rule":"Embedded-question presuppositions project per the embedding predicate's profile, not by a blanket rule","rationale":"factive and non-factive predicates project differently","trigger":"a uniform projection default applied across all predicates","action":"override the default with the predicate's projection profile"},
 {"rule":"Use the full PARTITION_SEM only for finite exhaustive cases; otherwise coarsen to a finite admissible set","rationale":"a complete partition is intractable over infinite world spaces","trigger":"a partition attempted over an infinite or very large world space","action":"coarsen to a finite admissible set or schema"},
 {"rule":"Encoded bias (NEGATIVE_QUESTION) must not narrow the neutral admissible answer space","rationale":"bias records the speaker's lean, not a restriction on what answers are admissible","trigger":"a biased question's admissible set narrowed to the favored answer","action":"keep the neutral answer space; record bias separately"},
]

# ===================== anti_rework_rules (9) =====================
ARR=[
 {"rule":"Do not select a direct answer before clearing presuppositions; a later false-presupposition finding invalidates the answer","prevents":"retracting an answer that conceded a false presupposition"},
 {"rule":"Do not compute answerhood before fixing the question type; a re-typing forces recomputation of the answer space","prevents":"recomputing the entire answer space after a late re-typing"},
 {"rule":"Do not enumerate an admissible set for an open wh-question; the unbounded list must be rebuilt as a schema","prevents":"discarding an infinite enumeration and re-deriving a sortal schema"},
 {"rule":"Do not judge completeness before fixing the mention-some/mention-all reading; the wrong bar must be redone","prevents":"re-grading every answer after the exhaustivity reading flips"},
 {"rule":"Do not accept an open-type answer before warrant; an unwarranted answer admitted now is re-litigated later","prevents":"re-opening accepted answers to attach missing warrants"},
 {"rule":"Do not assert question entailment without checking presupposition compatibility; clashes surface downstream","prevents":"unwinding a reduction built on incompatible presuppositions"},
 {"rule":"Do not apply a blanket presupposition-projection rule across embedding predicates; predicate-specific cases must be redone","prevents":"reanalyzing every embedded question after a projection misprediction"},
 {"rule":"Do not build a full partition over an unbounded world space; an intractable model must be coarsened and rebuilt","prevents":"rebuilding the semantic model after a partition proves infeasible"},
 {"rule":"Do not conflate encoded bias with a hard presupposition; misclassifying it forces re-judging answerhood","prevents":"re-deriving the answer space after a bias was mistaken for a presupposition"},
]

# ===================== iteration_protocol (8) =====================
IP=[
 {"trigger":"a question with a false presupposition was answered yes/no instead of flagged","action":"strengthen presupposition checking in PRESUPPOSITION and the flag/correct path in FALSE_PRESUP before answer selection","nodes":["FALSE_PRESUP","PRESUPPOSITION","DIRECT_ANSWER"],"priority":"critical"},
 {"trigger":"a mention-some question was rejected for failing a mention-all bar","action":"re-derive the exhaustivity reading in COMPLETE_ANSWER and re-set the bar in ANSWERHOOD","nodes":["COMPLETE_ANSWER","ANSWERHOOD"],"priority":"high"},
 {"trigger":"an open wh-question was forced into a finite admissible enumeration","action":"route it through TYPE_CONFORMANCE and schematize the answer set instead of enumerating","nodes":["TYPE_CONFORMANCE","ADMISSIBLE_SET","DIRECT_ANSWER"],"priority":"high"},
 {"trigger":"a type-conforming answer was accepted with no grounds","action":"enforce the warrant requirement in WARRANT_REQ before acceptance-as-asserted","nodes":["WARRANT_REQ","TYPE_CONFORMANCE"],"priority":"high"},
 {"trigger":"an embedded question's presupposition was attributed to the matrix speaker","action":"apply the predicate-specific projection profile in PRESUP_PROJECTION and re-compose in EMBEDDED_Q","nodes":["PRESUP_PROJECTION","EMBEDDED_Q"],"priority":"medium"},
 {"trigger":"a claimed question entailment broke on incompatible presuppositions","action":"add the presupposition-compatibility side condition to QENTAILMENT and re-check EROTETIC_INFERENCE","nodes":["QENTAILMENT","EROTETIC_INFERENCE"],"priority":"medium"},
 {"trigger":"a partition was attempted over an infinite world space","action":"coarsen PARTITION_SEM to a finite admissible set in ADMISSIBLE_SET","nodes":["PARTITION_SEM","ADMISSIBLE_SET"],"priority":"medium"},
 {"trigger":"sub-question answers did not compose into a parent answer","action":"require joint entailment of a parent answer in SUBQUESTION before decomposing under the QUD","nodes":["SUBQUESTION","QUD"],"priority":"medium"},
]

spec = {
 "domain":"erot__question_semantics",
 "domain_label":"Erotetic Logic & Question Semantics (Question Science subdomain)",
 "purpose":"session_bounded_method_for_analyzing_a_questions_logical_type_presuppositions_and_answerhood_conditions_computing_the_admissible_answer_set_for_closed_question_types_and_the_type_conformance_and_mandatory_evidential_warrant_requirements_for_open_question_types_and_detecting_false_or_loaded_presuppositions",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "analysis is purely logical and linguistic, with neutral informational framing; no domain-specific answer content is asserted",
   "a context / common ground is available against which presuppositions are evaluated",
   "questions are analyzed at the level of logical form, abstracting from speaker intention beyond encoded bias",
 ],
 "exclusions":[
   "deciding the actual true answer to any object-level question (only answerhood conditions are analyzed)",
   "speech-act force and politeness beyond encoded bias",
   "rhetorical questions whose function is assertion rather than inquiry",
   "psycholinguistic question processing and acquisition",
 ],
 "source_description":"heuristic prior estimates for erotetic-logic and question-semantics work units, informed by the logic of questions and answers, partition semantics, inferential erotetic logic, and Question-Under-Discussion pragmatics; no supplied dataset",
 "source_citation":"Belnap & Steel 1976 The Logic of Questions and Answers; Hamblin 1958 Questions; Groenendijk & Stokhof 1984 Studies on the Semantics of Questions and the Pragmatics of Answers (partition semantics); Wisniewski 1995 The Posing of Questions and 2013 Questions, Inferences, and Scenarios (inferential erotetic logic); Roberts 1996 Information Structure in Discourse (Question Under Discussion); van Fraassen 1980 The Scientific Image (why-questions and contrast classes)",
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
 "priority_rationale":"question typing (QTYPE_*) and presupposition extraction are foundational; FALSE_PRESUP gates answering; answerhood, direct answers and completeness fix the answer space; admissible-set vs type-conformance/warrant split closed from open types; partition semantics, erotetic implication, QUD, subordination and embedding close the analysis.",
 "eval_objective":"verify_question_typing_presupposition_extraction_false_presupposition_flagging_answerhood_admissibility_and_open_type_warrant_of_erot__question_semantics_kb",
}

out_dir = "branches/b10_question_compiler/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "erot__question_semantics.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
