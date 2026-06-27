# compliance/ — path-to-yes clearance gate (grounded in `compliance_heavy`, an enabler)

`clearance.py` evaluates each scene member's frozen `(item, figure, use)` tuple into a
**Clearance Record** disposition and writes the schema `safety_flags` block.

- **Green lanes (maximize what ships):** own-persona / original composite → ALLOW; consented →
  ALLOW_WITH_CONSTRAINTS; **parody-with-visible-cues** → the default when consent is absent;
  public topic/policy/institution → ALLOW (+ constraints if political).
- **Hard blocks (never auto-approved):** minor/protected → BLOCK; real person + political/election
  + voice/likeness → BLOCK; fabricated wrongdoing → BLOCK; scraped + unknown license → HUMAN_REVIEW.
- On a passing scene it signs a **C2PA-style disclosure manifest** and emits an in-band token
  bound to the gen_brief's `disclosure_slot`; `verify_token` is the **handoff re-check**.

The scene disposition is the worst member disposition; only ALLOW / ALLOW_WITH_CONSTRAINTS pass.
**Screening heuristic, not legal advice** — BLOCK/HUMAN_REVIEW route to counsel.

```bash
python3 compliance/clearance.py    # safe-lane scene passes + signs; block-trigger → BLOCK
```
