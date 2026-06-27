#!/usr/bin/env python3
"""Generate the scene__multi_entity_composition content spec (B60 content-intelligence
KB) for kb_forge.py. Compact authoring: node() applies sane defaults so only domain
content + base metric magnitudes are specified per node.

Domain: 2-3 Entity Scene & Comedic/Narrative Composition — compose two-to-three
linked entities into one audience-fit scene using incongruity->resolution humor,
proven meme formats as scaffolds, and micro-narrative beats, then gate the scene on
audience fit, sensitivity, shareability and timing.

Real literature cited (specific): Suls 1972 (incongruity-resolution two-stage model of
humor, in The Psychology of Humor); McGraw & Warren 2010 (Benign Violation Theory,
Psychological Science 21:1141-1149); Freytag 1863 (Die Technik des Dramas / Freytag's
pyramid: exposition, rising action, climax, falling action, denouement); Dawkins 1976
(The Selfish Gene, ch. 11, coining "meme"); Shifman 2014 (Memes in Digital Culture, MIT
Press; meme as a group of items sharing form/content/stance, propagated by imitation);
Berger & Milkman 2012 (What Makes Online Content Viral?, Journal of Marketing Research
49:192-205, high-arousal emotion drives sharing)."""
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
        "academic_fields": ["humor_studies", "narratology", "media_studies"],
        "subfields": subfields or ["incongruity_theory", "meme_studies", "comedic_structure"],
        "specialists": specialists or ["scene_composition_writer"],
        "contradictors": contradictors or ["random_mashup_generator_advocate"],
        "inputs": inputs or ["2-3 linked entities", "target audience brief"],
        "outputs": outputs or ["composed audience-fit scene"],
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

# ---- foundations: the entity bundle and audience targeting ----
N.append(node("ENTITY_BUNDLE", "two_to_three_related_entity_selection",
  "Select the 2-3 entities the scene will compose and verify they are genuinely related (shared domain, rivalry, category, or a known relationship) rather than arbitrary. The bundle is the scene's raw material: a pair anchors a single contrast, a triad adds a third beat (rule-of-three) without diluting focus. Reject bundles of 1 (no relation to play) or >=4 (cognitive overload, no clear contrast).",
  "foundations", [], ["CQ_01"],
  b(0.92, 0.82, 0.8, 0.5, 0.84, 0.8, 0.6, 0.7, 0.45, 0.78, 0.82, 0.9, 0.7, 0.18, 0.5, [0.2, 0.5],
    "entity-relatedness threshold or max-arity policy changes"),
  ["entity count 2..3", "pairwise relatedness check", "shared-frame identification"],
  ["entity disambiguation/linking internals", "rendering/asset production"],
  [pro("A vetted 2-3 entity bundle gives the scene exactly one contrast to exploit, the documented sweet spot for a single readable joke or beat",
       "pairing a budget airline with a luxury one sets up one clean status contrast")],
  [con("Forcing a relationship onto unrelated entities yields a non-sequitur the audience cannot resolve",
       "jamming a tax form and a dolphin together with no shared frame reads as noise")],
  ["arity exceeds 3 and the scene loses focus", "entities share no frame so no contrast exists"],
  ["bundle has 2 or 3 entities with an explicit shared frame and a named relationship"],
  ["relatedness threshold revised", "an arbitrary-mashup complaint is observed"],
  specialists=["scene_composition_writer", "content_strategist"]),)

N.append(node("AUDIENCE_FIT", "target_segment_reference_calibration",
  "Define the target audience segment and the reference frame the scene must speak in: the in-group knowledge, shared experiences, and platform norms the segment already holds. A scene lands only when its references resolve inside the audience's existing schema; a reference outside it reads as confusing or excluding. This node fixes the schema every later choice (format, references, register) is tuned against.",
  "foundations", ["ENTITY_BUNDLE"], ["CQ_02"],
  b(0.9, 0.84, 0.84, 0.55, 0.86, 0.86, 0.6, 0.66, 0.55, 0.74, 0.78, 0.9, 0.66, 0.2, 0.55, [0.24, 0.58],
    "target segment redefinition or platform-norm shift"),
  ["audience segment definition", "shared-schema inventory", "platform-norm capture"],
  ["paid-targeting/ad-buying mechanics", "post-hoc analytics attribution"],
  [pro("Calibrating to a concrete segment schema turns 'is this funny?' into 'is this funny to them?', a far more answerable question",
       "a scene for backend engineers can assume what a flaky CI pipeline feels like")],
  [con("Over-fitting to one narrow segment can shrink reach below a viable threshold",
       "an inside joke only 200 people understand caps the ceiling")],
  ["scene tuned to a schema the audience does not actually hold", "platform register mismatched to segment"],
  ["target segment named with an explicit shared-schema inventory the scene will use"],
  ["segment redefined", "a reference-miss (audience didn't get it) is reported"],
  specialists=["scene_composition_writer", "audience_researcher"]),)

# ---- humor engine: incongruity, benign violation, comedic structure ----
N.append(node("INCONGRUITY_RESOLUTION", "incongruity_then_resolution_humor_engine",
  "Engineer the humor as Suls' (1972) two-stage process: a setup creates an expectation, an incongruity violates it, and a punchline supplies a cognitive rule that resolves the surprise (the 'aha'). Humor lives in the gap being both surprising AND retrospectively makeable-sense-of; pure surprise without a resolving rule reads as random, and a fully expected turn reads as flat. For a 2-3 entity scene the incongruity is the unexpected relation drawn between the entities.",
  "humor", ["ENTITY_BUNDLE"], ["CQ_03"],
  b(0.94, 0.82, 0.84, 0.66, 0.88, 0.84, 0.62, 0.62, 0.6, 0.72, 0.76, 0.92, 0.62, 0.22, 0.58, [0.28, 0.66],
    "humor model revised or resolution-rule criteria change"),
  ["setup->incongruity->resolution arc", "expectation construction", "resolving-rule design"],
  ["delivery/timing of beats (own node)", "sensitivity adjudication"],
  [pro("A resolvable incongruity is the empirically recurring core of verbal humor: the audience does the closing work and rewards it with a laugh",
       "two CEOs feuding resolved by both using the same generic stock photo")],
  [con("If the resolving rule is too obscure the incongruity never resolves and the joke dies as confusion",
       "a punchline needing three layers of niche lore most readers lack")],
  ["surprise with no resolving rule (random)", "no surprise at all (predictable, flat)"],
  ["scene exhibits a clear setup, an incongruity, and a resolution the target audience can close"],
  ["humor model updated", "audience reports confusion rather than amusement"],
  specialists=["scene_composition_writer", "humor_analyst"]),)

N.append(node("BENIGN_VIOLATION", "benign_violation_safety_band",
  "Apply McGraw & Warren's (2010) Benign Violation Theory: humor arises when something is simultaneously a violation (of a norm, expectation, or dignity) AND benign (safe, playful, distant). Tune the scene into the band where the violation is felt but stays benign. Too benign = not funny (no violation); too severe / too close = offensive or distressing (not benign). This node sets the violation intensity that INCONGRUITY_RESOLUTION's turn carries.",
  "humor", ["INCONGRUITY_RESOLUTION"], ["CQ_03", "CQ_04"],
  b(0.86, 0.74, 0.76, 0.64, 0.86, 0.82, 0.6, 0.6, 0.6, 0.72, 0.74, 0.86, 0.6, 0.24, 0.58, [0.3, 0.68],
    "benign-violation calibration or norm context changes"),
  ["violation-intensity tuning", "psychological-distance levers", "benign-framing devices"],
  ["compliance/legal adjudication (delegated)", "platform policy enforcement"],
  [pro("The two dials (violation present, violation benign) explain both why something is funny and why it crosses a line, in one model",
       "a gentle roast of a beloved brand stays benign; the same words at a tragedy do not")],
  [con("Benignity is audience- and context-relative, so a violation benign to one segment is harmful to another",
       "an in-group joke that reads as cruel to outsiders who see the clip out of context")],
  ["violation absent so the scene is inert", "violation severe/too-close so it harms rather than amuses"],
  ["the scene's violation is detectable yet judged benign by the target segment in its likely contexts"],
  ["norm context shifts", "a benign-to-harmful boundary case is reported"],
  specialists=["scene_composition_writer", "humor_analyst"]),)

N.append(node("COMEDIC_STRUCTURE", "setup_turn_punchline_rule_of_three",
  "Impose comedic micro-structure on the scene: a setup that establishes the pattern, a turn that breaks it, and a punchline that lands the break; for triads use the rule of three (two beats establish a pattern, the third subverts it). Order and economy matter: the funniest word/image goes last, and every element not serving the turn is cut. This is the surface arrangement that delivers INCONGRUITY_RESOLUTION's gap.",
  "humor", ["INCONGRUITY_RESOLUTION"], ["CQ_05"],
  b(0.86, 0.76, 0.8, 0.6, 0.8, 0.78, 0.55, 0.66, 0.55, 0.76, 0.78, 0.86, 0.66, 0.2, 0.5, [0.24, 0.56],
    "comedic-structure conventions or platform format constraints change"),
  ["setup/turn/punchline ordering", "rule-of-three patterning", "economy/trimming pass"],
  ["the underlying incongruity design", "narrative arc (separate node)"],
  [pro("Putting the payload word/image last and cutting everything inessential maximizes the surprise impact of a fixed gap",
       "two normal panels then a third that violates the pattern lands harder than a front-loaded reveal")],
  [con("Rigid three-beat structure can feel formulaic when the audience has seen the template too often",
       "yet-another 'expectation / expectation / subversion' grid that reads as template fatigue")],
  ["punchline not in terminal position so impact leaks", "padding dilutes the turn"],
  ["scene has an identifiable setup, turn, and terminal punchline with no inessential elements"],
  ["platform format constraints change", "template-fatigue feedback observed"],
  specialists=["scene_composition_writer", "comedy_editor"]),)

# ---- scaffolding: meme format reuse ----
N.append(node("MEME_FORMAT_REUSE", "proven_template_as_scaffold",
  "Select a proven meme format (a recognizable template with conventional form, content slots, and stance) as the scaffold the scene is poured into. Per Shifman (2014), a meme is a group of items sharing form/content/stance propagated by imitation; reusing a live format gives the audience an instant frame and signals in-group fluency, lowering the comprehension cost of the joke. The format's known slots constrain where each entity and the turn go.",
  "scaffold", ["ENTITY_BUNDLE", "AUDIENCE_FIT"], ["CQ_06"],
  b(0.84, 0.8, 0.82, 0.58, 0.78, 0.82, 0.55, 0.66, 0.5, 0.76, 0.78, 0.84, 0.66, 0.2, 0.5, [0.22, 0.54],
    "format popularity decay or a new dominant template emerges"),
  ["format selection from live templates", "slot-to-entity mapping", "stance inheritance"],
  ["original-format invention", "asset rendering/typesetting"],
  [pro("A live, recognized format is a comprehension shortcut: the audience already knows how to read it, so the scene spends its budget on the turn not the frame",
       "the 'two buttons' sweating-choice format instantly frames a dilemma between two entities")],
  [con("Formats decay; a stale or overused template signals lateness and lowers shareability",
       "using a meme format six months past its peak reads as out-of-touch")],
  ["chosen format is dead/overused", "entities forced into slots the format does not support"],
  ["format is currently live for the segment and its slots map cleanly to the entities and the turn"],
  ["format popularity decays", "a fresher dominant template appears"],
  specialists=["scene_composition_writer", "meme_trend_analyst"]),)

# ---- narrative beats ----
N.append(node("NARRATIVE_BEATS", "micro_story_arc_beats",
  "Shape the scene as a compressed micro-story using Freytag's (1863) dramatic structure scaled to a single unit: exposition (who/what), rising tension (the entities collide), climax (the turn), and a brief denouement/button. Even a one-panel or single-line scene benefits from an implied arc; beats give the audience a tiny story to inhabit, which raises engagement beyond a static juxtaposition. Beats carry the incongruity through time rather than presenting it flat.",
  "narrative", ["ENTITY_BUNDLE"], ["CQ_07"],
  b(0.8, 0.74, 0.8, 0.6, 0.74, 0.78, 0.55, 0.66, 0.5, 0.76, 0.78, 0.8, 0.66, 0.2, 0.48, [0.24, 0.56],
    "narrative-beat schema or platform length norms change"),
  ["exposition/rising/climax/button beats", "implied-arc design", "beat-to-entity assignment"],
  ["multi-scene story arcs (out of scope)", "the joke's logical resolution (humor node)"],
  [pro("A micro-arc gives even a single image a beginning-middle-end the viewer completes, raising dwell and recall over a flat juxtaposition",
       "a three-panel build where tension rises before the climactic turn outperforms a single static contrast")],
  [con("Forcing a full arc into a tight format can bloat the scene past the platform's attention window",
       "a five-beat story stuffed into a format meant for a single punch reads as slow")],
  ["arc bloats past the attention window", "beats present the contrast flat with no rise"],
  ["scene has at least an implied exposition, climax (turn), and button within the format's length"],
  ["platform length norms change", "drop-off-before-payoff is observed"],
  specialists=["scene_composition_writer", "story_editor"]),)

# ---- cultural reference & relatability tradeoff ----
N.append(node("CULTURAL_REFERENCE", "local_in_joke_resonance",
  "Layer in cultural and local references (in-jokes, regional touchstones, subculture lore) that the target segment recognizes, raising resonance and the feeling that the scene was made for them. A landed reference is a strong identity and shareability signal; a missed reference is dead weight or, worse, an exclusion cue. References must be checked against AUDIENCE_FIT's schema, not the author's own.",
  "resonance", ["AUDIENCE_FIT"], ["CQ_08"],
  b(0.78, 0.76, 0.82, 0.56, 0.78, 0.82, 0.55, 0.62, 0.55, 0.74, 0.76, 0.78, 0.62, 0.22, 0.5, [0.26, 0.6],
    "the referenced cultural touchstone ages out or shifts meaning"),
  ["reference selection from segment lore", "recognition-rate estimate", "exclusion-risk check"],
  ["broad-appeal copywriting (relatability node)", "legal clearance of referenced IP"],
  [pro("A reference the segment owns acts as an identity handshake: getting it is itself part of the reward and a reason to share",
       "a niche reference to a specific in-community catchphrase signals 'this creator is one of us'")],
  [con("References are perishable and segment-bound; one that ages out or excludes outsiders caps reach and can read as gatekeeping",
       "a reference to last year's drama that newer audience members never saw")],
  ["reference recognized by too few to matter", "reference reads as exclusionary to adjacent audience"],
  ["each reference has an estimated recognition rate above threshold for the target segment"],
  ["the touchstone ages out", "a reference-exclusion complaint is reported"],
  specialists=["scene_composition_writer", "culture_researcher"]),)

N.append(node("RELATABILITY_VS_NICHE", "broad_relatability_versus_insider_appeal",
  "Manage the core tradeoff between broad relatability (a universal experience many recognize) and insider/niche appeal (deep resonance for few). Broad scenes maximize reach but risk blandness; niche scenes maximize intensity and shareability-per-viewer but cap the ceiling. This node sets the target point on that curve given the segment and goal, and decides how many references are insider vs universal.",
  "resonance", ["AUDIENCE_FIT", "CULTURAL_REFERENCE"], ["CQ_08", "CQ_09"],
  b(0.82, 0.8, 0.8, 0.6, 0.8, 0.84, 0.58, 0.62, 0.6, 0.72, 0.74, 0.82, 0.62, 0.22, 0.54, [0.28, 0.64],
    "growth goal shifts between reach and depth, moving the target point"),
  ["reach-vs-depth target point", "insider/universal reference mix", "ceiling estimate"],
  ["paid distribution decisions", "specific reference authoring"],
  [pro("Naming the target point explicitly prevents accidental drift to bland-broad or impenetrable-niche and aligns the scene with the goal",
       "a growth-phase scene leans relatable; a loyalty-phase scene leans insider")],
  [con("The optimum is unstable: the same scene can be too niche for reach goals and too broad for community goals",
       "a mid-niche scene satisfies neither a viral push nor a core-fan reward")],
  ["scene drifts to bland universality with no edge", "scene so niche the reach ceiling is unviable"],
  ["scene's insider/universal mix matches the stated reach-vs-depth goal for this segment"],
  ["growth goal shifts reach<->depth", "reach ceiling proves too low for the goal"],
  specialists=["scene_composition_writer", "content_strategist"]),)

# ---- sensitivity / safety ----
N.append(node("SENSITIVITY_AWARENESS", "harm_framing_avoidance_and_escalation",
  "Screen the scene for harm: avoid framing built on tragedy, ethnic/religious denigration, protected-class mockery, or punching-down at the vulnerable, and route any hard call to compliance rather than deciding it in the writing seat. This is the benign-side guardrail on BENIGN_VIOLATION: it does not kill edge, it relocates the violation off harmful targets and escalates ambiguous cases instead of guessing.",
  "safety", ["BENIGN_VIOLATION", "AUDIENCE_FIT"], ["CQ_04", "CQ_10"],
  b(0.9, 0.78, 0.78, 0.6, 0.92, 0.84, 0.78, 0.6, 0.62, 0.74, 0.78, 0.9, 0.6, 0.2, 0.7, [0.28, 0.62],
    "platform policy, legal guidance, or protected-class scope changes"),
  ["harm-target screening", "punching-up vs down check", "escalation-to-compliance trigger"],
  ["final legal/compliance ruling (delegated to compliance)", "the humor mechanism itself"],
  [pro("Relocating the violation off protected/tragic targets preserves the joke's edge while removing the harm, and escalation prevents a single writer absorbing a legal/ethical call",
       "redirect a roast from a person's ethnicity to their public product decision keeps it benign")],
  [con("Over-cautious screening can sand off all edge and produce inert, forgettable content",
       "removing every conceivable violation leaves a scene with no turn and no humor")],
  ["harmful framing ships undetected", "ambiguous case decided in the writing seat instead of escalated"],
  ["scene has no tragedy/ethnic/religious/punching-down framing and all hard calls are escalated to compliance"],
  ["platform policy or legal guidance changes", "a harm complaint or near-miss is reported"],
  specialists=["scene_composition_writer", "trust_and_safety_reviewer"],
  contradictors=["edge_at_all_costs_advocate"]),)

# ---- shareability & timing ----
N.append(node("SHAREABILITY_DRIVERS", "emotion_identity_surprise_share_levers",
  "Maximize the levers that make people share, per Berger & Milkman (2012): high-arousal emotion (awe, amusement, anger, anxiety) drives sharing more than low-arousal states, and content that lets the sharer express identity or signal in-group membership travels further. Surprise (the resolved incongruity) is itself a high-arousal trigger. This node audits the scene for at least one strong share lever and strengthens the weakest.",
  "distribution", ["INCONGRUITY_RESOLUTION", "CULTURAL_REFERENCE"], ["CQ_11"],
  b(0.84, 0.86, 0.82, 0.58, 0.78, 0.82, 0.55, 0.62, 0.55, 0.74, 0.76, 0.84, 0.62, 0.22, 0.5, [0.26, 0.6],
    "platform sharing mechanics or measured share-driver weights change"),
  ["high-arousal emotion audit", "identity/in-group expression lever", "surprise-strength check"],
  ["paid amplification", "engagement-farming tactics"],
  [pro("Auditing against named, evidence-backed share drivers (arousal, identity, surprise) turns 'will it spread?' into a checklist the scene can be strengthened against",
       "amusement plus an identity-signaling in-joke gives two reinforcing share reasons")],
  [con("Optimizing purely for shareability can push toward outrage-bait that wins shares but damages brand trust",
       "an anger-driven scene spreads but attaches the brand to a fight")],
  ["scene has no strong share lever (low arousal, no identity hook)", "share lever is outrage that harms trust"],
  ["scene exhibits at least one strong, non-harmful share lever from emotion/identity/surprise"],
  ["platform sharing mechanics change", "a high-quality scene underperforms on shares"],
  specialists=["scene_composition_writer", "growth_analyst"]),)

N.append(node("TIMING_RELEVANCE", "current_event_window_riding",
  "Decide whether and how to tie the scene to a current event, trend, or moment to ride its attention window. Timely scenes borrow the event's existing attention and feel alive, but the window is short and tying to a sensitive event (tragedy, controversy) multiplies sensitivity risk. This node sizes the window, sets a freshness deadline, and gates topical ties through SENSITIVITY_AWARENESS.",
  "distribution", ["AUDIENCE_FIT", "SENSITIVITY_AWARENESS"], ["CQ_12"],
  b(0.8, 0.82, 0.78, 0.56, 0.84, 0.8, 0.6, 0.6, 0.58, 0.72, 0.74, 0.8, 0.6, 0.24, 0.56, [0.3, 0.66],
    "the riding event's window closes or its valence shifts"),
  ["event-window sizing", "freshness deadline", "topical-tie sensitivity gate"],
  ["scheduling/publishing infrastructure", "the joke construction itself"],
  [pro("Riding a live window borrows attention the scene would otherwise have to earn cold, sharply raising reach for time-bound effort",
       "a scene tied to a same-day product launch rides the launch's search and feed spike")],
  [con("Topicality is perishable and risky: a missed window wastes the work and a tie to a sensitive event can blow up",
       "publishing a 'trend' joke a day late, or tying levity to an unfolding tragedy")],
  ["window closes before publish", "topical tie attaches the scene to a sensitive event"],
  ["any topical tie has a sized window, a freshness deadline, and passed the sensitivity gate"],
  ["the event window closes", "a topical tie's valence shifts negative"],
  specialists=["scene_composition_writer", "trends_editor"]),)

# ---- assembly / register / contrast ----
N.append(node("ENTITY_CONTRAST_AXIS", "shared_frame_contrast_axis_selection",
  "Choose the single contrast axis the entities will be played against (status, scale, competence, era, values, expectation) so the relation drawn between them is sharp and one-dimensional. A clear axis is what makes the incongruity legible; multiple competing axes muddy the turn. For a triad the axis must order all three (two conform, one breaks).",
  "assembly", ["ENTITY_BUNDLE", "AUDIENCE_FIT"], ["CQ_01", "CQ_03"],
  b(0.84, 0.74, 0.76, 0.58, 0.8, 0.8, 0.55, 0.64, 0.55, 0.76, 0.78, 0.84, 0.64, 0.2, 0.5, [0.24, 0.56],
    "contrast-axis taxonomy or entity attributes change"),
  ["single-axis selection", "axis-orders-all-entities check", "axis legibility test"],
  ["rendering of the contrast", "downstream share optimization"],
  [pro("One sharp axis makes the incongruity instantly legible and gives the punchline a clean fulcrum",
       "playing two phones purely on price makes the status flip read in a glance")],
  [con("A single axis can flatten genuinely multidimensional entities into a reductive comparison",
       "reducing two rich brands to one price point may miss the funnier values contrast")],
  ["multiple competing axes muddy the turn", "axis fails to order a triad's third entity"],
  ["exactly one contrast axis is chosen and it orders all 2-3 entities legibly"],
  ["contrast taxonomy changes", "audience misreads which axis is in play"],
  specialists=["scene_composition_writer"]),)

N.append(node("TONE_REGISTER", "voice_and_register_fit",
  "Set the tone and register (deadpan, absurd, wholesome, sarcastic, earnest) to match both the audience schema and the platform's norms, and hold it consistently across the scene. Register is the carrier wave for the joke: a turn that is funny in a deadpan register can fall flat or read as mean in a sarcastic one. Register must agree with BENIGN_VIOLATION (a harsh register can tip a benign violation into a harmful one).",
  "assembly", ["AUDIENCE_FIT", "BENIGN_VIOLATION"], ["CQ_02", "CQ_04"],
  b(0.78, 0.72, 0.76, 0.56, 0.78, 0.8, 0.55, 0.64, 0.55, 0.74, 0.76, 0.78, 0.64, 0.2, 0.48, [0.24, 0.56],
    "platform register norms or brand voice guidelines change"),
  ["register selection", "platform-norm fit", "register consistency hold"],
  ["the joke's logical content", "asset/visual styling"],
  [pro("A register matched to segment and platform makes the same joke land warm instead of cold and keeps a violation benign",
       "a wholesome-deadpan register keeps a roast affectionate rather than cruel")],
  [con("Register and content can fight: an edgy turn in an earnest register reads as tonally confused",
       "a sarcastic punchline inside a sincere brand voice jars")],
  ["register tips a benign violation into harm", "register drifts mid-scene"],
  ["one register is chosen, fits segment and platform, and is held consistently across the scene"],
  ["platform register norms shift", "brand voice guidelines change"],
  specialists=["scene_composition_writer", "brand_voice_editor"]),)

N.append(node("SCENE_ASSEMBLY", "compose_entities_into_single_scene",
  "Compose the vetted entities, chosen format, contrast axis, beats, and register into one coherent scene draft: place each entity in its format slot, route the contrast through the comedic structure and narrative beats, and ensure the parts cohere into a single readable unit rather than a collage. This is the integration step where format slots, beats, and the turn are reconciled into one artifact.",
  "assembly", ["MEME_FORMAT_REUSE", "COMEDIC_STRUCTURE", "NARRATIVE_BEATS", "ENTITY_CONTRAST_AXIS", "TONE_REGISTER"], ["CQ_05", "CQ_06"],
  b(0.9, 0.82, 0.84, 0.66, 0.84, 0.85, 0.62, 0.6, 0.6, 0.72, 0.74, 0.9, 0.6, 0.22, 0.56, [0.28, 0.64],
    "assembly contract between format/beats/structure changes"),
  ["entity-to-slot placement", "beat/structure/format reconciliation", "single-unit coherence check"],
  ["final sensitivity sign-off (separate gate)", "distribution scheduling"],
  [pro("A single integration step forces the format, beats, axis, and register to agree before any polish, catching incoherent collages early",
       "the contrast axis, the format's slots, and the punchline position all line up in one draft")],
  [con("Premature assembly before the format or axis is settled produces churn as the draft is rebuilt repeatedly",
       "assembling around a format that later proves dead wastes the layout work")],
  ["parts read as a collage not a unit", "format slots and beats contradict each other"],
  ["a single coherent scene draft exists with every entity placed and beats/format/axis reconciled"],
  ["assembly contract changes", "a draft reads as incoherent in review"],
  specialists=["scene_composition_writer", "content_producer"]),)

# ---- evaluation / gates ----
N.append(node("CLARITY_READABILITY", "instant_read_comprehension_check",
  "Verify the scene is comprehended on first contact within the platform's scroll-speed window: the entities are identifiable, the contrast axis is obvious, and the turn resolves without external explanation. A joke that needs a caption explaining it has failed comprehension. This gate measures whether the incongruity actually resolves for a fresh viewer, not just for the author.",
  "evaluation", ["SCENE_ASSEMBLY"], ["CQ_05", "CQ_13"],
  b(0.84, 0.78, 0.84, 0.56, 0.82, 0.78, 0.55, 0.64, 0.5, 0.78, 0.8, 0.84, 0.64, 0.2, 0.5, [0.22, 0.52],
    "platform scroll-speed norms or comprehension threshold changes"),
  ["first-contact comprehension test", "entity-identifiability check", "no-caption-needed test"],
  ["humor quality scoring (separate)", "distribution"],
  [pro("A fresh-viewer comprehension gate catches author's-curse failures where the scene is clear only to the person who built it",
       "a test reader who gets the turn in two seconds without explanation passes the gate")],
  [con("Optimizing purely for instant clarity can strip the rewarding layer that makes a scene re-shareable",
       "over-explaining the contrast removes the satisfying 'figure it out' beat")],
  ["scene needs an explanatory caption to land", "entities unidentifiable at scroll speed"],
  ["a fresh target-segment viewer comprehends entities and turn within the scroll window with no caption"],
  ["scroll-speed norms change", "comprehension-failure feedback is reported"],
  specialists=["scene_composition_writer", "qa_reviewer"]),)

N.append(node("HUMOR_QUALITY_SCORE", "funniness_and_resolution_scoring",
  "Score the assembled scene's humor quality: does the incongruity actually resolve (Suls), does the violation read as benign (McGraw & Warren), is the punchline in terminal position, and does a target-segment sample find it amusing rather than confusing or flat. Produces a go/no-go and a weakest-link diagnosis routed back to the humor or assembly nodes for one revision pass.",
  "evaluation", ["CLARITY_READABILITY", "INCONGRUITY_RESOLUTION", "BENIGN_VIOLATION"], ["CQ_03", "CQ_13"],
  b(0.86, 0.8, 0.82, 0.62, 0.84, 0.82, 0.58, 0.6, 0.58, 0.74, 0.76, 0.86, 0.6, 0.22, 0.54, [0.28, 0.62],
    "humor scoring rubric or sample-panel composition changes"),
  ["resolution-present check", "benign-violation check", "sample amusement rate"],
  ["legal sign-off", "distribution timing"],
  [pro("A rubric tied to the two humor theories plus a sample panel converts a subjective laugh into a repeatable, diagnosable score",
       "a scene that confuses 6 of 10 testers fails resolution and is sent back to the incongruity node")],
  [con("Small or unrepresentative sample panels give noisy scores that can reject good scenes or pass weak ones",
       "a panel of 3 colleagues over-rates an in-house in-joke")],
  ["weak scene passes on a biased panel", "good scene rejected on a noisy small sample"],
  ["scene's incongruity resolves, violation is benign, and a representative sample finds it amusing"],
  ["scoring rubric changes", "sample-panel bias is detected"],
  specialists=["scene_composition_writer", "humor_analyst"]),)

N.append(node("SENSITIVITY_GATE", "final_pre_publish_harm_gate",
  "Run the final pre-publish harm gate on the fully assembled scene: re-screen the integrated artifact (not just the raw idea) for harmful framing, verify any topical tie is still benign, and require an explicit compliance sign-off for any escalated item before publish. This gate is the hard stop; a scene that is funny but fails here does not ship.",
  "evaluation", ["SCENE_ASSEMBLY", "SENSITIVITY_AWARENESS", "TIMING_RELEVANCE"], ["CQ_10", "CQ_12"],
  b(0.92, 0.8, 0.78, 0.6, 0.94, 0.84, 0.82, 0.6, 0.62, 0.74, 0.78, 0.92, 0.6, 0.2, 0.72, [0.28, 0.6],
    "compliance policy, legal guidance, or escalation protocol changes"),
  ["integrated-artifact re-screen", "topical-tie re-check", "compliance sign-off enforcement"],
  ["the funniness judgment (humor node)", "creative ideation"],
  [pro("Screening the final integrated artifact catches harms that only emerge from the combination of parts, and a hard sign-off prevents a funny-but-harmful scene shipping on momentum",
       "two individually fine elements that combine into a slur are caught only at the assembled stage")],
  [con("A hard gate at the end can cause expensive late rework if sensitivity was ignored upstream",
       "a fully produced scene killed at the gate wastes all the assembly effort")],
  ["harm emergent from combination ships", "escalated item published without sign-off"],
  ["assembled scene re-screened clean, topical tie re-verified benign, and all escalations signed off before publish"],
  ["compliance policy changes", "a post-publish harm incident occurs"],
  specialists=["scene_composition_writer", "trust_and_safety_reviewer", "compliance_officer"],
  contradictors=["ship_it_anyway_advocate"]),)

N.append(node("SHAREABILITY_FORECAST", "pre_publish_spread_forecast",
  "Forecast the scene's likely spread before publish by auditing it against the share drivers and timing window: count the strong, non-harmful share levers present, estimate the segment recognition rate of its references, and confirm the timing window is still open. Produces a spread estimate and a final strengthen-or-ship decision; it does not override the sensitivity gate.",
  "evaluation", ["SHAREABILITY_DRIVERS", "TIMING_RELEVANCE", "HUMOR_QUALITY_SCORE"], ["CQ_11", "CQ_13"],
  b(0.8, 0.84, 0.8, 0.58, 0.78, 0.82, 0.58, 0.6, 0.55, 0.74, 0.76, 0.8, 0.6, 0.24, 0.52, [0.28, 0.62],
    "share-driver weights or platform distribution mechanics change"),
  ["share-lever count", "reference recognition estimate", "window-still-open check"],
  ["paid amplification spend", "the sensitivity hard stop (separate gate)"],
  [pro("A pre-publish forecast against named drivers lets a weak-but-fixable scene be strengthened rather than published flat or scrapped",
       "a scene with high amusement but no identity hook is sent back to add one before publishing")],
  [con("Forecasts from heuristic drivers are uncertain and can over-promise reach, inviting disappointment or over-investment",
       "a scene that checks every share box still flops because the timing closed")],
  ["forecast over-promises and the scene flops", "forecast used to override the sensitivity gate"],
  ["scene has counted share levers, an estimated recognition rate, and a confirmed-open window before the ship decision"],
  ["share-driver weights change", "forecasts diverge sharply from realized spread"],
  specialists=["scene_composition_writer", "growth_analyst"]),)

N.append(node("SCENE_HANDOFF", "publish_ready_scene_package",
  "Package the gated scene for publish: the final scene artifact, the entity bundle and contrast axis used, the format and register, the sensitivity sign-off record, the timing window/deadline, and the forecast rationale. The handoff makes the scene reproducible, auditable, and schedulable, and records why each major creative and safety choice was made.",
  "handoff", ["SENSITIVITY_GATE", "SHAREABILITY_FORECAST", "HUMOR_QUALITY_SCORE"], ["CQ_13", "CQ_14"],
  b(0.78, 0.8, 0.78, 0.52, 0.78, 0.78, 0.6, 0.7, 0.42, 0.8, 0.82, 0.78, 0.7, 0.16, 0.44, [0.18, 0.46],
    "handoff package schema or publishing-system contract changes"),
  ["final artifact packaging", "decision-provenance record", "publish window/deadline"],
  ["actual publishing/scheduling execution", "post-publish analytics"],
  [pro("A complete handoff package makes the scene reproducible and auditable: every safety and creative call is traceable after the fact",
       "a reviewer can reconstruct why a topical tie was approved from the sign-off record")],
  [con("Over-heavy handoff documentation can slow time-sensitive scenes past their window",
       "a same-day trend scene delayed by paperwork misses the spike")],
  ["scene ships without a sign-off record", "package omits the timing deadline"],
  ["handoff package contains the artifact, choices, sensitivity sign-off, timing window, and forecast rationale"],
  ["handoff schema changes", "a published scene cannot be audited from its package"],
  specialists=["scene_composition_writer", "content_producer"]),)

# ---------------- competency questions (14) ----------------
CQ = [
 ("CQ_01", "How are the 2-3 entities selected, verified related, and given a single contrast axis?",
  ["nodes", "glossary"], "ENTITY_BUNDLE selects 2-3 related entities; ENTITY_CONTRAST_AXIS fixes one contrast axis",
  ["ENTITY_BUNDLE", "ENTITY_CONTRAST_AXIS"]),
 ("CQ_02", "How is the target audience segment and its reference schema and register defined?",
  ["nodes"], "AUDIENCE_FIT fixes the segment schema; TONE_REGISTER sets a matching register",
  ["AUDIENCE_FIT", "TONE_REGISTER"]),
 ("CQ_03", "How is the humor engineered as a resolvable incongruity over the entities' contrast?",
  ["nodes"], "INCONGRUITY_RESOLUTION builds setup->incongruity->resolution over ENTITY_CONTRAST_AXIS",
  ["INCONGRUITY_RESOLUTION", "ENTITY_CONTRAST_AXIS", "HUMOR_QUALITY_SCORE"]),
 ("CQ_04", "How is the violation kept benign and aligned with the chosen register?",
  ["nodes", "conflict_axes"], "BENIGN_VIOLATION sets violation intensity; SENSITIVITY_AWARENESS and TONE_REGISTER keep it benign",
  ["BENIGN_VIOLATION", "SENSITIVITY_AWARENESS", "TONE_REGISTER"]),
 ("CQ_05", "How is the comedic micro-structure (setup/turn/punchline, rule of three) arranged and verified readable?",
  ["nodes", "workflow"], "COMEDIC_STRUCTURE orders the beats; CLARITY_READABILITY verifies first-contact comprehension",
  ["COMEDIC_STRUCTURE", "CLARITY_READABILITY", "SCENE_ASSEMBLY"]),
 ("CQ_06", "How is a proven meme format chosen and the entities mapped into its slots?",
  ["nodes"], "MEME_FORMAT_REUSE selects a live format and maps entities to slots; SCENE_ASSEMBLY integrates it",
  ["MEME_FORMAT_REUSE", "SCENE_ASSEMBLY"]),
 ("CQ_07", "How is a micro-narrative arc shaped over the scene?",
  ["nodes"], "NARRATIVE_BEATS imposes exposition/rising/climax/button on the scene",
  ["NARRATIVE_BEATS"]),
 ("CQ_08", "How are cultural references chosen and the relatability-vs-niche tradeoff set?",
  ["nodes", "conflict_axes"], "CULTURAL_REFERENCE selects segment references; RELATABILITY_VS_NICHE sets the reach-vs-depth point",
  ["CULTURAL_REFERENCE", "RELATABILITY_VS_NICHE"]),
 ("CQ_09", "How is the reach ceiling versus per-viewer intensity tradeoff decided?",
  ["nodes"], "RELATABILITY_VS_NICHE sets the target point on the reach-vs-depth curve",
  ["RELATABILITY_VS_NICHE"]),
 ("CQ_10", "How is harmful framing avoided and which calls are escalated to compliance?",
  ["nodes", "edge_cases"], "SENSITIVITY_AWARENESS screens harm targets and escalates; SENSITIVITY_GATE is the final hard stop",
  ["SENSITIVITY_AWARENESS", "SENSITIVITY_GATE"]),
 ("CQ_11", "How is the scene optimized for the empirical share drivers (emotion, identity, surprise)?",
  ["nodes"], "SHAREABILITY_DRIVERS audits arousal/identity/surprise levers; SHAREABILITY_FORECAST counts them pre-publish",
  ["SHAREABILITY_DRIVERS", "SHAREABILITY_FORECAST"]),
 ("CQ_12", "How is timing/current-event relevance decided and gated for sensitivity?",
  ["nodes"], "TIMING_RELEVANCE sizes the window and gates topical ties; SENSITIVITY_GATE re-verifies them",
  ["TIMING_RELEVANCE", "SENSITIVITY_GATE"]),
 ("CQ_13", "How is the assembled scene scored for clarity, humor, and spread before publish?",
  ["nodes", "workflow"], "CLARITY_READABILITY, HUMOR_QUALITY_SCORE and SHAREABILITY_FORECAST gate the assembled scene",
  ["CLARITY_READABILITY", "HUMOR_QUALITY_SCORE", "SHAREABILITY_FORECAST", "SCENE_HANDOFF"]),
 ("CQ_14", "How is the publish-ready scene packaged so its creative and safety choices are auditable?",
  ["nodes"], "SCENE_HANDOFF packages the artifact, choices, sign-off, timing and forecast rationale",
  ["SCENE_HANDOFF"]),
]
CQS = [{"id": i, "question": q, "must_be_answerable_from": m, "acceptance_condition": a, "covered_by": c}
       for (i, q, m, a, c) in CQ]

# consolidate node->CQ references onto the 14-CQ set
CQ_MAP = {
 "ENTITY_BUNDLE": ["CQ_01"], "AUDIENCE_FIT": ["CQ_02"], "INCONGRUITY_RESOLUTION": ["CQ_03"],
 "BENIGN_VIOLATION": ["CQ_03", "CQ_04"], "COMEDIC_STRUCTURE": ["CQ_05"], "MEME_FORMAT_REUSE": ["CQ_06"],
 "NARRATIVE_BEATS": ["CQ_07"], "CULTURAL_REFERENCE": ["CQ_08"], "RELATABILITY_VS_NICHE": ["CQ_08", "CQ_09"],
 "SENSITIVITY_AWARENESS": ["CQ_04", "CQ_10"], "SHAREABILITY_DRIVERS": ["CQ_11"], "TIMING_RELEVANCE": ["CQ_12"],
 "ENTITY_CONTRAST_AXIS": ["CQ_01", "CQ_03"], "TONE_REGISTER": ["CQ_02", "CQ_04"],
 "SCENE_ASSEMBLY": ["CQ_05", "CQ_06"], "CLARITY_READABILITY": ["CQ_05", "CQ_13"],
 "HUMOR_QUALITY_SCORE": ["CQ_03", "CQ_13"], "SENSITIVITY_GATE": ["CQ_10", "CQ_12"],
 "SHAREABILITY_FORECAST": ["CQ_11", "CQ_13"], "SCENE_HANDOFF": ["CQ_13", "CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

# ---------------- glossary ----------------
GL = [
 ("incongruity_resolution", "the two-stage humor process where a setup creates an expectation, an incongruity violates it, and a punchline supplies a rule that resolves the surprise (Suls 1972)",
  ["two_stage_humor", "surprise_then_sense"], ["pure_shock", "random_juxtaposition"], ["INCONGRUITY_RESOLUTION", "HUMOR_QUALITY_SCORE"]),
 ("benign_violation", "the condition under which something is funny: it is a violation yet simultaneously benign/safe (McGraw & Warren 2010)",
  ["safe_transgression"], ["pure_offense", "pure_safety"], ["BENIGN_VIOLATION", "SENSITIVITY_AWARENESS"]),
 ("rule_of_three", "a comedic pattern where two beats establish an expectation and a third subverts it",
  ["triadic_structure", "comic_triple"], ["two_beat_setup"], ["COMEDIC_STRUCTURE", "ENTITY_BUNDLE"]),
 ("meme_format", "a recognizable template with conventional form, content slots, and stance, propagated by imitation (Shifman 2014)",
  ["template", "format_scaffold"], ["original_composition"], ["MEME_FORMAT_REUSE", "SCENE_ASSEMBLY"]),
 ("freytag_pyramid", "the dramatic arc of exposition, rising action, climax, falling action, denouement (Freytag 1863)",
  ["dramatic_structure", "story_arc"], ["flat_juxtaposition"], ["NARRATIVE_BEATS"]),
 ("contrast_axis", "the single shared-frame dimension (status, scale, era, values) the entities are played against",
  ["comparison_axis", "tension_axis"], ["multi_axis_compare"], ["ENTITY_CONTRAST_AXIS", "INCONGRUITY_RESOLUTION"]),
 ("audience_schema", "the in-group knowledge, experiences, and platform norms the target segment already holds",
  ["reference_frame", "in_group_knowledge"], ["author_schema"], ["AUDIENCE_FIT", "CULTURAL_REFERENCE"]),
 ("share_driver", "an attribute that raises sharing likelihood: high-arousal emotion, identity expression, or surprise (Berger & Milkman 2012)",
  ["virality_lever"], ["passive_view_metric"], ["SHAREABILITY_DRIVERS", "SHAREABILITY_FORECAST"]),
 ("punching_down", "directing a violation at a less-powerful or vulnerable target, a harmful (non-benign) framing to avoid",
  ["downward_mockery"], ["punching_up"], ["SENSITIVITY_AWARENESS", "SENSITIVITY_GATE"]),
 ("attention_window", "the short period during which a current event or trend carries borrowable attention",
  ["trend_window", "topical_window"], ["evergreen"], ["TIMING_RELEVANCE", "SHAREABILITY_FORECAST"]),
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
              "risk_of_conflict": "unmanaged tension degrades the scene", "example": "see resolution_rule"})
def rel(f, t, et, rs, cc=0.7, cp=0.2, erc=0.3, why=""):
    E.append({"from": f, "to": t, "edge_type": et, "relation_strength": rs, "signed_tension": 0.0,
              "causal_confidence": cc, "conflict_probability": cp, "expected_rework_cost": erc,
              "why_related": why or f"{f} {et} {t}", "benefit_of_coupling": "coordinated behavior",
              "risk_of_conflict": "inconsistency if uncoordinated", "example": f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies)
dep("ENTITY_BUNDLE", "AUDIENCE_FIT", 0.84)
dep("ENTITY_BUNDLE", "INCONGRUITY_RESOLUTION", 0.88)
dep("ENTITY_BUNDLE", "NARRATIVE_BEATS", 0.76)
dep("INCONGRUITY_RESOLUTION", "BENIGN_VIOLATION", 0.84)
dep("INCONGRUITY_RESOLUTION", "COMEDIC_STRUCTURE", 0.82)
dep("ENTITY_BUNDLE", "MEME_FORMAT_REUSE", 0.78)
dep("AUDIENCE_FIT", "CULTURAL_REFERENCE", 0.8)
dep("AUDIENCE_FIT", "RELATABILITY_VS_NICHE", 0.8)
dep("BENIGN_VIOLATION", "SENSITIVITY_AWARENESS", 0.84)
dep("INCONGRUITY_RESOLUTION", "SHAREABILITY_DRIVERS", 0.78)
dep("SENSITIVITY_AWARENESS", "TIMING_RELEVANCE", 0.76)
dep("ENTITY_BUNDLE", "ENTITY_CONTRAST_AXIS", 0.84)
dep("AUDIENCE_FIT", "TONE_REGISTER", 0.78)
dep("MEME_FORMAT_REUSE", "SCENE_ASSEMBLY", 0.82)
dep("COMEDIC_STRUCTURE", "SCENE_ASSEMBLY", 0.82)
dep("NARRATIVE_BEATS", "SCENE_ASSEMBLY", 0.78)
dep("ENTITY_CONTRAST_AXIS", "SCENE_ASSEMBLY", 0.8)
dep("TONE_REGISTER", "SCENE_ASSEMBLY", 0.76)
dep("SCENE_ASSEMBLY", "CLARITY_READABILITY", 0.84)
dep("CLARITY_READABILITY", "HUMOR_QUALITY_SCORE", 0.82)
dep("SCENE_ASSEMBLY", "SENSITIVITY_GATE", 0.82)
dep("SENSITIVITY_AWARENESS", "SENSITIVITY_GATE", 0.84)
dep("SHAREABILITY_DRIVERS", "SHAREABILITY_FORECAST", 0.8)
dep("HUMOR_QUALITY_SCORE", "SHAREABILITY_FORECAST", 0.74)
dep("SENSITIVITY_GATE", "SCENE_HANDOFF", 0.84)
dep("SHAREABILITY_FORECAST", "SCENE_HANDOFF", 0.78)

# cross-cutting non-dependency edges
rel("ENTITY_CONTRAST_AXIS", "INCONGRUITY_RESOLUTION", "causal", 0.8,
    why="the chosen contrast axis is the dimension along which the incongruity is drawn")
rel("HUMOR_QUALITY_SCORE", "COMEDIC_STRUCTURE", "feedback", 0.76,
    why="a failed humor score routes a revision back to the comedic structure")
rel("CLARITY_READABILITY", "MEME_FORMAT_REUSE", "feedback", 0.72,
    why="a comprehension failure can send the scene back to pick a clearer format")
rel("RELATABILITY_VS_NICHE", "SHAREABILITY_DRIVERS", "constraint", 0.74,
    why="the reach-vs-depth point constrains which share levers (broad vs identity) to lean on")
rel("TONE_REGISTER", "SENSITIVITY_AWARENESS", "constraint", 0.74,
    why="a harsh register can tip a benign violation toward harm, constraining the safe register set")
rel("NARRATIVE_BEATS", "COMEDIC_STRUCTURE", "similarity", 0.7,
    why="narrative beats and comedic beats are coordinated arrangements of the same scene over time")
rel("SHAREABILITY_FORECAST", "SENSITIVITY_GATE", "constraint", 0.72,
    why="the spread forecast must never override the sensitivity hard stop")

# conflict edges (negative signed_tension + resolution_rule)
conf("BENIGN_VIOLATION", "SENSITIVITY_AWARENESS", 0.74, -0.6,
  "keep the violation that creates the humor but relocate it off protected/tragic/vulnerable targets; escalate ambiguous cases to compliance rather than removing all edge",
  "the edge that makes a violation funny conflicts with the safety requirement that it stay benign and harm-free")
conf("RELATABILITY_VS_NICHE", "SHAREABILITY_DRIVERS", 0.7, -0.5,
  "set the reach-vs-depth point from the stated goal first; then choose share levers that fit it (broad-emotion for reach, identity-signal for niche) instead of maximizing reach and depth at once",
  "broad relatability maximizes reach while insider niche maximizes per-viewer share intensity, pulling the scene in opposite directions")
conf("TIMING_RELEVANCE", "SENSITIVITY_GATE", 0.7, -0.55,
  "let the freshness deadline create urgency but never bypass the sensitivity gate; an unsafe topical tie is dropped even if the window is closing",
  "the pressure to publish inside a short topical window conflicts with the time the sensitivity gate needs to clear a topical tie")
conf("CULTURAL_REFERENCE", "CLARITY_READABILITY", 0.66, -0.45,
  "keep references whose estimated segment recognition clears threshold; demote or gloss references that fail the fresh-viewer comprehension gate",
  "dense insider references raise resonance but can break first-contact comprehension for the broader target viewer")

# ---------------- conflict axes (9) ----------------
CA = [
 {"name": "benign_violation_edge_vs_harm_safety", "description": "The violation that creates humor must stay benign; pushing edge raises laughs but risks crossing into harm, while sanding all edge kills the joke.",
  "poles": ["maximal_edge", "harm_free_safety"], "resolution_hint": "relocate the violation off harmful targets; escalate ambiguous calls to compliance",
  "tension_score": 0.78, "affected_nodes": ["BENIGN_VIOLATION", "SENSITIVITY_AWARENESS", "SENSITIVITY_GATE"]},
 {"name": "broad_relatability_vs_insider_niche", "description": "Broad scenes maximize reach but risk blandness; niche scenes maximize per-viewer intensity and shares but cap the ceiling.",
  "poles": ["broad_reach", "insider_depth"], "resolution_hint": "set the target point from the growth goal, then pick references and levers to fit it",
  "tension_score": 0.74, "affected_nodes": ["RELATABILITY_VS_NICHE", "CULTURAL_REFERENCE", "SHAREABILITY_DRIVERS"]},
 {"name": "insider_reference_density_vs_first_contact_clarity", "description": "Dense in-jokes raise resonance for insiders but can break instant comprehension for the broader viewer at scroll speed.",
  "poles": ["max_reference_density", "instant_clarity"], "resolution_hint": "keep references above a recognition threshold; gloss or cut those that fail the comprehension gate",
  "tension_score": 0.7, "affected_nodes": ["CULTURAL_REFERENCE", "CLARITY_READABILITY", "AUDIENCE_FIT"]},
 {"name": "timing_window_urgency_vs_sensitivity_clearance", "description": "A short topical window pressures fast publish, but the sensitivity gate needs time to clear topical ties safely.",
  "poles": ["ride_the_window_fast", "clear_sensitivity_first"], "resolution_hint": "deadline creates urgency but the gate is a hard stop; drop unsafe topical ties",
  "tension_score": 0.72, "affected_nodes": ["TIMING_RELEVANCE", "SENSITIVITY_GATE", "SHAREABILITY_FORECAST"]},
 {"name": "format_familiarity_vs_freshness", "description": "A familiar live format is an instant comprehension shortcut but an overused one signals lateness and lowers shareability.",
  "poles": ["familiar_template", "fresh_novelty"], "resolution_hint": "choose a format still live for the segment; refresh the turn within the familiar frame",
  "tension_score": 0.62, "affected_nodes": ["MEME_FORMAT_REUSE", "SHAREABILITY_DRIVERS", "AUDIENCE_FIT"]},
 {"name": "single_axis_legibility_vs_entity_richness", "description": "One sharp contrast axis makes the incongruity legible but can flatten genuinely multidimensional entities.",
  "poles": ["single_axis_clarity", "multidimensional_richness"], "resolution_hint": "pick the one axis that best serves the turn; carry richness in secondary texture not the spine",
  "tension_score": 0.6, "affected_nodes": ["ENTITY_CONTRAST_AXIS", "ENTITY_BUNDLE", "INCONGRUITY_RESOLUTION"]},
 {"name": "structure_economy_vs_template_fatigue", "description": "Rigid setup/turn/punchline economy maximizes impact but an over-seen three-beat template reads as formulaic.",
  "poles": ["tight_formula", "structural_novelty"], "resolution_hint": "keep the economy; vary the surface so the structure is felt not seen",
  "tension_score": 0.58, "affected_nodes": ["COMEDIC_STRUCTURE", "NARRATIVE_BEATS", "MEME_FORMAT_REUSE"]},
 {"name": "shareability_optimization_vs_brand_trust", "description": "Optimizing purely for shares can drift to outrage-bait that spreads but damages brand trust.",
  "poles": ["max_shares", "protect_trust"], "resolution_hint": "prefer amusement/identity/surprise levers over anger; gate outrage-driven scenes",
  "tension_score": 0.66, "affected_nodes": ["SHAREABILITY_DRIVERS", "SENSITIVITY_AWARENESS", "SHAREABILITY_FORECAST"]},
 {"name": "narrative_richness_vs_attention_window", "description": "A fuller micro-arc raises engagement but can bloat the scene past the platform's short attention window.",
  "poles": ["full_arc", "minimal_punch"], "resolution_hint": "imply the arc; cut any beat not serving the climax within the length norm",
  "tension_score": 0.56, "affected_nodes": ["NARRATIVE_BEATS", "CLARITY_READABILITY", "COMEDIC_STRUCTURE"]},
]

# ---------------- edge cases (12) ----------------
EC = [
 {"description": "Two entities are forced together with no shared frame, producing a non-sequitur the audience cannot resolve.",
  "trigger": "entity bundle assembled without a relatedness check or a chosen contrast axis",
  "affected_nodes": ["ENTITY_BUNDLE", "ENTITY_CONTRAST_AXIS", "INCONGRUITY_RESOLUTION"],
  "mitigation": "require a named relationship and a single contrast axis before assembly", "severity": "high"},
 {"description": "The incongruity surprises but supplies no resolving rule, so the scene reads as random rather than funny.",
  "trigger": "a turn with no retrospective sense-making path for the target segment",
  "affected_nodes": ["INCONGRUITY_RESOLUTION", "HUMOR_QUALITY_SCORE"],
  "mitigation": "verify a resolving rule the segment can close; send back if confusion dominates", "severity": "high"},
 {"description": "A violation lands as harmful (tragedy, ethnic/religious, punching-down) rather than benign and the scene causes offense.",
  "trigger": "violation aimed at a protected or vulnerable target with no benign distance",
  "affected_nodes": ["BENIGN_VIOLATION", "SENSITIVITY_AWARENESS", "SENSITIVITY_GATE"],
  "mitigation": "relocate the violation off harmful targets and escalate ambiguous calls to compliance", "severity": "critical"},
 {"description": "A chosen meme format is dead or overused, signaling lateness and depressing shares despite a good joke.",
  "trigger": "format selected past its segment peak with no freshness check",
  "affected_nodes": ["MEME_FORMAT_REUSE", "SHAREABILITY_DRIVERS"],
  "mitigation": "confirm the format is currently live for the segment before pouring the scene in", "severity": "medium"},
 {"description": "Dense insider references break first-contact comprehension for the broader target viewer.",
  "trigger": "reference density exceeds what the segment recognizes at scroll speed",
  "affected_nodes": ["CULTURAL_REFERENCE", "CLARITY_READABILITY", "RELATABILITY_VS_NICHE"],
  "mitigation": "keep references above a recognition threshold; gloss or cut the rest", "severity": "medium"},
 {"description": "A topical tie attaches the scene to an unfolding tragedy or controversy and the scene blows up.",
  "trigger": "timing tie to a sensitive event bypasses the sensitivity gate under window pressure",
  "affected_nodes": ["TIMING_RELEVANCE", "SENSITIVITY_GATE", "SENSITIVITY_AWARENESS"],
  "mitigation": "route every topical tie through the sensitivity gate; drop unsafe ties even mid-window", "severity": "critical"},
 {"description": "The attention window closes before publish, wasting time-sensitive work.",
  "trigger": "no freshness deadline set on a topical scene",
  "affected_nodes": ["TIMING_RELEVANCE", "SHAREABILITY_FORECAST"],
  "mitigation": "size the window and set a hard freshness deadline up front; abandon if missed", "severity": "low"},
 {"description": "The punchline is not in terminal position, so the surprise leaks and impact drops.",
  "trigger": "comedic structure places the payload before the end or pads after it",
  "affected_nodes": ["COMEDIC_STRUCTURE", "SCENE_ASSEMBLY"],
  "mitigation": "move the funniest element last and cut everything after the turn", "severity": "medium"},
 {"description": "The register fights the content, e.g. an edgy turn in an earnest brand voice, reading as tonally confused.",
  "trigger": "register chosen without agreement with the violation intensity",
  "affected_nodes": ["TONE_REGISTER", "BENIGN_VIOLATION", "SCENE_ASSEMBLY"],
  "mitigation": "align register with the violation intensity and hold it consistently", "severity": "medium"},
 {"description": "The scene is funny but optimized into outrage-bait that spreads while damaging brand trust.",
  "trigger": "shareability optimized on anger with no trust guardrail",
  "affected_nodes": ["SHAREABILITY_DRIVERS", "SENSITIVITY_AWARENESS"],
  "mitigation": "prefer amusement/identity/surprise levers; gate outrage-driven scenes", "severity": "high"},
 {"description": "A harm emerges only from the combination of individually-fine parts and slips past idea-stage screening.",
  "trigger": "sensitivity checked only on the raw idea, not the assembled artifact",
  "affected_nodes": ["SENSITIVITY_GATE", "SCENE_ASSEMBLY"],
  "mitigation": "re-screen the integrated artifact at the final gate, not just the concept", "severity": "high"},
 {"description": "The scene is clear only to its author and a fresh viewer cannot decode the entities or turn at scroll speed.",
  "trigger": "comprehension assumed from the author's seat with no fresh-viewer test",
  "affected_nodes": ["CLARITY_READABILITY", "AUDIENCE_FIT"],
  "mitigation": "test first-contact comprehension on a fresh target-segment viewer with no caption", "severity": "medium"},
]

# ---------------- workflow (12) ----------------
WF = [
 {"action": "select_entity_bundle", "node_ref": "ENTITY_BUNDLE", "description": "Pick 2-3 genuinely related entities and confirm a shared frame and named relationship.", "artifact": "entity_bundle_record", "gate": "2-3 related entities with an explicit shared frame"},
 {"action": "define_audience_and_register", "node_ref": "AUDIENCE_FIT", "description": "Name the target segment, inventory its shared schema and platform norms.", "artifact": "audience_schema", "gate": "segment named with a concrete reference schema"},
 {"action": "pick_contrast_axis", "node_ref": "ENTITY_CONTRAST_AXIS", "description": "Choose the single contrast axis that orders all entities legibly.", "artifact": "contrast_axis_record", "gate": "exactly one axis ordering all 2-3 entities"},
 {"action": "engineer_incongruity", "node_ref": "INCONGRUITY_RESOLUTION", "description": "Build the setup, incongruity, and a resolution the segment can close.", "artifact": "humor_core", "gate": "setup, incongruity, and a segment-closable resolution present"},
 {"action": "tune_benign_violation", "node_ref": "BENIGN_VIOLATION", "description": "Set the violation intensity into the band that is felt yet stays benign.", "artifact": "violation_calibration", "gate": "violation detectable and judged benign for the segment"},
 {"action": "arrange_structure_and_beats", "node_ref": "COMEDIC_STRUCTURE", "description": "Order setup/turn/punchline with the payload last; shape the micro-arc.", "artifact": "structured_beats", "gate": "punchline terminal; no inessential elements"},
 {"action": "select_format", "node_ref": "MEME_FORMAT_REUSE", "description": "Choose a live meme format and map entities to its slots.", "artifact": "format_mapping", "gate": "format currently live; slots map to entities and turn"},
 {"action": "assemble_scene", "node_ref": "SCENE_ASSEMBLY", "description": "Integrate entities, format, axis, beats, and register into one coherent draft.", "artifact": "scene_draft", "gate": "single coherent unit with all entities placed"},
 {"action": "check_clarity", "node_ref": "CLARITY_READABILITY", "description": "Test first-contact comprehension on a fresh segment viewer with no caption.", "artifact": "clarity_report", "gate": "entities and turn comprehended within the scroll window"},
 {"action": "score_humor", "node_ref": "HUMOR_QUALITY_SCORE", "description": "Score resolution, benignity, and sample amusement; diagnose the weakest link.", "artifact": "humor_scorecard", "gate": "incongruity resolves and sample finds it amusing"},
 {"action": "run_sensitivity_gate", "node_ref": "SENSITIVITY_GATE", "description": "Re-screen the assembled artifact for harm; require compliance sign-off for escalations.", "artifact": "sensitivity_signoff", "gate": "assembled scene clean; escalations signed off"},
 {"action": "forecast_and_handoff", "node_ref": "SHAREABILITY_FORECAST", "description": "Count share levers, estimate recognition, confirm window open, then package for publish.", "artifact": "publish_package", "gate": "share levers and open window confirmed; gates passed"},
]

# ---------------- dominance rules (9) ----------------
DR = [
 {"rule": "ENTITY_BUNDLE relatedness and a single ENTITY_CONTRAST_AXIS must be fixed before INCONGRUITY_RESOLUTION is engineered",
  "rationale": "the incongruity is drawn along the contrast axis; building humor on an unrelated bundle yields a non-sequitur",
  "trigger": "incongruity design begins on an unrelated bundle or with no axis", "action": "block until a named relationship and one axis exist"},
 {"rule": "SENSITIVITY_GATE is a hard stop that overrides HUMOR_QUALITY_SCORE and SHAREABILITY_FORECAST",
  "rationale": "a funny, highly-shareable scene that fails the harm gate must not ship",
  "trigger": "a scene passes humor and forecast but fails sensitivity", "action": "block publish regardless of humor or forecast scores"},
 {"rule": "BENIGN_VIOLATION must keep the violation benign before edge is maximized for shares",
  "rationale": "violation that crosses into harm spreads damage, not just reach",
  "trigger": "edge is pushed for shareability past the benign band", "action": "relocate the violation off harmful targets or escalate"},
 {"rule": "SENSITIVITY_AWARENESS must escalate any ambiguous harm call to compliance rather than deciding it in the writing seat",
  "rationale": "a single writer should not absorb a legal/ethical adjudication",
  "trigger": "an ambiguous harmful-framing case is resolved by the writer", "action": "route the case to compliance and hold publish"},
 {"rule": "CLARITY_READABILITY must pass before HUMOR_QUALITY_SCORE is trusted",
  "rationale": "a scene that is not comprehended cannot be fairly judged funny",
  "trigger": "humor scored on a scene that fails first-contact comprehension", "action": "fix comprehension before scoring humor"},
 {"rule": "TIMING_RELEVANCE topical ties must clear SENSITIVITY_GATE even under window pressure",
  "rationale": "an unsafe topical tie causes outsized harm precisely because it is timely",
  "trigger": "a topical tie is published to beat a closing window without the gate", "action": "drop the tie or the scene; never bypass the gate"},
 {"rule": "MEME_FORMAT_REUSE must confirm the format is currently live for the segment before SCENE_ASSEMBLY pours the scene in",
  "rationale": "assembling around a dead format wastes layout work and signals lateness",
  "trigger": "assembly proceeds on an unverified or stale format", "action": "verify format liveness before assembly"},
 {"rule": "RELATABILITY_VS_NICHE target point must be set from the growth goal before CULTURAL_REFERENCE density is chosen",
  "rationale": "reference density should follow the reach-vs-depth decision, not drive it",
  "trigger": "reference density chosen before the reach-vs-depth point", "action": "fix the target point first, then choose references"},
 {"rule": "SCENE_ASSEMBLY must reconcile format, beats, axis, and register before any evaluation gate runs",
  "rationale": "gates judge the integrated artifact, not loose parts; evaluating fragments wastes review",
  "trigger": "an evaluation gate runs on an un-integrated draft", "action": "require a single coherent draft before gating"},
]

# ---------------- anti-rework rules (9) ----------------
ARR = [
 {"rule": "Do not engineer the incongruity before the entity relationship and contrast axis are fixed; an axis change invalidates the whole joke",
  "prevents": "rebuilding the humor core after a late axis or relationship change"},
 {"rule": "Do not push edge for shares before confirming the violation stays benign; harm found later forces a full rewrite or a takedown",
  "prevents": "emergency rewrite or post-publish takedown after a harm complaint"},
 {"rule": "Do not assemble the scene around a meme format without confirming it is live; a dead format means redoing the layout",
  "prevents": "re-laying-out the scene after discovering the format is stale"},
 {"rule": "Do not load dense insider references before setting the reach-vs-depth point; over-niche references cap reach and need stripping",
  "prevents": "stripping references and re-pacing after a reach-ceiling miss"},
 {"rule": "Do not run evaluation gates on un-integrated fragments; fragment fixes do not survive integration",
  "prevents": "re-reviewing the same scene after parts are finally integrated"},
 {"rule": "Do not screen sensitivity only on the raw idea; harms that emerge from combination force late, expensive rework",
  "prevents": "killing a fully produced scene at the final gate over an emergent harm"},
 {"rule": "Do not commit to a register that fights the violation intensity; a tonal mismatch means re-voicing the whole scene",
  "prevents": "re-voicing the entire scene after a tonal-confusion review note"},
 {"rule": "Do not skip the freshness deadline on a topical scene; a missed window means the work is wasted and reworked as evergreen",
  "prevents": "salvaging a missed-window scene into an evergreen format under pressure"},
]

# ---------------- iteration protocol (8) ----------------
IP = [
 {"trigger": "a target-segment sample reports confusion rather than amusement",
  "action": "diagnose whether the resolution rule or the comprehension is at fault and revise INCONGRUITY_RESOLUTION or CLARITY_READABILITY",
  "nodes": ["INCONGRUITY_RESOLUTION", "CLARITY_READABILITY", "HUMOR_QUALITY_SCORE"], "priority": "high"},
 {"trigger": "a harm complaint or near-miss is reported on a benign-violation call",
  "action": "relocate the violation off the harmful target and tighten the escalation trigger in SENSITIVITY_AWARENESS",
  "nodes": ["BENIGN_VIOLATION", "SENSITIVITY_AWARENESS", "SENSITIVITY_GATE"], "priority": "critical"},
 {"trigger": "a scene underperforms on shares despite passing humor and clarity",
  "action": "re-audit the share levers and reference recognition in SHAREABILITY_DRIVERS and SHAREABILITY_FORECAST",
  "nodes": ["SHAREABILITY_DRIVERS", "SHAREABILITY_FORECAST", "CULTURAL_REFERENCE"], "priority": "high"},
 {"trigger": "a chosen meme format proves dead or overused after assembly",
  "action": "re-select a live format in MEME_FORMAT_REUSE and re-pour the scene in SCENE_ASSEMBLY",
  "nodes": ["MEME_FORMAT_REUSE", "SCENE_ASSEMBLY"], "priority": "medium"},
 {"trigger": "a topical tie's attention window closes before publish",
  "action": "tighten window sizing and the freshness deadline in TIMING_RELEVANCE or abandon the topical angle",
  "nodes": ["TIMING_RELEVANCE", "SHAREABILITY_FORECAST"], "priority": "medium"},
 {"trigger": "the audience misreads which contrast axis is in play",
  "action": "sharpen or re-select the axis in ENTITY_CONTRAST_AXIS and re-test comprehension",
  "nodes": ["ENTITY_CONTRAST_AXIS", "CLARITY_READABILITY"], "priority": "medium"},
 {"trigger": "reviewers flag tonal confusion between register and content",
  "action": "realign the register with the violation intensity in TONE_REGISTER and re-assemble",
  "nodes": ["TONE_REGISTER", "BENIGN_VIOLATION", "SCENE_ASSEMBLY"], "priority": "medium"},
 {"trigger": "a reach-ceiling miss shows the scene was too niche for the goal",
  "action": "reset the reach-vs-depth point in RELATABILITY_VS_NICHE and adjust reference density in CULTURAL_REFERENCE",
  "nodes": ["RELATABILITY_VS_NICHE", "CULTURAL_REFERENCE"], "priority": "medium"},
]

spec = {
 "domain": "scene__multi_entity_composition",
 "domain_label": "2-3 Entity Scene & Comedic/Narrative Composition",
 "purpose": "compose_two_to_three_linked_entities_into_one_audience_fit_scene_using_incongruity_resolution_meme_formats_and_micro_narrative_beats",
 "assumptions": [
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "scope is composing a single scene from a given 2-3 entity bundle; entity linking/disambiguation and asset rendering are delegated to sibling KBs",
   "a target audience segment with an inspectable shared schema is available",
   "final legal/compliance adjudication of hard sensitivity calls is owned by a compliance function, not the writer",
 ],
 "exclusions": [
   "entity disambiguation and relatedness scoring internals (delegated to link__entity_relevance)",
   "audience segment discovery and modeling internals (delegated to niche__audience_segmentation)",
   "visual asset rendering, typesetting, and production tooling",
   "paid distribution, ad-buying, and post-publish analytics attribution",
 ],
 "source_description": "heuristic prior estimates for multi-entity comedic/narrative scene composition work units, informed by incongruity-resolution and benign-violation humor theory, classical dramatic structure, meme studies, and virality research; no supplied dataset",
 "source_citation": "Suls 1972 (incongruity-resolution two-stage model, in Goldstein & McGhee eds., The Psychology of Humor); McGraw & Warren 2010 Benign Violation Theory (Psychological Science 21:1141-1149); Freytag 1863 Die Technik des Dramas (Freytag's pyramid); Dawkins 1976 The Selfish Gene ch.11 (meme); Shifman 2014 Memes in Digital Culture (MIT Press); Berger & Milkman 2012 What Makes Online Content Viral? (Journal of Marketing Research 49:192-205)",
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
 "priority_rationale": "ENTITY_BUNDLE, AUDIENCE_FIT and the humor engine (INCONGRUITY_RESOLUTION/BENIGN_VIOLATION) are foundational; ENTITY_CONTRAST_AXIS, structure, format and beats feed SCENE_ASSEMBLY; CLARITY/HUMOR/SENSITIVITY/SHAREABILITY gates and the handoff close the method, with the sensitivity gate dominating as a hard stop.",
 "eval_objective": "verify_multi_entity_scene_composition_incongruity_resolution_benign_violation_format_reuse_narrative_beats_audience_fit_sensitivity_and_shareability_of_scene__multi_entity_composition_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "scene__multi_entity_composition.spec.json")
open(path, "w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes", len(N), "edges", len(E), "CA", len(CA), "EC", len(EC), "WF", len(WF),
      "CQ", len(CQS), "DR", len(DR), "ARR", len(ARR), "IP", len(IP))
