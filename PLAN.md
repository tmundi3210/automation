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
  job prompt, and (b) recorded in the run's metrics/validation sidecar + run manifest (§2a).

### 2a. Run manifest (defined — closes the dangling HEADER.txt "run manifest" reference)
`schema/HEADER.txt` and the bullet above have promised since day 1 that the directive is
"recorded in the run manifest"; until 2026-07-02 (T13/G11) no manifest path, record shape,
or writer existed anywhere. Defined, minimally:
- **Path:** `calibration/results/RUN_MANIFEST.jsonl` — append-only JSONL, beside
  DECISION.json and the §3f GATE_LEDGER.jsonl (NOT `dist/MANIFEST.json`, which is a
  specialist catalog, a different object).
- **Writer:** the ORCHESTRATOR ONLY (§3b custody), at each terminal ACCEPT of an
  artifact — same moment as the §3f ledger line. Helpers never write it.
- **Record (one JSON line per accepted artifact):**
  `{artifact, kind: kb|specialist|taxonomy|index|router|decision, schema_version, mode,
  directive: "prompt_injected"|"embedded_directive_field", validator, est_tokens, ts}`.
  `directive` is the promised provenance of HOW the §2 directive reached the artifact;
  `est_tokens` doubles as the pipeline-level cost ledger G11 noted was missing.
- **Relation to §3f:** GATE_LEDGER.jsonl = gate OUTCOME events (all terminal runs,
  accepts AND rejects, with failed_check_ids); RUN_MANIFEST.jsonl = accepted-artifact
  PROVENANCE/description records. Complementary; neither replaces the other.
- **No backfill:** the manifest starts at adoption (first line = its own adoption event).
  Pre-adoption artifacts are NOT retro-manifested — inventing acceptance timestamps would
  violate the honesty discipline; their provenance remains their committed
  `.validation.json`/`.metrics.json` sidecars + git history, which already exist per KB.

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

### 3a. The determinism boundary (gate limits — named)
The gate above proves **STRUCTURE + MATH only**. A fabricated-but-well-formed KB
exits 0. **Exit 0 = schema-true, never content-true**; no operator may read a green
gate as a truth claim about prose. Content truthfulness is gated by **PROCESS**, in a
second, non-deterministic lane whose pieces exist today:
- **Honesty tags** `[FACT]/[ESTIMATE]/[UNKNOWN]` on every load-bearing claim; an
  estimate names method + basis + confidence; unknowns stay unknown. Enforced as
  neutralize-gate rule **N1** in `FACTORY/sweater_vertical/specialists/BRAIN/BRAIN.md`
  (machine form: `router.json`). The deterministic *slice* of honesty — no
  `observed`/`experimentally_validated` evidence labels without supplied data — is
  already checked by `kb_validator.py` (evidence-label legality).
- **Neutralize (skeptic) gate N1–N7** (`BRAIN.md`): no fabricated firm figures,
  off-paper economics = mechanism never accusation, no invented rates; run BEFORE
  any answer is returned.
- **Fresh-context critic loops (T12)** —
  `FACTORY/sweater_vertical/specialists/BRAIN/SELF_LOOP.md` +
  `self_improvement_loop.json`: every critic/judge role is a NEW context window
  reading only its named inputs, so audit ≠ the model agreeing with itself.
Prose truth claims are accepted only after this process lane (or human review for
`risk_if_wrong >= 0.80` nodes) has run — never on exit code alone.

### 3b. Gate custody (normative)
Helper-run validation (steps inside `prompts/*.md` job templates) is **ADVISORY
ONLY** — a pre-check that enables the helper's one self-repair pass. The
**ORCHESTRATOR ALWAYS RE-EXECUTES** `kb_validator.py` (and
`specialist_validator.py`) itself on the written file before accepting, committing,
or building on any helper output. A helper-reported `validator_overall_status` is
never grounds for acceptance. (Already practiced as §7 OPERATIONAL METHOD 6 —
"never trust helper self-reports; my gate is truth" — stated normatively here.)

### 3c. Density-band near-miss annotation (warn-level, additive)
The kernel's CORE GUARANTEE ORDER ranks "Useful density" **#9 (last)**; the gate
still hard-fails any band miss (bands are calibrated cut scores —
`calibration/results/DECISION.json`). When a count misses its band edge by <=10%,
the validator now ADDS `count.<key>.near_miss` (warn) beside the fail so the
orchestrator has machine-readable signal to route a **recorded, human-gated waiver
decision**. The fail status and exit code are unchanged; there is NO automatic
waiver — changing acceptance is a cut-score change requiring standard-setting
review, impact analysis, and re-validation, not code.

### 3d. Source reality (fabricated-sources lane — warn-level, additive)
`evidence_refs.resolve` proves refs POINT AT registry entries; nothing deterministic
can prove an entry NAMES A REAL SOURCE. Two-layer response (T13/G4):
- **Deterministic layer (in `kb_validator.py`, all WARN — no existing pass flips):**
  `source_registry.entry_shape` (kernel's 10 required source-record fields),
  `source_registry.external_locator_present` (a source_type OTHER than
  `heuristic_prior`/`expert_estimate` claims external authority and must carry a
  non-empty `citation_or_locator`; empty locators on heuristic priors are LEGAL per
  the kernel: "user_supplied_identifier_or_empty_string"),
  `source_registry.locator_format` (a locator that claims DOI/URL/ISBN shape must
  parse as one), `source_registry.label_eligibility_nonempty`,
  `source_registry.unreferenced` (registry entries never cited by any
  evidence_refs — decorative sources are the cheapest fabrication surface), and
  `source_registry.strong_label_source_eligible` (an `observed`/
  `experimentally_validated` label must cite a source whose `label_eligibility`
  permits it). Verified 2026-07-02: zero warns across all 144 gated KBs.
- **Process layer (non-deterministic, off-gate):** on each batch accept, the
  orchestrator SAMPLES >=1 non-heuristic source entry per batch and has a
  fresh-context helper (or human) verify the named work/identifier actually exists
  and supports the `supports` claims; any invented citation is a HARD reject of the
  KB and a fabrication incident recorded beside DECISION.json. The kernel already
  commands "Do not create fake source IDs for unsupported claims" — this is its
  enforcement path. A hard locator requirement stays out of the gate until a
  kb_schema_version bump makes locators structured (see change control).

### 3e. Schema-version pin (gate <-> kernel desync guard)
`kb_validator.py` transcribes the v1.3-family KB schema (bands, formulas, key
sets). It now asserts, per KB (T13/G6): `version.schema_matches_metadata`
(kernel VERSIONING MODEL: top-level `schema_version` MUST equal
`generation_metadata.kb_schema_version`) and `version.supported`
(`schema_version` in `SUPPORTED_KB_SCHEMA_VERSIONS = {"1.3"}`). Both are
FAIL-level assertions verified to hold for ALL 204 in-repo `*.kb.json`
artifacts (2026-07-02), so nothing currently passing changes status. A future
kernel/schema bump must extend the supported set together with re-derived
bands/formulas — the gate now fails LOUDLY instead of silently gating a new
schema with old math.

### 3f. Gate-failure feedback loop (closes the shelved PROMPT_SECTION_REFACTOR path)
§1 shelves the kernel's own self-improvement mode with no substitute; today a failed
gate run terminates in §7 OPERATIONAL METHOD 6 `FAIL -> rm`, which DESTROYS the error
signal (`validator_failed_check_ids`) that every job template already collects in its
compact report. That leaves the generation system OPEN-LOOP above the single-retry
level: gate output (the error signal) is measured, then dropped. The feedback path,
normative:
1. **RECORD (measurement).** For every unit's TERMINAL gate run (final accept or
   final reject), the ORCHESTRATOR — never the helper (§3b custody) — appends one
   line to `calibration/results/GATE_LEDGER.jsonl`:
   `{unit, mode, validator_version, overall_status, failed_check_ids,
   warn_check_ids, self_repair_attempted, respawn_count, ts}` — BEFORE any `rm` of
   a failing artifact. The `rm` rule itself is unchanged; only the signal is
   preserved.
2. **AGGREGATE (damping).** At each batch end the orchestrator groups ledger lines
   by check-id family. A single failure is sampling noise — the calibration lesson
   (one stochastic observation locked GROUP_SIZE; flagged as G10) applies in
   reverse: no prompt/spec change is ever proposed from one observation.
3. **THRESHOLD (deadband).** Only when one check family recurs in >=3 units of a
   batch or >=20% of the batch (heuristic caps, not observed rates) does the
   orchestrator spawn ONE helper to draft a refactor PROPOSAL: either (a) a
   job-template/spec revision, or (b) a kernel-section patch using the kernel's own
   `PROMPT_SECTION_REFACTOR` mode (schema §"PROMPT SECTION REFACTOR MODE" output
   structure) run as a normal one-job helper — this un-shelves the mode as an
   ESCALATION target only, still never during normal generation.
4. **HUMAN-GATED ACTUATION (breaker).** Proposals are written to
   `calibration/results/REFACTOR_PROPOSALS/` beside DECISION.json and are NEVER
   auto-applied. Applying one is a generator change: it requires a
   calibration-style confirmation run before mass use, recorded next to the
   proposal.
Loop-stability rationale: aggregation damps single-sample reactions; the threshold
is a deadband against prompt-churn oscillation; the human gate is the circuit
breaker against Goodharting — an auto-tuned generator would optimize passing the
structural gate, which §3a proves is blind to prose truth.

### 3g. Bounded self-repair + escalation (normative; the "once" rule kept)
§3's "helper self-repairs once, else escalate" is retained deliberately — it is a
bounded retry (cap=1) in front of a circuit breaker, and the cap stays at ONE
because: (i) the deterministic tool lane (§7 OPERATIONAL METHOD 3,
`tools/compute_kb_formulas.py`) already removed the dominant repairable failure
family (formula.consistency), so a report-guided single pass either converges or
signals a spec-level defect; (ii) further in-context retries against a
structure-only gate optimize the proxy (exit 0), not the content (§3a) — retry
amplification is Goodhart pressure; (iii) a second full-KB rewrite risks the
output-token ceiling (§7 OPERATIONAL METHOD 2). What "escalate" means, uniformly
(previously written explicitly only in `prompts/kb_generation_job.md` as
`needs_orchestrator`; the `_a4_*`/`_a5_*` templates only self-reported status):
- **Helper side:** after the one self-repair pass still fails, STOP — return the
  compact report with `validator_overall_status:"fail"` +
  `validator_failed_check_ids` (+ `needs_orchestrator:true` where the template has
  that field). Never loop again; never delete the failing file yourself.
- **Orchestrator side (breaker owner):** record the ledger line (§3f.1), then may
  respawn at most 2 FRESH-context helpers for the unit (fresh context, not more
  in-context retries; heuristic cap). After 1+2 total authoring attempts the unit
  is PARKED as a named backlog item and its ledger lines feed §3f aggregation — a
  unit that fails three independent contexts is evidence about the SPEC/prompt,
  not sampling noise. Respawn is never unbounded.

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
2. Neutralize: preserve intent, restate professionally/neutrally (ONE helper job;
   orchestrator only routes RAW_IDEA in and gates the result — §6a, T13/G12).
3. Academic conversion: collect key books basic->advanced; consult Phase A KB for *where/how*.
4. Build academic taxonomy (domains/subdomains/fields/subfields/topics/subtopics) for the idea.
5. (Optional) DOMAIN_ADAPTER_SCHEMA_GENERATION on the idea -> adapter EXISTING_SCHEMA.
6. Batch into GROUP_SIZE groups; helper runs dense KB per group; gate each
   (calibrated GROUP_SIZE=1: one coherent unit per dense KB — §6a).
7. Per DOMAIN -> helper derives a SPECIALIST grounded ONLY in that domain's validated
   KB(s) (boundary = domain coherence, never batch size — §6a).

### 6a. Unit & boundary rules (normative; T13/G8 + G12)
- **KB unit rule [FACT — calibration]:** `calibration/results/DECISION.json` fixed
  `domains_per_kb: 1` ("ONE coherent domain per dense KB run ... Do NOT batch multiple
  domains into a single dense KB"); the grain rule (§A.3, GRAIN_ANALYSIS/GRAIN_OPTIMUM)
  refines it: the generation unit is the FINEST grain that still NATURAL-FILLS a dense KB
  without padding, stopping before ~45k est output tokens. "GROUP_SIZE" was a
  capacity/batching knob whose calibrated value is 1; it is a throughput constant, not a
  semantic unit, and it sets the boundary of NOTHING downstream.
- **Specialist boundary rule [FACT — practiced repo-wide, verified 2026-07-02]:**
  specialist granularity = DOMAIN COHERENCE — one specialist per coherent domain of
  practice, grounded in ALL (and only) that domain's gate-passing KBs. It is NEVER
  derived from KB batch size or a token ceiling; a GROUP_SIZE of 4 would not have
  justified one specialist spanning four arbitrarily co-batched domains (G8). Step 7's
  original "per validated KB" phrasing described a coincidence (domain-grain units under
  GROUP_SIZE=1), not the rule. In-repo evidence: root `specialists/` 28/28 ground in
  exactly 3 KBs; `phase_b/specialists/` 11/12 in 2 and 1/12 in 3; sweater vertical 9/9
  in 3 — KB count per specialist follows the domain's decomposition, not a constant.
- **Neutralize custody rule (G12 — fixes §6 step 2's self-contradiction):** neutralization
  is a GENERATION task; §0 keeps heavy generation OUT of the orchestrator's context
  ("stays lean — never generates ... in-context"). Neutralize therefore runs as ONE
  helper job in a fresh context, applying the honesty discipline
  ([FACT]/[ESTIMATE]/[UNKNOWN]; skeptic rules N1–N7 where the vertical defines them —
  §3a). The orchestrator only: passes RAW_IDEA verbatim in, receives the neutralized
  restatement + intent map, gates it, records it. This is how the sweater vertical's
  neutralize gate actually ran; the day-1 "(orchestrator)" annotation was the single
  step that contradicted the never-ingest axiom.

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

## 9. Phase B (second application) — `cupcake_craft` specialist set
Same Phase-B machinery applied to a new raw idea: *"make the most delicious cupcake."*
- Neutralized brief: `phase_b/cupcake_craft.IDEA_NEUTRALIZED.md`.
- Taxonomy (fixes the count): `phase_b/taxonomy/cupcake_craft.taxonomy.json` —
  **7 domains -> 7 specialists**, 2 subdomains each -> **14 dense subdomain KBs**.
- KBs: `phase_b/knowledge_base/cupcake_craft/<domain>__<subdomain>.kb.json` (+ .validation + .metrics).
  All **14/14 PASS** `kb_validator --mode dense` (35 checks); 283 nodes / 544 edges / 190 CQs / ~554k tok.
  Derived fields filled by `tools/compute_kb_formulas.py` (formula-consistent, tol 0.02).
- Specialists: `phase_b/specialists/cupcake_craft/<domain>.specialist.json` (+ .validation).
  All **7/7 PASS** `specialist_validator`; each grounded in its 2 subdomain KBs. Domains:
  formula, mixing, bake, flavor, frosting, ingredient, quality.
- Aggregators: `phase_b/knowledge_base/cupcake_craft/INDEX.json` (kb_count 14, all_pass true)
  + `phase_b/specialists/cupcake_craft/ROUTER.json` (cupcake_craft_router, 7 specialists).
- Real-world tradeoffs encoded as conflict_axes/dominance_rules (moistness vs structure,
  sweetness vs complexity, fresh vs shelf-life); food safety = hard overriding dominance rule.
