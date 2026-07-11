# NEW_FEATURES_PLAN — owner dictation 2026-07-11 (planning-mode capture)

_Authored in planning mode (Fable 5). **No specialists authored yet; no build triggered.** This file captures the owner's live-graph findings and three new feature ideas, routes each to a design authority, names the gaps, and specifies the specialists to author next. The owner will then pick a model for authoring; Fable re-runs the ideas through the completed specialist set to produce the actual build plan._

---

## 0. Live-graph findings (owner walkthrough, 2026-07-11) — [SIGNAL]: source-data quality, NOT a render bug

The owner walked the live knowledge graph after the import and found relationships that are wrong at the source, faithfully rendered:

- Search **"heart"** → links only to *atria* and *aortic arch*. Thin linking [SIGNAL].
- **"malignancy"** — definition is correct (neoplasm / atypical pleomorphic cells / metastasis); but its relations point to *endocarditis* and *PTH-related peptide* — non-sequiturs [SIGNAL].
- **"endocarditis"** — no source definition; relations *has-finding murmur* / *has-finding Roth spot* are **correct** [observed-good].
- **"pituitary"** → *part-of adrenal cortex*; **"adrenal cortex"** → *part-of medulla* AND *part-of pituitary*. This mereology is **inverted / impossible** [SIGNAL: source KG defect].

**Conclusion.** These are defects in the **usmle-kg source data**, not in Base44's rendering — the graph is honestly displaying bad propositions. This validates the owner's instinct that an **LLM recheck/repair pass** is warranted (→ Feature A). The render layer is doing its job; the epistemics layer does not exist yet.

---

## 1. Negated-edge blocker (surfaced by q1b verify) + DECISION

**Finding.** The export = 162 edges, **all `negated: false`** — zero negations across the entire graph (checked every row).

**Consequences.**
- q1b's spot-check "one negated edge exists" cannot be met on real data.
- q2 (Pass 2) negation-rendering acceptance cannot be **live**-verified.

**Two readings, both a usmle-kg finding:** (i) the source genuinely contains no negations; (ii) the generation pipeline is dropping/never-emitting negations. Worth a background investigation on the Mac side either way.

**DECISION (recommended, reversible).** Build the negation-rendering path and **accept it against a clearly-labeled synthetic fixture** — one hand-authored negated edge tagged test-only, **never injected into the 190-concept real set as if real**. The code ships and lights up automatically the moment a real negated edge appears. This is non-blocking and keeps the invariant "negated edges never render positive" testable without faking data. → recorded as `decisions_pending.negated_edge_fixture`.

---

## 2. q7 linkTerms threshold — DECISION: keep 0.7

Post-import report: 195 concepts, 9 cards → 11 links, 10 exact (conf 1.0) + 1 fuzzy (`infections → Infection` @ 0.72), 0 abstentions. One fuzzy link is too little signal to justify moving the floor. **Keep 0.7**; revisit when the corpus is large enough to produce abstentions to stress-test against.

---

## 3. Three new features → specialist routing + gap analysis

### Feature A — LLM knowledge-graph relation validation / repair ("node rechecking")
- **What.** An AI pass that reads each `(A, relation, B)` proposition, judges clinical truth, **flags/repairs contradictions** (the pituitary↔adrenal inversion), scores confidence, and can re-link thin nodes (heart). Proposes corrections; never silently rewrites; owner-gated; machine-corrected edges carry an honesty tag and provenance.
- **Owner today.** `kgraph` owns the SCHEMA; **nobody owns proposition TRUTH at scale.** GAP.
- **Recommendation.** **NEW specialist** (working code `kgverify`): clinical-proposition verification + contradiction/inversion detection + repair-proposal discipline. Distinct skill from graph architecture, and it is a recurring runtime feature, not a one-off cleanup. *Alternative:* extend `kgraph` (lighter, but muddies its remit and mixes "is the structure well-formed" with "is the claim true").

### Feature B — category visual encoding (shape / emoji / color per discipline)
- **What.** cardiology = heart shape, pathology = one hue, anatomy / physiology / disease distinguished by shape + hue + icon.
- **Owner.** `viz` (Bertin visual variables) — **EXISTING; no new specialist.**
- **Real blocker is DATA, not rendering.** Nodes need a `discipline` / `category` attribute that does not exist in the KG yet (dependency on `kgraph` + `contenteng` to categorize). Rendering is trivial once nodes are categorized.
- **Tension to respect.** viz's channel budget: luminance is already committed to importance (Decision D1). Shape = discipline + hue = category are *different* channels, so they compose — but must stay inside viz's meaning-first doctrine and earn a legend line. Fold as a viz Pass-3-adjacent item **once nodes carry a category**.

### Feature C — curriculum / study-catalog browser (the "Review" selector)
- **What.** Choose what to learn by **system / subject / topic**; a collapsible markdown-style tree (collapsed → expand, or click a heading to start it directly); per-item ordering (do this 1st, this 2nd); filters (pathology / physiology / everything, or subject-wise variants); **pre-made catalogs**; **AI-generated catalogs**; shareable so a different learner can use it for a different subject.
- **Research required first.** How **INBDE**, **USMLE Step 1 & 2**, and **TOEFL** structure their content outlines (official blueprints).
- **Composes existing specialists.** `contenteng` (hierarchy from content), `dental` (INBDE blueprint), `toefl` (TOEFL blueprint), `sched` + `learner` (ordering / prereqs — prereq gating already shipped in S2), `uxguide` + `engage` (tree UX + progress rollup).
- **GAPS.**
  1. **USMLE Step 1/2 blueprint has no specialist** — usmle-kg is a data source, not an exam authority.
  2. **Pre-made + AI-generated + shareable catalogs** have no owner.
  3. **Sharing** ("someone else who wants to learn something else") collides with the **single-user architecture** baked into nearly every specialist — a real architectural decision: stay single-user with *importable catalog files*, vs go multi-user. → `decisions_pending.catalog_multiuser`.
- **Recommendation.** **ONE new specialist** (working code `curric`): owns the system→subject→topic hierarchy schema over the KG, exam blueprints as grounded data, catalog generation + the pre-made/shared-catalog model, and hands sequencing to `sched`. Plus the owner decision on single-user-importable vs multi-user.

---

## 4. Two META specialists (owner-requested)

### `planner` — factory planner / orchestrator
- **Owns.** Turning an idea into slices/passes; sequencing which specialists run and when; the verification loops (ledger → independent audit → runtime self-check); gate design; continuous-mode discipline. Formalizes the orchestration role Claude/Fable has performed by hand.
- **Not a duplicate of `sysloops`.** sysloops = the *app* improving *itself* at runtime (nightly analysis). planner = the *build/plan* orchestration. Different subject, different lifecycle.

### `handoff` — Base44 delivery / handoff authority
- **Owns.** The discuss-mode → edit-mode handoff; credit discipline; "RUN THE PLAN" continuous mode; gate-stopping; sync discipline; "unsynced = unfinished." This doctrine currently lives only in `PROTOCOL.md` — promoting it to a design authority makes it loadable and enforceable per slice.
- **Not a duplicate of `extint`.** extint = API/secret/bridge plumbing. handoff = the delivery protocol between the plan and the builder.

---

## 5. Pending specialists — never yet a design authority on a working slice [ESTIMATE from slice_status]

| Specialist | Status | Note |
|---|---|---|
| kgraph | USED | Slice 6 KG |
| viz | USED | Slice 6.5 graph beauty |
| uxguide | USED | Slice 6.5 |
| extint | USED | Slice 5 bridge, Slice 7 transport, Meshy |
| learner | USED | BKT (Slice T) + S2 adaptivity |
| sched | USED | S2 (thresholds provisional) |
| media | PARTIAL | Slice 7 phantom; rebuild pending (qS7r) |
| voice | DESIGNED, UNTESTED | Slice 7 capture path; no live test |
| **contenteng** | **PENDING** | content decomposition — needed by Feature C |
| **qcraft** | **PENDING** | question craft |
| **dental** | **PENDING** | INBDE blueprint — needed by Feature C |
| **toefl** | **PENDING** | TOEFL blueprint — needed by Feature C |
| **engage** | **PENDING** | gamification/progress — **this is why the graph reads "all zeros"** |
| **sysloops** | **PENDING** | self-improvement loops (Slice 8) |

**Note for the owner.** The "all zeros" you see is two honest things at once: (a) no study history yet (mastery unasked), and (b) **`engage` — the progress/gamification layer — has never been built.** Feature C naturally activates several pending specialists (contenteng, dental, toefl, engage, uxguide), so it is also how the app stops looking empty.

---

## 6. Model & process recommendation for authoring the four new specialists

The existing 14 are dense, KB-grounded, no-fabrication documents. New ones must match that bar (grounded `role`, explicit `boundaries`, `decision_procedure`, `escalation_triggers`, `validation_checklist`; honesty tags; zero invented facts/endpoints).

**Recommended authoring config.**
- **Agent type:** `general-purpose` — it needs full read access to the existing 14 as templates plus the recon KBs.
- **Model:** two defensible choices — **Fable 5** (top capability) or **Opus 4.8** (an independent second voice, so the later "run through Fable" step becomes a genuine two-model cross-check). **Recommendation: author with Opus 4.8, then run through Fable 5** — matches the house method of author-with-X / audit-with-Y and makes the second pass a real check rather than the same model re-reading itself.
- **Workflow (small, per specialist):** draft (grounded in an existing specialist template + the relevant recon/KB files) → verify-grounding (no invented facts; honesty tags present; boundaries explicit) → consistency-check (its boundaries don't collide with the existing 14). Same three-layer discipline used everywhere else.

**The process the owner described, made concrete:**
1. **(now, Fable 5 — this file):** plan captured; gaps named; specialists specified.
2. **(owner):** pick the authoring model (recommended: Opus 4.8).
3. **(me, that model):** generate the four specialists — `kgverify`, `curric`, `planner`, `handoff` — via the authoring workflow.
4. **(owner, back on Fable 5):** run Features A/B/C through the full **18-specialist** set to produce the concrete build plan (slices/passes).
5. **(delivery):** discuss-mode to Base44, then "RUN THE PLAN."

---

## 7. What does NOT block on any of this

The mainline Slice 6.5 passes 2–4 — semantic zoom, hover thinking-clouds, click-through, **zoom-anchored-at-cursor + fit-to-view navigation** (the fix for "looks basic / bad navigation"), semantic LOD, measured density — proceed via **"RUN THE PLAN"** independently. These new features are additive scope on top, not a precondition. Feature B (category encoding) is the one that *touches* the same viz surface, so it should land after nodes carry a `category` attribute rather than racing Pass 3's D1 encoding.
