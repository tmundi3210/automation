#!/usr/bin/env python3
"""Generate the geo__attention_topology content spec (B60 KB) for kb_forge.py.
Compact authoring: node() applies sane defaults so only domain content + base
metric magnitudes are specified per node. Domain = a finite, reproducible
attention web over a region hierarchy with a defined salience metric, decreasing-
order traversal and a bounded frontier. Grounded in Zipf 1949 rank-size,
Christaller central place theory, Rogers 2003 diffusion, Barabasi-Albert 1999
scale-free networks, Simon 1971 / Davenport-Beck 2001 attention economy,
Anderson 2006 long tail."""
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
        "academic_fields": ["economic_geography", "computational_social_science"],
        "subfields": subfields or ["attention_economy", "regional_science"],
        "specialists": specialists or ["attention_topologist"],
        "contradictors": contradictors or ["uniform_salience_advocate"],
        "inputs": inputs or ["region hierarchy", "public attention signals"],
        "outputs": outputs or ["scored attention sub-web"],
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

N.append(node("REGION_TAXONOMY","finite_region_hierarchy_definition",
  "Define the finite, closed hierarchy of geographic and demographic units the attention web is built over: country -> state/province -> city/metro, with market tiers (tier-1/2/3) attached, so every place has exactly one parent and the universe of nodes is enumerable and bounded.",
  "foundations", [], ["CQ_01"],
  b(0.92,0.82,0.78,0.55,0.84,0.8,0.66,0.7,0.42, 0.78,0.82, 0.9,0.7,0.16,0.5,[0.2,0.5],"administrative boundaries redrawn or a new market-tier scheme adopted"),
  ["country/state/city levels","market-tier labels","single-parent containment"],
  ["salience scoring","cross-border edges (handled by diaspora bridges)"],
  [pro("A closed single-parent hierarchy guarantees the node universe is finite and traversal terminates","India>Punjab>Ludhiana is one path; no place has two parents")],
  [con("Rigid administrative nesting hides cross-cutting demographic communities that ignore borders","a metro area that spans two states is forced into one parent")],
  ["overlapping or multi-parent regions break containment","unbounded 'neighborhood' granularity explodes the node set"],
  ["every region resolves to exactly one parent and the full set is enumerable and finite"],
  ["administrative boundaries change","a needed granularity level is missing from the hierarchy"],
  specialists=["regional_scientist","attention_topologist"],
  subfields=["central_place_theory","regional_science"],
  contradictors=["borderless_network_advocate"]))

N.append(node("SALIENCE_METRIC","quantitative_attention_score",
  "Specify the scalar salience score s(place) = weighted aggregate of obtainable public attention signals (Google Trends search interest, follower/subscriber counts, news article volume, Wikipedia pageviews), with explicit signal weights and units, so attention is a measurable quantity rather than a hunch.",
  "foundations", ["REGION_TAXONOMY"], ["CQ_02","CQ_03"],
  b(0.94,0.85,0.82,0.66,0.88,0.85,0.62,0.64,0.5, 0.74,0.78, 0.92,0.64,0.2,0.55,[0.24,0.6],"a signal source's API or methodology changes, or signal weights are revised"),
  ["signal selection","weighted aggregation formula","per-signal units and provenance"],
  ["true latent attention (unobservable)","private platform analytics"],
  [pro("A defined, weighted metric makes salience reproducible and comparable across places","s = 0.4*trends + 0.3*log(followers) + 0.2*news_volume + 0.1*pageviews")],
  [con("Public signals proxy attention imperfectly; the metric measures visibility, not the latent construct","a place with no search-trends coverage scores low despite real local attention")],
  ["a single dominant signal swamps the aggregate","signals with incomparable units summed without normalization"],
  ["s(place) is computable from named public sources and reproduces to within tolerance on re-pull","weights are documented and sum to a fixed total"],
  ["a signal source is deprecated","the weighting scheme is challenged by observed bias"],
  specialists=["data_scientist","attention_topologist"],
  subfields=["attention_economy","computational_social_science"],
  contradictors=["qualitative_attention_advocate"]))

N.append(node("SALIENCE_NORMALIZATION","cross_region_score_normalization",
  "Normalize raw salience so unequal-size regions are comparable: divide by or regress out population/baseline-volume, convert to per-capita or z-scored attention, so a small high-intensity place is not always dominated by a large low-intensity one.",
  "foundations", ["SALIENCE_METRIC"], ["CQ_03"],
  b(0.86,0.78,0.74,0.7,0.82,0.82,0.58,0.6,0.55, 0.72,0.76, 0.84,0.6,0.22,0.5,[0.26,0.62],"the population/baseline denominator or normalization scheme changes"),
  ["per-capita normalization","baseline-volume adjustment","z-scoring within a level"],
  ["raw signal collection","ranking (consumes the normalized score)"],
  [pro("Normalization lets a small region with intense per-capita attention compete with a populous one","a small diaspora hub outranks a large city on per-capita salience")],
  [con("Over-normalizing can erase the genuine mass-attention advantage of large hubs","per-capita scoring makes a megacity look unremarkable")],
  ["normalizing by a stale population figure distorts ranks","mixing normalized and raw scores in one ranking"],
  ["scores are comparable across levels: two regions with equal per-capita attention receive equal normalized scores"],
  ["population baselines are updated","a normalization artifact inverts an expected rank"],
  specialists=["statistician","attention_topologist"],
  subfields=["statistics","regional_science"]))

N.append(node("SIGNAL_SOURCING","public_signal_acquisition",
  "Acquire the raw public attention signals that feed the salience metric from obtainable sources (Google Trends, platform follower counts, GDELT/news volume, Wikipedia pageview API), recording source, timestamp, and access method for each datum.",
  "ingestion", ["SALIENCE_METRIC"], ["CQ_02"],
  b(0.84,0.76,0.7,0.64,0.78,0.74,0.55,0.66,0.48, 0.78,0.78, 0.82,0.66,0.2,0.46,[0.2,0.5],"a source's terms of service, rate limit, or schema changes"),
  ["source connectors","raw signal capture","provenance stamping"],
  ["weighting/aggregation (done in salience metric)","ranking"],
  [pro("Stamping each datum with source and timestamp makes the whole web auditable and re-pullable","every follower count carries the platform and the UTC pull time")],
  [con("Public APIs rate-limit and change schemas, so sourcing is the most fragile layer","Google Trends caps query volume and rescales to 0-100 per query window")],
  ["a source rate-limit silently truncates coverage","un-timestamped data cannot be reproduced or revisited"],
  ["each signal datum carries source id, timestamp, and access method and can be re-pulled","sourcing covers all regions at the seed level"],
  ["a source API changes","coverage gaps appear for a region level"],
  specialists=["data_engineer","attention_topologist"],
  subfields=["data_engineering","computational_social_science"]))

N.append(node("ENTITY_DISAMBIGUATION","place_entity_signal_resolution",
  "Resolve which place/entity each public signal actually refers to before it is scored: disambiguate homonymous place names, language variants, and entity-vs-place collisions (e.g. search interest for 'Punjab' spanning India, Pakistan, and namesake places abroad) so a signal is attributed to the correct hierarchy node.",
  "ingestion", ["SIGNAL_SOURCING","REGION_TAXONOMY"], ["CQ_02","CQ_04"],
  b(0.84,0.76,0.72,0.74,0.84,0.8,0.58,0.58,0.55, 0.72,0.74, 0.84,0.58,0.24,0.55,[0.28,0.66],"a naming/geocoding scheme changes or a new homonym collision is found"),
  ["homonym place resolution","language-variant unification","entity-vs-place attribution"],
  ["signal acquisition mechanics","ranking"],
  [pro("Correct attribution stops one node's salience from being inflated by a homonym's signal","'Punjab' search interest is split between the Indian and Pakistani regions, not summed onto one")],
  [con("Disambiguation needs a gazetteer/geocoder whose coverage and recency are themselves uncertain","a small place absent from the gazetteer is mis-attributed to its larger homonym")],
  ["homonymous signals merged onto one node","language variants of a place name counted as separate places"],
  ["every scored signal is attributed to exactly one hierarchy node via a documented disambiguation rule"],
  ["the gazetteer/geocoding scheme changes","a homonym mis-attribution is detected in residuals"],
  specialists=["geocoding_engineer","attention_topologist"],
  subfields=["geocoding","computational_social_science"],
  contradictors=["raw_name_match_advocate"]))

N.append(node("SAMPLING_BIAS","platform_visibility_bias_correction",
  "Characterize and correct platform-skewed visibility: the signals measure what a platform surfaces, not true salience. Identify per-platform demographic and geographic skews (age, language, urban tilt) and down-weight or re-balance signals accordingly.",
  "ingestion", ["ENTITY_DISAMBIGUATION"], ["CQ_04"],
  b(0.86,0.78,0.74,0.74,0.86,0.84,0.6,0.56,0.6, 0.7,0.72, 0.86,0.56,0.26,0.58,[0.3,0.7],"platform demographics shift or a new dominant platform emerges in a region"),
  ["per-platform skew profiles","re-weighting against demographics","coverage-gap flags"],
  ["signal acquisition mechanics","final ranking"],
  [pro("Modeling platform skew separates 'platform visibility' from 'true regional attention'","a youth-skewed platform is down-weighted when scoring an older-demographic region")],
  [con("Bias correction needs a ground-truth demographic baseline that is itself uncertain","census language data lags actual diaspora language use")],
  ["uncorrected urban/English-language tilt inflates already-visible places","over-correction invents attention where a platform simply is absent"],
  ["each platform signal carries a documented skew profile and a correction factor before aggregation","known coverage gaps are flagged not silently zero-filled"],
  ["a platform's user base shifts demographically","a systematic geographic bias is detected in residuals"],
  specialists=["survey_methodologist","attention_topologist"],
  subfields=["survey_methodology","computational_social_science"],
  contradictors=["raw_signal_purist"]))

N.append(node("RANK_SIZE_DISTRIBUTION","zipf_rank_size_of_attention",
  "Fit the rank-size (Zipf / power-law) distribution of salience across places and entities within a level: rank r places have salience approximately s(1)/r^a. Estimate the exponent a, validate the heavy tail, and use it to predict how concentrated attention is.",
  "structure", ["SALIENCE_NORMALIZATION"], ["CQ_05","CQ_06"],
  b(0.88,0.8,0.74,0.72,0.82,0.82,0.58,0.6,0.55, 0.72,0.74, 0.88,0.6,0.22,0.52,[0.26,0.62],"the fitted Zipf exponent drifts or the distribution stops being heavy-tailed"),
  ["rank-size fit","exponent estimation","heavy-tail goodness-of-fit"],
  ["traversal order (consumes the ranking)","decay modeling"],
  [pro("A Zipfian fit quantifies attention concentration and predicts the few hubs that dominate","top-3 cities hold most national attention, matching exponent a~1, per Zipf 1949")],
  [con("Naive power-law fitting overstates tails; many distributions are lognormal, not strictly Zipfian","a maximum-likelihood fit per Clauset et al. rejects power-law for some levels")],
  ["fitting a power law where the data is lognormal misstates concentration","ignoring finite-size cutoff at the tail"],
  ["the rank-size fit reproduces on re-pull and passes a tail goodness-of-fit threshold","the exponent is reported with a confidence interval"],
  ["the exponent drifts beyond its interval","goodness-of-fit rejects the heavy-tail assumption"],
  specialists=["complexity_scientist","attention_topologist"],
  subfields=["power_law_statistics","economic_geography"],
  contradictors=["lognormal_advocate"]))

N.append(node("SCALE_FREE_LINKAGE","preferential_attachment_topology",
  "Model the attention web's link topology as scale-free: salient hubs attract disproportionately many links (preferential attachment, Barabasi-Albert 1999), producing a few high-degree attention hubs and many low-degree places. Use degree distribution to identify structural hubs distinct from raw-salience leaders.",
  "structure", ["RANK_SIZE_DISTRIBUTION"], ["CQ_06"],
  b(0.82,0.74,0.68,0.74,0.78,0.82,0.55,0.58,0.55, 0.7,0.72, 0.82,0.58,0.24,0.5,[0.3,0.66],"the link-formation model or degree-distribution assumption changes"),
  ["degree distribution","hub identification","preferential-attachment link weighting"],
  ["raw salience scoring","seed selection"],
  [pro("Scale-free structure explains why a handful of hubs mediate most cross-region attention flow","a single diaspora-hub city links dozens of smaller regions, per Barabasi-Albert 1999")],
  [con("Real attention networks deviate from pure preferential attachment; degree alone misranks bridges","a low-degree but high-betweenness place is a critical bridge the model under-weights")],
  ["treating degree as the only hub signal hides high-betweenness bridges","assuming pure BA growth ignores aging/fitness of attention"],
  ["the degree distribution is heavy-tailed and identified hubs overlap with high-salience leaders on a sanity check"],
  ["the topology stops being scale-free","a critical bridge is mis-ranked by degree alone"],
  specialists=["network_scientist","attention_topologist"],
  subfields=["network_science","complex_systems"]))

N.append(node("ATTENTION_HALF_LIFE","salience_temporal_decay",
  "Model the temporal decay of salience: attention is perishable and follows an exponential-like decay with a place/topic-specific half-life (Simon 1971 scarce attention; Davenport-Beck 2001). Estimate the half-life so the map is declared valid only over a short, stated time window.",
  "temporal", ["SALIENCE_METRIC"], ["CQ_07"],
  b(0.86,0.78,0.74,0.66,0.84,0.78,0.6,0.6,0.55, 0.72,0.74, 0.86,0.6,0.24,0.55,[0.28,0.64],"observed decay rate changes or a viral spike resets the half-life"),
  ["half-life estimation","exponential-decay fit","validity-window derivation"],
  ["re-pull scheduling (consumes the half-life)","spatial structure"],
  [pro("A measured half-life turns 'the map is stale' into a quantitative validity window","a news-driven spike decays with a 9-day half-life, so the map is valid ~2 weeks")],
  [con("A single half-life hides bimodal decay: a fast news component plus a slow baseline","a celebrity's baseline followers persist while their news spike vanishes")],
  ["assuming a single decay rate misstates validity for mixed signals","ignoring decay yields a confidently wrong stale map"],
  ["each place/topic carries an estimated half-life and a derived validity window","decayed signals below threshold are flagged, not used as current"],
  ["a viral event resets decay dynamics","measured decay diverges from the fitted model"],
  specialists=["time_series_analyst","attention_topologist"],
  subfields=["time_series_analysis","attention_economy"],
  contradictors=["static_map_advocate"]))

N.append(node("SEED_SELECTION","start_region_choice",
  "Choose the seed region the traversal starts from (e.g. Punjab, the USA, or a specific major city), justified by salience, coverage, and the analysis goal, since the seed anchors the whole expansion and biases what the web emphasizes.",
  "traversal", ["RANK_SIZE_DISTRIBUTION","SAMPLING_BIAS"], ["CQ_08"],
  b(0.84,0.78,0.78,0.6,0.82,0.8,0.66,0.64,0.5, 0.76,0.78, 0.84,0.64,0.2,0.5,[0.22,0.54],"the analysis goal or the seed's own salience materially changes"),
  ["seed justification","seed-level signal coverage","goal-aligned anchoring"],
  ["frontier expansion mechanics","scoring"],
  [pro("A justified, salience-anchored seed makes the web's emphasis explicit and reproducible","seeding at Punjab makes diaspora bridges to Canada/UK first-class, not incidental")],
  [con("Seed choice biases the entire map; a different seed yields a different attention web","seeding at the USA buries a regionally salient but globally minor place")],
  ["an arbitrary seed produces a web that does not reflect the stated goal","seed lacks signal coverage, starving the traversal"],
  ["the seed is justified against salience and goal, and has full signal coverage at its level"],
  ["the analysis goal changes","the seed's salience rank shifts materially"],
  specialists=["domain_analyst","attention_topologist"],
  subfields=["regional_science","computational_social_science"]))

N.append(node("EXPANSION_FRONTIER","decreasing_order_traversal",
  "Traverse the region/entity space in strictly DECREASING salience (size/hype) order: maintain a priority frontier keyed by salience, always expand the most salient unvisited node next, so the most important attention is mapped first and the order is deterministic.",
  "traversal", ["SEED_SELECTION","SCALE_FREE_LINKAGE"], ["CQ_08","CQ_09"],
  b(0.92,0.82,0.8,0.72,0.88,0.85,0.66,0.6,0.6, 0.7,0.74, 0.92,0.6,0.24,0.58,[0.28,0.66],"the traversal order key or priority semantics change"),
  ["salience-keyed priority frontier","decreasing-order expansion","deterministic tie-breaking"],
  ["bound enforcement (delegated to depth/breadth bounds)","decay modeling"],
  [pro("Decreasing-order traversal maps the highest-attention regions first and is deterministic given the salience scores","the frontier pops the top city before any tier-3 town, like a best-first search")],
  [con("Pure greedy salience order can starve structurally important but low-salience bridge nodes","a low-salience but high-betweenness bridge is visited late or never under the bound")],
  ["non-deterministic tie-breaking makes the traversal irreproducible","greedy order strands critical bridges below the frontier cutoff"],
  ["the traversal expands nodes in strictly non-increasing salience order with deterministic tie-breaking","re-running the traversal on the same scores yields the same order"],
  ["the priority key definition changes","a critical bridge is consistently stranded by greedy order"],
  specialists=["search_engineer","attention_topologist"],
  subfields=["graph_traversal","network_science"],
  contradictors=["random_walk_advocate"]))

N.append(node("DEPTH_BREADTH_BOUNDS","bounded_frontier_termination",
  "Cap the frontier size and the per-node top_k fan-out (and total node budget / max depth) so the traversal provably terminates and the web stays finite, converting potential unbounded expansion into an explicit, handleable stop condition.",
  "control", ["EXPANSION_FRONTIER"], ["CQ_09","CQ_10"],
  b(0.9,0.8,0.74,0.64,0.86,0.8,0.64,0.66,0.55, 0.78,0.8, 0.9,0.66,0.18,0.55,[0.2,0.5],"the node budget, top_k, max-depth, or frontier cap policy changes"),
  ["frontier-size cap","per-node top_k fan-out cap","total node budget / max depth"],
  ["proof of termination semantics (delegated)","scoring"],
  [pro("Explicit caps guarantee finite, terminating traversal and bound compute cost","top_k=5 per node and a 500-node budget cap the entire web's size")],
  [con("Caps set too tight truncate salient regions; too loose let the web explode toward the long tail","a top_k=2 cap drops the 3rd-ranked but still-salient neighbor, per Anderson 2006 long tail")],
  ["a cap set below true required breadth drops salient nodes","an unbounded fan-out lets the long tail explode the node set"],
  ["every traversal terminates within the node budget and max depth with explicit caps","exceeding a cap raises a documented stop/defeater, not a crash"],
  ["the bound policy changes","truncation of a salient region is observed at the cap boundary"],
  specialists=["systems_engineer","attention_topologist"],
  subfields=["graph_traversal","complex_systems"]))

N.append(node("TOPK_FANOUT","per_node_neighbor_selection",
  "At each expanded node, select the top_k highest-salience neighbors/children to enqueue, so fan-out is bounded and only the most salient links propagate, implementing the long-tail cutoff (Anderson 2006) at the local level.",
  "control", ["DEPTH_BREADTH_BOUNDS"], ["CQ_10"],
  b(0.82,0.74,0.7,0.62,0.78,0.78,0.55,0.64,0.52, 0.76,0.78, 0.82,0.64,0.2,0.48,[0.22,0.54],"the top_k value or neighbor-ranking rule changes"),
  ["per-node neighbor ranking","top_k cutoff","long-tail truncation"],
  ["global frontier cap (handled by bounds)","raw sourcing"],
  [pro("Local top_k selection bounds branching and keeps only the most salient neighbors","a city enqueues its 5 most-salient linked places and drops the long tail")],
  [con("A fixed top_k ignores that some hubs legitimately have more salient neighbors than others","a top global hub with 20 salient links is cut to k=5, losing real structure")],
  ["a uniform top_k under-serves true hubs and over-serves leaves","ties at the kth position broken non-deterministically"],
  ["each expanded node enqueues exactly its top_k normalized-salience neighbors with deterministic tie-breaks"],
  ["the top_k policy changes","a true hub is observed to be over-truncated by a uniform k"]))

N.append(node("DIASPORA_BRIDGES","cross_border_salient_communities",
  "Model diaspora and cross-border salient communities as bridge nodes/edges that link otherwise-separate region subtrees (e.g. Punjabi communities linking India, Canada, the UK), since attention diffuses across borders through these communities (Rogers 2003 diffusion of innovations).",
  "structure", ["SCALE_FREE_LINKAGE","REGION_TAXONOMY"], ["CQ_11"],
  b(0.8,0.74,0.74,0.72,0.78,0.85,0.55,0.56,0.58, 0.7,0.72, 0.8,0.56,0.24,0.52,[0.3,0.68],"a diaspora community's size or cross-border activity changes materially"),
  ["bridge node/edge creation","cross-subtree linkage","diffusion-path modeling"],
  ["single-parent containment (bridges are extra edges, not re-parenting)","decay"],
  [pro("Bridge edges capture real cross-border attention diffusion the strict hierarchy cannot","a Punjabi diaspora bridge carries attention from Ludhiana to Brampton and back")],
  [con("Bridges can re-introduce cycles and weaken the finite-hierarchy guarantee if added carelessly","a bridge that re-parents a region breaks single-parent containment")],
  ["a bridge that creates a containment cycle","over-broad bridges that merge unrelated regions"],
  ["bridges are added as non-containment edges only and never re-parent a region","each bridge cites an identifiable cross-border community"],
  ["a diaspora community's cross-border salience shifts","a bridge is found to merge unrelated regions"],
  specialists=["migration_scholar","attention_topologist"],
  subfields=["diffusion_of_innovations","migration_studies"],
  contradictors=["strict_hierarchy_purist"]))

N.append(node("CENTRAL_PLACE_STRUCTURE","christaller_hierarchy_grounding",
  "Ground the region hierarchy's expected attention structure in central place theory (Christaller): higher-order central places serve and out-attract their lower-order hinterlands, giving a prior on parent-vs-child salience that the empirical web can be checked against.",
  "structure", ["REGION_TAXONOMY"], ["CQ_05"],
  b(0.78,0.72,0.66,0.66,0.74,0.78,0.52,0.62,0.5, 0.72,0.74, 0.78,0.62,0.22,0.46,[0.26,0.6],"the central-place prior conflicts repeatedly with observed salience"),
  ["central-place ordering prior","parent>child salience expectation","hinterland grouping"],
  ["empirical scoring (this is a prior to check against)","traversal"],
  [pro("Christaller's hierarchy gives a theory-based prior on which places should dominate their hinterland","a regional capital is expected to out-attract its surrounding towns")],
  [con("Central place theory assumes economic-service gravity that attention can violate in the social-media era","a small town goes viral and out-attracts its regional capital")],
  ["forcing the Christaller prior over contradicting data hides genuine viral inversions","treating the prior as ground truth instead of a checkable expectation"],
  ["the central-place prior is recorded as an expectation and deviations from observed salience are flagged, not overwritten"],
  ["observed salience repeatedly inverts the central-place prior","the hinterland grouping no longer matches administrative reality"],
  specialists=["economic_geographer","attention_topologist"],
  subfields=["central_place_theory","economic_geography"]))

N.append(node("DIFFUSION_DYNAMICS","attention_spread_modeling",
  "Model how attention spreads through the web over time using diffusion-of-innovations dynamics (Rogers 2003): innovators/early-adopter hubs ignite, attention propagates along bridges and high-degree links following an S-curve, informing which neighbors will become salient next.",
  "temporal", ["DIASPORA_BRIDGES","ATTENTION_HALF_LIFE"], ["CQ_07","CQ_11"],
  b(0.78,0.72,0.7,0.74,0.78,0.82,0.55,0.54,0.58, 0.68,0.7, 0.78,0.54,0.26,0.55,[0.32,0.72],"the diffusion model parameters or adopter-category mapping change"),
  ["S-curve adoption fit","propagation-along-links model","next-salient prediction"],
  ["static rank-size structure","bound enforcement"],
  [pro("Diffusion dynamics predict which low-salience neighbor is about to rise, guiding pre-emptive re-pull","a hub's spike forecasts its bridged diaspora region rising next, per Rogers 2003")],
  [con("Diffusion forecasts are noisy and often wrong at the individual-place level","an expected cascade fizzles because the bridge community is dormant")],
  ["overfitting an S-curve to a short window mis-forecasts the cascade","ignoring the late-majority plateau misreads saturation as decline"],
  ["diffusion forecasts are scored against subsequent observed salience and recalibrated when they miss"],
  ["forecast accuracy degrades","the adopter-category mapping no longer fits observed propagation"],
  specialists=["diffusion_modeler","attention_topologist"],
  subfields=["diffusion_of_innovations","time_series_analysis"]))

N.append(node("STALENESS_REVISIT","decay_triggered_repull",
  "Schedule re-pulls of salience signals driven by the estimated half-life: when a place's signal age exceeds a decay threshold, re-acquire and re-score it, so the map tracks attention as it moves rather than freezing at first capture.",
  "temporal", ["ATTENTION_HALF_LIFE","SIGNAL_SOURCING"], ["CQ_12"],
  b(0.82,0.76,0.74,0.62,0.8,0.8,0.55,0.62,0.52, 0.76,0.78, 0.82,0.62,0.22,0.5,[0.24,0.56],"the decay threshold or re-pull scheduling policy changes"),
  ["age-vs-half-life threshold","re-pull scheduling","incremental re-scoring"],
  ["initial sourcing","reproducibility snapshotting"],
  [pro("Half-life-driven re-pulls keep the map current without re-pulling everything every run","a 9-day half-life place is re-pulled biweekly; a stable place yearly")],
  [con("Aggressive re-pulling burns API budget; lazy re-pulling lets the map drift stale","re-pulling every place daily exhausts rate limits")],
  ["a fixed re-pull interval ignores per-place half-life","re-pull thrashing on a noisy signal"],
  ["any place whose signal age exceeds its decay threshold is queued for re-pull before the map is served as current"],
  ["the decay threshold policy changes","stale places are observed being served as current"],
  specialists=["data_engineer","attention_topologist"],
  subfields=["data_engineering","attention_economy"]))

N.append(node("LONG_TAIL_CUTOFF","tail_inclusion_policy",
  "Define the cutoff that separates the salient head from the long tail (Anderson 2006): a salience or rank threshold below which places are excluded from the web, keeping the map finite while documenting what the tail omits.",
  "control", ["RANK_SIZE_DISTRIBUTION"], ["CQ_06","CQ_10"],
  b(0.78,0.72,0.68,0.6,0.76,0.74,0.55,0.64,0.5, 0.76,0.78, 0.78,0.64,0.2,0.46,[0.24,0.56],"the head/tail cutoff threshold or its rationale changes"),
  ["head/tail threshold","tail-omission documentation","cutoff sensitivity check"],
  ["per-node fan-out (local cutoff)","scoring"],
  [pro("An explicit head/tail cutoff bounds the map and makes omissions transparent","include places above the 95th salience percentile; document the omitted tail count")],
  [con("A cutoff hides the aggregate weight of the long tail, which can rival the head (Anderson 2006)","thousands of tiny places together hold attention the head-only map ignores")],
  ["a cutoff that drops a place which later goes viral","an undocumented cutoff makes omissions invisible"],
  ["the cutoff threshold and the count/weight of omitted tail places are documented and reproducible"],
  ["the cutoff threshold changes","aggregate tail weight is found to rival the head"],
  specialists=["product_analyst","attention_topologist"],
  subfields=["long_tail_economics","attention_economy"]))

N.append(node("REPRODUCIBILITY","deterministic_map_rebuild",
  "Guarantee that the same inputs (signal snapshot, weights, bounds, seed) deterministically produce the same attention web: pin signal snapshots, fix random tie-breaks, version all parameters, so any run is byte-reproducible from its manifest.",
  "verification", ["EXPANSION_FRONTIER","TOPK_FANOUT","STALENESS_REVISIT"], ["CQ_12","CQ_13"],
  b(0.9,0.82,0.78,0.66,0.86,0.82,0.66,0.66,0.5, 0.8,0.82, 0.9,0.66,0.18,0.55,[0.2,0.5],"the manifest schema or the set of pinned inputs changes"),
  ["signal-snapshot pinning","parameter versioning","deterministic tie-breaks","run manifest"],
  ["live re-pull (snapshotted for reproducibility)","downstream analysis"],
  [pro("A pinned manifest makes the whole web byte-reproducible and auditable","re-running from manifest v3 yields an identical web months later")],
  [con("Strict pinning fights freshness: a reproducible snapshot is by definition not live","the reproducible map is a frozen past, not today's attention")],
  ["unpinned signals make the web irreproducible","a hidden time/random dependency breaks determinism"],
  ["the same manifest produces a byte-identical web on re-run","every parameter and signal snapshot is versioned in the manifest"],
  ["the manifest schema changes","a non-deterministic dependency is discovered in a run"],
  specialists=["reproducibility_engineer","attention_topologist"],
  subfields=["reproducible_research","data_engineering"]))

N.append(node("COVERAGE_VALIDATION","web_completeness_and_finiteness_check",
  "Validate the finished attention web: it is finite, every node is reachable from the seed under the bounds, salience order is respected, the head/tail cutoff and omissions are documented, and the map carries its validity window. This closes the method against the stated purpose.",
  "verification", ["REPRODUCIBILITY","LONG_TAIL_CUTOFF","COVERAGE_VALIDATION_DEP_PLACEHOLDER"], ["CQ_13"],
  b(0.92,0.84,0.8,0.66,0.88,0.84,0.66,0.64,0.5, 0.8,0.82, 0.92,0.64,0.18,0.56,[0.22,0.54],"the completeness/validity acceptance definition changes"),
  ["finiteness check","seed-reachability check","order-respect check","validity-window attachment"],
  ["live serving","downstream consumption"],
  [pro("A single completeness gate certifies the web is finite, ordered, reproducible, and time-bounded","the gate confirms 482 nodes, all seed-reachable, salience-ordered, valid 14 days")],
  [con("Completeness over the modeled universe can still miss attention the signals never captured","a region salient only on an unmonitored platform is absent yet the web passes")],
  ["passing completeness on a biased signal set certifies a biased map","skipping the validity-window check serves a stale web as current"],
  ["the web passes finiteness, seed-reachability, salience-order, cutoff-documentation, and validity-window checks before it is served"],
  ["the acceptance definition changes","a passing web is later found to omit a known-salient region"],
  specialists=["qa_engineer","attention_topologist"],
  subfields=["validation_and_qa","computational_social_science"]))

# DIFFUSION_BIAS_LINK: an extra structural node tying diffusion to bias (kept distinct).
# (We instead used 20 nodes above; fix the placeholder dependency before dumping.)

# Repair the placeholder dependency on COVERAGE_VALIDATION (it must reference real nodes only).
for _n in N:
    if _n["id"] == "COVERAGE_VALIDATION":
        _n["dependencies"] = ["REPRODUCIBILITY", "LONG_TAIL_CUTOFF"]

CQ = [
 ("CQ_01","How is the finite region hierarchy defined and kept enumerable with single-parent containment?",["nodes","glossary"],"REGION_TAXONOMY defines the closed country>state>city hierarchy with market tiers",["REGION_TAXONOMY"]),
 ("CQ_02","What public signals feed the salience metric and how are they acquired with provenance?",["nodes"],"SALIENCE_METRIC defines the weighted signal aggregate; SIGNAL_SOURCING acquires and stamps each datum",["SALIENCE_METRIC","SIGNAL_SOURCING"]),
 ("CQ_03","How is salience quantified and normalized so unequal-size regions are comparable?",["nodes"],"SALIENCE_METRIC scores attention; SALIENCE_NORMALIZATION makes scores comparable across levels",["SALIENCE_METRIC","SALIENCE_NORMALIZATION"]),
 ("CQ_04","How is platform-skewed visibility distinguished from true regional salience?",["nodes","conflict_axes"],"SAMPLING_BIAS profiles per-platform skew and re-weights signals",["SAMPLING_BIAS"]),
 ("CQ_05","What is the expected attention structure across the hierarchy and its theoretical grounding?",["nodes"],"RANK_SIZE_DISTRIBUTION fits Zipf concentration; CENTRAL_PLACE_STRUCTURE supplies the Christaller prior",["RANK_SIZE_DISTRIBUTION","CENTRAL_PLACE_STRUCTURE"]),
 ("CQ_06","How concentrated is attention and what link topology and tail cutoff describe it?",["nodes","edges"],"RANK_SIZE_DISTRIBUTION and SCALE_FREE_LINKAGE characterize concentration; LONG_TAIL_CUTOFF sets the head/tail boundary",["RANK_SIZE_DISTRIBUTION","SCALE_FREE_LINKAGE","LONG_TAIL_CUTOFF"]),
 ("CQ_07","How does salience decay over time and how is the map's validity window derived?",["nodes","iteration_protocol"],"ATTENTION_HALF_LIFE estimates decay; DIFFUSION_DYNAMICS models spread",["ATTENTION_HALF_LIFE","DIFFUSION_DYNAMICS"]),
 ("CQ_08","How is the seed region chosen and the traversal anchored to it?",["nodes","workflow"],"SEED_SELECTION justifies the start region; EXPANSION_FRONTIER anchors the traversal",["SEED_SELECTION","EXPANSION_FRONTIER"]),
 ("CQ_09","How does the traversal expand regions in decreasing salience order and stay bounded?",["nodes","workflow"],"EXPANSION_FRONTIER expands in decreasing-salience order; DEPTH_BREADTH_BOUNDS bounds it",["EXPANSION_FRONTIER","DEPTH_BREADTH_BOUNDS"]),
 ("CQ_10","How are frontier size and per-node fan-out capped so the web is finite and terminates?",["nodes"],"DEPTH_BREADTH_BOUNDS caps frontier/budget; TOPK_FANOUT and LONG_TAIL_CUTOFF bound fan-out and tail",["DEPTH_BREADTH_BOUNDS","TOPK_FANOUT","LONG_TAIL_CUTOFF"]),
 ("CQ_11","How are cross-border diaspora communities modeled as bridges and how does attention diffuse across them?",["nodes","edges"],"DIASPORA_BRIDGES adds cross-subtree bridge edges; DIFFUSION_DYNAMICS models spread along them",["DIASPORA_BRIDGES","DIFFUSION_DYNAMICS"]),
 ("CQ_12","How is the map kept fresh against decay through scheduled re-pulls?",["nodes","workflow"],"STALENESS_REVISIT schedules half-life-driven re-pulls feeding REPRODUCIBILITY snapshots",["STALENESS_REVISIT","REPRODUCIBILITY"]),
 ("CQ_13","How is reproducibility and overall web completeness/finiteness validated?",["nodes","workflow"],"REPRODUCIBILITY pins inputs for deterministic rebuilds; COVERAGE_VALIDATION certifies finiteness and order",["REPRODUCIBILITY","COVERAGE_VALIDATION"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

# consolidate node->CQ references onto the 13-CQ set (each node refs 1-2)
CQ_MAP = {
 "REGION_TAXONOMY":["CQ_01"], "SALIENCE_METRIC":["CQ_02","CQ_03"], "SALIENCE_NORMALIZATION":["CQ_03"],
 "SIGNAL_SOURCING":["CQ_02"], "ENTITY_DISAMBIGUATION":["CQ_02","CQ_04"], "SAMPLING_BIAS":["CQ_04"], "RANK_SIZE_DISTRIBUTION":["CQ_05","CQ_06"],
 "SCALE_FREE_LINKAGE":["CQ_06"], "ATTENTION_HALF_LIFE":["CQ_07"], "SEED_SELECTION":["CQ_08"],
 "EXPANSION_FRONTIER":["CQ_08","CQ_09"], "DEPTH_BREADTH_BOUNDS":["CQ_09","CQ_10"], "TOPK_FANOUT":["CQ_10"],
 "DIASPORA_BRIDGES":["CQ_11"], "CENTRAL_PLACE_STRUCTURE":["CQ_05"], "DIFFUSION_DYNAMICS":["CQ_07","CQ_11"],
 "STALENESS_REVISIT":["CQ_12"], "LONG_TAIL_CUTOFF":["CQ_06","CQ_10"], "REPRODUCIBILITY":["CQ_12","CQ_13"],
 "COVERAGE_VALIDATION":["CQ_13"],
}
# NB: 19 entries listed; node set has 20 (COVERAGE_VALIDATION counted once). Ensure all nodes covered.
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

GL = [
 ("salience","a scalar attention score for a place/entity aggregated from weighted public signals",["attention_score","visibility_score"],["true_latent_attention"],["SALIENCE_METRIC","SALIENCE_NORMALIZATION"]),
 ("region_hierarchy","the finite closed tree of country>state>city units with single-parent containment",["place_hierarchy","administrative_nesting"],["flat_place_list"],["REGION_TAXONOMY","CENTRAL_PLACE_STRUCTURE"]),
 ("rank_size_law","Zipf's regularity that the rank-r place's salience is about s(1)/r^a",["zipf_law","power_law_rank"],["uniform_distribution"],["RANK_SIZE_DISTRIBUTION","SCALE_FREE_LINKAGE"]),
 ("attention_half_life","the time over which a place's salience decays to half its value",["decay_half_life"],["permanent_salience"],["ATTENTION_HALF_LIFE","STALENESS_REVISIT"]),
 ("expansion_frontier","the salience-keyed priority queue of unvisited nodes awaiting expansion",["traversal_frontier","priority_queue"],["visited_set"],["EXPANSION_FRONTIER","DEPTH_BREADTH_BOUNDS"]),
 ("top_k_fanout","the cap on how many highest-salience neighbors a node enqueues",["branching_cap","neighbor_cutoff"],["unbounded_fanout"],["TOPK_FANOUT","DEPTH_BREADTH_BOUNDS"]),
 ("diaspora_bridge","a cross-border edge linking region subtrees via a salient migrant community",["cross_border_link","bridge_edge"],["containment_edge"],["DIASPORA_BRIDGES","DIFFUSION_DYNAMICS"]),
 ("long_tail","the many low-salience places below the head/tail cutoff whose aggregate weight can rival the head",["heavy_tail","low_salience_mass"],["head_of_distribution"],["LONG_TAIL_CUTOFF","RANK_SIZE_DISTRIBUTION"]),
 ("validity_window","the short time span over which the attention map is declared current",["freshness_window","map_validity"],["permanent_validity"],["ATTENTION_HALF_LIFE","COVERAGE_VALIDATION"]),
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
              "risk_of_conflict":"unmanaged tension degrades the attention web","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated behavior",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies)
dep("REGION_TAXONOMY","SALIENCE_METRIC",0.9)
dep("SALIENCE_METRIC","SALIENCE_NORMALIZATION",0.84)
dep("SALIENCE_METRIC","SIGNAL_SOURCING",0.8)
dep("SIGNAL_SOURCING","ENTITY_DISAMBIGUATION",0.8)
dep("REGION_TAXONOMY","ENTITY_DISAMBIGUATION",0.74)
dep("ENTITY_DISAMBIGUATION","SAMPLING_BIAS",0.8)
dep("SALIENCE_NORMALIZATION","RANK_SIZE_DISTRIBUTION",0.84)
dep("RANK_SIZE_DISTRIBUTION","SCALE_FREE_LINKAGE",0.8)
dep("SALIENCE_METRIC","ATTENTION_HALF_LIFE",0.82)
dep("RANK_SIZE_DISTRIBUTION","SEED_SELECTION",0.78)
dep("SAMPLING_BIAS","SEED_SELECTION",0.74)
dep("SEED_SELECTION","EXPANSION_FRONTIER",0.86)
dep("SCALE_FREE_LINKAGE","EXPANSION_FRONTIER",0.78)
dep("EXPANSION_FRONTIER","DEPTH_BREADTH_BOUNDS",0.86)
dep("DEPTH_BREADTH_BOUNDS","TOPK_FANOUT",0.8)
dep("SCALE_FREE_LINKAGE","DIASPORA_BRIDGES",0.78)
dep("REGION_TAXONOMY","DIASPORA_BRIDGES",0.72)
dep("REGION_TAXONOMY","CENTRAL_PLACE_STRUCTURE",0.76)
dep("DIASPORA_BRIDGES","DIFFUSION_DYNAMICS",0.78)
dep("ATTENTION_HALF_LIFE","DIFFUSION_DYNAMICS",0.76)
dep("ATTENTION_HALF_LIFE","STALENESS_REVISIT",0.82)
dep("SIGNAL_SOURCING","STALENESS_REVISIT",0.76)
dep("RANK_SIZE_DISTRIBUTION","LONG_TAIL_CUTOFF",0.78)
dep("EXPANSION_FRONTIER","REPRODUCIBILITY",0.8)
dep("TOPK_FANOUT","REPRODUCIBILITY",0.76)
dep("STALENESS_REVISIT","REPRODUCIBILITY",0.74)
dep("REPRODUCIBILITY","COVERAGE_VALIDATION",0.86)
dep("LONG_TAIL_CUTOFF","COVERAGE_VALIDATION",0.76)

# cross-cutting non-dependency edges (cross freely; reach the edge count without cycle risk)
rel("STALENESS_REVISIT","SALIENCE_METRIC","feedback",0.78,why="re-pulled signals feed back to recompute the salience metric for aged places")
rel("DIFFUSION_DYNAMICS","STALENESS_REVISIT","causal",0.74,why="a forecast cascade triggers a pre-emptive re-pull of the predicted-rising neighbor")
rel("CENTRAL_PLACE_STRUCTURE","EXPANSION_FRONTIER","constraint",0.72,why="the central-place prior constrains expected expansion order and flags inversions")
rel("SAMPLING_BIAS","SALIENCE_NORMALIZATION","constraint",0.72,why="platform skew corrections enter as adjustments before normalization compares regions")
rel("DIASPORA_BRIDGES","EXPANSION_FRONTIER","causal",0.72,why="bridge edges add cross-subtree neighbors the frontier may expand out of strict containment order")

# conflict edges (negative signed_tension + resolution_rule) -- 4 axes of genuine tension
conf("SAMPLING_BIAS","SALIENCE_METRIC",0.7,-0.55,
  "treat bias correction as a documented adjustment layer applied before aggregation: the raw metric stays auditable while the corrected score drives ranking, so neither raw visibility nor an unverifiable correction silently wins",
  "raw public signals measure platform visibility while bias correction claims true salience; the two pull the score in opposite directions")
conf("EXPANSION_FRONTIER","DEPTH_BREADTH_BOUNDS",0.7,-0.6,
  "expand in strict decreasing-salience order but stop at the bound; when the bound would strand a high-betweenness bridge, raise a documented defeater rather than silently dropping it or removing the bound",
  "greedy decreasing-salience traversal wants to keep expanding while the finite-frontier bound forces termination, risking stranded structurally-important nodes")
conf("STALENESS_REVISIT","REPRODUCIBILITY",0.68,-0.5,
  "serve from a pinned snapshot for reproducibility and attach its validity window; trigger a re-pull and a new versioned snapshot when age exceeds the decay threshold, so freshness produces a new reproducible manifest rather than mutating an old one",
  "decay-driven re-pulls keep the map fresh while reproducibility requires pinning a frozen snapshot; freshness and determinism conflict")
conf("LONG_TAIL_CUTOFF","COVERAGE_VALIDATION",0.66,-0.45,
  "keep the head/tail cutoff to stay finite but require COVERAGE_VALIDATION to document the count and aggregate weight of omitted tail places, so finiteness never silently hides material long-tail attention",
  "the head/tail cutoff trims the long tail to stay finite while completeness validation wants full coverage; per Anderson 2006 the omitted tail can hold material attention")

CA=[
 {"name":"platform_visibility_vs_true_salience","description":"Public signals measure what a platform surfaces; correcting for skew estimates true regional attention but needs an uncertain demographic baseline.","poles":["raw_platform_visibility","bias_corrected_true_salience"],"resolution_hint":"keep the raw metric auditable; apply correction as a documented adjustment layer","tension_score":0.7,"affected_nodes":["SAMPLING_BIAS","SALIENCE_METRIC","SALIENCE_NORMALIZATION"]},
 {"name":"greedy_salience_order_vs_bounded_frontier","description":"Decreasing-salience traversal wants to keep going; the finite frontier bound forces termination and can strand structurally important low-salience bridges.","poles":["greedy_decreasing_order","finite_bounded_frontier"],"resolution_hint":"raise a documented defeater when a bound would strand a high-betweenness bridge","tension_score":0.72,"affected_nodes":["EXPANSION_FRONTIER","DEPTH_BREADTH_BOUNDS","SCALE_FREE_LINKAGE"]},
 {"name":"freshness_vs_reproducibility","description":"Decay-driven re-pulls keep the map current while reproducibility requires a pinned frozen snapshot.","poles":["live_freshness","pinned_reproducible_snapshot"],"resolution_hint":"re-pull into a new versioned manifest rather than mutating an old snapshot","tension_score":0.68,"affected_nodes":["STALENESS_REVISIT","REPRODUCIBILITY","ATTENTION_HALF_LIFE"]},
 {"name":"head_cutoff_vs_long_tail_coverage","description":"A head/tail cutoff keeps the web finite but the omitted long tail can hold attention rivaling the head (Anderson 2006).","poles":["tight_head_cutoff","full_tail_coverage"],"resolution_hint":"document omitted-tail count and aggregate weight at validation","tension_score":0.66,"affected_nodes":["LONG_TAIL_CUTOFF","COVERAGE_VALIDATION","RANK_SIZE_DISTRIBUTION"]},
 {"name":"per_capita_vs_mass_attention","description":"Per-capita normalization lets small intense places compete but can erase the genuine mass-attention advantage of large hubs.","poles":["per_capita_normalized","absolute_mass_attention"],"resolution_hint":"report both normalized and absolute scores; rank on the goal-aligned one","tension_score":0.62,"affected_nodes":["SALIENCE_NORMALIZATION","SALIENCE_METRIC","RANK_SIZE_DISTRIBUTION"]},
 {"name":"strict_hierarchy_vs_diaspora_bridges","description":"Single-parent containment keeps the universe finite and acyclic; diaspora bridges capture real cross-border attention but risk re-introducing cycles.","poles":["strict_single_parent","cross_border_bridges"],"resolution_hint":"add bridges as non-containment edges only; never re-parent a region","tension_score":0.64,"affected_nodes":["REGION_TAXONOMY","DIASPORA_BRIDGES","SCALE_FREE_LINKAGE"]},
 {"name":"uniform_topk_vs_hub_heterogeneity","description":"A uniform top_k bounds fan-out but ignores that true scale-free hubs legitimately have more salient neighbors than leaves.","poles":["uniform_topk","hub_aware_fanout"],"resolution_hint":"allow a higher k for nodes whose degree exceeds a hub threshold","tension_score":0.6,"affected_nodes":["TOPK_FANOUT","SCALE_FREE_LINKAGE","DEPTH_BREADTH_BOUNDS"]},
 {"name":"central_place_prior_vs_observed_virality","description":"Christaller's hierarchy predicts higher-order places dominate hinterlands, but social-media virality can invert parent-vs-child salience.","poles":["central_place_prior","observed_empirical_salience"],"resolution_hint":"treat the prior as a checkable expectation; flag and keep observed inversions","tension_score":0.6,"affected_nodes":["CENTRAL_PLACE_STRUCTURE","RANK_SIZE_DISTRIBUTION","SALIENCE_METRIC"]},
 {"name":"power_law_fit_vs_lognormal_reality","description":"Fitting Zipf/power-law quantifies concentration but many attention distributions are lognormal, so a forced power-law misstates the tail.","poles":["power_law_fit","lognormal_or_empirical_fit"],"resolution_hint":"run a goodness-of-fit and report the better-supported model with its interval","tension_score":0.58,"affected_nodes":["RANK_SIZE_DISTRIBUTION","SCALE_FREE_LINKAGE","LONG_TAIL_CUTOFF"]},
]

EC=[
 {"description":"A diaspora bridge re-parents a region, creating a containment cycle that breaks the finite-hierarchy guarantee.","trigger":"a bridge edge is added as a containment edge instead of a non-containment cross-link","affected_nodes":["DIASPORA_BRIDGES","REGION_TAXONOMY","COVERAGE_VALIDATION"],"mitigation":"restrict bridges to non-containment edges; assert single-parent containment after every bridge insertion","severity":"critical"},
 {"description":"Greedy decreasing-salience traversal hits the frontier bound and silently strands a high-betweenness bridge, fragmenting the web.","trigger":"the node budget is reached before a structurally important low-salience bridge is expanded","affected_nodes":["EXPANSION_FRONTIER","DEPTH_BREADTH_BOUNDS","SCALE_FREE_LINKAGE"],"mitigation":"raise a documented defeater when a bound would strand a bridge above a betweenness threshold","severity":"high"},
 {"description":"A signal source rate-limits or rescales mid-pull, silently truncating coverage for a region level.","trigger":"Google Trends caps the query window or a follower API paginates incompletely","affected_nodes":["SIGNAL_SOURCING","SAMPLING_BIAS","COVERAGE_VALIDATION"],"mitigation":"detect truncation, flag coverage gaps explicitly, and never zero-fill missing signals","severity":"high"},
 {"description":"Platform skew is uncorrected, so an already-visible urban English-language place is inflated over a genuinely salient under-covered region.","trigger":"signals aggregated without a per-platform skew correction","affected_nodes":["SAMPLING_BIAS","SALIENCE_METRIC","SALIENCE_NORMALIZATION"],"mitigation":"apply documented per-platform correction factors before aggregation","severity":"high"},
 {"description":"The map is served as current after its validity window has elapsed, presenting decayed salience as live.","trigger":"signal age exceeds the half-life-derived decay threshold but no re-pull was triggered","affected_nodes":["ATTENTION_HALF_LIFE","STALENESS_REVISIT","COVERAGE_VALIDATION"],"mitigation":"gate serving on the validity window; queue aged places for re-pull before serving","severity":"high"},
 {"description":"A power-law is fit to a level whose salience is actually lognormal, overstating concentration and mis-setting the tail cutoff.","trigger":"rank-size fit applied without a goodness-of-fit test","affected_nodes":["RANK_SIZE_DISTRIBUTION","LONG_TAIL_CUTOFF","SCALE_FREE_LINKAGE"],"mitigation":"run a goodness-of-fit; report the better-supported model and a confidence interval on the exponent","severity":"medium"},
 {"description":"A uniform top_k truncates a true global hub's legitimate salient neighbors, deleting real structure.","trigger":"a fixed k applied to a node whose degree far exceeds the hub threshold","affected_nodes":["TOPK_FANOUT","SCALE_FREE_LINKAGE","DEPTH_BREADTH_BOUNDS"],"mitigation":"allow a higher k for nodes above a degree/hub threshold while keeping a global budget","severity":"medium"},
 {"description":"The seed region biases the whole web; a different seed would yield a materially different attention map.","trigger":"seed chosen without justification against salience and the analysis goal","affected_nodes":["SEED_SELECTION","EXPANSION_FRONTIER","RANK_SIZE_DISTRIBUTION"],"mitigation":"justify the seed against salience and goal; document seed-sensitivity of the resulting web","severity":"medium"},
 {"description":"A diffusion forecast predicts a cascade that fizzles, triggering wasted re-pulls of dormant regions.","trigger":"an S-curve overfit to a short window mis-forecasts propagation along a dormant bridge","affected_nodes":["DIFFUSION_DYNAMICS","STALENESS_REVISIT","DIASPORA_BRIDGES"],"mitigation":"score forecasts against subsequent observed salience and recalibrate; cap forecast-driven re-pulls","severity":"medium"},
 {"description":"Per-capita normalization erases a megacity's genuine mass-attention advantage, inverting an expected rank.","trigger":"ranking on normalized score alone for a goal that cares about absolute reach","affected_nodes":["SALIENCE_NORMALIZATION","SALIENCE_METRIC","RANK_SIZE_DISTRIBUTION"],"mitigation":"report both normalized and absolute scores and rank on the goal-aligned measure","severity":"low"},
 {"description":"The head/tail cutoff drops a place that goes viral days later, and the omission was undocumented.","trigger":"an undocumented cutoff threshold excludes a sub-threshold place","affected_nodes":["LONG_TAIL_CUTOFF","COVERAGE_VALIDATION","DIFFUSION_DYNAMICS"],"mitigation":"document the cutoff and omitted-tail weight; monitor near-cutoff places for diffusion-predicted rises","severity":"low"},
 {"description":"A run is irreproducible because a signal snapshot was not pinned and a random tie-break varied.","trigger":"live signals and unfixed tie-breaks used instead of a pinned, versioned manifest","affected_nodes":["REPRODUCIBILITY","EXPANSION_FRONTIER","SIGNAL_SOURCING"],"mitigation":"pin signal snapshots, fix tie-break seeds, version all parameters in a run manifest","severity":"high"},
]

WF=[
 {"action":"define_region_hierarchy","node_ref":"REGION_TAXONOMY","description":"Enumerate the finite country>state>city hierarchy with market tiers and assert single-parent containment.","artifact":"region_hierarchy","gate":"every place has exactly one parent and the node set is finite"},
 {"action":"define_salience_metric","node_ref":"SALIENCE_METRIC","description":"Specify the weighted salience formula over named public signals with documented units and weights.","artifact":"salience_metric_spec","gate":"salience is computable and reproducible from named public sources"},
 {"action":"acquire_signals","node_ref":"SIGNAL_SOURCING","description":"Pull raw public signals with source id, timestamp, and access method for every region at the seed level.","artifact":"signal_snapshot","gate":"each datum is provenance-stamped and re-pullable"},
 {"action":"correct_platform_bias","node_ref":"SAMPLING_BIAS","description":"Attach per-platform skew profiles and correction factors before aggregation; flag coverage gaps.","artifact":"bias_corrected_signals","gate":"each platform signal carries a documented correction factor"},
 {"action":"normalize_salience","node_ref":"SALIENCE_NORMALIZATION","description":"Normalize scores across unequal-size regions (per-capita / z-score) so levels are comparable.","artifact":"normalized_salience","gate":"equal per-capita attention yields equal normalized scores"},
 {"action":"fit_rank_size","node_ref":"RANK_SIZE_DISTRIBUTION","description":"Fit the Zipf rank-size distribution with a goodness-of-fit and report the exponent with an interval.","artifact":"rank_size_fit","gate":"the fit reproduces and passes the tail goodness-of-fit threshold"},
 {"action":"choose_seed","node_ref":"SEED_SELECTION","description":"Select and justify the seed region against salience and the analysis goal; confirm signal coverage.","artifact":"seed_record","gate":"seed is justified and has full signal coverage at its level"},
 {"action":"traverse_decreasing_order","node_ref":"EXPANSION_FRONTIER","description":"Expand the salience-keyed frontier in strictly non-increasing order with deterministic tie-breaking.","artifact":"traversal_order","gate":"expansion order is non-increasing and reproducible"},
 {"action":"bound_frontier","node_ref":"DEPTH_BREADTH_BOUNDS","description":"Apply frontier cap, per-node top_k, node budget and max depth so traversal terminates finitely.","artifact":"bounded_web","gate":"traversal terminates within budget; cap breaches raise a defeater"},
 {"action":"add_diaspora_bridges","node_ref":"DIASPORA_BRIDGES","description":"Insert cross-border bridge edges for salient diaspora communities as non-containment links only.","artifact":"bridged_web","gate":"no bridge re-parents a region; containment stays acyclic"},
 {"action":"schedule_repulls","node_ref":"STALENESS_REVISIT","description":"Queue half-life-driven re-pulls for places whose signal age exceeds the decay threshold.","artifact":"repull_schedule","gate":"every over-aged place is queued before the map is served as current"},
 {"action":"validate_and_pin","node_ref":"COVERAGE_VALIDATION","description":"Pin a reproducible manifest and certify finiteness, seed-reachability, salience-order, cutoff documentation, and validity window.","artifact":"validated_attention_web","gate":"all completeness, reproducibility, and validity-window checks pass"},
]

DR=[
 {"rule":"REGION_TAXONOMY must define a finite single-parent hierarchy before SALIENCE_METRIC scores any place","rationale":"scoring an unbounded or multi-parent place set makes the web infinite or cyclic","trigger":"salience scoring begins without a closed hierarchy","action":"block scoring until containment and finiteness are asserted"},
 {"rule":"SAMPLING_BIAS correction must be applied before SALIENCE_NORMALIZATION compares regions","rationale":"normalizing uncorrected platform-skewed signals propagates visibility bias into every rank","trigger":"normalization runs on raw un-corrected signals","action":"require documented per-platform correction factors first"},
 {"rule":"EXPANSION_FRONTIER must expand in strictly non-increasing salience order with deterministic tie-breaking","rationale":"non-decreasing or non-deterministic order makes the web irreproducible and misorders attention","trigger":"a lower-salience node is expanded before a higher-salience one, or ties break randomly","action":"reject the expansion and restore decreasing-order popping"},
 {"rule":"DEPTH_BREADTH_BOUNDS caps must be set and active before traversal starts","rationale":"an unbounded frontier lets the long tail explode the node set and traversal may not terminate","trigger":"traversal begins with no node budget, top_k, or max depth","action":"require explicit caps before the first expansion"},
 {"rule":"DIASPORA_BRIDGES may only add non-containment edges and must never re-parent a region","rationale":"a containment-altering bridge re-introduces cycles and breaks the finite-hierarchy guarantee","trigger":"a bridge edge would change a region's parent","action":"reject the bridge and raise a containment defeater"},
 {"rule":"STALENESS_REVISIT must queue any place past its decay threshold before the map is served as current","rationale":"serving decayed salience as live presents a confidently wrong stale map","trigger":"the map is served with a place whose age exceeds its validity window","action":"block serving and queue the over-aged place for re-pull"},
 {"rule":"RANK_SIZE_DISTRIBUTION must pass a goodness-of-fit before its exponent drives LONG_TAIL_CUTOFF","rationale":"a forced power-law on lognormal data mis-sets the head/tail cutoff","trigger":"the cutoff is derived from an unvalidated power-law fit","action":"require a goodness-of-fit test and report the better-supported model"},
 {"rule":"REPRODUCIBILITY must pin the signal snapshot and parameters before COVERAGE_VALIDATION certifies the web","rationale":"an unpinned run cannot be reproduced or audited, voiding the validation","trigger":"validation requested on a run with live or unversioned inputs","action":"require a pinned versioned manifest before certifying"},
 {"rule":"COVERAGE_VALIDATION must document omitted-tail count and weight before the web is accepted","rationale":"a silent cutoff can hide long-tail attention that rivals the head","trigger":"acceptance attempted without tail-omission documentation","action":"fail acceptance until the omitted tail is documented"},
]

ARR=[
 {"rule":"Do not score salience before the region hierarchy is finite and single-parent; re-scoring after a hierarchy change reworks the whole web","prevents":"full re-score after a late hierarchy redefinition"},
 {"rule":"Do not normalize or rank on uncorrected platform-skewed signals; un-baking bias from finished ranks requires re-scoring everything","prevents":"re-ranking the entire web after discovering a platform tilt"},
 {"rule":"Do not start traversal without active depth/breadth bounds; adding bounds after an unbounded blow-up means discarding the runaway web","prevents":"discarding an exploded long-tail traversal and restarting"},
 {"rule":"Do not add diaspora bridges as containment edges; untangling a cycle later requires a graph surgery and re-traversal","prevents":"re-traversing after a containment cycle is found"},
 {"rule":"Do not serve a map without its validity window; retrofitting decay tracking after serving stale data means re-pulling and re-scoring","prevents":"emergency re-pull after a stale map is published"},
 {"rule":"Do not derive the tail cutoff from an unvalidated power-law fit; a wrong exponent forces refitting and re-cutting the tail","prevents":"re-cutting the head/tail after a lognormal fit is found better"},
 {"rule":"Do not run without pinning the signal snapshot and parameters; an irreproducible run must be re-executed from scratch to audit","prevents":"re-running the full pipeline to reproduce an audited result"},
 {"rule":"Do not finalize the seed before its salience and goal-fit are justified; a late seed change re-anchors and re-traverses the whole web","prevents":"complete re-traversal after a seed is changed"},
]

IP=[
 {"trigger":"a containment cycle or multi-parent region is detected after a bridge insertion","action":"reject the offending edge in DIASPORA_BRIDGES and re-assert single-parent containment in REGION_TAXONOMY","nodes":["DIASPORA_BRIDGES","REGION_TAXONOMY"],"priority":"critical"},
 {"trigger":"traversal fails to terminate or the long tail explodes the node set","action":"tighten the caps in DEPTH_BREADTH_BOUNDS and the per-node cutoff in TOPK_FANOUT","nodes":["DEPTH_BREADTH_BOUNDS","TOPK_FANOUT"],"priority":"critical"},
 {"trigger":"a structurally important bridge is stranded below the frontier cutoff","action":"raise the bound defeater in EXPANSION_FRONTIER and re-check hub identification in SCALE_FREE_LINKAGE","nodes":["EXPANSION_FRONTIER","SCALE_FREE_LINKAGE"],"priority":"high"},
 {"trigger":"a stale map was served past its validity window","action":"lower the decay threshold in STALENESS_REVISIT and re-estimate the half-life in ATTENTION_HALF_LIFE","nodes":["STALENESS_REVISIT","ATTENTION_HALF_LIFE"],"priority":"high"},
 {"trigger":"platform-skew bias is found in the final ranks","action":"recompute correction factors in SAMPLING_BIAS and re-normalize in SALIENCE_NORMALIZATION","nodes":["SAMPLING_BIAS","SALIENCE_NORMALIZATION"],"priority":"high"},
 {"trigger":"a goodness-of-fit rejects the power-law for a level","action":"refit in RANK_SIZE_DISTRIBUTION and re-derive the cutoff in LONG_TAIL_CUTOFF","nodes":["RANK_SIZE_DISTRIBUTION","LONG_TAIL_CUTOFF"],"priority":"medium"},
 {"trigger":"a run cannot be reproduced from its manifest","action":"audit pinned inputs and tie-breaks in REPRODUCIBILITY and re-run COVERAGE_VALIDATION","nodes":["REPRODUCIBILITY","COVERAGE_VALIDATION"],"priority":"medium"},
]

spec = {
 "domain":"geo__attention_topology",
 "domain_label":"Geographic & Demographic Attention Topology",
 "purpose":"finite_reproducible_attention_web_over_a_region_hierarchy_with_a_defined_salience_metric_decreasing_order_traversal_and_bounded_frontier",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "attention is approximated by obtainable public signals (search interest, follower counts, news/Wikipedia volume), not by the unobservable latent construct",
   "the region hierarchy is a finite closed tree with single-parent containment so the node universe is enumerable",
   "salience is perishable: the map is valid only over a stated short validity window derived from estimated half-life",
 ],
 "exclusions":[
   "private platform analytics and non-public attention signals",
   "individual-level surveillance or person-tracking (only aggregate place/entity salience)",
   "causal claims that attention salience equals economic or political importance",
   "real-time streaming inference (the method snapshots and re-pulls, it does not stream)",
 ],
 "source_description":"heuristic prior estimates for geographic and demographic attention-topology work units, informed by Zipf rank-size law, Christaller central place theory, Rogers diffusion of innovations, Barabasi-Albert scale-free networks, the Simon/Davenport-Beck attention economy, and Anderson long-tail economics; no supplied dataset",
 "source_citation":"Zipf 1949 Human Behavior and the Principle of Least Effort (rank-size law); Christaller 1933 Central Places in Southern Germany (central place theory); Rogers 2003 Diffusion of Innovations (5th ed.); Barabasi & Albert 1999 'Emergence of Scaling in Random Networks' (Science); Simon 1971 'Designing Organizations for an Information-Rich World' and Davenport & Beck 2001 The Attention Economy; Anderson 2006 The Long Tail; Clauset, Shalizi & Newman 2009 'Power-law distributions in empirical data'",
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
 "priority_rationale":"REGION_TAXONOMY and SALIENCE_METRIC are foundational; sourcing/bias/normalization make scores trustworthy and comparable; rank-size, scale-free and central-place structure characterize concentration; seed, frontier and bounds drive the finite decreasing-order traversal; half-life, diffusion and re-pull keep it current; reproducibility and coverage validation close the method last.",
 "eval_objective":"verify_finite_reproducible_salience_scored_decreasing_order_bounded_traversal_with_decay_and_bias_handling_of_geo__attention_topology_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "geo__attention_topology.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
