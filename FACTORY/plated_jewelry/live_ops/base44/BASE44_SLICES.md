# BASE44_SLICES.md — build one slice at a time

Each slice is small, independently buildable, and doubles as the prompt you paste into Base44's build agent.
Build in order; do not start a slice until the prior one's **done-when** holds. Paste `BASE44_BUILD_PLAN.md`
first so the agent has the whole picture, then feed these one at a time. Pack paths are pack-relative to the
unzipped `plated_jewelry_market_pack/` root.

---

## Slice 0 — Data model + Settings

**Goal:** stand up the entities and the tweak panel so state exists before anything runs.

**Build:** the six entities **Signal, AnalysisRollup, RunState, PollSpec, Source, Specialist** plus supporting
**RunLogEntry** and **ApprovalRequest**, with fields exactly per `BASE44_APP_SPEC.json`. Seed **Specialist**
with the 6 rows from `orchestration/SPECIALIST_INDEX.md` (id, spec_path, kb_paths, role, enabled=true) and
**Source** with the 5 families from `orchestration/REALTIME_DATA_COLLECTION.md` (enabled=true). Create the
singleton **RunState** (run_id, cadence_minutes=60, active_lanes=[live,history], monitor_duration_minutes=1440,
live_leads=true, auto_poll=true, poll_ready=false). Build the **Settings** screen with every control in the
plan §4, writing to RunState / Specialist.enabled / Source.enabled.

**Done-when:** all entities exist; toggling any Settings control persists to the right field; RunState reads
back the seeded specialists and sources.

---

## Slice 1 — Backend Grok-CLI runner + Signal ingest

**Goal:** one manual tick can pull live signal through the Grok Code CLI and land Signal records.

**Build:** the `grok_cli_runner` bridge contract (full spec in `BASE44_BACKEND_GROK.md`) and a manual
**"Run one tick now"** action that: enqueues a LIVE_RESEARCH job (role prompt from
`live_ops/orchestrator/AGENTS.md`, method from `orchestration/REALTIME_DATA_COLLECTION.md`) for each enabled
live Source; the Mac runner invokes `grok` with the prompt file and captures a JSON array of SIGNAL RECORDs;
the app writes them as **Signal** rows with `status:pending`, `lane:live`. Add the HISTORY job (lane:history,
reads `report/MARKET_ANALYSIS_REPORT.md`, down-weighted) behind the history-lane checkbox.

**Done-when:** clicking "Run one tick now" produces ≥1 pending Signal with a valid honesty_tag and confidence,
url null ⇒ honesty_tag `[UNKNOWN]`, and the Signals screen shows them.

---

## Slice 2 — Analysis rollup + Dashboard

**Goal:** turn cleared signals into a ranked, dated `AnalysisRollup` and show it.

**Build:** two chained jobs — **ANALYST** (groups signals by bucket, scores on the reliability ladder,
verdicts strong/weak/watch/not_converged, writes a draft) and a **separate SYNTHESIZER** (independently
cross-checks evidence_ids, then writes the `AnalysisRollup` new dated snapshot + updates RunState;
`is_current=true` on the newest). Never the same window for both. Build the **Dashboard** reading the current
rollup: ranked buckets with demand_score, verdict, tag, evidence count, live/history badge, poll_ready,
monitor status.

**Done-when:** a tick with cleared signals writes one AnalysisRollup; Dashboard ranks the buckets; a
social-only bucket never shows `strong`; overlapping/thin evidence shows `not_converged`.

---

## Slice 3 — Tweak panel wiring

**Goal:** the Settings controls actually change what the next tick does.

**Build:** wire every control into the tick: disabled lane ⇒ its lane not dispatched; disabled Specialist ⇒
not run; `cadence_minutes` sets the tick interval; `monitor_duration_minutes` sets `monitor_until`;
`live_leads` toggles the ladder rule (history can never outrank corroborated live when on); `auto_poll`
gates whether Slice 4 auto-builds. Add the SPECIALIST_RUNNER job path so an enabled specialist can be run
against its `spec_path` + `kb_paths` (KB-grounded, emits lane:history signals for jewel_market /
jewel_assortment).

**Done-when:** unchecking a specialist or lane visibly removes it from the next tick's dispatch; changing
cadence changes the next-tick interval; monitor_until is recomputed on save.

---

## Slice 4 — Poll builder + export/host

**Goal:** produce a deployable poll draft from the top buckets, gated for deployment.

**Build:** the `build_poll` job (POLL_BUILDER role, instruments from `orchestration/POLL_KIT.md`,
methodology from `specialists/poll/jewel_poll.specialist.json` + its 3 KBs) → writes a **PollSpec**
`status:draft` with 6+ items ranked by demand_score, the finish question, the Gabor-Granger price/WTP
question, an incentive inside the FTC/sweepstakes envelope, and `legal_flags` (e.g. `ARV<=~$500`). Build the
**Poll builder** screen to preview it and export the runnable poll file. **Host** and **Send** buttons open
an ApprovalRequest — they do not act.

**Done-when:** with poll_ready true, a draft PollSpec renders with 6+ ranked items + finish + price + legal
flags; Host/Send create a pending approval instead of deploying.

---

## Slice 5 — Scheduling / continuous monitor

**Goal:** the app ticks on its own for the configured window.

**Build:** the scheduled `monitor_tick` job firing every `cadence_minutes` while `now < monitor_until`
(Continuous = never stop). Each tick runs the full pass (LIVE_RESEARCH [+HISTORY] → SAFETY_REVIEW → ANALYST →
SYNTHESIZER → triggers → RunLogEntry) and schedules the next. Build the **Run log** screen (one row per tick)
and a resume path: on restart read RunState + latest AnalysisRollup + Run log tail and continue.

**Done-when:** enabling monitoring produces ticks at the set cadence, each appending a RunLogEntry and a dated
AnalysisRollup; ticking stops at monitor_until; an interrupted tick resumes without double-counting.

---

## Slice 6 — Safety gate + human-in-loop approvals

**Goal:** nothing unsafe enters the analyzed set; no consequential action fires unattended.

**Build:** the `safety_review` job (SAFETY_REVIEW role) inserted **before** ANALYST every tick — schema +
injection + secret + fabricated-number scan → set Signal `status:cleared` or `quarantined`; only cleared
records feed ANALYST. Build the **Approvals** queue: the four human-in-loop actions (**spend, send_poll,
host_public, add_api_key**) each create an ApprovalRequest that the owner approves/denies before the runner
executes it. Surface quarantined records on the Signals screen (visible, flagged, uncounted).

**Done-when:** a planted injection string in a fetched observation is quarantined with `security_flag:true`
and excluded from the rollup; an unsourced number is quarantined; clicking Host/Send/Spend/Add-key routes
through Approvals and only runs after approval.
