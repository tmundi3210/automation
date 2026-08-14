#!/usr/bin/env python3
"""
test_plan_validator.py -- unittest suite for the B13 plan_validator.

Verifies:
  * the PASS fixture validates clean (exit 0, no failed checks);
  * the FAIL fixture is rejected and NAMES the violated rules, specifically the
    termination (non-decreasing ranking) rule and the missing-fallback / fatal-break
    rules required by the branch spec;
  * targeted negative cases (cycle, contract-composition gap, arity/children
    mismatch, leaf missing acceptance test).

Exit 0 when all tests pass. Stdlib only.

Run:  python3 test_plan_validator.py
"""
import copy
import json
import os
import unittest

import plan_validator as pv

HERE = os.path.dirname(os.path.abspath(__file__))
PASS = os.path.join(HERE, "examples", "plan.pass.json")
FAIL = os.path.join(HERE, "examples", "plan.fail.json")


def run(plan):
    r = pv.Report()
    pv.validate(plan, r)
    return r


def load(path):
    with open(path, encoding="utf-8") as fh:
        return json.load(fh)


class TestPassFixture(unittest.TestCase):
    def test_pass_fixture_validates_clean(self):
        r = run(load(PASS))
        self.assertEqual([], [c[0] for c in r.failed],
                         f"pass fixture should have no failures, got {r.failed}")

    def test_pass_fixture_overall_status_pass(self):
        out = run(load(PASS)).to_dict()
        self.assertEqual("pass", out["overall_status"])
        self.assertEqual([], out["violated_rules"])


class TestFailFixture(unittest.TestCase):
    def setUp(self):
        self.report = run(load(FAIL))
        self.violated = {c[0] for c in self.report.failed}

    def test_fail_fixture_is_rejected(self):
        self.assertTrue(self.report.failed, "fail fixture must produce at least one failure")

    def test_names_termination_rule(self):
        self.assertIn("termination.ranking_decreases_or_depth_capped", self.violated,
                      f"expected non-decreasing-ranking violation; got {sorted(self.violated)}")

    def test_names_missing_fallback_rule(self):
        self.assertIn("fallback.defeater_justified_or_spof_tagged", self.violated,
                      f"expected missing-fallback violation; got {sorted(self.violated)}")

    def test_names_fatal_break_rule(self):
        self.assertIn("fatal_break.spof_no_fallback_on_critical_path", self.violated,
                      f"expected fatal-break violation; got {sorted(self.violated)}")

    def test_report_lists_violated_rules(self):
        out = self.report.to_dict()
        self.assertIn("violated_rules", out)
        self.assertIn("termination.ranking_decreases_or_depth_capped", out["violated_rules"])
        self.assertIn("fatal_break.spof_no_fallback_on_critical_path", out["violated_rules"])


class TestTargetedNegatives(unittest.TestCase):
    def setUp(self):
        self.base = load(PASS)

    def _by_id(self, plan, nid):
        return next(n for n in plan["nodes"] if n["id"] == nid)

    def test_cycle_is_detected(self):
        plan = copy.deepcopy(self.base)
        # make the root point at a descendant as parent -> introduces a cycle
        self._by_id(plan, "N0")["parent"] = "N3"
        self._by_id(plan, "N3")["children"] = ["N0"]
        self._by_id(plan, "N3")["arity"] = 1
        r = run(plan)
        self.assertIn("tree.acyclic", {c[0] for c in r.failed})

    def test_contract_composition_gap(self):
        plan = copy.deepcopy(self.base)
        # remove a postcondition tag from a child so the parent's is uncovered
        self._by_id(plan, "N3")["postconditions"] = ["something_unrelated"]
        r = run(plan)
        self.assertIn("contract.composition_covers_parent", {c[0] for c in r.failed})

    def test_arity_must_match_children(self):
        plan = copy.deepcopy(self.base)
        self._by_id(plan, "N1")["arity"] = 5  # type-driven arity must equal len(children)
        r = run(plan)
        self.assertIn("schema.arity_matches_children", {c[0] for c in r.failed})

    def test_leaf_requires_acceptance_test(self):
        plan = copy.deepcopy(self.base)
        self._by_id(plan, "N3")["acceptance_tests"] = []
        r = run(plan)
        self.assertIn("contract.leaf_acceptance_tests", {c[0] for c in r.failed})

    def test_node_requires_pre_and_post(self):
        plan = copy.deepcopy(self.base)
        self._by_id(plan, "N2a")["preconditions"] = []
        r = run(plan)
        self.assertIn("contract.preconditions_present", {c[0] for c in r.failed})

    def test_non_decreasing_ranking_without_cap_rescue(self):
        plan = copy.deepcopy(self.base)
        # raise a deep child's ranking above its parent; depth 3 > depth_cap 4? cap=4 so
        # rescue applies. Tighten cap so the edge is not rescued.
        plan["depth_cap"] = 1
        self._by_id(plan, "N1a")["ranking_value"] = 999
        r = run(plan)
        self.assertIn("termination.ranking_decreases_or_depth_capped", {c[0] for c in r.failed})


if __name__ == "__main__":
    unittest.main(verbosity=2)
