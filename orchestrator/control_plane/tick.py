"""No-op, offline schedule tick for the Phase A control-plane skeleton."""

from __future__ import annotations

import argparse
from dataclasses import asdict, dataclass
from datetime import datetime, timezone
import json
from pathlib import Path
import uuid


@dataclass(frozen=True)
class TickResult:
    exit_code: int
    run_id: str | None
    trace_id: str | None


def utc_now() -> str:
    """Return an unambiguous UTC timestamp suitable for JSON records."""
    return datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")


def load_config(path: str | Path) -> dict[str, object]:
    with Path(path).open(encoding="utf-8") as handle:
        config = json.load(handle)
    if not isinstance(config.get("state_dir"), str):
        raise ValueError("configuration must contain a string state_dir")
    return config


def _append_audit(state_dir: Path, event: str, **fields: object) -> None:
    state_dir.mkdir(parents=True, exist_ok=True)
    record = {"timestamp_utc": utc_now(), "event": event, **fields}
    with (state_dir / "audit.log").open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(record, sort_keys=True) + "\n")


def run(config_path: str | Path) -> TickResult:
    """Perform one no-op tick without adapters, model calls, or network calls."""
    config = load_config(config_path)
    state_dir = Path(str(config["state_dir"]))

    if (state_dir / "PAUSE").exists():
        _append_audit(state_dir, "PAUSED")
        return TickResult(exit_code=1, run_id=None, trace_id=None)

    started_at = utc_now()
    run_id = f"run_{uuid.uuid4().hex}"
    trace_id = f"trace_{uuid.uuid4().hex}"
    finished_at = utc_now()
    run_record = {
        "run_id": run_id,
        "trace_id": trace_id,
        "started_at_utc": started_at,
        "finished_at_utc": finished_at,
        "component": config.get("component", "control_plane"),
        "status": "completed",
        "model_invocations": 0,
        "network_calls": 0,
    }
    runs_dir = state_dir / "runs"
    runs_dir.mkdir(parents=True, exist_ok=True)
    with (runs_dir / f"{run_id}.json").open("w", encoding="utf-8") as handle:
        json.dump(run_record, handle, indent=2, sort_keys=True)
        handle.write("\n")
    _append_audit(state_dir, "TICK_COMPLETED", **run_record)
    return TickResult(exit_code=0, run_id=run_id, trace_id=trace_id)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--config", required=True, help="path to JSON configuration")
    arguments = parser.parse_args(argv)
    return run(arguments.config).exit_code


if __name__ == "__main__":
    raise SystemExit(main())
