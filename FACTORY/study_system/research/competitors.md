# Competitor Scan — Spaced-Repetition / Study Systems and Exam-Prep Verticals

Input for the system-loops specialist. Survey of public pages/docs only.
Date of research: 2026-07-02.

**Method / verification caveat.** Direct page fetches (WebFetch) were blocked by this
session's egress policy for all external hosts; every claim below was verified through
WebSearch result content, which returns server-side-retrieved summaries of the listed
pages. [FACT] tags therefore mean "stated in retrieved search content attributed to the
cited URL," with official-domain URLs preferred. Where only third-party review sites
carried a number (some prices), the claim is tagged [ESTIMATE] with the basis named.
No user counts, revenue figures, or pass rates are asserted anywhere except where a
vendor publicly claims one (marked as vendor claim).

---

## 1. Anki (+ FSRS)

- **Core mechanism**: [FACT] Free, open-source (AGPLv3) flashcard app with spaced
  repetition; desktop (Win/macOS/Linux) free, AnkiDroid free, AnkiWeb sync free; the
  iOS app AnkiMobile is a one-time $24.99 purchase that funds development
  (https://apps.ankiweb.net/, https://apps.apple.com/us/app/ankimobile-flashcards/id373493387).
- **Scheduling**: two algorithms.
  - Legacy: SM-2 variant with per-card "ease factor" (Again −20%, Hard −15%, Good 0,
    Easy +15%, floor 130%). [FACT] per RemNote's public writeup of Anki SM-2
    (https://help.remnote.com/en/articles/6026144-the-anki-sm-2-spaced-repetition-algorithm)
    and community docs (https://kuroahna.github.io/anki_srs_kai/guide/issuesWithAnkiSM2.html).
  - FSRS (built in since v23.10): [FACT] models each card as a 3-component memory state —
    Difficulty, Stability (days for retrievability to fall 100%→90%), Retrievability —
    and schedules the next review when predicted retrievability drops to a user-chosen
    **desired retention** (default 90%, permitted range ~0.70–0.99); parameters can be
    optimized per-preset from the user's own review history
    (https://docs.ankiweb.net/deck-options.html,
    https://faqs.ankiweb.net/what-spaced-repetition-algorithm,
    https://github.com/open-spaced-repetition/fsrs4anki).
  - [FACT] FSRS Difficulty mean-reverts toward baseline on repeated correct answers,
    which is what eliminates SM-2's ease hell
    (https://expertium.github.io/Algorithm.html, via search summary).
- **Content model**: user-authored cards; note types with fields/templates; media
  (audio/images), cloze deletion, image occlusion; huge shared-deck library and add-on
  ecosystem. [FACT] for existence of shared decks and add-ons (https://apps.ankiweb.net/,
  https://ankiweb.net/shared/).
- **Gamification**: essentially none native (no streaks, leagues, points); add-ons fill
  the gap. [ESTIMATE] high confidence — no gamification features appear anywhere in the
  official manual or site.
- **Multimodal**: [FACT] audio/image/video on cards, TTS support (Anki manual,
  https://docs.ankiweb.net/).
- **Pricing**: [FACT] free everywhere except iOS ($24.99 one-time)
  (https://apps.apple.com/us/app/ankimobile-flashcards/id373493387).
- **Documented faults**:
  - [FACT] "Ease hell": SM-2 ease sinks fast (Again/Hard) and recovers slowly (only
    Easy raises it), so cards get stuck at 130% and return far too often
    (https://kuroahna.github.io/anki_srs_kai/guide/issuesWithAnkiSM2.html,
    https://readbroca.com/anki/ease-hell/). Fixed by FSRS, but FSRS must be enabled.
  - [FACT] Steep learning curve / dated UI; defaults widely considered poor; effective
    use historically required add-ons and tutorials (community thread
    https://forums.ankiweb.net/t/learning-curve-for-anki/7574; critique
    https://www.brainscape.com/academy/does-anki-work/ — note: competitor-authored).
  - [FACT] Shared decks are unvetted; no quality signal on accuracy/completeness
    (same Brainscape critique; also a recurring forum theme). [ESTIMATE] this is the
    biggest content-model weakness: the scheduler is world-class, the content supply
    chain is unmanaged.

## 2. SuperMemo

- **Core mechanism**: [FACT] the original spaced-repetition lineage; current Windows
  desktop uses Algorithm SM-18/SM-19 (SuperMemo 19 improved post-lapse stability
  estimation) (https://help.supermemo.org/wiki/Features,
  https://supermemo.guru/wiki/Algorithm_SM-17, https://en.wikipedia.org/wiki/SuperMemo).
- **Scheduling**: [FACT] two-component memory model (stability/retrievability) with
  per-item forgetting curves — conceptual ancestor of FSRS (https://supermemo.guru/wiki/Algorithm_SM-17).
- **Content model**: [FACT] **incremental reading** (since SuperMemo 10, 2000): import
  articles, read incrementally, extract fragments, convert to cloze items, priority
  queue over thousands of articles (https://help.supermemo.org/wiki/Incremental_reading,
  https://super-memory.com/archive/help16/adv_read.htm).
- **Gamification**: none meaningful. [ESTIMATE] high confidence from feature docs.
- **Multimodal**: [FACT] HTML components, images, audio in items (help.supermemo.org
  feature list). [UNKNOWN] exact current multimedia limits — feature matrix pages not
  directly fetchable this session.
- **Pricing**: [FACT] desktop SuperMemo has been sold ~$66 (SuperMemo 18 price point per
  retrieved results); supermemo.com web edition is freemium (https://en.wikipedia.org/wiki/SuperMemo).
  [UNKNOWN] current SuperMemo 19 price — supermemo.com blocked, not verified.
- **Documented faults**:
  - [FACT] Steep learning curve, acknowledged by the vendor itself ("optimized to make
    a life of a pro easy... makes life of beginners hard")
    (https://supermemopedia.com/wiki/SuperMemo_is_most_unfriendly_application).
  - [FACT] Windows-only desktop; dated, cluttered UI
    (https://www.supermemopedia.com/wiki/I_hate_SuperMemo_in_Windows_10).
  - [FACT] The author's supporting wiki (supermemo.guru) has been publicly accused of
    overreaching/pseudoscientific claims
    (https://supermemopedia.com/wiki/SuperMemo_Guru_is_a_low-IQ_pseudoscientific_blog).
  - [ESTIMATE] Net lesson: most sophisticated algorithm + incremental reading, near-zero
    adoption outside enthusiasts because onboarding cost is extreme.

## 3. Quizlet

- **Core mechanism**: [FACT] user-generated flashcard "sets" with multiple study modes:
  Learn (adaptive), Write, Test, Match (timed game), Flashcards
  (https://quizlet.com; mode list corroborated across retrieved reviews).
- **Scheduling**: Learn mode adapts within a session (focuses on missed items, moves
  through rounds); it is mastery/round-based, not calendar spaced repetition.
  [ESTIMATE] medium-high confidence — Quizlet's own marketing describes adaptivity but
  publishes no interval algorithm; no official algorithm doc found.
- **Content model**: term/definition sets, user-generated at massive scale; AI tools and
  textbook solutions added later. [FACT] for feature existence (retrieved reviews of
  quizlet.com features).
- **Gamification**: Match leaderboards per set; the Gravity game was **removed
  entirely** [FACT] (https://www.mintdeck.app/blog/quizlet-paywall-free-alternative).
- **Multimodal**: images and audio (TTS) on cards. [ESTIMATE] high confidence, standard
  documented features.
- **Pricing**: [FACT] Quizlet Plus ≈ $35.99/yr ($2.99/mo billed annually, $7.99/mo
  monthly); a "Plus Unlimited" tier (~$44.99/yr) removes monthly caps (3 practice
  tests / 20 Learn sessions on plain Plus) — per multiple retrieved 2026 pricing
  writeups (https://aistudymaster.com/quizlet-plus-cost/,
  https://studyguides.com/articles/how-much-is-quizlet-plus-pricing-plans-and-free-alternatives).
  [UNKNOWN] exact current official tier matrix — quizlet.com not directly fetchable.
- **Documented faults**:
  - [FACT] August 2022: Learn and Test modes were moved behind the paywall (free users
    get ~5 Learn rounds and 1 practice test per set), producing sustained public
    backlash from students ("predatory... preys on broke students")
    (https://www.ntdaily.com/opinion/quizlet-s-paywalls-place-priority-on-profits-over-pupils/article_848d54c6-4eca-11ef-b6bd-bba8eff900d3.html,
    https://pawebpage.com/3269/opinions/quizlets-paywall-proves-that-students-are-its-last-priority/).
  - [FACT] Critics note the free features left are the passive ones (flip cards, match)
    while the evidence-based active-recall features are paid
    (https://www.mintdeck.app/blog/quizlet-paywall-free-alternative).
  - [ESTIMATE] Structural fault: no true long-term scheduler; "mastery" is per-session,
    so retention decays between cram sessions.

## 4. Duolingo (gamification reference model)

- **Core mechanism**: [FACT] path of bite-sized lessons per language; freemium with ads
  (https://www.duolingo.com).
- **Streak mechanics** (describe-the-mechanics brief): [FACT] streak = consecutive days
  with ≥1 completed lesson before local midnight; resets to 0 on a missed day unless a
  **Streak Freeze** (bought in advance with gems, max 2 equipped, auto-consumed one per
  missed day) covers it. Duolingo's own blog frames this as habit research: flexibility
  ("slack") beats rigid rules for goal persistence
  (https://blog.duolingo.com/how-duolingo-streak-builds-habit/,
  https://blog.duolingo.com/how-to-keep-your-streak-on-vacation/).
- **Leagues/leaderboards**: [FACT] 10 tiers (Bronze → Diamond); each week ~30 users who
  started earning XP at similar times are grouped; ranked purely by weekly XP; top ranks
  promote, bottom (~16th–20th) demote; resets Mondays; Diamond has no promotion and
  feeds a Diamond Tournament for top finishers
  (https://blog.duolingo.com/duolingo-leagues-leaderboards/,
  https://www.duolingo.com/help/leaderboards-and-league).
- **Scheduling**: [ESTIMATE] medium confidence — Duolingo has published research on
  half-life regression for practice item selection, but the consumer product's spacing
  is opaque; practice/review is folded into the path and "mistakes review" rather than a
  user-visible SRS queue.
- **Content model**: closed, professionally authored courses; no user-generated content.
  [FACT] (site structure; course list at duolingo.com).
- **Multimodal**: [FACT] listening/speaking exercises; Max tier adds GPT-4-powered
  Roleplay and Video Call conversation practice
  (https://blog.duolingo.com/duolingo-max/,
  https://www.duolingo.com/help/what-is-duolingo-max).
- **Pricing**: [ESTIMATE] Free / Super ≈ $95.99/yr / Max ≈ $167.99/yr / Family ≈
  $119.99/yr — consistent across several third-party 2026 pricing pages, but regional
  variation exists and duolingo.com pricing was not directly fetchable
  (https://languageappguide.com/pricing/duolingo-cost/).
- **Documented faults**:
  - [FACT] Widely published critiques: streak anxiety and compulsive use patterns;
    leaderboards reward XP-farming over learning; "false fluency" — completing a course
    without conversational ability
    (https://duolingoguides.com/why-duolingo-is-bad/,
    https://theeconomyofmeaning.com/2025/08/25/a-critical-look-at-how-to-make-learning-as-addictive-as-social-media-a-ted-talk-about-duolingo/).
  - [ESTIMATE] The mechanics optimize the *engagement* loop (daily return) superbly and
    the *learning* loop only weakly; the streak is the single most copied retention
    mechanic in edtech, and it is content-agnostic — it certifies showing up, not knowing.

## 5. RemNote

- **Core mechanism**: [FACT] notes-first tool: hierarchical/linked notes where any line
  can become a flashcard (concept/descriptor framing, clozes, image occlusion), plus PDF
  annotation and AI card generation (https://www.remnote.com/,
  https://help.remnote.com/en/articles/6022755-getting-started-with-spaced-repetition).
- **Scheduling**: [FACT] built-in SRS; help docs describe scheduling a card when there
  is ~10% estimated chance you've forgotten it (i.e., ~90% retention target), with an
  Anki-SM-2-style option documented as well
  (https://help.remnote.com/en/articles/6022755-getting-started-with-spaced-repetition,
  https://help.remnote.com/en/articles/6026144-the-anki-sm-2-spaced-repetition-algorithm).
- **Content model**: your own knowledge base; cards live inside notes (no separation of
  notes and decks). [FACT] (remnote.com).
- **Gamification**: minimal (study streak/queue stats). [ESTIMATE] medium confidence.
- **Multimodal**: [FACT] PDFs, images, image occlusion, audio; AI generation from
  transcripts (https://www.remnote.com/).
- **Pricing**: [FACT] free tier with core notes+SRS; Pro subscription unlocks unlimited
  PDFs/image occlusion/AI features (https://www.remnote.com/pricing). [ESTIMATE] Pro
  ≈ $6–10/mo depending on billing — third-party pages disagree; official page not
  directly fetchable, so exact current price unverified.
- **Documented faults**: [FACT] reviews cite steep learning curve, complex hierarchy,
  and time spent structuring notes instead of studying
  (https://studycardsai.com/alternatives/remnote-alternatives, review aggregation).
  [ESTIMATE] classic notes-app failure mode: tinkering with the graph substitutes for
  retrieval practice.

## 6. Memrise

- **Core mechanism**: [FACT] language-vocab app: daily lessons/quizzes with spaced
  review, speed review, "difficult words," pronunciation and listening practice
  (aggregated from retrieved reviews of memrise.com; official feature-page fetch blocked).
- **Scheduling**: spaced repetition of vocab items with review surfacing. [ESTIMATE]
  medium confidence on mechanics detail — Memrise publishes no algorithm spec.
- **Content model**: professionally built official courses; **community/user-generated
  courses were removed from the main product** (moved off to a separate archive),
  sharply reducing available content. [FACT] for the removal
  (https://www.studyfrenchspanish.com/memrise-review/ and multiple 2025–26 reviews).
- **Gamification**: points, streaks, leaderboards. [ESTIMATE] high confidence from
  reviews; official docs not fetched.
- **Multimodal**: [FACT] signature feature "Learn with Locals" — thousands of short
  native-speaker video clips used as listening items; voice-enabled speaking practice;
  AI conversation ("MemBot") in paid tier (retrieved reviews, e.g.
  https://testprepinsight.com/reviews/memrise-review/).
- **Pricing**: [ESTIMATE] Pro ≈ $9/mo, ≈$59/yr, lifetime ≈ $129.99–$199 (varies by
  promo) — consistent across reviews; official pricing page not fetchable.
- **Documented faults**: [FACT] reviews consistently flag: no real grammar instruction,
  heavy multiple-choice recognition (weak production practice), and community-course
  removal as a trust-destroying content decision
  (https://www.studyfrenchspanish.com/memrise-review/,
  https://testprepinsight.com/reviews/memrise-review/).

## 7. Brainscape (confidence-based repetition)

- **Core mechanism**: [FACT] web/mobile flashcards; after each card the learner rates
  **confidence 1–5** ("not at all" → "totally confident"); the rating drives repetition
  spacing — low confidence returns soon, high confidence pushed far out. Vendor calls
  this Confidence-Based Repetition (CBR)
  (https://brainscape.zendesk.com/hc/en-us/articles/13103043051149,
  https://www.brainscape.com/academy/confidence-based-repetition-definition/).
- **Scheduling**: [FACT] repetition intervals are *relative* within the study stream
  (a deck of all-5s still cycles, letting users re-rate honestly), adapting to session
  length rather than a fixed calendar
  (https://brainscape.zendesk.com/hc/en-us/articles/115002384312).
- **Content model**: user-generated decks + paid "certified" expert-curated classes
  (law, MCAT, languages, etc.). [FACT] (brainscape.com marketplace pages via search).
- **Gamification**: progress percentages/mastery meters rather than streak/league
  mechanics. [ESTIMATE] medium confidence.
- **Multimodal**: images/audio on cards. [ESTIMATE] high confidence.
- **Pricing**: [FACT] Pro ≈ $19.99/mo, $59.99/6mo, $95.99/yr, $199.99 lifetime (vendor
  upgrade copy retrieved via search;
  https://www.brainscape.com/academy/brainscape-cognitive-science/ ecosystem).
- **Documented faults**:
  - [FACT] The obvious objection — self-rating is subjective/gameable — is acknowledged
    in their own help center, answered with the "relative repetition" argument
    (https://brainscape.zendesk.com/hc/en-us/articles/115002384312).
  - [ESTIMATE] CBR with 5 buckets is coarser than model-based scheduling (no forgetting
    curve, no per-item difficulty model); comparisons in third-party roundups describe
    it as less precise than SM-2/FSRS. Confidence ≠ competence: metacognitive
    miscalibration is a known failure mode (overconfident novices under-schedule).
  - [ESTIMATE] Interesting virtue to keep: a 1–5 self-rating doubles as a *metacognition
    prompt*, which itself has learning value.

---

## 8. Exam-prep verticals

### 8a. Dental (INBDE — includes foreign-trained dentists)

Context: [FACT] The INBDE (Integrated National Board Dental Examination, run by the
JCNDE/ADA) is the US dental board exam; foreign-trained dentists take the same INBDE as
part of the US licensure/advanced-standing pathway, with extra steps (ECE credential
evaluation, DENTPIN; certificate of eligibility only if not yet graduated)
(https://www.ada.org/resources/careers/licensure/licensure-for-the-international-dentists,
https://bootcamp.com/blog/the-ultimate-inbde-guide-for-foreign-trained-dentists).
There is no separate "foreign-graduate dental exam" — the product market for
international dentists IS the INBDE-prep market. [FACT]

**INBDE Bootcamp (bootcamp.com)**
- [FACT] Question bank of 3,000+ case-based items matched to the exam blueprint; video
  lessons by Dr. Ryan of Mental Dental with bite-sized quizzes after each video;
  progress tracking; image/radiograph question practice; question tagging by
  self-assessed understanding; simulation exams (https://bootcamp.com/inbde).
- [FACT] Pricing tiers around $219 (1 mo) / $319 (3 mo) / $419 (6 mo), frequently
  discounted (https://bootcamp.com/inbde/upgrade via retrieved summaries).
- [FACT] "Pass guarantee": refund if you fail after hitting published usage targets
  (score threshold + % qbank completed + simulations taken); vendor claims 99% of
  target-meeting students pass — vendor claim, not independently verified
  (https://bootcamp.com/inbde).
- [ESTIMATE] Faults: subscription clock pressure (time-boxed access punishes slow/working
  studiers, notably foreign-trained candidates studying part-time); no long-term SRS —
  it is a qbank+video funnel, retention is the user's problem.

**Dental Decks (dentaldecks.com)**
- [FACT] The classic flashcard product for dental boards, now INBDE-oriented: 2,700+
  Q&A flashcards across 16 study areas; formats = online access, printed cards, app
  "coming soon"; ~$429/6mo online, $499/12mo, $599 print+12mo; first-attempt
  money-back pass guarantee (https://dentaldecks.com/dental-decks-inbde/,
  https://store.dentaldecks.com/).
- [ESTIMATE] Faults: flashcards without an adaptive scheduler (fixed decks, self-managed
  review); very high price for static content; "wall-of-text card" style — board-prep
  cards traditionally pack full topic summaries onto one card, the anti-pattern minimal
  information principle exists to fix. (Confidence: medium; based on product format and
  long-standing prep-community characterizations, not a fetched card sample.)

**Mental Dental (free video resource) — verified**
- [FACT] Free YouTube channel (@mentaldental, ~358K subscribers per retrieved summary)
  by "Dr. Ryan," covering INBDE/ADAT/DAT topics (oral radiology, operative, perio, oral
  surgery, pharmacology, head & neck anatomy); slides + extras sold via Mental Dental
  Academy; the same instructor's videos are licensed into INBDE Bootcamp
  (https://www.youtube.com/@mentaldental, https://www.mentaldental.com/videos,
  https://bootcamp.com/inbde).
- [ESTIMATE] Significance: the de-facto free backbone of INBDE study, especially for
  international dentists; a free-content + paid-qbank funnel is the proven model here.

### 8b. TOEFL

**ETS TOEFL TestReady (official)** — verified to exist
- [FACT] Official ETS prep portal consolidating TOEFL iBT prep into one dashboard;
  free tier: sample section items, a rotating free daily practice activity, and a
  survey-generated personalized study plan with progress tracking; paid tier: full
  section practice with immediate scoring/feedback and full mock tests scored within
  24h; speaking feedback covers speech rate, rhythm, pronunciation, grammar, with
  transcripts and exemplars; marketed as AI-personalized
  (https://www.ets.org/toefl/test-takers/ibt/prepare/toefl-testready.html,
  https://www.ets.org/news/press-releases/introducing-toefl-testready-new-era-test-preparation.html).
- [UNKNOWN] itemized TestReady prices — pricing sits behind the account portal; not
  retrievable this session.
- [ESTIMATE] Fault: authenticity is its moat (real ETS items, real scoring), but
  pay-per-asset practice packs make sustained daily practice expensive; no
  retention/vocabulary system.

**Magoosh TOEFL** — verified to exist
- [FACT] Self-paced course: ~1,300+ practice questions licensed from ETS, ~150 video
  lessons, up to 10 full-length practice tests, AI tutor, study schedules; ~$109 (1 mo)
  / ~$129 (6 mo); +4-point score-improvement guarantee conditioned on completing the
  full program (all lessons, 400+ questions, wrong-answer explanations) and having a
  prior official score <2 yrs old (https://toefl.magoosh.com/plans,
  https://toefl.magoosh.com/score-guarantee, https://toefl.magoosh.com/).
- [ESTIMATE] Fault: guarantee conditions effectively require near-total course
  completion; content breadth over adaptivity — the "schedule" is a static plan, not a
  performance-adaptive loop.

**BestMyTest TOEFL** — verified to exist
- [FACT] Subscription prep: 1,000+ practice questions, ~20 full simulated practice
  tests with TOEFL-style integrated timing, 1,500+ lessons/vocabulary items, human
  detailed feedback on speaking and writing responses, analytics (item-level, section
  strengths/weaknesses, time management); plans roughly $39 (last-minute) to ~$69/1mo,
  ~$129/6mo, premium ~$219; 7-point improvement money-back guarantee on 1/6-month
  plans (https://www.bestmytest.com/toefl, https://www.bestmytest.com/toefl/upgrade).
- [ESTIMATE] Fault: unofficial items (realism varies); grading turnaround and depth of
  human feedback vary by plan; vocabulary component is a flat list, not a memory model.

---

## 9. Mechanism-extraction table

### Worth copying

| Mechanism | Best exemplar | What to copy | Evidence base |
|---|---|---|---|
| Model-based spaced repetition (DSR: difficulty/stability/retrievability; schedule at target retention) | Anki FSRS | Per-item memory state, user-set desired retention (default 0.90), parameters fitted to the learner's own history; mean-reverting difficulty (no ease hell) | [FACT] mechanism per docs.ankiweb.net + open-spaced-repetition |
| Streak calendar with pre-purchased slack | Duolingo | Daily completion streak + advance-purchase freezes (max 2); slack increases persistence vs rigid rules per Duolingo's cited habit research | [FACT] blog.duolingo.com |
| Confidence self-rating as scheduling input | Brainscape | 1–5 confidence tap: cheap input signal AND a metacognition prompt; but feed it into a real memory model instead of using it raw | [FACT] mechanic; [ESTIMATE] hybrid recommendation |
| Adaptive within-session difficulty / missed-item recycling | Quizlet Learn | Rounds that re-serve missed items until answered correctly; low-friction "it adapts to me" feel | [ESTIMATE] mechanics (no official algorithm doc) |
| Placement / diagnostic → generated study plan | ETS TestReady; Duolingo placement test | Short diagnostic that seeds a personalized plan and skips known material | [FACT] TestReady study-plan feature; Duolingo placement [ESTIMATE] |
| Usage-conditioned pass guarantee | INBDE Bootcamp, Magoosh, BestMyTest | Refund tied to measurable usage targets — aligns incentive, drives completion, de-risks purchase | [FACT] all three publish conditioned guarantees |
| Free-content funnel into paid structured practice | Mental Dental → INBDE Bootcamp | Free authoritative video builds trust; paid layer adds qbank, quizzing, progress, simulations | [FACT] the relationship is public (Bootcamp licenses Mental Dental videos) |
| Quiz-after-every-video micro-loop | INBDE Bootcamp | Bite-sized active recall immediately after passive content; converts watching into retrieval | [FACT] feature per bootcamp.com/inbde |
| Native-speaker micro-video as card material | Memrise "Learn with Locals" | Short authentic AV clips as prompts — multimodal encoding beats synthetic TTS | [FACT] feature exists |
| Notes ↔ cards single source of truth | RemNote | Cards generated in-context from notes/PDFs; fixes the "orphan deck" problem | [FACT] feature exists |

### Faults to avoid

| Fault | Where documented | Design rule for our system |
|---|---|---|
| Gamification outrunning learning (streak anxiety, XP-farming leagues, engagement metrics as proxy for knowledge) | Duolingo critiques [FACT cited above] | Gamify *retrieval events and retention outcomes*, never raw time/XP; streak = "completed today's due reviews," not "opened app" |
| Ease hell (punitive, hard-to-recover difficulty state) | Anki SM-2 [FACT] | Use mean-reverting difficulty (FSRS-style); a bad week must not permanently inflate workload |
| Wall-of-text cards / non-atomic content | Dental Decks format [ESTIMATE]; anti-pattern targeted by the minimum-information principle | Enforce atomic cards at authoring time; lint card length; one fact per card |
| Paywalling the effective (active) features while free tier keeps only passive ones | Quizlet 2022 [FACT] | Keep the core retention loop free/cheap; monetize content depth, feedback, and analytics instead |
| Removing/abandoning community content | Memrise [FACT]; Quizlet Gravity removal [FACT] | If user-generated content is accepted, treat it as a durable commitment; version and export |
| Unvetted shared-content marketplace with no quality signal | Anki shared decks [FACT] | Curate or score decks; provenance and accuracy signals on shared content |
| Expert-tool onboarding cliff | SuperMemo, Anki, RemNote [FACT] | Defaults must be correct out of the box; zero required configuration before first review |
| Raw self-rating as the whole scheduler | Brainscape [FACT mechanism; ESTIMATE weakness] | Use confidence as one input feature, calibrated against actual recall performance |
| Time-boxed access punishing slow studiers | Bootcamp/Dental Decks subscription windows [FACT pricing structure; ESTIMATE harm] | Long-horizon learners (e.g., foreign-trained dentists) need pause/extend options |
| Session mastery without calendar spacing | Quizlet Learn [ESTIMATE] | Every "mastered" item must enter a long-term review queue |

---

## Sources

Official/vendor: docs.ankiweb.net/deck-options.html · faqs.ankiweb.net/what-spaced-repetition-algorithm · apps.ankiweb.net · apps.apple.com (AnkiMobile id373493387) · github.com/open-spaced-repetition/fsrs4anki · help.supermemo.org/wiki/Features, /wiki/Incremental_reading · supermemo.guru/wiki/Algorithm_SM-17 · super-memory.com/archive/help16/adv_read.htm · supermemopedia.com (two pages cited) · blog.duolingo.com (streak-habit, vacation-streak, leagues-leaderboards, duolingo-max) · duolingo.com/help (leaderboards-and-league, what-is-duolingo-max) · remnote.com, remnote.com/pricing · help.remnote.com (6022755, 6026144) · brainscape.zendesk.com (13103043051149, 115002384312) · brainscape.com/academy (confidence-based-repetition-definition, brainscape-cognitive-science, does-anki-work) · bootcamp.com/inbde, /inbde/upgrade, /blog/the-ultimate-inbde-guide-for-foreign-trained-dentists · dentaldecks.com/dental-decks-inbde, store.dentaldecks.com · mentaldental.com/videos, youtube.com/@mentaldental · ada.org/resources/careers/licensure/licensure-for-the-international-dentists · ets.org (toefl-testready page, TestReady press release) · toefl.magoosh.com (/plans, /score-guarantee) · bestmytest.com/toefl, /toefl/upgrade.

Third-party (criticism/pricing corroboration): kuroahna.github.io/anki_srs_kai · readbroca.com/anki/ease-hell · expertium.github.io/Algorithm.html · en.wikipedia.org/wiki/SuperMemo · ntdaily.com, pawebpage.com, mintdeck.app (Quizlet paywall) · aistudymaster.com, studyguides.com (Quizlet pricing) · duolingoguides.com, theeconomyofmeaning.com (Duolingo critique) · languageappguide.com (Duolingo pricing) · studyfrenchspanish.com, testprepinsight.com (Memrise) · studycardsai.com (RemNote) · forums.ankiweb.net (learning-curve thread).

All third-party-only numbers are tagged [ESTIMATE] in the body; direct-fetch verification of official pages was blocked by session egress policy (see Method note).
