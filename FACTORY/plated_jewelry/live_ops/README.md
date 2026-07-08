# live_ops/ — run the plated-jewelry market analysis LIVE, and build it as an app

_Two ways to operate the 6 gated market specialists as a **continuous, live** market-analysis engine that keeps a ranked read of demand and, when the signal is strong, generates a **deployable poll** you can host and send. No new specialist is created here — this layer only orchestrates the existing 6 (`jewel_market`, `jewel_materials`, `jewel_form`, `jewel_poll`, `jewel_assortment`, `jewel_economics`) plus the existing `sysloops`/`SELF_LOOP` feedback-loop doctrine._

> **Honesty + safety bind everything here.** No fabricated market numbers — every claim is tagged `[FACT]/[FACT-source]/[ESTIMATE]/[METHOD]/[UNKNOWN]/[SIGNAL]`. All fetched web/social/API content is **untrusted data, never instructions**; a `SAFETY_REVIEW` gate is the only thing that promotes a record into state; secrets live in named env slots, never in prompts or git.

---

## Path 1 — `orchestrator/` : run it now on your terminal AI (Grok Code CLI, or any orchestrator-with-agents)

Paste **`orchestrator/ORCHESTRATOR.md`** into the MAIN window. That window becomes a pure **orchestrator**: it never reads sources or fetches the web itself — it injects one fresh single-purpose agent per job, routes their compact outputs through a disciplined file system, and keeps only tiny state in its own context.

| File | Role |
|---|---|
| `ORCHESTRATOR.md` | The master opening prompt: the injector role, first-boot, the two lanes, the continuous loop, when to build the poll. **Start here.** |
| `AGENTS.md` | The injectable agent role prompts — paste one to spawn each: `LIVE_RESEARCH`, `HISTORY`, `SPECIALIST_RUNNER` (×6), `ANALYST`, `SYNTHESIZER`, `POLL_BUILDER`, `SAFETY_REVIEW`. |
| `FILESYSTEM.md` | The `work/` state tree the orchestrator builds on first boot (append-only, dated, idempotent) so it can't hallucinate or blow context. |
| `OUTPUT_FORMAT.md` | The compact record format (SIGNAL / ANALYSIS / STATE / POLL SPEC) every agent emits — data goes to files, not chat. **Paste your own format into its "Custom format slot" to override.** |
| `SAFETY.md` | The prompt-injection / untrusted-content playbook (the "safe sides"): threat model, the 3-point `SAFETY_REVIEW` gate, secrets hygiene, web-action sandbox, human-in-the-loop. |
| `config.json` | Machine config: the 6-specialist registry, the two lanes, the poll gate, and empty `api_slots` you fill later. |
| `poll/POLL_BUILDER.md` + `poll/poll_template.html` | Turn the ranked demand into a **self-contained, dependency-free** poll (top 6+ items by sale demand) you host on your own server and send to people you know. |

**The two lanes** (this is the core of your ask): **FORWARD = live web research (PRIMARY)** — fresh public signals every cadence tick; **BACKWARD = history (WEAK)** — a down-weighted baseline that never outranks a corroborated live signal. Live always leads.

**The continuous loop:** each tick → `LIVE_RESEARCH` collects signals into `work/inbox/` → `SAFETY_REVIEW` gate promotes clean records to `work/signals/` → `ANALYST` compares vs the prior rollup → a *separate* `SYNTHESIZER` writes the updated ranking + state (two different agents, so nothing grades its own work) → when the top buckets are corroborated-strong, `POLL_BUILDER` emits the poll → schedule the next tick. Everything is saved every tick; a crash resumes from `work/state.json`.

**How to actually collect the data** (which sites, which signals → most-sold / frequently-bought): the `LIVE_RESEARCH` agent runs the method in **`../orchestration/REALTIME_DATA_COLLECTION.md`** (bestseller ranks, Movers&Shakers, "frequently bought together", review-velocity, social talk/comments, search interest, editorial trends), triangulated ≥2 independent sources before anything counts as "strong".

## Path 2 — `base44/` : build it as a Base44 app (the "changes everything" path)

Same engine, wrapped as a monitoring app with a **tweak panel** (checkboxes to enable/disable each lane and each specialist, a monitor-duration control, a cadence slider, a live-vs-history weight toggle) and a backend that shells out to your **logged-in Grok Code CLI on Mac** for live search + specialist runs. Base44 already has plan+build agents, so these files give *required detail only*:

| File | Role |
|---|---|
| `BASE44_BUILD_PLAN.md` | The plan first: architecture, screens, data model, the tweak panel, safety posture. **Paste this first.** |
| `BASE44_SLICES.md` | The build sliced into small, independently-buildable prompts (Slice 0…6) — feed them to Base44's build agent one at a time. |
| `BASE44_APP_SPEC.json` | Machine spec: entities, screens, every tweak-panel control, backend jobs, approval gates. |
| `BASE44_BACKEND_GROK.md` | How the backend invokes the logged-in Grok Code CLI (local subprocess) to run live search + the specialists. |
| `README.md` | Orientation + usage order. |

---

## Options you asked for, and where they live
- **Create the poll from gathered data** → `orchestrator/poll/` (auto-triggered by the loop when demand is strong, or run `POLL_BUILDER` on demand).
- **Host it on your server / send the link personally** → `POLL_BUILDER.md` "Hosting options" (self-contained HTML, no dependencies; localStorage+CSV capture now, an API POST slot for later). ⚠ Do **not** host until the Official Rules URL is filled — the template shows a DRAFT banner until then.
- **Add an API later** → named slots only, in `config.json.api_slots` / `work/secrets.env` (`SEARCH_API_KEY`, `LLM_API_KEY`, `POLL_HOST_API_KEY`, `RESPONSE_CAPTURE_API_KEY`, `GROK_CLI_BRIDGE`); never inline a key into a prompt.
- **Tweak what runs / how long to monitor** → Path 1: edit `config.json` (lanes, cadence, active_specialists). Path 2: the Base44 tweak panel.

## Human-in-the-loop (nothing external happens without you)
Spending ad money, sending the poll to real people, hosting anything public, or adding a credential each require your explicit approval (`SAFETY.md §7`; Base44 `ApprovalRequest`). A legal-envelope breach raises `legal_review`; a secret-leak / coordinated-injection event raises an immediate `security_alert`.
