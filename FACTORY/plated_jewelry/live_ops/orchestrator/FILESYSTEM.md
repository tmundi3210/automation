# FILESYSTEM.md — the orchestrator's working-directory spec (the STATE LAYER)

> **Authority.** This file is the single authority for the orchestrator's on-disk state. `live_ops/orchestrator/ORCHESTRATOR.md §2` sketches a working tree and says *"FILESYSTEM.md is the authority; follow it if it differs."* It differs — **use the `work/` tree defined here.** A name map from the earlier draft vocabularies (the ORCHESTRATOR.md sketch and earlier AGENTS.md / SAFETY.md revisions) to this canonical tree is at the bottom (§7).
>
> **Where it lives.** All paths below are **pack-relative** to the unzipped `plated_jewelry_market_pack` root. The orchestrator prompt lives at `live_ops/orchestrator/ORCHESTRATOR.md`; the mutable state layer it creates lives beside it at `live_ops/orchestrator/work/`. Companion contracts: `live_ops/orchestrator/OUTPUT_FORMAT.md` (the record schemas every file here holds), `live_ops/orchestrator/AGENTS.md` (who writes what), `live_ops/orchestrator/SAFETY.md` (the quarantine + secrets rules), `orchestration/*.md` (the analysis doctrine). Read this file ONCE at first boot, create the tree, then never re-read it.

---

## 1. THE EXACT TREE (created on first boot, one time)

On the very first tick the orchestrator detects that no `work/state.json` exists and creates **exactly** this tree. Nothing here is optional; every directory is created empty even if it will not be written until later.

```
live_ops/orchestrator/
└── work/                                   ← THE STATE LAYER (created first boot; entire mutable surface)
    ├── state.json                          ← STATE — the single source of truth (one file; atomically overwritten)
    ├── inbox/
    │   └── <agent>/*.json                   ← UNTRUSTED, UNGATED landing zone: every collector/runner agent (LIVE_RESEARCH, HISTORY, SPECIALIST_RUNNER, ANALYST) writes its returned batch HERE FIRST. Only SAFETY_REVIEW reads inbox and PROMOTES clean records into signals/. Nothing else reads it.
    ├── signals/
    │   └── YYYY-MM-DD--<lane>--<n>.json     ← append-only dated SIGNAL RECORD batches (a JSON array per file); ONLY the gate writes here, by promotion from inbox/
    ├── analysis/
    │   ├── YYYY-MM-DD--rollup.json          ← immutable dated ANALYSIS RECORD snapshots (one per analyzed tick)
    │   └── latest.json                      ← pointer+mirror of the newest rollup — THE ONLY analysis file the orchestrator reads
    ├── poll/
    │   ├── poll_spec.json                   ← POLL SPEC (written once the §5 build-gate is met)
    │   └── poll.html                        ← the generated, deployable poll (self-contained; owner hosts it)
    ├── logs/
    │   └── YYYY-MM-DD.md                    ← append-only run log: one line per tick / per injected agent
    ├── quarantine/
    │   └── YYYY-MM-DD--<id>.json            ← records the SAFETY_REVIEW gate REJECTED, each with its reason
    └── secrets.env.example                  ← named secret SLOTS for future APIs (real secrets gitignored, never committed)
```

The orchestrator also ensures, at first boot, that `work/secrets.env` (the REAL secrets file, if the owner ever creates one) is covered by a `.gitignore` rule — see §5.7. `work/secrets.env` itself is **never created by the orchestrator and never committed**.

---

## 2. EVERY FILE'S ROLE

| Path | Holds | Record type (see OUTPUT_FORMAT.md) | Written by | Read by | Write mode |
|---|---|---|---|---|---|
| `work/state.json` | The current cursor into all history: run id, last live-pull time, cadence, active lanes/specialists, `poll_ready`, `not_converged[]`. **Small on purpose** — a pointer, not a data store. | **STATE** | SYNTHESIZER (step d) → orchestrator persists (step f) | **Orchestrator** (into its own context) | Overwritten in place, **atomically** (write `state.json.tmp`, then rename) |
| `work/inbox/<agent>/*.json` | One collector/runner agent's returned batch — **UNTRUSTED and UNGATED**. `<agent>` ∈ `live_research｜history｜specialist｜analyst`. Written FIRST, before anything is trusted; the gate reads it and promotes only clean records onward. | pending **SIGNAL RECORD**s (collectors) / an ANALYSIS-RECORD-shaped **draft** (ANALYST) | LIVE_RESEARCH / HISTORY / SPECIALIST_RUNNER / ANALYST (step a) | **SAFETY_REVIEW** (collector batches, step b); SYNTHESIZER (the analyst draft, step d) — **by path** | Append-only: a **new file per batch**; the gate never edits it in place |
| `work/signals/YYYY-MM-DD--<lane>--<n>.json` | One tick's harvest from one lane: an **array** of SIGNAL RECORDS, one object per observation. `<lane>` ∈ `live｜history`; `<n>` is a zero-padded batch index (`00`,`01`,…) that increments per additional collection in the same date+lane. KB-grounded specialist reads clear into here too. | **SIGNAL RECORD** (array) | **SAFETY_REVIEW** (step b) — promoted from `inbox/` after a gate PASS | ANALYST (step c) — **by path** | Append-only: a **new file per batch**; existing batch files are never edited |
| `work/analysis/YYYY-MM-DD--rollup.json` | The ranked rollup produced by one analyzed tick — the immutable record of what the analysis said on that date. | **ANALYSIS RECORD** | SYNTHESIZER (step d) | ANALYST reads the **prior** rollup by path (step c) | Append-only: **new dated file each analyzed tick**; never overwritten |
| `work/analysis/latest.json` | A **mirror** of the newest `*--rollup.json` **plus** a `points_to` field naming which dated file it mirrors. Exists so the orchestrator reads exactly **one** small, stable path to know the whole current picture. | **ANALYSIS RECORD** + `points_to` | SYNTHESIZER (step d), after writing the dated rollup | **Orchestrator** (into its own context) | Overwritten in place, atomically, to mirror the latest rollup |
| `work/poll/poll_spec.json` | The generated poll's machine spec: ranked items, finish & price questions, incentive, legal flags. | **POLL SPEC** | POLL_BUILDER (step 5) | Owner / response-capture wiring | Written once the gate is met; re-written on a re-build (new `generated_at`) |
| `work/poll/poll.html` | A self-contained, deployable poll page (inline CSS/JS, no external calls) the owner hosts on their live storefront. | (rendered HTML; not a record) | POLL_BUILDER (step 5) | Owner (deploys); respondents | Written with `poll_spec.json` |
| `work/logs/YYYY-MM-DD.md` | The resumability breadcrumb: one terse line per tick and per agent injected — `HH:MM｜tick N｜agent｜wrote <path>｜summary`. | (log lines; not a record) | Orchestrator (step f) + each agent may append its own line | Orchestrator on resume (reads the **tail** only) | Append-only |
| `work/quarantine/YYYY-MM-DD--<id>.json` | Every record the SAFETY_REVIEW gate rejected, wrapped with the **reason** and a short excerpt of the offending text. Quarantined records are **NOT** promoted into `signals/`. | **QUARANTINE RECORD** (wrapper around the rejected SIGNAL RECORD) | SAFETY_REVIEW agent (step b) | Human review; audit | Append-only: one file per rejected record |
| `work/secrets.env.example` | **Named slots only, no values** — the env-var names for the five API slots (`SEARCH_API_KEY`, `LLM_API_KEY`, `POLL_HOST_API_KEY`, `RESPONSE_CAPTURE_API_KEY`, `GROK_CLI_BRIDGE`), mirroring `config.json.api_slots` (`search`, `llm`, `poll_host`, `response_capture`, `grok_cli_bridge`). A template the owner copies to `work/secrets.env` and fills. | (env template; not a record) | Orchestrator (first boot) | Owner | Written once |

**`config.json`** (a sibling of this file at `live_ops/orchestrator/config.json`, **not** inside `work/`) is the read-only machine config the orchestrator loads at boot to learn the specialist paths, lane weights, source families, poll gate, API slots, and safety switches. It is authored ahead of time and is never edited by an agent.

---

## 3. NAMING & FORMAT CONVENTIONS

- **Dates are `YYYY-MM-DD`** (UTC), sortable lexicographically. Timestamps inside records are full ISO-8601.
- **Lane token** in a signal filename is exactly `live` or `history` (matches `config.json.lanes` and the SIGNAL RECORD `lane` field).
- **Batch index `<n>`** is zero-padded two digits, starting at `00` for the first collection of that date+lane, incrementing for each additional collection the same day. It makes multiple ticks-per-day non-colliding and keeps writes append-only.
- **`<id>`** in a quarantine filename is the `id` of the rejected SIGNAL RECORD (e.g. `sig_a1b2`), so the quarantined item is traceable to what the collector emitted.
- **One record per line is not required** inside the JSON array files — they are standard JSON arrays; but each file is **write-once** (append a new file, never rewrite an old one) so line-level appends are unnecessary and forbidden.
- **All data lives in files; chat carries only a path + a short summary** (the DELEGATION CONTRACT, ORCHESTRATOR.md §7).

---

## 4. INVARIANTS (binding)

1. **Append-only + dated.** `signals/`, `analysis/*--rollup.json`, `logs/`, and `quarantine/` are **write-once, dated snapshots**. An existing dated file is **never edited or deleted**. New information is a **new file**. The only two files overwritten in place are `state.json` and `analysis/latest.json`, and both are pure **cursors/mirrors** whose content is fully re-derivable from the append-only snapshots — losing either loses no committed evidence.
2. **Idempotent re-run reproduces the same state.** Records are joined by `id`; batch files are keyed by `date+lane+n`. Re-running a tick with the same inputs writes to the **same keyed filenames** and recomputes the **same** `state.json` — nothing is double-counted, no evidence is duplicated. A capped number of ticks is a **cost bound, never a completeness claim**.
3. **The orchestrator reads ONLY two files into its own context: `state.json` + `analysis/latest.json`.** Nothing else. It never reads a `*.specialist.json`, a KB, a signal batch, a web page, or a report into its own window — it points **agents** at those paths and receives back a path + short summary. This is the mechanism that keeps the orchestrator's context tiny and constant-size across an unbounded run.
4. **Agents read/write specific files by path.** Each injected agent (LIVE_RESEARCH, HISTORY, SAFETY_REVIEW, ANALYST, SYNTHESIZER, POLL_BUILDER, and specialist calls) is handed the exact paths it may read and the exact path it must write, per `AGENTS.md`. It returns **only** a short summary + the path it wrote. Data never flows back as prose.
5. **Crash-safe.** Every stage writes its own dated file **before** the next stage begins, and `state.json` is written last, atomically (tmp+rename). An interruption at any point loses at most the single in-flight agent's return — never committed state. On restart the orchestrator reads `state.json`, then `analysis/latest.json`, then the **tail** of today's `logs/` file, and continues from the next step (ORCHESTRATOR.md §8). Never re-boot when `state.json` exists.
6. **SAFETY_REVIEW gates EVERY promotion.** Every collector/runner agent writes its returned batch to `inbox/<agent>/` **first** (untrusted, ungated); no SIGNAL RECORD reaches `signals/` until the SAFETY_REVIEW agent has scanned it there and promoted it. The gate is not only the signals door: it **re-scans the SYNTHESIZER's rollup** for fabricated numbers / injection carried in `note` fields **before `analysis/latest.json` is updated**, and **re-scans `poll/poll_spec.json` before `poll/poll.html` is emitted** — so nothing reaches `signals/`, `analysis/`, or `poll/` except through a gate PASS. Flagged records go to `quarantine/` with a reason and `security_flag:true`; they are never promoted. **Every record — regardless of lane — carries `security_flag`** (see OUTPUT_FORMAT.md validation).
7. **Secrets are never committed.** `secrets.env.example` holds **names only**. The real `work/secrets.env` is gitignored and referenced **by name** — never pasted into a subagent prompt or a request body (SAFETY.md; ORCHESTRATOR.md §6).

---

## 5. HOW THIS PREVENTS CONTEXT BLOAT AND HALLUCINATION

This tree is not bookkeeping — it is the **anti-hallucination and anti-bloat mechanism**. Concretely:

- **5.1 Bounded orchestrator context (anti-bloat).** Because the orchestrator holds only `state.json` (a few hundred bytes) + `analysis/latest.json` (one ranked rollup), its context is **constant-size no matter how many ticks run**. A month of continuous collection produces hundreds of signal batches on disk, but the orchestrator's window never grows. Reading a large file into the orchestrator is the one move that would break the continuous loop — so the tree makes every large read a **delegated, path-based** agent read instead.
- **5.2 Externalized memory (anti-hallucination).** The orchestrator never has to *remember* what it saw three ticks ago — it **re-reads the file**. Memory that lives in a context window degrades and gets confabulated; memory that lives in `signals/…json` is exact. Every claim the orchestrator acts on is a record it can point back to by `id`, not a half-remembered summary.
- **5.3 Every number is sourced or tagged (anti-fabrication).** Records carry an `honesty_tag` and, for signals, `evidence`/`corroborated_by` ids. The orchestrator cannot "fill in" a market size or a sales number, because the state layer has **no field to hold an untagged number** — an unsourced figure has nowhere to live except as a `[METHOD]` or `[UNKNOWN]`. The schema itself refuses fabrication.
- **5.4 Immutability makes disagreement visible, not averaged.** Because rollups are dated and append-only, a later tick that disagrees with an earlier one leaves **both** on disk. The ANALYST compares them and records the split (`not_converged`), rather than silently averaging two reads into a fake middle — the classic hallucination of a confident blended number.
- **5.5 Idempotency kills double-counting.** Corroboration ("≥2 independent live sources") is only meaningful if the same observation is not counted twice. Keying batches by `date+lane+n` and records by `id` means a re-run overwrites the same file rather than inventing a second, phantom corroboration.
- **5.6 The two-file read is also a two-agent audit.** `analysis/latest.json` is written by the SYNTHESIZER but compared against `signals/` by a **separate** ANALYST and gated by a **separate** SAFETY_REVIEW. No agent grades its own work; the filesystem is the hand-off medium that makes the independence real.
- **5.7 Secrets isolation.** Keeping credentials in a gitignored `work/secrets.env` (template `secrets.env.example`) referenced by name means a fetched page's injection attempt has **no key to exfiltrate from the context** — the key was never in the context to begin with.

---

## 6. FIRST-BOOT CREATION SEQUENCE (what the orchestrator does once)

1. Detect `work/state.json` is absent → this is first boot.
2. Create the full `work/` tree of §1 (all directories, empty).
3. Write `secrets.env.example` with the five named slots (§2: `SEARCH_API_KEY`, `LLM_API_KEY`, `POLL_HOST_API_KEY`, `RESPONSE_CAPTURE_API_KEY`, `GROK_CLI_BRIDGE`), and ensure `work/secrets.env` is gitignored.
4. Write the initial `state.json` (STATE schema): `run_id` (timestamped), `cadence_minutes` from `config.json.lanes.live.cadence_minutes` (default **60**), `active_lanes:["live","history"]`, `active_specialists` = the six ids from `config.json`, `poll_ready:false`, `not_converged:[]`.
5. Run the one-time **history** baseline (weak lane): the HISTORY agent writes to `inbox/history/`, then the SAFETY_REVIEW gate promotes the clean records into the first `signals/YYYY-MM-DD--history--00.json`, so the first live tick has a prior to compare against.
6. Write the first `analysis/YYYY-MM-DD--rollup.json` from that baseline and mirror it to `analysis/latest.json`.
7. Begin the continuous loop (ORCHESTRATOR.md §4). On every later start, `state.json` exists → **resume, do not re-boot.**

---

## 7. NAME MAP — earlier draft vocabularies → this canonical tree

Earlier parallel-authored drafts diverged from this authority: the `ORCHESTRATOR.md §2` sketch used a `records`-rooted tree, and earlier `AGENTS.md` / `SAFETY.md` revisions used a `state`-rooted tree with two SCREAMING_CASE record filenames. **All of them resolve to the `work/` tree above — use the right-hand column; the left column is dead.**

| Earlier draft vocabulary | Canonical (this file, `work/`) |
|---|---|
| a bare `state.json` | `work/state.json` |
| a `records`-rooted `signals` dir (`…signals/<YYYY-MM-DD>/`, per-observation files) | `work/signals/YYYY-MM-DD--<lane>--<n>.json` (per-batch arrays) |
| a `records`-rooted `analysis` file (`…analysis/analysis_<YYYY-MM-DD>.json`) | `work/analysis/YYYY-MM-DD--rollup.json` (+ `work/analysis/latest.json`) |
| a `records`-rooted `safety` gate log (`…safety/gate_<YYYY-MM-DD>.json`) | `work/quarantine/YYYY-MM-DD--<id>.json` |
| a `state`-rooted `inbox` dir (`…inbox/<agent>/<run>.json`) | `work/inbox/<agent>/*.json` (untrusted landing zone) |
| a `state`-rooted `records` dir (its `signals` or `specialist` subtrees) | `work/signals/…` (specialist KB-grounded reads clear into signals too) |
| the SCREAMING_CASE `ANALYSIS_RECORD.json` | `work/analysis/latest.json` (+ the dated `…--rollup.json`) |
| the SCREAMING_CASE `POLL_SPEC.json` | `work/poll/poll_spec.json` |
| a `state`-rooted or bare `poll` dir | `work/poll/poll_spec.json` + `work/poll/poll.html` |
| a `log`-rooted `tick_<YYYY-MM-DD>.md` run log | `work/logs/YYYY-MM-DD.md` |

_End FILESYSTEM.md. The tree is the state layer; the state layer is the discipline. Orchestrator reads two files; agents read/write by path; every snapshot is dated and append-only; secrets are named, never stored; and no field exists to hold an unsourced number._
