#!/usr/bin/env python3
"""
authenticity.py — calibrated P(inauthentic | public signals) + sentiment + the signal block.

Grounded in `signal_heavy` (BUILD.md stage 4): a CALIBRATED probability (never a
boolean), from five feature families (account age, templated text, timing/burst,
verification, sentiment uniformity), mapped through a hand-weighted logit and then
PLATT-SCALED against a versioned labeled CIB benchmark so the output is an actual
probability, not a raw score. Aggregate sentiment is ORGANIC-WEIGHTED (a bot-laden
platform is down-weighted), so "what real people think" isn't dominated by a campaign.

Real here: the logit, the Platt fit (gradient descent), the organic-weighted
aggregation, the threshold-based manipulation-signal flags. HOOK POINT: a real,
larger CIB benchmark + isotonic recalibration (the v1 uses Platt on a tiny set).
Stdlib only, deterministic.
"""
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402

BENCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_fixtures", "cib_benchmark.json")

# hand-weighted logit over the five feature families (pre-Platt raw score)
LOGIT_W = {"low_age_frac": 1.5, "templated_frac": 2.5, "burst_score": 2.0,
           "one_minus_verified": 1.0, "sentiment_uniformity": 1.0}
LOGIT_B = -2.5


def _sigmoid(z):
    if z < -60:
        return 0.0
    if z > 60:
        return 1.0
    return 1.0 / (1.0 + math.exp(-z))


def _features(pf):
    return {
        "low_age_frac": pf.get("low_age_frac", 0.0),
        "templated_frac": pf.get("templated_frac", 0.0),
        "burst_score": pf.get("burst_score", 0.0),
        "one_minus_verified": 1.0 - pf.get("verified_frac", 0.0),
        "sentiment_uniformity": 1.0 - pf.get("sentiment_var", 0.5),
    }


def _raw_logit(feat):
    return LOGIT_B + sum(LOGIT_W[k] * feat[k] for k in LOGIT_W)


def fit_platt(path=BENCH, steps=600, lr=0.2):
    """Fit Platt parameters (A,B) mapping raw logit -> calibrated P via logistic
    regression on the labeled benchmark. Deterministic gradient descent."""
    bench = common.read_json(path)
    zs = [_raw_logit(_features(e["f"])) for e in bench["examples"]]
    ys = [float(e["label"]) for e in bench["examples"]]
    A, B, n = 1.0, 0.0, len(zs)
    for _ in range(steps):
        gA = gB = 0.0
        for z, y in zip(zs, ys):
            p = _sigmoid(A * z + B)
            gA += (p - y) * z
            gB += (p - y)
        A -= lr * gA / n
        B -= lr * gB / n
    return {"A": round(A, 5), "B": round(B, 5), "benchmark": bench["version"], "n": n}


_PLATT = None


def _platt():
    global _PLATT
    if _PLATT is None:
        _PLATT = fit_platt()
    return _PLATT


def p_inauthentic(platform_features):
    """Calibrated probability the reaction on one platform is inauthentic."""
    feat = _features(platform_features)
    z = _raw_logit(feat)
    pl = _platt()
    return round(_sigmoid(pl["A"] * z + pl["B"]), 4)


def _manip_flags(pf):
    flags = []
    if pf.get("burst_score", 0) > 0.6:
        flags.append("BURST_TIMING")
    if pf.get("templated_frac", 0) > 0.4:
        flags.append("TEMPLATED_TEXT")
    if pf.get("low_age_frac", 0) > 0.4:
        flags.append("LOW_ACCOUNT_AGE")
    if (1 - pf.get("sentiment_var", 0.5)) > 0.7 and pf.get("volume_tier") in ("HIGH", "VIRAL"):
        flags.append("SENTIMENT_UNIFORMITY")
    if len(pf.get("origins", [])) <= 1:
        flags.append("SINGLE_SOURCE_AMPLIFY")
    return flags


def read_signal(raw_reactions, sentiment_baseline=0.0):
    """Build the schema `signal` block from raw per-platform reaction features."""
    per_platform, agg_w, agg_num, risk_w, risk_num = [], 0.0, 0.0, 0.0, 0.0
    sentiments, all_flags = [], set()
    for pf in raw_reactions.get("per_platform", []):
        p = p_inauthentic(pf)
        organic = round(1.0 - p, 4)
        n = pf.get("n", 0)
        flags = _manip_flags(pf)
        all_flags.update(flags)
        per_platform.append({
            "platform": pf["platform"], "sentiment": pf.get("sentiment", 0.0),
            "sentiment_variance": pf.get("sentiment_var", 0.0),
            "volume_tier": pf.get("volume_tier", "LOW"),
            "dominant_themes": pf.get("dominant_themes", []),
            "sample_size": n,
        })
        sentiments.append(pf.get("sentiment", 0.0))
        # organic-weighted sentiment: a manufactured platform contributes less
        ow = n * organic
        agg_w += ow
        agg_num += ow * pf.get("sentiment", 0.0)
        # volume-weighted bot risk: a big bot-laden platform raises aggregate risk
        risk_w += n
        risk_num += n * p
    aggregate_sentiment = round(agg_num / agg_w, 4) if agg_w else round(sentiment_baseline, 4)
    bot_risk = round(risk_num / risk_w, 4) if risk_w else 0.0
    organic_score = round(1.0 - bot_risk, 4)
    # consensus_vs_division: spread of per-platform sentiment, normalized to [0,1]
    if len(sentiments) > 1:
        mean = sum(sentiments) / len(sentiments)
        var = sum((s - mean) ** 2 for s in sentiments) / len(sentiments)
        division = round(min(1.0, math.sqrt(var)), 4)
    else:
        division = 0.0
    verified = max((pf.get("verified_frac", 0.0) for pf in raw_reactions.get("per_platform", [])), default=0.0)

    return {
        "per_platform": per_platform,
        "aggregate_sentiment": aggregate_sentiment,
        "consensus_vs_division": division,
        "authenticity": {
            "organic_score": organic_score,
            "bot_astroturf_risk": bot_risk,
            "manipulation_signals": sorted(all_flags),
            "verified_share": round(verified, 4),
            "notes": (f"calibrated via {_platt()['benchmark']} (Platt A={_platt()['A']},B={_platt()['B']}); "
                      f"aggregate sentiment is organic-weighted"),
        },
    }


def apply(record):
    sig = read_signal(record["raw"]["raw_reactions"],
                      record["node"]["audience"].get("sentiment_baseline", 0.0))
    record["node"]["signal"] = sig
    record["node"]["grounding_status"]["signal_grounded"] = bool(sig["per_platform"])
    return record


if __name__ == "__main__":
    from ingest import build_nodes  # noqa: E402
    pl = _platt()
    print(f"Platt fit on {pl['benchmark']}: A={pl['A']} B={pl['B']} (n={pl['n']})")
    recs, _ = build_nodes.ingest()
    for r in recs:
        apply(r)
        a = r["node"]["signal"]["authenticity"]
        print(f"  {r['node']['node_id']:26s} botrisk={a['bot_astroturf_risk']:.3f} "
              f"organic={a['organic_score']:.3f} sent={r['node']['signal']['aggregate_sentiment']:+.3f} "
              f"flags={a['manipulation_signals']}")
