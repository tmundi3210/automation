# High-Quality Flashcards + Book Decomposition — Ground Truth

Scope: (A) what makes a flashcard high-quality — Twenty Rules (Wozniak),
minimum information principle, cloze, image occlusion, images vs text, card
templates, bad-card patterns + fixes, "edges" as relation-cards; (B) academic
book decomposition — textbook anatomy, chunking research, Bloom as layering
ladder, prerequisite mapping (graph AND flat options), the owner ladder mapped
to named theory, concrete book→cards pipeline. Grounds the content-engineering
specialist (owner brief §2a-8/9, Q5/Q6).

Research constraint (read first): the session egress policy returned **HTTP 403**
for direct fetches (WebFetch AND curl) of `supermemo.com`, `super-memory.com`,
`supermemo.guru`, `docs.ankiweb.net`, `en.wikipedia.org`, `cmap.ihmc.us` and
essentially all external hosts — same constraint documented in the sibling exam
files. Facts below were confirmed via **search-engine snippets of the official
pages and of verbatim mirrors**, not by reading the pages directly. Tags:
[FACT] = confirmed in snippet text of the cited source; [ESTIMATE] = inference
or training-knowledge attribution with stated basis; [UNKNOWN] = could not
confirm. Canonical URLs are cited so a downstream fetch-enabled run can verify.

---

## PART A — HIGH-QUALITY FLASHCARDS

### A1. The Twenty Rules of Formulating Knowledge (Wozniak, 1999)

Author: Dr. Piotr Wozniak (founder of SuperMemo), compiled 1999. Rules are
ordered by importance — the earlier rules are "most often violated or bring
most benefit if complied with." [FACT — search snippet of the official article]
Canonical URLs (fetch blocked this session; titles below confirmed piecewise
across multiple search snippets of the official article and verbatim mirrors):
- https://www.supermemo.com/en/blog/twenty-rules-of-formulating-knowledge (current official)
- https://super-memory.com/articles/20rules.htm (original location)
- https://supermemo.guru/wiki/20_rules_of_knowledge_formulation (wiki version)

| # | Rule | One-line gloss |
|---|------|----------------|
| 1 | Do not learn if you do not understand | Memorizing un-understood text costs huge time and the knowledge is worthless. [FACT — snippet] |
| 2 | Learn before you memorize | Build an overall picture first; pieces that fit one coherent structure memorize faster. [FACT — snippet] |
| 3 | Build upon the basics | Start simple; don't hesitate to memorize obvious basics — their cost is small. [FACT — snippet] |
| 4 | Stick to the minimum information principle | Formulate as simply as possible: simple is easy to remember; complex answers mean more to forget. [FACT — snippet] |
| 5 | Cloze deletion is easy and effective | Fill-the-blank items are the fastest way to convert text to good items. [FACT — snippet confirms title] |
| 6 | Use imagery | A picture is worth a thousand words; visual items are far easier to retain. [FACT — snippet confirms title; gloss per article, ESTIMATE-high on exact wording] |
| 7 | Use mnemonic techniques | Peg lists, mind maps, etc. for otherwise intractable items. [FACT — snippet confirms title] |
| 8 | Graphic deletion is as good as cloze deletion | Occlude part of an image (e.g. anatomy figure) and ask for the missing part. [FACT — snippet] |
| 9 | Avoid sets | Unordered collections are "virtually un-memorizable unless you convert them into enumerations." [FACT — snippet] |
| 10 | Avoid enumerations | Ordered lists are also hard; if unavoidable, deal with them via cloze deletion. [FACT — snippet] |
| 11 | Combat interference | Similar items confuse each other; make Q and A maximally unambiguous. [FACT — snippet] |
| 12 | Optimize wording | Reduce sentences like equations — compact, minimal-trigger wording speeds recall. [FACT — snippet] |
| 13 | Refer to other memories | Anchor new items to existing memories; coherent structure resists forgetting. [FACT — snippet] |
| 14 | Personalize and provide examples | Personal examples are among the most effective ways to build on existing memories. [FACT — snippet] |
| 15 | Rely on emotional states | Emotion-laden formulations (vivid, personal, even shocking) leave stronger traces. [FACT — snippet] |
| 16 | Context cues simplify wording | A category prefix (e.g. "chem:", "bio:") replaces long qualifying phrases and fights interference. [FACT — snippet confirms concept; title wording ESTIMATE-high] |
| 17 | Redundancy does not contradict minimum information | The same fact viewed from different angles (multiple items) is welcome; reasoning steps may be memorized too. [FACT — snippet] |
| 18 | Provide sources | Source annotations let you update knowledge and judge reliability later. [FACT — snippet] |
| 19 | Provide date stamping | Stamp volatile facts (statistics!) with their collection year to manage obsolescence. [FACT — snippet] |
| 20 | Prioritize | You will always face more knowledge than you can master; choose deliberately. [FACT — snippet] |

Numbering note: rules 1, 2, 4, 5, 17, 18, 19, 20 were explicitly numbered in
snippets; remaining positions follow the canonical article order [ESTIMATE —
high confidence; verify when fetch access exists]. The article notes "the
first 16 rules revolve around making memories simple"; 17–20 are
meta-management. [FACT — snippet]

### A2. Minimum information principle (the core rule, expanded)

- One card = one atomic fact/relation. Complex answers → split into many simple
  items: simple items are answered fast, scheduled accurately (the scheduler
  grades ONE memory trace, not a bundle), and fail independently. [FACT on the
  principle per Rule 4 snippet; scheduling rationale [ESTIMATE — direct
  consequence of per-item scheduling, high confidence]]
- Violation symptom: a chronically "half-known" card whose grade is
  meaningless. Fix: split. [ESTIMATE — standard practice, high confidence]
- Complement, not contradiction: Rule 17 permits redundancy — the SAME fact
  from several angles as separate simple items. [FACT — snippet]
- Why question-format at all: the testing effect — retrieval practice beats
  restudying for delayed retention (Roediger & Karpicke 2006, *Test-Enhanced
  Learning*, Psychological Science; on delayed tests prior testing outperforms
  prior studying). [FACT — search snippets;
  https://journals.sagepub.com/doi/10.1111/j.1467-9280.2006.01693.x]

### A3. Cloze deletion

- Definition: sentence with a hidden fragment; learner supplies the fragment.
  Wozniak: "easy and effective," the fastest path from source text to items
  (Rule 5). [FACT — snippet]
- Anki syntax: wrap hidden text in `{{c1::hidden text}}`; `{{c2::...}}`,
  `{{c3::...}}` on the same note generate SEPARATE cards; holding Alt/Option
  reuses the same number so several blanks hide together on one card; nested
  clozes supported from Anki 2.1.56 (depth-limited, 3 levels in 24.11).
  [FACT — search snippets citing https://docs.ankiweb.net/editing.html and
  Anki forums]
- Quality guardrails: the deleted span must be SHORT (minimum information);
  the remaining sentence must uniquely determine the answer (no "guess which
  of five similar facts this is" — interference); keep the surrounding context
  meaningful, not a bare list skeleton. [ESTIMATE — direct application of
  Rules 4/11/12, high confidence]

### A4. Image occlusion (graphic deletion)

- Wozniak Rule 8: occluding part of an image is "as good as cloze deletion" —
  e.g. an anatomy illustration with one small area masked; the task is to name
  the missing area. [FACT — snippet]
- Anki implements this NATIVELY since version 23.10 as the **Image Occlusion**
  note type: "a special case of cloze deletion based on images instead of
  text" — rectangles/shapes mask regions of an image, one card per mask (or
  hide-all/reveal-one modes). [FACT — search snippets citing
  https://docs.ankiweb.net/editing.html]
- Pre-23.10 the same function was the third-party add-on **Image Occlusion
  Enhanced** (https://ankiweb.net/shared/info/1374772155). [FACT — snippet
  confirms the add-on page exists; authorship (Glutanimate) is [ESTIMATE —
  training knowledge, unverified this session]]
- Best targets: anatomy plates, dental charts/tooth numbering, maps, labeled
  diagrams, flowchart nodes, table cells. [ESTIMATE — direct extension of
  Rule 8's anatomy example to the owner's dental domain, high confidence]

### A5. When images/flowcharts beat text

- **Picture superiority effect**: pictures are remembered better than words;
  in Paivio's experiments people recalled roughly twice as many images as
  words after brief viewing. [FACT — search snippets;
  https://en.wikipedia.org/wiki/Picture_superiority_effect]
- **Dual coding theory** (Paivio): pictures get encoded via BOTH a visual and
  a verbal trace → two retrieval paths; words get one. Note: recent work
  (Higdon et al. 2025, QJEP/Sage) argues distinctiveness, not dual coding,
  explains the effect — the EFFECT is robust, the mechanism is debated.
  [FACT — search snippets; https://journals.sagepub.com/doi/10.1177/17470218241235520]
- **Multimedia principle** (Mayer): people learn better from words + pictures
  than from words alone; separate visual and auditory/verbal processing
  channels share the load on working memory. [FACT — search snippets;
  overview: https://www.hartford.edu/faculty-staff/faculty/fcld/_files/12%20Principles%20of%20Multimedia%20Learning.pdf]
- Practical rule for the card factory [ESTIMATE — synthesis, medium-high
  confidence]: use an image/occlusion card when the knowledge IS spatial or
  structural (anatomy, apparatus, chart positions, flow order); use a flowchart
  occlusion when the knowledge is a decision path or sequence (e.g. clinical
  management algorithms); use text/cloze when the knowledge is verbal-symbolic
  (definitions, numbers, names). Decorative images add load, not memory
  (Mayer's coherence principle — extraneous material hurts). [FACT on the
  coherence principle existing — snippet of Mayer's 12 principles lists]

### A6. Card templates (Anki's built-in note types)

| Template | Behavior | Use for |
|----------|----------|---------|
| Basic | Front→Back, one card | one-directional fact ("What nerve innervates X?") [FACT — Anki manual snippets, https://docs.ankiweb.net/getting-started.html] |
| Basic (and reversed card) | generates Front→Back AND Back→Front | symmetric pairs: term↔definition, drug↔class [FACT — snippet: "a reversed card (back→front) will also be created"] |
| Basic (optional reversed card) | reverse card only if the "Add Reverse" field is non-empty | pairs where only SOME items deserve a reverse [FACT on the mechanism per snippet; exact type name [ESTIMATE — training knowledge of the manual, high confidence]] |
| Basic (type in the answer) | learner types the answer, diffed against the field | spellings, terminology, TOEFL vocabulary [ESTIMATE — training knowledge of the Anki manual, high confidence; not snippet-confirmed this session] |
| Cloze | `{{c1::...}}` deletions, one card per cloze number | facts embedded in sentences, enumerations [FACT — snippet] |
| Image Occlusion | masks over image regions, cloze-of-images | diagrams, anatomy, charts (Anki ≥ 23.10) [FACT — snippet] |

- Reversed-card caution [ESTIMATE — standard practice + Rule 11, high
  confidence]: only auto-reverse when the back uniquely identifies the front;
  a reversed card whose "question" is a generic definition shared by several
  terms manufactures interference.

### A7. What makes a BAD card — the documented failure patterns and fixes

| Bad pattern | Why it fails | Documented fix |
|-------------|--------------|----------------|
| **Overloaded card** (violates Rule 4) | many sub-answers = one grade for many traces; chronic lapses | split into atomic items; keep redundancy per Rule 17 [FACT — Rules 4/17 snippets] |
| **Set** ("name the members of X", unordered) | "larger sets are virtually un-memorizable unless you convert them into enumerations" | convert set → enumeration (impose an order/grouping), then enumeration → clozes [FACT — Rule 9 snippet wording; the two-step chain is the article's own prescription, snippet-confirmed for step 1; step 2 per Rule 10] |
| **Enumeration** (ordered list asked whole) | long serial recall, uneven mastery | cloze each element separately (one blank per card); overlapping clozes so each item is cued by its neighbors; use mnemonics (Rule 7) for stubborn orders [FACT — Rule 10 snippet says "can be dealt with using cloze deletion"; overlapping-cloze technique documented in Anki community, e.g. https://eshapard.github.io/anki/learning/code/python/overlapping-clozes.html] |
| **Interference pair** (two similar items keep swapping answers) | "even the simplest items can be completely intractable if they are similar to other items" | reword to maximize contrast; add discriminating context cue (Rule 16); add an explicit A-vs-B comparison card; sometimes delete one item [FACT — Rule 11 snippet for the diagnosis; the fix set is the article's, snippet-confirmed in part; comparison-card fix is [ESTIMATE — common practice, medium-high confidence]] |
| **Ambiguous question** (multiple correct answers, only one accepted) | grading becomes noise | tighten wording until the answer is unique (Rule 12) [FACT — Rule 12 snippet on wording; consequence chain ESTIMATE-high] |
| **Orphan card** (fact with no understood context) | violates Rules 1–3; rote junk | learn the chapter first, link the card to prior items (Rule 13), add an example (Rule 14) [FACT — Rules 1/2/13/14 snippets] |
| **Undated volatile fact** (statistics, prices, guidelines) | silently goes stale | date-stamp (Rule 19) and source-stamp (Rule 18) — for this factory: every generated card carries its KB source id [FACT on Rules 18/19 — snippets; the KB-id policy is [ESTIMATE — house design decision]] |

### A8. "Edges" read as relation-cards (owner's term)

[ESTIMATE throughout — "edges" is the owner's word, not a literature term;
this section is the interpretation, flagged as such.]

- Reading: the owner's "edges" = the **relations between two concepts**,
  i.e. the edges of a concept graph. The closest literature object is the
  concept-map **proposition**: "two or more concepts connected using linking
  words to form a meaningful statement" (Novak & Cañas 2008). [FACT that
  propositions are defined that way — search snippet of
  https://cmap.ihmc.us/publications/researchpapers/theoryunderlyingconceptmaps.pdf;
  the equation edges≈propositions is [ESTIMATE — high confidence]]
- An edge is a triple **(A, relation, B)**: (lidocaine, blocks, Na+ channels);
  (maxillary first molar, has-root-count, 3). Each triple yields a small card
  family [ESTIMATE — design synthesis]:
  1. Forward: "What does A ⟨relation⟩?" → B
  2. Reverse (only if unique — Rule 11): "What ⟨relation⟩ B?" → A
  3. Relation card: "How are A and B related?" → the linking phrase
  4. Cloze form: "A ⟨relation⟩ {{c1::B}}."
- Edge cards ARE the Rule 13 ("refer to other memories") mechanism in card
  form: every edge card touches two already-learned nodes, so reviewing edges
  continuously re-binds the graph. [ESTIMATE — direct mapping to Rule 13,
  high confidence]
- Practical constraint: an edge card is only well-formed after BOTH endpoint
  term-cards exist (see the ladder, B5). [ESTIMATE — prerequisite logic,
  high confidence]

---

## PART B — BOOK DECOMPOSITION

### B1. Anatomy of an academic textbook

Standard publishing structure [FACT for the pedagogical features — confirmed
via OpenStax Anatomy & Physiology front-matter snippets, e.g.
https://courses.lumenlearning.com/openstax-anatomyandphysiology/front-matter/preface-2/;
the front/back-matter partition is standard publishing convention,
[ESTIMATE — high confidence, not fetched from a style manual this session]]:

- **Front matter**: title page, copyright, table of contents, preface (audience,
  approach), acknowledgments. Machine value: the TOC is the free unit map.
- **Body — chapter skeleton** (the exploitable pattern; OpenStax-style texts
  make it explicit):
  - chapter introduction / opening case
  - **learning objectives per section** ("objectives stated at the beginning of
    each section") [FACT — snippet]
  - sections/subsections with **key terms bolded**, diagrams, tables
  - boxed features (clinical notes, examples, "interactive links")
  - **chapter review/summary** ("chapter reviews summarize key concepts") [FACT — snippet]
  - **end-of-chapter questions**: review questions + critical-thinking, often
    case-based [FACT — snippet: "review and critical thinking questions that
    are often case-based"]
- **Back matter**: glossary ("a glossary of terms appears at the end"),
  appendices, references/bibliography, **index**. [FACT — snippet for glossary/index]
- Extraction leverage [ESTIMATE — synthesis, high confidence]: learning
  objectives ≈ ready-made unit goals (usually already Bloom-verb phrased);
  bold terms + glossary ≈ the terminology layer for free; end-of-chapter
  questions ≈ a validation set for generated cards (if our cards can't answer
  the book's own questions, decomposition missed content); the index ≈ a
  concept-frequency signal (heavily-indexed terms are hub nodes).

### B2. Chunking research (why units and cards must be small)

- Miller 1956, "The Magical Number Seven, Plus or Minus Two": immediate memory
  holds ~7±2 chunks. [FACT — search snippets;
  https://en.wikipedia.org/wiki/The_Magical_Number_Seven,_Plus_or_Minus_Two]
- Cowan 2001, "The magical number 4 in short-term memory" (Behavioral and
  Brain Sciences): controlling for rehearsal, the real focus-of-attention limit
  is **~4 chunks**. [FACT — search snippets;
  https://www.cambridge.org/core/journals/behavioral-and-brain-sciences/article/magical-number-4-in-shortterm-memory-a-reconsideration-of-mental-storage-capacity/44023F1147D4A1D44BDC0AD226838496]
- A **chunk** is a meaningful unit, not an item count: a chess master sees
  familiar configurations, not 32 pieces; "expertise is partly the development
  of increasingly complex chunks." [FACT — snippet; classic attribution Chase &
  Simon 1973 is [ESTIMATE — standard attribution, high confidence, not fetched]]
- Consequences for decomposition [ESTIMATE — direct application, high
  confidence]: a card's question should demand ≤ ~4 not-yet-chunked elements
  at once; a study unit should introduce a handful of new concepts, not
  dozens; the LADDER works because each rung turns last rung's items into
  chunks, freeing working-memory slots for the next composition.
- Same mechanism, instructional side: **cognitive load theory** — unguided
  problem-solving overloads working memory; **worked examples** (Sweller &
  Cooper 1985, Cognition and Instruction 2(1) 59–89) let novices learn the
  schema first: students who studied worked examples solved similar problems
  in ~half the time with ~1/5 the errors. Caveat: **expertise reversal
  effect** — worked examples lose value and can hurt as expertise grows, so
  fade them out. [FACT — search snippets;
  https://en.wikipedia.org/wiki/Worked-example_effect]

### B3. Bloom's taxonomy (revised) as the layering ladder

Anderson & Krathwohl 2001, *A Taxonomy for Learning, Teaching, and Assessing*
(revision of Bloom 1956; verbs instead of nouns). [FACT — search snippets;
overview by Krathwohl 2002: https://cmapspublic2.ihmc.us/rid=1Q2PTM7HL-26LTFBX-9YN8/Krathwohl%202002.pdf]

- **Cognitive process dimension** (six levels): Remember (retrieve knowledge) →
  Understand (determine meaning) → Apply (use a procedure) → Analyze (break
  into components, see how they relate) → Evaluate (judge by criteria) →
  Create (produce new work). [FACT — snippets]
- **Knowledge dimension** (added in the revision, orthogonal axis): Factual →
  Conceptual (interrelationships among elements) → Procedural →
  Metacognitive. [FACT — snippets]
- Use in this factory [ESTIMATE — design mapping, high confidence]: the
  process dimension is the LADDER (what a question demands); the knowledge
  dimension is the CONTENT TYPE (what a KB entry is). A card is fully typed by
  the pair, e.g. (Remember, Factual) = term card; (Understand, Conceptual) =
  edge card; (Apply, Procedural) = worked-step card; (Analyze, Conceptual) =
  multi-step inference card. Exams (INBDE-style case items) live mostly at
  Apply/Analyze — the deck must climb there, not stop at Remember.

### B4. Prerequisite / dependency mapping — graph option AND flat option

Owner said "graphs, or maybe not the graphs" — both options are kept live;
they share the same extracted data (terms + edges), so the choice is
reversible downstream. [FACT that owner is unsure — brief §2a-9]

**Option G — concept graph / knowledge structure:**
- Concept maps (Novak & Cañas 2008, IHMC): concepts in boxes, labeled linking
  lines; concept + link + concept = a proposition; grounded in Ausubel's
  meaningful-learning theory (new knowledge attaches to existing cognitive
  structure); maps are built to answer a **focus question**. [FACT — search
  snippets of https://cmap.ihmc.us/publications/researchpapers/theoryunderlyingconceptmaps.pdf;
  focus-question detail [ESTIMATE — training knowledge of the paper, high
  confidence, not snippet-confirmed]]
- Knowledge Space Theory (Doignon & Falmagne 1985): a learner's **knowledge
  state** = the set of items mastered; a **surmise/prerequisite relation**
  (q ≤ q′ iff mastering q′ implies q) constrains which states exist and which
  **learning paths** are admissible; the **fringe** of a state = items exactly
  one step ahead, "maximal diagnostic value" — this is what to teach/test
  next. Deployed commercially in ALEKS. [FACT — search snippets;
  https://en.wikipedia.org/wiki/Knowledge_space and
  https://www.aleks.com/about_aleks/knowledge_space_theory]
- Graph pros [ESTIMATE]: precise next-item selection (fringe), weakness
  localization (failed node → suspect its prerequisites), edge cards fall out
  directly. Cons: expensive to build correctly, brittle if edges are guessed,
  overkill for linear material (TOEFL grammar).

**Option F — flat units with ordering (no explicit graph):**
- Chapters/sections become a SEQUENCE of units; prerequisites approximated by
  "everything earlier" — the textbook's chapter order already encodes this
  (authors linearized the graph for you). [ESTIMATE — synthesis, high confidence]
- Pros [ESTIMATE]: cheap, robust, sufficient for scheduler + gamification
  (units = calendar boxes). Cons: no fine-grained weakness tracing;
  cross-chapter relations still need storing somewhere.
- **Recommended hybrid** [ESTIMATE — design recommendation]: flat units as the
  spine (scheduling, progress UI) plus a lightweight edge TABLE (term_a,
  relation, term_b, unit_id) extracted anyway for edge cards — graph-shaped
  data without graph-dependent machinery; upgrade to full KST only if
  adaptive selection later demands it.

### B5. The owner's ladder, mapped to named theory

Owner's ladder (verbatim intent): terminology → basic things → relate two
things → those two make something else → two-step/three-step thinking.
[FACT — owner brief §2a-9] The mapping below is [ESTIMATE — medium-high
confidence; each anchor citation is FACT per B2/B3 above]:

| Rung | Owner's phrase | Bloom process × knowledge | Literature anchor | Card form |
|------|----------------|---------------------------|-------------------|-----------|
| 0 | (implicit: read/understand the unit) | — (Wozniak Rules 1–2) | comprehension before memorization | none — reading pass |
| 1 | "first we need to know terminology" | Remember × Factual | glossary layer; chunk formation begins (Miller/Cowan) | Basic / type-in / reversed term cards |
| 2 | "learn basic things" | Remember–Understand × Factual | minimum-information single facts | cloze, image occlusion |
| 3 | "relate two things" | Understand × Conceptual | concept-map propositions (Novak); the "edges" | edge/relation cards (A7/A8) |
| 4 | "those two things make something else" | Apply × Procedural/Conceptual | worked examples (Sweller & Cooper) — study composed procedure before solving | worked-step cards, "given A and B, what follows?" |
| 5 | "two-step or three-step thinking" | Analyze (→Evaluate) × Conceptual | expertise reversal: fade worked examples into problems; chains of 2–3 edges; KST fringe = next reachable inference | multi-step case/vignette questions (exam-style) |

- Why the rung order is load-correct [ESTIMATE — high confidence]: each rung's
  outputs become single chunks at the next rung, keeping every question within
  the ~4-chunk working-memory budget (Cowan 2001). A rung-5 question asked of
  a learner without rung-1 vocabulary is unanswerable not because of missing
  logic but because every term costs a working-memory slot.
- Rung 5 upper bound: chains longer than 2–3 steps should be split into
  intermediate cards (Rule 17 explicitly blesses memorizing derivation steps).
  [FACT on Rule 17 permitting this — snippet; the 2–3 cap matches the owner's
  own phrasing]

### B6. Concrete decomposition pipeline: book → cards

[ESTIMATE — design synthesis of A+B; the per-stage citations are FACT as
tagged above. This is the content-engineering specialist's operating loop.]

```
BOOK ──1──> UNITS ──2──> TERMS ──3──> CLAIMS ──4──> RELATIONS(edges)
                                        │                │
                                        └───5────────────┴──> MULTI-STEP QUESTIONS
                                                                   │
                             6: card compilation (all layers) <────┘
```

1. **Book → units.** Parse TOC + chapter/section boundaries; carry over each
   section's learning objectives (they arrive Bloom-phrased — B1) and
   end-of-chapter questions (held out as a validation set). Unit size target:
   one sitting; a handful of new concepts (B2). Output: `unit(id, title,
   objectives[], source_pages, order)`.
2. **Unit → terms.** Harvest bold/glossary terms + index hits; one record per
   term: `term(id, unit_id, name, definition, image?)`. Rung-1 material.
3. **Terms → claims.** Extract atomic factual statements (one subject, one
   predicate, one value) — minimum information at the DATA level so cards
   inherit it; stamp source (Rule 18) and date if volatile (Rule 19):
   `claim(id, unit_id, text, terms[], source, date?)`. Rung-2 material.
4. **Claims → relations.** For every claim touching ≥2 terms, normalize an
   edge `(term_a, relation, term_b, unit_id)` — the concept-map proposition
   (B4), the owner's "edges." Rung-3 material; also the (optional) graph.
5. **Relations → multi-step questions.** Compose 2–3-edge chains and
   objective-aligned scenarios into Apply/Analyze questions; write each with
   its worked solution first (worked-example ordering — B2), then a
   problem-only variant to fade into (expertise reversal). Rung-4/5 material.
6. **Everything → cards.** Compile: terms → basic/reversed/type-in; claims →
   cloze (+ image occlusion where the claim is spatial — A5); relations → edge
   card families (A8); multi-step → case cards. Apply the bad-card lint (A7):
   split sets→enumerations→clozes, interference check across the WHOLE deck
   (Rule 11 is a global property), uniqueness check on reversed cards.
   Export `.apkg` / AnkiConnect per the spaced-repetition research file.
7. **Gate.** The deck must answer the book's own end-of-chapter questions
   (coverage) and every card must trace to a claim/edge with a source
   (honesty). Failures loop back to stage 3.

### B7. Open items

- [UNKNOWN] Exact verbatim text of all 20 rule headings/glosses — official
  pages 403-blocked; reconstructed from snippets of the article + mirrors.
  Top follow-up when egress allows.
- [UNKNOWN] Anki manual's full current note-type list — "Basic (optional
  reversed)" and "type in the answer" unconfirmed this session.
- [UNKNOWN] Whether image-occlusion cards outperform equivalent text clozes
  for dental/anatomy material specifically — no controlled card-format A/B
  study located; picture-superiority evidence (A5) is generic recall.
- [UNKNOWN] Focus-question detail of Novak & Cañas 2008 — attributed from
  training knowledge, PDF fetch blocked.

---

## Sources (canonical URLs; all confirmed via search snippets — direct fetch blocked by session egress policy except where noted)

1. Wozniak, Twenty Rules of Formulating Knowledge (1999) — https://www.supermemo.com/en/blog/twenty-rules-of-formulating-knowledge ; original: https://super-memory.com/articles/20rules.htm ; wiki: https://supermemo.guru/wiki/20_rules_of_knowledge_formulation
2. Anki Manual — note types, cloze, image occlusion: https://docs.ankiweb.net/getting-started.html ; https://docs.ankiweb.net/editing.html
3. Image Occlusion Enhanced add-on — https://ankiweb.net/shared/info/1374772155
4. Overlapping clozes technique — https://eshapard.github.io/anki/learning/code/python/overlapping-clozes.html
5. Roediger & Karpicke 2006, Test-Enhanced Learning — https://journals.sagepub.com/doi/10.1111/j.1467-9280.2006.01693.x
6. Miller 1956 — https://en.wikipedia.org/wiki/The_Magical_Number_Seven,_Plus_or_Minus_Two
7. Cowan 2001 — https://www.cambridge.org/core/journals/behavioral-and-brain-sciences/article/magical-number-4-in-shortterm-memory-a-reconsideration-of-mental-storage-capacity/44023F1147D4A1D44BDC0AD226838496
8. Sweller & Cooper 1985 / worked-example effect — https://en.wikipedia.org/wiki/Worked-example_effect
9. Anderson & Krathwohl 2001 (revised Bloom); Krathwohl 2002 overview — https://cmapspublic2.ihmc.us/rid=1Q2PTM7HL-26LTFBX-9YN8/Krathwohl%202002.pdf
10. Novak & Cañas 2008, Theory Underlying Concept Maps — https://cmap.ihmc.us/publications/researchpapers/theoryunderlyingconceptmaps.pdf
11. Knowledge Space Theory / ALEKS — https://en.wikipedia.org/wiki/Knowledge_space ; https://www.aleks.com/about_aleks/knowledge_space_theory
12. Picture superiority effect — https://en.wikipedia.org/wiki/Picture_superiority_effect ; Higdon et al. 2025 — https://journals.sagepub.com/doi/10.1177/17470218241235520
13. Mayer multimedia principles — https://www.hartford.edu/faculty-staff/faculty/fcld/_files/12%20Principles%20of%20Multimedia%20Learning.pdf
14. OpenStax Anatomy & Physiology front matter (textbook pedagogical features) — https://courses.lumenlearning.com/openstax-anatomyandphysiology/front-matter/preface-2/
