# connectivity_agent — Business Internet & Wireless Agent

A FACTORY vertical for the carrier agent/partner connectivity business: selling business
internet, 5G/fixed-wireless backup, mobility, managed Wi-Fi and adjacent services through
carrier partner programs. Revenue = activation compensation + ongoing/account-based
compensation, strongest when one customer is bundled into several honest streams
(internet + backup + Wi-Fi + phones + cameras).

## The 3 knowledge bases

- `kb1_carrier_programs_and_bundle_strategy` — channel models (who bills/supports/owns),
  program entry and portfolio selection, honest discovery, layered bundle design,
  disclosure-complete proposals, serviceability-contingent handoff.
- `kb2_site_qualification_and_activation` — suite-level serviceability quals and on-site
  signal measurement, site survey and building constraints, failure-independent
  primary-plus-backup design, accurate ordering, physically proven failover, verified
  activation and handoff.
- `kb3_account_compensation_and_lifecycle` — mechanism-class compensation ledger,
  statement reconciliation and disputes, clawback containment, renewal radar, technology
  refresh, churn saves within authority, the escalation layer, and book-level stewardship.

## Build

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/connectivity_agent
```

Forges each `*.spec.json` into its `*.kb.json`, gates the KBs (`--mode dense`), then
gates `connectivity_agent.specialist.json`. Must end `ALL GREEN`.

## Use the specialist

Fill `dist/prompt_template.json`'s `specialist_prompt_template`:
`{{DOMAIN_LABEL}}` = the spec's `domain_label`, `{{SPECIALIST_SPEC_JSON}}` = the entire
`connectivity_agent.specialist.json` verbatim, `{{USER_QUESTION}}` = the question. Send to
any capable model; it operates by the spec's role, decision procedure, workflow and
escalation triggers.

## Boundaries (summary)

- Program terms, compensation plans and product availability change — verify the current
  program before any commitment; nothing rests on remembered terms.
- Compensation is explained as mechanisms (activation vs residual vs account-based, with
  clawback windows), never as invented figures; economics are mechanisms or qualitative bands.
- Serviceability is address-specific fact-finding (suite-level quals, on-site readings),
  never assumed from maps or neighbors.
- No misrepresentation of carrier affiliation — the agent sells the carrier's service under
  the program's rules and says so plainly.
- Regulatory/licensing points name the obligation class and flag "verify current rules in
  your jurisdiction"; life-safety paths go to licensed vendors.
- Ethical selling is binding: disclosure-first, no fear-selling, vulnerable-consumer care.
- Educational and operational knowledge only — never personalized financial, legal, tax
  or insurance advice.
