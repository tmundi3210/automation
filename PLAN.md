# Knowledge-Base Automation — Master Plan & Orchestration State

> Single source of truth for the orchestrator (this Claude window). Read this first
> on every resume. Heavy LLM work runs in **helper sub-agents**; the orchestrator
> holds the plan, enforces gates, and aggregates compact reports only.

## 0. Roles
- **Orchestrator (main window):** owns this plan + taxonomy, spawns helpers, runs the
  deterministic gate (`validators/`), records decisions, commits artifacts. Stays lean —
  never generates KBs in-context.
- **Helper sub-agent (a "new session / terminal"):** runs ONE schema job, writes JSON to a
  file, runs the validators, returns a compact report (status, est_tokens, counts, paths).

## 1. The engine
`schema/kb_generator_v1.4.1.txt` — Expert-System JSON KB Generator. Modes:
- `KB_GENERATION` (default): DOMAIN -> strict-JSON KB. Density: compact/standard/**dense**.
- `DOMAIN_ADAPTER_SCHEMA_GENERATION`: RAW_IDEA -> adapter schema that constrains a later KB run.
- `PROMPT_SECTION_REFACTOR`: self-improvement of the prompt (not used in normal runs).

## 2. Header reconciliation (important)
User directive: every output is *machine-facing, token-dense, model-parse-optimized*
(see `schema/HEADER.txt`). But KB_GENERATION outputs MUST be strict JSON (`{...}`, no extra
keys). A literal text header would break that, and the schema's JSON-validity veto wins.
Resolution:
- Pipeline-owned artifacts (taxonomies, calibration/metrics/decision records, specialists):
  carry the directive verbatim in a top-level `"_directive"` field.
- KB_GENERATION outputs: stay pure JSON. The directive is (a) injected into the generation
  job prompt, and (b) recorded in the run's metrics/validation sidecar + manifest.

## 3. Quality + Quantity gate (deterministic, committed)
`validators/kb_validator.py KB.json --mode dense` enforces:
- QUANTITY: density-mode counts for nodes/edges/conflict_axes/edge_cases/workflow/CQs +
  dominance(7-12)/anti_rework(7-12)/iteration(6-10).
- QUALITY: JSON valid, required top-level keys, ID uniqueness, edge/dep/workflow/edge-case/
  conflict-axis/source/CQ reference integrity, priority_order completeness, numeric bounds
  [0,1] / signed_tension [-1,1], conflict-edge sign + resolution rule, high-risk &
  high-coupling obligations, uncertainty intervals, dependency-cycle absence, placeholder
  leakage, evidence-label legality (no observed/experimental without data), and FULL formula
  consistency (all node + edge formulas recomputed, tol 0.02).
`validators/metrics.py` records size (chars/bytes/est_tokens) + object counts.
Exit code 0 = accept; non-zero = reject -> helper self-repairs once, else escalate.

## 4. Directory map
```
schema/      kb_generator_v1.4.1.txt, HEADER.txt
taxonomy/    knowledge_searcher.taxonomy.json   (Phase A meta-domain seed)
validators/  kb_validator.py, metrics.py        (the gate)
prompts/     calibration_runner.md, kb_generation_job.md   (helper job specs)
calibration/ results/                            (per-run reports + DECISION.json)
knowledge_base/  <set>/<GROUP_ID>.kb.json (+ .validation.json, .metrics.json)
specialists/ <DOMAIN>.specialist.json           (Phase B output)
```

## 5. Phase A — build the "Knowledge Searcher" KB (how/where to find knowledge)
- A.1 Taxonomy seed (domains->...->subtopics + canonical books). **DONE (seed).**
- A.2 Deep population: helper expands seed + extracts book-content into exhaustive
      subfield/topic/subtopic inventory mapped to node ids. **PENDING.**
- A.3 Calibration sweep. **DONE.** Result: `calibration/results/DECISION.json`.
      RUN_1 (IR, 19n/39e, ~37k tok), RUN_2 (method, 21n/39e, ~25k tok), RUN_1+2
      (combined, 21n/40e, ~25k tok) all PASS the gate (35/0/0). KEY FINDING: dense
      mode has a FIXED node budget (19-24), so batching 2 domains into one KB halved
      per-domain depth (IR 19->8 nodes, ~58% coverage loss) without raising total
      output. Then a GRANULARITY follow-up (`calibration/results/GRAIN_ANALYSIS.json`):
      IR domain (19 nodes) vs IR broken into 3 subdomains (ranking/indexing/neural;
      20/19/20 nodes, all PASS). FINDINGS: per-KB node budget is FIXED by density
      (19-24) and INVARIANT to grain; grain sets RESOLUTION; batching-up thins depth,
      breaking-down multiplies it (subdomain grain = 3.1x nodes / 2.5x tokens vs domain
      grain, all gate-passing). Max single KB 40.6k tok, no truncation (ceiling ~40-45k).
      **DECISION: generate ONE dense KB per SUBDOMAIN/field-grain unit; scale by
      recursive decomposition + parallel fan-out, not by batch size.** RUN_4 (4 domains
      in one KB) unnecessary; available on request.
      Grain-optimum probe (`calibration/results/GRAIN_OPTIMUM.json`): IR ranking
      subdomain (20 nodes) decomposed into 3 TOPICS (term-weighting/LTR/feedback;
      21/21/20 nodes, all PASS, all natural_fill=true 0-padded). Cross-topic node
      redundancy = 0%; topic union = 62 distinct nodes (3.1x subdomain) with only 20%
      conceptual overlap. Topic+dense runs hit 39-44k tokens (just under ~45k ceiling).
      **GRAIN RULE: go as fine as a unit still NATURAL-FILLS a dense KB without padding,
      and stop before per-KB tokens approach ~45k. dense+topic is the practical finest
      grain under one response; finer units use standard/compact or underfill.**
- A.4 Mass generation: 1 domain per dense KB, fan out helpers in parallel, gate each,
      commit each. Low-value/narrow domains may use standard/compact. **PENDING.**
- A.5 The resulting KB set = the foundational "how to research knowledge" specialist,
      consulted by every Phase B run to locate sources first.

## 6. Phase B — idea -> specialist pipeline (reusable, per user-supplied idea)
1. Receive RAW_IDEA.
2. Neutralize: preserve intent, restate professionally/neutrally (orchestrator).
3. Academic conversion: collect key books basic->advanced; consult Phase A KB for *where/how*.
4. Build academic taxonomy (domains/subdomains/fields/subfields/topics/subtopics) for the idea.
5. (Optional) DOMAIN_ADAPTER_SCHEMA_GENERATION on the idea -> adapter EXISTING_SCHEMA.
6. Batch into GROUP_SIZE groups; helper runs dense KB per group; gate each.
7. Per validated KB -> helper derives a SPECIALIST grounded only in that KB.

## 7. Current state
- [x] Repo scaffolded; schema vendored; validators + metrics written; taxonomy seed; job specs.
- [x] A.3 calibration + granularity + grain-optimum probes (9 helpers, all pass) ->
      grain is the lever; unit = finest grain that NATURAL-FILLS dense (topic for rich
      areas, subdomain for thin); per-KB token ceiling ~45k. See DECISION/GRAIN_ANALYSIS/GRAIN_OPTIMUM.
- [ ] A.2 deep population (helper) — optional, can precede or follow A.4.
- [x] A.4 mass generation: 24/24 subdomain dense KBs, all gate-pass; INDEX.json built
      (489 nodes, 930 edges, ~964k tokens total). knowledge_base/knowledge_searcher/.
- [x] A.5 specialists: 8 per-domain specialists (ir/method/infosci/biblio/evsynth/kr/
      scholcomm/libarch), all pass specialist_validator; + specialists/ROUTER.json over them
      (routing verified on 8 sample needs). tools/build_index.py, tools/build_router.py.
- [x] **PHASE A COMPLETE** — foundational Knowledge Searcher (KBs + specialists + router).
- [~] **PHASE B IN PROGRESS** — RAW_IDEA received (orchestrated fine-tuned specialist "thinking"
      system). Neutralized + intent-mapped + boundaries (`phase_b/IDEA_NEUTRALIZED.md`); seed
      taxonomy: 12 domains / 25 subdomains + `frontier_not_in_books` (`phase_b/taxonomy/
      llm_engineering.taxonomy.json`); plan `phase_b/PLAN_B.md`. User course-correction:
      research-first, CORE SUBSET (6 domains: ftune, orch, eval, distill, reason, select =
      13 subdomain KBs), both boundaries confirmed.
  - **RESEARCH LAYER COMPLETE — all 12 domains:** 25/25 dense KBs gated 35/35
    (`phase_b/knowledge_base/llm_engineering/`, 543 nodes / 973 edges / ~876k tok); 12
    per-domain specialists gated (`phase_b/specialists/`); INDEX.json + ROUTER.json rebuilt
    over the full set (routing 11/12 on sample needs; math/reason overlap handled by
    multi_specialist_policy + fallback). Domains: ftune, orch, eval, distill, reason, select,
    tool, intent, steer, infra, math, code. Boundary guards held on steer__repeng (literature
    survey, not guardrail removal) and intent__cogmodel (adaptive UX, not clinical diagnosis).
  - **OPERATIONAL METHOD (hard-won; reuse for all future KB fan-out):**
    1. Helpers MUST be told: do the work YOURSELF, do NOT use the Agent tool / delegate / "wait"
       (general-purpose sub-agents otherwise spawn children and idle).
    2. Write the ~100KB KB via a PYTHON BUILDER SCRIPT run with Bash (json.dump), NOT via the
       Write tool — a direct write exceeds the 32000 output-token limit and errors.
    3. Helpers author BASE metrics only; `tools/compute_kb_formulas.py` fills ALL derived fields
       deterministically (mirrors validator formulas exactly) — kills formula.consistency fails.
    4. Helpers copy the STRUCTURE of a known-passing KB (knowledge_searcher/ir__ranking_and_
       relevance.kb.json) — kills required-key/edge-type/structure fails.
    5. Job spec: `prompts/_a4_phaseb_job.md` (v3). Validator hardened to FAIL (not crash) on
       malformed structure.
    6. Orchestrator HARVEST loop (never trust helper self-reports; my gate is truth):
       `git checkout -- <kbdir>` (revert straggler overwrites of committed files); for each
       untracked *.kb.json: run formula tool, validate; PASS -> regen sidecars + git add;
       FAIL -> rm (a failing file is never a v3 success, so deleting fails is safe; NEVER delete
       a passing/committed file — that lost a unit once). Commit passes; push.

## 8. Next action
Track 1 (research) AND Track 2 (scaffold) both COMPLETE.
- **Track 2 — runnable scaffold DONE** (`phase_b/scaffold/`, offline-runnable, mock backend;
  GPU/API marked as hook points). Six subsystems, each grounded in its Track-1 specialist:
  `ingest/` (PDF/text->chunks->seeded KB draft->THE REAL GATE: compute_kb_formulas + kb_validator
  + distillation work order), `train/` (eval-in-loop training with checkpoint rollback +
  neutral-last-N prune + lr decay + early stop + JSON lineage; SimulatedTrainer offline,
  HFTrainerBackend hook), `orch/` (router faithful to ROUTER.json selection_procedure + weighted
  any-to-any DAG + summarizer context-bound + aggregator value-saturation cap), `eval_harness/`
  (tool-grounded numeric / choice / LLM-as-judge open scorers + pass@k + contamination flag +
  score tracking; doubles as the loop's evaluate()), `select/` (hard-filter + objective-dial
  base-model selector + per-domain map; sheet is a verify-before-use template for volatile
  frontier data), `render/` (deterministic dense<->human neutral boundary). `common.py`/
  `backends.py` = shared loaders + pluggable ModelBackend. `run_all.sh` = 12/12 pass offline.
  `ARCHITECTURE_DECISION_RECORD.md` = the §8 deliverable (build/buy, proven/speculative,
  vertical-slice-first sequencing). Boundaries enforced in code (steer=format/neutral render
  only; intent=adaptive UX not diagnosis).
- **Remaining is EXTERNAL (user/compute), not code:**
  - **Supply data**: drop WCO/iCloud transcripts + book PDFs into `phase_b/sources/`; then
    `ingest/pdf_to_chunks.py` + the LLM-authoring pass wire real distillation.
  - **Rent GPUs** for real fine-tunes/serving (implement the HFTrainerBackend/LocalBackend hooks).
  - **Refresh** `select/model_sheet.json` from live model cards before any base-model decision.
- Suggested first real step (from the ADR): a single vertical slice (math or code specialist)
  end-to-end before scaling breadth; gate breadth on the orch SPEC_BEAT ablation.
Method + harvest loop for any future KB fan-out: see section 7 OPERATIONAL METHOD.
