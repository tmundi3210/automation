# Gold Plating Bench Specialists

The specialist **team** a gold-electroplating jeweler runs — the bench/production side,
complementing `FACTORY/plated_jewelry/` (the market side: assortment, economics, polling).
Each specialist is built by the standard FACTORY pipeline (`FACTORY/MAKE_A_SPECIALIST.md`,
Path B): 3 dense KBs (forged + gated `--mode dense`) → 1 distilled specialist (gated) →
fresh-context adversarial verify. Design source: **`BRIEFS.json`** (missions, boundaries,
KB scopes, honesty rules, and per-KB grounding into the in-repo process corpus
`FACTORY/plated_jewelry/processes/*.json`).

```
FACTORY/gold_plating/<slug>/
  kb1_*.spec.json  kb1_*.kb.json      # gated dense
  kb2_*.spec.json  kb2_*.kb.json      # gated dense
  kb3_*.spec.json  kb3_*.kb.json      # gated dense
  <slug>.specialist.json              # distilled operating spec, gated
```

Build/verify any specialist: `bash FACTORY/build.sh FACTORY/gold_plating/<slug>`
Use one: drop `<slug>.specialist.json` into `dist/prompt_template.json` (manual §7).

## The team and how a job flows through it

A plating job should consult the specialists in this order — safety/compliance is the
frozen gate before anything irreversible (think → decide split, as in
`branches/b60_content_intelligence/BRAIN_STEP1.md` + `GATE_STEP2.md`):

```
plating_business (intake, quote, risk sign-off)
   → surface_prep        (triage, mechanical finish, chemical prep)
   → bath_chemistry      (bath selection + health)
   → process_execution   (stack design, electrical control, fixturing)
   → quality_finish      (color, thickness, adhesion, defect diagnosis)
   ⇢ safety_compliance   (GATE: consulted before any chemistry, any waste
                          leaving the shop, and any marking/claim — its verdict wins)
```

## Status

| # | slug | Specialist | KBs (dense gate) | Specialist gate | Verified |
|---|------|-----------|:---:|:---:|:---:|
| 1 | `surface_prep` | Surface Preparation & Substrate Readiness | 3/3 | pass | ✅ |
| 2 | `bath_chemistry` | Gold Bath Chemistry & Solution Management | building | – | – |
| 3 | `process_execution` | Plating Process Control & Execution | building | – | – |
| 4 | `quality_finish` | Finish Quality, Color & Defect Diagnosis | building | – | – |
| 5 | `safety_compliance` | Plating Safety, Waste & Compliance | building | – | – |
| 6 | `plating_business` | Plating Service Economics & Client Management | building | – | – |

Content rules (non-negotiable, in `BRIEFS.json` `_readme`): setpoints/prices as mechanisms
with TDS-first verification, never invented figures; safety as obligation classes with the
absolute cyanide/acid segregation line; commercial chemistry only; regulated marking terms
as verify-jurisdiction classes; educational never personalized advice; never represent
plated as solid gold. Gates prove structure + math only — content honesty is enforced by
the briefs + grounding corpus + a fresh-context verifier pass per specialist.
