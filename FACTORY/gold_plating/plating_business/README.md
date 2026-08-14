# plating_business — Plating Service Economics & Client Management Specialist

**Mission:** make the craft a business — honest quoting from mechanism-based costs, customer
intake that protects heirlooms and the shop, and a repeat trade book with jewellers and repair
shops. It is the commercial front and back of the bench: what a job should cost and whether it
should be taken at all, how a piece is documented, disclosed, approved or refused, and how
repeat trade work is qualified, promised, moved, recorded, claimed against and reviewed.
Part of the gold-plating bench program (`FACTORY/gold_plating/BRIEFS.json`, slug
`plating_business`).

## The 3 knowledge bases

| KB | File | Covers |
|---|---|---|
| KB1 | `kb1_service_design_and_job_pricing.kb.json` | Service classes (flash refresh / heavy replate / full restore / new work), the price mechanism (attended time at a costed shop rate + gold from plateable area × thickness × karat at a dated metal basis + loss factor + consumables + options + risk line), minimum charge, batching and rush as fixed-cost mechanisms, the written quote with its validity window and metal-movement clause, stop-and-re-quote, decline thresholds, and job-card reconciliation back into the model. |
| KB2 | `kb2_client_intake_and_expectations.kb.json` | The evidence-and-consent gate: custody and tagging, written identification, dated condition photos, history interview, observed-vs-claimed-vs-unknown discipline, risk-feature and irreplaceability triage, refusal criteria and how to deliver them, the never-solid-gold rule, wear and pre-existing-damage disclosure, the signed risk acknowledgment, per-step gates on irreversible actions, mid-job discovery stop-and-call, handover acceptance and complaint triage. |
| KB3 | `kb3_trade_accounts_and_repeat_book.kb.json` | The recurring engine: account qualification and terms sheets, tier basis funded by named savings, disclosure that travels with the goods, declared-value ceilings, carrier and cover classes, parcel reconciliation, custody-chain ledger and transit-loss protocol, capacity-derived turnaround classes and early slippage warning, claim triage by cause, replate-cycle demand, job history, ledger discipline, concentration and referral loops, offboarding and book review. |

## Build (forge + gate everything)

```bash
bash FACTORY/build.sh FACTORY/gold_plating/plating_business
```

Forges each `*.spec.json` into its `*.kb.json`, gates the KBs (`kb_validator --mode dense`),
then gates `plating_business.specialist.json` (`specialist_validator`). Must end `ALL GREEN`.
Current status: **ALL GREEN** — 3/3 KBs pass dense, specialist passes with zero warnings
(`plating_business.specialist.validation.json`).

## Use the specialist

Fill `dist/prompt_template.json` (`specialist_prompt_template`):

- `{{DOMAIN_LABEL}}` ← "Plating Service Economics & Client Management (gold-electroplating bench)"
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `plating_business.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the pricing, intake or trade-account question

Send to any capable model; it operates by the spec's role, decision procedure, workflow,
escalation triggers and validation checklist. In the bench team this specialist runs **first**
(intake, quote, risk sign-off) and hands the piece to `surface_prep`; `safety_compliance` is
the gate whose verdict wins over anything here.

## Boundary summary

- **Prices and margins are mechanisms**, never invented dollar figures: labour time at a costed
  shop rate + gold consumed from plateable area × deposit thickness × karat at a **dated** metal
  basis + loss factor + consumables + options + an explicit risk line. Gold-price movement is
  handled by the quote-validity window and its recompute clause, not by a remembered number.
- **Setpoints, thicknesses and wear lives are never asserted as universal fact** — they are
  mechanisms with a verification route, governed by the **technical data sheet of the specific
  purchased commercial product** and the shop's own measurement; published ranges are cited only
  as handbook-class ranges. Commercial chemistry only; no bath formulation from raw chemicals.
- **Never represent plated goods as solid gold** — not in speech, on a ticket, in advertising,
  or by silence. Wear expectations are disclosed before sale and in writing on customer pieces,
  and the plated character travels in writing into a trade account's own paperwork.
- **Liability is documented at intake**: dated condition photos, written scope in and out, and a
  signed risk acknowledgment before any bench work. Written refusal criteria dominate revenue and
  any client waiver — **no job is worth an irreplaceable loss**, and irreplaceable pieces are
  refused or run only under recorded per-step approval, never merely priced with a premium.
- **Safety and compliance beat throughput, cost, margin and deadline everywhere they meet**;
  conflicts escalate to a human with safety winning. Cyanide-bearing solutions and acids are
  never mixed, never stored together, never share a drain, and customer goods never enter the
  chemical area.
- **Regulated terms are obligation classes** — gold plated / vermeil / gold filled / rolled gold,
  nickel-release limits, waste and discharge rules — named with their authority and a
  verify-your-jurisdiction instruction, never asserted as numbers; this specialist flags marking
  and karat claims, humans approve them.
- **Educational only**: trade terms, ceilings, credit and liability positions are commercial
  mechanisms for the shop's own counsel and insurer — not legal, tax, investment or medical
  advice. Nickel-allergy and exposure questions route to the regulation classes and to a
  professional; no skin-safety assurance is given at the counter.
