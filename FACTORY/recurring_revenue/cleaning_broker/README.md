# Commercial Cleaning Broker (`cleaning_broker`)

A FACTORY vertical: 3 dense knowledge bases + 1 distilled specialist for operating a
commercial cleaning brokerage.

**Money model.** Win commercial cleaning contracts, arrange the service through sourced and
verified crews / subcontractors, and retain the monthly margin — the spread between the
client's recurring price and everything delivery actually costs. The revenue unit is one
recurring monthly contract; the product is a managed outcome (a building cleaned to a written
standard with a named person accountable when it is not), not resold hours.

## The three knowledge bases

| KB | Domain | What it covers |
|---|---|---|
| `kb1_contract_acquisition_and_bid_strategy` | Contract Acquisition & Bid Strategy | Facility classes and their buyers, opportunity screening and incumbent diagnosis, the walkthrough as a capture protocol, cleanable area and the task-and-frequency matrix, labor-load arithmetic, delivered cost, price build, sensitivity tests and the walk-away floor, proposal construction and the clause classes that protect margin. |
| `kb2_crew_sourcing_and_quality_control` | Crew Sourcing, Quality Control & Walkthroughs | Engagement structure and the classification question routed to professional verification, crew supply mapping, screening, reference and entity verification, coverage/bonding obligation classes, normalized crew quoting, the scope handoff pack, key and credential control, the inspectable standard, risk-weighted inspection cadence and scoring, the deficiency remedy ladder, backup coverage and crew transition. |
| `kb3_margin_retention_and_renewals` | Margin Management, Client Retention & Renewals | The per-contract monthly ledger and realized-spread read, crew cost drift and scope creep as named mechanisms, single-channel complaint intake with one cause class and the acknowledge-correct-verify-close loop, relationship mapping and review cadence, the renewal / notice / escalator calendar, change orders, repricing cases and conversations, account expansion, and the remedy-to-exit ladder. |

Each KB is hand-authored as a compact `*.spec.json`, forged into a dense `*.kb.json`, and gated
at dense bands by `validators/kb_validator.py`.

## Build

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/cleaning_broker
```

This forges each `*.spec.json` into its `*.kb.json`, gates every KB in `--mode dense`, checks
that the specialist's `grounded_in_kbs` point at those forged KBs, and gates
`cleaning_broker.specialist.json`. It must end with `ALL GREEN`. Gates check structure and math
only — never prose quality. To gate the specialist alone:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/cleaning_broker/cleaning_broker.specialist.json \
  --report FACTORY/recurring_revenue/cleaning_broker/cleaning_broker.specialist.validation.json
```

## Using the specialist

`cleaning_broker.specialist.json` is a machine-facing operating spec, not a trained model.
Drop it into the harness at `dist/prompt_template.json` and send the result to any capable
model. Its `specialist_prompt_template` has three slots:

- `{{DOMAIN_LABEL}}` ← the spec's `domain_label`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `cleaning_broker.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the end-user's question

The system message tells the model to operate strictly by the spec's `role`,
`decision_procedure` and `workflow`, honor every `escalation_trigger`, and answer only inside
the domain. Invoke it for anything on the contract arc: bidding, quoting, scoping, staffing,
inspecting, retaining, repricing, expanding or exiting a recurring commercial cleaning
contract — or designing the operating system that does those things.

## Boundary summary

- **Worker classification** (employee vs independent contractor) is a legal risk class that
  turns on facts about control and varies by jurisdiction. Name the factor classes, flag for
  professional verification in the jurisdiction of performance, never assume or assert a
  conclusion.
- **Honest broker positioning is absolute.** The client knows contracted crews perform the
  cleaning and that the broker sources, verifies, supervises, inspects, covers and stands
  behind them. No proposal, uniform, badge, invoice, coverage crew or silence may imply an
  in-house workforce.
- **Numbers are mechanisms, never invented figures.** Prices, rates, wages, spreads and
  account values are described as how the number is built from measured inputs, what moves it,
  and how to verify it for this deal — or as qualitative bands.
- **Insurance, bonding, licensing and permits are obligation classes** verified per contract,
  client, facility type and jurisdiction; verify current rules in your jurisdiction rather than
  reciting limits, forms or endorsements as settled.
- **Ethical selling and fair dealing are first-class rules**: no fear-selling (observed
  conditions stated once, factually, in writing), plain-language walkthrough of scope,
  exclusions, term and cancellation for unsophisticated buyers, and no use of a crew's
  dependence, inexperience or language gap as leverage.
- **Educational and operational knowledge only** — never personalized legal, tax, financial,
  insurance or employment advice.
