# TRANSFER LOG  (cheap-probe go/no-go before committing a schema to a new domain)

### TRX_0007  schema=SCH_geo_link_react_brief  domain=tech_builders
probe: n=12 held-out tech items, metric=link_validity@anchor (pre-registered)
result: lift=+0.23 over cold-start, 95% CI [0.06, 0.39] EXCLUDES 0 → PASS · ECE=0.04
ablation: drop ALL punjab bindings → lift holds +0.19 ⇒ ACQUIRED-here, not just borrowed prior
verdict: COMMIT (schema.confidence[tech_builders]=0.71)
note: medium-tier re-measured in tech — NOT yet confirmed to peak at the medium band here (flagged → Q-019)
