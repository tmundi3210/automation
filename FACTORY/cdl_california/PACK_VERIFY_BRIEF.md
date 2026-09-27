# Independent fact check of pack lessons (default: refute)

You are an independent checker. Another writer produced lesson files in `FACTORY/cdl_california/source/packs/` from the CA Commercial Driver Handbook DL 650 (REV. 12/2019). Your job is to find every statement that is **wrong, unsupported, or misleading** against the handbook. Assume each claim is wrong until you find it on the cited page.

Handbook text per page: `/tmp/claude-0/-home-user-automation/4f22bdf7-6382-586c-a88b-74190a187585/scratchpad/pdf/pages/<label>.txt`. Garbled pages: render from `.../scratchpad/pdf/CDL .pdf` with PyMuPDF in `.../scratchpad/venv` and view the PNG.

Check, for each assigned lesson:
1. Every number, unit, colour, step order, and rule in Learn it, Numbers table, Exam traps, TYK answers, Flashcards, Practice test and Recap matches the handbook page it cites (or a nearby page in the same section). Flag wrong page cites too.
2. Every practice question: answer it yourself from the handbook, without reading the key first. Then compare. Flag: wrong key; more than one defensible answer; no right answer; distractor that is actually also true; invented numbers; stems a beginner cannot parse.
3. Anything taught as a test fact that is not in the handbook (federal rules, memory, websites).
4. Verbatim copying of handbook sentences (more than ~12 consecutive words identical) — flag with location.
5. Missing test-worthy facts: numbers or rules on the covered pages that the lesson omits entirely (list the most important ≤10).

Do NOT edit any file. Output ONLY a JSON array to the file `/tmp/claude-0/-home-user-automation/4f22bdf7-6382-586c-a88b-74190a187585/scratchpad/pv/<LESSON>.json` for each lesson (create the dir), and in your final report a summary line per lesson. Each issue:
`{"lesson":"AB-01","where":"Practice Q7" | "Numbers row 'Cut-out'" | "Learn it 5.1.2 bullet 3" | "Trap 4" | "Flashcard 12" | "TYK 5.1 #3" | "Recap 2","severity":"wrong"|"ambiguous"|"unsupported"|"copied"|"missing"|"cite","quote":"exact text from the lesson","handbook":"what page X-Y actually says (paraphrase)","fix":"exact replacement text"}`
Be precise and complete; do not pad with style opinions. If a lesson is clean, write `[]`.
