# DECISION — operator ruling on the bounded-autonomy planning package

Date: 2026-07-14. Channel: Door A (chat to hub).
Operator words (verbatim): "lets not use openclaw or hermes - but other
implementations - … and implement the new system."

## Ruling recorded

**APPROVE_WITH_CHANGES.** The STOP CONDITION is lifted; implementation
starts at Phase A per IMPLEMENTATION_PHASES.md.

Changes bound into the package by this decision:

1. **OpenClaw is OUT** as a component (gateway or otherwise). ARCH_REVIEW
   verdict #8's conflict resolves to REJECTED — the R6 isolation
   precondition never has to be met because the component is not used.
   OpenClaw remains a Phase-6 case study only.
2. **Hermes is OUT** as a component (memory/skills layer or otherwise).
   Remains a case study only. MEMORY_ARCHITECTURE's own 8-layer design is
   the only memory plan.
3. **"Other implementations"**: gateway/channel-delivery and memory needs
   are met by our own thin components (control plane per Phase A; delivery
   via operator-chosen first-party channels — this chat, terminal, GitHub
   PR/issue notifications). Third-party orchestrators/gateways remain
   VERIFY-FIRST case-study material (ARCH_REVIEW verdict #10), adopted only
   by a future explicit operator decision.
4. ARCH_REVIEW recommended edits 1–3 apply, minus every OpenClaw item:
   task-packet schema (wall-minutes/invocations ceiling, forbidden_paths),
   result-packet metrics into EVALUATION_REGISTRY, controller-service and
   per-lane interface-upgrade items into IMPLEMENTATION_PHASES. The inbound
   doc's step 10 (channel delivery) is retargeted to first-party channels.

## Standing security note (unchanged by this decision)

R6 still applies to the operator's MACHINE: the Hermes/OpenClaw gateways
remain installed on the Mac as LaunchAgents co-resident with credentials.
Not using them in this system does not un-install them. Standing
recommendation: `launchctl bootout gui/$(id -u)/ai.hermes.gateway` (and the
openclaw equivalent) or move them to a separate macOS user.

## Phase A opened

- TASK-019 (grok): live v1.1 poller validation + Phase A capability probes
  (plan/rate limits, A5) + URL-verification of the 14 TOP_SOURCES rows.
- TASK-020 (codex): `orchestrator/control_plane/` skeleton per Phase A
  (no-op tick, run records, budget counters + unit tests, kill switch;
  shell/python stdlib ONLY — any model call in the control plane is a
  review reject).
- Hub: integration, review, capability-matrix sign-off; Phase A operator
  checkpoint (demo no-op tick + kill switch) before Phase B.

---

# OPERATOR DECISION 2026-07-15 — codex promoted to LOCAL OPERATIONS OWNER

Operator ruling (verbatim): "if its not working yet then lets make the
codex the main operator - change the files directiosn line in agent.md etc
etc {that will be much easier as codex can manage it better as it can make
changes locally and can push to github also"

Context: the 2026-07-15 forensics (POLLING.md OPS FINDINGS) proved every
remaining orchestration failure is Mac-LOCAL plumbing — /tmp repo pins,
plist env overrides, a wrapper syntax error, and a gate that is remote-blind
under launchd — a class of defect the cloud hub can never touch directly.

## Bounded interpretation (in force)

1. codex ROLE becomes `builder + local-ops-owner`. It OWNS the Mac-side
   orchestration infrastructure: BOTH gate dirs (~/.exchange-gate,
   ~/.exchange-gate-codex), the com.mundi.exchange-poll-* LaunchAgent
   plists, wrapper/gate scripts, and local standing-orders installs. It may
   modify these LOCAL files under an authenticated TASK block, and under a
   standing maintenance mandate (MAINT-1) for like-for-like repair of wake
   plumbing, with pre-edit backups and evidence in its delivery messages.
2. Repo-side write scope is UNCHANGED (EXCHANGE/codex/** + per-task SCOPE).
   No new merge rights. GitHub integrator/single-writer authority — merges,
   verdicts, ledger, gates — remains with claude: the operator's stated
   rationale is local file management + push ability, both satisfied
   without transferring merge authority. If the operator additionally
   intends to transfer integrator/merge authority to codex, that requires
   a separate explicit ruling naming it.
3. grok is unaffected; codex touching ~/.exchange-gate (grok's dir) is
   authorized ONLY for wake-plumbing files, never grok's task content.
4. First act: TASK-023 (msg-055) — wrapper syntax fix, gate remote-blindness
   fix, error-visibility logging, F2 standing-orders re-point, end-to-end
   wake proof. TASK-022 folds into it (superseded).
