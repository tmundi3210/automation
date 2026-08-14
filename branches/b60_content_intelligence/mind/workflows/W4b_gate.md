# W4b_gate — THE GATE — the separated stop/yellow blocker check (run after ACT, before REFLECT)

> The **only** safety decision in the loop, dissected OUT of ACT into its own step so it runs **last
> and reviewable** — never buried inside the brain. ACT (W4) drafts and **flags facts**; this step
> reads that draft + its SAFETY FACTS and returns one explicit **GREEN / YELLOW(+required edits) /
> STOP(+reason)** verdict. **Owner: FROZEN — no specialist.** Deliberately *not* populated by a
> security/ethics specialist; it mirrors the human-authored gate at [`../GATE_STEP2.md`](../GATE_STEP2.md)
> and the runnable `../scaffold/orch/gate.py` + `../scaffold/compliance/clearance.py`. Loop position:
> between **W4 ACT** (draft+flag) and **W5 REFLECT**. This is the mind's copy of the brain/gate split:
> **W4 ACT = `BRAIN_STEP1` (step 1)** · **W4b GATE = `GATE_STEP2` (step 2)**.

## Why this is its own step (and frozen)

Keeping the blockers out of the brain prevents "unexplained stops" mid-thought: the brain always
produces a clean draft + an explicit facts list, and the gate gives one clear verdict you can read and
act on. The separation is also the safeguard — it is **not** owned by the execution faculty precisely
so that execution pressure (a deadline, a hot topic, a calibrated bet ACT is confident in) can never
bend the safety call. There is no `mind_gate` specialist by design: the safety rules are frozen, not
learned, tuned, or re-authored. ACT may **never** re-implement, soften, re-judge, or route around this
step; on STOP the step is marked `blocked` and the loop halts.

## Inputs (read) and Outputs (written)

Paths are relative to the run's memory root `branches/b60_content_intelligence/mind/memory/`.

**Reads:**
- The **Proposal** ACT handed off: the drafted artifact (idea, image description, audio script, story
  beat, suggested disclosure line) **plus the SAFETY FACTS list** (is any subject a real identifiable
  person? political/election? minor/protected? real voice/likeness? private-leaked or copyrighted
  source?). ACT reports these facts; it does not judge them.
- `kernel/environments.jsonl` — the active env's `constraints`; `tech_builders` carries
  `compliance-gate-mandatory`, so this step is non-optional for any publishable output.

**Writes (single-write-owner for the gate):**
- The **Clearance Record / verdict** (`VERDICT`, `REASON`, `REQUIRED EDITS`, `DISCLOSURE LINE`,
  `CLEAR TO RENDER`) — staged for REFLECT/CONSOLIDATE to record and, in the scaffold, written to
  `../scaffold/orch/_outbox/` on a pass.
- On a passing scene, the **signed disclosure token** bound to the draft's `disclosure_slot`
  (`clearance.py` C2PA-style manifest). The gate writes **no** ACT, REFLECT, or CONSOLIDATE file.

## The blockers (the whole point of this step)

These live here, and **only** here — dissected out of the brain.

### 🛑 STOP (refuse, no exceptions — hard blocks are never overridden)
- a **minor or protected person**;
- a **real identifiable person + political/election + their voice or likeness**;
- **fabricated wrongdoing, fake quotes, or defamation** about a real person;
- **private/leaked material**, or republishing someone's **copyrighted media**.

### 🟡 YELLOW (allowed, but emit the REQUIRED EDITS)
- a public **TOPIC/POLICY** → make it factual only, no candidate depicted, add disclosure;
- **PARODY** of a public figure → require visible parody cues, **no real voice clone**, no implied
  endorsement;
- a **real person from a scraped/unknown source** → hold for a license/consent check (`HUMAN_REVIEW`).

### 🟢 GREEN
- your **own original character/persona**, or a **consented use within scope**.

It is a **path-to-yes**: it finds the safe way to ship and only hard-stops the genuinely risky
combinations. *(Screening heuristic, not legal advice — a real STOP/borderline goes to a qualified
lawyer.)*

## Protocol

1. **Receive the Proposal.** Take ACT's drafted artifact + SAFETY FACTS. Do not create, extend, or
   "improve" the draft — you only judge it; never refuse silently.
2. **Classify against the blockers above.** Hard categorical blocks dominate procedural review: check
   minor/protected first, then real-person + political + voice/likeness, then fabrication/defamation,
   then private/leaked/copyrighted, then the YELLOW lanes, else GREEN. (This is exactly the order in
   `clearance.py:_classify`.)
3. **Emit the verdict block, exactly:**
   ```
   VERDICT: GREEN / YELLOW / STOP
   REASON: one or two lines, plain language
   REQUIRED EDITS: (for YELLOW — the exact changes that make it safe; else "none")
   DISCLOSURE LINE: the wording to show on/under the content
   CLEAR TO RENDER: yes / no
   ```
4. **On a pass (GREEN / YELLOW),** sign the disclosure token bound to the draft's `disclosure_slot`
   and set `CLEAR TO RENDER: yes`. Only now may the image be rendered and the audio finalized, **with
   every REQUIRED EDIT and the DISCLOSURE LINE applied.** If YELLOW edits are material, re-run this
   step on the edited output.
5. **On STOP,** set `CLEAR TO RENDER: no`. The step is marked `blocked`; nothing renders, voices, or
   emits. Hand the `blocked` marker + REASON to REFLECT.

## Worked example (run R0631 / goal G-12) — gate ran for real

ACT (W4) handed this step the drafted weekly brief ("this week's agent-framework spike is mostly
manufactured — 2 independent origins, not 3+") + its SAFETY FACTS (subject = a public AI-agent trend,
**no** real identifiable person, not political, no voice/likeness, public sources). Running the frozen
gate (`compliance/clearance.py`, same rules as `GATE_STEP2.md`):

```
VERDICT: GREEN
REASON: public AI-agent topic, no real identifiable person depicted
lane: public_topic   constraints: [SR_SYN_DISCLOSE]
DISCLOSURE LINE: "Synthetic/AI-assisted analysis."
CLEAR TO RENDER: yes
```

Counterfactual — had the brief instead pinned the hype on a **named real founder + election framing +
their voice**, the same frozen gate returns:

```
VERDICT: STOP
REASON: real person + political/election + voice/likeness without consent
lane: hard_block_political_voice
CLEAR TO RENDER: no   → step marked `blocked`, nothing renders
```

Same code, opposite outcome — and neither ACT nor any specialist can override it. That is the
separation doing its job.

## Failure modes & escalation

- **A blocker leaks back into the brain.** If any STOP/YELLOW decision is made inside W4 ACT, the
  separation is broken — ACT must only flag SAFETY FACTS. Fix: the decision belongs here, last.
- **Silent refusal.** Returning no verdict, or a STOP with no plain-language REASON. Always emit the
  full verdict block.
- **Softened STOP.** Emitting after a STOP, or watering down REQUIRED EDITS under deadline pressure.
  STOP halts unconditionally; this is the most serious failure.
- **ESCALATE to a human** (a goal can only end DONE / ABANDONED / ESCALATED) when: the gate STOPs the
  only viable form of a required deliverable with no compliant repair; a borderline case needs
  qualified legal judgment; or repeated STOPs on one goal mean its direction is structurally
  non-compliant (surface the repeated `blocked` hand-offs to Agency/human, never auto-proceed).

## Handoff

- **GREEN / YELLOW** → hand the **cleared output** (+ disclosure line, + any applied required edits) and
  the verdict to **W5 REFLECT**, which compares claimed vs realized vs baseline and scores ACT's bets.
- **STOP** → hand the **`blocked` marker + REASON** to REFLECT (the turn produced no emission;
  REFLECT/Consolidate record it and Agency may ESCALATE or re-scope the goal).

## Single-write-owner contract

- **The gate may write:** the verdict / Clearance Record and (on pass) the signed disclosure token +
  the outbox packet (`../scaffold/orch/_outbox/`).
- **The gate must never touch:** `calibration/predictions.jsonl` or `kernel/blackboard/*` (ACT owns —
  the gate judges ACT's output, it does not edit it); `experience/*`, `semantic/*`, `procedural/*`
  (REFLECT/CONSOLIDATE); `goals/*`, `attention/*`, `schemas/*`. And it never rewrites itself:
  `../GATE_STEP2.md` is frozen.
