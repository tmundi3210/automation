#!/usr/bin/env python3
"""Generate the defamation__false_light content spec (B60 KB) for kb_forge.py.

Domain: Defamation, False-Light & False-Association Screening for generated jokes
and derived scenes that name or depict REAL people.

Core scope finding (HARD REQUIREMENT, scope FP6): this KB is a fail-closed SCREENING
HEURISTIC that returns a pass / block / needs-review verdict and ESCALATES to counsel.
It never renders a legal determination. A joke or association passes only when it is
VISIBLY non-literal opinion / parody / hyperbole that no reasonable viewer would take
as an assertion of fact; anything that reads as a factual claim about a real person
(especially fabricated wrongdoing or a link to real political news) fails closed and
is blocked or routed to human/legal review. Actual-malice, substantial-truth and
fair-comment LEGAL adjudication are delegated to compliance/counsel, never decided here.

Grounded in real, named literature (cited in source_citation):
  - New York Times Co. v. Sullivan, 376 U.S. 254 (1964) -- public officials/figures
    must prove "actual malice" (knowledge of falsity or reckless disregard for truth)
  - Hustler Magazine, Inc. v. Falwell, 485 U.S. 46 (1988) -- parody that "could not
    reasonably have been interpreted as stating actual facts" is protected
  - Milkovich v. Lorain Journal Co., 497 U.S. 1 (1990) -- no wholesale "opinion"
    exemption; the test is whether a statement implies a provably false factual assertion
  - Gertz v. Robert Welch, Inc., 418 U.S. 323 (1974) -- "there is no such thing as a
    false idea"; private vs public figure standards
  - Restatement (Second) of Torts s652E -- false light: publicity placing a person in a
    false light that is highly offensive to a reasonable person, with the requisite fault
  - Lanham Act s43(a) (15 U.S.C. s1125(a)) -- false endorsement / false association
  - India: IPC s499/s500 (defamation, with exceptions) superseded by the Bharatiya Nyaya
    Sanhita 2023 s356 -- defamation as both a criminal offence and a civil tort in India

Compact authoring: node() applies sane defaults so only domain content + base metric
magnitudes are specified per node."""
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
        "academic_fields": ["media_law", "content_intelligence"],
        "subfields": subfields or ["defamation_screening", "false_light_and_false_association"],
        "specialists": specialists or ["content_risk_screener"],
        "contradictors": contradictors or ["anything_goes_satire_advocate"],
        "inputs": inputs or ["candidate joke or derived scene", "named/depicted real entities", "claim verifiability tags"],
        "outputs": outputs or ["pass/block/needs-review verdict with rationale and escalation flag"],
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

# ---------------- intake / foundations ----------------
N.append(node("CLAIM_INTAKE", "candidate_and_real_entity_capture",
  "Capture the candidate joke or derived scene together with every real person, brand, or institution it names or visually depicts, plus the literal surface meaning a viewer would extract, so screening operates on an explicit identified-target inventory rather than on the author's intent.",
  "foundations", [], ["CQ_01"],
  b(0.9, 0.8, 0.82, 0.5, 0.84, 0.78, 0.6, 0.7, 0.42, 0.78, 0.82, 0.88, 0.7, 0.18, 0.5, [0.2, 0.5],
    "entity-recognition coverage or the definition of an identifiable real target changes"),
  ["named real persons", "depicted likenesses", "literal surface reading", "identifiable-target inventory"],
  ["fully fictional characters with no real referent", "legal substantial-similarity adjudication"],
  [pro("An explicit target inventory is the precondition for every downstream test: identification is the first defamation element and the gate that distinguishes a real-person risk from harmless fiction",
       "a parody scene naming a sitting senator and showing a recognizable caricature is flagged as identifying that senator")],
  [con("Over-identification flags incidental or coincidental name matches that no viewer would tie to the real person, inflating false positives",
       "a common surname shared with a celebrity flagged as if it depicts the celebrity")],
  ["a real target depicted via context/likeness but not named is missed", "intent treated as a substitute for the viewer's reading"],
  ["every real person named or recognizably depicted is in the inventory with the literal surface reading recorded"],
  ["entity recognition model updated", "a missed real-target identification is found in audit"],
  specialists=["content_risk_screener", "entity_resolution_analyst"]))

N.append(node("REASONABLE_VIEWER_TEST", "reasonable_viewer_interpretation_standard",
  "Establish the controlling interpretive lens: how a reasonable viewer of the actual audience and medium would understand the joke or scene, since defamation, false light, and the parody safe harbor all turn on audience interpretation, not author intent or a hyper-literal parse.",
  "foundations", ["CLAIM_INTAKE"], ["CQ_01", "CQ_02"],
  b(0.92, 0.8, 0.82, 0.6, 0.88, 0.85, 0.62, 0.64, 0.55, 0.74, 0.78, 0.9, 0.64, 0.2, 0.56, [0.24, 0.6],
    "case law or platform norms shift what a reasonable viewer is taken to understand"),
  ["audience and medium context", "reasonable-viewer reading", "literal vs understood meaning gap"],
  ["author's private intent", "idiosyncratic minority interpretations"],
  [pro("Anchoring on the reasonable viewer matches the legal standard across defamation, false light, and parody, so the screen approximates the test a court would apply",
       "Falwell turned on whether the parody could reasonably be read as stating actual facts about Falwell")],
  [con("'Reasonable viewer' is context-dependent and contested; the same item reads differently to a satire audience than to a news audience",
       "an Onion-style headline is obvious to its readers but reposted out of context looks factual")],
  ["a niche-audience reading assumed for a general audience", "literalism overrides obvious contextual non-literal cues"],
  ["the verdict cites how the reasonable viewer of the actual medium/audience would read the item, not the author's intent"],
  ["audience/medium of distribution changes", "a contextual-stripping repost incident is observed"],
  specialists=["content_risk_screener", "media_law_analyst"]))

N.append(node("VERIFIABILITY_TAGS", "claim_verifiability_classification",
  "Tag every factual proposition embedded in or implied by the item by evidentiary status: VERIFIED, REPORTED, ALLEGED, or RUMOR. Only VERIFIED and REPORTED claims may inform factual framing; ALLEGED and RUMOR material must be rendered as visibly attributed opinion, never as the screen's own assertion of fact.",
  "foundations", ["CLAIM_INTAKE"], ["CQ_03"],
  b(0.88, 0.76, 0.74, 0.62, 0.86, 0.8, 0.62, 0.66, 0.5, 0.76, 0.8, 0.86, 0.66, 0.2, 0.55, [0.22, 0.55],
    "the verifiability taxonomy or sourcing policy changes"),
  ["VERIFIED/REPORTED/ALLEGED/RUMOR tagging", "implied-fact extraction", "sourcing provenance"],
  ["truth adjudication of contested facts", "investigative fact-checking"],
  [pro("Gating factual framing on verifiability tags operationalizes the truth defense: only well-sourced facts can ground a factual claim, and unsourced material is forced into the protected-opinion lane",
       "an ALLEGED corruption rumor is rendered as 'critics claim' opinion, not as 'X took a bribe'")],
  [con("Tag quality depends on the upstream source pipeline; a mis-tagged RUMOR as VERIFIED imports a false fact straight into factual framing",
       "a fabricated 'report' tagged REPORTED legitimizes a non-existent event")],
  ["RUMOR silently promoted to factual framing", "an implied factual claim left untagged"],
  ["no ALLEGED or RUMOR proposition appears as an unattributed assertion of fact in the output framing"],
  ["sourcing taxonomy revised", "a mis-tagged claim reaches factual framing"],
  specialists=["content_risk_screener", "fact_sourcing_analyst"]))

# ---------------- classification layer ----------------
N.append(node("OPINION_VS_FACT", "opinion_hyperbole_vs_provable_fact",
  "Classify each statement about a real target as protected pure opinion / rhetorical hyperbole or as an assertion (or implication) of objectively verifiable fact, applying Milkovich: there is no blanket opinion exemption; the dispositive question is whether the statement implies a provably false factual assertion.",
  "classification", ["REASONABLE_VIEWER_TEST", "VERIFIABILITY_TAGS"], ["CQ_02", "CQ_04"],
  b(0.92, 0.82, 0.8, 0.7, 0.9, 0.85, 0.66, 0.6, 0.6, 0.7, 0.74, 0.9, 0.6, 0.24, 0.6, [0.28, 0.66],
    "the opinion/fact dividing test or its precedent shifts"),
  ["pure opinion vs implied fact", "hyperbole and loose figurative language", "provable-falsity test"],
  ["fair-comment legal adjudication", "truth-or-falsity fact-finding"],
  [pro("The Milkovich provable-implication test is sharper than a naive opinion label: it catches 'in my opinion X is a thief', which implies undisclosed defamatory facts and is actionable",
       "'I think the mayor embezzled funds' implies a verifiable embezzlement and is not protected as opinion")],
  [con("The boundary between figurative hyperbole and implied fact is genuinely fuzzy; aggressive classification can chill protected rhetorical exaggeration",
       "'the landlord is a snake' is non-actionable hyperbole, not a factual zoological claim")],
  ["implied-fact statements waved through as 'just opinion'", "obvious hyperbole mis-scored as a factual claim"],
  ["each statement about a real target is labeled opinion/hyperbole or implied-fact, with the provable-falsity rationale recorded"],
  ["Milkovich-line precedent revisited", "an implied-fact slips through as opinion in audit"],
  specialists=["content_risk_screener", "media_law_analyst"]))

N.append(node("SATIRE_PARODY_SAFE_HARBOR", "parody_no_reasonable_belief_safe_harbor",
  "Determine whether the item qualifies for the satire/parody safe harbor: per Hustler v. Falwell, parody is protected when it could not reasonably have been interpreted as stating actual facts. The safe harbor requires that the non-literal nature be evident to the reasonable viewer, not merely intended by the author.",
  "classification", ["REASONABLE_VIEWER_TEST", "OPINION_VS_FACT"], ["CQ_02", "CQ_05"],
  b(0.9, 0.82, 0.82, 0.68, 0.9, 0.85, 0.66, 0.6, 0.62, 0.7, 0.74, 0.9, 0.6, 0.24, 0.6, [0.28, 0.66],
    "the parody safe-harbor standard or its evidentness requirement changes"),
  ["no-reasonable-belief test", "evident non-literal cues (exaggeration, absurdity, labeling)", "target-of-the-joke analysis"],
  ["whether the parody is funny or in good taste", "legal fair-use of underlying works"],
  [pro("The safe harbor is the principled exit for genuine parody: an outrageous, self-evidently fictional scene about a public figure is protected even if offensive, exactly as in Falwell",
       "an absurd cartoon clearly framed as satire of a politician's policy is safe-harbored")],
  [con("The harbor collapses when satire is dry, deadpan, or stripped of context: 'Poe's law' cases read as literal and lose protection despite satirical intent",
       "a deadpan fake-quote that omits any satire signal can be taken as a real statement")],
  ["author-intended satire with no viewer-visible non-literal cue", "deadpan fabrication mistaken for safe-harbored parody"],
  ["safe harbor is granted only when explicit, viewer-visible non-literal cues make a factual reading unreasonable"],
  ["parody standard revisited", "a deadpan item is wrongly safe-harbored in audit"],
  specialists=["content_risk_screener", "satire_analyst"]))

N.append(node("DEFAMATION_ELEMENTS", "core_defamation_element_test",
  "Apply the core defamation test to each statement about a real target: (1) a false statement of fact, (2) published/communicated to a third party, (3) of and concerning an identifiable plaintiff, (4) causing reputational harm, with the requisite fault. A statement clears defamation risk only if at least one element provably fails.",
  "classification", ["OPINION_VS_FACT", "CLAIM_INTAKE"], ["CQ_04", "CQ_06"],
  b(0.94, 0.84, 0.82, 0.66, 0.92, 0.85, 0.68, 0.6, 0.6, 0.72, 0.76, 0.92, 0.6, 0.22, 0.62, [0.26, 0.62],
    "the elements of the defamation tort or fault standards change"),
  ["false-statement-of-fact element", "publication element", "of-and-concerning identification", "reputational harm"],
  ["damages quantification", "final legal liability determination"],
  [pro("Decomposing into elements makes the screen auditable: a statement that is pure opinion fails element (1) and is cleared on a recorded, specific ground, not a vibe",
       "a clearly figurative insult fails the false-statement-of-fact element and clears")],
  [con("Element analysis can give false comfort: an item may clear one jurisdiction's elements yet be actionable under another's stricter standard",
       "a statement non-defamatory in the US may breach India's broader defamation rules")],
  ["clearing on a single weak element while another element is plainly met", "harm element assumed absent for a per-se defamatory imputation"],
  ["each statement is scored against all four elements and clears only when at least one element provably fails, with the failing element named"],
  ["fault-standard precedent changes", "a per-se defamatory imputation cleared in audit"],
  specialists=["content_risk_screener", "media_law_analyst"]))

# ---------------- tort branches ----------------
N.append(node("FALSE_LIGHT", "false_light_offensive_false_impression",
  "Screen for the false-light tort per Restatement (Second) of Torts s652E: publicity that places a real person in a false light highly offensive to a reasonable person, with knowledge of or reckless disregard for the falsity. False light reaches misleading IMPRESSIONS that are not literally false statements, so it catches material that survives a strict defamation parse.",
  "torts", ["DEFAMATION_ELEMENTS"], ["CQ_06", "CQ_07"],
  b(0.88, 0.8, 0.78, 0.66, 0.88, 0.82, 0.64, 0.6, 0.58, 0.72, 0.76, 0.88, 0.6, 0.22, 0.58, [0.26, 0.62],
    "the false-light tort's elements or its offensiveness threshold change"),
  ["highly-offensive false impression", "misleading juxtaposition/implication", "fault for falsity"],
  ["jurisdictions that reject false light as duplicative of defamation", "privacy intrusion torts"],
  [pro("False light catches true-statement-but-false-impression cases defamation misses, e.g. a real photo placed beside an unrelated scandal headline implying involvement",
       "a real person's photo captioned to imply they attended an event they never did")],
  [con("Several US states reject false light as redundant of defamation, so screening it can over-flag where it provides no independent cause of action",
       "a state that has abolished false light treats the same facts only under defamation")],
  ["a misleading juxtaposition cleared because no statement is literally false", "offensiveness judged from the author's tolerance not a reasonable person's"],
  ["any highly-offensive false impression about a real person is flagged even when no statement is literally false"],
  ["a false-light-recognizing vs rejecting jurisdiction is targeted", "a misleading-impression item cleared in audit"],
  specialists=["content_risk_screener", "media_law_analyst"]))

N.append(node("FALSE_ASSOCIATION", "false_endorsement_and_affiliation",
  "Screen for false association / false endorsement: implying that a real person or brand endorses, sponsors, is affiliated with, or authored the content when they do not, grounded in Lanham Act s43(a). This is distinct from defamation: the harm is misappropriated commercial identity and consumer confusion, not reputational falsity.",
  "torts", ["DEFAMATION_ELEMENTS", "CLAIM_INTAKE"], ["CQ_07", "CQ_08"],
  b(0.84, 0.82, 0.78, 0.64, 0.84, 0.82, 0.62, 0.62, 0.55, 0.74, 0.78, 0.84, 0.62, 0.2, 0.54, [0.24, 0.58],
    "false-endorsement or right-of-publicity standards change"),
  ["implied endorsement/sponsorship", "affiliation confusion", "misappropriated identity/likeness"],
  ["nominative fair use of a mark", "trademark dilution adjudication"],
  [pro("Catching implied endorsement prevents a joke from reading as a real celebrity or brand backing the content, which is a confusion/right-of-publicity exposure separate from any reputational harm",
       "a generated scene showing a celebrity using a product implies a paid endorsement that never existed")],
  [con("Nominative reference to a brand or person for commentary or parody is often lawful; over-flagging suppresses legitimate critical or comparative speech",
       "naming a brand to criticize it is nominative fair use, not false association")],
  ["nominative parody flagged as endorsement", "implied affiliation cleared because no explicit endorsement words appear"],
  ["any implication that a real person/brand endorses or is affiliated with the content is flagged unless an explicit disclaimer removes the confusion"],
  ["right-of-publicity or Lanham standard changes", "an implied-endorsement item cleared in audit"],
  specialists=["content_risk_screener", "brand_safety_analyst"]))

N.append(node("FABRICATED_WRONGDOING", "no_invented_acts_by_real_people",
  "Enforce the hard prohibition that the item must never depict a real, identifiable person committing acts they did not actually commit -- crimes, professional misconduct, abuse, or other wrongdoing -- because invented wrongdoing is a paradigm false statement of fact and false light, independent of comedic framing.",
  "torts", ["DEFAMATION_ELEMENTS", "VERIFIABILITY_TAGS"], ["CQ_09"],
  b(0.95, 0.84, 0.82, 0.6, 0.95, 0.85, 0.78, 0.6, 0.62, 0.7, 0.76, 0.94, 0.6, 0.2, 0.66, [0.26, 0.62],
    "the fabricated-wrongdoing prohibition or its narrow parody exception changes"),
  ["invented crimes/misconduct by a real person", "fabricated quotes confessing wrongdoing", "doctored evidence of acts"],
  ["depicting genuinely VERIFIED wrongdoing already on the public record", "fully fictional characters"],
  [pro("A bright-line block on fabricated wrongdoing eliminates the highest-severity defamation exposure, since invented criminal acts are defamatory per se and rarely survive any safe harbor",
       "a scene showing a named executive accepting a bribe that never happened is blocked outright")],
  [con("An absolute rule can over-block self-evident absurdist parody where no reasonable viewer believes the act occurred, conflicting with the Falwell safe harbor",
       "an obviously cartoonish scene of a politician fighting a dragon is not a factual wrongdoing claim")],
  ["fabricated misconduct passed under a comedic label", "doctored 'evidence' treated as verified"],
  ["no real person is depicted committing a non-VERIFIED act unless a self-evident parody cue makes a factual reading unreasonable and it is escalated for review"],
  ["the narrow parody carve-out is invoked", "a fabricated-wrongdoing item cleared in audit"],
  specialists=["content_risk_screener", "media_law_analyst"]))

# ---------------- fault / context modifiers ----------------
N.append(node("PUBLIC_FIGURE_STATUS", "plaintiff_status_classification",
  "Classify each real target as a public official, all-purpose public figure, limited-purpose public figure, or private individual, because the applicable fault standard and the breathing space for commentary depend on this status (Gertz), with private individuals entitled to greater protection.",
  "modifiers", ["CLAIM_INTAKE"], ["CQ_10"],
  b(0.84, 0.76, 0.72, 0.6, 0.82, 0.8, 0.58, 0.66, 0.5, 0.76, 0.78, 0.84, 0.66, 0.2, 0.5, [0.22, 0.54],
    "the public-figure classification doctrine changes"),
  ["public official/figure vs private individual", "limited-purpose figure for a controversy", "applicable fault tier"],
  ["the actual-malice evidentiary determination", "newsworthiness adjudication"],
  [pro("Status classification is the dial that sets the risk tier: a private individual gets the strictest protection, so misclassifying them as public would dangerously relax the screen",
       "a private citizen briefly in the news is not an all-purpose public figure and keeps strong protection")],
  [con("Public-figure status is notoriously fact-bound and contested; rigid classification can misallocate breathing space for borderline figures",
       "a limited-purpose figure is public only for the specific controversy, not for unrelated private matters")],
  ["a private individual misclassified as a public figure", "limited-purpose status over-extended beyond its controversy"],
  ["each target carries an explicit status classification that sets the applicable fault tier, defaulting to the protective private-individual tier when uncertain"],
  ["public-figure doctrine revisited", "a private individual treated as public in audit"],
  specialists=["content_risk_screener", "media_law_analyst"]))

N.append(node("ACTUAL_MALICE", "actual_malice_standard_for_public_figures",
  "Apply the New York Times v. Sullivan actual-malice standard for public officials and public figures: liability for a false factual statement requires knowledge of falsity or reckless disregard for the truth. The screen treats actual malice as a risk modifier, not a clearance: it never asserts a public figure's malice finding, only flags when content would require one.",
  "modifiers", ["PUBLIC_FIGURE_STATUS", "OPINION_VS_FACT"], ["CQ_10", "CQ_11"],
  b(0.86, 0.78, 0.74, 0.7, 0.86, 0.82, 0.62, 0.6, 0.58, 0.72, 0.74, 0.86, 0.6, 0.22, 0.56, [0.26, 0.62],
    "the actual-malice doctrine or its application to AI-generated content changes"),
  ["knowing-falsity vs reckless-disregard framing", "fault tier for public targets", "malice as a modifier not a clearance"],
  ["the in-court malice evidentiary finding", "discovery of the publisher's state of mind"],
  [pro("Treating actual malice as a flag (not a defense the screen can grant) keeps the screen fail-closed: it cannot clear a fabricated public-figure fact by assuming the speaker lacked malice",
       "a knowingly false 'quote' from a senator is flagged because publishing it would be reckless as to truth")],
  [con("Actual malice depends on subjective state of mind the screen cannot observe, so it can only approximate the standard from the content's evident disregard for truth",
       "an item may be reckless on its face yet the publisher's actual knowledge is unknowable at screen time")],
  ["assuming absence of malice to clear a public-figure fabrication", "applying the public-figure standard to a private individual"],
  ["public-figure factual claims that would require a malice finding are flagged and never cleared by assuming malice is absent"],
  ["actual-malice doctrine revisited", "a public-figure fabrication cleared via assumed good faith in audit"],
  specialists=["content_risk_screener", "media_law_analyst"]))

N.append(node("POLITICAL_NEWS_RISK", "real_person_real_news_factual_implication",
  "Flag the specific hazard that linking a real, identifiable person to real, current political or news events can imply a false factual claim about their conduct or position, even inside a joke. Proximity to a real news frame strips comedic context and makes a fabricated detail read as reportage.",
  "modifiers", ["VERIFIABILITY_TAGS", "REASONABLE_VIEWER_TEST"], ["CQ_03", "CQ_12"],
  b(0.88, 0.8, 0.78, 0.66, 0.9, 0.85, 0.66, 0.58, 0.62, 0.7, 0.74, 0.88, 0.58, 0.24, 0.62, [0.3, 0.68],
    "the boundary between protected political satire and implied factual reportage shifts"),
  ["real-person-plus-real-news juxtaposition", "implied conduct/position claims", "context-stripping in news frames"],
  ["genuine political opinion and commentary on verified events", "editorial cartoons clearly marked as satire"],
  [pro("Isolating the real-person/real-news interaction catches the most deceptive failure mode: a fabricated quote about a verified event reads as a news report, not a joke",
       "a fake statement attributing a real bill's authorship to the wrong legislator implies a false fact about a real event")],
  [con("Political satire is the most protected speech; over-flagging real-news juxtapositions can suppress core commentary the First Amendment most shields",
       "exaggerated commentary on a politician's verified vote is protected opinion, not a false factual claim")],
  ["a fabricated detail blended into a real-news frame and read as reportage", "protected political commentary over-flagged as factual implication"],
  ["any real-person-to-real-news link that implies a non-VERIFIED factual claim about the person is flagged and routed to review"],
  ["a context-stripping news-frame incident is observed", "political-satire scope is contested in audit"],
  specialists=["content_risk_screener", "media_law_analyst"]))

# ---------------- non-literal framing (HARD REQUIREMENT, scope FP6) ----------------
N.append(node("NON_LITERAL_FRAMING", "visible_non_literal_opinion_parody_framing",
  "Enforce the scope-FP6 hard requirement: any joke or association touching a real person must be rendered as visibly non-literal opinion or parody and must never read as the screen's own assertion of fact. This node converts intent into evident, viewer-facing non-literal cues (explicit satire labeling, attributed-opinion phrasing, evident exaggeration) or else fails the item closed.",
  "framing", ["SATIRE_PARODY_SAFE_HARBOR", "OPINION_VS_FACT", "POLITICAL_NEWS_RISK"], ["CQ_05", "CQ_13"],
  b(0.95, 0.84, 0.84, 0.68, 0.94, 0.88, 0.72, 0.58, 0.65, 0.68, 0.74, 0.94, 0.58, 0.24, 0.66, [0.3, 0.68],
    "the required set of viewer-visible non-literal cues or the FP6 scope finding changes"),
  ["explicit satire/parody labeling", "attributed-opinion phrasing", "evident hyperbole cues", "fail-closed when cues absent"],
  ["author's private satirical intent without a viewer-facing cue", "legal sufficiency of the disclaimer"],
  [pro("Forcing visible non-literal cues is the operational core of the safe harbor: it guarantees the reasonable viewer cannot take the item as fact, which is exactly what Falwell requires",
       "rephrasing 'X is a crook' as the clearly attributed, exaggerated opinion 'critics joke that X couldn't run a lemonade stand' removes the factual reading")],
  [con("Cues can be cosmetic: a buried 'satire' tag does not cure an item whose body reads as a straight factual assertion, so cue presence must be weighed by salience",
       "a tiny footnote disclaimer under a realistic fake quote does not make the quote non-literal to a scrolling viewer")],
  ["a non-literal cue present but not salient to the reasonable viewer", "intent-only framing with no viewer-facing signal"],
  ["every real-person joke/association either carries a salient viewer-visible non-literal cue or is failed closed", "no output frames a non-VERIFIED claim as the screen's own assertion of fact"],
  ["the FP6 non-literal-framing scope is revised", "a cosmetic-cue item is found to read as factual in audit"],
  specialists=["content_risk_screener", "media_law_analyst"]))

# ---------------- jurisdiction ----------------
N.append(node("US_FIRST_AMENDMENT_FRAME", "us_first_amendment_breathing_space",
  "Apply the US constitutional frame: 'there is no such thing as a false idea' (Gertz), broad protection for opinion, hyperbole, and parody, and the heightened public-figure fault standard, defining the breathing space within which the US analysis grants clearance.",
  "jurisdiction", ["DEFAMATION_ELEMENTS", "ACTUAL_MALICE"], ["CQ_11", "CQ_14"],
  b(0.82, 0.74, 0.72, 0.64, 0.82, 0.8, 0.6, 0.64, 0.5, 0.76, 0.78, 0.82, 0.64, 0.2, 0.5, [0.22, 0.54],
    "US First Amendment defamation doctrine shifts"),
  ["protected opinion/hyperbole space", "public-figure fault gate", "no-false-idea principle"],
  ["non-US speech regimes", "the underlying truth-finding"],
  [pro("Encoding US breathing space prevents the screen from over-blocking core protected speech: genuine opinion and parody about public figures get the wide latitude US law affords",
       "robust political hyperbole about a senator is recognized as protected, not blocked")],
  [con("US-style breadth is not portable: applying US opinion protection abroad would clear items that are actionable in less speech-protective jurisdictions",
       "an opinion safe in the US may be criminal defamation in India")],
  ["US protections over-applied to a non-US audience", "public-figure latitude extended to a private individual"],
  ["the US analysis grants clearance only within the recognized First Amendment breathing space and flags where another jurisdiction would diverge"],
  ["First Amendment defamation precedent changes", "US protection mis-applied to a non-US distribution in audit"],
  specialists=["content_risk_screener", "media_law_analyst"]))

N.append(node("INDIA_DEFAMATION", "india_criminal_and_civil_defamation",
  "Apply the Indian frame where defamation is both a CRIMINAL offence and a civil tort: historically IPC s499/s500 (with its statutory exceptions for truth-in-public-good, fair comment, etc.), now carried into the Bharatiya Nyaya Sanhita 2023 (s356). India's standard is materially less speech-protective than the US, so items cleared under the US frame can still create criminal exposure for an Indian audience.",
  "jurisdiction", ["DEFAMATION_ELEMENTS"], ["CQ_14"],
  b(0.86, 0.78, 0.74, 0.64, 0.88, 0.82, 0.68, 0.6, 0.55, 0.74, 0.78, 0.86, 0.6, 0.22, 0.6, [0.26, 0.62],
    "Indian defamation statute or its judicial interpretation changes"),
  ["criminal defamation exposure", "civil defamation tort", "statutory exceptions (truth-in-public-good, fair comment)"],
  ["US-style actual-malice protection", "constitutional opinion breadth"],
  [pro("Modeling India's criminal+civil regime catches exposure the US frame misses entirely, since criminal liability and a narrower opinion defense raise the stakes of a borderline joke",
       "a satirical imputation safe under US opinion doctrine can trigger s356 criminal defamation in India")],
  [con("The Indian exceptions (truth in public good, fair comment on public conduct) are fact-intensive; a screen can over-block legitimate public-interest commentary",
       "fair comment on a public servant's official conduct is a recognized exception that should clear")],
  ["US clearance assumed to carry to India", "an Indian statutory exception ignored, over-blocking public-interest comment"],
  ["items for an Indian audience are screened against both criminal and civil defamation, and US-only clearances are flagged as jurisdiction-limited"],
  ["the Bharatiya Nyaya Sanhita defamation provision is amended", "an Indian-audience item cleared solely on US grounds in audit"],
  specialists=["content_risk_screener", "india_media_law_analyst"]))

# ---------------- aggregation / verdict / governance ----------------
N.append(node("RISK_AGGREGATION", "multi_tort_risk_aggregation",
  "Aggregate the per-tort and per-jurisdiction signals (defamation elements, false light, false association, fabricated wrongdoing, fault tier, jurisdiction) into a single screening risk profile, taking the most severe unresolved exposure rather than averaging, so a single high-severity hit cannot be diluted by low-risk dimensions.",
  "aggregation", ["FALSE_LIGHT", "FALSE_ASSOCIATION", "FABRICATED_WRONGDOING", "NON_LITERAL_FRAMING", "INDIA_DEFAMATION", "US_FIRST_AMENDMENT_FRAME"], ["CQ_13", "CQ_15"],
  b(0.92, 0.82, 0.8, 0.7, 0.92, 0.88, 0.7, 0.58, 0.62, 0.7, 0.74, 0.92, 0.58, 0.24, 0.64, [0.3, 0.68],
    "the aggregation policy or severity ordering changes"),
  ["max-severity combination", "per-tort signal fusion", "cross-jurisdiction worst-case"],
  ["the final pass/block decision threshold", "counsel's legal determination"],
  [pro("Max-severity aggregation is the safe combinator for a fail-closed screen: a fabricated-wrongdoing hit dominates regardless of how clean the other dimensions are",
       "an item that is clean on association but fabricates a crime aggregates to high risk, not medium")],
  [con("Pure max-severity can over-escalate when several borderline-low signals together are the real issue, missing additive risk patterns",
       "many small misleading touches that individually clear but cumulatively defame")],
  ["a high-severity hit averaged down by clean dimensions", "additive low-severity risk under-counted"],
  ["the aggregate equals at least the most severe unresolved per-dimension exposure and is never diluted below it"],
  ["severity ordering revised", "a diluted high-severity hit reaches a pass verdict in audit"],
  specialists=["content_risk_screener", "risk_aggregation_analyst"]))

N.append(node("SCREEN_VERDICT", "pass_block_needs_review_verdict",
  "Emit the screening verdict -- PASS, BLOCK, or NEEDS-REVIEW -- from the aggregated risk profile against fail-closed thresholds: PASS only when every real-person touch is visibly non-literal and no unresolved factual-claim exposure remains; BLOCK on a clear high-severity hit; NEEDS-REVIEW on any unresolved ambiguity.",
  "verdict", ["RISK_AGGREGATION"], ["CQ_15", "CQ_16"],
  b(0.95, 0.86, 0.86, 0.62, 0.95, 0.85, 0.78, 0.6, 0.6, 0.72, 0.78, 0.95, 0.6, 0.2, 0.68, [0.26, 0.6],
    "the verdict thresholds or the pass/block/review label semantics change"),
  ["pass/block/needs-review decision", "fail-closed thresholding", "verdict rationale emission"],
  ["the legal liability determination", "remediation/rewrite authoring"],
  [pro("A three-way verdict with a needs-review lane prevents binary over- or under-blocking: genuinely ambiguous items go to humans instead of being force-decided",
       "a borderline deadpan parody is routed to NEEDS-REVIEW rather than auto-passed")],
  [con("Thresholds are heuristic, not legal lines; a mis-set threshold can either flood review or pass real exposure",
       "a too-lax PASS threshold lets a subtle implied-fact item through")],
  ["an ambiguous item force-decided as PASS instead of NEEDS-REVIEW", "thresholds tuned for throughput over safety"],
  ["the verdict is PASS only when no unresolved factual-claim or fabricated-wrongdoing exposure remains; all ambiguity yields NEEDS-REVIEW or BLOCK"],
  ["verdict thresholds are retuned", "a passed item is later found to assert a fact about a real person"],
  specialists=["content_risk_screener", "media_law_analyst"]))

N.append(node("FAIL_CLOSED_DEFAULT", "fail_closed_default_on_uncertainty",
  "Enforce the fail-closed invariant: whenever evidence is missing, a tag is ambiguous, a jurisdiction is unresolved, or any screening node abstains, the system defaults to the more restrictive outcome (BLOCK or NEEDS-REVIEW), never to PASS. Silence or uncertainty is treated as risk, not as clearance.",
  "verdict", ["SCREEN_VERDICT"], ["CQ_16", "CQ_17"],
  b(0.94, 0.82, 0.82, 0.56, 0.94, 0.82, 0.8, 0.62, 0.55, 0.74, 0.8, 0.92, 0.62, 0.18, 0.68, [0.22, 0.54],
    "the fail-closed default policy or its abstention triggers change"),
  ["restrictive default on missing evidence", "abstention-as-risk handling", "uncertainty routed to review"],
  ["optimistic defaults", "throughput-maximizing auto-pass"],
  [pro("Fail-closed makes the screen's errors safe-by-construction: the costly direction (passing a defamatory item) is structurally disfavored over the cheap one (an extra review)",
       "an item with an unresolved verifiability tag defaults to NEEDS-REVIEW rather than PASS")],
  [con("Aggressive fail-closed inflates review volume and can frustrate creators when benign items are repeatedly held",
       "a clearly fictional scene with one ambiguous tag is needlessly routed to review")],
  ["an abstaining node silently treated as a pass", "uncertainty resolved optimistically to clear an item"],
  ["every abstention, missing tag, or unresolved jurisdiction yields BLOCK or NEEDS-REVIEW, never PASS"],
  ["fail-closed policy is loosened for throughput", "an abstention is found to have produced a PASS in audit"],
  specialists=["content_risk_screener", "safety_policy_analyst"]))

N.append(node("COUNSEL_ESCALATION", "escalation_to_legal_counsel",
  "Route NEEDS-REVIEW and high-severity BLOCK items, and any item invoking a narrow parody carve-out over fabricated wrongdoing, to qualified legal counsel with a structured rationale package. The screen explicitly does not make legal determinations; counsel owns the liability call, and the screen owns flagging and packaging.",
  "governance", ["SCREEN_VERDICT", "FABRICATED_WRONGDOING", "INDIA_DEFAMATION"], ["CQ_17", "CQ_18"],
  b(0.9, 0.82, 0.82, 0.54, 0.92, 0.82, 0.72, 0.64, 0.55, 0.76, 0.8, 0.9, 0.64, 0.18, 0.62, [0.22, 0.54],
    "the escalation policy or counsel's intake contract changes"),
  ["structured rationale package", "review/counsel routing", "screen-vs-counsel responsibility boundary"],
  ["the legal liability determination itself", "litigation strategy"],
  [pro("A clean escalation boundary keeps the heuristic honest: it never pretends to give legal advice, and counsel receives a structured package rather than raw content",
       "a fabricated-wrongdoing item invoking a parody carve-out is escalated with elements and jurisdiction analysis attached")],
  [con("Escalation has cost and latency; over-escalating trivial items overloads counsel and slows the pipeline",
       "routing obvious pure-opinion items to counsel wastes scarce legal review")],
  ["a high-severity item resolved without counsel", "the screen rendering a de facto legal determination"],
  ["every needs-review and high-severity item reaches counsel with a structured rationale package, and the screen records that it made no legal determination"],
  ["the counsel intake contract changes", "a high-severity item is found resolved without escalation in audit"],
  specialists=["content_risk_screener", "legal_operations_analyst"]))

N.append(node("AUDIT_TRACE", "screening_decision_provenance",
  "Record the full provenance of each screening decision: the identified targets, verifiability tags, per-tort and per-jurisdiction signals, the aggregation, the verdict, and any escalation, so every PASS/BLOCK/NEEDS-REVIEW is explainable, reproducible, and reviewable after the fact.",
  "governance", ["SCREEN_VERDICT", "RISK_AGGREGATION"], ["CQ_18", "CQ_19"],
  b(0.8, 0.74, 0.76, 0.52, 0.78, 0.78, 0.6, 0.7, 0.42, 0.8, 0.82, 0.8, 0.7, 0.16, 0.46, [0.18, 0.46],
    "the audit/provenance schema changes"),
  ["per-decision provenance record", "signal-to-verdict trace", "reproducible rationale"],
  ["the verdict logic itself", "external case-management systems"],
  [pro("A complete audit trace turns the screen into an accountable system: a contested verdict can be reconstructed and a systematic error pattern can be found and fixed",
       "a creator dispute is resolved by replaying exactly which signals drove the BLOCK")],
  [con("Full provenance has storage and privacy cost, and logging the analyzed sensitive content itself can create its own exposure",
       "retaining drafted defamatory text indefinitely is its own liability")],
  ["a verdict with no reconstructable rationale", "provenance retaining sensitive content beyond need"],
  ["every verdict carries a reproducible trace of targets, tags, signals, aggregation, and escalation"],
  ["the provenance schema changes", "a verdict cannot be reconstructed in audit"],
  specialists=["content_risk_screener", "observability_analyst"]))

# ---------------- competency questions (<=14) ----------------
CQ = [
 ("CQ_01", "How are the candidate item, its identifiable real targets, and the controlling reasonable-viewer reading captured?", ["nodes", "glossary"], "CLAIM_INTAKE builds the target inventory and REASONABLE_VIEWER_TEST sets the interpretive lens", ["CLAIM_INTAKE", "REASONABLE_VIEWER_TEST"]),
 ("CQ_02", "By whose interpretation is meaning judged, and how is protected opinion/parody distinguished from an assertion of fact?", ["nodes"], "REASONABLE_VIEWER_TEST, OPINION_VS_FACT and SATIRE_PARODY_SAFE_HARBOR fix the reasonable-viewer and provable-fact tests", ["REASONABLE_VIEWER_TEST", "OPINION_VS_FACT", "SATIRE_PARODY_SAFE_HARBOR"]),
 ("CQ_03", "How does claim verifiability control whether material may be framed as fact or only as attributed opinion?", ["nodes"], "VERIFIABILITY_TAGS gates factual framing and POLITICAL_NEWS_RISK applies it to real-news juxtapositions", ["VERIFIABILITY_TAGS", "POLITICAL_NEWS_RISK"]),
 ("CQ_04", "What is the operative opinion-versus-fact test and how are the core defamation elements applied?", ["nodes"], "OPINION_VS_FACT applies the Milkovich provable-implication test feeding DEFAMATION_ELEMENTS", ["OPINION_VS_FACT", "DEFAMATION_ELEMENTS"]),
 ("CQ_05", "When does the satire/parody safe harbor apply, and how is non-literal framing made visible to the viewer?", ["nodes", "workflow"], "SATIRE_PARODY_SAFE_HARBOR sets the no-reasonable-belief test that NON_LITERAL_FRAMING operationalizes", ["SATIRE_PARODY_SAFE_HARBOR", "NON_LITERAL_FRAMING"]),
 ("CQ_06", "How are the four defamation elements and the distinct false-light tort screened?", ["nodes"], "DEFAMATION_ELEMENTS scores the elements and FALSE_LIGHT catches highly-offensive false impressions", ["DEFAMATION_ELEMENTS", "FALSE_LIGHT"]),
 ("CQ_07", "How is false-association/endorsement risk distinguished from defamation and false light, and detected?", ["nodes", "conflict_axes"], "FALSE_ASSOCIATION flags implied endorsement/affiliation as a distinct misappropriated-identity exposure", ["FALSE_ASSOCIATION", "FALSE_LIGHT"]),
 ("CQ_08", "How does the system prevent depicting a real person committing acts they did not commit?", ["nodes", "edge_cases"], "FABRICATED_WRONGDOING enforces the bright-line prohibition on invented wrongdoing", ["FABRICATED_WRONGDOING"]),
 ("CQ_09", "How does plaintiff status set the fault tier and engage the actual-malice standard within US breathing space?", ["nodes"], "PUBLIC_FIGURE_STATUS classifies the target, ACTUAL_MALICE applies Sullivan, and US_FIRST_AMENDMENT_FRAME bounds the space", ["PUBLIC_FIGURE_STATUS", "ACTUAL_MALICE", "US_FIRST_AMENDMENT_FRAME"]),
 ("CQ_10", "How is the hazard of linking a real person to real political/news events screened?", ["nodes", "edge_cases"], "POLITICAL_NEWS_RISK flags real-person-to-real-news factual implications", ["POLITICAL_NEWS_RISK"]),
 ("CQ_11", "How is the scope-FP6 requirement enforced that real-person jokes be visibly non-literal and never asserted as fact?", ["nodes", "workflow"], "NON_LITERAL_FRAMING forces salient visible non-literal cues or fails closed before RISK_AGGREGATION", ["NON_LITERAL_FRAMING", "RISK_AGGREGATION"]),
 ("CQ_12", "How are US and Indian (criminal+civil) defamation regimes applied and reconciled?", ["nodes"], "US_FIRST_AMENDMENT_FRAME and INDIA_DEFAMATION encode the two jurisdictional standards", ["US_FIRST_AMENDMENT_FRAME", "INDIA_DEFAMATION"]),
 ("CQ_13", "How are per-tort and per-jurisdiction signals aggregated into a fail-closed pass/block/needs-review verdict?", ["nodes", "workflow", "iteration_protocol"], "RISK_AGGREGATION fuses signals by max-severity, SCREEN_VERDICT emits the verdict, and FAIL_CLOSED_DEFAULT defaults uncertainty restrictively", ["RISK_AGGREGATION", "SCREEN_VERDICT", "FAIL_CLOSED_DEFAULT"]),
 ("CQ_14", "When are items escalated to counsel, and how is each decision made explainable and reproducible?", ["nodes", "dominance_rules"], "COUNSEL_ESCALATION routes review items to counsel without the screen ruling on liability and AUDIT_TRACE records full provenance", ["COUNSEL_ESCALATION", "AUDIT_TRACE"]),
]
CQS = [{"id": i, "question": q, "must_be_answerable_from": m, "acceptance_condition": a, "covered_by": c} for (i, q, m, a, c) in CQ]

# consolidate node->CQ references onto the 14-CQ set
CQ_MAP = {
 "CLAIM_INTAKE": ["CQ_01"], "REASONABLE_VIEWER_TEST": ["CQ_01", "CQ_02"], "VERIFIABILITY_TAGS": ["CQ_03"],
 "OPINION_VS_FACT": ["CQ_02", "CQ_04"], "SATIRE_PARODY_SAFE_HARBOR": ["CQ_02", "CQ_05"], "DEFAMATION_ELEMENTS": ["CQ_04", "CQ_06"],
 "FALSE_LIGHT": ["CQ_06", "CQ_07"], "FALSE_ASSOCIATION": ["CQ_07"], "FABRICATED_WRONGDOING": ["CQ_08"],
 "PUBLIC_FIGURE_STATUS": ["CQ_09"], "ACTUAL_MALICE": ["CQ_09"], "POLITICAL_NEWS_RISK": ["CQ_03", "CQ_10"],
 "NON_LITERAL_FRAMING": ["CQ_05", "CQ_11"], "US_FIRST_AMENDMENT_FRAME": ["CQ_09", "CQ_12"], "INDIA_DEFAMATION": ["CQ_12"],
 "RISK_AGGREGATION": ["CQ_11", "CQ_13"], "SCREEN_VERDICT": ["CQ_13"], "FAIL_CLOSED_DEFAULT": ["CQ_13"],
 "COUNSEL_ESCALATION": ["CQ_14"], "AUDIT_TRACE": ["CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

# ---------------- glossary ----------------
GL = [
 ("defamation", "a false statement of fact, communicated to a third party, that identifies and harms the reputation of a real person", ["libel", "slander"], ["protected_opinion"], ["DEFAMATION_ELEMENTS", "INDIA_DEFAMATION"]),
 ("false_light", "publicity placing a real person in a false light that is highly offensive to a reasonable person, even without a literally false statement", ["offensive_false_impression"], ["defamation"], ["FALSE_LIGHT"]),
 ("false_association", "implying that a real person or brand endorses, sponsors, or is affiliated with content when they are not", ["false_endorsement", "implied_affiliation"], ["nominative_fair_use"], ["FALSE_ASSOCIATION"]),
 ("actual_malice", "knowledge that a statement was false or reckless disregard for whether it was false, required for public officials and public figures", ["sullivan_standard"], ["negligence_standard"], ["ACTUAL_MALICE", "PUBLIC_FIGURE_STATUS"]),
 ("parody_safe_harbor", "the protection for parody that could not reasonably be interpreted as stating actual facts about a real person", ["no_reasonable_belief_defense"], ["deadpan_fabrication"], ["SATIRE_PARODY_SAFE_HARBOR", "NON_LITERAL_FRAMING"]),
 ("provable_falsity", "the Milkovich test: a statement is actionable if it implies an objectively verifiable, and provably false, factual assertion", ["implied_fact_test"], ["pure_opinion"], ["OPINION_VS_FACT"]),
 ("reasonable_viewer", "the hypothetical ordinary member of the actual audience whose interpretation, not the author's intent, governs the analysis", ["reasonable_reader", "ordinary_audience"], ["author_intent"], ["REASONABLE_VIEWER_TEST"]),
 ("verifiability_tag", "an evidentiary status label (VERIFIED, REPORTED, ALLEGED, RUMOR) controlling whether a claim may be framed as fact", ["sourcing_tag"], ["unsourced_assertion"], ["VERIFIABILITY_TAGS"]),
 ("fail_closed", "the policy that uncertainty, abstention, or missing evidence defaults to the restrictive outcome (block or review), never to pass", ["safe_default", "deny_by_default"], ["fail_open"], ["FAIL_CLOSED_DEFAULT", "SCREEN_VERDICT"]),
 ("screen_verdict", "the screening output: PASS, BLOCK, or NEEDS-REVIEW, distinct from any legal liability determination", ["screening_decision"], ["legal_ruling"], ["SCREEN_VERDICT", "COUNSEL_ESCALATION"]),
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
              "risk_of_conflict": "unmanaged tension degrades screening quality", "example": "see resolution_rule"})
def rel(f, t, et, rs, cc=0.7, cp=0.2, erc=0.3, why=""):
    E.append({"from": f, "to": t, "edge_type": et, "relation_strength": rs, "signed_tension": 0.0,
              "causal_confidence": cc, "conflict_probability": cp, "expected_rework_cost": erc,
              "why_related": why or f"{f} {et} {t}", "benefit_of_coupling": "coordinated behavior",
              "risk_of_conflict": "inconsistency if uncoordinated", "example": f"{f}/{t} {et} relation"})

# dependency edges (acyclic; the primary DAG spine of node.dependencies)
dep("CLAIM_INTAKE", "REASONABLE_VIEWER_TEST", 0.9)
dep("CLAIM_INTAKE", "VERIFIABILITY_TAGS", 0.86)
dep("REASONABLE_VIEWER_TEST", "OPINION_VS_FACT", 0.86)
dep("VERIFIABILITY_TAGS", "OPINION_VS_FACT", 0.8)
dep("OPINION_VS_FACT", "SATIRE_PARODY_SAFE_HARBOR", 0.8)
dep("OPINION_VS_FACT", "DEFAMATION_ELEMENTS", 0.86)
dep("DEFAMATION_ELEMENTS", "FALSE_LIGHT", 0.84)
dep("DEFAMATION_ELEMENTS", "FALSE_ASSOCIATION", 0.8)
dep("DEFAMATION_ELEMENTS", "FABRICATED_WRONGDOING", 0.84)
dep("CLAIM_INTAKE", "PUBLIC_FIGURE_STATUS", 0.8)
dep("PUBLIC_FIGURE_STATUS", "ACTUAL_MALICE", 0.84)
dep("VERIFIABILITY_TAGS", "POLITICAL_NEWS_RISK", 0.82)
dep("SATIRE_PARODY_SAFE_HARBOR", "NON_LITERAL_FRAMING", 0.86)
dep("POLITICAL_NEWS_RISK", "NON_LITERAL_FRAMING", 0.76)
dep("ACTUAL_MALICE", "US_FIRST_AMENDMENT_FRAME", 0.78)
dep("DEFAMATION_ELEMENTS", "INDIA_DEFAMATION", 0.8)
dep("FABRICATED_WRONGDOING", "RISK_AGGREGATION", 0.86)
dep("NON_LITERAL_FRAMING", "RISK_AGGREGATION", 0.86)
dep("INDIA_DEFAMATION", "RISK_AGGREGATION", 0.78)
dep("US_FIRST_AMENDMENT_FRAME", "RISK_AGGREGATION", 0.78)
dep("RISK_AGGREGATION", "SCREEN_VERDICT", 0.88)
dep("SCREEN_VERDICT", "FAIL_CLOSED_DEFAULT", 0.84)
dep("SCREEN_VERDICT", "COUNSEL_ESCALATION", 0.82)
dep("SCREEN_VERDICT", "AUDIT_TRACE", 0.78)

# cross-cutting non-dependency edges
rel("FAIL_CLOSED_DEFAULT", "SCREEN_VERDICT", "feedback", 0.8, why="the fail-closed default feeds back to flip an uncertain PASS into NEEDS-REVIEW or BLOCK")
rel("PUBLIC_FIGURE_STATUS", "DEFAMATION_ELEMENTS", "constraint", 0.78, why="plaintiff status sets the fault standard the element analysis must apply")
rel("POLITICAL_NEWS_RISK", "FABRICATED_WRONGDOING", "causal", 0.78, why="a real-news frame turns a fabricated detail into an implied factual wrongdoing claim")
rel("NON_LITERAL_FRAMING", "FABRICATED_WRONGDOING", "constraint", 0.8, why="the narrow parody carve-out for fabricated acts is admissible only with salient non-literal cues")
rel("REASONABLE_VIEWER_TEST", "SCREEN_VERDICT", "constraint", 0.74, why="the reasonable-viewer reading is the standard against which the verdict thresholds are set")
rel("VERIFIABILITY_TAGS", "DEFAMATION_ELEMENTS", "constraint", 0.76, why="only VERIFIED/REPORTED claims can satisfy the truth defense within the element analysis")
rel("AUDIT_TRACE", "COUNSEL_ESCALATION", "similarity", 0.72, why="the audit trace supplies the structured rationale package counsel receives")
rel("INDIA_DEFAMATION", "US_FIRST_AMENDMENT_FRAME", "similarity", 0.7, why="the two jurisdictions analyze the same elements but at materially different protection levels")

# conflict edges (negative signed_tension + resolution_rule)
conf("SATIRE_PARODY_SAFE_HARBOR", "FABRICATED_WRONGDOING", 0.72, -0.6,
  "the parody safe harbor never overrides the fabricated-wrongdoing block by default; the carve-out applies only when a self-evident, viewer-visible non-literal cue makes a factual reading unreasonable AND the item is escalated to counsel",
  "the parody safe harbor and the bright-line ban on fabricated wrongdoing pull in opposite directions on absurdist scenes")
conf("US_FIRST_AMENDMENT_FRAME", "INDIA_DEFAMATION", 0.7, -0.55,
  "when a US clearance conflicts with Indian exposure, the screen takes the more restrictive jurisdiction for the actual audience and flags the divergence rather than auto-passing on US grounds",
  "broad US opinion protection clears items that India's criminal+civil regime would treat as defamatory")
conf("OPINION_VS_FACT", "POLITICAL_NEWS_RISK", 0.68, -0.45,
  "protect genuine political opinion, but when a statement sits in a real-news frame and implies a non-VERIFIED fact, treat it as implied fact and route to review rather than clearing it as opinion",
  "core political opinion deserves wide latitude yet a real-news frame can convert exaggeration into an implied factual claim")
conf("FAIL_CLOSED_DEFAULT", "SCREEN_VERDICT", 0.66, -0.5,
  "throughput pressure to PASS borderline items yields to the fail-closed invariant: any unresolved factual-claim exposure defaults to NEEDS-REVIEW or BLOCK over a PASS",
  "the desire to pass more items for creator throughput conflicts with the fail-closed safety default")

# ---------------- conflict axes ----------------
CA = [
 {"name": "opinion_protection_vs_implied_fact", "description": "Broad protection for opinion and hyperbole preserves expressive freedom but can shelter statements that imply undisclosed defamatory facts.", "poles": ["protect_opinion", "catch_implied_fact"], "resolution_hint": "apply the Milkovich provable-implication test, not a blanket opinion label", "tension_score": 0.72, "affected_nodes": ["OPINION_VS_FACT", "DEFAMATION_ELEMENTS", "POLITICAL_NEWS_RISK"]},
 {"name": "parody_safe_harbor_vs_fabricated_wrongdoing", "description": "Parody protection shields outrageous fiction, but a bright-line ban on invented wrongdoing must not be dissolved by merely labeling content satire.", "poles": ["grant_safe_harbor", "block_fabrication"], "resolution_hint": "require salient viewer-visible non-literal cues and escalate carve-outs to counsel", "tension_score": 0.78, "affected_nodes": ["SATIRE_PARODY_SAFE_HARBOR", "FABRICATED_WRONGDOING", "NON_LITERAL_FRAMING"]},
 {"name": "author_intent_vs_reasonable_viewer", "description": "Satirical intent does not protect content the reasonable viewer reads as factual; the viewer's reading governs.", "poles": ["honor_intent", "apply_reasonable_viewer"], "resolution_hint": "decide on the reasonable viewer of the actual audience/medium, never on intent alone", "tension_score": 0.7, "affected_nodes": ["REASONABLE_VIEWER_TEST", "SATIRE_PARODY_SAFE_HARBOR", "NON_LITERAL_FRAMING"]},
 {"name": "us_breathing_space_vs_india_strictness", "description": "US First Amendment latitude clears speech that India's criminal+civil defamation regime would penalize.", "poles": ["us_broad_protection", "india_strict_standard"], "resolution_hint": "apply the more restrictive jurisdiction for the actual audience and flag divergence", "tension_score": 0.72, "affected_nodes": ["US_FIRST_AMENDMENT_FRAME", "INDIA_DEFAMATION", "RISK_AGGREGATION"]},
 {"name": "throughput_vs_fail_closed", "description": "Passing more borderline items pleases creators but conflicts with defaulting uncertainty to the restrictive outcome.", "poles": ["maximize_throughput", "fail_closed"], "resolution_hint": "any unresolved factual-claim exposure defaults to review/block, never pass", "tension_score": 0.68, "affected_nodes": ["FAIL_CLOSED_DEFAULT", "SCREEN_VERDICT", "RISK_AGGREGATION"]},
 {"name": "public_figure_latitude_vs_private_protection", "description": "Public figures bear a heightened actual-malice burden, but private individuals retain strong protection that must not be relaxed by misclassification.", "poles": ["public_figure_latitude", "private_protection"], "resolution_hint": "default to the protective private-individual tier when status is uncertain", "tension_score": 0.66, "affected_nodes": ["PUBLIC_FIGURE_STATUS", "ACTUAL_MALICE", "DEFAMATION_ELEMENTS"]},
 {"name": "defamation_vs_false_light_coverage", "description": "False light reaches misleading impressions defamation misses, but some jurisdictions reject it as duplicative, risking over-flagging.", "poles": ["broaden_via_false_light", "avoid_duplication"], "resolution_hint": "flag offensive false impressions but tag jurisdictions that reject false light", "tension_score": 0.6, "affected_nodes": ["FALSE_LIGHT", "DEFAMATION_ELEMENTS", "FALSE_ASSOCIATION"]},
 {"name": "nominative_reference_vs_false_endorsement", "description": "Naming a real person or brand for commentary is often lawful nominative use, yet the same reference can imply a false endorsement.", "poles": ["allow_nominative_use", "flag_false_endorsement"], "resolution_hint": "flag implied endorsement unless an explicit disclaimer removes confusion", "tension_score": 0.6, "affected_nodes": ["FALSE_ASSOCIATION", "CLAIM_INTAKE", "NON_LITERAL_FRAMING"]},
 {"name": "verifiable_fact_framing_vs_unsourced_rumor", "description": "Only VERIFIED/REPORTED claims may inform factual framing; rendering ALLEGED/RUMOR as fact imports false statements.", "poles": ["frame_verified_fact", "quarantine_rumor_as_opinion"], "resolution_hint": "force ALLEGED/RUMOR into attributed-opinion framing, never the screen's own fact", "tension_score": 0.64, "affected_nodes": ["VERIFIABILITY_TAGS", "OPINION_VS_FACT", "POLITICAL_NEWS_RISK"]},
]

# ---------------- edge cases ----------------
EC = [
 {"description": "A deadpan fabricated quote attributed to a real public figure reads as a genuine statement with no visible satire cue.", "trigger": "author-intended parody lacks a viewer-visible non-literal signal", "affected_nodes": ["SATIRE_PARODY_SAFE_HARBOR", "NON_LITERAL_FRAMING", "REASONABLE_VIEWER_TEST"], "mitigation": "deny the safe harbor without salient non-literal cues; fail closed to NEEDS-REVIEW", "severity": "critical"},
 {"description": "A joke depicts a named real executive committing a crime that never occurred.", "trigger": "fabricated wrongdoing about an identifiable real person", "affected_nodes": ["FABRICATED_WRONGDOING", "DEFAMATION_ELEMENTS", "RISK_AGGREGATION"], "mitigation": "block outright; escalate any parody carve-out to counsel", "severity": "critical"},
 {"description": "A fabricated detail is inserted into a real, current political news frame and reads as reportage.", "trigger": "real person linked to real news event implying a non-VERIFIED fact", "affected_nodes": ["POLITICAL_NEWS_RISK", "VERIFIABILITY_TAGS", "NON_LITERAL_FRAMING"], "mitigation": "tag the implied fact, force attributed-opinion framing, and route to review", "severity": "high"},
 {"description": "A generated scene shows a celebrity using a product, implying a paid endorsement that never existed.", "trigger": "implied endorsement/affiliation by a real person or brand", "affected_nodes": ["FALSE_ASSOCIATION", "CLAIM_INTAKE"], "mitigation": "flag false endorsement unless an explicit disclaimer removes consumer confusion", "severity": "high"},
 {"description": "A real photo of a person is captioned to imply they attended an event they never attended, with no literally false statement.", "trigger": "misleading juxtaposition creating a highly-offensive false impression", "affected_nodes": ["FALSE_LIGHT", "DEFAMATION_ELEMENTS"], "mitigation": "screen for false light independent of literal falsity; flag the offensive impression", "severity": "high"},
 {"description": "An item cleared under US opinion protection is distributed to an Indian audience where it risks criminal defamation.", "trigger": "US-only clearance applied to a non-US audience", "affected_nodes": ["US_FIRST_AMENDMENT_FRAME", "INDIA_DEFAMATION", "RISK_AGGREGATION"], "mitigation": "apply the more restrictive jurisdiction for the actual audience and flag the divergence", "severity": "high"},
 {"description": "A private individual briefly in the news is misclassified as a public figure, relaxing the fault standard.", "trigger": "plaintiff-status misclassification toward public-figure", "affected_nodes": ["PUBLIC_FIGURE_STATUS", "ACTUAL_MALICE"], "mitigation": "default to the protective private-individual tier when status is uncertain", "severity": "medium"},
 {"description": "A RUMOR-tagged claim is mis-promoted to VERIFIED and framed as the screen's own assertion of fact.", "trigger": "verifiability tag corruption upstream of factual framing", "affected_nodes": ["VERIFIABILITY_TAGS", "OPINION_VS_FACT", "DEFAMATION_ELEMENTS"], "mitigation": "treat unverifiable provenance as ALLEGED at most; force attributed-opinion framing", "severity": "high"},
 {"description": "An obviously cartoonish absurd scene about a politician is over-blocked as fabricated wrongdoing.", "trigger": "absolute fabrication rule applied to self-evident parody", "affected_nodes": ["FABRICATED_WRONGDOING", "SATIRE_PARODY_SAFE_HARBOR", "NON_LITERAL_FRAMING"], "mitigation": "permit the narrow carve-out only with salient cues and counsel escalation, not auto-pass", "severity": "medium"},
 {"description": "A buried 'satire' tag sits under a realistic fake quote that a scrolling viewer reads as factual.", "trigger": "non-literal cue present but not salient to the reasonable viewer", "affected_nodes": ["NON_LITERAL_FRAMING", "REASONABLE_VIEWER_TEST", "SCREEN_VERDICT"], "mitigation": "weight cues by salience; a non-salient disclaimer does not satisfy the safe harbor", "severity": "high"},
 {"description": "A screening node abstains on a missing tag and the item is silently passed.", "trigger": "node abstention treated as clearance instead of risk", "affected_nodes": ["FAIL_CLOSED_DEFAULT", "SCREEN_VERDICT", "RISK_AGGREGATION"], "mitigation": "treat every abstention as risk and default to NEEDS-REVIEW or BLOCK", "severity": "high"},
 {"description": "A contested BLOCK verdict cannot be reconstructed because the decision provenance was not recorded.", "trigger": "audit trace missing for a screening decision", "affected_nodes": ["AUDIT_TRACE", "SCREEN_VERDICT", "COUNSEL_ESCALATION"], "mitigation": "record targets, tags, signals, aggregation, verdict, and escalation for every decision", "severity": "medium"},
]

# ---------------- workflow ----------------
WF = [
 {"action": "capture_targets", "node_ref": "CLAIM_INTAKE", "description": "Capture the item, every named or depicted real target, and the literal surface reading into an explicit inventory.", "artifact": "identified_target_inventory", "gate": "every real person named or depicted is inventoried"},
 {"action": "set_viewer_lens", "node_ref": "REASONABLE_VIEWER_TEST", "description": "Fix how the reasonable viewer of the actual audience and medium would read the item.", "artifact": "reasonable_viewer_reading", "gate": "the controlling reading is recorded for the actual audience"},
 {"action": "tag_verifiability", "node_ref": "VERIFIABILITY_TAGS", "description": "Tag every embedded or implied factual claim as VERIFIED, REPORTED, ALLEGED, or RUMOR.", "artifact": "verifiability_tag_set", "gate": "no implied factual claim is left untagged"},
 {"action": "classify_opinion_vs_fact", "node_ref": "OPINION_VS_FACT", "description": "Label each statement as protected opinion/hyperbole or implied/asserted fact via the Milkovich test.", "artifact": "opinion_fact_labels", "gate": "each statement carries an opinion-or-implied-fact label"},
 {"action": "test_parody_safe_harbor", "node_ref": "SATIRE_PARODY_SAFE_HARBOR", "description": "Decide whether parody could not reasonably be read as stating actual facts.", "artifact": "safe_harbor_determination", "gate": "safe harbor granted only on viewer-visible non-literal cues"},
 {"action": "screen_defamation_elements", "node_ref": "DEFAMATION_ELEMENTS", "description": "Score each statement against the four defamation elements with the applicable fault standard.", "artifact": "element_scorecard", "gate": "clearance only when an element provably fails"},
 {"action": "screen_false_light_and_association", "node_ref": "FALSE_LIGHT", "description": "Screen for highly-offensive false impressions and implied endorsement/affiliation.", "artifact": "false_light_and_association_report", "gate": "offensive false impressions and implied endorsements are flagged"},
 {"action": "block_fabricated_wrongdoing", "node_ref": "FABRICATED_WRONGDOING", "description": "Detect and block any depiction of a real person committing non-VERIFIED acts.", "artifact": "fabrication_block_log", "gate": "no fabricated wrongdoing passes without a cued, escalated carve-out"},
 {"action": "enforce_non_literal_framing", "node_ref": "NON_LITERAL_FRAMING", "description": "Require salient viewer-visible non-literal cues for every real-person joke or fail it closed.", "artifact": "non_literal_framing_certificate", "gate": "every real-person touch is visibly non-literal or failed closed"},
 {"action": "apply_jurisdictions", "node_ref": "INDIA_DEFAMATION", "description": "Apply the US breathing-space and Indian criminal+civil frames and reconcile divergences.", "artifact": "jurisdiction_analysis", "gate": "the more restrictive jurisdiction governs the actual audience"},
 {"action": "aggregate_and_decide", "node_ref": "RISK_AGGREGATION", "description": "Fuse signals by max-severity and emit a fail-closed PASS/BLOCK/NEEDS-REVIEW verdict.", "artifact": "verdict_record", "gate": "the verdict is never diluted below the most severe unresolved exposure"},
 {"action": "escalate_and_audit", "node_ref": "COUNSEL_ESCALATION", "description": "Route review and high-severity items to counsel with a rationale package and record the full audit trace.", "artifact": "escalation_and_audit_record", "gate": "every review/high-severity item reaches counsel and is traced"},
]

# ---------------- dominance rules ----------------
DR = [
 {"rule": "CLAIM_INTAKE target identification and REASONABLE_VIEWER_TEST reading must be fixed before any tort screening runs", "rationale": "identification and the viewer lens are preconditions of every downstream element and safe-harbor test", "trigger": "a tort screen begins without an identified target or viewer reading", "action": "block until the target inventory and reasonable-viewer reading exist"},
 {"rule": "FABRICATED_WRONGDOING blocks dominate the parody safe harbor unless a salient non-literal cue is present and the item is escalated", "rationale": "invented wrongdoing is defamatory per se and the highest-severity exposure", "trigger": "a parody label is asserted over a fabricated-wrongdoing depiction", "action": "block and escalate the carve-out to counsel rather than auto-pass"},
 {"rule": "NON_LITERAL_FRAMING must certify salient viewer-visible non-literal cues before SCREEN_VERDICT can return PASS for a real-person item", "rationale": "scope-FP6 requires real-person jokes be visibly non-literal and never asserted as fact", "trigger": "a real-person item reaches verdict without a salient non-literal cue", "action": "fail the item closed to NEEDS-REVIEW or BLOCK"},
 {"rule": "FAIL_CLOSED_DEFAULT overrides any throughput-driven PASS when factual-claim exposure is unresolved", "rationale": "passing a defamatory item is far costlier than an extra review", "trigger": "an unresolved factual-claim exposure is about to PASS", "action": "default to NEEDS-REVIEW or BLOCK"},
 {"rule": "The more restrictive jurisdiction governs when US and Indian analyses diverge for the actual audience", "rationale": "a US clearance does not cure Indian criminal+civil exposure", "trigger": "a US PASS conflicts with Indian exposure for the audience", "action": "apply the Indian standard and flag the divergence"},
 {"rule": "VERIFIABILITY_TAGS gate factual framing: ALLEGED/RUMOR may never appear as the screen's own assertion of fact", "rationale": "unsourced material framed as fact imports a false statement", "trigger": "an ALLEGED/RUMOR claim is about to be framed as fact", "action": "force attributed-opinion framing"},
 {"rule": "PUBLIC_FIGURE_STATUS defaults to the protective private-individual tier when status is uncertain", "rationale": "misclassifying a private individual as public dangerously relaxes the fault standard", "trigger": "plaintiff status is indeterminate", "action": "apply the private-individual protection tier"},
 {"rule": "The screen never renders a legal determination; high-severity and needs-review items must reach COUNSEL_ESCALATION", "rationale": "liability is counsel's call, not the heuristic's", "trigger": "a high-severity item is about to be resolved without counsel", "action": "escalate to counsel with a structured rationale package"},
 {"rule": "AUDIT_TRACE must record a reproducible rationale for every PASS/BLOCK/NEEDS-REVIEW verdict", "rationale": "unreconstructable verdicts cannot be reviewed or corrected", "trigger": "a verdict is emitted without a recorded trace", "action": "block emission until the decision provenance is recorded"},
]

# ---------------- anti-rework rules ----------------
ARR = [
 {"rule": "Do not run tort screening before targets are identified and the viewer reading is set; re-running all tests after a late identification is costly", "prevents": "re-screening every tort after a missed real-target is discovered"},
 {"rule": "Do not grant the parody safe harbor on author intent alone; retrofitting visible cues after a factual-reading complaint requires re-authoring", "prevents": "emergency re-framing of a deadpan item already read as fact"},
 {"rule": "Do not pass a real-person item without a salient non-literal cue; adding framing after publication cannot un-publish the factual reading", "prevents": "post-publication scramble over an item that asserted a fact"},
 {"rule": "Do not clear an item on US grounds for a multi-jurisdiction audience; discovering Indian criminal exposure later forces a takedown", "prevents": "reactive takedown after Indian-audience exposure surfaces"},
 {"rule": "Do not frame ALLEGED/RUMOR material as fact; correcting an asserted false fact after distribution is a retraction, not an edit", "prevents": "issuing a retraction for an unsourced claim framed as fact"},
 {"rule": "Do not resolve a high-severity item without counsel; an after-the-fact legal review of a published item is litigation, not screening", "prevents": "litigating an item that should have been escalated pre-publication"},
 {"rule": "Do not emit a verdict without an audit trace; reconstructing a disputed decision without provenance requires re-screening", "prevents": "blind re-screening to defend a contested verdict"},
 {"rule": "Do not average per-tort signals; a high-severity hit diluted to a PASS forces a recall once the dominant risk surfaces", "prevents": "recalling content whose dominant risk was averaged away"},
]

# ---------------- iteration protocol ----------------
IP = [
 {"trigger": "a passed item is later found to assert a non-VERIFIED fact about a real person", "action": "tighten OPINION_VS_FACT and NON_LITERAL_FRAMING cue-salience thresholds and re-screen the affected cohort", "nodes": ["OPINION_VS_FACT", "NON_LITERAL_FRAMING", "SCREEN_VERDICT"], "priority": "critical"},
 {"trigger": "fabricated wrongdoing about a real person reaches publication", "action": "audit the FABRICATED_WRONGDOING block and the parody carve-out path and force counsel escalation", "nodes": ["FABRICATED_WRONGDOING", "SATIRE_PARODY_SAFE_HARBOR", "COUNSEL_ESCALATION"], "priority": "critical"},
 {"trigger": "a real-person/real-news item is read as reportage by the audience", "action": "tighten POLITICAL_NEWS_RISK detection and force attributed-opinion framing in NON_LITERAL_FRAMING", "nodes": ["POLITICAL_NEWS_RISK", "VERIFIABILITY_TAGS", "NON_LITERAL_FRAMING"], "priority": "high"},
 {"trigger": "an Indian-audience item cleared on US grounds creates exposure", "action": "make the more-restrictive-jurisdiction rule binding in RISK_AGGREGATION and re-run INDIA_DEFAMATION on the cohort", "nodes": ["INDIA_DEFAMATION", "US_FIRST_AMENDMENT_FRAME", "RISK_AGGREGATION"], "priority": "high"},
 {"trigger": "an implied endorsement passes as harmless nominative use", "action": "tighten FALSE_ASSOCIATION detection and require explicit disclaimers for brand/person references", "nodes": ["FALSE_ASSOCIATION", "CLAIM_INTAKE"], "priority": "medium"},
 {"trigger": "a node abstention produces a silent PASS", "action": "harden FAIL_CLOSED_DEFAULT so every abstention defaults to NEEDS-REVIEW or BLOCK", "nodes": ["FAIL_CLOSED_DEFAULT", "SCREEN_VERDICT"], "priority": "high"},
 {"trigger": "a contested verdict cannot be reconstructed", "action": "extend the AUDIT_TRACE schema to capture the missing signal-to-verdict path", "nodes": ["AUDIT_TRACE", "RISK_AGGREGATION"], "priority": "medium"},
 {"trigger": "review volume from fail-closed defaults overloads counsel", "action": "tune SCREEN_VERDICT thresholds for clear-opinion items without relaxing the fail-closed factual-claim invariant", "nodes": ["SCREEN_VERDICT", "COUNSEL_ESCALATION"], "priority": "medium"},
]

spec = {
 "domain": "defamation__false_light",
 "domain_label": "Defamation, False-Light & False-Association Screening",
 "purpose": "screen_jokes_and_derived_scenes_for_defamation_false_light_and_false_association_distinguishing_opinion_parody_from_assertions_of_fact_with_a_pass_block_review_verdict",
 "assumptions": [
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "this is a fail-closed screening heuristic, not a legal determination; liability calls are owned by counsel",
   "real-person identification and an upstream verifiability-tag pipeline are available as inputs",
   "the actual audience and medium of distribution are known so the reasonable-viewer and jurisdiction analyses apply",
 ],
 "exclusions": [
   "final legal liability and damages determinations (owned by qualified counsel)",
   "in-court actual-malice and substantial-truth fact-finding",
   "copyright substantial-similarity and fair-use adjudication (routed to compliance/counsel)",
   "investigative fact-checking and primary-source truth verification",
 ],
 "source_description": "heuristic prior estimates for defamation, false-light, and false-association screening of generated jokes and derived scenes about real people, informed by US First Amendment defamation doctrine, the Restatement (Second) of Torts, the Lanham Act, and Indian defamation law; no supplied dataset",
 "source_citation": "New York Times Co. v. Sullivan, 376 U.S. 254 (1964) (actual malice); Hustler Magazine, Inc. v. Falwell, 485 U.S. 46 (1988) (parody no-reasonable-belief); Milkovich v. Lorain Journal Co., 497 U.S. 1 (1990) (opinion vs provable fact); Gertz v. Robert Welch, Inc., 418 U.S. 323 (1974) (no false idea; public/private figures); Restatement (Second) of Torts s652E (false light); Lanham Act s43(a), 15 U.S.C. s1125(a) (false endorsement/association); India IPC s499/s500 and Bharatiya Nyaya Sanhita 2023 s356 (criminal+civil defamation)",
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
 "priority_rationale": "CLAIM_INTAKE, REASONABLE_VIEWER_TEST and VERIFIABILITY_TAGS are foundational; OPINION_VS_FACT, SATIRE_PARODY_SAFE_HARBOR and DEFAMATION_ELEMENTS classify; the tort branches and NON_LITERAL_FRAMING carry the highest-severity exposure; RISK_AGGREGATION, SCREEN_VERDICT, FAIL_CLOSED_DEFAULT, COUNSEL_ESCALATION and AUDIT_TRACE close the fail-closed verdict last.",
 "eval_objective": "verify_target_identification_opinion_vs_fact_classification_parody_safe_harbor_fabricated_wrongdoing_block_non_literal_framing_jurisdiction_reconciliation_and_fail_closed_verdict_of_defamation__false_light_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "defamation__false_light.spec.json")
open(path, "w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes", len(N), "edges", len(E), "CA", len(CA), "EC", len(EC), "WF", len(WF), "CQ", len(CQS), "DR", len(DR), "ARR", len(ARR), "IP", len(IP))
