#!/usr/bin/env python3
"""Generate the link__entity_relevance content spec (B60 content-intelligence KB)
for kb_forge.py. Compact authoring: node() applies sane defaults so only domain
content + base metric magnitudes are specified per node.

Domain: Entity Linking & Geo-Relevance Combination — license a "scene" of 2-3
related entities via typed-tag joins, relatedness scoring, and a geo-relevance
predicate, with a hard arity (2..3) constraint enforced.

Real literature cited: Fellegi & Sunter 1969 (record linkage theory);
Resnik 1995 (IC-based semantic similarity); Gabrilovich & Markovitch 2007 (ESA
semantic relatedness); knowledge-graph entity linking (Shen, Wang & Han 2015
survey; Hoffart et al. 2011 AIDA); McPherson, Smith-Lovin & Cook 2001 (homophily);
Adamic & Adar 2003 (friends-and-neighbors link prediction)."""
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
        "academic_fields": ["entity_resolution", "information_retrieval"],
        "subfields": subfields or ["entity_linking", "semantic_relatedness", "link_prediction"],
        "specialists": specialists or ["content_intelligence_engineer"],
        "contradictors": contradictors or ["naive_keyword_matcher_advocate"],
        "inputs": inputs or ["entity records", "typed tag sets"],
        "outputs": outputs or ["licensed entity scene"],
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

N.append(node("ENTITY_RESOLUTION", "canonical_entity_id_and_alias_dedup",
  "Resolve every surface mention to one canonical entity id and merge aliases/variants (spellings, transliterations, role titles) so that downstream joins compare entities, not strings. Implements probabilistic record linkage: a match weight is the log-likelihood ratio of agreement on each field given match vs non-match, thresholded into match / possible-match / non-match zones.",
  "foundations", [], ["CQ_01"],
  b(0.92, 0.8, 0.78, 0.62, 0.85, 0.78, 0.66, 0.7, 0.45, 0.78, 0.82, 0.9, 0.7, 0.18, 0.55, [0.2, 0.5],
    "blocking key, comparison vector, or match/non-match thresholds change"),
  ["surface-mention to canonical-id resolution", "alias and transliteration merging", "probabilistic match weighting (Fellegi-Sunter)"],
  ["typed-tag semantics", "relatedness scoring between distinct entities"],
  [pro("A single canonical id per entity makes joins and relatedness compare real entities instead of raw strings, killing duplicate-driven errors", "merging 'Bhagat Singh', 'Shaheed Bhagat Singh' and the Punjabi transliteration onto one id")],
  [con("Over-merging collapses two genuinely distinct entities into one and is hard to reverse downstream", "merging two different people who share a common name into one canonical id")],
  ["over-merge of distinct entities (false-match zone too wide)", "under-merge leaving aliases as separate ids (blocking key too strict)"],
  ["each surface mention maps to exactly one canonical id and no two canonical ids share a known alias set"],
  ["new transliteration/locale added", "duplicate canonical ids observed in production joins"],
  specialists=["entity_resolution_engineer", "knowledge_engineer"],
  subfields=["record_linkage", "entity_disambiguation"]))

N.append(node("MENTION_DISAMBIGUATION", "context_aware_mention_to_kb_linking",
  "Disambiguate an ambiguous surface mention to the correct knowledge-graph node using local context and global coherence (collective linking), so 'Punjab' the Indian state, 'Punjab' the Pakistani province, and 'Punjab' a band each link to distinct nodes. Combines a prior commonness probability with context similarity and entity-coherence, as in AIDA-style collective disambiguation.",
  "foundations", ["ENTITY_RESOLUTION"], ["CQ_01", "CQ_02"],
  b(0.86, 0.74, 0.72, 0.7, 0.84, 0.8, 0.6, 0.64, 0.5, 0.74, 0.78, 0.84, 0.64, 0.22, 0.52, [0.24, 0.58],
    "candidate-generation source or coherence model changes"),
  ["candidate generation for a mention", "context + coherence scoring", "link to canonical KB node"],
  ["alias dedup (upstream)", "geo-relevance predicate (downstream)"],
  [pro("Collective disambiguation using inter-entity coherence resolves mentions a per-mention prior alone gets wrong", "co-occurring 'Amritsar' pushes ambiguous 'Punjab' toward the Indian-state node")],
  [con("Coherence optimization is expensive and can propagate one wrong anchor into several linked errors", "a mis-linked seed entity drags coherent neighbors to the wrong sense")],
  ["popularity prior overrides correct rare sense", "context window too small to disambiguate homographs"],
  ["a held-out mention set links to the gold KB node above the target accuracy threshold"],
  ["KB node set changes", "disambiguation accuracy regresses on the audit set"],
  specialists=["entity_resolution_engineer"],
  subfields=["entity_linking", "wikification"]))

N.append(node("TYPED_TAG_JOIN", "join_on_typed_tags_not_bare_labels",
  "Join entities and news items on TYPED tags, where a tag is a (type, value) pair so that PLACE:punjab is never joined to TOPIC:punjab or ORG:punjab. The type namespace (PLACE / TOPIC / PERSON / ORG / EVENT) is the join key alongside the value, preventing the classic label-collision error where a place name and a topic name share a string.",
  "foundations", ["ENTITY_RESOLUTION"], ["CQ_02", "CQ_03"],
  b(0.9, 0.82, 0.78, 0.6, 0.86, 0.82, 0.62, 0.68, 0.55, 0.76, 0.8, 0.88, 0.68, 0.18, 0.55, [0.2, 0.5],
    "tag type namespace or join-key definition changes"),
  ["(type,value) typed-tag join key", "type-namespace enforcement", "rejecting cross-type label collisions"],
  ["relatedness scoring magnitude", "geo-relevance predicate logic"],
  [pro("Typing the join key eliminates the most common spurious link: a place string colliding with an unrelated topic or org string", "PLACE:punjab vs TOPIC:punjab kept distinct so a wheat-farming topic does not auto-link to a Punjabi musician")],
  [con("A rigid type namespace rejects legitimate cross-type bridges that a richer relation model would keep", "a PLACE and the EVENT named after it are not joinable on bare typed tags")],
  ["untyped join silently links place to topic on a shared string", "type vocabulary too coarse to separate ORG from EVENT"],
  ["no join exists between two tags that share a value but differ in type"],
  ["tag type vocabulary extended", "a cross-type collision link is observed downstream"],
  specialists=["content_intelligence_engineer", "taxonomy_engineer"],
  subfields=["typed_joins", "tag_taxonomy"]))

N.append(node("RELATION_TYPING", "edge_relation_type_assignment",
  "Assign each candidate link a relation type from a closed set: SAME_PLACE, SAME_TOPIC, AFFECTS, CONTRASTS_WITH. The relation type carries the semantics the scene exposes and determines which downstream checks (e.g. directionality for AFFECTS) apply. Relation typing turns an undifferentiated 'related' edge into an interpretable, checkable claim.",
  "structure", ["TYPED_TAG_JOIN"], ["CQ_03", "CQ_04"],
  b(0.84, 0.76, 0.74, 0.64, 0.8, 0.8, 0.58, 0.66, 0.52, 0.74, 0.78, 0.82, 0.66, 0.2, 0.5, [0.24, 0.56],
    "the closed relation-type set is revised"),
  ["SAME_PLACE / SAME_TOPIC / AFFECTS / CONTRASTS_WITH assignment", "closed relation vocabulary", "per-type downstream-check routing"],
  ["relatedness magnitude computation", "arity enforcement"],
  [pro("A closed, typed relation set makes each edge an interpretable claim and lets type-specific rules (directionality, contrast) be enforced", "tagging a Punjab-person <-> US-India-immigration edge AFFECTS rather than a vague 'related'")],
  [con("A closed relation set forces edges that are genuinely a blend into one bucket, losing nuance", "an edge that is both SAME_TOPIC and CONTRASTS_WITH must pick one")],
  ["relation type left null so directionality cannot be checked", "forcing a blended relation into a single wrong type"],
  ["every emitted edge carries exactly one relation type from the closed set"],
  ["relation vocabulary extended", "an edge's assigned type contradicts its directionality check"],
  specialists=["content_intelligence_engineer"],
  subfields=["relation_extraction", "edge_typing"]))

N.append(node("RELATEDNESS_SCORE", "semantic_plus_cooccurrence_relatedness",
  "Compute a relatedness score between two entities combining (a) distributional/semantic relatedness — Explicit Semantic Analysis cosine over concept vectors, or IC-based similarity over a taxonomy — with (b) empirical co-occurrence in a corpus. Relatedness is the continuous magnitude that ranks candidate links; it is symmetric and bounded to [0,1].",
  "scoring", ["TYPED_TAG_JOIN", "MENTION_DISAMBIGUATION"], ["CQ_04", "CQ_05"],
  b(0.88, 0.8, 0.78, 0.74, 0.84, 0.84, 0.6, 0.62, 0.58, 0.7, 0.74, 0.86, 0.62, 0.24, 0.55, [0.28, 0.64],
    "relatedness model (ESA index, taxonomy, or co-occurrence corpus) changes"),
  ["ESA/IC semantic relatedness", "corpus co-occurrence component", "symmetric bounded [0,1] score"],
  ["the licensing decision (threshold lives downstream)", "directionality (relatedness is symmetric)"],
  [pro("Blending distributional relatedness (ESA) with corpus co-occurrence captures both meaning-level and usage-level association, beating either alone", "ESA relates 'diaspora' and 'immigration' even without surface co-occurrence; co-occurrence catches a fresh news pairing ESA misses")],
  [con("Semantic relatedness is not relevance: two entities can be highly related yet irrelevant to license together", "'cricket' and 'India' are very related but that relatedness alone does not justify a news scene")],
  ["high relatedness mistaken for a license to link", "stale ESA index undervalues an emerging association"],
  ["relatedness is symmetric, in [0,1], and ranks a gold related pair above a gold unrelated pair on the audit set"],
  ["ESA/taxonomy index rebuilt", "co-occurrence corpus refreshed and rankings shift materially"],
  specialists=["content_intelligence_engineer", "nlp_engineer"],
  subfields=["semantic_relatedness", "distributional_semantics"]))

N.append(node("COOCCURRENCE_SIGNAL", "corpus_cooccurrence_estimation",
  "Estimate the empirical co-occurrence component of relatedness: how often two entities appear together in the same document/window beyond chance, using pointwise mutual information (PMI) or a normalized variant. Supplies the usage-level evidence that complements model-based semantic relatedness.",
  "scoring", ["MENTION_DISAMBIGUATION"], ["CQ_05"],
  b(0.78, 0.7, 0.66, 0.7, 0.74, 0.74, 0.5, 0.6, 0.5, 0.72, 0.74, 0.78, 0.6, 0.24, 0.48, [0.28, 0.6],
    "co-occurrence corpus, window size, or PMI normalization changes"),
  ["document/window co-occurrence counting", "PMI / normalized PMI estimation", "chance-corrected association"],
  ["semantic model relatedness (separate component)", "thresholding"],
  [pro("Chance-corrected co-occurrence (PMI) surfaces genuine associations and discounts pairs that co-occur only because both are frequent", "PMI keeps a rare-but-tight pairing while discounting two ubiquitous entities")],
  [con("Raw PMI is unstable for low-frequency pairs and over-rewards rare co-occurrences", "two entities co-occurring once get an inflated PMI")],
  ["low-count pairs produce unstable, inflated PMI", "window too wide dilutes real co-occurrence with noise"],
  ["PMI estimates are frequency-smoothed and a rare ubiquitous pair scores below a tight rare pair"],
  ["corpus refreshed", "low-count instability observed in scores"],
  specialists=["nlp_engineer"],
  subfields=["co_occurrence_statistics", "pmi"]))

N.append(node("LINK_PREDICTION_PRIOR", "neighborhood_link_prediction_prior",
  "Provide a structural link-prediction prior from the existing entity graph: the Adamic-Adar score (sum over shared neighbors weighted by inverse log degree) and common-neighbor counts estimate how plausible a new edge is from graph topology alone, independent of text. Used as a prior that nudges candidate generation and tie-breaks relatedness.",
  "scoring", ["ENTITY_RESOLUTION"], ["CQ_05", "CQ_06"],
  b(0.74, 0.68, 0.64, 0.72, 0.72, 0.76, 0.5, 0.58, 0.52, 0.7, 0.72, 0.74, 0.58, 0.26, 0.46, [0.3, 0.64],
    "the underlying entity graph or neighbor-weighting changes"),
  ["Adamic-Adar shared-neighbor score", "common-neighbor topology prior", "structural tie-breaking"],
  ["text relatedness (separate)", "the geo-relevance predicate"],
  [pro("A topology prior (Adamic-Adar) predicts plausible links from shared low-degree neighbors even when text evidence is thin", "two people sharing a rare-neighbor 'Jallianwala Bagh' get a high structural prior")],
  [con("Structural priors entrench existing graph bias and under-predict genuinely novel cross-community links", "a first-of-its-kind connection has no shared neighbors and is scored low")],
  ["sparse graph yields near-zero priors for all new pairs", "high-degree hub neighbors inflate spurious priors"],
  ["Adamic-Adar ranks a held-out true edge above a sampled false edge on the link-prediction audit"],
  ["graph topology shifts materially", "novel-link recall drops below target"],
  specialists=["graph_engineer"],
  subfields=["link_prediction", "graph_topology"]))

N.append(node("HOMOPHILY", "audience_overlap_shareability",
  "Model homophily — the principle that contact and shared interest concentrate among similar nodes ('birds of a feather') — to estimate audience overlap between two entities. High audience overlap makes a link more shareable and engaging; homophily explains why links within an interest community travel further than cross-community links.",
  "scoring", ["LINK_PREDICTION_PRIOR"], ["CQ_06", "CQ_07"],
  b(0.76, 0.78, 0.74, 0.66, 0.72, 0.8, 0.5, 0.6, 0.55, 0.7, 0.72, 0.78, 0.6, 0.24, 0.48, [0.3, 0.62],
    "audience-overlap measurement or homophily weighting changes"),
  ["audience-overlap estimation", "shareability prior from homophily", "within-community engagement lift"],
  ["the relevance predicate (overlap is not relevance)", "directionality"],
  [pro("Homophily-based audience overlap predicts which true links will actually be shared and engaged with, improving scene value", "two cricket entities share a fan audience, so their scene spreads within that community")],
  [con("Optimizing for homophily creates filter bubbles and starves valuable cross-community (bridging) links", "always linking within the diaspora community hides relevant outside-community context")],
  ["over-weighting homophily collapses scenes into one echo community", "audience-overlap proxy stale or biased"],
  ["scenes with high audience overlap show higher measured engagement than low-overlap scenes on the audit cohort"],
  ["audience-overlap proxy data refreshed", "bridging-link share rate falls below target"],
  specialists=["content_intelligence_engineer", "audience_analyst"],
  subfields=["homophily", "social_network_analysis"]))

N.append(node("GEO_RELEVANCE_PREDICATE", "geo_conditioned_link_license_predicate",
  "Define the boolean relevance predicate that LICENSES a link A<->N: license only when relevance(A,N) holds under a geo/topic-conditioned rule, not merely when relatedness is high. Example: a Punjab-rooted person links to a US-India-immigration news item because the geo lineage makes the person relevant to that story; relevance gates relatedness into an actual permission to link.",
  "decision", ["RELATEDNESS_SCORE", "RELATION_TYPING", "HOMOPHILY"], ["CQ_07", "CQ_08"],
  b(0.94, 0.86, 0.82, 0.72, 0.9, 0.86, 0.7, 0.6, 0.62, 0.72, 0.76, 0.92, 0.6, 0.24, 0.6, [0.28, 0.66],
    "the relevance rule, geo-conditioning, or relatedness->relevance gate changes"),
  ["relevance(A,N) boolean predicate", "geo/topic-conditioned licensing rule", "relatedness-gated-into-relevance"],
  ["the magnitude of relatedness (upstream)", "arity counting (downstream)"],
  [pro("A relevance predicate separates 'these are related' from 'these may be linked here', which is the difference between a coherent scene and a topical pile", "Punjab person <-> US-India immigration is licensed by geo lineage, while Punjab person <-> unrelated Punjab band is not")],
  [con("A hand-tuned relevance rule is brittle and can encode a narrow editorial bias as if it were ground truth", "a geo rule that demands birthplace match rejects a diaspora-relevant link")],
  ["relevance conflated with relatedness so any related pair is licensed", "geo rule too strict, rejecting valid diaspora relevance"],
  ["the predicate licenses a curated set of gold-relevant pairs and rejects a curated set of related-but-irrelevant pairs above target precision/recall"],
  ["geo/topic relevance rule revised", "a related-but-irrelevant link passes the predicate in audit"],
  specialists=["content_intelligence_engineer", "editorial_lead"],
  contradictors=["relatedness_equals_relevance_advocate"],
  subfields=["relevance_modeling", "geo_conditioning"]))

N.append(node("SPURIOUS_LINK_GUARD", "topically_tempting_irrelevant_join_rejection",
  "Reject topically-tempting but irrelevant joins: pairs that score high on relatedness or share a string yet fail the relevance predicate. Acts as a hard negative filter against the most seductive false positives (shared rare word, trending co-mention, popularity coincidence) that a relatedness-only ranker would license.",
  "decision", ["GEO_RELEVANCE_PREDICATE"], ["CQ_08", "CQ_09"],
  b(0.9, 0.82, 0.8, 0.68, 0.9, 0.84, 0.66, 0.6, 0.62, 0.74, 0.76, 0.9, 0.6, 0.22, 0.6, [0.26, 0.62],
    "the spurious-pattern catalog or hard-negative rules change"),
  ["hard-negative filtering of tempting-but-irrelevant pairs", "shared-string / popularity-coincidence rejection", "relatedness-without-relevance veto"],
  ["computing relatedness (upstream)", "arity counting"],
  [pro("An explicit guard against tempting false positives raises scene precision far more than tightening the relatedness threshold, which also kills true links", "vetoing a 'both mention cricket' link that has no real relevance instead of raising the global threshold")],
  [con("An aggressive guard suppresses subtle-but-real links that resemble known spurious patterns", "a legitimately oblique relevant link is filtered as if it were a popularity coincidence")],
  ["a trending co-mention is licensed as if relevant", "guard over-filters subtle true relevance"],
  ["the guard rejects a curated hard-negative set (string collisions, trend coincidences) while passing curated true links"],
  ["new spurious pattern observed", "guard false-rejection rate exceeds target"],
  specialists=["content_intelligence_engineer", "qa_engineer"],
  subfields=["false_positive_control", "hard_negatives"]))

N.append(node("LINK_STRENGTH", "shared_high_weight_tag_link_weighting",
  "Weight each licensed link by the strength of its evidence: the sum/aggregate of shared HIGH-WEIGHT tags (rare, specific, high-IDF tags count more than common ones), combined with the relatedness magnitude. Link strength orders the licensed edges so the scene can keep its strongest links and the workflow can rank scenes.",
  "scoring", ["GEO_RELEVANCE_PREDICATE", "RELATEDNESS_SCORE"], ["CQ_09", "CQ_10"],
  b(0.8, 0.76, 0.74, 0.66, 0.78, 0.78, 0.55, 0.64, 0.55, 0.74, 0.76, 0.8, 0.64, 0.2, 0.5, [0.24, 0.56],
    "tag-weighting (IDF) scheme or strength-aggregation changes"),
  ["shared high-weight (high-IDF) tag aggregation", "relatedness-weighted link strength", "edge ranking within a scene"],
  ["the boolean license (upstream)", "arity counting (downstream)"],
  [pro("Weighting by shared rare/specific tags (IDF-style) makes a link backed by a distinctive shared tag outrank one backed by a generic tag", "a shared PLACE:jallianwala_bagh tag outweighs a shared TOPIC:india tag")],
  [con("IDF-style rare-tag weighting can over-reward a single quirky shared tag and inflate a thin link", "one rare shared tag pushes a weakly relevant pair to the top")],
  ["common tags dominate strength because IDF weighting missing", "a single rare tag over-inflates a weak link"],
  ["link strength ranks a multi-rare-tag-shared edge above a single-common-tag-shared edge"],
  ["IDF / tag-weight table recomputed", "ranking inversions observed against editorial judgment"],
  specialists=["content_intelligence_engineer"],
  subfields=["edge_weighting", "tf_idf"]))

N.append(node("CANDIDATE_GEN_VS_RANK", "high_recall_generation_then_precise_ranking",
  "Separate candidate generation (cheap, high-recall: produce MANY plausible pairs via blocking, typed-tag overlap, and the link-prediction prior) from ranking (expensive, high-precision: keep the TOP pairs by relatedness, relevance, and link strength). The two-stage design controls the quadratic blow-up of all-pairs comparison while preserving recall.",
  "structure", ["LINK_PREDICTION_PRIOR", "TYPED_TAG_JOIN"], ["CQ_10", "CQ_11"],
  b(0.82, 0.78, 0.72, 0.72, 0.8, 0.8, 0.55, 0.62, 0.58, 0.72, 0.74, 0.82, 0.62, 0.22, 0.5, [0.26, 0.6],
    "blocking strategy or generate-vs-rank split changes"),
  ["high-recall candidate generation (blocking)", "top-k precise ranking", "recall/precision stage separation"],
  ["the relevance predicate semantics", "final arity selection"],
  [pro("Generate-many-keep-top bounds the O(n^2) pairing cost while keeping recall, the standard record-linkage blocking pattern", "blocking on shared typed tags reduces comparisons by orders of magnitude before precise scoring")],
  [con("A lossy blocking key drops true pairs that never enter the candidate set, capping achievable recall", "a true link whose entities share no blocking tag is never generated")],
  ["blocking key too tight: true pairs never generated", "ranking stage starved by a too-permissive generator"],
  ["recall on a gold candidate set stays above target after blocking, and top-k ranking precision meets target"],
  ["blocking key revised", "recall at generation stage drops below target"],
  specialists=["entity_resolution_engineer", "content_intelligence_engineer"],
  subfields=["blocking", "candidate_generation"]))

N.append(node("SYMMETRY_DIRECTIONALITY", "relation_symmetry_vs_direction_handling",
  "Handle the symmetry/directionality of each relation: SAME_PLACE and SAME_TOPIC are symmetric (A~B iff B~A) while AFFECTS is DIRECTIONAL (A AFFECTS N does not imply N AFFECTS A) and CONTRASTS_WITH is symmetric. Relatedness is symmetric but the relevance and relation layers are not, so the scene must store and render direction where it matters.",
  "structure", ["RELATION_TYPING"], ["CQ_04", "CQ_11"],
  b(0.78, 0.7, 0.68, 0.6, 0.76, 0.74, 0.52, 0.66, 0.5, 0.76, 0.78, 0.78, 0.66, 0.18, 0.46, [0.22, 0.52],
    "relation symmetry classification changes"),
  ["per-relation symmetry classification", "directed storage of AFFECTS edges", "symmetric storage of SAME_*/CONTRASTS"],
  ["relatedness magnitude (always symmetric)", "arity counting"],
  [pro("Storing AFFECTS as a directed edge preserves the causal/influence reading a symmetric edge would destroy", "'immigration policy AFFECTS Punjab family' is not the same claim as the reverse")],
  [con("Tracking direction doubles edge bookkeeping and invites inconsistent direction labels", "an AFFECTS edge stored with the wrong head/tail flips the claim")],
  ["a directional AFFECTS edge stored or rendered as symmetric", "symmetric relation needlessly duplicated as two directed edges"],
  ["AFFECTS edges are directed and SAME_PLACE/SAME_TOPIC/CONTRASTS_WITH edges are symmetric in storage and rendering"],
  ["relation symmetry rules change", "a reversed-direction AFFECTS edge observed"],
  specialists=["content_intelligence_engineer"],
  subfields=["relation_directionality", "graph_modeling"]))

N.append(node("ARITY_2_3", "hard_scene_arity_two_to_three_constraint",
  "Enforce the hard arity constraint that a licensed scene links EXACTLY 2 or 3 nodes: reject a singleton (arity 1, nothing to relate) and reject a sprawl (arity > 3, no longer a focused scene). Arity is a checkable obligation evaluated after relevance licensing and link-strength ranking, selecting the strongest 2-3 mutually relevant nodes.",
  "decision", ["GEO_RELEVANCE_PREDICATE", "LINK_STRENGTH", "SPURIOUS_LINK_GUARD"], ["CQ_11", "CQ_12"],
  b(0.92, 0.82, 0.8, 0.6, 0.88, 0.82, 0.66, 0.66, 0.58, 0.78, 0.8, 0.9, 0.66, 0.18, 0.58, [0.2, 0.5],
    "the arity bounds (2..3) or selection rule change"),
  ["arity in {2,3} enforcement", "singleton and >3 rejection", "strongest-2-3 mutually-relevant selection"],
  ["computing relevance (upstream)", "relatedness magnitude"],
  [pro("A hard 2-3 arity bound keeps every scene focused and rejects both empty singletons and unfocused sprawls deterministically", "a 5-entity pile is trimmed to the 3 most mutually relevant, and a lone entity is rejected as no scene")],
  [con("A rigid 2-3 bound rejects a genuinely coherent 4-entity scene that belongs together", "a tight quartet must be split or one true member dropped")],
  ["a singleton or 4+ scene slips through unfiltered", "trimming to 3 drops a node more relevant than one kept"],
  ["every licensed scene has arity 2 or 3; singletons and arity>3 are rejected with a reason"],
  ["arity bound policy revised", "a >3 or singleton scene reaches output"],
  specialists=["content_intelligence_engineer", "editorial_lead"],
  subfields=["arity_constraints", "scene_construction"]))

N.append(node("MUTUAL_RELEVANCE", "pairwise_to_scene_coherence_closure",
  "Verify mutual (pairwise) relevance closure within a 3-node scene: it is not enough that A-B and A-C are relevant; for a coherent triad the rule decides whether B-C must also be relevant (closed triad) or whether a hub-and-spoke pattern (A central) is acceptable. Prevents incoherent triads where two members are licensed only through a third.",
  "decision", ["GEO_RELEVANCE_PREDICATE", "ARITY_2_3"], ["CQ_12", "CQ_13"],
  b(0.84, 0.78, 0.76, 0.66, 0.84, 0.82, 0.6, 0.6, 0.58, 0.74, 0.76, 0.84, 0.6, 0.22, 0.55, [0.26, 0.6],
    "triad-closure policy (closed vs hub-and-spoke) changes"),
  ["pairwise relevance closure for triads", "closed-triad vs hub-and-spoke policy", "incoherent-triad rejection"],
  ["arity counting (upstream provides the 2-3 members)", "link strength magnitude"],
  [pro("Requiring (or explicitly allowing) triad closure stops scenes whose third member is relevant only via a hub, keeping the triad genuinely coherent", "rejecting A-B-C where B and C have nothing relevant in common beyond both touching A")],
  [con("Strict closure rejects valid hub-and-spoke scenes where a central entity legitimately bridges two others", "a person who genuinely connects two otherwise-unrelated events is rejected by closure")],
  ["an incoherent hub-only triad licensed", "valid hub-and-spoke triad rejected by over-strict closure"],
  ["every 3-node scene satisfies the configured closure policy (full closure or declared hub-and-spoke)"],
  ["triad-closure policy changes", "an incoherent triad reaches output"],
  specialists=["content_intelligence_engineer"],
  subfields=["triad_closure", "scene_coherence"]))

N.append(node("THRESHOLD_CALIBRATION", "relatedness_relevance_threshold_calibration",
  "Calibrate the thresholds that turn continuous relatedness and the relevance score into discrete keep/reject decisions: choose the operating point on the precision-recall curve, map Fellegi-Sunter-style match/possible/non-match zones, and route possible-matches to review. Calibration is the dial that trades scene precision against recall.",
  "control", ["RELATEDNESS_SCORE", "GEO_RELEVANCE_PREDICATE"], ["CQ_13", "CQ_14"],
  b(0.82, 0.78, 0.72, 0.68, 0.82, 0.78, 0.58, 0.62, 0.55, 0.72, 0.74, 0.82, 0.62, 0.24, 0.55, [0.28, 0.62],
    "operating point, PR target, or review-routing policy changes"),
  ["precision-recall operating-point selection", "match/possible-match/non-match zoning", "review routing for possible-matches"],
  ["the relevance rule itself (upstream)", "arity counting"],
  [pro("An explicit calibrated operating point makes the precision/recall trade auditable instead of an emergent accident of model scores", "setting the threshold at the PR knee and routing the ambiguous band to human review")],
  [con("A single global threshold is wrong for heterogeneous relation types that need per-type operating points", "one threshold over-licenses SAME_TOPIC while under-licensing AFFECTS")],
  ["one global threshold mis-serves a relation type", "possible-match band not routed to review"],
  ["the chosen operating point meets the target precision at the required recall on the calibration set"],
  ["score distribution drifts", "precision or recall at the operating point falls below target"],
  specialists=["content_intelligence_engineer", "ml_engineer"],
  subfields=["calibration", "operating_point_selection"]))

N.append(node("CONFIDENCE_PROVENANCE", "link_confidence_and_evidence_trace",
  "Attach to each licensed link a confidence score and a provenance trace: which typed tags matched, the relatedness and co-occurrence components, the relevance rule fired, and the alternatives rejected. Confidence supports thresholding and downstream trust; provenance supports audit, explanation, and targeted repair of a bad scene.",
  "verification", ["LINK_STRENGTH", "THRESHOLD_CALIBRATION"], ["CQ_14", "CQ_15"],
  b(0.78, 0.74, 0.76, 0.58, 0.72, 0.76, 0.52, 0.7, 0.45, 0.8, 0.8, 0.78, 0.7, 0.16, 0.42, [0.18, 0.46],
    "confidence definition or provenance schema changes"),
  ["per-link confidence scoring", "evidence/provenance trace (tags, components, rule, rejects)", "explanation rendering"],
  ["the licensing decision (upstream)", "execution / publishing"],
  [pro("A provenance trace makes every scene explainable ('why these entities?') and makes repair precise instead of a blind retry", "answering 'why is this person in the immigration scene' with the matched geo tag and the AFFECTS rule")],
  [con("Full provenance per link adds storage and write overhead proportional to candidate volume", "logging every rejected alternative for every candidate pair balloons storage")],
  ["missing provenance prevents targeted repair", "confidence uncalibrated so it cannot gate decisions"],
  ["every licensed link carries a confidence and a provenance trace naming the matched tags and the rule fired"],
  ["provenance schema changes", "a scene could not be repaired for lack of provenance"],
  specialists=["content_intelligence_engineer", "observability_engineer"],
  subfields=["provenance", "explainability"]))

N.append(node("FEEDBACK_RELABEL", "human_feedback_relabeling_loop",
  "Close the loop with human/editorial feedback on emitted scenes: confirmed-good and confirmed-spurious links become labeled examples that recalibrate thresholds, expand the spurious-pattern catalog, and correct the relevance rule. Turns one-shot heuristics into a supervised, continuously-improving system.",
  "control", ["CONFIDENCE_PROVENANCE", "SPURIOUS_LINK_GUARD"], ["CQ_15", "CQ_16"],
  b(0.76, 0.74, 0.74, 0.62, 0.7, 0.78, 0.5, 0.58, 0.5, 0.7, 0.72, 0.76, 0.58, 0.24, 0.46, [0.3, 0.62],
    "feedback ingestion or relabel policy changes"),
  ["editorial confirm/reject feedback ingestion", "threshold and guard recalibration from labels", "relevance-rule correction"],
  ["initial heuristic scoring (upstream)", "publishing"],
  [pro("Feeding confirmed spurious links back into the guard and calibration steadily raises precision without re-engineering the pipeline", "a recurring trend-coincidence false positive becomes a new hard-negative pattern after one editor rejection")],
  [con("Feedback that over-fits to a few editors' taste can drift the relevance rule away from broad audience relevance", "chasing one reviewer's rejections narrows the rule to their idiosyncrasy")],
  ["feedback over-fits to a small reviewer pool", "label leakage between calibration and audit sets"],
  ["confirmed-spurious examples measurably reduce the same false-positive class on the next audit"],
  ["feedback volume or reviewer pool changes", "feedback-driven drift detected against the holdout"],
  specialists=["content_intelligence_engineer", "editorial_lead"],
  subfields=["active_learning", "human_in_the_loop"]))

N.append(node("SCENE_VALIDATION", "end_to_end_scene_acceptance",
  "Validate a fully assembled scene end-to-end before licensing it for use: arity in {2,3}, every edge relevance-licensed (not merely related), spurious guard clear, mutual-relevance closure satisfied, directionality correct, and a confidence/provenance trace present. The final gate that turns a candidate scene into an accepted, publishable artifact.",
  "verification", ["ARITY_2_3", "MUTUAL_RELEVANCE", "SYMMETRY_DIRECTIONALITY", "CONFIDENCE_PROVENANCE"], ["CQ_16", "CQ_17"],
  b(0.9, 0.84, 0.82, 0.64, 0.9, 0.84, 0.66, 0.64, 0.5, 0.8, 0.82, 0.9, 0.64, 0.18, 0.58, [0.2, 0.5],
    "the scene-acceptance checklist or gate criteria change"),
  ["end-to-end scene acceptance gate", "arity + relevance + guard + closure + direction + provenance checks", "accept/reject with reason"],
  ["scoring internals (upstream)", "downstream publishing/serving"],
  [pro("A single end-to-end acceptance gate guarantees every published scene satisfies all invariants, instead of trusting each step in isolation", "a scene failing only the closure check is rejected with that specific reason")],
  [con("A monolithic final gate can mask which earlier step is systematically failing if it only reports pass/fail", "a recurring closure failure looks like a gate problem unless per-check reasons are surfaced")],
  ["a scene violating an invariant passes the gate", "gate rejects without an actionable per-check reason"],
  ["only scenes satisfying arity, relevance, guard, closure, direction, and provenance are accepted; each rejection names the failed check"],
  ["acceptance criteria change", "an invariant-violating scene reached output"],
  specialists=["content_intelligence_engineer", "qa_engineer"],
  subfields=["scene_validation", "acceptance_gating"]))

# ---------------- competency questions (14) ----------------
CQ = [
 ("CQ_01", "How is each surface mention resolved to one canonical entity id and disambiguated to the right KB node?", ["nodes", "glossary"],
  "ENTITY_RESOLUTION canonicalizes and dedups; MENTION_DISAMBIGUATION links to the correct KB sense", ["ENTITY_RESOLUTION", "MENTION_DISAMBIGUATION"]),
 ("CQ_02", "How does the typed-tag join keep PLACE:punjab distinct from TOPIC:punjab?", ["nodes"],
  "TYPED_TAG_JOIN uses a (type,value) join key so cross-type label collisions never join", ["TYPED_TAG_JOIN", "MENTION_DISAMBIGUATION"]),
 ("CQ_03", "What closed set of relation types is assigned to each edge and why?", ["nodes"],
  "RELATION_TYPING assigns SAME_PLACE/SAME_TOPIC/AFFECTS/CONTRASTS_WITH from a closed set", ["RELATION_TYPING", "TYPED_TAG_JOIN"]),
 ("CQ_04", "Which relations are symmetric and which are directional, and how is direction stored?", ["nodes"],
  "SYMMETRY_DIRECTIONALITY classifies AFFECTS as directed and the SAME_*/CONTRASTS as symmetric", ["SYMMETRY_DIRECTIONALITY", "RELATION_TYPING"]),
 ("CQ_05", "How is relatedness between two entities computed from semantic and co-occurrence evidence?", ["nodes"],
  "RELATEDNESS_SCORE blends ESA/IC semantic relatedness with COOCCURRENCE_SIGNAL and a LINK_PREDICTION_PRIOR", ["RELATEDNESS_SCORE", "COOCCURRENCE_SIGNAL", "LINK_PREDICTION_PRIOR"]),
 ("CQ_06", "What structural and audience priors inform link plausibility and shareability?", ["nodes"],
  "LINK_PREDICTION_PRIOR gives an Adamic-Adar topology prior; HOMOPHILY estimates audience-overlap shareability", ["LINK_PREDICTION_PRIOR", "HOMOPHILY"]),
 ("CQ_07", "How does the geo-relevance predicate license a link rather than relatedness alone?", ["nodes", "workflow"],
  "GEO_RELEVANCE_PREDICATE gates relatedness into a geo/topic-conditioned relevance license; HOMOPHILY feeds shareability", ["GEO_RELEVANCE_PREDICATE", "HOMOPHILY"]),
 ("CQ_08", "How are topically-tempting but irrelevant joins rejected?", ["nodes", "conflict_axes"],
  "SPURIOUS_LINK_GUARD vetoes high-relatedness pairs that fail the relevance predicate", ["SPURIOUS_LINK_GUARD", "GEO_RELEVANCE_PREDICATE"]),
 ("CQ_09", "How is each licensed link weighted by its shared high-weight tags?", ["nodes"],
  "LINK_STRENGTH aggregates shared high-IDF tags and relatedness into an edge weight", ["LINK_STRENGTH", "SPURIOUS_LINK_GUARD"]),
 ("CQ_10", "How does generation produce many candidates while ranking keeps the precise top?", ["nodes", "workflow"],
  "CANDIDATE_GEN_VS_RANK separates high-recall blocking from precise top-k ranking by LINK_STRENGTH", ["CANDIDATE_GEN_VS_RANK", "LINK_STRENGTH"]),
 ("CQ_11", "How is the hard arity-2-to-3 constraint enforced on a scene?", ["nodes", "iteration_protocol"],
  "ARITY_2_3 rejects singletons and arity>3 and selects the strongest 2-3 mutually relevant nodes", ["ARITY_2_3", "CANDIDATE_GEN_VS_RANK", "SYMMETRY_DIRECTIONALITY"]),
 ("CQ_12", "How is mutual/pairwise relevance closure verified within a 3-node scene?", ["nodes"],
  "MUTUAL_RELEVANCE checks triad closure (closed vs hub-and-spoke) under ARITY_2_3", ["MUTUAL_RELEVANCE", "ARITY_2_3"]),
 ("CQ_13", "How are relatedness/relevance thresholds calibrated and corrected by human feedback?", ["nodes", "iteration_protocol"],
  "THRESHOLD_CALIBRATION selects the PR operating point and routes possible-matches to review; FEEDBACK_RELABEL recalibrates guards, thresholds, and the relevance rule from editorial labels", ["THRESHOLD_CALIBRATION", "MUTUAL_RELEVANCE", "FEEDBACK_RELABEL"]),
 ("CQ_14", "How is per-link confidence/provenance captured and the assembled scene validated end-to-end against its invariants?", ["nodes", "workflow"],
  "CONFIDENCE_PROVENANCE attaches confidence and an evidence trace; SCENE_VALIDATION gates arity-2-3, relevance-licensed edges, guard-clear, closure, direction, and provenance with a per-check reason", ["CONFIDENCE_PROVENANCE", "SCENE_VALIDATION"]),
]
CQS = [{"id": i, "question": q, "must_be_answerable_from": m, "acceptance_condition": a, "covered_by": c} for (i, q, m, a, c) in CQ]

# consolidate node->CQ references onto the kept CQ set (<=2 each; every CQ covered by >=1)
CQ_MAP = {
 "ENTITY_RESOLUTION": ["CQ_01"], "MENTION_DISAMBIGUATION": ["CQ_01", "CQ_02"],
 "TYPED_TAG_JOIN": ["CQ_02", "CQ_03"], "RELATION_TYPING": ["CQ_03", "CQ_04"],
 "RELATEDNESS_SCORE": ["CQ_05"], "COOCCURRENCE_SIGNAL": ["CQ_05"],
 "LINK_PREDICTION_PRIOR": ["CQ_06"], "HOMOPHILY": ["CQ_06", "CQ_07"],
 "GEO_RELEVANCE_PREDICATE": ["CQ_07", "CQ_08"], "SPURIOUS_LINK_GUARD": ["CQ_08", "CQ_09"],
 "LINK_STRENGTH": ["CQ_09", "CQ_10"], "CANDIDATE_GEN_VS_RANK": ["CQ_10", "CQ_11"],
 "SYMMETRY_DIRECTIONALITY": ["CQ_04", "CQ_11"], "ARITY_2_3": ["CQ_11", "CQ_12"],
 "MUTUAL_RELEVANCE": ["CQ_12", "CQ_13"], "THRESHOLD_CALIBRATION": ["CQ_13", "CQ_14"],
 "CONFIDENCE_PROVENANCE": ["CQ_14"], "FEEDBACK_RELABEL": ["CQ_13"],
 "SCENE_VALIDATION": ["CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

# ---------------- glossary ----------------
GL = [
 ("canonical_entity", "the single normalized id an entity resolves to after alias and transliteration dedup",
  ["resolved_entity", "entity_record"], ["surface_mention"], ["ENTITY_RESOLUTION", "MENTION_DISAMBIGUATION"]),
 ("typed_tag", "a (type,value) pair (e.g. PLACE:punjab) used as a join key so type namespaces never collide",
  ["typed_label", "namespaced_tag"], ["bare_label"], ["TYPED_TAG_JOIN", "LINK_STRENGTH"]),
 ("relatedness", "a symmetric [0,1] magnitude of semantic + co-occurrence association between two entities",
  ["semantic_relatedness", "association_score"], ["relevance"], ["RELATEDNESS_SCORE", "COOCCURRENCE_SIGNAL"]),
 ("relevance_predicate", "the boolean geo/topic-conditioned rule that LICENSES a link, distinct from relatedness",
  ["link_license", "geo_relevance"], ["relatedness"], ["GEO_RELEVANCE_PREDICATE", "SPURIOUS_LINK_GUARD"]),
 ("scene", "a licensed, validated group of exactly 2 or 3 mutually relevant entities",
  ["entity_scene", "linked_triad"], ["single_entity", "entity_pile"], ["ARITY_2_3", "SCENE_VALIDATION"]),
 ("arity", "the count of nodes in a scene; constrained to the closed interval {2,3}",
  ["scene_size", "node_count"], ["unbounded_fanout"], ["ARITY_2_3", "MUTUAL_RELEVANCE"]),
 ("homophily", "the principle that similar nodes associate, used to estimate audience overlap and shareability",
  ["birds_of_a_feather", "audience_overlap"], ["bridging_tie"], ["HOMOPHILY", "LINK_PREDICTION_PRIOR"]),
 ("spurious_link", "a topically-tempting pair that is highly related yet fails the relevance predicate",
  ["false_positive_link", "tempting_join"], ["relevant_link"], ["SPURIOUS_LINK_GUARD", "FEEDBACK_RELABEL"]),
 ("match_weight", "the Fellegi-Sunter log-likelihood-ratio score thresholded into match/possible/non-match zones",
  ["linkage_weight", "llr_score"], ["raw_string_similarity"], ["ENTITY_RESOLUTION", "THRESHOLD_CALIBRATION"]),
 ("triad_closure", "the property that all three pairs in a 3-node scene are mutually relevant (vs hub-and-spoke)",
  ["closed_triad"], ["hub_and_spoke"], ["MUTUAL_RELEVANCE", "SCENE_VALIDATION"]),
]
GLS = [{"term": t, "definition": d, "synonyms": s, "not_same_as": ns, "used_by_nodes": u} for (t, d, s, ns, u) in GL]

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
              "risk_of_conflict": "unmanaged tension degrades scene quality", "example": "see resolution_rule"})
def rel(f, t, et, rs, cc=0.7, cp=0.2, erc=0.3, why=""):
    E.append({"from": f, "to": t, "edge_type": et, "relation_strength": rs, "signed_tension": 0.0,
              "causal_confidence": cc, "conflict_probability": cp, "expected_rework_cost": erc,
              "why_related": why or f"{f} {et} {t}", "benefit_of_coupling": "coordinated behavior",
              "risk_of_conflict": "inconsistency if uncoordinated", "example": f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies)
dep("ENTITY_RESOLUTION", "MENTION_DISAMBIGUATION", 0.86)
dep("ENTITY_RESOLUTION", "TYPED_TAG_JOIN", 0.84)
dep("ENTITY_RESOLUTION", "LINK_PREDICTION_PRIOR", 0.78)
dep("TYPED_TAG_JOIN", "RELATION_TYPING", 0.84)
dep("TYPED_TAG_JOIN", "RELATEDNESS_SCORE", 0.8)
dep("MENTION_DISAMBIGUATION", "COOCCURRENCE_SIGNAL", 0.76)
dep("LINK_PREDICTION_PRIOR", "HOMOPHILY", 0.78)
dep("LINK_PREDICTION_PRIOR", "CANDIDATE_GEN_VS_RANK", 0.76)
dep("RELATION_TYPING", "GEO_RELEVANCE_PREDICATE", 0.82)
dep("RELATEDNESS_SCORE", "GEO_RELEVANCE_PREDICATE", 0.86)
dep("RELATION_TYPING", "SYMMETRY_DIRECTIONALITY", 0.8)
dep("GEO_RELEVANCE_PREDICATE", "SPURIOUS_LINK_GUARD", 0.86)
dep("GEO_RELEVANCE_PREDICATE", "LINK_STRENGTH", 0.8)
dep("GEO_RELEVANCE_PREDICATE", "ARITY_2_3", 0.84)
dep("LINK_STRENGTH", "ARITY_2_3", 0.78)
dep("ARITY_2_3", "MUTUAL_RELEVANCE", 0.82)
dep("RELATEDNESS_SCORE", "THRESHOLD_CALIBRATION", 0.78)
dep("GEO_RELEVANCE_PREDICATE", "THRESHOLD_CALIBRATION", 0.76)
dep("LINK_STRENGTH", "CONFIDENCE_PROVENANCE", 0.76)
dep("THRESHOLD_CALIBRATION", "CONFIDENCE_PROVENANCE", 0.74)
dep("CONFIDENCE_PROVENANCE", "FEEDBACK_RELABEL", 0.78)
dep("SPURIOUS_LINK_GUARD", "FEEDBACK_RELABEL", 0.72)
dep("ARITY_2_3", "SCENE_VALIDATION", 0.84)
dep("MUTUAL_RELEVANCE", "SCENE_VALIDATION", 0.8)
dep("SYMMETRY_DIRECTIONALITY", "SCENE_VALIDATION", 0.74)
dep("CONFIDENCE_PROVENANCE", "SCENE_VALIDATION", 0.78)

# cross-cutting non-dependency edges (cross freely; no cycle constraint)
rel("COOCCURRENCE_SIGNAL", "RELATEDNESS_SCORE", "causal", 0.8,
    why="the co-occurrence component is summed into the blended relatedness score")
rel("HOMOPHILY", "SPURIOUS_LINK_GUARD", "feedback", 0.72,
    why="audience-overlap signal helps the guard distinguish shareable-relevant from merely-trending links")
rel("FEEDBACK_RELABEL", "THRESHOLD_CALIBRATION", "feedback", 0.78,
    why="editorial labels recalibrate the relatedness/relevance operating point")
rel("FEEDBACK_RELABEL", "GEO_RELEVANCE_PREDICATE", "feedback", 0.74,
    why="confirmed spurious/relevant labels correct the relevance rule itself")
rel("CANDIDATE_GEN_VS_RANK", "GEO_RELEVANCE_PREDICATE", "sequence", 0.76,
    why="generation yields the candidate pairs the relevance predicate then licenses or rejects")
rel("SYMMETRY_DIRECTIONALITY", "MUTUAL_RELEVANCE", "constraint", 0.72,
    why="closure over a triad must respect each relation's symmetry/direction")
rel("CONFIDENCE_PROVENANCE", "SPURIOUS_LINK_GUARD", "similarity", 0.7,
    why="provenance traces feed the spurious-pattern catalog the guard maintains")

# conflict edges (negative signed_tension + resolution_rule); 4 of them
conf("RELATEDNESS_SCORE", "GEO_RELEVANCE_PREDICATE", 0.78, -0.62,
  "relevance dominates: never license a link from relatedness magnitude alone; the geo-relevance predicate must hold even when relatedness is maximal",
  "high relatedness pressures licensing while the relevance predicate may still reject the link")
conf("HOMOPHILY", "GEO_RELEVANCE_PREDICATE", 0.7, -0.5,
  "treat audience overlap as a shareability prior, not a license: a high-homophily pair still requires the relevance predicate, and bridging (low-homophily) relevant links are kept",
  "optimizing for audience overlap pulls toward echo-community links that may not be the most relevant")
conf("CANDIDATE_GEN_VS_RANK", "SPURIOUS_LINK_GUARD", 0.68, -0.45,
  "generate for recall, filter for precision: the generator stays permissive and the guard plus relevance predicate remove the tempting false positives it admits",
  "a high-recall generator floods the pipeline with tempting candidates the precision stages must reject")
conf("ARITY_2_3", "MUTUAL_RELEVANCE", 0.66, -0.42,
  "satisfy arity first then closure: when a 3-node scene fails closure, drop to the strongest relevant pair (arity 2) rather than admitting an incoherent triad",
  "the 2-3 arity target can pressure keeping a third node that breaks mutual-relevance closure")

# ---------------- conflict axes (9) ----------------
CA = [
 {"name": "relatedness_vs_relevance", "description": "High semantic/co-occurrence relatedness tempts licensing, but only the geo-relevance predicate should license a link; conflating them produces topical piles.", "poles": ["relatedness_magnitude", "relevance_license"], "resolution_hint": "relevance predicate gates relatedness; relatedness never licenses alone", "tension_score": 0.8, "affected_nodes": ["RELATEDNESS_SCORE", "GEO_RELEVANCE_PREDICATE", "SPURIOUS_LINK_GUARD"]},
 {"name": "homophily_vs_bridging", "description": "Audience-overlap (homophily) links are shareable but create echo bubbles; bridging links reach new audiences but are less viral.", "poles": ["max_audience_overlap", "bridging_reach"], "resolution_hint": "use homophily as a shareability prior, not a license; preserve relevant bridging links", "tension_score": 0.7, "affected_nodes": ["HOMOPHILY", "GEO_RELEVANCE_PREDICATE", "LINK_PREDICTION_PRIOR"]},
 {"name": "recall_vs_precision", "description": "A permissive generator maximizes recall but floods the ranker with tempting false positives; a strict generator caps achievable recall.", "poles": ["high_recall_generation", "high_precision_filtering"], "resolution_hint": "generate broad, filter hard with guard + relevance predicate", "tension_score": 0.72, "affected_nodes": ["CANDIDATE_GEN_VS_RANK", "SPURIOUS_LINK_GUARD", "THRESHOLD_CALIBRATION"]},
 {"name": "arity_focus_vs_coverage", "description": "A hard 2-3 arity keeps scenes focused but can drop a genuinely relevant fourth node; loosening arity sprawls the scene.", "poles": ["tight_arity_2_3", "wider_coverage"], "resolution_hint": "keep arity hard; if a 4th is truly needed split into two scenes", "tension_score": 0.66, "affected_nodes": ["ARITY_2_3", "MUTUAL_RELEVANCE", "CANDIDATE_GEN_VS_RANK"]},
 {"name": "closed_triad_vs_hub_and_spoke", "description": "Requiring full pairwise closure guarantees a coherent triad but rejects valid hub-and-spoke scenes where one entity legitimately bridges two.", "poles": ["full_closure", "hub_and_spoke"], "resolution_hint": "declare the closure policy per scene type; allow hub-and-spoke explicitly", "tension_score": 0.62, "affected_nodes": ["MUTUAL_RELEVANCE", "ARITY_2_3", "GEO_RELEVANCE_PREDICATE"]},
 {"name": "over_merge_vs_under_merge", "description": "Wide entity-resolution thresholds merge distinct entities (over-merge); strict thresholds leave aliases unmerged (under-merge), both corrupting joins.", "poles": ["aggressive_merge", "conservative_merge"], "resolution_hint": "tune Fellegi-Sunter zones; route possible-matches to review", "tension_score": 0.7, "affected_nodes": ["ENTITY_RESOLUTION", "MENTION_DISAMBIGUATION", "THRESHOLD_CALIBRATION"]},
 {"name": "typed_strictness_vs_cross_type_bridges", "description": "Strict typed-tag joins kill place/topic string collisions but also reject legitimate cross-type relations a richer model would keep.", "poles": ["strict_typing", "cross_type_flexibility"], "resolution_hint": "keep typed join strict; express genuine cross-type ties via RELATION_TYPING (AFFECTS)", "tension_score": 0.6, "affected_nodes": ["TYPED_TAG_JOIN", "RELATION_TYPING", "GEO_RELEVANCE_PREDICATE"]},
 {"name": "rare_tag_weight_vs_thin_evidence", "description": "IDF-style weighting rewards distinctive shared tags but a single rare shared tag can over-inflate a thin link.", "poles": ["rare_tag_emphasis", "evidence_robustness"], "resolution_hint": "cap single-tag contribution; require corroborating relatedness", "tension_score": 0.58, "affected_nodes": ["LINK_STRENGTH", "RELATEDNESS_SCORE", "CONFIDENCE_PROVENANCE"]},
 {"name": "feedback_fit_vs_overfit", "description": "Editorial feedback raises precision but over-fitting to a few reviewers drifts the relevance rule from broad audience relevance.", "poles": ["responsive_feedback", "drift_resistance"], "resolution_hint": "hold out an audit set; regularize feedback against broad cohorts", "tension_score": 0.6, "affected_nodes": ["FEEDBACK_RELABEL", "GEO_RELEVANCE_PREDICATE", "THRESHOLD_CALIBRATION"]},
]

# ---------------- edge cases (12) ----------------
EC = [
 {"description": "A place name and an unrelated topic share a string and get joined (PLACE:punjab linked to TOPIC:punjab).", "trigger": "join performed on bare label instead of (type,value)", "affected_nodes": ["TYPED_TAG_JOIN", "SPURIOUS_LINK_GUARD"], "mitigation": "join only on typed (type,value) keys; never match across type namespaces", "severity": "high"},
 {"description": "Two highly related entities (e.g. cricket and India) are licensed into a scene despite no real relevance to the news item.", "trigger": "relatedness magnitude used as a license without the relevance predicate", "affected_nodes": ["RELATEDNESS_SCORE", "GEO_RELEVANCE_PREDICATE", "SPURIOUS_LINK_GUARD"], "mitigation": "require the geo-relevance predicate to hold; relatedness alone never licenses", "severity": "critical"},
 {"description": "A trending co-mention spikes co-occurrence and licenses a coincidental, irrelevant link.", "trigger": "co-occurrence spike treated as relevance", "affected_nodes": ["COOCCURRENCE_SIGNAL", "SPURIOUS_LINK_GUARD"], "mitigation": "chance-correct co-occurrence and gate through the relevance predicate and guard", "severity": "high"},
 {"description": "A scene of one entity (arity 1) is emitted with nothing to relate.", "trigger": "arity constraint not enforced", "affected_nodes": ["ARITY_2_3", "SCENE_VALIDATION"], "mitigation": "reject arity<2 with a no-scene reason", "severity": "high"},
 {"description": "A sprawling 5-entity pile is emitted as a single scene (arity > 3).", "trigger": "arity upper bound not enforced", "affected_nodes": ["ARITY_2_3", "MUTUAL_RELEVANCE"], "mitigation": "trim to the strongest 2-3 mutually relevant nodes or split into multiple scenes", "severity": "high"},
 {"description": "A 3-node scene is licensed where the third node is relevant only via a hub, so two members have nothing in common.", "trigger": "triad closure not checked", "affected_nodes": ["MUTUAL_RELEVANCE", "GEO_RELEVANCE_PREDICATE"], "mitigation": "enforce the declared closure policy; drop to arity 2 if closure fails", "severity": "medium"},
 {"description": "Entity resolution over-merges two distinct people sharing a common name into one canonical id.", "trigger": "match threshold (false-match zone) set too wide", "affected_nodes": ["ENTITY_RESOLUTION", "MENTION_DISAMBIGUATION"], "mitigation": "narrow the false-match zone; route possible-matches to human review", "severity": "high"},
 {"description": "An AFFECTS edge is stored or rendered symmetric, flipping the causal claim.", "trigger": "directionality not tracked for a directional relation", "affected_nodes": ["SYMMETRY_DIRECTIONALITY", "RELATION_TYPING"], "mitigation": "store AFFECTS as a directed head->tail edge and validate direction at the gate", "severity": "medium"},
 {"description": "A single rare shared tag over-inflates an otherwise thin link to the top of the ranking.", "trigger": "uncapped IDF weight on one tag", "affected_nodes": ["LINK_STRENGTH", "RELATEDNESS_SCORE"], "mitigation": "cap single-tag contribution and require corroborating relatedness", "severity": "low"},
 {"description": "Blocking key is too tight, so a true relevant pair is never generated as a candidate.", "trigger": "lossy blocking drops a true pair before scoring", "affected_nodes": ["CANDIDATE_GEN_VS_RANK", "TYPED_TAG_JOIN"], "mitigation": "use multiple complementary blocking keys to keep recall high", "severity": "high"},
 {"description": "A licensed scene has no provenance, so a later-found bad link cannot be repaired without rebuilding.", "trigger": "confidence/provenance not recorded per link", "affected_nodes": ["CONFIDENCE_PROVENANCE", "SCENE_VALIDATION"], "mitigation": "record matched tags, score components, rule fired, and rejected alternatives per link", "severity": "medium"},
 {"description": "Editorial feedback over-fits the relevance rule to a few reviewers' taste, drifting from broad audience relevance.", "trigger": "feedback applied without a holdout audit set", "affected_nodes": ["FEEDBACK_RELABEL", "GEO_RELEVANCE_PREDICATE"], "mitigation": "regularize feedback against a held-out broad cohort and monitor drift", "severity": "medium"},
]

# ---------------- workflow (12) ----------------
WF = [
 {"action": "resolve_entities", "node_ref": "ENTITY_RESOLUTION", "description": "Resolve mentions to canonical ids and dedup aliases/transliterations using Fellegi-Sunter match weights.", "artifact": "canonical_entity_table", "gate": "each mention maps to one canonical id; no alias-sharing duplicates"},
 {"action": "disambiguate_mentions", "node_ref": "MENTION_DISAMBIGUATION", "description": "Link each ambiguous mention to the correct KB node via context + coherence.", "artifact": "disambiguated_links", "gate": "held-out mention accuracy above target"},
 {"action": "typed_tag_join", "node_ref": "TYPED_TAG_JOIN", "description": "Join entities and news on (type,value) typed tags, rejecting cross-type collisions.", "artifact": "typed_join_pairs", "gate": "no join shares a value across differing types"},
 {"action": "generate_candidates", "node_ref": "CANDIDATE_GEN_VS_RANK", "description": "Produce a high-recall candidate set via blocking, typed-tag overlap, and the link-prediction prior.", "artifact": "candidate_pairs", "gate": "candidate recall above target on the gold set"},
 {"action": "score_relatedness", "node_ref": "RELATEDNESS_SCORE", "description": "Compute symmetric relatedness blending ESA/IC semantics with chance-corrected co-occurrence.", "artifact": "relatedness_scores", "gate": "relatedness symmetric and in [0,1]; ranks gold pairs correctly"},
 {"action": "type_relations", "node_ref": "RELATION_TYPING", "description": "Assign each candidate edge a closed relation type and route per-type checks.", "artifact": "typed_edges", "gate": "every edge has one relation type from the closed set"},
 {"action": "apply_relevance_predicate", "node_ref": "GEO_RELEVANCE_PREDICATE", "description": "License only pairs where the geo/topic-conditioned relevance predicate holds.", "artifact": "licensed_links", "gate": "no link licensed on relatedness alone"},
 {"action": "guard_spurious", "node_ref": "SPURIOUS_LINK_GUARD", "description": "Veto tempting-but-irrelevant pairs (string collisions, trend coincidences).", "artifact": "guarded_links", "gate": "curated hard negatives rejected; true links retained"},
 {"action": "weight_links", "node_ref": "LINK_STRENGTH", "description": "Weight each licensed link by shared high-IDF tags and relatedness, then rank.", "artifact": "weighted_ranked_links", "gate": "multi-rare-tag links rank above single-common-tag links"},
 {"action": "enforce_arity", "node_ref": "ARITY_2_3", "description": "Select the strongest 2-3 mutually relevant nodes; reject singletons and arity>3.", "artifact": "scene_candidate", "gate": "scene arity in {2,3}"},
 {"action": "check_mutual_relevance", "node_ref": "MUTUAL_RELEVANCE", "description": "Verify triad closure (closed or declared hub-and-spoke) and directionality.", "artifact": "coherent_scene", "gate": "closure policy satisfied; AFFECTS directed correctly"},
 {"action": "validate_scene", "node_ref": "SCENE_VALIDATION", "description": "Run the end-to-end acceptance gate (arity, relevance, guard, closure, direction, provenance) and accept or reject with a reason.", "artifact": "accepted_scene", "gate": "all invariants satisfied; rejection names the failed check"},
]

# ---------------- dominance rules (9) ----------------
DR = [
 {"rule": "GEO_RELEVANCE_PREDICATE must hold before any link is licensed; relatedness magnitude never licenses alone", "rationale": "relatedness is not relevance; licensing on relatedness produces topical piles", "trigger": "a link is about to be licensed on relatedness score alone", "action": "block licensing until the relevance predicate holds"},
 {"rule": "TYPED_TAG_JOIN must use (type,value) keys; bare-label joins are forbidden", "rationale": "place/topic string collisions are the most common spurious link", "trigger": "a join is attempted on a bare label", "action": "reject the join and require a typed key"},
 {"rule": "ARITY_2_3 must reduce every scene to exactly 2 or 3 nodes before SCENE_VALIDATION", "rationale": "singletons have nothing to relate and arity>3 is an unfocused pile", "trigger": "a scene with arity 1 or >3 reaches validation", "action": "reject or trim to the strongest 2-3 mutually relevant nodes"},
 {"rule": "SPURIOUS_LINK_GUARD must clear a pair before it counts toward a scene", "rationale": "tempting false positives sink precision more than threshold tuning fixes", "trigger": "a high-relatedness pair failing relevance enters scene assembly", "action": "veto the pair as a hard negative"},
 {"rule": "ENTITY_RESOLUTION canonical ids must be fixed before any typed-tag join or relatedness scoring", "rationale": "joining or scoring raw strings double-counts duplicates and corrupts ranking", "trigger": "a join or score runs on unresolved surface mentions", "action": "block until mentions are canonicalized"},
 {"rule": "MUTUAL_RELEVANCE closure policy must be satisfied for every 3-node scene", "rationale": "a hub-only triad is incoherent unless hub-and-spoke is explicitly declared", "trigger": "a triad fails the declared closure policy", "action": "drop to the strongest relevant pair (arity 2)"},
 {"rule": "SYMMETRY_DIRECTIONALITY must store AFFECTS as directed before SCENE_VALIDATION", "rationale": "rendering a directional relation as symmetric flips the claim", "trigger": "an AFFECTS edge is stored symmetric", "action": "require a head->tail direction and validate it"},
 {"rule": "THRESHOLD_CALIBRATION must route possible-match band to review, not auto-decide", "rationale": "the ambiguous zone is where over/under-merge and false licensing concentrate", "trigger": "a possible-match is auto-accepted or auto-rejected", "action": "route to human review"},
 {"rule": "CONFIDENCE_PROVENANCE must be present on every licensed link before publishing", "rationale": "without provenance a bad scene cannot be repaired or explained", "trigger": "a link reaches output without a provenance trace", "action": "block publishing until provenance is recorded"},
]

# ---------------- anti-rework rules (9) ----------------
ARR = [
 {"rule": "Do not license links on relatedness before the relevance predicate exists; relabeling a relatedness-only batch later is costly", "prevents": "mass un-licensing of topical piles after the relevance rule lands"},
 {"rule": "Do not join on bare labels; retrofitting typed keys after collisions requires re-joining everything", "prevents": "full re-join after place/topic string collisions are discovered"},
 {"rule": "Do not skip entity resolution before scoring; deduping after ranking invalidates all scores", "prevents": "re-scoring every pair after duplicate canonical ids are merged"},
 {"rule": "Do not emit scenes without enforcing arity; trimming sprawls after publishing breaks already-shared scenes", "prevents": "post-publish scene splits and re-licensing"},
 {"rule": "Do not over-merge entities with a wide false-match zone; un-merging a collapsed entity requires reprocessing its links", "prevents": "cascading link rework after an over-merge is reversed"},
 {"rule": "Do not store AFFECTS symmetrically; correcting direction after rendering requires re-validating every directional edge", "prevents": "bulk direction re-validation after a symmetric-storage bug"},
 {"rule": "Do not skip provenance recording; reconstructing why a scene was licensed requires re-running the whole pipeline", "prevents": "full pipeline replay to debug a single bad scene"},
 {"rule": "Do not tune one global threshold across relation types; per-type recalibration later forces re-deciding every band", "prevents": "re-decision of all borderline links after splitting thresholds per relation type"},
 {"rule": "Do not apply editorial feedback without a holdout; un-drifting an over-fit relevance rule requires reverting and re-auditing", "prevents": "rule rollback and re-audit after feedback-driven drift"},
]

# ---------------- iteration protocol (8) ----------------
IP = [
 {"trigger": "a related-but-irrelevant link reaches output", "action": "tighten the relevance predicate in GEO_RELEVANCE_PREDICATE and add the pattern to SPURIOUS_LINK_GUARD", "nodes": ["GEO_RELEVANCE_PREDICATE", "SPURIOUS_LINK_GUARD"], "priority": "critical"},
 {"trigger": "a place/topic string collision is observed", "action": "verify the join uses (type,value) keys in TYPED_TAG_JOIN and audit the candidate generator", "nodes": ["TYPED_TAG_JOIN", "CANDIDATE_GEN_VS_RANK"], "priority": "high"},
 {"trigger": "a scene with arity 1 or >3 reaches validation", "action": "re-run ARITY_2_3 selection and confirm SCENE_VALIDATION rejects out-of-range arity", "nodes": ["ARITY_2_3", "SCENE_VALIDATION"], "priority": "high"},
 {"trigger": "an incoherent hub-only triad is published", "action": "enforce the closure policy in MUTUAL_RELEVANCE and drop to arity 2 on failure", "nodes": ["MUTUAL_RELEVANCE", "ARITY_2_3"], "priority": "high"},
 {"trigger": "over-merge or under-merge observed in canonical ids", "action": "re-zone Fellegi-Sunter thresholds in ENTITY_RESOLUTION and route possible-matches via THRESHOLD_CALIBRATION", "nodes": ["ENTITY_RESOLUTION", "THRESHOLD_CALIBRATION"], "priority": "high"},
 {"trigger": "candidate recall drops below target", "action": "add complementary blocking keys in CANDIDATE_GEN_VS_RANK and re-check TYPED_TAG_JOIN coverage", "nodes": ["CANDIDATE_GEN_VS_RANK", "TYPED_TAG_JOIN"], "priority": "medium"},
 {"trigger": "a reversed-direction AFFECTS edge is found", "action": "fix directional storage in SYMMETRY_DIRECTIONALITY and re-validate at SCENE_VALIDATION", "nodes": ["SYMMETRY_DIRECTIONALITY", "SCENE_VALIDATION"], "priority": "medium"},
 {"trigger": "feedback-driven drift detected against the holdout", "action": "regularize FEEDBACK_RELABEL against the broad cohort and re-audit THRESHOLD_CALIBRATION", "nodes": ["FEEDBACK_RELABEL", "THRESHOLD_CALIBRATION"], "priority": "medium"},
]

spec = {
 "domain": "link__entity_relevance",
 "domain_label": "Entity Linking & Geo-Relevance Combination",
 "purpose": "license_a_scene_of_2_to_3_related_entities_via_typed_tag_joins_relatedness_scoring_and_a_geo_relevance_predicate_with_arity_2_to_3_enforced",
 "assumptions": [
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "a knowledge graph of canonical entities and a typed-tag taxonomy (PLACE/TOPIC/PERSON/ORG/EVENT) are available",
   "a semantic-relatedness index (ESA or an IC-bearing taxonomy) and a co-occurrence corpus are available",
   "the geo/topic relevance rule is editorially specified and revisable from feedback",
 ],
 "exclusions": [
   "building the knowledge graph and tag taxonomy themselves (delegated upstream)",
   "downstream rendering, publishing, and serving of accepted scenes",
   "training the ESA/relatedness model from raw corpora",
   "audience-segmentation modeling beyond the homophily/shareability prior (delegated to niche__audience_segmentation)",
 ],
 "source_description": "heuristic prior estimates for entity-linking and geo-relevance combination work units, informed by record-linkage theory, semantic-relatedness models, knowledge-graph entity linking, homophily, and link-prediction; no supplied dataset",
 "source_citation": "Fellegi & Sunter 1969 A Theory for Record Linkage (JASA); Resnik 1995 Using Information Content to Evaluate Semantic Similarity (IJCAI); Gabrilovich & Markovitch 2007 Computing Semantic Relatedness using Wikipedia-based Explicit Semantic Analysis (IJCAI); Shen, Wang & Han 2015 Entity Linking with a Knowledge Base (IEEE TKDE) and Hoffart et al. 2011 Robust Disambiguation of Named Entities (AIDA, EMNLP); McPherson, Smith-Lovin & Cook 2001 Birds of a Feather: Homophily in Social Networks (Annual Review of Sociology); Adamic & Adar 2003 Friends and Neighbors on the Web (Social Networks)",
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
 "priority_rationale": "ENTITY_RESOLUTION/MENTION_DISAMBIGUATION/TYPED_TAG_JOIN are foundational; RELATEDNESS_SCORE and GEO_RELEVANCE_PREDICATE are the relatedness-to-relevance core; SPURIOUS_LINK_GUARD and ARITY_2_3 gate scene precision and shape; MUTUAL_RELEVANCE, CONFIDENCE_PROVENANCE and SCENE_VALIDATION close the method last.",
 "eval_objective": "verify_entity_resolution_typed_tag_joins_relatedness_vs_relevance_gating_spurious_guarding_and_arity_2_to_3_scene_validation_of_link__entity_relevance_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "link__entity_relevance.spec.json")
open(path, "w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes", len(N), "edges", len(E), "CA", len(CA), "EC", len(EC), "WF", len(WF),
      "CQ", len(CQS), "DR", len(DR), "ARR", len(ARR), "IP", len(IP))
