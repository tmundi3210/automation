# TOP_SOURCES — best news sources for AI-as-productivity-SYSTEM intelligence

Answers the operator question: *"from the 100 list, what are the best news
sources?"* Scope = agentic/multi-agent systems, coding agents, orchestration,
tool/MCP ecosystems — NOT generic chatbot/model news.

Inputs: `import/seeds.json` (43 sources), `import/people100.json` (100 people),
`import/WATCHLIST_ANALYSIS.md`, `SOURCE_REGISTRY.yaml` (30 existing rows).
Produced 2026-07-14 under the scholcomm (provenance/source-evaluation) and
worldmodel (source authority/bias mapping) specialist procedures.
Second-pass review (agenteval advisor, 2026-07-14): APPROVE_WITH_CHANGES —
all findings revised in place.

**Trust rule (applies to every score cited below):** all Tier, Priority-score
and sub-score values from the operator xlsx are operator-supplied PRIORS
(`bootstrap: true`) — heuristic evidence labels, not verified facts. Their
numeric precision does not upgrade them. No URL was fetched in this session
(egress policy-blocked): every pick is status=CANDIDATE, and the registry
invariant holds — **UNVERIFIED ⇒ disabled** until a builder URL-verifies it.

## 1. Selection criteria (advisor-derived, weights explicit)

| # | Criterion | Weight | Rule applied |
|---|---|---|---|
| C1 | Provenance class | 0.30 | scholcomm source-typing first: official/primary (lab engineering blog, changelog, spec, release feed) = 1.0 > primary-repo (the project's own GitHub repo: releases/PRs/issues) = 0.9 > practitioner-primary (named builder's own blog/repo) = 0.7 > aggregator/synthesis (newsletter, curation page) = 0.3 |
| C2 | Signal specificity | 0.30 | Topics/signal-cluster must map to agentic-SYSTEM topics (agent loops, coding agents, orchestration, tool_use_mcp, agent memory/evals). Generic model or chatbot news scores low |
| C3 | Verifiability of URL | 0.15 | URL verbatim from operator data, machine-pollable surface (RSS/Atom/API/diffable page). Nothing invented; an unverifiable source is excluded and *said to be excluded* — never silently kept |
| C4 | Cadence fit vs COST_BUDGET | 0.15 | Must be pollable by zero-cost change gates (feed metadata / hash diff) inside COST_BUDGET.yaml invocation ceilings; hourly repo feeds OK via Atom, site-diff surfaces cost one daily scrape |
| C5 | Independence / bias | 0.10 | worldmodel authority-and-bias mapping: vendor incentive is FLAGGED, never auto-disqualifying (accuracy dominates stance); the portfolio as a whole must span ≥3 independent orgs to avoid single-narrative capture |

Ordering: the top SET is forced by C1+C2 (official/primary provenance ×
agentic-system specificity); the ORDER within near-ties (e.g. ranks 3–6,
ranks 11–12) is editorial judgment over C1 then C2, recorded here — it is
NOT mechanically derivable, since per-pick C1–C5 subscores were not computed
(reviewer finding, accepted). Follower counts, stars and subscriber counts
are NOT inputs (SOURCE_REGISTRY ranking_rule verbatim: demonstrated value,
not follower count).

## 2. TOP SOURCES — ranked shortlist (all status=CANDIDATE, disabled until URL-verified)

From the 43 seeds (ranks 1–12) plus two people100 "Best monitoring surface"
entries that are themselves followable news surfaces (ranks 13–14).

| Rank | Source | URL (verbatim) | Seed tier | Provenance class | Why top for THIS system | Registry overlap |
|---|---|---|---|---|---|---|
| 1 | Anthropic Engineering | https://www.anthropic.com/engineering | A/B | official-primary | Canonical context-engineering / tool-design / MCP-code-execution write-ups; feeds 7 of the T1 people directly (field-derived count: 7 by URL match — the full T1 Anthropic cohort) | **YES — `anthropic_engineering` (enabled)**; do not double-count |
| 2 | OpenAI Engineering & Research | https://openai.com/research/ | A | official-primary | Codex agent-loop, harness and compaction posts; the posts cited by people seeds 1–5 live at openai.com/index/… — /research/ is the seeds.json-verbatim hub, and URL-verification must confirm it surfaces them | Partial — registry has `openai_codex_changelog` only; the posts hub is NET NEW |
| 3 | Model Context Protocol | https://github.com/modelcontextprotocol | A | official-spec/primary | Protocol-level truth for tool interop, permissions, schemas — the orchestration backbone this system itself runs on | NO — NET NEW (WATCHLIST_ANALYSIS: "MCP org repo … we lacked") |
| 4 | OpenAI Codex GitHub | https://github.com/openai/codex | A | primary-repo | Harness, sandbox, compaction, app-server changes land here before any write-up | Partial — `openai_codex_changelog` feed points at this repo's releases.atom; repo activity surface is new |
| 5 | Gemini CLI GitHub | https://github.com/google-gemini/gemini-cli | A | primary-repo | Coding-agent hooks/extensions/lifecycle events (T1 builders Palencia, Patel commit here) | NO — NET NEW |
| 6 | Google ADK Python | https://github.com/google/adk-python | A | primary-repo | Agent-development-kit orchestration and protocol patterns from a third independent org | NO — NET NEW |
| 7 | Google Developers Blog | https://developers.googleblog.com/ | A/B | official-primary | Announcement layer for Gemini CLI/ADK hooks and extensions; RSS-pollable | NO — NET NEW |
| 8 | PydanticAI | https://github.com/pydantic/pydantic-ai | A | primary-repo | Typed agents + evals + MCP + observability in one non-lab, high-discipline codebase | NO — NET NEW |
| 9 | LangGraph | https://github.com/langchain-ai/langgraph | A | primary-repo | Stateful/durable agent execution releases; highest-traffic orchestration framework | **YES — `gh_releases_langgraph` (disabled, UNVERIFIED)**; merge, don't re-add |
| 10 | OpenHands | https://github.com/All-Hands-AI/OpenHands | A | primary-repo | Open coding-agent + benchmark releases; independent of the big-3 labs | **YES — `gh_releases_openhands` (disabled, UNVERIFIED)** |
| 11 | Cursor Changelog | https://www.cursor.com/changelog | A/B | official-primary (changelog) | Fastest-moving commercial coding-agent product surface; changelog = dated primary record | NO — NET NEW |
| 12 | Thinking Machines Connectionism | https://thinkingmachines.ai/blog/ | A/B | official-primary | Inference determinism / post-training posts that directly affect agent-eval reproducibility | NO — NET NEW |
| 13 | steipete.me (Peter Steinberger) | https://steipete.me/posts/2026/openclaw | T1 person, prior 100 | practitioner-primary | Local/persistent agent architecture from the OpenClaw creator; complements registry `openclaw_case_study` row. NOTE: this verbatim URL is a single dated post, not a pollable feed — the builder verification pass must resolve the blog root/feed, keeping this URL as the evidence anchor | Partial — OpenClaw repo already a (disabled) case-study row |
| 14 | philschmid.de (Philipp Schmid) | https://www.philschmid.de/ | T2 person, prior 96 | practitioner-primary | Reproducible implementation-first agent/deployment posts; verified identity (@_philschmid) | NO — NET NEW |

Portfolio independence check (C5): 14 picks span OpenAI, Anthropic, Google,
Pydantic, LangChain, All-Hands, Cursor, Thinking Machines + 2 independents —
no single vendor narrative dominates. Vendor bias_flags must be carried onto
every merged row exactly as SOURCE_REGISTRY.yaml already does.

Deliberately NOT ranked despite Tier-A seeds: AutoGen and Hugging Face Papers
(both already registry rows), arXiv cs.AI/cs.CL/cs.LG recent (preprint
firehoses — discovery lanes, not news sources; and preprints are never cited
as validated), Meta AI Research (on-scope but paper-cadence; route via the
research lane in Phase B rather than the news shortlist).

## 3. TOP PEOPLE (news-bearing surfaces)

Top picks by operator Priority prior, filtered to people whose "Best
monitoring surface" behaves like a news feed (posts/releases/PRs) rather than
occasional papers. Priors are operator-supplied, unverified.
Tiebreaker rule (stated explicitly, applied symmetrically — reviewer
finding, accepted): a person with an UNRESOLVED X handle is listed only when
their news-bearing surface is itself a ranked §2 source — you follow the
surface, not the person, until identity is verified. This admits both
David Soria Parra (surface = MCP org, rank 3) and Michael Bolin (surface =
Codex GitHub, rank 4) on identical terms.

| Rank | Person | Prior | Tier | Group | News-bearing surface | X handle | Handle status |
|---|---|---|---|---|---|---|---|
| 1 | Peter Steinberger | 100 | T1 | OpenAI / OpenClaw | Personal blog + OpenClaw releases/PRs | @steipete | Verified (per xlsx) |
| 2 | Omar Khattab | 100 | T1 | Frameworks/OSS | DSPy roadmap/releases | @lateinteraction | Verified (per xlsx) |
| 3 | Teknium | 100 | T1 | Frameworks/OSS | Hermes Agent releases, GitHub | @Teknium1 | Verified (per xlsx) |
| 4 | Xingyao Wang | 100 | T1 | Frameworks/OSS | OpenHands PRs/releases | @xingyaow_ | High-confidence |
| 5 | Deshraj Yadav | 100 | T1 | Frameworks/OSS | Mem0 PRs/releases | @deshraj | High-confidence |
| 6 | Marcelo Trylesinski | 100 | T1 | Frameworks/OSS | PRs/issues across PydanticAI + MCP | @Kludex | High-confidence |
| 7 | Jason Liu | 100 | T1 | OpenAI / Codex | Personal technical posts, whitepapers | @jxnlco | High-confidence |
| 8 | David Soria Parra | 100 | T1 | Anthropic MCP | MCP repos, issues, PR reviews (ranked surface: §2 rank 3) | — | **Not resolved** — follow the repo surface, not the person, until identity verified |
| 9 | Michael Bolin | 100 | T1 | OpenAI / Codex | Codex GitHub issues/PRs, OpenAI engineering posts (ranked surface: §2 rank 4) | — | **Not resolved** — same surface-only rule as rank 8 |

**Identity escalation (mandatory):** 54/100 handles are "Not resolved" and
even the 28 "Verified" + 18 "high-confidence" labels are xlsx claims this
session could not laterally verify. Per the worldmodel gate ("a claim lacking
a resolvable vetted source cannot be finalized") and the scholcomm lateral-
reading rule, **identity re-verification is required before following anyone**
— impersonation of high-profile builders is a known attack surface
(THREAT_MODEL boundary: ingested content is untrusted).

## 4. What NOT to follow yet

| Category | Examples | Reason (one line) |
|---|---|---|
| Unresolved-handle X accounts (54/100) | Ken Aizawa, Chi Wang, Charles Packer | Identity unvetted ⇒ cannot be finalized as a source; scout resolution first. (Bolin and Soria Parra appear in §3 via the ranked-surface rule only — their X accounts remain unfollowable) |
| Any X account as a *polled* source | all 46 resolved handles too | SOURCE_REGISTRY cadence amendment (TASK-015): X is session-tools only, not a pollable feed; scout-tick advisory only |
| Tier-C synthesis/newsletters | Latent Space, Import AI, The Batch, Interconnects | Already registry rows, aggregator provenance — discovery-only doctrine; re-adding double-counts |
| Duplicates of existing registry rows | Simon Willison, HF Papers, arXiv cs.CL, AutoGen | Already registered (some enabled); Phase-B merge dedupes, never re-adds |
| Broad arXiv firehoses | cs.AI recent, cs.LG recent | Preprint discovery, not news; volume breaks cadence budget; registry keeps fielded cs.MA/cs.CL/cs.SE instead |
| Off-scope lab feeds | NVIDIA GEAR (robotics), Mistral News, Cohere Research | Model/robotics releases, not agentic productivity systems |
| Paper-cadence research groups | Meta ARE/AIRA2 authors (seeds 38–45) | High value but publication-driven; belongs in the research lane, not the news shortlist |

## 5. Next-step hooks — ALL PENDING operator approval of the bounded_autonomy package

- **Builder URL-verification pass** over all 14 CANDIDATE rows (HTTP check +
  feed-endpoint discovery); invariant UNVERIFIED ⇒ `enabled: false` stands.
- **Scout handle-resolution task**: resolve the 54 missing handles and
  re-verify the 46 claimed ones (one harmless lookup each) before any follow
  list exists.
- **Phase-B merge**: seeds → SOURCE_REGISTRY.yaml as bootstrap rows (dedupe
  against the 30 existing; 6 overlapping picks touching 5 existing registry
  rows, flagged per-row in §2); people100 → `PEOPLE_REGISTRY.yaml` per
  WATCHLIST_ANALYSIS integration plan.
- **Affiliation lateral-read** (scout): at least two people100 affiliation
  claims (Karpathy→Anthropic, Shazeer→OpenAI) differ from this model's
  last independently known state; treat all affiliation fields as operator
  claims pending verification. No action implied now.

## SPECIALIST-COMPLIANCE

- **scholcomm / source_typing_and_provenance_chain:** every pick typed
  (official-primary / primary-repo / practitioner-primary / aggregator)
  before ranking; URLs verbatim from operator data, none invented.
- **scholcomm / evidence_label dominance:** xlsx tiers and Priority scores
  used only as labeled heuristic PRIORS; precise numbers did not upgrade
  any label; ranking rests on structural fields.
- **scholcomm / venue-vet-before-trust:** nothing enabled here — all picks
  CANDIDATE + disabled until builder URL verification (UNVERIFIED ⇒ disabled).
- **scholcomm / preprint rule:** arXiv listings classified discovery-only;
  no preprint surface presented as validated news.
- **worldmodel / source_authority_and_bias_mapping:** vendor incentive
  assessed at PORTFOLIO level (§2 independence check paragraph); per-pick
  bias_flags are deferred to the Phase-B merge, where every row carries
  bias_flags exactly as SOURCE_REGISTRY.yaml already does. Stance never
  auto-disqualified; portfolio spans ≥3 independent orgs.
- **worldmodel / no-unvetted-source-finalized (escalation honored):** all
  unresolved-handle accounts excluded and listed explicitly; identity
  re-verification named as a blocking gate, not silently skipped.
- **worldmodel / demonstrated value over popularity:** followers/stars
  excluded from criteria; C1–C5 weights documented. The top SET reproduces
  from C1+C2; within-set ordering is recorded editorial judgment, not a
  mechanical derivation (per §1 ordering note).
- **both / no unverifiable source silently kept:** every exclusion appears
  in section 4 with a stated reason; overlaps with SOURCE_REGISTRY flagged
  row-by-row so nothing is double-counted.
