#!/usr/bin/env python3
"""
test_acquisition_ledger.py — unittests for branch B11 acquisition_ledger tool.

Covers:
  - pass fixture validates (exit 0, overall pass)
  - fail fixture is rejected SPECIFICALLY for evidence-label legality
    (the 'high'-label-with-one-independence-group reason)
  - derive-views returns correct known_known / known_unknown counts
  - a probe spawns a known_unknown CQ for a topic
  - source scoring penalizes common-source dependence

Stdlib only. Exit 0 when passing.
"""
import json
import os
import unittest

import acquisition_ledger as al

HERE = os.path.dirname(os.path.abspath(__file__))
PASS = os.path.join(HERE, "examples", "ledger.pass.json")
FAIL = os.path.join(HERE, "examples", "ledger.fail.json")


def load(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)


class TestValidate(unittest.TestCase):
    def test_pass_fixture_validates(self):
        ledger = load(PASS)
        r = al.validate_ledger(ledger)
        self.assertEqual(r.failed, [], f"unexpected failures: {r.failed}")
        self.assertEqual(r.to_dict()["overall_status"], "pass")

    def test_fail_fixture_rejected_for_evidence_legality(self):
        ledger = load(FAIL)
        r = al.validate_ledger(ledger)
        self.assertTrue(r.failed, "fail fixture should not validate")
        # the failing check must be the evidence-label legality one
        failed_ids = {c[0] for c in r.failed}
        self.assertIn("evidence.label_legality", failed_ids)
        # and the human-readable reason must mention independence groups / high label
        detail = next(c[2] for c in r.failed if c[0] == "evidence.label_legality")
        self.assertIn("high", detail)
        self.assertIn("independence_group", detail)

    def test_evidence_label_legal_function_high_needs_two_groups(self):
        # one distinct group -> illegal for 'high'
        entry = {
            "cq_id": "CQ_X",
            "status": "known_known",
            "claim": "x",
            "evidence_label": "high",
            "sources": [
                {"id": "a", "independence_group": "g1"},
                {"id": "b", "independence_group": "g1"},
            ],
        }
        legal, reason = al.evidence_label_legal(entry)
        self.assertFalse(legal)
        self.assertIn("DISTINCT independence_group", reason)
        # two distinct groups -> legal
        entry["sources"][1]["independence_group"] = "g2"
        legal2, _ = al.evidence_label_legal(entry)
        self.assertTrue(legal2)

    def test_known_unknown_must_have_null_claim(self):
        entry = {
            "cq_id": "CQ_Y",
            "status": "known_unknown",
            "claim": "should not be here",
            "evidence_label": "ungraded",
            "sources": [],
        }
        legal, reason = al.evidence_label_legal(entry)
        self.assertFalse(legal)
        self.assertIn("null claim", reason)


class TestDerivedViews(unittest.TestCase):
    def test_counts(self):
        ledger = load(PASS)
        views = al.derive_views(ledger)
        # pass fixture: CQ_001, CQ_002 are known_known; CQ_003..CQ_007 known_unknown
        self.assertEqual(views["counts"]["known_known"], 2)
        self.assertEqual(views["counts"]["known_unknown"], 5)
        self.assertEqual(views["counts"]["total_entries"], 7)
        self.assertEqual(len(views["known_known_matrix"]), 2)
        self.assertEqual(len(views["known_unknown_matrix"]), 5)

    def test_views_are_pure_derivation(self):
        # deriving twice yields identical output (no hidden state)
        ledger = load(PASS)
        self.assertEqual(al.derive_views(ledger), al.derive_views(ledger))

    def test_known_known_row_reports_independent_groups(self):
        ledger = load(PASS)
        kk = al.derive_known_known_matrix(ledger)
        row = next(r for r in kk if r["cq_id"] == "CQ_001")
        self.assertEqual(row["n_independent_groups"], 2)


class TestProbes(unittest.TestCase):
    def test_probe_spawns_known_unknown_cq(self):
        spawned = al.spawn_cqs_for_topic("widget reliability")
        self.assertTrue(spawned)
        for s in spawned:
            self.assertEqual(s["spawns"], "a known_unknown CQ")
            self.assertIn("widget reliability", s["spawned_known_unknown_cq"])
        # ACH + KAC are reused from argue, others are new
        owners = {s["probe_id"]: s["owned_by"] for s in spawned}
        self.assertEqual(owners.get("PRB_ACH"), "argue")
        self.assertEqual(owners.get("PRB_KAC"), "argue")
        self.assertEqual(owners.get("PRB_PREMORTEM"), "new")

    def test_spawned_refs_validate_against_battery(self):
        # every spawned_by in the pass fixture resolves to a real probe
        ledger = load(PASS)
        r = al.Report()
        al.check_spawn_refs(ledger, r)
        self.assertEqual(r.failed, [], f"spawn refs should resolve: {r.failed}")


class TestSourceScoring(unittest.TestCase):
    def test_common_source_dependence_penalized(self):
        src = {"id": "s", "authority": 0.9, "recency": 0.9,
               "relevance": 0.9, "independence_group": "g1"}
        # standalone
        standalone = al.score_source(src)
        # in a 2-source set sharing the SAME group -> independence credit lost
        dependent_ctx = [
            {"id": "s", "independence_group": "g1"},
            {"id": "t", "independence_group": "g1"},
        ]
        dependent = al.score_source(src, dependent_ctx)
        self.assertLess(dependent["components"]["independence"],
                        standalone["components"]["independence"])
        # in a 2-source set with DISTINCT groups -> corroboration credit gained
        indep_ctx = [
            {"id": "s", "independence_group": "g1"},
            {"id": "t", "independence_group": "g2"},
        ]
        independent = al.score_source(src, indep_ctx)
        self.assertEqual(independent["components"]["independence"], 1.0)
        self.assertGreater(independent["components"]["corroboration"], 0.0)


if __name__ == "__main__":
    unittest.main(verbosity=2)
