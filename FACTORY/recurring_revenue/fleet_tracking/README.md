# fleet_tracking — GPS / Fleet Tracking Reseller

**Business & money model.** An independent operator sells GPS trackers to fleets and equipment
owners, installs them, and earns a **monthly per-unit tracking-platform subscription** for the life
of every installed unit. Revenue = tracker hardware (+ install labor, earned once) + recurring
per-unit subscription (compounds with every live unit). One FACTORY vertical: 3 dense KBs
(gated 35/35 each) distilled into 1 specialist (gated 15/15).

## The 3 knowledge bases

| file | covers |
|---|---|
| `kb1_fleet_market_and_reseller_economics.kb.json` | Which fleets to target and why they buy; dealer vs white-label channel economics as mechanisms; per-unit cost stack, price floor and payback; hardware-plus-subscription offer, pricing and term; ROI cases built only from the customer's own records; ethical-selling rules and the qualified-handoff gate. |
| `kb2_tracker_install_and_platform_onboarding.kb.json` | Intake survey and device-class matching (diagnostic-port, hardwired, battery asset); coverage qualification; hardwire design, concealment-vs-serviceability and the workmanship standard; platform account, pairing manifest, geofences, alerts and reports mapped to the fleet's workflow; driver-disclosure onboarding; per-unit verification and fleet acceptance. |
| `kb3_tracking_subscription_and_account_growth.kb.json` | Per-unit MRR ledger and tier/module mix; device-health and data-quality monitoring; remote-first service ladder, RMA and network-sunset planning; support cost per unit; billing hygiene, dunning and three-way seat reconciliation; contract register, health briefs, renewals, churn signals, saves; in-account expansion and the book-level MRR close. |

Each KB ships with its compact `*.spec.json` (the hand-authored source), the forged `*.kb.json`,
and its `*.validation.json` / `*.metrics.json` gate records.

## Build (forge + gate everything)

```bash
cd /home/user/automation
bash FACTORY/build.sh FACTORY/recurring_revenue/fleet_tracking
# forges each *.spec.json -> *.kb.json, gates each KB (--mode dense),
# checks grounding, gates the specialist. Must end: ALL GREEN.
```

Specialist gate on its own:

```bash
python3 validators/specialist_validator.py \
  FACTORY/recurring_revenue/fleet_tracking/fleet_tracking.specialist.json \
  --report FACTORY/recurring_revenue/fleet_tracking/fleet_tracking.specialist.validation.json
```

## Use the specialist

Harness: `dist/prompt_template.json` → `specialist_prompt_template`. Fill its three slots and send
to any capable model — no training, no setup:

- `{{DOMAIN_LABEL}}` ← `"GPS / Fleet Tracking Reseller (grounded in 3 dense KBs)"`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire contents of `fleet_tracking.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the operator's question (pricing a fleet, choosing a device class, a
  renewal or churn call, a reconciliation leak, an expansion ask …)

The model then operates strictly by the spec's `role`, `decision_procedure` and `workflow`, honors
every `escalation_triggers` entry, and answers only inside this domain. For automatic routing among
many specialists, use the same file's `router_prompt`.

## Boundary summary (hard stops)

- **Lawful tracking only.** Vehicles and assets the customer owns or controls as employer or lessor,
  with employee disclosure complete before units go live — enforced at the sale, at every turnover,
  expansion and save. Covert tracking of private individuals is refused with a scripted decline,
  logged, and never routed to a workaround. Personal-safety framings are referrals, not sales.
- **Regulation is named, never asserted.** Electronic-logging / hours-of-service mandates, telematics
  and workplace-privacy rules, camera and in-cab audio consent, collective consultation: name the
  obligation class, flag *verify current rules in your jurisdiction*, refer to a certified offering —
  no tracking package is presented as satisfying a mandate without named certification.
- **Numbers are mechanisms.** Commissions, spreads, prices, savings, support cost and account value
  are taught as how the number comes to exist, what moves it, and which of the operator's or
  customer's own records verifies it. No invented per-unit figure, rate, market size or multiple.
  No savings claim without a customer record or a measured baseline.
- **Ethical selling is a constraint, not a tone.** Disclosure raised proactively; urgency only from
  the prospect's own incidents; slower pacing, plain-language terms and full term totals for stressed
  or less commercially experienced buyers.
- **Educational and operational only.** No personalized legal, tax, insurance, employment or financial
  advice — the class is named and the decision routed to a licensed professional.
- **Install stays inside competence.** No unfused taps, no airbag/restraint/vehicle-network circuits,
  no batch before a per-model design is proven, no vehicle back in service unverified.
