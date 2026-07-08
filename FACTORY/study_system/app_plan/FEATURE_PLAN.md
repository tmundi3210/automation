# FEATURE_PLAN.md — the phased build plan for the Base44 study app

**What this document is.** You said: *"crop this whole thing, then plan one by one, not all of it together."* This is that plan: eight phases, each one independently shippable (the app is fully usable after each phase, and you can stop after any of them), ordered by dependency first and value second. Nothing here is built yet — every phase ends at an **owner approval gate**: a question only you can answer before the next phase starts.

**Honesty tags used throughout:** [FACT] verified in a named file · [FACT-source: URL] verified against an external source per EXTERNAL_RECON.md · [ESTIMATE] an inference, with basis and confidence stated · [UNKNOWN] not verifiable right now · [METHOD] how to find out · [SIGNAL] weak indicator.

**Terms defined once (plain language):**
- **Entity** — a data table in Base44 (e.g. `Card`, `DailyPlan`, `Weakness`).
- **Backend function** — a small server-side program in the app (runs on Deno, Base44's server runtime) [FACT-source: docs.base44.com backend-functions overview, via search snippet].
- **Automation** — Base44's way to run a backend function on a schedule or when data changes [FACT-source: same page, via snippet — verify in-app before relying on it].
- **Specialist** — one of the gated expert prompts in `FACTORY/study_system/specialists/` (9 exist). "Consult" means: use that specialist's spec+KBs as the design authority for that part of the phase.
- **Proposed specialist** — one of the 5 named-but-NOT-built specialists in `SPECIALIST_LIST.md §C` (kgraph, viz, extint, media, uxguide). "Blocks" means the phase should not be designed without building that specialist first.
- **MCP** — Model Context Protocol: a standard plug that lets AI tools (Codex, Claude Code, Grok) connect to external data/tools. A program can be an MCP **server** (offers data/tools) or **client** (uses them).
- **FSRS / desired retention** — the app's scheduling algorithm and its single workload dial (how reliably you want to remember; higher = more reviews).

---

## Build order at a glance

```
P1 Explainability ──► P2 Planner dynamics ──► P3 Frequency control + Anki + Calendar
   (no dependencies)     (no external infra)  ──► P4 Export + ingest + WhatsApp
                                              ──► P8 Managing agent (also wants P2's pipeline scheduled)
P5 MCP + Ollama ── needs only a stable data model (after P2); YOUR TOP PRIORITY —
                   can be pulled forward to run right after P2, in parallel with P3/P4
P6 Knowledge graph ── HARD-BLOCKED until proposed specialists kgraph + viz are built
P7 Media/3D cards ── needs proposed specialist media; uses P5's Codex bridge for image generation
P8 Managing agent ── last: it supervises everything the earlier phases created
```

Reading: P1→P2 is the spine. P3, P4, P5 are independent branches off P2 (any order; you said Codex/MCP is top priority, so P5 may jump the queue — nothing in P3/P4 feeds it). P6 and P7 each wait on specialists that must be **built first**. P8 goes last by design: an agent should manage loops that already exist.

---

## Cross-cutting facts that shape every phase

1. **The app has no scheduler.** Everything labeled "nightly" runs only when you press the Dashboard button [FACT — APP_RECON §3b, grep-verified]. Base44 Automations can schedule backend functions [FACT-source: docs.base44.com, via snippet — verify in-app]; wiring `runPipeline` to a real schedule is prerequisite work claimed by P2 (soft) and P8 (hard).
2. **Timezone bug.** Day-keys are computed in UTC while the calendar draws local days [FACT — APP_RECON risk 5]; for a user west of UTC, evening reviews land on "tomorrow." Any calendar/day-detail/next-day feature (P2, P3) must fix this first or the day view will show wrong data. [ESTIMATE — high confidence for negative-UTC-offset users.]
3. **Silent 500-row query caps** in pipeline and dashboard queries [FACT — APP_RECON risk 3]. At your planned 2000+ cards, stats will silently under-count. Fix rides along with P2.
4. **Secrets pattern.** Base44 stores user-pasted API keys as per-app Secrets, used from backend functions [FACT-source: docs.base44.com/Integrations/Using-integrations, via snippet]. This is the storage pattern for the Ollama/Tripo/WAHA/Google credentials in P3–P7.
5. **Weakness rows accumulate duplicates across days and "resolved" marks are wiped by same-day reruns** [FACT — APP_RECON §4a, risk 8]. P2's auto-resolution must also fix the lifecycle, not just the button.

---

## Phase 1 — Explainability layer ("what does this number mean?")

**Goal.** Every non-obvious label, number, and code in the app explains itself in plain language, in four steps of depth (a "ladder"): (1) a tiny caption under the label → (2) a hover card with 2–3 sentences → (3) a "more" icon opening a longer panel → (4) a full Guide page. Includes a glossary that decodes exam jargon like **FK6 = Foundation Knowledge area 6, "General and disease-specific pathology"** — one of the 10 FK areas of the INBDE blueprint [FACT — KG_RECON §3; exact official wording is [ESTIMATE-high], verify against the JCNDE Item Development Guide PDF per KG_RECON's METHOD].

**What changes in the app (prose only).**
- No entity changes. A tooltip component wraps existing labels on Dashboard (Due/New/Today/Streak), Planner (Budget share, Total cards, Last day to learn, go/tight/no_go), Learner Model (E1–E4, confidence %, BKT mastery, Elo), Review (AI-suggested rating), Settings (every field — especially `desired_retention`).
- One new Guide page (a route like `/guide`) holding the glossary and the long-form explanations; hover cards deep-link into it.
- The explanation *content* already exists: APP_RECON §4 documents what every number actually is (e.g. "confidence 80%" is a fixed heuristic constant of whichever diagnosis rule fired, **not** a computed probability [FACT — APP_RECON §4a]). The tooltips must say so honestly.
- Today the app has zero tooltips and no guide layer of any kind [FACT — APP_RECON §5.9], so this is pure addition, no rework.

**Existing specialists to consult.** learner (E1–E4 and mastery semantics), sched (retention dial wording — its doctrine: desired retention is the single workload knob, warn above 0.90 [FACT — SPECIALIST_RECON §1]), engage (day-box/streak wording), dental (FK/CC glossary), toefl (band semantics).

**Proposed specialist.** `uxguide` (SPECIALIST_LIST §C5) *governs* this but does **not hard-block it**: the factual content is already pinned down in APP_RECON, so a first version can ship now and be audited by uxguide once built. [ESTIMATE — medium-high; the risk of shipping without uxguide is wording quality, not correctness.]

**Integration prerequisites.** None. Entirely in-app.

**Risks.** (a) Explanations that overclaim — e.g. calling the 80% a "probability." Mitigation: every tooltip drafted from APP_RECON §4 wording, honesty tags carried into user-facing text where the number is heuristic. (b) The 56-CC full enumeration is [UNKNOWN] in-repo [FACT — KG_RECON §3]; the glossary ships the 3 CC groups + 10 FK areas now and marks the 56-item list "pending official source" ([METHOD]: extract from the JCNDE Item Development Guide PDF in a fetch-enabled session).

**Owner approval gate.** You read the Guide page and hover cards and confirm: "this is how I want the app to talk to me" (tone + depth), and you approve the FK/CC glossary wording before it's treated as canon.

---

## Phase 2 — Planner dynamics (the calendar comes alive)

**Goal.** Four connected changes: (a) clicking a day on the dashboard calendar opens a day-detail view (topics + time windows for that day); (b) detailed planning happens **only for the next day**, computed from today's + yesterday's answers — farther days show only a coarse forecast ("plan one by one, not all of it together"); (c) Anki-style **gating** — a concept group does not advance to new/harder material while its base concept is weak, and regression is allowed (a group that got weak again is pulled back); (d) weaknesses **auto-resolve** from evidence, replacing the manual Mark-Resolved button.

**What changes in the app (prose only).**
- *Day-detail view:* the calendar day boxes get a click action (today they have none [FACT — APP_RECON §5.10]). Clicking opens a panel: past days show what happened (reviews done, from Revlog + DailyPlan); today/tomorrow show the plan — the topics (concept names of cards due that day) and suggested **time windows**. Note honestly: the app stores no time-of-day anywhere today; "time windows" are a new planner output derived from your `daily_minutes` setting split into named windows — the split rule is a design decision for you at the gate.
- *Next-day-only planning:* the pipeline already writes tomorrow's DailyPlan (stage 4) [FACT — APP_RECON §3b]; it is extended to write a **detailed** tomorrow (per-topic breakdown informed by today+yesterday Revlogs and fresh weaknesses), while days beyond tomorrow only ever show the coarse feasibility curve from the Planner. Requires the pipeline to actually run daily — either the owner's morning button press, or a Base44 scheduled Automation [FACT-source: docs.base44.com, via snippet]; the schedule is strongly recommended here and mandatory by P8.
- *Gating:* the diagnosis already *says* "block new cards on this concept until prereq mastery ≥ 0.6" but nothing enforces it — it is prose only [FACT — APP_RECON §4a rule 2]. Enforcement goes into review-queue assembly: new-state cards of a gated concept are skipped (review of already-seen cards continues — gating never blocks reviews, per sched/engage's non-punitive contract [FACT — SPECIALIST_RECON §7]). Regression = the gate re-applies automatically if base-concept mastery falls back below threshold. Two honest caveats: (1) prereq relationships live in `ConceptEdge`, and **no code ever creates a ConceptEdge row today** [FACT — APP_RECON §5.5], so edge-based gating silently does nothing until edges exist — the phase therefore includes a minimal way to declare "A is the base of B" (manual, in the Library) and a concept-level fallback gate (a concept whose own mastery is weak gets no *new* cards). (2) The review queue today ignores the new-card allowance entirely [FACT — APP_RECON risk 7]; gating work fixes that too.
- *Auto-resolved weaknesses:* a Weakness closes itself when the evidence says recovered — concept mastery back above threshold **and** correct answers on that concept's probe cards across at least 2 distinct days ([ESTIMATE — design default per sysloops' trend-not-snapshot rule; thresholds to harden with real runs]). The Mark-Resolved button is replaced by a milder "dismiss for now" (snooze) that cannot fake mastery. The lifecycle bugs are fixed in the same stroke: one open row per (concept, cause) upserted across days instead of duplicates, and reruns no longer wipe same-day resolutions [FACT — both defects in APP_RECON risk 8].
- *Rides along:* the UTC/local-day fix and the 500-row cap fix (cross-cutting facts 2–3) — the day view is wrong without them.

**Existing specialists to consult.** sched (queue assembly, due semantics), learner (gating thresholds, weakness recovery evidence, uncertainty gate), sysloops (pipeline stage changes, idempotency, trend-based auto-resolution), engage (day-detail visual semantics — its day-box render spec is binding owner law [FACT — SPECIALIST_RECON §7]), qcraft (what counts as a probe item).

**Proposed specialist.** None blocks. (extint would own the Automation-scheduling platform details, but a manual morning button press is an acceptable Phase-2 fallback.)

**Integration prerequisites.** Base44 scheduled Automations [FACT-source: docs.base44.com backend-functions overview, via snippet]; [UNKNOWN] exact scheduling UI/limits — [METHOD] open the Base44 workspace → Automations and confirm a daily schedule can invoke `runPipeline`.

**Risks.** (a) Gating with hand-declared edges can dead-lock a topic if an edge is wrong — mitigation: gate always overridable per-concept in the UI, and regression/gating events are listed on the day view so nothing is silently hidden. (b) Auto-resolution thresholds are unobserved design defaults [ESTIMATE]; they must be displayed (Phase 1 tooltips) and tuned from real runs. (c) Writing plans on dashboard load already races with review-time updates [FACT — APP_RECON risk 6]; consolidating DailyPlan writers is part of this phase's cleanup.

**Owner approval gate.** You use the day view for ~a week and approve: the time-window split rule, the gating threshold (default 0.6 mastery [FACT — the prose rule's own number]), and that auto-resolution feels right before the manual button disappears for good.

---

## Phase 3 — Review-frequency control + AnkiConnect push + Google Calendar reminders

**Goal.** (a) A plain-language "review workload" control in Settings; (b) push cards from the app into your local Anki via AnkiConnect; (c) study-window reminders appear in your Google Calendar.

**What changes in the app (prose only).**
- *Review-frequency control:* honest framing first — the app already has the one dial that controls review frequency: `desired_retention` (0.70–0.97) [FACT — APP_RECON §5.11]. Scheduler doctrine says this is THE single workload knob and adding a raw "interval multiplier" would fight the algorithm [FACT — sched invariant, SPECIALIST_RECON §1]. So this feature is a friendlier presentation: a dial labeled in outcomes ("fewer reviews, forget more ↔ more reviews, forget less"), live-estimating the workload change, warning above 0.90, plus the existing `new_per_day` and `daily_minutes` grouped under one "How much do you want to study?" panel. No new scheduling math.
- *AnkiConnect push:* a "Send to Anki" action (per deck / per selection) creating notes in Anki. Settings already has `anki_mode`/`anki_endpoint` stubs with zero code behind them [FACT — APP_RECON §5.1].
- *Calendar reminders:* after each plan is made (P2), the app writes tomorrow's study windows as calendar events with popup reminders.

**Existing specialists to consult.** sched (owns Anki interop — the only external integration any existing specialist owns [FACT — SPECIALIST_RECON map]; also owns the retention-dial doctrine), engage (reminder-as-habit-cue framing, non-punitive rule), sysloops (when reminders fire relative to the nightly plan).

**Proposed specialist.** `extint` (SPECIALIST_LIST §C3) **blocks the Anki and Calendar halves** — nobody existing owns OAuth, localhost bridging, or retry/idempotency across the network boundary. The frequency-control half is not blocked.

**Integration prerequisites (all from EXTERNAL_RECON).**
- AnkiConnect: install via add-on code 2055492159; Anki must be running; it answers only on `http://127.0.0.1:8765` on your own computer [FACT-source: https://git.sr.ht/~foosoft/anki-connect README, via mirror]. A cloud server can never reach it [FACT — EXTERNAL_RECON cross-cutting]. Two honest routes: (1) the app's *frontend* (which runs in your browser, on your machine) calls it directly — AnkiConnect supports a CORS allowlist and shows an in-Anki permission popup for new origins [FACT-source: same README], so this likely works after you click "allow" once [ESTIMATE — medium-high; [METHOD]: test one `deckNames` call from the deployed app's origin]; (2) a tiny helper program on your machine (folds into P5's MCP bridge). Note-creation API: `addNote`/`addNotes` with deck, model, fields, tags, duplicate handling [FACT-source: same README]. What Anki-side scheduling we can and cannot touch: per-day limits and due dates yes (`saveDeckConfig`, `setDueDate`); FSRS parameters/desired retention **no** — AnkiConnect has no FSRS action; that is set inside Anki itself [FACT-source: same README grep + docs.ankiweb.net/deck-options.html via mirror].
- Google Calendar: creating events uses `events.insert` with per-event reminder overrides (`popup`/`email`, minutes up to 4 weeks ahead, max 5 per event) [FACT-source: developers.google.com/workspace/calendar/api — concepts/reminders + events/insert, via search snippets]. Requires OAuth consent (a "sign in with Google and grant access" flow), scope `.../auth/calendar.events`; a pasted API key is NOT enough for private calendars [FACT — EXTERNAL_RECON §3]. Base44 lists Google Workspace among its OAuth connectors [FACT-source: docs.base44.com/Integrations, via snippet]; whether that connector covers Calendar scope is [UNKNOWN] — [METHOD]: check the connector list in the Base44 workspace; fallback is a manual OAuth flow in a backend function.
- There is **no Google "Reminders" API** — Google migrated reminders into Tasks, and the Tasks API stores due *dates* without times [FACT-source: workspaceupdates.googleblog.com 2023-06 + developers.google.com/tasks reference, via snippets]. So timed nudges = Calendar events with popup reminders. Full stop.

**Risks.** (a) Push-to-Anki creates a second scheduling authority — the same card living in both systems gets two different due dates. Mitigation: push is one-way and marked ("managed by the study app; review it here OR there, not both"), per sched's import-boundary doctrine. (b) OAuth token expiry/revocation needs re-consent UX. (c) If the CORS route fails, Anki push slips to P5's bridge — the phase still ships frequency control + Calendar.

**Owner approval gate.** You approve the direction of sync (app → Anki one-way), grant the Google consent yourself, and confirm reminder times/wording after a trial week.

---

## Phase 4 — Local save/export, upload-and-ingest folder, WhatsApp nudges

**Goal.** (a) One-click export of your data — cards, notes, graphs (chart data), and fundamental units (the terms/claims/edges of the decomposition method) — as files you keep; (b) an upload area where you drop source documents to become cards; (c) optional WhatsApp study nudges via WAHA.

**What changes in the app (prose only).**
- *Export:* buttons in Settings producing downloadable files (JSON for entities; CSV for tabular; chart-data snapshots). No export/download code of any kind exists today [FACT — APP_RECON §5.1, §5.6 grep-verified]. Plain JSON download is pure frontend work; `.apkg` (Anki's package format) export stays out of this phase — it's the off-repo worker's job today [FACT — APP_RECON risk 2] and Anki transfer is better served by P3's live push.
- *Upload/ingest:* a Library upload zone accepting documents, feeding a staged pipeline: file → proposed units/terms/cards → **you approve** → cards created. The only upload in the app today is the single voice-answer file [FACT — APP_RECON §5.7]. Important scope honesty: contenteng's method governs decomposition **above** the parsing level, but "OCR/PDF parsing below the TOC" is explicitly out of its scope and no specialist owns it [FACT — SPECIALIST_RECON gap 5; deliberately not proposed as a specialist yet, SPECIALIST_LIST §C footnote]. Phase 4 therefore ships text-first ingestion (pasted text, clean text files) and treats scanned-PDF OCR as a flagged later add-on.
- *WhatsApp nudges:* a backend function sends "your plan for tomorrow" / "streak at risk" texts through a WAHA server you self-host.

**Existing specialists to consult.** contenteng (ingestion pipeline, card lint, coverage gate — its comprehension gate says no card is minted from un-understood text [FACT — SPECIALIST_RECON §5]), qcraft (item quality of generated cards), engage (nudge wording — cue, never punishment), voice (if voice notes ride the same queue).

**Proposed specialist.** `extint` blocks the WAHA half. Export is unblocked. Ingest is governed by contenteng (exists) with the OCR sub-gap flagged.

**Integration prerequisites (from EXTERNAL_RECON §4).** WAHA is a self-hosted Docker container (`devlikeapro/waha`, port 3000) wrapping WhatsApp Web; sessions authenticate by scanning a QR code; sending is `POST /api/sendText` with `chatId` like `<phone>@c.us` [FACT-source: waha.devlike.pro docs via github.com/devlikeapro/waha-docs]. The free Core edition = 1 session, text-only sends; media/multiple sessions require the paid Plus tier; exact prices [UNKNOWN] — [METHOD]: waha.devlike.pro/#pricing [FACT-source: WAHA README + site, via snippet]. You need a machine that keeps the container running (home lab), reachable by Base44's backend over HTTPS — exposure method (tunnel/VPN/port-forward) is [UNKNOWN] until your home-lab setup is known ([METHOD]: decide with extint once built).
- [SIGNAL] WAHA automates WhatsApp Web unofficially; account-ban risk exists. Keep it optional, off by default, and consider a spare number.

**Risks.** (a) Ingest quality: LLM-decomposed documents can produce confident wrong cards — the approval step and contenteng's lint gate are non-negotiable. (b) WhatsApp ban risk [SIGNAL above]. (c) Export completeness drift — new entities added in later phases must be added to the exporter (make the exporter enumerate entities, not hardcode them).

**Owner approval gate.** You verify a round-trip: export → look at the files → confirm nothing you care about is missing; you approve the first 20 ingested cards by hand before bulk ingestion is allowed; you explicitly opt in to WhatsApp knowing the ban risk.

---

## Phase 5 — MCP bridge (Codex first) + Ollama Cloud model picker — YOUR TOP PRIORITY

**Goal.** (a) Your coding agents — **Codex first**, Claude Code and Grok as options — can connect to the study system and read/write it (ask "what am I weak on?", add cards, trigger the pipeline); (b) in Settings you pick which LLM the app itself uses: a list of current Ollama Cloud models, **exactly 2 active at a time**, with an API-key slot — and a path that works when your terminal/app is already logged into Ollama.

**What changes in the app (prose only).**
- *App side:* backend functions exposed as authenticated custom endpoints (Base44 supports custom endpoints from backend functions [FACT-source: docs.base44.com backend-functions overview, via snippet]) forming a small read/write API: due summary, weaknesses, plan, card create, pipeline trigger.
- *Bridge side (runs on your computer):* a small MCP **server** program that translates MCP calls into those endpoints. This one bridge serves all three agents, because all three are MCP clients:
  - **Codex:** registered with `codex mcp add <name> -- <command>` (or a streamable-HTTP URL); servers stored in `~/.codex/config.toml`; `/mcp` in the TUI lists them [FACT-source: developers.openai.com/codex/mcp, via search snippet]. Codex can additionally act as an MCP *server* itself (`codex mcp-server`) [FACT-source: developers.openai.com/codex/cli/reference, via snippet] — which is how P7 asks Codex to do work *for* the app.
  - **Claude Code:** `claude mcp add --transport http|sse|stdio ...`, project-level `.mcp.json`; it can also serve (`claude mcp serve`) [FACT-source: code.claude.com/docs/en/mcp — fetched directly].
  - **Grok:** xAI's Grok Build CLI has native MCP support and reads the same `.mcp.json`/Claude-style configs unchanged [FACT-source: x.ai/news/grok-build-cli + x.ai/cli, via snippets]. Whether Grok Build can act AS a server is [UNKNOWN] ([METHOD]: docs.x.ai from an unblocked network).
- *Model picker:* a Settings panel that (1) lists the live cloud catalog by calling Ollama's list endpoint, (2) lets you activate exactly two (e.g. "generator" and "grader" roles), (3) stores your key. Today the app has zero model choice — both LLM calls use Base44's built-in `InvokeLLM` with no model parameter [FACT — APP_RECON §5.8]; the app's own LLM calls (card generation, answer grading) are then routed to your chosen models, with `InvokeLLM` kept as fallback.

**Existing specialists to consult.** sysloops (which app actions are safe to expose to an external agent; append-only rules), learner/sched (semantics of any data an agent writes back), voice (if agents grade answers).

**Proposed specialist.** `extint` **blocks this phase** — MCP wiring, key handling, model-ops (cost/drift/fallback) are exactly its scope, and no existing specialist covers any of it [FACT — SPECIALIST_RECON gaps 3, 7, 8].

**Integration prerequisites (from EXTERNAL_RECON §1, §5).**
- Ollama Cloud API: create a key at ollama.com/settings/keys; send it as `Authorization: Bearer` header; keys don't expire, revocable [FACT-source: docs.ollama.com docs/api/authentication.mdx via docs-source mirror]. Cloud base URLs: native `https://ollama.com/api`, OpenAI-compatible `https://ollama.com/v1/` [FACT-source: docs/api/introduction.mdx + docs/integrations/droid.mdx, same mirror]. List models: `GET https://ollama.com/api/tags` or `/v1/models` [FACT-source: docs/cloud.mdx + openai-compatibility.mdx]. The compatible surface supports chat completions with streaming, tools, JSON mode, vision [FACT-source: openai-compatibility.mdx].
- "Already logged in" path: after `ollama signin`, calls through your local Ollama (`http://localhost:11434`) to `-cloud` models are auto-authenticated with no pasted key [FACT-source: docs/cloud.mdx + api/authentication.mdx]. Honest limit: that only helps components running on *your machine* (the MCP bridge can use it); the Base44 cloud backend cannot see your localhost, so the app's own calls need the pasted key. Whether the app's web page itself can call your localhost Ollama from the browser is [UNKNOWN] (cross-origin/mixed-content behavior unverified) — [METHOD]: one test fetch from the deployed origin.
- Model catalog honesty: the full live cloud list is [UNKNOWN] from here (page egress-blocked); names verifiable today include `gpt-oss:120b-cloud`, and the docs' current-recommended set `kimi-k2.6`, `minimax-m3`, `glm-5.1`/`glm-5.2`, `qwen3.5` (+`:397b`), `deepseek-v4-flash`, `gemma4:31b`, `mistral-large-3:675b` [FACT-source: docs/cloud.mdx deprecations table]. **Several older cloud models retire 2026-07-15 — one week from today**; the picker must read the live list at runtime and never hardcode model names [FACT-source: same]. Rate limits/pricing tiers [UNKNOWN] — [METHOD]: ollama.com/cloud + account settings.

**Risks.** (a) An agent with write access can corrupt learning data — the bridge ships read-only by default; each write capability is enabled by you one at a time, and every agent write is journaled (sysloops append-only rule). (b) Model churn: retirements happen on weeks' notice [FACT-source: docs/cloud.mdx] — the picker must detect a retired selection and ask you to re-pick, never silently substitute. (c) Key leakage — key lives only in Base44 Secrets and your local bridge config, never in frontend code.

**Owner approval gate.** You run one end-to-end Codex session against the bridge (read-only), then decide which write permissions to unlock; you pick the initial 2 active models yourself.

---

## Phase 6 — Knowledge-graph section (view your concepts as a map)

**Goal.** A new app section that renders your concepts and their relationships as an interactive, color-coded map with **semantic zoom** (zoomed out: honest clusters/hubs; zoom in: individual concepts, then cards), fed by (a) the app's own data and (b) ingestion from your msi knowledge-graph project.

**What changes in the app (prose only).**
- New Graph page. The data model already fits: `Concept` and `ConceptEdge` entities exist with prereq/related/part_of edge types [FACT — APP_RECON §5.5]. What's missing is everything else: no code ever creates a ConceptEdge row, no page renders any graph, and no graph library is installed [FACT — same, grep-verified].
- Edge creation paths: manual (from P2's gating UI), extraction during P4 ingestion (contenteng's method: every multi-term claim becomes an edge triple `(term_a, relation, term_b, unit_id)` [FACT — KG_RECON §2a]), and import from msi.
- msi ingestion honesty: your `tmundi3210/msi` repo exists (TypeScript, branch `claude/learning-os-system-leSN5`) and is *likely* the USMLE-KG project [FACT existence / ESTIMATE purpose — KG_RECON §1; it was out of scope to read this session]. [METHOD]: a future session reads its schema and writes the mapping onto Concept/ConceptEdge before any import code is planned.
- Color coding and zoom semantics (what color *means* — mastery? high-yield? FK area? — and what detail appears at each zoom level) are exactly the two proposed specialists' scopes.

**Existing specialists to consult.** contenteng (edge-table semantics — it deliberately defers full graph theory until adaptive selection demands it [FACT — KG_RECON §2a DR_08]), learner (mastery as a color channel), dental (FK×CC as a grouping axis).

**Proposed specialists.** **HARD-BLOCKED by two builds, per SPECIALIST_LIST §C:** `kgraph` (C1 — the graph's data and meaning layer: schema, hubs, prerequisite structure, terminology linking) and `viz` (C2 — the drawing layer: semantic zoom, perceptual+colorblind-safe color encoding, layout). Both must go through the house pipeline (L1 research → MAKE_A_SPECIALIST Path B → build gate ALL GREEN) before this phase is designed [FACT — SPECIALIST_LIST §C method note]. Useful proven interaction vocabulary to hand them: Obsidian's graph view (force layout, node size by inbound links, color groups by query, hover-highlight of connections, local per-note graph) [FACT-source: help.obsidian.md/plugins/graph via obsidianmd/obsidian-help].

**Integration prerequisites.** None external beyond the msi read ([METHOD] above). A graph-rendering library must be chosen — none is installed [FACT — APP_RECON §5.5]; the choice belongs to viz.

**Risks.** (a) Building the view before the specialists exist would bake in arbitrary color/zoom semantics — that is why this phase is blocked, not just "later." (b) Graph views become decorative if nothing consumes them; tie every visual channel to a real decision (what to study next, where gating bites) per sysloops' named-consumer rule [FACT — SPECIALIST_RECON §9]. (c) 500-row caps again at graph scale.

**Owner approval gate.** Two gates: (1) you approve building kgraph + viz (specialist creation is owner-gated by house rules [FACT — SPECIALIST_LIST footer]); (2) you approve the color legend and zoom levels on a sample of ~50 real concepts before full rollout.

---

## Phase 7 — Media & 3D flashcards (and voice notes on cards)

**Goal.** Cards can carry generated images and rotatable 3D models: Codex generates ×3 images of a structure → Tripo turns images into a 3D model → the card shows a rotatable model in the browser. Vision Pro support later. Voice notes attachable to cards.

**What changes in the app (prose only).**
- Card entity/form gains media attachments (image set, 3D model file, voice note). Review screen embeds a 3D viewer for cards that have a model.
- Generation pipeline (asynchronous, like the voice queue): request → images → 3D task → model stored → attached to card, with a review step where you accept/reject the model.
- "Codex-generated images ×3": Codex is a coding agent; whether/how it generates raster images is [UNKNOWN] from the sources gathered ([METHOD]: test via the P5 bridge — Codex as MCP server `codex mcp-server` [FACT-source: developers.openai.com/codex/cli/reference, via snippet] — or substitute any image model; Ollama's OpenAI-compatible surface lists an *experimental* `/v1/images/generations` endpoint [FACT-source: docs/api/openai-compatibility.mdx]). The ×3 count fits Tripo's multiview input neatly (see below).
- Voice notes reuse the existing capture path (recorder → private file upload → queue) [FACT — APP_RECON §4d]; note the queue has **no consumer in the repo** — transcription was delegated to an off-repo worker [FACT — same]; this phase (or P8's agent) finally gives the queue a consumer, under voice's rules (candidates from voice notes require your confirmation before entering the knowledge base [FACT — SPECIALIST_RECON §8]).

**Existing specialists to consult.** voice (voice-note queue + grading contract), contenteng (when an image/3D actually serves the card — minimum-information rule), qcraft (media in item stems).

**Proposed specialists.** `media` (SPECIALIST_LIST §C4) **blocks** — when 3D helps learning vs. decorates is its core question; `extint` handles the Tripo API operationally.

**Integration prerequisites (from EXTERNAL_RECON §7).** Tripo: API-key auth (`TRIPO_API_KEY`), asynchronous task flow (create → poll → download), credit balance query [FACT-source: PyPI tripo3d + github.com/VAST-AI-Research/tripo-python-sdk docs/API.md]. Single-image `image_to_model` or `multiview_to_model` with up to 4 views — **front required, back/left/right optional (2–4 images)** — your "×3 images" maps to front+left+right [FACT-source: same SDK docs]. Output is GLB (a standard 3D file format); conversion to USDZ/FBX/OBJ/STL/3MF exists via `convert_model` [FACT-source: same]. In-browser display: Google's `<model-viewer>` web component renders GLB/glTF with one HTML tag [FACT-source: github.com/google/model-viewer]. Vision Pro path: USDZ is Apple's AR format and Tripo can convert to it [FACT-source: SDK docs for the conversion; the Vision-Pro-consumes-USDZ claim itself is [ESTIMATE — general platform knowledge, high confidence; [METHOD]: verify on developer.apple.com when that sub-phase starts]]. Pricing/credits [UNKNOWN] — [METHOD]: platform.tripo3d.ai/docs/billing.

**Risks.** (a) Cost per model is unknown — start with a small credit budget and a per-week cap. (b) 3D can violate the minimum-information principle (pretty but slow to review) — media specialist's accept/reject rubric gates every model. (c) Model files are heavy; storage/loading strategy needed before bulk generation.

**Owner approval gate.** You review the first 5 generated 3D cards side-by-side with their 2D versions and decide whether 3D earns a permanent place; you set the credit budget.

---

## Phase 8 — The managing agent (the system runs itself, with your hand on the gates)

**Goal.** A Base44 agent/automation that runs the nightly sysloops analysis without your button press, watches the self-metrics, routes anomalies to the right specialist, and proposes — never silently applies — anything non-mechanical.

**What changes in the app (prose only).**
- The Dashboard "Run nightly analysis" button becomes a real schedule (Base44 Automation) with the button kept as a manual override.
- The agent's duties are already specified by house doctrine — nightly-run steward, diagnostician, router/escalator, periodic spec auditor, benchmark scanner — mapped in SPECIALIST_MAP.md §2d [FACT — that mapping; the runtime is the new work].
- Approval gates are structural, not polite: only mechanical, gate-green fixes self-apply; everything else lands in a proposal queue for you; agent self-reports are never acceptance; an independent re-gate follows any applied change [FACT — SELF_LOOP.md invariants via SPECIALIST_RECON]. Degraded mode: if the pipeline is unhealthy the agent still ships a plain "FSRS-due-reviews-only" plan — a valid degraded plan always beats a rich broken one [FACT — sysloops invariant].
- New surface: a small "Agent activity" page — what ran, what it found, what awaits your approval.

**Existing specialists to consult.** sysloops (its entire mandate is this phase's spec), learner (diagnosis-efficacy meta-loop), all 9 (their declared boundaries/escalation triggers form the agent's routing table [FACT — SPECIALIST_RECON doctrine §b3]).

**Proposed specialist.** `extint` **blocks the runtime half** — Base44 agent wiring (`base44/agents/` JSONC configs, CLI sync), scheduling infra, and LLM-ops for the agent itself are covered by no existing specialist [FACT — SPECIALIST_RECON gaps 7, 8; SPECIALIST_LIST §C3].

**Integration prerequisites.** Base44 Automations (schedules + DB-event triggers) and agents-as-JSONC-configs [FACT-source: docs.base44.com backend-functions overview + AI-agents pages, via search snippets — both flagged verify-in-app]. [UNKNOWN]: agent runtime limits, allowed triggers, model/cost controls — [METHOD]: build a one-step throwaway automation in the Base44 workspace first and observe.

**Risks.** (a) An agent that edits learning state autonomously could quietly damage months of data — hence append-only journaling of every agent write and the proposal queue for anything non-mechanical. (b) Every threshold in the nightly loop is an unobserved design default [ESTIMATE — sysloops' own honesty carry]; the agent must report trends for ~2 weeks before it is allowed to act on them. (c) Cost: an LLM-driven agent running nightly needs a budget cap and the P5 model picker's cost visibility.

**Owner approval gate.** A two-week shadow period: the agent runs and *proposes only*; you approve its go-live for each duty separately (steward first, auditor last).

---

## What could reorder this plan

- You said **Codex/MCP is your top priority**: P5's only real dependency is a stable data API (post-P2). Pulling P5 ahead of P3/P4 is safe; pulling it before P2 risks building the bridge against data semantics P2 then changes. [ESTIMATE — planning judgment, high confidence.]
- P6 and P7 cannot be pulled forward past their specialist builds without breaking the house rule that ungoverned domains don't ship [FACT — SPECIALIST_LIST §C method].
- Near-term UI-only items from P1–P2 (plus placeholder Settings panels) are extracted into `BASE44_CHANGE_PROMPT.md` in this folder, ready to paste into the Base44 builder now.
