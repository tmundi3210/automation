# INTAKE — idea intake paths (intake-lite v1)

PROTOCOL-VERSION: 3.3
Owner: integrator. Purpose: every user idea gets the pipeline discipline it can
afford. The FULL pipeline (neutralize → intent map → boundaries → forge new
specialists → dense gates; see `PLAN.md` §6, `docs/HANDOFF.md`) is for ideas
that justify a durable new capability. **intake-lite** is the same discipline
at ~1% of the weight, for one-shot S/M ideas.

## When to use which

- FULL: the idea implies a reusable capability (a new specialist/KB), touches a
  risk boundary, or will be re-run many times. Output: forged specialists.
- LITE: one deliverable, existing specialists suffice as advisors, LOW risk.
  Output: a normal EXCHANGE task with specialist advice injected.
- Neither (raw): pure mechanical chores (rename, bump, re-run). No routing.

## intake-lite steps (recorded in the task message, auditable)

1. **NEUTRALIZE-LITE** — restate the raw idea in ≤120 words: affirmative
   framing (required action first), intent preserved, implementation bias and
   loaded wording removed, out-of-scope named. One pass, no committee.
2. **ROUTE** — apply `specialists/ROUTER.json` selection_procedure to the
   neutralized brief: score route_when term hits (domain_label ×2), take the
   top specialist plus any within 1 point, **cap 3**. Record the picks and a
   one-line rationale each. Zero hits → proceed raw, say so.
3. **INJECT** — assignee loads ONLY the selected specialists' distilled lines
   from `dist/specialist_prompts.jsonl` (match on `"code"`) as ADVISOR
   context. Dense KBs stay on disk; advisors advise, the task governs.
4. **EXECUTE** — normal EXCHANGE task (branch / write_scope / gates per
   ORCHESTRATION). The deliverable ends with a `SPECIALIST-COMPLIANCE:` block:
   which advisor rules were actually applied, one line each.
5. **REVIEW** — integrator verdict as usual, plus a spot-check against the
   selected specialists' escalation triggers (e.g. scholcomm: unverifiable
   source → must be dropped or flagged, never silently kept).

## Idea inbox — every door leads to the same pipeline (INBOX-1)

The operator may drop an idea at ANY agent; the result must be identical:
- **Door A (Claude directly):** intake runs immediately.
- **Door B (Grok or Codex CLI):** the builder does NOT execute the idea. It
  pushes it VERBATIM as `EXCHANGE/<agent>/msg-NNN.md` with `TYPE: idea`,
  `TO: claude` (own numbering, own branch push is not needed — idea messages
  may go straight on the integration branch since they touch only the
  builder's own dir), then tells the operator one line: "queued as msg-NNN".
- **Door C (no CLI at hand):** commit a text file to `inbox/` on the
  integration branch via GitHub web/mobile editor.
The integrator polls hourly (plus normal wakes): new idea → intake-lite →
task mint. Same neutralize/route/inject regardless of door. Operator-facing
summaries stay ≤4 bullet lines; STATUS.md remains the full view.

## Budget guardrail

intake-lite adds ≤1 short section to the task message and ≤3 jsonl lines to
the builder's context. If routing/injection would exceed that, the idea was
not S/M — escalate to FULL or split.
