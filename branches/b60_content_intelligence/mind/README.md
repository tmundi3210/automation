# `mind/` — the LLM-run cognitive + memory system

This is the **brain reborn as a mind**. The earlier b60 brain (`../scaffold/`) was a clever but
frozen skeleton — the intelligence analysis found it doesn't learn, doesn't generalize, has weak
agency, calibrates once, and forgets everything between runs. `mind/` fixes that — **not with more
Python, but with an LLM that operates a persistent memory** by reading and writing files.

A real LLM (with web/file/image/audio tools) *is* the reasoner. It **sets a direction, then
executes step by step** — a series/parallel mix chosen per situation — and **keeps a diary,
checkpoints, and a schedule** in `memory/` so it never forgets across sessions. There is **no
Python pipeline to run**; you run it by giving an LLM the kernel prompt and the `memory/` folder.

## How it's run (no code)

1. Point a capable, tool-using LLM at this folder.
2. Paste **`KERNEL.md`** as its operating instructions.
3. It reads `memory/INDEX.md` first, does one turn of the loop, and writes back to `memory/`.
4. Next turn (same session or days later) it re-reads `memory/` and continues exactly where it left off.

```
You ─paste→ KERNEL.md ─runs→ LLM ─reads/writes→ mind/memory/  ─(diary, checkpoints, schedule)→ never forgets
```

## The seven faculties (each a specialist closed a gap)

| Faculty | Specialist | Closes the gap |
|---|---|---|
| **Kernel** | `orch` | the turn loop, series/parallel, checkpoints, scheduling, multi-environment |
| **Agency** | `planner_heavy` | sets its own goals & direction, plans, replans on triggers |
| **Attention** | `erotetic_heavy` | decides what to look at / ask / explore next (curiosity) |
| **Calibration** | `epistemics_heavy` | tracks how right it tends to be and recalibrates over time |
| **Learning** | `eval_heavy` | turns experience into reusable lessons — honestly (no self-deception) |
| **Generalization** | `reason` | transfers skill to new domains + the unified "mind" loop |
| **Memory** | `signal_heavy` + `epistemics` + `orch` | the diary/tiers/retrieval that hold it all together |

Read order: **`KERNEL.md`** (the operating prompt) → **`MEMORY.md`** (how memory works) →
**`FACULTIES.md`** (each faculty in detail) → **`GAP_MAP.md`** (which intelligence criterion each
mechanism closes, honestly).

### `workflows/` — the kernel dissected into 7 runnable phase-workflows

`KERNEL.md` is the mind in one prompt; **`workflows/`** arranges that one prompt into the **seven
separate workflows** it runs each turn (READ-AT-START + the six loop beats), gives **each its own
gate-passing specialist** (`workflows/specialists/mind_*.specialist.json`), and **populates each** with
a dense, runnable protocol authored *by that specialist*. The **safety gate is frozen**: there is no
security/ethics specialist — `mind_act` defers every safety decision to `../GATE_STEP2.md` and halts on
STOP. Start at **`workflows/README.md`** to edit any single phase in isolation.

## The memory store (`memory/`)

```
memory/
  INDEX.md                 # read FIRST every turn — the map of what it knows
  kernel/                  # cursor, turn_log, checkpoint, schedule, environments, blackboard/
  goals/                   # goals, direction, plan, subgoals          (agency)
  attention/               # questions, surprises, explore_log, budget (attention)
  calibration/             # predictions, recalibration_map, report    (calibration)
  experience/              # episodes, outcomes, reflections, ledger    (learning, raw)
  schemas/                 # schemas, abstractions, transfer_log        (generalization)
  episodic/                # journal/<date>.md (the DIARY), timeline    (memory)
  semantic/                # world/, audiences/, glossary (beliefs)     (memory)
  procedural/              # playbooks/, lessons (promoted skills)      (memory)
  _meta/                   # retention policy, consolidation_log
```

A worked example run is already seeded in `memory/` (a "weekly brief: AI-agent hype vs organic"
turn) so you can see a populated diary, belief, prediction, and plan before running anything.

## Honest scope (what this is and isn't)

- It is a **design + operating protocol + seeded memory**, run by an LLM. The "intelligence" is the
  LLM's, *organized* by this architecture into something that persists, learns, and self-directs.
- The earlier scaffold's content pipeline (`../scaffold/`) becomes **one procedural skill** the mind
  can run inside its ACT step — the mind is the general layer above it.
- The hard safety gate stays hard: any publishable output still routes through the fail-closed
  compliance step (`../GATE_STEP2.md` / `../scaffold/orch/gate.py`); the mind may not route around it.
- Every gap is closed *as far as a prompt-run, file-backed mind honestly can* — see `GAP_MAP.md` for
  the candid "closed / partial" status of each.
