#!/usr/bin/env python3
"""Generate the novelty__derivative_vs_copy content spec (B60 KB) for kb_forge.py.

Domain: Novelty, Transformation & Derivative-Risk Flagging for generated content.
Core scope finding (HARD REQUIREMENT): "modify / transform / add own element" is a
CREATIVITY lever that raises novelty and reduces (but does NOT eliminate) the chance
of producing a near-copy. Transformation is NOT a legal safe harbor. Legal sufficiency
(substantial-similarity / fair-use determinations) is routed to compliance_heavy
(scope FP6 / KU8), never decided here; this KB only FLAGS derivative risk and supplies
the provenance compliance needs.

Grounded in:
  - Boden 1990/2004, "The Creative Mind: Myths and Mechanisms" (combinational,
    exploratory, and transformational creativity)
  - Lessig 2008, "Remix: Making Art and Commerce Thrive in the Hybrid Economy"
  - Genette 1982, "Palimpsestes" (pastiche = non-satirical stylistic imitation vs
    parody = transformation-with-commentary of a specific target)
  - Runco & Jaeger 2012, "The Standard Definition of Creativity", Creativity Research
    Journal 24(1): creativity = novelty (originality) AND appropriateness (usefulness/fit)

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
        "academic_fields": ["computational_creativity", "content_intelligence"],
        "subfields": subfields or ["novelty_assessment", "derivative_risk_flagging"],
        "specialists": specialists or ["creative_strategist"],
        "contradictors": contradictors or ["transformation_is_safe_harbor_advocate"],
        "inputs": inputs or ["candidate output", "source material reference", "prior output history"],
        "outputs": outputs or ["novelty score with derivative-risk flag and provenance"],
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

N.append(node("ORIGINALITY_DEFINITION","standard_definition_of_creativity",
  "Fix the operating definition of an original output as BOTH novel (different from what exists) AND appropriate (fitting the brief/audience/context); neither half alone qualifies. This is the yardstick every downstream novelty judgment is measured against.",
  "foundations", [], ["CQ_01"],
  b(0.9,0.82,0.85,0.5,0.82,0.8,0.6,0.7,0.42, 0.78,0.82, 0.9,0.72,0.16,0.5,[0.18,0.46],"definition of originality or appropriateness criterion revised"),
  ["novelty axis","appropriateness axis","two-pronged originality criterion"],
  ["legal originality threshold (delegated to compliance)","aesthetic quality grading"],
  [pro("A two-pronged definition stops 'random = creative': a novel but off-brief output is rejected, not praised","a surreal tagline that ignores the product is novel but inappropriate, so not original")],
  [con("Appropriateness is context-relative, so the same output can be original in one brief and derivative in another","reusing a meme format is fresh for a niche but stale for a mainstream audience")],
  ["scoring novelty alone and calling it creativity","conflating 'appropriate' with 'safe/legal'"],
  ["every novelty judgment cites both a novelty term and an appropriateness term"],
  ["originality definition revised","appropriateness rubric for a new brief class added"],
  specialists=["creative_strategist","content_evaluator"]))

N.append(node("SOURCE_INVENTORY","source_material_identification",
  "Enumerate the concrete source materials the output draws on (referenced works, prompts, exemplars, style targets) so that borrowing can be measured and attributed rather than assumed absent.",
  "foundations", [], ["CQ_02"],
  b(0.85,0.76,0.72,0.55,0.8,0.78,0.6,0.66,0.44, 0.76,0.8, 0.84,0.66,0.18,0.5,[0.2,0.5],"a new source class (e.g. live web retrieval) enters the pipeline"),
  ["enumerated source list","per-source role (style target vs content source)","retrieval/prompt provenance"],
  ["legal ownership determination","license interpretation (delegated to compliance)"],
  [pro("Naming sources up front turns 'is this derivative?' from a guess into a measurable distance from named items","listing the one viral post a draft was 'inspired by' lets distance be computed against it")],
  [con("Hidden or laundered sources (style absorbed via training) may never appear in the inventory","an output copies a living artist's signature style with no explicit source reference")],
  ["unlisted source that the output actually copies","treating absence-of-citation as absence-of-borrowing"],
  ["every claimed-original output has an explicit (possibly empty, justified) source inventory"],
  ["new source channel added","an uninventoried source surfaces in review"],
  specialists=["creative_strategist","provenance_analyst"]))

N.append(node("COMBINATIONAL_CREATIVITY","novelty_from_recombination",
  "Boden's first creativity mode: generate novelty by making unfamiliar combinations of familiar elements (concepts, references, formats), where each element is known but the juxtaposition is not.",
  "foundations", ["ORIGINALITY_DEFINITION"], ["CQ_03"],
  b(0.84,0.8,0.8,0.58,0.74,0.78,0.55,0.66,0.46, 0.74,0.78, 0.84,0.66,0.2,0.48,[0.22,0.52],"the combination-space or element vocabulary changes"),
  ["unfamiliar combination of familiar elements","cross-domain juxtaposition","analogy/blend construction"],
  ["inventing genuinely new primitive elements (that is transformational, not combinational)"],
  [pro("Recombination is the highest-yield, lowest-risk novelty lever: known parts, new whole, low copy risk per part","pairing a noir voiceover with a cooking tutorial is novel yet copies neither source wholesale")],
  [con("A combination can still be a near-copy if one element dominates and is itself protected expression","'recombining' is a fig leaf if 95% is one source plus a trivial tweak")],
  ["combination dominated by a single source element","arbitrary mash-up that is novel but inappropriate"],
  ["the output recombines >=2 distinct elements with no single element exceeding a dominance threshold"],
  ["element vocabulary expanded","combinations observed to collapse onto one source"]))

N.append(node("EXPLORATORY_CREATIVITY","exploration_within_a_style_space",
  "Boden's second mode: generate novelty by exploring the unfilled regions of an accepted conceptual/stylistic space (a genre, a format, a brand system) without breaking its rules.",
  "structure", ["COMBINATIONAL_CREATIVITY"], ["CQ_03","CQ_04"],
  b(0.78,0.74,0.74,0.6,0.7,0.74,0.5,0.66,0.45, 0.74,0.76, 0.78,0.66,0.2,0.46,[0.24,0.54],"the bounded style space or its rules change"),
  ["search of unfilled regions in a fixed space","within-genre variation","rule-respecting extrapolation"],
  ["redefining the space itself (transformational)","cross-space recombination (combinational)"],
  [pro("Exploration yields on-brand novelty: new instances that obey the brand rules, maximizing appropriateness","a new ad in an established campaign that fans recognize yet have not seen")],
  [con("Exhaustive exploration of a small space hits diminishing novelty fast as regions fill up","the tenth variation on the same format reads as a rerun")],
  ["mistaking exploration for transformation (the rules were never broken)","over-exploring a near-exhausted space"],
  ["new instances stay inside the declared style rules while differing from prior instances in the space"],
  ["style space redefined","novelty yield per new instance drops below threshold"]))

N.append(node("TRANSFORMATIONAL_CREATIVITY","altering_the_style_space_itself",
  "Boden's third and rarest mode: generate novelty by altering or dropping a defining constraint of the space itself, producing outputs the unmodified space could not contain.",
  "structure", ["EXPLORATORY_CREATIVITY"], ["CQ_04"],
  b(0.82,0.78,0.76,0.7,0.78,0.8,0.6,0.58,0.55, 0.7,0.74, 0.82,0.58,0.24,0.55,[0.3,0.66],"the set of space-defining constraints is reconsidered"),
  ["dropping/altering a defining constraint","creating outputs the old space could not hold","paradigm-level shift"],
  ["incremental within-space variation (exploratory)","copying another space's existing rules"],
  [pro("Transformation produces the most defensible, highest-novelty outputs because the result is structurally unlike the source","removing the 'single narrator' rule of a format opens an output class no prior instance occupies")],
  [con("Transformation is high-risk and often inappropriate: breaking the rule can also break the brief or audience fit","dropping a brand's core constraint yields novelty the brand cannot use")],
  ["breaking a constraint that was load-bearing for appropriateness","claiming transformation while merely re-skinning a copy"],
  ["a named space-defining constraint is explicitly altered/dropped and the result is verified appropriate to the brief"],
  ["space-defining constraint set changes","a transformation breaks audience fit in testing"],
  specialists=["creative_strategist","creative_director"]))

N.append(node("TRANSFORM_NOT_COPY","transform_modify_add_lever",
  "The core operating directive: do not reproduce a source; instead modify it and add an own element so the output is structurally and expressively distinct. This raises novelty and lowers near-copy probability, but is a CREATIVE measure, not a legal guarantee.",
  "structure", ["COMBINATIONAL_CREATIVITY","SOURCE_INVENTORY"], ["CQ_05","CQ_06"],
  b(0.92,0.85,0.84,0.62,0.88,0.85,0.66,0.6,0.6, 0.72,0.76, 0.92,0.6,0.24,0.6,[0.3,0.66],"the transform/add directive or its measurable thresholds change"),
  ["modify the source rather than reproduce it","add a distinguishing own element","measurable departure from source"],
  ["asserting legal sufficiency of the transformation (delegated to compliance_heavy)","license clearance"],
  [pro("Modify-and-add is the single highest-leverage move for both novelty and copy-risk reduction","rewriting a borrowed structure and inserting an original framing device departs measurably from the source")],
  [con("Transformation degree is a spectrum, and a small tweak to heavily-copied expression is still a near-copy that this lever cannot launder","find-and-replace on a protected text 'modifies' it but remains a copy")],
  ["treating any modification as sufficient","adding a cosmetic element while the core remains copied"],
  ["the output measurably departs from every inventoried source on structure AND expression, with the departure logged"],
  ["transform thresholds revised","a 'transformed' output later flagged as a near-copy"],
  specialists=["creative_strategist","content_evaluator"]))

N.append(node("ADD_OWN_ELEMENT","distinguishing_original_contribution",
  "Isolate and require at least one genuinely original contributed element (a framing, a juxtaposition, a new component) that no source supplied; this is the distinguishing creative lever that converts borrowing into authorship.",
  "structure", ["TRANSFORM_NOT_COPY"], ["CQ_06"],
  b(0.86,0.82,0.82,0.58,0.82,0.8,0.6,0.64,0.5, 0.76,0.78, 0.86,0.64,0.2,0.55,[0.24,0.56],"the bar for what counts as an own element changes"),
  ["identification of the added original element","authorship contribution log","element-level novelty check"],
  ["legal authorship determination (delegated)","quantifying license-relevant 'amount taken'"],
  [pro("Naming the specific added element makes 'we contributed something' auditable, not rhetorical","the log records exactly which framing device the author invented versus inherited")],
  [con("An added element can be original yet trivial, leaving the substantive content still derivative","an original title bolted onto an otherwise copied body adds little distinguishing weight")],
  ["adding an element that is original but immaterial to the whole","claiming an added element that is itself borrowed"],
  ["at least one named element is verified present in the output and absent from every inventoried source"],
  ["own-element bar revised","an 'added element' found to be itself derivative"]))

N.append(node("STYLE_VS_EXPRESSION","idea_style_reusable_vs_expression_protected",
  "Apply the idea/expression distinction operationally: general ideas, styles, genres, and methods are reusable creative raw material, while a source's specific, fixed expression is the protectable thing not to reproduce. Flag, do not adjudicate, the boundary.",
  "reasoning", ["TRANSFORM_NOT_COPY","SOURCE_INVENTORY"], ["CQ_07"],
  b(0.88,0.8,0.78,0.66,0.86,0.85,0.64,0.58,0.58, 0.72,0.76, 0.88,0.58,0.24,0.58,[0.3,0.64],"the operational idea/expression boundary heuristics change"),
  ["reusable layer (idea/style/genre/method)","protectable layer (specific fixed expression)","boundary flagging"],
  ["the legal idea/expression determination itself (delegated to compliance_heavy)","scenes-a-faire legal analysis"],
  [pro("Separating reusable style from protectable expression lets the system borrow boldly at the idea level while guarding the expression level","emulating a noir STYLE is fine; reproducing a specific noir SCRIPT's lines is flagged")],
  [con("The idea/expression line is genuinely blurry and fact-specific; the system can only approximate and must defer the hard cases","whether a distinctive plot structure is 'idea' or 'expression' is contestable")],
  ["copying specific expression while believing only 'style' was borrowed","treating the heuristic boundary as a legal ruling"],
  ["each borrowing is classified as idea/style-layer (reusable) or expression-layer (flag-for-review), with rationale"],
  ["idea/expression heuristics change","a 'style-only' borrow later judged to copy expression"],
  specialists=["creative_strategist","compliance_liaison"]))

N.append(node("PASTICHE_VS_PARODY","imitation_vs_commentary_classification",
  "Genette's distinction: classify the output's relation to a target as pastiche (non-satirical imitation of a style, no commentary) versus parody (transformation of a specific target that comments on it). The relation changes both creative intent and the risk profile.",
  "reasoning", ["STYLE_VS_EXPRESSION"], ["CQ_07","CQ_08"],
  b(0.8,0.74,0.74,0.64,0.8,0.78,0.58,0.6,0.55, 0.72,0.76, 0.8,0.6,0.22,0.52,[0.28,0.62],"the pastiche/parody taxonomy or target-identification rule changes"),
  ["pastiche (stylistic imitation, no commentary)","parody (targeted transformation with commentary)","intent and target identification"],
  ["legal fair-use/parody determination (delegated to compliance_heavy)","trademark dilution analysis"],
  [pro("Distinguishing imitation from commentary clarifies intent and routes risk: parody transforms a target, pastiche merely wears its style","a video that mocks a specific ad is parody-with-commentary; a generic 'in the style of' clip is pastiche")],
  [con("Pastiche imitation can shade into copying a recognizable expression, and parody's protection is jurisdiction-specific and not decidable here","an 'homage' pastiche that reproduces a signature sequence is closer to copying than to commentary")],
  ["labeling a copy as 'parody' to imply safety","pastiche that imitates protectable expression, not just style"],
  ["the output is labeled pastiche or parody (or neither) with its target and commentary status recorded"],
  ["pastiche/parody taxonomy changes","a labeled 'parody' lacks any commentary on its target"]))

N.append(node("SOURCE_DISTANCE","distance_from_source_material",
  "Estimate how far the candidate output sits from each inventoried source along structural and expressive axes; small distance to a protectable source is the primary derivative-risk signal.",
  "reasoning", ["TRANSFORM_NOT_COPY","STYLE_VS_EXPRESSION"], ["CQ_09"],
  b(0.86,0.78,0.74,0.7,0.85,0.8,0.6,0.58,0.58, 0.7,0.74, 0.86,0.58,0.24,0.58,[0.3,0.66],"the source-distance metric or its axes change"),
  ["per-source structural distance","per-source expressive distance","nearest-source identification"],
  ["legal substantial-similarity ruling (delegated)","plagiarism-grade exact-match forensics beyond flagging"],
  [pro("A per-source distance turns derivative risk into a continuous, rankable signal instead of a yes/no hunch","the draft scores 0.2 distance from one source and 0.8 from another, pinpointing the risky one")],
  [con("Distance metrics are proxy measures; semantic copying can be lexically distant and a paraphrase can be semantically near","a close paraphrase scores high lexical distance yet copies the underlying expression")],
  ["over-trusting a single distance metric","distance computed against an incomplete source inventory"],
  ["each inventoried source has a computed structural and expressive distance, with the nearest flagged"],
  ["distance metric changes","a low-distance output passes without a derivative-risk flag"]))

N.append(node("PRIOR_OUTPUT_DISTANCE","distance_from_own_prior_outputs",
  "Estimate how far the candidate sits from the system's OWN recent outputs; small distance to prior outputs signals self-repetition (recycling the same joke, hook, or format) rather than source-copying.",
  "reasoning", ["SOURCE_DISTANCE"], ["CQ_09","CQ_10"],
  b(0.8,0.78,0.8,0.62,0.74,0.76,0.5,0.64,0.5, 0.74,0.76, 0.8,0.64,0.22,0.48,[0.26,0.58],"the prior-output window or self-similarity metric changes"),
  ["distance to recent own outputs","self-similarity over a sliding window","format/joke reuse detection"],
  ["source-copying detection (handled by SOURCE_DISTANCE)","cross-account similarity"],
  [pro("Measuring distance from one's own history catches self-plagiarism and audience fatigue that source checks miss","two posts a week apart reuse the same hook structure and are flagged as self-repetition")],
  [con("A consistent brand voice legitimately repeats motifs, so distance must distinguish signature from staleness","a recurring catchphrase is on-brand repetition, not fatigue")],
  ["flagging intentional brand motifs as repetition","window too short to catch a reused format from last month"],
  ["the candidate's distance from each output in the recent window is computed and near-duplicates surfaced"],
  ["prior-output window changes","a recycled format ships without being flagged"]))

N.append(node("NOVELTY_SCORE","combined_novelty_metric",
  "Combine distance-from-source and distance-from-prior-outputs into a single novelty score, gated by appropriateness, so a high score means 'far from sources, far from our reruns, and still fitting the brief'.",
  "reasoning", ["SOURCE_DISTANCE","PRIOR_OUTPUT_DISTANCE","ORIGINALITY_DEFINITION"], ["CQ_10","CQ_11"],
  b(0.9,0.84,0.84,0.7,0.86,0.85,0.62,0.58,0.6, 0.7,0.74, 0.9,0.58,0.24,0.6,[0.3,0.66],"the novelty-score formula or appropriateness gate changes"),
  ["source-distance term","prior-output-distance term","appropriateness gate on the combined score"],
  ["legal originality scoring (delegated)","absolute creativity ranking across unrelated domains"],
  [pro("One gated score makes novelty comparable across candidates and prevents 'novel but off-brief' from scoring high","a candidate far from sources and reruns but off-brief is capped by the appropriateness gate")],
  [con("Collapsing two distances plus a gate into one number hides which term drove the result and can be gamed","an output games the prior-output term by trivial surface changes while staying near a source")],
  ["a high novelty score read as a low legal risk","the gate masking a near-copy that is merely brief-appropriate"],
  ["the novelty score exposes its source-distance, prior-distance, and appropriateness sub-terms and is not reported as a legal signal"],
  ["novelty formula changes","a high-novelty output is later flagged derivative"],
  specialists=["creative_strategist","content_evaluator"]))

N.append(node("REPETITION_GUARD","reuse_frequency_governor",
  "Govern how often the same joke, hook, structure, or format may be reused across outputs, enforcing a cooldown so a successful device is not run into the ground.",
  "control", ["PRIOR_OUTPUT_DISTANCE"], ["CQ_12"],
  b(0.78,0.78,0.82,0.56,0.72,0.74,0.48,0.66,0.5, 0.76,0.78, 0.78,0.66,0.22,0.46,[0.24,0.54],"the reuse cooldown policy or device taxonomy changes"),
  ["per-device reuse counter","cooldown enforcement","format rotation policy"],
  ["measuring distance itself (PRIOR_OUTPUT_DISTANCE)","cross-platform scheduling"],
  [pro("A reuse governor converts the fuzzy 'don't overuse it' into an enforceable cooldown per device","a hook used twice this week is blocked from a third use until its cooldown elapses")],
  [con("Cooldowns can suppress a device that is still resonating, trading freshness for lost performance","retiring a format the audience still loves leaves engagement on the table")],
  ["device taxonomy too coarse to count reuse","cooldown so strict it forbids a winning recurring bit"],
  ["each reusable device has a reuse count and an enforced cooldown before it may repeat"],
  ["cooldown policy changes","a device is overused despite the guard"]))

N.append(node("FRESHNESS_DECAY","format_wear_out_model",
  "Model how a format, trend, or device loses novelty over time and exposure (it wears out), so the system retires or refreshes it before audience fatigue rather than after.",
  "control", ["PRIOR_OUTPUT_DISTANCE"], ["CQ_12","CQ_13"],
  b(0.78,0.78,0.8,0.6,0.72,0.74,0.5,0.62,0.5, 0.74,0.76, 0.78,0.62,0.24,0.46,[0.28,0.6],"the decay model or trend-lifecycle data changes"),
  ["time/exposure decay curve per format","fatigue threshold","retire-or-refresh trigger"],
  ["initial novelty estimation (NOVELTY_SCORE)","paid-distribution scheduling"],
  [pro("Modeling decay lets the system anticipate staleness and refresh proactively instead of reacting to a flop","a trend's decay curve flags it for retirement before its engagement collapses")],
  [con("Decay rates vary by audience and channel and are hard to estimate, risking premature retirement of evergreen formats","an evergreen explainer format is wrongly retired on a generic decay curve")],
  ["one global decay curve applied to heterogeneous formats","retiring an evergreen format as if it were a fad"],
  ["each active format carries a freshness estimate and a retire-or-refresh trigger tied to a decay model"],
  ["decay model recalibrated","a format flopped from fatigue before its decay flag fired"]))

N.append(node("SUBSTANTIAL_SIMILARITY_PROBE","near_copy_detection_probe",
  "Probe for the near-copy condition: passages or structures whose distance to a protectable source is below a low threshold, surfacing the specific overlapping spans as evidence rather than a verdict.",
  "verification", ["SOURCE_DISTANCE","STYLE_VS_EXPRESSION"], ["CQ_14"],
  b(0.88,0.78,0.72,0.72,0.9,0.82,0.66,0.56,0.62, 0.7,0.74, 0.88,0.56,0.26,0.64,[0.32,0.7],"the near-copy threshold or span-matching method changes"),
  ["span-level overlap detection","low-distance region surfacing","evidence span extraction"],
  ["the legal substantial-similarity determination (delegated to compliance_heavy)","fair-use weighing"],
  [pro("Surfacing the exact overlapping spans gives compliance concrete evidence and gives authors a precise fix target","the probe highlights the three sentences that closely track a source paragraph")],
  [con("Threshold choice trades false alarms against missed near-copies, and the probe cannot judge whether overlap is legally actionable","a heavily-quoted-then-attributed passage trips the probe though it may be lawful")],
  ["a low threshold buries authors in false alarms","probe output mistaken for a copyright determination"],
  ["any region below the near-copy threshold is surfaced with its source, span, and distance as evidence (never as a verdict)"],
  ["near-copy threshold changes","an actual near-copy passes the probe undetected"],
  specialists=["content_evaluator","compliance_liaison"]))

N.append(node("DERIVATIVE_RISK_FLAG","derivative_risk_flag_not_safe_harbor",
  "Raise a derivative-risk flag whenever transformation may be insufficient, EXPLICITLY encoding that transformation is a creative measure and NOT a legal safe harbor; the flag asserts elevated risk and the need for legal screening, never that the output is clear.",
  "verification", ["SUBSTANTIAL_SIMILARITY_PROBE","TRANSFORM_NOT_COPY","SOURCE_ATTRIBUTION"], ["CQ_15"],
  b(0.94,0.84,0.78,0.66,0.95,0.88,0.74,0.55,0.65, 0.68,0.72, 0.94,0.55,0.28,0.72,[0.34,0.74],"the derivative-risk flagging policy or safe-harbor stance changes"),
  ["risk flag with severity and reasons","explicit non-safe-harbor assertion","trigger for legal screening"],
  ["deciding legal sufficiency, fair use, or licensing (delegated to compliance_heavy, scope FP6/KU8)","clearing the output"],
  [pro("A flag that explicitly denies safe-harbor status prevents the dangerous inference 'we transformed it, so we are fine'","the flag states: elevated derivative risk; transformation does not clear this; route to compliance")],
  [con("Over-flagging trains reviewers to ignore the flag, while under-flagging lets a near-copy ship as 'transformed'","if every borrow is flagged critical, real risks get lost in the noise")],
  ["treating the flag as a clearance once transformation is applied","suppressing the flag because an own element was added"],
  ["any below-threshold or high-similarity output carries a derivative-risk flag stating it is NOT cleared and must be legally screened","the flag never asserts legal sufficiency under any transformation"],
  ["safe-harbor policy or flag thresholds change","an output shipped on a 'transformed therefore safe' inference"],
  specialists=["compliance_liaison","content_evaluator"]))

N.append(node("SOURCE_ATTRIBUTION","borrowed_element_attribution_record",
  "Maintain a per-output attribution record of what was borrowed, from which source, at the idea/style or expression layer, so provenance is available for both credit and compliance screening.",
  "verification", ["SOURCE_INVENTORY","STYLE_VS_EXPRESSION"], ["CQ_16"],
  b(0.84,0.78,0.74,0.58,0.84,0.82,0.62,0.62,0.55, 0.76,0.78, 0.84,0.62,0.22,0.55,[0.28,0.6],"the attribution schema or credit policy changes"),
  ["per-borrow attribution entries","layer tag (idea/style vs expression)","credit/provenance ledger"],
  ["legal license sufficiency (delegated)","public-facing citation styling"],
  [pro("An attribution ledger makes borrowing transparent and feeds the derivative-risk flag and compliance with exact provenance","the ledger shows which structure came from which source at which layer")],
  [con("Attribution records credit but does not create permission; an attributed borrow can still be a near-copy","crediting a source does not make reproducing its expression lawful")],
  ["treating attribution as permission","incomplete ledger missing a material borrow"],
  ["every material borrow has a ledger entry naming source, layer, and the borrowed element"],
  ["attribution schema changes","a material borrow ships without a ledger entry"]))

N.append(node("NOVELTY_APPROPRIATENESS_BALANCE","novelty_appropriateness_tradeoff",
  "Resolve the standing tension between pushing novelty and preserving appropriateness/brief-fit, choosing the operating point per brief so neither maximal novelty nor maximal safety dominates by default.",
  "verification", ["NOVELTY_SCORE","ORIGINALITY_DEFINITION"], ["CQ_11","CQ_17"],
  b(0.82,0.8,0.8,0.64,0.8,0.82,0.58,0.6,0.6, 0.72,0.76, 0.82,0.6,0.24,0.52,[0.28,0.62],"the per-brief novelty/appropriateness operating point changes"),
  ["chosen novelty/appropriateness operating point","brief-specific weighting","tradeoff rationale"],
  ["legal risk weighting (delegated to compliance)","channel-specific scheduling"],
  [pro("Choosing the operating point explicitly stops the default from silently maximizing one axis at the other's expense","a conservative brand brief weights appropriateness higher; an experimental brief weights novelty higher")],
  [con("The right operating point is brief- and risk-dependent and can be set wrong, over- or under-shooting novelty","a conservative brief pushed to maximal novelty produces off-brand work")],
  ["maximizing novelty regardless of brief","defaulting to safe sameness to avoid any risk"],
  ["each brief records an explicit novelty/appropriateness operating point with a rationale, and outputs are scored against it"],
  ["brief operating point changes","outputs cluster at one axis extreme against the brief"]))

N.append(node("COMPLIANCE_ROUTING","route_legal_sufficiency_to_compliance_heavy",
  "Route every derivative-risk flag and its provenance to the compliance_heavy track (scope FP6 / KU8) for the legal-sufficiency determination, enforcing that this KB never decides fair use, substantial similarity, or licensing itself.",
  "verification", ["DERIVATIVE_RISK_FLAG","SOURCE_ATTRIBUTION"], ["CQ_15","CQ_18"],
  b(0.92,0.82,0.76,0.6,0.92,0.86,0.72,0.58,0.62, 0.7,0.74, 0.92,0.58,0.26,0.7,[0.32,0.72],"the compliance interface contract or scope boundary changes"),
  ["handoff package (flag + provenance + spans) to compliance_heavy","scope boundary enforcement","blocking on legal determination"],
  ["making the legal-sufficiency determination here","interpreting licenses or fair-use factors in this KB"],
  [pro("A hard route to compliance_heavy makes the safe-harbor boundary a structural fact, not a hope: legal calls happen only there","every flagged output is blocked from publish until compliance_heavy returns a determination")],
  [con("A heavy compliance gate adds latency and can bottleneck throughput if flagging is noisy","an over-flagging upstream stage floods compliance and stalls the pipeline")],
  ["bypassing the route when timelines are tight","deciding legal sufficiency upstream to 'save time'"],
  ["every derivative-risk-flagged output is routed to compliance_heavy with full provenance and is blocked from finalization until that track returns a determination","no legal-sufficiency call is made within this KB"],
  ["compliance interface changes","a flagged output published without a compliance determination"],
  specialists=["compliance_liaison","creative_strategist"]))

N.append(node("NOVELTY_AUDIT_TRAIL","novelty_decision_provenance",
  "Record the full provenance of every novelty decision: source inventory, distances, applied transformations, added elements, flags raised, and routing, so any novelty or derivative-risk judgment can be reconstructed and audited.",
  "verification", ["NOVELTY_SCORE","DERIVATIVE_RISK_FLAG","COMPLIANCE_ROUTING"], ["CQ_18","CQ_19"],
  b(0.8,0.76,0.74,0.58,0.78,0.78,0.6,0.66,0.46, 0.8,0.8, 0.8,0.66,0.18,0.5,[0.2,0.48],"the audit-trail schema or retention policy changes"),
  ["per-decision provenance record","linkage of score, flag, and routing","reconstructable audit log"],
  ["execution/publishing internals","compliance's own internal record (owned by that track)"],
  [pro("A complete audit trail makes every novelty and risk call defensible and lets a flagged decision be reconstructed end to end","an auditor can replay why an output scored high novelty yet was still flagged and routed")],
  [con("Full provenance adds storage and write overhead proportional to output volume","logging every distance and transform for high-volume generation is costly")],
  ["missing provenance prevents reconstructing a flag","trail volume overwhelms retention budget"],
  ["every novelty/derivative decision carries a reconstructable trail of inventory, distances, transforms, flags, and routing"],
  ["audit schema changes","a flagged decision could not be reconstructed from the trail"],
  specialists=["provenance_analyst","content_evaluator"]))

CQ = [
 ("CQ_01","What is the operating definition of an original output, and why is novelty alone insufficient?",["nodes","glossary"],"ORIGINALITY_DEFINITION fixes originality as novel AND appropriate",["ORIGINALITY_DEFINITION"]),
 ("CQ_02","How are the source materials an output draws on identified so borrowing can be measured?",["nodes"],"SOURCE_INVENTORY enumerates concrete sources and their roles",["SOURCE_INVENTORY"]),
 ("CQ_03","How is novelty generated by recombining known elements and exploring within a style space (Boden's combinational/exploratory modes)?",["nodes"],"COMBINATIONAL_CREATIVITY and EXPLORATORY_CREATIVITY define recombination and within-space exploration",["COMBINATIONAL_CREATIVITY","EXPLORATORY_CREATIVITY"]),
 ("CQ_04","What is transformational creativity and how does it differ from exploring within a space?",["nodes"],"TRANSFORMATIONAL_CREATIVITY alters a space-defining constraint, unlike EXPLORATORY_CREATIVITY",["TRANSFORMATIONAL_CREATIVITY","EXPLORATORY_CREATIVITY"]),
 ("CQ_05","What is the modify-and-add directive, how does it lower near-copy probability, and what counts as a verified added own element?",["nodes","workflow"],"TRANSFORM_NOT_COPY requires measurable departure; ADD_OWN_ELEMENT verifies a distinguishing original contribution",["TRANSFORM_NOT_COPY","ADD_OWN_ELEMENT"]),
 ("CQ_06","How is reusable style separated from protectable expression, and how does pastiche differ from parody?",["nodes"],"STYLE_VS_EXPRESSION flags the boundary; PASTICHE_VS_PARODY classifies imitation versus commentary",["STYLE_VS_EXPRESSION","PASTICHE_VS_PARODY"]),
 ("CQ_07","How is distance from external sources and from the system's own prior outputs measured?",["nodes"],"SOURCE_DISTANCE and PRIOR_OUTPUT_DISTANCE compute the two distance signals",["SOURCE_DISTANCE","PRIOR_OUTPUT_DISTANCE"]),
 ("CQ_08","How are the two distances combined into a novelty score gated by appropriateness?",["nodes","math_model"],"NOVELTY_SCORE combines source and prior distances under an appropriateness gate",["NOVELTY_SCORE"]),
 ("CQ_09","How is the novelty-versus-appropriateness tradeoff set per brief so neither reckless novelty nor defensive sameness dominates?",["nodes","conflict_axes"],"NOVELTY_APPROPRIATENESS_BALANCE picks the per-brief operating point against the originality definition",["NOVELTY_APPROPRIATENESS_BALANCE"]),
 ("CQ_10","How is overuse of the same device prevented and how is format wear-out anticipated before fatigue?",["nodes","iteration_protocol"],"REPETITION_GUARD enforces cooldowns; FRESHNESS_DECAY models format decay and retire-or-refresh",["REPETITION_GUARD","FRESHNESS_DECAY"]),
 ("CQ_11","How are near-copy passages detected and surfaced as evidence rather than as verdicts?",["nodes"],"SUBSTANTIAL_SIMILARITY_PROBE surfaces low-distance spans as evidence",["SUBSTANTIAL_SIMILARITY_PROBE"]),
 ("CQ_12","How is derivative risk flagged, and why is transformation explicitly NOT treated as a legal safe harbor?",["nodes","edge_cases"],"DERIVATIVE_RISK_FLAG raises a non-safe-harbor flag asserting elevated risk and the need for legal screening",["DERIVATIVE_RISK_FLAG"]),
 ("CQ_13","How is borrowed material attributed and how is legal sufficiency routed out of this KB to compliance_heavy?",["nodes","workflow"],"SOURCE_ATTRIBUTION records provenance; COMPLIANCE_ROUTING routes legal-sufficiency decisions to compliance_heavy",["SOURCE_ATTRIBUTION","COMPLIANCE_ROUTING"]),
 ("CQ_14","How is every novelty and derivative-risk decision made reconstructable and auditable?",["nodes"],"NOVELTY_AUDIT_TRAIL records the full decision provenance",["NOVELTY_AUDIT_TRAIL"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

# consolidate node->CQ references onto the 14-CQ set (each node refs 1-2; every CQ covered by >=1 node)
CQ_MAP = {
 "ORIGINALITY_DEFINITION":["CQ_01","CQ_08"], "SOURCE_INVENTORY":["CQ_02"],
 "COMBINATIONAL_CREATIVITY":["CQ_03"], "EXPLORATORY_CREATIVITY":["CQ_03","CQ_04"],
 "TRANSFORMATIONAL_CREATIVITY":["CQ_04"], "TRANSFORM_NOT_COPY":["CQ_05"],
 "ADD_OWN_ELEMENT":["CQ_05"], "STYLE_VS_EXPRESSION":["CQ_06"],
 "PASTICHE_VS_PARODY":["CQ_06"], "SOURCE_DISTANCE":["CQ_07"],
 "PRIOR_OUTPUT_DISTANCE":["CQ_07"], "NOVELTY_SCORE":["CQ_08"],
 "REPETITION_GUARD":["CQ_10"], "FRESHNESS_DECAY":["CQ_10"],
 "SUBSTANTIAL_SIMILARITY_PROBE":["CQ_11"], "DERIVATIVE_RISK_FLAG":["CQ_12"],
 "SOURCE_ATTRIBUTION":["CQ_13"], "NOVELTY_APPROPRIATENESS_BALANCE":["CQ_09"],
 "COMPLIANCE_ROUTING":["CQ_13"], "NOVELTY_AUDIT_TRAIL":["CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

GL = [
 ("originality","the property of being both novel (different from prior work) and appropriate (fitting the brief/context); per the standard definition of creativity",["creativity"],["mere_novelty"],["ORIGINALITY_DEFINITION","NOVELTY_SCORE"]),
 ("combinational_creativity","novelty produced by unfamiliar combinations of familiar elements (Boden)",["recombination","conceptual_blend"],["transformational_creativity"],["COMBINATIONAL_CREATIVITY","TRANSFORM_NOT_COPY"]),
 ("exploratory_creativity","novelty produced by exploring unfilled regions of an accepted conceptual space without breaking its rules (Boden)",["within_space_search"],["transformational_creativity"],["EXPLORATORY_CREATIVITY"]),
 ("transformational_creativity","novelty produced by altering a defining constraint of the conceptual space itself (Boden)",["space_altering_creativity"],["exploratory_creativity"],["TRANSFORMATIONAL_CREATIVITY"]),
 ("idea_expression_distinction","the principle that ideas/styles/methods are reusable while a source's specific fixed expression is protectable",["style_vs_expression"],["fair_use_determination"],["STYLE_VS_EXPRESSION","SOURCE_ATTRIBUTION"]),
 ("pastiche","non-satirical imitation of a style with no commentary on a specific target (Genette)",["stylistic_imitation","homage"],["parody"],["PASTICHE_VS_PARODY"]),
 ("parody","transformation of a specific target that comments on or subverts it (Genette)",["satirical_transformation"],["pastiche"],["PASTICHE_VS_PARODY"]),
 ("derivative_risk","the estimated likelihood that an output reproduces protectable expression of a source closely enough to require legal screening",["near_copy_risk"],["legal_determination"],["DERIVATIVE_RISK_FLAG","SUBSTANTIAL_SIMILARITY_PROBE"]),
 ("safe_harbor","a legal protection that clears a use; explicitly NOT something transformation provides and NOT decided in this KB",["legal_clearance"],["creative_transformation"],["DERIVATIVE_RISK_FLAG","COMPLIANCE_ROUTING"]),
 ("freshness_decay","the loss of perceived novelty of a format/device over time and exposure as it wears out",["format_fatigue","wear_out"],["evergreen_value"],["FRESHNESS_DECAY","REPETITION_GUARD"]),
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
              "risk_of_conflict":"unmanaged tension degrades output quality or compliance posture","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated behavior",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies)
dep("ORIGINALITY_DEFINITION","COMBINATIONAL_CREATIVITY",0.84)
dep("COMBINATIONAL_CREATIVITY","EXPLORATORY_CREATIVITY",0.8)
dep("EXPLORATORY_CREATIVITY","TRANSFORMATIONAL_CREATIVITY",0.78)
dep("COMBINATIONAL_CREATIVITY","TRANSFORM_NOT_COPY",0.82)
dep("SOURCE_INVENTORY","TRANSFORM_NOT_COPY",0.8)
dep("TRANSFORM_NOT_COPY","ADD_OWN_ELEMENT",0.84)
dep("TRANSFORM_NOT_COPY","STYLE_VS_EXPRESSION",0.82)
dep("SOURCE_INVENTORY","STYLE_VS_EXPRESSION",0.78)
dep("STYLE_VS_EXPRESSION","PASTICHE_VS_PARODY",0.8)
dep("TRANSFORM_NOT_COPY","SOURCE_DISTANCE",0.82)
dep("STYLE_VS_EXPRESSION","SOURCE_DISTANCE",0.78)
dep("SOURCE_DISTANCE","PRIOR_OUTPUT_DISTANCE",0.78)
dep("SOURCE_DISTANCE","NOVELTY_SCORE",0.84)
dep("PRIOR_OUTPUT_DISTANCE","NOVELTY_SCORE",0.82)
dep("ORIGINALITY_DEFINITION","NOVELTY_SCORE",0.8)
dep("PRIOR_OUTPUT_DISTANCE","REPETITION_GUARD",0.78)
dep("PRIOR_OUTPUT_DISTANCE","FRESHNESS_DECAY",0.76)
dep("SOURCE_DISTANCE","SUBSTANTIAL_SIMILARITY_PROBE",0.84)
dep("STYLE_VS_EXPRESSION","SUBSTANTIAL_SIMILARITY_PROBE",0.78)
dep("SOURCE_INVENTORY","SOURCE_ATTRIBUTION",0.8)
dep("STYLE_VS_EXPRESSION","SOURCE_ATTRIBUTION",0.76)
dep("SUBSTANTIAL_SIMILARITY_PROBE","DERIVATIVE_RISK_FLAG",0.86)
dep("TRANSFORM_NOT_COPY","DERIVATIVE_RISK_FLAG",0.8)
dep("SOURCE_ATTRIBUTION","DERIVATIVE_RISK_FLAG",0.78)
dep("NOVELTY_SCORE","NOVELTY_APPROPRIATENESS_BALANCE",0.8)
dep("ORIGINALITY_DEFINITION","NOVELTY_APPROPRIATENESS_BALANCE",0.76)
dep("DERIVATIVE_RISK_FLAG","COMPLIANCE_ROUTING",0.88)
dep("SOURCE_ATTRIBUTION","COMPLIANCE_ROUTING",0.78)
dep("NOVELTY_SCORE","NOVELTY_AUDIT_TRAIL",0.76)
dep("DERIVATIVE_RISK_FLAG","NOVELTY_AUDIT_TRAIL",0.78)
dep("COMPLIANCE_ROUTING","NOVELTY_AUDIT_TRAIL",0.76)
# cross-cutting non-dependency edges
rel("REPETITION_GUARD","FRESHNESS_DECAY","constraint",0.74,why="reuse cooldowns and format decay jointly govern when a device must be retired")
rel("ADD_OWN_ELEMENT","SOURCE_DISTANCE","causal",0.74,why="adding a genuine own element increases the candidate's distance from sources")
rel("PASTICHE_VS_PARODY","DERIVATIVE_RISK_FLAG","causal",0.72,why="a pastiche imitating protectable expression elevates the derivative-risk flag")
rel("NOVELTY_AUDIT_TRAIL","SOURCE_ATTRIBUTION","similarity",0.7,why="the audit trail consumes and links the attribution ledger entries")
# conflict edges (negative signed_tension + resolution_rule)
conf("TRANSFORM_NOT_COPY","DERIVATIVE_RISK_FLAG",0.74,-0.6,
  "transformation raises novelty but NEVER clears derivative risk: apply the transform lever for creativity, yet always let DERIVATIVE_RISK_FLAG fire on low source-distance and route to compliance_heavy regardless of how much was modified",
  "the transform-and-add lever is creative, but treating it as a legal safe harbor directly conflicts with the obligation to flag derivative risk")
conf("NOVELTY_SCORE","NOVELTY_APPROPRIATENESS_BALANCE",0.7,-0.5,
  "cap the novelty score by the brief's appropriateness gate: pursue novelty only up to the operating point set per brief, never maximizing novelty at the expense of fit",
  "maximizing the novelty score conflicts with preserving appropriateness and brief-fit")
conf("COMBINATIONAL_CREATIVITY","SUBSTANTIAL_SIMILARITY_PROBE",0.68,-0.45,
  "recombine boldly but verify per-element dominance: if any single recombined element falls below the near-copy threshold against a protectable source, the probe overrides the 'it is a new combination' claim",
  "an aggressive recombination can still embed a near-copy of one dominant source element")
conf("REPETITION_GUARD","FRESHNESS_DECAY",0.66,-0.4,
  "distinguish signature from staleness: exempt declared brand motifs from the reuse cooldown while still applying freshness decay to non-signature devices",
  "a strict reuse cooldown can suppress an on-brand recurring motif that freshness decay would still consider fresh")

CA=[
 {"name":"transformation_as_lever_vs_as_safe_harbor","description":"Transformation reliably raises novelty and lowers near-copy probability, but treating it as a legal clearance is the central error this KB exists to prevent; legal sufficiency belongs to compliance_heavy.","poles":["transformation_clears_risk","transformation_is_creative_only"],"resolution_hint":"always flag and route despite transformation; never infer safe-harbor from modification","tension_score":0.8,"affected_nodes":["TRANSFORM_NOT_COPY","DERIVATIVE_RISK_FLAG","COMPLIANCE_ROUTING"]},
 {"name":"novelty_maximization_vs_appropriateness","description":"Pushing novelty can erode brief-fit; defending appropriateness can collapse into safe sameness. The standard definition requires both.","poles":["max_novelty","max_appropriateness"],"resolution_hint":"set an explicit per-brief operating point; gate novelty by appropriateness","tension_score":0.75,"affected_nodes":["NOVELTY_SCORE","NOVELTY_APPROPRIATENESS_BALANCE","ORIGINALITY_DEFINITION"]},
 {"name":"recombination_boldness_vs_element_dominance","description":"Bold recombination yields novelty, but a single dominant protectable element turns the 'new combination' into a near-copy.","poles":["bold_recombination","element_dominance_control"],"resolution_hint":"cap per-element dominance and run the similarity probe on each element","tension_score":0.68,"affected_nodes":["COMBINATIONAL_CREATIVITY","TRANSFORM_NOT_COPY","SUBSTANTIAL_SIMILARITY_PROBE"]},
 {"name":"style_reuse_vs_expression_copying","description":"Reusing a style is legitimate raw material; reproducing specific protectable expression is the line, but the boundary is blurry and must be flagged, not adjudicated here.","poles":["bold_style_reuse","expression_protection"],"resolution_hint":"classify the borrow layer; flag expression-layer borrows for review","tension_score":0.72,"affected_nodes":["STYLE_VS_EXPRESSION","PASTICHE_VS_PARODY","SOURCE_ATTRIBUTION"]},
 {"name":"signature_repetition_vs_freshness","description":"On-brand recurring motifs build recognition, but the same device overused causes fatigue; signature and staleness must be told apart.","poles":["brand_signature","format_freshness"],"resolution_hint":"exempt declared signatures from cooldown; apply decay to the rest","tension_score":0.62,"affected_nodes":["REPETITION_GUARD","FRESHNESS_DECAY","PRIOR_OUTPUT_DISTANCE"]},
 {"name":"source_distance_vs_self_distance","description":"An output can be far from external sources yet a rerun of the system's own prior work, or vice versa; both distances matter and can disagree.","poles":["source_distance_priority","self_distance_priority"],"resolution_hint":"score both distances separately and surface whichever is small","tension_score":0.6,"affected_nodes":["SOURCE_DISTANCE","PRIOR_OUTPUT_DISTANCE","NOVELTY_SCORE"]},
 {"name":"flag_sensitivity_vs_alert_fatigue","description":"A sensitive derivative-risk flag catches near-copies but over-flagging trains reviewers to ignore it; threshold choice trades the two.","poles":["high_sensitivity","low_false_alarm"],"resolution_hint":"calibrate the threshold and report severity, not a binary alarm","tension_score":0.66,"affected_nodes":["DERIVATIVE_RISK_FLAG","SUBSTANTIAL_SIMILARITY_PROBE","COMPLIANCE_ROUTING"]},
 {"name":"pastiche_homage_vs_imitation_copying","description":"Pastiche/homage celebrates a style, but imitation can slide into reproducing a recognizable protectable sequence.","poles":["stylistic_homage","expression_imitation_risk"],"resolution_hint":"label pastiche vs parody and flag homages that reproduce specific expression","tension_score":0.6,"affected_nodes":["PASTICHE_VS_PARODY","STYLE_VS_EXPRESSION","DERIVATIVE_RISK_FLAG"]},
 {"name":"provenance_completeness_vs_overhead","description":"A complete audit trail makes every novelty/risk call defensible but costs storage proportional to output volume.","poles":["full_provenance","minimal_overhead"],"resolution_hint":"log decisions, distances, flags, and routing; sample verbose intermediates","tension_score":0.55,"affected_nodes":["NOVELTY_AUDIT_TRAIL","SOURCE_ATTRIBUTION","COMPLIANCE_ROUTING"]},
]

EC=[
 {"description":"An output is heavily copied from one source, then trivially modified, and shipped on the inference that 'we transformed it, so it is safe'.","trigger":"transformation treated as a legal safe harbor instead of a creative lever","affected_nodes":["TRANSFORM_NOT_COPY","DERIVATIVE_RISK_FLAG","COMPLIANCE_ROUTING"],"mitigation":"fire the derivative-risk flag on low source-distance regardless of modification and route to compliance_heavy; the flag explicitly denies safe-harbor status","severity":"critical"},
 {"description":"A near-copy of a protectable source passes because the source was never listed in the inventory.","trigger":"an uninventoried source the output actually copies","affected_nodes":["SOURCE_INVENTORY","SOURCE_DISTANCE","SUBSTANTIAL_SIMILARITY_PROBE"],"mitigation":"require an explicit (justified) source inventory; treat absence of citation as a risk, not as absence of borrowing","severity":"high"},
 {"description":"A 'new combination' is dominated by one protectable element and is effectively a copy of that element.","trigger":"recombination where a single source element exceeds the dominance threshold","affected_nodes":["COMBINATIONAL_CREATIVITY","SUBSTANTIAL_SIMILARITY_PROBE","DERIVATIVE_RISK_FLAG"],"mitigation":"cap per-element dominance and run the similarity probe on each recombined element","severity":"high"},
 {"description":"An output scores high novelty but is off-brief, and the high score is read as success.","trigger":"novelty maximized without the appropriateness gate","affected_nodes":["NOVELTY_SCORE","NOVELTY_APPROPRIATENESS_BALANCE","ORIGINALITY_DEFINITION"],"mitigation":"gate the novelty score by the per-brief appropriateness operating point","severity":"medium"},
 {"description":"A close paraphrase copies the underlying expression but scores high lexical distance and evades the probe.","trigger":"semantic copying that is lexically distant","affected_nodes":["SOURCE_DISTANCE","SUBSTANTIAL_SIMILARITY_PROBE","STYLE_VS_EXPRESSION"],"mitigation":"use both structural and expressive distance axes; do not rely on a single lexical metric","severity":"high"},
 {"description":"The same hook or format is reused too often and the audience tunes out.","trigger":"a successful device run past its cooldown and decay threshold","affected_nodes":["REPETITION_GUARD","FRESHNESS_DECAY","PRIOR_OUTPUT_DISTANCE"],"mitigation":"enforce per-device cooldowns and retire-or-refresh formats on the decay model","severity":"medium"},
 {"description":"An evergreen format is wrongly retired because a generic decay curve was applied.","trigger":"one global decay curve applied to a heterogeneous format","affected_nodes":["FRESHNESS_DECAY","REPETITION_GUARD"],"mitigation":"calibrate decay per format/audience; exempt verified evergreen formats","severity":"low"},
 {"description":"A pastiche 'homage' reproduces a recognizable protectable sequence and is labeled safe as 'just style'.","trigger":"pastiche that imitates protectable expression rather than only style","affected_nodes":["PASTICHE_VS_PARODY","STYLE_VS_EXPRESSION","DERIVATIVE_RISK_FLAG"],"mitigation":"classify the borrow layer; flag expression-layer imitation even when labeled homage","severity":"high"},
 {"description":"A material borrow is attributed (credited) and then assumed cleared because credit was given.","trigger":"attribution treated as permission","affected_nodes":["SOURCE_ATTRIBUTION","DERIVATIVE_RISK_FLAG","COMPLIANCE_ROUTING"],"mitigation":"keep attribution and clearance separate; route attributed near-copies to compliance anyway","severity":"high"},
 {"description":"A flagged output is published anyway because the compliance route was bypassed under deadline pressure.","trigger":"compliance routing skipped to save time","affected_nodes":["COMPLIANCE_ROUTING","DERIVATIVE_RISK_FLAG","NOVELTY_AUDIT_TRAIL"],"mitigation":"hard-block finalization of any flagged output until compliance_heavy returns a determination","severity":"critical"},
 {"description":"A derivative-risk decision cannot be reconstructed later because provenance was not logged.","trigger":"audit trail not recorded for a novelty/risk decision","affected_nodes":["NOVELTY_AUDIT_TRAIL","DERIVATIVE_RISK_FLAG","NOVELTY_SCORE"],"mitigation":"record inventory, distances, transforms, flags, and routing for every decision","severity":"medium"},
 {"description":"An author re-skins a copy and claims transformational creativity though no defining constraint was actually altered.","trigger":"claimed transformation with no constraint of the space changed","affected_nodes":["TRANSFORMATIONAL_CREATIVITY","TRANSFORM_NOT_COPY","SUBSTANTIAL_SIMILARITY_PROBE"],"mitigation":"require a named altered constraint for transformational claims; otherwise treat as copy plus cosmetic change","severity":"medium"},
]

WF=[
 {"action":"fix_originality_definition","node_ref":"ORIGINALITY_DEFINITION","description":"Establish the brief's originality bar as novel AND appropriate before any generation or scoring.","artifact":"originality_criterion","gate":"both novelty and appropriateness criteria are explicit"},
 {"action":"inventory_sources","node_ref":"SOURCE_INVENTORY","description":"Enumerate every source the output draws on and tag each source's role.","artifact":"source_inventory","gate":"an explicit (possibly empty, justified) source list exists"},
 {"action":"recombine_and_explore","node_ref":"COMBINATIONAL_CREATIVITY","description":"Generate novelty by recombining known elements and exploring the style space, capping per-element dominance.","artifact":"candidate_with_combination_log","gate":"output recombines >=2 elements with no single dominant source element"},
 {"action":"transform_and_add_element","node_ref":"TRANSFORM_NOT_COPY","description":"Modify the source material and add at least one named own element so the output departs measurably from every source.","artifact":"transformed_candidate","gate":"measurable departure on structure and expression plus a named added element"},
 {"action":"classify_borrow_layer","node_ref":"STYLE_VS_EXPRESSION","description":"Classify each borrow as reusable idea/style or protectable expression, and label pastiche versus parody.","artifact":"borrow_classification","gate":"every borrow classified by layer with rationale"},
 {"action":"measure_distances","node_ref":"NOVELTY_SCORE","description":"Compute source-distance and prior-output-distance and combine into an appropriateness-gated novelty score.","artifact":"novelty_score_record","gate":"score exposes its source, prior, and appropriateness sub-terms"},
 {"action":"govern_reuse_and_freshness","node_ref":"REPETITION_GUARD","description":"Apply reuse cooldowns and freshness decay so devices are retired or refreshed before fatigue.","artifact":"reuse_and_freshness_state","gate":"each device has a cooldown and a freshness estimate"},
 {"action":"probe_near_copy","node_ref":"SUBSTANTIAL_SIMILARITY_PROBE","description":"Probe for below-threshold overlap with protectable sources and surface the exact spans as evidence.","artifact":"similarity_evidence","gate":"all low-distance spans surfaced with source and distance"},
 {"action":"attribute_borrows","node_ref":"SOURCE_ATTRIBUTION","description":"Record a per-borrow attribution ledger entry naming source, layer, and borrowed element.","artifact":"attribution_ledger","gate":"every material borrow has a ledger entry"},
 {"action":"flag_derivative_risk","node_ref":"DERIVATIVE_RISK_FLAG","description":"Raise a derivative-risk flag on near-copies, explicitly stating transformation is not a safe harbor and the output is not cleared.","artifact":"derivative_risk_flag","gate":"flag asserts non-clearance and need for legal screening, never legal sufficiency"},
 {"action":"route_to_compliance","node_ref":"COMPLIANCE_ROUTING","description":"Route every flag with full provenance to compliance_heavy and block finalization until a determination returns.","artifact":"compliance_handoff_package","gate":"flagged output blocked until compliance_heavy returns a determination"},
 {"action":"record_audit_trail","node_ref":"NOVELTY_AUDIT_TRAIL","description":"Record the full provenance of the novelty and derivative-risk decision for reconstructability.","artifact":"novelty_audit_trail","gate":"decision reconstructable from inventory, distances, transforms, flags, and routing"},
]

DR=[
 {"rule":"DERIVATIVE_RISK_FLAG must fire on low source-distance regardless of how much transformation was applied","rationale":"transformation is a creative lever, not a legal safe harbor; modification cannot clear a near-copy","trigger":"a below-threshold output is treated as cleared because it was transformed","action":"raise the flag and block clearance; transformation never suppresses the flag"},
 {"rule":"COMPLIANCE_ROUTING owns every legal-sufficiency determination; this KB never decides fair use, substantial similarity, or licensing","rationale":"legal sufficiency is out of scope (FP6/KU8) and belongs to compliance_heavy","trigger":"a legal-sufficiency call is attempted within this KB","action":"block and route the decision to compliance_heavy with full provenance"},
 {"rule":"NOVELTY_SCORE must be gated by the brief's appropriateness operating point before it is reported","rationale":"a novel but off-brief output is not original under the standard definition","trigger":"novelty reported without an appropriateness gate","action":"apply the appropriateness gate from NOVELTY_APPROPRIATENESS_BALANCE"},
 {"rule":"SOURCE_INVENTORY must exist before SOURCE_DISTANCE or SUBSTANTIAL_SIMILARITY_PROBE run","rationale":"distance and near-copy checks are only as complete as the source list","trigger":"distance computed against an empty or stale inventory","action":"block distance/probe until the inventory is current"},
 {"rule":"TRANSFORM_NOT_COPY must produce a named added element verified absent from every source","rationale":"an unnamed or borrowed 'own element' does not convert borrowing into authorship","trigger":"a transform claim with no verified added element","action":"reject the transform claim until a genuine own element is present"},
 {"rule":"COMBINATIONAL_CREATIVITY must cap per-element dominance and probe each recombined element","rationale":"a combination dominated by one protectable element is a near-copy of that element","trigger":"a single element exceeds the dominance threshold","action":"flag the dominant element via SUBSTANTIAL_SIMILARITY_PROBE"},
 {"rule":"Declared brand signatures are exempt from REPETITION_GUARD cooldown but still subject to FRESHNESS_DECAY","rationale":"signature motifs build recognition and must not be mistaken for staleness","trigger":"a declared signature blocked as repetition","action":"exempt the signature from cooldown while tracking its freshness"},
 {"rule":"SOURCE_ATTRIBUTION credit must never be treated as permission or clearance","rationale":"crediting a source does not authorize reproducing its protectable expression","trigger":"an attributed near-copy assumed cleared","action":"route the attributed near-copy to compliance regardless of credit"},
 {"rule":"NOVELTY_AUDIT_TRAIL must record any flagged decision before finalization","rationale":"a flagged decision must be reconstructable for compliance and review","trigger":"finalization requested for a flagged output with no trail","action":"block finalization until the audit trail is recorded"},
]

ARR=[
 {"rule":"Do not infer legal safe-harbor from transformation; modifying a copy does not clear it and rebuilding clearance later is costly","prevents":"shipping a near-copy as 'transformed therefore safe' and recalling it after a takedown"},
 {"rule":"Do not compute distances or probe near-copies against an incomplete source inventory; re-running after a missed source wastes the whole pass","prevents":"re-scoring novelty after an uninventoried source surfaces in review"},
 {"rule":"Do not report a novelty score without the appropriateness gate; an off-brief 'novel' output must be redone","prevents":"reworking an off-brief output that scored high on raw novelty"},
 {"rule":"Do not claim a new combination when one protectable element dominates; the near-copy must be fixed at the element level","prevents":"reworking a 'recombination' that was really a copy of one dominant element"},
 {"rule":"Do not treat attribution as permission; an attributed borrow can still require licensing or removal","prevents":"removing or relicensing attributed-but-uncleared material after publication"},
 {"rule":"Do not overuse a device past its cooldown and decay threshold; fatigue is expensive to recover from","prevents":"rebuilding audience engagement after a format is run into the ground"},
 {"rule":"Do not skip provenance on a flagged decision; retrofitting an audit trail requires re-running the whole assessment","prevents":"re-assessing a flagged output that could not be reconstructed for compliance"},
 {"rule":"Do not bypass compliance routing under deadline; an unscreened flagged output creates downstream legal rework","prevents":"post-publication legal remediation of an unscreened flagged output"},
]

IP=[
 {"trigger":"an output shipped on a 'transformed therefore safe' inference","action":"reaffirm that transformation never clears risk in DERIVATIVE_RISK_FLAG and verify the COMPLIANCE_ROUTING block is enforced","nodes":["DERIVATIVE_RISK_FLAG","TRANSFORM_NOT_COPY","COMPLIANCE_ROUTING"],"priority":"critical"},
 {"trigger":"a near-copy passed because its source was not inventoried","action":"widen SOURCE_INVENTORY source classes and recompute SOURCE_DISTANCE and the SUBSTANTIAL_SIMILARITY_PROBE","nodes":["SOURCE_INVENTORY","SOURCE_DISTANCE","SUBSTANTIAL_SIMILARITY_PROBE"],"priority":"high"},
 {"trigger":"a 'recombination' turned out to be dominated by one protectable element","action":"lower the per-element dominance cap in COMBINATIONAL_CREATIVITY and probe each element","nodes":["COMBINATIONAL_CREATIVITY","SUBSTANTIAL_SIMILARITY_PROBE","TRANSFORM_NOT_COPY"],"priority":"high"},
 {"trigger":"a high-novelty output was off-brief","action":"recalibrate the appropriateness gate in NOVELTY_APPROPRIATENESS_BALANCE and re-gate NOVELTY_SCORE","nodes":["NOVELTY_APPROPRIATENESS_BALANCE","NOVELTY_SCORE","ORIGINALITY_DEFINITION"],"priority":"high"},
 {"trigger":"a paraphrase copied expression yet evaded the probe","action":"add an expressive-distance axis in SOURCE_DISTANCE and retune the SUBSTANTIAL_SIMILARITY_PROBE threshold","nodes":["SOURCE_DISTANCE","SUBSTANTIAL_SIMILARITY_PROBE","STYLE_VS_EXPRESSION"],"priority":"high"},
 {"trigger":"a format flopped from audience fatigue","action":"recalibrate the decay model in FRESHNESS_DECAY and tighten the cooldown in REPETITION_GUARD","nodes":["FRESHNESS_DECAY","REPETITION_GUARD","PRIOR_OUTPUT_DISTANCE"],"priority":"medium"},
 {"trigger":"an attributed borrow was assumed cleared","action":"reinforce the attribution-is-not-permission rule and route attributed near-copies through COMPLIANCE_ROUTING","nodes":["SOURCE_ATTRIBUTION","DERIVATIVE_RISK_FLAG","COMPLIANCE_ROUTING"],"priority":"medium"},
 {"trigger":"a flagged decision could not be reconstructed","action":"extend the NOVELTY_AUDIT_TRAIL schema and enforce trail-before-finalization","nodes":["NOVELTY_AUDIT_TRAIL","DERIVATIVE_RISK_FLAG"],"priority":"medium"},
]

spec = {
 "domain":"novelty__derivative_vs_copy",
 "domain_label":"Novelty, Transformation & Derivative-Risk Flagging",
 "purpose":"maximize_novelty_by_recombination_and_added_original_elements_while_flagging_derivative_risk_for_legal_screening_rather_than_assuming_transformation_is_a_safe_harbor",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "scope is the creative novelty and derivative-risk-flagging method; legal sufficiency (fair use, substantial similarity, licensing) is delegated to compliance_heavy (FP6/KU8)",
   "a source inventory and a history of the system's own prior outputs are available to compute distances",
   "transformation and added original elements raise novelty and lower near-copy probability but never constitute a legal safe harbor",
 ],
 "exclusions":[
   "legal-sufficiency, fair-use, and substantial-similarity determinations (delegated to compliance_heavy, scope FP6/KU8)",
   "license interpretation and clearance",
   "aesthetic quality grading and editorial taste judgments",
   "platform distribution scheduling and paid media",
 ],
 "source_description":"heuristic prior estimates for novelty and derivative-risk work units, informed by Boden's three modes of creativity, Lessig's remix economy, Genette's pastiche/parody distinction, and the standard (novelty-plus-appropriateness) definition of creativity; no supplied dataset",
 "source_citation":"Boden 1990/2004 The Creative Mind: Myths and Mechanisms (combinational, exploratory, transformational creativity); Lessig 2008 Remix: Making Art and Commerce Thrive in the Hybrid Economy; Genette 1982 Palimpsestes (pastiche vs parody); Runco & Jaeger 2012 The Standard Definition of Creativity, Creativity Research Journal 24(1):92-96",
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
 "priority_rationale":"ORIGINALITY_DEFINITION, SOURCE_INVENTORY, and the three creativity modes are foundational; TRANSFORM_NOT_COPY and ADD_OWN_ELEMENT are the core levers; SOURCE_DISTANCE/PRIOR_OUTPUT_DISTANCE/NOVELTY_SCORE quantify novelty; SUBSTANTIAL_SIMILARITY_PROBE/DERIVATIVE_RISK_FLAG/COMPLIANCE_ROUTING and the audit trail close the method by flagging and routing derivative risk without deciding legal sufficiency.",
 "eval_objective":"verify_recombinational_novelty_appropriateness_gating_repetition_control_and_non_safe_harbor_derivative_risk_flagging_with_compliance_routing_of_novelty__derivative_vs_copy_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "novelty__derivative_vs_copy.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
