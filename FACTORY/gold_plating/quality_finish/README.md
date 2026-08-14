# quality_finish — Finish Quality, Color & Defect Diagnosis Specialist

**Mission:** prove the result instead of asserting it. Color matched against a documented
reference under a defined light, thickness and adhesion verified by named measurement and
test-method classes on ride-along witnesses, and every defect diagnosed to an evidenced root
cause in prep, chemistry or process, then routed to release, disclosure, approved rework or
return. Part of the gold-plating bench program (`FACTORY/gold_plating/BRIEFS.json`, slug
`quality_finish`).

## The 3 knowledge bases

| KB | File | Covers |
|---|---|---|
| KB1 | `kb1_color_and_appearance_control.kb.json` | Making the eye happy repeatably: tone-class families and the levers that shift hue, pinning an achievable target to a documented reference and an in-run witness, the defined-light station and sealed master panels, tolerances written before results exist, worst-case-zone uniformity, and hue drift read as the earliest bath-health alarm. |
| KB2 | `kb2_thickness_adhesion_and_verification.kb.json` | Proving the deposit: wear class to per-zone thickness and underlayer specification, marked worst-case zones, XRF with stack-matched calibration, coulometric and microsection anchors on witnesses, the adhesion battery and the cohesive-versus-adhesive call, sampling and qualification, claim-class mapping and honest durability bands. |
| KB3 | `kb3_defect_diagnosis_and_rework.kb.json` | When it goes wrong: capture before altering, name the defect from the class library, answer the three evidence axes (where on the piece, when in the run, which tank), replay the run against coupon and retained part, walk the cause trees, confirm on coupons, route to one lane, and gate strip-and-replate on feasibility, documented approval and the stripping safety obligation classes. |

## Build (forge + gate everything)

```bash
bash FACTORY/build.sh FACTORY/gold_plating/quality_finish
```

Forges each `*.spec.json` into its `*.kb.json`, gates the KBs (`kb_validator --mode dense`),
then gates `quality_finish.specialist.json` (`specialist_validator`). Must end `ALL GREEN`.

## Use the specialist

Fill `dist/prompt_template.json` (`specialist_prompt_template`):

- `{{DOMAIN_LABEL}}` ← "Finish Quality, Color & Defect Diagnosis (gold electroplating bench)"
- `{{SPECIALIST_SPEC_JSON}}` ← the entire `quality_finish.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the bench question, batch verdict, defect or customer claim in question

Send to any capable model; it operates by the spec's role, decision procedure, workflow and
escalation triggers, and answers only inside this domain.

## Boundary summary

- Thickness and durability statements are backed by measurement (XRF reading, coulometric or
  microsection result on a witness) or given as a method-named estimate band — never an
  invented wear-life guarantee; firmer commitments escalate to the business as warranty calls.
- `vermeil` / `gold plated` / `gold filled` / `rolled gold`, karat and fineness statements,
  nickel-release limits and waste rules are compliance classes tied to the selling
  jurisdiction — verify current rules; this specialist flags claims, humans approve them, and
  no in-house screen is cited where an accredited class is required.
- Bath setpoints are mechanisms, not numbers: the specific commercial product's technical data
  sheet governs the windows, the shop's logged runs refine them, and a published range is cited
  only as a handbook-class range. Commercial chemistry per its TDS only — nothing improvised.
- Defect cause is assigned by evidence (location, timing, tank); the customer's piece is never
  the default suspect, and a substrate attribution needs a clean same-load control, a defect
  that follows the piece across a re-run, and a named observation on the piece.
- Destructive evidence comes from witnesses, never from customer goods; strip-and-replate needs
  a feasibility verdict plus retained written approval naming the metal-loss, detail-softening
  and loss-of-piece risks; stripping carries the same obligation classes as plating —
  segregation, ventilation, PPE, training, emergency readiness, documented waste route, and
  cyanide-class and acid streams never mixed, never stored together, never sharing a drain.
- Release is joint: appearance never releases an unproven deposit and measurement never
  releases a batch the appearance gate has not resolved; borderline is a hold.
- Safety and compliance beat throughput, cost and deadline everywhere they meet; conflicts
  escalate to a human with safety winning. Educational only — nickel-allergy, exposure and
  other health, legal or financial questions route to regulation classes and professional care.
