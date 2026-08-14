# ingest — PDF/text → chunks → gated KB draft

Grounded in the 'distill' specialist and `IDEA_NEUTRALIZED.md` ("break books into
chunks, agent per chunk → dense KB (JSON/JSONL)"). Reuses the Track-1 gate verbatim.

## Files
- `pdf_to_chunks.py` — source docs → token-bounded JSONL chunks (paragraph-aware,
  with overlap so claims spanning a boundary survive). `.txt`/`.md`/`.json` native;
  `.pdf` via optional `pypdf` (clear hook message if not installed).
- `chunk_to_kb.py` — chunks → seeds one node per chunk → assembles a structurally
  valid KB **draft** → runs `tools/compute_kb_formulas.py` + `validators/kb_validator.py`
  (the real gate) → reports the honest verdict + the gaps + a distillation **work order**.
- `_demo_source.txt` — committed fixture so both stages run with zero user data.

## Run (offline, no user data needed)
```bash
python3 phase_b/scaffold/ingest/pdf_to_chunks.py            # chunks the demo fixture
python3 phase_b/scaffold/ingest/chunk_to_kb.py --unit ftune__demo_ingest
```
With real data, drop files into `phase_b/sources/` and point the chunker at them:
```bash
python3 phase_b/scaffold/ingest/pdf_to_chunks.py phase_b/sources/ --out phase_b/sources/_chunks.jsonl
python3 phase_b/scaffold/ingest/chunk_to_kb.py phase_b/sources/_chunks.jsonl --unit <domain>__<subdomain>
```

## Honest by design
A handful of chunk-seeded nodes is **below dense quantity on purpose** — the deterministic
gate returns e.g. `fail (24 pass / 11 fail)` with the 11 `count.*` checks listed as the
gaps. The 24 passing checks prove the draft is structurally sound and referentially
consistent; the work order names the authoring job (`prompts/_a4_phaseb_job.md`) that an
LLM pass runs to reach a 35/35 dense pass — the same author→gate contract Track 1 used.
This stage does NOT fabricate a passing dense KB.

## Notes
- Drafts write to `ingest/_drafts/` (gitignored) — never under `knowledge_base/...`,
  because `build_index.py` globs `*.kb.json` and a draft would otherwise be ingested.
- The LLM-authoring step is a hook point: wire a `ModelBackend` to draft node
  definitions/metrics from each chunk, then let the gate decide.
