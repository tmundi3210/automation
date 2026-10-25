# Later phases: Parts 2-6 and the rest of Part 7

## Part cards

Brief cards. Each part is planned only far enough to decide whether to build it. Proposed order (proposal), [decision 25](04-decisions-and-checklist.md#decisions): Part 3 Mind, Part 2 Self-tracking, Part 4 Voice, Part 7 rest, Part 5 Body language, Part 6 Clothes. Hours per phase live in [phases](00-overview.md#how-the-apps-fit-around-big-brain).

Rules for every later slot (proposal):
- One part at a time. A slot starts only when the previous slot's gate has passed, the Big Brain stop rule is not active, and the owner says go.
- Where a gate names a "use X instead" trial, the trial runs for two weeks first, and the build covers only what the app leaves out.
- Turn counts in the cards are estimates (proposal); [budget](00-overview.md#budget) prices them.
- Hosting, [decision 26](04-decisions-and-checklist.md#decisions): learning content ships as packs inside the Reader app; the drills of Parts 4 and 5 share one new Practice app; Part 2 gets a Tracker app; Part 6 runs on the Mac with no iOS app. The `Practice/` and `Tracker/` folders extend the layout of [decision 7](04-decisions-and-checklist.md#decisions).
- Study facts in these cards come from search summaries of the cited papers, not the papers. A pack may teach a number only after its primary source has been re-read (to verify). L-01 enforces the re-read date.

Gate tasks: only the rows that decide whether a slot starts. Build tasks are written when a slot is approved. Thresholds are (proposal).

| id | files | done-check |
|---|---|---|
| L-01 Pack claims block and checker · Haiku | `schemas/book-package.schema.json`, `scripts/validate-package.py`, `scripts/myth-phrases.txt`, `tests/fixtures/packs/` | Exits 0 on 3 valid fixture packs and on the Reader pilot package (books are unchanged). Exits 1, naming the claim, on each planted fault: a claim with no source, no status (holds, partly, disputed) or no primary-source re-read date; a listed myth phrase such as "reptilian brain" or "7-38-55" outside a card marked as a myth. Runs before every pack repair pass. Gate for Parts 3, 5 and 7. |
| L-02 Self-tracking trial gap list · Haiku | `docs/self-tracking-trial.md` | After 14 days of Exist, Arc Timeline and Timing or ActivityWatch: one row per Part 2 item (location, steps, other activity, sleep, sleep and wake times, screen time, web history, camera) marked covered, partly or not covered, plus whether its data reaches the Big Brain folder. Part 2 build starts only if at least 1 wanted item is not covered. |
| L-03 Screen Time and web-domain export spike · Sonnet | `Tracker/Spike/`, `docs/screen-time-spike.md` | On the iPhone, dev build, in California: the iOS 26.4 export returns yesterday's per-app usage and at least 1 visited web domain, or the doc records the exact error. The doc also states whether Apple's Screen Time pane kept updating over 3 days. |
| L-04 Pace meter accuracy · Sonnet | `Shared/Speech/PaceMeter.swift`, `tests/fixtures/pace/`, `scripts/score-pace.py`, `docs/pace-meter-check.md` | Five recordings of one 300-word passage at slow, normal and fast pace: in all 5, the meter's session average is within 10% of words ÷ elapsed time from the script; a stopwatch check shows the gauge reacting within 2 s of a pace change. |
| L-05 Photo set check · Haiku | `scripts/check-photo-set.py`, `mac/clothes/labels.csv` | Exits 0 only when `mac/clothes/photos/` holds 15-30 photos, at most 3 labelled full body, each with an expression label, none flagged sunglasses or hand on face; prints counts per label; exits 1 on a planted 4th full-body photo. |
| L-06 Clothes privacy check · Sonnet | `docs/clothes-privacy.md` | Names the exact trainer; quotes the terms text that places it inside or outside fal's carve-outs, with the date read; records payload storage as off; the owner says go on this doc before any photo is uploaded. |

### Part 2. Self-tracking

**Goal.** One file per day of where I was, how I moved and slept, and how I used screens, for Big Brain to read ([decision 12](04-decisions-and-checklist.md#decisions)). "Precise location, the app sends it" is read as: my own app logs location and sends it to my Mac, with no Find My and no other person involved (assumption). Call history and Instagram stay out.

**Smallest useful version.** Step 0 builds nothing: run Exist, Arc Timeline and Timing or ActivityWatch for 14 days (L-02). Then build only the gaps, in a Tracker app that writes `tracking/<date>.json` to the Big Brain folder (proposal):
- Location: visit and significant-change monitoring with precise permission. A continuous trail only when switched on ([decision 28](04-decisions-and-checklist.md#decisions)).
- HealthKit: steps, workouts and other activity, sleep stages, sleep time and wake time. Sleep time = start of the night's first asleep sample; wake time = end of its last (proposal).
- Later: screen time and web domains if L-03 passes; Mac-side web history from ActivityWatch or Timing through a `mac/tracking/` export; camera only if [decision 29](04-decisions-and-checklist.md#decisions) says yes.

**Reuse.** The journal's export path and its rule that a missing file means no data ([feeding Big Brain](02-journal.md#feeding-big-brain)); `Shared/Export/` and `Shared/Storage/`; an intake doc for Big Brain like J-09's. Nothing from the Reader engine. Whether Exist, Arc Timeline and ActivityWatch exports can reach the Big Brain folder is (to verify).

**iOS limits and workaround.**

| item | limit | nearest workable alternative |
|---|---|---|
| Location | Readable. Background needs Always permission. Visits arrive late and may lack arrival or departure times. Significant change fires after 500 m or more, at most every 5 minutes. Background time for an upload is undocumented; behaviour after a force-quit was not checked. | Log on the device; write the day file on the next background wake or app open (proposal). |
| HealthKit | Readable on the free account. Background delivery needs its entitlement, and steps wake the app at most hourly. The store is encrypted while the phone is locked, so background reads can fail. A denied read looks like no data. | Build yesterday's summary on the first wake with the phone unlocked, or on open; flag a 0-step day as "check permission" (proposal). |
| Screen time | Blocked for export. The report extension is display-only and cannot send data out. The iOS 26.4 export works for customer installs only in the EU; develop-and-test is allowed in any region. Needs the paid account ([decision 4](04-decisions-and-checklist.md#decisions)). A forum report says Apple's own Screen Time pane stops updating while an app holds the data. | L-03 spike on my own dev build (to verify). Else Timing's reported import of iPhone Screen Time (to verify). Else view it in Settings, unexported. All (proposal). |
| Website history | Blocked. No iOS API reads Safari history. | Mac: ActivityWatch (free, local, Chrome, Firefox and Edge extensions) or Timing (website domains); Safari's Mac history is protected and Full Disk Access is the reported way in (to verify). iPhone: the iOS 26.4 visited-domains list (domains only, no timestamps, same EU rule) via L-03, or a Safari Web Extension that logs tab URLs to a shared app file (to verify). All (proposal). |
| Camera / vision | Blocked in the background. Capture stops when the app leaves the screen; one app uses the camera at a time; capture shuts down under heat; no battery or heat figures were found. | A phone on a stand runs a foreground app at a low frame rate and keeps only on-device Vision labels (at desk, away), never video (proposal). Battery, heat and privacy cost are assumed heavy until a 1-day test measures them. The spare iPhone already runs the Recorder ([decision 6](04-decisions-and-checklist.md#decisions)), so this needs another phone. For "how well I work", Mac-side ActivityWatch or Timing is the nearest signal without a camera. |

**Case against.** Use Exist + Arc Timeline + Timing or ActivityWatch instead. Exist pulls Apple Health and adds mood logging with correlations (paid subscription), Arc Timeline logs places and trips with an automated daily JSON export, and Timing or ActivityWatch (free) cover Mac apps and websites. No build. Cost to still build: a third app, Always-location and Health permissions, background-delivery debugging, about 14 Sonnet turns including the spike. This is the most blocked part: three of five items are blocked or region-gated. Data with no question attached is noise; Big Brain needs a use for steps and sleep before collection starts. A room camera also records anyone who walks in.

**Phase.** Later slot 2 of 6 (proposal), [decision 25](04-decisions-and-checklist.md#decisions). Gate: L-02 lists at least one wanted item no app covers; L-03 settles screen time and web domains before those rows are built. Why second: small, feeds the loop directly, mostly "use X instead". It moves to slot 1 if Big Brain wants this data sooner. Hours: [phases](00-overview.md#how-the-apps-fit-around-big-brain).

### Part 3. Mind

**Goal.** Learn how the mind works in the Reader's flow (brief, ask what I know, add more, deepen), teaching only claims that hold up and flagging the rest.

**Smallest useful version.** Six packs inside the Reader app plus a meditation hand-off ([decision 26](04-decisions-and-checklist.md#decisions)). Each pack is scheduled, gated and retold like a book. Every claim carries a badge: holds, partly or disputed (proposal).
- **Mental models.** Inversion, second-order effects, base rates, opportunity cost, map vs territory, circle of competence (proposal), each with one daily-life example. Taught as thinking tools, not findings.
- **Biases, with examples and stories.** Confirmation, anchoring, availability, loss aversion, sunk cost, hindsight (proposal), each with a story and a "spot it today" prompt. Perception story: blind men and the elephant (assumption), paired with the invisible gorilla experiment, where about half of viewers missed the gorilla (Simons and Chabris 1999). Other candidates: Plato's cave (runner-up), The Dress (2015), Rashomon. [Decision 27](04-decisions-and-checklist.md#decisions). Dunning-Kruger is marked disputed: random data reproduce the pattern. The Müller-Lyer "carpentered world" culture story is left out; a 2025 review challenged it.
- **Anger and regaining control.** Teach: arousal spikes fast, and the prefrontal cortex can turn it down. Tricks in evidence order: 1) lower arousal: slow breathing or cyclic sighing, step away; 2) name the feeling; 3) reappraise once calmer; 4) do not vent or ruminate. The brief's "brainstem and limbic system take over" is the triune-brain model, which neurobiologists discredit (Cesario et al. 2020); "amygdala hijack" is a pop simplification. Evidence: arousal-lowering reduced anger and venting did not, across 154 studies (Kjærvik and Bushman 2024). Naming a feeling lowered amygdala response in fMRI, tested with negative images, not anger (Lieberman 2007). Reappraisal works overall, with mixed results for anger. Cyclic sighing was tested as 5 minutes a day for mood, not as an in-the-moment fix. "Count to 10" was never tested directly.
- **Brain networks.** Default mode network (DMN) vs task-positive network, and what meditation and exercise do to them. Their opposition is real but its size is debated, because a common analysis step inflates it. Meditation: low-moderate evidence; Brewer 2011 compared experts with novices at one time point, and meta-analyses disagree on direction. Exercise: low evidence; one walking trial in older adults changed DMN links at 12 months, not at 6, and a 2025 review found results inconsistent.
- **Guided meditation.** No player in v1. The brain-networks pack ends with a hand-off to a meditation app (proposal). The app the owner meant is unknown, so no guess is made. Candidates, all (unverified) as the app meant. Seen in Mobbin meditation screens: [Insight Timer](https://mobbin.com/screens/ce233a53-df97-4728-9f14-9cab30bc9d4c), [Calm](https://mobbin.com/screens/6c340b3f-2d75-4bb9-aeba-0583bed57f5d), [Mindvalley](https://mobbin.com/screens/36cf6b31-defc-4f4a-b5a2-42b1913b4a39), [Breathwrk](https://mobbin.com/screens/a79fe461-e5eb-4087-af57-2c742e665b4e), [TIDE](https://mobbin.com/screens/aa5ee839-9bca-4096-882c-07e6feb24cd2), [Oura](https://mobbin.com/screens/3b2948a4-734e-4c08-a403-2c6ae0d5d1f7), [Opal](https://mobbin.com/screens/8d175bb7-478a-4f0d-ac1e-3d1ee980c62b), [Life Reset](https://mobbin.com/screens/a358e391-9178-4928-aa62-2b05c20e0c19), [Moonly](https://mobbin.com/screens/a2630565-db77-4a6b-9669-d39b2fd2d1e0), [Alan](https://mobbin.com/screens/e11fcc45-b238-43f6-ad35-1dc6040d153a), [Me+](https://mobbin.com/screens/a5eecade-e8f5-4ef0-9d3f-d3934a82a33f), [Liven](https://mobbin.com/screens/0b1c8e6d-b2a5-457f-a7b5-8a2d86e89fa5), [Noom](https://mobbin.com/screens/2e5a8e30-2953-42de-b84e-14fef814e161), [WHOOP](https://mobbin.com/screens/4a0ab5f9-c2d1-4e16-9842-faa4ea86ec60). Known from web search: Headspace, Waking Up, Balance, Medito, Healthy Minds Program, Happier (called Ten Percent Happier until September 2024). [Decision 31](04-decisions-and-checklist.md#decisions), asked last.
- **Probability and statistics.** Worked examples plus on-device simulations (proposal): coin-flip streaks, the hot hand, base rates in a medical test, Monty Hall, the birthday problem, regression to the mean. Seeded and deterministic, no LLM. Hot hand: Miller and Sanjurjo 2018 found a selection bias that reverses the classic "hot hand fallacy" result; the simulation shows that bias.
- **Psychology experiments.** Real experiments only, each with its badge. Holds: Asch conformity (varies by culture), Loftus misinformation. Partly: Milgram (behaviour replicated, meaning contested), the marshmallow test (link to later achievement about half the original size, and smaller again after family-background controls), facial feedback ([Part 5](#part-5-body-language-and-conversation)). Disputed or failed: Stanford prison (guards were instructed and knew the expected result), ego depletion (multi-lab d = 0.04 and 0.06), power posing ([Part 5](#part-5-body-language-and-conversation)), social priming (Thinking, Fast and Slow ch. 4; Kahneman conceded).
- Later: a plain timer with bells if no meditation app is chosen (proposal); packs from books the owner adds.

**Reuse.** The whole Reader engine: book package and [pipeline](01-reader.md#pdf-to-levels-pipeline), [spacing](01-reader.md#schedule-and-spacing), [gate and test-out](01-reader.md#no-skip-gate-and-test-out), [dictate-back](01-reader.md#dictate-back-loop) with [scoring](01-reader.md#recall-scoring-gap-and-root-cause), [talk about the text](01-reader.md#talk-about-the-text) and [notifications](01-reader.md#notifications-and-calls). Same LLM split as the Reader ([decision 14](04-decisions-and-checklist.md#decisions)): packs are written once in an interactive Sonnet session on the Mac with `prompts/pack-authoring.txt`, then linted like a book; scoring and chat use the Reader's runtime model. That model is small and weak at reasoning, so Mind chat answers only from the pack in view and shows each claim's badge. New code: the claims block (L-01), the badge, a simulation card type in `Reader/Views/Simulations/`.

**iOS limits and workaround.** None new. The Reader's limits and workarounds apply ([iOS limits](01-reader.md#ios-limits-that-block-a-piece)). Meditation audio stays in the chosen app, so there is no background-audio work.

**Case against.** Use Headspace, Insight Timer or the app named in [decision 31](04-decisions-and-checklist.md#decisions) for meditation. For the rest, the Reader can take the source books as they are, with zero new code, but it would teach failed chapters such as priming with no flag. This part is mostly content, not software, and its quality rests on authoring. An LLM draft can repeat exactly the myths this part must correct, which is why L-01 blocks myth phrases and unsourced claims. Knowing a bias's name does not reliably stop it in your own decisions (unverified). Cost to still build: about 26 Sonnet turns, mostly pack writing and the simulations.

**Phase.** Later slot 1 of 6 (proposal), [decision 25](04-decisions-and-checklist.md#decisions). Gate: the Reader [checks](01-reader.md#checks) pass, including the scorer check set; L-01 passes. Why first: highest reuse, no new permissions, little new code. Hours: [phases](00-overview.md#how-the-apps-fit-around-big-brain).

### Part 4. Voice and speech

**Goal.** Clearer, better-paced spoken English and better storytelling. Build only what Orai, BoldVoice and ELSA leave out.

**Smallest useful version.** A Practice app ([decision 26](04-decisions-and-checklist.md#decisions)) with a routine of about 10 minutes (proposal), plus two Reader packs:
- Warm-up: humming on low, middle and high tones. The app plays each reference tone with a timer; no pitch scoring in v1 (proposal). Counting 1-2-3 is read as counting aloud on one breath at an even pace, then slower and faster, with the meter on (assumption).
- Pen-between-the-teeth drill: word lists grouped by vowel movement (front, back, rounded, gliding) (proposal), read with the pen, then without. No trial supports the drill and related oral-motor drills lack support; the screen labels it "unproven". At best it is a tongue-position prop.
- Live speaking-rate meter: words per minute over a rolling window from live transcription. A gauge with a target band shows "slow down" or "speed up". Default band 140-160 words per minute for talks, a range secondary sources suggest; adjustable.
- After-session analysis: words per minute over time, pause count and length, filler-word count from a fixed word list. Then notes and 2-3 exercises from the runtime model (`prompts/speech-session-notes.txt`), shown as suggestions.
- Storytelling: a short story with marked pause points; the meter shows each pause's length. Emotion: read one line in three emotions and play it back for self-review. No automatic emotion scoring.
- Grammar and sentence building, spoken and written (Reader pack): build a sentence from 2-3 given words, say or type it, get one correction (`prompts/grammar-feedback.txt`).
- Common new words (Reader pack), read as common English words that are new to me (assumption). The owner adds words; each is a card on the Reader's spacing and passes when used correctly in a spoken sentence.
- California/US accent: use BoldVoice or ELSA. The app links out and logs minutes. No own accent engine.

**Reuse.** The Shared speech module the Reader builds (`Shared/Speech/`, [decision 17](04-decisions-and-checklist.md#decisions), [speech-to-text choice](01-reader.md#speech-to-text-choice)); the Reader engine for the grammar and word packs; the runtime model of [decision 14](04-decisions-and-checklist.md#decisions); the Reader's check-set method ([scoring](01-reader.md#recall-scoring-gap-and-root-cause)) to test grammar feedback before trusting it.

**iOS limits and workaround.** Live transcription has the device limits in [speech-to-text choice](01-reader.md#speech-to-text-choice) ([decision 5](04-decisions-and-checklist.md#decisions)); English is all this part needs. The mic runs only while a session is on screen. Meter accuracy depends on transcription, so L-04 measures it before anything else is built on it. Pitch detection was not researched, so humming has no scoring in v1 (proposal).

**Case against.** Use Orai instead: pace, energy, clarity and filler-word feedback on iOS. Use BoldVoice or ELSA for the accent (AI-scored, paid). Yoodli does live pace feedback on Mac and web, with no iOS app. The evidence is weak: semi-occluded vocal tract exercises help in voice therapy, but warm-up evidence for healthy speakers is thin and humming was not named in the sources; the pen drill has none; counting was not checked. The build-worthy gap is the owner's own routine, which is a checklist more than an app. Cost to still build: the Practice app, meter and report, about 19 Sonnet turns.

**Phase.** Later slot 3 of 6 (proposal), [decision 25](04-decisions-and-checklist.md#decisions). Gate: two weeks of Orai plus one free BoldVoice or ELSA lesson, then build only what they leave out; Reader dictate-back works on the phone; L-04 passes before the report is built. Why third: reuses the speech module and meets no new iOS block. Hours: [phases](00-overview.md#how-the-apps-fit-around-big-brain).

### Part 5. Body language and conversation

**Goal.** Body language and conversation habits that fit real situations, rehearsed by repetition, with the popular myths flagged.

**Smallest useful version.** Lessons as one Reader pack; drills in the Practice app ([decision 26](04-decisions-and-checklist.md#decisions)):
- Situational body language: cards for "assertive yet calm" and "open", per situation: meeting someone, disagreeing, listening, presenting (proposal). Each card covers posture, eye contact, hands and voice.
- Expressing emotion through the body: one card per emotion. Posed smiles can start or slightly lift happiness (Many Smiles 2022); scowling started anger but did not amplify it. The effects are small; the original pen-in-mouth result failed a 17-lab replication.
- Starting a conversation: openers by setting, plus small tips.
- Myths card: power posing had no hormone or risk-taking effect (Ranehill 2015, n = 200, against an original n = 42), leaving only a small "felt power" effect; Mehrabian's 7-38-55 applies only to feelings when words clash with tone or face; people read lies from body language at about 54%, near chance, and professionals do no better.
- Opening-up exercises (Practice): timed step cards (proposal).
- Walking simulation (Practice): model walk and bold walk as step-by-step cards (posture, gaze, stride, arm swing) with a metronome for cadence. "Simulation" is read as a guided step sequence, not a 3D figure (assumption).
- Repeat-phrase drill (Practice): a phrase on screen, the front camera as a mirror; say it aloud with the cued posture and a smile. Live transcription confirms the phrase was said. Nothing is recorded or kept (proposal).
- Quick-read tips page (Practice): one screen, 5-7 lines, offline, one tap from home.
- Later: self-video review of the walk; smile detection with Vision (to verify).

**Reuse.** The Reader engine for the pack; the Practice app, speech module and drill player from [Part 4](#part-4-voice-and-speech); the L-01 checker.

**iOS limits and workaround.** The camera works in the foreground only and for one app at a time, so the mirror runs only while a drill is on screen. Nothing else is blocked.

**Case against.** Body language improves in live conversations, not on a phone; drills are rehearsal at best. Much popular content is myth (the myths card). What holds is thin: upright posture under stress rests on one trial (n = 74, no independent replication found); expansive posture got about 27% more "yes" on a dating app but can read as arrogant; thin slices show people judge fast from brief behaviour, which is a correlation with ratings, not a method. Evidence for walking drills and conversation openers was not checked. Use instead: Alison's free "The Science of Body Language" course (about 3 hours) or the "Psychology Of Body Language" iPad app; quality of both not assessed. Cost to still build: about 9 Sonnet turns on top of Part 4.

**Phase.** Later slot 5 of 6 (proposal), [decision 25](04-decisions-and-checklist.md#decisions). Gate: the Alison course is finished and its gaps listed; Part 4's Practice app and L-04 are done; L-01 passes. Why fifth: depends on Part 4 and has the weakest evidence. Hours: [phases](00-overview.md#how-the-apps-fit-around-big-brain).

### Part 6. Clothes

**Goal.** Know what suits me and see it on my own photos before buying. Not trend chasing.

**Smallest useful version.** Mac only, no iOS app in v1 (proposal):
1. What suits me: a short questionnaire (build, height, colouring, lifestyle, dress code, fit likes and dislikes) goes to an interactive Sonnet session, which writes `docs/style-brief.md` with fit rules, a colour palette and a core list of about 10 items (`prompts/style-brief.txt`). No trend sources, no Instagram. Colour rules are styling convention; no evidence for them was checked.
2. Photos: collect 15-30, varied and sharp, mostly head or half body, at most 3 full body, no sunglasses, no hands on the face. Keep them locally in `mac/clothes/photos/`. Label each in `mac/clothes/labels.csv`: expression (neutral, smile, other), framing (head, half, full) and light. L-05 checks the set.
3. Looks: a LoRA trained in the cloud ([decision 30](04-decisions-and-checklist.md#decisions)); the Mac only renders tests. Outfits come from the style brief (`prompts/look-generation.txt`). Half-body looks first. Full-body likeness is the weak spot: reference-image methods drift there, and the training set holds at most 3 full-body photos. Whether it can reproduce my smile is unproven; compare against the smile-labelled photos.
4. Amazon: hand-off, not search (proposal). Save a look to the iPhone, open the Amazon Shopping app and use Amazon Lens (photo upload or circle an item) to find the nearest real item. Fallback: an amazon.com search link built from the item's text (to verify).
- Later: a small iPhone gallery if the Mac flow gets weekly use.

**Reuse.** Little. No Reader engine. Only the pattern of the book [pipeline](01-reader.md#pdf-to-levels-pipeline): deterministic script, interactive Sonnet session, script check.

**iOS limits and workaround.** No iOS block in v1, because it runs on the Mac. The blocks are elsewhere:
- Amazon search API: closed to personal apps. PA-API 5 was deprecated in 2026; its replacement, the Creators API, needs at least 10 qualifying affiliate sales in the past 30 days (as of 2026-10-25). Workaround: the Lens hand-off above (proposal).
- Training: the MacBook Air M4 (16 GB) cannot train a LoRA, so training is cloud only.
- Privacy per service (as of 2026-10-25):

| service | trains on uploads? | retention | use |
|---|---|---|---|
| Higgsfield | Yes, by default (terms §4.4). The only opt-out is deleting content or the account, and it applies going forward. | Not needed | Never |
| fal | Its API terms say it does not train on client content. Carve-outs: models marked "Pending Enterprise Ready", and content passed to third-party model providers. Whether its LoRA trainer falls in a carve-out is (to verify). | Inputs and outputs kept 30 days by default; the request header `X-Fal-Store-IO: 0` turns payload storage off. | Default candidate, only after L-06 |

**Case against.** Highest privacy cost of any part: face photos go to a cloud service. Clothes need full-body shots, where reference-image methods drift and the LoRA's likeness is untested. The smile is unproven. Generated items may not exist, so Lens finds lookalikes. Each training run costs about 2-2.40 USD (unverified, as of 2026-10-25). A store fitting room answers "does this suit me" with no build (proposal). Cost to still build: about 5 Sonnet turns plus cloud runs.

**Phase.** Later slot 6 of 6 (proposal), [decision 25](04-decisions-and-checklist.md#decisions). Gate: [decision 30](04-decisions-and-checklist.md#decisions) answered; L-05 and L-06 pass; the owner says go on the privacy doc. Why last: highest privacy and money cost, needs photos, no reuse of the Reader. Hours: [phases](00-overview.md#how-the-apps-fit-around-big-brain).

### Part 7. Productivity (rest)

**Goal.** Beat procrastination, decide better, and use book ideas that survive replication. Journal and gratitude are in [02-journal.md](02-journal.md#goal).

**Smallest useful version.**
- Procrastination: one Reader pack plus one journal change. Core tool: if-then plans (implementation intentions), d = 0.65 across 94 tests (Gollwitzer and Sheeran 2006). The journal's morning "Today I will…" line gets an optional if-then form, "If [time or place], then I will [task]" ([morning card](02-journal.md#morning-reminder-and-time-defaults)) (proposal). The pack also covers CBT-type programs, which beat other types in a preliminary meta-analysis of 24 studies (van Eerde and Klingsieck 2018), and temptation bundling, whose effect fades over time (Milkman 2014). Blocking distractions: Freedom; the Reader's session shield covers only reading sessions.
- Decision making: a short worksheet flow in the Journal app (proposal). Steps: at least 3 options (WRAP's "widen your options"), a pre-mortem ("a year on, it failed: why?"), 10/10/10, the choice, a review date. Saved as `decisions/<id>.json` and exported like journal entries ([decision 12](04-decisions-and-checklist.md#decisions)). A single one-shot reminder at the morning reminder time (Pacific, configurable) on the review date asks how it went; it fits the journal's notification budget ([notifications](02-journal.md#notifications)). Evidence: the pre-mortem lowered overconfidence more than other methods (Veinott 2010, n = 178); the often-quoted "30% better" may count reasons, not correct ones. WRAP is a framework built from research; no trial of WRAP itself was found.
- Book ideas: Reader packs for Thinking, Fast and Slow and Atomic Habits, with caveats on the cards. Thinking, Fast and Slow: its failed chapters are flagged on the [Part 3](#part-3-mind) cards (priming, ego depletion, the hot hand); its core System 1 and System 2 material was not checked. Atomic Habits: no source assessed the book; its implementation-intention step rests on strong evidence (above). "Habits take 66 days": in Lally 2010 the curve fit only 39 of 96 people, with a range of 18-254 days; a 2024 review found medians of 59-66 days and a range of 4-335; one missed day did not derail habit formation. Bias content lives in Part 3; these cards link to it.
- Later: links to Focusmate (body doubling) or Beeminder (money stakes) if the if-then line is not enough.

**Reuse.** The Reader engine for the packs; the journal's morning card, storage, export and notifications ([journal tasks](02-journal.md#task-table)); the L-01 checker.

**iOS limits and workaround.** Blocking distracting apps needs FamilyControls. Its limits and the soft "every app except this one" shield are in the Reader's [iOS limits](01-reader.md#ios-limits-that-block-a-piece). Nothing else is blocked.

**Case against.** Use Ahead instead for a procrastination course ([Ahead on Mobbin](https://mobbin.com/screens/ade4ebb2-545d-462c-ae54-e9fcc400a5ea): a course screen citing van Eerde 2024), [Forest](https://mobbin.com/screens/231ee6c3-7945-4f26-bb73-2e5ecca50f42) as a focus timer, Freedom as a blocker with a locked mode. No dedicated decision-making app was found, though the search was shallow. Reading about procrastination does not stop it; the if-then line and a blocker do the work, and both are small. The part overlaps Part 3 and the journal. Cost to still build: about 7 Sonnet turns.

**Phase.** Later slot 4 of 6 (proposal), [decision 25](04-decisions-and-checklist.md#decisions). Gate: two weeks of Ahead and Freedom, after which the procrastination pack is built only if they leave a gap; the journal is still in use (its week 2-3 check passes, [journal checks](02-journal.md#checks)); the Part 3 packs have shipped; L-01 passes. Why fourth: content in the Reader engine plus one journal line; it follows Part 3 because the bias content lives there. Hours: [phases](00-overview.md#how-the-apps-fit-around-big-brain).
