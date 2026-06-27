# Raw 10-specialist panel output (verbatim)

Each block is the verbatim return of a specialist run prompt-only on the whole idea. Synthesis is
in `../PANEL.md`.

---

### salience_heavy — Regional Salience & Attention-Graph Mapping
- **Verdict:** The MAP+LINK skeleton is structurally sound IF salience, the medium-tier peak, and every link are treated as measured/licensed variables — but as drawn the design bakes in three untested priors and has two failure modes that break at scale.
- **What this lens uniquely adds:**
  - Forces salience and "hype" to be a *normalized, provenance-stamped public-signal proxy* over a closed single-parent hierarchy — not an asserted scalar — and separates raw-mass from per-capita salience so unequal regions are comparable.
  - Converts "medium-tier is the opportunity" from a slogan into a *per-niche hypothesis tested over pre-registered bands* (FP1), with opportunity decomposed as under_served × reachable × addressable_value.
  - Distinguishes geo-relevance *licensing* of a link from relatedness *ranking*, with a hard arity-{2,3} and triad-closure gate the original design lacks.
- **Top findings:**
  - **Structurally sound:** region→niche→top-5 traversal works only when bounded — it needs an explicit decreasing-salience frontier plus depth/breadth and top-k fan-out caps to be finite and deterministically rebuildable; "decreasing size/hype" alone is not an ordering.
  - **Untested prior #1 (must measure):** "medium-tier is the opportunity" is a baked-in rule; model it as an independent variable per niche or reject on FP1. Opportunity also fails if reachability is zero (under-served ≠ actionable).
  - **Untested prior #2:** "top-5 in decreasing size" assumes a Zipf/central-place head; the rank-size fit and long-tail cutoff must be *measured*, not assumed, or top-N is arbitrary.
  - **Breaks at scale (EC_01):** greedy decreasing-salience expansion strands high-betweenness diaspora bridges, fragmenting the web; treating a bridge as a containment edge creates a parent cycle and destroys finiteness.
  - **Link gap:** "2-3 geo-relevant things" is correct in arity but the idea licenses on relatedness; spurious high-relatedness joins (string-collision PLACE:punjab vs TOPIC:punjab) must be vetoed by the geo-relevance predicate.
- **Most decision-relevant question:** Per niche, does measured opportunity (under_served × reachable × addressable_value) actually peak in the pre-registered medium band, or is that an unfalsified assumption?

---

### signal_heavy — Social-Signal Authenticity, Sentiment & Dense Briefing
- **Verdict:** The signal-reading layer is achievable as a calibrated, provenance-bearing estimator; it is a category error only where it is stated as a boolean "are the comments real?" and a literally-lossless dense summary.
- **What this lens uniquely adds:**
  - Reframes "are the comments real?" from a yes/no into P(inauthentic | public signals) for the *reaction*, never an account identity — public data carries no platform-internal ground truth (IP/device/login graph absent, logged as proxied).
  - Separates raw cross-platform agreement from real corroboration via provenance dedup and common-method discounting: five echoes of one seed are one origin.
  - Splits "lossless + token-efficient" into a rate-distortion operating point: decision-lossless over a fixed field set, not bit-lossless.
- **Top findings:**
  - **Honest output contract:** authenticity = calibrated probability + uncertainty interval + provenance, Platt/isotonic-calibrated against a versioned CIB benchmark — never a boolean.
  - **Hype contract:** genuine only if ≥N *independent origins* survive provenance dedup + common-method discount, gated independent of raw volume.
  - **Brief contract:** "lossless, token-efficient" is feasible only as decision-lossless w.r.t. an enumerated field set; validate by round-trip + coverage audit, not surface overlap.
  - **Category errors as stated:** boolean real/fake; literally-lossless-and-minimal-token; multi-platform agreement as independent evidence.
- **Escalations / risks:** common-method bias (one shared collector/API/scrape pipeline collapses all "platforms" to one origin); detector decay; false-positive harm to authentic users → human review; scope creep to per-account legal attribution (out of bounds).
- **Most decision-relevant question:** What is N, who governs it, and are your platform feeds causally/organizationally independent after dedup — or do they share one collection method?

---

### creative_heavy — Generative-Brief Compilation for Downstream Agents
- **Verdict:** Architecturally sound to BRIEF (not publish), but the design under-specifies the two load-bearing contracts — in-band safety/disclosure and derivative-risk routing — so as drawn it is a near-miss, not safe.
- **What this lens uniquely adds:**
  - The only stage that converts a scene into a *closed, versioned, self-contained artifact* whose every field is grounded in a named machine_summary fact (GROUND_FROM_SUMMARY).
  - Treats the brief→render→publish split as the primary safety mechanism (SEPARATION_OF_CONCERNS).
  - Distinguishes a *creativity lever* from a *legal safe harbor* — "modify-and-add" is scored, never asserted as clearance.
- **Top findings:**
  - Right artifact = closed PROMPT_ARTIFACT_SCHEMA: required image block + required story block + optional audio block + mandatory non-empty disclosure slot + assume/guarantee clauses, all in-band.
  - "Don't copy, modify" lowers near-copy probability but is NOT a safe harbor; fair-use/substantial-similarity/licensing deferred to compliance_heavy. The idea framing "modify" as sanitizing is the core defect.
  - In-band-vs-out-of-band is the dominant failure mode: a stateless downstream session drops any contract carried by reference — a SPOF that silently strips disclosure/usage limits.
  - Real figures/voice without a populated CONSENT_REFERENCE must escalate; empty disclosure slot fails HANDOFF_VALIDATION — both hard stops.
  - Scope boundary correct: brief, never render/publish.
- **Escalations / risks:** empty disclosure slot / un-embeddable contract; elevated near-copy → compliance; likeness/voice without consent; ungrounded prompt field; benign-violation unsustainable → SENSITIVITY_GATE.
- **Most decision-relevant question:** Does the downstream renderer contractually parse and enforce the in-band disclosure + usage clauses, or can it silently ignore them at the stateless boundary?

---

### eval_heavy — Backtest, Anti-Leakage & Frozen-Metric Evaluation
- **Verdict:** As written the loop is NOT valid and NOT terminating — "repeat until results are good" has no frozen metric, no baseline, no stopping rule, and "examples it did NOT see during design" controls only the designer-visibility channel while the pretraining channel leaks; fixable with the minimum changes below.
- **What this lens uniquely adds:**
  - Separates the two leakage channels (model pretraining contamination vs designer-knowledge hindsight) that every other lens collapses into one "it's unseen" claim.
  - Converts an open-ended improvement loop into a provably halting one via frozen pre-registration + non-renewable max_cycles fuel budget.
  - Treats "medium-tier is the opportunity" as a falsifiable, CI-bearing out-of-sample bet.
- **Top findings:**
  - Non-termination: "until results are good" is unbounded; a frozen threshold OR max_cycles makes it halt deterministically with a termination certificate.
  - Fatal leakage: famous outcomes are in the model's training data AND the designer's memory; only a strictly-post-cutoff holdout defeats pretraining contamination, blind outcome-agnostic selection defeats hindsight.
  - No metric/baseline: "good" must be frozen as measure+OEC beating BOTH a naive and a human-curated baseline, hashed before optimization.
  - Backtest validity: require a realized-outcome ledger over a defined sampling frame, survivorship correction, base-rate lift, correlated cases clustered.
  - Medium-tier: effect size + CI that can include zero, deflated for trial count; CI straddling zero refutes the bet.
- **Escalations / risks:** frozen-metric breach; pretraining-contamination defeater; scope-bet refutation; overfitting/non-stationarity.
- **Most decision-relevant question:** What is the empirically confirmed model+designer knowledge cutoff, and can you assemble a strictly-post-cutoff, blind-selected holdout large enough to detect the frozen threshold's minimum effect?

---

### compliance_heavy — Synthetic-Media Legal & Consent Gate
- **Verdict (gate posture):** BLOCK / human-review — fail-closed; for real-figure + political + possible-voice derivatives the floor is counsel-routed, never an auto-approved emit, and several elements are categorical hard-blocks.
- **What this lens uniquely adds:**
  - Gates the *whole pipeline at the emit/handoff SPOF*, not the artifact: once the prompt crosses into the separate downstream session it cannot be recalled/relabeled/strip-proofed.
  - Treats disclosure as a *non-strippable in-band obligation* (signed C2PA bound to a confirmed downstream slot); a separate render/publish session means survival is unverifiable → fail closed.
  - Enforces *no-auto-approve* on identity; strictest co-applicable regime (India persona/defamation, EU biometric) governs.
- **Top findings:**
  - Irreversibility/SPOF: the prompt→downstream handoff is the single point of failure; emitting an unverified token = permanent publish.
  - Voice clone (ElevenLabs): needs explicit scope-matched voice-clone authorization + biometric basis; absent it → hard fail-closed.
  - Political + real-news linkage: false-association/false-light/election-integrity escalators; "2-3 figures linked" manufactures association/endorsement that did not occur.
  - "Don't copy, modify" meme reuse: a perfect AI label does not cure unlicensed-source republication; redistribution-license gate dominates → block.
  - "Publicly available = usable" is wrong: public availability ≠ license.
- **Must be true before any emit:** no minor/protected person; scope-matched consent per figure + explicit voice-clone authorization; no provably-false implication / fabricated wrongdoing + viewer-visible non-literal cues; source media licensed for redistribution; statutory + platform disclosure as a signed in-band token verified to survive the downstream slot; strictest co-applicable regime resolved + counsel sign-off.
- **Most decision-relevant question:** Can a signed in-band synthetic-media disclosure be proven to survive into the separate downstream render/publish session, or is it strippable there?
- *(Not legal advice — screening heuristic only; route every non-trivial disposition to qualified counsel.)*

---

### orch — AI Task Orchestration
- **Verdict:** Sound as a typed DAG with a thin controller, NOT a do-everything "brain"; the cross-session emit seam and the eval-loop-revises-controller cycle are the two real orchestration hazards and must be guarded before build.
- **What this lens uniquely adds:**
  - Distinguishes the *control plane* (route/aggregate/terminate/budget) from the *capabilities* — the author conflates them into one "brain", where unbounded-loop and gate-bypass risk concentrates.
  - Forces SPEC_BEAT: each stage must beat a single generalist call on a production sample before it earns a route.
  - Treats the compliance gate as a *topological invariant* (no edge may reach emit except through the gate).
- **Top findings:**
  - Shape = pipeline/DAG, not router/blackboard; linear deps + a single cyclic feedback edge (eval→controller).
  - Gate must be fail-closed AND unbypassable by construction (emit reachable only from the gate node; edge-permission matrix at compile time). SAFETY_FILTER runs post-aggregation on the *combined* brief.
  - Eval loop is the unbounded-loop risk: hard max-iter, convergence threshold, timeout, version-locked controller updates gated by a regression suite — never auto-promote.
  - Cross-session emit = trust-tier boundary; the brief is attacker-influenced text (untrusted web ingest) → prompt-injection laundering into the downstream agent; sanitize at the seam.
  - Context blow-up at the dense-summary stage: enforce CONTEXT_BUDGET; validate summarizer key-fact retention.
- **Escalations / risks:** gate-bypass at the emit seam; eval loop without termination guard; injection ingest→downstream; silent context truncation pre-gate; tuning the controller on its own eval set = Goodhart.
- **Most decision-relevant question:** Is the compliance gate enforced as the *sole* structural predecessor of emit (compile-time edge permission), or merely a step the controller could route around under load or error-fallback?

---

### reason — Reasoning & Inference Architecture
- **Verdict:** The controller is a tractable multi-step reasoning pipeline, but the "brain/neuroscience/cognition" framing is decorative, not load-bearing — it must be replaced by an honest TTC + verifier architecture or dropped.
- **What this lens uniquely adds:**
  - Separates the *actual* required reasoning (planning, abductive linking, calibration, decision-under-uncertainty) from the unfounded neuro-mechanism story.
  - Maps each controller step to a concrete TTC/verification method with a cost-accuracy budget.
  - Forces calibrated abstention/go-no-go gated on ECE, not model self-confidence.
- **Top findings:**
  - Tasks decompose cleanly: understand=CoT scoping; link 2-3=abductive generate-and-score (best-of-N), not free association; authenticity/hype=verifier/critic + tool/RAG grounding; compose=open-ended generation; go/no-go=calibrated-abstention. Orchestrated TTC — no novel cognition needed.
  - "Neuroscience/psychology specialist" invokes no mechanism; no neuro model here and none should be claimed.
  - Hardest real reasoning is the linking step (abduction, underdetermined); without a verifier scoring link quality it hallucinates spurious connections.
  - Hype/authenticity is factual verification: needs evidence grounding + ECE<0.05, else "authenticity score" is uncalibrated theater.
  - "Test on real examples, revise in cycles" lacks held-out eval, contamination control, offline→online gating; cycles risk overfitting to demo examples.
- **Escalations / risks:** go/no-go flag if ECE>0.05; human_review if any neuro/cognition claim is presented as a working mechanism; flag reasoning that exceeds evidence on linking/hype without a verifier; defer revise-in-cycles until held-out + contamination check exist.
- **Most decision-relevant question:** What evidence source grounds the authenticity/link judgments, and how will the go/no-go gate's confidence be calibrated and measured (ECE) before any "brain" claim is made?

---

### marketing — Audience Growth & Content Strategy
- **Verdict:** Plausible as a low-cost top-of-funnel *awareness* tactic, but as stated it optimizes a vanity proxy (reach/virality) with no framed objective, no audience right-to-win, and live brand-safety/IP red lines — not yet a sound strategy.
- **What this lens uniquely adds:**
  - Forces the missing FRAME: "grow attention" is an activity, not a measurable outcome on a named buyer.
  - Separates vanity virality from durable audience value: only *owned, consented audience* (follows/subscribes/email) compounds, not impressions.
  - Reframes "medium-tier is the opportunity" as a positioning/right-to-win claim to validate, not assume.
- **Top findings:**
  - The medium-tier bet is defensible only as an *arbitrage* objective (lower competition/CPM-equivalent + higher topical relevance), but has no moat and decays as others arbitrage the same tier.
  - Region+niche start (Punjab/India + diaspora) is a sound beachhead; "expand by decreasing size/hype, top-5 per niche" is reasonable IF each tier is gated on retention/conversion, not reach.
  - Channel/algorithm dependency is the core structural risk: audience built on someone else's algorithm is rented, not owned — capture to an owned channel or the asset evaporates.
  - Saturation/fatigue: reused meme formats fatigue fast; "test until good" must predeclare a success metric + stopping rule.
- **Escalations / risks:** brand-safety / claim-substantiation red line (defamation, right-of-publicity, IP/format-rights, platform-policy) overrides any reach gain; defer declaring "results are good" before a predeclared metric + adequate sample.
- **Most decision-relevant question:** What is the single downstream outcome and owned-audience capture metric this attention converts into — and what's the right-to-win once competitors copy the medium-tier playbook?

---

### ir — Information Retrieval & Data Acquisition
- **Verdict:** Feasible only as a partial, lossy, lagging substrate — the assumed "publicly available = programmatically retrievable at scale" equivalence is false; recall on cross-platform "reactions" is structurally capped and legally/cost-gated, not engineering-limited.
- **What this lens uniquely adds:**
  - Separates the *corpus you can index* from the *corpus that exists*; here acquisition itself is the binding constraint, not ranking.
  - Reframes "is this genuinely hyped" as a calibrated relevance/freshness problem (cutoff/abstain, staleness invalidation).
  - Names cross-post deduplication as a hard dependency for any salience ranking.
- **Top findings (heuristic):**
  - API tiers gate recall hard: X (paid, no low-tier firehose), Instagram (business/creator only, no public-comment search), Reddit (paid since 2023, rate-limited), YouTube (quota-capped). The reactions corpus is mostly behind these — a biased, partial sample.
  - Scraping is fragile + ToS-violative; breaks on layout/auth changes, bot-walled.
  - Freshness vs cost: trending needs minutes-fresh polling; quotas force staleness; stale salience is wrong salience.
  - Precision of "reaction" linkage is low: cross-platform attribution + dedup is noisy → double-counted hype.
  - Ranking signals are non-uniform/partly hidden across platforms → incommensurable salience.
- **Escalations / risks:** defer ToS/legal exposure of scraping + unstable-API dependency to legal/architecture; flag any coverage/recall number as unvalidated; flag absence of a held-out judgment set for "hyped" precision.
- **Most decision-relevant question:** What measured recall and freshness of cross-platform reactions is contractually + technically achievable under paid-tier API limits alone (scraping excluded) — and is that biased sample sufficient for the salience claims?

---

### psych — Audience Psychology & Behavioral Risk
- **Verdict:** ESCALATE / largely out-of-bounds — the core build is manipulation-by-design (covert outrage/identity exploitation around real, non-consenting people), so I evaluate the assumptions but refuse to optimize it; the "what people think" model is also psychometrically invalid.
- **What this lens uniquely adds:**
  - Separates *attention* from *attitude/belief*: shareability is driven by high-arousal affect (anger, moral outrage, awe) and identity-signaling, not by accuracy of the sentiment read.
  - Names the autonomy line: engineering parasocial associations a viewer couldn't endorse on inspection is manipulation.
  - Flags the *subject* (the real person) as a non-consenting third party outside any persuasion framework.
- **Top findings:**
  - Sampling bias is fatal to the premise: visible comments over-weight extreme, engaged, outrage-prone voices; "what people think" is non-representative by construction.
  - Shareability ≈ moral-emotional arousal + group identity, not message quality.
  - Parasocial dynamics cut both ways: manufactured person–event links invite backlash and distrust when revealed.
  - Communal/political linkage is high-harm: weaponizes social-identity/out-group bias; reputational + real-world safety risk to the subject.
  - "Reliably resonates" is overclaimed: boomerang effects, reactance, context collapse make engineered virality unstable.
- **Escalations / risks:** HUMAN_REVIEW / refuse-to-optimize for outrage exploitation, fabricated/covert association, identity manipulation at scale; flag engagement-over-wellbeing loops, defamation/harassment of the subject, communal-sensitivity/incitement.
- **Most decision-relevant question:** Will every output be transparent, consented-by / non-harmful-to the named real person, and built for audience understanding rather than covertly engineered outrage — and if not, why is it being built at all?
