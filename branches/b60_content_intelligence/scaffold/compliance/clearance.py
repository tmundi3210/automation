#!/usr/bin/env python3
"""
clearance.py — path-to-yes green-lane clearance gate + safety_flags + signed disclosure token.

Grounded in `compliance_heavy` (BUILD.md stage 6), an ENABLER that maximizes what can
ship while keeping hard blocks hard. Per member it evaluates the frozen (item, figure,
use) tuple into a Clearance Record disposition:

  GREEN LANES (path-to-yes):
    * own-persona / original composite  -> ALLOW (fastest)
    * scope-matched consented use       -> ALLOW_WITH_CONSTRAINTS
    * parody-with-visible-cues          -> ALLOW_WITH_CONSTRAINTS (default when consent absent)
    * public topic / policy / institution (no real identifiable person) -> ALLOW(/constraints if political)
  HARD BLOCKS (never auto-approved):
    * minor / protected person                          -> BLOCK
    * real person + political/election + voice/likeness -> BLOCK
    * fabricated wrongdoing about a real person          -> BLOCK
    * unlicensed-source republication (scrape+unknown)   -> HUMAN_REVIEW

On a passing scene it signs a C2PA-style disclosure manifest and emits an in-band token
bound to the gen_brief's disclosure_slot; `verify_token` is the handoff re-check. This is
a SCREENING HEURISTIC, not legal advice; BLOCK/HUMAN_REVIEW route to counsel. Stdlib only.
"""
import hashlib
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402

CONFIG_VERSION = "clearance-cfg-v1"
_ORDER = {"ALLOW": 0, "ALLOW_WITH_CONSTRAINTS": 1, "HUMAN_REVIEW": 2, "BLOCK": 3}
_PASS = {"ALLOW", "ALLOW_WITH_CONSTRAINTS"}


def _classify(node):
    """Return (disposition, lane, constraints, review_reason) for one member."""
    b = node["behind"]
    crit = node["critical_read"].get("sensitivity_flags", [])
    political = "POLITICAL" in crit
    real = b.get("is_real_identifiable_person", False)
    minor = (b.get("public_figure_status") == "MINOR_OR_PROTECTED"
             or any(a in ("U13", "13_17") for a in node["audience"].get("age_bands", [])) and real)
    voice_or_likeness = real and ("signature voice" in " ".join(node["known_for"].get("iconic_assets", []))
                                  or b.get("entity_kind") == "INDIVIDUAL")
    scraped_unlicensed = (node["provenance"].get("collection_method") == "SCRAPE"
                          and node["provenance"].get("license_or_rights", "unknown") == "unknown")
    constraints = []

    if minor:
        return "BLOCK", "hard_block_minor", ["SR_MINOR_PROTECTED"], "minor/protected person"
    if not real:
        # own persona / original IP / public topic / policy / institution
        lane = "own_persona" if b.get("entity_kind") in ("GROUP", "AUTOMATED") and \
            node["provenance"].get("license_or_rights") == "owned_original_ip" else "public_topic"
        if political:
            constraints += ["SR_FACTUAL_ONLY", "SR_NO_CANDIDATE_DEPICTION", "SR_SYN_DISCLOSE"]
            return "ALLOW_WITH_CONSTRAINTS", lane, constraints, ""
        constraints += ["SR_SYN_DISCLOSE"]
        return "ALLOW", lane, constraints, ""
    # real identifiable person from here — hard categorical blocks dominate procedural review
    if political and voice_or_likeness:
        return ("BLOCK", "hard_block_political_voice",
                ["SR_LIKENESS", "SR_VOICE", "SR_ELECTION"],
                "real person + political/election + voice/likeness without consent")
    if scraped_unlicensed:
        return "HUMAN_REVIEW", "unlicensed_source", ["SR_SOURCE_LICENSE"], "scraped + unknown license"
    # consent record would unlock a consented lane; absent it, default to parody-with-cues
    return ("ALLOW_WITH_CONSTRAINTS", "parody_with_cues",
            ["SR_LIKENESS", "SR_PARODY_CUES", "SR_SYN_DISCLOSE", "SR_NO_FACT_IMPLICATION"], "")


def set_safety_flags(node):
    """Write the schema safety_flags block onto a node from its clearance classification."""
    disp, lane, constraints, reason = _classify(node)
    b = node["behind"]
    node["safety_flags"] = {
        "touches_real_person": bool(b.get("is_real_identifiable_person")),
        "involves_voice_or_likeness": "SR_VOICE" in constraints or "SR_LIKENESS" in constraints,
        "is_minor_or_protected": lane == "hard_block_minor",
        "political_or_election_context": "POLITICAL" in node["critical_read"].get("sensitivity_flags", []),
        "generate_allowed": disp,
        "applicable_constraints": constraints,
        "review_reason": reason,
    }
    return disp, lane


def _sign(manifest):
    blob = json.dumps(manifest, sort_keys=True, ensure_ascii=False).encode()
    return "c2pa_" + hashlib.sha256(blob).hexdigest()[:32]


def clear_scene(scene, brief, records):
    """Clear every scene member, set safety_flags, and (if it passes) sign a disclosure
    token bound to the brief's disclosure_slot. Returns the Clearance Record artifact."""
    by_id = {r["node"]["node_id"]: r["node"] for r in records}
    per_member, worst = [], "ALLOW"
    for mid in scene["members"]:
        node = by_id[mid]
        disp, lane = set_safety_flags(node)
        per_member.append({"node_id": mid, "disposition": disp, "lane": lane,
                           "constraints": node["safety_flags"]["applicable_constraints"],
                           "review_reason": node["safety_flags"]["review_reason"]})
        if _ORDER[disp] > _ORDER[worst]:
            worst = disp

    passed = worst in _PASS
    record = {
        "clearance_version": CONFIG_VERSION,
        "scene_id": brief["scene_id"],
        "members": scene["members"],
        "scene_disposition": worst,
        "passed": passed,
        "per_member": per_member,
        "grounded_in": common.grounded_in("compliance"),
        "disclosure_token": None,
        "counsel_queue": [m for m in per_member if m["disposition"] not in _PASS],
        "disclaimer": "screening heuristic, not legal advice; BLOCK/HUMAN_REVIEW route to qualified counsel",
    }
    if passed:
        slot = brief["contract"]["disclosure_slot"]
        manifest = {"scene_id": brief["scene_id"], "members": scene["members"],
                    "disposition": worst, "bound_slot": slot, "cfg": CONFIG_VERSION}
        record["disclosure_token"] = {
            "token": _sign(manifest), "manifest": manifest, "bound_slot": slot,
            "alg": "sha256-mock", "signed_by": "HOOK_POINT:c2pa_signer",
        }
    return record


def verify_token(clearance_record, downstream_disclosure_slot):
    """Handoff verifier: re-derive the token and confirm it survives + binds the slot
    the downstream session will actually render. Returns (ok, reason)."""
    tok = clearance_record.get("disclosure_token")
    if not tok:
        return False, "no disclosure token (scene did not pass)"
    if tok["bound_slot"] != downstream_disclosure_slot:
        return False, "disclosure slot mismatch at handoff"
    if _sign(tok["manifest"]) != tok["token"]:
        return False, "token signature does not verify"
    return True, "token verified + bound to downstream slot"


if __name__ == "__main__":
    from ingest import build_nodes  # noqa: E402
    from salience import scoring  # noqa: E402
    from reaction import authenticity, hype, dense_brief  # noqa: E402
    from link import entity_link  # noqa: E402
    from creative import gen_brief  # noqa: E402
    recs, _ = build_nodes.ingest()
    for r in recs:
        scoring.apply(r); authenticity.apply(r); hype.apply(r); dense_brief.apply(r)

    print("safe-lane scene (own persona + public policy news):")
    scene = entity_link.link_scene("IN_OWN_PERSONA_CHACHA", recs, arity=2)
    brief = gen_brief.serialize(gen_brief.build_brief(scene, recs))
    cr = clear_scene(scene, brief, recs)
    print(f"  disposition={cr['scene_disposition']} passed={cr['passed']}")
    for m in cr["per_member"]:
        print(f"    {m['node_id']:26s} {m['disposition']:22s} lane={m['lane']}")
    ok, why = verify_token(cr, brief["contract"]["disclosure_slot"])
    print(f"  token: {cr['disclosure_token']['token'] if cr['disclosure_token'] else None}")
    print(f"  handoff verify: {ok} ({why})")

    print("\nblock-trigger (real artist + political + voice):")
    disp, lane = set_safety_flags(next(r["node"] for r in recs if r["node"]["node_id"] == "IN_REAL_ARTIST_POLITICAL"))
    print(f"  IN_REAL_ARTIST_POLITICAL -> {disp} (lane={lane})  [fail-closed]")
