# Health / Medicare Insurance Agent — FACTORY vertical

**Business.** A licensed, certified, Medicare-focused independent insurance agent practice: earn and
re-verify the right to contact anyone at all, market inside the current plan year's rule layers, run
need-driven enrollments the beneficiary decides, then service the enrolled book year-round.

**Money model.** Revenue is enrollment compensation plus renewal compensation under **regulated
schedules** and carrier contracts. Compensation is treated as a **mechanism** — the product class that
decides which regime applies, the regulated maximum republished on a cycle, the carrier's contracted rate
inside that ceiling, the per-member rate-year determination of initial versus renewal level, the payment
path through any pass-through entity, and the recoupment classes that can take money back — verified per
carrier and plan year against the actual schedule, statement or portal record, never stated as an invented
percentage, amount, multiple or valuation. The durable asset is a **persistent, correctly-attributed,
well-served book**, so the practice is judged on members still enrolled and still served at the horizon
rather than on applications submitted.

This is the most heavily regulated vertical in the recurring-revenue set: **compliance dominates growth in
every conflict**, without exception.

## The three knowledge bases

| KB | Scope |
|---|---|
| `kb1_certifications_contracts_and_compliant_marketing` | The credential stack as requirement classes (state license and continuing education, the annual AHIP-class training plus carrier product certification, contracting, errors-and-omissions, upline hierarchy and release, composite ready-to-sell verification); the CMS marketing-rule classes (permission to contact, scope of appointment, prohibited practices, material review, event classes); compliant lead generation, third-party-vendor diligence, digital presence and community education; and the documentation, privacy, self-audit and allegation-response habits that survive an audit. |
| `kb2_enrollment_periods_and_plan_fit` | The enrollment-window system as the demand calendar (initial, annual election, open-enrollment and special classes with their evidence and effective-date rules); structured needs intake before any plan name; existing-coverage audit, creditable-coverage and late-charge exposure, assistance-program screening; provider-network and formulary verification; coverage-route architecture and the guaranteed-acceptance one-way door; the compensation-neutral comparison set; and the compliant enrollment conversation, submission, post-submission verification and documentation pack. |
| `kb3_renewal_compensation_service_and_retention` | Renewal compensation, per-member rate-year and continuation status, agent-of-record position, chargeback exposure and member-level reconciliation as regulated mechanisms; the year-round service desk (triage, provider disruption, drug-cost events, denials and billing) inside role limits, with the service-to-sales line detected and stopped; the annual review where "no change" is a complete outcome; risk-based book segmentation and consent-bounded cadence; retention through service quality, persistency diagnostics, compliant referral growth and book continuity as a contractual position. |

Each KB is forged from its `*.spec.json` and gated dense (node/edge/axis/workflow/CQ count bands, reference
integrity, acyclic dependencies, formula consistency). The specialist
`medicare_agent.specialist.json` is the distilled operating layer over all three.

## Build (forge + gate everything)

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/medicare_agent
```

Must end `ALL GREEN`. It forges each `*.spec.json` into `*.kb.json`, gates each KB with
`validators/kb_validator.py --mode dense`, checks `grounded_in_kbs` resolves to the KBs just forged, then
gates the specialist. Gate the specialist alone with:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/medicare_agent/medicare_agent.specialist.json \
  --report FACTORY/recurring_revenue/medicare_agent/medicare_agent.specialist.validation.json
```

## Use the specialist

Load `dist/prompt_template.json` and fill the three slots of `specialist_prompt_template`:

- `{{DOMAIN_LABEL}}` ← `Health / Medicare Insurance Agent Practice: Certification & Compliant Marketing, Enrollment-Period Plan Fit, Renewal Compensation & Retention`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `medicare_agent.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

Send to any capable model. Its `role`, `decision_procedure`, `workflow` and `escalation_triggers` drive the
answer; the KBs are not loaded at answer time. For automatic selection among many specialists, use the
`router_prompt` in the same file with a routing table.

## Boundaries (hard stops carried into the specialist)

- **Compliance dominates growth, always.** Federal Medicare marketing rules, every footprint state's
  producer-conduct code, each carrier's marketing policy and contract, and any upline policy **stack** —
  apply the strictest applicable layer and record which layers were compared. A tactic that cannot produce
  a permission or approval artifact does not run.
- **Certification and appointment are prerequisites, not paperwork.** No marketing, appointment or
  enrollment for any carrier, state or product whose composite **ready-to-sell** status (active license,
  in-force appointment, passed current-year annual and carrier certifications, contract in good standing)
  has not been confirmed from that carrier's own portal. A lapse stops that activity the same day.
- **Verify current rules in your jurisdiction.** Every threshold, interval, retention period, gift limit,
  continuing-education hour, window date, effective-date rule, compensation ceiling and chargeback period is
  an **obligation class**, not a fact to recite from memory — read it in the current-year source for your own
  state, carrier and county.
- **Educational, not personalized advice.** No personalized plan recommendation and no financial, legal,
  tax, medical or clinical determination. Keep-or-drop questions about employer, retiree, union, veterans'
  or marketplace coverage go in writing to the administrator; representation, medical-necessity argument and
  drug choice go to an appointed representative, the prescriber or the state's free counseling program. **The
  enrollment decision belongs to the beneficiary.**
- **No steering by compensation, in either direction.** Criteria fixed in writing before any candidate set
  exists; the do-nothing baseline retained; constraint failures shown rather than hidden; at least one
  non-maximizing option present; the member-side reason for a *stay* documented as rigorously as for a move;
  and no attribution position ever reclaimed by contacting a member.
- **No fabricated figures.** Commissions, premiums, cost-sharing, penalties, subsidy thresholds, gift limits,
  lead costs and book valuations appear as mechanisms, qualitative bands, or the practice's own dated
  measured numbers — each with the schedule, notice, statement or portal record that would produce the real
  current-year figure.
- **Seniors are a protected population and ethical contact is first-class.** Disclosure-first identification
  (name, licensed agent, which carriers you actually represent, not the government); consented channels and
  humane hours; plain language and a pace set by the beneficiary; comprehension checked by restatement in
  their own words; a trusted person of their own choosing welcomed and documented; no fear framing, no
  manufactured urgency, no deadline that cannot be sourced to that person's real window; stop requests
  immediate and permanent; and a documented **hard stop** where capacity, comprehension or signing authority
  is in genuine doubt.
- **Everything leaves an artifact, built at the time.** The operating test is whether a third party with no
  memory of the meeting could reconstruct why this person ended up with this coverage — produced complete
  inside the shortest applicable response window.
