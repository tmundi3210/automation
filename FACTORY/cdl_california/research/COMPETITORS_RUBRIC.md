# Competitor teardown, flaws→fixes, 10-dimension scoring rubric

_Research sweep 2026-09-27 (read-only agents; web access was search-index only — dmv.ca.gov, youtube.com, app stores blocked from the build container). Tags: [FACT src] / [ESTIMATE basis] / [UNKNOWN]. Source agent: `competitors`. Kept verbatim for provenance; corrections from the plan review are in `../BUILD_PLAN.md` and `REVIEW_FINDINGS.md`._

# CA CDL workshop: competitive teardown (DISCOVER step)

**What this covers.** This is step 1 (DISCOVER) of `/home/user/automation/FACTORY/study_system/app_plan/base44_plan/COMPETITIVE_TEARDOWN_METHOD.md`. Steps 2–3 (the owner logs in and analyzes hands-on) have not been done.

**Limits on the evidence.**
- WebFetch was blocked for every page I needed (`EGRESS_BLOCKED`: apps.apple.com, play.google.com, zutobi.com, driving-tests.org).
- Reddit was refused by the search tool (`400 not accessible`), so there is no r/Truckers or r/CDL material.
- Every claim below comes from WebSearch result content attributed to the cited URL, tagged **[FACT-s]**. Review "quotes" are snippet text. They may be paraphrased and still need a verbatim check.
- **[FACT-u]** means a claim taken from the user's zip (`00-START-HERE.md`, `GK-01…`). **[EST]** means my own estimate or analysis.
- There are no video IDs in this report. YouTube links are named at channel level only.

---

## 0. Regulatory flag that changes the "multilingual" design
- **CA handbook:** DL 650 says the knowledge tests are given in English, Arabic, Chinese, Punjabi, Russian and Spanish, on the AKTE machines. Skills tests are English only, with no interpreter (pp. 1-10, 1-11). The user's own file adds: "confirm with the DMV which languages it offers." [FACT-u GK-01 L192/209, 00-START-HERE L25]
- **Federal:**
  - On 2026-02-20 USDOT (Sec. Duffy) announced rulemaking to make all CDL testing English-only. [FACT-s cdllife.com/2026/usdot-to-require-that-all-cdl-testing-be-conducted-in-english-only/; thehill.com/…/5748488]
  - An ELP out-of-service NPRM was published 2026-08-10, with comments due 2026-10-09. [FACT-s federalregister.gov/documents/2026/08/10/2026-16288]
  - One source says a final rule took effect 2026-04-17. It is low-quality and I could not corroborate it. [UNKNOWN]
- **Other states:** Texas CDL/CLP knowledge tests went English-only, with no interpreters, on 2026-06-01. [FACT-s dps.texas.gov/news/dps-announces-changes-cdl-knowledge-testing] Florida did the same in Feb 2026. [FACT-s cdllife]
- **California:** CHP now runs roadside ELP checks, and DMV has been sending "your CDL is in question" notices since mid-January. [FACT-s overdriveonline.com/…/15816962] A claim that "SB 104 requires ELP for CA CDL applicants from 2026-07-01" comes only from weak sources. [UNKNOWN, verify]
- **Design consequence [EST]:**
  - Multilingual here means *scaffolding*. Explanations and the glossary come in the native language. Mock tests run in **English by default**, with a native-language toggle.
  - Add an "English test-readiness" track covering exam terms, sign reading and the pre-trip vocabulary.
  - Show a banner telling the learner to confirm the test language when booking at the DMV.
  - Pull the regulation status from a dated config entry, not hard-coded text.

---

## 1. Competitor census

| # | Product | Core mechanics | Price / paywall | Multilingual | CA accuracy | Sims / visuals | Adaptive review | Gamification |
|---|---|---|---|---|---|---|---|---|
| C1 | **DMV Genie / CDL Genie + Driving-Tests.org (DTO)** — same vendor [FACT-s driving-tests.org/dmv-genie; Play id `dto.ee.cdl.genius`] | Question bank of 1,000+ "exam-like" questions; Easy mode → Exam Simulator (same format and pass mark); ELDT theory certificate earned in the app and sent to TPR [FACT-s Play/App Store listings] | App purchases $14.99–$99.99; users report ~$20/mo; "CDL Premium ELDT from $69"; 7-day and 30-day plans auto-renew; "may be unable to refund" auto-renew charges [FACT-s driving-tests.org/cdl-premium, /terms-of-use, justuseapp] | Web: language switch + voice-over; "35 languages" (scope may be car tests only) [FACT-s] | CA passenger page mixes the generic "16 or more" definition with the CA ">10 incl. driver" rule [FACT-s] → mixed | None found | **Challenge Bank™** = test built from your missed questions; explanations [FACT-s] | Pass guarantee; vendor claims "99.06% pass" and "73% more effective" [FACT-s, vendor claims, unverified] |
| C2 | **Zutobi CDL (CA course)** | 950+ questions with explanations, per-state courses, weak-area stats; also an FMCSA-registered theory provider [FACT-s zutobi.com/us/ca-cdl] | Weekly $4.99–$5.99; $7.99 tiers [FACT-s] | English only; multilingual "planned" [FACT-s] | **Says P endorsement = "16 or more"** on CA material [FACT-s zutobi.com/us/ca-cdl/cdl-practice-test] → federal number, **wrong for the CA test** | Car course: "engaging videos"; CDL [UNKNOWN] | Weak-area stats | Points, levels, streaks, leaderboard ("dopamine reward system") [FACT-s zutobi driver-guides] |
| C3 | **CDL Help** | 2,500+ questions, flashcards, results you can filter by correct/incorrect/skipped [FACT-s cdlhelp.com] | First section of every test free, no signup; one subscription unlocks the rest; price [UNKNOWN] | **10 languages, shown side by side with English, one-tap toggle** (EN ES RU ZH AR PT KO TR UK UZ) [FACT-s] | "State-based" GK; CA fidelity [UNKNOWN] | None | Flashcards for weak areas | Daily streak |
| C4 | **TruckingTruth High Road 2.0** | The CDL manual cut into small sections with a few questions after each; progress reports; "AI" re-queues the questions you struggle with [FACT-s truckingtruth.com/cdl-training-program2] | **Free** | English [EST] | Built on a national manual. A forum user found it said stopping distance ">300 ft" where their state manual said ">450 ft" [FACT-s truckingtruth forum Topic-4980]. The CA handbook says **419 ft @55 mph** [FACT-u] | None | Priority re-queue | Graduation recognition + free ebook |
| C5 | **Generic "CDL Prep/Practice Test 2026" apps** (CristCDL, "Coco", ABC, …) | Question bank + tests | Ads plus subscriptions | Some offer Spanish or bilingual modes; "CDL Answers" has 8 languages [FACT-s] | Reviews report errors (see §2) | None | "Repeats wrong answers 2 more times" (Coco) [FACT-s] | Minimal |
| C6 | **Quizlet CDL sets** | Sets made by users; Learn mode rounds | Plus ≈$35.99/yr; Learn/Test capped on free [FACT per research/competitors.md] | Weak [EST] | Unvetted sets, e.g. "CA CDL GK" [FACT-s quizlet.com/6047604] | None | Recycles misses within a session only; no calendar spacing | Match game |
| C7 | **ELDT theory providers** (ELDT.com etc.) | Required FMCSA theory course before the skills test; submitted to TPR [FACT-s fmcsa.dot.gov ELDT] | $20–$119+; ELDT.com from $25 [FACT-s] | [UNKNOWN] | Federal curriculum, not the CA handbook [EST] | Video or reading | None | None. Note: ~75 ELDT schools under investigation as of 2026-07-16 [FACT-s eldtnation] |
| C8 | **CA DMV official** (DL 650 PDF + Sample Commercial Tests 1–2) | Source of truth + two sample tests [FACT-s dmv.ca.gov sample-commercial-drivers-written-test-1/-2] | Free | 6 test languages; handbook in several languages + audio [FACT-s, verify] | **10 (it defines the answer key)** | Handbook diagrams | None | None |
| C9 | **Niche simulators**: DMV-IQ Air-Brake Check Simulator; Virtual CDL Simulator; "Pre-Trip: CDL Inspection" app; PretripMaster | Live PSI gauge, "say-it-out-loud" drill; guided → practice → **strict test mode**; voice coach; 27 hotspots [FACT-s dmv-iq.com/tools/…, virtualcdlsimulator.com, apps.apple id6792211818, soft112] | DMV-IQ free; others [UNKNOWN] | English [EST] | DMV-IQ uses "**federal** pass/fail specs" [FACT-s], so CA may differ, e.g. the 20–45 psi pop-out figure [FACT-u] | **Best in market** | Weak | Daily missions (Pre-Trip app) |
| G1 | **Duolingo** | Path; Birdbrain predicts P(correct) and keeps exercises at medium difficulty [FACT-s blog.duolingo.com birdbrain]; Explain My Answer made free again Jan 2026 [FACT-s] | Energy system: 25 units, 1 per question, ~2–3 lessons for free users [FACT-s duoplanet, classcentral] | ~40 courses | n/a | Light | Strong | Streak + freeze, leagues |
| G2 | **Anki + FSRS** | Difficulty/Stability/Retrievability model, adjustable retention target [FACT per competitors.md] | Free; iOS $24.99 | Content is the user's | Shared CDL decks unvetted | None | **Best in class** | None |
| G3 | **Brilliant** | Learn by doing with interactive visuals, no video [FACT-s] | ~$20/mo annual, ~$30 month-to-month (sources conflict) [FACT-s] | English-centric [EST] | n/a | **Best in class** | Moderate | Streaks |
| G4 | **Khan Academy** | Mastery levels: Attempted <70%, Familiar 70–99%, Proficient 100%, Mastered; unit tests move skills up **or down**; course % = share of skills at Proficient or above [FACT-s support.khanacademy.org 5548760867853] | Free | Khanmigo/teacher tools in 39 languages [FACT-s] | n/a | Video + exercises | Good | Points, badges |
| G5 | **Memrise** | Clips of native speakers; "Mems" mnemonics brought back Feb 2026 with AI images [FACT-s languageappguide] | $8.49/mo, $89.99/yr, $119.99 lifetime [FACT-s] | Its core strength | n/a | Video | Spaced repetition | Points, streaks |
| G6 | **Aceable** (drivers ed) | 5–10 min chunks, animated scenarios such as skids and intersections [FACT-s driversedrankings/justuseapp] | [UNKNOWN] | [UNKNOWN] | n/a | Animations | Weak | Completion %, streaks, badges |

---

## 2. (a) FLAW LIST, ranked by frequency × severity [EST ranking]

**F1. Memorizing the question bank passes the test without understanding** (very common, high severity)
- *Evidence:*
  - An instructor on the TruckersReport forum: "THIS IS NOT HOW YOU LEARN THE SUBJECT… how you study to pass any multiple-choice test where the questions and answers are known in advance."
  - Another user: "I cannot just regurgitate the words I am supposed to say for the pre-trip inspection."
  - Source for both: thetruckersreport.com/truckingindustryforum/threads/cdl-practice-test-app.232321/ (thread attribution from snippet).
  - Someone who scored 100% on practice tests found "not a single question from practice tests appeared" on the real exam (…/i've-failed-the-permit-test-twice-now.1565809/).
  - Apps market the opposite: "98 to 99% the same" (Play, CristCDL). [FACT-s]
- *Root cause:* the product is a question bank, and marketing measures familiarity with seen items.
- *OUR SOLUTION:*
  - Every lesson teaches the concept first, with the reason ("Learn it → why"; the user's files already have this structure) [FACT-u].
  - Write **isomorphic variants** of each fact: same fact, rotated wording, numbers, distractors and scenario.
  - Mastery requires one **transfer item** (a diagram or scenario) as well as recall.
  - Readiness is scored **only on items the learner has never seen**.

**F2. Wrong, ambiguous, or federal-not-California answers** (common, **critical**)
- *Evidence:*
  - One App Store review estimates "5-7% of questions containing incorrect information, another 10% with multiple answers that could both be correct, 10% poorly worded" (apps.apple.com/us/app/cdl-prep-practice-test-2026/id6446039122?see-all=reviews).
  - Another review: "in direct contradiction to the CDL manual, and also mechanical reality" (App Store CDL app; which app is unclear from the snippet).
  - Zutobi's CA material gives P = "16 or more" where CA says **>10 incl. driver** (GK-01 L28/38) [FACT-u].
  - The High Road stopping-distance conflict (§1, C4).
  - The search engine's own summary concluded "16 or more… appears to be the official requirement." In other words, the majority of web text is wrong for California.
- *Root cause:* national question pools re-skinned per state; no page-level citations; no errata loop.
- *OUR SOLUTION:*
  - Every item carries a DL 650 page reference, a `[CA]` flag, and an "elsewhere you may see X" note.
  - A dedicated **CA-divergence drill** built from the 9-row "Answer the HANDBOOK way" table plus HOS 12/16/80 and the 3-axle >6,000 lb rule [FACT-u 00-START-HERE L61-75, GK-03 L424-426].
  - An item linter rejects any item without a citation.
  - A "Report an error" button feeding a public changelog.
  - Every item is stamped with the handbook edition.

**F3. Billing and paywall dark patterns** (common, high)
- *Evidence:*
  - Zutobi: "charged for an additional 25 weeks even after cancellation"; "$7.99 after canceling" (trustpilot.com/review/zutobi.com).
  - CDL Genie: after an update, progress reset and content went behind the paywall — "an EDUCATIONAL MONEY GRAB"; a lifetime purchase was lost on a phone switch because there is no login (justuseapp.com/en/app/1447415467/…/reviews).
  - CristCDL: "Every time you try to answer a question… another ad pops up" (apps.apple.com/us/app/cristcdl-com-cdl-practice-test/id6738142291?see-all=reviews).
  - Paywall-analytics sites list the Genie paywall at ~$40K/mo (paywallscreens.com).
- *Root cause:* app-store subscription economics; weekly plans.
- *OUR SOLUTION:*
  - The core learn → test → review loop is free with no ads.
  - Any monetization is a one-time price with no weekly plans, one-click cancel, and account-based restore with export.
  - Answers and explanations are **never** paywalled.

**F4. Missing or self-contradicting explanations** (moderate, high)
- *Evidence:* one app marks "opening a hot engine" acceptable in the correct answer, then says "never" in the explanation (App Store, snippet). "Answers make no sense." [FACT-s]
- *OUR SOLUTION:*
  - A separate "why this is wrong" note for **each distractor**.
  - The explanation links to the lesson and the handbook page.
  - An automated answer↔explanation consistency check (LLM), then human sign-off.

**F5. English-only content, or translation with no preparation for an English test** (moderate, rising — see §0)
- *Evidence:* Zutobi is English only [FACT-s]. The field is moving to English-only tests. CDL Help shows what good looks like: side-by-side with English. [FACT-s]
- *OUR SOLUTION:*
  - Bilingual side-by-side display.
  - Mocks in English by default.
  - A glossary of exam terms with audio.
  - Numbers and units must never be machine-translated; they are locked tokens.
  - Key terms get human review.

**F6. Complex systems taught as text or scripts** (air brakes, coupling, pre-trip) (moderate, high)
- *Evidence:* the "regurgitate" quote in F1. Simulators exist only as separate niche products (C9).
- *OUR SOLUTION:* simulations built into the relevant lessons:
  - An air-system schematic with a live PSI gauge: governor cut-in/cut-out, low-air warning, spring-brake/tractor-protection pop-out **20–45 psi (CA)**.
  - A coupling sequencer with the fifth wheel **tilted down toward the rear** [FACT-u].
  - Off-tracking and turn visuals.
  - A stopping-distance builder (142 + 61 + 216 = 419 ft) [FACT-u].
  - A placard and triangle-placement board.
  - An HOS clock with a federal-vs-CA toggle.

**F7. "Review" means re-taking missed questions** (common, medium)
- *Evidence:*
  - DTO's Challenge Bank is exactly that [FACT-s].
  - No cross-device progress sync (Genie review) [FACT-s].
  - Anki's default of 20 new cards/day "overwhelms most learners within 2 weeks" (my-senpai.com, third-party) [FACT-s].
- *OUR SOLUTION:*
  - FSRS scheduling per fact and concept.
  - Tag each mistake with a misconception type:
    - `federal-vs-CA`
    - `number-neighbor` (26,000 vs **26,001**)
    - `direction` (backing toward the driver's side)
    - `sequence`
    - `unit`
  - Misses go to the review list automatically.
  - A cap on new items per day and an estimate of the next day's workload.
  - Server-side resume across devices.

**F8. Uncalibrated readiness and unverifiable pass-rate claims** (common, medium)
- *Evidence:* "99.06% verified pass rate" (Genie); "4/20… then 18/20+" swings caused by filler questions (App Store review) [FACT-s].
- *OUR SOLUTION:*
  - Mocks weighted to the CA test blueprint (GK: 50 questions, 40 to pass, 3 choices) [FACT-u 00-START-HERE L15; GK-01 L197].
  - Readiness shown as a range, not a single number.
  - The target is **≥90%** before test day [FACT-u L59].
  - No pass-rate marketing.

**F9. A fragmented journey for beginners** (common, medium)
- *Evidence:* permit questions, ELDT, pre-trip and English each come from separate products. ELDT must be finished and submitted to TPR before the skills test [FACT-s tpr.fmcsa.dot.gov]. TPR upload delays happen [FACT-s eldtnation].
- *OUR SOLUTION:*
  - A roadmap per class and endorsement: medical card → knowledge tests → CLP → ELDT (link to the TPR search; **we are not an ELDT provider**) → pre-trip / basic control / road → DMV booking.
  - A scheduler that works back from the target test date.

**F10. Generic or manipulative gamification** (moderate, medium)
- *Evidence:*
  - Zutobi: points, leaderboards, "dopamine" [FACT-s].
  - Duolingo energy: "45% of negative sentiment stems from the energy system" (approast.app/analyses/duolingo.html) [FACT-s, third-party analysis].
  - Streak anxiety (see research/gamification_visuals_multimodal.md).
- *OUR SOLUTION:* mechanics matched to each topic (§3). Rewards go for **retention and transfer**, not time spent. Streak freeze plus a "days active in the last 30" metric. No leagues by default.

**F11. Tone and pacing mismatch** (low, low)
- *Evidence:* Aceable's "relatable jokes… distracting," glitching animations, and certificates not being generated. Brilliant's "hand-holding pace," "shallow learning" [FACT-s].
- *OUR SOLUTION:* a to-the-point core with a "Dive deeper" expander; a tone setting; the simulation always has a static fallback.

---

## 3. (b) What to copy, and how it maps to our workshop [EST mapping]

1. **DTO/Genie:** Easy → Exam Simulator progression; a missed-question bank (upgraded to FSRS plus misconception tags); a 3-day pre-billing reminder.
2. **CDL Help:** EN plus native language side by side with a one-tap toggle; results filterable by correct/incorrect/skipped; first section free with no signup.
3. **High Road:** the handbook cut into small sections with 2–3 questions after each; totally free.
4. **Simulators (DMV-IQ / Virtual CDL / Pre-Trip app):** guided → practice → **strict test mode**; a "say it out loud" drill with a voice coach for the in-cab air-brake check.
5. **Duolingo Birdbrain:** keep items in the 60–85% predicted-correct band. **Explain My Answer:** free, per item.
6. **Khan Academy:** mastery states that can go **down**; unit tests; course % = skills at Proficient or above.
7. **Anki FSRS:** a retention target (default 0.90); difficulty that drifts back toward normal (no "ease hell"); a new-item cap.
8. **Brilliant:** manipulate first, read the text second, for mechanical topics.
9. **Memrise Mems:** memory-hook images (the user's files already have "Memory hook" lines [FACT-u GK-01 L35]).
10. **Aceable:** short animated scenarios for skids, railroad crossings and intersections.
11. **CA DMV Sample Commercial Tests 1–2:** link them as outside calibration anchors.
12. **YouTube:** link out at channel/topic level (e.g. Smart Drive Test for the in-cab air-brake test [FACT-s smartdrivetest.com]), always with a "Handbook wins" disclaimer. Verify IDs at build time.

**Gamification tailored to each topic** [EST]

| Topic | Mechanic |
|---|---|
| Air brakes | "Keep the pressure": the learner runs the gauge through the governor and warning thresholds |
| Combination | "Couple it right": timed sequencing; mistakes trigger a visual consequence, e.g. a high-hitch |
| Space management | "Cushion keeper": set the following distance for a given length and speed |
| Hazmat | "Placard match" |
| HOS | "Clock puzzle", federal vs. CA |
| Inspection | "Find the defect" hotspots |
| Disqualifications | Timeline matching |
| CA-divergence | "Handbook vs. the internet" duel cards |

---

## 4. (c) What to avoid
- Weekly auto-renew plans, cancel mazes, lifetime purchases with no account.
- Paywalling answers or explanations; ads between questions.
- Resetting progress, or putting paid-for content behind a new paywall after an update.
- Unverifiable pass-rate claims and "questions identical to the DMV" marketing.
- Federal numbers presented as California answers; mixed sources without a flag. The user's own warning: remove the old Parts 01–04, 06 and 07 from NotebookLM [FACT-u 00-START-HERE L77].
- Treating a high score on seen items as readiness.
- Streak-anxiety and loss-aversion prompts; energy/heart limits on learning; XP leagues.
- Machine-translated numbers and units; an English-only interface; translating everything with no English-test preparation.
- Explanations that contradict the answer; missing "why wrong" notes for distractors.
- Meme or humor filler; unvetted content made by users (Quizlet/Anki decks) without provenance.

---

## 5. (d) Scoring rubric (1–10 per dimension; weights sum to 100)

**Scale anchors:** 1 = absent or harmful · 5 = table stakes · 8 = best in market · 10 = exemplary, with evidence.

**Our pass gate:** every dimension ≥8 **and** a weighted total ≥8.0. If we score below that, redesign and re-score.

| Dim | Weight | How to measure |
|---|---|---|
| D1 CA-handbook fidelity | 15 | Run the **CA-divergence probe** — 13 items: OOS 90 d; high-beam 500 ft; placard 9.84 in; WLL ½; passengers >10; fifth wheel tilted down; TP valve 20–45 psi; trailer hand valve; HOS 12/16/80; 3-axle >6,000 lb; 419 ft @55; employer notice; 26,001. Score = % correct ÷ 10 |
| D2 Understanding-first teaching + explanations | 15 | Share of items with concept lesson, "why" and per-distractor rationale; score on transfer items |
| D3 Interactive visuals/simulations | 10 | Coverage of the top 8 mechanical topics; strict test mode present |
| D4 Adaptive review and mistake diagnosis | 10 | Spaced scheduling? Misconception tags? Auto review list? |
| D5 Honest readiness | 8 | Mocks on unseen items only, blueprint-weighted, range shown, no inflated claims |
| D6 Class/endorsement personalization + end-to-end roadmap | 10 | A/B/C plus P/S/N/T/H/X paths; steps from medical card to skills test; scheduling |
| D7 Multilingual + English-test readiness | 10 | Number of languages, bilingual display, English mock default, glossary with audio |
| D8 Topic-specific, non-manipulative gamification | 7 | Mechanic tied to the skill; rewards retention; streak slack |
| D9 Continuity | 7 | Resume, cross-device sync, offline, reminders |
| D10 Price fairness / no dark patterns | 8 | Checklist from §4 |

**Estimated scores** [EST — basis is the snippets in §1–2; hands-on scoring still pending]

| Product | D1 | D2 | D3 | D4 | D5 | D6 | D7 | D8 | D9 | D10 | **Weighted** |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C1 DTO / Genie | 6 | 6 | 3 | 5 | 4 | 7 | 6 | 3 | 4 | 3 | **5.0** |
| C8 CA DMV official | 10 | 4 | 2 | 1 | 5 | 4 | 7 | 1 | 1 | 10 | **4.8** |
| C9 Niche simulators | 5 | 6 | 9 | 3 | 5 | 3 | 2 | 3 | 4 | 6 | **4.7** |
| C3 CDL Help | 5 | 5 | 2 | 4 | 3 | 5 | 8 | 3 | 5 | 5 | **4.6** |
| C2 Zutobi | 4 | 6 | 4 | 5 | 4 | 6 | 2 | 6 | 5 | 2 | **4.5** |
| C4 High Road | 4 | 6 | 2 | 5 | 3 | 5 | 1 | 2 | 5 | 10 | **4.3** |
| C7 ELDT theory | 5 | 6 | 3 | 2 | 2 | 6 | 4 | 1 | 6 | 6 | **4.3** |
| C5 Generic apps | 3 | 3 | 1 | 3 | 2 | 4 | 4 | 2 | 3 | 3 | **2.9** |
| C6 Quizlet sets | 3 | 2 | 1 | 4 | 1 | 1 | 3 | 4 | 7 | 5 | **2.9** |
| G4 Khan | n/a | 8 | 6 | 7 | 7 | 7 | 8 | 5 | 8 | 10 | **7.4*** |
| G1 Duolingo | n/a | 5 | 6 | 8 | 4 | 7 | 10 | 5 | 9 | 4 | **6.4*** |
| G3 Brilliant | n/a | 8 | 10 | 6 | 4 | 6 | 3 | 6 | 8 | 4 | **6.3*** |
| G5 Memrise | n/a | 4 | 6 | 7 | 3 | 5 | 9 | 6 | 8 | 5 | **5.8*** |
| G6 Aceable | n/a | 6 | 7 | 4 | 4 | 7 | 4 | 6 | 6 | 5 | **5.5*** |
| G2 Anki | n/a | 2 | 3 | 9 | 5 | 1 | 5 | 1 | 8 | 9 | **4.5*** |

\* Normalized over D2–D10 (weight 85).

**Takeaways [EST]:**
- No CDL product scores above ~5.0.
- The best-in-market score for each dimension sits in a different product: CA DMV for D1, the simulators for D3, CDL Help for D7, High Road for D10. General-platform benchmarks: Anki/Duolingo for D4, Brilliant for D3, Khan for D5 and D10.
- The opening is to combine them, with the CA handbook as the only answer key.

**Next steps under the method:**
1. The owner logs into 2–3 products (suggested: DTO CDL Premium, Zutobi CA CDL, CDL Help). Run the 13-item D1 probe hands-on to turn D1 into [observed] scores.
2. Check the §2 quotes verbatim.
3. Re-check the §0 regulatory status on the build date.
