# Personal iOS apps: overview

## Owner summary

- Two apps first: a Journal (nightly gratitude, morning reminder) and a Reader (PDF to levels, one paragraph per screen, voice retell every 4 hours, calls from the Mac); six later parts follow one at a time.
- Big Brain comes first: work that feeds or reads the digest waits for 3 clean digest nights in a row (about 2026-10-30), app building starts now in parallel, the loop never gets under 1.5 hours a day, and after that gate 2 failed nights in a row pause app work.
- Time: 1.5 hours a day of app work for about 2 weeks (Journal plus a book test), then 2.5 hours for about 7-10 weeks (Reader); reading sessions ([an estimated 40-60 minutes a day](01-reader.md#similar-apps-and-the-case-against)) and journaling (about 3 minutes) come on top (assumption).
- Cost: no Claude API key. Quality first: Claude cloud models are the default for every model job, Haiku for bulk, Sonnet where quality matters ([decision 16](04-decisions-and-checklist.md#decisions)). It all draws on the Max plan's weekly limit, worth about 87 USD of API use for Phases 0-3 and about 13 USD per new book (comparison only, as of 2026-10-25); runtime scoring and chat add about 1-3 USD a day only if the relay is approved ([decision 32](04-decisions-and-checklist.md#decisions)); weekly shares and fixed costs in [budget](#budget).
- Left out: Instagram, French, call history, any Claude API key, a phone lock (iOS needs MDM), reading Safari history on the iPhone, an own accent engine, Amazon search by API, App Store release ([full list](#wont-build-list)).

## Goal

Plan the Journal and the Reader near build-ready, and the six later parts briefly, in build order, for one person on one iPhone.
Protect the Big Brain loop: digest-dependent app work waits until it is stable ([decision 1](04-decisions-and-checklist.md#decisions)), the apps fit around its hours, and they only ever feed it optional files.
Nothing here is decided until you say yes, ok or go; open points are numbered in [decisions](04-decisions-and-checklist.md#decisions).

## How the apps fit around Big Brain

Phase plan (proposal). Hours are per day, out of 4. Inside app hours: talk = chats with AI bots to plan and decide; build = starting and watching Claude Code sessions; feedback = testing on the iPhone and reviewing reports. Hours and split are decided ([decision 2](04-decisions-and-checklist.md#decisions)). Phases 4-9 follow the order in [decision 26](04-decisions-and-checklist.md#decisions).

| phase | start / gate | loop h/day | apps h/day | inside apps: talk / build / feedback | what is built (task ids) | why this order |
|---|---|---|---|---|---|---|
| 0. Digest gate | From 2026-10-26: 24-hour recorder test, then the digest resumes. Exit: 3 clean nights in a row ([decision 1](04-decisions-and-checklist.md#decisions)), earliest about 2026-10-30. Gates only digest-dependent work: the Big Brain folder hookup in J-09 and R-38, and the Big Brain intake session | same days as Phase 1; hours in that row | — | — | R-01 Jeet inspection, read-only, runs unattended | The loop is the base the rest depends on; only work that feeds or reads the digest waits for it. R-01 costs a 15-minute review and informs J-06 and R-22 |
| 1. Journal and book-package spike | Now (2026-10-25), in parallel with Phase 0 ([decision 1](04-decisions-and-checklist.md#decisions)). Exit: J-11 done, [journal checks](02-journal.md#checks) C1-C9 pass and R-08 passed (pilot package clean); C10 runs once the Phase 0 gate has passed and does not hold the exit. About 2 weeks | 2.5 | 1.5 | 0.25 / 0.75 / 0.5 | J-01..J-11, R-02..R-08 | Journal first, then the Reader app (R-09 on): build order only, not rank ([decision 4](04-decisions-and-checklist.md#decisions)). The spike tests the Reader's riskiest input before any Reader code and runs in parallel |
| 2. Reader MVP, text mode | Phase 1 exit. Exit: [Reader checks](01-reader.md#checks) 1-8, 12 and 13 pass; R-19 result saved. About 4-6 weeks | 1.5 | 2.5 | 0.5 / 1.25 / 0.75 | R-09..R-28; R-39 if and when [decision 32](04-decisions-and-checklist.md#decisions) says build; R-40 only if R-20 passes | Gate, retell, scoring and spacing are the core; every extra depends on them |
| 3. Reader extras | Phase 2 exit and the use check in the [Reader case against](01-reader.md#similar-apps-and-the-case-against). Exit: Reader checks 9 and 11 pass; check 10 waits for a chat model and does not hold the exit. About 3-4 weeks | 1.5 | 2.5 | 0.5 / 1.25 / 0.75 | R-29..R-38 | Calls, paper-book mode, chat and the shield lean on unverified services and devices; built only once the MVP is in use |
| 4. Later slot 1: Part 3 Mind | Phase 3 exit and your go; build gate in [Part 3](03-later-phases.md#part-3-mind) | 1.5 | 2.5 | 0.5 / 1.25 / 0.75 | L-01, then packs written at approval | Highest reuse of the Reader engine |
| 5. Later slot 2: Part 2 Self-tracking | Slot 1 closed and your go; build gate in [Part 2](03-later-phases.md#part-2-self-tracking) | 1.5; 3.5 in trial | 2.5; 0.5 in trial | 0.5 / 1.25 / 0.75 when building | L-02, L-03; Tracker app only if L-02 finds a gap | Small; feeds the loop |
| 6. Later slot 3: Part 4 Voice | Slot 2 closed and your go; build gate in [Part 4](03-later-phases.md#part-4-voice-and-speech) | 1.5; 3.5 in trial | 2.5; 0.5 in trial | 0.5 / 1.25 / 0.75 when building | L-04, then the Practice app | Reuses the speech module |
| 7. Later slot 4: Part 7 rest | Slot 3 closed and your go; build gate in [Part 7](03-later-phases.md#part-7-productivity-rest) | 1.5; 3.5 in trial | 2.5; 0.5 in trial | 0.5 / 1.25 / 0.75 when building | Packs, if-then morning line, decision worksheet | Content in the Reader engine; follows Part 3 |
| 8. Later slot 5: Part 5 Body language | Slot 4 closed and your go; build gate in [Part 5](03-later-phases.md#part-5-body-language-and-conversation) | 1.5; 3.5 in trial | 2.5; 0.5 in trial | 0.5 / 1.25 / 0.75 when building | One pack, Practice drills, tips page | Depends on Part 4; weakest evidence |
| 9. Later slot 6: Part 6 Clothes | Slot 5 closed and your go; build gate in [Part 6](03-later-phases.md#part-6-clothes) | 1.5 | 2.5 | 0.5 / 1.25 / 0.75 | L-05, L-06, then Mac scripts | Highest privacy and money cost |

**Stop rule (approved 2026-10-25).** A failed digest night has no digest or needs a manual fix; a clean night has neither. After 2 failed nights in a row, app work pauses and its hours go to the loop until 2 clean nights in a row. The rule counts from the day the Phase 0 gate first passes, since failed nights during the digest test are expected and app work runs in parallel by [decision 1](04-decisions-and-checklist.md#decisions). A phase starts only when you say go.

**Later slots (proposal).** Phases 4-9 run one at a time. A slot closes when its approved build tasks are done, or when it stops at its build gate; trials and gate tasks run first inside it ([part cards](03-later-phases.md#part-cards)).

**Loop floor (approved 2026-10-25, [decision 2](04-decisions-and-checklist.md#decisions)).** The loop keeps at least 1.5 h a day in every phase. In a later slot's trial weeks (a "use X instead" app tried for 2 weeks, or a course), app time drops to 0.5 h and the rest goes back to the loop. Big Brain's Claude sessions, app build sessions and, if the relay is built ([decision 32](04-decisions-and-checklist.md#decisions)), the Reader's runtime calls draw on the same Max weekly limit (to verify). Build sessions (app code and book pipelines) pause until the weekly reset once the Max plan's 7-day usage bar (`/usage`) passes 60%; that is the only usage limit. Runtime scoring and chat keep running once the relay exists (proposal).

**How the apps feed the loop.** Only as optional files in the folder Big Brain reads, if [decision 14](04-decisions-and-checklist.md#decisions) is yes. A missing file means no data. This plan changes no Big Brain code; a Big Brain session adds the reading side after the Phase 0 gate.
- Journal: one file per night, J-09 ([feeding Big Brain](02-journal.md#feeding-big-brain)).
- Reader: one progress file per day, R-38 ([Reader task table](01-reader.md#task-table)).
- Later: Part 2 day files ([Part 2](03-later-phases.md#part-2-self-tracking)) and Part 7 decision worksheets ([Part 7](03-later-phases.md#part-7-productivity-rest)).

## Budget

Hours per day are in the [phase table](#how-the-apps-fit-around-big-brain). Turn counts are estimates (proposal): Phases 0-3 from the task tables, Phases 4-9 sized here from the part cards. Dollars are API-equivalent, for comparison only, as of 2026-10-25. A light turn is one small edit, a base turn one view or model change plus a build, a heavy turn a multi-file feature with a build-test-fix loop: Sonnet base about 0.33 USD and heavy about 1.26; Haiku light about 0.005 and base about 0.02; Opus base about 0.65. With no API key, none of it is billed while usage credits stay off (your account setting, not a plan rule); it draws on the Max plan's weekly limit.

| phase | length (proposal) | turns by model and shape | API-equivalent USD | weekly share |
|---|---|---|---|---|
| 0. Digest gate | about 5 days, in parallel with Phase 1 | Sonnet 2 heavy (R-01); your review 15 min | about 2.5 | low |
| 1. Journal, J-01..J-11 | about 2 weeks, with the spike | Sonnet 15 base + 4 heavy; Haiku 2 light; scripts for J-02 checks and J-10; your Xcode signing setup about 15 min | about 10 | medium, both Phase 1 rows together |
| 1. Book-package spike, R-02..R-08 | same 2 weeks | Sonnet 9 base + 9 heavy; Haiku 6 base; script for R-08; your paragraph hand count about 20 min (R-03) and spot check 10 min | about 14 | see row above |
| 2. Reader MVP | 4-6 weeks | Sonnet 54 base (about 15 are fix turns after device tests) + 15 heavy (one is the R-19 batch session), plus 4 heavy if R-20 runs, 2 if R-39 runs and 2 if R-40 runs; Haiku 5 base + 2 light; script for R-19's maths | about 37; R-20 adds about 5, R-39 and R-40 about 2.5 each (about 47 with all three) | medium |
| 3. Reader extras | 3-4 weeks | Sonnet 25 base (about 8 fix turns) + 12 heavy; your read-aloud tests about 30 min (R-30, check 9) | about 23 | medium |
| Each new book, from Phase 2 | one session | Sonnet 10 heavy (summaries and idea units); Haiku 4 base (quiz items, questions); 1 script lint; your spot check 10 min | about 13 per book | medium to high in that week on its own |
| Runtime, once the relay runs | only if [decision 32](04-decisions-and-checklist.md#decisions) says build the relay; every day of use; later parts add calls | 20-30 Sonnet calls a day (scoring, cause flag, chat); each resends about 30k tokens of Claude Code context, uncached when calls are hours apart (proposal) | about 0.10 USD a call uncached, 0.05 when minutes apart; 1-3 USD a day (proposal) | medium to high; measure in its first week |
| 4. Part 3 Mind | about 3 weeks | Sonnet 24 base + 2 heavy; Haiku 6 light | about 10.5 | low |
| 5. Part 2 Self-tracking | 2-week trial, then about 2 weeks | Sonnet 12 base + 2 heavy; Haiku 5 light | about 6.5 | low |
| 6. Part 4 Voice | 2-week trial, then about 3 weeks | Sonnet 17 base + 2 heavy; Haiku 4 light | about 8 | low |
| 7. Part 7 rest | 2-week trial, then about 1 week | Sonnet 7 base; Haiku 2 light | about 2.3 | low |
| 8. Part 5 Body language | course, then 1-2 weeks | Sonnet 8 base + 1 heavy; Haiku 2 light | about 3.9 | low |
| 9. Part 6 Clothes | about 2 weeks | Sonnet 5 base; Haiku 4 light; fal about 2-2.40 USD per 1,000-step training run, more for more steps (unverified, as of 2026-10-25); cloud renders of the final looks, price not checked | about 1.7 plus training and render runs | low |
| Plan revisions, any phase | per revision | Opus about 3 base | about 2 | low |

Model per stage. Quality first: Claude cloud models by default, Haiku for bulk, Sonnet where quality matters; the full job list is in [LLM options](01-reader.md#llm-options-no-claude-api-key).

| stage | model |
|---|---|
| Planning and plan revisions | Opus |
| App features, scheduler, scorer, relay (R-39, only if built), book summaries, idea units, docs that need judgment | Sonnet |
| Validators, settings-only screens, quiz items, suggested questions, harnesses, gap lists | Haiku |
| Schema validity, counts, duplicates, banned words, check-set maths, lint before any repair pass | script, no model |
| Recall scoring, cause flag and chat at runtime | Claude Sonnet through the Mac relay once [decision 32](04-decisions-and-checklist.md#decisions) says build; until then your own verdicts, or a local model that passed the side-by-side test ([decision 16](04-decisions-and-checklist.md#decisions)) |

Weekly share bands (proposal): count a heavy turn as 4 base turns and a runtime call as about a quarter of one (about their cost ratios); under 15 base-equivalent turns a week is low, 15-40 medium, over 40 high. Anthropic publishes no numeric weekly limit, so the bands are relative. They count app work only, builds and runtime; the only usage limit is in the [loop floor](#how-the-apps-fit-around-big-brain). A new book pushes its Phase 2 or 3 week to high.

Calibrate in Phase 0: read the `/usage` 7-day bar before and after R-01 (about 8 base-equivalent turns), scale each phase's weekly rate from R-01's share and re-rate the bands. The rest of that week's bar is roughly the loop's own share.

Fixed costs: Apple Developer Program 99 USD a year (as of 2026-10-25) if [decision 6](04-decisions-and-checklist.md#decisions) is yes; the ClawCall plan you already have, price not checked.

## Top-5 risks

1. App work crowds out the Big Brain loop, the base everything else depends on, in hours and in the weekly limit that app builds (and runtime Claude use, if the relay is built) share with it: [stop rule and loop floor](#how-the-apps-fit-around-big-brain).
2. The runtime scoring path is undecided: the Mac relay is on hold ([decision 32](04-decisions-and-checklist.md#decisions)), so until it or an R-20-passed local model exists, you confirm verdicts yourself; scorer quality is gated by the check set: [LLM options](01-reader.md#llm-options-no-claude-api-key), [recall scoring](01-reader.md#recall-scoring-gap-and-root-cause).
3. Drop-off from ping fatigue, piled-up reviews and a lapsed nightly habit: [Reader case against](01-reader.md#similar-apps-and-the-case-against), [journal case against](02-journal.md#similar-apps-and-the-case-against).
4. Messy PDFs give bad paragraphs, levels and idea units, and each new book costs a pipeline session: [pipeline](01-reader.md#pdf-to-levels-pipeline).
5. Apple and device gates (paid program, FamilyControls on a dev build, a mic the recorder may hold): [iOS limits](01-reader.md#ios-limits-that-block-a-piece), [physical-book mode](01-reader.md#physical-book-mode).

## Won't-build list

Items are (proposal) unless they are your exclusions or an iOS block.

**All apps**
- Instagram as a data source, any French content, call-history tracking (your exclusions).
- Anything that needs a Claude API key. The Mac relay on your Max login is on hold ([decision 32](04-decisions-and-checklist.md#decisions)).
- App Store release, other users, Android: personal use only.
- Gamification: XP, hearts, gems, leaderboards, streak counters or nags, badges.
- Big Brain code changes from this plan (intake docs only).

**Reader**
- App-started phone lock or MDM single-app mode (Guided Access tip and an optional shield instead).
- Making book packages on the phone in the MVP (Mac pipeline instead).
- Scanned PDFs (OCR) in the MVP.
- SFSpeechRecognizer for long audio.
- Import from Kindle, Drive, Gmail or web links (Files only).
- A book store or discovery feeds.
- Detecting silent reading from audio (page photos instead).
- An own VoIP or CallKit call server (ClawCall from the Mac instead).
- Keeping raw retell audio after transcription.
- Praise in boost calls (facts only).

**Journal**
- AI replies, insights or go-deeper prompts.
- Photo or video attachments; quote, affirmation or meditation feeds; social or share features.
- In v1: own audio recording (keyboard dictation only), the Journaling Suggestions picker, mood charts and insight dashboards.
- Snooze, repeat nags, time-sensitive or push notifications.

**Later parts**
- An iPhone Safari-history reader (no API exists).
- Camera room tracking until [decision 30](04-decisions-and-checklist.md#decisions) says yes; never on the Recorder phone.
- A continuous GPS trail by default; Screen Time capture beyond Apple's export API.
- An own accent engine (BoldVoice or ELSA instead); an own meditation player in v1.
- In v1: pitch scoring for humming, automatic emotion scoring of voice, smile scoring, video recording; a 3D walking figure at any stage.
- Lie-detection or "read people" content; power-pose hormone claims; Mehrabian's 7-38-55 as a general rule.
- Amazon search by API (Amazon Lens hand-off instead).
- LoRA training on the Mac; any upload to Higgsfield; trend feeds as a style source; an iOS app for Part 6 in v1.
- An own app blocker for productivity (Freedom instead).
