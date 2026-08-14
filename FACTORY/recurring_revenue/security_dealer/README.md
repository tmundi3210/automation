# security_dealer — Home Security & Alarm Dealer

FACTORY vertical for the recurring-revenue specialist program.

**Business / money model:** sell and install monitored security systems (alarm, doorbell,
cameras); revenue = equipment + installation fee + a recurring monitoring/support account.
The monitored account — not the installation — is the product; the recurring monitoring
spread is the asset the business exists to own.

## The 3 knowledge bases

| KB | File | Covers |
|---|---|---|
| KB1 | `kb1_dealer_program_and_account_economics.kb.json` | Authorized-dealer program vs independent model; how an account is created and funded (subsidy vs margin, wholesale-retail spread, sell-vs-hold, holdback/chargeback mechanics); territory, segments, lead engine and disclosure-first in-home selling. |
| KB2 | `kb2_system_design_and_installation.kb.json` | Site survey and risk walk-through; layered panel/sensor/doorbell/camera design to the home's threat model; permits, licensing awareness, false-alarm ordinances; workmanship, commissioning, training, closeout; sub vs in-house installs. |
| KB3 | `kb3_monitoring_rmr_and_retention.kb.json` | Central-station vs self/cloud monitoring stack; RMR ladder (monitoring, interactive, cloud video, service plans); attrition drivers, churn radar, ethical saves and mover program; contract/disclosure hygiene and monitored-book value. |

The specialist is `security_dealer.specialist.json`, distilled across all three and gated
by `validators/specialist_validator.py` (report: `security_dealer.specialist.validation.json`).

## Build / gate

```bash
cd /home/user/automation
bash FACTORY/build.sh FACTORY/recurring_revenue/security_dealer   # must end "ALL GREEN"
```

This forges each `*.spec.json` into its `*.kb.json`, gates every KB in dense mode, checks
the specialist's grounding, and gates the specialist.

## Using the specialist

Fill the three slots of `dist/prompt_template.json` (`specialist_prompt_template`):

- `{{DOMAIN_LABEL}}` <- "Home Security & Alarm Dealer Operations"
- `{{SPECIALIST_SPEC_JSON}}` <- the entire `security_dealer.specialist.json`, verbatim
- `{{USER_QUESTION}}` <- the end-user's question

Send the assembled prompt to any capable model; it operates by the spec's role, decision
procedure, workflow, and escalation triggers.

## Boundary summary

- **Defensive, lawful security only** — systems prevent, detect and alarm intrusion and
  monitored life-safety events; no covert surveillance, neighbor views, or offensive use.
- **Licensing/permits vary by state and city** — obligation classes are named with a
  "verify current rules in your jurisdiction" flag; local specifics are never asserted.
- **Consumer-protection-sensitive contracts** — term, auto-renewal, total cost and
  cancellation are disclosed honestly before signature; renewals get proactive notice.
- **Mechanisms, not invented figures** — subsidies, multiples, holdbacks, spreads and book
  values are explained as per-contract mechanisms to verify, never asserted numbers.
- **Ethical selling is first-class** — disclosure-first, no fear-selling, and
  vulnerable-consumer care dominate every quota, script and incentive.
- Educational/operational knowledge only — never personalized financial, legal, tax,
  medical or insurance advice.
