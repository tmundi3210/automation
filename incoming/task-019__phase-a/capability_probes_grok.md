# capability_probes_grok.md — Phase A plan/rate-limit probes (assumption A5)

**Task:** TASK-019  
**Agent:** grok  
**Verified at (UTC):** 2026-07-15T19:58Z  
**Host:** operator Mac (user-scope Terminal; not launchd)  
**Method:** harmless non-mutating probes only (config inspection, help, version, models, launchctl print)

Verification tags per probe: **VERIFIED(value + evidence)** or **UNVERIFIED(what was tried)**.

---

## Probes

| # | Limit / field | Result | Evidence |
|---|---|---|---|
| P1 | CLI product + version | **VERIFIED** `grok 0.2.101 (5bc4b5dfadcf) [stable]` | `/Users/mundi/.grok/bin/grok --version` |
| P2 | Auth / plan identity surface | **VERIFIED** logged in with grok.com; models list returns without error | `grok models` first line: `You are logged in with grok.com.` |
| P3 | Default model | **VERIFIED** `grok-4.5` | `grok models` → `Default model: grok-4.5` |
| P4 | Available models | **VERIFIED** `grok-4.5` (default), `grok-composer-2.5-fast` | `grok models` available list (2 entries) |
| P5 | Numeric rate limits / quota caps | **UNVERIFIED** — no numeric RPS, daily tokens, or plan-tier caps exposed | Tried: `grok models` (no numbers); `rg -ni 'rate\|limit\|quota\|budget' ~/.grok/config.toml` (only unrelated `fork_secondary_model` / subagent `plan = true`); `grok --help` rate/limit keywords show only `--max-turns`, model flags, tool allow/deny — no quota numbers |
| P6 | Config-declared permission mode | **VERIFIED** `permission_mode = "always-approve"` in `~/.grok/config.toml` | config key read (no secrets printed) |
| P7 | Headless safety knobs present | **VERIFIED** CLI accepts `--max-turns`, `--permission-mode`, `--sandbox`, `--output-format`, `--allow`/`--deny` | `grok --help` flag list |
| P8 | Scheduled poller job present | **VERIFIED** LaunchAgent `com.mundi.exchange-poll-v1` installed; `run interval = 300 seconds`; `last exit code = 0` | `launchctl list \| rg exchange`; `launchctl print gui/$(id -u)/com.mundi.exchange-poll-v1` |
| P9 | Poller repo pin | **VERIFIED** `EXCHANGE_REPO => /Users/mundi/Documents/movie/automation`; gate dir `~/.exchange-gate` | launchctl print env |
| P10 | Poller runtime health (content) | **VERIFIED(as of probes)** ticks fire, but **fetch fails under launchd TCC** with `GATE-ERROR op=fetch rc=128 reason=fatal: Unable to read current working directory: Operation not permitted` | `~/.exchange-gate/poll.log` 2026-07-15T19:00Z–19:56Z rows (see submit msg live-proof section) |
| P11 | Trivial non-mutating CLI call | **VERIFIED** exit 0, models listed | `grok models` re-run after help inspection |

---

## Interpretation for Phase A (A5)

- **What is known:** product version, auth surface, model inventory, config permission mode, CLI bounding flags, launchd interval, and current poller failure mode (TCC on Documents path — hub diagnosis in msg-058; out of this task's fix scope).
- **What remains open:** numeric plan rate limits / spend caps. No local config or CLI surface published them during this probe set. Do **not** invent numbers. Treat A5 rate-limit numerics as still **UNVERIFIED** for cost-budget math until the product exposes them or an operator provides plan docs.
- **Not claimed:** any sandbox isolation strength beyond flag presence; any remote API quota; any X firehose rate.

---

## Commands run (redacted paths only where required)

```text
grok --version
grok models
grok --help | head
rg -n '^(#|[a-zA-Z_\[])' ~/.grok/config.toml   # structure only
rg -ni 'rate|limit|quota|plan|budget|max_turn|model' ~/.grok/config.toml
launchctl list | rg exchange
launchctl print gui/$(id -u)/com.mundi.exchange-poll-v1
```

No mutating CLI calls. No secrets written to the repo.
