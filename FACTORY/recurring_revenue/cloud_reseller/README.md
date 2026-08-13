# cloud_reseller — Business Email & Cloud Reseller

**Money model.** Set up and manage Microsoft 365 / Google Workspace and adjacent cloud
subscriptions for small businesses. Two revenue legs: a **one-time setup and migration fee**
plus a **recurring per-seat margin and management fee**. What is sold is not the software —
the vendor makes it and would sell the client directly at list price — but the
administration, continuity and accountability wrapped around someone else's subscription.

## The 3 KBs (forged from `*.spec.json`, gated dense)

| KB | Domain | One line |
|---|---|---|
| `kb1_cloud_reseller_programs_and_smb_targeting` | Cloud Reseller Programs & SMB Targeting | Which partner-program route to transact under (who invoices, who carries credit risk, who answers first), per-seat margin as a per-deal mechanism, the anchor suite, SMB segments and trigger events, ethical right-sizing, disclosed offer and signed authorization. |
| `kb2_tenant_migration_and_onboarding` | Tenant Setup, Migration & Onboarding | Documented authorization and least-privilege custody, tenant and identity architecture, admin separation, break-glass and MFA enforcement, security baseline and recoverability, DNS/domain control and mail authentication, migration method and fidelity, pilot, timed cutover with rollback, training, hypercare, as-built pack and acceptance. |
| `kb3_subscription_management_and_expansion` | Subscription Management, Support & Expansion | The monthly rhythm: three-way seat reconciliation and true-up/true-down, access re-attestation and joiner/mover/leaver, invoice integrity, the support model and its boundary, recurring posture reviews and drift watch, incident first hour, evidence-led expansion, renewals and price-change disclosure, churn signals, clean exit. |

Each KB also ships `*.metrics.json` and `*.validation.json` beside it.

## Specialist

`cloud_reseller.specialist.json` — the distilled operating spec (gated, `overall_status: pass`,
0 warnings; report in `cloud_reseller.specialist.validation.json`). It is grounded in the three
`*.kb.json` files above by absolute path and covers the whole arc: acquisition → delivery →
recurring operations.

## Build / verify

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/cloud_reseller
```

Forges each `*.spec.json` → `*.kb.json`, gates all three dense, checks the specialist's
`grounded_in_kbs`, then gates the specialist. Must end **`ALL GREEN`**.

Specialist gate alone:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/cloud_reseller/cloud_reseller.specialist.json \
  --report FACTORY/recurring_revenue/cloud_reseller/cloud_reseller.specialist.validation.json
```

## Use the specialist

Standard harness — `dist/prompt_template.json` (see `FACTORY/MAKE_A_SPECIALIST.md` §7). Fill the
three slots of `specialist_prompt_template` and send to any model:

- `{{DOMAIN_LABEL}}` ← the spec's `domain_label`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `cloud_reseller.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the end-user's question

The model then operates strictly by the spec's `role`, `decision_procedure` and `workflow`, and
honours every `escalation_trigger`. No KBs are loaded at answer time — the spec stands alone.
For routing across many specialists, use the `router_prompt` in the same file.

**Invoke it when** the task concerns choosing or pricing a cloud-reseller offer, qualifying or
declining a small-business prospect, planning or executing a tenant build, migration or cutover,
administering seats, licences, invoices, support, posture reviews, renewals or expansion on a
live tenant, or handing an account back at exit.

## Boundary summary

- **Access is fiduciary-grade.** Nothing happens in a tenant or against client data without a
  current written authorization from a named signer covering that class of action — least
  privilege, time-bound, logged, re-attested, revocable by the client in minutes. No standing
  global rights except under a named, dated, expiring exception.
- **Client owns the tenant and the data.** A named client officer holds and has used their own
  administrative credential; a break-glass account exists and is alerted on. At exit, ownership,
  documentation, exports and credentials are handed over and operator access removed regardless
  of any unpaid balance; debt is pursued entirely separately.
- **Program terms are mechanisms, not memory.** CSP-class margin structures are described by how
  the number comes to exist and what moves it; price, term class, adjustment and cancellation
  windows, caps, eligibility and promotions come from a dated register entry with its source and
  checked date, or are marked unknown and verified. Verify the current program before relying on
  anything.
- **No invented figures.** No commission, price, margin percentage, rate, market size or account
  multiple is stated as fact — only mechanisms, the inputs that move them, and how to verify the
  real number for a specific deal.
- **No fear-selling.** Every upgrade or add-on cites a dated observation in this client's own
  environment, states the do-nothing option and its consequence and what the product does not
  cover; declines are recorded and respected. The compensation model and the client's
  direct-purchase alternative are disclosed in writing *before* the recommendation.
- **Regulatory content names the obligation class only.** Data protection, retention and legal
  hold, monitoring and export, breach notification, sector rules — each is named, routed to the
  client's own qualified adviser, and flagged "verify current rules in your jurisdiction". No
  compliance outcome is ever promised.
- **Educational and operational only** — never personalized financial, legal, tax, medical or
  insurance advice.
- **No promise that cannot be executed.** Cancellation and reduction rights mirror the commitment
  term actually purchased upstream; fidelity promises never exceed the chosen method and tool; a
  vendor's platform commitment is never restated as the operator's own response commitment.
- **Decline what cannot be served.** A regulated data class or contractual requirement beyond the
  operator's competence, an untested capability, or a client with nobody authorized to sign — is
  referred or declined with the reason stated, not learned on the client's live data.
