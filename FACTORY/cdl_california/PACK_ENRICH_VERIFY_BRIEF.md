# Independent check of enrichment for the new pack lessons (default: refute)

Another writer produced, per lesson, `FACTORY/cdl_california/enrich/summaries/<ID>.json` (key points per concept), `enrich/<ID>.json` (`numberItems`: extra number questions; `distractorTags`), and `enrich/notes/<ID>.json` ("why this option is wrong" notes per practice item). Source of truth: the lesson `source/packs/<ID>-*.md` and the handbook pages it cites (`/tmp/claude-0/-home-user-automation/4f22bdf7-6382-586c-a88b-74190a187585/scratchpad/pdf/pages/<label>.txt`). Item/option text: `app/src/content/content.json` → `items[id]` (options, key).

Assume each claim is wrong until you confirm it. Check:
1. Every `numberItems` entry: answer it yourself from the handbook first. Reject if: key wrong, a second option is also defensible, the stem is unclear, or a distractor is an invented number.
2. Every note: true per the handbook, and actually about that option (compare with `items[id].options[k]`). Flag false or misleading notes.
3. Every summary bullet: numbers and rules match the lesson/handbook exactly (units, "more than"/"or more").

Write fixes directly (you MAY edit these enrichment files, nothing else):
- A bad number item → delete it from `numberItems` (or correct it if the fix is obvious and certain).
- A bad note → rewrite it (≤25 words, never name options by letter/position).
- A bad bullet → rewrite it.
Then run `cd FACTORY/cdl_california/app && LESSON_ONLY=<ID> npm run -s parse` for each lesson: must end LESSON OK with no ENRICHMENT DROPPED lines.
Report per lesson: number items checked/deleted/fixed, notes fixed, bullets fixed — with one line per change saying what was wrong.
Do not run git, the full parse, or builds.
