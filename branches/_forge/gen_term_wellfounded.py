#!/usr/bin/env python3
"""Generate the term__well_foundedness content spec (B13 sibling KB) for kb_forge.py.
Sibling to htn__decomposition: the decomposition method delegates termination proofs
HERE. Scope is termination / well-foundedness only; the decomposition method itself and
contract composition are out of scope. Compact authoring via node()/b()/pro()/con() and
edge helpers, mirroring example_htn_gen.py."""
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
        "academic_fields": ["program_verification", "automated_planning"],
        "subfields": subfields or ["termination_analysis", "well_founded_recursion"],
        "specialists": specialists or ["verification_engineer"],
        "contradictors": contradictors or ["unbounded_recursion_advocate"],
        "inputs": inputs or ["recursive plan / decomposition", "reachable state model"],
        "outputs": outputs or ["discharged termination obligation"],
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

# ---- L0 foundations ----
N.append(node("WELLFOUND_ORDER","well_founded_order_selection",
  "Select and justify a well-founded order (a relation with no infinite strictly descending chain) over the codomain the ranking function maps into, e.g. (N,<), ordinals, or a lexicographic/multiset extension.",
  "foundations", [], ["CQ_01"],
  b(0.92,0.78,0.74,0.62,0.86,0.82,0.62,0.7,0.45, 0.78,0.82, 0.9,0.7,0.16,0.52,[0.2,0.5],"the chosen order's well-foundedness assumption or codomain changes"),
  ["choice of order (N,< / ordinals / lexicographic / multiset)","proof or citation of no-infinite-descent","codomain definition"],
  ["the ranking function values themselves","operational semantics of steps"],
  [pro("A well-founded order is the single hypothesis that turns 'measure decreases' into 'recursion halts' (Floyd's well-founded sets)","(N,<) admits no infinite descending chain, so a strictly decreasing N-valued measure must reach a minimum")],
  [con("Picking too weak an order (e.g. (Q>=0,<)) breaks well-foundedness and silently invalidates every downstream proof","0,1/2,1/4,... descends forever in the rationals: a measure into Q>=0 proves nothing")],
  ["order assumed well-founded but is not (dense or unbounded-below)","conflating a partial order with a well-founded one"],
  ["the selected order is proven or cited to have no infinite descending chain","the codomain of the ranking function is exactly this order's carrier"],
  ["order replaced or generalized to ordinals","a descending chain is exhibited in the chosen order"],
  specialists=["verification_engineer","order_theory_specialist"],
  subfields=["order_theory","well_founded_recursion"]))

N.append(node("RANK_FUNCTION","ranking_variant_function_definition",
  "Define a ranking (variant) function mapping each reachable state / recursive-call argument tuple into the carrier of the well-founded order, so that recursion behaviour is summarized by a single comparable quantity.",
  "foundations", [], ["CQ_01","CQ_02"],
  b(0.93,0.8,0.78,0.66,0.88,0.84,0.64,0.66,0.5, 0.76,0.8, 0.92,0.66,0.18,0.55,[0.22,0.55],"the argument structure or state representation the rank reads from changes"),
  ["the rank/variant expression","its domain (reachable states / call arguments)","its codomain (the order's carrier)"],
  ["proving the decrease (PROGRESS_MEASURE)","the order's well-foundedness (WELLFOUND_ORDER)"],
  [pro("A variant function reduces an open-ended halting question to arithmetic/order comparison on a single value (Turing's quantity that decreases)","rank(ackermann m n)=(m,n) collapses the whole recursion to one lexicographic pair")],
  [con("A rank that ignores a recursion-driving argument cannot be made to decrease and will block the proof","measuring only list length misses a recursion that shrinks an embedded tree, not the list")],
  ["rank omits an argument that actually drives recursion","rank is not a function (multi-valued on equal states)"],
  ["the ranking function is total on reachable states and lands in the order's carrier","its value is determined by the recursion-driving arguments"],
  ["recursion-driving arguments change","a recursive call is found that the current rank cannot distinguish"],
  specialists=["verification_engineer","logic_specialist"],
  subfields=["termination_analysis","variant_functions"]))

# ---- L1 ----
N.append(node("BASE_CASE","base_case_minimal_element_proof",
  "Identify the recursion's base cases and prove they coincide with the minimal elements of the well-founded order: when the rank is minimal, no further recursive call is issued.",
  "foundations", ["WELLFOUND_ORDER"], ["CQ_03"],
  b(0.86,0.74,0.72,0.56,0.82,0.74,0.58,0.7,0.42, 0.8,0.82, 0.86,0.7,0.16,0.5,[0.18,0.46],"the base-case predicate or minimal elements of the order change"),
  ["enumeration of base cases","proof base case implies no recursive call","minimal-element correspondence"],
  ["the decrease on recursive steps (PROGRESS_MEASURE)","fuel/budget fallback"],
  [pro("Anchoring base cases at the order's minimum makes induction along the order discharge the whole recursion","rank=0 is exactly 'empty input', the terminating leaf")],
  [con("A base case not at the minimum leaves a gap where the rank can still decrease but recursion stops early or never","stopping at length<=1 while rank counts pairs leaves an unhandled length-1 case")],
  ["a reachable minimal-rank state still issues a recursive call","base predicate and minimal element disagree"],
  ["every minimal-rank reachable state is a non-recursing base case","every base case is reachable and handled"],
  ["base-case predicate revised","a recursive call observed at minimal rank"]))

N.append(node("TOTALITY_CHECK","measure_totality_on_reachable_states",
  "Establish that the ranking function is total and defined on every reachable state / call argument, so the proof never compares an undefined rank and no reachable call escapes the measure.",
  "foundations", ["RANK_FUNCTION"], ["CQ_02"],
  b(0.82,0.7,0.66,0.6,0.8,0.76,0.55,0.66,0.45, 0.78,0.8, 0.82,0.66,0.18,0.48,[0.2,0.5],"the reachable-state set or the rank's domain definition changes"),
  ["domain-of-definition proof","reachable-state coverage","handling of partial/undefined inputs"],
  ["soundness wrt semantics (MEASURE_SOUNDNESS)","the order itself"],
  [pro("Totality on reachable states closes the 'measure undefined here' loophole that makes a decrease proof vacuous","every node a recursive call can reach has a defined rank, so the decrease is always checkable")],
  [con("Proving totality over an over-approximated reachable set can demand discharging unreachable, hard cases","including dead branches forces a rank for inputs the planner never produces")],
  ["rank undefined on a reachable argument","reachable set under-approximated, missing a real state"],
  ["the ranking function is defined on every reachable state","unreachable states are excluded with justification"],
  ["reachable-state over/under-approximation revised","an undefined-rank state is reached in a trace"]))

N.append(node("MEASURE_SOUNDNESS","measure_soundness_wrt_semantics",
  "Show the ranking function and its decrease are sound with respect to the operational semantics: the rank the proof manipulates is the rank the executed recursive call actually carries, with no abstraction gap.",
  "foundations", ["RANK_FUNCTION"], ["CQ_04"],
  b(0.88,0.76,0.7,0.74,0.86,0.85,0.66,0.6,0.55, 0.7,0.76, 0.88,0.6,0.22,0.58,[0.28,0.66],"the operational semantics or the state/argument abstraction the rank reads changes"),
  ["semantics-to-rank correspondence","abstraction-gap analysis","argument-passing fidelity"],
  ["the certificate assembly","syntactic guard placement"],
  [pro("Soundness ties the paper proof to the running engine, so a discharged obligation actually constrains execution","the rank is computed from the same arguments the interpreter binds at the call, not a model of them")],
  [con("A faithful semantic model is expensive and an over-idealized one can certify a recursion that diverges in practice","modelling integers as unbounded hides an overflow that resets the rank and loops")],
  ["abstraction gap: proven rank differs from executed rank","unmodeled side effect mutates the recursion-driving argument"],
  ["the rank used in the proof provably equals the rank of the executed call","abstractions are justified or shown sound"],
  ["operational semantics revised","a divergence reproduced that the proof did not predict"],
  specialists=["verification_engineer","semantics_specialist"]))

# ---- L2 ----
N.append(node("PROGRESS_MEASURE","strict_decrease_per_recursive_step",
  "Prove the ranking function strictly decreases, in the well-founded order, on every recursive step: for each call site, rank(callee args) < rank(caller args).",
  "core", ["RANK_FUNCTION","WELLFOUND_ORDER"], ["CQ_05"],
  b(0.94,0.82,0.8,0.7,0.9,0.85,0.66,0.62,0.55, 0.7,0.76, 0.94,0.62,0.2,0.6,[0.26,0.62],"a recursive call site is added, removed, or its argument transformation changes"),
  ["per-step strict-decrease lemma","case split over call sites","the decrease inequality in the order"],
  ["the depth/fuel fallback","mutual-recursion summing (MUTUAL_RECURSION)"],
  [pro("Strict decrease plus well-foundedness is the complete classical termination argument (Floyd/Turing variant method)","each recursive call on a strictly smaller sublist guarantees the descent bottoms out")],
  [con("A measure that decreases on some but not all call sites proves nothing; one non-decreasing edge breaks it","a fast path that recurses on the same size leaves a non-decreasing step")],
  ["a call site where the rank does not strictly decrease","non-strict (<=) decrease mistaken for strict (<)"],
  ["every recursive call site exhibits rank(callee) < rank(caller) in the chosen order","the decrease is strict, not merely non-increasing"],
  ["a new recursive call site is introduced","a non-decreasing step is found in a trace"],
  specialists=["verification_engineer","logic_specialist"]))

N.append(node("MEASURE_COMPOSITION","composite_lexicographic_multiset_measures",
  "Compose measures for multi-argument or structurally rich recursion using lexicographic products, multiset orderings (Dershowitz-Manna), or product/ordinal combinations, lifting component well-foundedness to the composite.",
  "core", ["RANK_FUNCTION"], ["CQ_06"],
  b(0.85,0.74,0.7,0.8,0.84,0.8,0.6,0.58,0.58, 0.68,0.72, 0.85,0.58,0.24,0.56,[0.3,0.7],"the number/structure of recursion-driving arguments changes"),
  ["lexicographic product","multiset extension (Dershowitz-Manna)","ordinal/product combination","well-foundedness lifting lemma"],
  ["single-argument simple measures","the syntactic guard"],
  [pro("Lexicographic and multiset orderings prove terminations no single scalar can, while staying well-founded","Ackermann needs lexicographic (m,n); rewrite systems need the multiset order over redexes")],
  [con("Composite measures multiply proof obligations: each component and the lift must be discharged, raising cost sharply","a 3-tuple lexicographic measure needs decrease/no-increase reasoning across all three positions per call")],
  ["wrong component dominates the lexicographic comparison","multiset extension applied to a non-well-founded base"],
  ["the composite order is shown well-founded from its components","each call decreases the composite in the defined sense"],
  ["argument structure changes","a single scalar measure is found to suffice (simplify) or to fail (escalate)"],
  specialists=["verification_engineer","rewriting_specialist"],
  subfields=["term_rewriting","order_theory"]))

# ---- L3 ----
N.append(node("MONOTONICITY_CHECK","monotone_decrease_verification",
  "Verify the claimed decrease is genuinely monotone with respect to the order on every step and across composition: no step increases or holds the rank, and the comparison operator matches the order's direction.",
  "verification", ["PROGRESS_MEASURE","WELLFOUND_ORDER"], ["CQ_05","CQ_07"],
  b(0.83,0.72,0.68,0.66,0.82,0.78,0.58,0.64,0.55, 0.74,0.78, 0.83,0.64,0.2,0.5,[0.22,0.54],"the comparison operator or the order's direction convention changes"),
  ["per-step monotonicity audit","operator/direction consistency","cross-composition monotonicity"],
  ["constructing a non-termination witness","fuel accounting"],
  [pro("An explicit monotonicity audit catches the most common termination bug: a sign or <=/< slip that voids the descent","flags a step proven with <= that the proof treated as <")],
  [con("Monotonicity over a coarse abstraction can pass while the concrete step does not strictly decrease","abstracting two distinct sizes to one bucket hides a non-decreasing concrete step")],
  ["off-by-direction comparison (proving increase)","a held-rank step counted as a decrease"],
  ["every recursive step is verified to strictly move down the order","comparison direction matches the order definition"],
  ["order direction convention changes","a non-monotone step is detected"]))

N.append(node("TERMINATION_OBLIGATION","per_call_site_discharge_obligation",
  "Emit and discharge one explicit termination proof obligation per recursive call site: a checkable goal of the form 'this call's rank is strictly below the enclosing rank in the order', tracked to closed.",
  "core", ["PROGRESS_MEASURE","BASE_CASE"], ["CQ_08"],
  b(0.9,0.8,0.76,0.68,0.88,0.85,0.66,0.6,0.58, 0.7,0.76, 0.9,0.6,0.2,0.58,[0.26,0.62],"the set of recursive call sites or the obligation schema changes"),
  ["one obligation per call site","obligation status tracking (open/discharged)","obligation-to-proof linkage"],
  ["the certificate envelope","fuel budgeting"],
  [pro("Per-call-site obligations make termination a finite, enumerable checklist rather than a global hand-wave","each of the 4 recursive calls becomes a named, individually discharged lemma")],
  [con("Obligations proliferate with call sites and higher-order calls, and an unlisted indirect call leaves a silent gap","a call hidden behind a function-valued parameter never gets an obligation")],
  ["a recursive call site with no emitted obligation","an obligation marked discharged without a real proof"],
  ["every recursive call site has an emitted obligation","every obligation is discharged or routed to a defeater"],
  ["a call site is added or discovered","an obligation is found discharged without proof"],
  specialists=["verification_engineer","planning_engineer"]))

# ---- L4 ----
N.append(node("GUARD_PLACEMENT","decreasing_check_guard_placement",
  "Place recursion guards / decreasing-checks at each call site that assert the variant strictly decreased before the call proceeds, turning a static obligation into a runtime-enforceable guard.",
  "control", ["TERMINATION_OBLIGATION"], ["CQ_08","CQ_09"],
  b(0.8,0.72,0.7,0.62,0.8,0.74,0.6,0.66,0.5, 0.78,0.8, 0.8,0.66,0.2,0.5,[0.2,0.5],"the guard insertion policy or call-site instrumentation changes"),
  ["guard at each recursive call site","decrease assertion","guard-violation defeater"],
  ["the depth cap (DEPTH_CAP)","witness construction"],
  [pro("A decreasing-check guard converts a violated variant from silent divergence into an immediate, localized failure","the guard fires the instant a call is issued with a non-smaller rank, naming the offending site")],
  [con("Guards add per-call overhead and a too-loose guard (<= instead of <) admits the very non-termination it should stop","a guard checking 'not larger' lets equal-rank loops slip through")],
  ["guard uses non-strict comparison","a call site instrumented without a guard"],
  ["each recursive call site carries a strict-decrease guard","guard violation raises a handleable defeater"],
  ["guard policy changes","a guard with non-strict comparison is found"]))

N.append(node("MUTUAL_RECURSION","mutual_indirect_recursion_measure",
  "Handle mutual and indirect recursion by a shared or summed/tagged measure that strictly decreases around every cycle in the call graph, not merely within a single function.",
  "core", ["MEASURE_COMPOSITION","TERMINATION_OBLIGATION"], ["CQ_06","CQ_10"],
  b(0.84,0.74,0.68,0.78,0.85,0.82,0.62,0.56,0.6, 0.68,0.72, 0.84,0.56,0.24,0.58,[0.3,0.7],"the call graph's strongly connected components change"),
  ["call-graph SCC analysis","shared/summed/tagged measure","decrease around every cycle"],
  ["single-function termination","syntactic guard"],
  [pro("A measure that decreases around each call-graph cycle proves mutual recursion that per-function measures cannot","f calls g calls f: a tag-lexicographic measure decreases over the f->g->f cycle")],
  [con("Indirect recursion through higher-order or dynamic dispatch can hide a cycle the SCC analysis never sees","a callback re-enters the recursion via a registered handler, invisible to static call-graph extraction")],
  ["a call-graph cycle with no decreasing shared measure","an indirect/dynamic call edge missing from the graph"],
  ["every strongly connected component of the call graph has a measure that strictly decreases around it","indirect edges are captured"],
  ["call graph topology changes","a hidden indirect recursion cycle is discovered"],
  specialists=["verification_engineer","program_analysis_specialist"]))

# ---- L5 ----
N.append(node("DEPTH_CAP","hard_depth_bound_fallback",
  "Impose a hard maximum recursion/decomposition depth as a fallback bound that guarantees halting even when a variant proof is absent or incomplete, raising a defeater on breach.",
  "control", ["GUARD_PLACEMENT"], ["CQ_09","CQ_11"],
  b(0.8,0.74,0.72,0.56,0.82,0.74,0.62,0.68,0.55, 0.8,0.82, 0.8,0.68,0.18,0.55,[0.18,0.46],"the depth-cap value or fallback policy changes"),
  ["hard max-depth constant","breach defeater","interaction with variant proof"],
  ["proving genuine well-foundedness","fixpoint convergence"],
  [pro("A depth cap is a soundness backstop: it guarantees the engine halts regardless of proof completeness","depth>64 halts and reports, so an unproven recursion can never hang the session")],
  [con("A cap tight enough to be safe can truncate a legitimately deep but terminating recursion, hurting completeness","a correct 80-level decomposition is cut at a cap of 64 and reported as a false defeater")],
  ["cap below the true required depth (false defeater)","cap so high it never bounds real blow-up in time"],
  ["recursion depth is bounded by an explicit hard cap","a breach raises a defeater rather than crashing"],
  ["depth-cap policy changes","a valid deep recursion is truncated by the cap"]))

N.append(node("FUEL_BUDGET","fuel_step_budget_defeater",
  "Allocate a fuel / step budget consumed by each recursive step so that exhaustion converts suspected non-termination into an explicit, catchable defeater rather than an unbounded hang.",
  "control", ["DEPTH_CAP"], ["CQ_11","CQ_12"],
  b(0.82,0.76,0.74,0.6,0.84,0.78,0.62,0.64,0.58, 0.78,0.8, 0.82,0.64,0.2,0.56,[0.2,0.5],"the fuel allocation formula or accounting granularity changes"),
  ["per-step fuel decrement","exhaustion-as-defeater","fuel accounting granularity"],
  ["proving the variant decreases","oscillation pattern detection"],
  [pro("Fuel makes non-termination observable and recoverable: the session is bounded by construction (a step budget that always runs out)","budget 10000 steps means the worst case is a defeater after 10000 steps, never a hang")],
  [con("Fuel that is too small cuts off a slow-but-terminating computation early, producing a soundness-irrelevant false defeater","a correct computation needing 12000 steps is killed at a 10000-step budget")],
  ["budget too small (early cutoff of valid runs)","fuel accounting misses steps, so the bound is not real"],
  ["every recursive step consumes fuel and exhaustion raises a defeater","the budget bounds the whole session"],
  ["fuel allocation policy changes","a valid computation is cut off by fuel exhaustion"],
  specialists=["verification_engineer","reliability_engineer"]))

# ---- L6 ----
N.append(node("OSCILLATION_DETECT","non_decreasing_cycle_oscillation_detection",
  "Detect non-decreasing cycles and oscillation: recurring states or rank values that never strictly descend, signalling the variant is not actually decreasing on some loop.",
  "diagnosis", ["MONOTONICITY_CHECK"], ["CQ_07","CQ_13"],
  b(0.78,0.7,0.68,0.7,0.8,0.76,0.55,0.6,0.58, 0.72,0.74, 0.78,0.6,0.24,0.5,[0.26,0.6],"the state-equality or rank-fingerprint definition used to detect cycles changes"),
  ["repeated-state / repeated-rank detection","cycle fingerprinting","stall (non-strict) detection"],
  ["the depth/fuel bound","the formal variant proof"],
  [pro("Catching a repeated rank or state pinpoints exactly where the descent stalls, far more precisely than a bare timeout","spotting rank revisiting value 7 twice localizes the non-decreasing loop")],
  [con("State-hashing for cycle detection can false-positive on benign revisits and costs memory proportional to history","a memoized re-derivation revisits a state legitimately and is flagged as oscillation")],
  ["benign revisit flagged as oscillation","a genuine stall missed because state fingerprint is too coarse"],
  ["a non-strictly-decreasing cycle is detected and localized to its call site","stalls are distinguished from progress"],
  ["state-equality definition changes","an oscillation escapes detection until fuel exhaustion"]))

N.append(node("CONVERGENCE_CRITERION","fixpoint_convergence_threshold",
  "Define a convergence criterion for fixpoint-style / iterative loops: a threshold or stable-difference test under which the iteration is declared converged and terminates, with the decreasing residual as its variant.",
  "control", ["MONOTONICITY_CHECK"], ["CQ_12","CQ_14"],
  b(0.78,0.72,0.7,0.7,0.8,0.74,0.58,0.6,0.55, 0.72,0.74, 0.78,0.6,0.24,0.5,[0.26,0.62],"the convergence threshold or residual metric changes"),
  ["convergence threshold","residual / stable-difference test","monotone-residual variant"],
  ["structural recursion variants","co-recursive productivity"],
  [pro("A convergence threshold gives iterative/fixpoint loops a principled stop that a structural variant cannot express","a numeric fixpoint stops when successive iterates differ by < epsilon")],
  [con("A threshold set too loose stops short of the true fixpoint; too tight may never be met under floating error and loops","epsilon below machine precision makes the residual test never fire")],
  ["threshold never met (effective non-termination)","premature convergence to a non-fixpoint"],
  ["the iteration terminates when the residual crosses the threshold","the residual provably decreases toward the threshold"],
  ["residual metric or threshold changes","an iteration fails to converge under the threshold"]))

N.append(node("BOUNDED_RETRY","bounded_retry_vs_unbounded_loop",
  "Replace unbounded retry/iteration loops with an explicitly bounded retry counter so that retry-style recursion (re-attempt on failure) terminates after a fixed number of attempts.",
  "control", ["FUEL_BUDGET"], ["CQ_12","CQ_15"],
  b(0.76,0.72,0.72,0.54,0.78,0.7,0.58,0.68,0.5, 0.8,0.82, 0.76,0.68,0.18,0.5,[0.18,0.46],"the retry-bound policy or backoff schedule changes"),
  ["explicit retry bound","attempt counter as variant","exhaustion defeater"],
  ["variant proofs over data structure","fixpoint convergence"],
  [pro("A retry counter is a trivially well-founded variant (counts down to zero), making retry loops provably finite","at most 5 attempts: the attempt count is a decreasing N-valued variant")],
  [con("A retry bound too low abandons a transiently failing operation that would have succeeded on the next attempt","giving up after 3 retries on a service that recovers on the 4th")],
  ["unbounded retry on a persistent failure","bound so low it abandons recoverable work"],
  ["every retry loop has an explicit decreasing attempt bound","exhaustion raises a defeater not a hang"],
  ["retry-bound policy changes","a recoverable operation is abandoned by a too-low bound"]))

# ---- L7 ----
N.append(node("DIVERGENCE_DEFEATER","suspected_divergence_as_defeater",
  "Treat suspected divergence (variant stall, oscillation, or budget exhaustion) as a first-class, handleable defeater that suspends the recursive plan and routes to repair, rather than an uncatchable hang.",
  "recovery", ["OSCILLATION_DETECT","FUEL_BUDGET"], ["CQ_13","CQ_16"],
  b(0.84,0.78,0.76,0.66,0.86,0.82,0.62,0.6,0.6, 0.72,0.76, 0.84,0.6,0.22,0.58,[0.26,0.62],"the divergence-signal taxonomy or defeater routing changes"),
  ["divergence signal aggregation","defeater object construction","routing to repair/backtrack"],
  ["constructing the formal witness","the certificate"],
  [pro("Promoting divergence to a defeater makes non-termination a recoverable planning event instead of a fatal hang","a stall signal suspends the subplan and hands a defeater to the planner's repair loop")],
  [con("A defeater raised on weak evidence (one slow step) aborts computations that would have terminated, trading completeness for safety","a single expensive step trips the defeater and discards a converging run")],
  ["divergence defeater raised on a converging computation","real divergence not aggregated into a defeater"],
  ["suspected divergence raises a structured defeater that suspends the plan","the defeater carries the offending site and signal"],
  ["divergence taxonomy changes","a divergence escaped as an uncatchable hang"],
  specialists=["verification_engineer","reliability_engineer"]))

N.append(node("PRODUCTIVITY","corecursive_productivity_guardedness",
  "For co-recursive / streaming processes that are not meant to terminate, prove productivity (guardedness): each step yields a finite observable prefix in finite time, the dual of termination for infinite output.",
  "core", ["MEASURE_SOUNDNESS","WELLFOUND_ORDER"], ["CQ_17"],
  b(0.8,0.7,0.66,0.82,0.82,0.78,0.6,0.54,0.58, 0.66,0.7, 0.8,0.54,0.26,0.56,[0.32,0.72],"the stream/codata semantics or guardedness criterion changes"),
  ["guardedness condition","finite-prefix-per-step proof","productivity (vs termination) distinction"],
  ["termination of finite recursion","depth caps on finite trees"],
  [pro("Productivity certifies useful non-terminating processes (streams, servers) that termination would wrongly reject (Bertot-Casteran guarded corecursion)","a guarded stream definition yields one element per step, productive though infinite")],
  [con("Productivity and termination pull opposite ways: forcing a termination measure on a stream rejects a correct process","demanding a decreasing variant on an event loop wrongly flags a productive server as non-terminating")],
  ["unguarded corecursion (no observable prefix per step)","termination criterion misapplied to a co-recursive process"],
  ["each co-recursive step is proven to emit a finite prefix in finite time","productive processes are not failed by termination checks"],
  ["codata semantics change","a process is found unproductive (stalls without output)"],
  specialists=["verification_engineer","type_theory_specialist"],
  subfields=["coinduction","type_theory"]))

# ---- L8 ----
N.append(node("NONTERM_WITNESS","non_termination_witness_construction",
  "When termination cannot be proven, construct an explicit witness/counterexample of non-termination: an infinite descending-free run, a recurrent state, or a non-decreasing cycle, as evidence the recursion truly diverges.",
  "diagnosis", ["DIVERGENCE_DEFEATER"], ["CQ_16","CQ_18"],
  b(0.82,0.72,0.7,0.78,0.84,0.78,0.6,0.56,0.6, 0.68,0.72, 0.82,0.56,0.26,0.56,[0.3,0.7],"the witness schema or what counts as a divergence proof changes"),
  ["recurrent-state / lasso witness","infinite-run construction","witness validity check"],
  ["the positive termination proof","the order selection"],
  [pro("A concrete non-termination witness turns a failed proof into actionable evidence: it shows the actual diverging run","a lasso (stem + repeated cycle with no rank decrease) demonstrates the loop is truly infinite")],
  [con("Constructing a sound witness is itself hard, and a spurious witness from an over-approximation falsely condemns a terminating recursion","an abstract cycle that no concrete run realizes wrongly labels the recursion non-terminating")],
  ["spurious witness from an over-approximate model","no witness found yet termination still unproven (inconclusive)"],
  ["a constructed witness is checked to be a genuine non-terminating run","spurious abstract witnesses are refuted"],
  ["witness schema changes","a witness is found to be spurious under concrete semantics"],
  specialists=["verification_engineer","model_checking_specialist"],
  subfields=["model_checking","termination_analysis"]))

# ---- L9 ----
N.append(node("TERMINATION_CERTIFICATE","machine_checkable_termination_certificate",
  "Assemble a machine-checkable termination certificate: the chosen order, ranking function, per-call-site decrease proofs, base cases, mutual-recursion coverage, and fallback bounds, bundled so an external checker can re-verify halting.",
  "certification", ["TERMINATION_OBLIGATION","MEASURE_SOUNDNESS","MONOTONICITY_CHECK","NONTERM_WITNESS"], ["CQ_18","CQ_19"],
  b(0.9,0.82,0.8,0.72,0.88,0.85,0.7,0.6,0.55, 0.74,0.8, 0.9,0.6,0.2,0.6,[0.24,0.58],"the certificate schema or the external checker's interface changes"),
  ["order + rank + decrease proofs bundle","base-case and mutual-recursion coverage","fallback-bound record","re-checkable envelope"],
  ["the decomposition method","contract composition"],
  [pro("A self-contained certificate lets a cheap external checker re-verify termination without trusting the prover (proof-carrying termination)","the certificate replays each call-site decrease so a small kernel re-checks the whole proof")],
  [con("A certificate that omits a call site or the well-foundedness witness is unsound yet looks complete; assembly is the last place gaps hide","a certificate missing the mutual-recursion SCC coverage certifies a recursion that can still loop")],
  ["certificate missing a call site or the order's well-foundedness witness","certificate not independently re-checkable"],
  ["the certificate bundles order, rank, every discharged obligation, base cases and fallbacks","an external checker re-verifies it independently"],
  ["certificate schema changes","an external re-check of the certificate fails"],
  specialists=["verification_engineer","proof_engineer"],
  subfields=["proof_carrying_code","certified_termination"]))

# ---- Competency questions (12-14; <=14) ----
CQ = [
 ("CQ_01","How is a ranking/variant function defined and what well-founded order does it map into?",["nodes","glossary"],"RANK_FUNCTION defines the variant; WELLFOUND_ORDER selects the order",["RANK_FUNCTION","WELLFOUND_ORDER"]),
 ("CQ_02","Is the ranking function total and defined on every reachable state?",["nodes"],"TOTALITY_CHECK establishes totality over the reachable set",["TOTALITY_CHECK","RANK_FUNCTION"]),
 ("CQ_03","How are base cases identified and shown to be the minimal elements of the order?",["nodes"],"BASE_CASE proves base cases coincide with minimal elements",["BASE_CASE"]),
 ("CQ_04","Is the measure sound with respect to the operational semantics of the engine?",["nodes"],"MEASURE_SOUNDNESS ties the proof rank to the executed rank",["MEASURE_SOUNDNESS"]),
 ("CQ_05","How is strict, monotone decrease proven on every recursive step?",["nodes","workflow"],"PROGRESS_MEASURE proves strict decrease; MONOTONICITY_CHECK audits monotonicity",["PROGRESS_MEASURE","MONOTONICITY_CHECK"]),
 ("CQ_06","How are composite (lexicographic/multiset) and mutual-recursion measures built and lifted?",["nodes"],"MEASURE_COMPOSITION composes measures; MUTUAL_RECURSION covers call-graph cycles",["MEASURE_COMPOSITION","MUTUAL_RECURSION"]),
 ("CQ_07","How is a non-decreasing or oscillating step detected and localized?",["nodes","conflict_axes"],"MONOTONICITY_CHECK audits decrease; OSCILLATION_DETECT localizes stalls",["MONOTONICITY_CHECK","OSCILLATION_DETECT"]),
 ("CQ_08","How is a per-call-site termination obligation emitted, guarded, and discharged?",["nodes","workflow"],"TERMINATION_OBLIGATION emits obligations; GUARD_PLACEMENT enforces decrease at call sites",["TERMINATION_OBLIGATION","GUARD_PLACEMENT"]),
 ("CQ_09","What guards and hard depth bounds backstop the recursion?",["nodes"],"GUARD_PLACEMENT and DEPTH_CAP provide guard and fallback bounds",["GUARD_PLACEMENT","DEPTH_CAP"]),
 ("CQ_10","How is mutual / indirect recursion handled across the call graph?",["nodes"],"MUTUAL_RECURSION builds a measure decreasing around every SCC cycle",["MUTUAL_RECURSION"]),
 ("CQ_11","How does a fuel/step budget bound the session and convert non-termination into a defeater?",["nodes","iteration_protocol"],"DEPTH_CAP and FUEL_BUDGET bound depth/steps and raise defeaters",["DEPTH_CAP","FUEL_BUDGET"]),
 ("CQ_12","How do bounded retry and fixpoint convergence criteria terminate iterative loops?",["nodes","workflow"],"BOUNDED_RETRY bounds attempts; CONVERGENCE_CRITERION stops fixpoint iteration; FUEL_BUDGET backstops",["BOUNDED_RETRY","CONVERGENCE_CRITERION","FUEL_BUDGET"]),
 ("CQ_13","How is suspected divergence detected and turned into a handleable defeater?",["nodes","workflow"],"OSCILLATION_DETECT detects stalls; DIVERGENCE_DEFEATER promotes them to defeaters",["OSCILLATION_DETECT","DIVERGENCE_DEFEATER"]),
 ("CQ_14","How are convergence and productivity distinguished from termination for iterative and infinite processes?",["nodes"],"CONVERGENCE_CRITERION and PRODUCTIVITY treat fixpoint and co-recursive cases",["CONVERGENCE_CRITERION","PRODUCTIVITY"]),
]
# add up to 14 (we have 14 with the next two); keep <=14
CQ += [
 ("CQ_15","When should a retry loop be bounded rather than left unbounded?",["nodes"],"BOUNDED_RETRY replaces unbounded retry with a decreasing attempt bound",["BOUNDED_RETRY"]),
 ("CQ_16","How is a non-termination witness constructed when no proof is found and how does it relate to the divergence defeater?",["nodes"],"NONTERM_WITNESS constructs a counterexample; DIVERGENCE_DEFEATER routes it",["NONTERM_WITNESS","DIVERGENCE_DEFEATER"]),
]
# That is 16 -> exceeds 14. Trim to 14 by dropping CQ_15/CQ_16 into existing themes is messy;
# instead keep exactly 14 by removing the two extras and folding their nodes into existing CQs.
CQ = CQ[:14]

CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

# Re-map node -> CQ refs onto the 14-CQ set so every node refs 1-2 and every CQ is covered.
CQ_MAP = {
 "WELLFOUND_ORDER":["CQ_01"], "RANK_FUNCTION":["CQ_01","CQ_02"], "BASE_CASE":["CQ_03"],
 "TOTALITY_CHECK":["CQ_02"], "MEASURE_SOUNDNESS":["CQ_04"], "PROGRESS_MEASURE":["CQ_05"],
 "MEASURE_COMPOSITION":["CQ_06"], "MONOTONICITY_CHECK":["CQ_05","CQ_07"],
 "TERMINATION_OBLIGATION":["CQ_08"], "GUARD_PLACEMENT":["CQ_08","CQ_09"],
 "MUTUAL_RECURSION":["CQ_06","CQ_10"], "DEPTH_CAP":["CQ_09","CQ_11"],
 "FUEL_BUDGET":["CQ_11","CQ_12"], "OSCILLATION_DETECT":["CQ_07","CQ_13"],
 "CONVERGENCE_CRITERION":["CQ_12","CQ_14"], "BOUNDED_RETRY":["CQ_12"],
 "DIVERGENCE_DEFEATER":["CQ_13"], "PRODUCTIVITY":["CQ_14"],
 "NONTERM_WITNESS":["CQ_13"], "TERMINATION_CERTIFICATE":["CQ_04","CQ_08"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

# ---- glossary ----
GL = [
 ("well_founded_order","a binary relation with no infinite strictly descending chain, the foundation of every termination argument",["noetherian_order","well_founded_relation"],["total_order","partial_order_without_minimality"],["WELLFOUND_ORDER","BASE_CASE"]),
 ("ranking_function","a function from states/call-arguments into a well-founded set that strictly decreases on recursive calls",["variant_function","measure","progress_measure"],["cost_function","heuristic_estimate"],["RANK_FUNCTION","PROGRESS_MEASURE"]),
 ("lexicographic_order","the well-founded order on tuples comparing component-wise left to right, used for multi-argument recursion",["lex_order"],["pointwise_order"],["MEASURE_COMPOSITION","MUTUAL_RECURSION"]),
 ("multiset_ordering","the Dershowitz-Manna well-founded order on finite multisets, lifting a base order to bags of elements",["dershowitz_manna_order"],["sequence_order"],["MEASURE_COMPOSITION"]),
 ("termination_obligation","a per-call-site proof goal that the variant strictly decreases for that recursive call",["decrease_obligation","proof_obligation"],["postcondition","invariant"],["TERMINATION_OBLIGATION","GUARD_PLACEMENT"]),
 ("fuel","a finite step budget decremented per recursive step whose exhaustion forces a defeater",["step_budget","gas"],["depth_cap","timeout_wall_clock"],["FUEL_BUDGET","BOUNDED_RETRY"]),
 ("defeater","a structured, handleable signal that suspends a recursive plan when divergence is suspected",["suspension_signal","abort_with_reason"],["uncaught_hang","crash"],["DIVERGENCE_DEFEATER","FUEL_BUDGET"]),
 ("productivity","for co-recursion, the guarantee that each step yields a finite observable prefix in finite time",["guardedness"],["termination"],["PRODUCTIVITY"]),
 ("termination_certificate","a machine-checkable bundle of order, rank and per-call-site proofs that re-verifies halting",["proof_of_termination","ranking_certificate"],["test_pass_record"],["TERMINATION_CERTIFICATE"]),
]
GLS=[{"term":t,"definition":d,"synonyms":s,"not_same_as":ns,"used_by_nodes":u} for (t,d,s,ns,u) in GL]

# ---- edges ----
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
              "risk_of_conflict":"unmanaged tension degrades the termination argument","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated behavior",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror node.dependencies)
dep("WELLFOUND_ORDER","BASE_CASE",0.86)
dep("RANK_FUNCTION","TOTALITY_CHECK",0.82)
dep("RANK_FUNCTION","MEASURE_SOUNDNESS",0.8)
dep("RANK_FUNCTION","PROGRESS_MEASURE",0.9)
dep("WELLFOUND_ORDER","PROGRESS_MEASURE",0.86)
dep("RANK_FUNCTION","MEASURE_COMPOSITION",0.8)
dep("PROGRESS_MEASURE","MONOTONICITY_CHECK",0.84)
dep("WELLFOUND_ORDER","MONOTONICITY_CHECK",0.78)
dep("PROGRESS_MEASURE","TERMINATION_OBLIGATION",0.88)
dep("BASE_CASE","TERMINATION_OBLIGATION",0.8)
dep("TERMINATION_OBLIGATION","GUARD_PLACEMENT",0.82)
dep("MEASURE_COMPOSITION","MUTUAL_RECURSION",0.8)
dep("TERMINATION_OBLIGATION","MUTUAL_RECURSION",0.78)
dep("GUARD_PLACEMENT","DEPTH_CAP",0.8)
dep("DEPTH_CAP","FUEL_BUDGET",0.84)
dep("MONOTONICITY_CHECK","OSCILLATION_DETECT",0.8)
dep("MONOTONICITY_CHECK","CONVERGENCE_CRITERION",0.74)
dep("FUEL_BUDGET","BOUNDED_RETRY",0.78)
dep("OSCILLATION_DETECT","DIVERGENCE_DEFEATER",0.82)
dep("FUEL_BUDGET","DIVERGENCE_DEFEATER",0.78)
dep("MEASURE_SOUNDNESS","PRODUCTIVITY",0.78)
dep("WELLFOUND_ORDER","PRODUCTIVITY",0.72)
dep("DIVERGENCE_DEFEATER","NONTERM_WITNESS",0.8)
dep("TERMINATION_OBLIGATION","TERMINATION_CERTIFICATE",0.86)
dep("MEASURE_SOUNDNESS","TERMINATION_CERTIFICATE",0.82)
dep("MONOTONICITY_CHECK","TERMINATION_CERTIFICATE",0.8)
dep("NONTERM_WITNESS","TERMINATION_CERTIFICATE",0.72)

# cross-cutting non-dependency edges
rel("FUEL_BUDGET","DEPTH_CAP","similarity",0.74,why="both are fallback session bounds; fuel is step-based, depth-cap is depth-based")
rel("DIVERGENCE_DEFEATER","TERMINATION_OBLIGATION","feedback",0.76,why="an undischargeable obligation feeds back as a divergence defeater")
rel("OSCILLATION_DETECT","PROGRESS_MEASURE","feedback",0.74,why="a detected non-decreasing cycle refutes the claimed strict-decrease measure")
rel("TOTALITY_CHECK","PROGRESS_MEASURE","constraint",0.72,why="the decrease proof is only valid where the measure is total")
rel("MUTUAL_RECURSION","GUARD_PLACEMENT","sequence",0.72,why="guards must be placed around every call-graph cycle, not just direct self-calls")
rel("BOUNDED_RETRY","CONVERGENCE_CRITERION","similarity",0.7,why="both terminate iterative loops: one by attempt count, one by residual threshold")
rel("NONTERM_WITNESS","BASE_CASE","causal",0.68,why="a missing/mis-placed base case is a common root cause a non-termination witness exposes")

# conflict edges (negative signed_tension + resolution_rule)
conf("DEPTH_CAP","TERMINATION_CERTIFICATE",0.7,-0.6,
  "prefer a proven variant over the depth cap: the cap is a soundness backstop, not a substitute for the certificate; raise the cap only when the certificate shows the truncated depth is genuinely required and reachable",
  "a tight depth cap guarantees halting but can truncate a terminating-yet-deep recursion, undermining the completeness the certificate aims to establish",cp=0.55,erc=0.55)
conf("FUEL_BUDGET","MEASURE_SOUNDNESS",0.7,-0.55,
  "size fuel above the soundly-proven worst-case bound when a variant exists, so exhaustion can only mean genuine divergence; use a small fuel only as a last-resort cutoff and label exhaustion as inconclusive, not as proof of non-termination",
  "a small fuel budget can cut off a sound, terminating computation early, so fuel exhaustion is not by itself a sound non-termination conclusion",cp=0.55,erc=0.55)
conf("MEASURE_COMPOSITION","TERMINATION_OBLIGATION",0.68,-0.5,
  "use the least expressive measure that discharges all obligations: try a scalar variant first; escalate to lexicographic then multiset only when an obligation cannot be discharged, accepting the higher proof cost only where needed",
  "richer lexicographic/multiset measures discharge more obligations but multiply the per-call-site proof cost, trading expressiveness against proof effort",cp=0.5,erc=0.55)
conf("PRODUCTIVITY","PROGRESS_MEASURE",0.68,-0.6,
  "classify the process first: require a strictly decreasing variant (PROGRESS_MEASURE) for processes that must terminate, and require guardedness/productivity instead for intentionally infinite co-recursive processes; never impose a termination measure on a productive stream",
  "termination via a decreasing measure and productivity for infinite output are dual and mutually exclusive goals; applying the wrong one rejects a correct process",cp=0.5,erc=0.5)

# ---- conflict_axes (8-10; author 9) ----
CA=[
 {"name":"depth_cap_tightness_vs_completeness","description":"A tight hard depth cap guarantees halting but can truncate a legitimately deep terminating recursion; a loose cap risks long blow-up before tripping.","poles":["tight_safe_cap","full_completeness"],"resolution_hint":"keep the cap as a backstop; raise it only against a proven, reachable required depth","tension_score":0.72,"affected_nodes":["DEPTH_CAP","TERMINATION_CERTIFICATE","FUEL_BUDGET"]},
 {"name":"fuel_soundness_vs_early_cutoff","description":"Fuel converts non-termination into a defeater, but a small budget cuts off slow-yet-terminating runs and a large one delays the defeater.","poles":["sound_large_budget","cheap_early_cutoff"],"resolution_hint":"size fuel above the proven worst case where a variant exists; label exhaustion inconclusive otherwise","tension_score":0.7,"affected_nodes":["FUEL_BUDGET","MEASURE_SOUNDNESS","DIVERGENCE_DEFEATER"]},
 {"name":"measure_expressiveness_vs_proof_cost","description":"Lexicographic/multiset measures prove more terminations than scalars but multiply per-call-site proof obligations.","poles":["expressive_composite_measure","low_proof_cost"],"resolution_hint":"use the least expressive measure that discharges all obligations; escalate only on failure","tension_score":0.68,"affected_nodes":["MEASURE_COMPOSITION","TERMINATION_OBLIGATION","MUTUAL_RECURSION"]},
 {"name":"termination_vs_productivity","description":"Finite recursion needs a decreasing variant; infinite co-recursion needs productivity instead. The two criteria are dual and exclusive.","poles":["require_termination","require_productivity"],"resolution_hint":"classify the process first; apply the matching criterion, never both","tension_score":0.7,"affected_nodes":["PRODUCTIVITY","PROGRESS_MEASURE","WELLFOUND_ORDER"]},
 {"name":"static_proof_vs_runtime_guard","description":"A static variant proof gives certainty before running but costs proof effort; runtime decreasing-check guards are cheap to add but only fail at run time.","poles":["static_certificate","runtime_guard"],"resolution_hint":"prove statically where feasible; keep guards as a runtime backstop and defeater source","tension_score":0.6,"affected_nodes":["PROGRESS_MEASURE","GUARD_PLACEMENT","TERMINATION_CERTIFICATE"]},
 {"name":"reachable_set_over_vs_under_approximation","description":"Over-approximating reachable states forces proving hard unreachable cases; under-approximating leaves a real state with an undefined or undecreased rank.","poles":["over_approximate_safe","under_approximate_cheap"],"resolution_hint":"approximate tightly with justification; recheck on reachable-set changes","tension_score":0.62,"affected_nodes":["TOTALITY_CHECK","MEASURE_SOUNDNESS","PROGRESS_MEASURE"]},
 {"name":"convergence_threshold_loose_vs_tight","description":"A loose fixpoint convergence threshold stops short of the true fixpoint; a tight one may never be met under numerical error and effectively loops.","poles":["loose_threshold","tight_threshold"],"resolution_hint":"set the threshold from the residual metric's known precision; back it with fuel","tension_score":0.6,"affected_nodes":["CONVERGENCE_CRITERION","FUEL_BUDGET","MONOTONICITY_CHECK"]},
 {"name":"eager_defeater_vs_completeness","description":"Raising a divergence defeater on weak evidence aborts computations that would have terminated; waiting for strong evidence risks longer hangs.","poles":["eager_defeater","wait_for_strong_evidence"],"resolution_hint":"require a corroborated signal (oscillation plus budget pressure) before defeating","tension_score":0.64,"affected_nodes":["DIVERGENCE_DEFEATER","OSCILLATION_DETECT","NONTERM_WITNESS"]},
 {"name":"witness_soundness_vs_construction_cost","description":"A concrete non-termination witness is strong evidence but expensive to build soundly; an abstract one is cheap but can be spurious.","poles":["sound_concrete_witness","cheap_abstract_witness"],"resolution_hint":"build abstract first to localize, then concretize before condemning the recursion","tension_score":0.58,"affected_nodes":["NONTERM_WITNESS","OSCILLATION_DETECT","DIVERGENCE_DEFEATER"]},
]

# ---- edge_cases (10-12; author 12) ----
EC=[
 {"description":"A recursive call passes the same argument, so the variant does not strictly decrease and the recursion runs forever.","trigger":"a call site recurses with rank(callee) = rank(caller)","affected_nodes":["PROGRESS_MEASURE","MONOTONICITY_CHECK","TERMINATION_OBLIGATION"],"mitigation":"require strict (<) decrease per call site; a held-rank step is a discharge failure","severity":"critical"},
 {"description":"The ranking function maps into a dense or unbounded-below order, so 'decreasing' admits an infinite chain and proves nothing.","trigger":"codomain is (Q>=0,<) or similar non-well-founded order","affected_nodes":["WELLFOUND_ORDER","RANK_FUNCTION"],"mitigation":"map into a proven well-founded order (N, ordinals, lexicographic/multiset)","severity":"critical"},
 {"description":"A reachable state has no defined rank, so the decrease proof is vacuous exactly where it is needed.","trigger":"rank is partial and undefined on a reachable argument","affected_nodes":["TOTALITY_CHECK","PROGRESS_MEASURE"],"mitigation":"prove totality over the reachable set or exclude unreachable states with justification","severity":"high"},
 {"description":"Mutual recursion through f->g->f decreases within neither function alone, so per-function measures miss the divergence.","trigger":"a call-graph cycle with no shared decreasing measure","affected_nodes":["MUTUAL_RECURSION","TERMINATION_OBLIGATION"],"mitigation":"build a shared/tagged measure that decreases around every SCC of the call graph","severity":"high"},
 {"description":"Indirect recursion via a higher-order callback re-enters the recursion through an edge the static call graph never recorded.","trigger":"a dynamic/higher-order call edge missing from the analyzed graph","affected_nodes":["MUTUAL_RECURSION","MEASURE_SOUNDNESS"],"mitigation":"capture indirect edges; guard at runtime where static capture is impossible","severity":"high"},
 {"description":"A hard depth cap truncates a correct but deep decomposition, reporting a false non-termination defeater.","trigger":"depth cap set below the true required depth","affected_nodes":["DEPTH_CAP","TERMINATION_CERTIFICATE"],"mitigation":"raise the cap only when the certificate proves the depth is required and reachable","severity":"medium"},
 {"description":"A small fuel budget kills a slow-but-terminating computation, and exhaustion is misread as proof of non-termination.","trigger":"fuel below the proven worst-case step count","affected_nodes":["FUEL_BUDGET","MEASURE_SOUNDNESS","DIVERGENCE_DEFEATER"],"mitigation":"size fuel above the worst case where a variant exists; label bare exhaustion inconclusive","severity":"medium"},
 {"description":"A fixpoint iteration's convergence threshold is below machine precision, so the residual test never fires and the loop runs until fuel exhaustion.","trigger":"convergence epsilon smaller than the residual metric's precision","affected_nodes":["CONVERGENCE_CRITERION","FUEL_BUDGET"],"mitigation":"derive the threshold from the metric's known precision; backstop with fuel","severity":"medium"},
 {"description":"A productive infinite process (event loop / stream) is wrongly failed because a termination measure was demanded of it.","trigger":"a co-recursive process subjected to a decreasing-variant check","affected_nodes":["PRODUCTIVITY","PROGRESS_MEASURE"],"mitigation":"classify as co-recursive and require guardedness/productivity, not termination","severity":"medium"},
 {"description":"A non-termination witness is built from an over-approximate model and condemns a recursion that actually terminates.","trigger":"an abstract cycle with no concrete realizing run is accepted as a witness","affected_nodes":["NONTERM_WITNESS","OSCILLATION_DETECT"],"mitigation":"concretize the abstract witness before condemning; refute spurious ones","severity":"medium"},
 {"description":"The abstraction the proof reasons over diverges from the executed semantics, so a discharged obligation does not actually constrain the run.","trigger":"the proof rank differs from the rank the interpreter computes at the call","affected_nodes":["MEASURE_SOUNDNESS","TERMINATION_CERTIFICATE"],"mitigation":"prove the proof rank equals the executed rank; justify every abstraction","severity":"high"},
 {"description":"The assembled certificate omits one recursive call site, so it certifies a recursion that can still loop through the missing site.","trigger":"a call site has no obligation in the certificate bundle","affected_nodes":["TERMINATION_CERTIFICATE","TERMINATION_OBLIGATION","GUARD_PLACEMENT"],"mitigation":"enumerate all call sites (including indirect) before sealing the certificate","severity":"high"},
]

# ---- workflow (9-12; author 12) ----
WF=[
 {"action":"select_well_founded_order","node_ref":"WELLFOUND_ORDER","description":"Choose and justify a well-founded order (N, ordinals, lexicographic, or multiset) with no infinite descending chain.","artifact":"order_justification","gate":"the order is proven or cited to have no infinite descent"},
 {"action":"define_ranking_function","node_ref":"RANK_FUNCTION","description":"Define the variant function from reachable states/arguments into the order's carrier.","artifact":"rank_definition","gate":"rank reads the recursion-driving arguments and lands in the carrier"},
 {"action":"check_totality","node_ref":"TOTALITY_CHECK","description":"Prove the rank is total on every reachable state; exclude unreachable states with justification.","artifact":"totality_proof","gate":"no reachable state has an undefined rank"},
 {"action":"prove_base_cases","node_ref":"BASE_CASE","description":"Show base cases coincide with the order's minimal elements and issue no recursive call.","artifact":"base_case_proof","gate":"every minimal-rank reachable state is a non-recursing base case"},
 {"action":"prove_strict_decrease","node_ref":"PROGRESS_MEASURE","description":"Prove rank(callee) < rank(caller) strictly, in the order, at every recursive call site.","artifact":"decrease_lemmas","gate":"every call site exhibits strict decrease"},
 {"action":"compose_and_cover_mutual","node_ref":"MUTUAL_RECURSION","description":"Compose lexicographic/multiset measures and ensure decrease around every call-graph cycle.","artifact":"composite_measure","gate":"every SCC has a decreasing shared measure"},
 {"action":"audit_monotonicity","node_ref":"MONOTONICITY_CHECK","description":"Audit that each step strictly moves down the order with the correct comparison direction.","artifact":"monotonicity_report","gate":"no non-decreasing or wrong-direction step remains"},
 {"action":"emit_and_guard_obligations","node_ref":"TERMINATION_OBLIGATION","description":"Emit one obligation per call site and place strict-decrease guards at each.","artifact":"obligation_ledger","gate":"every call site has an emitted, discharged-or-routed obligation"},
 {"action":"install_fallback_bounds","node_ref":"FUEL_BUDGET","description":"Install the depth cap and fuel budget as backstops that raise defeaters on breach.","artifact":"bound_config","gate":"depth and step bounds are configured and breach-defeating"},
 {"action":"detect_divergence","node_ref":"OSCILLATION_DETECT","description":"Detect non-decreasing cycles/oscillation and aggregate signals for the defeater.","artifact":"divergence_signals","gate":"stalls are detected and localized to a call site"},
 {"action":"defeat_or_witness","node_ref":"DIVERGENCE_DEFEATER","description":"Raise a divergence defeater on corroborated signals; construct a witness when proof fails.","artifact":"defeater_or_witness","gate":"divergence is a handleable defeater, not a hang"},
 {"action":"assemble_certificate","node_ref":"TERMINATION_CERTIFICATE","description":"Bundle order, rank, per-call-site proofs, base cases, mutual coverage and fallbacks into a re-checkable certificate.","artifact":"termination_certificate","gate":"an external checker re-verifies the certificate"},
]

# ---- dominance_rules (7-12; author 9) ----
DR=[
 {"rule":"WELLFOUND_ORDER must be proven well-founded before any decrease proof is accepted","rationale":"a decrease in a non-well-founded order proves nothing","trigger":"a decrease lemma is accepted against an unjustified order","action":"block acceptance until the order's no-infinite-descent is proven or cited"},
 {"rule":"PROGRESS_MEASURE must show strict (<) decrease at every call site, not <=","rationale":"a single non-strict or non-decreasing call site voids the whole argument","trigger":"a call site proven with non-strict decrease","action":"reject and require a strictly decreasing measure or a defeater"},
 {"rule":"TOTALITY_CHECK must pass before PROGRESS_MEASURE is certified","rationale":"a decrease proof is vacuous where the rank is undefined","trigger":"strict-decrease claimed on a partial rank","action":"require totality on the reachable set first"},
 {"rule":"MUTUAL_RECURSION coverage must decrease around every call-graph SCC before the certificate seals","rationale":"per-function decrease misses mutual/indirect divergence","trigger":"an SCC with no shared decreasing measure","action":"require a shared/tagged measure per SCC"},
 {"rule":"A proven variant dominates the DEPTH_CAP and FUEL_BUDGET fallbacks for the completeness verdict","rationale":"fallbacks bound the session but a tight bound can falsely truncate a terminating run","trigger":"a fallback breach reported as non-termination while a variant exists","action":"treat the breach as inconclusive when a sound variant is present"},
 {"rule":"PRODUCTIVITY, not a termination measure, governs intentionally infinite co-recursive processes","rationale":"termination and productivity are dual; the wrong one rejects a correct process","trigger":"a decreasing-variant check applied to a co-recursive process","action":"reclassify and require guardedness instead"},
 {"rule":"FUEL_BUDGET exhaustion alone is not a sound proof of non-termination","rationale":"a small budget can cut off a slow terminating run","trigger":"exhaustion reported as proven divergence","action":"label exhaustion inconclusive and require a witness or larger budget"},
 {"rule":"DIVERGENCE_DEFEATER requires a corroborated signal before aborting a running computation","rationale":"defeating on weak evidence kills computations that would terminate","trigger":"a defeater raised on a single slow step","action":"require oscillation plus budget pressure (or a witness) before defeating"},
 {"rule":"TERMINATION_CERTIFICATE must enumerate every call site, including indirect ones, before sealing","rationale":"one missing site certifies a recursion that can still loop","trigger":"a call site absent from the obligation ledger","action":"block sealing until all sites (direct and indirect) are covered"},
]

# ---- anti_rework_rules (7-12; author 9) ----
ARR=[
 {"rule":"Do not write decrease proofs before fixing the well-founded order; changing the order invalidates every decrease lemma","prevents":"re-proving all per-call-site decreases after a late order change"},
 {"rule":"Do not define the ranking function before identifying all recursion-driving arguments; a rank missing one must be redefined","prevents":"redefining the variant and redoing every decrease proof"},
 {"rule":"Do not certify decrease before totality; a partial rank discovered late forces re-checking every step","prevents":"re-auditing all decrease lemmas after a totality gap surfaces"},
 {"rule":"Do not size the fuel budget below the proven worst case; raising it after false defeaters re-runs the analysis","prevents":"re-tuning fuel and re-running runs killed by a too-small budget"},
 {"rule":"Do not set the depth cap without certificate feedback; a too-tight cap surfaces as false defeaters late","prevents":"re-planning after a valid deep recursion is truncated"},
 {"rule":"Do not impose a termination measure on a co-recursive process; reclassifying after the fact discards the measure work","prevents":"discarding a termination measure built for a process that needed productivity"},
 {"rule":"Do not skip mutual-recursion SCC coverage; discovering an uncovered cycle late forces a measure redesign","prevents":"redesigning the measure after an indirect-recursion divergence is found"},
 {"rule":"Do not seal the certificate with an incomplete call-site enumeration; a missing site forces re-opening and re-checking","prevents":"re-opening a sealed certificate to add a missed call site"},
 {"rule":"Do not accept an abstract non-termination witness without concretization; a spurious witness wrongly condemns and triggers needless redesign","prevents":"redesigning a recursion that actually terminated"},
]

# ---- iteration_protocol (6-10; author 8) ----
IP=[
 {"trigger":"a recursive step is found that does not strictly decrease the rank","action":"strengthen the measure in RANK_FUNCTION/PROGRESS_MEASURE or escalate MEASURE_COMPOSITION to a lexicographic/multiset order","nodes":["PROGRESS_MEASURE","RANK_FUNCTION","MEASURE_COMPOSITION"],"priority":"critical"},
 {"trigger":"the chosen order is shown not to be well-founded","action":"reselect a well-founded order in WELLFOUND_ORDER and re-base BASE_CASE on its minimal elements","nodes":["WELLFOUND_ORDER","BASE_CASE"],"priority":"critical"},
 {"trigger":"a reachable state with an undefined rank is observed","action":"extend totality in TOTALITY_CHECK and re-verify the decrease where it was vacuous","nodes":["TOTALITY_CHECK","PROGRESS_MEASURE"],"priority":"high"},
 {"trigger":"a mutual/indirect recursion cycle escapes the per-function measures","action":"recompute call-graph SCCs and build a shared measure in MUTUAL_RECURSION","nodes":["MUTUAL_RECURSION","MEASURE_COMPOSITION"],"priority":"high"},
 {"trigger":"a fallback bound trips on a computation that actually terminates","action":"raise the bound in DEPTH_CAP/FUEL_BUDGET only against a proven required depth/step count","nodes":["DEPTH_CAP","FUEL_BUDGET"],"priority":"medium"},
 {"trigger":"oscillation is detected without budget exhaustion","action":"corroborate in OSCILLATION_DETECT and raise a structured defeater via DIVERGENCE_DEFEATER","nodes":["OSCILLATION_DETECT","DIVERGENCE_DEFEATER"],"priority":"medium"},
 {"trigger":"a non-termination witness is suspected spurious","action":"concretize and re-check the witness in NONTERM_WITNESS against the operational semantics","nodes":["NONTERM_WITNESS","MEASURE_SOUNDNESS"],"priority":"medium"},
 {"trigger":"an external re-check of the certificate fails","action":"reopen TERMINATION_CERTIFICATE, find the missing/weak obligation, and re-discharge it","nodes":["TERMINATION_CERTIFICATE","TERMINATION_OBLIGATION"],"priority":"high"},
]

spec = {
 "domain":"term__well_foundedness",
 "domain_label":"Termination & Well-Foundedness for Recursive Plans (Recursive Planning Engine subdomain)",
 "purpose":"session_bounded_method_for_proving_a_recursive_decomposition_or_plan_terminates_by_choosing_a_well_founded_ranking_or_variant_function_discharging_a_per_call_site_termination_obligation_for_every_recursive_step_bounding_depth_with_a_fuel_or_step_budget_treating_suspected_divergence_as_a_handleable_defeater_and_assembling_a_machine_checkable_termination_certificate",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "scope is termination and well-foundedness only; the decomposition method itself and contract composition are delegated to sibling KBs",
   "the recursive plan exposes its recursive call sites and recursion-driving arguments to the prover",
   "a model of reachable states / call arguments and the engine's operational semantics is available",
 ],
 "exclusions":[
   "the hierarchical decomposition method itself (delegated to htn__decomposition)",
   "interface contracts and assume/guarantee contract composition (delegated to a contract-composition KB)",
   "operator implementation internals and runtime scheduling",
   "general program correctness beyond halting (partial-correctness, functional verification)",
 ],
 "source_description":"heuristic prior estimates for termination and well-foundedness work units, informed by classical termination and well-founded-set theory; no supplied dataset or benchmark",
 "source_citation":"Turing 1949 'Checking a Large Routine'; Floyd 1967 'Assigning Meanings to Programs' (well-founded sets); Dershowitz & Manna 1979 'Proving Termination with Multiset Orderings'; Bertot & Casteran 2004 'Interactive Theorem Proving and Program Development' (well-founded and guarded recursion); Cousot & Cousot, ranking-function / termination analysis",
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
 "priority_rationale":"WELLFOUND_ORDER and RANK_FUNCTION are foundational; TOTALITY_CHECK, BASE_CASE and MEASURE_SOUNDNESS underwrite the core PROGRESS_MEASURE and its composition; obligations, guards and mutual-recursion coverage discharge per call site; depth/fuel bounds, oscillation detection and the divergence defeater backstop the session; the machine-checkable TERMINATION_CERTIFICATE seals the proof last.",
 "eval_objective":"verify_well_founded_order_selection_variant_decrease_per_call_site_obligation_discharge_fuel_and_depth_bounding_divergence_defeating_and_certificate_assembly_of_term__well_foundedness_kb",
}

out_dir = "branches/b13_recursive_planner/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "term__well_foundedness.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
