# FIX SPEC — Format-Aware BKT Evidence Weighting (planning only, no code)

_Root-cause fix for the simulation findings of 2026-07-09: the app's BKT mastery estimate overestimates weak learners because every answer format is priced identically (slip cap 0.10 / guess cap 0.30, no format input), so lucky MCQ answers inflate mastery — driving premature weakness-resolution (0.81–1.0 on weak profiles), prereq-gate misses (threshold-miss 55–82% of failures), and plausibly the E1–E4 slip over-labeling. Authored AS the `learner` specialist, grounded in `learner.specialist.json`, `kb_knowledge_tracing.spec.json` (BKT posterior, IRT guessing floor, Elo/Rasch equivalence), and `APP_RECON.md` §3a/§3d/§4a. Sync to planit `docs/plan/` for the build._

**Author:** learner_model_adaptivity_strategist · **Scope:** BKT mastery layer only (skill layer). FSRS memory layer untouched.

## 0. What the app does today
- [FACT — APP_RECON.md §3a] `submitReview/entry.ts:106-111` runs the BKT posterior per concept with **slip capped at 0.10, guess capped at 0.30**, with **no question-format input** — the same P(G) prices a 4-option MCQ and a typed free-text answer. Elo θ updates with K=0.3 (`:112-126`); `elo_theta_sd` decays ×0.97 toward 0.3 **regardless of outcome** (`:122`).
- [FACT — APP_RECON.md §3d] BKT params (pT/pS/pG) are never re-fit. [FACT — §4a] Diagnosis rules key on mastery: prereq gate fires only when prereq `bkt_p_mastery < 0.5`; slip label requires mastery ≥ 0.6; retrieval_decay requires mastery ≥ 0.5.
- [UNKNOWN] The actual P(T) value and whether guess/slip sit at their caps — recon documents caps only. Implementation must read `:106-111` and report values before coding.
- [METHOD] Mechanism of the measured inflation: with S=0.10, G≤0.30, one correct MCQ multiplies mastery odds by (1−S)/G ≥ 3; two lucky guesses take P(L)=0.25 past 0.6 — exactly the lucky-guesser premature-resolution = 1.0 and the prereq "looks fine" threshold-miss.

## 1. Format-aware guess parameter P(G)
- Per-format table (static constants per question type):
  - 4-option MCQ: **P(G)=0.25** [FACT — IRT 3PL chance floor c≈0.25 for 4 options]; never below 0.25 for this format even if a future fit says lower; keep the ≤0.30 degeneracy cap [FACT — kb caps].
  - Typed / free-recall (incl. LLM-graded free_text, spoken): **P(G)=0.05 [ESTIMATE]** — doctrine fixes the direction (free recall ≫ MCQ evidence); 0.05 (not 0) allows for lenient LLM grading and cued recall.
- Posterior must consume the **item's** P(G): P(L|correct)=P(L)(1−S)/[P(L)(1−S)+(1−P(L))·G_item]; P(L|wrong)=P(L)·S/[P(L)·S+(1−P(L))(1−G_item)] [FACT — kb equations]. Unit-test direction (correct raises, wrong lowers) [FACT — kb anti-rework rule].
- Per-item G alone is insufficient (exact Bayes with G=0.25 still moves 0.2→0.47 on one correct [METHOD — arithmetic]). Doctrine additionally licenses **weighting the update magnitude by format**: apply a tempered update in odds space, LR_effective = LR^w_format, with **w_mcq ≈ 0.3, w_free_recall = 1.0 [ESTIMATE]**. Same exponent both directions (§2).
- **Qualitative check (binding):** learner at P(L)=0.2, one correct 4-option MCQ → P(L) ≈ 0.27 (Δ ≤ +0.07); one correct typed answer → ≈ 0.8. MCQ corrects barely move a low-mastery estimate; typed corrects move it a lot.
- [UNKNOWN → verify] P(T): report the coded value; require P(T) ≤ 0.15 [ESTIMATE] and run the §6 guesser test against it — an oversized P(T) can re-inflate a guesser even with correct emission math.

## 2. Slip parameter sanity (no overcorrection)
- Keep S within the doctrine cap band **0.05 ≤ P(S) ≤ 0.10–0.30** [FACT — kb degeneracy caps; ESTIMATE for the 0.05 floor]. A slip floor stops single wrong answers from cratering mastery.
- Temper wrong answers by the **same** w_format as corrects [ESTIMATE]: MCQ wrong at w=0.3 moves 0.2→≈0.12 — informative, not a hammer. Do NOT down-weight only corrects; asymmetric tempering would turn the guesser bias into a pessimist bias.
- For LLM-graded free text, allow S at the top of band (≈0.10–0.15 [ESTIMATE]) to price grader noise; never above 0.30.

## 3. Evidence weighting beyond format (easy-item pass = weak evidence)
- Elo/Rasch is already surprise-proportional — K(y−p) shrinks as expected correctness p→1 [FACT — ELO_DUAL_RATING]; Elo needs nothing new.
- For BKT [ESTIMATE — design synthesis]: for **corrects only**, scale the tempering exponent by `clamp((1−p̂)/0.15, 0.25, 1.0)` where p̂ is Elo-predicted correctness — items at/harder than the ~85% band count fully; far-easier passes shrink. Never down-weight a **miss** on an easy item (high-surprise evidence).
- Also apply w_format to the Elo θ step [FACT — doctrine]; low risk since Elo consumers are new.

## 4. Downstream expectations from THIS fix alone
- **Premature weakness-resolution:** lucky-guesser 1.0 → ~0; struggling 0.81–0.87 → near 0 (the mastery ≥ 0.6 leg stops being crossed by guessing). Additionally require resolution to demand low RD-style uncertainty AND ≥1 free-recall correct since the flag [ESTIMATE — uncertainty-gate doctrine].
- **Prereq-gate recall (0.17–0.42):** substantial improvement — threshold-miss dominated (55–82%); deflated mastery drops weak prereqs below the gate line. Residual misses from absent ConceptEdge prereq edges are a different layer [boundary].
- **E1–E4 cause accuracy (0.34–0.48): re-measure after re-baseline, do NOT redesign the router first.** The slip label structurally requires mastery ≥ 0.6 [FACT — §4a], so inflated mastery plausibly explains slip over-labeling. Redesign only if accuracy is still < 0.6 [ESTIMATE bar] after re-baseline.

## 5. What must NOT change
- FSRS review scheduling: CardState/ts-fsrs, desired_retention, w_optimized path — untouched (BKT=skill layer, FSRS=memory layer).
- Dial separation: acquisition ~80–85% band vs desired_retention — neither moved.
- Concept graph, grading rubrics/format authoring (other specialists' layers); no DKT; no hard-coded Glicko ([UNKNOWN] equations); append-only revlog + idempotent replay; Weakness UI semantics.
- Out of scope, flag separately: `elo_theta_sd` ×0.97 unconditional decay on ACTIVE reviews (partially addressed in the Slice 2 refinements; do not bundle further changes here).

## 6. Acceptance (simulation harness)
1. **Unit direction+magnitude:** P(L)=0.2 → one correct MCQ Δ ≤ +0.07; one correct typed Δ ≥ +0.4; one wrong MCQ Δ ≈ −0.08 (no hammer) [ESTIMATE targets].
2. **Lucky-guesser profile (pure chance, MCQ-only):** P(L) stays < 0.4 over ≥20 items; premature-resolution = 0 [ESTIMATE ≤0.05 tolerance].
3. **Struggling profile:** premature-resolution ≤ 0.05 (from 0.81–0.87).
4. **No overcorrection:** genuine-mastery profile (≈90% correct, mixed formats) reaches P(L) ≥ 0.8 within ~15 items and is never flagged weak [ESTIMATE].
5. **Prereq-gate recall ≥ 0.7** (from 0.17–0.42) with **false-gate ≤ 0.1**; threshold-miss share of failures drops below 25% [ESTIMATE].
6. **Cause accuracy: re-measure only.** Record post-fix E1–E4 accuracy and slip-label rate; open a router-redesign directive only if accuracy < 0.6 [ESTIMATE].
7. **FSRS regression guard:** identical rating streams produce byte-identical CardState schedules pre/post fix [METHOD — differential replay].

**Escalate before build if:** coded P(T) > 0.15, guess/slip found outside caps, or the resolution rule proves to live outside the pipeline where this spec assumes [UNKNOWN until `:106-111` and the resolution code path are read].
