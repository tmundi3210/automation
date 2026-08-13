# Subscription Bookkeeping Practice — FACTORY vertical

**Business.** A bookkeeping practice that sells small businesses a **fixed monthly subscription** for a
defined body of recurring bookkeeping and management-reporting work, instead of selling hours. The arc is
one system: productized offer and niche → onboarding, cleanup and the repeatable monthly close → the
recurring economics of the book already sold.

**Money model.** Revenue = the sum of the **fixed monthly fees of the live accounts**, one fee per client
tier, plus separately scoped, separately priced entry work (the catch-up/cleanup project) and add-ons.
Every quantity is a **mechanism, never a recalled figure**: a rung fee is derived from an effort estimate
for that rung's workload signature, the practice's own measured cost floor, an uncertainty premium sized by
how much of the signature is still hypothesis, and value context — then verified per deal against measured
delivery effort in the first months. Capacity, contribution and price debt come from the practice's own
recurring effort ledger with its dated measurement window. No competitor price, survey band, article
benchmark or practice multiple is ever asserted as fact.

## The three knowledge bases

| KB | Scope |
|---|---|
| `kb1_productized_offer_and_niche_selection` | What the practice sells and to whom: the offer charter and its three refusals (no hourly billing, no unbounded scope, no licensed-professional work); the bookkeeping-versus-licensed boundary and client-data access ethics as first-class rules; niche candidates compared on observed workload signatures, why niching compounds, and the commitment with its concentration cap; countable scope units, the tier ladder, exclusions and rung-move triggers; the fixed-fee mechanism from measured cost-to-serve; honest positioning against hourly bookkeeping and DIY software; the six-test fit screen, books-condition triage, mandatory declines with referrals, disclosure-first selling with vulnerable-owner care, and the written engagement artifact gate. |
| `kb2_onboarding_cleanup_and_monthly_close` | What happens inside the client file: least-privilege access provisioning and the confidentiality operating rule; the bounded diagnostic review and its findings register; the cleanup scoped, priced and accepted as a separate entry gate; the opening balance anchor; chart-of-accounts standard, categorization rulebook, document intake, feed health, open-item queue and per-client runbook; the monthly close backbone — statement-based reconciliation, categorization review pass, balance-sheet support and residue scan, packet build, named sign-off, delivery and period lock; three-way judgment-call routing, correction-with-reissue, and the year-end CPA handoff. |
| `kb3_subscription_pricing_capacity_and_retention` | The book after the sale: the recurring effort ledger and the derived clients-per-seat capacity band that gates admission; the contribution view and legacy price debt; scope-drift sensing with four destinations and never a fifth (silent absorption); the announced repricing cadence and the disclosure-first reprice conversation; report usefulness and the monthly review call; at-risk signals and the intervention ladder; the distressed-client protocol and seasonal terms; mis-fit exit criteria with the offboarding pack and dated revocation; CPA alliance and niche-community referral inside confidentiality; and the periodic book health review. |

Each KB is forged from its `*.spec.json` and gated **dense** (node/edge/axis/workflow/CQ count bands,
reference integrity, acyclic dependencies, formula consistency). `bookkeeping_sub.specialist.json` is the
distilled operating layer over all three.

## Build (forge + gate everything)

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/bookkeeping_sub
```

Must end `ALL GREEN`. It forges each `*.spec.json` into `*.kb.json`, gates each KB with
`validators/kb_validator.py --mode dense`, checks `grounded_in_kbs` resolves to the KBs just forged, then
gates the specialist. Gate the specialist alone with:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/bookkeeping_sub/bookkeeping_sub.specialist.json \
  --report FACTORY/recurring_revenue/bookkeeping_sub/bookkeeping_sub.specialist.validation.json
```

## Use the specialist

Load `dist/prompt_template.json` and fill the three slots of `specialist_prompt_template`:

- `{{DOMAIN_LABEL}}` ← `Subscription Bookkeeping Practice: Productized Offer & Niche, Onboarding, Cleanup & Monthly Close, Pricing, Capacity & Retention`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `bookkeeping_sub.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

Send to any capable model. Its `role`, `decision_procedure`, `workflow` and `escalation_triggers` drive the
answer; the KBs are not loaded at answer time. For automatic selection among many specialists, use the
`router_prompt` in the same file with a routing table.

## Boundaries (hard stops carried into the specialist)

- **Bookkeeping, not attest and not advice.** Recording, classifying, reconciling, maintaining the ledger
  and producing management reports are inside. Audits, reviews, compilations and anything issued with a
  practitioner's report are outside — as are return preparation, filing positions, notice responses,
  entity structure, worker classification and legal questions. Know where **licensed-CPA territory begins
  and flag it**: state what the records show, name the question, route it to the client's own licensed
  professional with the facts and no recommendation, and log the exchange.
- **Verify current rules in your jurisdiction.** Licensing, protected titles, registration, records
  retention and a client's entitlement to their records appear only as **obligation classes** with an
  instruction to verify locally with a qualified professional — never as recited rules.
- **Fixed-fee tiers are pricing mechanisms; no invented market rates.** Fees, tier bands, capacity counts,
  contribution and price debt are derived and dated, or they are not stated. An external figure is an
  unverified claim: re-derive, and decline rather than sell below the measured floor.
- **Client financial data confidentiality and access discipline are first-class.** Least-privilege,
  role-based access through the client's own user management to a named person for a named purpose; no
  owner credential ever held (anything sent unprompted is destroyed and recorded); a verified connection
  and an access register entry with a revocation path before work begins; periodic access review; dated
  revocation and a retention decision at exit; every outbound disclosure traced to a written client
  instruction naming recipient and material.
- **Educational and operational knowledge only** — for a practice operator, never personalized financial,
  legal, tax, accounting or insurance advice, and never a determination the client or their licensed
  professional owns.
- **Disclosure-first, no fear-selling, vulnerable-owner care.** Fee, inclusions, exclusions, client
  responsibilities and the separateness of any catch-up are stated before commitment is requested; every
  fee, scope or termination change goes out in writing with its effective date and notice period *before*
  it is discussed, with a genuine smaller-scope alternative. No urgency framing, no speculation about what
  an authority might do, no consequence claim the ledger cannot evidence. Where distress or
  non-comprehension appears: slow down, options in writing, no same-call commitment, no upsell, refer
  insolvency and tax-deferral questions out, and protect the practice with a stated limit on continued
  unpaid delivery rather than with pressure.
- **Nothing unsupported and undisclosed.** Reconciliations agree to third-party statements and settlement
  reports, never to a feed balance; no forced balancing entries; every balance-sheet line has named support
  or a written explanation; an unexplained difference beats the delivery date; a locked period changes only
  with recorded authorization and a reissue note.
- **No silent scope absorption, no bespoke rung, no records held hostage.** Out-of-scope work goes to a
  priced add-on, a rung move, a dated logged goodwill instance, or a decline with a referral. A rung
  invented for one prospect is labelled a custom engagement. A departing client gets the offboarding pack
  per the engagement terms; the fee is pursued through the terms, never through leverage over the books.
- **Proof and referrals leak nothing.** Sales, marketing and referral material is de-identified or
  synthetic, or covered by specific written client permission; introductions connect people rather than
  forward financial detail.
