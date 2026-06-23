#!/usr/bin/env python3
"""
dense_to_human.py — decompress a dense machine-facing KB/specialist into readable Markdown.

Grounded in the 'distill' + 'steer' specialists and IDEA_NEUTRALIZED.md ("dense internal
representation everywhere; a rendering/decompression model at the boundary for humans" +
"a final neutral renderer that emits plain, unembellished output").

Boundary (IDEA_NEUTRALIZED §5): this is FORMAT/DENSITY transformation and neutral
rendering only — persona-free, unembellished prose. It is not a guardrail-manipulation
tool. The rendering is fully deterministic (no model needed); a model-backed renderer is
an optional hook for narrative smoothing, never for changing content or safety behavior.

Handles both KB JSON and specialist JSON (auto-detected). Stdlib only.
"""
import argparse
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402


def _is_specialist(d):
    return "specialist_id" in d or "decision_procedure" in d


def _destep(s):
    """Strip a leading enumeration ('1. ', '3) ') so we don't double-number."""
    return re.sub(r"^\s*\d+[.)]\s*", "", str(s)).strip()


def _bullets(items, fmt, limit=None):
    out = []
    for i, it in enumerate(items or []):
        if limit and i >= limit:
            out.append(f"- …(+{len(items) - limit} more)")
            break
        out.append("- " + fmt(it))
    return out


def render_kb(d, neutral=False):
    L = [f"# {d.get('domain_label', d.get('domain', 'KB'))}", "",
         d.get("purpose", ""), ""]
    nodes = d.get("nodes", [])
    L += [f"_{len(nodes)} concepts · {len(d.get('edges', []))} relations · "
          f"{len(d.get('conflict_axes', []))} tensions · "
          f"{len(d.get('competency_questions', []))} competency questions_", ""]

    L += ["## Concepts", ""]
    for n in nodes if not neutral else nodes[:12]:
        L.append(f"### {n.get('topic', n['id'])}")
        L.append(n.get("definition", "").strip())
        L.append("")

    cax = d.get("conflict_axes", [])
    if cax:
        L += ["## Key tensions / trade-offs", ""]
        L += _bullets(cax, lambda c: f"**{c.get('axis', c.get('id',''))}** — "
                      f"{c.get('description', c.get('summary',''))}", limit=None if not neutral else 8)
        L.append("")

    dom = d.get("dominance_rules", [])
    if dom:
        L += ["## Decision rules (what wins when)", ""]
        L += _bullets(dom, lambda r: r.get("rule", r.get("statement", str(r)))
                      if isinstance(r, dict) else str(r), limit=None if not neutral else 8)
        L.append("")

    wf = d.get("workflow", [])
    if wf:
        L += ["## Workflow", ""]
        for i, step in enumerate(wf, 1):
            s = step.get("step", step.get("description", step)) if isinstance(step, dict) else step
            L.append(f"{i}. {_destep(s)}")
        L.append("")

    cqs = d.get("competency_questions", [])
    if cqs and not neutral:
        L += ["## Questions this KB can answer", ""]
        L += _bullets(cqs, lambda q: q.get("question", str(q)))
        L.append("")
    return "\n".join(L).rstrip() + "\n"


def render_specialist(d, neutral=False):
    L = [f"# {d.get('domain_label', d.get('specialist_id', 'Specialist'))}", "",
         d.get("purpose", ""), ""]
    if d.get("role"):
        L += ["## Role", d["role"].strip(), ""]
    caps = d.get("capabilities", [])
    if caps:
        L += ["## Capabilities", ""]
        L += _bullets(caps, lambda c: c.get("capability") or c.get("method", str(c))
                      if isinstance(c, dict) else str(c), limit=None if not neutral else 10)
        L.append("")
    dp = d.get("decision_procedure", [])
    if dp:
        L += ["## Decision procedure", ""]
        for i, step in enumerate(dp, 1):
            s = step if isinstance(step, str) else step.get("step", step)
            L.append(f"{i}. {_destep(s)}")
        L.append("")
    esc = d.get("escalation_triggers", [])
    if esc:
        L += ["## Escalate to a human when", ""]
        L += _bullets(esc, lambda e: e if isinstance(e, str) else e.get("trigger", str(e)),
                      limit=None if not neutral else 8)
        L.append("")
    return "\n".join(L).rstrip() + "\n"


def render(path, neutral=False):
    d = common.load_kb(path) if path.endswith(".kb.json") else json.load(open(path))
    return render_specialist(d, neutral) if _is_specialist(d) else render_kb(d, neutral)


def main():
    ap = argparse.ArgumentParser(description="Render a dense KB/specialist as human Markdown.")
    ap.add_argument("path", nargs="?",
                    default=os.path.join(common.SPECIALISTS_DIR, "ftune.specialist.json"))
    ap.add_argument("--neutral", action="store_true",
                    help="plainest, length-bounded rendering (the neutral boundary renderer)")
    ap.add_argument("--out", help="write Markdown here instead of stdout")
    a = ap.parse_args()
    md = render(a.path, a.neutral)
    if a.out:
        open(a.out, "w").write(md)
        print(f"wrote {os.path.relpath(a.out, common.REPO_ROOT)} ({common.est_tokens(md)} tok)")
    else:
        sys.stdout.write(md)


if __name__ == "__main__":
    main()
