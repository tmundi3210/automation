# voip_reseller — Business VoIP Reseller

**Business / money model:** a local operator sells and configures business phone systems for
small firms (dentists, trucking, restaurants, motels, small offices), ports their existing
numbers, and bills monthly. Revenue stacks four layers: hardware/setup margin + installation
labor + monthly per-seat service + ongoing support — compounding into an MRR book.

## The 3 KBs

1. **kb1_voip_channel_economics_and_segments** — agent vs wholesale-reseller vs white-label
   (who bills, who owns the customer, who carries the risk), vertical niche selection and
   beachhead ranking, offer bundling, per-seat price construction with a floor, and honest
   claims-substantiated positioning.
2. **kb2_deployment_porting_and_qos** — signed deal to stabilized service: site discovery and
   analog-dependency audits, seat/feature/endpoint/call-flow design, CSR-reconciled number
   porting that never endangers the number, network readiness/QoS/failover, E911 dispatchable
   locations as a hard gate, and a trained, tested, reversible cutover with a handoff gate.
3. **kb3_recurring_service_support_and_expansion** — running the MRR book: the support charter
   the seat fee funds, layered triage and carrier escalation, continuing E911 record upkeep,
   billing/contract/renewal hygiene, honest churn saves, clean port-out exits, and
   record-evidenced seat/feature/cross-sell expansion.

## Build (forge + gate everything)

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/voip_reseller
# must end: ALL GREEN
```

## Use the specialist

Fill `dist/prompt_template.json`'s `specialist_prompt_template` slots:
`{{DOMAIN_LABEL}}` = the spec's `domain_label`, `{{SPECIALIST_SPEC_JSON}}` = the entire
`voip_reseller.specialist.json` verbatim, `{{USER_QUESTION}}` = the question. Send to any
capable model; it operates by the spec's role, decision procedure, workflow, and escalation
triggers.

## Boundary summary

- E911 (Kari's Law / RAY BAUM class) is a hard flag on every quote, deployment, and account —
  verify current jurisdiction rules; never skip or value-engineer it out.
- Telecom regulatory/tax treatment (USF-class) varies by channel model — always routed to a
  qualified professional; no in-house position ever asserted.
- All economics (commissions, prices, margins) are mechanisms verified per deal against the
  live agreement or rate card — never invented figures stated as fact.
- No uptime or savings claims without a verified basis (bill audit; upstream published SLA
  plus demonstrated failover only).
- Disclosure-first selling, no fear-selling, vulnerable-buyer care slows the close; port-outs
  are never obstructed; educational/operational guidance only, never personalized legal, tax,
  or financial advice.
