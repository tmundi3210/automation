# FACTORY/SELF_LOOP.md — run the self-improvement loop on ANY vertical

The **canonical protocol is NOT here** — it lives, gated and already run 9x, at:

- `FACTORY/sweater_vertical/specialists/BRAIN/SELF_LOOP.md` (human-facing protocol)
- `FACTORY/sweater_vertical/specialists/BRAIN/self_improvement_loop.json` (machine form)

This file only **parameterizes** it so any vertical built with `FACTORY/build.sh`
(or the root `validators/` directly) can run the same loop without copying or
moving the sweater files. Read the canonical protocol first; then bind the slots
below for your vertical.

## What the loop is (one paragraph — full definition in the canonical files)

Per specialist, per round, four **fresh context windows**: R1 questioner (adversary,
does not answer) → R2 answerer (you ARE the specialist; answers only from its
spec/KBs with `[FACT]/[ESTIMATE]/[UNKNOWN]`; gaps stay gaps) → R3 judge (classifies
`gap|contradiction|stale|overclaim|untagged|scope|ok`, cites exact spec location,
sets disposition `apply_now|propose|watch`, sets `dry`) → dry-stop or one more round
(max 2; still-hot at cap = flag **NOT-CONVERGED**, never silently truncate) → R4
synthesizer (LOOP_REPORT.md + feedback.json). Finalizer applies **only** `apply_now`
items; the orchestrator then **independently re-runs the gate** before commit.

## Slot binding — what YOUR vertical must supply

| Slot | Sweater binding (reference) | Your vertical supplies |
|---|---|---|
| target specialists | `FACTORY/sweater_vertical/specialists/<slug>.specialist.json` | your gated `*.specialist.json` files |
| slug map | "Slugs" line in the sweater SELF_LOOP.md | one slug per specialist; slug = its working-dir name under `analysis/self_loop/` (default: the specialist file basename) |
| scope/glossary doc | `SCOPE_GLOSSARY.md` | your vertical's scope doc, if any (omit → R1/R2 read only spec + grounded KBs) |
| load-bearing joints | sweater list (viability posture, duty trio, pilling floor, knit+LINK `[UNKNOWN]`, CAC, AI autonomy, off-paper mechanism, naming legality) | **rewrite for your domain**: the 6–10 highest `risk_if_wrong` claims your specialists carry — R1 aims there |
| re-gate command | `bash FACTORY/build.sh FACTORY/sweater_vertical/specialists/<slug>` | `bash FACTORY/build.sh <your-folder>`; for root-repo artifacts: `python3 validators/kb_validator.py KB --mode dense` + `python3 validators/specialist_validator.py SPEC` |
| output dir | `FACTORY/sweater_vertical/analysis/self_loop/<slug>/` | `<your_vertical>/analysis/self_loop/<slug>/` (`roundN_questions/answers/judgment.json`, `LOOP_REPORT.md`, `feedback.json`) |
| rollups | `analysis/self_loop/SELF_LOOP_SUMMARY.md`, `APPLIED.md`, `PROPOSED_BACKLOG.md` | same three files under your vertical's `analysis/self_loop/` |
| residue greps | ~7% duty, food-template leftovers, banned tokens | your vertical's known discredited figures / template residue |

## Invariants — NOT re-parameterizable (the loop's stability conditions)

1. **Fresh context per role.** Every role is a NEW window reading only its named
   inputs — this is what makes the audit ≠ the model agreeing with itself.
2. **Honesty discipline is factory-invariant**, not sweater-specific:
   `[FACT]/[ESTIMATE]/[UNKNOWN]` on load-bearing claims; off-paper economics =
   mechanism, never accusation; no invented rates; a fix that needs a fabricated
   fact is downgraded to a named gap + `propose`, never applied.
3. **`apply_now` = high-confidence + in-scope + mechanical**, and the artifact must
   stay ALL GREEN under the re-gate. Everything else → `propose` backlog.
4. **Orchestrator independently re-runs the gate** after the finalizer; agent
   self-reports are never acceptance (same custody rule as `PLAN.md` §3b).
5. **Bounded rounds with an honest exit**: dry-stop or max 2 rounds; a specialist
   still producing high-severity findings at the cap is flagged NOT-CONVERGED —
   the cap is a cost bound, never a convergence claim.

## Feed-forward into the factory

Loop findings about GENERATION defects (not content) — e.g. a whole vertical's
specialists sharing the same structural weakness — belong in the gate-failure
feedback lane: record → aggregate → threshold → human-gated refactor proposal
(`PLAN.md` §3f). The self-loop improves one artifact; §3f improves the machine
that makes them.
