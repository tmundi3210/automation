# ORIGIN_EXCHANGE.md — the first message of the whole project (the core motto), and the first response

_Archived 2026-07-02 for T13 re-analysis, per the owner's clarification: the target is NOT the sweater-vertical kickoff (that one is archived in `INITIAL_EXCHANGE.md`) but the project's TRUE first exchange — "about making a specialist... the core motto of this project."_

## Provenance — what survives, honestly

- **[FACT]** The project's first commits are `2a64eca` ("Initialize repository") and `0330acf` ("Scaffold orchestrated KB-generation automation"), both **2026-06-23 ~07:00 UTC**, on branch `claude/eager-wozniak-74rlgj`.
- **[FACT]** The chat transcript now on disk begins 2026-06-24 (post-compaction). **The verbatim text of the owner's first chat message did not survive compaction and is unrecoverable.** Nothing here pretends otherwise.
- **[FACT]** What DOES survive verbatim, and is stronger evidence than chat text:
  1. **The owner's supplied kernel** — `schema/kb_generator_v1.4.1.txt` (1,924 lines, "EXPERT-SYSTEM JSON KNOWLEDGE BASE GENERATOR — VERSION 1.4.1 WITH DOMAIN ADAPTER MODE"), vendored verbatim in `0330acf` (PLAN.md §7: "schema vendored"). This is the owner's own artifact and the seed of everything since.
  2. **The owner's machine-facing directive** — `schema/HEADER.txt`: *"losslessly compressed, token-efficient, information-dense, fully detailed, machine-facing; optimized for model parsing over human readability."* PLAN.md records it as a **user directive**.
  3. **The first response** — the scaffold itself: `PLAN.md` + `README.md` + `validators/kb_validator.py` + `validators/metrics.py` + `taxonomy/knowledge_searcher.taxonomy.json` + `prompts/*.md`, all readable at `git show 0330acf:<path>`.
- **[ESTIMATE: reconstruction from the oldest compaction summary + the scaffold's own text, HIGH confidence on substance, zero claim on wording]** The first message asked, in substance: *take this KB-generator schema and build an orchestrated automation around it — an orchestrator that plans and gates but does not generate in-context; helper sub-agents that each run one schema job; a deterministic quality/quantity gate; first build a foundational "Knowledge Searcher" KB (how/where to find knowledge), then a reusable pipeline that turns any raw idea into neutralized, academically-grounded, dense, gated KBs and distills a specialist per KB.* The oldest surviving summary states the inherited intent as: **"Knowledge-architecture + runnable scaffold for an orchestrated system of fine-tuned specialist LLMs."**

## The core motto (as the first response encoded it)

From `README.md` @ `0330acf`, verbatim:

> An orchestrated pipeline that turns the **Expert-System JSON KB Generator** schema (`schema/kb_generator_v1.4.1.txt`) into a repeatable factory:
>
> 1. **Phase A** builds a foundational *Knowledge Searcher* KB — a machine-facing map of where/how authoritative knowledge is located, retrieved, evaluated, and synthesized.
> 2. **Phase B** takes any raw idea, neutralizes it, converts it to an academic taxonomy (grounded by Phase A), generates grouped dense KBs, and distills a **specialist** per KB.
>
> - **One orchestrator** (a Claude session) holds the plan and enforces gates. It does **not** do heavy generation in-context.
> - **Helper sub-agents** each run one schema job, write JSON to a file, and return a compact report. This keeps the orchestrator lean and lets work fan out.
> - A **deterministic gate** (`validators/kb_validator.py` + `validators/metrics.py`) checks every output for quality (schema/reference/formula integrity) and quantity (density-mode counts, token size) before it is accepted.

And the first response's load-bearing design decisions (from `PLAN.md` @ `0330acf`):

1. **Roles** — orchestrator owns plan/taxonomy/gates/commits, "stays lean — never generates KBs in-context"; helpers run ONE schema job each and return compact reports.
2. **Header reconciliation** — the user directive (machine-facing header on every output) conflicts with the schema's strict-JSON veto for KB outputs; resolved by the JSON-validity veto winning: pipeline-owned artifacts carry `"_directive"`, KB outputs stay pure JSON with the directive injected into the job prompt + recorded in sidecars.
3. **Deterministic gate** — `kb_validator.py --mode dense`: quantity bands + JSON/reference/formula integrity (tol 0.02), exit 0 = accept; helper self-repairs once, else escalate. **Structure + math only; prose is ungated.**
4. **Calibration before mass generation** — a GROUP_SIZE sweep (1 → 2 → combined → 4) to fix batch granularity empirically before Phase A mass generation, rather than guessing.
5. **Phase B as the reusable product** — idea → neutralize → academic taxonomy → (optional adapter schema) → grouped dense KBs → one specialist per validated KB.

## The lineage this seeded (for the hindsight lens)

Kernel (owner) → scaffold `0330acf` → calibration (GROUP_SIZE=1, "natural-fill grain rule") → Phase A: 10 → 28 specialists / 84 dense KBs (`knowledge_base/`, `specialists/`, ROUTER) → verbatim bundle (`dist/`) → applied verticals on the same factory: Turlock business catalog + sector specialists (S1–S11), movie-scoped brain, then the **sweater vertical** (T0–T12: 9 specialists, 27 KBs, BRAIN controller, GATED conditional-GO decision memo, T12 self-improvement loop). Every later stage kept the kernel's two axioms: **orchestrator never ingests, gates never trust prose.**

## What T13 does with this

1. **Critique** — was the scaffold the CORRECT and MOST INTELLIGENT response to the kernel? (Judged design decisions 1–5 above, plus what a top-1% systems designer would have added up front.)
2. **Improve** — the system's own specialists (from the 28 it later built: `kr` knowledge-representation, `method` methodology/measurement, `loops` iteration/self-improvement, `infosci` organization/retrieval) audit the factory core — schema contract, validator gate, loop protocol, taxonomy/grouping — and propose mechanical improvements.
3. **Pro test** — fault-injection against the validator gate (seed broken formulas / dangling refs / band violations into KB copies; the gate must reject every one) + an end-to-end mini Phase-B run on a neutral micro-domain + full-repo re-gate.
