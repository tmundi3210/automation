# FACTORY — build your own specialist, the same way this repo does

A self-contained kit to author, gate, and use **specialists** + **dense knowledge bases**
exactly like the ones already in this repository. Everything is **Python stdlib only** — no
pip install, no GPU, no keys, no network at build time.

## Start here

| File | What it is |
|---|---|
| **`MAKE_A_SPECIALIST.md`** | The full manual — the whole pipeline, both author paths, the gates, and how to *use* a finished specialist. **Read this first.** |
| `setup_local.sh` | Clone the repo and self-test the factory on your machine in one command. |
| `GENERATOR_PROMPT.md` | A paste-into-any-AI prompt that mints a new specialist + its 3 KBs in this repo's schema. |
| `specialist.template.json` | Blank specialist spec (15 required fields) to copy and fill. |
| `kb_spec.template.json` | Blank compact KB content-spec for the deterministic forge. |

## The 30-second version

```
3 dense KBs  ──distill──►  1 specialist spec  ──fill prompt template──►  a model acts as the expert
```

1. **Get the repo & verify:** `bash FACTORY/setup_local.sh --clone`
2. **Author 3 KBs** — either forge them deterministically
   (`cp FACTORY/kb_spec.template.json …` → `python3 branches/_forge/kb_forge.py … -o ….kb.json`)
   or have an AI write them (`FACTORY/GENERATOR_PROMPT.md`).
3. **Gate them:** `python3 validators/kb_validator.py KB.json --mode dense`
4. **Distill the specialist** (`cp FACTORY/specialist.template.json …`) and gate it:
   `python3 validators/specialist_validator.py SPEC.json`
5. **Use it:** drop the specialist JSON into `dist/prompt_template.json` and send to any model.
6. **(Optional) Wire many into a brain** — see `branches/b60_content_intelligence/BRAIN.md`
   and the `BRAIN_STEP1.md` + `GATE_STEP2.md` think/decide split.

All commands are confirmed working on Python 3.11; the same code is stdlib-only back to 3.8.
See `MAKE_A_SPECIALIST.md` §10 for the full cheat sheet.
