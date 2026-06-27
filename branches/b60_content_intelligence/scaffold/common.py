#!/usr/bin/env python3
"""
common.py — shared foundation for the B60 content-intelligence v1 MVP scaffold.

Loads the committed b60 artifacts (the information_node schema + the five heavy
specialists) and exposes the small primitives every stage needs: repo-relative
paths, JSON IO, a token estimator, a specialist loader, and a compact, stdlib-only
JSON-Schema validator (`validate_node`) that gates the completed InformationNode
before emit (the "schema-parse check before emit" the BUILD spec requires).

The validator implements the subset of JSON-Schema 2020-12 the node schema uses
(type / required / properties / additionalProperties:false / enum / const /
minimum / maximum / minItems / minLength / pattern / items). No third-party deps,
so the scaffold runs CPU-only and offline.

Stdlib only. Importable from any stage via `import common` (the path bootstrap in
each stage inserts the scaffold dir on sys.path), regardless of invocation cwd.
"""
import json
import os
import re
from functools import lru_cache

# --- repo layout ----------------------------------------------------------
# scaffold/ -> b60_content_intelligence/ -> branches/ -> repo root
SCAFFOLD_DIR = os.path.dirname(os.path.abspath(__file__))
B60_DIR = os.path.dirname(SCAFFOLD_DIR)
BRANCHES_DIR = os.path.dirname(B60_DIR)
REPO_ROOT = os.path.dirname(BRANCHES_DIR)

SCHEMA_PATH = os.path.join(B60_DIR, "schema", "information_node.schema.json")
OUTBOX_DIR = os.path.join(SCAFFOLD_DIR, "orch", "_outbox")

# The five heavy specialists each stage is grounded in (provenance stamp).
SPECIALIST_BY_STAGE = {
    "salience": "salience_heavy",
    "link": "salience_heavy",
    "signal": "signal_heavy",
    "dense_brief": "signal_heavy",
    "creative": "creative_heavy",
    "compliance": "compliance_heavy",
    "eval": "eval_heavy",
}


# --- io -------------------------------------------------------------------
def read_json(path):
    with open(path) as f:
        return json.load(f)


def write_json(path, obj):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        json.dump(obj, f, ensure_ascii=False, indent=2, sort_keys=True)
    return path


def read_jsonl(path):
    return [json.loads(line) for line in open(path) if line.strip()]


@lru_cache(maxsize=1)
def load_schema():
    return read_json(SCHEMA_PATH)


@lru_cache(maxsize=None)
def load_specialist(specialist_id):
    """Load a b60 heavy specialist by id (e.g. 'salience_heavy')."""
    path = os.path.join(B60_DIR, f"{specialist_id}.specialist.heavy.json")
    return read_json(path)


def grounded_in(stage):
    """The specialist id a pipeline stage is grounded in, or None."""
    return SPECIALIST_BY_STAGE.get(stage)


# --- token estimation -----------------------------------------------------
# chars/4, matching validators/metrics.py so dense-brief budget accounting agrees
# with the rest of the repo's token bookkeeping.
def est_tokens(text):
    if not isinstance(text, str):
        text = json.dumps(text, ensure_ascii=False)
    return max(1, len(text) // 4)


# --- minimal JSON-Schema validator (stdlib) -------------------------------
class SchemaError(Exception):
    pass


def _vtype(value):
    # JSON-Schema type name for a Python value (bool before int — bool is an int).
    if isinstance(value, bool):
        return "boolean"
    if isinstance(value, str):
        return "string"
    if isinstance(value, int):
        return "integer"
    if isinstance(value, float):
        return "number"
    if isinstance(value, list):
        return "array"
    if isinstance(value, dict):
        return "object"
    if value is None:
        return "null"
    return "unknown"


def _type_ok(value, expected):
    vt = _vtype(value)
    if expected == "number":
        return vt in ("number", "integer")
    return vt == expected


def _validate(node, schema, path, errors):
    # const / enum
    if "const" in schema and node != schema["const"]:
        errors.append(f"{path}: expected const {schema['const']!r}, got {node!r}")
        return
    if "enum" in schema and node not in schema["enum"]:
        errors.append(f"{path}: {node!r} not in enum {schema['enum']}")

    t = schema.get("type")
    if t and not _type_ok(node, t):
        errors.append(f"{path}: expected type {t}, got {_vtype(node)} ({node!r})")
        return  # type mismatch makes deeper checks meaningless

    if t == "object" or isinstance(node, dict):
        props = schema.get("properties", {})
        for req in schema.get("required", []):
            if req not in node:
                errors.append(f"{path}: missing required key '{req}'")
        if schema.get("additionalProperties") is False:
            for key in node:
                if key not in props:
                    errors.append(f"{path}: unexpected key '{key}' (additionalProperties:false)")
        for key, val in node.items():
            if key in props:
                _validate(val, props[key], f"{path}.{key}", errors)

    if t == "array" or isinstance(node, list):
        if "minItems" in schema and len(node) < schema["minItems"]:
            errors.append(f"{path}: array len {len(node)} < minItems {schema['minItems']}")
        item_schema = schema.get("items")
        if item_schema:
            for i, item in enumerate(node):
                _validate(item, item_schema, f"{path}[{i}]", errors)

    if isinstance(node, str):
        if "minLength" in schema and len(node) < schema["minLength"]:
            errors.append(f"{path}: string len {len(node)} < minLength {schema['minLength']}")
        pat = schema.get("pattern")
        if pat and not re.search(pat, node):
            errors.append(f"{path}: {node!r} does not match pattern /{pat}/")

    if _vtype(node) in ("number", "integer"):
        if "minimum" in schema and node < schema["minimum"]:
            errors.append(f"{path}: {node} < minimum {schema['minimum']}")
        if "maximum" in schema and node > schema["maximum"]:
            errors.append(f"{path}: {node} > maximum {schema['maximum']}")


def validate_node(node, schema=None):
    """Validate an InformationNode dict against the b60 schema.
    Returns [] on success, else a list of human-readable error strings."""
    schema = schema or load_schema()
    errors = []
    _validate(node, schema, "node", errors)
    return errors


if __name__ == "__main__":  # smoke test against committed artifacts
    sch = load_schema()
    print(f"schema : {sch['title']} v{sch['properties']['schema_version']['const']} "
          f"({len(sch['required'])} required blocks)")
    specs = sorted(set(SPECIALIST_BY_STAGE.values()))
    for sid in specs:
        s = load_specialist(sid)
        print(f"spec   : {sid:16s} <- {s.get('domain_label', s.get('domain', '?'))}")
    print(f"paths  : schema={os.path.relpath(SCHEMA_PATH, REPO_ROOT)}")
    # validator self-check: empty dict must report all missing required blocks.
    errs = validate_node({})
    print(f"validator self-check: empty node -> {len(errs)} errors (expected >= {len(sch['required'])})")
