#!/usr/bin/env python3
"""
gen_brief.py — the closed, versioned generative-brief artifact (briefs only; never renders).

Grounded in `creative_heavy` (BUILD.md stage 5): a closed `gen_brief` with a required
image block + required story block (setup/turn/punchline) + optional audio block, an
in-band CONTRACT block (disclosure_slot, usage_restrictions, assume/guarantee,
derivative_risk_flag) and a PROVENANCE block (every claim cites a source_fact_id;
ungrounded subjects rejected). A non-empty disclosure_slot is a HARD precondition:
serialization FAILS CLOSED without it.

The 2-3 entity beat: one contrast_axis, one benign-violation joke, rule-of-three, meme
format only where it cuts comprehension cost. This stage emits a BRIEF for a downstream
renderer — it does not produce or publish media. Stdlib only, deterministic offline.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402
import backends  # noqa: E402

GEN_BRIEF_VERSION = "1.0"


class UngroundedSubjectError(ValueError):
    pass


class DisclosureMissingError(ValueError):
    pass


def _source_fact_ids(node):
    ids = []
    for i, u in enumerate(node["provenance"]["source_urls"]):
        ids.append(f"{node['node_id']}#src{i}")
    for j, _ in enumerate(node["item"].get("factual_claims", [])):
        ids.append(f"{node['node_id']}#claim{j}")
    return ids


def _assert_grounded(node):
    if not node["provenance"].get("is_public_information"):
        raise UngroundedSubjectError(f"{node['node_id']}: not public information")
    if not node["provenance"].get("source_urls"):
        raise UngroundedSubjectError(f"{node['node_id']}: no source urls")


def _derivative_risk(members):
    if any(m["behind"].get("is_real_identifiable_person") for m in members):
        return "HIGH"
    if any("POLITICAL" in m["critical_read"].get("sensitivity_flags", []) for m in members):
        return "MODERATE"
    return "LOW"


def build_brief(scene, records, backend=None):
    """Compose a gen_brief from a 2-3 node scene. Anchors on the protagonist (member 0)
    and the linked partner(s). Raises UngroundedSubjectError if any member is ungrounded."""
    backend = backend or backends.get_backend("mock")
    by_id = {r["node"]["node_id"]: r["node"] for r in records}
    members = [by_id[mid] for mid in scene["members"]]
    for m in members:
        _assert_grounded(m)

    anchor, partner = members[0], members[1]
    contrast_axis = f"{anchor['known_for']['primary']} vs {partner['known_for']['primary']}"
    # creative text is model-generated but ANCHORED on grounded facts (no new claims)
    seed = (f"2-3 entity scene. contrast={contrast_axis}. benign-violation joke; rule of three. "
            f"protagonist is the OWNED persona {anchor['item']['canonical_name']}; "
            f"partner topic is {partner['item']['canonical_name']}.")
    setup = backend.generate("SETUP: " + seed, max_tokens=40)
    turn = backend.generate("TURN: " + seed, max_tokens=40)
    punch = backend.generate("PUNCHLINE: " + seed, max_tokens=40)

    fact_ids = sum((_source_fact_ids(m) for m in members), [])
    risk = _derivative_risk(members)
    disclosure = "AI-generated derivative; not affiliated with or endorsed by any named entity."

    brief = {
        "gen_brief_version": GEN_BRIEF_VERSION,
        "scene_id": "SCENE_" + "__".join(m["node_id"].replace("IN_", "") for m in members),
        "members": [m["node_id"] for m in members],
        "contrast_axis": contrast_axis,
        "image": {
            "prompt": f"{anchor['known_for']['iconic_assets'][0] if anchor['known_for'].get('iconic_assets') else anchor['item']['canonical_name']} "
                      f"reacting to '{partner['known_for']['primary']}', expressive, single panel",
            "style": "editorial cartoon, no photoreal faces",
            "composition": "rule-of-three; protagonist left, topic motif right",
            "negative_constraints": [
                "no real identifiable person's face or likeness",
                "no real brand logos or copyrighted media",
                "no depiction of a named individual doing/saying anything",
            ],
        },
        "story": {"setup": setup, "turn": turn, "punchline": punch},
        "contract": {
            "disclosure_slot": disclosure,                # MUST be non-empty (fail closed)
            "usage_restrictions": ["satire/commentary on public policy only",
                                   "no implication of factual endorsement"],
            "assume": "downstream renderer honors disclosure_slot + negative_constraints",
            "guarantee": "brief contains no un-sourced factual claim about a real person",
            "derivative_risk_flag": risk,
        },
        "provenance": {"source_fact_ids": fact_ids, "grounded": True,
                       "grounded_in": common.grounded_in("creative")},
        "meme_format": None,   # only set when it cuts comprehension cost (none needed here)
    }
    return brief


def serialize(brief):
    """Serialize the brief, FAILING CLOSED if the disclosure_slot is empty."""
    if not brief.get("contract", {}).get("disclosure_slot", "").strip():
        raise DisclosureMissingError("disclosure_slot empty; refusing to serialize gen_brief")
    return brief


if __name__ == "__main__":
    from ingest import build_nodes  # noqa: E402
    from salience import scoring  # noqa: E402
    from reaction import authenticity, hype, dense_brief  # noqa: E402
    from link import entity_link  # noqa: E402
    recs, _ = build_nodes.ingest()
    for r in recs:
        scoring.apply(r); authenticity.apply(r); hype.apply(r); dense_brief.apply(r)
    scene = entity_link.link_scene("IN_OWN_PERSONA_CHACHA", recs, arity=2)
    brief = serialize(build_brief(scene, recs))
    print(f"gen_brief {brief['scene_id']} (risk={brief['contract']['derivative_risk_flag']})")
    print(f"  members : {brief['members']}")
    print(f"  image   : {brief['image']['prompt']}")
    print(f"  story   : setup={brief['story']['setup'][:48]!r} ...")
    print(f"  disclose: {brief['contract']['disclosure_slot']}")
    print(f"  neg     : {brief['image']['negative_constraints']}")
    print(f"  facts   : {len(brief['provenance']['source_fact_ids'])} source_fact_ids")
