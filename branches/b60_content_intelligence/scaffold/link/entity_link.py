#!/usr/bin/env python3
"""
entity_link.py — 2-3 entity scene linker: typed-tag join + geo predicate + abductive verify.

Grounded in `salience_heavy` linking + `reason` (BUILD.md stages 2-3):
  * typed-tag join (PLACE:punjab != TOPIC:punjab) gives a relatedness score used only
    to RANK candidates.
  * a BOOLEAN geo-relevance predicate LICENSES a join (an overlapping geo_key is
    required before a regional persona may be linked to a national news item).
  * the link itself is ABDUCTIVE generate-and-score: a proposer emits candidate
    {target, relation, rationale}; a SEPARATE verifier scores relevance / non-
    obviousness / evidential support; only the top survive.
  * arity is hard-enforced to {2,3}; for arity 3 every pair must be geo-licensed
    (triad closure), so a scene can't smuggle in an un-licensed third entity.

The proposer + verifier go through backends.ModelBackend (mock by default → swap in a
real model). Structural geo/tag signal is weighted ABOVE the verifier so an un-grounded
model can't manufacture a spurious link past the geo predicate. Stdlib only.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402
import backends  # noqa: E402

ARITY_MIN, ARITY_MAX = 2, 3
W_STRUCTURAL, W_VERIFIER = 0.7, 0.3   # structural dominates so geo predicate governs


def _geo_overlap(a, b):
    return set(a["links"]["geo_keys"]) & set(b["links"]["geo_keys"])


def _tag_relatedness(a, b):
    """Weighted Jaccard over typed tags keyed by (type, value) — type-namespaced join."""
    def keyed(n):
        return {(t["type"], t["value"]): t.get("weight", 0.5) for t in n["links"]["typed_tags"]}
    ka, kb = keyed(a), keyed(b)
    shared = set(ka) & set(kb)
    if not shared:
        return 0.0
    num = sum(min(ka[k], kb[k]) for k in shared)
    den = sum(max(ka.get(k, 0), kb.get(k, 0)) for k in set(ka) | set(kb))
    return round(num / den, 4) if den else 0.0


def _relation(anchor, cand, geo):
    a_types = {t["type"] for t in anchor["links"]["typed_tags"]}
    c_types = {t["type"] for t in cand["links"]["typed_tags"]}
    if cand["item_type"] == "NEWS" or "POLICY" in c_types:
        return "AFFECTS"          # a news/policy item affects the persona's audience
    if any(t["type"] == "PLACE" for t in anchor["links"]["typed_tags"]) and \
       any(g in geo for g in geo):
        return "SAME_PLACE"
    if {"TOPIC"} & a_types & c_types:
        return "SAME_TOPIC"
    return "SAME_AUDIENCE"


def _propose_and_verify(anchor, cand, backend):
    """Abductive: propose candidate link rationales, score with a separate verifier."""
    geo = _geo_overlap(anchor, cand)
    geo_ok = len(geo) > 0
    related = _tag_relatedness(anchor, cand)
    structural = min(1.0, 0.6 * min(1.0, len(geo) / 3.0) + 0.4 * related)

    relation = _relation(anchor, cand, geo)
    prompt = (f"Link {anchor['node_id']} ({anchor['known_for']['primary']}) to "
              f"{cand['node_id']} ({cand['known_for']['primary']}) via {relation}; "
              f"shared geo={sorted(geo)}. Rate relevance, non-obviousness, evidence.")
    candidates = backend.propose(prompt, n=8)               # proposer samples
    verifier = max(backend.judge(prompt, c, reference=relation) for c in candidates)  # separate scorer
    strength = round(W_STRUCTURAL * structural + W_VERIFIER * verifier, 4)
    return {
        "node_id": cand["node_id"], "relation": relation, "geo_relevance_ok": geo_ok,
        "shared_geo": sorted(geo), "tag_relatedness": related, "structural": round(structural, 4),
        "verifier_score": round(verifier, 4), "strength": strength if geo_ok else 0.0,
        "rationale": f"{len(geo)} shared geo_key(s); relatedness {related}; verifier {round(verifier,3)}",
    }


def link_scene(anchor_id, records, backend=None, arity=2):
    """Pick the best geo-licensed {2,3}-entity scene anchored on `anchor_id`.
    Returns {anchor, members, links, arity, licensed} and writes suggested_link_partners
    onto the anchor node. Un-licensed candidates (no geo overlap) are excluded."""
    backend = backend or backends.get_backend("mock")
    arity = max(ARITY_MIN, min(ARITY_MAX, arity))
    by_id = {r["node"]["node_id"]: r["node"] for r in records}
    anchor = by_id[anchor_id]

    scored = sorted([_propose_and_verify(anchor, by_id[c], backend)
                     for c in by_id if c != anchor_id], key=lambda x: -x["strength"])
    # write ONLY the schema-allowed keys onto the node; keep diagnostics on the scene.
    anchor["links"]["suggested_link_partners"] = [
        {"node_id": s["node_id"], "relation": s["relation"],
         "strength": s["strength"], "geo_relevance_ok": s["geo_relevance_ok"]}
        for s in scored]

    licensed = [s for s in scored if s["geo_relevance_ok"] and s["strength"] > 0]
    licensed.sort(key=lambda x: -x["strength"])
    partners = licensed[:arity - 1]

    members = [anchor_id] + [p["node_id"] for p in partners]
    # triad closure: for arity 3 every pair must be geo-licensed, else drop to arity 2.
    if len(members) == 3:
        if not _geo_overlap(by_id[members[1]], by_id[members[2]]):
            members, partners = members[:2], partners[:1]

    return {
        "anchor": anchor_id, "members": members, "arity": len(members),
        "links": partners, "candidates": scored, "licensed": len(members) >= ARITY_MIN,
        "grounded_in": common.grounded_in("link"),
    }


if __name__ == "__main__":
    sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    from ingest import build_nodes  # noqa: E402
    from salience import scoring  # noqa: E402
    recs, _ = build_nodes.ingest()
    for r in recs:
        scoring.apply(r)
    scene = link_scene("IN_OWN_PERSONA_CHACHA", recs, arity=2)
    print(f"scene anchor={scene['anchor']} arity={scene['arity']} licensed={scene['licensed']}")
    print(f"members: {scene['members']}")
    for p in scene["links"]:
        print(f"  -> {p['node_id']:26s} rel={p['relation']:13s} geo_ok={p['geo_relevance_ok']} "
              f"strength={p['strength']}  ({p['rationale']})")
    print("\nranked candidates (geo predicate governs):")
    for s in scene["candidates"]:
        print(f"  {s['node_id']:26s} geo_ok={s['geo_relevance_ok']!s:5s} shared={s['shared_geo']} strength={s['strength']}")
