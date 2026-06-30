# Sweater Vertical — Master Plan (agent-orchestrated, file-memory)

**The venture, in one line.** Build the full knowledge spine for a vertically-aware knitwear /
apparel business that runs from **yarn in Ludhiana, India** → **knitting machines** → **manufacture &
quality** → **finishing** → an **AI-designed direct-to-consumer brand selling in California / Turlock,
USA**. We are not (yet) running the business — we are building the *brain*: a set of grounded
specialists, each with dense knowledge bases, that can later be wired into one routed decision engine.

This is the same machine that built the Turlock 9-sector business brain, pointed at a new domain.

---

## Operating method (owner's directive — binding)

The orchestrator only **plans and injects agents and wires files**. It does **not** research, analyze,
or write domain content itself. Every unit of real work is done by agents that **read named files and
write named files**. The orchestrator keeps its own context light by storing state in these files and
re-reading summaries instead of holding detail.

The repeating micro-pattern for any analysis segment:

```
2 independent analyst agents (different angles) ──► memory files A, B
            │
   compare / reconcile agent  (reads A + B) ───────► reconciled.md
            │
       writer agent (reads reconciled) ─────────────► DEEPDIVE.md  (the human-readable artifact)
            │
   3 KB-author agents (read DEEPDIVE + template) ───► kb1/2/3 .spec.json → forge → dense-gate
            │
       distiller agent (reads the 3 KBs) ───────────► <segment>.specialist.json → gate
            │
   NEUTRALIZE critic (reads everything) ────────────► questions every KB + the specialist; strips hype,
                                                       tags claims, demands sources; re-gates ALL GREEN
```

Specialists are built the FACTORY way (mirrors `FACTORY/turlock_business/sectors/food_dining/`):
compact `*.spec.json` → `python3 branches/_forge/kb_forge.py SPEC -o KB.json --quiet` →
`python3 validators/kb_validator.py KB --mode dense` → distill `*.specialist.json` →
`python3 validators/specialist_validator.py SPEC`. One command: `bash FACTORY/build.sh <dir>`
(run from repo root). **Gates check structure + math + reference integrity only, never prose quality** —
so the neutralize critic and orchestrator spot-checks are what guard real content.

---

## Honesty discipline (binding — and stricter here than Turlock)

Every claim about money, capacity, regulation, or behavior carries a tag: **[FACT]** (cited public
source), **[ESTIMATE]** (method + basis + confidence + the literal word "estimate"), or **[UNKNOWN]**.

This domain has a large **off-paper economy** the owner explicitly wants captured — "*political bias,
political bribery, how much they earn… maybe it's all not in papers, but it can be in discussions.*"
Rules for that material:

- It is described as **mechanism, not accusation**. We explain *how* such practices are generally
  reported to operate in industrial clusters (the structural incentives), tagged **[ESTIMATE:
  discussion/industry-lore]** with confidence stated.
- We **never** state, as fact, that a **named** person or company pays bribes, evades tax, or is
  politically captured. No fabricated dollar figures for a named firm's earnings — only **labeled
  benchmark estimates** (method + public anchor + confidence).
- Coverage is reported as a **floor, not a ceiling**. "Unknown" is a valid, frequent answer.

---

## The segments (built one by one; owner may reorder)

Grouped into four clusters along the value chain. Each segment becomes a gated specialist
(3 dense KBs + 1 specialist) plus a `DEEPDIVE.md`, except T0 (organizer) and T11 (controller).

### Cluster A — SOURCING (where raw value comes from)
- **T0 — Scope & Glossary / Organizer.** Defines the whole vertical and the shared vocabulary every
  downstream agent must use (yarn count Ne/Nm, GG gauge, denier, GSM, acrylic/wool/cotton/blends,
  fully-fashioned vs cut-&-sew, linking, fully-fashioned 3-thread, etc.). Output:
  `SCOPE_GLOSSARY.md` + organizer specialist `sweater_vertical_organizer`.
- **T1 — Ludhiana cluster.** Industrial geography; the biggest groups (e.g. Vardhman, Nahar, Oswal,
  Sportking…); *how they survive*; labour & living cost; rules/regulation; the **off-paper political
  economy** (bias, bribery, informal practice) handled per the honesty discipline; and **where the
  economically-better alternatives are** (Tirupur, Panipat, Bhilwara; Vietnam, Bangladesh, China).
- **T2 — Yarn & fibre.** Fibre choice for sweaters (acrylic / wool / merino / cotton / blends), yarn
  count & quality grading, *where the best yarn is from*, sourcing, MOQ, pricing, the yarn→knit pipeline.

### Cluster B — PRODUCTION (turning yarn into goods)
- **T3 — Machines.** Flat-bed knitting (Shima Seiki, Stoll), circular, linking/finishing machines;
  *where made, how made*; new vs used cost; **can we build/design one** (reverse-engineering, AI-assisted
  custom design, making our own parts, cost to do so); throughput economics.
- **T4 — Manufacturing economics.** Full per-unit cost model (machine, labour, overhead, yield);
  make-vs-buy; where to manufacture; capacity & batch planning for a small line.
- **T5 — Quality & the "thread checker."** Yarn-evenness & fabric-inspection tech (Uster-class testers,
  computer-vision defect detection); the owner's idea of a device that **grades thread-to-thread, records
  it, and sends a code back to the supplier** — feasibility, build-vs-buy, cost, what's actually possible.
- **T6 — Post-treatment / finishing.** Washing, anti-pill, softening, shrink control, dyeing — what to do
  *after* buying yarn/garment to reach best quality.

### Cluster C — MARKET (turning goods into a brand & sales)
- **T7 — Brand & California/Turlock market.** D2C positioning; the local consumer & where they get news;
  trend/hype signals; **brand naming** (athlete / event / meme-driven, tied to current California hype);
  trademark & legal basics.
- **T8 — Design specialist (AI-driven).** What designs work; sizing norms & grading **math**; the
  **3-article line plan**; fashion-show / runway trend ingestion; and an actual **generative-design
  capability** — a specialist that writes image prompts (Flux API, key supplied by owner), applies sizing
  math, and produces **samples for the owner to choose** (e.g. a youth-leaning capsule).
- **T9 — Logistics & end-to-end unit economics.** Ludhiana→California shipping, duties/HTS, landed cost,
  pricing ladder, the full P&L from yarn to a sold garment.

### Cluster D — DECISION (the brain)
- **T10 — Macro / neutralize brain.** An **advocate** pass (the opportunity case) and a **neutralize**
  pass (skeptic / balanced) over the whole venture → compare → a single **decision memo**: is it viable,
  what's the best configuration, where the money actually is, what kills it.
- **T11 — BRAIN controller.** Wire all specialists into one routed engine with a **neutralize gate** so a
  single question dispatches to the right specialist(s) + the macro layer.

---

## File layout

```
FACTORY/sweater_vertical/
  analysis/
    MASTER_PLAN.md                 (this file)
    SCOPE_GLOSSARY.md              (T0 shared vocabulary — every agent reads this)
    PROGRESS.md                    (orchestrator's running state / what's done / what's next)
    <segment>/                     deep-dive memory: analyst_a.md, analyst_b.md, reconciled.md, DEEPDIVE.md
    macro/                         T10 advocate/neutralize memory + DECISION_MEMO.md
  specialists/<segment>/           each specialist package (3 *.spec.json/*.kb.json + *.specialist.json)
```

Template every KB-author mirrors for structure: `FACTORY/turlock_business/sectors/food_dining/`.
Dense bands (unchanged): nodes 19–24, edges 32–40, conflict_axes 8–10, edge_cases 10–12,
workflow 9–12, competency_questions 10–14, dominance_rules 7–12, anti_rework_rules 7–12,
iteration_protocol 6–10, glossary ~10. Dependencies acyclic; formula nodes must recompute.

## Build order & cadence

Series, one segment at a time, with the orchestrator `/compact`-ing between segments (state lives in
files, so nothing is lost). Wave 1 = **T0 (scope/glossary)** + **T1 (Ludhiana)** in parallel, since
Ludhiana research doesn't depend on the glossary. Then T2 → … → T9, then the brain (T10, T11).

> Additive-only: this project never touches existing validated work (Turlock specialists, ROUTER, dist/,
> phase_b/, movie work). It only adds files under `FACTORY/sweater_vertical/`.
