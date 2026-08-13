# Recurring-Revenue Business Specialists

One FACTORY vertical per recurring-revenue business model (user brief, 2026-08-12: the
IPTV-box-analogue list — hardware/setup + recurring account). Each business gets its own
specialist built **exactly by the existing FACTORY steps** (`FACTORY/MAKE_A_SPECIALIST.md`,
Path B):

```
FACTORY/recurring_revenue/<slug>/
  kb1_*.spec.json   kb1_*.kb.json     # acquisition & offer economics      (gated dense)
  kb2_*.spec.json   kb2_*.kb.json     # delivery & install/service ops      (gated dense)
  kb3_*.spec.json   kb3_*.kb.json     # recurring account & book management (gated dense)
  <slug>.specialist.json               # the distilled operating spec        (gated 13/13)
```

Build/verify any vertical with the standard one-command gate:

```bash
bash FACTORY/build.sh FACTORY/recurring_revenue/<slug>
```

Use a finished specialist the standard way: drop `<slug>.specialist.json` into
`dist/prompt_template.json` (see `FACTORY/MAKE_A_SPECIALIST.md` §7).

Design source: **`BRIEFS.json`** — the 3-KB decomposition, boundaries and honesty rules per
business. Program-wide content rules (extends `docs/EXPANSION_PLAN.md` §4): no fabricated
firm figures (commissions/rates/multiples are described as *mechanisms* or qualitative
bands); licensing/regulatory points name the obligation class and flag "verify your
jurisdiction"; educational, never personalized financial/legal/tax/medical/insurance advice;
ethical-selling constraints are first-class content. Gates prove structure+math only —
content honesty is enforced by the briefs + a fresh-context verifier pass per vertical.

## Status

| # | slug | Business | Wave | KBs (dense gate) | Specialist | Verified |
|---|------|----------|:---:|:---:|:---:|:---:|
| 1 | `security_dealer` | Home Security & Alarm Dealer | 1 | 3/3 | pass | ✅ |
| 2 | `pos_agent` | Merchant Processing / POS Agent | 1 | 3/3 | pass | ✅ |
| 3 | `voip_reseller` | Business VoIP Reseller | 1 | 3/3 | pass | ✅ |
| 4 | `connectivity_agent` | Business Internet & Wireless Agent | 1 | 3/3 | pass | ✅ |
| 5 | `cctv_cloud` | CCTV & Cloud Video Recording | 1 | 3/3 | pass | ✅ |
| 6 | `fleet_tracking` | GPS / Fleet Tracking Reseller | 2 | 3/3 | pass | ✅ |
| 7 | `managed_wifi` | Managed Wi-Fi Provider | 2 | 3/3 | pass | ✅ |
| 8 | `web_hosting` | Website & Hosting Reseller | 2 | 3/3 | pass | ✅ |
| 9 | `cloud_reseller` | Business Email & Cloud Reseller | 2 | — | — | — |
| 10 | `managed_cyber` | Managed Cybersecurity Services | 2 | — | — | — |
| 11 | `life_insurance` | Life Insurance Agent | 3 | — | — | — |
| 12 | `pc_insurance` | P&C Insurance Agent | 3 | — | — | — |
| 13 | `medicare_agent` | Health / Medicare Insurance Agent | 3 | — | — | — |
| 14 | `medical_alert` | Medical Alert Systems | 3 | — | — | — |
| 15 | `home_warranty` | Home Warranty / Service Plan Sales | 3 | — | — | — |
| 16 | `payroll_services` | Payroll Service / Referral | 4 | — | — | — |
| 17 | `bookkeeping_sub` | Subscription Bookkeeping | 4 | — | — | — |
| 18 | `tax_prep` | Tax Preparation Practice | 4 | — | — | — |
| 19 | `cleaning_broker` | Commercial Cleaning Broker | 4 | — | — | — |
| 20 | `pest_control` | Pest Control Sales / Referral | 4 | — | — | — |
| 21 | `water_filter` | Water Filtration Systems | 5 | — | — | — |
| 22 | `copier_leasing` | Printer / Copier Leasing | 5 | — | — | — |
| 23 | `office_coffee` | Office Coffee & Water Service | 5 | — | — | — |
| 24 | `travel_agent` | Travel Agent / Advisory | 5 | — | — | — |
| 25 | `solar_referral` | Solar Sales / Referral | 5 | — | — | — |

Wave 1 landed 2026-08-12 (all five verified). Wave 1 = the five the user flagged as deserving particular attention. Statuses flip to
`3/3` / `13/13` / `✅` as each vertical lands; each business is committed individually.
