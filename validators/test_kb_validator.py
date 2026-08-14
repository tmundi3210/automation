#!/usr/bin/env python3
"""
test_kb_validator.py — fault-injection suite for validators/kb_validator.py
(T13 phase 3, closes origin-critique gap G3: "the gate was never tested").

Method
------
1. Copy a known-green KB (FACTORY/sweater_vertical/specialists/yarn/
   kb1_yarn_fibre_line.kb.json) into a temp dir as the PRISTINE artifact.
2. Control: assert the CLEAN copy passes the gate (exit 0).
3. For each fault case: restore a fresh working copy from PRISTINE (no
   cross-contamination), apply exactly ONE mutation targeting one defect
   class the validator claims to catch, run the gate exactly as build.sh
   does (--mode dense), and assert a NONZERO exit AND that the targeted
   check_id is reported with status "fail".
4. One WARN-level source_registry case asserts exit stays 0 while the warn
   check appears in the report.

Honesty contract: if the validator MISSES an injected defect (exit 0 where
nonzero was expected) this suite FAILS — the miss is a real finding and the
test must never be weakened to hide it.

Stdlib only. No pytest. Run:  python3 validators/test_kb_validator.py
Exit 0 = all cases pass, 1 = at least one case failed.
"""
import json
import os
import shutil
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(HERE)
VALIDATOR = os.path.join(HERE, "kb_validator.py")
GREEN_KB = os.path.join(
    REPO, "FACTORY", "sweater_vertical", "specialists", "yarn",
    "kb1_yarn_fibre_line.kb.json")


# --------------------------------------------------------------------------
# harness
# --------------------------------------------------------------------------

def run_validator(kb_path, report_path):
    """Run the gate exactly as FACTORY/build.sh does (--mode dense)."""
    proc = subprocess.run(
        [sys.executable, VALIDATOR, kb_path, "--mode", "dense",
         "--quiet", "--report", report_path],
        capture_output=True, text=True)
    report = None
    if os.path.exists(report_path):
        try:
            with open(report_path, encoding="utf-8") as f:
                report = json.load(f)
        except (OSError, json.JSONDecodeError):
            report = None
    return proc.returncode, report


def check_status(report, check_id):
    """Return the set of statuses recorded for check_id ('' if absent)."""
    if not report:
        return set()
    return {c["status"] for c in report.get("checks", [])
            if c.get("check_id") == check_id}


class Case:
    def __init__(self, cid, what, mutate, expect_check, expect_exit="nonzero",
                 expect_status="fail", raw_mutate=None):
        self.cid = cid
        self.what = what
        self.mutate = mutate            # fn(kb_dict) -> None, or None
        self.raw_mutate = raw_mutate    # fn(raw_text) -> raw_text, for JSON faults
        self.expect_check = expect_check
        self.expect_exit = expect_exit  # "nonzero" | "zero"
        self.expect_status = expect_status


# --------------------------------------------------------------------------
# mutations — each targets exactly ONE defect class
# --------------------------------------------------------------------------

def mut_missing_top_level(kb):
    del kb["glossary"]


def mut_duplicate_node_id(kb):
    kb["nodes"][1]["id"] = kb["nodes"][0]["id"]


def mut_dangling_edge_ref(kb):
    kb["edges"][0]["to"] = "NODE_THAT_DOES_NOT_EXIST"


def mut_broken_formula(kb):
    # perturb one computed value beyond tol 0.02 (delta 0.10), staying in [0,1]
    m = kb["nodes"][0]["metrics"]
    v = m["final_importance"]
    m["final_importance"] = round(v - 0.10 if v > 0.5 else v + 0.10, 4)


def mut_dependency_cycle(kb):
    # find B with dependency on A, then make A depend on B -> A<->B cycle
    for b in kb["nodes"]:
        for a_id in b.get("dependencies", []):
            for a in kb["nodes"]:
                if a["id"] == a_id:
                    a.setdefault("dependencies", []).append(b["id"])
                    return
    raise AssertionError("green KB has no node dependency to invert")


def mut_priority_order_incomplete(kb):
    kb["priority_order"].pop()


def mut_weight_out_of_range(kb):
    kb["nodes"][0]["metrics"]["criticality"] = 1.5


def mut_signed_tension_out_of_range(kb):
    for e in kb["edges"]:
        if e.get("edge_type") == "conflict":
            e["signed_tension"] = -1.5   # negative keeps sign rule green; bounds must trip
            return
    raise AssertionError("green KB has no conflict edge")


def mut_placeholder_todo_lorem(kb):
    kb["purpose"] = kb["purpose"] + " TODO lorem ipsum placeholder text"


def mut_placeholder_template_token(kb):
    # supplementary: token from the validator's own PLACEHOLDERS list
    kb["purpose"] = kb["purpose"] + " NODE_A"


def mut_observed_without_data(kb):
    kb["nodes"][0]["metric_labels"]["criticality"] = "observed"


def mut_density_below_floor(kb):
    # dense floor is 19 nodes; green KB has 20 -> drop 2 => 18 < 19
    del kb["nodes"][18:]


def mut_schema_version_pin(kb):
    kb["schema_version"] = "9.9"   # mismatched + unsupported


def mut_warn_source_registry(kb):
    # warn-level entry-shape violation: drop a required source_registry field
    del kb["source_registry"][0]["freshness_status"]


def raw_invalid_json(raw):
    return raw[: len(raw) // 2] + "{{{ not json"


CASES = [
    Case("invalid_json", "syntactically invalid JSON",
         None, "json.valid", raw_mutate=raw_invalid_json),
    Case("missing_top_level_key", "required top-level key 'glossary' deleted",
         mut_missing_top_level, "toplevel.required_keys"),
    Case("duplicate_node_id", "nodes[1].id set equal to nodes[0].id",
         mut_duplicate_node_id, "nodes.id_unique"),
    Case("dangling_edge_ref", "edges[0].to points at a nonexistent node",
         mut_dangling_edge_ref, "edges.endpoint_valid"),
    Case("broken_formula", "final_importance perturbed by 0.10 (> tol 0.02)",
         mut_broken_formula, "formula.consistency"),
    Case("dependency_cycle", "reciprocal node dependency A<->B injected",
         mut_dependency_cycle, "dependency.acyclic"),
    Case("priority_order_incomplete", "last priority_order entry removed",
         mut_priority_order_incomplete, "priority_order.complete"),
    Case("weight_out_of_range", "metrics.criticality set to 1.5 (>1)",
         mut_weight_out_of_range, "bounds.scores_in_0_1"),
    Case("signed_tension_out_of_range", "conflict edge signed_tension=-1.5",
         mut_signed_tension_out_of_range, "bounds.signed_tension_in_-1_1"),
    Case("placeholder_todo_lorem", "'TODO lorem ipsum' inserted into purpose",
         mut_placeholder_todo_lorem, "placeholders.none_leaked"),
    Case("placeholder_template_token", "template token 'NODE_A' inserted into purpose",
         mut_placeholder_template_token, "placeholders.none_leaked"),
    Case("observed_without_data", "metric_labels.criticality='observed' with no data",
         mut_observed_without_data, "evidence.no_observed_without_data"),
    Case("density_below_floor", "nodes cut 20 -> 18, below dense floor 19",
         mut_density_below_floor, "count.nodes"),
    Case("schema_version_pin", "schema_version set to unsupported '9.9'",
         mut_schema_version_pin, "version.supported"),
    Case("warn_source_registry", "source_registry[0].freshness_status deleted (warn-level)",
         mut_warn_source_registry, "source_registry.entry_shape",
         expect_exit="zero", expect_status="warn"),
]


# --------------------------------------------------------------------------
# runner
# --------------------------------------------------------------------------

def main():
    if not os.path.exists(GREEN_KB):
        print(f"BLOCKED: green KB not found: {GREEN_KB}", file=sys.stderr)
        return 1
    if not os.path.exists(VALIDATOR):
        print(f"BLOCKED: validator not found: {VALIDATOR}", file=sys.stderr)
        return 1

    tmp = tempfile.mkdtemp(prefix="kb_fault_injection_")
    failures = []
    try:
        pristine = os.path.join(tmp, "pristine.kb.json")
        shutil.copyfile(GREEN_KB, pristine)

        # ---- control: clean copy must pass ------------------------------
        work = os.path.join(tmp, "work.kb.json")
        report_path = os.path.join(tmp, "report.json")
        shutil.copyfile(pristine, work)
        rc, report = run_validator(work, report_path)
        if rc == 0:
            print("PASS  control_clean            clean copy passes gate (exit 0)")
        else:
            failed = [c for c in (report or {}).get("checks", [])
                      if c.get("status") == "fail"]
            print(f"FAIL  control_clean            expected exit 0, got {rc}; "
                  f"failed checks: {failed}")
            failures.append("control_clean")
            # a broken control invalidates every fault case
            print("BLOCKED: control failed; fault cases cannot be interpreted.")
            return 1

        # ---- fault cases -------------------------------------------------
        for case in CASES:
            # restore from pristine first: no cross-contamination
            if os.path.exists(report_path):
                os.remove(report_path)
            shutil.copyfile(pristine, work)

            if case.raw_mutate is not None:
                with open(work, encoding="utf-8") as f:
                    raw = f.read()
                with open(work, "w", encoding="utf-8") as f:
                    f.write(case.raw_mutate(raw))
            else:
                with open(work, encoding="utf-8") as f:
                    kb = json.load(f)
                case.mutate(kb)
                with open(work, "w", encoding="utf-8") as f:
                    json.dump(kb, f, indent=1)

            rc, report = run_validator(work, report_path)
            statuses = check_status(report, case.expect_check)

            exit_ok = (rc != 0) if case.expect_exit == "nonzero" else (rc == 0)
            status_ok = case.expect_status in statuses

            if exit_ok and status_ok:
                print(f"PASS  {case.cid:<24} exit={rc}, "
                      f"{case.expect_check}={case.expect_status}")
            else:
                print(f"FAIL  {case.cid:<24} [{case.what}] "
                      f"expected exit {case.expect_exit} + "
                      f"{case.expect_check}:{case.expect_status}; "
                      f"got exit={rc}, {case.expect_check} statuses={sorted(statuses) or ['<absent>']}")
                failures.append(case.cid)

        print()
        total = len(CASES) + 1
        print(f"{total - len(failures)}/{total} cases passed"
              + (f"; FAILED: {failures}" if failures else ""))
        if failures:
            print("A FAIL above means the gate MISSED an injected defect (or a "
                  "warn contract broke). Do not weaken this test; fix the gate.")
        return 1 if failures else 0
    finally:
        shutil.rmtree(tmp, ignore_errors=True)


if __name__ == "__main__":
    sys.exit(main())
