# SELF_LOOP.md — The Self-Improvement Loop (T12)

**What this is.** A re-runnable **reflection loop** that makes any one of the nine gated specialists interrogate **its own analysis** and feed the result back as prioritized, honesty-tagged improvements. The loop's **base unit is a fresh agent = a fresh context window**: every role in a round is a *new* agent that reads only its named inputs and writes only its named output, so no role sees another's private reasoning. That fresh-context separation is what turns "self-questioning" into a real audit instead of a model agreeing with itself. The machine-readable companion is `self_improvement_loop.json` in this same directory.

The loop is deliberately *modest about what it may do*: it can find gaps, contradictions, stale figures, over-claims and missing tags, and it can apply **only** high-confidence, in-scope, mechanical corrections that still pass `build.sh`. It may **never** invent content to fill a gap, fabricate a firm figure, turn the off-paper mechanism into an accusation, relax the three governing `[UNKNOWN]` blockers, or reintroduce the discredited ~7% cotton duty. Anything that would require inventing a fact becomes a **named gap + a `propose` backlog item**, not an edit.

---

## The round (one chat cycle, four fresh agents)

```
  R1 questioner ─► R2 answerer ─► R3 judge ─► (dry? stop : next round) ─► R4 synthesizer
   (fresh)          (fresh, blind)  (fresh)                                (fresh)
   writes           writes          writes                                 writes
   questions.json   answers.json    judgment.json                          LOOP_REPORT.md + feedback.json
```

- **R1 — Questioner (adversary).** Reads the target `*.specialist.json` + this protocol + the glossary (may open the specialist's KBs). Writes the *hardest* questions the specialist claims — via its `competency_questions`, `owns[]`, decision procedure and dominance rules — to be able to answer but is most likely to fumble. Targets the load-bearing joints: viability posture, the duty trio, the pilling floor, the knit+LINK `[UNKNOWN]`, CAC, AI autonomy, the off-paper mechanism, naming legality. It does **not** answer.
- **R2 — Answerer (the specialist itself).** Reads the specialist + the questions only. **You ARE this specialist**: answer strictly from your own spec/KBs, carrying `[FACT]/[ESTIMATE]/[UNKNOWN]`. Where the spec doesn't support an answer, say `specialist_supports = no/partial` and describe the gap — never invent. This is the self-LLM questioning turn.
- **R3 — Judge (neutral referee).** Reads specialist + questions + answers. Classifies each answer: `gap | contradiction | stale | overclaim | untagged | scope | ok`, cites the exact spec location, proposes a minimal fix, and assigns a disposition `apply_now | propose | watch`. Sets `dry=true` only if the round produced **no** high/med finding.
- **R4 — Synthesizer.** Once the loop stops, merges the rounds: dedupes, ranks by severity, splits into `apply_now / propose / watch`, and writes the human-facing `LOOP_REPORT.md` + machine `feedback.json`.

**Stop conditions.** Dry-stop when the judge returns `dry=true`; otherwise up to **2 rounds** (a cost cap, not a target — round 2's questioner also reads round 1's judgment and hunts the survivors). A specialist still throwing high-severity findings at the cap is flagged **NOT-CONVERGED**, never silently truncated.

**Disposition → what happens.**
| Disposition | Meaning | Action |
|---|---|---|
| `apply_now` | high-confidence, in-scope, mechanical | finalizer edits by hand → `build.sh` must stay **ALL GREEN** |
| `propose` | real improvement needing judgment / new measurement | → `PROPOSED_BACKLOG.md`, not applied |
| `watch` | low-severity / already mitigated | recorded so it isn't rediscovered every loop |

---

## Files

- Protocol: `specialists/BRAIN/SELF_LOOP.md` (this) + `specialists/BRAIN/self_improvement_loop.json`.
- Per specialist: `analysis/self_loop/<slug>/` → `roundN_questions.json`, `roundN_answers.json`, `roundN_judgment.json`, `LOOP_REPORT.md`, `feedback.json`.
- Rollups: `analysis/self_loop/SELF_LOOP_SUMMARY.md` (all nine), `APPLIED.md` (what changed + re-gate), `PROPOSED_BACKLOG.md`.

Slugs → specialist files: `ludhiana/ludhiana` · `yarn/yarn` · `machines/machines` · `mfg_economics/mfg` · `quality_checker/quality` · `finishing/finishing` · `brand_market/brand_market` · `unit_economics/unit_economics` · `design/design`.

---

## Verification (trust-but-verify)

After the finalizer applies `apply_now` items, the orchestrator **independently** re-runs `bash FACTORY/build.sh FACTORY/sweater_vertical/specialists/<slug>`, confirms **ALL GREEN**, greps for residue (any ~7% duty, food-template leftovers, banned specialist tokens), and only then commits. Agent self-reports are never trusted without this re-gate. The loop is an honesty amplifier — if it ever tempts an edit that needs a fabricated fact, the edit is refused and logged as a gap instead.

---

## Copy-paste PROMPT (run one manual round on a specialist in a fresh window)

```
You are running one round of the Sweater Vertical SELF-IMPROVEMENT LOOP on ONE specialist.
Protocol: FACTORY/sweater_vertical/specialists/BRAIN/self_improvement_loop.json
Honesty gate: BRAIN/router.json neutralize_gate N1–N7 (you may not fabricate to fill a gap).

Target specialist: <SLUG>  (file: FACTORY/sweater_vertical/specialists/<SLUG>.specialist.json)

Do it in three fresh passes, writing each to analysis/self_loop/<slug>/ :
1) QUESTIONER — read the specialist + glossary; write the 8–12 hardest questions it should be able to
   answer but is most likely to fumble (aim at load-bearing joints: viability/GATED posture, duty trio,
   pilling floor, knit+LINK [UNKNOWN], CAC, AI autonomy, off-paper mechanism, naming legality). → round1_questions.json
2) ANSWERER — you ARE the specialist; answer ONLY from its spec/KBs with [FACT]/[ESTIMATE]/[UNKNOWN];
   where unsupported, say so and describe the gap (never invent). → round1_answers.json
3) JUDGE — classify each answer gap|contradiction|stale|overclaim|untagged|scope|ok, cite the spec location,
   propose a minimal fix, assign apply_now|propose|watch, set dry. → round1_judgment.json
Then SYNTHESIZE → LOOP_REPORT.md + feedback.json. Apply ONLY apply_now items, then re-run
`bash FACTORY/build.sh FACTORY/sweater_vertical/specialists/<slug>` and confirm ALL GREEN.
```
