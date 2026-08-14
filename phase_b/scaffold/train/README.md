# train — eval-in-the-loop fine-tuning with rollback

Grounded in the 'ftune' specialist (monitored training loop subdomain) and
`IDEA_NEUTRALIZED.md`: "watch while fine-tuning; if no progress, step back, delete
neutral last-N" → checkpointing, early stopping, rollback, and neutral-tail pruning.

## Files
- `rollback_loop.py` — the control loop + a deterministic `SimulatedTrainer` (offline)
  and an `HFTrainerBackend` hook point (real GPU fine-tune).
- `config.example.json` — the tunable policy knobs.

## Run (offline, no GPU)
```bash
python3 phase_b/scaffold/train/rollback_loop.py                         # default policy
python3 phase_b/scaffold/train/rollback_loop.py --config phase_b/scaffold/train/config.example.json
```

## Policy implemented
| knob | meaning |
|---|---|
| `eval_every` | steps between held-out evals (the monitor cadence) |
| `min_delta` | score gain that counts as real progress |
| `patience` | evals without progress before a rollback |
| `neutral_prune_n` | how many neutral trailing checkpoints to delete on rollback |
| `lr_decay_on_rollback` | lr multiplier applied each rollback |
| `max_rollbacks` | consecutive rollbacks (no new best) → early stop |

On a stall/regression the loop **rewinds to the best checkpoint**, **prunes the neutral
last-N**, **decays the learning rate**, and resumes; a genuine new best resets the
rollback budget. Every eval and action is written to a JSON **lineage** record for
audit/reproducibility (`ftune` specialist requirement).

## Simulator note (honesty)
`SimulatedTrainer` collapses "resume-from-best-with-lower-lr" into a re-evaluation at
the best step under the new lr, modelling the knee-shift that a lower lr buys. It exists
so the *control logic* is exercised every run without compute — it is NOT a model of any
specific architecture. Real runs implement `HFTrainerBackend.train_steps/evaluate`
(transformers/TRL on a rented GPU; `evaluate` calls `phase_b/scaffold/eval_harness`).
