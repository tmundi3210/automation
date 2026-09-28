# Enrichment brief for the new pack lessons

The new lessons are `FACTORY/cdl_california/source/packs/<ID>-*.md` (written from the CA handbook DL 650; they are the ONLY source of truth for this task). The compiled content is `FACTORY/cdl_california/app/src/content/content.json` (objects `concepts`, `items`, `numbers`, keyed by id; filter by `.lesson`). Handbook page text: `/tmp/claude-0/-home-user-automation/4f22bdf7-6382-586c-a88b-74190a187585/scratchpad/pdf/pages/<label>.txt`.

For EACH assigned lesson write 3 files (look at the existing GK/CV files of the same kind for style):

## 1. `FACTORY/cdl_california/enrich/summaries/<ID>.json` — key points per concept
Follow `/tmp/claude-0/-home-user-automation/4f22bdf7-6382-586c-a88b-74190a187585/scratchpad/SUMMARY_BRIEF.md` exactly, except the lessons are in `source/packs/`. `{ "<ID>.c01": ["bullet", ...], ... }` — every concept id of the lesson, 2–5 bullets each, ≤30 words, **bold** key number/term, `[CA]` prefix for CA rules.

## 2. `FACTORY/cdl_california/enrich/<ID>.json` — distractor tags + number questions
```
{ "distractorTags": { "<itemId>": ["trap_named", null, "neighbor_number"], ... },
  "numberItems": [ { "numberId": "<numbers id>", "stem": "...?", "options": ["..","..",".."], "key": 1, "tags": [..3, null at key..], "explanation": "... (p. X-Y)", "verified": true } ] }
```
- `distractorTags`: for EVERY item with `origin === "pack"` in this lesson: one tag per option, `null` at the key index, else one of `neighbor_number` (a nearby/same-family number from the handbook), `round_number`, `sibling_rule_number` (a number from a different rule), `alt_source` (what federal rules/other sources say instead), `trap_named` (the wrong idea an Exam trap in the lesson names), `plausible_generic`, `true_statement` (true but not the answer to this stem).
- `numberItems`: 1 question per important row of the lesson's Numbers table that holds a number (aim for 60–80% of rows with numbers; skip trivial ones). Three options; the key's numbers must ALL appear in that row's value (the parser drops the item otherwise); distractors are real handbook numbers from the same family or named traps — never invented. Plain stem a beginner can parse. Explanation says why, with page cite.

## 3. `FACTORY/cdl_california/enrich/notes/<ID>.json` — "why this option is wrong"
`{ "<itemId>": [null, "note", "note"], ... }` for EVERY pack item of the lesson: `null` at the key; each other option gets ≤ 25 words saying why it is wrong per the handbook (what it confuses, what the right rule is). Never refer to options by letter or position ("option a", "(b)", "the first"). Facts only from the lesson.

## Check (required)
`cd FACTORY/cdl_california/app && LESSON_ONLY=<ID> npm run -s parse` must end with `LESSON OK`, print `summaries: N concepts` (N = all concepts of the lesson) and show NO `ENRICHMENT DROPPED` lines for your lesson. Then re-read every number question and note against the lesson once more (self-verify: exactly one right option, key correct, no invented numbers).
Do not edit lesson files or any other file; do not run the full parse, build or git.
Report: files written, counts (summaries concepts, tagged items, number items, notes), anything doubtful.
