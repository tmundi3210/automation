#!/usr/bin/env python3
"""
select.py — base-model selection under license + capability constraints.

Grounded in the 'select' specialist and IDEA_NEUTRALIZED.md ("select base model per
specialist: data, restriction[license], bench, context, size, tool support" + an
objective dial quality/cost/latency). The model facts are VOLATILE frontier data
(model_sheet.json is a TEMPLATE) — this tool is the durable decision *logic*; the
numbers must be re-verified against live model cards before any real choice.

Two modes:
  default : pick a base model for ONE requirement spec.
  --map   : recommend a base model for EACH of the 12 domain specialists, using the
            per-domain requirement profiles in the sheet.

Selection = HARD filters (license class allowed, context >= min, size <= max, tool
support if required) then a SOFT score per the objective dial. Stdlib only.
"""
import argparse
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402

SHEET = os.path.join(os.path.dirname(os.path.abspath(__file__)), "model_sheet.json")


def load_sheet(path=SHEET):
    with open(path) as f:
        return json.load(f)


def filter_and_score(sheet, primary_dim, *, min_context=0, max_params=None,
                     require_tool_use=False, allowed_licenses=None, objective="balanced"):
    allowed = set(allowed_licenses or sheet["license_classes"])
    weights = sheet["objectives"][objective]
    rows = []
    for c in sheet["candidates"]:
        reasons = []
        if c["license_class"] not in allowed:
            continue
        if c["context_tokens"] < min_context:
            continue
        if require_tool_use and not c.get("tool_use"):
            continue
        if max_params is not None and c.get("params_b") is not None and c["params_b"] > max_params:
            continue
        dim = c["tiers"].get(primary_dim, 0)
        params = c.get("params_b") or 0
        score = (weights["primary_dim"] * dim
                 + weights["rel_cost"] * c["rel_cost"]
                 - weights.get("params_penalty", 0.0) * (params / 70.0) * 5)
        rows.append({"id": c["id"], "score": round(score, 3), "primary_dim_tier": dim,
                     "license_class": c["license_class"], "context_tokens": c["context_tokens"],
                     "params_b": c.get("params_b"), "rel_cost": c["rel_cost"],
                     "verify": c["verify"]})
    rows.sort(key=lambda r: -r["score"])
    return rows


def recommend_map(sheet):
    out = {}
    profiles = {k: v for k, v in sheet["domain_requirement_profiles"].items() if not k.startswith("_")}
    for dom, prof in profiles.items():
        ranked = filter_and_score(
            sheet, prof["primary_dim"], min_context=prof.get("min_context", 0),
            require_tool_use=prof.get("require_tool_use", False),
            objective=prof.get("objective", "balanced"))
        out[dom] = {"profile": prof, "pick": ranked[0] if ranked else None,
                    "runner_up": ranked[1] if len(ranked) > 1 else None}
    return out


def main():
    ap = argparse.ArgumentParser(description="Select base models under constraints.")
    ap.add_argument("--map", action="store_true", help="recommend per-domain (all 12 specialists)")
    ap.add_argument("--dim", default="reasoning", help="primary capability dimension")
    ap.add_argument("--min-context", type=int, default=0)
    ap.add_argument("--max-params", type=int, default=None)
    ap.add_argument("--require-tool-use", action="store_true")
    ap.add_argument("--licenses", nargs="*", default=None, help="allowed license classes")
    ap.add_argument("--objective", default="balanced", choices=["quality", "balanced", "cost"])
    a = ap.parse_args()
    sheet = load_sheet()

    print(f"NOTE: {sheet['_status']}\n")
    if a.map:
        rec = recommend_map(sheet)
        print(f"{'domain':8s} {'pick':28s} {'dim':>3s} {'lic':16s} {'obj':9s} runner-up")
        for dom, r in rec.items():
            p, ru = r["pick"], r["runner_up"]
            print(f"{dom:8s} {p['id']:28s} {p['primary_dim_tier']:>3d} "
                  f"{p['license_class']:16s} {r['profile']['objective']:9s} "
                  f"{(ru['id'] if ru else '-')}")
        print("\nVERIFY each pick against the live model card before committing "
              "(license terms + current eval tiers + price).")
    else:
        rows = filter_and_score(sheet, a.dim, min_context=a.min_context, max_params=a.max_params,
                                require_tool_use=a.require_tool_use, allowed_licenses=a.licenses,
                                objective=a.objective)
        print(f"dim={a.dim} min_context={a.min_context} max_params={a.max_params} "
              f"tool_use={a.require_tool_use} objective={a.objective}")
        if not rows:
            print("  no candidate satisfies the hard constraints (loosen them or extend the sheet).")
        for r in rows:
            print(f"  {r['score']:>6.2f}  {r['id']:28s} dim={r['primary_dim_tier']} "
                  f"ctx={r['context_tokens']:>6d} params={r['params_b']} "
                  f"cost={r['rel_cost']} [{r['license_class']}]")
        if rows:
            print(f"\n  pick: {rows[0]['id']}  — verify: {rows[0]['verify']}")


if __name__ == "__main__":
    main()
