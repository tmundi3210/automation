#!/usr/bin/env python3
"""Generate the pubrights__likeness_voice content spec (B60 content-intelligence KB)
for kb_forge.py. Compact authoring: node() applies sane defaults so only domain
content + base metric magnitudes are specified per node.

Domain: a SCREENING heuristic for the (item, figure, use) tuple against right of
publicity, likeness and voice-clone consent. It is fail-closed, never legal advice,
never auto-approves, and ESCALATES to qualified counsel. The legal gate dominates."""
import json, os

SRC = "SRC_HEURISTIC_PRIOR"

def node(id, topic, definition, group, deps, cqs, base,
         scope_in, scope_out, pros, cons, fmodes, accept, revisit,
         inputs=None, outputs=None, specialists=None, contradictors=None,
         subfields=None, human_review=False):
    b = dict(base)
    n = {
        "id": id, "topic": topic, "definition": definition, "group": group,
        "node_type": "work_unit",
        "academic_fields": ["entertainment_and_media_law", "ai_governance"],
        "subfields": subfields or ["right_of_publicity", "biometric_identity_consent"],
        "specialists": specialists or ["ip_screening_analyst"],
        "contradictors": contradictors or ["free_expression_maximalist"],
        "inputs": inputs or ["(item,figure,use) tuple", "jurisdiction signal"],
        "outputs": outputs or ["screening signal"],
        "dependencies": deps, "must_not_finalize_before": [],
        "competency_question_refs": cqs, "evidence_refs": [SRC],
        "base": b,
        "scope_boundary": {"included": scope_in, "excluded": scope_out},
        "pros": pros, "cons": cons, "failure_modes": fmodes,
        "acceptance_tests": accept, "revisit_triggers": revisit,
        "handoff_artifact_required": True, "lifecycle_state": "draft",
    }
    if human_review:
        n["human_review_required"] = True
    return n

def b(C,BV,UV,TC,R,X,IR,CF,NCP,AT,DG,PI,EC,FR,DW,ui,sig):
    return {"criticality":C,"business_value":BV,"user_value":UV,"technical_complexity":TC,
            "risk_if_wrong":R,"cross_topic_coupling":X,"irreversibility":IR,"confidence":CF,
            "node_conflict_pressure":NCP,"acceptance_test_pass_rate":AT,"dependency_gate_pass_rate":DG,
            "prior_importance":PI,"evidence_confidence":EC,"failure_rate":FR,"downside_weight":DW,
            "uncertainty_interval":ui,"update_signal":sig}

def pro(claim, ex, w=0.78): return {"weight":w,"claim":claim,"example":ex}
def con(claim, ex, w=0.6): return {"weight":w,"claim":claim,"example":ex}

N = []

N.append(node("USE_INTAKE","use_tuple_capture_and_normalization",
  "Capture and normalize the screening unit as an (item, figure, use) tuple: what artifact is produced, which real identifiable person it depicts or evokes, and the concrete commercial/editorial use, plus the asserted jurisdiction(s) of publication.",
  "foundations", [], ["CQ_01"],
  b(0.9,0.78,0.82,0.5,0.82,0.78,0.55,0.7,0.4, 0.78,0.82, 0.88,0.7,0.18,0.5,[0.2,0.5],"intake schema or tuple definition changes"),
  ["artifact description","identifiable-person assertion","concrete use description","jurisdiction signal"],
  ["legal advice","final allow/block decision","operator execution of the use"],
  [pro("A normalized tuple makes every downstream right, exception, and jurisdiction test resolve against one fixed unit of analysis","'AI voice ad reading a script in the style of a named singer, run in California' is fixed before any rule fires")],
  [con("A thin or ambiguous tuple (vague 'figure', undefined use) silently weakens every later test","'a famous person' with no name or jurisdiction cannot be screened, only escalated")],
  ["figure left unidentified yet treated as screenable","editorial vs commercial use conflated at intake","jurisdiction omitted so the wrong right set is applied"],
  ["tuple has a named-or-clearly-evoked figure, a concrete use, and at least one asserted jurisdiction"],
  ["intake schema revised","new use category (e.g. interactive avatar) appears"],
  specialists=["ip_screening_analyst","intake_reviewer"]))

N.append(node("FIGURE_IDENTIFICATION","identifiability_of_the_person",
  "Determine whether a real, living-or-recently-deceased, identifiable person is depicted or evoked (by name, photoreal image, recognizable voice, catchphrase, or distinctive persona), since publicity rights attach to identifiability, not only to literal name or face.",
  "foundations", ["USE_INTAKE"], ["CQ_01","CQ_02"],
  b(0.9,0.78,0.8,0.6,0.85,0.82,0.55,0.66,0.5, 0.76,0.8, 0.88,0.66,0.2,0.52,[0.22,0.55],"identifiability standard or evocation case law shifts"),
  ["name/face identifiability","voice/persona evocation","robot/look-alike/sound-alike evocation","catchphrase and signature-prop evocation"],
  ["consent verification","jurisdictional descendibility","operator execution"],
  [pro("Anchoring on identifiability, not literal name, captures evocation harms the way White v Samsung and Wendt v Host did","a robot dressed and posed as a game-show hostess 'identified' the plaintiff without name or likeness")],
  [con("Broad evocation standards risk flagging generic archetypes that evoke no specific person","a generic 1950s crooner style may evoke a genre, not one identifiable singer")],
  ["evocation of a distinctive persona missed because no name/face is present","a generic archetype mis-flagged as a specific identifiable person"],
  ["a concrete identifiability theory (name, image, voice, or persona evocation) is recorded for the figure"],
  ["evocation case law shifts","a sound-alike/look-alike incident is mis-screened"],
  specialists=["ip_screening_analyst","persona_rights_specialist"]))

N.append(node("RIGHT_OF_PUBLICITY","commercial_appropriation_right",
  "Apply the core right of publicity: an identifiable person's control over the commercial use of their name, image, and likeness (NIL), so unconsented commercial appropriation of identity to sell or endorse goods is the central prohibited pattern.",
  "rights", ["FIGURE_IDENTIFICATION"], ["CQ_02","CQ_03"],
  b(0.92,0.82,0.82,0.6,0.86,0.82,0.6,0.66,0.55, 0.74,0.78, 0.9,0.66,0.2,0.55,[0.22,0.55],"NIL appropriation doctrine or remedy scope changes"),
  ["name-image-likeness appropriation","commercial endorsement implication","property-style transferable identity interest"],
  ["voice-specific protection","first-amendment/news carve-outs","minor-specific blocks"],
  [pro("The right of publicity gives a concrete, transferable identity interest the screen can reason about, rooted in Haelan v Topps which first named it","Haelan recognized a ball-player's assignable 'right of publicity' in his photograph for trading cards")],
  [con("The right is unevenly recognized and its commercial/expressive boundary is contested, so a single rule over-claims","a use that merely references a celebrity in commentary may not be appropriation at all")],
  ["expressive use mis-treated as commercial appropriation","appropriation found absent any identifiable person"],
  ["a commercial-appropriation theory is stated and tied to an identifiable person and a commercial use"],
  ["NIL appropriation doctrine changes","an endorsement-implication call is disputed"],
  specialists=["ip_screening_analyst","right_of_publicity_specialist"]))

N.append(node("VOICE_PROTECTION","voice_as_protectable_identity",
  "Treat a person's distinctive voice as a protectable element of identity: deliberately imitating or cloning a recognizable voice to sell or anchor a product can violate publicity/identity rights even when no actual recording is used and no name appears.",
  "rights", ["FIGURE_IDENTIFICATION"], ["CQ_02","CQ_04"],
  b(0.9,0.8,0.8,0.66,0.86,0.82,0.62,0.62,0.58, 0.72,0.76, 0.9,0.62,0.22,0.56,[0.24,0.6],"voice-cloning statutes or sound-alike doctrine change"),
  ["deliberate sound-alike imitation","voice clone / synthetic voice models","'voice as signature' identity claim"],
  ["literal master-recording copyright (a separate right)","general likeness","consent paperwork"],
  [pro("Midler v Ford and Waits v Frito-Lay establish that imitating a distinctive voice to sell a product is actionable misappropriation of identity, independent of copyright","Ford hired a Midler sound-alike after she declined; the deliberate imitation of her distinctive voice was actionable")],
  [con("Distinguishing protected distinctive voice from unprotectable style or genre imitation is fact-intensive and jurisdiction-bound","a raspy-blues delivery may be a genre convention rather than one singer's protectable signature")],
  ["synthetic voice clone treated as unprotected because 'no recording was copied'","generic style imitation mis-flagged as protected-voice misappropriation"],
  ["any deliberate recognizable-voice imitation or clone is flagged as a voice-identity risk, not dismissed as 'no recording used'"],
  ["a voice-cloning statute (e.g. Tennessee ELVIS Act) is enacted or amended","a sound-alike call is disputed"],
  specialists=["ip_screening_analyst","voice_rights_specialist"]))

N.append(node("LIKENESS_REPRODUCTION","photoreal_vs_caricature_likeness",
  "Classify how the figure's likeness is reproduced along the photoreal-to-caricature axis, because the reproduction mode (near-photoreal deepfake, stylized portrait, caricature, or abstract evocation) drives both the strength of the identity claim and the availability of expressive exceptions.",
  "rights", ["FIGURE_IDENTIFICATION"], ["CQ_04","CQ_05"],
  b(0.82,0.74,0.74,0.66,0.8,0.78,0.55,0.64,0.55, 0.74,0.76, 0.82,0.64,0.22,0.5,[0.24,0.58],"likeness-reproduction taxonomy or deepfake standards change"),
  ["photoreal/deepfake reproduction","stylized portrait","caricature/parody depiction","abstract persona evocation"],
  ["consent basis","commercial/editorial use class","voice-specific analysis"],
  [pro("Separating photoreal reproduction from caricature lets the screen weight a near-perfect deepfake as a strong identity claim while leaving room for transformative caricature","a photoreal AI face-swap is a far stronger NIL claim than a recognizable political caricature")],
  [con("The photoreal/caricature line does not by itself decide legality; a transformative caricature can still be commercial appropriation","a caricature on merchandise can still appropriate identity for commercial gain")],
  ["photoreal deepfake under-weighted as 'just a stylization'","caricature assumed automatically lawful regardless of commercial use"],
  ["the reproduction mode is recorded and its identity-claim strength is scored, not assumed lawful from style alone"],
  ["deepfake detection or likeness taxonomy changes","a photoreal reproduction is mis-classified as caricature"]))

N.append(node("USE_CLASSIFICATION","commercial_vs_editorial_vs_parody_use",
  "Classify the use as commercial (advertising, endorsement, merchandise), editorial/newsworthy (reporting, commentary, biography), or parody/expressive, because publicity liability turns heavily on this axis and most protective doctrine is anchored to a commercial use.",
  "classification", ["RIGHT_OF_PUBLICITY","LIKENESS_REPRODUCTION"], ["CQ_03","CQ_05"],
  b(0.88,0.8,0.78,0.62,0.84,0.82,0.55,0.62,0.6, 0.72,0.76, 0.86,0.62,0.22,0.55,[0.24,0.6],"commercial-speech / newsworthiness boundary shifts"),
  ["commercial/advertising/endorsement use","editorial/newsworthy use","parody/expressive use","mixed or hybrid use"],
  ["the substantive exception analysis itself","consent verification","operator execution"],
  [pro("Classifying use first lets the screen route a pure news report away from appropriation while keeping a thin 'editorial' label from laundering an ad","a magazine profile is editorial, but the same image on a product box is commercial regardless of an article wrapper")],
  [con("Modern uses are hybrid (branded content, influencer posts) so a clean three-way label can misroute a mixed use","a sponsored 'editorial' video is simultaneously news-styled and an advertisement")],
  ["an advertisement disguised as editorial routed as protected news","genuine commentary mis-routed as commercial appropriation"],
  ["a defensible use class (commercial / editorial / parody / hybrid) is recorded with the dominant-purpose rationale"],
  ["commercial-speech doctrine shifts","a hybrid sponsored use is mis-classified"],
  specialists=["ip_screening_analyst","media_law_specialist"]))

N.append(node("CONSENT_LICENSE","documented_consent_and_lawful_basis",
  "Require documented, scope-matched consent or another lawful basis for any use that implicates identity, voice, or likeness; for voice clones and biometric processing the consent must be specific, informed, and cover the actual use, term, and territory rather than a generic release.",
  "consent", ["RIGHT_OF_PUBLICITY","VOICE_PROTECTION"], ["CQ_06","CQ_07"],
  b(0.94,0.84,0.84,0.62,0.88,0.85,0.7,0.62,0.6, 0.74,0.78, 0.92,0.62,0.2,0.6,[0.22,0.55],"consent / lawful-basis requirements for biometric or voice data change"),
  ["written scope-matched consent","explicit voice-clone / synthetic-media authorization","term and territory coverage","lawful-basis record where consent is absent"],
  ["whether a use needs consent at all (that is the rights analysis)","jurisdictional descendibility","operator execution"],
  [pro("Demanding scope-matched consent turns 'we have a release' into a checkable artifact and blocks scope-creep voice cloning","a stock-photo model release does not authorize generating a synthetic voice clone of that model for an ad")],
  [con("Consent can be defective (coerced, expired, out-of-scope, or from the wrong rights-holder) yet look valid on its face","a release signed for a print campaign is silently stretched to cover an AI avatar")],
  ["generic release treated as voice-clone consent","consent from a non-rights-holder accepted","expired or out-of-territory consent honored"],
  ["a consent/lawful-basis record exists that matches the actual use, voice-clone status, term, and territory, or the use is constrained/blocked"],
  ["biometric/voice consent law changes","a release is found to be out-of-scope for the actual use"],
  specialists=["ip_screening_analyst","privacy_counsel_liaison"], human_review=True))

N.append(node("US_STATE_BY_STATE","us_no_uniform_federal_right",
  "Account for the absence of a uniform US federal right of publicity: protection is state-law-based and materially differs (California Civ. Code 3344 and common law, New York Civil Rights Law 50/51, Tennessee's personal-rights and ELVIS Act regime), so the governing state and its specific scope must be identified.",
  "jurisdiction", ["RIGHT_OF_PUBLICITY","USE_CLASSIFICATION"], ["CQ_03","CQ_08"],
  b(0.86,0.78,0.74,0.7,0.84,0.84,0.6,0.6,0.6, 0.7,0.74, 0.86,0.6,0.24,0.56,[0.28,0.66],"a state publicity statute is enacted or amended"),
  ["California statutory + common-law right","New York Civil Rights Law 50/51","Tennessee personal-rights / ELVIS Act","choice-of-law for the publication"],
  ["non-US jurisdictions","federal copyright","the exception analysis"],
  [pro("Explicitly resolving the governing state prevents applying a permissive state's rule to a use governed by a protective one","New York 50/51 (image/voice, living persons) differs sharply from Tennessee's broad, descendible, voice-clone-aware regime")],
  [con("Multi-state distribution and online publication make a single governing state hard to pin down, inviting forum uncertainty","a clip published nationwide implicates many states' rights at once")],
  ["a permissive state's rule applied to a use governed by a stricter state","online nationwide use treated as single-state"],
  ["the governing US state(s) and the specific statutory/common-law scope applied are recorded"],
  ["a state publicity statute changes","a multi-state use is collapsed to one state in error"],
  specialists=["ip_screening_analyst","us_publicity_law_specialist"]))

N.append(node("INDIA_PERSONALITY_RIGHTS","india_celebrity_persona_injunctions",
  "Account for India's rapidly developing personality-rights regime, where High Courts have granted injunctions protecting a celebrity's name, image, voice, and persona against unauthorized commercial and AI-generated exploitation, so an Indian-facing use of a recognizable persona carries real injunction risk.",
  "jurisdiction", ["RIGHT_OF_PUBLICITY","VOICE_PROTECTION"], ["CQ_08","CQ_09"],
  b(0.82,0.74,0.7,0.68,0.82,0.8,0.6,0.58,0.6, 0.7,0.74, 0.82,0.58,0.26,0.55,[0.3,0.68],"new Indian personality-rights judgment or statute issues"),
  ["name/image/voice/persona protection","AI-generated persona misuse","injunctive-relief risk for Indian-facing use"],
  ["US state law","EU data-protection basis","the consent artifact itself"],
  [pro("Anil Kapoor v Simply Life India (Delhi HC 2023) and DM Entertainment v Baby Gift House show Indian courts will enjoin unauthorized persona and AI exploitation, including voice and likeness","Anil Kapoor's suit secured an injunction protecting his name, image, voice, and signature catchphrase against AI/deepfake misuse")],
  [con("Indian personality rights are judge-developed and still evolving, so scope and durability vary case to case","the contours of post-mortem and persona protection remain unsettled across High Courts")],
  ["Indian-facing use of a recognizable persona screened under a permissive foreign standard","emerging Indian AI-persona protection ignored"],
  ["for India-facing uses, the persona/voice protection and injunction risk are assessed against current Indian case law"],
  ["a new Indian personality-rights judgment issues","an Indian-facing AI-persona use is under-screened"],
  specialists=["ip_screening_analyst","india_ip_specialist"]))

N.append(node("EU_BIOMETRIC_BASIS","eu_gdpr_biometric_and_image_basis",
  "Account for the EU/GDPR layer: a person's image and voice can be personal data, and a voice clone or face model can constitute biometric data (a special category under Art. 9) whose processing needs an explicit lawful basis, independent of any publicity right.",
  "jurisdiction", ["CONSENT_LICENSE"], ["CQ_07","CQ_09"],
  b(0.82,0.74,0.72,0.7,0.82,0.8,0.62,0.58,0.58, 0.7,0.74, 0.82,0.58,0.26,0.56,[0.3,0.68],"GDPR biometric guidance or national image-rights law changes"),
  ["image/voice as personal data","biometric special-category data (Art. 9)","explicit lawful basis for processing","data-subject rights overlay"],
  ["US/India publicity doctrine","the commercial-appropriation analysis","operator execution"],
  [pro("Treating a face/voice model as Art. 9 biometric data adds a consent/lawful-basis requirement that publicity law alone would miss for EU-facing uses","training a voice clone on an EU resident's recordings processes special-category biometric data needing an explicit basis")],
  [con("Not every image or voice use is biometric processing, and lawful bases beyond consent exist, so a blanket 'GDPR blocks it' over-claims","using a photo for editorial reporting may rest on a different lawful basis than consent")],
  ["EU-facing voice clone processed with no Art. 9 basis","ordinary image use mis-labeled as biometric processing"],
  ["for EU-facing uses, image/voice and any biometric processing are checked for an explicit lawful basis"],
  ["GDPR biometric guidance changes","an EU voice-clone use lacks a recorded lawful basis"],
  specialists=["ip_screening_analyst","data_protection_liaison"]))

N.append(node("POST_MORTEM_RIGHTS","descendible_post_mortem_publicity",
  "Resolve post-mortem publicity: in several jurisdictions the right survives death and is descendible for a statutory term (e.g. California 70 years, Tennessee renewable terms protecting Elvis-era estates), while elsewhere it dies with the person, so a deceased figure is not automatically free to use.",
  "jurisdiction", ["US_STATE_BY_STATE"], ["CQ_08","CQ_10"],
  b(0.8,0.72,0.7,0.66,0.82,0.78,0.62,0.58,0.55, 0.7,0.74, 0.8,0.58,0.26,0.55,[0.3,0.66],"a post-mortem term or descendibility rule changes"),
  ["survival and descendibility of the right","statutory post-mortem term","estate/rights-holder identification"],
  ["living-person analysis","EU data-protection (rights largely end at death)","consent paperwork mechanics"],
  [pro("Post-mortem analysis stops the screen from auto-clearing dead celebrities whose estates still control publicity for decades","California's statutory post-mortem right and Tennessee's regime keep Elvis-era and similar personas protected long after death")],
  [con("Post-mortem rules vary wildly by jurisdiction and term, so a single answer about 'is the dead person free' is unsafe","a figure may be protected in California or Tennessee but free to use under a state where the right dies with the person")],
  ["deceased figure auto-cleared as unprotected","wrong rights-holder/estate assumed to hold post-mortem rights"],
  ["for a deceased figure, descendibility and the post-mortem term in the governing jurisdiction are resolved before any clearance"],
  ["a post-mortem statutory term changes","a dead-celebrity use is auto-cleared in error"],
  specialists=["ip_screening_analyst","estate_rights_specialist"]))

N.append(node("PARODY_TRANSFORMATIVE_EXCEPTION","jurisdiction_specific_expressive_exception",
  "Evaluate parody, news, and transformative-use exceptions as jurisdiction-specific and fact-bound defenses, NOT automatic safe harbors: tests such as California's transformative-use test and newsworthiness can defeat a publicity claim, but they must be argued, not assumed, and they vary by forum.",
  "exceptions", ["USE_CLASSIFICATION","US_STATE_BY_STATE"], ["CQ_05","CQ_11"],
  b(0.84,0.74,0.74,0.72,0.85,0.82,0.55,0.58,0.62, 0.7,0.74, 0.84,0.58,0.26,0.56,[0.3,0.68],"transformative-use or newsworthiness test is revised"),
  ["parody / expressive defense","newsworthiness defense","transformative-use test","jurisdiction-specific availability"],
  ["the underlying rights analysis","consent verification","the hard-block conditions"],
  [pro("Framing exceptions as defenses-to-be-argued prevents the screen from rubber-stamping any 'it's parody' claim as automatic clearance","California's transformative-use test asks whether the work adds significant creative expression beyond the likeness, not merely whether it is labeled parody")],
  [con("Exception tests are notoriously indeterminate and forum-dependent, so over-relying on them produces false clearances","the same caricature can be transformative in one circuit's test and infringing under another's predominant-purpose test")],
  ["a labeled 'parody' auto-cleared without an exception analysis","a valid newsworthy use over-blocked by ignoring the defense"],
  ["any claimed exception is recorded as a fact-specific defense with the governing test, never as an automatic safe harbor"],
  ["a transformative-use/newsworthiness precedent changes","a 'parody' label is treated as automatic clearance"]))

N.append(node("MINOR_PROTECTION","minors_and_protected_persons_hard_block",
  "Apply a hard block for minors and other protected/vulnerable persons: any voice clone, deepfake, or commercial likeness use depicting a minor (or a non-consenting protected individual) is blocked and escalated outright, never merely constrained, given heightened consent, child-protection, and biometric concerns.",
  "hard_gate", ["FIGURE_IDENTIFICATION","CONSENT_LICENSE"], ["CQ_12","CQ_13"],
  b(0.95,0.82,0.86,0.55,0.92,0.82,0.78,0.66,0.55, 0.8,0.82, 0.94,0.66,0.16,0.66,[0.18,0.46],"child-protection or protected-class rules change"),
  ["minor / child subject","non-consenting protected individual","synthetic-media-of-minor hard block","mandatory escalation"],
  ["adult-celebrity commercial balancing","exception/parody analysis (does not unblock a minor)","operator execution"],
  [pro("A non-negotiable block for minors fails safe on the highest-harm cases where no commercial or expressive interest can outweigh child protection","an AI voice clone or deepface of an identifiable child is blocked outright regardless of any release")],
  [con("Age or protected-status signals can be uncertain at intake, risking either over-blocking adults or under-detecting minors","a borderline-age figure may be ambiguous from the artifact alone")],
  ["minor depiction passed through as a normal commercial balancing case","protected-status signal ignored because consent paperwork was present"],
  ["any minor or protected-person depiction is blocked and escalated, never constrained or auto-approved"],
  ["child-protection rules change","an ambiguous-age subject is processed without escalation"],
  specialists=["ip_screening_analyst","child_safety_reviewer"], human_review=True))

N.append(node("VOICE_CLONE_CONSENT_GATE","explicit_voice_clone_consent_hard_gate",
  "Apply a hard gate specifically for synthetic voice clones: a recognizable voice clone of a real person for commercial use requires explicit, specific voice-cloning consent (or a clear statutory basis); absent that, the use is blocked, because voice-clone statutes (e.g. Tennessee ELVIS Act 2024) treat unconsented commercial voice cloning as a direct violation.",
  "hard_gate", ["VOICE_PROTECTION","CONSENT_LICENSE"], ["CQ_06","CQ_13"],
  b(0.93,0.82,0.84,0.6,0.9,0.84,0.72,0.62,0.6, 0.78,0.8, 0.92,0.62,0.18,0.62,[0.2,0.5],"voice-cloning statute scope or consent standard changes"),
  ["synthetic voice clone of a real person","explicit voice-clone consent requirement","statutory voice-clone basis","block on unconsented commercial clone"],
  ["literal recording copyright","general likeness","adult-only assumption (minors go to the minor gate)"],
  [pro("Tennessee's ELVIS Act 2024 makes a person's voice a protected property right and targets unconsented AI voice cloning, so a clone-specific consent gate matches the live statutory direction","the ELVIS Act extended Tennessee's right of publicity to expressly cover a person's voice and unauthorized simulation")],
  [con("Voice-clone statutes are new, uneven across jurisdictions, and their commercial/expressive carve-outs are still being litigated","a clone used in a clearly expressive, non-commercial work may fall outside some statutes")],
  ["unconsented commercial voice clone passed as 'no recording copied'","clone-specific consent conflated with a generic likeness release"],
  ["a recognizable commercial voice clone without explicit voice-clone consent or statutory basis is blocked and escalated"],
  ["a voice-cloning statute is enacted or amended","a clone is cleared on a generic likeness release"],
  specialists=["ip_screening_analyst","voice_rights_specialist"], human_review=True))

N.append(node("FAIL_CLOSED_DEFAULT","fail_closed_default_posture",
  "Set the default posture to fail-closed: when identifiability, consent, jurisdiction, or exception signals are missing, conflicting, or low-confidence, the screen must NOT auto-approve; it defaults to constrain, block, or human-review rather than allow.",
  "synthesis", ["USE_CLASSIFICATION","CONSENT_LICENSE"], ["CQ_13","CQ_14"],
  b(0.95,0.84,0.86,0.55,0.92,0.86,0.75,0.66,0.6, 0.8,0.82, 0.94,0.66,0.16,0.68,[0.18,0.46],"risk tolerance or default-posture policy changes"),
  ["default-deny on uncertainty","no-auto-approve invariant","low-confidence routing to human-review"],
  ["the substantive merits decision (that is the verdict map)","legal advice","operator execution"],
  [pro("A fail-closed default guarantees that an incomplete or ambiguous screen never silently green-lights an identity use","a tuple missing jurisdiction and consent resolves to human-review, not allow")],
  [con("Aggressive fail-closed defaults raise false-positive blocks that frustrate clearly lawful uses and create review load","a plainly newsworthy use may be needlessly routed to review under a strict default")],
  ["uncertainty resolved toward allow","a missing-consent case auto-approved on weak signals"],
  ["with any missing/conflicting/low-confidence signal, the screen never outputs allow; it constrains, blocks, or routes to human-review"],
  ["risk-tolerance policy changes","an auto-approve on incomplete signals is observed"],
  specialists=["ip_screening_analyst","risk_governance_lead"], human_review=True))

N.append(node("VERDICT_MAP","tuple_to_disposition_mapping",
  "Map the assembled signals for an (item, figure, use) tuple to a disposition in {allow, constrain, block, human-review}, where allow is reserved for clearly lawful or fully consented uses, constrain attaches conditions, block stops disallowed identity uses, and human-review routes the genuinely uncertain.",
  "synthesis", ["RIGHT_OF_PUBLICITY","VOICE_PROTECTION","USE_CLASSIFICATION","CONSENT_LICENSE","PARODY_TRANSFORMATIVE_EXCEPTION","FAIL_CLOSED_DEFAULT"], ["CQ_11","CQ_14"],
  b(0.95,0.86,0.88,0.7,0.92,0.9,0.7,0.6,0.65, 0.74,0.78, 0.94,0.6,0.2,0.66,[0.24,0.6],"verdict taxonomy or disposition criteria change"),
  ["allow / constrain / block / human-review mapping","condition attachment for constrain","hard-gate override to block","uncertainty routing to human-review"],
  ["giving legal advice","auto-approving without a consent or clear-lawfulness basis","operator execution of the use"],
  [pro("A four-way disposition with allow reserved for clear/consented cases gives a deterministic, auditable output while preserving the no-auto-approve and hard-block invariants","a fully consented adult endorsement maps to allow; an unconsented voice clone maps to block; a hybrid sponsored use maps to human-review")],
  [con("Compressing a nuanced legal posture into four buckets can over- or under-call borderline uses, so the map must defer rather than guess","a novel transformative-AI use may not fit cleanly and should route to review, not a confident allow")],
  ["a hard-gate case mapped to allow/constrain instead of block","a genuinely uncertain case forced to a confident verdict"],
  ["every tuple maps to exactly one disposition; hard gates force block; uncertainty forces human-review; allow requires a consent or clear-lawfulness basis"],
  ["disposition criteria change","a hard-gate case is mis-mapped to allow"],
  specialists=["ip_screening_analyst","screening_decision_owner"], human_review=True))

N.append(node("ESCALATE_TO_COUNSEL","escalation_to_qualified_counsel",
  "Define the escalation heuristic that routes any block, hard-gate trigger, or human-review disposition to qualified legal counsel for an actual legal determination, making clear the screen is a triage signal and the binding decision belongs to counsel.",
  "synthesis", ["VERDICT_MAP","MINOR_PROTECTION","VOICE_CLONE_CONSENT_GATE"], ["CQ_12","CQ_15"],
  b(0.94,0.84,0.86,0.55,0.9,0.88,0.7,0.64,0.6, 0.8,0.82, 0.92,0.64,0.16,0.66,[0.2,0.5],"escalation routing or counsel-engagement policy changes"),
  ["escalation trigger set","routing to qualified counsel","triage-not-determination framing","escalation record"],
  ["rendering the legal determination itself","auto-approving to avoid escalation","operator execution"],
  [pro("Explicit escalation to counsel keeps the screen as a triage tool and ensures high-risk and blocked tuples reach a qualified human decision-maker","every minor, voice-clone-without-consent, and block disposition is routed to counsel with its rationale")],
  [con("Over-escalation can overwhelm counsel and erode the triage value, so the trigger set must be calibrated to genuine risk","routing every editorial mention to counsel would swamp the queue and defeat triage")],
  ["a high-risk block not escalated to counsel","escalation used as a substitute for, rather than a route to, a legal determination"],
  ["every block, hard-gate trigger, and human-review disposition is routed to qualified counsel with its rationale recorded"],
  ["escalation policy changes","a blocked tuple is closed without counsel review"],
  specialists=["ip_screening_analyst","legal_escalation_coordinator"], human_review=True))

N.append(node("PROVENANCE_RECORD","screening_provenance_and_signals",
  "Record the provenance of each screening decision: the input tuple, every rule/right/jurisdiction signal that fired, the consent artifact relied on (if any), and the resulting disposition, so a decision is reconstructable and reviewable.",
  "verification", ["VERDICT_MAP"], ["CQ_15","CQ_16"],
  b(0.8,0.74,0.76,0.55,0.74,0.78,0.55,0.7,0.45, 0.8,0.82, 0.8,0.7,0.16,0.46,[0.18,0.46],"provenance schema or audit requirements change"),
  ["input-tuple capture","fired-signal log","consent-artifact reference","disposition record"],
  ["the legal determination","operator execution","method-internal heuristics"],
  [pro("A provenance record turns each disposition into an auditable artifact counsel and reviewers can reconstruct","'why was this blocked?' is answered by the logged voice-clone gate trigger and absent consent")],
  [con("Full provenance carries storage and privacy overhead, and logging identity signals itself processes personal data needing care","logging a face/voice signal is itself biometric data handling")],
  ["disposition recorded without the signals that produced it","provenance log itself mishandles the identity data it stores"],
  ["every disposition stores the tuple, fired signals, consent reference, and outcome in a reconstructable record"],
  ["provenance schema changes","a disposition cannot be reconstructed for review"],
  specialists=["ip_screening_analyst","audit_engineer"]))

N.append(node("DECISION_AUDIT","calibration_and_disclaimer_audit",
  "Audit the screen for calibration and for two non-negotiable invariants: it never issues legal advice and never auto-approves; periodically sample dispositions against counsel outcomes to detect drift, over-blocking, or any auto-approve regression, and surface the screening disclaimer on every output.",
  "verification", ["VERDICT_MAP","ESCALATE_TO_COUNSEL","PROVENANCE_RECORD"], ["CQ_14","CQ_16"],
  b(0.86,0.78,0.8,0.6,0.86,0.85,0.6,0.62,0.55, 0.78,0.8, 0.86,0.62,0.2,0.6,[0.22,0.55],"audit policy, disclaimer wording, or calibration target changes"),
  ["no-legal-advice invariant check","no-auto-approve invariant check","disposition-vs-counsel calibration sample","screening disclaimer surfacing"],
  ["the substantive legal call","operator execution","per-tuple rule logic"],
  [pro("A standing audit of the two invariants catches drift and any regression toward auto-approval before it causes harm, and keeps the disclaimer visible","a sampled batch reveals over-blocking of newsworthy uses, prompting a calibration fix without weakening the hard gates")],
  [con("Audit signal lags real outcomes, so a rare auto-approve or advice-like output can slip between sampling windows","a single mis-phrased output reading as legal advice may pass until the next audit")],
  ["an auto-approve regression undetected between audits","disclaimer dropped from some output paths"],
  ["the audit confirms zero auto-approvals, no legal-advice outputs, and a calibrated disposition mix, with the disclaimer present on every output"],
  ["audit policy changes","a missed auto-approve or advice-like output is found"],
  specialists=["ip_screening_analyst","governance_auditor"], human_review=True))

# ----------------------- competency questions (14) -----------------------
CQ = [
 ("CQ_01","How is the screening unit captured as an (item, figure, use) tuple and is the figure an identifiable real person?",["nodes"],"USE_INTAKE normalizes the tuple and FIGURE_IDENTIFICATION resolves identifiability",["USE_INTAKE","FIGURE_IDENTIFICATION"]),
 ("CQ_02","What identity rights attach when a real person is identifiable by name, image, voice, or persona?",["nodes","glossary"],"RIGHT_OF_PUBLICITY and VOICE_PROTECTION define the NIL and voice-identity interests",["RIGHT_OF_PUBLICITY","VOICE_PROTECTION","FIGURE_IDENTIFICATION"]),
 ("CQ_03","How does the screen decide whether a use is a commercial appropriation of identity under the governing law?",["nodes"],"RIGHT_OF_PUBLICITY, USE_CLASSIFICATION, and US_STATE_BY_STATE tie appropriation to a commercial use and a governing state",["RIGHT_OF_PUBLICITY","USE_CLASSIFICATION","US_STATE_BY_STATE"]),
 ("CQ_04","How is a distinctive voice or a particular likeness reproduction mode assessed as an identity claim?",["nodes"],"VOICE_PROTECTION and LIKENESS_REPRODUCTION score voice imitation and the photoreal-to-caricature axis",["VOICE_PROTECTION","LIKENESS_REPRODUCTION"]),
 ("CQ_05","How are use class and expressive exceptions classified, and why are exceptions not automatic?",["nodes","conflict_axes"],"USE_CLASSIFICATION labels the use and PARODY_TRANSFORMATIVE_EXCEPTION treats exceptions as fact-bound defenses",["USE_CLASSIFICATION","LIKENESS_REPRODUCTION","PARODY_TRANSFORMATIVE_EXCEPTION"]),
 ("CQ_06","What consent is required for a voice clone, and what happens absent explicit clone consent?",["nodes"],"CONSENT_LICENSE and VOICE_CLONE_CONSENT_GATE require scope-matched explicit voice-clone consent or block",["CONSENT_LICENSE","VOICE_CLONE_CONSENT_GATE"]),
 ("CQ_07","How is documented consent or another lawful basis verified, including for EU biometric processing?",["nodes"],"CONSENT_LICENSE checks scope-matched consent and EU_BIOMETRIC_BASIS checks an Art. 9 lawful basis",["CONSENT_LICENSE","EU_BIOMETRIC_BASIS"]),
 ("CQ_08","How does the screen resolve which jurisdiction governs given the lack of a uniform federal right?",["nodes"],"US_STATE_BY_STATE, INDIA_PERSONALITY_RIGHTS, and POST_MORTEM_RIGHTS resolve the governing regime",["US_STATE_BY_STATE","INDIA_PERSONALITY_RIGHTS","POST_MORTEM_RIGHTS"]),
 ("CQ_09","How are non-US regimes (India persona rights, EU data protection) factored into the screen?",["nodes"],"INDIA_PERSONALITY_RIGHTS and EU_BIOMETRIC_BASIS add injunction-risk and biometric-basis layers",["INDIA_PERSONALITY_RIGHTS","EU_BIOMETRIC_BASIS"]),
 ("CQ_10","How are post-mortem and descendible publicity rights handled for a deceased figure?",["nodes"],"POST_MORTEM_RIGHTS resolves survival and the statutory term before any clearance",["POST_MORTEM_RIGHTS","US_STATE_BY_STATE"]),
 ("CQ_11","How is a final disposition reached and why must claimed exceptions be argued, not assumed?",["nodes","workflow"],"VERDICT_MAP maps signals to a disposition while PARODY_TRANSFORMATIVE_EXCEPTION keeps exceptions as defenses",["VERDICT_MAP","PARODY_TRANSFORMATIVE_EXCEPTION"]),
 ("CQ_12","How are minors and high-risk cases hard-blocked and escalated to counsel?",["nodes","iteration_protocol"],"MINOR_PROTECTION blocks protected persons and ESCALATE_TO_COUNSEL routes them to a qualified human",["MINOR_PROTECTION","ESCALATE_TO_COUNSEL"]),
 ("CQ_13","How does the screen fail closed and guarantee it never auto-approves an identity use?",["nodes"],"FAIL_CLOSED_DEFAULT, MINOR_PROTECTION, and VOICE_CLONE_CONSENT_GATE enforce no-auto-approve and hard blocks",["FAIL_CLOSED_DEFAULT","MINOR_PROTECTION","VOICE_CLONE_CONSENT_GATE"]),
 ("CQ_14","How is each decision made reconstructable, routed to counsel, audited for the no-legal-advice and no-auto-approve invariants, and the disclaimer surfaced on every output?",["nodes","workflow"],"ESCALATE_TO_COUNSEL routes high-risk dispositions, PROVENANCE_RECORD logs reconstructable provenance, and DECISION_AUDIT enforces the invariants and surfaces the disclaimer",["ESCALATE_TO_COUNSEL","PROVENANCE_RECORD","DECISION_AUDIT"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

# consolidate node->CQ refs (1-2 each); every CQ covered by >=1 node
CQ_MAP = {
 "USE_INTAKE":["CQ_01"], "FIGURE_IDENTIFICATION":["CQ_01","CQ_02"], "RIGHT_OF_PUBLICITY":["CQ_02","CQ_03"],
 "VOICE_PROTECTION":["CQ_02","CQ_04"], "LIKENESS_REPRODUCTION":["CQ_04","CQ_05"], "USE_CLASSIFICATION":["CQ_03","CQ_05"],
 "CONSENT_LICENSE":["CQ_06","CQ_07"], "US_STATE_BY_STATE":["CQ_03","CQ_08"], "INDIA_PERSONALITY_RIGHTS":["CQ_08","CQ_09"],
 "EU_BIOMETRIC_BASIS":["CQ_07","CQ_09"], "POST_MORTEM_RIGHTS":["CQ_08","CQ_10"], "PARODY_TRANSFORMATIVE_EXCEPTION":["CQ_05","CQ_11"],
 "MINOR_PROTECTION":["CQ_12","CQ_13"], "VOICE_CLONE_CONSENT_GATE":["CQ_06","CQ_13"], "FAIL_CLOSED_DEFAULT":["CQ_13"],
 "VERDICT_MAP":["CQ_11"], "ESCALATE_TO_COUNSEL":["CQ_12","CQ_14"], "PROVENANCE_RECORD":["CQ_14"],
 "DECISION_AUDIT":["CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

# ----------------------- glossary -----------------------
GL = [
 ("right_of_publicity","an identifiable person's control over the commercial use of their name, image, and likeness (NIL)",["publicity_right","NIL_right"],["copyright","trademark"],["RIGHT_OF_PUBLICITY","USE_CLASSIFICATION"]),
 ("identifiability","the threshold question of whether a real person is recognizable by name, image, voice, persona, or evocation",["recognizability"],["anonymized_subject"],["FIGURE_IDENTIFICATION","LIKENESS_REPRODUCTION"]),
 ("voice_clone","a synthetic model that reproduces a real person's recognizable voice, distinct from copying an actual recording",["synthetic_voice","sound_alike_model"],["master_recording_copy"],["VOICE_PROTECTION","VOICE_CLONE_CONSENT_GATE"]),
 ("scope_matched_consent","consent that specifically covers the actual use, voice-clone status, term, and territory, not a generic release",["specific_consent"],["generic_release"],["CONSENT_LICENSE","VOICE_CLONE_CONSENT_GATE"]),
 ("transformative_use","a jurisdiction-specific defense asking whether a work adds significant creative expression beyond the likeness itself",["transformative_test"],["automatic_safe_harbor"],["PARODY_TRANSFORMATIVE_EXCEPTION","USE_CLASSIFICATION"]),
 ("post_mortem_right","a publicity right that survives death and is descendible for a statutory term in some jurisdictions",["descendible_publicity"],["right_dies_with_person"],["POST_MORTEM_RIGHTS","US_STATE_BY_STATE"]),
 ("biometric_data","special-category personal data (e.g. face or voice model) whose processing needs an explicit lawful basis under GDPR Art. 9",["special_category_data"],["ordinary_personal_data"],["EU_BIOMETRIC_BASIS","CONSENT_LICENSE"]),
 ("fail_closed","a default posture that resolves missing or conflicting signals toward constrain/block/human-review rather than allow",["default_deny"],["fail_open"],["FAIL_CLOSED_DEFAULT","VERDICT_MAP"]),
 ("disposition","the screen's output for a tuple: allow, constrain, block, or human-review",["screening_verdict"],["legal_determination"],["VERDICT_MAP","ESCALATE_TO_COUNSEL"]),
 ("triage_signal","a non-binding screening output that routes risk to qualified counsel and is never itself legal advice",["screening_signal"],["legal_advice"],["ESCALATE_TO_COUNSEL","DECISION_AUDIT"]),
]
GLS=[{"term":t,"definition":d,"synonyms":s,"not_same_as":ns,"used_by_nodes":u} for (t,d,s,ns,u) in GL]

# ----------------------- edges -----------------------
E=[]
def dep(f,t,rs,cc=0.82,erc=0.28,cp=0.14,why="",ben="",rk="",ex=""):
    E.append({"from":f,"to":t,"edge_type":"dependency","relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{t} depends on {f}","benefit_of_coupling":ben or "ordered prerequisite",
              "risk_of_conflict":rk or "downstream rework if upstream changes","example":ex or f"{f} resolved before {t}"})
def conf(f,t,rs,st,rule,why,cp=0.5,erc=0.5,cc=0.6):
    E.append({"from":f,"to":t,"edge_type":"conflict","relation_strength":rs,"signed_tension":st,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "resolution_rule":rule,"why_related":why,"benefit_of_coupling":"tension surfaced and resolved by rule",
              "risk_of_conflict":"unmanaged tension degrades screening quality","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated screening behavior",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies) -- 23 edges
dep("USE_INTAKE","FIGURE_IDENTIFICATION",0.9)
dep("FIGURE_IDENTIFICATION","RIGHT_OF_PUBLICITY",0.86)
dep("FIGURE_IDENTIFICATION","VOICE_PROTECTION",0.84)
dep("FIGURE_IDENTIFICATION","LIKENESS_REPRODUCTION",0.82)
dep("RIGHT_OF_PUBLICITY","USE_CLASSIFICATION",0.84)
dep("RIGHT_OF_PUBLICITY","CONSENT_LICENSE",0.86)
dep("VOICE_PROTECTION","CONSENT_LICENSE",0.82)
dep("RIGHT_OF_PUBLICITY","US_STATE_BY_STATE",0.82)
dep("RIGHT_OF_PUBLICITY","INDIA_PERSONALITY_RIGHTS",0.78)
dep("CONSENT_LICENSE","EU_BIOMETRIC_BASIS",0.8)
dep("US_STATE_BY_STATE","POST_MORTEM_RIGHTS",0.8)
dep("USE_CLASSIFICATION","PARODY_TRANSFORMATIVE_EXCEPTION",0.78)
dep("FIGURE_IDENTIFICATION","MINOR_PROTECTION",0.8)
dep("VOICE_PROTECTION","VOICE_CLONE_CONSENT_GATE",0.84)
dep("CONSENT_LICENSE","VOICE_CLONE_CONSENT_GATE",0.82)
dep("CONSENT_LICENSE","FAIL_CLOSED_DEFAULT",0.8)
dep("CONSENT_LICENSE","VERDICT_MAP",0.84)
dep("FAIL_CLOSED_DEFAULT","VERDICT_MAP",0.84)
dep("VERDICT_MAP","ESCALATE_TO_COUNSEL",0.86)
dep("VOICE_CLONE_CONSENT_GATE","ESCALATE_TO_COUNSEL",0.8)
dep("VERDICT_MAP","PROVENANCE_RECORD",0.8)
dep("ESCALATE_TO_COUNSEL","DECISION_AUDIT",0.78)
dep("PROVENANCE_RECORD","DECISION_AUDIT",0.78)

# cross-cutting non-dependency edges (no cycle risk)
rel("INDIA_PERSONALITY_RIGHTS","VERDICT_MAP","constraint",0.76,why="India-facing injunction risk constrains the admissible disposition for a recognizable persona")
rel("EU_BIOMETRIC_BASIS","VERDICT_MAP","constraint",0.76,why="an absent Art. 9 biometric basis constrains an EU-facing use toward constrain/block")
rel("POST_MORTEM_RIGHTS","VERDICT_MAP","constraint",0.74,why="a surviving post-mortem right blocks auto-clearing a deceased figure in the verdict map")
rel("MINOR_PROTECTION","FAIL_CLOSED_DEFAULT","causal",0.82,cc=0.85,why="a minor signal forces the fail-closed posture to a hard block rather than a constrain")
rel("FIGURE_IDENTIFICATION","FAIL_CLOSED_DEFAULT","causal",0.74,why="unresolved identifiability is a missing signal that triggers the fail-closed default")
rel("ESCALATE_TO_COUNSEL","PROVENANCE_RECORD","sequence",0.74,why="escalation attaches its rationale to the provenance record handed to counsel")
rel("DECISION_AUDIT","VERDICT_MAP","feedback",0.78,why="audit findings of over-blocking or drift feed back to recalibrate the verdict map")
rel("DECISION_AUDIT","FAIL_CLOSED_DEFAULT","feedback",0.76,why="audit detection of an auto-approve regression feeds back to harden the fail-closed default")

# conflict edges (negative signed_tension + resolution_rule) -- 4
conf("PARODY_TRANSFORMATIVE_EXCEPTION","RIGHT_OF_PUBLICITY",0.74,-0.6,
  "the expressive defense never auto-clears: apply the governing transformative-use/newsworthiness test as a fact-specific defense; if it is not clearly met, the publicity right controls and the use is constrained, blocked, or escalated",
  "the parody/transformative defense pulls toward allow while the right of publicity pulls toward block")
conf("USE_CLASSIFICATION","FAIL_CLOSED_DEFAULT",0.7,-0.5,
  "a clean 'editorial' label does not override fail-closed: when use class is hybrid or low-confidence, the fail-closed default routes to human-review rather than honoring the editorial label as allow",
  "an editorial/newsworthy classification pulls toward allow while fail-closed pulls hybrid uses toward review")
conf("VOICE_CLONE_CONSENT_GATE","VERDICT_MAP",0.72,-0.6,
  "the voice-clone hard gate dominates the verdict map: an unconsented commercial voice clone is forced to block regardless of other allow-leaning signals, and escalated to counsel",
  "the verdict map may lean allow on other signals while the voice-clone gate forces block")
conf("INDIA_PERSONALITY_RIGHTS","PARODY_TRANSFORMATIVE_EXCEPTION",0.66,-0.45,
  "do not import a foreign expressive exception into an India-facing use: assess the exception only under the governing forum's law, and where India's persona-injunction risk applies, the exception does not auto-clear it",
  "a US-style transformative defense pulls toward allow while India's persona-rights regime pulls an India-facing use toward block")

# ----------------------- conflict axes (9) -----------------------
CA=[
 {"name":"identity_protection_vs_free_expression","description":"Protecting an identifiable persona guards against appropriation but can over-restrict legitimate expression, commentary, and parody.","poles":["identity_protection","free_expression"],"resolution_hint":"weight a commercial use toward protection and an expressive/newsworthy use toward expression, but argue, do not assume, the exception","tension_score":0.78,"affected_nodes":["RIGHT_OF_PUBLICITY","PARODY_TRANSFORMATIVE_EXCEPTION","USE_CLASSIFICATION"]},
 {"name":"voice_as_identity_vs_style_imitation","description":"A distinctive voice is protectable identity, but unprotectable genre or style imitation lies close by; over-protecting chills lawful homage, under-protecting permits sound-alike misappropriation.","poles":["protected_voice_identity","unprotected_style"],"resolution_hint":"flag deliberate recognizable-voice imitation/clone; treat generic genre style as style, and escalate the borderline","tension_score":0.72,"affected_nodes":["VOICE_PROTECTION","VOICE_CLONE_CONSENT_GATE","LIKENESS_REPRODUCTION"]},
 {"name":"commercial_use_vs_editorial_newsworthy","description":"Commercial appropriation is the core wrong, but an editorial/newsworthy label can both legitimately protect reporting and be misused to launder an advertisement.","poles":["commercial_appropriation","editorial_newsworthy"],"resolution_hint":"classify by dominant purpose; route hybrid/sponsored uses to human-review","tension_score":0.7,"affected_nodes":["USE_CLASSIFICATION","FAIL_CLOSED_DEFAULT","RIGHT_OF_PUBLICITY"]},
 {"name":"consent_presence_vs_consent_scope","description":"A signed release may exist yet not cover the actual use, term, territory, or voice-clone status; presence is not scope.","poles":["consent_present","consent_scope_matched"],"resolution_hint":"validate scope, term, territory, and clone-specificity, not mere presence of a release","tension_score":0.7,"affected_nodes":["CONSENT_LICENSE","VOICE_CLONE_CONSENT_GATE","VERDICT_MAP"]},
 {"name":"jurisdictional_divergence_vs_single_rule","description":"No uniform federal right and divergent state, Indian, and EU regimes mean one rule misapplies; resolving the governing forum is unavoidable but hard for multi-jurisdiction publication.","poles":["forum_specific_rule","single_global_rule"],"resolution_hint":"resolve the governing forum per audience; apply the strictest applicable regime when ambiguous","tension_score":0.72,"affected_nodes":["US_STATE_BY_STATE","INDIA_PERSONALITY_RIGHTS","EU_BIOMETRIC_BASIS"]},
 {"name":"post_mortem_protection_vs_public_domain_persona","description":"A deceased figure's persona may still be a descendible asset for decades or may have entered free use, depending entirely on jurisdiction and term.","poles":["descendible_protection","free_use_after_death"],"resolution_hint":"resolve descendibility and term in the governing jurisdiction before any clearance","tension_score":0.66,"affected_nodes":["POST_MORTEM_RIGHTS","US_STATE_BY_STATE","RIGHT_OF_PUBLICITY"]},
 {"name":"fail_closed_safety_vs_false_positive_blocks","description":"A strict fail-closed default prevents silent green-lighting but raises false-positive blocks and review load for clearly lawful uses.","poles":["fail_closed_safety","throughput_and_low_friction"],"resolution_hint":"keep no-auto-approve absolute; calibrate the constrain/review threshold via the audit loop","tension_score":0.68,"affected_nodes":["FAIL_CLOSED_DEFAULT","VERDICT_MAP","DECISION_AUDIT"]},
 {"name":"exception_availability_vs_indeterminacy","description":"Expressive exceptions can defeat a publicity claim but are forum-dependent and indeterminate, so relying on them risks false clearances.","poles":["exception_relied_on","exception_deferred_to_counsel"],"resolution_hint":"treat any exception as a defense to be argued; escalate indeterminate ones rather than auto-clearing","tension_score":0.66,"affected_nodes":["PARODY_TRANSFORMATIVE_EXCEPTION","ESCALATE_TO_COUNSEL","USE_CLASSIFICATION"]},
 {"name":"automation_triage_vs_legal_determination","description":"The screen adds value as fast triage but must never substitute for a qualified legal determination or issue legal advice.","poles":["automated_triage","human_legal_determination"],"resolution_hint":"emit a non-binding disposition plus disclaimer; route every block/hard-gate/review to counsel","tension_score":0.74,"affected_nodes":["VERDICT_MAP","ESCALATE_TO_COUNSEL","DECISION_AUDIT"]},
]

# ----------------------- edge cases (12) -----------------------
EC=[
 {"description":"An AI voice clone of a named singer is used in an ad with no actual recording copied, asserted as 'no copyright issue'.","trigger":"synthetic voice clone for commercial use without explicit voice-clone consent","affected_nodes":["VOICE_PROTECTION","VOICE_CLONE_CONSENT_GATE","CONSENT_LICENSE"],"mitigation":"flag voice-identity misappropriation (Midler/Waits) and block under the voice-clone gate absent explicit consent","severity":"critical"},
 {"description":"A photoreal deepfake of a living celebrity endorses a product under a generic stock-model release.","trigger":"out-of-scope release stretched to cover a synthetic deepfake endorsement","affected_nodes":["LIKENESS_REPRODUCTION","CONSENT_LICENSE","RIGHT_OF_PUBLICITY"],"mitigation":"reject the generic release as not scope-matched; block and escalate","severity":"critical"},
 {"description":"A robot or look-alike posed as a celebrity evokes them without name, image, or voice.","trigger":"evocation of a distinctive persona with no literal name/face/voice","affected_nodes":["FIGURE_IDENTIFICATION","RIGHT_OF_PUBLICITY"],"mitigation":"apply the identifiability-by-evocation standard (White v Samsung, Wendt) rather than dismissing for lack of a literal name","severity":"high"},
 {"description":"An identifiable minor is depicted in an AI-generated likeness or voice clone for a commercial use.","trigger":"minor or protected-person subject reaches the screen","affected_nodes":["MINOR_PROTECTION","FIGURE_IDENTIFICATION","ESCALATE_TO_COUNSEL"],"mitigation":"hard-block and escalate outright regardless of any release; never constrain or auto-approve","severity":"critical"},
 {"description":"A deceased celebrity's persona is used commercially, assumed free because the person has died.","trigger":"deceased figure auto-treated as public domain","affected_nodes":["POST_MORTEM_RIGHTS","US_STATE_BY_STATE","RIGHT_OF_PUBLICITY"],"mitigation":"resolve descendibility and the post-mortem term (e.g. California, Tennessee) before any clearance","severity":"high"},
 {"description":"A use of an Indian celebrity's AI-generated persona is screened only under a permissive foreign standard.","trigger":"India-facing recognizable-persona use screened under non-Indian law","affected_nodes":["INDIA_PERSONALITY_RIGHTS","VERDICT_MAP"],"mitigation":"assess India personality-rights injunction risk (Anil Kapoor v Simply Life) for India-facing uses","severity":"high"},
 {"description":"An EU-facing voice clone is trained on a resident's recordings with no Art. 9 lawful basis.","trigger":"biometric (voice model) processing of an EU data subject without an explicit basis","affected_nodes":["EU_BIOMETRIC_BASIS","CONSENT_LICENSE","VOICE_CLONE_CONSENT_GATE"],"mitigation":"require an explicit GDPR Art. 9 basis; block/constrain the EU-facing use absent one","severity":"high"},
 {"description":"A sponsored 'editorial' video is both news-styled and an advertisement and is routed as protected news.","trigger":"hybrid commercial/editorial use mis-classified as pure editorial","affected_nodes":["USE_CLASSIFICATION","FAIL_CLOSED_DEFAULT","VERDICT_MAP"],"mitigation":"classify by dominant purpose; route hybrid sponsored uses to human-review, not allow","severity":"medium"},
 {"description":"A caricature is labeled 'parody' and treated as automatically cleared regardless of commercial sale.","trigger":"parody label asserted as an automatic safe harbor","affected_nodes":["PARODY_TRANSFORMATIVE_EXCEPTION","USE_CLASSIFICATION","RIGHT_OF_PUBLICITY"],"mitigation":"apply the governing transformative-use test as a fact-specific defense; do not auto-clear on the label","severity":"medium"},
 {"description":"A tuple arrives with no asserted jurisdiction and an ambiguous figure, yet a disposition is demanded.","trigger":"missing jurisdiction and weak identifiability signals","affected_nodes":["USE_INTAKE","FIGURE_IDENTIFICATION","FAIL_CLOSED_DEFAULT"],"mitigation":"fail closed: route to human-review rather than allow on incomplete signals","severity":"high"},
 {"description":"The screen produces a confident 'allow' on a borderline use, reading like a legal clearance.","trigger":"a borderline use auto-approved and phrased as legal advice","affected_nodes":["VERDICT_MAP","DECISION_AUDIT","ESCALATE_TO_COUNSEL"],"mitigation":"enforce no-auto-approve and no-legal-advice invariants; surface the disclaimer and route to counsel","severity":"critical"},
 {"description":"Over time the screen drifts toward over-blocking clearly newsworthy uses, eroding trust in triage.","trigger":"calibration drift detected against counsel outcomes","affected_nodes":["DECISION_AUDIT","VERDICT_MAP","FAIL_CLOSED_DEFAULT"],"mitigation":"use the audit loop to recalibrate the constrain/review threshold without weakening hard gates","severity":"medium"},
]

# ----------------------- workflow (12) -----------------------
WF=[
 {"action":"capture_use_tuple","node_ref":"USE_INTAKE","description":"Normalize the screening unit into an (item, figure, use) tuple with asserted jurisdiction(s).","artifact":"normalized_tuple","gate":"tuple has a figure, a concrete use, and at least one jurisdiction"},
 {"action":"resolve_identifiability","node_ref":"FIGURE_IDENTIFICATION","description":"Determine whether a real, identifiable person is depicted or evoked and record the identifiability theory.","artifact":"identifiability_finding","gate":"an identifiability theory (name/image/voice/persona) is recorded or the figure is dropped"},
 {"action":"apply_publicity_right","node_ref":"RIGHT_OF_PUBLICITY","description":"State the commercial-appropriation theory tying the identifiable person to the commercial use.","artifact":"appropriation_assessment","gate":"a commercial-appropriation theory is stated or explicitly absent"},
 {"action":"assess_voice_and_likeness","node_ref":"VOICE_PROTECTION","description":"Assess distinctive-voice imitation/clone and the likeness reproduction mode.","artifact":"voice_and_likeness_assessment","gate":"any deliberate voice clone or photoreal likeness is flagged"},
 {"action":"classify_use","node_ref":"USE_CLASSIFICATION","description":"Classify the use as commercial, editorial/newsworthy, parody, or hybrid by dominant purpose.","artifact":"use_class","gate":"a use class with dominant-purpose rationale is recorded"},
 {"action":"verify_consent_basis","node_ref":"CONSENT_LICENSE","description":"Verify scope-matched consent or another lawful basis, including explicit voice-clone authorization.","artifact":"consent_record","gate":"consent/basis matches actual use/term/territory or the use is constrained/blocked"},
 {"action":"resolve_jurisdiction","node_ref":"US_STATE_BY_STATE","description":"Resolve the governing US state and any India/EU/post-mortem overlay for the audience.","artifact":"jurisdiction_finding","gate":"the governing regime(s) and their scope are recorded"},
 {"action":"evaluate_exceptions","node_ref":"PARODY_TRANSFORMATIVE_EXCEPTION","description":"Evaluate any claimed expressive/newsworthy exception as a fact-bound defense under the governing test.","artifact":"exception_analysis","gate":"any exception is recorded as a defense with its test, never as automatic clearance"},
 {"action":"apply_hard_gates","node_ref":"MINOR_PROTECTION","description":"Apply the minor/protected-person and unconsented voice-clone hard blocks.","artifact":"hard_gate_result","gate":"any minor or unconsented commercial voice clone is blocked and escalated"},
 {"action":"apply_fail_closed_default","node_ref":"FAIL_CLOSED_DEFAULT","description":"On any missing/conflicting/low-confidence signal, default to constrain/block/human-review, never allow.","artifact":"default_posture","gate":"no allow is emitted on incomplete or conflicting signals"},
 {"action":"map_disposition","node_ref":"VERDICT_MAP","description":"Map the assembled signals to exactly one of allow/constrain/block/human-review with hard gates overriding.","artifact":"disposition","gate":"exactly one disposition; allow requires a consent or clear-lawfulness basis"},
 {"action":"escalate_and_audit","node_ref":"ESCALATE_TO_COUNSEL","description":"Route every block/hard-gate/human-review disposition to qualified counsel; log provenance and audit the invariants.","artifact":"escalation_and_audit_record","gate":"high-risk dispositions reach counsel; disclaimer surfaced; no auto-approve or legal-advice output"},
]

# ----------------------- dominance rules (9) -----------------------
DR=[
 {"rule":"MINOR_PROTECTION blocks and escalates any minor or protected-person depiction regardless of consent, exception, or commercial value","rationale":"child protection and heightened consent concerns outweigh any commercial or expressive interest","trigger":"a minor or protected-person subject is detected","action":"hard-block and escalate; never constrain or allow"},
 {"rule":"VOICE_CLONE_CONSENT_GATE blocks any unconsented recognizable commercial voice clone regardless of allow-leaning signals","rationale":"voice-clone statutes (ELVIS Act) treat unconsented commercial cloning as a direct violation","trigger":"a commercial voice clone lacks explicit voice-clone consent or statutory basis","action":"force block and escalate to counsel"},
 {"rule":"FAIL_CLOSED_DEFAULT prohibits any allow output when identifiability, consent, jurisdiction, or exception signals are missing or conflicting","rationale":"the screen must never silently green-light an identity use on incomplete information","trigger":"a required signal is missing, conflicting, or low-confidence","action":"route to constrain/block/human-review, never allow"},
 {"rule":"The legal gate dominates: no exception, business value, or throughput pressure may convert a block or hard-gate trigger into an allow","rationale":"the screen is a fail-closed legal gate; identity-protection and statutory blocks take precedence over convenience","trigger":"an allow is proposed over an active block or hard-gate trigger","action":"reject the allow and uphold the block"},
 {"rule":"PARODY_TRANSFORMATIVE_EXCEPTION may never auto-clear; an unmet or indeterminate exception leaves the publicity right controlling","rationale":"expressive exceptions are fact-bound, forum-dependent defenses, not safe harbors","trigger":"a parody/transformative label is asserted as automatic clearance","action":"require the governing test be met or escalate; otherwise the right controls"},
 {"rule":"CONSENT_LICENSE requires scope-matched consent; a generic or out-of-scope release does not authorize the use","rationale":"presence of a release is not coverage of the actual use, term, territory, or clone status","trigger":"a release is offered that does not match the actual use","action":"treat as unconsented; constrain or block"},
 {"rule":"For multi-jurisdiction publication, apply the strictest applicable regime when the governing forum is ambiguous","rationale":"divergent regimes mean a permissive rule must not be applied to a use governed by a stricter one","trigger":"the governing forum cannot be uniquely resolved","action":"apply the strictest applicable regime or escalate"},
 {"rule":"POST_MORTEM_RIGHTS must be resolved before any deceased figure is cleared","rationale":"descendible publicity can protect a persona for decades after death in some jurisdictions","trigger":"a deceased figure is proposed for clearance","action":"resolve descendibility and term first; do not auto-clear"},
 {"rule":"VERDICT_MAP must emit exactly one disposition and reserve allow for clearly lawful or fully consented uses","rationale":"a deterministic, auditable output preserves the no-auto-approve and hard-block invariants","trigger":"a tuple is dispositioned","action":"emit one of allow/constrain/block/human-review with allow gated on consent or clear lawfulness"},
]

# ----------------------- anti-rework rules (8) -----------------------
ARR=[
 {"rule":"Do not screen a tuple before the figure's identifiability is resolved; an unidentified figure invalidates every downstream right test","prevents":"re-running rights, jurisdiction, and consent analysis after the figure is finally identified"},
 {"rule":"Do not accept a generic release as consent; validate scope/term/territory/clone-status up front","prevents":"clearing a use on a release that is later found out-of-scope, forcing a recall"},
 {"rule":"Do not apply a single jurisdiction's rule before resolving the governing forum for the actual audience","prevents":"re-screening under the correct regime after applying a permissive state's rule in error"},
 {"rule":"Do not auto-clear a deceased figure; resolve post-mortem descendibility before clearance","prevents":"recalling a 'cleared' use of a still-protected estate persona"},
 {"rule":"Do not treat a parody/transformative or newsworthiness label as automatic clearance; record it as a defense to be argued","prevents":"reversing a false clearance after the exception fails under the governing test"},
 {"rule":"Do not emit an allow on incomplete or conflicting signals; fail closed to human-review first","prevents":"retracting an auto-approval that should have been a review or block"},
 {"rule":"Do not let any disposition skip the hard gates; minors and unconsented voice clones must block before the verdict map","prevents":"rework when a hard-gate case is discovered to have been allowed or constrained"},
 {"rule":"Do not close a disposition without provenance and disclaimer; missing records force re-screening for counsel and audit","prevents":"reconstructing a decision from scratch when counsel or the audit cannot trace it"},
]

# ----------------------- iteration protocol (8) -----------------------
IP=[
 {"trigger":"an unconsented voice clone is found to have passed as 'no recording copied'","action":"tighten the voice-clone gate in VOICE_CLONE_CONSENT_GATE and re-evaluate VOICE_PROTECTION flagging","nodes":["VOICE_CLONE_CONSENT_GATE","VOICE_PROTECTION"],"priority":"critical"},
 {"trigger":"a minor or protected-person depiction is processed without a hard block","action":"audit the MINOR_PROTECTION trigger and the FAIL_CLOSED_DEFAULT routing for that path","nodes":["MINOR_PROTECTION","FAIL_CLOSED_DEFAULT"],"priority":"critical"},
 {"trigger":"an auto-approve or legal-advice-like output is detected in the audit","action":"harden the no-auto-approve invariant in FAIL_CLOSED_DEFAULT and VERDICT_MAP and re-surface the disclaimer in DECISION_AUDIT","nodes":["FAIL_CLOSED_DEFAULT","VERDICT_MAP","DECISION_AUDIT"],"priority":"critical"},
 {"trigger":"a use governed by a stricter forum was screened under a permissive one","action":"refine forum resolution in US_STATE_BY_STATE and the overlays in INDIA_PERSONALITY_RIGHTS and EU_BIOMETRIC_BASIS","nodes":["US_STATE_BY_STATE","INDIA_PERSONALITY_RIGHTS","EU_BIOMETRIC_BASIS"],"priority":"high"},
 {"trigger":"a deceased figure was auto-cleared as public domain","action":"re-run POST_MORTEM_RIGHTS descendibility and add the term check before clearance in VERDICT_MAP","nodes":["POST_MORTEM_RIGHTS","VERDICT_MAP"],"priority":"high"},
 {"trigger":"a parody/transformative label was treated as automatic clearance","action":"reassert the no-auto-clear rule in PARODY_TRANSFORMATIVE_EXCEPTION and route the borderline to ESCALATE_TO_COUNSEL","nodes":["PARODY_TRANSFORMATIVE_EXCEPTION","ESCALATE_TO_COUNSEL"],"priority":"high"},
 {"trigger":"a generic release was accepted as scope-matched consent","action":"tighten scope/term/territory validation in CONSENT_LICENSE and the clone-specific check in VOICE_CLONE_CONSENT_GATE","nodes":["CONSENT_LICENSE","VOICE_CLONE_CONSENT_GATE"],"priority":"high"},
 {"trigger":"the audit shows calibration drift toward over-blocking newsworthy uses","action":"recalibrate the constrain/review threshold in VERDICT_MAP via DECISION_AUDIT without weakening the hard gates","nodes":["VERDICT_MAP","DECISION_AUDIT"],"priority":"medium"},
]

spec = {
 "domain":"pubrights__likeness_voice",
 "domain_label":"Right of Publicity, Likeness & Voice Consent Screening",
 "purpose":"screen_an_item_figure_use_tuple_for_right_of_publicity_likeness_and_voice_clone_consent_yielding_allow_constrain_block_or_human_review_and_escalating_to_counsel",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "the screen is a fail-closed triage heuristic, not legal advice, and never auto-approves an identity use",
   "the binding legal determination belongs to qualified counsel; the screen routes risk to them",
   "an (item, figure, use) tuple and at least one asserted jurisdiction are available as input",
   "the legal gate dominates: identity-protection and statutory hard blocks take precedence over business value or throughput",
 ],
 "exclusions":[
   "rendering an actual legal determination or legal advice (delegated to qualified counsel)",
   "master-recording and musical-composition copyright clearance (separate rights)",
   "trademark and false-endorsement (Lanham Act) analysis beyond the publicity overlap",
   "operator execution of the cleared use and downstream content generation",
 ],
 "source_description":"heuristic prior estimates for right-of-publicity, likeness, and voice-clone consent screening work units, grounded in leading US, Indian, and EU identity-rights authorities; no supplied dataset",
 "source_citation":"Haelan Laboratories v Topps Chewing Gum (2d Cir. 1953); Midler v Ford Motor Co. (9th Cir. 1988); Waits v Frito-Lay (9th Cir. 1992); White v Samsung Electronics (9th Cir. 1992); Wendt v Host International (9th Cir. 1997); Cal. Civ. Code 3344 and N.Y. Civil Rights Law 50/51; Tennessee ELVIS Act 2024 (Ensuring Likeness Voice and Image Security Act); Anil Kapoor v Simply Life India (Delhi High Court 2023); DM Entertainment v Baby Gift House (Delhi High Court); EU GDPR Arts. 4(14) and 9 (biometric/special-category data)",
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
 "priority_rationale":"intake and identifiability are foundational; the right-of-publicity, voice, likeness, classification, and consent units build the substantive screen; jurisdiction and exception overlays refine it; the minor and voice-clone hard gates plus the fail-closed default dominate; the verdict map, counsel escalation, provenance, and audit close the fail-closed legal gate last.",
 "eval_objective":"verify_identifiability_resolution_voice_and_likeness_protection_consent_scope_jurisdictional_divergence_hard_gates_fail_closed_no_auto_approve_and_counsel_escalation_of_pubrights__likeness_voice_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "pubrights__likeness_voice.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
