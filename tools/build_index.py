#!/usr/bin/env python3
"""Build an INDEX.json aggregating a KB set's metrics/validation sidecars.
Defaults reproduce the Phase A knowledge_searcher index exactly; pass args for other sets."""
import json, glob, os, argparse

DIRECTIVE = ("losslessly compressed, token-efficient, information-dense, fully detailed, "
             "machine-facing; optimized for model parsing over human readability")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--set", default="knowledge_searcher")
    ap.add_argument("--kb-dir", default="knowledge_base/knowledge_searcher")
    ap.add_argument("--purpose", default="foundational KB set: where/how to find, retrieve, "
                    "evaluate, and synthesize authoritative knowledge; consulted by every "
                    "Phase B idea->specialist run")
    ap.add_argument("--out", default=None)
    a = ap.parse_args()
    out = a.out or os.path.join(a.kb_dir, "INDEX.json")

    items = []
    tot = {"nodes": 0, "edges": 0, "conflict_axes": 0, "edge_cases": 0,
           "workflow": 0, "competency_questions": 0, "est_tokens": 0}
    for kb in sorted(glob.glob(os.path.join(a.kb_dir, "*.kb.json"))):
        base = kb[:-len(".kb.json")]; unit = os.path.basename(base)
        m = json.load(open(base + ".metrics.json"))
        v = json.load(open(base + ".validation.json"))
        c = m["counts"]
        dom, _, sub = unit.partition("__")
        items.append({"unit": unit, "domain_code": dom, "subdomain": sub, "kb_path": kb,
                      "validator": v["overall_status"], "est_tokens": m["est_tokens"],
                      "nodes": c.get("nodes"), "edges": c.get("edges"),
                      "conflict_axes": c.get("conflict_axes"), "edge_cases": c.get("edge_cases"),
                      "workflow": c.get("workflow"), "competency_questions": c.get("competency_questions")})
        for k in tot:
            tot[k] += (m["est_tokens"] if k == "est_tokens" else c.get(k, 0)) or 0

    index = {"_directive": DIRECTIVE, "set": a.set, "purpose": a.purpose,
             "grain": "subdomain (dense)", "engine": "schema/kb_generator_v1.4.1.txt",
             "gate": "validators/kb_validator.py --mode dense (35 checks)",
             "kb_count": len(items), "all_pass": all(i["validator"] == "pass" for i in items),
             "totals": tot, "domains": sorted(set(i["domain_code"] for i in items)), "kbs": items}
    open(out, "w").write(json.dumps(index, indent=2))
    print("set:", a.set, "| KBs:", len(items), "| all_pass:", index["all_pass"])
    print("TOTAL nodes:", tot["nodes"], "edges:", tot["edges"], "CQs:", tot["competency_questions"],
          "| est_tokens:", tot["est_tokens"], "(~", round(tot["est_tokens"] / 1000), "k)")
    print("domains:", index["domains"], "| out:", out)


if __name__ == "__main__":
    main()
