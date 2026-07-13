# AI-as-productivity-SYSTEM — source directory (TASK-013)

Curated for operators running **agentic / multi-agent** workflows: orchestration,
tool use, autonomous coding/research pipelines, and agent evaluation — not
single-chat “prompt tips.”

**Method:** multi-angle web search (podcasts, newsletters, YouTube, arXiv
2025–2026 + foundational agent papers); every listed URL HTTP-checked (200)
on 2026-07-13 unless marked otherwise. Dropped or flagged if unverifiable.
Vendor-owned outlets flagged inline (`[vendor: …]`). One canonical entry per
source; cross-refs only.

**Focus filter applied:** agentic systems, multi-agent orchestration, tool
loops, autonomous build/research pipelines, evals. Generic ChatGPT prompt
lists and pure tool-roundups without an agentic angle excluded.

---

## Top-5 starter pack (hand these first)

1. **Latent Space (podcast + newsletter)** — production AI-engineer conversations on agents, harnesses, evals, and infra; closest match to “how teams actually run agent systems.”
2. **Simon Willison’s Weblog** — practitioner ground truth on tool-calling loops, prompt injection, and *agentic engineering patterns* for coding agents.
3. **MultiAgentBench (arXiv:2503.01935)** — 2025 benchmark for collaboration/competition among LLM agents; gives a measurement vocabulary for multi-agent work.
4. **SWE-agent (arXiv:2405.15793) + OpenHands (arXiv:2407.16741)** — the two must-read coding-agent scaffolds (ACI design + open multi-tool platform).
5. **AI Engineer (YouTube)** — conference talks from people shipping agents (orchestration, skills, multi-agent fleets), not tutorial fluff.

---

## 1. Podcasts (ranked; 10)

1. **Latent Space: The AI Engineer Podcast** — https://www.latent.space/podcast — weekly+ / active 2026 — Deep interviews with builders of agents, models, and agent infra; best single feed for production agentic practice.
2. **Practical AI (Changelog)** — https://changelog.com/practicalai — regular / active — Practitioner-oriented ML/AI with real deployment episodes; filter for agents, tools, and pipelines over pure research.
3. **TWIML AI Podcast** — https://twimlai.com/ — frequent / active 2026 — Long-running research+industry interviews; strong for agent security, multi-step systems, and eval framing.
4. **Cognitive Revolution** — https://www.cognitiverevolution.ai/ — frequent / active — Strategy + technical depth on agentic products and research trajectories; useful for “what is becoming system-level.”
5. **Machine Learning Street Talk (MLST)** — https://mlst.ai/ — frequent / active — Long-form technical discussions; use for multi-agent theory, tool use, and research critique (not how-tos).
6. **Invisible Machines** — https://www.invisiblemachines.ai/ — intermittent / active site 2026 — Agentic AI / orchestration-focused show (OneReach-adjacent hosts); good for enterprise multi-agent narrative. `[vendor-adjacent: OneReach.ai]`
7. **Software Engineering Daily** — https://softwareengineeringdaily.com/ — daily / active — High volume; pick agent, MCP, coding-agent, and orchestration episodes when building production harnesses.
8. **Gradient Dissent / W&B Podcast** — https://wandb.ai/site/podcast — episodic / active host site — ML ops and production ML conversations that map to agent observability and experiment discipline. `[vendor: Weights & Biases]`
9. **Latent Space (newsletter companion audio/posts)** — https://www.latent.space/ — continuous / 2026 — Same brand as #1; written deep dives when you need re-readable agent architecture notes (cross-ref podcast).
10. **IBM Think / Techsplainers (agentic explainers)** — https://www.ibm.com/think/podcasts/techsplainers — episodic — Short enterprise-oriented agentic-AI primers; useful orientation, not research depth. `[vendor: IBM]`

---

## 2. News & newsletters (ranked; 10)

1. **Import AI (Jack Clark)** — https://jack-clark.net/ — weekly / active through mid-2026 — Frontier research + policy digest; excellent for spotting agent/tool papers early. `[outlet: Anthropic co-founder as author — incentive flag, not a ban]`
2. **The Batch (DeepLearning.AI)** — https://www.deeplearning.ai/the-batch — weekly / active 2026 — Engineering-readable synthesis; regularly covers agentic coding loops and applied multi-step systems. `[vendor: DeepLearning.AI]`
3. **Interconnects (Nathan Lambert)** — https://www.interconnects.ai/ — regular / active — Post-training, RL, open weights; critical context for *why* agent stacks behave as they do.
4. **Ahead of AI (Sebastian Raschka)** — https://magazine.sebastianraschka.com/ — regular / active — Architecture and training depth; use when agent failures trace back to model limits.
5. **Simon Willison’s Weblog** — https://simonwillison.net/ — near-daily / active Jul 2026 — Living lab notebook for coding agents, MCP, tool loops, and agentic engineering patterns (canonical blog entry; not a classic “newsletter” but highest signal).
6. **Latent Space (written)** — https://www.latent.space/ — continuous / 2026 — AI-engineer essays and agent deep dives; pairs with podcast (canonical home: this URL).
7. **LangChain Blog** — https://blog.langchain.com/ — frequent / active — LangGraph, agents-in-production, observability; treat as framework-primary narrative. `[vendor: LangChain]`
8. **Anthropic Engineering / Research** — https://www.anthropic.com/engineering — intermittent+research stream — Agent skills, computer use, MCP, and eval writeups from a frontier lab. `[vendor: Anthropic]` — Research hub: https://www.anthropic.com/research
9. **Hugging Face Blog** — https://huggingface.co/blog — frequent / active — Open agents, smolagents, evals, and tooling releases; good for OSS agent ecosystem. `[vendor: Hugging Face]`
10. **TLDR AI** — https://tldr.tech/ai — daily digest / active — High-recall skim layer; follow links into primary sources, don’t stop at summaries. — Also useful secondary skim: **Last Week in AI** https://lastweekin.ai/ (weekly / verified 200).

**Skipped / not listed as confirmed feeds:** The Rundown AI (HTTP 403 at check time — do not treat as verified); OpenAI research index (403 at check — use primary papers instead).

---

## 3. YouTube channels (ranked; 10)

1. **AI Engineer** — https://www.youtube.com/@aiDotEngineer — high volume / active 2026 (talks hours old at check) — Conference-grade talks on multi-agent fleets, agent skills, harness engineering, and production agent systems.
2. **Sam Witteveen** — https://www.youtube.com/@samwitteveenai — regular / active (~125k subs) — Long-running LangChain/LangGraph and autonomous-agent tutorials with implementation detail.
3. **LangChain** — https://www.youtube.com/@LangChain — regular / active — Official LangGraph multi-agent and production-agent material. `[vendor: LangChain]`
4. **DeepLearningAI** — https://www.youtube.com/@DeepLearningAI — regular / active — Agentic AI courses and Andrew Ng-style design patterns for multi-step agents. `[vendor: DeepLearning.AI]`
5. **Prompt Engineering** — https://www.youtube.com/@PromptEngineering — regular / active — Framework comparisons and agent application interviews (e.g. agentic app stacks).
6. **Fahd Mirza** — https://www.youtube.com/@fahdmirza — frequent / active — Head-to-head multi-agent framework comparisons (AutoGen, CrewAI, LangGraph, CAMEL, etc.).
7. **Y Combinator** — https://www.youtube.com/@YCombinator — regular / active — Startup-side agent prompting, production reliability, and agent product lessons.
8. **Google DeepMind** — https://www.youtube.com/@GoogleDeepMind — regular / active — Research talks that often seed agent/tool directions. `[vendor: Google DeepMind]`
9. **Stanford Online** — https://www.youtube.com/@StanfordOnline — course drops / active — University lectures on agents, prompts, RAG (e.g. CS230 agents modules).
10. **Anthropic** — https://www.youtube.com/@anthropic-ai — intermittent / verified 200 — Lab talks on prompt/agent engineering and product agent features. `[vendor: Anthropic]` (alt handle also live: https://www.youtube.com/@Anthropic)

**Also solid (not double-counted above):** Weights & Biases https://www.youtube.com/@WeightsBiases `[vendor: W&B]`; Microsoft Developer https://www.youtube.com/@MicrosoftDeveloper (AutoGen / Azure agent stack) `[vendor: Microsoft]`.

---

## 4. Research papers (ranked; 14) — ≥8 from 2025–2026

Format: **Title — arXiv URL — submitted/year — why it matters**. Preprints unless noted; treat as preprint until you confirm venue (scholcomm).

### 2025–2026 (primary weight)

1. **MultiAgentBench: Evaluating the Collaboration and Competition of LLM agents** — https://arxiv.org/abs/2503.01935 — Mar 2025 (ACL 2025 track reported) — Benchmark + scenarios for multi-agent collaboration/competition; directly usable for eval design of multi-agent fleets.
2. **Multi-Agent Collaboration Mechanisms: A Survey of LLMs** — https://arxiv.org/abs/2501.06322 — Jan 2025 — Survey of collaboration mechanisms and an extensible framework; map for routing/orchestration choices.
3. **Agentic Large Language Models, a survey** — https://arxiv.org/abs/2503.23037 — Mar 2025 — Organizes agentic LLMs into reason / act / interact; good taxonomy when scoping a productivity agent system.
4. **Large Language Model Agent: A Survey on Methodology, Applications and Challenges** — https://arxiv.org/abs/2503.21460 — Mar 2025 — Methodology-centered agent taxonomy (architecture, collaboration, evolution, evaluation).
5. **AgentOrchestra: A Hierarchical Multi-Agent Framework for General-Purpose Task Solving** — https://arxiv.org/abs/2506.12508 — Jun 2025 — Hierarchical planner + modular specialists pattern; close to how multi-role agent fleets are wired.
6. **The Orchestration of Multi-Agent Systems: Architectures, Protocols, and Enterprise Adoption** — https://arxiv.org/abs/2601.13671 — Jan 2026 — Architecture + MCP/A2A-style protocol discussion for tool access and peer agent coordination.
7. **The Evolution of Tool Use in LLM Agents** — https://arxiv.org/abs/2603.22862 — 2026 — Survey/review of multi-tool LLM agents; state of tool orchestration (core of “agent as system”).
8. **Experience as a Compass: Multi-agent RAG with Evolving Memory** — https://arxiv.org/abs/2604.00901 — Apr 2026 — Adaptive multi-agent RAG orchestration and behavior-level learning limits — relevant to long-running research agents.
9. **LLM Agents Making Agent Tools** — https://arxiv.org/abs/2502.11705 — Feb 2025 (ACL 2025 noted) — Agents that mint tools from papers/code; meta-tooling for autonomous research pipelines.
10. **Can Agents Fix Agent Issues?** — https://arxiv.org/abs/2505.20749 — May 2025 — Shows SE agents struggle on agent-system bugs; cautionary eval for self-maintaining agent stacks.
11. **Measuring AI Ability to Complete Long Software Tasks** — https://arxiv.org/abs/2503.14499 — Mar 2025 — Long-horizon software task measurement — frames expectations for autonomous coding agents.
12. **APIGen-MT: Agentic Pipeline for Multi-Turn Data Generation via Simulated Agent-Human Interplay** — https://arxiv.org/abs/2504.03601 — Apr 2025 — Agentic multi-turn data pipeline; useful pattern for training/eval data generation with agents.

### Foundational / still-load-bearing (≤2024; keep for lineage)

13. **SWE-agent: Agent-Computer Interfaces Enable Automated Software Engineering** — https://arxiv.org/abs/2405.15793 — May 2024 — ACI design that made repo-level coding agents work; still the interface-design reference.
14. **OpenHands: An Open Platform for AI Software Developers as Generalist Agents** — https://arxiv.org/abs/2407.16741 — Jul 2024 — Open multi-tool coding-agent platform (ex-OpenDevin); multi-agent coordination + sandbox eval harness.

**Strong secondary foundations (verified, not ranked in the 14 to keep the list tight):**  
ReAct https://arxiv.org/abs/2210.03629 · MetaGPT https://arxiv.org/abs/2308.00352 · AutoGen https://arxiv.org/abs/2308.08155 · Voyager https://arxiv.org/abs/2305.16291 · AgentBench https://arxiv.org/abs/2308.03688 · Agentless https://arxiv.org/abs/2407.01489 · Mixture-of-Agents https://arxiv.org/abs/2406.04692 · The AI Scientist https://arxiv.org/abs/2408.06292 · Magentic-One https://arxiv.org/abs/2411.04468 · LLM multi-agent survey https://arxiv.org/abs/2402.01680 · Tree of Thoughts https://arxiv.org/abs/2305.10601 · MCP intro (product/protocol, not arXiv) https://modelcontextprotocol.io/ `[vendor-origin: Anthropic]`.

**cs.MA live feed for ongoing discovery:** https://arxiv.org/list/cs.MA/recent (verified 200) — use with fielded queries; do not cite from titles alone without opening the abs page.

---

## Verification log (abbrev.)

- Checked 2026-07-13 via HTTP GET (follow redirects); **included only if status 200** unless explicitly noted.
- Dropped from confirmed list: `practicalai.fm` (525), `youtube.com/@AnthropicAI` bare slug 404 (handles above work), Apple Invisible Machines id 404, OpenAI research index 403, The Rundown 403, several Latent Space Spotify show id 404 (Apple + latent.space/podcast OK).
- Papers: arXiv abs pages 200 for all primary IDs listed; venue acceptance noted only when visible on abs/page text (still treat as need-to-confirm for load-bearing claims).

---

## SPECIALIST-COMPLIANCE:

- **libarch:** multi-angle search per section; ranked selection; credibility/vendor screening; stop when section quotas met with verified links; dropped unverifiable items rather than padding.
- **scholcomm:** arXiv IDs for papers; preprint vs “reported ACL” distinguished; unknown venue → not over-claimed as published; UNVERIFIED/HTTP-fail items excluded from confirmed list.
- **worldmodel:** vendor-owned and vendor-adjacent outlets flagged inline (`[vendor: …]` / incentive notes); not banned; operator can weight accordingly.
