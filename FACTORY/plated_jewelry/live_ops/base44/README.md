# live_ops/base44 — the Base44 app build package

Turns the plated-jewelry market-analysis pack into a **continuously-monitoring web app**: two evidence lanes
(live web + down-weighted history), the six market specialists, a **tweak panel** the owner drives, and a
**Grok Code CLI backend** on the Mac that does the research. It ticks on a cadence, re-ranks demand buckets,
and — when the top buckets are `strong` and corroborated — drafts a deployable poll. Same specialists, same
compact record format, same live/history model, and same safety + honesty posture as the terminal
orchestrator in `live_ops/orchestrator/`.

## The files
- **BASE44_BUILD_PLAN.md** — the plan: architecture, 5 screens, 6 entities, tweak-panel controls, timing,
  and safety posture. Paste this first.
- **BASE44_SLICES.md** — the build sliced into 7 independently-buildable steps (Slice 0→6). Each doubles as a
  prompt for Base44's build agent.
- **BASE44_APP_SPEC.json** — machine spec: entities, screens, settings_controls, backend_jobs, safety,
  api_slots. Feed alongside the plan.
- **BASE44_BACKEND_GROK.md** — how the Mac-side runner shells to the already-logged-in Grok Code CLI
  (invocation contract, two-lane mapping, safety envelope, API slots).

## How to use it
Base44 already has plan + build agents. **Paste `BASE44_BUILD_PLAN.md` (with `BASE44_APP_SPEC.json`) so its
agents have the whole picture, then feed `BASE44_SLICES.md` one slice at a time**, not starting a slice until
the prior one's *done-when* holds. Wire the Grok-CLI runner per `BASE44_BACKEND_GROK.md` at Slice 1.

Binding, non-negotiable: every load-bearing claim carries an honesty tag; no market size / %/ rate / price is
ever stored as measured when it isn't; all fetched content is untrusted data (never instructions), gated by
SAFETY_REVIEW before it is analyzed; and spend / send / host / add-key are always human-in-loop.
