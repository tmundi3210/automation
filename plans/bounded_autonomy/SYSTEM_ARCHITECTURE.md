# SYSTEM_ARCHITECTURE — Bounded-Autonomy AI Intelligence System

Status: PLANNING (SPEC deliverable 5). No code is written in this phase; the
skeleton in §9 is documented only. Requirements source:
`plans/bounded_autonomy/SPEC.md`. Substrate of record: EXCHANGE protocol v3.3
(`EXCHANGE/ORCHESTRATION.md` is the constitution; this document EXTENDS it and
never contradicts it — where they appear to differ, ORCHESTRATION.md wins until
the hub bumps PROTOCOL-VERSION).

Verified ground truth this design builds on (not assumptions):

| Component | Where it runs | Interface | Scheduling | Verified capabilities |
|---|---|---|---|---|
| Claude Code | remote container | CLI + subagents | hourly Routine cron | hub/integrator/architect/critic; GitHub MCP scoped to `tmundi3210/automation`; sole writer of shared state |
| Grok CLI | operator Mac | CLI, launchd | 5-min poll behind zero-token `gate.sh` (POLLING.md) | web + X search; scout; cheap high-volume agentic runs |
| Codex CLI | operator Mac | `codex exec --sandbox workspace-write` | gated cron | implementer/verifier; terminal/coding builder |

Existing assets that migrate into (or are wrapped by) this architecture:
`tools/gate_all.sh`, `tools/check_scope.sh`, `tools/status.py`,
`validators/` (kb_validator, specialist_validator, metrics), the EXCHANGE
protocol docs, `dist/specialist_prompts.jsonl` (31 specialists),
`specialists/ROUTER.json`, `EXCHANGE/tasks.json` / `AGENTS.json` /
`scores.json` ledgers. See §9 migration map.

---

## 1. Deterministic control plane (SPEC governance #1)

The control plane manages schedules, state, permissions, budgets, logs, and
approval rules. It is deterministic end to end: shell + python3 stdlib + git.
A model NEVER decides when something runs, what it may spend, or whether an
approval rule applies — models only produce work products that the control
plane then gates.

### 1.1 State store: git-tracked versioned files (canonical) + derived local SQLite (cache)

**Recommendation: canonical control-plane and pipeline state lives in
git-tracked, append-oriented JSON/JSONL/YAML files in this repo, written by a
single writer (the hub). A local SQLite database is a DERIVED, rebuildable
cache for relational queries — gitignored, never authoritative, drift-checked
against the files exactly the way `specialists/ROUTER.json` and
`INDEX.json` are drift-checked today (gate 3 in `tools/gate_all.sh --repo`:
deterministic rebuild + byte compare).**

Justification:

1. **It already works.** `EXCHANGE/tasks.json`, `AGENTS.json`, `scores.json`
   are exactly this pattern, running in production across three heterogeneous
   agents on two machines. The repo is the only substrate all three components
   verifiably share; any external DB adds a second source of truth, a network
   dependency for the Mac pollers, and a new credential (violates least
   privilege, SPEC governance #4, and enlarges the secret-exposure surface of
   Phase 9).
2. **Audit for free.** `git log -p` on a ledger IS the complete audit log SPEC
   Phase 10 requires; single-writer + append-only makes every state transition
   attributable to a commit, a task id, and a verdict message (ORCHESTRATION
   §10). An external DB would need audit infrastructure built from scratch.
3. **Reviewability.** The hub deep-reads every delivery. Text ledgers diff;
   a committed SQLite binary does not — the hub cannot content-review a binary,
   merge conflicts are opaque, and byte-identical rebuild across SQLite
   versions is fragile. So SQLite-in-git is rejected as canonical; SQLite as
   a local derived cache keeps the relational query power without any of that.
4. **Fail-safe offline.** The zero-token idle gate must work when the network
   is degraded (POLLING §1 "network down → idle"). File state in
   `$HOME/.exchange-gate/` and in the clone requires no service to be up.

Concrete layout (documented here, created at implementation time):

- Canonical: `orchestrator/state/*.jsonl|yaml` (evidence, claims, candidates,
  experiments, decisions — schemas in DATA_SCHEMAS.md, deliverable 6) plus the
  existing `EXCHANGE/tasks.json` family, all single-writer hub.
- Derived: `var/orchestrator.db` (gitignored) rebuilt by a deterministic
  loader; a gate compares row counts + content hashes against the ledgers
  before any query result is trusted.
- Per-machine runtime state (never in repo): `$HOME/.exchange-gate/` (exists
  today), `$HOME/.agent-state/last_seen` (exists today).

### 1.2 Schedules — launchd/cron/Routine only, never model-decided

| Trigger | Owner | Mechanism | What it does |
|---|---|---|---|
| 5-min tick | Grok poller (Mac, dedicated OS user) | launchd + `gate.sh` (POLLING §1) | zero-token idle check; wakes model only on real change |
| gated cron | Codex poller (Mac, dedicated OS user) | cron + same two-stage gate | same |
| hourly | Claude Code | Routine cron | hub round: intake inbox, verdicts, integration, digest jobs |
| event-driven | adapters (planned) | webhooks/release feeds where possible (SPEC Phase 3) | mark source dirty; NO model call |
| 1–3 h | adapters (planned) | metadata-only checks of high-priority sources | hash compare; model invoked only on material change (SPEC Phase 3 cadence) |
| daily / weekly / monthly | Routine cron (planned entries) | project digest / synthesis / source-pruning review | mint EXCHANGE tasks |

Cadence changes are a control-plane edit (schedule manifests under
`orchestrator/config/`), which is on the human-gate list. A model may PROPOSE
a cadence change (Phase 11 bounded self-evolution) with before/after evidence;
it cannot apply one.

### 1.3 Budgets and approvals — enforced BEFORE any model call

- `orchestrator/config/COST_BUDGET.yaml` (deliverable 12) holds daily/monthly
  ceilings per model and per source. The poller wrapper (the same shell layer
  that runs `gate.sh`) reads a local spend ledger and refuses to invoke the
  model when a ceiling is hit — shell arithmetic, exit 1, alert file written.
  A model that is never invoked cannot overspend; this is the CaMeL-style
  "enforce outside the model" layer ORCHESTRATION §9b already mandates.
- Approval rules are data the control plane enforces: the acceptance token
  (`VERDICT: ACCEPTED TASK-NNN @<agent> <sha>`), the human-gate list
  (ORCHESTRATION §9), and SPEC governance #5/#7 (no protected-branch writes,
  no merge/deploy without explicit user approval) are checked by
  `check_scope.sh`, branch deny-rules, and the hub checklist — not by prompt
  text.
- SPEC governance #9: budgets, approval rules, security policy, and audit
  config live in hub-owned paths that no task scope may include; the mint-time
  disjointness gate (`gate_all.sh --repo`, gate 4) plus HARD DENIALS make a
  task that requests them invalid by construction.

## 2. Component diagram

```
                    OPERATOR (human — final authority; Doors A/B/C of INTAKE.md)
                        │ approvals: merges to main, deploys, budget/policy edits,
                        │ HUMAN_APPROVED state, STAND DOWN
                        ▼
   ┌─────────────────────────────────────────────────────────────────────────┐
   │ GITHUB REPO tmundi3210/automation  = message bus + state store + audit  │
   │   EXCHANGE/ (msgs, tasks.json, scores.json)   orchestrator/state/ (new) │
   │   memory layers (MEMORY_ARCHITECTURE.md)      tools/ validators/ dist/  │
   │   integration branch: claude/eager-wozniak-74rlgj (hub-only writes)     │
   └───────▲──────────────────────▲───────────────────────────▲──────────────┘
           │ fetch/push           │ fetch/push                │ fetch/push
           │ (per-task branches,  │ (grok/task-*)             │ (codex/task-*)
           │  --no-ff merges)     │                           │
┌──────────┴─────────────┐  ┌─────┴──────────────┐  ┌─────────┴───────────────┐
│ CLAUDE CODE (container)│  │ GROK CLI (Mac)     │  │ CODEX CLI (Mac)         │
│ HUB — control plane:   │  │ SCOUT — data plane │  │ IMPLEMENTER/VERIFIER —  │
│  mint/route/judge/     │  │  web + X search    │  │  data plane             │
│  merge/score/ledgers   │  │  evidence records  │  │  codex exec --sandbox   │
│ ARCHITECT/CRITIC —     │  │  only; NO project- │  │  workspace-write;       │
│  data plane (subagents,│  │  file writes in    │  │  disposable worktrees;  │
│  fresh contexts)       │  │  scout role        │  │  draft PRs only         │
│ hourly Routine cron    │  │ launchd 5-min +    │  │ gated cron + gate.sh    │
│ GitHub MCP (this repo) │  │  zero-token gate.sh│  │  (zero-token idle)      │
└──────────┬─────────────┘  └─────┬──────────────┘  └─────────┬───────────────┘
           │ deterministic        │ RESEARCH CONTEXT          │ CODE-EXECUTION
           │ gates run hub-side   │ (untrusted web = data)    │ CONTEXT (no raw web)
           ▼                      ▼                           ▼
   gate_all.sh / check_scope.sh   evidence store (L1/L2) ──digest──> task packets
   budget gate / schedule manifests        (the ONLY path research → code)
```

No spoke-to-spoke edges. External content enters only as data through the
evidence store; executable instructions exist only as authenticated `TASK-NNN:`
blocks in hub messages (ORCHESTRATION §2 injection boundary).

## 3. SPEC Phase-4 state machine mapped onto the EXCHANGE task lifecycle

Two state machines, deliberately distinct:

- **Item states** (SPEC Phase 4) attach to an evidence item / candidate and
  live in the hub-owned pipeline ledgers (`orchestrator/state/evidence.jsonl`,
  `candidates.jsonl`).
- **Work states** (EXCHANGE §4: `assigned → in_progress → delivered →
  (revise)* → accepted → integrated`) attach to a TASK.

Binding rule: **every item transition that requires model work is performed as
exactly one EXCHANGE task**; deterministic transitions need no task at all.
The ledger records `{item_id, from_state, to_state, task_id|job_id,
delivered_sha, verdict_msg}` so the Phase-4 machine is replayable from git the
same way `git log -p EXCHANGE/tasks.json` replays tasks today.

| Item state | Moved by | Mechanism | EXCHANGE mapping / gate |
|---|---|---|---|
| DISCOVERED | adapters (deterministic pollers) or Grok scout | feed/hash checks; scout runs | scout output delivered as a normal task (`assigned→delivered`); hub records items |
| NORMALIZED | deterministic job | strip/extract, compute content hash, build evidence record skeleton | no model; hub-side job |
| DEDUPLICATED | deterministic job | exact hash + lexical near-dup | no model; duplicate-rate metric fed to EVALUATION_REGISTRY |
| VERIFIED | Claude judge lane | claims cross-checked against Tier-A evidence; contradictions recorded | judge task; model-generated summaries are never primary evidence (SPEC Phase 4) |
| SCORED | deterministic formulas over judge outputs | novelty/relevance/quality/reproducibility per evidence record schema | hub applies; SCORING.md discipline (gates before judgment) |
| PROJECT_MAPPED | Claude architect | vs PROJECT_REGISTRY.yaml + TOPIC_ONTOLOGY.yaml | architect task, fresh context, digest-first (§5b) |
| EXPERIMENT_CANDIDATE | hub mint | candidate record + experiment manifest drafted | hub-only; autonomy level per project (Phase 1 registry) can require operator pre-approval here |
| EXPERIMENTED | Codex implementer | disposable worktree, pinned versions, thresholds from EVALUATION_REGISTRY.yaml | full task lifecycle incl. gates; SIZE/RISK per ORCHESTRATION §4 |
| ACCEPTED / REJECTED | hub verdict | deterministic thresholds first, then judge; REJECTED items go to the rejection log (memory L7) with reason + review date | the literal `VERDICT:` line is the only acceptance token |
| PR_READY | Codex | **draft** PR via GitHub (governance #8) | task deliverable; never merged by any agent |
| HUMAN_APPROVED | **operator only** | PR review/approval | never automated (governance #7); no task can produce this state |
| DEPLOYED | operator (system may prepare scripts/flags) | per-project deployment method | out of agent scope (governance #5) |
| MONITORED | scheduled eval runs | Routine cron runs registry evals against agreed metrics | canary entries in EVALUATION_REGISTRY.yaml (status: planned) |
| RETAINED / ROLLED_BACK | threshold gate decides; rollback executes the pre-recorded plan | rollback of production is human-gated; experiment-branch rollback is mechanical | closes the loop; outcome updates scores + episodic memory |

Any pre-integrated work state can go to `abandoned` (EXCHANGE) without killing
the item — the item re-enters at its last stable state and can be re-tasked
(this is the existing TASK-008→TASK-009 reassignment pattern).

## 4. Role architecture (SPEC Phase 7) and rotation rule

Roles are the SPEC's three, implemented as EXCHANGE lanes with the v3.3
evidence-based routing table (ORCHESTRATION §1) and trust-score override
(SCORING §2, observed trust dominates after n≥3 per lane):

| SPEC role | Agent | EXCHANGE lane(s) | Hard limits (enforced, not prompted) |
|---|---|---|---|
| Grok Scout | grok | research-curation, high-volume drafting/exploration | scout tasks carry write_scope = own EXCHANGE dir + evidence staging only; `check_scope.sh` makes project-file writes impossible in the scout role |
| Claude Architect/Critic | claude (data-plane subagents) | judging/fact-check, factual long-form, integration | architect/critic run as fresh-context subagents; the HUB control-plane role (mint/merge/gate authority) is singleton and never rotates |
| Codex Implementer/Verifier | codex | code build, mechanical verification (tests/CI/repros) | worktree/branch only; draft PRs only; never final-verifies own work (no self-certification) |

Every review returns the SPEC Phase-7 JSON object verbatim
(`decision/confidence/verified_claims/unverified_claims/concerns/
required_changes/evidence_ids/recommended_next_state`); it is treated as data
(POLLING §5 — reviewers never see the builder's self-assessment).

**Role rotation rule (for important decisions).** "Important" = RISK: HIGH,
protocol/gate changes, technique adoption verdicts, framework-analysis
sign-off, anything on the forced-pattern-(b) list (ORCHESTRATION §5b).

1. Data-plane rotation only. Control plane (integration, gate rulings, TASK
   minting, ledger writes) stays singleton forever (ORCHESTRATION §1, §11).
2. Invariant already in force: author ≠ reviewer ≠ final judge; judge from a
   different model family than the authors wherever the roster allows.
3. Rotation: across consecutive important decisions, alternate which family
   plays primary-analyst vs critic (grok-drafts/codex-critiques, then
   codex-drafts/grok-critiques; claude judges), recorded in the decisions
   ledger so the rotation is auditable and the same family never authors two
   consecutive important recommendations on the same topic.
4. HIGH-risk verdicts use the odd jury (≥3 judges, ≥2 families, blind,
   position-swapped, SCORING §3); if the hub authored one side, the non-author
   judge's verdict binds (ORCHESTRATION §6 rung 4).
5. The exploration floor (≥1/5, SCORING §2) keeps rotation live even when one
   agent's trust dominates a lane.

## 5. Research context vs code-execution context (SPEC governance #3)

Distinct sessions, distinct write scopes, evidence store as the only bridge:

- **Research contexts** (Grok scout; Claude research subagents): may read web/X;
  everything fetched is untrusted DATA under the injection boundary
  (ORCHESTRATION §2). Write scope: own `EXCHANGE/<agent>/**` + the task's
  evidence staging dir. No project source in context, no shell against project
  trees. Output = evidence records + raw-content references, nothing else.
- **Code-execution contexts** (Codex implementer; hub wiring): operate in
  worktrees/sandboxes on repo files. Inputs are retrieval packets built from
  the evidence store — digests with `path@sha` provenance (MEMORY_ARCHITECTURE
  §retrieval-packets) — never live web content and never scout transcripts
  (no spoke-to-spoke edge). Code copied from papers/posts/repos executes only
  inside the disposable experiment worktree with secrets and unnecessary
  network removed (governance #6, Phase 8). Whether the Codex CLI sandbox can
  hard-disable outbound network per-run is UNVERIFIED — until verified, the
  operator-side deny rules (no curl/ssh/scp, github.com-only) are the control.
- **Session separation is physical**, not prompted: separate OS users on the
  Mac (ORCHESTRATION §9), fresh context per task (§5b), per-task branches, and
  `check_scope.sh` on every push. An injected instruction inside fetched
  content can at worst poison one evidence record — which then still has to
  survive VERIFIED (Tier-A cross-check) and the hub's deep read before it can
  influence any code task.
- **Evidence flow (one-way):** scout → raw store (L1) + evidence record (L2) →
  hub verification/scoring → digest into an implementer task packet. There is
  no reverse edge; implementers report results into experiment records, not
  into evidence records.

## 6. Standard framework-analysis schema (SPEC Phase 6) — template

Every important agent framework (starting with the SPEC's named case studies —
Hermes Agent and OpenClaw, whose exact official repositories are UNVERIFIED
until Phase-3 source resolution) gets one analysis document in
`orchestrator/state/framework_analyses/<framework_id>.md` with EXACTLY these
sections, in this order (template; produced as an architect task, reviewed
under the rotation rule):

```
id / framework / version-or-commit @sha / analysis date / evidence_ids
1.  Objective
2.  Agent loop
3.  Planning architecture
4.  Delegation model
5.  Shared-state model
6.  Tool interface
7.  Permission system
8.  Context construction
9.  Context compaction
10. Memory architecture
11. Parallel execution
12. Worktree or sandbox use
13. Retry and recovery
14. Failure handling
15. Human approval
16. Observability
17. Evaluations
18. Cost controls
19. Security model
20. Reusable patterns            ← architectural learning
21. Incompatible or risky patterns
22. Projects to which the patterns may apply (project_ids from PROJECT_REGISTRY)
```

Sections 20–22 are the only ones allowed to make recommendations, and they
recommend PATTERNS, never direct code adoption — adoption requires the full
Phase-4 pipeline (candidate → experiment → thresholds → draft PR → human
approval). This keeps SPEC Phase 6's "separate architectural learning from
direct code adoption" structural rather than aspirational.

## 7. Failure/feedback posture (loops advisor)

- Every scheduled job is idempotent and keyed by run_id; retries use backoff;
  the DEGRADED circuit breaker + canary re-entry (ORCHESTRATION §8) is the
  agent-level breaker; budget ceilings are the spend-level breaker.
- The pipeline has no unbounded loops: revise cap 3, gate-fix cap 2 per task,
  ≤1 push per poll tick, lease/heartbeat reclaim — all existing, all
  deterministic.
- Goodhart resistance: self_report_gap term, hub re-judge canary of previously
  accepted artifacts, and gates that run only from hub copies. Reward-hacking
  is a measured threat (RESEARCH_DIGEST §B), not a hypothetical.

## 8. Proposed code skeleton (SPEC deliverable 15) — DOCUMENTED ONLY

`orchestrator/` exists today as an empty directory at the repo root. Nothing
below is created in this phase. Layout follows the SPEC's suggested skeleton;
annotations say what each dir will contain and which existing assets migrate.

```
orchestrator/
  config/            # PROJECT_REGISTRY.yaml, TOPIC_ONTOLOGY.yaml, SOURCE_REGISTRY.yaml,
                     #   AUTONOMY_POLICY.yaml, COST_BUDGET.yaml, schedule manifests
                     #   (launchd plists / crontab lines / Routine spec). EXCHANGE/*.md
                     #   protocol docs are REFERENCED, not moved — the constitution
                     #   stays in EXCHANGE/ (moving it would force a MAJOR re-arm).
  control_plane/     # state-machine runner over orchestrator/state/ ledgers; budget
                     #   pre-model gate; wake wrappers (generalizes POLLING.md gate.sh);
                     #   dead-letter queue dir; run/trace-id issuance.
  adapters/          # one dir per source/agent surface; all emit normalized
    claude_code/     #   evidence-record skeletons + raw refs, nothing else.
    codex/           #   claude_code/codex/grok = session-launch + task-packet I/O
    grok/            #   shims around the CLIs (they wrap, never replace, the CLIs).
    github/          #   repo/release/issue feeds via the scoped GitHub MCP.
    x/               #   X lists/search via Grok CLI (only component with X access).
    papers/          #   arXiv & co. metadata polling.
    rss/             #   newsletters/blogs release feeds.
    podcasts/        #   RSS + transcript fetch.
  ingestion/         # DISCOVERED→NORMALIZED: fetch, strip, hash, license capture.
  normalization/     # evidence-record construction; claim extraction prompts (data).
  verification/      # VERIFIED stage: Tier-A cross-check harness, contradiction capture.
  scoring/           # deterministic scoring formulas (novelty/relevance/quality/
                     #   reproducibility); SCORING.md trust math as code.
  project_mapping/   # topic-ontology matcher; PROJECT_MAPPED stage.
  memory/            # loaders/builders for the 8 layers (MEMORY_ARCHITECTURE.md);
                     #   migrate: tools/build_index.py, tools/build_router.py patterns;
                     #   dist/specialist_prompts.jsonl + specialists/ remain in place
                     #   as procedural memory (L6), referenced not moved.
  experiments/       # worktree/container lifecycle, version pinning, experiment
                     #   manifests, baseline capture (Phase 8).
  evaluations/       # eval runner reading EVALUATION_REGISTRY.yaml; migrate:
                     #   validators/kb_validator.py, validators/specialist_validator.py,
                     #   validators/metrics.py (wrapped, fingerprints preserved).
  security/          # migrate: tools/check_scope.sh; secret-pattern scan; injection
                     #   quarantine rules; sandbox profiles. tools/gate_all.sh migrates
                     #   here or stays in tools/ — NOTE: moving any gate file changes
                     #   the gate_all fingerprint set, which is a hub-only, broadcast,
                     #   human-gated change (ORCHESTRATION §6/§9); do it as its own
                     #   re-arm round, never silently.
  observability/     # run/trace logs, source-health + Phase-10 metric collection,
                     #   spend ledger, alert files.
  dashboard/         # migrate: tools/status.py → renders STATUS.md + health dashboard.
  audit/             # append-only audit index binding run_ids ↔ task ids ↔ commits
                     #   ↔ verdicts (git remains the primary record).
  tests/             # tests for the orchestrator itself; migrate:
                     #   validators/test_kb_validator.py pattern.
  state/             # (addition to SPEC skeleton) canonical JSONL/YAML ledgers:
                     #   evidence.jsonl, claims.jsonl, candidates.jsonl,
                     #   experiments.jsonl, decisions.jsonl, framework_analyses/.
```

Migration map (all migrations are hub-only commits; gate files additionally
human-gated + broadcast because fingerprints change):

| Existing asset | Destination | Mode |
|---|---|---|
| `tools/gate_all.sh` | `orchestrator/security/` (or stays `tools/`) | wrap; fingerprint-sensitive |
| `tools/check_scope.sh` | `orchestrator/security/` | wrap; fingerprint-sensitive |
| `tools/status.py` | `orchestrator/dashboard/` | move + extend |
| `validators/*` | `orchestrator/evaluations/validators/` | wrap; fingerprint-sensitive |
| `tools/build_router.py`, `tools/build_index.py` | `orchestrator/memory/` | move (drift-gate pattern reused for SQLite/vector caches) |
| `EXCHANGE/*.md` protocol docs | referenced from `orchestrator/config/` | stay in place |
| `EXCHANGE/tasks.json`, `AGENTS.json`, `scores.json` | stay in place | orchestrator/state/ adds pipeline ledgers alongside, same single-writer pattern |
| POLLING.md `gate.sh` | `orchestrator/control_plane/` | generalize (adds budget gate) |
| `dist/specialist_prompts.jsonl`, `specialists/` | stay in place | procedural memory (L6) |

## 9. Open items (UNVERIFIED / deferred)

- Hermes Agent and OpenClaw official repositories: UNVERIFIED — resolve in
  Phase 3 before analysis tasks are minted.
- Codex CLI per-run network disablement inside `--sandbox workspace-write`:
  UNVERIFIED — operator deny-rules are the interim control (§5).
- PROJECT_REGISTRY.yaml, TOPIC_ONTOLOGY.yaml, SOURCE_REGISTRY.yaml do not
  exist yet (deliverables 2–4); every reference above is forward-looking.
- Container persistence characteristics of the Claude Code environment across
  Routine runs: UNVERIFIED — canonical state is in the repo precisely so this
  does not matter, but local caches must be treated as disposable.
