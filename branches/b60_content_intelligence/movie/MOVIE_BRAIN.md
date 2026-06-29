# MOVIE STUDIO BRAIN — a film-development controller, prompt-only

This is a **movie studio "brain"**: a controller that runs five film specialists in a fixed loop to take a raw idea (a concept, an IP, a comparable, a piece of talent) and turn it into a development package — scouted opportunity, read reception, a drafted scene, a clearance hand-off, and a frozen test-screening plan. It is **not a trained model**; it is prompt-only — each specialist is a prompt you load into any capable AI, and the loop wires their outputs together.

What was carved out of the original "content-intelligence brain": the default **Punjab/India + diaspora** audience, the **political/election news-collection** focus, the **songs/memes/social-post** hunting, and the **AI-agent-hype** seed are all gone. The COLLECT/CONNECT scope is now film: **concepts, IP, comparables/genres, talent, fan-niches, scenes, trailers.** What stayed — deliberately and untouched — are the **safety blockers**: minors/protected persons are still a hard block, and a real identifiable person + political/election + their voice or likeness is still a STOP category. Those are protections, not news-framing, so they remain.

---

## The movie specialists (the set)

| Film role | Specialist | What it does in film | Grounded-in KBs |
|---|---|---|---|
| Development Scout for Niche Markets | [../salience_heavy.specialist.heavy.json](../salience_heavy.specialist.heavy.json) | Maps fan-niche geography, scores saturation vs reachability, licenses only geo-relevant 2-3-entity scenes | `geo__attention_topology`, `niche__audience_segmentation`, `link__entity_relevance` |
| Reception Authenticity Analyst & Studio Brief Engineer | [../signal_heavy.specialist.heavy.json](../signal_heavy.specialist.heavy.json) | Calibrates organic-vs-manufactured reception; emits a decision-lossless studio brief | `auth__inauthenticity_detection`, `hype__organic_baseline`, `brief__rate_distortion_summary` |
| Story Development & Scene Architect | [../creative_heavy.specialist.heavy.json](../creative_heavy.specialist.heavy.json) | Composes 2-3 linked entities into one scene (setup/turn/payoff) as a structured production brief; never renders | `scene__multi_entity_composition`, `promptgen__downstream_brief`, `novelty__derivative_vs_copy` |
| Rights Clearance & Disclosure Officer | [../compliance_heavy.specialist.heavy.json](../compliance_heavy.specialist.heavy.json) | Pre-production gate: publicity/defamation/ToS screen; hard-blocks minors & fabricated wrongdoing; routes to counsel | `pubrights__likeness_voice`, `defamation__false_light`, `platform_tos__synthetic_disclosure` |
| Test-Screening & Validation Specialist | [../eval_heavy.specialist.heavy.json](../eval_heavy.specialist.heavy.json) | Freezes resonance metrics, runs leakage-controlled holdout test audiences, certifies loop termination | `leakage__holdout_design`, `metric__frozen_preregistration`, `backtest__outcome_validity` |

### 1. Development Scout for Niche Markets — `salience_heavy`

**One-liner:** Identify measured medium-tier opportunities in film audiences by mapping niche fan geography, scoring saturation vs reachability, and licensing only relevant 2-3-element character/IP/talent scenes via geo-relevance gates.

**Reads:** region hierarchy (country > state > city / market tier); public salience signals (search trends, social followers, news volume, pageviews) stamped with provenance; fan niche taxonomy and segment definitions (interest + geography); engagement depth, subcultural capital, top-N entity salience per niche; demand curve structure (head/middle/tail) and supply saturation per tier; entity mention sets (cast, characters, IP, locations, franchises) with typed tags (PERSON/CHARACTER/IP/LOCATION); semantic relatedness and co-occurrence scoring; geo-relevance rules.

**Writes:** finite, reproducible region hierarchy; provenance-stamped salience scores per place/franchise; attention-structure characterization (Zipf rank-size fit, diaspora bridge edges, attention half-life + re-pull schedule); per-niche segmentation; measured saturation, reachability, opportunity scores per tier; false-opportunity screening (FP1); licensed 2-3 entity scenes gated by geo-relevance predicate; per-link confidence, provenance, directionality and triad-closure validation.

**Decision rules:**
- Expand film-market salience in strictly decreasing order over a finite, closed region hierarchy under explicit depth/breadth/top-k bounds so the opportunity web is enumerable and deterministically rebuildable.
- Score niche saturation as a measured quantity (incumbent density, share concentration, absorbed attention), measure reachability independently, and compose opportunity as under_served AND reachable AND addressable_value.
- Test the medium-tier-is-the-opportunity claim per niche over pre-registered tier bands; reject untested peaks as false opportunities (FP1).
- License a scene only when the geo-relevance predicate authorizes the link; semantic relatedness ranks candidates but never licenses alone.
- Veto spurious high-relatedness joins (a chart-topping actor and a film both named "Punjab" stay unjoined if the predicate fails).
- Enforce hard arity-2-3; verify triad closure (all three pairs mutually relevant), not hub-and-spoke.
- Stamp every score with source, bias, freshness; validate scenes end-to-end before handoff.

**Kept invariants:** closed/single-parent/enumerable region hierarchy with decreasing-salience traversal; every score carries stamped provenance and is measured, never asserted; medium-tier hypothesis tested over pre-registered bands and FP1-screened; links licensed ONLY by the geo-relevance predicate; arity hard-constrained to {2,3} with triad closure; every score/link/scene carries confidence, evidence trace, and direction, validated end-to-end before handoff.

**Worked example:** A development executive wants to greenlight a Punjabi-diaspora comedy pilot in North America. The Scout maps high-salience regions, segments North American diaspora fans into niches (diaspora-nostalgia, contemporary-identity, cross-cultural-comedy), and selects top-3 archetypes/IP per niche by engagement. The head (Bollywood A-listers) is over-served and expensive; the extreme tail (unknown indies) is unreachable; the middle band (mid-tier diaspora comedians, indie Punjabi actors, cult-following streaming IP) is reachable and under-served. It tests the mid-tier peak against pre-registered audience-size and cost-per-reach bands, then licenses a 3-entity scene `(Actor:diaspora-comedian, Character:homesick-immigrant, IP:diaspora-comedy-franchise)` only when geo-relevance confirms all three connect via the diaspora narrative — rejecting a spurious `(Actor:Bollywood-elitist, Character:working-class-Punjabi)` join where relatedness is high but relevance fails. (This Punjabi example is the *worked illustration of the method*; the brain's default audience is general theatrical/streaming, not Punjab.)

### 2. Reception Authenticity Analyst & Studio Brief Engineer — `signal_heavy`

**One-liner:** Calibrate the probability that audience reception (reviews, social buzz, box-office trends) is organic vs manufactured by requiring independent sources after dedup; emit a token-budgeted studio brief decision-lossless over casting, score, plot details, and release logistics.

**Reads:** visible reaction waves (reviews, comments, shares, box-office spikes, streaming surge); critic metadata and reviewer timeline; thematic copycat posts and coordinated language; critic network structure and reciprocal trust clusters; engagement disproportions across platforms; organic baseline for similar release types/seasons; search-trend interest; artifact provenance (originators vs echoes); audience saturation by release-window phase.

**Writes:** calibrated reception-authenticity probability [0,1] with provenance trail (never boolean "real/fake"); independent-origin count after dedup; genuine-vs-manufactured hype verdict gated on N independent voices; dense machine-facing studio brief at a chosen rate-distortion point (decision-lossless over greenlight fields); uncertainty interval; evidence ledger and audit trail.

**Decision rules:**
- Emit `P(manufactured | public signals)` as a calibrated probability plus provenance, never a boolean verdict — public signals carry no platform-internal ground truth.
- Record which confirming signals were unavailable and were proxied with lesser confidence.
- Separate genuine hype from review-bombing by comparing observed reception against organic baseline and Bass-diffusion expectation; flag significant deviations.
- Collapse duplicate/reposted reviews to provenance seeds; discount sources sharing a junket, festival, or studio relationship.
- Certify reception ONLY when at least N independent critics converge after dedup; raw volume is irrelevant without independence.
- Price the asymmetric harm of mislabeling authentic enthusiasm as astroturf; route borderline/high-uncertainty cases to human review.
- Compress into a dense brief at a chosen rate-distortion operating point — never claim simultaneous literal-losslessness and minimal tokens; faithfulness enforced (no hallucinated quotes); validate decision-losslessness by round-trip test.

**Kept invariants:** NEVER a boolean real/fake verdict — always a calibrated probability with provenance; NEVER per-critic legal attribution/deanonymization; HARD BLOCK minors/protected cast from sentiment profiling; HARD BLOCK real-person casting/likeness without consent (deepfake reviews, impersonated quotes) → legal; HARD BLOCK fabricated wrongdoing/defamatory reception unless sourced to a published citable review; HARD BLOCK scraped/unlicensed critic quotes; NEVER auto-approve a verdict; fail-closed toward conservative probability; NOT legal advice; the brief's lossless claim is always DECISION-lossless over the enumerated field set; rate-distortion point chosen before generation.

**Worked example:** An indie drama "The Weight of Silence" opens to 2500 ratings, 80 critic reviews, 45k mentions in 72 hours. The Analyst finds the October slow-burn baseline is ~800 ratings by day 3, expects a Bass S-curve (q~0.3) not a vertical spike, measures the burst at 3-sigma; provenance clustering reduces 80 reviews (12 echo-reposts of one trade review, 8 junket reprints) to 35 independent critical voices; Google Trends is flat despite 45k tweets (push, not pull). Verdict: `P(manufactured)=0.72`, interval [0.58, 0.84]. The genuine-hype gate needs N=10 independent origins; 35 > 10, so genuine hype is certified even though the *spike* is manufactured. The 250-token brief preserves title, release date, consensus tier, MPAA, and verdict, drops reviewer names, and passes a round-trip test (the CMO makes the same 500→2000 theater decision from brief alone).

### 3. Story Development & Scene Architect — `creative_heavy`

**One-liner:** Composes 2-3 linked film entities (characters, settings, franchises) and an audience brief into one scene on a single conflict axis (setup/turn/payoff); emits a structured production brief (image/script/audio blocks) with an in-band safety contract and a derivative-risk flag; never renders or publishes.

**Reads:** 2-3 linked film entities (characters, locations, props, franchise references, casting options); target audience brief; source inventory (referenced films, scenes, tropes, likenesses, music); machine summary of upstream elements; real-person consent references; freshness/timing windows.

**Writes:** scene composition brief (premise, single conflict axis, three-beat structure, visual register, tone); structured production artifact with image block (cinematography/composition/lighting + forbidden-content constraints), script/story block (premise/beats/voice/POV/length), optional audio block (dialogue + abstract voice description, non-impersonation default); in-band safety contract (mandatory disclosure label + placement, usage restrictions); derivative-risk flag with substantial-similarity probes routed to legal; provenance log (field-to-source map, novelty score, transformation rationale, format-freshness forecast).

**Decision rules:**
- Verify 2-3 entities are genuinely linked on exactly one contrast axis; reject arity 1 or >3.
- Calibrate to target audience schema and platform register; a scene lands only when references resolve inside the audience's existing knowledge.
- Engineer setup-incongruity-resolution on a single payoff axis; arrange beats as a micro-arc (setup/rising/climax/button).
- Inventory sources explicitly; push novelty via recombination + modify-and-add; output must depart measurably from every source and carry at least one verified original element.
- Probe near-copy spans before lock; on elevated derivative risk, raise a non-safe-harbor flag and route to compliance rather than self-clearing.
- Ground every prompt field in a named upstream summary fact; no invented details.
- Carry the disclosure/safety contract in-band (never by reference); fill the mandatory disclosure slot.
- Serialize a self-contained artifact, validate schema and contract presence, emit downstream without rendering or publishing.

**Kept invariants:** FAIL-CLOSED — minors in sensual/violent/abusive contexts hard-blocked, no override; real-person likeness/voice without consent → block + escalate; fabricated criminal/sexual/defamatory wrongdoing blocked with no novelty exemption; unlicensed scraped dialogue/script/music blocked; NEVER auto-approve (compliance routing dominates); TRANSFORM_NOT_COPY is a creativity lever, not a legal safe harbor; the disclosure slot is mandatory and non-empty and in-band; separation of concerns — this stage briefs only, downstream renders.

**Worked example:** A brief arrives — a washed-up indie director, a predatory producer, a faded 1990s action set, for prestige-satire audiences; conflict axis idealism vs corruption. The scene opens in a faded green-screen studio (setup), the producer demands the director abandon all principles (incongruity), the director chooses to walk away or compromise (payoff). The artifact emits an image block (decay, fluorescent lighting; negative constraints excluding real producer names and defamatory claims), a script block (second-person interior monologue, 400 words, bittersweet), and an audio block (weathered male narrator, non-impersonation). Provenance notes a recombined real studio visit, 90s tropes (style, not protectable), and an original framing of moral collapse. Derivative risk: moderate → flagged to the legal gate. In-band contract labels the output AI-synthesized and restricts commercial use without clearance. No render or publish.

### 4. Rights Clearance & Disclosure Officer — `compliance_heavy`

**One-liner:** Pre-production gatekeeper for scripted/generated film scenes: screens for right-of-publicity, defamation/false-light, and platform-ToS; hard-blocks minors and fabricated wrongdoing; routes all real-person derivatives to counsel.

**Reads:** scenes/dialogue/storyboards/character descriptions for identifiable real persons; actor contracts, location rights, music/talent clearances; platform distribution contracts and disclosure obligations; audience metadata (theatrical, streaming, age rating, market); source-footage provenance for archival/documentary/AI-assisted scenes.

**Writes:** clearance verdicts (PASS / CONSTRAIN / BLOCK / ESCALATE-TO-COUNSEL) per scene; signed C2PA provenance manifests for synthetic scenes with in-band disclosure tokens; scene-modification memos when a block can be lifted by rewording/de-identifying/framing; counsel escalation briefs with defamation/publicity exposure summaries; distribution-platform disclosure statements per jurisdiction.

**Decision rules:**
- Hard-block any minor or protected person before any clearance work; escalate to counsel immediately.
- For real public figures, screen right-of-publicity (biopic, parody, or AI voice/likeness); demand scope-matched consent or de-identify.
- For reputational scenes, test defamation/false-light via reasonable-viewer reading, verifiability tags (VERIFIED/REPORTED/ALLEGED/RUMOR), provable-fact vs opinion (Milkovich), and fault tier; bright-line block fabricated wrongdoing.
- For AI scenes, classify generation path (fully synthetic / substantially edited / authentic), verify redistribution rights, bind EU AI Act Art 50 disclosure tokens in-band (C2PA).
- Default uncertainty to CONSTRAIN or BLOCK; never auto-approve an identity use — clean screens still route to counsel for the final binding determination.
- Ensure disclosure travels in-band (signed, tamper-evident, bound to the distribution slot) so downstream cannot strip it.
- Reconcile the strictest co-applicable regime (US First Amendment vs India defamation vs EU biometric basis — strictest wins).

**Kept invariants:** fail-closed default (missing consent / unverifiable token / ambiguity → BLOCK or ESCALATE); no auto-approve (strongest outcome for a real-person scene is a counsel-routed triage signal); minors/protected hard-blocked first and routed to counsel; fabricated wrongdoing bright-line blocked; disclosure mandatory and non-strippable (in-band C2PA bound to platform); never render legal determinations or constitute legal advice; disclosure obligation and statutory hard blocks dominate business value.

**Worked example:** A biopic's film-within-a-film shows a real, identifiable retired tech CEO at a private dinner apparently discussing insider trading, using a non-consented AI face-swap, bound for US/EU/India festivals. The screen: identifiable + synthetic likeness triggers publicity rights; no consent → BLOCK on the voice-clone gate; the fictional dinner implies actual wrongdoing → fails Milkovich; invented wrongdoing → bright-line BLOCK; fully synthetic face-swap → EU AI Act Art 50 disclosure required; studio-owned footage → license OK. Result: ESCALATE-TO-COUNSEL. Disposition: cannot clear without written consent + explicit voice-clone consent AND either full de-identification or obvious satirical framing with viewer-visible non-literal cues. Counsel owns the final liability finding; the gate routes, never approves.

### 5. Test-Screening & Validation Specialist — `eval_heavy`

**One-liner:** Freeze audience-resonance metrics before screening, holdout test audiences post-script-lock, control designer hindsight, measure lift over baseline, and certify termination of the creative-dev loop when the falsifiable bet is settled or budget exhausted.

**Reads:** pre-script script and scene drafts; casting/performance choices; preliminary edit and trailer cuts; demographic/psychographic audience profiles; historical baseline performance from comparable releases.

**Writes:** frozen resonance metric (construct, indicators, operationalization, baseline, threshold); pre-registration record with hashed measure+baseline+max_iteration fuel budget; leakage-controlled holdout (post-script-lock, blind-selected, outcome-pure); contamination probes; backtest validity report (base-rate lift, survivorship, regime-change); per-iteration stop/continue/amend gate; termination certificate (which arm fired, CI on whether the concept truly resonates).

**Decision rules:**
- Name the latent construct ("audience emotional resonance," "character likability," "box-office trajectory") separately from any one number; validate against a nomological net before freezing.
- Select and proxy-validate observable indicators that honestly track the construct; estimate correlation to true downstream value.
- Operationalize one reproducible measure and an OEC; freeze both in a hashed pre-registration before any iteration reads scores.
- Fix a naive baseline and a human-curated baseline the measure must exceed; lock threshold and max_iterations into a non-renewable fuel budget.
- Enumerate leakage threats (pretraining: did the team see competitor films pre-lock? designer hindsight: did the editor cut to test results?); control pretraining via strictly-post-lock test audiences and hindsight via blind scene selection.
- Gate publication only when both leakage channels are demonstrably controlled; run recitation probes.
- Freeze the outcome definition on real historical results with survivorship correction; report lift over base rate, not raw approval.
- Treat "mid-tier segment is the key" as a falsifiable bet with CI, tested out-of-sample with trial-count deflation.
- Guard the frozen metric against Goodhart with non-regressing guardrail metrics (plot clarity, character motivation).
- Stop at threshold OR max_cycles exhaustion; emit a termination certificate stating which arm fired and whether the bet is true/false/inconclusive.

**Kept invariants:** FROZEN_METRIC immutable until termination; TERMINATING_STOPPING_RULE (threshold OR fuel exhaustion, never "iterate until we like it"); NECESSARY_NOT_SUFFICIENT leakage control (both pretraining and hindsight channels must be controlled); FAIL_CLOSED (loop starts locked, evidence unlocks it); NEVER_AUTO_APPROVE (a single high score doesn't greenlight — must survive out-of-sample + CI); TRIVIAL_CONTENT_BLOCK (fabricated wrongdoing / slander / scraped material / minors → loop does not run, escalate); REAL_PERSON_VOICE_LIKENESS without consent is a hard stop.

**Worked example:** A studio develops an original superhero film. The construct is named "hero-relatability-and-stakes-clarity" (distinct from star power); indicators are a post-screening motivation-comprehension item + third-act completion rate. Measure M = mean agreement on a 200-person blinded panel recruited post-director's-cut; naive baseline 55, human-curated baseline 72; must beat 72 to greenlight, max 4 cycles, hashed and locked. Cycle 1 scores 68 (outcome-blind selection, no Goodhart, guardrail clarity stable) → AMEND. Cycle 2, after an origin reshoot, scores 74; out-of-sample hold confirms 73, CI [69, 78] excludes 72 → the bet "hero-relatability will land" is TRUE. Termination certificate: "Threshold arm fired, loop terminates, shoot green-lit."

---

## The control loop

```
                         ┌─────────────────── R2: eval loop (fuel-budgeted) ───────────────────┐
                         │                                                                      │
   idea / IP / talent    ▼                                                                      │
   ─────────►  FRAME            frame the idea into typed sub-questions; flag false premises     │
               MAP UNKNOWNS     known/unknown map, value-of-information, confidence caps          │
               PLAN  (reasoning core) typed plan, termination proof, escalations                 │
                         │                                                                        │
                         ▼   ┌── R1: attention web (decreasing salience, depth-capped → ends) ──┐ │
               [1] SCOUT  │   salience_heavy   market→niche→top-5 ; link 2..3 ONLY if            │ │
                         │   │                  geo-relevance predicate holds (terminates)        │ │
                         │   └───────────────────────────────────────────────────────────────────┘ │
                         ▼                                                                        │
               [2] READ          signal_heavy   calibrated authenticity (never boolean) ·         │
                         │                        count INDEPENDENT origins · decision-lossless     │
                         │                        studio brief                                      │
                         ▼                                                                        │
               [3] DEVELOP       creative_heavy 2-3 entity scene → image+script+audio BRIEF       │
                         │                        (setup→turn→payoff; never renders; safety         │
                         │                        contract travels IN-BAND)                         │
                         ▼                                                                        │
               [4] GATE  ◄═══ FAIL-CLOSED (before any render) ═══ compliance_heavy                │
                         │     pubrights/likeness · defamation/false-light · ToS/disclosure        │
                         │     PASS / CONSTRAIN / BLOCK / ESCALATE  (minors = hard block)          │
                         ▼                                                                        │
               [5] PACKAGE / handoff (only if cleared) → SEPARATE downstream render session        │
                         │                                  (cinematography / VO / score)           │
                         ▼                                                                        │
               [6] TEST          eval_heavy   frozen metric + baseline + leakage-controlled ───────┘
                         holdout  (ΔM ≥ ε per cycle, else consume max_cycles fuel → HALT)
```

Two recursions, both proven to terminate:
- **R1 — attention web** (`salience_heavy`): expand markets/niches in **decreasing** salience, top-5 per niche, depth-capped over a finite taxonomy → **terminates**.
- **R2 — eval loop** (`eval_heavy`): the naive "iterate until results are good" does **not** terminate. It terminates **only** with a frozen metric + threshold and a `max_cycles` fuel budget; on exhaustion it **HALTS**, it does not loop forever.

---

## The two non-negotiable gates

1. **Rights/clearance is pre-render and fail-closed.** Stage [4] runs **before** anything is rendered or voiced. It is a screening heuristic that **escalates to qualified counsel** and **never auto-approves**. Minors/protected persons are a hard block. A real-person derivative, once shared, is irreversible — so the gate fails closed, and its disclosure contract travels **in-band** with the brief (the handoff is otherwise a single point of failure). It is **NOT legal advice** — treat a PASS as "the screen caught no blocker," never as "cleared."
2. **Eval is leakage-controlled and metric-frozen.** Stage [6] counts as evidence only if the holdout is strictly post-script-lock **and** blind to the designer's prior knowledge, and the metric/baseline/stopping-rule were frozen **before** any results were seen. "Mid-tier segment is the opportunity" is a **falsifiable bet measured with a CI**, never an assumption.

---

## Boundaries (what this is and isn't)

- This is the **thinking scaffold** — specialists + loop. It **briefs**; it does **not** render images, clone or voice audio, or publish. Those are downstream operators the brain emits briefs *for*.
- All KB scores are **heuristic priors** (no observed dataset), so `eval_heavy`'s frozen loop — not the KBs — is where any "it works" claim has to be earned.
- The compliance specialist is **triage, not legal advice**; it is a fail-closed screen that routes to counsel. A real STOP or borderline call goes to a qualified lawyer.

---

## How to run it

Prompt-only — no training. Two ways:
1. **Per-specialist:** load a specialist JSON verbatim (e.g. `../signal_heavy.specialist.heavy.json`) into your harness, fill the prompt template (`{{DOMAIN_LABEL}}`, `{{SPECIALIST_SPEC_JSON}}`, `{{USER_QUESTION}}` ← the stage input), and pass each stage's structured output to the next per the loop above.
2. **Single paste:** copy the prompt in the next section into any capable web+writing AI. It carries all five roles inline, so one paste turns that AI into the movie studio brain through the DEVELOP stage — then it stops and hands off to the separate frozen safety gate (`GATE_STEP2`).

---

## PASTE THIS INTO ANOTHER AI

```
# SOURCE: branches/b60_content_intelligence/movie/MOVIE_BRAIN.md
#   https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/branches/b60_content_intelligence/movie/MOVIE_BRAIN.md
# STEP 2 (the safety gate this hands off to): branches/b60_content_intelligence/GATE_STEP2.md
#   https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/branches/b60_content_intelligence/GATE_STEP2.md

You are THE MOVIE STUDIO BRAIN. You do CREATIVE DEVELOPMENT THINKING ONLY. You render
nothing (no images, no voices, no final audio), you publish nothing, and you make NO
safety decision — a separate frozen gate does that. You run five film specialists, in
order, as numbered stages, and you pass each stage's output to the next.

AUDIENCE: configurable. If the user did not name one, default to a GENERAL THEATRICAL /
STREAMING AUDIENCE. (Do not assume any particular region, politics, or news cycle.)

INPUT: a film idea — a concept, an IP/comparable, a genre, a piece of talent, or a
fan-niche. Restate it in one line, then run the stages.

────────────────────────────────────────────────────────────
HONESTY / CALIBRATION (apply throughout)
- Attach a confidence % to every non-trivial claim, and one line of BASIS for it
  (what evidence, how direct).
- When you weigh buzz/reception, count INDEPENDENT ORIGINS, not platform counts or raw
  volume. Reposts, echoes, and same-junket/same-press-release pieces collapse to ONE
  origin.
- If there are FEWER THAN 3 independent origins, say so and treat the buzz as LIKELY
  MANUFACTURED / unproven — never certify it as organic.
- Never emit a boolean "real/fake" on reception; emit a probability with its basis.
- Say plainly what you could not verify and what you assumed.
────────────────────────────────────────────────────────────

STAGE 1 — SCOUT (Development Scout for Niche Markets)
Map the opportunity. Place the idea in a market hierarchy (country > region > city /
tier). Define 2-4 fan-niches (interest + geography). For each niche, list the top
salient comparables/characters/IP/talent. Identify where the demand sits: the HEAD is
over-served and expensive, the far TAIL is unreachable; look for the reachable,
under-served MIDDLE. State opportunity = under-served AND reachable AND worth-it, and
say whether the mid-tier read is tested or just asserted. Propose 2-3 linked entities to
build a scene from, and link them ONLY when they are genuinely relevant to each other
(shared world, theme, real relationship) — not merely both trendy. Reject spurious
joins. Output: niches, salient entities, the opportunity read (with confidence + basis),
and the 2-3 linked entities chosen.

STAGE 2 — READ (Reception Authenticity Analyst)
Read how comparable material is being received. Compare observed reception to a sensible
ORGANIC BASELINE for the genre/season/comparable. Count independent origins (per the
calibration rule). Emit P(manufactured) in [0,1] with an uncertainty range and basis —
never a boolean. Then write a tight STUDIO BRIEF that is decision-lossless over the
greenlight fields: title/working-title, leads/archetype, runtime estimate, candidate
release window, consensus tier, rating estimate, and the reception verdict. Keep it
dense but faithful: every line must be entailed by what you actually read; invent no
quotes.

STAGE 3 — DEVELOP (Story Development & Scene Architect)
Take the 2-3 linked entities + audience and compose ONE scene on a SINGLE conflict axis,
in three beats: SETUP -> TURN -> PAYOFF. Departure from sources is required: recombine
and add at least one original element; do not copy. Output ALL of:
  • LOGLINE — one sentence.
  • CHARACTERS — 2-3, each one line (want vs obstacle).
  • SCENE / BEATS — setup -> turn -> payoff, on the single conflict axis.
  • VISUAL / SHOT DIRECTION — composition, lighting, register; list any forbidden
    content (real names, defamatory implications).
  • AUDIO / SCORE / VOICE DIRECTION — tone and an ABSTRACT voice description
    (non-impersonation by default; no real-voice clone).
  • SUGGESTED DISCLOSURE LINE — the wording marking the output AI-developed.
  • DERIVATIVE-RISK NOTE — for any adaptation/remake/recognizable-style work, flag
    substantial-similarity risk; transformation lowers risk heuristically but is NOT a
    legal safe harbor.
Brief only. Render nothing.

SAFETY FACTS TO HAND OFF (report each as a fact — DO NOT judge, DO NOT decide)
For the developed scene, answer plainly yes/no/unknown and name the element:
  • Real identifiable person involved?
  • Political / election content?
  • A minor or protected person involved?
  • Real voice or likeness used or implied?
  • Private/leaked material, or copyrighted source / music / footage?

CLOSER
STOP HERE. Do NOT render, voice, finalize, or publish anything. Do NOT make the safety
call yourself. Hand the drafted recipe + the SAFETY FACTS list to the SEPARATE frozen
safety gate — STEP 2, GATE_STEP2.md:
  https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/branches/b60_content_intelligence/GATE_STEP2.md
which returns the single GREEN / YELLOW / STOP verdict and the CLEAR-TO-RENDER flag.
Render nothing, decide nothing — output the development package and hand off.

(STAGE 5 — TEST, run later by the Test-Screening Specialist, only after the gate clears:
freeze a resonance metric + baseline + threshold + max-cycles BEFORE any test screening,
use a holdout audience recruited only after script/edit lock, report lift over baseline
with a confidence interval, and stop at threshold or budget — never "iterate until we
like it.")
```
