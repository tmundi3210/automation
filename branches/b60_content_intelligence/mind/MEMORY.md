# MEMORY — the persistent store that makes it not forget

Memory is the substrate every faculty stands on. It is **four tiers** of plain files the LLM reads at
the start of a turn and writes at the end, plus a tiny **INDEX** read first so the mind always knows
*what it knows and where*. Nothing here runs code — the LLM operates these files directly.

## The four tiers

| Tier | What lives here | On disk |
|---|---|---|
| **Working** | this run's scratch + the 3–7 things in attention right now (ephemeral) | `memory/kernel/blackboard/`, `goals/plan.md`, attention focus |
| **Episodic** | the dated **diary** — one entry per run — and the raw machine-readable episodes | `memory/episodic/journal/<date>.md`, `memory/experience/*.jsonl` |
| **Semantic** | consolidated, **calibrated beliefs** about the world & audiences (estimates, not facts) | `memory/semantic/world/*.md`, `memory/semantic/audiences/*.md` |
| **Procedural** | **skills/playbooks** — "how to do X that worked," with a success rate | `memory/procedural/playbooks/*.md`, `memory/procedural/lessons.md` |

*Episodic memory has two views of the same events: the **diary** (`episodic/journal/`, human-narrative,
one entry per run) and the **structured episodes** (`experience/*.jsonl`, machine-readable falsifiable
claims + outcomes the calibration & learning faculties consume). A diary entry references its episode id.*

## The canonical tree

```
memory/
  INDEX.md                       # READ FIRST every turn — the map of what the mind knows
  kernel/                        # runtime (orch)
    cursor.json                  #   resume pointer {env, turn, active_step, phase, plan_hash}
    turn_log.jsonl               #   one line per turn (audit trail)
    checkpoint.jsonl             #   append-only step checkpoints (crash-safe resume source of truth)
    schedule.jsonl               #   ALL future wake-ups: recheck | consolidate | recalibrate
    environments.jsonl           #   environment registry (one row per domain/audience/platform)
    blackboard/<env>/<turn>.md   #   per-turn typed scratch (one write-owner per block)
  goals/                         # agency (planner_heavy)
    goals.md  direction.md  plan.md  subgoals.md
  attention/                     # attention (erotetic_heavy)
    questions.md  surprises.md  explore_log.md  inquiry_budget.md
  calibration/                   # calibration (epistemics_heavy)
    predictions.jsonl  recalibration_map.json  calibration_report.md
  experience/                    # learning (eval_heavy) — raw experience
    episodes.jsonl  outcomes.jsonl  reflections/<ep>.md  _ledger.jsonl
  schemas/                       # generalization (reason)
    <schema_id>.md  abstractions.md  transfer_log.md
  episodic/                      # the DIARY (memory)
    journal/<date>.md  timeline.md  _archive/<period>.md
  semantic/                      # beliefs (memory)
    world/<topic>.md  audiences/<segment>.md  glossary.md
  procedural/                    # skills (memory)
    playbooks/<skill>.md  lessons.md
  _meta/
    retention.md  consolidation_log.md
```

**Single write-owner:** each faculty writes only its own folder. `INDEX.md` is the one file the
running turn rewrites in place. `semantic/` and `procedural/` are written **only by the consolidation
pass** — during a run, belief/lesson changes are *proposed* in the diary, then applied in "sleep."

## Format — `INDEX.md` (read first; tiny & decision-lossless)

```
# MIND INDEX · updated:<ts> · run:<id> · env:<env_id> · schema:mem/1.0
## NOW       goal · phase · step k/n · last_run outcome · top open question
## TIERS     working / episodic / semantic / procedural counts + last-consolidated
## HOT POINTERS   id → file · 1-line why · confidence      (the few records worth loading)
## RETRIEVAL RULE  load INDEX + working/* always; + pointers whose id ∈ goal.touches; + ≤2 semantic
                   by topic-match; budget ~6k tok. Not in INDEX = doesn't exist (create, don't invent).
## CONSOLIDATION DUE   next pass date · backlog vs threshold · bloat status
```

## Format — a diary entry (`episodic/journal/<date>.md`, one per run)

Each entry is a self-contained "what happened and what changed in me," so the next run's self can
reconstruct it losslessly:

```
## RUN <id> · <start–end ts> · goal:<G-id> · phase:<phase>
SET-OUT  : the intent for this run
LOADED   : exactly which memory slice came into context (so retrieval is auditable)
DID      : the episode
FOUND    : the finding
BELIEF Δ : what semantic memory should change (the load-bearing line)  | none
PREDICT  : link to a calibration ledger id + p()                        | none
ASSUMED  : conditional-knowns (assumptions to revisit)                  | none
LESSON?  : a candidate procedural distillation                         | none
OUTCOME  : result + the next action / next wake-up
PROV     : sources, model, thresholds, ledger ids
```

The `BELIEF Δ`, `PREDICT`, `ASSUMED`, `LESSON?` lines are **mandatory** (write `none` if empty) — they
are how an episode becomes knowledge in the consolidation pass.

## Format — a semantic record (`semantic/world/<topic>.md`, a calibrated belief — never a bare fact)

```
# SEMANTIC · world/<topic> · conf:<0–1> · updated:<date> · runs-seen:<n>
BELIEFS            claim · conf · type(epistemic=reducible | aleatory=irreducible,do-not-chase) · provenance
CONDITIONAL-KNOWNS assumptions; promote to BELIEF only if the condition is verified
KNOWN-UNKNOWNS     → attention/questions.md
SUPERSEDED         retired beliefs kept for audit (not loaded)
LINKS              related semantic/procedural records
```

Confidence on every claim; aleatory noise flagged *do-not-chase* (you can't learn out of irreducible
noise — anti-bloat for beliefs); assumptions quarantined until verified; superseded beliefs archived,
not deleted (auditable belief revision); `runs-seen` so a belief earns confidence by repetition.

## Protocol — read / write / consolidate

**READ-AT-START** (top of every turn): read `INDEX.md` in full → read `kernel/cursor.json` +
working scratch (restore an interrupted run) → apply the **retrieval rule** (load only the pointers the
goal touches + ≤2 topic-matched semantic records, within budget) → record what you loaded.

**WRITE-AT-END** (run end, and at every checkpoint): append a **diary entry** (with the four mandatory
lines) → append a `timeline.md` line → update `INDEX.md` → **do not** edit `semantic/`/`procedural/`
here (propose changes in the diary; consolidation applies them).

**CONSOLIDATION "SLEEP"** (scheduled: daily light / weekly deep, or when unconsolidated runs ≥ 7):
fold each diary `BELIEF Δ` into the matching semantic record (calibration outcomes move confidence) →
promote repeated `LESSON?` candidates (seen ≥2×) into `procedural/` → **forget/compress**: roll
consolidated journal days into `_archive/`, drop scratch, collapse duplicate beliefs → **round-trip
check** (could the next run reconstruct every decision from the consolidated store alone? keep anything
must-preserve) → append `_meta/consolidation_log.md` and refresh `INDEX.md`.

## Why this closes the gaps (and its honest limits)

Persistent memory **is** the continuity that makes it a mind (read your past self from disk each turn);
it is the **raw material** for learning (episodes → consolidation → lessons), the **home** for adaptive
calibration (the ledger + confidence deltas) and generalization (schemas/abstractions). Limits, stated
plainly: (1) memory is only as good as the **discipline of writing it** — skip write-at-end and
continuity silently breaks (the kernel enforces it); (2) **retrieval is lossy by design** — a relevant
but un-pointered record is forgotten until the INDEX names it, so consolidation must keep the INDEX
honest; (3) memory makes beliefs **persistent and revisable**, it does not make them correct — the
confidence/provenance typing is what stops a wrong belief from calcifying.
