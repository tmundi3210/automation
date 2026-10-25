# Journal and gratitude

## Goal
A nightly journal with gratitude (about 3 minutes) and a morning reminder, for one person on one iPhone. All times are Pacific and editable.
Small weekend-sized job, built in Phase 1 before the Reader app ([decision 4](04-decisions-and-checklist.md#decisions), decided; [phase table](00-overview.md#how-the-apps-fit-around-big-brain)). It is also the first user of the shared workspace and notification code that the Reader reuses.
Done when J-11 and [checks](#checks) C1-C9 pass; C10 runs once the Phase 0 gate has passed, and C11 in weeks 2-3 of use, during Phase 2.

## Design

### Similar apps and the case against

Feature notes in this table come from store and vendor summaries, not hands-on use (unverified). Screen notes come from Mobbin screens that were seen; see [screens](#screens).

| app | what it does | what this build takes |
|---|---|---|
| Five Minute Journal | Guided morning and evening prompts, reminders, calendar and timeline. Premium adds photos, custom questions and a mood tracker. | One prompt per screen, thin progress bar, mood last; month-grouped history. |
| Day One | Encrypted journal with prompts, templates, reminders, PDF and JSON export. Not on Mobbin, so no screen is cited. | One JSON file per entry as the export format. |
| Stoic | Morning preparation and evening reflection, prompts, mood and habit trackers, AI features, free tier. | Today screen with a morning card and an evening card. |
| Rosebud | AI conversational journaling, voice entries, guided journals including gratitude, pattern insights. Not on Mobbin. | Nothing for v1. AI replies are on the won't-build list. |
| Presently | Gratitude timeline, calendar, reminders, no prompts. Android only, written in Kotlin. The README says open source, but the repo has no license file, so the code is all rights reserved. | Ideas only: a plain gratitude timeline. No code. |

**Use Five Minute Journal or Day One instead.** Both already give nightly prompts and reminders at zero build cost. Decided: build our own ([decision 3](04-decisions-and-checklist.md#decisions)); Talwinder can reverse it. Five Minute Journal stays the fallback under the kill rule below. As the fallback it needs morning gratitude off and one custom evening gratitude question: its settings show both controls, but a custom question in the evening flow was not seen, and custom questions are premium (table above), so it may need a paid plan.

**Cost to still build.** Tasks J-01 to J-11, about one weekend of build turns ([budget](00-overview.md#budget)). Upkeep: fixes after iOS updates; until the paid account is active, also a reinstall every 7 days ([iOS limits](01-reader.md#ios-limits-that-block-a-piece), [decision 6](04-decisions-and-checklist.md#decisions)).

**Why build it anyway.**
1. Entries land as plain JSON files that Big Brain can read with no manual export ([feeding Big Brain](#feeding-big-brain)).
2. Gratitude sits in the night flow and comes back as a morning recap. No AI, no upsell, no streak pressure.
3. It is the cheapest real test of the workspace, signing, storage and notification code the Reader needs. Problems found here do not stall Phase 2.

**Strongest case against.**
- The hard part is the habit, not the app. Store apps already solve the app part. Drop-off is the norm for self-improvement apps ([Reader case against](01-reader.md#similar-apps-and-the-case-against)).
- Gratitude journaling has a small effect: g = -0.29 on depression and anxiety symptoms (Cregg and Cheavens 2021 meta-analysis; negative = fewer symptoms than controls), smaller against active controls than against waitlists; the authors call it modest (unverified). It is a symptom measure, not a well-being effect size. Expect a nightly record and a moment of reflection, not a mood fix.
- Two more pings a day stack on top of the Reader's pings later.
- Kill rule (proposal): if fewer than 4 of 7 nights are logged in week 2 or week 3, stop journal work, keep the exported files, and switch to Five Minute Journal.

### How each piece works

The journal uses no LLM, so no API key question arises.

| piece | how it is achieved | iOS API / service | label |
|---|---|---|---|
| Night entry | Sheet with up to 4 short steps, one prompt per screen, saved after each step | SwiftUI | (proposal) |
| Gratitude | Three separate numbered fields; one line is enough to continue | SwiftUI text fields | (proposal) |
| Voice input | Mic key on the system keyboard; no speech code in the app | System dictation | — |
| Mood | One tap on a 5-point scale, last step, closes the sheet | SwiftUI | (proposal) |
| Morning card | Shows last night's gratitude lines, takes one "Today I will…" line, offers backfill if last night is blank | SwiftUI | (assumption) |
| Storage | One JSON file per journal date in the app's container; the export is a copy of the same file | Files on device | (proposal) |
| Reminders | One-shot local notifications for the next 7 days, rebuilt on open, save and settings change | UserNotifications, calendar triggers | (proposal) |
| Opening from a ping | The notification carries the date; a tap opens the night flow or the morning card for that date | UserNotifications response handler | (proposal) |
| Big Brain feed | Copy of the entry file, once the night is done, into a folder Big Brain reads | iCloud Drive (paid account) or share-sheet export | [decision 14](04-decisions-and-checklist.md#decisions) |
| Left out | Journal items on the [won't-build list](00-overview.md#wont-build-list); also AlarmKit, too loud for a journal | — | (proposal) |

### Screens

Mobbin links as of 2026-10-25. Pattern, reason and runner-up cells describe only what the Mobbin previews showed; "Add" items are new. The choices are (proposal).

| screen | pattern chosen | reason | runner-up | copy / avoid |
|---|---|---|---|---|
| J1 Home (today) | stoic. Today: [screen](https://mobbin.com/screens/f97f3994-4a60-4b79-96b5-ecb944e23697); open state in [flow](https://mobbin.com/flows/c301d081-140b-44a9-983a-9b891020f3c0) | Morning and evening cards side by side, one button each; a finished card collapses to a summary, so tonight's status is clear at a glance. | 5 Minute Journal empty Today: [flow](https://mobbin.com/flows/aafc7811-4fcf-4ce1-a9f9-255c587d6d4c). Simplest (greeting plus one button) but no done state for the week. | Copy: 7-day strip with a check per done night; Morning and Tonight cards, one button each; done card shows the first gratitude line. Add a date header. Avoid: quote and affirmation feeds, photo carousels, "Get inspired" cards, streak flame, time-of-day greetings that can be wrong. |
| J2 Night flow | 5 Minute Journal "Logging a journal entry": [flow](https://mobbin.com/flows/aafc7811-4fcf-4ce1-a9f9-255c587d6d4c); one-page edit view [screen](https://mobbin.com/screens/ecc4961e-6952-4c89-96b0-cf4e5f80eaeb); section toggles [screen](https://mobbin.com/screens/06e594e2-e4fe-4f71-9ef0-c6e0751d563f) | One short prompt per screen, thin progress bar, one next button, mood last with a check: least friction for a short nightly entry. Its toggles show prompts as switchable sections. | stoic. evening reflection: [flow](https://mobbin.com/flows/c301d081-140b-44a9-983a-9b891020f3c0), with voice-note step [flow](https://mobbin.com/flows/42c5dd12-ff9d-4679-b61f-b037476ea54c). Same shape; its mood word grid takes more taps than a 5-face slider. | Copy: sheet with back and close; hint line above the field; single next button; mood last; one-page view for edits; step toggles in settings. Avoid: AI replies (stoic. "SENECA", "Go deeper"), photo pickers, morning items such as affirmations, yes/no checklists (WHOOP). |
| J3 Gratitude step | Calm three-slot gratitude: [empty](https://mobbin.com/screens/0818463c-bd37-4a78-b62d-a99da2f125d6), [filled](https://mobbin.com/screens/8e228158-3a6f-4c45-878f-847e65a524d5) | Three separate lines make "3 things" explicit and map one-to-one to three stored items; "Change Prompt" varies the wording. | 5 Minute Journal gratitude step: [filled](https://mobbin.com/screens/40ad765a-7b0f-4ed5-9224-214641933266), [empty with starter chips](https://mobbin.com/screens/812c5a90-2a28-4f67-b64e-7b5b9fce7c53). Has a "get specific" nudge, but one shared field is harder to store as items. | Copy: three numbered fields with an "I'm grateful for…" placeholder; the 5 Minute Journal "be specific" hint; a prompt-shuffle button; keyboard mic for dictation. Starter chips: later. Avoid: photo attach, "Post" or share wording, character counters, one unstructured box. |
| J4a Reminder setup (first run) | ABY Journal onboarding: [screen](https://mobbin.com/screens/e4d3ab58-fc14-4324-839d-8ff0ce0e9e98); without the sheet [screen](https://mobbin.com/screens/429dcf87-343e-43c0-940b-50194974c510) | Exactly two reminders, each with a one-line purpose, a time chip and a toggle, plus a "Not now" exit. | stoic. onboarding: [screen](https://mobbin.com/screens/bf867df9-6820-4519-adca-d573fba434d1). Large editable times, but it pushes a third daytime ping. | Copy: Morning and Night cards; time chip opens a wheel in 15-minute steps; iOS permission asked only after "Set reminders" is tapped (How We Feel order: [screen](https://mobbin.com/screens/220a74a9-69cb-4786-91fc-7c0436810dcf)); "Not now" link. Avoid: a third ping, random "surprise" windows, persuasion copy such as "3x more likely". |
| J4b Morning card | Insight Timer "Reflect": [screen](https://mobbin.com/screens/0b4bcba8-6fed-49d0-8b97-b879b8631589) | A "Today I will…" card and an opt-out on one screen. The morning has one input, so one screen beats a stepped flow. Research ranked stoic. first; swapped for that reason. | stoic. morning flow: [flow](https://mobbin.com/flows/3c1b50df-db4c-4db4-ab62-8c334a0adbd6) (preview-to-screen match inferred by order). One "Begin" card into a short stepped flow with a clear end; more taps for one line. | Copy: last night's gratitude lines as a recap (ABY "Yesterday" card: [screen](https://mobbin.com/screens/a3eed6fe-5a60-4fca-96d2-5456ed222294)); one intention field; Done. Add: date, and a footer with tonight's journal time. Avoid: blurred photo backgrounds, a second mood slider, fact or story filler screens, story circles, streak end screens. |
| J5 Reminder settings | ABY Journal settings: [screen](https://mobbin.com/screens/b0e9cd74-b970-4fee-a44d-afc22a092b77); inline wheel [screen](https://mobbin.com/screens/16fa43c7-a79a-4174-b261-1ff646780a20); off state [screen](https://mobbin.com/screens/6041b16c-eb04-4329-9d34-56cfb35053e0) | Same two rows as first run, each with a time chip, inline wheel and toggle; fewest taps to change a time. | 5 Minute Journal Reminders: [screen](https://mobbin.com/screens/c34722c8-d76d-46bb-a2a0-e6e591e3e41b). Separate morning and evening rows with "Remind me at"; drop its inspiration slider. | Copy: two rows (Nightly journal, Morning reminder) with a purpose line; greyed row when off; one-line default tip (Headspace: [screen](https://mobbin.com/screens/1a9d3611-dd55-4856-a14f-b94733b72143)); a caption naming the active time zone (Linear: [screen](https://mobbin.com/screens/cf837439-e9bf-410c-bfd3-7e3ec2e98e95)). Add below: day-start hour, step toggles, export toggle, debug pending list. Avoid: "roughly around these times" delivery (stoic.), extra notification types, multi-per-day sliders. |
| J6 History | 5 Minute Journal month list: [screen](https://mobbin.com/screens/59059226-c8cc-4e04-a48f-f32331a83c20); read view [screen](https://mobbin.com/screens/e82fcc00-3986-4080-afa2-0a2d79d4293d) | Month headers, date chip, one-line preview: the smallest build and the best for rereading. | 5 Minute Journal Insights calendar: [screen](https://mobbin.com/screens/a7b893b0-00f0-432c-92e5-dafb55b53599). Shows missed nights at a glance; more work, worse for rereading. | Copy: preview = first gratitude line; tap opens the one-page read view with Edit; "New for this date" backfill (5 Minute Journal: [screen](https://mobbin.com/screens/f256508e-c9a1-4576-aea7-dc57fef60fcf)). Avoid: photo and quote cards, insight panels, filters, streak-nag copy. |

Candidates compared: J1 stoic., 5 Minute Journal, pillowtalk, Liven, Finch. J2 5 Minute Journal, stoic., Life Reset, WHOOP, pillowtalk. J3 Calm, 5 Minute Journal, Finch, Insight Timer, ABY Journal, Life Reset. J4a ABY Journal, stoic., How We Feel, Ten Percent Happier, pillowtalk. J4b Insight Timer, stoic., 5 Minute Journal, pillowtalk, ABY Journal. J5 ABY Journal, 5 Minute Journal, Visible, stoic., Headspace. J6 5 Minute Journal, stoic., Calm, Apple Health, pliability, Lapse.

### Data model

Storage (proposal): one file per journal date, `entries/<date>.json`, in the app's container. The shape is fixed by `schemas/journal-entry.schema.json`. The Big Brain export is a byte-identical copy. One format serves storage, export and script validation, with no database and no migration layer. About 365 small files a year is trivial; J-08 checks speed at 365 entries. Downside: deleting the app deletes the entries unless export is on.

**JournalEntry** (one per journal date)

| field | type | rule |
|---|---|---|
| schemaVersion | integer | 1 |
| date | date, YYYY-MM-DD | Journal date and key; one file per date |
| timeZone | IANA zone name | Zone at first save, e.g. America/Los_Angeles |
| intention | text, optional | Morning "Today I will…" line for this date |
| gratitude | list of 0-3 non-empty texts | The night counts as done when this has at least 1 line; empty in a morning-only file |
| bestMoment | text, optional | Step 2 |
| lesson | text, optional | Step 3 |
| mood | integer 1-5, optional | Step 4 |
| nightStartedAt, nightCompletedAt | timestamps, optional | Flow open and last step saved; used for the time check |
| morningCompletedAt | timestamp, optional | Set when the intention is saved |
| backfilled | true or false | True if the night part was first saved after its journal date ended |
| updatedAt | timestamp | Every save; ISO 8601 with offset, like all timestamps |

**JournalSettings** (on device, not exported): nightly time and on/off, morning time and on/off, day-start hour, time-zone rule, step toggles, export on/off and folder. Defaults live in [time defaults](#morning-reminder-and-time-defaults).

**Reminder plan** (derived, never stored): a list of dated requests with ids `night-<date>` and `morning-<date>`, built from settings plus entries.

### Nightly flow and gratitude step

Gratitude is step 1 of the night flow, as the brief asks. Five Minute Journal keeps it in the morning (J2 section toggles in [screens](#screens)); here the morning card echoes last night's lines instead, so the morning gets gratitude without a second writing task.

Flow design is (proposal). Steps 2-4 are additions to the brief: [decision 10](04-decisions-and-checklist.md#decisions).

| step | content | input | rule |
|---|---|---|---|
| 1 Gratitude | Gratitude prompt, 3 numbered fields, "be specific" hint | Typing or keyboard dictation | Next unlocks at 1 non-empty line; 3 slots shown, none forced; cannot be switched off |
| 2 Best moment | One line on the best part of today | Typing | Skip allowed; can be off in settings |
| 3 Lesson | One thing learned or to change; this morning's intention shown above the field | Typing | Skip allowed; can be off |
| 4 Mood | 5 faces, one tap | Tap | The tap saves and closes; if off, step 3 closes the flow |

Rules:
- Each Next saves. The night counts as done once step 1 is saved; closing the sheet after that keeps the entry.
- On done: tonight's nightly request is removed, the reminder plan is rebuilt, and the export file is written if export is on.
- Target: the full default flow takes 5 taps beyond typing (Begin, Next, Next or Skip, Next or Skip, mood) and about 3 minutes, measured from nightStartedAt to nightCompletedAt.
- Journal date: an entry started before the day-start hour belongs to the previous date, so a 12:40 am entry counts for last night.
- Missed night: no second ping. The morning card offers "Add last night"; history offers "New for this date" for older nights. Both save with backfilled = true.
- Edits: from the history read view; they rewrite the same file and the export.
- Later, not v1: voice notes with audio and transcript, using the stoic. voice-note pattern (J2 runner-up) and the Shared speech module the Reader builds.

### Morning reminder and time defaults

Purpose (assumption): the brief asks for "a reminder in the morning" and does not say what for. Reading chosen: the ping opens the morning card (J4b), which shows last night's gratitude lines and takes one "Today I will…" line. If last night is blank, the card offers "Add last night" first. Other readings: a second journal session in the morning, or a plain nudge to journal tonight. [Decision 12](04-decisions-and-checklist.md#decisions).

Time defaults (proposal), [decision 11](04-decisions-and-checklist.md#decisions):

| setting | default | how to change |
|---|---|---|
| Nightly journal reminder | 10:00 pm Pacific, on | J5 time chip, 15-minute steps; toggle |
| Morning reminder | 7:15 am Pacific, on | J5 time chip, 15-minute steps; toggle |
| Journal day starts | 4:00 am | J5 stepper, midnight to 6:00 am |
| Time zone | Wall-clock times, Pacific at home | Travel rule: [decision 13](04-decisions-and-checklist.md#decisions); default follows the phone's zone, so 10:00 pm stays 10:00 pm local |

Why these times: 10:00 pm comes after the day's events and before sleep. If a Sleep Focus starts earlier, move the nightly time before it ([notifications](#notifications)). 7:15 am lands after waking, before the workday and 15 minutes before the Reader's first slot (07:30, [schedule](01-reader.md#schedule-and-spacing)), so the two apps do not ping at the same minute. Both are guesses about the routine; each takes one tap to change.

Big Brain timing: if the digest runs before the night entry is saved, that entry lands in the next day's digest. The digest run time is (to verify).

### Notifications

Design (proposal):
- Local notifications only. No server, no push, no time-sensitive level.
- One-shot requests, not repeating ones: `night-<date>` for each of the next 7 night times after now and `morning-<date>` for each of the next 7 morning times, minus skipped dates. At most 14 pending, within the per-app cap ([iOS limits](01-reader.md#ios-limits-that-block-a-piece)).
- Why one-shot: a repeating daily trigger would still fire after tonight's entry is done. A dated request can be removed by id the moment the entry is saved.
- Rebuild on: app launch or foreground, entry save, settings change, time-zone change. Never from background refresh: BGTaskScheduler has no exact timing, and a top-up would undo the 7-day lapse stop below.
- Rebuild method: remove the journal's own ids, then add. Do not remove all and re-add at once: a forum report says removal runs asynchronously and can wipe requests added right after (to verify).
- Skips: no night request for a date already done; no morning request for a date whose intention is saved.
- Content: no entry text on the lock screen. The morning text depends on whether last night is done when the plan is built; saving the night rebuilds it.
- Tap: opens the night flow for that date, or the morning card.
- Lapse: if the app is not opened for 7 days, pings stop after day 7. The last scheduled morning ping says so.
- Permission: asked only after "Set reminders" on J4a. If denied, home shows a one-line banner with a link to iOS notification settings.
- Focus: a Sleep Focus may hold back the nightly ping. Allow the Journal app in that Focus, or set the nightly time before it starts (to verify on device).
- No snooze and no second nag for a missed night.

### Reuse and to verify

| item | what the journal does with it | status |
|---|---|---|
| Shared package | J-01 creates the workspace and the package. The journal adds Storage (J-03), Notifications (J-06) and Export (J-09). The Reader adds Speech, LLM, Gap and Scheduling modules later and reuses the rest. | (proposal); the Reader links here |
| Jeet notification code | Read the R-01 report ([Reader tasks](01-reader.md#task-table)) before J-06. Adopt its scheduler only if it already does dated one-shot requests. | (to verify) |
| Big Brain input folder | Receives one JSON file per night | (to verify): folder path, whether the digest can read a second input besides the transcript, digest run time |
| iCloud Drive folder | Carries the export to the Mac | Needs the paid account ([iOS limits](01-reader.md#ios-limits-that-block-a-piece), [decision 6](04-decisions-and-checklist.md#decisions)); the Mac-side path is (to verify) |
| Recorder, ClawCall | Not used by the journal | — |
| Presently | Ideas only; no license, so no code | — |
| Device and iOS target | Needs no Apple Intelligence and no SpeechAnalyzer; takes the deployment target J-01 sets | [decision 7](04-decisions-and-checklist.md#decisions) |

#### Feeding Big Brain
- Default (proposal): when the night is done, write the entry file to `journal/<date>.json` in the shared folder; later saves for that date overwrite it. A morning-only file stays on the phone until the night is done. Nothing else leaves the phone. [Decision 14](04-decisions-and-checklist.md#decisions).
- Loop rules (optional file, missing file = no entry, no Big Brain code changes): [how the apps feed the loop](00-overview.md#how-the-apps-fit-around-big-brain). J-09 writes `docs/big-brain-journal-intake.md` (folder, file name, schema, update rule) for that Big Brain session.
- Without the paid account: an Export button sends the files through the share sheet (proposal).

## Task table

J-01 starts now, in parallel with the digest test ([decision 1](04-decisions-and-checklist.md#decisions)); only J-09's hookup to the Big Brain folder waits for the Phase 0 gate. Paths are relative to the new repo root ([decision 9](04-decisions-and-checklist.md#decisions)). The Shared-package tasks here are the base the Reader builds on. Lint runs as a script (J-10) before any repair pass.

| id | files | done-check |
|---|---|---|
| J-01 Workspace and Shared package scaffold · Sonnet | `mundi-apps.xcworkspace`, `Journal/Journal.xcodeproj`, `Shared/Package.swift`, `scripts/build-all.sh`, `docs/repo-layout.md` | You create the empty Journal app in Xcode once, inside the `~/Projects/mundi-apps` folder R-01 made, and set the signing team by hand: your free Personal Team now, then the paid team once Apple activates it ([decision 6](04-decisions-and-checklist.md#decisions)); after the switch the app reinstalls and its profile is no longer the 7-day one (new expiry to verify). `git init` keeps R-01's `docs/jeet-reuse-report.md`. The iPhone reports iOS 26 or later in Settings > General > About (update it first if not; [decision 7](04-decisions-and-checklist.md#decisions)). Then `scripts/build-all.sh` builds Journal and Shared with 0 errors; the app runs on the iPhone and shows an empty home; `Shared/Package.swift` declares Storage, Notifications and Export targets at `Shared/<Module>/` (no `Shared/Sources/`); deployment target iOS 26.0, as the Reader's APIs need (proposal); `docs/repo-layout.md` lists the module folders and the slot for the Reader target. |
| J-02 Entry schema, fixtures, check scripts · Haiku | `schemas/journal-entry.schema.json`, `tests/fixtures/journal/`, `scripts/validate-journal.py`, `scripts/make-fixtures.py`, `scripts/check-repo.sh` | Validator accepts 5 valid fixtures, one of them morning-only (intention, no gratitude), and rejects 5 invalid ones (an empty-string gratitude line, 4 lines, mood 6, bad date, missing schemaVersion) with exit 1 each; `make-fixtures.py` writes 365 valid entries; `check-repo.sh` runs fixture validation, the API-key search, `scripts/build-all.sh` and the unit tests, and exits non-zero on any failure (a planted fake API key gives 1; removing it gives 0). |
| J-03 Entry store and journal date · Sonnet | `Shared/Storage/JSONFileStore.swift`, `Journal/Model/JournalEntry.swift`, `Journal/Model/JournalDate.swift`, `tests/JournalTests/EntryStoreTests.swift` | Tests pass: 30 fixtures round-trip byte-identical; entries started at 12:30 am and 3:59 am get the previous date, 4:00 am gets the new date; every saved file passes `scripts/validate-journal.py`. |
| J-04 Night flow and gratitude step (J2, J3) · Sonnet | `Journal/Views/NightFlowView.swift`, `Journal/Views/GratitudeStepView.swift`, `Journal/Views/MoodStepView.swift`, `Journal/Resources/gratitude-hints.json` | On the iPhone: Next on gratitude stays disabled until 1 line is typed; the full default flow takes 5 taps beyond typing; closing after step 1 keeps the entry and marks the night done; keyboard dictation fills a field; a step switched off in settings disappears from the flow. |
| J-05 Home and morning card (J1, J4b) · Sonnet | `Journal/Views/HomeView.swift`, `Journal/Views/MorningCardView.swift` | Home shows the date, a 7-day strip with a check on each done night, and Tonight and Morning cards; the morning card lists last night's gratitude lines, or "Add last night" when blank, which saves with yesterday's date and backfilled = true; the intention saved in the morning shows on that night's lesson step. |
| J-06 Rolling reminder scheduler · Sonnet | `Shared/Notifications/RollingScheduler.swift`, `Journal/Reminders/JournalReminderPlan.swift`, `Journal/Views/DebugPendingView.swift`, `tests/JournalTests/ReminderPlanTests.swift` | Tests with the clock fixed at 21:00, tonight open: the plan has 14 requests with ids `night-<date>` and `morning-<date>`; tonight done gives 13 and no `night-<today>`; nightly off gives 7; building twice gives identical ids. On the iPhone: a ping set 2 minutes ahead fires once and opens the right screen; finishing the entry first means nothing fires; the debug list never shows more than 14. |
| J-07 Reminder setup and settings (J4a, J5) · Sonnet | `Journal/Views/ReminderSetupView.swift`, `Journal/Views/SettingsView.swift`, `Journal/Model/JournalSettings.swift` | First launch shows 10:00 pm and 7:15 am; the iOS permission prompt appears only after "Set reminders"; "Not now" leaves both off and shows the home banner; moving the nightly time to 9:15 pm shows 7 night requests at 9:15 pm in the debug list with morning ones unchanged; switching a row off removes its 7 requests. |
| J-08 History and read/edit view (J6) · Sonnet | `Journal/Views/HistoryView.swift`, `Journal/Views/EntryDetailView.swift` | With 365 fixture entries: the list opens in under 1 s, grouped by month, newest first, preview = first gratitude line; Edit changes a line and updatedAt; "New for this date" on an empty past date opens the flow and saves with backfilled = true. |
| J-09 Folder exporter and Big Brain export · Sonnet | `Shared/Export/FolderExporter.swift`, `tests/SharedTests/FolderExporterTests.swift`, `Journal/Export/JournalExport.swift`, `docs/big-brain-journal-intake.md` | Always, since R-33 needs it: exporter tests pass (writes a file to a given folder, overwrites by name, reports a missing folder). The rest only if [decision 14](04-decisions-and-checklist.md#decisions) is yes: with export on, finishing the night writes `journal/<date>.json` that passes the validator; a morning-only save writes nothing; an edit rewrites the same file (file count unchanged); with export off, no file is written; the file reaches the Mac folder Big Brain reads before the next digest run; no Big Brain file is changed. |
| J-10 Pre-repair lint run · script | `scripts/check-repo.sh`, `scripts/build-all.sh`, `docs/journal-lint.txt` | Runs `scripts/check-repo.sh` (J-02); writes results to `docs/journal-lint.txt`; exits 0, or lists failures for J-11 to fix. Runs before every repair pass. |
| J-11 Week-1 use and fix pass · Sonnet | `docs/journal-week1.md`; fixes only in `Journal/` and `Shared/` | 7 nights in a row logged on the phone; you note nightly in the doc that each open night got exactly one ping and none came after a finished entry; median nightStartedAt to nightCompletedAt at most 4 minutes; every issue in the doc fixed or moved to later; the doc lists every changed file; `scripts/check-repo.sh` still exits 0. |

## Checks

End-to-end, on the iPhone, after J-11. Use the debug pending list (J-06) wherever a count is needed.

| id | do this | pass when |
|---|---|---|
| C1 | Let the nightly ping fire, tap it, enter one gratitude line and a mood | Entry saved for today; `night-<today>` gone from the pending list; tomorrow's morning request still there |
| C2 | Set the nightly time 5 minutes ahead and finish the entry before then | No notification fires |
| C3 | Skip a night entirely | No second ping that night; next morning the card offers "Add last night"; the saved entry has yesterday's date and backfilled = true |
| C4 | Open the morning ping after a logged night | Card shows last night's lines; the intention saves; that night's lesson step shows it |
| C5 | Start an entry at 12:40 am | It is filed under the previous date |
| C6 | Change the phone's time zone, then open the app | Pending requests follow the [decision 13](04-decisions-and-checklist.md#decisions) rule; count unchanged |
| C7 | Do not open the app for 3 days | A ping still arrives each night and morning; on open, pending is back to 14 minus done slots |
| C8 | Look at the lock screen when a ping arrives | No entry text shown |
| C9 | Reinstall the app from Xcode without deleting it | All entries and settings still there; pending count rebuilt on first open |
| C10 | Finish a night with export on (only if [decision 14](04-decisions-and-checklist.md#decisions) is yes) | File appears in the Big Brain folder and passes `scripts/validate-journal.py`; whether the digest uses it is checked on the Big Brain side |
| C11 | Weeks 2 and 3 of normal use | At least 4 of 7 nights logged each week; otherwise apply the kill rule in [case against](#similar-apps-and-the-case-against) |
