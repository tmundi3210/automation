# Part 1. Reader (spaced-repetition reading app)

Near build-ready. Phase placement and hours live in [phases](00-overview.md#how-the-apps-fit-around-big-brain). Open items are numbered in [decisions](04-decisions-and-checklist.md#decisions).

## Goal

Turn a PDF or book into fixed learning levels (introduction first, then deeper) and show it one paragraph per screen; the only way past unread material is a passed test.
Every step is retrieval: retell by voice, see what was missed and a likely cause (hint only), retell again; sessions repeat every 4 hours by default (configurable) and hand off to day-scale spacing.
Extras: talk about the text, a physical-book recording mode, and push, boost and quiz calls placed from the Mac. One user, one iPhone, no Claude API key.

## Design

### Similar apps and the case against

Feature details below come from store listings and vendor pages seen only as search summaries (unverified).

| app | overlap with this brief | missing for this brief |
|---|---|---|
| RemNote Guided Learn | Closest. PDF split into sections with summaries, flashcards, quizzes, AI tutor chat, mastery tracker | No paragraph gate, no voice retell. iOS availability of Guided Learn not checked |
| SuperMemo incremental reading | Deepest reading-queue design: extracts, clozes, read points, priority queue | Windows desktop only |
| Recite – Voice Study AI (iOS) | Upload notes or PDF, voice quiz, AI marks covered vs missed, adaptive spacing | No paragraph gate, no fixed N-hour cycle |
| Blurt / Blurtr (iOS) | Speak or type all you remember; AI highlights missing points | No reading flow, no gate |
| Speechify AI Quiz | 5-20 questions generated from your own document | Quiz only; the user picks pages, so nothing is fixed |
| Feynman AI, FeynmanStudy | Explain aloud, clarity score, weak areas | No source reading, no levels; Feynman AI's listing says it is designed for iPad |
| Duolingo "Jump here" | Skip only by passing a short test | Pattern only (used in S7b) |
| Readwise Reader Ghostreader | Passage prompts and chat with the document | Pattern only (used in S9) |

**Use X instead.** For most of this brief, use RemNote Guided Learn for levels, summaries and quizzes, plus Recite or Blurt for voice retell with missed points. Speechify covers document quizzes and Feynman AI covers explain-back scoring. Nobody found combines a no-skip paragraph gate, voice recall every N hours and calls.

**Cost to still build.** This is the largest app in the plan: two phases of app hours, one Claude Code pipeline session per new book and, if the relay is built ([decision 32](04-decisions-and-checklist.md#decisions)), runtime Claude use on the Max plan every day (all in [budget](00-overview.md#budget)), a check set before the scorer is trusted, and a check-set re-run when the Claude model version changes. [Decision 5](04-decisions-and-checklist.md#decisions).

**Strongest case against.**
1. Drop-off is the norm. Education apps keep about 2% (Business of Apps) to 8% (OneSignal) of users at day 30 (unverified). Those are store cohorts, not a tool built by its only user, but fading motivation is the usual way this ends.
2. Ping fatigue and time. A 4-hour cycle in a 15-hour waking window is 4 pings a day; at 2 hours it is 8. At 10-15 minutes a session, the default costs an estimated 40-60 minutes of use a day (proposal).
3. A missed review stalls new content, and a strict gate makes that worse.
4. Hour gaps mostly help same-day memory. The best gap grows with how long you want to remember (about 20-40% of a one-week test delay, Cepeda 2008) (unverified), and the Anki manual says repeating a card several times in one day adds little to long-term memory. The 4-hour cycle is a learning-phase tool; long retention comes from the day phase.
5. The scorer is the weakest link. GPT-4 tracked human scorers closely in two studies (r about .94 on per-clause rates pooled over recalls; .97 or more on per-person totals); a clinical ChatGPT-4o study found r about .65, as close as its human raters came to each other. No study validated per-attempt idea-unit judging by any model, so the check set gates it (unverified). A wrong "you forgot X" breaks trust fast.
6. Gates can dead-end. A topic you cannot pass blocks everything behind it.

**What the design does about it (proposal).** Pings come at most once per base gap and pause after two ignored slots ([notifications](#notifications-and-calls)). Overdue reviews never block new content for more than one session ([schedule](#schedule-and-spacing)). Topics hand off to day-scale spacing after about a day. The scorer must pass a check set before its verdicts gate anything ([scoring](#recall-scoring-gap-and-root-cause)). A topic failed three times goes provisional instead of blocking ([gate](#no-skip-gate-and-test-out)). Use check: if MVP use falls under 8 sessions a week in weeks 2 and 3 after R-23 is on the phone, Phase 3 waits for your review.

### How each piece works

Platform, model and library facts in this section are as of 2026-10-25.

| piece | how it is achieved | iOS API or service | label |
|---|---|---|---|
| Book import | Book package (JSON) made on the Mac, opened in the app from Files | Files document picker, app container | (proposal) |
| Paragraphs | Deterministic script on the Mac ([pipeline](#pdf-to-levels-pipeline)) | PDF text layer or EPUB XHTML; scanned PDFs not in the MVP | (proposal) |
| Levels | Recursive summarization pattern (arXiv 2109.10862), Sonnet in an interactive Claude Code session you start | Claude Max plan, ordinary use, no key | (proposal) |
| Idea units, quiz items | Idea units Sonnet, quiz items Haiku, in the same session, checked by script | Same | (proposal) |
| Reader and gate | Paged view with no scroll, dwell timer, retell gate at topic end | SwiftUI | (proposal) |
| Hour-scale spacing | Anki constants applied in hours ([schedule](#schedule-and-spacing)) | Own code, or Jeet's scheduler if R-01 says it fits | (proposal) |
| Day-scale spacing | FSRS-6 (Free Spaced Repetition Scheduler), desired retention 0.9 | swift-fsrs (MIT) | — |
| Session pings | One local notification per session slot, rolling window | UserNotifications | — |
| Speech-to-text | Live, long-form, on device ([choice](#speech-to-text-choice)) | SpeechAnalyzer with SpeechTranscriber (iOS 26+) | — |
| Recall scoring | One verdict per idea unit plus an evidence quote; output schema checked by script | Claude Sonnet through the Mac relay once built, on hold; interim path in [LLM options](#llm-options-no-claude-api-key) | Scoring quality (to verify) |
| Cause hint | Rules on attempt history plus a contradiction flag from the model ([scoring](#recall-scoring-gap-and-root-cause)) | Same | (proposal) |
| Talk about the text | Model call fed the paragraph, its summary path and idea units; voice input via the system keyboard mic | Sonnet through the Mac relay once built (on hold), or an R-20-passed local model; system dictation | (proposal) |
| Paper-book recording | Record audio session plus the audio background mode; recording continues with the screen locked | AVAudioSession; SpeechTranscriber | Live transcription while locked (to verify) |
| Page-photo fallback | OCR of printed page numbers on the start and end pages | Vision | (proposal) |
| Calls | Scheduled job on the Mac reads the app's status file and asks ClawCall to dial | ClawCall; launchd on the Mac | (proposal) |
| No-server push | Alarm that sounds through Focus and silent mode | AlarmKit (iOS 26+) | (proposal) |
| Focus guard | Guided Access tip; during a session, shield every app except the Reader | Guided Access (user-started); FamilyControls `all(except:)`, paid account | Dev-signed build (to verify) |
| Phone-Mac files | One shared folder for packages, status and call results | iCloud Drive (paid account); else LAN or Tailscale | (to verify) |

#### LLM options (no Claude API key)

| option | strengths | limits | role in this plan |
|---|---|---|---|
| Claude Code, interactive, on the Max plan | Best quality. Ordinary use when you start the session | Uses the weekly limit | Build sessions and book packages |
| Claude Code headless (`claude -p`) through the Mac relay, Max plan | The same Claude models at runtime, no key | Mac must be awake and reachable; daily use of the weekly limit; terms and `--bare` risks in the runtime path below | Runtime default once [decision 32](04-decisions-and-checklist.md#decisions) says build (on hold) |
| Local models: Apple's on-device model, Apple's Private Cloud Compute model (runs on Apple's servers; held to the same R-20 rule), an 8B-class model on the Mac | Private; the on-device model needs no network | Weaker reasoning than Sonnet; idea-unit judging unmeasured; Apple's models need a supported iPhone, Private Cloud Compute also an entitlement (to verify) | None by default; a candidate takes a job only by passing R-20 |
| Other providers' APIs (OpenAI, Gemini) | Strong models | Need a key, an account and per-token billing | Not planned |

Recommendation, decided 2026-10-25 ([decision 16](04-decisions-and-checklist.md#decisions)): quality first. Claude cloud models are the default for every model job, Haiku for bulk and Sonnet where quality matters; Opus only plans. A local model replaces the cloud model on a job only after a side-by-side test on that job's check set ([R-20](#phase-2-reader-mvp)) shows it matches.

| job | model and where it runs |
|---|---|
| Planning and plan revisions | Opus, interactive on the Mac |
| App features, scheduler, scorer service, relay (only if built), docs needing judgment | Sonnet, interactive on the Mac |
| Validators, settings-only screens, harnesses, fixtures, lint scripts, gap lists | Haiku, interactive on the Mac |
| Book levels, summaries and idea units (idea units define what counts as recalled) | Sonnet, in the [pipeline](#pdf-to-levels-pipeline) session you start |
| Quiz items and suggested questions | Haiku in the same session; a topic that fails the spot check is redone with Sonnet |
| Recall scoring, contradiction flag, quiz-call scoring, talk about the text and "ask it for more", paper-book idea units | Sonnet, headless through the Mac relay once [decision 32](04-decisions-and-checklist.md#decisions) says build; until then your own verdicts or an R-20-passed local model |
| Reader packs (Parts 3 and 7, lessons of Parts 4-5); Part 6 style brief; Part 4 session notes, exercises and grammar feedback | Sonnet: packs and the style brief interactive on the Mac, Part 4 through the relay once built ([part cards](03-later-phases.md#part-cards)) |
| Push and boost call scripts | Templates, no model |

**Runtime path (proposal, on hold: [decision 32](04-decisions-and-checklist.md#decisions)).** With no key, the phone cannot call Claude itself. Not built until Talwinder decides. Until then: S6 shows the idea units and you mark each recalled, partial or missed yourself (the check-set labels); a local model that passes R-20 may score instead (R-40). The relay design, kept for that decision:
- The phone sends the transcript (text; audio only if [R-15 picks Whisper](#speech-to-text-choice)) to a small relay on the Mac (`mac/relay/`), over Tailscale or the home network (to verify). The relay runs Claude Code headless (`claude -p`) on your Max login, never with `--bare`, with ANTHROPIC_API_KEY unset. It passes the job's prompt file and model, checks the JSON output with a script, retries once on invalid output and returns the verdicts.
- Target: verdicts within 30 s for 20 idea units; R-39 measures it (to verify).
- The Mac must be awake and reachable; it already runs the Recorder transcription (assumption). When the relay is unreachable or the weekly limit is used up, the retell is saved, its result shows once scored, and the next topic opens provisionally, so nothing dead-ends. How runtime calls relate to the 60% pause: [loop floor](00-overview.md#how-the-apps-fit-around-big-brain).
- Risks: the consumer terms bar automated access except where Anthropic explicitly permits it. Personal scripted `claude -p` use is documented, but as an app backend it is a grey area (to verify). The docs say `--bare` (API key only) is slated to become the `-p` default (to verify). If the subscription path stops working, scoring goes back to the interim path above.

#### Speech-to-text choice

Choice (proposal): on-device SpeechAnalyzer with SpeechTranscriber for live words, in both dictate-back and physical-book mode. Speech-to-text is not a Claude job, so it stays on the iPhone or the Mac. [Decision 18](04-decisions-and-checklist.md#decisions).
- Shows live words while you speak (volatile results), which S5 and S8 need.
- Captures anywhere, with no Mac and no network; the transcript waits on the phone until it is scored.
- Built for long-form audio; no time limit is documented.
- Private; the system fetches the model once.

Why not the Recorder pipeline as the main path: it transcribes on the Mac (faster-whisper large-v3-turbo plus an Indic model), so it shows no live words and captures nothing while the Mac is unreachable. With the relay (on hold, [decision 32](04-decisions-and-checklist.md#decisions)) the Mac would be in the scoring loop anyway, so Whisper may supply the scored transcript: R-15 transcribes one scripted passage both ways and records the word error rate of each; if Whisper is more accurate and the relay exists, the retell audio also goes to the relay, Whisper's transcript is the one scored, and the audio is deleted after (proposal). Keep the pipeline also as a fallback for Punjabi or Hindi reading and as an optional word-count cross-check for long paper-book sessions at home ([reuse](#reuse-and-to-verify)). Whether SpeechTranscriber supports hi-IN or pa-IN is (to verify): no evidence was found, and R-15 reads `supportedLocales` on the iPhone. Books are in English (assumption). SpeechAnalyzer is unavailable on iPhone 11 and SE (2nd gen), per a forum report (unverified).

#### iOS limits that block a piece

| block | what it blocks | nearest workable alternative |
|---|---|---|
| The phone cannot be locked to one app without MDM; app-started Single App mode needs MDM | Stopping "losing interest" by locking the phone | Guided Access tip at session start (you start it); a "you left mid-topic" ping 5 min after leaving; optional shield of every app except the Reader during a session (paid account) (proposal). [Decision 23](04-decisions-and-checklist.md#decisions) |
| BGTaskScheduler is not a timer; its start date is only a floor | Waking the app every 4 hours | Each session slot is a local notification at a fixed time ([notifications](#notifications-and-calls)) (proposal) |
| 64 pending local notifications per app (Apple engineer, developer forums, 2026) | Scheduling far ahead | A plan of at most 10 requests ([notifications](#notifications-and-calls)); Reader and Journal are separate apps with separate budgets, [decision 9](04-decisions-and-checklist.md#decisions) (proposal) |
| SFSpeechRecognizer 1-minute limit (documented for the server path at least) | Long retells and reading sessions | SpeechAnalyzer long-form on device (proposal) |
| An app cannot phone its user; calls need server telephony | Push, boost and quiz calls from the app | ClawCall dials from the Mac; AlarmKit alarm as a no-server push (proposal) |
| Notification actions take buttons or text, not audio | Answering a review from the lock screen | A tap opens straight into the session (proposal) |
| No Claude API key, so the phone cannot call Claude | Scoring and chat from the phone | Mac relay to Claude Code headless on the Max plan, on hold ([decision 32](04-decisions-and-checklist.md#decisions)); until then your own verdicts or an R-20-passed local model ([runtime path](#llm-options-no-claude-api-key)) (proposal) |
| Silent reading makes no sound | Paper-book progress from audio | Read aloud ([physical-book mode](#physical-book-mode)); start and end page photos with OCR otherwise (proposal) |
| Force-quit ends background recording, per a forum report (to verify) | Long paper-book sessions | Save the transcript every 60 s; say so on the session screen (proposal) |
| Free account: builds expire after 7 days; no iCloud, time-sensitive notifications or FamilyControls | Daily use, phone-Mac sync, focus shield | Paid developer program now, while J-01 runs (proposal). [Decision 6](04-decisions-and-checklist.md#decisions) |

### Screens

Chosen pattern per screen, from Mobbin low-res previews seen on 2026-10-25; links as of 2026-10-25. No seen app had a pages-from-speech estimate or an "app calls me" setting; those two elements have no reference.

| screen | pattern chosen (app + Mobbin link) | reason | runner-up (app + link) | copy / avoid |
|---|---|---|---|---|
| S1 Library and import | Speechify [library](https://mobbin.com/screens/7684e3d6-1176-40ec-84f4-de89417dd7ec) with "+" import, [flow](https://mobbin.com/flows/2dc55043-b45e-4406-ab4c-9637d72cc5a2) | Each row shows its read %; import is "+" then Files (previews matched by order; Files sheet seen in Speechify's ["Typing text" flow](https://mobbin.com/flows/3ee8324a-4e85-45e2-b15d-ac11d2fe6aed)) | Google Gemini ["Adding sources"](https://mobbin.com/flows/77c72b90-2dac-42a8-a213-071b5640491a) | Copy: row = title + "N% read · date"; one persistent "+"; empty state pointing at "+"; inline "Analyzing, it will take a moment" row during import ([Mindtrip](https://mobbin.com/screens/2ad98a53-e98f-4c93-a76d-1c344a5e3650)); "Continue reading" mini-bar ([Blinkist](https://mobbin.com/screens/7cec730b-97f5-48c2-9ccb-4846a2d8ef69)). Avoid: 10+ import sources, book store, AI tooltips, onboarding checklist, streak widgets |
| S2 Brief and self-rating | Duolingo sequence: [brief "7 quick questions"](https://mobbin.com/screens/74201708-1b1b-4b6d-b804-92f9eb2d65c9), ["How much X do you know?"](https://mobbin.com/screens/ed5fd27f-19fb-48a9-ae0f-4481e5b7671a), [start from scratch or find my level](https://mobbin.com/screens/8888d91a-a97b-44e7-9c1e-08a75ced8b5c), [placement echo](https://mobbin.com/screens/24aaae49-d6dd-479e-913d-55740b8b25fc) | Only candidate that turns a self-rating into a stated placement and also offers a test-out | [Babbel "How much … do you already know?"](https://mobbin.com/screens/7f7e4dd5-5843-42cd-8a38-5e37958f6fc0) | Copy: brief laid out like the [Deepstash book overview](https://mobbin.com/screens/8fa958b3-67a7-44b4-ac3b-9d04540cd388) (thesis line, level count, minutes, one Start); one question per screen; 5 graded options with a line each; result screen repeats the placement in plain words. Avoid: mascot chatter, sign-up walls, testimonials, paywall cards, goal-commitment screens |
| S3 Level map | Uxcel Go [level list](https://mobbin.com/screens/dba87d9e-9955-49be-be73-686477e601ee) and [locked next level](https://mobbin.com/screens/96c930e2-480b-49a1-ba69-5132a1c5b2b2) | Text list with named levels, a LOCKED tag, and a level-test row that gates the next level | [Duolingo sections list](https://mobbin.com/screens/82371c0c-d233-45af-9fc7-26f96c2cde8f) | Copy: vertical list; level header with "x/y done"; current topic with a Start cue; "Level test" as the last row; "Test out?" on locked items; unlock rule written on the locked card ([Duolingo](https://mobbin.com/screens/a9ae901c-0f55-4e44-8685-2abb506fb08c)); "Time to review" card on top when due ([Babbel](https://mobbin.com/screens/832f5742-a189-409a-8bb1-06f5ed82bb60)). Avoid: icon-only map nodes, chests, gems, hearts, low-contrast locked labels, paywall cards |
| S4 One-paragraph reader | Deepstash [paragraph step](https://mobbin.com/screens/84ab2890-253c-42d1-8944-93ad00575413), [flow](https://mobbin.com/flows/de78ed74-a602-48a0-a966-83d646183300) | One paragraph in large type, segmented progress, one pinned Continue ([Deepstash](https://mobbin.com/screens/f275bf87-c092-416d-8a90-68f6eed26309)): closest to no-skip reading | [Ahead single paragraph](https://mobbin.com/screens/e9361fc0-fc39-4439-a5fe-4a9467d7101f) | Copy: segmented bar for the topic plus "x/N · M min left" for the level ([Blinkist](https://mobbin.com/screens/ea3c48f7-2bb6-4fd2-aec0-01dc925042cf)); Continue disabled during the dwell time; at topic end a "quick check" interstitial ([Brilliant](https://mobbin.com/screens/09af69bb-65c2-4c02-8445-76386c0bccbd)); disabled Continue with a back-link to the source ([Uxcel Go](https://mobbin.com/screens/d868d377-f346-43d1-af25-024fe4c2c528)); "View source" on every summary. Avoid: free long scroll (Blinkist), free prev and next arrows, tap-anywhere advance, illustrations, share and like chrome |
| S5 Dictate-back recording | [ABY Journal recording sheet](https://mobbin.com/screens/d818141b-7c83-40e2-b1ad-28e86e9cbc36) | Live words, waveform, timer, one big Stop and a discard: least friction for a 1-2 minute retell | [Speak question screen](https://mobbin.com/screens/3e01c79e-3175-4df8-9fc6-91966389ffb3) | Copy: topic title and "Retell in your own words" above the mic (Speak); live transcript with the newest word bold; Stop submits; discard asks to confirm; keyboard fallback (Speak). Avoid: "Skip interview" style exits ([Mindtrip](https://mobbin.com/screens/3022caa7-dc4d-4dcd-962e-76429b77e35b)); source text visible during the retell; tab chrome |
| S6 Recall result | [Babbel result groups](https://mobbin.com/screens/00c43458-c5b4-4e0f-9223-f5a933ccc59e) | Score on top, items grouped by status, wrong first, one action | [Duolingo roleplay feedback](https://mobbin.com/screens/35a6f682-bab6-4fc2-be70-b5fa58792795) | Copy: "n of N idea units recalled"; groups Missed, Partial, Recalled (no seen app had three states; this extends Babbel); one "Likely cause (hint)" card in Duolingo's key-takeaway slot; primary "Retell again" ([CapWords](https://mobbin.com/screens/43ae2042-624d-4cd4-805b-13c78b3138af)), secondary "Done" ([Speechify](https://mobbin.com/screens/589102c5-f8c9-4eb6-8677-3eeef7240a9b)). Avoid: "Great effort!" copy that hides misses, prose-only diagnosis, XP and streaks |
| S7a Quiz | Speechify [quiz](https://mobbin.com/screens/7098210e-e083-4635-badd-e77692e3474c), [flow](https://mobbin.com/flows/56036219-119d-407e-90f4-a09d7a64d235) | One question per screen with "n of N", instant right or wrong, built from the user's own document | [Nibble](https://mobbin.com/screens/93499943-b5af-4860-a9b2-23740d190ccd) | Copy: progress bar; locked right-or-wrong state ([Speechify incorrect](https://mobbin.com/screens/b21ae278-14b8-4b81-b500-75189e6d23dc)); explanation after the answer; an "I don't know" option (Nibble). Avoid: user-chosen page range and question count; illustrations that push options below the fold |
| S7b Test-out | Duolingo "Skipping a unit": [flow](https://mobbin.com/flows/2e28d9be-9864-4901-b45d-a26d80736504), [entry on the locked node](https://mobbin.com/screens/4cbc4c0c-4a24-45ce-bbdd-d15b55ad3ab7), [intro](https://mobbin.com/screens/5808d019-c3ee-4e49-bb80-00ceaf09161a) | The only skip is a test placed on the locked content; the miss budget is visible | [Liven test intro](https://mobbin.com/screens/8cc83f79-83f3-4617-86fe-a991b2e364af) | Copy: "Test out?" on the locked item; intro states the pass rule and length; "1 more miss ends the test"; pass unlocks; fail returns to reading. Avoid: "Not now" or "Skip for now" buttons that bypass the gate; hearts and XP |
| S8 Physical-book session | Otter AI [recording](https://mobbin.com/screens/d944a395-8fb1-4ef3-969b-b5d8fdd62eb8), [flow](https://mobbin.com/flows/d7a0fc7c-713c-40b5-bbc4-4447ba7e6a8f) | Only seen screen with a recording dot, elapsed time and live transcript together | [timespent](https://mobbin.com/screens/a6573a78-d93f-4152-b32e-8d530d76cbee) | Copy: big elapsed timer (timespent); red dot, level bars, pause and stop (Otter); last transcript line only; "keeps recording with the phone locked" ([Granola](https://mobbin.com/screens/e96bd18e-eec7-49cf-87c3-ff38ab46a47e)); hold to end ([Me+](https://mobbin.com/screens/fc95dd6b-6a3a-46e8-8723-f7afa9aead9f)); end sheet "about N pages, confirm or edit" ([Life Reset](https://mobbin.com/screens/058d8816-6c09-46ce-b59a-fc62b4896366)). Avoid: tabs, chat chrome, a full scrolling transcript |
| S9 Talk about the text | Matter [Co-Reader sheet](https://mobbin.com/screens/3762e7f0-1b4c-4182-b6e8-1a112542e325), [flow](https://mobbin.com/flows/0951c459-b39a-4ca1-80ea-a05918f623aa) | A sheet over the paragraph keeps the source in view; suggested questions come from that paragraph | [Mindvalley Eve AI](https://mobbin.com/screens/04f358bd-347f-400a-8b9b-3a4484471e1f) | Copy: half-height sheet with the paragraph highlighted; 3 suggested questions; one input with a mic; the answer under the question as its heading; one "Ask" button on the paragraph. Avoid: author chips, "More like this" and "Keep reading" rails (skip-ahead exits), model pickers, upsells, chat over the whole document |
| S10 Today and review queue | [Babbel Today](https://mobbin.com/screens/176965ed-84bd-427a-93ca-4d2b95f1660c) | One hero "Time to review" card makes what is due unmissable | [stoic. countdown](https://mobbin.com/screens/d58aec7d-dd66-494e-a43f-d9339b41305c) | Copy: hero card switches between "N due, start" and "Next session in 3 h 40 m" (stoic.); due count on the button ([Headway](https://mobbin.com/screens/1699f458-c7e7-4bb8-bdd1-47519ce9e340)); thin "paragraphs read x/y" bar. Avoid: a seconds counter, a streak pill (proposal), XP, leaderboards, discovery feeds, a queue buried under Library |
| S11 Settings | [Flo "Drink water"](https://mobbin.com/screens/e4037734-de89-4516-8d77-1cfb9cb43ed2) | On/off, start, end and "Interval 3 h" in one list: exactly base gap plus quiet hours | [Vocabulary "Edit reminder"](https://mobbin.com/screens/e17281e7-8944-4016-ad58-6dc56cccc88d) | Copy: "Study spacing" with a base-gap stepper (Vocabulary-style) and the research-steps preset; "Quiet hours" from and to with the footnote "no pings, no calls" ([Asana](https://mobbin.com/screens/6becc4af-9dd6-408f-8102-01edc32630ea)) and a caption naming the active time zone ([Linear](https://mobbin.com/screens/cf837439-e9bf-410c-bfd3-7e3ec2e98e95)); "Calls" with one toggle per call type and a one-line consequence ([Truecaller](https://mobbin.com/screens/b2aff19f-fbd0-4bad-b050-c8c57a5b87b0)); a link to iOS notification settings. Avoid: weekday-repeat lists, pause presets, extra rows, a generic Repeat picker |

Candidates compared per screen (3-5 seen in Mobbin on 2026-10-25): S1 Speechify, Google Gemini, Mindtrip, Blinkist, Fi. S2 Duolingo, Babbel, Brilliant, Deepstash, Speak. S3 Uxcel Go, Duolingo, Babbel, Mimo, Life Reset. S4 Deepstash, Blinkist, Ahead, Uxcel Go, Brilliant. S5 ABY Journal, Speak, Mindtrip, Otter AI, Fabric. S6 Babbel, Duolingo, Speechify, CapWords, Quizlet. S7a Speechify, Nibble, Quizlet, Acorns, Duolingo. S7b Duolingo, Liven, Equinox+, Crypto.com, Udemy. S8 Otter AI, timespent, Me+, Granola, Life Reset. S9 Matter, Perplexity, Mindvalley, Speechify, Fabric. S10 Babbel, stoic., Headway, Mimo, Speak. S11 Flo, Vocabulary, Asana, Linear, Truecaller.

### Data model

The book package stays a read-only JSON file. Progress goes in a Reader store in `Shared/Storage/`: the journal's JSON file store if R-10 shows it copes with the volume, else SQLite (proposal). Times are stored in UTC and shown in Pacific at home; the travel rule is shared with the Journal ([decision 13](04-decisions-and-checklist.md#decisions)).

| entity | key fields | notes |
|---|---|---|
| Book | id, title, author, source hash, package version, depth target (L1, L2 or L3), placement, status | One per imported package |
| Level | id, book, rank 0-3, title | 0 brief; 1 chapter summaries ("introduction"); 2 section summaries; 3 original paragraphs ("deeper") |
| Topic | id, level, chapter, order, unit ids, state (locked, open, passed, provisional, tested out) | The gating and scheduling unit |
| Unit | id, topic, order, text, word count, page refs, source paragraph ids, minimum dwell | A paragraph (L3) or a summary block (L0-L2); summaries always list their source paragraphs |
| IdeaUnit | id, unit, text, span in unit, central (yes or no), origin (book, ask-for-more, paper session), known by test-out | One proposition each; counts per unit in the [pipeline](#pdf-to-levels-pipeline) |
| QuizItem | id, topic, idea unit ids, stem, options plus "I don't know", answer, explanation | From the package; used for test-out, level tests and quiz calls |
| ReadEvent | unit, shown at, dwell seconds, advanced at | "How much I have read": paragraphs, words, minutes, % per book |
| Attempt | id, topic, kind (retell, review, quiz, test-out, level test, call quiz, paper session), loop 1-3, transcript, typed (yes or no), model and version, prompt version | Retell audio is deleted after transcription (proposal) |
| Verdict | attempt, idea unit, recalled / partial / missed, evidence quote, contradiction flag, cause hint (never learned, misunderstood, forgotten), user flip | Feeds the gap view and the check set |
| ScheduleState | topic, mode (hour or day), gap hours, ease, due at, reps, lapses, FSRS card (stability, difficulty, last review) | One per topic |
| PaperSession | id, book title, start page, confirmed end page, duration, words, words-per-page calibration, transcript | Physical-book mode |
| ChatTurn | unit, question, answer, source tag (book or outside), saved idea unit id | Talk about the text |
| Settings | base gap, spacing preset, quiet hours, time-zone mode, reviews per session, pass threshold, dwell rate, call toggles, calls per day | Each default is set where its rule is described |

Files exchanged, with schemas in `schemas/`: `book-package.schema.json` (Mac to phone), `status-export.schema.json` (phone to Mac, for calls), `call-result.schema.json` (Mac to phone), `bigbrain-export.schema.json` (phone to Big Brain, only if [decision 14](04-decisions-and-checklist.md#decisions) says yes).

### PDF-to-levels pipeline

Runs on the Mac, once per book, in an interactive Claude Code session you start. Output: one validated book package. Turns per book are counted in [budget](00-overview.md#budget).

1. **Input.** A PDF with a text layer, or an EPUB, in `mac/pipeline/inbox/`. "A PDF or book" means a PDF or an EPUB file; paper books use [physical-book mode](#physical-book-mode) (assumption). An EPUB is a zip of XHTML files read in spine order (unverified). Not in the MVP: scanned PDFs; the script stops with "needs OCR", and Vision OCR is a later add (proposal).
2. **Paragraphs (script).** `scripts/extract-paragraphs.py` joins hyphenated line breaks, drops lines repeated on most pages (running headers, page numbers), builds a chapter and section tree from headings, and keeps page refs.
3. **Levels (Sonnet, `prompts/level-generation.txt`).** Bottom-up: section summaries (L2) from paragraphs, chapter summaries (L1) from section summaries, a brief (L0) from chapter summaries. Every summary lists its source paragraph ids, so each summary screen has "View source": the source opens below the summary, not beside it, since a phone screen is too narrow for both (proposal). One chapter per step, so each summary is written from its whole source with nothing compacted away (proposal).
4. **Topics (script).** L3 paragraphs grouped into topics of 3-7 along section boundaries; each summary block is one topic.
5. **Idea units (Sonnet, `prompts/idea-units.txt`).** 2-4 per L3 paragraph, 3-8 per summary unit. Each is one proposition with its exact span in the unit; 1-2 per unit marked central.
6. **Quiz items and questions (Haiku, `prompts/quiz-generation.txt`, `prompts/suggested-questions.txt`).** 3-5 multiple-choice items per topic, each tied to idea units, plus "I don't know"; 3 suggested questions per topic for [talk about the text](#talk-about-the-text).
7. **Lint before any repair (script).** `scripts/validate-package.py` checks the schema; every L3 paragraph in exactly one topic; every summary has a source id; every idea-unit span is found in its unit; counts within limits; no duplicate ids; the banned-word list. Failures go to one repair pass that touches only the failing items.
8. **Spot check (you, about 10 minutes).** `scripts/sample-package.py` prints 10 random summaries beside their sources, 5 idea units and 5 quiz items. More than 2 bad out of 20: regenerate with Sonnet each chapter that had a bad summary or idea unit and each topic that had a bad quiz item, then sample again.
9. **Deliver.** The package reaches the phone through the shared iCloud Drive folder (paid account) or AirDrop to Files. The app validates it again before import.

Level order (assumption): L0 and L1 cover the whole book first ("introduction first"), then each chapter goes deeper, L2 then L3, in book order. Each book gets a depth target at import (L1, L2 or L3; default L3) that is fixed afterwards (proposal). [Decision 15](04-decisions-and-checklist.md#decisions).

Not in the MVP: making packages on the phone. The pipeline needs a Claude Code session and your spot check, both on the Mac (proposal).

### Schedule and spacing

The topic is the unit of scheduling. Numbers below are (proposal) unless they are Anki constants. Times are Pacific and configurable.

**(a) Base gap and the hour-scale rule.**
- Base gap G = 4 h by default. Settings stepper 1-12 h; a change re-plans pings at once and applies to future gaps.
- Session slots every G hours from the end of quiet hours: 07:30, 11:30, 15:30, 19:30 at G = 4. The active window is every hour outside quiet hours; slots exist only there. Each slot = due reviews first (tests earlier material), then the next new topic (adds new content). That is the requested 4-hour cycle. A review joins the first slot later than its due time minus G/2, so due times round to the nearest slot, ties going later.
- The grade comes from the first retell of the session, not from later loops: Again = below the pass threshold; Hard = pass with at least a quarter of units partial; Good = pass; Easy = every unit recalled, none partial.
- Rule in hours, with Anki's constants (start ease 2.5, floor 1.3). First review: G after reading. Good: next = (current + half the hours late) × ease. Hard: next = current × 1.2, ease −0.15. Easy: next = (current + hours late) × ease × 1.3, ease +0.15. Again: next = G, ease −0.2. Guard: next ≥ current + G, the hour version of Anki's "+1 day" (a design choice, not an Anki fact).
- Quiet hours (default 22:30-07:30) push a due time to the end of the window.
- "The gap grows after each success" (assumption). [Decision 21](04-decisions-and-checklist.md#decisions). If no, the gap stays G until the hand-off after 3 passes.
- Example, G = 4, all Good: read in the 07:30 slot; review in the 11:30 slot; the next 10 h gap is due about 21:30, which rounds past 19:30 into quiet hours, so 07:30 next day; the next gap would be over 24 h, so the topic moves to days. Two hour-scale reviews. At G = 2 the nominal gaps are 2 h, 5 h and 12.5 h, then days: three.

**(b) Hand-off to days.**
- When the computed next gap reaches 24 h, the topic moves to day mode: swift-fsrs with FSRS-6 parameters selected explicitly (the package defaults to FSRS-5, as of 2026-10-25), desired retention 0.9.
- The hand-off review is the FSRS card's first review, with the same grade (Again 1, Hard 2, Good 3, Easy 4).
- Day-mode reviews fall on the FSRS due day, in the slot nearest the time of the last review.
- A lapse (Again) in day mode gets one hour-scale relearning step at G, then returns to FSRS.

**(c) The difference, in 3 lines.**
- Your 4 h rule, with the gap growing by ease after each pass (assumption): 4 h, about 10 h, then days.
- Research rule: fixed steps of 2 h, 4 h and 8 h, then FSRS in days.
- Both reach day scale within about a day; hour gaps mostly help same-day recall, and long-term memory comes from the day phase.

**(d) [Decision 20](04-decisions-and-checklist.md#decisions).** Recommended default (proposal): your 4 h base gap growing by ease, hand-off at 24 h. It keeps your cycle and needs two hour-scale reviews per topic instead of three, so fewer pings. Settings also carry a "Research steps 2/4/8 h" preset so both can be tried. You decide.

**Missed reviews.** Overdue work never blocks new content for more than one session. A session takes at most 5 due reviews (most overdue first in hour mode, lowest predicted recall first in day mode), then the new topic; the rest roll to the next slot. Lateness is priced in: Anki's half-late rule in hour mode, FSRS in day mode. [Decision 22](04-decisions-and-checklist.md#decisions).

### No-skip gate and test-out

Gate unit: the topic (3-7 paragraphs). A retell after every paragraph would mean a voice recall every minute or so. [Decision 17](04-decisions-and-checklist.md#decisions). All thresholds here are (proposal) and live in Settings.

**Inside a topic.**
- One paragraph per screen. No scroll past it, no tap-anywhere advance, no search, no forward jump in the outline. Going back one paragraph inside the topic is allowed; re-reading is not skipping.
- Continue unlocks after a minimum dwell and you tap it; "it moves forward" is read as this, not as auto-advance (assumption). Dwell = word count at 500 words a minute, twice a typical reading speed of about 250 (unverified). A 100-word paragraph keeps Continue locked for 12 s.
- A paragraph advanced within 10% of its minimum dwell marks its idea units "skimmed", which feeds the cause hint.

**At the end of a topic.**
- A one-line "quick check" screen, then the retell (S5), then the result (S6).
- Pass = score of at least 70% (recalled counts 1, partial 0.5) and every central unit at least partial. Pass unlocks the next topic.
- Fail starts the [dictate-back loop](#dictate-back-loop). After 3 failed loops the topic goes provisional: the next topic unlocks, its missed units enter the review queue at G, and the level test cannot pass until they are recalled. What must be learned stays fixed, without a dead end.

**Level test.** Last row of each level on S3. Mixed quiz from the level's items plus one retell of its central units. Pass: quiz at least 80% and every central unit at least partial in the retell; it opens the next level.

**Test-out (S7b).**
- Offered on any locked topic or level as "Test out?". Never a plain skip.
- Format: every quiz item for that topic or level with the miss budget shown, plus a 2-minute "tell me what you know" retell scored against its central units.
- Pass: quiz at least 90% and central units at least 80% recalled. It unlocks; its idea units are marked known and start in day mode.
- Fail: back to reading, no penalty. A retake opens from the next slot, with items reordered.
- "I don't know" counts as wrong, and beats guessing.

**First run (S2).** Brief (L0), then the self-rating "How much do you already know?" (5 levels), then "Start from scratch" or "Find my level". Find my level asks 1-2 L1 quiz items per chapter plus an optional voice "tell me what you know" scored against L1 idea units. Chapters passed are tested out at L1. The first step starts at the first chapter not passed and includes the idea units you did not mention, so it adds content beyond what you said. The self-rating alone unlocks nothing; it picks the path offered: "new" starts at chapter 1, anything higher offers the placement test first (proposal).

**Losing interest.** See the focus-guard row in [iOS limits](#ios-limits-that-block-a-piece). Resume always reopens the exact paragraph.

### Recall scoring, gap and root cause

**Pass 1: which idea units are missing.** Sonnet through the relay once it exists, `prompts/recall-scoring.txt`.
- Input: the transcript and the topic's idea units. Output per unit: recalled, partial or missed, plus a short quote from the transcript as evidence. The output schema, checked by script, requires exactly one verdict per unit.
- Evidence check in app code: a recalled or partial verdict whose quote is not in the transcript is downgraded to missed. This blocks invented credit.
- Score = (recalled + 0.5 × partial) ÷ units. The interim path while the relay is on hold, the time target and the unreachable-Mac rule are in the [runtime path](#llm-options-no-claude-api-key).
- Each attempt stores the model version and prompt version.

**Pass 2: why.** Reading "two-step thinking, like Big Brain" as pass 1 = what is missing, pass 2 = why, is an (assumption). [Decision 19](04-decisions-and-checklist.md#decisions).
- Never learned: never recalled in any attempt, or skimmed, or known only through test-out. A rule, no model.
- Forgotten: recalled or partial before, missed now. A rule, no model.
- Misunderstood: the retell says something that contradicts the unit. A Sonnet flag (`prompts/contradiction-check.txt`), run only on missed or partial units; skipped while no live scorer exists.
- Precedence when two apply: misunderstood, then forgotten, then never learned.

**What you see.** The gap is the exact missed units and the paragraphs that hold them. The cause shows only as "Likely cause (hint)" on S6, never as a verdict. A "wrong hint" tap logs it.

**Check set before trusting the scorer (proposal).**
- Until the gate passes, every verdict on S6 has a flip toggle (with no live scorer, you mark every unit yourself), and your confirmed verdicts become the labels in `tests/recall-checkset/`, so no separate labelling session is needed.
- After 40 confirmed retells on the pilot book, `scripts/score-checkset.py` reports per-unit agreement, Cohen's kappa (recalled vs not) and false credit (missed units marked recalled).
- Batch session: until the relay exists, Sonnet scores those 40 in one batch in an interactive Claude Code session you start on the Mac (ordinary use); R-19 and R-20 use it. The result tells whether Sonnet scoring is trustworthy for when [decision 32](04-decisions-and-checklist.md#decisions) is decided.
- Gate: agreement at least 85%, kappa at least 0.6, false credit at most 10%. Then, once a live scorer exists and its own verdicts on the same 40 retells also meet these numbers, the toggles hide and its verdicts drive the topic gate.
- If it fails, user confirmation stays on; each prompt revision re-runs the gate on the same set. The gate also re-runs when the Claude model version changes.

### Dictate-back loop

Design (proposal):
1. Retell on S5: topic title and "Retell in your own words". Source hidden, live words, typed fallback for noisy places. Under 15 words counts as no attempt and the gate stays shut.
2. Score (both passes), then S6: Missed, Partial, Recalled and one hint card.
3. "Retell again": first re-read only the paragraphs that hold missed or partial units, one per screen with the same dwell rule; then retell. Only those units are scored; earlier credit is kept.
4. At most 3 loops per session. Then the topic is scheduled from the first retell's grade, or goes provisional ([gate](#no-skip-gate-and-test-out)).
5. Reviews of earlier topics use a short retell (about a minute) of the central units. "Quiz instead" is allowed when speaking is not possible; its grade is capped at Good, since recognition is easier than free recall (unverified).

Why retell at all: testing beats restudy (Rowland 2014 g ≈ 0.50; Adesope 2017 g ≈ 0.61) (unverified). Explain-back itself has only indirect evidence, from self-explanation and learning-by-teaching studies.

### Physical-book mode

You read aloud; silent reading makes no sound to analyze (assumption). [Decision 25](04-decisions-and-checklist.md#decisions).

1. On S8, pick or name the paper book and enter the start page.
2. Record, with the screen locked if wanted ([paper-book row](#how-each-piece-works)); no time limit is documented. Hold to end.
3. Transcribe live on device. The transcript is saved every 60 s, so a force-quit loses at most a minute. Whether SpeechTranscriber keeps running with the screen locked is (to verify) in R-30; if it stops, the audio goes to a file that is transcribed on Stop and then deleted (proposal).
4. At the end, pages ≈ words read ÷ words per page. The first session per book asks for the end page and stores words per page; later sessions show "about N pages, confirm or edit". Minutes and pages add to "how much I have read".
5. Optional retell. Idea units come from the session transcript (`prompts/session-idea-units.txt`; 2-3 per 600 words, at most 15) and need a live model, as [talk about the text](#talk-about-the-text) does; until one exists, the transcript is kept and the retell waits. One swipe deletes a bad unit. The rest are scheduled like any topic.

Alternative (proposal): photos of the start and end pages; Vision OCR reads the printed page numbers. It works for silent reading but gives no transcript, so no retell.

R-30 checks whether the Reader can record on the main iPhone while the Big Brain recorder holds the mic and the DJI receiver (to verify). Your brief names both a main iPhone with a DJI mic and a spare iPhone streaming 24/7; this plan reads both as parts of the Recorder setup (assumption). [Decision 8](04-decisions-and-checklist.md#decisions).

### Talk about the text

"Talk" means typing or dictating questions, with answers shown as text (assumption). Design (proposal):

- Entry: one "Ask" button on the paragraph (S4) opens a half-height sheet; the paragraph stays visible.
- Context: the current paragraph and the topic's earlier paragraphs, its summary path (brief line, chapter summary, section summary), the idea units of paragraphs already shown and the whole chat on this topic, sent to the chat model (honest limit below).
- No skip-ahead through chat: the context holds only units already shown. A question about later parts gets "that comes later in the book".
- Every answer carries a "from the book" or "outside the book" tag (a field in the output schema), shown under it.
- "Ask it for more": a More chip returns one extra card (an example, an analogy or more detail) for the paragraph. "Save as idea unit" adds it to the topic's reviews; at most 3 saved per topic.
- Honest limit: chat needs a live model: the relay once [decision 32](04-decisions-and-checklist.md#decisions) says build, or a local model that passed R-20's blind side-by-side for text outputs. Until one exists, the sheet says chat is not set up yet and keeps the question. When the chat model runs on the Mac and the Mac is unreachable, the sheet says so and keeps the question until it is.

### Notifications and calls

**Notifications** (local only; the shared planner and its rebuild method are in [journal notifications](02-journal.md#notifications)).
- One notification per session slot with work waiting: "Reader: 3 reviews + 1 new topic, about 12 min". A tap opens the session.
- Rolling plan (proposal): the next 2 slots, then only the first slot of each morning for the next 7 days; at most 10 pending, counting the mid-topic ping. Rebuilt when you open the app, after each session, after any settings change and after a time-zone change; never from background refresh, which would undo the pause below.
- Fatigue guards (proposal): none in quiet hours; at most one per G; after 2 ignored slots in a row, pings pause until the next morning's first slot. The plan's shape does this, since iOS tells the app nothing about an ignored ping.
- "You left mid-topic": one ping 5 min after leaving mid-topic, cancelled on return.
- Time-sensitive delivery (breaks through Focus) needs the paid account; off by default.

**Calls** (Phase 3). An app cannot call its user, so ClawCall dials from the Mac; US and English only. "During learning" means on days a book is active, at slot times, never mid-paragraph (assumption).

| call | when | content |
|---|---|---|
| push | A slot passes with no session for 30 min | "Your 15:30 session is waiting: 3 reviews, 1 new topic." |
| boost | After a level test passes, or at a set time | Today's facts only: paragraphs read, topics passed, next slot. "Boost" is read as this progress call with no praise, since you asked for none (assumption) |
| quiz | At a slot with 2-3 idea units due | 2-3 due questions read aloud; you answer by voice |

Data path (proposal):
1. After each session and on leaving the app, the app writes `status.json` (planned slots, due topics, 2-3 due idea units with questions, today's progress) to the shared folder.
2. A launchd job in `mac/calls/` runs every 15 min, applies the rules, and asks ClawCall to place the call with a short script built from `status.json`. No model writes call scripts: push and boost are templates, and quiz questions come from the package. Whether ClawCall's API takes a custom script per call and returns the transcript is checked in R-29 ([reuse](#reuse-and-to-verify)).
3. The Mac saves the quiz transcript at once, scores it with the live scorer if one runs on the Mac ([runtime path](#llm-options-no-claude-api-key); else an on-device scorer scores it after import, or you mark its verdicts on S6, as for retells), and writes transcript and verdicts to `call-result.json`; one listing says ClawCall deletes transcripts after 7 days (unverified, as of 2026-10-25). The app imports it on next open; it counts as a review.

Rules (proposal): at most 1 call a day, none in quiet hours or the 30 min before them (so a missed last slot at G = 2 does not ring over the Journal's 10:00 pm ping), none within 2 h of the last, no push after 2 missed slots in a row until the next morning's first slot; each type toggled in Settings (defaults: push and quiz on, boost off). The Mac must be awake for the call job. No-server fallback (proposal): an AlarmKit alarm 30 min after a planned slot with no session, under the same time rules, set only for slots in the notification plan so it pauses with the pings; push calls are off while it is on. [Decision 24](04-decisions-and-checklist.md#decisions).

### Reuse and to verify

| asset | intended reuse | status and check |
|---|---|---|
| Jeet (`~/Projects/jeet-ios`) | Scheduler, flashcard model, notifications and question generation, for spacing, quiz items and the notification planner | All (to verify); R-01 reports what is reusable. If its scheduler is day-only or SM-2 only, use swift-fsrs plus the hour layer above. If its question generation calls an LLM with a key, it is not used at runtime |
| Always-on Recorder | Speech-to-text fallback for Punjabi and Hindi; word-count cross-check; the scored transcript if R-15 finds its Whisper more accurate and the relay exists | (to verify): reading its transcript for a time window, and the Indic model's output format. Not the live path |
| ClawCall | Push, boost and quiz calls | A test call to you worked. Custom script per call, API use from a Mac job, transcript return, caller ID: (to verify), R-29 |
| Big Brain loop | Optional daily export of reading progress and recall results | [Decision 14](04-decisions-and-checklist.md#decisions). Reader work never changes loop code; the export is a file the loop may read |
| Shared package from the Journal phase | Storage, notification planner and folder exporter | Built in Phase 1 with the Journal ([journal task table](02-journal.md#task-table); [decision 3](04-decisions-and-checklist.md#decisions), decided: build); the Reader adds Speech, LLM, Gap and Scheduling modules |
| swift-fsrs | Day-scale scheduling | MIT; FSRS-6 must be selected explicitly; parses hour steps |
| This engine, later | Part 3 reuses the brief, ask, add, deepen flow and the pipeline | [Part 3](03-later-phases.md#part-3-mind) |

## Task table

Paths follow the new repo layout ([decision 9](04-decisions-and-checklist.md#decisions)). Models per job: [LLM options](#llm-options-no-claude-api-key); script = deterministic and model-free. A script lint runs before every repair pass. Prompt files are named only; their text is written after approval. Phase hours: [phases](00-overview.md#how-the-apps-fit-around-big-brain).

### Phase 0

| id | files | done-check |
|---|---|---|
| R-01 Inspect Jeet and report what is reusable · Sonnet | read-only `~/Projects/jeet-ios`; writes `~/Projects/mundi-apps/docs/jeet-reuse-report.md` (creates the folder only; J-01 later makes it the repo) | A Claude Code session on the Mac. The report has 5 sections (scheduler algorithm and time units; card and question model; notification code and pending-count handling; question generation and whether it calls an LLM with a key; whether it builds today), each with file paths and a verdict: reuse, adapt or drop. `git status` in jeet-ios shows no changes. The report states that ANTHROPIC_API_KEY is unset on the Mac. User review takes 15 min or less |

### Phase 1: book-package spike

Starts now, in parallel with J-01 ([decision 1](04-decisions-and-checklist.md#decisions)).

| id | files | done-check |
|---|---|---|
| R-02 Book-package schema and fixtures · Sonnet | `schemas/book-package.schema.json`, `docs/book-package.md`, `tests/fixtures/mini-book.json`, `tests/fixtures/bad/` | A standard JSON-schema validator accepts the 2-chapter fixture and rejects 3 bad fixtures (missing source ids, empty idea units, unknown level); 2 more (duplicate ids, 9 idea units on one paragraph) need cross-field checks and go to R-04 |
| R-03 Paragraph extraction · Sonnet | `scripts/extract-paragraphs.py`, `tests/extract/` | On the pilot PDF: paragraph count within 5% of your hand count on 3 sample chapters (about 20 min); 0 running headers or page numbers left (repeated-line check); 0 words split by line-break hyphens; an EPUB fixture yields paragraphs in spine order; a scanned PDF exits with "needs OCR" |
| R-04 Package validator and sampler · Haiku | `scripts/validate-package.py`, `scripts/sample-package.py`, `scripts/banned-words.txt` | Exit 0 on the good fixture; non-zero with a named error on each of the 5 bad ones; covers paragraph coverage, source ids, idea-unit spans, counts, duplicates and banned words; the sampler prints 10 summaries beside sources, 5 idea units and 5 quiz items |
| R-05 Pipeline prompts · Sonnet | `prompts/level-generation.txt`, `prompts/idea-units.txt`, `prompts/quiz-generation.txt`, `prompts/suggested-questions.txt`, `scripts/lint-prompts.py` | The lint script checks every file in `prompts/` for an output schema, limits, a banned list and one example; run on the mini fixture, the output passes R-04 with at most 2 failing items |
| R-06 Pilot levels and topics · Sonnet | `mac/pipeline/inbox/`, `mac/pipeline/build/pilot/paragraphs.json`, `mac/pipeline/build/pilot/levels.json`, `docs/pilot-package-report.md` | One real PDF you choose; L0 exists once; L1 and L2 exist for every chapter; every summary lists source ids; turns and time used are written to the report |
| R-07 Pilot idea units, quiz items and questions · Sonnet (idea units), Haiku (quiz items, questions) | `mac/pipeline/build/pilot/pilot.readerpkg.json` | Every unit has idea units within limits; every topic has 3-5 quiz items and 3 suggested questions; the file passes the schema |
| R-08 Lint and sample the pilot package · script; repair Sonnet (summaries, idea units) or Haiku (quiz items, questions) | `mac/pipeline/build/pilot/lint-report.txt`, `docs/pilot-package-report.md` | Validator exits 0 after at most one repair pass that touched only failing items; you mark at most 2 of 20 sampled items bad |

### Phase 2: Reader MVP

| id | files | done-check |
|---|---|---|
| R-09 Reader target on the shared package · Sonnet | `Reader/Reader.xcodeproj`, `Reader/App/ReaderApp.swift`, `mundi-apps.xcworkspace`, `scripts/build-all.sh` | Adds the Reader to the workspace and Shared package that [J-01](02-journal.md#task-table) created; `scripts/build-all.sh` builds Journal, Reader and Shared with 0 errors; the Reader installs next to the Journal and opens to an empty library; it imports Shared storage and notifications, no copies |
| R-10 Data model and package import · Sonnet | `Reader/Model/`, `Reader/Import/PackageImporter.swift`, `Shared/Storage/`, `tests/ReaderTests/ImportTests.swift` | Importing the pilot package creates exactly the level, topic, unit and idea-unit counts the validator reports; a corrupted package imports nothing and names the failing check; a store with 5,000 idea units and 500 attempts opens in under 1 s on the iPhone |
| R-11 Library and import S1 · Sonnet | `Reader/Views/LibraryView.swift` | "+" then Files imports a package in 2 taps; the row shows "N% read · date"; a processing row shows during import; the empty state points at "+" |
| R-12 Brief, self-rating and placement S2 · Sonnet | `Reader/Views/OnboardingView.swift`, `Reader/Placement/Placement.swift` | First open of a book shows brief, 5-level self-rating and start choice; "Find my level" asks 1-2 items per chapter; passed chapters show as tested out on S3; the first step starts at the first chapter not passed |
| R-13 Level map S3 · Sonnet | `Reader/Views/LevelMapView.swift` | Later levels show LOCKED with the unlock rule written; the current topic is highlighted; "Test out?" appears on locked items; a "Time to review" card sits on top when reviews are due |
| R-14 One-paragraph reader and dwell gate S4 · Sonnet | `Reader/Views/ParagraphReaderView.swift`, `tests/ReaderUITests/NoSkipTests.swift` | UI test: swipe, scroll and tap-anywhere leave the paragraph index unchanged; Continue stays disabled 12 s on a 100-word paragraph; back works only inside the topic; read counters update |
| R-15 On-device transcriber and Whisper comparison · Sonnet | `Shared/Speech/Transcriber.swift`, `Shared/Package.swift`, `scripts/word-error-rate.py`, `docs/device-capabilities.md` | On the iPhone, a 5-minute continuous read-aloud transcribes without stopping and live words appear within about 1 s; the same scripted passage is also transcribed by the Recorder's Whisper on the Mac, and the doc records the word error rate of both and which transcript gets scored with and without the relay; the doc records `supportedLocales` (hi-IN and pa-IN present or not) |
| R-16 Dictate-back screen S5 · Sonnet | `Reader/Views/RetellView.swift` | Source text hidden; live transcript with the newest word bold; Stop submits; discard asks to confirm; typed fallback works; under 15 words keeps the gate shut |
| R-17 Recall scorer with evidence check, manual verdicts and queued scoring · Sonnet | `Shared/LLM/RecallScorer.swift`, `Shared/Package.swift`, `prompts/recall-scoring.txt`, `prompts/contradiction-check.txt`, `tests/ScoringTests/` | Tested on fixture verdicts made in an interactive Claude Code session and on manual verdicts: the output schema check enforces exactly one verdict per idea unit; a recalled or partial verdict whose quote is missing from the transcript is downgraded (test); with no scorer, S6 takes your verdicts for every unit; an unscored retell stays queued and is scored once a scorer exists; model (or "manual") and prompt version saved per attempt; `scripts/lint-prompts.py` passes on both prompts |
| R-18 Check-set harness · Haiku | `scripts/score-checkset.py`, `tests/recall-checkset/` | On a 5-retell sample with known labels, the printed agreement, kappa and false-credit rate match a hand calculation |
| R-19 Run the check-set gate · script; Sonnet verdicts from the batch session | `tests/recall-checkset/results.txt` | 40 retells you confirmed, scored by Sonnet in the [batch session](#recall-scoring-gap-and-root-cause) until the relay exists; pass means agreement at least 85%, kappa at least 0.6, false credit at most 10%; the result is saved with the model version |
| R-20 Side-by-side test, only if a local model is proposed for a job · Sonnet | `scripts/side-by-side.py`, `docs/side-by-side.md`, `tests/recall-checkset/`, `tests/side-by-side/<job>/` | Needs no relay: the cloud model's outputs come from an interactive Claude Code session you start on the Mac (for recall scoring, the [batch session](#recall-scoring-gap-and-root-cause)). For recall scoring, the candidate and the cloud model score the same 40 labelled retells; the candidate may take the job only if its agreement is at least the cloud model's minus 2 points and its false credit is no higher, and it also passes the [check-set gate](#recall-scoring-gap-and-root-cause) on its own (proposal). While decision 32 is on hold, Apple's on-device model is proposed for recall scoring once R-19 is saved, if the iPhone supports it ([decision 7](04-decisions-and-checklist.md#decisions)) (proposal). For any other job, you first label 40 cases of it in `tests/side-by-side/<job>/`; where the output is text (chat, idea units, notes), you pick blind between the two outputs, and the candidate must be preferred or tied in at least half (proposal). The doc records both models' results |
| R-21 Result screen, cause hints and retell loop S6 · Sonnet | `Reader/Views/ResultView.swift`, `Shared/Gap/CauseRules.swift`, `Shared/Package.swift`, `tests/GapTests/` | Missed group shows first; the hint card says "hint"; rule tests cover never learned, forgotten and misunderstood with the stated precedence; "Retell again" shows only paragraphs with missed or partial units; the loop stops at 3; the schedule uses the first attempt's grade |
| R-22 Hour-scale scheduler and FSRS hand-off · Sonnet | `Shared/Scheduling/HourScheduler.swift`, `Shared/Scheduling/FSRSBridge.swift`, `Shared/Package.swift` (swift-fsrs dependency), `tests/SchedulingTests/` | Table tests: G 4, all Good, read at 07:30 gives a review in the 11:30 slot, then 07:30 next day, then day mode; G 2 gives 3 hour-scale reviews; Again resets to G and ease never drops below 1.3; next is always at least current + G; the FSRS parameter array has 21 values (FSRS-6), not 19 (FSRS-5) |
| R-23 Session builder and Today S10 · Sonnet | `Reader/Session/SessionBuilder.swift`, `Reader/Views/TodayView.swift` | With 9 overdue reviews the session is 5 reviews then the new topic, and 4 roll over; the hero card shows "N due" or "Next session in Xh Ym" |
| R-24 Quiz, level test and test-out S7 · Sonnet | `Reader/Views/QuizView.swift`, `Reader/TestOut/TestOut.swift` | Test-out at 90% unlocks and puts its units in day mode with a first review at least 24 h out; at 50% it returns to reading and changes nothing but the attempt log; a retake is blocked until the next slot; "I don't know" scores wrong; a level test at 80% with every central unit at least partial opens the next level, and at 70% it does not |
| R-25 Reader notification plan · Sonnet | `Reader/Reminders/ReaderSlotPlan.swift`, `Shared/Notifications/RollingScheduler.swift`, `tests/NotificationTests/` | After any rebuild, pending is 10 or fewer and none falls in quiet hours; changing G from 4 to 2 re-plans within 1 s; with the debug clock and no sessions, pings fire for the next 2 slots, then none until the next morning's first slot |
| R-26 Settings S11 · Haiku | `Reader/Views/SettingsView.swift` | Base-gap stepper 1-12 h; quiet hours with a caption naming the active time zone; the research-steps preset; values survive a relaunch; the link to iOS notification settings opens |
| R-27 Focus guard: mid-topic ping and Guided Access tip · Sonnet | `Reader/Focus/FocusGuard.swift` | Leaving mid-topic fires one ping at 5 min; returning earlier cancels it; the Guided Access tip shows at session start until "don't show again" |
| R-28 Debug clock and diagnostics · Haiku | `Reader/Views/DebugView.swift` | Debug builds only: a clock offset moves "now" by hours or days and the Today screen and pending list follow; the screen shows the pending notification count |
| R-39 Mac relay to Claude Code headless, only if [decision 32](04-decisions-and-checklist.md#decisions) says build · Sonnet | `mac/relay/`, `Shared/LLM/RelayClient.swift`, `Shared/Package.swift` | R-17's tests pass with the relay as scorer; verdicts for a 20-unit fixture return within 30 s; with the Mac off, the retell is saved and scored on reconnect; the relay refuses to run if ANTHROPIC_API_KEY is set; if R-15 picked Whisper, the relay scores Whisper's transcript of the fixture audio |
| R-40 Wire in a local model that passed R-20, only for the jobs it passed · Sonnet | `Shared/LLM/LocalModel.swift`, `Shared/Package.swift` | R-17's tests pass with the local model as scorer and queued retells get scored; for a chat job, R-32's checks pass with live answers; a transcript-only `call-result.json` is scored on import; runs on the iPhone only while [decision 32](04-decisions-and-checklist.md#decisions) is on hold |

### Phase 3: Reader extras

| id | files | done-check |
|---|---|---|
| R-29 ClawCall API check · Sonnet | `docs/clawcall-setup.md` | The doc answers with evidence: custom script per call, transcript returned and how fast, retention, caller ID, calls from a Mac script; one manual test call is placed from the Mac. If the per-call script or the transcript is missing: no quiz calls, R-34 and check 11 drop their quiz parts, and [decision 24](04-decisions-and-checklist.md#decisions) is asked again |
| R-30 Physical-book session S8 · Sonnet | `Reader/PaperBook/PaperSessionView.swift`, `prompts/session-idea-units.txt`, `docs/device-capabilities.md` | 20 minutes read aloud with the screen locked leaves no transcript gap over 60 s; after calibration the page estimate is within 10% of your hand count; a force-quit loses 60 s or less; `scripts/lint-prompts.py` passes on the prompt; the doc notes whether live transcription runs while locked and whether recording works while the Big Brain recorder runs |
| R-31 Page-photo fallback · Sonnet | `Reader/PaperBook/PagePhotoReader.swift` | On 10 page photos, the printed page number is read right on at least 9 |
| R-32 Talk about the text S9 · Sonnet | `Reader/Chat/ChatSheet.swift`, `prompts/talk-about-text.txt` | A test inspects the packed context: no unit after the current position; each answer shows its book or outside tag (fixture answers until a chat model exists); a saved "More" card appears in the next review; `scripts/lint-prompts.py` passes on the prompt |
| R-33 Status export and call-result import · Sonnet | `Reader/Calls/StatusExport.swift`, `Reader/Calls/CallResultImport.swift`, `schemas/status-export.schema.json`, `schemas/call-result.schema.json` | Writes through J-09's `Shared/Export/FolderExporter.swift`; `status.json` validates after every session; a fixture `call-result.json` with verdicts dropped into the folder is imported on next open and moves that topic's due time |
| R-34 Mac call runner · Sonnet | `mac/calls/call-runner.py`, `mac/calls/reader-calls.plist`, `tests/calls/fixtures/` | A dry run on 6 fixtures (push, boost, quiz, nothing due, quiet hours, a second call the same day) prints the right decision for each; a quiz-transcript fixture comes back with verdicts in `call-result.json` from the live scorer, or with the transcript only while none runs on the Mac; one live call of each type reaches the phone |
| R-35 AlarmKit missed-slot alarm · Sonnet | `Reader/Alarm/MissedSlotAlarm.swift` | Off by default; when on, a planned slot with no session sounds one alarm 30 min later with Focus on and the ringer silent; a session before then cancels it |
| R-36 Session shield, paid account · Sonnet | `Reader/Focus/SessionShield.swift`, `Reader/Extensions/Shield/`, `docs/device-capabilities.md` | During a session every other app shows the shield; ending the session lifts it; the doc notes whether this works on a dev-signed build |
| R-37 Recorder fallback ([decision 18](04-decisions-and-checklist.md#decisions)), only when you start a Punjabi or Hindi text and R-15 found that locale missing · Sonnet | `mac/recorder-bridge/` | For one Punjabi or Hindi session, the Mac transcript for the session window reaches the app within 10 minutes |
| R-38 Big Brain export, only if [decision 14](04-decisions-and-checklist.md#decisions) says yes · Sonnet | `Reader/Export/BigBrainExport.swift`, `schemas/bigbrain-export.schema.json`, `docs/big-brain-reader-intake.md` | The day's file in the shared folder is rewritten after each session and validates against the schema; a day with no session has no file; the intake doc names folder, file name, schema and update rule; no Big Brain file changes |

## Checks

End-to-end, on the iPhone, with the pilot book. Each check passes or fails.

Checks 4, 10 and 11 (and the scoring part of 12) need a live scorer or chat model: the relay ([decision 32](04-decisions-and-checklist.md#decisions)) or an R-20-passed local model. Until one exists, checks 4 and 11 use your own verdicts on S6, check 12 schedules the retell you mark offline at once (its provisional-open and score-on-reconnect parts wait), and check 10 waits. Check 5 compares your verdicts with the Sonnet verdicts from the R-19 [batch session](#recall-scoring-gap-and-root-cause). Once a live scorer exists, checks 4, 5, 11 and 12 run again with it.

1. **Pipeline.** Run the pipeline session on a second PDF. The validator exits 0, the spot check finds at most 2 bad of 20, and S3 shows the level and topic counts the validator reported.
2. **No skipping.** Inside a topic, try swipe, scroll, tap-anywhere and the outline. The paragraph does not change before Continue unlocks, later levels stay LOCKED, and the only ways forward are reading or a test-out.
3. **Test-out.** On a locked topic, answer half the items wrong: it stays locked and reading resumes. In the next slot, pass at 90% or more: it unlocks and its units show a first review at least a day away.
4. **Gap analysis.** Read a topic, then retell it leaving out two paragraphs on purpose. Their units appear under Missed with a hint card; "Retell again" shows only those paragraphs; a full second retell moves them to Recalled.
5. **Scorer gate.** `scripts/score-checkset.py` prints agreement, kappa and false credit for 40 retells and states pass or fail. While it fails, S6 still shows flip toggles.
6. **Spacing.** With the debug clock, read a topic in the 07:30 slot. Today shows the next session at 11:30; after a Good retell the next is 07:30 the following day; after one more the topic shows "in N days". Set G to 2 h: the slot times change at once.
7. **Notifications.** After 3 days of real use, the debug screen shows 10 or fewer pending. No ping arrived between 22:30 and 07:30 Pacific. Ignoring two slots stops pings until the next morning's first slot.
8. **Missed day.** Skip a full day. The next session holds at most 5 reviews and then the new topic; the rest appear in the following slot.
9. **Physical book.** Read aloud for 10 minutes with the phone locked on a table. The transcript exists; after the first calibration the page estimate is within 10% of a hand count; a force-quit mid-session keeps all but the last minute.
10. **Talk.** Ask about the current paragraph, then about a later chapter. The first answer is tagged "from the book"; the second says it comes later. Tap More and save the card; it appears in the next review.
11. **Calls.** Over one week: no call in quiet hours and at most one a day. In a live quiz call, answer two questions; opening the app within 10 minutes after the call shows it counted as a review, with verdicts.
12. **Offline.** In airplane mode, read and retell a topic: the retell is saved, the next topic opens provisionally and pings stay on plan. After reconnecting, the retell is scored and the topic is scheduled from its grade. Only scoring, chat, calls and folder sync waited.
13. **No API key.** ANTHROPIC_API_KEY is unset on the Mac and a script search of the repo for key patterns finds none. Once R-39 exists, the relay also refuses to start with the key set.
