#!/usr/bin/env python3
"""
blackboard.py — typed blackboard (one write-owner per field) + durable outbox queue.

Grounded in `orch` (BUILD.md): "Thin control plane: typed blackboard (one write-owner
per field)... The cross-session handoff is an OUTBOX: the gate writes an immutable,
schema-validated, sanitized EmitPacket to a durable queue with an idempotency key; the
downstream session PULLS it (no in-process call across the trust boundary)."

The single-write-owner rule is enforced here: a stage that tries to overwrite a block
owned by another stage raises. The outbox is a directory of immutable JSON packets keyed
by idempotency key; `pull()` reads them the way a separate downstream session would.
Stdlib only.
"""
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402

OUTBOX = common.OUTBOX_DIR


class OwnershipError(Exception):
    pass


class Blackboard:
    """Holds pipeline records + enforces one write-owner per (node_id, block)."""

    def __init__(self):
        self.records = {}        # node_id -> record {node, raw, meta}
        self._owner = {}         # (node_id, block) -> stage
        self.trace = []

    def put_record(self, record):
        self.records[record["node"]["node_id"]] = record

    def write_block(self, stage, node_id, block, value):
        key = (node_id, block)
        owner = self._owner.get(key)
        if owner and owner != stage:
            raise OwnershipError(f"{stage} may not overwrite '{block}' on {node_id} (owned by {owner})")
        self._owner[key] = stage
        self.records[node_id]["node"][block] = value
        self.trace.append(f"{stage} wrote {block} on {node_id}")

    def owners(self):
        return dict(self._owner)


def _sanitize(record):
    """Strip internal sidecars (raw inputs, meta) — only the schema node leaves the trust boundary."""
    return record["node"]


def make_packet(scene, brief, clearance, resonance, records):
    """Build the immutable EmitPacket (idempotency-keyed) for the outbox."""
    by_id = {r["node"]["node_id"]: r for r in records}
    nodes = [_sanitize(by_id[mid]) for mid in scene["members"]]
    idem = "PKT_" + brief["scene_id"]
    return {
        "idempotency_key": idem,
        "scene_id": brief["scene_id"],
        "members": scene["members"],
        "nodes": nodes,                       # schema-validated InformationNodes
        "gen_brief": brief,
        "clearance": {k: clearance[k] for k in
                      ("scene_disposition", "passed", "disclosure_token", "per_member", "clearance_version")},
        "resonance": {k: resonance[k] for k in ("publish_ok", "drivers", "honest_sentiment", "safe_lane_ok")},
        "disclosure_token": clearance.get("disclosure_token"),
    }


def enqueue(packet, outbox=OUTBOX):
    """Write an immutable packet to the durable outbox (idempotent on key)."""
    os.makedirs(outbox, exist_ok=True)
    path = os.path.join(outbox, packet["idempotency_key"] + ".json")
    if os.path.exists(path):
        return path, False                    # idempotent: already enqueued
    with open(path, "w") as f:
        json.dump(packet, f, ensure_ascii=False, indent=2, sort_keys=True)
    return path, True


def pull(outbox=OUTBOX):
    """Downstream-session view: read all enqueued packets (no in-process coupling)."""
    if not os.path.isdir(outbox):
        return []
    return [common.read_json(os.path.join(outbox, f))
            for f in sorted(os.listdir(outbox)) if f.endswith(".json")]


if __name__ == "__main__":
    bb = Blackboard()
    bb.put_record({"node": {"node_id": "IN_X", "item": {}}, "raw": {}, "meta": {}})
    bb.write_block("salience", "IN_X", "hype", {"raw_hype": 0.4})
    try:
        bb.write_block("creative", "IN_X", "hype", {"raw_hype": 0.9})
    except OwnershipError as e:
        print("single-write-owner enforced:", e)
    print("owners:", {f"{k[0]}/{k[1]}": v for k, v in bb.owners().items()})
    print("outbox dir:", os.path.relpath(OUTBOX, common.REPO_ROOT))
