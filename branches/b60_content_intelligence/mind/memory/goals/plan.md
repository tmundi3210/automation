# PLAN for G-12  (term-budget: max_steps=8)  plan_hash:a1b2c3

| step | dep | mode     | status | desc |
|------|-----|----------|--------|------|
| S1   | -   | SERIES   | DONE   | collect rising AI-agent items (medium-tier band) |
| S2   | S1  | PARALLEL | DONE   | read reactions across youtube/reddit/x-tech (independent fan-out) |
| S3   | S2  | SERIES   | DONE   | apply hype-adjudication playbook: provenance-dedup → ≥N-origin gate |
| S4   | S3  | SERIES   | DONE   | draft brief via SCH_geo_link_react_brief re-bound to tech_builders |
| S5   | S4  | SERIES   | DONE   | compliance gate (sole predecessor of emit) → GREEN → emit |
| S6   | S5  | SERIES   | DONE   | reflect + consolidate (belief Δ, prediction, lesson) |

ranking (what strictly decreases each cycle): open steps remaining (6→0)
re-plan triggers: ledger-exhausted | step-blocked | metric off-track ×K | assumption-broke | budget-low
