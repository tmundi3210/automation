#!/usr/bin/env python3
"""
resonance.py — 4-driver ethical-resonance check + safe-lanes whitelist + honest sentiment.

Grounded in `psych` (BUILD.md stage 7), an ENABLER: a 4-driver scene check (identity-fit,
belonging, emotion, surprise) that requires self-relevance + one positive high-arousal
emotion before publish, leads with shared in-group experience (not out-group contrast),
and gates eligible subjects through a SAFE-LANES WHITELIST *before* framing. The sentiment
read is honest: weighted by reach + saves/shares, not raw comment volume (correcting the
vocal-minority / outrage bias).

This raises content quality and keeps it pro-social; it never relaxes a compliance block.
Stdlib only, deterministic.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402

SAFE_LANES = {"own_persona", "public_topic", "consented", "parody_with_cues", "shared_experience"}
IDENTITY_FIT_MIN = 0.5
# volume-tier -> coarse reach multiplier (a proxy for saves/shares reach, not comment count)
_REACH = {"NONE": 0.0, "LOW": 0.3, "MED": 0.6, "HIGH": 0.85, "VIRAL": 1.0}


def _drivers(anchor, partner):
    aud = anchor["audience"]
    base = aud.get("sentiment_baseline", 0.0)
    identity_fit = round(min(1.0, 0.5 + 0.5 * max(0.0, base)), 4)         # in-group disposition
    belonging = 0.8 if anchor["place"].get("diaspora_relevance") else 0.4  # shared in-group experience
    # positive high-arousal emotion present if the persona's baseline is warm + comedic/pride hook
    positive_high_arousal = base > 0.2 and anchor["known_for"].get("interest_domain") in (
        "COMEDY", "MUSIC", "SPORT", "FILM", "INTERNET_CULTURE")
    emotion = 0.75 if positive_high_arousal else 0.25
    surprise = 0.7  # benign-violation contrast axis supplies novelty
    return {
        "identity_fit": identity_fit, "belonging": round(belonging, 4),
        "emotion": round(emotion, 4), "surprise": round(surprise, 4),
        "positive_high_arousal": positive_high_arousal,
        "self_relevance": identity_fit >= IDENTITY_FIT_MIN,
    }


def honest_sentiment(node):
    """Reach/saves-weighted sentiment (organic_score x reach-tier), correcting the
    vocal-minority bias of raw comment-volume weighting."""
    num = den = 0.0
    org = node["signal"]["authenticity"]["organic_score"]
    for pf in node["signal"]["per_platform"]:
        w = _REACH.get(pf.get("volume_tier", "LOW"), 0.3) * org
        num += w * pf["sentiment"]
        den += w
    return round(num / den, 4) if den else round(node["signal"]["aggregate_sentiment"], 4)


def check(scene, clearance, records):
    """Run the resonance + safe-lane gate over a cleared scene. Returns a resonance_check
    artifact with publish_ok (this gate's vote) and any blockers."""
    by_id = {r["node"]["node_id"]: r["node"] for r in records}
    anchor = by_id[scene["members"][0]]
    partner = by_id[scene["members"][1]] if len(scene["members"]) > 1 else anchor

    lanes = {m["lane"] for m in clearance["per_member"]}
    safe_lane_ok = lanes.issubset(SAFE_LANES)
    drivers = _drivers(anchor, partner)

    blockers = []
    if not safe_lane_ok:
        blockers.append(f"subject lane(s) not in safe-lanes whitelist: {sorted(lanes - SAFE_LANES)}")
    if not drivers["self_relevance"]:
        blockers.append("identity_fit below self-relevance threshold")
    if not drivers["positive_high_arousal"]:
        blockers.append("no positive high-arousal emotion (would lead with outrage)")

    return {
        "drivers": drivers,
        "safe_lane_ok": safe_lane_ok,
        "lead_with": "shared in-group experience (belonging), not out-group contrast",
        "honest_sentiment": honest_sentiment(anchor),
        "raw_aggregate_sentiment": anchor["signal"]["aggregate_sentiment"],
        "publish_ok": len(blockers) == 0,
        "blockers": blockers,
        "grounded_in": "psych",
    }


if __name__ == "__main__":
    from ingest import build_nodes  # noqa: E402
    from salience import scoring  # noqa: E402
    from reaction import authenticity, hype, dense_brief  # noqa: E402
    from link import entity_link  # noqa: E402
    from creative import gen_brief  # noqa: E402
    from compliance import clearance  # noqa: E402
    recs, _ = build_nodes.ingest()
    for r in recs:
        scoring.apply(r); authenticity.apply(r); hype.apply(r); dense_brief.apply(r)
    scene = entity_link.link_scene("IN_OWN_PERSONA_CHACHA", recs, arity=2)
    brief = gen_brief.serialize(gen_brief.build_brief(scene, recs))
    cr = clearance.clear_scene(scene, brief, recs)
    res = check(scene, cr, recs)
    print(f"publish_ok={res['publish_ok']} safe_lane_ok={res['safe_lane_ok']}")
    print(f"  drivers: {res['drivers']}")
    print(f"  honest_sentiment={res['honest_sentiment']} (raw aggregate={res['raw_aggregate_sentiment']})")
    print(f"  blockers: {res['blockers'] or 'none'}")
