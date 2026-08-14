# RELATION_QUALITY_FINDINGS — specialists applied to the 2026-07-11 live-graph defects

_Three specialists (`kgverify`, `kgraph`, `viz`+`uxguide`) were run as design authorities on the owner's live-graph screenshots (wrong `part_of` relationships; L0 zoom speck). This is `kgverify`'s first live application (a dry run — verdicts/proposals only, nothing edited). Their outputs decompose the problem into a **now-buildable structural+honesty+nav track** and a **deferred runtime truth-repair track (Feature A proper)**._

## The unifying defect
Nearly every bad edge is a `part_of` between distinct named endocrine structures that merely **co-occur on First Aid pp.333-342** — the extractor read *co-occurrence on a page* as *containment*. Two opposite defect classes, demanding opposite treatments:
- **Endocrine `part_of` edges → systematically FALSE** (retype to functional/`related`, or delete).
- **Malignancy edges → plausibly TRUE but MISTYPED** (`malignancy→endocarditis` = marantic/NBTE; `malignancy→PTHrP` = paraneoplastic hypercalcemia) → retype, **never delete**.
This asymmetry is the whole argument against a blanket "delete all part_of" pass.

## `kgverify` — per-edge verdicts (dry run; all clinical verdicts are high-prior CANDIDATES — no retrieval corpus this session)
| Edge | Verdict | Why |
|---|---|---|
| adrenal cortex `part_of` pituitary | **WRONG (certain, structural)** | antisymmetry + containment cycle with the reverse edge |
| medulla `part_of` pituitary | **WRONG (certain, structural)** | 3-cycle via adrenal cortex; adrenal medulla is a sibling of cortex |
| hypothalamus `part_of` pituitary | **WRONG (content)** | well-formed but false; functional (HPA) link mislabeled structural |
| hypothalamus `part_of` Organ | **ABSTAIN + DEFER** | "Organ" is a class token — no truth-evaluable proposition |
| FSH `expressed_in` pituitary | **CORRECT — protect** | type-appropriate; anterior pituitary secretes FSH |
| TSH `expressed_in` pituitary | **CORRECT — protect** | anterior pituitary secretes TSH |
| posterior pituitary `part_of` pituitary | **CORRECT — protect** | genuine mereology (neurohypophysis ⊂ pituitary) |
| malignancy → endocarditis / PTHrP | **ESCALATE — do NOT delete** | real associations, wrong/underspecified relation TYPE |

**Graph-wide detection rules (deterministic-first):** G1 antisymmetry/mutual-part_of · G2 containment-cycle over transitive closure · G3 co-occurrence-mislabeled-as-part_of (only-support-is-co-mention + no partitive cue + inconsistent partonomy) · G4 domain/range disjointness (correctly stays silent on `expressed_in`) · G5 functional-vs-partitive retype signal · G6 cardinality/hub anomaly · G7 placeholder/abstract-endpoint · G8 human-flag verification (every owner "non-sequitur" is itself verified) · G9 negation hook (0/162 negated) · G10 thin-link coverage signal (heart) — propose new edges only with a citation.

**Repair discipline:** propose-never-rewrite; owner-gated at any confidence incl. 1.0; the chosen op follows the *verdict* (retype/reorient), not the surface shape; provenance is immutable append-only per-assertion; **a corrected edge stays ESTIMATE until a separate human FACT-confirmation** — correcting never launders provenance.

## `kgraph` — structural guards (clinical-judgment-free, enforced at `writeEdge`)
| Inv | Rule | Action |
|---|---|---|
| INV-1 | irreflexivity — no self-loop, any type | HARD-REJECT |
| INV-2 | antisymmetry — reject `A part_of B` if `B part_of A` exists | HARD-REJECT (offer retype/drop) |
| INV-3 | `part_of` acyclicity — reject edges closing a part_of cycle | HARD-REJECT |
| INV-4 | over-generic/class target (`→ Organ`) — needs a `taxonomy_role` class marker | REJECT if class-marked, else FLAG |
| INV-5 | cardinality / co-occurrence fan-in burst (same import batch converging on one target) — the **structural fingerprint** of co-occurrence-as-containment | FLAG → quarantine → route to kgverify |

Of the observed defects, only **`hypothalamus → Organ`** is a clean structural reject (INV-4); the pituitary cluster is only a soft INV-5 flag on this data; INV-1/2/3 are the guard for the whole inversion/cycle *class*. Guards **reject or flag, never silently drop** (matches the existing prereq-cycle precedent).

**Load-bearing point (kgraph E1):** the guards are moot on imports unless `kgImport` actually routes through `writeEdge` — that is the D-1 sole-write-path issue. G0 recorded D-1 as fixed+live-verified 2026-07-10; qG's first acceptance re-confirms it (a sync-revert once undid a fix already).

**Amendments kgraph raises (SLICE_6 / KG-contract):** A1 extend acyclicity from prereq-only to `part_of` (+ **E2: kgraph's own KB1 `PREREQ_DAG` wrongly excludes part_of from acyclicity — self-flagged contradiction to fix**); A2 name irreflexivity; A3 add a `taxonomy_role` class value (or denylist) so `→ Organ` is hard-rejectable — a vocabulary addition, not a new field; A4 record the INV-5 fan-in flag policy (threshold owner-tunable [ESTIMATE]); A5 no schema change for negation (the `negated` field already exists).

## `viz`+`uxguide` — q4 navigation (camera-only; positions never move; no motion polish)
Root-cause hypothesis for the speck (R6): L0 computes its fit bounding box over the **wrong node set** (full L2 coordinate space instead of the 46 L0 meta-nodes) or before positions settle.
- **Fit-to-view (AC-FIT):** auto-fit on load/reload/**entering each LOD level**/drill-in/search-land; frame the level's *drawn* content (glyphs+labels+edges) centered in the viewport minus docked panels; padding ≈8% [ESTIMATE]; **never-a-speck** (binding axis fills 0.80–0.92; hard-fail <0.50), never clipped; **camera-only** (byte-identical node coords — regression guard for Pass-3 persistence); deterministic on reload.
- **Zoom-anchor (AC-ZOOM):** the graph point under the cursor/pinch stays put (≤4px); min-zoom = fit-all, sane max cap; manual zoom crossing an LOD threshold swaps representation **in place, no auto-fit** (preserves the anchor).
- **Keyboard/tap parity (AC-KBD):** fit-all/fit-selection/zoom by button + key + touch, never wheel-only; Escape ladder dismisses cloud→panel→selection but **not** the camera; honest labels ("Fit all" = reframe camera, not rearrange); WCAG/reduced-motion/tap-target reported [UNKNOWN until tested].
- **Out of scope (graphux tail):** flight/tween easing, momentum/rubber-band, LOD morph animation, Escape-restore choreography, search fly-to easing — the spec is satisfiable with instant cuts.

## Design decisions the specialists escalated
- **R1 (adopted):** auto-fit only on *programmatic* entry; manual zoom swaps representation in place. Confirm at the q4 before/after eyeball.
- **R4 (open):** does the far-parked "Unassigned" group get its own framing region or inflate fit-all? — surface at the q4 owner eyeball.
- **Interim honesty (kgverify §4):** default every edge to **"Unverified — not yet checked,"** show no green/verified marker on anything (even the 3 correct edges are only candidate-TRUE), status uses a **separate badge, not luminance** (luminance is committed to importance / D1); viz owns the encoding, uxguide owns the copy.

## Track split → executable items
- **qG (now, clinical-judgment-free):** D-1 re-verify + kgraph INV-1..5 write-time guards + SLICE_6/KG-contract amendments A1–A5 + fix kgraph KB1 E2 contradiction.
- **qH (now, cheap, high-value):** the interim "Unverified" edge-status label.
- **q4 (existing, sharpened):** the viz/uxguide navigation spec above.
- **Feature A / `kgverify` runtime pipeline (deferred to the Fable 18-specialist run):** G1–G10 detectors, propose-never-rewrite proposal cards, owner-gated retype repairs, provenance store. This is the real relationship *repair* and needs the full build, not a now-command.
