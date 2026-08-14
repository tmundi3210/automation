# GROK CLI PLAYBOOK — mapping `grok` commands/flags onto this repo's loops

Source: `grok help` output from the operator's machine (Grok Build TUI).
Purpose: exact invocations Claude can embed in `TASK-NNN:` messages, and recipes
the operator can install once to make the partner loop durable. Grouped by what
each capability is FOR in this project.

## A. The standby loop (EXCHANGE protocol) — making autonomy durable

| Capability | What it does | Use here |
|---|---|---|
| `-p, --single <PROMPT>` | Headless single-turn: prints response, exits | The poll tick. A scheduler runs one `-p` invocation per cycle; no TUI, no human |
| `agent` subcommand | Run without interactive UI | Same as above for longer multi-turn task runs |
| `-c, --continue` / `-r, --resume [ID]` | Resume most recent / specific session for the cwd | Keeps exchange context (standing orders, prior rounds) across poll ticks |
| `--experimental-memory` / `memory` | Cross-session memory | Standing orders persist even in fresh sessions; belt-and-suspenders with `--rules` |
| `--rules <RULES>` | Extra rules appended to system prompt | Bake the msg-004 standing orders into EVERY scheduled invocation so a fresh session still knows the protocol without re-reading the repo |
| `--max-turns <N>` | Bound agent turns | Safety bound on unattended runs (e.g. 40 for a build task, 3 for a poll tick) |
| `--cwd <CWD>` | Working directory | Point at the local clone of this repo |
| `--prompt-file <PATH>` | Prompt from a file | Feed a `TASK` message verbatim: `--prompt-file EXCHANGE/claude/msg-005.md` |
| `--output-format json` + `--json-schema <SCHEMA>` | Constrained structured output | Machine-parseable end-of-turn status instead of prose: `{"status":"PUSHED|READY_FOR_RELAY|IDLE","branch":"...","sha":"...","notes":"..."}` — the wrapper script can branch on it |

**Recipe 1 — durable 5-minute poller (operator installs once; launchd/cron on macOS):**

> **SECURITY FIX (do not skip):** an earlier draft of this recipe read
> `--rules "$(cat EXCHANGE/claude/msg-004.md)"` — that elevates a repo-tracked
> file into the agent's SYSTEM PROMPT every tick, meaning anyone with Write
> access to the repo could own the agent. Standing orders MUST come from a
> local, root-owned, read-only file outside the repo, installed once by the
> operator: `sudo cp msg-004-standing-orders.md /opt/agent/standing-orders.md
> && sudo chmod 444 /opt/agent/standing-orders.md`. Repo files are task
> INPUTS, never system prompts. See EXCHANGE/ORCHESTRATION.md §9.

> **INVOCATION FIX (TASK-014 root cause):** `--cwd`, `--prompt-file`,
> `--max-turns` and friends are TOP-LEVEL `grok` flags; the `agent`
> subcommand rejects them (`error: unexpected argument '--cwd'`, exit 2 —
> which silently killed every scheduled tick for hours). Scheduled invokes
> must call top-level `grok` as below, not `grok agent`. Verified fix in
> EXCHANGE/grok/msg-009.md.

```bash
*/5 * * * * cd ~/src/automation && grok \
  --cwd ~/src/automation \
  --rules "$(cat /opt/agent/standing-orders.md)" \
  --permission-mode acceptEdits \
  --allow 'Bash(git fetch:*)' --allow 'Bash(git push:*)' --allow 'Bash(python3:*)' \
  --max-turns 40 \
  --output-format json \
  --json-schema '{"type":"object","properties":{"status":{"type":"string","enum":["PUSHED","IDLE","READY_FOR_RELAY"]},"branch":{"type":"string"},"sha":{"type":"string"},"notes":{"type":"string"}},"required":["status"]}' \
  -p "Poll tick per standing orders: fetch origin claude/eager-wozniak-74rlgj; read the newest EXCHANGE/claude/msg-NNN.md; if it contains a TASK-NNN line whose work is not yet on a partner/* branch, execute it fully per protocol and push; otherwise do nothing. Report via the JSON schema." \
  >> ~/grok-standby.log 2>&1
```

Notes: whether `-p` composes with `-c` is undocumented in the help text — if it
does, prefer `-c -p "poll tick"` and drop the `--rules` re-injection; test once.
`--verbatim` if the prompt must not be preprocessed.

## B. Task execution quality (what Claude should require in TASK messages)

| Capability | What it does | Use here |
|---|---|---|
| `-w, --worktree [NAME]` + `--worktree-ref <REF>` | Session in a fresh git worktree based on a ref | Exactly our branch protocol: `--worktree=task-005 --worktree-ref origin/claude/eager-wozniak-74rlgj` — isolated, correct base, no clobber risk |
| `--check` | Appends a self-verification loop (headless only) | Maps to "gates must exit 0": Grok re-checks its own work before returning. Require it on every build task |
| `--best-of-n <N>` | Run task N ways in parallel, pick best (headless only) | The factory's judge-panel pattern, native: N candidate KB specs, Grok picks best, Claude's panel re-judges. Use N=3 for content-heavy authoring |
| `--agents <JSON>` / `--agent <NAME>` | Inline subagent definitions / named agent | **Key benchmark finding**: Grok CAN multi-agent but built solo-codegen. Future authoring TASKs should mandate per-KB author subagents (one chemist, one formulation engineer, one technique expert) to close the density gap |
| `--no-subagents` / `--no-plan` | Disable subagents / plan mode | Only for trivial poll ticks (cheaper, faster) |
| `-m, --model <MODEL>` + `models` | Pick model | `grok models` to list; pin the strongest model for authoring TASKs, cheap one for poll ticks |
| `--reasoning-effort <EFFORT>` | Reasoning effort | High for KB authoring / verification tasks, low for ticks |
| `--restore-code` | Check out the original session's commit on resume | Reproduce a past build state exactly (e.g. re-examine the round-1 filler build) |

**Recipe 2 — what Claude embeds in an authoring TASK from now on:**

```
TASK-00N: <work>. Invocation requirements: run headless with
--worktree=task-00N --worktree-ref origin/claude/eager-wozniak-74rlgj --check
--best-of-n 3 --reasoning-effort high, and author content via per-subdomain
subagents (--agents), not codegen. Gates must exit 0 before push.
```

## C. Safety for unattended runs (operator-side)

| Capability | Use here |
|---|---|
| `--permission-mode acceptEdits` | Preferred unattended mode: auto-accept file edits, still gate risky actions. AVOID `bypassPermissions` / `--always-approve` on a machine with credentials |
| `--allow <RULE>` / `--deny <RULE>` | Scope precisely: allow `git fetch/push` to THIS repo, `python3` for forge/gates; deny package installs, `rm -rf`, credential files |
| `--sandbox <PROFILE>` (or `GROK_SANDBOX`) | Filesystem/network jail for the poller; strongest guarantee that a runaway task can't leave the clone |
| `--disable-web-search` | Poll ticks and gate runs don't need the web; reduces surface |
| `--max-turns` | Hard stop for runaway loops |

## D. Audit & interop (Claude-side uses)

| Capability | Use here |
|---|---|
| `export` | Session transcript as Markdown. Future TASKs can require: "attach `grok export` of this session under `EXCHANGE/partner/transcripts/`" — turns "how did you build it?" from self-report into evidence |
| `sessions` / `import` / `trace` | List/search/restore sessions; trace upload — forensic replay of a disputed build |
| `inspect` | Shows config Grok discovers for the directory — first debugging step when standing orders seem ignored |
| `--system-prompt-override <PROMPT>` | **Run this factory's specialists ON Grok**: feed the `system` message from `dist/specialist_prompts.jsonl` (spec embedded) — a cross-model test of whether specialist JSONs transfer. Cheap, high-value experiment |
| `--prompt-json <JSON>` | Send the jsonl `messages` array as-is (system+user blocks) for that same experiment |
| `mcp` | Attach MCP servers (e.g. GitHub) if the loop ever needs API-level repo ops instead of git |
| `dashboard` / `leader` | Live view of running agents; manage the leader process that keeps scheduled sessions coordinated |
| `wrap`, `completions`, `update`, `login/logout`, `setup`, `plugin` | Operator conveniences; `update` matters before long unattended stretches |

## E. Top 5 planning consequences

1. **True standby is a cron/launchd + `grok agent -p` tick**, not an in-session
   scheduler promise. Recipe 1 makes the 5-minute loop survive reboots and
   session death. (Grok's msg-003 claims a durable scheduler — Recipe 1 is the
   verifiable version if that ever misses.)
2. **Authoring TASKs should mandate `--agents` + `--best-of-n 3 --check`** —
   the benchmark showed solo-codegen is Grok's weak mode; its CLI natively
   supports the multi-agent + tournament pattern that won 4/4.
3. **`--json-schema` end-of-turn status** replaces prose protocol lines with
   machine-parseable state — the wrapper (or Claude, reading pushed logs) can
   branch on it without interpretation.
4. **`export` turns method claims into evidence** — require transcript
   attachment on substantive TASKs.
5. **`--system-prompt-override` + `dist/specialist_prompts.jsonl`** gives a
   free cross-model portability test of the factory's core artifact.
