# APPLIED.md — Self-Improvement Loop, applied fixes + independent re-gate (T12)

_What the loop's finalizers actually changed, and the orchestrator's independent verification. The git diff is the ground truth of what was applied; the finalizers were deliberately conservative — they applied only the high-confidence mechanical subset that keeps `build.sh` ALL GREEN and pushed everything else (out-of-lane, needs-measurement, or would-risk-a-gate) to `PROPOSED_BACKLOG.md`. So "apply_now proposed" is larger than "applied": that gap is the conservatism working, not lost findings._

## Independent re-gate (trust-but-verify — not the agents' self-report)

`bash FACTORY/build.sh` re-run by the orchestrator on all nine specialist dirs **after** the finalizers ran:

```
GREEN  ludhiana   GREEN  yarn       GREEN  machines
GREEN  mfg_economics          GREEN  quality_checker   GREEN  finishing
GREEN  brand_market           GREEN  unit_economics    GREEN  design
```

**9/9 ALL GREEN.** Residue scan clean: no bare `~7%` cotton duty (every `~7%` hit is deliberate anti-revert guardrail text — "the corrected-away cotton ~7% figure"), no food-template residue (the only `menu` hits are "evasion menu" / "test menu"), no banned specialist tokens.

## Files changed by the finalizers (10 files, ~69 insertions)

| Specialist | Files edited | The fix that landed (fix family: **under-referencing** + one content HIGH) |
|---|---|---|
| **yarn** | `yarn.specialist.json`, `kb2_yarn_sourcing_supply.spec.json` + `.kb.json` | **Content HIGH (F1).** The named integrated spinners (Vardhman/Sportking/Nahar/Oswal) no longer carry "in-house USTER-class lab QC" as flat named-firm fact. Now a three-way honest split at the **KB layer** (so a re-grounding cannot re-import it): the roster is `[FACT]`; "large integrated spinners **as a class** run USTER-class lab QC" is `[ESTIMATE: industry-lore, MED-HIGH]`; whether any **specific** named firm runs such a lab and will certify our small clause-bound lot is `[UNKNOWN]` until a live quote/audit (else budget a third-party spot-check). |
| **ludhiana** | `ludhiana.specialist.json` | The "GO for Ludhiana" verdict now states in-file that this is a **SOURCING/LOCATION** verdict, **NOT a venture-viability yes** — viability is GATED-conditional-GO owned by unit_economics + the DECISION_MEMO (contribution-after-CAC, gates G1–G4), never inferred from a cluster GO. Added to both `capabilities[].outputs` and the decision procedure. |
| **brand_market** | `brand_market.specialist.json` | The **GATED-conditional-GO posture + the four hard gates + "inventory cash FORBIDDEN until they pass"** tripwire are now written into the role and the decision procedure (previously only in shared context). The content EDGE-not-moat caveat on the AI-design pipeline was also inlined. |
| **quality_checker** | `quality.specialist.json` | The pilling **FLOOR** is explicitly ceded to `yarn_sourcing` (yarn evenness = DEMAND-via-PO; finished-garment = the one layer the brand OWNS), so owning the *test* is no longer mistakable for owning the *floor* (gate G4). |
| **finishing** | `kb3_quality_acceptance.spec.json` + `.kb.json` | The `FINISHING_EFFICACY_FORMULA` now carries an explicit **OUTPUT CONTRACT: ORDINAL/DIRECTIONAL, not a cardinal decision-grade score**; the plugged-in "~half a grade" is flagged a convenient round number (magnitudes `[UNKNOWN]` pending PPS) and must **not** be fed as a spend-sizing input to the T4 economics model; `human_review_required: true`. |
| **machines** | `machines.specialist.json` | The "sub-$20k used vs ~$2M–10M+ build" marquee anchor was reconciled with the specialist's own glossary (used whole-garment machines are ~$20k–90k), so the phrasing no longer cherry-picks a cheapest-panel figure. The build-NEVER conclusion is unchanged (every buy class stays 1–2+ orders of magnitude below build). |
| **mfg_economics** | `mfg.specialist.json` | The own-knitting breakeven now inlines a self-contained worked illustration of the ">1.0 ⇒ subcontract wins" logic (previously only in KB2), so a file-only reader sees the arithmetic shape, not a bare assertion. |

## What was NOT applied (moved to the backlog, by design)

Everything that needed new measurement, sat outside a specialist's lane, or could not be shown to keep ALL GREEN was pushed to `PROPOSED_BACKLOG.md` — e.g. the yarn `propose` cluster (a measured knit-garment cost-share), the machines buy-used-ladder reachability items, and every measurement blocker (knit+LINK rate G2, live CAC G1, forward duty, PPS pilling G4) which **stays `[UNKNOWN]` until actually measured** and must never be invented to "close" a finding.

## Net

The loop found **zero fabrication failures** brain-wide and one real content-level HIGH (yarn's lab over-attribution), now fixed at the KB layer. The dominant weakness family was **under-referencing** — correct guardrails living in shared context instead of the standalone specialist files — now inlined for the load-bearing cases (viability posture, the four gates, the pilling floor). All fixes independently re-gated ALL GREEN.
