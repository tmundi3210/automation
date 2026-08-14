# Phase B — Track 2: the runnable scaffold

GPU-portable, **offline-runnable** code for the orchestrated fine-tuned specialist
system, grounded in the Track-1 research layer (25 dense KBs + 12 specialists +
ROUTER + INDEX). Every subsystem runs here on CPU with a deterministic mock backend;
the real GPU/API paths are marked **hook points** so nothing silently needs compute or
makes paid calls.

> Read `../IDEA_NEUTRALIZED.md` for the brief + the two boundaries (§5: format-control
> and neutral rendering, **not** guardrail removal; cognition modelling = adaptive UX,
> **not** clinical diagnosis). Both are enforced in the code below.

## How the pieces fit (the system loop)
```
 sources/ ──ingest──▶ chunks ──distill(+GATE)──▶ dense KB ──┐
                                                            ▼
   select ──base model per specialist──▶ train(rollback loop, eval-in-loop)
                                                            │ served checkpoint
                                                            ▼
        need ──orch.router──▶ orch.graph (weighted any-to-any DAG
                              + summarizer + token-budget cap) ──▶ answer
                                                            │
                              eval_harness (score + track)  │
                                                            ▼
                                    render.dense_to_human (neutral boundary)
```

## Grounding map (subsystem → Track-1 specialist → what it implements)
| dir | specialist(s) | implements |
|---|---|---|
| `common.py` / `backends.py` | — | loaders for the committed artifacts; routing tokenizer mirrored from `tools/build_router.py`; pluggable `ModelBackend` (Mock/Api/Local) |
| `ingest/` | distill | PDF/text → token-bounded chunks → seeded KB draft → **the real gate** (`compute_kb_formulas` + `kb_validator`) → distillation work order |
| `train/` | ftune | eval-in-loop training with checkpoint **rollback**, neutral-last-N prune, lr decay, early stop, JSON lineage (simulated; HF hook) |
| `orch/` | orch | `route()` faithful to `ROUTER.json`; weighted **any-to-any DAG** + summarizer (context bound) + aggregator (value-saturation cap) |
| `eval_harness/` | eval | per-task scorers (tool-grounded numeric / choice / open via LLM-as-judge), pass@k, contamination flag, score tracking; the `evaluate()` the loop calls |
| `select/` | select | base-model selection: hard filters (license/context/size/tool) + objective dial; per-domain map. Sheet is a verify-before-use template (volatile frontier data) |
| `render/` | distill, steer | dense↔human boundary: deterministic Markdown renderer + persona-free compressor (format only, never guardrail removal) |

## Run everything
```bash
bash phase_b/scaffold/run_all.sh        # smoke-tests every subsystem offline
```
Or individually — see each subdir's README. Quick tour:
```bash
python3 phase_b/scaffold/common.py                          # loads 25 KBs / 12 specialists
python3 phase_b/scaffold/orch/router.py                     # routing table
python3 phase_b/scaffold/orch/graph.py "fine-tune with LoRA and roll back on stall"
python3 phase_b/scaffold/ingest/chunk_to_kb.py --unit ftune__demo_ingest
python3 phase_b/scaffold/train/rollback_loop.py
python3 phase_b/scaffold/eval_harness/harness.py --system oracle
python3 phase_b/scaffold/select/select.py --map
python3 phase_b/scaffold/render/dense_to_human.py phase_b/specialists/orch.specialist.json
```

## Real vs hook (honesty)
- **Real & runnable now:** all routing, the DAG engine, the ingest chunker + gate reuse,
  the rollback control logic, the eval scorers + tracking, the selector logic, and the
  deterministic renderer.
- **Hook points (need compute/keys/data):** `ApiBackend`/`LocalBackend` (a served
  fine-tuned checkpoint), `HFTrainerBackend` (real GPU fine-tune), the LLM-authoring pass
  that turns an ingest draft into a 35/35 dense KB, and real model-sheet figures.
- **Needs your data:** drop WCO transcripts / book PDFs into `phase_b/sources/` and point
  `ingest/pdf_to_chunks.py` at them.

See `ARCHITECTURE_DECISION_RECORD.md` for what to build vs buy, and what is proven vs
speculative for *this* system.
