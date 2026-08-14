#!/usr/bin/env python3
"""Generate the acq__evidence_sourcing content spec (B11 KB) for kb_forge.py.
Compact authoring: node() applies sane defaults so only domain content + base
metric magnitudes are specified per node.

Domain: a session-bounded operational method for ACQUIRING and GRADING evidence
to close knowledge gaps — typing sources, grouping by independence, triangulating,
tracking provenance/freshness, grading with a GRADE-style tier, enforcing
evidence-label legality (a 'high' label requires >=2 sources in distinct
independence groups), and deciding when to stop. Neutral/epistemic: it appraises
evidence quality, it does NOT assert domain conclusions."""
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
        "academic_fields": ["knowledge_acquisition", "evidence_appraisal", "information_science"],
        "subfields": subfields or ["source_evaluation", "evidence_grading"],
        "specialists": specialists or ["evidence_analyst"],
        "contradictors": contradictors or ["single_source_advocate"],
        "inputs": inputs or ["knowledge gap / competency question", "candidate sources"],
        "outputs": outputs or ["graded evidence record"],
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

N.append(node("ACQUISITION_PLAN","gap_driven_acquisition_planning",
  "For each open knowledge gap (competency question), plan what evidence would close it: the claim to be supported, the kinds of sources that could bear on it, and the target evidence tier, before any source is collected.",
  "foundations", [], ["CQ_01"],
  b(0.9,0.8,0.82,0.55,0.84,0.78,0.55,0.7,0.42, 0.78,0.82, 0.9,0.7,0.16,0.5,[0.2,0.5],"a gap is added, retired, or its target evidence tier changes"),
  ["per-gap evidence requirement","candidate source kinds","target tier per claim"],
  ["assigning a final grade","asserting the answer itself"],
  [pro("A gap-driven plan makes acquisition purposeful and bounds search to what would actually change the answer","planning that a dosage claim needs >=2 independent trials before collecting any source")],
  [con("Planning the desired evidence shape can bias collection toward confirming sources","specifying 'find trials that show benefit' instead of 'find trials about the effect'")],
  ["collecting sources with no target claim or tier","plan conflates the question with a preferred answer"],
  ["every active gap has a target claim and a target evidence tier before collection begins"],
  ["gap set revised","a target tier policy changes"],
  specialists=["evidence_analyst","research_methodologist"]))

N.append(node("SOURCE_TYPING","source_type_classification",
  "Classify each candidate source by epistemic type: primary vs secondary, and empirical (observational/experimental/measured) vs testimonial (assertion/opinion/report), so its evidential weight is determined by what kind of thing it is.",
  "typing", ["ACQUISITION_PLAN"], ["CQ_02"],
  b(0.86,0.74,0.72,0.6,0.8,0.8,0.5,0.68,0.45, 0.76,0.8, 0.86,0.68,0.18,0.5,[0.2,0.52],"the source-type taxonomy or empirical/testimonial boundary is revised"),
  ["primary/secondary tag","empirical/testimonial tag","study-design subtype"],
  ["independence grouping","authority weighting"],
  [pro("Typing each source up front lets downstream weighting and grading apply type-appropriate rules rather than treating all citations alike","a peer-reviewed RCT and a tweet quoting it are not weighed the same once typed")],
  [con("Type labels can be gamed by presentation; a testimonial dressed as data can be mis-typed as empirical","a marketing whitepaper formatted like a study")],
  ["empirical claim backed only by testimonial source","secondary source mistaken for primary evidence"],
  ["every collected source carries a resolved primary/secondary and empirical/testimonial type"],
  ["taxonomy extended","a mis-typing observed in audit"],
  specialists=["evidence_analyst","information_scientist"]))

N.append(node("PRIMARY_SECONDARY","primary_vs_secondary_distinction",
  "Maintain the primary/secondary distinction precisely: a primary source reports the original observation, study, or record; a secondary source describes, summarizes, or interprets primary sources. Resolve mixed sources to the part being cited.",
  "typing", ["SOURCE_TYPING"], ["CQ_02","CQ_03"],
  b(0.8,0.7,0.68,0.58,0.76,0.74,0.48,0.7,0.42, 0.78,0.8, 0.8,0.7,0.16,0.46,[0.18,0.48],"the primary/secondary criteria are refined"),
  ["original-record identification","summary/interpretation detection","cited-part resolution"],
  ["sampling appraisal","conflict-of-interest flagging"],
  [pro("Tracing to the primary source removes layers of interpretation and lets the original evidence be appraised directly","a systematic review (secondary) is followed to the trials (primary) it pools")],
  [con("Primary is not automatically better: a flawed primary study can be worse than a careful secondary synthesis","a single small biased trial vs a rigorous meta-analysis")],
  ["citing a secondary summary as if it were the primary evidence","a review's framing inherited without checking the primaries"],
  ["each cited claim is resolved to whether it rests on a primary or a secondary source"],
  ["primary/secondary criteria change","a misattributed primary observed"]))

N.append(node("INDEPENDENCE_GROUP","independence_grouping",
  "Partition sources into independence groups: sources that ultimately derive from the same origin, dataset, author cluster, or funder share a group. Two sources corroborate only if they sit in DISTINCT independence groups.",
  "independence", ["SOURCE_TYPING"], ["CQ_04","CQ_05"],
  b(0.92,0.82,0.78,0.72,0.88,0.86,0.62,0.62,0.6, 0.72,0.76, 0.92,0.62,0.2,0.58,[0.26,0.62],"the independence criteria or grouping policy changes"),
  ["origin/dataset/funder clustering","shared-author detection","group assignment per source"],
  ["assigning the final tier","collection itself"],
  [pro("Grouping by independence is what makes corroboration meaningful: distinct groups give genuinely independent confirmation","three outlets all reprinting one wire story collapse into one group, not three confirmations")],
  [con("Independence is a judgment call; over-splitting fakes independence, over-merging discards real corroboration","treating two labs sharing a dataset as independent because the papers differ")],
  ["counting common-source repeats as independent confirmation","merging genuinely independent sources into one group"],
  ["every source is assigned an independence group, and corroboration is computed only across distinct groups"],
  ["independence criteria change","a hidden common source is discovered after grouping"],
  specialists=["evidence_analyst","intelligence_analyst"]))

N.append(node("CITATION_CHAIN","citation_chain_tracing",
  "Trace each source's citation chain back toward primaries and detect circular reporting: where A cites B, B cites C, and C cites A (or all trace to one origin), so apparent breadth collapses to a single point.",
  "independence", ["PRIMARY_SECONDARY","INDEPENDENCE_GROUP"], ["CQ_05"],
  b(0.85,0.76,0.72,0.74,0.86,0.84,0.58,0.6,0.58, 0.7,0.74, 0.85,0.6,0.22,0.56,[0.28,0.64],"the chain-tracing depth policy or circularity definition changes"),
  ["back-tracing to primaries","circular-reporting detection","origin collapse identification"],
  ["tier assignment","stopping decision"],
  [pro("Following citation chains exposes circular reporting that independence grouping alone can miss","five 'independent' articles all footnote the same retracted study")],
  [con("Chains can be deep, ambiguous, or paywalled, so full tracing is sometimes impractical","a claim's origin sits behind an unrecoverable internal report")],
  ["circular reporting mistaken for corroboration","chain tracing abandoned, origin left unknown"],
  ["every corroborating set is traced far enough to rule out a shared origin or an explicit circularity is flagged"],
  ["circularity definition changes","a circular-reporting incident is found post-grade"],
  specialists=["evidence_analyst","intelligence_analyst"]))

N.append(node("TRIANGULATION","cross_group_triangulation",
  "Triangulate a claim across independent groups and across source types: a claim gains strength when distinct independence groups, ideally using different methods, converge on it.",
  "synthesis", ["INDEPENDENCE_GROUP","CITATION_CHAIN"], ["CQ_06"],
  b(0.88,0.8,0.76,0.7,0.84,0.85,0.58,0.62,0.55, 0.72,0.76, 0.88,0.62,0.2,0.55,[0.24,0.6],"the triangulation rule or method-diversity policy changes"),
  ["cross-group convergence","method-diversity weighting","convergence vs divergence summary"],
  ["final tier assignment","provenance recording"],
  [pro("Convergence across independent groups using different methods is far stronger than repetition within one method","a survey, an administrative record, and an experiment all pointing the same way")],
  [con("Triangulation can manufacture false confidence if the 'independent' methods share a hidden bias","three methods all built on the same flawed sampling frame")],
  ["calling within-group agreement triangulation","ignoring a divergent independent group"],
  ["a claim's tier reflects convergence across distinct independence groups, not within-group repetition"],
  ["triangulation rule changes","a converged claim later contradicted by a missed group"]))

N.append(node("CORROBORATION","corroboration_and_contradiction_tracking",
  "Track, per claim, which sources corroborate and which contradict it, with the contradiction left visible rather than averaged away, so that disagreement is an explicit signal feeding the tier.",
  "synthesis", ["TRIANGULATION"], ["CQ_06","CQ_07"],
  b(0.82,0.74,0.72,0.62,0.8,0.8,0.55,0.64,0.55, 0.74,0.76, 0.82,0.64,0.2,0.52,[0.22,0.56],"the corroboration/contradiction accounting policy changes"),
  ["per-claim support tally","explicit contradiction log","disagreement surfaced to tier"],
  ["asserting which side is correct","stopping rule"],
  [pro("Keeping contradictions explicit prevents premature consensus and forces the tier to reflect real disagreement","two strong independent groups disagree, so the claim cannot be graded high")],
  [con("Counting sources for/against invites vote-counting that ignores quality differences","five weak corroborations outvoting one strong contradiction")],
  ["averaging away a credible contradiction","vote-counting sources instead of weighing them"],
  ["every claim records both its corroborating and its contradicting sources, and unresolved contradiction caps the tier"],
  ["accounting policy changes","a buried contradiction surfaces after grading"]))

N.append(node("PROVENANCE","provenance_recording",
  "Record the provenance of each source: who produced it, when, from what, through what chain of custody, so every judgment can be traced back to an identifiable origin.",
  "provenance", ["SOURCE_TYPING"], ["CQ_08"],
  b(0.84,0.76,0.74,0.58,0.78,0.8,0.6,0.68,0.45, 0.8,0.82, 0.84,0.68,0.16,0.48,[0.18,0.46],"the provenance schema or required fields change"),
  ["origin/author/date capture","chain-of-custody record","retrieval locator"],
  ["freshness scoring","authority weighting"],
  [pro("Recorded provenance is the precondition for independence grouping, freshness, and later audit or revision","a claim can be re-checked because its source, date, and access path were logged")],
  [con("Provenance capture adds overhead and is only as good as what the source discloses","an anonymized leak with no verifiable origin")],
  ["judgment recorded without a traceable source","provenance lost so revision is impossible"],
  ["every source carries origin, date, and a retrieval locator sufficient to re-find and re-appraise it"],
  ["provenance schema changes","a source cannot be re-located when re-checked"],
  specialists=["evidence_analyst","scholarly_communication_specialist"]))

N.append(node("FRESHNESS","recency_and_freshness_assessment",
  "Assess each source's recency relative to the claim's volatility: how quickly the underlying facts change and whether the source is current enough, while not discounting durable, well-established findings merely for being old.",
  "appraisal", ["PROVENANCE"], ["CQ_08","CQ_09"],
  b(0.78,0.72,0.7,0.56,0.76,0.74,0.5,0.66,0.5, 0.78,0.78, 0.78,0.66,0.18,0.46,[0.2,0.5],"the volatility model or freshness thresholds change"),
  ["claim-volatility estimate","source-age vs volatility check","supersession detection"],
  ["authority weighting","tier assignment"],
  [pro("Freshness keyed to volatility flags stale sources on fast-moving claims without penalizing stable established knowledge","a 2-year-old price is stale; a decades-old physical constant is not")],
  [con("Recency can be mistaken for reliability: the newest source is not automatically the most trustworthy","a fresh preprint preferred over a replicated older study")],
  ["stale source used for a volatile claim","durable finding discarded only for age"],
  ["each source's recency is judged against the claim's volatility, with supersession flagged"],
  ["volatility model changes","a stale source is found to have driven a grade"]))

N.append(node("AUTHORITY","authority_and_expertise_weighting",
  "Weight a source by the relevant authority and expertise of its producer in the specific claim domain, distinguishing demonstrated domain competence from generic fame or credential transfer.",
  "appraisal", ["SOURCE_TYPING","PROVENANCE"], ["CQ_09","CQ_10"],
  b(0.8,0.74,0.72,0.6,0.78,0.78,0.52,0.64,0.55, 0.74,0.76, 0.8,0.64,0.2,0.5,[0.22,0.54],"the authority rubric or domain-relevance criteria change"),
  ["domain-relevant expertise check","credential-vs-claim-domain match","track-record weighting"],
  ["independence grouping","conflict flagging"],
  [pro("Weighting domain-relevant expertise keeps appraisal grounded in competence on the actual claim","a virologist on a virology claim outweighs a celebrity endorsement")],
  [con("Authority weighting can become an appeal to authority that overrides independent corroboration","deferring to one eminent expert against several independent contradicting sources")],
  ["credential in one field transferred to an unrelated claim","fame counted as evidence"],
  ["authority weight reflects expertise relevant to the specific claim, not generic prominence"],
  ["authority rubric changes","a famous-source deference error is detected"],
  specialists=["evidence_analyst","subject_matter_expert"]))

N.append(node("SOURCE_BIAS","source_bias_appraisal",
  "Appraise systematic bias in a source: framing, selection, ideological or commercial slant, and direction of likely distortion, separate from random error, so the bias direction is carried into grading.",
  "appraisal", ["SOURCE_TYPING"], ["CQ_10","CQ_11"],
  b(0.8,0.72,0.7,0.64,0.8,0.78,0.52,0.6,0.58, 0.72,0.74, 0.8,0.6,0.22,0.52,[0.26,0.6],"the bias taxonomy or appraisal method changes"),
  ["framing/selection bias detection","slant direction estimate","bias separated from random error"],
  ["conflict-of-interest flagging","red-team check"],
  [pro("Naming the likely direction of bias lets corroboration across oppositely-biased independent groups be especially informative","agreement between a pro- and an anti- aligned source is strong signal")],
  [con("Bias appraisal can itself be biased; an analyst may over-discount sources they disagree with","dismissing a credible source as biased to protect a prior")],
  ["bias unexamined so slant leaks into the grade","analyst's own bias drives the appraisal"],
  ["each source carries a bias appraisal with an estimated direction of distortion"],
  ["bias taxonomy changes","a slanted source is found to have skewed a grade"]))

N.append(node("CONFLICT_OF_INTEREST","conflict_of_interest_flagging",
  "Flag conflicts of interest: financial, institutional, or personal stakes the source's producer has in the claim's outcome, recorded as a provenance-linked flag that constrains how far the source can lift a tier.",
  "appraisal", ["PROVENANCE","SOURCE_BIAS"], ["CQ_11"],
  b(0.82,0.74,0.7,0.6,0.82,0.78,0.55,0.6,0.58, 0.74,0.76, 0.82,0.6,0.22,0.54,[0.26,0.62],"the conflict-of-interest disclosure policy changes"),
  ["financial/institutional stake detection","disclosure capture","COI-linked tier constraint"],
  ["independence grouping","stopping decision"],
  [pro("An explicit COI flag prevents a stakeholder's self-serving evidence from silently inflating confidence","a manufacturer-funded study of its own product is flagged before it lifts a tier")],
  [con("COI is a risk indicator, not a refutation; conflicted sources can still be correct","over-discounting all industry research regardless of methodology")],
  ["undisclosed conflict inflates a tier","conflicted source treated as automatically false"],
  ["every source's known conflicts of interest are flagged and bounded in their tier influence"],
  ["disclosure policy changes","an undisclosed conflict is discovered after grading"]))

N.append(node("SAMPLING_BIAS","sampling_and_coverage_bias",
  "Assess sampling and coverage bias in empirical sources: whether the studied or measured population represents the claim's target population, and whether the search itself missed whole classes of evidence.",
  "appraisal", ["PRIMARY_SECONDARY"], ["CQ_12"],
  b(0.8,0.72,0.68,0.66,0.8,0.76,0.55,0.6,0.55, 0.72,0.74, 0.8,0.6,0.22,0.5,[0.26,0.6],"the sampling-appraisal or search-coverage policy changes"),
  ["representativeness check","search-coverage gap detection","publication-bias consideration"],
  ["authority weighting","final tier"],
  [pro("Checking representativeness and search coverage catches confident conclusions drawn from unrepresentative or selectively published evidence","a benefit seen only in one demographic, or only in published positive trials")],
  [con("Perfect coverage is unattainable; over-strict representativeness can reject all available evidence","demanding a globally representative sample for a local question")],
  ["unrepresentative sample generalized to the target population","search omits a whole evidence class"],
  ["each empirical source's representativeness and the search's coverage gaps are appraised"],
  ["sampling policy changes","a coverage gap is found to have biased a conclusion"],
  specialists=["evidence_analyst","statistician"]))

N.append(node("REPRODUCIBILITY","verifiability_and_reproducibility_check",
  "Check whether a source's claim is verifiable or reproducible: whether the method, data, or record could in principle be re-run or re-inspected to reach the same finding, and whether independent replication exists.",
  "appraisal", ["PRIMARY_SECONDARY"], ["CQ_12","CQ_13"],
  b(0.82,0.74,0.7,0.66,0.82,0.78,0.55,0.6,0.55, 0.72,0.74, 0.82,0.6,0.22,0.54,[0.26,0.62],"the reproducibility / verifiability criteria change"),
  ["method/data availability check","replication-existence check","verifiability classification"],
  ["tier assignment","red-team check"],
  [pro("Verifiability and replication are strong, hard-to-fake quality signals that a single confident assertion cannot match","an effect that independently replicates outranks a charismatic one-off claim")],
  [con("Reproducibility evidence is often absent even for true claims, so its lack must not be read as falsity","a sound field observation that cannot be re-run")],
  ["unverifiable assertion graded as if confirmed","absence of replication read as disproof"],
  ["each claim records whether it is verifiable and whether independent replication exists"],
  ["reproducibility criteria change","a non-reproducible result is found to have anchored a grade"]))

N.append(node("RED_TEAM_SOURCE","adversarial_source_and_disinformation_check",
  "Adversarially stress each pivotal source: could it be fabricated, planted, manipulated, or an artifact of a disinformation/astroturfing campaign engineered to look like independent corroboration?",
  "adversarial", ["INDEPENDENCE_GROUP","SOURCE_BIAS"], ["CQ_13","CQ_14"],
  b(0.84,0.76,0.72,0.7,0.86,0.82,0.58,0.58,0.62, 0.7,0.74, 0.84,0.58,0.24,0.58,[0.3,0.68],"the threat model or disinformation indicators change"),
  ["fabrication/manipulation probe","astroturf/coordinated-origin check","planted-evidence consideration"],
  ["assigning the answer","cost accounting"],
  [pro("An adversarial pass catches engineered corroboration that ordinary appraisal, taking sources at face value, would accept","a swarm of fake 'independent' reviews tracing to one operation")],
  [con("Adversarial skepticism can slide into unfalsifiable conspiracy that rejects all evidence","dismissing every contrary source as a plant")],
  ["coordinated disinformation accepted as independent corroboration","skepticism becomes unfalsifiable and rejects everything"],
  ["each pivotal corroboration is checked against a fabrication / coordinated-origin threat model"],
  ["threat model changes","an engineered-corroboration incident is found post-grade"],
  specialists=["evidence_analyst","intelligence_analyst"]))

N.append(node("GRADE_TIER","grade_style_tier_assignment",
  "Assign a GRADE-style evidence tier (high / moderate / low / very_low) to each claim from the full appraisal: source types, independence, triangulation, corroboration/contradiction, freshness, authority, bias, COI, sampling, reproducibility, and the adversarial check.",
  "grading", ["CORROBORATION","FRESHNESS","AUTHORITY","SOURCE_BIAS","CONFLICT_OF_INTEREST","SAMPLING_BIAS","REPRODUCIBILITY","RED_TEAM_SOURCE"], ["CQ_15","CQ_16"],
  b(0.94,0.84,0.8,0.74,0.9,0.88,0.66,0.6,0.62, 0.72,0.76, 0.94,0.6,0.22,0.6,[0.26,0.62],"the GRADE tier rubric or its inputs change"),
  ["tier rubric application","upgrade/downgrade reasoning","tier with justification"],
  ["asserting the claim is true","label-legality enforcement"],
  [pro("A single explicit tier, derived from many appraisal dimensions, gives a transparent, auditable confidence summary per claim","a claim downgraded for inconsistency and upgraded for a large independent effect, with reasons logged")],
  [con("Compressing many dimensions into one ordinal tier loses nuance and invites disputes at the boundaries","two analysts split between moderate and high on the same body of evidence")],
  ["tier asserted without recorded reasons","tier read as a claim of truth rather than confidence in evidence"],
  ["every graded claim carries a GRADE-style tier with explicit upgrade/downgrade justification"],
  ["GRADE rubric changes","two analysts disagree on a tier for the same evidence body"],
  specialists=["evidence_analyst","grade_methodologist"]))

N.append(node("EVIDENCE_LABEL_LEGAL","evidence_label_legality_gate",
  "Enforce evidence-label legality: a tier may only be applied if its structural preconditions hold. A 'high' label REQUIRES a non-null claim plus >=2 sources in DISTINCT independence groups; 'moderate' requires >=1 source; weaker labels carry no source-count obligation. An illegal label is rejected, not downgraded silently.",
  "grading", ["GRADE_TIER","INDEPENDENCE_GROUP"], ["CQ_16","CQ_17"],
  b(0.95,0.85,0.82,0.7,0.92,0.86,0.7,0.62,0.6, 0.74,0.78, 0.95,0.62,0.2,0.62,[0.24,0.58],"the label-legality rule or its source-count thresholds change"),
  ["distinct-group count check","label-precondition enforcement","illegal-label rejection"],
  ["the substantive tier judgment itself","stopping decision"],
  [pro("A legality gate makes a strong label structurally impossible without genuine independent corroboration, closing the gap between a confident grade and the evidence backing it","a 'high' with two same-origin sources is rejected as illegal before it can be recorded")],
  [con("Strict legality can block a defensible 'high' when only one strong source exists yet a decision is needed under thin evidence","one definitive but lone audited record cannot earn 'high' under the rule")],
  ["a high label recorded with sources sharing one independence group","gate bypassed under time pressure"],
  ["no 'high' label exists unless backed by >=2 sources in distinct independence groups; every illegal label is rejected with a reason"],
  ["legality thresholds change","an illegal label is found to have been recorded"],
  specialists=["evidence_analyst","evidence_governance_lead"]))

N.append(node("COST_OF_EVIDENCE","cost_of_acquisition_accounting",
  "Account for the cost and effort of acquiring further evidence per gap: access, time, and analyst load, expressed as the marginal cost of the next source against its expected effect on the tier.",
  "control", ["ACQUISITION_PLAN"], ["CQ_18"],
  b(0.76,0.74,0.68,0.58,0.7,0.72,0.5,0.66,0.5, 0.78,0.78, 0.76,0.66,0.18,0.42,[0.2,0.5],"the cost model or access constraints change"),
  ["marginal-cost-of-next-source estimate","expected-tier-gain estimate","budget tracking per gap"],
  ["the legality gate","the substantive grade"],
  [pro("Costing the next source against its expected tier impact keeps acquisition proportionate and feeds the stopping rule","not paying for a tenth same-group source that cannot legally lift the tier")],
  [con("Optimizing for cost can starve a high-stakes claim of the corroboration it genuinely needs","stopping cheaply on a safety-critical claim")],
  ["effort spent on sources that cannot change the tier","cost pressure starves a critical claim"],
  ["each gap tracks the marginal cost and expected tier impact of acquiring its next source"],
  ["cost model changes","a critical claim was under-evidenced to save cost"],
  specialists=["evidence_analyst","research_operations_lead"]))

N.append(node("STOPPING_RULE","acquisition_stopping_rule",
  "Decide when to stop acquiring evidence for a gap: stop when the target tier is reached and legal, when added independent sources stop changing the tier (saturation), or when marginal cost exceeds expected tier gain — recording WHY collection stopped.",
  "control", ["EVIDENCE_LABEL_LEGAL","COST_OF_EVIDENCE","CORROBORATION"], ["CQ_18","CQ_19"],
  b(0.88,0.8,0.78,0.66,0.84,0.84,0.62,0.6,0.6, 0.72,0.76, 0.88,0.6,0.22,0.58,[0.26,0.62],"the stopping-rule thresholds or saturation definition change"),
  ["target-tier-reached check","saturation detection","cost-vs-gain stop condition"],
  ["the answer to the question","provenance schema"],
  [pro("An explicit stopping rule prevents both premature closure and endless collection, and records the basis for stopping","stop at saturation once a third independent group adds no tier change")],
  [con("Any stopping threshold can stop too early on a hard claim or too late on an easy one","declaring saturation after two groups when a divergent third existed")],
  ["stopping before the legal target tier is reached","collecting past saturation with no tier change"],
  ["collection halts only on a recorded condition: legal target tier reached, saturation, or cost exceeding expected gain"],
  ["stopping thresholds change","a premature or runaway stop is found in audit"],
  specialists=["evidence_analyst","research_methodologist"]))

N.append(node("CONFIDENCE_DERIVE","claim_confidence_derivation",
  "Derive a claim's confidence as a function of its graded source set and the legality gate: confidence is the legal tier qualified by the live contradictions and disclosed revision conditions, never an assertion of truth.",
  "grading", ["EVIDENCE_LABEL_LEGAL","CORROBORATION"], ["CQ_17","CQ_20"],
  b(0.9,0.82,0.8,0.68,0.88,0.86,0.66,0.6,0.58, 0.72,0.76, 0.9,0.6,0.22,0.6,[0.26,0.62],"the confidence-derivation mapping or revision-condition policy changes"),
  ["tier-to-confidence mapping","contradiction-qualified confidence","disclosed revision conditions"],
  ["asserting the claim is true","collection mechanics"],
  [pro("Deriving confidence mechanically from the legal tier keeps the stated confidence and its evidential basis in lock-step and always defeasible","'moderate, revisable if the contradicting cohort study is corroborated' rather than a bare 'probably true'")],
  [con("A confidence number can be over-trusted as if it were a probability of truth rather than confidence in evidence","a 'high' read as 'certainly correct'")],
  ["confidence detached from the source set","confidence reported as truth, with no revision conditions"],
  ["each claim's confidence is a recorded function of its legal tier, its contradictions, and explicit revision conditions"],
  ["confidence mapping changes","a claim's confidence diverged from its underlying source set"],
  specialists=["evidence_analyst","grade_methodologist"]))

N.append(node("SOURCE_LEDGER","append_only_source_ledger",
  "Maintain an append-only ledger of every source, its type, independence group, provenance, all appraisals, the assigned tier, the legality decision, and the stopping basis — the single auditable record the whole method writes to and reads from.",
  "ledger", ["SOURCE_TYPING","PROVENANCE","GRADE_TIER","EVIDENCE_LABEL_LEGAL","CONFIDENCE_DERIVE","STOPPING_RULE"], ["CQ_08","CQ_20"],
  b(0.86,0.8,0.78,0.62,0.82,0.85,0.7,0.66,0.5, 0.8,0.82, 0.86,0.66,0.18,0.52,[0.2,0.5],"the ledger schema or append-only / immutability policy changes"),
  ["append-only source records","per-claim grade and legality entry","stopping-basis entry"],
  ["mutating prior judgments in place","the substantive answer"],
  [pro("One append-only ledger makes every grade, legality decision, and stop reproducible and revisable without losing the prior record","a downgrade appends a new entry citing a new contradiction, leaving the original judgment auditable")],
  [con("An append-only ledger grows without bound and needs disciplined indexing to stay usable","thousands of source entries across many gaps")],
  ["judgments overwritten so history is lost","ledger desynced from the actual appraisals"],
  ["every source and every grade/legality/stop decision is appended immutably with enough provenance to re-audit"],
  ["ledger schema changes","an in-place overwrite of a prior judgment is detected"],
  specialists=["evidence_analyst","evidence_governance_lead"]))

# ---------------- competency questions (exactly 14; gate caps at 14) ----------------
CQ = [
 ("CQ_01","For each knowledge gap, what evidence would close it and at what target tier, before any source is collected?",["nodes"],"ACQUISITION_PLAN defines the per-gap evidence requirement and target tier",["ACQUISITION_PLAN"]),
 ("CQ_02","How is each source typed as primary/secondary and empirical/testimonial, including mixed or interpretive sources?",["nodes"],"SOURCE_TYPING and PRIMARY_SECONDARY classify each source by epistemic type and resolve mixed sources",["SOURCE_TYPING","PRIMARY_SECONDARY"]),
 ("CQ_03","How are sources grouped by independence so corroboration is computed only across distinct groups?",["nodes"],"INDEPENDENCE_GROUP partitions sources into independence groups",["INDEPENDENCE_GROUP"]),
 ("CQ_04","How is circular reporting detected so apparent breadth is not mistaken for independent corroboration?",["nodes","edges"],"CITATION_CHAIN traces chains to primaries and flags circularity over the independence groups",["CITATION_CHAIN","INDEPENDENCE_GROUP"]),
 ("CQ_05","How is a claim triangulated across independent groups, and how are corroboration and contradiction tracked and kept visible?",["nodes","conflict_axes"],"TRIANGULATION converges across groups; CORROBORATION logs support and keeps contradiction explicit",["TRIANGULATION","CORROBORATION"]),
 ("CQ_06","How is provenance recorded and how does the append-only ledger keep every judgment auditable and revisable?",["nodes"],"PROVENANCE records origin/date/locator; SOURCE_LEDGER appends every judgment immutably",["PROVENANCE","SOURCE_LEDGER"]),
 ("CQ_07","How are freshness and authority appraised without conflating recency or fame with reliability?",["nodes","conflict_axes"],"FRESHNESS keys recency to volatility; AUTHORITY weights domain-relevant expertise over prominence",["FRESHNESS","AUTHORITY"]),
 ("CQ_08","How are source bias and conflicts of interest appraised and bounded so neither slant nor stake silently drives the grade?",["nodes"],"SOURCE_BIAS estimates distortion direction; CONFLICT_OF_INTEREST flags stakes and bounds tier lift",["SOURCE_BIAS","CONFLICT_OF_INTEREST"]),
 ("CQ_09","How are sampling/coverage bias and verifiability/reproducibility assessed for empirical sources?",["nodes"],"SAMPLING_BIAS checks representativeness and coverage; REPRODUCIBILITY checks verifiability and replication",["SAMPLING_BIAS","REPRODUCIBILITY"]),
 ("CQ_10","How are pivotal sources stress-tested for fabrication or coordinated disinformation without sliding into unfalsifiable rejection of all evidence?",["nodes","edge_cases"],"RED_TEAM_SOURCE checks pivotal corroboration against a fabrication/coordinated-origin threat model and bounds skepticism",["RED_TEAM_SOURCE"]),
 ("CQ_11","How is a GRADE-style evidence tier assigned to a claim from the full appraisal set?",["nodes","workflow"],"GRADE_TIER applies the rubric with explicit upgrade/downgrade reasoning",["GRADE_TIER"]),
 ("CQ_12","How is evidence-label legality enforced so a 'high' label is impossible without >=2 distinct-independence-group sources?",["nodes","dominance_rules"],"EVIDENCE_LABEL_LEGAL gates the tier against the distinct-group requirement; GRADE_TIER supplies the tier",["EVIDENCE_LABEL_LEGAL","GRADE_TIER"]),
 ("CQ_13","How is a claim's confidence derived from its legal tier and kept defeasible as a claim about evidence, not a claim of truth?",["nodes"],"CONFIDENCE_DERIVE maps the legal tier to confidence qualified by contradictions and disclosed revision conditions",["CONFIDENCE_DERIVE","EVIDENCE_LABEL_LEGAL"]),
 ("CQ_14","How is the cost of further acquisition accounted for and on what recorded basis does collection stop?",["nodes","iteration_protocol"],"COST_OF_EVIDENCE costs the next source; STOPPING_RULE halts on legal-tier/saturation/cost conditions with a recorded basis",["COST_OF_EVIDENCE","STOPPING_RULE"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

# consolidate node->CQ references onto the 14-CQ set (each node 1-2 refs; every CQ covered)
CQ_MAP = {
 "ACQUISITION_PLAN":["CQ_01"], "SOURCE_TYPING":["CQ_02"], "PRIMARY_SECONDARY":["CQ_02"],
 "INDEPENDENCE_GROUP":["CQ_03","CQ_04"], "CITATION_CHAIN":["CQ_04"], "TRIANGULATION":["CQ_05"],
 "CORROBORATION":["CQ_05"], "PROVENANCE":["CQ_06"], "FRESHNESS":["CQ_07"], "AUTHORITY":["CQ_07"],
 "SOURCE_BIAS":["CQ_08"], "CONFLICT_OF_INTEREST":["CQ_08"], "SAMPLING_BIAS":["CQ_09"],
 "REPRODUCIBILITY":["CQ_09"], "RED_TEAM_SOURCE":["CQ_10"], "GRADE_TIER":["CQ_11","CQ_12"],
 "EVIDENCE_LABEL_LEGAL":["CQ_12","CQ_13"], "COST_OF_EVIDENCE":["CQ_14"], "STOPPING_RULE":["CQ_14"],
 "CONFIDENCE_DERIVE":["CQ_13"], "SOURCE_LEDGER":["CQ_06"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

# ---------------- glossary ----------------
GL = [
 ("primary_source","a source reporting an original observation, study, measurement, or record at first hand",["original_source","first_hand_source"],["secondary_source"],["SOURCE_TYPING","PRIMARY_SECONDARY"]),
 ("secondary_source","a source that describes, summarizes, or interprets one or more primary sources",["derivative_source","review_source"],["primary_source"],["PRIMARY_SECONDARY","CITATION_CHAIN"]),
 ("empirical_source","a source whose claim rests on observation, measurement, or experiment rather than assertion",["observational_source","measured_source"],["testimonial_source"],["SOURCE_TYPING","REPRODUCIBILITY"]),
 ("testimonial_source","a source whose claim rests on assertion, opinion, or report rather than direct evidence",["assertion_source","opinion_source"],["empirical_source"],["SOURCE_TYPING","AUTHORITY"]),
 ("independence_group","a set of sources sharing an origin, dataset, author cluster, or funder; corroboration counts only across distinct groups",["independence_cluster","common_source_group"],["citation_chain"],["INDEPENDENCE_GROUP","TRIANGULATION"]),
 ("circular_reporting","apparent multi-source agreement that collapses to one origin because the sources all trace back to it",["echo_chamber_citation","source_laundering"],["independent_corroboration"],["CITATION_CHAIN","RED_TEAM_SOURCE"]),
 ("triangulation","strengthening a claim by convergence of distinct independence groups, ideally using different methods",["cross_validation","convergent_evidence"],["within_group_repetition"],["TRIANGULATION","CORROBORATION"]),
 ("provenance","the recorded origin, authorship, date, and chain of custody of a source",["lineage","chain_of_custody"],["citation_count"],["PROVENANCE","SOURCE_LEDGER"]),
 ("grade_tier","an ordinal GRADE-style label (high/moderate/low/very_low) summarizing confidence in a body of evidence, not a truth value",["evidence_tier","certainty_rating"],["truth_value"],["GRADE_TIER","EVIDENCE_LABEL_LEGAL"]),
 ("evidence_label_legality","the structural rule that a tier may be applied only if its preconditions hold (high requires >=2 distinct-group sources)",["label_legality_gate","tier_precondition_rule"],["substantive_grade"],["EVIDENCE_LABEL_LEGAL","INDEPENDENCE_GROUP"]),
 ("saturation","the point at which adding further independent sources no longer changes a claim's tier",["evidence_saturation","diminishing_returns_point"],["arbitrary_stop"],["STOPPING_RULE","CORROBORATION"]),
 ("defeasible_confidence","a stated confidence that remains revisable under disclosed conditions and is a claim about evidence, not about truth",["revisable_confidence","provisional_grade"],["assertion_of_truth"],["CONFIDENCE_DERIVE","SOURCE_LEDGER"]),
]
GLS=[{"term":t,"definition":d,"synonyms":s,"not_same_as":ns,"used_by_nodes":u} for (t,d,s,ns,u) in GL]

# ---------------- edges ----------------
E=[]
def dep(f,t,rs,cc=0.82,erc=0.28,cp=0.14,why="",ben="",rk="",ex=""):
    E.append({"from":f,"to":t,"edge_type":"dependency","relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{t} depends on {f}","benefit_of_coupling":ben or "ordered prerequisite",
              "risk_of_conflict":rk or "downstream rework if upstream changes","example":ex or f"{f} settled before {t}"})
def conf(f,t,rs,st,rule,why,cp=0.5,erc=0.5,cc=0.6):
    E.append({"from":f,"to":t,"edge_type":"conflict","relation_strength":rs,"signed_tension":st,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "resolution_rule":rule,"why_related":why,"benefit_of_coupling":"tension surfaced and resolved by rule",
              "risk_of_conflict":"unmanaged tension degrades evidence quality","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated appraisal",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies)
dep("ACQUISITION_PLAN","SOURCE_TYPING",0.86)
dep("SOURCE_TYPING","PRIMARY_SECONDARY",0.84)
dep("SOURCE_TYPING","INDEPENDENCE_GROUP",0.8)
dep("INDEPENDENCE_GROUP","CITATION_CHAIN",0.82)
dep("INDEPENDENCE_GROUP","TRIANGULATION",0.85)
dep("TRIANGULATION","CORROBORATION",0.84)
dep("SOURCE_TYPING","PROVENANCE",0.78)
dep("PROVENANCE","FRESHNESS",0.8)
dep("PROVENANCE","AUTHORITY",0.74)
dep("SOURCE_TYPING","SOURCE_BIAS",0.76)
dep("SOURCE_BIAS","CONFLICT_OF_INTEREST",0.74)
dep("PRIMARY_SECONDARY","SAMPLING_BIAS",0.76)
dep("PRIMARY_SECONDARY","REPRODUCIBILITY",0.78)
dep("SOURCE_BIAS","RED_TEAM_SOURCE",0.74)
dep("CORROBORATION","GRADE_TIER",0.84)
dep("REPRODUCIBILITY","GRADE_TIER",0.76)
dep("RED_TEAM_SOURCE","GRADE_TIER",0.78)
dep("GRADE_TIER","EVIDENCE_LABEL_LEGAL",0.88)
dep("INDEPENDENCE_GROUP","EVIDENCE_LABEL_LEGAL",0.82)
dep("ACQUISITION_PLAN","COST_OF_EVIDENCE",0.74)
dep("EVIDENCE_LABEL_LEGAL","STOPPING_RULE",0.82)
dep("COST_OF_EVIDENCE","STOPPING_RULE",0.78)
dep("EVIDENCE_LABEL_LEGAL","CONFIDENCE_DERIVE",0.84)
dep("GRADE_TIER","SOURCE_LEDGER",0.78)
dep("CONFIDENCE_DERIVE","SOURCE_LEDGER",0.76)
dep("PROVENANCE","SOURCE_LEDGER",0.78)
# cross-cutting non-dependency edges
rel("CITATION_CHAIN","RED_TEAM_SOURCE","similarity",0.74,why="circular-reporting traces are a primary input to the disinformation threat model")
rel("STOPPING_RULE","ACQUISITION_PLAN","feedback",0.74,why="a stop short of target tier feeds back to re-plan or re-scope the gap")
rel("CONFIDENCE_DERIVE","ACQUISITION_PLAN","feedback",0.72,why="a derived confidence below target reopens the gap for more acquisition")
rel("SAMPLING_BIAS","RED_TEAM_SOURCE","causal",0.7,why="coverage gaps are where coordinated planted evidence most easily hides")
rel("FRESHNESS","STOPPING_RULE","constraint",0.72,why="a claim's volatility sets how quickly a stopped grade must be revisited")
rel("AUTHORITY","INDEPENDENCE_GROUP","constraint",0.72,why="shared authoritative origins collapse otherwise-distinct-looking sources into one group")
rel("SOURCE_LEDGER","CONFIDENCE_DERIVE","feedback",0.7,why="appended contradictions in the ledger trigger re-derivation of a claim's confidence")

# conflict edges (negative signed_tension + resolution_rule) — real tensions
conf("ACQUISITION_PLAN","STOPPING_RULE",0.72,-0.6,
  "acquire to the legal target tier, then stop: when saturation or the cost-vs-gain bound is hit, halt even if more thoroughness is conceivable, recording the basis",
  "acquisition thoroughness pulls toward more sources while cost and the stopping rule pull toward halting", cp=0.55, erc=0.55)
conf("AUTHORITY","INDEPENDENCE_GROUP",0.7,-0.58,
  "never let authority weight substitute for independence: a single eminent source cannot satisfy the >=2-distinct-group requirement no matter how authoritative",
  "weighting a famous authority highly conflicts with the independence requirement that bars deferring to one source", cp=0.5, erc=0.5)
conf("FRESHNESS","REPRODUCIBILITY",0.68,-0.5,
  "key recency to volatility: prefer fresher sources only for volatile claims, and do not discount a replicated established finding merely for being older than a fresh un-replicated one",
  "freshness pressure to prefer the newest source conflicts with the reliability of an older but well-replicated established source", cp=0.45, erc=0.45)
conf("EVIDENCE_LABEL_LEGAL","CONFIDENCE_DERIVE",0.72,-0.55,
  "hold the legality line: when independent corroboration is missing, cap the legal tier below high and report the lower defeasible confidence rather than relaxing the gate to answer under thin evidence",
  "strict high-label legality conflicts with the pressure to state a high confidence and answer when evidence is thin", cp=0.55, erc=0.55)

# ---------------- conflict_axes (9) ----------------
CA=[
 {"name":"thoroughness_vs_cost_and_stopping","description":"Acquiring more independent sources raises confidence but costs time and access; the stopping rule must halt before effort is wasted on sources that cannot change the tier.","poles":["maximal_thoroughness","bounded_cost"],"resolution_hint":"stop at the legal target tier or saturation; cost the next source against expected tier gain","tension_score":0.72,"affected_nodes":["ACQUISITION_PLAN","COST_OF_EVIDENCE","STOPPING_RULE"]},
 {"name":"authority_weight_vs_independence","description":"Weighting an eminent source heavily can substitute deference to one famous voice for genuine independent corroboration.","poles":["authority_deference","independence_breadth"],"resolution_hint":"authority adjusts a source's weight but never satisfies the >=2-distinct-group requirement","tension_score":0.74,"affected_nodes":["AUTHORITY","INDEPENDENCE_GROUP","EVIDENCE_LABEL_LEGAL"]},
 {"name":"freshness_vs_established_reliability","description":"Preferring the newest source can displace an older but well-replicated established finding that remains more reliable.","poles":["prefer_freshest","prefer_replicated"],"resolution_hint":"key freshness to claim volatility; do not discount replicated durable findings for age alone","tension_score":0.62,"affected_nodes":["FRESHNESS","REPRODUCIBILITY","AUTHORITY"]},
 {"name":"high_label_strictness_vs_answering_thin_evidence","description":"Strict high-label legality blocks confident labels without independent corroboration, which can leave a question answered only at a lower tier under time pressure.","poles":["strict_legality","answer_under_thin_evidence"],"resolution_hint":"hold the gate; report the lower legal tier with disclosed revision conditions rather than relaxing the rule","tension_score":0.7,"affected_nodes":["EVIDENCE_LABEL_LEGAL","CONFIDENCE_DERIVE","GRADE_TIER"]},
 {"name":"corroboration_count_vs_quality_weighting","description":"Counting corroborating sources is simple but can vote-count weak sources over a strong contradiction.","poles":["source_count","quality_weighting"],"resolution_hint":"weigh sources by appraisal quality and independence, not raw count; unresolved contradiction caps the tier","tension_score":0.66,"affected_nodes":["CORROBORATION","TRIANGULATION","GRADE_TIER"]},
 {"name":"adversarial_skepticism_vs_usability","description":"A hard disinformation threat model catches engineered corroboration but, taken too far, becomes unfalsifiable and rejects all evidence.","poles":["max_skepticism","tractable_appraisal"],"resolution_hint":"apply the red-team check to pivotal sources only and require a concrete fabrication mechanism, not mere suspicion","tension_score":0.64,"affected_nodes":["RED_TEAM_SOURCE","INDEPENDENCE_GROUP","GRADE_TIER"]},
 {"name":"early_typing_vs_collection_bias","description":"Specifying the desired evidence shape up front bounds search but can bias collection toward confirming sources.","poles":["targeted_search","unbiased_collection"],"resolution_hint":"plan the evidence kind and tier, not the desired conclusion; keep the claim separate from a preferred answer","tension_score":0.6,"affected_nodes":["ACQUISITION_PLAN","SOURCE_TYPING","SAMPLING_BIAS"]},
 {"name":"provenance_completeness_vs_overhead","description":"Full provenance and an append-only ledger enable audit and revision but cost capture effort and unbounded growth.","poles":["full_provenance","minimal_overhead"],"resolution_hint":"capture origin/date/locator and every decision; index and summarize rather than discard","tension_score":0.55,"affected_nodes":["PROVENANCE","SOURCE_LEDGER","CONFLICT_OF_INTEREST"]},
 {"name":"conflict_of_interest_flag_vs_dismissal","description":"Flagging a conflict of interest is a risk indicator, but treating conflicted sources as automatically false discards potentially correct evidence.","poles":["coi_caution","coi_dismissal"],"resolution_hint":"bound a conflicted source's tier lift rather than excluding it; require independent corroboration to lift the tier","tension_score":0.58,"affected_nodes":["CONFLICT_OF_INTEREST","SOURCE_BIAS","GRADE_TIER"]},
]

# ---------------- edge_cases (12) ----------------
EC=[
 {"description":"Three outlets all reprint one wire story and are counted as three independent confirmations.","trigger":"sources sharing one origin assigned to distinct independence groups","affected_nodes":["INDEPENDENCE_GROUP","CITATION_CHAIN","TRIANGULATION"],"mitigation":"trace citation chains to the common origin and collapse the shared-origin sources into one group","severity":"critical"},
 {"description":"A 'high' label is recorded with two sources that share a single independence group.","trigger":"label-legality gate not enforced against the distinct-group requirement","affected_nodes":["EVIDENCE_LABEL_LEGAL","INDEPENDENCE_GROUP","GRADE_TIER"],"mitigation":"reject the label as illegal; require >=2 sources in distinct independence groups before allowing 'high'","severity":"critical"},
 {"description":"A swarm of fabricated 'independent' reviews all trace to one coordinated operation and are taken as corroboration.","trigger":"pivotal corroboration accepted without an adversarial origin check","affected_nodes":["RED_TEAM_SOURCE","INDEPENDENCE_GROUP","CITATION_CHAIN"],"mitigation":"run the fabrication/coordinated-origin threat model on pivotal corroboration before it lifts a tier","severity":"high"},
 {"description":"An eminent expert's lone assertion is graded 'high' on the strength of their reputation.","trigger":"authority weight allowed to satisfy the independence requirement","affected_nodes":["AUTHORITY","INDEPENDENCE_GROUP","EVIDENCE_LABEL_LEGAL"],"mitigation":"authority adjusts weight only; a single source cannot earn 'high' regardless of authority","severity":"high"},
 {"description":"A credible contradicting study is averaged away so the claim reads as agreed.","trigger":"contradiction merged into a net score instead of kept explicit","affected_nodes":["CORROBORATION","TRIANGULATION","GRADE_TIER"],"mitigation":"keep contradictions explicit; unresolved contradiction caps the tier below high","severity":"high"},
 {"description":"A two-year-old figure is used for a fast-moving claim as if still current.","trigger":"freshness not keyed to the claim's volatility","affected_nodes":["FRESHNESS","PROVENANCE","STOPPING_RULE"],"mitigation":"estimate claim volatility and flag supersession; require a fresh source for volatile claims","severity":"medium"},
 {"description":"A replicated decades-old established finding is discarded only for being older than a fresh un-replicated preprint.","trigger":"recency mistaken for reliability","affected_nodes":["FRESHNESS","REPRODUCIBILITY","AUTHORITY"],"mitigation":"do not discount durable replicated findings for age; weigh replication over mere recency","severity":"medium"},
 {"description":"A manufacturer-funded study of its own product silently lifts a claim to 'high'.","trigger":"conflict of interest not flagged or not bounded in tier influence","affected_nodes":["CONFLICT_OF_INTEREST","SOURCE_BIAS","GRADE_TIER"],"mitigation":"flag the conflict and bound its tier lift; require independent corroboration to reach 'high'","severity":"high"},
 {"description":"A benefit seen only in published positive trials is generalized as if the whole evidence base agreed.","trigger":"publication/sampling bias and search-coverage gaps not appraised","affected_nodes":["SAMPLING_BIAS","REPRODUCIBILITY","GRADE_TIER"],"mitigation":"appraise representativeness and search coverage; consider unpublished and null results","severity":"high"},
 {"description":"An unverifiable one-off assertion is graded as if it were a confirmed result.","trigger":"verifiability/reproducibility not assessed before grading","affected_nodes":["REPRODUCIBILITY","RED_TEAM_SOURCE","GRADE_TIER"],"mitigation":"record verifiability and replication status; treat the absence of either as a downgrade, not a disproof","severity":"medium"},
 {"description":"Collection continues past saturation, paying for same-group sources that cannot legally change the tier.","trigger":"stopping rule not applied; cost-vs-gain ignored","affected_nodes":["STOPPING_RULE","COST_OF_EVIDENCE","INDEPENDENCE_GROUP"],"mitigation":"detect saturation and stop when the next source cannot lift the legal tier","severity":"low"},
 {"description":"A claim's stated confidence is read as a probability of truth and a contradicting source later surfaces with no recorded revision path.","trigger":"confidence detached from its source set and from disclosed revision conditions","affected_nodes":["CONFIDENCE_DERIVE","SOURCE_LEDGER","CORROBORATION"],"mitigation":"derive confidence from the legal tier with explicit revision conditions; append revisions to the ledger","severity":"medium"},
]

# ---------------- workflow (12) ----------------
WF=[
 {"action":"plan_acquisition","node_ref":"ACQUISITION_PLAN","description":"For each open gap, fix the target claim, the source kinds that could bear on it, and the target evidence tier before collecting.","artifact":"acquisition_plan","gate":"every active gap has a target claim and target tier"},
 {"action":"type_sources","node_ref":"SOURCE_TYPING","description":"Type each collected source as primary/secondary and empirical/testimonial.","artifact":"typed_source_set","gate":"every source has a resolved type"},
 {"action":"group_by_independence","node_ref":"INDEPENDENCE_GROUP","description":"Assign each source an independence group by origin, dataset, author cluster, and funder.","artifact":"independence_grouping","gate":"every source is assigned an independence group"},
 {"action":"trace_citation_chains","node_ref":"CITATION_CHAIN","description":"Trace chains toward primaries and flag circular reporting and shared origins.","artifact":"chain_trace","gate":"no corroborating set is left with an untraced shared origin"},
 {"action":"triangulate_and_track","node_ref":"TRIANGULATION","description":"Triangulate across distinct groups and record corroboration and contradiction per claim.","artifact":"triangulation_record","gate":"convergence is computed across distinct groups; contradictions logged"},
 {"action":"record_provenance","node_ref":"PROVENANCE","description":"Record origin, author, date, and retrieval locator for every source.","artifact":"provenance_record","gate":"every source is re-locatable from its provenance"},
 {"action":"appraise_quality","node_ref":"AUTHORITY","description":"Appraise authority, freshness, bias, conflicts of interest, sampling, and reproducibility per source.","artifact":"quality_appraisal","gate":"each source carries the full appraisal set"},
 {"action":"red_team_pivotal","node_ref":"RED_TEAM_SOURCE","description":"Stress pivotal corroboration against a fabrication/coordinated-origin threat model.","artifact":"adversarial_review","gate":"pivotal corroboration passes the threat model or is flagged"},
 {"action":"assign_tier","node_ref":"GRADE_TIER","description":"Assign a GRADE-style tier per claim with explicit upgrade/downgrade reasoning.","artifact":"graded_claim","gate":"every claim has a tier with recorded reasons"},
 {"action":"enforce_legality","node_ref":"EVIDENCE_LABEL_LEGAL","description":"Reject any tier whose structural preconditions fail; 'high' needs >=2 distinct-group sources.","artifact":"legality_decision","gate":"no illegal label is recorded"},
 {"action":"derive_confidence_and_stop","node_ref":"STOPPING_RULE","description":"Derive defeasible confidence from the legal tier and stop on a recorded condition.","artifact":"confidence_and_stop_record","gate":"confidence is derived and a stop basis is recorded"},
 {"action":"append_to_ledger","node_ref":"SOURCE_LEDGER","description":"Append every source, appraisal, tier, legality decision, confidence, and stop basis immutably.","artifact":"source_ledger_entry","gate":"the ledger contains an immutable, auditable entry for every judgment"},
]

# ---------------- dominance_rules (9) ----------------
DR=[
 {"rule":"A 'high' evidence label must never be recorded without >=2 sources in distinct independence groups","rationale":"high confidence requires genuine independent corroboration, not repetition of one origin","trigger":"a high label is proposed with sources sharing one independence group","action":"reject the label as illegal and cap at the highest legal tier"},
 {"rule":"INDEPENDENCE_GROUP and CITATION_CHAIN must be settled before TRIANGULATION or the legality gate runs","rationale":"corroboration and the distinct-group count are only meaningful once independence and circularity are resolved","trigger":"triangulation or legality run on ungrouped or untraced sources","action":"block until grouping and chain tracing are complete"},
 {"rule":"Authority weight may adjust a source's weight but must never substitute for the independence requirement","rationale":"deferring to one famous source is not independent corroboration","trigger":"a lone authoritative source proposed as sufficient for 'high'","action":"require a second distinct independence group regardless of authority"},
 {"rule":"Unresolved credible contradiction caps a claim's tier below high","rationale":"genuine independent disagreement is incompatible with high confidence","trigger":"a credible contradicting source in a distinct group remains unresolved","action":"cap the tier and record the live contradiction"},
 {"rule":"Conflicts of interest must be flagged before a conflicted source can lift a tier","rationale":"a stakeholder's self-serving evidence must not silently inflate confidence","trigger":"a conflicted source proposed as tier-lifting without a flag","action":"flag the conflict and require independent corroboration to lift the tier"},
 {"rule":"Freshness must be keyed to claim volatility, not applied as a blanket preference for the newest source","rationale":"recency is not reliability; replicated durable findings should not be discounted for age","trigger":"a fresh un-replicated source preferred over a replicated established one on a stable claim","action":"weigh replication and volatility, not recency alone"},
 {"rule":"Pivotal corroboration must pass the adversarial origin check before lifting a tier","rationale":"engineered corroboration can fake independence","trigger":"a pivotal corroboration would lift a tier without a red-team pass","action":"require the fabrication/coordinated-origin check first"},
 {"rule":"Acquisition must stop on a recorded condition (legal target tier, saturation, or cost-exceeds-gain), never silently","rationale":"both premature closure and endless collection are failures; the basis must be auditable","trigger":"collection halts or continues with no recorded stop basis","action":"record the stop condition or continue to the legal target tier"},
 {"rule":"Every grade, legality decision, confidence, and stop basis must be appended to the ledger immutably","rationale":"reproducibility and revision require an auditable, non-overwritten record","trigger":"a prior judgment overwritten in place","action":"append a new entry citing the change rather than mutating the old one"},
]

# ---------------- anti_rework_rules (8) ----------------
ARR=[
 {"rule":"Do not grade or label before sources are grouped by independence and chains are traced; regrading after a shared origin surfaces is wasteful","prevents":"re-grading every claim after circular reporting is discovered late"},
 {"rule":"Do not propose a 'high' label without confirming >=2 distinct independence groups first; a rejected illegal label discards the grading work","prevents":"rebuilding a grade after the legality gate rejects an illegal high label"},
 {"rule":"Do not record judgments without provenance; retrofitting origin and date for audit requires re-collecting sources","prevents":"re-collection when a judgment cannot be traced to its source"},
 {"rule":"Do not defer the conflict-of-interest and bias appraisal to grading time; a late COI flag can invalidate an already-recorded tier","prevents":"re-grading after a conflict surfaces post-tier"},
 {"rule":"Do not skip the adversarial check on pivotal corroboration; discovering engineered corroboration after grading forces a full re-appraisal","prevents":"re-appraising a claim after planted evidence is found post-grade"},
 {"rule":"Do not collect past saturation; sources that cannot change the legal tier add cost without value","prevents":"wasted acquisition on tier-neutral same-group sources"},
 {"rule":"Do not detach confidence from the source set; a confidence stated without revision conditions must be re-derived when evidence changes","prevents":"reconstructing the basis for a confidence when a contradiction appears"},
 {"rule":"Do not overwrite prior ledger judgments; reconstructing lost history to explain a grade requires re-running the appraisal","prevents":"re-running appraisals to recover an overwritten judgment trail"},
]

# ---------------- iteration_protocol (8) ----------------
IP=[
 {"trigger":"circular reporting or a shared origin is discovered after grading","action":"re-trace chains in CITATION_CHAIN, re-cluster in INDEPENDENCE_GROUP, and re-run the legality gate for affected claims","nodes":["CITATION_CHAIN","INDEPENDENCE_GROUP","EVIDENCE_LABEL_LEGAL"],"priority":"critical"},
 {"trigger":"an illegal 'high' label is found in the ledger","action":"reject and cap it via EVIDENCE_LABEL_LEGAL and append the correction in SOURCE_LEDGER","nodes":["EVIDENCE_LABEL_LEGAL","SOURCE_LEDGER","GRADE_TIER"],"priority":"critical"},
 {"trigger":"engineered or coordinated corroboration is detected post-grade","action":"re-run RED_TEAM_SOURCE on pivotal sources and re-grade the affected claims","nodes":["RED_TEAM_SOURCE","GRADE_TIER","INDEPENDENCE_GROUP"],"priority":"high"},
 {"trigger":"a credible contradiction surfaces in a distinct independence group","action":"record it in CORROBORATION, cap the tier, and re-derive confidence","nodes":["CORROBORATION","GRADE_TIER","CONFIDENCE_DERIVE"],"priority":"high"},
 {"trigger":"an undisclosed conflict of interest is discovered for a tier-lifting source","action":"flag it in CONFLICT_OF_INTEREST, bound its influence, and re-grade","nodes":["CONFLICT_OF_INTEREST","SOURCE_BIAS","GRADE_TIER"],"priority":"high"},
 {"trigger":"a graded claim's underlying facts have plausibly changed (volatility window elapsed)","action":"re-assess recency in FRESHNESS and re-open the gap via STOPPING_RULE if the tier may have shifted","nodes":["FRESHNESS","STOPPING_RULE","ACQUISITION_PLAN"],"priority":"medium"},
 {"trigger":"a search-coverage gap or sampling bias is found after grading","action":"re-appraise representativeness in SAMPLING_BIAS and re-grade if coverage changed the evidence base","nodes":["SAMPLING_BIAS","REPRODUCIBILITY","GRADE_TIER"],"priority":"medium"},
 {"trigger":"acquisition cost is overrunning with no tier change","action":"recompute marginal cost-vs-gain in COST_OF_EVIDENCE and apply the saturation stop in STOPPING_RULE","nodes":["COST_OF_EVIDENCE","STOPPING_RULE","CORROBORATION"],"priority":"medium"},
]

# ---------------- spec assembly ----------------
spec = {
 "domain":"acq__evidence_sourcing",
 "domain_label":"Evidence Acquisition & Source Quality (Knowledge Acquisition subdomain)",
 "purpose":"session_bounded_method_for_acquiring_and_grading_evidence_to_close_knowledge_gaps_by_typing_sources_as_primary_or_secondary_and_empirical_or_testimonial_grouping_them_by_independence_tracing_citation_chains_to_detect_circular_reporting_triangulating_across_independent_groups_tracking_provenance_and_freshness_appraising_authority_bias_conflicts_of_interest_sampling_and_reproducibility_running_an_adversarial_disinformation_check_assigning_a_grade_style_evidence_tier_enforcing_evidence_label_legality_so_a_high_label_requires_at_least_two_sources_in_distinct_independence_groups_deriving_a_defeasible_claim_confidence_and_deciding_on_a_recorded_basis_when_to_stop_all_written_to_an_append_only_source_ledger_neutrally_assessing_evidence_quality_without_asserting_domain_conclusions",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "scope is the acquisition-and-grading method itself; the substantive answer to the question is out of scope and lives in the composed domain specialists",
   "evidence labels are GRADE/CERQual-style claims about confidence in a body of evidence, not assertions of ground truth, and are always defeasible",
   "candidate sources and the knowledge gaps (competency questions) are available as inputs to the method",
 ],
 "exclusions":[
   "asserting domain conclusions or making decisions, professional advice, or recommendations",
   "generating new domain knowledge rather than appraising existing sources",
   "search/retrieval engineering and crawling infrastructure (assumed upstream)",
   "ground-truth verification; the method grades confidence in evidence, not truth",
 ],
 "source_description":"heuristic prior estimates for evidence-acquisition and source-quality work units, informed by GRADE evidence grading, library/source-evaluation rubrics (CRAAP), intelligence-analysis source-independence discipline, and systematic-review methodology; no supplied dataset",
 "source_citation":"GRADE Working Group: Guyatt et al. 2008 (BMJ, 'GRADE: an emerging consensus on rating quality of evidence'); Lewis & Wilson on source evaluation and the CRAAP test (California State University, Meriam Library) cf. Patrick Wilson 1983 'Second-Hand Knowledge: An Inquiry into Cognitive Authority'; Richards J. Heuer Jr. 1999 'Psychology of Intelligence Analysis' (source independence and circular reporting); Higgins & Green (eds.) Cochrane Handbook for Systematic Reviews of Interventions; Lewis & Clark / CERQual (Lewin et al. 2015) for qualitative-evidence confidence",
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
 "priority_rationale":"ACQUISITION_PLAN and SOURCE_TYPING are foundational; INDEPENDENCE_GROUP and CITATION_CHAIN establish the independence basis that makes corroboration meaningful; the appraisal layer (freshness, authority, bias, COI, sampling, reproducibility, red-team) feeds GRADE_TIER; EVIDENCE_LABEL_LEGAL gates the tier; CONFIDENCE_DERIVE, COST_OF_EVIDENCE and STOPPING_RULE close the method; SOURCE_LEDGER records everything.",
 "eval_objective":"verify_source_typing_independence_grouping_circular_reporting_detection_triangulation_grade_tiering_evidence_label_legality_and_stopping_of_acq__evidence_sourcing_kb",
}

out_dir = "branches/b11_knowledge_acquisition/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "acq__evidence_sourcing.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
