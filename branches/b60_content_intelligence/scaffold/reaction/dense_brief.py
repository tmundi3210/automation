#!/usr/bin/env python3
"""
dense_brief.py — the decision-lossless pipe/KV machine_summary + round-trip audit.

Grounded in `signal_heavy` (BUILD.md stage 4): "The dense brief is one pipe/KV line
with a schema_hint paid once and v:obs|v:derived|v:proxied tags — decision-lossless
over a fixed field set, validated by a round-trip + coverage audit."

"Lossless" here means DECISION-lossless: a downstream agent can recover every field
that changes a GENERATE decision from `dense` + `schema_hint` alone. The audit parses
the line back and asserts every REQUIRED_SLOT is present and non-empty; if not, the
producer fails closed rather than shipping a lossy brief. Stdlib only, deterministic.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402

# the fixed decision-relevant field set (the lossless contract)
REQUIRED_SLOTS = ["id", "type", "name", "realperson", "place", "knownfor", "interest",
                  "aud", "sent", "auth", "hype", "tier", "opp", "crit", "safety", "tags", "geo"]

SCHEMA_HINT = ("slots pipe-delimited k=v; v:obs=observed,v:derived=computed,v:proxied=estimated. "
               "sent=[-1,1] organic-weighted(derived); auth=org<organic[0,1]>/bot<botrisk[0,1]>(derived); "
               "hype=authenticity_adjusted[0,1](derived); tier in {UNDERGROUND,RISING,MEDIUM_OPPORTUNITY,"
               "PEAKING,OVER_SATURATED,DECLINING}; opp=opportunity[0,1](derived); realperson=bool; "
               "safety=intrinsic sensitivity (realperson/political/minor/flags); tags=type:value;... ; geo=keys")


def _san(s):
    return str(s).replace("|", "/").replace("=", "-").strip()


def _top_platforms(audience):
    mix = audience.get("platform_mix", {})
    return ",".join(k for k, _ in sorted(mix.items(), key=lambda kv: -kv[1])[:2]) or "?"


def _safety_slot(node):
    b = node["behind"]
    flags = list(node["critical_read"].get("sensitivity_flags", []))
    if b.get("is_real_identifiable_person"):
        flags.append("REAL_PERSON")
    if b.get("public_figure_status") == "MINOR_OR_PROTECTED":
        flags.append("MINOR_PROTECTED")
    return ",".join(flags) or "none"


def build_dense(node):
    """Build the dense slots dict, then serialize to the pipe/KV line."""
    sig = node["signal"]
    auth = sig["authenticity"]
    hype = node["hype"]
    place = node["place"]
    slots = {
        "id": node["node_id"],
        "type": node["item_type"],
        "name": _san(node["item"]["canonical_name"])[:48],
        "realperson": str(node["behind"]["is_real_identifiable_person"]).lower(),
        "place": _san(place["primary_region"]) + ">" + "/".join(place.get("region_hierarchy", [])),
        "knownfor": _san(node["known_for"]["primary"])[:40],
        "interest": node["known_for"].get("interest_domain", "OTHER") + "/" +
                    (node["known_for"].get("interest_subgenre", ["_"]) or ["_"])[0],
        "aud": _san(node["audience"]["core_demo"])[:30] + "@" + _top_platforms(node["audience"]),
        "sent": f"{sig['aggregate_sentiment']:+.2f}",
        "auth": f"org{auth['organic_score']:.2f}/bot{auth['bot_astroturf_risk']:.2f}",
        "hype": f"{hype['authenticity_adjusted_hype']:.2f}",
        "tier": hype["tier"],
        "opp": f"{hype['opportunity_score']:.2f}",
        "crit": _san(node["critical_read"]["summary"])[:60],
        "safety": _safety_slot(node),
        "tags": ";".join(f"{t['type']}:{t['value']}" for t in node["links"]["typed_tags"][:6]),
        "geo": ",".join(node["links"]["geo_keys"]),
    }
    line = "|".join(f"{k}={slots[k]}" for k in REQUIRED_SLOTS)
    return line, slots


def parse_dense(line):
    out = {}
    for slot in line.split("|"):
        if "=" in slot:
            k, v = slot.split("=", 1)
            out[k] = v
    return out


def round_trip_audit(line):
    """Parse the dense line back and assert coverage of every REQUIRED_SLOT.
    Returns (ok, missing_or_empty)."""
    parsed = parse_dense(line)
    missing = [k for k in REQUIRED_SLOTS if not parsed.get(k)]
    return (len(missing) == 0), missing


def apply(record):
    node = record["node"]
    line, _ = build_dense(node)
    ok, missing = round_trip_audit(line)
    if not ok:
        raise ValueError(f"dense brief not decision-lossless; missing slots {missing}")
    node["machine_summary"] = {
        "dense": line,
        "schema_hint": SCHEMA_HINT,
        "token_estimate": common.est_tokens(line),
        "lossless_assertion": True,
    }
    return record


if __name__ == "__main__":
    from ingest import build_nodes  # noqa: E402
    from salience import scoring  # noqa: E402
    from reaction import authenticity, hype  # noqa: E402
    recs, _ = build_nodes.ingest()
    for r in recs:
        scoring.apply(r)
        authenticity.apply(r)
        hype.apply(r)
        apply(r)
        ms = r["node"]["machine_summary"]
        ok, missing = round_trip_audit(ms["dense"])
        print(f"\n{r['node']['node_id']}  (~{ms['token_estimate']} tok, lossless={ok})")
        print("  " + ms["dense"])
