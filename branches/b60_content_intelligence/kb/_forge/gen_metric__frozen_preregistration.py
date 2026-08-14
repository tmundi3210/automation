#!/usr/bin/env python3
"""Generate the metric__frozen_preregistration content spec (B60) for kb_forge.py.
Compact authoring: node() applies sane defaults so only domain content + base
metric magnitudes are specified per node.

Domain: turn an informal "results are good" judgement into a NAMED construct ->
indicator -> measure, anchored to a baseline-to-beat, with a pre-registered
(frozen) metric + threshold, a per-cycle improvement criterion, guardrails against
Goodhart / multiple-comparisons, and a terminating STOPPING_RULE (threshold OR
max_cycles fuel budget). This is what makes the eval loop terminate (scope: R2
non-termination) -- a frozen metric + baseline + threshold + max_cycles.

Literature grounded: Goodhart 1975 / Strathern 1997; Benjamini & Hochberg 1995
(FDR); Kohavi, Tang & Xu 2020 Trustworthy Online Controlled Experiments (OEC);
Cronbach & Meehl 1955 construct validity; Nosek et al. preregistration.
"""
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
        "academic_fields": ["measurement_theory", "experimentation"],
        "subfields": subfields or ["construct_validity", "metric_design", "stopping_rules"],
        "specialists": specialists or ["experimentation_lead"],
        "contradictors": contradictors or ["vibes_based_evaluator"],
        "inputs": inputs or ["informal good/bad judgement", "candidate outputs"],
        "outputs": outputs or ["frozen metric artifact"],
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

N.append(node("SUCCESS_CONSTRUCT", "name_the_latent_construct_behind_good",
  "Turn the informal claim 'the results are good' into a single named latent construct (the unobservable quality being claimed, e.g. 'audience resonance' or 'editorial quality'), stated as a theoretical concept distinct from any one number that will later index it.",
  "foundations", [], ["CQ_01"],
  b(0.93, 0.85, 0.84, 0.55, 0.86, 0.82, 0.66, 0.66, 0.42, 0.76, 0.8, 0.9, 0.66, 0.18, 0.55, [0.2, 0.5],
    "the underlying notion of what 'good' means for this asset changes"),
  ["a named construct", "its conceptual definition", "what it explicitly is NOT"],
  ["the numeric measure itself", "the optimization loop"],
  [pro("A named construct gives the whole loop a fixed referent so later disputes are about measurement, not about what 'good' even meant",
       "'good thumbnail' is named as the construct click-through-worthiness, separable from any single CTR number")],
  [con("Naming a construct prematurely can reify a vague intuition into a target the team then over-trusts",
       "labelling 'engagement' as the construct hides that watch-time and shares pull in opposite directions")],
  ["the construct is conflated with one convenient metric from the start",
   "two stakeholders silently hold different constructs under the same word"],
  ["the construct has a one-sentence definition and an explicit not-this list, agreed by >=2 stakeholders"],
  ["the business definition of success for this asset is revised", "a new stakeholder disputes what good means"],
  specialists=["experimentation_lead", "measurement_theorist"]))

N.append(node("CONSTRUCT_VALIDITY", "evidence_the_construct_is_real_and_coherent",
  "Establish construct validity in the Cronbach-Meehl sense: assemble the nomological net (the lawful relations the construct should have with other variables) and gather convergent and discriminant evidence that the named construct actually exists and is internally coherent before it is allowed to drive a loop.",
  "foundations", ["SUCCESS_CONSTRUCT"], ["CQ_01", "CQ_02"],
  b(0.86, 0.74, 0.72, 0.7, 0.84, 0.84, 0.62, 0.58, 0.5, 0.7, 0.74, 0.84, 0.58, 0.24, 0.55, [0.28, 0.62],
    "convergent/discriminant evidence shifts or the nomological net is revised"),
  ["nomological net of expected relations", "convergent evidence", "discriminant evidence"],
  ["threshold setting", "the optimization schedule"],
  [pro("Construct validity makes 'we measured the right thing' an argued claim with evidence rather than an assertion",
       "share-rate and a human resonance rubric correlating r~0.6 is convergent evidence the construct is real")],
  [con("Full construct-validation is slow and can stall a loop that needs to start with an imperfect proxy",
       "demanding a validated nomological net before any iteration freezes a fast content pipeline")],
  ["no discriminant evidence, so the construct is indistinguishable from a confound",
   "the nomological net is assumed, never tested"],
  ["at least one convergent and one discriminant relation are documented with their observed direction"],
  ["a predicted relation in the nomological net is violated by data", "the construct is reused in a new domain"],
  specialists=["measurement_theorist", "psychometrician"]))

N.append(node("INDICATOR_SELECTION", "choose_observable_indicators_for_the_construct",
  "Select the observable indicator(s) that stand in for the latent construct: the measurable signals (e.g. share-rate, dwell-time, completion) hypothesized to reflect the construct, with an explicit reflective-vs-formative stance on how indicators relate to it.",
  "indicators", ["SUCCESS_CONSTRUCT"], ["CQ_02", "CQ_03"],
  b(0.85, 0.78, 0.74, 0.62, 0.8, 0.8, 0.58, 0.62, 0.48, 0.74, 0.76, 0.84, 0.62, 0.22, 0.5, [0.24, 0.56],
    "a candidate indicator is added, dropped, or re-weighted"),
  ["chosen indicators", "reflective/formative stance", "indicator-to-construct hypotheses"],
  ["the exact estimator/statistic", "FDR control"],
  [pro("Explicit indicator selection separates 'what we observe' from 'what we mean', so a weak indicator can be swapped without redefining success",
       "completion-rate chosen as the indicator for 'narrative grip', leaving the construct intact if completion is later dropped")],
  [con("A single convenient indicator invites tunnel vision and tighter Goodhart pressure",
       "using only watch-time as the indicator for quality rewards padding")],
  ["the indicator is chosen for availability, not for reflecting the construct",
   "reflective and formative indicators are mixed without saying which is which"],
  ["each indicator has a stated hypothesis linking it to the construct and a reflective/formative tag"],
  ["an indicator's correlation with the construct degrades", "a cheaper indicator becomes available"],
  specialists=["experimentation_lead", "analyst"]))

N.append(node("PROXY_VALIDITY", "test_whether_the_proxy_tracks_real_value",
  "Test proxy validity: quantify how well the chosen indicator (e.g. share-rate) actually tracks the real downstream value the construct is meant to capture, estimating the proxy-to-true-value correlation and the conditions under which it breaks down.",
  "indicators", ["INDICATOR_SELECTION", "CONSTRUCT_VALIDITY"], ["CQ_03", "CQ_04"],
  b(0.87, 0.8, 0.78, 0.68, 0.86, 0.82, 0.6, 0.56, 0.55, 0.7, 0.74, 0.86, 0.56, 0.26, 0.58, [0.3, 0.66],
    "the proxy-to-true-value relationship is re-estimated or found unstable"),
  ["proxy-to-true-value correlation estimate", "breakdown conditions", "validity caveats"],
  ["the freeze decision", "the stopping rule"],
  [pro("Knowing the proxy's true-value correlation lets you weight or discount it instead of trusting it blindly",
       "share-rate correlates r~0.4 with renewal revenue, so it is treated as a noisy directional proxy not a target")],
  [con("Proxy-validity estimates are themselves uncertain and can lend false confidence to a weak proxy",
       "a one-off r~0.7 on a small sample is read as proof the proxy is value-aligned")],
  ["the proxy is assumed valid because it is convenient",
   "validity is estimated once and never re-checked as the distribution shifts"],
  ["the proxy-to-true-value correlation is estimated with a confidence interval and stated breakdown conditions"],
  ["the proxy decouples from downstream value in a holdout", "the population the proxy was validated on changes"],
  specialists=["analyst", "causal_inference_specialist"]))

N.append(node("MEASURE_OPERATIONALIZATION", "define_the_exact_estimator_and_unit",
  "Operationalize the measure M: pin the exact estimator, population, unit of analysis, aggregation window, and computation that turns indicators into a single number, so 'the metric' is a reproducible function, not a description.",
  "measure", ["INDICATOR_SELECTION"], ["CQ_04", "CQ_05"],
  b(0.86, 0.76, 0.72, 0.7, 0.84, 0.78, 0.62, 0.62, 0.5, 0.76, 0.8, 0.84, 0.62, 0.22, 0.52, [0.24, 0.54],
    "the estimator, population, or aggregation window definition changes"),
  ["exact estimator formula", "population and unit of analysis", "aggregation window and computation"],
  ["the construct definition", "FDR / multiple-comparison control"],
  [pro("A fully operational measure makes the number reproducible and auditable by anyone with the data",
       "M = mean(shares/impressions) over 7 days on logged-in users, ties broken by impression count")],
  [con("Over-specifying the estimator early can hard-code a choice (window, population) that later proves wrong and is costly to unwind once frozen",
       "freezing a 7-day window before learning the signal matures at 14 days")],
  ["the metric is described in prose but never pinned to a single reproducible computation",
   "population or unit of analysis is left ambiguous so two teams compute different numbers"],
  ["two independent implementations of the estimator produce the same number on the same data"],
  ["the estimator is found ambiguous in implementation", "the data schema feeding the measure changes"],
  specialists=["analyst", "data_engineer"]))

N.append(node("OEC", "combine_proxies_into_one_overall_evaluation_criterion",
  "Define the Overall Evaluation Criterion (Kohavi): a single decision metric that combines the chosen proxies (and trade-offs between them) into one number the loop optimizes, so the team is not left arbitrating between several metrics every cycle.",
  "measure", ["INDICATOR_SELECTION", "PROXY_VALIDITY"], ["CQ_05", "CQ_06"],
  b(0.88, 0.82, 0.78, 0.74, 0.86, 0.85, 0.64, 0.56, 0.6, 0.7, 0.74, 0.88, 0.56, 0.26, 0.6, [0.3, 0.66],
    "the OEC weighting or its component proxies change"),
  ["single combined decision metric", "component weights and trade-offs", "directionality of each component"],
  ["guardrail thresholds", "per-comparison FDR control"],
  [pro("One OEC turns multi-metric debates into a single optimizable, pre-agreed objective the loop can act on",
       "OEC = 0.7*normalized_share_rate + 0.3*normalized_completion, agreed before iteration begins")],
  [con("Collapsing several proxies into one OEC hides trade-offs and can be gamed by pushing the cheapest component",
       "an OEC dominated by clicks rewards clickbait that tanks completion")],
  ["the OEC is a post-hoc average chosen to make current results look good",
   "component directions conflict so improving the OEC can worsen real value"],
  ["the OEC is a single function of named components with fixed weights agreed before any optimization cycle"],
  ["a component proxy is revalued or dropped", "the OEC is found gameable by a degenerate strategy"],
  specialists=["experimentation_lead", "decision_scientist"]))

N.append(node("BASELINE_TO_BEAT", "establish_naive_and_human_curated_baselines",
  "Establish the baseline(s) to beat: at minimum a naive/random baseline (what the metric scores with no skill) and a human-curated baseline (a strong reference the loop must exceed), so 'improvement' is defined against a fixed anchor rather than against the previous noisy cycle.",
  "baseline", ["MEASURE_OPERATIONALIZATION"], ["CQ_06", "CQ_07"],
  b(0.9, 0.82, 0.78, 0.6, 0.88, 0.8, 0.66, 0.64, 0.5, 0.78, 0.8, 0.9, 0.64, 0.2, 0.58, [0.22, 0.5],
    "the baseline population, reference set, or computation changes"),
  ["naive/random baseline value", "human-curated baseline value", "baseline computation method"],
  ["the improvement delta threshold", "the stopping rule"],
  [pro("A fixed naive+human baseline turns the metric into a yardstick: a score is now meaningful relative to chance and to a strong human",
       "a share-rate of 0.04 means little until you know random is 0.01 and a human editor hits 0.05")],
  [con("A baseline computed on a non-comparable population silently inflates or deflates measured improvement",
       "a human baseline drawn from a premium audience makes the model look worse than it is")],
  ["only the previous cycle is used as the reference, so noise reads as improvement",
   "the baseline is recomputed each cycle, removing the fixed anchor"],
  ["both a naive and a human-curated baseline are computed on the same population and unit as the live metric",
   "the baseline values are frozen alongside the metric, not recomputed per cycle"],
  ["the comparison population shifts away from the baseline population", "a stronger human reference becomes available"],
  specialists=["experimentation_lead", "analyst"]))

N.append(node("FROZEN_METRIC", "preregister_and_freeze_metric_plus_threshold",
  "Pre-register and freeze the metric: lock the operational measure M, its baseline, and a success threshold into an immutable record (Nosek-style preregistration) before optimization begins, forbidding post-hoc redefinition of the metric to fit results.",
  "freeze", ["BASELINE_TO_BEAT", "MEASURE_OPERATIONALIZATION", "OEC"], ["CQ_07", "CQ_08"],
  b(0.95, 0.88, 0.82, 0.66, 0.92, 0.86, 0.82, 0.6, 0.62, 0.78, 0.82, 0.94, 0.6, 0.22, 0.66, [0.24, 0.52],
    "the frozen metric, baseline, or threshold is proposed for amendment"),
  ["immutable metric+baseline+threshold record", "freeze timestamp and owner", "amendment policy"],
  ["the actual optimization runs", "guardrail definitions"],
  [pro("Freezing the metric before results removes the degree of freedom that lets a team redraw the target around whatever happened",
       "the threshold M>=0.05 is registered and hashed before cycle 1, so a later 0.045 cannot be relabelled a success")],
  [con("A frozen metric that turns out mis-specified cannot be quietly fixed; amendment requires a visible, costly change-control step",
       "discovering the window was wrong forces a logged amendment that resets the comparison")],
  ["the metric is 'frozen' but quietly amended when results disappoint (HARKing / outcome switching)",
   "no immutable record exists, so the pre-registration claim is unfalsifiable"],
  ["the metric, baseline and threshold are stored in an immutable, timestamped, owner-attributed record before any optimization cycle",
   "any change to the frozen record is recorded as a dated amendment, never a silent edit"],
  ["a mis-specification in the frozen metric is discovered", "a stakeholder requests a metric change mid-loop"],
  specialists=["experimentation_lead", "preregistration_steward"]))

N.append(node("IMPROVEMENT_CRITERION", "require_delta_at_least_epsilon_per_cycle",
  "Define the improvement criterion: require delta_M >= epsilon (a pre-set minimum effect size) over the frozen baseline for a cycle to count as progress, so trivial or noise-sized gains do not justify continuing the loop.",
  "criterion", ["FROZEN_METRIC"], ["CQ_08", "CQ_09"],
  b(0.9, 0.82, 0.78, 0.68, 0.88, 0.82, 0.66, 0.6, 0.58, 0.74, 0.78, 0.9, 0.6, 0.24, 0.6, [0.28, 0.62],
    "epsilon, the effect-size definition, or the noise model changes"),
  ["minimum delta epsilon", "effect-size and noise model", "per-cycle progress test"],
  ["the absolute success threshold", "fuel-budget accounting"],
  [pro("A minimum delta epsilon makes 'did this cycle improve things' a hypothesis test, not a glance at a noisy number",
       "require delta_M >= 0.5 sigma over baseline before a cycle is logged as an improvement")],
  [con("Setting epsilon too high discards real small gains; too low lets noise masquerade as progress",
       "epsilon below the metric's standard error declares random walk steps as wins")],
  ["epsilon is smaller than the metric's noise, so the loop 'improves' on noise",
   "delta is measured against the previous cycle rather than the frozen baseline"],
  ["epsilon exceeds the metric's estimated standard error and is fixed before iteration",
   "a cycle counts as progress only when delta_M over the frozen baseline meets epsilon"],
  ["the metric's measured noise rises above epsilon", "the effect-size convention for the domain changes"],
  specialists=["experimentation_lead", "statistician"]))

N.append(node("GOODHART", "guard_against_metric_capture_and_gaming",
  "Guard against Goodhart's law (Goodhart 1975; Strathern 1997 'when a measure becomes a target it ceases to be a good measure'): detect and prevent the optimization from gaming the frozen metric in ways that decouple it from the construct it was meant to index.",
  "guardrail", ["FROZEN_METRIC", "PROXY_VALIDITY"], ["CQ_09", "CQ_10"],
  b(0.9, 0.82, 0.8, 0.74, 0.9, 0.86, 0.64, 0.54, 0.66, 0.68, 0.72, 0.9, 0.54, 0.28, 0.64, [0.32, 0.7],
    "a new gaming strategy is observed or the metric-construct gap widens"),
  ["gaming-strategy watchlist", "metric-construct decoupling checks", "anti-gaming guardrails"],
  ["FDR control across comparisons", "the fuel budget"],
  [pro("Explicit Goodhart guards catch the failure where the metric keeps rising while the real construct falls",
       "share-rate climbs while a human resonance audit drops, flagging the metric has become a gamed target")],
  [con("Aggressive anti-gaming checks can suppress genuine improvements that merely look like gaming",
       "penalizing every CTR jump as clickbait discourages legitimately better thumbnails")],
  ["the metric is optimized to the point it no longer reflects the construct",
   "gaming is only noticed after the construct has visibly degraded"],
  ["a held-out construct check (e.g. human rubric) runs alongside the metric and a divergence triggers review"],
  ["the metric and a construct audit diverge", "a degenerate strategy that inflates the metric is discovered"],
  specialists=["experimentation_lead", "red_team_analyst"]))

N.append(node("GUARDRAIL_METRICS", "watch_for_harm_while_optimizing",
  "Define guardrail metrics (Kohavi): secondary metrics that must not regress while the OEC is optimized, so the loop cannot trade away safety, retention, or quality to win on the primary metric.",
  "guardrail", ["OEC", "FROZEN_METRIC"], ["CQ_10", "CQ_11"],
  b(0.86, 0.8, 0.82, 0.66, 0.86, 0.8, 0.62, 0.6, 0.58, 0.74, 0.78, 0.86, 0.6, 0.24, 0.6, [0.28, 0.6],
    "a guardrail metric or its non-regression bound changes"),
  ["set of guardrail metrics", "non-regression bounds", "harm/quality watch list"],
  ["the primary OEC optimization", "termination certification"],
  [pro("Guardrails let you optimize aggressively on the OEC while bounding collateral damage on what you are unwilling to lose",
       "allow share-rate gains only if unsubscribe-rate and complaint-rate stay within their bounds")],
  [con("Too many guardrails make every cycle fail some bound and stall the loop in over-caution",
       "a dozen guardrails each with tight bounds means no candidate ever clears them all")],
  ["a guardrail metric is harmed silently because nobody is watching it",
   "guardrails are added post-hoc to explain away an unwanted result"],
  ["each guardrail has a pre-set non-regression bound checked every cycle alongside the OEC"],
  ["a guardrail breach occurs", "a new harm dimension emerges that no guardrail covers"],
  specialists=["experimentation_lead", "trust_and_safety_analyst"]))

N.append(node("MULTIPLE_COMPARISONS", "control_false_discovery_across_many_tests",
  "Control multiple comparisons: when the metric is evaluated across many fields, niches, segments, or cycles, apply Benjamini-Hochberg FDR control so the rate of false 'wins' is bounded and the loop does not declare success on noise found by searching many slices.",
  "guardrail", ["IMPROVEMENT_CRITERION"], ["CQ_11", "CQ_12"],
  b(0.85, 0.78, 0.72, 0.78, 0.88, 0.78, 0.62, 0.54, 0.6, 0.7, 0.74, 0.85, 0.54, 0.28, 0.62, [0.32, 0.68],
    "the number of comparisons or the target FDR level changes"),
  ["family of comparisons", "Benjamini-Hochberg FDR procedure", "adjusted significance per comparison"],
  ["the construct definition", "human review policy"],
  [pro("FDR control bounds the expected fraction of false wins when scanning many niches, replacing naive per-test p-values",
       "testing 50 niches at q=0.1 via Benjamini-Hochberg keeps expected false discoveries near 10% of declared wins")],
  [con("FDR control reduces power, so real but small effects in some niches may be missed",
       "a genuine lift in one rare niche fails the adjusted threshold and is discarded")],
  ["per-niche p-values are read without any multiplicity adjustment",
   "the comparison family is defined after seeing which slices won (garden of forking paths)"],
  ["the family of comparisons is declared before testing and Benjamini-Hochberg controls FDR at a pre-set level"],
  ["the number of slices tested grows", "the target FDR level is renegotiated"],
  specialists=["statistician", "analyst"]))

N.append(node("STOPPING_RULE", "stop_at_threshold_or_max_cycles",
  "Define the terminating stopping rule: stop the optimization loop when the frozen metric crosses its threshold OR when the max_cycles fuel budget is exhausted, whichever comes first. This conjunction of a frozen metric + baseline + threshold + max_cycles is exactly what makes the eval loop terminate (scope: R2 non-termination).",
  "stopping", ["FROZEN_METRIC", "IMPROVEMENT_CRITERION"], ["CQ_12", "CQ_13"],
  b(0.96, 0.9, 0.84, 0.66, 0.94, 0.88, 0.84, 0.6, 0.64, 0.78, 0.82, 0.95, 0.6, 0.22, 0.7, [0.24, 0.5],
    "the threshold, max_cycles budget, or the stop condition logic changes"),
  ["stop-on-threshold condition", "stop-on-max_cycles condition", "termination guarantee"],
  ["guardrail definitions", "construct definition"],
  [pro("A threshold-OR-max_cycles rule guarantees the loop halts: either it wins or it runs out of fuel, never spins forever",
       "stop when M>=0.05 or after 8 cycles, so the loop provably terminates in at most 8 cycles")],
  [con("A max_cycles budget set too low stops a loop that was still genuinely improving toward the threshold",
       "halting at 8 cycles when the trajectory would have crossed the threshold at cycle 9")],
  ["only a threshold stop exists, so a loop that never reaches it runs forever (R2 non-termination)",
   "max_cycles is mutable mid-loop, defeating the termination guarantee"],
  ["the loop halts deterministically when M crosses the frozen threshold or max_cycles is reached, whichever first",
   "max_cycles is fixed in the frozen record so termination is bounded a priori"],
  ["the fuel budget is repeatedly exhausted without crossing threshold", "the threshold is found unreachable in principle"],
  specialists=["experimentation_lead", "reliability_engineer"]))

N.append(node("FUEL_BUDGET", "account_the_max_cycles_compute_budget",
  "Account the fuel budget: track and enforce max_cycles (and any per-cycle compute/cost ceiling) as a hard, non-renewable resource, so the stopping rule's max_cycles arm has a real ledger and the loop cannot quietly extend itself.",
  "stopping", ["STOPPING_RULE"], ["CQ_13", "CQ_14"],
  b(0.84, 0.8, 0.74, 0.6, 0.84, 0.76, 0.7, 0.66, 0.5, 0.78, 0.8, 0.84, 0.66, 0.2, 0.56, [0.2, 0.48],
    "the max_cycles budget or per-cycle cost ceiling is changed"),
  ["max_cycles ledger", "per-cycle cost ceiling", "remaining-fuel accounting"],
  ["the threshold condition", "construct validity"],
  [pro("A hard fuel ledger makes the max_cycles arm enforceable: remaining budget is a tracked number, not a vibe",
       "a counter decrements each cycle and the loop refuses to start cycle 9 when budget is 8")],
  [con("A rigid fuel ledger with no contingency can waste a near-win that needed one more cheap cycle",
       "the loop halts with budget zero one cycle short of a clear threshold crossing")],
  ["the budget is treated as advisory and silently extended when results look promising",
   "no remaining-fuel counter exists, so max_cycles is never actually enforced"],
  ["a monotone non-renewable counter enforces max_cycles and blocks any cycle beyond the frozen budget"],
  ["the budget is exhausted without a decision", "compute cost per cycle changes materially"],
  specialists=["reliability_engineer", "cost_analyst"]))

N.append(node("METRIC_DRIFT_AUDIT", "detect_silent_drift_of_a_frozen_metric",
  "Audit for metric drift: detect when the data pipeline, population, or computation feeding a frozen metric silently changes so that the 'same' frozen metric no longer measures the same thing, which would invalidate the comparison to the frozen baseline.",
  "audit", ["FROZEN_METRIC", "MEASURE_OPERATIONALIZATION"], ["CQ_14"],
  b(0.82, 0.74, 0.7, 0.66, 0.84, 0.8, 0.6, 0.6, 0.52, 0.74, 0.78, 0.82, 0.6, 0.24, 0.54, [0.28, 0.6],
    "the pipeline, schema, or population feeding the frozen metric changes"),
  ["drift detection on inputs", "population stability checks", "computation-hash verification"],
  ["the freeze decision itself", "the OEC weighting"],
  [pro("Drift auditing protects the integrity of a frozen comparison by catching when 'the metric' silently mutated",
       "a logging change halves impression counts, detected before it is mistaken for a metric drop")],
  [con("Over-sensitive drift alarms create noise that erodes trust in the freeze",
       "every minor schema tweak fires a drift alert and the team starts ignoring them")],
  ["a pipeline change shifts the metric and is read as a real movement",
   "the frozen computation is altered without a recorded amendment"],
  ["the frozen metric's input distribution and computation hash are monitored and any change raises a drift flag"],
  ["an input distribution shift is detected", "the metric computation hash changes unexpectedly"],
  specialists=["data_engineer", "analyst"]))

N.append(node("SENSITIVITY_ANALYSIS", "test_robustness_of_the_decision_to_assumptions",
  "Run sensitivity analysis: vary the metric's free choices (window, weights, population, epsilon) and confirm the stop/continue decision is robust, so the conclusion does not hinge on one arbitrary frozen choice.",
  "audit", ["IMPROVEMENT_CRITERION", "OEC"], ["CQ_09", "CQ_14"],
  b(0.8, 0.74, 0.7, 0.72, 0.82, 0.8, 0.58, 0.58, 0.52, 0.72, 0.76, 0.8, 0.58, 0.24, 0.52, [0.3, 0.64],
    "a free analysis choice is found to flip the decision"),
  ["specification-curve over free choices", "decision-flip detection", "robustness summary"],
  ["the frozen record", "fuel accounting"],
  [pro("Sensitivity analysis shows whether a win survives reasonable alternative choices or is an artifact of one window",
       "the threshold crossing holds across 7/14/28-day windows, so it is not a windowing artifact")],
  [con("A wide sensitivity sweep can manufacture a specification where any result looks fragile",
       "testing 40 weighting schemes is sure to find one where the decision flips")],
  ["the decision rests on a single arbitrary specification never stress-tested",
   "sensitivity is run only over choices that confirm the desired answer"],
  ["the stop/continue decision is shown stable across a pre-declared set of reasonable alternative specifications"],
  ["a reasonable alternative specification flips the decision", "a previously fixed choice becomes contested"],
  specialists=["statistician", "analyst"]))

N.append(node("PREREGISTRATION_RECORD", "produce_the_immutable_preregistration_artifact",
  "Produce the preregistration record: a single immutable, timestamped artifact (Nosek-style) bundling the construct, indicators, measure, baselines, frozen threshold, epsilon, guardrails, FDR plan, and max_cycles, hashed so the whole pre-commitment is auditable and tamper-evident.",
  "record", ["FROZEN_METRIC", "STOPPING_RULE", "GUARDRAIL_METRICS", "MULTIPLE_COMPARISONS"], ["CQ_08", "CQ_15"],
  b(0.9, 0.84, 0.8, 0.62, 0.9, 0.85, 0.84, 0.62, 0.55, 0.8, 0.82, 0.9, 0.62, 0.2, 0.64, [0.22, 0.5],
    "any pre-committed element is amended or the record is re-hashed"),
  ["bundled immutable artifact", "content hash and timestamp", "amendment log"],
  ["live optimization", "drift monitoring internals"],
  [pro("A single hashed preregistration artifact makes the entire pre-commitment falsifiable and tamper-evident",
       "the record's hash is published before cycle 1, so any later edit is detectable")],
  [con("Bundling everything into one record makes legitimate mid-course amendment heavy and visible by design",
       "fixing one wrong guardrail bound requires re-hashing and logging an amendment to the whole record")],
  ["the record is editable in place, so pre-registration cannot be verified",
   "the artifact omits the stopping rule, leaving termination unpinned"],
  ["the artifact is content-hashed, timestamped, and includes construct, measure, baseline, threshold, epsilon, guardrails, FDR plan and max_cycles",
   "every amendment is appended to an immutable log rather than overwriting the artifact"],
  ["an amendment is requested", "the artifact is found to omit a pre-committed element"],
  specialists=["preregistration_steward", "experimentation_lead"]))

N.append(node("METRIC_PROVENANCE", "trace_why_each_metric_choice_was_made",
  "Record metric provenance: for every frozen choice (construct naming, indicator, estimator, baseline, threshold, epsilon, max_cycles), capture who chose it, on what evidence, and the alternatives rejected, so the freeze is explainable and a future amendment is principled rather than ad hoc.",
  "record", ["PREREGISTRATION_RECORD", "CONSTRUCT_VALIDITY"], ["CQ_15", "CQ_16"],
  b(0.78, 0.72, 0.74, 0.58, 0.74, 0.78, 0.6, 0.68, 0.42, 0.8, 0.8, 0.78, 0.68, 0.16, 0.44, [0.18, 0.46],
    "the provenance schema or evidence-capture policy changes"),
  ["per-choice rationale", "evidence and alternatives-rejected log", "owner attribution"],
  ["the live loop", "FDR computation"],
  [pro("Provenance turns the frozen metric from an opaque constant into an explainable, amendable decision record",
       "'why epsilon=0.5sigma?' is answered by the recorded power analysis and the rejected 0.2sigma option")],
  [con("Full provenance capture adds documentation overhead that teams under time pressure may skip",
       "logging the rationale and rejected alternatives for every choice doubles the freeze effort")],
  ["a frozen choice has no recorded rationale, so amendments are guesses",
   "provenance is written after the fact to justify the chosen number"],
  ["each frozen choice carries an owner, the evidence used, and at least one rejected alternative"],
  ["a choice must be amended without recorded rationale", "the evidence behind a choice is invalidated"],
  specialists=["preregistration_steward", "analyst"]))

N.append(node("DECISION_GATE", "render_the_stop_continue_amend_decision",
  "Render the decision gate: at each cycle boundary, apply the frozen rules (improvement criterion, guardrails, FDR, stopping rule) to render an explicit stop / continue / amend decision with its justification, so loop control is a logged judgement, not drift.",
  "decision", ["STOPPING_RULE", "IMPROVEMENT_CRITERION", "GUARDRAIL_METRICS", "MULTIPLE_COMPARISONS"], ["CQ_12", "CQ_16"],
  b(0.9, 0.85, 0.8, 0.64, 0.9, 0.86, 0.7, 0.6, 0.6, 0.78, 0.82, 0.9, 0.6, 0.22, 0.62, [0.24, 0.52],
    "the decision logic or any rule it composes changes"),
  ["per-cycle stop/continue/amend verdict", "justification against frozen rules", "decision log"],
  ["the construct definition", "raw indicator selection"],
  [pro("A logged decision gate makes every cycle's stop/continue/amend a justified, auditable call against the frozen rules",
       "cycle 5 logs 'continue: delta>=epsilon, all guardrails green, FDR-adjusted win, budget remaining'")],
  [con("A rigid gate can force a mechanically correct but contextually wrong stop, with no room for judgement",
       "the gate halts at max_cycles despite an obvious near-win, because the rule admits no override")],
  ["the gate is bypassed and the loop continues on intuition",
   "an amend verdict is used to quietly relax the frozen threshold"],
  ["every cycle boundary produces a logged stop/continue/amend verdict justified by the frozen rules"],
  ["a decision is overridden manually", "a frozen rule the gate composes is amended"],
  specialists=["experimentation_lead", "decision_scientist"]))

N.append(node("TERMINATION_CERTIFICATE", "certify_the_loop_terminated_correctly",
  "Issue a termination certificate: when the loop stops, emit a record stating which arm fired (threshold crossed or max_cycles exhausted), the final metric vs baseline vs threshold, guardrail status, and FDR-adjusted significance, certifying the eval loop terminated under the frozen rules (closing R2 non-termination).",
  "decision", ["DECISION_GATE", "FUEL_BUDGET", "SENSITIVITY_ANALYSIS", "METRIC_DRIFT_AUDIT"], ["CQ_13", "CQ_16"],
  b(0.88, 0.82, 0.82, 0.6, 0.88, 0.82, 0.78, 0.64, 0.5, 0.8, 0.82, 0.88, 0.64, 0.2, 0.6, [0.22, 0.5],
    "the certification format or the termination criteria change"),
  ["which stop arm fired", "final metric vs baseline vs threshold", "guardrail and FDR status at stop"],
  ["the next loop's construct", "post-hoc reinterpretation"],
  [pro("A termination certificate makes 'the loop is done and it terminated legitimately' an inspectable artifact, closing R2",
       "the certificate reads 'stopped at cycle 6: M=0.052>=0.05 threshold, guardrails green, BH-significant'")],
  [con("A certificate can lend false closure if the underlying frozen rules were themselves mis-specified",
       "a clean certificate on a Goodharted metric certifies a hollow win")],
  ["the loop stops with no record of which arm fired or the final comparison",
   "a max_cycles stop is reported as a threshold success"],
  ["the certificate states the firing arm, final metric/baseline/threshold, guardrail status and FDR-adjusted significance",
   "a threshold stop and a max_cycles stop are distinguishable in the certificate"],
  ["the certified result is challenged", "the termination criteria are revised for the next loop"],
  specialists=["experimentation_lead", "reliability_engineer"]))

# ---------------- competency questions (14) ----------------
CQ = [
 ("CQ_01", "How is an informal 'the results are good' turned into a single named, validated construct?",
  ["nodes", "glossary"], "SUCCESS_CONSTRUCT names the construct and CONSTRUCT_VALIDITY evidences it",
  ["SUCCESS_CONSTRUCT", "CONSTRUCT_VALIDITY"]),
 ("CQ_02", "Which observable indicators are chosen to reflect the construct and on what stance?",
  ["nodes"], "INDICATOR_SELECTION chooses indicators with a reflective/formative stance tied to the construct",
  ["INDICATOR_SELECTION", "CONSTRUCT_VALIDITY"]),
 ("CQ_03", "How is it tested whether a proxy indicator actually tracks real downstream value?",
  ["nodes"], "PROXY_VALIDITY estimates the proxy-to-true-value correlation and its breakdown conditions",
  ["PROXY_VALIDITY", "INDICATOR_SELECTION"]),
 ("CQ_04", "How is the measure operationalized into one reproducible number?",
  ["nodes"], "MEASURE_OPERATIONALIZATION pins the estimator, population, window and computation",
  ["MEASURE_OPERATIONALIZATION", "PROXY_VALIDITY"]),
 ("CQ_05", "How are several proxies combined into one overall evaluation criterion?",
  ["nodes"], "OEC defines a single weighted decision metric over the proxies",
  ["OEC", "MEASURE_OPERATIONALIZATION"]),
 ("CQ_06", "What baselines must a result beat to count as good?",
  ["nodes"], "BASELINE_TO_BEAT fixes a naive and a human-curated baseline on the same population",
  ["BASELINE_TO_BEAT", "OEC"]),
 ("CQ_07", "How is the metric frozen and pre-registered so it cannot be redefined post-hoc?",
  ["nodes"], "FROZEN_METRIC locks measure+baseline+threshold into an immutable record before optimization",
  ["FROZEN_METRIC", "BASELINE_TO_BEAT"]),
 ("CQ_08", "What hashed pre-registration artifact records the whole pre-commitment, and how is each frozen choice made explainable and tamper-evident?",
  ["nodes"], "PREREGISTRATION_RECORD bundles and hashes the frozen elements and METRIC_PROVENANCE records the rationale behind each choice",
  ["PREREGISTRATION_RECORD", "FROZEN_METRIC", "METRIC_PROVENANCE"]),
 ("CQ_09", "What minimum per-cycle improvement counts as progress, and is it robust?",
  ["nodes"], "IMPROVEMENT_CRITERION requires delta_M>=epsilon and SENSITIVITY_ANALYSIS checks robustness",
  ["IMPROVEMENT_CRITERION", "SENSITIVITY_ANALYSIS"]),
 ("CQ_10", "How is the loop kept from gaming the metric away from the construct?",
  ["nodes", "conflict_axes"], "GOODHART guards metric-construct decoupling alongside the frozen metric",
  ["GOODHART", "GUARDRAIL_METRICS"]),
 ("CQ_11", "What secondary metrics must not regress while optimizing, and how is false discovery bounded?",
  ["nodes"], "GUARDRAIL_METRICS bounds harm; MULTIPLE_COMPARISONS controls FDR across slices",
  ["GUARDRAIL_METRICS", "MULTIPLE_COMPARISONS"]),
 ("CQ_12", "What is the terminating stopping rule, how is each cycle's stop/continue/amend decided, and how is termination certified?",
  ["nodes", "iteration_protocol"], "STOPPING_RULE stops at threshold OR max_cycles, DECISION_GATE renders the verdict, and TERMINATION_CERTIFICATE certifies which arm fired",
  ["STOPPING_RULE", "DECISION_GATE", "TERMINATION_CERTIFICATE"]),
 ("CQ_13", "How is the max_cycles fuel budget accounted so the termination guarantee is enforced?",
  ["nodes"], "FUEL_BUDGET enforces max_cycles as a hard non-renewable ledger feeding the stopping rule",
  ["FUEL_BUDGET", "STOPPING_RULE"]),
 ("CQ_14", "How is a frozen metric protected from silent drift and arbitrary specification?",
  ["nodes"], "METRIC_DRIFT_AUDIT detects pipeline drift; SENSITIVITY_ANALYSIS tests specification robustness",
  ["METRIC_DRIFT_AUDIT", "SENSITIVITY_ANALYSIS"]),
]
CQS = [{"id": i, "question": q, "must_be_answerable_from": m, "acceptance_condition": a, "covered_by": c}
       for (i, q, m, a, c) in CQ]

# consolidate node->CQ references (each node refs 1-2; every CQ covered by >=1 node)
CQ_MAP = {
 "SUCCESS_CONSTRUCT": ["CQ_01"], "CONSTRUCT_VALIDITY": ["CQ_01", "CQ_02"],
 "INDICATOR_SELECTION": ["CQ_02", "CQ_03"], "PROXY_VALIDITY": ["CQ_03", "CQ_04"],
 "MEASURE_OPERATIONALIZATION": ["CQ_04", "CQ_05"], "OEC": ["CQ_05", "CQ_06"],
 "BASELINE_TO_BEAT": ["CQ_06", "CQ_07"], "FROZEN_METRIC": ["CQ_07", "CQ_08"],
 "IMPROVEMENT_CRITERION": ["CQ_09"], "GOODHART": ["CQ_10"],
 "GUARDRAIL_METRICS": ["CQ_10", "CQ_11"], "MULTIPLE_COMPARISONS": ["CQ_11"],
 "STOPPING_RULE": ["CQ_12", "CQ_13"], "FUEL_BUDGET": ["CQ_13"],
 "METRIC_DRIFT_AUDIT": ["CQ_14"], "SENSITIVITY_ANALYSIS": ["CQ_09", "CQ_14"],
 "PREREGISTRATION_RECORD": ["CQ_08"], "METRIC_PROVENANCE": ["CQ_08"],
 "DECISION_GATE": ["CQ_12"], "TERMINATION_CERTIFICATE": ["CQ_12"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

# ---------------- glossary ----------------
GL = [
 ("construct", "the unobservable quality 'good' is claimed to be, named and defined apart from any one number",
  ["latent_construct", "theoretical_concept"], ["indicator", "measure"], ["SUCCESS_CONSTRUCT", "CONSTRUCT_VALIDITY"]),
 ("indicator", "an observable signal hypothesized to reflect the construct",
  ["proxy_signal", "observable"], ["construct", "estimator"], ["INDICATOR_SELECTION", "PROXY_VALIDITY"]),
 ("measure", "the exact reproducible function (estimator + population + window) that yields the metric number M",
  ["estimator", "metric_definition"], ["construct", "threshold"], ["MEASURE_OPERATIONALIZATION", "FROZEN_METRIC"]),
 ("OEC", "overall evaluation criterion: one combined decision metric over the proxies (Kohavi)",
  ["overall_evaluation_criterion", "decision_metric"], ["guardrail_metric"], ["OEC", "GUARDRAIL_METRICS"]),
 ("baseline_to_beat", "a fixed anchor (naive/random and human-curated) the metric must exceed for a result to count",
  ["reference_baseline", "anchor"], ["previous_cycle_score"], ["BASELINE_TO_BEAT", "IMPROVEMENT_CRITERION"]),
 ("frozen_metric", "a pre-registered, immutable measure+baseline+threshold that cannot be redefined post-hoc",
  ["preregistered_metric", "locked_metric"], ["adjustable_metric"], ["FROZEN_METRIC", "PREREGISTRATION_RECORD"]),
 ("epsilon", "the minimum delta_M over baseline that counts as a genuine per-cycle improvement",
  ["minimum_effect_size", "mde"], ["noise_band"], ["IMPROVEMENT_CRITERION", "SENSITIVITY_ANALYSIS"]),
 ("stopping_rule", "the rule that halts the loop at threshold OR max_cycles, guaranteeing termination",
  ["termination_rule", "halt_condition"], ["open_loop"], ["STOPPING_RULE", "FUEL_BUDGET"]),
 ("Goodharts_law", "when a measure becomes a target it ceases to be a good measure (Goodhart 1975; Strathern 1997)",
  ["metric_gaming", "surrogation"], ["valid_optimization"], ["GOODHART", "PROXY_VALIDITY"]),
 ("FDR", "false discovery rate: expected fraction of declared wins that are false, controlled by Benjamini-Hochberg",
  ["false_discovery_rate", "benjamini_hochberg"], ["familywise_error_rate"], ["MULTIPLE_COMPARISONS", "DECISION_GATE"]),
 ("max_cycles", "the fuel budget: the hard cap on optimization cycles whose exhaustion forces a stop",
  ["fuel_budget", "cycle_cap"], ["unlimited_iteration"], ["FUEL_BUDGET", "STOPPING_RULE"]),
 ("guardrail_metric", "a secondary metric that must not regress while the OEC is optimized (Kohavi)",
  ["non_regression_metric", "safety_metric"], ["OEC"], ["GUARDRAIL_METRICS", "TERMINATION_CERTIFICATE"]),
]
GLS = [{"term": t, "definition": d, "synonyms": s, "not_same_as": ns, "used_by_nodes": u}
       for (t, d, s, ns, u) in GL]

# ---------------- edges ----------------
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
              "risk_of_conflict": "unmanaged tension degrades the metric or the loop", "example": "see resolution_rule"})
def rel(f, t, et, rs, cc=0.7, cp=0.2, erc=0.3, why=""):
    E.append({"from": f, "to": t, "edge_type": et, "relation_strength": rs, "signed_tension": 0.0,
              "causal_confidence": cc, "conflict_probability": cp, "expected_rework_cost": erc,
              "why_related": why or f"{f} {et} {t}", "benefit_of_coupling": "coordinated behavior",
              "risk_of_conflict": "inconsistency if uncoordinated", "example": f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror the primary spine of node.dependencies; secondary
# parents remain in node.dependencies for the DAG, non-dependency edges fill the rest)
dep("SUCCESS_CONSTRUCT", "CONSTRUCT_VALIDITY", 0.88)
dep("SUCCESS_CONSTRUCT", "INDICATOR_SELECTION", 0.86)
dep("INDICATOR_SELECTION", "PROXY_VALIDITY", 0.84)
dep("INDICATOR_SELECTION", "MEASURE_OPERATIONALIZATION", 0.84)
dep("PROXY_VALIDITY", "OEC", 0.78)
dep("MEASURE_OPERATIONALIZATION", "BASELINE_TO_BEAT", 0.85)
dep("BASELINE_TO_BEAT", "FROZEN_METRIC", 0.88)
dep("OEC", "FROZEN_METRIC", 0.8)
dep("FROZEN_METRIC", "IMPROVEMENT_CRITERION", 0.86)
dep("FROZEN_METRIC", "GOODHART", 0.8)
dep("FROZEN_METRIC", "GUARDRAIL_METRICS", 0.78)
dep("IMPROVEMENT_CRITERION", "MULTIPLE_COMPARISONS", 0.8)
dep("FROZEN_METRIC", "STOPPING_RULE", 0.9)
dep("STOPPING_RULE", "FUEL_BUDGET", 0.86)
dep("FROZEN_METRIC", "METRIC_DRIFT_AUDIT", 0.78)
dep("IMPROVEMENT_CRITERION", "SENSITIVITY_ANALYSIS", 0.78)
dep("FROZEN_METRIC", "PREREGISTRATION_RECORD", 0.86)
dep("PREREGISTRATION_RECORD", "METRIC_PROVENANCE", 0.8)
dep("STOPPING_RULE", "DECISION_GATE", 0.88)
dep("GUARDRAIL_METRICS", "DECISION_GATE", 0.78)
dep("MULTIPLE_COMPARISONS", "DECISION_GATE", 0.76)
dep("DECISION_GATE", "TERMINATION_CERTIFICATE", 0.86)
dep("FUEL_BUDGET", "TERMINATION_CERTIFICATE", 0.8)

# cross-cutting non-dependency edges (cross freely; reach count without cycle risk)
rel("GOODHART", "INDICATOR_SELECTION", "feedback", 0.78,
    why="detected gaming feeds back to revise or add indicators so the metric re-tracks the construct")
rel("METRIC_DRIFT_AUDIT", "FROZEN_METRIC", "feedback", 0.74,
    why="a detected drift forces a logged amendment to the frozen metric rather than a silent edit")
rel("GUARDRAIL_METRICS", "STOPPING_RULE", "constraint", 0.76,
    why="a guardrail breach can force a stop even before the threshold or fuel budget conditions fire")
rel("FUEL_BUDGET", "DECISION_GATE", "constraint", 0.78,
    why="remaining fuel constrains whether the gate can return continue")
rel("PROXY_VALIDITY", "OEC", "causal", 0.74,
    why="estimated proxy validity sets the weight each proxy earns in the OEC")
rel("SENSITIVITY_ANALYSIS", "DECISION_GATE", "feedback", 0.72,
    why="a decision-flip under sensitivity analysis routes back to the gate to withhold a stop verdict")
rel("METRIC_PROVENANCE", "FROZEN_METRIC", "similarity", 0.7,
    why="provenance records the rationale behind each frozen choice for principled amendment")
rel("CONSTRUCT_VALIDITY", "GOODHART", "causal", 0.72,
    why="the construct-validity audit is the held-out reference Goodhart checks the metric against")

# conflict edges (negative signed_tension + resolution_rule)
conf("IMPROVEMENT_CRITERION", "STOPPING_RULE", 0.7, -0.6,
  "the loop continues only while delta_M>=epsilon AND fuel remains AND threshold unmet; when these conflict, the stopping rule's max_cycles arm dominates to guarantee termination",
  "per-cycle improvement pressure pushes to keep iterating while the stopping rule must eventually halt the loop")
conf("OEC", "GUARDRAIL_METRICS", 0.7, -0.55,
  "accept an OEC gain only if every guardrail stays within its pre-set non-regression bound; otherwise the guardrail veto overrides the OEC improvement",
  "optimizing the OEC can push a guardrail metric past its acceptable bound")
conf("GOODHART", "IMPROVEMENT_CRITERION", 0.68, -0.5,
  "a metric gain that the construct-validity audit shows is decoupled from the construct does not count as improvement, even if delta_M>=epsilon",
  "raw metric improvement can be Goodhart gaming rather than real construct gain")
conf("MULTIPLE_COMPARISONS", "IMPROVEMENT_CRITERION", 0.66, -0.45,
  "a per-niche delta counts as a win only after Benjamini-Hochberg FDR adjustment across the declared family, not on its unadjusted value",
  "an unadjusted per-comparison improvement can be a false discovery from testing many slices")

# ---------------- conflict axes (9) ----------------
CA = [
 {"name": "frozen_metric_vs_adaptive_redefinition",
  "description": "Freezing the metric prevents gaming the target post-hoc but blocks fixing a genuinely mis-specified metric without costly change-control.",
  "poles": ["immutable_freeze", "adaptive_redefinition"], "resolution_hint": "amend only via a logged, hashed change-control step, never a silent edit",
  "tension_score": 0.74, "affected_nodes": ["FROZEN_METRIC", "METRIC_DRIFT_AUDIT", "PREREGISTRATION_RECORD"]},
 {"name": "keep_iterating_vs_terminate",
  "description": "Per-cycle improvement pressure wants to keep optimizing while the stopping rule must halt to avoid R2 non-termination.",
  "poles": ["keep_iterating", "guaranteed_termination"], "resolution_hint": "stop at threshold OR max_cycles, whichever fires first",
  "tension_score": 0.78, "affected_nodes": ["IMPROVEMENT_CRITERION", "STOPPING_RULE", "FUEL_BUDGET"]},
 {"name": "metric_optimization_vs_construct_fidelity",
  "description": "Pushing the metric harder raises Goodhart risk that the number rises while the construct it indexes falls.",
  "poles": ["maximize_metric", "preserve_construct"], "resolution_hint": "run a held-out construct audit and discount gains that decouple from it",
  "tension_score": 0.76, "affected_nodes": ["GOODHART", "PROXY_VALIDITY", "IMPROVEMENT_CRITERION"]},
 {"name": "single_OEC_vs_multi_metric_transparency",
  "description": "One OEC makes the loop decidable but hides trade-offs that separate metrics would expose.",
  "poles": ["single_OEC", "multi_metric"], "resolution_hint": "optimize one OEC but bound the hidden trade-offs with guardrail metrics",
  "tension_score": 0.68, "affected_nodes": ["OEC", "GUARDRAIL_METRICS", "MEASURE_OPERATIONALIZATION"]},
 {"name": "statistical_power_vs_false_discovery_control",
  "description": "FDR control across many niches bounds false wins but lowers power to detect real small effects.",
  "poles": ["high_power", "low_false_discovery"], "resolution_hint": "set the target FDR level before testing and accept the power trade-off",
  "tension_score": 0.66, "affected_nodes": ["MULTIPLE_COMPARISONS", "IMPROVEMENT_CRITERION", "DECISION_GATE"]},
 {"name": "epsilon_too_high_vs_too_low",
  "description": "A large epsilon discards real small gains; a small epsilon lets noise read as improvement.",
  "poles": ["large_epsilon", "small_epsilon"], "resolution_hint": "set epsilon just above the metric's estimated standard error",
  "tension_score": 0.62, "affected_nodes": ["IMPROVEMENT_CRITERION", "BASELINE_TO_BEAT", "SENSITIVITY_ANALYSIS"]},
 {"name": "tight_fuel_budget_vs_near_win",
  "description": "A tight max_cycles guarantees a hard halt but can stop a loop one cycle short of crossing the threshold.",
  "poles": ["tight_budget", "allow_near_win"], "resolution_hint": "fix max_cycles in the frozen record; a near-win at budget end is a documented non-result, not an override",
  "tension_score": 0.64, "affected_nodes": ["FUEL_BUDGET", "STOPPING_RULE", "DECISION_GATE"]},
 {"name": "convenient_proxy_vs_validated_proxy",
  "description": "A cheap available indicator speeds the loop but a validated proxy better tracks real value.",
  "poles": ["convenient_proxy", "validated_proxy"], "resolution_hint": "use the convenient proxy but weight it by its estimated true-value correlation",
  "tension_score": 0.6, "affected_nodes": ["PROXY_VALIDITY", "INDICATOR_SELECTION", "OEC"]},
 {"name": "preregistration_rigor_vs_speed",
  "description": "Full construct-validation and provenance capture before iterating is rigorous but slows a fast content pipeline.",
  "poles": ["full_rigor", "ship_fast"], "resolution_hint": "freeze the minimum decidable bundle first; deepen validity evidence in parallel",
  "tension_score": 0.6, "affected_nodes": ["CONSTRUCT_VALIDITY", "PREREGISTRATION_RECORD", "METRIC_PROVENANCE"]},
]

# ---------------- edge cases (12) ----------------
EC = [
 {"description": "The optimization loop has only a threshold stop and never reaches it, so it iterates forever (R2 non-termination).",
  "trigger": "no max_cycles fuel budget is registered alongside the threshold",
  "affected_nodes": ["STOPPING_RULE", "FUEL_BUDGET", "FROZEN_METRIC"],
  "mitigation": "register max_cycles in the frozen record so the loop halts at threshold OR budget exhaustion", "severity": "critical"},
 {"description": "Disappointing results trigger a quiet redefinition of the 'frozen' metric to relabel a miss as a win (HARKing / outcome switching).",
  "trigger": "the metric record is editable in place with no amendment log",
  "affected_nodes": ["FROZEN_METRIC", "PREREGISTRATION_RECORD"],
  "mitigation": "store the metric in an immutable hashed record; any change is a dated, logged amendment", "severity": "critical"},
 {"description": "The metric keeps rising while the underlying construct degrades: pure Goodhart capture.",
  "trigger": "the metric is optimized with no held-out construct audit",
  "affected_nodes": ["GOODHART", "CONSTRUCT_VALIDITY", "PROXY_VALIDITY"],
  "mitigation": "run a held-out construct check; divergence between metric and audit triggers review", "severity": "high"},
 {"description": "Scanning many niches finds a 'win' that is just noise surfaced by multiplicity (garden of forking paths).",
  "trigger": "per-niche p-values are read with no FDR adjustment and the family is defined after seeing winners",
  "affected_nodes": ["MULTIPLE_COMPARISONS", "DECISION_GATE"],
  "mitigation": "declare the comparison family up front and apply Benjamini-Hochberg FDR control", "severity": "high"},
 {"description": "Epsilon is set below the metric's noise, so the loop 'improves' on random fluctuations.",
  "trigger": "epsilon chosen without reference to the metric's standard error",
  "affected_nodes": ["IMPROVEMENT_CRITERION", "BASELINE_TO_BEAT"],
  "mitigation": "set epsilon above the estimated standard error and verify with sensitivity analysis", "severity": "high"},
 {"description": "A logging or pipeline change silently shifts the frozen metric, and the shift is read as a real movement.",
  "trigger": "the data feeding the frozen metric changes without drift detection",
  "affected_nodes": ["METRIC_DRIFT_AUDIT", "MEASURE_OPERATIONALIZATION"],
  "mitigation": "monitor input distribution and computation hash; a change raises a drift flag, not a metric verdict", "severity": "high"},
 {"description": "Optimizing the OEC quietly harms a metric nobody set as a guardrail (e.g. unsubscribe-rate).",
  "trigger": "no guardrail metric covers the harmed dimension",
  "affected_nodes": ["GUARDRAIL_METRICS", "OEC"],
  "mitigation": "enumerate guardrail metrics with non-regression bounds checked every cycle", "severity": "high"},
 {"description": "The baseline is recomputed each cycle, so the anchor moves and noise is mistaken for improvement.",
  "trigger": "baseline is not frozen alongside the metric",
  "affected_nodes": ["BASELINE_TO_BEAT", "FROZEN_METRIC"],
  "mitigation": "freeze naive and human baselines on the same population and never recompute per cycle", "severity": "medium"},
 {"description": "The max_cycles budget is silently extended because results 'look promising', defeating the termination guarantee.",
  "trigger": "the fuel ledger is advisory rather than enforced",
  "affected_nodes": ["FUEL_BUDGET", "STOPPING_RULE"],
  "mitigation": "enforce a monotone non-renewable counter fixed in the frozen record", "severity": "high"},
 {"description": "A win declared on one window vanishes under a slightly different reasonable specification.",
  "trigger": "the decision rests on a single arbitrary specification never stress-tested",
  "affected_nodes": ["SENSITIVITY_ANALYSIS", "IMPROVEMENT_CRITERION"],
  "mitigation": "require the stop/continue decision to hold across a pre-declared set of reasonable specifications", "severity": "medium"},
 {"description": "Two stakeholders optimize the same metric name while privately holding different constructs.",
  "trigger": "the construct was never named and validated before indicators were chosen",
  "affected_nodes": ["SUCCESS_CONSTRUCT", "CONSTRUCT_VALIDITY", "INDICATOR_SELECTION"],
  "mitigation": "name and validate the construct with an explicit not-this list before selecting indicators", "severity": "medium"},
 {"description": "A max_cycles stop is reported as if the threshold had been crossed, hiding a non-result.",
  "trigger": "the termination certificate does not distinguish which stop arm fired",
  "affected_nodes": ["TERMINATION_CERTIFICATE", "DECISION_GATE"],
  "mitigation": "the certificate must state the firing arm and the final metric vs threshold vs baseline", "severity": "medium"},
]

# ---------------- workflow (12) ----------------
WF = [
 {"action": "name_the_construct", "node_ref": "SUCCESS_CONSTRUCT",
  "description": "Turn 'the results are good' into a single named construct with a one-line definition and an explicit not-this list.",
  "artifact": "named_construct_record", "gate": "construct named and agreed by >=2 stakeholders"},
 {"action": "select_indicators", "node_ref": "INDICATOR_SELECTION",
  "description": "Choose observable indicators with a reflective/formative stance and an indicator-to-construct hypothesis each.",
  "artifact": "indicator_set", "gate": "each indicator has a stated link to the construct"},
 {"action": "validate_proxy", "node_ref": "PROXY_VALIDITY",
  "description": "Estimate how well each indicator tracks real downstream value and document breakdown conditions.",
  "artifact": "proxy_validity_report", "gate": "proxy-to-true-value correlation estimated with an interval"},
 {"action": "operationalize_measure", "node_ref": "MEASURE_OPERATIONALIZATION",
  "description": "Pin the exact estimator, population, unit and window so M is reproducible.",
  "artifact": "operational_measure", "gate": "two implementations agree on the same data"},
 {"action": "define_OEC", "node_ref": "OEC",
  "description": "Combine the proxies into one overall evaluation criterion with fixed weights and directions.",
  "artifact": "oec_definition", "gate": "OEC is a single function with pre-agreed weights"},
 {"action": "fix_baselines", "node_ref": "BASELINE_TO_BEAT",
  "description": "Compute a naive/random and a human-curated baseline on the same population as the live metric.",
  "artifact": "baseline_record", "gate": "both baselines computed on the comparison population"},
 {"action": "freeze_and_preregister", "node_ref": "FROZEN_METRIC",
  "description": "Lock measure, baseline and threshold into an immutable timestamped record before any optimization.",
  "artifact": "frozen_metric_record", "gate": "metric+baseline+threshold immutably recorded before cycle 1"},
 {"action": "set_improvement_criterion", "node_ref": "IMPROVEMENT_CRITERION",
  "description": "Set epsilon above the metric's standard error as the minimum per-cycle delta over the frozen baseline.",
  "artifact": "epsilon_definition", "gate": "epsilon exceeds the estimated standard error and is fixed"},
 {"action": "install_guardrails_and_fdr", "node_ref": "GUARDRAIL_METRICS",
  "description": "Define guardrail metrics with non-regression bounds and declare the FDR-controlled comparison family.",
  "artifact": "guardrail_and_fdr_plan", "gate": "guardrails bounded and comparison family declared up front"},
 {"action": "set_stopping_rule_and_budget", "node_ref": "STOPPING_RULE",
  "description": "Register the stop-at-threshold-OR-max_cycles rule and the enforced fuel budget so termination is bounded.",
  "artifact": "stopping_rule_record", "gate": "stop fires at threshold or max_cycles, whichever first"},
 {"action": "render_cycle_decision", "node_ref": "DECISION_GATE",
  "description": "At each cycle boundary apply the frozen rules to render a logged stop/continue/amend verdict.",
  "artifact": "cycle_decision_log", "gate": "every cycle ends with a justified verdict against the frozen rules"},
 {"action": "certify_termination", "node_ref": "TERMINATION_CERTIFICATE",
  "description": "On stop, emit a certificate stating which arm fired, final metric vs baseline vs threshold, guardrail and FDR status.",
  "artifact": "termination_certificate", "gate": "firing arm and final comparison recorded; R2 closed"},
]

# ---------------- dominance rules (9) ----------------
DR = [
 {"rule": "The metric, baseline and threshold must be frozen in an immutable record before the first optimization cycle",
  "rationale": "an editable target lets results redraw the goal post-hoc (HARKing)",
  "trigger": "optimization begins without an immutable frozen record",
  "action": "block iteration until the metric+baseline+threshold are immutably registered"},
 {"rule": "Every optimization loop must register a max_cycles fuel budget alongside its threshold",
  "rationale": "a threshold-only loop can iterate forever if the threshold is never reached (R2 non-termination)",
  "trigger": "a loop is configured with a threshold but no max_cycles",
  "action": "require a max_cycles budget so the stopping rule halts at threshold OR budget exhaustion"},
 {"rule": "An OEC gain must not be accepted if it breaches any guardrail's non-regression bound",
  "rationale": "winning on the primary metric by harming a guardrail is a net loss",
  "trigger": "a candidate improves the OEC while a guardrail regresses past its bound",
  "action": "reject the candidate and route to guardrail review"},
 {"rule": "A per-niche win must clear Benjamini-Hochberg FDR adjustment before it is declared",
  "rationale": "unadjusted multiplicity manufactures false wins across many slices",
  "trigger": "a slice is declared a win on an unadjusted p-value",
  "action": "apply FDR control over the declared family before declaring any win"},
 {"rule": "Epsilon must exceed the metric's estimated standard error before iteration starts",
  "rationale": "an epsilon below the noise floor declares random walk steps as improvement",
  "trigger": "epsilon is set without a noise estimate",
  "action": "block freeze until epsilon is shown to exceed the standard error"},
 {"rule": "A metric gain that diverges from the held-out construct audit must not count as improvement",
  "rationale": "Goodhart gaming inflates the metric while the construct falls",
  "trigger": "the metric rises while the construct audit drops",
  "action": "discount the gain and trigger a Goodhart review"},
 {"rule": "Any change to a frozen element must be a logged, hashed amendment, never a silent edit",
  "rationale": "silent edits make the pre-registration claim unfalsifiable",
  "trigger": "a frozen value is changed in place",
  "action": "reject the edit and require an appended amendment with rationale"},
 {"rule": "The termination certificate must distinguish a threshold stop from a max_cycles stop",
  "rationale": "a budget-exhaustion stop reported as a threshold success hides a non-result",
  "trigger": "a certificate omits which stop arm fired",
  "action": "require the firing arm and final metric/baseline/threshold in the certificate"},
 {"rule": "The construct must be named and validated before indicators are selected",
  "rationale": "choosing indicators first lets convenience define success and hides stakeholder disagreement",
  "trigger": "indicators are proposed before a named, validated construct exists",
  "action": "block indicator selection until the construct is named with a not-this list"},
]

# ---------------- anti-rework rules (8) ----------------
ARR = [
 {"rule": "Do not start optimization before the metric is frozen; redefining it mid-loop invalidates every prior cycle's comparison",
  "prevents": "re-running all cycles after a late metric redefinition"},
 {"rule": "Do not run a loop without a max_cycles budget; retrofitting a stopping rule onto a runaway loop wastes the spent fuel",
  "prevents": "emergency halt and re-design after a non-terminating loop burns the compute budget"},
 {"rule": "Do not recompute the baseline each cycle; a moving anchor forces re-deriving every prior delta",
  "prevents": "re-evaluating all historical improvements against a shifting baseline"},
 {"rule": "Do not set epsilon below the metric's noise; discovering the loop optimized noise forces discarding the run",
  "prevents": "throwing away cycles that 'improved' only random fluctuations"},
 {"rule": "Do not read per-niche wins without FDR control; un-adjusting later means re-testing the whole family",
  "prevents": "re-litigating every slice result after a false-discovery scandal"},
 {"rule": "Do not omit guardrails and add them post-hoc; a late guardrail breach forces re-running optimization",
  "prevents": "re-optimizing after discovering collateral harm no guardrail caught"},
 {"rule": "Do not edit a frozen element in place; reconstructing the original for audit requires re-deriving the history",
  "prevents": "forensic reconstruction of a metric's true frozen value"},
 {"rule": "Do not select indicators before naming the construct; a late construct change invalidates the indicator-to-construct links",
  "prevents": "re-selecting and re-validating indicators after the construct is renamed"},
]

# ---------------- iteration protocol (8) ----------------
IP = [
 {"trigger": "the optimization loop fails to terminate or has no registered max_cycles",
  "action": "register a max_cycles fuel budget in FUEL_BUDGET and wire it into STOPPING_RULE so the loop halts at threshold OR budget",
  "nodes": ["STOPPING_RULE", "FUEL_BUDGET", "FROZEN_METRIC"], "priority": "critical"},
 {"trigger": "the frozen metric is proposed for redefinition after seeing results",
  "action": "route the change through a logged, hashed amendment in PREREGISTRATION_RECORD instead of a silent edit to FROZEN_METRIC",
  "nodes": ["FROZEN_METRIC", "PREREGISTRATION_RECORD"], "priority": "critical"},
 {"trigger": "the metric and the held-out construct audit diverge",
  "action": "trigger a Goodhart review in GOODHART and re-check PROXY_VALIDITY before counting further gains",
  "nodes": ["GOODHART", "PROXY_VALIDITY", "CONSTRUCT_VALIDITY"], "priority": "high"},
 {"trigger": "wins are being declared across many niches without multiplicity control",
  "action": "declare the comparison family and apply Benjamini-Hochberg FDR control in MULTIPLE_COMPARISONS at the DECISION_GATE",
  "nodes": ["MULTIPLE_COMPARISONS", "DECISION_GATE"], "priority": "high"},
 {"trigger": "a guardrail metric breaches its non-regression bound",
  "action": "reject the OEC gain in GUARDRAIL_METRICS and re-render the cycle verdict at DECISION_GATE",
  "nodes": ["GUARDRAIL_METRICS", "OEC", "DECISION_GATE"], "priority": "high"},
 {"trigger": "a drift in the frozen metric's inputs is detected",
  "action": "raise a drift flag in METRIC_DRIFT_AUDIT and hold the comparison to the frozen baseline until resolved",
  "nodes": ["METRIC_DRIFT_AUDIT", "MEASURE_OPERATIONALIZATION", "FROZEN_METRIC"], "priority": "medium"},
 {"trigger": "the stop/continue decision flips under a reasonable alternative specification",
  "action": "run SENSITIVITY_ANALYSIS and withhold the stop verdict at DECISION_GATE until the decision is robust",
  "nodes": ["SENSITIVITY_ANALYSIS", "IMPROVEMENT_CRITERION", "DECISION_GATE"], "priority": "medium"},
 {"trigger": "the loop stops without a clear record of which arm fired",
  "action": "emit a TERMINATION_CERTIFICATE distinguishing a threshold stop from a max_cycles stop with the final comparison",
  "nodes": ["TERMINATION_CERTIFICATE", "DECISION_GATE", "FUEL_BUDGET"], "priority": "medium"},
]

spec = {
 "domain": "metric__frozen_preregistration",
 "domain_label": "Frozen Pre-Registered Success Metric & Stopping Rule",
 "purpose": "turn_results_are_good_into_a_named_construct_indicator_measure_with_a_baseline_to_beat_an_improvement_criterion_and_a_terminating_stopping_rule",
 "assumptions": [
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "scope is the metric-and-stopping-rule design itself; the optimization engine that proposes candidates is delegated to sibling KBs",
   "an eval loop exists that repeatedly scores candidate outputs against the metric and can be halted by a stopping rule",
   "the team can compute a naive baseline, a human-curated baseline, and the metric's standard error before freezing",
 ],
 "exclusions": [
   "the candidate-generation / optimization algorithm itself (delegated to the generation engine KBs)",
   "causal-inference design of randomized experiments beyond OEC and guardrail definition",
   "long-term post-deployment monitoring after the loop terminates",
   "automated learning of the construct from data without human naming",
 ],
 "source_description": "heuristic prior estimates for frozen pre-registered success-metric and stopping-rule work units, grounded in construct validity, online controlled experimentation, multiple-comparison control, and preregistration practice; no supplied dataset",
 "source_citation": "Goodhart 1975 (Goodhart's law) and Strathern 1997 'Improving ratings: audit in the British University system'; Benjamini & Hochberg 1995 'Controlling the False Discovery Rate' (JRSS-B); Kohavi, Tang & Xu 2020 'Trustworthy Online Controlled Experiments' (OEC, guardrail metrics); Cronbach & Meehl 1955 'Construct Validity in Psychological Tests' (Psychological Bulletin); Nosek et al. 2018 'The preregistration revolution' (PNAS)",
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
 "priority_rationale": "SUCCESS_CONSTRUCT/CONSTRUCT_VALIDITY/INDICATOR_SELECTION are foundational; MEASURE/OEC/BASELINE feed the freeze; FROZEN_METRIC plus IMPROVEMENT_CRITERION plus STOPPING_RULE plus FUEL_BUDGET are the termination core that closes R2; guardrails, drift, sensitivity and the decision gate gate correctness; the termination certificate closes the loop.",
 "eval_objective": "verify_construct_naming_metric_freezing_baseline_anchoring_improvement_criterion_guardrails_fdr_control_and_a_terminating_stopping_rule_of_metric__frozen_preregistration_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "metric__frozen_preregistration.spec.json")
open(path, "w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes", len(N), "edges", len(E), "CA", len(CA), "EC", len(EC), "WF", len(WF),
      "CQ", len(CQS), "DR", len(DR), "ARR", len(ARR), "IP", len(IP))
