#!/usr/bin/env python3
"""Generate the leakage__holdout_design content spec (B60 KB) for kb_forge.py.

Domain: construct a leakage-controlled, strictly-post-cutoff, pre-registered, BLIND
holdout for evaluating a predictive/forecasting capability, where the system under test
is (or includes) a large language model AND a human designer. The hard scope finding
this KB encodes (scope FP3/KU4): "the model/designer did not see this case during
DESIGN" is NECESSARY but NOT SUFFICIENT to call an evaluation leakage-free. Two
distinct contamination channels must both be controlled:
  (1) PRETRAINING CONTAMINATION -- an LLM very likely ingested famous historical
      outcomes during pretraining, so any pre-cutoff case is suspect regardless of how
      the holdout was assembled (Sainz et al. 2023; Magar & Schwartz 2022);
  (2) DESIGNER-KNOWLEDGE LEAKAGE -- the human who selects cases, writes features, or
      sets thresholds already knows the outcomes; the human is part of the predicting
      system, so their hindsight leaks through case selection and feature engineering
      (Kaufman et al. 2012 "Leakage in Data Mining"; the analyst is in the loop).
The design controls BOTH: strictly post-cutoff events (Bergmeir & Benitez 2012 temporal
validation; Sainz cutoff hygiene), pre-registration of cases + metrics before any
result is seen (Nosek et al. 2018), blind selection by an uninvolved party, explicit
label-leakage screening (Kaufman et al. 2012), contamination probes that ask the model
to recite the held-out outcome, a residual-leakage estimate, one-time holdout hygiene,
and a standing leakage audit.

Compact authoring (mimics example_htn_gen.py / the B60 siblings): node()/b()/pro()/con()
/dep()/conf()/rel() helpers carry sane defaults so each call supplies only domain
content + base metric magnitudes. Grounded in real, named literature cited per node:
- Kaufman, Rosset, Perlich & Stitelman 2012 "Leakage in Data Mining: Formulation,
  Detection, and Avoidance" (ACM TKDD) -- legitimacy of features, target leakage,
  train/test contamination, the analyst-as-leak;
- Nosek, Ebersole, DeHaven & Mellor 2018 "The preregistration revolution" (PNAS) --
  freezing hypotheses, samples, and analysis before observing outcomes;
- Sainz, Campos, Garcia-Ferrero, Etxaniz, de Lacalle & Agirre 2023 "NLP Evaluation in
  Trouble: On the Need to Measure LLM Data Contamination for each Benchmark" (EMNLP
  Findings) -- benchmark/cutoff contamination, recitation probes;
- Magar & Schwartz 2022 "Data Contamination: From Memorization to Exploitation" (ACL) --
  distinguishing memorization from genuine exploitation of contaminated data;
- Bergmeir & Benitez 2012 "On the use of cross-validation for time series predictor
  evaluation" (Information Sciences) -- temporal blocking, why random CV leaks in
  time-ordered data."""
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
        "academic_fields": ["experimental_methodology", "machine_learning_evaluation"],
        "subfields": subfields or ["data_leakage_control", "holdout_design"],
        "specialists": specialists or ["evaluation_methodologist", "leakage_auditor"],
        "contradictors": contradictors or ["did_not_see_it_is_enough_advocate"],
        "inputs": inputs or ["capability under test", "candidate event pool"],
        "outputs": outputs or ["leakage-controlled holdout artifact"],
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


# ---------------------------------------------------------------------------
# NODES
# Layering (dependency DAG, prerequisites listed left-to-right):
#  L0 foundations:  THREAT_MODEL, CUTOFF_DETERMINATION
#  L1 channels:     PRETRAINING_CONTAMINATION, DESIGNER_KNOWLEDGE_LEAKAGE
#  L2 temporal:     POST_CUTOFF_HOLDOUT, TEMPORAL_SPLIT
#  L3 governance:   PRE_REGISTRATION, BLIND_SELECTION
#  L4 features:     LABEL_LEAKAGE_SCREEN, FEATURE_PROVENANCE
#  L5 probes:       CONTAMINATION_PROBES, RECITATION_BASELINE
#  L6 quant:        RESIDUAL_LEAKAGE_ESTIMATE, POWER_AND_SAMPLE
#  L7 hygiene:      HOLDOUT_HYGIENE, SEALED_DELIVERY
#  L8 audit:        LEAKAGE_AUDIT, PREREG_DEVIATION_LOG
#  L9 reporting:    NECESSARY_NOT_SUFFICIENT, LEAKAGE_DISCLOSURE
# ---------------------------------------------------------------------------
N = []

N.append(node("THREAT_MODEL", "leakage_threat_model",
  "Enumerate every channel by which information about a held-out outcome can reach the "
  "system under test, treating the system as the LLM PLUS the human designer; classify "
  "each channel (pretraining ingestion, designer hindsight, label leakage, leaderboard "
  "overfitting) and assign each an owning control node so no channel is left uncovered.",
  "foundations", [], ["CQ_01"],
  b(0.92, 0.82, 0.8, 0.6, 0.88, 0.82, 0.62, 0.66, 0.5, 0.78, 0.82, 0.9, 0.66, 0.16, 0.55, [0.2, 0.5],
    "a new contamination channel is discovered or the system-under-test boundary changes"),
  ["channel enumeration", "system-under-test boundary (model+human)", "channel-to-control mapping"],
  ["statistical power analysis", "operator-level data plumbing"],
  [pro("Naming the human designer as part of the predicting system makes designer-knowledge leakage a first-class threat rather than an afterthought",
       "a threat-model row for 'analyst selected cases knowing which firms later failed' forces a blind-selection control")],
  [con("An over-broad threat model can paralyze design by treating every coincidence as contamination",
       "flagging all pre-2024 world knowledge as fatal even for genuinely novel composite events")],
  ["a channel exists with no owning control", "the model+human boundary is drawn to exclude the analyst, hiding designer leakage"],
  ["every enumerated channel maps to at least one control node and a residual-risk note"],
  ["a leakage incident traces to an unlisted channel", "the system-under-test definition changes"],
  specialists=["evaluation_methodologist", "threat_modeler"]))

N.append(node("CUTOFF_DETERMINATION", "training_cutoff_determination",
  "Establish the model's verifiable training-data cutoff date (and the human designer's "
  "knowledge cutoff) from documentation and empirical probing, because every downstream "
  "'strictly post-cutoff' guarantee is anchored to this date; treat vendor-stated cutoffs "
  "as upper bounds to be confirmed, not as ground truth.",
  "foundations", [], ["CQ_02"],
  b(0.9, 0.78, 0.74, 0.62, 0.86, 0.78, 0.66, 0.62, 0.48, 0.74, 0.78, 0.88, 0.62, 0.2, 0.55, [0.24, 0.58],
    "the model is updated/retrained or new evidence revises the effective cutoff"),
  ["vendor cutoff documentation review", "empirical cutoff probing", "designer knowledge-cutoff statement"],
  ["case selection", "metric pre-registration"],
  [pro("Empirically confirming the cutoff (e.g. asking the model about events near the claimed date) guards against silently extended or mid-training data",
       "the model answers a question about an event two weeks after its claimed cutoff, revealing the true cutoff is later")],
  [con("Cutoffs can be fuzzy: continued pretraining, RAG, or tool use can inject post-cutoff knowledge the date does not capture",
       "a model with web-browsing has no meaningful static cutoff for retrievable facts")],
  ["vendor cutoff trusted without probing", "the cutoff used is the model release date rather than the data cutoff"],
  ["the effective cutoff is documented and empirically corroborated against near-boundary events"],
  ["the model is retrained or gains retrieval", "near-boundary probing reveals a later effective cutoff"],
  specialists=["evaluation_methodologist", "llm_red_teamer"]))

N.append(node("PRETRAINING_CONTAMINATION", "pretraining_contamination_control",
  "Control the channel where the LLM ingested the held-out outcome during pretraining: "
  "assume any famous, well-documented historical event is memorized, and require that "
  "holdout cases be novel composite events or strictly post-cutoff so the answer cannot "
  "have been seen in training (per benchmark-contamination findings).",
  "channels", ["THREAT_MODEL", "CUTOFF_DETERMINATION"], ["CQ_01", "CQ_03"],
  b(0.92, 0.82, 0.8, 0.66, 0.9, 0.84, 0.66, 0.6, 0.58, 0.72, 0.76, 0.9, 0.6, 0.22, 0.6, [0.26, 0.62],
    "evidence emerges that the model can recite or exploit a supposedly unseen outcome"),
  ["memorized-event assumption", "novel-composite-event requirement", "famous-outcome exclusion list"],
  ["human designer hindsight (separate channel)", "feature-level label leakage"],
  [pro("Treating any pre-cutoff famous outcome as memorized-by-default is the conservative assumption that benchmark-contamination studies show is warranted",
       "Sainz et al. demonstrate models reproduce held test instances verbatim, so a famous 2008 default cannot test 'prediction'")],
  [con("Memorization is not always exploitation: a model may have seen text yet not use it to answer, so blanket exclusion can discard usable cases",
       "Magar & Schwartz show contaminated data is sometimes memorized but not exploited, inflating false exclusions")],
  ["a pre-cutoff outcome used as a test item the model has memorized", "'novelty' assumed from obscurity rather than from post-cutoff timing"],
  ["no held-out case has a published outcome at or before the confirmed training cutoff",
   "candidate famous events are excluded or shown unrecitable by a probe"],
  ["a probe shows the model reciting a held-out outcome", "a new contamination study revises the memorization assumption"],
  specialists=["llm_red_teamer", "evaluation_methodologist"]))

N.append(node("DESIGNER_KNOWLEDGE_LEAKAGE", "designer_knowledge_leakage_control",
  "Control the channel where the human designer already knows the outcomes and leaks that "
  "hindsight through case selection, feature choice, or threshold setting; because the "
  "human is part of the predicting system, 'the model did not see it' is insufficient if "
  "the designer steered the holdout toward outcomes they knew.",
  "channels", ["THREAT_MODEL"], ["CQ_01", "CQ_04"],
  b(0.92, 0.82, 0.82, 0.66, 0.9, 0.85, 0.68, 0.58, 0.6, 0.7, 0.74, 0.9, 0.58, 0.24, 0.62, [0.28, 0.66],
    "evidence that case selection or feature choices correlate with known outcomes"),
  ["hindsight-in-selection control", "hindsight-in-feature-engineering control", "designer-blindness requirement"],
  ["model pretraining ingestion (separate channel)", "compute/power budgeting"],
  [pro("Kaufman et al. identify the analyst as a leakage source: a designer who knows outcomes leaks them through which cases and features they pick",
       "an analyst who knows which 2025 startups failed quietly over-samples failures into the 'balanced' holdout")],
  [con("Full designer blindness is operationally hard: the designer needs domain knowledge to build the task at all",
       "the same expertise that makes a good forecasting task also carries outcome hindsight")],
  ["the designer selects cases knowing outcomes", "features are engineered with hindsight (a proxy for the answer)"],
  ["case selection and feature definitions are frozen and either outcome-blind or selected by an uninvolved party",
   "no chosen feature is shown to encode the outcome"],
  ["selected cases correlate with known outcomes", "a post-hoc audit finds a hindsight-laden feature"],
  specialists=["evaluation_methodologist", "leakage_auditor"]))

N.append(node("POST_CUTOFF_HOLDOUT", "strictly_post_cutoff_event_pool",
  "Assemble the candidate holdout exclusively from events whose outcome was not determined "
  "and not publicly knowable until strictly after the confirmed training/knowledge cutoff, "
  "so the model could not have ingested the answer and the designer could not have known it "
  "when the task was frozen.",
  "temporal", ["PRETRAINING_CONTAMINATION", "CUTOFF_DETERMINATION"], ["CQ_03", "CQ_05"],
  b(0.9, 0.8, 0.8, 0.62, 0.88, 0.82, 0.7, 0.6, 0.55, 0.74, 0.78, 0.9, 0.6, 0.2, 0.58, [0.24, 0.58],
    "the resolution date of an event is found to predate the cutoff"),
  ["post-cutoff event sourcing", "outcome-resolution-date verification", "leak-by-leading-indicator screening"],
  ["pre-cutoff historical cases", "designer arbitration of ties"],
  [pro("Strict post-cutoff timing makes the unseen-by-training guarantee a checkable date comparison rather than an assumption of obscurity",
       "only events whose outcome resolved after the cutoff (e.g. an election, an earnings report) enter the pool")],
  [con("Post-cutoff events are scarcer and may be unrepresentative of the deployment distribution",
       "restricting to recent months yields too few cases or a skewed event mix")],
  ["an event's outcome was knowable before the cutoff via leading indicators", "resolution date confused with announcement date"],
  ["every pooled event has an outcome-resolution date strictly after the confirmed cutoff",
   "no event's outcome was forecastable from pre-cutoff public information"],
  ["a pooled event's true resolution date predates the cutoff", "leading-indicator leakage is found for an event class"],
  specialists=["evaluation_methodologist", "domain_forecaster"]))

N.append(node("TEMPORAL_SPLIT", "temporal_train_test_blocking",
  "When the evaluation reuses any in-context or fine-tuning data, split strictly by time "
  "(a temporal block) rather than at random, and forbid look-ahead, because random "
  "cross-validation on time-ordered data leaks future information into the training side.",
  "temporal", ["CUTOFF_DETERMINATION"], ["CQ_05", "CQ_06"],
  b(0.84, 0.74, 0.72, 0.6, 0.82, 0.78, 0.6, 0.64, 0.5, 0.76, 0.78, 0.84, 0.64, 0.2, 0.5, [0.22, 0.54],
    "the evaluation adds an in-context or fine-tuning component that mixes time periods"),
  ["time-ordered block split", "look-ahead prohibition", "embargo gap between train and test"],
  ["random k-fold splitting", "pretraining cutoff (handled upstream)"],
  [pro("Bergmeir & Benitez show random CV on time series leaks future into the past; temporal blocking with an embargo removes that leak",
       "a forecasting eval that fine-tunes on later months and tests on earlier ones leaks the future unless blocked by time")],
  [con("Temporal blocking reduces effective sample size and can entangle a regime change with the split",
       "a single split point straddles a market crash, confounding period with regime")],
  ["random CV used on time-ordered data", "no embargo, so test-adjacent leakage bleeds across the split"],
  ["any train/test partition is strictly time-ordered with an embargo gap and no look-ahead feature"],
  ["a regime change coincides with the split point", "an in-context example postdates a test item"],
  specialists=["evaluation_methodologist", "time_series_methodologist"]))

N.append(node("PRE_REGISTRATION", "case_and_metric_preregistration",
  "Freeze the exact case set, the prediction question wording, the scoring metric, and the "
  "analysis plan in a timestamped, tamper-evident pre-registration BEFORE any model output "
  "or outcome is observed, so the design cannot be silently tuned to the answer after the "
  "fact.",
  "governance", ["POST_CUTOFF_HOLDOUT", "DESIGNER_KNOWLEDGE_LEAKAGE"], ["CQ_07", "CQ_08"],
  b(0.9, 0.82, 0.8, 0.58, 0.88, 0.82, 0.78, 0.62, 0.55, 0.78, 0.8, 0.9, 0.62, 0.18, 0.6, [0.2, 0.5],
    "the case set, question wording, or scoring rule needs to change after registration"),
  ["frozen case list", "frozen metric and analysis plan", "timestamped tamper-evident registration"],
  ["exploratory analysis (must be labeled non-confirmatory)", "model internals"],
  [pro("Nosek et al.'s preregistration converts an after-the-fact rationalization into a before-the-fact commitment, removing researcher degrees of freedom",
       "the scoring threshold is registered before any prediction, so it cannot be moved to flatter the model")],
  [con("Pre-registration is rigid: a genuine design flaw discovered post-registration forces a logged deviation or a fresh holdout",
       "a discovered ambiguity in the prediction question cannot be silently reworded after freeze")],
  ["metrics chosen after seeing results", "case set quietly edited post-registration without a deviation log"],
  ["the case set, question, metric, and analysis plan are timestamped and hashed before any output is seen"],
  ["a needed change is discovered after registration", "the registry timestamp cannot be independently verified"],
  specialists=["evaluation_methodologist", "open_science_steward"]))

N.append(node("BLIND_SELECTION", "blind_case_selection_by_uninvolved_party",
  "Have the specific holdout cases chosen by a party with no stake in the result and no "
  "access to model outputs, ideally outcome-blind, so the selection itself cannot encode "
  "the designer's or evaluator's hindsight about which cases the model will get right.",
  "governance", ["DESIGNER_KNOWLEDGE_LEAKAGE", "PRE_REGISTRATION"], ["CQ_04", "CQ_08"],
  b(0.86, 0.78, 0.78, 0.6, 0.86, 0.82, 0.66, 0.6, 0.55, 0.74, 0.78, 0.86, 0.6, 0.22, 0.58, [0.24, 0.58],
    "the selecting party gains outcome knowledge or a stake in the result"),
  ["uninvolved-selector protocol", "outcome-blind selection rule", "selector-evaluator separation"],
  ["designer-driven hand-picking", "scoring (separate stage)"],
  [pro("An uninvolved selector breaks the link between the designer's outcome knowledge and which cases appear, closing the selection leg of designer leakage",
       "a librarian draws cases by a pre-registered rule from the post-cutoff pool, with no view of outcomes")],
  [con("A naive blind selector may pick degenerate or trivially-answerable cases, trading leakage control for low task quality",
       "random blind sampling yields ten near-identical events that test one narrow skill")],
  ["the designer hand-picks cases under the guise of blindness", "the selector can see model outputs or outcomes"],
  ["cases are drawn by an uninvolved, outcome-blind party following a pre-registered selection rule"],
  ["the selector is found to share interests with the evaluator", "selected cases are trivially answerable"],
  specialists=["evaluation_methodologist", "independent_curator"]))

N.append(node("LABEL_LEAKAGE_SCREEN", "feature_label_leakage_screening",
  "Screen every feature, prompt field, and context document for label leakage: information "
  "that is a proxy for, or only available because of, the outcome (post-outcome timestamps, "
  "outcome-conditioned text, identifiers that encode the result), and remove or quarantine "
  "such legitimacy violations before the case is finalized.",
  "features", ["BLIND_SELECTION"], ["CQ_09", "CQ_10"],
  b(0.9, 0.8, 0.78, 0.66, 0.9, 0.82, 0.66, 0.6, 0.58, 0.72, 0.76, 0.9, 0.6, 0.22, 0.62, [0.26, 0.62],
    "a new feature, context document, or data source is added to a case"),
  ["legitimacy check per feature", "post-outcome-information removal", "outcome-proxy detection"],
  ["temporal split (handled upstream)", "scoring metric choice"],
  [pro("Kaufman et al. formalize legitimacy: a feature is leaking if it would be unavailable at genuine prediction time; screening enforces that boundary",
       "a news snippet dated after the event that already names the outcome is quarantined from the prompt")],
  [con("Subtle leakage is hard to catch: an innocuous-looking field can be a near-deterministic outcome proxy",
       "a record ID assigned only to firms that later filed for bankruptcy silently encodes the label")],
  ["a post-outcome document leaks into the context", "an identifier or timestamp encodes the result"],
  ["every feature is available strictly at prediction time and none is shown to proxy the outcome"],
  ["a new data source is added to a case", "an audit finds a high-leverage feature correlates near-perfectly with the label"],
  specialists=["leakage_auditor", "data_engineer"]))

N.append(node("FEATURE_PROVENANCE", "feature_provenance_and_as_of_dating",
  "Attach an as-of timestamp and source provenance to every feature value so each can be "
  "proven to have existed and been knowable strictly before the event's resolution, turning "
  "leakage screening from a judgment call into a checkable as-of date comparison.",
  "features", ["LABEL_LEAKAGE_SCREEN"], ["CQ_10"],
  b(0.82, 0.74, 0.72, 0.62, 0.82, 0.78, 0.6, 0.64, 0.5, 0.78, 0.8, 0.82, 0.64, 0.2, 0.5, [0.22, 0.52],
    "a feature's provenance or as-of date is missing or disputed"),
  ["as-of timestamp per feature", "source provenance record", "point-in-time reconstruction"],
  ["outcome scoring", "model prompting mechanics"],
  [pro("Point-in-time as-of dating makes 'available at prediction time' an auditable fact, the operational core of the legitimacy criterion",
       "each feature carries the date it was first publicly available, all strictly before the resolution date")],
  [con("Point-in-time reconstruction is laborious and many real datasets silently back-fill revised values",
       "an economic series is revised after the fact, so its stored value is not what was knowable then")],
  ["a back-filled (revised) value is used as if it were point-in-time", "a feature lacks any as-of date"],
  ["every feature value carries an as-of date strictly before its event's resolution date"],
  ["a feature is found to be a revised/back-filled value", "provenance for a feature cannot be reconstructed"],
  specialists=["data_engineer", "leakage_auditor"]))

N.append(node("CONTAMINATION_PROBES", "model_recitation_contamination_probes",
  "Actively probe whether the model can reproduce the held-out outcome or recognize the test "
  "item from pretraining: ask it to complete, recite, or date the event before the evaluation, "
  "and treat any successful recitation as evidence the case is contaminated and must be "
  "dropped or quarantined.",
  "probes", ["POST_CUTOFF_HOLDOUT", "PRETRAINING_CONTAMINATION"], ["CQ_03", "CQ_11"],
  b(0.88, 0.78, 0.78, 0.7, 0.88, 0.8, 0.6, 0.58, 0.6, 0.7, 0.74, 0.88, 0.58, 0.26, 0.6, [0.3, 0.66],
    "the probe protocol or the model version changes"),
  ["recitation/completion probing", "membership-inference style probing", "quarantine-on-recitation rule"],
  ["human designer hindsight (separate channel)", "metric computation"],
  [pro("Sainz et al. recommend exactly this: measure contamination per benchmark by checking whether the model can reproduce held instances",
       "prompting the model with the case stem and finding it completes the true outcome flags contamination directly")],
  [con("Probes have false negatives: a model may exploit contaminated knowledge without verbatim recitation, so a clean probe is not a clean bill",
       "Magar & Schwartz show exploitation can occur without reproducible memorization, so probes under-detect")],
  ["a recitable case kept because the probe was too narrow", "probe run on a different model version than the one evaluated"],
  ["every retained case passes a recitation probe (model cannot reproduce the outcome) on the exact evaluated model version"],
  ["the model version under test changes", "an exploited-but-not-recited contamination case is discovered"],
  specialists=["llm_red_teamer", "evaluation_methodologist"]))

N.append(node("RECITATION_BASELINE", "non_predictive_recall_baseline",
  "Establish a baseline that measures pure recall versus genuine prediction: give the model "
  "the case with and without the discriminating pre-cutoff context, so that score gains "
  "attributable to memorized knowledge can be separated from gains attributable to actual "
  "forecasting skill.",
  "probes", ["CONTAMINATION_PROBES"], ["CQ_11", "CQ_12"],
  b(0.8, 0.72, 0.72, 0.7, 0.82, 0.78, 0.58, 0.6, 0.55, 0.72, 0.74, 0.8, 0.6, 0.24, 0.52, [0.28, 0.62],
    "the baseline conditions or the discriminating-context definition change"),
  ["recall-only condition", "prediction-with-context condition", "memorization-vs-skill decomposition"],
  ["sealed delivery mechanics", "audit cadence"],
  [pro("A recall baseline operationalizes Magar & Schwartz's memorization-vs-exploitation split, giving a quantitative handle on how much score is leakage",
       "if the no-context score already matches the with-context score, the 'skill' is memorized recall")],
  [con("Designing a truly information-free recall condition is hard; residual cues can leak the answer into the baseline too",
       "stripping context still leaves a distinctive entity name that the model has memorized")],
  ["baseline still leaks the answer through residual cues", "recall and prediction conditions differ on more than the target information"],
  ["the recall-only baseline isolates memorized recall from forecasting skill within a stated margin"],
  ["the discriminating-context definition changes", "the recall baseline scores implausibly high"],
  specialists=["evaluation_methodologist", "experiment_designer"]))

N.append(node("RESIDUAL_LEAKAGE_ESTIMATE", "residual_leakage_quantification",
  "Quantify the leakage that remains after all controls: combine probe hit rates, the recall "
  "baseline, and feature-screen residuals into an explicit residual-leakage estimate with an "
  "uncertainty band, and attach it to every reported result so consumers can discount the "
  "score accordingly.",
  "quantification", ["RECITATION_BASELINE", "FEATURE_PROVENANCE", "TEMPORAL_SPLIT"], ["CQ_12", "CQ_13"],
  b(0.86, 0.78, 0.78, 0.72, 0.86, 0.84, 0.62, 0.56, 0.6, 0.68, 0.72, 0.86, 0.56, 0.26, 0.62, [0.32, 0.68],
    "any control's measured effectiveness or a probe hit-rate changes materially"),
  ["residual-leakage point estimate", "uncertainty band", "score-discount guidance"],
  ["selection mechanics", "disclosure formatting"],
  [pro("An explicit residual estimate replaces the false binary 'leak-free vs leaky' with an honest quantified remainder consumers can act on",
       "reporting 'estimated residual leakage 5-12%, discount accuracy accordingly' instead of claiming zero leakage")],
  [con("The residual estimate is itself uncertain and can be falsely reassuring if its own assumptions are wrong",
       "a low residual estimate built on an under-powered probe gives unwarranted confidence")],
  ["residual leakage reported as zero", "the estimate's own uncertainty omitted, implying false precision"],
  ["a residual-leakage estimate with an uncertainty band accompanies every reported metric"],
  ["a control's measured effectiveness changes", "a post-hoc leak exceeds the residual band"],
  specialists=["evaluation_methodologist", "statistician"]))

N.append(node("POWER_AND_SAMPLE", "statistical_power_and_sample_size",
  "Pre-compute the holdout size needed to detect the target effect given that post-cutoff "
  "events are scarce, and reconcile the leakage-driven shrinkage of the usable pool with the "
  "power requirement, so the holdout is neither contaminated nor too small to conclude "
  "anything.",
  "quantification", ["POST_CUTOFF_HOLDOUT", "PRE_REGISTRATION"], ["CQ_07", "CQ_14"],
  b(0.82, 0.74, 0.7, 0.64, 0.78, 0.76, 0.6, 0.66, 0.5, 0.78, 0.8, 0.82, 0.66, 0.18, 0.5, [0.2, 0.5],
    "the target effect size, scoring rule, or the usable-pool size changes"),
  ["power analysis", "minimum-detectable-effect", "pool-shrinkage reconciliation"],
  ["probe design", "disclosure"],
  [pro("Pre-computing power before freezing prevents the common failure of a perfectly clean but uninformatively tiny holdout",
       "power analysis shows 40 post-cutoff cases are needed to detect a 10-point gap, so the pool is sized accordingly")],
  [con("Enlarging the pool to gain power can force inclusion of borderline or weaker post-cutoff events, trading purity for size",
       "reaching the required n means admitting events whose post-cutoff status is only marginally established")],
  ["holdout too small to detect the target effect", "power chased by admitting contaminated cases"],
  ["the holdout meets a pre-computed minimum-detectable-effect at the registered metric"],
  ["the target effect size is revised", "the usable post-cutoff pool shrinks below the power requirement"],
  specialists=["statistician", "evaluation_methodologist"]))

N.append(node("HOLDOUT_HYGIENE", "one_time_use_holdout_hygiene",
  "Enforce single-use hygiene: the holdout is scored once, never iterated against, never used "
  "to tune prompts or thresholds, and is retired after use, because repeated evaluation against "
  "the same holdout re-introduces leakage through adaptive overfitting.",
  "hygiene", ["RESIDUAL_LEAKAGE_ESTIMATE"], ["CQ_13", "CQ_15"],
  b(0.88, 0.8, 0.8, 0.58, 0.88, 0.82, 0.82, 0.62, 0.55, 0.78, 0.8, 0.88, 0.62, 0.18, 0.62, [0.2, 0.5],
    "the holdout is requested for a second evaluation or for tuning"),
  ["single-use rule", "no-peeking/no-tuning rule", "retirement-after-use policy"],
  ["sealed delivery mechanics", "audit cadence"],
  [pro("One-time use closes the adaptive-overfitting channel: re-running against the same holdout leaks via the analyst's reaction to prior scores",
       "a team that retunes a prompt after a failing run has leaked the holdout into their prompt, even though the model never 'saw' new data")],
  [con("Single-use is costly: a fresh leakage-controlled holdout must be built for every new evaluation cycle",
       "each model release needs a newly assembled post-cutoff holdout, which is expensive to source")],
  ["the holdout reused to tune a prompt", "scores from a prior run inform the next design without retiring the set"],
  ["the holdout is scored exactly once and retired; no tuning decision references its results"],
  ["a second evaluation against the same holdout is requested", "prompt tuning is observed to reference holdout scores"],
  specialists=["evaluation_methodologist", "ml_ops_engineer"]))

N.append(node("SEALED_DELIVERY", "sealed_blind_delivery_to_evaluator",
  "Deliver the frozen holdout to the model under a sealed, blind protocol: the evaluator runs "
  "the model without the ability to inspect or amend cases, and outputs are captured before any "
  "outcome is revealed, so the act of evaluation cannot itself introduce hindsight or selection.",
  "hygiene", ["PRE_REGISTRATION", "HOLDOUT_HYGIENE"], ["CQ_15"],
  b(0.82, 0.74, 0.74, 0.6, 0.82, 0.78, 0.66, 0.64, 0.5, 0.78, 0.8, 0.82, 0.64, 0.2, 0.52, [0.22, 0.52],
    "the delivery or output-capture protocol changes"),
  ["sealed case delivery", "output capture before outcome reveal", "evaluator no-amend rule"],
  ["case authoring", "residual estimation"],
  [pro("Sealing the cases and capturing outputs before outcomes are revealed prevents the evaluator from quietly dropping hard cases",
       "model outputs are hashed and stored before the real-world outcomes resolve, so nothing can be retrofitted")],
  [con("A fully sealed pipeline reduces the evaluator's ability to catch a genuine case-construction bug mid-run",
       "a malformed prompt cannot be fixed without breaking the seal and logging a deviation")],
  ["the evaluator edits cases during the run", "outputs captured only after outcomes are known"],
  ["all model outputs are captured and hashed before any outcome resolves, with no evaluator amendment"],
  ["the delivery protocol changes", "an output is found to have been captured post-outcome"],
  specialists=["ml_ops_engineer", "evaluation_methodologist"]))

N.append(node("LEAKAGE_AUDIT", "standing_leakage_audit",
  "Run a standing, independent leakage audit over the finished holdout and its evaluation: "
  "re-check cutoffs, re-run probes, re-screen features, and verify pre-registration integrity, "
  "producing a sign-off (or a list of defects) before any result is published.",
  "audit", ["CONTAMINATION_PROBES", "LABEL_LEAKAGE_SCREEN", "SEALED_DELIVERY"], ["CQ_11", "CQ_16"],
  b(0.9, 0.8, 0.8, 0.66, 0.9, 0.85, 0.7, 0.58, 0.62, 0.7, 0.74, 0.9, 0.58, 0.24, 0.64, [0.3, 0.66],
    "any control is changed, or a published result is challenged on leakage grounds"),
  ["independent re-check of every control", "audit sign-off or defect list", "pre-registration integrity verification"],
  ["case sourcing", "model internals"],
  [pro("An independent standing audit catches control failures the designer is blind to, closing the loop on the analyst-as-leak problem",
       "the auditor re-runs the recitation probe on the exact evaluated model and finds two contaminated cases the designer missed")],
  [con("Audits add latency and can become a rubber stamp if the auditor lacks independence or teeth",
       "an under-resourced audit signs off without re-running the expensive probes")],
  ["the audit is not independent of the designer", "controls passed at design time but drifted by evaluation time"],
  ["an independent auditor re-verifies every control and signs off or files defects before publication"],
  ["a control is modified after sign-off", "a published result is challenged on leakage grounds"],
  specialists=["leakage_auditor", "evaluation_methodologist"]))

N.append(node("PREREG_DEVIATION_LOG", "preregistration_deviation_log",
  "Maintain a transparent, append-only log of every deviation from the pre-registration "
  "(dropped cases, reworded questions, metric changes) with timestamp and justification, so "
  "post-registration changes are visible and cannot covertly tune the design toward the "
  "outcome.",
  "audit", ["PRE_REGISTRATION", "LEAKAGE_AUDIT"], ["CQ_08", "CQ_16"],
  b(0.82, 0.74, 0.74, 0.56, 0.8, 0.78, 0.7, 0.66, 0.5, 0.8, 0.82, 0.82, 0.66, 0.16, 0.5, [0.18, 0.46],
    "any deviation from the registered plan occurs"),
  ["append-only deviation log", "per-deviation justification", "timestamped change record"],
  ["case selection", "scoring"],
  [pro("An append-only deviation log preserves the credibility of pre-registration even when changes are unavoidable, per the preregistration revolution",
       "dropping three contaminated cases is logged with the probe evidence and timestamp, not done silently")],
  [con("A deviation log can be abused to legitimize so many changes that the pre-registration becomes meaningless",
       "twenty post-hoc deviations effectively rebuild the design after seeing results")],
  ["a change made without a logged justification", "deviations so numerous the registration is gutted"],
  ["every post-registration change is recorded append-only with timestamp and justification"],
  ["a silent (unlogged) deviation is discovered", "the count of deviations exceeds a credibility threshold"],
  specialists=["open_science_steward", "leakage_auditor"]))

N.append(node("NECESSARY_NOT_SUFFICIENT", "necessary_not_sufficient_finding",
  "State and enforce the core finding: 'the model/designer did not see this case during "
  "design' is NECESSARY but NOT SUFFICIENT for a leakage-free evaluation; a result may be "
  "published as leakage-controlled only if BOTH the pretraining channel and the designer-"
  "knowledge channel are demonstrably controlled, not merely the design-time visibility.",
  "reporting", ["PRETRAINING_CONTAMINATION", "DESIGNER_KNOWLEDGE_LEAKAGE", "LEAKAGE_AUDIT"], ["CQ_01", "CQ_17"],
  b(0.92, 0.84, 0.84, 0.6, 0.92, 0.86, 0.78, 0.6, 0.6, 0.74, 0.78, 0.92, 0.6, 0.2, 0.66, [0.22, 0.54],
    "a claim of leakage-freedom rests on design-time blindness alone"),
  ["necessary-not-sufficient gate", "both-channels-controlled requirement", "publishability criterion"],
  ["operator data plumbing", "power analysis"],
  [pro("Making the sufficiency gap explicit prevents the most common evaluation error: declaring a test clean because nobody peeked, while the model memorized the answer",
       "a benchmark called 'held-out' because the team kept it private still leaks if the model pretrained on the underlying events")],
  [con("Holding results to the both-channels bar will block some otherwise interesting evaluations from claiming leakage-freedom",
       "a compelling result must be downgraded to 'leakage-suspected' because the designer channel was not fully controlled")],
  ["a result published as leak-free on design-time blindness alone", "only one of the two channels is controlled but the claim is unqualified"],
  ["no result is labeled leakage-controlled unless both the pretraining and designer-knowledge channels are demonstrably controlled"],
  ["a published claim rests on visibility alone", "an audit finds one channel uncontrolled behind a leak-free claim"],
  specialists=["evaluation_methodologist", "research_integrity_officer"]))

N.append(node("LEAKAGE_DISCLOSURE", "leakage_controls_disclosure_statement",
  "Emit a structured disclosure that travels with every reported result: the confirmed cutoff, "
  "which channels were controlled and how, the probe and audit findings, the residual-leakage "
  "estimate, and the deviation log, so a consumer can independently judge the evaluation's "
  "leakage standing.",
  "reporting", ["RESIDUAL_LEAKAGE_ESTIMATE", "NECESSARY_NOT_SUFFICIENT", "PREREG_DEVIATION_LOG"], ["CQ_13", "CQ_17"],
  b(0.84, 0.78, 0.82, 0.54, 0.82, 0.8, 0.72, 0.66, 0.5, 0.8, 0.82, 0.84, 0.66, 0.16, 0.56, [0.18, 0.46],
    "any control, residual estimate, or deviation changes after disclosure"),
  ["structured leakage disclosure", "channel-by-channel control statement", "residual + deviation reporting"],
  ["internal probe mechanics", "case sourcing"],
  [pro("A disclosure that carries cutoff, controls, probes, residual estimate, and deviations lets consumers replicate the leakage judgment rather than trust it",
       "the result ships with 'cutoff 2024-03, both channels controlled, residual 5-12%, two logged deviations'")],
  [con("Disclosure can be gamed by burying weak control coverage in dense boilerplate that few readers parse",
       "a long disclosure that technically lists every channel but hides that the designer channel was uncontrolled")],
  ["a result published without its leakage disclosure", "disclosure omits the residual estimate or the deviation log"],
  ["every published result carries a structured disclosure naming cutoff, both channels, probes, residual, and deviations"],
  ["a control changes after disclosure", "a consumer cannot reconstruct the leakage judgment from the disclosure"],
  specialists=["evaluation_methodologist", "research_integrity_officer"]))


# ---------------------------------------------------------------------------
# COMPETENCY QUESTIONS (14)
# ---------------------------------------------------------------------------
CQ = [
 ("CQ_01", "What leakage channels exist, why is 'did not see it during design' necessary but not sufficient, and how is each channel owned by a control?",
  ["nodes"], "THREAT_MODEL enumerates channels; NECESSARY_NOT_SUFFICIENT and the two channel nodes enforce both-channel control",
  ["THREAT_MODEL", "PRETRAINING_CONTAMINATION", "DESIGNER_KNOWLEDGE_LEAKAGE", "NECESSARY_NOT_SUFFICIENT"]),
 ("CQ_02", "How is the model's (and designer's) effective training/knowledge cutoff determined and corroborated?",
  ["nodes"], "CUTOFF_DETERMINATION fixes and empirically confirms the cutoff that anchors post-cutoff guarantees",
  ["CUTOFF_DETERMINATION"]),
 ("CQ_03", "How is pretraining contamination controlled and how is a held-out case shown to be unseen by the model?",
  ["nodes", "edges"], "PRETRAINING_CONTAMINATION assumes memorization; POST_CUTOFF_HOLDOUT and CONTAMINATION_PROBES establish unseenness",
  ["PRETRAINING_CONTAMINATION", "POST_CUTOFF_HOLDOUT", "CONTAMINATION_PROBES"]),
 ("CQ_04", "How is designer-knowledge leakage through case selection controlled?",
  ["nodes"], "DESIGNER_KNOWLEDGE_LEAKAGE names the channel; BLIND_SELECTION removes hindsight from selection",
  ["DESIGNER_KNOWLEDGE_LEAKAGE", "BLIND_SELECTION"]),
 ("CQ_05", "How is the holdout kept strictly post-cutoff, and why is random cross-validation unsafe on time-ordered data so that temporal blocking is required?",
  ["nodes"], "POST_CUTOFF_HOLDOUT enforces post-cutoff timing; TEMPORAL_SPLIT blocks by time with an embargo because random CV leaks future into the past",
  ["POST_CUTOFF_HOLDOUT", "TEMPORAL_SPLIT"]),
 ("CQ_07", "What is frozen by pre-registration, how is the holdout sized for power, and how is leakage-driven pool shrinkage reconciled with that power?",
  ["nodes", "workflow"], "PRE_REGISTRATION freezes cases/metrics/analysis; POWER_AND_SAMPLE sizes the holdout and reconciles the scarce post-cutoff pool against the minimum-detectable-effect",
  ["PRE_REGISTRATION", "POWER_AND_SAMPLE"]),
 ("CQ_08", "How are post-registration changes kept transparent and how is selection made independent?",
  ["nodes"], "PRE_REGISTRATION freezes the plan; BLIND_SELECTION separates selector from evaluator; PREREG_DEVIATION_LOG records changes",
  ["PRE_REGISTRATION", "BLIND_SELECTION", "PREREG_DEVIATION_LOG"]),
 ("CQ_09", "How are features screened for label leakage and proven knowable strictly before the outcome resolved?",
  ["nodes"], "LABEL_LEAKAGE_SCREEN applies the legitimacy criterion and FEATURE_PROVENANCE attaches as-of dating making it an auditable date check",
  ["LABEL_LEAKAGE_SCREEN", "FEATURE_PROVENANCE"]),
 ("CQ_11", "How do contamination probes and the audit detect that a model can recite a held-out outcome?",
  ["nodes", "workflow"], "CONTAMINATION_PROBES run recitation/membership probes; LEAKAGE_AUDIT re-runs them independently",
  ["CONTAMINATION_PROBES", "RECITATION_BASELINE", "LEAKAGE_AUDIT"]),
 ("CQ_12", "How is memorized recall separated from genuine forecasting skill?",
  ["nodes"], "RECITATION_BASELINE contrasts recall-only and prediction conditions; RESIDUAL_LEAKAGE_ESTIMATE quantifies the gap",
  ["RECITATION_BASELINE", "RESIDUAL_LEAKAGE_ESTIMATE"]),
 ("CQ_13", "How is residual leakage quantified, hygiene maintained, and the result disclosed?",
  ["nodes", "workflow"], "RESIDUAL_LEAKAGE_ESTIMATE quantifies the remainder; HOLDOUT_HYGIENE keeps single use; LEAKAGE_DISCLOSURE reports it",
  ["RESIDUAL_LEAKAGE_ESTIMATE", "HOLDOUT_HYGIENE", "LEAKAGE_DISCLOSURE"]),
 ("CQ_15", "How is the holdout kept single-use and delivered to the evaluator without introducing hindsight?",
  ["nodes", "workflow"], "HOLDOUT_HYGIENE enforces one-time use; SEALED_DELIVERY seals cases and captures outputs pre-outcome",
  ["HOLDOUT_HYGIENE", "SEALED_DELIVERY"]),
 ("CQ_16", "Who independently audits the controls and how are deviations recorded before publication?",
  ["nodes"], "LEAKAGE_AUDIT independently re-checks controls; PREREG_DEVIATION_LOG records every change",
  ["LEAKAGE_AUDIT", "PREREG_DEVIATION_LOG"]),
 ("CQ_17", "Under what criterion may a result be published as leakage-controlled, and what disclosure accompanies it?",
  ["nodes", "workflow"], "NECESSARY_NOT_SUFFICIENT sets the both-channels bar; LEAKAGE_DISCLOSURE ships the structured disclosure",
  ["NECESSARY_NOT_SUFFICIENT", "LEAKAGE_DISCLOSURE"]),
]
CQS = [{"id": i, "question": q, "must_be_answerable_from": m, "acceptance_condition": a, "covered_by": c}
       for (i, q, m, a, c) in CQ]

# consolidate node->CQ references onto the 14-CQ set (each node refs 1-2)
CQ_MAP = {
 "THREAT_MODEL": ["CQ_01"], "CUTOFF_DETERMINATION": ["CQ_02"],
 "PRETRAINING_CONTAMINATION": ["CQ_01", "CQ_03"], "DESIGNER_KNOWLEDGE_LEAKAGE": ["CQ_01", "CQ_04"],
 "POST_CUTOFF_HOLDOUT": ["CQ_03", "CQ_05"], "TEMPORAL_SPLIT": ["CQ_05"],
 "PRE_REGISTRATION": ["CQ_07", "CQ_08"], "BLIND_SELECTION": ["CQ_04", "CQ_08"],
 "LABEL_LEAKAGE_SCREEN": ["CQ_09"], "FEATURE_PROVENANCE": ["CQ_09"],
 "CONTAMINATION_PROBES": ["CQ_03", "CQ_11"], "RECITATION_BASELINE": ["CQ_11", "CQ_12"],
 "RESIDUAL_LEAKAGE_ESTIMATE": ["CQ_12", "CQ_13"], "POWER_AND_SAMPLE": ["CQ_07"],
 "HOLDOUT_HYGIENE": ["CQ_13", "CQ_15"], "SEALED_DELIVERY": ["CQ_15"],
 "LEAKAGE_AUDIT": ["CQ_11", "CQ_16"], "PREREG_DEVIATION_LOG": ["CQ_08", "CQ_16"],
 "NECESSARY_NOT_SUFFICIENT": ["CQ_01", "CQ_17"], "LEAKAGE_DISCLOSURE": ["CQ_13", "CQ_17"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

# ---------------------------------------------------------------------------
# GLOSSARY
# ---------------------------------------------------------------------------
GL = [
 ("data_leakage", "use of information during model building or evaluation that would be unavailable at genuine prediction time, inflating apparent performance",
  ["leakage", "target leakage", "contamination"], ["legitimate_feature_use"], ["THREAT_MODEL", "LABEL_LEAKAGE_SCREEN"]),
 ("pretraining_contamination", "the held-out outcome was present in the model's pretraining corpus, so the model can recall rather than predict it",
  ["benchmark_contamination", "training_contamination"], ["designer_knowledge_leakage"], ["PRETRAINING_CONTAMINATION", "CONTAMINATION_PROBES"]),
 ("designer_knowledge_leakage", "hindsight held by the human designer that enters the holdout through case selection, feature choice, or thresholding",
  ["analyst_leak", "hindsight_leak"], ["pretraining_contamination"], ["DESIGNER_KNOWLEDGE_LEAKAGE", "BLIND_SELECTION"]),
 ("training_cutoff", "the date after which no data entered the model's pretraining; the anchor for every post-cutoff guarantee",
  ["knowledge_cutoff", "data_cutoff"], ["model_release_date"], ["CUTOFF_DETERMINATION", "POST_CUTOFF_HOLDOUT"]),
 ("legitimacy", "the property that a feature was knowable strictly before the outcome resolved and thus does not leak the label",
  ["point_in_time_validity"], ["post_outcome_information"], ["LABEL_LEAKAGE_SCREEN", "FEATURE_PROVENANCE"]),
 ("preregistration", "a timestamped, tamper-evident freeze of cases, questions, metrics, and analysis plan recorded before any outcome is observed",
  ["prereg", "registered_plan"], ["post_hoc_analysis"], ["PRE_REGISTRATION", "PREREG_DEVIATION_LOG"]),
 ("recitation_probe", "a test that asks the model to reproduce or complete a held-out outcome to detect memorization",
  ["memorization_probe", "contamination_probe"], ["forecast_query"], ["CONTAMINATION_PROBES", "RECITATION_BASELINE"]),
 ("temporal_blocking", "splitting time-ordered data strictly by time with an embargo gap, rather than at random, to prevent look-ahead leakage",
  ["time_series_split", "forward_chaining"], ["random_k_fold"], ["TEMPORAL_SPLIT"]),
 ("adaptive_overfitting", "leakage that accumulates when a holdout is evaluated repeatedly and the design reacts to prior scores",
  ["holdout_reuse_leak", "leaderboard_overfitting"], ["single_use_holdout"], ["HOLDOUT_HYGIENE", "LEAKAGE_AUDIT"]),
 ("residual_leakage", "the leakage estimated to remain after all controls are applied, reported with an uncertainty band",
  ["remaining_contamination"], ["zero_leakage_claim"], ["RESIDUAL_LEAKAGE_ESTIMATE", "LEAKAGE_DISCLOSURE"]),
]
GLS = [{"term": t, "definition": d, "synonyms": s, "not_same_as": ns, "used_by_nodes": u} for (t, d, s, ns, u) in GL]

# ---------------------------------------------------------------------------
# EDGES: dependency edges mirror node.dependencies (DAG) + cross-cutting edges + conflicts
# ---------------------------------------------------------------------------
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
              "risk_of_conflict": "unmanaged tension degrades leakage control", "example": "see resolution_rule"})


def rel(f, t, et, rs, cc=0.7, cp=0.2, erc=0.3, why=""):
    E.append({"from": f, "to": t, "edge_type": et, "relation_strength": rs, "signed_tension": 0.0,
              "causal_confidence": cc, "conflict_probability": cp, "expected_rework_cost": erc,
              "why_related": why or f"{f} {et} {t}", "benefit_of_coupling": "coordinated behavior",
              "risk_of_conflict": "inconsistency if uncoordinated", "example": f"{f}/{t} {et} relation"})


# --- dependency edges (acyclic; cover the spine + key fan-ins of node.dependencies) ---
dep("THREAT_MODEL", "PRETRAINING_CONTAMINATION", 0.88)
dep("CUTOFF_DETERMINATION", "PRETRAINING_CONTAMINATION", 0.84)
dep("THREAT_MODEL", "DESIGNER_KNOWLEDGE_LEAKAGE", 0.88)
dep("PRETRAINING_CONTAMINATION", "POST_CUTOFF_HOLDOUT", 0.86)
dep("CUTOFF_DETERMINATION", "TEMPORAL_SPLIT", 0.8)
dep("POST_CUTOFF_HOLDOUT", "PRE_REGISTRATION", 0.84)
dep("DESIGNER_KNOWLEDGE_LEAKAGE", "BLIND_SELECTION", 0.86)
dep("PRE_REGISTRATION", "BLIND_SELECTION", 0.78)
dep("BLIND_SELECTION", "LABEL_LEAKAGE_SCREEN", 0.8)
dep("LABEL_LEAKAGE_SCREEN", "FEATURE_PROVENANCE", 0.82)
dep("POST_CUTOFF_HOLDOUT", "CONTAMINATION_PROBES", 0.82)
dep("CONTAMINATION_PROBES", "RECITATION_BASELINE", 0.82)
dep("RECITATION_BASELINE", "RESIDUAL_LEAKAGE_ESTIMATE", 0.82)
dep("FEATURE_PROVENANCE", "RESIDUAL_LEAKAGE_ESTIMATE", 0.78)
dep("POST_CUTOFF_HOLDOUT", "POWER_AND_SAMPLE", 0.78)
dep("RESIDUAL_LEAKAGE_ESTIMATE", "HOLDOUT_HYGIENE", 0.82)
dep("HOLDOUT_HYGIENE", "SEALED_DELIVERY", 0.8)
dep("CONTAMINATION_PROBES", "LEAKAGE_AUDIT", 0.82)
dep("SEALED_DELIVERY", "LEAKAGE_AUDIT", 0.78)
dep("LEAKAGE_AUDIT", "PREREG_DEVIATION_LOG", 0.76)
dep("PRETRAINING_CONTAMINATION", "NECESSARY_NOT_SUFFICIENT", 0.84)
dep("DESIGNER_KNOWLEDGE_LEAKAGE", "NECESSARY_NOT_SUFFICIENT", 0.84)
dep("NECESSARY_NOT_SUFFICIENT", "LEAKAGE_DISCLOSURE", 0.8)
dep("RESIDUAL_LEAKAGE_ESTIMATE", "LEAKAGE_DISCLOSURE", 0.8)

# --- cross-cutting non-dependency edges (cross freely; no cycle risk) ---
rel("CONTAMINATION_PROBES", "POST_CUTOFF_HOLDOUT", "feedback", 0.8,
    why="a probe hit feeds back to drop the case from the post-cutoff pool")
rel("LEAKAGE_AUDIT", "CUTOFF_DETERMINATION", "feedback", 0.76,
    why="the audit can revise the effective cutoff if near-boundary probing reveals a later one")
rel("FEATURE_PROVENANCE", "POST_CUTOFF_HOLDOUT", "constraint", 0.74,
    why="as-of dating of features constrains which post-cutoff events can be fully reconstructed")
rel("POWER_AND_SAMPLE", "BLIND_SELECTION", "constraint", 0.72,
    why="the power-driven target size constrains the blind selector's sampling rule")
rel("RECITATION_BASELINE", "NECESSARY_NOT_SUFFICIENT", "causal", 0.74,
    why="a high recall baseline is direct evidence the necessary-not-sufficient gate must block a leak-free claim")
rel("PREREG_DEVIATION_LOG", "HOLDOUT_HYGIENE", "constraint", 0.7,
    why="reusing or re-tuning the holdout would generate deviations the log must capture")
rel("TEMPORAL_SPLIT", "LABEL_LEAKAGE_SCREEN", "similarity", 0.7,
    why="temporal look-ahead and feature label leakage are two forms of the same prediction-time-availability violation")
rel("THREAT_MODEL", "LEAKAGE_AUDIT", "similarity", 0.72,
    why="the audit checklist is the threat model re-applied independently at the end")

# --- conflict edges (negative signed_tension + resolution_rule) ---
conf("POST_CUTOFF_HOLDOUT", "POWER_AND_SAMPLE", 0.72, -0.6,
  "purity dominates: never admit borderline-cutoff cases to reach power; instead lower the target effect or report an under-powered result with widened confidence intervals",
  "strict post-cutoff sourcing shrinks the pool, conflicting with the sample size needed for statistical power")
conf("DESIGNER_KNOWLEDGE_LEAKAGE", "POST_CUTOFF_HOLDOUT", 0.7, -0.5,
  "post-cutoff timing controls the model channel but not the designer channel; require BLIND_SELECTION on the post-cutoff pool so neither channel is left open",
  "post-cutoff events can still be designer-leaked through which post-cutoff cases the designer picks")
conf("BLIND_SELECTION", "POWER_AND_SAMPLE", 0.68, -0.45,
  "constrain the blind selector with a pre-registered stratified rule so blindness yields enough informative cases to meet power without hand-picking",
  "fully blind random selection can produce too few or degenerate cases to satisfy the power requirement")
conf("HOLDOUT_HYGIENE", "POWER_AND_SAMPLE", 0.66, -0.4,
  "single-use forbids reusing the holdout to chase power; build a larger or fresh holdout rather than re-scoring the retired one",
  "one-time-use hygiene conflicts with the temptation to reuse a scarce holdout to accumulate statistical power")

# ---------------------------------------------------------------------------
# CONFLICT AXES (9)
# ---------------------------------------------------------------------------
CA = [
 {"name": "purity_vs_statistical_power", "description": "Strict post-cutoff sourcing maximizes leakage control but shrinks the pool below the size needed to detect the target effect.",
  "poles": ["maximal_purity", "adequate_power"], "resolution_hint": "lower the target effect or widen intervals rather than admitting borderline-cutoff cases", "tension_score": 0.75,
  "affected_nodes": ["POST_CUTOFF_HOLDOUT", "POWER_AND_SAMPLE", "BLIND_SELECTION"]},
 {"name": "pretraining_channel_vs_designer_channel", "description": "Controlling pretraining ingestion does not control designer hindsight, and vice versa; both must be closed independently.",
  "poles": ["model_channel_focus", "designer_channel_focus"], "resolution_hint": "require demonstrable control of BOTH channels before any leak-free claim", "tension_score": 0.78,
  "affected_nodes": ["PRETRAINING_CONTAMINATION", "DESIGNER_KNOWLEDGE_LEAKAGE", "NECESSARY_NOT_SUFFICIENT"]},
 {"name": "designer_expertise_vs_designer_blindness", "description": "The domain expertise needed to build a good forecasting task carries the very outcome hindsight that must be excluded.",
  "poles": ["expert_designer", "outcome_blind_designer"], "resolution_hint": "separate task authoring from blind case selection by an uninvolved party", "tension_score": 0.7,
  "affected_nodes": ["DESIGNER_KNOWLEDGE_LEAKAGE", "BLIND_SELECTION", "THREAT_MODEL"]},
 {"name": "preregistration_rigidity_vs_correctability", "description": "Freezing the design prevents post-hoc tuning but blocks fixing a genuine flaw discovered after registration.",
  "poles": ["strict_freeze", "logged_correction"], "resolution_hint": "allow only append-only logged deviations with justification", "tension_score": 0.65,
  "affected_nodes": ["PRE_REGISTRATION", "PREREG_DEVIATION_LOG", "LEAKAGE_AUDIT"]},
 {"name": "memorization_vs_exploitation", "description": "A model can ingest a contaminated item yet not exploit it; treating all memorization as fatal over-excludes, ignoring it under-detects.",
  "poles": ["exclude_all_memorized", "exclude_only_exploited"], "resolution_hint": "use a recall baseline to measure exploitation, not just memorization", "tension_score": 0.68,
  "affected_nodes": ["PRETRAINING_CONTAMINATION", "RECITATION_BASELINE", "CONTAMINATION_PROBES"]},
 {"name": "probe_sensitivity_vs_false_negatives", "description": "Recitation probes detect verbatim memorization but miss non-recited exploitation, so a clean probe can falsely reassure.",
  "poles": ["trust_clean_probe", "assume_probe_under_detects"], "resolution_hint": "pair probes with a recall baseline and report residual leakage, never zero", "tension_score": 0.66,
  "affected_nodes": ["CONTAMINATION_PROBES", "RECITATION_BASELINE", "RESIDUAL_LEAKAGE_ESTIMATE"]},
 {"name": "single_use_vs_holdout_reuse", "description": "One-time use prevents adaptive overfitting but forces rebuilding a scarce holdout for every cycle.",
  "poles": ["single_use", "reuse_for_power"], "resolution_hint": "build fresh holdouts; never re-score a retired one to chase power", "tension_score": 0.64,
  "affected_nodes": ["HOLDOUT_HYGIENE", "POWER_AND_SAMPLE", "SEALED_DELIVERY"]},
 {"name": "point_in_time_fidelity_vs_data_availability", "description": "Proving features were knowable before the outcome requires point-in-time data that many sources silently back-fill.",
  "poles": ["strict_point_in_time", "use_available_revised_values"], "resolution_hint": "require an as-of date per feature; quarantine values that cannot be reconstructed", "tension_score": 0.62,
  "affected_nodes": ["FEATURE_PROVENANCE", "LABEL_LEAKAGE_SCREEN", "POST_CUTOFF_HOLDOUT"]},
 {"name": "disclosure_completeness_vs_readability", "description": "A complete leakage disclosure is long and can bury weak control coverage that few readers parse.",
  "poles": ["exhaustive_disclosure", "concise_headline"], "resolution_hint": "lead with a channel-by-channel control summary and residual estimate, detail in appendix", "tension_score": 0.55,
  "affected_nodes": ["LEAKAGE_DISCLOSURE", "RESIDUAL_LEAKAGE_ESTIMATE", "NECESSARY_NOT_SUFFICIENT"]},
]

# ---------------------------------------------------------------------------
# EDGE CASES (12) -- note: builder maps affected_nodes; validator reads c.get("nodes")
# kb_forge stores them under affected_nodes; we provide affected_nodes (builder key).
# ---------------------------------------------------------------------------
EC = [
 {"description": "A held-out case is a famous historical event the model memorized verbatim during pretraining, so a high score reflects recall, not prediction.",
  "trigger": "a pre-cutoff famous outcome is used as a test item", "affected_nodes": ["PRETRAINING_CONTAMINATION", "POST_CUTOFF_HOLDOUT", "CONTAMINATION_PROBES"],
  "mitigation": "restrict to strictly post-cutoff events and drop any case the model can recite", "severity": "critical"},
 {"description": "The designer, knowing which firms later failed, quietly over-samples failures into a nominally balanced holdout.",
  "trigger": "the designer selects cases while knowing outcomes", "affected_nodes": ["DESIGNER_KNOWLEDGE_LEAKAGE", "BLIND_SELECTION"],
  "mitigation": "have an uninvolved, outcome-blind party draw cases by a pre-registered rule", "severity": "critical"},
 {"description": "The team declares a benchmark 'held-out' because they kept it private, while the underlying events were in pretraining.",
  "trigger": "a leak-free claim rests on design-time blindness alone", "affected_nodes": ["NECESSARY_NOT_SUFFICIENT", "PRETRAINING_CONTAMINATION"],
  "mitigation": "enforce the both-channels bar: control pretraining AND designer channels, not just visibility", "severity": "critical"},
 {"description": "Random k-fold cross-validation is used on time-ordered evaluation data, leaking future periods into the training side.",
  "trigger": "random CV applied to time-ordered data", "affected_nodes": ["TEMPORAL_SPLIT", "RESIDUAL_LEAKAGE_ESTIMATE"],
  "mitigation": "split strictly by time with an embargo gap and forbid look-ahead features", "severity": "high"},
 {"description": "A context document dated after the event already names the outcome and is fed into the model's prompt.",
  "trigger": "a post-outcome document leaks into the case context", "affected_nodes": ["LABEL_LEAKAGE_SCREEN", "FEATURE_PROVENANCE"],
  "mitigation": "screen every feature for as-of date strictly before resolution; quarantine post-outcome text", "severity": "high"},
 {"description": "An identifier or back-filled timestamp silently encodes the label (e.g. an ID assigned only to firms that later failed).",
  "trigger": "a feature is a near-deterministic outcome proxy", "affected_nodes": ["LABEL_LEAKAGE_SCREEN", "FEATURE_PROVENANCE"],
  "mitigation": "apply the legitimacy criterion and point-in-time reconstruction to every feature", "severity": "high"},
 {"description": "A recitation probe comes back clean, yet the model still exploits contaminated knowledge without reproducing it verbatim.",
  "trigger": "a clean probe is taken as proof of zero contamination", "affected_nodes": ["CONTAMINATION_PROBES", "RECITATION_BASELINE", "RESIDUAL_LEAKAGE_ESTIMATE"],
  "mitigation": "pair probes with a recall baseline and report a non-zero residual-leakage estimate", "severity": "high"},
 {"description": "After a failing run, the team re-tunes the prompt and re-scores the same holdout, leaking it via adaptive overfitting.",
  "trigger": "the holdout is reused to tune prompts or thresholds", "affected_nodes": ["HOLDOUT_HYGIENE", "SEALED_DELIVERY"],
  "mitigation": "enforce single-use and retire the holdout after one scoring", "severity": "high"},
 {"description": "The usable post-cutoff pool is too small to detect the target effect, so the clean holdout is uninformative.",
  "trigger": "leakage control shrinks the pool below the power requirement", "affected_nodes": ["POWER_AND_SAMPLE", "POST_CUTOFF_HOLDOUT"],
  "mitigation": "pre-compute power; lower the target effect or widen intervals rather than admitting contaminated cases", "severity": "medium"},
 {"description": "A vendor-stated cutoff is trusted, but continued pretraining or retrieval injects post-cutoff knowledge the date does not capture.",
  "trigger": "the cutoff is taken from documentation without empirical probing", "affected_nodes": ["CUTOFF_DETERMINATION", "CONTAMINATION_PROBES"],
  "mitigation": "empirically corroborate the cutoff against near-boundary events and account for retrieval", "severity": "high"},
 {"description": "Cases are silently dropped after results are seen, with no record, quietly tuning the design toward a flattering outcome.",
  "trigger": "post-registration changes are made without a deviation log", "affected_nodes": ["PRE_REGISTRATION", "PREREG_DEVIATION_LOG"],
  "mitigation": "record every change append-only with timestamp and justification before publication", "severity": "high"},
 {"description": "The evaluation audit is performed by the same person who designed the holdout, so designer blind spots survive into publication.",
  "trigger": "the leakage audit is not independent of the designer", "affected_nodes": ["LEAKAGE_AUDIT", "THREAT_MODEL"],
  "mitigation": "assign an independent auditor to re-run probes and re-screen features before sign-off", "severity": "medium"},
]

# ---------------------------------------------------------------------------
# WORKFLOW (12)
# ---------------------------------------------------------------------------
WF = [
 {"action": "model_threats", "node_ref": "THREAT_MODEL", "description": "Enumerate every leakage channel treating the system as model PLUS designer; map each channel to an owning control.",
  "artifact": "leakage_threat_model", "gate": "every channel has an owning control and a residual-risk note"},
 {"action": "fix_cutoff", "node_ref": "CUTOFF_DETERMINATION", "description": "Determine and empirically corroborate the model and designer knowledge cutoff.",
  "artifact": "confirmed_cutoff_record", "gate": "the cutoff is documented and probed against near-boundary events"},
 {"action": "control_channels", "node_ref": "DESIGNER_KNOWLEDGE_LEAKAGE", "description": "Stand up controls for both the pretraining and designer-knowledge channels.",
  "artifact": "channel_control_plan", "gate": "both channels have an assigned, instantiated control"},
 {"action": "source_post_cutoff_pool", "node_ref": "POST_CUTOFF_HOLDOUT", "description": "Assemble candidate events whose outcomes resolved strictly after the confirmed cutoff.",
  "artifact": "post_cutoff_event_pool", "gate": "every pooled event resolves strictly after the cutoff"},
 {"action": "preregister", "node_ref": "PRE_REGISTRATION", "description": "Freeze cases, question wording, metric, and analysis plan in a timestamped tamper-evident registration before any output is seen.",
  "artifact": "preregistration_record", "gate": "cases, metric, and analysis plan are hashed and timestamped"},
 {"action": "blind_select", "node_ref": "BLIND_SELECTION", "description": "Have an uninvolved, outcome-blind party draw the final cases by the pre-registered rule.",
  "artifact": "blind_selection_record", "gate": "selection is made by an uninvolved outcome-blind party"},
 {"action": "screen_features", "node_ref": "LABEL_LEAKAGE_SCREEN", "description": "Screen every feature and context document for label leakage and attach as-of provenance.",
  "artifact": "feature_legitimacy_report", "gate": "every feature is knowable strictly before resolution"},
 {"action": "probe_contamination", "node_ref": "CONTAMINATION_PROBES", "description": "Probe the exact evaluated model for recitation of each held-out outcome; quarantine any recitable case.",
  "artifact": "contamination_probe_report", "gate": "no retained case is recitable by the evaluated model"},
 {"action": "size_and_baseline", "node_ref": "POWER_AND_SAMPLE", "description": "Verify the holdout meets the pre-computed minimum-detectable-effect and set the recall baseline.",
  "artifact": "power_and_baseline_record", "gate": "the holdout meets its minimum-detectable-effect"},
 {"action": "estimate_residual", "node_ref": "RESIDUAL_LEAKAGE_ESTIMATE", "description": "Combine probe, baseline, and feature-screen residuals into a residual-leakage estimate with an uncertainty band.",
  "artifact": "residual_leakage_estimate", "gate": "a residual estimate with an uncertainty band exists"},
 {"action": "seal_and_evaluate", "node_ref": "SEALED_DELIVERY", "description": "Deliver the sealed holdout, run the model once under hygiene rules, and capture outputs before outcomes resolve.",
  "artifact": "sealed_evaluation_outputs", "gate": "outputs are captured and hashed before any outcome resolves"},
 {"action": "audit_and_disclose", "node_ref": "LEAKAGE_AUDIT", "description": "Run the independent leakage audit, log deviations, and publish only with the structured leakage disclosure.",
  "artifact": "audit_signoff_and_disclosure", "gate": "an independent audit signs off and a disclosure ships with the result"},
]

# ---------------------------------------------------------------------------
# DOMINANCE RULES (9)
# ---------------------------------------------------------------------------
DR = [
 {"rule": "CUTOFF_DETERMINATION must be confirmed before any case is admitted to the post-cutoff pool",
  "rationale": "every post-cutoff guarantee is anchored to the confirmed cutoff; an unconfirmed cutoff invalidates the whole pool",
  "trigger": "cases are sourced before the cutoff is empirically corroborated", "action": "block pool assembly until the cutoff is confirmed"},
 {"rule": "Both the pretraining and designer-knowledge channels must be controlled before any result is labeled leakage-controlled",
  "rationale": "design-time blindness is necessary but not sufficient; one open channel leaks regardless of the other",
  "trigger": "a leak-free claim rests on only one channel or on visibility alone", "action": "block the leak-free label until both channels are demonstrably controlled"},
 {"rule": "Case selection and metrics must be pre-registered before any model output or outcome is observed",
  "rationale": "selecting or scoring after seeing results reintroduces researcher degrees of freedom",
  "trigger": "metric or case set chosen after an output is seen", "action": "reject the result and require a fresh pre-registration"},
 {"rule": "Final case selection must be performed by an uninvolved outcome-blind party",
  "rationale": "a designer who knows outcomes leaks them through which cases appear",
  "trigger": "the designer hand-picks the final cases", "action": "void the selection and re-draw blindly"},
 {"rule": "Every feature must pass the legitimacy (as-of date) check before its case is finalized",
  "rationale": "a feature unavailable at prediction time is label leakage",
  "trigger": "a feature lacks an as-of date strictly before resolution", "action": "quarantine the feature or drop the case"},
 {"rule": "Any case the evaluated model can recite must be dropped or quarantined",
  "rationale": "a recitable outcome is memorized, not predicted",
  "trigger": "a recitation probe succeeds on a retained case", "action": "drop the case from the scored set"},
 {"rule": "Residual leakage must be reported as a non-zero estimate with an uncertainty band; zero-leakage claims are forbidden",
  "rationale": "probes under-detect non-recited exploitation, so zero leakage cannot be asserted",
  "trigger": "a result claims zero or unstated residual leakage", "action": "require a residual estimate with its uncertainty before publication"},
 {"rule": "The holdout must be scored exactly once and retired; reuse for tuning is prohibited",
  "rationale": "repeated evaluation leaks the holdout through adaptive overfitting",
  "trigger": "a second scoring or a tuning decision references holdout results", "action": "retire the holdout and require a fresh one"},
 {"rule": "The leakage audit must be independent of the designer and complete before publication",
  "rationale": "a self-audit preserves the designer's blind spots",
  "trigger": "the auditor is the designer or lacks independence", "action": "assign an independent auditor and block publication until sign-off"},
]

# ---------------------------------------------------------------------------
# ANTI-REWORK RULES (8)
# ---------------------------------------------------------------------------
ARR = [
 {"rule": "Do not source the candidate pool before confirming the cutoff; a revised cutoff invalidates every sourced case",
  "prevents": "re-sourcing the entire pool after the effective cutoff is found to be later than assumed"},
 {"rule": "Do not let the designer pick cases; reconstructing a blind selection after the fact requires discarding the contaminated set",
  "prevents": "rebuilding the holdout after a hindsight-laden selection is discovered"},
 {"rule": "Do not finalize features without as-of provenance; retrofitting point-in-time dating forces re-screening every case",
  "prevents": "re-screening all features after a back-filled value is found to leak the label"},
 {"rule": "Do not run probes on a different model version than the one evaluated; a version change voids the probe evidence",
  "prevents": "re-running the full contamination probe suite after the evaluated model version changes"},
 {"rule": "Do not choose the metric or threshold after seeing outputs; post-hoc metric changes require a fresh pre-registration and holdout",
  "prevents": "scrapping a result and rebuilding the holdout because the metric was tuned to the answer"},
 {"rule": "Do not reuse the holdout to chase statistical power; a reused holdout is leaked and cannot be un-leaked",
  "prevents": "discarding adaptive-overfit results and assembling a fresh holdout under deadline"},
 {"rule": "Do not make post-registration changes without a deviation log; unlogged edits force re-justifying the whole design",
  "prevents": "reconstructing the change history to defend a challenged result"},
 {"rule": "Do not publish before the independent audit signs off; a post-publication leak finding forces a retraction",
  "prevents": "retracting a published result after an independent audit finds an uncontrolled channel"},
]

# ---------------------------------------------------------------------------
# ITERATION PROTOCOL (8)
# ---------------------------------------------------------------------------
IP = [
 {"trigger": "near-boundary probing reveals the effective cutoff is later than assumed",
  "action": "revise CUTOFF_DETERMINATION and re-source POST_CUTOFF_HOLDOUT against the corrected date",
  "nodes": ["CUTOFF_DETERMINATION", "POST_CUTOFF_HOLDOUT"], "priority": "critical"},
 {"trigger": "a recitation probe shows the evaluated model can reproduce a held-out outcome",
  "action": "drop the case in CONTAMINATION_PROBES and re-estimate residual leakage",
  "nodes": ["CONTAMINATION_PROBES", "POST_CUTOFF_HOLDOUT", "RESIDUAL_LEAKAGE_ESTIMATE"], "priority": "critical"},
 {"trigger": "a result is claimed leakage-free on design-time blindness alone",
  "action": "apply the NECESSARY_NOT_SUFFICIENT gate and verify both channels are controlled before any leak-free label",
  "nodes": ["NECESSARY_NOT_SUFFICIENT", "PRETRAINING_CONTAMINATION", "DESIGNER_KNOWLEDGE_LEAKAGE"], "priority": "high"},
 {"trigger": "selected cases are found to correlate with known outcomes",
  "action": "void the selection and re-draw blindly in BLIND_SELECTION under the pre-registered rule",
  "nodes": ["BLIND_SELECTION", "DESIGNER_KNOWLEDGE_LEAKAGE"], "priority": "high"},
 {"trigger": "a feature is found to be a back-filled or post-outcome value",
  "action": "quarantine it in LABEL_LEAKAGE_SCREEN and reconstruct point-in-time provenance in FEATURE_PROVENANCE",
  "nodes": ["LABEL_LEAKAGE_SCREEN", "FEATURE_PROVENANCE"], "priority": "high"},
 {"trigger": "the usable post-cutoff pool shrinks below the power requirement",
  "action": "lower the target effect or widen intervals in POWER_AND_SAMPLE rather than admitting contaminated cases",
  "nodes": ["POWER_AND_SAMPLE", "POST_CUTOFF_HOLDOUT"], "priority": "medium"},
 {"trigger": "the holdout is requested for a second evaluation or for tuning",
  "action": "retire it under HOLDOUT_HYGIENE and assemble a fresh holdout instead of re-scoring",
  "nodes": ["HOLDOUT_HYGIENE", "SEALED_DELIVERY"], "priority": "medium"},
 {"trigger": "an independent audit files a leakage defect after sign-off was assumed",
  "action": "log the deviation in PREREG_DEVIATION_LOG and update the disclosure before publication",
  "nodes": ["LEAKAGE_AUDIT", "PREREG_DEVIATION_LOG", "LEAKAGE_DISCLOSURE"], "priority": "high"},
]

# ---------------------------------------------------------------------------
# SPEC
# ---------------------------------------------------------------------------
spec = {
 "domain": "leakage__holdout_design",
 "domain_label": "Anti-Leakage Holdout Design (pretraining + designer knowledge)",
 "purpose": "construct_a_leakage_controlled_strictly_post_cutoff_pre_registered_blind_holdout_that_controls_both_model_pretraining_and_designer_knowledge_contamination",
 "assumptions": [
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "the system under test is the LLM together with the human designer; the designer's hindsight is part of the predicting system",
   "a verifiable training/knowledge cutoff can be determined and empirically corroborated for the model under evaluation",
   "post-cutoff events whose outcomes resolve after the cutoff are available, though scarcer than historical cases",
 ],
 "exclusions": [
   "model training, fine-tuning, or architecture choices (the evaluation treats the model as fixed)",
   "the substantive forecasting/prediction method being evaluated (this KB designs the holdout, not the predictor)",
   "general benchmark construction unrelated to leakage (item difficulty calibration, annotation guidelines)",
   "downstream statistical modeling beyond the power/sample reconciliation needed to size the holdout",
 ],
 "source_description": "heuristic prior estimates for anti-leakage holdout design work units, informed by the data-mining leakage, preregistration, LLM benchmark-contamination, and temporal-validation literatures; no supplied dataset",
 "source_citation": "Kaufman, Rosset, Perlich & Stitelman 2012 'Leakage in Data Mining: Formulation, Detection, and Avoidance' (ACM TKDD 6(4)); Nosek, Ebersole, DeHaven & Mellor 2018 'The preregistration revolution' (PNAS 115(11)); Sainz, Campos, Garcia-Ferrero, Etxaniz, de Lacalle & Agirre 2023 'NLP Evaluation in Trouble: On the Need to Measure LLM Data Contamination for each Benchmark' (EMNLP Findings); Magar & Schwartz 2022 'Data Contamination: From Memorization to Exploitation' (ACL); Bergmeir & Benitez 2012 'On the use of cross-validation for time series predictor evaluation' (Information Sciences 191)",
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
 "priority_rationale": "THREAT_MODEL and CUTOFF_DETERMINATION are foundational; the two channel-control nodes and POST_CUTOFF_HOLDOUT establish the core guarantees; pre-registration, blind selection, feature screening, and probes operationalize them; residual estimation, hygiene, audit, and the necessary-not-sufficient gate close the method before disclosure.",
 "eval_objective": "verify_dual_channel_leakage_control_post_cutoff_sourcing_preregistration_contamination_probing_and_necessary_not_sufficient_publishability_of_leakage__holdout_design_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "leakage__holdout_design.spec.json")
open(path, "w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes", len(N), "edges", len(E), "CA", len(CA), "EC", len(EC), "WF", len(WF),
      "CQ", len(CQS), "DR", len(DR), "ARR", len(ARR), "IP", len(IP))
