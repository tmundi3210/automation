# B60 v1 MVP — the runnable scaffold

The `BUILD.md` v1 MVP, turned into **offline-runnable code**. Every stage of the content-
intelligence spine runs here on CPU with a deterministic mock backend — no GPU, no network,
no keys, no paid data. The real model/data paths are marked **hook points** so nothing
silently needs compute, makes paid calls, or scrapes anything.

> Read order: `../SCOPE.md` → `../BRAIN.md` → `../RUN.md` → `../PANEL.md` → `../BUILD.md` →
> **this scaffold**. BUILD.md is the spec; this is the spec made to run.

## The honest MVP target (from BUILD.md)

Not virality — **prove the loop closes**:
- **(a)** one end-to-end run that ships a **clearance-passing brief about a safe-lane subject**, and
- **(b)** one **leakage-controlled eval verdict**.

`run_pipeline.py` does both in one command (and shows the gate failing closed on a
real-person + political + voice subject).

```bash
python3 branches/b60_content_intelligence/scaffold/run_pipeline.py   # the two deliverables
bash    branches/b60_content_intelligence/scaffold/run_all.sh        # smoke-test all 17 subsystems
```

Current offline result: **(a)** emits `SCENE_OWN_PERSONA_CHACHA__NEWS_H1B_2025`
(ALLOW_WITH_CONSTRAINTS, signed disclosure token, 2/2 nodes schema-valid) to the outbox;
**(b)** returns **NO-GO** — the medium-tier bet is *confirmed* (bootstrap CI excludes zero)
but the system does *not* beat the human-curated baseline. Both are the honest outcome.

## The spine (controller DAG)

```
ingest ─► salience ─► entity_link ─► signal{authenticity ∥ hype} ─► dense_summary
        ─► creative_brief ─► compliance_gate ─► emit ─►(outbox)─► downstream pull
                                  ▲
                    eval harness (OFFLINE loop, not in the live path)
```
`compliance_gate` is the **sole compile-time predecessor of `emit`** (a static check runs
every invocation). The handoff is an **outbox**: an immutable, schema-validated, sanitized
packet with an idempotency key that a downstream session *pulls* — no in-process call across
the trust boundary.

## Grounding map (stage → b60 specialist → what it implements)

| dir | specialist | implements |
|---|---|---|
| `common.py` / `backends.py` | — | schema loader + stdlib JSON-Schema validator; pluggable Mock/Api/Local backend |
| `ingest/` | `ir` (data layer) | tiered pull (mock fixtures) → URL+SimHash dedup → partial InformationNode "understanding" |
| `salience/` | `salience_heavy` | 4-signal geometric-mean score + opportunity composite + bounded region→niche→top-5 expander |
| `link/` | `salience_heavy` + `reason` | typed-tag join (rank) + geo predicate (license) + abductive propose-and-verify, arity {2,3} |
| `reaction/` | `signal_heavy` | Platt-calibrated authenticity + N-independent-origins hype + decision-lossless dense brief |
| `creative/` | `creative_heavy` | closed `gen_brief` (image+story+contract+provenance); disclosure-or-fail-closed; briefs only |
| `compliance/` | `compliance_heavy` | green-lane clearance + safety_flags + signed C2PA-style disclosure token (hard blocks stay hard) |
| `psych/` | `psych` | 4-driver resonance + safe-lanes whitelist + reach/saves-weighted honest sentiment |
| `orch/` | `orch` | typed blackboard (single write-owner) + DAG spine (gate-before-emit) + durable outbox |
| `eval/` | `eval_heavy` | frozen pre-registration + post-cutoff blind holdout + bootstrap CI + terminating improve-loop |

## Real vs hook (honesty)

- **Real & runnable now:** the dedup (URL+SimHash), the salience math + bounded expander, the
  geo-predicate + abductive link selection, the Platt calibration fit, the independent-origins
  hype gate, the dense-brief round-trip audit, the gen_brief disclosure-fail-closed, the
  green-lane clearance + signed token + handoff verifier, the resonance gate, the blackboard +
  outbox, the controller spine + static emit guard + schema validation, and the leakage-
  controlled bootstrap backtest.
- **Hook points (need compute/keys/data):** `ApiBackend`/`LocalBackend` (a real proposer/
  verifier/NLI/judge), the live Tier-0/1/2 source fetchers (`ingest.build_nodes.load_raw`),
  isotonic recalibration + a real labeled CIB benchmark, a real C2PA signer, and a real
  realized-outcome ledger + confirmed cutoff for the eval.
- **Needs your data:** replace `ingest/_fixtures/raw_items.json`, `reaction/_fixtures/
  cib_benchmark.json`, and `eval/_fixtures/holdout.frozen.jsonl` with real artifacts. Until
  then every score is a heuristic prior and the eval is the gate that must earn any "it works."

See `ARCHITECTURE_DECISION_RECORD.md` for what's proven vs speculative for *this* system.
