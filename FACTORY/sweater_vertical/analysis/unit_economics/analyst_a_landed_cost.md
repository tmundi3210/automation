# Landed Cost — India → California (T9, Analyst A)

**For:** the owner of a small, AI-designed D2C knitwear brand sourcing in Ludhiana and selling in California / Turlock.
**Question this answers:** Taking T4's ex-factory bands (A1 cotton-blend core ≈ $4–9 / A2 fine-acrylic value ≈ $3.5–8 / A3 merino-lambswool premium ≈ $6–22+), what does each article cost **landed at our California door** — duty by HTS, the 2025–26 India tariff surcharge, freight, fees, broker, IOR setup — and **does duty re-rank the three articles**? And what happened to the **de-minimis / Section-321 ship-direct-from-India lever** that the whole D2C-from-India idea quietly assumed?

**How to read the tags (binding honesty discipline, inherited from T0/T1/T2/T4):**
- **[FACT]** = cited public source / true-by-definition identity.
- **[ESTIMATE, confidence]** = reasoned estimate; method + basis + confidence stated; carries the literal word *estimate*.
- **[UNKNOWN]** = genuinely not known; we do not fake it. Coverage is a **floor, not a ceiling.**
- **Untagged** = an arithmetic identity (e.g. landed = ex-factory × (1+duty%) + per-unit fees).

> **The one sentence to remember.** *Two policy facts — both 2025 and both bad for this venture's original shape — dominate the entire landed stack: (1) **MMF/acrylic sweaters carry a 32% base duty, ~2× cotton's 16.5% and 2× wool's 16%, which inverts the article ranking — the A2 "value" article becomes the most duty-punished of the three** [FACT — USITC]; and (2) **the de-minimis (<$800) exemption that made "ship each order direct from India duty-free" viable was suspended for ALL countries effective Aug 29, 2025, so the ship-direct-D2C model now eats full duty + a surcharge + brokered entry on every parcel** [FACT, dated]. On top of MFN duty sits a **volatile India-specific surcharge (~10% as of mid-2026, down from a 50% peak in Aug 2025, and under active litigation)** [FACT/ESTIMATE]. Net: import in **consolidated bulk to a US 3PL** (duty paid once, freight amortised, MPF capped), and **bias the line toward cotton (A1), away from acrylic (A2)** on landed-cost grounds the factory-gate model could not see.*

---

## 0. The boundary — what this number is, and what it still is NOT

This model computes **landed cost per garment at our US receiving door** = the number that becomes COGS-to-stock. It takes T4's **ex-factory** band as its input and stacks the India→California import chain on top:

```
EX-FACTORY (T4)
  → FOB Nhava Sheva / Mundra  (+ inland Ludhiana→port haulage, export clearance)
  → ocean or air freight + insurance
  → US DUTY = MFN ad-valorem (by HTS/fibre) + India surcharge (Sec 122/IEEPA)
  → MPF (0.3464%, capped) + HMF (0.125%, ocean only)
  → customs broker entry + bond + ISF + IOR setup (amortised per unit)
  → drayage / last-mile to 3PL
  = LANDED COST per garment  →  COGS-to-stock
```

It is still **NOT**:
1. **Not COGS-sold / not a P&L.** CAC, returns, payment fees, AOV, contribution margin are the **D2C survival test (T7)**. Per T0 cross-team rule #8, **contribution margin after CAC — not landed cost — decides viability.** A clean landed number can still die on CAC.
2. **Not a live quote.** Duty rates are [FACT] (USITC). Freight/broker/IOR are **[ESTIMATE] bands** from 2026 forwarder/broker reportage — get a forwarder quote before any margin commitment.
3. **Not stable.** The India surcharge layer is moving monthly through 2025–26 (litigation + trade-deal flux, §2b). The duty *base* (MFN) is stable; the *surcharge* is the volatile part — model it as a band, re-check at order time.

---

## 1. US import DUTY by HTS — the layer that re-ranks the articles

### 1a. The base MFN (Column-1-General) duty, by fibre — heading 6110

Sweaters/pullovers/cardigans = **heading 6110** (knitted/crocheted). The fibre of chief weight sets the subheading and the duty — and the spread is enormous:

| Article | HTS (representative) | Fibre | **MFN base duty** | Source |
|---|---|---|---|---|
| **A2 — fine-acrylic value** | **6110.30.30** (man-made fibre) | acrylic / MMF | **32%** ad valorem | [FACT — USITC; CBP ruling N272437] |
| **A1 — cotton-blend core** | **6110.20.20** (cotton, nesoi) | cotton-rich | **16.5%** ad valorem | [FACT — USITC] |
| **A3 — merino/lambswool premium** | **6110.11.00** (wool) | wool/merino | **16%** ad valorem | [FACT — USITC] |

> **THE headline finding — duty INVERTS the article ranking.** At the factory gate (T4) the articles ranked A2 cheapest → A1 → A3 dearest. **Duty flips the cheap one to the most-punished:** acrylic's **32%** is roughly **double** both cotton (16.5%) and wool (16%). The "value" article carries the worst duty in the line. This is not a rounding effect — on a $6 ex-factory acrylic sweater, MMF duty alone is ~$1.92 vs ~$0.99 for the same-priced cotton garment. **Surface this loudly to T7/T8: the acrylic value-driver's entire reason to exist is cost, and the US tariff wall specifically targets its fibre.** [FACT — the three rates are public; the ranking-inversion conclusion is arithmetic on them]

**Three classification cautions (binding):**
- **"Chief weight" governs.** A cotton/acrylic blend is classified by the fibre of **chief weight by mass**. A 55% cotton / 45% acrylic A1 garment is cotton (16.5%); flip to 55% acrylic and it becomes MMF (**32%**). **The blend ratio on the care label is therefore also a duty switch** — exactly the T2 §1d point, now quantified: drifting a "cotton-blend" past 50% MMF nearly **doubles** its duty. Write blend % into the PO with a tolerance *and* a duty-classification check. [FACT — chief-weight rule; ESTIMATE on the specific cliff: HIGH]
- **The 10-digit statistical suffix doesn't change the duty but changes the data line** (e.g. 6110.30.30.59 "other"). The duty is set at the 8-digit; classify carefully but the rate is as above.
- **A small wool quirk helps A3 slightly:** wool sweaters **≤ certain value thresholds** historically sat at the 16% line used here; some finer wool lines differ — confirm the exact 10-digit at order time. Held at 16% [FACT — 6110.11.00 general rate], with the specific suffix [UNKNOWN until the tech pack fixes fibre/value].

### 1b. Why this compounds, not just adds

Duty is a **percentage of a value that already includes freight on a CIF-style entered value in some structures** — but for apparel the dutiable (entered) value is normally the **FOB/transaction value**, *not* including international freight (US uses FOB-equivalent transaction value for most apparel entries). So duty applies to ~the FOB number, not the freight-inflated one. [FACT — US customs valuation is transaction value, freight to the US generally excluded if separately stated]. Still, because duty is ad-valorem, **A3's higher ex-factory value × 16% produces more absolute duty than A1's, and A2's lower value × 32% can still exceed A1's** — the percentage is what makes fibre choice a landed-cost lever, not just a factory-cost one (T4 §5 rule #3, now quantified).

---

## 2. The 2025–26 India surcharge — the volatile layer on TOP of MFN

This is the part that is **[FACT] in mechanism but moving monthly in level**, so it is modelled as a band with the timeline shown honestly.

### 2a. What it is

Since 2025 the US has layered **country-specific "reciprocal"/IEEPA tariffs ON TOP of MFN duty**. For India these stack additively with the 6110 base rate above — i.e. **total duty ≈ MFN base + India surcharge**, both applied to entered value. [FACT — additive stacking of reciprocal tariffs on MFN is how CBP has applied them]

### 2b. The timeline (read the dates — this is why it's a band, not a point)

| Date | India surcharge on apparel | Total on a cotton sweater (16.5% MFN + surcharge) | Source |
|---|---|---|---|
| Apr 2, 2025 | 25% reciprocal announced | ~41.5% | [FACT] |
| **Aug 27, 2025** | **+25% penal (Russia-oil) → 50% total surcharge** | **~66.5%** (peak) | [FACT — effective Aug 27, 2025] |
| ~Feb 6, 2026 | interim deal cuts reciprocal 25→18% | ~34.5% | [FACT — US-India interim framework] |
| **Feb 20–24, 2026** | SCOTUS strikes IEEPA basis; replaced by **~10% Section 122** | **~26.5% all-in** on cotton | [FACT — SCOTUS ruling Feb 20, 2026; Section 122 effective ~Feb 24, 2026] |
| mid-2026 (now) | **~10% Section 122 in force, under appeal; set to expire ~Jul 24, 2026** | **~26.5% cotton / ~42% acrylic / ~26% wool** | [FACT on the 10% being in force; ESTIMATE, MED that it persists] |

> **The honest read:** as of the build date (mid-2026) the India surcharge is **~10% (Section 122), down from a 50% peak**, but it is **legally unstable** — CIT ruled it unlawful (May 2026), a Federal Circuit stay keeps it in force pending appeal, and it has a statutory ~Jul-2026 sunset. **Model the surcharge as a band 0–15% for planning, central case ~10%, and treat anything above that as a tail risk that already materialised once (the 50% Aug-2025 spike).** [ESTIMATE, MED — the level is a genuine [UNKNOWN] forward; only the current ~10% and the history are [FACT]]
>
> **Strategic consequence:** the surcharge is **fibre-blind** (it hits all 6110 equally), so it does **not** change the A2-vs-A1 ranking — the **32% MMF base** already did that. But it **raises the whole stack** and makes the **de-minimis loss (§3) much more painful**, because a ship-direct parcel now pays MFN **plus** this surcharge with no $800 shield.

---

## 3. The de-minimis question — the lever that just broke (the most important single finding for a ship-direct-from-India D2C)

### 3a. What de minimis was, and why the original D2C-from-India model leaned on it

**Section 321 (19 USC 1321)** historically let shipments with a fair retail value **≤ $800** enter the US **duty-free and with minimal data**, once per person per day. [FACT] For a D2C brand this was the *magic lever*: knit in Ludhiana, hold no US inventory, and **ship each individual customer order direct from India duty-free** — every order is one parcel well under $800, so **0% duty, no broker, no formal entry**. That single rule is what made "factory in Ludhiana, customer in Turlock, nothing in between" look viable. The whole ship-direct concept silently assumed it.

### 3b. What actually happened in 2025 (the honest current state, with dates)

- **Feb 2025:** de minimis suspended for **China/Hong Kong** (then reinstated briefly, then ended).
- **May 2, 2025:** China/HK de minimis **ended**.
- **Jul 30, 2025: Executive Order 14324** signed — suspends duty-free de minimis **for ALL countries**.
- **Aug 29, 2025 (12:01 a.m. EDT): de minimis duty-free treatment ENDED for shipments from ALL countries, regardless of value, origin, or mode** (incl. India). [FACT — EO 14324; Federal Register 2025-16802, implemented Aug 29, 2025; CBP fact sheet]

> **Current state (binding, dated):** as of the build date, **there is NO de-minimis duty-free exemption for shipments from India.** A direct-to-consumer parcel from Ludhiana now **must be entered** (via an appropriate ACE entry type, except certain postal items which carry their own duty mechanism) and **pays full applicable duty** = MFN (16–32% by fibre) **+ the India surcharge (§2)**. The $800 shield is gone. [FACT, dated Aug 29, 2025]

### 3c. What this does to the venture's shape — surface this as a first-order strategic finding

The death of de minimis **removes the single biggest reason to ship direct from India** and **flips the logistics design toward bulk-import-then-3PL**:

| Model | Pre-Aug-2025 (de minimis alive) | Post-Aug-2025 (now) |
|---|---|---|
| **Ship-direct from India, per order** | 0% duty, no broker, no entry — the magic lever | **Full duty + surcharge on every parcel**, brokered/ACE entry per parcel, per-parcel MPF floor (~$33.58 formal / informal fees), slow air cost per unit. **Economically punished.** |
| **Bulk import → US 3PL → domestic ship** | duty paid (no shield benefit at bulk anyway) | **The new default.** Duty paid once at bulk, **MPF capped at $651.50 per entry** (so it amortises to near-zero per unit at volume), ocean freight amortised, domestic last-mile is cheap/fast. |

> **The clean conclusion for T7/T9 strategy:** **the de-minimis change makes "hold US inventory at a 3PL and ship domestically" the correct model**, and makes the original "zero-inventory, ship-each-order-from-India" idea uneconomic. This is a **structural** finding, not a cost tweak — it changes the operating model, the working-capital profile (T4 §4 — you now pre-commit to bulk inventory), and where the brand carries risk. **The de-minimis lever the concept assumed no longer exists; plan as a bulk importer of record.** [FACT on the rule change; ESTIMATE, HIGH on the model-flip conclusion]

---

## 4. The freight, fees, and import-setup stack — the per-unit lines

All **[ESTIMATE]** bands from 2026 forwarder/broker reportage; **get a forwarder quote before margin use.**

### 4a. Inland + FOB origin (Ludhiana is landlocked — T1's freight penalty, quantified-ish)
- **Ludhiana → port (Nhava Sheva/Mundra) inland haulage + export clearance + origin docs (ISF data, B/L).** Ludhiana's landlocked location (T1 §6) adds inland trucking + lead time vs a Tirupur (coastal) alternative. Per-unit at bulk it's small; **[ESTIMATE, LOW-MED]** ~$0.05–0.25/garment at FCL scale, more at LCL. The bigger penalty is **lead time**, not cost.
- We assume **FOB Nhava Sheva/Mundra, Incoterms 2020** as the baseline quote (T0 §6) — seller delivers export-cleared onto the vessel; we own ocean + insurance + US import from there.

### 4b. Ocean vs air (the bulk-import freight line)
| Mode | Rate band (India→LA/Oakland, 2026) | Per-garment at typical density | Use case |
|---|---|---|---|
| **Ocean FCL (40ft)** | **~$3,500–7,800 / container** | A 40ft holds **~5,000–10,000 folded sweaters** [ESTIMATE, MED] → **~$0.5–1.5/garment** | The right answer at bulk. ~24–28 days transit. [FACT — rate band] |
| **Ocean LCL** | **~$30–180 / CBM** + ~$150–300 fixed CFS/doc/ISF fees per shipment | ~30–60 sweaters/CBM → **~$1–6/garment** + fixed-fee drag on small loads | First small bulk runs; 29–35 days. [FACT — rate band] |
| **Air (consolidated)** | **~$5–8/kg** economy consolidated | a sweater ~0.3–0.5 kg → **~$1.5–4/garment** | Speed/restock/sampling, not bulk. 7–14 days. [FACT — rate band] |
| **Air express courier (DHL/FedEx/UPS)** | **~$8–12/kg**, 3–5 days | **~$3–6/garment** + per-parcel fees | Only for samples/PPS, never bulk or per-order D2C. [FACT — rate band] |

> **Freight conclusion:** **ocean FCL once volume justifies a container (~$0.5–1.5/garment); LCL for the first runs (~$1–6 + fixed fees); air only for samples.** The per-order air-from-India route (the old de-minimis model) is **~$3–6/garment of freight alone** before duty — another nail in ship-direct. [ESTIMATE, MED]
> **Insurance:** marine cargo ~**0.2–0.5% of insured value** → cents per garment. [ESTIMATE, MED]

### 4c. Government fees (these are [FACT] rates)
- **MPF (Merchandise Processing Fee), FY2026:** **0.3464% of entered value**, **min $33.58 / max $651.50 per formal entry.** [FACT — CBP, entries on/after Oct 1, 2025]
  - *The cap is the whole de-minimis-replacement story:* on a bulk entry of, say, $50,000, MPF hits the **$651.50 cap** → **~$0.13–0.65/garment** at 1,000–5,000 pieces. On a single ship-direct parcel it's the **$33.58 floor on one garment** → murderous per unit. **MPF structure alone rewards bulk and punishes per-order.**
- **HMF (Harbor Maintenance Fee):** **0.125% of cargo value, ocean only, no cap/floor.** [FACT] → ~$0.006/garment on a $5 sweater. Trivial. Air freight is **HMF-exempt.**

### 4d. Customs broker, bond, ISF, IOR setup (per-entry, amortised)
- **Customs broker entry fee:** **~$150–400 per formal entry** (+ ISF filing ~$35–75). Informal entries (<$2,500) ~$75–150. [FACT — 2026 broker reportage]
- **Customs bond:** **continuous bond ~$400–1,200/yr** (covers all imports 12 mo; break-even vs single-entry at ~3 shipments/yr) **or single-entry ~$75–275 + ISF bond ~$40–60.** [FACT]
- **New-importer / IOR setup:** broker **POA + setup ~$50–150 one-time**; the brand is the **Importer of Record** = legally liable for duty + correct HTS classification + blend-% declaration (T2 §1d duty-misdeclaration exposure is **our** liability as IOR). [FACT — IOR = legally liable party]
- **Amortised:** at a few entries/year on bulk runs, broker+bond+ISF+setup ≈ **$0.10–0.80/garment** at 1,000–5,000-piece entries; **dominant per-unit only if you do many tiny entries** (i.e. the ship-direct model again). [ESTIMATE, MED]
- **Drayage / last-mile port→3PL:** ~$300–800 per container locally → **cents/garment at FCL.** [ESTIMATE, LOW-MED]

---

## 5. Full landed cost per article — the bands [ESTIMATE]

**Method (reproducible, not authoritative):** take T4's ex-factory band as the entered (≈FOB) value; apply **MFN duty by fibre (§1, [FACT]) + central-case India surcharge ~10% (§2, [ESTIMATE])**; add **freight+insurance ocean-FCL ~$0.5–1.5 + LCL up to ~$6 (§4b)**; add **MPF/HMF (§4c) + amortised broker/bond/drayage ~$0.2–1.0/garment (§4d)** at a bulk entry. Duty is applied to the **ex-factory value only** (freight excluded from dutiable value, §1b). USD/INR ≈ ₹83–88/$ [ESTIMATE, MED]. **Bands are wide because (a) T4's ex-factory band is itself blocked by the [UNKNOWN] knit+LINK rate, and (b) the India surcharge is volatile.**

### 5a. Central case — bulk ocean import, ~10% India surcharge

| Article | Ex-factory (T4) | MFN duty | +Surcharge | **Total duty %** | Duty $ | Freight+fees+broker (bulk) | **LANDED band** | Confidence |
|---|---|---|---|---|---|---|---|---|
| **A1 — cotton core** | $4–9 | 16.5% | +10% | **~26.5%** | ~$1.06–2.39 | ~$0.8–2.5 | **≈ $6–14 / garment** | [ESTIMATE, LOW-MED] |
| **A2 — acrylic value** | $3.5–8 | **32%** | +10% | **~42%** | ~$1.47–3.36 | ~$0.8–2.5 | **≈ $6–14 / garment** | [ESTIMATE, LOW-MED] |
| **A3 — merino/lambswool premium** | $6–22+ | 16% | +10% | **~26%** | ~$1.56–5.72+ | ~$0.9–2.8 | **≈ $9–31+ / garment** | [ESTIMATE, LOW — yarn-dominated upstream] |

### 5b. The finding that jumps out of the table — duty erases A2's factory-gate advantage

- **At the factory gate A2 (acrylic) was the cheapest** (T4: $3.5–8 vs A1 $4–9). **After duty, A1 and A2 land in essentially the same band (~$6–14)** — because A2's **32% MMF duty (~$1.5–3.4)** almost exactly **eats the ~$0.5–1 ex-factory saving** it had over cotton. **The acrylic value article's cost advantage does not survive the US border.** [ESTIMATE on the bands; the mechanism (32% vs 16.5%) is FACT]
- **A1 (cotton) is the structurally friendliest article into the US** — second-lowest duty (16.5%), and its ex-factory and landed stories agree (T4 §3c). **It is the article whose whole stack — factory, duty, finish — points the same direction.**
- **A3 (premium) stays dearest and least reliable** — yarn-dominated (T4/T2), and even at the *lower* 16% wool rate its high ex-factory value makes its **absolute duty the largest** ($1.6–5.7+). Needs an importer quote on the tops before any margin use.

> **Re-ranking verdict (the deliverable's core ask):** **YES, duty changes the ranking.** Factory-gate order (cheapest→dearest) **A2 < A1 < A3** becomes, landed, **A1 ≈ A2 < A3**, with **A2 losing its lead entirely** and A1 becoming the cost-and-duty-friendly anchor. **Recommend the line lean cotton-core (A1) for margin, treat acrylic (A2) as a price-point/MOQ-flex play whose landed cost is NOT meaningfully below cotton, and keep A3 a thin premium capsule.** This is a finding T4 (factory-only) and T2 (yarn-only) structurally could not produce. [ESTIMATE, MED-HIGH on the re-rank; FACT on the duty spread driving it]

### 5c. The tail case — if the surcharge spikes again (it did once)
At the **Aug-2025 50% surcharge peak**, the same garments landed at **A1 ~66.5% / A2 ~82% / A3 ~66% total duty** — i.e. an acrylic sweater's duty alone nearly **equalled its ex-factory cost.** This is not hypothetical; it was the actual rate for ~5 months. **Model a surcharge-spike scenario in T9's P&L sensitivity; the venture must survive a return to 25–50%, not just today's 10%.** [FACT that it happened; ESTIMATE, MED that it could recur]

---

## 6. Bottom line for the owner

1. **Duty re-ranks the articles, and it's the headline.** Acrylic (A2) carries **32%** MMF duty — ~2× cotton's 16.5% and wool's 16%. **A2's factory-gate cost advantage is erased at the border**; landed, A1 (cotton) and A2 (acrylic) sit in the same ~$6–14 band, and **cotton becomes the structurally friendliest article into the US.** Lean the line cotton-core. [FACT on rates; ESTIMATE on bands]
2. **De minimis is dead (Aug 29, 2025).** The <$800 duty-free ship-direct-from-India lever the original D2C concept assumed **no longer exists for any country.** Plan as a **bulk importer of record → US 3PL → domestic shipping**, not zero-inventory ship-direct. This is a **model-flip**, not a tweak. [FACT, dated]
3. **A volatile India surcharge sits on top of MFN.** ~10% now (Section 122), down from a **50% peak (Aug 2025)**, legally unstable (SCOTUS struck IEEPA; Section 122 under appeal; ~Jul-2026 sunset). **Model it as a 0–15% band, central ~10%, with a 25–50% spike scenario** — because that spike already happened. [FACT history; ESTIMATE, MED forward]
4. **Bulk import wins on every fee, not just freight.** **MPF caps at $651.50/entry** and **broker/bond/ISF amortise per entry**, so per-order import is punished and bulk rewarded; ocean FCL freight is **~$0.5–1.5/garment**, air only for samples. [FACT on MPF cap; ESTIMATE on freight]
5. **Landed bands: A1 ≈ $6–14 / A2 ≈ $6–14 / A3 ≈ $9–31+** (central case, bulk ocean, ~10% surcharge). Wide because T4's ex-factory band is blocked by the [UNKNOWN] knit+LINK rate **and** the surcharge is volatile. **Feed these to T7/T9 as COGS-to-stock, then run the CAC/contribution test — landed cost still isn't the viability number; contribution after CAC is (T0 rule #8).** [ESTIMATE]
6. **The IOR liability is ours.** As Importer of Record we own HTS classification and **blend-% declaration** — and the **cotton/acrylic chief-weight line is a duty cliff** (cross 50% MMF and a "cotton" sweater's duty nearly doubles to 32%). Write blend % + tolerance + a duty-classification check into every PO. [FACT — chief-weight rule + IOR liability]

---

## 7. Open questions — what only a forwarder/broker quote or the next policy month can close (floor, not ceiling)

1. **The India surcharge level at *order date*** — **[UNKNOWN] forward.** ~10% now, but litigation + the ~Jul-2026 Section-122 sunset + trade-deal flux make the forward level a genuine unknown. The single most volatile input in the stack; re-check at every PO. *(Policy-dependent; never fabricate a forward rate.)*
2. **Exact 10-digit HTS suffix per article** once the tech pack fixes fibre %/value — **[UNKNOWN]**; sets the precise duty line (esp. any wool value-break on A3 and the cotton-vs-MMF chief-weight call on A1/A2). [ESTIMATE, MED that the 8-digit rates above hold]
3. **Live forwarder quote: FCL/LCL India→LA-Oakland + drayage + 3PL receiving** — **[ESTIMATE]** bands only here; a real quote tightens freight from a ~$0.5–6 range to a point. [carried from §4b]
4. **Real garments-per-FCL at our fold/pack spec** — **[ESTIMATE, MED]** (~5,000–10,000); sets the per-garment ocean amortisation precisely.
5. **Whether any India apparel line gets carve-outs** in a finalized US-India deal (post-interim-framework) — **[UNKNOWN]**; could lower the surcharge for textiles specifically. *(Policy-dependent.)*
6. **Duty drawback / RoSCTL pass-through** — does the Ludhiana exporter pass RoSCTL (extended to 31 Mar 2026, post-date [UNKNOWN] — T0 §6) into a lower FOB, and is US duty drawback worth filing on any re-exports? **[ESTIMATE, LOW]** — likely immaterial at this scale; flagged.
7. **Section 301 / other add-ons** beyond the India reciprocal/122 layer — **[UNKNOWN]**; checked and not currently found stacking on India apparel beyond §2, but the policy surface is live; re-verify at order time.
8. **The break-even import frequency** (continuous bond ~$400–1,200/yr vs single-entry) at the brand's real drop cadence — **[ESTIMATE, MED]**; continuous bond wins above ~3 entries/yr (T9 to set against the drop calendar).

---

## Manifest
- **Wrote:** `FACTORY/sweater_vertical/analysis/unit_economics/analyst_a_landed_cost.md`
- **Read:** `FACTORY/sweater_vertical/analysis/MASTER_PLAN.md`; `SCOPE_GLOSSARY.md`; `mfg_economics/DEEPDIVE.md`; `yarn/DEEPDIVE.md`; `ludhiana/DEEPDIVE.md`
- **Web-grounded [FACT]:** 6110 duty rates — MMF/acrylic **6110.30 = 32%**, cotton **6110.20 = 16.5%**, wool **6110.11 = 16%** (USITC HTS; CBP ruling N272437); **de-minimis suspended ALL countries Aug 29, 2025** (EO 14324; Federal Register 2025-16802; CBP fact sheet); India surcharge timeline (25% Apr-2025 → 50% Aug-27-2025 peak → 18% interim Feb-2026 → ~10% Section 122 post-SCOTUS Feb-20-2026, under appeal, ~Jul-2026 sunset); **MPF FY26 0.3464%, $33.58–$651.50**; **HMF 0.125% ocean**; ocean FCL ~$3,500–7,800/40ft, LCL ~$30–180/CBM, air ~$5–12/kg; broker ~$150–400/entry, continuous bond ~$400–1,200/yr, IOR setup ~$50–150.
- **Delivered (owner-facing):** (1) duty by HTS/fibre with the **32% MMF ranking-inversion** surfaced as the headline; (2) the **de-minimis death (dated Aug 29, 2025)** and its **model-flip** to bulk-import→3PL; (3) the **volatile India surcharge** band with honest timeline + spike scenario; (4) full freight/MPF/HMF/broker/bond/IOR stack as per-unit lines; (5) **landed bands A1≈$6–14 / A2≈$6–14 / A3≈$9–31+** with the explicit **A1≈A2<A3 re-ranking verdict** (A2 loses its lead); (6) bottom line + 8 open questions.
- **Honesty discipline:** all three MFN duty rates + the de-minimis end-date + MPF/HMF rates held as **[FACT]** (cited); the **India surcharge level held as a volatile band**, central ~10% **[ESTIMATE, MED]**, forward level **[UNKNOWN]**, with the 50% spike flagged as realised history not hypothetical; freight/broker/IOR all **[ESTIMATE]** bands pending a forwarder quote; landed bands inherit T4's [UNKNOWN]-knit+LINK width and are labelled illustrations, not quotes; the ranking-inversion + model-flip conclusions are arithmetic/structural on [FACT] inputs and labelled as such; no fabricated forward tariff rate; IOR/blend-% duty-cliff liability flagged as the brand's legal exposure.
- **Bottom line:** the US tariff wall specifically punishes acrylic (32% MMF), erasing A2's factory-gate advantage and re-ranking the line to **A1≈A2<A3 with cotton as the friendly anchor**; de minimis is dead so the venture must be a **bulk importer of record → 3PL → domestic ship**, not ship-direct-from-India; a volatile ~10% India surcharge (50% peak history) rides on top of MFN; landed COGS-to-stock ≈ **A1 $6–14 / A2 $6–14 / A3 $9–31+**, handed to T7/T9 where contribution-after-CAC — not landed cost — decides viability.
