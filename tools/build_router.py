#!/usr/bin/env python3
"""Build a ROUTER.json over a set of specialist files (route_when = distinctive vocab).
Defaults reproduce the Phase A knowledge_searcher router; pass args for other sets."""
import json, glob, os, re, argparse

DIRECTIVE = ("losslessly compressed, token-efficient, information-dense, fully detailed, "
             "machine-facing; optimized for model parsing over human readability")
STOP = {'of', 'the', 'and', 'for', 'in', 'to', 'a', 'with', 'by', 'on', 'from', 'via', 'using',
        'based', 'how', 'what', 'when', 'which', 'that', 'this', 'are', 'is', 'an', 'or', 'as',
        'at', 'its', 'it', 'into', 'data', 'analysis', 'method', 'methods', 'model', 'models',
        'knowledge', 'research', 'specialist', 'across', 'given', 'need', 'use', 'should', 'can',
        'will', 'between', 'within', 'their', 'than', 'more'}


def kw(s):
    return [w for w in re.split(r'[^a-z0-9]+', (s or '').lower()) if w and w not in STOP and len(w) > 3]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--set", default="knowledge_searcher")
    ap.add_argument("--spec-dir", default="specialists")
    ap.add_argument("--router-id", default="knowledge_searcher_router")
    ap.add_argument("--purpose", default="route any knowledge-finding/evaluation/synthesis need "
                    "to the appropriate Knowledge Searcher domain specialist(s); foundational "
                    "entrypoint consulted by every Phase B idea->specialist run")
    ap.add_argument("--multi", default="compose specialists when a need spans domains (e.g. find + "
                    "appraise evidence => libarch/ir to find + evsynth to appraise + scholcomm for "
                    "source trust); on conflict, the stricter escalation_trigger wins.")
    ap.add_argument("--fallback", default="general source-finding -> libarch (reference_and_discovery) "
                    "+ ir (search); methodology/validity -> method; source trust/provenance -> scholcomm; "
                    "if ambiguous, run libarch then route follow-ups.")
    ap.add_argument("--out", default=None)
    a = ap.parse_args()
    out = a.out or os.path.join(a.spec_dir, "ROUTER.json")

    specs = []
    for p in sorted(glob.glob(os.path.join(a.spec_dir, "*.specialist.json"))):
        s = json.load(open(p))
        caps = [c.get("capability") or c.get("method", "") for c in s.get("capabilities", [])]
        gloss = [g.get("term", "") for g in s.get("glossary", [])]
        cqs = s.get("competency_questions_covered", [])
        sig = []
        for src in [s.get("domain_label", "")] + caps + gloss + cqs:
            sig += kw(src)
        seen = set(); route_when = []
        for w in sig:
            if w not in seen:
                seen.add(w); route_when.append(w)
        specs.append({"specialist_id": s.get("specialist_id"), "spec_path": p,
                      "domain_label": s.get("domain_label"), "purpose": s.get("purpose"),
                      "route_when": route_when, "capabilities": caps, "example_questions": cqs[:4]})

    router = {"_directive": DIRECTIVE, "router_id": a.router_id, "set": a.set, "purpose": a.purpose,
              "specialist_count": len(specs), "specialists": specs,
              "selection_procedure": [
                  "1. Normalize the knowledge need into (intent, domain_signal_terms).",
                  "2. Score each specialist = count of need terms in its route_when set, with domain_label term matches weighted x2.",
                  "3. Select the top-scoring specialist; if a second is within 1 point, invoke both and merge.",
                  "4. Apply the selected specialist's decision_procedure + workflow to the need.",
                  "5. If no specialist scores >0, fall back per fallback_policy."],
              "multi_specialist_policy": a.multi, "fallback_policy": a.fallback,
              "escalation": "if any selected specialist raises a human_review escalation_trigger, surface it before acting."}
    open(out, "w").write(json.dumps(router, indent=2))
    print("set:", a.set, "| specialists:", len(specs),
          "| avg signals:", round(sum(len(s['route_when']) for s in specs) / max(1, len(specs))), "| out:", out)


if __name__ == "__main__":
    main()
