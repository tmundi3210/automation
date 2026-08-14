# Payroll Service / Referral Business — FACTORY vertical

**Business.** Acquire small-business payroll clients and stand on one of three structurally different
rungs: **introduce-only referral partner** (the platform owns contract, billing, service and liability),
**wholesale reseller** (the operator buys platform capacity at a partner rate, owns the invoice, the
receivable and the first support call), or **service bureau on platform** (the operator does the recurring
payroll work inside the platform's partner console and charges for that labour). The arc is one system:
acquisition and offer economics → onboarding, migration and cutover → recurring compliance operations,
retention and expansion.

**Money model.** Referral or revenue-share compensation from a payroll platform, plus the recurring
per-client relationship (a service fee for labour on the higher rungs). Compensation is treated as a
**mechanism**, never as a number: the payment form (one-time introduction payment on a qualifying start,
an ongoing share of what the client pays, a wholesale discount realized as spread, a service fee for
labour), the drivers that move it, the attribution mechanism that decides whether it is ever credited, and
the reversal/clawback classes that can take it back — each read per deal from the **current partner
agreement or price sheet with a dated extract on file**. No commission rate, platform price, per-employee
fee, clawback window, capacity figure or book multiple is ever asserted as fact.

Payroll is a **compliance-critical** domain: errors land as penalty, interest and unpaid employees on the
client. Compliance-critical accuracy dominates growth, convenience and throughput in every conflict.

## The three knowledge bases

| KB | Scope |
|---|---|
| `kb1_referral_models_and_client_acquisition` | The engagement-rung ladder and what each rung owns; scope-of-practice, data-minimization and compensation-disclosure frame; partner compensation and attribution as verifiable mechanisms with clawback exposure; platform and partner-program diligence; complexity profiling, employer registration classes, classification-risk routing and honest fit-or-decline; referral channels (bookkeepers and accountants, tax practitioners, insurance advisors, associations), the covenant that keeps them open and the restriction classes to verify before promising a share; switching triggers, the year/quarter boundary calendar, pressure-free discovery, the two-part offer and the acquisition handoff packet. |
| `kb2_client_onboarding_and_setup` | The onboarding project: framing and complexity triage against a first live pay date; the three-party responsibility map (platform, service layer, client) with no shared duties; authorization, agency-account and least-privilege access classes; cutover-boundary choice and its migration consequences; roster, deduction, contribution and withholding-order migration; year-to-date balance loading tied to filed returns; direct-deposit and company funding setup; employee transition comms in the employer's voice; parallel-run verification, filed-return reconciliation, the all-green readiness gate, prior-provider exit, supervised first live run, correction paths, the compliance-calendar handoff and the closing evidence file. |
| `kb3_recurring_relationship_and_expansion` | The live book after go-live: the standing service charter and duty register; least-privilege standing access with out-of-band verification of money and identity changes; question routing out of scope; per-cycle preflight, archived run record, post-run variance review and funding/settlement watch; the compliance calendar as a rolling control through quarter close, year-end sweep and the new-year parameter reset; correction-window triage, agency-notice classes, issue-ownership routing, platform escalation craft and own-error disclosure; retention through defect-free runs, churn signals, residual-statement reconciliation, capacity and segmentation; need-signal expansion inside the ethics and licensing guardrail; and offboarding without leverage. |

Each KB is forged from its `*.spec.json` and gated dense (node/edge/axis/workflow/CQ count bands,
reference integrity, acyclic dependencies, formula consistency). The specialist
`payroll_services.specialist.json` is the distilled operating layer over all three.

## Build (forge + gate everything)

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/payroll_services
```

Must end `ALL GREEN`. It forges each `*.spec.json` into `*.kb.json`, gates each KB with
`validators/kb_validator.py --mode dense`, checks `grounded_in_kbs` resolves to the KBs just forged, then
gates the specialist. Gate the specialist alone with:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/payroll_services/payroll_services.specialist.json \
  --report FACTORY/recurring_revenue/payroll_services/payroll_services.specialist.validation.json
```

## Use the specialist

Load `dist/prompt_template.json` and fill the three slots of `specialist_prompt_template`:

- `{{DOMAIN_LABEL}}` ← `Payroll Service / Referral Business: Engagement Models & Client Acquisition, Onboarding & Migration, Recurring Compliance Operations & Expansion`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `payroll_services.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

Send to any capable model. Its `role`, `decision_procedure`, `workflow` and `escalation_triggers` drive the
answer; the KBs are not loaded at answer time. For automatic selection among many specialists, use the
`router_prompt` in the same file with a routing table.

## Boundaries (hard stops carried into the specialist)

- **Compliance-critical, and not advice.** Payroll tax filing carries assessed penalty and interest
  exposure for the client. Nothing here is tax, legal, accounting or employment-law advice: determinations
  reserved to the employer or a licensed professional — worker classification, exempt status, taxability of
  a specific item, which deposit schedule applies, how to answer a specific agency notice, entity form,
  contract enforceability, insurance adequacy — are routed out **in writing**, with the question recorded,
  no recommendation attached, and only the client's own recorded determination configured.
- **Referral and revenue-share terms are mechanisms.** Verify current partner-program terms per deal and
  keep a dated extract; test the attribution mechanism end to end before the first introduction. No rate,
  tier, price or clawback window is recalled or assumed.
- **Client data is the densest concentration of harm a small employer holds.** Aggregate-first
  qualification, auditable intake channels, employee self-entry of identifiers and bank details where
  possible, the narrowest role that does the job with the client retaining owner-level control, dated
  revocation on departure, and **out-of-band verification of every instruction that changes where money
  goes**. Never hold agency credentials as a convenience.
- **Verify current rules in your jurisdiction.** Registration thresholds, account formats, local tax lists,
  deposit schedules, filing deadlines, form identifiers, retention periods, penalty amounts and abatement
  paths appear only as **obligation classes**, never as recited facts.
- **No fabricated figures.** Commissions, revenue shares, platform prices, per-employee fees, service
  prices, penalty amounts, attach rates, clients-per-person capacity and book multiples are mechanisms,
  qualitative bands, or the operator's own dated measured numbers.
- **Disclosure-first, no fear-selling, vulnerable-operator care.** The compensation relationship is stated
  at the first mention of a platform and mirrored in writing; fit is decided before compensation; compliance
  risk is described in class terms with the client's own documents as the only case-specific evidence — no
  penalty figures, no audit-probability claims, no deadline that cannot be sourced to that client's real
  window; where distress, exhaustion or non-comprehension appears, the pace slows, the recommendation goes
  in writing, a trusted adviser is welcomed and a same-day commitment is declined.
- **Never touch the withheld funds boundary.** No assistance with delaying, reducing or borrowing against
  the tax portion of a run: present lawful option classes, escalate to the client's tax professional and
  counsel, document the boundary, and resign rather than assist.
- **Never act as the professional of record.** No deposits, filings, amendments or agency representation,
  and no position taken before an agency on a client's behalf.
- **Expansion never uses dependence.** Offers come from dated need signals with evidence, never in the same
  conversation as an unresolved incident or notice, never as a condition of the base payroll service; and
  benefits, retirement, insurance and HR-legal work are **introductions into licensed channels** with a
  compensation disclosure delivered first and consent recorded, never operator recommendations.
- **Gates over dates, disclosure over self-protection, autonomy over retention.** The readiness gate moves
  the go-live date rather than the standard; an operator-caused error is disclosed before the client
  discovers it and before the remedy is known; and a departing client gets a fast, complete, no-leverage
  exit with the filing-responsibility split written down.
