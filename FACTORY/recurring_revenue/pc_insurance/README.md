# pc_insurance — Property & Casualty Insurance Agent

A FACTORY vertical: 3 dense knowledge bases + 1 distilled specialist, all gate-passing.

## The business and the money model

A licensed property and casualty agent sells auto, home and small-commercial policies.
Revenue is **new-business commission plus renewal commission**, so the in-force **renewal book
is the asset** — a bound policy pays once for the placement and again every term it persists.
Every economic quantity in this vertical (commission rate and base, settlement path, chargeback
trigger, contingency formula, acquisition cost, retention, runway, book-valuation drivers) is
described as a **mechanism to compute from the operator's own agreements, carrier statements and
management-system records** — never as an invented figure, rate or multiple. Regulatory content
names the **obligation class** and its proof artifact and tells you to **verify the current rules
in your jurisdiction**.

## The three knowledge bases

| KB | Covers |
|---|---|
| `kb1_licensing_appointments_and_agency_model.kb.json` | Licensing and appointment obligation classes, entity licence and designated responsible producer, captive vs independent vs franchise vs employed-producer economics and ownership of expirations, carrier access paths (direct, aggregator/cluster, wholesale, surplus lines), commission and contingency as mechanisms, errors-and-omissions posture, market-focus selection, disclosure-first solicitation conduct. |
| `kb2_quoting_binding_and_coverage_design.kb.json` | Duty frame and conduct before any quote, risk intake and exposure discovery, loss/driving/coverage-history verification, replacement-cost valuation basis, honest application accuracy, limits, funded deductibles, exclusion-gap mapping, endorsements, umbrella layering, auto/home/small-commercial program design, catastrophe-peril track, appetite triage, normalized multi-carrier comparison, proposal ethics, offered-and-declined records, bind readiness and the issued-policy audit. |
| `kb3_renewals_retention_and_agency_book_value.kb.json` | Book as asset and segmentation, system-of-record hygiene, honest retention measurement, the dated renewal pipeline, renewal-offer decomposition and the increase conversation, remarket-or-hold rule and gap-free rewrite execution, carrier standing, hard/soft market posture, claims-time advocacy inside the licensed scope, everyday service, retention-vs-new economics, churn diagnosis, need-based account rounding and limits review, renewal-cycle errors-and-omissions exposure, book-valuation drivers and transferability. |

Each KB is forged from its `*.spec.json` and gated dense by `validators/kb_validator.py`;
`*.validation.json` and `*.metrics.json` hold the gate records.

## Build (forge + gate everything)

```bash
cd /home/user/automation
bash FACTORY/build.sh FACTORY/recurring_revenue/pc_insurance
# ends: ALL GREEN — every KB and the specialist passed every deterministic gate.
```

Specialist gate on its own:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/pc_insurance/pc_insurance.specialist.json \
  --report FACTORY/recurring_revenue/pc_insurance/pc_insurance.specialist.validation.json
```

## Using the specialist

`pc_insurance.specialist.json` is an operating spec, not a model. Drop it into the harness at
`dist/prompt_template.json` and send the result to any capable model — fill the three slots of
`specialist_prompt_template`:

- `{{DOMAIN_LABEL}}` ← the spec's `domain_label`
- `{{SPECIALIST_SPEC_JSON}}` ← the **entire** `pc_insurance.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

The model then reasons strictly by the spec's `role`, `decision_procedure` and `workflow`, honours
every `escalation_trigger`, and answers only inside the domain. Invoke it for: starting or
restructuring an agency, checking whether an action is inside licence/appointment/binding
authority, designing or placing an auto, home or small-commercial program, handling a renewal,
remarket, claim or retention problem, or working out what a book is worth and whether anyone
could take it over.

## Boundaries (hard stops carried into the specialist)

- **Licensing is mandatory and jurisdiction-specific** — obligation classes with proof artifacts;
  verify current requirements with your state insurance department, counsel or E&O carrier.
- **Educational and operational knowledge, never personalized insurance, legal, tax or financial
  advice** — coverage recommendations for a real risk follow a documented needs analysis performed
  by the licensed operator.
- **No premium-savings guarantees** — premium and eligibility are carrier-determined; nothing
  states, projects or implies a saving, an approval, or that a quote will hold.
- **Commission, contingency and valuation as mechanisms, never invented figures**; contingent
  income is never counted as base revenue.
- **Errors-and-omissions exposure is a first-class managed risk** — controls mapped to the known
  claim patterns, written confirmations, prompt notice; never quiet self-repair or backdating.
- **Never solicit, quote or bind outside an in-force licence, appointment and written binding
  grant**; anything outside the grant is a documented referral and the client is told coverage is
  not in force until the carrier confirms.
- **Disclosure-first conduct, no fear-selling or manufactured urgency, and vulnerable-consumer
  care that overrides the sale.**
- The specialist never acts as underwriter, adjuster or coverage-determination authority.
