# Handoff — Heavy specialists + the KB→specialist pipeline

This document is a self-contained handoff for the three **heavy specialists** built on the
`branches/` track and the deterministic pipeline that produces and runs them. Read top to
bottom once; the command cheat-sheet is at the end.

---

## 0. TL;DR

- A **specialist** is a machine-facing JSON operating spec (role + decision procedure +
  workflow + escalation triggers) distilled from a set of dense **Knowledge Bases (KBs)**.
- You **run** a specialist with no training and no fine-tuning: drop its JSON verbatim into
  `dist/prompt_template.json`'s `specialist_prompt_template` and send to any model — the
  model then reasons *as* that specialist. This is "prompt-only mode."
- **3 heavy specialists** are done, each grounded in **3 freshly generated dense KBs**
  (9 KBs total), all passing the real deterministic gate:
  | id | domain | grounded in (3 dense KBs) |
  |---|---|---|
  | `planner_heavy` | Recursive plan decomposition | `htn__decomposition`, `term__well_foundedness`, `contract__composition_failure` |
  | `erotetic_heavy` | Question science / erotetic compilation | `erot__question_semantics`, `frame__operationalization`, `qeval__answer_quality` |
  | `epistemics_heavy` | Knowledge acquisition / known-unknown mapping | `kumap__uncertainty_taxonomy`, `acq__evidence_sourcing`, `sat__structured_probing` |
- A worked end-to-end demo (each specialist run on its own founding idea) is in
  `branches/_forge/demo/self_application_run.md`.
- **Applied at scale — `b60_content_intelligence/`:** the same pipeline run on a real idea
  (ingest/understand/link/generate public info + a self-testing brain) produced **5 more heavy
  specialists from 15 freshly generated dense KBs** (`salience_heavy`, `signal_heavy`,
  `creative_heavy`, `eval_heavy`, `compliance_heavy` — all gate-passing), an information-node
  schema, and a controller loop wiring all 8 specialists together. Read
  `branches/b60_content_intelligence/README.md` → `SCOPE.md` → `BRAIN.md` → `RUN.md`. This is the
  best concrete example of using the pipeline end-to-end on a new idea.
- These live under `branches/` in isolation; the validated 28-specialist main set and
  `specialists/ROUTER.json` are untouched. Wiring into the main router is optional (§8).

---

## 1. What "this pipeline" is

The repo (`/home/user/automation`) is a **factory**: it turns the Expert-System JSON KB
generator schema (`schema/kb_generator_v1.4.1.txt`) into dense, machine-facing KBs, runs
them through a deterministic quality gate, and distills each domain's KB set into a
**specialist** that an agent can load and operate as. See the repo root `README.md` and
`PLAN.md` for the original factory; this `branches/` track is a heavy-specialist build on
top of it.

Two artifact kinds matter here:

1. **Dense KB** (`*.kb.json`) — a typed knowledge graph: 19–24 `nodes` (work units, each with
   scored metrics), 32–40 `edges`, conflict axes, edge cases, a workflow, dominance/
   anti-rework/iteration rules, competency questions, glossary, math model, and validation
   blocks. Every derived metric obeys an exact formula the gate re-checks.
2. **Specialist** (`*.specialist.json` / `specialist.heavy.json`) — distilled from a KB set:
   `role`, `capabilities`, `decision_procedure`, `workflow`, `escalation_triggers`,
   `conflicts_and_dominance`, `validation_checklist`, `glossary`,
   `competency_questions_covered`, and `grounded_in_kbs` (the KB files it stands on).

"Heavy" vs "composition": each branch has **both**. `specialist.json` is a lighter
*composition* spec that reuses existing specialists' KBs; `specialist.heavy.json` is the
*heavy* spec grounded in its own 3 newly generated dense KBs — same construction as the
original 28-specialist set.

---

## 2. The build pipeline (how a heavy specialist is made)

```
  idea
   │
   ▼
 (a) CONTENT SPEC        author prose + base metric magnitudes only
     <name>.spec.json    (a compact JSON; see branches/_forge/example_htn_gen.py)
   │
   ▼
 (b) BUILD               python3 branches/_forge/kb_forge.py spec.json -o out.kb.json
     kb_forge.py         deterministically computes ALL derived metrics, ids,
                         priority_order, label maps, static schema blocks, and
                         enforces high-risk / high-coupling obligations
   │
   ▼
 (c) GATE (KB)           python3 validators/kb_validator.py out.kb.json --mode dense
     kb_validator.py     exit 0 = pass. Checks counts, ref integrity, formula
                         consistency (tol 0.02), acyclicity, placeholders, bounds
   │   (×3 KBs per specialist)
   ▼
 (d) DISTILL SPECIALIST  author specialist.heavy.json grounded_in_kbs=[the 3 KBs]
   │
   ▼
 (e) GATE (specialist)   python3 validators/specialist_validator.py specialist.heavy.json
     specialist_validator exit 0 = pass. Checks required keys, non-empty lists,
                         grounded KB files EXIST, capability shape, no placeholders
   │
   ▼
 (f) RUN (prompt-only)   inject specialist.heavy.json into dist/prompt_template.json
                         → an agent reasons AS the specialist (no training)
```

Key property: **the builder + gate are the source of truth.** A human/agent authors only
content; `kb_forge.py` guarantees the mechanical correctness (the formula checks at
tolerance 0.02 are impossible to hand-author reliably, so they are *computed*). Always
re-run the gate yourself — never trust a generator's self-report.

---

## 3. How to RUN a specialist (prompt-only injection)

The harness is `dist/prompt_template.json`. To run specialist S on a question Q:

1. **Pick** S (manually, or route — see §8).
2. **Load** S's JSON verbatim, e.g. `branches/b13_recursive_planner/specialist.heavy.json`.
3. **Fill** `specialist_prompt_template`:
   - `system` ← as given, with `{{DOMAIN_LABEL}}` ← S's `domain_label`.
   - `context` ← `<specialist_spec>` + the **entire** S JSON in `{{SPECIALIST_SPEC_JSON}}`.
   - `user` ← `{{USER_QUESTION}}` ← Q.
4. **Send** to any capable model. Its `role` / `decision_procedure` / `workflow` /
   `escalation_triggers` / `conflicts_and_dominance` drive the answer. The system prompt
   already enforces: ground every claim, label heuristic priors as heuristic, escalate
   irreversible/high-risk actions, answer only within the specialist's domain.

The model may optionally read the `grounded_in_kbs` files to cite specific node ids, but the
spec alone is sufficient to operate.

**Worked example (already run, for reference):** `branches/_forge/demo/self_application_run.md`
shows all three heavy specialists invoked exactly this way, each on its own founding idea.
E.g. `planner_heavy` decomposed "build the planner," proved its own termination, and
*rejected the idea's binary-halving default as non-well-founded*; `epistemics_heavy` graded
its own usefulness confidence as LOW under its evidence-label-legality rule. The specialists
critique, not flatter — that is the intended verify-not-flatter behavior.

> Note: the repo's MAIN 28-specialist set is also pre-assembled, one self-contained injected
> prompt per line, in `dist/specialist_prompts.jsonl` — pick a line, drop in the question.
> The branch heavy specialists are not in that file yet (§8).

---

## 4. The deterministic gate (what "passing" means)

### KB gate — `validators/kb_validator.py KB.json --mode dense`
Exit 0 = pass. Independent checks include: required top-level keys; dense-mode counts
(nodes 19–24, edges 32–40, conflict_axes 8–10, edge_cases 10–12, workflow 9–12,
competency_questions 10–14, dominance/anti-rework 7–12, iteration 6–10); id uniqueness;
edge endpoint/type validity; conflict edges have `signed_tension<0` + `resolution_rule`;
dependency-graph acyclicity; evidence/CQ reference resolution; metric bounds [0,1];
high-risk (`risk_if_wrong>=0.80`) ⇒ acceptance_tests + human_review; high-coupling
(`cross_topic_coupling>=0.75`) ⇒ revisit_triggers; uncertainty intervals; **formula
consistency** (every `final_importance/risk_score/confidence_score/revisit_pressure/
lock_score/posterior_importance/edge_*` matches its weighted formula within 0.02); and
placeholder-token leakage. `--report out.json` writes the per-check record.

### Specialist gate — `validators/specialist_validator.py SPEC.json`
Exit 0 = pass. Checks required keys, non-empty required lists, that every
`grounded_in_kbs` path **exists** (run from repo root), capability object shape, the
machine-facing `_directive`, and absence of placeholder tokens.

Current status of this track: **9/9 KBs pass; 6/6 branch specialists pass** (3 heavy +
3 composition).

---

## 5. The builder (authoring a NEW KB)

`branches/_forge/kb_forge.py` — read its module docstring; it is the authoritative
content-spec contract. You supply, per node, only: prose (topic, definition, scope,
inputs/outputs, pros/cons, failure_modes, acceptance_tests, revisit_triggers),
`dependencies` (must stay acyclic), `competency_question_refs`, and a `base` dict of the
9 metric magnitudes + `acceptance_test_pass_rate` + `dependency_gate_pass_rate` +
`prior_importance` + `evidence_confidence` + `failure_rate` + `downside_weight` +
`uncertainty_interval` + `update_signal` (all in [0,1]). The builder computes everything
derived and assembles the full KB.

- **Exemplar to copy:** `branches/_forge/example_htn_gen.py` (a complete generator that
  builds + passes the gate). It defines compact helpers `node()/b()/pro()/con()/dep()/
  conf()/rel()`.
- **Authoring brief / rules:** `branches/_forge/HELPER_BRIEF.md` (counts, DAG rule, ref
  integrity, obligations, forbidden placeholder tokens).
- **Per-KB sources** are kept under `branches/<branch>/kb/_src/<name>.spec.json` and the
  generators under `branches/_forge/gen_<name>.py`, so every KB is reproducible.

---

## 6. File map

```
branches/
  HANDOFF.md                         ← this file
  README.md                          ← branch tracker (active vs pending)
  _forge/
    kb_forge.py                      ← deterministic dense-KB builder (the engine)
    HELPER_BRIEF.md                  ← authoring contract / rules
    example_htn_gen.py               ← worked exemplar generator (copy this)
    gen_*.py                         ← one generator per KB (reproducible source)
    demo/
      self_application_run.md        ← worked run: 3 specialists on their own ideas
  b10_question_compiler/             ← B10 "erotetic"
    specialist.heavy.json            ← heavy spec (grounded in 3 KBs below)
    specialist.json                  ← lighter composition spec
    erotetic_lint.py + schema + tests
    kb/
      erot__question_semantics.kb.json        (+ .validation.json, _src/*.spec.json)
      frame__operationalization.kb.json
      qeval__answer_quality.kb.json
  b11_knowledge_acquisition/         ← B11 "epistemics"
    specialist.heavy.json / specialist.json
    acquisition_ledger.py + sat_battery.json + rubric + tests
    kb/ kumap__uncertainty_taxonomy | acq__evidence_sourcing | sat__structured_probing
  b13_recursive_planner/             ← B13 "planner"
    specialist.heavy.json / specialist.json
    plan_validator.py + schema + tests
    kb/ htn__decomposition | term__well_foundedness | contract__composition_failure

dist/prompt_template.json            ← the injection harness (how to RUN a specialist)
dist/specialist_prompts.jsonl        ← pre-assembled prompts for the MAIN 28 (not branch ones)
validators/kb_validator.py           ← KB gate (--mode dense)
validators/specialist_validator.py   ← specialist gate
schema/kb_generator_v1.4.1.txt       ← the generator schema the KBs conform to
specialists/ROUTER.json              ← main 28-specialist router (branch ones NOT in it yet)
```

Each branch also ships a **deterministic connective artifact** (a stdlib lint/validator):
`erotetic_lint.py` (question-object linting), `acquisition_ledger.py` (CQ-ledger + evidence-
label legality), `plan_validator.py` (termination/contract/coverage checks) — each with a
JSON schema, pass/fail fixtures, and unit tests. Run the branch tests with:
`python3 -m unittest discover -s branches -p 'test_*.py'`.

---

## 7. Add a 4th heavy specialist (recipe)

1. Pick a domain; define 3 sibling KB scopes (foundations / mechanism / verification).
2. For each KB: copy `example_htn_gen.py`, write a `gen_<name>.py` that emits
   `branches/<branch>/kb/_src/<name>.spec.json`; build with `kb_forge.py`; gate with
   `kb_validator.py --mode dense`; iterate to exit 0.
3. Author `branches/<branch>/specialist.heavy.json` with `grounded_in_kbs` = the 3 KB paths
   (relative to repo root) and the required keys (mirror an existing heavy spec).
4. Gate with `specialist_validator.py` → exit 0.
5. (Optional) add a connective lint + tests; (optional) wire into the router (§8).

---

## 8. Wiring into the main router (optional, not done yet)

The branch specialists are intentionally isolated so the validated 28-set + `ROUTER.json`
stay frozen. To make them first-class:
- append each heavy spec's `{code: route_when[]}` to the routing table the
  `router_prompt` in `dist/prompt_template.json` consumes, and
- append one assembled line per heavy spec to `dist/specialist_prompts.jsonl`.
Until then, invoke them directly per §3. (Ask before modifying `specialists/ROUTER.json`.)

---

## 9. Boundaries & honest caveats

- **Heuristic priors.** No observed dataset backs the KB scores; every metric is labeled
  `heuristic` and `empirical_status: heuristic_prior_not_observed_dataset`. The specialists
  surface this themselves. Do not read the numbers as measured.
- **Scope boundaries are enforced in-spec.** `erotetic_heavy` types/scores questions and
  never answers domain content; `epistemics_heavy` grades confidence in evidence and never
  asserts domain truths; `planner_heavy` flags any node lacking a ranking function or a
  fallback without a named defeater. Security/health/legal/financial framing stays
  defensive/informational/educational, never advice.
- **Prompt-only ≠ trained.** Running a specialist is structured prompting over a gated spec,
  not a fine-tune. Gains come from decomposition/typing/verification discipline, not new
  model weights.

---

## 10. Command cheat-sheet

```bash
# Gate a KB (exit 0 = pass)
python3 validators/kb_validator.py branches/b13_recursive_planner/kb/htn__decomposition.kb.json --mode dense

# Gate a specialist (run from repo root so grounded_in_kbs paths resolve)
python3 validators/specialist_validator.py branches/b13_recursive_planner/specialist.heavy.json

# Re-gate everything on this track
for kb in branches/*/kb/*.kb.json; do python3 validators/kb_validator.py "$kb" --mode dense --quiet >/dev/null && echo "PASS $kb" || echo "FAIL $kb"; done
for s in branches/*/specialist*.json; do python3 validators/specialist_validator.py "$s" --quiet >/dev/null && echo "PASS $s" || echo "FAIL $s"; done

# Build a KB from a content spec
python3 branches/_forge/kb_forge.py branches/<branch>/kb/_src/<name>.spec.json -o branches/<branch>/kb/<name>.kb.json

# Run the branch lints + tests
python3 -m unittest discover -s branches -p 'test_*.py'
```
