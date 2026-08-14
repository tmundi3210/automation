# Water Filtration Systems — FACTORY vertical

**Business.** Sell and install residential water treatment — whole-home (point-of-entry) softening,
carbon and specialty media, and point-of-use reverse osmosis — for households on a municipal main or a
private well, then service that installed base forever.

**Money model.** Revenue is **equipment + installation** at the front and a **filter-replacement /
service-plan annuity** behind it. Every number in that sentence is a **mechanism**, never an invented
figure. The job price is a build-up the operator computes from their own dated documents: equipment at
the current distributor tier, freight and drop-ship charges, crew-hours for this configuration at a fully
loaded rate, permit and subcontract cost, and a warranty/callback reserve. The recurring side exists
because media is consumed — resin exchange capacity, carbon adsorption throughput, membrane fouling,
sediment loading, lamp hours — so each installed component carries an interval derived from what
physically exhausts it, anchored to the installed model's rated throughput and adjusted by named
household load variables. Plan margin is the per-serviced-system cost stack (consumable invoices,
freight and disposal, technician and drive time from timesheets, van and stock carrying cost, callback
reserve, billing overhead) against the tier price, and its main controllable input is **drive density**.
Attach, churn and book health are computed **from the operator's own book and ledger** — never quoted
from an outside benchmark.

## The three knowledge bases

| KB | Scope |
|---|---|
| `kb1_offer_design_and_honest_diagnostics` | Territory profiling by water supply and problem-class segmentation; intake, qualification and the honest disqualify-or-refer decision; the evidence ladder (utility annual water-quality report → premise survey → in-home screening panel with its stated limits → accredited-laboratory escalation with chain of custody); the claim perimeter, the demonstration ban and vulnerable-consumer choreography; standard-class interpretation without health claims; confirmed-problem→treatment-class mapping with anti-patterns; point-of-use vs point-of-entry scoping; disclosed tier architecture, quote build-up mechanics and the diagnostic-dossier gate. |
| `kb2_system_sizing_installation_and_compliance` | Pre-install site survey and feasibility verdict; converting dated chemistry and household demand into sizing loads (daily volume vs peak simultaneous flow); softener capacity/regeneration, carbon contact-time and point-of-use reverse-osmosis sizing; well vs municipal design adaptation; the flow-and-pressure budget as a hard gate; train sequencing and prefiltration; placement, bypass, isolation, drain/air-gap and discharge destination; cross-connection, thermal expansion, bonding continuity, permits and licensing as **obligation classes verified with the authority having jurisdiction**; in-house vs subcontracted licensed labor; joints, leak-risk control and change-order discipline; commissioning against the pre-install reading, homeowner orientation and the as-built record. |
| `kb3_filter_service_plans_and_retention` | The per-system service book; what actually exhausts each consumable and the per-class interval derivation; plan tiers vs a-la-carte, the agreement and disclosure page, and the cost-stack mechanics behind them; contact consent and record hygiene; plan attach at install handoff; the due-date engine, consented reminder/booking ladder, route batching and the standard service visit; retest cadence as documented proof of value; performance-drift triage across exhaustion, fault and source change; warranty vs plan-labor ownership; churn taxonomy, renewal and lapse recovery, property transfer, price-change governance, the referral loop and book-health metrics. |

Each KB is forged from its `*.spec.json` and gated **dense** (count bands, reference integrity, acyclic
dependencies, formula consistency). `water_filter.specialist.json` is the distilled operating layer over
all three — acquisition → delivery → recurring operations in one spec.

## Build (forge + gate everything)

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/water_filter
```

Must end `ALL GREEN`. It forges each `*.spec.json` into `*.kb.json`, gates each KB with
`validators/kb_validator.py --mode dense`, checks `grounded_in_kbs` resolves to the KBs just forged, then
gates the specialist. Gate the specialist alone with:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/water_filter/water_filter.specialist.json \
  --report FACTORY/recurring_revenue/water_filter/water_filter.specialist.validation.json
```

## Use the specialist

Load `dist/prompt_template.json` and fill the three slots of `specialist_prompt_template`:

- `{{DOMAIN_LABEL}}` ← `Water Filtration Systems - Honest Diagnostics, Compliant Installation & the Filter-Service Annuity (grounded in 3 dense KBs)`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `water_filter.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

Send to any capable model. Its `role`, `decision_procedure`, `workflow` and `escalation_triggers` drive
the answer; the KBs are not loaded at answer time. For automatic selection among many specialists, use the
`router_prompt` in the same file with a routing table.

## Boundaries (hard stops carried into the specialist)

- **No fear-selling, no health or medical claims.** Water-quality findings must come from **real, dated,
  shown tests at that address**. Claims are limited to what the treatment class verifiably does to named
  measured parameters and to what that specific model's **current third-party certification listing**
  carries. A serious finding (coliform-positive well, health-based exceedance) is delivered in the words
  the laboratory and the public-health authority use, health questions are referred to that authority or a
  physician, and no purchase deadline is attached to it.
- **Honest chemistry, never theater.** A demonstration is permitted only if it is explained before it runs,
  carries a control and is reproducible by the homeowner. Precipitator or dye tricks and any demonstration
  staged to imply contamination are banned outright — including as an answer to a competitor doing them.
- **Codes and licensing are obligation classes, verified locally.** Permits and who may pull them, adopted
  plumbing code and local amendments, brine/reject discharge to sewer or septic, air-gap and
  cross-connection protection, closed-system thermal expansion, metallic-pipe bonding continuity,
  contractor/specialty licensing, consumer cancellation-rights and outreach-consent rules are named as
  **classes** with **verify current rules in your jurisdiction** — never asserted as specific rules,
  thresholds or fees. Unverified obligations block the equipment order, not the conversation.
- **No fabricated figures.** Equipment margins, quote build-ups, plan prices, attach and churn rates,
  market sizes and account multiples appear as mechanisms — which inputs exist, what moves them, and which
  **dated document** verifies each one for this deal — never as invented numbers or industry averages.
- **Evidence dominates enthusiasm.** A dated sample from this tap beats the utility report and the
  territory profile; an accredited-laboratory result beats any disagreeing field screening; minimal
  sufficient treatment beats job margin, and anything beyond the evidence is a plainly declinable line.
- **Vulnerable households get extra care, not extra pressure.** Disclosure-first entry, permission to test
  and photograph, no manufactured urgency, no same-visit close under a vulnerability signal, a trusted
  second person invited, and the applicable cancellation-rights notice handed over.
- **Compliance wins every conflict with growth**, and that tension is escalated rather than traded away:
  build to the verified requirement or decline the work, and write the decision and its basis into the job
  record. Leak prevention beats the schedule; written approval beats any verbal promise; consent state
  beats the reminder machine.
- **Refer out rather than sell** when the defect is the well, pump, pressure tank, water heater or premise
  plumbing rather than treatable water chemistry, and hand any work leaving the service envelope back to
  the compliance-verified installation process.
- **Educational and operational only.** No personalized financial, legal, tax, insurance or medical advice,
  and no interpretation of a laboratory result beyond chemistry against a published standard class.
