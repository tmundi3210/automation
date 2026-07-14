# Phase A control plane

`python3 -m orchestrator.control_plane.tick --config orchestrator/config/config.json`
runs one deterministic, no-op scheduling tick.

It records a run and appends a JSONL audit entry under `orchestrator/state/`.
`orchestrator/control_plane/budget.py` tracks local component counters and
selects deterministic degradation rungs at 70%, 85%, 95%, and 100%.

Creating `orchestrator/state/PAUSE` makes a tick audit `PAUSED` and exit 1.
Removing it restores normal ticks.

Phase A deliberately has no adapters, model calls, or network operations.
