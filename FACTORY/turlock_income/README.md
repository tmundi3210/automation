# Central Valley Income Strategist — a worked dense-mode specialist

A complete, gate-passing **dense-mode** specialist built with the FACTORY pipeline,
covering how to earn more in Turlock and the wider Central Valley of California.
One specialist, grounded in **three dense knowledge bases**:

| KB | Covers |
|---|---|
| `kb1_turlock_wage_jobs` | The local wage-job income ladder — which sectors and employers pay more, realistic pay bands, low-barrier entry, short-credential higher-wage targets, where jobs are actually filled, total-comp vs wage, gig vs W-2, and a dated income plan. |
| `kb2_paid_driving_credentials` | Paid driving — CDL classes A/B/C and endorsements (hazmat/tanker/doubles/passenger/school-bus), which pay more vs which need least experience, DMV/medical/permit/ELDT steps, company-sponsored training, company-driver vs owner-operator, **and the California licensed driving-instructor path with a dual-control vehicle** (e.g. a 2018 Corolla). |
| `kb3_ca_service_contract_business` | Starting a California service business — cleaning, landscaping/grass-cutting, sign hanging, **towing**, and **baking** — entity/registration, trade licenses & permits (CSLB, food, tow carrier), insurance/bonding, equipment per trade, finding & bidding contracts (**including how big retailers like Walmart actually contract through national facilities-management vendors**), and pricing. |

The specialist `central_valley_income.specialist.json` distills across all three and
routes a person to the best path (better job / driving credential / service business)
with a dated, self-funding plan.

## Files

- `kb1_*.spec.json`, `kb2_*.spec.json`, `kb3_*.spec.json` — the hand-authored compact
  content specs (prose + base metrics). **These are the source of truth.**
- `kb1_*.kb.json`, `kb2_*.kb.json`, `kb3_*.kb.json` — the forged dense KBs (generated).
- `central_valley_income.specialist.json` — the specialist, grounded in the three KBs.

## Rebuild / verify (one command, from the repo root)

```bash
bash FACTORY/build.sh FACTORY/turlock_income
```

This forges each `*.spec.json` → `*.kb.json`, gates each KB in `--mode dense`,
checks the specialist's grounding, and gates the specialist. Expected: `ALL GREEN`.

> The specialist's `grounded_in_kbs` use **repo-relative** paths, so run the build and
> the gates **from the repository root**.

## How it was built (no content-generating scripts)

Every spec was **hand-authored** (model-reasoned prose). The only programs run were the
deterministic `kb_forge.py` (math/derived fields) and the two validators. The gates check
**structure + counts + reference integrity + formula recomputation** — never prose quality
— so the knowledge itself was written, not templated.

## Important

The KBs give **reasoned bands and method**, not quoted figures. Confirm every wage, fee,
threshold and eligibility rule against the current official source — EDD, CA DMV, FMCSA,
TSA, CSLB, CDTFA, county health, CHP — before relying on it.
