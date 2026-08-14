# Project Handoff — Orchestrated Fine-Tuned Specialist System

> **Status: PARKED / stopped at a clean stopping point (2026-06-23).**
> All planned in-repo work is complete, committed, and pushed. Remaining work is **external**
> (your data, rented GPUs, live model facts). This document is the cold-start guide: read it
> top to bottom and you can resume without prior context.

---

## 1. What this project is (and whose it is)

This is **your original project** — custom IP, not a fork of any off-the-shelf agent framework.
It is a **knowledge architecture + runnable code scaffold** for engineering an **orchestrated
system of fine-tuned, narrow specialist LLMs** ("thinking, not study"): many deep specialists,
composed by a non-linear router + weighted graph, fed by a dense machine-facing knowledge
pipeline, trained with a monitored/rollback loop, grounded by tools for exact work.

The thesis, router, specialists, and orchestration graph are bespoke and grounded in your own
neutralized idea brief (`phase_b/IDEA_NEUTRALIZED.md`), not in priors.

---

## 2. Architecture: two phases, two tracks

| Layer | Where | What it is |
|---|---|---|
| **Phase A — Knowledge Searcher** | repo root: `knowledge_base/`, `specialists/`, `tools/`, `validators/`, `prompts/`, `schema/` | The reusable engine: source material → **dense gated KBs** → **domain specialists** → **INDEX + ROUTER**. 24 KBs, **8 specialists** (`biblio, evsynth, infosci, ir, kr, libarch, method, scholcomm`). |
| **Phase B — the idea applied** | `phase_b/` | The orchestrated-specialist system, in two tracks. |
| ↳ **Track 1 — research** | `phase_b/knowledge_base/`, `phase_b/specialists/` | 25 dense KBs (gated 35/35), **12 specialists** across the build domains, + ROUTER + INDEX. **COMPLETE.** |
| ↳ **Track 2 — runnable scaffold** | `phase_b/scaffold/` | Offline-runnable, GPU-portable code: ingest → distill → train-with-rollback → orchestrate → eval → render. Mock backend; GPU/API paths are explicit hook points. **COMPLETE — `run_all.sh` = 12/12 pass.** |

**The 12 Phase-B domains:** `ftune` (fine-tuning), `orch` (orchestration), `eval`, `distill`,
`reason`, `select` (base-model selection), `tool`, `intent` (front-end), `steer`, `infra`,
`math`, `code`.

---

## 3. Current state — what is done

- [x] Idea received, **neutralized**, intent-mapped, boundaries set (`phase_b/IDEA_NEUTRALIZED.md`).
- [x] Seed taxonomy: 12 domains / 25 subdomains (`phase_b/taxonomy/llm_engineering.taxonomy.json`).
- [x] **Track 1 COMPLETE** — 25 dense KBs gated, 12 specialists, INDEX + ROUTER.
- [x] **Track 2 COMPLETE** — 6 subsystems grounded in their specialists; 12/12 offline smoke pass.
- [x] **Architecture Decision Record** — build/buy, proven/speculative, vertical-slice-first
      sequencing (`phase_b/scaffold/ARCHITECTURE_DECISION_RECORD.md`).
- [x] **Prototype Brief** — `method` neutralize/scope + `libarch`/`ir` sourcing plan for the
      first prototype (`phase_b/PROTOTYPE_BRIEF.md`).
- [ ] **External / not done** — see §8.

---

## 4. Repository map

```
automation/
├── README.md, PLAN.md              # Phase-A overview + master plan
├── docs/HANDOFF.md                 # ← this file
├── schema/                         # KB generator spec (kb_generator_v1.4.1)
├── prompts/                        # A.4 subdomain + A.5 specialist job prompts
├── tools/                          # build_index.py, build_router.py, compute_kb_formulas.py
├── validators/                     # kb_validator.py (the 35-check dense gate)
├── knowledge_base/knowledge_searcher/   # Phase-A dense KBs (24)
├── specialists/                    # Phase-A specialists (8) + ROUTER.json
├── orchestrator/, calibration/, taxonomy/
└── phase_b/
    ├── IDEA_NEUTRALIZED.md         # the brief + boundaries (READ FIRST)
    ├── PLAN_B.md                   # Phase-B execution plan + state
    ├── PROTOTYPE_BRIEF.md          # how to start the first prototype
    ├── taxonomy/                   # 12 domains / 25 subdomains
    ├── knowledge_base/llm_engineering/   # Track-1 dense KBs (25) + INDEX.json
    ├── specialists/                # Track-1 specialists (12) + ROUTER.json
    ├── sources/                    # EMPTY — drop WCO transcripts / book PDFs here
    ├── ingest/                     # (Phase-A gate reuse target)
    └── scaffold/                   # Track-2 runnable code (see below)
        ├── README.md, ARCHITECTURE_DECISION_RECORD.md, run_all.sh
        ├── common.py, backends.py  # loaders + pluggable ModelBackend (Mock/Api/Local)
        ├── ingest/                 # pdf_to_chunks.py, chunk_to_kb.py (+ the gate)
        ├── train/                  # rollback_loop.py (eval-in-loop, rollback, prune)
        ├── orch/                   # router.py (faithful to ROUTER), graph.py (weighted DAG)
        ├── eval_harness/           # harness.py (scorers, LLM-judge, contamination, tracking)
        ├── select/                 # select.py + model_sheet.json (verify-before-use template)
        └── render/                 # dense_to_human.py, human_to_dense.py
```

---

## 5. How to run it (offline, no GPU, no keys, no cost)

```bash
cd automation
bash phase_b/scaffold/run_all.sh          # smoke-tests all 6 subsystems → expect 12/12 PASS
```
Individual tours:
```bash
python3 phase_b/scaffold/common.py                                    # loads KBs + specialists
python3 phase_b/scaffold/orch/router.py                               # routing table
python3 phase_b/scaffold/orch/graph.py "fine-tune with LoRA, roll back on stall"
python3 phase_b/scaffold/train/rollback_loop.py
python3 phase_b/scaffold/eval_harness/harness.py --system oracle
python3 phase_b/scaffold/select/select.py --map                       # base model per domain
python3 phase_b/scaffold/render/dense_to_human.py phase_b/specialists/orch.specialist.json
```
Stdlib-only, Python 3.11. The distiller demo honestly reports `fail (24/35)` by design — a
3-node draft is below dense quantity and emits the authoring work order.

---

## 6. Key documents — read in this order

1. `phase_b/IDEA_NEUTRALIZED.md` — the brief, intent map, **and the two boundaries (§5)**.
2. `phase_b/PLAN_B.md` — the two-track plan + state.
3. `phase_b/scaffold/ARCHITECTURE_DECISION_RECORD.md` — what to build/buy, proven vs speculative.
4. `phase_b/scaffold/README.md` — the system loop + "real vs hook (honesty)".
5. `phase_b/PROTOTYPE_BRIEF.md` — the concrete plan to start prototype #1.

---

## 7. Git state

- **Branch:** `claude/eager-wozniak-74rlgj`
- **HEAD:** `5bd5f75` (prototype brief)
- **PR:** #1 (draft) tracks this branch — pushed and current.
- Push convention: `git push -u origin claude/eager-wozniak-74rlgj` with backoff retry.

---

## 8. What is NOT done — external dependencies & deferred work

**External blockers (cannot be done in-container):**
- **Source data** — drop WCO AI↔AI transcripts / book PDFs into `phase_b/sources/` (currently
  empty), then point `ingest/pdf_to_chunks.py` at them. *(The WCO corpus lives on your Mac /
  iCloud — not reachable from here.)*
- **GPU compute** — rent GPUs to run the real `HFTrainerBackend`/`LocalBackend` hooks (this repo
  is the portable scaffold + configs, not a trainer).
- **Live model facts** — refresh `select/model_sheet.json` from current model cards/leaderboards
  before any base-model decision (figures go stale monthly).

**Deferred in-repo options (designed, not built):**
- Wire the `math`/`code` **vertical slice** end-to-end (the recommended resume — see §9).
- Upgrade routing from bag-of-words to a learned classifier (when task types multiply).
- Implement the LLM-authoring pass that turns an ingest draft into a 35/35 dense KB.
- Mark PR #1 ready for review.

---

## 9. Recommended resume path (when you return)

Per the ADR and the Prototype Brief, **do the vertical slice first**:

1. Pick one **correctness-checkable** domain (`math` or `code`).
2. End-to-end on a **base model + prompt (no fine-tune yet)**: ingest a few sources → `select`
   base → `orch` route → tool-grounded answer → `eval` vs a held-out checkable set → `render`.
3. **Pre-register** the gates from `PROTOTYPE_BRIEF.md`: G1 (loop runs, 0 crashes on ~20 tasks)
   and **G2 (the `SPEC_BEAT` test:** slice `pass@1` beats a single-generalist baseline by ≥10 pts,
   net of overhead, on ~30 contamination-checked tasks).
4. **Gate breadth on G2.** Pass → add 2–3 specialists + the real ablation. Fail → pivot or stop.

Build the prototype in **Python**; Mac/iPhone/Android come later as a thin client over the same
backend (the specialist set does not change with platform).

First live step available now: source base-model facts via the Hugging Face Hub tools and
populate `select/model_sheet.json` with cited, dated figures.

---

## 10. Boundaries (must be preserved — non-negotiable)

1. **No guardrail-removal pipeline.** `steer`/`render` = terse, persona-free, token-efficient
   format/density transformation + neutral rendering ONLY. The legitimate efficiency goal is
   fully served without stripping safety. (`IDEA_NEUTRALIZED.md §5`.)
2. **Cognition modelling = adaptive UX, not clinical diagnosis.** The `intent` front-end uses
   probabilistic signals to ask better questions — never medical/IQ diagnosis.

Both are enforced in the scaffold code, not just stated here.

---

## 11. Glossary of moving parts

- **Dense KB** — machine-facing JSON knowledge base (nodes/edges), gated by 35 checks.
- **The gate** — `tools/compute_kb_formulas.py --inplace` + `validators/kb_validator.py --mode dense`.
- **Specialist** — a distilled operating spec (decision_procedure + workflow + dominance rules)
  over a domain's KBs.
- **ROUTER** — scores need-terms against each specialist's `route_when` (domain_label ×2),
  fans out up to `max_multi=3`.
- **Orchestration graph** — weighted any-to-any DAG: source → specialists → summarizer →
  aggregator (value-saturation / token-budget cap); cycles rejected.
- **SPEC_BEAT** — the `orch` rule: *measure* net value (accuracy gain − routing overhead) before
  committing to specialization; it can lose to overhead.
- **Hook point** — a real GPU/API path left as `NotImplementedError` so nothing silently needs
  compute or makes paid calls.
- **MockBackend** — deterministic offline backend that lets every subsystem run on CPU.
```
