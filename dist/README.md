# Knowledge Searcher — COMPLETE Bundle

28 expert specialists + the 84 dense knowledge bases they are grounded in (all verbatim, unmodified),
plus JSON/JSONL prompt scaffolding.

## Layers
- specialists/ = 28 distilled operating specs (what you usually prompt with)
- knowledge_base/ = 84 dense KBs = the 3 subdomain knowledge bases behind each specialist (deeper grounding)

## Files
- specialists/<code>.specialist.json   — 28 specialists, AS-IS
- specialists/ROUTER.json              — routing table over the 28
- knowledge_base/<code>__<subdomain>.kb.json — 84 dense KBs, AS-IS
- knowledge_base/INDEX.json            — index of the 84 KBs
- MANIFEST.json                        — brief description per specialist (verbatim domain_label + purpose)
- KB_MANIFEST.json                     — brief description per KB (verbatim), grouped by specialist + node/edge/CQ counts
- prompt_template.json                 — reusable scaffold for using any specialist/KB as an AI prompt
- specialist_prompts.jsonl             — one self-contained prompt PER specialist (spec embedded)
- kb_prompts.jsonl                     — one self-contained prompt PER KB (KB embedded) for fine-grained subdomain queries

## Use it for an AI prompt
- Broad/expert answer -> use specialist_prompts.jsonl (pick by `code`/`route_when`).
- Deep single-subdomain answer -> use kb_prompts.jsonl (pick by `kb_file`/`specialist`).
- Either line's `messages` + embedded spec/KB IS the prompt; replace {{question}} with your question.

Boundaries are baked into each spec/KB: security = offense-aware DEFENSE only; psych/health = informational,
not clinical; uslaw = legal literacy, not advice; markets/money/business = financial education, not advice.

(The repo also holds <kb>.validation.json + <kb>.metrics.json sidecars proving each KB passed the 35/35 dense gate.)
