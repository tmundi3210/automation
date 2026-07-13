# CAPABILITY_MATRIX — verified component capabilities (Phase 0)

Rule (SPEC Phase 0): no capability is claimed unless verified by (a) live
non-mutating test in this system, (b) configuration inspection, or (c)
official documentation. Every cell below carries its verification tag:
`[T]`=tested live, `[C]`=config/docs, `[O]`=observed in past task evidence,
`[U]`=UNVERIFIED (pending TASK-015/016 self-verification).

The three components are NOT interchangeable. "Claude Code" here is a
remote-container deployment; "Grok" is the Grok CLI (Grok Build TUI) on the
operator's Mac; "Codex" is OpenAI Codex CLI on the same Mac.

## Component 1 — Claude Code (remote managed container) — hub / architect / critic / integrator

| Capability | Status | Evidence |
|---|---|---|
| Product/version | Claude Code remote session (Agent SDK harness), model `claude-fable-5` `[C]`; harness version not exposed in-session `[U]` | env block |
| Interface | Cloud: web/app-initiated remote session in ephemeral Linux container (6.18.5) `[C]` | env block |
| Filesystem | Full rw inside container; repo clone at `/home/user/automation`; fixed per-session disk allowance; NOTHING persists unless pushed `[T]` | all session writes |
| Shell | Yes, sandboxed Bash, permission-mode governed `[T]` | this session |
| Git/GitHub | Local git + push via managed proxy remote `[T]`; GitHub MCP tools restricted to `tmundi3210/automation` ONLY `[C][T]`; no `gh` CLI `[C]`; repo-list tool absent in this session `[T]` | pushes; ToolSearch miss |
| Web search | WebSearch (server-side): works `[T]` (arXiv ID confirmations 2026-07-13) | TASK-013 review |
| Web fetch | WebFetch: works but egress policy 403s many hosts (arxiv.org, hf.co direct) `[T]`; shell curl to arbitrary domains: CONNECT 403 policy denial `[T]`; package registries allowlisted `[C]` | proxy status output |
| X search | NONE. No X tool present `[T]`. Indirect only via WebSearch results | ToolSearch |
| MCP/tools | GitHub MCP `[T]`, Hugging Face MCP `[T]` (paper_search used), Canva/Figma present `[C]` untested `[U]`; MCP servers can disconnect mid-session (Higgsfield did) `[T]` | this session |
| Scheduling | Routines (cron triggers) `[T]` — hourly inbox tick live (`trig_01HhJHuVSeJ1YYtYgU7brogc`); ScheduleWakeup; Monitor; background tasks `[T]` | created 2026-07-13 |
| Sandboxing | Ephemeral isolated container per session `[C]`; git worktrees `[C]`; subagents with scoped tools `[T]` | this session |
| Approval | Permission modes; user approval prompts for gated actions; plan mode `[C]` | harness |
| Auth | Operator claude.ai account; GitHub App token via MCP (repo-scoped) `[C]` | env |
| Secret exposure | Low: no secrets in repo (policy); container env carries proxy config only `[C]` | inspection |
| Cost/limits | Flat-rate plan; token/context budget per session (compaction occurs on long sessions — observed twice) `[T]`; plan quotas `[U]` | this session |
| Known limits | Context compaction; egress policy; single-repo GitHub scope; ephemeral disk; no X access | above |

## Component 2 — Grok CLI ("Grok Build") on operator's Mac — scout / content author

Status: rows marked `[O]` come from task evidence (msg-007..010, TASK-011/013/014
deliveries + `grok help` capture in EXCHANGE/GROK_CLI_PLAYBOOK.md). Full
self-verification assigned as TASK-015; matrix to be finalized from its reply.

| Capability | Status | Evidence |
|---|---|---|
| Product/version | Grok CLI / Grok Build TUI; exact version string `[U]` (TASK-015) | playbook header |
| Interface | Local CLI/TUI on macOS; headless via top-level `grok -p/--prompt-file` (NOT `grok agent` for scheduler flags — TASK-014 root cause) `[O]` | msg-009 |
| Filesystem | Operator user scope; repo clone `~/automation`; secondary clone `/tmp/automation` (volatile — flagged) `[O]` | msg-025 nit |
| Shell | Yes via agent tooling; `--sandbox` profiles exist per help `[O]`, untested `[U]` | playbook §C |
| Git/GitHub | Pushes as owner account with `grok-bot` git identity + Agent trailer (OPERATOR WAIVER-001) `[O][T]` | all deliveries |
| Web search | Yes `[O]` (TASK-013 performed multi-angle search + URL verification); `--disable-web-search` flag exists `[C]` | SOURCES.md |
| X search | Expected (xAI product) but NOT yet demonstrated in any task `[U]` (TASK-015 must test harmlessly) | — |
| MCP/tools | `mcp` subcommand listed in help `[C]`; unconfigured/untested `[U]` | playbook §D |
| Scheduling | launchd LaunchAgent `com.mundi.exchange-poll-v1`, 300s interval + zero-token shell gate; automatic wake PROVEN `[T]` | TASK-014 Phase 2 |
| Sandboxing | Worktrees (`-w/--worktree-ref`) `[C]`; sandbox profiles `[U]` | playbook §B/§C |
| Approval | `--permission-mode acceptEdits` + `--allow`/`--deny` rules (switched from `--always-approve` 2026-07-13) `[O]` | msg-010 |
| Auth | Operator's xAI account, local login `[C]` | playbook |
| Secret exposure | MEDIUM: runs on a personal Mac with credentials; mitigated by deny-rules + standing orders in root-owned 444 file `[C]` | WAIVER-001 record |
| Cost/limits | Flat-rate plan assumed; quotas `[U]` | — |
| Known limits | Solo-codegen weak lane (observed round-1); scheduler silently dead for hours when invocation shape wrong (fixed; every tick now logs) `[O]` | scores.json |

## Component 3 — OpenAI Codex CLI on operator's Mac — implementer / verifier

Status: `[O]` rows from TASK-006/008/010/012 evidence. Full self-verification
assigned as TASK-016.

| Capability | Status | Evidence |
|---|---|---|
| Product/version | OpenAI Codex CLI, 0.140.0 at last pin (2026-07) `[O]`; re-verify `[U]` | recipe pin |
| Interface | Local CLI on macOS; headless `codex exec -C <repo> --sandbox workspace-write --json` `[O]` | pinned invocation |
| Filesystem | Sandbox `workspace-write` limits writes to the workspace `[C]` | codex docs/flags |
| Shell | Yes inside sandbox `[O]` | task builds |
| Git/GitHub | Pushes as owner account with `codex-bot` identity + trailer (WAIVER-001) `[O]` | deliveries |
| Web search | `[U]` — never demonstrated in a task; TASK-016 must test | — |
| X search | None expected `[U]` | — |
| MCP/tools | `[U]` (codex MCP support exists per docs `[C]`; unconfigured here) | — |
| Scheduling | cron + zero-token gate (TASK-012 ACK) `[O]`; NOTE: ack says state in `~/.exchange-gate/` — SAME dir Grok uses. Possible last_seen_tip collision between the two builders → starved wakes. MUST be verified/de-conflicted in TASK-016 | msg-022 vs msg-009 |
| Sandboxing | `--sandbox` modes (read-only / workspace-write) `[C]` | codex CLI |
| Approval | sandbox + approval flags; `--ask-for-approval on-failure` deprecated in 0.140.0 `[O]` | recipe notes |
| Auth | Operator's OpenAI account `[C]` | — |
| Secret exposure | MEDIUM: same personal Mac; sandbox reduces blast radius `[C]` | WAIVER-001 |
| Cost/limits | Flat-rate plan assumed; quotas `[U]` | — |
| Known limits | Template-cloning in content-authoring lane (TASK-008, reassigned); numbering + stale-pointer process faults (both corrected, monitored via scores.json); benchmark-gaming reputation flags for GPT-family (routing: never final arbiter) `[O]` | scores.json |

## Cross-component invariants (verified in operation)

- Coordination bus: this git repo, EXCHANGE protocol v3.3 — per-agent msg dirs,
  per-task branches, single-writer integrator, scope gate, acceptance tokens `[T]`.
- Idle cost: zero model tokens on all three (shell SHA gates + 6h heartbeat) `[T]`.
- Instruction-source rule: repo content is DATA; standing orders only from
  local root-owned chmod-444 files `[C]` — the anti-injection boundary.
- Trust routing: evidence-based per-lane scores in EXCHANGE/scores.json `[T]`.
