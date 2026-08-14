#!/usr/bin/env python3
"""
plan_validator.py -- deterministic validator for the B13 recursive
plan-decomposition engine.

This is the quality gate for a plan tree produced by the "planner" meta-specialist
(specialists/planner via branches/b13_recursive_planner/specialist.json). It composes
the obligations of four gate-passed specialists into one machine-checkable standard:

  method  -> validity gates: schema shape and tree integrity.
  loops   -> termination via a RANKING FUNCTION (strictly decreasing parent->child)
             OR an explicit global depth_cap circuit-breaker; no cycles.
  argue   -> DEFEATER-justified fallbacks: every node has >=1 fallback whose trigger
             is a stated defeater, OR is tagged single_point_of_failure with a reason;
             a SPOF on a root->leaf path with no fallback is a FATAL BREAK.
  appdev  -> per-node CONTRACT (pre/postconditions, acceptance tests on leaves) and a
             CONTRACT-COMPOSITION check (children's postcondition tags must COVER the
             parent's -- a tractable set-cover over declared postcondition tags).

It STRUCTURES and VALIDATES plans; it does NOT execute them.

Usage:
  python3 plan_validator.py <plan.json> [--report out.json] [--quiet]

Exit code 0 = PASS (no hard failures), 1 = hard failure(s), 2 = bad invocation.
Stdlib only. Mirrors validators/kb_validator.py conventions (checks list + report).
"""
import argparse
import json
import sys
from typing import Any, Dict, List, Optional, Set, Tuple

VALID_STATUS = {"planned", "decomposed", "ready", "blocked", "done"}


class Report:
    """Accumulates (check_id, status, detail); status in pass/fail/warn."""

    def __init__(self) -> None:
        self.checks: List[Tuple[str, str, str]] = []

    def ok(self, cid: str, detail: str = "") -> None:
        self.checks.append((cid, "pass", detail))

    def fail(self, cid: str, detail: str = "") -> None:
        self.checks.append((cid, "fail", detail))

    def warn(self, cid: str, detail: str = "") -> None:
        self.checks.append((cid, "warn", detail))

    @property
    def failed(self) -> List[Tuple[str, str, str]]:
        return [c for c in self.checks if c[1] == "fail"]

    @property
    def warned(self) -> List[Tuple[str, str, str]]:
        return [c for c in self.checks if c[1] == "warn"]

    def to_dict(self) -> Dict[str, Any]:
        return {
            "_directive": "machine-facing validation record; model-parse optimized",
            "validator": "plan_validator/1.0",
            "overall_status": "fail" if self.failed else ("pass_with_warnings" if self.warned else "pass"),
            "summary": {
                "passed": sum(1 for c in self.checks if c[1] == "pass"),
                "failed": len(self.failed),
                "warnings": len(self.warned),
            },
            "violated_rules": [c[0] for c in self.failed],
            "checks": [{"check_id": a, "status": b, "detail": c} for a, b, c in self.checks],
        }


def _is_str(x: Any) -> bool:
    return isinstance(x, str) and x.strip() != ""


# ----------------------------------------------------------------------------
# method: schema shape
# ----------------------------------------------------------------------------
def check_schema_shape(plan: Dict[str, Any], r: Report) -> Dict[str, Dict]:
    """Light structural validation (full JSON Schema is in plan.schema.json).
    Returns a dict id->node for nodes that are at least minimally shaped."""
    for k in ("plan_id", "root", "ranking_name", "depth_cap", "nodes"):
        if k not in plan:
            r.fail("schema.toplevel_keys", f"missing top-level key '{k}'")
    if not isinstance(plan.get("depth_cap"), int) or isinstance(plan.get("depth_cap"), bool) \
            or (isinstance(plan.get("depth_cap"), int) and plan["depth_cap"] < 1):
        r.fail("schema.depth_cap_int", f"depth_cap must be int>=1, got {plan.get('depth_cap')!r}")
    if not _is_str(plan.get("ranking_name")):
        r.fail("schema.ranking_name", "ranking_name must be a non-empty string")
    nodes = plan.get("nodes")
    by_id: Dict[str, Dict] = {}
    if not isinstance(nodes, list) or not nodes:
        r.fail("schema.nodes_list", "nodes must be a non-empty list")
        return by_id

    req_fields = ["id", "goal", "type", "arity", "parent", "children", "ranking_value",
                  "preconditions", "postconditions", "acceptance_tests", "fallbacks",
                  "single_point_of_failure", "spof_reason", "status"]
    shape_bad: List[str] = []
    for i, n in enumerate(nodes):
        if not isinstance(n, dict):
            shape_bad.append(f"node[{i}] not an object")
            continue
        nid = n.get("id")
        miss = [f for f in req_fields if f not in n]
        if miss:
            shape_bad.append(f"{nid!r} missing {miss}")
        if not isinstance(n.get("arity"), int) or isinstance(n.get("arity"), bool):
            shape_bad.append(f"{nid!r} arity not int")
        if not isinstance(n.get("ranking_value"), (int, float)) or isinstance(n.get("ranking_value"), bool):
            shape_bad.append(f"{nid!r} ranking_value not a number")
        if not isinstance(n.get("children"), list):
            shape_bad.append(f"{nid!r} children not a list")
        if not isinstance(n.get("single_point_of_failure"), bool):
            shape_bad.append(f"{nid!r} single_point_of_failure not bool")
        if n.get("status") not in VALID_STATUS:
            shape_bad.append(f"{nid!r} status {n.get('status')!r} not in {sorted(VALID_STATUS)}")
        if _is_str(nid):
            by_id[nid] = n

    (r.ok if not shape_bad else r.fail)("schema.node_shape", f"issues={shape_bad[:20]}")

    # id uniqueness
    ids = [n.get("id") for n in nodes if isinstance(n, dict)]
    dups = sorted({x for x in ids if ids.count(x) > 1})
    (r.ok if not dups else r.fail)("schema.id_unique", f"dups={dups}")

    # arity == len(children)
    bad_arity = []
    for n in by_id.values():
        ch = n.get("children", [])
        if isinstance(n.get("arity"), int) and not isinstance(n.get("arity"), bool) and isinstance(ch, list):
            if n["arity"] != len(ch):
                bad_arity.append((n.get("id"), n["arity"], len(ch)))
    (r.ok if not bad_arity else r.fail)("schema.arity_matches_children",
                                        f"type_driven_arity_mismatch(id,arity,len)={bad_arity}")
    return by_id


# ----------------------------------------------------------------------------
# method + loops: tree integrity (single root, parent/child consistency, no cycles)
# ----------------------------------------------------------------------------
def check_tree_integrity(plan: Dict[str, Any], by_id: Dict[str, Dict], r: Report) -> None:
    root = plan.get("root")
    ids = set(by_id)

    if root not in by_id:
        r.fail("tree.root_exists", f"root {root!r} not among node ids")
    else:
        r.ok("tree.root_exists", f"root={root}")

    # exactly one node with parent == null, and it is the declared root
    roots = [nid for nid, n in by_id.items() if n.get("parent") is None]
    if len(roots) == 1 and roots[0] == root:
        r.ok("tree.single_root", f"root={roots[0]}")
    else:
        r.fail("tree.single_root", f"nodes_with_null_parent={roots} declared_root={root}")

    # children referenced must exist; parent pointer must be consistent with child listing
    bad_child_ref, bad_parent_link, child_multi = [], [], []
    child_parents: Dict[str, List[str]] = {}
    for nid, n in by_id.items():
        for c in n.get("children", []):
            if c not in ids:
                bad_child_ref.append((nid, c))
            else:
                child_parents.setdefault(c, []).append(nid)
                if by_id[c].get("parent") != nid:
                    bad_parent_link.append((c, by_id[c].get("parent"), nid))
    for c, parents in child_parents.items():
        if len(parents) > 1:
            child_multi.append((c, parents))
    (r.ok if not bad_child_ref else r.fail)("tree.child_refs_exist", f"bad={bad_child_ref}")
    (r.ok if not bad_parent_link else r.fail)("tree.parent_child_consistent",
                                              f"child_parent_mismatch(child,parent_field,listed_under)={bad_parent_link}")
    (r.ok if not child_multi else r.fail)("tree.single_parent", f"children_with_multiple_parents={child_multi}")

    # parent pointers must reference existing nodes
    bad_parent_exist = [(nid, n.get("parent")) for nid, n in by_id.items()
                        if n.get("parent") is not None and n.get("parent") not in ids]
    (r.ok if not bad_parent_exist else r.fail)("tree.parent_refs_exist", f"bad={bad_parent_exist}")

    # NO CYCLES: walk parent pointers to root; any node not reaching root w/o repeat = cycle/orphan
    cycles, orphans = [], []
    for nid in by_id:
        seen: Set[str] = set()
        cur: Optional[str] = nid
        while cur is not None:
            if cur in seen:
                cycles.append(sorted(seen))
                break
            seen.add(cur)
            nxt = by_id.get(cur, {}).get("parent")
            if nxt is None:
                if cur != root:
                    orphans.append(nid)
                break
            if nxt not in by_id:
                break
            cur = nxt
    (r.ok if not cycles else r.fail)("tree.acyclic", f"cycle_member_sets={[c for c in cycles][:3]}")
    (r.ok if not orphans else r.fail)("tree.reaches_root", f"nodes_not_reaching_root={sorted(set(orphans))}")


def _depth_of(nid: str, by_id: Dict[str, Dict]) -> Optional[int]:
    """Depth from root (root depth = 0). None if a cycle is hit while walking."""
    depth = 0
    seen: Set[str] = set()
    cur: Optional[str] = nid
    while cur is not None:
        if cur in seen:
            return None
        seen.add(cur)
        p = by_id.get(cur, {}).get("parent")
        if p is None:
            return depth
        depth += 1
        cur = p
    return depth


# ----------------------------------------------------------------------------
# loops: termination proof obligation
# ----------------------------------------------------------------------------
def check_termination(plan: Dict[str, Any], by_id: Dict[str, Dict], r: Report) -> None:
    """For every parent->child edge, EITHER ranking_value strictly decreases,
    OR both endpoints fall within depth_cap (explicit circuit-breaker)."""
    depth_cap = plan.get("depth_cap")
    cap_ok = isinstance(depth_cap, int) and not isinstance(depth_cap, bool) and depth_cap >= 1

    non_decreasing: List[Tuple[str, str, Any, Any]] = []   # edges with no proof at all
    cap_relied: List[Tuple[str, str, int]] = []            # edges leaning on depth_cap
    cap_exceeded: List[Tuple[str, int]] = []               # nodes beyond depth_cap

    for pid, p in by_id.items():
        pv = p.get("ranking_value")
        for c in p.get("children", []):
            cn = by_id.get(c)
            if cn is None:
                continue
            cv = cn.get("ranking_value")
            strictly_decreases = (isinstance(pv, (int, float)) and isinstance(cv, (int, float))
                                  and not isinstance(pv, bool) and not isinstance(cv, bool)
                                  and cv < pv)
            if strictly_decreases:
                continue
            # no ranking decrease -> must be justified by depth_cap
            cdepth = _depth_of(c, by_id)
            if cap_ok and cdepth is not None and cdepth <= depth_cap:
                cap_relied.append((pid, c, cdepth))
            else:
                non_decreasing.append((pid, c, pv, cv))

    # also: any node deeper than depth_cap is a circuit-breaker violation
    for nid in by_id:
        d = _depth_of(nid, by_id)
        if cap_ok and d is not None and d > depth_cap:
            cap_exceeded.append((nid, d))

    (r.ok if not non_decreasing else r.fail)(
        "termination.ranking_decreases_or_depth_capped",
        f"edges_without_proof(parent,child,rv_parent,rv_child)={non_decreasing}")
    (r.ok if not cap_exceeded else r.fail)(
        "termination.depth_within_cap",
        f"depth_cap={depth_cap} nodes_exceeding(id,depth)={cap_exceeded}")
    if cap_relied:
        r.warn("termination.depth_cap_relied",
               f"edges relying on depth_cap rather than strict ranking decrease={cap_relied}")


# ----------------------------------------------------------------------------
# appdev: per-node contracts + leaf acceptance tests
# ----------------------------------------------------------------------------
def _is_leaf(n: Dict[str, Any]) -> bool:
    ch = n.get("children", [])
    return not (isinstance(ch, list) and len(ch) > 0)


def check_contracts(by_id: Dict[str, Dict], r: Report) -> None:
    no_pre, no_post, leaf_no_test, bad_test = [], [], [], []
    for nid, n in by_id.items():
        pre = n.get("preconditions")
        post = n.get("postconditions")
        if not (isinstance(pre, list) and len(pre) >= 1):
            no_pre.append(nid)
        if not (isinstance(post, list) and len(post) >= 1):
            no_post.append(nid)
        if _is_leaf(n):
            ats = n.get("acceptance_tests")
            if not (isinstance(ats, list) and len(ats) >= 1):
                leaf_no_test.append(nid)
            else:
                for t in ats:
                    if not (isinstance(t, dict) and _is_str(t.get("id")) and _is_str(t.get("desc"))):
                        bad_test.append((nid, t))
    (r.ok if not no_pre else r.fail)("contract.preconditions_present", f"nodes_without_preconditions={no_pre}")
    (r.ok if not no_post else r.fail)("contract.postconditions_present", f"nodes_without_postconditions={no_post}")
    (r.ok if not leaf_no_test else r.fail)("contract.leaf_acceptance_tests",
                                           f"leaves_without_acceptance_test={leaf_no_test}")
    (r.ok if not bad_test else r.fail)("contract.acceptance_test_shape", f"malformed={bad_test}")


def check_contract_composition(by_id: Dict[str, Dict], r: Report) -> None:
    """CONTRACT COMPOSITION: the conjunction of children's postconditions must COVER
    the parent's postconditions. Tractable set-cover over declared postcondition tags:
    every parent postcondition tag must appear in the union of its children's
    postcondition tags. Leaves are exempt (no children)."""
    uncovered: List[Tuple[str, List[str]]] = []
    for nid, n in by_id.items():
        if _is_leaf(n):
            continue
        parent_post: Set[str] = set(t for t in n.get("postconditions", []) if isinstance(t, str))
        child_union: Set[str] = set()
        for c in n.get("children", []):
            cn = by_id.get(c, {})
            child_union |= set(t for t in cn.get("postconditions", []) if isinstance(t, str))
        missing = sorted(parent_post - child_union)
        if missing:
            uncovered.append((nid, missing))
    (r.ok if not uncovered else r.fail)(
        "contract.composition_covers_parent",
        f"parents_with_uncovered_postconditions(parent,missing_tags)={uncovered}")


# ----------------------------------------------------------------------------
# argue: defeater-justified fallbacks + SPOF tagging + FATAL BREAK detection
# ----------------------------------------------------------------------------
def _has_defeater_fallback(n: Dict[str, Any]) -> bool:
    fbs = n.get("fallbacks", [])
    if not isinstance(fbs, list) or not fbs:
        return False
    for f in fbs:
        if isinstance(f, dict) and _is_str(f.get("trigger")) and _is_str(f.get("action")) and _is_str(f.get("id")):
            return True
    return False


def check_fallbacks_and_spof(by_id: Dict[str, Dict], r: Report) -> None:
    """Every node must have >=1 defeater-justified fallback OR be tagged
    single_point_of_failure==true WITH a non-empty spof_reason."""
    unguarded, spof_no_reason = [], []
    for nid, n in by_id.items():
        has_fb = _has_defeater_fallback(n)
        is_spof = n.get("single_point_of_failure") is True
        reason = n.get("spof_reason")
        if is_spof and not _is_str(reason):
            spof_no_reason.append(nid)
        if not has_fb and not (is_spof and _is_str(reason)):
            unguarded.append(nid)
    (r.ok if not spof_no_reason else r.fail)("fallback.spof_reason_present",
                                             f"spof_nodes_without_reason={spof_no_reason}")
    (r.ok if not unguarded else r.fail)(
        "fallback.defeater_justified_or_spof_tagged",
        f"nodes_without_fallback_and_not_spof_tagged={unguarded}")


def check_fatal_breaks(plan: Dict[str, Any], by_id: Dict[str, Dict], r: Report) -> None:
    """FATAL BREAK: a SPOF node WITH NO fallback that lies on at least one path from
    root to a leaf (a critical path). Reported as a list."""
    # set of nodes on some root->leaf path = every reachable node (each reachable node
    # is on the path root..node..(down to a leaf)). Compute reachable-from-root set.
    root = plan.get("root")
    reachable: Set[str] = set()
    stack = [root] if root in by_id else []
    while stack:
        cur = stack.pop()
        if cur in reachable or cur not in by_id:
            continue
        reachable.add(cur)
        for c in by_id[cur].get("children", []):
            if c in by_id:
                stack.append(c)

    fatal: List[str] = []
    for nid in reachable:
        n = by_id[nid]
        if n.get("single_point_of_failure") is True and not _has_defeater_fallback(n):
            fatal.append(nid)
    (r.ok if not fatal else r.fail)(
        "fatal_break.spof_no_fallback_on_critical_path",
        f"FATAL_BREAKS(spof_node_no_fallback_on_root_to_leaf_path)={sorted(fatal)}")


# ----------------------------------------------------------------------------
# driver
# ----------------------------------------------------------------------------
def validate(plan: Dict[str, Any], r: Report) -> None:
    def guard(fn, *a):
        try:
            return fn(*a)
        except Exception as e:  # turn any structural crash into a deterministic FAIL
            r.fail("validator.exception", f"{fn.__name__}: {type(e).__name__}: {e}")
            return None

    by_id = guard(check_schema_shape, plan, r) or {}
    guard(check_tree_integrity, plan, by_id, r)
    guard(check_termination, plan, by_id, r)
    guard(check_contracts, by_id, r)
    guard(check_contract_composition, by_id, r)
    guard(check_fallbacks_and_spof, by_id, r)
    guard(check_fatal_breaks, plan, by_id, r)


def main() -> int:
    ap = argparse.ArgumentParser(description="Validate a B13 recursive plan-decomposition tree.")
    ap.add_argument("plan", help="path to plan JSON file")
    ap.add_argument("--report", help="write JSON report to this path")
    ap.add_argument("--quiet", action="store_true")
    args = ap.parse_args()

    try:
        raw = open(args.plan, encoding="utf-8").read()
    except OSError as e:
        print(f"cannot read {args.plan}: {e}", file=sys.stderr)
        return 2

    r = Report()
    try:
        plan = json.loads(raw)
    except json.JSONDecodeError as e:
        r.fail("json.valid", str(e))
        out = r.to_dict()
        if args.report:
            open(args.report, "w").write(json.dumps(out, indent=2))
        if not args.quiet:
            print(json.dumps(out, indent=2))
        return 1
    r.ok("json.valid")

    if not isinstance(plan, dict):
        r.fail("json.is_object", f"top-level JSON must be an object, got {type(plan).__name__}")
    else:
        validate(plan, r)

    out = r.to_dict()
    if args.report:
        open(args.report, "w").write(json.dumps(out, indent=2))
    if not args.quiet:
        print(json.dumps(out, indent=2))
    return 1 if r.failed else 0


if __name__ == "__main__":
    sys.exit(main())
