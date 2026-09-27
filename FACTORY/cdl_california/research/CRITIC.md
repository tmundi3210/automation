# Completeness critique of the research sweep

_Research sweep 2026-09-27 (read-only agents; web access was search-index only — dmv.ca.gov, youtube.com, app stores blocked from the build container). Tags: [FACT src] / [ESTIMATE basis] / [UNKNOWN]. Source agent: `critic`. Kept verbatim for provenance; corrections from the plan review are in `../BUILD_PLAN.md` and `REVIEW_FINDINGS.md`._

# Completeness critique: CA CDL workshop research (6 researchers)

## 0. Verification performed by the critic
- Read the whole of `00-START-HERE.md` with `unzip -p`. Grepped GK-01, GK-02, GK-03, GK-04, CV-01, part-05, part-09 and part-10.
- Web checks:
  - Hazmat question count
  - Retake wait
  - Road-test scoring
  - DANGEROUS placard threshold
  - Where the fail-rate stat comes from
  - Spot-check of one video ID
- Tried to fetch DL 650. Every mirror returned `EGRESS_BLOCKED`: qr.dmv.ca.gov (wp-json handbook API), rc-hr.com/comlhdbk.pdf, static.epermittest.com (2019 PDF). Earlier researchers also found dmv.ca.gov blocked.
  - **So nothing can be checked against the handbook from this environment.**
- The WebSearch budget is now used up (200/200).

## 1. Contradictions between researchers and their resolution

| # | Topic | Conflict | Resolution / status |
|---|---|---|---|
| X1 | HazMat test size | ca-rules and part-09 say 30/24. content-old says a web search gave 20/16. | Third-party sources agree on **30/24** [FACT-3p driving-tests.org, nextdoordriving, voltexam]. Not confirmed by DMV. Keep it as a config value. |
| X2 | Knowledge-test retake wait | Handbook p.1-4 says **no waiting period** [FACT ARC GK-01 L103, L454]. Third parties say 7 days (epermittest) or 3 days (drivetrucks, found by the critic). | The answer key uses the handbook: no wait. The roadmap tells learners to confirm with DMV. |
| X3 | DANGEROUS placard one-class limit | ca-rules: "edition-dependent, 5,000?" part-09 caveat: "older editions print 5,000". | **Resolved: 2,205 lb.** The current DMV Section 9 text reads "not loaded 2,205 pounds or more… at any one place" [FACT-s qr.dmv.ca.gov section-9 snippet]. Drop the part-09 caveat. |
| X4 | Telling the employer about a suspension | GK-01 trap marks "next business day" correct. GK-03 trap (L423) marks "2 business days" correct and says "30 d is for out-of-state conviction". | This is inside the handbook itself (p.1-4 vs p.1-16). Item rule: **never offer both as options**, and accept either. The two lesson files disagree on which is "Correct", so harmonize them. |
| X5 | Gamification | content-new proposes a "3 Hearts" life bar, Radar Gun, timed Idle Timer, reaction/timing games (Mirror Watch, landmark count, gap acceptance, Hill Start pedal timing, Double-Clutch rhythm). pedagogy INV and competitors reject hearts/lives, speed scoring and energy. | **Unresolved.** Suggestion: timing sims become ungraded "explore" mode and never write to the learner model. "3 Hearts" becomes informational only (it shows the DMV's 3-attempt rule, not a learning limit). |
| X6 | Resume / persistence | media-tech: local IndexedDB plus an export code, single-file build. competitors F3/F7: server-side, cross-device, account restore. | **Unresolved** (decision D5). |
| X7 | Platform | The repo's `app_plan/` targets a hosted Base44 app. media-tech recommends a standalone Vite/Preact PWA plus a single-file build. | **Unresolved** (D4). |
| X8 | Readiness gate | competitors: score readiness **only on unseen items**. pedagogy: hard gate = START-HERE readiness sets (GK 93 Q, CV 68 Q), which the learner has already seen as lesson tests. Also, 78 of the 93 GK and 52 of the 68 CV questions are in **missing** lessons (GK-07/10/12/14, CV-02–04) [FACT ARC START-HERE L156-157]. | **Unresolved** (D8). A large bank of fresh items is needed. |
| X9 | Self-test rubric | competitors: 10 dimensions, weights sum to 100, pass = every dimension ≥8 **and** weighted ≥8.0. pedagogy: 7 axes, pass 8, hard fails cap the score at 5. Neither has a **visual/screenshot** rubric. | **Unresolved** (D15). |
| X10 | ts-fsrs FSRS-6 support | pedagogy: [FACT via snippet] ts-fsrs 5.x = FSRS-6. media-tech: [UNKNOWN]. | Check at build time: `default_w.length === 21`. |
| X11 | D1 "CA-divergence probe" | The probe includes fifth wheel tilted down, TP valve 20–45 psi and 419 ft. These are probably also the federal model-manual values [ESTIMATE, recollection; verify], so they test "handbook vs websites", not "CA vs federal". The competitors' implication that DMV-IQ's "federal specs" differ on 20–45 psi is unsupported. | Relabel the probe "handbook-fidelity probe". |
| X12 | Non-domiciled CDL figures | ca-rules: "~13,000 cancelled 2026-03-06". | [UNKNOWN]. The critic recalls ~17,000 announced Nov 2025 [ESTIMATE]. Off-test; never hardcode. |
| X13 | Persona | START-HERE copy is written for one learner ("You passed Air Brakes") and the old parts are framed "Class A". | Copy must be conditional on the profile. Air Brakes (section 5) needs a full lesson for other learners. |
| X14 | Validator vs content | media-tech's validator requires a DL 650 page on every question. Flashcards have no page column. Old parts have no pages at all. Part-07/B13 mocks have no pages. | The validator would reject all interim content. Decide on a tag such as `page:inferred` or `unverified`. |

## 2. Unverified, fabricated-risk and miscomputed claims

**Computation errors in pedagogy-engine (recomputed with its own formulas):**
- **FSRS example.** "Good → S 2.31; review at R=0.9 Good → 11.9 ✓; a lapse *at that point* gives S=0.62 ✗."
  - Lapse from S=2.31 (D≈2.12, R=0.9) gives **0.62**.
  - Lapse from S=11.9 gives the long-term branch **≈1.60** (the short-term cap is 11.33).
  - The example has the wrong S. Pin the FSRS behaviour with ts-fsrs golden tests.
- **BKT example.** "P(L)=0.2 → ≈0.25 after a correct 3-option MCQ; meets Δ≤+0.07."
  - That holds before the learning transition (≈0.252 with S=0.1, w_fmt=0.3).
  - Step 4 with P(T)=0.15 gives **≈0.36 (Δ+0.16), which fails the acceptance limit.**
  - Fixes (pick one): P(T) ≤ ~0.02 on MCQ; scale P(T) by w_fmt; or apply the transition once per session.
- The binomial readiness table is correct (e.g. Bin(50, 0.9) ≥ 40 = 0.991).

**Unverified or at risk:**
- **Video IDs** (30, media-tech): index-only. Spot-check of `hiobnwMv2cw`: title and URL are in the search index, published 2024-01-06 [FACT-s], channel [UNKNOWN]. Availability and embeddability are UNKNOWN for all 30.
  - None is an official CA DMV video. Several are from 2012 or have titles saying "procedure changed".
  - Treat all as unverified. They must never be an answer source.
- **Competitor evidence:** App Store IDs (id6446039122, id6738142291, id6792211818); "99.06% pass"; "$40K/mo paywall"; "45% of negative sentiment"; "~75 ELDT schools under investigation"; "Explain My Answer free Jan 2026". These come from search snippets or vendor claims, not verbatim, and Reddit was inaccessible. Use for internal design only; never display.
- **1st-try fail rates 39.5% GK / 43.9% CV:**
  - They probably come from DMV's "Statewide Evaluation of the CDL Written Knowledge Tests" (Mar 2008). That report gives an overall 56.6% fail by computer score [FACT-s dmv.ca.gov file snippet].
  - The per-test figures are unverified and the data is 18 years old. Show with the date, or omit.
- **Legal/regulatory claims** (verify or omit):
  - AB 1272 / VC §25 domain rule (it applies to occupational licensees)
  - NRII CA date "2026-07-27"
  - CA ELDT 15 h BTW / 10 h on public roads / DL 1236
  - Duffy's "~20 languages"
  - Restriction codes 46 and 50
  - Basic-control fail at "12 points" (federal ESTIMATE)
  - Elo K=0.4 labelled "FACT" (should be ESTIMATE)
- **Corroborated:** road test pass = ≤30 errors and no critical driving error [FACT-s dmv.ca.gov section-13 snippet]. The snippet also says driving 10 mph *under* the limit without cause is a critical error [FACT-s, verify].
- Playwright `page.clock`, which media-tech marked [UNKNOWN]: the clock API arrived in 1.45 [ESTIMATE, recollection]. Confirm against the 1.56.1 types.
- **Handbook edition is UNKNOWN.** The lesson page numbers (e.g. p.1-1…6-17) are tied to one edition, and lessons cite a 2020 fee table. Page references could drift from the live edition.

## 3. Coverage holes vs the goal (status: ✗ none, ◐ partial, ✓ specified)

| Goal area | Status | Hole |
|---|---|---|
| Source content, GK/CV | ◐ | 11 of 18 new lessons are missing (GK-06, 07, 09–14; CV-02–04), about half of pp. 2-12–2-49, 3-1–3-5 and 6-5–6-17. Of the pack totals, 207/338 practice questions, 412/657 flashcards and 69/98 TYK questions are missing. |
| Sections 4, 5, 7, 8, 9, 10 (P, AB, T, N, H, S) | ◐ | Only the old uncited parts cover these: no pages, no explanations, no distractors, Class A framing. part-10 S is from the **Oregon** manual. |
| Sections 11–13 skills tests | ✗ | Zero content: pre-trip verbal scripts per vehicle type (tractor-trailer, straight truck, bus, school bus), in-cab air-brake check, basic-control exercises and scoring, road-test critical errors. |
| Class B and C paths | ✗ | No B or C curriculum. Figure 1.2 mapping only. Class C exists only with H, P or N. Bus-specific inspection is absent. |
| Endorsements and certificates | ◐ | T, N, H, P and S are thin with no mocks. F (CA) has no content. X is derived only. CA certificates (CHP school bus, SPAB, GPPV, VTT, FLV, HAM, ambulance, tow) are listed but not taught. No paths for removing restrictions (E, L, Z, O). |
| Teaching style per topic | ◐ | Topic → modality → mechanic maps exist for the 7 present lessons only. Nothing for the other 11 lessons or sections 4, 5, 7–13. The source of "deep dive" content (handbook excerpt? extended Learn-it? video?) is undefined. |
| Testing | ◐ | Official per-topic item distribution is UNKNOWN. Unseen-item readiness needs isomorphic variants per fact, and there is no generation, verification or review pipeline. There are 262 wrong options (131 × 2) that need distractor tags authored. |
| Resume | ◐ | Local resume is specified. Cross-device is undecided (X6). |
| Scheduling | ◐ | Planner specified. Missing: .ics export; reminders (push needs a service worker and server, so not possible in single-file); multi-test sequencing (GK → CLP → 14 d → skills); DMV booking deep link; ELDT-completion milestone. |
| Guidance / beginner end-to-end | ✗ | Only a bullet list exists. Missing: onboarding ("which class do I need?" from Class Sorter), glossary, next-step coach, eligibility pre-check (age, medical, domicile), cost/time estimator, forms walkthrough (DL 44C, MER/MEC, DL 694), Clearinghouse registration, ELDT/TPR explainer. |
| Multilingual | ◐ | Locales not chosen. Undecided: script variants (Punjabi Gurmukhi vs Shahmukhi; Chinese Simplified vs Traditional); who translates and reviews; CJK/Gurmukhi fonts vs the single-file size budget; RTL QA; TTS voice availability per locale [UNKNOWN]; English-proficiency track content. |
| Mistake intelligence | ✓ spec / ✗ content | The misconception catalogue (IDs) has not been authored from the 97 traps and 20 conflicts. There is no prerequisite graph between concepts, but E3 diagnosis needs one. |
| Topic-specific gamification | ◐ | Designs cover 7 lessons only, plus the unresolved X5 conflict. Nothing for H, N, T, P, S, AB or skills. |
| In-app intelligence | ✗ | Nobody decided whether an LLM tutor or "explain my answer" chat exists. It needs a network and an API key, which conflicts with the offline single-file build. |
| Self-test | ◐ | Two rubrics, no visual rubric, no per-step scoring protocol. The evaluator is the builder, which is biased. No persona set. No automated accessibility checks (axe-core via the npm registry is reachable). The synthetic-learner harness has not been adapted. |
| Competitor analysis | ◐ | Hands-on steps 2–3 not done. Review quotes not verbatim. No r/Truckers or r/CDL. |
| Legal | ◐ | Reproducing the 98 verbatim TYK questions and handbook text is a copyright question (DL 650 licence UNKNOWN). Product name, disclaimer and monetization are undecided. |

## 4. Top 15 decisions to settle before building

1. **Access to the source of truth.** Will the user upload the DL 650 PDF (which edition), or allowlist dmv.ca.gov? Without it, nothing beyond the 7 lessons can be checked "the handbook way", and the edition stamp and page references can't be confirmed.
2. **The 11 missing lessons.** Does the user have GK-06, 07, 09–14 and CV-02–04 in the same format? If not, choose between:
   - (a) generating them from DL 650 (depends on D1), or
   - (b) reusing corrected old parts as an interim, flagged "unverified".
3. **v1 scope and staging.** Suggested: v1 = GK + CV (Class A, the user's own need) + AB. Then P, S, N, T, H, X, F. Then skills tests 11–13. Then Class B/C bus and truck paths and certificates. Define "complete" per stage.
4. **Platform and hosting.** Standalone Vite/Preact PWA plus single-file build, or the repo's Base44 plan, or both. Also where it is hosted, and whether it must work offline from `file://`.
5. **Persistence and privacy.** Local-only (IndexedDB plus export code) or accounts with server sync across devices. Include data retention, analytics yes/no, and CCPA.
6. **Languages for v1.** Which of EN, ES, ZH, PA, AR, RU. Script variants. Translation method (LLM plus a native reviewer?). Number tokens locked. Mocks in English by default with a native toggle. TTS yes/no.
7. **Item-bank pipeline.** Who or what generates the isomorphic, flipped-stem (NOT/EXCEPT/TF) variants. Verification: numeric check against `numbers.json`, a second-model consistency check, human spot-check rate. Target bank size per test. Item rules for conflicts inside the handbook (X4, P "10 or more" vs ">10", GK vs CV hand valve, 50→90/3 min vs 85→100/45 s): never offer both as options; tag each item with its test.
8. **Mastery, readiness and learner-model settings.**
   - Readiness from unseen-only mocks or the START-HERE sets.
   - Whether to show P(pass), and in which bands.
   - BKT P(T) and tempering (fix the Δ bug).
   - Whether to add confidence buttons (Sure/Unsure/Guess). They are extra friction for beginners.
   - FSRS MCQ grade mapping.
9. **Gamification policy.** Resolve X5. Publish the allowed/forbidden mechanics list (timed sims, lives, streak freeze, collections). Rule: games never write to the learner model.
10. **In-app AI.** Include an LLM tutor or explainer or not; which provider/key; offline fallback. The user asked for an "intelligent system", which may mean rule-based diagnosis only.
11. **Video policy.** Ship curated third-party YouTube (index-only, possibly outdated or conflicting) with "Handbook wins" banners and click-to-load nocookie embeds, or ship v1 with zero video and in-app SVG sims only. Name who verifies and when to re-verify.
12. **Depth of skills-test prep.** Pre-trip verbal scripts per vehicle type, voice "say it aloud" drill (Web Speech availability UNKNOWN), air-brake check sim, backing/alley-dock sims, road-test critical-error trainer. All need a source (DL 650 sections 11–13).
13. **Volatile regulatory content.** Whether to include English-only testing status, non-domiciled eligibility, ELDT/TPR, fees and NRII. Use a dated config with source URLs and a last-verified date. Eligibility checker yes/no, given legal-advice risk.
14. **Legal posture.** Product name and domain without "DMV". "Not affiliated with DMV/CHP/FMCSA" disclaimer. Verbatim TYK and handbook text vs paraphrase plus page cite. Monetization (free core, no paywalled answers).
15. **Self-test protocol.** One unified rubric: content fidelity, learning, UX, visual, i18n/RTL, accessibility, honesty. Also decide:
    - Pass rule: every dimension ≥8, or weighted ≥8.
    - Hard fails that cap the score at 5.
    - An independent evaluator (a separate agent) scoring the screenshots against anchored criteria.
    - Persona runs: a beginner Class A in English, Class B bus in Spanish, Class C HazMat in Arabic RTL, on mobile at 390×844.
    - Per-step re-score loop and a Playwright flow list.

Sources:
- [driving-tests.org CA HazMat](https://driving-tests.org/california/ca-cdl-hazmat-practice-test/)
- [nextdoordriving CA HazMat](https://nextdoordriving.com/california/ca-cdl-hazmat-practice-test)
- [voltexam CA HazMat](https://www.voltexam.com/cdl-hazmat-california)
- [DMV Section 9 Hazardous Materials (search snippet)](https://qr.dmv.ca.gov/portal/es/handbook/commercial-driver-handbook/section-9-hazardous-materials/)
- [DMV Section 13 Road Test (search snippet)](https://www.dmv.ca.gov/portal/es/handbook/commercial-driver-handbook/section-13-road-test/)
- [DMV Statewide Evaluation of CDL Written Knowledge Tests (search snippet)](https://www.dmv.ca.gov/portal/file/statewideevaluationofcommercialdriverslicensewrittenknowledgetests)
- [drivetrucks — fail my CDL test (3-day claim)](https://drivetrucks.com/blog/fail-my-cdl-test/)
- [YouTube hiobnwMv2cw (index only)](https://www.youtube.com/watch?v=hiobnwMv2cw)
- [DL 650 PDF (blocked)](https://www.dmv.ca.gov/web/eng_pdf/comlhdbk.pdf)
- [DL 650 mirror (blocked)](https://rc-hr.com/files/migrated/Portals/23/Handbooks/comlhdbk.pdf)
