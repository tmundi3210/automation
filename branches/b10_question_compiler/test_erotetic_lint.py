#!/usr/bin/env python3
"""
test_erotetic_lint.py — unittest suite for the B10 erotetic compiler validator.

Asserts:
  * the clean closed-type fixture PASSES (exit semantics: no hard failures);
  * the loaded fixture FAILS and the specific violated rules are named
    (false presupposition -> presupposition.not_loaded; cyclic sub-question
     dependency -> sub_questions.dag);
  * the closed-vs-open answer-schema branching behaves correctly
    (open type without a warrant slot is rejected; admissible set on an open
     type is a category error).

Run: python3 -m unittest test_erotetic_lint -v   (exits 0 when the suite passes)
"""
import json
import os
import unittest

import erotetic_lint as el

HERE = os.path.dirname(os.path.abspath(__file__))
PASS_FIXTURE = os.path.join(HERE, "examples", "q_polar.pass.json")
FAIL_FIXTURE = os.path.join(HERE, "examples", "q_loaded.fail.json")


def run(path):
    """Return (failed_bool, report_dict) for a question-object file."""
    with open(path, encoding="utf-8") as fh:
        q = json.load(fh)
    r = el.Report()
    el.validate(q, r)
    return bool(r.failed), r.to_dict()


def run_obj(obj):
    r = el.Report()
    el.validate(obj, r)
    return bool(r.failed), r.to_dict()


class TestPassFixture(unittest.TestCase):
    def test_pass_fixture_passes(self):
        failed, report = run(PASS_FIXTURE)
        self.assertFalse(failed, f"clean fixture should pass; violated={report['violated_rules']}")
        self.assertEqual(report["overall_status"], "pass")

    def test_pass_fixture_rubric_strong(self):
        _, report = run(PASS_FIXTURE)
        rub = report["question_quality_rubric"]
        # clean closed question with verified presuppositions should score well
        self.assertEqual(rub["presupposition_validity"], 1.0)
        self.assertEqual(rub["answerability"], 1.0)
        self.assertGreaterEqual(rub["composite"], 0.8)


class TestFailFixture(unittest.TestCase):
    def test_fail_fixture_fails(self):
        failed, _ = run(FAIL_FIXTURE)
        self.assertTrue(failed, "loaded/cyclic fixture must fail")

    def test_names_false_presupposition_rule(self):
        _, report = run(FAIL_FIXTURE)
        self.assertIn("presupposition.not_loaded", report["violated_rules"],
                      "false presupposition must trip presupposition.not_loaded")

    def test_names_cycle_rule(self):
        _, report = run(FAIL_FIXTURE)
        self.assertIn("sub_questions.dag", report["violated_rules"],
                      "cyclic sub-question dependency must trip sub_questions.dag")

    def test_fail_fixture_rubric_penalizes_presupposition(self):
        _, report = run(FAIL_FIXTURE)
        self.assertEqual(report["question_quality_rubric"]["presupposition_validity"], 0.0)


class TestClosedVsOpenBranching(unittest.TestCase):
    def _base_open(self):
        return {
            "question_id": "Q_T",
            "text": "Why does the cache thrash under load?",
            "interrogative_type": "why",
            "presuppositions": [
                {"id": "P_THRASH", "text": "The cache thrashes under load.", "validity": "verified"}
            ],
            "answer_schema": {"explanans_schema": "causal_mechanism", "warrant_required": True},
            "resolution_criteria": "Complete when a warranted causal explanans is supplied that covers the observed thrash.",
            "decision_relevance": "Determines whether to choose a different eviction policy.",
            "sub_questions": [],
        }

    def test_open_requires_warrant_slot(self):
        q = self._base_open()
        q["answer_schema"]["warrant_required"] = False
        failed, report = run_obj(q)
        self.assertTrue(failed)
        self.assertIn("answer.scorable", report["violated_rules"],
                      "open type without warrant slot must fail answer.scorable")

    def test_open_rejects_admissible_set(self):
        q = self._base_open()
        q["answer_schema"]["admissible"] = ["a", "b"]  # category error on an open type
        failed, report = run_obj(q)
        self.assertTrue(failed)
        self.assertIn("answer_schema.consistent", report["violated_rules"],
                      "admissible set on an open type is a category error")

    def test_closed_polar_admissible_must_be_subset(self):
        q = {
            "question_id": "Q_P",
            "text": "Is the build green?",
            "interrogative_type": "polar",
            "presuppositions": [],
            "answer_schema": {"admissible": ["yes", "no", "maybe"], "warrant_required": True},
            "resolution_criteria": "Complete when one admissible value is chosen with a warrant.",
            "decision_relevance": "Determines whether to ship.",
            "sub_questions": [],
        }
        failed, report = run_obj(q)
        self.assertTrue(failed)
        self.assertIn("answer_schema.consistent", report["violated_rules"],
                      "polar admissible set must be subset of {yes,no,indeterminate}")

    def test_closed_open_pass_through(self):
        # a clean open question should pass
        failed, report = run_obj(self._base_open())
        self.assertFalse(failed, f"clean open question should pass; violated={report['violated_rules']}")


if __name__ == "__main__":
    unittest.main()
