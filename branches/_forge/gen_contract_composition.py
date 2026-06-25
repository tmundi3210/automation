#!/usr/bin/env python3
"""Generate the contract__composition_failure content spec (B13 KB) for kb_forge.py.
Sibling KB to htn__decomposition and term__well_foundedness. Scope = interface
contracts (pre/post/invariant, assume/guarantee), contract composition with set-cover
obligation discharge, and failure handling (defeater-justified fallbacks, SPOF tagging,
compensation/rollback, fatal-break detection, circuit breaking, graceful degradation).
Excludes the decomposition method and termination proofs themselves.

Compact authoring: node() applies sane defaults so only domain content + base metric
magnitudes are specified per node. Mirrors example_htn_gen.py shape exactly."""
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
        "academic_fields": ["software_engineering", "formal_methods", "reliability_engineering"],
        "subfields": subfields or ["design_by_contract", "fault_tolerance"],
        "specialists": specialists or ["reliability_engineer"],
        "contradictors": contradictors or ["best_effort_advocate"],
        "inputs": inputs or ["plan step spec", "interface contract"],
        "outputs": outputs or ["contract-checked step"],
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

# ---- foundations: the contract primitives ----
N.append(node("PRECOND", "step_preconditions_required",
  "Specify the preconditions a plan step requires of its caller and environment: the state predicates and typed inputs that must hold before the step may run; a violated precondition is the caller's fault, not the step's.",
  "foundations", [], ["CQ_01"],
  b(0.9, 0.8, 0.78, 0.55, 0.84, 0.78, 0.55, 0.7, 0.45, 0.78, 0.82, 0.88, 0.7, 0.16, 0.5, [0.2, 0.5], "precondition predicate language or input typing changes"),
  ["required input typing", "required state predicates", "caller obligations"],
  ["postcondition guarantees", "runtime monitoring mechanics"],
  [pro("A precise precondition makes the caller's obligation explicit and checkable before any effect occurs", "a 'charge_card' step requires a valid, non-expired token and a positive amount")],
  [con("Over-strong preconditions shrink the set of contexts a step can legally be used in", "demanding a fully warmed cache excludes valid cold-start callers")],
  ["unstated precondition assumed true", "precondition too weak to exclude a bad input"],
  ["every step exposes an explicit, machine-checkable precondition over typed inputs and state"],
  ["precondition language extended", "a caller-fault failure traced to a missing precondition"],
  specialists=["reliability_engineer", "api_designer"], subfields=["design_by_contract", "hoare_logic"]))

N.append(node("POSTCOND", "step_postconditions_guaranteed",
  "Specify the postconditions a plan step guarantees on success: the state predicates and typed outputs the step promises to establish given its preconditions held; a violated postcondition is the step's (callee's) fault.",
  "foundations", [], ["CQ_01"],
  b(0.9, 0.8, 0.78, 0.55, 0.84, 0.78, 0.55, 0.7, 0.45, 0.78, 0.82, 0.88, 0.7, 0.16, 0.5, [0.2, 0.5], "postcondition guarantee language or output typing changes"),
  ["guaranteed output typing", "guaranteed state predicates", "callee obligations"],
  ["precondition requirements", "compensation logic"],
  [pro("A precise postcondition is the unit of composition: the next step's precondition is discharged against it", "'reserve_inventory' guarantees a reservation id valid for 10 minutes")],
  [con("Guaranteeing too much makes the step brittle and hard to implement honestly", "promising exactly-once side effects over an at-least-once transport")],
  ["postcondition stronger than the step can actually deliver", "success returned without establishing the promised state"],
  ["every step's success path establishes its declared postcondition under its preconditions"],
  ["guarantee language extended", "a callee-fault failure traced to an unmet postcondition"],
  specialists=["reliability_engineer", "api_designer"], subfields=["design_by_contract", "hoare_logic"]))

N.append(node("INVARIANT", "step_preserved_invariants",
  "Specify invariants the step must preserve across its execution: predicates that hold before and after (and are not transiently broken in observable ways), spanning data integrity, resource conservation, and safety properties.",
  "foundations", [], ["CQ_01", "CQ_02"],
  b(0.86, 0.76, 0.72, 0.6, 0.82, 0.8, 0.58, 0.66, 0.5, 0.76, 0.8, 0.84, 0.66, 0.18, 0.52, [0.22, 0.54], "invariant catalog or safety-property set changes"),
  ["data-integrity invariants", "resource-conservation invariants", "safety predicates"],
  ["liveness/termination properties", "per-step pre/post only"],
  [pro("Invariants catch corruption a pre/post pair misses: a step can satisfy its post yet break a global invariant", "ledger debits-equal-credits must hold even mid-transfer")],
  [con("Strong invariants over shared state serialize otherwise-parallel steps", "a global uniqueness invariant forces a single-writer bottleneck")],
  ["invariant broken transiently and observed by a concurrent reader", "invariant scoped too narrowly to protect shared state"],
  ["declared invariants hold before and after the step and are not observably violated mid-step"],
  ["invariant catalog changes", "an invariant violation observed in a concurrent trace"],
  specialists=["reliability_engineer", "formal_methods_engineer"], subfields=["design_by_contract", "concurrency"]))

N.append(node("INTERFACE_CONTRACT", "typed_interface_contract_bundle",
  "Bundle a step's precondition, postcondition, and invariants into one typed interface contract: a first-class, versioned object that fully characterizes the step's obligations to callers and its guarantees in return.",
  "foundations", ["PRECOND", "POSTCOND", "INVARIANT"], ["CQ_02", "CQ_03"],
  b(0.92, 0.82, 0.8, 0.62, 0.86, 0.85, 0.62, 0.66, 0.52, 0.76, 0.8, 0.9, 0.66, 0.18, 0.55, [0.22, 0.56], "contract bundling schema or versioning policy changes"),
  ["pre/post/invariant bundling", "contract typing and versioning", "obligation/guarantee pairing"],
  ["composition algebra", "runtime assertion mechanics"],
  [pro("A first-class contract object lets composition, refinement, and monitoring all reference one source of truth", "a 'payment' contract versioned v2 adds an idempotency-key precondition")],
  [con("Heavyweight contracts add authoring and versioning overhead to every step", "trivial pure steps carry full contract ceremony")],
  ["contract drifts from the step's real behavior", "unversioned contract change silently breaks callers"],
  ["each step has one typed, versioned contract bundling its pre, post and invariants"],
  ["contract schema changes", "a contract-implementation drift incident is observed"],
  specialists=["reliability_engineer", "api_designer"], subfields=["design_by_contract", "interface_design"]))

# ---- composition & proof ----
N.append(node("ASSUME_GUARANTEE", "assume_guarantee_for_open_components",
  "Reason about open steps whose correctness depends on an environment using assume/guarantee: the step guarantees its postcondition only while its assumptions about the environment hold, discharging the rely-guarantee circularity soundly.",
  "composition", ["INTERFACE_CONTRACT"], ["CQ_03", "CQ_04"],
  b(0.84, 0.74, 0.68, 0.74, 0.82, 0.82, 0.58, 0.6, 0.55, 0.7, 0.74, 0.84, 0.6, 0.22, 0.55, [0.28, 0.66], "assume/guarantee proof rule or environment model changes"),
  ["environment assumptions", "conditional guarantees", "rely/guarantee discharge"],
  ["closed-world pre/post", "scheduling"],
  [pro("Assume/guarantee lets two open steps be verified separately then composed without re-verifying the whole", "a consumer assumes the queue is FIFO; the producer guarantees FIFO, closing the pair")],
  [con("Unsound circular A/G reasoning can 'prove' a guarantee that never actually holds", "two steps each assume the other's guarantee with no base case")],
  ["assumption left implicit and violated by the real environment", "circular A/G admitted without a well-founded discharge"],
  ["each open step's guarantee is conditioned on stated assumptions, discharged non-circularly against its environment"],
  ["environment model changes", "a guarantee held in isolation fails in composition"],
  specialists=["formal_methods_engineer", "reliability_engineer"], subfields=["assume_guarantee", "compositional_verification"],
  contradictors=["monolithic_verification_advocate"]))

N.append(node("CONTRACT_COMPOSE", "sequential_and_parallel_composition",
  "Compose step contracts sequentially (chain: predecessor's postcondition must discharge successor's precondition) and in parallel (conjoin guarantees, intersect assumptions), producing the contract of the composite from its parts.",
  "composition", ["INTERFACE_CONTRACT", "ASSUME_GUARANTEE"], ["CQ_04", "CQ_05"],
  b(0.9, 0.8, 0.74, 0.74, 0.86, 0.85, 0.62, 0.6, 0.6, 0.7, 0.74, 0.9, 0.6, 0.22, 0.58, [0.28, 0.66], "composition algebra or conjoining rule changes"),
  ["sequential chaining of contracts", "parallel conjunction of contracts", "composite contract derivation"],
  ["obligation coverage proof", "runtime monitoring"],
  [pro("Composing contracts yields a contract for the whole plan, so the plan itself becomes a checkable specification", "chaining 'reserve' then 'charge': reserve's post discharges charge's pre")],
  [con("Parallel composition must handle interference: two steps' effects can violate each other's assumptions", "two parallel writers each assume sole ownership of a row")],
  ["a chain link where the predecessor post does not discharge the successor pre", "parallel composition ignores cross-step interference"],
  ["the composite contract is derived from its parts with every internal pre discharged by an upstream post"],
  ["composition algebra changes", "a composed plan fails though each step verified alone"],
  specialists=["formal_methods_engineer", "reliability_engineer"], subfields=["compositional_verification", "contract_algebra"]))

N.append(node("OBLIGATION_SETCOVER", "obligation_discharge_set_cover",
  "Prove the composition discharges all obligations: treat every internal precondition and the plan's external postconditions as obligations, and show each is covered by an upstream guarantee or an external assumption (a set-cover closure over obligations).",
  "composition", ["CONTRACT_COMPOSE"], ["CQ_05", "CQ_06"],
  b(0.92, 0.82, 0.78, 0.72, 0.88, 0.85, 0.66, 0.6, 0.58, 0.72, 0.76, 0.92, 0.6, 0.2, 0.6, [0.26, 0.62], "obligation-coverage definition or discharge proof changes"),
  ["obligation enumeration", "guarantee-to-obligation mapping", "set-cover closure of obligations"],
  ["decomposition arity (delegated to htn KB)", "execution"],
  [pro("Set-cover closure makes 'the composition is sound' a checkable property: no precondition is left undischarged", "every internal pre maps to an upstream post or a declared external assumption")],
  [con("Coverage over modeled obligations can still miss an unstated real-world obligation", "the contract omits a regulatory hold step that the plan still needs")],
  ["an undischarged internal precondition (a coverage gap)", "an external assumption silently relied upon but never declared"],
  ["every obligation in the composition is covered by exactly an upstream guarantee or a declared external assumption"],
  ["obligation-coverage definition changes", "an undischarged obligation reaches execution"],
  specialists=["formal_methods_engineer", "reliability_engineer"], subfields=["compositional_verification", "proof_obligations"]))

N.append(node("REFINEMENT", "contract_refinement_substitutability",
  "Govern when one step may substitute for another via contract refinement: a refinement weakens preconditions and strengthens postconditions (Liskov substitutability), so any context satisfying the original contract is served by the refinement.",
  "composition", ["INTERFACE_CONTRACT"], ["CQ_06", "CQ_07"],
  b(0.84, 0.74, 0.7, 0.7, 0.82, 0.82, 0.6, 0.62, 0.55, 0.72, 0.76, 0.84, 0.62, 0.2, 0.54, [0.26, 0.6], "refinement/substitutability rule changes"),
  ["precondition weakening", "postcondition strengthening", "Liskov substitution check"],
  ["composition closure", "blame mechanics"],
  [pro("Refinement lets a step be swapped for a stronger one without re-verifying every caller", "a 'fetch' refined to add caching keeps the same contract, so callers are unaffected")],
  [con("A 'refinement' that strengthens preconditions silently breaks existing callers", "a v2 step now demands an auth scope v1 callers never supplied")],
  ["substitution that strengthens a precondition or weakens a postcondition", "covariance/contravariance reversed in the contract"],
  ["a substituting step weakens preconditions and strengthens postconditions relative to the one it replaces"],
  ["substitutability rule changes", "a substitution broke a previously valid caller"],
  specialists=["api_designer", "formal_methods_engineer"], subfields=["liskov_substitution", "behavioral_subtyping"]))

# ---- failure model: defeaters, fallback, SPOF, compensation ----
N.append(node("DEFEATER", "explicit_defeater_conditions",
  "Enumerate the explicit defeater (exception) conditions that void a step's guarantee: the specific environment or input states under which the postcondition is not promised, distinguishing a defeated guarantee from a contract violation.",
  "failure", ["INTERFACE_CONTRACT"], ["CQ_07", "CQ_08"],
  b(0.86, 0.76, 0.72, 0.66, 0.84, 0.82, 0.58, 0.62, 0.58, 0.72, 0.76, 0.86, 0.62, 0.22, 0.55, [0.28, 0.64], "defeater taxonomy or exception-condition set changes"),
  ["defeater enumeration", "voided-guarantee conditions", "defeater-vs-violation distinction"],
  ["fallback path selection", "retry policy"],
  [pro("Naming defeaters turns 'it sometimes fails' into specific, handleable conditions each with a planned response", "'rate_limited' is a declared defeater of 'send', routed to a fallback rather than treated as a bug")],
  [con("An over-broad defeater set lets a step excuse genuine violations as 'expected' failures", "labeling all 5xx as defeaters hides a real postcondition bug")],
  ["a genuine contract violation mislabeled as a benign defeater", "an unanticipated failure with no defeater and no handler"],
  ["every way a guarantee can be voided is an enumerated defeater with a defined handling path"],
  ["defeater taxonomy changes", "an unhandled failure mode appears with no matching defeater"],
  specialists=["reliability_engineer"], subfields=["defeasible_reasoning", "fault_modeling"]))

N.append(node("FALLBACK", "defeater_justified_fallback_path",
  "Provide a fallback path that is justified by a specific defeater, not a blanket retry: when a named defeater fires, take an alternative step whose weaker contract still advances the goal, recording why the fallback is admissible.",
  "failure", ["DEFEATER"], ["CQ_08", "CQ_09"],
  b(0.84, 0.76, 0.74, 0.68, 0.82, 0.8, 0.6, 0.6, 0.6, 0.7, 0.74, 0.84, 0.6, 0.24, 0.55, [0.3, 0.66], "fallback admissibility or defeater-to-fallback mapping changes"),
  ["defeater-keyed fallback selection", "fallback contract admissibility", "fallback justification record"],
  ["bounded retry of the same step", "circuit breaking"],
  [pro("A defeater-justified fallback degrades capability deliberately instead of retrying a step that cannot succeed", "on 'primary_region_down', fall back to the read replica rather than retrying the primary")],
  [con("Fallbacks multiply the contract surface: each fallback's weaker guarantee must still cover the obligation", "a cache fallback returns stale data that violates a freshness obligation")],
  ["fallback fires for an unrelated failure (blanket catch)", "fallback's weaker contract fails to cover the obligation it replaces"],
  ["each fallback is triggered by a named defeater and its contract still discharges the obligation it serves"],
  ["fallback mapping changes", "a fallback masked a failure it was not justified for"],
  specialists=["reliability_engineer"], subfields=["graceful_degradation", "fault_tolerance"]))

N.append(node("SPOF_TAG", "single_point_of_failure_tagging",
  "Tag single points of failure: steps whose failure is uncompensated and irreversible and that have no fallback, so a break there cannot be recovered; surface SPOFs for design review before the plan is allowed to run.",
  "failure", ["FALLBACK", "INVARIANT"], ["CQ_09", "CQ_10"],
  b(0.88, 0.78, 0.74, 0.6, 0.86, 0.82, 0.7, 0.62, 0.58, 0.74, 0.78, 0.88, 0.62, 0.2, 0.58, [0.24, 0.56], "SPOF criteria or irreversibility threshold changes"),
  ["uncompensated-step detection", "no-fallback step tagging", "SPOF surfacing for review"],
  ["compensation authoring", "retry mechanics"],
  [pro("Tagging SPOFs makes the riskiest steps explicit so they get a fallback, compensation, or human gate before launch", "an irreversible 'wire_funds' with no compensation is tagged and gated on approval")],
  [con("Over-tagging floods review with steps that are cheap to retry and not truly single points", "every external call tagged SPOF drowns the genuinely irreversible ones")],
  ["an uncompensated irreversible step ships untagged", "SPOF tag added but never gated, so it is ignored"],
  ["every uncompensated, irreversible, fallback-less step is tagged as a SPOF and gated before execution"],
  ["SPOF criteria change", "an untagged SPOF caused an unrecoverable failure"],
  specialists=["reliability_engineer", "risk_analyst"], subfields=["fault_tree_analysis", "fault_tolerance"]))

N.append(node("COMPENSATION", "compensation_rollback_saga",
  "Define compensation/rollback for steps with committed effects: pair each non-idempotent committing step with a compensating action (saga) that semantically undoes it, so a later failure can unwind earlier commits to a consistent state.",
  "failure", ["POSTCOND", "SPOF_TAG"], ["CQ_10", "CQ_11"],
  b(0.9, 0.8, 0.76, 0.74, 0.88, 0.85, 0.78, 0.58, 0.62, 0.7, 0.74, 0.9, 0.58, 0.24, 0.6, [0.3, 0.68], "compensation semantics or saga boundary changes"),
  ["compensating-action pairing", "saga unwind ordering", "semantic (not physical) rollback"],
  ["two-phase commit internals", "circuit breaking"],
  [pro("Sagas give long-running plans a recovery story without distributed locks: failures unwind committed effects in reverse", "if 'ship' fails after 'charge', run 'refund' to compensate the charge")],
  [con("Compensation is not always possible or complete: some effects (an email sent) cannot be truly undone", "you cannot un-send a notification that already triggered downstream action")],
  ["a committed step left without a compensation", "compensation runs out of order and leaves inconsistent state"],
  ["every committing, non-idempotent step has a compensating action that restores a consistent state when unwound"],
  ["compensation semantics change", "an unwind left the system in an inconsistent state"],
  specialists=["reliability_engineer", "distributed_systems_engineer"], subfields=["sagas", "transaction_management"]))

N.append(node("IDEMPOTENCE", "idempotency_for_safe_retry",
  "Make committing steps idempotent (via idempotency keys or dedup) so that a retry repeats the request without repeating the effect; idempotence is the precondition that makes any retry policy safe rather than effect-duplicating.",
  "failure", ["POSTCOND"], ["CQ_11", "CQ_12"],
  b(0.86, 0.78, 0.74, 0.68, 0.85, 0.8, 0.62, 0.62, 0.6, 0.74, 0.78, 0.86, 0.62, 0.22, 0.56, [0.26, 0.6], "idempotency-key scheme or dedup window changes"),
  ["idempotency-key design", "effect deduplication", "retry-safety precondition"],
  ["retry scheduling/backoff", "compensation"],
  [pro("Idempotence converts an unsafe retry into a safe one: a duplicated request yields the same single effect", "a payment with an idempotency key charges once even if retried three times")],
  [con("Idempotency infrastructure (keys, dedup stores, windows) adds latency and storage to every committing call", "a dedup store lookup on the hot path adds a round-trip per request")],
  ["retry duplicates a non-idempotent effect", "idempotency window too short, so a late retry double-applies"],
  ["every retryable committing step is idempotent under its key so retries cannot duplicate effects"],
  ["idempotency scheme changes", "a duplicate effect is traced to a non-idempotent retry"],
  specialists=["reliability_engineer", "distributed_systems_engineer"], subfields=["idempotency", "exactly_once_semantics"]))

N.append(node("ERROR_PROPAGATION", "typed_error_propagation",
  "Propagate failures as typed errors carrying the violated obligation and blame, rather than swallowing them or collapsing them to a generic failure; the error type tells the caller which contract clause failed and who is responsible.",
  "failure", ["INTERFACE_CONTRACT", "DEFEATER"], ["CQ_12", "CQ_13"],
  b(0.84, 0.74, 0.72, 0.62, 0.82, 0.82, 0.55, 0.64, 0.55, 0.74, 0.78, 0.84, 0.64, 0.2, 0.5, [0.24, 0.56], "error taxonomy or propagation contract changes"),
  ["typed error channel", "violated-clause carrying", "no-swallow propagation"],
  ["blame adjudication policy", "monitoring"],
  [pro("Typed errors let a caller route a 'precondition-violated' differently from a 'defeater-fired', preserving information", "a 'PreconditionViolated(amount<=0)' error is handled distinctly from a 'RateLimited' defeater")],
  [con("Rich typed errors couple caller and callee to a shared, evolving error vocabulary", "adding an error variant forces every caller's match to be updated")],
  ["an error swallowed and reported as success", "failure collapsed to a generic error that loses the violated clause"],
  ["failures propagate as typed errors naming the violated obligation and the responsible party"],
  ["error taxonomy changes", "a swallowed error masked a contract violation"],
  specialists=["reliability_engineer", "api_designer"], subfields=["error_handling", "result_types"]))

# ---- control / halt / degrade ----
N.append(node("FATAL_BREAK", "fatal_break_detection_halt",
  "Detect fatal breaks that must halt rather than retry: violations of safety invariants, exhausted budgets, or unrecoverable (uncompensable) SPOF failures, where continuing is unsafe; convert these into an immediate, blame-tagged halt.",
  "control", ["ERROR_PROPAGATION", "SPOF_TAG"], ["CQ_13", "CQ_14"],
  b(0.92, 0.82, 0.78, 0.66, 0.9, 0.85, 0.72, 0.62, 0.62, 0.74, 0.78, 0.92, 0.62, 0.2, 0.62, [0.24, 0.56], "fatal-break classification or halt policy changes"),
  ["safety-invariant-violation detection", "uncompensable-failure halt", "blame-tagged stop"],
  ["bounded retry", "graceful degradation"],
  [pro("Detecting fatal breaks stops the plan from retrying or degrading into an unsafe state", "a broken ledger invariant halts the run instead of retrying the corrupting step")],
  [con("Aggressive fatal-break classification halts plans that a fallback could have rescued", "a transient outage misread as fatal stops a plan a replica could have served")],
  ["a fatal break retried instead of halted", "a recoverable failure misclassified as fatal and halted needlessly"],
  ["every safety-invariant violation or uncompensable failure triggers an immediate halt, never a retry"],
  ["fatal-break classification changes", "a fatal condition was retried and worsened the state"],
  specialists=["reliability_engineer", "safety_engineer"], subfields=["fail_fast", "safety_engineering"],
  contradictors=["always_retry_advocate"]))

N.append(node("CIRCUIT_BREAK", "circuit_breaker_cascade_stop",
  "Stop cascading failure with a circuit breaker: after a failure threshold against a dependency, open the circuit to fail fast (and shed load) instead of piling retries onto a failing component, with a half-open probe to recover.",
  "control", ["ERROR_PROPAGATION"], ["CQ_14"],
  b(0.84, 0.78, 0.76, 0.66, 0.82, 0.78, 0.58, 0.64, 0.58, 0.74, 0.78, 0.84, 0.64, 0.22, 0.52, [0.24, 0.56], "breaker thresholds or half-open probe policy changes"),
  ["failure-threshold tripping", "fail-fast open state", "half-open recovery probe"],
  ["fatal-break halt (whole plan)", "compensation"],
  [pro("A breaker prevents retries from amplifying an outage and gives the failing dependency room to recover", "after 50% errors over 10s, the breaker opens and sheds calls for 30s")],
  [con("A breaker that opens too eagerly turns a brief blip into a self-inflicted outage", "a 2-second hiccup trips a long open window, dropping healthy traffic")],
  ["breaker never trips, so retries flood a dead dependency", "breaker opens on noise and starves a healthy service"],
  ["a per-dependency breaker trips at threshold, fails fast while open, and probes before closing"],
  ["breaker thresholds change", "a retry storm bypassed the breaker"],
  specialists=["reliability_engineer", "sre"], subfields=["circuit_breaker", "load_shedding"]))

N.append(node("GRACEFUL_DEGRADE", "graceful_degradation_reduced_service",
  "Degrade gracefully to a reduced but contract-honoring service when a dependency is unavailable: serve a weaker guarantee (cached, partial, or default) that still satisfies a declared reduced-service contract rather than failing the whole plan.",
  "control", ["FALLBACK", "CIRCUIT_BREAK"], ["CQ_09", "CQ_14"],
  b(0.82, 0.78, 0.8, 0.66, 0.8, 0.8, 0.55, 0.62, 0.6, 0.72, 0.76, 0.82, 0.62, 0.24, 0.52, [0.28, 0.62], "reduced-service contract or degradation tiers change"),
  ["reduced-service contract tiers", "partial/cached/default responses", "degradation-vs-halt choice"],
  ["fatal halt", "blame adjudication"],
  [pro("Graceful degradation keeps the goal partly met under failure instead of an all-or-nothing outage", "when recommendations are down, serve a static popular list under a 'best-effort' contract")],
  [con("Silent degradation can violate an obligation the caller assumed was still guaranteed", "serving stale prices as if fresh during a degradation breaks a freshness contract")],
  ["degraded mode violates an undeclared obligation", "degradation hides an outage that should have halted"],
  ["each degraded mode honors an explicit reduced-service contract and is signaled to the caller, never silent"],
  ["degradation tiers change", "a degraded response violated a freshness or correctness obligation"],
  specialists=["reliability_engineer", "product_engineer"], subfields=["graceful_degradation", "service_tiers"]))

N.append(node("RETRY_POLICY", "bounded_retry_with_backoff",
  "Apply a bounded retry policy with backoff and jitter, gated on a retryable defeater and on idempotence: retry only transient, idempotent failures a bounded number of times, then surface the failure to fallback or halt.",
  "control", ["DEFEATER", "IDEMPOTENCE"], ["CQ_08", "CQ_12"],
  b(0.8, 0.74, 0.72, 0.6, 0.8, 0.76, 0.55, 0.64, 0.6, 0.74, 0.78, 0.8, 0.64, 0.24, 0.5, [0.26, 0.58], "retry bound, backoff, or jitter policy changes"),
  ["retryable-defeater gating", "bounded attempts with backoff+jitter", "exhaustion handoff to fallback/halt"],
  ["fatal-break detection", "compensation"],
  [pro("Bounded, idempotence-gated retry recovers transient failures cheaply without amplifying load or duplicating effects", "retry a timed-out idempotent read up to 3 times with exponential backoff and jitter")],
  [con("Retries against a non-idempotent or fatal failure duplicate effects or delay an inevitable halt", "retrying a non-idempotent charge double-bills the customer")],
  ["unbounded retry storms a failing dependency", "retry of a non-idempotent or fatal failure"],
  ["retries are bounded, jittered, gated on a retryable defeater and idempotence, then hand off on exhaustion"],
  ["retry policy changes", "a retry storm or duplicate effect is traced to this policy"],
  specialists=["reliability_engineer", "sre"], subfields=["retry_backoff", "transient_fault_handling"]))

# ---- runtime QA: monitoring, blame ----
N.append(node("CONTRACT_MONITOR", "runtime_contract_assertions",
  "Monitor contracts at runtime: evaluate pre/post/invariant assertions around each step in production (sampled or full), so a contract violation is detected when it happens rather than inferred later, feeding blame and propagation.",
  "verification", ["INTERFACE_CONTRACT", "OBLIGATION_SETCOVER"], ["CQ_13", "CQ_15"],
  b(0.82, 0.74, 0.74, 0.62, 0.8, 0.78, 0.55, 0.66, 0.5, 0.78, 0.8, 0.82, 0.66, 0.18, 0.5, [0.2, 0.5], "assertion sampling or monitoring overhead policy changes"),
  ["runtime pre/post/invariant checks", "violation detection at the boundary", "sampling vs full assertion"],
  ["static composition proof", "blame adjudication policy"],
  [pro("Runtime monitoring catches the contract drift and environment surprises a static proof cannot foresee", "an assertion fires when a downstream service returns an out-of-contract null")],
  [con("Full assertion checking on the hot path adds latency; sampling trades coverage for cost", "validating every large payload's invariant doubles step latency")],
  ["assertions disabled in production, so violations go unseen", "monitoring overhead pushed the step past its latency budget"],
  ["each step's contract is asserted at runtime (at least sampled) and violations are detected at the boundary"],
  ["sampling policy changes", "a production violation was detected only downstream, not at its boundary"],
  specialists=["reliability_engineer", "observability_engineer"], subfields=["runtime_verification", "assertions"]))

N.append(node("BLAME_ASSIGN", "blame_assignment_caller_vs_callee",
  "Assign blame on a contract violation: a precondition violation blames the caller, a postcondition/invariant violation blames the callee, and a defeater blames neither; correct blame routes the failure to the party that can fix it.",
  "verification", ["CONTRACT_MONITOR", "ERROR_PROPAGATION"], ["CQ_15", "CQ_16"],
  b(0.8, 0.72, 0.72, 0.6, 0.8, 0.78, 0.55, 0.66, 0.55, 0.76, 0.78, 0.8, 0.66, 0.18, 0.5, [0.2, 0.5], "blame-assignment rule or violation taxonomy changes"),
  ["caller-fault on precondition", "callee-fault on postcondition/invariant", "no-fault on defeater"],
  ["error transport mechanics", "degradation tiers"],
  [pro("Correct blame routes a failure to the only party who can fix it and prevents finger-pointing in composition", "a null input blamed on the caller is rejected at the boundary, not retried by the callee")],
  [con("Blame can be ambiguous when an invariant spans both parties or a defeater overlaps a violation", "a shared-state invariant break is not cleanly caller- or callee-fault")],
  ["caller-fault failure handled as a callee bug (or vice versa)", "blame ambiguous, so the failure is misrouted"],
  ["every contract violation is classified as caller-fault, callee-fault, or defeater and routed accordingly"],
  ["blame taxonomy changes", "a misassigned blame sent a failure to the wrong owner"],
  specialists=["reliability_engineer", "formal_methods_engineer"], subfields=["blame_calculus", "fault_attribution"]))

# ---------------- competency questions (14) ----------------
CQ = [
 ("CQ_01", "What pre/post/invariant does a plan step declare, and what does each constrain?", ["nodes", "glossary"], "PRECOND, POSTCOND and INVARIANT define the required, guaranteed and preserved predicates", ["PRECOND", "POSTCOND", "INVARIANT"]),
 ("CQ_02", "How are pre/post/invariant bundled into one typed, versioned interface contract?", ["nodes"], "INTERFACE_CONTRACT bundles the three primitives into a first-class versioned contract", ["INTERFACE_CONTRACT", "INVARIANT"]),
 ("CQ_03", "How is an open step whose correctness depends on its environment specified and verified?", ["nodes"], "ASSUME_GUARANTEE conditions a guarantee on stated assumptions and discharges them non-circularly", ["ASSUME_GUARANTEE", "INTERFACE_CONTRACT"]),
 ("CQ_04", "How are step contracts composed sequentially and in parallel?", ["nodes", "edges"], "CONTRACT_COMPOSE chains posts into pres and conjoins parallel guarantees", ["CONTRACT_COMPOSE", "ASSUME_GUARANTEE"]),
 ("CQ_05", "How is it proved that a composition discharges all its obligations (set-cover closure)?", ["nodes", "workflow"], "OBLIGATION_SETCOVER shows every obligation is covered by an upstream guarantee or external assumption", ["OBLIGATION_SETCOVER", "CONTRACT_COMPOSE"]),
 ("CQ_06", "When may one step substitute for another without breaking callers?", ["nodes"], "REFINEMENT requires weakened preconditions and strengthened postconditions (Liskov)", ["REFINEMENT", "OBLIGATION_SETCOVER"]),
 ("CQ_07", "How are the conditions that void a step's guarantee enumerated and distinguished from violations?", ["nodes"], "DEFEATER enumerates voiding conditions and separates them from contract violations", ["DEFEATER", "REFINEMENT"]),
 ("CQ_08", "How is a fallback chosen and bounded so it is not a blanket retry?", ["nodes", "workflow"], "FALLBACK keys on a named defeater; RETRY_POLICY bounds retries before fallback", ["FALLBACK", "RETRY_POLICY", "DEFEATER"]),
 ("CQ_09", "How are single points of failure and degraded modes identified and handled?", ["nodes", "conflict_axes"], "SPOF_TAG surfaces uncompensated irreversible steps; GRACEFUL_DEGRADE serves a reduced-service contract", ["SPOF_TAG", "FALLBACK", "GRACEFUL_DEGRADE"]),
 ("CQ_10", "How are committed effects unwound on later failure, and how are retries made safe against duplicated effects?", ["nodes"], "COMPENSATION pairs each committing step with a saga undo; IDEMPOTENCE makes retryable steps repeat without duplicate effects", ["COMPENSATION", "IDEMPOTENCE", "SPOF_TAG"]),
 ("CQ_11", "How are failures propagated and retried without being swallowed?", ["nodes"], "ERROR_PROPAGATION emits typed errors; RETRY_POLICY bounds retries gated on idempotence", ["ERROR_PROPAGATION", "RETRY_POLICY"]),
 ("CQ_12", "How are fatal breaks detected and separated from recoverable failures, and monitored at runtime?", ["nodes", "iteration_protocol"], "FATAL_BREAK halts on uncompensable/safety failures; CONTRACT_MONITOR detects violations at runtime", ["FATAL_BREAK", "CONTRACT_MONITOR"]),
 ("CQ_13", "How is cascading failure stopped and service degraded instead of halted?", ["nodes", "workflow"], "CIRCUIT_BREAK stops cascades; GRACEFUL_DEGRADE serves a reduced contract; FATAL_BREAK halts only the unsafe", ["CIRCUIT_BREAK", "GRACEFUL_DEGRADE", "FATAL_BREAK"]),
 ("CQ_14", "On a contract violation detected at runtime, who is to blame and how does that route the failure?", ["nodes", "conflict_axes"], "BLAME_ASSIGN classifies caller-fault (precondition), callee-fault (postcondition/invariant) or no-fault defeater and routes accordingly", ["BLAME_ASSIGN", "CONTRACT_MONITOR"]),
]
CQS = [{"id": i, "question": q, "must_be_answerable_from": m, "acceptance_condition": a, "covered_by": c} for (i, q, m, a, c) in CQ]

# consolidate node->CQ references (each node refs 1-2; every CQ covered by >=1 node)
CQ_MAP = {
 "PRECOND": ["CQ_01"], "POSTCOND": ["CQ_01"], "INVARIANT": ["CQ_01", "CQ_02"],
 "INTERFACE_CONTRACT": ["CQ_02", "CQ_03"], "ASSUME_GUARANTEE": ["CQ_03", "CQ_04"],
 "CONTRACT_COMPOSE": ["CQ_04", "CQ_05"], "OBLIGATION_SETCOVER": ["CQ_05", "CQ_06"],
 "REFINEMENT": ["CQ_06", "CQ_07"], "DEFEATER": ["CQ_07", "CQ_08"], "FALLBACK": ["CQ_08", "CQ_09"],
 "SPOF_TAG": ["CQ_09", "CQ_10"], "COMPENSATION": ["CQ_10"], "IDEMPOTENCE": ["CQ_10", "CQ_11"],
 "ERROR_PROPAGATION": ["CQ_11", "CQ_12"], "FATAL_BREAK": ["CQ_12", "CQ_13"], "CIRCUIT_BREAK": ["CQ_13"],
 "GRACEFUL_DEGRADE": ["CQ_09", "CQ_13"], "RETRY_POLICY": ["CQ_08", "CQ_11"],
 "CONTRACT_MONITOR": ["CQ_12", "CQ_14"], "BLAME_ASSIGN": ["CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

# ---------------- glossary ----------------
GL = [
 ("precondition", "a predicate over inputs and state that must hold before a step runs; its violation is the caller's fault", ["requires_clause", "rely_condition"], ["postcondition"], ["PRECOND", "INTERFACE_CONTRACT"]),
 ("postcondition", "a predicate the step guarantees on success given its precondition held; its violation is the callee's fault", ["ensures_clause", "guarantee"], ["precondition"], ["POSTCOND", "INTERFACE_CONTRACT"]),
 ("invariant", "a predicate preserved across a step, not observably broken mid-execution", ["safety_property", "consistency_condition"], ["transient_state"], ["INVARIANT", "FATAL_BREAK"]),
 ("interface_contract", "the typed, versioned bundle of a step's pre, post and invariants", ["design_contract", "spec_contract"], ["implementation"], ["INTERFACE_CONTRACT", "REFINEMENT"]),
 ("assume_guarantee", "reasoning where a guarantee is conditioned on assumptions about the environment", ["rely_guarantee"], ["closed_world_contract"], ["ASSUME_GUARANTEE", "CONTRACT_COMPOSE"]),
 ("defeater", "an enumerated condition that voids a guarantee without being a contract violation", ["exception_condition", "voiding_condition"], ["contract_violation"], ["DEFEATER", "FALLBACK"]),
 ("fallback", "an alternative path taken for a named defeater whose weaker contract still covers the obligation", ["alternative_path", "degraded_path"], ["blanket_retry"], ["FALLBACK", "GRACEFUL_DEGRADE"]),
 ("single_point_of_failure", "an uncompensated, irreversible, fallback-less step whose failure is unrecoverable", ["spof", "uncompensated_step"], ["redundant_step"], ["SPOF_TAG", "FATAL_BREAK"]),
 ("compensation", "a semantic undo (saga) paired with a committing step to unwind it on later failure", ["rollback_action", "saga_step"], ["physical_rollback"], ["COMPENSATION", "IDEMPOTENCE"]),
 ("idempotence", "the property that repeating a request yields a single effect, making retry safe", ["effect_dedup", "retry_safety"], ["at_least_once_effect"], ["IDEMPOTENCE", "RETRY_POLICY"]),
 ("circuit_breaker", "a guard that opens after a failure threshold to fail fast and stop cascades", ["breaker", "tripwire"], ["unbounded_retry"], ["CIRCUIT_BREAK", "GRACEFUL_DEGRADE"]),
 ("blame", "attribution of a violation to caller (precondition) or callee (postcondition/invariant)", ["fault_attribution", "responsibility"], ["no_fault_defeater"], ["BLAME_ASSIGN", "ERROR_PROPAGATION"]),
 ("obligation_set_cover", "the property that every internal precondition is discharged by an upstream guarantee or external assumption", ["coverage_closure", "discharge_closure"], ["partial_discharge"], ["OBLIGATION_SETCOVER", "CONTRACT_COMPOSE"]),
]
GLS = [{"term": t, "definition": d, "synonyms": s, "not_same_as": ns, "used_by_nodes": u} for (t, d, s, ns, u) in GL]

# ---------------- edges: dependency (DAG) + cross-cutting + conflict ----------------
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
              "risk_of_conflict": "unmanaged tension degrades plan reliability", "example": "see resolution_rule"})


def rel(f, t, et, rs, cc=0.7, cp=0.2, erc=0.3, why=""):
    E.append({"from": f, "to": t, "edge_type": et, "relation_strength": rs, "signed_tension": 0.0,
              "causal_confidence": cc, "conflict_probability": cp, "expected_rework_cost": erc,
              "why_related": why or f"{f} {et} {t}", "benefit_of_coupling": "coordinated behavior",
              "risk_of_conflict": "inconsistency if uncoordinated", "example": f"{f}/{t} {et} relation"})


# dependency edges (acyclic, mirror node.dependencies)
dep("PRECOND", "INTERFACE_CONTRACT", 0.9)
dep("POSTCOND", "INTERFACE_CONTRACT", 0.9)
dep("INVARIANT", "INTERFACE_CONTRACT", 0.84)
dep("INTERFACE_CONTRACT", "ASSUME_GUARANTEE", 0.8)
dep("INTERFACE_CONTRACT", "CONTRACT_COMPOSE", 0.84)
dep("ASSUME_GUARANTEE", "CONTRACT_COMPOSE", 0.8)
dep("CONTRACT_COMPOSE", "OBLIGATION_SETCOVER", 0.86)
dep("INTERFACE_CONTRACT", "REFINEMENT", 0.78)
dep("INTERFACE_CONTRACT", "DEFEATER", 0.8)
dep("DEFEATER", "FALLBACK", 0.82)
dep("FALLBACK", "SPOF_TAG", 0.78)
dep("INVARIANT", "SPOF_TAG", 0.72)
dep("POSTCOND", "COMPENSATION", 0.78)
dep("SPOF_TAG", "COMPENSATION", 0.8)
dep("POSTCOND", "IDEMPOTENCE", 0.78)
dep("INTERFACE_CONTRACT", "ERROR_PROPAGATION", 0.8)
dep("DEFEATER", "ERROR_PROPAGATION", 0.74)
dep("ERROR_PROPAGATION", "FATAL_BREAK", 0.82)
dep("SPOF_TAG", "FATAL_BREAK", 0.78)
dep("ERROR_PROPAGATION", "CIRCUIT_BREAK", 0.78)
dep("FALLBACK", "GRACEFUL_DEGRADE", 0.8)
dep("CIRCUIT_BREAK", "GRACEFUL_DEGRADE", 0.76)
dep("DEFEATER", "RETRY_POLICY", 0.78)
dep("IDEMPOTENCE", "RETRY_POLICY", 0.8)
dep("INTERFACE_CONTRACT", "CONTRACT_MONITOR", 0.8)
dep("OBLIGATION_SETCOVER", "CONTRACT_MONITOR", 0.74)
dep("CONTRACT_MONITOR", "BLAME_ASSIGN", 0.82)
dep("ERROR_PROPAGATION", "BLAME_ASSIGN", 0.76)
# cross-cutting non-dependency edges
rel("OBLIGATION_SETCOVER", "REFINEMENT", "constraint", 0.74, why="a refinement must still satisfy the composition's obligation set-cover")
rel("IDEMPOTENCE", "COMPENSATION", "constraint", 0.74, why="idempotent steps need no compensation; only non-idempotent committed effects do")
rel("BLAME_ASSIGN", "FALLBACK", "causal", 0.72, why="defeater (no-fault) blame routes to a fallback, while caller/callee fault does not")
rel("FATAL_BREAK", "COMPENSATION", "sequence", 0.74, why="on a fatal break the saga unwinds committed effects before the halt completes")
rel("CONTRACT_MONITOR", "DEFEATER", "feedback", 0.7, why="monitored boundary failures feed the defeater taxonomy with new voiding conditions")
rel("CIRCUIT_BREAK", "RETRY_POLICY", "constraint", 0.72, why="an open breaker suppresses retries so they cannot storm a failing dependency")
rel("ASSUME_GUARANTEE", "BLAME_ASSIGN", "similarity", 0.7, why="a violated environment assumption is the blame boundary between a component and its environment")
# conflict edges (negative signed_tension + resolution_rule) -- real tensions
conf("FATAL_BREAK", "GRACEFUL_DEGRADE", 0.72, -0.6,
  "classify first: a safety-invariant or uncompensable failure halts (fatal) and is never degraded; only failures with a declared reduced-service contract may degrade",
  "fail-fast fatal-break halting conflicts with graceful degradation that keeps serving")
conf("RETRY_POLICY", "IDEMPOTENCE", 0.7, -0.55,
  "gate retries on idempotence: a step may be retried only if it is idempotent under its key; otherwise route to compensation or fallback instead of retrying",
  "bounded retry presumes safe repetition, but non-idempotent steps make each retry duplicate effects (idempotence cost)")
conf("INTERFACE_CONTRACT", "CONTRACT_COMPOSE", 0.68, -0.45,
  "favor composability: keep preconditions as weak and postconditions as honest as possible so contracts chain; strengthen only where a real obligation demands it",
  "strict, strong contracts ease local reasoning but reduce the set of compositions that type-check end to end")
conf("COMPENSATION", "RETRY_POLICY", 0.66, -0.5,
  "bound the recovery budget: prefer compensation completeness for committed effects, but cap unwind/retry latency with a timeout that escalates to fatal-break when the budget is exhausted",
  "compensation completeness (full saga unwind) conflicts with latency budgets that retries and rollbacks consume")

# ---------------- conflict_axes (9) ----------------
CA = [
 {"name": "fail_fast_fatal_break_vs_graceful_degrade", "description": "Halting on a fatal break protects safety but ends service; degrading keeps serving but can mask an unsafe state.", "poles": ["fail_fast_halt", "graceful_degrade"], "resolution_hint": "halt only safety/uncompensable failures; degrade only under a declared reduced-service contract", "tension_score": 0.75, "affected_nodes": ["FATAL_BREAK", "GRACEFUL_DEGRADE", "CIRCUIT_BREAK"]},
 {"name": "bounded_retry_vs_idempotence_cost", "description": "Retry cheaply recovers transients but is unsafe without idempotence, whose key/dedup infrastructure costs latency and storage.", "poles": ["aggressive_retry", "idempotence_overhead"], "resolution_hint": "gate retries on idempotence; for non-idempotent steps compensate instead of retrying", "tension_score": 0.7, "affected_nodes": ["RETRY_POLICY", "IDEMPOTENCE", "COMPENSATION"]},
 {"name": "strict_contracts_vs_composability", "description": "Strong pre/post ease local proof but shrink the set of contracts that chain end to end.", "poles": ["strict_strong_contracts", "weak_composable_contracts"], "resolution_hint": "weakest honest preconditions, strongest honest postconditions; strengthen only on real obligation", "tension_score": 0.68, "affected_nodes": ["INTERFACE_CONTRACT", "CONTRACT_COMPOSE", "REFINEMENT"]},
 {"name": "compensation_completeness_vs_latency", "description": "A complete saga unwind restores consistency but the rollback chain adds latency that competes with budgets.", "poles": ["full_unwind", "low_latency"], "resolution_hint": "cap unwind/retry budget; escalate to fatal-break when exhausted", "tension_score": 0.66, "affected_nodes": ["COMPENSATION", "RETRY_POLICY", "FATAL_BREAK"]},
 {"name": "defeater_justified_fallback_vs_blanket_retry", "description": "Defeater-keyed fallbacks degrade deliberately; blanket retries are simpler but amplify load and mask faults.", "poles": ["defeater_justified_fallback", "blanket_retry"], "resolution_hint": "fallback only on a named defeater; retry only transient idempotent failures, bounded", "tension_score": 0.66, "affected_nodes": ["FALLBACK", "DEFEATER", "RETRY_POLICY"]},
 {"name": "runtime_monitoring_coverage_vs_overhead", "description": "Full assertion checking catches every violation at its boundary but adds hot-path latency; sampling trades coverage for cost.", "poles": ["full_assertion", "sampled_assertion"], "resolution_hint": "always assert cheap invariants; sample expensive ones; full-check around SPOFs", "tension_score": 0.6, "affected_nodes": ["CONTRACT_MONITOR", "INVARIANT", "OBLIGATION_SETCOVER"]},
 {"name": "circuit_breaker_sensitivity_vs_availability", "description": "An eager breaker stops cascades but can drop healthy traffic; a lax breaker lets retries amplify an outage.", "poles": ["eager_trip", "lax_trip"], "resolution_hint": "threshold on error-rate over a window with a half-open probe before closing", "tension_score": 0.63, "affected_nodes": ["CIRCUIT_BREAK", "RETRY_POLICY", "GRACEFUL_DEGRADE"]},
 {"name": "blame_precision_vs_shared_invariant_ambiguity", "description": "Clean caller/callee blame routes failures correctly, but invariants spanning both parties resist clean attribution.", "poles": ["clean_blame", "shared_responsibility"], "resolution_hint": "split shared invariants into per-party obligations; treat overlap as a defeater needing review", "tension_score": 0.58, "affected_nodes": ["BLAME_ASSIGN", "INVARIANT", "ASSUME_GUARANTEE"]},
 {"name": "spof_tagging_sensitivity_vs_review_load", "description": "Tagging every risky step surfaces real SPOFs but floods review; tagging too few lets an unrecoverable step ship.", "poles": ["tag_broadly", "tag_only_irreversible"], "resolution_hint": "tag only uncompensated AND irreversible AND fallback-less steps", "tension_score": 0.57, "affected_nodes": ["SPOF_TAG", "COMPENSATION", "FALLBACK"]},
]

# ---------------- edge_cases (12) ----------------
EC = [
 {"description": "A safety invariant is broken mid-plan yet the engine retries the corrupting step instead of halting.", "trigger": "fatal break misclassified as a transient, retryable failure", "affected_nodes": ["FATAL_BREAK", "INVARIANT", "RETRY_POLICY"], "mitigation": "classify safety-invariant and uncompensable failures as fatal; halt, never retry", "severity": "critical"},
 {"description": "A non-idempotent committing step is retried and double-applies its effect (e.g. double charge).", "trigger": "retry policy not gated on idempotence", "affected_nodes": ["RETRY_POLICY", "IDEMPOTENCE", "COMPENSATION"], "mitigation": "gate retries on idempotency keys; route non-idempotent failures to compensation", "severity": "critical"},
 {"description": "A composed plan fails in production though every step verified in isolation.", "trigger": "an internal precondition was never discharged by an upstream postcondition", "affected_nodes": ["OBLIGATION_SETCOVER", "CONTRACT_COMPOSE", "ASSUME_GUARANTEE"], "mitigation": "require set-cover closure: every internal pre mapped to an upstream guarantee or declared assumption", "severity": "high"},
 {"description": "An uncompensated, irreversible step fails and the plan cannot be unwound.", "trigger": "a SPOF shipped untagged and ungated", "affected_nodes": ["SPOF_TAG", "COMPENSATION", "FATAL_BREAK"], "mitigation": "tag uncompensated irreversible steps; gate on a fallback, compensation, or human approval", "severity": "critical"},
 {"description": "A blanket retry storms a dead dependency, turning a brief outage into a sustained one.", "trigger": "retries not bounded and no circuit breaker in front of the dependency", "affected_nodes": ["RETRY_POLICY", "CIRCUIT_BREAK", "DEFEATER"], "mitigation": "bound retries with backoff/jitter; open a breaker at threshold to shed load", "severity": "high"},
 {"description": "A fallback returns stale cached data that violates a freshness obligation the caller still assumes.", "trigger": "degraded mode honors no declared reduced-service contract", "affected_nodes": ["FALLBACK", "GRACEFUL_DEGRADE", "POSTCOND"], "mitigation": "each degraded mode honors an explicit reduced-service contract and is signaled to the caller", "severity": "high"},
 {"description": "An error is caught and reported as success, hiding a postcondition violation from the caller.", "trigger": "failure swallowed instead of propagated as a typed error", "affected_nodes": ["ERROR_PROPAGATION", "POSTCOND", "CONTRACT_MONITOR"], "mitigation": "propagate typed errors naming the violated clause; never swallow", "severity": "high"},
 {"description": "A caller-fault precondition violation is retried by the callee as if it were the callee's bug.", "trigger": "blame misassigned on a contract violation", "affected_nodes": ["BLAME_ASSIGN", "PRECOND", "ERROR_PROPAGATION"], "mitigation": "classify precondition violation as caller-fault; reject at the boundary, do not retry", "severity": "medium"},
 {"description": "Two open steps each assume the other's guarantee with no base case, 'proving' a guarantee that never holds.", "trigger": "circular assume/guarantee admitted without a well-founded discharge", "affected_nodes": ["ASSUME_GUARANTEE", "CONTRACT_COMPOSE", "OBLIGATION_SETCOVER"], "mitigation": "require a non-circular (well-founded) discharge of the rely/guarantee pair", "severity": "high"},
 {"description": "A circuit breaker trips on a 2-second blip and opens a long window, dropping healthy traffic.", "trigger": "breaker threshold too sensitive or open window too long", "affected_nodes": ["CIRCUIT_BREAK", "GRACEFUL_DEGRADE", "RETRY_POLICY"], "mitigation": "threshold on error-rate over a window; half-open probe before fully closing", "severity": "medium"},
 {"description": "A v2 step strengthens a precondition (new required auth scope), silently breaking existing callers.", "trigger": "a substitution presented as a refinement that is not behaviorally substitutable", "affected_nodes": ["REFINEMENT", "INTERFACE_CONTRACT", "PRECOND"], "mitigation": "refinements weaken preconditions and strengthen postconditions; version-gate any strengthening", "severity": "high"},
 {"description": "A saga's compensating action runs out of order and leaves the system inconsistent.", "trigger": "compensation unwind order not the reverse of the commit order", "affected_nodes": ["COMPENSATION", "FATAL_BREAK", "INVARIANT"], "mitigation": "unwind compensations in strict reverse commit order; verify the consistency invariant after", "severity": "high"},
]

# ---------------- workflow (12) ----------------
WF = [
 {"action": "declare_pre_post_invariant", "node_ref": "PRECOND", "description": "Declare each step's required preconditions, guaranteed postconditions, and preserved invariants over typed inputs and state.", "artifact": "pre_post_invariant_spec", "gate": "every step has explicit, typed pre/post/invariant"},
 {"action": "bundle_interface_contract", "node_ref": "INTERFACE_CONTRACT", "description": "Bundle pre/post/invariant into one typed, versioned interface contract per step.", "artifact": "interface_contract", "gate": "each step carries one versioned contract"},
 {"action": "condition_open_components", "node_ref": "ASSUME_GUARANTEE", "description": "For open steps, state environment assumptions and condition guarantees on them with a non-circular discharge.", "artifact": "assume_guarantee_record", "gate": "each open guarantee names its assumptions, discharged non-circularly"},
 {"action": "compose_contracts", "node_ref": "CONTRACT_COMPOSE", "description": "Chain contracts sequentially and conjoin them in parallel to derive the composite contract.", "artifact": "composite_contract", "gate": "composite derived; parallel interference handled"},
 {"action": "prove_obligation_coverage", "node_ref": "OBLIGATION_SETCOVER", "description": "Show every internal precondition is discharged by an upstream guarantee or a declared external assumption.", "artifact": "obligation_coverage_proof", "gate": "no undischarged obligation (set-cover closed)"},
 {"action": "check_substitutability", "node_ref": "REFINEMENT", "description": "Verify any substituted step weakens preconditions and strengthens postconditions.", "artifact": "refinement_check", "gate": "substitutions are behaviorally substitutable"},
 {"action": "enumerate_defeaters", "node_ref": "DEFEATER", "description": "Enumerate the conditions that void each guarantee, distinct from violations, each with a handling path.", "artifact": "defeater_catalog", "gate": "every voiding condition is a named defeater with a handler"},
 {"action": "wire_fallbacks_and_retry", "node_ref": "FALLBACK", "description": "Attach defeater-keyed fallbacks and bound, idempotence-gated retries; suppress retries behind an open breaker.", "artifact": "recovery_routes", "gate": "fallbacks keyed to defeaters; retries bounded and idempotent"},
 {"action": "tag_spofs_and_compensate", "node_ref": "SPOF_TAG", "description": "Tag uncompensated irreversible steps and pair committing steps with compensating saga actions.", "artifact": "spof_and_saga_map", "gate": "every committing step compensated or tagged and gated"},
 {"action": "classify_fatal_vs_recoverable", "node_ref": "FATAL_BREAK", "description": "Separate fatal breaks (halt) from recoverable failures (retry/fallback/degrade) and circuit-break cascading deps.", "artifact": "failure_classification", "gate": "safety/uncompensable failures halt; others routed to recovery"},
 {"action": "degrade_or_halt", "node_ref": "GRACEFUL_DEGRADE", "description": "Serve a declared reduced-service contract when a dependency is down, or halt if the failure is fatal.", "artifact": "degradation_plan", "gate": "degraded modes honor a reduced-service contract; fatal halts"},
 {"action": "monitor_and_assign_blame", "node_ref": "CONTRACT_MONITOR", "description": "Assert contracts at runtime; on violation, assign caller/callee blame and propagate a typed error.", "artifact": "monitoring_and_blame_report", "gate": "violations detected at boundary and correctly attributed"},
]

# ---------------- dominance_rules (9) ----------------
DR = [
 {"rule": "PRECOND, POSTCOND and INVARIANT must be declared before INTERFACE_CONTRACT is finalized", "rationale": "the contract bundle is meaningless without its three primitives", "trigger": "a contract bundled with a missing pre, post, or invariant", "action": "block bundling until all three primitives are declared"},
 {"rule": "OBLIGATION_SETCOVER closure must hold before a composition is allowed to execute", "rationale": "an undischarged internal precondition is an unsound composition", "trigger": "execution requested with an uncovered obligation", "action": "block execution and route to CONTRACT_COMPOSE to discharge it"},
 {"rule": "FATAL_BREAK classification dominates retry, fallback, and degradation for safety-invariant or uncompensable failures", "rationale": "continuing past an unsafe state corrupts more than it recovers", "trigger": "a safety-invariant violation or uncompensable failure is retried or degraded", "action": "halt immediately with a blame tag, overriding any recovery route"},
 {"rule": "RETRY_POLICY must be gated on IDEMPOTENCE before any retry of a committing step", "rationale": "retrying a non-idempotent step duplicates effects", "trigger": "a retry scheduled for a non-idempotent committing step", "action": "block the retry; route to compensation or fallback"},
 {"rule": "Every committing, non-idempotent step must have a COMPENSATION or be SPOF-tagged and gated before launch", "rationale": "an uncompensated irreversible commit is unrecoverable", "trigger": "a committing step ships with neither compensation nor SPOF gate", "action": "block launch until compensated or gated"},
 {"rule": "FALLBACK must be keyed to a named DEFEATER, never a blanket catch-all", "rationale": "a blanket fallback masks failures it was not justified for", "trigger": "a fallback triggered by an unclassified failure", "action": "require a named defeater before the fallback may fire"},
 {"rule": "ERROR_PROPAGATION must emit a typed error; failures may not be swallowed into success", "rationale": "a swallowed error hides a contract violation", "trigger": "a failure path returns success", "action": "force a typed error carrying the violated clause and blame"},
 {"rule": "REFINEMENT substitutions must weaken preconditions and strengthen postconditions", "rationale": "the reverse silently breaks existing callers", "trigger": "a substitution strengthens a precondition or weakens a postcondition", "action": "reject the substitution or version-gate it"},
 {"rule": "CONTRACT_MONITOR assertions must be enabled in production at least on a sampled basis for high-risk steps", "rationale": "violations detected only downstream are far costlier to attribute", "trigger": "assertions disabled on a SPOF-tagged step in production", "action": "require at least sampled assertion around SPOF-tagged steps"},
]

# ---------------- anti_rework_rules (8) ----------------
ARR = [
 {"rule": "Do not compose contracts before pre/post/invariant are stable; a primitive change reshapes every composite", "prevents": "re-deriving composite contracts after a late precondition edit"},
 {"rule": "Do not execute a composition before set-cover closure; an undischarged obligation surfaces as a production failure", "prevents": "emergency re-verification after a composed plan fails in production"},
 {"rule": "Do not add retry before idempotence; retrofitting idempotency keys after a double-effect incident is costly", "prevents": "incident-driven idempotency retrofit on a non-idempotent step"},
 {"rule": "Do not ship a committing step without compensation or a SPOF gate; unwinding after an irreversible commit is impossible", "prevents": "unrecoverable state after an untagged SPOF fails"},
 {"rule": "Do not wire blanket fallbacks; replacing them with defeater-keyed routes later requires re-auditing every catch", "prevents": "re-auditing catch-all fallbacks that masked real faults"},
 {"rule": "Do not strengthen a precondition under the guise of refinement; callers break silently and must all be revisited", "prevents": "tracking down every caller broken by a non-substitutable change"},
 {"rule": "Do not classify failures as fatal vs recoverable after recovery is wired; misclassification forces re-routing all handlers", "prevents": "re-routing recovery paths after a fatal condition was retried"},
 {"rule": "Do not defer runtime monitoring; adding assertions after a silent violation requires reproducing the incident", "prevents": "reconstructing an undetected violation that monitoring would have caught at the boundary"},
]

# ---------------- iteration_protocol (8) ----------------
IP = [
 {"trigger": "a composed plan fails though each step verified alone", "action": "re-run OBLIGATION_SETCOVER to find the undischarged obligation and fix the chain in CONTRACT_COMPOSE", "nodes": ["OBLIGATION_SETCOVER", "CONTRACT_COMPOSE"], "priority": "critical"},
 {"trigger": "a safety invariant was violated and the engine retried instead of halting", "action": "reclassify the condition as fatal in FATAL_BREAK and assert the invariant in CONTRACT_MONITOR", "nodes": ["FATAL_BREAK", "INVARIANT", "CONTRACT_MONITOR"], "priority": "critical"},
 {"trigger": "a duplicate effect is traced to a retry", "action": "gate RETRY_POLICY on IDEMPOTENCE and add an idempotency key to the committing step", "nodes": ["RETRY_POLICY", "IDEMPOTENCE"], "priority": "critical"},
 {"trigger": "an irreversible step failed with no way to unwind", "action": "tag it in SPOF_TAG and author a COMPENSATION or a launch gate", "nodes": ["SPOF_TAG", "COMPENSATION"], "priority": "high"},
 {"trigger": "a retry storm amplified a dependency outage", "action": "bound retries and place a CIRCUIT_BREAK in front of the dependency", "nodes": ["RETRY_POLICY", "CIRCUIT_BREAK"], "priority": "high"},
 {"trigger": "a degraded response violated an obligation the caller assumed", "action": "define a reduced-service contract in GRACEFUL_DEGRADE and key the FALLBACK to a named defeater", "nodes": ["GRACEFUL_DEGRADE", "FALLBACK"], "priority": "high"},
 {"trigger": "a failure was reported as success", "action": "enforce typed propagation in ERROR_PROPAGATION and attribute it in BLAME_ASSIGN", "nodes": ["ERROR_PROPAGATION", "BLAME_ASSIGN"], "priority": "medium"},
 {"trigger": "a substitution broke existing callers", "action": "check substitutability in REFINEMENT against the original INTERFACE_CONTRACT and version-gate the change", "nodes": ["REFINEMENT", "INTERFACE_CONTRACT"], "priority": "medium"},
]

spec = {
 "domain": "contract__composition_failure",
 "domain_label": "Contract-Based Composition & Failure Handling for Plans (Recursive Planning Engine subdomain)",
 "purpose": "session_bounded_method_for_composing_plan_steps_under_typed_interface_contracts_of_preconditions_postconditions_and_invariants_with_assume_guarantee_reasoning_proving_each_composition_discharges_its_obligations_by_set_cover_closure_and_handling_failure_through_defeater_justified_fallbacks_single_point_of_failure_tagging_compensation_and_rollback_idempotent_safe_retry_typed_error_propagation_fatal_break_detection_circuit_breaking_graceful_degradation_runtime_contract_monitoring_and_caller_versus_callee_blame_assignment",
 "assumptions": [
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "scope is interface contracts and failure handling; the decomposition method and termination proofs are delegated to sibling KBs",
   "each plan step exposes (or can be given) a typed contract of preconditions, postconditions and invariants",
   "committing steps can be made idempotent or paired with a semantic compensating action",
 ],
 "exclusions": [
   "the hierarchical/recursive decomposition method itself (delegated to htn__decomposition)",
   "formal termination and well-foundedness proofs (delegated to term__well_foundedness)",
   "operator implementation internals and the transport/runtime substrate",
   "learning contracts or failure policies from observed data",
 ],
 "source_description": "heuristic prior estimates for contract-based composition and failure-handling work units, informed by design-by-contract, Hoare logic, assume/guarantee (rely-guarantee) reasoning, behavioral subtyping, the saga pattern, and stability patterns (circuit breaker, bulkhead, graceful degradation); no supplied dataset",
 "source_citation": "Meyer 1992 Applying Design by Contract; Hoare 1969 An Axiomatic Basis for Computer Programming; Jones 1983 Rely/Guarantee (Tentative Steps Toward a Development Method for Interfering Programs); Abadi & Lamport 1995 Conjoining Specifications; Lamport 2002 Specifying Systems; Liskov & Wing 1994 A Behavioral Notion of Subtyping; Garcia-Molina & Salem 1987 Sagas; Nygard 2018 Release It! (circuit breaker, bulkhead, graceful degradation); Findler & Felleisen 2002 Contracts for Higher-Order Functions (blame)",
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
 "priority_rationale": "PRECOND/POSTCOND/INVARIANT and INTERFACE_CONTRACT are foundational; ASSUME_GUARANTEE, CONTRACT_COMPOSE and OBLIGATION_SETCOVER are the composition/proof core; DEFEATER through COMPENSATION build the failure model; FATAL_BREAK, CIRCUIT_BREAK and GRACEFUL_DEGRADE govern halt-vs-degrade; CONTRACT_MONITOR and BLAME_ASSIGN close the loop at runtime.",
 "eval_objective": "verify_contract_typing_composition_obligation_setcover_closure_defeater_justified_failure_handling_compensation_fatal_break_detection_and_blame_assignment_of_contract__composition_failure_kb",
}

out_dir = "branches/b13_recursive_planner/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "contract__composition_failure.spec.json")
open(path, "w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes", len(N), "edges", len(E), "CA", len(CA), "EC", len(EC), "WF", len(WF), "CQ", len(CQS), "DR", len(DR), "ARR", len(ARR), "IP", len(IP))
