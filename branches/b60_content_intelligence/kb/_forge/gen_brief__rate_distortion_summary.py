#!/usr/bin/env python3
"""Generate the brief__rate_distortion_summary content spec (B60 KB) for kb_forge.py.

Domain: producing a token-budgeted, machine-facing summary at a chosen rate-distortion
operating point that is DECISION-LOSSLESS over a fixed downstream field set. The scope
finding (FP2) it encodes: reframe the folk goal "make it lossless AND token-efficient"
as a rate-distortion / decision-lossless problem -- lossless w.r.t. the decisions a
fixed downstream consumer must make, NOT literal bit-losslessness, which is bounded
below in rate by the source entropy.

Compact authoring (mimics example_htn_gen.py): node()/b()/pro()/con()/dep()/conf()/rel()
helpers carry sane defaults so each call supplies only domain content + base metric
magnitudes. Grounded in: Shannon 1948 (rate-distortion / source coding theorem),
Rissanen 1978 (Minimum Description Length), Maynez et al. 2020 (faithfulness/hallucination
in abstractive summarization), Cover & Thomas Elements of Information Theory."""
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
        "academic_fields": ["information_theory", "natural_language_processing"],
        "subfields": subfields or ["rate_distortion_theory", "summarization"],
        "specialists": specialists or ["information_theorist", "summarization_engineer"],
        "contradictors": contradictors or ["literal_losslessness_advocate"],
        "inputs": inputs or ["source document", "downstream field set"],
        "outputs": outputs or ["dense machine-facing brief"],
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

# ---------------- foundations layer ----------------
N.append(node("FIELD_SET_SPEC", "fixed_downstream_field_set_specification",
  "Fix the exact set of fields/decisions a downstream consumer (a generator, classifier, or router) must make from the brief alone; this field set is the reference against which 'lossless' is defined and is the boundary condition for the whole rate-distortion problem.",
  "foundations", [], ["CQ_01"],
  b(0.92, 0.85, 0.8, 0.5, 0.86, 0.8, 0.66, 0.7, 0.42, 0.8, 0.84, 0.9, 0.7, 0.16, 0.5, [0.2, 0.5],
    "downstream consumer's required field set or decision contract changes"),
  ["enumerated downstream fields", "decision contract", "field criticality weights"],
  ["source-side entropy estimation", "grammar design"],
  [pro("Pinning the field set turns 'lossless' from an unbounded ideal into a finite, checkable obligation per field",
       "fields={priority, owner, due_date, blocking_dep} -> only these four must be reconstructable")],
  [con("A field set fixed too early can omit a decision the consumer silently needs, making the brief locally lossless but globally wrong",
       "consumer later needs 'risk_tier' that was never enumerated")],
  ["unenumerated decision relied on downstream", "field set conflated with source schema rather than decision needs"],
  ["every downstream decision maps to at least one enumerated field with a criticality weight"],
  ["downstream consumer contract changes", "a new decision field is requested"],
  inputs=["downstream consumer spec", "decision contract"],
  outputs=["fixed field set with weights"],
  specialists=["systems_analyst", "summarization_engineer"]))

N.append(node("SOURCE_ENTROPY_BOUND", "source_entropy_rate_floor",
  "Estimate the information content of the source restricted to the field set, establishing the rate floor: by Shannon's source coding theorem no lossless code can compress decision-relevant content below its entropy, so 'lossless AND arbitrarily short' is provably impossible.",
  "foundations", [], ["CQ_02"],
  b(0.88, 0.78, 0.7, 0.66, 0.82, 0.78, 0.62, 0.6, 0.5, 0.7, 0.74, 0.86, 0.6, 0.22, 0.5, [0.28, 0.62],
    "source distribution assumptions or field-restricted entropy estimate changes"),
  ["field-restricted entropy estimate", "rate floor (lower bound on tokens)", "compressibility headroom"],
  ["actual encoding", "human readability tuning"],
  [pro("An explicit entropy floor exposes the hard limit: tokens cannot go below the field-restricted entropy without distortion",
       "if the four decision fields carry ~30 bits, no zero-distortion brief is shorter than ~30 bits of code")],
  [con("Entropy of natural-language fields is hard to estimate; a wrong floor mis-prices the achievable budget",
       "treating a high-entropy free-text justification as if it were a low-entropy enum")],
  ["entropy underestimated -> impossible budget targeted", "source modeled as i.i.d. when it has strong dependencies"],
  ["the chosen token budget is consistent with (>=) the estimated field-restricted entropy floor"],
  ["source distribution shifts", "field-restricted entropy re-estimated materially higher"],
  inputs=["source document", "fixed field set"],
  outputs=["entropy floor estimate", "compressibility headroom"],
  specialists=["information_theorist"]))

N.append(node("MUST_PRESERVE_CLASSES", "decision_relevant_must_preserve_classes",
  "Partition source content into classes that MUST survive compression because they are load-bearing for at least one downstream field: identifiers, quantities, polarity/negation, deadlines, named entities, and explicit constraints whose loss flips a decision.",
  "selection", ["FIELD_SET_SPEC"], ["CQ_01", "CQ_03"],
  b(0.9, 0.82, 0.78, 0.6, 0.88, 0.82, 0.64, 0.66, 0.55, 0.76, 0.8, 0.9, 0.66, 0.18, 0.55, [0.22, 0.54],
    "field set changes or a content class is reclassified as decision-relevant"),
  ["must-preserve class taxonomy", "field-to-class mapping", "negation/quantity/identifier preservation"],
  ["droppable classification", "token costing"],
  [pro("Naming must-preserve classes makes preservation a typed, auditable obligation rather than an editorial hope",
       "negation and numeric quantities are tagged must-preserve so 'not approved' never compresses to 'approved'")],
  [con("Over-broad must-preserve sets defeat compression by protecting content no decision actually needs",
       "marking every proper noun must-preserve when only the assignee entity is decision-relevant")],
  ["a decision-flipping token (a negation, a unit) dropped as droppable", "must-preserve set so large no budget is feasible"],
  ["every must-preserve class traces to >=1 downstream field and survives the round-trip test"],
  ["field set changes", "a dropped token is found to have flipped a decision"],
  specialists=["summarization_engineer", "systems_analyst"]))

N.append(node("DROPPABLE_CLASSES", "redundant_low_value_droppable_classes",
  "Identify content classes that are redundant, inferable, or decision-irrelevant and therefore compressible or droppable: rhetorical framing, repeated restatements, hedging, examples that re-encode an already-stated fact, and stylistic prose.",
  "selection", ["FIELD_SET_SPEC", "MUST_PRESERVE_CLASSES"], ["CQ_03"],
  b(0.8, 0.74, 0.68, 0.56, 0.74, 0.74, 0.5, 0.66, 0.5, 0.78, 0.78, 0.8, 0.66, 0.2, 0.46, [0.22, 0.52],
    "droppable taxonomy or redundancy model changes"),
  ["redundancy detection", "inferable-content detection", "droppable taxonomy"],
  ["must-preserve protection", "grammar encoding"],
  [pro("Explicitly classing content as droppable is where almost all rate savings come from; redundancy is free to remove",
       "three paragraphs restating one due date collapse to a single date token")],
  [con("Aggressive dropping can remove content that is redundant in the source but disambiguating for the consumer",
       "an 'example' that is the only place a unit is stated gets dropped as illustrative")],
  ["inferable-but-not-actually-recoverable content dropped", "redundancy mis-detected across negation boundary"],
  ["dropped content is provably reconstructable or provably decision-irrelevant under the field set"],
  ["redundancy model changes", "a 'droppable' drop degrades a downstream decision"]))

# ---------------- rate-distortion core layer ----------------
N.append(node("DISTORTION_METRIC", "decision_indexed_distortion_metric",
  "Define the distortion measure d(source, brief) as decision error over the field set -- the fraction or weighted count of downstream fields the consumer would get wrong from the brief versus from the source -- NOT token edit distance or surface n-gram overlap.",
  "rd_core", ["FIELD_SET_SPEC", "MUST_PRESERVE_CLASSES"], ["CQ_04"],
  b(0.9, 0.8, 0.74, 0.7, 0.86, 0.84, 0.6, 0.6, 0.58, 0.72, 0.76, 0.88, 0.6, 0.22, 0.56, [0.28, 0.64],
    "distortion definition or field weighting changes"),
  ["decision-error distortion measure", "per-field weighting", "weighted aggregate distortion"],
  ["surface ROUGE-style overlap metrics", "human-preference scoring"],
  [pro("A decision-indexed distortion makes 'lossless' precise: zero distortion = the consumer makes every field decision identically from the brief",
       "distortion counts a flipped 'blocking=true->false' as full loss regardless of token overlap")],
  [con("Decision-error distortion needs a downstream oracle or proxy to evaluate, which can be expensive or noisy",
       "approximating consumer decisions with an LLM judge introduces its own error")],
  ["distortion proxied by surface overlap, rewarding faithful-looking but decision-wrong briefs", "field weights mis-set so a critical flip scores low"],
  ["distortion is computed from downstream field decisions, not surface overlap, and weights sum to a fixed total"],
  ["field weighting changes", "distortion metric found to reward decision-wrong output"],
  specialists=["information_theorist", "evaluation_engineer"]))

N.append(node("RATE_DISTORTION", "rate_distortion_tradeoff_curve",
  "Frame the core tension: the achievable region of (rate=tokens, distortion=decision error) pairs is governed by a rate-distortion function R(D); lower distortion costs more rate. 'Lossless AND minimal tokens' is not a point but a tradeoff bounded by R(0)>=source entropy on the field set.",
  "rd_core", ["SOURCE_ENTROPY_BOUND", "DISTORTION_METRIC"], ["CQ_02", "CQ_05"],
  b(0.95, 0.88, 0.82, 0.72, 0.9, 0.88, 0.72, 0.58, 0.62, 0.68, 0.74, 0.94, 0.58, 0.24, 0.6, [0.3, 0.66],
    "rate-distortion model or achievable-region assumptions change"),
  ["R(D) rate-distortion function", "achievable (rate,distortion) region", "FP2 reframing of lossless+efficient as a tradeoff"],
  ["the literal-losslessness framing", "operating-point selection (delegated)"],
  [pro("Casting the problem as R(D) dissolves the contradiction in 'lossless AND token-efficient': they trade off along a curve, and the right question is which point to pick",
       "at D=0 rate equals the entropy floor; relaxing to D=0.02 may halve the tokens")],
  [con("The true R(D) for natural-language field reconstruction is not closed-form; we work with an empirically traced approximation",
       "the curve is sampled at a few budgets, not derived analytically")],
  ["treating zero distortion as free (ignoring R(0) floor)", "assuming a single 'best' point exists independent of the consumer's loss tolerance"],
  ["the stated goal is expressed as a point on a rate-distortion tradeoff, never as simultaneous literal-lossless + minimal-token"],
  ["rate-distortion model assumptions change", "achievable region re-traced and shifts materially"],
  specialists=["information_theorist"],
  contradictors=["literal_losslessness_advocate", "pure_token_minimizer"]))

N.append(node("DECISION_LOSSLESS", "decision_lossless_criterion",
  "Define decision-losslessness: the brief is lossless w.r.t. a fixed downstream field set iff the consumer makes every field decision identically from the brief and from the full source (distortion=0 under the decision metric). This is strictly weaker than literal/bit-losslessness and is the property actually required.",
  "rd_core", ["RATE_DISTORTION", "DISTORTION_METRIC"], ["CQ_05", "CQ_06"],
  b(0.96, 0.9, 0.86, 0.66, 0.92, 0.9, 0.74, 0.6, 0.6, 0.74, 0.78, 0.95, 0.6, 0.2, 0.62, [0.26, 0.6],
    "definition of the downstream field set or the losslessness criterion changes"),
  ["decision-lossless predicate", "equivalence of brief-decisions and source-decisions", "scope FP2 finding"],
  ["literal bit-losslessness", "human-perceived completeness"],
  [pro("Decision-losslessness is achievable at far lower rate than literal losslessness because it only protects what changes a decision",
       "a 50-page contract compresses to a few decision fields while remaining decision-lossless for an approval router")],
  [con("Decision-losslessness is only as trustworthy as the field set; it is silent about decisions outside that set",
       "a brief decision-lossless for routing is useless to an auditor who needs the dropped clauses")],
  ["claiming losslessness without naming the field set", "field set narrower than the consumer's actual decisions"],
  ["for every field, the consumer's decision from the brief equals its decision from the full source on a held-out test set"],
  ["downstream field set changes", "a consumer decision diverges between brief and source"],
  specialists=["information_theorist", "systems_analyst"],
  contradictors=["literal_losslessness_advocate"]))

N.append(node("OPERATING_POINT", "rate_distortion_operating_point_selection",
  "Choose the operating point on R(D): the (token budget, tolerated distortion) pair to target, given the consumer's loss tolerance per field. Decision-critical fields demand the D=0 point; advisory fields may tolerate small distortion to buy rate.",
  "rd_core", ["RATE_DISTORTION", "DECISION_LOSSLESS"], ["CQ_05", "CQ_07"],
  b(0.88, 0.84, 0.78, 0.66, 0.86, 0.82, 0.66, 0.6, 0.58, 0.72, 0.76, 0.86, 0.6, 0.22, 0.55, [0.28, 0.62],
    "consumer loss tolerance or budget constraint changes"),
  ["operating-point selection", "per-field distortion tolerance", "budget-vs-fidelity tradeoff decision"],
  ["curve estimation (delegated)", "grammar encoding"],
  [pro("Selecting the point explicitly makes the lossless/efficient tradeoff a deliberate, documented decision rather than an accident of prompt length",
       "set D=0 for {amount, party, deadline}; allow D<=0.05 for {tone, background}")],
  [con("A single global operating point can over- or under-protect heterogeneous fields with different stakes",
       "one budget forces the same fidelity on a legal amount and a stylistic note")],
  ["operating point chosen without per-field tolerance", "point set below the entropy floor (infeasible at zero distortion)"],
  ["the selected operating point is feasible (rate>=entropy floor at the chosen distortion) and per-field tolerances are recorded"],
  ["consumer loss tolerance changes", "budget constraint tightened or relaxed"],
  specialists=["summarization_engineer", "systems_analyst"]))

# ---------------- budget / curve layer ----------------
N.append(node("TOKEN_BUDGET", "fixed_token_budget_and_cost_measurement",
  "Fix a token budget B for the brief and measure realized token cost against it with a concrete tokenizer; the budget is the rate axis made operational, and cost must be measured in the same tokenizer the downstream consumer uses.",
  "budget", ["OPERATING_POINT", "SOURCE_ENTROPY_BOUND"], ["CQ_07", "CQ_08"],
  b(0.84, 0.8, 0.76, 0.54, 0.8, 0.76, 0.55, 0.7, 0.5, 0.82, 0.82, 0.84, 0.7, 0.16, 0.46, [0.18, 0.46],
    "tokenizer or budget constraint B changes"),
  ["fixed budget B", "tokenizer-consistent cost measurement", "budget-overrun defeater"],
  ["entropy estimation", "fidelity verification"],
  [pro("A measured budget in the consumer's own tokenizer makes 'token-efficient' an enforced number, not an aspiration",
       "B=180 tokens measured with the same BPE the downstream model parses with")],
  [con("Budgets measured in the wrong tokenizer mislead: a brief 'within budget' in one tokenizer overflows another",
       "counting whitespace-split words while the consumer uses sub-word BPE")],
  ["cost measured in a different tokenizer than the consumer's", "budget set without reference to the entropy floor"],
  ["realized token cost <= B measured with the consumer's tokenizer, and B >= the entropy floor"],
  ["tokenizer changes", "budget constraint B revised"],
  inputs=["operating point", "consumer tokenizer"],
  outputs=["budget B", "measured token cost"],
  specialists=["summarization_engineer"]))

N.append(node("DISTORTION_CURVE", "empirical_distortion_vs_budget_curve",
  "Trace the empirical distortion-vs-budget curve by generating briefs at several budgets and measuring decision distortion at each, approximating R(D) and locating the knee where further compression starts flipping decisions.",
  "budget", ["TOKEN_BUDGET", "DISTORTION_METRIC"], ["CQ_08", "CQ_09"],
  b(0.82, 0.76, 0.72, 0.68, 0.78, 0.78, 0.55, 0.58, 0.55, 0.7, 0.74, 0.82, 0.58, 0.24, 0.5, [0.3, 0.66],
    "curve sampling protocol or distortion measurement changes"),
  ["budget sweep", "per-budget distortion measurement", "knee/operating-point identification"],
  ["analytic R(D) derivation", "final encoding"],
  [pro("An empirical curve replaces guesswork: it shows exactly how many tokens buy how much fidelity for THIS source and field set",
       "curve shows distortion is flat from 250 down to 180 tokens, then rises sharply -> pick ~180")],
  [con("The curve is only valid for the sampled source/consumer; it does not generalize to a different document or field set",
       "a curve traced on invoices misapplied to legal briefs")],
  ["curve sampled too coarsely to locate the knee", "distortion measured with a proxy that hides decision flips"],
  ["the curve has enough budget samples to locate the knee, and the chosen budget sits at or above it"],
  ["source or field set changes (curve invalidated)", "curve knee moves materially on re-sampling"],
  specialists=["evaluation_engineer", "information_theorist"]))

# ---------------- grammar / encoding layer ----------------
N.append(node("MACHINE_FACING", "machine_facing_optimization_target",
  "Commit to optimizing the brief for machine parsing rather than human readability: prose connectives, hewing to natural prosody, and narrative flow are dropped in favor of token-dense, position-stable, unambiguously parseable structure.",
  "encoding", ["DROPPABLE_CLASSES"], ["CQ_10"],
  b(0.84, 0.8, 0.74, 0.58, 0.78, 0.78, 0.6, 0.68, 0.5, 0.78, 0.8, 0.84, 0.68, 0.18, 0.48, [0.2, 0.5],
    "downstream consumer changes from machine to human or vice versa"),
  ["machine-parse optimization target", "removal of human-prosody overhead", "position-stable structure"],
  ["human-readability tuning", "stylistic polish"],
  [pro("Optimizing for the parser removes whole classes of tokens (articles, transitions, hedges) that cost rate but carry no decision information",
       "'The owner of the task is, as noted, Alice' -> 'owner=Alice'")],
  [con("Machine-facing briefs are hostile to human review, so errors are harder for a person to spot before consumption",
       "a flipped boolean in a pipe-delimited line is invisible at a glance")],
  ["machine-facing form so terse it becomes ambiguous to the parser", "human reviewer assumed where only a parser consumes"],
  ["the brief parses unambiguously under the declared grammar with no reliance on prose connectives"],
  ["consumer switches to a human reader", "parser ambiguity observed on the brief form"],
  specialists=["summarization_engineer", "prompt_engineer"],
  contradictors=["human_readability_advocate"]))

N.append(node("DENSE_GRAMMAR", "fixed_pipe_kv_dense_grammar",
  "Fix a dense, self-describing grammar -- a stable pipe/key-value line format with a leading schema_hint -- so the brief is unambiguously parseable and self-delimiting; the grammar is part of the code, paid for once, and amortized across every field.",
  "encoding", ["MACHINE_FACING", "MUST_PRESERVE_CLASSES"], ["CQ_10", "CQ_11"],
  b(0.88, 0.82, 0.78, 0.64, 0.84, 0.84, 0.7, 0.64, 0.56, 0.76, 0.8, 0.88, 0.64, 0.2, 0.55, [0.24, 0.58],
    "grammar specification or delimiter set changes"),
  ["pipe/KV line grammar", "leading schema_hint", "self-delimiting field encoding"],
  ["abbreviation expansion (delegated)", "distortion measurement"],
  [pro("A fixed grammar with a schema_hint lets the dense line self-describe, so the consumer parses it deterministically without a separate spec",
       "'#schema:owner|prio|due|blocks; owner=Alice|prio=P1|due=2026-07-01|blocks=SVC-9'")],
  [con("A rigid grammar can fail to encode an unexpected field shape, forcing either an escape hatch or silent loss",
       "a multi-valued field that the single-value KV slot cannot hold")],
  ["delimiter collision with field content (unescaped pipe in a value)", "grammar drift between producer and consumer versions"],
  ["the brief carries a schema_hint and every field parses unambiguously; delimiters in values are escaped"],
  ["grammar/delimiter set changes", "a parse failure traced to the grammar"],
  specialists=["summarization_engineer", "prompt_engineer"]))

N.append(node("SCHEMA_HINT", "self_describing_schema_hint",
  "Prepend a compact schema_hint that names the fields and their order/types so the dense line is self-describing; this is the one-time grammar cost that makes the rest of the line interpretable and version-checkable by the consumer.",
  "encoding", ["DENSE_GRAMMAR", "FIELD_SET_SPEC"], ["CQ_11"],
  b(0.8, 0.76, 0.74, 0.54, 0.74, 0.78, 0.58, 0.7, 0.48, 0.8, 0.8, 0.8, 0.7, 0.16, 0.44, [0.18, 0.46],
    "field set or schema versioning policy changes"),
  ["field-name/order/type declaration", "schema version tag", "self-description of the dense line"],
  ["value encoding", "round-trip evaluation"],
  [pro("A schema_hint makes the brief portable and version-checkable: the consumer validates the field set before parsing values",
       "'#v=2;fields=owner:str,prio:enum,due:date' lets the consumer reject a v1 parser")],
  [con("The schema_hint is pure overhead at very small budgets where its tokens rival the payload",
       "a 4-field hint costs as much as the 4 values it describes")],
  ["schema_hint omitted, leaving the consumer to guess field order", "schema version not bumped on a field change"],
  ["the schema_hint declares every field and a version tag, and the consumer validates it before parsing"],
  ["field set changes", "schema versioning policy changes"],
  inputs=["fixed field set", "grammar"],
  outputs=["schema_hint header"]))

N.append(node("ABBREVIATION_KEY", "expandable_abbreviation_key",
  "Maintain an expandable abbreviation/symbol key so high-frequency long tokens are coded to short symbols recoverable by the consumer; this is an MDL move -- shifting cost into a shared, amortized codebook to lower the message rate.",
  "encoding", ["DENSE_GRAMMAR"], ["CQ_11", "CQ_12"],
  b(0.76, 0.72, 0.68, 0.6, 0.72, 0.74, 0.5, 0.66, 0.5, 0.78, 0.78, 0.76, 0.66, 0.2, 0.44, [0.22, 0.5],
    "abbreviation codebook or sharing assumption changes"),
  ["symbol/abbreviation codebook", "expansion rules", "codebook-vs-message cost split"],
  ["schema declaration", "faithfulness tagging"],
  [pro("By MDL, moving repeated long strings into a shared key lowers total description length when the codebook is amortized over many briefs",
       "code 'organization'->'org', 'requirement'->'req' once, save tokens on every line")],
  [con("If the consumer lacks the key (or it drifts), abbreviations are unrecoverable and the brief becomes lossy",
       "a brief using an org-specific code shipped to a consumer without the codebook")],
  ["abbreviation collides with a literal value", "codebook drift between producer and consumer"],
  ["every abbreviation in the brief is expandable by the consumer's current key with no collisions"],
  ["codebook changes", "an unexpandable abbreviation reaches the consumer"],
  specialists=["summarization_engineer"]))

# ---------------- faithfulness / fidelity layer ----------------
N.append(node("FAITHFULNESS", "no_hallucinated_facts_faithfulness",
  "Enforce faithfulness: every fact in the brief must be entailed by the source -- no hallucinated, fabricated, or unsupported content. Following Maynez et al. 2020, distinguish extrinsic hallucination (content not in source) from intrinsic (content contradicting source); both are zero-tolerance for decision fields.",
  "fidelity", ["MUST_PRESERVE_CLASSES", "DISTORTION_METRIC"], ["CQ_13"],
  b(0.94, 0.86, 0.82, 0.66, 0.92, 0.84, 0.7, 0.6, 0.6, 0.72, 0.76, 0.92, 0.6, 0.22, 0.6, [0.28, 0.64],
    "faithfulness definition or entailment-checking method changes"),
  ["source-entailment requirement", "extrinsic vs intrinsic hallucination distinction", "zero-tolerance on decision fields"],
  ["fluency optimization", "human-readability"],
  [pro("Faithfulness keeps compression honest: a shorter brief is worthless if it invents a deadline or flips a polarity",
       "abstractive rewrite that introduces a date absent from the source is rejected as extrinsic hallucination")],
  [con("Strict entailment can reject valid sound inferences the consumer would accept, forcing verbose hedging",
       "an obvious aggregate the source implies but does not literally state is flagged unsupported")],
  ["abstractive paraphrase silently introduces an unsupported fact", "intrinsic hallucination flips a polarity (approved<->denied)"],
  ["every decision-field value in the brief is entailed by the source on a faithfulness check"],
  ["faithfulness criterion changes", "a hallucinated fact is found in a shipped brief"],
  specialists=["summarization_engineer", "evaluation_engineer"]))

N.append(node("VERIFIABILITY_TAGS", "verifiability_and_provenance_tags",
  "Attach verifiability tags (source span pointer, or an explicit 'inferred'/'derived' marker) to facts in the brief so the consumer can distinguish source-grounded values from derived ones and audit any field back to the source.",
  "fidelity", ["FAITHFULNESS"], ["CQ_13", "CQ_14"],
  b(0.8, 0.76, 0.74, 0.58, 0.78, 0.78, 0.55, 0.66, 0.5, 0.78, 0.8, 0.8, 0.66, 0.2, 0.46, [0.2, 0.5],
    "provenance tagging scheme or source addressing changes"),
  ["per-fact source-span pointers", "inferred/derived markers", "auditability to source"],
  ["entailment computation", "budget setting"],
  [pro("Verifiability tags let the consumer trust-but-verify and let an auditor trace any decision back to a source span cheaply",
       "owner=Alice[span:p2L4] vs risk=high[inferred] makes provenance explicit")],
  [con("Provenance tags add rate; at tight budgets they compete with the payload they describe",
       "a span pointer per field can double the token cost of a terse brief")],
  ["inferred value presented as if source-grounded", "span pointers stale after source revision"],
  ["every fact carries a provenance tag (source span or inferred marker) that resolves against the current source"],
  ["provenance scheme changes", "a span pointer fails to resolve"]))

N.append(node("ROUND_TRIP_TEST", "downstream_reconstruction_round_trip_test",
  "Run the round-trip test: hand the brief alone (no source) to the downstream consumer and check whether it reconstructs every field decision correctly. This is the operational definition of decision-losslessness and the primary acceptance gate.",
  "fidelity", ["DECISION_LOSSLESS", "FAITHFULNESS", "DENSE_GRAMMAR"], ["CQ_06", "CQ_14"],
  b(0.93, 0.86, 0.82, 0.64, 0.9, 0.86, 0.66, 0.62, 0.58, 0.74, 0.78, 0.92, 0.62, 0.2, 0.6, [0.26, 0.6],
    "round-trip protocol or consumer model changes"),
  ["brief-only reconstruction trial", "field-by-field decision comparison", "decision-lossless acceptance gate"],
  ["source-assisted evaluation", "stylistic scoring"],
  [pro("The round-trip test directly measures the property that matters: can the consumer act correctly from the brief alone",
       "feed only the dense line to the router; confirm it picks the same owner/priority as from the full ticket")],
  [con("The test is only as representative as the consumer model and the held-out cases used to run it",
       "passing on easy cases while failing on the rare decision-flipping edge case not in the sample")],
  ["round-trip run with the source still in context (leakage)", "test cases miss the decision-critical edge cases"],
  ["the consumer reconstructs every field decision from the brief alone, matching the source-based decision on the test set"],
  ["consumer model changes", "a round-trip failure is observed in production"],
  specialists=["evaluation_engineer", "summarization_engineer"]))

# ---------------- selection / operations layer ----------------
N.append(node("SALIENCE_RANKING", "decision_salience_ranking",
  "Rank source content units by decision salience -- expected reduction in downstream decision error if retained -- so that under a binding budget the highest-salience must-preserve content is encoded first and droppable content is shed last.",
  "selection", ["MUST_PRESERVE_CLASSES", "DROPPABLE_CLASSES"], ["CQ_03", "CQ_12"],
  b(0.82, 0.78, 0.74, 0.66, 0.8, 0.8, 0.55, 0.6, 0.55, 0.72, 0.76, 0.82, 0.6, 0.22, 0.5, [0.28, 0.6],
    "salience model or field weighting changes"),
  ["decision-salience scoring", "budget-aware retention order", "shed-order for droppable content"],
  ["grammar encoding", "round-trip evaluation"],
  [pro("Ranking by decision salience makes budget cuts principled: you lose the least decision-relevant content first",
       "a binding constraint clause outranks a background paragraph for retention")],
  [con("Salience estimates are noisy; a mis-ranked but decision-critical unit can be shed under a tight budget",
       "an easily-overlooked exception clause scored low and dropped")],
  ["salience proxied by frequency, demoting rare-but-critical clauses", "ranking ignores field weights from the operating point"],
  ["retained content is ordered by decision salience and no must-preserve unit is shed before any droppable unit"],
  ["salience model changes", "a high-salience unit is found to have been shed"]))

N.append(node("COMPRESSION_OPS", "lossy_and_lossless_compression_operators",
  "Apply the compression operators that move along the rate axis: lossless ones (deduplication, abbreviation, normalization to canonical form) and bounded-lossy ones (paraphrase, aggregation, quantization of low-stakes fields), each tagged by the distortion it may introduce.",
  "operations", ["SALIENCE_RANKING", "ABBREVIATION_KEY", "OPERATING_POINT"], ["CQ_12", "CQ_15"],
  b(0.84, 0.78, 0.72, 0.7, 0.82, 0.82, 0.58, 0.58, 0.58, 0.7, 0.74, 0.84, 0.58, 0.24, 0.54, [0.3, 0.66],
    "compression operator set or per-operator distortion budget changes"),
  ["lossless operators (dedup, normalize, abbreviate)", "bounded-lossy operators (paraphrase, aggregate, quantize)", "per-operator distortion accounting"],
  ["faithfulness checking (delegated)", "final round-trip"],
  [pro("Separating lossless from bounded-lossy operators lets you spend the distortion budget deliberately and only where the operating point allows",
       "normalize dates losslessly; quantize a 'confidence 0.73' to 'high' only for advisory fields")],
  [con("Composed lossy operators accumulate distortion that can exceed the per-field tolerance non-obviously",
       "paraphrase plus aggregation together flip a decision neither would alone")],
  ["lossy operator applied to a D=0 decision field", "accumulated distortion across operators exceeds the field tolerance"],
  ["each operator's distortion is accounted, and the summed distortion per field stays within its tolerance from the operating point"],
  ["operator set changes", "accumulated distortion exceeds a field tolerance"],
  specialists=["summarization_engineer"]))

# ---------------- verification / governance layer ----------------
N.append(node("COVERAGE_AUDIT", "field_set_coverage_and_overrun_audit",
  "Audit the final brief on both axes at once: field-set coverage (every required field is present and decision-lossless via the round-trip) and budget conformance (cost <= B). A pass requires both; either failure routes back to operating-point or compression.",
  "verification", ["ROUND_TRIP_TEST", "TOKEN_BUDGET", "DISTORTION_CURVE"], ["CQ_09", "CQ_16"],
  b(0.9, 0.84, 0.8, 0.62, 0.88, 0.84, 0.66, 0.64, 0.55, 0.78, 0.82, 0.9, 0.64, 0.18, 0.56, [0.22, 0.54],
    "audit criteria or budget/coverage thresholds change"),
  ["field coverage check", "budget conformance check", "joint pass/fail gate"],
  ["operating-point reselection (routed)", "source re-ingestion"],
  [pro("A joint audit prevents optimizing one axis at the other's expense: it fails a brief that is cheap but decision-lossy or lossless but over budget",
       "180-token brief that drops 'blocking_dep' fails coverage even though it meets budget")],
  [con("Joint pass/fail can stall when no point satisfies both (entropy floor exceeds budget), needing escalation not iteration",
       "budget below the field-restricted entropy floor makes a clean pass impossible")],
  ["coverage passed but a decision field silently absent", "budget conformance measured in the wrong tokenizer"],
  ["the brief is decision-lossless over the full field set AND within budget B, both measured with the consumer's tokenizer"],
  ["audit thresholds change", "an infeasibility (floor>budget) is detected"],
  specialists=["evaluation_engineer", "summarization_engineer"]))

N.append(node("REVISION_TRIGGER", "drift_and_revision_trigger_protocol",
  "Define when a previously-passing brief must be regenerated: the field set changes, the source is revised, the consumer/tokenizer changes, or a production round-trip failure is observed. Each trigger invalidates a specific upstream artifact, scoping the rework.",
  "governance", ["COVERAGE_AUDIT"], ["CQ_16"],
  b(0.82, 0.78, 0.74, 0.56, 0.78, 0.82, 0.6, 0.66, 0.5, 0.8, 0.8, 0.82, 0.66, 0.18, 0.48, [0.2, 0.5],
    "trigger taxonomy or invalidation mapping changes"),
  ["revision trigger taxonomy", "trigger-to-artifact invalidation map", "scoped regeneration"],
  ["initial generation", "curve estimation"],
  [pro("Mapping each trigger to the specific invalidated artifact scopes rework: a tokenizer change re-measures budget without re-deriving the field set",
       "source revision invalidates entropy floor and curve but not the field set spec")],
  [con("Over-eager triggers cause needless regeneration; under-eager ones let a stale brief drift into decision loss",
       "regenerating on a cosmetic source edit that touches no decision field")],
  ["a decision-relevant source change does not trigger revision", "trigger fires but invalidates the wrong artifact, missing the real staleness"],
  ["every trigger maps to the precise upstream artifact it invalidates and scoped regeneration restores a passing audit"],
  ["trigger taxonomy changes", "a stale brief reaches production without a trigger firing"],
  specialists=["systems_analyst", "summarization_engineer"]))

# ---------------- competency questions (14) ----------------
CQ = [
 ("CQ_01", "What is the fixed downstream field set and what content must be preserved for it?",
  ["nodes", "glossary"], "FIELD_SET_SPEC fixes the field set; MUST_PRESERVE_CLASSES maps content to it",
  ["FIELD_SET_SPEC", "MUST_PRESERVE_CLASSES"]),
 ("CQ_02", "Why can a brief not be both literally lossless and arbitrarily short, and what is the rate floor?",
  ["nodes"], "SOURCE_ENTROPY_BOUND sets the entropy floor; RATE_DISTORTION frames the tradeoff",
  ["SOURCE_ENTROPY_BOUND", "RATE_DISTORTION"]),
 ("CQ_03", "How is source content partitioned into must-preserve versus droppable, and in what order is it shed?",
  ["nodes"], "MUST_PRESERVE_CLASSES and DROPPABLE_CLASSES partition; SALIENCE_RANKING orders shedding",
  ["MUST_PRESERVE_CLASSES", "DROPPABLE_CLASSES", "SALIENCE_RANKING"]),
 ("CQ_04", "How is distortion defined so that 'lossless' means the right thing?",
  ["nodes"], "DISTORTION_METRIC defines distortion as decision error over the field set, not surface overlap",
  ["DISTORTION_METRIC"]),
 ("CQ_05", "How is the lossless-versus-efficient goal reframed as a rate-distortion tradeoff and an operating point?",
  ["nodes", "conflict_axes"], "RATE_DISTORTION reframes it (FP2); OPERATING_POINT and DECISION_LOSSLESS pick the target",
  ["RATE_DISTORTION", "DECISION_LOSSLESS", "OPERATING_POINT"]),
 ("CQ_06", "What does decision-losslessness mean and how is it operationally tested?",
  ["nodes", "workflow"], "DECISION_LOSSLESS defines it; ROUND_TRIP_TEST tests it operationally",
  ["DECISION_LOSSLESS", "ROUND_TRIP_TEST"]),
 ("CQ_07", "How is the operating point and token budget chosen given per-field loss tolerance?",
  ["nodes"], "OPERATING_POINT selects the point; TOKEN_BUDGET fixes and measures B",
  ["OPERATING_POINT", "TOKEN_BUDGET"]),
 ("CQ_08", "How is the token budget measured and how does distortion vary with it?",
  ["nodes"], "TOKEN_BUDGET measures cost; DISTORTION_CURVE traces distortion vs budget",
  ["TOKEN_BUDGET", "DISTORTION_CURVE"]),
 ("CQ_09", "How is the operating budget validated against coverage and the distortion knee?",
  ["nodes", "workflow"], "DISTORTION_CURVE locates the knee; COVERAGE_AUDIT validates coverage and budget jointly",
  ["DISTORTION_CURVE", "COVERAGE_AUDIT"]),
 ("CQ_10", "Why is the brief optimized for machines and what target does that set?",
  ["nodes"], "MACHINE_FACING sets the parse-optimized target; DENSE_GRAMMAR realizes it",
  ["MACHINE_FACING", "DENSE_GRAMMAR"]),
 ("CQ_11", "What grammar makes the dense line self-describing and unambiguously parseable?",
  ["nodes"], "DENSE_GRAMMAR fixes the pipe/KV format; SCHEMA_HINT and ABBREVIATION_KEY make it self-describing",
  ["DENSE_GRAMMAR", "SCHEMA_HINT", "ABBREVIATION_KEY"]),
 ("CQ_12", "Which compression operators move along the rate axis and how is their distortion controlled?",
  ["nodes"], "COMPRESSION_OPS applies lossless/bounded-lossy operators ordered by SALIENCE_RANKING",
  ["COMPRESSION_OPS", "SALIENCE_RANKING"]),
 ("CQ_13", "How is faithfulness enforced and how is each fact made verifiable?",
  ["nodes"], "FAITHFULNESS forbids hallucination; VERIFIABILITY_TAGS attach provenance",
  ["FAITHFULNESS", "VERIFIABILITY_TAGS"]),
 ("CQ_14", "How is decision-losslessness verified end-to-end before the brief is accepted?",
  ["nodes", "workflow"], "ROUND_TRIP_TEST reconstructs decisions; VERIFIABILITY_TAGS support the audit",
  ["ROUND_TRIP_TEST", "VERIFIABILITY_TAGS"]),
 ("CQ_15", "How is accumulated distortion across compression operators kept within per-field tolerance?",
  ["nodes"], "COMPRESSION_OPS accounts per-operator distortion against the operating-point tolerances",
  ["COMPRESSION_OPS"]),
 ("CQ_16", "When must a passing brief be regenerated and what does each trigger invalidate?",
  ["nodes", "iteration_protocol"], "COVERAGE_AUDIT gates acceptance; REVISION_TRIGGER maps drift to scoped rework",
  ["COVERAGE_AUDIT", "REVISION_TRIGGER"]),
]
# Trim to <=14 CQs: keep CQ_01..CQ_14, fold CQ_15/CQ_16 coverage onto kept ids via CQ_MAP.
CQ = CQ[:14]
CQS = [{"id": i, "question": q, "must_be_answerable_from": m, "acceptance_condition": a, "covered_by": c}
       for (i, q, m, a, c) in CQ]

# consolidate node->CQ references onto the kept 14-CQ set (each node refs 1-2; every CQ covered)
CQ_MAP = {
 "FIELD_SET_SPEC": ["CQ_01"], "SOURCE_ENTROPY_BOUND": ["CQ_02"],
 "MUST_PRESERVE_CLASSES": ["CQ_01", "CQ_03"], "DROPPABLE_CLASSES": ["CQ_03"],
 "DISTORTION_METRIC": ["CQ_04"], "RATE_DISTORTION": ["CQ_02", "CQ_05"],
 "DECISION_LOSSLESS": ["CQ_05", "CQ_06"], "OPERATING_POINT": ["CQ_05", "CQ_07"],
 "TOKEN_BUDGET": ["CQ_07", "CQ_08"], "DISTORTION_CURVE": ["CQ_08", "CQ_09"],
 "MACHINE_FACING": ["CQ_10"], "DENSE_GRAMMAR": ["CQ_10", "CQ_11"],
 "SCHEMA_HINT": ["CQ_11"], "ABBREVIATION_KEY": ["CQ_11", "CQ_12"],
 "FAITHFULNESS": ["CQ_13"], "VERIFIABILITY_TAGS": ["CQ_13", "CQ_14"],
 "ROUND_TRIP_TEST": ["CQ_06", "CQ_14"], "SALIENCE_RANKING": ["CQ_03", "CQ_12"],
 "COMPRESSION_OPS": ["CQ_12"], "COVERAGE_AUDIT": ["CQ_09"],
 "REVISION_TRIGGER": ["CQ_09"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

# ---------------- glossary ----------------
GL = [
 ("rate_distortion_function", "R(D): the minimum rate (bits/tokens) needed to encode a source within distortion D; the lower boundary of the achievable (rate,distortion) region",
  ["R(D)", "rate_distortion_curve"], ["lossless_compression_ratio"], ["RATE_DISTORTION", "DISTORTION_CURVE"]),
 ("decision_lossless", "lossless with respect to a fixed downstream field set: the consumer makes every field decision identically from the brief and from the source",
  ["semantically_lossless", "task_lossless"], ["literal_lossless", "bit_lossless"], ["DECISION_LOSSLESS", "ROUND_TRIP_TEST"]),
 ("source_entropy_floor", "the field-restricted entropy of the source; a lower bound on the rate of any zero-distortion code by Shannon's source coding theorem",
  ["entropy_rate_floor", "rate_floor"], ["compression_target"], ["SOURCE_ENTROPY_BOUND", "TOKEN_BUDGET"]),
 ("distortion", "decision error over the field set: weighted fraction of downstream field decisions the brief would get wrong versus the source",
  ["decision_error", "fidelity_loss"], ["edit_distance", "rouge_overlap"], ["DISTORTION_METRIC", "DISTORTION_CURVE"]),
 ("operating_point", "the chosen (token budget, tolerated distortion) pair on R(D) that the brief targets",
  ["rd_point", "budget_fidelity_point"], ["maximum_compression"], ["OPERATING_POINT", "TOKEN_BUDGET"]),
 ("must_preserve_class", "a content class load-bearing for at least one downstream field, whose loss can flip a decision (identifiers, quantities, negation, deadlines)",
  ["decision_relevant_content"], ["droppable_class"], ["MUST_PRESERVE_CLASSES", "SALIENCE_RANKING"]),
 ("droppable_class", "a content class that is redundant, inferable, or decision-irrelevant and may be compressed or removed",
  ["compressible_content"], ["must_preserve_class"], ["DROPPABLE_CLASSES", "COMPRESSION_OPS"]),
 ("schema_hint", "a compact self-description prepended to the dense line declaring field names, order, types, and a version so the line is self-parsing",
  ["self_describing_header"], ["external_schema_doc"], ["SCHEMA_HINT", "DENSE_GRAMMAR"]),
 ("hallucination", "content in the brief not entailed by the source: extrinsic (absent from source) or intrinsic (contradicting source), per Maynez et al. 2020",
  ["unfaithful_content", "fabrication"], ["valid_inference"], ["FAITHFULNESS", "VERIFIABILITY_TAGS"]),
 ("round_trip_test", "feeding the brief alone to the downstream consumer and checking it reconstructs every field decision correctly; the operational test of decision-losslessness",
  ["reconstruction_test"], ["surface_similarity_check"], ["ROUND_TRIP_TEST", "COVERAGE_AUDIT"]),
 ("minimum_description_length", "Rissanen's MDL principle: prefer the model+data encoding of shortest total length; here, splitting cost between a shared codebook and the message",
  ["mdl", "two_part_code"], ["maximum_likelihood_only"], ["ABBREVIATION_KEY", "DENSE_GRAMMAR"]),
]
GLS = [{"term": t, "definition": d, "synonyms": s, "not_same_as": ns, "used_by_nodes": u}
       for (t, d, s, ns, u) in GL]

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
              "risk_of_conflict": "unmanaged tension degrades the rate-distortion outcome", "example": "see resolution_rule"})


def rel(f, t, et, rs, cc=0.7, cp=0.2, erc=0.3, why=""):
    E.append({"from": f, "to": t, "edge_type": et, "relation_strength": rs, "signed_tension": 0.0,
              "causal_confidence": cc, "conflict_probability": cp, "expected_rework_cost": erc,
              "why_related": why or f"{f} {et} {t}", "benefit_of_coupling": "coordinated behavior",
              "risk_of_conflict": "inconsistency if uncoordinated", "example": f"{f}/{t} {et} relation"})


# dependency edges (acyclic, mirror node.dependencies)
dep("FIELD_SET_SPEC", "MUST_PRESERVE_CLASSES", 0.9)
dep("FIELD_SET_SPEC", "DROPPABLE_CLASSES", 0.8)
dep("FIELD_SET_SPEC", "DISTORTION_METRIC", 0.84)
dep("SOURCE_ENTROPY_BOUND", "RATE_DISTORTION", 0.88)
dep("DISTORTION_METRIC", "RATE_DISTORTION", 0.84)
dep("RATE_DISTORTION", "DECISION_LOSSLESS", 0.86)
dep("RATE_DISTORTION", "OPERATING_POINT", 0.84)
dep("OPERATING_POINT", "TOKEN_BUDGET", 0.84)
dep("TOKEN_BUDGET", "DISTORTION_CURVE", 0.82)
dep("DROPPABLE_CLASSES", "MACHINE_FACING", 0.74)
dep("MACHINE_FACING", "DENSE_GRAMMAR", 0.84)
dep("DENSE_GRAMMAR", "SCHEMA_HINT", 0.82)
dep("DENSE_GRAMMAR", "ABBREVIATION_KEY", 0.78)
dep("MUST_PRESERVE_CLASSES", "FAITHFULNESS", 0.8)
dep("FAITHFULNESS", "VERIFIABILITY_TAGS", 0.8)
dep("DECISION_LOSSLESS", "ROUND_TRIP_TEST", 0.86)
dep("MUST_PRESERVE_CLASSES", "SALIENCE_RANKING", 0.78)
dep("SALIENCE_RANKING", "COMPRESSION_OPS", 0.8)
dep("ABBREVIATION_KEY", "COMPRESSION_OPS", 0.74)
dep("OPERATING_POINT", "COMPRESSION_OPS", 0.76)
dep("ROUND_TRIP_TEST", "COVERAGE_AUDIT", 0.84)
dep("TOKEN_BUDGET", "COVERAGE_AUDIT", 0.78)
dep("DISTORTION_CURVE", "COVERAGE_AUDIT", 0.74)
dep("COVERAGE_AUDIT", "REVISION_TRIGGER", 0.8)

# cross-cutting non-dependency edges (feedback / constraint / causal / similarity)
rel("DISTORTION_CURVE", "OPERATING_POINT", "feedback", 0.8,
    why="the traced distortion curve feeds back to relocate the operating point at the knee")
rel("ROUND_TRIP_TEST", "COMPRESSION_OPS", "feedback", 0.76,
    why="a round-trip failure feeds back to back off a lossy compression operator")
rel("COVERAGE_AUDIT", "OPERATING_POINT", "feedback", 0.76,
    why="a failed audit routes back to reselect the operating point or budget")
rel("REVISION_TRIGGER", "FIELD_SET_SPEC", "feedback", 0.72,
    why="a field-set-change trigger reopens the field set specification")
rel("SOURCE_ENTROPY_BOUND", "OPERATING_POINT", "constraint", 0.78,
    why="the entropy floor constrains feasible operating points to rate>=floor at zero distortion")
rel("OPERATING_POINT", "COMPRESSION_OPS", "constraint", 0.76,
    why="per-field distortion tolerances from the operating point bound which lossy operators may apply")
rel("VERIFIABILITY_TAGS", "ROUND_TRIP_TEST", "causal", 0.72,
    why="provenance tags let the round-trip audit attribute a decision flip to a specific source span")
rel("SCHEMA_HINT", "COVERAGE_AUDIT", "similarity", 0.7,
    why="the schema_hint enumerates the fields the coverage audit checks for presence")

# conflict edges (negative signed_tension + resolution_rule)
conf("DECISION_LOSSLESS", "TOKEN_BUDGET", 0.74, -0.6,
  "respect the entropy floor: hold decision-losslessness for D=0 fields and spend rate to meet it; only relax distortion on fields whose operating-point tolerance allows, never compress a D=0 field below its floor",
  "driving the token budget down conflicts with decision-losslessness once the budget approaches the entropy floor")
conf("MACHINE_FACING", "FAITHFULNESS", 0.68, -0.45,
  "terseness must not introduce ambiguity that the consumer resolves into an unsupported fact; keep every dense token entailed by the source and add a verifiability tag rather than guess",
  "maximal machine-facing terseness can drop qualifiers, risking an unfaithful over-strong claim")
conf("COMPRESSION_OPS", "DECISION_LOSSLESS", 0.7, -0.55,
  "apply lossy operators only to fields whose operating-point distortion tolerance is non-zero; route any operator that would flip a D=0 field's decision back to a lossless alternative",
  "aggressive lossy compression to hit the budget conflicts with keeping decision-critical fields lossless")
conf("ABBREVIATION_KEY", "MACHINE_FACING", 0.64, -0.4,
  "only abbreviate with symbols the consumer's current codebook expands; if the consumer cannot expand a symbol, prefer the longer literal even though it costs rate",
  "abbreviation saves rate but, if the codebook is not shared, makes the brief unparseable by the machine consumer")

# ---------------- conflict axes (9) ----------------
CA = [
 {"name": "decision_lossless_vs_token_budget",
  "description": "Driving tokens down toward the entropy floor eventually starts flipping decisions; holding decisions exact costs rate. This is the central FP2 rate-distortion tension.",
  "poles": ["minimal_tokens", "decision_lossless"],
  "resolution_hint": "pick an operating point on R(D); hold D=0 only for decision-critical fields, never compress them below the floor",
  "tension_score": 0.8,
  "affected_nodes": ["DECISION_LOSSLESS", "TOKEN_BUDGET", "RATE_DISTORTION", "OPERATING_POINT"]},
 {"name": "literal_lossless_vs_decision_lossless",
  "description": "Literal/bit-losslessness is bounded below by source entropy and usually infeasible at a tight budget; decision-losslessness only protects what changes a decision.",
  "poles": ["literal_lossless", "decision_lossless"],
  "resolution_hint": "adopt decision-losslessness over a named field set; document that out-of-set content is intentionally not preserved",
  "tension_score": 0.78,
  "affected_nodes": ["DECISION_LOSSLESS", "SOURCE_ENTROPY_BOUND", "FIELD_SET_SPEC"]},
 {"name": "machine_facing_vs_human_readable",
  "description": "Parse-optimized density removes prose overhead but makes human review and error-spotting hard.",
  "poles": ["machine_facing", "human_readable"],
  "resolution_hint": "optimize for the parser; gate with an automated round-trip rather than human readability",
  "tension_score": 0.66,
  "affected_nodes": ["MACHINE_FACING", "DENSE_GRAMMAR", "ROUND_TRIP_TEST"]},
 {"name": "faithfulness_vs_compression",
  "description": "Aggressive abstractive compression risks extrinsic/intrinsic hallucination; strict entailment limits how far you can compress.",
  "poles": ["max_compression", "strict_faithfulness"],
  "resolution_hint": "permit only entailment-preserving operators on decision fields; tag inferred values explicitly",
  "tension_score": 0.74,
  "affected_nodes": ["FAITHFULNESS", "COMPRESSION_OPS", "MACHINE_FACING"]},
 {"name": "must_preserve_breadth_vs_budget",
  "description": "A broad must-preserve set protects more content but defeats the budget; a narrow one risks dropping a decision-flipping token.",
  "poles": ["broad_preservation", "tight_budget"],
  "resolution_hint": "include in must-preserve only content tracing to a field; rank the rest by decision salience",
  "tension_score": 0.7,
  "affected_nodes": ["MUST_PRESERVE_CLASSES", "DROPPABLE_CLASSES", "SALIENCE_RANKING"]},
 {"name": "abbreviation_savings_vs_recoverability",
  "description": "Codebook abbreviation lowers message rate (MDL) but is unrecoverable if the consumer lacks or drifts from the key.",
  "poles": ["max_abbreviation", "guaranteed_recoverability"],
  "resolution_hint": "abbreviate only with a shared, version-checked codebook; fall back to literals otherwise",
  "tension_score": 0.62,
  "affected_nodes": ["ABBREVIATION_KEY", "DENSE_GRAMMAR", "MACHINE_FACING"]},
 {"name": "provenance_completeness_vs_rate",
  "description": "Per-fact verifiability tags enable audit but cost tokens that compete with the payload at tight budgets.",
  "poles": ["full_provenance", "minimal_rate"],
  "resolution_hint": "tag decision-critical and inferred fields; omit tags for self-evident source-grounded enums",
  "tension_score": 0.58,
  "affected_nodes": ["VERIFIABILITY_TAGS", "FAITHFULNESS", "TOKEN_BUDGET"]},
 {"name": "empirical_curve_cost_vs_point_accuracy",
  "description": "Sampling more budgets locates the knee precisely but costs many generations; coarse sampling is cheap but mis-places the operating point.",
  "poles": ["dense_sampling", "cheap_sampling"],
  "resolution_hint": "sample densely only near the suspected knee; coarse elsewhere",
  "tension_score": 0.55,
  "affected_nodes": ["DISTORTION_CURVE", "OPERATING_POINT", "TOKEN_BUDGET"]},
 {"name": "early_field_set_freeze_vs_late_discovery",
  "description": "Freezing the field set early enables all downstream work but risks omitting a decision the consumer later needs; deferring it stalls everything.",
  "poles": ["early_freeze", "deferred_field_set"],
  "resolution_hint": "freeze the field set against the consumer's decision contract; treat additions as a revision trigger",
  "tension_score": 0.6,
  "affected_nodes": ["FIELD_SET_SPEC", "REVISION_TRIGGER", "COVERAGE_AUDIT"]},
]

# ---------------- edge cases (12) ----------------
EC = [
 {"description": "The field-restricted entropy floor exceeds the imposed token budget, so no decision-lossless brief fits the budget.",
  "trigger": "budget B set below the source entropy floor on the must-preserve fields",
  "affected_nodes": ["SOURCE_ENTROPY_BOUND", "TOKEN_BUDGET", "COVERAGE_AUDIT"],
  "mitigation": "escalate: raise B, narrow the field set, or accept bounded distortion on non-critical fields; do not silently drop a decision field",
  "severity": "critical"},
 {"description": "A negation or unit is compressed away, flipping a decision while the brief still looks faithful and fluent.",
  "trigger": "a must-preserve token (negation, quantity, unit) classed as droppable",
  "affected_nodes": ["MUST_PRESERVE_CLASSES", "DROPPABLE_CLASSES", "DISTORTION_METRIC"],
  "mitigation": "tag negation/quantity/identifier as must-preserve; round-trip test detects the decision flip",
  "severity": "critical"},
 {"description": "Abstractive paraphrase introduces a fact absent from the source (extrinsic hallucination) that the consumer treats as real.",
  "trigger": "lossy paraphrase applied without an entailment check",
  "affected_nodes": ["FAITHFULNESS", "COMPRESSION_OPS", "VERIFIABILITY_TAGS"],
  "mitigation": "gate every value with source entailment; mark derived values 'inferred'",
  "severity": "high"},
 {"description": "The brief is decision-lossless for the named field set but the consumer silently needs a field never enumerated.",
  "trigger": "field set fixed without the full decision contract",
  "affected_nodes": ["FIELD_SET_SPEC", "DECISION_LOSSLESS", "REVISION_TRIGGER"],
  "mitigation": "validate the field set against the consumer's decision contract; treat a new decision as a revision trigger",
  "severity": "high"},
 {"description": "Token cost is measured in a different tokenizer than the consumer uses, so a 'within budget' brief overflows downstream.",
  "trigger": "cost measured with a word-split or wrong-BPE tokenizer",
  "affected_nodes": ["TOKEN_BUDGET", "COVERAGE_AUDIT"],
  "mitigation": "always measure cost in the consumer's tokenizer",
  "severity": "high"},
 {"description": "An abbreviation symbol is unrecoverable because the consumer's codebook drifted or was never shared.",
  "trigger": "abbreviation used outside a shared, version-checked codebook",
  "affected_nodes": ["ABBREVIATION_KEY", "DENSE_GRAMMAR", "MACHINE_FACING"],
  "mitigation": "version the codebook and validate it before parsing; fall back to literals on mismatch",
  "severity": "high"},
 {"description": "A pipe or delimiter appears unescaped inside a field value, corrupting the parse and shifting subsequent fields.",
  "trigger": "delimiter collision with field content in the dense grammar",
  "affected_nodes": ["DENSE_GRAMMAR", "SCHEMA_HINT"],
  "mitigation": "escape delimiters in values; the schema_hint lets the consumer detect field-count mismatch",
  "severity": "high"},
 {"description": "The distortion curve is sampled too coarsely and the operating point is placed past the knee, in the region where decisions flip.",
  "trigger": "budget sweep with too few samples near the knee",
  "affected_nodes": ["DISTORTION_CURVE", "OPERATING_POINT"],
  "mitigation": "densify sampling near the knee; choose a budget at or above it",
  "severity": "medium"},
 {"description": "Composed lossy operators each within tolerance accumulate to flip a field neither would alone.",
  "trigger": "paraphrase plus aggregation applied to the same field without summed-distortion accounting",
  "affected_nodes": ["COMPRESSION_OPS", "DISTORTION_METRIC"],
  "mitigation": "account summed per-field distortion across operators; stop before the field tolerance is exceeded",
  "severity": "medium"},
 {"description": "A source revision changes a decision-relevant value but the cached brief is reused without regeneration.",
  "trigger": "source updated with no revision trigger fired",
  "affected_nodes": ["REVISION_TRIGGER", "COVERAGE_AUDIT", "VERIFIABILITY_TAGS"],
  "mitigation": "fire a source-revision trigger that invalidates the entropy floor, curve, and brief; re-audit",
  "severity": "high"},
 {"description": "The round-trip test passes because the source leaked into the consumer's context, masking a decision-lossy brief.",
  "trigger": "round-trip run with the full source still in context",
  "affected_nodes": ["ROUND_TRIP_TEST", "DECISION_LOSSLESS"],
  "mitigation": "run the consumer on the brief alone with the source removed from context",
  "severity": "high"},
 {"description": "Distortion is scored by surface n-gram overlap, rewarding a faithful-looking brief that nonetheless flips a decision.",
  "trigger": "distortion proxied by ROUGE-style overlap instead of decision error",
  "affected_nodes": ["DISTORTION_METRIC", "ROUND_TRIP_TEST"],
  "mitigation": "score distortion by downstream decision error, not surface similarity",
  "severity": "medium"},
]

# ---------------- workflow (12) ----------------
WF = [
 {"action": "fix_field_set", "node_ref": "FIELD_SET_SPEC",
  "description": "Enumerate the downstream consumer's required fields/decisions and assign criticality weights; this is the losslessness reference.",
  "artifact": "fixed_field_set", "gate": "every downstream decision maps to >=1 weighted field"},
 {"action": "estimate_entropy_floor", "node_ref": "SOURCE_ENTROPY_BOUND",
  "description": "Estimate the field-restricted source entropy to set the rate floor and compressibility headroom.",
  "artifact": "entropy_floor_estimate", "gate": "floor estimated; budget will be checked against it"},
 {"action": "classify_content", "node_ref": "MUST_PRESERVE_CLASSES",
  "description": "Partition content into must-preserve (decision-relevant) and droppable (redundant/inferable) classes.",
  "artifact": "content_class_partition", "gate": "every must-preserve class traces to a field"},
 {"action": "define_distortion", "node_ref": "DISTORTION_METRIC",
  "description": "Define distortion as weighted decision error over the field set, not surface overlap.",
  "artifact": "distortion_metric_spec", "gate": "distortion computed from field decisions with fixed weights"},
 {"action": "reframe_as_rate_distortion", "node_ref": "RATE_DISTORTION",
  "description": "Express the lossless-and-efficient goal as a point on R(D); confirm zero-distortion rate >= entropy floor (FP2).",
  "artifact": "rate_distortion_framing", "gate": "goal stated as an operating point, not literal-lossless+minimal"},
 {"action": "select_operating_point", "node_ref": "OPERATING_POINT",
  "description": "Choose the (budget, per-field distortion tolerance) point; D=0 for decision-critical fields.",
  "artifact": "operating_point_spec", "gate": "point feasible (rate>=floor) with recorded per-field tolerances"},
 {"action": "fix_and_measure_budget", "node_ref": "TOKEN_BUDGET",
  "description": "Fix budget B and measure realized cost in the consumer's tokenizer.",
  "artifact": "budget_and_cost", "gate": "cost <= B in the consumer's tokenizer; B >= floor"},
 {"action": "fix_grammar_and_encode", "node_ref": "DENSE_GRAMMAR",
  "description": "Fix the pipe/KV grammar with a schema_hint and abbreviation key; encode the dense, machine-facing line.",
  "artifact": "dense_brief_draft", "gate": "line parses unambiguously with a schema_hint and escaped delimiters"},
 {"action": "compress_to_budget", "node_ref": "COMPRESSION_OPS",
  "description": "Apply salience-ordered lossless then bounded-lossy operators to meet B, accounting per-field distortion.",
  "artifact": "compressed_brief", "gate": "summed per-field distortion within tolerance; no lossy op on a D=0 field"},
 {"action": "enforce_faithfulness", "node_ref": "FAITHFULNESS",
  "description": "Check every value is source-entailed; attach verifiability tags and mark inferred values.",
  "artifact": "faithfulness_report", "gate": "no extrinsic/intrinsic hallucination on decision fields"},
 {"action": "round_trip_test", "node_ref": "ROUND_TRIP_TEST",
  "description": "Feed the brief alone to the consumer and compare its field decisions against source-based decisions.",
  "artifact": "round_trip_result", "gate": "consumer reconstructs every field decision from the brief alone"},
 {"action": "joint_audit_and_govern", "node_ref": "COVERAGE_AUDIT",
  "description": "Audit coverage and budget jointly; on pass, register revision triggers; on fail, route back.",
  "artifact": "coverage_audit_report", "gate": "decision-lossless over the field set AND within budget B"},
]

# ---------------- dominance rules (9) ----------------
DR = [
 {"rule": "FIELD_SET_SPEC must be fixed before MUST_PRESERVE_CLASSES, DISTORTION_METRIC, or any budget work",
  "rationale": "the field set is the reference for losslessness; classifying or measuring distortion first causes rework",
  "trigger": "content classification or distortion definition begins without a fixed field set",
  "action": "block until the field set is fixed against the decision contract"},
 {"rule": "The token budget must never be set below the field-restricted entropy floor for zero-distortion fields",
  "rationale": "by Shannon's source coding theorem a sub-floor budget cannot be decision-lossless on those fields",
  "trigger": "a budget below the entropy floor is proposed for D=0 fields",
  "action": "raise the budget, narrow the field set, or relax distortion on non-critical fields; never compress a D=0 field below floor"},
 {"rule": "Distortion must be measured as decision error over the field set, not surface overlap, wherever the two disagree",
  "rationale": "surface metrics reward faithful-looking but decision-wrong briefs",
  "trigger": "a surface-overlap distortion proxy is used for acceptance",
  "action": "replace the proxy with a decision-error measure via the round-trip test"},
 {"rule": "No lossy compression operator may be applied to a field whose operating-point distortion tolerance is zero",
  "rationale": "a lossy op on a D=0 field can flip a decision and break decision-losslessness",
  "trigger": "a lossy operator targets a decision-critical field",
  "action": "substitute a lossless operator or back off the operator"},
 {"rule": "FAITHFULNESS gating must pass before ROUND_TRIP_TEST is trusted",
  "rationale": "a hallucinated value can pass a round-trip that the consumer cannot distinguish from truth",
  "trigger": "round-trip run on a brief with unverified entailment",
  "action": "run the entailment check and tag provenance before the round-trip"},
 {"rule": "ROUND_TRIP_TEST must be run on the brief alone, with the source removed from context",
  "rationale": "source leakage masks a decision-lossy brief",
  "trigger": "round-trip executed with the source still in context",
  "action": "strip the source from context and re-run on the brief alone"},
 {"rule": "Abbreviations may only be used when the consumer's current, version-checked codebook expands them",
  "rationale": "an unexpandable abbreviation makes the brief lossy and unparseable",
  "trigger": "an abbreviation is emitted without a shared codebook entry",
  "action": "expand to the literal or add the symbol to the shared codebook first"},
 {"rule": "COVERAGE_AUDIT must pass on both axes (decision-lossless AND within budget) before acceptance",
  "rationale": "optimizing one axis at the other's expense yields a cheap-but-lossy or lossless-but-overbudget brief",
  "trigger": "a brief passes only one of coverage or budget",
  "action": "route back to operating-point or compression; never accept a single-axis pass"},
 {"rule": "A revision trigger must fire whenever the field set, source, or consumer/tokenizer changes",
  "rationale": "a stale brief silently drifts into decision loss",
  "trigger": "a decision-relevant change occurs with no trigger",
  "action": "fire the trigger, invalidate the mapped upstream artifact, and re-audit"},
]

# ---------------- anti-rework rules (8) ----------------
ARR = [
 {"rule": "Do not classify content or define distortion before the field set is fixed; a late field set invalidates both",
  "prevents": "re-classifying content and re-deriving distortion after the field set changes"},
 {"rule": "Do not target a budget below the entropy floor; discovering infeasibility after encoding wastes the whole pass",
  "prevents": "encoding and compressing toward an impossible zero-distortion budget"},
 {"rule": "Do not use surface-overlap distortion as the acceptance metric; swapping it in late re-opens every operating-point decision",
  "prevents": "re-tracing the distortion curve after the metric is corrected"},
 {"rule": "Do not apply lossy operators to decision-critical fields; reverting a decision-flip found at audit means regenerating",
  "prevents": "regenerating the brief after a lossy op flips a D=0 field"},
 {"rule": "Do not measure budget in the wrong tokenizer; an overflow found downstream forces a re-encode",
  "prevents": "re-encoding after a 'within budget' brief overflows the consumer's tokenizer"},
 {"rule": "Do not abbreviate without a shared, version-checked codebook; an unexpandable symbol forces a re-encode at the consumer",
  "prevents": "re-encoding after the consumer cannot expand an abbreviation"},
 {"rule": "Do not run the round-trip with the source in context; a false pass surfaces as a production decision failure",
  "prevents": "shipping a decision-lossy brief that passed a leaked round-trip"},
 {"rule": "Do not omit revision triggers; an undetected source/field/tokenizer change surfaces as silent drift requiring full reanalysis",
  "prevents": "full re-derivation after a stale brief drifts undetected into decision loss"},
]

# ---------------- iteration protocol (7) ----------------
IP = [
 {"trigger": "the entropy floor exceeds the token budget (no feasible zero-distortion brief)",
  "action": "escalate via OPERATING_POINT: raise B, narrow FIELD_SET_SPEC, or relax distortion on non-critical fields",
  "nodes": ["SOURCE_ENTROPY_BOUND", "TOKEN_BUDGET", "OPERATING_POINT"], "priority": "critical"},
 {"trigger": "the round-trip test shows a decision flip from the brief alone",
  "action": "trace the flipped field to its compression operator and back off, re-checking faithfulness",
  "nodes": ["ROUND_TRIP_TEST", "COMPRESSION_OPS", "FAITHFULNESS"], "priority": "critical"},
 {"trigger": "a hallucinated (extrinsic/intrinsic) fact is found on a decision field",
  "action": "tighten FAITHFULNESS entailment and add verifiability tags, then re-run the round-trip",
  "nodes": ["FAITHFULNESS", "VERIFIABILITY_TAGS", "ROUND_TRIP_TEST"], "priority": "high"},
 {"trigger": "the coverage audit fails on the budget axis while coverage passes",
  "action": "re-locate the operating point at the distortion-curve knee and re-apply salience-ordered compression",
  "nodes": ["COVERAGE_AUDIT", "DISTORTION_CURVE", "OPERATING_POINT"], "priority": "high"},
 {"trigger": "a must-preserve token was found shed under a tight budget",
  "action": "re-rank by decision salience and protect the token as must-preserve before re-compressing",
  "nodes": ["SALIENCE_RANKING", "MUST_PRESERVE_CLASSES", "COMPRESSION_OPS"], "priority": "high"},
 {"trigger": "the consumer's tokenizer or codebook changes",
  "action": "re-measure budget in the new tokenizer and re-validate the abbreviation key before re-auditing",
  "nodes": ["TOKEN_BUDGET", "ABBREVIATION_KEY", "COVERAGE_AUDIT"], "priority": "medium"},
 {"trigger": "the downstream field set changes",
  "action": "fire a revision trigger that reopens FIELD_SET_SPEC and invalidates the dependent must-preserve, distortion, and curve artifacts",
  "nodes": ["REVISION_TRIGGER", "FIELD_SET_SPEC", "DISTORTION_METRIC"], "priority": "high"},
]

spec = {
 "domain": "brief__rate_distortion_summary",
 "domain_label": "Decision-Lossless Dense Machine-Facing Brief",
 "purpose": "produce_a_token_budgeted_machine_facing_summary_at_a_chosen_rate_distortion_point_that_is_decision_lossless_over_a_fixed_downstream_field_set",
 "assumptions": [
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "a fixed downstream consumer with an enumerable decision/field set exists; 'lossless' is defined relative to that field set, not literal bit-losslessness",
   "the consumer's tokenizer is known and used to measure token cost",
   "the source is available in full at generation time so faithfulness can be checked by entailment against it",
 ],
 "exclusions": [
   "human-readability and stylistic quality of the brief (the target is machine parsing)",
   "literal bit-level lossless compression (provably infeasible below the source entropy floor at a tight budget)",
   "training or fine-tuning the downstream consumer model",
   "general open-domain abstractive summarization quality metrics decoupled from a downstream field set",
 ],
 "source_description": "heuristic prior estimates for decision-lossless rate-distortion summarization work units, grounded in Shannon's rate-distortion and source coding theory, Rissanen's MDL, and faithfulness research in abstractive summarization; no supplied dataset",
 "source_citation": "Shannon 1948 A Mathematical Theory of Communication (source coding theorem and rate-distortion); Cover & Thomas 2006 Elements of Information Theory (ch. 10 rate-distortion theory, ch. 5 data compression); Rissanen 1978 Modeling by Shortest Data Description (Minimum Description Length); Maynez, Narayan, Bohnet & McDonald 2020 On Faithfulness and Factuality in Abstractive Summarization (ACL)",
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
 "priority_rationale": "FIELD_SET_SPEC and SOURCE_ENTROPY_BOUND are foundational; DISTORTION_METRIC, RATE_DISTORTION and DECISION_LOSSLESS encode the FP2 reframing; OPERATING_POINT/TOKEN_BUDGET fix the rate; grammar and compression realize the brief; faithfulness, round-trip, and the joint coverage audit close decision-losslessness last.",
 "eval_objective": "verify_rate_distortion_framing_decision_losslessness_faithfulness_budget_conformance_and_round_trip_reconstruction_of_brief__rate_distortion_summary_kb",
}

out_dir = "branches/b60_content_intelligence/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "brief__rate_distortion_summary.spec.json")
open(path, "w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes", len(N), "edges", len(E), "CA", len(CA), "EC", len(EC),
      "WF", len(WF), "CQ", len(CQS), "DR", len(DR), "ARR", len(ARR), "IP", len(IP))
