#!/usr/bin/env python3
"""
kb_validator.py — deterministic quality + quantity gate for KB_GENERATION outputs
produced by schema/kb_generator_v1.4.1.txt.

This is the orchestrator's enforcement layer. A helper sub-agent generates a KB
JSON file; this validator decides pass/fail BEFORE the artifact is accepted.

It checks two independent dimensions:
  QUANTITY  -> density-mode object counts (nodes, edges, axes, cases, steps, CQs,
              dominance/anti-rework/iteration items)
  QUALITY   -> JSON validity, required top-level keys, ID uniqueness, reference
              integrity (edges/deps/workflow/edge-cases/conflict-axes/sources/CQs),
              numeric bounds, conflict-edge sign rule, high-risk / high-coupling
              obligations, uncertainty intervals, dependency-cycle absence,
              placeholder leakage, evidence-label legality, and formula consistency.

Usage:
  python3 kb_validator.py KB.json --mode dense [--allow-observed] [--tolerance 0.02]
                                  [--report out.json] [--quiet]

Exit code 0 = pass (no hard failures), 1 = hard failures, 2 = bad invocation.
Stdlib only.
"""
import argparse
import json
import re
import sys

# ----- density-mode quantity targets (from schema DENSITY MODE section) -----
DENSITY = {
    "compact":  {"nodes": (8, 12),  "edges": (12, 20), "conflict_axes": (4, 6),
                 "edge_cases": (5, 8),  "workflow": (5, 7),  "competency_questions": (5, 8)},
    "standard": {"nodes": (13, 18), "edges": (20, 32), "conflict_axes": (6, 8),
                 "edge_cases": (8, 10), "workflow": (7, 10), "competency_questions": (8, 12)},
    "dense":    {"nodes": (19, 24), "edges": (32, 40), "conflict_axes": (8, 10),
                 "edge_cases": (10, 12), "workflow": (9, 12), "competency_questions": (10, 14)},
}
# applies to all modes (schema: "Always include ..."); relaxed only by compact safety
GLOBAL_COUNTS = {
    "dominance_rules": (7, 12),
    "anti_rework_rules": (7, 12),
    "iteration_protocol": (6, 10),
}

REQUIRED_TOP_LEVEL = [
    "domain", "domain_label", "purpose", "empirical_status", "assumptions",
    "exclusions", "schema_version", "generation_metadata", "schema_contract",
    "input_summary", "competency_questions", "glossary", "lens_coverage",
    "evidence_label_policy", "source_registry", "math_model", "nodes", "edges",
    "conflict_axes", "edge_cases", "workflow", "priority_order",
    "priority_rationale", "dominance_rules", "anti_rework_rules",
    "iteration_protocol", "validation_protocol", "validation_report",
    "evaluation_suite", "traceability_matrix", "change_control",
]

LEGAL_EVIDENCE_LABELS = {"heuristic", "expert_estimate", "observed", "experimentally_validated"}
LEGAL_EDGE_TYPES = {"dependency", "constraint", "conflict", "causal", "sequence", "feedback", "similarity"}

# ---- schema-version pin (T13/G6) -------------------------------------------
# The kernel (schema/kb_generator_v1.4.1.txt, VERSIONING MODEL) mandates that the
# top-level schema_version IS the KB schema version and MUST equal
# generation_metadata.kb_schema_version. This validator transcribes the v1.3-style
# KB schema bands/formulas; it therefore only supports the versions listed here.
# A future kernel bump must extend this set CONSCIOUSLY (after re-deriving bands
# and formulas), never silently. Verified 2026-07-02: all 204 *.kb.json artifacts
# in this repo declare schema_version == generation_metadata.kb_schema_version == "1.3".
SUPPORTED_KB_SCHEMA_VERSIONS = {"1.3"}

# ---- source_registry entry contract (T13/G4, warn-level) --------------------
# Kernel "source_registry item structure": every source record must carry these
# fields ("citation_or_locator when available" — empty string is legal for
# heuristic priors: "user_supplied_identifier_or_empty_string").
SOURCE_ENTRY_FIELDS = [
    "source_id", "source_type", "label_eligibility", "description",
    "citation_or_locator", "supports", "does_not_support", "freshness_status",
    "provenance", "grounding_limits",
]
# source types that legitimately carry an empty locator (model-internal priors);
# any OTHER source_type claims external authority and should name its locator.
INTERNAL_SOURCE_TYPES = {"heuristic_prior", "expert_estimate"}
STRONG_EVIDENCE_LABELS = {"observed", "experimentally_validated"}
DOI_RE = re.compile(r"^10\.\d{4,9}/\S+$")
URL_RE = re.compile(r"^https?://\S+\.\S+")

# literal template placeholders that must NOT survive into a populated KB
PLACEHOLDERS = {
    "precise_pro", "precise_con", "concrete_example", "concrete_domain_specific_example",
    "NODE_A", "NODE_B", "SHORT_ID", "precise_relation", "precise_benefit", "precise_risk",
    "machine_testable_question", "human_readable_domain_name", "normalized_domain_id",
    "precise_machine_readable_purpose", "compact_topic_name", "one_sentence_operational_definition",
    "[DOMAIN]", "CHECK_ID", "ARTIFACT_ID", "EDGE_PATTERN_ID", "FAMILY_ID", "SECTION_ID",
    "which_side_dominates_or_how_to_resolve_when_conflict_constraint_or_feedback_occurs",
}

# generic draft markers that must NOT survive into a populated KB (word-boundary,
# uppercase-only for the code-style markers so prose words like "autodoc" never match;
# verified 0 hits across all 204 repo KBs before adding — T13 fault-injection fix)
DRAFT_MARKER_RES = [
    re.compile(r"\bTODO\b"), re.compile(r"\bFIXME\b"), re.compile(r"\bTBD\b"),
    re.compile(r"\bXXX\b"), re.compile(r"lorem\s+ipsum", re.IGNORECASE),
]

EDGE_ID_RE = re.compile(r"^EDGE_\d+$")


class Report:
    def __init__(self):
        self.checks = []   # (check_id, status, detail) status in pass/fail/warn

    def ok(self, cid, detail=""):
        self.checks.append((cid, "pass", detail))

    def fail(self, cid, detail=""):
        self.checks.append((cid, "fail", detail))

    def warn(self, cid, detail=""):
        self.checks.append((cid, "warn", detail))

    @property
    def failed(self):
        return [c for c in self.checks if c[1] == "fail"]

    @property
    def warned(self):
        return [c for c in self.checks if c[1] == "warn"]

    def to_dict(self, mode):
        return {
            "_directive": "machine-facing validation record; model-parse optimized",
            "validator": "kb_validator/1.0",
            "density_mode": mode,
            "overall_status": "fail" if self.failed else ("pass_with_warnings" if self.warned else "pass"),
            "summary": {
                "passed": sum(1 for c in self.checks if c[1] == "pass"),
                "failed": len(self.failed),
                "warnings": len(self.warned),
            },
            "checks": [{"check_id": a, "status": b, "detail": c} for a, b, c in self.checks],
        }


def normalize(x):
    return min(1.0, max(0.0, x))


def _num(d, k, default=None):
    v = d.get(k, default)
    return v if isinstance(v, (int, float)) else default


def check_counts(kb, mode, r):
    tgt = DENSITY[mode]
    sized = {
        "nodes": kb.get("nodes", []),
        "edges": kb.get("edges", []),
        "conflict_axes": kb.get("conflict_axes", []),
        "edge_cases": kb.get("edge_cases", []),
        "workflow": kb.get("workflow", []),
        "competency_questions": kb.get("competency_questions", []),
    }
    for key, (lo, hi) in tgt.items():
        n = len(sized[key])
        (r.ok if lo <= n <= hi else r.fail)(f"count.{key}", f"{n} (target {lo}-{hi})")
        # ADDITIVE ANNOTATION ONLY (T13/G9). Kernel CORE GUARANTEE ORDER ranks
        # "Useful density" #9 (last); a <=10% band miss is machine-flagged here so the
        # orchestrator can route a RECORDED, HUMAN-GATED waiver decision. This warn
        # fires only next to an already-FAILED count check: overall_status and the
        # exit code are unchanged (the KB still fails). Never an automatic waiver —
        # the bands are calibrated cut scores (calibration/results/DECISION.json);
        # changing acceptance requires standard-setting review, not code.
        if n < lo and (lo - n) <= max(1, round(0.10 * lo)):
            r.warn(f"count.{key}.near_miss",
                   f"{n} misses lo={lo} by {lo - n} (<=10% of band edge); "
                   "eligible for orchestrator/human-gated waiver review, NOT auto-accept")
        elif n > hi and (n - hi) <= max(1, round(0.10 * hi)):
            r.warn(f"count.{key}.near_miss",
                   f"{n} exceeds hi={hi} by {n - hi} (<=10% of band edge); "
                   "eligible for orchestrator/human-gated waiver review, NOT auto-accept")
    for key, (lo, hi) in GLOBAL_COUNTS.items():
        n = len(kb.get(key, []))
        # compact mode may relax these per schema "unless compact mode is required"
        if mode == "compact" and n < lo:
            r.warn(f"count.{key}", f"{n} (target {lo}-{hi}, relaxed for compact)")
        else:
            (r.ok if lo <= n <= hi else r.fail)(f"count.{key}", f"{n} (target {lo}-{hi})")


def check_top_level(kb, r):
    missing = [k for k in REQUIRED_TOP_LEVEL if k not in kb]
    (r.ok if not missing else r.fail)("toplevel.required_keys", f"missing={missing}")


def check_ids_and_refs(kb, r):
    nodes = kb.get("nodes", [])
    node_ids = [n.get("id") for n in nodes]
    nid_set = set(node_ids)
    dup_nodes = [x for x in node_ids if node_ids.count(x) > 1]
    (r.ok if not dup_nodes else r.fail)("nodes.id_unique", f"dups={sorted(set(dup_nodes))}")

    edges = kb.get("edges", [])
    edge_ids = [e.get("id") for e in edges]
    dup_edges = [x for x in edge_ids if edge_ids.count(x) > 1]
    (r.ok if not dup_edges else r.fail)("edges.id_unique", f"dups={sorted(set(dup_edges))}")
    bad_form = [e for e in edge_ids if not (isinstance(e, str) and EDGE_ID_RE.match(e))]
    (r.ok if not bad_form else r.warn)("edges.id_form", f"non_EDGE_nnn={bad_form}")

    # edge endpoint validity + conflict sign + resolution rule
    bad_ep, bad_sign, no_res, bad_type = [], [], [], []
    for e in edges:
        eid = e.get("id")
        if e.get("from") not in nid_set or e.get("to") not in nid_set:
            bad_ep.append(eid)
        if e.get("edge_type") not in LEGAL_EDGE_TYPES:
            bad_type.append(eid)
        if e.get("edge_type") == "conflict":
            st = _num(e, "signed_tension")
            if st is None or st >= 0:
                bad_sign.append(eid)
            if not (e.get("resolution_rule") or "").strip():
                no_res.append(eid)
    (r.ok if not bad_ep else r.fail)("edges.endpoint_valid", f"bad={bad_ep}")
    (r.ok if not bad_type else r.fail)("edges.type_legal", f"bad={bad_type}")
    (r.ok if not bad_sign else r.fail)("edges.conflict_signed_tension_negative", f"bad={bad_sign}")
    (r.ok if not no_res else r.fail)("edges.conflict_resolution_rule_present", f"bad={no_res}")

    # node dependency + must_not_finalize_before refs
    bad_dep = []
    for n in nodes:
        for ref in n.get("dependencies", []) + n.get("must_not_finalize_before", []):
            if ref not in nid_set:
                bad_dep.append((n.get("id"), ref))
    (r.ok if not bad_dep else r.fail)("nodes.dependency_refs_valid", f"bad={bad_dep}")

    # workflow node refs
    bad_wf = []
    for s in kb.get("workflow", []):
        for ref in s.get("nodes", []):
            if ref not in nid_set:
                bad_wf.append((s.get("step"), ref))
    (r.ok if not bad_wf else r.fail)("workflow.node_refs_valid", f"bad={bad_wf}")

    # edge_case node refs
    bad_ec = []
    for c in kb.get("edge_cases", []):
        for ref in c.get("nodes", []):
            if ref not in nid_set:
                bad_ec.append((c.get("id"), ref))
    (r.ok if not bad_ec else r.fail)("edge_cases.node_refs_valid", f"bad={bad_ec}")

    # conflict axis node refs (only flag uppercase-token refs that look like node IDs)
    bad_ca = []
    for a in kb.get("conflict_axes", []):
        for ref in a.get("side_a", []) + a.get("side_b", []):
            if isinstance(ref, str) and re.match(r"^[A-Z][A-Z0-9_]+$", ref) and ref not in nid_set:
                bad_ca.append((a.get("id"), ref))
    (r.ok if not bad_ca else r.warn)("conflict_axes.node_refs_valid", f"unresolved_idlike={bad_ca}")

    # priority_order: every node exactly once
    po = kb.get("priority_order", [])
    po_norm = [p.get("node_id") if isinstance(p, dict) else p for p in po]
    if sorted([x for x in po_norm if x is not None]) == sorted(node_ids) and len(po_norm) == len(node_ids):
        r.ok("priority_order.complete", f"{len(po_norm)} nodes")
    else:
        r.fail("priority_order.complete",
               f"po={len(po_norm)} nodes={len(node_ids)} missing={sorted(nid_set-set(po_norm))} extra={sorted(set(po_norm)-nid_set)}")

    # source reference validity (evidence_refs -> source_registry)
    src_ids = {s.get("source_id") for s in kb.get("source_registry", [])}
    bad_src = []
    for n in nodes:
        for ref in n.get("evidence_refs", []):
            if ref not in src_ids:
                bad_src.append(("node", n.get("id"), ref))
    for e in edges:
        for ref in e.get("evidence_refs", []):
            if ref not in src_ids:
                bad_src.append(("edge", e.get("id"), ref))
    (r.ok if not bad_src else r.fail)("evidence_refs.resolve", f"bad={bad_src[:20]}")

    # competency-question refs
    cq_ids = {q.get("id") for q in kb.get("competency_questions", [])}
    bad_cq = []
    for n in nodes:
        for ref in n.get("competency_question_refs", []):
            if ref not in cq_ids:
                bad_cq.append((n.get("id"), ref))
    (r.ok if not bad_cq else r.fail)("competency_question_refs.resolve", f"bad={bad_cq}")

    return nid_set


def check_bounds_and_obligations(kb, r, allow_observed):
    bad_bounds, bad_tension, bad_unc = [], [], []
    hr_missing, hc_missing = [], []
    illegal_labels, illegal_observed = [], []

    def scan_numbers(obj, path):
        if isinstance(obj, dict):
            for k, v in obj.items():
                scan_numbers(v, f"{path}.{k}")
        elif isinstance(obj, list):
            for i, v in enumerate(obj):
                scan_numbers(v, f"{path}[{i}]")

    for n in kb.get("nodes", []):
        nid = n.get("id")
        m = n.get("metrics", {})
        for k, v in m.items():
            if isinstance(v, (int, float)) and not (0.0 <= v <= 1.0):
                bad_bounds.append((nid, k, v))
        # high-risk obligation
        if _num(m, "risk_if_wrong", 0) >= 0.80:
            if not n.get("acceptance_tests") or n.get("human_review_required") is not True:
                hr_missing.append(nid)
        # high-coupling obligation
        if _num(m, "cross_topic_coupling", 0) >= 0.75:
            if not n.get("revisit_triggers"):
                hc_missing.append(nid)
        # uncertainty interval
        pl = n.get("probabilistic_layer", {})
        ui = pl.get("uncertainty_interval")
        if not (isinstance(ui, list) and len(ui) == 2 and
                all(isinstance(x, (int, float)) for x in ui) and
                0.0 <= ui[0] <= ui[1] <= 1.0):
            bad_unc.append((nid, ui))
        # observed/experimental labels without data
        if not allow_observed:
            for lab_field in ("metric_labels", "score_derivation_input_labels"):
                for k, v in n.get(lab_field, {}).items():
                    if v in ("observed", "experimentally_validated"):
                        illegal_observed.append((nid, lab_field, k))
            for k, v in pl.get("value_labels", {}).items():
                if v in ("observed", "experimentally_validated"):
                    illegal_observed.append((nid, "value_labels", k))

    for e in kb.get("edges", []):
        eid = e.get("id")
        st = _num(e, "signed_tension")
        if st is not None and not (-1.0 <= st <= 1.0):
            bad_tension.append((eid, st))
        for k in ("relation_strength", "edge_score", "edge_heat", "edge_priority",
                  "prior_relation_strength", "observed_co_occurrence", "causal_confidence",
                  "conflict_probability", "expected_rework_cost"):
            v = _num(e, k)
            if v is not None and not (0.0 <= v <= 1.0):
                bad_bounds.append((eid, k, v))

    # collect all evidence_label values anywhere they appear at top level objects
    def collect_labels(obj):
        if isinstance(obj, dict):
            for k, v in obj.items():
                if k in ("evidence_label",) and isinstance(v, str):
                    if v not in LEGAL_EVIDENCE_LABELS:
                        illegal_labels.append(v)
                else:
                    collect_labels(v)
        elif isinstance(obj, list):
            for v in obj:
                collect_labels(v)
    collect_labels(kb)

    (r.ok if not bad_bounds else r.fail)("bounds.scores_in_0_1", f"bad={bad_bounds[:20]}")
    (r.ok if not bad_tension else r.fail)("bounds.signed_tension_in_-1_1", f"bad={bad_tension}")
    (r.ok if not bad_unc else r.fail)("nodes.uncertainty_interval_valid", f"bad={bad_unc}")
    (r.ok if not hr_missing else r.fail)("nodes.high_risk_obligations", f"bad={hr_missing}")
    (r.ok if not hc_missing else r.fail)("nodes.high_coupling_revisit_triggers", f"bad={hc_missing}")
    (r.ok if not illegal_labels else r.fail)("evidence.labels_legal", f"bad={set(illegal_labels)}")
    if not allow_observed:
        (r.ok if not illegal_observed else r.fail)("evidence.no_observed_without_data", f"bad={illegal_observed[:20]}")


def check_dependency_acyclicity(kb, r, nid_set):
    # build dependency graph from edges of type dependency (source -> target)
    adj = {n: [] for n in nid_set}
    for e in kb.get("edges", []):
        if e.get("edge_type") == "dependency":
            f, t = e.get("from"), e.get("to")
            if f in adj and t in nid_set:
                adj[f].append(t)
    # also node.dependencies (dep is prerequisite -> node): dep -> node
    for n in kb.get("nodes", []):
        nid = n.get("id")
        for dep in n.get("dependencies", []):
            if dep in adj and nid in nid_set:
                adj[dep].append(nid)
    WHITE, GRAY, BLACK = 0, 1, 2
    color = {n: WHITE for n in adj}
    cycle = []

    def dfs(u, stack):
        color[u] = GRAY
        for v in adj.get(u, []):
            if color.get(v) == GRAY:
                cycle.append(stack + [u, v])
                return True
            if color.get(v) == WHITE and dfs(v, stack + [u]):
                return True
        color[u] = BLACK
        return False

    for n in adj:
        if color[n] == WHITE and dfs(n, []):
            break
    (r.ok if not cycle else r.fail)("dependency.acyclic", f"cycle={cycle[:1]}")


def check_placeholders(raw_text, r):
    hits = sorted({p for p in PLACEHOLDERS if p in raw_text})
    hits += sorted({m.group(0) for rx in DRAFT_MARKER_RES for m in [rx.search(raw_text)] if m})
    (r.ok if not hits else r.fail)("placeholders.none_leaked", f"found={hits}")


def check_formulas(kb, r, tol, allow_observed):
    errs = []
    for n in kb.get("nodes", []):
        nid = n.get("id")
        m = n.get("metrics", {})
        sdi = n.get("score_derivation_inputs", {})
        pl = n.get("probabilistic_layer", {})
        C = _num(m, "criticality", 0); BV = _num(m, "business_value", 0)
        UV = _num(m, "user_value", 0); TC = _num(m, "technical_complexity", 0)
        R = _num(m, "risk_if_wrong", 0); X = _num(m, "cross_topic_coupling", 0)
        IR = _num(m, "irreversibility", 0); CF = _num(m, "confidence", 0)
        NCP = _num(m, "node_conflict_pressure", 0)
        AT = _num(sdi, "acceptance_test_pass_rate", 0); DG = _num(sdi, "dependency_gate_pass_rate", 0)
        UR = _num(sdi, "uncertainty_range_width", 0); RT = _num(sdi, "revisit_trigger_count_normalized", 0)
        EC = _num(pl, "evidence_confidence", 0); DQ = _num(pl, "data_quality", 0)
        FR = _num(pl, "failure_rate", 0); DW = _num(pl, "downside_weight", 0)
        PI = _num(pl, "prior_importance", 0); OI = _num(pl, "observed_impact", 0)

        exp = {
            "final_importance": normalize(0.18*C+0.12*BV+0.12*UV+0.14*TC+0.16*R+0.12*X+0.10*IR+0.06*CF),
            "risk_score": normalize(0.32*R+0.24*IR+0.18*X+0.16*TC+0.10*(1-CF)),
            "confidence_score": normalize(0.45*EC+0.25*DQ+0.15*AT+0.15*DG),
            "revisit_pressure": normalize(0.30*(1-CF)+0.25*UR+0.20*RT+0.15*NCP+0.10*FR),
            "lock_score": normalize(0.30*AT+0.25*DG+0.20*CF+0.15*(1-R)+0.10*(1-IR)),
        }
        for k, ev in exp.items():
            av = _num(m, k)
            if av is None or abs(av - ev) > tol:
                errs.append((nid, k, av, round(ev, 2)))
        # posterior
        post = normalize(PI*EC + OI*DQ - FR*DW) if allow_observed else normalize(PI*EC)
        ap = _num(pl, "posterior_importance")
        if ap is None or abs(ap - post) > tol:
            errs.append((nid, "posterior_importance", ap, round(post, 2)))
        # uncertainty_range_width consistency
        ui = pl.get("uncertainty_interval")
        if isinstance(ui, list) and len(ui) == 2 and all(isinstance(x, (int, float)) for x in ui):
            if UR is None or abs(UR - (ui[1]-ui[0])) > tol:
                errs.append((nid, "uncertainty_range_width", UR, round(ui[1]-ui[0], 2)))

    for e in kb.get("edges", []):
        eid = e.get("id")
        RS = _num(e, "relation_strength", 0); ST = _num(e, "signed_tension", 0)
        ERC = _num(e, "expected_rework_cost", 0); CP = _num(e, "conflict_probability", 0)
        CC = _num(e, "causal_confidence", 0)
        ep = normalize(RS*(0.40*abs(ST)+0.30*ERC+0.20*CP+0.10*CC))
        for k, ev in (("edge_priority", ep), ("edge_score", ep),
                      ("edge_heat", normalize(0.50*ep+0.20*RS+0.15*CP+0.15*ERC))):
            av = _num(e, k)
            if av is None or abs(av - ev) > tol:
                errs.append((eid, k, av, round(ev, 2)))

    (r.ok if not errs else r.fail)("formula.consistency", f"mismatches={errs[:25]} total={len(errs)}")


def check_schema_version(kb, r):
    """T13/G6 — pin gate to the KB schema versions it actually implements.

    Both checks are FAIL-level assertions VERIFIED to hold for every existing
    artifact (204/204 *.kb.json in-repo declare "1.3" == "1.3" as of 2026-07-02),
    so no currently-passing KB changes status. They exist to make a future
    kernel/schema bump fail LOUDLY here instead of silently desynchronizing the
    gate from the generator (kernel VERSIONING MODEL: schema_version "must match
    generation_metadata.kb_schema_version")."""
    sv = kb.get("schema_version")
    gm = kb.get("generation_metadata")
    kv = gm.get("kb_schema_version") if isinstance(gm, dict) else None
    (r.ok if sv == kv else r.fail)(
        "version.schema_matches_metadata",
        f"schema_version={sv!r} generation_metadata.kb_schema_version={kv!r}")
    (r.ok if sv in SUPPORTED_KB_SCHEMA_VERSIONS else r.fail)(
        "version.supported",
        f"schema_version={sv!r} supported={sorted(SUPPORTED_KB_SCHEMA_VERSIONS)}"
        " (validator bands/formulas transcribe this KB schema family only)")


def check_source_registry(kb, r):
    """T13/G4 — constrain what a source_registry ENTRY is (warn-level only).

    evidence_refs already must RESOLVE to entries (evidence_refs.resolve); this
    adds the entry-quality layer the fabricated-sources critique named. All
    checks are WARN so no existing pass can flip (verified: zero warns across
    all 144 in-scope KBs; the only repo files that would warn are two _drafts
    that already fail, plus unreferenced-source warns in phase_b whose exit
    code is unaffected). True source EXISTENCE is not deterministically
    checkable offline — that lane is the sampled source-verification protocol
    (PLAN.md section 3d), never this gate."""
    reg = kb.get("source_registry", [])
    entries = [s for s in reg if isinstance(s, dict)]

    # 1) entry shape per kernel "source_registry item structure"
    bad_shape = [(s.get("source_id"), [k for k in SOURCE_ENTRY_FIELDS if k not in s])
                 for s in entries if any(k not in s for k in SOURCE_ENTRY_FIELDS)]
    if not entries and reg:
        bad_shape.append((None, ["<entries are not objects>"]))
    (r.ok if not bad_shape else r.warn)("source_registry.entry_shape",
                                        f"missing_fields={bad_shape[:10]}")

    # 2) external-authority sources must name a locator (heuristic priors may not)
    bad_loc = [s.get("source_id") for s in entries
               if s.get("source_type") not in INTERNAL_SOURCE_TYPES
               and not (isinstance(s.get("citation_or_locator"), str)
                        and s.get("citation_or_locator").strip())]
    (r.ok if not bad_loc else r.warn)("source_registry.external_locator_present",
                                      f"external_source_without_locator={bad_loc}")

    # 3) locator format sanity: only fires when the string CLAIMS a DOI/URL/ISBN shape
    bad_fmt = []
    for s in entries:
        loc = s.get("citation_or_locator")
        if not (isinstance(loc, str) and loc.strip()):
            continue
        loc = loc.strip()
        low = loc.lower()
        if low.startswith(("doi:", "https://doi.org/", "http://doi.org/")):
            d = loc.split("doi.org/")[-1]
            if d.lower().startswith("doi:"):
                d = d[4:].strip()
            if not DOI_RE.match(d):
                bad_fmt.append((s.get("source_id"), "doi", loc[:60]))
        elif low.startswith(("http://", "https://")):
            if not URL_RE.match(loc):
                bad_fmt.append((s.get("source_id"), "url", loc[:60]))
        elif low.startswith("isbn"):
            digits = re.sub(r"[^0-9Xx]", "", loc[4:])
            if len(digits) not in (10, 13):
                bad_fmt.append((s.get("source_id"), "isbn", loc[:60]))
    (r.ok if not bad_fmt else r.warn)("source_registry.locator_format",
                                      f"malformed_identifier={bad_fmt[:10]}")

    # 4) label_eligibility must be a non-empty list (what labels may cite this source)
    bad_elig = [s.get("source_id") for s in entries
                if not (isinstance(s.get("label_eligibility"), list) and s.get("label_eligibility"))]
    (r.ok if not bad_elig else r.warn)("source_registry.label_eligibility_nonempty",
                                       f"bad={bad_elig}")

    # 5) decorative sources: registry entries never cited by any evidence_refs
    cited = set()

    def _collect_refs(obj):
        if isinstance(obj, dict):
            for k, v in obj.items():
                if k == "evidence_refs" and isinstance(v, list):
                    cited.update(x for x in v if isinstance(x, str))
                else:
                    _collect_refs(v)
        elif isinstance(obj, list):
            for v in obj:
                _collect_refs(v)
    _collect_refs(kb)
    unref = [s.get("source_id") for s in entries if s.get("source_id") not in cited]
    (r.ok if not unref else r.warn)("source_registry.unreferenced",
                                    f"never_cited_by_evidence_refs={unref[:10]}")

    # 6) strong labels must be ELIGIBLE under a cited source (groundedness of the
    #    label itself; complements evidence.no_observed_without_data)
    by_id = {s.get("source_id"): s for s in entries}
    bad_strong = []

    def _check_strong(obj, where):
        if isinstance(obj, dict):
            lab = obj.get("evidence_label")
            if isinstance(lab, str) and lab in STRONG_EVIDENCE_LABELS:
                refs = obj.get("evidence_refs") or []
                eligible = any(
                    isinstance(by_id.get(x), dict)
                    and lab in (by_id[x].get("label_eligibility") or [])
                    for x in refs if isinstance(x, str))
                if not eligible:
                    bad_strong.append((where, lab))
            for k, v in obj.items():
                _check_strong(v, f"{where}.{k}")
        elif isinstance(obj, list):
            for i, v in enumerate(obj):
                _check_strong(v, f"{where}[{i}]")
    _check_strong(kb, "$")
    (r.ok if not bad_strong else r.warn)("source_registry.strong_label_source_eligible",
                                         f"strong_label_without_eligible_source={bad_strong[:10]}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("kb", help="path to KB JSON file")
    ap.add_argument("--mode", default="dense", choices=list(DENSITY))
    ap.add_argument("--allow-observed", action="store_true",
                    help="permit observed/experimentally_validated labels + observed posterior formula")
    ap.add_argument("--tolerance", type=float, default=0.02)
    ap.add_argument("--report", help="write JSON report to this path")
    ap.add_argument("--quiet", action="store_true")
    args = ap.parse_args()

    try:
        raw = open(args.kb, encoding="utf-8").read()
    except OSError as e:
        print(f"cannot read {args.kb}: {e}", file=sys.stderr)
        return 2

    r = Report()
    try:
        kb = json.loads(raw)
    except json.JSONDecodeError as e:
        r.fail("json.valid", str(e))
        out = r.to_dict(args.mode)
        if args.report:
            open(args.report, "w").write(json.dumps(out, indent=2))
        if not args.quiet:
            print(json.dumps(out, indent=2))
        return 1
    r.ok("json.valid")

    def _guard(fn, *a):
        # Convert any unexpected crash inside a check (e.g. malformed structure:
        # a string where an object is expected) into a deterministic FAIL rather
        # than an uncaught traceback. Pass behavior for well-formed KBs is unchanged.
        try:
            return fn(*a)
        except Exception as e:
            r.fail("validator.exception", f"{fn.__name__}: {type(e).__name__}: {e}")
            return None

    _guard(check_top_level, kb, r)
    _guard(check_schema_version, kb, r)
    _guard(check_counts, kb, args.mode, r)
    nid_set = _guard(check_ids_and_refs, kb, r) or set()
    _guard(check_bounds_and_obligations, kb, r, args.allow_observed)
    _guard(check_dependency_acyclicity, kb, r, nid_set)
    _guard(check_placeholders, raw, r)
    _guard(check_formulas, kb, r, args.tolerance, args.allow_observed)
    _guard(check_source_registry, kb, r)

    out = r.to_dict(args.mode)
    if args.report:
        open(args.report, "w").write(json.dumps(out, indent=2))
    if not args.quiet:
        print(json.dumps(out, indent=2))
    return 1 if r.failed else 0


if __name__ == "__main__":
    sys.exit(main())
