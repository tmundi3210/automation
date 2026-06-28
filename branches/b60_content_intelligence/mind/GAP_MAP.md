# GAP_MAP — which intelligence criterion each mechanism closes (honestly)

The intelligence analysis judged the old b60 brain against established definitions and found it
**narrow**: an expert system that doesn't learn, generalize, or persist. This maps each criterion
(the "book critics" — Chollet, Newell, Legg–Hutter — plus ML/agency) to the `mind/` mechanism that
now covers it, and states candidly whether it's **closed**, **partial**, or **depends-on-discipline**.

| Intelligence criterion (the bar) | Old brain | `mind/` mechanism (faculty → memory) | Status |
|---|---|---|---|
| **Learning from experience** (ML) | ❌ "improve loop" re-ran identical inputs, never improved | Learning (`eval_heavy`): seal a claim *before* outcome → reflect → distill TRIGGER→MOVE→EVIDENCE lesson → promote only on ≥k independent confirms with CI excluding zero. Inputs to the next cycle now *change* (`experience/` → `procedural/`). | **Closed, but depends on the consolidation pass actually running** |
| **Agency / goal-directedness** (situated agent) | ❌ every branch a frozen constant; never set goals | Agency (`planner_heavy`): a real goal stack it writes to, self-set subgoals ("I lack X → go get X"), direction set *once* then committed, replan only on a **named defeater** (no thrash, no blind persistence). | **Closed (procedural agency; goals still human-seeded at the top)** |
| **Calibration / uncertainty** (epistemics) | 🟡 one-shot Platt fit on 12 examples, frozen forever | Calibration (`epistemics_heavy`): every falsifiable claim logged as a bet `{prediction, confidence, due, basis}`; scored at outcome; bucketed by confidence; a **recalibration map** rewritten on schedule so "70%" actually means 70%. | **Closed (now adaptive & auditable; still noisy at small n — gated by `min_band_n`)** |
| **Generalization to novelty** (Chollet) | ❌ re-applied one frozen envelope and degraded | Generalization (`reason`): on a new domain, retrieve *structurally* analogous episodes → lift into general **schema** + specific bindings → re-bind → **cheap probe** (CI excludes 0, ECE<0.05) → **ablate** borrowed bindings to separate *acquired skill* from *borrowed prior*. | **Partial — honestly.** It *measures & labels* transfer vs borrowed prior; it composes existing "moves," it doesn't yet invent brand-new ones |
| **Goals across many environments** (Legg–Hutter) | ❌ one fixed objective/domain | Kernel (`orch`): an **environment registry** + per-env context slice + per-env blackboard namespacing; one mind operates across domains/audiences/platforms. | **Partial — environments are registered by hand; it generalizes across listed slices, not yet to fully unlisted ones** |
| **Being a "mind"** (Newell: unified, persistent, self-monitoring) | ❌ stateless between runs, decorative "brain" framing | Generalization+Kernel: one **unified loop** (perceive→recall→plan→act→reflect→consolidate) over **one** diary/schedule/memory with single-write-owner blocks; Memory makes it re-instantiate itself from disk each turn. | **Closed architecturally** (unified *operating procedure over persistent files*), **not** a unified internal representation |
| **Persistent memory / continuity** (the substrate) | ❌ forgot everything between runs | Memory (`signal_heavy`+`epistemics`+`orch`): 4 tiers (working/episodic/semantic/procedural), an INDEX read first every turn, a decision-lossless retrieval slice, and a consolidation "sleep" that turns episodes into beliefs & skills. | **Closed (this is the keystone the other six stand on)** |
| **Curiosity / directed attention** (what to do next) | ❌ purely reactive | Attention (`erotetic_heavy`): a ranked **question queue** scored by value-of-information, surprises spawn high-priority why-questions, a budgeted explore/exploit split feeds generalization. | **Closed (VoI inputs are heuristic priors — relative ranking, not measured information gain)** |

## The honest one-paragraph verdict

`mind/` converts the old narrow expert system into a **persistent, self-directing, self-evaluating
agent** that closes the *learning*, *agency*, *calibration*, *memory*, *attention*, and *unified-mind*
gaps outright, and **partially** closes *generalization* and *multi-environment* — with the limits
named rather than hidden. Two things it deliberately does **not** claim: (1) the LLM's raw priors are
not "its" acquired skill — generalization is *measured and labeled* (ablation) so borrowed prior can't
masquerade as learning; (2) the unity is *architectural* (shared external memory + one loop), not a
single internal mind. The standing risk is **discipline**: skip the write-at-end or the consolidation
pass and continuity silently breaks — the kernel must enforce it, not hope for it. Within those bounds,
this is the most a prompt-run, file-backed mind can honestly be — and it is a real, large step past the
frozen brain.
