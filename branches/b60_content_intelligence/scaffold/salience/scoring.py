#!/usr/bin/env python3
"""
scoring.py — public-signal salience score + opportunity composite + hype tier.

Grounded in `salience_heavy` (BUILD.md stage 2):
  * salience.score = weighted GEOMETRIC mean of four log-scaled public signals
    (Trends, follower count, news/GDELT volume, Wikipedia pageviews), dual raw/
    normalized track, every score provenance-stamped.
  * opportunity = under_served x reachable x addressable_value  (the "medium-tier is
    the opportunity" claim is *computed per node*, never assumed).
  * a provisional hype tier; the signal stage later applies the authenticity discount.

CONFIG is FROZEN and committed (the reproducibility requirement): signal weights, the
log caps, the single-channel floor, and the tier bands. Change it only with a version
bump. Stdlib only, deterministic.
"""
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402

CONFIG = {
    "version": "salience-cfg-v1",
    "weights": {"trends": 0.30, "followers": 0.30, "news_volume": 0.25, "wiki_pageviews": 0.15},
    "caps": {"trends": 100, "followers": 5_000_000, "news_volume": 100, "wiki_pageviews": 1000},
    "channel_floor": 0.05,   # a single absent channel must not zero the geometric mean
    "audience_cap": 5_000_000,
    "tier_bands": [           # (max_raw_hype, tier) evaluated low->high; saturation>=sat_cap overrides
        [0.20, "UNDERGROUND"], [0.40, "RISING"], [0.70, "MEDIUM_OPPORTUNITY"],
        [0.85, "PEAKING"], [1.01, "OVER_SATURATED"],
    ],
    "oversaturated_saturation": 0.80,
}


def _scale(raw, cap, floor):
    return max(floor, min(1.0, math.log1p(max(0.0, raw)) / math.log1p(cap)))


def _wgeomean(scaled, weights):
    tot = sum(weights.values())
    acc = sum(weights[k] * math.log(scaled[k]) for k in weights)
    return math.exp(acc / tot)


def _tier(raw_hype, saturation):
    if saturation >= CONFIG["oversaturated_saturation"]:
        return "OVER_SATURATED"
    for hi, name in CONFIG["tier_bands"]:
        if raw_hype < hi:
            return name
    return "OVER_SATURATED"


def score(salience_signals, market, audience):
    """Return a salience+opportunity reading. Inputs are the raw sidecar from ingest."""
    caps, floor, w = CONFIG["caps"], CONFIG["channel_floor"], CONFIG["weights"]
    scaled = {k: _scale(salience_signals.get(k, 0), caps[k], floor) for k in w}
    raw_hype = round(_wgeomean(scaled, w), 4)

    saturation = float(market.get("saturation", 0.5))
    value = float(market.get("addressable_value", 0.5))
    under_served = round(1.0 - saturation, 4)
    reach_raw = audience.get("size_estimate", 0)
    reachable = round(_scale(reach_raw, CONFIG["audience_cap"], 0.0), 4)
    opportunity = round(under_served * reachable * value, 4)
    tier = _tier(raw_hype, saturation)

    return {
        "salience_score": raw_hype,          # normalized salience (== provisional raw_hype)
        "raw_hype": raw_hype,
        "opportunity_score": opportunity,
        "opportunity_factors": {"under_served": under_served, "reachable": reachable,
                                "addressable_value": value},
        "tier": tier,
        "scaled_signals": {k: round(v, 4) for k, v in scaled.items()},
        "config_version": CONFIG["version"],
        "grounded_in": common.grounded_in("salience"),
    }


def apply(record):
    """Attach the salience reading to a pipeline record's hype block (provisional;
    authenticity_adjusted_hype is left for the signal stage). Mutates and returns node."""
    s = score(record["raw"]["salience_signals"], record["raw"]["market"], record["node"]["audience"])
    record["node"]["hype"] = {
        "raw_hype": s["raw_hype"],
        "authenticity_adjusted_hype": s["raw_hype"],  # placeholder until signal discount
        "tier": s["tier"],
        "trajectory": "EMERGING",
        "opportunity_score": s["opportunity_score"],
        "evidence": [f"salience={s['salience_score']} via {s['config_version']}; "
                     f"opp=under_served*reachable*value="
                     f"{s['opportunity_factors']['under_served']}*{s['opportunity_factors']['reachable']}*"
                     f"{s['opportunity_factors']['addressable_value']}"],
    }
    record.setdefault("meta", {})["salience"] = s
    return record


if __name__ == "__main__":
    sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    from ingest import build_nodes  # noqa: E402
    recs, _ = build_nodes.ingest()
    print(f"salience [{CONFIG['version']}] — medium-tier should outrank over-saturated:")
    rows = [apply(r)["meta"]["salience"] | {"id": r["node"]["node_id"]} for r in recs]
    for s in sorted(rows, key=lambda x: -x["opportunity_score"]):
        print(f"  {s['id']:26s} hype={s['raw_hype']:.3f} tier={s['tier']:18s} "
              f"opp={s['opportunity_score']:.3f}  scaled={s['scaled_signals']}")
