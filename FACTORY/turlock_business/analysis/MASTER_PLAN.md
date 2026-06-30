# Turlock Business Intelligence — Master Plan (agent-orchestrated, file-memory)

**Operating method (per the owner's directive).** The orchestrator only *plans and injects
agents and wires files* — it does not research, analyze, or write content itself. Every unit of
real work is done by agents that **read named files and write named files**. The repeating
micro-pattern for any analysis:

```
2 independent analyst agents (different angles) ──► memory files A, B
            │
   compare/reconcile agent  (reads A + B) ──────► reconciled.md
            │
       writer agent (reads reconciled) ─────────► the artifact
```

Specialists are built the FACTORY way: agents author compact specs → `kb_forge.py` → `kb_validator.py --mode dense` → distill → `specialist_validator.py`. Gates check structure/math, never prose.

**Standing quality bar.** Revenue and government/tax figures appear **only** as (a) cited public
facts or (b) clearly *labeled benchmark estimates* (method + source + confidence). Never a
fabricated dollar figure stated as fact. Coverage is always reported honestly (floors, not ceilings).

---

## Segments (done one by one; owner may reorder)

- **S0 — DONE.** Organizer (meta) specialist `turlock_biz_organizer` + 197-business catalog (74 categories).
- **S1 — Food & Dining** *(exemplar, in progress)*: deep-dive (supply chain · per-business enrichment ·
  labeled benchmark revenue · saturation "why this many") → **Food & Dining sector specialist** (3 KBs + specialist).
- **S2–S9 — the other 8 sectors**, same pattern, one at a time: Retail & Goods · Health & Wellness ·
  Personal Care · Home & Contractor Trades · Automotive · Professional & Financial · Ag & Specialty · Ethnic Markets & Grocers.
- **S10 — Macro / Comparative Economic Brain** (the owner's big questions), each via 2-analyst → compare → write:
  - **10a** Turlock economic base & *why this many businesses, not more* (population, business density, demand math, saturation).
  - **10b** Government / tax revenue — city budget, sales & property tax, what the city actually makes (public figures + labeled estimates, sourced).
  - **10c** Growth charts — Turlock vs California vs top-growing US cities; business-formation trends.
  - **10d** Comparative city analysis & diffusion — which business types grow in which cities, which cities lead
    (NY, Austin, etc.), what Turlock is adopting / could adopt, which local sectors are growing.
  - Then a **BRAIN controller** spec wires the sector specialists + macro analysts, with a **neutralize gate**.
- **S11 — Run the brain on the owner's direction**: an *advocate* pass (the opportunity case for the owner's idea)
  and a *neutralize* pass (skeptic / balanced), → compare → a single decision memo.

## File layout

```
FACTORY/turlock_business/
  analysis/
    MASTER_PLAN.md                 (this file)
    <sector>/                      deep-dive memory: supply_ops.md, market_saturation.md, reconciled.md, DEEPDIVE.md
    macro/                         10a..10d memory + MACRO_BRAIN report
  sectors/<sector>/                each sector specialist package (3 *.spec.json/*.kb.json + *.specialist.json)
  catalog/                         the 197-business catalog (S0)
```

> Owner note: you can `/compact` between segments to keep the orchestrator sharp — state lives in
> these files, not in the orchestrator's context, so nothing is lost.
