# Heavy-specialist self-application run

Each heavy specialist was loaded **verbatim** into `dist/prompt_template.json`'s
`specialist_prompt_template` (prompt-only mode) and run against the **original intent +
neutralized core** of the very idea it was distilled from (from
`scratchpad/IDEA_BRANCHES_REPORT.md`). No code executed the domains — an agent reasoned
*as* each specialist, strictly per its `role` / `decision_procedure` / `workflow` /
`escalation_triggers` / `conflicts_and_dominance`, grounded in its 3 dense KBs.

This is a self-application test: does a specialist do something real and honest when you
feed it the idea that created it? Summary of standout results:

- **planner_heavy (B13)** ran the planning engine *on itself* and **rejected the idea's own
  "binary recursive halving" default** as non-well-founded (it is a production *method*, not
  a ranking function; it can descend forever over a dense quantity). It supplied the actual
  well-founded measure — a lexicographic `(residual_compound_obligations, open_preconditions)`
  — and adjudicated the idea's 4 self-claims: **(1) unsound default, (2) type-bounded,
  (3) type-bounded, (4) sound** under set-cover semantics.
- **erotetic_heavy (B10)** independently **re-derived the same false presupposition the
  neutralization had stripped** ("a *single* canonical question/answer ontology exists"),
  flagged + reframed it instead of answering, compiled the idea into a 14-node MECE
  sub-question DAG, and showed the central adoption question is the most decision-relevant
  yet *least* answerable (no measured baseline). Held scope: never answered domain content.
- **epistemics_heavy (B11)** built a known/unknown map of itself, separated aleatory vs
  epistemic uncertainty, ranked gaps by value-of-information, and — applying its own
  evidence-label-legality rule — **capped confidence that the protocol is *useful* at LOW**,
  because all its grounding traces to a single independence group (the heuristic-prior
  lineage). It graded the evidence as *unproven*, explicitly not *wrong*.

Cross-cutting meta-finding: each specialist, turned on its own origin, produced a
**calibrated critique of its founding idea** rather than a flattering restatement — exactly
the behavior the verify-not-flatter design intends.

---

## 1. planner_heavy (B13) on "build the recursive plan-decomposition engine"

> Run AS `Recursive Plan Decomposition Engine (heavy)`, grounded in
> `htn__decomposition` + `term__well_foundedness` + `contract__composition_failure`.

- **Typed goal:** `BuildEngine : Idea → CertifiedPlanner` — a compound task whose deliverable
  is itself a function `Planner : Idea → (Plan × TerminationCertificate × CoverageProof)`.
  Machine-checkable success = conjunction of 8 obligations (acyclicity, type-driven arity,
  termination discharged-or-budgeted, obligation set-cover closed, leaves grounded, coverage
  set-cover, every fallback defeater-keyed / SPOF tagged, certificate emitted).
- **Decomposition:** a 5-capability **AND** compound (arity 5 — *not* binary): Decomposer,
  TerminationProver, ContractComposer, FailureHandler, CoverageCloser; each expanded to
  depth 2–3 over the KB node ids. "Split into halves" rejected at the type level.
- **Termination (the headline):** "split into halves" is **not** a sound progress measure —
  it names no codomain/order, halving a dense quantity (`½, ¼, …`) is not well-founded, and
  it has no base case at the minimum. Replacement: rank
  `= (residual_compound_obligations(n), open_preconditions(n))` over a lexicographic
  `(ℕ,<)×(ℕ,<)` (well-founded; base `(0,0)` = grounded primitive leaf), with a
  `DEPTH_CAP/FUEL_BUDGET → DIVERGENCE_DEFEATER` backstop (mandatory because the rank proof
  is a heuristic prior, not machine-verified).
- **Contracts + set-cover:** pre/post/invariant per step; obligation set-cover **closed
  modulo one declared external assumption A1** (an operator catalog exposing typed
  signatures) — declared, not hidden, so legal under assume/guarantee.
- **Fallbacks vs SPOF:** TerminationProver fallback keyed to `no_ranking_function_found`;
  ObligationSetcover fallback keyed to `uncovered_obligation` (bounded repair → backtrack);
  FATAL_BREAK halts-and-compensates (no retry); RankFunction-with-no-budget and
  PrimitiveGrounding operator-gap are **uncompensated SPOFs → escalate** (a "retry" there is
  the forbidden blanket retry).
- **Verdict on the 4 claims:** binary halving = **unsound default**; deterministic/reusable
  = **type-bounded** (only with recorded choice points; greedy selection isn't globally
  deterministic); node fallbacks + fatal-break = **type-bounded** (adequate only under
  no-blanket-retry + SPOF discipline); quality gate validates completeness = **sound** iff
  "complete" means obligation-set-cover closure over a machine-checkable criterion.
- **Escalations raised:** rank proof is heuristic (budget backstop required); operator-gap
  SPOF must be witnessed before lock.

---

## 2. erotetic_heavy (B10) on "an operational ontology of questions and answers"

> Run AS `Question Science & Erotetic Compilation (heavy)`, grounded in
> `erot__question_semantics` + `frame__operationalization` + `qeval__answer_quality`.
> Boundary held: types/scores/decomposes questions; never produces the domain artifacts.

- **Answerability:** the idea is **not one question** — it splits into engineering/empirical,
  conceptual/constructive, and normative answerability classes. Load-bearing construct
  "improves" has no measure; under-specified, not unanswerable. Bound
  construct→indicator→measure for elicitation/framing/expansion/grading/routing before any
  empirical sub-question is scorable.
- **Presuppositions (the headline):** extracted P1–P6; **flagged two FALSE/loaded** and
  reframed rather than answered — **P2** "a *single canonical* Q&A ontology exists" (erotetic
  theory is plural: partition vs answer-set vs inferential vs QUD — no unique fixed point) and
  **P3** "answer-goodness is context-free" (it's QUD-relative; mention-some vs mention-all is
  fixed by the asker's purpose). **P4** "the pipeline is measurably deficient" is *unverified*
  (no baseline) and blocks the central question.
- **Compiled sub-question DAG:** ~14 typed sub-questions across the idea's 5 artifacts
  (MECE, acyclic 1→2→3→{4,5}→6), each tagged interrogative_type + admissible answer_schema,
  marked closed-type (admissible-set scorable) vs open-type (type-conformance + mandatory
  warrant).
- **Quality scores (dimension vectors, not scalars):** Q1.1 "which interrogative types
  partition the space" `[WF .85 / AN .80 / PV .70 / SC .65 / DR .80]`; Q3.1 "which dimensions
  constitute question quality" `[.80/.78/.72/.70/.85]`; **Q6** "does the scaffolding beat
  baseline B0" `[.70/.45/.40/.75/.92]` — highest decision-relevance, lowest answerability
  (no baseline). Effort should go to defining B0 first or any "it improves things" answer is
  an overclaim.
- **Unknown-discovery (5 unasked, decision-relevant):** baseline & counterfactual cost;
  who validates the rubric (evaluator-of-the-evaluator); Goodhart/gaming of the rubric;
  behavior on out-of-taxonomy question types; stopping rule / over-flagging.
- **Escalations:** false-presupposition (P2,P3 → flagged not answered); no-measurable-
  construct (P4/Q6 held until baseline); open-type-without-warrant guarded pre-emptively.

---

## 3. epistemics_heavy (B11) on "a domain-agnostic knowledge-acquisition protocol"

> Run AS `Knowledge Acquisition & Known/Unknown Mapping (heavy)`, grounded in
> `kumap__uncertainty_taxonomy` + `acq__evidence_sourcing` + `sat__structured_probing`.
> Boundary held: grades confidence in evidence; does not assert the protocol works.

- **Known/unknown map:** 6 known-knowns (the *components* are well-attested — Johari/Rumsfeld
  cells, Knight/Walker uncertainty split, GRADE/CRAAP/CERQual grading, Heuer independence,
  SAT probes, Howard VOI) vs 6 known-unknowns (does the *assembled* protocol beat unaided
  cold-start? reproducibility across operators? cost/throughput? domain-agnosticism limit?
  probe yield? label calibration?). Aleatory (operator/topic run-to-run variance, bounded not
  removed) separated from epistemic (each has a concrete evidence channel). 3 assumptions
  registered as conditional knowns (components compose; vague topics operationalizable;
  independence assessable).
- **Gap priority by VOI:** KU1 outperform-baseline (go/no-go) > KU6 calibration (mis-calibrated
  labels are actively harmful) > KU2 reproducibility > KU3 cost > KU4 domain boundary.
- **Acquisition plan for KU1:** would need a randomized protocol-vs-control comparison with
  pre-registered metrics; source types T1–T4; grouped by independence (authors' own work =
  ONE group α however many write-ups; independent replication β; different method family γ);
  citation chains checked for circular reporting. Target tier **high**.
- **Label-legality (the headline):** 'high' is **legal only** with ≥2 sources in distinct
  independence groups. Authors' study alone, or three papers all citing that one trial, or any
  stack of expert opinion → **cap at moderate**; an illegal 'high' is rejected, not silently
  downgraded.
- **Unknown-unknown probes:** KAC (stress the compose/independence assumptions), pre-mortem
  (the labeling scheme could launder overconfidence — its safety feature becomes its hazard),
  outside-view (base rate of "universal methodology kits" surviving is sobering), what-if
  (domains where "primary source"/"independence group" have no analogue). ACH deliberately not
  front-loaded (wrong question type).
- **Confidence + stopping:** **MODERATE** that it's *buildable as an artifact*, **LOW** (legally
  capped) that it's *demonstrably useful* — zero observed comparative evidence and all grounding
  is one independence group. Don't stop acquiring on KU1 (below tier, high VOI); stop on the
  known-knowns (saturated). Graded as *unproven*, explicitly not *wrong*.
- **Escalations:** single-origin evidence for a high-stakes claim; deep/Knightian uncertainty
  in "works on *any* future domain"; pre-mortem-surfaced reframe (false-rigor hazard) → human
  review before deployment.
