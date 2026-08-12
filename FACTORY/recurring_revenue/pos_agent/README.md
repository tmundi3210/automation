# pos_agent — Merchant Processing / POS Agent

**The business:** an independent agent acquires local merchants (restaurants, salons,
c-stores, auto repair), places or arranges POS and card acceptance as one accountable
bundle, and earns POS sale/lease revenue plus a share of processing economics. The
durable asset is the **book of merchants** paying monthly residuals — not hardware margin.

## The 3 KBs

- `kb1_merchant_acquisition_and_vertical_targeting` — ISO/agent program selection and
  agreement diligence, beachhead vertical targeting, the POS-anchored bundle, honest
  statement analysis, compliant claims, and disclosure-first closes through the
  deployment handoff.
- `kb2_pos_deployment_and_onboarding` — vertical-fit stack selection, honest
  application/underwriting/KYC shepherding, site survey, payment-network segmentation,
  menu build, legacy cutover, install, role-based training, and go-live hypercare
  through batch-to-deposit verified first settlement.
- `kb3_residual_book_and_portfolio_management` — residual waterfall mechanics and
  monthly reconciliation, merchant margin health and tiering, attrition early warning
  and ethical saves, disclosed repricing, support economics, chargeback education,
  risk-event shepherding, cross-sell from need, and book valuation / sell-hold-prune.

## Build (forge + gate everything)

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/pos_agent
```

Must end `ALL GREEN`. This forges each `*.spec.json` into its `*.kb.json`, gates the
KBs with `validators/kb_validator.py --mode dense`, then gates
`pos_agent.specialist.json` with `validators/specialist_validator.py`.

## Use the specialist

Fill the three slots of `dist/prompt_template.json`'s `specialist_prompt_template`:
`{{DOMAIN_LABEL}}` = `Merchant Processing / POS Agent Business`,
`{{SPECIALIST_SPEC_JSON}}` = the entire `pos_agent.specialist.json` verbatim,
`{{USER_QUESTION}}` = the question — then send to any capable model. The model
operates by the spec's role, decision procedure, workflow, and escalation triggers.

## Boundary summary

No guaranteed-savings or deceptive rate-comparison claims — statement analysis is a
diagnostic, savings are verified per merchant. Interchange, markups, residual splits,
and book multiples are described as mechanisms or bands, never invented figures stated
as fact. Card-network rules, PCI-DSS, and surcharging law are compliance classes to
flag ("verify current rules in your jurisdiction"), not specifics to assert. Education
only — never personalized financial/legal/tax advice; underwriting, holds, reserves,
and terminations belong to the processor. Disclosure-first selling with
vulnerable-owner care is a hard gate at acquisition, onboarding, and save time.
