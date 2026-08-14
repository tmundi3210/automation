# KG_PIPELINE.md — book → fundamental units → knowledge graph → app

**What this document is.** A planning-only map of how a book becomes a *knowledge graph*, and how that
graph would feed the live Base44 study app. It connects three things that already exist separately: (a) your
other repo `tmundi3210/msi` (the likely "USMLE KG" builder), (b) the decomposition *method* already written
into this repo's `contenteng` knowledge bases, and (c) the app's already-present `Concept` and `ConceptEdge`
data slots that a graph would populate. Nothing here builds anything.

**Terms, defined once.**
- **Knowledge graph (KG)** — a picture of a subject as *nodes* (concepts/terms) joined by *edges* (labelled
  relationships: "is a prerequisite of", "is part of", "relates to").
- **Fundamental unit** — the smallest teachable piece. In this method the units climb a ladder: term → fact →
  relationship (edge) → composition → multi-step inference (§2).
- **Concept / ConceptEdge** — the two database tables the app *already has* for graph data. They are the
  landing zone the KG must fill (§1, §4).
- **Honesty tags** — [FACT] verified in a named file/source · [FACT-source: URL] · [ESTIMATE] inference +
  basis/confidence · [METHOD] how to obtain · [UNKNOWN] not verifiable here · [SIGNAL] weak/design-intent
  indicator.

Sources: KG_RECON.md, APP_RECON.md, EXTERNAL_RECON.md (read in full); SPECIALIST_RECON.md, SPECIALIST_LIST.md
(skimmed), all dated 2026-07-08.

---

## §1. THE KG PROJECT — your separate `tmundi3210/msi` repo

- [FACT — found by repo search; **contents unread, out of session scope**] You own a separate GitHub repo
  **`tmundi3210/msi`**, default branch **`claude/learning-os-system-leSN5`**, primary language **TypeScript**.
  It is the **likely "USMLE KG" project**. [ESTIMATE — medium confidence] "USMLE KG" is inferred from the repo
  name/branch only, *not* from reading it; the app itself targets **INBDE + TOEFL**, not USMLE, so the graph
  builder may need re-pointing at the INBDE blueprint (§3) before its output fits this app.
- **Why it matters:** this app has the *slots* for a knowledge graph but **none of the data**. [FACT —
  APP_RECON §5.5] The `Concept` and `ConceptEdge` tables exist; `ConceptEdge` is *read* exactly once (the
  pipeline's prerequisite check, `runPipeline/entry.ts:84-89`); **no code anywhere ever creates a
  `ConceptEdge` row**, and no page draws a graph. The `msi` project is the natural producer of that missing
  data.

**The two landing tables (exact schema, so `msi` knows what to emit).** [FACT — read from
`base44/entities/Concept.jsonc` + `ConceptEdge.jsonc`]

- **`Concept`** (a node): `exam_code` ("INBDE"|"TOEFL"), `name` (required), `bloom`
  (remember|understand|apply|analyze|evaluate|create), `inbde_fk` (integer 1–10, the Foundation-Knowledge
  area — this is where "FK6 Pathology" comes from), `inbde_cc` (integer 1–56, the fine Clinical-Content
  area — **defined but used nowhere yet**), `inbde_cc_section` (DTP|OHM|PP), `toefl_section` (R|L|S|W),
  `toefl_task_type` (free string).
- **`ConceptEdge`** (an edge): `src_concept_id`, `dst_concept_id`, `type` (**prereq** | related | part_of,
  default `prereq`). All three fields required.

**[METHOD] to bring `msi` in (a future in-scope session, not now).**
1. Add `tmundi3210/msi` to a session (or clone it locally) and read its README + graph/schema files.
2. **Confirm its input→output contract:** what it ingests (book text? a term list? a blueprint?) and what it
   emits (node list? edge list? in what format?).
3. **Diff its KG schema against §2's edge method** — does it produce `(term_a, relation, term_b)` triples, and
   are its relation labels reducible to the app's three edge types (prereq/related/part_of)?
4. **Map its outputs → the two tables above.** Each `msi` node → one `Concept` row (stamp `inbde_fk` /
   `inbde_cc_section` so mastery can roll up to the blueprint axes, §4); each `msi` relationship → one
   `ConceptEdge` row, with its prerequisite links typed `prereq` so the gating behaviour in §4 can fire.
5. Confirm whether `msi` is USMLE-only or re-targetable to INBDE; if USMLE-only, treat it as a *method*
   template, not a drop-in data source.

---

## §2. DECOMPOSITION METHOD — as actually encoded in `contenteng`

This is restated faithfully from the two `contenteng` knowledge bases; it is the repo's own answer to "how do
you turn a book into fundamental units and then into a graph." [FACT — KG_RECON §2, read from
`kb_book_decomposition` + `kb_knowledge_layering`]

**The pipeline** (`kb_book_decomposition`, 11 steps WF_01–WF_11, each emitting a typed record):
1. **TEXTBOOK_ANATOMY** — map the book's structure (parts/chapters/sections/glossary/index/end-of-chapter Qs).
2. **CHUNKING_LIMITS** — set a working-memory budget (~4–7 elements) so no unit overloads the learner.
3. **BOOK_TO_UNITS** — parse the table of contents into ordered *one-sitting units*
   `unit(id, title, objectives[], source_pages, order)`; **hold back end-of-chapter questions** as a
   validation set (never turned into cards).
4. **UNIT_TO_TERMS** — harvest bold terms + glossary + index into `term(id, unit_id, name, definition,
   image?)`. **Heavily-indexed terms become hub nodes** (the big, well-connected concepts).
5. **TERM_TO_CLAIMS** — break prose into atomic one-subject/one-predicate/one-value `claim`s, each
   FACT/ESTIMATE/UNKNOWN-tagged and source-cited.
6. **CLAIM_TO_RELATIONS** — every multi-term claim becomes an **edge** `edge(term_a, relation, term_b,
   unit_id)`. **This is the knowledge-graph layer** — the same triple that becomes a `ConceptEdge` row.
7. **PREREQ_MAPPING / GRAPH_VS_FLAT** — decide representation (see hybrid decision below).
8. **RELATIONS_TO_MULTISTEP** — chain **2–3 edges** into worked-solution-first case questions.
9–11. **CARD_COMPILATION → BAD_CARD_LINT → COVERAGE_HONESTY_GATE** — compile to Anki note types, lint bad
   cards, and gate: the deck must answer the book's own held-back end-of-chapter questions, and every card
   must trace to a sourced claim/edge.

**The terminology / knowledge ladder** (`kb_knowledge_layering`, the organizing spine). [FACT] The unit of
decomposition *climbs*, and each rung is typed by a Bloom-process × knowledge-kind pair:
1. **terminology** — term cards (Remember × Factual).
2. **basic facts** — single minimum-information facts / cloze (Remember/Understand × Factual).
3. **relate two things** = **the "edges"** (Understand × Conceptual).
4. **compositions** — "those two make a third thing," worked examples (Apply × Procedural).
5. **two/three-step inference** — multi-step vignettes (Analyze→Evaluate × Conceptual).

**Edges, defined.** [FACT] An edge is a triple `(A, relation, B)` — in concept-map terms a *proposition*
(Novak & Cañas 2008). Both endpoint term-cards must exist before an edge card is emitted; inference chains are
capped at 2–3 steps, worked-solution-first.

**Graph vs flat — the repo's deliberate choice.** [FACT] The KB recommends a **HYBRID** and records that the
owner was "unsure about graphs": keep **flat, ordered units as the scheduling spine**, but **extract a
lightweight edge table `(term_a, relation, term_b, unit_id)` anyway**. Full Knowledge-Space-Theory adaptive
selection (Doignon & Falmagne 1985) is **deferred** until adaptive item-selection actually needs it (dominance
rule DR_08: "flat-spine + edge-table hybrid dominates a premature full-graph commitment"). **Consequence for
this plan:** the `ConceptEdge` table *is* that extracted edge table; `msi` populating it does **not** require
committing to a full graph engine — it is the reversible hybrid the method already endorses.

**Scope caveat.** [FACT — KG_RECON §2a] `kb_book_decomposition` is **generic** and explicitly *excludes*
INBDE-specific blueprint mapping ("which INBDE competency a card serves"). The blueprint mapping lives in the
`dental` specialist / research file, not in the decomposition method — so `msi` must get the FK/CC stamping
from the blueprint side (§3), not from the generic decomposition method.

---

## §3. WHICH BOOKS TO RUN — and how to break each down

**Honest starting point: the repo names essentially no content textbooks.** [FACT — KG_RECON §4]
- The **only book named anywhere** in the decomposition assets is **OpenStax *Anatomy & Physiology***, and it
  is cited **purely as a structural exemplar** (per-section objectives, glossary, end-of-chapter questions) —
  **not** as an INBDE content reference. [FACT-source: research/flashcards_and_book_decomposition.md ref #14]
- The dental/INBDE research file names only **official JCNDE/ADA documents** (INBDE Candidate Guide, Item
  Development Guide + Appendices A–E, Model Domain of Dentistry, Technical Report) and **non-endorsed prep
  aggregators** (Penn Libraries INBDE guide, dentalcare.com, INBDE Bootcamp, Kaplan, UOP Dugoni). None are
  subject-matter textbooks to decompose. [FACT — KG_RECON §4]
- **[UNKNOWN]** There is **no canonical INBDE reference-book list in this repo**, and JCNDE deliberately does
  **not** publish a "recommended textbook" list.

**[METHOD] to obtain a defensible reading list (fetch-enabled session).** Pull the **official INBDE Item
Development Guide** and the **"Prepare for the INBDE" page** (jcnde.ada.org/inbde) — these describe the
**10-FK × 56-CC blueprint** rather than name books; then map each FK/CC area to a de-facto standard text by
cross-referencing the **Penn Libraries INBDE guide** and school prep lists. **Tag every resulting title
[ESTIMATE]** — they are library/aggregator picks, not a JCNDE-endorsed list. The `dental` specialist owns this
blueprint-as-taxonomy and its "resource shelf is existence-only, no quality claim" rule. [FACT — SPECIALIST_RECON §3]

**Ordering principle (blueprint coverage first).** [FACT — the blueprint is a 10-FK × 56-CC matrix; every item
integrates ≥1 FK applied within a CC area, KG_RECON §3] Run books in the order that fills the graph's
**foundation** before its **clinical application**, because the ladder (§2) and the prereq edges (§4) require
lower concepts to exist before higher ones can attach:

| Order | Blueprint band | What to run | Unit granularity to decompose at |
|---|---|---|---|
| 1 | **Foundation Knowledge (FK1–FK10)** — biomedical basis | The basic-science texts behind FK1–FK10 (e.g. a general **anatomy & physiology** text — OpenStax A&P is the confirmed *structural* exemplar [FACT]; specific FK titles are [UNKNOWN]+[METHOD] above). Prioritise **FK6 Pathology, FK7 Microbiology, FK5 Immunology, FK8 Pharmacology** — the mechanism-heavy areas the graph's hub nodes live in. | **Fine (term + edge level).** Basic-science books are glossary- and mechanism-dense → harvest terms and `(A relation B)` edges heavily (steps WF_04–WF_06); this is where prerequisite chains are richest. |
| 2 | **Clinical Content — Diagnosis & Tx Planning (DTP, ~13 CC areas)** | Diagnosis / oral-radiology / treatment-planning references. | **Medium (claim + composition level).** Decompose to atomic claims and 2–3-edge case compositions (WF_05, WF_08) that *depend on* the FK terms already in the graph. |
| 3 | **Clinical Content — Oral Health Management (OHM, ~23 CC areas, largest item share)** | Operative, perio, endo, prosth, oral-surgery, pharmacotherapeutics references. | **Coarser at term level, heavy at multi-step.** These are application areas → fewer new terms, many **multi-step vignettes** (WF_08) chaining FK mechanisms to clinical decisions. |
| 4 | **Clinical Content — Practice & Profession (PP, ~20 CC areas)** | Ethics, jurisprudence, behavioral science, practice management, informatics references. | **Fact + case level.** Mostly Factual/Conceptual rungs (1–2); fewer deep prereq chains. |

- [ESTIMATE — per-group CC counts and item shares carry the recon's own [ESTIMATE] tags; totals: **56 CC areas
  [FACT]**, grouped DTP/OHM/PP; the verbatim 56-area enumeration is **[UNKNOWN]** pending the Item Development
  Guide PDF, KG_RECON §3.]
- **How "how to break it down" is decided per book:** a *glossary-and-mechanism* book → decompose to the
  **term/edge** rungs (many hub nodes, many prereq edges); an *application/case* book → decompose to the
  **composition/multi-step** rungs (few new nodes, many 2–3-edge chains). This mirrors the ladder in §2:
  foundation books build the graph's **nodes and prerequisite skeleton**; clinical books build its **higher
  edges and case chains**.

---

## §4. KG → APP — how nodes and edges actually drive behaviour

The graph is not decoration; four app behaviours are *waiting* for it. [FACT — APP_RECON §§3–4]

1. **Weakness diagnosis (prerequisite cause).** [FACT — APP_RECON §4a, `runPipeline/entry.ts:84-89, 92-106`]
   When you miss cards on a concept, the nightly pipeline walks that concept's **`ConceptEdge type:'prereq'`**
   links; if any prerequisite concept has `bkt_p_mastery < 0.5`, it diagnoses **`missing_prereq`** (confidence
   0.7) and prescribes: **"Schedule the prerequisite concept first; block new cards on this concept until
   prereq mastery ≥ 0.6."** **This is the "block new cards until prerequisite mastered" behaviour — and today
   it is *text only*; no code enforces the block** (grep-verified). Prerequisite edges are literally the fuel
   for this rule; with `ConceptEdge` empty, the rule can never fire.
2. **Planner / next-item gating (currently open).** [FACT — APP_RECON §3d] The review queue is **due-order
   only**; nothing reads mastery or edges to *gate* what is served. Closing the loop means: before releasing a
   new card, check its concept's prereq edges and *withhold* it until upstream mastery clears the threshold —
   turning the prose prescription above into an actual scheduler gate. (Owned by **learner** for the selection
   rule + **sysloops** for where it runs in the nightly loop; the enforcement seam is declared but unbuilt.)
3. **Mastery roll-up to blueprint axes (already works, needs stamped nodes).** [FACT — APP_RECON §4c] Pipeline
   Stage 5 averages `bkt_p_mastery` per **FK area** (`fk|N`) and **CC section** (`cc|DTP/OHM/PP`) into
   `MasteryHistory`, feeding the MasteryBars and trend chart. This only rolls up correctly if each `Concept`
   node carries its `inbde_fk` / `inbde_cc_section` — i.e. `msi` must stamp them (§1 step 4).
4. **The future KG view (greenfield).** [FACT — APP_RECON §5.5] No page renders a graph and **no graph library
   is installed** (no cytoscape/react-flow/d3-force/sigma). The visual graph (§5) is a from-scratch build on
   top of populated `Concept`/`ConceptEdge` data.

**What `msi` must emit to power all four:** (a) one `Concept` per node **stamped with `inbde_fk` and
`inbde_cc_section`** (and `inbde_cc` 1–56 if it wants the fine axis to finally have data); (b) one
`ConceptEdge` per relationship with **prerequisite links typed `prereq`**; (c) enough edge coverage that a
concept's true upstreams are present, or the `missing_prereq` rule silently under-fires. [ESTIMATE — high
confidence; derived from the four consumers above.]

**Debt to respect** [FACT — APP_RECON risk 8, §3d]: weakness rows accumulate duplicates across days,
`missing_prereq` blocking is unenforced prose, and `probe_card_ids` are stored but never used. Any "close the
KG→gating loop" work must define dedup + real enforcement, not just fill the tables.

---

## §5. KG VISUALIZATION SPEC — the owner's dictated vision (planning only)

Every element below is **[SIGNAL — owner's design intent]**, cleaned into a build brief. None is a committed
design; the final choices must be *grounded* by proposed specialists (below) before build.

- **Semantic zoom.** [SIGNAL] Zoomed out → few nodes with concise names (only hubs / high-level concepts);
  as you zoom in, more nodes and finer terms appear. The graph should abstract *honestly* at low zoom, not
  just shrink — fewer, bigger, well-labelled nodes far out; detail on demand.
- **Color = taxonomy role.** [SIGNAL] Distinct hues for distinct roles — e.g. **pathology vs mechanism vs
  treatment as separate colors**; concepts *related along an edge* take **related shades** of a hue so a
  chain reads as a color family.
- **Brightness / intensity = high-yield importance.** [SIGNAL] Brighter / more saturated = higher-yield
  (more exam-important) concept; low-yield nodes recede.
- **Corner indicator dot = knowledge state.** [SIGNAL] A small dot on each node: **green = known**, **gray =
  not yet asked / unseen**, **red (with intensity) = weak/missed**. (Maps naturally to the app's existing
  per-concept `bkt_p_mastery` and weakness state.)
- **Hover = thinking-cloud definition.** [SIGNAL] Hovering a node pops a small "thought-cloud" with the
  concept's definition; it **dismisses on mouse-out**. (The definition text is exactly what the terminology
  harvest in §2 step WF_04 already captures.)
- **Click-through to the full catalog entry.** [SIGNAL] Clicking a node opens its full library/catalog entry,
  with **back-navigation** *or* **open-in-a-new-column** so you don't lose the graph.
- **Nested terminology popups.** [SIGNAL] A definition popup may itself contain terms whose own popups nest —
  drilling terminology within terminology.
- **Shape & font = secondary channels.** [SIGNAL] Node shape and font are *secondary* encodings (e.g. node
  type or ladder rung), used only after color/brightness/dot carry the primary meaning.

**Who must ground these choices** (proposed specialists, **names only — none built**; per SPECIALIST_LIST.md §C):
- **`viz`** — the visual-encoding & graph-view cartographer: owns semantic zoom (cartographic
  level-of-detail), the **color system** (perceptual + colorblind-safe hues for taxonomy role and high-yield
  intensity), and graph layout. **Must ground the semantic-zoom + color-semantics decisions** so "pathology
  vs mechanism vs treatment" hues are perceptually distinct and accessible, not arbitrary. [FACT — gap: engage
  explicitly routes out exact styling; no specialist owns graph rendering, SPECIALIST_RECON gap 1,2,12]
- **`kgraph`** — the knowledge-graph architect: owns the node/edge schema, hub/prereq structure, and
  terminology linking (recognizing a term, attaching the hover definition). **Must ground the
  data-and-meaning layer** the colors and hovers render. [FACT — contenteng stops at the edge table; nobody
  owns terminology linking, gap 1]

**[FACT-source] precedents from EXTERNAL_RECON to anchor the design (proven interaction vocabulary):**
- **Obsidian graph view** [FACT-source: help.obsidian.md/plugins/graph] — force-directed graph where **node
  size scales with inbound references** (a ready model for "hub = bigger"), **groups color nodes by query**
  (precedent for color-by-taxonomy-role), and **hover highlights a node's connections, click opens it, local
  graph per note** (precedents for the hover and click-through behaviours above).
- **RemNote** [FACT-source: remnote.com + help.remnote.com] — flashcards live *inside* hierarchical, bidirectionally-linked
  notes, giving each card **knowledge-graph context**; the closest analog to a KG-driven study app and evidence
  that "graph + SRS in one object model" is buildable.
- **Anki / FSRS** [FACT-source: docs.ankiweb.net/deck-options.html] — precedent for **exposing one meaningful
  dial, not raw internals**: applied here, let the graph *show* high-yield intensity and knowledge state
  rather than expose raw BKT/Elo numbers.
- **UWorld** [FACT-source: medical.uworld.com/usmle/features] — weakness reporting as a **hierarchical
  subject/system/topic breakdown**; supports rolling the graph up to the FK/CC axes (§4.3), though its
  **peer-percentile** signal does **not** apply to this single-user app. [SIGNAL — adapt, don't copy the peer part]

---

*Planning document only. Nothing here creates, modifies, or authorizes building any specialist, app code,
`*.specialist.json`, or `*.kb.json`. The `tmundi3210/msi` repo remains out of scope until a future session
brings it in per §1's [METHOD].*
