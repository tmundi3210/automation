# B13 — Recursive Plan-Decomposition Engine (standalone)

A self-contained capability that turns one high-level **goal** into a **validated
recursive plan tree**. It composes the obligations of four gate-passed specialists —
`method` (validity gates), `loops` (termination / ranking functions / circuit-breakers),
`argue` (defeaters → justified fallbacks), and `appdev` (acceptance tests / pre-post
contracts) — into a single, machine-checkable quality gate.

It **structures and validates** plans. It does **not** execute them.

## Files

| File | What it is |
|------|------------|
| `plan.schema.json` | JSON Schema for a plan tree (plan_id, root, ranking_name, depth_cap, nodes[]). |
| `plan_validator.py` | Deterministic validator / quality gate. CLI, stdlib only. |
| `examples/plan.pass.json` | A small valid 2–3 level plan (decreasing ranking, contracts, fallbacks/SPOF tag, leaf acceptance tests). |
| `examples/plan.fail.json` | A plan that fails on specific rules (non-decreasing ranking on one edge **and** an untagged SPOF with no fallback → fatal break). |
| `test_plan_validator.py` | `unittest` suite: pass fixture passes, fail fixture fails and names the violated rules. |
| `specialist.json` | Injection-ready meta-specialist (`specialist_id: "planner"`) describing the engine. |

## Run

Validate a plan (exit 0 = PASS, nonzero = FAIL):

```bash
python3 plan_validator.py examples/plan.pass.json            # exit 0
python3 plan_validator.py examples/plan.fail.json            # exit 1, lists violated_rules
python3 plan_validator.py <plan.json> --report report.json   # also write a JSON report
python3 plan_validator.py <plan.json> --quiet                # suppress stdout, keep exit code
```

Run the tests:

```bash
python3 test_plan_validator.py
```

The validator mirrors the repo convention (`validators/kb_validator.py`): a `checks`
list, a JSON report, `--report` / `--quiet` flags, and an exit code (0 PASS, 1 hard
failure, 2 bad invocation).

## The four design corrections (encoded, not aspirational)

These are corrections from prior adversarial review; each is **enforced** by
`plan_validator.py`, not merely described:

1. **Type-driven arity.** A node's `type` determines how many children it decomposes
   into — there is no fixed binary "split into halves". The validator checks
   `arity == len(children)` (`schema.arity_matches_children`).

2. **Termination is a proof obligation, not an assumption.** Every parent→child edge
   must **either** strictly decrease the named `ranking_value` **or** fall under the
   explicit global `depth_cap` circuit-breaker, and no node may sit deeper than
   `depth_cap`. The validator actually checks the ranking decrease and the depth bound
   (`termination.ranking_decreases_or_depth_capped`, `termination.depth_within_cap`).

3. **Defeater-justified fallbacks, not blanket ones.** Every node must have **either**
   ≥1 fallback whose `trigger` names the defeater/failure condition it answers, **or**
   be tagged `single_point_of_failure: true` **with** a non-empty `spof_reason`. A SPOF
   node with **no** fallback on a root→leaf critical path is a **FATAL BREAK** — reported
   as a list (`fallback.defeater_justified_or_spof_tagged`,
   `fallback.spof_reason_present`, `fatal_break.spof_no_fallback_on_critical_path`).

4. **The quality gate validates against a defined standard.** Correctness = passing the
   validator: schema shape, tree integrity + **no cycles**, the termination obligation,
   per-node **contracts** (preconditions, postconditions, leaf acceptance tests), and a
   **contract-composition** check — the union of a parent's children's postcondition tags
   must **cover** the parent's postcondition tags (a tractable set-cover over declared
   tags) (`contract.composition_covers_parent`).

## What the validator checks (rule list)

- `schema.*` — top-level keys, node shape, id uniqueness, `arity == len(children)`.
- `tree.*` — single root, parent/child consistency, single parent per node, every node
  reaches the root, **acyclicity**.
- `termination.*` — ranking strictly decreases per edge **or** within `depth_cap`; no node
  exceeds `depth_cap`.
- `contract.*` — pre+postconditions on every node, ≥1 acceptance test on every leaf,
  contract composition (children cover parent postconditions).
- `fallback.*` — defeater-justified fallback **or** SPOF-tagged-with-reason on every node.
- `fatal_break.*` — SPOF-with-no-fallback on a root→leaf path, reported as a list.

## Boundary note

This engine is **standalone** and depends only on its own `plan.schema.json` and
`plan_validator.py` (plus the documented obligations of the composed specialists). It is
strictly a planning/validation capability:

- **In scope:** type-driven decomposition, per-node contracts, a proven termination
  bound, defeater-justified fallbacks / reasoned SPOF tags, and the validator gate.
- **Out of scope:** executing the plan, running steps, running real acceptance tests
  against a live system, asserting any acceptance test already passes, or treating
  heuristic ranking values as measured durations/costs. Domain-specific subject reasoning
  is delegated to the relevant domain specialist; this engine hands off execution.
