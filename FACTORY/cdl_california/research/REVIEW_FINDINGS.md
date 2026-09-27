# Plan review findings (independent architect review, 2026-09-27) — applied to BUILD_PLAN.md

Verified: START-HERE totals exact — 338 practice (GK 270 / CV 68), 657 flashcards (525/132), 98 TYK (76/22), 486 Numbers rows, 264 traps, 215 Learn-it H3, 1,014 options, answer letters a/b/c = 113/117/108, 44 negative-stem items. Every key has a page cite. Option shuffling safe (no "both a and b" options).
Page cites align with DL 650 (© DMV 2019-2021, edition id "DL 650 R12-2019" in PDF metadata): label s-n → 0-based PDF index n+offset {1:5,2:35,3:87,4:93,5:99,6:113,7:131,8:137,9:141,10:167,11:181,12:193,13:197}; 0 footer mismatches (PyMuPDF, P0).

| # | Sev | Finding | Applied fix |
|---|---|---|---|
| 1 | BLOCKER | Artifact runtime: browser storage is per-viewer convenience only | 3-tier StorageAdapter; Artifact adds per-viewer private store capability; resume code |
| 2 | BLOCKER | Artifact downloads capability disallows .ics; frame cannot download directly | .json via downloads capability; .ics PWA-only |
| 3 | BLOCKER | BKT Δ≤0.07 unreachable with transition | Acceptance on evidence update at P(L)=0.2; transition once/concept/day |
| 4 | HIGH | "All concepts P(L)≥0.6" unlock gate unreachable (~1.6 items/concept) | Unlock on lesson practice ≥90% or placement |
| 5 | HIGH | FSRS unit unspecified; per-surface scheduling = bank memorization | KnowledgeUnit layer, surface rotation, max interval ≤ days-to-exam |
| 6 | HIGH | ~1,350 LLM items + 45 widgets before any app | Walking skeleton → Artifact v0 first; deterministic derived items |
| 7 | HIGH | Research not persisted | research/*.md |
| 8 | HIGH | Held-out mock pool unspecified | Seeded split, GK≥150 / CV≥60 held-out |
| 9 | HIGH | Readiness overconfident (independence, R≠transfer) | Monte Carlo + weighting schemes + latent offset + Platt calibration; band on lower bound |
| 10 | HIGH | Widgets can't count as evidence | evidence_class taxonomy, onEvidence |
| 11 | HIGH | No truck-anatomy primer for beginners | Rig Anatomy flagship widget #1 |
| 12 | HIGH | "Beyond the handbook — not on the test" leakage | Validator FAIL if generated items derive from it |
| 13 | HIGH | .gitignore lacks node_modules | Added before install |
| 14 | MED | Positional IDs / huge resume code | Hash IDs + migration; snapshot resume code |
| 15 | MED | SW can't run on file:// | Offline test on localhost PWA only |
| 16 | MED | Only Chromium available | iOS manual checklist; durable store in Artifact |
| 17–23 | MED/LOW | RT normalization, calendar edge cases, Elo deferral, mock early-stop unverified, fuzz vs determinism, Vite 8 risk, legacy as alt_source distractors | All applied in BUILD_PLAN.md |

"Hard Ready" (2 mocks ≥90%) is strict: P(both GK mocks ≥45/50 | p=.90)=0.38 → renamed "Strong ready" badge; band uses calibrated lower bound.
