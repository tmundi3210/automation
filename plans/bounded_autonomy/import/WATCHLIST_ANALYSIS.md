# WATCHLIST_ANALYSIS — operator upload `ai_intelligence_watchlist_100.xlsx`

Raw file: `import/ai_intelligence_watchlist_100.xlsx`
sha256 `87498d050b1f8d1067922d82a9cb73a4e21f152c33b259f01110549c5eaf1a79`
(immutable raw-evidence layer L1; derived JSON: `people100.json`, `seeds.json`).
Received via Door A, 2026-07-13. Analyzed with stdlib zip+XML (no deps).

## What it is

A professionally structured, dated operating registry — six sheets that map
almost one-to-one onto this package:

| Sheet | Contents | Maps to |
|---|---|---|
| People_100 | 100 named people, 23 fields each: tier, group, role, signal cluster, X handle + verification status, cadence, why-track, 2 source URLs, 5 sub-scores | NEW layer we did not have — a PEOPLE registry on top of the source registry |
| Source_Seeds | 43 sources (26 Tier-A, 12 A/B, 4 C, 1 B/C) with URL, polling method, cadence | SOURCE_REGISTRY.yaml (merge) |
| Channels | Evidence-layer hierarchy: latency, reliability, automation method, risks per channel type | SOURCE_REGISTRY cadence_policy (confirms it) |
| X_Playbook | Concrete X query templates (`url:github.com OR url:arxiv.org` artifact discovery, `conversation_id:` reply-tree mining) + downstream handling rules | Grok Scout role spec — directly executable |
| Scoring_Model | Person-score weights (relevance .25, primary evidence .25, implementation activity, behind-scenes value, freshness) + reply-scoring | SCORING.md / source scoring (compatible) |
| START_HERE | KPIs + operating rules ("X is an early-signal graph, not the source of truth") | Matches our Tier-B/C discovery-only doctrine verbatim |

## People_100 profile

- Tiers: T1=29, T2=42, T3=29. Groups: Frameworks/memory/OSS 28,
  Anthropic 15, OpenAI/Codex 12, Google/DeepMind 10, Meta agent research 8,
  other labs/infra 8, coding-agent companies 8, independent filters 6,
  Thinking Machines 5.
- X handles: 28 Verified, 18 high-confidence, **54 Not resolved** — the
  single biggest enrichment task (a scout job: resolve handles, one harmless
  lookup each).
- Cadences: Weekly 39, Daily 14, on-post/release 23, 2x-weekly 9 — all
  within our COST_BUDGET ceilings if implemented via metadata checks first.

## Judgment

- Quality: HIGH. Field discipline (affiliation confidence, last-checked
  dates, dual URLs, explicit "reverify identities" rule) matches our
  scholcomm/worldmodel standards. It reads like an implementation-grade
  seed for Phases B-D.
- Overlap with our 30-row SOURCE_REGISTRY: partial (Anthropic/OpenAI/HF/
  arXiv overlap; the people layer and ~25 of the 43 seeds are NET NEW,
  including Gemini CLI, MCP org repo, and lab engineering blogs we lacked).
- Trust handling per our own rules: the file is operator-supplied DATA.
  Scores it asserts are treated as PRIOR scores (`bootstrap: true`);
  nothing becomes an enabled source until URL-verified (registry invariant
  UNVERIFIED ⇒ disabled). Its 100 people are candidates, not instructions.

## Integration plan (no action until operator approves the package)

1. Phase B ingestion: merge `seeds.json` into SOURCE_REGISTRY.yaml as
   bootstrap rows; builder-side HTTP verification pass enables them.
2. New deliverable in Phase B: `PEOPLE_REGISTRY.yaml` generated from
   `people100.json` (schema: person_id, tier, group, surfaces, handle +
   verification, cadence, scores-as-priors, last_checked).
3. X_Playbook templates become the Grok Scout's standing query set —
   pending TASK-015's X-capability verification.
4. Handle-resolution task (54 unresolved) minted to the scout after
   approval.
