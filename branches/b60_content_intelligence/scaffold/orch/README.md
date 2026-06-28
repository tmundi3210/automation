# orch/ — the two separated steps + blackboard + outbox (grounded in `orch`)

The pipeline is **dissected into two independent steps** so the creative work and the safety
decision never tangle (no "unexplained stops" mid-thought; every stop is an explicit, reviewable
gate verdict):

- **`brain.py` — STEP 1.** Collect → connect → draft the recipe + **report safety facts** (real
  person? political? minor? voice/likeness? license?). It makes **no** safety decision and sets
  **no** safety_flags. It writes a **Proposal** to `_proposals/` and stops.
- **`gate.py` — STEP 2.** Reads a Proposal and returns one explicit verdict — **GREEN /
  YELLOW(+required edits) / STOP(+reason)** — plus the disclosure line and a single
  `clear_to_render` flag. Only on clear does it complete safety_flags, run the **schema-parse
  check**, and write the `EmitPacket` to the outbox. Hard blocks are never overridden.
- `blackboard.py` — a typed blackboard enforcing **one write-owner per (node, block)**, plus the
  durable **outbox**: an immutable, sanitized, idempotency-keyed packet a downstream session
  **pulls** (no in-process call across the trust boundary).
- `controller.py` — the thin runner that chains **brain → gate**. A static check enforces that
  **the gate is the sole predecessor of emit**, so nothing reaches the outbox except through it.

The two halves match the two copy-paste prompts in `../BRAIN_STEP1.md` and `../GATE_STEP2.md`.

```bash
python3 orch/brain.py        # STEP 1: draft + flag → writes _proposals/<scene>.json
python3 orch/gate.py         # STEP 2: reads the latest proposal → GREEN/YELLOW/STOP → emit
python3 orch/controller.py   # runs both in sequence → EMITTED
python3 orch/controller.py --anchor IN_REAL_ARTIST_POLITICAL --no-emit   # → STOP / HELD_COUNSEL
python3 orch/blackboard.py   # single-writer + outbox demo
```
