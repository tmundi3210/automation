#!/usr/bin/env python3
"""
common.py — shared foundation for the Phase B Track-2 scaffold.

Loads the committed Track-1 artifacts (ROUTER.json, INDEX.json, the per-domain
specialists, and the dense KBs) and exposes the small primitives every subsystem
needs: repo-relative paths, JSON loaders, a token estimator, and the routing
tokenizer that MIRRORS tools/build_router.py exactly (so scaffold routing scores
match the way ROUTER.json's route_when sets were built).

Stdlib only. Importable from any scaffold subsystem via:
    from phase_b.scaffold import common      # if run as a package
or  import common                            # if cwd is phase_b/scaffold
The path resolution below works regardless of how it is imported or invoked.
"""
import json
import os
import re
import glob
from functools import lru_cache

# --- repo layout ----------------------------------------------------------
# scaffold/ -> phase_b/ -> repo root
_THIS = os.path.dirname(os.path.abspath(__file__))
PHASE_B = os.path.dirname(_THIS)
REPO_ROOT = os.path.dirname(PHASE_B)

SPECIALISTS_DIR = os.path.join(PHASE_B, "specialists")
ROUTER_PATH = os.path.join(SPECIALISTS_DIR, "ROUTER.json")
KB_DIR = os.path.join(PHASE_B, "knowledge_base", "llm_engineering")
INDEX_PATH = os.path.join(KB_DIR, "INDEX.json")
SOURCES_DIR = os.path.join(PHASE_B, "sources")

# gate tools (reused verbatim by the ingest pipeline)
FORMULA_TOOL = os.path.join(REPO_ROOT, "tools", "compute_kb_formulas.py")
KB_VALIDATOR = os.path.join(REPO_ROOT, "validators", "kb_validator.py")

DIRECTIVE = ("losslessly compressed, token-efficient, information-dense, fully detailed, "
             "machine-facing; optimized for model parsing over human readability")

# --- routing tokenizer (MIRROR of tools/build_router.py kw()/STOP) --------
# Kept byte-identical so scaffold scoring reproduces how route_when was built.
STOP = {'of', 'the', 'and', 'for', 'in', 'to', 'a', 'with', 'by', 'on', 'from', 'via', 'using',
        'based', 'how', 'what', 'when', 'which', 'that', 'this', 'are', 'is', 'an', 'or', 'as',
        'at', 'its', 'it', 'into', 'data', 'analysis', 'method', 'methods', 'model', 'models',
        'knowledge', 'research', 'specialist', 'across', 'given', 'need', 'use', 'should', 'can',
        'will', 'between', 'within', 'their', 'than', 'more'}


def kw(s):
    """Tokenize text into routing keywords (len>3, no stopwords). Mirrors build_router.kw."""
    return [w for w in re.split(r'[^a-z0-9]+', (s or '').lower()) if w and w not in STOP and len(w) > 3]


# --- token estimation -----------------------------------------------------
# Matches the heuristic used by validators/metrics.py (chars/4) so token-budget
# accounting in the orchestrator agrees with the KB metrics sidecars.
def est_tokens(text):
    if not isinstance(text, str):
        text = json.dumps(text, ensure_ascii=False)
    return max(1, len(text) // 4)


# --- loaders --------------------------------------------------------------
@lru_cache(maxsize=1)
def load_router():
    with open(ROUTER_PATH) as f:
        return json.load(f)


@lru_cache(maxsize=1)
def load_index():
    with open(INDEX_PATH) as f:
        return json.load(f)


@lru_cache(maxsize=None)
def load_specialist(specialist_id):
    path = os.path.join(SPECIALISTS_DIR, f"{specialist_id}.specialist.json")
    with open(path) as f:
        return json.load(f)


def specialist_ids():
    return [s["specialist_id"] for s in load_router()["specialists"]]


def load_kb(path_or_unit):
    """Load a KB by full path or by INDEX unit id (e.g. 'ftune__sft_peft')."""
    if os.path.exists(path_or_unit):
        path = path_or_unit
    else:
        path = os.path.join(KB_DIR, f"{path_or_unit}.kb.json")
    with open(path) as f:
        return json.load(f)


def iter_kb_paths():
    return sorted(p for p in glob.glob(os.path.join(KB_DIR, "*.kb.json")))


def kbs_for_domain(domain_code):
    """KB units belonging to a domain, per INDEX.json (e.g. domain 'ftune')."""
    return [k for k in load_index()["kbs"] if k["domain_code"] == domain_code]


if __name__ == "__main__":  # smoke test against the committed artifacts
    r = load_router()
    idx = load_index()
    print(f"router: {r['specialist_count']} specialists -> {', '.join(specialist_ids())}")
    print(f"index : {idx['kb_count']} KBs, {idx['totals']['nodes']} nodes, "
          f"{idx['totals']['edges']} edges, ~{idx['totals']['est_tokens']:,} tok")
    print(f"paths : KB_DIR={os.path.relpath(KB_DIR, REPO_ROOT)} | "
          f"gate={os.path.relpath(KB_VALIDATOR, REPO_ROOT)}")
