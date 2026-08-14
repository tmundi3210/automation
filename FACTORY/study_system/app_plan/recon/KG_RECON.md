# KG_RECON — Book→Units→Knowledge-Graph pipeline assets + INBDE raw material

Repo scanned: `/home/user/automation` (this repo). Planning-only recon; no app source,
`*.specialist.json`, or `*.kb.json` created or modified.

Honesty tags: [FACT] verified in a file/source · [ESTIMATE] inference + basis/confidence ·
[UNKNOWN] not verifiable here + [METHOD] to obtain · [SIGNAL] weak indicator.

---

## 1. The msi repo (out-of-scope record)

- [FACT] The owner has a **separate GitHub repo `tmundi3210/msi`**, default branch
  `claude/learning-os-system-leSN5`, primary language **TypeScript**. It is **likely the
  "USMLE KG" project**. Established prior to this session.
- **CAVEAT (binding this session):** `tmundi3210/msi` is **OUT OF SCOPE to read** here.
  Nothing in this recon is derived from that repo's contents; only its existence and the
  above metadata are recorded. [ESTIMATE — "likely the USMLE KG project": medium
  confidence, inferred from repo name/branch, NOT from reading it.]
- [METHOD] To confirm the USMLE-KG hypothesis in a future in-scope session: read
  `tmundi3210/msi` on branch `claude/learning-os-system-leSN5` (README + graph/schema
  files) and diff its KG schema against the decomposition method in §2 below.

---

## 2. Decomposition method as encoded in `contenteng` (this repo)

Two KBs encode the owner's book→fundamental-units→knowledge-graph pipeline.

### 2a. `kb_book_decomposition.kb.json` (+ `.spec.json`) — the full pipeline
Path: `/home/user/automation/FACTORY/study_system/specialists/contenteng/kb_book_decomposition.kb.json`
(5740-line generated KB) and its authored source `kb_book_decomposition.spec.json` (517 lines).

- [FACT] **Domain**: `content_engineering_book_decomposition` — "book -> units -> terms ->
  claims -> relations -> multi-step questions -> cards". `empirical_status:
  heuristic_prior_not_observed_dataset` (all scores are heuristic priors, no observed data).

- [FACT] **The pipeline (11 workflow steps, WF_01–WF_11), each stage emits a typed record:**
  1. `TEXTBOOK_ANATOMY` — map book structure to downstream artifacts.
  2. `CHUNKING_LIMITS` — set the working-memory size budget.
  3. `BOOK_TO_UNITS` — parse TOC/chapter/section into ordered one-sitting units;
     record `unit(id, title, objectives[], source_pages, order)`; **hold back
     end-of-chapter questions as the validation set**.
  4. `UNIT_TO_TERMS` — harvest bold terms + back-of-book glossary + index into
     `term(id, unit_id, name, definition, image?)`; heavily-indexed terms = **hub nodes**.
  5. `TERM_TO_CLAIMS` — break text into atomic one-subject/one-predicate/one-value
     `claim(id, text, terms, source, date?)`, FACT/ESTIMATE/UNKNOWN-tagged.
  6. `CLAIM_TO_RELATIONS` — every multi-term claim becomes an **edge**
     `edge(term_a, relation, term_b, unit_id)` — this IS the owner's "edges" / the
     knowledge-graph layer.
  7. `PREREQ_MAPPING` / `GRAPH_VS_FLAT` — representation decision.
  8. `RELATIONS_TO_MULTISTEP` — chain **2–3 edges** into worked-solution-first, then
     problem-only case items.
  9. `CARD_COMPILATION` — compile each layer to its Anki note type.
  10. `BAD_CARD_LINT` — deck-wide lint of bad-card patterns.
  11. `COVERAGE_HONESTY_GATE` — deck must answer the book's own end-of-chapter questions
      (coverage oracle) + every card traces to a sourced claim/edge (honesty).

- [FACT] **"Fundamental units" = the terminology ladder.** The unit of decomposition
  climbs: terms → atomic claims → edges (relations) → 2–3-step composed questions.

- [FACT] **"Edges" defined**: a triple `(A, relation, B)` = concept-map **proposition**
  (Novak & Canas 2008). Rendered as a card family: forward, reverse-if-unique (Rule 11),
  relation card, cloze. `edges ~= propositions` is flagged [ESTIMATE] (owner's own term).

- [FACT] **Graph vs flat decision (owner "unsure about graphs")**: KB recommends a
  **HYBRID** — flat ordered units are the scheduling/calendar **spine**, plus a
  lightweight **edge table** `(term_a, relation, term_b, unit_id)` extracted anyway.
  Full Knowledge Space Theory (Doignon & Falmagne 1985 surmise relation + fringe, as in
  ALEKS) is **deferred** until adaptive selection demands it. Dominance rule DR_08:
  "flat-spine + edge-table hybrid dominates a premature full-graph commitment."

- [FACT] **Theory anchors** (each snippet-confirmed, direct fetch was HTTP-403 this
  session): Bloom revised (Anderson & Krathwohl 2001) process×knowledge grid; chunking
  (Miller 1956 ~7±2, Cowan 2001 ~4); Wozniak Twenty Rules of Formulating Knowledge
  (minimum-information Rule 4, cloze Rule 5, image occlusion Rule 8, interference Rule 11,
  context cues Rule 16, sources/dating Rules 18–19); worked examples (Sweller & Cooper
  1985) + expertise-reversal; testing effect (Roediger & Karpicke 2006).

- [FACT] **INBDE is explicitly EXCLUDED from this KB's scope**: exclusions list
  "domain-specific ... exam-blueprint mapping of a particular certification (e.g. which
  INBDE competencies a card serves) beyond the generic climb to Apply/Analyze." So the
  method is generic; the INBDE blueprint mapping lives in the research file (§3), not here.

### 2b. `kb_knowledge_layering.kb.json` (+ `.spec.json`) — the learning ladder
Path: `/home/user/automation/FACTORY/study_system/specialists/contenteng/kb_knowledge_layering.kb.json`

- [FACT] **Domain**: `contenteng_knowledge_layering` — "Knowledge Layering Ladder (books
  to cards, owner ladder mapped to theory)".
- [FACT] **The owner's five-rung ladder (verbatim design intent, the organizing spine):**
  1. **terminology** → Remember × Factual (term cards)
  2. **basic facts** → Remember/Understand × Factual (minimum-info single facts, cloze)
  3. **relate two things** → Understand × Conceptual = **the "edges"**
  4. **compositions** ("those two make something else") → Apply × Procedural (worked examples)
  5. **two/three-step inference** → Analyze(→Evaluate) × Conceptual (multi-step vignettes)
- [FACT] Node IDs include `LADDER_SPINE`, `BLOOM_PROCESS_MAP`, `CHUNKING_BUDGET`,
  `LADDER_ORDER_INVARIANT`. The ladder is "the spine: scheduling, progress UI and card
  emission all key off the rung a unit of knowledge sits at"; each card is fully typed by
  a (Bloom-process × knowledge) pair. The theory mapping is a design [ESTIMATE]
  (medium-high); each cited anchor is [FACT].

---

## 3. INBDE official exam blueprint (explains app terms like "FK6")

Source file: `/home/user/automation/FACTORY/study_system/research/exam_dental_us_fmg.md`
(390 lines). Title: "US Dental Licensure for Foreign-Trained Graduates — Exam Ground Truth".

- [FACT-source: jcnde.ada.org/inbde] **INBDE = Integrated National Board Dental
  Examination**, single computer-based national board exam, launched Aug 2020, replaced
  NBDE Part I & II. Owner: **JCNDE** under the **ADA**. Knowledge exam only (clinical
  ADEX/DLOSCE is separate).

- [FACT] **Blueprint = a MATRIX: 10 Foundation Knowledge (FK) areas × 56 Clinical
  Content (CC) areas.** Every item integrates ≥1 FK area applied within a CC area (the
  "Integrated" in INBDE). This is the source of app terms like **FK6**.

- [FACT / ESTIMATE-high wording] **The 10 FK areas** (enumeration corroborated across
  secondary sources quoting the JCNDE Domain of Dentistry model; exact wording verify
  vs Item Development Guide PDF):
  - FK1 Molecular, biochemical, cellular, systems-level development, structure, function
  - FK2 Physics & chemistry to explain normal biology and pathobiology
  - FK3 Physics & chemistry — characteristics/use of technologies and materials
  - FK4 Genetic, congenital, developmental diseases → patient risk
  - FK5 Cellular/molecular bases of immune & non-immune host defense
  - **FK6 General and disease-specific pathology to assess patient risk**  ← e.g. app "FK6"
  - FK7 Biology of microorganisms in physiology and pathology
  - FK8 Pharmacology
  - FK9 Behavioral sciences, ethics, and jurisprudence
  - FK10 Research methodology and analysis, and informatics tools

- [FACT total / ESTIMATE per-group] **The 56 CC areas** are grouped into **3 component
  sections** (counts and item-share differ because they measure different things):
  | Group | # CC areas | Share of scored items |
  |---|---|---|
  | A. Diagnosis and Treatment Planning | 13 [ESTIMATE] | ~36.2% [ESTIMATE] |
  | B. Oral Health Management | 23 [ESTIMATE] | ~42.0% [ESTIMATE] |
  | C. Practice and Profession | 20 [ESTIMATE] | ~21.8% [ESTIMATE] |
  | **Total** | **56 [FACT]** | **100%** |

- [UNKNOWN] **The full verbatim enumeration of all 56 CC areas with official numbering**
  is NOT in the repo — JCNDE PDFs present them as Figures 1–4 (images) and every direct
  fetch returned HTTP-403. This is flagged as the repo's #1 follow-up. [UNKNOWN] official
  per-FK / per-CC item-weight percentages.
- [METHOD] Extract the 56 CC areas + FK weights from the **INBDE Item Development Guide**
  (+ Appendices A–E) and **Model Domain of Dentistry** PDFs in a fetch-enabled run:
  - jcnde.ada.org/-/media/.../inbde_item_development_guide.pdf
  - jcnde.ada.org/-/media/.../inbde_domain_dentistry_model.pdf
  - mirror: dental.ufl.edu/wordpress/files/2025/03/INBDE_Item_Development_Guide.pdf

---

## 4. Sourced reference-book list — [UNKNOWN] + [METHOD]

- [FACT] **No content reference-textbook titles are named** in the dental/INBDE research
  file. The file names only: (a) **official JCNDE/ADA documents** — INBDE Candidate Guide,
  Item Development Guide (+ Appendices A–E), Model Domain of Dentistry, Domain of Dentistry
  (July 2018), INBDE Technical Report, "INBDE facts" one-pagers; (b) **prep/aggregator
  resources for existence only, no quality claim** — Penn Libraries INBDE Prep Guide,
  dentalcare.com (Dentsply Sirona), INBDE Bootcamp, Kaplan INBDE, UOP Dugoni FK PDF.
  None of these are subject-matter textbooks to decompose.
- [FACT] The ONLY book named anywhere in the decomposition assets is **OpenStax Anatomy &
  Physiology**, cited purely as a **structural exemplar** of textbook anatomy (per-section
  learning objectives, glossary, end-of-chapter questions) — NOT as an INBDE content
  reference. Source: research/flashcards_and_book_decomposition.md ref #14 (OpenStax A&P
  front matter, courses.lumenlearning.com).
- [UNKNOWN] **No canonical INBDE reference-book list exists in this repo.**
- [METHOD] To obtain an authoritative reading list: JCNDE deliberately does not publish a
  "recommended textbook" list; pull the **official JCNDE INBDE guides** (Item Development
  Guide + "Prepare for the INBDE" page, jcnde.ada.org/inbde/inbde-prepare) which describe
  the blueprint rather than name books. For de-facto references, cross-reference the
  Penn Libraries INBDE guide (guides.library.upenn.edu/INBDE) and school prep lists — but
  tag any resulting titles [ESTIMATE], since they are library/aggregator picks, not a
  JCNDE-endorsed list.

---

## 5. Other in-repo KG / graph / USMLE assets

- [FACT] **Glob `*kg*`** under `/home/user/automation`: **none.**
- [FACT] **Glob `*usmle*`**: **none** — the USMLE KG lives only in the out-of-scope
  `tmundi3210/msi` repo (§1).
- [FACT] **Glob `*graph*`** returned 3 files, NONE a knowledge-graph asset:
  - `phase_b/scaffold/orch/graph.py` — [FACT] a generic weighted-DAG **model-orchestration
    engine** ("any-to-any weighted DAG orchestration with summarizer + token-budget cap"),
    unrelated to a study/knowledge graph. False positive.
  - `FACTORY/movie_studio/research/cinematography_lighting_camera.md` — cinematography
    ("camera/graph" substring). False positive.
  - `FACTORY/movie_studio/productions/ad01_gold_kada/parts/30_cinematography.md` — same.
    False positive.
- [FACT] The only **knowledge-graph layer that exists in this repo** is the conceptual
  **edge table** `(term_a, relation, term_b, unit_id)` specified inside the two contenteng
  KBs (§2) — a design/method, not built graph data.

---

## 6. Bottom line

- The book→fundamental-units→knowledge-graph **method** lives in this repo, fully
  specified, in `FACTORY/study_system/specialists/contenteng/` (book_decomposition +
  knowledge_layering KBs). It is generic (tool = Anki), and it explicitly defers full
  graph/KST in favor of a flat-units spine + extracted edge table.
- The **built** USMLE knowledge graph is in the separate `tmundi3210/msi` repo, which is
  out of scope this session (§1).
- The INBDE **raw material** for a dental KG is the `exam_dental_us_fmg.md` research file:
  the 10 FK × 56 CC blueprint (explains "FK6" etc.) is captured, but the verbatim 56-CC
  enumeration and item weights are [UNKNOWN] pending a fetch-enabled pull of the JCNDE
  Item Development Guide PDF.
- **No INBDE reference-book list exists** in-repo; only official JCNDE blueprint documents
  and non-endorsed prep aggregators are named.
