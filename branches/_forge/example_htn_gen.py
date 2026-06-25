#!/usr/bin/env python3
"""Generate the htn__decomposition content spec (B13 KB1) for kb_forge.py.
Compact authoring: node() applies sane defaults so only domain content + base
metric magnitudes are specified per node."""
import json, os

SRC = "SRC_HEURISTIC_PRIOR"

def node(id, topic, definition, group, deps, cqs, base,
         scope_in, scope_out, pros, cons, fmodes, accept, revisit,
         inputs=None, outputs=None, specialists=None, contradictors=None,
         subfields=None):
    # base: dict with the 9 metrics + AT/DG + prob(PI,EC,FR,DW) + ui + update_signal
    b = dict(base)
    return {
        "id": id, "topic": topic, "definition": definition, "group": group,
        "node_type": "work_unit",
        "academic_fields": ["automated_planning", "ai_methods"],
        "subfields": subfields or ["hierarchical_task_networks", "recursive_decomposition"],
        "specialists": specialists or ["planning_engineer"],
        "contradictors": contradictors or ["flat_planner_advocate"],
        "inputs": inputs or ["task spec", "world/state model"],
        "outputs": outputs or ["refined subtask set"],
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
N.append(node("GOAL_INTAKE","root_goal_capture_and_typing",
  "Capture the root goal/task and attach a type signature, success criterion, and the world/state model the decomposition will refine against.",
  "foundations", [], ["CQ_01"],
  b(0.9,0.8,0.82,0.55,0.82,0.78,0.6,0.7,0.4, 0.78,0.82, 0.88,0.7,0.18,0.5,[0.2,0.5],"goal grammar or success-criterion definition changes"),
  ["root goal statement","success criterion","initial state model"],
  ["primitive ground operator binding","method authoring"],
  [pro("A typed, criterion-bearing goal makes every later refinement checkable against a fixed target","goal 'ship release' typed as compound with acceptance = all gates green")],
  [con("Premature goal typing can over-constrain decomposition before the space is understood","typing a research goal as a fixed pipeline hides exploratory subgoals")],
  ["untyped goal admits no termination or coverage check","success criterion conflated with method"],
  ["goal has an explicit type and machine-checkable success criterion"],
  ["goal grammar revised","new success-criterion class added"],
  specialists=["planning_engineer","requirements_analyst"]))

N.append(node("METHOD_LIBRARY","decomposition_method_catalog",
  "Maintain the catalog of decomposition methods: for each compound task type, the alternative expansions (subtask templates) with their preconditions and applicability conditions.",
  "foundations", [], ["CQ_02","CQ_03"],
  b(0.85,0.78,0.7,0.62,0.78,0.8,0.62,0.66,0.45, 0.74,0.78, 0.84,0.66,0.2,0.5,[0.22,0.55],"new task type added or a method's preconditions change"),
  ["method templates","applicability conditions","alternative expansions per type"],
  ["method selection at runtime","primitive execution"],
  [pro("A first-class method library separates 'what expansions exist' from 'which to pick now', enabling reuse and audit","one 'deploy' type with three methods: blue-green, canary, rolling")],
  [con("Library drift: stale methods whose preconditions no longer match the world model","a method assumes a tool that was removed")],
  ["unindexed methods cause exponential selection cost","duplicate methods with subtly different preconditions"],
  ["every compound type has at least one applicable method indexed by type"],
  ["new task type introduced","method precondition vocabulary changes"],
  specialists=["planning_engineer","knowledge_engineer"]))

N.append(node("TASK_TYPING","primitive_vs_compound_typing",
  "Classify each task node as primitive (directly executable) or compound (must be expanded), and assign its type signature so arity and method selection are determined by type.",
  "foundations", ["GOAL_INTAKE"], ["CQ_01","CQ_04"],
  b(0.88,0.76,0.74,0.6,0.84,0.82,0.6,0.68,0.5, 0.76,0.8, 0.86,0.68,0.18,0.52,[0.2,0.5],"type system or primitive/compound boundary redefined"),
  ["primitive/compound classification","type signature assignment"],
  ["operator implementation","scheduling"],
  [pro("Type-driven typing makes decomposition obligations explicit and lets arity be checked, not guessed","a 'transfer' type fixes exactly {debit, credit, reconcile} subtasks")],
  [con("A coarse type system forces unnatural classifications","binary primitive/compound hides 'semi-primitive' tasks needing a tool call")],
  ["mis-typing a compound as primitive yields an unexecutable leaf","type signature too loose to constrain arity"],
  ["each task carries a resolved type; no leaf is left untyped"],
  ["new primitive operator class added","type confusion observed in trace"],
  specialists=["planning_engineer","type_designer"]))

N.append(node("ARITY_TYPING","type_driven_subtask_arity",
  "Derive the number and types of subtasks a compound task must produce from its type signature, so decomposition arity is a checkable obligation rather than an ad hoc choice.",
  "structure", ["TASK_TYPING"], ["CQ_04"],
  b(0.8,0.7,0.66,0.66,0.78,0.78,0.55,0.64,0.48, 0.74,0.76, 0.8,0.64,0.2,0.48,[0.24,0.56],"a task type's required subtask set is revised"),
  ["arity obligation per type","required vs optional subtask typing"],
  ["ordering of subtasks","parameter values"],
  [pro("Arity-as-obligation catches under-decomposition (a missing required subtask) deterministically","'transfer' missing 'reconcile' fails arity check before execution")],
  [con("Strict arity rejects legitimate variability when a type genuinely has variable fan-out","aggregating an unknown number of shards")],
  ["fixed arity blocks data-dependent fan-out","optional subtasks silently dropped"],
  ["arity obligations satisfied for every compound node"],
  ["variable-arity task type introduced","fan-out bound policy changes"]))

N.append(node("METHOD_SELECT","applicable_method_selection",
  "Select among the methods applicable to a compound task given current preconditions; rank by cost/confidence and record the choice plus the unselected alternatives for backtracking.",
  "selection", ["TASK_TYPING","METHOD_LIBRARY"], ["CQ_03","CQ_05"],
  b(0.86,0.78,0.72,0.7,0.82,0.82,0.6,0.62,0.55, 0.72,0.76, 0.84,0.62,0.22,0.52,[0.24,0.6],"selection policy or cost model changes"),
  ["applicability filtering","ranking of methods","recording alternatives"],
  ["method authoring","primitive grounding"],
  [pro("Recording unselected alternatives turns method choice into a recoverable decision, not a commit","keep canary as fallback when blue-green is selected")],
  [con("Greedy selection by local cost can pick a method that dead-ends deeper in the tree","cheapest expansion needs an unavailable resource two levels down")],
  ["no applicable method (selection gap)","selection ignores downstream feasibility"],
  ["at least one applicable method selected or an explicit no-method defeater is raised"],
  ["new method added to a type","downstream dead-ends traced to a selection"],
  specialists=["planning_engineer"]))

N.append(node("PRECOND_CHECK","method_precondition_verification",
  "Verify a candidate method's preconditions against the current world/state model before expansion, so methods are only applied where their assumptions hold.",
  "selection", ["METHOD_SELECT"], ["CQ_05","CQ_06"],
  b(0.82,0.72,0.68,0.64,0.82,0.76,0.55,0.66,0.5, 0.76,0.78, 0.8,0.66,0.2,0.5,[0.2,0.5],"world-model schema or precondition language changes"),
  ["precondition evaluation","state-model querying"],
  ["effect application","scheduling"],
  [pro("Checking preconditions before expansion prevents committing to infeasible subtrees","skip canary method when no traffic-splitter exists")],
  [con("Precondition checks are only as good as the world model; stale state yields false applicability","model says service is up but it is mid-restart")],
  ["precondition evaluated against stale state","unobservable precondition assumed true"],
  ["preconditions evaluate true against a freshly read state before expansion"],
  ["world-model freshness policy changes","false-applicability incident observed"]))

N.append(node("SUBTASK_EMIT","subtask_instantiation",
  "Instantiate the chosen method into concrete subtasks: create child task nodes, carry over parameters, and attach their type signatures and obligations.",
  "structure", ["PRECOND_CHECK","ARITY_TYPING"], ["CQ_06","CQ_07"],
  b(0.83,0.74,0.7,0.62,0.78,0.8,0.55,0.66,0.48, 0.76,0.78, 0.82,0.66,0.18,0.48,[0.2,0.5],"subtask instantiation contract changes"),
  ["child node creation","parameter carry-over","obligation attachment"],
  ["ordering constraints","conflict detection"],
  [pro("Explicit instantiation makes the refinement step a typed, inspectable artifact","each child records which method and binding produced it")],
  [con("Instantiation without provenance makes later repair and audit impossible","a child appears with no link to its parent method")],
  ["children created without obligations","parameters lost across instantiation"],
  ["each emitted subtask is typed and linked to its generating method"],
  ["instantiation contract revised","orphan subtask observed"]))

N.append(node("DECOMP_DAG","decomposition_graph_maintenance",
  "Maintain the decomposition as an acyclic refinement graph (tree/DAG): nodes are tasks, edges are refinement links; enforce that no task refines (transitively) into itself.",
  "structure", ["SUBTASK_EMIT"], ["CQ_07","CQ_08"],
  b(0.9,0.8,0.72,0.7,0.86,0.85,0.7,0.62,0.55, 0.72,0.76, 0.88,0.62,0.2,0.55,[0.24,0.6],"refinement-graph invariants change"),
  ["refinement edges","acyclicity invariant","shared-subtask DAG reuse"],
  ["execution scheduling","operator internals"],
  [pro("An explicit acyclic refinement graph makes recursion safe and enables shared-subtask reuse","two parents share one 'build-image' subtask without duplication")],
  [con("DAG reuse complicates ownership and parameter binding of shared subtasks","shared subtask needs different parameters per parent")],
  ["a refinement cycle (task expands into itself)","silent duplication instead of reuse"],
  ["refinement graph is acyclic at every step","shared subtasks are reused not duplicated"],
  ["refinement invariant changes","a cycle is detected in staging"],
  specialists=["planning_engineer","graph_engineer"]))

N.append(node("ORDERING_CONSTRAINTS","partial_order_and_causal_links",
  "Establish partial-order and causal-link constraints among subtasks (before/after, producer/consumer) so the decomposition is a partial order, not an over-committed total order.",
  "structure", ["SUBTASK_EMIT"], ["CQ_08","CQ_09"],
  b(0.8,0.72,0.68,0.7,0.78,0.78,0.55,0.62,0.55, 0.72,0.76, 0.8,0.62,0.22,0.5,[0.26,0.62],"ordering semantics or causal-link policy changes"),
  ["before/after constraints","causal links (producer->consumer)","threat detection on links"],
  ["resource scheduling","execution"],
  [pro("Least-commitment partial order preserves parallelism and avoids premature sequencing","two independent subtasks left unordered run concurrently downstream")],
  [con("Causal-link bookkeeping (threats/promotion/demotion) is intricate and error-prone","a new subtask threatens an existing producer-consumer link")],
  ["over-commitment to a total order kills parallelism","unprotected causal link clobbered by a later effect"],
  ["ordering is a consistent partial order with all causal links protected"],
  ["ordering semantics revised","a clobbered link traced in execution"]))

N.append(node("PARAM_BINDING","variable_binding_and_unification",
  "Bind variables and unify parameters across subtasks so outputs of one subtask flow as typed inputs to another, maintaining a consistent substitution.",
  "structure", ["SUBTASK_EMIT"], ["CQ_09"],
  b(0.78,0.7,0.66,0.68,0.76,0.74,0.55,0.64,0.5, 0.74,0.76, 0.78,0.64,0.2,0.46,[0.24,0.56],"binding/type-unification rules change"),
  ["parameter unification","type-consistent substitution","dataflow binding"],
  ["ordering","conflict arbitration"],
  [pro("Unification gives a single consistent substitution, catching type-incompatible dataflow early","binding a string output into an int-typed input fails at bind time")],
  [con("Aggressive early binding can over-constrain when a value is genuinely deferred","binding a region before the cost-optimizer has chosen one")],
  ["inconsistent substitution across subtasks","late binding hides a type error until execution"],
  ["all bound parameters are type-consistent under one substitution"],
  ["type-unification rules change","a binding type error reaches execution"]))

N.append(node("CONSTRAINT_PROP","constraint_propagation",
  "Propagate constraints up and down the decomposition (preconditions, resource limits, deadlines) so that a constraint introduced at one level is enforced across the affected subtree.",
  "reasoning", ["ORDERING_CONSTRAINTS","PARAM_BINDING"], ["CQ_10"],
  b(0.8,0.72,0.66,0.74,0.8,0.82,0.58,0.6,0.58, 0.7,0.74, 0.8,0.6,0.24,0.52,[0.28,0.66],"constraint language or propagation rules change"),
  ["downward refinement of constraints","upward aggregation of obligations","resource-limit propagation"],
  ["operator execution","external scheduling"],
  [pro("Propagation makes a parent-level deadline bind every descendant automatically","a 5-minute budget at the root caps the sum of leaf budgets")],
  [con("Propagation can over-tighten and make a satisfiable decomposition look infeasible","conservative interval arithmetic rejects a feasible schedule")],
  ["constraint added at one level not enforced at another","propagation diverges (oscillates)"],
  ["constraints are consistent across the affected subtree after propagation"],
  ["constraint language extended","an unpropagated constraint causes a late failure"]))

N.append(node("REFINE_LOOP","iterative_refinement_to_primitives",
  "Iteratively expand compound leaves until every leaf is primitive, interleaving method selection and precondition checks; this is the core recursive loop and must make monotonic progress.",
  "reasoning", ["DECOMP_DAG"], ["CQ_11","CQ_12"],
  b(0.92,0.82,0.78,0.72,0.88,0.85,0.68,0.6,0.6, 0.7,0.74, 0.9,0.6,0.24,0.58,[0.28,0.66],"refinement/termination interface changes"),
  ["compound-leaf expansion loop","progress measure","interleaved precondition checks"],
  ["termination proof (delegated)","operator execution"],
  [pro("A single refinement loop with a progress measure localizes all recursion in one auditable place","loop exits when |compound leaves| reaches 0")],
  [con("Without a ranking/measure the loop can expand forever on recursive task types","'simplify' that re-emits a 'simplify' subtask")],
  ["non-terminating expansion","progress measure not strictly decreasing"],
  ["refinement terminates with all leaves primitive on every test goal"],
  ["termination interface changes","a runaway expansion observed in staging"],
  specialists=["planning_engineer","reliability_engineer"]))

N.append(node("DECOMP_DEPTH_BUDGET","depth_and_breadth_bounds",
  "Bound decomposition depth and per-node breadth (fan-out) and expose the bound to the termination layer, so unbounded or explosive expansion is caught as a defeater, not a crash.",
  "control", ["REFINE_LOOP"], ["CQ_12","CQ_13"],
  b(0.82,0.72,0.66,0.62,0.84,0.78,0.6,0.66,0.55, 0.78,0.8, 0.82,0.66,0.2,0.55,[0.2,0.5],"depth/breadth budget policy changes"),
  ["max-depth cap","max-breadth cap","budget-exceeded defeater"],
  ["proof of termination (delegated to termination KB)"],
  [pro("An explicit budget converts silent blow-up into an explicit, handleable defeater","depth>12 raises a budget defeater routed to repair")],
  [con("A fixed budget can truncate a legitimately deep decomposition","a genuinely 15-level task is cut at 12")],
  ["budget too low truncates valid plans","budget too high lets blow-up run long before tripping"],
  ["every expansion path is bounded by an explicit depth and breadth budget"],
  ["budget policy changes","truncation of a valid plan observed"]))

N.append(node("CONFLICT_DETECT","subgoal_conflict_detection",
  "Detect inconsistencies across the decomposition: contradictory subgoals, resource clobbering, and mutually exclusive method choices, before they reach execution.",
  "reasoning", ["CONSTRAINT_PROP"], ["CQ_10","CQ_14"],
  b(0.84,0.74,0.7,0.72,0.85,0.82,0.6,0.6,0.62, 0.7,0.74, 0.84,0.6,0.24,0.56,[0.28,0.64],"conflict taxonomy or detection method changes"),
  ["contradiction detection","resource clobber detection","mutex method detection"],
  ["arbitration policy (delegated)","execution"],
  [pro("Detecting clobbering at decomposition time is far cheaper than at execution time","two subtasks both claim an exclusive lock are flagged pre-run")],
  [con("Detection sensitivity trades false positives against missed conflicts","semantic contradictions that are lexically dissimilar slip through")],
  ["undetected resource clobber","false-positive conflicts block valid plans"],
  ["all resource clobbers and contradictory subgoals are detected before execution"],
  ["conflict taxonomy extended","an undetected conflict reached execution"],
  specialists=["planning_engineer"]))

N.append(node("ALT_METHOD_BACKTRACK","alternative_method_backtracking",
  "On a detected dead-end or conflict, backtrack to the nearest decision with an untried alternative method and re-expand, rather than restarting the whole decomposition.",
  "recovery", ["CONFLICT_DETECT","METHOD_SELECT"], ["CQ_05","CQ_15"],
  b(0.8,0.72,0.66,0.74,0.82,0.8,0.6,0.58,0.6, 0.7,0.74, 0.8,0.58,0.26,0.55,[0.3,0.68],"backtracking policy or choice-point recording changes"),
  ["choice-point identification","alternative re-expansion","dead-end localization"],
  ["full restart","operator execution"],
  [pro("Chronological/dependency-directed backtracking reuses the valid prefix instead of discarding it","conflict at depth 6 re-tries the depth-5 method, keeping depths 0-4")],
  [con("Naive backtracking can thrash between two failing alternatives without progress","two methods alternately tried, both dead-ending")],
  ["thrashing between alternatives","backtrack target chosen too shallow or too deep"],
  ["backtracking reaches a consistent decomposition or exhausts alternatives with an explicit failure"],
  ["backtracking policy changes","thrashing observed in trace"]))

N.append(node("PARTIAL_PLAN_REPAIR","partial_decomposition_repair",
  "Repair a partial decomposition locally (replace a failing subtree, re-bind, re-order) instead of backtracking or restarting, when the failure is contained.",
  "recovery", ["CONFLICT_DETECT"], ["CQ_15"],
  b(0.78,0.7,0.68,0.74,0.8,0.78,0.58,0.58,0.58, 0.7,0.72, 0.78,0.58,0.26,0.52,[0.3,0.66],"repair scope or locality criteria change"),
  ["local subtree replacement","re-binding","re-ordering"],
  ["global restart","method authoring"],
  [pro("Local repair preserves most of the decomposition and converges faster than restart when failure is contained","swap one failing leaf's method, keep the rest")],
  [con("Repair can mask a structural problem that really needs backtracking","patching leaves repeatedly around a wrong high-level method")],
  ["repair loops without converging","repair masks a need for structural backtrack"],
  ["repair restores consistency or escalates to backtracking after a bounded number of attempts"],
  ["repair locality criteria change","repair-masking incident observed"]))

N.append(node("PRIMITIVE_GROUNDING","primitive_operator_binding",
  "Bind each primitive leaf to an executable operator/action with its concrete parameters and verify the operator's signature matches the leaf's type.",
  "grounding", ["REFINE_LOOP"], ["CQ_11","CQ_16"],
  b(0.86,0.78,0.74,0.64,0.85,0.78,0.65,0.66,0.5, 0.78,0.8, 0.86,0.66,0.2,0.55,[0.2,0.5],"operator catalog or grounding contract changes"),
  ["operator lookup","signature matching","parameter grounding"],
  ["operator implementation internals","scheduling"],
  [pro("Grounding with signature checks guarantees every leaf is actually executable","a 'send-email' leaf binds to an operator with matching typed params")],
  [con("A missing operator for a typed leaf reveals a gap that decomposition alone cannot fix","no operator exists for a required primitive")],
  ["leaf bound to a signature-mismatched operator","ungrounded primitive leaf"],
  ["every primitive leaf binds to a signature-matching executable operator"],
  ["operator catalog changes","an ungrounded leaf reached execution"],
  specialists=["planning_engineer","integration_engineer"]))

N.append(node("LEAF_VERIFICATION","leaf_executability_verification",
  "Verify that the set of grounded leaves is collectively executable and that each leaf's preconditions are establishable by the partial order, closing open conditions.",
  "verification", ["PRIMITIVE_GROUNDING"], ["CQ_16","CQ_17"],
  b(0.84,0.76,0.72,0.66,0.84,0.78,0.6,0.66,0.5, 0.78,0.82, 0.84,0.66,0.2,0.52,[0.2,0.5],"executability criteria change"),
  ["open-condition closure","collective executability check","precondition establishment"],
  ["runtime execution","monitoring"],
  [pro("Closing open conditions before execution turns 'looks done' into 'provably executable'","each leaf precondition is established by an earlier leaf's effect")],
  [con("Establishment proofs assume the world model is complete; unmodeled effects break them","an external actor changes state between plan and run")],
  ["open precondition left unestablished","executability assumed without checking effects"],
  ["no open preconditions remain; the grounded leaf set is collectively executable"],
  ["executability criteria change","an unestablished precondition failed at runtime"]))

N.append(node("DECOMP_TRACE","decomposition_provenance_trace",
  "Record the full provenance of the decomposition: for each node, the parent task, chosen method, binding, and the alternatives not taken, for audit, explanation, and repair.",
  "verification", ["DECOMP_DAG"], ["CQ_17","CQ_18"],
  b(0.76,0.7,0.74,0.58,0.7,0.76,0.55,0.7,0.42, 0.8,0.8, 0.76,0.7,0.16,0.42,[0.18,0.46],"provenance schema changes"),
  ["per-node provenance records","alternatives-not-taken log","explanation rendering"],
  ["execution","method selection logic"],
  [pro("A provenance trace makes the decomposition explainable and makes repair/backtracking precise","'why this subtask?' answered by parent+method+binding")],
  [con("Full provenance adds storage and write overhead proportional to tree size","a deep tree logs thousands of choice records")],
  ["missing provenance prevents targeted repair","trace volume overwhelms storage"],
  ["every node carries parent, method, binding and unselected-alternative provenance"],
  ["provenance schema changes","a repair failed for lack of provenance"],
  specialists=["planning_engineer","observability_engineer"]))

N.append(node("COVERAGE_CHECK","goal_coverage_verification",
  "Verify that the decomposition fully covers the root goal's success criterion: every obligation maps to at least one leaf, and no leaf is outside the goal's scope (set-cover closure).",
  "verification", ["LEAF_VERIFICATION","DECOMP_TRACE"], ["CQ_18","CQ_19"],
  b(0.88,0.8,0.78,0.66,0.86,0.82,0.66,0.64,0.5, 0.78,0.82, 0.88,0.64,0.2,0.56,[0.22,0.54],"goal-coverage definition changes"),
  ["obligation-to-leaf mapping","set-cover closure","scope-containment check"],
  ["execution","method authoring"],
  [pro("Set-cover closure makes 'the plan achieves the goal' a checkable property, not a hope","each success-criterion clause is covered by >=1 leaf")],
  [con("Coverage over the modeled criterion can still miss an unstated real-world obligation","criterion omits a compliance step that matters")],
  ["an uncovered obligation (missing leaf)","out-of-scope leaf doing unrequested work"],
  ["every success-criterion obligation is covered and no leaf is out of scope"],
  ["goal-coverage definition changes","an uncovered obligation reached production"],
  specialists=["planning_engineer","qa_engineer"]))

CQ = [
 ("CQ_01","How is the root goal captured and typed, and what is its success criterion?",["nodes","glossary"],"GOAL_INTAKE and TASK_TYPING define goal typing and success criterion",["GOAL_INTAKE","TASK_TYPING"]),
 ("CQ_02","What decomposition methods exist and how are they catalogued?",["nodes"],"METHOD_LIBRARY defines the catalog and applicability conditions",["METHOD_LIBRARY"]),
 ("CQ_03","How is a method selected among applicable alternatives, and how does selection recover on dead-ends?",["nodes","workflow"],"METHOD_SELECT ranks applicable methods; ALT_METHOD_BACKTRACK recovers",["METHOD_SELECT","METHOD_LIBRARY","ALT_METHOD_BACKTRACK"]),
 ("CQ_04","How is subtask arity determined and checked from a task's type?",["nodes"],"ARITY_TYPING derives the required subtask set from the type signature",["ARITY_TYPING","TASK_TYPING"]),
 ("CQ_05","How are method preconditions verified before expansion?",["nodes"],"PRECOND_CHECK evaluates preconditions against fresh state",["PRECOND_CHECK","METHOD_SELECT"]),
 ("CQ_06","How are concrete subtasks instantiated from a chosen method?",["nodes"],"SUBTASK_EMIT instantiates typed child tasks with provenance",["SUBTASK_EMIT","PRECOND_CHECK"]),
 ("CQ_07","How is the decomposition kept acyclic and how is shared-subtask reuse handled?",["nodes","edges"],"DECOMP_DAG maintains the acyclic refinement graph",["DECOMP_DAG","SUBTASK_EMIT"]),
 ("CQ_08","How are subtasks ordered, causal links protected, and parameters bound?",["nodes"],"ORDERING_CONSTRAINTS and PARAM_BINDING establish the partial order and bindings",["ORDERING_CONSTRAINTS","PARAM_BINDING","DECOMP_DAG"]),
 ("CQ_09","How are constraints propagated and conflicts/clobbers detected across levels?",["nodes","conflict_axes"],"CONSTRAINT_PROP propagates; CONFLICT_DETECT finds inconsistencies",["CONSTRAINT_PROP","CONFLICT_DETECT"]),
 ("CQ_10","What is the core refinement loop and how does it ground primitives?",["nodes","workflow"],"REFINE_LOOP expands to primitives; PRIMITIVE_GROUNDING binds operators",["REFINE_LOOP","PRIMITIVE_GROUNDING"]),
 ("CQ_11","How is decomposition depth/breadth bounded, what happens on budget exhaustion, and how does it interface with termination?",["nodes","iteration_protocol"],"DECOMP_DEPTH_BUDGET bounds expansion and raises defeaters",["DECOMP_DEPTH_BUDGET","REFINE_LOOP"]),
 ("CQ_12","How does the planner recover: backtracking versus bounded local repair?",["nodes","workflow"],"ALT_METHOD_BACKTRACK and PARTIAL_PLAN_REPAIR define recovery",["ALT_METHOD_BACKTRACK","PARTIAL_PLAN_REPAIR"]),
 ("CQ_13","How is each primitive leaf grounded, verified executable, and its provenance recorded?",["nodes"],"PRIMITIVE_GROUNDING grounds; LEAF_VERIFICATION closes open conditions; DECOMP_TRACE records provenance",["PRIMITIVE_GROUNDING","LEAF_VERIFICATION","DECOMP_TRACE"]),
 ("CQ_14","How is full goal coverage and scope containment verified?",["nodes","workflow"],"COVERAGE_CHECK verifies set-cover closure and that no leaf is out of scope",["COVERAGE_CHECK","DECOMP_TRACE"]),
]
CQS=[{"id":i,"question":q,"must_be_answerable_from":m,"acceptance_condition":a,"covered_by":c} for (i,q,m,a,c) in CQ]

# consolidate node->CQ references onto the 14-CQ set (dense target 10-14)
CQ_MAP = {
 "GOAL_INTAKE":["CQ_01"], "METHOD_LIBRARY":["CQ_02","CQ_03"], "TASK_TYPING":["CQ_01","CQ_04"],
 "ARITY_TYPING":["CQ_04"], "METHOD_SELECT":["CQ_03","CQ_05"], "PRECOND_CHECK":["CQ_05","CQ_06"],
 "SUBTASK_EMIT":["CQ_06","CQ_07"], "DECOMP_DAG":["CQ_07","CQ_08"], "ORDERING_CONSTRAINTS":["CQ_08"],
 "PARAM_BINDING":["CQ_08"], "CONSTRAINT_PROP":["CQ_09"], "REFINE_LOOP":["CQ_10","CQ_11"],
 "DECOMP_DEPTH_BUDGET":["CQ_11"], "CONFLICT_DETECT":["CQ_09"], "ALT_METHOD_BACKTRACK":["CQ_03","CQ_12"],
 "PARTIAL_PLAN_REPAIR":["CQ_12"], "PRIMITIVE_GROUNDING":["CQ_10","CQ_13"],
 "LEAF_VERIFICATION":["CQ_13","CQ_14"], "DECOMP_TRACE":["CQ_13","CQ_14"], "COVERAGE_CHECK":["CQ_14"],
}
for _n in N:
    _n["competency_question_refs"] = CQ_MAP[_n["id"]]

GL = [
 ("compound_task","a task that is not directly executable and must be expanded by a method into subtasks",["non_primitive_task"],["primitive_task"],["TASK_TYPING","SUBTASK_EMIT"]),
 ("primitive_task","a task directly executable by a single grounded operator/action",["leaf_task","operator_task"],["compound_task"],["TASK_TYPING","PRIMITIVE_GROUNDING"]),
 ("method","a named expansion of a compound task type into a typed subtask set with preconditions",["decomposition_method","expansion"],["operator"],["METHOD_LIBRARY","METHOD_SELECT"]),
 ("refinement_graph","the acyclic graph of tasks and refinement edges produced during decomposition",["decomposition_dag","refinement_tree"],["execution_graph"],["DECOMP_DAG","DECOMP_TRACE"]),
 ("causal_link","a producer->consumer protection between two subtasks over a condition",["protection_interval"],["ordering_only_edge"],["ORDERING_CONSTRAINTS","CONFLICT_DETECT"]),
 ("open_condition","a subtask precondition not yet established by another subtask's effect",["unsupported_precondition"],["closed_condition"],["LEAF_VERIFICATION","CONSTRAINT_PROP"]),
 ("clobbering","when a subtask's effect deletes a condition another subtask depends on",["link_threat","resource_clobber"],["independent_effects"],["CONFLICT_DETECT","ORDERING_CONSTRAINTS"]),
 ("choice_point","a recorded decision with untried alternatives used as a backtracking target",["decision_point","backtrack_point"],["committed_choice"],["METHOD_SELECT","ALT_METHOD_BACKTRACK"]),
 ("set_cover_closure","the property that every goal obligation is covered by at least one leaf",["coverage_closure"],["partial_coverage"],["COVERAGE_CHECK"]),
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
              "risk_of_conflict":"unmanaged tension degrades plan quality","example":"see resolution_rule"})
def rel(f,t,et,rs,cc=0.7,cp=0.2,erc=0.3,why=""):
    E.append({"from":f,"to":t,"edge_type":et,"relation_strength":rs,"signed_tension":0.0,
              "causal_confidence":cc,"conflict_probability":cp,"expected_rework_cost":erc,
              "why_related":why or f"{f} {et} {t}","benefit_of_coupling":"coordinated behavior",
              "risk_of_conflict":"inconsistency if uncoordinated","example":f"{f}/{t} {et} relation"})

# dependency edges (acyclic, mirror deps)
dep("GOAL_INTAKE","TASK_TYPING",0.9)
dep("TASK_TYPING","ARITY_TYPING",0.82)
dep("TASK_TYPING","METHOD_SELECT",0.84)
dep("METHOD_LIBRARY","METHOD_SELECT",0.8)
dep("METHOD_SELECT","PRECOND_CHECK",0.82)
dep("PRECOND_CHECK","SUBTASK_EMIT",0.82)
dep("ARITY_TYPING","SUBTASK_EMIT",0.78)
dep("SUBTASK_EMIT","DECOMP_DAG",0.85)
dep("SUBTASK_EMIT","ORDERING_CONSTRAINTS",0.8)
dep("SUBTASK_EMIT","PARAM_BINDING",0.78)
dep("DECOMP_DAG","REFINE_LOOP",0.86)
dep("ORDERING_CONSTRAINTS","CONSTRAINT_PROP",0.78)
dep("PARAM_BINDING","CONSTRAINT_PROP",0.76)
dep("REFINE_LOOP","PRIMITIVE_GROUNDING",0.85)
dep("REFINE_LOOP","DECOMP_DEPTH_BUDGET",0.8)
dep("CONSTRAINT_PROP","CONFLICT_DETECT",0.8)
dep("CONFLICT_DETECT","ALT_METHOD_BACKTRACK",0.8)
dep("METHOD_SELECT","ALT_METHOD_BACKTRACK",0.72)
dep("CONFLICT_DETECT","PARTIAL_PLAN_REPAIR",0.78)
dep("PRIMITIVE_GROUNDING","LEAF_VERIFICATION",0.84)
dep("DECOMP_DAG","DECOMP_TRACE",0.76)
dep("LEAF_VERIFICATION","COVERAGE_CHECK",0.84)
dep("DECOMP_TRACE","COVERAGE_CHECK",0.74)
# cross-cutting non-dependency edges
rel("DECOMP_DEPTH_BUDGET","REFINE_LOOP","feedback",0.78,why="budget defeater feeds back to halt/redirect the refinement loop")
rel("ALT_METHOD_BACKTRACK","METHOD_SELECT","feedback",0.76,why="backtracking re-invokes selection at a prior choice point")
rel("PARTIAL_PLAN_REPAIR","SUBTASK_EMIT","feedback",0.72,why="repair re-instantiates a replacement subtree")
rel("CONSTRAINT_PROP","PARAM_BINDING","constraint",0.74,why="propagated constraints restrict admissible bindings")
rel("ORDERING_CONSTRAINTS","CONFLICT_DETECT","causal",0.74,why="causal links are the objects whose threats CONFLICT_DETECT checks")
rel("ARITY_TYPING","COVERAGE_CHECK","constraint",0.72,why="arity obligations are part of the coverage obligation set")
rel("DECOMP_TRACE","ALT_METHOD_BACKTRACK","similarity",0.7,why="provenance supplies the choice points backtracking targets")
rel("DECOMP_TRACE","PARTIAL_PLAN_REPAIR","similarity",0.7,why="provenance localizes the subtree repair replaces")
# conflict edges (negative signed_tension + resolution_rule)
conf("METHOD_SELECT","REFINE_LOOP",0.7,-0.6,
  "prefer downstream-feasible methods: when greedy local selection dead-ends, REFINE_LOOP's failure backpropagates to re-rank selection by realized feasibility",
  "greedy least-cost method selection conflicts with global refinement feasibility")
conf("DECOMP_DEPTH_BUDGET","COVERAGE_CHECK",0.7,-0.55,
  "raise the depth budget only when COVERAGE_CHECK proves the truncated obligations are real; otherwise treat truncation as a defeater not a coverage failure",
  "a tight depth budget can truncate decomposition before full goal coverage is reached")
conf("ORDERING_CONSTRAINTS","REFINE_LOOP",0.66,-0.45,
  "least-commitment: keep subtasks unordered during refinement and add ordering only when a threat is detected, not eagerly",
  "eager ordering during refinement reduces the parallelism least-commitment aims to preserve")
conf("PARTIAL_PLAN_REPAIR","ALT_METHOD_BACKTRACK",0.66,-0.5,
  "try bounded local repair first; escalate to backtracking when repair attempts exceed the bound without restoring consistency",
  "local repair and backtracking compete as recovery strategies for the same failure")

CA=[
 {"name":"least_commitment_vs_eager_ordering","description":"Deferring ordering preserves parallelism but complicates threat tracking; eager ordering simplifies reasoning but over-commits.","poles":["least_commitment","eager_total_order"],"resolution_hint":"add ordering only to resolve a detected threat","tension_score":0.7,"affected_nodes":["ORDERING_CONSTRAINTS","REFINE_LOOP","CONFLICT_DETECT"]},
 {"name":"greedy_local_selection_vs_global_feasibility","description":"Greedy method selection is cheap but can dead-end deep in the tree where a globally better method was available.","poles":["greedy_local_cost","global_feasibility"],"resolution_hint":"record alternatives; backpropagate refinement failure to re-rank","tension_score":0.75,"affected_nodes":["METHOD_SELECT","REFINE_LOOP","ALT_METHOD_BACKTRACK"]},
 {"name":"depth_budget_vs_goal_coverage","description":"A tight depth/breadth budget bounds cost but can truncate before full coverage; a loose budget risks blow-up.","poles":["tight_budget","full_coverage"],"resolution_hint":"treat truncation as a defeater; raise budget only against proven real obligations","tension_score":0.72,"affected_nodes":["DECOMP_DEPTH_BUDGET","COVERAGE_CHECK","REFINE_LOOP"]},
 {"name":"backtracking_vs_local_repair","description":"Backtracking is sound but can discard valid work; local repair is fast but can mask structural errors.","poles":["backtrack_restart_prefix","local_repair"],"resolution_hint":"bounded repair first, then escalate to backtracking","tension_score":0.68,"affected_nodes":["ALT_METHOD_BACKTRACK","PARTIAL_PLAN_REPAIR","CONFLICT_DETECT"]},
 {"name":"early_binding_vs_deferred_binding","description":"Early parameter binding catches type errors sooner but over-constrains genuinely deferred values.","poles":["early_binding","least_commitment_binding"],"resolution_hint":"bind when a type or causal link forces it; otherwise defer","tension_score":0.62,"affected_nodes":["PARAM_BINDING","CONSTRAINT_PROP","SUBTASK_EMIT"]},
 {"name":"fixed_arity_vs_variable_fanout","description":"Type-fixed arity catches under-decomposition but rejects legitimate data-dependent fan-out.","poles":["fixed_arity","variable_fanout"],"resolution_hint":"model variable fan-out as a typed collection task with its own arity rule","tension_score":0.6,"affected_nodes":["ARITY_TYPING","TASK_TYPING","SUBTASK_EMIT"]},
 {"name":"shared_subtask_reuse_vs_binding_isolation","description":"DAG reuse of a shared subtask saves work but entangles per-parent parameter binding.","poles":["max_reuse","binding_isolation"],"resolution_hint":"reuse structure but clone binding context per parent","tension_score":0.6,"affected_nodes":["DECOMP_DAG","PARAM_BINDING","SUBTASK_EMIT"]},
 {"name":"provenance_completeness_vs_overhead","description":"Full provenance enables precise repair and explanation but costs storage proportional to tree size.","poles":["full_provenance","minimal_overhead"],"resolution_hint":"record decisions and alternatives; sample verbose intermediate state","tension_score":0.55,"affected_nodes":["DECOMP_TRACE","DECOMP_DAG","PARTIAL_PLAN_REPAIR"]},
 {"name":"world_model_trust_vs_recheck_cost","description":"Trusting the world model speeds precondition checks but risks acting on stale state.","poles":["trust_model","recheck_state"],"resolution_hint":"re-read volatile conditions just before expansion and execution","tension_score":0.65,"affected_nodes":["PRECOND_CHECK","LEAF_VERIFICATION","CONSTRAINT_PROP"]},
]

EC=[
 {"description":"Recursive task type re-emits itself, producing an infinite refinement chain.","trigger":"a method expands a compound task into a subtask of the same type with no progress measure","affected_nodes":["REFINE_LOOP","DECOMP_DAG","DECOMP_DEPTH_BUDGET"],"mitigation":"require a strictly decreasing progress measure; depth budget raises a defeater","severity":"critical"},
 {"description":"A required subtask is silently missing, so the plan looks complete but cannot achieve the goal.","trigger":"arity obligation not enforced for a compound task","affected_nodes":["ARITY_TYPING","COVERAGE_CHECK","SUBTASK_EMIT"],"mitigation":"enforce type-driven arity; coverage check maps obligations to leaves","severity":"high"},
 {"description":"Two subtasks clobber a shared resource detected only at execution.","trigger":"causal-link threat not checked at decomposition time","affected_nodes":["CONFLICT_DETECT","ORDERING_CONSTRAINTS"],"mitigation":"detect clobbering during decomposition; protect links by promotion/demotion","severity":"high"},
 {"description":"Greedy method selection dead-ends at depth and the planner restarts from scratch, discarding valid work.","trigger":"no choice points recorded for backtracking","affected_nodes":["METHOD_SELECT","ALT_METHOD_BACKTRACK"],"mitigation":"record alternatives at each selection; backtrack to nearest untried alternative","severity":"high"},
 {"description":"Depth budget truncates a legitimately deep decomposition, dropping real obligations.","trigger":"budget set below the true required depth","affected_nodes":["DECOMP_DEPTH_BUDGET","COVERAGE_CHECK"],"mitigation":"raise budget only when coverage proves the truncated obligations are real","severity":"medium"},
 {"description":"A primitive leaf has no matching operator, so the plan is unexecutable.","trigger":"operator catalog lacks an operator for a typed leaf","affected_nodes":["PRIMITIVE_GROUNDING","LEAF_VERIFICATION"],"mitigation":"signature-match grounding; raise a gap defeater when no operator exists","severity":"high"},
 {"description":"Local repair loops indefinitely, patching leaves around a wrong high-level method.","trigger":"repair attempts exceed bound without restoring consistency","affected_nodes":["PARTIAL_PLAN_REPAIR","ALT_METHOD_BACKTRACK"],"mitigation":"bound repair attempts; escalate to backtracking the higher method","severity":"medium"},
 {"description":"Stale world model makes a precondition look satisfied; expansion commits to an infeasible subtree.","trigger":"precondition evaluated against stale state","affected_nodes":["PRECOND_CHECK","CONSTRAINT_PROP"],"mitigation":"re-read volatile conditions immediately before expansion","severity":"high"},
 {"description":"Eager total ordering during refinement removes parallelism the plan needed to meet a deadline.","trigger":"ordering added before any threat required it","affected_nodes":["ORDERING_CONSTRAINTS","REFINE_LOOP"],"mitigation":"least-commitment ordering: add constraints only on detected threats","severity":"low"},
 {"description":"Shared (reused) subtask receives conflicting parameter bindings from two parents.","trigger":"DAG reuse without per-parent binding context","affected_nodes":["DECOMP_DAG","PARAM_BINDING"],"mitigation":"clone binding context per parent while sharing structure","severity":"medium"},
 {"description":"An open precondition is left unestablished and fails at runtime.","trigger":"leaf verification skipped for a closed-world assumption","affected_nodes":["LEAF_VERIFICATION","PRIMITIVE_GROUNDING"],"mitigation":"close all open conditions before execution; mark unmodeled effects","severity":"high"},
 {"description":"Provenance is absent, so a failed leaf cannot be repaired without a full restart.","trigger":"decomposition trace not recorded","affected_nodes":["DECOMP_TRACE","PARTIAL_PLAN_REPAIR"],"mitigation":"record parent/method/binding/alternatives for every node","severity":"medium"},
]

WF=[
 {"action":"capture_and_type_goal","node_ref":"GOAL_INTAKE","description":"Capture the root goal, attach a type and machine-checkable success criterion, and load the world model.","artifact":"typed_goal_record","gate":"goal has a type and success criterion"},
 {"action":"index_methods","node_ref":"METHOD_LIBRARY","description":"Ensure every compound type has at least one applicable, precondition-annotated method indexed.","artifact":"method_catalog","gate":"each compound type has >=1 indexed method"},
 {"action":"select_method","node_ref":"METHOD_SELECT","description":"Filter to applicable methods, rank by cost/confidence, record unselected alternatives as choice points.","artifact":"selection_record","gate":"a method is selected or a no-method defeater raised"},
 {"action":"check_preconditions","node_ref":"PRECOND_CHECK","description":"Evaluate the chosen method's preconditions against freshly read state before expansion.","artifact":"precondition_report","gate":"preconditions hold against current state"},
 {"action":"instantiate_subtasks","node_ref":"SUBTASK_EMIT","description":"Create typed child tasks with provenance and satisfy the type's arity obligation.","artifact":"subtask_set","gate":"arity obligation satisfied; children carry provenance"},
 {"action":"maintain_refinement_graph","node_ref":"DECOMP_DAG","description":"Insert refinement edges and assert acyclicity; reuse shared subtasks where sound.","artifact":"refinement_graph","gate":"graph is acyclic"},
 {"action":"order_and_bind","node_ref":"ORDERING_CONSTRAINTS","description":"Add least-commitment ordering and causal links; unify parameter bindings.","artifact":"partial_order_plus_bindings","gate":"partial order consistent; causal links protected"},
 {"action":"refine_to_primitives","node_ref":"REFINE_LOOP","description":"Expand compound leaves until all are primitive, bounded by the depth/breadth budget.","artifact":"primitive_leaf_set","gate":"all leaves primitive; progress measure strictly decreased"},
 {"action":"detect_and_recover","node_ref":"CONFLICT_DETECT","description":"Detect clobbers/contradictions; recover by bounded repair then backtracking.","artifact":"conflict_resolution_log","gate":"no unresolved conflicts remain"},
 {"action":"ground_primitives","node_ref":"PRIMITIVE_GROUNDING","description":"Bind each primitive leaf to a signature-matching operator.","artifact":"grounded_plan","gate":"every leaf bound to a matching operator"},
 {"action":"verify_executability","node_ref":"LEAF_VERIFICATION","description":"Close open conditions and confirm collective executability.","artifact":"executability_certificate","gate":"no open preconditions remain"},
 {"action":"verify_coverage","node_ref":"COVERAGE_CHECK","description":"Confirm set-cover closure of the success criterion and scope containment.","artifact":"coverage_report","gate":"all obligations covered; no out-of-scope leaf"},
]

DR=[
 {"rule":"GOAL_INTAKE typing and success criterion must be fixed before METHOD_SELECT runs","rationale":"selection and arity are derived from the goal type; selecting first causes rework","trigger":"method selection begins without a typed goal","action":"block until the goal has a type and success criterion"},
 {"rule":"REFINE_LOOP must carry a strictly decreasing progress measure or a depth budget before any cyclic task type is expanded","rationale":"recursive task types without a measure cause non-termination","trigger":"a self-recursive task type is expanded","action":"require a progress measure or budget defeater on the loop"},
 {"rule":"CONFLICT_DETECT must run before PRIMITIVE_GROUNDING commits operators","rationale":"clobbers are far cheaper to fix at decomposition time than at execution","trigger":"grounding starts with unresolved conflicts","action":"block grounding until conflicts are resolved"},
 {"rule":"ARITY_TYPING obligations must be satisfied before COVERAGE_CHECK can pass","rationale":"a missing required subtask is an uncovered obligation","trigger":"coverage check on a node with unmet arity","action":"fail coverage and route to SUBTASK_EMIT"},
 {"rule":"METHOD_SELECT must record unselected alternatives before expansion","rationale":"backtracking needs choice points; unrecorded alternatives force a full restart","trigger":"expansion without recorded alternatives","action":"require choice-point recording"},
 {"rule":"PRECOND_CHECK must read volatile conditions fresh immediately before expansion","rationale":"stale state causes commitment to infeasible subtrees","trigger":"precondition evaluated against cached state","action":"force a fresh read of volatile conditions"},
 {"rule":"PARTIAL_PLAN_REPAIR attempts must be bounded before escalation to ALT_METHOD_BACKTRACK","rationale":"unbounded repair masks structural errors","trigger":"repair attempt count exceeds bound","action":"escalate to backtracking the higher method"},
 {"rule":"LEAF_VERIFICATION must close all open conditions before execution is permitted","rationale":"open preconditions fail at runtime","trigger":"execution requested with open conditions","action":"block execution until conditions are closed"},
 {"rule":"DECOMP_DAG acyclicity invariant must hold after every refinement step","rationale":"a refinement cycle is non-terminating recursion","trigger":"a refinement edge would create a cycle","action":"reject the edge and raise a defeater"},
]

ARR=[
 {"rule":"Do not select methods before the goal is typed; type changes invalidate selection","prevents":"re-selecting all methods after a late goal re-typing"},
 {"rule":"Do not expand recursive task types without a progress measure; adding one after blow-up requires reworking the loop","prevents":"emergency loop-guard retrofit on a runaway decomposition"},
 {"rule":"Do not ground operators before conflict detection; clobbers found post-grounding require regrounding","prevents":"regrounding leaves after execution-time clobber discovery"},
 {"rule":"Do not eagerly totally-order subtasks; removing spurious ordering later is costly","prevents":"re-deriving parallelism after over-committed ordering"},
 {"rule":"Do not discard choice points during selection; reconstructing them for backtracking is expensive","prevents":"full restart when a deep dead-end has no recorded alternatives"},
 {"rule":"Do not reuse a shared subtask without per-parent binding context; untangling it later requires a graph edit","prevents":"binding-conflict cleanup on reused subtasks"},
 {"rule":"Do not skip provenance recording; retrofitting it for repair requires re-running decomposition","prevents":"blind repair with no parent/method/binding trace"},
 {"rule":"Do not set the depth budget without coverage feedback; truncation surfaces as missing obligations late","prevents":"re-planning after late discovery of truncated obligations"},
]

IP=[
 {"trigger":"refinement fails to terminate or progress measure stalls","action":"tighten the depth/breadth budget in DECOMP_DEPTH_BUDGET and audit recursive methods in REFINE_LOOP","nodes":["REFINE_LOOP","DECOMP_DEPTH_BUDGET"],"priority":"critical"},
 {"trigger":"a resource clobber reaches execution","action":"extend the conflict taxonomy in CONFLICT_DETECT and add link protection in ORDERING_CONSTRAINTS","nodes":["CONFLICT_DETECT","ORDERING_CONSTRAINTS"],"priority":"high"},
 {"trigger":"a deep dead-end causes a full restart","action":"verify choice points are recorded in METHOD_SELECT and reachable by ALT_METHOD_BACKTRACK","nodes":["METHOD_SELECT","ALT_METHOD_BACKTRACK"],"priority":"high"},
 {"trigger":"coverage check finds an uncovered obligation","action":"re-run ARITY_TYPING and SUBTASK_EMIT for the affected node","nodes":["COVERAGE_CHECK","ARITY_TYPING","SUBTASK_EMIT"],"priority":"high"},
 {"trigger":"a primitive leaf has no operator","action":"file an operator gap and add the operator before grounding in PRIMITIVE_GROUNDING","nodes":["PRIMITIVE_GROUNDING","LEAF_VERIFICATION"],"priority":"medium"},
 {"trigger":"repair loops without converging","action":"lower the repair bound in PARTIAL_PLAN_REPAIR and escalate to ALT_METHOD_BACKTRACK","nodes":["PARTIAL_PLAN_REPAIR","ALT_METHOD_BACKTRACK"],"priority":"medium"},
 {"trigger":"a stale-state false applicability is observed","action":"tighten volatile-condition freshness in PRECOND_CHECK and re-check in LEAF_VERIFICATION","nodes":["PRECOND_CHECK","LEAF_VERIFICATION"],"priority":"medium"},
]

spec = {
 "domain":"htn__decomposition",
 "domain_label":"Hierarchical & Recursive Task Decomposition (Recursive Planning Engine subdomain)",
 "purpose":"session_bounded_stepwise_method_for_recursively_decomposing_a_typed_goal_into_a_verified_acyclic_network_of_primitive_operators_with_type_driven_arity_least_commitment_ordering_conflict_detection_bounded_recovery_and_goal_coverage_closure",
 "assumptions":[
   "no observed dataset or benchmark evidence supplied; all scores are heuristic priors",
   "scope is the decomposition method itself; termination proofs and contract composition are delegated to sibling KBs",
   "a world/state model and an operator catalog are available to ground primitives",
   "tasks carry type signatures from which arity and method applicability are derived",
 ],
 "exclusions":[
   "formal termination proofs (delegated to term__well_foundedness)",
   "interface contracts and assume/guarantee composition (delegated to contract__composition_failure)",
   "operator implementation internals and runtime execution",
   "learning method libraries from data",
 ],
 "source_description":"heuristic prior estimates for hierarchical task decomposition work units, informed by classical HTN planning (Erol/Hendler/Nau SHOP/SHOP2), partial-order causal-link planning (UCPOP), and least-commitment planning; no supplied dataset",
 "source_citation":"Erol, Hendler & Nau 1994 HTN Planning; Nau et al. 2003 SHOP2; Weld 1994 An Introduction to Least-Commitment Planning; Ghallab, Nau & Traverso 2004 Automated Planning",
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
 "priority_rationale":"GOAL_INTAKE/TASK_TYPING/METHOD_LIBRARY are foundational; REFINE_LOOP and DECOMP_DAG are the recursion core; CONFLICT_DETECT and recovery gate correctness; verification and coverage close the method last.",
 "eval_objective":"verify_recursive_decomposition_typing_termination_interface_conflict_handling_and_goal_coverage_of_htn__decomposition_kb",
}

out_dir = "branches/b13_recursive_planner/kb/_src"
os.makedirs(out_dir, exist_ok=True)
path = os.path.join(out_dir, "htn__decomposition.spec.json")
open(path,"w").write(json.dumps(spec, indent=2))
print("wrote", path)
print("nodes",len(N),"edges",len(E),"CA",len(CA),"EC",len(EC),"WF",len(WF),"CQ",len(CQS),"DR",len(DR),"ARR",len(ARR),"IP",len(IP))
