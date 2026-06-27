#!/usr/bin/env python3
"""
hype.py — genuine-hype gate: N-independent-origins + common-method discount -> adjusted hype.

Grounded in `signal_heavy` (BUILD.md stage 4): "Genuine-hype = provenance-dedup to
distinct origins, common-method discount, >=N-independent-origins gate INDEPENDENT of
volume." A campaign that floods one platform with a single hashtag is loud but NOT
genuine hype; this stage strips that inflation and recomputes the authenticity-adjusted
hype + final tier the MAP stage actually ranks on.

Runs after authenticity (needs bot_astroturf_risk). Stdlib only, deterministic.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402
from salience import scoring as _sal  # noqa: E402

N_INDEPENDENT_MIN = 2


def _independent_origins(raw_reactions):
    """Distinct origins, EXCLUDING platforms that are single-source amplification
    (common-method): those add volume, not independent corroboration."""
    independent, single_source_volume, total_volume = set(), 0, 0
    for pf in raw_reactions.get("per_platform", []):
        origins = pf.get("origins", [])
        n = pf.get("n", 0)
        total_volume += n
        if len(origins) <= 1:
            single_source_volume += n
            continue
        independent.update(origins)
    return independent, single_source_volume, total_volume


def assess(record):
    node = record["node"]
    raw_hype = node["hype"]["raw_hype"]
    bot_risk = node["signal"]["authenticity"]["bot_astroturf_risk"]
    saturation = record["raw"]["market"].get("saturation", 0.5)

    origins, single_vol, total_vol = _independent_origins(record["raw"]["raw_reactions"])
    n_independent = len(origins)
    genuine_hype_ok = n_independent >= N_INDEPENDENT_MIN
    common_method_discount = round(single_vol / total_vol, 4) if total_vol else 0.0

    adjusted = round(raw_hype * (1.0 - bot_risk), 4)
    tier = _sal._tier(adjusted, saturation)
    return {
        "authenticity_adjusted_hype": adjusted,
        "tier": tier,
        "genuine_hype_ok": genuine_hype_ok,
        "independent_origins": sorted(origins),
        "n_independent_origins": n_independent,
        "common_method_discount": common_method_discount,
    }


def apply(record):
    a = assess(record)
    h = record["node"]["hype"]
    h["authenticity_adjusted_hype"] = a["authenticity_adjusted_hype"]
    h["tier"] = a["tier"]
    h["evidence"].append(
        f"genuine_hype_ok={a['genuine_hype_ok']} (N={a['n_independent_origins']} independent "
        f"origins>={N_INDEPENDENT_MIN}); common_method_discount={a['common_method_discount']}; "
        f"adjusted=raw*(1-botrisk)")
    record["node"]["grounding_status"]["hype_grounded"] = True
    record.setdefault("meta", {})["hype"] = a
    return record


if __name__ == "__main__":
    from ingest import build_nodes  # noqa: E402
    from reaction import authenticity  # noqa: E402
    recs, _ = build_nodes.ingest()
    for r in recs:
        _sal.apply(r)
        authenticity.apply(r)
        apply(r)
        h, a = r["node"]["hype"], r["meta"]["hype"]
        print(f"  {r['node']['node_id']:26s} raw={h['raw_hype']:.3f} -> adj={h['authenticity_adjusted_hype']:.3f} "
              f"tier={h['tier']:18s} genuine={a['genuine_hype_ok']} Nindep={a['n_independent_origins']} "
              f"cm_discount={a['common_method_discount']}")
