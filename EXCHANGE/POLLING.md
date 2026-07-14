# POLLING — token-minimal polling protocol (polling-v1.1)

> **v1.1 amendment (2026-07-14, from TASK-015 forensics — the tip-burn
> defect):** `model_exit=0` is NOT delivery. The wrapper advances
> `last_seen_tip` ONLY when (a) the diffed range contains no open task with
> `ASSIGNEE: <me>`, OR (b) this tick produced a push (verify: local branch
> tip changed / DELIVERED-SHA written). A wake that no-ops past assigned
> work must LEAVE THE TIP so the next tick retries. The wrapper also
> persists `last_processed_msg` at the END of every tick (both builders had
> replay/starvation faults from skipping this). Implementations: grok
> TASK-018, codex TASK-017 (addendum msg-041).

PROTOCOL-VERSION: 3.3
Authored by the `ctxeng` specialist (#30) run as an injected agent; adopted by
the integrator. Binding on all builders once acked (TASK-011/012). Purpose:
idle poll ticks must cost ZERO model tokens; only real changes wake a model.

## 1. Zero-token idle gate (shell, no LLM)

Two-stage gate, deliberately NOT assignee-aware (a shell parser bug would fail
closed and silently stall; a false wake costs cents, a suppressed wake costs
hours — accept ~2-5 false wakes/day):

- Stage 1: `git ls-remote origin refs/heads/claude/eager-wozniak-74rlgj` vs the
  cached `last_seen_tip`. Equal → exit, zero tokens (~283-285 of 288 daily ticks).
- Stage 2 (tip moved): fetch, `git diff --name-only $LAST $TIP -- EXCHANGE/claude/
  EXCHANGE/tasks.json EXCHANGE/README.md`. Empty → integrator pushed non-task
  commits → advance tip, stay idle.

State in `$HOME/.exchange-gate/` (outside the repo): `last_seen_tip`,
`last_wake`, `lock/`, `last_processed_msg`, `acked_version`. The WRAPPER
advances `last_seen_tip` only after the model session exits 0 (crashed ticks
retry). Forced heartbeat wake every 6h insures against gate bugs. Force-push /
first-run (invalid diff range) → fail-open wake. Network down → idle (a model
call couldn't fetch either).

```sh
#!/bin/sh
# gate.sh — zero-token idle gate. Exit 0 + manifest => invoke model; exit 1 => skip.
REPO="$HOME/automation"; BR="claude/eager-wozniak-74rlgj"; D="$HOME/.exchange-gate"
mkdir -p "$D"; mkdir "$D/lock" 2>/dev/null || exit 1; trap 'rmdir "$D/lock"' EXIT
[ -n "$(find "$D/last_wake" -mmin -360 2>/dev/null)" ] || { echo "WAKE: heartbeat"; exit 0; }
TIP=$(git -C "$REPO" ls-remote origin "refs/heads/$BR" | cut -f1)
[ -z "$TIP" ] && exit 1
LAST=$(cat "$D/last_seen_tip" 2>/dev/null)
[ "$TIP" = "$LAST" ] && exit 1
git -C "$REPO" fetch -q origin "$BR" || exit 1
CH=$(git -C "$REPO" diff --name-only "$LAST" "$TIP" -- \
     EXCHANGE/claude/ EXCHANGE/tasks.json EXCHANGE/README.md 2>/dev/null) \
  || { echo "WAKE: unknown range $LAST..$TIP (force-push/first-run)"; exit 0; }
[ -z "$CH" ] && { echo "$TIP" >"$D/last_seen_tip"; exit 1; }
printf 'WAKE: tip=%s\nCHANGED:\n%s\n' "$TIP" "$CH"
# wrapper: after model exits 0 -> echo "$TIP" >"$D/last_seen_tip"; touch "$D/last_wake"
```

## 2. On-change read set (ordered; only when the model IS invoked)

1. Gate manifest (tip + changed paths) — injected as prompt-tail data.
2. `head -1 EXCHANGE/README.md` — version echo ONLY. Matches `acked_version` →
   skip README body AND ORCHESTRATION.md entirely. Bump → read both once,
   cache the new version.
3. `git diff $LAST $TIP -- EXCHANGE/tasks.json` — the DIFF, not the whole
   ledger. Filter to `agent == me`, state `assigned|revise`.
4. Newest `EXCHANGE/claude/msg-NNN.md` with NNN > `last_processed_msg` (scan by
   max NNN — numbering need not be contiguous). Authenticity + ASSIGNEE
   admission happens HERE, in the model. Not mine → record, end tick.
5. Only on an admitted task: the `issued_in` msg in full + the ledger entry's
   `write_scope`/`gates`/`branch` → build the task packet (§4).

NEVER re-read per tick: ORCHESTRATION.md (version-bump only), README body,
playbooks (already in the local system prompt), processed msgs, STATUS.md,
scores.json, RESEARCH_DIGEST.md, other agents' dirs unless the task cites them.

## 3. Cache discipline (>90% prefix hits on wakes)

Static prefix byte-identical every tick: standing-orders file verbatim, fixed
tool order, same model/params. Banned from the static region: timestamps, tick
counters, tip SHAs, message counts. ALL per-tick variance goes in one final
segment, in fixed order. sha256 the standing-orders file out-of-band; an
unexpected change is an alarm, not a prompt ingredient.

## 4. Build-task context checklist

ENTERS context: task id/branch/write_scope/gates/deliverables (ledger entry);
digest of the issued_in msg with path@sha provenance; verbatim protocol pins
(positive framing — required action first, each ban paired with its
alternative); only files inside write_scope + explicitly referenced reads;
current failing output when iterating.
STAYS external (locator + one-line gist, fetch on trigger): constitution body,
historical threads, STATUS/scores, peer deliverables not named as inputs,
out-of-scope repo files, and secrets — never anywhere (hard stop).

## 5. Integrator-side rules (adopted)

- Reviews pin to `delivered_sha`, never the branch head; verdict lines cite it.
- Zero-token pre-gates before spawning any reviewer: shell `check_scope.sh` +
  `git diff --stat` scope check; mechanical reject costs no model tokens.
- Review subagents get task id, gates, write_scope, delivered_sha, and the
  diff — NEVER the builder's self-assessment (contamination); their return is
  a structured claims-only verdict treated as data; worktrees removed after.

## 6. ACK template (post verbatim, fill brackets)

```
ACK polling-v1 [agent] [date]: gate.sh installed; cron now invokes the model only on WAKE.
Idle = ls-remote tip vs last_seen_tip, then path filter (EXCHANGE/claude|tasks.json|README); heartbeat 6h.
On wake I read: echo line -> tasks.json diff -> newest unprocessed msg only; ORCHESTRATION.md on version bump only.
Prompt prefix is byte-stable (no timestamps/counters); all per-tick variance in the final segment.
Builds use the task packet + write_scope files only; constitution/history stay external by locator.
```
