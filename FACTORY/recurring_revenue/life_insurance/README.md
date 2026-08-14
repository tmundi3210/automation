# Life Insurance Agent — FACTORY vertical

**Business.** A licensed life insurance agent practice: solicit and place life products for households
and business owners, then service what was placed.

**Money model.** Revenue is commission on placed policies plus renewal and trail compensation where the
carrier's schedule and the producer's contract provide it. Compensation structures are treated as
**mechanisms** (which premium basis, first-year versus renewal, advance versus as-earned, chargeback rule
and window, what contract level does to the split) that are verified per product on the current schedule —
never as invented percentages. The durable asset is a **persistent, well-served book**, so the practice is
judged on cases still paying at the horizon rather than on submitted production.

## The three knowledge bases

| KB | Scope |
|---|---|
| `kb1_licensing_carriers_and_market_focus` | The licensing path and continuing obligations as jurisdiction-specific requirement classes; captive vs independent vs organization-affiliated models and who owns the book; carrier and product-shelf selection; market focus (families, business owners, final expense) and lead-generation ethics. |
| `kb2_needs_analysis_product_fit_and_suitability` | Disclosure-first fact-finding and needs analysis; term vs permanent mechanics; matching product to need under suitability and best-interest standards; application, underwriting and delivery; honest illustration handling and replacement discipline. |
| `kb3_persistency_renewals_and_book_building` | Persistency as the economic engine (chargebacks, renewals, trails as mechanisms); onboarding and service cadence that keeps policies in force; reviews, contractual deadlines and life-event triggers; referral systems; building a book that compounds rather than churns. |

Each KB is forged from its `*.spec.json` and gated dense (nodes/edges/axes/workflow/CQ count bands, reference
integrity, formula consistency). The specialist `life_insurance.specialist.json` is the distilled operating
layer over all three.

## Build (forge + gate everything)

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/life_insurance
```

Must end `ALL GREEN`. It forges each `*.spec.json` into `*.kb.json`, gates each KB with
`validators/kb_validator.py --mode dense`, checks `grounded_in_kbs`, then gates the specialist with
`validators/specialist_validator.py`. Gate the specialist alone with:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/life_insurance/life_insurance.specialist.json \
  --report FACTORY/recurring_revenue/life_insurance/life_insurance.specialist.validation.json
```

## Use the specialist

Load `dist/prompt_template.json` and fill `specialist_prompt_template`:

- `{{DOMAIN_LABEL}}` ← `Life Insurance Agent Practice: Licensing, Suitable Placement & Book Persistency`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `life_insurance.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

Send to any capable model. Its `role`, `decision_procedure`, `workflow` and `escalation_triggers` drive the
answer; the KBs are not loaded at answer time. For automatic selection among many specialists, use the
`router_prompt` in the same file with a routing table.

## Boundaries (hard stops carried into the specialist)

- **Licensing first.** No solicitation, quote, illustration or application before an active license with the
  life line of authority in the applicable state **plus** an in-force carrier appointment (or a verified
  just-in-time rule). Requirements are jurisdiction-specific and time-varying — verify current rules with the
  insurance department, carrier or the practice's own compliance counsel.
- **Educational and operational knowledge only.** Not personalized financial, insurance, tax, legal, estate or
  medical advice; suitability and best-interest obligation classes govern every recommendation, which must be
  derived from the documented fact-find rather than from inventory or compensation.
- **No performance promises on cash-value products.** Guaranteed columns are the purchase-decision basis;
  non-guaranteed columns are labelled projections shown with a reduced-assumption scenario. No
  investment-return framing.
- **Replacement is a regulated transaction.** Burden of proof sits on the new contract with a documented
  side-by-side comparison and the jurisdiction's notices; existing coverage stays in force until the new
  contract is issued and delivered.
- **No fabricated figures.** Commissions, overrides, premiums, rates, lead costs, persistency rates and book
  multiples are mechanisms, qualitative bands, or the practice's own dated measured numbers — each with the
  document that would produce the real figure.
- **Ethical selling is first-class.** Disclosure-first identification on every contact; no fear-selling,
  manufactured urgency or obscured purpose; heightened care for older, grieving, impaired, language-limited,
  isolated or third-party-pressured buyers, including a documented stop where comprehension or capacity cannot
  be established; consent and contact-preference evidence outranks lead volume and even a policy save.
