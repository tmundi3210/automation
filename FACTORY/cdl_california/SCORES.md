# Score ledger — every gate, every round (1–10; pass = each dim ≥8 & weighted ≥8.0; objective gates pass/fail)

| Date | Gate | Round | Method | Result | Notes |
|---|---|---|---|---|---|
| 2026-09-27 | G0 page map | 1 | PyMuPDF footer-label map, 208 pp, 190 labelled content pages | PASS | 0 footer mismatches; spot facts on cited pages (419→2-16, tilted→6-9) |
| 2026-09-27 | G1 parse fidelity | 1 | parser golden fixture (per-lesson + totals) | PASS | 338/1014/657/98/486/264 exact, 0 unparsed lines; heuristic number-MCQs rejected (semantic distractor errors) → deferred to verified enrichment |
| 2026-09-27 | G3a engine unit | 1 | vitest 21 tests | PASS | FSRS-6 determinism + exam cap, BKT Δ≤0.07 & lucky-guesser<0.4, diagnosis T/N, notebook resolution, calendar 5% fill, planner feasibility, readiness vs Bin(50,.9)=0.9906, resume code tamper |
| 2026-09-27 | G2 skeleton E2E | 1 | Playwright 1.56.1, file:// single-file build, 390×844 + 1280×800 | 29/32 → FIX → 32/32 PASS | found real defect: progress lost on reload within save debounce → sync localStorage backup + newest-wins load; axe wcag2a/aa 0 serious/critical on 9 screens; no horizontal overflow |
