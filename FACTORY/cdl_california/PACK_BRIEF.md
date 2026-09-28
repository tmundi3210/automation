# Pack lesson authoring brief (Air Brakes, endorsements, skills)

You are writing new lessons for **CDL Workshop CA**, an offline app that teaches the California CDL tests from the **CA Commercial Driver Handbook DL 650 (REV. 12/2019)**. The existing 18 lessons (General Knowledge GK-01..14, Combination CV-01..04) in `FACTORY/cdl_california/source/lessons/` are the style and format reference. Your new lessons go in `FACTORY/cdl_california/source/packs/`.

## Sources
- Handbook text per page: `/tmp/claude-0/-home-user-automation/4f22bdf7-6382-586c-a88b-74190a187585/scratchpad/pdf/pages/<label>.txt` (e.g. `5-3.txt`). Some pages are garbled by OCR (figures, tables). If a page's text is unclear, render it: the PDF is `.../scratchpad/pdf/CDL .pdf`, and PyMuPDF is in `.../scratchpad/venv` (`venv/bin/python -c "import fitz; ..."`). Find the PDF page index by searching page text for the label footer; render with `page.get_pixmap(dpi=110).save(...)` and look at the PNG with the Read tool.
- The handbook is the **only** source of test facts. Never use memory, federal rules, or websites for a fact. If you know that other sources differ, you may add a `> Handbook vs other sources: …` callout, but the lesson teaches the handbook answer.
- **Copyright:** paraphrase in your own plain words. Never copy a handbook sentence verbatim (short technical phrases and exact numbers/terms are fine). Never reproduce figures; describe them.

## Format (the parser is strict — copy the grammar of an existing lesson)
Read `source/lessons/CV-02-combination-air-brakes-and-abs.md` and `source/lessons/GK-11-railroad-and-mountain.md` in full first, and `research/PARSER_SPEC.md` section 1. Then write each lesson with exactly this skeleton:

```
# AB-01 · Parts of an air brake system

> **Handbook:** Section 5.1, pages 5-1–5-5 · **Test:** Air Brakes · **Exam weight:** High · **Study time:** ~40 min

## What you'll be able to answer
- 5–6 bullets

## Learn it
Intro paragraph(s) (p. X-Y) — this becomes the lesson's "Start here" concept.

**Key words**
- **Term** = plain-English definition.      ← these feed the app glossary; define every term a beginner won't know

### 5.1.1 Air compressor (p. 5-1)
- bullets / short paragraphs; *Why:* lines; *Worked example:* where numbers are involved
...

## Numbers & terms to memorize
| Item | Value / meaning | Page |
|---|---|---|
| ... | **bold the value** | 5-1 |

## Exam traps
- **Trap:** wrong idea → **Correct:** right answer, with the reason. (p. 5-3)

## Handbook review questions — answered
**TYK 5.1 #1** question text (paraphrased)
→ answer. (p. 5-3)
   (answer every "Test Your Knowledge" question the handbook prints for the sections your lesson covers; number them as the handbook does, label `TYK <section> #<n>`. If the sections have no box, write exactly: `_This handbook section has no review box — see Flashcards and Practice test._`)

## Flashcards
| # | Question | Answer |
|---|---|---|
| 1 | ... | ... |

## Practice test (real-exam style)
1. Stem?
   a) option
   b) option
   c) option
...
10. Stem (from Q10 on, options are indented 4 spaces)
    a) option
### Answer key
1. a — explanation of why, and why the tempting wrong option is wrong. (p. 5-3)

## One-minute recap
- 6–8 bullets
```

Rules the parser enforces or the app relies on:
- H1 id prefix and meta `**Test:**` name: AB = `Air Brakes`, DT = `Doubles and Triples`, TK = `Tank Vehicles`, PV = `Passenger Transport`, SB = `School Bus`, HM = `Hazardous Materials`, SK = `Skills Tests`. File name `source/packs/<ID>-<kebab-title>.md`.
- Every H3 in Learn it: `### <section number> <Title> (p. X-Y)`; a `[CA]` tag marks California-only rules (see GK-03).
- Exam traps: every line exactly `- **Trap:** … → **Correct:** … (p. X-Y)`.
- Practice: exactly 3 options a/b/c; exactly one correct; answer key line `N. x — explanation (p. X-Y)`; every key line ends with a page cite. Spread keys roughly evenly over a/b/c. Include some "Which is NOT…"/"all EXCEPT…" stems (the DMV flips wording). Distractors must be plausible and come from real handbook values/rules or named traps — **never invent numbers** that appear nowhere in the handbook's topic. Never make "all of the above" style options.
- Numbers table: exactly 3 cells per row; include every number the sections contain (psi, seconds, feet, percentages, years, counts).
- Minimum sizes per lesson: **≥ 16 practice questions (target 18–25), ≥ 25 flashcards, ≥ 8 traps, ≥ 10 numbers rows**, 5–10 H3 concepts. Longer sections deserve more.
- Plain English for a complete beginner (never seen a truck): short sentences, define each term on first use, give the reason behind each rule (*Why:*). Grade-7 reading level. Use `**bold**` for the fact that gets tested.
- Page cites everywhere: `(p. 5-3)`, `(pp. 5-3–5-4)`.

## Check before you report (required)
1. `cd FACTORY/cdl_california/app && LESSON_ONLY=<ID> npm run -s parse` must end with `LESSON OK (nothing written)` for each of your lessons. It prints your counts and concept list. Fix every PARSE PROBLEM.
2. Fact check: go through your file line by line against the page text. Every number, color, step order, and rule must match the handbook page you cite. Fix or delete anything you cannot find on the page.
3. Answer key check: re-answer every practice question from the handbook alone, without looking at your key; they must agree, and exactly one option must be right.
4. Do NOT edit any file outside `source/packs/<your lesson files>`. Do not run `npm run parse` without LESSON_ONLY (other agents are writing in parallel), do not build, do not commit.

## Final report (dense plain text)
Files; per lesson counts (practice, flashcards, TYK, numbers, traps, concepts with ids); handbook pages covered; any page that was garbled and how you resolved it; any internal handbook conflicts or ambiguities you found (quote page + paraphrase); anything you were unsure of.
