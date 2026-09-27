# CDL Workshop CA

An offline study app for every California CDL test: the written tests **General Knowledge** (50 questions, pass 40), **Combination Vehicles** (20/16), **Air Brakes** (25/20), **Doubles/Triples** (20/16), **Tank** (20/16), **Passenger** (20/16), **School Bus** (20/16) and **HazMat** (30/24), plus preparation for the three **skills tests**. It is built from 34 lessons: the user's 18 GK/CV lessons in `../source/lessons/` (kept verbatim) and 16 added lessons in `../source/packs/` (written from the handbook, each independently fact-checked; briefs in `../PACK_*.md`). All lessons follow which follow the California Commercial Driver Handbook (DL 650, 2019 edition) with page numbers for every fact.

Not affiliated with the California DMV, CHP or FMCSA. When anything here differs from the current handbook, the handbook wins.

## Use it

| Where | How |
|---|---|
| Mac | `bash ../mac/setup-mac.sh`: installs, builds, and puts **Start CDL Workshop.command** and a one-file copy on the Desktop. Run it again to update. |
| Any computer | `npm ci && npm run build:pwa && npm run preview`, then open the printed localhost URL. Chrome/Edge: install icon in the address bar; Safari (macOS 14+): File → Add to Dock. Works offline once loaded. |
| One file | `npm run build:single` → `dist-single/index.html`. Opens in any browser, no server. |
| claude.ai | Private Artifact: https://claude.ai/artifact/Nty4XqrFLkp3eTjd69NBez (share it from the page's Share menu). Progress is kept in each viewer's private per-artifact store. |

## What it does

- **Lessons**: short "key points" view per concept, "Dive deeper" for the full text, tap-to-define glossary terms, [CA] badges, handbook page plates, YouTube search links (videos are extra; the handbook wins).
- **23 interactives**, one or more per lesson: class finder (Fig 1.1), penalty timeline, duty clock, 7-step walkaround, retarders, mirrors, warning triangles, stopping distance, following distance, turns, hazard clues, night/fog/winter/heat, railroad crossing, downgrade braking, emergencies, crash/fire/alcohol scene, cargo tie-downs, rig anatomy, crack-the-whip and rollover, trailer skid, air system, coupling (16 + 10 steps), combination brake checks. Explore mode never changes progress; challenge answers count as light practice evidence; a clean challenge earns a topic stamp.
- **Testing**: lesson practice tests (the pack's 338 questions), trap duels (true/false), 327 handbook-checked number questions (156 held out for mocks only), DMV-style mocks (3 choices, immediate right/wrong, skip returns at the end, no timer).
- **Mistake intelligence**: every wrong option carries a "why it's wrong" note; misses are diagnosed (trap wording, known trap, wrong source, number mix-up, faded memory, not learned yet) and grouped by topic with a fix drill; a topic clears after correct answers on two different days.
- **Scheduling**: FSRS-6 spaced review per fact (intervals capped at the test date), exam-date planner with feasibility check, calendar that fills in 5 % steps, `.ics` export (installable app only).
- **Readiness**: Monte Carlo estimate shown as a range, predicted-vs-actual after each mock, "strong ready" after two mocks at 90 %+.
- **Continuity**: autosave (IndexedDB + synchronous backup), resume unfinished tests after reload, checksummed resume code and progress file to move between devices.

## Develop

```
npm ci
npm run parse          # lessons + enrich/ → src/content/content.json (fails on any golden-count drift)
npm run dev            # local dev server
npm test               # unit tests (engine, content, number-drift guard, test choice): 31
npx playwright test    # E2E: flows, a11y (axe light + dark), widgets (isolation, legibility), both viewports: 151 + 33 phone-only skips
node scripts/pwa-offline.mjs   # after build:pwa — service worker + offline reload
```

Layout: `scripts/` (parser, markdown renderer, screenshot/gate tools, artifact converter), `src/engine/` (FSRS/BKT/diagnosis/planner/readiness), `src/store/` (storage adapter, resume code), `src/ui/` (screens), `src/widgets/w/` (one file per interactive, auto-registered), `tests/`.
Content enrichment lives in `../enrich/` (number questions, distractor tags, per-option notes, verifier rejections); research and scoring anchors in `../research/`; the gate ledger in `../SCORES.md`.

## Known limits

- Skills-test lessons teach what the examiner checks; they cannot replace behind-the-wheel training (ELDT).
- Firefighter (F) and other California certificates are not covered.
- YouTube videos could not be checked from the build environment, so each topic links to a YouTube search rather than a fixed video.
- Tested in Chromium only; iPhone/iPad Safari is untested.
- Question counts and pass marks are the DMV test format; the handbook itself does not print them.
