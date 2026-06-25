#!/usr/bin/env python3
"""
erotetic_lint.py — deterministic validator for typed question-objects (branch B10).

A typed question-object (see question_object.schema.json) is the output of the
erotetic compiler: a natural-language question lowered into a machine-checkable
object. This validator is the gate that decides whether such an object is
well-formed before it is allowed to drive answer collection.

CRITICAL DESIGN POINT (from adversarial review):
  "verification == answer-set membership" holds ONLY for CLOSED interrogative
  types (polar, alternative), whose admissible answer set is finite. For OPEN
  types (wh, why, how, quantitative, definitional, counterfactual) the admissible
  answer set is open-ended, so an answer cannot be scored by clean set-membership.
  For OPEN types we instead require TYPE-CONFORMANCE (the answer_schema declares
  the right shape for the type) + WARRANT-PRESENCE (a warrant slot is required).
  The validator BRANCHES on this distinction.

Checks (hard = can fail the run; soft = recorded, never fails):
  json.valid                       file parses as JSON
  schema.shape                     required top-level keys present & typed
  type.in_enum                     interrogative_type is a legal type
  answer_schema.consistent         answer_schema matches the declared type
                                   (closed -> admissible set; open -> conformance
                                    fields + warrant_required==true)
  presuppositions.present          presuppositions array present
  presuppositions.validity         each presupposition has a legal validity value
  presupposition.not_loaded        no presupposition.validity == "false"
                                   (loaded question -> refuse-or-repair)
  resolution_criteria.nonempty     stopping rule is non-empty
  decision_relevance.nonempty      decision relevance is non-empty
  sub_questions.refs_exist         every depends_on id resolves to a known id
  sub_questions.dag                sub_question dependency graph is acyclic
  answer.scorable                  closed -> admissible set usable for membership;
                                   open  -> warrant slot present

The report also carries a deterministic QUESTION-QUALITY rubric (5 dimensions,
each scored 0 / 0.5 / 1 by a mechanical rule) — this is diagnostic, never a hard
failure.

Usage:
  python3 erotetic_lint.py QUESTION.json [--report out.json] [--quiet]
Exit code 0 = PASS (no hard failure), 1 = FAIL, 2 = bad invocation. Stdlib only.
"""
import argparse
import json
import sys
from typing import Any, Dict, List, Tuple

CLOSED_TYPES = {"polar", "alternative"}
OPEN_TYPES = {"wh", "why", "how", "quantitative", "definitional", "counterfactual"}
ALL_TYPES = CLOSED_TYPES | OPEN_TYPES

VALIDITY_VALUES = {"asserted", "verified", "false", "unchecked"}

# required answer_schema fields per interrogative type (besides closed admissible)
OPEN_REQUIRED_FIELD = {
    "wh": "entity_or_relation_type",
    "why": "explanans_schema",
    "how": "ordered_step_schema",
    "quantitative": None,  # handled specially (unit + range)
    "definitional": "definiendum",
    "counterfactual": None,  # handled specially (antecedent + consequent_type)
}

REQUIRED_TOP_LEVEL = [
    "question_id", "text", "interrogative_type", "presuppositions",
    "answer_schema", "resolution_criteria", "decision_relevance", "sub_questions",
]


class Report:
    """Mirrors validators/kb_validator.py: a list of {name,status,detail} checks."""

    def __init__(self) -> None:
        self.checks: List[Tuple[str, str, str]] = []
        self.rubric: Dict[str, float] = {}
        self.violated_rules: List[str] = []

    def ok(self, cid: str, detail: str = "") -> None:
        self.checks.append((cid, "pass", detail))

    def fail(self, cid: str, detail: str = "") -> None:
        self.checks.append((cid, "fail", detail))
        self.violated_rules.append(cid)

    def warn(self, cid: str, detail: str = "") -> None:
        self.checks.append((cid, "warn", detail))

    @property
    def failed(self) -> List[Tuple[str, str, str]]:
        return [c for c in self.checks if c[1] == "fail"]

    @property
    def warned(self) -> List[Tuple[str, str, str]]:
        return [c for c in self.checks if c[1] == "warn"]

    def to_dict(self) -> Dict[str, Any]:
        return {
            "_directive": "machine-facing erotetic validation record; model-parse optimized",
            "validator": "erotetic_lint/1.0",
            "overall_status": "fail" if self.failed else (
                "pass_with_warnings" if self.warned else "pass"),
            "summary": {
                "passed": sum(1 for c in self.checks if c[1] == "pass"),
                "failed": len(self.failed),
                "warnings": len(self.warned),
            },
            "violated_rules": self.violated_rules,
            "question_quality_rubric": self.rubric,
            "checks": [{"name": a, "status": b, "detail": c} for a, b, c in self.checks],
        }


# --------------------------------------------------------------------------- #
# hard checks
# --------------------------------------------------------------------------- #
def check_shape(q: Any, r: Report) -> bool:
    if not isinstance(q, dict):
        r.fail("schema.shape", "top-level is not an object")
        return False
    missing = [k for k in REQUIRED_TOP_LEVEL if k not in q]
    if missing:
        r.fail("schema.shape", f"missing keys={missing}")
        return False
    typed_ok = (
        isinstance(q.get("question_id"), str)
        and isinstance(q.get("text"), str)
        and isinstance(q.get("interrogative_type"), str)
        and isinstance(q.get("presuppositions"), list)
        and isinstance(q.get("answer_schema"), dict)
        and isinstance(q.get("resolution_criteria"), str)
        and isinstance(q.get("decision_relevance"), str)
        and isinstance(q.get("sub_questions"), list)
    )
    if not typed_ok:
        r.fail("schema.shape", "a required field has the wrong type")
        return False
    r.ok("schema.shape", "all required keys present and typed")
    return True


def check_type_enum(q: Dict[str, Any], r: Report) -> str:
    t = q.get("interrogative_type", "")
    if t in ALL_TYPES:
        r.ok("type.in_enum", t)
    else:
        r.fail("type.in_enum", f"illegal type={t!r}")
    return t


def check_answer_schema_consistency(q: Dict[str, Any], t: str, r: Report) -> None:
    """Branch on closed vs open. This is the core erotetic correctness check."""
    a = q.get("answer_schema", {})
    if not isinstance(a, dict):
        r.fail("answer_schema.consistent", "answer_schema is not an object")
        return

    if t in CLOSED_TYPES:
        adm = a.get("admissible")
        if not (isinstance(adm, list) and len(adm) >= 1 and all(isinstance(x, str) for x in adm)):
            r.fail("answer_schema.consistent",
                   f"closed type {t} requires a non-empty 'admissible' string set")
            return
        if t == "polar":
            legal = {"yes", "no", "indeterminate"}
            illegal = [x for x in adm if x not in legal]
            if illegal:
                r.fail("answer_schema.consistent",
                       f"polar admissible must be subset of {sorted(legal)}, bad={illegal}")
                return
        if t == "alternative" and len(adm) < 2:
            r.fail("answer_schema.consistent",
                   "alternative type needs >=2 admissible alternatives")
            return
        r.ok("answer_schema.consistent",
             f"closed type {t}: admissible set of {len(adm)} -> set-membership scorable")
    elif t in OPEN_TYPES:
        # open types: require the type-specific conformance field(s) + warrant slot
        if t == "quantitative":
            unit = a.get("unit")
            rng = a.get("range")
            if not (isinstance(unit, str) and unit):
                r.fail("answer_schema.consistent", "quantitative requires non-empty 'unit'")
                return
            if not (isinstance(rng, list) and len(rng) == 2
                    and all(isinstance(x, (int, float)) for x in rng)):
                r.fail("answer_schema.consistent",
                       "quantitative requires 'range' of two numbers")
                return
        elif t == "counterfactual":
            if not (isinstance(a.get("antecedent"), str) and a.get("antecedent")):
                r.fail("answer_schema.consistent", "counterfactual requires 'antecedent'")
                return
            if not (isinstance(a.get("consequent_type"), str) and a.get("consequent_type")):
                r.fail("answer_schema.consistent", "counterfactual requires 'consequent_type'")
                return
        else:
            field = OPEN_REQUIRED_FIELD[t]
            if not (isinstance(a.get(field), str) and a.get(field)):
                r.fail("answer_schema.consistent",
                       f"open type {t} requires non-empty '{field}' conformance field")
                return
        # an admissible-set on an open type is a category error
        if "admissible" in a:
            r.fail("answer_schema.consistent",
                   f"open type {t} must NOT carry an 'admissible' set "
                   "(answer set is open-ended; score by conformance+warrant)")
            return
        r.ok("answer_schema.consistent",
             f"open type {t}: conformance fields present -> type-conformance scorable")
    else:
        r.fail("answer_schema.consistent", f"unknown type {t!r}; cannot check answer_schema")


def check_answer_scorable(q: Dict[str, Any], t: str, r: Report) -> None:
    """Closed -> admissible set usable; open -> warrant slot must be required."""
    a = q.get("answer_schema", {})
    if not isinstance(a, dict):
        r.fail("answer.scorable", "answer_schema not an object")
        return
    if t in CLOSED_TYPES:
        adm = a.get("admissible")
        if isinstance(adm, list) and adm:
            r.ok("answer.scorable",
                 f"closed: {len(adm)} admissible answers -> strict set-membership check")
        else:
            r.fail("answer.scorable", "closed type lacks a usable admissible set")
    elif t in OPEN_TYPES:
        if a.get("warrant_required") is True:
            r.ok("answer.scorable",
                 "open: warrant_required==true -> conformance+warrant scoring")
        else:
            r.fail("answer.scorable",
                   "open type requires warrant_required==true (answer set is open-ended; "
                   "no clean set-membership, so a warrant slot is mandatory)")
    else:
        r.fail("answer.scorable", f"unknown type {t!r}")


def check_presuppositions(q: Dict[str, Any], r: Report) -> None:
    pres = q.get("presuppositions", [])
    if not isinstance(pres, list):
        r.fail("presuppositions.present", "presuppositions is not a list")
        return
    r.ok("presuppositions.present", f"{len(pres)} presupposition(s)")

    bad_validity = []
    false_ones = []
    for p in pres:
        if not isinstance(p, dict):
            bad_validity.append(p)
            continue
        v = p.get("validity")
        if v not in VALIDITY_VALUES:
            bad_validity.append(p.get("id", p))
        if v == "false":
            false_ones.append(p.get("id", "?"))
    if bad_validity:
        r.fail("presuppositions.validity",
               f"missing/illegal validity on={bad_validity}")
    else:
        r.ok("presuppositions.validity", "every presupposition carries a legal validity")

    if false_ones:
        r.fail("presupposition.not_loaded",
               f"LOADED question: false presupposition(s)={false_ones} -> refuse-or-repair")
    else:
        r.ok("presupposition.not_loaded", "no false presupposition")


def check_nonempty_strings(q: Dict[str, Any], r: Report) -> None:
    rc = q.get("resolution_criteria", "")
    (r.ok if isinstance(rc, str) and rc.strip() else r.fail)(
        "resolution_criteria.nonempty",
        "present" if (isinstance(rc, str) and rc.strip()) else "empty resolution_criteria")
    dr = q.get("decision_relevance", "")
    (r.ok if isinstance(dr, str) and dr.strip() else r.fail)(
        "decision_relevance.nonempty",
        "present" if (isinstance(dr, str) and dr.strip()) else "empty decision_relevance")


def check_sub_question_dag(q: Dict[str, Any], r: Report) -> None:
    subs = q.get("sub_questions", [])
    if not isinstance(subs, list):
        r.fail("sub_questions.refs_exist", "sub_questions is not a list")
        return
    ids = [s.get("id") for s in subs if isinstance(s, dict)]
    id_set = set(ids)
    # the root question_id is also a legal dependency target
    root = q.get("question_id")
    if isinstance(root, str):
        id_set = id_set | {root}

    # type legality on sub-questions
    bad_types = [s.get("id") for s in subs
                 if isinstance(s, dict) and s.get("type") not in ALL_TYPES]
    if bad_types:
        r.fail("sub_questions.type_enum", f"illegal sub-question type on={bad_types}")
    else:
        r.ok("sub_questions.type_enum", "all sub-question types legal")

    # reference existence
    bad_refs = []
    adj: Dict[str, List[str]] = {i: [] for i in ids}
    for s in subs:
        if not isinstance(s, dict):
            continue
        sid = s.get("id")
        for dep in s.get("depends_on", []) or []:
            if dep not in id_set:
                bad_refs.append((sid, dep))
            elif sid in adj and dep in adj:
                # edge: sid depends_on dep  =>  dep -> sid
                adj[dep].append(sid)
    if bad_refs:
        r.fail("sub_questions.refs_exist", f"dangling depends_on={bad_refs}")
    else:
        r.ok("sub_questions.refs_exist", "every depends_on id resolves")

    # cycle detection (DFS 3-color) over sub-question ids only
    WHITE, GRAY, BLACK = 0, 1, 2
    color = {i: WHITE for i in ids}
    cycle: List[List[str]] = []

    def dfs(u: str, stack: List[str]) -> bool:
        color[u] = GRAY
        for v in adj.get(u, []):
            if color.get(v) == GRAY:
                cycle.append(stack + [u, v])
                return True
            if color.get(v) == WHITE and dfs(v, stack + [u]):
                return True
        color[u] = BLACK
        return False

    for i in ids:
        if color[i] == WHITE and dfs(i, []):
            break
    if cycle:
        r.fail("sub_questions.dag", f"cycle detected={cycle[0]}")
    else:
        r.ok("sub_questions.dag", "sub_question dependencies form a DAG")


# --------------------------------------------------------------------------- #
# question-quality rubric (diagnostic, deterministic, never a hard failure)
# each dimension is scored 0 / 0.5 / 1 by a mechanical rule
# --------------------------------------------------------------------------- #
def compute_rubric(q: Dict[str, Any], r: Report) -> None:
    rub: Dict[str, float] = {}

    # well_formedness: type legal + answer_schema consistent + presup validity legal
    type_ok = q.get("interrogative_type") in ALL_TYPES
    schema_ok = not any(c[0] == "answer_schema.consistent" and c[1] == "fail" for c in r.checks)
    validity_ok = not any(c[0] == "presuppositions.validity" and c[1] == "fail" for c in r.checks)
    rub["well_formedness"] = 1.0 if (type_ok and schema_ok and validity_ok) else (
        0.5 if type_ok else 0.0)

    # answerability: closed -> admissible set present; open -> warrant slot present
    t = q.get("interrogative_type")
    a = q.get("answer_schema", {}) if isinstance(q.get("answer_schema"), dict) else {}
    if t in CLOSED_TYPES:
        rub["answerability"] = 1.0 if isinstance(a.get("admissible"), list) and a.get("admissible") else 0.0
    elif t in OPEN_TYPES:
        rub["answerability"] = 1.0 if a.get("warrant_required") is True else 0.0
    else:
        rub["answerability"] = 0.0

    # presupposition_validity: any false -> 0 ; any unchecked/asserted -> 0.5 ; all verified/none -> 1
    pres = q.get("presuppositions", []) if isinstance(q.get("presuppositions"), list) else []
    vals = [p.get("validity") for p in pres if isinstance(p, dict)]
    if "false" in vals:
        rub["presupposition_validity"] = 0.0
    elif any(v in ("asserted", "unchecked") for v in vals):
        rub["presupposition_validity"] = 0.5
    else:
        rub["presupposition_validity"] = 1.0

    # scope_specificity: resolution_criteria length-banded stopping rule
    rc = q.get("resolution_criteria", "")
    rc_len = len(rc.strip()) if isinstance(rc, str) else 0
    rub["scope_specificity"] = 1.0 if rc_len >= 40 else (0.5 if rc_len >= 1 else 0.0)

    # decision_relevance: non-empty + mentions a decision/choice cue -> 1 ; non-empty -> 0.5 ; empty -> 0
    dr = q.get("decision_relevance", "")
    dr_l = dr.lower() if isinstance(dr, str) else ""
    cues = ("decid", "choos", "choice", "select", "whether to", "determine", "act", "trade")
    if dr_l.strip() and any(c in dr_l for c in cues):
        rub["decision_relevance"] = 1.0
    elif dr_l.strip():
        rub["decision_relevance"] = 0.5
    else:
        rub["decision_relevance"] = 0.0

    rub["composite"] = round(sum(rub.values()) / 5.0, 3)
    r.rubric = rub


# --------------------------------------------------------------------------- #
def validate(q: Any, r: Report) -> None:
    if not check_shape(q, r):
        # shape failed hard; still compute rubric best-effort then stop deep checks
        compute_rubric(q if isinstance(q, dict) else {}, r)
        return
    t = check_type_enum(q, r)
    check_answer_schema_consistency(q, t, r)
    check_presuppositions(q, r)
    check_nonempty_strings(q, r)
    check_sub_question_dag(q, r)
    check_answer_scorable(q, t, r)
    compute_rubric(q, r)


def main() -> int:
    ap = argparse.ArgumentParser(description="Validate a typed question-object (branch B10).")
    ap.add_argument("question", help="path to question-object JSON file")
    ap.add_argument("--report", help="write JSON report to this path")
    ap.add_argument("--quiet", action="store_true")
    args = ap.parse_args()

    try:
        raw = open(args.question, encoding="utf-8").read()
    except OSError as e:
        print(f"cannot read {args.question}: {e}", file=sys.stderr)
        return 2

    r = Report()
    try:
        q = json.loads(raw)
        r.ok("json.valid")
    except json.JSONDecodeError as e:
        r.fail("json.valid", str(e))
        out = r.to_dict()
        if args.report:
            open(args.report, "w").write(json.dumps(out, indent=2))
        if not args.quiet:
            _print_summary(out)
        return 1

    try:
        validate(q, r)
    except Exception as e:  # convert unexpected crashes into deterministic failure
        r.fail("validator.exception", f"{type(e).__name__}: {e}")

    out = r.to_dict()
    if args.report:
        open(args.report, "w").write(json.dumps(out, indent=2))
    if not args.quiet:
        _print_summary(out)
    return 1 if r.failed else 0


def _print_summary(out: Dict[str, Any]) -> None:
    s = out["summary"]
    print(f"erotetic_lint: {out['overall_status'].upper()}  "
          f"(passed={s['passed']} failed={s['failed']} warnings={s['warnings']})")
    if out["violated_rules"]:
        print("  violated_rules: " + ", ".join(out["violated_rules"]))
    rub = out.get("question_quality_rubric", {})
    if rub:
        dims = ", ".join(f"{k}={v}" for k, v in rub.items())
        print("  rubric: " + dims)
    for c in out["checks"]:
        if c["status"] == "fail":
            print(f"  FAIL {c['name']}: {c['detail']}")


if __name__ == "__main__":
    sys.exit(main())
