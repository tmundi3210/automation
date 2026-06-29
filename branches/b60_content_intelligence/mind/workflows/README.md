# mind/workflows/ — the kernel, dissected into 7 runnable workflows

`KERNEL.md` is the whole mind in one prompt. This folder **arranges** that one prompt into the steps it
actually runs each turn: **seven specialist-owned workflows** (READ-AT-START + the six loop beats), each
with its own machine-facing specialist and a dense protocol *populated by that specialist*, **plus one
separate, frozen gate step (W4b) that has no specialist**. The split is so every phase can be read,
edited, and reasoned about on its own — and so the safety gate is a genuinely separate step (run last,
reviewable) rather than logic buried inside the brain.

> **Why dissect it?** The kernel is the heartbeat; these workflows are the organs. Same loop, but now
> each phase has a single write-owner, a self-contained doc, and an independent specialist you can
> improve in isolation. The runtime is still an LLM reading `KERNEL.md` + `memory/`; these docs are the
> reference each phase expands into.

## The loop, as workflows (run in this order each turn)

| # | Workflow | Owns (writes) | Specialist | Grounded in (3 dense KBs) |
|---|---|---|---|---|
| 0 | [`W0_read_at_start.md`](W0_read_at_start.md) — boot & decision-lossless retrieval | the LOADED manifest; a missing-record INDEX pointer-stub | [`mind_read_at_start`](specialists/mind_read_at_start.specialist.json) | brief · kumap · acq |
| 1 | [`W1_perceive.md`](W1_perceive.md) — bind environment & surface the top question | `attention/questions.md` re-rank; focus line | [`mind_perceive`](specialists/mind_perceive.specialist.json) | erot · frame · qeval |
| 2 | [`W2_recall.md`](W2_recall.md) — retrieve relevant memory & TRANSFER on a new domain | `schemas/abstractions.md` (lift ledger) | [`mind_recall`](specialists/mind_recall.specialist.json) | link · leakage · backtest |
| 3 | [`W3_plan.md`](W3_plan.md) — direction-then-execute | `goals/{direction,plan,subgoals}.md` | [`mind_plan`](specialists/mind_plan.specialist.json) | htn · term · contract |
| 4 | [`W4_act.md`](W4_act.md) — execute one step as a calibrated BET; **drafts + flags only** | `calibration/predictions.jsonl`; `kernel/blackboard/<env>/` | [`mind_act`](specialists/mind_act.specialist.json) | sat · acq · contract |
| 4b | [`W4b_gate.md`](W4b_gate.md) — **THE GATE** (the separated stop/yellow blocker check) | the verdict + disclosure token | **FROZEN — no specialist** | mirrors `../GATE_STEP2.md` |
| 5 | [`W5_reflect.md`](W5_reflect.md) — did it ACTUALLY work? (vs a baseline) & distil a lesson | `experience/*`; a candidate lesson; a WHY-question | [`mind_reflect`](specialists/mind_reflect.specialist.json) | leakage · metric · backtest |
| 6 | [`W6_consolidate.md`](W6_consolidate.md) — write memory, checkpoint, scheduled "sleep" | `episodic/*`, `INDEX.md`, `kernel/{checkpoint,cursor,schedule}`; sleep folds into `semantic/`+`procedural/` | [`mind_consolidate`](specialists/mind_consolidate.specialist.json) | brief · kumap · metric |

KB short names resolve to the dense KBs under `branches/b60_content_intelligence/kb/`,
`branches/b10_question_compiler/kb/`, `branches/b11_knowledge_acquisition/kb/`, and
`branches/b13_recursive_planner/kb/` (all gate-passing). See each spec's `grounded_in_kbs` for full paths.

## The gate is its OWN step (W4b), FROZEN, with no specialist (by design)

The kernel's one hard rule — *publishable output must pass the fail-closed compliance gate* — is its
**own separate step**, **[`W4b_gate.md`](W4b_gate.md)**, run *after* ACT and *before* REFLECT. The
blockers do **not** live inside ACT: ACT (W4) only drafts the artifact and **flags the SAFETY FACTS**
(it makes no safety decision, exactly like `BRAIN_STEP1`); W4b makes the **only** verdict (🛑 STOP / 🟡
YELLOW / 🟢 GREEN) and holds the entire blocker catalog. We deliberately did **not** generate a
specialist for security / ethics / legal / compliance — W4b is **frozen**, mirroring the existing
human-authored `../GATE_STEP2.md` + `../scaffold/orch/gate.py`. `mind_act`'s spec hands off to W4b and
**halts on STOP**; it cannot soften, re-implement, or route around it. This is the brain/gate split —
`BRAIN_STEP1.md` (step 1) → `GATE_STEP2.md` (step 2) — carried into the mind as **W4 → W4b**, so *what
to make* and *whether it's safe to ship* are genuinely two separate steps.

## How this was built (the factory, applied to the mind itself)

1. **Arrange** — the kernel loop was split into the 7 phase-workflows above (READ-AT-START + the six
   loop beats), each with one write-owner, **plus the gate dissected out of ACT into its own frozen
   step (W4b)** so the blockers run last and reviewable, never inside the brain.
2. **Create a specialist for each** — 7 `mind_*.specialist.json` specs, each a machine-facing operating
   spec grounded in 3 real dense KBs; **all 7 pass `validators/specialist_validator.py`**.
3. **Populate by running the specialist** — each workflow doc was authored *by a model reasoning as that
   specialist* (its role + decision procedure), expanding every idea in the phase into a runnable
   protocol with decision rules, a worked example on the seeded run, failure modes, and a handoff.

```
# re-gate the 7 specialists from repo root:
for s in branches/b60_content_intelligence/mind/workflows/specialists/*.specialist.json; do \
  python3 validators/specialist_validator.py "$s" --quiet && echo "PASS $(basename $s)"; done
```

## Single-write-owner map (no two workflows write the same file)

- **W0** → the LOADED manifest (staged for the diary) + a missing-record pointer stub in `INDEX.md`.
- **W1** (Attention) → `attention/*`.
- **W2** (Generalization) → `schemas/*`.
- **W3** (Agency) → `goals/*`.
- **W4** (Kernel-exec + Calibration) → `calibration/predictions.jsonl`, `kernel/blackboard/*`. *Drafts + flags SAFETY FACTS only; makes no safety decision.*
- **W4b** (the GATE, frozen) → the verdict / Clearance Record + (on pass) the signed disclosure token + the outbox packet. *Owns every blocker; mirrors `../GATE_STEP2.md`; never edits ACT's or REFLECT's files.*
- **W5** (Learning) → `experience/*` (+ appends a WHY-question to `attention/surprises.md`).
- **W6** (Memory + Scheduling) → `episodic/*`, `INDEX.md` NOW block, `kernel/{checkpoint,cursor,schedule}`; **sleep-only**: `semantic/*`, `procedural/*`, `_meta/consolidation_log.md`, `calibration/recalibration_map.json`.

## Worked grounding

Every doc is grounded in the seeded run already in `../memory/`: run **R0631**, env **tech_builders**
(transferred from punjab_diaspora), goal **G-12** ("weekly brief: AI-agent hype vs organic"), belief
**B2** (manufactured-hype .70→.74), prediction **P-114** (refuted), candidate lesson **L-23**, schema
**SCH_geo_link_react_brief**. Read a doc top-to-bottom to watch one phase execute on that run.
