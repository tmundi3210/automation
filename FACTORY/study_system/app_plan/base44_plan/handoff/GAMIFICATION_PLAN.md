# GAMIFICATION_PLAN — owner request 2026-07-11 (water-drop drill + "other ways to gamify")

_Three specialists were run as design authorities: `engage` (gamification/progress-visual authority — primary), `learner`+`sched` (learning-science guardrail), `qcraft`+`uxguide`+`viz` (question validity + honest UI/encoding). This file is the synthesis. Nothing is built yet; it defines the design, the invariants, the coverage gap (a new specialist), and the integration map._

## Headline verdict
The water-drop drill is a **conditional GO — but narrowly fenced**: an opt-in, isolated automaticity drill whose reward meter is **ephemeral, quarantined from the learning model, and visually unmistakable from real progress.** All three specialists independently reached the same core: **gamify the wall, not the hole in it** — reward showing up to the spaced plan and durable recall, never speed/volume/XP. The single biggest honesty flag: **"answer in time" rewards SPEED, not mastery**, and a clock actively breaks the BKT math — so timed answers must never touch the learning signal.

---

## 1. The water-drop mechanic — spec

**What it is:** an opt-in "timed warm-up / fluency drill" over a pool of *small* MCQs, separate from the FSRS-scheduled Review. Uses the app's existing-but-unused `Card.kind = "skill_drill"` surface [FACT — APP_RECON §6.11].

**Data model — ephemeral session flow-meter, not a record:**
`WaterMeter { level: 0..100 (rendered STEPPED, not smooth), rise_per_hit, decay_per_sec, window_ms }`
- **Rise:** only when `correct == true` AND within the fall window → `level = min(100, level + rise_per_hit)`; drop disappears.
- **Decay:** `level -= decay_per_sec·dt`, floored at 0. **Framed as session momentum ("use it or lose it *this session*"), NOT forgetting/retrievability** — calling it "forgetting" would falsely borrow the FSRS concept. **DECAY MUST BE SESSION-ONLY, never over days** (a multi-day decay creates a pull to review early = spacing violation). [open decision `water_decay_timescale`]
- **Non-persistence invariant:** resets each session; writes NOTHING to day-box pct, course mastery, `LearnerState`/BKT, `ItemElo`, FSRS `CardState`, or `CalibrationBin`.

**What it honestly rewards:** correctness-within-a-window = speed + accuracy = *recognition fluency*, valid ONLY where automaticity is the actual objective. Flagged as such in the UI, never sold as mastery.

**Content routing (qcraft — hard):**
- **ELIGIBLE:** INBDE standalone `remember/understand` terminology gates (anatomy/structure naming, drug-class recognition, definition recall — short homogeneous options); TOEFL already-time-boxed small tasks (Complete-the-Words, possibly Listen-and-Choose), gated current-2026-vs-legacy.
- **PROHIBITED:** INBDE itemsets/cases, any vignette-based apply/analyze item (diagnosis/next-step/mechanism), EXCEPT/NOT items, and all constructed-response/productive tasks. Timing these trains recognition-over-understanding — the exact INBDE anti-goal.
- **The MCQ under the animation is unchanged:** it must still pass the full ship gate (cover-the-options, homogeneous options, real-error distractors, no testwiseness cues). A gamified item that fails cover-the-options is "a broken item wearing an animation."

**Timeout ≠ wrong (biggest validity trap):** a missed drop has no `chosen_option_id` → no misconception, no knowledge verdict, and it is **NOT** an FSRS "Again." UI: *"Correct — but the drop fell first. Speed, not knowledge,"* with an offer to re-see the item untimed. It writes no learning-model evidence.

**Fall-time = ESL-fairness hazard:** the window must exceed a floor that lets a **domain-lite ESL learner** read the stem + all options once — else it silently tests English reading rate. Fall time is a per-card ESTIMATE calibrated to option count + ESL reading rate, with a readable floor — never a fixed cinematic constant.

**Honest UI (uxguide):** water rises only on genuine correctness (never on motion/speed alone); no shame loop on wrong/timeout; first-run onboarding pre-explains the timeout framing.

**Accessibility (hard requirements):** `prefers-reduced-motion` → static countdown numeral/ring + discrete step meter (drop physics is decorative and fully removable; the *timer information* is not). **Selection targets are the STATIC option buttons (keys 1–4 / tap) — never a moving drop** (clicking a moving target is a motor-dexterity test, construct-invalid). Colorblind → level reads by height + numeric label, never green/red water alone.

**Encoding (viz):** fill = position/length (the *most accurate* channel — so it must be **quantized into discrete steps** like the DayBox `floor(...*20)`, signaling "coarse counter" not "measured continuum"). Distinguished from real mastery by four separators: **form** (vertical tank, not a mastery bar/line), **hue** (a cool/cyan token OUTSIDE the mastery green/yellow/red palette), **location** (drill surface only — never Dashboard/Learner-Model page), **label + no persistence** ("session," writes nowhere).

---

## 2. The gamification menu — ship order (engage, verdicts against its anti-patterns)

**SHIP FIRST (all clear engage's four gates today; these fix the "all zeros"):**
1. **Day-box goal-gradient fill** — green download-bar, 5% steps, tick at true 100% (the owner's core; mostly built).
2. **Streak + visible freeze + a "days-active-in-last-30" secondary metric** — build on the already-shipped `streak_slack_tokens` (default 2); a single miss can never zero standing.
3. **Course mastery meter + honest concept-collection** — KG concepts light up as they reach a *real* BKT mastery threshold. This is the concrete fix for the "all zeros," and it's honest collection (maps to real mastered units, not vanity badges).
4. **Future-load forecast** on the calendar (Anki-heatmap precedent; additive, low-risk).

**SHIP LAST (experimental):** the water-drop drill (#1 above) — highest appeal, highest learning-risk, and the piece that needs the new specialist.

**REJECT (engage + learner/sched, dark-pattern or signal-corrupting):** points/XP economy, badges-for-prizes/currency, leaderboard (also single-user out of scope), illusionary head-start progress, variable/random reward schedules, **hearts/lives/lose-on-error resources** (punish errors + suppress the error signal FSRS needs — e.g. Duolingo hearts), **avatars & pets** (vanity/identity mechanics with no single-user audience and no learning denominator), **speed-scored competitive quiz** (Kahoot/Quizizz — rewards speed+volume; even Kahoot dropped its streak-bonus), **cooperative/team play** (no second user).

### 2b. Broader survey — kids + adults apps (engage, 12 apps grounded 2026-07-11)
**The transfer rule [ESTIMATE]:** a "kid" mechanic transfers to this serious single-user adult exam-prepper **iff it can be re-anchored to a real learning denominator (mastery / completion / prerequisite structure) and stripped of any audience-facing or fictional layer.** Collection and level-maps re-anchor cleanly; avatars, pets, co-op, and narrative cannot (their pull *is* the fiction or the audience → REJECT above).

**ADDS to the positive menu (priority order), all rendered honestly over existing data, no new specialist needed:**
1. **KG mastery-map / honest progressive-unlock** (transform of skill-trees/level-maps; Brilliant/Duolingo/Prodigy precedent) — *highest value.* Render the concept knowledge-graph colored by REAL BKT mastery; a concept "opens" as its prerequisites are genuinely mastered. **Unlock is cosmetic/navigational ONLY — it must never gate a review the scheduler wants to surface** (non-punitive contract). Sequencing owned by kgraph+learner+sched. Keep the graph, drop any fantasy skin.
2. **Concept maturation gradient** (Anki young→mature, Memrise seed→flower — minus the point-coupling) — each concept ripens seedling→mature keyed to FSRS stability / BKT tier, in a mastery palette distinct from day-box green; reads the model one-way. Tier cutoffs = a learner decision.
3. **Adjustable daily-target line on the day-box** (Lumosity/Peak "workout of the day", Duolingo daily goal) — a user-set commitment measured against the **scheduler's due+planned denominator** (never a self-invented XP target), non-punitive on 0-due rest days.
4. **Beat-your-own-time "Match"-style companion drill** (Quizlet Match, self-vs-self) — an optional second ephemeral, quarantined `game_timed` fluency minigame on the `skill_drill` surface, same guardrails as the water-drop; **ships LAST, only if the water-drop validates.**
5. **Neutral milestone framing** (thin residue of "quests" — "Clear the Endodontics unit") — named sub-goals as labels on the KG map, no story/fantasy. Lowest priority.

**Prior-art that validates the design (adopt the principle, not the mechanic):** Khan Academy independently separates *energy points* (effort) from *mastery points* (mastery) as disjoint never-merged numbers — the same effort-vs-mastery split already in this plan.

**Out of THIS pass (scope reassignment):** visual mnemonics / memory palace (Sketchy/Osmosis) is high-value for INBDE content but is a **content-authoring** technique (media/kgraph/qcraft), not a gamification mechanic — routed away so it isn't misfiled into engage/graphux.

---

## 3. Binding invariants — "gamification must NEVER…" (learner+sched, hard law)
1. Never let a game reward alter a **mastery estimate** (no game answer writes BKT `P(L)` / Elo θ / item b).
2. Never let a game reward alter a **scheduling interval** (no game/streak activity advances `CardState.due` or feeds the FSRS optimizer's revlog).
3. **Timed-mode data is quarantined:** logged only as a distinct `evidence_class = "game_timed"` (extend `grade_source`), excluded by default from every nightly BKT/Elo replay, diagnosis, and calibration bin. Flag-and-trust is insufficient — the bkt_evidence path was silently broken once already.
4. Never punish a **spacing-correct rest day** — a scheduler-mandated 0-due day satisfies the streak/goal; streak is freeze-protected, never a bare reset.
5. Never reward **raw volume, speed, or XP** over retention outcomes — reward = *scheduled work completed* + *delayed-retrieval success*.
6. Never reward **hammering easy items** (mastered-easy cards yield load first; they are not a point source).
7. Rewards are **informational feedback, not a tangible reward economy**.
8. A game clock **never** stands in for the learner model's latency/honesty signal.
9. The arrow is **one-way**: game *reads* the model, never writes it.
10. Every change touching the `bkt_evidence` / `submitReview` path is **re-verified, never Reverted-and-trusted**.

---

## 4. Coverage verdict — a new specialist IS needed (answers "build the specialist for this purpose")
`engage` (2 day-box-centric KBs) does **NOT** fully cover this request. Unanimous across the three consults:
- **(a) NEW sibling specialist `graphux`** — the game-feel / "juice" / real-time-motion authority that `engage`, `viz`, and `uxguide` all explicitly disclaim. Owns: the drop fall/gravity/easing, water rise/surface, the decay animation curve, splash/particles/squash-stretch, haptics/sound, and the render-performance envelope. **Constrained** by uxguide's honesty contract (motion never implies unmeasured mastery; no shame loop), the reduced-motion removability rule, viz's honest *quantized* encoding, and qcraft's fall-time floor. [FACT — APP_RECON §5: no animation/physics lib in the app today; graphux's build-vs-adopt is UNKNOWN until prototyped.]
- **(b) NEW third KB inside `engage`: `kb_session_drill_mechanics`** — timed-drill / decaying-resource-meter / variable-reward / collection mechanics, judged against engage's four gates. Its current two KBs have no node for any of these (they are day-box-centric).

So "the specialist for this purpose" = **author `graphux` (new) + add `kb_session_drill_mechanics` to `engage`.** Not a from-scratch gamification specialist.

**Broader-survey confirmation (2026-07-11):** the wider concept set in §2b needs **nothing beyond `graphux` + engage's three KBs.** The KG mastery-map and maturation gradient are engage *renders* over kgraph + learner/sched data (all built); the Match drill is covered by `kb_session_drill_mechanics`; celebration/juice is `graphux`. The only tempting "new specialist" is the KG-map, but it is a render over existing data, not a new domain. (Both new specialists — `graphux` + the engage KB — are now AUTHORED and verified 2/2 PASS.)

---

## 5. Open decisions / UNKNOWNs (owner or downstream)
- `water_decay_timescale` — **session-only** (recommended) vs multi-day (rejected: pulls early review). Owner states the constant.
- `game_answer_logging` — quarantine-and-exclude (recommended) vs retain a `game_timed` fluency side-metric (needs the `evidence_class` schema change + bkt_evidence re-verification BEFORE any writer ships).
- Timeout scoring → sched/learner call (recommended: writes nothing to the model).
- Fall-time budget → per-card ESTIMATE, empirically calibrated to ESL reading rate before it ships as a real timer.
- graphux motion-library build-vs-adopt → UNKNOWN until prototyped/measured (present as a measured verdict, not asserted).
- **KG mastery-map unlock (§2b #1) → must be cosmetic/navigational ONLY** — never withholds a scheduled review (learner+sched+kgraph confirm; non-punitive contract).
- **Concept-maturation tier cutoffs (§2b #2)** → which FSRS-stability / BKT values map seedling→mature = a learner decision; UNKNOWN until set.
- **Second (Match) drill (§2b #4)** → owner decides whether a second `game_timed` variant is worth the surface; same fence as the water-drop; ships last.
- **Scope size** → recommended sequence: safe menu first, then §2b #1 (KG-map) + #2 (maturation) as the two highest-value adds, holding #3/#4/#5 until those land.
- TOEFL drill eligibility → current-2026 gate; wrong-answer penalty UNKNOWN (no-penalty only by analogy to the INBDE FACT).

---

## 6. Integration map into the existing system
- **New authoring:** `graphux` specialist (3 KBs + distilled, matching the 16-key schema) + `engage/kb_session_drill_mechanics.kb.json` (+ engage.specialist.json `grounded_in_kbs` updated to reference it).
- **Entities/fields:** `WaterMeter` (client/session state, non-persisted); `Card.kind="skill_drill"` (exists, unused); `Revlog.evidence_class="game_timed"` (extend `grade_source`); nightly replay + diagnosis + CalibrationBin filter that class out.
- **Screens:** a drill/warm-up surface (opt-in), separate from `Review.jsx`; the safe menu (#2 items) lands on the existing Dashboard/day-box/mastery surfaces engage already owns.
- **Sequencing:** (1) author `graphux` + the engage KB [owner picks model]; (2) run the whole gamification design through the specialist set on Fable to produce the concrete build plan; (3) build the SAFE menu first (day-box fill, streak+freeze+30-day, mastery meter + concept-collection, future-load forecast), then the water-drop drill LAST behind all the guardrails above.
- **Slice placement:** this is the long-planned "gamification pass (engage)" from the horizon, now specified — the safe menu is engage's core build; the drill is a fenced experimental sub-feature gated on `graphux`.

## 7. Next step
Per the established process: **owner picks the model to author `graphux` + `kb_session_drill_mechanics`** (recommended: Opus 4.8 authors, Fable runs — the author-with-X / audit-with-Y cross-check). Then Fable runs the gamification design through the full specialist set to produce the build plan, then it goes to Base44 (safe menu first, drill last).

---

## 8. Base44 discussion-plan review + corrections (2026-07-11)
Base44 returned a discussion-mode plan (Phase 1 safe menu, Phase 2 water-drop). Directionally faithful (quarantine, timeout≠wrong, opt-in skill_drill, off-mastery hue, reduced-motion). **Four corrections before build-level detail:**
1. **Day-box is EFFORT, not mastery.** Base44's acceptance said the fill "reflects mastery-based daily units" — conflation. Fill = tasks done ÷ planned (EFFORT); explicitly NOT a mastery metric; mastery is the separate meter. The planner being mastery-aware never makes the bar a mastery number.
2. **The 0.6 concept-light-up threshold is invented.** Per §5 / engage E2, mastery cutoffs are a `learner` decision, `[UNKNOWN]` until set — must come from the learner model's actual mastery definition, never hardcoded (classic BKT "mastered" ≈ 0.95; 0.6 looks low).
3. **Answer window is NOT hardcoded 5s.** qcraft §1.5: a fixed short timer tests ESL reading speed, not knowledge. DECISION (recommended): dynamic per-card window (option count + reading floor). Meter DECAY = session-only (multi-day rejected: pulls early review).
4. **Two survey adds missing.** (a) KG mastery-map must carry the explicit non-punitive invariant — coloring is cosmetic/navigational ONLY, never withholds a scheduled review. (b) Concept-maturation gradient (§2b #2) omitted — add it to the sequence.

**Owner-gate decisions (recommended; owner confirms on relay):**
- `water_decay_timescale` → RECOMMENDED: dynamic per-card answer window + session-only meter decay.
- `game_answer_logging` → RECOMMENDED: strictly ephemeral for first ship (write nothing). Logging as `game_timed` is a gated FOLLOW-UP (needs the `evidence_class` schema addition + bkt_evidence re-verification before any writer ships).

**Detail bar before execution:** the plan must reach build-level — per-item testable acceptance lines, exact entity/field changes, screens touched, and a per-item credit estimate (not just "low-to-moderate").

### 8b. Build-level plan review (2026-07-11) — near-approve, guardrails must become acceptance checks
Base44's build-level plan applied all 4 corrections + both decisions correctly. Blocking gap: the load-bearing invariants are absent from the testable acceptance columns (in this project, un-asserted = unverified). Require these as explicit PASS/FAIL acceptance lines before "RUN":
- **Streak (item 2):** a scheduler-mandated 0-due day counts as active (rest day = win); a miss spends a freeze token, never a bare reset; the 30-day "days active" secondary metric is always visible.
- **Drill content fence (item 7):** the skill_drill pool is restricted to remember/understand terminology gates + TOEFL small-answer; reasoning/vignette/case/EXCEPT items are NOT eligible and cannot be authored as skill_drill.
- **Drill logic (item 8):** selection is on STATIC option buttons only (moving drop never a target); prefers-reduced-motion → static numeral/ring + step meter preserving all timer info; colorblind → height + numeral, not water color; meter rises ONLY on genuine correctness; timeout shows "Correct — but the drop fell first" + offer to re-see untimed, no shame loop; zero writes to Revlog/BKT/Elo/FSRS/CalibrationBin (grep-verifiable).
- **Field-existence (items 4-6, phantom-build guard):** confirm learner.current_mastery / concept_mastery / memory_strength / MasteryHistory actually exist and are read-only before building; render an honest empty state ("not yet studied") when mastery data is absent — never fabricate. Items 5-6 depend on the pending learner threshold.

### 8c. Adversarial double-check (2026-07-11) — NOT approve-ready; 3 BLOCKERs
A specialist refutation pass + APP_RECON/BASE44_APP_SPEC field grep found the build submission would ship phantom fields and a forecast over empty data. Doctrine is sound; the failures are data-layer reality.

**REAL FIELD MAPPING (the submission named fields that DO NOT EXIST):**
- `learner.current_mastery` (item 4) → does NOT exist → use **`LearnerState.bkt_p_mastery`** (per concept).
- `learner.concept_mastery` (item 5) → does NOT exist; `Concept` has no mastery field → **join `LearnerState.bkt_p_mastery` → Concept**.
- `learner.memory_strength` (item 6) → does NOT exist, and is the WRONG owner: "memory strength" = **FSRS stability on `CardState` (sched-owned)**; BKT is no-forgetting so keying maturation to BKT just double-counts item 5.
- `MasteryHistory` exists but is an **aggregate, pipeline-written (manual), stale** snapshot (FK-area/CC-section, ≥2 run dates) — not per-concept, not "current."

**BLOCKERS (must fix before RUN):**
- **B1 (V1) — Streak punishes a rest day.** Shipped streak math scores a 0-due day as a miss (0/0 fails the 0.8 threshold → spends a token). Violates INV-4; §8b "rest day = win" cannot pass on current code. FIX: guard `planned==0`/no-due → day counts active, no token spent. This is a build task, not reuse.
- **B2 (G1) — Item 3 forecast has no data.** `DailyPlan` only holds today + tomorrow (pipeline/manual); no days +2..+7. FIX: re-source to a **scheduler due-forecast counting `CardState.due` per future day (sched-owned)** — a build, not a read; NOT Phase-1-trivial. Never shows tick/green; labeled a forecast.
- **B3 (P1) — Item 6 maturation is incoherent.** `memory_strength` is a phantom + wrong owner. FIX: either DROP item 6 as redundant with mastery, or define it as an aggregate of per-card FSRS stability (a **sched** deliverable) with tier cutoffs set by learner. Do not name it a learner field.

**SHOULD-FIX before RUN:**
- Re-bind items 4/5 to `LearnerState.bkt_p_mastery`; item 4 and item 5 must read the SAME source (or label staleness) so they can't visibly disagree (aggregate MasteryHistory vs live LearnerState).
- Add §8b acceptance lines for items 1 & 3 (item 1: `planned==0` divide-by-zero guard on `floor(done/planned*20)` + effort-not-mastery; item 3: source, never-tick/green, labeled forecast).
- Define the `skill_drill` authoring path + a MECHANICAL content-fence (Bloom=remember/understand + option-homogeneity validation) — `Card.kind="skill_drill"` is today an unused schema promise with NO authoring path; the fence must PREVENT authoring an ineligible item, not just say so.
- Drill surface calls NO backend function (not `submitReview`/`gradeTextAnswer`) — structural fence, not only a grep.
- §6 `game_timed` writer + replay-filter is explicitly DEFERRED (per the ephemeral decision) — not part of this ship.
- Hard-gate items 5-6 on the learner light-up threshold + maturation tiers (a learner decision); replace "dynamic cutoff" with a set value.

**Buildable-now after fixes:** item 1 (with guard), item 2 (with the rest-day fix), items 4/5 (real fields, same source). Item 3 needs sched. Item 6 needs disambiguation. Items 5/6 gated on the learner threshold. Items 7/8 need the authoring path + graphux prototype.

### 8d. Threshold spec (learner+sched, 2026-07-11) — the two cutoffs + build refinements
Fields verified in the live entity schemas: `LearnerState.bkt_p_mastery` (default 0.3) and `CardState.stability` (no default; undefined until first review) both EXIST [FACT].

**① CONCEPT LIGHT-UP (item 5, `bkt_p_mastery`) — headline ≥ 0.95, tiered:**
- **dark** `< 0.5` (the app's own weakness line) · **dim glow** `0.5–0.95` (the 0.6 prereq-unblock line sits here as a "ready to build on" sub-tick) · **full light-up = "mastered", counts for collection** `≥ 0.95` (Corbett & Anderson BKT convention [FACT]; ≈ 3 clean corrects with the app's frozen caps — NOT "dark forever").
- Sits ABOVE the 0.6 readiness line by design (mastery is a stronger claim than readiness). Owner-tunable within [0.90, 0.95]; NEVER down to 0.6.
- "Mastered" claims ~95% P(latent skill known); does NOT claim current recall, calibration, or free-recall (MCQ-guess-inflatable — document the caveat).

**② CONCEPT MATURATION (item 6, `CardState.stability`) — min over review-state cards:**
- Aggregate = **min(stability) over the concept's cards in `review` state** (weakest established card; a concept isn't mature if its weakest card is fragile). Missing S / any non-review card ⇒ **seedling**, and the concept **cannot be "mature."**
- Tiers (days): **seedling `<7` · growing `7–21` · mature `≥21`** (optional evergreen `≥100`). `21` = Anki mature convention via `I(0.9,S)=S` [FACT]; cutoffs [ESTIMATE], owner-tunable. Key on **S (not the displayed interval)** so tiers are independent of the desired_retention slider.

**Light-up ≠ Maturation (not double-counting):** "**Lit = you've learned it; Mature = it will stick.**" BKT is no-forgetting/monotonic; FSRS stability decays on lapse. A concept can be LIT but SEEDLING (the E4 retrieval-decay signature the app already diagnoses).

**BUILD REFINEMENTS this forces (add to Base44):**
- **R1 (honesty) — monotonic light-up lies over time.** Because BKT never decays, a lit concept stays "mastered" forever even after its cards lapse. Make **maturation the dominant/truthful visual**; a lit concept whose `min stability` later collapses must be **visually demoted** (light-up label = "learned/seen," not a permanent trophy).
- **Vocabulary reconcile:** MasteryBars already turn green at `≥0.60` — call that "on track"; reserve "mastered" for the map light-up `≥0.95`. Never two "mastered" signals.
- **Live reads:** light-up/maturation read `LearnerState`/`CardState` LIVE, never the stale manual-pipeline `MasteryHistory` snapshot.
- **Scale:** page past the 500-row query cap per concept and batch the Card→CardState join (N+1) — else concepts mis-light at 2000+ cards.

**Plus 3 earlier refinements:** item 1 rest-day should render a distinct "nothing due / rest" state, NOT a full 20/20 green fill (a full bar reads as "did a full day's work"); item 2 must keep the "days-active-in-last-30" secondary metric in its acceptance; item 7 fence must explicitly bar EXCEPT/NOT stems + gate TOEFL current-2026-vs-legacy.

**STATUS: with these in, the plan is APPROVE-READY.**

### 8e. Final specialist verification (2026-07-11) — NOT approve-ready: 1 bug + dropped acceptance lines
Data-layer fixes (§8c B1/B2/B3, §8d thresholds) ALL landed correctly. Blockers are a logic bug + guardrail acceptance lines that regressed when the drill items were compressed.

**MUST-FIX before RUN:**
1. **Item 7 boolean INVERTED** (currently "prevent authoring UNLESS ... + NOT/EXCEPT stems + TOEFL 2026+" → this REQUIRES except/not stems AND excludes all INBDE). Rewrite as BAR-IF: bar authoring if EXCEPT/NOT stem OR Bloom∉{remember,understand} OR non-homogeneous options OR not a single-best-answer MCQ OR INBDE itemset/vignette/case OR legacy(non-2026) TOEFL. ALLOW only: remember/understand + homogeneous + POSITIVE stem + single-best MCQ + (INBDE terminology gate OR current-2026 TOEFL small task).
2. **Restore item-8 guardrail acceptance lines** (dropped): static-option-button selection (never a moving drop); prefers-reduced-motion → static numeral/ring + step meter preserving 100% timer info; colorblind → height+numeral; rise ONLY on genuine correctness + "speed, not mastery" label; timeout≠wrong copy ("Correct — but the drop fell first") + untimed re-see + no shame loop. (INV-5/INV-8.)
3. **Add session-only meter-decay** acceptance to item 8 (multi-day decay rejected).
4. **Item 5 R1 wording:** "DEMOTE the light-up to 'Learned/seen' (never REVOKE the learned state); maturation is the dominant visual" — and item 5 must READ min(CardState.stability) for the demotion trigger (cross-source dependency to declare).

**SHOULD-FIX (cheap, verification-safety):**
5. Items 5/6: add the N+1 Card→CardState BATCH-JOIN acceptance (only pagination is asserted; batch-join is the 2000+-card mis-light guard).
6. Items 4/5/6: add honest empty-state ("not yet studied") acceptance.
7. Item 2: a 0-due day is a FREE active day (NO token spent); a freeze token is spent only on a MISSED due-work day (else rest days drain the 2-token budget = B1 in disguise).
8. Item 6: "reviewed cards" → "cards in `review` state" (learning/relearning cards excluded).
9. Item 5: add the non-punitive line (light-up coloring cosmetic/navigational only; never withholds a scheduled review).

**Still-open [UNKNOWN] gates on Phase 3:** graphux motion build-vs-adopt (measured verdict), TOEFL wrong-answer penalty, the empirical ESL fall-floor constant, the water decay_per_sec value.

### 8f. Systematic build spec authored → slices/SLICE_GAMIFICATION.md (2026-07-11)
Per-phase specialist agents authored a full build-spec slice (758 lines, per-item (a)GOAL/(b)WHAT/(c)FIELDS/(d)DATA-CONTRACT/(e)GIVEN-WHEN-THEN ACCEPTANCE/(f)UI-STATES/(g)SPECIALIST+INVARIANT/(h)GATES); adversarial verify = **APPROVE_READY** (every §8-8e requirement traces to a testable line; zero invented fields; item-7 BAR-IF; all 8 drill guardrails; item-5 demote-not-revoke; 4 Phase-3 UNKNOWNs gated). This slice is the loadable source of truth Base44 reads per item (replaces the compressible chat table).

**Grounded findings from the deep pass (fix during build):**
- Streak bug B1 CONFIRMED live at `Dashboard.jsx:53/55` (a rest day drains a freeze token); streak is client-recomputed with NO token-ledger entity (persistence is an open choice — G2-a).
- `submitReview`'s `floor(done/planned*20)` WRITER lacks the divide-by-zero guard the Dashboard reader has — fix the writer too (Item 1).
- Item 3 must PAGE `CardState.due` past the hard 500-row cap (the one non-trivial Phase-1 build).
- Item 6 is a **sched deliverable** (min FSRS `CardState.stability`), needs the sched read path; Item 5's demotion is cross-source-dependent on it.

**New design gates the detail surfaced (were hidden by the compressed table):**
- **LOAD-BEARING [UNKNOWN-4a] — Mastery Meter denominator.** Area-mean over ALL concepts (untested ones sit at the 0.3 BKT prior → meter pinned ~30% forever, a new "all-zeros") vs over ONLY-ASSESSED concepts. **RECOMMEND: only-assessed, and show "X of N concepts assessed"** so the number is honest and not prior-dominated. Owner/learner confirm.
- Low-stakes defaults (Base44 may pick, or owner sets): G1-a rest-day encoding = derived flag (no schema change); G2-a streak-token persistence (client-recompute vs minimal token-state); G2-b/G3-a missing-row + forecast inclusion (exclude new-state cards with no due date); G3-b forecast horizon N (default 7 days); G3-c day-bucketing = match the app's existing UTC/local convention.

**STATUS: APPROVE-READY.** Owner: confirm 4a (only-assessed recommended); the rest are safe defaults. Then Base44 loads the slice and runs safe-menu → mastery/maturation → drill last (Phase 3 held behind its 4 [UNKNOWN] gates).

### 8g. Phases 1+2 BUILT (2026-07-11, owner-reported) — cross-check PENDING slice sync
Base44 built Phases 1+2, held Phase 3. **Honest caveat (Base44-disclosed):** SLICE_GAMIFICATION.md never reached the planit workspace (sync gap), so Base44 built from the in-chat approved plan + the assessed-only denominator ruling — NOT the full 758-line slice. Evidence grade = **owner-reported, UNAUDITED against the slice.**
Shipped (Base44-reported): rest-day-safe streak (freeze tokens spared on 0-due days) + 30-day active metric; rest-state day boxes (no full green on planned=0); amber-dashed due-load forecast (never tick/green); Mastery Meter on Learner Model with "(n of N assessed)"; Concept Collection + maturation stages on Library; mastery/maturation lines in the graph node panel; all from LIVE LearnerState/CardState; all cosmetic, nothing gates reviews. Ledger row recorded on the app side.
**OPEN — verify before trusting as done:** the build was NOT checked against the slice's fine-grained acceptance lines. Unconfirmed slice-only items: (a) the submitReview WRITER divide-by-zero guard (not just the reader); (b) batch-join/pagination past the 500-row cap (fails only at 2000+ cards); (c) demote-not-revoke exact behavior (Learned→"Learned/seen" when min-stability<21d); (d) honest empty-states; (e) light-up tiers dark/dim/Learned; (f) "On track"(≥0.60) vs "Learned"(≥0.95) vocabulary non-collision. → CLOSE via: sync SLICE_GAMIFICATION.md to planit, then a Codex verbatim code-vs-slice cross-check (the independent-audit layer). Phase 3 correctly held behind its 4 [UNKNOWN] gates.
