# Media registry candidates (unverified) + tech recon

_Research sweep 2026-09-27 (read-only agents; web access was search-index only — dmv.ca.gov, youtube.com, app stores blocked from the build container). Tags: [FACT src] / [ESTIMATE basis] / [UNKNOWN]. Source agent: `media-tech`. Kept verbatim for provenance; corrections from the plan review are in `../BUILD_PLAN.md` and `REVIEW_FINDINGS.md`._

# Task A (media) and Task B (tech recon) for the California CDL workshop

## Verification limits (read first)
- **Live checks were not possible.** WebFetch returned `EGRESS_BLOCKED` for every host that could confirm a video or image: www.youtube.com, youtube.com, i.ytimg.com, noembed.com, commons.wikimedia.org and www.dmv.ca.gov. The proxy status page also lists CONNECT 403s for www.dmv.ca.gov and www.youtube.com. [FACT: tool errors]
- **So no oEmbed check was done.** Every video and image below was found through the WebSearch index only: the title and URL appear together in the results. [FACT-search]
- **Current availability, embeddability and duration are [UNKNOWN] for every item.** No result gave a duration.
- **Build rule:** keep a media registry with `verify_method` and `verified_on` for each item. Run an oEmbed check (`https://www.youtube.com/oembed?url=…&format=json`) from a network that allows youtube.com before publishing.

## Task A: Videos (search-index verified, not live-checked)

| Topic | Title (as indexed) | Channel / uploader | ID | Notes |
|---|---|---|---|---|
| CA pre-trip, in-cab and air brake | "CDL Class A Pre-Trip In-Cab Checklist & Air Brake Check Inspection \| California DMV 2026" | [UNKNOWN]. Not the DMV channel: the index says published 2024-01-06 | hiobnwMv2cw | CA-specific; best candidate |
| CA in-cab | "California DMV CLASS A CDL In-Cab Inspection" | [UNKNOWN] | K2E-jT6FtbA | Published 2012-09-08 [FACT-search]. May be out of date |
| CA outside inspection | "California DMV CLASS A CDL OUT-SIDE inspection" | [UNKNOWN] | 0VZtYJGcbH8 | Published 2012-09-11 [FACT-search]. May be out of date |
| CA in-cab and air brakes | "IN CAB AND AIR BRAKES CDL TEST (California) See description for 2020 changes" | Zone One Trucking School [FACT-search] | Nor8v8OY8qM | Published 2019-12-09. The title itself flags procedure changes |
| CA air brake and in-cab | "Air Brake Test and In Cab inspection for Class A License in a Manual Truck School" | Abylex Truck School, Sacramento [FACT-search] | PkUC8myvr6s | Published 2021-06-05 |
| CA 2023 | "California 2023 cdl airbrakes and in -cab inspection" | [UNKNOWN] | NSKosm1ExNM | — |
| Air brake leak-down | "CDL AIR BRAKE LEAK TEST: See new Wording in the Description." | [UNKNOWN] | Mx-yFdIA838 | Wording changed per the title; check it against DL 650 |
| Pre-trip, modernized test | "CDL Class A Pre-Trip Inspection \| New Modernized CDL Test \| New 2026 CDL Pre-Trip Training Guide" | [UNKNOWN] | Kiqpd6nv6QE | Published 2024-09-07. Federal/general, not CA |
| Pre-trip | "HOW TO PASS A Class A Pre trip inspection in 26 min. Done by State CDL Examiner" | [UNKNOWN] | zH9f-AquD7w | State not stated |
| Coupling and uncoupling | "How to couple and uncouple a tractor-trailer" | Schneider driver instructor [FACT-search] | b50jQ4kRuXg | Published 2023-01 |
| Coupling and uncoupling | "How to Couple and Uncouple a Tractor Trailer \| Step By Step Guide" | [UNKNOWN] | wb-EAiOV-XI | Published 2023-09 |
| Coupling and uncoupling | "Coupling and Uncoupling Tutorial" | J-Tech lead instructor [FACT-search] | VPJ1biinnx8 | Published 2015-04 |
| Coupling and uncoupling | "How To Couple & Uncouple A Semi Tractor-Trailer" | [UNKNOWN] | ucrtgb5ywYU | Published 2023-11 |
| Backing (straight, offset, alley dock) | "CDL Basic Backing Skills; Alley Dock, Offset, Straight Line Backing" | [UNKNOWN] | 5C1HYKEBP1c | Published 2018-07-17 |
| Backing | "Class A CDL Skills Test Straight Line Backing, Offset, Alley Dock" | Theasianmaishow [FACT-search] | Tj8zdT1FaFE | Published 2019-07-19 |
| Backing (playlist) | "CDL College: CDL Skills Test–Backing Maneuvers videos" | CDL College | playlist PLZ8QLN78x74bTlvn4xVOMCqYDc6j-VPA_ | Playlist, not a single video |
| Off-tracking and right turns | "How to Make a Right Turn in a Tractor Trailer the Right Way - CDL Driving Academy" | CDL Driving Academy [FACT-search] | 5lBXVleV5CU | Published 2020-01 |
| Off-tracking and turns | "How to Make a Right Turn + Left Turn in a Tractor Trailer Without Hitting Anything" | [UNKNOWN] | F9H3kxhs2U0 | Published 2021-11 |
| Skid and jackknife | "How to Handle a JackKnife Skid- A Critical Professional Truck Driver Skill" | [UNKNOWN] | Iah_ZCEjc_o | Published 2017-12 |
| Downgrades and braking | "Trucking & Safe Downhill Braking Video Explained HD" | [UNKNOWN] | AbtUdgbg0Nk | Check against the DL 650 snub-braking rule |
| Blind spots / No-Zone (space) | "Our Roads, Our Safety - Blind Spots" | FMCSA campaign [FACT-search] | eY0hKZq0zMc | Published 2022-09-27. Official federal |
| ABS | "What a Truck Driver Should Know About Antilock Brake Systems" | US DOT, per snippet [FACT-search] | hSz5gZSvlZ8 | Official-origin; age [UNKNOWN] |
| Hazmat placards | "DOT Chart 16- Understanding HazMat Placards and Labels" | Supplement to the 2020 ERG series [FACT-search]; channel [UNKNOWN] | wrSX5L9EWZ0 | Aimed at responders, not drivers |
| Doubles and converter dolly | "Hooking Doubles With Converter Dolly Attached While Backing" | [UNKNOWN] | 0YOzLTYuuL0 | — |
| Doubles (manual read-along) | "CDL Manual, Section 7, Doubles & Triples (Manual Read Along)" | [UNKNOWN] | d3yk5bMiZiw | Federal model manual, not DL 650 |
| Converter dolly selection | "TRUCKING \| Choosing Converter Dollies \| Pulling Doubles" | [UNKNOWN] | HBRaSTglqjE | Supplementary |
| School bus pre-trip | "CDL Pre-Trip Inspection Demonstration on a School Bus" | [UNKNOWN] | 4ZDh37OE4J8 | Danger zones and mirrors not confirmed in the video |
| School bus pre-trip | "School Bus class B CDL pretrip inspection demonstration" | [UNKNOWN] | 1qeLLB-uNPI | Published 2022-03 |
| Passenger bus pre-trip | "CDL Class B Pre-Trip Inspection Demonstration \| Passenger Endorsement" | [UNKNOWN] | FAsM-dc1g5s | Published 2024-04 |
| Passenger bus pre-trip | "CDL Class B Passenger Bus Pre-Trip Inspection - Pass Your CDL Road Test 2022" | [UNKNOWN] | o2QOfxf89so | Published 2022-05 |

**Official channels**
- FMCSA YouTube channel: `UCCuJr4LRogGPmnFjbCm5fSA` [FACT-search]. The individual CDL-topic videos on it are [UNKNOWN].
- CA DMV has an "Educational and Instructional Videos" page at `dmv.ca.gov/portal/driver-education-and-safety/educational-materials/videos-2/` [FACT-search]. Whether any of those are CDL videos is [UNKNOWN] because the site was blocked. No official CA DMV CDL YouTube video was found.

**No suitable video found:** glad hands or air lines alone, fifth wheel alone (both are covered inside the coupling videos), railroad crossings (FMCSA's clip is on Facebook, not YouTube), tank surge and baffles, warning triangles, and a dedicated school bus danger-zones and mirrors video. [UNKNOWN] For all of these, build in-app simulations or diagrams.

**Media risks found**
- **Third-party content can contradict the handbook.** Example: a search snippet said a "typical tractor-trailer" needs 4 seconds of following distance. The handbook rule is 1 second per 10 ft of length below 40 mph, plus 1 second above 40 mph, which gives more for a 60 ft rig. Third-party leak-rate figures also vary. [FACT-search snippets]
  - Rule: videos are optional "watch" extras shown with a "Handbook wins" banner.
  - Videos are never a source for quiz answer keys.
  - Show a per-video "differs from DL 650 on: …" note where known.
- **Several CA videos are from 2012, and several titles themselves say procedures changed** ("2020 changes", "new Wording"). Tag them "may be outdated".
- **Embeds need a network connection** and the owner's embed permission [UNKNOWN per video].
  - Use a click-to-load `youtube-nocookie.com` iframe with a plain-link fallback.
  - The app must be complete without videos, both offline and as the single-file build.

**Official reference documents (links only)**
- DL 650 PDF: `https://www.dmv.ca.gov/web/eng_pdf/comlhdbk.pdf` [FACT-search]
- DL 650 online: `https://www.dmv.ca.gov/portal/handbook/commercial-driver-handbook/` [FACT: cited in the zip]
- A Spanish handbook path exists under `qr.dmv.ca.gov/portal/es/...dmv_handbook` [FACT-search]
- PHMSA DOT Chart 16 (placards), English and Spanish PDFs: `https://www.phmsa.dot.gov/training/hazmat/dot-chart-16-hazardous-materials-markings-labeling-and-placarding-guide` [FACT-search]
- FMCSA railroad-crossing text of the CDL Manual, section 2.15 [FACT-search]

### Images (openly licensed)

| Need | Candidate | License | Status |
|---|---|---|---|
| Hazmat placards | `Category:SVG_diagrams_of_US_DOT_hazmat_symbols`, `File:DOT_hazmat_class_3.svg` (plus alt 1–3), `File:DOT_hazmat_class_2.2.svg`, `File:DOT_hazmat_-_Inhalation_Hazard_-_Label.svg` | `File:HAZMAT_Class_3_Flammable_Liquids.png` is marked "PD US DOT" [FACT-search]. The SVGs are probably PD too [ESTIMATE] | Check each file |
| Converter dolly | `File:Dolly_trailer.jpg` | CC BY-SA 2.5 [FACT-search] | European mid-axle dolly, not a typical US A-dolly [FACT-search description]. Needs an attribution and share-alike note |
| Converter dolly | `File:Single_axel_dolly.webp` (3916×2267) | [UNKNOWN] | — |
| Fifth wheel | `Category:Fifth-wheel_couplings` (94 files) [FACT-search] | Per file | **Do NOT use `File:Fifth-wheel.jpg`**: the index describes it as an RV fifth-wheel trailer "with a single slide-out", not a coupling [FACT-search] |
| Tractor and semitrailer | `File:MAZ-5440M9_fifth-wheel_truck_with_MAZ-930011_semitrailer.jpg` | CC BY-SA 3.0 [FACT-search] | Russian truck, not a close-up |
| Kingpin | None found. The search returned cosplay files named "Kingping.jpg" | — | [UNKNOWN] |
| Glad hands | None found; `Category:Hose_connectors` (48 files) is generic | — | [UNKNOWN] |
| Brake chamber and slack adjuster | None found | — | [UNKNOWN]. **Do not use** `File:AirBrake.gif` or `File:Westinghouse_Air_Brake_piping_diagram.jpg`: both are railway air brakes and would mislead |
| ABS | `Category:Anti-lock_braking_systems` exists | Per file | [UNKNOWN] |

**Recommendation:** make original inline SVGs the main visuals. [ESTIMATE-design]
- **Why:** no license risk, they work offline and inside the single file, and they cost a few KB each. They can be animated (air flow, spring-brake pop at 20–45 psi, jaws locking) and have clickable hotspots. Text sits on separate layers so it can be translated.
- **Placards:** draw them from the 49 CFR 172 Subpart F specs. US federal works are not copyrighted [FACT 17 U.S.C. §105].
- **Commons photos:** optional "real-world photo" layer only, with attribution metadata. Whether the build environment can reach upload.wikimedia.org is [UNKNOWN] (commons.wikimedia.org is blocked).

## Task B: Tech recon

### Tools [FACT: commands run]
- **Runtimes:** node v22.22.2, npm 10.9.7, pnpm 10.33.0, yarn 1.22.22, bun 1.3.11, python3 3.11.15.
- **Global npm packages:** playwright 1.56.1, typescript 6.0.2, eslint 10.1.0, prettier 3.8.1, http-server 14.1.1, serve 14.2.6, ts-node 10.9.2.
- **npm registry:** registry.npmjs.org works. It is in the proxy's NO_PROXY list, and `npm view` succeeded.
- **Playwright browsers:** `PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers`, containing `chromium-1194`, `chromium_headless_shell-1194` and `ffmpeg-1011`.
  - The global `playwright-core/browsers.json` maps chromium revision 1194 to Chromium 141.0.7390.37, and the binary reports that version.
  - **Pin `@playwright/test@1.56.1` exactly** (it exists on the registry).
  - The latest, 1.63.0, would expect a newer browser revision that is not installed. Downloading it from the Playwright CDN is probably blocked by the egress policy. [ESTIMATE]

### Latest versions on npm [FACT: `npm view`]

| Package | Version | Peer / engines |
|---|---|---|
| vite | 8.3.1 (7.3.6 is the latest 7.x) | node ^20.19 or >=22.12, OK |
| preact | 10.29.8 | — |
| @preact/preset-vite | 2.10.6 | vite 2.x–8.x, @babel/core 7.x |
| @preact/signals | 2.11.2 | — |
| react / react-i18next | 19.3.0 / 17.0.15 | — |
| typescript | 7.0.2 latest (6.0.2 installed globally) | — |
| i18next | 26.4.2 | TS ^5, ^6 or ^7 |
| ts-fsrs | 5.4.2 | node >=20 |
| vite-plugin-singlefile | 2.3.3 | vite ^5.4.21–^8, rollup ^4.59 (rollup latest 4.63.5) |
| dexie | 4.4.6 | — |
| idb-keyval | 6.3.0 | — |
| vite-plugin-pwa | 1.3.0 | vite 3–8, workbox-build and workbox-window ^7.4.1 |
| workbox-build / workbox-window | 7.4.1 | — |
| fflate / lz-string | 0.8.3 / 1.5.0 | — |
| @playwright/test / playwright | 1.63.0 latest; **use 1.56.1** | — |

### Repo conventions [FACT]
- **Repo state:** `/home/user/automation`, branch `claude/inspiring-clarke-q3ovcj`, origin `github.com/tmundi3210/automation`.
- **No CI, no JS app yet:**
  - There is no `.github/`, no `CLAUDE.md`, and no `package.json` anywhere.
  - The only HTML file is `FACTORY/plated_jewelry/live_ops/orchestrator/poll/poll_template.html`.
  - This would be the repo's first Node app.
- **How a vertical is laid out:** `FACTORY/<vertical>/` holds `OWNER_BRIEF.md` (the owner's message saved verbatim, then an interpretation with tags), `BUILD_PLAN.md`, `research/`, `specialists/<code>/` and `app_plan/`.
  - Each specialist folder has 3 `*.spec.json` → `*.kb.json` knowledge bases plus a `<code>.specialist.json`.
  - movie_studio adds `productions/`. `FACTORY/webshop` holds only `ORCHESTRATION_BLUEPRINT.md`.
- **`FACTORY/study_system` (current working directory) already has 19 specialists we can reuse:**
  - `sched`: FSRS/SM-2 and timeline planning
  - `learner`: BKT/Elo, weakness diagnosis, challenge level
  - `qcraft`: item writing and answering strategy
  - `engage`: gamification, progress visuals including the day-box calendar spec, drill mechanics
  - `uxguide`: progressive disclosure, plain language
  - Also `viz`, `media`, `voice`, `contenteng`, `sysloops`, `planner`, `curric`, `graphux`, `kgraph`, `kgverify`, `extint`, `handoff`, `dental`, `toefl`
  - Its `app_plan/` targets a hosted Base44 app, which is a different product.
  - `app_plan/base44_plan/TESTING_PLAN.md` describes a fast-forward simulated learner with a virtual clock and planted ground truth. We can reuse that method to test SRS and weakness detection.
- **Where the new app should live (recommendation, [ESTIMATE]):** `FACTORY/study_system/cdl_ca/` containing `OWNER_BRIEF.md` (the user's request verbatim), `research/`, `content/` (compiled from the zip), `app/` (Vite project) and `tests/` (Playwright). This keeps it next to the specialists it uses. A separate `FACTORY/cdl_california/` would also fit the pattern.
- **Validators to copy:** `validators/kb_validator.py`, `validators/specialist_validator.py`, `validators/metrics.py`, `tools/gate_all.sh`, `FACTORY/build.sh`.
  - Their style: Python stdlib only, prints ALL GREEN or per-file FAIL with the re-run command, and stays honest that a green gate proves structure, not truth (FACTORY/README).
  - Copy this as `validate_content.py` with these checks:
    - Every question has 3 options (a/b/c, like the real test), exactly one key, and a DL 650 page reference `p. X-Y`.
    - IDs are unique.
    - Every number a question relies on matches one canonical `numbers.json` (e.g. 60 psi low-air warning, 20–45 psi spring brakes, the 1,001 lb placard threshold).
    - Every locale has every key, with an English-term fallback.
    - Every media registry entry has `verify_method`, `verified_on` and a `handbook_conflict` field.
    - Every class/endorsement tag (A/B/C, T/N/H/P/S/X) maps to a module.

### Content facts from the zip [FACT: unzip]
- **Size:** 18 files, about 381 KB of markdown.
- **Two generations of content:**
  - `part-01…part-10` (2026-09-22): the broad pack, including endorsements, passenger, school bus, hazmat, doubles and tank, plus mock tests.
  - `GK-01…05, GK-08, CV-01` and `00-START-HERE.md` (2026-09-24): the newer, deeper lessons.
- **Gap:** `00-START-HERE.md` promises 18 lessons (GK-01…14, CV-01…04), but the zip contains only 7. **GK-06, 07, 09–14 and CV-02…04 are missing**, so those topics must come from the `part-0x` files or be written fresh.
- **Useful facts inside the start file:**
  - The learner has already passed Air Brakes.
  - GK is 50 questions with 40 to pass; Combination Vehicles is 20 with 16 to pass; answers are 3 options.
  - Test languages listed at p. 1-10: English, Arabic, Chinese, Punjabi, Russian, Spanish. These are the natural target locales, and technical terms should also be shown in English.
  - First-try failure rates quoted: GK 39.5%, Combination 43.9%.

### Stack recommendation [ESTIMATE-design, versions are FACT]
- **UI:** Preact 10.29.8 with @preact/signals 2.11.2 and @preact/preset-vite 2.10.6, in TypeScript. Pin typescript 6.0.2 to match the global install; TS 7.0.2 is brand new and a compatibility risk for plugins [ESTIMATE].
  - Preact keeps the single-file bundle small.
  - Diagrams and simulations use inline SVG plus plain Canvas/pointer events: coupling sequence, air-system gauges, off-tracking sweep, placard sorter, danger-zone map. No heavy engine is needed.
- **Build:** Vite 8.3.1, with 7.3.6 as a fallback if singlefile or PWA break on Vite 8's new bundler [ESTIMATE risk]. Two targets:
  1. `build:pwa` uses vite-plugin-pwa 1.3.0 (generateSW, precache everything) for an installable offline app.
  2. `build:single` uses vite-plugin-singlefile 2.3.3 to produce one `index.html` with all JS, CSS, SVG and locale JSON inlined, and the service worker turned off. A service worker needs its own script URL and a secure context, so it cannot live inside one file or run from `file://` [FACT web platform].
  - Budget is under 16 MB; about 381 KB of source markdown times 6 locales should come to well under 5 MB [ESTIMATE].
- **Content pipeline:** a Node script converts markdown to typed JSON: modules, lessons (short "core" view plus "deep dive" view), questions, flashcards, the numbers table, exam traps, and a media registry. Then the Python validator gates it.
- **i18n:** i18next 26.4.2 for UI strings. Lesson content lives in parallel locale files keyed by stable IDs. A "show English term" toggle is always available.
- **SRS and learner model:** ts-fsrs 5.4.2 (whether it implements FSRS-6 is [UNKNOWN]; check its README). Mistakes feed a review list tagged by error type (number confusion, NOT/EXCEPT trap, misconception, missing prerequisite), following `learner/kb_weakness_diagnosis`.
- **Persistence:** Dexie 4.4.6 over IndexedDB.
  - Stores: profile (class plus endorsements, language, exam date), progress, reviewLog, cardState, sessions, schedule.
  - A small localStorage "resume pointer" for the last screen and scroll position.
  - Call `navigator.storage.persist()`.
  - Whether IndexedDB works inside the hosted page's iframe or sandbox is [UNKNOWN]. If it fails, fall back to in-memory storage, which makes export/import mandatory.
- **Export/import progress code:** JSON (with schema version and CRC) → fflate 0.8.3 deflate → base64url string, plus a `.json` file download. The import screen validates and merges by latest-timestamp per record. lz-string 1.5.0 is a smaller alternative.
- **Test harness:** `@playwright/test@1.56.1`, reusing `PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers` with no download.
  - Two projects: desktop 1280×800 and mobile 390×844.
  - `webServer` runs `vite preview` for the PWA build; the single-file build is tested via `page.goto('file://…/index.html')`.
  - Flow tests: choose class, lesson, quiz, answer wrong, check the review queue, reload, confirm resume, export, clear storage, import, confirm restored.
  - Use `page.clock` to fast-forward days for SRS due-date tests. When Playwright added the clock API is [UNKNOWN]; check that 1.56.1 has it.
  - Take screenshots per screen and per locale into the scratchpad, for 1–10 visual scoring (pass mark 8).
  - The globally installed `serve` or `http-server` can serve static builds for manual checks.
