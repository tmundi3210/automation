# KB-authoring helper brief (read fully before starting)

You author ONE dense Knowledge Base for the KB-factory at `/home/user/automation`
(cwd = repo root). Work ONLY within your assigned output paths. Do NOT modify any
existing file outside them (never touch `specialists/`, `knowledge_base/`,
`phase_b/`, `validators/`, `schema/`, or other branches).

## Method — read these three first, in order
1. `branches/_forge/kb_forge.py` — the deterministic builder. Its docstring is the
   authoritative content-spec contract. The builder COMPUTES every derived metric,
   all ids, `priority_order`, label maps, and the static schema blocks. You author
   only CONTENT + base metric magnitudes.
2. `branches/_forge/example_htn_gen.py` — a COMPLETE working exemplar generator that
   builds + PASSES the gate. Mimic its shape: compact helpers `node()/b()/pro()/con()
   /dep()/conf()/rel()`, then assemble `N, CQS, GLS, E, CA, EC, WF, DR, ARR, IP` and
   dump a spec JSON. Copy its patterns; change only the domain content.
3. `branches/b13_recursive_planner/kb/htn__decomposition.kb.json` — a passing built
   output, for reference shape.

## Your pipeline
a. Write a generator at the given `gen_path`.
b. It emits a content spec JSON to `spec_path`.
c. Build: `python3 branches/_forge/kb_forge.py <spec_path> -o <kb_path>`
d. Gate:  `python3 validators/kb_validator.py <kb_path> --mode dense --report <report_path> --quiet ; echo exit=$?`
e. ITERATE until validator exits 0, `overall_status:"pass"`, 0 failures. Read the
   report's `checks` for any `fail` and fix. The builder prints a precise SPEC ERROR
   if your content is malformed — fix and rerun.

## Hard count targets (dense mode — gate enforces)
- nodes 19–24 (author 20) · edges 32–40 (author 34–37) · conflict_axes 8–10 (author 9)
- edge_cases 10–12 (author 11–12) · workflow 9–12 (author ~12)
- competency_questions 10–14 (author 12–14, **never exceed 14**)
- dominance_rules 7–12 (author 8–10) · anti_rework_rules 7–12 (author 8–10)
- iteration_protocol 6–10 (author 7–8)

## Content-correctness rules (you own these; builder errors precisely if violated)
- node ids UPPER_SNAKE, unique.
- `node.dependencies` + all **dependency-type edges** must be jointly ACYCLIC (a DAG /
  layering). Other edge types (`constraint, conflict, causal, sequence, feedback,
  similarity`) may cross freely — use them to reach the edge count without cycle risk.
- every `node.dependencies` / edge `from`,`to` / `competency_question_refs` /
  workflow `node_ref` / edge_case `affected_nodes` / iteration_protocol `nodes` entry
  must reference a REAL node id (or real CQ id).
- each node needs a `base` with all 9 metrics + `acceptance_test_pass_rate` +
  `dependency_gate_pass_rate` + `prior_importance` + `evidence_confidence` +
  `failure_rate` + `downside_weight` + `uncertainty_interval:[lo,hi]` + `update_signal`.
  All values in [0,1], 0<=lo<=hi<=1.
- OBLIGATIONS: any node with `risk_if_wrong >= 0.80` MUST have non-empty
  `acceptance_tests`; any node with `cross_topic_coupling >= 0.75` MUST have non-empty
  `revisit_triggers`. Easiest: always give every node 1–2 of each (the exemplar does).
- conflict-type edges MUST carry a non-empty `resolution_rule` (3–4 such edges).
- ~10+ dependency edges mirroring `node.dependencies`, plus ~8 cross-cutting
  non-dependency edges to reach the count.
- NEVER emit these placeholder tokens in any text (gate scans raw text):
  `precise_pro, precise_con, concrete_example, NODE_A, NODE_B, SHORT_ID, TODO,
  placeholder, FAMILY_ID, method_catalog, [DOMAIN], CHECK_ID, ARTIFACT_ID`.
- competency_questions: `{id:"CQ_01".., question, must_be_answerable_from:[...],
  acceptance_condition, covered_by:[real node ids]}`. 12–14 total; each node refs 1–2;
  every CQ covered by >=1 node.
- `source_citation`: cite REAL, well-known literature for the domain. Content must be
  genuine, specific domain knowledge — concrete definitions, realistic pros/cons/
  failure_modes/acceptance_tests. No filler.

## Return (your final message = the report, terse)
`kb_path`, validator `exit` code, the 9 section counts, `overall_status`. If any check
still fails after your best effort, return the failing `check_id`(s) + `detail`
verbatim. Do NOT claim success unless validator exit = 0.
