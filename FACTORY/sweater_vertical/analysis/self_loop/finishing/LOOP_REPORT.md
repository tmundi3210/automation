# Finishing Specialist — Self-Improvement Loop Report

**Slug:** `finishing` | **Segment:** T12 | **Rounds run:** 2 (max) | **Generated:** 2026-07-01

---

## Health verdict (one line)

**CONVERGED — HEALTHY-WITH-HARDENING.** The finishing specialist is honest, in-lane and correct on every load-bearing joint; no HIGH-severity finding survives. Three mechanical contract-hardening edits (`apply_now`) close latent over-trust/over-claim risks in KB node *contracts* (not substance), and four genuine content/procedure gaps are backlogged (`propose`) because closing them requires PPS measurement or judgment the honesty binding forbids fabricating.

---

## Round-by-round story

- **Round 1** fired 12 adversarial questions at the load-bearing joints (viability deference, the pilling FLOOR-vs-ceiling boundary, the [UNKNOWN] per-piece job-work rate, the ~$0.25–1.80 cost band, irreversible milling, softener masking, the two-PO shared gate, ISO 12945-1 method, the outsource-all-wet boundary, off-paper mechanism, the corrected duty trio, and the efficacy formula). **Eight passed cleanly** (Q02 viability deferral, Q03 refuse-the-acrylic-rescue, Q04 grade-split direction-not-magnitude, Q05 milling gate-not-setpoint, Q06 outsource/route-regulated, Q07 off-paper mechanism, Q10 ISO-12945-1 post-wash master gate, Q11 no ~7%/duty stays with unit_economics). One `watch` (Q01 cost-band inline derivation), three MED `propose` (Q08 two-PO fault seam, Q09 blend caveat, Q12 efficacy cardinal-read). `dry=false` → a second round was warranted.
- **Round 2** re-aimed 11 sharper questions at the three surviving joints plus fresh angles the KBs themselves confess (route-stacking, blend-drift/duty/IOR, milling in-run control, unconfirmed shrinkage tolerance, spirality attribution, the softener-masking approved-reference loop, off-paper drift under a new framing, the A2 merged go/no-go). The specialist answered self-critically: **four clean OK** (Q05 composition flag-and-route with corrected trio, Q08 spirality detect/route-upstream, Q10 off-paper mechanism, Q11 A2 verdict defers viability). Round 2 sharpened two former `watch`/`propose` items into **mechanical `apply_now`** fixes (efficacy node, cost band) and surfaced one new `apply_now` (shrinkage tag) plus two new `propose` items (milling in-run control, masked approved reference).
- **Stop condition:** `max_rounds=2` reached with MED findings still open, so the round-2 judge marked it "NOT-CONVERGED-BUT-BOUNDED". Per the synthesizer rule (`converged=true` unless a **HIGH**-severity finding is open at the cap), and since **no HIGH finding is open** — every residual is either a mechanical hardening or a correctly-[UNKNOWN]-held content gap — this report records **converged=true** with a real health grade.

---

## Findings table (deduped across rounds)

| ref | type | severity | evidence | fix | disposition |
|---|---|---|---|---|---|
| F1 (R1-Q12 / R2-Q03) | overclaim | MED | `FINISHING_EFFICACY_FORMULA` self-describes as "recomputes derived scores from base metrics" with `human_review_required=false`; nothing forces the OUTPUT to be labeled ordinal, inviting a cardinal A1>A2>A3 spend ranking off a plugged-in "half a grade". Mitigated by `EDGE_033` + failure_modes but not hard-stopped. | Set `human_review_required=true` and add an output-contract line: output is ORDINAL/directional, magnitudes [UNKNOWN] pending PPS, never a cardinal T4 spend number. | **apply_now** |
| F2 (R1-Q01 / R2-Q04) | untagged | LOW→MED | `COST_OF_QUALITY_TRADE` bands (~$0.30–0.90 A1 / ~$0.25–0.70 A2 / ~$0.60–1.80 A3) are tagged `[ESTIMATE]` with ordering-HIGH/absolutes-LOW and a do-not-feed-as-points guard, but carry no single INLINE method+basis+confidence string, so the ordering can be over-read as points. | Append inline: "method=triangulated by route step-content; basis=A3 milling+boarding > A1 +cellulase bath > A2 chemical top-up; confidence: ordering HIGH, absolutes LOW; per-piece Ludhiana finishing rate [UNKNOWN] — not a T4 point." | **apply_now** |
| F3 (R2-Q07) | untagged | MED | `SHRINKAGE_GATE` presents ±3–5% as the accept band inside an ACCEPTANCE gate; its own anti_pattern forbids "treating ±3–5% as fixed without buyer confirmation" and `escalation_triggers[5]` says confirm-per-buyer BEFORE it gates, yet the threshold carries no [UNKNOWN]-pending-buyer status tag. A Turlock tumble-dry lot can pass green at ±4% and still fail the real customer. | Tag threshold `[ESTIMATE: working target; buyer-confirmed value UNKNOWN-pending-buyer]`, gate provisionally at the tighter end (±3%) for tumble-dry, require the exact customer wash+dry cycle before it is a hard line. | **apply_now** |
| F4 (R1-Q09 / R2-Q01) | gap | MED | The four anti-pill routes and `A1_COTTON_RECIPE` are written on PURE-fibre archetypes, but A1 SHIPS as a cotton-blend. Directions transfer (singe beads the thermoplastic fraction, cellulase touches only cellulosics, acrylic-fraction pilling is a yarn-PO lever) but no blend-fraction threshold is specified. | Add a per-article blend caveat naming the A1 ratio, degrading singe/cellulase yield with rising acrylic fraction (thresholds [UNKNOWN] until PPS), routing acrylic-fraction pilling to the yarn PO. **Do not fabricate a cutoff %.** | **propose** |
| F5 (R1-Q08 / R2-Q02) | gap | MED | Shared post-wash gate closes the "skipped step passes at the wrong vendor" seam but not fault-attribution on a mixed/stacked pilling fail (A1 = singe AND cellulase). `RETAINED_STRENGTH_CAP` partially fingers cellulase over/under-dose; "yarn floor dominates" is a direction, not a contractual adjudicator. | Add a fault-isolation procedure: on a stacked pilling fail, isolate with a singed-yarn-only and a bio-polished-garment swatch re-test + a both-ends retained-strength/hairiness read; code back on both POs until localized. | **propose** |
| F6 (R2-Q06) | gap | MED | Milling (irreversible, "single most destructive finishing failure") is outsourced; the felt/area-shrink gate is POST-HOC and only rejects already-dead wool. Spec honestly concedes no real-time control; residual is an unclosable exposure, not a false claim. | Make the exposure explicit in `A3_WOOL_RECIPE`/`MILLING_GATE`: no in-run brand control → protection = vendor felt-control qualification + PPS/first-piece dial-in + post-hoc reject; price the over-mill risk into A3 working-capital exposure. | **propose** |
| F7 (R2-Q09) | gap | MED | Post-wash gate dominates the showroom sample, but if the buyer-APPROVED reference/PPS sample is itself the freshly-softened masked one, a lot matching it can pass sign-off while failing post-wash; rejection authority is asserted, not operationalised. | Require the approved reference/PPS sample to be POST-WASH-graded (approve on the washed hand), so sign-off and gate share the same post-wash condition and rejection is contractual. | **propose** |
| F8 (R1-Q01 residual) | ok/watch | LOW | Cost band's guard exists in substance; residual over-trust risk is documentation, largely resolved by F2. | Covered by F2 apply_now. | **watch** |
| F9 (R2-Q11) | ok/watch | LOW | A2 verdict defers viability correctly; the full G1–G4 / "dead as first drawn" framing is a brain/DECISION_MEMO construct inherited at the router layer, not native to the finishing spec. | Optional one-line in-spec pointer that viability-touching finishing verdicts defer to unit_economics + DECISION_MEMO and carry the GATED posture + G4-on-A2. Not required for correctness. | **watch** |

---

## apply_now (3 — mechanical, must re-gate ALL GREEN via build.sh)

1. **F1 — Efficacy-node backstop.** On `FINISHING_EFFICACY_FORMULA` (kb3): set `human_review_required=true` and add an output-contract line making the output explicitly ORDINAL/directional with magnitudes [UNKNOWN] pending PPS — never a cardinal T4 spend-sizing number. Cross-reference `EDGE_033` and `escalation_triggers[4]`.
2. **F2 — Cost-band inline derivation.** On `COST_OF_QUALITY_TRADE` (kb3): append the inline `method+basis+confidence` string (see table) so the ordering cannot be lifted as decision points; keep the per-piece rate [UNKNOWN].
3. **F3 — Shrinkage-threshold status tag.** On `SHRINKAGE_GATE` (kb3): tag the ±3–5% threshold `[ESTIMATE: working target; buyer-confirmed value UNKNOWN-pending-buyer]`, gate provisionally at ±3% for a tumble-dry household, require the buyer-confirmed value + exact wash+dry cycle before it becomes a hard pass/fail line.

*Re-gate rule: after applying, re-run `bash FACTORY/build.sh FACTORY/sweater_vertical/specialists/finishing` → ALL GREEN, and confirm no ~7% duty residue and no new untagged number.*

## propose (4 — need PPS measurement or judgment; backlog, never fabricate)

- **F4** — Per-article blend-fraction caveat on the anti-pill routes (thresholds [UNKNOWN] until PPS).
- **F5** — Stacked/mixed-fail fault-isolation procedure for the two-PO shared gate.
- **F6** — Explicit A3 milling in-run-control exposure statement + working-capital risk pricing.
- **F7** — Post-wash-graded approved-reference / PPS sign-off loop to operationalise rejection authority against a masked sample.

## watch (2 — recorded so they are not rediscovered)

- **F8** — Cost-band over-trust residual (subsumed by F2).
- **F9** — In-spec viability-deference pointer (correct deference already achieved; inherited-vs-native N7 posture is a design fact, not an error).

---

## Verified strengths (the clean 'ok' answers — balance, not just defects)

1. **Viability deferral (R1-Q02, R2-Q11).** Refuses to answer "does the venture make money"; defers to unit_economics + DECISION_MEMO, carries the GATED-conditional-GO posture and the four hard gates, and does not let "finishing is a small slice" imply a naive GO. N7 satisfied.
2. **Refuses the acrylic finishing-rescue lie (R1-Q03).** Will not carry a cheap high-pill A2 to ICI ≥3–4 "at finishing"; defers the floor to yarn_sourcing and holds the fraction magnitude [UNKNOWN] — the specialist's core value at the brand-ending G4 gate.
3. **Grade split held directional, not measured (R1-Q04).** "Real grade on cotton / fraction on acrylic / craft on wool" stated as DIRECTION with per-construction magnitudes [UNKNOWN] until PPS; no contradiction with second-line-of-defence.
4. **Milling = gate, not setpoint (R1-Q05).** Hands a felt/area-shrink gate + PPS dial-in, refuses to quote ~40 °C / 10 min as a machine setpoint on the one irreversible step.
5. **Outsource-all-wet, route the regulated node (R1-Q06).** Outsources even one small bath, gives no individualized effluent/legal counsel, holds effluent-compliance cost [UNKNOWN], routes cluster-compliance and legal status to Ludhiana T1 / human_review.
6. **Off-paper stays mechanism (R1-Q07, R2-Q10).** Structural incentives only, no named-party accusation, magnitudes [UNKNOWN], substantive read deferred to Ludhiana. N3 satisfied on a standing failure-mode axis.
7. **ISO 12945-1 post-wash master gate (R1-Q10).** Rejects a bare "grade 4", rejects Martindale for soft knits, rejects dry/showroom reads, defers the un-trialled garment-dye hero drop.
8. **Corrected duty trio, no lane crossing (R1-Q11, R2-Q05).** The composition/blend-drift check flags duty EXPOSURE with cotton ~16.5% (never ~7%), wool ~16%, acrylic/MMF ~32% and the 50% chief-weight cliff, then defers the landed/IOR CONSEQUENCE to unit_economics. N5 satisfied.
9. **Spirality detect-and-route (R2-Q08).** Read at the gate, no false finishing rescue, fix routed upstream to the yarn-PO balanced-ply dial; honestly concedes the failed lot's cost is sunk.

---

## Honesty audit (N1–N7)

PASS on all. Every money/duty/uplift/cost claim tagged; the three governing blockers (Ludhiana knit+LINK rate, live blended CAC, forward India duty) **and** the per-piece Ludhiana finishing job-work rate held [UNKNOWN] throughout; no fabricated firm figure; off-paper strictly structural mechanism; cotton duty ~16.5% with **no ~7% residue anywhere**; the four hard gates surfaced where leaned on; viability carries the GATED-conditional-GO posture. No AI/build-our-own over-promise surfaced.
