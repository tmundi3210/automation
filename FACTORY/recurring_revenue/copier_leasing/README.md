# copier_leasing — Printer / Copier Leasing & Managed Print

One FACTORY vertical: 3 dense knowledge bases + 1 distilled specialist.

**Money model.** Place office print/copy equipment under an equipment lease plus a
cost-per-page service contract. Revenue comes from three streams with different owners:
equipment placement margin (realized once), the lease payment stream (usually assigned to a
third-party funder or captive finance arm, so it is generally not dealer revenue), and the
per-page service and supplies annuity on the installed base (the durable earnings).

## The 3 knowledge bases

| file | what it holds |
|---|---|
| `kb1_market_and_lease_sales.kb.json` | Office-equipment market & lease sales — which segments still generate pages, dealer/manufacturer program models, the lease-sale conversation under a fixed plain-language disclosure standard, displacement timing against the incumbent's notice window, honest total-cost framing and rollover transparency, credit path, signed-order handoff. |
| `kb2_fleet_assessment_deployment_and_service_setup.kb.json` | Print-fleet assessment, deployment & service setup — counter-based volume evidence over guesses, page/colour mix, workflow and destination mapping, right-sizing under a no-oversell guardrail, IT and obligation-class discovery, coverage/response/supply terms as mechanisms, delivery, removal with data sanitization, integration, training, baseline meter capture and the acceptance gate. |
| `kb3_lease_service_economics_and_renewal_cycles.kb.json` | Lease/service economics & renewal cycles — the installed-base annuity, per-serial contract register, meter-read integrity and reproducible billing, disputes and credits, escalators, realized per-page margin and consumable/service-cost control, uptime and churn signals, the notice-window and expiration calendar, four-option end-of-term analysis, re-rating from billed history, returns, data turnover and portfolio review. |

Each KB is authored as a compact `*.spec.json`, forged into `*.kb.json`, and gated dense
(nodes 19–24, edges 32–40, and the rest of the count/reference/formula bands).

## Build

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/copier_leasing
```

Forges each `*.spec.json` into its `*.kb.json`, gates all three KBs with
`validators/kb_validator.py --mode dense`, checks the specialist's `grounded_in_kbs`, then
gates `copier_leasing.specialist.json`. It must end **ALL GREEN**. Specialist gate alone:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/copier_leasing/copier_leasing.specialist.json \
  --report FACTORY/recurring_revenue/copier_leasing/copier_leasing.specialist.validation.json
```

## Using the specialist

Fill the three slots of `specialist_prompt_template` in `dist/prompt_template.json` and send
to any capable model — no training, no setup:

- `{{DOMAIN_LABEL}}` ← `Printer / Copier Leasing & Managed Print (grounded in 3 dense KBs)`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire contents of `copier_leasing.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

The model then operates strictly by the spec's `role`, `decision_procedure` and `workflow`,
honors every `escalation_triggers` entry and every `conflicts_and_dominance` rule, and answers
only inside this domain. Invoke it for anything about targeting, quoting, disclosing,
assessing, deploying, servicing, billing, renewing, returning or reviewing office print
equipment placed under a lease plus a per-page service agreement.

## Boundary summary

- **Disclosure is a rule, not a style.** Evergreen/automatic-continuation clauses with their
  notice window, delivery method and recipient, buyouts and purchase mechanisms, return
  conditions and freight exposure, the unconditional payment obligation and the lessor's
  identity are stated plainly in writing and walked through out loud before signature — at the
  same completeness on the smallest deal as on the largest.
- **Economics as mechanisms, never invented rates.** Lease rate factors, page rates,
  escalators, payoffs, buyouts, program funds, margins and market sizes are described by what
  produces them, what moves them and how to verify them against a current written quote or the
  funder's own written figure. No specific number is asserted as market fact.
- **No overselling capacity the client's volume cannot justify.** Every device class, speed
  step, finisher, colour capability, allowance and term traces to dated meter evidence or a
  mapped workflow requirement; anything beyond the documented peak is a separately shown,
  elected option.
- **No fear-selling.** No urgency, price-increase, obsolescence, end-of-support or
  compliance-risk claim without a document on file; compliance concerns route to the customer's
  compliance owner, never to a close.
- **Vulnerable-buyer care.** Buyers without procurement or advisory support get the term summary
  before the signing session, an invitation to independent review, recorded reading time,
  verified signing authority — and a smaller structure or a declined deal rather than pressure.
- **Regulatory content names obligation classes only,** with an explicit instruction to verify
  current rules in the applicable jurisdiction. No device feature or certificate is presented as
  legal compliance.
- **Educational and operational, never personalized financial, legal, tax, accounting or
  insurance advice.**
