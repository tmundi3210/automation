# render — dense ↔ human boundary

Grounded in the 'distill' + 'steer' specialists and `IDEA_NEUTRALIZED.md` ("dense
internal representation everywhere; a rendering/decompression model at the boundary for
humans" + "a final neutral renderer that emits plain, unembellished output").

## Files
- `dense_to_human.py` — decompress a dense KB or specialist JSON into readable Markdown
  (auto-detects type; `--neutral` = plainest, length-bounded). Fully deterministic.
- `human_to_dense.py` — compress human notes into a terse, persona-free skeleton +
  salient-term index; reports the compression ratio (the "conversion-loss" reduction).

## Run
```bash
python3 phase_b/scaffold/render/dense_to_human.py phase_b/specialists/orch.specialist.json
python3 phase_b/scaffold/render/dense_to_human.py \
        phase_b/knowledge_base/llm_engineering/ftune__sft_peft.kb.json --neutral --out /tmp/ftune.md
python3 phase_b/scaffold/render/human_to_dense.py            # built-in sample (ratio ~0.32)
python3 phase_b/scaffold/render/human_to_dense.py notes.md --model   # + optional model pass
```

## Boundary (IDEA_NEUTRALIZED §5) — read this
Both directions are **format/density transformations only**. `dense_to_human` is the
neutral, unembellished renderer for humans; `human_to_dense` strips human-oriented filler
to cut inter-agent token cost. Neither removes or alters safety behavior. The optional
model-backed pass in `human_to_dense` must preserve meaning and guardrails — it is for
tightening prose, never for stripping refusals. The deterministic baselines need no model
at all, so the boundary renderer always works offline.

## Notes
- `dense_to_human` runs against the *actual committed* KBs/specialists — it is the
  human-facing window onto the otherwise machine-facing Track-1 artifacts.
- Compression is measured on the dense payload (not JSON boilerplate); the directive
  string is constant metadata.
