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
      output. **DECISION: GROUP_SIZE = 1 domain per dense KB; scale by parallel
      fan-out, not by co-batching.** RUN_4 skipped (user-conditional; degradation
      already shown; available on request).
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
- [x] A.3 calibration sweep (3 helpers, all pass) -> GROUP_SIZE = 1 domain/dense KB; see DECISION.json.
- [ ] A.2 deep population (helper) — optional, can precede or follow A.4.
- [ ] A.4 mass generation — **next action, gated on user go-ahead**
- [ ] Phase B (awaiting a RAW_IDEA)

## 8. Next action
GROUP_SIZE fixed at 1 domain per dense KB. Awaiting user go-ahead to either (a) run A.2
deep population first, or (b) start A.4 mass generation: fan out one helper per taxonomy
domain (dense; standard/compact for narrow/low-value), gate + commit each KB, then derive
the foundational Knowledge Searcher specialist (A.5).
