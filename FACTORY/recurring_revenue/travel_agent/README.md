# travel_agent — Travel Agent / Advisory Practice

A FACTORY vertical: 3 dense knowledge bases + 1 distilled specialist for operating a leisure
travel advisory practice.

## The business and the money model

A one-to-few-person travel advisory practice that books travel either under a **host agency**
or on its **own accreditation**. Revenue arises from four mechanisms, never from a fixed rate:

- **supplier commission** on booked travel — payable on that supplier's own *commissionable base*
  and released by that supplier's own *payment trigger* (commonly after travel is completed,
  sometimes at final payment, occasionally at deposit);
- **client-paid planning and service fees** — the instrument that pays for advice whether or not
  the client books;
- **commission on travel-protection and other add-ons** — only inside a verified licensing and
  appointment requirement class;
- **supplier / consortium incentives** — always disclosed context, never a neutral endorsement.

Recurrence in this vertical is not a subscription. It comes from **repeat households and
referrals**: the post-trip debrief, the anniversary and planning-window cadence, and a client
book whose value rests on documentation depth, demonstrated repeat behaviour, concentration,
and what the affiliation and consortium clauses actually permit to transfer.

## The three knowledge bases

| KB | Covers |
|---|---|
| `kb1_agency_models_hosts_and_niches` | **Acquisition & offer economics** — host affiliation vs independent accreditation as an infrastructure-for-margin trade, host diligence and exit/data-ownership terms, registration and protection-licensing obligation *classes*, niche selection and its viability test, fee-model design, disclosure and capacity limits. |
| `kb2_trip_design_booking_and_suppliers` | **Delivery operations** — discovery as the value product, per-traveler constraint mapping, entry-document requirement classes, supplier vetting and fit-first selection with compensation disclosed, itinerary architecture, pre-commit booking verification, payment and deadline discipline, disruption response and redress, post-trip quality control. |
| `kb3_commissions_repeat_clients_and_addons` | **Recurring account & book management** — commissionable base and void conditions, booked-vs-traveled timing, expectation rows and reconciliation, unpaid-commission chase and clawbacks, cash-flow runway, planning-fee capture, protection add-ons under verified requirement classes, the repeat/referral engine and client-book value. |

Each KB is forged from its `*.spec.json` and gated at dense density; the specialist is
`travel_agent.specialist.json`, gated by `validators/specialist_validator.py`.

## Build

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/travel_agent
```

Forges each `*.spec.json` into a `*.kb.json`, gates all three at `--mode dense`, checks the
specialist's `grounded_in_kbs` resolve to those KBs, and gates the specialist. It must end
with `ALL GREEN`. Gates check structure and math only — never prose.

Specialist gate on its own:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/travel_agent/travel_agent.specialist.json \
  --report FACTORY/recurring_revenue/travel_agent/travel_agent.specialist.validation.json
```

## Using the specialist

Fill the three slots in `dist/prompt_template.json` → `specialist_prompt_template` and send to
any capable model — no training, no setup:

- `{{DOMAIN_LABEL}}` ← `"Travel Agent / Advisory Practice (heavy: grounded in 3 dense KBs)"`
- `{{SPECIALIST_SPEC_JSON}}` ← the **entire** contents of `travel_agent.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

```bash
python3 - <<'PY'
import json
t = json.load(open("dist/prompt_template.json"))["specialist_prompt_template"]
spec = open("FACTORY/recurring_revenue/travel_agent/travel_agent.specialist.json").read()
q = "Two suppliers can deliver this trip and one is a preferred partner that pays better — which do I book?"
print(t["system"].replace("{{DOMAIN_LABEL}}", json.loads(spec)["domain_label"]))
print(t["context"].replace("{{SPECIALIST_SPEC_JSON}}", spec))
print(t["user"].replace("{{USER_QUESTION}}", q))
PY
```

Invoke it for anything about starting, structuring, running, pricing, compliance-checking,
troubleshooting or growing a travel-advisor practice; any single client engagement inside one;
or the money and repeat-client machinery behind it.

## Boundary summary

- **Seller-of-travel registration classes** exist in some jurisdictions and are amended. The
  specialist names the class and the regulator that publishes it and requires the operator to
  **verify current rules in their own jurisdiction** — for every jurisdiction sold from *and*
  into — before advertising, quoting or taking money. Host affiliation is never assumed to cover
  an affiliated advisor without the clause or rule that says so.
- **Travel-protection selling** may carry licensing, insurer-appointment and permitted-activity
  obligation classes. Verify with regulator, insurer and host before any offer; if it cannot be
  confirmed, book the travel only and refer to a licensed source.
- **No invented figures.** Commissions, host splits, planning fees, insurance rates, market sizes
  and book-value multiples are always *mechanisms* — what the base is, what triggers payment,
  which document defines it, what moves it, which counterparty or statement yields the real
  number — or qualitative bands. Never a number asserted as fact.
- **Disclosure-first, fit-first.** The signed brief decides every recommendation; compensation may
  break a genuine tie and never create one; every preferred, consortium, host or add-on
  relationship is disclosed in writing, in the client's channel of record, before commitment.
  No pay-to-play steering.
- **Honest pricing against online.** State which price case applies, support any claimed advantage
  with a specific verifiable difference, and say plainly when the client's own channel is equal
  or better.
- **No fear-selling.** Risk is stated with the trip's own dates and exposed amounts — never
  catastrophe stories or manufactured scarcity. A booking may be lost rather than closed on fear.
- **Vulnerable-traveler and third-party-booking care outranks the sale.** Slow down, confirm
  authority and comprehension in writing, invite a support person of the traveler's choosing,
  accept a decline once and finally.
- **Educational and operational only** — never personalized financial, legal, tax, immigration,
  medical or insurance advice; never interpret coverage, assert a client is covered, or predict a
  claim outcome. Entry/health questions go to the official source; coverage questions to the
  policy wording, the insurer or a licensed professional.
- **Never merchant of record by default** — supplier-as-merchant and per-booking retention are the
  standing defaults; any pass-through of client funds is blocked until the client-funds
  requirement class is verified as current.
- **Money is counted traveled-and-received**, never booked; booked volume stays a dated pipeline
  line and clawback exposure is carried explicitly.
