#!/usr/bin/env python3
"""
specialist_validator.py — deterministic gate for A.5 specialist artifacts.

A specialist is a machine-facing operating spec distilled from a domain's KB set.
This checks structural completeness + grounding so the orchestrator can accept/reject
a helper's specialist output (quality gate, mirroring kb_validator for KBs).

Usage:
  python3 specialist_validator.py SPEC.json [--report out.json] [--quiet]
Exit 0 = pass, 1 = fail, 2 = bad invocation. Stdlib only.
"""
import argparse, json, os, sys

REQUIRED = [
    "_directive", "specialist_id", "domain", "domain_label", "purpose",
    "grounded_in_kbs", "role", "capabilities", "decision_procedure", "workflow",
    "escalation_triggers", "validation_checklist", "conflicts_and_dominance",
    "glossary", "competency_questions_covered",
]
NONEMPTY_LISTS = [
    "grounded_in_kbs", "capabilities", "decision_procedure", "workflow",
    "escalation_triggers", "validation_checklist", "competency_questions_covered",
]
PLACEHOLDERS = {"precise_pro", "concrete_example", "NODE_A", "SHORT_ID", "<...>",
                "system-prompt-style", "FAMILY_ID", "method_catalog", "TODO", "placeholder"}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("spec")
    ap.add_argument("--report")
    ap.add_argument("--quiet", action="store_true")
    a = ap.parse_args()
    checks = []
    def ok(c, d=""): checks.append((c, "pass", d))
    def fail(c, d=""): checks.append((c, "fail", d))

    try:
        raw = open(a.spec, encoding="utf-8").read()
    except OSError as e:
        print(f"cannot read {a.spec}: {e}", file=sys.stderr); return 2
    try:
        s = json.loads(raw); ok("json.valid")
    except json.JSONDecodeError as e:
        out = {"overall_status": "fail", "checks": [{"check_id": "json.valid", "status": "fail", "detail": str(e)}]}
        if a.report: open(a.report, "w").write(json.dumps(out, indent=2))
        if not a.quiet: print(json.dumps(out, indent=2))
        return 1

    missing = [k for k in REQUIRED if k not in s]
    (ok if not missing else fail)("required_keys", f"missing={missing}")

    d = s.get("_directive", "")
    (ok if isinstance(d, str) and "machine" in d.lower() else fail)("directive_present", d[:40])

    for k in NONEMPTY_LISTS:
        v = s.get(k)
        (ok if isinstance(v, list) and len(v) > 0 else fail)(f"nonempty.{k}", f"len={len(v) if isinstance(v,list) else 'NA'}")

    # grounding: referenced KB files must exist
    bad = [p for p in s.get("grounded_in_kbs", []) if not (isinstance(p, str) and os.path.exists(p))]
    (ok if not bad else fail)("grounded_kbs_exist", f"missing={bad}")

    # capabilities shape
    caps = s.get("capabilities", [])
    badcap = [i for i, c in enumerate(caps) if not (isinstance(c, dict) and (c.get("capability") or c.get("method")) and c.get("when_to_use"))]
    (ok if isinstance(caps, list) and caps and not badcap else fail)("capabilities_shape", f"bad_idx={badcap[:10]}")

    hits = sorted({p for p in PLACEHOLDERS if p in raw})
    (ok if not hits else fail)("no_placeholders", f"found={hits}")

    failed = [c for c in checks if c[1] == "fail"]
    out = {
        "_directive": "machine-facing specialist validation record; model-parse optimized",
        "validator": "specialist_validator/1.0",
        "overall_status": "fail" if failed else "pass",
        "summary": {"passed": sum(1 for c in checks if c[1] == "pass"), "failed": len(failed)},
        "checks": [{"check_id": x, "status": y, "detail": z} for x, y, z in checks],
    }
    if a.report: open(a.report, "w").write(json.dumps(out, indent=2))
    if not a.quiet: print(json.dumps(out, indent=2))
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
