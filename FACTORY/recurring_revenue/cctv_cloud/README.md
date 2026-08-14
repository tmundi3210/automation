# cctv_cloud — CCTV & Cloud Video Recording

FACTORY vertical for the recurring-revenue specialist program.

**Business / money model:** own-build vertical (not a major-company agent): camera
installation plus a cloud recording/monitoring/support subscription per site; revenue =
one-time install + monthly cloud storage/monitoring/support. The per-site subscription
book — not the camera hardware — is the compounding asset the business exists to own.

## The 3 knowledge bases

| KB | File | Covers |
|---|---|---|
| KB1 | `kb1_surveillance_offer_design_and_pricing.kb.json` | Productizing cameras + cloud + support into honest subscription tiers; segment targeting and beachhead choice; white-label VSaaS vs own-stack as a mechanism; cost-stack, floor/ceiling pricing, hardware-margin allocation and contract design; disclosure-first ethical selling without fear-mongering. |
| KB2 | `kb2_camera_system_design_and_installation.kb.json` | Site survey and measured-uplink qualification; per-zone deter/detect/identify coverage with the lawful-placement screen; DORI pixel-density camera selection and placement; edge/cloud/hybrid recording sized by bitrate/storage/uplink math; camera-network segmentation and hardening; workmanship, day-and-night commissioning and as-built handover. |
| KB3 | `kb3_cloud_video_ops_and_privacy_compliance.kb.json` | Running the recurring service: camera-health and recording-gap monitoring, storage/retention management with auto-purge and legal holds, evidence-grade incident retrieval and third-party request adjudication; privacy/consent/signage compliance, least-privilege access and view auditing; proof-of-value reporting, honest renewals, ethical saves and clean offboarding. |

The specialist is `cctv_cloud.specialist.json`, distilled across all three and gated by
`validators/specialist_validator.py` (report: `cctv_cloud.specialist.validation.json`).

## Build / gate

```bash
cd /home/user/automation
bash FACTORY/build.sh FACTORY/recurring_revenue/cctv_cloud   # must end "ALL GREEN"
```

This forges each `*.spec.json` into its `*.kb.json`, gates every KB in dense mode, checks
the specialist's grounding, and gates the specialist.

## Using the specialist

Fill the three slots of `dist/prompt_template.json` (`specialist_prompt_template`):

- `{{DOMAIN_LABEL}}` <- "CCTV & Cloud Video Recording Business"
- `{{SPECIALIST_SPEC_JSON}}` <- the entire `cctv_cloud.specialist.json`, verbatim
- `{{USER_QUESTION}}` <- the end-user's question

Send the assembled prompt to any capable model; it operates by the spec's role, decision
procedure, workflow, and escalation triggers.

## Boundary summary

- **Lawful, disclosed surveillance only** — deterrence, detection and evidence for the
  property owner; covert or unlawful monitoring requests get a scripted decline and are
  logged, at sale, design and mid-contract alike.
- **Jurisdiction variance is first-class** — video privacy, audio-recording consent,
  camera-placement, signage and auto-renewal rules are named as obligation classes with a
  "verify current rules in your jurisdiction" flag on every design and promise; local
  specifics are never asserted.
- **Retention and access are product, not fine print** — retention days, recording mode,
  viewer roles, export handling and operator access limits appear priced on the tier
  sheet, decided in the design spec, and enforced in live purge/access configuration.
- **Mechanisms, not invented figures** — VSaaS vs own-stack economics, storage costs,
  per-camera prices, install rates and book multiples are explained as mechanisms to
  verify per deal (rate cards, measured usage, own documents), never asserted numbers.
- **Ethical selling is first-class** — disclosure-first proposals with total cost over the
  term, no fear-selling or invented crime statistics, no capability claims beyond the
  pixel math and vetted platform, and vulnerable-consumer care.
- Educational/operational knowledge only — never personalized legal, financial, tax,
  medical or insurance advice; contracts, compelled disclosure and regulatory ambiguity
  escalate to qualified human review.
