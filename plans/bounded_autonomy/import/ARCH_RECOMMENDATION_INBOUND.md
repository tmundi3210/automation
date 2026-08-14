# ARCH_RECOMMENDATION_INBOUND — operator-pasted architecture analysis (verbatim)

Received: 2026-07-14, Door A (pasted directly to the hub).
Provenance: external AI-generated analysis supplied by the operator
(citation style and `utm_source=chatgpt.com` markers indicate a ChatGPT-family
origin; not produced by this system). Treated as operator-supplied RESEARCH
INPUT: its claims about third-party projects (Ultraswarm, Claude Code Fusion,
Deliberation, Sub-Agents Skills, ACPX, OpenClaw docs, Hermes issue #413) and
all bracketed citations are UNVERIFIED from this container (egress
policy-blocked) and are recorded as claims, not facts.
Hub assessment: `plans/bounded_autonomy/ARCH_REVIEW.md`.

Everything below this line is the operator's paste, unaltered.

---

## Conclusion
The best architecture is **not** a free-form conversation among Claude, Grok, and Codex. Use them as independently callable workers beneath a deterministic controller.
For your system, the strongest arrangement is:
```text
Telegram / Discord / Mac / scheduler
              │
       OpenClaw — optional outer gateway
              │
    Deterministic orchestration service
    state • routing • budgets • permissions
              │
     ┌────────┼─────────┐
     │        │         │
   Grok     Claude    Codex
 research   planning   coding
 X signal   critique   tests
     │        │         │
     └────────┼─────────┘
              │
   Separate worktrees/containers
              │
 Deterministic tests and evaluation
              │
       Draft PR / report
              │
         Human approval
```
Use:
* **OpenClaw** as the optional external control surface: messaging channels, scheduling, persistent sessions, remote commands, and agent-session routing.
* A **small custom controller** as the source of truth for tasks, state, results, budgets, permissions, and approvals.
* **Claude Agent SDK** for Claude.
* **Codex SDK or Codex app-server** for Codex.
* **Grok ACP for coding** and the **xAI API with X Search for intelligence gathering**.
* **Hermes optionally for memory, learned skills, and personal automations**, but not initially as the central three-agent coding orchestrator.
Terminal CLIs remain useful, but primarily as execution backends and debugging interfaces—not as the system's permanent communication architecture.
---
# 1. What people are actually doing
Current implementations mostly follow four patterns.
## Pattern A: One lead agent delegates to other models
One agent remains the user-facing lead. It asks the other models for specialist outputs.
Examples reviewed include systems where:
* Claude serves as technical lead.
* Codex receives bounded coding tasks.
* Grok performs research, alternative analysis, or overflow implementation.
* Results return to the lead for synthesis.
Claude Code Fusion explicitly uses a Claude-led pattern with Codex as a primary implementation lane and Grok as an independent opinion or research lane. Deliberation exposes multiple models as expert tools to a Claude host through MCP, obtains independent answers, and synthesizes or compares them. These are useful examples of the pattern, although neither should be treated as an industry standard. ([GitHub][1])
This is the easiest configuration to understand, but it has one weakness: the lead model can distort, omit, or misinterpret the other agents' findings.
## Pattern B: A standalone runner manages all agents
A separate program—not one of the models—owns:
* Task decomposition
* Agent routing
* Process supervision
* Worktrees
* Dependencies
* Review stages
* Testing gates
* Approval gates
* Recovery
* Token and cost accounting
Ultraswarm is a current open-source example. It can run Claude Code, Codex, Grok, and shell workers while maintaining task state in SQLite, assigning separate Git worktrees, applying quality gates, and tracking results. Its architecture is more relevant than its specific implementation: **the runner owns state; the models are replaceable workers**. ([GitHub][2])
This is the best architectural pattern for your long-term system.
## Pattern C: An always-on personal gateway launches coding agents
OpenClaw uses a persistent gateway/control-plane model. It supports isolated agents, channel bindings, session stores, persistent sessions, background work, and external coding harnesses. Its ACP integration is specifically intended to run external agents such as Claude Code and Codex rather than pretending those tools are merely text-completion APIs. ([OpenClaw][3])
OpenClaw's own documentation favors its native Codex integration where available and uses ACP/ACPX for external harnesses. The ACPX repository lists native Grok Build support through Grok's `grok agent stdio` interface. The primary OpenClaw harness table has not always listed Grok alongside Claude and Codex, so the exact Grok adapter and permissions should be validated on the installed versions rather than assumed. ([OpenClaw][4])
This is currently the closest fit when you need:
* Telegram, Discord, or another remote interface
* Scheduled agent runs
* Persistent named sessions
* Multiple project workspaces
* Human approvals through chat
* A personal assistant around the coding agents
## Pattern D: A memory-first personal agent invokes coding CLIs
Hermes Agent emphasizes persistent memory, skill creation, cron jobs, subagents, messaging integrations, terminal access, and multiple execution backends. It is relevant to your vision because it is intended to learn reusable skills and retain useful information over time. ([GitHub][5])
However, Hermes' own cross-CLI orchestration discussion identifies current gaps: external Claude and Codex skills have been largely sequential, while mixed parallel workflows, richer dependency graphs, output passing, and inter-agent communication remain areas under development. ([GitHub][6])
Therefore:
* Hermes is strong as a **memory, skills, scheduled research, and personal-automation layer**.
* It is not presently my first choice as the sole control plane for a complex Claude–Codex–Grok coding pipeline.
---
# 2. CLI, MCP, ACP, SDK, or direct API?
They serve different purposes.
| Interface                    | Correct use                                                                         | Limitation                                                                                         | Recommendation                                           |
| ---------------------------- | ----------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | -------------------------------------------------------- |
| Interactive terminal/TUI     | Human-supervised coding, debugging, inspecting agent behavior                       | Difficult to automate reliably; terminal output and prompts can change                             | Keep for manual operation                                |
| Headless CLI with JSON/JSONL | Simple scripts, scheduled jobs, early prototypes                                    | Subprocess lifecycle, authentication, permissions, and session recovery remain your responsibility | Good initial common denominator                          |
| MCP                          | Let one agent call tools or another specialist service                              | Primarily a tool/context protocol, not a complete multi-agent control plane                        | Use for expert calls and bounded delegation              |
| ACP                          | Manage stateful coding-agent sessions through a common protocol                     | Not every agent exposes identical functionality through ACP                                        | Useful for OpenClaw and uniform coding-agent integration |
| Native SDK                   | Programmatic invocation, session management, events, permissions, structured output | Requires one adapter per provider                                                                  | Best inner integration                                   |
| App-server                   | Deep integration with an agent's runtime, approvals, history, and event streams     | More implementation work than a simple CLI call                                                    | Best for advanced Codex integration                      |
| Files/database               | Durable task handoffs, outputs, decisions, evidence, and audit history              | Requires explicit schemas                                                                          | Best inter-agent communication layer                     |
Claude supports programmatic execution through the Claude Agent SDK and headless CLI output including JSON and streaming JSON. The SDK exposes agent tools, sessions, hooks, subagents, MCP connections, and permission controls. ([Claude Platform Docs][7])
Codex supports `codex exec`, structured output, the Codex SDK, MCP-server mode, and an app-server interface for deeper integrations involving approvals, sessions, history, and event streams. OpenAI explicitly documents Codex-as-MCP-server when Codex is a specialist inside a broader agent workflow. ([OpenAI Developers][8])
Grok Build supports interactive operation, headless JSON/streaming-JSON operation, and a native ACP server using `grok agent stdio`. Separately, the xAI API exposes X-search capabilities including keyword, semantic, user, and thread searches. ([SpaceXAI Docs][9])
## Practical decision
Use this order of preference:
```text
Native SDK/app-server
    ↓
ACP when standardized coding-session control is useful
    ↓
MCP when one agent needs to call another as a specialist
    ↓
Headless CLI as fallback
    ↓
Interactive TUI only for humans and debugging
```
Do not automate by sending keystrokes to three terminal windows and scraping their ANSI output when structured interfaces are available.
---
# 3. Do not let them "talk" through unrestricted conversation
A three-way group chat sounds intuitive but produces several problems:
* Repetition and token growth
* Agents anchoring on the first answer
* Correlated mistakes
* Endless agreement loops
* No clear task ownership
* No reliable record of which output changed the project
* Ambiguous permissions
* Difficulty reproducing decisions
* Difficulty interrupting or recovering failed work
Instead, the controller should pass structured artifacts.
## Task packet
```json
{
  "task_id": "agent-memory-eval-2026-07-001",
  "project_id": "social-orchestrator",
  "base_commit": "abc123",
  "assigned_role": "implementation",
  "objective": "Evaluate the proposed context-compaction technique",
  "input_artifacts": [
    "evidence/grok-research.json",
    "plans/claude-plan.json"
  ],
  "allowed_paths": [
    "src/memory/**",
    "tests/memory/**"
  ],
  "forbidden_paths": [
    ".env",
    "infra/production/**",
    ".github/workflows/deploy.yml"
  ],
  "acceptance_criteria": [
    "Existing tests pass",
    "Retrieval score does not decline",
    "Median token use improves by at least 15%",
    "No increase in tool-call failure rate"
  ],
  "verification_commands": [
    "npm test",
    "npm run eval:memory",
    "npm run lint"
  ],
  "permission_profile": "isolated-write",
  "maximum_cost_usd": 8,
  "expected_output_schema": "schemas/agent-result.json"
}
```
## Result packet
```json
{
  "task_id": "agent-memory-eval-2026-07-001",
  "agent": "codex",
  "status": "completed",
  "summary": "Implemented experimental compaction strategy",
  "changed_files": [],
  "worktree": "",
  "branch": "",
  "commit": "",
  "tests": [],
  "baseline_metrics": {},
  "candidate_metrics": {},
  "evidence": [],
  "risks": [],
  "unresolved_questions": [],
  "confidence": 0.82,
  "recommended_next_state": "independent_review"
}
```
Agents may reference one another's output files, but they should not own the master workflow. The controller decides when an output is ready, rejected, sent for review, or escalated.
---
# 4. Recommended roles for the three systems
These are initial routing hypotheses, not permanent claims about which model is universally best.
## Grok: intelligence scout and independent challenger
Use Grok for:
* X monitoring
* Fetching reply trees and quote-post discussions
* Finding early practitioner commentary
* Locating original announcements
* Identifying disagreements or counterexamples
* Producing an independent implementation or review on selected tasks
Use two distinct Grok paths:
```text
xAI API + X Search
    → social and current-information intelligence
Grok Build / ACP
    → repository and coding tasks
```
Do not give the X-research process project write access. It should produce an evidence packet only. X content is untrusted input.
## Claude: architect, context synthesizer, and critic
Use Claude initially for:
* Converting project context and evidence into a plan
* Architecture analysis
* Decomposing complex work
* Identifying assumptions and risks
* Reviewing whether a patch matches the intended design
* Writing user-facing decision summaries
Claude's current tooling supports sessions, subagents, hooks, permissions, MCP connections, worktrees, memory, and structured noninteractive operation. ([Claude Platform Docs][10])
## Codex: implementer, test runner, and repair worker
Use Codex initially for:
* Editing code in bounded paths
* Implementing experiments
* Running tests
* Repairing test failures
* Producing compact diffs
* Reviewing another agent's implementation independently
* Operating through the SDK or app-server inside your controller
Codex's structured execution, SDK, app-server, MCP mode, sandboxing, and approval model make it suitable as an implementation worker under an external state machine. ([OpenAI Developers][8])
## Do not hard-code these roles forever
Record performance by task type:
```yaml
metrics:
  successful_task_rate:
  accepted_patch_rate:
  review_defect_rate:
  regression_rate:
  median_latency:
  median_cost:
  test_repair_rate:
  human_edit_distance:
  unsupported_claim_rate:
```
After 20–50 representative tasks, route based on your own evidence. One model may outperform another on a particular repository, language, task size, or testing environment.
---
# 5. Best orchestration behavior
Use three modes rather than invoking all three for every task.
## Mode 1: Single agent
Use for routine, low-risk work:
```text
Controller → best-performing worker → tests → result
```
Examples:
* Rename or mechanical refactor
* Documentation update
* Known test repair
* Simple research extraction
## Mode 2: Worker plus reviewer
Use for ordinary feature work:
```text
Claude plan
    ↓
Codex implementation
    ↓
Grok or Claude independent review
    ↓
Deterministic tests
```
The reviewer should not edit the implementation immediately. It should first produce a structured defect report.
## Mode 3: Independent parallel attempts
Use for high-risk or uncertain problems:
```text
Shared task packet
    ├─ Claude solution in worktree A
    ├─ Codex solution in worktree B
    └─ Grok solution in worktree C
              ↓
     Tests and metric comparison
              ↓
   Blind cross-review / synthesis
```
Do not show agents the other solutions before their initial attempt. Independent first passes reduce anchoring.
Agent-team and multi-agent execution can consume substantially more tokens than ordinary single-agent work; Anthropic's documentation notes that agent-team workloads may use roughly seven times the tokens of standard sessions in representative cases. Multi-agent execution should therefore be triggered by risk or expected value, not used automatically. ([Claude Platform Docs][11])
---
# 6. Worktree isolation is essential
Never allow Claude, Codex, and Grok to edit the same working directory simultaneously.
Use:
```text
main repository
├── worktree/task-001-claude
├── worktree/task-001-codex
└── worktree/task-001-grok
```
Each agent receives:
* A fixed base commit
* Its own branch
* An allowed-path list
* Test commands
* A permission profile
* A cost ceiling
* A result schema
The controller, not an agent worker, performs the final integration.
Worktree-based isolation appears in Claude's official coding workflows and in current multi-agent orchestration implementations such as Ultraswarm. ([Claude Platform Docs][12])
A worktree only isolates repository files. It does not isolate:
* Environment variables
* SSH keys
* Browser sessions
* System credentials
* Network access
* Other user files
Use a container or restricted process environment for untrusted repositories, external code, dependency testing, or autonomous shell execution.
---
# 7. OpenClaw versus Hermes
## Choose OpenClaw when the priority is orchestration access
OpenClaw is the better outer layer when you need:
* Telegram/Discord or remote control
* Persistent sessions
* Background jobs
* Channel-to-agent routing
* Multiple isolated agent workspaces
* External coding harnesses through ACP
* Native Codex integration
* Session spawning and status monitoring
OpenClaw distinguishes its own subagents from external ACP agents, provides session tools for spawning and messaging, and supports routing multiple isolated agents through one gateway. ([OpenClaw][4])
Recommended OpenClaw arrangement:
```text
OpenClaw
├── Codex: native Codex integration/app-server
├── Claude Code: ACP/ACPX session
├── Grok Build: native ACP through ACPX
└── Custom orchestrator: task state, routing, evaluation
```
OpenClaw should trigger and observe work. The custom orchestrator should still own the authoritative state machine.
## Choose Hermes when the priority is accumulated personal intelligence
Hermes is compelling when the priority is:
* Persistent personal memory
* Learning reusable skills
* Cron jobs
* Research automations
* Terminal and browser workflows
* Personal messaging access
* Reusing experience across sessions
Use Hermes later as:
```text
Hermes
├── curated long-term knowledge
├── reusable procedural skills
├── scheduled intelligence collection
└── distilled experiment lessons
```
Do not run OpenClaw and Hermes as two competing master orchestrators over the same repositories. Select one outer gateway. The other can run as a bounded service beneath it.
## Direct recommendation
For your stated project:
```text
OpenClaw = outer gateway
Custom controller = inner orchestrator
Hermes = optional memory/skills service later
```
---
# 8. Where Ultraswarm, Deliberation, and similar tools fit
| Tool/pattern         | Best use                                                | Assessment                                                                |
| -------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------- |
| Ultraswarm           | Multi-agent coding, worktrees, routing, gates, recovery | Best implementation reference reviewed for the inner control plane        |
| Deliberation         | Ask Claude, Codex, and Grok for independent opinions    | Good for design decisions and reviews; not a full coding lifecycle        |
| Sub-Agents Skills    | Lightweight portable delegation between CLIs            | Good prototype layer; limited durable control-plane features              |
| Claude Code Fusion   | Claude-led implementation delegation                    | Useful role-pattern example; evaluate maturity before depending on it     |
| CCB/terminal bridges | Human-visible concurrent terminal agents                | Good for interactive supervision; less suitable for unattended automation |
| OpenClaw             | Persistent gateway, sessions, channels, schedules       | Best outer control layer for your use case                                |
| Hermes               | Memory, skills, personal automation                     | Best considered as a supplementary intelligence layer                     |
Sub-Agents Skills uses portable task definitions to delegate between Claude, Codex, and Grok Build. CCB uses native terminal agents with explicit communication relationships. These demonstrate that the ecosystem is converging on delegation and artifact passing rather than a single shared conversational context. ([GitHub][13])
I would study Ultraswarm's architecture and possibly use it for an initial prototype, but avoid making a small third-party orchestrator the permanent owner of unrestricted machine access without reviewing its code, permissions, recovery behavior, and update process.
---
# 9. Shared project instructions
Use one canonical instruction file to reduce divergence.
```text
repository/
├── AGENTS.md               # shared project rules
├── CLAUDE.md               # imports AGENTS.md; Claude-specific additions
├── .codex/                 # Codex-specific configuration
├── .grok/                  # Grok-specific configuration
└── .orchestrator/
    ├── schemas/
    ├── policies/
    ├── evaluations/
    └── tasks/
```
Recommended division:
```markdown
# AGENTS.md
- Project architecture
- Build and test commands
- Coding conventions
- Allowed and prohibited modifications
- Security constraints
- Acceptance criteria
- Definition of done
```
```markdown
# CLAUDE.md
@AGENTS.md
- Claude-specific planning format
- Required architecture-review output
- Claude-specific tool constraints
```
Codex uses `AGENTS.md` for repository instructions. Claude supports project memory and importing shared instruction files. Grok Build supports Claude-compatible configuration and agent instruction conventions, including `AGENTS.md` and Claude-related project configuration. ([OpenAI Developers][14])
Provider-specific files should only contain provider-specific behavior. Do not maintain three independent copies of the project architecture.
---
# 10. Recommended inner-controller implementation
TypeScript is a practical initial choice because Claude and Codex have official TypeScript programmatic interfaces, while Grok ACP uses JSON-RPC over a subprocess and the xAI API is straightforward to call from the same service. Python is also viable, but one language should own the control plane. ([Claude Platform Docs][7])
A minimal interface:
```ts
type AgentName = "claude" | "codex" | "grok";
interface TaskPacket {
  taskId: string;
  projectId: string;
  baseCommit: string;
  role: "research" | "plan" | "implement" | "review" | "verify";
  objective: string;
  inputArtifacts: string[];
  allowedPaths: string[];
  forbiddenPaths: string[];
  acceptanceCriteria: string[];
  verificationCommands: string[];
  permissionProfile: string;
  maxCostUsd: number;
  outputSchema: string;
}
interface AgentResult {
  taskId: string;
  agent: AgentName;
  status: "completed" | "failed" | "blocked";
  summary: string;
  artifacts: string[];
  branch?: string;
  commit?: string;
  testResults: TestResult[];
  risks: string[];
  unresolvedQuestions: string[];
  confidence: number;
}
interface AgentAdapter {
  healthCheck(): Promise<void>;
  execute(task: TaskPacket): Promise<AgentResult>;
  cancel(taskId: string): Promise<void>;
  resume(taskId: string): Promise<AgentResult>;
}
```
Suggested storage:
```text
SQLite initially:
- projects
- tasks
- task_dependencies
- runs
- agent_sessions
- artifacts
- reviews
- evaluations
- approvals
- costs
- events
Git:
- source code
- worktree changes
- commits
- patches
Object/file storage:
- research evidence
- logs
- benchmark outputs
- screenshots
- large reports
```
Do not begin with a vector database merely because the system includes AI. Store structured task and decision history first. Add semantic retrieval only when you can specify what must be retrieved and how retrieval quality will be evaluated.
---
# 11. Exact workflow for your AI-update system
A complete run should look like this:
```text
1. Scheduler triggers topic scan.
2. Grok/X Search collects:
   - original posts
   - author replies
   - quote-post criticisms
   - linked repositories
   - linked papers
   - named techniques
3. Evidence verifier retrieves:
   - official documentation
   - source repository
   - releases and commits
   - paper and supplementary material
   - benchmark artifacts
4. Claude receives a structured evidence packet:
   - maps technique to your projects
   - explains architecture
   - identifies assumptions
   - proposes an experiment
   - defines expected metrics
5. Controller decides:
   - store only
   - monitor
   - create issue
   - run experiment
   - request human review
6. Codex receives a bounded implementation task:
   - isolated worktree
   - restricted paths
   - no production credentials
   - explicit tests
   - fixed cost/time budget
7. Claude or Grok independently reviews the patch.
8. Deterministic evaluator runs:
   - tests
   - benchmark
   - static analysis
   - security checks
   - baseline comparison
9. Controller creates:
   - experiment report
   - draft pull request
   - recommendation
   - approval request
10. OpenClaw delivers the result through your chosen channel.
11. After approval:
   - controller merges
   - monitors metrics
   - records the outcome
   - stores reusable lessons
```
This gives all three systems a meaningful role without allowing social-media content to flow directly into executable changes.
---
# 12. What not to build
Avoid:
```text
Claude ↔ Grok ↔ Codex
```
with all three continually prompting each other.
Also avoid:
* Three agents editing one checkout
* Giving all three unrestricted shell access
* Giving research agents access to secrets
* Allowing one model to plan, implement, test, and approve its own work
* Treating majority agreement as proof
* Saving complete chat transcripts as "memory"
* Running global `always approve` or permission-bypass modes
* Automatically executing commands found in posts, papers, READMEs, or issues
* Using all three agents on every trivial task
* Operating OpenClaw and Hermes as competing master controllers
* Allowing the agents to modify the controller's approval or security policies
OpenAI recommends scoping credentials carefully when Codex operates around untrusted code, and Claude and Grok provide explicit permission or approval controls that should be configured rather than bypassed globally. ([OpenAI Developers][15])
# Final recommendation
For your Mac-based system:
```yaml
outer_gateway:
  choice: OpenClaw
  responsibilities:
    - Telegram/Discord/user interface
    - schedules
    - notifications
    - session visibility
    - approval requests
inner_orchestrator:
  choice: custom lightweight TypeScript service
  responsibilities:
    - state machine
    - task routing
    - task/result schemas
    - budgets
    - worktrees
    - permission profiles
    - evaluation gates
    - audit log
agents:
  grok:
    research: xAI API with X Search
    coding: Grok Build through ACP
    initial_role: scout_and_independent_challenger
  claude:
    interface: Claude Agent SDK
    initial_role: architect_and_critic
  codex:
    interface: Codex SDK_or_app_server
    initial_role: implementer_and_test_repair
optional:
  hermes:
    role:
      - long_term_personal_memory
      - learned_skills
      - scheduled_research
    status: add_after_core_orchestration_is_stable
execution:
  isolation: one_worktree_per_agent_per_task
  production_changes: human_approval_required
  communication: structured_artifacts_not_freeform_chat
```
For an immediate prototype, use **Ultraswarm's architecture as a reference or temporary runner**, place **OpenClaw above it for channels and scheduling**, and keep Grok's X-search ingestion separate from Grok Build's coding access. The durable end state should be your own thin controller using native Claude, Codex, and Grok interfaces rather than depending entirely on terminal automation or one third-party orchestrator.

[1]: https://github.com/okisdev/claude-code-fusion "GitHub - okisdev/claude-code-fusion: Multi-model orchestration for Claude Code: Claude worker tiers plus the Codex and Grok CLIs as peer engineers · GitHub"
[2]: https://github.com/fubak/ultraswarm "GitHub - fubak/ultraswarm: Multi-CLI agent swarm orchestrated by Claude Code: external AI CLIs code in isolated worktrees, Claude verifies and merges · GitHub"
[3]: https://docs.openclaw.ai/agent-runtime-architecture "Agent runtime architecture - OpenClaw"
[4]: https://docs.openclaw.ai/tools/acp-agents "ACP agents - OpenClaw"
[5]: https://github.com/nousresearch/hermes-agent "GitHub - NousResearch/hermes-agent: The agent that grows with you · GitHub"
[6]: https://github.com/NousResearch/hermes-agent/issues/413 "Feature: Cross-CLI Agent Orchestration — Mixed Workflows with External Agent CLIs (inspired by AgentWorkforce/relay) · Issue #413 · NousResearch/hermes-agent · GitHub"
[7]: https://docs.anthropic.com/en/docs/claude-code/sdk?utm_source=chatgpt.com "Agent SDK overview - Claude Code Docs"
[8]: https://developers.openai.com/codex/non-interactive-mode?utm_source=chatgpt.com "Non-interactive mode | ChatGPT Learn - OpenAI Developers"
[9]: https://docs.x.ai/build/overview "Grok Build | SpaceXAI Docs"
[10]: https://docs.anthropic.com/en/docs/claude-code/overview "Overview - Claude Code Docs"
[11]: https://docs.anthropic.com/en/docs/claude-code/costs?utm_source=chatgpt.com "Manage costs effectively - Claude Code Docs"
[12]: https://docs.anthropic.com/en/docs/claude-code/ide-integrations?utm_source=chatgpt.com "Use Claude Code in VS Code - Claude Code Docs"
[13]: https://github.com/shinpr/sub-agents-skills "GitHub - shinpr/sub-agents-skills: Cross-LLM sub-agent orchestration as an Agent Skills. Route tasks to Codex, Claude Code, Grok, GLM, Cursor, Gemini, or OpenCode from any compatible tool. · GitHub"
[14]: https://developers.openai.com/codex/concepts/customization?utm_source=chatgpt.com "Customization – Codex"
[15]: https://developers.openai.com/codex/agent-approvals-security?utm_source=chatgpt.com "Agent approvals & security | ChatGPT Learn"
