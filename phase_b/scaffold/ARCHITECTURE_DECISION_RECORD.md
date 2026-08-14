# Architecture Decision Record — orchestrated fine-tuned specialist system

> Success criterion (`IDEA_NEUTRALIZED.md §8`): "a grounded architecture decision record
> that says, for *this* idea, what to build, what to buy, what is proven, and what is
> speculative." Grounded in the 12-domain Track-1 KBs/specialists, not in priors. This is
> a decision record, not a fact sheet — the volatile rows (model specs, prices) must be
> re-verified against live sources.

## 1. Decision drivers
- **Narrow-but-deep specialists, composed** — not one generalist (ftune, orch, select).
- **Test-time compute** is the intelligence lever: more *structured* output (CoT,
  self-consistency, tool calls, verifier loops) buys accuracy on hard tasks (reason).
- **Exact work is computed, not generated** — math/code go through tools + constrained
  decoding, never free-form guessing (math, code, eval).
- **Dense machine-facing I/O** between agents; a neutral renderer only at the human edge
  (distill, steer). This is the dominant token-cost lever in a multi-hop graph.
- **Monitored training with rollback** — never ship a regressed checkpoint (ftune).
- **Volatile execution layer** — base models, licenses, prices change monthly; the
  science is stable, the *execution* lives in model cards/leaderboards (select, infra).

## 2. Build vs Buy
| Capability | Decision | Why (grounded) |
|---|---|---|
| Orchestration graph (router + weighted any-to-any DAG + summarizer + budget cap) | **BUILD** | Core IP of the idea; cheap to build; no off-the-shelf tool encodes *your* router/specialists. `orch/` does it. |
| Routing | **BUILD** (rule/score first; classifier later) | orch specialist: start rule-based for speed/interpretability; upgrade to a classifier only when task types multiply and the accuracy gain pays the latency. |
| Per-specialist fine-tuning (SFT + LoRA/QLoRA, preference) | **BUILD pipeline, BUY compute** | ftune specialist; PEFT makes this cheap and multi-adapter-per-base feasible. GPUs are **rented**, not owned. |
| Base models | **BUY/adopt open weights** (don't pretrain) | select specialist: adopt permissive open-weight bases; pretraining is out of scope and uneconomic. Frontier API only as teacher/judge or for non-tunable nodes. |
| GPU infra (training/serving/quant) | **BUY/rent** | infra specialist; rent per-run, serve with vLLM/TGI, quantize (QLoRA/4-bit) to fit. No capex. |
| Eval harness | **BUILD thin wrapper, BUY/PULL datasets** | eval specialist: own the scoring/tracking/contamination logic; pull real public benchmarks rather than authoring them. `eval_harness/` does it. |
| Ingestion / distillation (PDF→chunk→dense KB) | **BUILD** (reuse the Phase-A gate) | distill specialist; the gate already exists. `ingest/` reuses it verbatim. |
| Dense↔human renderer | **BUILD deterministic; model-back optionally** | distill+steer; deterministic rendering needs no model and always works offline. `render/`. |
| Vector store / memory / retrieval | **BUY** (off-the-shelf store) + thin client | tool specialist; don't reinvent a vector DB. |
| Intent front-end (user model + clarifying Qs + objective dial) | **BUILD** | intent specialist; product-specific; modest. Adaptive UX, **not** diagnosis (boundary). |
| Steering / format control | **BUILD legitimate path** (SFT on dense pairs + constrained decoding) | steer specialist; literature survey only for repeng. **No** guardrail-removal pipeline (boundary). |

## 3. Proven vs Speculative
**Well-supported (build with confidence):**
- PEFT (LoRA/QLoRA) gives most of full-FT quality at a fraction of cost; multi-adapter
  serving from one base is real (ftune).
- Eval-in-the-loop + early stopping + checkpoint rollback is standard and prevents
  shipping regressions (ftune).
- Self-consistency / majority vote and a summarizer to bound fan-in context are
  established orchestration patterns (orch, reason).
- Tool-grounding for exact numeric/symbolic work beats free-form generation (math, code).
- Contamination is a first-order eval threat; benchmark scores are untrustworthy without
  a contamination check (eval).
- Dense intermediate representations cut inter-agent token cost materially (distill).

**Speculative / needs your own ablation (don't assume):**
- That a *graph* of small specialists beats one strong generalist **for your tasks** —
  orch specialist's `SPEC_BEAT` rule says **measure** net value (accuracy gain − routing
  overhead) before committing; specialization can lose to overhead.
- "Train on raw/representative data beats curated bookish data" — domain-dependent; treat
  as a hypothesis to test per specialist, not a law (ftune data-curation finding).
- Representation-engineering / steering control beyond format constraints — promising in
  the literature, unproven as a production lever here (steer; survey-only).
- Exact monthly model/leaderboard/price figures — **always stale**; verify (select).
- Compact code/library representation as a token-efficiency win — research item, measure
  the decode-fidelity trade-off (code).

## 4. Recommended sequencing
1. **Vertical slice first**: one specialist (e.g. `math` or `code`, where correctness is
   checkable) end-to-end — ingest → select base → fine-tune-with-rollback → serve →
   route → eval. Proves the loop cheaply before breadth.
2. **Add the orchestration graph** with 2–3 specialists; run the `SPEC_BEAT` ablation
   (specialists vs one generalist) on a real task sample. **Gate breadth on a positive
   result.**
3. **Scale specialists** only for domains that clear the ablation and have data.
4. **Harden**: contamination-checked eval sets, score tracking over time, the neutral
   renderer at the human edge, the intent front-end last.

## 5. Risks & boundaries
- **Routing is the weak link** — bag-of-words routing ties on generic needs (documented
  math↔reason overlap). Mitigation already in place: `multi_specialist_policy` composes
  tied leaders; upgrade to a learned classifier when justified.
- **Cost discipline** — fan-out + test-time compute multiply token cost; the budget cap
  and summarizer are not optional.
- **Boundaries (enforced in code):** steer = format/neutral rendering only, not guardrail
  removal; cognition modelling = adaptive UX signals, not clinical diagnosis.

## 6. External dependencies (you supply)
- **Data**: WCO transcripts + book PDFs → `phase_b/sources/` to drive real distillation.
- **Compute**: rented GPUs for actual fine-tunes/serving (this repo is the portable
  scaffold + configs, not a trainer).
- **Live facts**: refresh `select/model_sheet.json` from current model cards/leaderboards
  before any base-model decision.
