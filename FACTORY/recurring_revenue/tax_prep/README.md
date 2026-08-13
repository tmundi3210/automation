# Tax Preparation Practice — FACTORY vertical

**Business.** A paid tax preparation practice: establish the authorization, registration and
electronic-filing standing that permits paid work at all, commit a client segment, service model and
pricing architecture against the season's arrival curve, move each accepted return through a gated
pipeline from signed engagement to logged acknowledgment, and convert each finished season into a
measured returning book with authorized off-season lines. The arc is one system: **practice setup and
demand → per-return delivery → recurring client base and off-season revenue**.

**Money model.** Per-return fees from an annually returning client base — a compressed seasonal peak
that carries most of the year's revenue against costs that run all twelve months, plus off-season
extension, amendment, notice-support, estimated-payment and planning-adjacent education work. Every
economic quantity is a **mechanism**, never a number: how a fee comes to exist (per form/schedule, tier,
hourly, hybrid), what moves the peak ceiling (obligated client-contact time, reviewer hours, rework
reserve, the binding constraint), and how the operator observes each one **in their own logged arrivals,
minutes, rework and client lists** rather than in a borrowed benchmark. No fee, price point, retention
rate, capacity figure or book multiple is asserted as fact.

Preparation is **accuracy- and penalty-exposed**: verification dominates the calendar, and preparer
due-diligence obligations on certain credits and filing statuses are compliance classes to satisfy in
full, never traded against throughput.

## The three knowledge bases

| KB | Scope |
|---|---|
| `kb1_practice_setup_credentials_and_demand` | Authorization and registration obligation classes (paid-preparer identification, firm-level e-file authorization as a long-lead prerequisite, state/local registration layers, the representation-rights boundary), the competence perimeter and due-diligence exposure limit, the refund-neutral honest-offer stance and vulnerable-filer care, taxpayer-data infrastructure before first intake, segment work signatures and the commitment decision, office/virtual/hybrid delivery, software stack, the seasonal arrival curve as the practice's economic engine, the peak ceiling and published intake cutoff, pricing architecture, tier ladder, quote-and-repricing rule, collection terms, the dated pre-season readiness gate and post-season recalibration. |
| `kb2_return_workflow_accuracy_and_documentation` | One return from acceptance to accepted acknowledgment: the preparation-versus-advice boundary, the no-refund-promise conduct rule, the research authority ladder, the safeguards program and incident readiness, engagement acceptance and written scope, prior-year baseline and carryforward inventory, personalized document request and tracked chase, the completeness gate, peak queue triage, the workpaper and tie-out standard, software diagnostics as a partial control, the named error-class taxonomy, the due-diligence record class, ambiguity triage (research / escalate / extend / decline), risk-based review tiers and recorded sign-off, the client walkthrough, authorization capture, transmission and acknowledgment, file retention and secure destruction, post-filing error disclosure and correction, and defect-driven control tuning. |
| `kb3_returning_clients_and_offseason_revenue` | The book between seasons: its counting unit and client states, the causal map of why clients return (earned trust, proactivity, switching friction, inertia), off-season conduct, consent and permitted-use limits, the education-versus-advice line, cohort retention with the extension-lag correction, churn-reason capture, effort-adjusted book tiering, the post-deadline debrief window, next-year positioning, contact rhythm and restraint, clean early disengagement, the extension book worked in waves, amendment and notice-support lines inside the credential boundary, recurring estimate support, general education, the bookkeeping crossover, capacity blocks that protect recovery and pre-season, referral generation at the referrable moment, growth pacing behind the returning base, and the annual book health review. |

Each KB is forged from its `*.spec.json` and gated dense (node/edge/axis/workflow/CQ count bands,
reference integrity, acyclic dependencies, formula consistency). `tax_prep.specialist.json` is the
distilled operating layer over all three.

## Build (forge + gate everything)

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/tax_prep
```

Must end `ALL GREEN`. It forges each `*.spec.json` into `*.kb.json`, gates each KB with
`validators/kb_validator.py --mode dense`, checks that `grounded_in_kbs` resolves to the KBs just
forged, then gates the specialist. Gate the specialist alone with:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/tax_prep/tax_prep.specialist.json \
  --report FACTORY/recurring_revenue/tax_prep/tax_prep.specialist.validation.json
```

## Use the specialist

Load `dist/prompt_template.json` and fill the three slots of `specialist_prompt_template`:

- `{{DOMAIN_LABEL}}` ← `Tax Preparation Practice — Setup, Return Delivery & Returning Client Base`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `tax_prep.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

Send to any capable model. Its `role`, `decision_procedure`, `workflow` and `escalation_triggers` drive
the answer; the KBs are not loaded at answer time. For automatic selection among many specialists, use
the `router_prompt` in the same file with a routing table.

## Boundaries (hard stops carried into the specialist)

- **Credentials are prerequisites, verified currently.** The paid-preparer identification obligation,
  firm-level e-file authorization, and any state or local registration, education, examination, bonding
  or disclosure layer appear only as **obligation classes** with a *verify the current rules in your
  jurisdiction and filing year* instruction. Current standing dominates every commercial plan: the
  opening date or the intake limit moves, never the standing.
- **Process knowledge, not tax advice.** No determination of a particular taxpayer's treatment, filing
  status, deduction, credit eligibility, liability or refund. Fact-specific questions are acknowledged
  the same day and answered only inside a scoped, priced engagement under the return-workflow discipline,
  or referred to a credentialed professional with representation rights; general answers carry a
  general-information label with a source rung and a date.
- **Verification is non-negotiable.** No return advances past a stage gate it has not satisfied, no
  unclear item is closed as assumed, an incomplete due-diligence record holds transmission, and a return
  that cannot be finished properly is extended rather than rushed.
- **No refund promises, no aggressive-position selling.** No promise, implication, comparison or estimate
  of refund size or speed; no fear framing or manufactured urgency; no fee input derived from a refund
  amount. The answer is bounded by what the client's documents support, and losing the client is
  preferred to signing the position.
- **Disclosure-first and vulnerable-client care.** Any refund-funded fee product is presented after the
  direct payment routes, never as the default, with its client-borne cost stated plainly in advance; a
  deliberate slow lane and a comprehension check apply at peak exactly as in the quiet months.
- **Taxpayer data protection dominates convenience.** One indexed secure channel, individually
  credentialed and removable access, multi-factor authentication, a written safeguards program with a
  named responsible person and an incident path; unsafe client-offered channels are declined and
  converted, never quietly tolerated. Retention and secure destruction are scheduled obligation classes.
- **Disclosure over self-protection.** A discovered post-filing error is disclosed promptly in writing
  with option classes and the practice's own borne cost, is never billed as an amendment or notice line,
  and always produces a defect log entry naming the control that missed it.
- **Compliance and finishable capacity dominate growth.** The published cutoff, the four-part off-season
  eligibility filter (authorization, competence, infrastructure, capacity) refreshed each cycle, the
  protected recovery and pre-season blocks, and the returning base's first claim on capacity all outrank
  an attractive acceptance; demand beyond the computed growth room gets a dated next-season slot or an
  outward referral.
- **Honest measurement.** Every reported figure carries its denominator, its as-of date and an
  observed-or-estimated label; retention is provisional until the extension tail has filed and is
  recomputed before anything irreversible turns on it.
