# Gamification / Visual Progress + Multimodal (Spoken-Answer) Capture — Research Ground Truth

Input for the engagement/UX specialist and the multimodal-capture specialist.
Date of research: 2026-07-02.

**Method / verification note.** Unlike earlier L1 files in this folder, WebFetch WAS
partially available this session: `github.com` and `raw.githubusercontent.com` pages were
fetched and read directly (tagged [FACT — fetched]). Several official hosts still returned
HTTP 403 through the egress proxy (`docs.github.com`, `blog.duolingo.com`, `ankiweb.net`,
`ets.org`, `platform.openai.com`, `docs.aws.amazon.com`); claims from those are grounded in
WebSearch result content attributed to the cited URL and tagged [FACT — via search snippet],
which means "stated in retrieved search content attributed to that URL," a weaker grade than
a direct fetch. Design guidance is tagged [ESTIMATE]. Nothing here is invented.

---

## PART A — GAMIFICATION + VISUAL PROGRESS

### A1. The owner's spec (binding requirement — document faithfully, do not redesign)

[FACT — owner's verbatim brief, `../OWNER_BRIEF.md` §1/§2a-6] The owner specified,
unusually concretely:

| Element | Owner requirement |
|---|---|
| Layout | A calendar view — one month OR one week — with a box per day |
| Empty state | Box is **white** (owner: "first it's empty, let's say white with one in it") |
| Filling | As the day's tasks/revision complete, the box **fills like a downloading bar, green** |
| Complete state | 100% done → **tick** (checkmark) replaces/overlays the fill |
| Partial state | Not fully done → **show the percentage**, in **5% increments** ("like 10, 15 with 5% increment") |
| Scope of visuals | "visual representation of the course, visual representation of the days, tasks which have been completed, showing percentage also" |

Open points the owner did NOT specify (hold as [UNKNOWN], resolve at build time):
- Rounding direction for the 5% steps (67% → 65 or 70?). [ESTIMATE — recommend **floor**
  (67→65), so the tick and "100" appear only at true completion; medium-high confidence
  this matches intent since the tick is the owner's distinguished terminal state.]
- Whether the "one in it" means a day-number label inside the empty box (most natural
  reading) or something else. [ESTIMATE — day-number label; high confidence from context.]
- Whether past incomplete days stay at their frozen percentage or gray out. [UNKNOWN]

Derived data model per day-box [ESTIMATE — design]:
`{date, tasks_total, tasks_done, weight_done?, pct = floor((done/total)*20)*5, state: empty|partial(pct)|complete}`
— 21 possible fill states (0,5,…,95, tick). Task weighting (a 40-min revision vs a 2-min
check-in) is a build-time decision; unweighted count is the literal reading of the spec.

### A2. Prior art the spec rhymes with

**GitHub contribution graph (the canonical "calendar of colored day-boxes").**
- [FACT — via search snippet] GitHub's profile shows a year of day-squares colored by
  contribution count; the color scale is **relative to the user's own activity**, computed
  by quartiles over the displayed window: after excluding zero days, contribution counts are
  split into 4 quartiles → 4 green intensity levels, and the scale **recalibrates** as the
  rolling window moves (official docs page returned 403; grounded in GitHub Community
  discussion https://github.com/orgs/community/discussions/176081 and
  https://github.com/orgs/community/discussions/23261; official doc URL for later
  verification: https://docs.github.com/en/account-and-profile/setting-up-and-managing-your-github-profile/managing-contribution-settings-on-your-profile/viewing-contributions-on-your-profile).
- Key difference from the owner's spec: GitHub encodes **relative volume** by color
  intensity; the owner wants **absolute completion** (% of that day's assigned tasks) by
  fill height + numeral. The owner's version is better for a task system because each day
  has a defined denominator. [ESTIMATE — analysis, high confidence]

**Anki Review Heatmap add-on (proof this pattern works in spaced-repetition study).**
- [FACT — fetched https://github.com/glutanimate/review-heatmap] "Review Heatmap" by
  Aristotelis P. (Glutanimate), AGPLv3, adds "a heatmap graph to Anki's main window which
  visualizes past AND FUTURE card review activity," GitHub-contribution-style; shows
  **streak** data; clicking a day shows the cards reviewed; built on d3.js + cal-heatmap.
  AnkiWeb listing (403 this session, for verification): https://ankiweb.net/shared/info/1771074083.
- Notable feature to copy: it plots **future scheduled load**, not just past completion —
  useful for the owner's scheduling system (show tomorrow's forecast burden). [ESTIMATE]

**Duolingo streaks + streak freeze.**
- [FACT — via search snippet, blog.duolingo.com 403] Duolingo's streak = consecutive days
  with at least one lesson; resets to zero on a missed day unless a **streak freeze** is
  equipped in advance (bought in the shop; freezes are consumed one per missed day; must be
  acquired before the miss, not retroactively). Sources:
  https://blog.duolingo.com/how-duolingo-streak-builds-habit/,
  https://www.duolingo.com/help/what-is-a-streak, https://duolingo.fandom.com/wiki/Streak.
- [FACT — via search snippet attributed to Duolingo's own blog] Duolingo states learners
  who reach a **7-day streak are 2.4x more likely** to continue the next day than learners
  without a streak, and explicitly says the mechanic taps **loss aversion**
  (https://blog.duolingo.com/how-streaks-keep-duolingo-learners-committed-to-their-language-goals/,
  https://blog.duolingo.com/how-duolingo-streak-builds-habit/). Vendor claim; treat the
  2.4x as Duolingo's number, not independent.

**Course-level visual representations.**
- [ESTIMATE — pattern synthesis, medium confidence; no single page fetched] Standard
  patterns for "visual representation of the course": (a) syllabus-progress percentage
  (units mastered / total units, same 5%-step display can be reused at course level);
  (b) unit map / path (ordered nodes with locked→in-progress→done states, Duolingo-path
  style); (c) per-topic mastery bars (Khan-Academy-style mastery levels). For this system
  the course % should be derived from the KB coverage model (units × mastery state from the
  learner model), so day-view (effort) and course-view (mastery) are DIFFERENT metrics and
  should not be conflated. [ESTIMATE — design recommendation]

### A3. The psychology: why the owner's design should work

**Goal-gradient effect (progress bars accelerate effort near completion).**
- [FACT] Kivetz, Urminsky & Zheng (2006), "The Goal-Gradient Hypothesis Resurrected,"
  *Journal of Marketing Research* 43(1):39–58: in a real café reward program, customers
  purchased **more frequently the closer they were** to the free-coffee reward; song-raters
  visited more often, rated more per visit, and persisted longer as they approached the
  reward; **illusionary progress** (a 12-stamp card with 2 pre-stamped, vs a 10-stamp card)
  also accelerated behavior. DOI page: https://journals.sagepub.com/doi/abs/10.1509/jmkr.43.1.39;
  author PDF: https://home.uchicago.edu/ourminsky/Goal-Gradient_Illusionary_Goal_Progress.pdf.
  (The title's "resurrected" refers to Hull's 1930s animal goal-gradient work, which the
  paper revives for humans. [FACT — from the paper's own framing in retrieved abstracts])
- Implication for the day-box: a visible partial fill (35%… 60%…) exploits goal-gradient —
  the nearer the bar is to full, the stronger the pull to finish today's tasks. The 5%-step
  numeral makes progress feel granular and always-moving. [ESTIMATE — direct application,
  high confidence]

**Habit formation.**
- [FACT] Lally, van Jaarsveld, Potts & Wardle (2010), "How are habits formed: Modelling
  habit formation in the real world," *European Journal of Social Psychology*: 96 volunteers
  repeated a chosen daily behavior in a stable context for 12 weeks; automaticity followed
  an asymptotic curve; median-model time to plateau averaged **66 days, range 18–254**
  (https://onlinelibrary.wiley.com/doi/abs/10.1002/ejsp.674; UCL summary:
  https://www.ucl.ac.uk/news/2009/aug/how-long-does-it-take-form-habit).
  Design consequence: the calendar must survive ~2+ months of daily use before the habit is
  self-sustaining — early-phase streak protection matters most. [ESTIMATE]
- Cue → routine → reward ("habit loop"): [ESTIMATE — the three-part framing is popularized
  by Duhigg, *The Power of Habit* (2012), and is the frame Duolingo's habit posts use per
  search snippets; the book itself was not fetched, so tagged ESTIMATE with high confidence
  on attribution]. Mapping: push notification / opening the app = **cue**; the day's task
  queue = **routine**; the green fill advancing + tick = **reward**. Same-context repetition
  (fixed study time) is what Lally 2010 actually measured, so the scheduler should default
  to a consistent daily slot. [ESTIMATE — design]

**What the literature warns about (build these guardrails in).**
- Overjustification effect: [FACT] Lepper, Greene & Nisbett (1973), *JPSP* 28(1):129–137 —
  children promised an extrinsic reward for an activity they already enjoyed showed
  **reduced subsequent intrinsic interest**; unexpected rewards did not cause the drop
  (PDF: https://web.mit.edu/curhan/www/docs/Articles/15341_Readings/Motivation/Lepper_et_al_Undermining_Childrens_Intrinsic_Interest.pdf;
  overview: https://en.wikipedia.org/wiki/Overjustification_effect).
- [FACT] Deci, Koestner & Ryan (1999) meta-analysis of 128 experiments, *Psychological
  Bulletin* 125(6):627–668: expected tangible rewards significantly undermined free-choice
  intrinsic motivation (engagement-contingent d = −0.40, completion-contingent d = −0.36,
  performance-contingent d = −0.28)
  (https://home.ubalt.edu/tmitch/642/articles%20syllabus/Deci%20Koestner%20Ryan%20meta%20IM%20psy%20bull%2099.pdf).
  Guardrail: keep the visuals as **informational feedback on progress** (which the same
  literature treats far more favorably than tangible rewards) rather than adding
  points/prizes for studying. [ESTIMATE — application, high confidence]
- Streak anxiety / demotivation-on-break: [FACT — via search snippets] Duolingo itself
  frames streaks via loss aversion (see A2); a University of Oulu thesis on Duolingo streak
  motivation found broken streaks can flip the feature into a demotivator ("having your
  motivational feature be the reason a learner is completely demotivated should raise
  concerns," https://oulurepo.oulu.fi/bitstream/handle/10024/54117/nbnfioulu-202502121605.pdf);
  abundant first-person accounts of quitting entirely after a long streak breaks
  (e.g. https://medium.com/babel/why-i-quit-duolingo-after-a-588-day-streak-dd38de2d8d79).
  Guardrails for this system [ESTIMATE — design]: (1) offer freeze/repair mechanics rather
  than hard resets; (2) show a secondary long-horizon metric (e.g. "days active in last 30")
  that a single miss cannot zero; (3) never let a broken streak reduce actual scheduling
  quality — the SRS backlog view must stay non-punitive.

---

## PART B — MULTIMODAL CAPTURE (spoken answers, any language, queued for AI analysis)

### B1. Owner requirement (binding)

[FACT — owner's verbatim brief, `../OWNER_BRIEF.md` §2a-7] The user shouldn't always have
to type: e.g., 10 questions answered **by speaking, in whatever language**; answers are
**saved and queued** for later analysis by AI — "the local AI or the cloud, later on
decision" (the local-vs-cloud choice is explicitly **deferred by the owner**; the
architecture must therefore keep both paths open behind one interface). Voice
**note-making** is also named.

### B2. Speech-to-text — LOCAL options

**OpenAI Whisper (the base model family).**
- [FACT — fetched and machine-counted from https://raw.githubusercontent.com/openai/whisper/main/whisper/tokenizer.py]
  The `LANGUAGES` dict in Whisper's tokenizer contains exactly **100 entries** (en, zh, de,
  es, ru, … yue/Cantonese). The commonly cited "~99 languages" figure predates the addition
  of Cantonese (`yue`) with large-v3. Multilingual recognition + speech translation +
  language identification are core features [FACT — fetched https://github.com/openai/whisper].
  Accuracy varies strongly by language (the repo publishes per-language WER breakdowns for
  large-v2/v3) [FACT — fetched, same page]. For the owner's "any language" requirement this
  is the coverage ceiling for the local path.

**whisper.cpp (local, CPU-first).**
- [FACT — fetched https://github.com/ggml-org/whisper.cpp] Plain C/C++ implementation, no
  dependencies; runs on macOS (Intel/ARM), iOS, Android, Linux, FreeBSD, Windows, Raspberry
  Pi, WebAssembly, Docker; acceleration via ARM NEON/Metal/Core ML, x86 AVX, NVIDIA CUDA,
  AMD ROCm, Vulkan, OpenVINO; integer quantization; VAD; word-level timestamps.
- [FACT — fetched, same README] Memory/disk by model: tiny 75 MiB disk / ~273 MB RAM;
  base 142 MiB / ~388 MB; small 466 MiB / ~852 MB; medium 1.5 GiB / ~2.1 GB;
  large 2.9 GiB / ~3.9 GB. So the full-quality multilingual model runs in <4 GB RAM on CPU —
  feasible on a normal laptop; smaller models fit a phone. [last clause ESTIMATE — direct
  inference from the table, high confidence]

**faster-whisper (local, GPU-or-CPU, fastest common path).**
- [FACT — fetched https://github.com/SYSTRAN/faster-whisper] Reimplementation of Whisper on
  the CTranslate2 inference engine; claims **up to 4x faster than openai/whisper at same
  accuracy with less memory** (README benchmark: 13 min audio in 1m03s fp16 vs 2m23s for
  openai/whisper on GPU); GPU path requires CUDA 12 + cuDNN 9 (cuBLAS); CPU-only also
  supported.
- Local-path recommendation [ESTIMATE — design, medium-high confidence]: batch (overnight)
  transcription has no latency constraint, so even CPU-only whisper.cpp `medium`/`large`
  is sufficient; use faster-whisper if an NVIDIA GPU is present.

### B3. Speech-to-text — CLOUD options (existence + language support)

| Provider / API | Exists | Language support | Grounding |
|---|---|---|---|
| Google Cloud Speech-to-Text | [FACT] | **125+ languages** (vendor claim on product page) | https://cloud.google.com/speech-to-text, https://docs.cloud.google.com/speech-to-text/docs/speech-to-text-supported-languages [via search snippet] |
| Microsoft Azure AI Speech | [FACT] | Large locale table for real-time/fast/batch STT; **exact current count [UNKNOWN]** — the table page (https://learn.microsoft.com/en-us/azure/ai-services/speech-service/language-support?tabs=stt) was not directly fetchable this session | [via search snippet] |
| Amazon Transcribe | [FACT] | Official supported-languages page exists (https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html — 403 this session); an AWS blog documents 31 languages as of its date; **current count [UNKNOWN]** | [via search snippet] |
| OpenAI Audio API | [FACT] | Models `whisper-1` plus newer `gpt-4o-transcribe`-family; 25 MB per-file upload limit for whisper-1; help center lists ~57 officially supported transcription languages (Whisper's 100 minus low-accuracy ones) | https://developers.openai.com/api/docs/guides/speech-to-text, https://help.openai.com/en/articles/7031512-audio-api-faq [via search snippet; direct fetch 403] |

Also existent and commonly used: Deepgram, AssemblyAI (specialist STT vendors).
[ESTIMATE — well-known vendors; their pages were not fetched this session, so no numbers
are claimed for them.]

### B4. The queue architecture (record → analyze later) [ESTIMATE — design, whole section]

The owner's "saved and queued for later analysis" implies an **asynchronous pipeline**, which
is the cheap and robust choice (no real-time ASR needed):

1. **Record**: capture per-question audio (one file per answer). Store lossless-enough
   compressed audio (16 kHz mono is what Whisper consumes anyway).
2. **Store + enqueue**: write file + a metadata row:
   `{answer_id, question_id, session_id, recorded_at, duration, language_hint?, status: queued}`.
   A directory-of-files + SQLite table is fully sufficient at single-user scale; a message
   broker is overkill.
3. **Batch transcribe** (nightly, aligned with the owner's "every night analysis mode"):
   worker drains `status=queued` through a **TranscriberInterface** with two
   implementations — `LocalWhisper` (whisper.cpp / faster-whisper) and `CloudSTT` — because
   the owner deferred local-vs-cloud; the decision then becomes a config flag, not a
   rewrite. Whisper's language-identification handles "whatever language" without asking
   the user to declare it (store detected language with confidence).
4. **LLM-grade against rubric**: transcript + the question's model answer/rubric → LLM
   grader → `{score, subscores per rubric dimension, error notes, misconception tags}`.
   Grade the CONTENT from the transcript; do not grade pronunciation unless the course is
   TOEFL-speaking, where delivery matters (see B5).
5. **Write back to learner model**: grading output updates the per-item mastery state and
   weakness map; the scheduler consumes it next morning. Keep audio + transcript + grade
   linked for audit ("why did it say I was wrong?").
6. **Failure handling**: statuses `queued → transcribed → graded | failed(reason)`; failed
   items retry next night; nothing is silently dropped.

### B5. Precedent: grading spoken answers automatically is established practice

- [FACT — via search snippets; ets.org direct fetch 403] ETS — the maker of TOEFL — has
  operated **SpeechRater**, an automated scoring engine for spontaneous non-native speech,
  since ~2008 (ETS Research Report "Automated Scoring of Spontaneous Speech Using
  SpeechRater v1.0," https://www.ets.org/research/policy_research_reports/publications/report/2008/hukv.html;
  "Automated Scoring of Nonnative Speech Using the SpeechRater v5.0 Engine," *ETS Research
  Report Series* 2018, https://onlinelibrary.wiley.com/doi/full/10.1002/ets2.12198).
- [FACT — via search snippets] Operational TOEFL iBT Speaking is scored by a **combination
  of human raters and SpeechRater**: humans rate holistically (content/meaning/language),
  the engine rates analytically (delivery/language-use features)
  (secondary confirmations: https://www.toeflresources.com/speechrater/,
  https://www.myspeakingscore.com/blog/how-toefl-speaking-is-scored). Exact current
  human/machine weighting: [UNKNOWN — official ETS scoring page not fetchable this session].
- Implication: LLM-grading of transcribed spoken answers against a rubric is not exotic —
  the exam this system targets already machine-scores speech. For TOEFL-speaking practice
  specifically, delivery features (fluency, pause patterns) live in the AUDIO, not the
  transcript, so keep the audio path available to a future delivery-scoring step.
  [ESTIMATE — design consequence, high confidence]

### B6. Voice note-making pipeline [ESTIMATE — design, short]

Same pipeline as B4 minus grading: record → transcribe → LLM post-process (cleanup,
punctuate, structure into note/flashcard candidates per the flashcard-quality rules in
`flashcards_and_book_decomposition.md`) → human-confirm before entering the KB. Whisper's
word timestamps allow linking each note back to its audio moment. [ESTIMATE]

### B7. Privacy for cloud audio (FACT-level considerations)

- [FACT — via search snippets attributed to EDPB] The EDPB's Guidelines 02/2021 on Virtual
  Voice Assistants treat voice data as inherently personal data; it becomes **Article 9
  "special category" biometric data when processed to uniquely identify a person**
  (speaker-ID, voice embeddings, diarization-for-identity); plain transcription without
  identification purpose does not by itself trigger Article 9
  (https://www.edpb.europa.eu/ — guidelines PDF; secondary:
  https://summitnotes.app/blog/gdpr-voice-recordings-biometric-data/). Design consequence:
  **never run speaker identification** on the recordings; single-user system has no need.
  [last clause ESTIMATE — design]
- [FACT — via search snippets attributed to openai.com] OpenAI API policy: API inputs/outputs
  are **not used for model training by default** (since 2023-03-01, opt-in only); may be
  retained up to 30 days for abuse monitoring; zero-data-retention available for qualifying
  orgs (https://openai.com/enterprise-privacy/, https://openai.com/policies/how-your-data-is-used-to-improve-model-performance/).
  Other major clouds publish comparable data-use terms; their specific current terms:
  [UNKNOWN — not fetched this session; verify the chosen vendor's page at decision time].
- Practical posture given the owner deferred local-vs-cloud [ESTIMATE — recommendation]:
  default to **local** transcription (whisper.cpp covers the need at zero marginal cost and
  the audio never leaves the machine); if cloud is chosen later, that is exactly one adapter
  swap (B4 step 3), plus recording an explicit consent note and the vendor's retention terms.

---

## Sources (grouped; fetch grade noted)

**Directly fetched:** github.com/openai/whisper · raw.githubusercontent.com/openai/whisper/main/whisper/tokenizer.py (LANGUAGES counted programmatically = 100) · github.com/ggml-org/whisper.cpp · github.com/SYSTRAN/faster-whisper · github.com/glutanimate/review-heatmap

**Via search snippets (host 403 or not fetched; URL cited for later verification):**
journals.sagepub.com/doi/abs/10.1509/jmkr.43.1.39 (Kivetz 2006) · home.uchicago.edu/ourminsky/Goal-Gradient_Illusionary_Goal_Progress.pdf · onlinelibrary.wiley.com/doi/abs/10.1002/ejsp.674 (Lally 2010) · ucl.ac.uk/news/2009/aug/how-long-does-it-take-form-habit · web.mit.edu/.../Lepper_et_al_Undermining_Childrens_Intrinsic_Interest.pdf (1973) · home.ubalt.edu/.../Deci Koestner Ryan meta IM psy bull 99.pdf · en.wikipedia.org/wiki/Overjustification_effect · blog.duolingo.com/how-duolingo-streak-builds-habit/ · blog.duolingo.com/how-streaks-keep-duolingo-learners-committed-to-their-language-goals/ · duolingo.com/help/what-is-a-streak · duolingo.fandom.com/wiki/Streak · oulurepo.oulu.fi (Oulu thesis on streak motivation) · github.com/orgs/community/discussions/176081 and /23261 (contribution-graph quartiles) · docs.github.com viewing-contributions page (403) · ankiweb.net/shared/info/1771074083 (403) · cloud.google.com/speech-to-text · learn.microsoft.com/.../speech-service/language-support · docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html (403) · developers.openai.com/api/docs/guides/speech-to-text (403) · help.openai.com/en/articles/7031512-audio-api-faq · ets.org/.../report/2008/hukv.html (SpeechRater v1.0; 403) · onlinelibrary.wiley.com/doi/full/10.1002/ets2.12198 (SpeechRater v5.0) · toeflresources.com/speechrater/ · myspeakingscore.com/blog/how-toefl-speaking-is-scored · edpb.europa.eu Guidelines 02/2021 on Virtual Voice Assistants · summitnotes.app/blog/gdpr-voice-recordings-biometric-data/ · openai.com/enterprise-privacy/ · openai.com/policies/how-your-data-is-used-to-improve-model-performance/ · medium.com/babel/why-i-quit-duolingo-after-a-588-day-streak
