# Decisions and checklist

## Decisions

Ordered by what blocks the build soonest. Each default is a proposal; nothing is decided until you say yes, ok or go. Decided items keep their number. Items marked "Default, Talwinder can reverse" were applied on 2026-10-25 at the owner's instruction; the rest that are not decided (14, 24, 25, 29, 30, 31, 32) involve money, privacy, Anthropic's terms or something hard to undo, and stay open.

Decision 32 was added after the list was ordered; it blocks R-39 and the runtime model jobs.

1. Decided 2026-10-25: only digest-dependent work (the Big Brain folder hookup in J-09 and R-38, and the Big Brain intake session) waits for 3 clean digest nights in a row, earliest about 2026-10-30; app building starts now with J-01 and the Reader book-package test R-02..R-08, in parallel, since neither depends on the digest. The stop rule counts from the day that gate first passes.
2. Decided 2026-10-25 (defaults accepted): loop 2.5 h and apps 1.5 h a day from now, while Phases 0 and 1 run together; loop 1.5 h from Phase 2 on and never less; app time about 20% talk, 50% build, 30% feedback; build sessions pause until the weekly reset once the Max 7-day usage bar passes 60% (the only usage limit; corrected 2026-10-25); daily reading and journaling sit outside the 4 h.
3. Decided 2026-10-25: build our own Journal (J-01 already creates the Journal app); Talwinder can reverse this. Five Minute Journal stays the fallback under the kill rule in [case against](02-journal.md#similar-apps-and-the-case-against).
4. Decided 2026-10-25: Journal first (Phase 1), then the Reader app from R-09 (Phases 2-3); build order only, not rank. R-02..R-08 keep running in parallel.
5. Decided 2026-10-25: build the Reader; Talwinder wants to keep adding features over time. No RemNote, Recite or Blurt trial. Order unchanged: Journal first, then the Reader from R-09 ([decision 4](#decisions)).
6. Decided 2026-10-25: the paid Apple Developer Program is already bought (no new purchase); activation is pending with Apple, expected about 2026-10-26. J-01 starts now on the free account, and signing switches to the paid team once it is active ([iOS limits](01-reader.md#ios-limits-that-block-a-piece)).
7. Decided 2026-10-25: the apps run on the main iPhone, an iPhone 15 Pro Max (Apple Intelligence capable). The spare iPhone 15 Pro can run them too and is optional until it takes over the Recorder ([decision 8](#decisions)). Battery is not weighed. [R-20](01-reader.md#phase-2-reader-mvp) may test Apple's on-device model. The iOS version is not confirmed: J-01 checks it is iOS 26 or later.
8. Decided 2026-10-25: the Recorder runs on the main iPhone with the DJI mic for now and moves to the spare iPhone 15 Pro long-term, streaming 24/7 to the Mac; the Recorder project sets the move date. Once it moves, the main iPhone's mic is free for Reader retells; until then R-30 checks whether the Reader can record while the Recorder runs.
9. Decided 2026-10-25: one new repo, `~/Projects/mundi-apps`, with one Xcode workspace, two apps (Journal, Reader) and one shared local Swift package; J-01 creates it, and each app keeps its own pending-notification cap. The GitHub remote is private. Room is left for a third app: the learning-system app from your separate decision list (D30) may join this workspace but keeps its own SRS engine (D31). Those D-numbers are not this plan's decisions 30 and 31.
10. Decided 2026-10-25: gratitude is always on; best moment, lesson and mood are each switchable in settings; all four are on by default.
11. Default, Talwinder can reverse (2026-10-25): journal reminders at 10:00 pm nightly and 7:15 am in the morning, Pacific; the journal day starts at 4:00 am; reminder times editable in 15-minute steps, the day start by the hour.
12. Default, Talwinder can reverse (2026-10-25): the morning reminder opens last night's gratitude lines plus one "Today I will" line, and offers to fill in a blank night.
13. Default, Talwinder can reverse (2026-10-25): when travelling, journal reminders, Reader slots and quiet hours follow the phone's time zone (Pacific at home), with a setting that pins Pacific; same rule in both apps.
14. Should app data (journal entries, Reader progress, later tracking files and decision worksheets) feed Big Brain as optional files in its input folder? Default: yes; one file per day (per decision for worksheets), a missing file means no data, no Big Brain code changes, and Big Brain reads them only after the Phase 0 gate.
15. Default, Talwinder can reverse (2026-10-25): each book covers the whole book first (brief and chapter summaries), then takes each chapter deeper; depth is fixed at import, with the full text as the target.
16. Decided 2026-10-25: quality first. Claude cloud models are the default for every model job, with no API key: Haiku for bulk, Sonnet where quality matters (job list in [LLM options](01-reader.md#llm-options-no-claude-api-key)). A local or on-device model may take a job only after a side-by-side test (R-20) shows it matches cloud quality.
17. Default, Talwinder can reverse (2026-10-25): the gate is on each topic (3-7 paragraphs), not every paragraph.
18. Default, Talwinder can reverse (2026-10-25): on-device SpeechAnalyzer for live words; R-15 compares it with the Recorder's Whisper on the same passage and the more accurate transcript is scored (Whisper's only once the relay exists, [decision 32](#decisions)); the Recorder pipeline is also the Punjabi or Hindi fallback.
19. Default, Talwinder can reverse (2026-10-25): "two-step thinking" means pass 1 finds the missing idea units and pass 2 proposes why (never learned, misunderstood, forgotten), shown only as a hint.
20. Default, Talwinder can reverse (2026-10-25): your 4-hour base gap, growing by ease ([decision 21](#decisions)), with a hand-off to FSRS days at 24 h (two hour-scale reviews per topic instead of three); the research's fixed 2 h, 4 h and 8 h steps ship as a settings preset.
21. Default, Talwinder can reverse (2026-10-25): the gap grows after each successful retell, multiplied by ease as in Anki (start 2.5, floor 1.3).
22. Default, Talwinder can reverse (2026-10-25): at most 5 piled-up reviews go into one session before the new topic; the rest roll to the next slot.
23. Default, Talwinder can reverse (2026-10-25): a Guided Access tip plus a "left mid-topic" ping in Phase 2; an optional shield over other apps in Phase 3.
24. Which calls should ClawCall place, and how often? Default: push and quiz on, boost off (facts only when on); at most 1 call a day, none in quiet hours.
25. Does paper-book mode assume you read aloud? Default: yes; start and end page photos with OCR cover silent reading.
26. Default, Talwinder can reverse (2026-10-25): later parts follow the Reader extras in this order: Part 3 Mind, Part 2 Self-tracking, Part 4 Voice, Part 7 rest, Part 5 Body language, Part 6 Clothes, one at a time, each behind its gate.
27. Default, Talwinder can reverse (2026-10-25): later parts are hosted as Reader packs (Parts 3 and 7, lessons of Parts 4-5), Journal additions (Part 7's if-then line and decision worksheet), one new Practice app (drills of Parts 4-5) and one Tracker app (Part 2), both new targets in the [decision 9](#decisions) workspace, and Mac-only scripts (Part 6).
28. Default, Talwinder can reverse (2026-10-25): blind men and the elephant leads the biases pack, paired with the invisible gorilla experiment; runner-up Plato's cave.
29. Should the location log keep only visits and significant moves, or also a continuous precise trail? Default: visits and significant moves; the continuous trail only when switched on.
30. Build camera-based room tracking at all? Default: no; reconsider after Part 2 v1 has run 4 weeks, starting with a 1-day heat test on a phone that is not the Recorder (battery is not weighed, [decision 7](#decisions)).
31. Which cloud service may receive your face photos for LoRA training and render the final looks with that LoRA? Default: fal for both, only after L-06 shows its trainer and render model are outside the no-training carve-outs, with payload storage off; never Higgsfield.
32. Build the Mac relay that runs Claude Code headless on your Max login for runtime scoring and chat (R-39)? On hold: Talwinder decides separately; risks in [runtime path](01-reader.md#llm-options-no-claude-api-key). Default: hold; quality argues for the relay, the terms risk for waiting. Until then retells are scored by your own confirmation on S6, or by a local model that passed R-20. The hold also covers any phone-to-Mac link for a Mac-hosted local model (assumption).
33. Default, Talwinder can reverse (2026-10-25): no meditation app is assumed; Part 3 ends with a generic meditation hand-off until Talwinder names the app.

## Checklist

| id | requirement | status | evidence or reason |
|---|---|---|---|
| R1 | Part 1: PDF or book in, broken into learning levels, introduction first, then deeper | PASS | 01-reader.md, "PDF-to-levels pipeline" (level order) and "Data model" (Level); tasks R-02..R-08, R-10; decision 15 |
| R2 | Part 1: one paragraph per screen, moves on after reading, tracks how much is read | PASS | 01-reader.md, "No-skip gate and test-out", "Screens" S4, "Data model" (ReadEvent); tasks R-11, R-14 |
| R3 | Part 1: blocks skipping ahead and losing interest; what must be learned is fixed | PASS | 01-reader.md, "No-skip gate and test-out" and "iOS limits that block a piece" (focus guard); tasks R-14, R-27, R-36; decision 23 |
| R4 | Part 1: talk about the text and ask it for more | PASS | 01-reader.md, "Talk about the text"; task R-32. PASS on design; live chat waits for decision 32 or an R-20-passed local model |
| R5 | Part 1: second mode records reading of a physical book and analyzes how much was read | PASS | 01-reader.md, "Physical-book mode"; tasks R-30, R-31; decisions 8, 25 |
| R6 | Part 1: first run gives a brief, asks what I know, builds a next step that adds new content and tests earlier material | PASS | 01-reader.md, "No-skip gate and test-out" (First run) and "Schedule and spacing" (a) session slots; tasks R-12, R-23 |
| R7 | Part 1: repeats every 4 hours by default, easy to change (for example to 2 hours) | PASS | 01-reader.md, "Schedule and spacing" (a) and "Screens" S11; tasks R-22, R-25, R-26 |
| R8 | Part 1: Anki-style rule at hour scale, gap growth labelled assumption, hand-off to days, difference in 3 lines, decision with default | PASS | 01-reader.md, "Schedule and spacing" (a)-(d); task R-22; decisions 20, 21 |
| R9 | Part 1: dictate-back: retell, hear what I forgot, retell again | PASS | 01-reader.md, "Dictate-back loop"; tasks R-16, R-21. PASS on design, with the live-scorer caveat in R64 |
| R10 | Part 1: gap analysis of the specific missing idea and its root cause; two-step reading labelled assumption, hint only, decision | PASS | 01-reader.md, "Recall scoring, gap and root cause"; tasks R-17, R-21; decision 19. PASS on design, with the live-scorer caveat in R64 |
| R11 | Part 1: calls during learning to push, boost or quiz | PASS | 01-reader.md, "Notifications and calls"; tasks R-29, R-33, R-34; decision 24. ClawCall's per-call script and transcript return are (to verify) in 01-reader.md, "Reuse and to verify", ClawCall row; R-29 checks them, and if either is missing, quiz calls drop and decision 24 is asked again |
| R12 | Part 2: precise location sent by my own app, no Find My | PASS | 03-later-phases.md, "Part 2. Self-tracking" (Goal, Smallest useful version, iOS limits); decision 29 |
| R13 | Part 2: HealthKit steps, other activity, sleep, sleep and wake times | PASS | 03-later-phases.md, "Part 2. Self-tracking" (Smallest useful version, iOS limits) |
| R14 | Part 2: screen time | PASS | 03-later-phases.md, "Part 2. Self-tracking", iOS limits row (blocked; spike and fallbacks); task L-03 |
| R15 | Part 2: website history | PASS | 03-later-phases.md, "Part 2. Self-tracking", iOS limits row (blocked on iPhone; Mac-side and domain-list alternatives) |
| R16 | Part 2: camera or vision of how I work and spend time in a room | UNKNOWN | 03-later-phases.md, "Part 2. Self-tracking" gives the block and a stand-phone alternative; heat and privacy cost are unmeasured, and decision 30 defaults to no |
| R17 | Part 2: iOS limits stated honestly, nearest workable alternative for each blocked item | PASS | 03-later-phases.md, "Part 2. Self-tracking", iOS limits table; each row's alternative is marked (proposal) |
| R18 | Part 3: mental models | PASS | 03-later-phases.md, "Part 3. Mind", Mental models bullet |
| R19 | Part 3: biases with examples and stories, including a perception story chosen from candidates | PASS | 03-later-phases.md, "Part 3. Mind", Biases bullet (choice labelled assumption); decision 28 |
| R20 | Part 3: anger: how the frontal lobe and which tricks regain control | PASS | 03-later-phases.md, "Part 3. Mind", Anger bullet (corrects the brainstem framing; tricks in evidence order) |
| R21 | Part 3: guided meditation, existing apps checked on Mobbin, candidates unverified, question in decisions | PASS | 03-later-phases.md, "Part 3. Mind", Guided meditation bullet (Mobbin links, all marked unverified); decision 33, last |
| R22 | Part 3: default mode and task-positive networks, effects of meditation and exercise | PASS | 03-later-phases.md, "Part 3. Mind", Brain networks bullet |
| R23 | Part 3: probability and statistics with examples and simulations | PASS | 03-later-phases.md, "Part 3. Mind", Probability bullet |
| R24 | Part 3: only real psychology experiments, simplified or disputed claims flagged | PASS | 03-later-phases.md, "Part 3. Mind", Psychology experiments bullet; task L-01 blocks unsourced claims and myth phrases |
| R25 | Part 3: same flow as the Reader (brief, ask, add more, deepen) | PASS | 03-later-phases.md, "Part 3. Mind", Goal and Reuse |
| R26 | Part 4: warm-ups: humming different tones, counting 1-2-3 | PASS | 03-later-phases.md, "Part 4. Voice and speech", Warm-up bullet (counting reading labelled assumption) |
| R27 | Part 4: pen-between-the-teeth drill with words for different vowel movements | PASS | 03-later-phases.md, "Part 4. Voice and speech", pen drill bullet (shown as unproven) |
| R28 | Part 4: a section for a US (California) accent | PASS | 03-later-phases.md, "Part 4. Voice and speech", accent bullet (hand-off to BoldVoice or ELSA) |
| R29 | Part 4: grammar and sentence building, spoken and written | PASS | 03-later-phases.md, "Part 4. Voice and speech", grammar bullet |
| R30 | Part 4: common new English words | PASS | 03-later-phases.md, "Part 4. Voice and speech", new words bullet (reading labelled assumption) |
| R31 | Part 4: live speaking-rate meter showing slow down or speed up | PASS | 03-later-phases.md, "Part 4. Voice and speech", meter bullet; task L-04 |
| R32 | Part 4: after-session analysis with notes and exercises | PASS | 03-later-phases.md, "Part 4. Voice and speech", after-session bullet |
| R33 | Part 4: storytelling with pauses and emotion | PASS | 03-later-phases.md, "Part 4. Voice and speech", storytelling bullet |
| R34 | Part 5: which body language suits which situation (assertive yet calm, open) | PASS | 03-later-phases.md, "Part 5. Body language and conversation", situational bullet |
| R35 | Part 5: opening-up exercises | PASS | 03-later-phases.md, "Part 5. Body language and conversation", opening-up bullet: open-posture drills as timed step cards; reading labelled assumption |
| R36 | Part 5: walking exercises with a step-by-step simulation (model walk, bold walk) | PASS | 03-later-phases.md, "Part 5. Body language and conversation", walking bullet (reading of "simulation" labelled assumption) |
| R37 | Part 5: expressing emotion through the body | PASS | 03-later-phases.md, "Part 5. Body language and conversation", emotion bullet |
| R38 | Part 5: how to start a conversation, with small tips | PASS | 03-later-phases.md, "Part 5. Body language and conversation", conversation bullet |
| R39 | Part 5: quick-read tips page for right before a conversation | PASS | 03-later-phases.md, "Part 5. Body language and conversation", tips page bullet |
| R40 | Part 5: repeat phrases with body language and a smile | PASS | 03-later-phases.md, "Part 5. Body language and conversation", repeat-phrase drill bullet |
| R41 | Part 6: research what suits me | PASS | 03-later-phases.md, "Part 6. Clothes", step 1 |
| R42 | Part 6: looks generated on my own photos, with a plan to collect and label them | PASS | 03-later-phases.md, "Part 6. Clothes", steps 2-3; tasks L-05, L-06; decision 31 |
| R43 | Part 6: find the items on Amazon | PASS | 03-later-phases.md, "Part 6. Clothes", step 4 and iOS limits (search API closed; Amazon Lens hand-off) |
| R44 | Part 6: not fashion-trend chasing | PASS | 03-later-phases.md, "Part 6. Clothes", Goal and step 1; 00-overview.md, "Won't-build list" |
| R45 | Part 7: journal every night, with gratitude | PASS | 02-journal.md, "Nightly flow and gratitude step"; task J-04; decision 10 |
| R46 | Part 7: a reminder in the morning | PASS | 02-journal.md, "Morning reminder and time defaults" and "Notifications"; tasks J-05, J-06; decision 12 |
| R47 | Part 7: beating procrastination | PASS | 03-later-phases.md, "Part 7. Productivity (rest)", procrastination bullet |
| R48 | Part 7: decision making | PASS | 03-later-phases.md, "Part 7. Productivity (rest)", decision-making bullet |
| R49 | Part 7: ideas from books such as Thinking, Fast and Slow and Atomic Habits | PASS | 03-later-phases.md, "Part 7. Productivity (rest)", book ideas bullet |
| R50 | Reader and journal not ranked unless labelled assumption | PASS | 00-overview.md, "How the apps fit around Big Brain", Phase 1 row (Journal first, not ranked); decision 4 (decided 2026-10-25: build order only) |
| R51 | Journal kept small, not over-scoped | PASS | 02-journal.md, "Goal" and "Similar apps and the case against" (kill rule, Five Minute Journal as fallback); tasks J-01..J-11; 00-overview.md, "Won't-build list" (Journal); decision 3 (decided 2026-10-25: build) |
| R52 | Early task: a Claude Code session on the Mac inspects Jeet and reports what is reusable | PASS | 01-reader.md, "Task table", R-01 in Phase 0 |
| R53 | Speech-to-text: Recorder pipeline or on-device SpeechAnalyzer, with the choice and why | PASS | 01-reader.md, "Speech-to-text choice" (SpeechAnalyzer for live words; task R-15 compares it with the Recorder's Whisper); decision 18 |
| R54 | LLM options and a recommendation with no Claude API key, put in decisions | PASS | 01-reader.md, "LLM options (no Claude API key)" (options, job-to-model list, runtime path); decision 16, decided 2026-10-25: quality first; decision 32 (relay on hold; meanwhile your own verdicts or an R-20-passed local model) |
| R55 | Whether app data feeds Big Brain is a decision question | PASS | 04-decisions-and-checklist.md, decision 14; 02-journal.md, "Feeding Big Brain"; 00-overview.md, "How the apps fit around Big Brain" |
| R56 | Budget per phase: time per day, turns, model per stage, API-equivalent cost and weekly subscription share | PASS | 00-overview.md, "Budget" (phase rows, a runtime row that applies only if decision 32 says build, model per stage) and the phase table in "How the apps fit around Big Brain"; runtime shares are estimates until measured in the relay's first week |
| R57 | Big Brain loop kept first | PASS | 00-overview.md, "How the apps fit around Big Brain" (Phase 0 gate on digest-dependent work, stop rule, loop floor with the 60% build pause, the only usage limit) and "Budget" (Phase 0 `/usage` calibration); decision 1 (decided: apps start in parallel), decision 2 |
| R58 | Instagram not used | PASS | 00-overview.md, "Owner summary" and "Won't-build list"; 03-later-phases.md, Part 2 Goal and Part 6 step 1 |
| R59 | French left out | PASS | 00-overview.md, "Owner summary" and "Won't-build list". Screens cited with French UI serve only as layout or app name: Duolingo in 01-reader.md "Screens" (S2, S3 runner-up), Calm in 03-later-phases.md, Part 3 |
| R60 | Call history left out | PASS | 00-overview.md, "Owner summary" and "Won't-build list"; 03-later-phases.md, Part 2 Goal |
| R61 | 4-hours-a-day budget | PASS | 00-overview.md, "How the apps fit around Big Brain" (loop plus apps hours total 4 in every phase) and "Budget"; decision 2 |
| R62 | Times in Pacific time; the 4-hour interval configurable | PASS | 01-reader.md, "Schedule and spacing" and "Screens" S11 (task R-26); 02-journal.md, "Morning reminder and time defaults" (task J-07); decision 13 |
| R63 | Each Reader and journal screen cites a Mobbin reference by app name | PASS | 01-reader.md, "Screens" (12 rows); 02-journal.md, "Screens" (7 rows); every row has a Mobbin link (script check), the chosen app, a reason and a runner-up; both files list the 3-5 candidates compared per screen |
| R64 | Reader has the no-skip gate, test-out, dictate-back, gap analysis and calls | PASS | 01-reader.md, sections "No-skip gate and test-out", "Dictate-back loop", "Recall scoring, gap and root cause", "Notifications and calls"; tasks R-14, R-16, R-17, R-21, R-24, R-29..R-34. PASS on design; the live scorer for gap analysis and quiz calls needs the relay (decision 32, R-39) or a local model that passed R-20 (R-40); until then you confirm verdicts on S6 |
| R65 | iOS limits stated, each with a nearest workable alternative marked proposal | PASS | 01-reader.md, "iOS limits that block a piece" (every alternative marked proposal); 03-later-phases.md, each card's "iOS limits and workaround"; 02-journal.md, "Notifications" |
| R66 | Each part has a strongest case against, with "use X instead" where it applies | PASS | 01-reader.md and 02-journal.md, "Similar apps and the case against"; 03-later-phases.md, each card's "Case against" |
| R67 | Reader and journal near build-ready; all other parts brief and phased | PASS | 01-reader.md and 02-journal.md (design, task tables, checks); 03-later-phases.md, "Part cards" (six fields each); 00-overview.md, phase table |
| R68 | Reuse of Jeet, Recorder, ClawCall and Big Brain marked to verify where needed | PASS | 01-reader.md, "Reuse and to verify"; 02-journal.md, "Reuse and to verify" |
| R69 | No Claude API key assumed | PASS | 01-reader.md, "LLM options (no Claude API key)" (runtime relay on hold, decision 32; if built, R-39 uses the Max login and refuses to run with ANTHROPIC_API_KEY set; terms risk and interim path stated) and "Checks" item 13; task J-02 key search; 00-overview.md, "Won't-build list" |
| R70 | Open questions numbered, each with a default | PASS | 04-decisions-and-checklist.md, "Decisions" (33 items, meditation app last; 26 decided or applied by default, 7 still open with defaults: 14, 24, 25, 29, 30, 31, 32) |
| R71 | Owner summary present | PASS | 00-overview.md, "Owner summary" (5 lines) |
| R72 | Won't-build list present | PASS | 00-overview.md, "Won't-build list" |
| R73 | Top-5 risks present | PASS | 00-overview.md, "Top-5 risks" (5 items, each linked to its detail) |
