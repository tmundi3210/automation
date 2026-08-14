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
import argparse, json, os, re, sys

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

# T13/G2 — ID-like token: UPPER_SNAKE with at least one underscore (the KB id
# convention). Used by the warn-level groundedness check only.
ID_TOKEN_RE = re.compile(r"\b[A-Z][A-Z0-9]*(?:_[A-Z0-9]+)+\b")
# id-bearing keys harvested from each grounded KB
KB_ID_KEYS = ("id", "source_id", "rule_id", "check_id", "step_id", "case_id",
              "axis_id", "cq_id")
KB_ID_SECTIONS = ("nodes", "edges", "conflict_axes", "edge_cases",
                  "competency_questions", "source_registry", "dominance_rules",
                  "anti_rework_rules", "iteration_protocol", "workflow",
                  "evaluation_suite")


def resolve_kb_path(p, spec_path):
    """Resolve a grounded_in_kbs path cwd-independently (T13/G2 standalone-load).

    Tries, in order: the path as given (absolute or cwd-relative — the original
    behavior), then relative to the spec file's directory, then relative to each
    ancestor of the spec file's directory (covers repo-root-relative paths like
    'knowledge_base/...' regardless of the caller's cwd). Strictly widens the
    original lookup: anything that resolved before still resolves."""
    if not isinstance(p, str) or not p:
        return None
    if os.path.exists(p):
        return p
    base = os.path.dirname(os.path.abspath(spec_path))
    for _ in range(8):
        cand = os.path.join(base, p)
        if os.path.exists(cand):
            return cand
        parent = os.path.dirname(base)
        if parent == base:
            break
        base = parent
    return None


def kb_id_universe(kb_paths):
    ids = set()
    for kp in kb_paths:
        try:
            kb = json.load(open(kp, encoding="utf-8"))
        except Exception:
            continue
        for sec in KB_ID_SECTIONS:
            v = kb.get(sec, [])
            if isinstance(v, list):
                for item in v:
                    if isinstance(item, dict):
                        for k in KB_ID_KEYS:
                            if isinstance(item.get(k), str):
                                ids.add(item[k])
    return ids


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("spec")
    ap.add_argument("--report")
    ap.add_argument("--quiet", action="store_true")
    a = ap.parse_args()
    checks = []
    def ok(c, d=""): checks.append((c, "pass", d))
    def fail(c, d=""): checks.append((c, "fail", d))
    # T13/G2: warn channel — advisory findings that NEVER affect the exit code.
    def warn(c, d=""): checks.append((c, "warn", d))

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

    # grounding: referenced KB files must exist (resolved cwd-independently —
    # as given, then against the spec file's directory and its ancestors)
    resolved = {p: resolve_kb_path(p, a.spec) for p in s.get("grounded_in_kbs", [])}
    bad = [p for p, rp in resolved.items() if rp is None]
    (ok if not bad else fail)("grounded_kbs_exist", f"missing={bad}")

    # T13/G2 warn-level KB-node groundedness: UPPER_SNAKE id-like tokens cited in
    # the spec text should resolve to an id in the grounded KBs. Advisory only:
    # legitimate non-KB tokens (artifact names, acronym pairs) can trip it.
    kb_ids = kb_id_universe([rp for rp in resolved.values() if rp])
    unresolved = sorted(t for t in set(ID_TOKEN_RE.findall(raw)) if t not in kb_ids)
    (ok if not unresolved else warn)(
        "grounding.node_ids_resolve",
        f"idlike_tokens_not_in_grounded_kbs={unresolved[:15]} total={len(unresolved)}")

    # T13/G2 warn-level CQ coverage floor: at least one covered competency
    # question per grounded KB (full KB-CQ acceptance testing is an LLM-lane
    # protocol step, not this gate — see FACTORY/MAKE_A_SPECIALIST.md section 6).
    ncq = len(s.get("competency_questions_covered") or [])
    nkb = len(s.get("grounded_in_kbs") or [])
    (ok if ncq >= max(nkb, 1) else warn)(
        "coverage.cq_floor", f"covered={ncq} grounded_kbs={nkb}")

    # capabilities shape
    caps = s.get("capabilities", [])
    badcap = [i for i, c in enumerate(caps) if not (isinstance(c, dict) and (c.get("capability") or c.get("method")) and c.get("when_to_use"))]
    (ok if isinstance(caps, list) and caps and not badcap else fail)("capabilities_shape", f"bad_idx={badcap[:10]}")

    hits = sorted({p for p in PLACEHOLDERS if p in raw})
    (ok if not hits else fail)("no_placeholders", f"found={hits}")

    failed = [c for c in checks if c[1] == "fail"]
    warned = [c for c in checks if c[1] == "warn"]
    out = {
        "_directive": "machine-facing specialist validation record; model-parse optimized",
        "validator": "specialist_validator/1.1",
        "overall_status": "fail" if failed else ("pass_with_warnings" if warned else "pass"),
        "summary": {"passed": sum(1 for c in checks if c[1] == "pass"),
                    "failed": len(failed), "warnings": len(warned)},
        "checks": [{"check_id": x, "status": y, "detail": z} for x, y, z in checks],
    }
    if a.report: open(a.report, "w").write(json.dumps(out, indent=2))
    if not a.quiet: print(json.dumps(out, indent=2))
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
