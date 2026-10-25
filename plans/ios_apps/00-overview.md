# Personal iOS apps: overview

## Owner summary

- Two apps first: a Journal (nightly gratitude, morning reminder) and a Reader (PDF to levels, one paragraph per screen, voice retell every 4 hours, calls placed from the Mac). The six later parts follow one at a time, each behind a gate.
- Big Brain comes first. Until the nightly digest runs 7 clean nights, the apps get 15 minutes a day for one read-only task. The loop never gets less than 1.5 hours a day, and 2 failed digest nights pause all app work.
- Time: 1.5 hours a day of app work for about 2 weeks (Journal plus a book test), then 2.5 hours a day for about 7-10 weeks (Reader). Reading and journaling themselves sit outside the 4 hours (assumption).
- Cost: no Claude API key. Build turns come out of the weekly subscription limit, at a medium share in Phases 1-3 and low after. For comparison only, that equals about 80 USD of API use for Phases 0-3, plus about 8 USD per new book (as of 2026-10-25).
- Fixed costs: the Apple Developer Program at 99 USD a year (as of 2026-10-25) if you say yes to [decision 4](04-decisions-and-checklist.md#decisions), and the ClawCall plan you already have.
- Left out: Instagram, French, call history, any Claude API key, a phone lock (iOS needs MDM), reading Safari history on the iPhone, an own accent engine, Amazon search by API, App Store release. Full list at the end.
- Case against: existing apps cover much of each part (RemNote plus Recite for the Reader, Five Minute Journal for the journal). Each file states what it costs to still build.

## Goal

Plan the Journal and the Reader near build-ready, and the six later parts briefly, in build order, for one person on one iPhone.
Protect the Big Brain loop: the apps start after it is stable, fit around its hours, and only ever feed it optional files.
Nothing here is decided until you say yes, ok or go; open points are numbered in [decisions](04-decisions-and-checklist.md#decisions).

## How the apps fit around Big Brain

Phase plan (proposal). Hours are per day, out of 4. Inside app hours: talk = chats with AI bots to plan and decide; build = starting and watching Claude Code sessions; feedback = testing on the iPhone and reviewing reports (assumption). The split is [decision 2](04-decisions-and-checklist.md#decisions). Phases 4-9 follow the order in [decision 25](04-decisions-and-checklist.md#decisions).

| phase | start / gate | loop h/day | apps h/day | inside apps: talk / build / feedback | what is built (task ids) | why this order |
|---|---|---|---|---|---|---|
| 0. Loop only | From 2026-10-26: 24-hour recorder test, then the digest resumes. Exit: 7 clean nights in a row ([decision 1](04-decisions-and-checklist.md#decisions)), earliest about 2026-11-03 | 3.75 | 0.25 | 0 / 0 / 0.25 | R-01 Jeet inspection, read-only, runs unattended | The loop is the base the rest depends on. R-01 costs a 15-minute review and informs J-06 and R-22 |
| 1. Journal and book-package spike | Phase 0 exit and your go. Exit: J-11 done (7 nights logged) and R-08 passed (pilot package clean). About 2 weeks | 2.5 | 1.5 | 0.25 / 0.75 / 0.5 | J-01..J-11, R-02..R-08 | Journal first because it is the smallest build and the first user of the shared code, not because it ranks higher (assumption), [decision 3](04-decisions-and-checklist.md#decisions). The spike tests the Reader's riskiest input before any Reader code |
| 2. Reader MVP, text mode | Phase 1 exit. Exit: [Reader checks](01-reader.md#checks) 1-8, 12 and 13 pass; R-19 result saved. About 4-6 weeks | 1.5 | 2.5 | 0.5 / 1.25 / 0.75 | R-09..R-28 | Gate, retell, scoring and spacing are the core; every extra depends on them |
| 3. Reader extras | Phase 2 exit and the use check in the [Reader case against](01-reader.md#similar-apps-and-the-case-against). Exit: Reader checks 9-11. About 3-4 weeks | 1.5 | 2.5 | 0.5 / 1.25 / 0.75 | R-29..R-38 | Calls, paper-book mode, chat and the shield lean on unverified services and devices; built only once the MVP is in use |
| 4. Later slot 1: Part 3 Mind | Gate in [Part 3](03-later-phases.md#part-3-mind) | 1.5 | 2.5 | 0.5 / 1.25 / 0.75 | L-01, then packs written at approval | Highest reuse of the Reader engine |
| 5. Later slot 2: Part 2 Self-tracking | Gate in [Part 2](03-later-phases.md#part-2-self-tracking) | 1.5; 3.5 in trial | 2.5; 0.5 in trial | 0.5 / 1.25 / 0.75 when building | L-02, L-03; Tracker app only if L-02 finds a gap | Small; feeds the loop |
| 6. Later slot 3: Part 4 Voice | Gate in [Part 4](03-later-phases.md#part-4-voice-and-speech) | 1.5; 3.5 in trial | 2.5; 0.5 in trial | 0.5 / 1.25 / 0.75 when building | L-04, then the Practice app | Reuses the speech module |
| 7. Later slot 4: Part 7 rest | Gate in [Part 7](03-later-phases.md#part-7-productivity-rest) | 1.5; 3.5 in trial | 2.5; 0.5 in trial | 0.5 / 1.25 / 0.75 when building | Packs, if-then morning line, decision worksheet | Content in the Reader engine; follows Part 3 |
| 8. Later slot 5: Part 5 Body language | Gate in [Part 5](03-later-phases.md#part-5-body-language-and-conversation) | 1.5; 3.5 in trial | 2.5; 0.5 in trial | 0.5 / 1.25 / 0.75 when building | One pack, Practice drills, tips page | Depends on Part 4; weakest evidence |
| 9. Later slot 6: Part 6 Clothes | Gate in [Part 6](03-later-phases.md#part-6-clothes) | 1.5 | 2.5 | 0.5 / 1.25 / 0.75 | L-05, L-06, then Mac scripts | Highest privacy and money cost |

**Stop rule (proposal).** If the nightly digest fails 2 nights in a row, app work pauses and its hours go to the loop until the digest runs clean again. A phase starts only when you say go.

**Loop floor (proposal).** The loop keeps at least 1.5 h a day in every phase. In a later slot's trial weeks (a "use X instead" app tried for 2 weeks, or a course), app time drops to 0.5 h and the rest goes back to the loop.

**Daily use (assumption).** Reading sessions and journaling are use, not build, and sit outside the 4 h. The Reader's estimate of daily use is in its [case against](01-reader.md#similar-apps-and-the-case-against).

**How the apps feed the loop.** Only as optional files in the folder Big Brain reads, if [decision 12](04-decisions-and-checklist.md#decisions) is yes. A missing file means no data. This plan changes no Big Brain code; a Big Brain session adds the reading side after the Phase 0 gate.
- Journal: one file per night, J-09 ([feeding Big Brain](02-journal.md#feeding-big-brain)).
- Reader: one progress file per day, R-38 ([Reader task table](01-reader.md#task-table)).
- Later: Part 2 day files ([Part 2](03-later-phases.md#part-2-self-tracking)) and Part 7 decision worksheets ([Part 7](03-later-phases.md#part-7-productivity-rest)).

## Budget

Daily hours per phase are in the [phase table](#how-the-apps-fit-around-big-brain); this section prices the turns. Turn counts are estimates (proposal) taken from the task tables and part cards. Dollars are API-equivalent, for comparison only, as of 2026-10-25, using the research turn shapes: Sonnet base about 0.33 USD and heavy about 1.26; Haiku light about 0.005 and base about 0.02; Opus base about 0.65. With no API key, none of it is billed; it draws on the weekly subscription limit.

| phase | length (proposal) | turns by model and shape | API-equivalent USD | weekly share |
|---|---|---|---|---|
| 0. Loop only | 1-2 weeks | Sonnet 2 heavy (R-01); your review 15 min | about 2.5 | low |
| 1. Journal, J-01..J-11 | about 2 weeks, with the spike | Sonnet 15 base + 4 heavy; Haiku 2 light; scripts for J-02 checks and J-10; your Xcode signing setup about 15 min | about 10 | medium, both Phase 1 rows together |
| 1. Book-package spike, R-02..R-08 | same 2 weeks | Sonnet 9 base + 6 heavy; Haiku 11 base; script for R-08; your spot check 10 min | about 11 | see row above |
| 2. Reader MVP | 4-6 weeks | Sonnet 52 base (about 15 are fix turns after device tests) + 14 heavy, plus 4 heavy if R-20 runs; Haiku 7 base + 2 light; script for R-19 | about 35; 40 with R-20 | medium |
| 3. Reader extras | 3-4 weeks | Sonnet 21 base (about 8 fix turns) + 12 heavy; Haiku 4 base | about 22 | medium |
| Each new book, from Phase 2 | one session | Sonnet 6 heavy; Haiku 8 base; 1 script lint; your spot check 10 min | about 8 per book | adds medium in that week |
| 4. Part 3 Mind | about 3 weeks | Sonnet 24 base + 2 heavy; Haiku 6 light | about 10.5 | low |
| 5. Part 2 Self-tracking | 2-week trial, then about 2 weeks | Sonnet 12 base + 2 heavy; Haiku 5 light | about 6.5 | low |
| 6. Part 4 Voice | 2-week trial, then about 3 weeks | Sonnet 17 base + 2 heavy; Haiku 4 light | about 8 | low |
| 7. Part 7 rest | 2-week trial, then about 1 week | Sonnet 7 base; Haiku 2 light | about 2.3 | low |
| 8. Part 5 Body language | course, then 1-2 weeks | Sonnet 8 base + 1 heavy; Haiku 2 light | about 3.9 | low |
| 9. Part 6 Clothes | about 2 weeks | Sonnet 5 base; Haiku 4 light; fal about 2-2.40 USD per training run (unverified, as of 2026-10-25) | about 1.7 plus runs | low |
| Plan revisions, any phase | per revision | Opus about 3 base | about 2 | low |

Model per stage:

| stage | model |
|---|---|
| Planning and plan revisions | Opus at its default effort, never xhigh or max |
| App features, scheduler, scorer, book summaries, docs that need judgment | Sonnet |
| Validators, settings screens, idea units, quiz items, harnesses, gap lists | Haiku |
| Schema validity, counts, duplicates, banned words, check-set maths, lint before any repair pass | script, no model |
| Recall scoring, cause flag and chat at runtime | Apple's on-device model, no subscription use ([LLM options](01-reader.md#llm-options-no-claude-api-key)) |

Weekly share bands (proposal): count a heavy turn as 4 base turns (about its cost ratio); under 15 base-equivalent turns a week is low, 15-40 medium, over 40 high. Anthropic publishes no numeric weekly limit, so the bands are relative. Big Brain loop work draws on the same limit and is not counted here. A new book in a Phase 2 or 3 week can push that week to high, so run books in a light week (proposal).

Calibrate after R-01: read the 7-day bar in `/usage`; R-01 is about 8 base-equivalent turns, so scale each phase's weekly rate from R-01's share and re-rate the bands.

Fixed costs: Apple Developer Program 99 USD a year (as of 2026-10-25) if [decision 4](04-decisions-and-checklist.md#decisions) is yes; the ClawCall plan, already connected, whose price was not checked (unverified).

## Top-5 risks

1. App work crowds out the Big Brain loop, the base everything else depends on: [stop rule and loop floor](#how-the-apps-fit-around-big-brain).
2. The small on-device scorer mislabels retells, so "you forgot X" is wrong and trust breaks: [recall scoring and check set](01-reader.md#recall-scoring-gap-and-root-cause).
3. Drop-off from ping fatigue, piled-up reviews and a lapsed nightly habit: [Reader case against](01-reader.md#similar-apps-and-the-case-against), [journal case against](02-journal.md#similar-apps-and-the-case-against).
4. Messy PDFs give bad paragraphs, levels and idea units, and each new book costs a pipeline session: [pipeline](01-reader.md#pdf-to-levels-pipeline).
5. Apple and device gates (paid program, Apple Intelligence iPhone, PCC eligibility, FamilyControls on a dev build, a mic the recorder may hold): [iOS limits](01-reader.md#ios-limits-that-block-a-piece), [physical-book mode](01-reader.md#physical-book-mode).

## Won't-build list

Items are (proposal) unless they are your exclusions or an iOS block.

**All apps**
- Instagram as a data source, any French content, call-history tracking (your exclusions).
- Anything that needs a Claude API key; `claude -p` as a runtime backend.
- App Store release, other users, Android: personal use only.
- Gamification: XP, hearts, gems, leaderboards, streak counters or nags, badges.
- Big Brain code changes from this plan (intake docs only).

**Reader**
- App-started phone lock or MDM single-app mode (Guided Access tip and an optional shield instead).
- Making book packages on the phone in the MVP (Mac pipeline instead).
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
- Camera room tracking until [decision 29](04-decisions-and-checklist.md#decisions) says yes; never on the Recorder phone.
- A continuous GPS trail by default; Screen Time capture beyond Apple's export API.
- An own accent engine (BoldVoice or ELSA instead); an own meditation player in v1.
- In v1: pitch scoring for humming, automatic emotion scoring of voice, smile scoring, video recording; a 3D walking figure at any stage.
- Lie-detection or "read people" content; power-pose hormone claims; Mehrabian's 7-38-55 as a general rule.
- Amazon search by API (Amazon Lens hand-off instead).
- LoRA training on the Mac; any upload to Higgsfield; trend feeds as a style source; an iOS app for Part 6 in v1.
- An own app blocker for productivity (Freedom instead).
