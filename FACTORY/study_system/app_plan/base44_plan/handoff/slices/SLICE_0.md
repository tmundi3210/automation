# SLICE 0 — Platform foundations (everything else rides on these)

_Repo-driven mode: obey `docs/plan/PROTOCOL.md`. Any specialist path like `FACTORY/study_system/specialists/<code>/<code>.specialist.json` in the directive block resolves HERE to `docs/plan/specialists/<code>.specialist.json`._

## SLICE 0 — Platform foundations (everything else rides on these)
**Goal.** A real scheduler, correct day keys, exact counts, an enforced new-card allowance, a single credential store, and a reliability spine for every external effect.

**What to build.**
- **S0.1 Scheduler.** Today nothing recurs: "nightly" = the Dashboard button [FACT — APP_RECON §3b, risk 1]. Wire `runPipeline` to a Base44 Automation on a daily schedule (Automations run backend functions on schedules or DB events [FACT-source: docs.base44.com backend-functions overview, via snippet — verify in the workspace before relying on it; exact scheduling UI/limits UNKNOWN, METHOD: open workspace → Automations and confirm a daily invoke of `runPipeline`]). Overlap-safety is mandatory: the pipeline is already keyed by `run_date` delete-then-recreate [FACT — APP_RECON §3b], so add a per-run-date guard row (skip if a PipelineRun for that key is in progress/complete) so a schedule firing while the manual button runs cannot double-apply. Keep the button as manual override. Schedule fires in the **owner's local day** (see S0.2), after local midnight.
- **S0.2 Day-key fix.** Day keys are computed in UTC while the calendar draws local days [FACT — APP_RECON risk 5]. Behavior: one shared day-key rule — an explicit `timezone` field in Settings (owner-set, IANA name), and every day-key producer/consumer (Dashboard, submitReview's DailyPlan write, runPipeline stages, DayBoxCalendar) derives the day from that timezone. Acceptance: a review submitted at 23:00 local lands on today's plan, not tomorrow's.
- **S0.3 Lift the 500-row caps.** Pipeline and dashboard queries silently truncate at 500 rows [FACT — APP_RECON risk 3]; at the planned 2000+ cards stats under-count. Behavior: every capped query becomes a paginated loop to exhaustion (or a server-side aggregate), and each pipeline stage records rows-scanned in PipelineRun so truncation is observable, never silent. Base44 pagination semantics [UNKNOWN — METHOD: verify SDK list/filter paging in the workspace].
- **S0.4 Enforce `new_per_day`.** The review queue takes the first 30 due states ignoring the new-card allowance [FACT — APP_RECON risk 7]. Behavior: queue assembly counts new-state cards introduced today (from Revlog) and admits new cards only up to `Settings.new_per_day`; reviews of seen cards are never blocked. Acceptance: with new_per_day=5 and 40 due new cards, at most 5 new cards enter a day's queue.
- **S0.5 Secrets are the ONLY credential store.** Every external credential lives in a per-app Secret read server-side inside a backend function [FACT-source: docs.base44.com/Integrations/Using-integrations, via snippet] — never in client code, never in the two-way-synced repo (sync would leak it [FACT-source: docs.base44.com GitHub-sync pages]). Named slots in the master plan's integration table; empty slots render as "not configured" in Settings, showing slot name only, never values. Rotation: on suspected exposure, rotate and fire the `security_alert` gate. Filling any slot passes the `credential_add` gate (owner pastes it personally).
- **S0.6 Reliability spine (new entity: `OutboxItem`).** Every must-not-lose external effect (Anki note push, Calendar event, WhatsApp send) persists intent first: `OutboxItem {target, action, payload, idempotency_key, status queued|sent|failed|dead, attempts, last_error, external_id}`. A drainer Automation retries only retriable classes (timeouts, 5xx, 429) with exponential backoff + jitter within a timeout budget; exhausted items go `dead` (DLQ) and surface in UI; per-target circuit breaker fails fast during a sustained outage. Non-idempotent POSTs are never blindly retried — the idempotency key dedupes (exactly-once is unachievable; at-least-once + idempotency is the contract).

**Acceptance checks.** The S0.2 and S0.4 acceptance lines above; a scheduled run and a same-night button press yield one applied run; a killed external send retries to `sent` or lands in the visible DLQ, never duplicates; no secret value ever appears in client code, repo, or UI.

**Owner approval gate.** `schedule_enable` — the owner enables the daily Automation in the workspace themself; `credential_add` for any slot filled. No other external action in this slice.

```
Base44 agent directive
> Load FACTORY/study_system/specialists/extint/extint.specialist.json (paste the full JSON into your planning context). Design strictly by its role + decision_procedure; treat its escalation_triggers as stop-and-ask rules and its validation_checklist as this slice's acceptance test. Do not act outside its boundaries.
```

---
