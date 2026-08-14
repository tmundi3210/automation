# home_warranty — Home Warranty / Service Plan Sales

FACTORY vertical for the recurring-revenue specialist program.

**Business / money model:** sell home service contracts (home warranty / service plans) as an
appointed representative of one or more obligors; revenue = compensation created by the
obligor's compensation exhibit when a contract is written, plus renewal-side compensation on
contracts that stay in force. The durable asset is a persistent in-force book that renews
because expectations were set correctly at sale — not because terms were glossed. Every
figure in this vertical (contract price, service-fee levels, caps, chargebacks, commissions,
persistency, book value) is handled as a **mechanism to verify per deal**, never as an
asserted number.

## The 3 knowledge bases

| KB | File | Covers |
|---|---|---|
| KB1 | `kb1_products_and_distribution_channels.kb.json` | What the product actually is; obligor / administrator / backing party map; the jurisdictional regulatory-class screen and the seller authorization stack; tier and option architecture, cap and service-fee mechanics, price build and the compensation mechanism; the four channels (real-estate closing, direct-to-homeowner, property-manager portfolio, trade-partner origination) with their referral-compliance class, lead provenance and consent, disclosure-first offer design, vulnerable-buyer safeguards, the per-channel obligation matrix and the order packet. |
| KB2 | `kb2_coverage_terms_and_honest_disclosure.kb.json` | Reading the governing contract edition in a fixed order; the component-level coverage map; exclusion families as claim-time evidence mechanisms; caps and limits against locally sourced replacement cost; waiting period and effective date; the pre-existing-condition clause as an evidence test; the home risk profile, the three-question tier test and the decline-to-sell path; plain-language translation that never widens the promise, the three-part coverage answer, claim-experience preview, condition intake, and the dated disclosure evidence file. |
| KB3 | `kb3_renewals_claims_experience_and_reputation.kb.json` | The post-sale role charter and in-force ledger; expectation records carried forward; per-obligor claim path and stall points; first-call triage, advocacy limits and status cadence; denial decoding, the response playbook and cash-in-lieu / capped outcomes; the renewal calendar, the four-input value test, rate-change disclosure, honest retention levers, cancellation ethics and the do-not-renew recommendation; vulnerable-client post-sale care; reputation-risk attribution, public-complaint handling, referral-partner loops, obligor performance review and book health. |

The specialist is `home_warranty.specialist.json`, distilled across all three and gated by
`validators/specialist_validator.py` (report: `home_warranty.specialist.validation.json`).

## Build / gate

```bash
cd /home/user/automation
bash FACTORY/build.sh FACTORY/recurring_revenue/home_warranty   # must end "ALL GREEN"
```

This forges each `*.spec.json` into its `*.kb.json`, gates every KB in dense mode, checks the
specialist's grounding, and gates the specialist.

## Using the specialist

Fill the three slots of `dist/prompt_template.json` (`specialist_prompt_template`):

- `{{DOMAIN_LABEL}}` <- "Home Warranty / Service Plan Sales: Service-Contract Products &
  Distribution, Coverage Terms & Honest Disclosure, Renewals, Claims Experience & Reputation"
- `{{SPECIALIST_SPEC_JSON}}` <- the entire `home_warranty.specialist.json`, verbatim
- `{{USER_QUESTION}}` <- the end-user's question

Send the assembled prompt to any capable model; it operates by the spec's `role`,
`decision_procedure`, `workflow`, and `escalation_triggers`. Load a KB alongside it only when
you need the full node/edge/workflow graph for one phase.

## Boundary summary

- **State-regulated product** — registration, obligor-qualification, backing, disclosure,
  delivery, free-look, refund and auto-renewal-notice obligations vary by jurisdiction and
  attach to different parties. The specialist names the obligation *class* and the party it
  attaches to, then instructs "verify current rules in your jurisdiction" with the regulator,
  the obligor's compliance function and counsel — it never asserts a specific rule.
- **Disclosure before money, always** — exclusions, caps, sub-limits, the waiting period, the
  per-visit service fee and who decides claims are surfaced before signature in *every*
  channel, including a compressed closing. Burying them is this vertical's core ethical
  failure; an unwritten order beats an unacknowledged one.
- **A defined contract, not a promise** — claim outcomes belong to the obligor. The seller
  never adjudicates, predicts or implies a determination, turnaround or service level the
  contract does not state; "is it covered" is answered in the fixed three-part form ending in
  who decides.
- **Mechanisms, not invented figures** — commissions, prices, caps, allowances, chargebacks,
  denial rates, persistency and book value are explained by how they come to exist, what moves
  them, and which artifact verifies them for this deal (contract cap table, current price
  schedule, compensation exhibit, obligor statement, client-obtained local quote).
- **No fear-selling** — risk is stated only as observable facts with sources; predicted
  failures, invented statistics, unsourced repair costs and manufactured urgency are banned by
  name in every script and template, and templates are audited rather than trusted.
- **Vulnerable-buyer and vulnerable-client care is first-class** — slowed pace, written
  summary, cool-down, client-chosen third party, comprehension confirmed by restatement, no
  auto-renew carrying a flagged account, no upgrade during an open claim, and no sale where
  informed consent is absent.
- **The not-a-sale outcomes are first-class** — the decline-to-sell recommendation and the
  do-not-renew recommendation have their own documented paths; a downgrade never substitutes
  for a met decline trigger, and the rate of do-not-renew advice is reported, not hidden.
- Educational and operational knowledge only — never personalized financial, legal, tax,
  medical or insurance advice.
