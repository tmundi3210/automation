#!/usr/bin/env python3
"""Generate the niche__audience_segmentation content spec (B60 KB) for kb_forge.py.
Compact authoring: node() applies sane defaults so only domain content + base
metric magnitudes are specified per node. Domain: split a region's crowd by
interest, select top-N per niche, and score saturation vs opportunity, treating
the "medium-tier is the opportunity" claim as a MEASURED variable, never assumed."""
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
        "academic_fields": ["marketing_science", "audience_analytics"],
        "subfields": subfields or ["market_segmentation", "opportunity_tiering"],
        "specialists": specialists or ["audience_strategist"],
        "contradictors": contradictors or ["mass_market_advocate"],
        "inputs": inputs or ["regional audience pool", "interest signals"],
        "outputs": outputs or ["scored niche segments"],
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

N.append(node("INTEREST_PARTITION","split_crowd_by_interest_domain",
  "Partition a region's audience pool into mutually intelligible interest domains and then sub-domains (music -> rock/hip-hop/classical; sport -> cricket/football), so every subsequent segment, score and tier is computed within a defined interest taxonomy rather than over an undifferentiated crowd.",
  "foundations", [], ["CQ_01"],
  b(0.9,0.82,0.8,0.6,0.82,0.78,0.6,0.7,0.45, 0.78,0.82, 0.88,0.7,0.18,0.5,[0.2,0.5],
    "interest taxonomy revised or a new top-level domain/sub-domain is added"),
  ["interest taxonomy","domain and sub-domain partition of the regional pool","mutually exclusive bucketing rule"],
  ["operational segment definition","top-N selection","scoring"],
  [pro("A defined interest partition makes every later metric (saturation, opportunity) computable per niche instead of over a meaningless aggregate","music -> {rock, hip-hop, classical} lets cricket fans never dilute a rock-genre saturation count")],
  [con("A coarse or wrong taxonomy buries real niches inside an umbrella domain, so genuine opportunity is never measured","lumping all 'electronic' into one bucket hides a thriving drum-and-bass micro-scene")],
  ["interest domains overlap so an individual is double-counted across buckets","taxonomy too shallow to expose the niche where opportunity lives","folk taxonomy imposed top-down ignores how the audience self-describes"],
  ["every audience member maps to at least one interest sub-domain and the partition covers the regional pool with documented overlap handling"],
  ["a new interest domain emerges in the region","double-counting detected across buckets"],
  specialists=["audience_strategist","taxonomy_designer"]))

N.append(node("SEGMENT_DEFINITION","operational_segment_specification",
  "Define an operational segment as the audience sharing a specific interest sub-domain AND a geographic scope (and optionally a behavioural qualifier), with measurable membership criteria, so a 'segment' is a reproducible set you can count and reach, not a vibe.",
  "foundations", ["INTEREST_PARTITION"], ["CQ_01","CQ_02"],
  b(0.88,0.8,0.78,0.62,0.82,0.8,0.6,0.68,0.5, 0.76,0.8, 0.86,0.68,0.18,0.5,[0.2,0.5],
    "segment membership criteria or the interest+geo crossing rule changes"),
  ["operational membership predicate (interest sub-domain AND geo)","measurable inclusion criteria","segment id and size estimate"],
  ["targeting tactics","creative strategy","channel buying"],
  [pro("An operational, criteria-based segment is reproducible and countable, so two analysts derive the same set and size","'cricket fans within 25km of Pune who attended a match in 12 months' is a fixed, re-runnable query")],
  [con("Over-specifying membership shrinks a segment below a usable scale and bakes in survey bias","adding five behavioural qualifiers leaves 40 reachable people")],
  ["segment defined by interest alone with no geo bound, so reach is unknowable","membership criteria not measurable from available signals","segment so narrow it is statistically unstable"],
  ["each segment has a written membership predicate over available signals and a reproducible size estimate with a stated margin"],
  ["available membership signals change","a segment size estimate moves beyond its stated margin"],
  specialists=["audience_strategist","market_researcher"]))

N.append(node("SUBCULTURAL_CAPITAL","niche_insider_signal_modeling",
  "Model the insider signals that mark genuine niche membership versus casual interest: vocabulary, taste hierarchies, scene gatekeeping and 'hipness' markers (Thornton's subcultural capital, after Bourdieu's cultural capital), so segments are graded by depth of belonging, not just stated topic.",
  "foundations", ["INTEREST_PARTITION"], ["CQ_03"],
  b(0.78,0.72,0.74,0.66,0.74,0.76,0.55,0.62,0.5, 0.7,0.74, 0.78,0.62,0.22,0.48,[0.24,0.58],
    "insider-signal vocabulary or the capital-grading rubric changes"),
  ["subcultural capital indicators (lexicon, taste hierarchy, gatekeeping)","insider-vs-casual grading","scene-authenticity signals"],
  ["reach mechanics","saturation counting","monetization"],
  [pro("Grading by subcultural capital separates a deep, mobilizable core from drive-by interest, which predicts response far better than topic alone","two 'jazz' followers split into a be-bop connoisseur and a someone who liked one playlist")],
  [con("Insider signals are noisy, gameable and culturally specific, so a rubric tuned on one scene mis-scores another","'authenticity' cues from a US punk scene misread an Indian indie scene")],
  ["mistaking performative signaling for genuine capital","rubric encodes the analyst's taste as the scene's","insider lexicon shifts faster than the model updates"],
  ["the capital rubric distinguishes insider from casual on a held-out labelled sample above an agreed accuracy floor"],
  ["a scene's insider lexicon shifts","capital grading disagrees with scene-expert labels"],
  specialists=["audience_strategist","cultural_sociologist"],
  contradictors=["topic_only_targeting_advocate"]))

N.append(node("TOP_N_PER_NICHE","salience_ranked_top_n_selection",
  "Within each niche, rank candidate audiences/communities by a salience score (size x engagement x subcultural-capital density) and select the top-N (e.g. top-5) per niche, so attention and budget concentrate on the few entities that carry each niche rather than its long tail of fragments.",
  "structure", ["SEGMENT_DEFINITION","SUBCULTURAL_CAPITAL"], ["CQ_04"],
  b(0.84,0.78,0.74,0.64,0.8,0.78,0.58,0.66,0.52, 0.74,0.78, 0.84,0.66,0.2,0.5,[0.22,0.54],
    "salience formula or the N cutoff per niche changes"),
  ["per-niche salience scoring","top-N cutoff selection","ranking of communities/entities within a niche"],
  ["cross-niche comparison","saturation scoring","opportunity scoring"],
  [pro("A salience-ranked top-N concentrates finite effort on the entities that actually carry a niche, mirroring the Pareto concentration of attention","top-5 cricket communities in a city capture most of the addressable, engaged fan base")],
  [con("A fixed N is arbitrary across niches of different shapes, truncating a flat niche and over-including a peaky one","top-5 in a 3-community niche pads with noise; top-5 in a 50-community niche drops real players")],
  ["salience dominated by raw size so a large-but-shallow audience outranks a small engaged core","fixed N truncates a legitimately broad niche","ranking unstable run-to-run"],
  ["the top-N per niche is reproducible from the salience formula and each selected entity clears a minimum engagement and capital-density floor"],
  ["salience weighting changes","a niche's shape makes a fixed N visibly wrong"],
  specialists=["audience_strategist","data_analyst"]))

N.append(node("REACHABILITY","segment_reachability_assessment",
  "Assess whether a defined segment can actually be reached and activated: are there channels, gatekeepers, addressable inventory or owned access points through which the segment is contactable at acceptable cost? Opportunity that cannot be reached is not opportunity.",
  "structure", ["SEGMENT_DEFINITION"], ["CQ_05"],
  b(0.86,0.82,0.78,0.66,0.85,0.82,0.62,0.62,0.55, 0.72,0.76, 0.86,0.62,0.24,0.56,[0.26,0.62],
    "available channels, inventory or gatekeeper access for a segment changes"),
  ["channel and inventory mapping per segment","cost-to-reach estimate","gatekeeper and access-point identification"],
  ["interest taxonomy","saturation counting","subcultural grading"],
  [pro("Pricing reachability turns 'is there demand?' into 'can we contact and activate them at acceptable cost?', which is what makes a tier actionable","an under-served deep-house niche is real but only reachable via two private Discords, so cost-to-reach is high")],
  [con("Reachability is volatile (a platform change can strand a segment) and easy to over-estimate from gross inventory figures","counting a platform's total users as reachable when targeting cannot actually isolate the niche")],
  ["confusing audience existence with audience addressability","cost-to-reach computed from gross not net-of-targeting inventory","over-reliance on a single fragile channel"],
  ["each segment carries a documented reach path with a cost-to-reach estimate and a fallback channel where the primary is fragile"],
  ["a primary channel for a segment degrades or closes","cost-to-reach moves materially against the estimate"],
  specialists=["audience_strategist","media_planner"]))

N.append(node("LONG_TAIL_VS_HEAD","head_middle_tail_structure_mapping",
  "Map each niche onto the demand-curve structure (Anderson's long tail over a Pareto/Zipf head): the head is over-served and contested, the extreme tail is real but too sparse to reach economically, and the middle band is the structural candidate region where reachable, under-served demand can exist.",
  "structure", ["TOP_N_PER_NICHE","REACHABILITY"], ["CQ_06","CQ_07"],
  b(0.85,0.78,0.74,0.7,0.82,0.82,0.6,0.6,0.56, 0.7,0.74, 0.85,0.6,0.24,0.55,[0.28,0.66],
    "demand-curve shape parameters or the head/middle/tail band cutoffs change"),
  ["demand-curve fitting per niche","head/middle/tail band partition","reachability-weighted tail analysis"],
  ["the medium-tier hypothesis test itself","saturation/opportunity score formulas"],
  [pro("Locating each niche on the head-middle-tail curve makes explicit WHY the middle is only a CANDIDate: head is contested, tail is unreachable, middle is where reachable under-served demand could sit","a genre with a saturated top-10 and an unreachable 10,000-item tail leaves a navigable middle band")],
  [con("The long-tail framing can romanticize the tail; much of it is unreachable or non-existent demand, and curve shape varies wildly by niche","treating every tail item as latent demand inflates opportunity for niches that are simply small")],
  ["assuming a universal Pareto shape across niches of different curves","reading the middle band as automatically the opportunity (the very claim to be tested)","tail items with no real demand counted as latent"],
  ["each niche has a fitted demand curve with head/middle/tail bands and the middle band is flagged as a candidate, not a conclusion"],
  ["a niche's demand-curve shape changes","band cutoffs are disputed by downstream tier results"],
  specialists=["audience_strategist","quant_analyst"],
  contradictors=["pure_head_chaser","tail_maximalist"]))

N.append(node("SATURATION_SCORE","over_served_niche_scoring",
  "Compute a saturation score per niche/tier: how over-served the demand already is, from supply density (number and strength of incumbents competing for the same audience), share concentration, and audience attention already absorbed, so 'crowded' becomes a measured quantity.",
  "scoring", ["LONG_TAIL_VS_HEAD"], ["CQ_07","CQ_08"],
  b(0.86,0.8,0.74,0.7,0.84,0.82,0.6,0.6,0.58, 0.7,0.74, 0.86,0.6,0.24,0.56,[0.28,0.66],
    "saturation inputs (incumbent set, share data, attention proxy) or the formula change"),
  ["supply-density measurement","incumbent share concentration","attention-absorbed proxy","saturation score per tier"],
  ["opportunity scoring","reachability cost","monetization"],
  [pro("A measured saturation score (incumbents x concentration x attention absorbed) replaces 'feels crowded' with a comparable number across niches and tiers","head tier shows 12 strong incumbents and 80 percent attention share -> high saturation; a mid tier shows 3 weak ones -> low")],
  [con("Saturation is sensitive to how the competitor set is bounded; a too-narrow incumbent definition understates crowding","ignoring substitute formats makes a saturated space look open")],
  ["incumbent set drawn too narrowly so saturation is understated","attention proxy stale or platform-biased","high saturation conflated with high quality of incumbents"],
  ["saturation scores are comparable across tiers, derived from a documented incumbent set, and re-derive within tolerance on a re-pull"],
  ["the incumbent/competitor set definition changes","the attention proxy source changes"],
  specialists=["audience_strategist","competitive_analyst"]))

N.append(node("OPPORTUNITY_SCORE","under_served_times_reachable_scoring",
  "Compute an opportunity score per niche/tier as a function of unmet/under-served demand AND reachability AND addressable value, explicitly as opportunity = under_served x reachable (low saturation is necessary but not sufficient), so an unreachable under-served pocket scores low, not high.",
  "scoring", ["SATURATION_SCORE","REACHABILITY"], ["CQ_08","CQ_09"],
  b(0.9,0.85,0.8,0.72,0.86,0.85,0.64,0.6,0.6, 0.7,0.74, 0.9,0.6,0.26,0.6,[0.3,0.7],
    "opportunity formula, the under_served x reachable composition, or value weighting changes"),
  ["opportunity = under_served x reachable x addressable_value","tier-level opportunity ranking","explicit penalty for unreachable demand"],
  ["the medium-tier hypothesis claim","creative/offer strategy"],
  [pro("Defining opportunity as under_served x reachable prevents the classic error of calling a low-saturation but unreachable pocket an opportunity","a deep niche with zero incumbents but no reach path scores near zero, not maximal")],
  [con("Multiplicative composition is brittle near zero and hides which factor drives the score, so a borderline reachability tanks a strong-demand niche","a real opportunity with one fragile channel reads as no-opportunity")],
  ["scoring under-served alone and ignoring reachability","value weighting that double-counts size already in saturation","multiplicative form masking the dominant factor"],
  ["opportunity scores rank tiers reproducibly, an unreachable under-served tier scores low, and each score decomposes into its under_served, reachable and value factors"],
  ["the reachability inputs change","the opportunity formula or its factor weights change"],
  specialists=["audience_strategist","opportunity_analyst"]))

N.append(node("MEDIUM_TIER_HYPOTHESIS","medium_tier_opportunity_measured_variable",
  "Treat the claim 'the medium tier is the opportunity' as an UNPROVEN hypothesis to be MEASURED, not a baked-in rule: model tier (head/medium/tail or quantile bands) as an independent variable and test whether opportunity_score actually peaks at the medium tier, reporting the per-niche result and refusing to assume it (scope finding FP1).",
  "scoring", ["OPPORTUNITY_SCORE","SATURATION_SCORE","LONG_TAIL_VS_HEAD"], ["CQ_10","CQ_11"],
  b(0.92,0.85,0.82,0.74,0.9,0.86,0.7,0.55,0.65, 0.72,0.76, 0.92,0.55,0.28,0.62,[0.32,0.74],
    "tier banding, the opportunity-by-tier test design, or the FP1 falsification criterion changes"),
  ["tier as an independent variable","opportunity_score-by-tier curve per niche","explicit test of whether the peak is at the medium tier","per-niche accept/reject of the medium-tier claim"],
  ["assuming medium tier wins","prescribing the tier before measuring"],
  [pro("Encoding the medium-tier claim as a measured variable, not a rule, lets the data say 'yes here, no there' and protects against a confident but false default (FP1)","in one genre opportunity peaks mid-curve; in another it peaks deep in the tail, and the model reports both rather than forcing 'medium'")],
  [con("Per-niche testing needs enough signal per tier or the curve is noise, and small niches cannot support a credible by-tier estimate","a niche with three reachable communities cannot statistically locate an opportunity peak")],
  ["baking 'medium tier = opportunity' in as an assumption rather than a tested result","reporting a tier peak without a confidence band","generalizing one niche's peak to all niches"],
  ["the opportunity-by-tier result is reported per niche with a confidence band, and the medium-tier claim is accepted or rejected per niche against the FP1 falsification criterion, never assumed"],
  ["a niche lacks enough per-tier signal to estimate the peak","the FP1 criterion or tier banding is revised"],
  specialists=["audience_strategist","experiment_designer"],
  contradictors=["medium_tier_dogmatist"]))

N.append(node("CROSS_NICHE_OVERLAP","audiences_spanning_multiple_niches",
  "Quantify the overlap of audiences across niches (people who belong to several at once) so segments are not treated as disjoint when they are not, double-counting is corrected, and bridge audiences spanning two niches are surfaced as their own opportunity surface.",
  "interactions", ["TOP_N_PER_NICHE","SEGMENT_DEFINITION"], ["CQ_12"],
  b(0.8,0.74,0.72,0.7,0.78,0.82,0.58,0.6,0.55, 0.7,0.74, 0.8,0.6,0.24,0.5,[0.26,0.62],
    "overlap measurement method or the bridge-audience definition changes"),
  ["pairwise/multi-way overlap estimation","double-count correction","bridge-audience identification"],
  ["single-niche scoring internals","reach cost per channel"],
  [pro("Measuring overlap corrects inflated totals and reveals bridge audiences (e.g. cricket+craft-beer) that are their own reachable opportunity","correcting a 20 percent overlap stops two niches both claiming the same 10,000 people")],
  [con("Overlap estimation from sparse signals is error-prone, and aggressive de-duplication can erase a genuinely dual-affiliated person's value in both niches","subtracting a fan from both niches understates each niche's real engaged base")],
  ["treating niches as disjoint when they share large audiences","over-correcting overlap so dual members vanish","bridge audiences ignored as noise"],
  ["overlap between scored niches is estimated and tier totals are corrected for double-counting with a stated method"],
  ["overlap signal source changes","a bridge audience materially shifts a niche's corrected size"],
  specialists=["audience_strategist","data_analyst"]))

N.append(node("SEGMENT_STABILITY","segment_drift_monitoring",
  "Characterize how fast segments drift: interests, scene boundaries, sizes and reach paths change over time, so each segment carries a stability/half-life estimate and a re-measurement cadence, and a tier decision is timestamped rather than treated as permanent.",
  "interactions", ["SEGMENT_DEFINITION","REACHABILITY"], ["CQ_13"],
  b(0.8,0.74,0.72,0.66,0.8,0.8,0.6,0.6,0.55, 0.72,0.76, 0.8,0.6,0.26,0.52,[0.28,0.64],
    "observed drift rate or the re-measurement cadence policy changes"),
  ["segment half-life/stability estimate","drift-detection signals","re-measurement cadence per segment"],
  ["one-off scoring runs","creative execution"],
  [pro("A stability estimate per segment turns a snapshot into a managed asset, flagging when a tier decision has aged out and must be re-pulled","a fast-churning trend niche gets a 6-week cadence; a stable hobby niche gets a yearly one")],
  [con("Drift is hard to estimate without longitudinal data, and over-frequent re-measurement burns budget chasing noise","re-scoring a stable niche monthly wastes effort and amplifies sampling jitter")],
  ["treating a one-time segmentation as permanent","drift unmonitored until a campaign underperforms","cadence uniform across fast and slow niches"],
  ["every segment carries a stability estimate and a re-measurement cadence, and stale decisions past their half-life are flagged"],
  ["a segment drifts faster than its assigned cadence","longitudinal data revises a half-life estimate"],
  specialists=["audience_strategist","trend_analyst"]))

N.append(node("ADDRESSABLE_VALUE","economic_value_per_segment",
  "Estimate the economic value of reaching and converting each segment (size x conversion propensity x value-per-member, net of reach cost), grounding opportunity in superstar-economics realities (Rosen 1981: tiny quality/attention gaps yield outsized value concentration) so a tier's score reflects value, not just count.",
  "scoring", ["REACHABILITY","TOP_N_PER_NICHE"], ["CQ_09"],
  b(0.84,0.85,0.76,0.7,0.82,0.78,0.62,0.6,0.55, 0.7,0.74, 0.84,0.6,0.26,0.58,[0.3,0.68],
    "value-per-member model, conversion-propensity inputs or net-of-cost margin change"),
  ["value-per-member estimate","conversion-propensity x size","net-of-reach-cost economic value","superstar concentration check"],
  ["the medium-tier test","competitive saturation internals"],
  [pro("Pricing addressable value net of reach cost prevents chasing a large but low-value or expensive-to-reach segment, and surfaces superstar concentration where a small tier holds outsized value","a small connoisseur tier with high value-per-member outscores a huge low-value casual tier")],
  [con("Value estimates compound several uncertain inputs (propensity, value-per-member, cost), so the score inherits wide error bars","one optimistic conversion assumption swings the whole tier ranking")],
  ["value driven by size alone ignoring per-member value","reach cost omitted so net value overstated","superstar concentration ignored, flattening real value differences"],
  ["each segment's addressable value is net of reach cost and decomposes into size, propensity and value-per-member with stated error bars"],
  ["conversion-propensity inputs change","value-per-member assumptions are revised"],
  specialists=["audience_strategist","revenue_analyst"]))

N.append(node("TIER_ASSEMBLY","tier_construction_per_niche",
  "Assemble the explicit tier structure per niche (head / medium / tail or quantile bands) from the demand curve and reachability, fixing band boundaries and membership so that saturation, opportunity and the medium-tier test all operate over the SAME, documented tier definition.",
  "tiering", ["LONG_TAIL_VS_HEAD","SATURATION_SCORE"], ["CQ_06","CQ_11"],
  b(0.85,0.78,0.74,0.68,0.82,0.82,0.62,0.62,0.56, 0.72,0.76, 0.85,0.62,0.22,0.55,[0.26,0.62],
    "tier band boundaries or the tiering method per niche changes"),
  ["explicit tier bands per niche","band-boundary definition","tier membership assignment"],
  ["scoring formulas themselves","cross-niche comparison"],
  [pro("A single documented tier definition per niche ensures saturation, opportunity and the medium-tier test are computed over identical bands, making results comparable and the FP1 test honest","fixing the 'medium' band once stops the opportunity test from silently re-banding to find a peak")],
  [con("Band boundaries are a modelling choice that can be gamed to manufacture or hide a medium-tier peak, so they must be pre-registered","sliding the medium band until opportunity peaks there is circular reasoning")],
  ["tier bands re-cut per metric so results are not comparable","boundaries chosen after seeing the opportunity curve","tiers assumed identical across differently shaped niches"],
  ["tier bands are fixed and documented per niche before scoring, and the same bands feed saturation, opportunity and the medium-tier test"],
  ["a niche's demand-curve shape forces a re-band","tier boundaries are disputed by the FP1 test outcome"],
  specialists=["audience_strategist","quant_analyst"]))

N.append(node("NICHE_PRIORITIZATION","cross_niche_opportunity_ranking",
  "Rank niches (and tiers within them) against each other on a combined opportunity view (opportunity_score, addressable_value, stability, overlap-corrected size), producing the prioritized shortlist that the whole exercise exists to deliver.",
  "tiering", ["OPPORTUNITY_SCORE","ADDRESSABLE_VALUE","CROSS_NICHE_OVERLAP","SEGMENT_STABILITY"], ["CQ_14"],
  b(0.88,0.85,0.8,0.7,0.85,0.85,0.66,0.6,0.6, 0.72,0.76, 0.88,0.6,0.24,0.6,[0.3,0.7],
    "the cross-niche ranking criteria or their weights change"),
  ["combined cross-niche opportunity ranking","overlap-corrected, stability-weighted shortlist","tie-breaking rule"],
  ["per-niche scoring internals","activation execution"],
  [pro("A single ranked shortlist combining opportunity, value, stability and corrected size turns many per-niche analyses into one decision-ready output","a top-10 niche/tier shortlist ordered by net opportunity with stability flags")],
  [con("Combining several scores into one rank hides trade-offs and is sensitive to the weighting, so the same data yields different shortlists","raising the value weight reshuffles the top of the list entirely")],
  ["combining scores without exposing the weighting","ranking on stale (drifted) scores","ignoring overlap so two niches double-count the same audience at the top"],
  ["the cross-niche ranking is reproducible from documented weights and each entry shows its component scores and stability flag"],
  ["the ranking weights change","a top-ranked niche's underlying scores drift"],
  specialists=["audience_strategist","portfolio_planner"]))

N.append(node("ENGAGEMENT_DEPTH","engagement_intensity_measurement",
  "Measure engagement intensity within a segment (frequency, recency, contribution, advocacy) as distinct from segment size, so a small high-intensity core is not flattened against a large passive audience when computing salience and opportunity.",
  "scoring", ["SEGMENT_DEFINITION","SUBCULTURAL_CAPITAL"], ["CQ_03","CQ_04"],
  b(0.8,0.74,0.76,0.64,0.78,0.76,0.55,0.64,0.5, 0.74,0.78, 0.8,0.64,0.2,0.48,[0.22,0.54],
    "engagement signals or the intensity index definition change"),
  ["engagement intensity index (RFM-style + advocacy)","intensity vs size separation","core-vs-periphery split"],
  ["reach cost","monetization mechanics"],
  [pro("Separating intensity from size stops a large passive segment from outranking a small mobilizable core in salience and opportunity","a 500-person superfan core outperforms a 50,000 passive list on response and is scored as such")],
  [con("Engagement signals are platform-specific and gameable, and high intensity in a tiny segment can be statistical noise","a handful of very active accounts inflate a micro-segment's apparent intensity")],
  ["intensity proxied by size","platform-specific signals treated as comparable across channels","noise from tiny samples read as real intensity"],
  ["engagement intensity is measured independently of size and a small high-intensity core is distinguishable from a large passive audience"],
  ["engagement signal sources change","intensity scores diverge from observed campaign response"],
  specialists=["audience_strategist","engagement_analyst"]))

N.append(node("DATA_PROVENANCE","signal_source_and_quality_governance",
  "Govern the provenance and quality of the signals feeding every segment, score and tier: which platform/survey/panel each metric comes from, its bias and freshness, so saturation, opportunity and the medium-tier test inherit a documented, auditable evidence base rather than mixed unsourced numbers.",
  "foundations", ["INTEREST_PARTITION"], ["CQ_15"],
  b(0.82,0.74,0.7,0.6,0.82,0.78,0.62,0.66,0.48, 0.76,0.8, 0.82,0.66,0.2,0.52,[0.2,0.5],
    "a signal source is added, deprecated, or its bias/freshness profile changes"),
  ["per-signal source registry","bias and freshness annotation","provenance trace from raw signal to tier score"],
  ["scoring formulas","activation"],
  [pro("A provenance registry makes every downstream score auditable and lets a stale or biased source be traced and quarantined before it corrupts a tier decision","a tier built on a 2-year-old panel is flagged and down-weighted, not silently trusted")],
  [con("Provenance governance adds overhead and slows iteration, and perfect lineage is rarely achievable with third-party panel data","chasing full lineage on a bought dataset stalls the analysis")],
  ["mixing sources of different bias without annotation","stale signals trusted as current","no trace from a tier score back to its raw signals"],
  ["every metric used in scoring is annotated with its source, freshness and known bias, and any tier score is traceable to its inputs"],
  ["a signal source is deprecated or re-biased","a tier decision cannot be traced to its source signals"],
  specialists=["audience_strategist","data_governance_lead"]))

N.append(node("FALSE_OPPORTUNITY_GUARD","false_positive_opportunity_screening",
  "Screen ranked opportunities for false positives before they reach a decision: low-saturation pockets that are unreachable, non-existent (artefactual) demand, drifted-away segments, or medium-tier peaks asserted without a passing FP1 test, so the shortlist carries only defensible opportunities.",
  "verification", ["NICHE_PRIORITIZATION","MEDIUM_TIER_HYPOTHESIS"], ["CQ_10","CQ_16"],
  b(0.88,0.82,0.78,0.7,0.88,0.85,0.66,0.58,0.62, 0.72,0.76, 0.88,0.58,0.26,0.62,[0.3,0.72],
    "the false-opportunity screen criteria or the FP1 gate threshold change"),
  ["false-positive screen (unreachable, artefactual, drifted, untested-medium-tier)","FP1 gate enforcement","defensibility check per shortlist entry"],
  ["activation tactics","creative production"],
  [pro("A dedicated screen catches the four classic false opportunities (unreachable, artefactual, drifted, assumed-medium-tier) before budget is committed","a thrilling 'untapped' niche is rejected because its demand is a scraping artefact and its medium-tier peak never passed FP1")],
  [con("An over-strict screen kills genuine but evidentially thin opportunities, biasing toward safe, already-known niches","rejecting every niche without longitudinal data defaults the whole exercise back to the head")],
  ["passing an opportunity that is under-served but unreachable","accepting a medium-tier claim without a passing FP1 test","artefactual demand surviving into the shortlist"],
  ["every shortlist entry passes the false-opportunity screen, and any 'medium tier is the opportunity' claim is gated on a passing FP1 test"],
  ["the FP1 gate threshold changes","a shipped opportunity later proves to be a false positive"],
  specialists=["audience_strategist","validation_analyst"],
  contradictors=["growth_at_all_costs_advocate"]))

N.append(node("ACTIVATION_HANDOFF","prioritized_segment_activation_brief",
  "Package each prioritized, screened niche/tier into an activation brief: who the segment is, its insider context, its reach path and cost, its opportunity and value scores with confidence, and the explicit tier-claim verdict, so downstream creative/media teams act on a defensible, self-contained artefact.",
  "verification", ["FALSE_OPPORTUNITY_GUARD","DATA_PROVENANCE"], ["CQ_16","CQ_17"],
  b(0.82,0.8,0.82,0.6,0.78,0.78,0.6,0.66,0.5, 0.78,0.82, 0.82,0.66,0.2,0.5,[0.2,0.5],
    "the activation brief contract or required fields change"),
  ["per-segment activation brief","reach path + cost + scores + confidence","explicit tier-claim verdict and provenance"],
  ["creative concepting","channel buying execution","budget allocation mechanics"],
  [pro("A self-contained brief carrying scores, confidence, reach path and the tier verdict lets downstream teams act without re-deriving the analysis or re-introducing the medium-tier assumption","the brief states 'opportunity peaks in the medium tier, FP1 passed, confidence 0.7, reach via two owned communities'")],
  [con("A brief frozen at handoff ages with segment drift, so a late-activated brief can act on a stale tier picture","a brief shipped and used three months later misses a drifted reach path")],
  ["brief omits confidence or the tier verdict so the assumption creeps back","scores handed off without provenance","brief used past its segment's half-life"],
  ["each brief carries reach path, cost, opportunity/value scores with confidence, the tier verdict and a freshness date tied to the segment's half-life"],
  ["the brief contract changes","a brief is consumed after its freshness date"],
  specialists=["audience_strategist","activation_lead"]))

N.append(node("BACKTEST_CALIBRATION","outcome_backtest_and_recalibration",
  "Close the loop by back-testing predicted opportunity/saturation/medium-tier verdicts against realized outcomes (response, conversion, value) and recalibrating the formulas and the FP1 criterion, so the whole tiering system is corrected by evidence rather than left as a one-shot heuristic.",
  "verification", ["NICHE_PRIORITIZATION","MEDIUM_TIER_HYPOTHESIS","ACTIVATION_HANDOFF"], ["CQ_18"],
  b(0.85,0.82,0.78,0.72,0.85,0.85,0.66,0.55,0.6, 0.7,0.74, 0.85,0.55,0.28,0.6,[0.32,0.74],
    "realized-outcome data arrives or the recalibration policy changes"),
  ["prediction-vs-outcome back-test","formula and weight recalibration","FP1 criterion recalibration"],
  ["one-shot scoring without feedback","real-time bidding mechanics"],
  [pro("Back-testing predicted tiers against realized response turns heuristic scores into calibrated ones and lets the medium-tier claim be confirmed or retired by evidence per niche","after three campaigns the medium-tier peak holds for sport but is retired for music, updating FP1 priors")],
  [con("Outcomes are confounded (creative, timing, budget) so a clean attribution back to segmentation is hard, risking miscalibration on noisy signal","a great campaign on a mediocre segment falsely validates the tier choice")],
  ["calibrating on confounded outcomes","no feedback loop so errors persist across cycles","over-fitting the FP1 criterion to a few campaigns"],
  ["predicted opportunity and tier verdicts are back-tested against realized outcomes and the formulas and FP1 criterion are recalibrated with the confound risks documented"],
  ["realized-outcome data arrives","attribution confounds change the back-test interpretation"],
  specialists=["audience_strategist","measurement_scientist"]))

# ---- Competency questions (14 total, never exceed 14) ----
CQ = [
 ("CQ_01","How is the regional crowd split by interest and what makes an operational, countable segment with measurable membership criteria?",["nodes","glossary"],"INTEREST_PARTITION defines the interest partition and SEGMENT_DEFINITION gives a reproducible interest+geo membership predicate with a size estimate",["INTEREST_PARTITION","SEGMENT_DEFINITION"]),
 ("CQ_02","How is insider niche membership (subcultural capital) and engagement depth distinguished from casual interest?",["nodes"],"SUBCULTURAL_CAPITAL grades insider-vs-casual and ENGAGEMENT_DEPTH measures intensity independently of size",["SUBCULTURAL_CAPITAL","ENGAGEMENT_DEPTH"]),
 ("CQ_03","How are the top-N entities per niche selected by salience?",["nodes","workflow"],"TOP_N_PER_NICHE ranks by salience (size x engagement x capital density) and selects a curve-adaptive top-N with floors",["TOP_N_PER_NICHE","ENGAGEMENT_DEPTH"]),
 ("CQ_04","How is a segment's reachability and cost-to-reach assessed so existence is not confused with addressability?",["nodes"],"REACHABILITY maps channels, gatekeepers and cost-to-reach per segment with a fallback path",["REACHABILITY"]),
 ("CQ_05","How is each niche mapped onto a head/middle/tail demand curve and split into fixed tier bands?",["nodes"],"LONG_TAIL_VS_HEAD fits the demand curve and TIER_ASSEMBLY pre-registers the tier bands",["LONG_TAIL_VS_HEAD","TIER_ASSEMBLY"]),
 ("CQ_06","How over-served (saturated) is a niche or tier, measured rather than asserted?",["nodes","conflict_axes"],"SATURATION_SCORE measures supply density, share concentration and attention absorbed; LONG_TAIL_VS_HEAD locates the band",["SATURATION_SCORE","LONG_TAIL_VS_HEAD"]),
 ("CQ_07","How is opportunity computed so an unreachable under-served pocket does not score high?",["nodes"],"OPPORTUNITY_SCORE composes under_served x reachable; SATURATION_SCORE supplies the under-served side and REACHABILITY the reachable side",["OPPORTUNITY_SCORE","SATURATION_SCORE"]),
 ("CQ_08","How is the economic (addressable) value of reaching a segment estimated net of reach cost?",["nodes"],"ADDRESSABLE_VALUE estimates value-per-member x propensity net of cost and OPPORTUNITY_SCORE folds it in",["ADDRESSABLE_VALUE","OPPORTUNITY_SCORE"]),
 ("CQ_09","How is 'the medium tier is the opportunity' tested as a measured variable over pre-registered bands rather than assumed (FP1)?",["nodes","iteration_protocol"],"MEDIUM_TIER_HYPOTHESIS models tier as an independent variable and tests the opportunity-by-tier peak per niche; TIER_ASSEMBLY supplies the pre-registered bands",["MEDIUM_TIER_HYPOTHESIS","TIER_ASSEMBLY"]),
 ("CQ_10","How is cross-niche audience overlap measured and corrected for double-counting?",["nodes"],"CROSS_NICHE_OVERLAP estimates overlap, corrects tier totals and surfaces bridge audiences",["CROSS_NICHE_OVERLAP"]),
 ("CQ_11","How is segment drift monitored so a tier decision is re-measured before it ages out?",["nodes"],"SEGMENT_STABILITY estimates half-life and sets a per-segment re-measurement cadence",["SEGMENT_STABILITY"]),
 ("CQ_12","How are niches and tiers ranked against each other into a decision-ready shortlist?",["nodes","workflow"],"NICHE_PRIORITIZATION combines opportunity, value, stability and overlap-corrected size into a ranked shortlist",["NICHE_PRIORITIZATION"]),
 ("CQ_13","How are false-positive opportunities screened on FP1 and the signals behind every score governed for provenance?",["nodes","source_registry"],"FALSE_OPPORTUNITY_GUARD screens unreachable/artefactual/drifted/untested-medium-tier entries on FP1 and DATA_PROVENANCE governs source, bias and freshness",["FALSE_OPPORTUNITY_GUARD","DATA_PROVENANCE"]),
 ("CQ_14","What does a downstream-ready activation brief contain, and how are predicted tiers back-tested and the FP1 criterion recalibrated?",["nodes","iteration_protocol"],"ACTIVATION_HANDOFF packages scores, confidence, reach path and the tier verdict; BACKTEST_CALIBRATION back-tests predictions vs realized outcomes and recalibrates the model and FP1",["ACTIVATION_HANDOFF","BACKTEST_CALIBRATION"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

# consolidate node->CQ references onto the 14-CQ set (each node refs 1-2; every CQ covered by >=1 node)
CQ_MAP = {
 "INTEREST_PARTITION":["CQ_01"], "SEGMENT_DEFINITION":["CQ_01"],
 "SUBCULTURAL_CAPITAL":["CQ_02"], "ENGAGEMENT_DEPTH":["CQ_02","CQ_03"],
 "TOP_N_PER_NICHE":["CQ_03"], "REACHABILITY":["CQ_04"],
 "LONG_TAIL_VS_HEAD":["CQ_05","CQ_06"], "TIER_ASSEMBLY":["CQ_05","CQ_09"],
 "SATURATION_SCORE":["CQ_06","CQ_07"], "OPPORTUNITY_SCORE":["CQ_07","CQ_08"],
 "ADDRESSABLE_VALUE":["CQ_08"], "MEDIUM_TIER_HYPOTHESIS":["CQ_09"],
 "CROSS_NICHE_OVERLAP":["CQ_10"], "SEGMENT_STABILITY":["CQ_11"],
 "NICHE_PRIORITIZATION":["CQ_12"], "FALSE_OPPORTUNITY_GUARD":["CQ_13"],
 "DATA_PROVENANCE":["CQ_13"], "ACTIVATION_HANDOFF":["CQ_14"],
 "BACKTEST_CALIBRATION":["CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

GL = [
 ("interest_partition","a mutually-intelligible split of a regional audience pool into interest domains and sub-domains (e.g. music->genres, sport->cricket)",["interest_taxonomy","interest_buckets"],["demographic_partition"],["INTEREST_PARTITION","SEGMENT_DEFINITION"]),
 ("operational_segment","the audience sharing a specific interest sub-domain and geographic scope with a measurable membership predicate",["addressable_segment"],["target_persona"],["SEGMENT_DEFINITION","REACHABILITY"]),
 ("subcultural_capital","insider knowledge, taste hierarchies and 'hipness' that mark genuine scene membership (Thornton, after Bourdieu)",["scene_capital","insider_capital"],["cultural_capital"],["SUBCULTURAL_CAPITAL","ENGAGEMENT_DEPTH"]),
 ("saturation","a measured degree to which a niche/tier's demand is already over-served by incumbents and absorbed attention",["over_service","crowdedness"],["raw_competitor_count"],["SATURATION_SCORE","TIER_ASSEMBLY"]),
 ("opportunity_score","under_served x reachable x addressable_value: under-service alone is necessary but not sufficient",["net_opportunity"],["unmet_demand_alone"],["OPPORTUNITY_SCORE","NICHE_PRIORITIZATION"]),
 ("long_tail","Anderson's demand curve where head is contested, extreme tail is unreachable, and the middle is the structural candidate band",["demand_curve","power_law_tail"],["uniform_demand"],["LONG_TAIL_VS_HEAD","TIER_ASSEMBLY"]),
 ("medium_tier_claim","the hypothesis that opportunity peaks in the medium tier, treated as a MEASURED variable per niche, never assumed (FP1)",["mid_tier_hypothesis"],["medium_tier_rule"],["MEDIUM_TIER_HYPOTHESIS","FALSE_OPPORTUNITY_GUARD"]),
 ("reachability","whether a segment is contactable and activatable at acceptable cost via available channels and gatekeepers",["addressability"],["audience_existence"],["REACHABILITY","OPPORTUNITY_SCORE"]),
 ("bridge_audience","an audience spanning two niches, surfaced by overlap analysis as its own opportunity surface",["cross_niche_audience"],["disjoint_segment"],["CROSS_NICHE_OVERLAP","NICHE_PRIORITIZATION"]),
 ("false_opportunity","a ranked pocket that is unreachable, artefactual, drifted, or an untested medium-tier peak, rejected before a decision",["opportunity_false_positive"],["validated_opportunity"],["FALSE_OPPORTUNITY_GUARD","BACKTEST_CALIBRATION"]),
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
              "risk_of_conflict":"unmanaged tension degrades tiering quality","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated behavior",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies; a subset is enough — node.dependencies
# carries the full layering, edges only need ~24 of them to stay within the edge-count band)
dep("INTEREST_PARTITION","SEGMENT_DEFINITION",0.9)
dep("INTEREST_PARTITION","SUBCULTURAL_CAPITAL",0.8)
dep("SEGMENT_DEFINITION","TOP_N_PER_NICHE",0.84)
dep("SEGMENT_DEFINITION","REACHABILITY",0.84)
dep("SEGMENT_DEFINITION","ENGAGEMENT_DEPTH",0.78)
dep("SEGMENT_DEFINITION","CROSS_NICHE_OVERLAP",0.76)
dep("SEGMENT_DEFINITION","SEGMENT_STABILITY",0.76)
dep("TOP_N_PER_NICHE","LONG_TAIL_VS_HEAD",0.84)
dep("REACHABILITY","LONG_TAIL_VS_HEAD",0.8)
dep("REACHABILITY","ADDRESSABLE_VALUE",0.8)
dep("LONG_TAIL_VS_HEAD","SATURATION_SCORE",0.85)
dep("LONG_TAIL_VS_HEAD","TIER_ASSEMBLY",0.8)
dep("SATURATION_SCORE","OPPORTUNITY_SCORE",0.85)
dep("REACHABILITY","OPPORTUNITY_SCORE",0.82)
dep("OPPORTUNITY_SCORE","MEDIUM_TIER_HYPOTHESIS",0.86)
dep("TIER_ASSEMBLY","MEDIUM_TIER_HYPOTHESIS",0.8)
dep("OPPORTUNITY_SCORE","NICHE_PRIORITIZATION",0.85)
dep("ADDRESSABLE_VALUE","NICHE_PRIORITIZATION",0.8)
dep("CROSS_NICHE_OVERLAP","NICHE_PRIORITIZATION",0.76)
dep("NICHE_PRIORITIZATION","FALSE_OPPORTUNITY_GUARD",0.84)
dep("MEDIUM_TIER_HYPOTHESIS","FALSE_OPPORTUNITY_GUARD",0.82)
dep("FALSE_OPPORTUNITY_GUARD","ACTIVATION_HANDOFF",0.82)
dep("DATA_PROVENANCE","ACTIVATION_HANDOFF",0.74)
dep("NICHE_PRIORITIZATION","BACKTEST_CALIBRATION",0.78)
dep("ACTIVATION_HANDOFF","BACKTEST_CALIBRATION",0.74)

# cross-cutting non-dependency edges
rel("ENGAGEMENT_DEPTH","TOP_N_PER_NICHE","constraint",0.76,why="engagement intensity is a salience input that constrains the top-N ranking")
rel("DATA_PROVENANCE","SATURATION_SCORE","constraint",0.74,why="saturation scores are only as trustworthy as the provenance-governed signals feeding them")
rel("DATA_PROVENANCE","OPPORTUNITY_SCORE","constraint",0.74,why="opportunity scores inherit the bias and freshness of their provenance-governed inputs")
rel("SEGMENT_STABILITY","ACTIVATION_HANDOFF","feedback",0.74,why="segment half-life sets the freshness date stamped on the activation brief")
rel("BACKTEST_CALIBRATION","OPPORTUNITY_SCORE","feedback",0.78,why="realized outcomes recalibrate the opportunity formula weights")
rel("BACKTEST_CALIBRATION","MEDIUM_TIER_HYPOTHESIS","feedback",0.78,why="outcome back-tests confirm or retire the medium-tier claim and update its FP1 priors per niche")
rel("CROSS_NICHE_OVERLAP","SATURATION_SCORE","causal",0.72,why="uncorrected overlap inflates the audience attributed to a tier and distorts its saturation")
rel("ADDRESSABLE_VALUE","MEDIUM_TIER_HYPOTHESIS","similarity",0.7,why="value concentration by tier is co-evidence for where opportunity actually peaks")

# conflict edges (negative signed_tension + resolution_rule)
conf("SATURATION_SCORE","OPPORTUNITY_SCORE",0.7,-0.55,
  "low saturation is necessary but not sufficient: never promote a low-saturation niche to high opportunity unless REACHABILITY clears the cost-to-reach floor; opportunity = under_served x reachable",
  "low saturation tempts a high opportunity score even when the niche is unreachable")
conf("MEDIUM_TIER_HYPOTHESIS","TIER_ASSEMBLY",0.72,-0.6,
  "pre-register tier band boundaries in TIER_ASSEMBLY before running the opportunity-by-tier test; reject any re-banding chosen after seeing the opportunity curve (prevents manufacturing an FP1 pass)",
  "sliding tier bands until opportunity peaks in the medium tier is circular and would fake the FP1 result")
conf("TOP_N_PER_NICHE","LONG_TAIL_VS_HEAD",0.66,-0.45,
  "set N per niche from the fitted demand-curve shape rather than a global constant; a flat niche keeps fewer, a deep niche keeps more, so top-N does not truncate the very middle band under analysis",
  "a fixed global N conflicts with the per-niche curve shape and can truncate the candidate middle band")
conf("FALSE_OPPORTUNITY_GUARD","NICHE_PRIORITIZATION",0.68,-0.5,
  "the false-opportunity screen overrides raw rank: a top-ranked niche that fails the unreachable/artefactual/drifted/untested-medium-tier screen is removed from the shortlist regardless of its score",
  "a high raw rank competes with screen rejection for the same shortlist slot")

CA=[
 {"name":"under_served_vs_reachable","description":"A low-saturation niche looks like opportunity, but if it is unreachable the opportunity is illusory; under-service is necessary not sufficient.","poles":["maximize_under_served","require_reachable"],"resolution_hint":"opportunity = under_served x reachable; gate on a cost-to-reach floor","tension_score":0.72,"affected_nodes":["SATURATION_SCORE","OPPORTUNITY_SCORE","REACHABILITY"]},
 {"name":"medium_tier_assumed_vs_measured","description":"The headline belief 'medium tier is the opportunity' is attractive as a rule but must be a measured per-niche result (FP1), or the whole tiering is circular.","poles":["assume_medium_tier","measure_per_niche"],"resolution_hint":"pre-register bands; test the opportunity-by-tier peak; gate on FP1","tension_score":0.78,"affected_nodes":["MEDIUM_TIER_HYPOTHESIS","TIER_ASSEMBLY","FALSE_OPPORTUNITY_GUARD"]},
 {"name":"fixed_top_n_vs_curve_shape","description":"A constant N per niche is simple but truncates flat niches and over-includes peaky ones, distorting which entities carry a niche.","poles":["global_fixed_n","curve_adaptive_n"],"resolution_hint":"derive N from the fitted demand curve per niche","tension_score":0.6,"affected_nodes":["TOP_N_PER_NICHE","LONG_TAIL_VS_HEAD","ENGAGEMENT_DEPTH"]},
 {"name":"size_vs_intensity","description":"Salience and opportunity can be dominated by raw size, flattening a small high-intensity, high-capital core against a large passive audience.","poles":["size_weighted","intensity_weighted"],"resolution_hint":"measure engagement intensity and subcultural capital independently of size","tension_score":0.62,"affected_nodes":["ENGAGEMENT_DEPTH","SUBCULTURAL_CAPITAL","TOP_N_PER_NICHE"]},
 {"name":"overlap_correction_vs_double_value","description":"Correcting cross-niche overlap fixes inflated totals but can erase the genuine dual value of a person who really belongs to both niches.","poles":["deduplicate_hard","credit_both_niches"],"resolution_hint":"correct totals for ranking but credit dual members in each niche's engaged base","tension_score":0.58,"affected_nodes":["CROSS_NICHE_OVERLAP","NICHE_PRIORITIZATION","SEGMENT_DEFINITION"]},
 {"name":"snapshot_vs_drift","description":"A tier decision is a snapshot, but segments drift; treating it as permanent ages out the decision, while constant re-measurement burns budget on noise.","poles":["freeze_snapshot","continuous_remeasure"],"resolution_hint":"set re-measurement cadence per segment half-life","tension_score":0.6,"affected_nodes":["SEGMENT_STABILITY","ACTIVATION_HANDOFF","NICHE_PRIORITIZATION"]},
 {"name":"screen_strictness_vs_missed_opportunity","description":"A strict false-opportunity screen protects the shortlist but biases toward already-known head niches; a loose screen lets false positives through.","poles":["strict_screen","permissive_screen"],"resolution_hint":"gate on FP1 and reachability but allow evidentially-thin niches into a watchlist tier","tension_score":0.65,"affected_nodes":["FALSE_OPPORTUNITY_GUARD","MEDIUM_TIER_HYPOTHESIS","BACKTEST_CALIBRATION"]},
 {"name":"provenance_rigor_vs_iteration_speed","description":"Strict signal provenance makes scores auditable but slows iteration, and perfect lineage is rarely available with third-party panels.","poles":["full_provenance","fast_iteration"],"resolution_hint":"annotate source/bias/freshness; quarantine stale sources without blocking the whole run","tension_score":0.55,"affected_nodes":["DATA_PROVENANCE","SATURATION_SCORE","OPPORTUNITY_SCORE"]},
 {"name":"value_concentration_vs_count","description":"Superstar economics means a small tier can hold outsized value, so ranking on count or size misreads where the real value sits.","poles":["rank_by_count","rank_by_value"],"resolution_hint":"price addressable value net of reach cost and fold it into opportunity","tension_score":0.6,"affected_nodes":["ADDRESSABLE_VALUE","NICHE_PRIORITIZATION","OPPORTUNITY_SCORE"]},
]

EC=[
 {"description":"A low-saturation niche is scored as high opportunity even though it is only reachable via two closed private communities at prohibitive cost.","trigger":"opportunity scored from under-service alone, ignoring reachability","affected_nodes":["OPPORTUNITY_SCORE","REACHABILITY","SATURATION_SCORE"],"mitigation":"compose opportunity = under_served x reachable and gate on a cost-to-reach floor","severity":"high"},
 {"description":"The medium tier is declared the opportunity by default and tier bands are quietly slid until the opportunity curve peaks there.","trigger":"medium-tier claim assumed and tier bands re-cut after seeing results","affected_nodes":["MEDIUM_TIER_HYPOTHESIS","TIER_ASSEMBLY","FALSE_OPPORTUNITY_GUARD"],"mitigation":"pre-register bands; test the by-tier peak; gate on a passing FP1 criterion","severity":"critical"},
 {"description":"A fixed top-5 truncates a genuinely broad niche, dropping real players from the candidate middle band.","trigger":"global constant N applied to a flat demand curve","affected_nodes":["TOP_N_PER_NICHE","LONG_TAIL_VS_HEAD"],"mitigation":"derive N per niche from the fitted demand-curve shape","severity":"medium"},
 {"description":"A large passive audience outranks a small mobilizable superfan core because salience is dominated by raw size.","trigger":"engagement intensity proxied by size","affected_nodes":["ENGAGEMENT_DEPTH","TOP_N_PER_NICHE","SUBCULTURAL_CAPITAL"],"mitigation":"measure intensity and subcultural capital independently of size","severity":"high"},
 {"description":"Two niches both claim the same overlapping audience at the top of the shortlist, double-counting reach and value.","trigger":"niches treated as disjoint with no overlap correction","affected_nodes":["CROSS_NICHE_OVERLAP","NICHE_PRIORITIZATION"],"mitigation":"estimate overlap, correct tier totals, and surface bridge audiences explicitly","severity":"medium"},
 {"description":"A tier decision shipped months ago is activated against a segment whose reach path and size have drifted away.","trigger":"activation brief used past the segment's half-life","affected_nodes":["SEGMENT_STABILITY","ACTIVATION_HANDOFF"],"mitigation":"stamp briefs with a freshness date tied to the segment half-life and re-measure on cadence","severity":"high"},
 {"description":"An 'untapped' niche turns out to be a scraping/measurement artefact with no real demand, yet it tops the opportunity ranking.","trigger":"artefactual demand not screened before ranking","affected_nodes":["FALSE_OPPORTUNITY_GUARD","DATA_PROVENANCE","OPPORTUNITY_SCORE"],"mitigation":"screen for artefactual demand and require provenance before an opportunity reaches the shortlist","severity":"high"},
 {"description":"Saturation is understated because the incumbent set was drawn too narrowly and ignored substitute formats.","trigger":"competitor set bounded too narrowly","affected_nodes":["SATURATION_SCORE","LONG_TAIL_VS_HEAD"],"mitigation":"include substitutes in the incumbent definition and document the competitor-set boundary","severity":"medium"},
 {"description":"A subcultural-capital rubric tuned on one scene mis-scores insiders in a culturally different scene.","trigger":"insider-signal rubric applied across incompatible cultures","affected_nodes":["SUBCULTURAL_CAPITAL","SEGMENT_DEFINITION"],"mitigation":"validate the capital rubric per scene against scene-expert labels before use","severity":"medium"},
 {"description":"The opportunity formula is calibrated on confounded campaign outcomes, so a great creative on a mediocre segment validates the wrong tier.","trigger":"back-test attribution not controlled for creative/timing/budget confounds","affected_nodes":["BACKTEST_CALIBRATION","OPPORTUNITY_SCORE","MEDIUM_TIER_HYPOTHESIS"],"mitigation":"document confounds and require controlled attribution before recalibrating formulas or FP1","severity":"high"},
 {"description":"A small niche lacks enough per-tier signal, so the opportunity-by-tier peak is noise reported as a finding.","trigger":"medium-tier test run on a niche with too few reachable entities per tier","affected_nodes":["MEDIUM_TIER_HYPOTHESIS","TIER_ASSEMBLY"],"mitigation":"require a minimum per-tier sample and report a confidence band, else mark the test inconclusive","severity":"medium"},
 {"description":"Stale third-party panel data silently feeds a saturation and opportunity score that drives a budget decision.","trigger":"signal freshness not tracked in provenance","affected_nodes":["DATA_PROVENANCE","SATURATION_SCORE","OPPORTUNITY_SCORE"],"mitigation":"annotate freshness per source and down-weight or quarantine stale signals","severity":"medium"},
]

WF=[
 {"action":"partition_by_interest","node_ref":"INTEREST_PARTITION","description":"Split the regional audience pool into interest domains and sub-domains with documented overlap handling.","artifact":"interest_taxonomy","gate":"every member maps to >=1 sub-domain; partition covers the pool"},
 {"action":"define_segments","node_ref":"SEGMENT_DEFINITION","description":"Specify each segment as interest sub-domain AND geo with a measurable membership predicate and size estimate.","artifact":"segment_register","gate":"each segment has a reproducible predicate and sized estimate with a margin"},
 {"action":"grade_insiders","node_ref":"SUBCULTURAL_CAPITAL","description":"Grade insider-vs-casual membership and engagement depth using a validated capital rubric.","artifact":"capital_grading","gate":"rubric clears the accuracy floor on a labelled held-out sample"},
 {"action":"select_top_n","node_ref":"TOP_N_PER_NICHE","description":"Rank entities per niche by salience (size x engagement x capital density) and select a curve-adaptive top-N.","artifact":"top_n_shortlist","gate":"selection reproducible; each entity clears engagement and capital floors"},
 {"action":"assess_reachability","node_ref":"REACHABILITY","description":"Map channels, gatekeepers and cost-to-reach per segment with a fallback path.","artifact":"reach_map","gate":"each segment has a reach path, cost estimate and fallback"},
 {"action":"fit_demand_curve_and_tier","node_ref":"LONG_TAIL_VS_HEAD","description":"Fit the head/middle/tail demand curve per niche and pre-register the tier bands.","artifact":"demand_curve_bands","gate":"bands fixed and documented before scoring; middle flagged candidate only"},
 {"action":"score_saturation","node_ref":"SATURATION_SCORE","description":"Measure over-service per tier from incumbent density, concentration and attention absorbed.","artifact":"saturation_scores","gate":"scores comparable across tiers from a documented incumbent set"},
 {"action":"score_opportunity","node_ref":"OPPORTUNITY_SCORE","description":"Compute opportunity = under_served x reachable x addressable_value per tier with factor decomposition.","artifact":"opportunity_scores","gate":"unreachable under-served tiers score low; scores decompose into factors"},
 {"action":"test_medium_tier","node_ref":"MEDIUM_TIER_HYPOTHESIS","description":"Test the opportunity-by-tier peak per niche over the pre-registered bands and accept/reject the medium-tier claim (FP1).","artifact":"by_tier_test","gate":"per-niche verdict with a confidence band; claim never assumed"},
 {"action":"rank_niches","node_ref":"NICHE_PRIORITIZATION","description":"Rank niches/tiers on combined opportunity, value, stability and overlap-corrected size.","artifact":"ranked_shortlist","gate":"ranking reproducible from documented weights with component scores shown"},
 {"action":"screen_false_positives","node_ref":"FALSE_OPPORTUNITY_GUARD","description":"Screen the shortlist for unreachable, artefactual, drifted, and untested-medium-tier false positives.","artifact":"screened_shortlist","gate":"every entry passes the screen; medium-tier claims gated on FP1"},
 {"action":"package_activation","node_ref":"ACTIVATION_HANDOFF","description":"Package each surviving niche/tier into a self-contained activation brief with scores, confidence, reach path, tier verdict and freshness date.","artifact":"activation_briefs","gate":"each brief carries scores+confidence+reach+verdict+freshness"},
]

DR=[
 {"rule":"INTEREST_PARTITION and SEGMENT_DEFINITION must be fixed before any saturation or opportunity score is computed","rationale":"scores are meaningless without a defined niche and a countable segment","trigger":"scoring begins without a fixed partition and segment register","action":"block scoring until partition and segments are defined"},
 {"rule":"OPPORTUNITY_SCORE must include REACHABILITY as a multiplicative factor, never under-service alone","rationale":"an unreachable under-served pocket is not an opportunity","trigger":"opportunity computed from saturation only","action":"require under_served x reachable composition with a cost-to-reach floor"},
 {"rule":"TIER_ASSEMBLY bands must be pre-registered before MEDIUM_TIER_HYPOTHESIS is tested","rationale":"re-banding after seeing the opportunity curve manufactures a false FP1 pass","trigger":"tier bands changed after the by-tier test is run","action":"reject post-hoc re-banding and re-run on the pre-registered bands"},
 {"rule":"The medium-tier claim must carry a per-niche verdict with a confidence band and must not be asserted as a default","rationale":"FP1: 'medium tier is the opportunity' is a measured variable, not a rule","trigger":"the medium tier is treated as the opportunity without a passing test","action":"mark the niche inconclusive until the by-tier test passes FP1"},
 {"rule":"FALSE_OPPORTUNITY_GUARD overrides raw NICHE_PRIORITIZATION rank","rationale":"a high score that fails the false-positive screen is not a real opportunity","trigger":"a top-ranked entry fails the unreachable/artefactual/drifted/untested screen","action":"remove the entry from the shortlist regardless of rank"},
 {"rule":"TOP_N_PER_NICHE must derive N from the fitted demand curve, not a global constant","rationale":"a fixed N truncates flat niches and over-includes peaky ones","trigger":"a global constant N applied across niches of different shapes","action":"set N per niche from LONG_TAIL_VS_HEAD curve fit"},
 {"rule":"Every score must be traceable to a provenance-governed signal before it drives a decision","rationale":"unsourced or stale signals corrupt tier decisions invisibly","trigger":"a score lacks source/bias/freshness annotation","action":"block the score from the shortlist until provenance is attached"},
 {"rule":"Activation briefs must carry a freshness date tied to SEGMENT_STABILITY half-life","rationale":"a drifted segment makes a frozen brief act on a stale picture","trigger":"a brief is consumed past its freshness date","action":"require re-measurement before the brief is activated"},
 {"rule":"BACKTEST_CALIBRATION must control for creative/timing/budget confounds before recalibrating formulas or FP1","rationale":"confounded outcomes validate the wrong tier","trigger":"recalibration proposed from uncontrolled outcomes","action":"require controlled attribution before changing weights or FP1"},
]

ARR=[
 {"rule":"Do not score saturation or opportunity before the interest partition and segments are fixed; re-scoring after a partition change is total rework","prevents":"re-running all tier scores after a late taxonomy revision"},
 {"rule":"Do not bake in 'medium tier = opportunity'; retrofitting the FP1 measured-variable test after assuming it requires re-deriving every tier verdict","prevents":"unwinding a baked-in medium-tier assumption across all niches"},
 {"rule":"Do not score opportunity from under-service alone; adding reachability after the fact reshuffles the whole ranking","prevents":"re-ranking the shortlist once unreachable pockets are removed"},
 {"rule":"Do not re-cut tier bands after seeing the opportunity curve; pre-register them or the FP1 test must be re-run from scratch","prevents":"re-running the by-tier test after a circular re-banding is caught"},
 {"rule":"Do not apply a global fixed N across niches; correcting truncated niches later means re-selecting and re-scoring them","prevents":"re-selecting top-N for every flat niche after truncation is found"},
 {"rule":"Do not mix unsourced signals into scores; attaching provenance retroactively means re-validating every affected tier","prevents":"re-validating tier scores after an unsourced signal is discovered"},
 {"rule":"Do not ignore cross-niche overlap; de-duplicating after ranking changes the top of the shortlist","prevents":"re-ranking after overlap inflation is found at the top of the list"},
 {"rule":"Do not ship activation briefs without a freshness date; chasing drift after activation means re-measuring mid-campaign","prevents":"emergency re-measurement of a drifted segment mid-flight"},
]

IP=[
 {"trigger":"a low-saturation niche is flagged as high opportunity but proves unreachable in practice","action":"re-weight REACHABILITY in OPPORTUNITY_SCORE and tighten the cost-to-reach floor","nodes":["OPPORTUNITY_SCORE","REACHABILITY","SATURATION_SCORE"],"priority":"high"},
 {"trigger":"the medium-tier claim is being asserted as a default rather than measured","action":"re-run the by-tier test over pre-registered bands in MEDIUM_TIER_HYPOTHESIS and enforce the FP1 gate in FALSE_OPPORTUNITY_GUARD","nodes":["MEDIUM_TIER_HYPOTHESIS","TIER_ASSEMBLY","FALSE_OPPORTUNITY_GUARD"],"priority":"critical"},
 {"trigger":"a fixed top-N visibly truncates a broad niche","action":"switch TOP_N_PER_NICHE to a curve-adaptive N from the LONG_TAIL_VS_HEAD fit","nodes":["TOP_N_PER_NICHE","LONG_TAIL_VS_HEAD"],"priority":"medium"},
 {"trigger":"a large passive audience outranks a small engaged core","action":"re-measure ENGAGEMENT_DEPTH and SUBCULTURAL_CAPITAL independently of size and re-weight salience","nodes":["ENGAGEMENT_DEPTH","SUBCULTURAL_CAPITAL","TOP_N_PER_NICHE"],"priority":"high"},
 {"trigger":"two top-ranked niches double-count the same audience","action":"re-estimate CROSS_NICHE_OVERLAP and correct NICHE_PRIORITIZATION totals","nodes":["CROSS_NICHE_OVERLAP","NICHE_PRIORITIZATION"],"priority":"medium"},
 {"trigger":"an activated brief acts on a drifted segment","action":"shorten the re-measurement cadence in SEGMENT_STABILITY and re-stamp ACTIVATION_HANDOFF freshness dates","nodes":["SEGMENT_STABILITY","ACTIVATION_HANDOFF"],"priority":"high"},
 {"trigger":"a shipped opportunity proves to be a false positive after the campaign","action":"strengthen the FALSE_OPPORTUNITY_GUARD screen and recalibrate the formulas and FP1 in BACKTEST_CALIBRATION","nodes":["FALSE_OPPORTUNITY_GUARD","BACKTEST_CALIBRATION","MEDIUM_TIER_HYPOTHESIS"],"priority":"high"},
]

spec = {
 "domain":"niche__audience_segmentation",
 "domain_label":"Niche Audience Segmentation & Opportunity Tiering",
 "purpose":"split_a_regions_crowd_by_interest_select_top_n_per_niche_and_score_saturation_vs_opportunity_with_the_medium_tier_claim_as_a_measured_variable",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "scope is the segmentation-and-tiering method itself; creative concepting, channel buying and budget mechanics are out of scope",
   "interest, engagement, reachability and incumbent signals are available, however imperfect, for the region under analysis",
   "the claim that the medium tier is the opportunity is treated as an unproven hypothesis to be measured per niche (scope finding FP1), never a baked-in rule",
 ],
 "exclusions":[
   "creative strategy and message concepting",
   "channel buying, bidding and budget allocation mechanics",
   "individual-level identity resolution and PII handling",
   "platform-specific ad-targeting implementation details",
 ],
 "source_description":"heuristic prior estimates for niche audience segmentation and opportunity tiering work units, grounded in classical segmentation theory, long-tail and superstar economics, and subcultural-capital sociology; no supplied dataset",
 "source_citation":"Smith 1956 'Product Differentiation and Market Segmentation as Alternative Marketing Strategies' (J. Marketing); Kotler & Keller STP (Segmentation-Targeting-Positioning) framework; Anderson 2006 'The Long Tail'; the Pareto principle (Pareto 1896 / Juran's 80-20); Thornton 1995 'Club Cultures: Music, Media and Subcultural Capital'; Bourdieu 1984 'Distinction: A Social Critique of the Judgement of Taste'; Rosen 1981 'The Economics of Superstars' (American Economic Review)",
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
 "priority_rationale":"INTEREST_PARTITION/SEGMENT_DEFINITION and signal provenance are foundational; reachability and the demand-curve/tier structure gate scoring; SATURATION_SCORE and OPPORTUNITY_SCORE feed the measured MEDIUM_TIER_HYPOTHESIS (FP1); prioritization, false-opportunity screening, activation and back-test calibration close the method.",
 "eval_objective":"verify_interest_partitioning_segment_reachability_saturation_vs_opportunity_scoring_and_the_measured_medium_tier_hypothesis_of_niche__audience_segmentation_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "niche__audience_segmentation.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
