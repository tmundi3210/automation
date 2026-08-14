# creative/ — generative-brief compiler (grounded in `creative_heavy`)

`gen_brief.py` composes a closed, versioned **`gen_brief`** artifact from a 2-3 node scene:
required image block + required story block (setup/turn/punchline) + optional audio block, an
in-band **contract block** (`disclosure_slot`, `usage_restrictions`, assume/guarantee,
`derivative_risk_flag`), and a **provenance block** (every claim cites a `source_fact_id`;
ungrounded subjects are rejected). The 2-3 beat = one contrast axis, one benign-violation joke,
rule-of-three.

Two fail-closed precondition: an empty `disclosure_slot` raises on serialize; an ungrounded or
non-public subject raises before any brief is built. **This stage briefs only — it never renders
or publishes media.**

```bash
python3 creative/gen_brief.py
```
