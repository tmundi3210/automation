# Medical Alert Systems — FACTORY vertical

**Business.** A local medical-alert operation: sell and set up personal emergency response equipment —
a worn or fixed help button that opens a path to a contracted monitoring centre — for older adults and
their families, then run the monitoring service behind it for years.

**Money model.** Revenue is the device and activation transaction plus the recurring monthly monitoring
subscription. Every figure is treated as a **mechanism**, never an asserted number: equipment at the
operator's own purchase cost, the installation visit at their own loaded cost, the acquisition spend
attributed to the channel, the billed subscription less the wholesale monitoring rate, processing and
servicing, how long the account persists, and what survives its ending. Payback is a *derived duration*
and channels are judged on **cost per retained account**, both instantiated from the operator's own
invoices, monitoring contract, processing statements and spend records — no industry commission, price,
response time, detection rate, market size or account multiple is ever stated as fact. The durable asset
is a book of live, well-fitted accounts that still signal and still want the service.

## The three knowledge bases

| KB | Scope |
|---|---|
| `kb1_market_and_ethical_senior_sales` | The buyer triangle of wearer, adult-child payer and professional referrer; referral channels (home care, senior centres, discharge planners) built inside the inducement boundary; transparent offer design with all-in first-year cost disclosed before any ask; vulnerable-consumer and no-fear-selling rules as hard stops; honest competitive positioning and the decline-to-sell list. |
| `kb2_device_setup_monitoring_and_testing` | Home survey and measured connectivity, range and outage findings; device-class selection and wearer fit with fall detection framed honestly; monitoring-account build, verified dispatch address, contact tree and responder entry; the mandatory end-to-end signal test, teach-back training, documentation and the binary go-live gate (guided remote installs included, with no test waivers). |
| `kb3_monitoring_retention_and_caregiver_relations` | Running the live subscription: test cadence inside the platform supervision window, device-health and consumable servicing, address and contact currency, per-account health triage; caregiver communication inside recorded disclosure permissions; activation aftercare, missed-signal reconstruction and honest service recovery; nuisance-activation ladder; fleet migration on transport sunset; dignified endings, provable exit symmetry, book health and earned referrals. |

Each KB is forged from its `*.spec.json` and gated dense (node/edge/axis/workflow/CQ count bands,
reference integrity, formula consistency). `medical_alert.specialist.json` is the distilled operating
layer over all three.

## Build (forge + gate everything)

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/medical_alert
```

Must end `ALL GREEN`. It forges each `*.spec.json` into `*.kb.json`, gates each KB with
`validators/kb_validator.py --mode dense`, checks `grounded_in_kbs`, then gates the specialist with
`validators/specialist_validator.py`. Gate the specialist alone with:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/medical_alert/medical_alert.specialist.json \
  --report FACTORY/recurring_revenue/medical_alert/medical_alert.specialist.validation.json
```

## Use the specialist

Load `dist/prompt_template.json` and fill `specialist_prompt_template`:

- `{{DOMAIN_LABEL}}` ← `Medical Alert Systems: Ethical Senior Acquisition, Verified Installation & Monitoring Book Operations`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `medical_alert.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

Send to any capable model. Its `role`, `decision_procedure`, `workflow` and `escalation_triggers` drive
the answer; the KBs are not loaded at answer time. For automatic selection among many specialists, use
the `router_prompt` in the same file with a routing table.

## Boundaries (hard stops carried into the specialist)

- **Equipment and monitoring, not medicine.** This is emergency-signaling equipment plus a monitoring
  service — not medical advice, not a medical device consultation, not clinical assessment, not
  supervision and not care. Clinical, capacity, medication, driving, living-alone, legal, tax and
  benefit-eligibility questions are refused with the charter's refusal sentence and routed to a named
  neutral destination; account fields carry the household's own words.
- **Vulnerable-consumer rules are first-class.** The all-in first-year cost and the cancellation terms are
  disclosed in writing before any ask; no same-visit pressure close on an older adult who is alone and has
  consulted nobody; no agreement from a person whose comprehension cannot be demonstrated by teach-back;
  no manufactured deadline and no unfalsifiable personal prediction. The wearer's own assent is separate
  from the payer's decision and its absence is a **decline**, not an objection to overcome; the wearer
  stays the customer even when a relative pays.
- **Reliability is device-verified, never asserted.** Battery, range, fall-detection behaviour and answer
  performance require a named source with its measurement condition, the monitoring contract, or a test in
  this building. Fall detection is a probabilistic **supplement** to the button — missed events and false
  alarms stated, no detection rate, no guarantee, no prevention language. The **end-to-end signal test is
  mandatory** at setup, a partial pass is a failure, remote installs get no waivers, and an account that
  has not signalled does not go live.
- **The exit is part of the product.** Cancelling is no harder than joining on steps, channels and elapsed
  time, proven by a symmetry test; billing stops on bereavement notification without documents with no save
  attempt; no remedy is ever conditioned on continued subscription or on withholding a complaint.
- **Economics are mechanisms.** Prices, rates, payback, tenure and book contribution name the internal
  document each term traces to; no external benchmark is presented as this operator's number.
- **Regulation is named by class and verified locally.** Referral inducement and anti-kickback exposure,
  solicitation and cooling-off duties, automatic-renewal and cancellation-disclosure duties, alarm-monitoring
  or contractor licensing and permits, health-information privacy and consent, and consumer-protection rules
  for older adults — the obligation class is stated and the specifics must be **verified against current
  rules in your jurisdiction** with the applicable authority or the operator's own counsel. Educational and
  operational knowledge only; never personalized medical, financial, legal, tax or insurance advice.
