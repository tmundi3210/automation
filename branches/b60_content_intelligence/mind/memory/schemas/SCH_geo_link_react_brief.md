---
id: SCH_geo_link_react_brief
name: Anchored 2-3 link → reaction-read → derivative brief
move: connect → read-reaction → adjudicate-hype → draft → flag        # the DOMAIN-GENERAL skill
open_slots: {audience, anchor, entities[2..3], platform_set, culture_codes}
invariants:                                                            # true in ANY domain (acquired)
  - link only via a SHARED anchor (place/topic/event), never coincidence
  - reaction buzz is a calibrated p(inflated), never a boolean
  - "medium-tier is the opportunity" is a HYPOTHESIS, re-measured per domain
  - publishable output MUST pass the compliance gate
borrowed_from: [EP_punjab_h1b_2025]                                    # priors, not proof
acquired_in: [punjab_diaspora, tech_builders]                          # domains where lift survived ablation
confidence: {punjab_diaspora: 0.82, tech_builders: 0.71}
---
SKELETON: find {{entities}} sharing {{anchor}} salient to {{audience}}; read {{platform_set}}
reactions → p(inflated) via the >=N-origin gate; draft image+audio+story using {{culture_codes}};
emit SAFETY FACTS → gate. (empty slots must be RE-BOUND per domain, never inherited.)

# tech_builders binding (committed via TRX_0007):
# anchor: a shared tool/release event | culture_codes: build-in-public/VC/founder | platform_set: +x-tech,+HN −regional-IG
