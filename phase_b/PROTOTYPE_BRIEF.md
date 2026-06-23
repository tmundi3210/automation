# Prototype Brief — vertical-slice prototype of the orchestrated specialist system

> Produced by running the **Phase-A first set** (Knowledge Searcher) on the need
> *"start the prototype"*: `method` neutralizes + scopes it; `libarch` + `ir` draft the
> sourcing plan. Each section traces to the specialist's `decision_procedure`
> (`specialists/<id>.specialist.json`). Per `method`/`ir` dominance rules, **every numeric
> target below is a HEURISTIC pre-registration value, not observed data** — relabel only
> when real runs supply evidence.

---

## Part A — `method` run: the neutralized one-pager

`method` treats the prototype not as "an app" but as a **falsifiable experiment** whose
result decides whether the whole architecture is worth scaling. Its `decision_procedure`
step 1 forces the foundation first.

### 1. Epistemic aim (decision_procedure §1)
**Control + prediction** — *"Can I stand up an end-to-end orchestrated-specialist loop, and
does it produce better, checkable answers than a single generalist call, for my tasks, at
acceptable cost?"* Not "truth about intelligence" — a buildable, measurable claim.

### 2. Research-question typology + unit of analysis (decision_procedure §1)
- **Q1 (feasibility / descriptive):** does the loop run end-to-end, offline, without crashing?
- **Q2 (comparative / quasi-causal — the real question):** does the **specialist slice beat a
  single-generalist baseline** on my task sample, *net of routing overhead*? (This is the
  ADR's `SPEC_BEAT` rule made testable.)
- **Unit of analysis:** one task instance (a single math/code problem) — the thing that gets scored.

### 3. The smallest testable slice (decision_procedure §3; ADR §4 "vertical slice first")
One domain where **correctness is checkable** → **`math` or `code`**. End-to-end, cheapest path:

```
ingest a few sources → select a base model → serve (base + prompt; NO fine-tune yet)
   → route (orch) → answer with tool-grounding → eval vs held-out checkable set → render
```

Dependency order dominates value order (`method` conflict rule): prove the *loop* before
adding fine-tuning, breadth, or UI. **Defer** the real fine-tune (GPU cost), the 2nd–12th
specialists, and any native UI until G1+G2 pass.

### 4. Success criterion — pre-register BEFORE running (decision_procedure §8; analysis-pre-specification gate)
| Gate | Operationalization | Verdict logic |
|---|---|---|
| **G1 feasibility** | Loop runs on **N = 20** tasks, **0 crashes**, every answer traces to a tool result/source. | Binary pass/fail. |
| **G2 the SPEC_BEAT claim** | On a **held-out, contamination-checked set of K = 30** checkable tasks, slice `pass@1` exceeds the single-generalist baseline by **δ ≥ 10 pts**, *after subtracting routing/overhead cost*. | Pass ⇒ architecture warranted for this domain. **Fail ⇒ specialization did not beat the generalist here — do not scale.** |

`method` escalation: with small K this is **exploratory, not confirmatory** evidence — label
heuristic, and the result **generalizes only to the sampled task type** (no statistical
generalization from a purposive sample).

### 5. Stopping rule (decision_procedure §6; stopping/sufficiency)
Stop the prototype when **any** holds:
- **(a) G1 ∧ G2 pass** on the pre-registered K → proceed to "add 2–3 specialists + run the real
  `SPEC_BEAT` ablation" (ADR §4.2).
- **(b) G2 fails** after a bounded **≤ 5** tune iterations → stop; record "specialist slice did
  not beat generalist for `<domain>`"; pivot domain or shelve.
- **(c) cost/iteration budget B exhausted without G1** → the loop itself isn't feasible yet;
  fix the engineering before re-testing the hypothesis.

### 6. Validity threats → controls (decision_procedure §8 threat-by-control matrix)
| Threat | Why it fakes a win | Control |
|---|---|---|
| **Contamination** (eval set leaked into base model) | inflates G2 | run the harness contamination flag; prefer fresh/checkable tasks; exempt only `choice` items |
| **Weak baseline** | unfair generalist loss | give the generalist the **same tools + a fair prompt** |
| **Underpowered K** | can't confirm | pre-register K/δ; report exploratory + heuristic |
| **Unmeasured routing overhead** | overstates net value | G2 = accuracy gain **minus** overhead (SPEC_BEAT *net*) |
| **Construct slip** ("intelligence") | unmeasurable | operationalize as `pass@1` on checkable tasks, never vibes |

### 7. Escalation / human-review flags (escalation_triggers)
- A causal claim ("specialists *cause* better results") from a non-experimental comparison →
  keep language **bounded/exploratory**.
- Any numeric estimate presented as observed without supplied data → **relabel heuristic**.

### 8. Decision this prototype informs
The ADR's **"gate breadth on a positive result."** G2 pass → scale. G2 fail → pivot or stop.
The prototype exists to make *that* call cheaply, not to ship a product.

### 9. Platform call (the Python / Mac / iPhone / Android question)
The unit of analysis (a scored task) and the loop are **platform-independent**, so `method`
isolates the architecture test from UI engineering (construct-irrelevant variance):
- **Build the prototype in Python.** Fastest to a scored loop; the entire `phase_b/scaffold/`
  is already Python + offline-runnable. The architecture hypothesis is tested *here*.
- **Mac / iPhone / Android are a later, separate usability question** — a thin client over the
  same backend. **The specialist set does not change with platform; only the UI shell does.**
  Do not entangle the two; a UI confound would invalidate G2.

---

## Part B — `libarch` + `ir` run: the sourcing plan

`libarch` owns *negotiate → scope → strategy → screen → reproducible record*; `ir` owns *how to
retrieve well* (relevance notion, hybrid mode, freshness, grounding, verify-vs-reference).

### 1. Typed needs (libarch `need_negotiation_and_typing`; ir intent classes)
| # | Need (feeds which decision) | `libarch` need-type | `ir` intent class |
|---|---|---|---|
| S1 | Base model per specialist (license/context/size/tool-use) → `select` | comprehensive/recall (bounded) | base-model fact |
| S2 | Serving + (later) fine-tune path → `infra`/`train` | known-item | framework how-to |
| S3 | Eval/benchmark tasks for the checkable domain → `eval` | comprehensive | benchmark/eval |
| S4 | Platform/UI framework (only once G1+G2 pass) → front-end | verification | platform/UI |

### 2. Scope contract + stopping rule (libarch `scope_contract_and_stopping_rule`)
- **Precision-favoring**, not a systematic review — the prototype needs *a few authoritative,
  current* sources per decision, not exhaustive recall.
- **Stopping rule:** stop when **each** of S1–S3 has **≥ 1 authoritative + current** source and a
  full pass surfaces **no materially better option**. **Do not expand scope silently** (record
  any change — libarch escalation).

### 3. Source list, ranked by fitness (libarch `search_strategy_and_source_selection`)
| Rank | Source | For | Currency flag |
|---|---|---|---|
| 1 | **This repo's 12 Track-1 KBs** (`phase_b/knowledge_base/llm_engineering/`) | the *science* (already gated/authoritative) | stable |
| 2 | **Official model cards + Hugging Face Hub** | S1 base-model facts | ⚠️ **stale monthly — re-verify** |
| 3 | **Leaderboards + public benchmark sets** | S1 comparison, S3 eval tasks | ⚠️ volatile |
| 4 | **Official framework/SDK docs** (serving stack, UI framework) | S2, S4 | versioned |
| 5 | Grey lit: GitHub READMEs, release notes | currency/edge cases | appraise authority |

### 4. Retrieval method (ir `decision_procedure`)
- **Relevance = graded** (authoritative ∧ current ∧ fit), not keyword match.
- **Hybrid retrieval** (ir §2): lexical for **exact** tokens (model names, SDK symbols, version
  numbers, context-window figures) **+** semantic for vocabulary gaps ("serve a fine-tune
  cheaply" → vLLM/TGI). ir defaults to hybrid exactly when the query mix has exact-match slices.
- **Freshness dominates cache** (ir cache-drift rule; mirrors ADR "always stale"): never trust a
  remembered model/price/leaderboard figure — **re-verify against the live card**.
- **Ground + cite + abstain** (ir RAG rule): every base-model claim cites its model card;
  **abstain/flag** if unsourced.
- **Verify vs reference** (ir validation): check a "fact" (context window, license) against the
  **authoritative card**, never a blog's restatement.

### 5. Screening & provenance (libarch `relevance_screening_bias_and_credibility`; scholcomm light)
Appraise **authority / accuracy / currency**; **official docs + model cards > blogs**; label every
volatile fact with its as-of date; drop or qualify any claim that won't resolve to a source.

### 6. Output of Part B
A reproducible source record (libarch `synthesis_documentation_and_alerts`): for each of S1–S3,
the source, query, as-of date, and the prototype decision it grounds — feeding directly into
`select/model_sheet.json` and the eval set.

### 7. Concrete next action (offline → then live)
The live retrieval surface for **S1** is already available in this session: the **Hugging Face
Hub tools** (`hub_repo_search`, `hub_repo_details`, model cards, `paper_search`) for base-model
facts, plus web search/fetch for official SDK docs. Say the word and I'll execute the S1 sourcing
against live model cards and populate `select/model_sheet.json` with cited, dated figures.

---

## Traceability (method requires every recommendation → competency question + source)
| This brief | Specialist source |
|---|---|
| Part A §1–§8 | `method` `decision_procedure` §1,3,6,8 + `escalation_triggers` |
| Part A §9 platform isolation | `method` construct-irrelevant-variance / dependency-order dominance |
| Part B §1–§2 | `libarch` `need_negotiation_and_typing`, `scope_contract_and_stopping_rule` |
| Part B §3,§5 | `libarch` `search_strategy_and_source_selection`, `relevance_screening_bias_and_credibility` |
| Part B §4 | `ir` `decision_procedure` §1–2, RAG + cache-drift + validation rules |
