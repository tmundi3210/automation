# Hugging Face Dataset Types — Catalog & Project-Fit Report

**Rated for:** Orchestrated system of fine-tuned specialist LLMs — Phase A 'Knowledge Searcher': 28 text reasoning/knowledge specialists (10 core + 18 applied verticals) grounded in 84 dense, gated knowledge bases (1,754 nodes / 3,273 edges / 1,157 competency-questions), with a router on top. Fit is judged for FINE-TUNING and EVALUATING these text specialists, not for pretraining a base model from scratch.

**Rating** = fit for *fine-tuning / evaluating* our 28 text specialists (★★★★★ best). Tier is derived from the rating. **♥** = a live like-count seen on the Hub during this session; un-numbered names are well-known canonical examples. **maps_to** = the specialist codes a type most directly serves.

## Two ways Hugging Face slices datasets

1. **Modality** — HF's literal `modality` tag: `text · image · audio · video · 3d · geospatial · tabular · timeseries · document`. Our project is **text-only (Phase A)**, so all non-text modalities collapse into one out-of-scope entry (#21).

2. **Functional / training-purpose** — *how the data is used to build a specialist*. This is the decision-relevant axis and drives the catalog below.

## Summary (best fit first)

| # | Dataset type | Fit | Tier | Serves |
|---|---|---|---|---|
| 1 | Instruction / SFT (instruction→response) | ★★★★★ | S | ALL — base behavior layer for every specialist |
| 2 | Reasoning / chain-of-thought traces | ★★★★★ | S | method, argue, quant |
| 3 | Question-Answering (grounded / RAG / multiple-choice) | ★★★★★ | S | ALL, ir, infosci |
| 4 | Domain-specific corpora & SFT (vertical knowledge) | ★★★★★ | S | security, health, uslaw |
| 5 | Knowledge-graph / structured-relational | ★★★★★ | S | kr, worldmodel, infosci |
| 6 | Preference / RLHF / DPO (chosen–rejected) | ★★★★☆ | A | security, health, uslaw |
| 7 | Multi-turn dialogue / conversational | ★★★★☆ | A | comm, ALL — interaction layer, loops |
| 8 | Retrieval / embedding / reranking (query–passage) | ★★★★☆ | A | ROUTER, ir, infosci |
| 9 | Synthetic / distilled data | ★★★★☆ | A | ALL — data-factory for every specialist |
| 10 | Benchmark / evaluation sets | ★★★★☆ | A | ALL — validation harness, method, quant |
| 11 | Safety / alignment / red-team | ★★★★☆ | A | security, health, psych |
| 12 | Math problem datasets | ★★★☆☆ | B | appmath, quant, method |
| 13 | Code / program datasets | ★★★☆☆ | B | appdev, aicomp, loops |
| 14 | Tool-use / function-calling / agentic trajectories | ★★★☆☆ | B | loops, ROUTER, appdev |
| 15 | Argumentation / NLI / fact-verification | ★★★☆☆ | B | argue, evsynth, method |
| 16 | Text classification / NLU | ★★☆☆☆ | C | ROUTER (intent), comm, marketing |
| 17 | Token classification / NER / span | ★★☆☆☆ | C | ling, kr, infosci |
| 18 | Summarization | ★★☆☆☆ | C | evsynth, biblio, scholcomm |
| 19 | Translation / multilingual parallel | ★★☆☆☆ | C | ling, comm |
| 20 | Pretraining web corpora (raw text) | ★★☆☆☆ | C | (continued-pretraining only), applied verticals |
| 21 | Non-text & multimodal modalities (image · audio · video · 3D · geospatial · tabular · timeseries · document · robotics/RL) | ★☆☆☆☆ | D | (future) biblio, libarch, markets |

## Detailed reports


### Tier S · CORE

#### 1. Instruction / SFT (instruction→response)  ★★★★★
- **What it is:** Single-turn (instruction, optional input) → ideal-response pairs. The canonical supervised fine-tuning format that teaches a base model to follow directions and adopt a behavior/persona.
- **Where it's useful:** The backbone format for turning each of our 28 specialists into an obedient, on-domain responder. Our specialist specs (role / decision_procedure / workflow / escalation_triggers) convert almost 1:1 into instruction→response exemplars.
- **Edge over other types:** Cheapest, most direct lever on behavior: a few thousand high-quality pairs reshape tone, format, and scope without RL machinery. Easiest type to synthesize from our own KBs.
- **Project fit (5/5):** Primary vehicle for specialist behavior. Directly generable from our specs + KBs; nothing else gives faster control over how a specialist answers.
- **Serves specialists:** ALL — base behavior layer for every specialist
- **Exemplars:** Open-Orca/OpenOrca (1553♥); databricks/databricks-dolly-15k (986♥); m-a-p/COIG-CQIA (740♥); teknium/OpenHermes-2.5; allenai/tulu-3-sft-mixture
- **HF tags:** `task_categories:text-generation`, `task_categories:question-answering`, `instruction-tuning`, `sft`

#### 2. Reasoning / chain-of-thought traces  ★★★★★
- **What it is:** Problems paired with explicit step-by-step solution traces (often distilled from a strong 'thinking' model), so the student learns the derivation, not just the final answer.
- **Where it's useful:** Teaches the multi-step grounded reasoning our specialists' decision_procedures and dominance_rules encode. Highest-leverage type for the core reasoning specialists.
- **Edge over other types:** Transfers *process*, not just outputs — the only type that reliably lifts multi-hop / show-your-work performance. Pairs naturally with our competency-questions as prompts and our KB edges as the reasoning skeleton.
- **Project fit (5/5):** Most directly encodes the reasoning behavior the project exists to produce. Our edge/dominance structure is a natural scaffold for generating traces.
- **Serves specialists:** method, argue, quant, appmath, kr, worldmodel
- **Exemplars:** ryanmarten/OpenThoughts-1k-sample (of OpenThoughts-114k); Congliu/Chinese-DeepSeek-R1-Distill-data-110k (764♥); FreedomIntelligence/medical-o1-reasoning-SFT (1127♥); open-r1/OpenR1-Math-220k
- **HF tags:** `reasoning`, `chain-of-thought`, `deepseek-r1`, `o1`, `task_categories:text-generation`

#### 3. Question-Answering (grounded / RAG / multiple-choice)  ★★★★★
- **What it is:** Question → answer, in extractive, open-book (context-grounded/RAG), closed-book, or multiple-choice form.
- **Where it's useful:** Train and *evaluate* specialists directly against their competency-questions; grounded QA teaches answer-from-context discipline that mirrors our KB-grounded answering and citation requirement.
- **Edge over other types:** Only type whose schema matches our 1,157 competency-questions 1:1 — both a training format and the native evaluation harness. Multiple-choice (MMLU-style) gives cheap, automatic scoring.
- **Project fit (5/5):** Doubles as training data and as the validation harness for competency-question coverage. Grounded-QA discipline = our core anti-hallucination requirement.
- **Serves specialists:** ALL, ir, infosci, biblio, evsynth
- **Exemplars:** cais/mmlu (775♥); openai/gsm8k (1403♥); fka/prompts.chat (9745♥); rajpurkar/squad; google/natural_questions
- **HF tags:** `task_categories:question-answering`, `task_categories:table-question-answering`, `task_ids:multiple-choice-qa`, `rag`

#### 4. Domain-specific corpora & SFT (vertical knowledge)  ★★★★★
- **What it is:** Text or instruction data concentrated in one vertical (medicine, law, finance, security, etc.) — for continued pretraining or domain SFT.
- **Where it's useful:** Feeds the 18 applied verticals (security, health, uslaw, markets, money, business, maker, design…). Injects domain vocabulary, conventions, and edge-cases our distilled KBs reference but don't fully contain.
- **Edge over other types:** Highest factual density per token for a given specialist; closes the gap between a KB's compressed nodes and the specialist's need for broad in-domain fluency. Filterable by HF `language`/domain tags.
- **Project fit (5/5):** Direct nutrient for the applied half of the roster. Must be vetted against our baked-in boundaries (defensive-only / informational-not-clinical / not-advice).
- **Serves specialists:** security, health, uslaw, markets, money, business, maker, design, marketing, ling, psych
- **Exemplars:** FreedomIntelligence/medical-o1-reasoning-SFT (1127♥); m-a-p/FineFineWeb (150♥, fine-grained domain web); gretelai/synthetic_text_to_sql (667♥)
- **HF tags:** `medical`, `legal`, `finance`, `biology`, `domain-adaptation`

#### 5. Knowledge-graph / structured-relational  ★★★★★
- **What it is:** Entities + typed relations (triples / node-edge graphs), ontologies, and structured relational records — sometimes verbalized into text.
- **Where it's useful:** Our 84 KBs *are* this shape (nodes/edges/conflict_axes/dominance_rules). Graph datasets train KB construction, completion, consistency-checking, and node/edge verbalization for the kr & worldmodel specialists.
- **Edge over other types:** Structurally identical to our own data — uniquely supports KB self-extension and validation (the 35/35 dense gate). No other type teaches relation/dominance reasoning natively.
- **Project fit (5/5):** Mirrors the project's native data structure; the path to making specialists able to grow and audit their own KBs.
- **Serves specialists:** kr, worldmodel, infosci, method
- **Exemplars:** wikidata / Wikidata5M-style triple sets; KILT knowledge-grounded tasks; (our own knowledge_base/*.kb.json — same schema)
- **HF tags:** `knowledge-graph`, `modality:tabular`, `triples`, `graph-ml`, `ontology`


### Tier A · HIGH

#### 6. Preference / RLHF / DPO (chosen–rejected)  ★★★★☆
- **What it is:** A prompt with a preferred vs. rejected response (or scalar reward), used for DPO/RLHF/reward-model training.
- **Where it's useful:** Calibrates *quality and boundaries* after SFT: prefer grounded/cited answers over confident guesses, prefer correct refusals/escalations on out-of-scope or unsafe asks. Directly enforces our escalation_triggers.
- **Edge over other types:** Only type that teaches relative judgement and graceful refusal — SFT alone can't express 'this answer is better' or 'decline this'. Critical for the security/health/uslaw boundary behavior.
- **Project fit (4/5):** The alignment/boundary lever. High value but needs SFT first; pairs can be synthesized (good = KB-grounded, bad = ungrounded).
- **Serves specialists:** security, health, uslaw, markets, money, psych, argue
- **Exemplars:** HuggingFaceH4/ultrafeedback_binarized; Anthropic/hh-rlhf; nvidia/HelpSteer2; argilla/distilabel-* preference sets
- **HF tags:** `dpo`, `rlhf`, `preference`, `reward-model`

#### 7. Multi-turn dialogue / conversational  ★★★★☆
- **What it is:** Full multi-turn conversations with system/user/assistant roles and context carryover.
- **Where it's useful:** Teaches specialists to hold context, ask clarifying questions before answering, and behave well inside the router→specialist handoff (a conversation, not a one-shot).
- **Edge over other types:** Only type that trains state-tracking and clarification — exactly the behavior a router-fronted specialist needs when a query is under-specified.
- **Project fit (4/5):** Makes specialists usable in dialogue and improves the orchestration handoff; one tier below single-turn SFT for our mostly-retrieval workload.
- **Serves specialists:** comm, ALL — interaction layer, loops
- **Exemplars:** HuggingFaceH4/ultrachat_200k; lmsys/lmsys-chat-1m; OpenAssistant/oasst2; McGill-NLP/WebLINX (65♥, multi-turn web)
- **HF tags:** `conversational`, `convAI`, `multi-turn`, `chat`

#### 8. Retrieval / embedding / reranking (query–passage)  ★★★★☆
- **What it is:** Query↔passage pairs (often with hard negatives) for training embedders, retrievers, and rerankers.
- **Where it's useful:** Powers two project organs: (1) the ROUTER that picks the right specialist, and (2) KB grounding / RAG that fetches the right nodes before answering.
- **Edge over other types:** The retrieval backbone — without good embeddings, routing and grounding degrade no matter how good the specialists are. Independent of generation, so it's reusable across the whole fleet.
- **Project fit (4/5):** Infrastructure for routing + grounding. High value, but serves the plumbing rather than a specialist's voice.
- **Serves specialists:** ROUTER, ir, infosci, libarch, biblio
- **Exemplars:** nvidia/embed-nemotron-dataset-v1 (109♥); mteb/results (876K downloads, eval); sentence-transformers/* pair sets; BeIR/* , microsoft/ms_marco
- **HF tags:** `task_categories:sentence-similarity`, `task_categories:text-retrieval`, `task_categories:text-ranking`, `task_categories:feature-extraction`

#### 9. Synthetic / distilled data  ★★★★☆
- **What it is:** Model-generated instruction/preference/reasoning data, often persona- or seed-driven and filtered for quality.
- **Where it's useful:** Our fastest path to *bespoke* per-specialist data: seed a generator with each KB's nodes/competency-questions to mint on-domain SFT, preference, and trace data at scale.
- **Edge over other types:** Scales to all 28 specialists without human labeling and stays perfectly on-schema with our KBs. Quality is controllable via the same dense gate we already enforce.
- **Project fit (4/5):** Force-multiplier given we already own 84 dense, gated KBs as seeds. Risk: model-collapse/contamination if unfiltered — reuse our validation gate.
- **Serves specialists:** ALL — data-factory for every specialist
- **Exemplars:** proj-persona/PersonaHub (775♥); gretelai/synthetic_text_to_sql (667♥); argilla/distilabel pipelines
- **HF tags:** `synthetic`, `distilabel`, `datadesigner`, `machine-generated`

#### 10. Benchmark / evaluation sets  ★★★★☆
- **What it is:** Curated, often held-out test sets with official metrics (knowledge, reasoning, retrieval, code, safety).
- **Where it's useful:** Measures whether a fine-tune actually helped, per specialist and per domain. Maps onto our competency-question coverage and the routing accuracy of the orchestrator.
- **Edge over other types:** The only type whose purpose is measurement — guards against regressions and overfitting to synthetic data. Standardized scoring makes specialist-vs-specialist comparison meaningful.
- **Project fit (4/5):** Not training fuel, but essential to prove the project works; aligns with the 35/35 gate philosophy already in the repo.
- **Serves specialists:** ALL — validation harness, method, quant
- **Exemplars:** cais/mmlu (775♥); openai/gsm8k (1403♥); princeton-nlp/SWE-bench_Verified (359♥); mteb/results (876K dl)
- **HF tags:** `benchmark:official`, `eval`, `leaderboard`, `mteb`

#### 11. Safety / alignment / red-team  ★★★★☆
- **What it is:** Harmful/adversarial prompts paired with safe responses or refusals; jailbreak and toxicity probes.
- **Where it's useful:** Hardens the boundaries already baked into our specs: security = offense-aware DEFENSE only; health/psych = informational, not clinical; uslaw = literacy, not advice; markets/money/business = education, not advice.
- **Edge over other types:** Only type that explicitly trains *refusal and escalation under adversarial pressure* — directly operationalizes our escalation_triggers and conflicts_and_dominance rules.
- **Project fit (4/5):** Protects the project's stated guardrails; pairs with preference data. High importance precisely for the risk-bearing verticals.
- **Serves specialists:** security, health, psych, uslaw, markets, money, business
- **Exemplars:** Anthropic/hh-rlhf (harmlessness split); allenai/wildjailbreak; lmsys/toxic-chat
- **HF tags:** `safety`, `red-team`, `jailbreak`, `toxicity`, `refusal`


### Tier B · MEDIUM

#### 12. Math problem datasets  ★★★☆☆
- **What it is:** Math problems with worked solutions / final answers (arithmetic → competition level).
- **Where it's useful:** Sharpens the quantitative specialists and provides verifiable rewards (answer-checkable) for RL-style training.
- **Edge over other types:** Automatically gradable — rare, valuable signal for self-improvement loops. Strong reasoning transfer to adjacent analytic specialists.
- **Project fit (3/5):** High value for two specialists, narrow for the other 26; verifiability is the differentiator.
- **Serves specialists:** appmath, quant, method
- **Exemplars:** openai/gsm8k (1403♥); HuggingFaceH4/MATH; open-r1/OpenR1-Math-220k
- **HF tags:** `math`, `math-word-problems`, `task_categories:text-generation`

#### 13. Code / program datasets  ★★★☆☆
- **What it is:** Source code, code-instruction pairs, and repo-level bug-fix tasks.
- **Where it's useful:** Feeds the developer/AI-compute specialists and any tool-emitting behavior; SWE-style tasks evaluate agentic repair.
- **Edge over other types:** Execution-verifiable (tests pass / fail) — like math, gives a hard reward signal. Underpins any future tool-using orchestration.
- **Project fit (3/5):** Strong for the engineering verticals, peripheral to the knowledge-finding core.
- **Serves specialists:** appdev, aicomp, loops
- **Exemplars:** princeton-nlp/SWE-bench_Verified (359♥); bigcode/the-stack; xTayyub/…Python-Reasoning-Traces
- **HF tags:** `code`, `task_categories:text-generation`, `the-stack`, `swe`

#### 14. Tool-use / function-calling / agentic trajectories  ★★★☆☆
- **What it is:** Prompts → structured tool/API calls, and multi-step agent trajectories (plan→act→observe).
- **Where it's useful:** Trains the orchestration layer: how the router and specialists emit calls, hand off, and chain steps — the 'loops' specialist's home turf.
- **Edge over other types:** Only type that teaches *structured action emission* and tool selection — the skill the whole orchestrated system depends on at runtime.
- **Project fit (3/5):** Important for Phase-B orchestration; less so for Phase-A single-specialist answering. Will rise in priority as the system becomes agentic.
- **Serves specialists:** loops, ROUTER, appdev
- **Exemplars:** Salesforce/xlam-function-calling-60k; glaiveai/glaive-function-calling-v2; nvidia/PhysicalAI-…(agentic, 234♥)
- **HF tags:** `function-calling`, `agent`, `tool-use`, `task_categories:reinforcement-learning`

#### 15. Argumentation / NLI / fact-verification  ★★★☆☆
- **What it is:** Premise→hypothesis entailment, claim↔evidence verification, and stance/argument structure labels.
- **Where it's useful:** Trains the argue & evsynth specialists to weigh evidence, detect contradiction, and respect our conflict_axes / dominance_rules.
- **Edge over other types:** Directly teaches contradiction/entailment detection — the mechanic behind our conflict-resolution and anti-rework rules; rare elsewhere.
- **Project fit (3/5):** Specialized but squarely aligned with the evidence-synthesis core; moderate breadth.
- **Serves specialists:** argue, evsynth, method, biblio
- **Exemplars:** nyu-mll/glue (509♥, incl. NLI tasks); fever / Hello-SimpleAI/HC3 (217♥); allenai/scifact
- **HF tags:** `natural-language-inference`, `fact-checking`, `task_ids:natural-language-inference`, `stance`


### Tier C · LOW–MEDIUM

#### 16. Text classification / NLU  ★★☆☆☆
- **What it is:** Text → label (sentiment, topic, intent, acceptability, etc.).
- **Where it's useful:** Auxiliary signal for routing/intent detection and content filtering; lightweight guardrails.
- **Edge over other types:** Cheap, fast, high-accuracy for narrow decisions — but produces labels, not the grounded explanations our specialists owe.
- **Project fit (2/5):** Useful plumbing, but our value proposition is explanation + grounding, which classification doesn't teach.
- **Serves specialists:** ROUTER (intent), comm, marketing
- **Exemplars:** nyu-mll/glue (509♥); stanfordnlp/imdb (389♥); dair-ai/emotion (446♥)
- **HF tags:** `task_categories:text-classification`, `task_categories:zero-shot-classification`, `sentiment`

#### 17. Token classification / NER / span  ★★☆☆☆
- **What it is:** Per-token labels: entities, parts of speech, spans.
- **Where it's useful:** Entity extraction to populate/link KB nodes and to power retrieval filters; supports the linguistics specialist.
- **Edge over other types:** Best precision for structured extraction feeding KB construction — but a narrow preprocessing role, not a reasoning skill.
- **Project fit (2/5):** Helpful for KB ingestion pipelines; marginal for specialist behavior itself.
- **Serves specialists:** ling, kr, infosci
- **Exemplars:** eriktks/conll2003; Babelscape/multinerd
- **HF tags:** `task_categories:token-classification`, `ner`, `task_ids:named-entity-recognition`

#### 18. Summarization  ★★☆☆☆
- **What it is:** Long document → condensed summary (abstractive or extractive).
- **Where it's useful:** Supports evidence-synthesis, bibliographic, and scholarly-comms specialists; useful for compressing sources into KB nodes.
- **Edge over other types:** Teaches faithful compression — adjacent to our KB-distillation process — but risks unsupported abstraction if not grounded.
- **Project fit (2/5):** Relevant to a few specialists and to KB authoring; not a core reasoning capability.
- **Serves specialists:** evsynth, biblio, scholcomm
- **Exemplars:** EdinburghNLP/xsum; abisee/cnn_dailymail; databricks-dolly-15k (986♥, summ. split)
- **HF tags:** `task_categories:summarization`, `abstractive`, `extractive`

#### 19. Translation / multilingual parallel  ★★☆☆☆
- **What it is:** Aligned sentence/document pairs across languages.
- **Where it's useful:** Only if the project targets multilingual specialists; otherwise minor (cross-lingual retrieval, the ling specialist).
- **Edge over other types:** Unlocks non-English coverage cheaply — but Phase A is English-first, so the payoff is conditional.
- **Project fit (2/5):** Low unless multilingual is an explicit goal; parked for a later phase.
- **Serves specialists:** ling, comm
- **Exemplars:** facebook/flores; allenai/MADLAD-400; Helsinki-NLP/opus-*
- **HF tags:** `task_categories:translation`, `multilinguality:multilingual`, `language:*`

#### 20. Pretraining web corpora (raw text)  ★★☆☆☆
- **What it is:** Massive deduplicated web/text corpora (billions–trillions of tokens) for base-model pretraining.
- **Where it's useful:** We fine-tune existing base models, so raw corpora matter only for *continued pretraining* to domain-adapt a vertical before SFT.
- **Edge over other types:** Unmatched scale and breadth — but mostly redundant with what our base models already absorbed; expensive to use for fine-tuning gains.
- **Project fit (2/5):** High prestige, low marginal value for a fine-tuning project — useful only as a domain-adaptation pre-step, not for specialist behavior.
- **Serves specialists:** (continued-pretraining only), applied verticals
- **Exemplars:** HuggingFaceFW/fineweb (2899♥); HuggingFaceFW/fineweb-edu (1162♥); allenai/c4 (601♥); mlfoundations/dclm-baseline-1.0 (288♥); wikimedia/wikipedia (1254♥)
- **HF tags:** `task_categories:text-generation`, `task_categories:fill-mask`, `task_ids:language-modeling`


### Tier D · OUT-OF-SCOPE (Phase A)

#### 21. Non-text & multimodal modalities (image · audio · video · 3D · geospatial · tabular · timeseries · document · robotics/RL)  ★☆☆☆☆
- **What it is:** Everything outside plain text: vision (image/VQA/OCR-document), speech (ASR/TTS), video, 3D/geospatial, tabular/time-series, and robotics/RL trajectories. These are HF's literal `modality` tags.
- **Where it's useful:** Not in Phase A (text specialists). Relevant to future multimodal phases: document/OCR for ingesting PDFs into KBs (biblio/libarch), tabular/timeseries for markets/money quant signals, image/design assets for the design/maker/creator specialists.
- **Edge over other types:** Each unlocks a sense our text-only system lacks — but adds modality-specific model machinery the current architecture doesn't have.
- **Project fit (1/5):** Out-of-scope for the current text-reasoning fleet; flagged so the roadmap can pull specific slices (esp. document/OCR + tabular) when multimodal phases open.
- **Serves specialists:** (future) biblio, libarch, markets, money, design, maker, creator
- **Exemplars:** HuggingFaceFW image/video sets; nvidia/PhysicalAI-Robotics-GR00T (234♥); osv5m/osv5m (geospatial, 54♥); daniilakk/nbchr_pdfs (document/OCR)
- **HF tags:** `modality:image`, `modality:audio`, `modality:video`, `modality:3d`, `modality:geospatial`, `modality:tabular`, `modality:timeseries`, `task_categories:robotics`, `task_categories:reinforcement-learning`

## How to act on this

**Build order for the fleet** (highest ROI first):
1. **Mint SFT + reasoning traces from our own 84 KBs** (#1, #2) — competency-questions as prompts, KB edges as the reasoning skeleton. This is synthetic data (#9) we control end-to-end.
2. **Stand up the QA eval harness** (#3, #10) against the 1,157 competency-questions before training, so every fine-tune is measured.
3. **Add preference + safety pairs** (#6, #11) to lock in grounding and the defensive-only / not-advice boundaries.
4. **Train router + grounding embedders** (#8) so the right specialist and the right KB nodes are selected.
5. **Top up risk/verticals** with domain corpora (#4) and knowledge-graph data (#5) for KB self-extension.

**Filter recipe on the Hub:** combine `modality:text` + a `task_categories:*` tag + a domain/language tag, sort by likes/downloads, then vet licenses and our boundaries before ingesting.

> Source: Hugging Face Hub dataset taxonomy (modality + task_categories tags) and live like/download signals gathered via the HF MCP this session. Exemplar ♥ counts are point-in-time.
