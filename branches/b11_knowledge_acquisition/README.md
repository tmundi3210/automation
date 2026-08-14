# B11 — Knowledge Acquisition / Known-Unknown Mapping

Meta-specialist (`specialist_id: epistemics`) that turns a vague topic into a
self-updating **competency-question (CQ) ledger**, derives known/unknown views
from it, runs a Structured-Analytic-Technique (SAT) probe battery to surface
unknown-unknowns, and GRADE/CERQual-labels every claim under a strict
evidence-label legality gate.

It **composes** five existing gate-passed specialists rather than re-deriving
their content: `method` (epistemic aims, operationalization), `evsynth`
(GRADE/CERQual grading, search strategy), `scholcomm` (provenance, CRAAP/SIFT),
`libarch` (scope contracts, curation), and `argue` (reused SATs + independence
discipline).

## Files

| File | Role |
|------|------|
| `cq_ledger.schema.json` | JSON Schema for the one ledger (the single source of truth). |
| `sat_battery.json` | Named SAT probe battery; each probe spawns a known_unknown CQ. |
| `source_quality_rubric.json` | Parameterized authority/recency/relevance/independence/corroboration rubric. |
| `acquisition_ledger.py` | Deterministic tool: `validate`, `derive-views`, `score-source`, `list-probes`. |
| `examples/ledger.pass.json` | Valid ledger (a known_known backed by 2 independent sources + several known_unknowns). |
| `examples/ledger.fail.json` | FAILS evidence-label legality (a `high` label whose 2 sources share one independence_group). |
| `test_acquisition_ledger.py` | unittest suite (10 tests). |
| `specialist.json` | Injection-ready meta-specialist (`epistemics`). |

## Run

```bash
cd branches/b11_knowledge_acquisition

# validate (default action; exit 0=pass, 1=fail, 2=bad invocation)
python3 acquisition_ledger.py examples/ledger.pass.json          # -> exit 0
python3 acquisition_ledger.py examples/ledger.fail.json          # -> exit 1

# derived known/unknown matrices over the ONE ledger
python3 acquisition_ledger.py derive-views examples/ledger.pass.json

# print the SAT battery and the CQs each probe would spawn for a topic
python3 acquisition_ledger.py list-probes --topic "your topic here"

# score a single source (optionally in the context of a CQ's source set)
python3 acquisition_ledger.py score-source some_source.json --ledger examples/ledger.pass.json --cq CQ_001

# tests
python3 -m unittest test_acquisition_ledger -v
```

The CLI mirrors `validators/kb_validator.py`:
`python3 <tool>.py <input.json> [--report out.json] [--quiet]`, exit 0 = PASS,
nonzero = FAIL, with a machine-facing `checks` list in the report.

## Design: one ledger, derived views

There is **exactly one** CQ ledger. The `known_known_matrix` and
`known_unknown_matrix` are **derived views** computed on demand by
`derive-views` — pure functions over the ledger, never separately maintained
tables. This eliminates the desync risk of three parallel tables. (The validator
and the meta-specialist both block any attempt to persist a matrix as
standalone state.)

**unknown-unknowns are operationalized, not enumerated.** Trying to list "what
we don't know we don't know" is impossible. Instead, the SAT battery is a set of
probe-*operators*; applying a probe to a topic **spawns a new `known_unknown`
CQ** appended to the ledger with `spawned_by = probe_id`. That converts a blind
spot into a watched, falsifiable open question.

## SAT reuse note (argue overlap)

The existing **argue** specialist already references Structured Analytic
Techniques — specifically **Analysis of Competing Hypotheses (ACH)** and
**Key Assumptions Check** (via its claim/evidence-appraisal defeater + critical-
question + unstated-assumption capabilities). Per the adversarial-review
correction, B11 **does not reinvent these**: `PRB_ACH` and `PRB_KAC` are marked
`owned_by: "argue"` with an explicit `reuse_citation`. B11 only **adds** the
genuinely missing probes, marked `owned_by: "new"`:

- `PRB_WHATIF` — What-If Analysis
- `PRB_PREMORTEM` — Pre-Mortem Analysis
- `PRB_QOIC` — Quality-of-Information Check
- `PRB_INDSIG` — Indicators / Signposts

## Evidence-label legality rule

Mirroring the repo ethos (`kb_validator.py` forbids `observed` /
`experimentally_validated` labels without data), B11 **rejects a strong evidence
label that lacks the required independent corroboration**:

- `high` → requires a non-null claim **and ≥ 2 sources in DISTINCT
  `independence_group`s** (genuinely independent corroboration).
- `moderate` → requires a non-null claim and ≥ 1 source.
- `low` / `very_low` / `ungraded` → no source-count obligation.
- a `known_unknown` → must have `claim == null` and a label in
  `{very_low, ungraded}`; a `known_known` must have a non-null claim.

Two sources that share an `independence_group` (e.g. a vendor press release and
a blog quoting it) count as **one** independent group — common-source dependence
is not independent corroboration. This is exactly what `examples/ledger.fail.json`
demonstrates, and `validate` rejects it.

## Boundary note

This capability is **informational only**. Evidence labels are GRADE/CERQual
**claims-about-evidence** (confidence in a body of evidence), **not ground
truth** — a `high`-labeled claim can still be wrong and is always defeasible with
disclosed revision conditions. B11 does not make decisions, give professional
advice, or generate domain knowledge itself; domain depth lives in the five
composed specialists' KBs. Source independence is only as good as the provenance
vetting (`scholcomm`) that assigns the `independence_group` labels.
