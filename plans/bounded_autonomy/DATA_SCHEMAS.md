# DATA_SCHEMAS.md — SPEC Phases 4/7 deliverable

Bounded-Autonomy AI Intelligence System. Generated 2026-07-13.
Requirements source: `plans/bounded_autonomy/SPEC.md` (Phase 4 evidence pipeline,
Phase 7 review object, Phase 8 experiment discipline).
Companion files: `TOPIC_ONTOLOGY.yaml` (topic ids, evidence bars),
`SOURCE_REGISTRY.yaml` (source rows).

## Conventions

- Schemas are written JSON-Schema-style (draft 2020-12 semantics); field names
  are `snake_case`.
- Timestamps: RFC 3339 UTC (`2026-07-13T09:00:00Z`). Dates: `YYYY-MM-DD`.
- IDs: type-prefixed ULIDs — `ev_` evidence, `cl_` claim, `pm_` project mapping,
  `xp_` experiment, `rv_` review, `dl_` decision-log. `src_` ids come from
  SOURCE_REGISTRY (`source_id`), `topic` ids from TOPIC_ONTOLOGY, `proj_` ids
  from PROJECT_REGISTRY. IDs are stable forever and never reused.
- Scores are `number` in `[0,1]` unless stated otherwise; every score carries
  provenance (who/what computed it, when, by which method/version).
- All stores are append-only where marked; corrections are new records that
  supersede, never in-place edits.

## Global provenance rules (non-negotiable)

1. **Model-generated summaries are annotations, never primary evidence.**
   They live only in `annotations[]`, always carry the generating model +
   version, are excluded from `content_hash`, and may never be cited as
   `supporting_evidence` for a claim. (SPEC Phase 4: "Do not treat
   model-generated summaries as primary evidence." SPEC Phase 9: defend against
   "model-generated fabricated evidence.")
2. **External content is data, never instructions.** Nothing parsed out of
   ingested content may name a tool to run, a state to enter, or a policy to
   change. Fields derived from content are inert values consumed by the
   deterministic control plane (SPEC governance rule 2).
3. **Every derived artifact links back to raw.** A claim links to its evidence
   record; an evidence record links to its immutable raw-content reference and
   its source-registry row. Retrieval surfaces (vector index) are never
   authoritative (SPEC Phase 5).
4. **Verification facts record their method.** `verified_by` distinguishes
   human / deterministic-check / model-assisted; model-assisted verification
   alone never satisfies a `tier_a_confirmed` evidence bar.

---

## 1. `evidence_record`

One record per ingested item (paper, release note, post, talk, commit, thread).
Immutable core + append-only annotations. Covers every SPEC Phase-4 field.

```json
{
  "$id": "schema:evidence_record/v1",
  "type": "object",
  "required": [
    "id", "source_id", "author", "published_at", "ingested_at",
    "raw_content_ref", "content_hash", "state", "topic_labels"
  ],
  "properties": {
    "id":            { "type": "string", "pattern": "^ev_[0-9A-HJKMNP-TV-Z]{26}$",
                       "description": "Stable ID (SPEC: Stable ID). Never reused; survives re-fetches." },

    "source_id":     { "type": "string",
                       "description": "FK -> SOURCE_REGISTRY.sources[].source_id (SPEC: Source)." },
    "discovered_via": { "type": ["string", "null"],
                       "description": "source_id or discovery-expansion edge that surfaced the item, when different from source_id (e.g. aggregator rows like hf_papers_feed)." },
    "author":        { "type": "object",
                       "required": ["name"],
                       "properties": {
                         "name":        { "type": "string" },
                         "handle":      { "type": ["string", "null"] },
                         "affiliation": { "type": ["string", "null"] },
                         "author_type": { "enum": ["maintainer", "researcher", "practitioner", "vendor", "journalist", "unknown"] }
                       },
                       "description": "SPEC: author. Affiliation feeds bias weighting (vendor flags)." },

    "published_at":  { "type": "string", "format": "date-time",
                       "description": "SPEC: publication timestamp. If the source gives none, best-estimate plus published_at_precision." },
    "published_at_precision": { "enum": ["exact", "date_only", "estimated"], "default": "exact" },
    "ingested_at":   { "type": "string", "format": "date-time",
                       "description": "SPEC: ingestion timestamp. Set by control plane, not by content." },

    "raw_content_ref": { "type": "string", "format": "uri",
                       "description": "URI into the immutable raw-evidence store (SPEC Phase 5 layer 1), e.g. raw://evidence/2026/07/13/<sha256>. Write-once." },
    "content_hash":  { "type": "string", "pattern": "^sha256:[0-9a-f]{64}$",
                       "description": "Hash of exact raw bytes. See content_hash spec below." },
    "normalized_hash": { "type": ["string", "null"], "pattern": "^sha256:[0-9a-f]{64}$",
                       "description": "Hash of canonicalized text; used for dedup and material-change detection. See spec below." },

    "extracted_claims": { "type": "array", "items": { "type": "string", "pattern": "^cl_" },
                       "description": "SPEC: extracted claims. FKs -> claim records; claims are separate objects, not inline prose." },
    "supporting_evidence": { "type": "array", "items": { "type": "string", "pattern": "^ev_" },
                       "description": "SPEC: supporting evidence. Evidence records that corroborate this item's claims." },
    "contradicting_evidence": { "type": "array", "items": { "type": "string", "pattern": "^ev_" },
                       "description": "SPEC: contradicting evidence. Kept, never deleted — contradiction is signal." },

    "related_code": { "type": "array",
                       "items": { "type": "object",
                         "required": ["repo_url"],
                         "properties": {
                           "repo_url":  { "type": "string", "format": "uri" },
                           "ref":       { "type": ["string", "null"], "description": "Commit SHA or tag actually inspected — 'main' is not a ref for evidence purposes." },
                           "license":   { "type": ["string", "null"] },
                           "executed_in_sandbox": { "type": "boolean", "default": false,
                             "description": "True only after the isolated-sandbox run recorded in an experiment_manifest (SPEC governance rule 6)." }
                         } },
                       "description": "SPEC: relevant code/repositories." },

    "topic_labels": { "type": "array", "minItems": 1,
                       "items": { "type": "object",
                         "required": ["topic_id", "labeled_by"],
                         "properties": {
                           "topic_id":   { "type": "string", "description": "FK -> TOPIC_ONTOLOGY topic id." },
                           "confidence": { "type": ["number", "null"], "minimum": 0, "maximum": 1 },
                           "labeled_by": { "enum": ["classifier", "human", "rule"] }
                         } },
                       "description": "SPEC: topic labels. Determines the evidence bar via TOPIC_ONTOLOGY default_evidence_bar (strictest bar among labels wins)." },

    "affected_projects": { "type": "array", "items": { "type": "string", "pattern": "^proj_" },
                       "description": "SPEC: affected projects. Populated at PROJECT_MAPPED; respects exclusion_policy." },

    "novelty_score":        { "type": ["number", "null"], "minimum": 0, "maximum": 1, "description": "SPEC: novelty score." },
    "relevance_score":      { "type": ["number", "null"], "minimum": 0, "maximum": 1, "description": "SPEC: relevance score." },
    "evidence_quality_score": { "type": ["number", "null"], "minimum": 0, "maximum": 1,
                       "description": "SPEC: evidence-quality score. Inputs: source tier, primary_proximity, corroboration count, bias_flags." },
    "reproducibility_score": { "type": ["number", "null"], "minimum": 0, "maximum": 1,
                       "description": "SPEC: reproducibility score. 1.0 requires executed_in_sandbox code, not the existence of a repo link." },
    "scoring_provenance": { "type": ["object", "null"],
                       "properties": {
                         "scored_by":   { "type": "string", "description": "model id+version or 'rule:<name>'" },
                         "method":      { "type": "string" },
                         "scored_at":   { "type": "string", "format": "date-time" }
                       },
                       "description": "Required once any score is non-null." },

    "expected_impact": { "type": ["object", "null"],
                       "properties": {
                         "level":     { "enum": ["low", "medium", "high"] },
                         "dimension": { "type": "array", "items": { "enum": ["quality", "latency", "cost", "reliability", "security", "capability"] } },
                         "rationale": { "type": "string" }
                       },
                       "description": "SPEC: expected impact." },
    "estimated_implementation_cost": { "type": ["object", "null"],
                       "properties": {
                         "engineering_days": { "type": ["number", "null"] },
                         "compute_usd":      { "type": ["number", "null"] },
                         "uncertainty":      { "enum": ["low", "medium", "high"] }
                       },
                       "description": "SPEC: estimated implementation cost." },
    "risks": { "type": "object",
                       "properties": {
                         "security":      { "type": "array", "items": { "type": "string" } },
                         "compatibility": { "type": "array", "items": { "type": "string" } }
                       },
                       "description": "SPEC: security and compatibility risks." },

    "confidence": { "type": ["number", "null"], "minimum": 0, "maximum": 1,
                       "description": "SPEC: confidence — overall belief that this item's claims are true as stated." },
    "review_by":  { "type": "string", "format": "date",
                       "description": "SPEC: expiration/review date. After this date the record is stale: it must be re-reviewed before being used in any new recommendation. Benchmark standings decay fast — default 90 days, security topics 30 days." },

    "state": { "enum": ["DISCOVERED", "NORMALIZED", "DEDUPLICATED", "VERIFIED", "SCORED",
                        "PROJECT_MAPPED", "EXPERIMENT_CANDIDATE", "EXPERIMENTED",
                        "ACCEPTED", "REJECTED", "PR_READY", "HUMAN_APPROVED",
                        "DEPLOYED", "MONITORED", "RETAINED", "ROLLED_BACK"],
               "description": "Current state-machine position (section 8). Transitions only via decision_log entries." },

    "duplicate_of": { "type": ["string", "null"], "pattern": "^ev_",
                       "description": "Set at DEDUPLICATED when normalized_hash (or near-dup detector) matches an earlier record; duplicates park here and do not advance." },

    "annotations": { "type": "array",
                       "items": { "type": "object",
                         "required": ["annotation_id", "annotation_type", "generated_by", "created_at", "content_ref"],
                         "properties": {
                           "annotation_id":   { "type": "string" },
                           "annotation_type": { "enum": ["summary", "classification_rationale", "translation", "extraction_note", "comparison"] },
                           "generated_by":    { "type": "string", "description": "Exact model id + version, or human identity." },
                           "created_at":      { "type": "string", "format": "date-time" },
                           "content_ref":     { "type": "string", "format": "uri" }
                         } },
                       "description": "ANNOTATIONS ONLY. Model output about the evidence. Never hashed into content_hash, never citable as supporting/contradicting evidence, never an input to claim verification. Deleting an annotation never changes evidence state." }
  }
}
```

### `content_hash` spec

- **What is hashed:** the exact raw bytes as fetched, after transport decoding
  (chunked/gzip removed) and **before** any normalization, rendering, boilerplate
  stripping, or model processing. For binary media (PDF, audio, video, images):
  the file bytes. For API/JSON responses: the raw response body bytes.
- **Algorithm/format:** SHA-256, lowercase hex, serialized as `sha256:<64 hex>`.
  Algorithm agility: any future algorithm change keeps the prefix convention and
  re-hashes nothing retroactively (old records keep old hashes).
- **`normalized_hash` (for dedup + material-change detection):** SHA-256 over the
  canonicalized text: Unicode NFC, HTML/markup stripped by a deterministic
  extractor (fixed version recorded in the raw store), whitespace collapsed to
  single spaces, leading/trailing whitespace removed. The extractor version is
  part of the raw store metadata; changing the extractor never rewrites existing
  hashes.
- **Usage rules:**
  - Same `normalized_hash` as an existing record ⇒ duplicate (set `duplicate_of`,
    increment source `duplicate_rate` statistics).
  - Same URL re-fetched with a different `normalized_hash` ⇒ **material change**
    ⇒ new evidence record (new `id`) with `supersedes: <old ev_id>` noted in the
    raw store; this is the only trigger for "expensive model analysis" on
    re-fetches (SPEC cadence rule), and it is computed without any model call.
  - `raw_content_ref` is write-once; hash mismatch between store and record is a
    P1 integrity alert (audit).

---

## 2. `source_record`

The typed form of one `sources[]` row in SOURCE_REGISTRY.yaml (that file is the
human-edited surface; this schema is what the control plane validates it
against). Field semantics are documented in SOURCE_REGISTRY `_schema` and are
not repeated here.

```json
{
  "$id": "schema:source_record/v1",
  "type": "object",
  "required": ["source_id", "type", "tier", "topics", "expected_latency",
               "poll_mechanism", "enabled", "status"],
  "properties": {
    "source_id":         { "type": "string", "pattern": "^[a-z0-9_]+$" },
    "url":               { "type": ["string", "null"], "format": "uri" },
    "feed":              { "type": ["string", "null"], "format": "uri" },
    "api":               { "type": ["string", "null"], "format": "uri" },
    "type":              { "enum": ["official_docs", "changelog_release_notes", "repository_releases",
                                    "paper_listing", "paper", "benchmark", "blog", "engineering_writeup",
                                    "newsletter", "podcast", "youtube_channel", "conference_talks",
                                    "x_list", "aggregator", "case_study"] },
    "tier":              { "enum": ["A", "B", "C"] },
    "topics":            { "type": "array", "minItems": 1, "items": { "type": "string" } },
    "expected_latency":  { "enum": ["minutes", "hours", "days", "weekly", "monthly"] },
    "poll_mechanism":    { "enum": ["webhook", "rss_event", "rss_daily", "api_poll_1_3h",
                                    "api_poll_daily", "scrape_daily", "static", "none"] },
    "accuracy_history":  { "type": ["number", "null"], "minimum": 0, "maximum": 1 },
    "snr_score":         { "type": ["number", "null"], "minimum": 0, "maximum": 1 },
    "duplicate_rate":    { "type": ["number", "null"], "minimum": 0, "maximum": 1 },
    "relevance":         { "type": ["number", "null"], "minimum": 0, "maximum": 1 },
    "primary_proximity": { "enum": ["primary", "near_primary", "secondary", "aggregator", null] },
    "repro_availability": { "enum": ["always", "frequent", "occasional", "rare", "none", null] },
    "license_constraints": { "type": "string" },
    "rate_limits_cost":  { "type": "string" },
    "enabled":           { "type": "boolean" },
    "bootstrap":         { "type": "boolean" },
    "bias_flags":        { "type": "array", "items": { "type": "string" } },
    "status":            { "enum": ["active", "probation", "pending_resolution", "disabled_dead_link", "pruned"] },
    "verification":      { "type": "string" },
    "provenance":        { "type": "string" },
    "notes":             { "type": ["string", "null"] }
  },
  "allOf": [
    { "if": { "properties": { "status": { "const": "pending_resolution" } } },
      "then": { "properties": { "enabled": { "const": false } } } },
    { "if": { "properties": { "enabled": { "const": true } } },
      "then": { "not": { "properties": { "verification": { "pattern": "^UNVERIFIED" } } } } }
  ]
}
```

Invariants: `pending_resolution` ⇒ `enabled: false`; a row whose `verification`
starts with `UNVERIFIED` cannot be `enabled: true`. Observed scores may only be
written by the monthly source-quality job or a human — never by ingestion agents.

---

## 3. `claim`

An atomic, checkable assertion extracted from evidence. Claims — not documents —
are what gets verified, contradicted, and mapped to projects.

```json
{
  "$id": "schema:claim/v1",
  "type": "object",
  "required": ["id", "origin_evidence_id", "text", "claim_type", "verification_status"],
  "properties": {
    "id":                 { "type": "string", "pattern": "^cl_[0-9A-HJKMNP-TV-Z]{26}$" },
    "origin_evidence_id": { "type": "string", "pattern": "^ev_",
                            "description": "Evidence record the claim was extracted from. The claim inherits that record's provenance chain." },
    "text":               { "type": "string",
                            "description": "The assertion, as close to verbatim as extraction allows." },
    "is_paraphrase":      { "type": "boolean", "default": false,
                            "description": "True when text is not a verbatim quote. Paraphrases are extraction artifacts: verification must go back to the raw content, not the paraphrase." },
    "locator":            { "type": ["string", "null"],
                            "description": "Where in the raw content: section/paragraph/timestamp offset." },
    "claim_type":         { "enum": ["capability", "benchmark_result", "technique", "security_vulnerability",
                                     "cost_pricing", "availability_release", "incompatibility", "opinion"] },
    "quantitative":       { "type": ["object", "null"],
                            "properties": {
                              "metric":    { "type": "string" },
                              "value":     { "type": "number" },
                              "unit":      { "type": "string" },
                              "baseline":  { "type": ["string", "null"] },
                              "conditions": { "type": ["string", "null"], "description": "Measurement conditions as stated; a number without conditions verifies at most as 'reported'." }
                            } },
    "supporting_evidence": { "type": "array",
                            "items": { "type": "object",
                              "required": ["evidence_id"],
                              "properties": {
                                "evidence_id": { "type": "string", "pattern": "^ev_" },
                                "locator":     { "type": ["string", "null"] },
                                "independence": { "enum": ["same_org", "independent", "unknown"],
                                  "description": "tier_a_confirmed bars require at least one 'independent' entry." }
                              } },
                            "description": "SPEC: supporting evidence — links, not prose." },
    "contradicting_evidence": { "type": "array",
                            "items": { "type": "object",
                              "required": ["evidence_id"],
                              "properties": {
                                "evidence_id": { "type": "string", "pattern": "^ev_" },
                                "locator":     { "type": ["string", "null"] },
                                "note":        { "type": ["string", "null"] }
                              } } },
    "verification_status": { "enum": ["unverified", "verified", "contested", "refuted", "expired"],
                            "description": "contested = credible support AND contradiction coexist; surfaced to humans, never silently resolved by a model." },
    "verification_method": { "type": ["string", "null"],
                            "description": "How verified: 'deterministic:<check>' (hash, version lookup, sandbox run), 'cross_source' (independent Tier-A corroboration), 'human'. Model-assisted checks are recorded as cross_source inputs, not as verification by themselves." },
    "verified_by":        { "type": ["string", "null"], "description": "Actor identity (role + model version, or human)." },
    "verified_at":        { "type": ["string", "null"], "format": "date-time" },
    "confidence":         { "type": ["number", "null"], "minimum": 0, "maximum": 1 },
    "topic_ids":          { "type": "array", "items": { "type": "string" } }
  }
}
```

---

## 4. `project_mapping`

The join between a scored evidence record (or claim set) and one project.
Created at the PROJECT_MAPPED state; one record per (evidence, project) pair.

```json
{
  "$id": "schema:project_mapping/v1",
  "type": "object",
  "required": ["id", "evidence_id", "project_id", "matched_topics", "exclusion_check", "proposed_action"],
  "properties": {
    "id":            { "type": "string", "pattern": "^pm_[0-9A-HJKMNP-TV-Z]{26}$" },
    "evidence_id":   { "type": "string", "pattern": "^ev_" },
    "claim_ids":     { "type": "array", "items": { "type": "string", "pattern": "^cl_" },
                       "description": "The specific claims that make this relevant — mapping a document without naming claims is not allowed." },
    "project_id":    { "type": "string", "pattern": "^proj_", "description": "FK -> PROJECT_REGISTRY.yaml." },
    "matched_topics": { "type": "array", "minItems": 1, "items": { "type": "string" } },
    "exclusion_check": { "type": "object",
                       "required": ["evaluated_at", "excluded"],
                       "properties": {
                         "evaluated_at":      { "type": "string", "format": "date-time" },
                         "excluded":          { "type": "boolean" },
                         "matched_exclusions": { "type": "array", "items": { "type": "string" },
                            "description": "Exclusion record topic_ids that fired (TOPIC_ONTOLOGY exclusion_policy)." },
                         "mode":              { "enum": ["full", "digest_only", null] }
                       },
                       "description": "Mandatory even when negative — proves the exclusion policy ran." },
    "relevance_score": { "type": ["number", "null"], "minimum": 0, "maximum": 1 },
    "expected_impact": { "type": ["object", "null"],
                       "properties": {
                         "level":     { "enum": ["low", "medium", "high"] },
                         "dimension": { "type": "array", "items": { "enum": ["quality", "latency", "cost", "reliability", "security", "capability"] } },
                         "rationale": { "type": "string" }
                       } },
    "existing_implementation": { "type": ["string", "null"],
                       "description": "What the project does today for this concern (from PROJECT_REGISTRY architecture summary / Architect analysis) — the comparison baseline." },
    "integration_risks": { "type": "array", "items": { "type": "string" } },
    "proposed_action": { "enum": ["ignore", "digest_only", "watch", "experiment_candidate"],
                       "description": "experiment_candidate additionally requires: evidence bar met, autonomy level of the project permits experiments, and budget headroom — checked by control plane, not asserted here." },
    "evidence_bar_check": { "type": "object",
                       "required": ["required_bar", "met"],
                       "properties": {
                         "required_bar": { "enum": ["tier_a_confirmed", "tier_a_primary", "tier_a_or_repro", "repro_required"] },
                         "met":          { "type": "boolean" },
                         "basis":        { "type": "array", "items": { "type": "string" },
                                           "description": "evidence/claim ids satisfying the bar." }
                       } },
    "created_by":    { "type": "string", "description": "Agent role + model version (typically Claude Architect/Critic)." },
    "created_at":    { "type": "string", "format": "date-time" }
  }
}
```

---

## 5. `experiment_manifest`

One per isolated experiment (SPEC Phase 8 discipline, machine-checkable).
Created before EXPERIMENT_CANDIDATE → EXPERIMENTED; frozen at execution start.

```json
{
  "$id": "schema:experiment_manifest/v1",
  "type": "object",
  "required": ["id", "project_mapping_id", "hypothesis", "baseline", "pinned_versions",
               "isolation", "thresholds", "evaluations", "rollback_plan", "status"],
  "properties": {
    "id":                { "type": "string", "pattern": "^xp_[0-9A-HJKMNP-TV-Z]{26}$" },
    "project_mapping_id": { "type": "string", "pattern": "^pm_" },
    "evidence_ids":      { "type": "array", "items": { "type": "string", "pattern": "^ev_" } },
    "hypothesis":        { "type": "string",
                           "description": "Falsifiable statement: expected metric movement, direction, size." },

    "baseline":          { "type": "object",
                           "required": ["captured_at", "metrics", "commands"],
                           "properties": {
                             "captured_at": { "type": "string", "format": "date-time" },
                             "commit":      { "type": "string", "description": "SHA of unchanged baseline code." },
                             "metrics":     { "type": "object", "additionalProperties": { "type": "number" } },
                             "commands":    { "type": "array", "items": { "type": "string" },
                                              "description": "Exact commands that produced the baseline metrics." }
                           },
                           "description": "SPEC Phase 8: record the existing baseline BEFORE testing." },

    "pinned_versions":   { "type": "object",
                           "required": ["models", "prompts", "dependencies", "tools", "evaluators"],
                           "properties": {
                             "models":       { "type": "array", "items": { "type": "object",
                                               "required": ["provider", "model_id"],
                                               "properties": {
                                                 "provider": { "type": "string" },
                                                 "model_id": { "type": "string", "description": "Exact versioned id, not an alias like 'latest'." } } } },
                             "prompts":      { "type": "array", "items": { "type": "object",
                                               "required": ["prompt_id", "hash"],
                                               "properties": { "prompt_id": { "type": "string" },
                                                               "hash": { "type": "string", "pattern": "^sha256:" } } } },
                             "dependencies": { "type": "string", "description": "Lockfile content hash (sha256:...)." },
                             "tools":        { "type": "array", "items": { "type": "string" }, "description": "tool@version list incl. MCP servers." },
                             "evaluators":   { "type": "array", "items": { "type": "string" }, "description": "Evaluator/judge versions (SPEC Phase 5 layer 8)." }
                           } },

    "isolation":         { "type": "object",
                           "required": ["kind", "ref", "network_policy", "secrets_policy"],
                           "properties": {
                             "kind":           { "enum": ["worktree", "container", "worktree_in_container"] },
                             "ref":            { "type": "string", "description": "Worktree path / container image digest." },
                             "network_policy": { "type": "string", "description": "Default deny; explicit allowlist entries with reasons." },
                             "secrets_policy": { "type": "string", "description": "Default: no secrets mounted; each exception named + justified (SPEC Phase 8: remove unnecessary secrets and networks)." }
                           } },

    "thresholds":        { "type": "object",
                           "required": ["success", "failure", "regression"],
                           "properties": {
                             "success":    { "type": "string", "description": "Machine-evaluable predicate over result metrics." },
                             "failure":    { "type": "string" },
                             "regression": { "type": "string", "description": "Max tolerated degradation on any baseline metric." }
                           },
                           "description": "Defined BEFORE execution; editing thresholds after start voids the experiment." },

    "evaluations":       { "type": "array", "minItems": 1,
                           "items": { "type": "object",
                             "required": ["kind", "command", "repetitions"],
                             "properties": {
                               "kind":        { "enum": ["unit", "integration", "e2e", "security", "compatibility", "latency", "quality", "cost"] },
                               "command":     { "type": "string" },
                               "repetitions": { "type": "integer", "minimum": 1,
                                                "description": ">1 required for nondeterministic evals to estimate variance (SPEC Phase 8)." }
                             } } },

    "results":           { "type": ["object", "null"],
                           "properties": {
                             "metrics":            { "type": "object", "additionalProperties": { "type": "number" } },
                             "variance":           { "type": ["object", "null"], "additionalProperties": { "type": "number" } },
                             "baseline_delta":     { "type": ["object", "null"], "additionalProperties": { "type": "number" } },
                             "logs_ref":           { "type": "string", "format": "uri", "description": "All commands, environment details, outputs (SPEC Phase 8)." },
                             "artifacts":          { "type": "array", "items": { "type": "string", "format": "uri" } }
                           } },

    "verdict":           { "enum": ["accepted", "rejected", "inconclusive", null],
                           "description": "Computed by the control plane from thresholds + results. Agent self-report is never the verdict (research digest: reward hacking is measured; gates run from the integrator's copies)." },
    "rollback_plan":     { "type": "string",
                           "description": "Recorded before deployment; executed on post-deploy threshold violation (SPEC Phase 8)." },
    "status":            { "enum": ["draft", "frozen", "running", "completed", "voided"] },
    "run_ids":           { "type": "array", "items": { "type": "string" } }
  }
}
```

---

## 6. `review_object`

SPEC Phase 7, verbatim — every independent review (Claude Code Architect/Critic,
Codex Verifier, Grok Scout, or rotated role) must return exactly this object:

```json
{
  "decision": "approve | approve_with_changes | reject",
  "confidence": 0.0,
  "verified_claims": [],
  "unverified_claims": [],
  "concerns": [],
  "required_changes": [],
  "evidence_ids": [],
  "recommended_next_state": ""
}
```

Typed schema and storage envelope (the envelope wraps, never alters, the SPEC
object):

```json
{
  "$id": "schema:review_object/v1",
  "type": "object",
  "required": ["review_id", "subject", "reviewer", "created_at", "object"],
  "properties": {
    "review_id":  { "type": "string", "pattern": "^rv_[0-9A-HJKMNP-TV-Z]{26}$" },
    "subject":    { "type": "object",
                    "required": ["type", "id"],
                    "properties": {
                      "type": { "enum": ["evidence_record", "claim", "project_mapping", "experiment_manifest", "pull_request", "plan"] },
                      "id":   { "type": "string" }
                    } },
    "reviewer":   { "type": "object",
                    "required": ["role", "model"],
                    "properties": {
                      "role":  { "enum": ["grok_scout", "claude_architect_critic", "codex_implementer_verifier", "human"] },
                      "model": { "type": "string", "description": "Exact model id + version; 'n/a' for human." },
                      "rotation_note": { "type": ["string", "null"], "description": "Set when role rotation assigned this review to a non-default model (SPEC Phase 7 single-model-bias mitigation)." }
                    } },
    "created_at": { "type": "string", "format": "date-time" },
    "object":     { "type": "object",
                    "required": ["decision", "confidence", "verified_claims", "unverified_claims",
                                 "concerns", "required_changes", "evidence_ids", "recommended_next_state"],
                    "properties": {
                      "decision":            { "enum": ["approve", "approve_with_changes", "reject"] },
                      "confidence":          { "type": "number", "minimum": 0, "maximum": 1 },
                      "verified_claims":     { "type": "array", "items": { "type": "string" }, "description": "claim ids (cl_...)" },
                      "unverified_claims":   { "type": "array", "items": { "type": "string" } },
                      "concerns":            { "type": "array", "items": { "type": "string" } },
                      "required_changes":    { "type": "array", "items": { "type": "string" } },
                      "evidence_ids":        { "type": "array", "items": { "type": "string" } },
                      "recommended_next_state": { "type": "string",
                        "description": "Must name a state legal from the subject's current state per section 8. A recommendation is advisory: only the control plane or a human executes transitions." }
                    } }
  }
}
```

---

## 7. `decision_log` entry

Append-only, hash-chained audit record. Every state transition, approval,
budget action, and configuration change writes exactly one entry.

```json
{
  "$id": "schema:decision_log_entry/v1",
  "type": "object",
  "required": ["id", "ts", "run_id", "actor", "action", "subject", "rationale", "prev_entry_hash", "entry_hash"],
  "properties": {
    "id":       { "type": "string", "pattern": "^dl_[0-9A-HJKMNP-TV-Z]{26}$" },
    "ts":       { "type": "string", "format": "date-time" },
    "run_id":   { "type": "string" },
    "trace_id": { "type": ["string", "null"], "description": "Correlates with observability traces (SPEC Phase 10)." },
    "actor":    { "type": "object",
                  "required": ["kind", "identity"],
                  "properties": {
                    "kind":     { "enum": ["human", "control_plane", "agent_role"] },
                    "identity": { "type": "string", "description": "Username / job name / role+exact model version." }
                  },
                  "description": "External content is never an actor and cannot appear here." },
    "action":   { "enum": ["state_transition", "review_submitted", "approval_granted", "approval_denied",
                           "config_change_proposed", "config_change_applied", "budget_event",
                           "source_score_update", "rollback_executed", "integrity_alert"] },
    "subject":  { "type": "object",
                  "required": ["type", "id"],
                  "properties": {
                    "type": { "type": "string" },
                    "id":   { "type": "string" }
                  } },
    "prior_state": { "type": ["string", "null"], "description": "For state_transition: state before." },
    "new_state":   { "type": ["string", "null"], "description": "For state_transition: state after. Pair must be a legal transition (section 8)." },
    "evidence_ids": { "type": "array", "items": { "type": "string" } },
    "review_ids":   { "type": "array", "items": { "type": "string" } },
    "rationale":    { "type": "string", "description": "Why — required, human-readable, no empty strings." },
    "approval":     { "type": ["object", "null"],
                  "properties": {
                    "required":   { "type": "boolean" },
                    "granted_by": { "type": ["string", "null"], "description": "Human identity; must be non-null when required=true and the action proceeded." },
                    "granted_at": { "type": ["string", "null"], "format": "date-time" }
                  } },
    "budget_impact": { "type": ["object", "null"],
                  "properties": {
                    "usd":    { "type": ["number", "null"] },
                    "tokens": { "type": ["integer", "null"] },
                    "budget_line": { "type": ["string", "null"], "description": "FK -> COST_BUDGET.yaml." }
                  } },
    "prev_entry_hash": { "type": "string", "pattern": "^sha256:",
                  "description": "Hash of the previous entry's canonical serialization (genesis: sha256 of empty string). Makes the log tamper-evident." },
    "entry_hash":      { "type": "string", "pattern": "^sha256:",
                  "description": "Hash of this entry's canonical serialization excluding entry_hash itself." }
  }
}
```

Audit invariants: append-only; chain verification is a scheduled integrity job;
the system may not autonomously alter audit configuration (SPEC governance
rule 9 / Phase 11 restrictions).

---

## 8. `state_machine`

States exactly as SPEC Phase 4, in pipeline order:

`DISCOVERED → NORMALIZED → DEDUPLICATED → VERIFIED → SCORED → PROJECT_MAPPED →
EXPERIMENT_CANDIDATE → EXPERIMENTED → ACCEPTED | REJECTED → PR_READY →
HUMAN_APPROVED → DEPLOYED → MONITORED → RETAINED | ROLLED_BACK`

### Legal transitions and trigger authority

| # | From | To | Trigger | Allowed actor(s) | Gate conditions |
|---|------|----|---------|------------------|-----------------|
| 1 | (none) | DISCOVERED | New item arrives from an enabled source (webhook/poll) | control_plane ingestion job | Source row `enabled: true`; raw bytes stored; `content_hash` computed |
| 2 | DISCOVERED | NORMALIZED | Deterministic parse/extract completes | control_plane | Extractor version recorded; `normalized_hash` computed |
| 3 | NORMALIZED | DEDUPLICATED | Hash + near-dup comparison completes | control_plane | Duplicate ⇒ `duplicate_of` set, item PARKS here permanently |
| 4 | DEDUPLICATED | VERIFIED | Verification checks pass | control_plane deterministic checks; agent_role (scout/critic) cross-source checks | Source authenticity confirmed; claims extracted; topic evidence bar evaluated and recorded |
| 5 | VERIFIED | SCORED | Scoring stage completes | agent_role (model-assisted) + control_plane records | `scoring_provenance` populated; scores are advisory inputs, not authority |
| 6 | SCORED | PROJECT_MAPPED | Mapping stage completes | agent_role claude_architect_critic + control_plane | `project_mapping` records written; `exclusion_check` ran for every candidate project |
| 7 | PROJECT_MAPPED | EXPERIMENT_CANDIDATE | Architect recommendation accepted by control plane | control_plane (on claude_architect_critic review_object) | Evidence bar met; project autonomy level permits experiments; budget headroom |
| 8 | EXPERIMENT_CANDIDATE | EXPERIMENTED | Experiment executes in isolation | control_plane schedules; codex_implementer_verifier executes | `experiment_manifest` frozen; isolation active (SPEC rule 6); baseline captured |
| 9 | EXPERIMENTED | ACCEPTED | Thresholds met | control_plane (deterministic threshold evaluation) | Verdict computed from manifest thresholds + results; independent review_object(s) attached; no agent self-certification |
| 10 | EXPERIMENTED | REJECTED | Thresholds not met, or reviewer reject | control_plane; human | Rejection rationale + evidence logged (Phase 5 layer 7 rejection log). Terminal — re-entry only as a NEW evidence record |
| 11 | ACCEPTED | PR_READY | Draft PR created in authorized branch | agent_role codex_implementer_verifier | Draft-only; clean diff + manifest attached; protected branches untouched (SPEC rules 5, 8) |
| 12 | PR_READY | HUMAN_APPROVED | Explicit human approval | **human only** | No agent, job, schedule, or content may trigger (SPEC rule 7) |
| 13 | HUMAN_APPROVED | DEPLOYED | Human-initiated (or human-approved runbook) deploy | human; control_plane executing a human-approved deploy plan | Feature flag / canary where applicable; rollback plan on file |
| 14 | DEPLOYED | MONITORED | Monitoring window opens | control_plane | Agreed metrics + thresholds registered before window starts |
| 15 | MONITORED | RETAINED | Window closes, thresholds held | control_plane | Post-deploy metrics recorded to baseline registry |
| 16 | MONITORED | ROLLED_BACK | Post-deploy threshold violation | control_plane (auto-executes the RECORDED rollback plan); human any time | Rollback execution logged; incident entry in decision_log (SPEC Phase 8) |

### Invariants

1. **External content may NEVER trigger transitions past VERIFIED.** The
   arrival and deterministic processing of external content drives at most
   rows 1–4 (through VERIFIED). Every transition from VERIFIED onward requires
   an internal actor from the table — control plane, a designated agent role,
   or a human. Nothing embedded in ingested content (text, code comments,
   metadata, filenames, commit messages) has transition authority anywhere,
   at any state — content is data, never instructions (SPEC governance rule 2,
   Phase 9).
2. **HUMAN_APPROVED and beyond are human-gated.** Row 12 is human-only; row 13
   executes only a human-approved plan; the sole autonomous action after
   deployment is executing the pre-recorded rollback plan (row 16), which is a
   safety reversion, not a promotion.
3. **No skipping.** Only the pairs in the table are legal. Any other (from, to)
   pair is rejected by the control plane and raises an `integrity_alert`
   decision-log entry.
4. **Every transition writes exactly one decision_log entry** with
   `prior_state`, `new_state`, actor, rationale, and the evidence/review ids
   it relied on.
5. **Recommendations are not transitions.** `recommended_next_state` in a
   review_object is advisory input to the control plane; the review itself
   changes nothing.
6. **Terminal states:** REJECTED, RETAINED, ROLLED_BACK (and permanent parking
   at DEDUPLICATED for duplicates). New evidence about the same technique
   starts a new record that references the old one — history is never rewritten.
7. **Stale records freeze.** Past `review_by`, a record may not advance until
   re-reviewed (transition attempts log and halt).
8. **Model summaries have no state effects.** Adding, editing, or deleting
   `annotations[]` never changes `state` — annotations are not evidence
   (global provenance rule 1).
