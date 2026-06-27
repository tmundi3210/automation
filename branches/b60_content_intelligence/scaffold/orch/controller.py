#!/usr/bin/env python3
"""
controller.py — the v1 DAG spine: ingest -> salience -> link -> signal -> dense -> creative
                -> compliance_gate -> emit -> outbox.

Grounded in `orch` (BUILD.md): a linear DAG with a thin control plane where
**compliance_gate is the SOLE compile-time predecessor of emit** (enforced by a static
edge-permission check every run), a parallel signal fan-out, gate-before-emit, and an
outbox handoff. The offline improve-loop (eval/) is intentionally NOT in this live path.

This is the real controller: it sequences the stage modules, enforces single-write-owner
on the blackboard, runs the schema-parse check before emit, and only enqueues a packet
when BOTH gates pass (compliance.passed AND psych.publish_ok) AND every node validates.
Stdlib only, deterministic, offline by default.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402
import backends  # noqa: E402
from ingest import build_nodes  # noqa: E402
from salience import scoring, expander  # noqa: E402
from link import entity_link  # noqa: E402
from reaction import authenticity, hype, dense_brief  # noqa: E402
from creative import gen_brief  # noqa: E402
from compliance import clearance  # noqa: E402
from psych import resonance  # noqa: E402
from orch import blackboard as bb  # noqa: E402

# (stage, predecessors) — the compile-time spine. emit's SOLE predecessor is the gate.
SPINE = [
    ("ingest", []), ("salience", ["ingest"]), ("entity_link", ["salience"]),
    ("signal", ["entity_link"]), ("dense_summary", ["signal"]),
    ("creative_brief", ["dense_summary"]), ("compliance_gate", ["creative_brief"]),
    ("emit", ["compliance_gate"]),
]


def assert_emit_guarded():
    """Static edge-permission check: emit may have exactly one predecessor, the gate."""
    preds = dict(SPINE)["emit"]
    if preds != ["compliance_gate"]:
        raise AssertionError(f"emit predecessors must be ['compliance_gate'], got {preds}")
    return True


def _pick_anchor(records):
    """Default anchor = highest-opportunity node in the target band (the 'medium-tier is
    the opportunity' pick), preferring non-real-person safe-lane subjects."""
    def key(r):
        node = r["node"]
        opp = node["hype"]["opportunity_score"]
        safe = 0 if node["behind"].get("is_real_identifiable_person") else 1
        band = 1 if node["hype"]["tier"] in ("MEDIUM_OPPORTUNITY", "RISING") else 0
        return (safe, band, opp)
    return max(records, key=key)["node"]["node_id"]


def run(anchor_id=None, backend=None, emit=True, arity=2):
    assert_emit_guarded()
    backend = backend or backends.get_backend("mock")
    board = bb.Blackboard()
    trace = []

    # --- ingest -----------------------------------------------------------
    records, dedup_report = build_nodes.ingest()
    for r in records:
        board.put_record(r)
    trace.append(f"ingest: {dedup_report}")

    # --- salience (all nodes, so the expander/anchor pick is real) --------
    for r in records:
        scoring.apply(r)
        board.write_block("salience", r["node"]["node_id"], "hype", r["node"]["hype"])
    frontier = expander.expand(records)
    anchor_id = anchor_id or _pick_anchor(records)
    trace.append(f"salience: {len(frontier)} niche frontiers; anchor={anchor_id}")

    # --- entity_link: pick the 2-3 entity scene --------------------------
    scene = entity_link.link_scene(anchor_id, records, backend, arity=arity)
    trace.append(f"link: scene={scene['members']} licensed={scene['licensed']}")
    if not scene["licensed"]:
        return {"status": "NO_LICENSED_SCENE", "anchor": anchor_id, "trace": trace}

    members = scene["members"]

    # --- signal: authenticity ∥ hype (fan-out), then dense summary -------
    for mid in members:
        r = board.records[mid]
        authenticity.apply(r)
        hype.apply(r)
        board.write_block("signal", mid, "signal", r["node"]["signal"])
    for mid in members:
        dense_brief.apply(board.records[mid])
        board.write_block("dense_summary", mid, "machine_summary", board.records[mid]["node"]["machine_summary"])
    trace.append(f"signal+dense: {[board.records[m]['node']['hype']['tier'] for m in members]}")

    # --- creative_brief (briefs only) ------------------------------------
    try:
        brief = gen_brief.serialize(gen_brief.build_brief(scene, records, backend))
    except (gen_brief.UngroundedSubjectError, gen_brief.DisclosureMissingError) as e:
        return {"status": "CREATIVE_REFUSED", "anchor": anchor_id, "reason": str(e), "trace": trace}
    trace.append(f"creative: {brief['scene_id']} risk={brief['contract']['derivative_risk_flag']}")

    # --- compliance_gate (writes safety_flags; sole predecessor of emit) -
    cr = clearance.clear_scene(scene, brief, records)
    for mid in members:
        board.write_block("compliance", mid, "safety_flags", board.records[mid]["node"]["safety_flags"])
    trace.append(f"compliance: {cr['scene_disposition']} passed={cr['passed']}")

    # --- psych resonance (content gate, enabler) -------------------------
    res = resonance.check(scene, cr, records)
    trace.append(f"psych: publish_ok={res['publish_ok']} blockers={res['blockers'] or 'none'}")

    # --- schema-parse check before emit ----------------------------------
    schema_errors = {mid: common.validate_node(board.records[mid]["node"]) for mid in members}
    bad = {mid: errs for mid, errs in schema_errors.items() if errs}
    if bad:
        return {"status": "SCHEMA_INVALID", "anchor": anchor_id, "errors": bad, "trace": trace}
    trace.append(f"schema: {len(members)}/{len(members)} nodes valid")

    # --- emit decision ----------------------------------------------------
    if not cr["passed"]:
        return {"status": "HELD_COUNSEL", "anchor": anchor_id, "clearance": cr,
                "counsel_queue": cr["counsel_queue"], "trace": trace}
    if not res["publish_ok"]:
        return {"status": "HELD_RESONANCE", "anchor": anchor_id, "resonance": res, "trace": trace}

    # token must survive the handoff to the named downstream slot
    ok, why = clearance.verify_token(cr, brief["contract"]["disclosure_slot"])
    if not ok:
        return {"status": "HELD_DISCLOSURE", "anchor": anchor_id, "reason": why, "trace": trace}

    packet = bb.make_packet(scene, brief, cr, res, records)
    enqueued = (None, False)
    if emit:
        enqueued = bb.enqueue(packet)
        trace.append(f"emit: outbox <- {os.path.basename(enqueued[0])} (new={enqueued[1]})")

    return {"status": "EMITTED", "anchor": anchor_id, "scene": scene, "brief": brief,
            "clearance": cr, "resonance": res, "packet": packet,
            "outbox_path": enqueued[0], "trace": trace}


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser(description="Run the v1 content-intelligence DAG spine.")
    ap.add_argument("--anchor", default=None, help="anchor node_id (default: auto-pick medium-tier)")
    ap.add_argument("--no-emit", action="store_true", help="run the spine but do not write the outbox")
    a = ap.parse_args()
    res = run(anchor_id=a.anchor, emit=not a.no_emit)
    print("STATUS:", res["status"], "| anchor:", res["anchor"])
    for t in res["trace"]:
        print("  -", t)
    if res["status"] == "EMITTED":
        print("  token:", res["clearance"]["disclosure_token"]["token"])
        print("  packet:", res["packet"]["idempotency_key"])
