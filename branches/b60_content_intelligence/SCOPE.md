# B60 — Content-Intelligence Engine: scope (Phase 1 of the pipeline)

This is the **scope** output of running the original idea through the pipeline. The idea
("scrape public info → understand it → link 2–3 related things → generate derivative content
for an audience, driven by a brain that tests itself on real examples") was run **prompt-only**
through the three existing heavy specialists (`planner_heavy`, `erotetic_heavy`,
`epistemics_heavy`) plus a schema designer. Raw findings: `demo/scope_findings.json`.

The point of this phase is the author's own step 1 — *"first plan what is information, then think
about the different aspects."* The specialists did that, and (by design) **verified rather than
flattered**: all three independently surfaced the same defects.

## The idea, typed

`Plan(G)` where `G = ingest(item) → understand(item) → link(2..3 related nodes) → generate(image_prompt, story_prompt, audio_brief?) → emit(→ separate downstream agent session)`, governed by a
controller ("brain") running `build → test_on_unseen_historical → measure → revise`.

Two recursions live inside it:
- **R1 — attention web:** `expand(region) → split_by_interest(niche) → top_k(5)`, recursing over regions in **decreasing** size/hype. **Provably terminates** (finite taxonomy + decreasing order + depth cap).
- **R2 — eval loop:** `revise(model) until results are good`. **Does not terminate as written** — "good" has no metric and "revise the whole model" resets state instead of decreasing a measure. This is the single biggest defect.

## The six author capabilities → coverage

| # | Capability | Status after scope |
|---|---|---|
| 1 | INFO-FIRST understanding | groundable; needs a defined "must-preserve" field set (see FP2) |
| 2 | MAP / attention-web (region→niche→top-5, decreasing) | R1 sound; "medium-tier is the opportunity" is an **unproven prior**, not a given (FP1) |
| 3 | LINKING (2–3 related, geo-relevant) | **well-formed core** — arity 2..3 + geo-relevance predicate are typeable/enforceable |
| 4 | SIGNAL reading (sentiment, bot, hype, dense brief) | partial: bot-"real?" is **only a calibrated probability**, never boolean (FP4); hype needs source-independence dedup (FP7) |
| 5 | GENERATE (image/story/audio prompts, reuse-but-modify) | groundable, but gated by the legal contract; "modify" is **not** a legal safe-harbor (FP6) |
| 6 | BRAIN + EVAL loop | **blocked** until R2 gets a frozen metric + baseline + leakage-controlled holdout (FP3, FP5) |
| + | Legal/consent gate (cross-cutting) | **dominates all of the above**; irreversible once published; fail-closed, pre-emit, human-reviewed |

## The seven findings (verdicts on the idea's own claims)

1. **FP1 — "medium-tier is the opportunity."** Unproven heuristic prior. It is exactly the hypothesis the eval loop should *falsify*, not assume. Make it a **measured variable** (top vs medium vs long-tail).
2. **FP2 — "lossless AND token-efficient summary."** Internally contradictory: lossless compression is bounded below by source entropy. Reframe as **rate-distortion** — pick must-preserve vs droppable classes at a fixed budget ("decision-lossless", not literally lossless).
3. **FP3 — "test on examples it didn't see during design."** Leakage risk: *you* (and a pretrained model) already know the outcomes of famous events. Needs a **strictly post-cutoff, pre-registered, blind** holdout, else the backtest measures memorization.
4. **FP4 — "decide if the comments are real."** No public ground truth → unsound as a boolean. Downgrade the contract to **calibrated probability + provenance**, with a false-positive cost.
5. **FP5 — "repeat until results are good."** No success construct → non-terminating, self-confirming. Bind **metric + baseline + threshold + stopping rule**.
6. **FP6 — "reuse meme formats; don't copy, modify."** Over real public figures + political news this collides with right-of-publicity and defamation. "Modify" is not a safe harbor — **gate it**.
7. **FP7 — "read 5 platforms = corroboration."** The platforms often share one upstream origin (one press release, one astroturf campaign, cross-posted screenshots). **Deduplicate to distinct origins** before trusting a hype signal.

## Gap priority (by value-of-information, from epistemics_heavy)

1. **Legality** of synthetic likeness/voice + political/joke association (jurisdiction-dependent; escalate to counsel — *can void the use case regardless of how well the system works*).
2. **Define "lossless dense summary"** operationally (cheap; unblocks the understanding stage).
3. **Define "good" + baseline (KU6) and leakage control (KU4)** — jointly make the eval loop valid.
4. **Test the medium-tier insight (KU1)** — high value, but only once 2–3 give it a valid measurement substrate.
5. **Astroturf detectability (KU2)** — scope to "raise the floor vs labeled benchmarks", don't chase a full solution (partly irreducible / aleatory).
6. **Platform data access (KU7)** — resolve early; can hard-block signal reading regardless of model quality.

**Confidence (capped, honestly):** *buildable* MEDIUM-HIGH (components all exist); *useful* LOW-to-MEDIUM and **unverifiable without a baseline** (the two load-bearing claims — medium-tier and "good" — are undefined, and a legal gate sits in front of usefulness). No "high" label is legal here because zero evidence was acquired in this pass.

## What gets built (Phase 2) — the 5-specialist scaffold

The scaffold covers all six capabilities **plus** the legal gate, folding in every escalation above.
Each is a **heavy** specialist grounded in **3 freshly generated dense KBs** through
`kb_validator --mode dense` (same construction as the validated 28-set).

| Specialist | Covers | 3 dense KBs |
|---|---|---|
| **`salience_heavy`** | MAP + LINK (cap. 2, 3) | `geo__attention_topology`, `niche__audience_segmentation`, `link__entity_relevance` |
| **`signal_heavy`** | UNDERSTAND + SIGNAL (cap. 1, 4) | `auth__inauthenticity_detection`, `hype__organic_baseline`, `brief__rate_distortion_summary` |
| **`creative_heavy`** | GENERATE (cap. 5) | `scene__multi_entity_composition`, `promptgen__downstream_brief`, `novelty__derivative_vs_copy` |
| **`eval_heavy`** | BRAIN + EVAL (cap. 6) | `leakage__holdout_design`, `metric__frozen_preregistration`, `backtest__outcome_validity` |
| **`compliance_heavy`** | LEGAL GATE (cross-cutting) | `pubrights__likeness_voice`, `defamation__false_light`, `platform_tos__synthetic_disclosure` |

The five specialists **are** the "brain"; the existing `planner_heavy` / `erotetic_heavy` /
`epistemics_heavy` are the reasoning core that plans, frames, and maps known/unknowns. The
controller loop that wires all eight together is documented in `BRAIN.md` (Phase 3).

`compliance_heavy` encodes a **screening heuristic that escalates to qualified counsel** — it is
not legal advice and never auto-approves; it is fail-closed and pre-emit.

## The schema the author asked for

`schema/information_node.schema.json` — one record per ingested public-info item (the
"schema to think about the information first"). It is a closed (`additionalProperties:false`),
machine-facing JSON-Schema with blocks for provenance, what/who-behind, place (region hierarchy +
diaspora for cross-border links), known-for (interest domain/subgenre for the niche split),
audience, cross-platform `signal` (incl. an `authenticity` sub-block — "are the comments real?"),
`hype` (with a `MEDIUM_OPPORTUNITY` tier and an authenticity-adjusted score), a short `critical_read`,
typed `links` (with `geo_keys` enforcing geo-relevance), a `machine_summary` (dense + a
`schema_hint` that makes it decision-lossless), and a `safety_flags` gate that travels in-band to
the downstream session. Field-by-field rationale and the safety register are in
`demo/scope_findings.json`.
