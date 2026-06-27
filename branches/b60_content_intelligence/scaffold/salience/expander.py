#!/usr/bin/env python3
"""
expander.py — bounded region -> niche -> top-5 attention-frontier expander.

Grounded in `salience_heavy` (BUILD.md stage 2): "region->niche->top-5 expander over
a closed single-parent hierarchy, strictly-decreasing-salience frontier, depth<=4 /
breadth<=8 / top_k=5 + long-tail cutoff (finite, deterministic rebuild)."

The expander is what makes the MAP traversal terminate: every level is capped, the
frontier only keeps strictly-decreasing-opportunity children, and the whole thing is
a pure function of the scored node set (deterministic rebuild). Stdlib only.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402

BOUNDS = {"max_depth": 4, "max_breadth": 8, "top_k": 5, "min_opportunity": 0.02}


def _region_key(node):
    rh = node["place"].get("region_hierarchy") or [node["place"]["primary_region"]]
    return "/".join(rh[:BOUNDS["max_depth"]])


def expand(records):
    """Group scored records into region -> domain -> subgenre buckets and return the
    top-k-by-opportunity frontier per niche, strictly decreasing. Returns a list of
    {region, domain, subgenre, frontier:[{node_id, opportunity, tier}]}."""
    buckets = {}
    for r in records:
        node, meta = r["node"], r.get("meta", {}).get("salience", {})
        opp = meta.get("opportunity_score", node.get("hype", {}).get("opportunity_score", 0.0))
        if opp < BOUNDS["min_opportunity"]:
            continue  # long-tail cutoff
        region = _region_key(node)
        domain = node["known_for"].get("interest_domain", "OTHER")
        for sub in (node["known_for"].get("interest_subgenre") or ["_general"]):
            buckets.setdefault((region, domain, sub), []).append(
                {"node_id": node["node_id"], "opportunity": round(opp, 4),
                 "tier": node.get("hype", {}).get("tier", "?")})

    out = []
    for (region, domain, sub), items in sorted(buckets.items()):
        ranked = sorted(items, key=lambda x: -x["opportunity"])[:BOUNDS["max_breadth"]]
        # strictly-decreasing frontier: keep only items below the running max (dedup ties)
        frontier, last = [], 1.01
        for it in ranked:
            if it["opportunity"] < last:
                frontier.append(it)
                last = it["opportunity"]
            if len(frontier) >= BOUNDS["top_k"]:
                break
        out.append({"region": region, "domain": domain, "subgenre": sub, "frontier": frontier})
    return out


if __name__ == "__main__":
    from ingest import build_nodes  # noqa: E402
    from salience import scoring as _sal  # noqa: E402
    recs, _ = build_nodes.ingest()
    for r in recs:
        _sal.apply(r)
    print(f"expander bounds={BOUNDS}")
    for niche in expand(recs):
        head = f"{niche['region']} | {niche['domain']}/{niche['subgenre']}"
        chain = " > ".join(f"{f['node_id'].replace('IN_','')}({f['opportunity']})" for f in niche["frontier"])
        print(f"  {head:48s} -> {chain}")
