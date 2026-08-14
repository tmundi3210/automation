# MEMORY_ARCHITECTURE — 8-layer memory for the bounded-autonomy system

Status: PLANNING (SPEC deliverable 7; implements SPEC Phase 5). Storage
choices are limited to what is verifiably available in this environment: git +
text files (proven substrate), python3 stdlib incl. SQLite, and optionally
pip-installable local vector libraries. No external database services, no new
credentials. Single-writer discipline (hub writes all shared memory paths)
carries over from EXCHANGE v3.3 unchanged.

Governing invariants (SPEC Phase 5 + ctxeng advisor):

1. Retrieval is project-scoped, source-aware, recency-aware, confidence-aware,
   and provenance-preserving.
2. **A vector index is never the authoritative record.** Every retrieved claim
   must hydrate back to its evidence record and raw source (`evidence_id` +
   `path@sha`). A hit that cannot be hydrated is discarded, not summarized.
3. Model-generated summaries are never primary evidence (SPEC Phase 4); they
   are derived artifacts that always cite the records they compress.
4. Conflicting sources are surfaced with the authority ladder (Tier A > B > C,
   then recency, then evidence-quality score) — never silently blended
   (ctxeng hard rule).
5. Secrets never enter any memory layer, any prompt, or any telemetry.

---

## The 8 layers

### L1 — Immutable raw-evidence store

- **Storage:** git-tracked append-only tree
  `orchestrator/memory/evidence_raw/<yyyy>/<mm>/<evidence_id>/` containing the
  fetched content (stripped text/markdown for HTML, PDF as-is if small,
  transcript for audio/video) plus `meta.json` (source_id, exact URL, fetch
  timestamp, sha256 of original bytes, license note). Filenames embed the
  content hash. This is the same append-only, never-edited discipline as
  `EXCHANGE/<agent>/msg-NNN.md`, enforced the same way: single writer (hub)
  plus a gate rule that any commit MODIFYING an existing path under
  `evidence_raw/` fails.
- **Why git-tracked:** immutability is checkable (`git log --follow` shows one
  commit per file or it's tampering — the existing message-ledger integrity
  check generalized); every other layer cites into it by `path@sha`.
- **Size control:** store extraction + hash + URL for large/binary sources
  rather than blobs; hard cap per item (proposed 256 KB, tune later); git-lfs
  availability in this environment is UNVERIFIED, so do not depend on it.
- **Compaction/expiry:** never edited, never deleted. Items past their
  evidence-record review date and not cited by any ACCEPTED decision move to
  an `archive/` branch in bulk (history preserved, working tree slim). No
  automatic purge — expiry of MEANING is handled at L2 via review dates.

### L2 — Structured relational store (sources, claims, projects, candidates, experiments, decisions)

- **Storage — canonical:** git-tracked JSONL/YAML ledgers under
  `orchestrator/state/`: `sources.yaml` (= SOURCE_REGISTRY, deliverable 4),
  `evidence.jsonl` (the Phase-4 evidence records, one JSON object per line,
  append-oriented; state changes append a superseding record rather than
  rewriting history), `claims.jsonl`, `project_map.jsonl`, `candidates.jsonl`,
  `experiments.jsonl`, `decisions.jsonl`. Single writer: hub. Schemas:
  DATA_SCHEMAS.md (deliverable 6).
- **Storage — derived:** local SQLite `var/orchestrator.db` (gitignored),
  rebuilt deterministically from the ledgers, drift-checked before use
  (rebuild + row/content-hash compare — the ROUTER.json/INDEX.json gate-3
  pattern from `tools/gate_all.sh`). All relational queries (joins for packet
  building, dedup lookups, metric rollups) run here; nothing is ever written
  here first.
- **Why not SQLite-as-canonical:** binary files defeat hub review, git diffs,
  and the audit-by-`git log -p` property; see SYSTEM_ARCHITECTURE §1.1.
- **Compaction/expiry:** ledgers are append-oriented; a monthly hub job writes
  a `snapshot/` compaction (latest state per item) and the eval registry's
  drift check guarantees snapshot ≡ replay. Evidence records carry an
  `expiration/review date` (SPEC Phase 4); a scheduled job mints re-verify
  tasks for expired-but-still-cited records and archives the rest.

### L3 — Vector index for semantic retrieval

Realistic local options, with tradeoffs:

| Option | Cost to adopt | Pros | Cons |
|---|---|---|---|
| **No vector initially (lexical)** — ripgrep + the proven `route_when` term-scoring pattern (`specialists/ROUTER.json`) + INDEX.json-style keyword indexes | zero | already running in this repo; deterministic; auditable; no embedding spend | recall limits on paraphrase/semantic queries |
| **sqlite-vec** | `pip install sqlite-vec` | lives inside the same derived SQLite file; single-file, serverless; fits the rebuildable-cache pattern exactly | young project; needs an embedding model (API spend or local — UNVERIFIED which is available/affordable) |
| **FAISS** | `pip install faiss-cpu` + numpy | fast, battle-tested at scale | separate index file to manage; overkill below ~100k chunks; same embedding dependency |
| Chroma / LanceDB | heavier install, more moving parts | nice APIs | server-ish footprint contradicts the minimal-substrate principle |

- **Recommendation:** ship Phase 1 with **lexical only** (option 1) — the
  corpus starts at zero and the existing term-scoring already routes 31
  specialists acceptably. Add **sqlite-vec** inside `var/orchestrator.db` when
  either (a) evidence records exceed ~2,000 or (b) the missed-retrieval rate
  measured by the eval registry shows lexical recall failing. Embedding model
  selection and its cost ceiling are UNVERIFIED and go through COST_BUDGET.yaml
  before adoption.
- **Authority rule:** vector rows store only `{evidence_id, chunk_ref,
  embedding}`. Query output is candidate IDs; the packet builder hydrates
  from L2/L1 and attaches provenance. The index is rebuilt from ledgers at any
  time; deleting it loses nothing.
- **Compaction/expiry:** rebuilt on ledger snapshot; chunks of archived L1
  items drop out automatically at rebuild.

### L4 — Project-specific semantic memory

- **Storage:** `orchestrator/memory/projects/<project_id>/digest.md` (curated,
  hub-written, ≤200 lines — the ctxeng standing-file band) plus `facts.jsonl`
  (one claim per line, each with `evidence_id` and confidence). Project ids
  come from PROJECT_REGISTRY.yaml (deliverable 2 — does not exist yet).
- **Retrieval:** strictly project-scoped; cross-project reads happen only in
  hub synthesis jobs, never in builder task packets (cross-project data
  leakage is a named Phase-9 threat).
- **Compaction/expiry:** digest.md is regenerated (not appended) by a weekly
  hub job from facts.jsonl + decisions ledger; facts whose evidence record was
  rejected or expired are dropped at regeneration — the digest can never
  outlive its provenance.

### L5 — Episodic run and experiment history

- **Storage:** `orchestrator/state/runs.jsonl` (run_id, trace_id, task_id,
  agent, tokens, cost, wall time, outcome, artifact paths) + full transcripts
  under `EXCHANGE/<agent>/transcripts/task-NNN/` (existing v3.3 mechanism for
  MED/HIGH tasks) + experiment manifests in `experiments.jsonl` (L2).
- **Retrieval:** by task/candidate/project id; used for trust scoring
  (SCORING.md inputs are ledger facts), variance estimation across repeated
  eval runs, and "have we tried this before" checks during PROJECT_MAPPED.
- **Compaction/expiry:** run rows are permanent (they are small and feed
  Phase-10 metrics). Transcripts: after 90 days, keep the hub-written summary
  + verdict, move raw transcripts to the archive branch. Never delete
  transcripts of HIGH-risk or disputed tasks.

### L6 — Procedural memory (approved workflows and skills)

- **Storage:** exists today — `specialists/*.specialist.json` (31 wired),
  `dist/specialist_prompts.jsonl`, `specialists/ROUTER.json`, the EXCHANGE
  playbooks (`CODEX_CLI_PLAYBOOK.md`, `GROK_CLI_PLAYBOOK.md`), and the
  standing-orders files (which live root-owned OUTSIDE the repo on the Mac by
  design — ORCHESTRATION §9 — with repo copies as reference only).
- **Write path:** new workflows/skills enter ONLY via the forge pipeline (FULL
  intake → spec → byte-identical re-forge → dense gates) — procedural memory
  is executable-adjacent, so it gets the strictest gate in the system. The
  system may PROPOSE prompt improvements (Phase 11) but security-relevant
  prompts are on the may-not-autonomously-change list.
- **Compaction/expiry:** monthly source-quality review also reviews
  specialists; retired ones get `status: RETIRED`, frozen not deleted
  (the offboarding pattern).

### L7 — Architecture-decision and rejection log

- **Storage:** `orchestrator/memory/decisions/ADR-NNNN.md` (append-only,
  hub-written) + `decisions.jsonl` index (L2). Every ACCEPTED/REJECTED verdict
  from the Phase-4 pipeline writes an entry: decision, alternatives
  considered, evidence_ids, review outcome (the Phase-7 JSON object verbatim),
  and — for rejections — the reason and a review date.
- **Why rejections are first-class:** a rejected technique WILL resurface in
  scout feeds; the DEDUPLICATED and PROJECT_MAPPED stages check this log so
  the system does not re-litigate (or silently re-adopt) rejected ideas
  without new evidence. New Tier-A evidence reopens an item explicitly, citing
  the ADR it supersedes.
- **Compaction/expiry:** never. This is the layer that most directly prevents
  repeated mistakes; it is small text and stays in the working tree
  permanently.

### L8 — Model, prompt, tool, dependency, and evaluator version registry

- **Storage:** `orchestrator/memory/registry/versions.yaml` (hub-written) —
  exact model ID strings per agent, prompt files pinned as `path@sha`,
  CLI/tool versions from the capability matrix (deliverable 1), evaluator
  fingerprints = the `gate_all.sh --fingerprint` FPR lines (this mechanism
  already exists and is reused as-is: e.g. at HEAD `c0a0f1e5`,
  `gate_all.sh = bd8210c2…`, `kb_validator.py = e0348b61…`). Python
  dependencies are captured per-experiment (`pip freeze` into the experiment
  manifest) rather than globally, because experiments pin, the substrate
  itself is stdlib-only by design.
- **Retrieval:** every experiment record and every eval baseline references a
  registry snapshot (`versions.yaml@sha`); Phase-8 "pin model, prompt,
  dependency, and tool versions" is a lookup, not a ritual.
- **Compaction/expiry:** append/supersede; old pins are never rewritten (they
  are what makes historical eval results interpretable).

---

## Global retrieval rules (apply to every layer)

Applied in order by the deterministic packet builder (`orchestrator/memory/`),
not by model judgment:

1. **Project scope filter** — only items mapped to the requesting task's
   project(s), plus explicitly whitelisted global items (protocol docs,
   ontology).
2. **Authority ladder** — Tier A > Tier B > Tier C; within a tier, evidence
   records outrank model-generated derivatives; contradictions are included
   as contradictions (both sides, labeled), never merged.
3. **Recency + review dates** — expired records are excluded from
   architect/implementer packets (they may appear in scout packets flagged
   re-verify); recency ranks within equal authority.
4. **Confidence floor** — default packets include only VERIFIED-or-later items
   with evidence-quality ≥ threshold (tunable per task RISK); scout packets
   may see DISCOVERED items (that is their job).
5. **Provenance mandatory** — every item in a packet carries
   `evidence_id + path@sha` (and URL for external sources). The packet
   builder drops anything it cannot cite; fabricated-locator checks (ctxeng)
   reject packets citing paths/SHAs that do not resolve.
6. **Vector is candidate-generation only** — L3 output must hydrate through
   L2/L1 before it can enter a packet (rule 5 then applies).

## Retrieval packets per role (ctxeng budget + isolation rules)

Shared discipline (POLLING §3–4, ORCHESTRATION §5b): fresh context per task;
byte-stable prompt prefix (no timestamps/counters in the static region); all
per-task variance in one final segment; critical constraints FIRST and
restated LAST; live context ≤40–60% of the window; digests in the ~300-token
to ~10k band; plan/state persists to the staging dir, not the chat.

| Packet | Enters context | Stays external (locator + one-line gist) | Budget target |
|---|---|---|---|
| **Scout (grok)** | task pins; topic terms from ontology; SOURCE_REGISTRY slice for its beat (ids, URLs, last-seen watermarks); recent-hash sample for cheap client-side dedup; evidence-record output schema | project code (never); other layers; prior scout transcripts | ~2–4k tokens |
| **Architect/Critic (claude subagent)** | task pins; hydrated evidence digests with `evidence_id + path@sha`; the target project's `digest.md` (L4); relevant ADRs (L7); Phase-6 schema when analyzing a framework | raw L1 content (fetch on trigger by locator); other projects' memory; scout self-assessments | ~8–12k tokens |
| **Implementer/Verifier (codex)** | task pins (id/branch/write_scope/gates/deliverables); candidate spec digest; acceptance thresholds copied from EVALUATION_REGISTRY.yaml; version pins from L8; only files inside write_scope; current failing output when iterating | constitution body; raw web content (never); scout transcripts (never); peer deliverables not named as inputs; secrets (hard stop) | per POLLING §4 |
| **Judge (claude / panel)** | artifact diff pinned to delivered_sha; rubric + anchors; deterministic gate output; both-order position-swapped presentation | builder's self-assessment (NEVER — contamination, POLLING §5); authorship (blind) | minimal; gates ruled first |

Packet construction is itself logged (run_id + the exact locator list) into L5
so a bad decision can be traced to what the model was — and was not — shown.

## Compaction/expiry summary

| Layer | Policy |
|---|---|
| L1 raw evidence | immutable; bulk-archive branch for stale+uncited after 12 months; no purge |
| L2 relational ledgers | append/supersede; monthly snapshot compaction with replay-equivalence check; review dates drive re-verify tasks |
| L3 vector | disposable; rebuilt from L2 snapshots; archived items drop at rebuild |
| L4 project memory | digest regenerated weekly from provenance; orphaned facts dropped automatically |
| L5 episodic | run rows permanent; transcripts summarized at 90 days except HIGH-risk/disputed (kept) |
| L6 procedural | forge-gated writes; monthly review; retire-freeze, never delete |
| L7 decisions/rejections | never compacted, never expires |
| L8 version registry | append/supersede; historical pins immutable |
