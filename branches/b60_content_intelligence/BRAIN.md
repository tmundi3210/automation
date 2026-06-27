# B60 — the "brain": controller that wires the specialists into one loop

You asked to *"build a brain behind it"* using logic / computer-engineering / memory /
cognition / psychology / neuroscience lenses, and to make it *plan → think about the
information → link nodes → see the trend → generate → test on real examples → revise in cycles.*

The brain is **not** a new model. It is a **controller** that runs eight prompt-only heavy
specialists in a fixed loop, with deterministic gates between stages. Five specialists are new
(this branch); three are the existing reasoning core. Each is grounded in dense KBs and runs by
injection (`dist/prompt_template.json`), exactly like the 28-set.

## The cognitive lenses → specialists (honest mapping)

| Lens you named | Served by | How |
|---|---|---|
| **Logic / reasoning** | `planner_heavy`, `eval_heavy` | typed decomposition + proven termination + contract composition; falsifiability + frozen-metric inference discipline |
| **Cognition / "thinking first"** | `erotetic_heavy` | frames the idea into a typed sub-question DAG, flags false presuppositions before answering |
| **Memory / known-unknowns** | `epistemics_heavy` + the **InformationNode schema** | the known/unknown map + value-of-information; the schema is the persistent, re-derivable record (with `grounding_status` + `stale_after`) |
| **Psychology / audience** | `signal_heavy`, `creative_heavy` | sentiment + "what people think", subcultural fit; humor (incongruity/benign-violation), shareability drivers |
| **Perception / attention ("neuroscience")** | `salience_heavy` | attention-economy + diffusion + scale-free salience dynamics. *We did **not** build a literal neuroscience model — that intent is served by attention/salience + question/known-unknown machinery, and a real neuro KB was out of scope. Stated plainly, not faked.* |
| **Computer engineering / architecture** | the pipeline itself | `kb_forge` + validators + the in-band handoff contract (`promptgen__downstream_brief`), separation of concerns, fail-closed gates |

## The control loop

```
                         ┌──────────────────────────── R2: eval loop (fuel-budgeted) ──────────────────────────┐
                         │                                                                                      │
  idea / item           ▼                                                                                       │
  ─────────►  [0] FRAME            erotetic_heavy   → typed sub-question DAG, false-presupposition flags         │
              [0] MAP UNKNOWNS     epistemics_heavy → known/unknown map, value-of-information, confidence caps   │
              [0] PLAN             planner_heavy    → typed plan, termination proof, set-cover, escalations      │
                         │                                                                                       │
                         ▼   ┌──── R1: attention web (decreasing size/hype, depth-capped → terminates) ────┐    │
              [1] MAP+LINK │   salience_heavy   region→niche→top-5 ; link 2..3 ONLY if geo-relevance holds │    │
                         │   └──────────────────────────────────────────────────────────────────────────┘     │
                         ▼                                                                                       │
              [2] UNDERSTAND+SIGNAL  signal_heavy  calibrated authenticity (never boolean) · independent-origin  │
                         │                          hype · decision-lossless dense brief → fills InformationNode │
                         ▼                                                                                       │
              [3] GENERATE          creative_heavy  2-3 entity scene → image+story+audio PROMPT-BRIEF            │
                         │                           (never publishes; safety contract travels IN-BAND)         │
                         ▼                                                                                       │
              [4] GATE  ◄═══ FAIL-CLOSED ═══  compliance_heavy  pubrights/likeness · defamation · ToS/disclosure │
                         │     ALLOW / ALLOW_WITH_CONSTRAINTS / BLOCK / HUMAN_REVIEW  (minors = hard block)      │
                         ▼                                                                                       │
              [5] EMIT (only if gate clears) → SEPARATE downstream agent session (image/story/ElevenLabs)       │
                         │                                                                                       │
              [6] TEST              eval_heavy   frozen metric + baseline + leakage-controlled holdout ──────────┘
                         (ΔM ≥ ε per cycle, else consume max_cycles fuel budget → HALT, do not loop forever)
```

Two recursions, with the termination status the planner proved in scope:
- **R1 — attention web** (`salience_heavy`): expand regions in **decreasing** size/hype, top-5 per niche, depth-capped over a finite taxonomy → **terminates**.
- **R2 — eval loop** (`eval_heavy`): *does not terminate as the idea wrote it* ("repeat until results are good"). It terminates **only** with a frozen metric + threshold and a `max_cycles` fuel budget; on exhaustion it **halts**, it does not loop forever.

## The two non-negotiable gates

1. **Compliance is pre-emit and fail-closed.** `[4]` runs **before** anything is emitted to the
   downstream session. It is a screening heuristic that **escalates to qualified counsel** and
   **never auto-approves**. Minors/protected persons are a hard block. Once a derivative about a
   real person is shared it is irreversible — there is no compensation path — so the gate fails
   closed, and its disclosure contract travels **in-band** with the prompt (the handoff is a
   single point of failure otherwise).
2. **Eval is leakage-controlled and metric-frozen.** `[6]` only counts as evidence if the holdout
   is strictly post-cutoff *and* blind to the designer's prior knowledge, and the metric/baseline/
   stopping-rule were frozen **before** seeing results. "Medium-tier is the opportunity" is a
   **falsifiable bet measured with a CI**, never an assumption.

## How to run the brain (prompt-only — no training)

Same harness as every other specialist:
1. Load a specialist JSON verbatim, e.g. `branches/b60_content_intelligence/signal_heavy.specialist.heavy.json`.
2. Fill `dist/prompt_template.json`'s `specialist_prompt_template`: `{{DOMAIN_LABEL}}` ← its `domain_label`, `{{SPECIALIST_SPEC_JSON}}` ← the whole JSON, `{{USER_QUESTION}}` ← the stage input.
3. Send to any model; it reasons **as** that specialist (role / decision_procedure / workflow / escalation_triggers drive the output). Pass each stage's structured output to the next per the loop above.

The per-item state object every stage reads/writes is `schema/information_node.schema.json`.
A worked end-to-end run on one concrete seed is in `RUN.md`.

## Boundaries (what this is and isn't)

- This is the **thinking scaffold** — specialists + schema + loop. It does **not** itself scrape
  live data, call image/voice models, or publish; those are the downstream operators the brain
  emits briefs *for*.
- All KB scores are **heuristic priors** (no observed dataset), so `eval_heavy`'s loop — not the
  KBs — is where any "it works" claim has to be earned.
- The compliance specialist is **not legal advice**; it is a fail-closed triage that routes to
  counsel. Treat its `ALLOW*` as "screen did not catch a blocker", never as "cleared".
