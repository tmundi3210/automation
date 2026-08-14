#!/usr/bin/env python3
"""
router.py — need -> specialist(s), faithful to ROUTER.json's selection_procedure.

Grounded in: phase_b/specialists/ROUTER.json (built by tools/build_router.py) and the
'orch' domain specialist. Reproduces the documented scoring EXACTLY:

  1. Normalize the need into domain_signal_terms (common.kw, mirrors build_router).
  2. Score each specialist = count of need terms in its route_when set, with
     domain_label term matches weighted x2 (a need term that is also a domain_label
     term counts once for route_when membership + once more for the label).
  3. Select the top scorer; if a second is within 1 point, invoke both and merge.
  4. (Caller) applies the selected specialist's decision_procedure + workflow.
  5. If nothing scores > 0, fall back per the router's fallback_policy.

Pure scoring lives here; the DAG that executes the chosen specialists is graph.py.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402


def _route_when(spec):
    return set(spec.get("route_when", []))


def _label_terms(spec):
    return set(common.kw(spec.get("domain_label", "")))


def score_specialists(need, router=None):
    """Return [(specialist_id, score, matched_terms, purpose_overlap)] sorted high->low.

    `score` is the faithful primary metric (route_when membership + domain_label x2).
    `purpose_overlap` (need terms also appearing in the specialist's purpose text, which
    build_router does NOT fold into route_when) is a SECONDARY tie-break only — it
    reorders equal-primary candidates by relevance without altering the documented
    primary scoring."""
    router = router or common.load_router()
    terms = list(dict.fromkeys(common.kw(need)))  # unique, order-preserving:
    # a word repeated in the query must not dominate scoring (route_when is itself deduped).
    rows = []
    for spec in router["specialists"]:
        rw, lab = _route_when(spec), _label_terms(spec)
        purpose_terms = set(common.kw(spec.get("purpose", "")))
        matched, score = [], 0
        for t in terms:
            hit = False
            if t in rw:
                score += 1
                hit = True
            if t in lab:  # domain_label match => the documented x2 weighting
                score += 1
                hit = True
            if hit:
                matched.append(t)
        p_overlap = sum(1 for t in terms if t in purpose_terms)
        rows.append((spec["specialist_id"], score, matched, p_overlap))
    rows.sort(key=lambda r: (-r[1], -r[3], r[0]))  # primary score, then purpose tie-break
    return rows


def route(need, router=None, within=1, max_multi=3):
    """Routing decision matching ROUTER.json selection_procedure.

    Returns dict: selected (list of specialist_ids), scores (full table), mode
    ('single' | 'multi' | 'fallback'), and rationale.

    `within` is the point band for merging a close second (selection_procedure
    step 3); `max_multi` caps the fan-out so an ambiguous bag-of-words tie composes
    a handful of leaders rather than the whole table (the multi_specialist_policy
    then reconciles them)."""
    router = router or common.load_router()
    rows = score_specialists(need, router)
    top_id, top_score = rows[0][0], rows[0][1]

    if top_score == 0:
        return {
            "need": need,
            "selected": [],
            "mode": "fallback",
            "fallback_policy": router.get("fallback_policy"),
            "scores": rows,
            "rationale": "no specialist scored > 0; defer to fallback_policy",
        }

    band = [r[0] for r in rows if r[1] > 0 and top_score - r[1] <= within]
    selected = band[:max_multi]
    mode = "single" if len(selected) == 1 else "multi"
    return {
        "need": need,
        "selected": selected,
        "mode": mode,
        "multi_specialist_policy": router.get("multi_specialist_policy") if mode == "multi" else None,
        "scores": rows,
        "rationale": f"top={top_id}({top_score}); merged within {within} pt: {selected}",
    }


def _fmt(decision, topn=5):
    lines = [f"need: {decision['need']}",
             f"mode: {decision['mode']}  selected: {decision['selected'] or '(fallback)'}"]
    lines.append("scores (primary | purpose-tiebreak):")
    for sid, sc, matched, pov in decision["scores"][:topn]:
        lines.append(f"  {sid:8s} {sc:>3d} | {pov:>2d}  {' '.join(matched[:8])}")
    if decision["mode"] == "fallback":
        lines.append(f"fallback_policy: {decision['fallback_policy']}")
    return "\n".join(lines)


if __name__ == "__main__":
    needs = sys.argv[1:] or [
        "fine-tune a small model with LoRA and roll back if eval score stalls",
        "compose many specialist outputs into a weighted any-to-any graph with a summarizer",
        "pull real benchmark questions and judge answers with an LLM judge",
        "compute a probability exactly with Python and a strict numeric format",
        "pick a base model under a permissive license with long context and tool support",
        "break a PDF book into chunks and distill each into a dense JSON knowledge base",
    ]
    for n in needs:
        print(_fmt(route(n)))
        print("-" * 72)
