# CASE_STUDIES — initial framework analyses (SPEC Phase 6)

Resolution + first-pass analysis executed by the HUB (2026-07-13) because the
scout was unresponsive and Codex's web search is verifiably broken (model-pin
defect, TASK-016). Sources are WebSearch-verified; cells needing repo-level
reading are marked DEEP-DIVE (a scout task once Grok returns, or Phase-B
ingestion work). Grok's own TASK-015 deliverables remain open for its
self-verification + review object only.

## Resolution (was ASSUMPTIONS A4 — now closed)

- **Hermes Agent** = `NousResearch/hermes-agent` — official repo
  https://github.com/nousresearch/hermes-agent (docs:
  https://hermes-agent.nousresearch.com/). Open-source self-improving agent
  by Nous Research, released Feb 2026; >152k GitHub stars by May 2026.
  Disambiguation note: distinct from Nous's Hermes model series; this is the
  agent product.
- **OpenClaw** = `openclaw/openclaw` — official repo
  https://github.com/openclaw/openclaw (site: https://openclaw.ai/).
  Peter Steinberger's self-hosted personal AI assistant, formerly
  Clawdbot/Moltbot; MIT license; >310k stars, >1,200 contributors.
  Disambiguation note: distinct from the classic Captain-Claw engine remake
  of the same name.

## Hermes Agent (NousResearch) — Phase-6 schema

| Field | Finding | Evidence |
|---|---|---|
| Objective | Long-lived personal agent that "grows with you": learns skills from experience, persists knowledge across sessions | repo/docs tagline |
| Agent loop | Single long-lived server process; conversation-driven; self-nudging to persist knowledge | docs summary; DEEP-DIVE for loop internals |
| Planning architecture | DEEP-DIVE | — |
| Delegation model | Single-agent (no fleet) per instance | docs; DEEP-DIVE |
| Shared-state model | Own server-side store; searches its own past conversations | docs |
| Tool interface | Skills system; model-agnostic backends (Nous Portal, OpenRouter, OpenAI, custom endpoints) with no code changes | docs |
| Permission system | Self-hosted trust model; DEEP-DIVE for granular controls | — |
| Context construction | Persistent memory + retrieval over past sessions ("deepening model of who you are") | docs |
| Context compaction | DEEP-DIVE | — |
| Memory architecture | Built-in learning loop: creates skills from experience, improves them during use — procedural memory formation is the headline feature | docs |
| Parallel execution | DEEP-DIVE | — |
| Worktree/sandbox | Self-hosted server; sandboxing DEEP-DIVE | — |
| Retry/recovery, failure handling | DEEP-DIVE | — |
| Human approval | DEEP-DIVE | — |
| Observability, evals, cost controls | DEEP-DIVE | — |
| Security model | Self-hosted; multi-channel gateway (Telegram/Discord/Slack/WhatsApp/Signal/CLI) = large inbound-injection surface | docs; our THREAT_MODEL lens |
| Reusable patterns | (1) skills-from-experience = our procedural-memory layer L6 made automatic; (2) self-nudge-to-persist = a compaction-survival pattern relevant to our hub; (3) model-agnostic backend swap mirrors our router philosophy |
| Risky patterns | Automatic self-improvement writes to its own skill set — collides with our AUTONOMY immutable-set rule; adopt the LEARNING signal, keep human/gate approval on skill persistence |
| Applies to | exchange_protocol (procedural memory), specialist_factory_core (skill forging parallels), bounded_autonomy L6 design |

## OpenClaw — Phase-6 schema

| Field | Finding | Evidence |
|---|---|---|
| Objective | Self-hosted personal assistant reachable on every chat channel you already use | repo README |
| Agent loop | Gateway server process + CLI; agents configured via templates (SOUL.md configs) | repo structure; AGENTS.md in repo |
| Planning architecture | DEEP-DIVE | — |
| Delegation model | Agent templates (162+ community SOUL.md configs across 19 categories) — persona-level delegation rather than runtime fleets | awesome-openclaw-agents |
| Shared-state model | Self-hosted store; DEEP-DIVE | — |
| Tool interface | Skills system + 50+ integrations; ~23 chat channels (WhatsApp, Telegram, Slack, Discord, iMessage, Teams, Matrix, WeChat, ...) | repo/site |
| Permission system | DEEP-DIVE (critical: massive channel surface) | — |
| Context construction / compaction | DEEP-DIVE | — |
| Memory architecture | DEEP-DIVE | — |
| Parallel execution / sandbox | DEEP-DIVE | — |
| Human approval | DEEP-DIVE | — |
| Observability / evals / cost | DEEP-DIVE | — |
| Security model | MIT self-hosted; every chat channel is an untrusted-input channel — the strongest real-world stress test of our governance rule #2 | our THREAT_MODEL lens |
| Reusable patterns | (1) single gateway process fanning to many channels ≈ our INBOX-1 many-doors design, validated at 310k-star scale; (2) SOUL.md per-agent config = versioned agent personas as plain files (matches our standing-orders-as-files doctrine); (3) RL extension exists (Gen-Verse/OpenClaw-RL) for train-by-talking — watch, don't adopt |
| Risky patterns | Community agent-template marketplace = supply-chain surface (untrusted SOUL.md configs as quasi-instructions — exactly what our instruction-source rule forbids); channel breadth without per-channel privilege tiers |
| Applies to | exchange_protocol (inbox doors), bounded_autonomy security policy (channel-injection controls), study_system/plated_jewelry app messaging surfaces |

## Sources

- https://github.com/nousresearch/hermes-agent · https://hermes-agent.nousresearch.com/ · https://github.com/NousResearch/hermes-agent/releases
- https://github.com/openclaw/openclaw · https://openclaw.ai/ · https://github.com/SamurAIGPT/awesome-openclaw · https://github.com/mergisi/awesome-openclaw-agents · https://github.com/Gen-Verse/OpenClaw-RL
