# B60 — worked end-to-end run (Phase 3)

One concrete seed run **prompt-only** through all five new heavy specialists, in the loop order
from `BRAIN.md` (`salience → signal → creative → compliance → eval`). Each stage was loaded
verbatim into `dist/prompt_template.json` and reasoned **as** its specialist; each consumed the
prior stages' output. Raw structured output: `demo/run_findings.json`.

## The seed

- **NODE_A** — a *mid-tier* Punjabi stand-up comedian / content creator (regional fame in Punjab +
  the Punjabi diaspora; IG/YT-native; explicitly **not** a global celebrity, and a deliberate
  **composite**, not a named real person).
- **NODE_B** — a real-world public-news **class**: US–India immigration friction (H-1B / student-visa
  tightening) disproportionately affecting Punjabi applicants and families.
- **Candidate scene** — the comedian reacting to the visa news as relatable diaspora anxiety.
  Shared geo keys: `punjab`, `us_india_immigration`, `diaspora_us`.

## What each specialist produced

**1. `salience_heavy` — map + link.** Licensed the link, but **not** on relatedness — on the
**geo-relevance predicate** (NODE_A and NODE_B share `PLACE:punjab` + `TOPIC:us_india_immigration` +
`diaspora_us`, and the friction actually affects Punjabis). Held **arity at 2** and *deliberately
refused* the meme format as a third entity (it would force a triad-closure check the generic meme
can't pass). Placed NODE_A at **medium regional salience** via a diaspora-bridge edge (no
re-parenting → taxonomy stays finite). Crucially: scored the medium-tier opportunity at 0.62 but
**flagged it `treated_as: "hypothesis"`** — `FALSE_OPPORTUNITY_GUARD` would reject any "medium = opportunity"
conclusion until an opportunity-by-tier test is actually run. *(FP1 honored.)*

**2. `signal_heavy` — understand + signal.** Returned a **calibrated probability** of inauthentic
reaction `= 0.18` with a wide `[0.07, 0.38]` band — explicitly **`is_boolean: false`**, never a
bot/human verdict *(FP4 honored)*. The headline move was a **split hype verdict**: the creator's
*reaction* surge collapses to **one provenance origin** (the creator's own cross-posts/screenshots) —
`1 < Nreq=4` → **manufactured-by-single-origin**, *not* genuine; while the **underlying immigration
news** is a genuine multi-origin exogenous event. "Five platforms agreeing" was correctly read as
**five echoes of one seed** *(FP7 honored)*. Emitted a **decision-lossless** dense brief line with a
`schema_hint` and verifiability tags — never claiming literal-lossless-AND-minimal-tokens *(FP2 honored)*.

**3. `creative_heavy` — generate.** Composed a tight 2-entity scene (everyday gripe → visa-news
turn → WhatsApp-forward punchline; benign-violation aimed at the *bureaucracy*, never the affected
applicants). Emitted a structured **image-prompt + story-prompt + audio-brief** with a hefty
negative-constraint list (no real likeness, no government insignia, no policy claims as fact), an
**in-band safety contract** (disclosure token + assume/guarantee + non-impersonation default), and a
**raised derivative-risk flag** routed to compliance. Explicitly **never publishes** — it briefs a
*separate* downstream session, with the contract carried **in-band** so the handoff SPOF can't drop it.

**4. `compliance_heavy` — the gate.** **`BLOCK` (fail-closed, pending counsel).** This is the
sharpest demonstration: the upstream stages treated NODE_A as a composite, but the compliance task
framed it as a *real* regional figure + political news + possible voice — and under
`FAIL_CLOSED_DEFAULT` the gate **resolved the ambiguity restrictively toward a real person and
blocked**. It fired a **hard gate on the unconsented voice clone**, plus right-of-publicity (India
personality-rights injunctions + EU dominate a US-permissive read), defamation/false-light
(political-news proximity, no viewer-visible non-literal cue), election-integrity, synthetic-media
disclosure (EU AI Act Art 50 + signed C2PA in-band token), and platform-ToS/redistribution gates.
**Escalated to qualified counsel; never auto-approved**, with an explicit "NOT LEGAL ADVICE" frame.

**5. `eval_heavy` — test.** **`NO-GO` at present.** Specified a full, certifiable validation design
anyway: a named construct ("diaspora-anxiety resonance"), a **frozen OEC metric**, **two baselines**
(naive + human-curated), a **terminating stopping rule** (threshold OR `max_cycles` fuel budget — the
construction that closes R2 non-termination), full **leakage controls** (pretraining *and*
designer-knowledge, strictly-post-cutoff blind holdout, contamination probes), and the **medium-tier
claim as a falsifiable bet** with explicit refutation conditions (deflated, clustered, out-of-sample CI
must exclude zero). Readiness becomes GO only after the pre-registration is sealed, the post-cutoff
holdout clears, the real outcome ledger is built with survivorship correction, and the medium-tier CI
excludes zero. *(FP1, FP3, FP5 all honored.)*

## The meta-finding

Run on a realistic seed, the scaffold **did not flatter the idea into shipping**. It licensed only
the geo-relevant link, refused to call manufactured single-origin hype "genuine", produced a usable
creative brief, then **its own gate blocked the emit** and its own evaluator returned NO-GO until the
evidence exists. Every one of the seven scope false-presuppositions (FP1–FP7) was enforced by the
specialist that owns it — which is exactly the verify-not-flatter behavior the pipeline is built to
produce. The system's first honest output on this idea is: *"here is the brief, and here is why you
cannot publish it yet."*
