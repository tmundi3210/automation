# SLICE GAMIFICATION — engage progress/gamification pass (water-drop drill + safe menu)

_Repo-driven mode: obey `docs/plan/PROTOCOL.md`. Any specialist path like `FACTORY/study_system/specialists/<code>/<code>.specialist.json` in the directive block resolves HERE to `docs/plan/specialists/<code>.specialist.json`._

**Goal.** Turn the "all zeros" surfaces into an honest, non-punitive progress/gamification pass, built in three phases and shipped in a fixed order. **Phase 1 (safe menu)** hardens the day-box into an honest EFFORT meter, fixes the streak so a scheduler-mandated rest day is a free win (never a token drain), and adds a scheduler-sourced future-load forecast that can never wear a completion tick. **Phase 2 (mastery/maturation)** adds a live course-level BKT mastery meter, an honest concept-collection light-up mapped to real mastery (not vanity badges), and a concept-maturation gradient keyed to FSRS memory strength — three pure one-way reads that never write the learning model. **Phase 3 (drill, ship LAST)** is the narrowly-fenced experimental water-drop timed drill: a mechanical authoring fence so only eligible remember/understand single-best-MCQ automaticity items can be drilled, and an ephemeral in-memory WaterMeter surface that writes NOTHING to the learning model. The binding law across all three phases: **the arrow is one-way — gamification READS the learner/scheduler model, never WRITES it (GAMIFICATION_PLAN §3 INV-9)**; EFFORT and MASTERY are kept strictly separate; a rest day is never punished; every external/numeric/behavioral claim carries its honesty tag ([FACT] / [ESTIMATE] / [UNKNOWN] / [UNVERIFIED-SOURCE]) and an un-asserted acceptance line is unverified.

---

# PHASE 1 — Safe Menu (Items 1–3)

_Governing specialists: **engage** (primary — day-box render model, streak+freeze, future-load forecast), **uxguide** (honest labels / effort≠mastery / staleness), **viz** (honest encoding — fill=length, forecast visually distinct, never-tick), **sched** (CardState.due is the scheduler's state; forecast is a build not a read), **learner** (rest-day = spacing-correct win, non-punitive). Grounded in GAMIFICATION_PLAN §1–3 invariants + §8–8e correction ledger and APP_RECON. Every external/numeric/behavioral claim is tagged [FACT] (read from an entity schema or shipped source this session), [ESTIMATE] (design synthesis) or [UNKNOWN] (owner-gated)._

**Honesty ledger — what was read directly this session (all [FACT]):**
- `base44/entities/DailyPlan.jsonc` — fields: `date` (string, format date), `planned_units` (integer, default 0), `done_units` (integer, default 0), `daybox_state` (integer, min 0 max 20, default 0), `per_exam_alloc_json` (string, no default; unused per APP_RECON §2).
- `base44/entities/CardState.jsonc` — fields: `card_id` (string), `due` (string, format date-time), `state` (enum `new|learning|review|relearning`, default `new`), `stability` (number, **no default** — undefined until first review), `difficulty` (number), `reps` (integer, default 0), `lapses` (integer, default 0), `last_review` (date-time), `fsrs_json` (string). Required: `card_id`, `due`.
- `base44/entities/Settings.jsonc` — `streak_slack_tokens` (integer, default 2), `day_done_threshold` (number, default 0.8), `new_per_day` (integer, default 10), `desired_retention` (number, 0.7–0.97, default 0.9).
- `base44/entities/LearnerState.jsonc` — `bkt_p_mastery` (number, default 0.3) [not used in Phase 1; listed to bound scope].
- `src/pages/Dashboard.jsx:44–60` — the shipped streak loop (see Item 2, the B1 bug lives at line 53/55).
- `src/components/dashboard/DayBoxCalendar.jsx` — `DayBox` renders `pct = state*5`, green fill `height:${pct}%`, tick iff `state===20`, future days `state=null` + `opacity-40`. No "nothing due" state exists today.
- `submitReview/entry.ts:140–150` (per APP_RECON §3a) — writes today's `done_units+1` and `daybox_state = floor(done/planned*20)`.

---

## ITEM 1 — Day-Box Effort Fill

### (a) GOAL
Render each calendar day-box as an **effort** meter — that day's tasks done ÷ planned, green download-bar fill in 5% floored steps, tick only at true 100%, with a rest-day (`planned==0`) shown as a distinct "Nothing due" state, never a full-green day.

### (b) WHAT TO BUILD
The day-box mechanism already exists (`DayBoxCalendar.jsx` + `submitReview` writing `daybox_state`); Phase 1 hardens it to three corrections from the ledger (§8.1, §8c should-fix, §8d rest-day):
1. **Divide-by-zero guard on the writer.** `daybox_state = floor((done/planned)*20)*` must never divide by `planned==0`. When `planned_units==0`, do not compute a ratio — set a sentinel rest-day state (see below), matching the Dashboard reader which already guards with `Math.max(1, planned)` (`Dashboard.jsx:39`) [FACT] but which the `submitReview` writer path does **not** guard (`submitReview/entry.ts:140–150`, per APP_RECON) [FACT].
2. **Distinct rest-day state.** A `planned==0` day (scheduler mandated nothing due) is a *separate* visual token — "Nothing due" / neutral, NOT `daybox_state=20` and NOT a tick. §8d R-3: "item 1 rest-day should render a distinct 'nothing due / rest' state, NOT a full 20/20 green fill (a full bar reads as 'did a full day's work')" [FACT — plan]. Implement as a nullable/negative sentinel distinct from both `0` (planned>0, none done yet → white) and `20` (complete → tick). Recommended: keep `daybox_state` in its 0..20 integer domain and carry the rest-day flag as `planned_units==0 && done_units==0` derived at render time (no schema change), OR reserve a sentinel — see (h)/OWNER GATE G1-a.
3. **"Effort, not Mastery" label.** The day-box surface carries an explicit rung-1 caption "Effort — tasks done today, not mastery" (uxguide honest-translation contract; §8.1 correction: fill = tasks done ÷ planned = EFFORT, explicitly NOT a mastery metric) [FACT — plan]. Mastery is a separate meter (not in Phase 1).

Fill semantics preserved exactly (engage FILL_RENDER_MODEL / FLOOR_ROUNDING_RULE): floor to 5% so `display-100 ⇔ tick ⇔ done==total`; numeral and fill height both driven from the one `daybox_state` value so they cannot disagree.

### (c) ENTITIES / FIELDS
All read from `DailyPlan.jsonc` [FACT] — **no new field required for the recommended derived-flag approach**:
- `DailyPlan.planned_units` — integer, default 0. The denominator.
- `DailyPlan.done_units` — integer, default 0. The numerator.
- `DailyPlan.daybox_state` — integer, min 0, max 20, default 0. The 21-state fill value (`floor(done/planned*20)`; tick at 20).
- `DailyPlan.date` — string (date). Day key (`toISOString().slice(0,10)`, UTC — see risk in (h)).

Formula (engage DAY_BOX_DATA_MODEL): `pct_display = daybox_state * 5`; `daybox_state = min(20, floor((done_units / planned_units) * 20))` **only when `planned_units > 0`**. Rest-day render key = `(planned_units == 0)`.

_No invented field. If the owner prefers a persisted sentinel over a derived flag, that is a schema addition gated at G1-a — do not add silently (viz honest-encoding + no-phantom-field rule, §8c)._

### (d) DATA CONTRACT
- **Read (render):** already shipped — `DailyPlan.list("-date", 62)` (`Dashboard.jsx:20`) [FACT], then `byDate[date] = daybox_state` (`DayBoxCalendar.jsx:35`). Extend the mapping to also carry `planned_units`/`done_units` per day so the renderer can detect `planned==0` rest-days (today the calendar only receives `daybox_state`, §4b `DayBoxCalendar` has no `onClick`, no per-day detail) [FACT].
- **Write (guarded):** in `submitReview` DailyPlan step, replace `daybox_state = floor(done/planned*20)` with:
  `daybox_state = planned_units > 0 ? Math.min(20, Math.floor((done_units / planned_units) * 20)) : 0` and treat `planned_units==0` as the rest-day path (no ratio). [ESTIMATE — exact guard expression]
- **Aggregation:** none beyond the per-day ratio.
- **500-row cap / pagination:** N/A for Item 1 — the calendar reads at most 62 DailyPlan rows (`list("-date",62)`), far under the 500 cap [FACT]. No batch-join. (The cap matters for Item 3, not here.)

### (e) ACCEPTANCE CHECKS (each guardrail = its own PASS/FAIL line)
- **AC1.1 [floor invariant]** GIVEN `done_units=13, planned_units=20` WHEN daybox_state computed THEN `daybox_state==13` and `pct_display==65` (not 70) and NOT tick. — enforces engage `FLOOR_ROUNDING_RULE` / KB FLOOR_ROUNDING_RULE acceptance ("0.99→95, partial").
- **AC1.2 [tick ⇔ complete]** GIVEN `done_units==planned_units>0` WHEN rendered THEN tick shows and `daybox_state==20`; GIVEN `done_units=19, planned_units=20` THEN numeral `95%`, NO tick. — engage `TERMINAL_TICK_STATE` (`state===20` gate, `DayBoxCalendar.jsx:22`) [FACT].
- **AC1.3 [divide-by-zero guard]** GIVEN `planned_units==0` WHEN the writer runs THEN NO division occurs and no `NaN`/`Infinity` is written to `daybox_state` (value stays in 0..20). — enforces GAMIFICATION_PLAN §8c should-fix "item 1: `planned==0` divide-by-zero guard on `floor(done/planned*20)`"; engage DAY_BOX_DATA_MODEL zero-task guard.
- **AC1.4 [rest-day distinct state]** GIVEN `planned_units==0 && done_units==0` WHEN rendered THEN the box shows the "Nothing due" neutral token AND does NOT show a tick AND does NOT show full-green fill. — enforces §8d R-3 (rest day ≠ 20/20 green); learner non-punitive contract (INV-4).
- **AC1.5 [effort-not-mastery label]** GIVEN the day-box surface WHEN rendered THEN a visible rung-1 caption states the fill is *effort (tasks done)*, not mastery. — enforces §8.1 correction; uxguide honest-translation contract (`no heuristic/effort number reads as mastery`).
- **AC1.6 [single source]** GIVEN a day's `daybox_state` WHEN rendered THEN the fill height and the `%` numeral are both derived from that one value and cannot diverge. — engage FILL_RENDER_MODEL acceptance ("numeral and fill driven from the one model pct").
- **AC1.7 [no false tick on rest day]** GIVEN a rest-day box THEN it is visually distinguishable from a completed (ticked) box at a glance. — viz honest-abstraction (a rest box must not read as a finished day).

### (f) UI STATES (enumerated)
- **Empty / not-yet-studied (planned>0, done==0):** white box, day number, no fill, no numeral (matches shipped `state===0 → bg-white`, `pct>0` gate hides numeral at 0) [FACT].
- **Loading:** Dashboard shows the spinner until `data` resolves (`Dashboard.jsx:84–85`) [FACT]; day-boxes render only after `plans` load.
- **Populated / partial (planned>0, 0<done<planned):** white box + green download-bar fill to `pct%` + floored `%` numeral (5..95).
- **Complete (done==planned>0):** green fill + tick, `daybox_state==20`.
- **Edge / rest-day (planned==0):** distinct "Nothing due" neutral token — NEW state to build; today no such state exists (a 0-planned day currently maps to `state=0` white or, past-day, `state ?? 0`) [FACT].
- **Future day:** dimmed (`opacity-40`, `state=null`) — untouched by Item 1 (Item 3 owns future) [FACT].
- **Error:** if a DailyPlan row is malformed/missing, render the empty white state (never fabricate a fill); Dashboard's write-on-load creates today's row (`Dashboard.jsx:32–42`) [FACT].

### (g) SPECIALIST + INVARIANT per acceptance line
- AC1.1/AC1.2/AC1.6 → **engage** KB nodes `FLOOR_ROUNDING_RULE`, `TERMINAL_TICK_STATE`, `FILL_RENDER_MODEL`; invariant `display-100 ⇔ tick ⇔ done==total`.
- AC1.3 → **engage** `DAY_BOX_DATA_MODEL` (tasks_total==0 guard) + GAMIFICATION_PLAN §8c.
- AC1.4/AC1.7 → **learner/sched** INV-4 (never punish a spacing-correct rest day) + §8d R-3; **viz** honest-abstraction.
- AC1.5 → **uxguide** honest-translation contract; **engage** `EFFORT_VS_MASTERY_SEPARATION`; GAMIFICATION_PLAN §8.1.

### (h) OWNER GATES / [UNKNOWN]
- **G1-a [rest-day encoding choice]** — derived flag (`planned==0`, no schema change, recommended) vs a persisted `daybox_state` sentinel (schema addition). [UNKNOWN — owner/impl decision]. Do not add a field silently (§8c phantom-field guard).
- **G1-b [past incomplete-day policy]** — freeze at final `%` vs gray-out is an engage `PAST_DAY_STATE_POLICY` [UNKNOWN] carried from the KB; a switch-either-way design stores the frozen pct regardless. Not blocking Item 1 fill, but must be resolved before "past day" visuals ship.
- **G1-c [weighting]** — unweighted task count is the literal reading (engage `TASK_WEIGHTING_DECISION`); effort-weighting is an [UNKNOWN] behind the denominator. Phase 1 = unweighted.
- **Known risk (not a gate, must be flagged):** UTC/local day-key mismatch — `date` keys are UTC `toISOString().slice(0,10)` while the calendar builds local `Date`s (APP_RECON §6.5) [FACT]; for negative-UTC-offset users evening work can land on the wrong box. Pre-existing; Item 1 does not fix it but must not assume local==UTC.

---

## ITEM 2 — Streak + Metrics

### (a) GOAL
A freeze-protected daily streak that stays **active on a 0-due / rest day spending NO token**, spends a freeze token **only on a missed due-work day**, and always shows a secondary "days-active-in-last-30" metric a single miss cannot zero.

### (b) WHAT TO BUILD
This is a **build/fix**, not reuse — the shipped streak has the B1 bug. Current logic (`Dashboard.jsx:44–60`) [FACT]:
```
const done = p && p.planned_units > 0 && p.done_units / p.planned_units >= threshold;  // line 53
if (done) streak++;
else if (slack > 0) { slack--; }   // line 55 — a rest day (planned==0) drains a token
else break;
```
A `planned_units==0` rest day makes `done=false` → falls to line 55 → **spends a slack token**. With only 2 tokens (`streak_slack_tokens` default 2) [FACT], two rest days exhaust the budget and the third breaks the streak — a scheduler-mandated rest day is scored as a miss. This violates INV-4 and cannot pass §8b "rest day = win". **B1 fix (§8c):** classify each past day into three outcomes, not two:
1. **Active-by-work** — `planned_units>0 && done_units/planned_units >= day_done_threshold` → `streak++`, no token spent.
2. **Active-by-rest** — `planned_units==0` (nothing was due) → `streak++`, **NO token spent** (a rest day is a free win; §8e should-fix 7: "a 0-due day is a FREE active day"). [FACT — plan]
3. **Missed due-work day** — `planned_units>0 && ratio < threshold` → spend one freeze token if available (`streak` preserved), else break. Token spent **only here**.

Plus **secondary metric (always visible):** `days_active_in_last_30` = count of the last 30 calendar days that are Active-by-work OR Active-by-rest. A single miss cannot zero it (engage `SECONDARY_HORIZON_METRIC`). §8b + §8d require it in acceptance [FACT].

### (c) ENTITIES / FIELDS
Read from `Settings.jsonc` + `DailyPlan.jsonc` [FACT]:
- `Settings.streak_slack_tokens` — integer, default 2. The freeze budget.
- `Settings.day_done_threshold` — number, default 0.8. The "day counts as done" ratio (a day-level threshold, NOT a mastery number — uxguide label).
- `DailyPlan.planned_units`, `DailyPlan.done_units`, `DailyPlan.date` — as Item 1.

**No streak/token entity exists** — the streak is computed **client-side per Dashboard load and is not persisted**; `slack` is re-seeded from `Settings.streak_slack_tokens` every render and decremented only within the loop (`Dashboard.jsx:46,55`) [FACT]. Consequence: "spending a token" today is ephemeral (recomputed each load), not a durable ledger. If the owner wants *persisted* token consumption (a token truly gone until refilled), that needs a new persistence surface — see G2-a. Phase 1 recommended: keep the streak a **derived/recomputed** value over the DailyPlan history (no new entity), which is honest and idempotent.

### (d) DATA CONTRACT
- **Read:** `DailyPlan.list("-date", 62)` already loaded (`Dashboard.jsx:20`) [FACT]; the streak walks back day-by-day from yesterday (`Dashboard.jsx:48–58`). 62 rows covers a ~60-day back-walk and the 30-day window; both under the 500 cap → no pagination needed [FACT]. If a streak longer than ~60 days must be counted exactly, raise the `list` limit (still < 500 for < 1.4 yr) or page — [ESTIMATE].
- **Aggregation — streak (replace the two-branch loop):** for each day `d` from yesterday backward: look up `p = plans[d]`; classify Active-by-work / Active-by-rest / Missed as in (b); `streak++` on either active; on Missed decrement an in-loop token, break when tokens exhausted. Then add today if today is active. [ESTIMATE — control flow]
- **Aggregation — secondary metric:** `days_active_in_last_30 = Σ over the last 30 calendar days of [day is Active-by-work OR Active-by-rest]`. A day with no DailyPlan row and no due cards is Active-by-rest (nothing was due) — **but a missing row is ambiguous** (could be an un-created plan): treat "no row AND no CardState was due that day" as rest; else missed. Simplest honest Phase-1 rule: a day with `planned_units==0` (or no row where nothing came due) counts active. [ESTIMATE — ambiguity resolution; see G2-b].
- **Batch-join / 500 cap:** none for the recommended DailyPlan-only computation. (If rest-day detection is instead re-derived from historical CardState.due, that crosses the 500 cap and needs Item-3-style pagination — avoid; use DailyPlan.planned_units as the rest signal.)

### (e) ACCEPTANCE CHECKS
- **AC2.1 [rest day = active, no token]** GIVEN a day with `planned_units==0` WHEN the streak is computed THEN that day counts as active AND the token budget is unchanged. — INV-4; §8c B1; §8e should-fix 7. (This is the shipped bug's fix — FAIL on current code.)
- **AC2.2 [token spent only on missed due-work day]** GIVEN `planned_units>0 && ratio < day_done_threshold` WHEN computed THEN exactly one freeze token is spent (if available) and the streak is preserved; GIVEN tokens exhausted THEN the streak breaks. — engage `STREAK_FREEZE_REPAIR` (consumed one-per-miss, never retroactive); INV-4.
- **AC2.3 [miss never bare-resets]** GIVEN a single missed due-work day with ≥1 token WHEN computed THEN the streak is NOT zeroed. — engage `STREAK_ANXIETY_GUARDRAIL`; §8b "a single miss can never zero standing".
- **AC2.4 [active-by-work]** GIVEN `planned_units>0 && ratio >= day_done_threshold` THEN `streak++`, no token spent. — baseline correctness.
- **AC2.5 [secondary metric always visible]** GIVEN the Dashboard WHEN rendered (any streak value, including 0) THEN "days active in last 30" is shown. — engage `SECONDARY_HORIZON_METRIC`; §8b + §8d "always visible"; §8e should-fix.
- **AC2.6 [secondary metric miss-resilient]** GIVEN one missed day inside the window WHEN computed THEN `days_active_in_last_30` decreases by at most 1 and is not zeroed. — engage secondary-metric definition.
- **AC2.7 [rest days don't drain budget over time]** GIVEN N consecutive rest days (N > streak_slack_tokens) WHEN computed THEN the streak survives all N. — direct guard against "B1 in disguise" (§8e should-fix 7).
- **AC2.8 [honest label]** GIVEN the streak surface THEN copy does not imply the streak measures mastery or learning quality; it measures daily return. — uxguide honest-translation; overjustification guardrail (informational, not a reward economy).

### (f) UI STATES
- **Empty / new user (no DailyPlan history):** streak `0`, "days active in last 30" `0`, with an onboarding caption ("Your streak starts when you finish a day's due work; rest days keep it alive") — uxguide first-run/empty-state, avoid the onboarding cliff.
- **Loading:** Dashboard spinner until `data` resolves [FACT].
- **Populated:** streak number (`StatsRow` "Streak" tile, `Dashboard.jsx:112`) [FACT] + a visible freeze-token indicator (tokens remaining) + the "days active in last 30" secondary tile (NEW).
- **Edge / rest-day active:** streak increments with a distinct "rest day counted" affordance (no token badge decrement) — makes AC2.1 visible.
- **Edge / freeze spent:** show that a token was consumed (e.g. "1 freeze used") without alarm (no shame loop — uxguide/engage streak-anxiety guardrail).
- **Error:** if Settings is missing, default `streak_slack_tokens=2`, `day_done_threshold=0.8` (schema defaults) [FACT] — never crash the streak.

### (g) SPECIALIST + INVARIANT per acceptance line
- AC2.1/AC2.7 → **learner/sched** INV-4 (never punish a spacing-correct rest day); GAMIFICATION_PLAN §8c B1, §8e#7.
- AC2.2/AC2.3/AC2.4 → **engage** `STREAK_MECHANIC` + `STREAK_FREEZE_REPAIR` + `STREAK_ANXIETY_GUARDRAIL`; §8b.
- AC2.5/AC2.6 → **engage** `SECONDARY_HORIZON_METRIC`; §8b, §8d.
- AC2.8 → **uxguide** honest-translation + **engage** `OVERJUSTIFICATION_GUARDRAIL` (INV-7, informational feedback not a reward economy).

### (h) OWNER GATES / [UNKNOWN]
- **G2-a [token persistence]** — ephemeral recomputed streak (recommended, no new entity, idempotent) vs a persisted token ledger (new surface; note APP_RECON §6.1 "no scheduler exists" and §6.6 "Dashboard writes on read" race — a persisted ledger would need careful write ownership). [UNKNOWN — owner/impl]. Do not invent an entity silently.
- **G2-b [missing-row ambiguity]** — how to score a past calendar day with **no** DailyPlan row: rest (nothing due) vs missed. Recommended: rest only if no CardState was due that day; otherwise the safest non-punitive default is "not counted against the streak" but "not counted active" for the 30-day metric. [UNKNOWN — needs owner call; must be stated, not guessed].
- **G2-c [threshold source]** — `day_done_threshold` default 0.8 is the day-completion bar, distinct from any mastery/retention number; confirm the owner wants 0.8 (do not conflate with `desired_retention` 0.9). [ESTIMATE — carried from schema default].

---

## ITEM 3 — Future-Load Forecast

### (a) GOAL
Show, on the calendar, the **count of scheduled reviews due per future day for N days**, sourced from the FSRS scheduler state (`CardState.due`) — a distinct "Forecast" encoding that **never shows a tick or completion-green** and is never mistaken for a finished day.

### (b) WHAT TO BUILD
A **new scheduler-sourced read + aggregation** — this is a build, not a read of existing data (§8c B2). The B2 blocker: `DailyPlan` only holds **today + tomorrow** (written by Dashboard-load, `submitReview`, and pipeline stage 4) — there are no `+2..+N` rows [FACT — APP_RECON §3c]. So the forecast **cannot** come from DailyPlan. It must come from the scheduler's own state: count `CardState` rows whose `due` falls on each future calendar day. This mirrors Anki's future-due heatmap (engage `FUTURE_LOAD_FORECAST` / `ANKI_HEATMAP_PRECEDENT`) [FACT — plan]. Render as a forecast layer on the future (currently dimmed, `state=null`) day-boxes:
- **Distinct encoding (viz + engage):** the forecast is NOT the green completion fill and NEVER the tick. Use a visually separate channel — e.g. a count numeral or a cool/neutral load bar clearly outside the completion-green semantics — so a forecast box can never read as a completed day (engage `future-load forecast … can never show the tick`; viz honest-abstraction + separable-channel) [FACT — plan].
- **Labeled "Forecast":** rung-1 caption "Forecast — reviews the scheduler expects to fall due, not work you've done" (uxguide; it is a projection of scheduler state, and staleness applies — see below).

### (c) ENTITIES / FIELDS
Read from `CardState.jsonc` [FACT]:
- `CardState.due` — string, date-time. The per-card next-due timestamp (FSRS-scheduled). **The forecast's sole source.**
- `CardState.state` — enum `new|learning|review|relearning`, default `new`. Used to decide inclusion: the forecast is *review load*; `new`-state cards are created due-immediately (`CardForm`/`generateCards` create CardState due now, APP_RECON §4d) so they populate "today/overdue", not future days. Recommended: count future-due cards with `state != 'new'` (i.e. actually scheduled reviews) — [ESTIMATE, see G3-a].
- `CardState.card_id` — string. Identity (not needed for a pure count).

**No DailyPlan field is read for the forecast** — using DailyPlan would be the B2 phantom-data bug (it has no future rows) [FACT]. No new entity is required for a read-only count; the forecast is computed at render time.

### (d) DATA CONTRACT
- **Read (paginated past the 500 cap — this is the load-bearing part):** the existing due query `CardState.filter({ due: { $lte: nowIso } }, "due", 500)` (`Dashboard.jsx:18`) fetches only **past/now-due** and is capped at 500 with silent truncation (APP_RECON §6.3) [FACT]. The forecast needs **future** rows: `CardState.filter({ due: { $gt: endOfTodayIso, $lte: horizonIso } }, "due", 500)` where `horizonIso = end of day today+N`. At the owner's planned scale (2000+ cards, APP_RECON §6.3) [FACT] a mature deck can exceed 500 future-due rows across the window, so a single call silently under-counts. **Pagination approach:** page by ascending `due` cursor — fetch 500 sorted by `due`, then re-query `due > lastSeenDue` until a page returns `< 500` rows (or the horizon is passed). Because sort is by `due`, the cursor is monotonic and safe across pages. [ESTIMATE — cursor pagination pattern; Base44 SDK filter+sort+limit is [FACT], explicit cursor pages are the standard workaround for the hard 500 cap].
- **Aggregation:** bucket the fetched future-due rows by calendar day: `dayKey = due.slice(0,10)` (UTC — see risk); `forecast[dayKey] += 1`. Emit a count per future day for `date in (today+1 .. today+N)`. Zero-fill days with no due cards (a genuine "light day"). [ESTIMATE — bucketing].
- **N (horizon):** default `N = 7` [ESTIMATE — matches a week view; owner-tunable, G3-b]. Month view can show up to the visible future days.
- **Batch-join:** none needed — the forecast is a pure count over CardState; it does **not** join Card or CardState→Card (unlike Items 5/6 mastery/maturation which do). If the forecast is later enriched (e.g. per-exam split), that join must be batched, not N+1 (APP_RECON §6.4) — out of Phase-1 scope.

### (e) ACCEPTANCE CHECKS
- **AC3.1 [scheduler-sourced, not DailyPlan]** GIVEN the forecast for day today+3 WHEN computed THEN it counts `CardState.due` rows on that day and reads NO `DailyPlan` row (which does not exist for +3). — enforces §8c B2; sched (CardState is the scheduler's state).
- **AC3.2 [never tick / never completion-green]** GIVEN any forecast day-box WHEN rendered THEN it shows NO tick and NOT the completion-green fill; its encoding is visually distinct from a done day. — engage `FUTURE_LOAD_FORECAST` ("style clearly distinct … can never show the tick"); viz honest-abstraction; §8c should-fix.
- **AC3.3 [labeled forecast]** GIVEN the forecast surface THEN a visible caption labels it a forecast/projection, not completed work. — uxguide honest-translation; §8c should-fix ("labeled forecast").
- **AC3.4 [pagination past 500]** GIVEN > 500 future-due CardState rows within the horizon WHEN the forecast is computed THEN all are counted (paged), NOT silently truncated at 500. — APP_RECON §6.3 hard-cap guard; sched scale note; §8d "page past the 500-row query cap".
- **AC3.5 [zero-fill honest]** GIVEN a future day with no due cards WHEN rendered THEN it shows a genuine "0 / light" forecast, not blank-because-unqueried. — viz honest-abstraction (absence shown as real zero).
- **AC3.6 [no write]** GIVEN the forecast render THEN it writes nothing to CardState/DailyPlan/LearnerState/FSRS (read-only projection). — INV-2/INV-9 (game reads the model, never writes; a forecast must not advance `CardState.due`).
- **AC3.7 [staleness honest]** GIVEN the forecast reflects the current `CardState.due` snapshot WHEN a review later reschedules a card THEN the forecast is understood as a live projection that shifts as cards are answered (not a promise). — uxguide staleness disclosure (the forecast is a moving projection; note the pipeline/UTC caveats).

### (f) UI STATES
- **Empty / no future-due cards:** future boxes show "no reviews scheduled" / light state — never a fill. (A brand-new user with only due-now new cards will legitimately have an empty forecast — uxguide honest empty state.)
- **Loading:** while the paged future-due query runs, show a subtle loading affordance on the forecast layer (it is a second async read beyond the current Dashboard loads).
- **Populated:** each future day shows its review count in the distinct forecast encoding (numeral / cool load bar), dimmed relative to real completion boxes (future days are already `opacity-40`, `state=null`, `DayBoxCalendar.jsx:84,86`) [FACT].
- **Edge / heavy day:** a spike day (large count) is legible without implying it is "done" — no green, no tick.
- **Error / truncation-averted:** if a page fails, show the partial forecast with an explicit "partial — could not load all" rather than a silently-truncated wrong number (uxguide: never false precision).

### (g) SPECIALIST + INVARIANT per acceptance line
- AC3.1/AC3.4 → **sched** (CardState.due is the scheduler's state; forecast is a build over it, §8c B2) + APP_RECON §6.3 cap.
- AC3.2/AC3.5 → **engage** `FUTURE_LOAD_FORECAST` + `ANKI_HEATMAP_PRECEDENT`; **viz** honest-abstraction + separable-channel (never-tick).
- AC3.3/AC3.7 → **uxguide** honest-translation + staleness disclosure.
- AC3.6 → GAMIFICATION_PLAN INV-2 / INV-9 (never advance `CardState.due`; one-way read).

### (h) OWNER GATES / [UNKNOWN]
- **G3-a [inclusion rule]** — count future-due cards with `state != 'new'` (review load, recommended) vs all future-due CardState. Also decide whether `learning`/`relearning` (short intraday intervals) belong in a per-*day* forecast. [UNKNOWN — sched/owner call]. State it; do not guess into the encoding.
- **G3-b [horizon N]** — default 7 [ESTIMATE]; owner-tunable (week vs month view span). Must not exceed what the paginated query can honestly cover.
- **G3-c [UTC/local day bucketing]** — `due.slice(0,10)` buckets by UTC day while the calendar is local (APP_RECON §6.5) [FACT]; for negative-UTC-offset users a card due late-evening local can bucket into the next day's forecast. Must be flagged; recommended to bucket in the user's local day to match the calendar. [UNKNOWN — timezone policy, shared with G1's known risk].
- **G3-d [distinct encoding token]** — the exact cool/neutral hue and mark are a **viz** deliverable (perceptual, CVD-safe, WCAG on the dark theme, tagged ESTIMATE-to-source-verify per viz honesty stop); Phase 1 fixes only the *semantic* (not-green, not-tick, labeled), not the hex. [UNKNOWN — viz styling, non-blocking to the data contract].
- **Structural dependency:** the forecast is the one Phase-1 item that needs the scheduler read path built; it is NOT "Phase-1-trivial" (§8c B2). It has no dependency on the learner threshold (Items 5/6) and can ship independently of them.

---

## Phase 1 cross-item summary

| Item | Buildable now? | New entity/field? | 500-cap pagination? | Blocking gate |
|---|---|---|---|---|
| 1 Day-Box Effort Fill | Yes (with guard + rest-day state) | No (derived rest-flag) | No (≤62 rows) | G1-a encoding choice |
| 2 Streak + Metrics | Yes (B1 fix + secondary metric) | No (recomputed) | No (≤62 rows) | G2-a persistence, G2-b missing-row |
| 3 Future-Load Forecast | Yes, but needs scheduler read built (§8c B2) | No (read-only count) | **Yes — page future `CardState.due`** | G3-a inclusion, G3-c timezone |

**Invariants enforced across Phase 1:** INV-2 (no game activity advances `CardState.due` — Item 3), INV-4 (rest day never punished — Items 1 & 2), INV-7 (informational feedback, no reward economy — all), INV-9 (one-way read, never write — Item 3). Effort (day-box) and mastery are kept strictly separate (engage `EFFORT_VS_MASTERY_SEPARATION`; §8.1) — no Phase-1 item reads or writes BKT/FSRS mastery.

---

# PHASE 2 — Mastery, Collection, Maturation (Items 4–6)

Governing specialists: **engage** (progress-visual / effort-vs-mastery separation, honest collection, non-punitive contract), **learner** (BKT semantics, thresholds, no-forgetting), **sched** (FSRS stability, review-state, interval inversion), **viz** (honest quantized encoding, redundant channels, honest empty-state).

Scope: Items 4 (Mastery Meter), 5 (Concept Collection / light-up), 6 (Concept Maturation). Read-only over the live learner/scheduler model. **Arrow is one-way: these visuals READ the model, never WRITE it** (GAMIFICATION_PLAN §3 INV-9). Nothing here touches `submitReview`/BKT/Elo/FSRS/CalibrationBin.

## Phase 2 shared data-layer facts (verified against live entity schemas)

All [FACT] read directly from `scratchpad/base44_app/base44/entities/*.jsonc` on 2026-07-12.

| Entity.field | Type | Default | Source |
|---|---|---|---|
| `LearnerState.concept_id` | string (required) | — | `LearnerState.jsonc:5-7,41-43` |
| `LearnerState.bkt_p_mastery` | number | **0.3** | `LearnerState.jsonc:8-11` |
| `LearnerState.last_updated` | string(date-time) | — | `LearnerState.jsonc:36-39` |
| `CardState.card_id` | string (required) | — | `CardState.jsonc:5-7,44-47` |
| `CardState.state` | enum `new\|learning\|review\|relearning` | **new** | `CardState.jsonc:12-21` |
| `CardState.stability` | number | **none — undefined until first review** | `CardState.jsonc:22-24` |
| `CardState.difficulty` | number | none | `CardState.jsonc:25-27` |
| `Card.concept_ids` | array<string> | — | `Card.jsonc:17-22` |
| `Card.kind` | enum `srs_card\|skill_drill\|adaptive_module_sim` | srs_card | `Card.jsonc:8-16` |
| `Concept.exam_code` | enum `INBDE\|TOEFL` (required) | — | `Concept.jsonc:5-11` |
| `Concept.name` | string (required) | — | `Concept.jsonc:12-14` |
| `Concept.inbde_fk` | integer 1–10 | — | `Concept.jsonc:26-30` |
| `Concept.inbde_cc_section` | enum `DTP\|OHM\|PP` | — | `Concept.jsonc:36-43` |
| `Concept.toefl_section` | enum `R\|L\|S\|W` | — | `Concept.jsonc:44-52` |

**Phantom-field guard [FACT — GAMIFICATION_PLAN §8c]:** `learner.current_mastery`, `learner.concept_mastery`, `learner.memory_strength`, `Concept.<any mastery field>` **DO NOT EXIST**. Never name them. Mastery = `LearnerState.bkt_p_mastery`; memory strength = `CardState.stability`.

**Join topology (load-bearing for Items 5 & 6):**
`Concept` --(`LearnerState.concept_id == Concept.id`)--> mastery.
`Concept` --(`Card.concept_ids CONTAINS Concept.id`)--> `Card` --(`CardState.card_id == Card.id`)--> stability/state.
The Concept→Card step and the Card→CardState step are **two separate queries**; naive per-card `CardState.get` is the N+1 pattern APP_RECON §6.4 calls out.

**Live-read invariant [FACT — GAMIFICATION_PLAN §8d "Live reads"]:** Items 4/5/6 read `LearnerState`/`CardState` **live**, NEVER the `MasteryHistory` snapshot — `MasteryHistory` is pipeline-written (manual `runPipeline` button, delete-then-bulkCreate, keyed by run_date), per-area not per-concept, and stale between manual runs [FACT — APP_RECON §3b stage 5, §6.1]. Reading it would make Item 4 (area meter) and Item 5 (per-concept) visibly disagree.

**500-row cap [FACT — APP_RECON §6.3]:** Base44 `.filter`/`.list` silently truncate at 500 rows. At the owner's planned 2000+ cards this is crossed for CardState and plausibly for Concept/LearnerState. Every read below MUST paginate past 500 and batch the Card→CardState join, or concepts mis-light/mis-mature (GAMIFICATION_PLAN §8d "Scale", §8e should-fix #5).

---

## ITEM 4 — Mastery Meter (course-level, area-mean)

### (a) GOAL
A course-level meter showing real BKT mastery aggregated per INBDE/TOEFL area, live, honest — the concrete fix for the "all zeros," distinct from the day-box effort bar.

### (b) WHAT TO BUILD
A read-only meter (or row of meters) per **area group**. For each group, value = **arithmetic mean of `LearnerState.bkt_p_mastery` over the concepts in that group**. Render on the mastery/course surface (LearnerModel page — reuse the existing `MasteryBars` surface). Colour/label by the **"On track" line ≥ 0.60** (this reuses the existing MasteryBars green-at-60% behavior). This is a **MASTERY** metric and must be visually and lexically separate from the day-box **EFFORT** bar — never merged into one number (engage EFFORT_VS_MASTERY_SEPARATION).

Vocabulary reconcile [FACT — GAMIFICATION_PLAN §8d]: the ≥0.60 green here is labelled **"On track"**, NOT "mastered." The word "mastered" is reserved for Item 5's ≥0.95 light-up. Never two "mastered" signals.

### (c) ENTITIES / FIELDS
- Reads `LearnerState.bkt_p_mastery` (number, default 0.3) [FACT — `LearnerState.jsonc:8-11`].
- Groups by `LearnerState.concept_id` → `Concept.inbde_fk` (1–10) / `Concept.inbde_cc_section` (DTP/OHM/PP) / `Concept.toefl_section` (R/L/S/W) [FACT — `Concept.jsonc:26-52`], mirroring the existing 10-FK + 3-CC axes [FACT — APP_RECON §4c].
- **No new field.** No writes. (engage: course metric consumes the learner-model output as numerator only.)

### (d) DATA CONTRACT
Read query (paginate past 500):
```
concepts   = paginate(Concept.filter({exam_code}))           # all concepts, batched by 500
states     = paginate(LearnerState.list())                    # all LearnerState rows, batched by 500
byConcept  = index states by concept_id
for each area-group G (fk|1..10, cc|DTP/OHM/PP, toefl|R/L/S/W):
    members = concepts in G that HAVE a LearnerState row      # see [UNKNOWN] gate below
    mean_G  = sum(byConcept[c].bkt_p_mastery for c in members) / |members|
    state_G = "On track" if mean_G >= 0.60 else "Building"    # honest label, floor threshold
```
Aggregation formula: `mean_G = (1/|members|) · Σ bkt_p_mastery`. [FACT — identity; mirrors APP_RECON §4c MasteryBars mean.]
Empty group (`|members| == 0`) → honest empty state, NOT 0% and NOT 0.3-default fill (see UI states).
Pagination: loop `.filter(..., {limit:500, offset:n*500})` until a short page; assert total pulled ≥ prior counts so silent truncation can't under-count the mean (APP_RECON §6.3).

### (e) ACCEPTANCE CHECKS
- **A4.1 (aggregate correctness)** GIVEN area FK6 has 3 concepts with `bkt_p_mastery` 0.4/0.6/0.8 WHEN the meter renders THEN FK6 shows mean 0.60 and state "On track". *(learner BKT mastery read; engage COURSE_LEVEL_PROGRESS)* — PASS/FAIL.
- **A4.2 (On-track threshold)** GIVEN a group mean of 0.599 WHEN rendered THEN it is NOT "On track"; GIVEN 0.600 THEN it IS "On track". *(GAMIFICATION_PLAN §8d vocabulary reconcile — 0.60 = "on track", never "mastered")* — PASS/FAIL.
- **A4.3 (effort/mastery separation)** GIVEN a day with day-box 20/20 green (full effort) but group mastery mean 0.35 WHEN both render THEN the mastery meter shows "Building" (~35%), never inherits the green/tick. *(engage EFFORT_VS_MASTERY_SEPARATION; INV-5 no effort masquerading as mastery)* — PASS/FAIL.
- **A4.4 (live source, not snapshot)** GIVEN `MasteryHistory` is stale (last `runPipeline` days ago) but `LearnerState` was updated by a review 1 min ago WHEN the meter renders THEN it reflects the live `LearnerState` value, not the snapshot. *(GAMIFICATION_PLAN §8d "Live reads"; APP_RECON §3b/§6.1)* — PASS/FAIL.
- **A4.5 (honest empty state)** GIVEN an area with zero LearnerState rows WHEN rendered THEN it shows "not yet studied", NOT 0% and NOT a 0.3-default fill. *(viz honest abstraction; GAMIFICATION_PLAN §8e should-fix #6)* — PASS/FAIL.
- **A4.6 (pagination past 500)** GIVEN >500 LearnerState rows WHEN the meter aggregates THEN all rows are pulled (batched) and the mean is computed over the full set, not the first 500. *(APP_RECON §6.3; engage validation)* — PASS/FAIL.
- **A4.7 (read-only)** GIVEN the meter renders/refreshes WHEN inspected THEN it issues zero writes to any entity (grep-verifiable: no `.create/.update/.upsert` in the meter path). *(INV-9 one-way arrow)* — PASS/FAIL.

### (f) UI STATES
- **empty / not-yet-studied:** area has no LearnerState rows → neutral "not yet studied" chip, no fill, no percentage.
- **loading:** skeleton meter; never render 0% as a placeholder (0% is a real value).
- **populated:** per-area fill + mean %; ≥0.60 = "On track" (green), else "Building".
- **edge / partial area:** some concepts in the area have rows, some don't → mean over rows-present, with a "(n of N concepts assessed)" honesty suffix (mirrors existing `· N` count in MasteryBars).
- **rest-day:** N/A (mastery is not day-scoped; no rest-day semantics).
- **error:** query failure → "mastery unavailable," never a fabricated value.

### (g) SPECIALIST + INVARIANT per line
A4.1 engage COURSE_LEVEL_PROGRESS + learner BKT · A4.2 GAMIFICATION_PLAN §8d + engage EFFORT_VS_MASTERY_SEPARATION · A4.3 engage EFFORT_VS_MASTERY_SEPARATION + INV-5 · A4.4 GAMIFICATION_PLAN §8d live-reads · A4.5 viz honest-abstraction + §8e#6 · A4.6 APP_RECON §6.3 + engage validation · A4.7 INV-9.

### (h) OWNER GATES / [UNKNOWN]
- **[RESOLVED-4a — owner-locked 2026-07-11: option (i) ONLY-ASSESSED concepts (those with a LearnerState row), with an explicit "(n of N assessed)" coverage suffix so the un-assessed remainder is never hidden] Denominator definition** — does the area-mean run over (i) only concepts that have a `LearnerState` row, or (ii) ALL concepts in the area, counting never-touched ones at the 0.3 prior/`bkt_pL0` default? Load-bearing: (ii) drags every mean toward 0.3 and reads more honestly as "coverage," (i) reports only assessed concepts and can look inflated. **GATE before build.** Recommended [ESTIMATE, medium]: option (i) with an explicit "(n of N assessed)" coverage suffix so the un-assessed remainder is never hidden.
- **OWNER GATE-4b** confirm the "On track" line stays at **0.60** (matches shipped MasteryBars) and is never relabelled "mastered."

---

## ITEM 5 — Concept Collection / Light-up

### (a) GOAL
Honest "collection": each Concept lights up as it reaches real BKT mastery — mapped to mastered units, never vanity badges; cosmetic/navigational only.

### (b) WHAT TO BUILD
Per-concept light-up driven by `LearnerState.bkt_p_mastery`, in three tiers:
- **dark** `< 0.5` (the app's own weakness line) [FACT — GAMIFICATION_PLAN §8d].
- **dim glow** `0.5 – < 0.95` (the 0.60 prereq-readiness line sits inside this band as a "ready to build on" sub-tick).
- **"Learned" (full light-up, counts for collection)** `≥ 0.95` (Corbett & Anderson BKT convention) [FACT — GAMIFICATION_PLAN §8d; learner glossary ~85% rule / BKT].

**R1 demotion (honesty — §8e must-fix #4):** because BKT is no-forgetting/monotonic, a lit concept stays "Learned" forever even after its cards lapse — that would lie over time. So Item 5 **READS the maturation aggregate cross-source** = `min(CardState.stability over the concept's review-state cards)`; if that min `< 21d`, **DEMOTE the label from "Learned" to "Learned/seen"** — a softer past-tense label. **NEVER REVOKE the learned/lit state** (the tier stays ≥0.95 lit; only the label softens). Maturation (Item 6) is the dominant/truthful visual; the light-up is the weaker claim. Distinction: **"Lit = you've learned it; Mature = it will stick."**

**Non-punitive contract [FACT — GAMIFICATION_PLAN §5 / §8e should-fix #9]:** light-up colouring is **cosmetic/navigational ONLY — it must NEVER gate or withhold a review the scheduler wants to surface.** A dark concept is not blocked; a lit concept is not fast-forwarded.

### (c) ENTITIES / FIELDS
- Reads `LearnerState.bkt_p_mastery` (number, default 0.3) [FACT — `LearnerState.jsonc:8-11`] → tier.
- Reads `CardState.stability` (number, no default) + `CardState.state` (enum, default new) [FACT — `CardState.jsonc:12-24`] over the concept's `review`-state cards → demotion trigger.
- Join `Concept` → `Card.concept_ids CONTAINS concept.id` → `CardState.card_id` [FACT — `Card.jsonc:17-22`, `CardState.jsonc:5-7`].
- **No new field. No writes.** (engage HONEST_COLLECTION: collection maps only to a real BKT mastery threshold, never to activity/badges.)

### (d) DATA CONTRACT
```
mastery = byConcept[c.id].bkt_p_mastery ?? 0.3          # default prior if no row
tier    = mastery >= 0.95 ? "learned"
        : mastery >= 0.5  ? "dim"
        : "dark"
# cross-source demotion (only relevant when tier=="learned"):
minS    = MIN(cs.stability for cs in cardStates(c)      # Item-6 aggregate, reused
              where cs.state == "review" and cs.stability is defined)
label   = (tier=="learned" and (minS is undefined or minS < 21)) ? "Learned/seen" : "Learned"
```
- `cardStates(c)`: **batch-join** — collect all card ids for the concept, then fetch their CardStates in **paged batches (≤500 per query)**, NOT per-card `.get` (N+1). [FACT — APP_RECON §6.3/§6.4; §8e should-fix #5.]
- `21` = Anki mature convention via `I(0.9,S)=S` [FACT — GAMIFICATION_PLAN §8d; sched FSRS interval-inversion glossary].
- If the concept has no `review`-state card with defined stability → `minS` undefined → demote (a "Learned" claim with no established memory strength is exactly the case to soften).

### (e) ACCEPTANCE CHECKS
- **A5.1 (tier cutoffs)** GIVEN mastery 0.49/0.50/0.94/0.95 WHEN rendered THEN tiers are dark/dim/dim/learned respectively. *(GAMIFICATION_PLAN §8d; learner BKT)* — PASS/FAIL.
- **A5.2 (headline mastered = 0.95, not 0.60)** GIVEN mastery 0.60 WHEN rendered THEN the concept is **dim** ("ready to build on"), NOT "Learned"; only ≥0.95 lights fully. *(GAMIFICATION_PLAN §8b correction #2 / §8d — 0.6 light-up is invented; owner-tunable only within [0.90,0.95], never down to 0.6)* — PASS/FAIL.
- **A5.3 (R1 demotion, cross-source)** GIVEN mastery 0.97 (lit) AND `min(stability over review-state cards) = 12d` WHEN rendered THEN label = "Learned/seen" AND the concept **stays lit** (tier unchanged). *(GAMIFICATION_PLAN §8e must-fix #4; INV — BKT no-forget vs FSRS decay)* — PASS/FAIL.
- **A5.4 (never revoke)** GIVEN a concept was ever ≥0.95 lit WHEN its min-stability later collapses THEN the lit tier is NEVER downgraded to dim/dark by the maturation signal — only the label softens to "Learned/seen." *(§8e must-fix #4 "never REVOKE the learned state")* — PASS/FAIL.
- **A5.5 (non-punitive)** GIVEN a dark (<0.5) concept with a card due now WHEN the scheduler builds the review queue THEN the card is surfaced normally; GIVEN a lit concept THEN its cards are neither suppressed nor advanced. Light-up writes nothing to `CardState.due`/queue order. *(GAMIFICATION_PLAN §5 / §8e#9; engage NON_PUNITIVE_SCHEDULER_CONTRACT; INV-2)* — PASS/FAIL.
- **A5.6 (honest collection, not activity)** GIVEN a concept with many drills/reviews done but `bkt_p_mastery` still 0.4 WHEN rendered THEN it is dark — collection maps to mastery, never to volume/streak/hits. *(engage HONEST_COLLECTION; INV-5/INV-6)* — PASS/FAIL.
- **A5.7 (batch-join, no mis-light at scale)** GIVEN a concept with >500 associated cards WHEN the demotion trigger computes THEN all review-state CardStates are batched (not first-500, not per-card N+1) and the min is exact. *(APP_RECON §6.3/§6.4; §8e#5)* — PASS/FAIL.
- **A5.8 (live source)** GIVEN stale `MasteryHistory` WHEN a concept renders THEN its tier is from live `LearnerState`, not the snapshot. *(GAMIFICATION_PLAN §8d live-reads)* — PASS/FAIL.
- **A5.9 (honest empty state)** GIVEN a concept with no `LearnerState` row WHEN rendered THEN it shows the "not yet studied" dark/neutral state (prior 0.3 → dark), never a fabricated glow. *(viz honest-abstraction; §8e#6)* — PASS/FAIL.
- **A5.10 (read-only)** GIVEN light-up renders WHEN inspected THEN zero writes to any entity (grep-verifiable). *(INV-9)* — PASS/FAIL.

### (f) UI STATES
- **empty / not-yet-studied:** no LearnerState row → neutral "not yet studied" (renders as dark, labelled distinctly from an assessed-but-weak dark if the surface allows).
- **loading:** skeleton concept tiles.
- **populated:** dark / dim / learned tiers; "Learned" vs "Learned/seen" label per demotion.
- **edge / lit-but-fragile:** ≥0.95 with min-stability <21d → lit + "Learned/seen" (the E4 retrieval-decay signature).
- **rest-day:** N/A (concept-scoped, not day-scoped).
- **error:** query failure → tiles render "unavailable," never a default glow.

### (g) SPECIALIST + INVARIANT per line
A5.1 learner BKT + §8d · A5.2 §8b#2/§8d · A5.3/A5.4 §8e#4 (BKT-no-forget vs FSRS-decay) · A5.5 engage NON_PUNITIVE_SCHEDULER_CONTRACT + INV-2 · A5.6 engage HONEST_COLLECTION + INV-5/6 · A5.7 APP_RECON §6.3/6.4 + §8e#5 · A5.8 §8d live-reads · A5.9 viz honest-abstraction + §8e#6 · A5.10 INV-9.

### (h) OWNER GATES / [UNKNOWN]
- **OWNER GATE-5a** light-up threshold is SET at **0.95** (learner decision, §8d), owner-tunable **only within [0.90, 0.95]** — NEVER down to 0.60. Confirm on relay.
- **Cross-source dependency to declare:** Item 5's demotion label depends on Item 6's aggregate (`min(CardState.stability over review-state cards) < 21d`). Item 6 MUST ship (or its aggregate be available) for the R1 demotion to function; if Item 6 aggregate is unavailable, default to "Learned/seen" (the conservative/honest label) rather than the permanent "Learned" trophy.
- Caveat to surface in copy [FACT — §8d]: "Learned" claims ~95% P(latent skill known) via MCQ-inflatable evidence; does not claim current recall or calibration.

---

## ITEM 6 — Concept Maturation

### (a) GOAL
Show whether a learned concept will *stick*: a seedling→growing→mature gradient keyed to real FSRS memory strength, distinct from and dominant over the light-up.

### (b) WHAT TO BUILD
Per-concept maturation tier = **`MIN(CardState.stability)` over the concept's cards in `review` state** (the weakest established card — a concept isn't mature if its weakest card is fragile). Tiers (days):
- **seedling** `< 7`
- **growing** `7 – < 21`
- **mature** `≥ 21` (optional evergreen `≥ 100`)

**Missing-S / non-review guard [FACT — GAMIFICATION_PLAN §8d, §8e must-fix wording]:** if the concept has **no defined stability** on any card, OR **any** of its cards is in a **non-`review`** state (`new`/`learning`/`relearning`), the concept ⇒ **seedling** and **cannot be "mature."** (`learning`/`relearning` cards excluded from the min but their presence caps the concept at seedling — §8e should-fix #8: "cards in `review` state," not "reviewed cards.")

**Key on S, not the displayed interval [FACT — §8d]:** aggregate over `CardState.stability`, NOT over `due`-derived interval — so tiers are independent of the `desired_retention` slider. This is a **sched-owned** deliverable (FSRS memory strength), NOT a learner field; keying maturation to BKT would double-count Item 5 (§8c B3).

Distinction preserved: **"Lit = you've learned it (BKT, no-forget); Mature = it will stick (FSRS stability, decays on lapse)."** A concept can be LIT but SEEDLING.

### (c) ENTITIES / FIELDS
- Reads `CardState.stability` (number, **no default — undefined until first review**) [FACT — `CardState.jsonc:22-24`].
- Reads `CardState.state` (enum `new|learning|review|relearning`, default new) [FACT — `CardState.jsonc:12-21`] → only `review` cards feed the min; any non-`review` card caps at seedling.
- Join `Concept` → `Card.concept_ids CONTAINS concept.id` → `CardState.card_id` [FACT — `Card.jsonc:17-22`, `CardState.jsonc:5-7`].
- **No new field. No writes.** ("memory_strength" is a phantom — never name it; use `CardState.stability`, §8c.)

### (d) DATA CONTRACT
```
cs        = cardStates(concept)                      # batched, paged ≤500 (shared with Item 5)
reviewCS  = [x for x in cs if x.state == "review" and x.stability is defined]
nonReview = any(x.state != "review" for x in cs)     # new/learning/relearning present
if reviewCS is empty or nonReview or any stability undefined among review cards:
    tier = "seedling"                                # cannot be mature
else:
    minS = MIN(x.stability for x in reviewCS)
    tier = minS >= 100 ? "evergreen"
         : minS >= 21  ? "mature"
         : minS >= 7   ? "growing"
         : "seedling"
```
- Aggregate = `MIN(stability)` over review-state cards [FACT — §8d]. `21` = Anki mature via `I(0.9,S)=S` [FACT — §8d; sched interval-inversion]. `7`/`21`/`100` cutoffs [ESTIMATE — §8d, owner-tunable].
- **Batch-join + pagination:** identical shared read as Item 5 — collect card ids per concept, fetch CardStates in ≤500-row batches, never per-card `.get`. At 2000+ cards a per-card join both N+1-stalls and silently truncates (APP_RECON §6.3/6.4).

### (e) ACCEPTANCE CHECKS
- **A6.1 (min over review cards)** GIVEN a concept whose review-state cards have stability {5, 30, 40}d WHEN rendered THEN tier = **seedling** (min 5 < 7), not mature — the weakest card governs. *(GAMIFICATION_PLAN §8d min-aggregate)* — PASS/FAIL.
- **A6.2 (tier cutoffs)** GIVEN min-stability 6.9 / 7.0 / 20.9 / 21.0 WHEN rendered THEN tiers = seedling / growing / growing / mature. *(§8d tiers)* — PASS/FAIL.
- **A6.3 (missing S ⇒ seedling)** GIVEN a concept with a review-state card whose `stability` is undefined WHEN rendered THEN tier = seedling, never mature. *(§8d "Missing S ⇒ seedling"; CardState.stability no default)* — PASS/FAIL.
- **A6.4 (non-review card caps at seedling)** GIVEN a concept with cards in states {review S=30, learning} WHEN rendered THEN tier = seedling (a `learning`/`new`/`relearning` card present ⇒ not mature). *(§8d "any non-review card ⇒ seedling"; §8e#8 "cards in review state")* — PASS/FAIL.
- **A6.5 (key on S, not interval)** GIVEN two concepts with identical `stability` but different `due` (because `desired_retention` differs) WHEN rendered THEN they show the SAME tier — tier is independent of the retention slider. *(§8d "key on S not interval"; sched acquisition-vs-retention-dial separation)* — PASS/FAIL.
- **A6.6 (dominant over light-up / Lit-but-seedling)** GIVEN a concept LIT (mastery 0.97) but min-stability 4d WHEN both Item 5 and Item 6 render THEN Item 6 shows **seedling** and Item 5's label reads "Learned/seen" — maturation is the truthful dominant visual. *(§8e#4 R1; "Lit ≠ Mature")* — PASS/FAIL.
- **A6.7 (batch-join at scale)** GIVEN a concept with >500 cards WHEN the min computes THEN all review-state CardStates are batched (not first-500, not N+1) and the min is exact. *(APP_RECON §6.3/6.4; §8e#5)* — PASS/FAIL.
- **A6.8 (sched-owned, not a learner/BKT key)** GIVEN the maturation value WHEN traced THEN it is derived ONLY from `CardState.stability`/`state`, never from `bkt_p_mastery` (no BKT double-count). *(§8c B3; sched ownership)* — PASS/FAIL.
- **A6.9 (honest empty state)** GIVEN a concept with no CardState rows (unstudied) WHEN rendered THEN it shows "not yet studied"/seedling-empty, never a fabricated mature state. *(viz honest-abstraction; §8e#6)* — PASS/FAIL.
- **A6.10 (read-only)** GIVEN maturation renders WHEN inspected THEN zero writes to any entity (grep-verifiable). *(INV-9)* — PASS/FAIL.

### (f) UI STATES
- **empty / not-yet-studied:** no CardState / no review-state card → seedling-empty labelled "not yet studied."
- **loading:** skeleton gradient.
- **populated:** seedling / growing / mature (/ evergreen) keyed on min-S.
- **edge / lit-but-fragile:** mastered concept whose weakest review card is <7d → seedling (the retrieval-decay signature).
- **rest-day:** N/A (concept-scoped).
- **error:** query failure → "maturation unavailable," never a fabricated tier.

### (g) SPECIALIST + INVARIANT per line
A6.1/A6.2 sched FSRS-stability + §8d · A6.3 §8d missing-S + CardState schema · A6.4 §8d/§8e#8 review-state filter · A6.5 sched acquisition-vs-retention separation + §8d key-on-S · A6.6 §8e#4 R1 (BKT-no-forget vs FSRS-decay) · A6.7 APP_RECON §6.3/6.4 + §8e#5 · A6.8 §8c B3 sched ownership · A6.9 viz honest-abstraction + §8e#6 · A6.10 INV-9.

### (h) OWNER GATES / [UNKNOWN]
- **OWNER GATE-6a** maturation tier cutoffs SET at **seedling <7 / growing 7–21 / mature ≥21** (+ optional evergreen ≥100); the 7/100 edges are [ESTIMATE — §8d], `21` is the Anki mature [FACT]; all owner-tunable. Confirm on relay.
- **Ownership note:** this is a **sched deliverable** (min-of-FSRS-stability). If Phase 2 build capacity is learner/engage-only, flag Item 6 as needing the sched read path before RUN (it is not a learner field and not phantom `memory_strength`).

---

## Phase 2 cross-item honesty & guardrail summary

- **One-way arrow (INV-9):** Items 4/5/6 are pure reads — grep-verify no `.create/.update/.upsert` in any of the three render paths. A4.7 / A5.10 / A6.10.
- **Live over snapshot (§8d):** all three read `LearnerState`/`CardState` live; `MasteryHistory` is banned as a source. A4.4 / A5.8.
- **Effort ≠ mastery (INV-5):** Item 4 never inherits the day-box green/tick. A4.3.
- **Non-punitive (INV-2, §5):** Item 5 light-up is cosmetic/navigational; it never gates or reorders a scheduled review. A5.5.
- **Lit ≠ Mature (§8e#4):** Item 5 (BKT, monotonic, never revoked, label-demoted) and Item 6 (FSRS min-stability, decays) are distinct; maturation is dominant. A5.3/A5.4/A6.6.
- **Scale (§8d/§8e#5):** every Concept→Card→CardState read paginates past 500 AND batch-joins (no N+1). A4.6 / A5.7 / A6.7.
- **Honest empty state (§8e#6):** all three render "not yet studied," never fabricate. A4.5 / A5.9 / A6.9.
- Every numeric threshold tagged: 0.60 on-track [FACT §8d], 0.95 learned [FACT §8d, owner-tunable [0.90,0.95]], 21d mature [FACT §8d], 7d/100d edges [ESTIMATE §8d], 0.3 mastery default [FACT LearnerState.jsonc], stability no-default [FACT CardState.jsonc].

---

# PHASE 3 — Water-Drop Timed Drill (Items 7–8) — SHIP LAST

Governing specialists: **engage** (`kb_session_drill_mechanics`: WaterMeter, four gates G1–G4, quarantine, timeout≠wrong, honest collection, session-only decay), **qcraft** (item typology, cover-the-options / homogeneity ship-gate, one-best-answer model, EXCEPT/NOT ban, TOEFL 2026-vs-legacy gating, ESL/per-item time budget), **graphux** (game-feel motion, reduced-motion removability, compositor render budget, build-vs-adopt measured verdict), **uxguide** (honest-translation contract, no-shame framing, onboarding pre-explain), **viz** (quantized-honest / blue-not-green / height+numeral colorblind encoding).

Scope: **Item 7 Drill Authoring Fence** (mechanical, BAR-IF, UI-input-layer prevention) and **Item 8 Drill Surface** (ephemeral in-memory WaterMeter, zero backend writes). This is the **ship-LAST, narrowly-fenced** experimental sub-feature (GAMIFICATION_PLAN §2 "SHIP LAST", §5, §8e). It is gated behind the four still-open [UNKNOWN]s listed at the end and behind the graphux motion prototype.

**Binding cross-item invariant [FACT — GAMIFICATION_PLAN §3]:** the arrow is one-way. Nothing in Phase 3 writes `Revlog`, `CardState`, `LearnerState`/BKT, `ItemElo`, FSRS state, `DailyPlan`, or `CalibrationBin`. A timeout is never an FSRS "Again". The drill READS the eligible-card pool and READS nothing from the learning model beyond the card content it renders.

## Phase 3 shared data-layer facts (verified against live entity schemas 2026-07-12)

All [FACT] read directly from `scratchpad/base44_app/base44/entities/*.jsonc`.

| Entity.field | Type | Default | Source |
|---|---|---|---|
| `Card.kind` | enum `srs_card\|skill_drill\|adaptive_module_sim` | **srs_card** | `Card.jsonc:8-16` |
| `Card.answer_type` | enum `mcq\|free_text\|speaking\|writing` | **mcq** | `Card.jsonc:29-38` |
| `Card.bloom` | enum `remember\|understand\|apply\|analyze\|evaluate\|create` | **none (unset)** | `Card.jsonc:39-49` |
| `Card.prompt` | string (required) | — | `Card.jsonc:23-25,71-74` |
| `Card.patient_box` | string | — | `Card.jsonc:54-56` |
| `Card.case_group_id` | string | — | `Card.jsonc:57-59` |
| `Card.standalone` | boolean | **true** | `Card.jsonc:60-63` |
| `Card.concept_ids` | array<string> | — | `Card.jsonc:17-22` |
| `Card.quality_flag` | string | — | `Card.jsonc:67-69` |
| `ItemOption.card_id` | string (required) | — | `ItemOption.jsonc:5-7,19-22` |
| `ItemOption.text` | string (required) | — | `ItemOption.jsonc:8-10` |
| `ItemOption.is_correct` | boolean | **false** | `ItemOption.jsonc:11-14` |
| `ItemOption.misconception_tag` | string | — | `ItemOption.jsonc:15-17` |
| `Concept.exam_code` | enum `INBDE\|TOEFL` (required) | — | `Concept.jsonc:5-11` |
| `Concept.toefl_task_type` | string (free) | — | `Concept.jsonc:53-55` |
| `Concept.toefl_section` | enum `R\|L\|S\|W` | — | `Concept.jsonc:44-52` |
| `Revlog.grade_source` | enum `mcq\|llm\|pron\|voice_async\|self` | — | `Revlog.jsonc:59-68` |

**Unused-surface fact [FACT — APP_RECON §6.11, §2].** `Card.kind="skill_drill"` is a schema promise with **NO authoring path and NO reader** in the shipped app — `CardForm.jsx`/`generateCards` only ever create `srs_card`, and `Review.jsx` reads `CardState` due-order, never `kind`. Phase 3 is the first code to write and read this value. Item 7 is therefore a **new authoring path**, not a reuse.

**`game_timed` is NOT in the schema [FACT — `Revlog.jsonc:59-68`].** `grade_source` has five enum values; `game_timed` is absent. Adding it (the §6 `evidence_class` extension) is **explicitly DEFERRED** and out of Phase 3 scope [FACT — GAMIFICATION_PLAN §8c "explicitly DEFERRED", §5 `game_answer_logging` recommended strictly-ephemeral]. Phase 3 writes no Revlog at all, so it needs no new enum value; if a later phase adds `game_timed`, the `bkt_evidence`/`submitReview` path must be re-verified, never reverted-and-trusted (INV-3/INV-10).

**No legacy/current vintage field on TOEFL [FACT — `Concept.jsonc`].** `Concept.toefl_task_type` is a free string; nothing marks a task-type as 2026-current vs legacy. The Item-7 TOEFL gate therefore cannot read a flag — it must match `toefl_task_type` against a **curated allowlist of current-2026 small-task types** authored from qcraft's 2026 inventory. This allowlist is a build-time constant, not a schema read.

**500-row cap [FACT — APP_RECON §6.3].** Base44 `.filter`/`.list` silently truncate at 500 rows. The drill pool read (Item 8) and the Card→ItemOption join must paginate past 500 and batch-join, or the pool silently under-populates at scale.

---

## ITEM 7 — Drill Authoring Fence

### (a) GOAL
Mechanically prevent an ineligible item from ever being authored (or re-tagged) as `Card.kind="skill_drill"`, so the timed pool can only ever contain remember/understand single-best-MCQ automaticity items — a broken/ineligible item can never "wear the animation."

### (b) WHAT TO BUILD
A **BAR-IF validation gate at the UI input layer** of the skill_drill authoring path (a new drill-authoring form, or a `kind` selector added to `CardForm`). The gate evaluates the item + its `ItemOption` set against a predicate and **structurally blocks the `Card.create`/`Card.update` that would set `kind="skill_drill"`** until the predicate passes (disabled submit + inline reason, not a post-hoc lint). This is prevention, not detection [FACT — GAMIFICATION_PLAN §8c should-fix "the fence must PREVENT authoring an ineligible item, not just say so"].

**BAR-IF predicate — author is BARRED from setting `kind="skill_drill"` if ANY of:**
1. **`answer_type != "mcq"`** (not a single-best MCQ) — `Card.jsonc:29-38`.
2. **`count(ItemOption where is_correct==true) != 1`** (not single-best: zero or multiple keys) — `ItemOption.jsonc:11-14`.
3. **`bloom ∉ {remember, understand}`** — including `bloom` **unset** (no default), which cannot be confirmed as remember/understand → BAR — `Card.jsonc:39-49`.
4. **EXCEPT/NOT stem:** `Card.prompt` matches a negative-lead-in scan (case-insensitive whole-word `EXCEPT | NOT | LEAST | FALSE`, plus the reviewer confirm) — qcraft ban on negative lead-ins.
5. **Non-homogeneous options / fails cover-the-options:** the item does not pass qcraft's homogeneity + cover-the-options ship-gate (mechanical proxy: options share one length band / one category; final call is the qcraft ship-gate flag on the card).
6. **INBDE itemset / vignette / case:** `patient_box` non-empty **OR** `case_group_id` non-empty **OR** `standalone==false` — `Card.jsonc:54-63`.
7. **Legacy (non-2026) TOEFL:** `Concept.exam_code=="TOEFL"` **AND** `Concept.toefl_task_type ∉` the current-2026 small-task allowlist.

**ALLOW `kind="skill_drill"` only if ALL of:** `bloom ∈ {remember, understand}` **AND** homogeneous options (passes cover-the-options) **AND** POSITIVE stem (no EXCEPT/NOT) **AND** single-best MCQ (`answer_type=="mcq"` and exactly one `is_correct`) **AND** ( **INBDE terminology gate**: `exam_code=="INBDE"` ∧ `standalone==true` ∧ empty `patient_box` ∧ empty `case_group_id` **OR** **current-2026 TOEFL small task**: `exam_code=="TOEFL"` ∧ `toefl_task_type ∈` allowlist ). [FACT — GAMIFICATION_PLAN §8e must-fix #1 exact BAR-IF rewrite; qcraft DRILL_CONTENT_ELIGIBILITY / CQ_D08; engage DRILL_CONTENT_ELIGIBILITY.]

### (c) ENTITIES / FIELDS
- Writes `Card.kind = "skill_drill"` — only on ALLOW [FACT — `Card.jsonc:8-16`; this is the sole entity write in Item 7, an authoring write, not a learning-model write].
- Reads `Card.answer_type`, `Card.bloom`, `Card.prompt`, `Card.patient_box`, `Card.case_group_id`, `Card.standalone` [FACT — `Card.jsonc:29-38,39-49,23-25,54-63`].
- Reads `ItemOption.is_correct` (count of keys), `ItemOption.text` (homogeneity) for the card [FACT — `ItemOption.jsonc:11-14,8-10`].
- Reads `Concept.exam_code`, `Concept.toefl_task_type` via `Card.concept_ids` [FACT — `Concept.jsonc:5-11,53-55`, `Card.jsonc:17-22`].
- **No new entity field.** The current-2026 TOEFL allowlist is a **build-time constant** (curated from qcraft's 2026 task inventory), NOT a schema field — because no vintage field exists (see shared facts). `Card.quality_flag` (`Card.jsonc:67-69`) MAY be reused to stamp the qcraft ship-gate verdict but is not required.

### (d) DATA CONTRACT
Authoring-time (single item, no aggregation):
```
options   = ItemOption.filter({card_id: card.id})          # one card's options, ≤ handful
keyCount  = count(o in options where o.is_correct)
concept   = Concept.get(card.concept_ids[0])               # exam gate
BAR = (card.answer_type != "mcq")
   OR (keyCount != 1)
   OR (card.bloom not in {"remember","understand"})        # unset counts as BAR
   OR negativeStem(card.prompt)                             # EXCEPT/NOT/LEAST/FALSE scan
   OR not homogeneous(options)                              # qcraft cover-the-options ship-gate
   OR (card.patient_box not empty) OR (card.case_group_id not empty) OR (card.standalone == false)
   OR (concept.exam_code == "TOEFL" AND concept.toefl_task_type not in TOEFL_2026_SMALL_TASK_ALLOWLIST)
ALLOW = not BAR
# create/update is blocked unless ALLOW; kind="skill_drill" cannot be persisted otherwise
```
Pool-integrity read (for the surface + a periodic re-validation over any pre-existing skill_drill rows), paginate past 500:
```
drillPool = paginate(Card.filter({kind:"skill_drill"}))     # ≤500 batches (APP_RECON §6.3)
optsByCard = batch-join ItemOption.filter({card_id IN chunk(drillPoolIds, ≤500)})  # NOT per-card .get (N+1)
assert every card in drillPool still satisfies ALLOW        # re-run the predicate; quarantine any that regressed
```
The BAR-IF predicate is [FACT — GAMIFICATION_PLAN §8e#1 exact wording]. The homogeneity/cover-the-options call is qcraft's ship-gate [FACT — qcraft item-writing KB]. The TOEFL allowlist membership is [ESTIMATE — curated from qcraft's 2026 inventory; the wrong-answer-penalty consequence is [UNKNOWN], see (h)].

### (e) ACCEPTANCE CHECKS (each guardrail = its own PASS/FAIL)
- **A7.1 (bar non-MCQ)** GIVEN a card `answer_type="free_text"` WHEN the author sets `kind="skill_drill"` THEN create is BLOCKED with reason "drill items must be single-best MCQ". *(qcraft one-best-answer model; engage DRILL_CONTENT_ELIGIBILITY)* — PASS/FAIL.
- **A7.2 (bar not-single-best)** GIVEN an MCQ with 0 or ≥2 `is_correct` options WHEN set skill_drill THEN BLOCKED. *(qcraft one-best-answer; §8e#1 "single-best MCQ")* — PASS/FAIL.
- **A7.3 (bar Bloom)** GIVEN `bloom="apply"` (or `analyze`/`evaluate`/`create`, or **unset**) WHEN set skill_drill THEN BLOCKED. *(qcraft Bloom-bounded-by-format; engage — timing apply/analyze trains recognition-over-understanding, the INBDE anti-goal; §8e#1)* — PASS/FAIL.
- **A7.4 (bar EXCEPT/NOT stem)** GIVEN `prompt` containing "EXCEPT" / "NOT" / "LEAST" / "which is FALSE" WHEN set skill_drill THEN BLOCKED. *(qcraft ban on negative lead-ins; GAMIFICATION_PLAN §8d "item 7 fence must explicitly bar EXCEPT/NOT stems")* — PASS/FAIL.
- **A7.5 (bar non-homogeneous)** GIVEN options that fail cover-the-options / homogeneity WHEN set skill_drill THEN BLOCKED — "a gamified item that fails cover-the-options is a broken item wearing an animation." *(qcraft homogeneous-options ship-gate; engage TIMED_DRILL_MODE)* — PASS/FAIL.
- **A7.6 (bar INBDE itemset/vignette/case)** GIVEN `patient_box` non-empty OR `case_group_id` non-empty OR `standalone==false` WHEN set skill_drill THEN BLOCKED. *(qcraft itemset-vs-case typology; engage — INBDE itemsets/cases PROHIBITED; GAMIFICATION_PLAN §1 content routing)* — PASS/FAIL.
- **A7.7 (bar legacy TOEFL)** GIVEN a TOEFL card whose `toefl_task_type` is NOT in the 2026 allowlist (e.g. legacy prose-summary select-3-of-6 or an integrated task) WHEN set skill_drill THEN BLOCKED. *(qcraft current-vs-legacy gating; GAMIFICATION_PLAN §8d "gate TOEFL current-2026-vs-legacy", §8e#1)* — PASS/FAIL.
- **A7.8 (allow eligible INBDE terminology gate)** GIVEN `exam_code=INBDE`, `bloom=remember`, positive stem, single-best MCQ, homogeneous, `standalone=true`, empty `patient_box`/`case_group_id` WHEN set skill_drill THEN create SUCCEEDS and `kind="skill_drill"` persists. *(engage/qcraft ELIGIBLE: INBDE remember/understand terminology gate)* — PASS/FAIL.
- **A7.9 (allow eligible 2026 TOEFL small task)** GIVEN `exam_code=TOEFL`, `toefl_task_type ∈` allowlist, `bloom=understand`, positive single-best MCQ, homogeneous WHEN set skill_drill THEN SUCCEEDS. *(engage/qcraft ELIGIBLE: current-2026 TOEFL small task)* — PASS/FAIL.
- **A7.10 (mechanical, not advisory)** GIVEN any BARRED item WHEN the author attempts to bypass (submit anyway / direct save) THEN NO `Card` row with `kind="skill_drill"` is persisted — the block is at the write/input layer, not a dismissible warning. *(GAMIFICATION_PLAN §8c "must PREVENT authoring … not just say so")* — PASS/FAIL.
- **A7.11 (pool re-validation)** GIVEN a pre-existing `skill_drill` card that later fails the predicate (edited to add a `patient_box`, or `bloom` changed to `apply`) WHEN the pool re-validation runs THEN it is quarantined out of the eligible pool, not silently drilled. *(qcraft quarantine; engage DRILL_SCOPE_FIDELITY)* — PASS/FAIL.
- **A7.12 (pagination past 500)** GIVEN >500 skill_drill cards WHEN pool integrity re-validates THEN all rows are paged and each re-checked, not the first 500. *(APP_RECON §6.3)* — PASS/FAIL.
- **A7.13 (no learning-model write)** GIVEN the fence runs (block or allow) WHEN inspected THEN it writes only `Card.kind` on ALLOW and touches no `Revlog`/`CardState`/`LearnerState`/`ItemElo`/`DailyPlan`/`CalibrationBin` (grep-verifiable). *(INV-9 one-way arrow)* — PASS/FAIL.

### (f) UI STATES
- **empty / no drill content yet:** authoring form with no card selected → neutral "select or author an item to drill"; the eligible-pool count reads 0 → "no drill items yet" (honest, not a fabricated pool).
- **loading:** validating options/concept → disabled submit with a spinner; never optimistically enable.
- **populated / ALLOW:** all predicate clauses green → submit enabled, "Eligible for timed drill (remember/understand · single-best MCQ)".
- **edge / BAR:** submit disabled with the **specific** failed clause named inline (e.g. "Blocked: apply-level items are not drill-eligible"), one reason per failed clause (uxguide honest-translation, no vague "invalid").
- **rest-day:** N/A (authoring is not day-scoped).
- **error:** option/concept query failure → "cannot validate eligibility — not saved as drill"; **fail closed** (never allow skill_drill on an unvalidated item).

### (g) SPECIALIST + INVARIANT per line
A7.1 qcraft one-best-answer + engage DRILL_CONTENT_ELIGIBILITY · A7.2 qcraft one-best + §8e#1 · A7.3 qcraft Bloom-bound + engage (recognition-over-understanding) + §8e#1 · A7.4 qcraft negative-lead-in ban + §8d · A7.5 qcraft cover-the-options ship-gate + engage TIMED_DRILL_MODE · A7.6 qcraft typology + engage PROHIBITED + §1 · A7.7 qcraft current-vs-legacy + §8d/§8e#1 · A7.8/A7.9 engage+qcraft ELIGIBLE routing · A7.10 §8c PREVENT-not-advise · A7.11 qcraft quarantine + engage DRILL_SCOPE_FIDELITY · A7.12 APP_RECON §6.3 · A7.13 INV-9.

### (h) OWNER GATES / [UNKNOWN] pre-build gates
- **[UNKNOWN-7a] TOEFL wrong-answer penalty** — qcraft holds this UNKNOWN (treated no-penalty only by analogy to the INBDE FACT). Load-bearing for a TOEFL drill: if a wrong/timed answer carries a real exam penalty, the "timeout≠wrong, no shame" framing (Item 8) may mis-train. **GATE:** confirm no-penalty (or restrict the drill to INBDE only for first ship) before any TOEFL card is drilled. [UNKNOWN — GAMIFICATION_PLAN §5, §8e "still-open gates"; qcraft no-guessing-penalty glossary.]
- **[UNKNOWN-7b] the current-2026 TOEFL small-task allowlist contents** — which task types (Complete-the-Words, possibly Listen-and-Choose) are in-scope small tasks is a qcraft/owner curation; there is no schema flag to read (see shared facts). **GATE:** owner/qcraft confirm the allowlist before the TOEFL branch ships; recommended [ESTIMATE] first ship = **INBDE terminology gates only**, TOEFL held until 7a+7b resolve.
- **OWNER GATE-7c** confirm the homogeneity/cover-the-options call is qcraft's ship-gate (the mechanical proxy is a filter, the qcraft flag is the authority) — not a hand-rolled string check that would pass a broken item.

---

## ITEM 8 — Drill Surface (ephemeral WaterMeter)

### (a) GOAL
An opt-in timed fluency surface over the eligible skill_drill pool whose reward meter rises on genuine recognition-fluency, is unmistakable from real mastery, is fully accessible, and writes **nothing** to the learning model — a structural firewall, not a grep-hope.

### (b) WHAT TO BUILD
A separate opt-in drill screen (NOT `Review.jsx`) rendering one eligible MCQ at a time with an **ephemeral in-memory `WaterMeter`**:

`WaterMeter { level: 0..100 (rendered STEPPED, not smooth), rise_per_hit, decay_per_sec, window_ms }` — a client/session object, **non-persisted** [FACT — GAMIFICATION_PLAN §1; engage WATERMETER_MODEL].

- **Dynamic per-card window** `window_ms = ESL_floor_ms + option_count · read_rate_ms_per_option` — never a fixed cinematic constant [FACT — GAMIFICATION_PLAN §1, §8b#3; qcraft ESL fall-floor; graphux ESL-readable-fall-floor]. `option_count` from the card's `ItemOption` rows.
- **Rise (genuine correctness only):** on `correct==true` AND selection within `window_ms` → `level = min(100, level + rise_per_hit)`; `rise_per_hit` is **deterministic/fixed** (no variable-ratio reward). Labelled **"speed, not mastery"** [FACT — §1; engage WATER_RISE_RULE / VARIABLE_REWARD_REJECT].
- **Decay (session-only):** `level = max(0, level − decay_per_sec · dt)`, framed as session momentum ("use it or lose it *this session*"), NOT forgetting. **Multi-day decay REJECTED** (spacing violation) [FACT — §1, §8e#3; engage SESSION_DECAY_RULE].
- **Selection target = STATIC option buttons only** (keys 1–4 / tap). The falling drop is **never** a click target (a moving target = motor-dexterity test, construct-invalid) [FACT — §1, §8e#2; graphux static-target-selection; engage TIMED_DRILL_MODE].
- **Timeout ≠ wrong:** a missed drop has no `chosen_option_id`, is not an FSRS "Again", carries no misconception, writes nothing. Copy: **"Correct — but the drop fell first. Speed, not knowledge,"** + offer to **re-see the item untimed** + **no shame loop** [FACT — §1, §8e#2; engage TIMEOUT_NOT_WRONG; uxguide no-shame; INV-8].
- **Accessibility (hard):** `prefers-reduced-motion` → **static countdown numeral/ring + discrete step meter preserving 100% of the timer information** (drop physics decorative/removable, timer info essential/non-removable). Colorblind → level reads by **height + numeral**, never cyan water alone. Encoding = viz **blue/cyan vertical tank, stepped, tickless** — outside the mastery green/yellow/red palette, the completion tick reserved for real day-box completion [FACT — §1, §8e#2; graphux reduced-motion-removability; viz HONEST_DRILL_ENCODING].
- **Zero backend calls (structural fence):** the drill component calls **no** backend function — not `submitReview`, not `gradeTextAnswer` — and issues **no** `.create/.update/.upsert` to `Revlog`/`CardState`/`LearnerState`/`ItemElo`/`DailyPlan`/`CalibrationBin`. The firewall is structural (the component never imports a learning-model writer), verified not only by grep [FACT — §3 INV-1/2/9; §8c should-fix "calls NO backend function … structural fence, not only a grep"; graphux one-way-arrow].

### (c) ENTITIES / FIELDS
- **Reads** the eligible pool: `Card` where `kind=="skill_drill"` [FACT — `Card.jsonc:8-16`] + its `ItemOption` rows (`text`, `is_correct`, `option_count`) [FACT — `ItemOption.jsonc:8-14`]. Renders `Card.prompt` [FACT — `Card.jsonc:23-25`].
- **`WaterMeter` is NOT an entity** — it is in-memory session state (client only), with fields `{level, rise_per_hit, decay_per_sec, window_ms}` [FACT — GAMIFICATION_PLAN §1 "ephemeral … writes NOTHING"; engage WATERMETER_MODEL]. No schema row, no persistence, resets on reload/session end.
- **Writes: NONE.** No new field. `game_timed` on `Revlog.grade_source` is **NOT added** in Phase 3 (deferred, §6/§8c) — Phase 3 logs no answers at all.

### (d) DATA CONTRACT
Pool read (paginate past 500, batch-join options — APP_RECON §6.3/§6.4):
```
pool     = paginate(Card.filter({kind:"skill_drill"}))              # ≤500 batches
poolIds  = [c.id for c in pool]
optsByCard = {}                                                     # batch-join, NOT per-card .get (N+1)
for chunk in chunks(poolIds, 500):
    for o in ItemOption.filter({card_id IN chunk}): optsByCard[o.card_id].append(o)
pool = [c in pool where c passes Item-7 ALLOW predicate]            # re-validate at load (defense in depth)
```
Per-card timing (the only "aggregation formula" — no cross-row aggregation, no learning-model read):
```
option_count = len(optsByCard[card.id])
window_ms    = ESL_floor_ms + option_count * read_rate_ms_per_option        # [UNKNOWN-8b], [UNKNOWN-8c]
```
WaterMeter mutation (pure client state, per animation frame / per answer):
```
on correct-within-window:  level = min(100, level + rise_per_hit)           # deterministic rise
each tick (dt seconds):    level = max(0, level - decay_per_sec * dt)       # session-only  [UNKNOWN-8d]
on timeout:                no write, no chosen_option_id, show re-see offer  # timeout != wrong
render:                    level -> floor to discrete steps (stepped, tickless, blue tank)
```
`rise_per_hit` deterministic [FACT — §1 VARIABLE_REWARD_REJECT]. `window_ms` formula [FACT — §1 structure] with its two constants [UNKNOWN] (see (h)). `decay_per_sec` [UNKNOWN] session-only (see (h)). Batch-join + pagination [FACT — APP_RECON §6.3/§6.4]. **No 500-cap risk of contamination:** even a truncated pool is a smaller drill, never a learning-model corruption — but pagination is still required so the pool doesn't silently shrink at 2000+ cards.

### (e) ACCEPTANCE CHECKS (each guardrail = its own PASS/FAIL)
- **A8.1 (ephemeral / non-persistence)** GIVEN a drill session raises the meter to 60 WHEN the page reloads or the session ends THEN `level` is gone (resets to 0) and no durable store holds it. *(engage WATERMETER_MODEL; §1 non-persistence)* — PASS/FAIL.
- **A8.2 (zero learning-model write — structural)** GIVEN any number of correct/wrong/timeout drill answers WHEN the session runs THEN there are **zero** writes to `Revlog`/`CardState`/`LearnerState`/`ItemElo`/`DailyPlan`/`CalibrationBin` AND **zero** calls to `submitReview`/`gradeTextAnswer` — the component imports no learning-model writer (structural), not merely "no writes observed". *(INV-1/2/9; §8c structural fence; graphux one-way-arrow)* — PASS/FAIL.
- **A8.3 (rise only on genuine correctness)** GIVEN a correct selection within `window_ms` THEN `level += rise_per_hit`; GIVEN a wrong selection, or motion/speed with no correct answer, THEN `level` does NOT rise. *(engage WATER_RISE_RULE; §1 "rises only on genuine correctness")* — PASS/FAIL.
- **A8.4 (deterministic reward, no variable ratio)** GIVEN N identical correct-in-window hits THEN each adds exactly `rise_per_hit` (no random/variable bonus). *(engage VARIABLE_REWARD_REJECT; INV-7)* — PASS/FAIL.
- **A8.5 ("speed, not mastery" label)** GIVEN the meter is shown WHEN inspected THEN it is labelled speed/recognition-fluency, never mastery/progress, and carries no completion tick. *(engage HONEST_DRILL_ENCODING; uxguide honest-translation; viz tickless)* — PASS/FAIL.
- **A8.6 (session-only decay)** GIVEN the meter at 50 WHEN the session ends and a new session starts the next day THEN it starts at 0 (no multi-day carry, no cross-day decay curve). *(engage SESSION_DECAY_RULE; §8e#3; INV-2/INV-4 — no pull to review early)* — PASS/FAIL.
- **A8.7 (static-target selection)** GIVEN the drop is falling WHEN the user clicks/taps the moving drop THEN NO answer registers; answers register ONLY on the static option buttons (keys 1–4 / tap). *(graphux static-target-selection; §8e#2; construct validity)* — PASS/FAIL.
- **A8.8 (timeout ≠ wrong copy + re-see + no shame)** GIVEN the window elapses with no selection THEN the UI shows "Correct — but the drop fell first. Speed, not knowledge," offers an **untimed re-see**, shows **no** shame/streak-break/lives-lost animation, and records no `chosen_option_id`. *(engage TIMEOUT_NOT_WRONG; uxguide no-shame; §8e#2; INV-8)* — PASS/FAIL.
- **A8.9 (timeout is not an FSRS "Again")** GIVEN a timeout THEN nothing is written to `CardState`/`Revlog` and the card's FSRS schedule is unchanged. *(INV-8; engage TIMEOUT_NOT_WRONG)* — PASS/FAIL.
- **A8.10 (prefers-reduced-motion preserves 100% timer info)** GIVEN `prefers-reduced-motion: reduce` THEN the drop physics is removed AND a static countdown numeral/ring + discrete step meter conveys the full remaining-time + level information (nothing timer-related is lost). *(graphux reduced-motion-removability; WCAG 2.3.3/C39 [UNVERIFIED-SOURCE]; §8e#2)* — PASS/FAIL.
- **A8.11 (colorblind: height + numeral)** GIVEN a colorblind user THEN `level` is decodable by fill height AND a numeric label, never by cyan water hue alone. *(viz HONEST_DRILL_ENCODING / redundant-channel; §8e#2)* — PASS/FAIL.
- **A8.12 (blue/stepped/tickless, off mastery palette)** GIVEN the meter renders THEN it is a blue/cyan vertical tank in discrete steps, outside the mastery green/yellow/red palette, on the drill surface only (never Dashboard/Learner-Model), and never shows the completion tick. *(viz four-separator honest encoding; §1)* — PASS/FAIL.
- **A8.13 (dynamic per-card window ≥ ESL floor)** GIVEN a card with `option_count=4` THEN `window_ms = ESL_floor_ms + 4·read_rate` and is never below the ESL floor; GIVEN more options THEN a longer window. *(qcraft ESL fall-floor; graphux ESL-readable-floor; §1, §8b#3)* — PASS/FAIL.
- **A8.14 (underlying MCQ still ship-gate valid)** GIVEN a rendered drill item THEN its options are homogeneous / pass cover-the-options (guaranteed by Item 7; the surface renders only ALLOW-passing cards). *(qcraft; engage TIMED_DRILL_MODE)* — PASS/FAIL.
- **A8.15 (honest empty state)** GIVEN the eligible pool is empty (no skill_drill cards) THEN the surface shows "no drill items yet", never a fabricated card or a filled meter. *(viz honest-abstraction; uxguide empty-state onboarding)* — PASS/FAIL.
- **A8.16 (onboarding pre-explains timeout)** GIVEN a first-run drill THEN onboarding pre-explains the timeout≠wrong framing before the first timed item. *(uxguide first-run onboarding; §1 "first-run onboarding pre-explains the timeout framing")* — PASS/FAIL.
- **A8.17 (pagination + batch-join)** GIVEN >500 skill_drill cards THEN the pool pages past 500 and options are batch-joined (≤500 per query), never per-card N+1. *(APP_RECON §6.3/§6.4)* — PASS/FAIL.
- **A8.18 (no honest-collection coupling)** GIVEN drill hits accrue THEN no KG concept lights up / no collection advances from drill activity — collection maps only to real BKT mastery (Phase 2 Item 5), never to the WaterMeter. *(engage HONEST_COLLECTION; §2.3; INV-5/6)* — PASS/FAIL.

### (f) UI STATES
- **empty / no drill content:** eligible pool = 0 → "no drill items yet" + link to author one (never a fabricated item or a pre-filled meter).
- **loading:** pool/options fetching → skeleton tank at level 0 (0 is the real start, not a placeholder lie).
- **populated / drilling:** card + static option buttons + falling drop (or reduced-motion static ring) + stepped blue tank; meter rises on genuine correct-in-window, decays this session.
- **edge / timeout:** "Correct — but the drop fell first. Speed, not knowledge." + untimed re-see button; no shame animation; meter unchanged (no rise, normal decay continues).
- **edge / reduced-motion:** no drop physics; static countdown numeral/ring + step meter carrying 100% timer info.
- **rest-day:** the drill is opt-in and NOT part of the streak/day-box denominator — a rest day neither requires nor is advanced by drilling (drilling on a rest day writes nothing to `DailyPlan`, INV-2/INV-4).
- **error:** pool query failure → "drill unavailable"; never fall back to an un-validated or non-drill card.

### (g) SPECIALIST + INVARIANT per line
A8.1 engage WATERMETER_MODEL/§1 · A8.2 INV-1/2/9 + §8c structural-fence + graphux one-way-arrow · A8.3 engage WATER_RISE_RULE · A8.4 engage VARIABLE_REWARD_REJECT + INV-7 · A8.5 engage HONEST_DRILL_ENCODING + uxguide + viz · A8.6 engage SESSION_DECAY_RULE + §8e#3 + INV-2/4 · A8.7 graphux static-target + §8e#2 · A8.8 engage TIMEOUT_NOT_WRONG + uxguide no-shame + INV-8 · A8.9 INV-8 · A8.10 graphux reduced-motion-removability + §8e#2 · A8.11 viz redundant-channel + §8e#2 · A8.12 viz four-separator encoding + §1 · A8.13 qcraft ESL-floor + graphux + §8b#3 · A8.14 qcraft ship-gate + engage TIMED_DRILL_MODE · A8.15 viz honest-abstraction + uxguide · A8.16 uxguide onboarding + §1 · A8.17 APP_RECON §6.3/6.4 · A8.18 engage HONEST_COLLECTION + INV-5/6.

### (h) OWNER GATES / [UNKNOWN] pre-build gates
- **[UNKNOWN-8a] graphux motion build-vs-adopt (measured verdict).** The app ships **no animation/physics library** [FACT — APP_RECON §5]. Whether to hand-roll (CSS transforms/`linear()` springs + `requestAnimationFrame`) or adopt a spring/motion library is a **measured verdict** from prototyping BOTH and benchmarking target-device frame time against the ~**16.7 ms** 60 fps budget (16.7 ms = 1000/60, [FACT — identity]; the compositor-property list and WCAG numbers are [UNVERIFIED-SOURCE], many publishers 403'd). **GATE:** do NOT assert a library or a frame figure — prototype, measure, then decide; the reduced-motion fallback (A8.10) must hold regardless of the verdict. [UNKNOWN — GAMIFICATION_PLAN §5, §8e "still-open gates"; graphux build-vs-adopt.]
- **[UNKNOWN-8b] the empirical ESL fall-floor constant (`ESL_floor_ms`).** The floor that lets a domain-lite ESL learner read stem + all options once must be **empirically calibrated to ESL reading rate before it ships as a real timer** — not a guessed cinematic value. **GATE:** owner/qcraft supply the measured floor. [UNKNOWN — §5, §8e; qcraft ESL fall-floor; graphux ESL-readable-floor.]
- **[UNKNOWN-8c] `read_rate_ms_per_option`** — the per-option reading increment in `window_ms`. Paired with 8b; a per-card ESTIMATE until measured. **GATE:** calibrate with 8b before the timer is live. [UNKNOWN — §1, §8b#3.]
- **[UNKNOWN-8d] the water `decay_per_sec` value.** Recommended **session-only** (multi-day rejected). The **exact constant** is owner-open (`water_decay_timescale`). **GATE:** owner states the value; two different values give very different session urgency. [UNKNOWN — §5 `water_decay_timescale`, §8e "still-open gates"; engage SESSION_DECAY_RULE.]
- **[UNKNOWN-8e] TOEFL wrong-answer penalty** (shared with 7a) — bears on whether the "timeout≠wrong / no-shame" framing is safe for TOEFL items. Recommended first ship = INBDE-only until resolved. [UNKNOWN — §5, §8e; qcraft.]
- **OWNER GATE-8f** confirm `game_answer_logging` stays **strictly ephemeral** for first ship (write nothing). The `game_timed` `evidence_class` writer + replay-filter is a gated FOLLOW-UP requiring the `Revlog.grade_source` schema addition + `bkt_evidence` re-verification BEFORE any writer ships — **out of Phase 3** [FACT — GAMIFICATION_PLAN §5, §8c, §6; INV-3/INV-10].

---

## Phase 3 cross-item honesty & guardrail summary
- **One-way arrow (INV-1/2/9):** Item 8 issues zero learning-model writes and imports no writer (structural, A8.2); Item 7 writes only `Card.kind` on ALLOW (A7.13). Grep-verify + import-graph-verify.
- **Mechanical fence, not advisory (§8c):** Item 7 blocks at the write/input layer; a barred item can never persist as skill_drill (A7.10).
- **BAR-IF correctly oriented (§8e#1):** bar on EXCEPT/NOT ∨ Bloom∉{remember,understand} ∨ non-homogeneous ∨ not single-best MCQ ∨ INBDE itemset/vignette/case ∨ legacy TOEFL; allow only the two eligible routes (A7.1–A7.9). This is the corrected (un-inverted) predicate.
- **Timeout ≠ wrong (INV-8):** no `chosen_option_id`, not an FSRS "Again", no shame, untimed re-see (A8.8/A8.9).
- **Session-only, ephemeral, deterministic (§1/§8e#3):** WaterMeter non-persisted (A8.1), decay session-only (A8.6), rise deterministic (A8.4).
- **Static target + accessible + honest encoding (§8e#2):** buttons not drop (A8.7), reduced-motion keeps 100% timer info (A8.10), colorblind height+numeral (A8.11), blue/stepped/tickless off-palette (A8.12).
- **ESL fairness (§8b#3):** dynamic per-card window ≥ ESL floor (A8.13) — never a fixed short timer testing English reading rate.
- **Every numeric/external claim tagged:** `Card.kind`/`answer_type`/`bloom`/`patient_box`/`standalone` defaults [FACT — entity schemas]; BAR-IF predicate [FACT — §8e#1]; TOEFL allowlist contents [UNKNOWN/ESTIMATE]; `ESL_floor_ms`, `read_rate_ms_per_option`, `decay_per_sec` [UNKNOWN]; ~16.7 ms = 1000/60 [FACT — identity]; compositor list / WCAG 2.3.3 numbers [UNVERIFIED-SOURCE]; TOEFL penalty [UNKNOWN]; `game_timed` not in schema [FACT — `Revlog.jsonc:59-68`].
- **Four [UNKNOWN] pre-build gates open (§8e):** graphux motion build-vs-adopt (8a), ESL fall-floor constant (8b/8c), water decay_per_sec (8d), TOEFL wrong-answer penalty (7a/8e). None may be asserted; all must be resolved/measured before RUN. Recommended first ship: **INBDE terminology gates only**, water-drop behind the graphux prototype.

---

# CONSOLIDATED ACCEPTANCE-CHECKS RECAP

_Every line below is its own PASS/FAIL guardrail. An un-asserted line is unverified — the build is not "done" until each is exercised. Full GIVEN/WHEN/THEN wording and per-line specialist+invariant mapping live in each item's §(e)/§(g) above._

**Phase 1 — Safe Menu**
- Item 1 Day-Box Effort Fill: AC1.1 floor invariant · AC1.2 tick ⇔ complete · AC1.3 divide-by-zero guard · AC1.4 rest-day distinct state · AC1.5 effort-not-mastery label · AC1.6 single source · AC1.7 no false tick on rest day.
- Item 2 Streak + Metrics: AC2.1 rest day = active, no token · AC2.2 token spent only on missed due-work day · AC2.3 miss never bare-resets · AC2.4 active-by-work · AC2.5 secondary metric always visible · AC2.6 secondary metric miss-resilient · AC2.7 rest days don't drain budget over time · AC2.8 honest label.
- Item 3 Future-Load Forecast: AC3.1 scheduler-sourced, not DailyPlan · AC3.2 never tick / never completion-green · AC3.3 labeled forecast · AC3.4 pagination past 500 · AC3.5 zero-fill honest · AC3.6 no write · AC3.7 staleness honest.

**Phase 2 — Mastery, Collection, Maturation**
- Item 4 Mastery Meter: A4.1 aggregate correctness · A4.2 On-track threshold (0.60) · A4.3 effort/mastery separation · A4.4 live source, not snapshot · A4.5 honest empty state · A4.6 pagination past 500 · A4.7 read-only.
- Item 5 Concept Collection / Light-up: A5.1 tier cutoffs · A5.2 mastered = 0.95 not 0.60 · A5.3 R1 demotion cross-source · A5.4 never revoke · A5.5 non-punitive · A5.6 honest collection not activity · A5.7 batch-join no mis-light at scale · A5.8 live source · A5.9 honest empty state · A5.10 read-only.
- Item 6 Concept Maturation: A6.1 min over review cards · A6.2 tier cutoffs · A6.3 missing S ⇒ seedling · A6.4 non-review card caps at seedling · A6.5 key on S not interval · A6.6 dominant over light-up (Lit-but-seedling) · A6.7 batch-join at scale · A6.8 sched-owned not BKT key · A6.9 honest empty state · A6.10 read-only.

**Phase 3 — Water-Drop Drill (ship LAST)**
- Item 7 Drill Authoring Fence: A7.1 bar non-MCQ · A7.2 bar not-single-best · A7.3 bar Bloom · A7.4 bar EXCEPT/NOT stem · A7.5 bar non-homogeneous · A7.6 bar INBDE itemset/vignette/case · A7.7 bar legacy TOEFL · A7.8 allow eligible INBDE terminology gate · A7.9 allow eligible 2026 TOEFL small task · A7.10 mechanical, not advisory · A7.11 pool re-validation · A7.12 pagination past 500 · A7.13 no learning-model write.
- Item 8 Drill Surface (WaterMeter): A8.1 ephemeral/non-persistence · A8.2 zero learning-model write (structural) · A8.3 rise only on genuine correctness · A8.4 deterministic reward · A8.5 "speed, not mastery" label · A8.6 session-only decay · A8.7 static-target selection · A8.8 timeout ≠ wrong copy + re-see + no shame · A8.9 timeout is not an FSRS "Again" · A8.10 reduced-motion preserves 100% timer info · A8.11 colorblind height + numeral · A8.12 blue/stepped/tickless off mastery palette · A8.13 dynamic per-card window ≥ ESL floor · A8.14 underlying MCQ still ship-gate valid · A8.15 honest empty state · A8.16 onboarding pre-explains timeout · A8.17 pagination + batch-join · A8.18 no honest-collection coupling.

**Cross-slice invariants asserted by the recap:** INV-2 (no game activity advances `CardState.due`), INV-4 (rest day never punished), INV-5 (effort never masquerades as mastery), INV-7 (informational feedback, no reward economy), INV-8 (timeout is never an FSRS "Again"), INV-9 (one-way read, never write). EFFORT (day-box) and MASTERY (meter/collection) and RECOGNITION-FLUENCY (WaterMeter) are three separate signals, never merged.

---

# OWNER GATES + [UNKNOWN] PRE-BUILD GATES

_These must be resolved (owner call or measurement) before the corresponding item RUNs. Recommended defaults are tagged; none may be silently guessed into the build. No phantom entity/field is ever added silently (§8c phantom-field guard)._

**Phase 1**
- **G1-a [rest-day encoding choice]** — derived flag (`planned==0`, no schema change, recommended) vs a persisted `daybox_state` sentinel (schema addition). [UNKNOWN — owner/impl].
- **G1-b [past incomplete-day policy]** — freeze at final `%` vs gray-out (engage `PAST_DAY_STATE_POLICY`). [UNKNOWN]. Not blocking the fill, blocking "past day" visuals.
- **G1-c [weighting]** — unweighted task count (literal reading) vs effort-weighting. Phase 1 = unweighted. [UNKNOWN behind the denominator].
- **Known risk (flag, not a gate):** UTC/local day-key mismatch (`date` keys UTC vs local calendar, APP_RECON §6.5) [FACT] — pre-existing; Item 1 must not assume local==UTC.
- **G2-a [token persistence]** — ephemeral recomputed streak (recommended, no new entity) vs persisted token ledger (new surface; §6.6 write-on-read race). [UNKNOWN — owner/impl].
- **G2-b [missing-row ambiguity]** — score a past day with no DailyPlan row as rest vs missed. Recommended: rest only if no CardState was due that day. [UNKNOWN — owner call].
- **G2-c [threshold source]** — confirm `day_done_threshold` = 0.8 is the day-completion bar, not `desired_retention` 0.9. [ESTIMATE — schema default].
- **G3-a [forecast inclusion rule]** — count future-due `state != 'new'` (recommended) vs all; decide learning/relearning inclusion. [UNKNOWN — sched/owner].
- **G3-b [horizon N]** — default 7 [ESTIMATE], owner-tunable; must not exceed honest paginated coverage.
- **G3-c [UTC/local day bucketing]** — `due.slice(0,10)` buckets UTC vs local calendar (APP_RECON §6.5) [FACT]; recommended bucket in user-local day. [UNKNOWN — timezone policy].
- **G3-d [distinct encoding token]** — exact cool/neutral hue + mark are a **viz** deliverable (CVD-safe, WCAG dark-theme); Phase 1 fixes only the semantic (not-green, not-tick, labeled), not the hex. [UNKNOWN — viz styling, non-blocking to data contract].

**Phase 2**
- **[RESOLVED-4a — owner-locked 2026-07-11: option (i) ONLY-ASSESSED concepts (those with a LearnerState row), with an explicit "(n of N assessed)" coverage suffix so the un-assessed remainder is never hidden] Denominator definition** — area-mean over (i) only concepts with a LearnerState row vs (ii) ALL concepts (never-touched at 0.3 prior). **GATE before build.** Recommended (i) with "(n of N assessed)" coverage suffix. [UNKNOWN — owner].
- **OWNER GATE-4b** — confirm "On track" line stays at **0.60**, never relabelled "mastered."
- **OWNER GATE-5a** — light-up threshold SET at **0.95**, owner-tunable only within [0.90, 0.95], NEVER down to 0.60. Confirm on relay.
- **Item 5 cross-source dependency (declare):** the R1 demotion label needs Item 6's `min(stability over review-state cards) < 21d`. Item 6 must ship (or its aggregate be available); if unavailable, default to the conservative "Learned/seen" label, never a permanent "Learned" trophy.
- **OWNER GATE-6a** — maturation cutoffs SET at seedling <7 / growing 7–21 / mature ≥21 (+ optional evergreen ≥100); 7/100 edges [ESTIMATE], 21 = Anki mature [FACT]; all owner-tunable. Confirm on relay.
- **Item 6 ownership note:** sched deliverable (min-of-FSRS-stability); if Phase 2 capacity is learner/engage-only, flag Item 6 as needing the sched read path before RUN.

**Phase 3 (all four must be resolved/measured before RUN — recommended first ship: INBDE-only, water-drop behind the graphux prototype)**
- **[UNKNOWN-7a / 8e] TOEFL wrong-answer penalty** — confirm no-penalty (or restrict drill to INBDE for first ship) before any TOEFL card is drilled; bears on the "timeout≠wrong / no-shame" framing. [UNKNOWN — qcraft/owner].
- **[UNKNOWN-7b] current-2026 TOEFL small-task allowlist contents** — no schema flag to read; a build-time constant curated by qcraft/owner. Confirm before the TOEFL branch ships. [UNKNOWN].
- **OWNER GATE-7c** — confirm the homogeneity/cover-the-options authority is qcraft's ship-gate, not a hand-rolled string check.
- **[UNKNOWN-8a] graphux motion build-vs-adopt** — measured verdict from prototyping BOTH against the ~16.7 ms (1000/60) 60 fps budget [FACT — identity]; compositor list/WCAG numbers [UNVERIFIED-SOURCE]. Do NOT assert a library or a frame figure; reduced-motion fallback holds regardless. [UNKNOWN].
- **[UNKNOWN-8b] `ESL_floor_ms`** — empirically calibrated to ESL reading rate before it ships as a real timer. [UNKNOWN — owner/qcraft].
- **[UNKNOWN-8c] `read_rate_ms_per_option`** — per-option reading increment; paired with 8b. [UNKNOWN].
- **[UNKNOWN-8d] `decay_per_sec` (`water_decay_timescale`)** — session-only recommended (multi-day rejected); exact constant owner-open. [UNKNOWN — owner].
- **OWNER GATE-8f** — confirm `game_answer_logging` stays strictly ephemeral (write nothing) for first ship. The `game_timed` `evidence_class` writer + replay-filter is a gated FOLLOW-UP requiring the `Revlog.grade_source` schema addition + `bkt_evidence` re-verification BEFORE any writer ships — out of Phase 3 (INV-3/INV-10).

---

```
Base44 agent directive
> Repo-driven mode: obey docs/plan/PROTOCOL.md. Every specialist path below resolves to docs/plan/specialists/<code>.specialist.json. Build the SAFE MENU FIRST, then MASTERY/MATURATION, then the water-drop DRILL LAST — behind all guardrails and the graphux motion prototype. The arrow is ONE-WAY (GAMIFICATION_PLAN §3): gamification READS the learner/scheduler model and NEVER writes it; treat every OWNER GATE and [UNKNOWN] pre-build gate as stop-and-ask (never guess a threshold, constant, hue, allowlist, or entity/field into the build); the union of the item validation_checklists (AC1.*/AC2.*/AC3.*/A4.*/A5.*/A6.*/A7.*/A8.*) is this slice's acceptance test — an un-asserted line is unverified.
>
> PHASE 1 — Safe Menu (Items 1–3). Load docs/plan/specialists/engage.specialist.json as the PRIMARY design authority (day-box FILL_RENDER_MODEL/FLOOR_ROUNDING_RULE/TERMINAL_TICK_STATE/DAY_BOX_DATA_MODEL/EFFORT_VS_MASTERY_SEPARATION, STREAK_MECHANIC/STREAK_FREEZE_REPAIR/STREAK_ANXIETY_GUARDRAIL/SECONDARY_HORIZON_METRIC/OVERJUSTIFICATION_GUARDRAIL, FUTURE_LOAD_FORECAST/ANKI_HEATMAP_PRECEDENT). Load docs/plan/specialists/uxguide.specialist.json (honest-translation: effort≠mastery labels, staleness disclosure, first-run/empty-state). Load docs/plan/specialists/viz.specialist.json (honest encoding: fill=length, forecast separable-channel + never-tick, rest-day ≠ done). Load docs/plan/specialists/sched.specialist.json (CardState.due is the scheduler's state; the forecast is a BUILD over it, not a DailyPlan read — §8c B2 — and must page past the 500-row cap). Load docs/plan/specialists/learner.specialist.json (rest day = spacing-correct FREE win, non-punitive INV-4; the B1 streak fix).
>
> PHASE 2 — Mastery / Collection / Maturation (Items 4–6). Load docs/plan/specialists/engage.specialist.json (COURSE_LEVEL_PROGRESS, EFFORT_VS_MASTERY_SEPARATION, HONEST_COLLECTION, NON_PUNITIVE_SCHEDULER_CONTRACT). Load docs/plan/specialists/learner.specialist.json (BKT bkt_p_mastery semantics, no-forget/monotonic, tier thresholds — 0.60 on-track, 0.95 learned). Load docs/plan/specialists/sched.specialist.json (FSRS CardState.stability, review-state filter, interval-inversion 21d; Item 6 is a SCHED deliverable — min-of-stability, not a BKT key — §8c B3). Load docs/plan/specialists/viz.specialist.json (honest quantized encoding, honest empty-state "not yet studied", redundant channels). All three items are pure reads of LIVE LearnerState/CardState (never the MasteryHistory snapshot, §8d), paginate past 500 AND batch-join Concept→Card→CardState (no N+1), grep-verify zero writes (A4.7/A5.10/A6.10).
>
> PHASE 3 — Water-Drop Drill (Items 7–8), SHIP LAST, INBDE-only recommended for first ship. Load docs/plan/specialists/engage.specialist.json AND its third KB docs/plan/specialists/kb_session_drill_mechanics (WaterMeter model, four gates, DRILL_CONTENT_ELIGIBILITY, TIMED_DRILL_MODE, WATER_RISE_RULE, VARIABLE_REWARD_REJECT, SESSION_DECAY_RULE, TIMEOUT_NOT_WRONG, HONEST_DRILL_ENCODING, DRILL_SCOPE_FIDELITY). Load docs/plan/specialists/qcraft.specialist.json (the BAR-IF authoring fence: one-best-answer, homogeneity/cover-the-options ship-gate, EXCEPT/NOT ban, INBDE itemset/case typology, TOEFL current-2026-vs-legacy gating, ESL per-item time budget). Load docs/plan/specialists/graphux.specialist.json (game-feel motion, static-target selection, reduced-motion removability preserving 100% timer info, build-vs-adopt MEASURED verdict against the ~16.7 ms budget). Load docs/plan/specialists/uxguide.specialist.json (no-shame framing, timeout onboarding pre-explain, honest-translation). Load docs/plan/specialists/viz.specialist.json (blue/cyan stepped tickless tank off the mastery palette, height+numeral colorblind encoding). Item 7 is a MECHANICAL write/input-layer fence (a barred item can never persist as skill_drill), writing only Card.kind on ALLOW; Item 8 writes NOTHING to the learning model and imports no learning-model writer (structural firewall, not only a grep) — a timeout is never an FSRS "Again". Ship order across the whole slice: safe menu -> mastery/maturation -> drill last.
```
