#!/usr/bin/env python3
"""
build_nodes.py — Tier-0/1/2 pull (mock fixtures) -> dedup -> partial InformationNode "understanding".

Grounded in the `ir` data layer (BUILD.md stage 1). What is REAL here: the dedup
pass, the node-id minting, and the assembly of the schema's understanding blocks
(provenance / item / behind / place / known_for / audience / critical_read /
links.typed_tags+geo_keys / grounding_status) from source records. What is a HOOK
POINT: the live source pulls themselves — `load_raw()` reads deterministic offline
fixtures standing in for YouTube Data v3 + Reddit OAuth (Tier 0), RSS/sitemaps
(Tier 1), and Trends/Wikipedia (Tier 2). Swap `load_raw()` for real fetchers and
nothing downstream changes.

The node returned is PARTIAL: signal/hype/machine_summary/safety_flags are filled by
later stages. Validation against the schema happens once, right before emit.

Stdlib only, deterministic, offline.
"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402
from ingest import dedup as _dedup  # noqa: E402

FIXTURES = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_fixtures", "raw_items.json")

_ITEM_TYPE = {"own_persona": "PERSON", "public_news": "NEWS", "block_trigger": "PERSON"}


def load_raw(path=FIXTURES):
    """HOOK POINT: replace with live Tier-0/1/2 fetchers. Returns raw source records."""
    return common.read_json(path)


def _node_id(raw_id):
    body = re.sub(r"^raw_", "", raw_id)
    body = re.sub(r"[^A-Za-z0-9]+", "_", body).upper().strip("_")
    return "IN_" + body


def _provenance(rec):
    urls = [_dedup.canonical_url(u) for u in rec.get("source_urls", [])]
    # de-dup the canonical url list while preserving order, and re-attach scheme.
    seen, canon = set(), []
    for u in urls:
        if u not in seen:
            seen.add(u)
            canon.append("https://" + u)
    prov = {
        "source_urls": canon,
        "collection_method": rec["collection_method"],
        "first_seen": rec["first_seen"],
        "is_public_information": rec["is_public_information"],
    }
    if rec.get("mirror_urls"):
        prov["mirror_urls"] = rec["mirror_urls"]
    if rec.get("publisher"):
        prov["publisher"] = rec["publisher"]
    if rec.get("license_or_rights"):
        prov["license_or_rights"] = rec["license_or_rights"]
    # provenance confidence: SCRAPE is less trustworthy than API/RSS.
    prov["confidence"] = round({"API": 0.95, "RSS": 0.9, "SEARCH": 0.75,
                                "MANUAL": 0.85, "CROSSPOST_TRACE": 0.7, "SCRAPE": 0.55}
                               .get(rec["collection_method"], 0.7), 2)
    return prov


def build_node(rec):
    """Assemble the partial understanding node (schema blocks ingest owns)."""
    node = {
        "node_id": _node_id(rec["_id"]),
        "schema_version": "1.0",
        "ingested_at": rec["first_seen"],
        "item_type": _ITEM_TYPE.get(rec.get("_lane", ""), "NEWS"),
        "provenance": _provenance(rec),
        "item": rec["item"],
        "behind": rec["behind"],
        "place": rec["place"],
        "known_for": rec["known_for"],
        "audience": rec["audience"],
        "critical_read": rec["critical"],
        "links": {
            "typed_tags": rec.get("tags", []),
            "geo_keys": rec.get("geo_keys", []),
            "suggested_link_partners": [],  # filled by the link stage
        },
        "grounding_status": {
            "overall": "MIXED",
            "signal_grounded": bool(rec.get("raw_reactions", {}).get("per_platform")),
            "hype_grounded": "salience_signals" in rec,
            "audience_grounded": "size_estimate" in rec.get("audience", {}),
            "stale_after": rec["first_seen"][:10] + "T23:59:59Z",
        },
    }
    return node


def ingest(path=FIXTURES):
    """Full ingest: load -> dedup -> build partial nodes + sidecar raw inputs.
    Returns (records, dedup_report) where each record is
    {event_id, node, raw:{salience_signals, market, raw_reactions}}."""
    raw = load_raw(path)
    deduped, report = _dedup.dedup(raw)
    records = []
    for rec in deduped:
        if rec.get("_dup_of"):
            continue  # collapsed near-duplicate; understanding already built for the canonical
        records.append({
            "event_id": rec["event_id"],
            "node": build_node(rec),
            "raw": {
                "lane": rec.get("_lane"),
                "salience_signals": rec.get("salience_signals", {}),
                "market": rec.get("market", {}),
                "raw_reactions": rec.get("raw_reactions", {}),
            },
        })
    return records, report


if __name__ == "__main__":
    recs, rep = ingest()
    print(f"ingest: {rep}")
    for r in recs:
        n = r["node"]
        gk = ",".join(n["links"]["geo_keys"])
        print(f"  {n['node_id']:26s} type={n['item_type']:6s} lane={r['raw']['lane']:13s} "
              f"realperson={n['behind']['is_real_identifiable_person']!s:5s} geo=[{gk}]")
