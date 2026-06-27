#!/usr/bin/env python3
"""Generate the backtest__outcome_validity content spec (B60 KB) for kb_forge.py.
Compact authoring: node() applies sane defaults so only domain content + base
metric magnitudes are specified per node. Domain: running outcome-based backtests
against real known historical results with base-rate calibration and controls for
survivorship, hindsight, and single-origin evidence bias, plus overfitting/multiple
-testing/regime-change defenses, and treating 'medium-tier is the opportunity' as a
falsifiable bet measured with a CI (scope FP1)."""
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
        "academic_fields": ["statistical_validation", "econometrics_of_backtesting", "judgment_and_decision_making"],
        "subfields": subfields or ["backtest_overfitting", "bias_control", "base_rate_calibration"],
        "specialists": specialists or ["quantitative_validation_analyst"],
        "contradictors": contradictors or ["raw_accuracy_advocate"],
        "inputs": inputs or ["historical outcome record", "candidate predictive insight"],
        "outputs": outputs or ["validity-controlled backtest verdict"],
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

N.append(node("OUTCOME_DEFINITION","backtest_outcome_and_label_definition",
  "Define the binary or graded outcome the backtest scores against (e.g. 'went viral', 'beat threshold'), pin its measurement window, and freeze the labeling rule before any prediction is scored, so success is a fixed external fact rather than a movable target.",
  "foundations", [], ["CQ_01"],
  b(0.9,0.82,0.78,0.55,0.82,0.7,0.65,0.7,0.4, 0.78,0.82, 0.9,0.7,0.18,0.55,[0.2,0.5],"outcome definition, threshold, or measurement window changes"),
  ["outcome label rule","measurement window","success threshold"],
  ["model fitting","feature engineering"],
  [pro("A frozen, externally measurable outcome makes the backtest falsifiable and prevents post hoc relabeling of wins","'viral' fixed as >100k views within 14 days, set before scoring any item")],
  [con("A poorly chosen threshold can make almost everything or almost nothing a success, destroying discrimination","a 1M-view bar labels 0.2% positive, leaving too few positives to estimate anything")],
  ["outcome silently redefined after seeing results","window so long that later confounds contaminate the label"],
  ["outcome label rule is written down and frozen before any case is scored","positive class has enough instances to estimate a rate"],
  ["outcome threshold revised","measurement window changed"],
  specialists=["quantitative_validation_analyst","measurement_designer"]))

N.append(node("HISTORICAL_GROUND_TRUTH","real_known_results_ledger",
  "Assemble the ledger of real, already-known historical results the backtest tests against: actual past items with their actual realized outcomes, with provenance and timestamps, distinguishing genuinely known outcomes from estimated or imputed ones.",
  "foundations", ["OUTCOME_DEFINITION"], ["CQ_01","CQ_02"],
  b(0.9,0.8,0.76,0.6,0.84,0.74,0.7,0.66,0.45, 0.76,0.8, 0.88,0.66,0.2,0.56,[0.22,0.55],"new historical records added or outcome provenance corrected"),
  ["realized-outcome ledger","provenance and timestamps","known vs imputed flag"],
  ["forward prediction","live deployment scoring"],
  [pro("Testing against real known results, not a simulated proxy, is the core of an outcome backtest and grounds every later claim","each past post carries its true realized reach, not a modeled estimate")],
  [con("Historical ledgers are themselves selected and incomplete; what was recorded is not a random sample of what happened","platform only retained metrics for posts that crossed a logging threshold")],
  ["imputed outcomes treated as known truth","ledger silently filtered by an unrecorded inclusion rule"],
  ["every scored case has a real realized outcome with provenance","imputed outcomes are flagged and excluded from primary scoring"],
  ["ledger inclusion rule discovered","outcome provenance corrected"],
  specialists=["quantitative_validation_analyst","data_provenance_engineer"]))

N.append(node("SAMPLING_FRAME","population_and_inclusion_frame",
  "Specify the population the backtest claims to generalize to and the inclusion rule that admits a case into the sample, so the tested set is a defined, defensible slice of that population rather than an arbitrary convenience set.",
  "foundations", ["HISTORICAL_GROUND_TRUTH"], ["CQ_02","CQ_03"],
  b(0.86,0.76,0.72,0.62,0.82,0.8,0.62,0.64,0.5, 0.74,0.78, 0.86,0.64,0.2,0.54,[0.24,0.58],"target population or inclusion rule changes"),
  ["target population definition","case inclusion rule","frame-vs-population gap audit"],
  ["model internals","threshold tuning"],
  [pro("An explicit sampling frame turns 'works in general' into a checkable claim about a named population","frame = all English short-form videos posted by accounts under 50k followers in 2024")],
  [con("Any frame narrows external validity; a result true on the frame need not hold on the population of interest","trained on creator-economy posts but deployed on brand ads")],
  ["frame conflated with the full population","convenience sample passed off as representative"],
  ["the population and inclusion rule are written down and the frame-to-population gap is stated"],
  ["target population redefined","systematic exclusion discovered in the frame"],
  specialists=["quantitative_validation_analyst","sampling_statistician"]))

N.append(node("SURVIVORSHIP_BIAS","survivorship_bias_control",
  "Detect and correct survivorship bias: the systematic over-representation of winners because losers, deleted items, and dead accounts are absent from the record, which inflates apparent success rates and corrupts any model fit only on survivors.",
  "bias_control", ["SAMPLING_FRAME"], ["CQ_03","CQ_04"],
  b(0.92,0.82,0.78,0.66,0.88,0.82,0.72,0.6,0.6, 0.7,0.74, 0.92,0.6,0.24,0.62,[0.28,0.66],"data source's retention/deletion policy or the dead-account recovery method changes"),
  ["dead/deleted-case recovery","survivor-vs-full-cohort comparison","retention-policy audit"],
  ["forward deployment","live monitoring"],
  [pro("Reconstructing the full entry cohort (including failures) removes the upward bias that survivor-only data imposes","recovering deleted/flopped posts from a pre-deletion crawl restores the true denominator")],
  [con("Lost cases are often unrecoverable, so the correction is itself an estimate with its own uncertainty","deleted posts leave no trace, so the failure rate must be bounded, not measured")],
  ["model trained only on surviving winners","success rate computed over survivors as if over entrants"],
  ["the entry cohort is reconstructed or a defensible bound on the missing-failure mass is stated","success rates are reported over entrants, not survivors"],
  ["data source retention policy changes","a large block of deleted/dead cases is discovered"],
  specialists=["quantitative_validation_analyst","survivorship_auditor"],
  contradictors=["winners_only_case_study_advocate"]))

N.append(node("BASE_RATE_CALIBRATION","outcome_base_rate_calibration",
  "Calibrate every claim against the outcome's base rate, not raw accuracy: report lift over the base rate, precision against the prevalence, and avoid the base-rate-neglect trap where a model that always predicts the majority class looks accurate while adding no information.",
  "calibration", ["SURVIVORSHIP_BIAS"], ["CQ_04","CQ_05"],
  b(0.92,0.84,0.82,0.64,0.86,0.84,0.66,0.62,0.58, 0.72,0.76, 0.92,0.62,0.22,0.6,[0.26,0.62],"outcome prevalence shifts or the calibration metric set changes"),
  ["base-rate estimation","lift-over-base-rate reporting","calibration curve / reliability check"],
  ["model architecture","raw-accuracy-only reporting"],
  [pro("Scoring lift over the base rate exposes models that merely exploit prevalence and adds the information raw accuracy hides","at a 3% viral base rate, 97% accuracy from always-predicting-no is worthless; lift makes that explicit")],
  [con("Base rates can be unstable or unknown for rare outcomes, making the calibration denominator itself uncertain","a 0.5% base rate estimated from 200 cases has a wide CI on prevalence")],
  ["raw accuracy reported against a skewed base rate","prevalence assumed constant when it drifts"],
  ["every accuracy claim is paired with the base rate and a lift figure","a no-skill majority-class baseline is reported alongside the model"],
  ["outcome prevalence shifts materially","a new majority-class baseline beats the model on raw accuracy"],
  specialists=["quantitative_validation_analyst","calibration_statistician"],
  contradictors=["raw_accuracy_advocate"]))

N.append(node("HINDSIGHT_BIAS","hindsight_bias_control",
  "Control hindsight bias ('I knew it all along'): the tendency, after an outcome is known, to see it as having been predictable, which inflates retrospective confidence and contaminates any feature or rule chosen with knowledge of the outcome.",
  "bias_control", ["HISTORICAL_GROUND_TRUTH"], ["CQ_05","CQ_06"],
  b(0.86,0.78,0.74,0.62,0.84,0.8,0.62,0.6,0.55, 0.72,0.76, 0.86,0.6,0.24,0.58,[0.28,0.66],"feature-selection-vs-outcome separation protocol changes"),
  ["outcome-blind feature derivation","pre-registration of the rule","memory/justification de-biasing"],
  ["model fitting on full data","ad hoc rule discovery"],
  [pro("Deriving features and rules without seeing the outcome breaks the 'knew-it-all-along' loop that makes the past look obvious","analysts label predictive features on masked cases before outcomes are revealed")],
  [con("Full outcome blinding is hard to enforce when the same humans both know history and design the test","an analyst already remembers which campaigns succeeded")],
  ["rule chosen with knowledge of which cases won","retrospective confidence reported as if it were predictive"],
  ["the predictive rule is fixed (ideally pre-registered) before outcomes are revealed to the rule's authors"],
  ["feature-selection protocol changes","a rule is found to have used outcome knowledge"],
  specialists=["quantitative_validation_analyst","decision_psychologist"],
  contradictors=["retrospective_obviousness_advocate"]))

N.append(node("FEATURE_PROVENANCE","point_in_time_feature_provenance",
  "Establish point-in-time feature provenance: every predictor is reconstructed as it would have been known at each case's decision time, with documented source, lag, and as-of timestamp, so the feature set is constructed honestly before any leakage audit can confirm it.",
  "design", ["HINDSIGHT_BIAS","HISTORICAL_GROUND_TRUTH"], ["CQ_06","CQ_07"],
  b(0.84,0.78,0.72,0.7,0.84,0.8,0.62,0.62,0.55, 0.72,0.76, 0.84,0.62,0.24,0.58,[0.28,0.64],"a feature's source, lag, or as-of timing definition changes"),
  ["as-of feature reconstruction","source/lag documentation","point-in-time snapshotting"],
  ["model fitting","threshold tuning"],
  [pro("Reconstructing each feature as-of the decision time builds look-ahead-free predictors by construction, before any audit","follower count is read as of the post date, not the latest value")],
  [con("Point-in-time reconstruction needs historical snapshots that are often unavailable, forcing approximations","only current follower counts survive, so the as-of value must be estimated")],
  ["a feature uses the latest value instead of its as-of value","undocumented lag hides a look-ahead path"],
  ["every predictor has a documented source, lag, and as-of timestamp consistent with the case decision time"],
  ["a feature's source or lag changes","a snapshot gap forces an approximation"],
  specialists=["quantitative_validation_analyst","feature_provenance_engineer"]))

N.append(node("SINGLE_ORIGIN_EVIDENCE","single_origin_evidence_control",
  "Control single-origin (correlated-source) evidence: many cases drawn from one account, campaign, platform, or time slice are not independent observations, so they inflate apparent confidence and effective sample size far beyond the true independent information.",
  "bias_control", ["SAMPLING_FRAME"], ["CQ_06","CQ_07"],
  b(0.86,0.78,0.72,0.68,0.84,0.82,0.6,0.6,0.6, 0.7,0.74, 0.86,0.6,0.24,0.58,[0.3,0.68],"clustering structure or independence assumption changes"),
  ["cluster identification","effective-sample-size estimation","clustered/robust standard errors"],
  ["point-estimate-only reporting","naive iid assumption"],
  [pro("Counting clusters, not rows, and using clustered standard errors keeps confidence honest when cases share an origin","30 hits from one viral creator count as ~1 independent signal, not 30")],
  [con("Defining the right clustering unit is judgment-laden; over-clustering wastes real information","is the cluster the post, the account, the network, or the trend?")],
  ["correlated cases treated as independent","effective sample size overstated by ignoring clustering"],
  ["the clustering unit is declared and effective sample size / clustered standard errors are used"],
  ["a dominant single source is found in the sample","clustering unit is redefined"],
  specialists=["quantitative_validation_analyst","econometric_clustering_specialist"],
  contradictors=["iid_assumption_advocate"]))

N.append(node("TRAIN_TEST_SPLIT","temporal_train_test_partition",
  "Partition the data into fitting and evaluation sets with a temporal cut, so the model is fit only on information available before each test case, preventing any use of future information to predict the past.",
  "design", ["SURVIVORSHIP_BIAS","HINDSIGHT_BIAS","FEATURE_PROVENANCE"], ["CQ_07","CQ_08"],
  b(0.88,0.8,0.74,0.66,0.86,0.82,0.66,0.62,0.55, 0.72,0.78, 0.88,0.62,0.22,0.58,[0.26,0.62],"split policy or temporal cut definition changes"),
  ["temporal cut point","train/test membership rule","no-future-information invariant"],
  ["hyperparameter search internals","deployment"],
  [pro("A strict temporal split enforces that predictions use only past-available information, the minimum bar for an honest backtest","a 2024-06-01 cut fits on pre-June data and tests on later items only")],
  [con("Temporal splits shrink the usable training set and can straddle a regime change that makes train and test incomparable","the cut lands right before a platform algorithm change")],
  ["future information leaks into the training fold","split point chosen to flatter the result"],
  ["no test case uses any feature derived from information dated after that case","the temporal cut is fixed before scoring"],
  ["split policy changes","a leak from future information is detected"],
  specialists=["quantitative_validation_analyst","backtest_engineer"]))

N.append(node("OUT_OF_SAMPLE","out_of_sample_generalization_test",
  "Run the genuine generalization test: evaluate the frozen model on data it never touched during fitting or selection, treating only out-of-sample performance as evidence of real predictive power rather than memorization.",
  "validation", ["TRAIN_TEST_SPLIT"], ["CQ_08","CQ_09"],
  b(0.92,0.84,0.8,0.66,0.88,0.84,0.7,0.6,0.6, 0.7,0.74, 0.92,0.6,0.24,0.62,[0.28,0.66],"out-of-sample protocol or holdout boundary changes"),
  ["frozen-model evaluation","untouched holdout scoring","in-sample-vs-out-of-sample gap reporting"],
  ["model refitting","threshold re-tuning on the holdout"],
  [pro("Out-of-sample performance is the only honest evidence of generalization; in-sample fit is nearly always optimistic","a model with 0.9 in-sample AUC but 0.55 out-of-sample is exposed as overfit")],
  [con("A single holdout can be lucky or unlucky; one out-of-sample number has its own sampling variance","a small holdout gives a noisy point estimate that can mislead either way")],
  ["holdout reused for tuning, silently turning it in-sample","reporting in-sample numbers as if out-of-sample"],
  ["the reported headline metric comes only from data untouched during fit and selection","the in-sample-vs-out-of-sample gap is reported"],
  ["holdout boundary changes","the out-of-sample gap widens beyond tolerance"],
  specialists=["quantitative_validation_analyst","cross_validation_specialist"]))

N.append(node("CROSS_VALIDATION","cross_validation_resampling",
  "Use resampling (k-fold, blocked, or purged time-series cross-validation) to estimate out-of-sample performance with its sampling variability, while respecting temporal order and avoiding leakage across folds.",
  "validation", ["OUT_OF_SAMPLE"], ["CQ_09","CQ_10"],
  b(0.84,0.76,0.72,0.74,0.82,0.78,0.6,0.6,0.55, 0.7,0.74, 0.84,0.6,0.24,0.56,[0.3,0.66],"resampling scheme or fold-leakage policy changes"),
  ["fold construction","purged/embargoed time-series CV","variance-of-estimate reporting"],
  ["single-split-only reporting","leaky random shuffling of time series"],
  [pro("Cross-validation yields a distribution of out-of-sample scores, exposing variance a single holdout hides","purged k-fold gives a mean AUC with a CI rather than one fragile number")],
  [con("Naive k-fold on time series leaks information across folds and over-states performance","random shuffling lets near-future leak into a fold's training set")],
  ["random CV on temporally ordered data","overlapping windows leak label information across folds"],
  ["folds respect temporal order with purging/embargo so no fold's training set sees its test horizon"],
  ["resampling scheme changes","a leakage path between folds is found"],
  specialists=["quantitative_validation_analyst","cross_validation_specialist"]))

N.append(node("OVERFITTING_PAST_VIRALITY","overfitting_to_past_hits_control",
  "Control overfitting to past virality: a model tuned to reproduce the specific past hits captures idiosyncratic noise of those events and fails to generalize, especially when many configurations were tried until one matched history.",
  "validation", ["CROSS_VALIDATION"], ["CQ_10","CQ_11"],
  b(0.9,0.82,0.78,0.74,0.88,0.84,0.68,0.58,0.62, 0.68,0.72, 0.9,0.58,0.26,0.64,[0.3,0.7],"model-complexity or search-budget policy changes"),
  ["complexity penalization","in-sample/out-of-sample degradation tracking","fit-to-noise diagnosis"],
  ["live ranking","feature store internals"],
  [pro("Penalizing complexity and watching in-to-out degradation catches models that memorize specific past hits","a 40-feature model that nails 2023 hits but collapses on 2024 is flagged as overfit")],
  [con("Aggressive regularization can underfit a genuine but subtle signal, trading false discovery for missed discovery","shrinking away a real weak-but-stable predictor")],
  ["model fit to the noise of specific viral events","past hits reproduced by capacity, not by signal"],
  ["out-of-sample performance does not collapse relative to in-sample beyond a stated tolerance"],
  ["model complexity increases","in-to-out degradation exceeds the tolerance"],
  specialists=["quantitative_validation_analyst","overfitting_specialist"],
  contradictors=["fit_the_winners_advocate"]))

N.append(node("MULTIPLE_TESTING_IN_BACKTEST","multiple_testing_false_discovery_control",
  "Control multiple testing in the backtest: when many strategies, features, or thresholds are evaluated, the best-looking one is selected by chance as much as by skill, so significance and performance must be adjusted for the number of trials.",
  "validation", ["CROSS_VALIDATION"], ["CQ_11","CQ_12"],
  b(0.9,0.82,0.76,0.74,0.88,0.82,0.66,0.58,0.62, 0.68,0.72, 0.9,0.58,0.26,0.64,[0.3,0.7],"number of trials or multiple-comparison correction method changes"),
  ["trial counting","family-wise / FDR correction","deflated-performance adjustment"],
  ["single-hypothesis reporting","strategy-store internals"],
  [pro("Counting trials and deflating the best result removes the selection bias of picking the winner from many candidates","testing 100 thresholds, the best Sharpe is deflated for the 100 looks before it is believed")],
  [con("The true number of trials is often unknown (analysts try many things informally), so the correction is a lower bound","undocumented manual experimentation makes the effective trial count larger than recorded")],
  ["best-of-many reported as if it were a single pre-specified test","trial count under-counted, under-correcting significance"],
  ["the number of configurations tried is recorded and the headline result is corrected for it"],
  ["the number of trials grows","an undocumented search history is discovered"],
  specialists=["quantitative_validation_analyst","multiple_comparisons_specialist"],
  contradictors=["best_strategy_cherry_picker"]))

N.append(node("DEFLATED_PERFORMANCE","deflated_performance_metric",
  "Compute a deflated performance metric (e.g. the Deflated Sharpe Ratio) that adjusts the observed in-sample performance for the number of trials, the non-normality of returns, and the sample length, yielding the probability that the result is not a fluke.",
  "validation", ["MULTIPLE_TESTING_IN_BACKTEST","OVERFITTING_PAST_VIRALITY"], ["CQ_12","CQ_13"],
  b(0.86,0.8,0.74,0.78,0.86,0.8,0.66,0.58,0.6, 0.68,0.72, 0.86,0.58,0.26,0.62,[0.32,0.72],"deflation model assumptions or trial-count input changes"),
  ["deflated metric computation","probability-of-skill estimate","sample-length adjustment"],
  ["raw-metric reporting","live PnL attribution"],
  [pro("A deflated metric converts a raw score into a fluke-adjusted probability of genuine skill, directly per Bailey & Lopez de Prado","a raw Sharpe of 2.0 across 50 trials deflates to a modest probability of true skill")],
  [con("Deflation needs an estimate of the effective number of trials and return moments, which are themselves uncertain","mis-specifying the trial count or skew/kurtosis biases the deflated estimate")],
  ["raw metric believed without deflation","deflation fed a wrong trial count and over-credits the result"],
  ["the reported performance is deflated for trials, sample length, and non-normality before any go/no-go decision"],
  ["deflation assumptions change","the effective trial-count input is revised"],
  specialists=["quantitative_validation_analyst","deflated_sharpe_specialist"]))

N.append(node("STATISTICAL_SIGNIFICANCE","significance_and_confidence_intervals",
  "Attach uncertainty to every backtest estimate: report confidence intervals (preferably from the clustered, deflated, resampled distribution) rather than point estimates, so a result is judged by its interval, not a single flattering number.",
  "inference", ["BASE_RATE_CALIBRATION","SINGLE_ORIGIN_EVIDENCE","DEFLATED_PERFORMANCE"], ["CQ_13","CQ_14"],
  b(0.88,0.82,0.78,0.7,0.86,0.84,0.66,0.6,0.58, 0.7,0.74, 0.88,0.6,0.24,0.6,[0.28,0.64],"interval method or confidence level changes"),
  ["confidence-interval estimation","bootstrap/clustered intervals","point-vs-interval reporting"],
  ["raw point-estimate reporting","model internals"],
  [pro("Reporting an interval makes the precision of the claim explicit and prevents over-reading a noisy point estimate","lift = 1.4x with 95% CI [0.9x, 2.1x] tells a very different story than '1.4x'")],
  [con("Intervals assume a noise model; the wrong model (e.g. ignoring clustering) gives falsely tight bounds","an iid bootstrap on clustered data yields a CI that is too narrow")],
  ["point estimate reported without any interval","CI computed under an iid assumption on clustered data"],
  ["every headline estimate is reported with a confidence interval built on the clustered, deflated distribution"],
  ["interval method changes","a CI is found to ignore the clustering structure"],
  specialists=["quantitative_validation_analyst","inferential_statistician"],
  contradictors=["point_estimate_only_advocate"]))

N.append(node("MEDIUM_TIER_AS_FALSIFIABLE_BET","medium_tier_opportunity_as_tested_hypothesis",
  "Treat the scope finding 'medium-tier is the opportunity' (scope FP1) as a falsifiable bet, not a premise: state it as a hypothesis with a directional effect, test it out-of-sample against the medium-tier subpopulation, and report the result as an effect size with a confidence interval that can include zero or a negative value.",
  "inference", ["OUT_OF_SAMPLE","BASE_RATE_CALIBRATION","STATISTICAL_SIGNIFICANCE"], ["CQ_14"],
  b(0.9,0.86,0.84,0.7,0.88,0.85,0.7,0.58,0.62, 0.68,0.72, 0.9,0.58,0.26,0.66,[0.3,0.7],"the medium-tier hypothesis definition or its tier boundary changes"),
  ["hypothesis statement of FP1","medium-tier subpopulation test","effect size with CI that can refute"],
  ["assuming FP1 is true a priori","strategy deployment"],
  [pro("Framing FP1 as a bet with a refutable CI prevents a hunch from hardening into an unexamined premise that drives strategy","FP1 tested: medium-tier lift = 1.15x, 95% CI [0.95x, 1.38x] -> not yet established, do not bet on it")],
  [con("A medium-tier definition is itself a choice; a different tier boundary can flip the verdict, so the boundary must be pre-stated","defining medium-tier as 10k-100k vs 5k-200k followers changes the estimated effect")],
  ["FP1 assumed true and never tested","tier boundary tuned until FP1 looks confirmed"],
  ["FP1 is written as a hypothesis with a pre-stated tier boundary and tested out-of-sample with a CI that could include zero or negative"],
  ["the medium-tier boundary is redefined","FP1's tested CI crosses or excludes the null"],
  specialists=["quantitative_validation_analyst","hypothesis_design_specialist"],
  contradictors=["medium_tier_premise_believer"]))

N.append(node("REGIME_CHANGE","regime_change_and_nonstationarity",
  "Account for regime change and non-stationarity: attention and platform dynamics shift over time, so a relationship that held in the past need not hold in the future; test for structural breaks and report the regime the result is valid within.",
  "robustness", ["OUT_OF_SAMPLE"], ["CQ_10","CQ_14"],
  b(0.86,0.8,0.78,0.72,0.86,0.82,0.66,0.58,0.6, 0.68,0.72, 0.86,0.58,0.26,0.62,[0.3,0.68],"a structural break is detected or the regime window changes"),
  ["structural-break testing","regime-window labeling","stability-across-regimes check"],
  ["model architecture","live serving"],
  [pro("Testing for structural breaks scopes the claim to the regime it was learned in and warns when past != future","a 2022 model's edge vanishes after a 2023 ranking-algorithm change, flagged by a break test")],
  [con("Regimes are only visible in hindsight; a break detected late still leaves a window of invalid predictions","the algorithm change is only identifiable months after it shifts the dynamics")],
  ["a past relationship assumed stationary into the future","regime boundary ignored so cross-regime data are pooled"],
  ["the result is labeled with the regime it is valid within and a structural-break test is run before extrapolating"],
  ["a structural break is detected","the deployment regime diverges from the test regime"],
  specialists=["quantitative_validation_analyst","time_series_econometrician"]))

N.append(node("LEAKAGE_AUDIT","information_leakage_audit",
  "Audit for information leakage end to end: any path by which outcome-correlated or future information reaches the model at fit or selection time (target leakage, look-ahead features, test contamination), which silently inflates every downstream metric.",
  "robustness", ["TRAIN_TEST_SPLIT","CROSS_VALIDATION","FEATURE_PROVENANCE"], ["CQ_09","CQ_11"],
  b(0.9,0.82,0.76,0.74,0.9,0.84,0.7,0.58,0.62, 0.66,0.72, 0.9,0.58,0.26,0.66,[0.32,0.72],"feature provenance or a new data join introduces a leakage path"),
  ["target-leakage detection","look-ahead feature audit","train/test contamination check"],
  ["model training internals","deployment monitoring"],
  [pro("A systematic leakage audit catches the single most common cause of unbelievably good backtests","a feature computed using the full-period mean leaks the future into every row, caught by the audit")],
  [con("Leakage can hide in joins, derived features, and preprocessing fit on all data, so audits are never provably complete","scaler fit on train+test together leaks test distribution into training")],
  ["a feature that encodes the outcome or its future","preprocessing statistics fit on the full dataset"],
  ["no feature uses information unavailable at the case's decision time and no preprocessing is fit across the split"],
  ["a new feature or data join is added","a too-good-to-be-true metric appears"],
  specialists=["quantitative_validation_analyst","leakage_auditor"]))

N.append(node("ROBUSTNESS_REPLICATION","robustness_and_replication",
  "Test robustness and (where possible) replicate: vary thresholds, sub-periods, sources, and definitions to confirm the result is not an artifact of one arbitrary choice, and report whether the finding survives reasonable perturbations.",
  "robustness", ["OVERFITTING_PAST_VIRALITY","REGIME_CHANGE","LEAKAGE_AUDIT"], ["CQ_11","CQ_14"],
  b(0.84,0.78,0.74,0.7,0.84,0.8,0.62,0.58,0.58, 0.7,0.74, 0.84,0.58,0.24,0.58,[0.3,0.66],"the perturbation set or replication source changes"),
  ["sensitivity-to-choices sweep","sub-period replication","independent-source replication"],
  ["single-configuration reporting","live serving"],
  [pro("A finding that survives threshold, period, and source perturbations is far more trustworthy than a single point result","the medium-tier effect holds across three definitions and two independent creator cohorts")],
  [con("Perturbation testing is itself multiple testing; reporting only the configurations that survive re-introduces selection bias","quietly dropping perturbations that broke the result")],
  ["result reported from one lucky configuration","surviving-perturbations-only reported, hiding the failures"],
  ["the result is shown to survive a pre-stated set of perturbations, and the perturbations that broke it are reported too"],
  ["the perturbation set changes","a perturbation breaks the result"],
  specialists=["quantitative_validation_analyst","replication_specialist"]))

N.append(node("VALIDITY_VERDICT","backtest_validity_verdict_and_decision",
  "Produce the final validity verdict: synthesize base-rate lift, bias controls, deflated out-of-sample performance, and robustness into a go / hold / no-go decision with its confidence interval, an explicit scope of validity, and the conditions that would falsify it.",
  "decision", ["STATISTICAL_SIGNIFICANCE","MEDIUM_TIER_AS_FALSIFIABLE_BET","ROBUSTNESS_REPLICATION"], ["CQ_13","CQ_14"],
  b(0.94,0.88,0.86,0.7,0.92,0.86,0.78,0.6,0.6, 0.7,0.74, 0.94,0.6,0.22,0.68,[0.26,0.6],"the decision threshold or scope-of-validity policy changes"),
  ["go/hold/no-go verdict","scope-of-validity statement","falsification conditions"],
  ["live deployment execution","model retraining loop"],
  [pro("A verdict that carries its CI, scope, and falsifiers turns the backtest into an actionable, accountable decision rather than a number","verdict: hold; medium-tier edge not yet established (CI crosses null); revisit after one more regime")],
  [con("A single synthesized verdict can over-compress the underlying uncertainty if the caveats are dropped downstream","decision-makers read 'hold' and forget the CI that justified it")],
  ["verdict issued without its scope or falsification conditions","caveats stripped when the verdict is communicated"],
  ["the verdict states its confidence interval, scope of validity, and the conditions that would overturn it"],
  ["decision threshold changes","a falsification condition is met in production"],
  specialists=["quantitative_validation_analyst","decision_owner"]))

# ---- competency questions (14; <=14 enforced) ----
CQ = [
 ("CQ_01","How is the backtest outcome defined and grounded in real known historical results?",["nodes"],"OUTCOME_DEFINITION freezes the label rule and HISTORICAL_GROUND_TRUTH supplies real realized outcomes",["OUTCOME_DEFINITION","HISTORICAL_GROUND_TRUTH"]),
 ("CQ_02","What population does the backtest claim to generalize to, and how is the sample frame defined?",["nodes"],"SAMPLING_FRAME defines the population and inclusion rule against the known-outcome ledger",["SAMPLING_FRAME","HISTORICAL_GROUND_TRUTH"]),
 ("CQ_03","How is survivorship bias detected and corrected so success rates are computed over entrants, not survivors?",["nodes","edge_cases"],"SURVIVORSHIP_BIAS reconstructs the entry cohort within the declared frame",["SURVIVORSHIP_BIAS","SAMPLING_FRAME"]),
 ("CQ_04","How is performance calibrated against the outcome base rate rather than raw accuracy?",["nodes"],"BASE_RATE_CALIBRATION reports lift over the base rate after survivorship correction",["BASE_RATE_CALIBRATION","SURVIVORSHIP_BIAS"]),
 ("CQ_05","How are hindsight bias and base-rate neglect prevented when choosing features and rules?",["nodes"],"HINDSIGHT_BIAS blinds rule selection to outcomes; BASE_RATE_CALIBRATION removes base-rate neglect",["HINDSIGHT_BIAS","BASE_RATE_CALIBRATION"]),
 ("CQ_06","How is single-origin / correlated evidence prevented from inflating confidence?",["nodes","conflict_axes"],"SINGLE_ORIGIN_EVIDENCE counts clusters not rows; HINDSIGHT_BIAS keeps selection outcome-blind",["SINGLE_ORIGIN_EVIDENCE","HINDSIGHT_BIAS"]),
 ("CQ_07","How is the data split temporally so no future information predicts the past?",["nodes"],"TRAIN_TEST_SPLIT enforces a temporal cut; SINGLE_ORIGIN_EVIDENCE keeps clusters out of both folds",["TRAIN_TEST_SPLIT","SINGLE_ORIGIN_EVIDENCE"]),
 ("CQ_08","How is genuine out-of-sample generalization established rather than in-sample fit?",["nodes","workflow"],"OUT_OF_SAMPLE evaluates the frozen model on untouched data after the TRAIN_TEST_SPLIT",["OUT_OF_SAMPLE","TRAIN_TEST_SPLIT"]),
 ("CQ_09","How is out-of-sample performance estimated with its variability and without fold leakage?",["nodes"],"CROSS_VALIDATION resamples with purging; LEAKAGE_AUDIT verifies no information crosses the split",["CROSS_VALIDATION","OUT_OF_SAMPLE","LEAKAGE_AUDIT"]),
 ("CQ_10","How is overfitting to past virality and regime change controlled?",["nodes","edge_cases"],"OVERFITTING_PAST_VIRALITY tracks in-to-out degradation; REGIME_CHANGE tests structural breaks",["OVERFITTING_PAST_VIRALITY","REGIME_CHANGE"]),
 ("CQ_11","How is multiple testing across many strategies/features controlled, including via robustness checks?",["nodes"],"MULTIPLE_TESTING_IN_BACKTEST corrects for trials; ROBUSTNESS_REPLICATION and LEAKAGE_AUDIT guard against artifacts",["MULTIPLE_TESTING_IN_BACKTEST","ROBUSTNESS_REPLICATION","LEAKAGE_AUDIT"]),
 ("CQ_12","How is observed performance deflated for the number of trials and sample properties?",["nodes"],"DEFLATED_PERFORMANCE computes a fluke-adjusted metric from the trial count",["DEFLATED_PERFORMANCE","MULTIPLE_TESTING_IN_BACKTEST"]),
 ("CQ_13","How is every estimate reported with a confidence interval that drives the final verdict?",["nodes","workflow"],"STATISTICAL_SIGNIFICANCE attaches CIs; VALIDITY_VERDICT carries them into the decision",["STATISTICAL_SIGNIFICANCE","DEFLATED_PERFORMANCE","VALIDITY_VERDICT"]),
 ("CQ_14","How is 'medium-tier is the opportunity' (FP1) treated as a falsifiable bet measured with a CI rather than a premise?",["nodes","conflict_axes"],"MEDIUM_TIER_AS_FALSIFIABLE_BET tests FP1 out-of-sample with a CI that can refute; VALIDITY_VERDICT records the verdict",["MEDIUM_TIER_AS_FALSIFIABLE_BET","VALIDITY_VERDICT","REGIME_CHANGE","ROBUSTNESS_REPLICATION"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

# consolidate node->CQ references (every node refs 1-2 of the 14 CQs)
CQ_MAP = {
 "OUTCOME_DEFINITION":["CQ_01"], "HISTORICAL_GROUND_TRUTH":["CQ_01","CQ_02"], "SAMPLING_FRAME":["CQ_02","CQ_03"],
 "SURVIVORSHIP_BIAS":["CQ_03","CQ_04"], "BASE_RATE_CALIBRATION":["CQ_04","CQ_05"], "HINDSIGHT_BIAS":["CQ_05","CQ_06"],
 "FEATURE_PROVENANCE":["CQ_06","CQ_07"],
 "SINGLE_ORIGIN_EVIDENCE":["CQ_06","CQ_07"], "TRAIN_TEST_SPLIT":["CQ_07","CQ_08"], "OUT_OF_SAMPLE":["CQ_08","CQ_09"],
 "CROSS_VALIDATION":["CQ_09","CQ_10"], "OVERFITTING_PAST_VIRALITY":["CQ_10","CQ_11"], "MULTIPLE_TESTING_IN_BACKTEST":["CQ_11","CQ_12"],
 "DEFLATED_PERFORMANCE":["CQ_12","CQ_13"], "STATISTICAL_SIGNIFICANCE":["CQ_13","CQ_14"], "MEDIUM_TIER_AS_FALSIFIABLE_BET":["CQ_14"],
 "REGIME_CHANGE":["CQ_10","CQ_14"], "LEAKAGE_AUDIT":["CQ_09","CQ_11"], "ROBUSTNESS_REPLICATION":["CQ_11","CQ_14"],
 "VALIDITY_VERDICT":["CQ_13","CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

GL = [
 ("outcome_backtest","a test that scores predictions against real, already-known historical outcomes rather than a simulated proxy",["retrospective_outcome_test"],["forward_paper_trade"],["OUTCOME_DEFINITION","HISTORICAL_GROUND_TRUTH"]),
 ("base_rate","the unconditional prevalence of the outcome in the population, used as the no-skill reference for lift",["prevalence","prior_probability"],["raw_accuracy"],["BASE_RATE_CALIBRATION","STATISTICAL_SIGNIFICANCE"]),
 ("survivorship_bias","over-representation of winners because failures, deletions, and dead accounts are absent from the record",["survival_bias"],["selection_into_the_frame"],["SURVIVORSHIP_BIAS","SAMPLING_FRAME"]),
 ("hindsight_bias","the post-outcome tendency to view an event as having been predictable, inflating retrospective confidence",["knew_it_all_along_effect","creeping_determinism"],["genuine_foresight"],["HINDSIGHT_BIAS","OUT_OF_SAMPLE"]),
 ("single_origin_evidence","multiple cases drawn from one correlated source that are not independent observations",["correlated_evidence","clustered_data"],["independent_replications"],["SINGLE_ORIGIN_EVIDENCE","STATISTICAL_SIGNIFICANCE"]),
 ("out_of_sample","data untouched during fitting and selection, used as the only honest evidence of generalization",["holdout","oos"],["in_sample_fit"],["OUT_OF_SAMPLE","CROSS_VALIDATION"]),
 ("backtest_overfitting","selecting a configuration that matches past data by chance after many trials, which fails to generalize",["selection_under_multiple_testing"],["genuine_predictive_signal"],["OVERFITTING_PAST_VIRALITY","MULTIPLE_TESTING_IN_BACKTEST"]),
 ("deflated_performance","a performance metric adjusted for trials, sample length, and non-normality to estimate the probability it is not a fluke",["deflated_sharpe_ratio"],["raw_sharpe_ratio"],["DEFLATED_PERFORMANCE","STATISTICAL_SIGNIFICANCE"]),
 ("regime_change","a structural break after which past relationships no longer hold, violating stationarity",["structural_break","nonstationarity"],["sampling_noise"],["REGIME_CHANGE","ROBUSTNESS_REPLICATION"]),
 ("information_leakage","any path by which outcome-correlated or future information reaches the model at fit or selection time",["target_leakage","look_ahead_bias"],["legitimate_past_feature"],["LEAKAGE_AUDIT","TRAIN_TEST_SPLIT"]),
 ("falsifiable_bet","a scope finding stated as a refutable hypothesis with an effect size and CI that can include the null or a negative",["testable_hypothesis"],["unexamined_premise"],["MEDIUM_TIER_AS_FALSIFIABLE_BET","VALIDITY_VERDICT"]),
]
GLS=[{"term":t,"definition":d,"synonyms":s,"not_same_as":ns,"used_by_nodes":u} for (t,d,s,ns,u) in GL]

# ---- edges ----
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
              "risk_of_conflict":"unmanaged tension degrades backtest validity","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated behavior",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies) -- 23 edges
dep("OUTCOME_DEFINITION","HISTORICAL_GROUND_TRUTH",0.9)
dep("HISTORICAL_GROUND_TRUTH","SAMPLING_FRAME",0.84)
dep("SAMPLING_FRAME","SURVIVORSHIP_BIAS",0.86)
dep("SURVIVORSHIP_BIAS","BASE_RATE_CALIBRATION",0.84)
dep("HISTORICAL_GROUND_TRUTH","HINDSIGHT_BIAS",0.8)
dep("SAMPLING_FRAME","SINGLE_ORIGIN_EVIDENCE",0.8)
dep("SURVIVORSHIP_BIAS","TRAIN_TEST_SPLIT",0.8)
dep("HINDSIGHT_BIAS","TRAIN_TEST_SPLIT",0.78)
dep("HINDSIGHT_BIAS","FEATURE_PROVENANCE",0.8)
dep("FEATURE_PROVENANCE","TRAIN_TEST_SPLIT",0.82)
dep("FEATURE_PROVENANCE","LEAKAGE_AUDIT",0.82)
dep("TRAIN_TEST_SPLIT","OUT_OF_SAMPLE",0.88)
dep("OUT_OF_SAMPLE","CROSS_VALIDATION",0.84)
dep("CROSS_VALIDATION","OVERFITTING_PAST_VIRALITY",0.84)
dep("CROSS_VALIDATION","MULTIPLE_TESTING_IN_BACKTEST",0.82)
dep("MULTIPLE_TESTING_IN_BACKTEST","DEFLATED_PERFORMANCE",0.84)
dep("OVERFITTING_PAST_VIRALITY","DEFLATED_PERFORMANCE",0.8)
dep("BASE_RATE_CALIBRATION","STATISTICAL_SIGNIFICANCE",0.82)
dep("SINGLE_ORIGIN_EVIDENCE","STATISTICAL_SIGNIFICANCE",0.8)
dep("DEFLATED_PERFORMANCE","STATISTICAL_SIGNIFICANCE",0.82)
dep("OUT_OF_SAMPLE","MEDIUM_TIER_AS_FALSIFIABLE_BET",0.82)
dep("BASE_RATE_CALIBRATION","MEDIUM_TIER_AS_FALSIFIABLE_BET",0.78)
dep("STATISTICAL_SIGNIFICANCE","MEDIUM_TIER_AS_FALSIFIABLE_BET",0.84)
dep("OUT_OF_SAMPLE","REGIME_CHANGE",0.78)
dep("TRAIN_TEST_SPLIT","LEAKAGE_AUDIT",0.84)
dep("CROSS_VALIDATION","LEAKAGE_AUDIT",0.8)
dep("OVERFITTING_PAST_VIRALITY","ROBUSTNESS_REPLICATION",0.78)
dep("REGIME_CHANGE","ROBUSTNESS_REPLICATION",0.78)
dep("LEAKAGE_AUDIT","ROBUSTNESS_REPLICATION",0.78)
dep("STATISTICAL_SIGNIFICANCE","VALIDITY_VERDICT",0.86)
dep("MEDIUM_TIER_AS_FALSIFIABLE_BET","VALIDITY_VERDICT",0.84)
dep("ROBUSTNESS_REPLICATION","VALIDITY_VERDICT",0.8)
# cross-cutting non-dependency edges (4)
rel("LEAKAGE_AUDIT","OVERFITTING_PAST_VIRALITY","causal",0.78,why="undetected leakage manifests as apparent overfitting / too-good out-of-sample scores")
rel("SINGLE_ORIGIN_EVIDENCE","CROSS_VALIDATION","constraint",0.76,why="correlated single-origin clusters must be kept within a single fold to avoid leakage across folds")
rel("REGIME_CHANGE","DEFLATED_PERFORMANCE","feedback",0.74,why="a regime break shortens the effective stationary sample length used in deflation")
# conflict edges (negative signed_tension + resolution_rule) -- 4
conf("MEDIUM_TIER_AS_FALSIFIABLE_BET","BASE_RATE_CALIBRATION",0.7,-0.55,
  "FP1 is credited only as lift over the medium-tier base rate; if the medium-tier base rate already explains the apparent edge, treat FP1 as unconfirmed rather than a real opportunity",
  "the belief that medium-tier is the opportunity conflicts with calibrating against the medium-tier base rate, which may absorb the apparent effect")
conf("OUT_OF_SAMPLE","HISTORICAL_GROUND_TRUTH",0.68,-0.5,
  "spend known historical data on a strict untouched holdout even though it shrinks the fitting set; never recycle the holdout to recover sample size",
  "limited real known outcomes create tension between maximizing the fitting sample and reserving an honest out-of-sample holdout")
conf("MULTIPLE_TESTING_IN_BACKTEST","ROBUSTNESS_REPLICATION",0.68,-0.45,
  "pre-register the robustness perturbation set and count it in the trial budget; do not add perturbations until one survives and then report only that one",
  "robustness perturbations improve trust but also multiply the number of trials, worsening false-discovery risk if uncounted")
conf("OVERFITTING_PAST_VIRALITY","CROSS_VALIDATION",0.66,-0.45,
  "tune complexity using only the inner cross-validation folds and judge it on the outer untouched folds; never select complexity on the same data used to report performance",
  "fitting a model expressive enough to capture past virality conflicts with cross-validated generalization, which penalizes that expressiveness")

CA=[
 {"name":"raw_accuracy_vs_base_rate_lift","description":"Raw accuracy looks impressive against a skewed outcome but can encode pure base-rate exploitation; lift over the base rate is the informative but harsher metric.","poles":["raw_accuracy","base_rate_lift"],"resolution_hint":"always pair accuracy with the base rate and a no-skill baseline; judge on lift","tension_score":0.74,"affected_nodes":["BASE_RATE_CALIBRATION","STATISTICAL_SIGNIFICANCE","OUTCOME_DEFINITION"]},
 {"name":"survivor_only_vs_full_cohort","description":"Survivor-only data is easy to obtain but biases success rates upward; reconstructing the full entry cohort is hard but honest.","poles":["survivor_only","full_entry_cohort"],"resolution_hint":"reconstruct entrants or bound the missing-failure mass before reporting any rate","tension_score":0.78,"affected_nodes":["SURVIVORSHIP_BIAS","SAMPLING_FRAME","BASE_RATE_CALIBRATION"]},
 {"name":"in_sample_fit_vs_out_of_sample_truth","description":"Maximizing in-sample fit reproduces history but overstates skill; out-of-sample performance is lower but is the only honest evidence.","poles":["in_sample_fit","out_of_sample_truth"],"resolution_hint":"report only out-of-sample headline metrics and the in-to-out gap","tension_score":0.76,"affected_nodes":["OUT_OF_SAMPLE","OVERFITTING_PAST_VIRALITY","CROSS_VALIDATION"]},
 {"name":"search_breadth_vs_false_discovery","description":"Trying many strategies raises the chance of finding a real edge but also the chance the best one is a fluke; correction shrinks the apparent winner.","poles":["broad_search","controlled_false_discovery"],"resolution_hint":"count every trial and deflate the best result for the number of looks","tension_score":0.75,"affected_nodes":["MULTIPLE_TESTING_IN_BACKTEST","DEFLATED_PERFORMANCE","ROBUSTNESS_REPLICATION"]},
 {"name":"sample_size_vs_honest_holdout","description":"Reserving an untouched holdout reduces the fitting sample, especially scarce when real known outcomes are limited.","poles":["max_fitting_sample","reserved_holdout"],"resolution_hint":"reserve the holdout first; use cross-validation to recover statistical efficiency without recycling it","tension_score":0.66,"affected_nodes":["OUT_OF_SAMPLE","HISTORICAL_GROUND_TRUTH","CROSS_VALIDATION"]},
 {"name":"stationarity_assumption_vs_regime_awareness","description":"Pooling all history maximizes data but assumes stationarity; respecting regimes scopes the claim but shrinks usable data.","poles":["pool_all_history","scope_to_regime"],"resolution_hint":"test for structural breaks and label the regime the result is valid within","tension_score":0.7,"affected_nodes":["REGIME_CHANGE","OUT_OF_SAMPLE","ROBUSTNESS_REPLICATION"]},
 {"name":"clustered_evidence_vs_apparent_sample_size","description":"Treating correlated single-origin cases as independent inflates the effective sample; clustering is honest but reduces apparent power.","poles":["naive_iid_count","clustered_effective_n"],"resolution_hint":"count clusters not rows and use clustered standard errors","tension_score":0.68,"affected_nodes":["SINGLE_ORIGIN_EVIDENCE","STATISTICAL_SIGNIFICANCE","CROSS_VALIDATION"]},
 {"name":"premise_belief_vs_falsifiable_test","description":"Treating 'medium-tier is the opportunity' as a premise speeds strategy but risks building on an untested belief; testing it as a bet may refute it.","poles":["assume_fp1_true","test_fp1_as_bet"],"resolution_hint":"state FP1 as a hypothesis with a pre-stated tier boundary and a CI that can include the null","tension_score":0.72,"affected_nodes":["MEDIUM_TIER_AS_FALSIFIABLE_BET","BASE_RATE_CALIBRATION","VALIDITY_VERDICT"]},
 {"name":"retrospective_confidence_vs_blinded_selection","description":"Designing the rule with knowledge of which cases won feels efficient but injects hindsight bias; outcome-blind selection is slower but valid.","poles":["outcome_aware_design","outcome_blind_design"],"resolution_hint":"fix or pre-register the rule before outcomes are revealed to its authors","tension_score":0.67,"affected_nodes":["HINDSIGHT_BIAS","TRAIN_TEST_SPLIT","LEAKAGE_AUDIT"]},
]

EC=[
 {"description":"A model is fit only on surviving viral hits, so its success rate is computed over winners and looks far stronger than it is.","trigger":"failures, deletions, and dead accounts are absent from the ledger","affected_nodes":["SURVIVORSHIP_BIAS","SAMPLING_FRAME","BASE_RATE_CALIBRATION"],"mitigation":"reconstruct the entry cohort or bound the missing-failure mass; report rates over entrants","severity":"critical"},
 {"description":"A classifier reports 97% accuracy on a 3% base-rate outcome while adding no information over always predicting the majority class.","trigger":"raw accuracy reported against a highly skewed base rate","affected_nodes":["BASE_RATE_CALIBRATION","OUTCOME_DEFINITION"],"mitigation":"report lift over the base rate and a no-skill majority-class baseline","severity":"high"},
 {"description":"Features and rules are chosen with knowledge of which past items went viral, so the past looks obviously predictable.","trigger":"rule selection is not blinded to outcomes","affected_nodes":["HINDSIGHT_BIAS","TRAIN_TEST_SPLIT"],"mitigation":"fix or pre-register the rule before outcomes are revealed to its authors","severity":"high"},
 {"description":"Thirty wins from a single creator are counted as thirty independent successes, inflating confidence many-fold.","trigger":"correlated single-origin cases treated as iid","affected_nodes":["SINGLE_ORIGIN_EVIDENCE","STATISTICAL_SIGNIFICANCE"],"mitigation":"count clusters not rows; use clustered standard errors and effective sample size","severity":"high"},
 {"description":"A preprocessing scaler is fit on train and test together, leaking the test distribution into training and inflating every metric.","trigger":"preprocessing statistics fit across the train/test split","affected_nodes":["LEAKAGE_AUDIT","TRAIN_TEST_SPLIT","CROSS_VALIDATION"],"mitigation":"fit all preprocessing inside the training fold only and audit feature provenance","severity":"critical"},
 {"description":"K-fold cross-validation is run with random shuffling on a time series, letting near-future leak into each fold's training set.","trigger":"random CV on temporally ordered data","affected_nodes":["CROSS_VALIDATION","OUT_OF_SAMPLE"],"mitigation":"use purged/embargoed time-series cross-validation that respects temporal order","severity":"high"},
 {"description":"One hundred thresholds are tested and the best Sharpe is reported as if it were a single pre-specified hypothesis.","trigger":"best-of-many selected without counting trials","affected_nodes":["MULTIPLE_TESTING_IN_BACKTEST","DEFLATED_PERFORMANCE"],"mitigation":"record the trial count and deflate the headline result for the number of looks","severity":"high"},
 {"description":"A 2022 model's edge silently vanishes after a 2023 ranking-algorithm change, but it is extrapolated forward anyway.","trigger":"a structural break is not tested before extrapolation","affected_nodes":["REGIME_CHANGE","OUT_OF_SAMPLE","ROBUSTNESS_REPLICATION"],"mitigation":"run a structural-break test and label the regime the result is valid within","severity":"high"},
 {"description":"A raw Sharpe of 2.0 across fifty trials is believed without deflation, when the fluke-adjusted probability of skill is low.","trigger":"raw performance metric reported without deflation","affected_nodes":["DEFLATED_PERFORMANCE","MULTIPLE_TESTING_IN_BACKTEST"],"mitigation":"compute a deflated metric adjusting for trials, sample length, and non-normality","severity":"medium"},
 {"description":"The medium-tier-opportunity finding (FP1) is assumed true and drives strategy without ever being tested out-of-sample.","trigger":"FP1 treated as a premise rather than a hypothesis","affected_nodes":["MEDIUM_TIER_AS_FALSIFIABLE_BET","VALIDITY_VERDICT","BASE_RATE_CALIBRATION"],"mitigation":"state FP1 as a bet with a pre-stated tier boundary and test it with a CI that can include the null","severity":"high"},
 {"description":"Only the robustness perturbations that survived are reported, hiding the configurations that broke the result.","trigger":"surviving-perturbations-only reporting under multiple testing","affected_nodes":["ROBUSTNESS_REPLICATION","MULTIPLE_TESTING_IN_BACKTEST"],"mitigation":"pre-register the perturbation set, count it as trials, and report failures too","severity":"medium"},
 {"description":"The final verdict is communicated as 'go' while the confidence interval that crossed the null is dropped downstream.","trigger":"caveats and CI stripped from the verdict when communicated","affected_nodes":["VALIDITY_VERDICT","STATISTICAL_SIGNIFICANCE"],"mitigation":"bind the CI, scope of validity, and falsification conditions to the verdict itself","severity":"medium"},
]

WF=[
 {"action":"define_outcome","node_ref":"OUTCOME_DEFINITION","description":"Freeze the outcome label rule, threshold, and measurement window before scoring any case.","artifact":"frozen_outcome_definition","gate":"label rule fixed and positive class adequately populated"},
 {"action":"assemble_ground_truth","node_ref":"HISTORICAL_GROUND_TRUTH","description":"Build the ledger of real realized outcomes with provenance, flagging imputed values.","artifact":"realized_outcome_ledger","gate":"every scored case has a real provenanced outcome"},
 {"action":"define_frame_and_correct_survivorship","node_ref":"SURVIVORSHIP_BIAS","description":"Declare the population/inclusion frame and reconstruct or bound the missing failures.","artifact":"entrant_cohort_with_bias_bound","gate":"rates reported over entrants, not survivors"},
 {"action":"calibrate_to_base_rate","node_ref":"BASE_RATE_CALIBRATION","description":"Estimate the base rate and report lift and a no-skill baseline alongside accuracy.","artifact":"calibration_report","gate":"every accuracy claim paired with base rate and lift"},
 {"action":"blind_selection_and_control_clustering","node_ref":"HINDSIGHT_BIAS","description":"Fix/pre-register the rule outcome-blind and declare the clustering unit for evidence.","artifact":"preregistered_rule_and_clustering","gate":"rule fixed before outcomes revealed; clusters declared"},
 {"action":"split_temporally","node_ref":"TRAIN_TEST_SPLIT","description":"Apply a temporal cut so no test case uses future information.","artifact":"temporal_split_spec","gate":"no future information in any training fold"},
 {"action":"evaluate_out_of_sample","node_ref":"OUT_OF_SAMPLE","description":"Score the frozen model only on untouched data and report the in-to-out gap.","artifact":"out_of_sample_scorecard","gate":"headline metric from untouched data only"},
 {"action":"resample_and_audit_leakage","node_ref":"CROSS_VALIDATION","description":"Run purged/embargoed cross-validation and audit for leakage across folds and features.","artifact":"resampled_distribution_with_leakage_audit","gate":"folds respect time and no leakage path remains"},
 {"action":"correct_overfitting_and_trials","node_ref":"MULTIPLE_TESTING_IN_BACKTEST","description":"Count trials, track in-to-out degradation, and correct for multiple testing.","artifact":"trial_corrected_results","gate":"trial count recorded and applied"},
 {"action":"deflate_performance","node_ref":"DEFLATED_PERFORMANCE","description":"Compute the deflated metric adjusting for trials, sample length, and non-normality.","artifact":"deflated_performance_report","gate":"performance deflated before any go decision"},
 {"action":"interval_and_robustness","node_ref":"STATISTICAL_SIGNIFICANCE","description":"Attach clustered/deflated confidence intervals and confirm robustness across perturbations and regimes.","artifact":"interval_and_robustness_report","gate":"every estimate has a CI; result survives pre-stated perturbations"},
 {"action":"test_fp1_and_decide","node_ref":"VALIDITY_VERDICT","description":"Test FP1 as a falsifiable bet and issue a go/hold/no-go verdict with CI, scope, and falsifiers.","artifact":"validity_verdict","gate":"verdict carries CI, scope of validity, and falsification conditions"},
]

DR=[
 {"rule":"OUTCOME_DEFINITION must be frozen before any case is scored","rationale":"a movable outcome lets wins be relabeled after the fact and destroys falsifiability","trigger":"scoring begins with an unfrozen outcome rule","action":"block scoring until the label rule and window are fixed"},
 {"rule":"SURVIVORSHIP_BIAS correction must precede BASE_RATE_CALIBRATION","rationale":"a base rate computed over survivors is biased upward and corrupts every lift figure","trigger":"base rate computed before the entry cohort is reconstructed or bounded","action":"recompute rates over entrants before any calibration"},
 {"rule":"BASE_RATE_CALIBRATION must accompany every raw-accuracy claim","rationale":"raw accuracy on a skewed outcome hides base-rate exploitation","trigger":"accuracy reported without a base rate and lift","action":"reject the report and require lift over a no-skill baseline"},
 {"rule":"HINDSIGHT_BIAS blinding must fix the rule before outcomes are revealed to its authors","rationale":"outcome-aware rule selection injects creeping determinism","trigger":"a rule is chosen with knowledge of which cases won","action":"discard the rule and re-derive it outcome-blind or pre-registered"},
 {"rule":"TRAIN_TEST_SPLIT and LEAKAGE_AUDIT must pass before OUT_OF_SAMPLE results are believed","rationale":"future information or leakage makes out-of-sample numbers fictitious","trigger":"an out-of-sample metric is reported before the leakage audit clears","action":"block the result until no future-information path remains"},
 {"rule":"MULTIPLE_TESTING_IN_BACKTEST trial count must be applied before DEFLATED_PERFORMANCE is reported","rationale":"the best-of-many result is a fluke unless deflated for the number of looks","trigger":"a headline metric is reported without the trial count","action":"record trials and deflate before reporting"},
 {"rule":"SINGLE_ORIGIN_EVIDENCE clustering must be reflected in STATISTICAL_SIGNIFICANCE intervals","rationale":"iid intervals on clustered data are falsely tight","trigger":"a CI is computed under iid on single-origin data","action":"recompute with clustered standard errors and effective sample size"},
 {"rule":"MEDIUM_TIER_AS_FALSIFIABLE_BET must be tested out-of-sample before VALIDITY_VERDICT treats FP1 as established","rationale":"FP1 is a hypothesis, not a premise; an untested belief must not drive the verdict","trigger":"the verdict assumes FP1 is true without an out-of-sample test","action":"run the FP1 bet with a CI that can refute and gate the verdict on it"},
 {"rule":"REGIME_CHANGE structural-break test must run before extrapolating a result forward","rationale":"a past edge that spans a regime break does not transfer to the future","trigger":"a result is extrapolated without a break test","action":"label the valid regime and block extrapolation across a detected break"},
]

ARR=[
 {"rule":"Do not score cases before freezing the outcome definition; relabeling later invalidates every earlier score","prevents":"rescoring the whole backtest after a post hoc outcome redefinition"},
 {"rule":"Do not compute base rates or fit models on survivor-only data; reconstructing the cohort afterward forces a full re-estimate","prevents":"redoing all rates and fits once missing failures are recovered"},
 {"rule":"Do not report raw accuracy without the base rate; adding calibration later means re-communicating every claim","prevents":"retracting and restating accuracy figures after base-rate exposure"},
 {"rule":"Do not select features or rules with outcome knowledge; de-biasing afterward requires re-deriving the rule from scratch","prevents":"rebuilding the predictive rule once hindsight contamination is found"},
 {"rule":"Do not reuse the out-of-sample holdout for tuning; once touched it must be replaced with fresh untouched data","prevents":"sourcing a new holdout and re-running evaluation after contamination"},
 {"rule":"Do not report best-of-many results without counting trials; recovering the true trial count later changes every significance claim","prevents":"re-deflating and restating all results after the search history surfaces"},
 {"rule":"Do not treat FP1 (medium-tier opportunity) as a premise; if strategy is built on it and it fails the bet, the strategy must be unwound","prevents":"reversing strategy decisions that were built on an untested medium-tier belief"},
 {"rule":"Do not extrapolate across an untested regime boundary; a late-detected break forces re-scoping every forward claim","prevents":"re-issuing forward verdicts after a structural break is found post hoc"},
]

IP=[
 {"trigger":"a backtest result looks too good to be true (near-perfect out-of-sample)","action":"run the LEAKAGE_AUDIT end to end and re-check the TRAIN_TEST_SPLIT for future-information paths","nodes":["LEAKAGE_AUDIT","TRAIN_TEST_SPLIT","OUT_OF_SAMPLE"],"priority":"critical"},
 {"trigger":"the success rate is suspiciously high for the outcome","action":"reconstruct the entry cohort in SURVIVORSHIP_BIAS and recompute lift in BASE_RATE_CALIBRATION over entrants","nodes":["SURVIVORSHIP_BIAS","BASE_RATE_CALIBRATION"],"priority":"high"},
 {"trigger":"many strategies or thresholds were tried before the reported one","action":"count the trials in MULTIPLE_TESTING_IN_BACKTEST and recompute the deflated metric in DEFLATED_PERFORMANCE","nodes":["MULTIPLE_TESTING_IN_BACKTEST","DEFLATED_PERFORMANCE"],"priority":"high"},
 {"trigger":"the sample is dominated by a single account, campaign, or platform","action":"re-declare the clustering unit in SINGLE_ORIGIN_EVIDENCE and widen the intervals in STATISTICAL_SIGNIFICANCE","nodes":["SINGLE_ORIGIN_EVIDENCE","STATISTICAL_SIGNIFICANCE"],"priority":"high"},
 {"trigger":"a structural break or platform-algorithm change is detected","action":"re-label the valid regime in REGIME_CHANGE and re-run ROBUSTNESS_REPLICATION across sub-periods","nodes":["REGIME_CHANGE","ROBUSTNESS_REPLICATION"],"priority":"high"},
 {"trigger":"FP1 (medium-tier opportunity) is driving decisions without a test","action":"convert FP1 into a falsifiable bet in MEDIUM_TIER_AS_FALSIFIABLE_BET and gate the VALIDITY_VERDICT on its CI","nodes":["MEDIUM_TIER_AS_FALSIFIABLE_BET","VALIDITY_VERDICT"],"priority":"high"},
 {"trigger":"in-sample performance far exceeds out-of-sample performance","action":"increase regularization or reduce capacity in OVERFITTING_PAST_VIRALITY and re-estimate via CROSS_VALIDATION","nodes":["OVERFITTING_PAST_VIRALITY","CROSS_VALIDATION"],"priority":"medium"},
 {"trigger":"a falsification condition fires in production","action":"reopen the VALIDITY_VERDICT and re-test against fresh OUT_OF_SAMPLE data in the new regime","nodes":["VALIDITY_VERDICT","OUT_OF_SAMPLE"],"priority":"high"},
]

spec = {
 "domain":"backtest__outcome_validity",
 "domain_label":"Outcome Backtest Validity & Bias Control",
 "purpose":"run_outcome_based_backtests_against_real_known_results_with_base_rate_calibration_and_controls_for_survivorship_hindsight_and_single_origin_evidence_bias",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "scope is the validity and bias-control method for outcome backtests; model architecture and live deployment are delegated to sibling KBs",
   "real, already-known historical outcomes are available for at least part of the population of interest",
   "the scope finding 'medium-tier is the opportunity' (FP1) is treated as a falsifiable bet measured with a confidence interval, never as a premise",
 ],
 "exclusions":[
   "predictive model architecture and feature engineering internals",
   "live deployment, serving, and online monitoring",
   "holdout partition mechanics (delegated to leakage__holdout_design)",
   "causal identification beyond predictive validity",
 ],
 "source_description":"heuristic prior estimates for outcome-backtest validity and bias-control work units, informed by the backtest-overfitting, survivorship-bias, base-rate-neglect, hindsight-bias, and cross-validation literatures; no supplied dataset",
 "source_citation":"Bailey & Lopez de Prado 2014, 'The Deflated Sharpe Ratio: Correcting for Selection Bias, Backtest Overfitting and Non-Normality' (Journal of Portfolio Management); Brown, Goetzmann, Ibbotson & Ross 1992, 'Survivorship Bias in Performance Studies' (Review of Financial Studies); Kahneman & Tversky 1973, 'On the Psychology of Prediction' (base-rate neglect, Psychological Review); Tversky & Kahneman 1974, 'Judgment under Uncertainty: Heuristics and Biases' (Science); Fischhoff 1975, 'Hindsight is not equal to Foresight' (JEP: Human Perception and Performance); Arlot & Celisse 2010, 'A survey of cross-validation procedures for model selection' (Statistics Surveys)",
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
 "priority_rationale":"OUTCOME_DEFINITION/HISTORICAL_GROUND_TRUTH/SAMPLING_FRAME ground the test; the bias controls (survivorship, hindsight, single-origin) and base-rate calibration precede the temporal split; out-of-sample, cross-validation, overfitting and multiple-testing controls plus deflation establish honest performance; significance, the FP1 falsifiable bet, regime and robustness checks feed the final validity verdict.",
 "eval_objective":"verify_outcome_grounding_bias_control_out_of_sample_validity_multiple_testing_deflation_and_falsifiable_fp1_bet_of_backtest__outcome_validity_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "backtest__outcome_validity.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
