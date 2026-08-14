#!/usr/bin/env python3
"""
controller.py — chains the two SEPARATED steps: brain (step 1) -> gate (step 2).

The pipeline is deliberately dissected into two independent things:
  * `brain.py`  — collect -> connect -> draft + flag safety facts (NO safety decision),
  * `gate.py`   — the stop/yellow safety check + emit (the ONLY place safety is decided).

This controller is just the thin runner that does step 1 then step 2 and reports a combined
status. You can also run each step on its own (`python3 orch/brain.py`, then `orch/gate.py`).
The structural invariant still holds: nothing reaches the outbox except through the gate.
Stdlib only, deterministic, offline by default.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import backends  # noqa: E402
from orch import brain as _brain  # noqa: E402
from orch import gate as _gate  # noqa: E402

# the two-step spine. emit lives behind the gate; the brain never reaches it.
SPINE = [("brain", []), ("gate", ["brain"]), ("emit", ["gate"])]

# gate verdict -> the controller's coarse status (kept for back-compat with run_pipeline)
_STATUS = {"GREEN": "EMITTED", "YELLOW": "EMITTED", "STOP": "HELD_COUNSEL"}


def assert_emit_guarded():
    """Static check: emit may have exactly one predecessor, the gate."""
    preds = dict(SPINE)["emit"]
    if preds != ["gate"]:
        raise AssertionError(f"emit predecessors must be ['gate'], got {preds}")
    return True


def run(anchor_id=None, backend=None, emit=True, arity=2):
    assert_emit_guarded()
    backend = backend or backends.get_backend("mock")

    # --- STEP 1: brain (draft + flag, no safety decision) ----------------
    proposal = _brain.run_brain(anchor_id, backend, arity=arity, save=True)
    if proposal.get("status") != "DRAFTED":
        return {"status": proposal.get("status"), "anchor": proposal.get("anchor"),
                "reason": proposal.get("reason"), "trace": [f"brain: {proposal.get('status')}"]}

    # --- STEP 2: gate (the separated stop/yellow check + emit) -----------
    g = _gate.run_gate(proposal, emit=emit, backend=backend)
    status = "EMITTED" if g["clear_to_render"] and emit else _STATUS.get(g["verdict"], "HELD")
    if g["verdict"] != "STOP" and not g["clear_to_render"]:
        status = "HELD_RESONANCE" if not g["resonance"]["publish_ok"] else "HELD_DISCLOSURE"

    trace = [f"brain: drafted {proposal['scene_id']} (anchor={proposal['anchor']}, members={proposal['members']})",
             f"gate: verdict={g['verdict']} clear_to_render={g['clear_to_render']}"]
    return {"status": status, "anchor": proposal["anchor"], "proposal": proposal,
            "scene": {"anchor": proposal["anchor"], "members": proposal["members"]},
            "brief": proposal["brief"], "gate": g, "verdict": g["verdict"],
            "clearance": g["clearance"], "resonance": g["resonance"],
            "packet": g["packet"], "counsel_queue": g["counsel_queue"],
            "outbox_path": g["outbox_path"], "trace": trace}


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser(description="Run both steps: brain -> gate.")
    ap.add_argument("--anchor", default=None, help="anchor node_id (default: auto-pick medium-tier)")
    ap.add_argument("--no-emit", action="store_true", help="run both steps but do not write the outbox")
    a = ap.parse_args()
    res = run(anchor_id=a.anchor, emit=not a.no_emit)
    print("STATUS:", res["status"], "| verdict:", res.get("verdict"), "| anchor:", res["anchor"])
    for t in res["trace"]:
        print("  -", t)
    if res["status"] == "EMITTED":
        print("  token:", res["clearance"]["disclosure_token"]["token"])
        print("  packet:", res["packet"]["idempotency_key"])
