# surface_prep — Surface Preparation & Substrate Readiness Specialist

**Mission:** get every piece plate-ready and keep the wrong pieces out of the tanks entirely.
Most plating failures are prep failures, so this specialist owns the step where quality is
actually decided: triage what should never enter the tanks, then deliver a chemically clean,
activated, correctly finished surface. Part of the gold-plating bench program
(`FACTORY/gold_plating/BRIEFS.json`, slug `surface_prep`).

## The 3 knowledge bases

| KB | File | Covers |
|---|---|---|
| KB1 | `kb1_substrate_assessment_and_triage.kb.json` | Verify what the piece actually is (test-method classes, never appearance), map coatings/joints/stones/traps/antique risk, and issue the accept / accept-with-conditions / refuse / refer verdict with sign-offs and the route card. |
| KB2 | `kb2_mechanical_finishing_before_plate.kb.json` | Make the surface the customer will see before the tank: target finish spec, defect map, metal-removal budget, wheel/tumble routing, compound and wheel dedication, detail and hallmark preservation, stop discipline. |
| KB3 | `kb3_chemical_prep_and_rinse_discipline.kb.json` | Polished to plate-ready: soak/ultrasonic degrease, electroclean polarity by mechanism, the water-break gate, activation keyed to the verified substrate class, rinse cascade and water quality, no-touch handling, the transfer clock, and the plate-ready release record. |

## Build (forge + gate everything)

```bash
bash FACTORY/build.sh FACTORY/gold_plating/surface_prep
```

Forges each `*.spec.json` into its `*.kb.json`, gates the KBs (`kb_validator --mode dense`),
then gates `surface_prep.specialist.json` (`specialist_validator`). Must end `ALL GREEN`.

## Use the specialist

Fill `dist/prompt_template.json` (`specialist_prompt_template`):

- `{{DOMAIN_LABEL}}` ← "Surface Preparation & Substrate Readiness (gold-plating bench)"
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `surface_prep.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the bench question or job

Send to any capable model; it operates by the spec's role, decision procedure, workflow,
and escalation triggers.

## Boundary summary

- Stone-set, pearl/organic, antique, filled or repaired pieces are triaged for refusal,
  stone removal or referral **before** any chemistry — never risk a customer piece to keep a job.
- Substrate identity comes from test-method classes (magnet, weight/feel, hallmark reading,
  spot test) — two agreeing methods or the piece is held unknown; never assumed from appearance.
- Prep chemicals are commercial products used per their TDS; no improvised chemistry, no
  unsanctioned mixing; cyanide/acid segregation is absolute where both exist on a line.
- Irreversible mechanical work (polishing through plate, softening detail, touching hallmarks)
  requires explicit documented customer sign-off first.
- Safety and compliance beat throughput, cost, and deadline everywhere they meet; conflicts
  escalate to a human with safety winning. Regulated terms and nickel-release limits are
  compliance classes — verify the jurisdiction; the specialist flags claims, humans approve them.
