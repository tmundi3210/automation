#!/usr/bin/env python3
"""Generate the hype__organic_baseline content spec (B60 content-intelligence KB)
for kb_forge.py. Domain: separate genuine hype from manufactured hype by comparing
observed engagement to organic diffusion baselines and requiring N independent
origins after provenance deduplication. Compact authoring: node() applies sane
defaults so only domain content + base metric magnitudes are specified per node."""
import json, os

SRC = "SRC_HEURISTIC_PRIOR"

def node(id, topic, definition, group, deps, cqs, base,
         scope_in, scope_out, pros, cons, fmodes, accept, revisit,
         inputs=None, outputs=None, specialists=None, contradictors=None,
         subfields=None):
    # base: dict with the 9 metrics + AT/DG + prob(PI,EC,FR,DW) + ui + update_signal
    b = dict(base)
    return {
        "id": id, "topic": topic, "definition": definition, "group": group,
        "node_type": "work_unit",
        "academic_fields": ["computational_social_science", "information_diffusion"],
        "subfields": subfields or ["virality_modeling", "source_independence_analysis"],
        "specialists": specialists or ["content_intelligence_analyst"],
        "contradictors": contradictors or ["raw_engagement_maximalist"],
        "inputs": inputs or ["per-platform engagement time series", "content provenance metadata"],
        "outputs": outputs or ["genuine-vs-manufactured hype verdict"],
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

N.append(node("ORGANIC_BASELINE","expected_organic_engagement_baseline",
  "Estimate the expected engagement (views, reshares, comments) a topic should attract per platform, region, and content category under normal organic diffusion, to serve as the reference against which observed hype is judged.",
  "foundations", [], ["CQ_01"],
  b(0.92,0.85,0.8,0.62,0.85,0.8,0.55,0.66,0.45, 0.78,0.8, 0.9,0.66,0.18,0.55,[0.22,0.55],"platform algorithm change, seasonality shift, or new region/category added to the baseline panel"),
  ["per-platform/region/category expected-engagement estimates","seasonality and audience-size normalization","baseline confidence intervals"],
  ["spike attribution","origin deduplication","bot scoring"],
  [pro("A calibrated organic baseline turns 'this looks big' into a quantitative deviation, the prerequisite for any anomaly judgment","a fashion reel at 40k views means little until the category baseline says median is 8k")],
  [con("Baselines drift silently when the platform changes ranking, making yesterday's 'normal' wrong today","a feed-algorithm update doubles organic reach and inflates every deviation")],
  ["baseline computed on a non-comparable audience (wrong region/category)","stale baseline from before an algorithm change","survivorship bias from only sampling already-viral items"],
  ["baseline reproduces held-out organic items' engagement within its stated confidence interval","baseline panel covers every platform/region/category the verdict will be issued for"],
  ["platform ranking algorithm changes","baseline residuals show systematic drift over weeks"],
  inputs=["historical per-platform engagement","platform/region/category taxonomy"],
  outputs=["expected-engagement baseline with confidence intervals"],
  specialists=["content_intelligence_analyst","quantitative_marketing_scientist"]))

N.append(node("DIFFUSION_CURVE","bass_diffusion_adoption_shape",
  "Fit the Bass diffusion model (innovation coefficient p, imitation coefficient q, market potential m) to a topic's cumulative adoption, characterizing the expected S-curve shape that genuine word-of-mouth spread produces.",
  "foundations", ["ORGANIC_BASELINE"], ["CQ_01","CQ_02"],
  b(0.85,0.78,0.72,0.74,0.82,0.78,0.55,0.6,0.5, 0.72,0.76, 0.84,0.6,0.22,0.55,[0.26,0.62],"diffusion model family changes or p/q priors are recalibrated for a platform"),
  ["Bass p/q/m parameter fitting","expected adoption S-curve","word-of-mouth shape characterization"],
  ["instantaneous-burst attribution","provenance clustering","threshold policy"],
  [pro("The Bass model gives a principled expected adoption shape so deviations from genuine word-of-mouth become measurable, not eyeballed","a genuine trend follows a smooth p+q*(N/m) hazard, not a vertical jump")],
  [con("Bass assumes a single homogeneous market and one initiating event, so multi-seed or networked spread fits poorly","a meme reignited by three unrelated creators violates the single-market assumption")],
  ["fitting Bass to a partial/early curve yields unstable q","forcing an S-curve onto genuinely networked structural-virality spread","ignoring repeat adoption inflates m"],
  ["fitted curve reproduces the held-out tail of a known-organic trend within tolerance","p and q lie in plausible ranges seen for the platform"],
  ["new platform with unknown p/q priors","structural-virality patterns recur that Bass cannot fit"],
  inputs=["cumulative adoption time series","Bass p/q priors per platform"],
  outputs=["fitted diffusion curve and residuals"],
  specialists=["content_intelligence_analyst","diffusion_modeler"]))

N.append(node("SEARCH_INTEREST","search_interest_semi_independent_channel",
  "Track search-query interest (e.g., trends indices) for the topic as a semi-independent demand signal that, unlike platform engagement, reflects pull-driven attention and is less easily inflated by a single seeding campaign.",
  "foundations", ["ORGANIC_BASELINE"], ["CQ_03"],
  b(0.78,0.72,0.7,0.6,0.7,0.74,0.5,0.66,0.45, 0.76,0.78, 0.78,0.66,0.2,0.46,[0.2,0.5],"search-trends provider methodology changes or query-mapping for the topic is revised"),
  ["search-query interest time series","query-to-topic mapping","pull-vs-push signal separation"],
  ["platform reshare counting","origin clustering","bot detection"],
  [pro("Search interest is a pull signal a push campaign cannot easily manufacture, so concordance with engagement is corroborating evidence of genuine demand","a product whose hype rises but whose searches stay flat is likely push-only")],
  [con("Search indices are coarse, normalized, and sampled, and brand-name queries can be gamed by ad spend","a paid search campaign lifts the trends curve without organic demand")],
  ["mapping the wrong queries to the topic","treating a privacy-sampled index as exact counts","ignoring that searches lag virality by hours"],
  ["search interest for a known-organic topic correlates positively with its independent-origin engagement"],
  ["trends provider changes normalization","search-engagement correlation breaks for known cases"],
  inputs=["search-trends index","topic query set"],
  outputs=["semi-independent demand corroboration signal"],
  specialists=["content_intelligence_analyst","search_demand_analyst"]))

N.append(node("ANOMALY_DETECTION","burst_vs_expected_anomaly_detection",
  "Detect statistically significant bursts: flag windows where observed engagement deviates from the organic baseline and Bass-expected trajectory beyond a chosen threshold, distinguishing real surges from normal variation.",
  "detection", ["ORGANIC_BASELINE","DIFFUSION_CURVE"], ["CQ_02","CQ_04"],
  b(0.88,0.8,0.74,0.74,0.85,0.8,0.55,0.6,0.55, 0.72,0.76, 0.86,0.6,0.24,0.55,[0.28,0.64],"anomaly-detection method or significance threshold changes"),
  ["residual computation vs baseline/curve","burst/change-point detection","statistical significance scoring"],
  ["origin deduplication","manufactured-vs-genuine adjudication","threshold governance"],
  [pro("Casting hype as anomaly detection against an expected model is a well-studied, falsifiable framing rather than ad hoc spike-spotting","a 6-sigma residual over the Bass-expected window is an objective burst flag")],
  [con("Anomaly thresholds trade false positives against missed bursts, and bursty platforms have heavy-tailed normals","a viral-but-organic Friday spike trips a Gaussian threshold tuned on weekdays")],
  ["Gaussian thresholds on heavy-tailed engagement over-flag","change-point lag misses fast bursts","baseline error propagates into every anomaly score"],
  ["on labeled organic surges, false-positive rate stays under the configured target","detected bursts align with known seeding events in the validation set"],
  ["engagement distribution shifts to heavier tails","false-positive rate exceeds target on review"],
  inputs=["observed engagement","baseline and fitted-curve residuals"],
  outputs=["scored burst/anomaly flags"],
  specialists=["content_intelligence_analyst","anomaly_detection_engineer"]))

N.append(node("PROVENANCE_CLUSTERING","cross_post_provenance_clustering",
  "Cluster content items that are cross-posts, screenshots, re-uploads, or quote-shares of the same underlying artifact so that one piece of content propagated across surfaces collapses to a single provenance seed.",
  "deduplication", ["ANOMALY_DETECTION"], ["CQ_05"],
  b(0.9,0.82,0.76,0.8,0.88,0.85,0.62,0.58,0.6, 0.7,0.74, 0.88,0.58,0.26,0.6,[0.3,0.68],"matching method (perceptual hash, near-dup text, watermark) changes or a new repost surface appears"),
  ["perceptual/near-duplicate matching","screenshot and re-upload linkage","cluster-to-seed assignment"],
  ["independence accounting","origin counting","bot scoring"],
  [pro("Collapsing reposts to one seed prevents counting a single artifact's spread as many independent signals, the core deduplication step","one screenshot reposted to five accounts is one seed, not five origins")],
  [con("Aggressive clustering can merge genuinely distinct independent creations of similar content into one seed","two creators independently filming the same public event get wrongly merged")],
  ["perceptual hash collisions merge distinct content","missed match leaves duplicates as fake independence","cropping/re-encoding defeats the matcher"],
  ["known repost chains in the validation set collapse to one seed each","independently-created near-duplicates are not merged on the labeled set"],
  ["a new repost/quote-share surface emerges","matcher precision or recall degrades on audit"],
  inputs=["content artifacts and hashes","cross-post and quote-share metadata"],
  outputs=["provenance clusters keyed to seeds"],
  specialists=["content_intelligence_analyst","provenance_forensics_engineer"]))

N.append(node("SOURCE_INDEPENDENCE","independent_origin_deduplication",
  "Reduce a set of platform signals to DISTINCT independent origins: after provenance clustering, determine how many causally and organizationally independent sources actually carry the topic, because multi-platform agreement can collapse to a single origin.",
  "deduplication", ["PROVENANCE_CLUSTERING"], ["CQ_05","CQ_06"],
  b(0.95,0.88,0.82,0.82,0.92,0.9,0.7,0.56,0.65, 0.7,0.74, 0.94,0.56,0.28,0.66,[0.3,0.7],"independence definition or origin-merging rules change"),
  ["causal/organizational independence assessment","collapsing correlated platforms to shared origins","distinct-origin counting"],
  ["raw-count adjustment","threshold decision","search corroboration"],
  [pro("Heuer's intelligence principle that corroboration requires source independence directly applies: five platforms echoing one press release is one source, not five","cross-platform agreement is discounted to its true independent-origin count")],
  [con("Independence is hard to observe; shared funding, coordinated networks, and syndication hide behind apparent diversity","an astroturf network of 'independent' accounts shares one operator")],
  ["treating correlated platforms as independent inflates corroboration","missing a shared operator behind diverse-looking accounts","syndicated wire content counted as many origins"],
  ["five platforms tracing to one press release reduce to one independent origin on the labeled set","known astroturf networks collapse to their true operator count"],
  ["evidence of coordinated/syndicated networks emerges","independence assumptions contradicted by provenance audit"],
  inputs=["provenance clusters","platform/account ownership and coordination signals"],
  outputs=["count of distinct independent origins"],
  specialists=["content_intelligence_analyst","osint_source_analyst"]))

N.append(node("COMMON_METHOD_BIAS","common_method_non_independence",
  "Account for common-method bias: sources sharing the same data-collection method, instrumentation, or platform pipeline are not truly independent even if nominally distinct, and their agreement must be discounted.",
  "deduplication", ["SOURCE_INDEPENDENCE"], ["CQ_06"],
  b(0.82,0.74,0.7,0.74,0.82,0.85,0.58,0.58,0.58, 0.72,0.74, 0.82,0.58,0.24,0.55,[0.28,0.64],"shared-method taxonomy changes or a new common pipeline is identified"),
  ["shared-method identification","method-correlation discounting","instrumentation overlap detection"],
  ["origin counting (delegated)","threshold policy","bot scoring"],
  [pro("Naming common-method variance prevents mistaking shared-pipeline artifacts for independent corroboration, a known confound in survey and measurement science","two 'independent' trackers both buying the same third-party panel are one method")],
  [con("Method correlation is partly latent and easy to under- or over-estimate, risking both inflated and deflated independence","over-discounting punishes sources that merely use common best-practice tooling")],
  ["ignoring shared third-party data panels","conflating method overlap with topical correlation","double-counting after both clustering and method discounting"],
  ["sources sharing a single data panel are flagged as method-correlated on the labeled set","independence count after discounting does not double-penalize already-clustered seeds"],
  ["a new shared data pipeline is discovered","method-correlation estimates contradicted by audit"],
  inputs=["per-source method/instrumentation metadata","shared-vendor and panel registries"],
  outputs=["method-correlation discount on independence"],
  specialists=["content_intelligence_analyst","measurement_methodologist"]))

N.append(node("BOT_RISK_SCORING","bot_and_coordination_risk_scoring",
  "Score the share of engagement attributable to bots, sockpuppets, and coordinated inauthentic behavior on each origin, producing a per-signal bot-risk estimate used to discount raw hype.",
  "adjustment", ["SOURCE_INDEPENDENCE"], ["CQ_07"],
  b(0.84,0.78,0.74,0.76,0.85,0.78,0.55,0.58,0.58, 0.72,0.74, 0.84,0.58,0.26,0.58,[0.3,0.66],"bot-detection model or coordination heuristics change"),
  ["bot/sockpuppet share estimation","coordinated-behavior detection","per-origin bot-risk score"],
  ["raw-count computation","threshold decision","provenance clustering"],
  [pro("A per-origin bot-risk score lets raw engagement be discounted by inauthenticity rather than trusted at face value","an origin with 70% burst accounts created same-day is heavily discounted")],
  [con("Bot detection is adversarial and lags new evasion, so scores are systematically conservative and can be evaded","a sophisticated farm with aged, warmed accounts evades behavioral signals")],
  ["false-positive bot labels suppress genuine grassroots accounts","detector blind to a new automation pattern","coordination scored per-account misses network-level patterns"],
  ["on labeled bot campaigns the bot-risk score exceeds the genuine-baseline score","grassroots known-organic origins are not flagged as high bot-risk"],
  ["a new automation/evasion pattern appears","bot-risk scores diverge from labeled audits"],
  inputs=["account behavioral features","coordination/network signals"],
  outputs=["per-origin bot-risk score"],
  specialists=["content_intelligence_analyst","platform_integrity_analyst"]))

N.append(node("SATURATION_ESTIMATE","audience_saturation_estimate",
  "Estimate how saturated the addressable audience already is for the topic (fraction of market potential m already reached), since late-stage engagement on a saturated topic signals less genuine novel demand than early-stage growth.",
  "adjustment", ["DIFFUSION_CURVE"], ["CQ_07","CQ_08"],
  b(0.78,0.72,0.68,0.7,0.76,0.74,0.52,0.6,0.5, 0.72,0.74, 0.78,0.6,0.22,0.5,[0.26,0.6],"market-potential m estimate or saturation model changes"),
  ["fraction-of-m-reached estimation","early-vs-late-stage classification","saturation discount factor"],
  ["origin independence (delegated)","bot scoring","baseline estimation"],
  [pro("Saturation context discounts plateau-phase engagement that looks large but represents recycled, not novel, attention","100k late reshares on a near-saturated topic add little genuine novelty")],
  [con("Market potential m is itself estimated and unstable, so the saturation fraction inherits that uncertainty","an underestimated m makes a still-growing topic look prematurely saturated")],
  ["using a stale m from before audience growth","confusing saturation with genuine fading","saturation discount applied twice with the curve fit"],
  ["saturation estimate tracks the fitted curve's cumulative fraction on known trends","early-stage genuine growth is not over-discounted as saturated"],
  ["market-potential m is revised","saturation estimates mismatch observed plateaus"],
  inputs=["fitted Bass m and cumulative adoption","addressable-audience size"],
  outputs=["audience-saturation fraction and discount"],
  specialists=["content_intelligence_analyst","market_sizing_analyst"]))

N.append(node("RAW_VS_ADJUSTED_HYPE","raw_to_adjusted_hype_discounting",
  "Compute adjusted hype by discounting raw engagement for bot risk and audience saturation, so the headline number reflects plausibly genuine human, novel attention rather than inflated or recycled volume.",
  "adjustment", ["BOT_RISK_SCORING","SATURATION_ESTIMATE"], ["CQ_07","CQ_08"],
  b(0.88,0.85,0.8,0.7,0.86,0.82,0.6,0.6,0.58, 0.72,0.76, 0.88,0.6,0.24,0.6,[0.28,0.64],"discounting formula or weighting of bot/saturation factors changes"),
  ["raw-engagement aggregation","bot and saturation discount application","adjusted-hype magnitude"],
  ["origin counting","threshold pass/fail","trajectory classification"],
  [pro("Separating raw from adjusted hype makes the inflation explicit and auditable instead of buried in a single number","raw 500k reshares at 60% bot-risk and high saturation yields ~120k adjusted")],
  [con("Multiplicative discounts can compound errors from two noisy estimators into a misleadingly precise adjusted figure","small errors in bot and saturation each compound in the product")],
  ["double-counting a discount already applied upstream","treating adjusted hype as exact rather than an interval","negative or clipped values from over-discounting"],
  ["adjusted hype on labeled manufactured campaigns falls far below raw, while genuine campaigns stay close to raw","adjusted-hype confidence interval is reported, not just a point"],
  ["discount weighting policy changes","adjusted-vs-raw gap behaves unexpectedly on audits"],
  inputs=["raw engagement","bot-risk and saturation discounts"],
  outputs=["adjusted hype magnitude with interval"],
  specialists=["content_intelligence_analyst","quantitative_marketing_scientist"]))

N.append(node("STRUCTURAL_VIRALITY","structural_virality_measurement",
  "Measure structural virality (the Goel et al. average-distance/Wiener-index of the diffusion tree) to distinguish a broadcast (one source to many) from genuine multi-generational person-to-person spread.",
  "detection", ["DIFFUSION_CURVE","PROVENANCE_CLUSTERING"], ["CQ_02","CQ_09"],
  b(0.82,0.76,0.72,0.82,0.82,0.8,0.55,0.56,0.55, 0.7,0.74, 0.82,0.56,0.26,0.55,[0.3,0.68],"structural-virality estimator or reshare-tree reconstruction method changes"),
  ["diffusion-tree reconstruction","average-distance (Wiener) computation","broadcast-vs-viral classification"],
  ["origin independence (delegated)","threshold policy","baseline estimation"],
  [pro("Goel et al. show identical popularity can arise from a broadcast or true virality; structural virality tells them apart, which raw counts cannot","a star-shaped tree from one influencer is broadcast, not grassroots virality")],
  [con("Reshare-tree reconstruction is noisy and platforms expose incomplete cascade data, biasing the structural-virality estimate","missing parent edges flatten the tree toward apparent broadcast")],
  ["incomplete cascade data biases the Wiener index","conflating high reach with high structural virality","tree reconstruction errors from missing edges"],
  ["broadcast cascades score low structural virality and grassroots cascades score high on labeled trees","structural-virality estimate is stable under modest missing-edge rates"],
  ["platform changes cascade-data availability","structural-virality estimates contradict known broadcast/viral labels"],
  inputs=["reshare/cascade graph","provenance-clustered seeds"],
  outputs=["structural-virality score per cascade"],
  specialists=["content_intelligence_analyst","network_scientist"]))

N.append(node("MANUFACTURED_HYPE","manufactured_hype_adjudication",
  "Adjudicate whether a hype event is manufactured (seeded, astroturfed, or coordinated) versus genuine, by combining anomaly evidence, low independent-origin count, high bot-risk, broadcast structure, and absent search corroboration.",
  "adjudication", ["ANOMALY_DETECTION","SOURCE_INDEPENDENCE","BOT_RISK_SCORING","STRUCTURAL_VIRALITY","SEARCH_INTEREST"], ["CQ_04","CQ_10"],
  b(0.95,0.9,0.85,0.78,0.95,0.9,0.78,0.54,0.7, 0.7,0.74, 0.95,0.54,0.3,0.7,[0.32,0.72],"adjudication evidence weighting or manufactured-hype definition changes"),
  ["evidence fusion across signals","manufactured-vs-genuine verdict","confidence and rationale on the verdict"],
  ["raw-count computation","baseline estimation","provenance matching internals"],
  [pro("A multi-signal adjudication is robust to any single gameable metric, since manufacturing must defeat baseline, independence, bot, structure, and search jointly","a campaign can buy reshares but rarely also fakes independent origins and organic search")],
  [con("Fusing correlated signals risks overconfidence, and a determined operator can partially spoof several at once","a well-funded campaign seeds genuine-looking micro-influencers across surfaces")],
  ["over-weighting one signal makes the verdict gameable","correlated evidence treated as independent inflates confidence","verdict issued without recording which signals drove it"],
  ["labeled astroturf campaigns are adjudicated manufactured with high confidence","labeled genuine viral events are not flagged manufactured","every verdict records its contributing signals"],
  ["a new manufacturing tactic defeats the current signal set","verdict accuracy drops on the labeled audit set"],
  inputs=["anomaly, independence, bot, structural, and search signals"],
  outputs=["manufactured-vs-genuine verdict with rationale"],
  specialists=["content_intelligence_analyst","disinformation_analyst"]))

N.append(node("GENUINE_HYPE_THRESHOLD","independent_origin_threshold_gate",
  "Apply the hard gate that genuine hype requires at least N independent origins after provenance deduplication and method-bias discounting; below N, even large adjusted hype is classified as not-genuine regardless of volume (scope finding FP7).",
  "adjudication", ["MANUFACTURED_HYPE","COMMON_METHOD_BIAS","RAW_VS_ADJUSTED_HYPE"], ["CQ_10","CQ_11"],
  b(0.97,0.92,0.88,0.7,0.96,0.92,0.82,0.56,0.7, 0.74,0.78, 0.97,0.56,0.28,0.72,[0.3,0.7],"the independence threshold N or its discounting inputs are revised"),
  ["N-independent-origin gate","volume-cannot-substitute-for-independence rule","gate pass/fail with margin"],
  ["raw engagement maximization","bot detector internals","baseline fitting"],
  [pro("A bright-line independence threshold operationalizes the scope finding: no amount of single-origin volume can be promoted to 'genuine'","a 2M-view campaign from one independent origin fails the >=N gate")],
  [con("A fixed N is a blunt instrument; the right threshold varies by topic risk and platform, and a too-high N suppresses real niche trends","a genuine small-community trend with few but real origins is gated out")],
  ["setting N so high that real niche hype fails","letting adjusted volume override the independence gate","applying N before provenance dedup inflates the origin count"],
  ["any signal set with fewer than N independent origins is classified not-genuine regardless of adjusted volume","known genuine multi-origin trends pass the gate on the labeled set"],
  ["topic-risk policy changes the required N","real niche trends are systematically gated out"],
  inputs=["independent-origin count after dedup and discounting","adjusted hype"],
  outputs=["genuine-hype gate verdict (pass/fail) with margin"],
  specialists=["content_intelligence_analyst","policy_governance_lead"]))

N.append(node("HYPE_TRAJECTORY","hype_trajectory_classification",
  "Classify the topic's current trajectory as emerging, accelerating, plateauing, or fading using the position and slope on the fitted diffusion curve relative to its inflection point.",
  "adjudication", ["DIFFUSION_CURVE","SATURATION_ESTIMATE"], ["CQ_09","CQ_12"],
  b(0.8,0.78,0.78,0.66,0.74,0.74,0.5,0.62,0.5, 0.74,0.76, 0.8,0.62,0.2,0.48,[0.24,0.56],"trajectory-phase boundaries or slope thresholds change"),
  ["curve-position and slope estimation","emerging/accelerating/plateau/fading labeling","inflection-point detection"],
  ["origin counting","bot scoring","threshold decision"],
  [pro("Phase labels make the genuine-hype verdict actionable by saying not just whether but when and where on the curve","'accelerating with rising independent origins' is a buy signal; 'fading plateau' is not")],
  [con("Phase boundaries are fuzzy near the inflection point and a brief dip can be misread as fading","a weekend lull on a still-growing topic is mislabeled fading")],
  ["misreading a transient dip as fading","late inflection detection delays the accelerating label","phase assigned from noisy slope estimates"],
  ["assigned phases match held-out trends' realized continuation on the labeled set","phase transitions are stable under short-window noise"],
  ["slope-threshold policy changes","phase labels mismatch realized trend continuation"],
  inputs=["fitted curve position and slope","saturation fraction"],
  outputs=["trajectory phase label"],
  specialists=["content_intelligence_analyst","trend_forecasting_analyst"]))

N.append(node("CROSS_PLATFORM_RECONCILE","cross_platform_signal_reconciliation",
  "Reconcile the five platform signals onto a common time base, audience normalization, and topic definition before independence accounting, so that origin counting compares like with like.",
  "deduplication", ["ORGANIC_BASELINE","PROVENANCE_CLUSTERING"], ["CQ_03","CQ_05"],
  b(0.83,0.78,0.74,0.78,0.82,0.85,0.58,0.58,0.58, 0.72,0.74, 0.82,0.58,0.24,0.55,[0.28,0.64],"platform set changes or topic-alignment/normalization rules are revised"),
  ["time-base alignment","audience-size normalization","topic-definition harmonization across platforms"],
  ["independence verdict (delegated)","bot scoring","threshold decision"],
  [pro("Reconciling platforms first prevents spurious independence from mismatched topics or time windows masquerading as distinct origins","aligning a hashtag and a search term to one topic avoids double-counting them as origins")],
  [con("Over-harmonization can merge genuinely different framings of a topic, hiding a real second origin","two distinct but related stories forced into one topic lose a real independent origin")],
  ["misaligned time windows fabricate or hide concurrency","audience normalization errors distort relative magnitudes","topic over-merging erases a real origin"],
  ["the same artifact across platforms aligns to one topic and time window on the labeled set","distinct topics are not force-merged during reconciliation"],
  ["the platform panel changes","topic-alignment errors surface in independence audits"],
  inputs=["five platform signals","platform audience and time metadata"],
  outputs=["reconciled cross-platform signal set"],
  specialists=["content_intelligence_analyst","data_integration_engineer"]))

N.append(node("THRESHOLD_GOVERNANCE","threshold_and_n_governance",
  "Govern the numeric policy parameters (anomaly significance level, bot-risk cutoffs, saturation weights, and the independence threshold N) as versioned, auditable settings with documented rationale and review cadence.",
  "governance", ["ANOMALY_DETECTION","GENUINE_HYPE_THRESHOLD"], ["CQ_11","CQ_13"],
  b(0.85,0.8,0.74,0.6,0.82,0.85,0.7,0.62,0.55, 0.78,0.8, 0.85,0.62,0.2,0.6,[0.22,0.54],"any governed parameter is changed or a new parameter is introduced"),
  ["parameter versioning and rationale","review-cadence enforcement","change-impact audit on thresholds"],
  ["model fitting internals","verdict computation","provenance matching"],
  [pro("Treating N and thresholds as governed policy, not buried constants, keeps the genuine-vs-manufactured line auditable and contestable","a threshold change is versioned with before/after impact on labeled cases")],
  [con("Heavy governance can ossify thresholds that need fast adaptation when adversaries shift tactics","a quarterly review cadence lags a same-week manufacturing innovation")],
  ["silent threshold edits with no audit trail","thresholds tuned to a single high-profile case","governance cadence too slow for adversarial drift"],
  ["every threshold change is versioned with documented rationale and labeled-case impact","current parameter set reproduces the certified validation results"],
  ["adversary tactics shift faster than the review cadence","a parameter change degrades labeled-case accuracy"],
  inputs=["proposed parameter changes","labeled validation outcomes"],
  outputs=["versioned, audited parameter policy"],
  specialists=["content_intelligence_analyst","policy_governance_lead"]))

N.append(node("EVIDENCE_LEDGER","verdict_evidence_provenance_ledger",
  "Maintain a per-verdict evidence ledger recording the baseline, anomaly score, origin count, discounts, structural virality, search corroboration, and the parameter version, so any genuine-vs-manufactured call is reproducible and contestable.",
  "governance", ["MANUFACTURED_HYPE","GENUINE_HYPE_THRESHOLD","THRESHOLD_GOVERNANCE"], ["CQ_13","CQ_14"],
  b(0.86,0.82,0.82,0.6,0.82,0.82,0.68,0.66,0.5, 0.8,0.82, 0.86,0.66,0.18,0.55,[0.2,0.5],"ledger schema changes or a new evidence field is added to verdicts"),
  ["per-verdict evidence record","parameter-version stamping","reproducibility and contestability support"],
  ["model internals","execution of platform crawls","threshold tuning"],
  [pro("A reproducible evidence ledger turns a hype verdict into an auditable claim with provenance, essential when a manufactured-hype call is disputed","a flagged campaign's ledger shows exactly which signals and N drove the call")],
  [con("Comprehensive ledgers add storage and write cost and can leak sensitive source-attribution detail if mishandled","retaining full account-level evidence raises privacy and disclosure concerns")],
  ["a verdict issued without a complete ledger entry","ledger omits the parameter version used","sensitive source detail over-retained"],
  ["every issued verdict has a complete, parameter-stamped ledger entry","a ledger entry alone reproduces the verdict under its recorded parameters"],
  ["ledger schema changes","a disputed verdict cannot be reproduced from its ledger"],
  inputs=["all upstream signals and the parameter version","the issued verdict"],
  outputs=["reproducible per-verdict evidence ledger"],
  specialists=["content_intelligence_analyst","audit_and_compliance_engineer"]))

N.append(node("BASELINE_BACKTEST","baseline_and_model_backtest",
  "Backtest the baseline and diffusion model against historical labeled organic and manufactured episodes to calibrate residual distributions and validate that anomaly and independence signals separate the two classes.",
  "validation", ["ANOMALY_DETECTION","DIFFUSION_CURVE","SOURCE_INDEPENDENCE"], ["CQ_04","CQ_14"],
  b(0.84,0.8,0.74,0.74,0.84,0.8,0.55,0.6,0.5, 0.74,0.78, 0.84,0.6,0.24,0.55,[0.26,0.6],"the labeled backtest set or calibration window changes"),
  ["historical replay on labeled episodes","residual-distribution calibration","class-separation validation"],
  ["live verdict issuance","governance cadence","provenance matching internals"],
  [pro("Backtesting against labeled genuine and manufactured episodes turns the pipeline's thresholds into empirically calibrated, not guessed, settings","replaying a known astroturf quarter sets the bot-risk cutoff where it actually separates classes")],
  [con("Backtests overfit to past tactics and labels are noisy, so calibrated thresholds can fail on novel manufacturing","a threshold tuned on last year's farms misses a new account-aging tactic")],
  ["overfitting thresholds to historical episodes","label noise in the backtest corrupts calibration","leakage from using test episodes in baseline fitting"],
  ["on the held-out backtest split, genuine and manufactured episodes separate above the target margin","calibrated residual distributions match the held-out organic class"],
  ["new manufacturing tactics appear in production","backtest separation degrades on refreshed labels"],
  inputs=["labeled historical episodes","model and threshold settings"],
  outputs=["calibrated thresholds and separation metrics"],
  specialists=["content_intelligence_analyst","ml_evaluation_engineer"]))

N.append(node("DECISION_REPORT","genuine_hype_decision_report",
  "Assemble the final decision report: the genuine-vs-manufactured verdict, the independent-origin count versus N, the adjusted-hype magnitude and trajectory, the driving evidence, and the confidence, for downstream consumers.",
  "validation", ["GENUINE_HYPE_THRESHOLD","HYPE_TRAJECTORY","EVIDENCE_LEDGER","BASELINE_BACKTEST"], ["CQ_12","CQ_14"],
  b(0.9,0.88,0.88,0.6,0.85,0.82,0.7,0.64,0.5, 0.8,0.82, 0.9,0.64,0.18,0.6,[0.2,0.5],"report contract or downstream consumer requirements change"),
  ["verdict and origin-count vs N","adjusted-hype magnitude and trajectory","driving evidence and confidence"],
  ["model internals","crawl execution","parameter tuning"],
  [pro("A single decision report with verdict, N-comparison, magnitude, trajectory, and confidence makes the analysis directly actionable and contestable","a report reading 'genuine, 6>=N=4 origins, accelerating, high confidence' is decision-ready")],
  [con("Compressing a multi-signal analysis into one report can hide tail uncertainty unless the confidence and evidence are surfaced","a confident-sounding verdict masks a borderline origin count")],
  ["report omits the origin-count-vs-N margin","confidence not surfaced alongside the verdict","trajectory dropped, leaving timing ambiguous"],
  ["every report states verdict, origin count vs N, adjusted hype, trajectory, and confidence","a consumer can reconstruct the call from the report and its linked ledger"],
  ["report contract changes","consumers report the verdict was not actionable as delivered"],
  inputs=["gate verdict, trajectory, ledger, and backtest context"],
  outputs=["final genuine-hype decision report"],
  specialists=["content_intelligence_analyst","product_decision_lead"]))

# ---- competency questions (14) ----
CQ = [
 ("CQ_01","What is the expected organic engagement baseline and diffusion shape for a topic on a platform?",["nodes","glossary"],"ORGANIC_BASELINE sets expected engagement and DIFFUSION_CURVE the expected adoption shape",["ORGANIC_BASELINE","DIFFUSION_CURVE"]),
 ("CQ_02","How is the expected adoption S-curve characterized and how is structural virality measured?",["nodes"],"DIFFUSION_CURVE fits Bass p/q/m and STRUCTURAL_VIRALITY measures the cascade tree",["DIFFUSION_CURVE","STRUCTURAL_VIRALITY"]),
 ("CQ_03","What semi-independent demand signals corroborate engagement and how are platforms reconciled?",["nodes"],"SEARCH_INTEREST supplies a pull signal and CROSS_PLATFORM_RECONCILE aligns platform signals",["SEARCH_INTEREST","CROSS_PLATFORM_RECONCILE"]),
 ("CQ_04","How is a burst distinguished from expected variation and validated against labeled episodes?",["nodes","workflow"],"ANOMALY_DETECTION flags significant bursts and BASELINE_BACKTEST validates separation",["ANOMALY_DETECTION","MANUFACTURED_HYPE","BASELINE_BACKTEST"]),
 ("CQ_05","How are cross-posts and reposts collapsed to seeds and reconciled before counting origins?",["nodes"],"PROVENANCE_CLUSTERING collapses reposts and CROSS_PLATFORM_RECONCILE aligns them",["PROVENANCE_CLUSTERING","SOURCE_INDEPENDENCE","CROSS_PLATFORM_RECONCILE"]),
 ("CQ_06","How are distinct independent origins counted and how is common-method non-independence handled?",["nodes","conflict_axes"],"SOURCE_INDEPENDENCE counts distinct origins and COMMON_METHOD_BIAS discounts shared-method sources",["SOURCE_INDEPENDENCE","COMMON_METHOD_BIAS"]),
 ("CQ_07","How is raw engagement discounted to adjusted hype using bot risk and saturation?",["nodes"],"RAW_VS_ADJUSTED_HYPE applies BOT_RISK_SCORING and SATURATION_ESTIMATE discounts",["RAW_VS_ADJUSTED_HYPE","BOT_RISK_SCORING"]),
 ("CQ_08","How does audience saturation affect the genuineness of late-stage engagement?",["nodes"],"SATURATION_ESTIMATE estimates fraction of m reached and feeds RAW_VS_ADJUSTED_HYPE",["SATURATION_ESTIMATE","RAW_VS_ADJUSTED_HYPE"]),
 ("CQ_09","How is the hype trajectory classified and broadcast distinguished from genuine virality?",["nodes","workflow"],"HYPE_TRAJECTORY labels the phase and STRUCTURAL_VIRALITY separates broadcast from viral",["HYPE_TRAJECTORY","STRUCTURAL_VIRALITY"]),
 ("CQ_10","How is manufactured hype adjudicated against genuine hype across multiple signals?",["nodes","workflow"],"MANUFACTURED_HYPE fuses signals into a verdict gated by GENUINE_HYPE_THRESHOLD",["MANUFACTURED_HYPE","GENUINE_HYPE_THRESHOLD"]),
 ("CQ_11","Why does genuine hype require at least N independent origins after dedup, and who governs N?",["nodes","conflict_axes"],"GENUINE_HYPE_THRESHOLD enforces the >=N independence gate governed by THRESHOLD_GOVERNANCE",["GENUINE_HYPE_THRESHOLD","THRESHOLD_GOVERNANCE"]),
 ("CQ_12","What does the final genuine-hype decision report contain and how is trajectory included?",["nodes","workflow"],"DECISION_REPORT assembles the verdict, magnitude, and HYPE_TRAJECTORY phase",["DECISION_REPORT","HYPE_TRAJECTORY"]),
 ("CQ_13","How are thresholds, N, and verdict evidence governed and made reproducible?",["nodes","iteration_protocol"],"THRESHOLD_GOVERNANCE versions parameters and EVIDENCE_LEDGER records reproducible verdicts",["THRESHOLD_GOVERNANCE","EVIDENCE_LEDGER"]),
 ("CQ_14","How is the whole pipeline validated and reported so verdicts are auditable?",["nodes","workflow"],"BASELINE_BACKTEST calibrates and validates, EVIDENCE_LEDGER records, DECISION_REPORT reports",["BASELINE_BACKTEST","EVIDENCE_LEDGER","DECISION_REPORT"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

# consolidate node->CQ references onto the 14-CQ set (dense target 10-14)
CQ_MAP = {
 "ORGANIC_BASELINE":["CQ_01"], "DIFFUSION_CURVE":["CQ_01","CQ_02"], "SEARCH_INTEREST":["CQ_03"],
 "ANOMALY_DETECTION":["CQ_04"], "PROVENANCE_CLUSTERING":["CQ_05"], "SOURCE_INDEPENDENCE":["CQ_06"],
 "COMMON_METHOD_BIAS":["CQ_06"], "BOT_RISK_SCORING":["CQ_07"], "SATURATION_ESTIMATE":["CQ_08"],
 "RAW_VS_ADJUSTED_HYPE":["CQ_07","CQ_08"], "STRUCTURAL_VIRALITY":["CQ_02","CQ_09"],
 "MANUFACTURED_HYPE":["CQ_04","CQ_10"], "GENUINE_HYPE_THRESHOLD":["CQ_10","CQ_11"],
 "HYPE_TRAJECTORY":["CQ_09","CQ_12"], "CROSS_PLATFORM_RECONCILE":["CQ_03","CQ_05"],
 "THRESHOLD_GOVERNANCE":["CQ_11","CQ_13"], "EVIDENCE_LEDGER":["CQ_13","CQ_14"],
 "BASELINE_BACKTEST":["CQ_04","CQ_14"], "DECISION_REPORT":["CQ_12","CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

GL = [
 ("organic_baseline","the expected engagement a topic attracts under normal diffusion for a given platform, region, and category, used as the deviation reference",["expected_engagement","normal_diffusion_reference"],["raw_engagement"],["ORGANIC_BASELINE","ANOMALY_DETECTION"]),
 ("bass_diffusion","an adoption model with innovation coefficient p, imitation coefficient q, and market potential m yielding an S-shaped cumulative-adoption curve",["bass_model","p_q_m_model"],["instantaneous_spike"],["DIFFUSION_CURVE","SATURATION_ESTIMATE"]),
 ("structural_virality","the average shortest-path distance (Wiener index) of a diffusion tree, distinguishing broadcast from multi-generational spread",["wiener_index","cascade_depth_measure"],["raw_reach"],["STRUCTURAL_VIRALITY","HYPE_TRAJECTORY"]),
 ("independent_origin","a causally and organizationally distinct source of a topic after provenance dedup and method-bias discounting",["distinct_source","independent_seed"],["correlated_platform_echo"],["SOURCE_INDEPENDENCE","GENUINE_HYPE_THRESHOLD"]),
 ("provenance_seed","the single underlying artifact to which all its cross-posts, screenshots, and re-uploads are clustered",["content_seed","origin_artifact"],["each_repost_separately"],["PROVENANCE_CLUSTERING","SOURCE_INDEPENDENCE"]),
 ("common_method_bias","shared instrumentation or data-collection pipeline that makes nominally distinct sources non-independent",["shared_method_variance","method_correlation"],["topical_correlation"],["COMMON_METHOD_BIAS","SOURCE_INDEPENDENCE"]),
 ("adjusted_hype","raw engagement discounted for bot risk and audience saturation to approximate genuine novel human attention",["discounted_hype","genuine_attention_estimate"],["raw_engagement"],["RAW_VS_ADJUSTED_HYPE","BOT_RISK_SCORING"]),
 ("manufactured_hype","a hype event produced by seeding, astroturfing, or coordinated inauthentic behavior rather than organic demand",["astroturf","seeded_campaign"],["genuine_hype"],["MANUFACTURED_HYPE","GENUINE_HYPE_THRESHOLD"]),
 ("genuine_hype_threshold","the policy that genuine hype requires at least N independent origins after deduplication, independent of volume",["n_origin_gate","independence_gate"],["volume_threshold"],["GENUINE_HYPE_THRESHOLD","THRESHOLD_GOVERNANCE"]),
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
              "risk_of_conflict":"unmanaged tension degrades verdict quality","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated behavior",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic; mirror the key node.dependencies, ~10+ of them)
dep("ORGANIC_BASELINE","DIFFUSION_CURVE",0.86)
dep("ORGANIC_BASELINE","ANOMALY_DETECTION",0.85)
dep("DIFFUSION_CURVE","ANOMALY_DETECTION",0.82)
dep("ANOMALY_DETECTION","PROVENANCE_CLUSTERING",0.8)
dep("PROVENANCE_CLUSTERING","SOURCE_INDEPENDENCE",0.88)
dep("SOURCE_INDEPENDENCE","COMMON_METHOD_BIAS",0.8)
dep("SOURCE_INDEPENDENCE","BOT_RISK_SCORING",0.78)
dep("DIFFUSION_CURVE","SATURATION_ESTIMATE",0.78)
dep("BOT_RISK_SCORING","RAW_VS_ADJUSTED_HYPE",0.82)
dep("SATURATION_ESTIMATE","RAW_VS_ADJUSTED_HYPE",0.78)
dep("DIFFUSION_CURVE","STRUCTURAL_VIRALITY",0.78)
dep("ANOMALY_DETECTION","MANUFACTURED_HYPE",0.84)
dep("SOURCE_INDEPENDENCE","MANUFACTURED_HYPE",0.9)
dep("BOT_RISK_SCORING","MANUFACTURED_HYPE",0.82)
dep("STRUCTURAL_VIRALITY","MANUFACTURED_HYPE",0.78)
dep("MANUFACTURED_HYPE","GENUINE_HYPE_THRESHOLD",0.9)
dep("COMMON_METHOD_BIAS","GENUINE_HYPE_THRESHOLD",0.8)
dep("RAW_VS_ADJUSTED_HYPE","GENUINE_HYPE_THRESHOLD",0.78)
dep("SATURATION_ESTIMATE","HYPE_TRAJECTORY",0.74)
dep("PROVENANCE_CLUSTERING","CROSS_PLATFORM_RECONCILE",0.76)
dep("GENUINE_HYPE_THRESHOLD","THRESHOLD_GOVERNANCE",0.8)
dep("GENUINE_HYPE_THRESHOLD","EVIDENCE_LEDGER",0.8)
dep("SOURCE_INDEPENDENCE","BASELINE_BACKTEST",0.76)
dep("GENUINE_HYPE_THRESHOLD","DECISION_REPORT",0.86)

# cross-cutting non-dependency edges
rel("CROSS_PLATFORM_RECONCILE","SOURCE_INDEPENDENCE","feedback",0.8,why="reconciled like-with-like signals feed the independence count so platforms are not double-counted as origins")
rel("SEARCH_INTEREST","MANUFACTURED_HYPE","causal",0.74,why="absent search corroboration is evidence a hype event is push-manufactured")
rel("COMMON_METHOD_BIAS","BASELINE_BACKTEST","feedback",0.7,why="backtests recalibrate how much method correlation discounts independence")
rel("THRESHOLD_GOVERNANCE","ANOMALY_DETECTION","constraint",0.76,why="governed significance levels constrain what anomaly detection flags")
rel("THRESHOLD_GOVERNANCE","BOT_RISK_SCORING","constraint",0.72,why="governed bot-risk cutoffs constrain the adjusted-hype discount")
rel("BASELINE_BACKTEST","THRESHOLD_GOVERNANCE","feedback",0.76,why="backtest separation metrics drive governed threshold updates")
rel("EVIDENCE_LEDGER","DECISION_REPORT","similarity",0.74,why="the ledger supplies the evidence the report summarizes and links")
rel("STRUCTURAL_VIRALITY","HYPE_TRAJECTORY","causal",0.7,why="broadcast-vs-viral structure informs whether a trajectory reflects genuine person-to-person spread")

# conflict edges (negative signed_tension + resolution_rule)
conf("GENUINE_HYPE_THRESHOLD","RAW_VS_ADJUSTED_HYPE",0.72,-0.6,
  "independence dominates volume: when adjusted hype is large but independent origins are below N, the >=N gate fails the event regardless of magnitude (scope finding FP7)",
  "large adjusted volume from too-few independent origins conflicts with the genuine-hype independence requirement")
conf("ANOMALY_DETECTION","ORGANIC_BASELINE",0.7,-0.5,
  "recalibrate the baseline rather than the threshold when residuals drift: a platform algorithm change shifts 'normal', so refit ORGANIC_BASELINE before trusting anomaly flags",
  "a drifting organic baseline conflicts with a fixed anomaly threshold, producing systematic false bursts")
conf("PROVENANCE_CLUSTERING","SOURCE_INDEPENDENCE",0.7,-0.45,
  "tune clustering to maximize independence-count accuracy on labeled data: under-clustering inflates origins, over-clustering hides real ones, so calibrate against known independent creations",
  "aggressive provenance clustering and accurate independent-origin counting pull in opposite directions")
conf("THRESHOLD_GOVERNANCE","MANUFACTURED_HYPE",0.68,-0.5,
  "favor adaptability under audit: when adversary tactics shift faster than the review cadence, allow expedited parameter updates with retroactive backtest validation rather than waiting for the cycle",
  "slow threshold governance conflicts with rapidly evolving manufacturing tactics the adjudicator must catch")

CA=[
 {"name":"independence_count_vs_raw_volume","description":"Genuine hype is defined by independent origins, but raw volume is the loudest and most gameable signal; volume must never substitute for independence.","poles":["independent_origin_count","raw_engagement_volume"],"resolution_hint":"apply the >=N independence gate before volume is allowed to influence the genuine verdict","tension_score":0.78,"affected_nodes":["GENUINE_HYPE_THRESHOLD","RAW_VS_ADJUSTED_HYPE","SOURCE_INDEPENDENCE"]},
 {"name":"baseline_recalibration_vs_threshold_stability","description":"Algorithm changes drift the organic baseline, so a stable anomaly threshold yields false bursts; but constant recalibration undermines comparability.","poles":["recalibrate_baseline","stable_threshold"],"resolution_hint":"refit the baseline on drift and version the change; keep thresholds stable between certified baselines","tension_score":0.7,"affected_nodes":["ORGANIC_BASELINE","ANOMALY_DETECTION","THRESHOLD_GOVERNANCE"]},
 {"name":"aggressive_clustering_vs_origin_preservation","description":"Collapsing reposts prevents inflated independence, but over-clustering merges genuinely distinct independent creations and erases real origins.","poles":["aggressive_clustering","origin_preservation"],"resolution_hint":"calibrate clustering thresholds against labeled independent-creation cases","tension_score":0.72,"affected_nodes":["PROVENANCE_CLUSTERING","SOURCE_INDEPENDENCE","CROSS_PLATFORM_RECONCILE"]},
 {"name":"single_signal_simplicity_vs_multi_signal_robustness","description":"One headline metric is easy to communicate but trivially gameable; multi-signal fusion is robust but harder to interpret and can be overconfident.","poles":["single_headline_metric","multi_signal_fusion"],"resolution_hint":"fuse multiple signals for the verdict but report a single confidence-bearing summary","tension_score":0.68,"affected_nodes":["MANUFACTURED_HYPE","DECISION_REPORT","RAW_VS_ADJUSTED_HYPE"]},
 {"name":"push_engagement_vs_pull_search_corroboration","description":"Platform engagement is push-inflatable while search interest is a harder-to-fake pull signal; disagreement is informative but search is coarse and laggy.","poles":["platform_engagement","search_pull_signal"],"resolution_hint":"treat search concordance as corroborating evidence, not a veto, given its coarseness and lag","tension_score":0.62,"affected_nodes":["SEARCH_INTEREST","ANOMALY_DETECTION","MANUFACTURED_HYPE"]},
 {"name":"bot_recall_vs_grassroots_precision","description":"Aggressive bot detection catches more inauthentic engagement but risks suppressing genuine grassroots accounts as false positives.","poles":["high_bot_recall","grassroots_precision"],"resolution_hint":"tune bot-risk cutoffs to bound grassroots false positives on labeled organic cases","tension_score":0.65,"affected_nodes":["BOT_RISK_SCORING","RAW_VS_ADJUSTED_HYPE","MANUFACTURED_HYPE"]},
 {"name":"broadcast_reach_vs_structural_virality","description":"High reach from a single broadcaster looks like virality but lacks the multi-generational structure of genuine person-to-person spread.","poles":["broadcast_reach","structural_virality"],"resolution_hint":"weight structural virality, not reach alone, when judging genuine grassroots spread","tension_score":0.6,"affected_nodes":["STRUCTURAL_VIRALITY","HYPE_TRAJECTORY","DIFFUSION_CURVE"]},
 {"name":"saturation_discount_vs_late_genuine_demand","description":"Discounting plateau-phase engagement as recycled attention can wrongly penalize a genuine second wave of real new demand.","poles":["saturation_discount","late_genuine_demand"],"resolution_hint":"distinguish a fitted-curve plateau from a fresh independent-origin second wave before discounting","tension_score":0.58,"affected_nodes":["SATURATION_ESTIMATE","RAW_VS_ADJUSTED_HYPE","HYPE_TRAJECTORY"]},
 {"name":"governance_cadence_vs_adversary_adaptation","description":"Versioned, slow threshold governance preserves auditability but lags adversaries who change tactics within a review cycle.","poles":["slow_audited_governance","fast_adaptation"],"resolution_hint":"allow expedited, retroactively backtested parameter changes for confirmed adversary shifts","tension_score":0.64,"affected_nodes":["THRESHOLD_GOVERNANCE","MANUFACTURED_HYPE","BASELINE_BACKTEST"]},
]

EC=[
 {"description":"Five platforms all amplify one press release, creating apparent cross-platform consensus that is actually a single origin.","trigger":"correlated platform echoes counted as independent before provenance dedup","affected_nodes":["SOURCE_INDEPENDENCE","PROVENANCE_CLUSTERING","CROSS_PLATFORM_RECONCILE"],"mitigation":"cluster to the seed and collapse correlated platforms to one independent origin before counting","severity":"critical"},
 {"description":"A 2M-view campaign originates from a single independent origin yet has enormous adjusted hype.","trigger":"large adjusted volume from fewer than N independent origins","affected_nodes":["GENUINE_HYPE_THRESHOLD","RAW_VS_ADJUSTED_HYPE","SOURCE_INDEPENDENCE"],"mitigation":"fail the >=N independence gate regardless of volume per scope finding FP7","severity":"critical"},
 {"description":"A platform algorithm change doubles organic reach, so a stable anomaly threshold flags normal engagement as bursts.","trigger":"anomaly threshold applied against a drifted, stale organic baseline","affected_nodes":["ORGANIC_BASELINE","ANOMALY_DETECTION","THRESHOLD_GOVERNANCE"],"mitigation":"refit and version the baseline on detected drift before trusting anomaly flags","severity":"high"},
 {"description":"An astroturf network of aged, warmed accounts evades behavioral bot detection and looks grassroots.","trigger":"bot detector blind to a new account-aging evasion tactic","affected_nodes":["BOT_RISK_SCORING","MANUFACTURED_HYPE"],"mitigation":"corroborate with independence, structural virality, and search; escalate to backtest recalibration","severity":"high"},
 {"description":"Two creators independently film the same public event and perceptual hashing merges them into one seed.","trigger":"over-aggressive provenance clustering merges genuinely distinct creations","affected_nodes":["PROVENANCE_CLUSTERING","SOURCE_INDEPENDENCE"],"mitigation":"calibrate clustering against labeled independent-creation cases to preserve real origins","severity":"high"},
 {"description":"A star-shaped broadcast from one influencer reaches millions and is mistaken for grassroots virality.","trigger":"reach used as a virality proxy without structural-virality measurement","affected_nodes":["STRUCTURAL_VIRALITY","HYPE_TRAJECTORY","DIFFUSION_CURVE"],"mitigation":"compute the Wiener-index structural virality and label broadcasts as non-grassroots","severity":"medium"},
 {"description":"Two 'independent' trackers both purchase the same third-party data panel, faking corroboration.","trigger":"common-method bias from a shared data pipeline not discounted","affected_nodes":["COMMON_METHOD_BIAS","SOURCE_INDEPENDENCE"],"mitigation":"identify shared panels and discount method-correlated sources from the independence count","severity":"high"},
 {"description":"A paid search campaign lifts the trends curve so search interest falsely corroborates a push event.","trigger":"search interest treated as a clean pull signal despite ad-driven query inflation","affected_nodes":["SEARCH_INTEREST","MANUFACTURED_HYPE"],"mitigation":"treat search as corroborating not decisive and watch for ad-spend-driven query spikes","severity":"medium"},
 {"description":"A genuine niche-community trend has few but real independent origins and is gated out by a too-high N.","trigger":"independence threshold N set too high for niche topics","affected_nodes":["GENUINE_HYPE_THRESHOLD","THRESHOLD_GOVERNANCE"],"mitigation":"govern N by topic risk and review niche false-negatives in backtests","severity":"medium"},
 {"description":"A genuine second wave of new demand on a near-saturated topic is wrongly discounted as recycled attention.","trigger":"saturation discount applied without checking for fresh independent origins","affected_nodes":["SATURATION_ESTIMATE","RAW_VS_ADJUSTED_HYPE","HYPE_TRAJECTORY"],"mitigation":"check for new independent origins before applying the plateau saturation discount","severity":"medium"},
 {"description":"A manufactured-hype verdict is disputed and cannot be reproduced because the parameter version was not recorded.","trigger":"verdict issued without a complete, parameter-stamped evidence ledger","affected_nodes":["EVIDENCE_LEDGER","DECISION_REPORT","THRESHOLD_GOVERNANCE"],"mitigation":"stamp every verdict with its parameter version and full driving evidence in the ledger","severity":"high"},
 {"description":"Adversary tactics shift within a single review cycle and the governed thresholds lag, letting a campaign pass.","trigger":"governance cadence slower than adversary adaptation","affected_nodes":["THRESHOLD_GOVERNANCE","MANUFACTURED_HYPE","BASELINE_BACKTEST"],"mitigation":"allow expedited retroactively backtested parameter updates for confirmed tactic shifts","severity":"medium"},
]

WF=[
 {"action":"estimate_organic_baseline","node_ref":"ORGANIC_BASELINE","description":"Estimate expected engagement per platform/region/category and its confidence intervals as the deviation reference.","artifact":"organic_baseline_panel","gate":"baseline reproduces held-out organic items within its interval"},
 {"action":"fit_diffusion_curve","node_ref":"DIFFUSION_CURVE","description":"Fit Bass p/q/m to cumulative adoption to characterize the expected genuine adoption S-curve.","artifact":"fitted_diffusion_curve","gate":"p and q lie in plausible platform ranges; tail reproduced within tolerance"},
 {"action":"reconcile_platforms","node_ref":"CROSS_PLATFORM_RECONCILE","description":"Align the five platform signals to a common time base, audience normalization, and topic definition.","artifact":"reconciled_signal_set","gate":"the same artifact across platforms aligns to one topic and window"},
 {"action":"detect_anomalies","node_ref":"ANOMALY_DETECTION","description":"Flag bursts where observed engagement exceeds baseline and curve expectations beyond the governed significance level.","artifact":"scored_burst_flags","gate":"false-positive rate under target on labeled organic surges"},
 {"action":"cluster_provenance","node_ref":"PROVENANCE_CLUSTERING","description":"Collapse cross-posts, screenshots, and re-uploads to single provenance seeds.","artifact":"provenance_clusters","gate":"known repost chains collapse to one seed each"},
 {"action":"count_independent_origins","node_ref":"SOURCE_INDEPENDENCE","description":"Reduce signals to distinct independent origins and discount common-method correlation.","artifact":"independent_origin_count","gate":"correlated platforms and shared-method sources collapse to true origins"},
 {"action":"discount_to_adjusted_hype","node_ref":"RAW_VS_ADJUSTED_HYPE","description":"Discount raw engagement by bot risk and audience saturation to adjusted hype with an interval.","artifact":"adjusted_hype_estimate","gate":"manufactured campaigns fall far below raw; genuine stay close"},
 {"action":"measure_structural_virality","node_ref":"STRUCTURAL_VIRALITY","description":"Reconstruct the cascade tree and compute structural virality to separate broadcast from grassroots spread.","artifact":"structural_virality_score","gate":"broadcasts score low and grassroots cascades score high"},
 {"action":"adjudicate_manufactured","node_ref":"MANUFACTURED_HYPE","description":"Fuse anomaly, independence, bot, structural, and search signals into a manufactured-vs-genuine verdict.","artifact":"manufactured_verdict","gate":"labeled astroturf adjudicated manufactured; genuine not flagged"},
 {"action":"apply_independence_gate","node_ref":"GENUINE_HYPE_THRESHOLD","description":"Apply the >=N independent-origin gate; below N the event is not-genuine regardless of volume.","artifact":"genuine_hype_gate_verdict","gate":"fewer than N independent origins fails the gate regardless of adjusted volume"},
 {"action":"classify_trajectory","node_ref":"HYPE_TRAJECTORY","description":"Label the trajectory emerging, accelerating, plateauing, or fading from curve position and slope.","artifact":"trajectory_label","gate":"assigned phase matches realized continuation on labeled trends"},
 {"action":"report_decision","node_ref":"DECISION_REPORT","description":"Assemble the verdict, origin count vs N, adjusted hype, trajectory, evidence, and confidence into the decision report.","artifact":"genuine_hype_decision_report","gate":"report states verdict, origin-vs-N margin, magnitude, trajectory, and confidence"},
]

DR=[
 {"rule":"ORGANIC_BASELINE must be current (refit after any algorithm change) before ANOMALY_DETECTION flags are trusted","rationale":"a drifted baseline makes every anomaly score systematically wrong","trigger":"anomaly detection runs against a baseline older than the last algorithm change","action":"block anomaly flags until the baseline is refit and versioned"},
 {"rule":"PROVENANCE_CLUSTERING must run before SOURCE_INDEPENDENCE counts origins","rationale":"counting origins before collapsing reposts inflates apparent independence","trigger":"independent-origin counting requested on unclustered signals","action":"block counting until provenance clusters are assigned"},
 {"rule":"The GENUINE_HYPE_THRESHOLD >=N independence gate dominates raw and adjusted volume","rationale":"scope finding FP7: no amount of single-origin volume can be promoted to genuine","trigger":"a verdict would label an event genuine with fewer than N independent origins","action":"fail the gate regardless of volume and record the margin"},
 {"rule":"COMMON_METHOD_BIAS discounting must be applied before the independence count feeds the threshold","rationale":"shared-method sources are not independent and would inflate the origin count","trigger":"origin count reaches the threshold without method-bias discounting","action":"apply method-correlation discount before evaluating the gate"},
 {"rule":"MANUFACTURED_HYPE verdicts must record which signals drove them in the EVIDENCE_LEDGER","rationale":"an unrecorded verdict is not reproducible or contestable","trigger":"a verdict is issued without a ledger entry","action":"block issuance until the evidence ledger entry is complete"},
 {"rule":"BOT_RISK_SCORING cutoffs must be governed, not ad hoc, before discounting raw hype","rationale":"ungoverned cutoffs make adjusted hype non-comparable and gameable","trigger":"a bot-risk cutoff is changed outside THRESHOLD_GOVERNANCE","action":"route the change through governed, versioned parameters"},
 {"rule":"STRUCTURAL_VIRALITY must be considered before reach is treated as grassroots virality","rationale":"a broadcast and true virality can have identical reach","trigger":"a high-reach broadcast is labeled grassroots without structural measurement","action":"compute structural virality and re-label broadcasts as non-grassroots"},
 {"rule":"SATURATION_ESTIMATE must check for fresh independent origins before discounting plateau engagement","rationale":"a genuine second wave of new demand should not be discounted as recycled attention","trigger":"plateau saturation discount applied while new independent origins are appearing","action":"defer the saturation discount when new independent origins are present"},
 {"rule":"THRESHOLD_GOVERNANCE must version every parameter change with labeled-case impact before it takes effect","rationale":"silent threshold edits make the genuine-vs-manufactured line unauditable","trigger":"a governed parameter changes without a versioned rationale and impact record","action":"block the change until it is versioned with rationale and backtest impact"},
]

ARR=[
 {"rule":"Do not run anomaly detection on a stale baseline; refitting after a flagged-burst false alarm wastes the whole review","prevents":"re-reviewing a batch of false bursts caused by an un-refit baseline"},
 {"rule":"Do not count independent origins before provenance clustering; un-collapsing inflated origins later means redoing the verdict","prevents":"re-adjudicating a campaign after discovering its origins were correlated echoes"},
 {"rule":"Do not let adjusted volume override the >=N independence gate; reversing a wrongly-genuine verdict is costly and public","prevents":"retracting a 'genuine' label on a single-origin high-volume campaign"},
 {"rule":"Do not skip common-method discounting before the gate; an inflated origin count that passes must be re-litigated","prevents":"re-running the gate after discovering shared-panel non-independence"},
 {"rule":"Do not issue a verdict without a parameter-stamped ledger entry; reconstructing evidence for a dispute is expensive","prevents":"reconstructing a disputed verdict's evidence after the parameters changed"},
 {"rule":"Do not treat reach as virality without structural measurement; relabeling a broadcast later invalidates downstream reports","prevents":"reissuing reports after a broadcast was misclassified as grassroots"},
 {"rule":"Do not change governed thresholds without versioning and backtest impact; untraceable edits force a full re-certification","prevents":"re-certifying the whole pipeline after an untraceable threshold edit"},
 {"rule":"Do not over-cluster provenance to inflate dedup; recovering wrongly-merged real origins requires re-running clustering and counting","prevents":"re-deriving independence after distinct creations were merged into one seed"},
]

IP=[
 {"trigger":"anomaly false-positive rate exceeds target on review","action":"refit and re-version ORGANIC_BASELINE and re-tune the significance level in ANOMALY_DETECTION via THRESHOLD_GOVERNANCE","nodes":["ORGANIC_BASELINE","ANOMALY_DETECTION","THRESHOLD_GOVERNANCE"],"priority":"high"},
 {"trigger":"a campaign passes as genuine but is later confirmed manufactured","action":"audit SOURCE_INDEPENDENCE and COMMON_METHOD_BIAS for missed correlation and recalibrate in BASELINE_BACKTEST","nodes":["SOURCE_INDEPENDENCE","COMMON_METHOD_BIAS","BASELINE_BACKTEST"],"priority":"critical"},
 {"trigger":"a high-volume single-origin event is wrongly labeled genuine","action":"verify the >=N gate in GENUINE_HYPE_THRESHOLD dominates RAW_VS_ADJUSTED_HYPE volume","nodes":["GENUINE_HYPE_THRESHOLD","RAW_VS_ADJUSTED_HYPE","SOURCE_INDEPENDENCE"],"priority":"critical"},
 {"trigger":"a new bot or account-aging evasion tactic is observed","action":"update BOT_RISK_SCORING heuristics and recalibrate cutoffs through THRESHOLD_GOVERNANCE and BASELINE_BACKTEST","nodes":["BOT_RISK_SCORING","THRESHOLD_GOVERNANCE","BASELINE_BACKTEST"],"priority":"high"},
 {"trigger":"genuine niche trends are systematically gated out","action":"review the topic-risk-conditioned N in GENUINE_HYPE_THRESHOLD and THRESHOLD_GOVERNANCE against niche backtest false-negatives","nodes":["GENUINE_HYPE_THRESHOLD","THRESHOLD_GOVERNANCE"],"priority":"medium"},
 {"trigger":"provenance clustering merges distinct independent creations","action":"recalibrate clustering thresholds in PROVENANCE_CLUSTERING against labeled independent-creation cases and re-check SOURCE_INDEPENDENCE","nodes":["PROVENANCE_CLUSTERING","SOURCE_INDEPENDENCE"],"priority":"high"},
 {"trigger":"a verdict is disputed and cannot be reproduced","action":"audit EVIDENCE_LEDGER completeness and parameter stamping and patch DECISION_REPORT to surface the ledger link","nodes":["EVIDENCE_LEDGER","DECISION_REPORT"],"priority":"medium"},
]

spec = {
 "domain":"hype__organic_baseline",
 "domain_label":"Genuine-vs-Manufactured Hype & Source-Independence",
 "purpose":"separate_genuine_hype_from_manufactured_by_comparing_to_organic_baselines_and_requiring_independent_origins_after_provenance_dedup",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "scope is detecting genuine-vs-manufactured hype and counting independent origins; content moderation and takedown policy are out of scope",
   "per-platform engagement, search-trends, and content provenance metadata are available across five platforms",
   "genuine hype requires at least N independent origins after provenance deduplication (scope finding FP7)",
 ],
 "exclusions":[
   "content moderation, takedown, and enforcement policy",
   "legal attribution of campaign operators",
   "real-time streaming infrastructure and crawl execution internals",
   "individual-account deanonymization",
 ],
 "source_description":"heuristic prior estimates for genuine-vs-manufactured hype and source-independence work units, grounded in diffusion modeling, structural-virality, source-independence intelligence analysis, and anomaly-detection literature; no supplied dataset",
 "source_citation":"Bass 1969 A New Product Growth Model for Consumer Durables (diffusion p/q/m); Berger & Milkman 2012 What Makes Online Content Viral (JMR); Heuer 1999 Psychology of Intelligence Analysis (source independence and corroboration); Goel, Anderson, Hofman & Watts 2016 The Structural Virality of Online Diffusion (Management Science); Chandola, Banerjee & Kumar 2009 Anomaly Detection: A Survey (ACM Computing Surveys)",
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
 "priority_rationale":"ORGANIC_BASELINE and DIFFUSION_CURVE are foundational references; ANOMALY_DETECTION, PROVENANCE_CLUSTERING, and SOURCE_INDEPENDENCE establish deviation and independence; MANUFACTURED_HYPE and the >=N GENUINE_HYPE_THRESHOLD gate are the adjudication core; governance, ledger, backtest, and report close the method.",
 "eval_objective":"verify_organic_baseline_anomaly_detection_provenance_dedup_independent_origin_threshold_and_manufactured_hype_adjudication_of_hype__organic_baseline_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "hype__organic_baseline.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
