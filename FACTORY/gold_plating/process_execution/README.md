# process_execution — Plating Process Control & Execution Specialist

The bench-side specialist that turns a prepped piece and a healthy commercial bath into the
specified deposit: **layer architecture, electrical control, fixturing and thickness — delivered
on target and logged.**

Part of the **gold plating bench program** (`FACTORY/gold_plating/`, six specialists). Built by
the standard FACTORY Path B arc: 3 dense KBs (forged + gated) → 1 distilled specialist (gated).

- Specialist spec: `process_execution.specialist.json`
- Gate report: `process_execution.specialist.validation.json`

## The three knowledge bases

| KB | Domain | What it covers |
|---|---|---|
| `kb1_electrical_control_and_thickness` | Electrical Control & Thickness Delivery | Plateable-area estimation, current density as the master variable and its mass-transport burn ceiling, Faraday time planning corrected by *measured* efficiency, rectifier setup and meter verification inside manufacturer instructions, current distribution with shields / robbers / sanctioned auxiliary anodes, and proving delivered microns by X-ray fluorescence or witness coupon at the significant surfaces. |
| `kb2_layer_architecture_and_underplating` | Layer Architecture & Underplating | Designing the whole stack: strikes for adhesion and activation, barrier underplates (nickel as a compliance-flagged class; palladium and white bronze as nickel-free classes; copper for levelling and ductility), interdiffusion and pore corrosion, flash vs micron-class gold per wear class, duplex systems, interface hold limits, topcoat classes, and the as-built record that settles what may be claimed. |
| `kb3_fixturing_racking_and_run_handling` | Fixturing, Racking & Run Handling | Executing the run: rack vs barrel vs brush route fitness, contact placement and mark hiding, masking classes, fixture certification, spacing / orientation / venting, agitation classes, drag-out and counterflow rinse cascades with conductivity gates, the inter-tank transfer clock, the cyanide–acid segregation line, post-plate rinse-dry-first-touch, and run logging that ties every job to its parameters. |

## Build / gate

```bash
bash FACTORY/build.sh FACTORY/gold_plating/process_execution
```

Forges each `*.spec.json` → `*.kb.json`, gates each KB with `validators/kb_validator.py --mode dense`,
checks the specialist's `grounded_in_kbs`, then gates the specialist. Must end **`ALL GREEN`**.
Gates check structure and math only — never prose quality.

## Using the specialist

Fill the three slots in `dist/prompt_template.json` → `specialist_prompt_template` and send to any
capable model:

- `{{DOMAIN_LABEL}}` ← `Plating Process Control & Execution (heavy: grounded in 3 dense KBs)`
- `{{SPECIALIST_SPEC_JSON}}` ← the entire contents of `process_execution.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the bench question or job

The model then operates strictly by the spec's `role`, `decision_procedure` and `workflow`, honors
every `escalation_trigger`, and answers only inside this domain. No training, no setup.

**Invoke it when** the question is how a gold-plating run is planned, stacked, racked, powered,
executed, measured or defended after the fact. **Route elsewhere** for cleaning/polishing/activation
before the first tank (surface prep), bath make-up and maintenance chemistry, final cosmetic and
adhesion judgement, shop safety programme design, or pricing and selling.

## Boundary summary

- **No invented setpoints.** Current density, temperature, pH, gold concentration and time are
  taught as *mechanisms*; the **technical data sheet of the specific purchased product governs**,
  refined by the shop's own dated logged runs. Handbook-class ranges are cited only *as*
  handbook-class ranges — never as a knob setting.
- **Estimate ≠ claim.** Faraday arithmetic plans a run; only a measurement certifies one. First
  article measured every batch; delivery judged as the **minimum at the named significant surfaces**,
  never an average.
- **Safety is content.** Cyanide-bearing solutions and acids are **never mixed, never stored
  together, never share a drain** — or a rinse, vessel, tool, spill path or transfer route.
  Segregation, storage, labeling, PPE, ventilation and mist control, training and emergency
  readiness are stated as obligation classes with *verify your jurisdiction's workplace and
  hazardous-materials rules*. Formal-training pointers, never improvised procedure.
- **Equipment boundary.** All work stays downstream of manufacturer-sanctioned rectifier
  connections; no mains-side improvisation, and a faulty unit is tagged out of service.
- **Nickel is a compliance class.** Nickel anywhere in a skin-contact stack — strike, barrier,
  palladium-**nickel** alloy, nickel-hardened acid gold — triggers the nickel-release obligation
  class with a dated jurisdiction check; palladium and white bronze are the nickel-free classes.
  No numeric release limit is ever asserted as fact.
- **Commercial products only.** Purchased plating chemistry run per its own data sheet; no DIY bath
  formulation from raw chemicals.
- **Regulated terms are classes.** *gold plated*, *vermeil*, *gold filled*, *rolled gold*, plus waste
  and discharge rules — obligation classes with *verify current rules in your jurisdiction*.
- **Ethical selling first-class.** Plated goods are never represented as solid gold, wear
  expectations are disclosed, heirloom risk is documented, and a claim the as-built record cannot
  support is refused in writing.
- **Never force a piece.** A piece that cannot be racked or contacted without unsanctioned damage
  escalates to intake and the customer decision path with its options documented.
- **Educational only** — never personalized legal, medical or financial advice; nickel-allergy and
  exposure questions route to the regulation classes and to professional care.
- **In every conflict axis where they meet, safety and compliance beat throughput, cost and
  deadline** — and the schedule absorbs the loss.
