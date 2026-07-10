# KG_EXCHANGE_CONTRACT.md — the folder handshake between usmle-kg and the Base44 app

_This is the agreed contract for how the knowledge-graph project (`usmle-kg`, on the owner's Mac at `/Users/mundi/usmle-kg`) hands its output to the Base44 study app. usmle-kg writes ONE export folder in this exact shape; the app's Slice 6 ingest reads it. Grounded in the `kgraph` specialist (graph data + meaning is its authority) and the Codex inspection report of usmle-kg dated 2026-07-08. Planning only — no code here._

## 0. Plain-language summary

usmle-kg reads a medical book and produces a web of medical facts (concepts + relationships). To get that web into the app cleanly, usmle-kg writes three files into one dated folder. The app imports those three files into its existing `Concept` and `ConceptEdge` tables and draws the graph. Nothing else needs to be shared — no databases, no live connection. A folder is the whole bridge (the app is cloud-hosted and cannot reach the Mac; a folder of files it can import is the honest, simple channel).

## 1. What kind of graph this is (the load-bearing caveat)

- usmle-kg produces **semantic medical edges** (e.g. `has_finding`, biolink predicates over UMLS CUIs) [FACT — Codex report §3]. These say "these two medical ideas are related," NOT "learn this one first."
- The app's planner gating (Slice 2) needs **prerequisite edges** (`type: "prereq"`, the default on `ConceptEdge` [FACT — entity file, verified 2026-07-08]). **usmle-kg does not produce prerequisite edges** [FACT — Codex report §3, edges are biolink semantic relations].
- **Therefore:** imported edges are tagged `relation_class: "semantic"` and power the **Slice 6 visual explore-graph only**. They must **never** silently drive Slice 2 gating. Prerequisite edges are a separate, later concern (derive from chapter/TOC order, or author key chains by hand) — tracked as an open item, not solved here.
- Coverage is **INBDE/medical only**; there is no TOEFL graph from this pipeline [FACT — Codex report §1, §6]. The KG view is INBDE-scoped until a TOEFL source exists.

## 2. The export folder

```
kg_export/<book_id>_<YYYY-MM-DD>/
├── manifest.json     ← one object: what this run is
├── nodes.jsonl       ← one JSON object per line = one concept
└── edges.jsonl       ← one JSON object per line = one relationship
```

One folder per export run. Dated so re-runs don't overwrite. The app imports the newest folder (or the owner picks one).

## 3. `manifest.json`

```json
{
  "format": "usmle-kg-exchange",
  "format_version": "1.0",
  "generated_at": "2026-07-08",
  "source_book_id": "first_aid",
  "source_book_title": null,
  "page_range": [312, 342],
  "pipeline_git_sha": "nogit",
  "exam_code": "INBDE",
  "importance_metric": "pagerank",
  "counts": { "nodes": 0, "edges": 0 },
  "coverage_note": "semantic biomedical edges (biolink) only; NOT pedagogical prerequisite edges",
  "definitions_present": false
}
```
- `source_book_title` may be `null` if unknown — do not invent one [honesty rule].
- `exam_code` is fixed `"INBDE"` for this pipeline (it is medical) [FACT — Codex report].
- `definitions_present` tells the app whether to expect node definitions (see §4).

## 4. `nodes.jsonl` — one concept per line

```json
{
  "node_id": "C1536220",
  "name": "STEMI",
  "node_type": "Disease",
  "definition": null,
  "importance_raw": 1.3039,
  "importance_norm": 0.87,
  "evidence_count": 7,
  "community_id": 37,
  "source_books": ["first_aid"],
  "source_pages": [313, 314],
  "exam_code": "INBDE",
  "fk_area": null
}
```

| field | source in usmle-kg | app use | honesty |
|---|---|---|---|
| `node_id` | the UMLS `cui` | stable id / dedupe key | [FACT] |
| `name` | `preferred_term` | node label | [FACT] |
| `node_type` | `node_type` (Disease/Finding/Anatomy/…) | **viz color by taxonomy role** | [FACT] |
| `definition` | from an Evidence node if available, else `null` | hover "thinking cloud" text | if `null`, hover says "no definition in source" — **never invent one** |
| `importance_raw` | `pagerank` | — | [FACT] the raw score |
| `importance_norm` | pagerank min-max scaled 0–1 within this export | **viz brightness = high-yield intensity** | [ESTIMATE] PageRank is a structural popularity proxy for high-yield, not verified exam-yield — label it as such in UI |
| `evidence_count` | `n_evidence` | secondary confidence signal | [FACT] |
| `community_id` | `community_id` | optional cluster grouping | [FACT] |
| `source_books` / `source_pages` | as stored | provenance / click-to-source | [FACT] |
| `exam_code` | fixed `"INBDE"` | app `Concept.exam_code` | [FACT] |
| `fk_area` | **not produced** → `null` | app blueprint tag | [UNKNOWN] — needs a CUI→FK-area mapping that does not exist yet; leave null, do not guess |

## 5. `edges.jsonl` — one relationship per line

```json
{
  "edge_id": "edge_5204ae694c087361",
  "from": "C1536220",
  "to": "C0520886",
  "relation": "has_finding",
  "relation_class": "semantic",
  "confidence": 0.95,
  "evidence_count": 1,
  "negated": false,
  "sources": [{"source": "llm_primary", "book_id": "first_aid", "chapter_id": "smoke_0312_0342", "chunk_id": "chk_...", "pages": [316, 317], "extractor_model": "kimi-k2.6", "evidence_span": {"start": 96, "end": 122, "text": "…ST elevation (STEMI"}}]
}
```
- `sources` is a list of **rich provenance objects** [AMENDED 2026-07-09 from the first real export — the exporter emits objects, not bare strings]. Each carries at least `source` (e.g. `"llm_primary"`); the rest (`book_id`, `chapter_id`, `chunk_id`, `pages`, `extractor_model`, `evidence_span`) enable click-to-source traceability. The app import reads `sources[].source` for the origin label and may store the object for provenance; it must NOT assume `sources` is a list of strings.
- `relation` = the biolink predicate with the `biolink:` prefix stripped [FACT — Codex report §3].
- `relation_class` is **always `"semantic"`** from this pipeline. The app must keep this field so its `ConceptEdge` can distinguish imported semantic edges from any native `prereq` edges. Semantic edges do not drive gating (§1).
- `negated: true` edges (e.g. "X does NOT cause Y") must render differently or be filtered — never shown as a plain positive link.
- `from`/`to` must both exist in `nodes.jsonl`; drop or log any edge whose endpoints are missing (no dangling edges).

## 6. How the app maps this in (Slice 6, `kgraph` is the design authority)

- `Concept` ← each node: `node_id, name, exam_code` map to existing fields; add Slice-6 deltas for `node_type, definition, importance_norm, community_id, source_pages` (kept nullable). `fk_area` stays null.
- `ConceptEdge` ← each edge: `from, to, relation, confidence`; add a `relation_class` field (existing native edges = `"prereq"`, imported = `"semantic"`).
- The graph **view** (`viz`) colors by `node_type`, sets brightness by `importance_norm` (shown as an estimate, not a verified yield), draws `semantic` edges for exploration, and uses the shared hover-cloud→panel→catalog idiom from Slice 1.
- Learner state (`mastery_state`: known / unasked) is filled by the app from the learner model at display time — it is **not** in the export.

## 7. Validation gate before any full run

1. Build the exporter and run it on the **existing smoke-run artifacts** (first_aid pages 312–342) — no new full-book run yet [reason: a full book is many hours at ~2.45h/31pp, and the pipeline is only smoke-tested, services were down — Codex report §5, §6].
2. Confirm the sample folder parses: valid JSON manifest, every `nodes.jsonl`/`edges.jsonl` line is valid JSON, no dangling edges, counts match the manifest.
3. Base44 imports the sample into a throwaway view and the owner eyeballs ~50 real concepts (Slice 6 gate G3: color legend, role vocabulary, importance encoding on real data) before committing to a full-book run.
4. Only then decide whether/which full book to run.

## 8. Open items (honest, not solved here)

- **Prerequisite edges** for planner gating — usmle-kg doesn't make them. Options: derive from chapter/TOC order, or hand-author key chains. Defer to a dedicated step.
- **FK/CC blueprint mapping** (CUI → INBDE Foundation-Knowledge area) — doesn't exist; needed if the graph should group by exam blueprint. Defer.
- **Definitions** — live on Evidence nodes, not concepts; if hovers should always show one, add a step to pull/generate + human-check (media/kgraph handshake). Until then, honest "no definition in source".
- **Pipeline hardening** — full-book success is unproven; some books lack `source.pdf`; `epi` currently yields zero triples [FACT — Codex report §6]. Treat any export as provisional until a clean full run exists.

---
_When Slice 6 is reached, copy this file into the app repo alongside the handoff (`docs/plan/`) so Base44's Slice-6 build has the contract in hand._
