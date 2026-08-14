# branches/ — idea-branch capabilities

Self-contained meta-capabilities derived from the original idea backlog (B10–B51).
Each active branch composes existing **gate-passed** specialists (`specialists/*.specialist.json`)
into a prompt-only specialist plus one deterministic stdlib connective artifact (validator/tool),
kept here so the validated 28-specialist set + `ROUTER.json` stay untouched until a branch is promoted.

> **Handoff + how to use:** see **[`HANDOFF.md`](HANDOFF.md)** for the full pipeline (build →
> gate → distill → run a specialist in prompt-only mode) and a command cheat-sheet.
>
> **Heavy track (done):** B10/B11/B13 now each also have a **heavy** specialist
> (`specialist.heavy.json`) grounded in its own **3 freshly generated dense KBs** (9 KBs total,
> all passing `kb_validator --mode dense`), built via `_forge/kb_forge.py` — same construction
> as the original 28. A worked self-application run is in `_forge/demo/self_application_run.md`.
>
> **B60 — Content-Intelligence Engine (heavy scaffold + runnable v1, done):** a full application of
> the pipeline to a real idea (scrape/understand/link/generate public info + a self-testing brain).
> **5 new heavy specialists grounded in 15 freshly generated dense KBs** (all gate-passing) + an
> information-node schema + a controller loop, then the v1 MVP turned into **offline-runnable code**
> (`scaffold/run_pipeline.py` ships a clearance-passing brief + a leakage-controlled verdict). See
> **[`b60_content_intelligence/`](b60_content_intelligence/README.md)**
> (`SCOPE.md` → `BRAIN.md` → `RUN.md` → `PANEL.md` → `BUILD.md` → `scaffold/`).

Run a branch's gate from the repo root, e.g.:

```
python3 branches/b10_question_compiler/erotetic_lint.py      branches/b10_question_compiler/examples/q_polar.pass.json
python3 branches/b11_knowledge_acquisition/acquisition_ledger.py branches/b11_knowledge_acquisition/examples/ledger.pass.json
python3 branches/b13_recursive_planner/plan_validator.py     branches/b13_recursive_planner/examples/plan.pass.json
python3 -m unittest discover -s branches -p 'test_*.py'
```

Each `specialist.json` is injection-ready (load verbatim into `dist/prompt_template.json`) **and**
passes the repo's `validators/specialist_validator.py` (exit 0).

## Active (built, gated)

| Branch | Capability | Composes | Connective artifact | Gate |
|---|---|---|---|---|
| **B10** `b10_question_compiler` | Question science / framing — erotetic compiler: seed question → typed question-object + sub-question DAG; scores questions & answers | ling, argue, kr, infosci, method, psych | `erotetic_lint.py` (closed→answer-set / open→type-conformance+warrant) | pass 0 / fail 1 · 10 tests · spec ✓ |
| **B11** `b11_knowledge_acquisition` | Knowledge acquisition / known-unknown mapping — one CQ-ledger, derived matrices, SAT probe battery | method, evsynth, scholcomm, libarch, (argue) | `acquisition_ledger.py` (evidence-label legality, derived views, probe→CQ) | pass 0 / fail 1 · 10 tests · spec ✓ |
| **B13** `b13_recursive_planner` | Recursive planning engine — type-first decomposition with proven termination + contract gate | method, loops, argue, appdev | `plan_validator.py` (termination proof, defeater-fallbacks/SPOF, contract composition, fatal-break) | pass 0 / fail 1 · 13 tests · spec ✓ |

## Heavy build (dense-KB-grounded specialists)

Beyond the prompt-only composition specialists above, each active branch now also ships a
**heavy** specialist (`specialist.heavy.json`) distilled from **three freshly generated dense
KBs of its own** — the same construction as the validated 28-specialist set (schema 1.3, gated
by `validators/kb_validator.py --mode dense`). The KBs are built deterministically by
`_forge/kb_forge.py` from compact content specs (`kb/_src/*.spec.json`); the builder computes
every derived metric / id / `priority_order` so the gate's formula and reference checks pass by
construction. `_forge/HELPER_BRIEF.md` is the author contract and `_forge/example_htn_gen.py`
is a worked exemplar.

Run the heavy gate from repo root:

```
for f in branches/b1*/kb/*.kb.json; do python3 validators/kb_validator.py "$f" --mode dense --quiet && echo "PASS $f"; done
python3 validators/specialist_validator.py branches/b13_recursive_planner/specialist.heavy.json
```

| Branch | Heavy specialist | Grounded in 3 new dense KBs (each `--mode dense` exit 0) |
|---|---|---|
| **B10** | `specialist.heavy.json` (`erotetic_heavy`) | `erot__question_semantics`, `frame__operationalization`, `qeval__answer_quality` |
| **B11** | `specialist.heavy.json` (`epistemics_heavy`) | `kumap__uncertainty_taxonomy`, `acq__evidence_sourcing`, `sat__structured_probing` |
| **B13** | `specialist.heavy.json` (`planner_heavy`) | `htn__decomposition`, `term__well_foundedness`, `contract__composition_failure` |

All 9 KBs are 19–24 nodes / 32–40 edges / 14 CQs each; **9/9 pass** `kb_validator --mode dense`
and all 3 heavy specialists pass `specialist_validator.py`. Each branch keeps BOTH its
composition specialist (`specialist.json`) and its heavy specialist (`specialist.heavy.json`);
the validated 28-set + `ROUTER.json` stay untouched.

## B60 — Content-Intelligence Engine (applied scaffold)

A full, worked application of the whole pipeline to a real idea (ingest public info → understand
densely → link 2–3 related things → generate image/story/audio prompt-briefs for an audience,
driven by a self-testing "brain"). Built in 3 phases: **scope** (run through the existing
planner/erotetic/epistemics) → **build** (5 new heavy specialists from **15 freshly generated dense
KBs**) → **run** (worked end-to-end on one concrete seed). All 15 KBs pass `kb_validator --mode
dense`; all 5 specialists pass `specialist_validator.py` (independently re-gated).

| Specialist | Covers | Grounded in 3 dense KBs |
|---|---|---|
| `salience_heavy` | map + link | `geo__attention_topology`, `niche__audience_segmentation`, `link__entity_relevance` |
| `signal_heavy` | understand + signal | `auth__inauthenticity_detection`, `hype__organic_baseline`, `brief__rate_distortion_summary` |
| `creative_heavy` | generate | `scene__multi_entity_composition`, `promptgen__downstream_brief`, `novelty__derivative_vs_copy` |
| `eval_heavy` | brain + eval | `leakage__holdout_design`, `metric__frozen_preregistration`, `backtest__outcome_validity` |
| `compliance_heavy` | legal gate (fail-closed) | `pubrights__likeness_voice`, `defamation__false_light`, `platform_tos__synthetic_disclosure` |

Docs: `b60_content_intelligence/SCOPE.md` (findings + plan) → `BRAIN.md` (controller loop) →
`RUN.md` (worked seed run) → `PANEL.md` (critical 10-specialist read) → `BUILD.md` (constructive
build-forward spec) + `schema/information_node.schema.json`. The five specialists are the "brain";
`planner/erotetic/epistemics_heavy` are the reasoning core that frames and plans.

The v1 MVP from `BUILD.md` is now **running code** in `b60_content_intelligence/scaffold/` —
offline, deterministic, stdlib-only. `python3 .../scaffold/run_pipeline.py` ships a clearance-passing
safe-lane brief to an outbox **and** returns a leakage-controlled eval verdict (currently NO-GO);
`bash .../scaffold/run_all.sh` smoke-tests all 17 subsystems. See `scaffold/README.md`.

An intelligence analysis found the deterministic scaffold is only a narrow expert system, so
`b60_content_intelligence/mind/` rebuilds the brain as an **LLM-run cognitive + memory system** — seven
specialist-designed faculties (kernel/agency/attention/calibration/learning/generalization/memory) that
close the learning, agency, calibration, generalization, multi-environment, and being-a-mind gaps using
**persistent memory files** (diary/checkpoints/schedule) operated by an LLM, no Python pipeline. Run via
`mind/KERNEL.md`; see `mind/GAP_MAP.md` for the honest closed/partial status of each gap.

## Pending (analyzed, not started — see scratchpad/IDEA_BRANCHES_REPORT.md)

Suggested order honors local-first / eval-before-training / no-LoRA-before-failure-data.

| Branch | One-line | Start after |
|---|---|---|
| B50 | Compile specialists into per-pipeline-role pre-injection prompts + `prompt_contract_validator` | — (substrate) |
| B14 | Idea intake: capture + forced multi-branch enumeration + append-only hash-chained ledger | — (substrate) |
| B12 | Non-coder "software-director" specialist (appdev lifecycle + business TOC + aicomp bounded-AI) | B50 |
| B21 | Distortion/debias audit specialist (evidence-tier, side-effect ledger; "remove a layer" = falsifiable hypothesis) | B50 |
| B30 | Eval-FIRST failure-dataset factory (CQs-as-oracles vs existing specialists) — gates any future LoRA | B21 |
| B40 | Offline topology bench (7 orchestration shapes, paired/FDR ablation) | B30 |
| B51 | Event-sourced memory (append-only log + re-derivable gate-passing projections) | B21 |
| B33 | Planner→command bridge (test-justified commands, least-privilege allowlist; defensive) | B50 |
| B20 | Transformer-internals dissection KB + `interp` specialist (evidence-tier; understanding ≠ control) | B21 |
| B31 | Web/UI design-critic (dual-channel: deterministic a11y PASS/FAIL + heuristic hierarchy grades) | B21 |
| B32 | Cognitive-load explanation judge (audience-conditioned load-budget rubric, matched pairs) | B21 |
| B22 | Nano-LM from-scratch lab (CPU char-transformer on synthetic known-answer tasks) | B21 |
| B41 | Before-you-rent capacity estimator (params×bytes + GQA KV-cache; prices verify-live) | B21 |
