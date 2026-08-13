# Use These Specialists — hand one business to any AI and build it end to end

Every business below ships as **one specialist spec + 3 dense knowledge bases**, all machine-facing
JSON. You do not need this repo, Python, or any tooling to use them: paste the specialist JSON into
any capable model and it reasons *as* that business operator. The KBs are the deep reference the
specialist was distilled from — load one when you need the full node/edge/workflow graph for a phase.

**Repo:** https://github.com/tmundi3210/automation · **Branch:** `claude/eager-wozniak-74rlgj`
**Folder:** https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue

The repo is **public**, so every `raw` link below is directly fetchable by any AI or `curl` with no
token. Shorter equivalent: swap the branch for `HEAD`, e.g.
`https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/pos_agent/pos_agent.specialist.json`

---

## 1. The 60-second version

1. Pick your business in the table below and open its **specialist** raw link.
2. Copy the **entire** JSON.
3. Paste the operator prompt from §3 into a fresh chat with any strong model, with that JSON in the
   `<specialist_spec>` slot.
4. Ask it to run the end-to-end build (§4 gives the exact sequence of asks).

That is the whole mechanism. No fine-tuning, no install — the intelligence is in the spec.

---

## 2. The catalog

Each row: the money model, then the specialist (paste this) and its 3 KBs (deep reference).
`raw` links give plain JSON an AI can fetch or you can copy; `view` links are browsable on GitHub.

### Home Security & Alarm Dealer  ·  `security_dealer`

*How it makes money:* sell/install monitored security systems (alarm, doorbell, cameras); revenue = equipment + installation fee + recurring monitoring/support account

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/security_dealer/security_dealer.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/security_dealer/security_dealer.specialist.json)
- KB1 — Alarm Dealer Programs & Account-Creation Economics — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/security_dealer/kb1_dealer_program_and_account_economics.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/security_dealer/kb1_dealer_program_and_account_economics.kb.json)
- KB2 — Security System Design & Installation Operations — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/security_dealer/kb2_system_design_and_installation.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/security_dealer/kb2_system_design_and_installation.kb.json)
- KB3 — Monitoring, RMR & Account-Book Retention — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/security_dealer/kb3_monitoring_rmr_and_retention.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/security_dealer/kb3_monitoring_rmr_and_retention.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/security_dealer)

*Hard boundaries this specialist enforces:*
  - defensive, lawful security only - systems exist to prevent, detect and alarm intrusions
  - low-voltage/alarm licensing and permit rules vary by state and city - flag 'verify locally', never assert specifics
  - alarm contracts are consumer-protection sensitive: auto-renewal, term and cancellation terms must be disclosed honestly
  - dealer-program economics (subsidies, account multiples, holdbacks) described as mechanisms, never invented figures

### Merchant Processing / POS Agent  ·  `pos_agent`

*How it makes money:* acquire merchants (restaurants, salons, c-stores, mechanics); place/arrange POS and card acceptance; revenue = POS sale/lease + share of processing economics; the durable asset is the book of merchants

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/pos_agent/pos_agent.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pos_agent/pos_agent.specialist.json)
- KB1 — Merchant Acquisition & Vertical Targeting — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/pos_agent/kb1_merchant_acquisition_and_vertical_targeting.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pos_agent/kb1_merchant_acquisition_and_vertical_targeting.kb.json)
- KB2 — POS Deployment & Merchant Onboarding — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/pos_agent/kb2_pos_deployment_and_onboarding.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pos_agent/kb2_pos_deployment_and_onboarding.kb.json)
- KB3 — Residual Book & Merchant-Portfolio Management — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/pos_agent/kb3_residual_book_and_portfolio_management.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pos_agent/kb3_residual_book_and_portfolio_management.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pos_agent)

*Hard boundaries this specialist enforces:*
  - no deceptive rate-comparison or 'we'll definitely save you X%' claims - statement analysis is a diagnostic mechanism, savings are verified per merchant
  - interchange, markup and residual splits described as mechanisms, never invented percentages stated as fact
  - card-network rules, PCI-DSS and surcharging law are compliance classes to flag, jurisdiction- and network-specific
  - not financial or legal advice; merchant underwriting decisions belong to the processor

### Business VoIP Reseller  ·  `voip_reseller`

*How it makes money:* sell/configure business phone systems, port numbers, provide monthly service; revenue = hardware/setup + installation + monthly per-seat service + ongoing support; targets: dentists, trucking, restaurants, motels, small offices

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/voip_reseller/voip_reseller.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/voip_reseller/voip_reseller.specialist.json)
- KB1 — VoIP Channel Economics & Target Segments — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/voip_reseller/kb1_voip_channel_economics_and_segments.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/voip_reseller/kb1_voip_channel_economics_and_segments.kb.json)
- KB2 — VoIP Deployment, Number Porting & Call Quality — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/voip_reseller/kb2_deployment_porting_and_qos.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/voip_reseller/kb2_deployment_porting_and_qos.kb.json)
- KB3 — VoIP Recurring Service, Support & Expansion — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/voip_reseller/kb3_recurring_service_support_and_expansion.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/voip_reseller/kb3_recurring_service_support_and_expansion.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/voip_reseller)

*Hard boundaries this specialist enforces:*
  - E911 obligations (Kari's Law / RAY BAUM class of rules) are a hard compliance flag on every deployment - verify current requirements, never skip
  - telecom regulatory/tax treatment (USF-class obligations) varies by model (agent vs reseller) - flag for professional verification
  - agent/reseller/white-label economics described as mechanisms, never invented commission figures
  - no guaranteed-uptime or savings claims without a verified basis

### Business Internet & Wireless Agent  ·  `connectivity_agent`

*How it makes money:* sell business internet, backup 5G/wireless and related services through carrier agent/partner programs; revenue = activation + ongoing/account-based compensation; strongest when bundled (internet + backup + Wi-Fi + phones + cameras)

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/connectivity_agent/connectivity_agent.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/connectivity_agent/connectivity_agent.specialist.json)
- KB1 — Carrier Partner Programs & Bundle Strategy — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/connectivity_agent/kb1_carrier_programs_and_bundle_strategy.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/connectivity_agent/kb1_carrier_programs_and_bundle_strategy.kb.json)
- KB2 — Site Qualification & Service Activation — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/connectivity_agent/kb2_site_qualification_and_activation.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/connectivity_agent/kb2_site_qualification_and_activation.kb.json)
- KB3 — Account Compensation & Lifecycle Management — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/connectivity_agent/kb3_account_compensation_and_lifecycle.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/connectivity_agent/kb3_account_compensation_and_lifecycle.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/connectivity_agent)

*Hard boundaries this specialist enforces:*
  - carrier partner-program terms, compensation plans and product availability change - always verify the current program before any commitment
  - compensation structures described as mechanisms (activation vs residual vs account-based), never invented figures
  - serviceability is address-specific fact-finding, never assumed
  - no misrepresentation of carrier affiliation; the agent sells the carrier's service under the program's rules

### CCTV & Cloud Video Recording  ·  `cctv_cloud`

*How it makes money:* own-build vertical (not a major-company agent): camera installation + cloud recording/support subscription; revenue = install + monthly cloud storage/monitoring/support per site

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/cctv_cloud/cctv_cloud.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cctv_cloud/cctv_cloud.specialist.json)
- KB1 — Surveillance Offer Design & Subscription Pricing — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/cctv_cloud/kb1_surveillance_offer_design_and_pricing.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cctv_cloud/kb1_surveillance_offer_design_and_pricing.kb.json)
- KB2 — Camera System Design & Installation — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/cctv_cloud/kb2_camera_system_design_and_installation.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cctv_cloud/kb2_camera_system_design_and_installation.kb.json)
- KB3 — Cloud Video Operations & Privacy Compliance — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/cctv_cloud/kb3_cloud_video_ops_and_privacy_compliance.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cctv_cloud/kb3_cloud_video_ops_and_privacy_compliance.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cctv_cloud)

*Hard boundaries this specialist enforces:*
  - lawful, disclosed surveillance only - no covert or unlawful monitoring; purpose is deterrence, detection and evidence for the property owner
  - video privacy, audio-recording consent and camera-placement law vary sharply by jurisdiction - flag 'verify locally' on every design
  - retention and access policy is a first-class design decision, not an afterthought
  - VSaaS platform vs own-stack economics as mechanisms; no invented per-camera price points stated as fact

### GPS / Fleet Tracking Reseller  ·  `fleet_tracking`

*How it makes money:* sell/install GPS trackers to fleets and equipment owners; revenue = tracker hardware + monthly tracking-platform subscription per unit

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/fleet_tracking/fleet_tracking.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/fleet_tracking/fleet_tracking.specialist.json)
- KB1 — Fleet-Tracking Market & Reseller Economics — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/fleet_tracking/kb1_fleet_market_and_reseller_economics.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/fleet_tracking/kb1_fleet_market_and_reseller_economics.kb.json)
- KB2 — Tracker Installation & Platform Onboarding — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/fleet_tracking/kb2_tracker_install_and_platform_onboarding.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/fleet_tracking/kb2_tracker_install_and_platform_onboarding.kb.json)
- KB3 — Tracking Subscriptions & Fleet-Account Growth — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/fleet_tracking/kb3_tracking_subscription_and_account_growth.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/fleet_tracking/kb3_tracking_subscription_and_account_growth.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/fleet_tracking)

*Hard boundaries this specialist enforces:*
  - lawful tracking only: owner/employer-authorized vehicles and assets, with required employee disclosure - covert tracking of private individuals is out of scope
  - ELD/hours-of-service and telematics-privacy rules are jurisdiction- and fleet-type-specific - flag for verification
  - platform reseller economics as mechanisms, never invented per-unit figures

### Managed Wi-Fi Provider  ·  `managed_wifi`

*How it makes money:* design/install business Wi-Fi (router/APs) + monthly management, monitoring and support subscription; venues: restaurants, motels, offices, retail, multi-tenant

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/managed_wifi/managed_wifi.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/managed_wifi/managed_wifi.specialist.json)
- KB1 — Managed Wi-Fi Offer & Target Venues — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/managed_wifi/kb1_managed_wifi_offer_and_target_venues.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/managed_wifi/kb1_managed_wifi_offer_and_target_venues.kb.json)
- KB2 — Wi-Fi Design, Deployment & Handoff — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/managed_wifi/kb2_wifi_design_deployment_and_handoff.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/managed_wifi/kb2_wifi_design_deployment_and_handoff.kb.json)
- KB3 — Wi-Fi Monitoring, Management & SLA Operations — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/managed_wifi/kb3_monitoring_management_and_sla_operations.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/managed_wifi/kb3_monitoring_management_and_sla_operations.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/managed_wifi)

*Hard boundaries this specialist enforces:*
  - guest-network privacy: no traffic interception or sale of user data; captive-portal data collection must be disclosed
  - managed-service pricing as mechanism (per-AP / per-site tiers), never invented figures
  - spectrum/wiring and code constraints are site-specific facts to survey, not assumptions

### Website & Hosting Reseller  ·  `web_hosting`

*How it makes money:* build small-business websites + recurring hosting/maintenance/care plans; revenue = site build fee + monthly hosting, updates, backups, edits

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/web_hosting/web_hosting.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/web_hosting/web_hosting.specialist.json)
- KB1 — Productized Web Offer & Client Acquisition — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/web_hosting/kb1_productized_web_offer_and_acquisition.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/web_hosting/kb1_productized_web_offer_and_acquisition.kb.json)
- KB2 — Site Build Stack & Delivery Process — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/web_hosting/kb2_site_build_stack_and_delivery.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/web_hosting/kb2_site_build_stack_and_delivery.kb.json)
- KB3 — Hosting, Care Plans & Client Retention — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/web_hosting/kb3_hosting_care_plans_and_retention.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/web_hosting/kb3_hosting_care_plans_and_retention.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/web_hosting)

*Hard boundaries this specialist enforces:*
  - client owns their domain and content - lock-in through service quality, never through hostage-holding of assets
  - care-plan pricing as mechanism/tiers, never invented market rates
  - no SEO/traffic guarantees; performance claims must be verifiable

### Business Email & Cloud Reseller  ·  `cloud_reseller`

*How it makes money:* set up and manage Microsoft 365 / Google Workspace and adjacent cloud subscriptions for small businesses; revenue = setup/migration fees + monthly per-seat margin/management

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/cloud_reseller/cloud_reseller.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cloud_reseller/cloud_reseller.specialist.json)
- KB1 — Cloud Reseller Programs & SMB Targeting — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/cloud_reseller/kb1_cloud_reseller_programs_and_smb_targeting.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cloud_reseller/kb1_cloud_reseller_programs_and_smb_targeting.kb.json)
- KB2 — Tenant Setup, Migration & Onboarding — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/cloud_reseller/kb2_tenant_migration_and_onboarding.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cloud_reseller/kb2_tenant_migration_and_onboarding.kb.json)
- KB3 — Subscription Management, Support & Expansion — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/cloud_reseller/kb3_subscription_management_and_expansion.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cloud_reseller/kb3_subscription_management_and_expansion.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cloud_reseller)

*Hard boundaries this specialist enforces:*
  - client data handled under least-privilege and documented authorization; admin access is a fiduciary-grade responsibility
  - partner-program (CSP-class) margin structures as mechanisms - terms change, verify current program
  - no fear-based upselling of licenses the client does not need

### Managed Cybersecurity Services  ·  `managed_cyber`

*How it makes money:* package security stack (endpoint protection, patching, backup, awareness, monitoring) for small businesses; revenue = setup/hardening project + monthly per-seat/per-site subscription

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/managed_cyber/managed_cyber.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/managed_cyber/managed_cyber.specialist.json)
- KB1 — Security Service Packaging & SMB Sales — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/managed_cyber/kb1_security_service_packaging_and_smb_sales.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/managed_cyber/kb1_security_service_packaging_and_smb_sales.kb.json)
- KB2 — Stack Deployment & Baseline Hardening — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/managed_cyber/kb2_stack_deployment_and_baseline_hardening.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/managed_cyber/kb2_stack_deployment_and_baseline_hardening.kb.json)
- KB3 — Monitoring, Response & Compliance Reporting — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/managed_cyber/kb3_monitoring_response_and_compliance_reporting.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/managed_cyber/kb3_monitoring_response_and_compliance_reporting.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/managed_cyber)

*Hard boundaries this specialist enforces:*
  - strictly defensive scope: protect, detect, respond for the authorized client - no offensive services, no unauthorized testing
  - written authorization and scope definition precede any security work
  - no absolute-protection claims - security reduces risk, never eliminates it; honest incident communication
  - compliance framework mapping is guidance, not certified audit or legal advice

### Life Insurance Agent  ·  `life_insurance`

*How it makes money:* licensed agent selling life products; revenue = policy commissions + renewal/trail compensation depending on product and carrier; the durable asset is a persistent, well-served book

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/life_insurance/life_insurance.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/life_insurance/life_insurance.specialist.json)
- KB1 — Licensing, Carrier Selection & Market Focus — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/life_insurance/kb1_licensing_carriers_and_market_focus.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/life_insurance/kb1_licensing_carriers_and_market_focus.kb.json)
- KB2 — Needs Analysis, Product Fit & Suitability — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/life_insurance/kb2_needs_analysis_product_fit_and_suitability.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/life_insurance/kb2_needs_analysis_product_fit_and_suitability.kb.json)
- KB3 — Persistency, Renewals & Book Building — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/life_insurance/kb3_persistency_renewals_and_book_building.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/life_insurance/kb3_persistency_renewals_and_book_building.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/life_insurance)

*Hard boundaries this specialist enforces:*
  - state insurance licensing is mandatory before any solicitation - jurisdiction-specific, verify current requirements
  - educational and process knowledge, not personalized financial or insurance advice; suitability rules govern recommendations
  - no performance promises on cash-value products; illustrations are projections, not guarantees
  - replacement regulations and disclosure duties are compliance classes to flag; commission structures as mechanisms, never invented percentages

### Property & Casualty Insurance Agent  ·  `pc_insurance`

*How it makes money:* licensed P&C agent selling auto/home/business policies; revenue = new-business + renewal commissions; renewal book is the asset

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/pc_insurance/pc_insurance.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pc_insurance/pc_insurance.specialist.json)
- KB1 — P&C Licensing, Appointments & Agency Model — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/pc_insurance/kb1_licensing_appointments_and_agency_model.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pc_insurance/kb1_licensing_appointments_and_agency_model.kb.json)
- KB2 — Quoting, Binding & Coverage Design — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/pc_insurance/kb2_quoting_binding_and_coverage_design.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pc_insurance/kb2_quoting_binding_and_coverage_design.kb.json)
- KB3 — Renewals, Retention & Agency Book Value — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/pc_insurance/kb3_renewals_retention_and_agency_book_value.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pc_insurance/kb3_renewals_retention_and_agency_book_value.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pc_insurance)

*Hard boundaries this specialist enforces:*
  - P&C licensing is mandatory and state-specific - verify current requirements
  - educational, not personalized insurance advice; coverage recommendations follow documented needs analysis
  - no premium-savings guarantees; quotes are carrier-determined
  - commission and contingency structures as mechanisms, never invented figures; E&O exposure is a first-class risk to manage

### Health / Medicare Insurance Agent  ·  `medicare_agent`

*How it makes money:* licensed, certified agent enrolling clients in Medicare-related and health plans; revenue = enrollment compensation + renewals under regulated schedules

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/medicare_agent/medicare_agent.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/medicare_agent/medicare_agent.specialist.json)
- KB1 — Certifications, Contracts & Compliant Marketing — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/medicare_agent/kb1_certifications_contracts_and_compliant_marketing.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/medicare_agent/kb1_certifications_contracts_and_compliant_marketing.kb.json)
- KB2 — Enrollment Periods, Plan Comparison & Needs Fit — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/medicare_agent/kb2_enrollment_periods_and_plan_fit.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/medicare_agent/kb2_enrollment_periods_and_plan_fit.kb.json)
- KB3 — Renewal Compensation, Service & Compliant Retention — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/medicare_agent/kb3_renewal_compensation_service_and_retention.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/medicare_agent/kb3_renewal_compensation_service_and_retention.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/medicare_agent)

*Hard boundaries this specialist enforces:*
  - the most regulated vertical in this set: CMS marketing/communication rules, annual certification (AHIP-class) and carrier certifications are hard prerequisites - compliance dominates growth in every conflict
  - educational, not personalized plan advice; enrollment decisions belong to the beneficiary
  - no steering by compensation; plan comparison must be need-driven and documented
  - compensation is set by regulated schedules - describe as mechanism, verify current-year rules; senior clients are a protected/vulnerable population - ethical-contact rules are first-class

### Medical Alert Systems  ·  `medical_alert`

*How it makes money:* sell/set up personal emergency response devices for seniors; revenue = device + monthly monitoring subscription

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/medical_alert/medical_alert.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/medical_alert/medical_alert.specialist.json)
- KB1 — Medical-Alert Market & Ethical Senior Sales — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/medical_alert/kb1_market_and_ethical_senior_sales.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/medical_alert/kb1_market_and_ethical_senior_sales.kb.json)
- KB2 — Device Setup, Monitoring Integration & Testing — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/medical_alert/kb2_device_setup_monitoring_and_testing.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/medical_alert/kb2_device_setup_monitoring_and_testing.kb.json)
- KB3 — Monitoring Service, Retention & Caregiver Relations — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/medical_alert/kb3_monitoring_retention_and_caregiver_relations.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/medical_alert/kb3_monitoring_retention_and_caregiver_relations.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/medical_alert)

*Hard boundaries this specialist enforces:*
  - not medical advice or a medical device consultation - it is emergency-signaling equipment plus monitoring service
  - seniors and their families are a vulnerable-consumer population: transparent pricing, easy cancellation, no scare-selling - these are first-class rules
  - reliability claims (battery, range, response time) must be device-verified, not asserted; testing protocol is mandatory at setup
  - monitoring economics as mechanisms, never invented figures

### Home Warranty / Service Plan Sales  ·  `home_warranty`

*How it makes money:* sell home service contracts/warranty plans; revenue = contract commissions + renewals

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/home_warranty/home_warranty.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/home_warranty/home_warranty.specialist.json)
- KB1 — Service-Contract Products & Distribution Channels — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/home_warranty/kb1_products_and_distribution_channels.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/home_warranty/kb1_products_and_distribution_channels.kb.json)
- KB2 — Coverage Terms, Exclusions & Honest Disclosure — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/home_warranty/kb2_coverage_terms_and_honest_disclosure.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/home_warranty/kb2_coverage_terms_and_honest_disclosure.kb.json)
- KB3 — Renewals, Claims Experience & Reputation Management — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/home_warranty/kb3_renewals_claims_experience_and_reputation.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/home_warranty/kb3_renewals_claims_experience_and_reputation.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/home_warranty)

*Hard boundaries this specialist enforces:*
  - service contracts are state-regulated products (registration/obligor rules vary) - verify jurisdiction before selling
  - exclusions, caps and claim conditions MUST be surfaced before sale - burying them is the vertical's core ethical failure
  - the agent sells a defined contract, not a promise that everything is covered; claims outcomes belong to the obligor
  - commissions as mechanisms, never invented figures

### Payroll Service / Referral Business  ·  `payroll_services`

*How it makes money:* acquire small-business payroll clients (referral partnership with a payroll platform, or run a service practice on top of one); revenue = referral/revenue-share + recurring per-client relationship

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/payroll_services/payroll_services.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/payroll_services/payroll_services.specialist.json)
- KB1 — Payroll Referral Models & Client Acquisition — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/payroll_services/kb1_referral_models_and_client_acquisition.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/payroll_services/kb1_referral_models_and_client_acquisition.kb.json)
- KB2 — Client Onboarding, Data Migration & Setup — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/payroll_services/kb2_client_onboarding_and_setup.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/payroll_services/kb2_client_onboarding_and_setup.kb.json)
- KB3 — Recurring Relationship, Compliance Calendar & Expansion — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/payroll_services/kb3_recurring_relationship_and_expansion.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/payroll_services/kb3_recurring_relationship_and_expansion.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/payroll_services)

*Hard boundaries this specialist enforces:*
  - payroll tax filing is a compliance-critical professional domain - errors carry penalties; not tax or legal advice
  - referral/revenue-share structures as mechanisms - verify current partner-program terms
  - client data (SSNs, wages, banking) demands strict confidentiality and least-privilege handling

### Subscription Bookkeeping Practice  ·  `bookkeeping_sub`

*How it makes money:* monthly bookkeeping/accounting packages for small businesses; revenue = fixed monthly subscription per client tier

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/bookkeeping_sub/bookkeeping_sub.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/bookkeeping_sub/bookkeeping_sub.specialist.json)
- KB1 — Productized Bookkeeping Offer & Niche Selection — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/bookkeeping_sub/kb1_productized_offer_and_niche_selection.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/bookkeeping_sub/kb1_productized_offer_and_niche_selection.kb.json)
- KB2 — Client Onboarding, Cleanup & Monthly Close — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/bookkeeping_sub/kb2_onboarding_cleanup_and_monthly_close.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/bookkeeping_sub/kb2_onboarding_cleanup_and_monthly_close.kb.json)
- KB3 — Subscription Pricing, Capacity & Client Retention — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/bookkeeping_sub/kb3_subscription_pricing_capacity_and_retention.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/bookkeeping_sub/kb3_subscription_pricing_capacity_and_retention.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/bookkeeping_sub)

*Hard boundaries this specialist enforces:*
  - bookkeeping, not attest/audit work and not tax or legal advice; know where licensed-CPA territory begins - flag it
  - fixed-fee tiers as pricing mechanisms; no invented market rates
  - client financial data confidentiality and access discipline are first-class

### Tax Preparation Practice  ·  `tax_prep`

*How it makes money:* per-return fees with an annually returning client base; seasonal peak with off-season extension/amendment/planning work

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/tax_prep/tax_prep.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/tax_prep/tax_prep.specialist.json)
- KB1 — Practice Setup, Credentials & Seasonal Demand — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/tax_prep/kb1_practice_setup_credentials_and_demand.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/tax_prep/kb1_practice_setup_credentials_and_demand.kb.json)
- KB2 — Return Workflow, Accuracy & Documentation — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/tax_prep/kb2_return_workflow_accuracy_and_documentation.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/tax_prep/kb2_return_workflow_accuracy_and_documentation.kb.json)
- KB3 — Returning Clients & Off-Season Revenue — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/tax_prep/kb3_returning_clients_and_offseason_revenue.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/tax_prep/kb3_returning_clients_and_offseason_revenue.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/tax_prep)

*Hard boundaries this specialist enforces:*
  - preparer registration/credential classes (PTIN/EFIN-class requirements, state rules) are prerequisites - verify current rules
  - process and practice knowledge, not tax advice for specific situations; accuracy and penalty exposure make verification non-negotiable
  - no refund-size promises or aggressive-position selling; due-diligence obligations on credits are compliance classes to respect

### Commercial Cleaning Broker  ·  `cleaning_broker`

*How it makes money:* win commercial cleaning contracts, arrange service through crews/subcontractors, retain the monthly margin

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/cleaning_broker/cleaning_broker.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cleaning_broker/cleaning_broker.specialist.json)
- KB1 — Contract Acquisition & Bid Strategy — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/cleaning_broker/kb1_contract_acquisition_and_bid_strategy.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cleaning_broker/kb1_contract_acquisition_and_bid_strategy.kb.json)
- KB2 — Crew Sourcing, Quality Control & Walkthroughs — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/cleaning_broker/kb2_crew_sourcing_and_quality_control.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cleaning_broker/kb2_crew_sourcing_and_quality_control.kb.json)
- KB3 — Margin Management, Client Retention & Renewals — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/cleaning_broker/kb3_margin_retention_and_renewals.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cleaning_broker/kb3_margin_retention_and_renewals.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/cleaning_broker)

*Hard boundaries this specialist enforces:*
  - employee-vs-independent-contractor classification is a legal risk class that varies by jurisdiction - flag for professional verification, never assume
  - honest broker positioning: the client knows the service model; no misrepresentation of who cleans
  - margin structures as mechanisms, never invented figures; insurance/bonding requirement classes verified per contract

### Pest Control Sales / Referral Business  ·  `pest_control`

*How it makes money:* sell recurring pest-control service accounts (own licensed operation or commission/referral partnership with licensed operators); revenue = new-account commissions + recurring contract value

- **SPECIALIST (paste this one)** — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/pest_control/pest_control.specialist.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pest_control/pest_control.specialist.json)
- KB1 — Pest Account Sales & Route-Density Economics — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/pest_control/kb1_account_sales_and_route_density.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pest_control/kb1_account_sales_and_route_density.kb.json)
- KB2 — Service Delivery Partnering & Licensing Awareness — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/pest_control/kb2_service_delivery_and_licensing_awareness.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pest_control/kb2_service_delivery_and_licensing_awareness.kb.json)
- KB3 — Recurring Contracts, Retention & Seasonal Upsells — [raw](https://raw.githubusercontent.com/tmundi3210/automation/HEAD/FACTORY/recurring_revenue/pest_control/kb3_recurring_contracts_and_seasonal_upsells.kb.json) · [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pest_control/kb3_recurring_contracts_and_seasonal_upsells.kb.json)
- Folder + README — [view](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/pest_control)

*Hard boundaries this specialist enforces:*
  - pesticide application requires operator/applicator licensing - jurisdiction-specific and non-negotiable; unlicensed application is out of scope
  - door-to-door and neighborhood sales are regulated (permits, do-not-knock, cooling-off rules) - flag and respect
  - no infestation scare-selling; findings must be shown, not invented
  - commission structures as mechanisms, never invented figures
---

## 3. The operator prompt (paste-ready)

Use this verbatim. Replace the two slots. This mirrors `dist/prompt_template.json` in the repo.

```
SYSTEM:
You are {{DOMAIN_LABEL}}, an expert specialist. Operate STRICTLY according to the machine-facing
specialist spec provided in <specialist_spec>. Follow its `role`, `decision_procedure`, and
`workflow`; honor every `escalation_triggers` and `conflicts_and_dominance` rule; respect all
scope/safety boundaries stated in its `purpose`/`role` and `boundaries`. Ground every claim.
Label estimates as estimates and name the method behind them. Never invent commission rates,
prices, market sizes, or regulatory specifics — describe the MECHANISM and tell me exactly what
to verify and where. Escalate irreversible or high-risk decisions to human review.
Answer ONLY within this specialist's domain.

CONTEXT:
<specialist_spec>
{{PASTE THE ENTIRE SPECIALIST JSON HERE}}
</specialist_spec>

USER:
{{YOUR QUESTION OR THE BUILD STEP FROM SECTION 4}}
```

`{{DOMAIN_LABEL}}` is the `domain_label` field inside the JSON you pasted.

---

## 4. Building one business end to end

Every specialist is built on the same three-phase arc, so the same sequence works for all 25:

```
KB1 acquisition & offer economics -> KB2 delivery & installation/service ops -> KB3 recurring account & book
```

Run these as separate turns in the same chat, in order. Each one names the artifact you should
end up holding — if the model gives you prose instead of the artifact, ask again for the artifact.

**Phase 0 — orient (1 turn)**
> Summarize your own `role`, `boundaries`, and `decision_procedure` in plain language. Then list the
> 8-12 decisions I must make to stand this business up, in dependency order, and mark which ones are
> irreversible or licence-gated. Output: a decision list.

**Phase 1 — model & offer (KB1 territory)**
> Using your acquisition/offer-economics capabilities: define my target segment, the offer
> architecture, and the unit economics AS MECHANISMS (what each number is made of, what moves it,
> and exactly how I verify it locally). Name the programs/partners I must qualify with and what
> their agreements govern. Output: a one-page business model + a verification checklist of every
> number and rule I must confirm before committing.

**Phase 2 — legal, licensing & compliance gate**
> List every licence, registration, permit, certification, insurance and disclosure obligation CLASS
> that applies, what triggers each, and the authority I check for current rules. Flag anything that
> must be in place BEFORE I sell or deliver anything. Output: a compliance gate checklist.
> (Do not proceed past this gate until each item is confirmed for your jurisdiction.)

**Phase 3 — go-to-market (KB1 territory)**
> Design my first-100-prospects plan: sourcing, qualification, the discovery conversation, the
> proposal, and the ethical-selling constraints from your boundaries. Include the exact questions to
> ask a prospect and the disclosures I owe them. Output: a sales playbook + call/visit script skeleton.

**Phase 4 — delivery operations (KB2 territory)**
> Give me the repeatable delivery runbook: site survey / needs assessment, design or configuration
> decisions and their trade-offs, the installation or onboarding steps in order, the acceptance tests
> that prove it was done right, and the documentation I hand the customer. Output: a delivery runbook
> with a per-job checklist.

**Phase 5 — the recurring engine (KB3 territory)**
> Design the recurring side: what exactly recurs, how it is billed, the service cadence that earns it,
> the churn signals and the retention playbook, contract hygiene and honest renewal practice, and what
> makes the account book itself valuable. Output: an operations calendar + retention playbook.

**Phase 6 — risk & failure modes**
> Walk your `escalation_triggers` and `conflicts_and_dominance`: what most often kills operators in
> this business, what tradeoffs I will face, which side wins in each, and when I must stop and get a
> professional. Output: a risk register with triggers and responses.

**Phase 7 — the 90-day plan**
> Compress everything above into a sequenced 90-day plan with weekly milestones, the gate that must
> pass before each next step, and the artifacts I should be holding at day 30 / 60 / 90.
> Output: the plan.

### Going deeper on any phase

When a phase needs more depth than the specialist carries, load that phase's KB (the raw links in §2)
into a fresh chat and ask:

> This is a dense knowledge-base graph for <subdomain>. Use its `nodes` (units of expert work),
> `edges` (dependencies and conflicts), `workflow` (the anti-rework order) and `competency_questions`
> to give me the deep version of <the phase>. Follow the workflow order and tell me the gate for each step.

The KBs are large (~200KB each) — use a long-context model, or ask for one section at a time.

---

## 5. Bundling two or more businesses

The strongest play in this set is the bundle: one small-business customer becomes several recurring
streams. POS + phones + internet + Wi-Fi + cameras + security all sell to the same owner, and each
specialist knows the cross-sell surface from its own side.

To plan a bundle, run this in a fresh chat with TWO OR MORE specialist JSONs pasted in sequence:

> Here are N specialist specs. For a single small-business customer (describe the type), design the
> combined offer: which service leads, the order to attach the others, what each adds to monthly
> recurring revenue as a mechanism, the delivery sequence that avoids rework, and every compliance
> obligation that carries across all of them. Respect every specialist's boundaries — where two
> conflict, the stricter one wins.

---

## 6. What these are — and what they are not

- **They are** operating specs: role, decision procedure, workflow, escalation triggers, validation
  checklists, and the conflict/dominance rules of each business, distilled from 3 dense KBs each.
- **They deliberately contain no invented numbers.** No commission percentages, price points, or
  market sizes are stated as fact. Every economic quantity is described as a mechanism plus how to
  verify it for your market. This is by design — a confident fake number is worse than no number.
- **They are educational and operational, not advice.** Nothing here is legal, tax, financial,
  medical, or insurance advice. Licensing and regulatory content names the obligation CLASS and
  tells you to verify current rules in your jurisdiction, because those rules change and vary.
- **The deterministic gates prove structure and math only** — schema, reference integrity, count
  bands, and every derived score recomputed. They do not judge whether prose is true. Content
  honesty comes from the authoring rules plus an independent fresh-context verification pass per
  vertical, not from the exit code.

---

## 7. If you want to build more of these

The full build manual is [FACTORY/MAKE_A_SPECIALIST.md](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/MAKE_A_SPECIALIST.md) —
it teaches the whole pipeline (3 dense KBs -> gate -> distill the specialist -> gate) and the
one-command build: `bash FACTORY/build.sh <folder>`. The per-business design briefs for all 25
live in [BRIEFS.json](https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/recurring_revenue/BRIEFS.json).

