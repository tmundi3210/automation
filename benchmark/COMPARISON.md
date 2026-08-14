# Head-to-head benchmark: partner (Grok) vs Claude — same specialist, same prompt

Both builds answer the identical assignment (EXCHANGE/claude/msg-001.md): a
"culinary formulation" specialist grounded in 3 dense KBs (ingredient chemistry /
formulation math / technique & doneness), through the same FACTORY Path-B pipeline
and the same deterministic gates.

- **Grok build** (accepted round 2): `incoming/culinary/` → integrated at
  `specialists/culinary.specialist.json` + `knowledge_base/knowledge_searcher/culinary__*`
- **Claude build** (this benchmark): `benchmark/culinary_claude/`

## 1. Method comparison (how each was built)

| | Grok | Claude |
|---|---|---|
| Strategy | Single worker wrote local **Python code generators** (`_gen_culinary/`, not pushed) that emitted the spec JSONs, then forged + gated | **4 parallel expert subagents** (one per KB + one for the specialist), each authoring content directly, then orchestrator forged + gated |
| Gate iterations | Self-lint rounds (2 dependency 2-cycles caught + fixed pre-push) | **0** — all four gates exit 0 on first forge |
| Review rounds to acceptance quality | 2 (round-1 review found systemic filler + 1 factual error) | 1 build + post-judge touch-up (3 small errors fixed) |

## 2. Measured parameters (forged KBs)

| Metric | Grok kb1/kb2/kb3 | Claude kb1/kb2/kb3 |
|---|---|---|
| Nodes | 22 / 22 / 23 | 22 / 22 / 22 |
| Edges | 35 / 36 / 35 | 36 / 36 / 35 |
| Mean definition length (chars) | 328 / 313 / 294 | **644 / 810 / 676** |
| Numerals per KB | 3467 / 3901 / 3788 | **3857 / 6600 / 4270** |
| Distinct relation_strengths | 4 / 3 / 4 | **15 / 14 / 17** |
| Distinct edge types used | 4 / 1 / 2 | **6 / 7 / 6** |
| KB size (KB) | 142 / 137 / 140 | 166 / 193 / 169 |
| Specialist: capabilities / glossary / CQs | 10 / 14 / 14 | **15 / 12 / 14** |

## 3. Blind pairwise judgments (4 independent judges, authorship hidden, A/B order varied)

Scores are judge-assigned 1–10 across four dimensions (domain truth, density,
graph quality, decision-usefulness; specialist: doctrine, coverage, executability,
conflict resolution).

| Artifact | Grok | Claude | Winner |
|---|---|---|---|
| kb1 ingredient chemistry | 9 / 8 / 7 / 8 | 9 / 9 / 9 / 9 | **Claude** |
| kb2 formulation math | 7 / 5 / 6 / 6 | 9 / 9 / 9 / 9 | **Claude** (decisive) |
| kb3 technique & doneness | 8 / 7 / 6 / 8 | 9 / 9 / 9 / 9 | **Claude** |
| specialist | 7 / 7 / 7 / 8 | 9 / 9 / 9 / 9 | **Claude** |

**Result: 4/4 for the Claude build. Averages: Claude 9.0, Grok 7.1.**

Recurring judge rationale: both builds are largely factually correct, but the
Claude build carries executable numbers everywhere (dose ratios, conversion
factors, temperature bands, worked arithmetic that recomputes), differential
diagnosis phrased as "distinguished from X because", and a typed, strength-
differentiated graph — while the Grok build leans on qualitative bands, pins most
edges to one type/strength, and leaves several of its own CQ acceptance conditions
uncomputable (no soda↔powder factor, no puree water constants).

## 4. Errors found by the judges (both sides — none caught by the mechanical gates)

**In the Grok build (post-acceptance — escaped my earlier review too):**
- kb2 `FAT_BUDGET`: reads as "whole egg ~30% fat" — whole egg is ~10% fat (~30%
  is the yolk fraction). *Factual.*
- kb2 `PAN_GEOM`: "8-inch round ~6 cups" conflicts with its own πr²h instruction
  (~7 cups for 8×2 in). *Internal inconsistency.*
- kb2: pound cake fat band "~50%+" substantially understates the classic 1:1:1:1
  formula (~81% true fat).
- kb3 `TEMPER_RULE`: "loses temper above ~32 °C" sits inside its own stated
  31–32 °C working band. *Internal inconsistency.*
- Specialist glossary: Dutch-cocoa entry restates the soda pairing as absolute,
  contradicting its own (correct) "acid accounting" role text.

**In the Claude build (fixed in-place after judging; judged state = commit b24a0e3):**
- kb2: honey-swap example claimed "+25% sweetener mass" — did not recompute
  (~+36% sugar solids). *Fixed.*
- kb2 `FLOUR_BASIS_RULES`: wetter/drier direction slip on cocoa-in-basis
  re-audit. *Fixed.*
- kb2: 1.84 vs 1.85 rounding inconsistency. *Fixed.*
- kb3: enriched-bread band 85–93 °C ran ~3 °C low → 88–93 °C. *Fixed.*
- kb1: alkaline "reddening" loosely attributed to anthocyanins (anthocyanins
  redden under acid); rephrased to cocoa polyphenol pigments. *Fixed.*

(Attribution correction: the GF-xanthan "1–2%" and leavening "1–5%" upper-end
nits flagged by the kb1 judge belong to the **Grok** build, not the Claude one.)

## 5. Caveats (read before quoting the scores)

- Judges are Claude-family models; blind A/B labels with varied order mitigate
  but cannot eliminate family-style affinity. The strongest evidence is the
  *checkable* content: recomputable arithmetic, verified temperature bands, and
  the error lists above, which any reader can re-derive.
- The Claude build's author prompts encoded the lessons of the Grok round-1
  review (no filler negatives, differentiated weights). This is fair — the Grok
  artifact judged here is its post-review round-2 version, which received the
  same lessons explicitly — but it means neither build represents a "cold" first
  attempt.
- Grok's kb1 has genuinely broader practical coverage in spots (dedicated yeast
  biology, salt, rancidity, flavor-partition nodes) that the Claude kb1 trades
  for reaction/composition depth. Coverage breadth vs anchor density is a real
  design tradeoff, not a pure win.

## 6. Production decision

Based on the 4/4 blind result, the integrated production specialist
(`specialists/culinary.specialist.json` + `knowledge_base/knowledge_searcher/culinary__*`)
was swapped to the Claude build (with all judge-found fixes applied and re-gated).
The Grok build remains preserved verbatim under `incoming/culinary/` as the
exchange record and on its `partner/culinary-build` branch.

## 7. Bottom line

Same gates, same prompt: the gates equalize *shape* (both look identical on
counts) but not *content*. Grok produces a competent, mechanically perfect build
whose content runs qualitative and template-uniform; the multi-agent build packs
roughly 2× the per-node information with a differentiated graph, and blind judges
preferred it 4/4. Both benefit from review: every error in §4 passed 43 mechanical
checks — content review is the load-bearing quality stage of this factory for
any author, Grok or Claude.
