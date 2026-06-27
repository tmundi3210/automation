#!/usr/bin/env python3
"""Generate the auth__inauthenticity_detection content spec (B60 KB) for kb_forge.py.

Domain: Bot / Astroturf / Coordinated-Inauthentic-Behavior (CIB) detection from
PUBLIC signals alone. Core scope finding (FP4): with no platform-internal labels,
the output MUST be a CALIBRATED PROBABILITY + provenance, never a clean boolean
verdict. Grounded in:
  - Ferrara et al. 2016, "The Rise of Social Bots", Comm. ACM 59(7)
  - Davis et al. 2016, "BotOrNot: A System to Evaluate Social Bots" (Botometer), WWW
  - Ratkiewicz et al. 2011, "Detecting and Tracking Political Abuse in Social Media"
    (Truthy / astroturf), ICWSM
  - Cresci et al. 2017, "The Paradigm-Shift of Social Spambots", WWW companion
  - Meta Adversarial Threat / Coordinated Inauthentic Behavior reports & Graphika
    network analyses
  - Platt 1999, "Probabilistic Outputs for Support Vector Machines" (Platt scaling)
  - Zadrozny & Elkan 2002 isotonic calibration; Niculescu-Mizil & Caruana 2005

Compact authoring: node() applies sane defaults so only domain content + base
metric magnitudes are specified per node."""
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
        "academic_fields": ["computational_social_science", "network_science"],
        "subfields": subfields or ["social_bot_detection", "coordinated_inauthentic_behavior"],
        "specialists": specialists or ["disinformation_analyst"],
        "contradictors": contradictors or ["naive_bot_or_not_classifier_advocate"],
        "inputs": inputs or ["public account/post signals", "engagement timeline"],
        "outputs": outputs or ["calibrated inauthenticity probability with provenance"],
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

# ---------------- FOUNDATIONS ----------------
N.append(node("PROBLEM_FRAMING","calibrated_probability_not_boolean_verdict",
  "Frame the task as estimating a calibrated probability P(inauthentic | public signals) for a visible reaction (a wave of comments/likes/shares), never emitting a clean boolean 'bot'/'human' verdict, because public data carries no platform-internal ground truth (IP, device, login graph) that a verdict would require.",
  "foundations", [], ["CQ_01"],
  b(0.95,0.85,0.82,0.55,0.9,0.85,0.78,0.66,0.5, 0.74,0.8, 0.92,0.66,0.18,0.6,[0.22,0.55],"product asks for a hard label, or platform-internal signals become available"),
  ["probability target [0,1]","scope = visible reaction, not a single account's identity","provenance requirement attached to every estimate"],
  ["asserting a named account IS a bot","platform-side enforcement decisions"],
  [pro("A probability + provenance is defensible and survives appeal; a boolean verdict on public-only signals is not","Meta/Graphika CIB reports describe 'coordinated behavior', not per-account guilt verdicts")],
  [con("Probabilities are harder for downstream consumers to act on than a clean yes/no","a dashboard wants a single bot flag, not a 0.63")],
  ["product pressure collapses the probability into a hard threshold and loses calibration","framing as account-level guilt invites defamation risk"],
  ["output schema is a probability in [0,1] plus a provenance object; no boolean verdict field exists"],
  ["a consumer requests a boolean label","scope creeps from reaction-level to account-identity-level"],
  specialists=["disinformation_analyst","trust_and_safety_lead"],
  contradictors=["hard_label_product_manager"]))

N.append(node("NO_GROUND_TRUTH","absence_of_platform_internal_labels",
  "Encode the core epistemic limit: public signals lack the platform-internal labels (account-creation IP, device fingerprint, phone verification, login co-location, suspension history) that alone could confirm inauthenticity, so any output is an estimate under uncertainty, not a measurement.",
  "foundations", ["PROBLEM_FRAMING"], ["CQ_01","CQ_02"],
  b(0.93,0.8,0.78,0.5,0.88,0.82,0.72,0.62,0.48, 0.72,0.78, 0.9,0.62,0.2,0.58,[0.24,0.6],"platform grants labelled data, or a new public signal proxies an internal label"),
  ["enumerate which confirming signals are private","estimate, never measure","uncertainty is irreducible from public data alone"],
  ["claiming access to internal platform telemetry","legal attribution to a named operator"],
  [pro("Stating the ground-truth gap up front prevents over-claiming and sets honest confidence bounds","Ferrara 2016 notes bot detection has no clean public oracle")],
  [con("Acknowledged uncertainty can be weaponized to dismiss real, well-evidenced CIB findings","'you can't prove it' used to wave away a synchronized campaign")],
  ["treating a strong public proxy as if it were confirmed ground truth","silent assumption that absence of a signal means authenticity"],
  ["every estimate documents which confirming signals were unavailable and were therefore proxied"],
  ["a public signal is found to reliably proxy an internal label","platform partnership supplies partial labels"],
  specialists=["disinformation_analyst","research_methodologist"]))

N.append(node("SIGNAL_INVENTORY","public_signal_catalog_and_provenance",
  "Maintain the catalog of observable public signals (account metadata, post text, timing series, engagement counts, follower graph) with, for each, its collection method, freshness, and reliability, so every downstream feature is traceable to a logged, time-stamped source.",
  "foundations", ["NO_GROUND_TRUTH"], ["CQ_02","CQ_03"],
  b(0.85,0.78,0.72,0.6,0.78,0.82,0.6,0.68,0.45, 0.78,0.8, 0.84,0.68,0.18,0.5,[0.22,0.54],"a platform API deprecates a field or rate-limits a signal source"),
  ["signal-to-collection-method map","freshness and reliability per signal","provenance log per observation"],
  ["feature engineering math","model training"],
  [pro("A provenance-first inventory means every probability can be traced back to logged raw signals for audit and appeal","Botometer documents exactly which public features it ingests")],
  [con("Public APIs change without notice, silently degrading signals the catalog assumes are present","Twitter/X API tier changes removed free timeline access")],
  ["a deprecated API field feeds stale or empty features unnoticed","sampling bias from rate-limited collection skews the signal set"],
  ["every signal in use has a recorded collection method, timestamp, and reliability rating"],
  ["an API field is deprecated or rate-limited","a new public signal source is added to the catalog"],
  specialists=["data_engineer","disinformation_analyst"]))

# ---------------- SIGNAL FAMILIES ----------------
N.append(node("ACCOUNT_FEATURES","account_level_anomaly_features",
  "Extract per-account features that historically separate inauthentic from organic accounts: account age, follower/following ratio anomalies, default/AI-generated avatars, missing bio, high posting volume, and screen-name entropy (random suffixes), per Botometer's account-feature family.",
  "signals", ["SIGNAL_INVENTORY"], ["CQ_03","CQ_04"],
  b(0.82,0.74,0.7,0.6,0.76,0.74,0.55,0.66,0.5, 0.76,0.78, 0.82,0.66,0.22,0.5,[0.24,0.58],"a new account-creation cohort or avatar-generation technique shifts feature distributions"),
  ["age / follower-following ratio / avatar / bio / name-entropy features","per-account anomaly scoring"],
  ["coordination across accounts","text duplication detection"],
  [pro("Account features are cheap, fast, and the most studied bot signal family since Davis 2016 Botometer","a 3-day-old account following 5000 with 0 followers and a default egg avatar")],
  [con("Account features alone yield high false positives on legitimate new users, brands, and low-effort real people","a new real user with a default avatar and no bio looks bot-like")],
  ["over-weighting age penalizes legitimate new users","avatar heuristics break as default avatars and GAN faces evolve"],
  ["account features are computed for each account with documented anomaly thresholds, not hard cutoffs"],
  ["follower/following norms shift on the platform","a feature's discriminative power decays in validation"],
  specialists=["disinformation_analyst","ml_engineer"]))

N.append(node("TEMPLATED_TEXT","near_duplicate_copypasta_detection",
  "Detect near-duplicate / templated text across the reaction: copypasta, slot-filled templates, and high textual similarity clusters, using shingling/MinHash and edit-distance, a hallmark of spambot and astroturf content per Ratkiewicz 2011 (Truthy) and Cresci 2017.",
  "signals", ["SIGNAL_INVENTORY"], ["CQ_04","CQ_05"],
  b(0.83,0.74,0.72,0.66,0.78,0.74,0.55,0.66,0.52, 0.76,0.78, 0.82,0.66,0.22,0.5,[0.24,0.58],"template-evasion (spinning/paraphrase/LLM rewriting) raises text diversity"),
  ["shingle/MinHash near-dup clustering","slot-filled template detection","cross-post similarity matrix"],
  ["timing analysis","account metadata"],
  [pro("Near-duplicate clustering is a strong, interpretable astroturf signal and was central to Truthy's meme-injection detection","50 replies sharing the same 12-word template with only the @handle swapped")],
  [con("LLM-paraphrased or 'spun' content defeats exact and near-duplicate matching, inflating false negatives","a campaign uses a model to rewrite each post uniquely")],
  ["paraphrase/spinning evades n-gram similarity","legitimate viral catchphrases trigger false copypasta flags"],
  ["near-duplicate clusters are reported with similarity scores and the matching method recorded"],
  ["paraphrase-based evasion observed in validation","a benign viral phrase causes a false-positive cluster"],
  specialists=["nlp_engineer","disinformation_analyst"]))

N.append(node("COORDINATION_SIGNALS","synchronized_timing_and_burst_detection",
  "Detect coordination in the temporal dimension: synchronized posting bursts, abnormally tight inter-arrival times, lockstep activity windows, and co-action (same accounts acting together repeatedly), the defining property of Coordinated Inauthentic Behavior in Meta/Graphika reports.",
  "signals", ["SIGNAL_INVENTORY"], ["CQ_05","CQ_06"],
  b(0.88,0.8,0.76,0.72,0.84,0.82,0.6,0.62,0.58, 0.72,0.76, 0.88,0.62,0.24,0.56,[0.28,0.64],"coordination definition or burst-window parameters change, or schedulers add jitter"),
  ["burst/inter-arrival timing analysis","lockstep window detection","repeated co-action across reactions"],
  ["per-account static features","network reciprocity structure (handled by NETWORK_STRUCTURE)"],
  [pro("Synchronized timing is the most defensible CIB signal because organic crowds do not act in millisecond lockstep","200 accounts all post within a 90-second window on a 3am local time")],
  [con("Real coordinated-but-authentic activity (fandoms, activism, breaking news) also produces synchronized bursts","K-pop fans organically mass-reply on cue")],
  ["legitimate organized communities are flagged as inauthentic coordination","injected timing jitter defeats tight-window detectors"],
  ["coordination is reported as a co-activity score over a defined window with the window parameters logged"],
  ["bursts traced to an authentic organized community","schedulers add jitter that defeats the current window"],
  specialists=["network_scientist","disinformation_analyst"]))

N.append(node("NETWORK_STRUCTURE","sybil_ring_and_reciprocal_cluster_detection",
  "Analyze the follower/interaction graph for inauthentic structure: dense reciprocal follow clusters, sybil rings, star/hub amplification patterns, and unusually high clustering coefficients among the reacting accounts, drawing on Ratkiewicz 2011 diffusion-network analysis and Graphika cluster maps.",
  "signals", ["SIGNAL_INVENTORY","COORDINATION_SIGNALS"], ["CQ_06","CQ_07"],
  b(0.86,0.78,0.74,0.74,0.83,0.84,0.62,0.6,0.58, 0.72,0.74, 0.86,0.6,0.24,0.56,[0.28,0.66],"graph-construction policy or the reciprocity/clustering thresholds change"),
  ["reciprocal cluster detection","sybil-ring / dense-subgraph mining","hub-amplification pattern detection"],
  ["temporal burst timing","text content"],
  [pro("Network structure exposes amplification machinery that single-account features miss, central to Graphika's CIB takedowns","a ring of 80 accounts that mutually follow each other and only amplify one source")],
  [con("Public graph data is partial and sampled, so absent edges can fake or hide a ring","API limits hide most of the real follower edges")],
  ["sampled/partial graph yields spurious or missed clusters","legitimate tight-knit communities resemble sybil rings"],
  ["network structure metrics (reciprocity, clustering, dense-subgraph membership) are computed on the logged subgraph"],
  ["graph sampling coverage drops below a usable threshold","a benign community matches a sybil-ring signature"],
  specialists=["network_scientist","graph_engineer"]))

N.append(node("ENGAGEMENT_MISMATCH","engagement_ratio_disproportion_detection",
  "Detect implausible ratios among engagement types: views-to-likes, likes-to-comments, comment-to-share, and reach-to-conversation, where bot amplification typically inflates one channel (e.g. likes/views) far out of proportion to the genuine-discussion channels.",
  "signals", ["SIGNAL_INVENTORY","ACCOUNT_FEATURES"], ["CQ_07","CQ_08"],
  b(0.8,0.74,0.72,0.62,0.78,0.74,0.55,0.66,0.5, 0.76,0.78, 0.8,0.66,0.22,0.5,[0.24,0.58],"platform changes how engagement counts are computed or displayed"),
  ["cross-channel engagement ratios","baseline-vs-observed ratio deviation","disproportion scoring"],
  ["timing synchronization","graph structure"],
  [pro("Ratio disproportion is platform-agnostic and resistant to per-account obfuscation because it aggregates the whole reaction","100k likes with 4 comments signals purchased likes, not a real conversation")],
  [con("Baseline 'normal' ratios vary wildly by topic, format, and audience, so a fixed baseline mislabels organic outliers","a meme legitimately gets huge likes and almost no comments")],
  ["topic/format-dependent baselines mislabel organic outliers","platform reach-metric changes invalidate stored baselines"],
  ["engagement ratios are compared to a topic/format-matched baseline and the baseline source is recorded"],
  ["platform changes engagement metric definitions","an organic format produces a false disproportion flag"],
  specialists=["data_analyst","disinformation_analyst"]))

# ---------------- AGGREGATION & MODELING ----------------
N.append(node("FEATURE_FUSION","weighted_multi_signal_fusion",
  "Fuse the signal families (account, text, timing, network, engagement) into a single inauthenticity score via a documented weighting/model, keeping per-family contributions inspectable so the final score can be decomposed into its evidence sources.",
  "modeling", ["ACCOUNT_FEATURES","TEMPLATED_TEXT","COORDINATION_SIGNALS","NETWORK_STRUCTURE","ENGAGEMENT_MISMATCH"], ["CQ_08","CQ_09"],
  b(0.88,0.82,0.76,0.74,0.84,0.85,0.62,0.6,0.55, 0.72,0.74, 0.88,0.6,0.24,0.56,[0.28,0.64],"a signal family is added/removed or its reliability rating changes the fusion weights"),
  ["per-family weighting","score decomposition by evidence source","fusion model (ensemble/logistic)"],
  ["calibration to a probability (handled by CALIBRATION)","threshold/routing"],
  [pro("Multi-signal fusion is far more robust than any single family, matching Botometer's ensemble-over-feature-classes design","timing + network + text agreeing raises confidence beyond any one signal")],
  [con("Fusion can launder a weak, correlated set of signals into a falsely confident composite score","five features that all derive from the same burst double-count one event")],
  ["correlated signals double-count one underlying event","opaque weights make the score uninterpretable and unappealable"],
  ["the fused score decomposes into per-family contributions that sum/trace back to logged signals"],
  ["a signal family is added or removed","family reliability ratings are revised"],
  specialists=["ml_engineer","disinformation_analyst"]))

N.append(node("CALIBRATION","probability_calibration_platt_isotonic",
  "Map the raw fused score to a calibrated probability using Platt scaling (Platt 1999) or isotonic regression (Zadrozny & Elkan 2002) fit on the labeled validation set, and report the reliability diagram plus FPR/TPR at operating points so 0.7 means roughly 70% empirical inauthenticity.",
  "modeling", ["FEATURE_FUSION","VALIDATION_SET"], ["CQ_09","CQ_10"],
  b(0.92,0.84,0.8,0.74,0.88,0.84,0.66,0.6,0.55, 0.7,0.74, 0.92,0.6,0.24,0.6,[0.3,0.68],"the validation label distribution shifts or the base rate of the deployment population changes"),
  ["Platt/isotonic fit","reliability diagram (calibration curve)","FPR/TPR at operating points"],
  ["raw score fusion","the no-data confirmation of ground truth"],
  [pro("Calibration makes the probability semantically meaningful and comparable across reactions, per Niculescu-Mizil 2005","after Platt scaling a 0.8 cohort is empirically ~80% inauthentic in validation")],
  [con("Calibration is only valid for populations matching the validation set; base-rate shift silently miscalibrates","a model calibrated on bot-heavy data is overconfident on a clean feed")],
  ["base-rate shift between validation and deployment breaks calibration","tiny or skewed validation sets yield unstable calibration curves"],
  ["a reliability diagram is produced and the calibrated probability's empirical accuracy is reported with FPR/TPR"],
  ["deployment base rate diverges from the validation set","the reliability diagram shows systematic miscalibration"],
  specialists=["ml_engineer","statistician"]))

N.append(node("UNCERTAINTY_QUANT","confidence_interval_on_probability",
  "Attach an uncertainty interval to the point probability (e.g. via bootstrap over signals or Bayesian posterior) so a 0.6 estimated from sparse, noisy public signals is reported with a wide band, distinguishing 'confidently uncertain' from 'uncertainly confident'.",
  "modeling", ["CALIBRATION"], ["CQ_10","CQ_11"],
  b(0.82,0.74,0.74,0.7,0.8,0.78,0.58,0.62,0.5, 0.74,0.76, 0.82,0.62,0.22,0.52,[0.26,0.6],"the signal-sparsity regime or interval method changes"),
  ["bootstrap/Bayesian interval on the probability","sparsity-aware widening","interval reporting alongside the point estimate"],
  ["point-estimate calibration","downstream thresholding"],
  [pro("An interval prevents over-trusting a point estimate built on a handful of public signals","P=0.6 [0.35,0.82] tells a reviewer not to act on the point alone")],
  [con("Intervals add cognitive load and are easy for consumers to ignore or misread","a reviewer reads only the 0.6 and ignores the wide band")],
  ["wide intervals are dropped by consumers who read only the point estimate","interval method understates uncertainty under correlated signals"],
  ["each probability ships with an uncertainty interval whose width grows as signal sparsity grows"],
  ["signal sparsity regime changes","consumers are observed ignoring the interval"],
  specialists=["statistician","disinformation_analyst"]))

N.append(node("BASELINE_REFERENCE","organic_baseline_distribution_modeling",
  "Build topic/format/audience-matched baselines of what organic timing, engagement ratios, and account mixes look like, so anomaly is measured as deviation from a like-for-like organic reference rather than against a single global norm that mislabels legitimate outliers.",
  "modeling", ["COORDINATION_SIGNALS","ENGAGEMENT_MISMATCH"], ["CQ_07","CQ_08"],
  b(0.82,0.76,0.74,0.66,0.8,0.8,0.58,0.62,0.52, 0.74,0.76, 0.82,0.62,0.22,0.52,[0.26,0.6],"a platform format change or audience shift moves the organic reference distribution"),
  ["topic/format-matched organic baselines","deviation-from-baseline scoring","reference versioning"],
  ["the calibration map (handled by CALIBRATION)","decision routing"],
  [pro("Like-for-like organic baselines stop fixed global norms from flagging naturally skewed organic content","a meme is compared to other memes, not to news-article engagement")],
  [con("Good organic baselines are themselves contaminated by undetected inauthenticity, biasing the reference","the 'organic' baseline already contains unflagged bot activity")],
  ["a contaminated baseline normalizes away real inauthenticity","too-narrow baselines have too little data to be stable"],
  ["anomaly scores are computed as deviation from a topic/format-matched, versioned organic baseline"],
  ["a platform format change shifts organic norms","a baseline is found to be contaminated by inauthentic activity"],
  specialists=["data_analyst","statistician"]))

N.append(node("CROSS_PLATFORM_CORROBORATION","multi_platform_campaign_linkage",
  "Corroborate a suspected inauthentic reaction across platforms and surfaces: shared shortened URLs, identical media hashes, synchronized cross-site posting, and reused infrastructure, the linkage method Graphika and Meta use to attribute a single coordinated operation spanning multiple networks.",
  "validation", ["NETWORK_STRUCTURE","COORDINATION_SIGNALS"], ["CQ_06"],
  b(0.8,0.76,0.72,0.74,0.82,0.84,0.62,0.58,0.55, 0.7,0.74, 0.8,0.58,0.26,0.56,[0.3,0.68],"a platform restricts cross-site signals or the operation changes its shared infrastructure"),
  ["shared-URL/media-hash linkage","cross-site synchronized-posting detection","reused-infrastructure clustering"],
  ["single-platform scoring","legal attribution to a named operator"],
  [pro("Cross-platform linkage corroborates single-platform suspicion and is central to Graphika/Meta CIB attribution","the same shortened URL seeded simultaneously across four networks reveals one operation")],
  [con("Cross-platform data is even sparser and harder to join reliably, raising both false links and missed links","a coincidental URL reshare is mistaken for coordinated cross-posting")],
  ["coincidental cross-site overlap mistaken for one operation","missing cross-platform data hides a multi-network campaign"],
  ["cross-platform links are reported with the shared artifact (URL/hash/infrastructure) and a join-confidence recorded"],
  ["a platform restricts cross-site signals","a coincidental overlap produces a false cross-platform link"],
  specialists=["disinformation_analyst","network_scientist"]))

# ---------------- VALIDATION & ADVERSARIAL ----------------
N.append(node("VALIDATION_SET","labeled_cib_benchmark_curation",
  "Curate and version a benchmark of labeled accounts/reactions from known CIB datasets (Cresci 2017 social-spambot corpora, Botometer's annotated sets, published Meta/Graphika takedown account lists) to measure detector FPR/TPR and to fit calibration, acknowledging label provenance and staleness.",
  "validation", ["SIGNAL_INVENTORY"], ["CQ_11","CQ_12"],
  b(0.88,0.8,0.76,0.66,0.86,0.8,0.7,0.6,0.5, 0.74,0.78, 0.88,0.6,0.22,0.58,[0.28,0.64],"a new labeled CIB dataset is released or existing labels are revised/retracted"),
  ["versioned labeled benchmark","label provenance and date","FPR/TPR measurement harness"],
  ["live deployment scoring","calibration math (handled by CALIBRATION)"],
  [pro("A versioned labeled benchmark is the only way to quote FPR/TPR honestly and to fit calibration, as Cresci 2017 enabled","Cresci's social-spambot corpora became a standard CIB benchmark")],
  [con("Public CIB labels are stale and platform-specific, so benchmark performance overstates live accuracy","2017 spambots look nothing like 2024 LLM-driven accounts")],
  ["stale labels make the benchmark unrepresentative of current adversaries","label leakage between train and test inflates reported accuracy"],
  ["the benchmark is versioned with label sources, dates, and train/test splits documented to prevent leakage"],
  ["a new CIB dataset is published","benchmark accuracy diverges from spot-checked live performance"],
  specialists=["research_methodologist","disinformation_analyst"]))

N.append(node("ADVERSARIAL_ADAPTATION","detector_decay_and_evasion_modeling",
  "Model the adversarial arms race: bot operators evolve (LLM-generated text, jittered timing, aged accounts, residential proxies) so any fixed detector decays; track per-signal decay and schedule re-fitting, per Cresci 2017's 'paradigm shift' and Ferrara 2016's evolving-bots thesis.",
  "validation", ["VALIDATION_SET","FEATURE_FUSION"], ["CQ_12","CQ_13"],
  b(0.86,0.78,0.74,0.74,0.85,0.82,0.66,0.56,0.6, 0.7,0.72, 0.86,0.56,0.28,0.58,[0.32,0.72],"a new evasion technique is observed in the wild or a signal's TPR drops over time"),
  ["per-signal decay tracking","evasion-technique catalog","re-fit scheduling"],
  ["the static feature math itself","platform enforcement"],
  [pro("Explicitly modeling decay prevents silent confidence in a detector the adversary already beat, the central lesson of Cresci 2017","social spambots evaded all detectors that worked on the previous bot generation")],
  [con("Chasing every evasion technique causes overfitting to the last attack and detector instability","retuning weekly to the newest trick degrades generalization")],
  ["detector decays silently as adversaries adapt and no one notices","over-frequent re-fitting overfits to the most recent campaign"],
  ["per-signal TPR is tracked over time and a re-fit is triggered when any signal's discriminative power decays below threshold"],
  ["a novel evasion technique is reported","a signal's measured TPR drops over successive validation runs"],
  specialists=["disinformation_analyst","adversarial_ml_researcher"],
  contradictors=["static_detector_advocate"]))

# ---------------- DECISION & DOWNSTREAM ----------------
N.append(node("FALSE_POSITIVE_COST","asymmetric_mislabeling_cost_model",
  "Encode the asymmetric harm of false positives: wrongly labeling a real user or community as inauthentic causes reputational, legal, and chilling-effect harm that usually outweighs missing some bots, so the decision policy is FP-averse and the cost asymmetry is explicit.",
  "decision", ["UNCERTAINTY_QUANT","NO_GROUND_TRUTH"], ["CQ_13","CQ_14"],
  b(0.9,0.82,0.84,0.6,0.9,0.82,0.74,0.62,0.55, 0.74,0.78, 0.9,0.62,0.2,0.66,[0.26,0.62],"the deployment context changes the relative cost of false positives vs false negatives"),
  ["explicit FP vs FN cost weights","FP-averse decision policy","harm documentation per action class"],
  ["the probability estimate itself","model fitting"],
  [pro("Making FP cost explicit forces conservative action on weak evidence and protects real users from defamation","flagging an activist network as 'bots' on thin evidence causes real-world harm")],
  [con("Strong FP aversion lets genuine large coordinated campaigns pass under-actioned","an FP-averse policy under-flags a real influence operation")],
  ["uniform error cost mislabels real users at scale","over-aversion lets coordinated campaigns run unchallenged"],
  ["the decision policy uses explicit, asymmetric FP/FN costs and documents the harm of each possible action"],
  ["deployment stakes change the cost asymmetry","a false-positive incident harms a real user or community"],
  specialists=["trust_and_safety_lead","ethicist"]))

N.append(node("HUMAN_REVIEW_THRESHOLD","routing_band_for_human_adjudication",
  "Define probability bands that route cases: low band auto-clears, high band with low uncertainty escalates, and the ambiguous middle (and any high-uncertainty case) routes to a human reviewer; no automated action is taken on a public-only probability without this gate when stakes are high.",
  "decision", ["FALSE_POSITIVE_COST","UNCERTAINTY_QUANT"], ["CQ_14"],
  b(0.88,0.8,0.82,0.62,0.88,0.8,0.7,0.62,0.55, 0.76,0.8, 0.88,0.62,0.2,0.6,[0.24,0.58],"the action stakes, FP cost weights, or reviewer capacity change"),
  ["probability/uncertainty routing bands","human-in-the-loop gate for high-stakes actions","auto-clear and auto-escalate rules"],
  ["the reviewer's adjudication UI","enforcement mechanics"],
  [pro("A review band keeps a human accountable for high-stakes calls on probabilistic, public-only evidence","P>0.85 with a tight interval escalates; 0.4-0.7 always gets human eyes")],
  [con("Review capacity is finite, so wide review bands create backlogs and pressure to widen auto-action","a flood of ambiguous cases overwhelms the review queue")],
  ["band boundaries set without cost grounding mis-route cases","review backlog pressures unsafe auto-actioning"],
  ["routing bands are defined from FP/FN costs and uncertainty, and high-stakes actions require human review"],
  ["reviewer capacity changes","FP cost weights are revised"],
  specialists=["trust_and_safety_lead","operations_lead"]))

N.append(node("CONFIDENCE_PROPAGATION","authenticity_discount_to_downstream_hype",
  "Propagate the calibrated authenticity probability downstream so that any hype/virality/trend metric computed over the reaction is discounted by its estimated inauthentic fraction, preventing inflated reach numbers from feeding decisions as if organic.",
  "decision", ["CALIBRATION","HUMAN_REVIEW_THRESHOLD"], ["CQ_09","CQ_14"],
  b(0.84,0.82,0.78,0.66,0.82,0.86,0.6,0.6,0.55, 0.74,0.76, 0.84,0.6,0.22,0.54,[0.28,0.64],"a downstream consumer changes how it ingests the authenticity discount"),
  ["inauthentic-fraction discount on hype metrics","propagation of the probability and its interval downstream","flagging of discounted vs raw metrics"],
  ["the upstream estimation math","platform enforcement"],
  [pro("Discounting hype by estimated inauthentic fraction stops manufactured virality from driving real decisions","a trend with 60% likely-inauthentic amplification is downweighted before ranking")],
  [con("A wrong discount distorts genuinely organic trends as much as it corrects manufactured ones","over-discounting buries a real grassroots surge")],
  ["discount applied as a hard cut loses the uncertainty band","downstream consumers ignore the discount and use raw reach"],
  ["downstream hype metrics carry the authenticity discount and its uncertainty, not a silently corrected number"],
  ["a downstream consumer changes ingestion of the discount","over-discounting suppresses a verified organic trend"],
  specialists=["data_analyst","disinformation_analyst"]))

N.append(node("EXPLANATION_PROVENANCE","evidence_trail_for_each_estimate",
  "Attach to every probability a human-readable provenance object: which signals fired, their weights, the matching examples (template cluster, burst window, ring members), the calibration version, and the validation FPR/TPR, so the estimate is auditable and appealable rather than an opaque number.",
  "decision", ["FEATURE_FUSION","CALIBRATION"], ["CQ_03","CQ_15"],
  b(0.86,0.8,0.82,0.62,0.84,0.82,0.66,0.66,0.5, 0.78,0.8, 0.86,0.66,0.18,0.52,[0.22,0.56],"the provenance schema or audit/appeal requirements change"),
  ["per-estimate evidence trail","contributing-signal list with weights and examples","calibration and validation version stamps"],
  ["the scoring math itself","reviewer decision recording"],
  [pro("Provenance turns a contestable number into an auditable claim, mirroring how Graphika reports show their evidence","the report cites the 90-second burst window and the 12-word template as the evidence")],
  [con("Rich provenance can expose detection heuristics to adversaries who then engineer around them","publishing exact thresholds teaches operators how to evade")],
  ["provenance leaks heuristics that adversaries exploit","missing provenance makes an estimate impossible to appeal or audit"],
  ["every probability carries a provenance object listing contributing signals, weights, examples, and calibration version"],
  ["audit or appeal requirements change","a provenance leak is found to aid evasion"],
  specialists=["disinformation_analyst","trust_and_safety_lead"]))

N.append(node("OUTPUT_CONTRACT","probability_plus_provenance_schema",
  "Define and enforce the final output contract: a calibrated probability in [0,1], its uncertainty interval, the provenance object, the calibration/validation version, and an explicit 'no boolean verdict' invariant, so no consumer can extract a clean yes/no the data cannot support.",
  "decision", ["UNCERTAINTY_QUANT","EXPLANATION_PROVENANCE","CONFIDENCE_PROPAGATION"], ["CQ_15","CQ_01"],
  b(0.92,0.84,0.84,0.6,0.92,0.85,0.78,0.64,0.55, 0.78,0.82, 0.92,0.64,0.18,0.66,[0.22,0.55],"a consumer requires a schema change, or governance mandates a different output form"),
  ["probability + interval + provenance schema","explicit no-boolean-verdict invariant","calibration/validation version stamps in output"],
  ["per-account identity attribution","platform enforcement actions"],
  [pro("A hard schema invariant is the only durable defense against the boolean collapse that public-only data cannot justify","the schema has no 'is_bot' field, only a calibrated probability with provenance")],
  [con("A strict no-boolean contract frustrates simple integrations that genuinely need a gated action","an automated mute rule still needs a threshold somewhere downstream")],
  ["a consumer post-processes the probability into an unprovenanced boolean","schema drift reintroduces a verdict field"],
  ["the output schema validates: probability + interval + provenance present, and no boolean verdict field exists"],
  ["a consumer requests a boolean field","governance mandates a different output form"],
  specialists=["trust_and_safety_lead","schema_engineer"],
  contradictors=["hard_label_product_manager"]))

CQ = [
 ("CQ_01","Why is the output a calibrated probability with provenance rather than a boolean bot/human verdict?",["nodes"],"PROBLEM_FRAMING and OUTPUT_CONTRACT establish probability+provenance and the no-boolean invariant grounded in the absence of public ground truth",["PROBLEM_FRAMING","NO_GROUND_TRUTH","OUTPUT_CONTRACT"]),
 ("CQ_02","Which confirming signals are private/unavailable from public data, and how is that gap recorded?",["nodes"],"NO_GROUND_TRUTH enumerates the private confirming signals; SIGNAL_INVENTORY logs what is proxied",["NO_GROUND_TRUTH","SIGNAL_INVENTORY"]),
 ("CQ_03","What public signals are collected, with what provenance, and how is each estimate traced back to them?",["nodes"],"SIGNAL_INVENTORY catalogs signals with provenance; EXPLANATION_PROVENANCE traces estimates back",["SIGNAL_INVENTORY","ACCOUNT_FEATURES","EXPLANATION_PROVENANCE"]),
 ("CQ_04","How are inauthentic account-level and textual features (age, avatar, copypasta) extracted?",["nodes"],"ACCOUNT_FEATURES and TEMPLATED_TEXT extract account-level and near-duplicate text signals",["ACCOUNT_FEATURES","TEMPLATED_TEXT"]),
 ("CQ_05","How is temporal coordination (synchronized bursts, lockstep timing) detected?",["nodes"],"COORDINATION_SIGNALS detects synchronized timing and co-action over defined windows",["COORDINATION_SIGNALS","TEMPLATED_TEXT"]),
 ("CQ_06","How is inauthentic network structure (sybil rings, dense reciprocal clusters) detected?",["nodes","edges"],"NETWORK_STRUCTURE mines reciprocal clusters and sybil rings from the interaction graph",["NETWORK_STRUCTURE","COORDINATION_SIGNALS"]),
 ("CQ_07","How is engagement disproportion (views vs comments vs shares) used as a signal?",["nodes"],"ENGAGEMENT_MISMATCH scores cross-channel ratio deviation against matched baselines",["ENGAGEMENT_MISMATCH","NETWORK_STRUCTURE"]),
 ("CQ_08","How are the signal families fused into one inauthenticity score while staying decomposable?",["nodes"],"FEATURE_FUSION weights families into a decomposable score traceable to logged signals",["FEATURE_FUSION","ENGAGEMENT_MISMATCH"]),
 ("CQ_09","How is the fused score turned into a calibrated probability and propagated to downstream hype metrics?",["nodes","workflow"],"CALIBRATION maps the score to a calibrated probability; CONFIDENCE_PROPAGATION discounts downstream hype",["CALIBRATION","FEATURE_FUSION","CONFIDENCE_PROPAGATION"]),
 ("CQ_10","How is the probability calibrated (Platt/isotonic) and how is uncertainty quantified?",["nodes"],"CALIBRATION fits Platt/isotonic and reports FPR/TPR; UNCERTAINTY_QUANT attaches an interval",["CALIBRATION","UNCERTAINTY_QUANT"]),
 ("CQ_11","What labeled benchmark measures FPR/TPR and how is its staleness handled?",["nodes"],"VALIDATION_SET curates versioned labeled CIB benchmarks with documented provenance",["VALIDATION_SET","UNCERTAINTY_QUANT"]),
 ("CQ_12","How is detector decay under adversarial adaptation tracked and corrected?",["nodes","iteration_protocol"],"ADVERSARIAL_ADAPTATION tracks per-signal decay and schedules re-fitting against the benchmark",["ADVERSARIAL_ADAPTATION","VALIDATION_SET"]),
 ("CQ_13","How is the asymmetric cost of false positives encoded in the decision policy?",["nodes","conflict_axes"],"FALSE_POSITIVE_COST encodes explicit asymmetric FP/FN costs feeding an FP-averse policy",["FALSE_POSITIVE_COST","ADVERSARIAL_ADAPTATION"]),
 ("CQ_14","What provenance, human-review routing, and output contract make each estimate auditable, accountable, and free of a boolean verdict?",["nodes","workflow"],"HUMAN_REVIEW_THRESHOLD routes by probability/uncertainty; EXPLANATION_PROVENANCE attaches an evidence trail; OUTPUT_CONTRACT enforces probability+provenance with no boolean field",["HUMAN_REVIEW_THRESHOLD","EXPLANATION_PROVENANCE","OUTPUT_CONTRACT"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

# consolidate node->CQ references onto the 14-CQ set (each node refs 1-2)
CQ_MAP = {
 "PROBLEM_FRAMING":["CQ_01"], "NO_GROUND_TRUTH":["CQ_01","CQ_02"], "SIGNAL_INVENTORY":["CQ_02","CQ_03"],
 "ACCOUNT_FEATURES":["CQ_04"], "TEMPLATED_TEXT":["CQ_04","CQ_05"], "COORDINATION_SIGNALS":["CQ_05","CQ_06"],
 "NETWORK_STRUCTURE":["CQ_06"], "ENGAGEMENT_MISMATCH":["CQ_07"], "FEATURE_FUSION":["CQ_08"],
 "BASELINE_REFERENCE":["CQ_07","CQ_08"], "CROSS_PLATFORM_CORROBORATION":["CQ_06"],
 "CALIBRATION":["CQ_09","CQ_10"], "UNCERTAINTY_QUANT":["CQ_10","CQ_11"], "VALIDATION_SET":["CQ_11"],
 "ADVERSARIAL_ADAPTATION":["CQ_12"], "FALSE_POSITIVE_COST":["CQ_13"], "HUMAN_REVIEW_THRESHOLD":["CQ_14"],
 "CONFIDENCE_PROPAGATION":["CQ_09","CQ_14"], "EXPLANATION_PROVENANCE":["CQ_03","CQ_14"], "OUTPUT_CONTRACT":["CQ_14","CQ_01"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

GL = [
 ("coordinated_inauthentic_behavior","organized use of multiple accounts to deceive about who is behind a reaction, the unit of analysis in Meta/Graphika takedowns",["CIB"],["organic_coordination"],["COORDINATION_SIGNALS","NETWORK_STRUCTURE"]),
 ("social_bot","an automated or semi-automated account that mimics human activity, per Ferrara 2016",["bot","automated_account"],["human_operated_account"],["ACCOUNT_FEATURES","ADVERSARIAL_ADAPTATION"]),
 ("astroturf","fabricated grassroots appearance manufactured by a hidden coordinated source, per Ratkiewicz 2011 Truthy",["fake_grassroots"],["genuine_grassroots"],["TEMPLATED_TEXT","COORDINATION_SIGNALS"]),
 ("sybil_ring","a cluster of accounts controlled by one operator that mutually reinforce to fake reach",["bot_ring","amplification_ring"],["organic_community"],["NETWORK_STRUCTURE"]),
 ("copypasta","identical or near-identical text reposted across many accounts as a coordination tell",["templated_text","slot_filled_template"],["viral_catchphrase"],["TEMPLATED_TEXT"]),
 ("calibration","mapping a model score to a probability whose value matches empirical frequency, via Platt 1999 or isotonic regression",["platt_scaling","isotonic_calibration"],["raw_score"],["CALIBRATION","UNCERTAINTY_QUANT"]),
 ("reliability_diagram","a plot of predicted probability vs observed frequency used to assess calibration",["calibration_curve"],["roc_curve"],["CALIBRATION","VALIDATION_SET"]),
 ("false_positive","a real, authentic user or community wrongly estimated as inauthentic",["type_i_error","wrongful_flag"],["false_negative"],["FALSE_POSITIVE_COST","HUMAN_REVIEW_THRESHOLD"]),
 ("provenance","the logged trail of which signals, weights, examples and model versions produced an estimate",["evidence_trail","audit_trail"],["opaque_score"],["EXPLANATION_PROVENANCE","OUTPUT_CONTRACT"]),
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
              "risk_of_conflict":"unmanaged tension degrades estimate quality","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated behavior",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies)
dep("PROBLEM_FRAMING","NO_GROUND_TRUTH",0.9)
dep("NO_GROUND_TRUTH","SIGNAL_INVENTORY",0.84)
dep("SIGNAL_INVENTORY","ACCOUNT_FEATURES",0.82)
dep("SIGNAL_INVENTORY","TEMPLATED_TEXT",0.82)
dep("SIGNAL_INVENTORY","COORDINATION_SIGNALS",0.84)
dep("SIGNAL_INVENTORY","NETWORK_STRUCTURE",0.78)
dep("COORDINATION_SIGNALS","NETWORK_STRUCTURE",0.8)
dep("SIGNAL_INVENTORY","ENGAGEMENT_MISMATCH",0.78)
dep("COORDINATION_SIGNALS","BASELINE_REFERENCE",0.78)
dep("NETWORK_STRUCTURE","CROSS_PLATFORM_CORROBORATION",0.8)
dep("ACCOUNT_FEATURES","FEATURE_FUSION",0.82)
dep("TEMPLATED_TEXT","FEATURE_FUSION",0.82)
dep("COORDINATION_SIGNALS","FEATURE_FUSION",0.84)
dep("NETWORK_STRUCTURE","FEATURE_FUSION",0.82)
dep("ENGAGEMENT_MISMATCH","FEATURE_FUSION",0.8)
dep("SIGNAL_INVENTORY","VALIDATION_SET",0.78)
dep("FEATURE_FUSION","CALIBRATION",0.86)
dep("VALIDATION_SET","CALIBRATION",0.84)
dep("CALIBRATION","UNCERTAINTY_QUANT",0.82)
dep("VALIDATION_SET","ADVERSARIAL_ADAPTATION",0.8)
dep("UNCERTAINTY_QUANT","FALSE_POSITIVE_COST",0.8)
dep("FALSE_POSITIVE_COST","HUMAN_REVIEW_THRESHOLD",0.84)
dep("CALIBRATION","CONFIDENCE_PROPAGATION",0.8)
dep("HUMAN_REVIEW_THRESHOLD","CONFIDENCE_PROPAGATION",0.74)
dep("FEATURE_FUSION","EXPLANATION_PROVENANCE",0.78)
dep("UNCERTAINTY_QUANT","OUTPUT_CONTRACT",0.82)
dep("EXPLANATION_PROVENANCE","OUTPUT_CONTRACT",0.82)
# cross-cutting non-dependency edges
rel("ADVERSARIAL_ADAPTATION","FEATURE_FUSION","feedback",0.78,why="observed signal decay feeds back to re-weight or retire fusion features")
rel("ADVERSARIAL_ADAPTATION","TEMPLATED_TEXT","feedback",0.74,why="paraphrase/LLM evasion feeds back to update near-duplicate detection")
rel("VALIDATION_SET","CALIBRATION","causal",0.82,why="benchmark labels are the data the calibration map is fit against")
rel("NO_GROUND_TRUTH","OUTPUT_CONTRACT","constraint",0.82,why="the absence of public ground truth constrains the output to a probability, not a verdict")
rel("FALSE_POSITIVE_COST","CONFIDENCE_PROPAGATION","constraint",0.72,why="FP-aversion constrains how aggressively authenticity discounts downstream hype")
rel("EXPLANATION_PROVENANCE","HUMAN_REVIEW_THRESHOLD","similarity",0.74,why="the provenance trail is the evidence a human reviewer adjudicates on")
rel("BASELINE_REFERENCE","FEATURE_FUSION","constraint",0.76,why="organic baselines define the deviation magnitudes the fusion model consumes as features")
rel("CROSS_PLATFORM_CORROBORATION","FEATURE_FUSION","causal",0.74,why="cross-platform linkage raises the corroborated inauthenticity contribution in fusion")
rel("BASELINE_REFERENCE","CONFIDENCE_PROPAGATION","constraint",0.7,why="the organic baseline grounds the inauthentic-fraction estimate used to discount hype")

# conflict edges (negative signed_tension + resolution_rule)
conf("COORDINATION_SIGNALS","FALSE_POSITIVE_COST",0.72,-0.6,
  "weight coordination evidence by FP cost: synchronized bursts alone never cross the action threshold for an authentic-looking organized community without corroborating network/text signals and human review",
  "aggressive coordination detection raises false positives against legitimate organized communities (fandoms, activism)",cp=0.55,erc=0.55)
conf("ADVERSARIAL_ADAPTATION","CALIBRATION",0.7,-0.55,
  "re-fit calibration on the freshest labeled slice whenever adversarial decay is detected; treat a calibration as valid only for the adversary generation it was fit on",
  "adversarial evolution invalidates a previously well-calibrated probability, so static calibration drifts out of date",cp=0.5,erc=0.6)
conf("ENGAGEMENT_MISMATCH","CONFIDENCE_PROPAGATION",0.66,-0.45,
  "discount hype by the calibrated inauthentic fraction with its interval, never by a raw ratio anomaly; a topic-baseline mismatch must not hard-cut an organic trend",
  "raw engagement disproportion can over-discount genuinely organic viral content with naturally skewed ratios",cp=0.45,erc=0.45)
conf("HUMAN_REVIEW_THRESHOLD","FEATURE_FUSION",0.66,-0.5,
  "the model proposes; the human disposes on high-stakes cases: a high fused score never auto-actions when uncertainty is wide or FP cost is high, regardless of model confidence",
  "an automated high-confidence fused score conflicts with the requirement for human accountability on high-stakes public-only calls",cp=0.5,erc=0.5)

CA=[
 {"name":"probability_vs_boolean_verdict","description":"Public-only signals support a calibrated probability but not a clean yes/no; consumers pressure for a boolean the data cannot justify.","poles":["calibrated_probability","hard_boolean_verdict"],"resolution_hint":"enforce a no-boolean output contract; push any thresholding downstream of the probability+provenance","tension_score":0.78,"affected_nodes":["PROBLEM_FRAMING","OUTPUT_CONTRACT","HUMAN_REVIEW_THRESHOLD"]},
 {"name":"sensitivity_vs_false_positive_cost","description":"More aggressive detection catches more bots but mislabels more real users; FP harm is asymmetric.","poles":["high_sensitivity","fp_aversion"],"resolution_hint":"weight evidence by explicit asymmetric FP/FN cost and route ambiguous cases to humans","tension_score":0.75,"affected_nodes":["COORDINATION_SIGNALS","FALSE_POSITIVE_COST","HUMAN_REVIEW_THRESHOLD"]},
 {"name":"authentic_coordination_vs_inauthentic_coordination","description":"Fandoms, activism, and breaking-news crowds coordinate organically and resemble CIB's synchronized timing.","poles":["organic_coordination","inauthentic_coordination"],"resolution_hint":"require corroborating network/account signals before treating synchronized timing as inauthentic","tension_score":0.72,"affected_nodes":["COORDINATION_SIGNALS","NETWORK_STRUCTURE","FALSE_POSITIVE_COST"]},
 {"name":"detector_stability_vs_adversarial_chase","description":"A fixed detector decays as adversaries adapt, but chasing every new evasion overfits to the last attack.","poles":["stable_detector","aggressive_re_fitting"],"resolution_hint":"track per-signal decay and re-fit only on measured TPR drops, not on every reported trick","tension_score":0.7,"affected_nodes":["ADVERSARIAL_ADAPTATION","FEATURE_FUSION","VALIDATION_SET"]},
 {"name":"benchmark_fidelity_vs_label_staleness","description":"Labeled CIB benchmarks enable honest FPR/TPR but are stale and platform-specific, overstating live accuracy.","poles":["benchmark_measurable","live_representativeness"],"resolution_hint":"version benchmarks with label dates; discount reported accuracy by staleness","tension_score":0.68,"affected_nodes":["VALIDATION_SET","CALIBRATION","ADVERSARIAL_ADAPTATION"]},
 {"name":"calibration_fit_vs_base_rate_shift","description":"Calibration is only valid for populations matching the validation set; deployment base-rate shift miscalibrates silently.","poles":["validation_fit","deployment_base_rate"],"resolution_hint":"re-estimate base rate at deployment and recalibrate or widen the interval","tension_score":0.66,"affected_nodes":["CALIBRATION","UNCERTAINTY_QUANT","VALIDATION_SET"]},
 {"name":"signal_fusion_power_vs_correlated_double_counting","description":"Fusing signal families is robust but can double-count correlated signals derived from one underlying event.","poles":["multi_signal_robustness","independence_assumption"],"resolution_hint":"decorrelate families and decompose the score so one event cannot be counted five times","tension_score":0.62,"affected_nodes":["FEATURE_FUSION","COORDINATION_SIGNALS","NETWORK_STRUCTURE"]},
 {"name":"provenance_transparency_vs_evasion_exposure","description":"Rich provenance enables audit and appeal but can teach adversaries exactly how to evade.","poles":["full_transparency","heuristic_protection"],"resolution_hint":"expose evidence categories and examples to reviewers; withhold exact thresholds from public output","tension_score":0.6,"affected_nodes":["EXPLANATION_PROVENANCE","OUTPUT_CONTRACT","ADVERSARIAL_ADAPTATION"]},
 {"name":"point_estimate_actionability_vs_uncertainty_honesty","description":"A point probability is easy to act on but hides the wide uncertainty intrinsic to sparse public signals.","poles":["actionable_point","honest_interval"],"resolution_hint":"always ship the interval with the point and gate actions on interval width","tension_score":0.58,"affected_nodes":["UNCERTAINTY_QUANT","HUMAN_REVIEW_THRESHOLD","CONFIDENCE_PROPAGATION"]},
]

EC=[
 {"description":"A real, organically coordinated community (a fandom mass-reply) is flagged as inauthentic coordination.","trigger":"synchronized-timing detection fires on authentic organized activity with no corroboration","affected_nodes":["COORDINATION_SIGNALS","FALSE_POSITIVE_COST","HUMAN_REVIEW_THRESHOLD"],"mitigation":"require corroborating network/account signals and route to human review before action","severity":"high"},
 {"description":"A campaign uses LLM-paraphrased text so every post is unique, defeating near-duplicate detection.","trigger":"template-spinning / model rewriting raises textual diversity above the similarity threshold","affected_nodes":["TEMPLATED_TEXT","ADVERSARIAL_ADAPTATION","FEATURE_FUSION"],"mitigation":"track text-signal decay; lean on timing/network signals; re-fit detection","severity":"high"},
 {"description":"A public-only probability is collapsed into a boolean 'is_bot' flag by a downstream consumer.","trigger":"a consumer thresholds the probability and drops the provenance and interval","affected_nodes":["OUTPUT_CONTRACT","PROBLEM_FRAMING","CONFIDENCE_PROPAGATION"],"mitigation":"enforce the no-boolean output contract; require provenance to travel with any derived flag","severity":"critical"},
 {"description":"Calibration fit on bot-heavy validation data is overconfident on a clean live feed (base-rate shift).","trigger":"deployment base rate of inauthenticity diverges sharply from the validation set","affected_nodes":["CALIBRATION","VALIDATION_SET","UNCERTAINTY_QUANT"],"mitigation":"re-estimate deployment base rate and recalibrate or widen the uncertainty interval","severity":"high"},
 {"description":"Five fused signals all derive from one synchronized burst, so the score double-counts a single event.","trigger":"correlated signal families are summed as if independent","affected_nodes":["FEATURE_FUSION","COORDINATION_SIGNALS","NETWORK_STRUCTURE"],"mitigation":"decorrelate families and cap the contribution of any single underlying event","severity":"medium"},
 {"description":"Partial/sampled follower graph fabricates or hides a sybil ring.","trigger":"API rate limits return only a fraction of the real edges","affected_nodes":["NETWORK_STRUCTURE","SIGNAL_INVENTORY"],"mitigation":"record graph sampling coverage; downweight network signal when coverage is low","severity":"high"},
 {"description":"A new real user with a default avatar and empty bio is scored as a likely bot.","trigger":"account-feature heuristics over-penalize legitimate new users","affected_nodes":["ACCOUNT_FEATURES","FALSE_POSITIVE_COST"],"mitigation":"use soft anomaly thresholds, never hard cutoffs; weight by FP cost","severity":"medium"},
 {"description":"An organic meme with naturally skewed likes-to-comments ratio is flagged as purchased engagement.","trigger":"a fixed engagement baseline ignores topic/format variation","affected_nodes":["ENGAGEMENT_MISMATCH","CONFIDENCE_PROPAGATION"],"mitigation":"compare against topic/format-matched baselines; never hard-cut on ratio alone","severity":"medium"},
 {"description":"Published provenance thresholds teach operators exactly how to stay under detection.","trigger":"exact heuristic thresholds are exposed in public output","affected_nodes":["EXPLANATION_PROVENANCE","ADVERSARIAL_ADAPTATION"],"mitigation":"expose evidence categories to reviewers; withhold exact thresholds publicly","severity":"medium"},
 {"description":"A detector silently decays as the adversary evolves and confidence stays high while TPR collapses.","trigger":"no per-signal decay tracking is in place","affected_nodes":["ADVERSARIAL_ADAPTATION","VALIDATION_SET","FEATURE_FUSION"],"mitigation":"track per-signal TPR over time; trigger re-fit on measured decay","severity":"high"},
 {"description":"A high fused score auto-actions on a high-stakes case despite a wide uncertainty interval.","trigger":"the human-review gate is bypassed under queue pressure","affected_nodes":["HUMAN_REVIEW_THRESHOLD","UNCERTAINTY_QUANT","FALSE_POSITIVE_COST"],"mitigation":"hard-gate high-stakes actions on interval width and FP cost; never auto-action wide-interval cases","severity":"high"},
 {"description":"Acknowledged public-data uncertainty is weaponized to dismiss a well-evidenced coordinated campaign.","trigger":"adversary or stakeholder argues 'you can't prove it' against corroborated CIB evidence","affected_nodes":["NO_GROUND_TRUTH","EXPLANATION_PROVENANCE","OUTPUT_CONTRACT"],"mitigation":"present the calibrated probability with full provenance and FPR/TPR, not a deniable single number","severity":"medium"},
]

WF=[
 {"action":"frame_as_probability","node_ref":"PROBLEM_FRAMING","description":"Fix the task as estimating P(inauthentic | public signals) for a visible reaction, with provenance and no boolean verdict.","artifact":"problem_frame_record","gate":"output target is a probability+provenance, not a boolean"},
 {"action":"record_ground_truth_gap","node_ref":"NO_GROUND_TRUTH","description":"Enumerate the private confirming signals that are unavailable and mark every estimate as proxied, not measured.","artifact":"ground_truth_gap_log","gate":"unavailable confirming signals are documented"},
 {"action":"catalog_public_signals","node_ref":"SIGNAL_INVENTORY","description":"Collect public signals with provenance, freshness, and reliability per signal.","artifact":"signal_catalog","gate":"every signal has a logged collection method and timestamp"},
 {"action":"extract_account_and_text_signals","node_ref":"ACCOUNT_FEATURES","description":"Compute account-anomaly features and near-duplicate text clusters.","artifact":"per_account_and_text_features","gate":"features computed with soft thresholds, not hard cutoffs"},
 {"action":"detect_coordination","node_ref":"COORDINATION_SIGNALS","description":"Detect synchronized bursts, lockstep timing, and repeated co-action over logged windows.","artifact":"coordination_scores","gate":"window parameters logged with each coordination score"},
 {"action":"analyze_network_and_engagement","node_ref":"NETWORK_STRUCTURE","description":"Mine reciprocal clusters/sybil rings and score engagement-ratio disproportion.","artifact":"network_and_engagement_metrics","gate":"graph sampling coverage recorded"},
 {"action":"fuse_signals","node_ref":"FEATURE_FUSION","description":"Fuse signal families into a decomposable inauthenticity score traceable to logged signals.","artifact":"fused_score_with_decomposition","gate":"score decomposes into per-family contributions"},
 {"action":"calibrate_probability","node_ref":"CALIBRATION","description":"Map the fused score to a calibrated probability via Platt/isotonic on the labeled benchmark; report FPR/TPR.","artifact":"calibrated_probability_with_reliability_diagram","gate":"reliability diagram and FPR/TPR produced"},
 {"action":"quantify_uncertainty","node_ref":"UNCERTAINTY_QUANT","description":"Attach an uncertainty interval that widens with signal sparsity.","artifact":"probability_with_interval","gate":"interval present and sparsity-sensitive"},
 {"action":"apply_cost_and_route","node_ref":"HUMAN_REVIEW_THRESHOLD","description":"Apply asymmetric FP/FN costs and route by probability/uncertainty band; gate high-stakes actions on human review.","artifact":"routing_decision","gate":"high-stakes actions require human review"},
 {"action":"propagate_and_explain","node_ref":"CONFIDENCE_PROPAGATION","description":"Discount downstream hype by the inauthentic fraction and attach the provenance trail.","artifact":"discounted_metrics_plus_provenance","gate":"hype metrics carry the discount and its interval"},
 {"action":"emit_contracted_output","node_ref":"OUTPUT_CONTRACT","description":"Emit the probability + interval + provenance + version stamps; validate the no-boolean invariant.","artifact":"final_inauthenticity_estimate","gate":"schema validates and contains no boolean verdict field"},
]

DR=[
 {"rule":"PROBLEM_FRAMING and the no-boolean OUTPUT_CONTRACT must be fixed before any signal is scored for action","rationale":"a downstream boolean cannot be justified by public-only data; framing must precede scoring","trigger":"a scoring pipeline starts without the probability+provenance contract","action":"block until the output contract forbids a boolean verdict"},
 {"rule":"NO_GROUND_TRUTH gap must be documented before any signal is treated as confirming inauthenticity","rationale":"public proxies are not internal labels; over-claiming creates legal and reputational risk","trigger":"a public proxy is asserted as confirmed ground truth","action":"require the unavailable confirming signals to be logged as proxied"},
 {"rule":"CALIBRATION must be fit on a versioned VALIDATION_SET before any probability is reported","rationale":"an uncalibrated score is not a probability and FPR/TPR cannot be quoted honestly","trigger":"a raw score is shipped as if it were a probability","action":"block until a reliability diagram and FPR/TPR exist"},
 {"rule":"FALSE_POSITIVE_COST weights must be set before HUMAN_REVIEW_THRESHOLD bands are drawn","rationale":"routing bands are derived from the asymmetric cost of false positives","trigger":"review bands set without explicit FP/FN costs","action":"require cost weights before band definition"},
 {"rule":"High-stakes actions must pass HUMAN_REVIEW_THRESHOLD when uncertainty is wide or FP cost is high","rationale":"public-only probabilistic evidence requires human accountability on consequential calls","trigger":"an auto-action on a wide-interval high-stakes case","action":"hard-gate the action on human review"},
 {"rule":"FEATURE_FUSION must decorrelate signal families before fusing","rationale":"correlated families derived from one event double-count and falsely inflate confidence","trigger":"correlated signals summed as independent","action":"decorrelate and cap any single event's contribution"},
 {"rule":"ADVERSARIAL_ADAPTATION decay tracking must run before trusting a fixed detector over time","rationale":"detectors decay silently as adversaries evolve, per Cresci 2017","trigger":"a detector is trusted without per-signal TPR monitoring","action":"require per-signal decay tracking and re-fit triggers"},
 {"rule":"EXPLANATION_PROVENANCE must accompany every reported probability","rationale":"an opaque number is unappealable and indistinguishable from a verdict","trigger":"a probability is emitted without an evidence trail","action":"block output until provenance is attached"},
 {"rule":"CONFIDENCE_PROPAGATION discounts must carry the uncertainty interval, never a hard cut","rationale":"a hard discount distorts organic trends as much as it corrects manufactured ones","trigger":"a downstream metric is hard-cut by a point estimate","action":"require the interval to travel with the discount"},
]

ARR=[
 {"rule":"Do not score signals for action before fixing the probability+provenance output contract; retrofitting it after a boolean shipped is costly and harmful","prevents":"a defamatory hard 'bot' label that must later be retracted and re-engineered into a probability"},
 {"rule":"Do not treat a public proxy as confirmed ground truth; walking back an over-claim after publication is far costlier than hedging up front","prevents":"retraction of an over-claimed attribution and the trust damage it causes"},
 {"rule":"Do not report a raw fused score as a probability; calibrating after the fact invalidates every prior reported number","prevents":"re-issuing all previously reported probabilities once calibration reveals miscalibration"},
 {"rule":"Do not draw human-review bands before setting FP/FN costs; re-routing the entire case queue after a cost change is expensive","prevents":"re-adjudication of a backlog routed under the wrong cost assumptions"},
 {"rule":"Do not fuse correlated signal families as independent; unwinding double-counted confidence after action is costly","prevents":"reversing actions taken on a falsely confident composite score"},
 {"rule":"Do not deploy a detector without decay tracking; discovering silent decay after a missed campaign forces a full re-fit and post-mortem","prevents":"emergency re-fitting and incident review after an undetected evolved-bot campaign"},
 {"rule":"Do not emit a probability without provenance; reconstructing the evidence trail during an appeal requires re-running the whole pipeline","prevents":"unappealable estimates that collapse under audit and force full re-computation"},
 {"rule":"Do not hard-cut downstream hype by a point estimate; restoring a wrongly suppressed organic trend after the fact is costly","prevents":"re-instating a buried grassroots trend and recomputing affected rankings"},
]

IP=[
 {"trigger":"a downstream consumer collapses the probability into a boolean verdict","action":"re-assert the no-boolean OUTPUT_CONTRACT and require provenance to travel with any derived flag in PROBLEM_FRAMING","nodes":["OUTPUT_CONTRACT","PROBLEM_FRAMING","CONFIDENCE_PROPAGATION"],"priority":"critical"},
 {"trigger":"per-signal TPR decays or a novel evasion technique is observed","action":"update the evasion catalog in ADVERSARIAL_ADAPTATION and re-fit FEATURE_FUSION and CALIBRATION on the freshest labeled slice","nodes":["ADVERSARIAL_ADAPTATION","FEATURE_FUSION","CALIBRATION"],"priority":"high"},
 {"trigger":"a false positive harms a real user or community","action":"raise FP cost weights in FALSE_POSITIVE_COST and tighten the human-review band in HUMAN_REVIEW_THRESHOLD","nodes":["FALSE_POSITIVE_COST","HUMAN_REVIEW_THRESHOLD"],"priority":"high"},
 {"trigger":"deployment base rate diverges from the validation set","action":"re-estimate the base rate and recalibrate or widen the interval in CALIBRATION and UNCERTAINTY_QUANT","nodes":["CALIBRATION","UNCERTAINTY_QUANT","VALIDATION_SET"],"priority":"high"},
 {"trigger":"graph sampling coverage drops below a usable threshold","action":"downweight the network signal in FEATURE_FUSION and record coverage in SIGNAL_INVENTORY","nodes":["NETWORK_STRUCTURE","SIGNAL_INVENTORY","FEATURE_FUSION"],"priority":"medium"},
 {"trigger":"an authentic organized community is flagged as coordinated inauthentic behavior","action":"require corroborating signals in COORDINATION_SIGNALS and route to human review in HUMAN_REVIEW_THRESHOLD","nodes":["COORDINATION_SIGNALS","NETWORK_STRUCTURE","HUMAN_REVIEW_THRESHOLD"],"priority":"medium"},
 {"trigger":"a new labeled CIB dataset is released","action":"version it into VALIDATION_SET and re-measure FPR/TPR and recalibrate","nodes":["VALIDATION_SET","CALIBRATION","ADVERSARIAL_ADAPTATION"],"priority":"medium"},
]

spec = {
 "domain":"auth__inauthenticity_detection",
 "domain_label":"Bot / Astroturf / Coordinated-Inauthentic-Behavior Detection",
 "purpose":"estimate_a_calibrated_probability_that_visible_reaction_is_inauthentic_from_public_signals_alone_never_a_boolean_verdict_by_fusing_account_text_timing_network_and_engagement_signals_calibrating_with_platt_or_isotonic_against_labeled_cib_benchmarks_quantifying_uncertainty_modeling_adversarial_decay_pricing_false_positive_harm_routing_to_human_review_and_emitting_a_probability_plus_provenance_output_contract",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "only public signals are available; platform-internal labels (IP, device, login graph) are not accessible",
   "the unit of analysis is a visible reaction (a wave of engagement), not the legal identity of any single named account",
   "a labeled CIB benchmark can be assembled from published corpora for calibration and FPR/TPR measurement",
 ],
 "exclusions":[
   "per-account legal attribution to a named operator (delegated to legal/forensic process)",
   "platform-side enforcement, suspension, or takedown mechanics",
   "access to platform-internal telemetry or confirmed ground-truth labels",
   "content veracity / fact-checking of the messages themselves (a separate task from authenticity of the reaction)",
 ],
 "source_description":"heuristic prior estimates for inauthentic-reaction detection work units, informed by the social-bot and CIB literature; no supplied dataset",
 "source_citation":"Ferrara, Varol, Davis, Menczer & Flammini 2016 'The Rise of Social Bots' (Comm. ACM 59:7); Davis, Varol, Ferrara, Flammini & Menczer 2016 'BotOrNot/Botometer' (WWW); Ratkiewicz et al. 2011 'Detecting and Tracking Political Abuse in Social Media' / Truthy (ICWSM); Cresci, Di Pietro, Petrocchi, Spognardi & Tesconi 2017 'The Paradigm-Shift of Social Spambots' (WWW); Meta Adversarial Threat / Coordinated Inauthentic Behavior reports & Graphika network analyses; Platt 1999 'Probabilistic Outputs for Support Vector Machines'; Zadrozny & Elkan 2002 isotonic calibration; Niculescu-Mizil & Caruana 2005 calibration comparison",
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
 "priority_rationale":"PROBLEM_FRAMING/NO_GROUND_TRUTH/SIGNAL_INVENTORY set the epistemic frame; the five signal families feed FEATURE_FUSION; CALIBRATION and UNCERTAINTY_QUANT make the score an honest probability; FALSE_POSITIVE_COST, HUMAN_REVIEW_THRESHOLD, provenance and the OUTPUT_CONTRACT enforce the probability-not-verdict finding last.",
 "eval_objective":"verify_public_only_calibrated_probability_estimation_signal_fusion_adversarial_decay_handling_false_positive_pricing_and_no_boolean_output_contract_of_auth__inauthenticity_detection_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "auth__inauthenticity_detection.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
