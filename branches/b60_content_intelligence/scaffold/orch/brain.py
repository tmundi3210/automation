#!/usr/bin/env python3
"""
brain.py — STEP 1 of 2: the BRAIN. Collect -> connect -> draft + flag safety facts.

This is the creative half, deliberately separated from the safety half (`gate.py`). The
brain does the thinking only: ingest -> salience -> link -> signal -> dense -> creative
draft. It makes NO safety decision and sets NO safety_flags — it just *reports* the facts
the gate will need (real person? political? minor? voice/likeness? source license?). It
writes a Proposal to `_proposals/` and stops. The gate is a separate step you run next.

Why split it: keeping the checks out of the brain prevents "unexplained stops" mid-thought
— the brain always produces a clean draft + an explicit facts list, and the gate gives one
clear, reviewable STOP/YELLOW/GREEN verdict you can act on. Stdlib only, deterministic.
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

PROPOSALS = common.PROPOSALS_DIR


def _pick_anchor(records):
    """Default anchor = highest-opportunity node in the target band (the 'medium-tier is
    the opportunity' pick), preferring non-real-person safe-lane subjects."""
    def key(r):
        node = r["node"]
        return (0 if node["behind"].get("is_real_identifiable_person") else 1,
                1 if node["hype"]["tier"] in ("MEDIUM_OPPORTUNITY", "RISING") else 0,
                node["hype"]["opportunity_score"])
    return max(records, key=key)["node"]["node_id"]


def safety_facts(node):
    """REPORT ONLY — the facts the gate will judge. The brain never decides on these."""
    b = node["behind"]
    crit = node["critical_read"].get("sensitivity_flags", [])
    return {
        "node_id": node["node_id"],
        "is_real_identifiable_person": bool(b.get("is_real_identifiable_person")),
        "political_or_election": "POLITICAL" in crit,
        "minor_or_protected": b.get("public_figure_status") == "MINOR_OR_PROTECTED",
        "uses_voice_or_likeness": bool(b.get("is_real_identifiable_person")) and (
            b.get("entity_kind") == "INDIVIDUAL"
            or "voice" in " ".join(node["known_for"].get("iconic_assets", []))),
        "source_method": node["provenance"].get("collection_method"),
        "license_or_rights": node["provenance"].get("license_or_rights", "unknown"),
        "is_public_information": bool(node["provenance"].get("is_public_information")),
        "sensitivity_flags": crit,
    }


def reaction_read(node):
    a = node["signal"]["authenticity"]
    return {"node_id": node["node_id"], "aggregate_sentiment": node["signal"]["aggregate_sentiment"],
            "bot_astroturf_risk": a["bot_astroturf_risk"], "organic_score": a["organic_score"],
            "manipulation_signals": a["manipulation_signals"]}


def run_brain(anchor_id=None, backend=None, arity=2, save=True):
    """Run step 1 and return a Proposal (also written to _proposals/ when save=True).
    The Proposal carries PARTIAL nodes (no safety_flags) + the drafted brief + the facts."""
    backend = backend or backends.get_backend("mock")
    records, dedup_report = build_nodes.ingest()

    # collect: salience over all nodes, so the anchor pick + niche frontier are real
    for r in records:
        scoring.apply(r)
    frontier = expander.expand(records)
    anchor_id = anchor_id or _pick_anchor(records)

    # connect: the geo-licensed 2-3 entity scene
    scene = entity_link.link_scene(anchor_id, records, backend, arity=arity)
    if not scene["licensed"]:
        return {"status": "NO_LICENSED_SCENE", "anchor": anchor_id, "frontier": frontier}
    members = scene["members"]
    by_id = {r["node"]["node_id"]: r for r in records}

    # understand: reaction read + dense summary for the scene members
    for mid in members:
        authenticity.apply(by_id[mid])
        hype.apply(by_id[mid])
        dense_brief.apply(by_id[mid])

    # draft the recipe (briefs only — nothing rendered, no safety decision)
    try:
        brief = gen_brief.serialize(gen_brief.build_brief(scene, records, backend))
    except (gen_brief.UngroundedSubjectError, gen_brief.DisclosureMissingError) as e:
        return {"status": "CREATIVE_REFUSED", "anchor": anchor_id, "reason": str(e)}

    proposal = {
        "step": "brain",
        "status": "DRAFTED",
        "scene_id": brief["scene_id"],
        "anchor": anchor_id,
        "members": members,
        "nodes": {mid: by_id[mid]["node"] for mid in members},      # PARTIAL: no safety_flags
        "raw": {mid: by_id[mid]["raw"] for mid in members},
        "brief": brief,
        "reaction_read": [reaction_read(by_id[mid]["node"]) for mid in members],
        "safety_facts": [safety_facts(by_id[mid]["node"]) for mid in members],
        "dedup_report": dedup_report,
        "note": "BRAIN output. No safety decision made here — hand to gate.py (step 2).",
    }
    if save:
        common.write_json(os.path.join(PROPOSALS, brief["scene_id"] + ".json"), proposal)
    return proposal


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser(description="STEP 1 — the brain (collect -> connect -> draft).")
    ap.add_argument("--anchor", default=None)
    a = ap.parse_args()
    p = run_brain(anchor_id=a.anchor)
    if p.get("status") != "DRAFTED":
        print("BRAIN:", p["status"], p.get("reason", ""))
        sys.exit(0)
    print(f"BRAIN drafted {p['scene_id']}  (anchor={p['anchor']}, members={p['members']})")
    print("  reaction read:")
    for rr in p["reaction_read"]:
        print(f"    {rr['node_id']:26s} sent={rr['aggregate_sentiment']:+.2f} botrisk={rr['bot_astroturf_risk']:.2f}")
    print("  safety FACTS handed to the gate (not judged here):")
    for sf in p["safety_facts"]:
        print(f"    {sf['node_id']:26s} real={sf['is_real_identifiable_person']!s:5s} "
              f"political={sf['political_or_election']!s:5s} voice/likeness={sf['uses_voice_or_likeness']!s:5s} "
              f"license={sf['license_or_rights']}")
    print(f"  -> proposal written to _proposals/{p['scene_id']}.json ; run gate.py next")
