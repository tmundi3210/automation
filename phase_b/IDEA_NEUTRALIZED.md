# Phase B — Neutralized Project Brief

> Orchestrator-owned planning doc (human-readable). The **intent is preserved**; the
> wording is professionalized and de-personalized so it can drive a research + build
> pipeline. Nothing here changes WHAT you asked for — only how precisely it is stated.
> You asked me to use the Knowledge Searcher (Phase A) to work out **how to execute this
> in the real world**, including factors that are *not* in academic books.

## 1. One-line objective
Engineer an **orchestrated system of specialized, fine-tuned small language models for
expert reasoning** ("thinking, not study") — narrow-but-deep specialists, composed by a
non-linear router, fed by a dense machine-facing knowledge pipeline, trained with a
monitored/roll-back-capable loop, and grounded by tools for exact work (math, code, memory).

## 2. Preserved intent — raw element → neutralized statement
| You said (intent) | Neutralized engineering statement |
|---|---|
| "increase intelligence by forcing it to output more, research more" | Exploit **test-time compute** (CoT, self-consistency, tool calls, verifier loops): more *structured* output → higher accuracy on hard tasks. (D9) |
| "specialist for thinking only, not general" | Build **narrow domain specialists**, not a generalist; each masters a few fields (e.g. probability, finance/legal, web/graphic/CS). (D1,D10,D11) |
| "orchestrate multi-models, fine-tune them" | **Per-specialist fine-tuning** (SFT + PEFT/LoRA, preference tuning) over selected base models. (D1,D12) |
| "watch while fine-tuning; if no progress, step back, delete neutral last-N" | **Eval-in-the-loop training** with checkpointing, early-stopping, and **rollback** when the monitored score stalls/regresses. (D1) |
| "AI↔AI conversation logs as training data (the WCO project dir)" | A **data source**: distilled AI-to-AI transcripts → SFT pairs. Needs ingestion (you'll supply the files; see §6). |
| "find academic books/PDFs, break them into chunks, agent per chunk → dense KB (JSON/JSONL)" | The **distillation pipeline** — extends Phase A: PDF→chunk→per-chunk helper→dense machine-facing KB. (D5) |
| "machine-facing dense output only; add a model to convert dense→human-readable" | Dense internal representation everywhere; a **rendering/decompression model** at the boundary for humans. (D5,D7) |
| "strict-format, no-hallucination outputs for probability/math" | **Constrained decoding + tool-grounded computation** for numeric/symbolic work. (D7,D10) |
| "remove personality/refusal layers so sub-agents talk filter-free / reduce conversion loss" | Reframed as **output controllability**: terse, persona-free, machine-to-machine I/O via fine-tuning on dense I/O pairs + format constraints, plus the published **steering/representation-engineering** literature. **Boundary in §5.** (D7) |
| "neutral raw human-readable layer at the end" | A final **neutral renderer** that emits plain, unembellished output. (D7) |
| "tools: live info, my history/what I've read, store + retrieve" | **Tool use + agent memory + retrieval**; a personal-context store the front-end can read/write. (D4) |
| "front-end that understands intent, asks follow-ups, knows direction (quality/money/speed)" | **Intent-elicitation front-end**: user model + clarifying-question policy + an explicit objective dial (quality/cost/latency). (D6) |
| "psychology, linguistics, multilingual (Punjabi/Hindi), IQ/education level, cognitive biases, time-of-day/tiredness" | **User & cognition modeling** for the front-end; estimates are *probabilistic and time-varying*, used to adapt questioning — explicitly **not** clinical diagnosis. (D6; see §5) |
| "models not in series, not parallel, but 3D/4D — any-to-any, weighted, generate by token budget" | **Graph/DAG orchestration**: arbitrary any-to-any edges with **weighted aggregation**; output length **capped at the value-saturation point**. (D2,D9) |
| "summarizer model when many outputs exceed an input" | A **summarizer/compressor** node to bound inter-agent context. (D2,D5) |
| "scoring system; pull real benchmark questions; test yourself" | **Evaluation harness** + curated/real test sets + LLM-as-judge + score tracking. (D3) |
| "select base model per specialist (data, restriction, bench, context, size, tool support)" | **Base-model selection** from model cards/leaderboards under license + capability constraints. (D12) |
| "rent servers, train there; math models crucial + Python for exact math; coding agents" | **Infra** (GPU rental, serving, quantization) + the math/code specialists. (D8,D10,D11) |
| "compressed code/library representation, math to encode→decode" | **Compact code/library representation** as a token-efficiency technique (research item). (D11,D5) |
| "the medicine model trained on raw DNA/protein data that beat specialist models → use raw data" | Research lesson: **train on raw/representative data, not only curated bookish examples**; linked to data-curation findings. (D1) |
| "collect prompts by asking a person to explain X, then pick best option" | A **preference-data collection method** (explain → choose) for SFT/DPO pairs. (D1,D3) |

## 3. Subsystems (the 12 build domains — see taxonomy)
D1 ftune · D2 orch · D3 eval · D4 tool · D5 distill · D6 intent · D7 steer · D8 infra ·
D9 reason · D10 math · D11 code · D12 select. Each becomes a KB set + (later) a specialist,
exactly as Phase A did for the Knowledge Searcher.

## 4. What "research" means here (your definition, honored)
"Research = create specialists on it and run the idea through them." So each domain above is
researched by building its dense KB set (helpers, gated) and distilling a specialist — then
the **architecture decisions are grounded in those KBs**, not in my priors.

## 5. Boundaries (stated up-front, correct me if I misread intent)
- **Filter/layer removal:** I will build the *legitimate* core you actually need — terse,
  persona-free, token-efficient machine-to-machine I/O (via SFT on dense pairs + constrained
  decoding) and a KB of the **published** steering / representation-engineering / refusal-
  direction literature (it exists and is researchable). I will **not** build a pipeline whose
  purpose is to strip a model's safety guardrails to defeat them. Your stated goal (efficiency,
  neutral raw output, less "conversion loss") is fully served without that.
- **User cognition / "IQ / medical / diagnostic":** modeled as *probabilistic, adaptive*
  signals to ask better questions — **not** clinical diagnosis. I'll keep it framed as UX
  personalization, not medicine.

## 6. External dependencies I cannot reach from here
- The **"WCO project" data** at `iCloud Drive/documents/codex/2026/06/21/...` is **not
  accessible** from this container. To use it: add the files into `phase_b/sources/` (or push
  them to the repo), and I'll wire the ingestion. Same for any book PDFs you hold — drop them
  in `phase_b/sources/`. I can also pull **publicly available** academic sources online.
- **GPU training/serving** is not available in this container. I will produce the *architecture,
  configs, and runnable code scaffold* (training-with-rollback loop, orchestration graph,
  ingestion, eval) so it runs on rented GPUs; I cannot execute real fine-tunes here.

## 7. "In books" vs "NOT in books" (your core question, answered at a glance)
Most of the *science* (probability, methodology, cognition, KR, IR) **is** in books — Phase A
already covers the find/evaluate side. Most of the *execution* of this specific system is
**NOT** in books; it lives in **model cards, framework docs, leaderboards, vendor pricing, and
licenses**, and changes monthly. The taxonomy tags this per-domain (`frontier_not_in_books`),
and `phase_b/taxonomy/llm_engineering.taxonomy.json` is the reviewable map.

## 8. Success criteria
A gated KB set + specialists for the 12 domains; a runnable (GPU-portable) scaffold for
ingest → distill → fine-tune-with-rollback → orchestrate → evaluate; and a grounded
architecture decision record that says, for *this* idea, what to build, what to buy, what is
proven, and what is speculative.
