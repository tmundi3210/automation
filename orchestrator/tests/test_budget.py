import json
from pathlib import Path
import tempfile
import unittest

from orchestrator.control_plane.budget import (
    BudgetCounterStore,
    RUNG_DEFER_NONURGENT,
    RUNG_HARD_STOP,
    RUNG_PAUSE_NEW,
    RUNG_WARN,
    evaluate_budget,
    load_budgets,
)


class BudgetTests(unittest.TestCase):
    def setUp(self):
        self.budgets = {"components": {"test": {"invocations_max": 100, "wall_minutes_max": 100}}}

    def test_all_degradation_rungs(self):
        expectations = {
            70: RUNG_WARN,
            85: RUNG_DEFER_NONURGENT,
            95: RUNG_PAUSE_NEW,
            100: RUNG_HARD_STOP,
        }
        for used, expected_rung in expectations.items():
            with self.subTest(used=used):
                status = evaluate_budget(self.budgets, "test", {"invocations": used})
                self.assertEqual(expected_rung, status.rung)
                self.assertEqual(float(used), status.utilization_percent)

    def test_wall_minutes_can_trigger_the_stricter_rung(self):
        status = evaluate_budget(
            self.budgets, "test", {"invocations": 1, "wall_minutes": 95}
        )
        self.assertEqual(RUNG_PAUSE_NEW, status.rung)

    def test_counter_store_accumulates_component_usage(self):
        with tempfile.TemporaryDirectory() as temporary:
            store = BudgetCounterStore(Path(temporary) / "counters.json")
            self.assertEqual(
                {"invocations": 2.0, "wall_minutes": 5.0},
                store.record("test", invocations=2, wall_minutes=5),
            )
            self.assertEqual(
                {"invocations": 3.0, "wall_minutes": 7.0},
                store.record("test", invocations=1, wall_minutes=2),
            )

    def test_committed_budgets_match_phase_a_source_values(self):
        budgets_path = Path(__file__).parents[1] / "config" / "budgets.json"
        budgets = load_budgets(budgets_path)
        self.assertEqual(30, budgets["components"]["claude_code_hub"]["invocations_max"])
        self.assertEqual(180, budgets["components"]["claude_code_hub"]["wall_minutes_max"])
        self.assertEqual(15, budgets["components"]["grok_cli"]["invocations_max"])
        self.assertEqual(150, budgets["components"]["grok_cli"]["wall_minutes_max"])
        self.assertEqual(12, budgets["components"]["codex_cli"]["invocations_max"])
        self.assertEqual(120, budgets["components"]["codex_cli"]["wall_minutes_max"])
