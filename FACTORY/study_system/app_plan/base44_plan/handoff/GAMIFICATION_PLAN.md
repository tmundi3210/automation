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

**REJECT (engage + learner/sched, dark-pattern or signal-corrupting):** points/XP economy, badges-for-prizes/currency, leaderboard (also single-user out of scope), illusionary head-start progress, variable/random reward schedules.

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

---

## 5. Open decisions / UNKNOWNs (owner or downstream)
- `water_decay_timescale` — **session-only** (recommended) vs multi-day (rejected: pulls early review). Owner states the constant.
- `game_answer_logging` — quarantine-and-exclude (recommended) vs retain a `game_timed` fluency side-metric (needs the `evidence_class` schema change + bkt_evidence re-verification BEFORE any writer ships).
- Timeout scoring → sched/learner call (recommended: writes nothing to the model).
- Fall-time budget → per-card ESTIMATE, empirically calibrated to ESL reading rate before it ships as a real timer.
- graphux motion-library build-vs-adopt → UNKNOWN until prototyped/measured (present as a measured verdict, not asserted).
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
