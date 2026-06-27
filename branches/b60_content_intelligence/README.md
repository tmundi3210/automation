# B60 — Content-Intelligence Engine (heavy, dense-KB scaffold)

Applies the KB→specialist pipeline to the idea *"ingest public info → understand it densely →
link 2–3 related things → generate derivative content (image/story/audio prompts) for an
audience, driven by a brain that tests itself on real examples."*

It is a **scaffold of specialists + a schema + a controller loop** — the thinking brain a system
like this runs on. It does **not** scrape live data, call image/voice models, or publish; it emits
briefs for downstream operators and gates them.

## Read order

| Doc | What it is |
|---|---|
| **`SCOPE.md`** | Phase 1 — the idea typed and run through the 3 existing heavy specialists; the 7 false-presupposition findings; the build plan. |
| **`BRAIN.md`** | Phase 3 — the controller that wires the 5 new + 3 existing specialists into one build→link→signal→generate→gate→test→revise loop. |
| **`RUN.md`** | Phase 3 — a worked end-to-end run of one concrete seed through all 5 specialists (the gate BLOCKs; the evaluator returns NO-GO). |
| **`PANEL.md`** | The whole idea read by a 10-specialist panel (5 new + orch/ir/marketing/reason/psych) — the critical "should you / can you" verdict. |
| **`BUILD.md`** | The same 10-specialist panel re-run **constructively** — the idea built forward into a buildable spec with a v1 MVP + phased roadmap. |
| **`scaffold/`** | The v1 MVP **made to run** — offline, deterministic, stdlib-only code for the whole spine; `run_pipeline.py` ships a clearance-passing brief + returns a leakage-controlled verdict. |
| `schema/information_node.schema.json` | The closed machine-facing record per ingested item (the "schema to think first"). |
| `demo/*_findings.{json,md}` | Raw structured specialist output (scope, run, panel, build-forward). |

## The 5 heavy specialists (each grounded in 3 dense KBs, all gate-passing)

| Specialist | Covers | 3 dense KBs |
|---|---|---|
| `salience_heavy` | map + link | `geo__attention_topology`, `niche__audience_segmentation`, `link__entity_relevance` |
| `signal_heavy` | understand + signal | `auth__inauthenticity_detection`, `hype__organic_baseline`, `brief__rate_distortion_summary` |
| `creative_heavy` | generate | `scene__multi_entity_composition`, `promptgen__downstream_brief`, `novelty__derivative_vs_copy` |
| `eval_heavy` | brain + eval | `leakage__holdout_design`, `metric__frozen_preregistration`, `backtest__outcome_validity` |
| `compliance_heavy` | legal gate (fail-closed) | `pubrights__likeness_voice`, `defamation__false_light`, `platform_tos__synthetic_disclosure` |

## Gates (independently verified)

```
# 15/15 KBs pass dense gate:
for f in branches/b60_content_intelligence/kb/*.kb.json; do python3 validators/kb_validator.py "$f" --mode dense --quiet && echo "PASS $f"; done
# 5/5 specialists pass:
for s in branches/b60_content_intelligence/*.specialist.heavy.json; do python3 validators/specialist_validator.py "$s"; done
```

KBs are built deterministically by `../_forge/kb_forge.py` from compact content specs
(`kb/_src/*.spec.json`, generators in `kb/_forge/`); the builder computes every derived metric so
the gate's formula/reference checks pass by construction. The validated 28-set + `ROUTER.json` are
untouched.

## Caveats

- All KB scores are **heuristic priors** (no observed dataset); `eval_heavy`'s loop is where any
  "it works" claim must be earned.
- `compliance_heavy` is a **fail-closed screening heuristic that escalates to qualified counsel** —
  not legal advice, never an auto-approval.
- Prompt-only ≠ trained: a model reasons *as* a specialist from its spec; nothing here is fine-tuned.
