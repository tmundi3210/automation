# OPERATIONS_RUNBOOK — day-2 operations for the bounded-autonomy system

Planning deliverable #14 (SPEC.md PLANNING DELIVERABLES). Audience: the human
operator. This runbook covers the system as designed in IMPLEMENTATION_PHASES.md;
sections referring to not-yet-built components are marked PLANNED. Existing
EXCHANGE machinery (tasks.json, scores.json, STATUS.md, gate.sh, launchd
pollers) is live today and referenced as-is. Machine-specific commands are
placeholders where marked UNVERIFIED — confirm on the actual Mac before relying
on them.

---

## 1. Morning digest — what you see

Delivered daily (PLANNED: Phase D; budgeted in COST_BUDGET.yaml
`digests_and_synthesis`). One screen, no scrolling ambition. Format:

```
DIGEST 2026-07-14  budget: hub 42% | grok 31% | codex 18%   dead-letters: 0   sources: 23 ok / 1 paused

PROJECT <id> — <name>
• <top new finding> [Tier A, score 8.2, EVID-0231]
• AWAITING YOU: CAND-017 <one-line summary> — reply APPROVE CAND-017 or REJECT CAND-017: <reason>
• EXP-009 passed thresholds (+12% eval, no regression) -> draft PR #41 open, unmerged
• source `xlist-agents` silent 5 days (usually daily) — flagged

(next project…)
```

Rules:
- **≤4 bullets per project**, enforced by the renderer, not by prompt. A
  project with nothing new gets zero bullets, not filler.
- Every bullet ends in a clickable ID (EVID-/CAND-/EXP-/PR-/SRC-) that
  resolves in the evidence store or dashboard — no unsourced claims.
- Header always shows: budget % per component, dead-letter count, source
  health summary. A degradation-ladder rung being active is always bullet #1
  of the header line.
- Anything requiring your decision is prefixed `AWAITING YOU:` and includes
  the exact reply form inline.

## 2. Approving / rejecting a candidate — exact reply forms

Decisions are one-line replies (in the digest channel or as an operator note
the hub ingests). The control plane parses these forms and nothing else —
free-text intent does not move state (PLANNED: Phase F wiring):

```
APPROVE CAND-NNN                      # candidate -> authorized for experiment
APPROVE PR-NNN                        # authorizes merge of draft PR; you (or the
                                      # hub, only after this line) perform the merge
APPROVE PROPOSAL-NNN                  # Phase H self-evolution proposal; the
                                      # operator applies the change, not the system
REJECT CAND-NNN: <reason>             # reason is mandatory; recorded verbatim
REJECT PR-NNN: <reason>
HOLD CAND-NNN until YYYY-MM-DD        # parks without rejecting; auto-resurfaces
QUARANTINE EVID-NNN: <reason>         # suspicious content -> quarantine flow (section 6.3)
```

Semantics:
- Approval is affirmative and specific — silence never approves anything;
  expiry of a HOLD resurfaces the item, it does not approve it.
- `APPROVE PR-NNN` authorizes exactly one merge of exactly that delivered SHA.
  A force-push or new commit after approval voids it (same rule as the
  EXCHANGE convention of pinning verdicts to `delivered_sha`, never branch head).
- Every decision line is appended to the decision log (section 9) with
  timestamp and the digest it responded to. Rejections feed the scoring system
  (rejected-candidate patterns lower source/topic weights via Phase H
  proposals — which you also approve).

Budget-ceiling changes are NOT a reply form on purpose: they require you to
edit COST_BUDGET.yaml yourself (`budget_change_requires: operator`,
SPEC governance #9).

## 3. Kill switch — how to pause everything

Order matters: stop the schedulers first (no new work), then confirm quiet.
Nothing here destroys state; everything resumes cleanly.

**Step 1 — pause the Mac builders (soft, reversible, verified mechanism):**
```sh
mkdir ~/.exchange-gate/lock        # gate.sh's mutex; while this dir exists,
                                   # every 5-min tick exits 1 before any fetch
                                   # (mechanism verified in EXCHANGE/POLLING.md §1)
```
UNVERIFIED: whether grok and codex share `~/.exchange-gate/` or each poller
runs under its own OS user with its own state dir (per ORCHESTRATION §9 each
poller is a dedicated non-admin user — then run the mkdir as each user).

**Step 2 — hard-stop the Mac builders (launchd):**
```sh
launchctl list | grep -i -e grok -e codex -e poll   # discover the labels (UNVERIFIED labels)
launchctl bootout gui/$(id -u) ~/Library/LaunchAgents/<label>.plist   # per builder
```
UNVERIFIED: exact plist filenames/labels. Known from TASK-014: grok runs as a
LaunchAgent in `~/Library/LaunchAgents/` with `StartInterval 300`; codex's
equivalent is assumed symmetrical but UNVERIFIED.

**Step 3 — pause the hub (Claude Code remote):**
Disable the hourly **Routine** cron in Claude Code remote sessions.
UNVERIFIED exact mechanism — placeholder: disable/pause the Routine from the
Claude Code web/app interface where it was created. There is no local command
for this; it is a remote-side setting.

**Step 4 — verify quiet:**
```sh
tail -5 ~/.exchange-gate/poll.log            # ticks stop appearing (or show lock exits)
git ls-remote origin | head                  # repo tips stop moving
```
Plus: no new run records in the control-plane run ledger (PLANNED, Phase A).

**Resume:** remove the lock dir(s), `launchctl bootstrap gui/$(id -u)
~/Library/LaunchAgents/<label>.plist` per builder, re-enable the Routine —
hub first, then builders, so the hub sees builder traffic from tick one.

**Escalation beyond pause** (suspected credential compromise): this is an
incident, not a pause — follow ORCHESTRATION §9 runbook: revoke PAT → remove
collaborator → kill poller → `git bundle` evidence → audit push events →
re-baseline → new credential → re-arm. Evidence before cleanup.

## 4. Failure playbook — scheduler silent again

Symptom: no wakes, no digest, tips move but nothing acks. Apply the **TASK-014
diagnostic pattern** (proven 2026-07-13; see EXCHANGE/claude/msg-030.md and
EXCHANGE/grok/msg-010.md):

1. **Is the job loaded?** `launchctl list | grep -i <label>` and
   `launchctl print gui/$(id -u)/<label>` — check last exit status. Not
   loaded → `launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/<plist>`;
   verify `StartInterval` is 300.
2. **Does it die instantly?** launchd's default PATH excludes
   `/opt/homebrew/bin` — every binary in the wrapper (git, the CLI, sh) must be
   absolute-pathed or an explicit PATH line set at the top. (This was the
   actual TASK-014 root cause class.)
3. **Is failure visible?** The wrapper must append a timestamped line to
   poll.log on EVERY tick including idle exits, and the plist's
   StandardOutPath/StandardErrorPath must point at real log files. 0-byte logs
   since install = misrouted output, not health.
4. **Heartbeat bookkeeping:** wrapper must `touch last_wake` after each
   successful model tick and advance `last_seen_tip` only after the session
   exits 0 (POLLING.md §1) — otherwise the 6h heartbeat backstop misbehaves.
5. **Prove the fix with the two-phase protocol:** Phase 1 — root-cause report
   + fix + poll.log tail proving 5-min ticks. Phase 2 — the hub pushes a probe
   commit; the builder's AUTOMATIC wake (no human touch) must ack the probe
   SHA. A manual kick on Phase 2 fails the proof.

Never accept "it should work now" — automatic-wake proof or it isn't fixed.

## 5. Failure playbook — builder delivers out-of-scope

Symptom: delivered branch touches paths outside the task's `write_scope`.

1. Zero-token mechanical check first: `tools/check_scope.sh --agent <agent>
   --task TASK-NNN` + `git diff --stat <base> <delivered_sha>` against the
   ledger's write_scope. This runs before any model review — a scope violation
   is a **mechanical reject**, costing no model budget (POLLING.md §5).
2. Verdict `REJECTED` pinned to the delivered SHA, reason "scope violation:
   <paths>", one revise round issued with the scope restated first and last in
   the task digest.
3. Record it: the rejection lands in scores.json (it dents first-pass accept
   rate and, if the builder claimed compliance, self_report_gap — SCORING §1b).
4. Repeat offense in the same lane → route the next task of that type to the
   other builder (routing is trust-driven); 3 same-class fails opens the
   circuit breaker (SCORING §0).
5. Out-of-scope writes to protected paths (validators/, tools/, EXCHANGE/claude/)
   are never a mere quality issue — treat as suspected compromise and jump to
   the ORCHESTRATION §9 incident runbook.

## 6. Failure playbook — suspicious content

Covers: prompt-injection payloads in ingested pages/posts/repos, instructions
addressed to agents inside content, secret-looking strings, malware-shaped
code. Full policy: plans/bounded_autonomy/SECURITY_POLICY.md (deliverable #9);
binding rules today: ORCHESTRATION §2 injection boundary + §9.

1. **Nothing pauses for content alone** — external content is DATA by
   architecture; an injected page cannot instruct any agent. Quarantine is
   about keeping analysis honest, not about containment panic.
2. Agent-side: the discovering agent refuses the embedded ask, records
   file+line, reports under heading `SUSPECTED-INJECTION` in its next message
   (the standing-orders rule), and continues its task.
3. Operator/hub-side: `QUARANTINE EVID-NNN: <reason>` → the evidence record is
   flagged quarantined; excluded from retrieval, scoring, digests, and any
   experiment input; raw content retained immutably for analysis (it is
   evidence, in both senses).
4. Quarantined items get a hub review (isolated read-only context, no tools):
   false alarm → unflag with note; real → keep quarantined, downscore the
   source's accuracy history, and if a source repeats, auto-pause it and
   propose removal at the monthly review.
5. Code from any external source runs ONLY inside the Phase E isolated
   worktree/sandbox regardless of how clean it looks (SPEC governance #6) —
   quarantine review never executes anything.

## 7. Dead-letter handling

Jobs that fail `max_attempts: 3` (with backoff, per COST_BUDGET.yaml) land in
the dead-letter queue (PLANNED: `orchestrator/audit/dead_letter/`, Phase A).
Each entry: run_id, trace_id, job type, input reference, error, attempt
timestamps.

- **Daily:** digest header shows the count. 0 is the healthy steady state; any
  nonzero count is worth the click.
- **Weekly triage (part of the weekly ritual):** for each entry — transient
  (source outage, rate limit): replay via idempotent re-enqueue (run_id-keyed
  writes make replays safe); deterministic bug: file a fix task; poison item
  that killed a worker twice: park with an operator note, never auto-replay a
  third time.
- Dead-letter capture is on the never-degraded floor: budget exhaustion stops
  new work, never the recording of failures.

## 8. Weekly and monthly rituals

**Weekly (~15 min, alongside the weekly synthesis digest):**
- Read the weekly architecture/technique synthesis (SPEC PHASE 3 cadence).
- Decide any batched `AWAITING YOU` items and Phase H proposals.
- Dead-letter triage (section 7).
- Glance at trust movement in STATUS.md / scores.json — a builder's lane trust
  dropping or a self_report_gap opening is a leading indicator worth a task.
- Budget: week-over-week spend vs COST_BUDGET.yaml; ladder rungs hit this week.

**Monthly (~45 min — the SPEC monthly source-quality and pruning review):**
- Source pruning: for each source, check signal-to-noise, duplicate rate,
  accuracy history, latency vs expectation. Disable sources below threshold;
  approve promotions from the discovery expansion (citations, maintainers,
  first-to-signal performance — rank by demonstrated value, not follower
  count, per SPEC PHASE 3).
- Budget re-baseline: adjust ceilings in COST_BUDGET.yaml yourself if
  warranted (operator-only, governance #9); re-verify the UNVERIFIED plan
  limits against current subscription terms.
- Metrics review: the PHASE 10 ten (useful-alert precision, missed-update
  rate, source latency, duplicate rate, verification rate, experiment success
  rate, adopted-improvement rate, regression rate, cost per accepted
  improvement, human-review burden).
- Retention: confirm storage policy is being applied; spot-check one audit
  trail end-to-end (section 9) to prove reconstruction still works.
- Phase H recurring checkpoint: predicted vs measured outcomes of any applied
  proposals.

## 9. Audit-trail map — "what happened and why"

No answer should depend on anyone's memory. Existing files are live today;
PLANNED files arrive with their phase.

| Question | Look in |
|---|---|
| What tasks exist / what state is each in? | `EXCHANGE/tasks.json` (current truth); `git log -p EXCHANGE/tasks.json` (full state-machine replay) |
| Who built what, how well, and is the trend good? | `EXCHANGE/scores.json` (machine), `EXCHANGE/STATUS.md` (plain-language mirror) |
| Why was this decision made? | Decision log: hub verdict messages `EXCHANGE/claude/msg-NNN.md` (today) + the architecture-decision and rejection log (PLANNED, PHASE 5 layer 7, Phase C) + your recorded APPROVE/REJECT lines with reasons |
| What did agents actually do during a task? | Commit trailers (Task/Agent/Gates/Scope on every task commit) + `EXCHANGE/<agent>/transcripts/task-NNN/` for MED/HIGH tasks |
| Integration timeline — what merged when? | `git log --first-parent` on the hub branch; merge commits cite task id, delivered SHA, verdict msg path |
| Where did this claim/finding come from? | Evidence store record via its EVID id → raw-content reference + content hash + source id (PLANNED, Phase C); a vector-index hit is never the authority — follow the link back (SPEC PHASE 5) |
| What ran on which schedule, and what did it cost? | Control-plane run ledger: run_id, trace_id, per-stage budget counters (PLANNED, Phase A); daily digest headers for the human-readable trace |
| What failed and was anything lost? | Dead-letter queue (section 7) + poll.log per-tick lines on the Mac + failure alerts in digests |
| Why is this source enabled/disabled? | SOURCE_REGISTRY.yaml history (`git log -p`) + monthly review notes |
| Who changed budgets/policy, when? | `git log -p plans/bounded_autonomy/COST_BUDGET.yaml` (and later the deployed config) — any non-operator author = incident (section 6 / governance #9) |

Reconstruction drill (run it monthly, section 8): pick one merged change and
walk digest bullet → EVID → CAND → EXP record → PR → APPROVE line → merge
commit → post-merge monitoring note. Every hop must exist in a file above.
