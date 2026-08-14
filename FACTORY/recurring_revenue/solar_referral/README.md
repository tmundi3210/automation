# Solar Sales / Referral Business — FACTORY vertical

**Business.** Sell residential solar as a commissioned representative under another party's agreement,
as an independent dealer contracting with licensed installers, or as a paid referral partner — qualify
homeowners honestly, build proposals whose every number traces to a captured artifact, disclose financing
and incentive classes before price, and carry the job through the installer's queue to permission to
operate.

**Money model.** A **commission per installed system** — the weakest recurring potential in this program:
there is no contracted recurring revenue, only transaction commissions plus a reputation-driven pipeline.
Every figure is a **mechanism**, never an invented number. Commission is the output of a formula in the
executed agreement: a mechanism class (per-watt spread over a redline, percentage of contract value, flat
per installed system, or a fee per introduced homeowner reaching a named milestone), times modifiers
(adders, tier or volume position, self-generated versus supplied lead, deductions), released against
milestones (contract or design, installation, permission to operate) with a holdback — and reversible by a
knowable list of clawback triggers with their recovery method and lookback. So the operator computes one
worked project from the agreement alone, reconciles the first statement to it line by line, holds
clawbackable advances as a separate non-spendable balance, and judges every lead channel on
**cancellation-adjusted cost per installed and paid deal** computed only from their own records. The one
asset that compounds is **reputation**: referred pipeline from verified-happy energized installs.

## The three knowledge bases

| KB | Scope |
|---|---|
| `kb1_market_models_and_lead_economics` | Representative vs dealer vs referral position and where cancellation, capital and licensing risk sits; seller-side obligation classes and consumer-protection/contact-consent rules; commission mechanism, milestone and clawback literacy; income timing and the operating reserve; the lead-channel portfolio, vendor diligence and consent provenance; the spend-to-installed-deal ratio chain and stage-wise funnel diagnosis; the ordered qualification gates and the scripted walk-away; pipeline record discipline, stage model and build-ahead math with cancellation modeled in. |
| `kb2_site_qualification_and_proposal_integrity` | The verified-numbers proposal charter and the per-household evidence ledger; consumption baseline from twelve months of the homeowner's own data with committed-vs-aspirational forward load; rate structure and export-compensation class read as mechanisms; roof planes, seasonal shade, roof condition and service-equipment screening; modeled production with a band and named loss assumptions; array sizing with its binding constraint; tariff-faithful bill comparison and escalator honesty; cash / loan / lease / power-purchase class literacy with disclosure-first walk-throughs, dealer-fee and step-payment mechanics, incentive framing without advice; permitting and interconnection sequence; vulnerable-consumer protocol, the ordered disclosure pack and the pre-signature stop-gate. |
| `kb3_installer_partnerships_and_reputation` | Reputation as the only compounding asset; the seller stays the homeowner's contact after signature while every technical answer routes to the licence holder; weighted installer criteria, first-hand build-quality evidence, backlog and cycle-time cohorts, continuity diligence and clause-by-clause agreement mapping; concentration, exit path and partner scorecard; the handoff packet and dated expectation log; milestone communication, the bad-news protocol and change-order re-disclosure; activation and first-bill closeout; cancellation cause taxonomy, cancellation ethics and chargeback administration; the verified-happy gate, referral and review mechanics, complaint handling and the reputation ledger. |

Each KB is forged from its `*.spec.json` and gated **dense** (count bands, reference integrity, acyclic
dependencies, formula consistency). `solar_referral.specialist.json` is the distilled operating layer over
all three — acquisition → delivery → recurring operations in one spec.

## Build (forge + gate everything)

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/solar_referral
```

Must end `ALL GREEN`. It forges each `*.spec.json` into `*.kb.json`, gates each KB with
`validators/kb_validator.py --mode dense`, checks `grounded_in_kbs` resolves to the KBs just forged, then
gates the specialist. Gate the specialist alone with:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/solar_referral/solar_referral.specialist.json \
  --report FACTORY/recurring_revenue/solar_referral/solar_referral.specialist.validation.json
```

## Use the specialist

Load `dist/prompt_template.json` and fill the three slots of `specialist_prompt_template`:

- `{{DOMAIN_LABEL}}` ← `Solar Sales / Referral Business (heavy: grounded in 3 dense KBs)`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `solar_referral.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

Send to any capable model. Its `role`, `decision_procedure`, `workflow` and `escalation_triggers` drive
the answer; the KBs are not loaded at answer time. For automatic selection among many specialists, use the
`router_prompt` in the same file with a routing table.

## Boundaries (hard stops carried into the specialist)

- **Anti-pressure, verified-numbers selling is the identity, not garnish.** This vertical's reputation is
  already damaged by high-pressure closing and invented savings claims. The pace belongs to the homeowner
  without exception; the only legitimate accelerator is better preparation. No expiring price, no
  manufactured deadline, no fear framing, no implied utility or government affiliation — and a
  demonstrated willingness to disqualify out loud is core practice.
- **Savings and production claims are mechanism-based, never invented and never guaranteed.** Every
  quantity traces to the homeowner's own twelve-month usage record, the published rate schedule and export
  rules with identifier and retrieval date, a site production model with its tool, dataset and loss
  assumptions named, the installer's quoted scope, or the lender's own disclosure. Production is a central
  estimate **with a band**; savings are reconstructed the way the utility computes a bill, keeping the
  fixed, minimum, non-bypassable and tax components that survive; any escalation assumption is sourced and
  shown **beside a zero-escalation case**.
- **Financing and consumer-protection rules are obligation classes, verified locally.** Home-solicitation
  and cooling-off notices, canvassing permits and do-not-knock regimes, contact-consent and registry-scrub
  classes, language-of-negotiation rules, seller-side registration, lease/PPA and loan disclosure, referral
  compensation and review disclosure are named as **classes** with **verify current rules in your
  jurisdiction** and record the check with its date — never asserted as a specific window, form, threshold
  or fee. An unverified obligation closes the territory or suspends the channel.
- **No fabricated figures.** Commissions, prices, redlines, rates, conversion ratios, market sizes and
  account multiples appear as mechanisms — how the number comes to exist, what moves it, and which document
  verifies it per deal — never as invented numbers or imported industry benchmarks. An unmeasured input is
  stated as unmeasured and decided under stated uncertainty.
- **Installer licensing belongs to the installer.** Engineering, code compliance, structural and electrical
  work, permit drawings, inspection remediation and workmanship warranty are the licence holder's; the
  seller states the possibility and the decision path with installer-sourced pricing and routes the question
  as a written ticket with a named owner and a callback it then chases.
- **Vulnerable-homeowner care is a procedural duty.** A recognition signal defers the signature to another
  visit with the reason recorded, invites a trusted person or adviser, supplies documents in the language of
  negotiation, and runs the teach-back — a failed restatement is a stop, never an objection to handle.
- **Compliance wins every conflict with growth**, and the tension is escalated rather than traded away.
  A cancellation instruction is executed, not handled; a post-signature price or scope change stops work
  until written re-disclosure and a fresh decision; the verified-happy gate precedes every referral ask.
- **Educational and operational only.** No personalized financial, legal, tax, medical or insurance advice;
  no credit data collected, stored or interpreted by the seller — prequalification is run by the homeowner
  through the lender's own path, and every eligibility or suitability question goes to their own adviser.
