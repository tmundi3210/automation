import json
from pathlib import Path
import tempfile
import unittest

from orchestrator.control_plane.tick import run


class TickTests(unittest.TestCase):
    def _config(self, directory: str) -> Path:
        config_path = Path(directory) / "config.json"
        config_path.write_text(
            json.dumps({"component": "control_plane", "state_dir": str(Path(directory) / "state")}),
            encoding="utf-8",
        )
        return config_path

    def test_tick_writes_a_complete_run_record_and_audit_line(self):
        with tempfile.TemporaryDirectory() as temporary:
            config_path = self._config(temporary)
            result = run(config_path)
            self.assertEqual(0, result.exit_code)
            self.assertIsNotNone(result.run_id)
            state_dir = Path(temporary) / "state"
            record_path = state_dir / "runs" / f"{result.run_id}.json"
            record = json.loads(record_path.read_text(encoding="utf-8"))
            self.assertEqual(result.run_id, record["run_id"])
            self.assertEqual(result.trace_id, record["trace_id"])
            self.assertTrue(record["started_at_utc"].endswith("Z"))
            self.assertTrue(record["finished_at_utc"].endswith("Z"))
            self.assertEqual(0, record["model_invocations"])
            self.assertEqual(0, record["network_calls"])
            audit = state_dir.joinpath("audit.log").read_text(encoding="utf-8").splitlines()
            self.assertEqual(1, len(audit))
            self.assertEqual("TICK_COMPLETED", json.loads(audit[0])["event"])

    def test_pause_writes_only_paused_audit_line_and_no_run_record(self):
        with tempfile.TemporaryDirectory() as temporary:
            config_path = self._config(temporary)
            state_dir = Path(temporary) / "state"
            state_dir.mkdir()
            state_dir.joinpath("PAUSE").touch()
            result = run(config_path)
            self.assertEqual(1, result.exit_code)
            self.assertIsNone(result.run_id)
            self.assertFalse((state_dir / "runs").exists())
            audit = state_dir.joinpath("audit.log").read_text(encoding="utf-8").splitlines()
            self.assertEqual(1, len(audit))
            self.assertEqual("PAUSED", json.loads(audit[0])["event"])
