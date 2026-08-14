# Pest Control Sales / Referral Business — FACTORY vertical

**Business.** Sell recurring pest-control service accounts — either as an **own licensed operation**
or as a **commission / referral partner of a licensed operator** — and turn those signatures into a
route-dense, ethically-retained account book.

**Money model.** Revenue is **new-account commissions plus recurring contract value**. Every number in
that sentence is treated as a **mechanism**, never an invented figure: the initial or set-up fee, the
recurring fee per visit or per month at the sold frequency, add-on scope, and referral or override
compensation each carry a basis, a timing, named drivers and the **clause, statement or account record**
that proves the real figure for that specific deal. The governing economics are **route density**: the
recurring price is roughly flat across geography while the time cost of reaching a customer is not, so
every prospective account is judged both on revenue added and on its effect on travel minutes per stop
in its zone. The governing constraint is **licensing**: pesticide application is credentialed work, and
no account is written in a service class or jurisdiction whose credential row is unverified.

## The three knowledge bases

| KB | Scope |
|---|---|
| `kb1_account_sales_and_route_density` | Route density as the economics of the business (drive time as the hidden cost mechanism), territory geometry and route-day capacity, pest-pressure reading and neighborhood targeting, anchor-and-fill seeding, the out-of-cluster dilution test, ethical door / referral / partner / inbound channels under home-solicitation regulation, own-operation vs sell-for-operator models, commission and clawback mechanics, seasonal demand waves, and pacing sales to serviceable capacity. |
| `kb2_service_delivery_and_licensing_awareness` | The credential classes (firm operator licence, individual applicator certification, seller registration where it exists, wood-destroying-organism inspection) and who must hold each; the sell-versus-apply line and the not-doing list; primary-regulator verification loop; partnering structures, operator diligence, account-ownership and commission terms; pest / inspection / treatment-class and IPM literacy sufficient to sell honestly; label law, sensitive-site and occupant safety; pre-signature disclosure, vulnerable-customer care, the promise ledger, technician handoff, first-service verification, callbacks and the incident escalation path. |
| `kb3_recurring_contracts_and_seasonal_upsells` | Service-agreement anatomy, frequency classes, disclosure standard, automatic-renewal and notice obligation classes, billing and payment-failure handling; the front-loaded payback shape and the early-term risk window; retention through visible-result evidence, contact cadence and low-friction scheduling; cohort survival measurement and a cancellation-reason taxonomy; the ethical save (a clear cancellation request is always honored) and the remedy ladder; seasonal add-on classes gated behind documented need and tested ride-along vs separate-trip; price-adjustment waves; book record hygiene, density-weighted book value and periodic triage / release. |

Each KB is forged from its `*.spec.json` and gated **dense** (count bands, reference integrity, acyclic
dependencies, formula consistency). `pest_control.specialist.json` is the distilled operating layer over
all three.

## Build (forge + gate everything)

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/pest_control
```

Must end `ALL GREEN`. It forges each `*.spec.json` into `*.kb.json`, gates each KB with
`validators/kb_validator.py --mode dense`, checks `grounded_in_kbs` resolves to the KBs just forged, then
gates the specialist. Gate the specialist alone with:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/pest_control/pest_control.specialist.json \
  --report FACTORY/recurring_revenue/pest_control/pest_control.specialist.validation.json
```

## Use the specialist

Load `dist/prompt_template.json` and fill the three slots of `specialist_prompt_template`:

- `{{DOMAIN_LABEL}}` ← `Pest Control Sales & Referral Business — Route-Dense Account Acquisition, Licensed Service Delivery, Recurring Book`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `pest_control.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

Send to any capable model. Its `role`, `decision_procedure`, `workflow` and `escalation_triggers` drive
the answer; the KBs are not loaded at answer time. For automatic selection among many specialists, use the
`router_prompt` in the same file with a routing table.

## Boundaries (hard stops carried into the specialist)

- **Licensed application is non-negotiable and jurisdiction-specific.** Unlicensed application is out of
  scope. The specialist never applies, directs, specifies, prescribes a product or rate, advises
  self-application, or makes a definitive pest or structural determination — those belong to the licensed
  operator, certified applicator or credentialed inspector. Licensing content names the **obligation class**
  and instructs the reader to **verify current rules with the primary regulator in their own jurisdiction**.
- **Regulated selling at the door.** Permits, permitted hours, do-not-knock registries, posted signage,
  private-community rules and cooling-off / cancellation-notice duties are cleared **per municipality**,
  dated, before a canvass plan is worked; listed and posted addresses come out of the plan.
- **No scare-selling.** Findings are shown, never invented. Conducive conditions, observed activity and
  diagnostic determinations stay in separate tiers, uncertainty is stated aloud, and where nothing is
  present prevention is sold on its own merits or nothing is sold. No eradication or pest-free promise —
  recurring service is pressure managed to a threshold with monitoring and a defined re-service entitlement.
- **No fabricated figures.** Commissions, prices, initial fees, payback points, retention rates, market
  sizes and account multiples appear as mechanisms with drivers and the proving document — never an
  invented number, and never a multiple asserted as market fact.
- **Disclosure first, consent protected.** Role and material terms are disclosed before any signature or
  payment authorization; authority to bind the property is established; and where capacity, comprehension,
  language, isolation or fear puts informed consent in doubt the sale stops, written material and
  identification are left, a nominated contact or interpreter is invited, and any cancellation is processed
  with **no save attempt**.
- **Capacity governs pace.** Sold-not-yet-serviced work is a **capped backlog** per zone measured against
  remaining route-day slots and the operator's dated capacity statement — never a pipeline to celebrate.
- **Compliance wins every conflict with growth**, and compliance-vs-growth tension is escalated rather than
  traded away. Exposure, illness and non-target-harm reports go immediately to the operator, to medical
  channels and, where applicable, to the regulator; no customer is ever discouraged from contacting one.
- **Educational and operational only.** No personalized legal, financial, tax, insurance or medical advice;
  agreement interpretation is counsel's work.
- **No demographic-proxy targeting.** Neighborhood ranking rests only on pest, property, access and route facts.
