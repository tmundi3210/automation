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
