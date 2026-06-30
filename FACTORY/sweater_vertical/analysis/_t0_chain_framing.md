# T0 — Business / Value-Chain Glossary (`_t0_chain_framing.md`)

**Scope.** This is the *business* vocabulary every downstream agent (T1–T11) shares: the chain stages
and who owns each, trade/sourcing/Incoterms terms, manufacturing-model terms, D2C/brand-finance terms,
and Ludhiana-context terms. The **technical** textile vocabulary (Ne/Nm count, GG gauge, denier, GSM,
GSM, fully-fashioned vs cut-&-sew, linking, fibre chemistry) lives in the sibling file
`SCOPE_GLOSSARY.md` — this file references those but does not redefine them.

**Honesty tags (binding).** `[FACT]` = cited public source. `[ESTIMATE]` = method + basis + confidence,
uses the literal word "estimate". `[UNKNOWN]` = not reliably known. `[ESTIMATE: discussion/industry-lore]`
= off-paper mechanism described structurally, never as a named-party accusation. Coverage here is a
**floor, not a ceiling** — "unknown" is a valid answer and is used.

---

## 1. The value-chain stages (and who typically does each)

The spine: **fibre → spinning → yarn → knitting → linking → finishing → QC → packing → export →
import → D2C retail.** Stage definitions below are structural [FACT] (standard apparel-supply-chain
description); the "who does it in Ludhiana" column blends [FACT] cluster structure with
[ESTIMATE: industry-lore] on typical ownership boundaries.

| Stage | One line | Who typically does it (Ludhiana → US lens) |
|---|---|---|
| **Fibre** | Raw input: acrylic staple (petrochemical), wool/merino (animal), cotton (agri), blends. | Acrylic from petrochemical majors (e.g. Vardhman/IPCL-type feedstock chains); wool largely imported (Australia/NZ); cotton from Indian belts. [FACT] composition; [ESTIMATE] sourcing split. |
| **Spinning** | Fibre twisted into yarn of a target count (Ne/Nm) and ply. | Large integrated spinners (Vardhman, Nahar, Sportking, Oswal) or independent spinning mills; some knitters run **captive spinning**. [FACT] firms exist; [ESTIMATE] who-spins-for-whom. |
| **Yarn (trade)** | Finished yarn sold by count/shade/lot, on cones, to knitters. | Yarn merchants/agents + mill direct sales in the Ludhiana yarn market. [ESTIMATE: industry-lore]. |
| **Knitting** | Yarn → fabric/panels: flat-bed (Shima Seiki, Stoll) fully-fashioned, or circular, then cut. | Thousands of MSME knitting units; ~70% are micro. [FACT] (≈14,000 MSMEs, 70% micro — see §6). |
| **Linking** | Joining knitted panels (shoulder, side seams) on a linking machine; defines a "fully-fashioned" sweater. | Specialist linking sub-units / in-house finishing rooms; often **job-work** subcontracted. [ESTIMATE]. |
| **Finishing** | Wash, anti-pill, softening, shrink control, dye/over-dye, pressing, mending. | Dye-houses + finishing job-workers; environmentally regulated (effluent). [ESTIMATE]; covered in T6. |
| **QC** | Inspect yarn evenness, fabric defects, seams, measurements vs spec; AQL sampling. | In-house QC + buyer's third-party inspector (e.g. for export orders). [FACT] AQL is standard; T5. |
| **Packing** | Fold, polybag, tag, carton, mark per buyer's packing list. | The manufacturer/exporter. [FACT] standard. |
| **Export** | Clearing goods out of India: invoice, packing list, HS code, Incoterm, shipping bill, drawback/RoSCTL claim. | Manufacturer-cum-exporter or a merchant exporter / buying house. [FACT] documents standard. |
| **Import (US)** | Customs entry into the US: HTS classification, duty, customs broker, FDA/CPSC labeling for apparel. | US importer of record = the brand or its 3PL/broker. [FACT] structure. |
| **D2C retail** | Selling the finished garment directly to the end consumer online/locally — no wholesale middleman. | The brand (our venture) in California/Turlock. [FACT] definition; economics in T7/T9. |

**Key structural note.** A single Ludhiana **"manufacturer-cum-exporter"** can vertically span
spinning→export; a thin venture instead buys yarn or **CMT** capacity and owns only design + brand +
US import. Which boundary we sit at is the central make-vs-buy question (T4/T9).
[ESTIMATE: design-framing, high confidence it's the key lever].

---

## 2. Trade / sourcing / Incoterms terms

Incoterms® 2020 are 11 ICC-defined rules splitting cost/risk between seller and buyer; the four below
are the ones a yarn/garment importer actually negotiates. Definitions [FACT] (ICC / trade.gov).

| Term | One line | Relevance to us |
|---|---|---|
| **EXW** (Ex Works) | Seller just makes goods available at its premises; **buyer bears all cost & risk** from the factory gate. | Cheapest quoted price but we'd arrange Ludhiana→port→US ourselves; max control, max hassle. [FACT]. |
| **FOB** (Free On Board) | Seller delivers, cleared for export, **onto the vessel**; risk/cost passes to buyer there. | The common apparel quote; we own ocean freight + insurance + US import. Baseline for landed-cost math. [FACT]. |
| **CIF** (Cost, Insurance, Freight) | Seller pays freight + insurance **to destination port**, but **risk still passes at origin** on loading. | Convenient early on; note insurance is minimum-cover and risk is already ours mid-ocean. [FACT]. |
| **DDP** (Delivered Duty Paid) | Seller bears everything incl. US customs clearance + duty to the named destination. | Lowest-effort for us, highest price; rarely offered by small Indian units for US duty. [FACT]. |
| **Incoterms** | The ICC rule-set itself; must be cited with year + named place (e.g. "FOB Nhava Sheva, Incoterms 2020"). | Every PO must name one; ambiguity = disputes over who pays freight/duty. [FACT]. |
| **MOQ** (Minimum Order Quantity) | Smallest quantity a supplier will run (per style/colour/yarn lot). | Gatekeeper for a small D2C line; high yarn/knit MOQs force capsule sizing (T8) or job-work. [FACT concept]; specific MOQs [UNKNOWN until T2]. |
| **Lead time** | Calendar time from PO (or yarn-in) to goods-ready / to delivered. | Drives drop calendar + cash tied up; ocean India→US adds weeks (T9). [FACT concept]. |
| **Landed cost** | True per-unit cost delivered to our US door: FOB price + freight + insurance + duty + broker + drayage + financing. | The number that actually sets margin; the whole T9 model computes it. [FACT definition]. |
| **HS code / HTS code** | HS = 6-digit global product code; **HTS** = US 10-digit extension setting the duty rate. Sweaters/pullovers/cardigans = **heading 6110**. | Determines our import duty; mis-classification = penalties or overpaid duty. [FACT] (6110 = sweaters/pullovers, USITC). |
| **Duty / tariff** | Tax on import, % of declared value (ad valorem). Cotton sweaters 6110.20 ≈ **7%**; wool 6110.11 higher (~16% range). | Direct cost line in landed cost; fibre choice changes the duty. [FACT] 6110.20 general ≈7%, 6110.11 ~16% (USITC/Flexport/UNIS); exact line + any 2025–26 add-on tariffs to confirm in T9. |
| **Duty drawback (DBK)** | Indian rebate of **customs duty paid on imported inputs** used to make an exported good. | Lowers the exporter's effective cost → can lower our FOB if passed through. [FACT] (DGFT/CBIC). |
| **RoDTEP / RoSCTL** | Indian export rebate schemes. For apparel (HS ch. 61/62/63) **RoSCTL** applies and **RoDTEP is not claimed on the same goods**. RoSCTL extended through 31 Mar 2026. | Affects how cheaply a Ludhiana exporter can quote us; a sourcing-leverage point. [FACT] (AEPC/DGFT); post-2026 continuation [UNKNOWN]. |
| **LC / Letter of Credit** | Bank guarantee that the seller is paid once it presents shipping docs matching the LC terms. | De-risks first deals with an unknown Ludhiana supplier; costs bank fees + ties working capital. [FACT]. |
| **FCA / DAP / CFR** | Other Incoterms (FCA = handed to carrier export-cleared; CFR = CIF minus insurance; DAP = delivered, duty unpaid). | May appear in quotes; know them to compare apples-to-apples. [FACT]. |
| **Merchant exporter / buying house** | Intermediary that aggregates factory output and exports under its name. | An alternative to dealing factory-direct; adds margin but eases compliance for a first-timer. [ESTIMATE: industry-lore]. |
| **Customs broker / IOR** | Licensed US agent filing the entry; **Importer of Record** = legally liable party for duty/compliance. | The brand is usually IOR; broker fee is a real landed-cost line. [FACT]. |

---

## 3. Manufacturing / business-model terms

| Term | One line | Relevance |
|---|---|---|
| **CMT** (Cut–Make–Trim) | Factory is paid only for labour to cut, sew/link, and trim; **buyer supplies the fabric/yarn**. | Lets a thin brand own materials + design and rent only stitching capacity; lowers capital, keeps IP. [FACT industry term]. |
| **Job-work** | Indian term for subcontracted processing (knit/link/dye) where the principal owns the material and pays a conversion charge. | The connective tissue of the Ludhiana cluster; how MSMEs share specialised steps. [FACT/industry usage]. |
| **FOB manufacturing / full-package (FPP)** | Factory sources its own yarn and delivers finished, packed goods at an FOB price. | Simplest for us (one price, one vendor) but we lose material control + transparency. [FACT industry term]. |
| **Vertical integration** | One firm owns multiple consecutive chain stages (e.g. spinning→knitting→finishing→export). | Big Ludhiana groups do this; a small brand mimics it only via tight contracts, not ownership. [FACT concept]. |
| **Captive spinning** | A knitter/garmenter runs its own spinning so it controls yarn count, shade, and cost in-house. | Explains why integrated groups can undercut yarn-buyers; relevant to T2/T4 cost gaps. [FACT concept]. |
| **Captive power** | Self-generated electricity (diesel/gas/solar/cogen) to dodge unreliable or costly grid supply. | A real Ludhiana cost/uptime factor; affects unit cost + ESG story (T1/T4). [FACT concept; magnitudes T1]. |
| **Make-vs-buy** | The decision to produce a step in-house vs purchase it. | The recurring lever across this venture (own a machine? own spinning? own knitting?). [FACT framing]. |
| **Sampling / pre-production sample (PPS)** | Approval garments (proto → fit → PP) signed off before bulk. | Gates quality before committing yarn money; a lead-time + cost item often underestimated. [FACT industry term]. |
| **Tech pack** | The spec sheet: measurements, yarn count/shade, GSM, stitch, trims, tolerances, packing. | The hand-off doc from design (T8) to factory; precision here = fewer QC failures. [FACT industry term]. |

---

## 4. D2C / brand-finance terms

Definitions [FACT] (standard retail/finance usage). Any *numbers* are [UNKNOWN] until T7/T9 model them.

| Term | One line | Relevance |
|---|---|---|
| **SKU** (Stock-Keeping Unit) | One uniquely sellable variant = style × colour × size. | A 3-style capsule in 4 sizes × 3 colours = 36 SKUs — drives MOQ pain + inventory math. [FACT]. |
| **Line plan** | The planned matrix of styles/colours/sizes/prices for a season. | T8 deliverable; ties design to MOQ, margin, and the drop calendar. [FACT]. |
| **Capsule** | A small, cohesive set of styles meant to mix-and-match. | The venture's deliberately tiny launch unit (e.g. youth capsule) to keep MOQ + risk low. [FACT]. |
| **Drop** | A timed, often limited release of product (vs always-in-stock). | Hype-friendly D2C tactic fitting a California/meme brand (T7); manufactures scarcity + urgency. [FACT/marketing usage]. |
| **MSRP** | Manufacturer's Suggested Retail Price = the sticker price to the consumer. | Top of our pricing ladder; everything below is cost/margin structure. [FACT]. |
| **COGS** (Cost of Goods Sold) | The direct cost of the unit sold = landed cost + any direct fulfilment-to-stock cost. | Denominator of margin; built from the T9 landed-cost model. [FACT]. |
| **Gross margin** | (Revenue − COGS) / Revenue, as %. | Headline product profitability before marketing/ops. [FACT]. |
| **Contribution margin** | Price − **all** variable costs (COGS + payment fees + shipping-to-customer + returns + variable CAC). | The honest per-order profit that must cover fixed costs; the metric that actually decides D2C viability. [FACT]. |
| **CAC** (Customer Acquisition Cost) | Marketing/sales spend ÷ new customers acquired. | For paid-ads D2C this often eats the whole margin; the make-or-break number (T7). [FACT]. |
| **AOV** (Average Order Value) | Total revenue ÷ number of orders. | Higher AOV (bundles, capsule) offsets CAC and shipping; a key lever. [FACT]. |
| **Sell-through** | Units sold ÷ units received, over a period. | Measures whether a drop "worked"; low sell-through = trapped cash + markdowns. [FACT]. |
| **LTV** (Lifetime Value) | Total contribution margin a customer yields over their lifetime. | LTV:CAC ratio (>~3 healthy) is the survival test for a D2C brand. [FACT/industry rule-of-thumb]. |
| **Markdown / clearance** | Price cut to move unsold inventory. | The penalty for over-ordering against MOQ; directly attacks gross margin. [FACT]. |
| **Working capital / cash conversion cycle** | Cash tied up between paying for yarn and getting paid by customers. | Long ocean lead time + upfront MOQ = a cash trap that can kill a profitable-on-paper brand. [FACT framing]. |

**Worked illustration (labelled estimate, not a forecast).** [ESTIMATE: arithmetic illustration, method =
plug assumed values into the identities above, confidence n/a — these are placeholder inputs, not
researched figures]: if MSRP = $80, landed COGS = $20, payment+ship+returns = $10, and blended CAC = $25,
then gross margin = 75% but **contribution after CAC ≈ $25/order (~31%)** — showing why CAC, not factory
cost, usually governs survival. Real inputs come from T9.

---

## 5. Ludhiana-context terms

| Term | One line | Relevance |
|---|---|---|
| **"Hosiery"** | In Ludhiana usage, **"hosiery" = the knitwear industry broadly** (sweaters, tees, thermals) — *not* just socks. | Avoids a vocabulary trap: a "hosiery unit" is a knitwear maker. [FACT] (local industry usage; "winter hosiery"). |
| **"Manchester of India"** | Common epithet for Ludhiana as a textile/knitwear hub. | Signals scale + heritage; useful brand-story framing and a [FACT] that the cluster is dominant. [FACT] (popular usage; cluster docs). |
| **Cluster / agglomeration** | A dense geographic concentration of firms in one trade sharing labour, suppliers, and know-how. | Ludhiana's core advantage: spinners, knitters, dyers, machine dealers, job-workers all within reach → low coordination cost. [FACT] (cluster economics; Sameeeksha). |
| **MSME** | Micro/Small/Medium Enterprise (Indian statutory size class). | The cluster is ~14,000 MSMEs, ~70% micro — explains fragmentation, low MOQ flexibility per unit, and reliance on job-work. [FACT] (Sameeeksha cluster profile). |
| **Captive power** | (see §3) Self-generation to beat grid unreliability/cost. | A recurring Ludhiana cost line and resilience factor. [FACT concept]. |
| **Job-work ecosystem** | The web of subcontracting that lets micro-units specialise in one step. | Means a brand can assemble a full garment across many tiny vendors — flexible but QC-fragmented (T5). [ESTIMATE: industry-lore, high confidence]. |
| **Yarn market / mandi** | The physical/relational marketplace where yarn is traded by count/shade/lot. | Where pricing + MOQ are actually negotiated (T2); relationship- and cash-driven. [ESTIMATE: industry-lore]. |
| **Off-paper economy (mechanism)** | Portions of activity (cash sales, informal labour, under-/over-invoicing, "facilitation") that sit outside formal books. | The owner explicitly wants this captured. Described as a **structural incentive**, never as a named-party act. [ESTIMATE: discussion/industry-lore]. |

**Off-paper mechanism — how it is *generally reported* to operate in fragmented industrial clusters
(structural, not an accusation about any named firm):** thin margins + high competition + cash-heavy
local trade + fragmented micro-units create incentives for informal cash transactions, informal/contract
labour, and discretionary interaction with inspectors/regulators where enforcement is patchy. We state
this as a *mechanism* and a hypothesis to probe in T1; we do **not** assert that any specific person or
company does any of it. [ESTIMATE: discussion/industry-lore; confidence MEDIUM that such mechanisms exist
in clusters generally, LOW/UNKNOWN on magnitude in Ludhiana specifically — to be revisited, never
fabricated, in T1].

---

## 6. Anchored facts used above (sources)

- **Cluster size/structure:** ~14,000 MSMEs in Ludhiana textiles, ≈70% micro / 20% small / 10% medium;
  knitwear manufacturers-cum-exporters at the core, with spinners/dyers/job-knitters as backward support.
  [FACT] — Sameeeksha Ludhiana knitwear cluster profile (sameeeksha.org).
- **Historic dominance:** early-1990s Ludhiana held >80% of India's woollen-knitwear firms and >90% of
  woollen/acrylic knitwear output. [FACT] — academic cluster study (ScienceDirect / World Development).
- **HTS heading:** sweaters/pullovers/sweatshirts/cardigans = **6110**; cotton 6110.20 general duty ≈ **7%**,
  wool/fine-hair 6110.11 ~16% range. [FACT] — USITC HTS, Flexport, UNIS/Datamyne tariff data. *Any 2025–26
  US tariff surcharges/Section-301-type add-ons are [UNKNOWN] here and must be confirmed in T9.*
- **Incoterms 2020 (EXW/FOB/CIF/DDP):** cost/risk split per ICC. [FACT] — trade.gov "Know Your Incoterms".
- **Export rebates:** DBK rebates customs duty on imported inputs; for apparel (ch. 61/62/63) **RoSCTL**
  applies and RoDTEP is not double-claimed; RoSCTL extended to 31 Mar 2026. [FACT] — DGFT/CBIC, AEPC FAQ.

Source links:
- https://sameeeksha.org/index.php?option=com_content&view=article&id=139&Itemid=502
- https://www.sciencedirect.com/science/article/abs/pii/S0305750X99000790
- https://hts.usitc.gov/search?query=6110
- https://www.flexport.com/data/hs-code/6110-sweaters-pullovers-sweatshirts-waistcoats-vests-and-similar-articles-knitted-or-crocheted/
- https://www.trade.gov/know-your-incoterms
- https://content.dgft.gov.in/Website/EPS.pdf
- https://www.aepcindia.com/system/files/FAQ%20on%20RoSCTL.pdf

---

## 7. Coverage statement (floor, not ceiling)

This glossary covers the **shared business vocabulary** needed for T1–T11 to interoperate. It deliberately
does **not** quote firm-level revenues, specific MOQs/prices, or current-year tariff surcharges — those are
`[UNKNOWN]` here by design and are the jobs of T1 (Ludhiana), T2 (yarn), T9 (logistics/unit economics).
Where industry behaviour is described it is tagged `[ESTIMATE: discussion/industry-lore]` and framed as
mechanism. Downstream agents should treat every `[FACT]` here as a citeable anchor and every `[ESTIMATE]`
as a hypothesis to harden or refute.
