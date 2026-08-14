#!/usr/bin/env python3
"""
compute_kb_formulas.py — deterministic derived-field computer for KB JSON.

Mirrors EXACTLY the formulas enforced by validators/kb_validator.py:check_formulas,
so a KB whose BASE metrics are authored by hand gets all DERIVED fields filled
consistently (validator tolerance 0.02). Helpers author semantic content + base
metric estimates only; this tool computes the rest. Stdlib only.

Base inputs read (must be authored):
  node.metrics: criticality, business_value, user_value, technical_complexity,
                risk_if_wrong, cross_topic_coupling, irreversibility, confidence,
                node_conflict_pressure
  node.score_derivation_inputs: acceptance_test_pass_rate, dependency_gate_pass_rate
                (uncertainty_range_width + revisit_trigger_count_normalized are DERIVED here)
  node.probabilistic_layer: evidence_confidence, data_quality, failure_rate,
                downside_weight, prior_importance, observed_impact, uncertainty_interval
  edge: relation_strength, signed_tension, expected_rework_cost, conflict_probability,
                causal_confidence

Derived written (rounded 2dp):
  node.metrics: final_importance, risk_score, confidence_score, revisit_pressure, lock_score
  node.score_derivation_inputs: uncertainty_range_width (=hi-lo from uncertainty_interval),
                revisit_trigger_count_normalized (=min(1,len(revisit_triggers)/5))
  node.probabilistic_layer: posterior_importance (=normalize(PI*EC), or observed form)
  edge: edge_priority, edge_score, edge_heat

Usage: python3 compute_kb_formulas.py KB.json [--inplace] [--allow-observed]
       (without --inplace, writes KB.json and prints a one-line summary anyway)
"""
import argparse, json, sys


def normalize(x):
    return min(1.0, max(0.0, x))


def num(d, k, default=0.0):
    v = d.get(k) if isinstance(d, dict) else None
    return float(v) if isinstance(v, (int, float)) else default


def compute(kb, allow_observed=False):
    changed = 0
    for n in kb.get("nodes", []):
        m = n.setdefault("metrics", {})
        sdi = n.setdefault("score_derivation_inputs", {})
        pl = n.setdefault("probabilistic_layer", {})
        C = num(m, "criticality"); BV = num(m, "business_value"); UV = num(m, "user_value")
        TC = num(m, "technical_complexity"); R = num(m, "risk_if_wrong")
        X = num(m, "cross_topic_coupling"); IR = num(m, "irreversibility")
        CF = num(m, "confidence"); NCP = num(m, "node_conflict_pressure")
        AT = num(sdi, "acceptance_test_pass_rate"); DG = num(sdi, "dependency_gate_pass_rate")
        EC = num(pl, "evidence_confidence"); DQ = num(pl, "data_quality")
        FR = num(pl, "failure_rate"); DW = num(pl, "downside_weight")
        PI = num(pl, "prior_importance"); OI = num(pl, "observed_impact")

        # uncertainty_range_width from interval; revisit_trigger_count_normalized from triggers
        ui = pl.get("uncertainty_interval")
        if isinstance(ui, list) and len(ui) == 2 and all(isinstance(x, (int, float)) for x in ui):
            sdi["uncertainty_range_width"] = round(ui[1] - ui[0], 2)
        UR = num(sdi, "uncertainty_range_width")
        rtl = n.get("revisit_triggers")
        if isinstance(rtl, list):
            sdi["revisit_trigger_count_normalized"] = round(min(1.0, len(rtl) / 5.0), 2)
        RT = num(sdi, "revisit_trigger_count_normalized")

        derived = {
            "final_importance": normalize(0.18*C+0.12*BV+0.12*UV+0.14*TC+0.16*R+0.12*X+0.10*IR+0.06*CF),
            "risk_score": normalize(0.32*R+0.24*IR+0.18*X+0.16*TC+0.10*(1-CF)),
            "confidence_score": normalize(0.45*EC+0.25*DQ+0.15*AT+0.15*DG),
            "revisit_pressure": normalize(0.30*(1-CF)+0.25*UR+0.20*RT+0.15*NCP+0.10*FR),
            "lock_score": normalize(0.30*AT+0.25*DG+0.20*CF+0.15*(1-R)+0.10*(1-IR)),
        }
        for k, v in derived.items():
            nv = round(v, 2)
            if m.get(k) != nv:
                changed += 1
            m[k] = nv
        post = normalize(PI*EC + OI*DQ - FR*DW) if allow_observed else normalize(PI*EC)
        pv = round(post, 2)
        if pl.get("posterior_importance") != pv:
            changed += 1
        pl["posterior_importance"] = pv

    for e in kb.get("edges", []):
        RS = num(e, "relation_strength"); ST = num(e, "signed_tension")
        ERC = num(e, "expected_rework_cost"); CP = num(e, "conflict_probability")
        CC = num(e, "causal_confidence")
        ep = normalize(RS*(0.40*abs(ST)+0.30*ERC+0.20*CP+0.10*CC))
        vals = {"edge_priority": round(ep, 2), "edge_score": round(ep, 2),
                "edge_heat": round(normalize(0.50*ep+0.20*RS+0.15*CP+0.15*ERC), 2)}
        for k, v in vals.items():
            if e.get(k) != v:
                changed += 1
            e[k] = v
    return changed


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("kb")
    ap.add_argument("--inplace", action="store_true")
    ap.add_argument("--allow-observed", action="store_true")
    a = ap.parse_args()
    try:
        kb = json.load(open(a.kb, encoding="utf-8"))
    except Exception as e:
        print(f"cannot load {a.kb}: {e}", file=sys.stderr); return 2
    changed = compute(kb, a.allow_observed)
    open(a.kb, "w", encoding="utf-8").write(json.dumps(kb, indent=2, ensure_ascii=False))
    print(json.dumps({"kb": a.kb, "nodes": len(kb.get("nodes", [])),
                      "edges": len(kb.get("edges", [])), "fields_updated": changed}))
    return 0


if __name__ == "__main__":
    sys.exit(main())
