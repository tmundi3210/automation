# Cupcake Craft — Neutralized Project Brief

> Orchestrator-owned planning doc (human-readable). The **intent is preserved**; the
> raw idea is professionalized and de-personalized so it can drive the Phase-B research
> pipeline (taxonomy → per-subdomain dense KB → per-domain specialist → router).

## 1. One-line objective
Engineer **the most delicious cupcake** by building a set of narrow-but-deep culinary
specialists that, composed together, cover every decision area that determines a
cupcake's flavor, texture, moisture, aroma, finish, and eating quality.

## 2. Preserved intent — raw element → neutralized statement
| You said (intent) | Neutralized engineering statement |
|---|---|
| "baking cupcakes" | The product class is the **cupcake** (small high-ratio batter cake baked in a liner), not layer cakes, muffins, or quick breads (adjacent but distinct). |
| "the most delicious one" | The optimization target is **maximal eating quality**: flavor balance + aroma + crumb texture + moistness + frosting harmony + finish — judged by sensory evaluation, not by yield or cost. |
| "create new specialists ... how many it requires" | Decompose the craft into its **independent expert domains**; each domain becomes one machine-facing **specialist**. The count is whatever a non-padded decomposition yields. |

## 3. Decomposition decision (fixes the specialist count)
A complete, non-overlapping decomposition of "engineer the most delicious cupcake"
yields **7 domains → 7 specialists**, each grounded in 2 subdomain KBs (14 KBs total):

1. `formula`   — Recipe Formulation & Baking Chemistry (ratios, leavening, pH)
2. `mixing`    — Batter Mixing Method & Emulsion (creaming/reverse-creaming, aeration, gluten)
3. `bake`      — Thermal Process & Oven Control (temperature, rise, doming, doneness)
4. `flavor`    — Flavor Design & Sensory Balance (pairing, Maillard/caramelization, aroma)
5. `frosting`  — Frosting, Filling & Finishing (buttercreams/ganache, assembly, decoration)
6. `ingredient`— Ingredient Selection, Quality & Substitution (flour/cocoa/butter/egg/sugar)
7. `quality`   — Sensory QC, Defect Troubleshooting & Storage (defect root-cause, staling, safety)

## 4. Real-world execution notes (encode as KB conflict_axes / dominance_rules)
- The SCIENCE underneath (food chemistry, heat transfer, emulsion physics, sensory science)
  is book-stable; the CRAFT (specific brands, ovens, humidity, altitude, palate) is tacit
  and variable — encode that variability as conflict axes, not as single right answers.
- Hard tradeoffs to encode: moistness vs structure (more fat/sugar/liquid tenderizes but
  weakens crumb); tenderness vs rise; sweetness vs flavor complexity; frosting sweetness vs
  cake sweetness balance; speed/convenience vs quality; shelf-life vs fresh-baked texture;
  food safety (egg/dairy/temperature) is a hard non-negotiable dominance rule.

## 5. Boundaries
- Scope is the cupcake itself + its frosting/finish; not commercial bakery operations,
  packaging logistics, or nutrition/medical claims.
- Substitutions (gluten-free / vegan / allergen) are in scope as a subdomain, framed as
  structure/flavor tradeoffs — not as dietary or medical advice.
