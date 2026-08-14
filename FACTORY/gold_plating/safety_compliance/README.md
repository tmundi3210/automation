# Plating Safety, Waste & Compliance Specialist

**Vertical:** `FACTORY/gold_plating` (the BENCH side of the gold-electroplating jeweler program)
**Slug:** `safety_compliance` · **Specialist id:** `safety_compliance`

**Mission —** keep people, premises and claims legal and safe: chemical safety discipline, regulated
waste handling, and marking/advertising compliance as the gate every job passes through — safety
beats throughput everywhere they meet.

The specialist is a distilled operating spec (`safety_compliance.specialist.json`) standing on three
dense, gate-passing knowledge bases. It is not an advisory aside on other people's jobs: it runs
three release gates — *can this room and these people carry this chemistry today*, *does every stream
this job creates have a lawful container and route*, and *has this batch earned the words on its tag*
— and each one fails closed.

## The three knowledge bases

| KB | Domain | One line |
|---|---|---|
| `kb1_chemical_safety_operations.kb.json` | `gold_plating_chemical_safety` | What keeps the plating room safe — the hazard classes actually present (cyanide-bearing solutions, mineral acids, caustic alkalis, nickel salts, hot tanks, stored electrical energy), the non-negotiable cyanide/acid segregation line as physical architecture, containment and drains, storage, security and labeling, source-capture ventilation, gas detection and the alkaline pH floor, task-derived PPE, training and authorization, eyewash / zoned spill kits / exposure routing / drills, and the pre-run readiness gate that fails closed. |
| `kb2_waste_and_environmental_handling.kb.json` | `gold_plating_waste_handling` | What leaves the shop — the full output-stream inventory (spent baths, still-rinse concentrate, flowing rinses, pickle, filters, sludges, spill residues, extraction outlet), characterization before movement, compatible labeled accumulation containers, drag-out reduction and counterflow rinse architecture, gold-bearing routing to a licensed reclaimer with settlement-only metal accounting, the verified licensed transporter/destination chain and its manifests, the no-discharge default, and the pre-run waste release gate. |
| `kb3_marking_claims_and_consumer_rules.kb.json` | `gold_plating_marking_compliance` | What you may call it and promise — the regulated marking-term ladder (gold flashed/washed, gold electroplate, heavy gold electroplate, vermeil, gold filled and rolled gold) as term classes gated by base metal, worst-case-point thickness and deposit fineness, nickel release for skin-contact goods as a market-access class, accredited-versus-in-house evidence grades, the per-product claim dossier and approved wording set, pre-purchase disclosure, stamps, channel and implied-claim review, the never-solid-gold line, and the claim release gate. |

Each KB carries its own `.spec.json` (the compact content spec the forge builds from), `.kb.json`
(the forged knowledge base), `.validation.json` and `.metrics.json`.

## Build / gate

```bash
bash FACTORY/build.sh FACTORY/gold_plating/safety_compliance
```

Forges each `*.spec.json` into its `*.kb.json`, gates all three in `--mode dense`, checks the
specialist's `grounded_in_kbs` resolve, then gates the specialist. It must end **`ALL GREEN`**.
Gate the specialist alone with:

```bash
python3 validators/specialist_validator.py \
  FACTORY/gold_plating/safety_compliance/safety_compliance.specialist.json \
  --report FACTORY/gold_plating/safety_compliance/safety_compliance.specialist.validation.json
```

## Using the specialist

The harness is `dist/prompt_template.json`. Fill the three slots of its
`specialist_prompt_template` and send to any capable model — no training, no setup:

- `{{DOMAIN_LABEL}}` ← `Plating Safety, Waste & Compliance (heavy: grounded in 3 dense KBs)`
- `{{SPECIALIST_SPEC_JSON}}` ← the **entire** contents of `safety_compliance.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the bench question

The model then operates strictly by the spec's `role`, `decision_procedure` and `workflow`, honours
every `escalation_trigger`, and answers only inside the domain. Invoke it whenever the question
touches hazardous plating chemistry in the room (segregation, containment, storage, labeling,
ventilation, gas detection, pH floor, PPE, training and authorization, spill or exposure readiness),
anything that leaves the shop (spent baths, rinse waters, filters, sludges, spill residue, drains,
extraction outlets, reclaim shipments, manifests, records), or anything a finished piece is called,
stamped, pictured, promised or sold as.

## Boundary summary

- **Safety is an obligation class plus a formal-training pointer** — never a substitute for hazmat and
  workplace-safety training, never an improvised bench procedure. *Verify your jurisdiction's rules
  with the named authority and record the date.*
- **The hard line.** Cyanide-bearing solutions and acids are never mixed, never stored together, and
  never share a drain, bund, tool, glove, spill kit or waste container. No efficiency, cost, space or
  schedule argument reopens it, and every rinse, drag-out vessel, filter and sludge that touched a
  cyanide-bearing solution carries that solution's hazard class.
- **Waste is regulated-class material.** Characterize before it moves, containerise, manifest, hand
  only to a verified licensed transporter and destination. No sewer-disposal advice, no bench
  treatment, and a cyanide waste is never acidified for any reason. Default: **no discharge**.
- **Marking terms are regulated classes.** The specialist places a product on the term-class ladder,
  flags what the held evidence cannot carry, and freezes the release — **final legal sign-off is human.**
- **No fabricated firm figures.** Setpoints, alkaline floors, exposure limits, hold times, permit
  limits, thickness class boundaries, prices and wear lives are given as mechanisms, as published
  handbook-class ranges labelled *as* handbook-class ranges, or as pointers to the product's technical
  data sheet, its safety data sheet, the named authority, or a measurement — never as universal fact.
- **Commercial products only.** Purchased plating chemistry used per its own technical data sheet. No
  bath formulation from raw chemicals.
- **Medical topics route out.** Nickel allergy, exposure, a rash or a cough after a run go to the
  regulation classes and to emergency services or occupational health with the product sheet in hand —
  never a diagnosis, a dose, or clearing a person to keep working.
- **Ethical selling is first-class.** A plated article is never represented as solid gold; no lifetime,
  permanent, never-tarnish, hypoallergenic or nickel-free language the evidence does not carry; wear
  expectations disclosed before purchase; heirloom and irreplaceable-piece risk escalated at intake.
- **Educational, never personalized.** No individualized legal, tax, insurance or financial advice —
  the specialist names the class, the authority, and the question to ask.
- **Safety and compliance beat throughput, cost, deadline and account revenue** in every conflict axis
  where they meet. Where a control class cannot be sustained, the honest options are substitute a
  lower-hazard chemistry, reduce the scope, outsource to a licensed plater, or decline — never a
  discounted version of the same control set.
