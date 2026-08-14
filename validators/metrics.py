#!/usr/bin/env python3
"""
metrics.py — fast size + structure metrics for a KB JSON artifact.

Used by the calibration harness to record the QUANTITY dimension: how large the
output is (chars + estimated tokens) and how many of each object type it carries.
Token estimate is heuristic (intended for RELATIVE comparison across calibration
runs, not billing). Stdlib only.

Usage:
  python3 metrics.py KB.json [--label L] [--report out.json]
"""
import argparse
import json
import re
import sys

COUNT_KEYS = ["nodes", "edges", "conflict_axes", "edge_cases", "workflow",
              "competency_questions", "dominance_rules", "anti_rework_rules",
              "iteration_protocol", "source_registry", "glossary",
              "priority_order", "traceability_matrix"]


def estimate_tokens(text):
    # blended heuristic: avg of char/4 and word*1.3, which tracks GPT/Claude
    # tokenization reasonably for dense JSON. Relative comparison only.
    chars = len(text)
    words = len(re.findall(r"\S+", text))
    return round((chars / 4.0 + words * 1.3) / 2.0)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("kb")
    ap.add_argument("--label", default="")
    ap.add_argument("--report")
    args = ap.parse_args()

    raw = open(args.kb, encoding="utf-8").read()
    out = {
        "_directive": "machine-facing metrics record; model-parse optimized",
        "label": args.label,
        "path": args.kb,
        "bytes": len(raw.encode("utf-8")),
        "chars": len(raw),
        "est_tokens": estimate_tokens(raw),
        "counts": {},
        "json_valid": True,
    }
    try:
        kb = json.loads(raw)
        for k in COUNT_KEYS:
            v = kb.get(k)
            out["counts"][k] = len(v) if isinstance(v, list) else None
    except json.JSONDecodeError as e:
        out["json_valid"] = False
        out["json_error"] = str(e)

    text = json.dumps(out, indent=2)
    if args.report:
        open(args.report, "w").write(text)
    print(text)
    return 0


if __name__ == "__main__":
    sys.exit(main())
