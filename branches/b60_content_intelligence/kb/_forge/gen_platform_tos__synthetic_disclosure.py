#!/usr/bin/env python3
"""Generate the platform_tos__synthetic_disclosure content spec (B60 content-intelligence KB)
for kb_forge.py. Compact authoring: node() applies sane defaults so only domain
content + base metric magnitudes are specified per node.

Domain: a FAIL-CLOSED disclosure gate. It converts an emit-action (an artifact about to
be published into a downstream session/platform) into the REQUIRED in-band synthetic-media
disclosure tokens and platform-ToS clearances, then BLOCKS the emit at the downstream
handoff SPOF when no disclosure slot exists or the token cannot survive transit. Disclosure
is mandatory (EU AI Act Art 50) and travels in-band (C2PA Content Credentials); the
fail-closed handoff is a single point of failure. It is never legal advice and never strips
or downgrades a disclosure to make an emit go through."""
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
        "academic_fields": ["ai_governance", "platform_policy_and_media_law"],
        "subfields": subfields or ["synthetic_media_transparency", "content_provenance"],
        "specialists": specialists or ["disclosure_gate_analyst"],
        "contradictors": contradictors or ["frictionless_publishing_advocate"],
        "inputs": inputs or ["emit-action artifact", "source/platform context"],
        "outputs": outputs or ["disclosure obligation signal"],
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

# ---------- foundations layer ----------
N.append(node("EMIT_INTAKE","emit_action_capture_and_normalization",
  "Capture and normalize the emit-action as a fixed unit: the concrete artifact about to be published downstream, its generation path (fully synthetic, AI-edited, or captured-real), the destination platform/session, and the asserted publication jurisdiction(s), so every later disclosure and ToS test resolves against one frozen unit of analysis.",
  "foundations", [], ["CQ_01"],
  b(0.9,0.78,0.8,0.5,0.82,0.78,0.5,0.7,0.4, 0.78,0.82, 0.88,0.7,0.18,0.5,[0.2,0.5],"intake schema or emit-action definition changes"),
  ["artifact + generation-path description","destination platform/session","asserted jurisdiction(s)","reach/age hints"],
  ["legal advice","final allow/block decision","operator execution of the emit"],
  [pro("A normalized emit unit makes generation-path, platform, and jurisdiction tests resolve against one fixed artifact rather than a moving target","'a fully AI-generated portrait video, posted to TikTok, viewers in the EU' is frozen before any disclosure rule fires")],
  [con("A thin intake (unknown generation path, vague destination) silently weakens every downstream disclosure test","an artifact tagged 'maybe edited' with no platform cannot be screened, only escalated")],
  ["generation path left unknown yet treated as captured-real","destination platform omitted so the wrong ToS set applies","jurisdiction omitted so the wrong labeling regime is applied"],
  ["emit unit records a generation path, a destination platform/session, and at least one asserted jurisdiction"],
  ["intake schema revised","a new generation path (e.g. real-time avatar) appears"],
  specialists=["disclosure_gate_analyst","intake_reviewer"]))

N.append(node("PLATFORM_TOS","platform_terms_of_service_constraints",
  "Resolve the destination (and any source) platform's Terms of Service constraints that bind this emit: scraping limits on how source media was obtained, redistribution and republication permissions, API/automation rules, and the platform's own synthetic/manipulated-media policy hooks, so platform-level prohibitions are known before a disclosure plan is built.",
  "foundations", ["EMIT_INTAKE"], ["CQ_01","CQ_02"],
  b(0.88,0.82,0.74,0.6,0.84,0.8,0.55,0.66,0.55, 0.74,0.78, 0.86,0.66,0.2,0.55,[0.22,0.55],"a destination/source platform updates its ToS or developer policy"),
  ["scraping/collection limits","redistribution/republication permission","API and automation rules","platform synthetic-media policy hooks"],
  ["statutory labeling law","the in-band token mechanism","operator execution"],
  [pro("Reading ToS first means a redistribution prohibition or scraping ban can block an emit before disclosure work is wasted on it","TikTok and YouTube ToS forbid redistributing other users' content without permission, independent of any AI label")],
  [con("ToS are unilateral, frequently revised, and per-platform, so a cached snapshot can silently diverge from the live policy","a platform tightens its automation policy overnight and a previously allowed bulk emit becomes a violation")],
  ["stale ToS snapshot applied to a live emit","source-platform scraping limits ignored because only the destination was checked","API rate/automation rule treated as advisory"],
  ["the destination platform's redistribution, scraping, automation, and synthetic-media policy clauses relevant to this emit are identified and dated"],
  ["a platform ToS or developer-policy version changes","a redistribution-permission call is disputed"],
  specialists=["disclosure_gate_analyst","platform_policy_specialist"]))

N.append(node("SYNTHETIC_MEDIA_CLASSIFY","synthetic_or_manipulated_media_classification",
  "Classify the artifact's generation path on the spectrum that triggers disclosure: fully AI-generated, AI-substantially-edited/altered, AI-assisted-but-authentic, or captured-real-unaltered; this classification is the switch that decides whether any synthetic-media disclosure obligation exists at all.",
  "classification", ["EMIT_INTAKE"], ["CQ_02","CQ_03"],
  b(0.92,0.8,0.8,0.66,0.86,0.85,0.55,0.62,0.58, 0.72,0.78, 0.9,0.62,0.22,0.55,[0.24,0.6],"the AI-content definition or manipulation threshold shifts"),
  ["fully-synthetic detection","substantial-edit / alteration threshold","AI-assisted-but-authentic carve-out","captured-real classification"],
  ["the legal labeling requirement itself","provenance signing","operator execution"],
  [pro("An explicit four-way classification makes 'does disclosure apply?' a recorded decision rather than an implicit assumption, matching the EU AI Act's 'artificially generated or manipulated' trigger","an AI-generated voiceover is classed fully-synthetic; a colour-graded real clip is captured-real and triggers no AI label")],
  [con("The synthetic/authentic boundary is genuinely fuzzy for AI-assisted edits, and an over-broad threshold floods authentic media with needless labels","mild generative denoising of a real photo may or may not cross the 'substantially manipulated' line")],
  ["substantial AI manipulation mis-classified as authentic, suppressing a required label","trivial AI assistance over-classified as synthetic, mislabeling authentic media","deepfake of a real person not recognized as manipulated media"],
  ["a generation-path class (fully-synthetic, substantially-edited, AI-assisted-authentic, captured-real) is recorded with the evidence that placed it there"],
  ["the AI-content / 'substantially manipulated' definition changes","a borderline AI-edit is mis-classified in review"],
  specialists=["disclosure_gate_analyst","synthetic_media_classifier"]))

N.append(node("SCRAPE_LICENSE","scrape_provenance_and_redistribution_license",
  "Trace how the source media was obtained and whether it carries a redistribution license: SCRAPE-sourced or otherwise unlicensed third-party media must not be republished, so the emit is gated on having either an original/owned source or an explicit redistribution license, independent of any AI label.",
  "classification", ["PLATFORM_TOS"], ["CQ_02","CQ_04"],
  b(0.86,0.8,0.7,0.6,0.85,0.78,0.6,0.64,0.55, 0.74,0.78, 0.86,0.64,0.22,0.58,[0.24,0.6],"the source-provenance signal or licensing model changes"),
  ["source acquisition path (owned / licensed / scraped)","redistribution-license presence","third-party rights in the underlying media"],
  ["statutory AI labeling","the in-band token format","operator execution"],
  [pro("Gating on redistribution license catches unlicensed-republish violations that a synthetic-media label would never surface, since a perfectly labeled AI edit of scraped footage is still an unlicensed republish","an AI restyle of a scraped news clip is blocked on the scrape-license gate even though its AI label is correct")],
  [con("Provenance of source media is often unverifiable at emit time, so a conservative gate blocks legitimate fair-use or owned material lacking a paper trail","a creator's own footage with no stored license record is treated as unlicensed")],
  ["scraped/unlicensed source republished under a correct AI label","owned material blocked for lack of a license record","license assumed from platform availability"],
  ["the source is recorded as owned, explicitly licensed for redistribution, or flagged as scraped/unlicensed-and-non-republishable"],
  ["the provenance/licensing model changes","a scraped-source republish is detected post-emit"],
  specialists=["disclosure_gate_analyst","rights_provenance_specialist"]))

N.append(node("AUDIENCE_CONTEXT","audience_reach_and_age_context",
  "Establish the audience context that escalates obligations: estimated reach, whether the content is directed to or likely to reach minors, and the topical sensitivity (political, health, financial), since age and reach determine which heightened platform and statutory rules attach to the same artifact.",
  "classification", ["EMIT_INTAKE"], ["CQ_03","CQ_05"],
  b(0.8,0.72,0.74,0.55,0.8,0.78,0.5,0.66,0.5, 0.76,0.78, 0.8,0.66,0.2,0.5,[0.22,0.54],"audience-context signals or the minor-directed standard change"),
  ["reach estimate","minor-directed / minor-likely audience signal","topical sensitivity (political/health/financial)"],
  ["the specific minor-policy rules","election-period rules","operator execution"],
  [pro("Fixing audience context lets the gate apply minor and election escalations to the right emits instead of all of them","an AI clip directed to a kids' channel pulls in stricter platform minor rules than the same clip on a general feed")],
  [con("Reach and minor-directedness are predictions, not facts, at emit time, so the context can be wrong in either direction","a niche post unexpectedly goes viral into an audience the gate never anticipated")],
  ["minor-directed content not flagged for heightened policy","reach over-estimated, applying needless escalations","political topical sensitivity missed before an election window"],
  ["an audience context (reach band, minor-likelihood, topical sensitivity) is recorded for the emit"],
  ["the minor-directed standard changes","an emit reaches a materially different audience than predicted"],
  specialists=["disclosure_gate_analyst","audience_policy_analyst"]))

# ---------- rights / regime layer ----------
N.append(node("SYNTHETIC_MEDIA_DISCLOSURE","mandatory_ai_content_disclosure",
  "Apply the mandatory AI-content transparency obligation: under EU AI Act Article 50, deployers of systems that generate or manipulate image, audio, or video constituting deep fakes must disclose that the content is artificially generated or manipulated, in a clear and distinguishable way; this is the central required-label obligation the gate enforces.",
  "regime", ["SYNTHETIC_MEDIA_CLASSIFY"], ["CQ_03","CQ_06"],
  b(0.95,0.82,0.82,0.6,0.9,0.85,0.62,0.62,0.6, 0.72,0.78, 0.94,0.62,0.22,0.6,[0.24,0.58],"EU AI Act Art 50 transparency scope or guidance changes"),
  ["deep-fake disclosure obligation","clear-and-distinguishable labeling requirement","machine-readable marking expectation"],
  ["the per-platform label UI","provenance signing internals","operator execution"],
  [pro("Grounding the core obligation in EU AI Act Art 50 gives the gate a concrete statutory anchor: deep-fake content must be disclosed as artificially generated or manipulated","Art 50(4) requires deployers to disclose AI-generated or manipulated image/audio/video that is a deep fake")],
  [con("Art 50 carries exceptions (assistive editing, law-enforcement, artistic/satirical works with proportionate disclosure) that a blunt always-label rule mis-handles","an evidently artistic synthetic work needs disclosure that does not spoil the work, not a blanket banner")],
  ["a required deep-fake disclosure omitted entirely","disclosure present but not clear-and-distinguishable","artistic/satirical proportionate-disclosure exception ignored or over-applied"],
  ["a clear-and-distinguishable AI-content disclosure obligation is recorded for any artifact classed synthetic/manipulated, with any applicable exception noted"],
  ["EU AI Act Art 50 transparency guidance changes","a disclosure is found not to be clear-and-distinguishable in review"],
  specialists=["disclosure_gate_analyst","ai_act_transparency_specialist"],
  human_review=True))

N.append(node("MANIPULATED_MEDIA_POLICY","platform_manipulated_media_policy",
  "Apply destination-platform manipulated-media and synthetic-content policies (Meta, YouTube, TikTok) which independently require labeling or prohibit certain altered/deepfake media; these platform rules co-exist with statute and can be stricter, so the emit must satisfy the platform policy even where statute is silent.",
  "regime", ["SYNTHETIC_MEDIA_CLASSIFY","PLATFORM_TOS"], ["CQ_06","CQ_07"],
  b(0.88,0.8,0.76,0.62,0.85,0.82,0.58,0.62,0.58, 0.72,0.78, 0.88,0.62,0.22,0.56,[0.24,0.58],"a platform's synthetic/manipulated-media policy version changes"),
  ["platform AI-content label requirement","prohibited-deepfake categories","altered-media context rules"],
  ["statutory labeling text","the in-band token format","operator execution"],
  [pro("Honoring platform policy catches removals/penalties statute would not predict, since platforms label and demote unlabeled AI media on their own terms","YouTube requires creators to disclose realistic altered or synthetic content; TikTok requires labeling AI-generated content and auto-labels C2PA-marked uploads; Meta labels AI-generated imagery")],
  [con("Platform policies are uneven, fast-changing, and sometimes vaguer than statute, so encoding them risks both over- and under-blocking","one platform's 'realistic' threshold differs from another's, and the same artifact is treated differently per destination")],
  ["destination platform's mandatory AI label not satisfied","a platform-prohibited deepfake category emitted","altered-media policy treated as identical across platforms"],
  ["the destination platform's manipulated/synthetic-media labeling or prohibition rules for this artifact are identified and reconciled with statute"],
  ["a platform synthetic-media policy changes","a platform demotes/removes a correctly-statute-labeled emit"],
  specialists=["disclosure_gate_analyst","platform_policy_specialist"],
  human_review=True))

N.append(node("CONTENT_PROVENANCE","c2pa_content_credentials_provenance",
  "Use C2PA / Content Credentials as the provenance substrate: a tamper-evident, cryptographically signed manifest bound to the asset that records its creation and edit history (including whether AI tools were used), giving the disclosure a verifiable, machine-readable carrier rather than a strippable visual caption.",
  "regime", ["SYNTHETIC_MEDIA_CLASSIFY"], ["CQ_04","CQ_08"],
  b(0.86,0.78,0.74,0.74,0.82,0.85,0.6,0.6,0.55, 0.7,0.76, 0.86,0.6,0.24,0.55,[0.26,0.62],"the C2PA spec or Content Credentials adoption profile changes"),
  ["signed provenance manifest","creation/edit assertions","AI-tool-use assertion","binding of manifest to asset bytes"],
  ["statutory obligation text","the platform label UI","operator execution"],
  [pro("C2PA gives disclosure a tamper-evident, signed carrier whose alteration is detectable, which a visual caption can never offer","a C2PA manifest records that a generative-fill tool edited the image, and any later tampering breaks the signature")],
  [con("C2PA manifests can be stripped by re-encoding or screenshotting, and verification depends on platform support, so provenance is necessary but not self-sufficient","a screenshot of a credentialed image drops the manifest, defeating provenance unless a durable in-band mark also exists")],
  ["provenance manifest absent on a synthetic asset","manifest present but not bound to the asset bytes","provenance relied on where the destination cannot verify it"],
  ["a synthetic/manipulated asset carries a C2PA/Content Credentials manifest binding creation and AI-edit assertions to the asset, with a recorded stripping-risk assessment"],
  ["the C2PA spec or adoption profile changes","a credentialed asset is observed stripped of its manifest"],
  specialists=["disclosure_gate_analyst","provenance_engineer"]))

N.append(node("LABELING_REGIMES","per_jurisdiction_and_per_platform_label_set",
  "Assemble the applicable labeling regime set: the union of statutory requirements per asserted jurisdiction (EU AI Act, India IT Rules 2021 and the 2023 deepfake advisories, plus any state synthetic-media laws) and per-destination-platform policy, so the gate reasons over the full superset of labels the emit must carry.",
  "regime", ["SYNTHETIC_MEDIA_DISCLOSURE","MANIPULATED_MEDIA_POLICY"], ["CQ_07","CQ_09"],
  b(0.88,0.8,0.78,0.66,0.85,0.85,0.6,0.6,0.6, 0.72,0.76, 0.88,0.6,0.22,0.56,[0.24,0.58],"a jurisdiction adds, amends, or repeals a synthetic-media labeling rule"),
  ["per-jurisdiction statutory label set","per-platform policy label set","superset reconciliation across regimes"],
  ["jurisdiction selection logic","the token mechanism","operator execution"],
  [pro("Treating labels as a superset over all applicable regimes prevents satisfying one jurisdiction while violating another","an emit reaching both EU and Indian audiences must satisfy EU AI Act Art 50 and India's IT Rules deepfake-labeling advisories simultaneously")],
  [con("Regimes conflict in form and wording, and stacking every label can produce an over-cluttered or self-contradictory disclosure","two regimes mandate differently-worded mandatory captions that cannot both be the single visible label")],
  ["a jurisdiction's required label omitted from the set","conflicting regime requirements collapsed to the weaker one","platform policy dropped because statute was satisfied"],
  ["the labeling-requirement set is the reconciled superset of every applicable jurisdiction and platform rule for this emit"],
  ["a jurisdiction's labeling law changes","two regimes' label requirements are found to conflict"],
  specialists=["disclosure_gate_analyst","multi_jurisdiction_policy_analyst"],
  human_review=True))

N.append(node("ELECTION_INTEGRITY","election_period_synthetic_content_rules",
  "Apply heightened election-integrity rules: during defined election periods, synthetic or manipulated political content (especially depicting candidates or officials) is subject to stricter disclosure, prohibition, or removal under platform political-content policies and laws such as India's advisories and EU/state election rules, escalating the obligation for politically-classed emits.",
  "regime", ["LABELING_REGIMES","AUDIENCE_CONTEXT"], ["CQ_05","CQ_10"],
  b(0.86,0.76,0.78,0.62,0.88,0.82,0.66,0.6,0.62, 0.72,0.76, 0.86,0.6,0.24,0.62,[0.26,0.62],"an election-period rule or window definition changes"),
  ["election-period window detection","political/candidate-depiction synthetic content","heightened-disclosure or prohibition escalation"],
  ["general-audience labeling","operator execution","content moderation appeals"],
  [pro("Escalating in election windows catches the highest-harm synthetic-media cases where a generic label is insufficient or the content is outright prohibited","a synthetic clip of a candidate during an election period can require prominent disclosure or be barred entirely under platform political-deepfake rules")],
  [con("Election periods, jurisdictions, and 'political' scope are hard to define precisely, so the escalation can fire on satire or news or miss novel formats","a satirical synthetic skit during an election may be protected expression, not a prohibited deepfake")],
  ["a prohibited election-period synthetic political emit not blocked","general satire over-blocked as a prohibited deepfake","election window not recognized for the relevant jurisdiction"],
  ["politically-classed synthetic emits in an active election window are escalated to heightened disclosure or block per the applicable rule"],
  ["an election-period rule or window changes","a synthetic political emit is mis-escalated in review"],
  specialists=["disclosure_gate_analyst","election_integrity_specialist"],
  human_review=True))

N.append(node("AGE_PLATFORM_POLICY","minor_audience_and_platform_age_rules",
  "Apply audience-age and platform minor-protection policies: where content is directed to or likely to reach minors, platform rules and child-safety regimes impose stricter limits on synthetic likenesses, synthetic minors, and advertising, escalating disclosure or prohibition for minor-facing synthetic emits.",
  "regime", ["LABELING_REGIMES","AUDIENCE_CONTEXT"], ["CQ_05","CQ_10"],
  b(0.84,0.74,0.76,0.58,0.86,0.8,0.62,0.62,0.58, 0.74,0.78, 0.84,0.62,0.22,0.6,[0.24,0.58],"a platform minor policy or child-safety regime changes"),
  ["minor-directed escalation","synthetic-minor and synthetic-likeness limits","minor-facing advertising restrictions"],
  ["general labeling","election rules","operator execution"],
  [pro("Minor-facing escalation blocks the highest-sensitivity synthetic cases (synthetic depictions of or ads to children) that a default label does not address","a synthetic depiction of a minor, or AI-personalized advertising to a kids' audience, triggers stricter platform limits than a general AI label")],
  [con("Minor-directedness is a prediction and child-safety regimes vary, so the escalation can over-restrict general content or miss minor reach","a general-audience post that incidentally reaches minors may be over-restricted by a minor-directed assumption")],
  ["synthetic-minor content not blocked under child-safety rules","general content over-restricted as minor-directed","minor reach unaccounted in the escalation"],
  ["minor-directed or minor-likely synthetic emits are escalated to the applicable platform minor and child-safety limits"],
  ["a platform minor policy changes","a minor-facing synthetic emit is mis-handled in review"],
  specialists=["disclosure_gate_analyst","child_safety_policy_analyst"],
  human_review=True))

N.append(node("REDISTRIBUTION_VS_REFERENCE","redistribution_versus_reference_only_use",
  "Distinguish republishing the media (redistribution, which triggers the full ToS and licensing gate) from merely referencing or linking it (reference-only, which typically does not redistribute the bytes), because the obligation set and the scrape-license gate apply only to actual redistribution, not to a citation or link.",
  "regime", ["SCRAPE_LICENSE","PLATFORM_TOS"], ["CQ_04","CQ_07"],
  b(0.8,0.74,0.72,0.58,0.8,0.78,0.55,0.66,0.52, 0.76,0.78, 0.8,0.66,0.2,0.5,[0.22,0.54],"the redistribution/reference boundary or embedding behavior changes"),
  ["republish-vs-link classification","embed/quote handling","reference-only carve-out"],
  ["statutory AI labeling of original synthetic content","operator execution"],
  [pro("Separating reference from redistribution avoids blocking legitimate citation while still gating actual republication of unlicensed bytes","linking to a hosted news clip is reference-only and skips the scrape-license republish gate, whereas re-uploading the clip is redistribution and does not")],
  [con("Embeds and quote-reposts blur the line, since an embed can effectively redistribute the bytes while looking like a reference","a platform embed that re-hosts the media is functionally redistribution despite presenting as a link")],
  ["an embed that re-hosts bytes treated as reference-only","a true reference over-gated as redistribution","quote-repost mis-classified"],
  ["the emit is classified as redistribution or reference-only, and the scrape-license/ToS republish gate is applied only to redistribution"],
  ["the redistribution/reference boundary changes","an embed is found to re-host bytes it was treated as referencing"],
  specialists=["disclosure_gate_analyst","redistribution_rights_analyst"]))

# ---------- token / mechanism layer ----------
N.append(node("IN_BAND_DISCLOSURE_TOKEN","non_strippable_in_band_disclosure_token",
  "Specify the in-band disclosure token: a machine-readable disclosure marker carried inside the artifact itself (embedded metadata, C2PA assertion, and a durable watermark) that the downstream session cannot trivially strip, so the disclosure travels WITH the content rather than as an out-of-band note the next hop can drop.",
  "mechanism", ["CONTENT_PROVENANCE","SYNTHETIC_MEDIA_DISCLOSURE"], ["CQ_08","CQ_11"],
  b(0.92,0.8,0.8,0.74,0.9,0.85,0.66,0.58,0.62, 0.7,0.76, 0.92,0.58,0.24,0.62,[0.26,0.62],"the token format or non-strippability requirement changes"),
  ["embedded machine-readable disclosure","C2PA assertion carrier","durable watermark layer","strip-resistance properties"],
  ["the visible human-facing caption only","statutory text","operator execution"],
  [pro("An in-band token means a downstream session that re-emits the asset carries the disclosure forward by default, since stripping it requires deliberate tampering rather than mere omission","a watermarked + C2PA-marked clip stays disclosed when reposted, whereas a caption-only label is lost on the first re-upload")],
  [con("No single in-band channel is unstrippable: metadata is dropped on re-encode, watermarks degrade, and C2PA breaks on screenshot, so robustness requires redundancy and is never absolute","a determined re-encode can defeat any one carrier, so the token must layer several and still cannot guarantee survival")],
  ["disclosure carried only out-of-band where the next hop can drop it","a single fragile carrier relied on as if unstrippable","token machine-unreadable to the destination"],
  ["the disclosure is carried in-band via at least one durable machine-readable channel, with redundant carriers and a recorded strip-resistance assessment"],
  ["the token format changes","an in-band token is observed stripped at a downstream hop"],
  specialists=["disclosure_gate_analyst","provenance_engineer"],
  human_review=True))

N.append(node("PROVENANCE_SIGNING","cryptographic_provenance_manifest_signing",
  "Cryptographically sign the C2PA provenance manifest so the disclosure and edit-history assertions are tamper-evident and verifiable by a relying party: any alteration of the asset or its claims after signing breaks verification, turning the disclosure into a checkable claim rather than an editable annotation.",
  "mechanism", ["CONTENT_PROVENANCE"], ["CQ_08","CQ_11"],
  b(0.84,0.76,0.7,0.78,0.82,0.78,0.62,0.6,0.55, 0.7,0.76, 0.84,0.6,0.24,0.56,[0.26,0.62],"the signing trust model or certificate policy changes"),
  ["manifest signing","trust-anchor/certificate handling","tamper-evidence verification"],
  ["watermark robustness","statutory text","operator execution"],
  [pro("Signing makes the provenance claim verifiable and tamper-evident, so a relying platform can trust the disclosure or detect that it was altered","a C2PA signature lets TikTok verify and auto-apply an AI label from the manifest, and detect if the manifest was edited")],
  [con("Signing depends on key custody and a trust ecosystem; a compromised key, expired cert, or unrecognized trust anchor undermines verification","a manifest signed by an untrusted issuer is cryptographically valid but not trusted by the destination")],
  ["manifest left unsigned, so claims are silently editable","signing key compromise unaddressed","trust anchor not recognized by the destination"],
  ["the provenance manifest is signed by a recognized trust anchor and its tamper-evidence verifies at the destination"],
  ["the signing trust model changes","a signature fails verification at a destination that should trust it"],
  specialists=["disclosure_gate_analyst","provenance_engineer"]))

N.append(node("DISCLOSURE_SLOT_BINDING","downstream_disclosure_slot_binding",
  "Bind the required disclosure token to a concrete disclosure slot in the downstream handoff: confirm the destination session/platform exposes a place the in-band token (and any required visible label) can be deposited and will be carried, so the disclosure has a guaranteed home in the next hop rather than an assumed one.",
  "mechanism", ["IN_BAND_DISCLOSURE_TOKEN","LABELING_REGIMES"], ["CQ_11","CQ_12"],
  b(0.9,0.78,0.78,0.7,0.9,0.85,0.66,0.58,0.62, 0.7,0.74, 0.9,0.58,0.26,0.62,[0.28,0.66],"the downstream handoff contract or available slot changes"),
  ["destination disclosure-slot discovery","token-to-slot binding","visible-label slot confirmation"],
  ["the block decision itself","statutory text","operator execution"],
  [pro("Explicitly binding the token to a real downstream slot turns 'the next hop will surely carry it' into a verified property, exposing missing-slot cases before emit","confirming TikTok's AI-content toggle and C2PA ingestion exist binds the disclosure to a real slot rather than hoping a caption survives")],
  [con("Downstream slots are heterogeneous and undocumented, and a slot may exist but silently not propagate the token, so binding can give false assurance","a platform accepts the metadata field but strips it server-side, so the bound slot does not actually carry the disclosure")],
  ["disclosure emitted with no bound downstream slot","slot bound but not verified to propagate","visible-label slot assumed but absent"],
  ["each required disclosure is bound to a confirmed downstream slot that is verified to carry it forward"],
  ["the downstream handoff contract changes","a bound slot is found not to propagate the token"],
  specialists=["disclosure_gate_analyst","handoff_integration_engineer"],
  human_review=True))

# ---------- aggregation / decision layer ----------
N.append(node("JURISDICTION_RESOLUTION","applicable_jurisdiction_resolution",
  "Resolve which jurisdictions' rules actually apply to this emit from the asserted publication geography, audience location, and platform reach, producing the jurisdiction set that LABELING_REGIMES and the escalations reason over, since applying the wrong jurisdiction set silently mis-scopes every obligation.",
  "decision", ["EMIT_INTAKE","LABELING_REGIMES"], ["CQ_09","CQ_10"],
  b(0.86,0.78,0.74,0.66,0.85,0.82,0.62,0.6,0.58, 0.72,0.76, 0.86,0.6,0.22,0.58,[0.24,0.6],"the jurisdiction-determination rule or audience-geo signal changes"),
  ["publication-geography resolution","audience-location reach","applicable-jurisdiction set output"],
  ["the specific statutory text per jurisdiction","operator execution"],
  [pro("Resolving jurisdiction explicitly prevents applying one country's rules to an audience governed by another's, scoping the obligation set correctly","an EU-targeted emit pulls in the AI Act, while an India-reaching emit pulls in the IT Rules deepfake advisories")],
  [con("Reach crosses borders and geo-signals are noisy, so the resolved jurisdiction set can be over- or under-inclusive","a globally accessible post nominally reaches every jurisdiction, making a literal union impractically broad")],
  ["a reaching jurisdiction omitted from the set","an inapplicable jurisdiction's rules applied","global reach collapsed to a single home jurisdiction"],
  ["the applicable-jurisdiction set is resolved from publication geography and audience reach and is passed to the labeling and escalation rules"],
  ["the jurisdiction-determination rule changes","an emit is found to reach a jurisdiction not in the resolved set"],
  specialists=["disclosure_gate_analyst","multi_jurisdiction_policy_analyst"]))

N.append(node("DISCLOSURE_OBLIGATION_SET","aggregated_disclosure_obligation_set",
  "Aggregate every triggered obligation into one disclosure-obligation set for the emit: the union of statutory labels, platform policies, provenance/in-band token requirements, redistribution clearances, and any election/minor escalations, with each obligation tagged satisfied or unmet, producing the single checklist the handoff gate enforces.",
  "decision", ["JURISDICTION_RESOLUTION","ELECTION_INTEGRITY","AGE_PLATFORM_POLICY","REDISTRIBUTION_VS_REFERENCE","DISCLOSURE_SLOT_BINDING","PROVENANCE_SIGNING","SCRAPE_LICENSE"], ["CQ_09","CQ_12"],
  b(0.92,0.82,0.82,0.7,0.92,0.88,0.66,0.58,0.62, 0.7,0.74, 0.92,0.58,0.24,0.64,[0.28,0.66],"any upstream obligation, escalation, or token requirement changes"),
  ["union of all triggered obligations","per-obligation satisfied/unmet tagging","single emit-level disclosure checklist"],
  ["the block/allow action itself","statutory text drafting","operator execution"],
  [pro("A single aggregated, per-obligation-tagged set makes the gate's decision a deterministic checklist rather than a scattered set of partial checks","the set lists 'EU AI Act label: bound', 'TikTok AI toggle: bound', 'scrape license: missing' so the unmet item is unambiguous")],
  [con("Aggregation hides the provenance of each obligation unless every entry is traceable, and a missing upstream trigger silently shrinks the set","if the election escalation never ran, its obligation is simply absent from the set with no signal that it was skipped")],
  ["an obligation present upstream missing from the aggregated set","obligations aggregated without satisfied/unmet tags","set treated as complete despite a skipped upstream check"],
  ["the obligation set is the union of all triggered obligations, each tagged satisfied or unmet, with traceability to the rule that produced it"],
  ["an upstream obligation or escalation changes","an obligation is found missing from a previously-passed set"],
  specialists=["disclosure_gate_analyst","obligation_aggregation_engineer"],
  human_review=True))

N.append(node("FAIL_CLOSED_AT_HANDOFF","fail_closed_handoff_spof_gate",
  "Enforce the fail-closed handoff gate, the single point of failure of the whole pipeline: at the downstream handoff, if any disclosure obligation is unmet, no disclosure slot is bound, or the in-band token cannot be verified to survive transit, the emit is BLOCKED; the gate defaults to block on any uncertainty and never strips or downgrades a disclosure to let the emit through.",
  "decision", ["DISCLOSURE_OBLIGATION_SET"], ["CQ_12","CQ_13"],
  b(0.97,0.85,0.85,0.66,0.95,0.9,0.78,0.6,0.65, 0.7,0.74, 0.96,0.6,0.22,0.7,[0.26,0.6],"the fail-closed default or block criteria change"),
  ["block-on-any-unmet-obligation rule","block-on-missing-slot rule","block-on-unverifiable-token rule","default-to-block under uncertainty"],
  ["legal advice","appeals/override workflow","operator execution of an allowed emit"],
  [pro("Defaulting to block at the single handoff means a missing or strippable disclosure can never silently leak through; the failure mode is a blocked emit, not an unlabeled synthetic publication","an emit whose C2PA slot is unverified is blocked rather than published with a disclosure that the next hop would drop")],
  [con("A strict fail-closed gate at one SPOF over-blocks legitimate emits when an upstream signal is merely uncertain, and concentrates all risk in one component that must itself be highly reliable","a transient verification failure on a perfectly compliant emit blocks it, creating pressure to weaken the default")],
  ["gate fails open and emits without a satisfied obligation","disclosure stripped or downgraded to force a pass","SPOF gate itself unmonitored so its own failure is silent"],
  ["any unmet obligation, unbound slot, or unverifiable in-band token causes the emit to be BLOCKED, with no path that strips or downgrades a disclosure to allow it"],
  ["the fail-closed default or block criteria change","the gate is found to have failed open on any emit"],
  specialists=["disclosure_gate_analyst","reliability_engineer"],
  human_review=True))

N.append(node("HANDOFF_GATE_VERIFICATION","token_survival_and_decision_audit",
  "Verify that the bound in-band token actually survives the handoff and record the gate decision: re-read the disclosure at (or simulating) the downstream destination to confirm the token and visible label are present and verifiable, and write an audit record of the obligation set, the survival check, and the allow-with-disclosure / block outcome.",
  "verification", ["FAIL_CLOSED_AT_HANDOFF"], ["CQ_13","CQ_14"],
  b(0.88,0.78,0.78,0.66,0.9,0.82,0.66,0.62,0.55, 0.74,0.78, 0.88,0.62,0.22,0.6,[0.24,0.58],"the survival-verification method or audit schema changes"),
  ["post-handoff token re-read","visible-label presence check","decision audit record"],
  ["the block rule itself","legal advice","downstream content moderation"],
  [pro("Re-reading the disclosure at the destination converts 'the token was attached' into 'the token is present and verifiable where it must be', closing the gap the SPOF most fears","a post-emit fetch confirms the C2PA manifest and AI label are intact on the published TikTok asset")],
  [con("True downstream re-read may be impossible before emit, so a simulation can diverge from real platform processing and miss server-side stripping","a staging check passes but the live platform's transcoding strips the metadata the staging path preserved")],
  ["survival assumed rather than verified","audit record omits the obligation set or outcome","simulation diverges from live platform behavior"],
  ["the in-band token's survival is verified (or best-effort simulated) at the destination and an audit record of obligations, survival, and outcome is written"],
  ["the survival-verification method changes","a token verified at emit is found stripped at the live destination"],
  specialists=["disclosure_gate_analyst","verification_engineer"],
  human_review=True))

# ---- competency questions (14) ----
CQ = [
 ("CQ_01","How is the emit-action captured and how are the destination platform's ToS constraints resolved?",["nodes"],"EMIT_INTAKE normalizes the emit unit and PLATFORM_TOS resolves binding ToS constraints",["EMIT_INTAKE","PLATFORM_TOS"]),
 ("CQ_02","How is the artifact classified as synthetic/manipulated versus authentic, and how is its source-license provenance traced?",["nodes"],"SYNTHETIC_MEDIA_CLASSIFY sets the generation path and SCRAPE_LICENSE traces redistribution license",["SYNTHETIC_MEDIA_CLASSIFY","SCRAPE_LICENSE","PLATFORM_TOS"]),
 ("CQ_03","What statutory AI-content disclosure obligation attaches to a synthetic artifact, and how does audience context modify it?",["nodes"],"SYNTHETIC_MEDIA_DISCLOSURE applies EU AI Act Art 50; AUDIENCE_CONTEXT and SYNTHETIC_MEDIA_CLASSIFY set the trigger",["SYNTHETIC_MEDIA_DISCLOSURE","SYNTHETIC_MEDIA_CLASSIFY","AUDIENCE_CONTEXT"]),
 ("CQ_04","How are redistribution and reference-only uses distinguished, and how does provenance gate unlicensed source media?",["nodes","conflict_axes"],"REDISTRIBUTION_VS_REFERENCE and SCRAPE_LICENSE/CONTENT_PROVENANCE gate unlicensed republication",["REDISTRIBUTION_VS_REFERENCE","SCRAPE_LICENSE","CONTENT_PROVENANCE"]),
 ("CQ_05","How are election-period and minor-audience escalations triggered from audience context?",["nodes"],"AUDIENCE_CONTEXT feeds ELECTION_INTEGRITY and AGE_PLATFORM_POLICY escalations",["AUDIENCE_CONTEXT","ELECTION_INTEGRITY","AGE_PLATFORM_POLICY"]),
 ("CQ_06","How do statutory disclosure and platform manipulated-media policy combine for the same artifact?",["nodes","edges"],"SYNTHETIC_MEDIA_DISCLOSURE and MANIPULATED_MEDIA_POLICY co-apply, platform possibly stricter",["SYNTHETIC_MEDIA_DISCLOSURE","MANIPULATED_MEDIA_POLICY"]),
 ("CQ_07","How is the full per-jurisdiction and per-platform labeling superset assembled and reconciled?",["nodes"],"LABELING_REGIMES unions statute and platform rules; MANIPULATED_MEDIA_POLICY and REDISTRIBUTION_VS_REFERENCE feed it",["LABELING_REGIMES","MANIPULATED_MEDIA_POLICY","REDISTRIBUTION_VS_REFERENCE"]),
 ("CQ_08","How does C2PA provenance carry the disclosure, and how is the manifest made tamper-evident?",["nodes"],"CONTENT_PROVENANCE supplies the manifest; PROVENANCE_SIGNING signs it; IN_BAND_DISCLOSURE_TOKEN carries it",["CONTENT_PROVENANCE","PROVENANCE_SIGNING","IN_BAND_DISCLOSURE_TOKEN"]),
 ("CQ_09","How is the applicable jurisdiction set resolved and aggregated into the obligation set?",["nodes"],"JURISDICTION_RESOLUTION scopes jurisdictions feeding LABELING_REGIMES and DISCLOSURE_OBLIGATION_SET",["JURISDICTION_RESOLUTION","LABELING_REGIMES","DISCLOSURE_OBLIGATION_SET"]),
 ("CQ_10","How are election and minor escalations scoped to the right jurisdictions and audiences?",["nodes","conflict_axes"],"JURISDICTION_RESOLUTION and AUDIENCE_CONTEXT scope ELECTION_INTEGRITY and AGE_PLATFORM_POLICY",["ELECTION_INTEGRITY","AGE_PLATFORM_POLICY","JURISDICTION_RESOLUTION"]),
 ("CQ_11","How is the disclosure made in-band and non-strippable, and bound to a downstream slot?",["nodes"],"IN_BAND_DISCLOSURE_TOKEN, PROVENANCE_SIGNING and DISCLOSURE_SLOT_BINDING make disclosure travel in-band to a confirmed slot",["IN_BAND_DISCLOSURE_TOKEN","PROVENANCE_SIGNING","DISCLOSURE_SLOT_BINDING"]),
 ("CQ_12","How are all obligations aggregated and bound before the handoff?",["nodes","workflow"],"DISCLOSURE_OBLIGATION_SET aggregates obligations and DISCLOSURE_SLOT_BINDING binds them to slots",["DISCLOSURE_OBLIGATION_SET","DISCLOSURE_SLOT_BINDING","FAIL_CLOSED_AT_HANDOFF"]),
 ("CQ_13","What happens at the fail-closed handoff SPOF when an obligation is unmet or a token may not survive?",["nodes","edge_cases"],"FAIL_CLOSED_AT_HANDOFF blocks the emit and HANDOFF_GATE_VERIFICATION verifies token survival",["FAIL_CLOSED_AT_HANDOFF","HANDOFF_GATE_VERIFICATION"]),
 ("CQ_14","How is the gate decision audited and token survival verified at the destination?",["nodes","workflow"],"HANDOFF_GATE_VERIFICATION re-reads the disclosure and records the decision audit",["HANDOFF_GATE_VERIFICATION"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

# consolidate node->CQ refs onto the 14-CQ set (1-2 each; every CQ covered)
CQ_MAP = {
 "EMIT_INTAKE":["CQ_01"], "PLATFORM_TOS":["CQ_01","CQ_02"], "SYNTHETIC_MEDIA_CLASSIFY":["CQ_02","CQ_03"],
 "SCRAPE_LICENSE":["CQ_02","CQ_04"], "AUDIENCE_CONTEXT":["CQ_03","CQ_05"],
 "SYNTHETIC_MEDIA_DISCLOSURE":["CQ_03","CQ_06"], "MANIPULATED_MEDIA_POLICY":["CQ_06","CQ_07"],
 "CONTENT_PROVENANCE":["CQ_04","CQ_08"], "LABELING_REGIMES":["CQ_07","CQ_09"],
 "ELECTION_INTEGRITY":["CQ_05","CQ_10"], "AGE_PLATFORM_POLICY":["CQ_05","CQ_10"],
 "REDISTRIBUTION_VS_REFERENCE":["CQ_04","CQ_07"], "IN_BAND_DISCLOSURE_TOKEN":["CQ_08","CQ_11"],
 "PROVENANCE_SIGNING":["CQ_08","CQ_11"], "DISCLOSURE_SLOT_BINDING":["CQ_11","CQ_12"],
 "JURISDICTION_RESOLUTION":["CQ_09","CQ_10"], "DISCLOSURE_OBLIGATION_SET":["CQ_09","CQ_12"],
 "FAIL_CLOSED_AT_HANDOFF":["CQ_12","CQ_13"], "HANDOFF_GATE_VERIFICATION":["CQ_13","CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

GL = [
 ("synthetic_media","image, audio, or video that is artificially generated or substantially manipulated by AI, as opposed to captured-real unaltered media",["ai_generated_media","generative_media"],["captured_real_media"],["SYNTHETIC_MEDIA_CLASSIFY","SYNTHETIC_MEDIA_DISCLOSURE"]),
 ("deep_fake","synthetic or manipulated media that appreciably resembles real persons, objects, places, or events and would falsely appear authentic, the trigger term used in EU AI Act Art 50",["deepfake"],["benign_ai_edit"],["SYNTHETIC_MEDIA_DISCLOSURE","MANIPULATED_MEDIA_POLICY"]),
 ("in_band_disclosure","a disclosure carried inside the artifact itself (metadata, C2PA assertion, watermark) so it travels with the content rather than as a separable note",["embedded_disclosure"],["out_of_band_caption"],["IN_BAND_DISCLOSURE_TOKEN","DISCLOSURE_SLOT_BINDING"]),
 ("c2pa","the Coalition for Content Provenance and Authenticity standard for a tamper-evident, signed provenance manifest bound to a media asset, surfaced to users as Content Credentials",["content_credentials","content_provenance_standard"],["visual_watermark_only"],["CONTENT_PROVENANCE","PROVENANCE_SIGNING"]),
 ("redistribution","republishing the actual media bytes to a destination, as distinct from referencing or linking to them",["republication","re_upload"],["reference_only"],["REDISTRIBUTION_VS_REFERENCE","SCRAPE_LICENSE"]),
 ("fail_closed","a default that blocks the protected action whenever any required condition is unmet or uncertain, rather than allowing it",["default_deny","block_on_uncertainty"],["fail_open"],["FAIL_CLOSED_AT_HANDOFF","DISCLOSURE_OBLIGATION_SET"]),
 ("handoff_spof","the single downstream handoff point at which the disclosure gate is enforced and whose failure determines the whole pipeline's safety",["single_point_of_failure","handoff_gate"],["redundant_gate"],["FAIL_CLOSED_AT_HANDOFF","HANDOFF_GATE_VERIFICATION"]),
 ("labeling_regime","a statutory or platform rule set that mandates a particular synthetic-media disclosure label for a given jurisdiction or platform",["disclosure_regime"],["voluntary_label"],["LABELING_REGIMES","JURISDICTION_RESOLUTION"]),
 ("scrape_sourced","media obtained by scraping a platform without a redistribution license, which must not be republished",["unlicensed_scrape"],["owned_or_licensed_source"],["SCRAPE_LICENSE","REDISTRIBUTION_VS_REFERENCE"]),
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
              "risk_of_conflict":"unmanaged tension degrades disclosure correctness","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated behavior",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic, subset of node.dependencies for the spine) -- 24 total
dep("EMIT_INTAKE","PLATFORM_TOS",0.86)
dep("EMIT_INTAKE","SYNTHETIC_MEDIA_CLASSIFY",0.88)
dep("EMIT_INTAKE","AUDIENCE_CONTEXT",0.8)
dep("PLATFORM_TOS","SCRAPE_LICENSE",0.82)
dep("SYNTHETIC_MEDIA_CLASSIFY","SYNTHETIC_MEDIA_DISCLOSURE",0.9)
dep("SYNTHETIC_MEDIA_CLASSIFY","MANIPULATED_MEDIA_POLICY",0.84)
dep("SYNTHETIC_MEDIA_CLASSIFY","CONTENT_PROVENANCE",0.82)
dep("SYNTHETIC_MEDIA_DISCLOSURE","LABELING_REGIMES",0.86)
dep("MANIPULATED_MEDIA_POLICY","LABELING_REGIMES",0.82)
dep("LABELING_REGIMES","ELECTION_INTEGRITY",0.8)
dep("LABELING_REGIMES","AGE_PLATFORM_POLICY",0.8)
dep("SCRAPE_LICENSE","REDISTRIBUTION_VS_REFERENCE",0.8)
dep("CONTENT_PROVENANCE","IN_BAND_DISCLOSURE_TOKEN",0.84)
dep("CONTENT_PROVENANCE","PROVENANCE_SIGNING",0.82)
dep("IN_BAND_DISCLOSURE_TOKEN","DISCLOSURE_SLOT_BINDING",0.86)
dep("EMIT_INTAKE","JURISDICTION_RESOLUTION",0.8)
dep("JURISDICTION_RESOLUTION","DISCLOSURE_OBLIGATION_SET",0.82)
dep("ELECTION_INTEGRITY","DISCLOSURE_OBLIGATION_SET",0.8)
dep("AGE_PLATFORM_POLICY","DISCLOSURE_OBLIGATION_SET",0.8)
dep("REDISTRIBUTION_VS_REFERENCE","DISCLOSURE_OBLIGATION_SET",0.78)
dep("DISCLOSURE_SLOT_BINDING","DISCLOSURE_OBLIGATION_SET",0.84)
dep("PROVENANCE_SIGNING","DISCLOSURE_OBLIGATION_SET",0.78)
dep("DISCLOSURE_OBLIGATION_SET","FAIL_CLOSED_AT_HANDOFF",0.9)
dep("FAIL_CLOSED_AT_HANDOFF","HANDOFF_GATE_VERIFICATION",0.86)

# cross-cutting non-dependency edges
rel("HANDOFF_GATE_VERIFICATION","IN_BAND_DISCLOSURE_TOKEN","feedback",0.78,why="a failed downstream survival check feeds back to strengthen the in-band token's strip-resistance")
rel("HANDOFF_GATE_VERIFICATION","DISCLOSURE_SLOT_BINDING","feedback",0.76,why="verification of token survival validates or invalidates the bound downstream slot")
rel("AUDIENCE_CONTEXT","JURISDICTION_RESOLUTION","causal",0.74,why="audience location drives which jurisdictions' rules apply")
rel("CONTENT_PROVENANCE","MANIPULATED_MEDIA_POLICY","causal",0.74,why="a C2PA manifest lets platforms auto-apply their AI-content label, satisfying platform policy")
rel("SCRAPE_LICENSE","FAIL_CLOSED_AT_HANDOFF","constraint",0.78,why="an unlicensed-scrape redistribution is an unmet obligation that forces a block")
rel("PROVENANCE_SIGNING","HANDOFF_GATE_VERIFICATION","similarity",0.72,why="signature verification at the destination is the tamper-evidence the survival check relies on")
rel("ELECTION_INTEGRITY","FAIL_CLOSED_AT_HANDOFF","constraint",0.76,why="a prohibited election-period synthetic emit is an unmet obligation that forces a block")
rel("AGE_PLATFORM_POLICY","FAIL_CLOSED_AT_HANDOFF","constraint",0.74,why="a minor-protection violation is an unmet obligation that forces a block")

# conflict edges (negative signed_tension + resolution_rule) -- 4
conf("FAIL_CLOSED_AT_HANDOFF","PLATFORM_TOS",0.7,-0.6,
  "disclosure dominates throughput: when a fail-closed block would stop an emit a publishing SLA wants out, the gate still blocks; throughput is recovered by binding a real disclosure slot, never by stripping or downgrading the disclosure",
  "the fail-closed disclosure block conflicts with platform/business pressure to publish quickly and frictionlessly")
conf("LABELING_REGIMES","IN_BAND_DISCLOSURE_TOKEN",0.7,-0.5,
  "satisfy the superset in a single durable carrier: when two regimes mandate differently-worded visible labels that cannot both be the one caption, carry the machine-readable union in-band and render the strictest human-visible label, escalating genuine conflicts to review",
  "stacking conflicting per-regime label requirements collides with a single in-band token that must carry one coherent disclosure")
conf("REDISTRIBUTION_VS_REFERENCE","SCRAPE_LICENSE",0.66,-0.45,
  "apply the republish gate only to genuine redistribution: classify embeds that re-host bytes as redistribution and subject them to the scrape-license gate, while true reference-only links bypass it",
  "a permissive reference-only classification can let an embed that actually re-hosts unlicensed bytes escape the scrape-license redistribution gate")
conf("SYNTHETIC_MEDIA_DISCLOSURE","CONTENT_PROVENANCE",0.66,-0.4,
  "treat provenance as necessary but not sufficient: a signed C2PA manifest does not by itself satisfy the statutory clear-and-distinguishable disclosure when the destination cannot verify it, so require a human-perceptible label in addition where provenance may be stripped",
  "relying on C2PA provenance to discharge the statutory disclosure conflicts with provenance being strippable and unverifiable at some destinations")

CA=[
 {"name":"disclosure_mandatory_vs_publishing_throughput","description":"Fail-closed disclosure blocks maximize transparency but add friction and can stop emits a publishing pipeline wants out fast.","poles":["mandatory_disclosure","frictionless_throughput"],"resolution_hint":"recover throughput by binding a real disclosure slot, never by weakening the disclosure","tension_score":0.78,"affected_nodes":["FAIL_CLOSED_AT_HANDOFF","DISCLOSURE_OBLIGATION_SET","PLATFORM_TOS"]},
 {"name":"in_band_robustness_vs_strippability","description":"In-band tokens travel with the content but no single carrier is unstrippable, so robustness needs redundancy and is never absolute.","poles":["in_band_durability","carrier_strippability"],"resolution_hint":"layer metadata, C2PA, and watermark; verify survival; default to block on doubt","tension_score":0.75,"affected_nodes":["IN_BAND_DISCLOSURE_TOKEN","PROVENANCE_SIGNING","HANDOFF_GATE_VERIFICATION"]},
 {"name":"statutory_label_vs_platform_policy","description":"Statute and platform policy can both apply and diverge in form and strictness for the same artifact.","poles":["statutory_label","platform_policy"],"resolution_hint":"take the reconciled superset and satisfy the strictest applicable rule","tension_score":0.7,"affected_nodes":["SYNTHETIC_MEDIA_DISCLOSURE","MANIPULATED_MEDIA_POLICY","LABELING_REGIMES"]},
 {"name":"over_labeling_vs_under_labeling","description":"A broad synthetic threshold floods authentic media with needless labels; a narrow one suppresses required disclosures.","poles":["broad_threshold","narrow_threshold"],"resolution_hint":"calibrate the substantial-manipulation threshold and escalate borderline cases to review","tension_score":0.68,"affected_nodes":["SYNTHETIC_MEDIA_CLASSIFY","SYNTHETIC_MEDIA_DISCLOSURE","MANIPULATED_MEDIA_POLICY"]},
 {"name":"redistribution_gate_vs_legitimate_reference","description":"Gating republication protects rights but can over-block legitimate citation and fair reference.","poles":["strict_republish_gate","open_reference"],"resolution_hint":"gate only genuine redistribution; treat byte-re-hosting embeds as redistribution","tension_score":0.66,"affected_nodes":["REDISTRIBUTION_VS_REFERENCE","SCRAPE_LICENSE","PLATFORM_TOS"]},
 {"name":"jurisdiction_union_vs_practical_scope","description":"Global reach makes a literal union of all jurisdictions' rules impractically broad, while a narrow home-jurisdiction scope under-covers.","poles":["full_union","home_jurisdiction"],"resolution_hint":"resolve jurisdictions from actual audience reach, not nominal global availability","tension_score":0.65,"affected_nodes":["JURISDICTION_RESOLUTION","LABELING_REGIMES","DISCLOSURE_OBLIGATION_SET"]},
 {"name":"election_escalation_vs_protected_expression","description":"Heightened election rules catch high-harm political deepfakes but can over-block satire and news.","poles":["strict_election_escalation","protected_expression"],"resolution_hint":"escalate candidate-depicting deceptive synthesis; preserve clearly-marked satire and news per exception","tension_score":0.66,"affected_nodes":["ELECTION_INTEGRITY","AUDIENCE_CONTEXT","JURISDICTION_RESOLUTION"]},
 {"name":"minor_protection_vs_general_audience_reach","description":"Minor-directed escalations protect children but can over-restrict general content that incidentally reaches minors.","poles":["minor_directed_strictness","general_reach"],"resolution_hint":"escalate on directed-to-minors signals; treat incidental reach proportionately","tension_score":0.62,"affected_nodes":["AGE_PLATFORM_POLICY","AUDIENCE_CONTEXT","DISCLOSURE_OBLIGATION_SET"]},
 {"name":"single_spof_gate_vs_distributed_enforcement","description":"Concentrating enforcement at one fail-closed handoff is simple and auditable but makes that gate a single point of failure that must be highly reliable.","poles":["single_spof_gate","distributed_checks"],"resolution_hint":"keep the SPOF gate but monitor it heavily and verify token survival downstream","tension_score":0.7,"affected_nodes":["FAIL_CLOSED_AT_HANDOFF","HANDOFF_GATE_VERIFICATION","DISCLOSURE_OBLIGATION_SET"]},
]

EC=[
 {"description":"The downstream destination exposes no disclosure slot, so an in-band token cannot be deposited or carried.","trigger":"DISCLOSURE_SLOT_BINDING finds no slot that propagates the token at the handoff","affected_nodes":["DISCLOSURE_SLOT_BINDING","FAIL_CLOSED_AT_HANDOFF","IN_BAND_DISCLOSURE_TOKEN"],"mitigation":"fail closed and BLOCK the emit; do not publish a disclosure the next hop will drop","severity":"critical"},
 {"description":"A correctly C2PA-marked synthetic asset is screenshotted or re-encoded downstream, stripping the manifest and the disclosure.","trigger":"the in-band token's only durable carrier is defeated by re-encode/screenshot","affected_nodes":["IN_BAND_DISCLOSURE_TOKEN","CONTENT_PROVENANCE","HANDOFF_GATE_VERIFICATION"],"mitigation":"layer redundant carriers (watermark + metadata + C2PA) and verify survival; block if none survive","severity":"critical"},
 {"description":"A deepfake of a real person is mis-classified as authentic, suppressing the required AI-content disclosure.","trigger":"SYNTHETIC_MEDIA_CLASSIFY threshold misses substantial manipulation","affected_nodes":["SYNTHETIC_MEDIA_CLASSIFY","SYNTHETIC_MEDIA_DISCLOSURE","MANIPULATED_MEDIA_POLICY"],"mitigation":"calibrate the manipulation threshold conservatively and escalate borderline cases to review","severity":"high"},
 {"description":"An AI restyle of scraped, unlicensed source footage is emitted with a correct AI label but is still an unlicensed republish.","trigger":"SCRAPE_LICENSE gate skipped because the AI disclosure looked sufficient","affected_nodes":["SCRAPE_LICENSE","REDISTRIBUTION_VS_REFERENCE","FAIL_CLOSED_AT_HANDOFF"],"mitigation":"run the scrape-license redistribution gate independently of the AI label and block unlicensed republication","severity":"high"},
 {"description":"Two applicable jurisdictions mandate differently-worded mandatory visible labels that cannot both be the single caption.","trigger":"LABELING_REGIMES superset contains conflicting human-visible label requirements","affected_nodes":["LABELING_REGIMES","IN_BAND_DISCLOSURE_TOKEN","JURISDICTION_RESOLUTION"],"mitigation":"carry the machine-readable union in-band, render the strictest visible label, and escalate true conflicts to review","severity":"high"},
 {"description":"A synthetic clip depicting a candidate is emitted during an active election window without the heightened election disclosure or block.","trigger":"ELECTION_INTEGRITY escalation not triggered for the resolved jurisdiction","affected_nodes":["ELECTION_INTEGRITY","JURISDICTION_RESOLUTION","FAIL_CLOSED_AT_HANDOFF"],"mitigation":"detect election windows per jurisdiction and escalate candidate-depicting synthesis to prominent disclosure or block","severity":"high"},
 {"description":"An embed that re-hosts the underlying media bytes is treated as reference-only and escapes the redistribution gate.","trigger":"REDISTRIBUTION_VS_REFERENCE mis-classifies a byte-re-hosting embed","affected_nodes":["REDISTRIBUTION_VS_REFERENCE","SCRAPE_LICENSE"],"mitigation":"classify byte-re-hosting embeds as redistribution and subject them to the scrape-license gate","severity":"medium"},
 {"description":"The fail-closed handoff gate itself fails open on a transient verification error, publishing an unlabeled synthetic emit.","trigger":"the SPOF gate defaults to allow instead of block on an internal error","affected_nodes":["FAIL_CLOSED_AT_HANDOFF","HANDOFF_GATE_VERIFICATION"],"mitigation":"default to block on any internal/verification uncertainty and monitor the gate's own health","severity":"critical"},
 {"description":"A provenance manifest is signed by an issuer the destination platform does not trust, so the disclosure is valid but not honored.","trigger":"PROVENANCE_SIGNING uses an unrecognized trust anchor","affected_nodes":["PROVENANCE_SIGNING","CONTENT_PROVENANCE","DISCLOSURE_SLOT_BINDING"],"mitigation":"sign with a trust anchor the destination recognizes and add a human-perceptible label as fallback","severity":"medium"},
 {"description":"A general-audience synthetic post unexpectedly reaches a minor audience that the gate never escalated for.","trigger":"AUDIENCE_CONTEXT under-predicted minor reach","affected_nodes":["AUDIENCE_CONTEXT","AGE_PLATFORM_POLICY"],"mitigation":"apply proportionate minor protections on minor-likely reach and re-evaluate on reach signals","severity":"medium"},
 {"description":"A cached platform ToS snapshot diverges from the live policy, so a now-prohibited redistribution is allowed.","trigger":"PLATFORM_TOS applied a stale snapshot","affected_nodes":["PLATFORM_TOS","SCRAPE_LICENSE","REDISTRIBUTION_VS_REFERENCE"],"mitigation":"date ToS snapshots, refresh on a schedule, and treat staleness beyond a bound as an unmet obligation","severity":"medium"},
 {"description":"An obligation triggered upstream is silently absent from the aggregated set because its producing check was skipped.","trigger":"DISCLOSURE_OBLIGATION_SET aggregates without verifying every upstream check ran","affected_nodes":["DISCLOSURE_OBLIGATION_SET","JURISDICTION_RESOLUTION","FAIL_CLOSED_AT_HANDOFF"],"mitigation":"require each upstream check to register a present/absent status; a skipped check is itself an unmet obligation","severity":"high"},
]

WF=[
 {"action":"capture_emit_unit","node_ref":"EMIT_INTAKE","description":"Normalize the emit-action into a fixed unit: artifact, generation path, destination platform/session, and asserted jurisdictions.","artifact":"emit_unit_record","gate":"generation path, destination, and >=1 jurisdiction recorded"},
 {"action":"resolve_platform_tos","node_ref":"PLATFORM_TOS","description":"Resolve destination and source platform ToS: scraping, redistribution, automation, and synthetic-media clauses, dated.","artifact":"tos_constraint_set","gate":"relevant ToS clauses identified and dated"},
 {"action":"classify_generation_path","node_ref":"SYNTHETIC_MEDIA_CLASSIFY","description":"Classify the artifact as fully-synthetic, substantially-edited, AI-assisted-authentic, or captured-real.","artifact":"generation_path_class","gate":"a generation-path class is recorded with evidence"},
 {"action":"trace_source_license","node_ref":"SCRAPE_LICENSE","description":"Trace source provenance and redistribution license; flag scraped/unlicensed source media.","artifact":"source_license_record","gate":"source recorded as owned, licensed, or scraped/non-republishable"},
 {"action":"apply_statutory_disclosure","node_ref":"SYNTHETIC_MEDIA_DISCLOSURE","description":"Apply EU AI Act Art 50 deep-fake disclosure with any artistic/assistive exception noted.","artifact":"statutory_disclosure_obligation","gate":"clear-and-distinguishable disclosure obligation recorded for synthetic artifacts"},
 {"action":"reconcile_labeling_regimes","node_ref":"LABELING_REGIMES","description":"Assemble the per-jurisdiction and per-platform labeling superset and reconcile conflicts.","artifact":"labeling_requirement_set","gate":"reconciled superset of statute and platform labels assembled"},
 {"action":"build_provenance_token","node_ref":"IN_BAND_DISCLOSURE_TOKEN","description":"Build the in-band token via C2PA assertion, metadata, and durable watermark; sign the manifest.","artifact":"in_band_disclosure_token","gate":"disclosure carried in-band via >=1 durable signed channel"},
 {"action":"resolve_jurisdiction","node_ref":"JURISDICTION_RESOLUTION","description":"Resolve the applicable jurisdiction set from publication geography and audience reach.","artifact":"jurisdiction_set","gate":"applicable jurisdiction set resolved from actual reach"},
 {"action":"apply_escalations","node_ref":"ELECTION_INTEGRITY","description":"Apply election-period and minor-audience escalations to politically- or minor-classed emits.","artifact":"escalation_record","gate":"election and minor escalations applied per resolved jurisdiction and audience"},
 {"action":"bind_disclosure_slot","node_ref":"DISCLOSURE_SLOT_BINDING","description":"Bind each required disclosure to a confirmed downstream slot verified to carry it.","artifact":"slot_binding_record","gate":"each required disclosure bound to a propagating downstream slot"},
 {"action":"aggregate_obligations","node_ref":"DISCLOSURE_OBLIGATION_SET","description":"Aggregate all triggered obligations into one set, each tagged satisfied or unmet, with traceability.","artifact":"obligation_set","gate":"every triggered obligation present and tagged satisfied/unmet"},
 {"action":"gate_and_verify_handoff","node_ref":"FAIL_CLOSED_AT_HANDOFF","description":"Block on any unmet obligation, unbound slot, or unverifiable token; then verify survival and write the audit.","artifact":"handoff_decision_record","gate":"emit allowed-with-disclosure only if all obligations met and token survival verified; else blocked"},
]

DR=[
 {"rule":"FAIL_CLOSED_AT_HANDOFF must BLOCK the emit whenever any obligation in DISCLOSURE_OBLIGATION_SET is unmet, no slot is bound, or token survival is unverifiable","rationale":"the handoff is the SPOF; failing open publishes an unlabeled or strippable synthetic emit","trigger":"an emit reaches the handoff with an unmet obligation or unbound slot","action":"block the emit and route to remediation; never strip or downgrade the disclosure"},
 {"rule":"A disclosure may never be stripped or downgraded to make an emit pass the gate","rationale":"stripping defeats the entire purpose of mandatory in-band disclosure","trigger":"a pass would require removing or weakening a required disclosure","action":"block instead and escalate for a compliant disclosure slot"},
 {"rule":"SYNTHETIC_MEDIA_DISCLOSURE statutory obligation dominates platform-policy convenience but never overrides a stricter platform rule","rationale":"both regimes co-apply; the strictest applicable rule governs","trigger":"statute and platform policy diverge for the same artifact","action":"satisfy the reconciled strictest superset from LABELING_REGIMES"},
 {"rule":"SCRAPE_LICENSE redistribution gate must run independently of any AI-content disclosure","rationale":"a correct AI label does not license republication of scraped source media","trigger":"an emit redistributes third-party media with only an AI label","action":"block on missing redistribution license regardless of the AI label"},
 {"rule":"IN_BAND_DISCLOSURE_TOKEN must use redundant durable carriers and survival must be verified before allow","rationale":"no single carrier is unstrippable; an out-of-band note can be dropped by the next hop","trigger":"disclosure carried only out-of-band or via a single fragile carrier","action":"require layered in-band carriers and a survival check"},
 {"rule":"ELECTION_INTEGRITY and AGE_PLATFORM_POLICY escalations dominate the default label for in-scope emits","rationale":"election and minor contexts carry the highest harm and stricter or prohibitive rules","trigger":"a politically- or minor-classed synthetic emit in an in-scope window or audience","action":"apply the heightened disclosure or block before the default label"},
 {"rule":"JURISDICTION_RESOLUTION must be fixed before LABELING_REGIMES finalizes the obligation superset","rationale":"the wrong jurisdiction set silently mis-scopes every label","trigger":"labeling assembled before jurisdictions are resolved from reach","action":"resolve jurisdictions from actual audience reach first"},
 {"rule":"DISCLOSURE_OBLIGATION_SET must record every upstream check as present or absent","rationale":"a silently skipped check shrinks the set and hides an unmet obligation","trigger":"an obligation is absent because its producing check did not run","action":"treat a skipped upstream check as itself an unmet obligation"},
 {"rule":"PROVENANCE_SIGNING alone does not discharge the statutory clear-and-distinguishable disclosure where the destination cannot verify it","rationale":"provenance is necessary but strippable and not always verifiable downstream","trigger":"a signed manifest relied on at a destination that cannot verify it","action":"add a human-perceptible label in addition to provenance"},
]

ARR=[
 {"rule":"Do not build the in-band token before the labeling superset is reconciled; a late regime adds a label the token must re-carry","prevents":"re-signing and re-binding the token after a missed jurisdiction's label surfaces"},
 {"rule":"Do not finalize the obligation set before jurisdiction resolution; a late jurisdiction invalidates the whole set","prevents":"re-aggregating every obligation after the jurisdiction scope changes"},
 {"rule":"Do not bind a downstream slot without verifying it propagates the token; an unverified slot silently drops the disclosure","prevents":"re-emitting after discovering the bound slot stripped the disclosure server-side"},
 {"rule":"Do not rely on a single in-band carrier; retrofitting redundancy after a strip incident requires re-marking every asset","prevents":"emergency re-watermarking of already-published assets after a stripping incident"},
 {"rule":"Do not run the AI-disclosure path while skipping the scrape-license gate; an unlicensed republish surfaces as a takedown later","prevents":"post-publication takedown and re-clearance of unlicensed redistributed media"},
 {"rule":"Do not classify embeds as reference-only without checking byte-re-hosting; mis-classification leaks an ungated redistribution","prevents":"re-screening embeds after an ungated republish is discovered"},
 {"rule":"Do not let the SPOF gate default to allow on internal errors; a single fail-open publishes an unlabeled emit","prevents":"incident response and recall after an unlabeled synthetic emit leaks through a fail-open gate"},
 {"rule":"Do not cache platform ToS indefinitely; a stale snapshot permits a now-prohibited redistribution","prevents":"re-clearing emits after a ToS change invalidates a cached redistribution permission"},
]

IP=[
 {"trigger":"an in-band token is observed stripped at a downstream hop","action":"strengthen redundant carriers in IN_BAND_DISCLOSURE_TOKEN and re-verify survival in HANDOFF_GATE_VERIFICATION","nodes":["IN_BAND_DISCLOSURE_TOKEN","HANDOFF_GATE_VERIFICATION","DISCLOSURE_SLOT_BINDING"],"priority":"critical"},
 {"trigger":"the fail-closed gate is found to have failed open on any emit","action":"force default-to-block on uncertainty in FAIL_CLOSED_AT_HANDOFF and add health monitoring of the SPOF gate","nodes":["FAIL_CLOSED_AT_HANDOFF","HANDOFF_GATE_VERIFICATION"],"priority":"critical"},
 {"trigger":"a deepfake is mis-classified as authentic, suppressing a required label","action":"recalibrate the manipulation threshold in SYNTHETIC_MEDIA_CLASSIFY and escalate borderline cases","nodes":["SYNTHETIC_MEDIA_CLASSIFY","SYNTHETIC_MEDIA_DISCLOSURE","MANIPULATED_MEDIA_POLICY"],"priority":"high"},
 {"trigger":"an unlicensed scraped source is republished under a correct AI label","action":"enforce the independent redistribution gate in SCRAPE_LICENSE and REDISTRIBUTION_VS_REFERENCE","nodes":["SCRAPE_LICENSE","REDISTRIBUTION_VS_REFERENCE","FAIL_CLOSED_AT_HANDOFF"],"priority":"high"},
 {"trigger":"a prohibited election-period synthetic political emit passes the gate","action":"tighten election-window detection in ELECTION_INTEGRITY scoped by JURISDICTION_RESOLUTION","nodes":["ELECTION_INTEGRITY","JURISDICTION_RESOLUTION","AUDIENCE_CONTEXT"],"priority":"high"},
 {"trigger":"a reaching jurisdiction's required label is missing from the obligation set","action":"re-run JURISDICTION_RESOLUTION from actual reach and re-assemble LABELING_REGIMES","nodes":["JURISDICTION_RESOLUTION","LABELING_REGIMES","DISCLOSURE_OBLIGATION_SET"],"priority":"high"},
 {"trigger":"a minor audience is reached without the minor escalation","action":"tighten minor-likelihood signals in AUDIENCE_CONTEXT and apply AGE_PLATFORM_POLICY proportionately","nodes":["AUDIENCE_CONTEXT","AGE_PLATFORM_POLICY"],"priority":"medium"},
 {"trigger":"a stale platform ToS permits a now-prohibited redistribution","action":"refresh and date the snapshot in PLATFORM_TOS and treat staleness as an unmet obligation","nodes":["PLATFORM_TOS","SCRAPE_LICENSE"],"priority":"medium"},
]

spec = {
 "domain":"platform_tos__synthetic_disclosure",
 "domain_label":"Platform ToS & Synthetic-Media Disclosure Gate",
 "purpose":"convert_an_emit_action_into_required_in_band_synthetic_media_disclosure_tokens_and_enforce_platform_tos_failing_closed_at_the_downstream_handoff",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "scope is the disclosure/ToS gate over an emit-action at a downstream handoff; this is not legal advice",
   "disclosure is mandatory and must travel in-band so the downstream session cannot strip it",
   "the downstream handoff is a single point of failure that defaults to block on any unmet obligation or unverifiable token",
   "the gate never strips or downgrades a disclosure to allow an emit through",
 ],
 "exclusions":[
   "legal advice or definitive statutory interpretation (escalate to qualified counsel)",
   "right-of-publicity / likeness-and-voice consent screening (delegated to pubrights__likeness_voice)",
   "detection of whether content is synthetic from pixels alone (delegated to a detector); this gate consumes a generation-path classification",
   "operator execution of the allowed emit and downstream content moderation appeals",
 ],
 "source_description":"heuristic prior estimates for a synthetic-media disclosure and platform-ToS gate, grounded in the EU AI Act, the C2PA/Content Credentials provenance standard, major platform synthetic-media policies, and India's IT Rules and deepfake advisories; no supplied dataset",
 "source_citation":"EU Regulation (EU) 2024/1689 (AI Act) Article 50 transparency obligations for AI-generated/manipulated content and deep fakes; C2PA Technical Specification (Coalition for Content Provenance and Authenticity) and Content Credentials; Meta, YouTube and TikTok synthetic/manipulated/AI-generated content disclosure policies; India Information Technology (Intermediary Guidelines and Digital Media Ethics Code) Rules 2021 and MeitY 2023 deepfake advisories",
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
 "priority_rationale":"EMIT_INTAKE/PLATFORM_TOS/SYNTHETIC_MEDIA_CLASSIFY are foundational; the regime and provenance nodes build the obligation superset; the in-band token and slot binding make disclosure survive; FAIL_CLOSED_AT_HANDOFF is the SPOF that blocks on any gap and HANDOFF_GATE_VERIFICATION closes the loop.",
 "eval_objective":"verify_synthetic_media_classification_statutory_and_platform_disclosure_in_band_token_survival_and_fail_closed_handoff_blocking_of_platform_tos__synthetic_disclosure_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "platform_tos__synthetic_disclosure.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
