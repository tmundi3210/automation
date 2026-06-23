#!/usr/bin/env python3
"""
human_to_dense.py — compress human notes into a dense, machine-facing skeleton.

The inverse of dense_to_human.py. Grounded in the 'distill' + 'steer' specialists and
IDEA_NEUTRALIZED.md ("machine-facing dense output only ... reduce conversion loss").

Boundary (IDEA_NEUTRALIZED §5): this performs DENSITY/FORMAT compression — terse,
persona-free, token-efficient representation — NOT removal of any safety behavior. The
deterministic baseline below strips human-oriented filler and emits a compact claim/term
skeleton carrying the project _directive. A model-backed pass (hook) can compress
further but must preserve meaning and safety, never strip guardrails.

Stdlib only. Reports the compression ratio (the "conversion loss" reduction).
"""
import argparse
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402

FILLER = {"basically", "really", "very", "just", "actually", "simply", "kind", "sort",
          "maybe", "perhaps", "obviously", "essentially", "literally", "definitely",
          "please", "thanks", "note", "okay", "well", "anyway", "however", "moreover",
          "think", "want", "know", "make", "sure", "honestly", "stuff", "thing", "things",
          "also", "like", "guess", "kinda", "gonna", "wanna", "lot", "lots"}
# glue/stopwords to strip from the dense payload (machine-facing => order kept, glue dropped)
GLUE = FILLER | common.STOP | {"i", "we", "you", "to", "of", "a", "the", "so", "is", "are",
                               "be", "all", "there", "too", "without", "because", "when"}


def _claims(text):
    """Split into claim units (sentences / bullet lines); drop empties + pure filler."""
    units = re.split(r"(?<=[.!?])\s+|\n+", text)
    out = []
    for u in units:
        u = re.sub(r"^\s*[-*•\d.)]+\s*", "", u).strip()
        words = [w for w in re.findall(r"[A-Za-z0-9']+", u) if w.lower() not in FILLER]
        if len(words) >= 2:
            out.append(" ".join(words))
    return out


def _terse(claim):
    """Content-word skeleton of a claim: drop glue/filler, keep order (machine-facing)."""
    return " ".join(w for w in re.findall(r"[A-Za-z0-9']+", claim) if w.lower() not in GLUE)


def _terms(text):
    seen, terms = set(), []
    for w in common.kw(text):  # salient (len>3, stopword-free) ...
        if w not in seen and w not in GLUE:  # ... and not glue/filler
            seen.add(w)
            terms.append(w)
    return terms


def compress(text, backend=None):
    claims_terse = [c for c in (_terse(c) for c in _claims(text)) if c]
    terms = _terms(text)[:12]
    payload = "\n".join(claims_terse)
    before, after = common.est_tokens(text), common.est_tokens(payload)
    dense = {
        "_directive": common.DIRECTIVE,
        "claims": claims_terse,           # the dense, persona-free payload
        "terms": terms,                   # small salient index
        "tokens_before": before,
        "tokens_after": after,            # measured on the payload, not JSON boilerplate
        "compression_ratio": round(after / max(1, before), 3),
    }
    if backend is not None:  # optional model-backed tightening (hook); preserves meaning
        dense["model_compressed"] = backend.generate(
            f"Rewrite as dense, persona-free, machine-facing notes (no safety changes): {payload}",
            max_tokens=max(16, after))
    return dense


_SAMPLE = ("Okay so basically I think we really just want the models to talk to each "
           "other very tersely, you know, without all the personality fluff, because "
           "honestly that wastes tokens. Also please make sure the summarizer kicks in "
           "when there are too many outputs. Thanks!")


def main():
    ap = argparse.ArgumentParser(description="Compress human notes to a dense skeleton.")
    ap.add_argument("path", nargs="?", help="text/markdown file (default: built-in sample)")
    ap.add_argument("--model", action="store_true", help="also run the optional model-backed pass (MockBackend)")
    ap.add_argument("--out", help="write dense JSON here")
    a = ap.parse_args()
    text = open(a.path).read() if a.path else _SAMPLE
    backend = None
    if a.model:
        import backends
        backend = backends.get_backend("mock")
    dense = compress(text, backend)
    js = json.dumps(dense, indent=1, ensure_ascii=False)
    if a.out:
        open(a.out, "w").write(js)
        print(f"wrote {os.path.relpath(a.out, common.REPO_ROOT)}")
    else:
        print(js)
    print(f"\ncompression: {dense['tokens_before']} -> {dense['tokens_after']} tok "
          f"(ratio {dense['compression_ratio']}; lower = denser)", file=sys.stderr)


if __name__ == "__main__":
    main()
