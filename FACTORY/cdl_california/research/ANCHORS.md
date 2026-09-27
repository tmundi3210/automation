# Scoring anchors (1–10) — used by independent evaluators. Pass = every dim ≥8 AND weighted ≥8.0.
Evaluators see only screenshots, DOM text, test output, this file. Any score ≥8 must cite ≥3 concrete observations and list ≥2 remaining defects. Unbuilt feature = N/A (not 0).

| Dim (weight) | 3 = poor | 5 = table stakes (typical CDL app) | 8 = best in market | 10 = exemplary, evidenced |
|---|---|---|---|---|
| D1 Handbook fidelity (15) — OBJECTIVE | keys contradict DL 650 or federal numbers shown as CA answers | mostly right, no page cites, mixed sources unflagged | every item paged to DL 650, [CA] flags, handbook-vs-elsewhere notes, validator ALL GREEN, audit discrepancies resolved | + independent audit of 100% triaged claims, errata surfaced, internal handbook conflicts handled by item rules |
| D2 Understanding-first teaching (15) | question bank only | text lessons + answer explanations | concept-first core view w/ why + worked examples, per-distractor notes, transfer items (scenario/diagram) | + beginner can explain the rule from app content alone (knowledge-blind persona) |
| D3 Interactive visuals (10) | none | static images | purpose-built sims for mechanical topics (coupling, air system, stopping/following distance) w/ consequences | + every lesson has a topic-specific interactive, a11y fallbacks, no errors at 390 px |
| D4 Adaptive review & diagnosis (10) — partly OBJECTIVE | none | re-take missed questions | spaced repetition per fact + cause-tagged mistake notebook + remediation actions | + sim-verified honesty (lucky guesser, crammer) and resolution rules |
| D5 Honest readiness (8) — OBJECTIVE | "99% pass" claims | raw % on seen items | unseen-item mocks mirroring real format, readiness as calibrated range | + Brier calibration shown, no inflated claims |
| D6 Class path + end-to-end guidance (10) | one generic list | class picker only | class decision tree → personal test list → schedule → CLP/ELDT/skills roadmap w/ official links | + next-best-step coach always correct for state |
| D7 Plain-language accessibility (10) — partly OBJECTIVE | jargon wall, axe criticals | readable, some a11y issues | plain English, glossary tooltips, WCAG AA (axe 0 serious), keyboard, reduced motion | + knowledge-blind persona comprehension ≥90% |
| D8 Topic-specific, non-manipulative gamification (7) | XP/hearts/leagues/streak anxiety | generic streak + badges | mechanics tied to each topic's skill, rewards verified mastery, rest days safe | + zero learner-model writes by games (tested) |
| D9 Continuity (7) — OBJECTIVE | progress lost on reload | local save only | resume exact spot, export/import, offline, planner calendar | + works in sandbox/iframe fallback and PWA offline |
| D10 Visual/UX polish (8) | broken layout/overflow | functional, generic | coherent design system, light/dark, mobile-first, no overflow, clear hierarchy | + delightful yet calm; consistent across all screens |

Hard fails (cap 5): a key contradicting the handbook; an unverified URL/ID presented as verified; a game writing the learner model; FSRS nondeterminism under pinned clock; progress loss on reload.
Canaries (seeded-defect build scored alongside): 4/32↔2/32 key swap · horizontal overflow at 390 px · 3:1 contrast text · a widget stating "fifth wheel level". Missed canary ⇒ that evaluator's scores void.
