# B60 — full-panel read of the idea (10 specialists)

The whole idea (not a single seed) run **prompt-only** through a 10-specialist panel: the **5 new
`b60` specialists** + **5 old-but-appropriate** specialists chosen for lenses the idea needs.
The 3 reasoning-core specialists (`planner/erotetic/epistemics_heavy`) already read the idea in
`SCOPE.md`; their findings are folded into the synthesis below. Each specialist was loaded
verbatim into `dist/prompt_template.json` and reasoned as itself.

## Headline

The idea is **buildable as plumbing, but not as claimed, and not safely as stated.** Every
load-bearing claim is independently flagged by the specialist that owns it, and **two specialists
reach a hard stop on the as-written core**: `compliance_heavy` (BLOCK / counsel-routed for
real-figure + political + voice derivatives) and `psych` (refuse-to-optimize: the core is
*manipulation-by-design around non-consenting people*, and the "what people think" model is
psychometrically invalid). The new five tell you *how* to build each capability honestly; the old
five tell you *whether you should*, *whether you can get the data*, and *whether the brain holds
together*.

## What the OLD specialists added beyond the scope (the reason to run them)

- **`orch`** — make the compliance gate the **sole structural predecessor of `emit`** (compile-time
  edge-permission, not a step the controller can route around under load); the ingest→downstream
  path is **prompt-injection laundering** (untrusted web content reaching a privileged downstream
  session); the eval loop is the **unbounded-loop** risk and tuning the controller on its own eval
  set is **Goodhart**; enforce a context budget before `generate`.
- **`reason`** — the **linking step is abduction** and needs a verifier or it hallucinates spurious
  connections; authenticity/hype is a factual-verification task needing grounding + **calibration
  (ECE < 0.05)**, else the score is "theater"; the **"neuroscience/cognition" framing is decorative**
  — drop it or relabel it metaphor, it is prompt-orchestration + verifiers, not a brain.
- **`marketing`** — there is **no framed objective** ("grow attention" is an activity, not an
  outcome); audience built on a recommendation algorithm is **rented, not owned** — capture to an
  owned channel (follow/email) or the asset evaporates; the **medium-tier bet is arbitrage with no
  moat** (decays as competitors copy the format).
- **`ir`** — **"publicly available ≠ programmatically retrievable at scale"**: API tiers (X/Reddit
  paid, Instagram comment search closed, YouTube quota-capped) **cap recall on "reactions"**;
  scraping is fragile + ToS-violative; freshness fights cost; **data acquisition is the binding
  constraint, not the model**.
- **`psych`** — the visible-comment sample is **vocal-minority/outrage-biased**, so "what people
  think" is non-representative by construction; shareability ≈ **moral-emotional arousal + identity
  signaling**, not message quality or accuracy; engineering parasocial person↔political-news
  associations around a **non-consenting subject** crosses the manipulation line and invites backlash.

## Convergence (independently reached, ≥3 specialists each)

| Finding | Reached by |
|---|---|
| "Medium-tier is the opportunity" is an **unproven prior**, not a result | salience, eval, marketing (+ scope FP1) |
| "**Lossless + token-efficient**" is a category error → rate-distortion / decision-lossless | signal (+ scope FP2) |
| The eval loop **doesn't terminate / leaks** as written | eval, orch, reason (+ scope FP3/FP5) |
| Bot/"real comments" is **not a boolean** → calibrated probability + provenance | signal, reason (+ scope FP4) |
| Multi-platform agreement ≠ corroboration (**single-origin / common-method**) | signal, ir (+ scope FP7) |
| "**Publicly available = usable**" is false; legal gate dominates and is irreversible | compliance, marketing, psych (+ scope KU5) |
| The **emit→downstream handoff is the SPOF**; contract must travel in-band | creative, compliance, orch |

## Posture by specialist

| Specialist | Posture on the idea as stated |
|---|---|
| `salience_heavy` | sound skeleton **if** salience/medium-tier/links are measured & licensed, not asserted |
| `signal_heavy` | achievable **as a calibrated estimator**; category error only where stated as boolean / literally-lossless |
| `creative_heavy` | architecturally sound to **brief (not publish)**; under-specifies the two load-bearing contracts |
| `eval_heavy` | **NOT valid / NOT terminating** as written; fixable with frozen metric + leakage-controlled holdout |
| `compliance_heavy` | **BLOCK / counsel-routed** (fail-closed); several elements are categorical hard-blocks |
| `orch` | sound as a **typed DAG with a thin controller**, not a do-everything brain; guard the emit seam + eval cycle |
| `reason` | tractable TTC pipeline; **"brain/neuroscience" framing is decorative** and must be dropped or relabeled |
| `marketing` | low-cost awareness tactic, **not yet a strategy** (no objective, rented audience, no moat) |
| `ir` | feasible only as a **partial, lossy, lagging substrate**; acquisition is the weakest link |
| `psych` | **ESCALATE / refuse-to-optimize**: manipulation-by-design around non-consenting subjects; invalid sentiment model |

## The single most decision-relevant question, per specialist

- **salience** — Does measured opportunity actually peak in the medium band per niche, or is that unfalsified?
- **signal** — What is N (independent origins), who governs it, and are your feeds actually independent after dedup?
- **creative** — Does the downstream renderer contractually enforce the in-band disclosure, or can it silently ignore it?
- **eval** — What is the confirmed model+designer knowledge cutoff, and can you build a strictly-post-cutoff blind holdout big enough?
- **compliance** — Can a signed in-band disclosure be proven to survive into the separate render/publish session?
- **orch** — Is the compliance gate the *sole* compile-time predecessor of emit, or merely a step the controller could route around?
- **reason** — What evidence grounds the authenticity/link judgments, and how is the go/no-go gate's confidence calibrated (ECE)?
- **marketing** — What single owned-audience outcome does this attention convert into, and what's the right-to-win once copied?
- **ir** — What recall + freshness of cross-platform reactions is achievable under paid-API limits alone (no scraping)?
- **psych** — Will every output be consented-by / non-harmful-to the named real person and built for understanding, not engineered outrage?

## Bottom line

Run honestly through its own panel, the idea resolves to three things you can keep and three you
must change before building:

**Keep:** (1) the staged pipeline shape (orch confirms typed DAG); (2) geo-relevant 2–3 linking as
a *licensed* operation (salience); (3) a calibrated signal layer + decision-lossless brief (signal).

**Change before building:** (1) replace "test until good" with a frozen-metric, leakage-controlled,
terminating eval loop (eval) and stop calling it a "brain/neuroscience" (reason); (2) make the
compliance gate an unbypassable, fail-closed, counsel-routed predecessor of emit — and drop the
"publicly available = usable" assumption (compliance, orch); (3) confront the two existential
constraints the new five can't fix on their own — **you may not be able to get the data** (ir) and
**the as-stated content goal is manipulative around non-consenting people** (psych). The second is a
*decision*, not a bug: decide what this is for and whom it must not harm before any of the rest matters.

Raw per-specialist output is in this file's commit; the seed-level run is `RUN.md`; the build is `SCOPE.md` → `BRAIN.md`.
