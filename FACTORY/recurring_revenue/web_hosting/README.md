# web_hosting — Website & Hosting Reseller

FACTORY vertical for the **Website & Hosting Reseller** business (`BRIEFS.json` slug
`web_hosting`, wave 2). One specialist standing on three gated dense KBs.

**Money model:** build small-business websites and sell a monthly care plan alongside every
build — revenue = a one-time fixed-scope build fee + recurring hosting, updates, backups,
monitoring and a bounded edit allowance. Both prices are *derived* from the operator's own
measured cost ledger (hours at the rate the operator needs their hours to earn, pass-through
costs, rework and incident reserves, per-site infrastructure share, logged support minutes,
margin) — never from an invented or remembered market rate.

## The three KBs

| file | subdomain | one line |
|---|---|---|
| `kb1_productized_web_offer_and_acquisition` | Productized Web Offer & Client Acquisition | What the fixed package plus care plan is, which local niche it aims at, how both prices are derived from measured cost, and how clients are won, qualified and scoped so nothing is promised that cannot be verified or delivered. |
| `kb2_site_build_stack_and_delivery` | Site Build Stack & Delivery Process | One repeatable stack and design system, the build order → content intake → staging build → bounded review pipeline, the quality/performance/accessibility baseline, and a domain/DNS/email cutover and launch that never breaks the client's existing business. |
| `kb3_hosting_care_plans_and_retention` | Hosting, Care Plans & Client Retention | Where a portfolio of small sites lives and how blast radius is contained, backups/restore drills/updates/security/monitoring/incidents run as *proven* work, care tiers derived from real procedures and repriced from measured cost-to-serve, monthly evidence of invisible work, and retention, dunning, capacity and graceful offboarding with a complete asset handover. |

Each KB is authored as a compact `*.spec.json`, forged to a dense `*.kb.json`, and gated at
`--mode dense` (nodes 23 / 23 / 24, edges 38 / 40 / 38, 10 conflict axes and 12 workflow
steps each). `*.validation.json` and `*.metrics.json` hold the gate records.

## Build / verify

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/web_hosting
```

Forges each spec → KB, gates every KB with `kb_validator --mode dense`, checks the
specialist's `grounded_in_kbs` point at the KBs just forged, then gates
`web_hosting.specialist.json` with `specialist_validator`. Must end **`ALL GREEN`**.
Current status: all 3 KBs pass, specialist passes 15/15 with zero warnings
(`web_hosting.specialist.validation.json`).

## Using the specialist

Standard harness — `dist/prompt_template.json` (`FACTORY/MAKE_A_SPECIALIST.md` §7). Fill the
three slots of `specialist_prompt_template` and send to any capable model:

- `{{DOMAIN_LABEL}}` ← the spec's `domain_label`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `web_hosting.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

No training, no KBs at inference time: the model reasons *as* the specialist from the spec's
`role`, `decision_procedure`, `workflow` and `escalation_triggers`. Invoke it for offer and
niche design, build or care-plan pricing mechanics, proposals and scope, the build pipeline,
DNS/email cutover, launch gates, hosting architecture, tier contents, monthly value
reporting, churn and dunning, capacity, or offboarding a web client.

## Boundaries (summary)

- **The client owns their assets** — domain, content, photographs, site files, database,
  analytics and lead inbox. Lock-in is earned by service quality, never by holding assets;
  they are released on request regardless of any unpaid balance, and money is pursued only
  through the agreed terms and ordinary collections.
- **Prices are mechanisms, not figures** — build fee and care-plan tiers are described by how
  the number is built and what moves it, with the operator's own invoices and logged minutes
  named as the verification; no invented market rates, averages or multiples.
- **No SEO/traffic guarantees, ever** — no ranking, traffic, enquiry, lead-count or revenue
  promise in any wording; every performance, availability, security or accessibility claim
  names the page, the tool, the date and the record that can be re-run by a skeptic.
- **Ethical selling and retention are first-class** — disclosure of the whole cost picture
  before any signature and in writing before every irreversible or billable step; no
  fear-selling, manufactured urgency or exit friction; heightened care and a recorded
  restatement for a buyer who cannot evaluate the claims being made to them.
- **Continuity is untouchable** — no DNS or hosting change without a zone export, verbatim
  mail-record replication, a one-person rollback and a post-change mailbox test.
- **Regulation by obligation class only** — accessibility, privacy and cookie disclosure,
  personal-data handling and breach notification, consumer-contract and billing duties are
  named as classes with the instruction to *verify current rules in the client's
  jurisdiction*; no specific requirement, threshold, deadline or penalty is asserted.
- **Educational and operational knowledge only** — never personalized legal, tax,
  accounting, insurance or financial advice.
