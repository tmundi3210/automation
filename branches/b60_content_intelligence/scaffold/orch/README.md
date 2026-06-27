# orch/ — controller spine + blackboard + outbox (grounded in `orch`)

- `blackboard.py` — a typed blackboard enforcing **one write-owner per (node, block)** (a stage
  that overwrites another's block raises), plus the durable **outbox**: an immutable, sanitized,
  idempotency-keyed `EmitPacket` written to disk that a downstream session **pulls** (no
  in-process call across the trust boundary).
- `controller.py` — the v1 **DAG spine**: ingest → salience → entity_link → signal{auth ∥ hype}
  → dense_summary → creative_brief → **compliance_gate → emit** → outbox. A static check
  (`assert_emit_guarded`) enforces that **compliance_gate is the sole compile-time predecessor of
  emit** every run. Before emit it runs the **schema-parse check** on every scene node; it emits
  only when compliance passes AND psych `publish_ok` AND the disclosure token survives the handoff.
  The offline improve-loop (`eval/`) is intentionally **not** in this live path.

```bash
python3 orch/blackboard.py                              # single-writer + outbox demo
python3 orch/controller.py                              # auto-pick anchor → EMITTED
python3 orch/controller.py --anchor IN_REAL_ARTIST_POLITICAL --no-emit   # → HELD_COUNSEL
```
