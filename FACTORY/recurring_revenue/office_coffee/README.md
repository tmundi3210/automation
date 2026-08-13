# office_coffee — Office Coffee & Water Service

One FACTORY vertical: 3 dense knowledge bases + 1 distilled specialist.

**Money model.** Place coffee brewers and water systems (bottled coolers or plumbed
point-of-use) into offices, keeping title to the equipment, and earn recurring revenue from
consumables — coffee in the placed machine's format, tea, creamer, cups, filters, water — plus
the service route that delivers them. Three streams with different behaviour: equipment
placement (capital on loan, recovered inside consumables margin over the account's life, or
rented/sold outright as a disclosed charge), the consumables annuity (the durable earnings),
and the route itself (the cost engine — drive minutes are pure cost no client pays for).
Every figure in this vertical is a **mechanism** the operator computes from their own delivery
tickets, supplier invoices, minute ledger and payroll — never a quoted market number.

## The 3 knowledge bases

| file | what it holds |
|---|---|
| `kb1_ocs_market_and_account_acquisition.kb.json` | OCS market targeting & account acquisition — drivable cluster territory and office segmentation by headcount, attendance pattern and culture; the breakroom discovery visit; the consumption band as an observed-factor chain; incumbent supply diagnosis and the sourced displacement case against self-serve retail buying; water feasibility screen; the qualification go/no-go on the band's low case; tasting/free-trial economics and written trial terms; place-vs-rent-vs-sell; disclosure-first, honestly sized proposals; agreement term-class literacy; the signed-account handoff packet. |
| `kb2_equipment_placement_and_route_logistics.kb.json` | Equipment placement, route setup & consumables logistics — the placement done-state; sanitation, potable-water, backflow, plumbing and trade obligation classes verified locally; breakroom survey and source-water testing; brewer and water-system class matching; utility readiness, the licensed-trade boundary and install/handover; catalog design, honest par from measured draw-down, cadence derivation; territory zoning, sequencing around hard access windows, van load-out and depot replenishment; preventive maintenance, reliability and swap, serialized asset register, the standard stop and its record. |
| `kb3_route_economics_and_account_expansion.kb.json` | Route economics, retention & expansion — the stop as the unit of account; park-to-park minute ledger and counted draw (with shrink separated); the stop-contribution chain; the volume-vs-time fork for thin stops and the remedies each branch licenses; disclosed price architecture, cost passthrough and the honest reprice play; delivery minimums and concession discipline; run-out incident economics, capacity-honest service promises, recovery, champion continuity and churn signals; the honest expansion gate, category ladder, micro-market class upgrade and shifting obligation classes; route density, the marginal-stop test, book value and concentration risk. |

Each KB is authored as a compact `*.spec.json`, forged into `*.kb.json`, and gated dense
(nodes 19–24, edges 32–40, and the rest of the count / reference / formula bands).

## Build

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/office_coffee
```

Forges each `*.spec.json` into its `*.kb.json`, gates all three KBs with
`validators/kb_validator.py --mode dense`, checks the specialist's `grounded_in_kbs`, then
gates `office_coffee.specialist.json`. It must end **ALL GREEN**. Specialist gate alone:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/office_coffee/office_coffee.specialist.json \
  --report FACTORY/recurring_revenue/office_coffee/office_coffee.specialist.validation.json
```

## Using the specialist

Fill the three slots of `specialist_prompt_template` in `dist/prompt_template.json` and send to
any capable model — no training, no setup:

- `{{DOMAIN_LABEL}}` ← `Office Coffee & Water Service - Account Acquisition, Placement & Route Economics (grounded in 3 dense KBs)`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire contents of `office_coffee.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

The model then operates strictly by the spec's `role`, `decision_procedure` and `workflow`,
honors every `escalation_triggers` entry and every `conflicts_and_dominance` rule, and answers
only inside this domain. Invoke it for anything about targeting, qualifying, proposing,
trialling, installing, stocking, routing, servicing, measuring, repricing, retaining, expanding
or valuing office coffee, water and breakroom-supply accounts.

## Boundary summary

- **Route economics as mechanisms, never invented figures.** Stop contribution, markup, cost to
  serve, trial cost, delivery minimums, conversion rates, density gains and book value are
  stated as chains run on the operator's own records — with the inputs, the source record, what
  moves the number and how to verify it for this deal. No industry rule of thumb is asserted as
  fact.
- **Food-handling and sanitation requirement classes verified locally.** Licensing, food-handler
  training, health-department registration, potable-water and backflow protection, plumbing and
  electrical trade licensing, transport, weights-and-measures and unattended-retail obligations
  are named as **classes** with the authority to ask and a "verify current rules in your
  jurisdiction" flag — the placement or the launch is held, never the requirement.
- **Honest consumption-based proposals — no overstocking clients into resentment.** Opening
  orders and minimum commitments sit at or below the low case of the evidence-backed band; par
  comes from measured draw-down capped by the client's storage and the shortest freshness
  window; every increase or new item needs a dated client agreement; reduction requests are
  actioned in the next cycle.
- **Disclosure-first selling, no fear-selling.** Identity and purpose at first contact; all-in
  recurring cost, equipment obligation and how it is recovered, cadence commitment and exit
  terms stated before any signature; no health, safety or water-quality claim without a
  verifiable published source; objections answered with the client's own numbers, never with
  claims about a competitor.
- **Vulnerable-buyer care.** The office manager or admin who signs is usually spending an
  employer's money under pressure without procurement support — an honest downgrade or a decline
  recommendation is a required output whenever their own numbers do not support the programme.
- **Promise only what the route can hold** on its worst ordinary week, inside a drivable cluster
  (or as a recorded density exception).
- **Educational and operational, never personalized legal, tax, accounting, financial or
  insurance advice** — term *classes* are explained in plain language, the client is told in
  writing to have their own advisor review the document, and no breach of an incumbent
  agreement is ever advised or assisted.
