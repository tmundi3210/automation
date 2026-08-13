# managed_wifi — Managed Wi-Fi Provider

FACTORY vertical for the recurring-revenue specialist program.

**Business / money model:** design and install business Wi-Fi (gateway, switching, cabling,
access points) for a venue, then charge a **monthly management, monitoring and support
subscription** to own the question "is the Wi-Fi working?" Venues: restaurants and bars,
motels and small lodging, offices, retail shops, and multi-dwelling / multi-tenant
properties. Revenue = one-time design + install, plus the recurring per-site subscription —
and the subscription book, not the hardware markup, is the compounding asset. The monthly
figure is built as a **mechanism**: a per-site component (platform or controller licence
attached to the site, monitoring, share of the support rota, reporting time, travel radius)
plus a per-managed-device component, every part traced to a cost document the provider
actually holds. No invented price points, credit amounts or availability figures.

## The 3 knowledge bases

| KB | File | Covers |
|---|---|---|
| KB1 | `kb1_managed_wifi_offer_and_target_venues.kb.json` | Productizing Wi-Fi-as-a-service (design + hardware + monthly care); venue-class buying anatomy and per-class offer shaping; guest-amenity vs operations-dependence value split; guest-privacy promise and honest-claim discipline; per-site/per-AP price mechanism, scope and tier ladder, carrier/managed/application responsibility boundary, equipment ownership and recovery, term and exit; serviceability check, go/no-go screen and the signable-proposal gate. |
| KB2 | `kb2_wifi_design_deployment_and_handoff.kb.json` | Engineering scope from the signed offer; RF and physical site survey with measured-vs-blocking input tagging; coverage thresholds and busy-hour capacity targets; AP placement, channel and power planning; guest/POS/back-office SSID-VLAN segmentation with default-deny policy, guest isolation and the payment-segment obligation class; cloud-manageable platform and tenancy; captive portal with its disclosure and consent record; cabling, PoE budget and code classes; installation, ISP handoff, config baseline; validation suite, punch list, as-built package and the operations handoff gate. |
| KB3 | `kb3_monitoring_management_and_sla_operations.kb.json` | Operations intake and baseline; telemetry-only guest-privacy stewardship; per-site responsibility matrix; monitoring stack with an independent probe, alert tuning and remote-first outage triage; preventive maintenance, staged firmware rings and the separate assessed-critical vulnerability path; change control and drift detection; keepable SLA design, honest availability measurement and breach remedy; support tiers and vendor/carrier escalation; cost-to-serve, value reporting, churn watch, honest renewal, venue-ownership transitions and health-gated expansion to more sites and adjacent services. |

The specialist is `managed_wifi.specialist.json`, distilled across all three and gated by
`validators/specialist_validator.py` (report: `managed_wifi.specialist.validation.json` —
15/15 pass, no warnings).

## Build / gate

```bash
cd /home/user/automation
bash FACTORY/build.sh FACTORY/recurring_revenue/managed_wifi   # must end "ALL GREEN"
```

This forges each `*.spec.json` into its `*.kb.json`, gates every KB in dense mode, checks
the specialist's grounding, and gates the specialist.

## Using the specialist

Fill the three slots of `dist/prompt_template.json` (`specialist_prompt_template`):

- `{{DOMAIN_LABEL}}` <- "Managed Wi-Fi Provider Business"
- `{{SPECIALIST_SPEC_JSON}}` <- the entire `managed_wifi.specialist.json`, verbatim
- `{{USER_QUESTION}}` <- the end-user's question

Send the assembled prompt to any capable model; it operates by the spec's `role`,
`decision_procedure`, `workflow` and `escalation_triggers`, and answers only within the
domain.

## Boundary summary

- **Guest-network privacy is absolute** — no traffic interception, content inspection,
  browsing-history logging, profiling or sale/brokerage of guest or resident data;
  monitoring is telemetry-only, the guest segment runs with client isolation on, and any
  captive-portal collection is venue-chosen, plainly disclosed on the page, affirmatively
  consented for marketing use, venue-owned, retention-bounded and signed off in writing
  before go-live.
- **Pricing is a mechanism, never an invented figure** — per-site plus per-managed-device
  components traced to held cost documents, with modifiers and a validity window; service
  credits, licence costs, hardware recovery and any book valuation are described by what
  moves them and how to verify them per deal, in mechanisms or qualitative bands only.
- **Spectrum, wiring, construction and code are site facts to survey** — never assumptions
  imported from venue type; every design-driving input is tagged measured, verified or
  unknown-and-blocking, and no dependent step proceeds on an unknown.
- **Regulatory content names the obligation class** — contractor/low-voltage licensing,
  electrical and fire code, plenum and penetration rules, permits and inspections,
  landlord/lease/franchisor consent, payment-card segment obligations, privacy-notice
  duties and automatic-renewal statutes each carry "verify current rules in your
  jurisdiction"; no local specific is asserted and no compliance state is claimed as
  certified.
- **Ethical selling is first-class** — disclosure-first proposals with exclusions in the
  same document and the same plain language as inclusions, no fear-selling or manufactured
  urgency, consequences drawn only from the venue's own stated dependencies, competitors
  described by inclusions/exclusions rather than insinuation, permissioned or attributed
  proof assets, and explicit care for buyers with no technical staff.
- **Uptime is measured or unknown** — every availability figure traces to probe data under
  a disclosed, stable method; monitoring-blind windows are reported as unknown, method
  changes are announced prospectively, and a bad month is published with its cause and
  corrective action.
- **The venue owns its own network** — credentials, current configuration and documentation
  for venue-owned equipment are handed over as written at exit; retention is never pursued
  by withholding passwords or records.
- Educational and operational knowledge only — never personalized legal, financial, tax,
  insurance or employment advice; binding contracts, regulatory ambiguity and compelled
  data demands escalate to qualified human review.
