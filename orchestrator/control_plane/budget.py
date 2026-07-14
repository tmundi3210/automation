"""Local budget counters and deterministic degradation decisions.

This module deliberately has no provider, adapter, or network dependency.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass
import json
from pathlib import Path
from typing import Any, Mapping


RUNG_WARN = "warn"
RUNG_DEFER_NONURGENT = "defer-nonurgent"
RUNG_PAUSE_NEW = "pause-new"
RUNG_HARD_STOP = "hard-stop"
RUNG_NORMAL = "normal"


@dataclass(frozen=True)
class BudgetStatus:
    """The current usage and the strongest applicable degradation rung."""

    component: str
    invocations: float
    wall_minutes: float
    invocation_percent: float
    wall_minutes_percent: float
    utilization_percent: float
    rung: str


def load_budgets(path: str | Path) -> dict[str, Any]:
    """Load the committed JSON budget configuration."""
    with Path(path).open(encoding="utf-8") as handle:
        budgets = json.load(handle)
    if not isinstance(budgets.get("components"), dict):
        raise ValueError("budget configuration must contain components")
    return budgets


def rung_for_percent(percent: float) -> str:
    """Map the strongest resource utilization to its required action."""
    if percent >= 100:
        return RUNG_HARD_STOP
    if percent >= 95:
        return RUNG_PAUSE_NEW
    if percent >= 85:
        return RUNG_DEFER_NONURGENT
    if percent >= 70:
        return RUNG_WARN
    return RUNG_NORMAL


def evaluate_budget(
    budgets: Mapping[str, Any], component: str, counters: Mapping[str, Any]
) -> BudgetStatus:
    """Evaluate one component using the stricter of its two daily resources."""
    try:
        limits = budgets["components"][component]
        invocation_limit = float(limits["invocations_max"])
        wall_limit = float(limits["wall_minutes_max"])
    except (KeyError, TypeError, ValueError) as exc:
        raise ValueError(f"invalid budget configuration for {component}") from exc
    if invocation_limit <= 0 or wall_limit <= 0:
        raise ValueError(f"budget limits for {component} must be positive")

    invocations = float(counters.get("invocations", 0))
    wall_minutes = float(counters.get("wall_minutes", 0))
    if invocations < 0 or wall_minutes < 0:
        raise ValueError("budget counters must not be negative")
    invocation_percent = invocations * 100 / invocation_limit
    wall_minutes_percent = wall_minutes * 100 / wall_limit
    utilization_percent = max(invocation_percent, wall_minutes_percent)
    return BudgetStatus(
        component=component,
        invocations=invocations,
        wall_minutes=wall_minutes,
        invocation_percent=invocation_percent,
        wall_minutes_percent=wall_minutes_percent,
        utilization_percent=utilization_percent,
        rung=rung_for_percent(utilization_percent),
    )


class BudgetCounterStore:
    """A small local JSON ledger for deterministic budget accounting."""

    def __init__(self, path: str | Path) -> None:
        self.path = Path(path)

    def read(self) -> dict[str, dict[str, float]]:
        if not self.path.exists():
            return {}
        with self.path.open(encoding="utf-8") as handle:
            contents = json.load(handle)
        components = contents.get("components", {})
        if not isinstance(components, dict):
            raise ValueError("budget counter store must contain components")
        return components

    def record(
        self, component: str, *, invocations: float = 0, wall_minutes: float = 0
    ) -> dict[str, float]:
        """Append usage by replacing the local counter snapshot atomically."""
        if invocations < 0 or wall_minutes < 0:
            raise ValueError("budget increments must not be negative")
        components = self.read()
        current = components.get(component, {"invocations": 0, "wall_minutes": 0})
        updated = {
            "invocations": float(current.get("invocations", 0)) + invocations,
            "wall_minutes": float(current.get("wall_minutes", 0)) + wall_minutes,
        }
        components[component] = updated
        self.path.parent.mkdir(parents=True, exist_ok=True)
        temporary = self.path.with_suffix(self.path.suffix + ".tmp")
        with temporary.open("w", encoding="utf-8") as handle:
            json.dump({"components": components}, handle, indent=2, sort_keys=True)
            handle.write("\n")
        temporary.replace(self.path)
        return updated


def status_as_dict(status: BudgetStatus) -> dict[str, Any]:
    """Return a JSON-ready representation for an audit record or caller."""
    return asdict(status)
