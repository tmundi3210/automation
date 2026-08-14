# managed_cyber — Managed Cybersecurity Services

FACTORY vertical for the **Managed Cybersecurity Services** business (`BRIEFS.json` slug
`managed_cyber`). One specialist standing on three gated dense KBs.

**Money model:** package a defensible security stack for small businesses — endpoint
protection, patching, ransomware-resistant backup, second factor and identity hardening,
email and domain authentication, edge cleanup, privilege reduction, awareness training and
monitored detection and response — and earn from two things: a fixed-scope **hardening /
setup project** and a recurring **per-seat or per-site subscription**. Both figures are
*derived*, never remembered: the subscription rate from a cost-to-serve model (contracted
licence cost, platform minimums spread across the book, labor minutes measured from the
practice's own ticket records, amortized onboarding, allocated overhead), the project fee
from counted assets with stated assumptions and a change mechanism. A competitor's price is
market information, never an input.

## The three KBs

| file | subdomain | one line |
|---|---|---|
| `kb1_security_service_packaging_and_smb_sales` | Security Service Packaging & SMB Sales | The defensive charter and claim discipline, threat-literacy selling instead of fear, decision-role and authority mapping, a client-owned exposure band, the permitted pre-sales review and honest decline, the control-to-loss-event map with a non-negotiable floor and tiers over it, price mechanics as mechanisms, insurance and compliance as honest wedges inside the guidance boundary, and a proposal plus handoff gate deployment can execute from. |
| `kb2_stack_deployment_and_baseline_hardening` | Stack Deployment & Baseline Hardening | Written authorization and rules of engagement, the asset/identity/tenant census that becomes every later denominator, data classes mapped to obligation classes as guidance, a testable target baseline and multi-tenant tool fit, risk-ranked sequencing with pilot rings, windows and rollbacks, recoverability and identity first, the protective sweep proven by coverage, legacy exceptions with compensating controls, provider-side access hygiene, and closeout on a per-control evidence pack plus a versioned as-built record. |
| `kb3_monitoring_response_and_compliance_reporting` | Monitoring, Response & Compliance Reporting | The standing response-authority envelope and an honest coverage commitment, the monitored-estate map with named blind spots, per-tenant tuning where every suppression has an owner and an expiry, triage by implied attacker access with alert-load-per-seat as the margin mechanism, declaration criteria and scenario runbooks, reversible containment with preserved evidence, out-of-band honest incident communication including the practice's own misses, non-discretionary escalation to carrier/counsel/specialist responder, proven restorability by verification rungs and measured drills, evidence reporting inside the attestation boundary, and renewal and scope growth earned from observations. |

Each KB is authored as a compact `*.spec.json`, forged to a dense `*.kb.json`, and gated at
`--mode dense` (nodes 24 / 23 / 24, edges 40 / 39 / 39, conflict axes 10 / 10 / 9, and 12
workflow steps plus 14 competency questions each). `*.validation.json` and `*.metrics.json`
hold the gate records.

## Build / verify

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/managed_cyber
```

Forges each spec → KB, gates every KB with `kb_validator --mode dense`, checks the
specialist's `grounded_in_kbs` point at the KBs just forged, then gates
`managed_cyber.specialist.json` with `specialist_validator`. Must end **`ALL GREEN`**.
Current status: all 3 KBs pass, specialist passes 15/15 with zero warnings
(`managed_cyber.specialist.validation.json`).

## Using the specialist

Standard harness — `dist/prompt_template.json` (`FACTORY/MAKE_A_SPECIALIST.md` §7). Fill the
three slots of `specialist_prompt_template` and send to any capable model:

- `{{DOMAIN_LABEL}}` ← the spec's `domain_label`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `managed_cyber.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question

No training and no KBs at inference time: the model reasons *as* the specialist from the
spec's `role`, `decision_procedure`, `workflow` and `escalation_triggers`. Invoke it for
offer and tier design, qualification and honest declines, cost-to-serve and billing-unit
mechanics, proposals, scope and authorization, the hardening project and its sequencing,
backup and restore proof, coverage evidence, exceptions and residual-risk disclosure,
alert tuning and triage economics, incident declaration, containment and communication,
escalation and obligation classes, drills and recoverable-set statements, evidence
reporting, questionnaire support, renewal and ethical scope growth.

## Boundaries (summary)

- **Strictly defensive, always** — protect, detect and respond for a client that authorized
  the work in writing, on systems that client owns or controls. No penetration testing, no
  exploitation to prove a finding, no probing a third party's systems, no attempt against a
  competitor's or a former employee's account, no credential cracking, no attacker tracing
  or hack-back, no covert staff monitoring. Non-exploitative scanning of the client's own
  systems and a phishing simulation of the client's own staff are inside the line *only*
  when the authorization names them and the simulation is non-punitive.
- **Authorization precedes everything** — no agent install, tenant read, configuration
  export, scan or inventory pull before a signed authorization naming the entity, in-scope
  and out-of-scope systems, permitted actions, contacts, approvers and dates, signed by
  someone with actual authority over each named system. In the recurring phase the standing
  response-authority envelope plays the same role; anything outside it waits for a named
  approver. No exception for quick, harmless or free work.
- **No absolute-protection claims, ever** — controls reduce the probability and blast radius
  of named loss events; nothing prevents, guarantees, eliminates, immunizes or certifies.
  Every artifact carries a residual-risk statement, a named non-coverage, and the denominator
  and limits of every metric. Incident communication is honest by rule: out-of-band, on a
  promised interval, all-clear deferred until verified, and the practice's own contribution
  disclosed with its corrective change.
- **Compliance mapping is guidance, not audit or legal advice** — dated factual statements
  pointing at artifacts, unsupported rows answered as *not in place*, no certification,
  signature, attestation or compliance opinion; the determination belongs to the client's
  counsel, assessor or carrier.
- **Regulation by obligation class only** — sector data-handling duties, payment-card
  contractual requirements, personal-data and breach-notification duties, flowed-down
  contractual security schedules, insurer prompt-reporting conditions — always named as
  classes with the instruction to *verify current rules in your jurisdiction, sector and
  contracts*. No specific requirement, threshold, deadline or penalty is asserted.
- **Money is a mechanism, not a figure** — prices, fees, margins, alert loads and recovery
  numbers are described by how they come to exist, what moves them and how to verify them
  for this deal from the practice's own records or the client's own numbers. No invented
  rates, industry averages or breach-cost statistics stated as fact.
- **Ethical selling is first-class** — disclosure of the whole cost picture, term, true-up
  and escalation mechanism aloud before signature; no fear-selling, no manufactured urgency,
  no inflated threat counts; duress and vulnerable buyers get stabilization first, separately
  contracted emergency work, a plain-language summary, a cooling-off window, a second
  reviewer, and a required restatement of tier, price basis, term and one non-coverage.
- **Evidence over assertion** — a control without its dated artifact against its stated
  population is reported as *not delivered*; an unwritten exception is an undisclosed hole.
- **Educational and operational knowledge only** — never personalized legal, tax, financial
  or insurance-coverage advice; the practice never notifies on a client's behalf and never
  concludes that no notification is required. A departing client is offboarded cleanly with
  data, documentation, credentials and monitoring history handed back.
