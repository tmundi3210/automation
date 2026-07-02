# learner_model_and_adaptivity.md — L1 research: modeling the learner + adaptive challenge

_Grounds the learner-analysis / adaptive-learning specialist (owner brief §2a items 5, 17; Q8, Q9). Honesty tags per claim: [FACT] / [ESTIMATE] / [UNKNOWN]._

**Sourcing caveat for this file** [FACT — observed in this session]: direct page fetches (WebFetch) were blocked by this container's egress policy for most hosts (wikipedia.org, arxiv.org, nature.com, glicko.net all returned proxy 403). All [FACT] claims below were verified through WebSearch retrieval, which returned quoted content from the named source URLs. Where only my prior knowledge supports a formula (not the retrieved text), it is tagged [ESTIMATE] with confidence, never [FACT].

---

## 1. Knowledge tracing — estimating "does the learner know skill X?"

Four families, in ascending data hunger. All four are per-SKILL (or per-concept-tag) models layered on top of the per-CARD memory model (FSRS — see §4 and `spaced_repetition_and_anki.md`).

### 1.1 Bayesian Knowledge Tracing (BKT)

- [FACT] Introduced by Corbett & Anderson (1995). Models student knowledge of one skill as a latent **binary** state (known / not known) in a Hidden Markov Model with **four parameters**: `P(L0)` initial mastery, `P(T)` learn/transition probability, `P(G)` guess, `P(S)` slip. Sources: van de Sande, "Properties of the Bayesian Knowledge Tracing Model," JEDM — https://files.eric.ed.gov/fulltext/EJ1115329.pdf ; Baker et al. — https://learninganalytics.upenn.edu/ryanbaker/behaviormetrika_vfinal.pdf
- [FACT — update equations as retrieved from the sources above] Posterior after observing a **correct** answer:
  `P(L|correct) = P(L)·(1−P(S)) / [ P(L)·(1−P(S)) + (1−P(L))·P(G) ]`
  Posterior after an **incorrect** answer:
  `P(L|wrong) = P(L)·P(S) / [ P(L)·P(S) + (1−P(L))·(1−P(G)) ]`
- [FACT — by definition, given the HMM structure the sources describe] Predicted probability of a correct response is exactly the denominator of the first update: `P(correct) = P(L)·(1−P(S)) + (1−P(L))·P(G)`.
- [ESTIMATE, high confidence — canonical BKT step, consistent with the cited HMM description but not verbatim in the text I retrieved] After the evidence update, apply the learning transition: `P(L_next) = P(L|obs) + (1−P(L|obs))·P(T)`. Classic BKT assumes **no forgetting** (P(known→unknown)=0), which is why a separate memory model (FSRS) is still required for scheduling.
- [FACT] Parameters are fit by EM / Baum-Welch (forward-backward) on response sequences — https://files.eric.ed.gov/fulltext/EJ1115329.pdf
- [ESTIMATE, high confidence — standard practice in the BKT literature] Degenerate fits must be guarded: constrain P(G) < 0.3 and P(S) < 0.1–0.3, else the model can "explain" performance as guessing/slipping instead of knowledge.
- Design fit for this system [ESTIMATE — synthesis]: BKT is the right *starter* skill model — 4 parameters per concept-tag, works from the first handful of answers, fully interpretable ("P(knows periodontal-anatomy) = 0.72"), trivially computable in a nightly batch job.

### 1.2 Deep Knowledge Tracing (DKT)

- [FACT] Piech et al. 2015: an LSTM/RNN takes the sequence of (question-id, correct?) interaction tuples and outputs, at each step, a vector of predicted mastery probabilities over all concepts. On the Khan Academy dataset it reached **AUC 0.85 vs 0.68 for standard BKT** (marginal baseline 0.63); reported gains of ~25–30% relative AUC across canonical datasets. Sources: https://arxiv.org/pdf/1506.05908 ; https://stanford.edu/~cpiech/bio/papers/deepKnowledgeTracing.pdf ; overview https://www.emergentmind.com/topics/deep-knowledge-tracing-dkt
- [ESTIMATE, high confidence — widely replicated critique in the follow-up literature, e.g. Khajah et al. 2016, "How Deep is Knowledge Tracing?" https://www.educationaldatamining.org/EDM2016/proceedings/paper_144.pdf] Much of DKT's advantage disappears when BKT is extended with forgetting/abilities; DKT is also uninterpretable and needs thousands of learners' data.
- Design fit [ESTIMATE — synthesis, high confidence]: **not applicable** to a single-user system — DKT trains across large student populations. Note it as the population-scale option; do not build it.

### 1.3 Elo / Glicko-style ratings for learner AND items

- [FACT] Pelánek 2016, "Applications of the Elo rating system in adaptive educational systems," Computers & Education 98:169–179: each answer is treated as a "match" between student and item; both a student skill θ and an item difficulty d are rated and updated after every interaction with a simple formula; popular in large adaptive systems for simplicity and O(1) updates. Sources: https://www.sciencedirect.com/science/article/abs/pii/S036013151630080X ; https://dl.acm.org/doi/10.1016/j.compedu.2016.03.017
- [FACT — formula as retrieved from the Elo-for-education literature, e.g. https://arxiv.org/pdf/2411.07028 and https://pmc.ncbi.nlm.nih.gov/articles/PMC12784335/]
  Expected correctness: `p = 1 / (1 + e^−(θ − d))`
  Update after observing y ∈ {0,1}: `θ ← θ + K·(y − p)` and symmetrically `d ← d − K·(y − p)`.
  K regulates step size; K = 0.4 has been considered appropriate in some educational settings.
- [ESTIMATE, medium-high confidence — Pelánek 2016 describes an "uncertainty function" variant; abstract-level verification only] Common refinement: make K decay with the number of attempts, e.g. `K = a / (1 + b·n)`, so early answers move ratings more.
- [FACT] Glicko (Glickman, 1995) extends Elo with a **rating deviation (RD)** — an explicit uncertainty around the rating, so estimates are distributions rather than points; low-RD (well-measured) players move less per result; RD grows during inactivity and shrinks with play. Sources: https://elote.mcginniscommawill.com/rating_systems/glicko.html ; https://en.wikipedia.org/wiki/Glicko_rating_system (listing retrieved via search; page fetch blocked)
- [UNKNOWN] Exact Glicko update equations — the canonical spec at glicko.net was egress-blocked; do not hard-code them without fetching http://www.glicko.net/glicko/glicko.pdf later.
- Design fit [ESTIMATE — synthesis]: Elo is the cheapest dual model — it rates the LEARNER per concept *and* every ITEM (card/question) at once, so the same machinery that finds weak concepts also finds miscalibrated cards ("this card's difficulty rating drifted far above its siblings" → card quality flag, §6). Glicko's RD idea maps to "how sure are we this weakness is real vs under-sampled" — use an RD-like uncertainty per concept to decide when to probe (§2).

### 1.4 Item Response Theory (IRT) basics

- [FACT] 3PL model: `P(correct|θ) = c + (1−c) / (1 + e^{−a(θ − b)})` with θ = ability, b = difficulty (the θ where the c-adjusted curve passes its midpoint; for c=0, P=0.5 at θ=b), a = discrimination (slope), c = pseudo-guessing lower asymptote (for a 4-option MCQ random guessing gives c≈0.25, though estimated c often differs). Sources: https://www.cogn-iq.org/learn/theory/three-parameter-logistic-model/ ; https://metricgate.com/docs/three-parameter-logistic-irt/ ; https://bookdown.org/jorgetendeiro/ParametricIRT/the-3pl-model-3plm.html
- [FACT — by definition, nested models] 2PL sets c = 0; 1PL/Rasch additionally fixes a common a for all items, leaving only per-item difficulty b.
- [FACT — by inspection of the formulas] The educational Elo expectation `1/(1+e^−(θ−d))` is exactly the Rasch/1PL curve; Elo is best understood as an online, streaming approximation of Rasch estimation (also the framing in https://arxiv.org/pdf/1803.05926 "Learning meets Assessment: On the relation between IRT and BKT", per its listing).
- Design fit [ESTIMATE — synthesis]: full IRT calibration needs many respondents per item — not available for one user. Use IRT as the *conceptual* frame (ability vs item difficulty vs guessing floor for MCQ) and Elo as the practical estimator. The guessing floor c matters for the dental/TOEFL MCQ formats: a "correct" on a 4-option MCQ is ~25% expected by chance, so mastery evidence from MCQ corrects is weaker than from typed/spoken free recall — weight updates accordingly.

### 1.5 Which model where [ESTIMATE — design synthesis]

| Layer | Model | Granularity | Update cadence |
|---|---|---|---|
| Memory (when to review) | FSRS DSR (already researched) | per card | every review |
| Skill (does user know concept) | BKT per concept-tag | per concept | every answer + nightly re-fit |
| Challenge matching | Elo (θ per concept, d per item) + RD-style uncertainty | per concept × per item | every answer |
| Format correction | IRT guessing floor c by question format | per format | static constants |

---

## 2. Weakness modeling — not just WHERE, but WHY (owner: "why that weakness is there")

### 2.1 Error taxonomy — the four rival explanations for a wrong answer

[FACT where cited; taxonomy assembly itself is [ESTIMATE — synthesis, high confidence]]

| # | Cause | Signature | Cited basis |
|---|---|---|---|
| E1 | **Slip** (knows it, fumbled) | isolated error amid correct history on same concept | [FACT] KT models explicitly produce slip-vs-non-mastery probabilities; BKT's P(S) is exactly this — https://learninganalytics.upenn.edu/ryanbaker/behaviormetrika_vfinal.pdf ; slip-conditioned tutoring responses discussed in https://arxiv.org/html/2605.16207 |
| E2 | **Misconception** (stable wrong model) | *systematic*, reproducible wrong answers; picks the same wrong-idea distractor repeatedly | [FACT] Repair Theory / "buggy" procedures (VanLehn): errors as systematic application of buggy rules; VanLehn also argues many misconception-consistent actions are "patches" for impasses — the impasse endures, the patch may vary — http://act-r.psy.cmu.edu/wordpress/wp-content/uploads/2012/12/173Chapter_37_Intelligent_Tutoring_Systems.pdf ; misconception-tagged distractors + predicting which distractor a student selects: https://arxiv.org/html/2602.02414v1 |
| E3 | **Missing prerequisite** (never had the base) | fails item AND fails its prerequisite items; errors cluster along prerequisite edges | [FACT] Knowledge Space Theory (Doignon & Falmagne 1985): surmise relation q ≤ q′ = "mastering q′ implies having mastered q"; states form a prerequisite-respecting family — https://en.wikipedia.org/wiki/Knowledge_space (listing) ; https://www.aleks.com/about_aleks/knowledge_space_theory |
| E4 | **Retrieval failure** (learned, decayed) | previously answered correctly; time since success is long; FSRS predicted-recall R is low | [FACT — by definition of the FSRS DSR model, see repo file `spaced_repetition_and_anki.md`: R declines with elapsed time given stability S] |

### 2.2 Differential diagnosis — targeted probes to TEST which cause it is [ESTIMATE — probe design synthesis, medium-high confidence; the *mechanisms* it leans on are cited]

Decision procedure per flagged weakness (concept with low θ / low P(L)):

1. **Rule out slip (E1) first — retest, varied surface.** Re-ask the same concept within 1–2 days via a *differently worded* item. Pass → was a slip; raise P(L) back, no intervention. (Basis: BKT already prices this via P(S); a single error moves the posterior less than two.)
2. **Test retrieval failure (E4) vs never-learned.** Check history: if the card has prior successes and FSRS R at failure time was low (overdue), classify as decay → schedule fix (shorten interval / relearning steps), not content fix. If R was HIGH at failure time (recent success, early lapse), suspect E2/E3 or a bad card. [FACT — FSRS supplies the predicted R needed for this test; see repo FSRS file] [ESTIMATE — the threshold logic itself].
3. **Prerequisite back-off (E3 test).** Walk DOWN the concept graph: ask 1–2 items from each *direct prerequisite* of the failed concept. KST's fringe logic applies in reverse — the **inner fringe** (items whose removal leaves a valid state) and **outer fringe** (items addable next) are where assessment has maximal diagnostic value [FACT — ALEKS/KST: fringe items are "one step ahead of the current state and have maximal diagnostic value" — https://www.aleks.com/about_aleks/knowledge_space_theory ; https://jmatayoshi.github.io/publications/JMP2021_KST_ALEKS_preprint.pdf]. Prereqs pass → not E3. Prereqs fail → recurse: the REAL weakness is lower; treat the surface concept as blocked, remediate the deepest failing prerequisite first, then rebuild upward (matches the owner's terminology → basics → 2-relations → multi-step ladder).
4. **Misconception probe (E2 test).** Ask MCQ items whose distractors are *tagged with specific wrong ideas* (the Eedi-style design: each distractor encodes one misconception) [FACT — distractor-misconception tagging as diagnostic signal: https://arxiv.org/html/2602.02414v1 ; https://www.cs.cornell.edu/~molly/chi2018.pdf]. If the learner repeatedly selects the SAME misconception-tagged distractor across ≥2–3 items, classify E2 and remediate with a contrast card (right model vs wrong model side by side). Random distractor choice → not E2, likely E3 or thin encoding.
5. **Confusion-matrix check between sibling concepts.** Maintain a concept×concept matrix M where M[i][j] counts "asked about i, answered with j's content" (from tagged distractors, or LLM-graded free/spoken answers naming the wrong entity). High off-diagonal pairs (e.g., two drugs, two nerves, two TOEFL word senses) = **discrimination failure** — a subtype of E2 remediated with direct-comparison items and interleaved practice of exactly that pair (§3 interleaving evidence: interleaving is what trains discrimination). [ESTIMATE — design synthesis; confusion-matrix bookkeeping is standard ML practice applied here, and misconception-distractor prediction (cited above) supplies the data.]

Output of diagnosis = a **weakness record**: `{concept, cause ∈ {E1..E4}, evidence, prescribed fix}` — this is the "why" the owner asked for, stored in the learner KB.

### 2.3 Cause → fix mapping [ESTIMATE — synthesis]

| Cause | Fix | Scheduler effect |
|---|---|---|
| E1 slip | none / note fatigue context | none |
| E2 misconception | contrast cards, distractor-targeted MCQs, worked example | inject new cards; keep old ones |
| E3 missing prerequisite | back off to prerequisite unit; suspend downstream cards until prereq P(L) recovers | re-order queue |
| E4 retrieval decay | normal lapse handling (FSRS relearning); if chronic → card is a leech, rewrite it | shorter intervals; leech flag |

---

## 3. Desirable difficulties — keeping it challenging (owner: "keeping it challenging")

- [FACT] "Desirable difficulties" — term coined by Robert A. Bjork (1994): conditions of practice that feel harder and slow apparent progress but enhance long-term retention and transfer. Canonical four: **spacing, interleaving, retrieval practice (testing effect), generation**. Sources: Bjork & Bjork 2011 — https://bjorklab.psych.ucla.edu/wp-content/uploads/sites/13/2016/04/EBjork_RBjork_2011.pdf ; https://www.unh.edu/teaching-learning-resource-hub/sites/default/files/media/2023-06/itow-introducing-desirable-difficulties-into-practice-and-instruction-bjork-and-bjork.pdf ; https://en.wikipedia.org/wiki/Desirable_difficulty (listing)
- Key operational consequence [ESTIMATE — direct implication, high confidence]: fluency during study is a *misleading* signal; the system must judge learning by delayed retrieval success, not by how easy a session felt. Never let the learner (or the scheduler) optimize for in-session smoothness.

### 3.1 The ~85% success-rate target — verified, with scope caveat

- [FACT] Wilson, Shenhav, Straccia & Cohen, "The Eighty Five Percent Rule for optimal learning," **Nature Communications 10:4646 (2019)**: for a broad class of gradient-descent-based learning algorithms on **binary classification** tasks, the learning rate is maximized when training accuracy is ~85% — optimal error rate exactly **15.87%** (= Φ(−1)); demonstrated for ANNs and biologically plausible neural networks. Sources: https://www.nature.com/articles/s41467-019-12552-4 (fetch blocked; content verified via search retrieval) ; https://experts.arizona.edu/en/publications/the-eighty-five-percent-rule-for-optimal-learning ; press summary https://www.sciencedaily.com/releases/2019/11/191105113457.htm
- [ESTIMATE, high confidence — scope reading of the above] This is a result about *learning algorithms*, not a direct human RCT; treat "~85% success on NEW-material practice" as a principled default target, not a law. Note it is strikingly consistent with FSRS's default desired retention 0.90 for *review* (different quantity: retention of learned cards vs accuracy on training items) — the system should keep the two dials separate: **acquisition difficulty target ≈ 80–85% correct; review retention target = FSRS desired_retention (owner-tunable 0.8–0.97)** [FACT for the FSRS dial — see repo FSRS file].
- Challenge-calibration mechanism [ESTIMATE — synthesis]: with Elo in place this is one line — select next items whose difficulty d satisfies `p(θ, d) ≈ 0.85` for acquisition drills; loosen toward 0.7 when the learner is coasting (rolling 2-week success > 92%) and toward 0.9 when frustrated (success < 70% or session abandonment).

### 3.2 Interleaving vs blocking — the evidence

- [FACT] Brunmair & Richter 2019 (Psychological Bulletin) meta-analysis of interleaved learning: overall **medium positive effect, Hedges' g = 0.42**; effects for mathematical tasks smaller and mixed (primary studies range strongly negative to positive). Source: https://www.psychologie.uni-wuerzburg.de/fileadmin/06020400/2019/Brunmair_Richter_in_press__2019_META-ANALYSIS_OF_INTERLEAVED_LEARNING.pdf ; discussion https://econtent.hogrefe.com/doi/10.1026/0049-8637/a000260
- [FACT] Rohrer, Dedrick, Hartwig & Cheung 2020 (J. Educational Psychology 112(1):40–52), RCT: 787 seventh-graders, 54 classes, 5 schools, 4 months of interleaved vs blocked math assignments; on an unannounced delayed test one month later, interleaved scored **61% vs 38%**, **d = 0.83** [95% CI 0.68–0.97]. Sources: https://gwern.net/doc/psychology/spaced-repetition/2019-rohrer.pdf ; https://files.eric.ed.gov/fulltext/ED595322.pdf
- [ESTIMATE, high confidence — the mechanism consensus in the cited literature] Interleaving works chiefly where items are *confusable* (discriminative-contrast hypothesis) — which is why §2.2's confusion matrix should FEED the interleaving policy: the pairs the learner confuses are exactly the pairs to interleave.
- Scheduler rules [ESTIMATE — synthesis]: (a) never drill one concept-tag in a long block once past first exposure; (b) daily queue mixes tags, with confused-pair items deliberately adjacent-but-not-identical; (c) blocking allowed only for the very first encounter with a topic (initial encoding), per the mixed math results above.

---

## 4. "Easy for the user" detection → lower repetition load (owner: "what's easy... doesn't require too much repetition")

This is exactly what FSRS's D (difficulty) and S (stability) already encode — connect, don't duplicate. All FSRS formula claims below are [FACT] per the repo's verified file `FACTORY/study_system/research/spaced_repetition_and_anki.md` (which cites the open-spaced-repetition FSRS wiki):

- Per-card **difficulty D ∈ [1,10]** initialized from the first rating (`D0(G) = w4 − e^{w5·(G−1)} + 1`) and nudged by every rating; per-card **stability S** grows multiplicatively on success, and the growth factor SInc *shrinks* as D rises and *grows* as predicted recall R falls.
- Pressing **Easy (G=4)** applies an extra stability multiplier (w16) > that of Good, and low D makes future SInc larger → easy cards' intervals stretch fastest → **the repetition-load reduction the owner wants is emergent from FSRS, not a feature to bolt on**.
- What the learner-analysis layer ADDS on top [ESTIMATE — design synthesis]:
  1. **Concept-level easiness**: nightly, aggregate D and S over all cards sharing a concept-tag → concepts with low mean D + high median S + Elo θ ≫ d are "mastered-easy"; new cards from those concepts can start with longer initial steps and the specialist stops generating fresh drill items there.
  2. **Rating-honesty check**: if the user marks Easy but response latency is long or later lapses are high, flag over-optimistic self-grading (revlog stores per-review duration `time`, capped 60s — [FACT, repo FSRS file]).
  3. **Load budgeting**: predicted daily review load is computable from the S distribution; the nightly job (§5) uses it to keep tomorrow within the owner's time budget by (in order) postponing mastered-easy cards (lowest predicted-R loss), not weak-concept cards.

---

## 5. The nightly analysis mode as a batch job [ESTIMATE — design synthesis; this whole section is design, grounded in the cited machinery above]

Owner: "every night it should go into analysis mode." Inputs: full revlog (one row per review ever — [FACT, repo FSRS file]), answer records incl. queued spoken-answer transcriptions + their AI grades, concept graph, yesterday's plan. Pipeline (idempotent, order matters):

1. **Ingest & reconcile**: pull the day's reviews and any late-arriving graded voice answers; attach concept-tags.
2. **Retention actuals vs predicted**: for every review, FSRS predicted R at that moment; bin by predicted R (e.g., deciles) and compare to observed recall rate → per-bin calibration table + Brier score / log-loss (log-loss is the exact objective the FSRS optimizer trains on — [FACT, repo FSRS file]). Store the daily calibration snapshot for trend lines.
3. **Re-fit memory parameters (gated)**: if ≥ ~1000 total reviews and ≥ ~2–4 weeks since last fit, rerun the FSRS optimizer on the revlog (py-fsrs / fsrs-rs expose optimize — [FACT, repo FSRS file]); otherwise skip — refitting on thin data adds noise.
4. **Update skill models**: replay the day's answers through BKT (per concept) and Elo (θ per concept, d per item); decay Glicko-style uncertainty upward for concepts untouched > N days.
5. **Weakness clustering & diagnosis**: recompute concept×concept confusion matrix; flag concepts with (low θ OR P(L) < threshold) AND low uncertainty; for each flag, run the §2.2 differential-diagnosis decision procedure as far as existing data allows and emit **probe items** for whatever remains ambiguous → probes go into tomorrow's queue (few per day, so diagnosis never crowds out learning).
6. **Card-quality pass**: leech detection (repeated lapses — Anki's native leech mechanism [FACT, repo FSRS file]); Elo item-difficulty outliers vs siblings; cards whose predicted-vs-actual gap is extreme → flag for rewrite by the flashcard-quality specialist rather than punishing the learner.
7. **Tomorrow's schedule**: due FSRS reviews + probe items + new-material quota, ordered by interleaving rules (§3.2), trimmed to the time budget by postponing mastered-easy items first (§4.3), with acquisition items selected at p ≈ 0.85 (§3.1).
8. **Challenge calibration**: adjust the target success band from the rolling 2-week actuals (§3.1 mechanism).
9. **Reporting artifacts**: update the learner KB (weakness records with WHY), the gamification layer's day-completion percentages, and the self-metrics of §6. Everything written as data, no in-place mutation of history.

Cadence note [ESTIMATE]: steps 1–2, 4–9 nightly; step 3 weekly-to-monthly; all steps must be safe to re-run (crash mid-job → rerun produces same result).

## 6. System-level feedback loops — metrics the system tracks about ITSELF [ESTIMATE — design synthesis]

Owner: "feedback loops, the system itself... what are the faults." The system is a predictor + scheduler + content factory; each part gets its own scorecard:

| Loop | Metric | Computed from | Acts on |
|---|---|---|---|
| Prediction calibration | per-decile predicted-R vs actual recall; Brier/log-loss trend | §5 step 2 | triggers FSRS re-fit; warns if model degrades after a fit (revert parameters) |
| Skill-model sanity | BKT parameter degeneracy checks (P(G), P(S) caps §1.1); Elo θ/d divergence | nightly fits | freeze bad fits, fall back to previous parameters |
| Card quality | leech rate, item-Elo outliers, per-card lapse rate, per-template error rate | §5 step 6 | rewrite queue for the flashcard specialist; template-level fixes if a whole card TYPE underperforms |
| Schedule adherence | planned vs completed items/minutes; time-of-day completion pattern; abandonment mid-session | plan vs revlog | shrink plans that are chronically 60%-done (a plan that is never finished is a bad plan, not a bad learner); feeds the calendar fill-bar UI honestly |
| Challenge calibration | rolling success rate vs target band (§3.1) | daily answers | difficulty dial |
| Diagnosis efficacy | after a weakness was diagnosed E2/E3/E4 and its fix applied: did θ / P(L) recover within K days? | weakness records | if a fix class chronically fails, the *diagnosis procedure* (§2.2 thresholds) gets adjusted — this is the loop that improves the loop |
| Pipeline health | nightly-job completion, voice-answer grading backlog age, ungraded-item count | job logs | alerts; degraded mode = schedule on FSRS alone |

Principle [ESTIMATE]: every metric has (a) a stored daily snapshot (trend, not point), (b) a threshold, (c) a named automatic action or a named human-visible flag — a metric with no consumer is deleted.

---

## Cross-file links
- Memory model + Anki/FSRS integration surfaces: `research/spaced_repetition_and_anki.md`
- Question formats the probes must use: `research/question_types_and_answering.md`
- Concept graph / prerequisite ladder the E3 back-off walks: `research/flashcards_and_book_decomposition.md`

## Sources
1. van de Sande, Properties of the BKT Model (JEDM) — https://files.eric.ed.gov/fulltext/EJ1115329.pdf
2. Baker et al., Degree of Error in BKT Estimates — https://learninganalytics.upenn.edu/ryanbaker/behaviormetrika_vfinal.pdf
3. Piech et al. 2015, Deep Knowledge Tracing — https://arxiv.org/pdf/1506.05908 and https://stanford.edu/~cpiech/bio/papers/deepKnowledgeTracing.pdf
4. Khajah, Lindsey, Mozer 2016, How Deep is Knowledge Tracing? — https://www.educationaldatamining.org/EDM2016/proceedings/paper_144.pdf
5. Pelánek 2016, Applications of the Elo rating system in adaptive educational systems — https://www.sciencedirect.com/science/article/abs/pii/S036013151630080X ; https://dl.acm.org/doi/10.1016/j.compedu.2016.03.017
6. Elo-for-education formula expositions — https://arxiv.org/pdf/2411.07028 ; https://pmc.ncbi.nlm.nih.gov/articles/PMC12784335/
7. Glicko overviews — https://elote.mcginniscommawill.com/rating_systems/glicko.html ; https://en.wikipedia.org/wiki/Glicko_rating_system
8. IRT / 3PL — https://www.cogn-iq.org/learn/theory/three-parameter-logistic-model/ ; https://metricgate.com/docs/three-parameter-logistic-irt/ ; https://bookdown.org/jorgetendeiro/ParametricIRT/the-3pl-model-3plm.html
9. IRT↔BKT relation — https://arxiv.org/pdf/1803.05926
10. VanLehn / Repair Theory / ITS chapter — http://act-r.psy.cmu.edu/wordpress/wp-content/uploads/2012/12/173Chapter_37_Intelligent_Tutoring_Systems.pdf
11. Misconception-tagged distractors / diagnosis — https://arxiv.org/html/2602.02414v1 ; https://www.cs.cornell.edu/~molly/chi2018.pdf
12. Knowledge Space Theory / ALEKS — https://www.aleks.com/about_aleks/knowledge_space_theory ; https://jmatayoshi.github.io/publications/JMP2021_KST_ALEKS_preprint.pdf ; https://en.wikipedia.org/wiki/Knowledge_space
13. Bjork & Bjork, desirable difficulties — https://bjorklab.psych.ucla.edu/wp-content/uploads/sites/13/2016/04/EBjork_RBjork_2011.pdf ; https://www.unh.edu/teaching-learning-resource-hub/sites/default/files/media/2023-06/itow-introducing-desirable-difficulties-into-practice-and-instruction-bjork-and-bjork.pdf
14. Wilson et al. 2019, The Eighty Five Percent Rule (Nat. Commun. 10:4646) — https://www.nature.com/articles/s41467-019-12552-4 ; https://experts.arizona.edu/en/publications/the-eighty-five-percent-rule-for-optimal-learning ; https://www.sciencedaily.com/releases/2019/11/191105113457.htm
15. Brunmair & Richter 2019, interleaving meta-analysis — https://www.psychologie.uni-wuerzburg.de/fileadmin/06020400/2019/Brunmair_Richter_in_press__2019_META-ANALYSIS_OF_INTERLEAVED_LEARNING.pdf
16. Rohrer et al. 2020, RCT of interleaved mathematics practice — https://gwern.net/doc/psychology/spaced-repetition/2019-rohrer.pdf ; https://files.eric.ed.gov/fulltext/ED595322.pdf
17. Repo-internal (already gated): `FACTORY/study_system/research/spaced_repetition_and_anki.md` (FSRS DSR formulas, revlog schema, optimizer, leech mechanism)
