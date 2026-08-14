# link/ — 2-3 entity scene linker (grounded in `salience_heavy` + `reason`)

`entity_link.py` builds the best geo-licensed scene anchored on one node:

- **typed-tag join** (PLACE:punjab ≠ TOPIC:punjab) → a relatedness score used only to **rank**.
- a **boolean geo-relevance predicate** (overlapping `geo_key`) **licenses** a join — topical
  temptation is not enough.
- the link is **abductive generate-and-score**: a proposer emits candidates, a *separate*
  verifier scores them; structural geo/tag signal is weighted **above** the verifier so an
  un-grounded model can't manufacture a spurious link past the predicate.
- **arity hard-enforced to {2,3}**; arity-3 requires triad closure (every pair geo-licensed).

The schema-allowed `suggested_link_partners` keys are written to the node; full diagnostics
stay on the returned scene. In the demo the safe news partner (3 shared geo keys) wins over the
real artist (1 shared key).

```bash
python3 link/entity_link.py
```
