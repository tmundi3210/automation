#!/usr/bin/env python3
"""
chunk_to_kb.py — chunks (JSONL) -> dense-KB DRAFT -> THE GATE -> work order.

Second stage of the distillation pipeline (IDEA_NEUTRALIZED.md: "agent per chunk ->
dense KB"). Grounded in the 'distill' specialist. Reuses the Track-1 gate VERBATIM:
tools/compute_kb_formulas.py (derived fields) + validators/kb_validator.py (35 checks).

What it does (honestly):
  1. Seeds one work_unit NODE per salient chunk (topic + definition from the chunk),
     with BASE metrics stubs — exactly the inputs a human/LLM author would refine.
  2. Assembles a structurally-valid KB draft (referentially consistent among the
     seeded nodes: edges, CQs, source_registry, evidence_refs all resolve).
  3. Runs the formula tool, then the validator, and REPORTS the true verdict +
     the list of failed checks (the gaps).
  4. Emits the distillation WORK ORDER (prompts/_a4_phaseb_job.md, parameterized)
     that an LLM-authoring pass completes to reach a 35/35 dense pass.

It does NOT fake a passing dense KB: a handful of chunk-seeded nodes is below dense
quantity by design. The deterministic gate decides — same contract as Track 1.
"""
import argparse
import json
import os
import re
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402

TEMPLATE_KB = os.path.join(common.REPO_ROOT, "knowledge_base", "knowledge_searcher",
                           "ir__ranking_and_relevance.kb.json")
JOB_SPEC = os.path.join(common.REPO_ROOT, "prompts", "_a4_phaseb_job.md")


def _slug(text, n=4):
    words = re.findall(r"[a-z0-9]+", text.lower())
    words = [w for w in words if w not in common.STOP and len(w) > 3][:n]
    return "_".join(words) or "topic"


def _base_metrics():
    # Neutral mid-range BASE inputs; the author refines these. Derived fields are
    # filled by tools/compute_kb_formulas.py, never hand-set.
    return {
        "metrics": {"criticality": 0.6, "business_value": 0.6, "user_value": 0.6,
                    "technical_complexity": 0.5, "risk_if_wrong": 0.5,
                    "cross_topic_coupling": 0.4, "irreversibility": 0.3,
                    "confidence": 0.6, "node_conflict_pressure": 0.3},
        "score_derivation_inputs": {"acceptance_test_pass_rate": 0.7,
                                    "dependency_gate_pass_rate": 0.8,
                                    "revisit_triggers": []},
        "probabilistic_layer": {"evidence_confidence": 0.6, "data_quality": 0.6,
                                "failure_rate": 0.2, "downside_weight": 0.5,
                                "prior_importance": 0.6, "observed_impact": 0.0,
                                "uncertainty_interval": [0.4, 0.8]},
    }


def seed_draft(chunks, unit, domain_code, subdomain):
    tmpl = common.load_kb(TEMPLATE_KB)
    nodes, cqs, edges, sources = [], [], [], []
    seen_src = {}
    for i, ch in enumerate(chunks):
        nid = f"N{i+1:02d}_{_slug(ch['text']).upper()}"[:40]
        src_file = ch["source"]
        if src_file not in seen_src:
            sid = f"SRC_{len(seen_src)+1:02d}"
            seen_src[src_file] = sid
            sources.append({"id": sid, "title": os.path.basename(src_file),
                            "type": "ingested_document", "locator": src_file,
                            "evidence_label": "reported"})
        defn = " ".join(ch["text"].split())[:300]
        node = {"id": nid, "topic": _slug(ch["text"], 6), "definition": defn,
                "node_type": "work_unit", "group": "ingested",
                "scope_boundary": {"included": [_slug(ch["text"], 3)], "excluded": []},
                "academic_fields": [domain_code], "subfields": [subdomain],
                "specialists": ["distiller"], "contradictors": [],
                "inputs": ["source chunk"], "outputs": ["KB node"],
                "dependencies": ([f"N{i:02d}"] if False else []),
                "must_not_finalize_before": [],
                "competency_question_refs": [f"CQ_{i+1:02d}"],
                "evidence_refs": [seen_src[src_file]]}
        node.update(_base_metrics())
        nodes.append(node)
        cqs.append({"id": f"CQ_{i+1:02d}",
                    "question": f"What does the source establish about {node['topic']}?",
                    "must_be_answerable_from": ["nodes", "source_registry"],
                    "acceptance_condition": f"{nid} states it with an evidence ref",
                    "covered_by": [nid]})
        if i > 0:
            edges.append({"id": f"EDGE_{i:03d}", "from": nodes[i-1]["id"], "to": nid,
                          "edge_type": "dependency", "relation_strength": 0.6,
                          "signed_tension": 0.0, "expected_rework_cost": 0.3,
                          "conflict_probability": 0.1, "causal_confidence": 0.7})

    draft = dict(tmpl)  # inherit the structural shape (auxiliary section field names)
    draft.update({
        "domain": domain_code,
        "domain_label": f"{domain_code} :: {subdomain} (ingested draft)",
        "purpose": f"DRAFT KB seeded from {len(chunks)} ingested chunks for {unit}; "
                   f"requires LLM-authoring pass to reach dense gate.",
        "nodes": nodes, "edges": edges, "competency_questions": cqs,
        "source_registry": sources,
        # minimal valid-shaped auxiliary stubs (author expands to dense quantity)
        "conflict_axes": [], "edge_cases": [], "workflow": [],
        "dominance_rules": [], "anti_rework_rules": [], "iteration_protocol": [],
        "priority_order": [n["id"] for n in nodes],
    })
    return draft


def run_gate(draft_path, mode):
    subprocess.run([sys.executable, common.FORMULA_TOOL, draft_path, "--inplace"],
                   capture_output=True)
    report_path = os.path.splitext(draft_path)[0] + ".validation.json"
    r = subprocess.run([sys.executable, common.KB_VALIDATOR, draft_path, "--mode", mode,
                        "--report", report_path, "--quiet"], capture_output=True)
    verdict = json.load(open(report_path))
    gaps = [c["check_id"] for c in verdict["checks"] if c["status"] != "pass"]
    return verdict["overall_status"], verdict["summary"], gaps, report_path, r.returncode


def work_order(unit, draft_path, gaps):
    job = ""
    if os.path.exists(JOB_SPEC):
        job = open(JOB_SPEC).read()
    return {
        "unit": unit,
        "draft": os.path.relpath(draft_path, common.REPO_ROOT),
        "gate_gaps_to_close": gaps,
        "authoring_job_spec": os.path.relpath(JOB_SPEC, common.REPO_ROOT) if job else None,
        "instruction": ("Complete the draft to a 35/35 dense pass: author 19-24 distinct "
                        "nodes from the chunks (not 1/chunk), 32-40 edges, and the dense "
                        "auxiliary sections (conflict_axes/edge_cases/workflow/CQs/dominance/"
                        "anti_rework/iteration). Author BASE metrics only; rerun the formula "
                        "tool + validator (the gate already wired here)."),
    }


def main():
    ap = argparse.ArgumentParser(description="Distill chunks into a gated KB draft.")
    ap.add_argument("chunks", nargs="?", help="chunks JSONL (default: run chunker on demo)")
    ap.add_argument("--unit", default="ingest__demo", help="<domain>__<subdomain> unit id")
    ap.add_argument("--mode", default="dense", choices=["compact", "standard", "dense"])
    # NOT under KB_DIR: build_index.py globs *.kb.json and a *.draft.kb.json would match.
    ap.add_argument("--out-dir", default=os.path.join(os.path.dirname(os.path.abspath(__file__)), "_drafts"))
    a = ap.parse_args()

    if not a.chunks:
        # chain from the chunker's demo so this stage is runnable standalone
        from ingest import pdf_to_chunks  # noqa
        scratch = os.path.join(a.out_dir, "_demo_chunks.jsonl")
        pdf_to_chunks.run(pdf_to_chunks.DEMO_SOURCE, scratch, 120, 30)
        a.chunks = scratch

    chunks = [json.loads(l) for l in open(a.chunks) if l.strip()]
    domain_code = a.unit.split("__")[0]
    subdomain = a.unit.split("__", 1)[1] if "__" in a.unit else "draft"
    draft = seed_draft(chunks, a.unit, domain_code, subdomain)

    os.makedirs(a.out_dir, exist_ok=True)
    draft_path = os.path.join(a.out_dir, f"{a.unit}.draft.kb.json")
    json.dump(draft, open(draft_path, "w"), indent=1)

    status, summary, gaps, report_path, _ = run_gate(draft_path, a.mode)
    wo = work_order(a.unit, draft_path, gaps)
    json.dump(wo, open(os.path.splitext(draft_path)[0] + ".workorder.json", "w"), indent=1)

    print(f"unit={a.unit}  chunks={len(chunks)}  nodes={len(draft['nodes'])}")
    print(f"draft : {os.path.relpath(draft_path, common.REPO_ROOT)}")
    print(f"gate  : {status}  ({summary['passed']} pass / {summary['failed']} fail) mode={a.mode}")
    print(f"gaps  : {len(gaps)} checks remain for the authoring pass: {', '.join(gaps[:8])}"
          + (" ..." if len(gaps) > 8 else ""))
    print(f"order : {os.path.relpath(os.path.splitext(draft_path)[0] + '.workorder.json', common.REPO_ROOT)}")
    print("NOTE  : a 3-node draft is below dense quantity BY DESIGN; the gate verdict is "
          "honest. Run the authoring job spec to reach 35/35.")


if __name__ == "__main__":
    main()
