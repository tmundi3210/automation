# Phase B — Execution Plan (idea: orchestrated fine-tuned specialist "thinking" system)

> Reuses the Phase A machinery verbatim: `schema/kb_generator_v1.4.1.txt`,
> `validators/` gate, `prompts/_a4_subdomain_job.md`, `prompts/_a5_specialist_job.md`,
> `tools/build_index.py`, `tools/build_router.py`. Orchestrator stays lean; helpers do
> the heavy generation and return compact reports.

## Inputs (committed)
- `phase_b/IDEA_NEUTRALIZED.md` — neutralized brief + intent map + boundaries.
- `phase_b/taxonomy/llm_engineering.taxonomy.json` — 12 domains / 25 subdomains, with
  canonical books + `frontier_not_in_books` per domain.

## Two tracks (run in this order)
### Track 1 — RESEARCH (KB build, gated; "research = make specialists")
For each of the 25 subdomains:
1. Consult Phase A router (`specialists/ROUTER.json`) to fix WHERE/HOW to source the
   literature for that subdomain (method/source-evaluation/IR specialists).
2. Spawn an A.4 helper (`prompts/_a4_subdomain_job.md`, dense, subdomain grain) →
   `phase_b/knowledge_base/llm_engineering/<domain>__<subdomain>.kb.json` + .validation + .metrics.
3. Gate each (kb_validator, 35 checks, dense). Self-repair once; else escalate.
4. Per domain, distill an A.5 specialist (`prompts/_a5_specialist_job.md`) from its
   subdomain KBs → `phase_b/specialists/<domain>.specialist.json` (gated).
5. Build `phase_b/knowledge_base/llm_engineering/INDEX.json` and
   `phase_b/specialists/ROUTER.json` (reuse `tools/`).
- **Encode the real-world constraints** from `real_world_execution_notes` as KB
  conflict_axes/dominance_rules so specialists reason about license/cost/contamination,
  not just textbook theory.

### Track 2 — BUILD (runnable, GPU-portable scaffold; grounded in Track 1)
Produce code + configs (architecture only here; real fine-tunes need rented GPUs):
- `phase_b/ingest/` — PDF→chunk→JSONL breaker + chunk→KB-node distiller (reuses the gate).
- training-with-rollback loop (eval-in-loop, checkpoint rewind, neutral-last-N prune).
- orchestration graph (any-to-any weighted DAG + router + summarizer node + token-budget cap).
- eval harness (real test sets + LLM-as-judge + score tracking).
- model-selection sheet (per-specialist base-model + license + cost).
- dense↔human renderer at the boundary.
Each subsystem's design is grounded in the corresponding Track-1 specialist's KB.

## Decision points awaiting user (see chat)
- First wave: research-first vs architecture-first vs one vertical slice.
- Breadth now: all 12 domains vs a core subset to validate cheaply.
- Confirm boundary reframes (steer/D7; cognition/D6) in IDEA_NEUTRALIZED §5.

## External dependencies (blocking for parts)
- WCO AI↔AI data (iCloud) and any book PDFs → user drops into `phase_b/sources/`.
- GPU compute → rented; this container produces scaffold/configs only, not real trainings.

## State
- [x] Idea received, neutralized, intent-mapped, boundaries set.
- [x] Seed taxonomy (12 domains / 25 subdomains) committed + JSON-valid.
- [x] User course-correction: research-first, then full 12-domain breadth; boundaries confirmed.
- [x] **Track 1 KB fan-out COMPLETE** — 25/25 dense KBs gated 35/35, 12 specialists,
      INDEX + ROUTER. See PLAN.md §7.
- [x] **Track 2 scaffold COMPLETE** — `phase_b/scaffold/` (offline-runnable, mock backend;
      GPU/API hooks). Six subsystems grounded in their specialists: ingest (chunk→draft→the
      gate), train (eval-in-loop rollback + neutral prune), orch (faithful router + weighted
      any-to-any DAG + summarizer + budget cap), eval_harness (scorers + LLM-judge + tracking),
      select (constraint base-model selector + per-domain map), render (dense↔human neutral
      boundary). `run_all.sh` = 12/12 subsystems pass offline. ADR written (build/buy,
      proven/speculative, vertical-slice-first sequencing). Boundaries enforced in code.
- [ ] Remaining = EXTERNAL: user drops WCO transcripts/PDFs into `phase_b/sources/`; rent
      GPUs for real fine-tunes; refresh `select/model_sheet.json` from live model cards.
