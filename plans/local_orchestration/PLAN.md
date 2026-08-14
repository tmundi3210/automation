# PLAN — Two-layer orchestration: GitHub bus + local low-latency lane

Status: PLANNING ONLY — no implementation, no config changes, no commits by
this document. Protocol context: EXCHANGE v3.3 (ORCHESTRATION.md), POLLING
v1.1 (delivery-gated tip advance). Operator ask: "GitHub as main, then
terminal CLI as talking brain." All latency numbers below are HEURISTIC
(order-of-magnitude); no timing data was collected.
Second-pass review (agenteval advisor, 2026-07-14): APPROVE_WITH_CHANGES —
all findings revised in place, plus two live facts folded in (clone path
confirmed; tip-reset kick disproven on the current grok wrapper).

## 1. System framing (loops)

**Purpose.** Cut the round-trip time of the task/verdict cycle when the
operator is at the Mac, without weakening the durable audit trail or the
single-writer invariants. **Reference mode:** today a deliver→verdict→revise
hop costs minutes-to-an-hour (poll tick 0–300 s per hop + hub availability);
target is interactive (seconds-to-minutes) during attended work, unchanged
when unattended.

**Boundary.** Endogenous: the GitHub repo (bus), three CLIs, the two
LaunchAgent pollers and their gate state dirs, the Mac clone(s), this plan's
lock marker. Exogenous: GitHub availability, model/subscription rate limits,
the operator, the co-resident third-party gateways (R6).

**Stocks and flows.**

| Stock (accumulation) | Inflow | Outflow | Units |
|---|---|---|---|
| Unprocessed hub messages per builder | hub msg pushes | processed on wake (`last_processed_msg`) | msgs |
| Open tasks by state (`tasks.json`) | mints | verdicts / abandonment | tasks |
| Per-lane trust scores (SCORING.md) | accepted work | decay / rejections | score |
| Gate state (`last_seen_tip`) | delivery-gated advance | manual reset (kick) | 1 sha |
| **NEW: unpushed local commits** | local-lane work | pushes to GitHub | commits |

The local lane creates the last stock. It must be explicitly bounded (§4) —
an unbounded unpushed-work stock silently breaks the audit trail (R3: anything
unpushed is lost) and starves the cloud hub's view.

**Loops today.**

- **B1 poll-tick loop (balancing):** tip moved → wake → process → advance tip
  → idle. Information delay: 0–300 s scheduler + model runtime. Goal-seeking:
  drives "unprocessed messages" to zero.
- **B2 verdict loop (balancing):** deliver → hub review → verdict →
  (revise → rework)*. Each hop crosses B1's delay twice; delay-bearing
  balancing loop, therefore oscillation-prone — revise ping-pong is the
  observed form. This is the loop the local lane accelerates.
- **R1 trust/routing loop (reinforcing):** accepted work → lane trust up →
  more assignments (success-to-the-successful; capped by SCORING §2 OBSERVED
  override). Unchanged by this plan.

**What the local lane changes.** It removes B1's 300 s information delay from
B2's hops when the operator is present (manual kicks, interactive sessions).
Advisor caution applied: shortening a delay raises effective loop gain — every
guard that made the slow loop safe (replay guard, delivery-gated tip, scope
gates) must run unchanged in the fast lane, or the fast lane amplifies the
very failure loops below. Guards precede speed.

**Failure loops (all have occurred) and how the design avoids re-creating them.**

| Failure loop | Structure | Guard in this design |
|---|---|---|
| Tip-burn (TASK-015 forensics) | Wrapper advanced `last_seen_tip` on `model_exit=0` without delivery → wake flow decoupled from the unprocessed-work stock → starvation (broken balancing loop) | POLLING v1.1 delivery-gated advance stays the ONLY writer of `last_seen_tip`; local kicks go THROUGH the wrapper, never hand-edit gate state except the §3(b) tip reset (NEW — defined by this plan, unreliable until TASK-018 lands; see §3(b)) |
| Duplicate-ACK (A3, two incidents) | Colliding gate state (`~/.exchange-gate/`) → same message processed twice → double-applied effect (idempotency violated) | Separate state dirs per agent (`~/.exchange-gate`, `~/.exchange-gate-codex`) + per-agent REPLAY GUARD; kicks are per-agent commands; idempotency-before-retry: never kick both agents "to be safe" for one agent's task |
| Cross-agent clobber | Two writers, one path → last-write-wins race (positive/amplifying coupling) | Ownership stays a total function: per-agent clones or worktrees, per-task disjoint scopes gated at mint (`gate_all --repo`) and pre-push (`check_scope.sh`); topology C rejected outright (§2) |

## 2. Topology decision

| | A: per-agent clones + GitHub-only rendezvous + kicks + local claude hub | B: shared local bare mirror as fast remote, batched push to GitHub | C: one shared working tree |
|---|---|---|---|
| Mechanics | Status quo bus; operator kicks pollers on demand; a LOCAL interactive `claude` session in the clone acts as talking brain / (optionally) integrator | Agents push/fetch `~/src/automation-mirror.git`; a sync job pushes batches to GitHub | All three CLIs in one checkout |
| Hop latency (heuristic) | ~2–10 s per push/fetch; wake = seconds with a kick, 0–300 s scheduled | <1 s local push/fetch; GitHub visibility delayed by batch interval | ~0 s (no transport) |
| Governance impact | None — every invariant untouched | GitHub stops being THE source of truth between batches; authenticity checks and pollers pin to origin tips → wrapper + §9 checks all need rework; verdicts issued off local-only state are invisible to audit until sync | Breaks single-writer-per-path, scope gates meaningless at FS level |
| New failure loops | None new; both-hubs race (mitigated §4) | Mirror/GitHub divergence is a NEW stock with no gate watching it; two rendezvous points = split-brain | Re-creates cross-agent clobber structurally (index.lock races, checkout stomps) |
| Complexity added | ~0 | Mirror + sync job + rewritten gate/authenticity logic | Negative — removes existing safety |

**C is REJECTED:** it is the cross-agent clobber loop rebuilt at the
filesystem layer. Two CLIs editing one working tree race on the index, on
checkouts, and on files; no gate runs at `write()` time.

**B is not recommended now:** the latency it buys (~2–10 s → <1 s per hop) is
not the binding delay — the 300 s poll tick and hub availability are. It
spends its complexity budget attacking the wrong delay and weakens the one
non-negotiable (GitHub as durable source of truth). Revisit only if measured
GitHub round-trips ever dominate the loop.

**RECOMMENDATION: A.** The binding delays are (1) the poll tick and (2) hub
attention. A removes both when the operator is present — kicks collapse (1),
a local interactive claude hub collapses (2) — while changing zero governance
mechanics. Unattended behavior is byte-identical to today.

## 3. Operator commands (copy-paste)

Clone path CONFIRMED 2026-07-14 (operator `find` output):
`~/Documents/movie/automation`. All commands below use it.

**(a) After-restart health check** — use first thing after any reboot/login to
confirm both pollers reloaded.

```sh
launchctl list | grep -E 'exchange-poll'
```

Expect two lines (`com.mundi.exchange-poll-v1`, `com.mundi.exchange-poll-codex`);
a `-` in the first column means not-running-now (normal between ticks). A
PERSISTENTLY nonzero second column plus errors in the wrapper log suggests a
failing job — but verify what the wrapper propagates before reading exit codes
as failures: gate.sh exits 1 on every idle tick by design (POLLING.md), and
the operator's 2026-07-14 check showed both jobs healthy at status 0.
Note: `last_seen_tip` files PERSIST
across restarts and may hold a stale-but-valid tip; that is safe (next real
push wakes them) but means no catch-up wake happens at login — kick manually
if work was pushed while the Mac was off.

**(b) Grok kick** — use when grok must process the current tip NOW (e.g. a
task was just pushed, or after a restart with work pending). Preferred form
(bypasses the gate entirely; verified working 2026-07-14):

```sh
grok --cwd ~/Documents/movie/automation --permission-mode acceptEdits \
  --allow 'Bash(git fetch:*)' --allow 'Bash(git push:*)' --max-turns 40 \
  -p "WAKE: fetch origin claude/eager-wozniak-74rlgj, read the newest EXCHANGE/claude message; execute any open task assigned to you per protocol and push."
```

Tip-reset variant (NEW — defined by this plan, not by POLLING.md):
`: > ~/.exchange-gate/last_seen_tip && sh ~/.exchange-gate/poll-wrapper.sh`.
**LIVE-DISPROVEN on the current wrapper** (operator test 2026-07-14 returned
`IDLE gate_exit=1` — the wrapper silently re-arms on an empty tip, the very
defect class TASK-018 fixes). It becomes the cheap kick only after TASK-018's
v1.1 wrapper lands; until then use the direct form above.

**(c) Codex kick** — same purpose for codex; its wrapper wakes on tip
difference without a reset.

```sh
sh ~/.exchange-gate-codex/poll-wrapper.sh
```

**(d) Local claude hub (talking brain / candidate integrator)** — use for
attended integration rounds: reviewing deliveries, issuing verdicts, minting
tasks at interactive latency. Take the hub lock first (§4). Until HUB_LOCK
ships (Q3), apply Q3's rule instead: cloud session live ⇒ local hub is
read-only (advise/draft only, no integration-branch pushes, no verdicts);
the operator arbitrates.

```sh
cd ~/Documents/movie/automation && git fetch origin && git checkout claude/eager-wozniak-74rlgj && git pull --ff-only && claude
```

(`claude -c` resumes the previous local hub session; `claude -p "<prompt>"`
for a one-shot question that needs no session.)

**(e) Ad-hoc interactive builder sessions** — use for "talking brain" work
with a builder: exploring an approach, debugging a failed gate, drafting.
NOT for delivering task work outside the branch/scope rules — anything that
becomes a deliverable still goes on the task branch through the gates.

```sh
# grok (top-level flags only; the `agent` subcommand rejects --cwd, exit 2)
grok --cwd ~/Documents/movie/automation --rules "$(cat /opt/agent/standing-orders.md)"
```

```sh
# codex (plain `codex` is interactive; --cd sets the working dir)
codex --cd ~/Documents/movie/automation --sandbox workspace-write --ask-for-approval on-request
```

Codex flag shapes use long form throughout; both verified against the
operator-pasted `codex help` output of 2026-07-14 (`-C/--cd`, `-s/--sandbox`,
`-a/--ask-for-approval` with `on-request` all present on this build). If a
future build rejects `--ask-for-approval`, drop it and rely on the config
default (OPERATOR-VERIFY via `codex --help` after any update).

## 4. Local-lane rules

1. **Same rails, faster clock.** Local commits follow the identical branch
   naming (`<agent>/task-NNN-<slug>`), scope rules, message-commit purity,
   and pre-push gates (`tools/gate_all.sh`, `tools/check_scope.sh`) as
   scheduled work. The local lane changes WHEN work happens, never WHAT is
   allowed. Standing orders still come only from local root-owned 444 files.
2. **Push cadence (bounds the unpushed-commit stock).** Local work MUST reach
   GitHub: (i) before any verdict is issued that references it; (ii) before
   the local session ends; (iii) on operator request; (iv) heuristic bound —
   during active work, nothing sits unpushed longer than ~30 minutes. A
   verdict on unpushed state is invalid by definition: acceptance cites a
   DELIVERED-SHA that must be reachable from origin.
3. **Integrator lock (both-hubs-alive).** Exactly one integrator may write
   the integration branch at a time. Marker: `EXCHANGE/claude/HUB_LOCK`, a
   one-line file `HOLDER: cloud|local  SINCE: <utc>  SESSION: <id>` inside
   the hub-owned namespace. Rules:
   - Default holder is **cloud** (this system's normal state).
   - Taking the lock = commit the HUB_LOCK change and push it as the FIRST
     act of a hub session. Fast-forward-only pushes on the single branch make
     git's atomic ref update the arbiter: if the push is rejected non-FF, you
     lost the race — fetch, read the current holder, stand down to read-only.
   - The CLOUD hub also pushes a SINCE-refresh of HUB_LOCK as its session's
     first act — otherwise "no cloud commit for >6 h" cannot distinguish a
     live-but-quiet cloud session from a dead one, and a local takeover could
     green-light while cloud is alive (reviewer finding, accepted).
   - The non-holder instance is read-only: it may read, advise, and draft,
     but must not push to the integration branch or issue verdicts.
   - Stale-holder takeover: if HOLDER is cloud and the integration branch has
     had no cloud commit for >6 h (heuristic; cloud sessions are ephemeral,
     R3), the operator may direct the local hub to take the lock; the
     takeover commit message must say so.
4. **Kicks are per-agent and single-shot.** Kick the agent whose task moved;
   do not blanket-kick both per event (duplicate-ACK history). One kick per
   event; if nothing happens, read the wrapper log before kicking again
   (bounded retries, no retry storm).
5. **No new writers of gate state.** Only the wrappers advance
   `last_seen_tip` (POLLING v1.1). The local claude hub never edits either
   agent's `~/.exchange-gate*` contents beyond the §3(b) tip reset (NEW,
   defined by this plan; pending operator adoption and TASK-018).

## 5. Risks and open questions

| # | Item | Type | Handling |
|---|---|---|---|
| Q1 | ~~Clone path unconfirmed~~ RESOLVED 2026-07-14: `~/Documents/movie/automation` (operator `find` output) | done | — |
| Q2 | LaunchAgents after restart: jobs reload at login but stale `last_seen_tip` means no catch-up wake; grok's clone was found 284 commits behind after one restart | Risk (med, observed) | §3a health check + §3b direct kick are the documented recovery; consider a login-time forced wake later |
| Q3 | Both-hubs-alive race window before HUB_LOCK exists | Risk (med) | Until the marker ships, rule of thumb: cloud session live ⇒ local hub is read-only; operator arbitrates |
| Q4 | Codex `--search` broken (model pin `gpt-5.6-terra` rejected by 0.140.0) | Known defect | Treat codex as no-web; route research to grok/claude; operator may fix the pin (A2) |
| Q5 | Gate-state collision A3 (duplicate-ACK root cause) not yet closed by audit | Open | Operator runs `incoming/task-016__capability-codex/gate_audit.sh` and pastes output |
| Q6 | Unpushed local work lost to Mac failure | Risk (low) | §4.2 push cadence bounds exposure to ~30 min |
| Q7 | HUB_LOCK is a convention, not an enforcement — a buggy hub could ignore it | Risk (low) | FF-only pushes + pusher-identity checks remain the hard layer; lock is coordination, not security |
| Q8 | Third-party gateways co-resident with builder credentials (R6) may spawn their own local sessions | Recorded (R6) | Out of scope here; existing recommendation stands (separate macOS user) |

## SPECIALIST-COMPLIANCE

- **Frame-before-fix (loops decision_procedure #1):** §1 fixes purpose, reference mode, boundary, and the loop inventory before any topology choice.
- **Stock/flow typing with units (#3):** §1 table types the five stocks, names inflows/outflows, and surfaces the NEW unpushed-commit stock the design must bound.
- **Loop tracing + polarity by negative-link parity (#3, #6):** B1/B2 balancing, R1 reinforcing; failure loops classified structurally (broken balancing, idempotency violation, amplifying coupling) before choosing guards.
- **Delays located, oscillation flagged (#3):** the 300 s poll delay in B2 is named as the oscillation source (revise ping-pong) and as the correct intervention point — which drives the A-over-B recommendation.
- **Idempotency before faster retries (#5):** kicks are defined as idempotent, per-agent, single-shot; wrapper guards stay in path; tip stays delivery-gated.
- **Bounded operation over unbounded growth (#8):** push-cadence bound on the unpushed stock; stale-lock takeover bound; one-kick-per-event bound.
- **Stability margin over response speed (#8):** topology B's raw speed is declined because it erodes the governance margin; guards-precede-speed stated explicitly in §1.
- **Numbers labeled heuristic (#10):** every latency and threshold above is marked heuristic; no observed timing data claimed.
- **Human review for irreversible nodes (escalation):** lock takeover, LaunchAgent changes, and any topology-B move are operator-gated.
- **ctxeng concern:** each lane keeps its context lean — kicks reuse the token-minimal wrappers (zero-token idle unchanged); interactive sessions load standing orders from local files, not repo dumps; no cross-agent context bleed (per-agent state dirs, per-agent sessions, hub reviews still receive diffs, never builder self-assessments).
