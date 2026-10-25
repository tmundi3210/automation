# Decisions and checklist

## Decisions

Ordered by what blocks the build soonest. Each default is a proposal; nothing is decided until you say yes, ok or go.

1. Leave Phase 0 and start app building (J-01) once the nightly digest has run 7 nights in a row with no manual fix? Default: yes, 7 clean nights.
2. How do your 4 hours a day split between the Big Brain loop and the apps, and app time between talk, build and feedback? Default: loop 3.75 h in Phase 0, 2.5 h in Phase 1, 1.5 h from Phase 2 on and never less; app time about 20% talk, 50% build, 30% feedback; daily reading and journaling sit outside the 4 h.
3. Build the Journal first (Phase 1, with the book-package spike) and the Reader after it (Phases 2-3)? Default: yes, as build order only, not rank: the Journal is the smallest build and tests the shared code first.
4. Join the paid Apple Developer Program (99 USD a year, as of 2026-10-25) before J-01? Default: yes; a free account means a reinstall every 7 days and no iCloud folder, time-sensitive alerts, FamilyControls or PCC.
5. Which iPhone runs the apps, and is it an iPhone 15 Pro or later? Default: the main iPhone on iOS 27; if it is older, Reader scoring falls back to the Mac.
6. The brief puts the Big Brain recorder on the main iPhone with a DJI mic and also on a spare iPhone streaming 24/7: which is it? Default: both, main iPhone for the loop test and spare iPhone for 24/7, so the Reader can use the main iPhone's mic; R-30 tests this.
7. One new repo (~/Projects/mundi-apps) and Xcode workspace with separate Journal and Reader apps sharing one local Swift package? Default: yes; J-01 creates it, and each app keeps its own 64-notification budget.
8. Night flow of gratitude (always on) plus best moment, lesson and mood, each of the last three switchable in settings? Default: yes, all four on.
9. Journal reminders at 10:00 pm nightly and 7:30 am in the morning, Pacific, with the journal day starting at 4:00 am? Default: yes, each editable in 15-minute steps.
10. What is the morning reminder for? Default: it opens last night's gratitude lines plus one "Today I will" line, and offers to fill in a blank night.
11. When you travel, do journal reminders, Reader slots and quiet hours follow the phone's time zone or stay on Pacific? Default: follow the phone's zone (10:00 pm stays 10:00 pm local), Pacific at home, with a setting that pins Pacific; same rule in both apps.
12. Should app data (journal entries, Reader progress, later tracking files and decision worksheets) feed Big Brain as optional files in its input folder? Default: yes; one file per day, a missing file means no data, no Big Brain code changes, and Big Brain reads them only after the Phase 0 gate.
13. Should each book cover the whole book first (brief and chapter summaries), then take each chapter deeper, with its depth fixed at import? Default: yes; the depth target is the full text.
14. Which LLM and model run each Reader job, including recall scoring, with no Claude API key? Default: book packages in interactive Claude Code sessions on the Mac (Sonnet summaries, Haiku idea units and quizzes); recall scoring, cause flag and chat on Apple's on-device model once it passes the check set; fallback Private Cloud Compute, then a local model on the Mac.
15. Build the Reader, or use RemNote Guided Learn plus Recite or Blurt instead? Default: build, since no app found combines the paragraph gate, voice recall every N hours and calls; trial Recite on the pilot book during Phase 1 and copy what works.
16. Gate on each topic (3-7 paragraphs) or on every paragraph? Default: each topic.
17. Which speech-to-text runs dictate-back and paper-book mode? Default: on-device SpeechAnalyzer; the Mac Recorder pipeline only as a Punjabi or Hindi fallback.
18. Does "two-step thinking" mean pass 1 finds the missing idea units and pass 2 proposes why (never learned, misunderstood, forgotten), shown only as a hint? Default: yes.
19. Spacing default: your 4-hour base gap growing by ease with a hand-off to FSRS days at 24 h, or the research's fixed 2 h, 4 h and 8 h steps, then days? Default: your 4-hour rule (two hour-scale reviews per topic instead of three); the research steps ship as a settings preset.
20. Does the gap grow after each successful retell? Default: yes, multiplied by ease as in Anki (start 2.5, floor 1.3).
21. When reviews pile up, how many go into one session before the new topic? Default: 5; the rest roll to the next slot.
22. Which guard keeps you from drifting off mid-topic? Default: a Guided Access tip plus a "left mid-topic" ping in Phase 2; an optional shield over other apps in Phase 3 if the paid account allows it.
23. Which calls should ClawCall place, and how often? Default: push and quiz on, boost off (facts only when on); at most 1 call a day, none in quiet hours.
24. Does paper-book mode assume you read aloud? Default: yes; start and end page photos with OCR cover silent reading.
25. In what order should the later parts follow the Reader extras? Default: Part 3 Mind, Part 2 Self-tracking, Part 4 Voice, Part 7 rest, Part 5 Body language, Part 6 Clothes, one at a time, each behind its gate.
26. Host the later parts as Reader packs (Parts 3 and 7, lessons of Parts 4-5), one new Practice app (drills of Parts 4-5), one Tracker app (Part 2) and Mac-only scripts (Part 6)? Default: yes.
27. Which perception story leads the biases pack? Default: blind men and the elephant, paired with the invisible gorilla experiment; runner-up Plato's cave.
28. Should the location log keep only visits and significant moves, or also a continuous precise trail? Default: visits and significant moves; the continuous trail only when switched on.
29. Build camera-based room tracking at all? Default: no; reconsider after Part 2 v1 has run 4 weeks, starting with a 1-day battery and heat test on a phone that is not the Recorder.
30. Which cloud service may receive your face photos for LoRA training? Default: fal, only after L-06 shows its trainer is outside the no-training carve-outs, with payload storage off; never Higgsfield.
31. Which meditation app did you mean? Default: none assumed; Part 3 ends with a generic meditation hand-off until you name it.

## Checklist

| id | requirement | status | evidence or reason |
|---|---|---|---|
| R1 | Part 1: PDF or book in, broken into learning levels, introduction first, then deeper | PASS | 01-reader.md, "PDF-to-levels pipeline" (level order) and "Data model" (Level); tasks R-02..R-08, R-10; decision 13 |
| R2 | Part 1: one paragraph per screen, moves on after reading, tracks how much is read | PASS | 01-reader.md, "No-skip gate and test-out", "Screens" S4, "Data model" (ReadEvent); tasks R-11, R-14 |
| R3 | Part 1: blocks skipping ahead and losing interest; what must be learned is fixed | PASS | 01-reader.md, "No-skip gate and test-out" and "iOS limits that block a piece" (focus guard); tasks R-14, R-27, R-36; decision 22 |
| R4 | Part 1: talk about the text and ask it for more | PASS | 01-reader.md, "Talk about the text"; task R-32 |
| R5 | Part 1: second mode records reading of a physical book and analyzes how much was read | PASS | 01-reader.md, "Physical-book mode"; tasks R-30, R-31; decisions 6, 24 |
| R6 | Part 1: first run gives a brief, asks what I know, builds a next step that adds new content and tests earlier material | PASS | 01-reader.md, "No-skip gate and test-out" (First run) and "Schedule and spacing" (a) session slots; tasks R-12, R-23 |
| R7 | Part 1: repeats every 4 hours by default, easy to change (for example to 2 hours) | PASS | 01-reader.md, "Schedule and spacing" (a) and "Screens" S11; tasks R-22, R-25, R-26 |
| R8 | Part 1: Anki-style rule at hour scale, gap growth labelled assumption, hand-off to days, difference in 3 lines, decision with default | PASS | 01-reader.md, "Schedule and spacing" (a)-(d); task R-22; decisions 19, 20 |
| R9 | Part 1: dictate-back: retell, hear what I forgot, retell again | PASS | 01-reader.md, "Dictate-back loop"; tasks R-16, R-21 |
| R10 | Part 1: gap analysis of the specific missing idea and its root cause; two-step reading labelled assumption, hint only, decision | PASS | 01-reader.md, "Recall scoring, gap and root cause"; tasks R-17, R-21; decision 18 |
| R11 | Part 1: calls during learning to push, boost or quiz | PASS | 01-reader.md, "Notifications and calls"; tasks R-29, R-33, R-34; decision 23. ClawCall's per-call script and transcript return are (to verify) in R-29 |
| R12 | Part 2: precise location sent by my own app, no Find My | PASS | 03-later-phases.md, "Part 2. Self-tracking" (Goal, Smallest useful version, iOS limits); decision 28 |
| R13 | Part 2: HealthKit steps, other activity, sleep, sleep and wake times | PASS | 03-later-phases.md, "Part 2. Self-tracking" (Smallest useful version, iOS limits) |
| R14 | Part 2: screen time | PASS | 03-later-phases.md, "Part 2. Self-tracking", iOS limits row (blocked; spike and fallbacks); task L-03 |
| R15 | Part 2: website history | PASS | 03-later-phases.md, "Part 2. Self-tracking", iOS limits row (blocked on iPhone; Mac-side and domain-list alternatives) |
| R16 | Part 2: camera or vision of how I work and spend time in a room | UNKNOWN | 03-later-phases.md, "Part 2. Self-tracking" states the block and a stand-phone alternative, but battery, heat and privacy cost are unmeasured and decision 29 defaults to not building it |
| R17 | Part 2: iOS limits stated honestly, nearest workable alternative for each blocked item | PASS | 03-later-phases.md, "Part 2. Self-tracking", iOS limits table; each row's alternative is marked (proposal) |
| R18 | Part 3: mental models | PASS | 03-later-phases.md, "Part 3. Mind", Mental models bullet |
| R19 | Part 3: biases with examples and stories, including a perception story chosen from candidates | PASS | 03-later-phases.md, "Part 3. Mind", Biases bullet (choice labelled assumption); decision 27 |
| R20 | Part 3: anger: how the frontal lobe and which tricks regain control | PASS | 03-later-phases.md, "Part 3. Mind", Anger bullet; it corrects the brainstem-and-limbic framing and orders tricks by evidence |
| R21 | Part 3: guided meditation, existing apps checked on Mobbin, candidates unverified, question in decisions | PASS | 03-later-phases.md, "Part 3. Mind", Guided meditation bullet (Mobbin links, all marked unverified); decision 31, last |
| R22 | Part 3: default mode and task-positive networks, effects of meditation and exercise | PASS | 03-later-phases.md, "Part 3. Mind", Brain networks bullet |
| R23 | Part 3: probability and statistics with examples and simulations | PASS | 03-later-phases.md, "Part 3. Mind", Probability bullet |
| R24 | Part 3: only real psychology experiments, simplified or disputed claims flagged | PASS | 03-later-phases.md, "Part 3. Mind", Psychology experiments bullet; task L-01 blocks unsourced claims and myth phrases |
| R25 | Part 3: same flow as the Reader (brief, ask, add more, deepen) | PASS | 03-later-phases.md, "Part 3. Mind", Goal and Reuse |
| R26 | Part 4: warm-ups: humming different tones, counting 1-2-3 | PASS | 03-later-phases.md, "Part 4. Voice and speech", Warm-up bullet (counting reading labelled assumption) |
| R27 | Part 4: pen-between-the-teeth drill with words for different vowel movements | PASS | 03-later-phases.md, "Part 4. Voice and speech", pen drill bullet (shown as unproven) |
| R28 | Part 4: a section for a US (California) accent | PASS | 03-later-phases.md, "Part 4. Voice and speech", accent bullet: a hand-off to BoldVoice or ELSA with logged minutes, no own engine |
| R29 | Part 4: grammar and sentence building, spoken and written | PASS | 03-later-phases.md, "Part 4. Voice and speech", grammar bullet |
| R30 | Part 4: common new English words | PASS | 03-later-phases.md, "Part 4. Voice and speech", new words bullet (reading labelled assumption) |
| R31 | Part 4: live speaking-rate meter showing slow down or speed up | PASS | 03-later-phases.md, "Part 4. Voice and speech", meter bullet; task L-04 |
| R32 | Part 4: after-session analysis with notes and exercises | PASS | 03-later-phases.md, "Part 4. Voice and speech", after-session bullet |
| R33 | Part 4: storytelling with pauses and emotion | PASS | 03-later-phases.md, "Part 4. Voice and speech", storytelling bullet |
| R34 | Part 5: which body language suits which situation (assertive yet calm, open) | PASS | 03-later-phases.md, "Part 5. Body language and conversation", situational bullet |
| R35 | Part 5: opening-up exercises | PASS | 03-later-phases.md, "Part 5. Body language and conversation", opening-up bullet; only "timed step cards" so far, content set when the slot is approved |
| R36 | Part 5: walking exercises with a step-by-step simulation (model walk, bold walk) | PASS | 03-later-phases.md, "Part 5. Body language and conversation", walking bullet (reading of "simulation" labelled assumption) |
| R37 | Part 5: expressing emotion through the body | PASS | 03-later-phases.md, "Part 5. Body language and conversation", emotion bullet |
| R38 | Part 5: how to start a conversation, with small tips | PASS | 03-later-phases.md, "Part 5. Body language and conversation", conversation bullet |
| R39 | Part 5: quick-read tips page for right before a conversation | PASS | 03-later-phases.md, "Part 5. Body language and conversation", tips page bullet |
| R40 | Part 5: repeat phrases with body language and a smile | PASS | 03-later-phases.md, "Part 5. Body language and conversation", repeat-phrase drill bullet |
| R41 | Part 6: research what suits me | PASS | 03-later-phases.md, "Part 6. Clothes", step 1 |
| R42 | Part 6: looks generated on my own photos, with a plan to collect and label them | PASS | 03-later-phases.md, "Part 6. Clothes", steps 2-3; tasks L-05, L-06; decision 30 |
| R43 | Part 6: find the items on Amazon | PASS | 03-later-phases.md, "Part 6. Clothes", step 4 and iOS limits (search API closed; Amazon Lens hand-off) |
| R44 | Part 6: not fashion-trend chasing | PASS | 03-later-phases.md, "Part 6. Clothes", Goal and step 1; 00-overview.md, "Won't-build list" |
| R45 | Part 7: journal every night, with gratitude | PASS | 02-journal.md, "Nightly flow and gratitude step"; task J-04; decision 8 |
| R46 | Part 7: a reminder in the morning | PASS | 02-journal.md, "Morning reminder and time defaults" and "Notifications"; tasks J-05, J-06; decision 10 |
| R47 | Part 7: beating procrastination | PASS | 03-later-phases.md, "Part 7. Productivity (rest)", procrastination bullet |
| R48 | Part 7: decision making | PASS | 03-later-phases.md, "Part 7. Productivity (rest)", decision-making bullet |
| R49 | Part 7: ideas from books such as Thinking, Fast and Slow and Atomic Habits | PASS | 03-later-phases.md, "Part 7. Productivity (rest)", book ideas bullet |
| R50 | Reader and journal not ranked unless labelled assumption | PASS | 00-overview.md, "How the apps fit around Big Brain", Phase 1 row (build order labelled assumption); decision 3 |
| R51 | Journal kept small, not over-scoped | PASS | 02-journal.md, "Goal" and "Similar apps and the case against"; tasks J-01..J-11; 00-overview.md, "Won't-build list" (Journal) |
| R52 | Early task: a Claude Code session on the Mac inspects Jeet and reports what is reusable | PASS | 01-reader.md, "Task table", R-01 in Phase 0 |
| R53 | Speech-to-text: Recorder pipeline or on-device SpeechAnalyzer, with the choice and why | PASS | 01-reader.md, "Speech-to-text choice"; decision 17 |
| R54 | LLM options and a recommendation with no Claude API key, put in decisions | PASS | 01-reader.md, "LLM options (no Claude API key)"; decision 14 |
| R55 | Whether app data feeds Big Brain is a decision question | PASS | 04-decisions-and-checklist.md, decision 12; 02-journal.md, "Feeding Big Brain"; 00-overview.md, "How the apps fit around Big Brain" |
| R56 | Budget per phase: time per day, turns, model per stage, API-equivalent cost and weekly subscription share | PASS | 00-overview.md, "Budget" and the phase table in "How the apps fit around Big Brain" |
| R57 | Big Brain loop kept first | PASS | 00-overview.md, "How the apps fit around Big Brain" (Phase 0, stop rule, loop floor); decision 1 |
| R58 | Instagram not used | PASS | 00-overview.md, "Won't-build list"; 03-later-phases.md, Part 2 Goal and Part 6 step 1; a script search finds no other mention in the plan |
| R59 | French left out | PASS | 00-overview.md, "Won't-build list"; a script search finds no other mention. Three Duolingo screens cited in 01-reader.md "Screens" (S2, S3 runner-up) show a French course; only their layout is used |
| R60 | Call history left out | PASS | 00-overview.md, "Won't-build list"; 03-later-phases.md, Part 2 Goal |
| R61 | 4-hours-a-day budget | PASS | 00-overview.md, "How the apps fit around Big Brain" (loop plus apps hours total 4 in every phase) and "Budget"; decision 2 |
| R62 | Times in Pacific time; the 4-hour interval configurable | PASS | 01-reader.md, "Schedule and spacing" and "Screens" S11 (task R-26); 02-journal.md, "Morning reminder and time defaults" (task J-07); decision 11 |
| R63 | Each Reader and journal screen cites a Mobbin reference by app name | PASS | 01-reader.md, "Screens" (12 rows); 02-journal.md, "Screens" (7 rows); a script found a Mobbin link in every row, and each row names the chosen app, the reason and a runner-up |
| R64 | Reader has the no-skip gate, test-out, dictate-back, gap analysis and calls | PASS | 01-reader.md, sections "No-skip gate and test-out", "Dictate-back loop", "Recall scoring, gap and root cause", "Notifications and calls"; tasks R-14, R-16, R-17, R-21, R-24, R-29..R-34 |
| R65 | iOS limits stated, each with a nearest workable alternative marked proposal | PASS | 01-reader.md, "iOS limits that block a piece" (every alternative marked proposal); 03-later-phases.md, each card's "iOS limits and workaround"; 02-journal.md, "Notifications" |
| R66 | Each part has a strongest case against, with "use X instead" where it applies | PASS | 01-reader.md and 02-journal.md, "Similar apps and the case against"; 03-later-phases.md, each card's "Case against" |
| R67 | Reader and journal near build-ready; all other parts brief and phased | PASS | 01-reader.md and 02-journal.md (design, task tables, checks); 03-later-phases.md, "Part cards" (six fields each, with Phase); 00-overview.md, phase table |
| R68 | Reuse of Jeet, Recorder, ClawCall and Big Brain marked to verify where needed | PASS | 01-reader.md, "Reuse and to verify"; 02-journal.md, "Reuse and to verify" |
| R69 | No Claude API key assumed | PASS | 01-reader.md, "LLM options (no Claude API key)" and "Checks" item 13; task J-02 key search; 00-overview.md, "Won't-build list" |
| R70 | Open questions numbered, each with a default | PASS | 04-decisions-and-checklist.md, "Decisions" (31 items, meditation app last) |
| R71 | Owner summary present | PASS | 00-overview.md, "Owner summary" (7 lines) |
| R72 | Won't-build list present | PASS | 00-overview.md, "Won't-build list" |
| R73 | Top-5 risks present | PASS | 00-overview.md, "Top-5 risks" (5 items, each linked to its detail) |
