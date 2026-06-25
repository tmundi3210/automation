#!/usr/bin/env python3
"""
acquisition_ledger.py — deterministic tool for branch B11
(knowledge acquisition / known-unknown mapping).

ONE-LEDGER DESIGN
-----------------
There is exactly one source of truth: a competency-question (CQ) ledger
(cq_ledger.schema.json). The known-knowns matrix and the known-unknowns matrix
are DERIVED VIEWS over that ledger (pure functions, no duplicated state).
unknown-unknowns are operationalized as SAT probe-operators (sat_battery.json)
that SPAWN new known_unknown CQs into the ledger.

EVIDENCE-LABEL LEGALITY (mirrors validators/kb_validator.py ethos)
------------------------------------------------------------------
A strong evidence label may not be claimed without the required corroboration:
  - "high"     requires a non-null claim AND >= 2 sources in DISTINCT
               independence_groups (genuinely independent corroboration).
  - "moderate" requires a non-null claim AND >= 1 source.
  - "low"/"very_low"/"ungraded" carry no source-count obligation.
A known_unknown MUST have claim == null and evidence_label in
{very_low, ungraded}. A known_known MUST have a non-null claim.

CLI (mirrors validators/kb_validator.py convention)
---------------------------------------------------
  Default (validate):
    python3 acquisition_ledger.py <ledger.json> [--report out.json] [--quiet]
  Subcommands:
    python3 acquisition_ledger.py derive-views  <ledger.json> [--report out.json] [--quiet]
    python3 acquisition_ledger.py score-source  <source.json> [--ledger L --cq CQ_xxx]
    python3 acquisition_ledger.py list-probes    [--topic "..."] [--report out.json]
    python3 acquisition_ledger.py validate       <ledger.json> [--report out.json] [--quiet]

Exit code 0 = PASS (no hard failures), 1 = FAIL, 2 = bad invocation.
Stdlib only.
"""
import argparse
import json
import os
import re
import sys
from typing import Any, Dict, List, Optional, Tuple

HERE = os.path.dirname(os.path.abspath(__file__))
SAT_BATTERY_PATH = os.path.join(HERE, "sat_battery.json")
RUBRIC_PATH = os.path.join(HERE, "source_quality_rubric.json")

LEGAL_LABELS = {"high", "moderate", "low", "very_low", "ungraded"}
LEGAL_STATUS = {"known_known", "known_unknown"}
# labels that are illegitimate for an OPEN (known_unknown) entry:
KNOWN_UNKNOWN_OK_LABELS = {"very_low", "ungraded"}
CQ_ID_RE = re.compile(r"^CQ_[0-9]+$")

# source-count obligation per label (min sources, min distinct independence groups)
LABEL_OBLIGATION = {
    "high": (2, 2),
    "moderate": (1, 1),
    "low": (0, 0),
    "very_low": (0, 0),
    "ungraded": (0, 0),
}


# ----------------------------------------------------------------------------
# report object (mirrors kb_validator.Report)
# ----------------------------------------------------------------------------
class Report:
    def __init__(self) -> None:
        self.checks: List[Tuple[str, str, str]] = []

    def ok(self, cid: str, detail: str = "") -> None:
        self.checks.append((cid, "pass", detail))

    def fail(self, cid: str, detail: str = "") -> None:
        self.checks.append((cid, "fail", detail))

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
            "_directive": "machine-facing validation record; model-parse optimized",
            "validator": "acquisition_ledger/1.0",
            "overall_status": "fail" if self.failed
            else ("pass_with_warnings" if self.warned else "pass"),
            "summary": {
                "passed": sum(1 for c in self.checks if c[1] == "pass"),
                "failed": len(self.failed),
                "warnings": len(self.warned),
            },
            "checks": [{"check_id": a, "status": b, "detail": c} for a, b, c in self.checks],
        }


def clamp01(x: float) -> float:
    return min(1.0, max(0.0, x))


def _num(d: Dict[str, Any], k: str) -> Optional[float]:
    v = d.get(k)
    return v if isinstance(v, (int, float)) and not isinstance(v, bool) else None


# ----------------------------------------------------------------------------
# evidence-label legality (the heart of the gate)
# ----------------------------------------------------------------------------
def distinct_independence_groups(sources: List[Dict[str, Any]]) -> List[str]:
    """Distinct independence_group labels present in a source list (order-stable)."""
    seen: List[str] = []
    for s in sources or []:
        g = s.get("independence_group")
        if isinstance(g, str) and g not in seen:
            seen.append(g)
    return seen


def evidence_label_legal(entry: Dict[str, Any]) -> Tuple[bool, str]:
    """
    Return (is_legal, reason). Enforces the source-count obligation for the
    label AND the status<->label/claim coherence rules.
    """
    label = entry.get("evidence_label")
    status = entry.get("status")
    claim = entry.get("claim")
    sources = entry.get("sources") or []
    cq = entry.get("cq_id")

    if label not in LEGAL_LABELS:
        return False, f"{cq}: illegal evidence_label '{label}'"

    # status / claim coherence
    if status == "known_unknown":
        if claim is not None:
            return False, f"{cq}: known_unknown must have null claim (got a claim)"
        if label not in KNOWN_UNKNOWN_OK_LABELS:
            return False, (f"{cq}: known_unknown may only be labeled "
                           f"{sorted(KNOWN_UNKNOWN_OK_LABELS)} (got '{label}')")
        return True, f"{cq}: ok (open CQ)"
    if status == "known_known":
        if claim is None or (isinstance(claim, str) and not claim.strip()):
            return False, f"{cq}: known_known must have a non-null, non-empty claim"

    # source-count obligation
    min_sources, min_groups = LABEL_OBLIGATION[label]
    n_sources = len(sources)
    groups = distinct_independence_groups(sources)
    n_groups = len(groups)
    if n_sources < min_sources:
        return False, (f"{cq}: evidence_label '{label}' requires >= {min_sources} "
                       f"source(s); has {n_sources}")
    if n_groups < min_groups:
        return False, (f"{cq}: evidence_label '{label}' requires >= {min_groups} "
                       f"DISTINCT independence_group(s); has {n_groups} "
                       f"(groups={groups}) -- common-source dependence is not "
                       f"independent corroboration")
    return True, f"{cq}: ok ('{label}' backed by {n_groups} independent group(s))"


# ----------------------------------------------------------------------------
# schema validation (lightweight, stdlib-only; mirrors schema file)
# ----------------------------------------------------------------------------
def check_schema(ledger: Dict[str, Any], r: Report) -> None:
    if not isinstance(ledger, dict):
        r.fail("schema.root_object", "ledger root is not an object")
        return
    topic = ledger.get("topic")
    (r.ok if isinstance(topic, str) and topic.strip() else r.fail)(
        "schema.topic", f"topic={topic!r}")

    entries = ledger.get("entries")
    if not isinstance(entries, list):
        r.fail("schema.entries_is_list", f"type={type(entries).__name__}")
        return
    r.ok("schema.entries_is_list", f"{len(entries)} entries")

    ids: List[str] = []
    required = ["cq_id", "question", "status", "claim",
                "evidence_label", "sources", "spawned_by", "created"]
    src_required = ["id", "title", "authority", "recency", "relevance", "independence_group"]
    bad_fields, bad_status, bad_idform, bad_src = [], [], [], []

    for e in entries:
        if not isinstance(e, dict):
            bad_fields.append(("<non-object entry>", "not an object"))
            continue
        cq = e.get("cq_id")
        ids.append(cq if isinstance(cq, str) else repr(cq))
        for k in required:
            if k not in e:
                bad_fields.append((cq, f"missing:{k}"))
        if not (isinstance(cq, str) and CQ_ID_RE.match(cq)):
            bad_idform.append(cq)
        if e.get("status") not in LEGAL_STATUS:
            bad_status.append((cq, e.get("status")))
        srcs = e.get("sources")
        if not isinstance(srcs, list):
            bad_src.append((cq, "sources not a list"))
        else:
            for s in srcs:
                if not isinstance(s, dict):
                    bad_src.append((cq, "source not an object"))
                    continue
                for k in src_required:
                    if k not in s:
                        bad_src.append((cq, f"source missing:{k}"))
                for nk in ("authority", "recency", "relevance"):
                    v = _num(s, nk)
                    if v is None or not (0.0 <= v <= 1.0):
                        bad_src.append((cq, f"source.{nk} out of [0,1]: {s.get(nk)!r}"))

    (r.ok if not bad_fields else r.fail)("schema.entry_required_fields", f"bad={bad_fields[:20]}")
    (r.ok if not bad_idform else r.fail)("schema.cq_id_form", f"bad={bad_idform[:20]}")
    (r.ok if not bad_status else r.fail)("schema.status_legal", f"bad={bad_status[:20]}")
    (r.ok if not bad_src else r.fail)("schema.source_fields", f"bad={bad_src[:20]}")

    dups = sorted({x for x in ids if ids.count(x) > 1})
    (r.ok if not dups else r.fail)("schema.cq_id_unique", f"dups={dups}")


def check_evidence_legality(ledger: Dict[str, Any], r: Report) -> None:
    entries = ledger.get("entries", [])
    illegal: List[str] = []
    for e in entries:
        if not isinstance(e, dict):
            continue
        legal, reason = evidence_label_legal(e)
        if not legal:
            illegal.append(reason)
    (r.ok if not illegal else r.fail)("evidence.label_legality", f"violations={illegal[:20]}")


def check_spawn_refs(ledger: Dict[str, Any], r: Report) -> None:
    """spawned_by, when present, must reference a real probe in the SAT battery."""
    try:
        battery = load_battery()
        probe_ids = {p["probe_id"] for p in battery.get("probes", [])}
    except Exception as ex:  # pragma: no cover - defensive
        r.warn("spawn.battery_loaded", f"could not load SAT battery: {ex}")
        return
    bad = []
    for e in ledger.get("entries", []):
        if not isinstance(e, dict):
            continue
        sb = e.get("spawned_by")
        if sb is not None and sb not in probe_ids:
            bad.append((e.get("cq_id"), sb))
    (r.ok if not bad else r.fail)("spawn.refs_resolve",
                                  f"unknown_probe_ids={bad[:20]} known={sorted(probe_ids)}")


def validate_ledger(ledger: Dict[str, Any]) -> Report:
    r = Report()

    def guard(fn) -> None:
        try:
            fn(ledger, r)
        except Exception as ex:  # convert crashes into deterministic FAIL
            r.fail("validator.exception", f"{fn.__name__}: {type(ex).__name__}: {ex}")

    guard(check_schema)
    guard(check_evidence_legality)
    guard(check_spawn_refs)
    return r


# ----------------------------------------------------------------------------
# derived views (pure functions over the ONE ledger; no duplicate state)
# ----------------------------------------------------------------------------
def derive_known_known_matrix(ledger: Dict[str, Any]) -> List[Dict[str, Any]]:
    rows = []
    for e in ledger.get("entries", []):
        if isinstance(e, dict) and e.get("status") == "known_known":
            rows.append({
                "cq_id": e.get("cq_id"),
                "question": e.get("question"),
                "claim": e.get("claim"),
                "evidence_label": e.get("evidence_label"),
                "n_sources": len(e.get("sources") or []),
                "n_independent_groups": len(distinct_independence_groups(e.get("sources") or [])),
            })
    return rows


def derive_known_unknown_matrix(ledger: Dict[str, Any]) -> List[Dict[str, Any]]:
    rows = []
    for e in ledger.get("entries", []):
        if isinstance(e, dict) and e.get("status") == "known_unknown":
            rows.append({
                "cq_id": e.get("cq_id"),
                "question": e.get("question"),
                "spawned_by": e.get("spawned_by"),
                "evidence_label": e.get("evidence_label"),
            })
    return rows


def derive_views(ledger: Dict[str, Any]) -> Dict[str, Any]:
    kk = derive_known_known_matrix(ledger)
    ku = derive_known_unknown_matrix(ledger)
    return {
        "_directive": "DERIVED views over the single CQ ledger; do not maintain separately",
        "topic": ledger.get("topic"),
        "counts": {
            "total_entries": len(ledger.get("entries", [])),
            "known_known": len(kk),
            "known_unknown": len(ku),
            "spawned_by_probe": sum(1 for e in ledger.get("entries", [])
                                    if isinstance(e, dict) and e.get("spawned_by")),
        },
        "known_known_matrix": kk,
        "known_unknown_matrix": ku,
    }


# ----------------------------------------------------------------------------
# source scoring (applies source_quality_rubric.json)
# ----------------------------------------------------------------------------
def load_rubric() -> Dict[str, Any]:
    with open(RUBRIC_PATH, encoding="utf-8") as f:
        return json.load(f)


def load_battery() -> Dict[str, Any]:
    with open(SAT_BATTERY_PATH, encoding="utf-8") as f:
        return json.load(f)


def score_source(source: Dict[str, Any],
                 cq_sources: Optional[List[Dict[str, Any]]] = None) -> Dict[str, Any]:
    """
    Apply source_quality_rubric to a single source object.
    If cq_sources (all sources on the same CQ) is given, independence and
    corroboration are computed in context; otherwise standalone defaults apply.
    """
    rubric = load_rubric()
    dims = rubric["dimensions"]
    w = {k: dims[k]["weight"] for k in dims}

    authority = clamp01(_num(source, "authority") or 0.0)
    recency = clamp01(_num(source, "recency") or 0.0)
    relevance = clamp01(_num(source, "relevance") or 0.0)

    if cq_sources:
        groups = distinct_independence_groups(cq_sources)
        my_group = source.get("independence_group")
        same_group = sum(1 for s in cq_sources
                         if s.get("independence_group") == my_group)
        independence = 1.0 if same_group <= 1 else 0.0
        corroboration = clamp01(float(len(groups) - 1))
    else:
        independence = 1.0
        corroboration = 0.0

    score = clamp01(
        w["authority"] * authority
        + w["recency"] * recency
        + w["relevance"] * relevance
        + w["independence"] * independence
        + w["corroboration"] * corroboration
    )

    band = "poor"
    for b in rubric["scoring_rule"]["bands"]:
        if b["min"] <= score <= b["max"]:
            band = b["label"]
            break

    return {
        "source_id": source.get("id"),
        "components": {
            "authority": round(authority, 4),
            "recency": round(recency, 4),
            "relevance": round(relevance, 4),
            "independence": round(independence, 4),
            "corroboration": round(corroboration, 4),
        },
        "score": round(score, 4),
        "band": band,
    }


# ----------------------------------------------------------------------------
# probe listing + spawning
# ----------------------------------------------------------------------------
def spawn_cqs_for_topic(topic: str) -> List[Dict[str, Any]]:
    """For each probe, render the known_unknown CQ it would spawn for a topic."""
    battery = load_battery()
    out = []
    for p in battery.get("probes", []):
        tmpl = p.get("question_template", "")
        out.append({
            "probe_id": p.get("probe_id"),
            "name": p.get("name"),
            "owned_by": p.get("owned_by"),
            "spawns": p.get("spawns"),
            "spawned_known_unknown_cq": tmpl.replace("{topic}", topic),
        })
    return out


def list_probes(topic: Optional[str]) -> Dict[str, Any]:
    battery = load_battery()
    probes_view = [{
        "probe_id": p.get("probe_id"),
        "name": p.get("name"),
        "owned_by": p.get("owned_by"),
        "reuse_citation": p.get("reuse_citation"),
        "description": p.get("description"),
        "spawns": p.get("spawns"),
        "question_template": p.get("question_template"),
    } for p in battery.get("probes", [])]
    result = {
        "battery_id": battery.get("battery_id"),
        "reuse_note": battery.get("reuse_note"),
        "probes": probes_view,
    }
    if topic:
        result["spawned_for_topic"] = {
            "topic": topic,
            "known_unknown_cqs": spawn_cqs_for_topic(topic),
        }
    return result


# ----------------------------------------------------------------------------
# CLI
# ----------------------------------------------------------------------------
def _read_json(path: str) -> Any:
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def _emit(obj: Any, report_path: Optional[str], quiet: bool) -> None:
    text = json.dumps(obj, indent=2)
    if report_path:
        with open(report_path, "w", encoding="utf-8") as f:
            f.write(text)
    if not quiet:
        print(text)


def cmd_validate(args: argparse.Namespace) -> int:
    try:
        ledger = _read_json(args.ledger)
    except OSError as e:
        print(f"cannot read {args.ledger}: {e}", file=sys.stderr)
        return 2
    except json.JSONDecodeError as e:
        r = Report()
        r.fail("json.valid", str(e))
        _emit(r.to_dict(), args.report, args.quiet)
        return 1
    r = validate_ledger(ledger)
    _emit(r.to_dict(), args.report, args.quiet)
    return 1 if r.failed else 0


def cmd_derive_views(args: argparse.Namespace) -> int:
    try:
        ledger = _read_json(args.ledger)
    except (OSError, json.JSONDecodeError) as e:
        print(f"cannot read/parse {args.ledger}: {e}", file=sys.stderr)
        return 2
    _emit(derive_views(ledger), args.report, args.quiet)
    return 0


def cmd_score_source(args: argparse.Namespace) -> int:
    try:
        source = _read_json(args.source)
    except (OSError, json.JSONDecodeError) as e:
        print(f"cannot read/parse {args.source}: {e}", file=sys.stderr)
        return 2
    cq_sources = None
    if args.ledger and args.cq:
        ledger = _read_json(args.ledger)
        for e in ledger.get("entries", []):
            if isinstance(e, dict) and e.get("cq_id") == args.cq:
                cq_sources = e.get("sources") or []
                break
    _emit(score_source(source, cq_sources), args.report, args.quiet)
    return 0


def cmd_list_probes(args: argparse.Namespace) -> int:
    _emit(list_probes(args.topic), args.report, args.quiet)
    return 0


def build_parser() -> argparse.ArgumentParser:
    ap = argparse.ArgumentParser(
        description="B11 acquisition ledger tool (validate / derive-views / "
                    "score-source / list-probes). Default action is validate.")
    sub = ap.add_subparsers(dest="command")

    p_val = sub.add_parser("validate", help="schema + evidence-label legality check")
    p_val.add_argument("ledger")
    p_val.add_argument("--report")
    p_val.add_argument("--quiet", action="store_true")
    p_val.set_defaults(func=cmd_validate)

    p_dv = sub.add_parser("derive-views", help="emit known_known/known_unknown matrices")
    p_dv.add_argument("ledger")
    p_dv.add_argument("--report")
    p_dv.add_argument("--quiet", action="store_true")
    p_dv.set_defaults(func=cmd_derive_views)

    p_ss = sub.add_parser("score-source", help="apply source_quality_rubric to a source")
    p_ss.add_argument("source")
    p_ss.add_argument("--ledger", help="ledger for in-context independence/corroboration")
    p_ss.add_argument("--cq", help="cq_id whose source set provides context")
    p_ss.add_argument("--report")
    p_ss.add_argument("--quiet", action="store_true")
    p_ss.set_defaults(func=cmd_score_source)

    p_lp = sub.add_parser("list-probes", help="print SAT battery; spawn CQs for a topic")
    p_lp.add_argument("--topic")
    p_lp.add_argument("--report")
    p_lp.add_argument("--quiet", action="store_true")
    p_lp.set_defaults(func=cmd_list_probes)

    return ap


def main(argv: Optional[List[str]] = None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    # Default action (no recognized subcommand) is `validate <ledger.json>`,
    # mirroring `python3 kb_validator.py KB.json ...`.
    known = {"validate", "derive-views", "score-source", "list-probes"}
    if not argv:
        build_parser().print_help()
        return 2
    if argv[0] not in known and not argv[0].startswith("-"):
        argv = ["validate"] + argv
    ap = build_parser()
    args = ap.parse_args(argv)
    if not getattr(args, "func", None):
        ap.print_help()
        return 2
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
