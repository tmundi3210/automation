# OWNER_BRIEF.md — L0: the owner's brief, saved as-is (2026-07-02)

_Full context of the owner's message, verbatim, before any processing — per the standing instruction "first, save the full scale of all of this context to me, as it is." Speech-to-text artifacts present. §2 unpacks it clause by clause with interpretation confidence tags. Nothing in §2 overwrites §1. A follow-up clarification from the owner is in §1b._

## 1. The owner's message (verbatim, unedited)

> So want to create a system for, specialized for scheduling, keeping check, pushing the questions, the time, time dependent, response dependent, analysis gates, long-term memory, scheduling, re-analysis, specialized for teaching, so that in a way it should help someone to keep what he is learning easy to remember, using best techniques like interval, that Anki, that how with time it gives you the repetition. So use its method, I think it's public, the method to calculate how often the thing should be revised. The Anki is also installed, so use it also, and collect the knowledge base for dental, for someone who is, who want to give dental exam in USA, like foreign graduates, whatever the exam is. Also preparing for TOEFL from starting, take that syllabus, what's their syllabus, and make knowledge base. And right now just make knowledge base. With time we'll create flashcards, study cards, or questions, whatever is required. And I want a question specialist, and I also need what type of questions they ask there. So directive learning, and then about how to answer a question, like guidance. But mainly focus on collecting the material. Then there might be audios, videos, public available data, and I will also add my material which I have. Timeline specialist, like, and also I want that specialist which with time, so it can analyze how much the person is learning, adjusting to it, keeping it challenging, thinking about what gamification can be done, visual representation of the course, visual representation of the days, tasks which have been completed, showing percentage also. Like, let's suppose there's a calendar, and the boxes for, let's say, one month calendar is there, or one week. Then there are boxes for each day. If I have completed the revision or whatever the task is, it will fill up as like first it's empty, let's say white with one in it, then it will fill like a filling bar, like downloading bar, green, green, green. Then if it's all done, then tick. And if it's not fully done, then it should show the percentage, like 10, 15 with 5% increment. And then like adding multimodal capabilities, how to add them, like so the user doesn't always need to write the answer, he can speak. Like maybe there are 10 questions and he can just give the answer by just speaking in whatever language. Then it will be saved and queued for analysis by the AI, like the local AI or the cloud, later on decision. And with time, building a knowledge base about what the user know, what his weakness, how to test the weakness, asking questions around the weakness, so why that weakness is there, so that, and what's easy for the user, doesn't require too much repetition. So according to that, adjusting the future schedule, like every night it should go into analysis mode, like it should have feedback loops, the system itself, so that it can help the user maximum, what are the faults or other things, how other competitors do it, note making, using voice, how to create a flashcard, a high-quality flashcard, what images, flowcharts, or questions, or answers, or edges, and how to like first we need to convert a book into edges, knowledge base, flashcards, or whatever, or graphs, or maybe not the graphs, but breaking the book into units, like not units, but so that, so search, do a research how a book can be broken down, like academic book, and what are its aspects, and how, like first we need to know terminology, then we step up, we need to learn basic things, then we can relate two things, then those two things make something else, and learning how to two-step thinking or three-step thinking. So how does that

_(The message ends mid-sentence — "So how does that" — held as [UNKNOWN], not guessed into a requirement.)_

## 1b. The owner's follow-up clarification (verbatim)

> i want to make new specialists for a new project

Read: the learning/teaching system above IS the new project, and the deliverable is **new specialists** for it, built the standing FACTORY way (research → dense gated KBs → distilled gated specialists). [FACT — explicit, interpreted against the message it interrupts]

## 2. Clause-by-clause unpacking (interpretation layer — tagged)

### 2a. The SYSTEM being designed (what the specialists must collectively know how to build/run)
1. **Core system**: specialized for scheduling, keeping check, pushing questions; time-dependent AND response-dependent behavior; analysis gates; long-term memory; re-analysis; specialized for TEACHING — goal: make what someone is learning easy to remember. [FACT — explicit]
2. **Spaced repetition as the engine**: "best techniques like interval, that Anki" — use Anki's method to calculate revision timing; owner believes it is public. [FACT on the ask; the algorithms ARE public: SM-2 is published, Anki's current FSRS scheduler is open-source — to be pinned with sources in L1 research]
3. **"The Anki is also installed, so use it also"** — Anki exists as an integration target. WHERE it is installed is [UNKNOWN] (not present in this build container — verified). Integration must therefore go through public, location-independent surfaces: `.apkg` deck format, AnkiConnect API, FSRS parameters. [FACT on the directive; [UNKNOWN] on location]
4. **Nightly analysis mode**: every night the system enters analysis mode; system-level feedback loops so the system improves itself; study "what are the faults" and "how other competitors do it". [FACT — explicit]
5. **Learner model over time**: build a KB about what the user knows, weaknesses, how to TEST a weakness, questions around the weakness, WHY the weakness exists; what is easy for the user needs less repetition; future schedule adjusts accordingly. [FACT — explicit]
6. **Gamification + visual representation**: of the course, of the days, of completed tasks, with percentages. Concrete UI spec from the owner: a calendar (month or week) of per-day boxes; empty box = white; as the day's tasks complete it fills like a downloading bar, green; fully done = tick; partially done = show percentage in 5% increments (10%, 15%, ...). [FACT — explicit, unusually concrete]
7. **Multimodal answering**: user shouldn't always have to type — e.g. 10 questions answered by SPEAKING, in ANY language; recordings saved and QUEUED for later analysis by AI ("local AI or the cloud, later on decision" — the local-vs-cloud choice is deferred by the owner). [FACT — explicit]
8. **Voice note-making** and **high-quality flashcard creation**: what images, flowcharts, questions, answers, edges make a flashcard high-quality. [FACT — explicit]
9. **Book decomposition**: research how an academic book is broken down — its aspects; the learning ladder: first terminology → then basic things → then relate two things → those two make something else → two-step / three-step thinking. [FACT — explicit; "maybe not the graphs" = owner unsure about graph representation, keep both options open]

### 2b. The CONTENT to collect (the "mainly focus" priority)
10. **Dental exam, USA, foreign graduates** — "whatever the exam is": identify the exam(s) a foreign dental graduate must pass for US licensure and collect its knowledge base. [FACT on the ask; exam identity to be pinned from OFFICIAL sources in L1 — working hypothesis INBDE (JCNDE/ADA) for the knowledge exam, [ESTIMATE] until cited]
11. **TOEFL from zero** — take the official syllabus and make a knowledge base. [FACT — explicit]
12. **"Right now just make knowledge base. With time we'll create flashcards, study cards, or questions, whatever is required."** — PRIORITY ORDER: KBs now; cards/questions later. [FACT — explicit]
13. **"But mainly focus on collecting the material."** — material collection is the center of gravity of this phase. [FACT — explicit]
14. **Sources**: audios, videos, publicly available data; owner will ALSO add his own material later. [FACT — explicit; own-material ingestion is a future hook, not a current input]
15. **Question intelligence**: a QUESTION SPECIALIST; what TYPES of questions the exams ask; directive learning; how to answer a question (guidance/strategy). [FACT — explicit]
16. **Timeline specialist**. [FACT — explicit, named directly]
17. **Adaptive-learning specialist**: with time, analyzes how much the person is learning, adjusts to it, keeps it challenging. [FACT — explicit]

### 2c. The explicit QUESTION BANK (to be answered by the specialists once built)
- Q1. Which exam(s) must a foreign dental graduate pass for US dental licensure, and what is each exam's OFFICIAL content blueprint?
- Q2. What is the official TOEFL iBT syllabus — sections, question types, timing, scoring — for a from-zero learner?
- Q3. What exactly is Anki's revision-interval method (SM-2, FSRS)? Is it public, and how is the next-review time computed from a response?
- Q4. What question types do these exams use, and what is the guidance for answering each type (directive learning)?
- Q5. What makes a flashcard high-quality (rules, images, flowcharts, cloze, "edges")?
- Q6. How is an academic book decomposed into learnable structure — terminology → basics → paired relations → multi-step (2–3 step) reasoning?
- Q7. How do competitors do it (Anki, SuperMemo, Quizlet, Duolingo, dental/TOEFL prep platforms) and what are their faults?
- Q8. How to model the learner: what they know, weakness detection, testing a weakness, WHY a weakness exists, easy-vs-hard calibration?
- Q9. How should the nightly analysis mode + system feedback loops work?
- Q10. How to build the gamified visual layer (calendar fill-bar boxes, tick, 5%-increment percentages, course/day views)?
- Q11. How to capture spoken answers in any language, store them, and queue them for local-or-cloud AI analysis?
- Q12. How to integrate with the owner's installed Anki (deck export, AnkiConnect, FSRS parameters)?

### 2d. Deliverables ladder (L-series), saving one by one
- **L0 (this file):** brief saved as-is. → commit.
- **L1:** `research/` — grounded research files with official-source citations: exam identification + blueprint (dental, USA, FMG), TOEFL official syllabus, spaced-repetition algorithms (SM-2/FSRS) + Anki integration surfaces, exam question typology + answering guidance, flashcard quality, book decomposition, competitor scan, learner-model/loops/gamification/multimodal patterns. → commit.
- **L2:** `BUILD_PLAN.md` + taxonomy — the specialist roster (domains, boundaries per the domain-coherence rule) and the KB decomposition per specialist, seeded from L1. → commit.
- **L3:** KBs — hand-authored `*.spec.json` → forge → `kb_validator.py --mode dense` gates, per domain, committed domain by domain. **This is the "mainly focus" phase: the material lives here.**
- **L4:** specialists — distilled from the gated KBs, gated by `specialist_validator.py`, `bash FACTORY/build.sh` ALL GREEN per domain. → commit.
- **L5+ (with time, per owner):** flashcards/study cards/question banks generated FROM the KBs; Anki deck export; the runnable system (scheduler, nightly analysis, UI, voice capture); owner's own material ingested.

_Honesty rules bind throughout: [FACT]/[ESTIMATE]/[UNKNOWN] tags; exam blueprints and syllabi ONLY from official sources (JCNDE/ADA, ETS) with URLs; no invented statistics, prices, or pass rates; competitor claims only from their public pages/docs; algorithm math only from published sources. Gates check structure + math, never prose truth — prose truth is carried by the source citations._
