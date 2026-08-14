# Gold Bath Chemistry & Solution Management Specialist

**Vertical:** `FACTORY/gold_plating` (the BENCH side of the gold-electroplating jeweler program)
**Slug:** `bath_chemistry` · **Specialist id:** `bath_chemistry`

**Mission —** select, maintain and troubleshoot the gold and support baths so colour, karat class and
deposit quality stay stable run after run: data-sheet-first, log-driven, contamination-aware.

The specialist is a distilled operating spec (`bath_chemistry.specialist.json`) standing on three
dense, gate-passing knowledge bases. It answers three territories and routes everything else:
*which bath to run*, *is the bath in window today*, and *how does this solution live, sicken and die*.

## The three knowledge bases

| KB | Domain | One line |
|---|---|---|
| `kb1_gold_bath_families_and_selection.kb.json` | `gold_plating_bath_selection` | Which gold system a job must run — alkaline cyanide, acid hardened, near-neutral, sulfite non-cyanide and no-rectifier immersion/autocatalytic classes; strike vs build vs colour roles; support and barrier baths; colour and karat as an alloying mechanism; substrate, forming-order, wear-class and claim-class filters; vacuum coating named only as an out-of-scope alternative. |
| `kb2_bath_analysis_and_replenishment.kb.json` | `gold_plating_bath_maintenance` | Keeping a bath inside its supplier window — what gold content, pH, temperature, agitation and additive balance each do to the deposit; drift signatures; the analysis method classes (titration, Hull cell, witness coupon + XRF, supplier lab); little-and-often replenishment vs staged crash correction; the bath log as the shop's evidence base. |
| `kb3_contamination_and_bath_lifecycle.kb.json` | `gold_plating_bath_lifecycle` | Why baths die and what to do about it — drag-in/drag-out control, metallic vs organic vs carbonate-aging signatures, the differential verdict before any treatment, filtration/carbon/dummy/freeze-out method classes, quarantine, regenerate-vs-replace economics, and gold recovery and regulated end-of-life handoff. |

Each KB carries its own `.spec.json` (the compact content spec the forge builds from), `.kb.json`
(the forged knowledge base), `.validation.json` and `.metrics.json`.

## Build / gate

```bash
bash FACTORY/build.sh FACTORY/gold_plating/bath_chemistry
```

Forges each `*.spec.json` into its `*.kb.json`, gates all three in `--mode dense`, checks the
specialist's `grounded_in_kbs` resolve, then gates the specialist. It must end **`ALL GREEN`**.
Gate the specialist alone with:

```bash
python3 validators/specialist_validator.py \
  FACTORY/gold_plating/bath_chemistry/bath_chemistry.specialist.json \
  --report FACTORY/gold_plating/bath_chemistry/bath_chemistry.specialist.validation.json
```

## Using the specialist

The harness is `dist/prompt_template.json`. Fill the three slots of its
`specialist_prompt_template` and send to any capable model — no training, no setup:

- `{{DOMAIN_LABEL}}` ← `Gold Bath Chemistry & Solution Management (heavy: grounded in 3 dense KBs)`
- `{{SPECIALIST_SPEC_JSON}}` ← the **entire** contents of `bath_chemistry.specialist.json`, verbatim
- `{{USER_QUESTION}}` ← the bench question

The model then operates strictly by the spec's `role`, `decision_procedure` and `workflow`, honours
every `escalation_trigger`, and answers only inside the domain. Invoke it when the question is which
bath to run, why the deposit changed, what a reading or drift signature means, what and how much to
add, whether a sick bath is contaminated or merely aged, regenerate or replace, or where the gold and
the spent solution go.

## Boundary summary

- **Setpoints are mechanisms, not facts.** Temperature, pH, gold and hardener concentration, current
  density, replenisher dose and contamination tolerance are explained as *what the parameter does and
  how its drift shows up*, then sourced from the specific commercial product's technical data sheet,
  from a published range cited **as** a handbook-class range, or from the shop's own dated measurement.
- **Safety is content.** Cyanide-bearing solutions and acids are never mixed, never stored together
  and never share a drain, tool, spill path or staging zone; cyanide streams stay alkaline. All of it
  points to formal training and to *verify your jurisdiction's workplace and hazmat rules* — never an
  improvised procedure.
- **Commercial products only.** Purchased plating chemistry used per its data sheet, with only
  supplier-sanctioned treatments. No formulation from raw chemicals.
- **Analysis is a method class.** Titration, Hull cell, witness coupon + XRF and supplier lab results
  are reported with their method and limits against the sheet window — never as numeric truths.
- **Regulated terms are obligation classes.** Gold plated / vermeil / gold filled / rolled gold,
  nickel-release limits and waste and discharge rules carry *verify the current rules in your
  jurisdiction and date the check* — never asserted thresholds.
- **Educational, never personalized.** No legal, medical, regulatory or financial advice; nickel-allergy
  and exposure questions route to the obligation class and to the person's own professional.
- **Ethical selling first-class.** A plated piece is never called solid gold, the wear expectation is
  disclosed in writing before work starts, and heirloom risk is documented at intake.
- **Safety and compliance beat throughput, cost and deadline** in every conflict where they meet — the
  out-of-window stop rule, quarantine, coupon qualification and the claim-class refusal do not bend to
  a delivery date.
- **Out of scope, named and routed:** surface preparation and cleaning, racking and rectifier practice,
  finished-part acceptance QC, effluent-treatment engineering, pricing, and PVD / vacuum coating.
