# W2_recall — RECALL — retrieve relevant memory & TRANSFER on a new domain

> Pull the decision-relevant memory slice for the inherited focus, and — when the bound environment is NEW — lift 3–5 structurally-analogous past episodes into reusable schemas and stage a re-bind, instead of reusing the old envelope. Owner specialist: `mind_recall` (the Generalization faculty). Loop position: step 2 of the ONE MIND loop, immediately after PERCEIVE and immediately before PLAN. Single-write-owner file: `schemas/abstractions.md` (written only in TRANSFER mode); the rest of `schemas/*` is read here and authored by consolidation, never by this phase.

## Purpose

RECALL is the retrieval-and-transfer seam of one persistent agent's turn. It answers one question for the current focus: **what does this mind already know that bears on the active goal, and is that knowledge directly reusable or does it have to be lifted and re-bound first?**

Two regimes, decided by one test (the `acquired_in` scan, Protocol step 3):

- **PLAIN RECALL** — the bound `env_id` already appears in some `schemas/<id>.md` `acquired_in`. The schema is competent here; return the loaded memory slice plus the committed binding for this domain. Write nothing. Carry forward any unresolved caveat as a flag, not an answer.
- **TRANSFER** — the bound `env_id` appears in no schema's `acquired_in`. The domain is NEW. Retrieve past episodes that share the *move* (not the topic), lift each into a domain-general schema with its old domain-specific bindings split out, stage every open slot as empty / re-bind-required, and prepare (do not run) a leakage-controlled cheap probe for ACT. Append the lift ledger to `schemas/abstractions.md`.

The hard discipline of this phase: **structure over surface**. A topic match ("also about AI agents") is worthless; a move match ("rising-but-not-saturated event + identity-coded community, adjudicate hype before drafting") is what transfers. RECALL never reuses an old envelope's bindings on a new domain — borrowed bindings are priors to be re-measured, never proof. And RECALL never invents a record: if the focus needs something not present in `INDEX.md`, that absence is a gap flagged to attention, not a hallucinated memory.

RECALL retrieves and lifts. It does not plan, does not act, does not draft, does not score, and does not run probes. It hands a clean envelope to PLAN and stops.

## Inputs (memory read) and Outputs (memory written)

All paths are relative to `branches/b60_content_intelligence/mind/memory/`.

**Inherited from PERCEIVE (do NOT re-derive; read-only):**

- `kernel/cursor.json` — the bound `env_id`, `turn`, `goal`, `phase`, `plan_hash`. RECALL inherits these; it does not re-bind the environment.
- `kernel/environments.jsonl` — the env record for the bound `env_id` (domain, audience, platform, `context_slice`, constraints). Read to know the active envelope, not to choose it.
- `kernel/turn_log.jsonl` — the current run row, to confirm which turn/goal RECALL is operating on.

**Read-first map (mandatory, in full):**

- `INDEX.md` — the NOW block, TIERS counts, HOT POINTERS table, the RETRIEVAL RULE, and CONSOLIDATION DUE. This is the budget and the index; nothing is loaded that the rule does not justify.

**Goal & attention (to know `touches` and open caveats):**

- `goals/goals.md` — the ACTIVE goal block and its `touches:` list (for G-12: `W-ai-agents, A-builders, P-hype-adjud, SCH-geo-link`).
- `goals/subgoals.md`, `goals/direction.md`, `goals/plan.md` — only if the active goal block points into them.
- `attention/questions.md` — open/active questions tied to the focus (e.g. `Q-OQ-07`, `Q-019`), surfaced as carry-caveats.
- `attention/surprises.md` — surprises that bear on the focus (e.g. `S-009`), as transfer/retrieval context.
- `attention/inquiry_budget.md` — the explore ratio, consulted only to label whether this turn is an exploit or explore turn for the diary line.

**Schemas (the transfer decision surface):**

- `schemas/*` — every `schemas/<id>.md` (here `SCH_geo_link_react_brief.md`) is scanned for `acquired_in` to run the new-domain test.
- `schemas/abstractions.md` — read to see prior lifts before appending (TRANSFER mode also writes here).
- `schemas/transfer_log.md` — read to see prior probe verdicts (e.g. `TRX_0007`) and any flagged caveats (`Q-019`).

**Episodic / experience (the analog-retrieval corpus, TRANSFER only):**

- `experience/episodes.jsonl`, `experience/outcomes.jsonl`, `experience/_ledger.jsonl` — claim/outcome rows scored for MOVE-similarity.
- `experience/reflections/*` (e.g. `reflections/e0631.md`) — WHAT-WORKED / WHY lines that name the move.
- `episodic/timeline.md`, `episodic/journal/*` — the run spine, to locate candidate analog episodes by run.

**Semantic / procedural (loaded by topic-match and pointer, ≤2 semantic):**

- `semantic/world/ai-agents.md`, `semantic/audiences/builders.md`, `semantic/glossary.md`.
- `procedural/playbooks/hype-adjudication.md`, `procedural/lessons.md`.

**Calibration (read for probe design, never re-scored here):**

- `calibration/predictions.jsonl`, `calibration/recalibration_map.json`, `calibration/calibration_report.md` — the ECE floor and conf-band conventions used to set the probe commit rule.

**Outputs (memory WRITTEN by RECALL):**

- `schemas/abstractions.md` — **the only file this phase ever writes**, and only in TRANSFER mode. RECALL appends one dated LIFT block per retained analog (the lift ledger: domain-general move + domain-specific old bindings). In PLAIN RECALL mode RECALL writes nothing.

Everything RECALL produces other than that append — the loaded slice, the staged open-slots envelope, the probe spec, the flagged gaps — is handed to PLAN as the turn's working state (PERCEIVE/PLAN own the working/blackboard surface), not written by RECALL into any other memory tier.

## Protocol

### Step 0 — Inherit, do not re-bind

Read `kernel/cursor.json`. Bind locally: `env_id`, `goal`, `turn`, `phase`, `plan_hash`. Read the matching line in `kernel/environments.jsonl` for the active envelope (domain, audience, platform, `context_slice`, constraints). **Do not re-select or re-bind the environment** — PERCEIVE already did. If `cursor.json.env_id` and the only `active:true` env in `environments.jsonl` disagree, stop and escalate (see Failure modes); do not guess.

For the seeded run: `env_id = tech_builders`, `goal = G-12`, `turn = 631`. The active env line is the `tech_builders` record (`"transferred from punjab_diaspora via SCH_geo_link_react_brief"`, `active:true`).

### Step 1 — Read the INDEX in full

Read `INDEX.md` end to end. Extract:
- NOW: active goal id and one-line, current phase.
- HOT POINTERS: the `id → file · why · conf` rows.
- RETRIEVAL RULE: `load INDEX + working/* (always) → + pointers whose id ∈ goal.touches → + ≤2 semantic by topic-match. budget ~6k tok.`
- The hallucination guard: *"if a needed record isn't listed here, it doesn't exist yet — create it, don't hallucinate it."* RECALL does not create records; it flags the absence for attention.

### Step 2 — Apply the INDEX RETRIEVAL RULE and record the LOADED slice

Build the load set, in this exact order, under the ~6k-token budget:

1. `working/*` — **always**. Here: `INDEX.md` NOW block + `kernel/blackboard/tech_builders/` focus (the 3 focus items named in `INDEX.md` TIERS: `working : focus(3)`).
2. **HOT POINTERS whose `id ∈ goal.touches`.** Read `goals/goals.md`, the ACTIVE block's `touches:` list. For G-12 `touches = {W-ai-agents, A-builders, P-hype-adjud, SCH-geo-link}`, so load:
   - `W-ai-agents` → `semantic/world/ai-agents.md`
   - `A-builders` → `semantic/audiences/builders.md`
   - `P-hype-adjud` → `procedural/playbooks/hype-adjudication.md`
   - `SCH-geo-link` → `schemas/SCH_geo_link_react_brief.md`
3. **≤2 topic-matched semantic records** not already loaded. Here the two touched semantic records (`ai-agents`, `builders`) already cover the topic; `semantic/glossary.md` is loaded only if an abbreviation in the slice is unresolved. Do not exceed two extra semantic pulls.

Then record the exact loaded slice as the diary **LOADED line** — the literal list of files+ids pulled. Example LOADED line for this run:

```
LOADED R0631 env:tech_builders goal:G-12 budget~6k :: working[INDEX.NOW, blackboard/tech_builders focus×3] · pointers∈touches[W-ai-agents, A-builders, P-hype-adjud, SCH-geo-link] · semantic≤2[ai-agents, builders] · glossary[on-demand]
```

This slice is **decision-lossless**: it must contain every record needed to make the recall/transfer call, and nothing the budget cannot justify. If a focus-needed record is not in `INDEX.md` HOT POINTERS or TIERS, do **not** load it from disk speculatively and do **not** invent it — list it as a GAP for attention (Step 10), naming what is missing and why the focus needs it.

### Step 3 — The new-domain test (`acquired_in` scan): branch PLAIN vs TRANSFER

Open every `schemas/<id>.md` (here only `schemas/SCH_geo_link_react_brief.md`). Read each `acquired_in:` list. Compare against the inherited `env_id`.

- If `env_id ∈ acquired_in` for any schema → **PLAIN RECALL.** Jump to Step 9.
- If `env_id` is absent from every schema's `acquired_in` → **NEW domain → TRANSFER.** Continue to Step 4.

For the seeded run: `SCH_geo_link_react_brief.acquired_in = [punjab_diaspora, tech_builders]`. `tech_builders ∈ acquired_in` → **PLAIN RECALL** (Step 9). The TRANSFER branch (Steps 4–8) is the procedure that *produced* this committed binding on run R0628 (TRX_0007) and is the protocol RECALL would run on the next genuinely-new env; it is fully specified below so it is runnable when the test goes the other way.

### Step 4 — TRANSFER: retrieve candidate analogs (MOVE-similarity, leakage-controlled)

The domain is NEW. Pull candidate past episodes from `experience/episodes.jsonl`, `experience/outcomes.jsonl`, `episodic/timeline.md`, and `experience/reflections/*`. For each candidate, score two **separate** numbers in [0,1]:

- `struct` = MOVE / structural similarity: does the candidate share the same skeleton of operators — *connect entities on a shared anchor → read reaction → adjudicate hype → draft → flag* — and the same relational invariants (e.g. "rising-but-not-saturated event meets an identity-coded community")? Score the relation, not the nouns.
- `surface` = topic / surface similarity: same entities, same platform, same vocabulary, same domain words.

Keep the two scores apart; never average them into one similarity. Retain **3–5 candidates with HIGH `struct` and LOW `surface`**. The reference profile from the seed corpus is the punjab analog at `struct ≈ 0.79`, `surface ≈ 0.11` (recorded in `schemas/abstractions.md`): that high-struct / low-surface shape is exactly what transfers. **Reject any candidate that clears the bar only on topic** (high surface, low struct) — that is the classic false-transfer trap. Leakage control: the items used to *select* analogs must be disjoint from the items that will later be used to *probe* the lift (Step 8); reserve a holdout now and never let an analog-selection item enter it.

### Step 5 — TRANSFER: structural bar / cold-start gate

Define the structural bar: `struct ≥ 0.70` AND `surface ≤ 0.30` for at least one retained analog (the punjab profile sits comfortably inside this: 0.79 / 0.11). If **no** retained candidate clears the bar:

- Emit a **cold-start flag** to PLAN (`transfer:cold-start, reason:no-analog-cleared-struct-bar, best={id, struct, surface}`).
- **Stop the lift work.** Do not append to `schemas/abstractions.md`. Do not force a weak transfer from a topic-only match — a forced lift poisons the schema and the calibration. PLAN then decides whether to author competence from scratch or defer the goal.

If at least one analog clears the bar, continue to Step 6.

### Step 6 — TRANSFER: lift each retained analog (write `schemas/abstractions.md`)

For each retained analog, split **DOMAIN-GENERAL** (move + invariants that hold in any domain) from **DOMAIN-SPECIFIC** (the old bindings that were true only in the source domain). Append one dated LIFT block to `schemas/abstractions.md` — the only file RECALL writes — in this exact shape (matching the existing ledger format):

```
### LIFT <UTC-timestamp>
from_episode: <EP_id>   to_schema: <SCH_id>   target_domain: <new env_id>
GENERAL (transfers):  "<the move + invariants, domain-free>"
SPECIFIC (re-bind):   <slot>: <old binding> → <to-be-rebound>
                      <slot>: <old binding> → <to-be-rebound>
                      ...   (one line per open_slot)
analogy_basis: STRUCTURAL (<one line naming the shared relation>)
similarity: {struct: <0..1>, surface: <0..1>}
status: probed
```

Every field is required. `analogy_basis` is always `STRUCTURAL` here (a topic-only basis was already rejected in Step 4). `status` starts at `probed` and is only ever advanced to `committed` later by the ACT/consolidation write that records the probe verdict — RECALL leaves it at `probed`. The `SPECIFIC` lines must be paired one-to-one with the schema's `open_slots`, so PLAN knows exactly which bindings are now empty.

### Step 7 — TRANSFER: stage the open slots as EMPTY / RE-BIND-REQUIRED

Take the target schema's `open_slots` — for `SCH_geo_link_react_brief` these are `{audience, anchor, entities[2..3], platform_set, culture_codes}` — and stage every one as **EMPTY / RE-BIND-REQUIRED** in the envelope handed to PLAN. Assert explicitly that the old domain's bindings are **NOT inherited** into the new envelope. Concretely, the punjab bindings (`anchor: Punjab/diaspora`, `culture_codes: bhangra/NRI`, `audience: diaspora youth`, `platform_set: +regional-IG`) are listed as the *source* of each slot's lift, never as its value in the new domain. The invariants travel; the bindings do not.

### Step 8 — TRANSFER: prepare (do NOT run) a leakage-controlled cheap-probe spec for ACT

Design the probe; hand the spec to PLAN/ACT; run nothing. The spec must contain:

- **Holdout**: a frozen NEW-domain holdout, **disjoint from the analog-selection items** of Step 4 (no leakage). Freeze it now so ACT cannot tune against it.
- **n**: a cheap sample size (≈12 items, matching the `TRX_0007` precedent of `n=12` held-out tech items).
- **Pre-registered metric**: one metric chosen before any measurement, e.g. `link_validity@anchor` (the metric `TRX_0007` used). Pre-registration kills metric-shopping.
- **Baseline**: the cold-start / human-curated baseline the lift is measured *over* (TRX_0007 used a `human_curated_trend_post`).
- **Commit rule**: advance the LIFT `status: probed → committed` only if the **lift 95% CI excludes 0** AND **ECE < 0.05** (the glossary's hard calibration floor). TRX_0007 passed at `lift=+0.23, CI [0.06, 0.39], ECE=0.04`.
- **Ablation**: drop ALL source-domain bindings and re-measure; the lift must survive (`acquired-here`, not merely a borrowed prior). TRX_0007 ablation held at `+0.19`.

Hand the spec to PLAN. **Stop** — RECALL does not execute the probe; ACT does.

### Step 9 — PLAIN RECALL: return the slice + committed binding (no lift, no write)

(Reached when Step 3 found `env_id ∈ acquired_in`.) Return:
- The loaded slice from Step 2 (the LOADED line records it).
- The matching already-acquired schema and its **committed binding for this domain** — for the seed: `SCH_geo_link_react_brief` with the `tech_builders` binding committed via `TRX_0007` (`anchor: a shared tool/release event`; `culture_codes: build-in-public/VC/founder`; `platform_set: +x-tech,+HN −regional-IG`; `schema.confidence[tech_builders] = 0.71`).
- **Do NOT re-lift. Do NOT write `schemas/abstractions.md`.** The competence already exists; re-lifting would duplicate the ledger and re-open a settled binding.
- **Surface unresolved transfer caveats to carry, not answer** — e.g. `Q-019` ("medium-tier not yet confirmed to peak at the medium band in tech"; `transfer_log.md` note on `TRX_0007`). These are flags for PLAN/ATTEND, not questions RECALL resolves.

### Step 10 — Hand off to PLAN and stop

Hand PLAN exactly:
1. The **recall slice** (the LOADED line + the loaded records).
2. The **schema state**: in PLAIN mode, the schema + its committed binding; in TRANSFER mode, the staged open-slots envelope (Step 7) + the lift ledger entries (Step 6) + the probe spec (Step 8).
3. Any **flagged gaps** (focus-needed records absent from INDEX) and **carry-caveats** (open questions/surprises).

Attach to **every surfaced item** a `confidence` and a `basis` (the provenance — which runs/records support it). Then **stop**: do not plan, act, draft, or score.

## Decision rules & edge cases

- **Budget overflow (~6k tokens).** Honor the RETRIEVAL RULE priority order: `working/*` first (never dropped), then pointers `∈ touches`, then ≤2 semantic. If the slice would exceed budget, drop from the *lowest* priority up (semantic before pointers; never drop `working/*` or a touched pointer). Record what was dropped on the LOADED line so the omission is auditable. Never silently truncate a touched pointer.
- **Record named in `touches` but absent from INDEX.** Do not load from disk on a hunch and do not invent it. Flag it as a GAP for attention (`gap: <id> needed by <goal> for <reason>, not in INDEX`). The hallucination guard is absolute: *not listed → does not exist yet → flag, don't fabricate.*
- **`acquired_in` ambiguity (partial / probed-not-committed).** If the only evidence that `env_id` is acquired is a `status: probed` (not yet `committed`) lift in `schemas/abstractions.md` with no passing `transfer_log` verdict, treat the domain as **still NEW** (TRANSFER), because competence is not yet earned. Only a committed binding (CI-excludes-0 probe) counts as `acquired_in`.
- **Conflicting beliefs in the loaded slice.** Surface the conflict with both confidences and provenance; do not resolve it. RECALL retrieves; PLAN/REVISE adjudicate. (E.g. the superseded `~~B0 "agent hype is organic" .55~~` is *not* loaded — it is in the SUPERSEDED block, kept for audit only.)
- **Multiple analogs tie at the structural bar (TRANSFER).** Retain up to 5; prefer higher `struct`, then lower `surface`, then more recent / more outcomes-confirmed episodes. Never pad the retained set with a topic-only match to reach 3.
- **No analog clears the bar (TRANSFER).** Emit the cold-start flag (Step 5) and stop the lift work — do not force a weak transfer.
- **Stale or due consolidation.** If `INDEX.md` CONSOLIDATION DUE shows backlog over threshold or bloat not OK, still run RECALL on the current INDEX, and add a note to the handoff that the index may be stale (so PLAN can weigh it). RECALL does not consolidate.
- **Aleatory beliefs.** Beliefs typed `ALEATORY` / `DO-NOT-CHASE` (e.g. `B3 spike magnitude is noisy`) are loaded as context but never spawned into a transfer caveat or a question — chasing irreducible noise is forbidden.
- **False-presupposition questions.** A question flagged `FALSE_PRESUP` (e.g. `Q-016` "have we stopped over-counting bots" — bot ground-truth was never measured) is carried as-is to attention for rewrite; RECALL does not answer it.

## Worked example (run R0631 / goal G-12)

**Inherit (Step 0).** `kernel/cursor.json` → `env_id=tech_builders, goal=G-12, turn=631`. `environments.jsonl` active line = `tech_builders` (`"transferred from punjab_diaspora via SCH_geo_link_react_brief"`). RECALL inherits, does not re-bind.

**INDEX + LOADED slice (Steps 1–2).** `INDEX.md` NOW: `G-12 "weekly brief: AI-agent hype vs organic (tech_builders)"`, with the note that this env was TRANSFERRED from `punjab_diaspora` via `SCH_geo_link_react_brief` (`TRX_0007`). `goals/goals.md` G-12 `touches = {W-ai-agents, A-builders, P-hype-adjud, SCH-geo-link}`. Load set:

```
LOADED R0631 env:tech_builders goal:G-12 budget~6k :: working[INDEX.NOW, blackboard/tech_builders focus×3] · pointers∈touches[W-ai-agents→semantic/world/ai-agents.md, A-builders→semantic/audiences/builders.md, P-hype-adjud→procedural/playbooks/hype-adjudication.md, SCH-geo-link→schemas/SCH_geo_link_react_brief.md] · semantic≤2[ai-agents, builders] · glossary[on-demand]
```

**New-domain test (Step 3).** Scan `schemas/SCH_geo_link_react_brief.md` → `acquired_in: [punjab_diaspora, tech_builders]`. `tech_builders` **is present** → **PLAIN RECALL** (Step 9). No lift, no write to `schemas/abstractions.md`.

**Plain-recall return (Step 9).** Hand PLAN:
- the loaded slice above;
- `SCH_geo_link_react_brief` with its **committed `tech_builders` binding** (`anchor: a shared tool/release event`; `culture_codes: build-in-public/VC/founder`; `platform_set: +x-tech,+HN −regional-IG`; `schema.confidence[tech_builders]=0.71`, committed via `TRX_0007`);
- the relevant beliefs as carry-context with confidence + basis: `B2 "public AI-agent hype is mostly MANUFACTURED" conf .74 (epistemic; provenance R0625,R0630,R0631 — note this is the run after the bump from .70→.74)`; `B1 dev-adoption rising QoQ .68`; `A1 builders reward signal over volume .80`; `A2 "manufactured hype, here's the real read" framing outperforms "X is blowing up" .72`; `P-hype-adjud` win 7/9, the `≥N-origin gate` (N=3, count origins not platforms);
- **carry-caveats, not answers**: `Q-019` ("why did medium-tier outperform top-tier in punjab but NOT yet confirmed in tech" — transfer not yet confirmed; flagged in the `TRX_0007` note); `Q-OQ-07` (active polar, "are the 2 'independent' origins funded by one operator?" from surprise `S-009`); and the open prediction `P-118` (re-spike within 14 days, conf .35, due 2026-07-12).

**What RECALL deliberately does NOT touch this run.** It does not score the refuted prediction `P-114` (deduped to 2 origins, Brier 0.42 — that scoring already happened in the calibration phase); it does not promote the candidate lesson `L-23` ("≥3 platforms agreeing ≠ ≥3 origins — dedup BEFORE counting"; seen 1×, needs 1 more independent confirm); it does not re-bump `B2`; it does not re-open the open episode `e0631` (resolves 2026-07-12 / wk_025). It surfaces `L-23` and `e0631` as carry-state to PLAN with their confidence and provenance, and stops.

**Counterfactual (TRANSFER branch).** Had the env instead been, say, a fresh `crypto_founders` domain with no schema listing it in `acquired_in`, Step 3 would route to TRANSFER. RECALL would pull analogs from `episodes.jsonl` / `timeline.md` / `reflections/`, score them struct-vs-surface, retain the `EP_punjab_h1b_2025`-shaped move ("rising-but-not-saturated event + identity-coded community → adjudicate hype before drafting") at high struct / low surface, append a dated `status: probed` LIFT block to `schemas/abstractions.md` (mirroring the existing `LIFT 2026-06-24T10:00Z` entry), stage `{audience, anchor, entities[2..3], platform_set, culture_codes}` as EMPTY / RE-BIND-REQUIRED with the old punjab/tech bindings explicitly NOT inherited, and hand PLAN a probe spec (`n≈12`, pre-registered `link_validity@anchor`, commit iff CI excludes 0 and ECE < 0.05, plus a bindings-ablation) — exactly the shape that produced `TRX_0007`. Then stop.

## Failure modes & escalation

- **Hallucinated record.** RECALL loads or cites a record not present in `INDEX.md` / on disk. Hard failure — it corrupts every downstream phase. Mitigation: the Step-2 hallucination guard; any focus-needed-but-absent record is a GAP flag, never a fabricated load.
- **Surface-transfer (topic masquerading as structure).** A high-`surface` / low-`struct` analog is retained and lifted. This poisons the schema and the calibration. Mitigation: the two scores are kept separate; the structural bar (`struct ≥ 0.70`, `surface ≤ 0.30`) is enforced; topic-only matches are rejected in Step 4.
- **Envelope reuse on a new domain.** Old bindings inherited instead of re-bound. Mitigation: Step 7 stages all `open_slots` as EMPTY and asserts non-inheritance; the schema SKELETON itself states "empty slots must be RE-BOUND per domain, never inherited."
- **Leakage.** Analog-selection items overlap the frozen probe holdout, inflating the measured lift. Mitigation: Steps 4 and 8 require disjoint sets; the holdout is frozen before ACT runs.
- **Forced lift past a cold-start.** No analog clears the bar but RECALL lifts anyway to avoid emitting a cold-start. Mitigation: Step 5 mandates stop-and-flag.
- **Scope creep.** RECALL plans, drafts, scores, or runs the probe. Mitigation: the phase ends at Step 10 with an explicit stop; probe is *prepared*, never *run*.
- **Budget violation.** The slice silently exceeds ~6k tokens and drops a touched pointer without record. Mitigation: priority-ordered dropping with the drop logged on the LOADED line.

**Escalate to a human when:** (1) `cursor.json.env_id` and the `active:true` env in `environments.jsonl` disagree — the binding is incoherent and RECALL must not guess; (2) two schemas claim `acquired_in` for the same `env_id` with conflicting committed bindings; (3) the only path to clear the structural bar requires retaining a `compliance`/safety-relevant analog whose lift would re-bind a constraint slot (a transfer that touches the fail-closed gate must be human-reviewed, never auto-committed); (4) `INDEX.md` itself is missing, malformed, or its CONSOLIDATION backlog indicates the map is no longer trustworthy. In every escalation, hand off the partial slice and the precise reason; do not paper over it.

## Handoff

**Next workflow: PLAN (step 3 of the loop).** RECALL must leave this state for PLAN:

- The **recall slice** (LOADED line + loaded records), each item carrying `confidence` + `basis`.
- The **schema state**:
  - PLAIN mode (the R0631 case): the matching schema + its committed domain binding (`SCH_geo_link_react_brief` / `tech_builders` / conf 0.71 via `TRX_0007`). `schemas/abstractions.md` is **untouched**.
  - TRANSFER mode: the staged open-slots envelope (all `open_slots` EMPTY / RE-BIND-REQUIRED, old bindings explicitly not inherited), the appended `status: probed` LIFT ledger entries in `schemas/abstractions.md`, and the prepared (un-run) cheap-probe spec.
- **Flagged gaps** (focus-needed records absent from INDEX) and **carry-caveats** (open questions/surprises/predictions: `Q-019`, `Q-OQ-07`, `P-118`, candidate `L-23`, open `e0631`) — as flags to carry, not answers.
- A clear regime label so PLAN knows whether it is planning against a committed binding (exploit the known move) or against an empty, probe-gated envelope (re-bind then probe before committing). On a cold-start flag, PLAN must decide author-from-scratch vs defer.

RECALL stops here. It does not plan, act, draft, score, or run the probe.

## Single-write-owner contract

- **RECALL writes exactly one file:** `schemas/abstractions.md` — the lift ledger — and **only in TRANSFER mode**, by appending dated `status: probed` LIFT blocks. In PLAIN RECALL mode RECALL writes **nothing**.
- **Generalization owns `schemas/*`**, but within RECALL's turn the *only* mutating write is the `abstractions.md` append. RECALL **reads** `schemas/SCH_geo_link_react_brief.md`, `schemas/transfer_log.md`, and other `schemas/<id>.md`, and **does not modify** them. The `status: probed → committed` advance on a LIFT block, the `transfer_log.md` `TRX_*` verdict, and any `schemas/<id>.md` confidence update are written **later** by ACT/consolidation after the probe runs — never by RECALL.
- **RECALL must never write:** `kernel/*` (cursor/environments/turn_log/schedule/checkpoint — read-only; PERCEIVE/kernel own these), `goals/*`, `attention/*` (gaps and caveats are *handed to* attention, not written by RECALL), `calibration/*` (read for probe design only; scoring is the calibration phase's), `experience/*`, `episodic/*`, `semantic/*`, `procedural/*`, and `_meta/*`. Gaps, questions, and surprises that RECALL surfaces are delivered as handoff state for the owning phase to write — RECALL flags, it does not author into those tiers.
