#!/usr/bin/env python3
"""Generate the promptgen__downstream_brief content spec (B60 KB) for kb_forge.py.

Domain: compile a STRUCTURED downstream prompt-brief -- a structured image-prompt, a
story/script-prompt, and an OPTIONAL audio-brief -- that is emitted NOW and injected
into a SEPARATE downstream agent session LATER. The hard scope finding (SPOF + scope
containment) this KB encodes: the legal/disclosure/safety contract MUST travel IN-BAND
with the prompt artifact (design-by-contract assume/guarantee carried in the payload),
because the downstream session shares NO state with this one; and this stage produces
prompts ONLY -- it NEVER publishes or renders (Dijkstra separation of concerns).

Compact authoring (mimics example_htn_gen.py / the B60 siblings): node()/b()/pro()/con()
/dep()/conf()/rel() helpers carry sane defaults so each call supplies only domain
content + base metric magnitudes. Grounded in real literature cited per node:
- design-brief practice (creative brief: objective/audience/deliverable/constraints);
- prompt-engineering patterns (role / constraint / prompt-chaining; White et al. 2023
  "A Prompt Pattern Catalog"; OpenAI/Anthropic prompting guidance);
- Rombach et al. 2022 "High-Resolution Image Synthesis with Latent Diffusion Models"
  (text conditioning, classifier-free guidance, negative prompts);
- Dijkstra 1982 "On the role of scientific thought" (separation of concerns);
- Meyer 1992 "Applying Design by Contract" (assume/guarantee, preconditions carried
  with the supplier-consumer boundary);
- in-band signalling / SPOF reasoning (a contract assumed-by-reference fails when the
  downstream session never receives it)."""
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
        "academic_fields": ["prompt_engineering", "software_design"],
        "subfields": subfields or ["downstream_handoff", "synthetic_media_briefing"],
        "specialists": specialists or ["prompt_engineer", "creative_brief_author"],
        "contradictors": contradictors or ["assume_downstream_knows_advocate"],
        "inputs": inputs or ["dense machine_summary", "downstream agent target"],
        "outputs": outputs or ["structured prompt-brief artifact"],
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

N.append(node("PROMPT_ARTIFACT_SCHEMA", "structured_prompt_brief_schema",
  "Define the closed schema of the emitted prompt-brief artifact: a structured image-prompt block, a story/script-prompt block, and an OPTIONAL audio-brief block, plus a header carrying the disclosure and in-band contract slots, so every consumer parses one fixed, versioned shape.",
  "foundations", [], ["CQ_01"],
  b(0.92, 0.85, 0.82, 0.6, 0.86, 0.82, 0.66, 0.7, 0.45, 0.78, 0.82, 0.9, 0.7, 0.16, 0.55, [0.2, 0.5], "artifact schema version or required-block set changes"),
  ["block typing (image/story/audio/header)", "schema version field", "required vs optional block declaration"],
  ["rendering of media", "downstream publishing", "model-specific prompt syntax tuning"],
  [pro("A single versioned artifact schema makes the brief machine-parseable and lets the downstream session validate before acting", "header.schema_version=1 with image/story blocks required and audio optional")],
  [con("A rigid schema can lag a new media type and force out-of-band side channels until it is extended", "a new 3D-asset prompt has no block and gets smuggled into the story field")],
  ["under-specified blocks force guessing downstream", "optional audio block silently treated as required"],
  ["the artifact validates against its declared schema_version and every required block is present and typed"],
  ["a new downstream media type is requested", "schema version bumped"],
  outputs=["versioned prompt-brief schema"], specialists=["prompt_engineer", "schema_designer"]))

N.append(node("GROUND_FROM_SUMMARY", "ground_prompts_in_dense_summary",
  "Construct every prompt-brief field strictly from the dense upstream machine_summary fields (subject, entities, claims, constraints), never from free invention, so the brief is traceable to source facts rather than hallucinated detail.",
  "foundations", ["PROMPT_ARTIFACT_SCHEMA"], ["CQ_02"],
  b(0.88, 0.8, 0.78, 0.62, 0.85, 0.8, 0.6, 0.66, 0.5, 0.74, 0.78, 0.86, 0.66, 0.2, 0.52, [0.24, 0.56], "machine_summary field schema or grounding policy changes"),
  ["field-to-summary-fact mapping", "use only summary-attested entities/claims", "no-invention rule"],
  ["summary generation itself", "fact verification of the summary", "rendering"],
  [pro("Grounding each prompt field in a named summary fact makes the brief auditable and curbs hallucinated subjects", "image subject pulled from summary.primary_entity, not invented")],
  [con("Strict grounding can starve a prompt of the connotative detail that makes generations vivid", "a summary lists 'a city' but no mood, yielding a flat image prompt")],
  ["prompt invents a subject absent from the summary", "summary field misread into the wrong block"],
  ["every populated prompt field cites at least one machine_summary field as its source"],
  ["machine_summary schema changes", "an ungrounded invented detail is found in a brief"],
  inputs=["dense machine_summary", "summary field index"], specialists=["prompt_engineer", "data_provenance_analyst"]))

N.append(node("IMAGE_PROMPT_SPEC", "structured_image_prompt_spec",
  "Specify the image-prompt block as structured fields -- subject, style, composition/framing, lighting, and an explicit negative-constraint list -- aligned to text-to-image conditioning so a downstream diffusion model can be steered without prose ambiguity.",
  "specs", ["PROMPT_ARTIFACT_SCHEMA", "GROUND_FROM_SUMMARY"], ["CQ_03"],
  b(0.85, 0.8, 0.82, 0.66, 0.78, 0.74, 0.55, 0.66, 0.5, 0.76, 0.78, 0.84, 0.66, 0.2, 0.48, [0.22, 0.54], "downstream image-model conditioning interface changes"),
  ["subject/style/composition/lighting fields", "negative-constraint list", "aspect-ratio and reference hints"],
  ["actual image generation", "model-weights or LoRA selection", "post-processing"],
  [pro("Structured fields plus a negative list map directly onto diffusion text-conditioning and classifier-free guidance, steering output and suppressing unwanted content", "negative: 'text, watermark, extra fingers' removes common diffusion artifacts (Rombach et al. 2022)")],
  [con("Over-stuffed prompts dilute the conditioning signal and can fight classifier-free guidance, degrading coherence", "fifteen competing style tokens yield a muddy, averaged image")],
  ["negative list omitted, so disallowed content reappears", "style and subject fields contradict each other"],
  ["the image block contains a non-empty subject and a negative-constraint list and validates against the schema"],
  ["downstream image model or its prompt syntax changes", "generations repeatedly violate a negative constraint"],
  outputs=["structured image-prompt block"], specialists=["prompt_engineer", "diffusion_prompt_specialist"],
  subfields=["text_to_image_conditioning", "negative_prompting"]))

N.append(node("STORY_PROMPT_SPEC", "structured_story_script_prompt_spec",
  "Specify the story/script-prompt block as structured fields -- premise, beat outline, narrative voice/POV, tone, and target length -- so a downstream text model produces a story or script with controlled structure rather than an open-ended ramble.",
  "specs", ["PROMPT_ARTIFACT_SCHEMA", "GROUND_FROM_SUMMARY"], ["CQ_04"],
  b(0.83, 0.78, 0.8, 0.62, 0.74, 0.72, 0.52, 0.66, 0.48, 0.76, 0.78, 0.82, 0.66, 0.2, 0.46, [0.22, 0.54], "story prompt field set or length-control policy changes"),
  ["premise/beats/voice/tone/length fields", "POV and tense declaration", "structural constraints"],
  ["actual story generation", "editing or proofreading of output", "publishing"],
  [pro("Beat-outlined, length-bounded prompts give the downstream model a scaffold, improving coherence and controllability over a single instruction", "five-beat outline plus 'second person, present tense, 300 words' yields a structured micro-story")],
  [con("Heavy structural constraints can suppress the model's narrative creativity and produce formulaic output", "rigid beats force a predictable arc and flatten voice")],
  ["length target absent, output runs unbounded", "voice/POV unspecified, tone drifts mid-story"],
  ["the story block declares a premise, a beat outline, and a target length and validates against the schema"],
  ["downstream text model changes", "stories repeatedly miss the target length or voice"],
  outputs=["structured story/script-prompt block"], specialists=["prompt_engineer", "narrative_designer"],
  subfields=["controllable_generation", "narrative_structure"]))

N.append(node("AUDIO_BRIEF_SPEC", "optional_audio_brief_spec",
  "Specify the OPTIONAL audio-brief block -- script text, an abstract voice DESCRIPTION (timbre/pace/register), and a default NON-impersonating constraint with a consent-reference slot -- so any downstream TTS or voice synthesis is steered by description, never by an identifiable real person, unless explicit consent is referenced in-band.",
  "specs", ["PROMPT_ARTIFACT_SCHEMA", "STORY_PROMPT_SPEC"], ["CQ_05"],
  b(0.84, 0.74, 0.72, 0.62, 0.86, 0.78, 0.62, 0.62, 0.55, 0.74, 0.76, 0.82, 0.62, 0.24, 0.6, [0.26, 0.62], "voice-cloning policy, consent model, or audio synthesis interface changes"),
  ["script text", "abstract voice description (timbre/pace/register)", "non-impersonation default", "consent-reference slot"],
  ["actual audio synthesis", "voice-model selection", "distribution of audio"],
  [pro("Describing a voice abstractly and defaulting to non-impersonation lets audio be steered without cloning a real person's identity", "voice: 'warm mid-range female narrator, unhurried' instead of naming a celebrity")],
  [con("Abstract voice description gives less precise control than a reference clip, so timbre may vary run to run", "'gravelly older male' admits a wide range of realized voices")],
  ["a named real person is requested without a consent reference", "consent slot left empty but impersonation implied"],
  ["if present, the audio block carries a script, an abstract voice description, and either a non-impersonation default or a populated consent reference"],
  ["voice-cloning or consent policy changes", "an impersonation request appears without consent"],
  outputs=["optional audio-brief block"], specialists=["prompt_engineer", "audio_safety_reviewer"],
  contradictors=["voice_clone_by_default_advocate"], subfields=["text_to_speech_briefing", "voice_consent"]))

N.append(node("IN_BAND_CONTRACT", "in_band_safety_disclosure_contract",
  "Carry the safety/disclosure contract -- required synthetic-media disclosure tokens, usage restrictions, and the assume/guarantee clauses -- IN-BAND inside the artifact payload, so the downstream session (which shares no state with this one) receives the contract WITH the prompt instead of assuming it by reference. This closes the single point of failure where an out-of-band contract is silently dropped at the session boundary.",
  "contract", ["PROMPT_ARTIFACT_SCHEMA"], ["CQ_06"],
  b(0.95, 0.86, 0.8, 0.66, 0.92, 0.85, 0.74, 0.66, 0.6, 0.76, 0.8, 0.94, 0.66, 0.22, 0.68, [0.22, 0.52], "disclosure law, safety policy, or the assume/guarantee contract clauses change"),
  ["embedded disclosure tokens", "usage restrictions", "assume/guarantee clauses carried in payload", "contract version"],
  ["enforcement at render time", "downstream publishing decisions", "legal drafting of the policy text itself"],
  [pro("Embedding the contract in the payload makes the safety assumption a guaranteed precondition delivered to the consumer, not a fragile cross-session assumption (Meyer design-by-contract; in-band signalling closes the SPOF)", "the brief itself states 'mark output as AI-generated' so a stateless downstream session cannot miss it")],
  [con("In-band contract text adds payload weight and can be stripped or ignored by a non-compliant consumer if not validated", "a downstream tool truncates the header and loses the disclosure clause")],
  ["contract assumed downstream by reference and never delivered (the SPOF)", "in-band clause present but unvalidated and silently ignored"],
  ["the artifact payload contains the disclosure tokens, usage restrictions, and assume/guarantee clauses with a contract version, verifiable without any external lookup"],
  ["disclosure regulation or safety policy changes", "a downstream session is found acting without the contract"],
  outputs=["in-band contract block"], specialists=["safety_engineer", "policy_contract_author"],
  contradictors=["assume_downstream_knows_advocate"], subfields=["design_by_contract", "in_band_signalling"]))

N.append(node("DISCLOSURE_SLOT", "mandatory_synthetic_media_disclosure_slot",
  "Provide a mandatory, non-empty disclosure slot in the artifact that names the required synthetic-media label and where it must appear, so downstream generation always carries an AI-generated/synthetic-content label as a hard precondition of the contract.",
  "contract", ["IN_BAND_CONTRACT"], ["CQ_07"],
  b(0.9, 0.82, 0.82, 0.55, 0.9, 0.78, 0.7, 0.7, 0.55, 0.8, 0.82, 0.9, 0.7, 0.18, 0.62, [0.18, 0.46], "synthetic-media labeling requirements or placement rules change"),
  ["required label text", "label placement directive", "non-empty enforcement of the slot"],
  ["visual rendering of the label", "platform-specific watermarking", "the legal text of disclosure law"],
  [pro("A mandatory non-empty slot makes the synthetic-media label a structural requirement that cannot be forgotten downstream", "slot: label='AI-generated', placement='visible caption + metadata'")],
  [con("A single generic label may not satisfy every jurisdiction's specific disclosure wording", "one region requires a longer prescribed sentence the generic label omits")],
  ["disclosure slot left empty yet artifact still validates", "label specified but no placement, so it is rendered invisibly"],
  ["the disclosure slot is non-empty, names a label and a placement, and the schema rejects an empty slot"],
  ["labeling regulation changes", "an unlabeled synthetic output is produced downstream"],
  outputs=["disclosure label directive"], specialists=["safety_engineer", "compliance_reviewer"]))

N.append(node("SEPARATION_OF_CONCERNS", "produce_prompts_never_publish",
  "Enforce that this stage PRODUCES prompt-briefs only and NEVER renders, synthesizes, or publishes media -- a strict separation of concerns (Dijkstra) that contains scope, so generation and distribution are the downstream session's responsibility under the carried contract.",
  "boundary", ["PROMPT_ARTIFACT_SCHEMA"], ["CQ_08"],
  b(0.93, 0.82, 0.78, 0.5, 0.9, 0.85, 0.78, 0.7, 0.58, 0.8, 0.82, 0.92, 0.7, 0.16, 0.66, [0.18, 0.46], "stage responsibility boundary or pipeline ownership changes"),
  ["prompt production only", "explicit no-render/no-publish boundary", "scope-containment assertion"],
  ["image/story/audio synthesis", "publishing or distribution", "platform posting"],
  [pro("Separating prompt production from rendering contains this stage's scope and blast radius: a bug here cannot publish unsafe media, only emit a brief (Dijkstra separation of concerns)", "this stage has no render or post capability wired in at all")],
  [con("A hard boundary adds a handoff step and latency versus an end-to-end pipeline that generates inline", "two sessions and a serialization round-trip instead of one call")],
  ["this stage gains a render/publish path, collapsing the separation", "scope creep adds distribution responsibilities here"],
  ["this stage exposes no render or publish capability; its only output is a prompt-brief artifact"],
  ["pipeline ownership boundaries change", "a render or publish path appears in this stage"],
  outputs=["scope-boundary assertion"], specialists=["software_architect", "safety_engineer"],
  contradictors=["end_to_end_inline_pipeline_advocate"], subfields=["separation_of_concerns", "scope_containment"]))

N.append(node("DOWNSTREAM_HANDOFF", "emit_now_inject_into_separate_session",
  "Emit the prompt-brief now for injection into a SEPARATE downstream agent session later; treat the handoff as a stateless boundary where nothing from this session's context is available unless it was serialized into the artifact.",
  "handoff", ["IMAGE_PROMPT_SPEC", "STORY_PROMPT_SPEC", "IN_BAND_CONTRACT"], ["CQ_09"],
  b(0.9, 0.84, 0.78, 0.66, 0.88, 0.85, 0.7, 0.62, 0.58, 0.74, 0.78, 0.9, 0.62, 0.22, 0.62, [0.24, 0.58], "handoff transport, serialization format, or session boundary semantics change"),
  ["self-contained serialized artifact", "stateless-boundary assumption", "no reliance on this session's context"],
  ["the downstream session's execution", "transport security", "rendering"],
  [pro("Treating the boundary as stateless forces everything the consumer needs -- including the contract -- into the artifact, eliminating dropped-context failures", "the brief carries subject, style, AND disclosure so a fresh session needs no back-reference")],
  [con("A fully self-contained artifact duplicates context and is larger than a referenced handoff", "the disclosure and source facts are restated rather than linked")],
  ["handoff relies on shared state that the new session lacks", "artifact references an id the downstream session cannot resolve"],
  ["the emitted artifact is self-contained: a fresh session with no prior context can act on it using only the payload"],
  ["handoff transport or serialization format changes", "a downstream session fails for lack of carried context"],
  outputs=["serialized handoff artifact"], specialists=["integration_engineer", "prompt_engineer"],
  subfields=["session_handoff", "stateless_serialization"]))

N.append(node("DETERMINISM_PROVENANCE", "record_node_facts_used",
  "Record the provenance of the brief: which machine_summary fields and which contract/disclosure clauses each prompt field drew on, so the same inputs reproduce the same brief and any field is traceable back to its source fact.",
  "provenance", ["GROUND_FROM_SUMMARY", "IN_BAND_CONTRACT"], ["CQ_10"],
  b(0.8, 0.74, 0.72, 0.6, 0.74, 0.78, 0.55, 0.68, 0.48, 0.78, 0.8, 0.8, 0.68, 0.18, 0.46, [0.2, 0.48], "provenance schema or determinism requirement changes"),
  ["field-to-source-fact map", "contract-clause references", "deterministic-build record"],
  ["downstream provenance of generations", "summary provenance", "rendering"],
  [pro("A provenance map makes the brief reproducible and auditable: every field points at the summary fact or contract clause that produced it", "image.subject -> summary.entity[0]; header.disclosure -> contract.clause[2]")],
  [con("Provenance records add weight and must be kept in sync when a field is edited", "an edited style field whose provenance pointer was not updated misleads an auditor")],
  ["provenance pointer stale after a field edit", "a field has no recorded source, blocking audit"],
  ["every prompt field carries a provenance pointer to a summary field or contract clause and rebuilds deterministically from them"],
  ["provenance schema changes", "a non-reproducible brief is observed for identical inputs"],
  outputs=["provenance map"], specialists=["data_provenance_analyst", "prompt_engineer"],
  subfields=["provenance_tracking", "reproducibility"]))

N.append(node("HANDOFF_VALIDATION", "downstream_parseability_validation",
  "Validate, before emission, that a downstream consumer can parse the artifact: schema-validate every block, confirm the disclosure slot and in-band contract are present and non-empty, and confirm no required field is missing, so a malformed brief is caught here rather than failing in the separate session.",
  "validation", ["DOWNSTREAM_HANDOFF", "DISCLOSURE_SLOT", "DETERMINISM_PROVENANCE", "AUDIO_BRIEF_SPEC", "SEPARATION_OF_CONCERNS"], ["CQ_11"],
  b(0.9, 0.82, 0.8, 0.64, 0.88, 0.82, 0.66, 0.64, 0.55, 0.8, 0.84, 0.9, 0.64, 0.2, 0.6, [0.2, 0.5], "validation contract or downstream parser expectations change"),
  ["schema validation of all blocks", "presence/non-empty checks for disclosure and contract", "required-field completeness check"],
  ["downstream execution correctness", "semantic quality of generations", "transport"],
  [pro("Validating parseability and contract presence at emission time turns a downstream parse failure into a local, fixable error and guarantees the safety contract ships", "validator rejects a brief whose disclosure slot is empty before it ever leaves this stage")],
  [con("Validation only checks structure, not whether a permissive downstream consumer will honor the carried contract", "a consumer parses the brief yet ignores the disclosure clause")],
  ["validator passes a brief missing the in-band contract", "downstream parser is stricter than the validator, so a valid-here brief still fails there"],
  ["the validator rejects any artifact with a missing block, an empty disclosure slot, or an absent in-band contract before emission"],
  ["downstream parser contract changes", "a brief that passed validation fails to parse downstream"],
  outputs=["validated, emission-ready artifact"], specialists=["qa_engineer", "integration_engineer"],
  subfields=["schema_validation", "contract_presence_checking"]))

# ---- additional finer-grained work-units (decompositions of the themes above) ----

N.append(node("TARGET_MODEL_PROFILE", "downstream_model_capability_profile",
  "Declare the downstream model/agent profile each block targets -- modality, prompt syntax dialect, token budget, and supported controls (e.g. negative prompts, system role) -- so block fields are shaped to controls the consumer actually honors rather than to a generic ideal.",
  "foundations", ["PROMPT_ARTIFACT_SCHEMA"], ["CQ_13"],
  b(0.78, 0.74, 0.72, 0.62, 0.7, 0.74, 0.5, 0.66, 0.45, 0.76, 0.78, 0.78, 0.66, 0.2, 0.44, [0.22, 0.52], "downstream model roster or its supported-control set changes"),
  ["target modality per block", "prompt-syntax dialect", "token budget", "supported-control flags"],
  ["model weights or hosting", "actual generation", "vendor selection policy"],
  [pro("Profiling the target lets the brief use controls the consumer honors (negative prompts, system role) instead of tokens it ignores", "skip a negative-prompt field for a model with no classifier-free guidance")],
  [con("Profiles drift as vendors change syntax, so a stale profile shapes prompts for controls that no longer exist", "a model deprecates a weighting syntax the profile still emits")],
  ["block shaped to a control the target does not support", "token budget exceeds the target's context window"],
  ["each block declares the target profile it was shaped for"],
  ["the downstream model roster changes", "a supported-control assumption is found stale"],
  inputs=["downstream agent target", "model capability roster"], specialists=["prompt_engineer", "integration_engineer"],
  subfields=["model_capability_profiling", "prompt_dialects"]))

N.append(node("ROLE_FRAMING", "role_and_constraint_prompt_framing",
  "Apply role/constraint prompt-engineering patterns to each block: set the downstream model's role, state hard constraints as imperatives, and order constraints before content, so the brief steers behavior through pattern rather than hope.",
  "specs", ["PROMPT_ARTIFACT_SCHEMA", "TARGET_MODEL_PROFILE"], ["CQ_13"],
  b(0.76, 0.72, 0.74, 0.6, 0.68, 0.72, 0.48, 0.66, 0.44, 0.76, 0.78, 0.76, 0.66, 0.2, 0.42, [0.22, 0.52], "prompt-pattern catalog or role-framing convention changes"),
  ["role assignment", "constraint-as-imperative phrasing", "constraint-before-content ordering"],
  ["model fine-tuning", "actual generation", "evaluation of output quality"],
  [pro("Role and constraint patterns are documented to improve controllability and reduce drift versus a bare instruction (White et al. 2023 prompt pattern catalog)", "'You are a storyboard artist. Hard constraints: ... Now render the scene as:'")],
  [con("Over-engineered role framing can crowd out the actual content and waste the token budget", "a long persona preamble leaves little budget for the subject")],
  ["constraints buried after content and ignored", "role framing contradicts the carried contract"],
  ["each block applies a role and states its hard constraints as imperatives before content"],
  ["prompt-pattern conventions change", "role framing is found to crowd out content"],
  specialists=["prompt_engineer"], subfields=["prompt_patterns", "role_framing"]))

N.append(node("BRIEF_OBJECTIVE_FRAME", "creative_brief_objective_framing",
  "Frame each block with the creative-brief essentials -- objective, audience, deliverable, and constraints -- so the downstream generation has the same orienting context a human creative brief provides, not just raw fields.",
  "foundations", ["PROMPT_ARTIFACT_SCHEMA", "GROUND_FROM_SUMMARY"], ["CQ_13"],
  b(0.77, 0.76, 0.78, 0.55, 0.66, 0.72, 0.48, 0.68, 0.42, 0.78, 0.78, 0.77, 0.68, 0.18, 0.4, [0.2, 0.48], "creative-brief field conventions change"),
  ["objective statement", "audience descriptor", "deliverable definition", "constraint list"],
  ["campaign strategy", "media buying", "publishing"],
  [pro("Carrying objective/audience/deliverable/constraints gives the downstream model the orienting frame a human creative brief provides, improving on-target output", "objective: 'evoke calm'; audience: 'wellness app users'; deliverable: 'square hero image'")],
  [con("A full brief frame adds length and can over-specify when a single field would suffice", "a one-word subject wrapped in a paragraph of brief boilerplate")],
  ["objective missing, so output is technically correct but off-purpose", "audience omitted, tone misjudged"],
  ["each block states an objective, an audience, and a deliverable definition"],
  ["creative-brief conventions change", "outputs are on-spec but off-objective"],
  specialists=["creative_brief_author", "prompt_engineer"], subfields=["creative_brief", "objective_framing"]))

N.append(node("NEGATIVE_CONSTRAINT_SET", "negative_and_safety_constraint_set",
  "Compile the consolidated negative/safety constraint set spanning all blocks -- disallowed visual content, prohibited narrative themes, and audio exclusions -- and map it onto each modality's exclusion mechanism, so safety exclusions are uniform rather than per-block afterthoughts.",
  "contract", ["IMAGE_PROMPT_SPEC", "IN_BAND_CONTRACT"], ["CQ_14"],
  b(0.84, 0.78, 0.76, 0.6, 0.84, 0.8, 0.62, 0.64, 0.55, 0.78, 0.8, 0.84, 0.64, 0.2, 0.58, [0.2, 0.5], "prohibited-content policy or per-modality exclusion mechanism changes"),
  ["consolidated disallowed-content list", "per-modality exclusion mapping", "uniform safety-exclusion policy"],
  ["content moderation of outputs", "rendering", "policy authorship"],
  [pro("A single consolidated exclusion set mapped per modality keeps safety constraints uniform across image, story, and audio instead of per-block afterthoughts", "one 'no real minors' rule mapped to image negatives, story themes, and audio script")],
  [con("A blanket exclusion set can be over-broad and suppress legitimate content in one modality to satisfy another", "an image-motivated exclusion needlessly blocks a benign story theme")],
  ["an exclusion applied to one block but missed in another", "exclusion set diverges from the carried contract"],
  ["the same disallowed-content set is mapped onto every block's exclusion mechanism"],
  ["prohibited-content policy changes", "an exclusion is found missing in one modality"],
  specialists=["safety_engineer", "prompt_engineer"], subfields=["safety_constraints", "negative_prompting"]))

N.append(node("CONSENT_REFERENCE", "consent_and_likeness_reference",
  "Specify the consent-reference mechanism for any depiction or voice of an identifiable real person: a structured reference to a consent record carried in-band, defaulting to absent (which forces non-impersonation/non-likeness), so identity use is gated on explicit, traceable consent.",
  "contract", ["IN_BAND_CONTRACT", "AUDIO_BRIEF_SPEC"], ["CQ_14"],
  b(0.85, 0.76, 0.72, 0.58, 0.88, 0.78, 0.66, 0.62, 0.55, 0.78, 0.8, 0.84, 0.62, 0.22, 0.62, [0.2, 0.5], "consent model, likeness law, or reference format changes"),
  ["structured consent-record reference", "default-absent gating", "likeness and voice coverage"],
  ["obtaining consent itself", "legal validity adjudication", "rendering"],
  [pro("A default-absent, in-band consent reference makes real-person likeness and voice use traceable and gated, not implicit", "consent_ref empty => image must not depict a recognizable real person; populated => allowed and logged")],
  [con("A reference cannot verify the consent record's validity, only assert it exists, so a forged reference still passes structurally", "a consent_ref points to a record that does not actually grant the use")],
  ["likeness used with an empty consent reference", "consent reference present but pointing to an invalid record"],
  ["any real-person likeness or voice carries a populated, in-band consent reference; otherwise it is gated off"],
  ["consent or likeness policy changes", "a likeness is used without a valid consent reference"],
  specialists=["safety_engineer", "policy_contract_author"], contradictors=["likeness_by_default_advocate"],
  subfields=["consent_modeling", "likeness_rights"]))

N.append(node("ARTIFACT_SERIALIZATION", "deterministic_artifact_serialization",
  "Serialize the assembled brief into a canonical, deterministic on-the-wire form (stable field order, explicit encoding, schema_version header) so the same logical brief always serializes identically and the downstream parser has an unambiguous input.",
  "handoff", ["PROMPT_ARTIFACT_SCHEMA", "DETERMINISM_PROVENANCE"], ["CQ_14"],
  b(0.8, 0.74, 0.7, 0.62, 0.78, 0.76, 0.58, 0.66, 0.48, 0.78, 0.8, 0.8, 0.66, 0.2, 0.5, [0.2, 0.5], "serialization format or canonicalization rules change"),
  ["canonical field ordering", "explicit encoding declaration", "schema_version header in the wire form"],
  ["transport security", "downstream parsing logic", "rendering"],
  [pro("Canonical, deterministic serialization gives a stable wire form so identical briefs are byte-identical and the parser input is unambiguous", "stable key order plus UTF-8 makes two builds of the same brief diff-clean")],
  [con("Canonicalization constrains the wire form and can complicate streaming or partial emission", "a strict canonical order blocks emitting blocks as they are ready")],
  ["non-deterministic field order breaks byte-level reproducibility", "encoding left implicit, parsed wrongly downstream"],
  ["the same logical brief serializes to an identical canonical wire form on every build"],
  ["serialization format changes", "a non-deterministic serialization is observed"],
  specialists=["integration_engineer", "prompt_engineer"], subfields=["canonical_serialization", "wire_format"]))

N.append(node("SCHEMA_VERSION_PIN", "schema_version_negotiation_pin",
  "Pin and negotiate the artifact schema_version against the downstream parser's supported range, so producer and consumer agree on the shape before emission and a version mismatch is caught here rather than as a silent misparse downstream.",
  "validation", ["PROMPT_ARTIFACT_SCHEMA", "TARGET_MODEL_PROFILE"], ["CQ_13"],
  b(0.79, 0.74, 0.7, 0.58, 0.76, 0.78, 0.58, 0.66, 0.48, 0.78, 0.82, 0.79, 0.66, 0.18, 0.48, [0.2, 0.48], "schema versioning policy or downstream supported-range changes"),
  ["schema_version pin", "downstream supported-range check", "mismatch detection"],
  ["schema authoring", "downstream parsing internals", "rendering"],
  [pro("Negotiating the schema version against the consumer's supported range catches a shape mismatch at emission instead of as a silent downstream misparse", "producer emits v2 only if the consumer advertises support for v2")],
  [con("Strict version negotiation can block a brief when a consumer lags behind a new schema version", "a consumer stuck on v1 cannot receive a v2-only block")],
  ["version mismatch silently misparsed downstream", "producer emits a version the consumer cannot read"],
  ["the pinned schema_version lies within the downstream parser's supported range before emission"],
  ["schema versioning policy changes", "a version mismatch reaches the downstream parser"],
  specialists=["integration_engineer", "schema_designer"], subfields=["schema_versioning", "version_negotiation"]))

N.append(node("CONTRACT_ATTESTATION", "downstream_compliance_attestation_requirement",
  "Require the downstream consumer to attest, as its own precondition, that it has parsed and will honor the in-band contract (apply the disclosure label, respect usage restrictions) before it generates, turning structural contract presence into a behavioral guarantee at the consumer boundary.",
  "contract", ["IN_BAND_CONTRACT", "SEPARATION_OF_CONCERNS"], ["CQ_14"],
  b(0.86, 0.78, 0.74, 0.6, 0.88, 0.82, 0.66, 0.6, 0.58, 0.76, 0.78, 0.86, 0.6, 0.24, 0.64, [0.24, 0.56], "consumer attestation protocol or contract honoring requirements change"),
  ["consumer attestation requirement", "honor-before-generate precondition", "behavioral-guarantee handshake"],
  ["enforcement inside the consumer", "rendering", "policy authorship"],
  [pro("Requiring an attestation makes the carried contract a behavioral precondition of generation, not merely structurally present text the consumer may ignore (Meyer design-by-contract: the consumer's assume becomes its obligation)", "consumer must echo 'disclosure applied' before any pixels are produced")],
  [con("This stage can require attestation but cannot enforce it inside a non-cooperative consumer, so a malicious consumer can still lie", "a rogue downstream tool attests then drops the label")],
  ["consumer parses the contract but never attests, generating anyway", "attestation faked by a non-compliant consumer"],
  ["the brief requires a consumer attestation of contract compliance as a precondition of downstream generation"],
  ["attestation protocol changes", "a consumer generates without attesting"],
  specialists=["safety_engineer", "policy_contract_author"], contradictors=["trust_consumer_implicitly_advocate"],
  subfields=["design_by_contract", "compliance_attestation"]))

N.append(node("EMISSION_AUDIT_LOG", "emission_audit_and_replay_log",
  "Write an emission audit log: for each emitted brief, record its schema_version, contract version, disclosure directive, provenance map digest, and target profile, so emissions are auditable and a brief can be reconstructed or recalled if a policy later changes.",
  "validation", ["HANDOFF_VALIDATION", "ARTIFACT_SERIALIZATION"], ["CQ_14"],
  b(0.76, 0.72, 0.7, 0.55, 0.72, 0.74, 0.52, 0.68, 0.42, 0.8, 0.82, 0.76, 0.68, 0.16, 0.42, [0.18, 0.44], "audit-log schema or retention/recall policy changes"),
  ["per-emission audit record", "contract/disclosure/version capture", "replay and recall support"],
  ["downstream generation logging", "transport", "rendering"],
  [pro("An emission audit log makes every brief reconstructable and recallable, so a later policy change can identify and reissue affected briefs", "a disclosure-law update finds all emissions with the old directive via the log")],
  [con("Audit logging adds storage and a retention obligation, and the log itself must be access-controlled", "logged contract text and provenance become sensitive records to protect")],
  ["emission not logged, so an affected brief cannot be found after a policy change", "log records a digest that does not match the emitted artifact"],
  ["every emitted brief writes an audit record capturing its schema/contract/disclosure versions and provenance digest"],
  ["audit-log or recall policy changes", "an emission is found with no audit record"],
  specialists=["qa_engineer", "compliance_reviewer"], subfields=["audit_logging", "recall_support"]))

# ---- competency questions (12) ----
CQ = [
 ("CQ_01", "What fixed, versioned schema does the emitted prompt-brief artifact take, and which blocks are required versus optional?", ["nodes", "glossary"], "PROMPT_ARTIFACT_SCHEMA defines the versioned block set with image/story required and audio optional", ["PROMPT_ARTIFACT_SCHEMA"]),
 ("CQ_02", "How is every prompt field grounded in the dense upstream machine_summary rather than invented?", ["nodes"], "GROUND_FROM_SUMMARY maps each field to a named summary fact under a no-invention rule", ["GROUND_FROM_SUMMARY"]),
 ("CQ_03", "How is the image-prompt block structured for text-to-image conditioning, including negative constraints?", ["nodes"], "IMAGE_PROMPT_SPEC defines subject/style/composition/lighting plus a negative-constraint list", ["IMAGE_PROMPT_SPEC"]),
 ("CQ_04", "How is the story/script-prompt block structured to control premise, beats, voice, and length?", ["nodes"], "STORY_PROMPT_SPEC defines premise/beats/voice/tone/length fields", ["STORY_PROMPT_SPEC"]),
 ("CQ_05", "How is the optional audio-brief specified so synthesis is non-impersonating by default unless consent is referenced?", ["nodes"], "AUDIO_BRIEF_SPEC defines script + abstract voice description + non-impersonation default + consent slot", ["AUDIO_BRIEF_SPEC"]),
 ("CQ_06", "How does the safety/disclosure contract travel in-band so a stateless downstream session cannot miss it?", ["nodes", "conflict_axes"], "IN_BAND_CONTRACT embeds disclosure tokens and assume/guarantee clauses in the payload, closing the SPOF", ["IN_BAND_CONTRACT"]),
 ("CQ_07", "How is the mandatory synthetic-media disclosure label guaranteed to be present in the artifact?", ["nodes"], "DISCLOSURE_SLOT enforces a non-empty label-and-placement slot", ["DISCLOSURE_SLOT", "IN_BAND_CONTRACT"]),
 ("CQ_08", "How is scope contained so this stage produces prompts and never renders or publishes?", ["nodes", "conflict_axes"], "SEPARATION_OF_CONCERNS enforces a no-render/no-publish boundary", ["SEPARATION_OF_CONCERNS"]),
 ("CQ_09", "How is the artifact emitted now and made safe to inject into a separate stateless downstream session later?", ["nodes", "workflow"], "DOWNSTREAM_HANDOFF serializes a self-contained artifact for a stateless boundary", ["DOWNSTREAM_HANDOFF", "IN_BAND_CONTRACT"]),
 ("CQ_10", "How is the brief made reproducible and traceable to the facts and contract clauses it drew on?", ["nodes"], "DETERMINISM_PROVENANCE records a field-to-source map and a deterministic build", ["DETERMINISM_PROVENANCE", "GROUND_FROM_SUMMARY"]),
 ("CQ_11", "How is downstream parseability and contract presence validated before the artifact is emitted?", ["nodes", "workflow"], "HANDOFF_VALIDATION schema-validates blocks and checks disclosure/contract presence", ["HANDOFF_VALIDATION", "DOWNSTREAM_HANDOFF"]),
 ("CQ_12", "Why must the disclosure/safety contract travel in-band, and what is the single point of failure if it does not?", ["nodes", "edge_cases"], "IN_BAND_CONTRACT and DOWNSTREAM_HANDOFF show that an out-of-band contract is dropped at the stateless boundary (the SPOF)", ["IN_BAND_CONTRACT", "DOWNSTREAM_HANDOFF", "SEPARATION_OF_CONCERNS"]),
 ("CQ_13", "How are prompts shaped to the downstream model's actual capabilities and framed with role, constraint, brief-objective, and a negotiated schema version?", ["nodes"], "TARGET_MODEL_PROFILE, ROLE_FRAMING, BRIEF_OBJECTIVE_FRAME, and SCHEMA_VERSION_PIN shape prompts to the consumer", ["TARGET_MODEL_PROFILE", "ROLE_FRAMING", "BRIEF_OBJECTIVE_FRAME", "SCHEMA_VERSION_PIN"]),
 ("CQ_14", "How are safety exclusions, consent for real-person likeness/voice, deterministic serialization, consumer attestation, and emission audit handled across the brief?", ["nodes", "edge_cases"], "NEGATIVE_CONSTRAINT_SET, CONSENT_REFERENCE, ARTIFACT_SERIALIZATION, CONTRACT_ATTESTATION, and EMISSION_AUDIT_LOG cover safety, consent, serialization, attestation, and audit", ["NEGATIVE_CONSTRAINT_SET", "CONSENT_REFERENCE", "ARTIFACT_SERIALIZATION", "CONTRACT_ATTESTATION", "EMISSION_AUDIT_LOG"]),
]
CQS = [{"id": i, "question": q, "must_be_answerable_from": m, "acceptance_condition": a, "covered_by": c} for (i, q, m, a, c) in CQ]

# consolidate node->CQ references (dense target 10-14; each node refs 1-2; every CQ covered)
CQ_MAP = {
 "PROMPT_ARTIFACT_SCHEMA": ["CQ_01"],
 "GROUND_FROM_SUMMARY": ["CQ_02", "CQ_10"],
 "IMAGE_PROMPT_SPEC": ["CQ_03"],
 "STORY_PROMPT_SPEC": ["CQ_04"],
 "AUDIO_BRIEF_SPEC": ["CQ_05"],
 "IN_BAND_CONTRACT": ["CQ_06", "CQ_12"],
 "DISCLOSURE_SLOT": ["CQ_07"],
 "SEPARATION_OF_CONCERNS": ["CQ_08", "CQ_12"],
 "DOWNSTREAM_HANDOFF": ["CQ_09", "CQ_12"],
 "DETERMINISM_PROVENANCE": ["CQ_10"],
 "HANDOFF_VALIDATION": ["CQ_11"],
 "TARGET_MODEL_PROFILE": ["CQ_13"],
 "ROLE_FRAMING": ["CQ_13"],
 "BRIEF_OBJECTIVE_FRAME": ["CQ_13"],
 "NEGATIVE_CONSTRAINT_SET": ["CQ_14"],
 "CONSENT_REFERENCE": ["CQ_14"],
 "ARTIFACT_SERIALIZATION": ["CQ_14"],
 "SCHEMA_VERSION_PIN": ["CQ_13"],
 "CONTRACT_ATTESTATION": ["CQ_14", "CQ_06"],
 "EMISSION_AUDIT_LOG": ["CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

# ---- glossary ----
GL = [
 ("prompt_brief_artifact", "the structured, versioned object this stage emits, bundling image/story/optional-audio prompt blocks with a disclosure and in-band contract header", ["downstream_brief", "prompt_handoff"], ["rendered_media", "published_post"], ["PROMPT_ARTIFACT_SCHEMA", "DOWNSTREAM_HANDOFF"]),
 ("in_band_contract", "the safety/disclosure/usage clauses carried inside the artifact payload so a stateless consumer receives them with the prompt", ["carried_contract", "embedded_contract"], ["out_of_band_assumption", "referenced_policy"], ["IN_BAND_CONTRACT", "DISCLOSURE_SLOT"]),
 ("negative_constraint", "an explicit list of content the downstream image model must avoid, mapped onto diffusion negative prompting", ["negative_prompt", "exclusion_list"], ["positive_subject", "style_token"], ["IMAGE_PROMPT_SPEC"]),
 ("disclosure_slot", "a mandatory non-empty artifact field naming the synthetic-media label and where it must appear", ["label_slot", "synthetic_media_label"], ["optional_caption"], ["DISCLOSURE_SLOT", "IN_BAND_CONTRACT"]),
 ("stateless_handoff", "a session boundary across which no prior context is shared, so the artifact must be fully self-contained", ["cross_session_boundary", "cold_handoff"], ["shared_context_call"], ["DOWNSTREAM_HANDOFF", "HANDOFF_VALIDATION"]),
 ("separation_of_concerns", "the discipline of producing prompts only and delegating rendering/publishing downstream, containing this stage's scope", ["scope_containment", "responsibility_split"], ["end_to_end_pipeline"], ["SEPARATION_OF_CONCERNS"]),
 ("assume_guarantee_clause", "a design-by-contract pair stating what the producer guarantees and what the consumer must assume, carried in-band", ["precondition_postcondition", "contract_clause"], ["informal_expectation"], ["IN_BAND_CONTRACT", "DETERMINISM_PROVENANCE"]),
 ("non_impersonation_default", "the audio-brief default that voice is steered by abstract description rather than a named real person absent a consent reference", ["no_voice_clone_default"], ["voice_clone_request"], ["AUDIO_BRIEF_SPEC"]),
 ("provenance_pointer", "a reference from a prompt field to the machine_summary fact or contract clause that produced it", ["source_pointer", "field_trace"], ["invented_detail"], ["DETERMINISM_PROVENANCE", "GROUND_FROM_SUMMARY"]),
]
GLS = [{"term": t, "definition": d, "synonyms": s, "not_same_as": ns, "used_by_nodes": u} for (t, d, s, ns, u) in GL]

# ---- edges ----
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
              "risk_of_conflict": "unmanaged tension degrades the brief or its safety guarantee", "example": "see resolution_rule"})
def rel(f, t, et, rs, cc=0.7, cp=0.2, erc=0.3, why=""):
    E.append({"from": f, "to": t, "edge_type": et, "relation_strength": rs, "signed_tension": 0.0,
              "causal_confidence": cc, "conflict_probability": cp, "expected_rework_cost": erc,
              "why_related": why or f"{f} {et} {t}", "benefit_of_coupling": "coordinated behavior",
              "risk_of_conflict": "inconsistency if uncoordinated", "example": f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies)
dep("PROMPT_ARTIFACT_SCHEMA", "GROUND_FROM_SUMMARY", 0.86)
dep("GROUND_FROM_SUMMARY", "IMAGE_PROMPT_SPEC", 0.8)
dep("GROUND_FROM_SUMMARY", "STORY_PROMPT_SPEC", 0.8)
dep("STORY_PROMPT_SPEC", "AUDIO_BRIEF_SPEC", 0.74)
dep("PROMPT_ARTIFACT_SCHEMA", "IN_BAND_CONTRACT", 0.88)
dep("IN_BAND_CONTRACT", "DISCLOSURE_SLOT", 0.88)
dep("PROMPT_ARTIFACT_SCHEMA", "SEPARATION_OF_CONCERNS", 0.82)
dep("IMAGE_PROMPT_SPEC", "DOWNSTREAM_HANDOFF", 0.82)
dep("IN_BAND_CONTRACT", "DOWNSTREAM_HANDOFF", 0.86)
dep("GROUND_FROM_SUMMARY", "DETERMINISM_PROVENANCE", 0.8)
dep("DOWNSTREAM_HANDOFF", "HANDOFF_VALIDATION", 0.86)
dep("DISCLOSURE_SLOT", "HANDOFF_VALIDATION", 0.84)
dep("DETERMINISM_PROVENANCE", "HANDOFF_VALIDATION", 0.78)
dep("SEPARATION_OF_CONCERNS", "HANDOFF_VALIDATION", 0.76)
# dependency edges for the finer-grained work-units (acyclic, mirror node.dependencies)
dep("PROMPT_ARTIFACT_SCHEMA", "TARGET_MODEL_PROFILE", 0.8)
dep("TARGET_MODEL_PROFILE", "ROLE_FRAMING", 0.78)
dep("GROUND_FROM_SUMMARY", "BRIEF_OBJECTIVE_FRAME", 0.76)
dep("IMAGE_PROMPT_SPEC", "NEGATIVE_CONSTRAINT_SET", 0.8)
dep("IN_BAND_CONTRACT", "NEGATIVE_CONSTRAINT_SET", 0.78)
dep("IN_BAND_CONTRACT", "CONSENT_REFERENCE", 0.82)
dep("AUDIO_BRIEF_SPEC", "CONSENT_REFERENCE", 0.76)
dep("DETERMINISM_PROVENANCE", "ARTIFACT_SERIALIZATION", 0.8)
dep("TARGET_MODEL_PROFILE", "SCHEMA_VERSION_PIN", 0.76)
dep("IN_BAND_CONTRACT", "CONTRACT_ATTESTATION", 0.84)
dep("SEPARATION_OF_CONCERNS", "CONTRACT_ATTESTATION", 0.76)
dep("HANDOFF_VALIDATION", "EMISSION_AUDIT_LOG", 0.8)
dep("ARTIFACT_SERIALIZATION", "EMISSION_AUDIT_LOG", 0.78)

# cross-cutting non-dependency edges
rel("DISCLOSURE_SLOT", "DOWNSTREAM_HANDOFF", "constraint", 0.82, why="the disclosure slot must be serialized into the handoff artifact or the contract is lost at the boundary")
rel("IN_BAND_CONTRACT", "SEPARATION_OF_CONCERNS", "constraint", 0.8, why="carrying the contract in-band is what lets this stage refuse to render yet still bind the downstream consumer")
rel("AUDIO_BRIEF_SPEC", "DISCLOSURE_SLOT", "constraint", 0.74, why="synthetic audio inherits the same mandatory disclosure obligation as image and story output")
rel("IMAGE_PROMPT_SPEC", "STORY_PROMPT_SPEC", "similarity", 0.7, why="image and story blocks share the structured-field pattern and must stay mutually consistent on subject and tone")
rel("HANDOFF_VALIDATION", "PROMPT_ARTIFACT_SCHEMA", "feedback", 0.74, why="validation failures that recur drive a schema revision to make the missing field structurally required")
rel("SEPARATION_OF_CONCERNS", "DOWNSTREAM_HANDOFF", "causal", 0.78, why="because this stage does not render, the handoff is the mechanism by which generation happens at all")
rel("CONTRACT_ATTESTATION", "HANDOFF_VALIDATION", "feedback", 0.74, why="the required consumer attestation is checked for presence as part of pre-emission validation")

# conflict edges (negative signed_tension + resolution_rule) -- 4 edges
conf("IN_BAND_CONTRACT", "DOWNSTREAM_HANDOFF", 0.78, -0.6,
  "always serialize the full contract into the artifact even though it enlarges the payload: a self-contained, larger handoff is mandatory over a smaller one that references an out-of-band contract the stateless session cannot fetch",
  "minimizing handoff payload size conflicts with carrying the entire safety/disclosure contract in-band")
conf("GROUND_FROM_SUMMARY", "IMAGE_PROMPT_SPEC", 0.68, -0.45,
  "ground all factual fields (subject, named entities) strictly in the summary, but allow stylistic fields (style, lighting, mood) to add connotative detail not in the summary, marking them as non-factual embellishment in provenance",
  "strict summary-grounding conflicts with the evocative detail that makes an image prompt vivid")
conf("SEPARATION_OF_CONCERNS", "DOWNSTREAM_HANDOFF", 0.66, -0.4,
  "keep render/publish strictly downstream; accept the extra handoff latency and serialization round-trip as the price of scope containment rather than collapsing the stages into one inline pipeline",
  "a strict no-render boundary conflicts with the lower latency of an end-to-end inline pipeline")
conf("AUDIO_BRIEF_SPEC", "DISCLOSURE_SLOT", 0.64, -0.4,
  "default audio to non-impersonating abstract description AND require the disclosure label; when a consent reference is supplied it relaxes impersonation but never the disclosure obligation",
  "precise voice control via a named reference conflicts with the non-impersonation default and the mandatory disclosure obligation")

# ---- conflict axes (9) ----
CA = [
 {"name": "in_band_contract_vs_payload_size", "description": "Carrying the full safety/disclosure contract in the payload guarantees a stateless consumer receives it, but enlarges the handoff versus an out-of-band reference.", "poles": ["full_in_band_contract", "minimal_referenced_payload"], "resolution_hint": "always carry the contract in-band; size is the accepted cost of closing the SPOF", "tension_score": 0.78, "affected_nodes": ["IN_BAND_CONTRACT", "DOWNSTREAM_HANDOFF", "DISCLOSURE_SLOT"]},
 {"name": "summary_grounding_vs_evocative_detail", "description": "Strictly grounding fields in the machine_summary keeps the brief auditable but can starve prompts of vivid connotative detail.", "poles": ["strict_grounding", "evocative_embellishment"], "resolution_hint": "ground factual fields strictly; allow marked stylistic embellishment in provenance", "tension_score": 0.6, "affected_nodes": ["GROUND_FROM_SUMMARY", "IMAGE_PROMPT_SPEC", "STORY_PROMPT_SPEC"]},
 {"name": "scope_containment_vs_inline_pipeline", "description": "Producing prompts only contains scope and blast radius but adds a handoff versus an end-to-end inline generator.", "poles": ["prompts_only_boundary", "end_to_end_inline"], "resolution_hint": "keep the no-render boundary; accept handoff latency as the price of containment", "tension_score": 0.68, "affected_nodes": ["SEPARATION_OF_CONCERNS", "DOWNSTREAM_HANDOFF", "PROMPT_ARTIFACT_SCHEMA"]},
 {"name": "non_impersonation_default_vs_voice_fidelity", "description": "Defaulting audio to abstract non-impersonating description protects identity but yields less precise voice control than a named reference.", "poles": ["non_impersonation_default", "named_voice_fidelity"], "resolution_hint": "abstract description by default; relax only with an in-band consent reference, never the disclosure", "tension_score": 0.62, "affected_nodes": ["AUDIO_BRIEF_SPEC", "DISCLOSURE_SLOT", "IN_BAND_CONTRACT"]},
 {"name": "schema_rigidity_vs_media_extensibility", "description": "A closed versioned schema makes the brief reliably parseable but lags new media types until it is extended.", "poles": ["closed_schema", "open_extensible_schema"], "resolution_hint": "version the schema and extend deliberately; forbid out-of-band side channels", "tension_score": 0.58, "affected_nodes": ["PROMPT_ARTIFACT_SCHEMA", "IMAGE_PROMPT_SPEC", "AUDIO_BRIEF_SPEC"]},
 {"name": "self_contained_artifact_vs_context_duplication", "description": "A self-contained handoff eliminates dropped-context failures but duplicates context the upstream already holds.", "poles": ["fully_self_contained", "referenced_context"], "resolution_hint": "serialize everything the consumer needs; duplication is cheaper than a dropped contract", "tension_score": 0.64, "affected_nodes": ["DOWNSTREAM_HANDOFF", "DETERMINISM_PROVENANCE", "IN_BAND_CONTRACT"]},
 {"name": "prompt_specificity_vs_conditioning_dilution", "description": "More structured prompt tokens steer generation precisely but, over-stuffed, dilute the conditioning signal and degrade coherence.", "poles": ["maximal_specificity", "lean_conditioning"], "resolution_hint": "prefer a few high-signal fields plus a negative list over many competing tokens", "tension_score": 0.56, "affected_nodes": ["IMAGE_PROMPT_SPEC", "STORY_PROMPT_SPEC", "GROUND_FROM_SUMMARY"]},
 {"name": "validation_strictness_vs_emission_throughput", "description": "Strict pre-emission validation guarantees a parseable, contract-bearing artifact but rejects more briefs and slows emission.", "poles": ["strict_validation", "permissive_emission"], "resolution_hint": "hard-fail only on missing blocks, empty disclosure, or absent contract; warn on the rest", "tension_score": 0.55, "affected_nodes": ["HANDOFF_VALIDATION", "DISCLOSURE_SLOT", "DOWNSTREAM_HANDOFF"]},
 {"name": "structural_presence_vs_downstream_compliance", "description": "Validating that the contract is structurally present does not guarantee a permissive downstream consumer will honor it.", "poles": ["structural_guarantee", "behavioral_compliance"], "resolution_hint": "guarantee presence here; require the consumer to attest compliance as its own precondition", "tension_score": 0.6, "affected_nodes": ["HANDOFF_VALIDATION", "IN_BAND_CONTRACT", "SEPARATION_OF_CONCERNS"]},
]

# ---- edge cases (12) ----
EC = [
 {"description": "The disclosure/safety contract is assumed to be known downstream and is left out of the artifact; the stateless session never receives it and generates unlabeled synthetic media.", "trigger": "contract carried out-of-band by reference instead of in-band (the single point of failure)", "affected_nodes": ["IN_BAND_CONTRACT", "DOWNSTREAM_HANDOFF", "DISCLOSURE_SLOT"], "mitigation": "embed the full contract and disclosure tokens in the payload; validation hard-fails an artifact lacking them", "severity": "critical"},
 {"description": "The disclosure slot is left empty yet the artifact still passes as valid, so downstream output carries no synthetic-media label.", "trigger": "schema does not enforce a non-empty disclosure slot", "affected_nodes": ["DISCLOSURE_SLOT", "HANDOFF_VALIDATION"], "mitigation": "make the slot a required non-empty field with a placement directive; reject empty slots at validation", "severity": "critical"},
 {"description": "This stage acquires a render or publish path and generates media inline, collapsing the separation of concerns and widening the blast radius.", "trigger": "scope creep wires a generation/publish capability into the prompt stage", "affected_nodes": ["SEPARATION_OF_CONCERNS", "DOWNSTREAM_HANDOFF"], "mitigation": "assert no render/publish capability is wired in; the only output is a prompt-brief artifact", "severity": "high"},
 {"description": "An audio brief requests a named real person's voice with no consent reference, inviting impersonation.", "trigger": "named-voice request without a populated consent slot", "affected_nodes": ["AUDIO_BRIEF_SPEC", "IN_BAND_CONTRACT"], "mitigation": "default to abstract non-impersonating description; require an in-band consent reference to name a real person", "severity": "high"},
 {"description": "A prompt field invents a subject or entity absent from the machine_summary, producing a brief untraceable to source facts.", "trigger": "grounding rule not enforced; free invention permitted", "affected_nodes": ["GROUND_FROM_SUMMARY", "DETERMINISM_PROVENANCE"], "mitigation": "require each factual field to cite a summary fact; mark stylistic embellishment separately in provenance", "severity": "high"},
 {"description": "The handoff relies on this session's context (an id or memory) that the fresh downstream session cannot resolve, so the brief fails when injected.", "trigger": "artifact references shared state instead of being self-contained", "affected_nodes": ["DOWNSTREAM_HANDOFF", "HANDOFF_VALIDATION"], "mitigation": "serialize everything the consumer needs into the payload; validation rejects unresolved references", "severity": "high"},
 {"description": "The image block omits the negative-constraint list, so disallowed content (watermarks, text, artifacts) reappears in generations.", "trigger": "negative list treated as optional and left empty", "affected_nodes": ["IMAGE_PROMPT_SPEC"], "mitigation": "require a non-empty negative-constraint list in the image block schema", "severity": "medium"},
 {"description": "An over-stuffed image prompt with many competing style tokens dilutes conditioning and yields an incoherent, averaged image.", "trigger": "too many low-signal tokens crammed into the prompt", "affected_nodes": ["IMAGE_PROMPT_SPEC", "GROUND_FROM_SUMMARY"], "mitigation": "favor a few high-signal fields plus a negative list; cap competing style tokens", "severity": "medium"},
 {"description": "A provenance pointer goes stale after a field is edited, so an auditor is misled about a field's source.", "trigger": "field edited without updating its provenance pointer", "affected_nodes": ["DETERMINISM_PROVENANCE", "GROUND_FROM_SUMMARY"], "mitigation": "rebuild provenance on every field edit; validation flags fields without a current source pointer", "severity": "medium"},
 {"description": "The story prompt omits a length target, so the downstream model produces unbounded output that overruns its budget.", "trigger": "length field left unset in the story block", "affected_nodes": ["STORY_PROMPT_SPEC"], "mitigation": "require a target length and structural bounds in the story block schema", "severity": "low"},
 {"description": "The artifact passes this stage's validator but the downstream parser is stricter, so a brief valid here still fails there.", "trigger": "downstream parser contract diverges from this stage's validation contract", "affected_nodes": ["HANDOFF_VALIDATION", "DOWNSTREAM_HANDOFF"], "mitigation": "pin the validator to the downstream parser contract and version them together", "severity": "medium"},
 {"description": "The contract is present in-band but the downstream consumer parses the brief and then ignores the disclosure clause.", "trigger": "structural presence guaranteed but behavioral compliance not required of the consumer", "affected_nodes": ["IN_BAND_CONTRACT", "HANDOFF_VALIDATION", "SEPARATION_OF_CONCERNS"], "mitigation": "require the consumer to attest contract compliance as its own precondition before generating", "severity": "high"},
]

# ---- workflow (12) ----
WF = [
 {"action": "fix_artifact_schema", "node_ref": "PROMPT_ARTIFACT_SCHEMA", "description": "Pin the versioned artifact schema: header plus image/story required blocks and an optional audio block.", "artifact": "versioned_schema", "gate": "schema_version set and required/optional blocks declared"},
 {"action": "ground_fields_in_summary", "node_ref": "GROUND_FROM_SUMMARY", "description": "Map each factual prompt field to a named machine_summary fact under a no-invention rule.", "artifact": "field_to_fact_map", "gate": "every factual field cites a summary fact"},
 {"action": "build_image_block", "node_ref": "IMAGE_PROMPT_SPEC", "description": "Populate subject/style/composition/lighting and a non-empty negative-constraint list.", "artifact": "image_prompt_block", "gate": "subject present and negative list non-empty"},
 {"action": "build_story_block", "node_ref": "STORY_PROMPT_SPEC", "description": "Populate premise, beat outline, voice/POV, tone, and a target length.", "artifact": "story_prompt_block", "gate": "premise, beats, and length present"},
 {"action": "build_optional_audio_block", "node_ref": "AUDIO_BRIEF_SPEC", "description": "If audio is requested, populate script and an abstract voice description with a non-impersonation default or a consent reference.", "artifact": "audio_brief_block", "gate": "non-impersonation default or populated consent reference present"},
 {"action": "embed_in_band_contract", "node_ref": "IN_BAND_CONTRACT", "description": "Embed disclosure tokens, usage restrictions, and assume/guarantee clauses with a contract version into the payload.", "artifact": "in_band_contract_block", "gate": "contract clauses and version present in payload"},
 {"action": "enforce_disclosure_slot", "node_ref": "DISCLOSURE_SLOT", "description": "Populate the mandatory synthetic-media label and its placement directive.", "artifact": "disclosure_directive", "gate": "disclosure slot non-empty with label and placement"},
 {"action": "assert_no_render_boundary", "node_ref": "SEPARATION_OF_CONCERNS", "description": "Assert this stage exposes no render or publish capability; output is the brief only.", "artifact": "scope_boundary_assertion", "gate": "no render/publish path present"},
 {"action": "record_provenance", "node_ref": "DETERMINISM_PROVENANCE", "description": "Record the field-to-source map and a deterministic build record for reproducibility.", "artifact": "provenance_map", "gate": "every field has a current source pointer"},
 {"action": "serialize_handoff", "node_ref": "DOWNSTREAM_HANDOFF", "description": "Serialize a self-contained artifact for injection into a separate stateless downstream session.", "artifact": "serialized_handoff_artifact", "gate": "artifact is self-contained with no unresolved references"},
 {"action": "validate_before_emit", "node_ref": "HANDOFF_VALIDATION", "description": "Schema-validate all blocks and confirm disclosure and in-band contract are present and non-empty.", "artifact": "validation_report", "gate": "no missing block, empty disclosure, or absent contract"},
 {"action": "emit_artifact", "node_ref": "DOWNSTREAM_HANDOFF", "description": "Emit the validated prompt-brief artifact for downstream injection; this stage does not render or publish.", "artifact": "emitted_prompt_brief", "gate": "validated artifact emitted; no media rendered here"},
]

# ---- dominance rules (9) ----
DR = [
 {"rule": "The in-band contract and disclosure tokens must be embedded in the payload before DOWNSTREAM_HANDOFF emits the artifact", "rationale": "a stateless downstream session that never receives the contract is the single point of failure; an out-of-band contract is silently dropped", "trigger": "emission attempted without the contract embedded", "action": "block emission until the contract and disclosure tokens are in the payload"},
 {"rule": "The disclosure slot must be non-empty before HANDOFF_VALIDATION passes", "rationale": "an empty disclosure slot yields unlabeled synthetic media downstream", "trigger": "validation run on an artifact with an empty disclosure slot", "action": "fail validation and route back to DISCLOSURE_SLOT"},
 {"rule": "SEPARATION_OF_CONCERNS must hold: no render or publish capability may exist in this stage", "rationale": "scope containment keeps a bug here from publishing unsafe media", "trigger": "a render or publish path is detected in the stage", "action": "remove the path and re-assert the no-render boundary"},
 {"rule": "Factual prompt fields must be grounded in the machine_summary before IMAGE_PROMPT_SPEC or STORY_PROMPT_SPEC finalize", "rationale": "invented subjects make the brief untraceable and unsafe", "trigger": "a factual field has no summary source", "action": "block finalization until the field cites a summary fact or is marked stylistic"},
 {"rule": "An audio brief naming a real person must carry an in-band consent reference, otherwise it defaults to non-impersonating description", "rationale": "voice impersonation without consent is an identity and legal risk", "trigger": "named-voice request without a consent reference", "action": "force the non-impersonation default or require a populated consent reference"},
 {"rule": "DOWNSTREAM_HANDOFF must produce a self-contained artifact before HANDOFF_VALIDATION can certify it", "rationale": "a brief that relies on shared state fails in a fresh session", "trigger": "artifact references unresolved shared state", "action": "serialize the referenced context into the payload"},
 {"rule": "PROMPT_ARTIFACT_SCHEMA must be fixed and versioned before any block is populated", "rationale": "populating blocks against an unfixed schema causes rework when the shape changes", "trigger": "block population begins without a pinned schema_version", "action": "pin the schema_version first"},
 {"rule": "DETERMINISM_PROVENANCE pointers must be current before emission so the brief is reproducible and auditable", "rationale": "stale provenance misleads audit and breaks reproducibility", "trigger": "a field edited without a provenance update", "action": "rebuild provenance for the edited field before emission"},
 {"rule": "HANDOFF_VALIDATION must run and pass before the artifact is emitted", "rationale": "a malformed brief should fail here, not in the separate downstream session", "trigger": "emission attempted before validation", "action": "block emission until validation passes"},
]

# ---- anti-rework rules (8) ----
ARR = [
 {"rule": "Do not assume the downstream session knows the contract; carry it in-band, or you will reissue every brief once an unlabeled output is discovered", "prevents": "mass reissue of briefs after a dropped out-of-band contract causes unlabeled synthetic media"},
 {"rule": "Do not leave the disclosure slot optional; retrofitting a mandatory non-empty slot means revalidating every prior artifact", "prevents": "bulk revalidation after unlabeled outputs surface"},
 {"rule": "Do not wire a render or publish path into this stage; removing it later requires re-architecting the pipeline boundary", "prevents": "pipeline re-architecture to restore scope containment"},
 {"rule": "Do not invent prompt details ungrounded in the summary; auditing and removing them later is far costlier than grounding up front", "prevents": "post-hoc audit and rewrite of hallucinated prompt fields"},
 {"rule": "Do not reference this session's context in the handoff; making the artifact self-contained after the fact requires re-serialization of every brief", "prevents": "re-serialization of briefs that broke on a stateless boundary"},
 {"rule": "Do not populate blocks before pinning the schema version; a late schema change forces re-populating every block", "prevents": "re-population of all blocks after a schema reshape"},
 {"rule": "Do not skip provenance recording; reconstructing it for an audit means re-deriving every field from the summary", "prevents": "re-derivation of field provenance during an audit"},
 {"rule": "Do not default audio to a named real voice; switching to a non-impersonation default later means revisiting every audio brief and its consent status", "prevents": "rework of audio briefs to add consent or strip impersonation"},
]

# ---- iteration protocol (8) ----
IP = [
 {"trigger": "an unlabeled synthetic output is found downstream", "action": "confirm the contract and disclosure tokens are embedded in IN_BAND_CONTRACT and that DISCLOSURE_SLOT is enforced non-empty", "nodes": ["IN_BAND_CONTRACT", "DISCLOSURE_SLOT", "HANDOFF_VALIDATION"], "priority": "critical"},
 {"trigger": "a downstream session fails for lack of carried context", "action": "make the DOWNSTREAM_HANDOFF artifact fully self-contained and reject unresolved references in HANDOFF_VALIDATION", "nodes": ["DOWNSTREAM_HANDOFF", "HANDOFF_VALIDATION"], "priority": "high"},
 {"trigger": "a render or publish path appears in this stage", "action": "remove it and re-assert the no-render boundary in SEPARATION_OF_CONCERNS", "nodes": ["SEPARATION_OF_CONCERNS", "DOWNSTREAM_HANDOFF"], "priority": "high"},
 {"trigger": "an ungrounded invented detail is found in a brief", "action": "tighten the no-invention rule in GROUND_FROM_SUMMARY and require a provenance pointer in DETERMINISM_PROVENANCE", "nodes": ["GROUND_FROM_SUMMARY", "DETERMINISM_PROVENANCE"], "priority": "high"},
 {"trigger": "an impersonation request appears without consent", "action": "reinforce the non-impersonation default in AUDIO_BRIEF_SPEC and require an in-band consent reference in IN_BAND_CONTRACT", "nodes": ["AUDIO_BRIEF_SPEC", "IN_BAND_CONTRACT"], "priority": "high"},
 {"trigger": "generations repeatedly violate a negative constraint", "action": "expand the negative-constraint list in IMAGE_PROMPT_SPEC and re-validate the image block", "nodes": ["IMAGE_PROMPT_SPEC", "HANDOFF_VALIDATION"], "priority": "medium"},
 {"trigger": "a brief that passed validation fails to parse downstream", "action": "pin HANDOFF_VALIDATION to the downstream parser contract and version PROMPT_ARTIFACT_SCHEMA with it", "nodes": ["HANDOFF_VALIDATION", "PROMPT_ARTIFACT_SCHEMA"], "priority": "medium"},
 {"trigger": "an audit cannot reproduce a brief from identical inputs", "action": "rebuild DETERMINISM_PROVENANCE pointers and confirm the deterministic build in GROUND_FROM_SUMMARY", "nodes": ["DETERMINISM_PROVENANCE", "GROUND_FROM_SUMMARY"], "priority": "medium"},
]

spec = {
 "domain": "promptgen__downstream_brief",
 "domain_label": "Downstream Prompt-Brief Compilation (image/story/audio)",
 "purpose": "emit_structured_image_story_and_optional_audio_prompts_for_injection_into_a_separate_downstream_agent_session_with_the_safety_contract_carried_in_band",
 "assumptions": [
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "a dense upstream machine_summary supplies the facts every prompt field is grounded in",
   "the downstream agent session shares no state with this one; the boundary is stateless",
   "this stage produces prompts only and never renders, synthesizes, or publishes media",
 ],
 "exclusions": [
   "image/story/audio synthesis and rendering (the downstream session's job)",
   "publishing, distribution, or platform posting",
   "generation of the upstream machine_summary itself",
   "the legal drafting of disclosure-law text (only its carriage in-band is in scope)",
 ],
 "source_description": "heuristic prior estimates for downstream prompt-brief compilation work units, informed by creative-brief practice, prompt-engineering pattern catalogs, latent-diffusion text conditioning, separation of concerns, and design-by-contract; no supplied dataset",
 "source_citation": "Rombach et al. 2022 High-Resolution Image Synthesis with Latent Diffusion Models (text conditioning, classifier-free guidance, negative prompts); White et al. 2023 A Prompt Pattern Catalog to Enhance Prompt Engineering with ChatGPT; Dijkstra 1982 On the Role of Scientific Thought (separation of concerns); Meyer 1992 Applying Design by Contract (assume/guarantee); creative-brief practice (objective/audience/deliverable/constraints)",
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
 "priority_rationale": "PROMPT_ARTIFACT_SCHEMA and GROUND_FROM_SUMMARY are foundational; IN_BAND_CONTRACT/DISCLOSURE_SLOT/SEPARATION_OF_CONCERNS carry the safety scope finding; IMAGE/STORY/AUDIO specs populate the blocks; DOWNSTREAM_HANDOFF, DETERMINISM_PROVENANCE, and HANDOFF_VALIDATION close the emission.",
 "eval_objective": "verify_in_band_contract_carriage_disclosure_enforcement_scope_containment_summary_grounding_and_downstream_parseability_of_promptgen__downstream_brief_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "promptgen__downstream_brief.spec.json")
open(path, "w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes", len(N), "edges", len(E), "CA", len(CA), "EC", len(EC), "WF", len(WF), "CQ", len(CQS), "DR", len(DR), "ARR", len(ARR), "IP", len(IP))
