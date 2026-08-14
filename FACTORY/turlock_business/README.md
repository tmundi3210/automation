# Turlock Independent-Business Organizer — an autonomous catalog/specialist factory

A gate-passing **dense-mode** specialist whose job is to *run a project*: autonomously build,
categorize, deep-research, and maintain a near-complete census of **independent (non-chain)
businesses in Turlock, CA**, and then **spawn one category specialist per category** through the
same FACTORY pipeline. It is the "organize this autonomously" layer for the larger Turlock
business-intelligence effort.

One organizer specialist, grounded in **three dense knowledge bases**:

| KB | Covers |
|---|---|
| `kb1_business_discovery_methodology` | Building a near-complete **census** of independent Turlock businesses from public sources — Chamber directory, Downtown association, Yelp/Google/YellowPages, **county FBN filings**, **City of Turlock business-tax certificates**, maps and public social signals — plus entity resolution/dedup, independent-vs-chain filtering, active-vs-closed checks, provenance/confidence labeling, **capture–recapture coverage estimation**, sourcing ethics, and the handoff gate. |
| `kb2_business_categorization_taxonomy` | A **granular, deliberately not-over-generalized** category taxonomy tuned to Turlock's real economy (heavy ag/dairy; **Assyrian, Portuguese/Azorean, Mexican, Punjabi/South-Asian** communities). Split/merge granularity rules, stable category ids, primary/secondary multi-category handling, edge assignment, and the **one-category → one-specialist** mapping. |
| `kb3_business_profiling_protocol` | The per-business **deep-research** protocol — canonical profile schema, ownership/longevity, size signals, **supply-chain patterns + typical suppliers by trade**, demand/seasonality, **honest revenue estimation** (labeled benchmark band × size signal, never a stated fact), weakest-link confidence, an unknowns register, the profile QA gate, category aggregation, and the **spawn protocol** that emits a category specialist once a category holds enough grounding-quality profiles. |

The specialist `turlock_business_organizer.specialist.json` (`turlock_biz_organizer`) distills across
all three and drives the loop: **discover → dedup/verify → categorize → deep-profile → aggregate →
spawn a category specialist → maintain.**

## The honest-data rule (baked in, not optional)

Per-business **revenue is not public**. The KBs and the specialist's boundaries forbid stating any
private business's revenue/margin/financials as **fact**. Economic figures are only ever **labeled
benchmark estimates** (method named, public source, confidence band, the words *estimate*/*benchmark*
travelling with the number). Only **public** business data is used; no private/personal owner data
beyond what public records (e.g. an FBN registrant name) already disclose; source terms of service
are respected (no bypassing logins/paywalls/rate-limits/bot-blocks).

## Files

- `kb1_*.spec.json`, `kb2_*.spec.json`, `kb3_*.spec.json` — the compact content specs (**source of truth**).
- `kb1_*.kb.json`, `kb2_*.kb.json`, `kb3_*.kb.json` — the forged dense KBs (generated).
- `turlock_business_organizer.specialist.json` — the organizer specialist, grounded in the three KBs.

## Rebuild / verify (one command, from the repo root)

```bash
bash FACTORY/build.sh FACTORY/turlock_business
```

Forges each `*.spec.json` → `*.kb.json`, gates each KB in `--mode dense`, checks the specialist's
grounding, and gates the specialist. Expected: `ALL GREEN`. (`grounded_in_kbs` use **repo-relative**
paths, so run from the repository root.)

## How it was built

Authored the proper FACTORY way: helper sub-agents wrote each compact spec (model-reasoned prose +
base metrics), self-forged + self-gated each to `EXIT=0`, an adversarial critic checked each KB for
templated duplicates / coverage / the revenue-honesty rule, a revise pass fixed anything flagged, and
the organizer specialist was distilled and gated. The only programs run are the deterministic
`kb_forge.py` and the two validators — the gates check **structure + counts + reference integrity +
formula recomputation**, never prose quality, so the knowledge itself is written, not templated.

## Roadmap (this organizer drives it)

1. **Catalog** — run the organizer to compile the categorized, provenance-labeled census.
2. **Profile** — deep-research each business per category (supply chain, benchmarks, labeled revenue estimates).
3. **Spawn** — for each mature category, build a category specialist (3 dense KBs + specialist) via this same pipeline.
