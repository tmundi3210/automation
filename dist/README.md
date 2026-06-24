# Knowledge Searcher — Specialist Bundle

28 expert specialists (verbatim, unmodified) + JSON/JSONL prompt scaffolding.

## Files
- specialists/<code>.specialist.json   — the 28 specialists, AS-IS (the actual knowledge specs)
- specialists/ROUTER.json              — routing table over all 28
- knowledge_base/INDEX.json            — index of the 84 dense KBs the specialists are grounded in
- MANIFEST.json                        — brief description per specialist (their own domain_label + purpose, verbatim)
- prompt_template.json                 — reusable scaffold: how to turn any specialist into an AI prompt
- specialist_prompts.jsonl             — one fully-assembled, self-contained prompt PER specialist (spec embedded)

## Use it for an AI prompt (fastest path)
1. Open specialist_prompts.jsonl, pick the line whose `code`/`route_when` matches your question.
2. That line's `messages` (system + user) + `specialist_spec` IS the prompt. Replace {{question}} with your question.
3. Send to any chat model.

## Use it generically
Use prompt_template.json: route with router_prompt, then fill specialist_prompt_template with the
chosen specialists/<code>.specialist.json and your question.

Boundaries are baked into each spec: security = offense-aware DEFENSE only; psych/health = informational,
not clinical; uslaw = legal literacy, not advice; markets/money/business = financial education, not advice.
