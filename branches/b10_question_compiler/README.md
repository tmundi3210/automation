# B10 — Erotetic Compiler (Question Science / Question-Framing)

The **erotetic compiler** takes a raw, vague, or compound need and *lowers* it into
one or more **typed question-objects** — machine-checkable structures that make a
question's type, presuppositions, admissible-answer shape, stopping rule, decision
relevance, and decomposition explicit. It then **validates and scores** those
objects deterministically.

It **structures and scores** questions and candidate answers. It **does not
fabricate domain answers** — domain content routes to the relevant domain
specialist.

## Files

| File | Role |
|------|------|
| `question_object.schema.json` | JSON Schema (draft 2020-12) for a typed question-object. |
| `erotetic_lint.py` | Deterministic validator + question-quality rubric. CLI gate. |
| `examples/q_polar.pass.json` | A clean closed-type (polar) question that PASSES. |
| `examples/q_loaded.fail.json` | A loaded wh/why question that FAILS (false presupposition + cyclic sub-question DAG). |
| `test_erotetic_lint.py` | unittest suite: pass fixture passes, fail fixture fails and names the violated rules, closed-vs-open branching. |
| `specialist.json` | Injection-ready meta-specialist `erotetic`, composing ling, argue, kr, infosci, method, psych. |

## The closed-vs-open verification distinction (core design point)

The naive view is that *verifying an answer = checking membership in the question's
admissible answer set*. That is correct **only for CLOSED interrogative types**:

- **CLOSED** — `polar` (yes/no/indeterminate), `alternative` (enumerated disjuncts).
  The admissible answer set is **finite**, so an answer is verified by **strict
  set-membership**.

- **OPEN** — `wh`, `why`, `how`, `quantitative`, `definitional`, `counterfactual`.
  The admissible answer set is **open-ended**. You cannot enumerate it, so
  membership is the wrong test. Instead an answer is scored by:
  1. **type-conformance** — the `answer_schema` declares the right shape for the
     type (e.g. `why` -> `explanans_schema`, `how` -> `ordered_step_schema`,
     `quantitative` -> `unit` + `range`), and
  2. **warrant-presence** — a warrant slot is **required** (`warrant_required: true`).

`erotetic_lint.py` **branches on this distinction**:
- closed types must carry a usable `admissible` set (and `polar` must be a subset of
  `{yes,no,indeterminate}`); they are checked by membership;
- open types must carry their type-specific conformance field(s) **and** a required
  warrant slot, and must **not** carry an `admissible` set (that would be a category
  error).

## Validator checks

Hard checks (any failure → nonzero exit): JSON validity; schema shape; legal
`interrogative_type`; **answer_schema consistency with the declared type (closed vs
open branching)**; presuppositions present with legal `validity`; **no
`validity=="false"`** (loaded question → refuse-or-repair); non-empty
`resolution_criteria` and `decision_relevance`; sub-question `depends_on` references
resolve; sub-questions form a **DAG**; answer scorability (closed → admissible set;
open → warrant slot).

It also computes a deterministic **question-quality rubric** — `well_formedness`,
`answerability`, `presupposition_validity`, `scope_specificity`,
`decision_relevance`, each scored 0 / 0.5 / 1 by a mechanical rule, plus a
`composite`. The rubric is diagnostic and never causes a hard failure.

## How to run

Lint a question-object (CLI mirrors `validators/kb_validator.py`):

```bash
cd branches/b10_question_compiler
python3 erotetic_lint.py examples/q_polar.pass.json            # exit 0 = PASS
python3 erotetic_lint.py examples/q_loaded.fail.json           # exit 1 = FAIL
python3 erotetic_lint.py examples/q_polar.pass.json --report report.json --quiet
```

Run the tests:

```bash
cd branches/b10_question_compiler
python3 -m unittest test_erotetic_lint -v     # exits 0 when the suite passes
```

## How it composes the existing specialists

`specialist.json` is an injection-ready meta-specialist (`specialist_id: erotetic`)
that composes gate-passed specialists:

- **ling** — erotetic/answerhood semantics, presupposition triggers/projection,
  interrogative typing.
- **argue** — answer appraisal (RSA defeaters), Critical Questions, fallacy/bias
  screening, the complex-question (many-questions) fallacy for loaded questions.
- **kr** — competency-question framing and ontology evaluation.
- **infosci**, **method**, **psych** — support (organization; measurement/validity
  and "what counts as a sufficient answer"; framing/anchoring bias).

Its `grounded_in_kbs` is the deduplicated union of those specialists' own
`grounded_in_kbs` (18 existing `knowledge_base/knowledge_searcher/*.kb.json` paths,
all verified to exist).

## Boundary note

The compiler **structures and scores**; it **does not produce domain answers**. It
assigns interrogative types, surfaces and validity-labels presuppositions, builds
type-correct answer schemas, writes resolution criteria and decision relevance,
decomposes questions into DAGs, runs `erotetic_lint`, and appraises answers for
conformance and warrant. Domain content is delegated to the composed specialists.
Numeric estimates are heuristic unless grounded in observed data; loaded or
contested framings are refused, repaired, or escalated rather than answered.
