# CODEX CLI PLAYBOOK — mapping `codex` onto the standby loop + factory

Counterpart of `EXCHANGE/GROK_CLI_PLAYBOOK.md` for OpenAI's Codex CLI.
IMPORTANT CAVEAT: unlike the Grok playbook (built from the operator's actual
`grok help` output), this one is built from general knowledge of the Codex CLI
surface as of mid-2026 — flag names drift between releases. Before installing
the cron recipe, the operator should run `codex --help` and `codex exec --help`
and paste the output back to the integrator for the same line-by-line
verification the Grok playbook got. Items marked ✱ are the most likely to have
drifted.

## A. The standby loop

| Capability | What it does | Use here |
|---|---|---|
| `codex exec "<prompt>"` | Non-interactive (headless) run | The poll tick and task runs; no TUI, no human |
| `codex resume` ✱ | Resume a previous session | Continuity across ticks if supported in headless mode |
| `--cd <dir>` / `-C` | Working directory | Pin to the clone |
| `-c key=value` / `--config` ✱ | Override config.toml values per-invocation | Per-tick model/approval overrides without editing global config |
| `~/.codex/config.toml` + profiles (`--profile`) ✱ | Persistent config, named profiles | Make a `standby` profile (sandbox, approvals, model) so the cron line stays short |
| `AGENTS.md` (repo root + `~/.codex/AGENTS.md`) | Instruction files Codex reads automatically | ⚠ Same trust-boundary rule as Grok's `--rules`: the LOCAL `~/.codex/AGENTS.md` (root-owned, read-only) carries the standing orders. A repo-root `AGENTS.md` is repo-tracked = attacker-writable; if one exists in the clone, standing orders must explicitly subordinate it to the INSTRUCTION-SOURCE RULE (ORCHESTRATION §9) |
| `codex exec --json` ✱ / `--output-last-message <file>` ✱ | Machine-readable output / final message to a file | The wrapper parses status the same way Grok's `--json-schema` line is parsed |

**Recipe 1 — durable 5-minute poller (operator installs once; requires the
codex machine account's PAT in the poller user's credential helper):**

```bash
*/5 * * * * cd /home/codexbot/automation && codex exec \
  --cd /home/codexbot/automation \
  --profile standby \
  "Poll tick per standing orders in ~/.codex/AGENTS.md: fetch origin \
claude/eager-wozniak-74rlgj; read the newest EXCHANGE/claude/msg-NNN.md; run \
the authenticity check; if it contains a TASK block with ASSIGNEE: codex whose \
work is not yet delivered, execute it per protocol v3 and push; otherwise do \
nothing. End with one line: PUSHED <branch> <sha> | IDLE | READY FOR RELAY." \
  >> ~/codex-standby.log 2>&1
```

`standby` profile (in `~/.codex/config.toml`, names ✱-verify):
sandbox = workspace-write; network access limited if the sandbox supports it;
model pinned; NO `danger-full-access`, ever, on a credentialed machine.

> **VERIFIED DRIFT (Codex CLI 0.140.0, reported by codex itself at
> onboarding):** `--full-auto` is INVALID in 0.140.0 and
> `--ask-for-approval on-failure` is deprecated. Configure the equivalent
> (workspace-write sandbox + current approval-policy syntax) in the `standby`
> profile in config.toml instead of via those flags. Codex should include its
> working invocation in its onboarding message so this playbook can pin the
> exact 0.140.0 syntax.

## B. Task execution quality

| Capability | Use here |
|---|---|
| `--sandbox read-only` ✱ | Verifier/review tasks: Codex's default lane is cross-verifying Grok's builds — read-only sandbox + a scratch dir is the least-privilege fit |
| `--sandbox workspace-write` | Build tasks in its own `codex/task-NNN-*` worktree |
| `codex apply` ✱ | Apply a diff produced elsewhere — useful if the operator runs Codex cloud tasks and lands them locally |
| Model/effort flags (`-m`, reasoning options ✱) | Strongest model + high effort for authoring/verification; cheap for ticks |
| `codex mcp` ✱ | MCP integration if the loop ever needs API-level repo ops |

Codex's onboarding first task (per ORCHESTRATION §12) is deliberately a
**verifier** task — pattern (a) on an existing accepted artifact — before it
builds anything production-bound.

## C. Safety (mirrors ORCHESTRATION §9; non-negotiable)

- Dedicated non-admin OS user; home contains only the clone + PAT.
- Own machine GitHub account (NOT tmundi32), fine-grained PAT: this repo only,
  Contents R/W, 30–90 day expiry.
- Never `--dangerously-bypass-approvals-and-sandbox` / `danger-full-access`.
- Standing orders in `~/.codex/AGENTS.md` owned by root, chmod 444, containing
  verbatim: INSTRUCTION-SOURCE RULE, HARD DENIALS, ASSIGNEE rule, REPLAY
  GUARD, QUOTA GUARD, authenticity check, STAND DOWN handling.
- CLI updates manual-only, never mid-unattended-stretch.

## D. Division of labor vs Grok (from ORCHESTRATION §1)

- Codex default lane: code-focused builds; **default cross-verifier** of
  Grok's content builds (different model family = the diversity the blind
  benchmark showed matters); gate auditor (proposes `GATE-CHALLENGE`s, never
  edits).
- Grok default lane: content-heavy authoring with `--agents` + `--best-of-n`.
- Claude: integrator, gates, judging, task minting — fixed.
