# Question Typology + Answering Guidance — Research Ground for the Question Specialist

Research date: 2026-07-02. Machine-facing. Grounds the question-specialist KBs: what item
formats the two target exams actually use, the science of writing good MCQs, per-format
answering strategy ("directive learning"), and how question banks structure item metadata.

## 0. Verification notes (read first)

- [FACT] The sandbox egress proxy denies CONNECT to general web hosts (confirmed this
  session via `$HTTPS_PROXY/__agentproxy/status`: 403 for `jcnde.ada.org`, `nbme.org`,
  `dental.ufl.edu`, `en.wikipedia.org`, `ia*.us.archive.org`, `ets.org`). Same constraint
  hit by the sibling files `exam_dental_us_fmg.md` and `exam_toefl.md`.
- Consequence: every "[FACT]" below sourced to an official page was verified through
  **search-engine excerpts quoting that named page**, not by reading the page body. The
  official URLs are cited so a run with fetch access can verify verbatim. Where only
  third-party prep sites carry a detail, that is stated and downgraded to [ESTIMATE].
- Cross-references: TOEFL task enumeration cross-checked against the sibling research file
  `/home/user/automation/FACTORY/study_system/research/exam_toefl.md` (same session date,
  same sourcing method); INBDE identity against `research/exam_dental_us_fmg.md`.

---

## 1. INBDE item formats (US dental knowledge exam for foreign graduates)

Exam identity: INBDE (Integrated National Board Dental Examination), run by JCNDE under
the ADA — pinned in `research/exam_dental_us_fmg.md`. [FACT] https://jcnde.ada.org/inbde

### 1.1 Formats actually used (from the official guides)

- [FACT] "The INBDE relies exclusively on multiple-choice items, some of which are
  presented in isolation (**standalone items**) while others are presented together in
  groups that are accompanied by a common set of stimuli (case materials including
  radiographic images, photographs, charts, etc.)." Source: INBDE Item Development Guide
  (JCNDE) https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_item_development_guide.pdf
  (via search excerpt; mirror: https://dental.ufl.edu/files/2018/06/INBDE_Item_Development_Guide.pdf).
- [FACT] All items are **one-best-answer**: "only one of the responses is considered the
  correct, or best option," with **3 to 5 answer choices** per item. Source: INBDE 2026
  Candidate Guide https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_candidate_guide.pdf
  (via search excerpt).
- [FACT] Grouped-item vocabulary from the Item Development Guide:
  - **Itemset** = group of **3–6 items** associated with a **Patient Box only**.
  - **Case** = group of **3–6 items** associated with a **Patient Box plus an image or
    other stimulus** (radiograph, photo, chart).
  - The **Patient Box** is a standardized patient-summary table (demographics, chief
    complaint, history, findings) usable with standalones as well as sets; the guide says
    it exists to enable direct skill assessment while "avoiding unnecessary verbiage."
- [FACT] No fixed quota of standalones vs. sets: "The INBDE does not at present contain a
  predetermined number or percentage of items to be allocated to standalones as opposed to
  these item sets" — test construction teams pick whichever format best tests the concept.
  Source: Item Development Guide via search excerpt.
- [FACT — negative finding] Searches against the official Candidate Guide and Item
  Development Guide surfaced **no multiple-response ("select all that apply"), ordering,
  drag-and-drop, or hotspot formats**. The guide language ("exclusively multiple-choice…
  only one correct response") excludes them. The brief's hypothesis of multiple-correct /
  ordering items is **not supported** for the INBDE. [UNKNOWN residual: whether unscored
  pretest items ever pilot other formats — not stated in any excerpt seen.]

### 1.2 Delivery structure and per-item time budget

| Day / section | Items | Format | Time | Sec/item |
|---|---|---|---|---|
| Day 1, sections 1–3 | 100 each (300) | standalone | 105 min each | ~63 s |
| Day 1, section 4 | 60 | case-based | 105 min | ~105 s |
| Day 2, sections 1–2 | 70 each (140) | case-based | 105 min each | ~90 s |

- [ESTIMATE — high confidence] Table triangulated from prep-site readings of the Candidate
  Guide: https://blog.caapidsimplified.com/cracking-the-inbde-your-exam-guide/ ,
  https://dentaistudy.com/blogs/inbde/inbde-day1-vs-day2-breakdown.html . Consistent with
  the official totals (500 items; Day 1 ≈ 8h15m administrative, Day 2 ≈ 4h15m) reported in
  `research/exam_dental_us_fmg.md`. Optional scheduled 15-min breaks between sections;
  unscheduled breaks run the clock. Verify section-level numbers against the Candidate
  Guide PDF when fetchable.
- [FACT] **No penalty for guessing**; pass/fail with a scaled score **49–99, pass = 75**;
  a scaled 75 is NOT 75% correct (scaling accounts for item difficulty, discrimination,
  guessing susceptibility). Sources: JCNDE https://jcnde.ada.org/inbde/inbde-results and
  JCNDE scoring FAQ https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/scoring_nbde_faq.pdf
  (via search excerpts); https://www.kaptest.com/study/inbde/inbde-scoring-system/ .

---

## 2. TOEFL iBT question types per section

Context [FACT]: the TOEFL iBT changed format on 2026-01-21 (section-adaptive Reading and
Listening, ~90 min, 1–6 band scores). Full treatment in `research/exam_toefl.md`. ETS
content page: https://www.ets.org/toefl/test-takers/ibt/about/content.html ; sample test:
https://www.ets.org/toefl/test-takers/ibt/prepare/sample-test-jan-2026-1.html (both
surfaced via search; bodies not fetchable this session).

### 2.1 Current (2026) task types

| Section | Task type | Mechanics | Item format |
|---|---|---|---|
| Reading | Complete the Words | ~70–100-word paragraph; 10 words with second half deleted; type the missing letters | constrained text entry (C-test style) |
| Reading | Read in Daily Life | short practical texts 15–150 words (emails, announcements, posts) | single-answer MC |
| Reading | Read an Academic Passage | ~200-word academic passage + ~5 questions | single-answer MC |
| Listening | Listen and Choose a Response | hear one line (not shown on screen), pick the appropriate reply; ~8 items | single-answer MC |
| Listening | Listen to a Conversation | short daily/campus dialogue + questions | single-answer MC |
| Listening | Listen to an Announcement | 40–85-word announcement + questions | single-answer MC |
| Listening | Academic Talk | short lecture-style talk + questions | single-answer MC |
| Speaking | Listen and Repeat | 7 sentences heard once each, tied to one on-screen picture; repeat exactly | spoken repetition |
| Speaking | Take an Interview | 4 simulated-interview questions on a familiar topic | spoken response |
| Writing | Build a Sentence | assemble/complete sentences | constrained construction |
| Writing | Write an Email | short functional email to a given recipient/purpose | short production |
| Writing | Writing for an Academic Discussion (WAD) | professor's discussion prompt + 2 classmate posts; write your contribution (~10 min, ≥100 words recommended) | timed production |

- [FACT for the enumeration] Sources: ETS content page + sample-test page above via search
  excerpts; corroborated by https://www.toeflresources.com/blog/2026_toefl_format_revealed/ ,
  https://college-council.com/en/blog/toefl-2026-reading-new-tasks-strategies ,
  https://mliesl.edu/contents/the-new-toefl-ibt-2026-all-the-changes-you-need-to-know/ ,
  and the sibling file `exam_toefl.md`. [ESTIMATE] Exact per-type item counts vary with the
  adaptive stage-2 module.
- [FACT] The 2026 test **dropped the legacy integrated tasks and the essay**: "the test
  will no longer contain integrated questions. Nor will it contain an essay task"
  (search excerpt attributed to ETS 2026-update coverage). Implication for the question
  specialist: integrated read-listen-speak note templates apply only to **legacy-format**
  practice material, which still dominates published prep.

### 2.2 Legacy (2023–2025) types — still what most prep banks contain

- [ESTIMATE — high confidence, standard published taxonomy, ETS pages unfetchable this
  session] Reading: factual information, negative factual (EXCEPT/NOT), inference,
  rhetorical purpose, vocabulary-in-context, sentence simplification, insert-a-sentence,
  prose summary (select 3 of 6 — a true multiple-response format), reference.
  Listening: gist-content, gist-purpose, detail, function ("why does the professor say…",
  often with replayed audio), attitude, organization, connecting content (sometimes
  table/multi-select). Speaking: 1 independent + 3 integrated tasks. Writing: integrated
  (read 3 min → listen → 20-min summary of how the lecture responds to the reading) + WAD.
- Note the asymmetry: legacy TOEFL DID use multiple-response and table formats; INBDE does
  not. A question specialist must key strategy to the format actually in front of the user.

---

## 3. The science/craft of good MCQs (NBME item-writing canon)

Primary source: NBME Item-Writing Guide, *Constructing Written Test Questions* —
https://www.nbme.org/educators/item-writing-guide ; PDF
https://www.nbme.org/sites/default/files/2021-02/NBME_Item%20Writing%20Guide_R_6.pdf
(host 403 this session; principles below verified via search excerpts quoting the guide,
plus mirror listings at https://cmsru.rowan.edu/documents/education-documents/assessment-forms-documents/faculty-and-staff/nbme-item-writing-guide-2020.pdf ).

### 3.1 Core rules [FACT — NBME guide via excerpts]

1. **One best answer**: the dominant format; options must be rankable on a single
   dimension with exactly one defensibly best option.
2. **Cover-the-options rule**: a properly focused lead-in lets the test-taker read the
   vignette + lead-in, cover the options, and produce the answer unaided. If they can't,
   the lead-in is unfocused or the item is really a true/false set in disguise.
3. **Avoid negatively phrased lead-ins** (EXCEPT / NOT): they measure reading care more
   than knowledge.
4. **Homogeneous options**: all options grammatically parallel, same category, judged
   true/false on one dimension; distractors "directly related to the lead-in and
   homogeneous with the correct answer" — plausible to the non-master.
5. **Two families of technical flaws**:
   - *Irrelevant difficulty* (confuses everyone): long/complex options, vague frequency
     terms ("often", "usually"), nonparallel options, complicated multi-part stems,
     numeric options not in order, "none/all of the above".
   - *Testwiseness cues* (help the savvy): grammatical cues (option doesn't follow from
     the lead-in), **absolute terms** ("always", "never") marking wrong options, the
     **longest/most detailed option** being correct, word repeats between stem and key,
     logical cues (a subset of options being collectively exhaustive), convergence cues.
   The guide catalogs ~13 flaw categories across these two families. [ESTIMATE on the
   exact count — "13" from a secondary summary of the guide.]
6. **Vignette (clinical stem) structure**: present patient data in the canonical clinical
   order (age/sex/site → chief complaint → history → exam → labs/imaging), then a focused
   lead-in; the stem should contain all data needed and no red-herring padding.
   [ESTIMATE — high confidence; canonical NBME teaching, exact wording unverified this
   session.] The INBDE Patient Box (§1.1) is the same idea made into a fixed template. [FACT]
7. **Test application, not recall**: NBME pushes vignette-based items requiring a decision
   (diagnosis, next step) over "which of the following is true about X" recall items.
   [ESTIMATE — high confidence, core stated purpose of the guide.]

### 3.2 Bloom levels in items

- [FACT] Revised Bloom taxonomy (Anderson & Krathwohl 2001): **remember, understand,
  apply, analyze, evaluate, create**; "synthesis" became "create" and moved above
  evaluate. Sources: https://www.coloradocollege.edu/other/assessment/how-to-assess-learning/learning-outcomes/blooms-revised-taxonomy.html ,
  https://pmc.ncbi.nlm.nih.gov/articles/PMC9727608/ .
- [FACT] Format constrains level: MCQs most easily assess remember/understand; scenario
  (vignette) framing pushes MCQs into apply/analyze; create is effectively out of reach
  for single-answer MC. Source: PMC9727608 (notes the taxonomy's own assumption that
  question format bounds assessable processes).
- Bank tagging convention [ESTIMATE — standard practice]: tag each item with a Bloom level;
  a board-style bank should be mostly apply/analyze (vignette + decision), with
  remember-level items reserved for terminology gates early in the learning ladder.

---

## 4. Answering strategy per format ("directive learning")

Craft layer. [ESTIMATE — medium-high confidence throughout unless marked; synthesized from
the NBME flaw taxonomy (§3), the official structural facts (§1–2), and widely taught
test-strategy practice, e.g. https://www.kaptest.com/study/inbde/inbde-question-types-strategies/ ,
https://truelearn.com/resource-library/5-tips-for-examining-testwiseness-question-flaws-in-usmle-the-writers-series/ .]

### 4.1 Reading a stem (single-best-answer, INBDE standalone)

1. Read the **lead-in (last line) first**, then the vignette — you then know what data
   matters while reading. For short stems, straight-through is fine.
2. **Answer before looking** (the cover-the-options rule inverted into a solving move):
   commit to a predicted answer, then find its match. This immunizes against distractors,
   which are engineered to be plausible (§3.1.4).
3. If no prediction possible → **elimination**: strike options that are (a) true but
   non-responsive to the lead-in, (b) internally false, (c) heterogeneous with the others.
   Exploit writer flaws only as tiebreakers: absolute-term options are usually wrong;
   the odd-one-out grammatically is usually wrong (§3.1.5).
4. **Negative stems** (rare on well-built exams, common in bad banks): rewrite mentally as
   "mark each option true/false; pick the false one."
5. Never leave blanks — no guessing penalty on the INBDE [FACT, §1.2]; TOEFL MC likewise
   has no announced wrong-answer penalty [ESTIMATE — no penalty documented anywhere seen].

### 4.2 Case-cluster strategy (INBDE itemsets/cases)

- **Scan the Patient Box once, systematically** (demographics → complaint → meds/history →
  findings), before item 1; then treat it as a lookup table per item rather than
  rereading. Images: orient first (film type, region), read the question, return.
- Items within a set are **independently scored**; JCNDE writes sets so one item does not
  reveal another's answer, and getting one wrong does not cascade [ESTIMATE — standard
  professional-exam construction; not verified verbatim in the guide]. So: never revise an
  earlier answer just to be "consistent" with a later item.
- Budget: the set shares the section clock — ~105 s/item Day 1 section 4, ~90 s/item
  Day 2 (§1.2). Spend the surplus on the shared stimulus up front, then answer at
  standalone speed.

### 4.3 Time budgeting per item type (working numbers)

| Context | Budget | Rule |
|---|---|---|
| INBDE standalone | ~63 s | first pass ≤45 s; flag and move on at 90 s |
| INBDE case item | ~90–105 s | box-scan ≤60 s per set, then ~60 s/item |
| TOEFL Reading academic (2026) | ~1 min/question | passage is only ~200 words; read fully first |
| TOEFL Complete the Words | ~10 s/blank | first sentence is intact — use it to fix topic |
| TOEFL Listen and Choose a Response | one hearing, no replay | pre-focus on function (request? apology? question type) |
| TOEFL WAD | ~10 min | 1 min read + plan, 7 min write (~100–130 words), 2 min check |

[ESTIMATE — budgets derived from official counts/times in §1.2 and §2.1; the split within
each budget is craft.]

### 4.4 Note templates for TOEFL productive tasks

- **WAD template** (current test): one sentence taking a clear position on the professor's
  question → one reason + concrete example → one sentence engaging a classmate by name
  ("Unlike Ana, I think…" or extending their point) → optional concession. ≥100 words.
  [ESTIMATE — aligned with published ETS WAD scoring emphasis on contribution + relevance.]
- **Write an Email template** (current test): greeting matched to recipient register →
  purpose sentence → 2 details/requests → closing action + sign-off. Register match
  (professor vs. friend) is the scored skill. [ESTIMATE]
- **Legacy integrated writing/speaking note grid** (for practice banks): two columns,
  Reading | Lecture; three rows = three points; capture the lecture's verb of opposition
  per row (refutes / casts doubt / gives counterexample). Response formula: "The lecture
  challenges each of the reading's three claims: First, … however the professor argues …".
  Applies only to pre-2026 material (§2.1). [ESTIMATE — standard, matches legacy task
  mechanics documented in `exam_toefl.md`.]
- **Listen and Repeat**: chunk the sentence prosodically (2–4 word groups), reproduce
  stress pattern over perfect articles; scoring is on intelligibility + accuracy.
  [ESTIMATE — mechanics from §2.1 sources; tactic is craft.]

---

## 5. Question-bank metadata: how banks structure items

What the study system's own bank should record per item, grounded in psychometrics.

### 5.1 Blueprint / content tags

- [FACT] INBDE items are classified on a two-axis blueprint: Foundation Knowledge areas ×
  Clinical Content items (56 clinical content items; enumeration pending official-PDF
  access — see `exam_dental_us_fmg.md`). Source: JCNDE INBDE pages via excerpts.
- Minimum tag set per item [ESTIMATE — synthesis of exam-blueprint practice]:
  `exam` (INBDE/TOEFL), `blueprint_axis1`, `blueprint_axis2`, `format` (standalone |
  itemset | case | task-type per §2.1), `bloom_level`, `stimulus` (none | patient_box |
  image | audio), `learning_objective_id`, `source_ref`.

### 5.2 Classical test theory (CTT) statistics

- [FACT] **Difficulty index (p-value)** = proportion of examinees answering correctly
  (0–1; higher = easier). Working ranges: 0.30–0.70 for discriminating items; 0.80–1.00
  acceptable for mastery items. Sources: https://phoenixmed.arizona.edu/assessment/item-analysis ,
  https://assess.com/item-statistics-classical-test-theory/ .
- [FACT] **Point-biserial (r_pb)** = correlation between the item score (0/1) and total
  test score; ≥0.20 acceptable, ≥0.30 good for discriminating items, ~0.40+ excellent,
  <0.20 poor; negative r_pb flags a miskeyed or defective item. Same sources; also
  https://www.cogn-iq.org/learn/theory/item-discrimination/ .
- [FACT] **Discrimination index D** = p(upper group) − p(lower group), typically upper/
  lower 27%; published medical-education bands: ≤0.20 poor, 0.21–0.24 acceptable,
  0.25–0.34 good, ≥0.35 excellent. Sources: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC12805726/ ,
  cogn-iq page above.
- [FACT] Difficulty bounds discrimination: discrimination is mathematically maximal near
  mid-difficulty and shrinks as p→0 or 1, so the two must be read together. Source:
  cogn-iq item-discrimination page via excerpt.
- **Distractor analysis** [ESTIMATE — standard practice per the Arizona item-analysis
  page]: every distractor should be chosen by someone (>~5% of takers) and should
  correlate negatively with total score; a never-chosen distractor is dead weight
  (functionally reduces a 4-option item to 3).

### 5.3 IRT one-liners

- [FACT] IRT models the probability of a correct response as a logistic function of latent
  ability θ. Nested family: **1PL/Rasch** — difficulty *b* only (b = θ at which P(correct)
  = 0.50, absent guessing); **2PL** adds discrimination *a* (slope at the inflection);
  **3PL** adds guessing asymptote *c* (floor probability for very low θ). Sources:
  https://www.cogn-iq.org/learn/theory/logistic-model/ , https://assess.com/irt-item-difficulty-parameter/ ,
  https://pmc.ncbi.nlm.nih.gov/articles/PMC5875705/ .
- [FACT] JCNDE's scaled scoring explicitly weighs "difficulty, how well it distinguishes
  between strong and weak candidates, and susceptibility to guessing" — i.e., a 3PL-style
  parameterization. Source: JCNDE scoring FAQ via search excerpt (§1.2).
- Bank implication [ESTIMATE]: store per item `p_value`, `r_pb`, `n_exposures`,
  `irt_b` (+ `irt_a`, `irt_c` when calibratable), `last_calibrated`; recalibrate on a
  schedule and quarantine items with r_pb < 0 or dead distractors. IRT *b* is the natural
  difficulty signal to feed the scheduler/adaptive specialist (match item b to learner θ
  to "keep it challenging").

---

## 6. Sources

Official (verified via search excerpts; direct fetch 403 this session):
- JCNDE INBDE: https://jcnde.ada.org/inbde ; results/scoring: https://jcnde.ada.org/inbde/inbde-results
- INBDE Item Development Guide: https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_item_development_guide.pdf
- INBDE 2026 Candidate Guide: https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_candidate_guide.pdf
- JCNDE scoring FAQ: https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/scoring_nbde_faq.pdf
- INBDE sample questions: https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_practice_questions.pdf
- ETS TOEFL content: https://www.ets.org/toefl/test-takers/ibt/about/content.html ;
  2026 sample test: https://www.ets.org/toefl/test-takers/ibt/prepare/sample-test-jan-2026-1.html ;
  2026 updates: https://www.ets.org/toefl/test-takers/ibt/upcoming-updates-jan-2026.html
- NBME Item-Writing Guide: https://www.nbme.org/educators/item-writing-guide ;
  PDF: https://www.nbme.org/sites/default/files/2021-02/NBME_Item%20Writing%20Guide_R_6.pdf

Secondary (used where official bodies unreachable; identified as such above):
- https://blog.caapidsimplified.com/cracking-the-inbde-your-exam-guide/
- https://dentaistudy.com/blogs/inbde/inbde-day1-vs-day2-breakdown.html
- https://www.kaptest.com/study/inbde/inbde-question-types-strategies/ ; https://www.kaptest.com/study/inbde/inbde-scoring-system/
- https://www.toeflresources.com/blog/2026_toefl_format_revealed/ ; https://college-council.com/en/blog/toefl-2026-reading-new-tasks-strategies ; https://mliesl.edu/contents/the-new-toefl-ibt-2026-all-the-changes-you-need-to-know/
- https://truelearn.com/resource-library/5-tips-for-examining-testwiseness-question-flaws-in-usmle-the-writers-series/
- https://phoenixmed.arizona.edu/assessment/item-analysis ; https://assess.com/item-statistics-classical-test-theory/ ; https://assess.com/irt-item-difficulty-parameter/
- https://www.cogn-iq.org/learn/theory/item-discrimination/ ; https://www.cogn-iq.org/learn/theory/logistic-model/
- https://www.ncbi.nlm.nih.gov/pmc/articles/PMC12805726/ ; https://pmc.ncbi.nlm.nih.gov/articles/PMC5875705/ ; https://pmc.ncbi.nlm.nih.gov/articles/PMC9727608/
- https://www.coloradocollege.edu/other/assessment/how-to-assess-learning/learning-outcomes/blooms-revised-taxonomy.html

Sibling ground-truth files: `research/exam_dental_us_fmg.md`, `research/exam_toefl.md`.
