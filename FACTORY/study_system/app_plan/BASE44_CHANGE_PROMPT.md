# BASE44_CHANGE_PROMPT.md — clean, paste-ready prompts for the Base44 builder

**What this document is.** You were dictating change requests to the Base44 builder and asked us to rewrite that dictation better where we understood it. Below is the cleaned version, split into **five sequential prompts** — paste Prompt 1 into the Base44 builder chat, check the result in the app, then paste Prompt 2, and so on. Small sequential prompts are safer than one giant one: if something comes out wrong, you know exactly which prompt caused it.

**Scope rule.** These prompts cover ONLY near-term changes that need **no new external infrastructure** — no Anki, no Google, no WhatsApp, no MCP, no Ollama keys, no 3D. Where those appear here at all, they are visual placeholders (disabled panels), so the Settings page is ready for them later.

**Honesty notes.**
- [FACT] Claims about what the app does today were verified by reading the exported app code (see APP_RECON.md); the prompts state them so the builder targets the right components.
- [kept as dictated — ambiguous] marks the spots where your original wording had more than one reasonable reading. We kept your words and added a default interpretation the builder should use unless you say otherwise.
- These prompts intentionally do NOT ask the builder to invent statistics or fake data anywhere; every number shown to you must come from real entity data or be labeled as an estimate in the UI.

---

## Prompt 1 — Tooltip ladder + Guide page

```
I want the app to explain itself. Today it has no tooltips and no help/guide layer
at all. Add an "explanation ladder" with four depths, applied consistently:

1. TINY LABEL: under or beside each stat/term, a one-line plain-language caption
   (muted small text).
2. HOVER CARD: hovering the label (or tapping an info dot on touch) shows a card
   with 2-3 plain sentences explaining what the number means and where it comes from.
3. "MORE" ICON: each hover card has a small "learn more" link/icon.
4. GUIDE PAGE: the "learn more" link opens a new page at /guide — a single guide
   page with sections and a glossary — scrolled to the matching section.

Apply the ladder to at least these places, with these meanings (these reflect how
the app actually computes them — do not change the computations, only explain them):

- Dashboard "Due Reviews": cards whose next review time has arrived.
- Dashboard "New Cards": today's allowance of never-studied cards (capped by the
  "new per day" setting) — NOT the size of my deck.
- Dashboard "Today": percent of today's planned study units completed.
- Dashboard "Streak": consecutive days I met my daily goal; slack tokens can cover
  a missed day.
- Planner "Budget share": this exam's slice of my daily study minutes, split in
  proportion to each exam's total card count.
- Planner "Total cards": the number I entered as the cards to learn for that exam.
- Planner "New / day" and "Last day to learn": the planner's recommended new-cards
  pace, and exam date minus a 14-day buffer.
- Planner feasibility "go / tight / no_go": whether the plan fits my daily minutes.
- Learner Model "Open Weaknesses (E1-E4 diagnosed)": E1 slip = careless miss on a
  strong concept; E2 misconception = repeatedly choosing the same wrong idea;
  E3 missing prerequisite = a foundation concept is weak; E4 retrieval decay =
  I knew it but it faded. The "confidence NN%" badge is a fixed rating built into
  whichever diagnosis rule fired — it is NOT a computed probability; the tooltip
  must say this honestly. NOTE: the E1–E4 letter order above is our assumed
  enum-order mapping (the code never binds the letters explicitly) — confirm or
  adjust the labels against the Weakness `cause` enum before shipping.
- Learner Model "BKT mastery": the model's estimate (0-100%) that I've internalized
  a concept, updated after every answer.
- Learner Model "Elo": a difficulty-vs-ability rating — my level per concept vs
  each card's difficulty.
- Review "AI suggests rating N": the AI grader's suggestion; my own button click is
  what counts.
- Settings "Desired retention": the single workload dial — how reliably I want to
  remember; higher = more daily reviews. Add to its hover card: values above 0.90
  increase workload steeply.

Guide page + glossary must also define: FSRS (the scheduling algorithm), spaced
repetition, BKT, Elo, retrievability, and the INBDE blueprint codes: FK1-FK10 are
the ten "Foundation Knowledge" areas (e.g. FK6 = general and disease-specific
pathology) and the CC sections DTP = Diagnosis & Treatment Planning,
OHM = Oral Health Management, PP = Practice & Profession. Where the app already has
these display names in code, reuse them; do not invent new FK/CC names.

Keep the existing dark theme; tooltips must work on desktop hover and mobile tap.
Do not change any calculation, entity, or backend function in this prompt — this is
a pure explanation layer.
```

---

## Prompt 2 — Dashboard calendar: click a day, see that day

```
On the Dashboard calendar (the white day-boxes that fill green), the boxes are
currently not clickable. Make every day box clickable. Clicking opens a day-detail
view (a side panel or modal, matching the app's dark style) — make it a nice
visual, not a raw list [kept as dictated — ambiguous: "nice visual" — default
interpretation: a clean card layout with a progress ring for that day, topic chips,
and a simple timeline strip for time windows; no decorative charts without data].

What the day-detail view shows, by day type:
- PAST DAY: what actually happened — reviews done vs planned that day, and which
  topics (concept names) were reviewed. Use the real review-log and daily-plan data
  for that date; if there is no data, say "no study recorded" rather than showing
  zeros that look like data.
- TODAY: today's plan — planned vs done units, the topics due today (concept names
  of due cards, grouped, with counts), and today's remaining time windows.
- TOMORROW: the planned topics and time windows from the planner/pipeline
  (see Prompt 3).
- LATER FUTURE DAYS: only a coarse forecast — expected number of due cards and
  estimated minutes. State in the panel: "Detailed planning happens one day ahead."

"Time windows": derive them by splitting my daily_minutes setting into 1-3 labeled
study blocks [kept as dictated — ambiguous: I said the day view should show the
day's "time windows", but the app stores no time-of-day today — default
interpretation: propose blocks like "Morning 25 min / Evening 35 min" from
daily_minutes, clearly labeled "suggested", until real scheduling exists].

Also fix, as part of this change, the timezone handling for day boundaries: the
app currently computes the day key in UTC but draws the calendar in local time, so
late-evening reviews can land on the wrong day. All day keys shown on the calendar
and in the day-detail view must use my local date consistently.

Do not change the day-box fill/tick semantics (white box, green fill in 5% steps,
tick only at true 100%) — those stay exactly as they are.
```

---

## Prompt 3 — Next-day planning, gating with regression, driven by my answers

```
Change how planning works, in three connected rules:

1. NEXT-DAY-ONLY DETAILED PLANNING. Detailed plans exist only for tomorrow, built
   from my actual answers today and yesterday (the review log and the latest
   weakness diagnosis). The analysis pipeline already writes tomorrow's plan;
   extend it so tomorrow's plan also records the topic breakdown (which concepts,
   how many units each) that the calendar day view (Prompt 2) displays. Days after
   tomorrow never get detailed plans — only the coarse forecast. Plan one day at a
   time, not all of it together. Keep the existing "Run nightly analysis" button as
   the trigger for now.

2. GATING (Anki-style), WITH REGRESSION. No group advances while its base concept
   is weak [kept as dictated — ambiguous: "group" — default interpretation: the set
   of cards belonging to a concept, where "base concept" means a prerequisite
   concept linked to it; if no prerequisite link exists, the concept is its own
   base]. Concretely:
   - When assembling the review session, do NOT introduce never-studied (new) cards
     for a concept whose base concept's mastery is below 0.6.
   - Already-studied cards are NEVER blocked — reviews always continue; gating only
     stops NEW material. The daily new-card allowance must also actually be
     enforced in the review session (today the session ignores it).
   - REGRESSION ALLOWED: if a base concept's mastery falls back below the
     threshold, the gate closes again automatically for the dependent concept.
   - Where prerequisite links are missing (most concepts today have none), fall
     back to self-gating: a concept whose own mastery is below 0.6 gets no
     additional new cards until it recovers.
   - Make gating visible, never silent: in the day view and in the Library, show a
     small "gated" badge with a tooltip naming the weak base concept, and give me a
     per-concept override switch to open a gate manually.
   - Add a simple way for me to declare "concept A is the base of concept B" in the
     Library (this creates a prerequisite edge between the two concepts using the
     existing concept-edge data model).

3. THE PLAN FOLLOWS THE ANSWERS. Tomorrow's detailed plan must prioritize, in
   order: (a) due reviews, (b) probe/repair items for currently open weaknesses,
   (c) new cards only through open gates and within the daily new allowance. If the
   plan cannot fit in my daily minutes, shrink the new-cards portion first and say
   so in the day view ("plan trimmed to fit your time").

Do not add any external integration in this prompt. Do not change the FSRS
scheduling math for individual cards — gating decides only WHICH cards enter a
session, never WHEN a seen card is due.
```

---

## Prompt 4 — Weaknesses manage themselves (replaces "Mark Resolved")

```
Change the weakness list from manually managed to self-managing:

1. REMOVE the "Mark Resolved" button as the primary mechanism. Replace it with:
   - AUTO-RESOLVE: a weakness closes itself when the evidence shows recovery —
     the concept's mastery is back above threshold AND I have answered items on
     that concept correctly on at least 2 different days since the weakness was
     detected. When it closes, keep it visible for a while in a collapsed
     "Recently resolved" section with the reason ("auto-resolved: mastery
     recovered"), so I can see the system working.
   - DISMISS (snooze): a small secondary action that hides a weakness for 7 days
     without pretending it's fixed. A dismissed weakness that is re-detected
     returns with a "returned" badge.

2. DEDUPLICATE ACROSS DAYS: today, the same persistent weakness creates a new open
   row every time the analysis runs on a new day. Change this to one open row per
   concept-and-cause pair: if the same weakness is re-detected, update the existing
   row (refresh its evidence and detection date, increment a "days seen" counter)
   instead of creating a duplicate. Re-running today's analysis must not wipe or
   duplicate resolutions from earlier today.

3. SHOW THE LIFECYCLE: each weakness card shows: detected date, days seen, what
   would auto-resolve it ("needs: mastery above 60% + correct answers on 2 separate
   days"), and progress toward that. The E1-E4 tooltips from Prompt 1 apply here.

Thresholds (mastery 0.6, 2 distinct days, 7-day snooze) are starting defaults —
put them in one obvious place in code/settings so they are easy to tune later, and
label them in the UI as defaults, not truths.
```

---

## Prompt 5 — Settings additions (one real control + honest placeholders + export)

```
Reorganize the Settings page into clear sections and add the following. Anything
marked PLACEHOLDER must be visibly disabled with a "coming soon — not yet
connected" note; placeholders must not fake functionality.

1. SECTION "How much do you want to study?" (real, working):
   - Present the existing desired-retention slider as the review-frequency control,
     relabeled in plain language: "Review workload — how reliably do you want to
     remember?" with endpoint captions ("fewer reviews, forget more" ... "more
     reviews, forget less"), the current value shown as a percent, and a warning
     state above 0.90 ("workload rises steeply past this point").
   - Group the existing daily minutes and new-cards-per-day fields into this same
     section. Do not add any other scheduling knob (no raw interval multipliers).

2. SECTION "Integrations" (PLACEHOLDERS): disabled panels for
   - "Anki sync" (reuse the existing anki mode/endpoint fields, disabled),
   - "Google Calendar reminders",
   - "WhatsApp nudges".
   Each panel: one sentence on what it will do, and a disabled Connect button.

3. SECTION "AI models" (PLACEHOLDER): a model-picker panel showing two empty slots
   labeled "Active model 1" and "Active model 2", one masked "API key" input, and
   the note "Model selection is not yet connected — the app currently uses the
   built-in AI." All disabled.

4. SECTION "Your data — export" (real, working, no external services): buttons that
   download my data as files generated client-side:
   - "Export cards" — all decks, cards, and answer options as JSON;
   - "Export notes & concepts" — concepts and concept-edges as JSON;
   - "Export learning history" — review log, daily plans, mastery history as JSON;
   - "Export everything" — one JSON file bundling the above with an export date.
   Each export states how many records it contains. If any entity query would be
   truncated by a row limit, page through ALL records so exports are complete —
   an incomplete export is worse than none.

Add hover-card explanations (Prompt 1 style) to every new control.
```

---

## After pasting

- Check each prompt's result in the app before pasting the next; tell the builder to adjust rather than re-pasting a whole prompt.
- The three thresholds in Prompts 3–4 (0.6 mastery, 2 days, 7-day snooze) are design defaults for you to tune after a week of real use.

Larger integrations (Anki, Calendar, MCP, Ollama, 3D, KG) are phased in FEATURE_PLAN.md — do not paste those here yet.
