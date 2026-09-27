# Legacy part-01..10 audit (conflicts with DL 650, reuse verdicts)

_Research sweep 2026-09-27 (read-only agents; web access was search-index only — dmv.ca.gov, youtube.com, app stores blocked from the build container). Tags: [FACT src] / [ESTIMATE basis] / [UNKNOWN]. Source agent: `content-old`. Kept verbatim for provenance; corrections from the plan review are in `../BUILD_PLAN.md` and `REVIEW_FINDINGS.md`._

# Old 10-part guide audit: parts 01–10 checked against 00-START-HERE and the new GK/CV lessons

Archive: `/root/.claude/uploads/4f22bdf7-6382-586c-a88b-74190a187585/46413609-Archive.zip`. I read it only with `unzip -p`/`-l` and did not write any files.

## 0. Critical context
- **Only 7 of the 18 new lessons are in the zip.** Present: GK-01, 02, 03, 04, 05, 08 and CV-01, holding 131 practice questions (counted: 18/15/25/19/18/20/16). Missing: GK-06, 07, 09, 10, 11, 12, 13, 14 and CV-02, 03, 04 [FACT `unzip -l`]. 00-START-HERE claims 18 lessons, 338 questions and 657 flashcards; that holds for the full set, not this zip.
  - Consequence: for seeing/communicating, speed, hazards, night/weather, railroad/mountain, emergencies, accidents/fire/alcohol, cargo, CV air brakes, coupling and CV inspection, the **old parts 02/03/04/06 are the only in-zip source.** Either get the missing lessons or reuse the parts after correction.
- 00-START-HERE L76-82 already tells the user to drop parts 01–04, 06 and 07 for GK/CV.
- Appendix A in part-10 (L323-359) claims "127/127 coverage" and "No defects". **It is unreliable.** It checked keyword presence, not correctness, and the errors in §3 got past it.

## 1. Format spec

**Per-file header** (all parts, L1-10): an H1 `# CA CDL Class A — Part N of 10`, an H2 title, then a key/value table with rows `Covers | B1`, `Practice items | 35 table questions`, `Question sets | 15–20 items each, labelled Set n`, `Previous`, `Next`.
- Content blocks are B1–B13, with subsections `## B1.1 …`. These make usable topic IDs.
- The "15–20 items" header is wrong in places: the B11/B12 sets hold 12–13 items.

**Format A: two-column Q→A table** (parts 01–06 and 08–10; 431 items):
```
**Set 1 — questions 1–18 (18 items)**
| # | Question | Answer |
| 1 | CMV BAC that counts as DUI | 0.04 |            (part-01 L178)
| 22 | Waving another driver on to pass | No |       (part-02 L163)
| 14 | En-route cargo checks | Within 50 mi, then every 150 mi or 3 hr, and after breaks |  (part-01 L191)
```
- Stems are noun-phrase fragments, not full questions.
- There are no options or distractors, no explanations and no handbook page references.
- Many answers have several parts.
- These fit flashcards or cloze. They cannot be used as 3-option multiple choice without generating distractors.

**Format B: inline 3-option multiple-choice mock** (part-07 B7.3: 50 questions; part-10 B13.1 air brakes: 25; B13.2 combination: 20; 95 in total):
```
1. Maximum BAC while operating a CMV: a) 0.08 b) 0.04 c) 0.02      (part-07 L85)
**Answer key**
```1b  2b  3b  4a  5b ...```                                          (part-07 L139)
```
- The whole question sits on one line.
- The answer key is a fenced block of number+letter pairs, with no explanation and no page reference.
- **Answer-position bias** [FACT, counted]:
  - part-07: b = 38/50 (76%), a = 9, c = 3.
  - B13.1: b = 21/25 (84%).
  - B13.2: b = 15/20 (75%), a = 4, c = 1.
  - The correct option is also often the longest or most specific one (e.g. part-07 Q50 b "extinguisher, spare fuses, 3 triangles").
  - **Options must be shuffled when imported.**

**How the new GK/CV lessons differ** (e.g. GK-05):
- Header blockquote: `> **Handbook:** Section 2.2–2.3, pages 2-9–2-11 · **Test:** General Knowledge · **Exam weight:** Medium · **Study time:** ~25 min`.
- Fixed sections: `What you'll be able to answer` → `Learn it` (with `*Why:*` reasons, `[CA]` tags and "Note — handbook conflict" callouts) → `Numbers & terms to memorize` (`| Item | Value / meaning | Page |`) → `Exam traps` (`**Trap:** … → **Correct:** … (p. x)`) → `Handbook review questions — answered` (`**TYK 2.2–2.3 #1** … → … (p.)`) → `Flashcards` (`| # | Question | Answer |`, full questions) → `Practice test (real-exam style)` → `### Answer key` → `One-minute recap`.
- New practice tests use scenario stems, NOT/EXCEPT wording, and options on separate indented lines:
  ```
  1. You are stopped on an uphill grade…
     a) … 
     b) …
  ```
- New answer keys give a reason and a page: `1. b — With the clutch partly engaged … (p. 2-9)`.
- Summary: the old parts have no page references, no explanations, no trap notes and no answer-shuffling discipline. The new lessons carry all of these, so the new format is the schema to target.
- Neither set has any YouTube or video links. The only URLs in the old parts are the source list at part-10 L394-399.

## 2. Inventory

| Part | Blocks / topics | Items | Mocks | Test | Stated question count / pass |
|---|---|---|---|---|---|
| 01 | B1: classes, CLP, notification duties, BAC, disqualifications (major/serious/railroad/OOS), phones, inspection types, tire/steering/brake limits, 7-step inspection | 35 table | – | GK | Blueprint L22-38: GK 50/40, AB 25/20, CV 20/16, T 20/16, N 20/16, H 30/24, P 20/16, S 20/16, X = H+N |
| 02 | B2: starting/stopping, backing, trailer backing, double clutch, downshift, retarders, seeing 12–15 s, mirrors, signalling, triangles, following distance, turns, stopping distance 419 ft, slippery roads, hydroplaning, curves, CA 55 mph/lanes | 40 | – | GK | – |
| 03 | B3: hazard clues, distraction, evasive steering, stab/controlled braking, brake/tire failure, ABS dates, skids, crash steps, SR-1, fires/extinguishers, alcohol/drugs/fatigue, federal + CA hours of service, railroad crossings, mountain/snub braking, night/fog/winter/heat/wind/work zones, human trafficking | 50 | – | GK | – |
| 04 | B4: driver cargo duty, weight terms, federal axle limits, balance, blocking/bracing/tie-downs (handbook + 49 CFR 393.110), 0.8/0.5 g, special cargo, hazmat awareness | 45 | – | GK (+ intro to H) | – |
| 05 | B5: 3 brake systems, compressor/governor 100/125, safety valve 150, drains, alcohol evaporator, S-cam/wedge/disc, gauges, <60 psi warning, wig-wag, front limiting valve, spring brakes 20–45, yellow knob, dual system, slack 1 in, 7-step air brake check table, brake lag 32 ft/451 ft, snub braking, fade | 50 | – | AB | 25/20 |
| 06 | B6: rollover (10×), rearward amplification, empty/bobtail stopping, low-clearance units, trailer/tractor jackknife, off-tracking, trailer hand valve, tractor protection valve, red/blue lines, glad hands, dummy couplers, crossed lines, shut-off valves, 16-step coupling, 10-step uncoupling, combination inspection, 4 brake checks | 52 | – | CV | 20/16 |
| 07 | B7: CA speed/lanes, alcohol, phones/headsets, lights, railroad, idling, weigh stations, chains, SR-1, size table (102 in / 14 ft / 40 / 65 / KPRA 40/38), CA hours of service, medical, Employer Pull Notice, ages, triples; endorsement and restriction codes | 0 table | 50-question GK mock (pass 40) | GK | 50/40 |
| 08 | B8: doubles/triples driving, converter dolly and its ABS, heavy trailer first, coupling/uncoupling doubles and triples, valve/petcock states, inspection. B9: tank definition (>119 gal each, ≥1,000 gal total), leaks, valves/manholes, special equipment, high centre of gravity, surge, bulkheads/baffles/smooth bore, outage, density, driving | 30 T + 30 N | – | T, N | 20/16 each |
| 09 | B10: roles, hazmat table columns 1–6 and symbols, classes 1–9, shipping papers, marking/labels/placards, bulk thresholds, Table 1/Table 2, DANGEROUS placard, loading by class, transport index 50, tires, routes, 300 ft/5 ft/100 ft distances, attendance, chlorine, National Response Center reporting | 50 | – | H | 30/24 |
| 10 | B11 passengers: inspection, forbidden hazmat, 500/100 lb limits, standee line, railroad/drawbridge, prohibited practices. B12 school bus: danger zones 30/10 ft, mirrors, loading/unloading, railroad, evacuation. B13: air brake and combination mocks. Quick-reference sheet, Appendix A (self-audit), Appendix B (study order), sources | 25 P + 24 S | 25 air brakes (pass 20) + 20 combination (pass 16) | P, S, AB, CV | 20/16 each |

- **Total: 526 items** (part-10 L329-337, which I re-counted and confirmed). That is 431 table items and 95 multiple-choice questions.
- **Question counts are not from the DMV.** They come from third-party sites (part-10 L397). A web search gives H as 20/16, against the part's 30/24 [UNKNOWN; the DMV site is blocked from here, so this needs checking].
- The Air Brakes 25/20 figure is supported by third-party sites [PLAUSIBLE].
- The whole guide is framed as "Class A". There is no Class B or C path.

## 3. Conflicts with the handbook
"HB" below means the California handbook (DL 650), as reflected in 00-START-HERE and the new lessons, which were checked against it.

| # | Where | Old text | Handbook way | Source |
|---|---|---|---|---|
| 1 | p01 B1.2 L125; Q29 L211 | "OOS 1st: 180 days–1 year. 2nd within 10 years: 2–5 years" | 90 days (–1 yr); 2nd 1–5 yr (1 yr in GK-02); 3rd 3–5 yr. p01 L126 (hazmat/16+ passengers: 180 days–2 yr) **matches** HB | [FACT START-HERE L66; GK-03 L60-63 p.1-19; GK-02 L65,71] |
| 2 | p03 B3.10 L137; p07 B7.1 L30; p07 Q48 L132 + key 48b | Dim high beams "300 ft when following (CA)" | 500 ft oncoming **and** following | [FACT START-HERE L67,79] |
| 3 | p04 L80, Q35 L141; p09 L75, Q10 L166; p10 quick-ref L310; App A L359 | Placard "10¾ in." | at least 9.84 in (250 mm) | [FACT START-HERE L68] |
| 4 | p01 B1.1 L83 | "Class C: … carries 16+ people including the driver" | CDL needed at more than 10 incl. driver; a paid or nonprofit vehicle for more than 10 is **Class B** in the class chart (Fig 1.1) | [FACT GK-01 L28, L38, L64] |
| 5 | p10 B11 L16; Q1 L75 | P endorsement "16 or more people including the driver" | P = built for "10 or more" incl. driver (p.1-1 says "more than 10") | [FACT GK-01 L128, L138] |
| 6 | p01 B1.1 L81-85 | Class list has no 3-axle rule | [CA] 3-axle vehicle over 6,000 lb needs a CDL (Class B) | [FACT START-HERE L82; GK-01 L29, L62] |
| 7 | p02 B2.2 L48; Q5 L142 | "The gear you'd use to climb the hill is usually right for descending it" / "the climbing gear" | usually **lower** than the climbing gear (p.2-11) | [FACT GK-05 L170, L181; START-HERE L98] |
| 8 | p10 B11.3 L47; Q4 L78 | Standee line "(or a 2-ft line behind the driver's seat)" | a **2-inch** line on the floor; no rider forward of the rear of the driver's seat | [FACT from web-search snippet quoting DMV Section 4; not fetched directly] |
| 9 | p10 B12.3 L153 | "California uses flashing red lights at rail crossings for school buses" | HB section 10: turn on **hazard** lights about 200 ft before the crossing; CVC 22112 limits red lights and stop arm to loading/unloading | [PLAUSIBLE, from search snippets of DMV section 10 and CVC 22112] |
| 10 | p01 L43 | Tests "in English or Spanish" | English, Arabic, Chinese, Punjabi, Russian, Spanish (p.1-10); confirm with DMV because of the 2025–26 English-only moves | [FACT GK-01 L192, L197; START-HERE L25] |
| 11 | p07 B7.2 L74-81 | 6 restriction codes (L, Z, E, O, K, V); "K = under 21 or no federal medical" | 10 codes E, K, L, M, N, O, P, V, X, Z plus CA 88; K = certified intrastate | [FACT GK-01 L140-157] |
| 12 | p04 B4.4 L59; p07 Q28 option c | 49 CFR 393.110 tie-down count (a 25-ft load → 4; a ≤5-ft load ≤1,100 lb → 1) | HB: 1 per 10 ft, never fewer than 2 (25 ft → 3). The CFR answer is also offered as a distractor, which is ambiguous | [FACT START-HERE L111; p04 L58] |
| 13 | p04 B4.2 L39; Q10-12 | Federal 20,000 single / 34,000 tandem / 80,000 gross | [CA] HB p.1-23: 20,000/10,500 single vehicle; 18,000/9,500 in combinations; steer axle 12,500; CVC tables for axle groups | [FACT GK-03 L167-181] |
| 14 | p02 L129; p07 L19 | CA 55 mph = 3+ axles and towing only | plus school bus with pupils, farm labor vehicle, explosives, trailer bus; turnout rule when 5+ vehicles are behind | [FACT GK-03 L224-230, L264] |
| 15 | p07 L34; Q47 | Idling limit for vehicles ">10,000 lb" | over 10,001 lb GVWR, 5 min, **no idling at schools** | [FACT GK-03 L82] |
| 16 | p07 L36 | Stop at weigh stations "when open or directed" | every CMV stops at each posted CHP site | [FACT GK-03 L70] |
| 17 | p01 L146; Q25 | "A missing leaf … dangerous" | out of service when ¼ or more of the leaves are missing | [FACT GK-04 L189] |
| 18 | p01 L149 | Emergency equipment = 3 triangles | 3 triangles **or** 6 fusees **or** 3 liquid-burning flares | [FACT GK-04 L192] |
| 19 | p01 L128 | Phone fines $2,750 / $11,000 (federal) | [CA] hands-free/texting: 2nd in 3 years = 60 days + 1 point; 3rd = 120 days + 1 point, in any vehicle | [FACT GK-02 L162] |
| 20 | p01 L108; p03 L93, Q26 | "alcohol within 4 hours of duty → 24-hour OOS" | HB: any measurable alcohol under .04 → 24 h out of service. The 4-hour rule is federal (49 CFR 392.5) | [FACT GK-02 L41; the 4 h rule's presence in HB is UNKNOWN] |
| 21 | p03 L104, Q31; p10 quick-ref L316 | Federal 30-minute break after 8 hours | HB hours-of-service table has no break rule; "not on the test" | [FACT GK-03 L290-309] |
| 22 | p09 L116, Q29; p10 L313 | Hazmat duals "every 2 hours or 100 miles" | federal periodic check removed in 2002 (start of trip and each park remain); what DL 650 prints is unknown | [FACT Fed. Reg. 2002-10-04; DL 650 text UNKNOWN] |
| 23 | p09 L86 | "Some older handbook editions print 5,000 lb" | DMV section 9 prints **2,205 lb** — keep 2,205 and drop the caveat | [PLAUSIBLE, web snippet of DMV section 9] |
| 24 | p09 L83 | Table 1 list lacks 5.2 (temperature-controlled organic peroxide Type B); gives "6.1 PG I inhalation" | federal model manual lists 5.2 and "6.1 inhalation hazard" | [ESTIMATE, federal model manual; verify DL 650] |
| 25 | p10 L39 | "Small-arms ammunition (ORM-D)" | ORM-D phased out federally around 2020; the handbook may still print it — answer the handbook way | [ESTIMATE; verify] |
| 26 | p01 L42, L91 | ELDT only before the "Class A skills test" | ELDT (49 CFR 380, since Feb 2022) also applies to Class B, upgrades, P, S and H (H before the knowledge test). No new lesson mentions it | [ESTIMATE; verify] |
| 27 | p07 L50-51 | KPRA 40/38 ft | not in HB size section (GK-03 has 40/65/75 ft, 102 in, 14 ft) — extraneous | [FACT GK-03 L96-155] |
| 28 | p03 L143, L149; p07 L38 | Chain controls R1–R3; work-zone fines doubled | not found in the zip's new lessons (GK-10 is missing) | [UNKNOWN] |
| 29 | p02 L116 | Wet roads "55 → about 35–40" | federal model says "about 35" | [UNKNOWN, minor] |

Checked and **consistent** [FACT]:
- Tractor protection valve / spring brakes 20–45 psi (p05 L56, L92; p06 L43).
- Fifth wheel tilted down (p06 L70, Q28).
- Tie-down working load limit ½ (p04 L58).
- Hand valve for testing only on the CV test (p06 L41). The GK exception (p.2-9: it may hold a rig on a start) is missing.
- 50→90 psi in 3 min in the p01 inspection (matches HB section 2.1, GK-04 L118).
- Following distance, 419 ft, triangle placement, 15–50 ft, 60/120/1-yr railroad penalties, 60/120-day serious violations, 14-day CLP wait.

## 4. Reusability verdict
- **p01 — Supersede.** GK-01, 02 and 04 are in the zip and cover it better. Contains conflicts 1, 4, 6, 10, 17–20, 26.
- **p02 — Supersede B2.1, B2.2 and B2.5** (GK-05, GK-08 exist).
  - B2.3, B2.4 and B2.6 (seeing, communicating, speed) can be reused for now until GK-06/07 are obtained.
  - Must fix #7.
  - Verify against DL 650 pp. 2-12–2-18.
  - Add CA rules: 2 mirrors with a 200-ft view (p.2-13), service brake stop from 20 mph (p.2-16), 300-ft following distance behind trucks (p.2-18) [FACT GK-03 L23].
- **p03 — Interim reuse** for GK-09 to GK-13 content, none of which is in the zip.
  - Fix #2; tag or remove #20, #21 and #28.
  - Hours of service is superseded by GK-03.
  - Verify every number against pp. 2-21–2-49: ABS dates, railroad 14 s/15 s, stuck-on-tracks procedure, SR-1, extinguisher.
- **p04 — Interim reuse of B4.1, B4.3, B4.5** (GK-14 is not in the zip).
  - Remove #12 (CFR rule), #13 (federal weights) and the 0.8/0.5 g figures (CFR, not HB).
  - Fix placard size in B4.6.
  - Verify against pp. 3-1–3-5 and 2-44–2-49.
- **p05 Air Brakes — Reuse.** No conflicts found.
  - Add page references, explanations, the handbook's air-brake "Test Your Knowledge" answers, a system diagram, and the in-cab air brake check sequence.
  - Verify against DL 650 section 5: 100/125/150 psi, 85→100 in 45 s, leakage 2/3 and 3/4 psi/min, 1-inch slack, primary = rear axle.
  - The user has already passed AB; this matters for other learners.
- **p06 CV — B6.1 is superseded by CV-01.**
  - B6.2–B6.5 can be reused until CV-02 to CV-04 are obtained. They are consistent with 00-START-HERE.
  - Add the trailer ABS lamp (yellow, left side, trailers built from 3/1/1998; currently only in p03) and the GK hand-valve nuance.
  - Verify against pp. 6-5–6-17.
- **p07 — Supersede the CA-rules block and restriction codes** (GK-03, GK-01).
  - The GK mock can be reused as an item bank after: changing Q48 so the correct answer is 500 ft; fixing Q28's distractor c; fixing Q47 (10,001 lb / schools); shuffling all options; and adding topic tags and page references.
- **p08 T/N — Reuse** (the new lessons don't cover these).
  - Verify against DL 650 sections 7 and 8.
  - Add: cement trucks count as tanks, CLP holders get restriction X (GK-01 L127, L153), triples illegal in CA (p.7-1).
  - Needs distractors, explanations, page references and a 20-question mock for each.
- **p09 H — Reuse with corrections:** #3, #22, #23, #24.
  - Verify the National Response Center thresholds ($50,000 / 1 h), the 30-day written report, attendance within 100 ft, and the 25-ft smoking rule against DL 650 section 9.
- **p10 P — Reuse after #5 and #8.**
  - Verify against DL 650 section 4.
- **p10 S — Rebuild.** It is sourced from the **Oregon manual** (L396), not California.
  - Contains #9.
  - Missing CA law: CVC 22112 escort for pre-K to grade 8 with a hand-held STOP sign; red lights and stop arm at every loading/unloading stop [FACT, search snippet of CVC 22112].
  - Verify against DL 650 section 10.
- **B13 air-brake and combination mocks — Reuse** after shuffling options.
- **Quick-reference sheet — regenerate** from the corrected data.
- **Appendix A — discard.**

## 5. Gaps (thin or missing everywhere in the zip)
1. **Skills tests: zero content.** p10 L357 explicitly excludes DL 650 sections 11–13, and GK-01 covers only the rules (3 attempts, only the section 11 guide allowed as an aid).
   - Needed: class-specific pre-trip inspection scripts (A tractor-trailer, B straight truck, bus/school bus, with verbal "checking for" lines), the in-cab air brake test, the basic control exercises (straight-line backing, offset back left/right, parallel park both sides, alley dock; scoring by encroachments and pull-ups), and road-test manoeuvres [ESTIMATE, from the federal model manual; verify DL 650 sections 11–13].
2. **ELDT, and the Class B/C learning paths:** absent.
3. **Other endorsements and certificates:**
   - F (firefighter) endorsement: no content.
   - Restriction 88 and horse trailers: only in GK-01.
   - CA certificates (school bus driver certificate, transit training, SPAB, youth bus, farm labor vehicle, hazardous agricultural materials): listed in GK-01 only, with no study material.
4. **School bus (S):**
   - CA escort rule, red lights/stop arm, no idling at schools, annual medical at age 65+.
   - The handbook's railroad procedure: hazard lamps, containment/storage area.
   - Student management, strobe lights, high winds, evacuation plan detail.
   - No mock.
5. **Passenger (P):** brake-door interlock, curves, after-trip inspection detail, the handbook's "Test Your Knowledge" answers, no mock.
6. **Tank (N):** thin — about 60 lines. No mock, no worked outage/density examples, no surge/rollover interactivity.
7. **Hazmat (H):**
   - Class/division definitions (1.4–1.6, poison-inhalation zones), subsidiary placards, table columns 7–10.
   - Hazardous-substances (RQ) and marine-pollutant appendices, full segregation table (only 2 examples given).
   - Class-specific emergency responses, cargo-tank attendance during loading, explosives paperwork.
   - Placard-selection and shipping-paper exercises, ERG lookup drills.
   - No mock.
8. **Doubles/triples (T):** no mock, no step-by-step coupling-sequence interaction.
9. **Across all old parts:**
   - No handbook page references, so review items can't link to handbook pages.
   - No explanations, so mistake analysis has no "why" text.
   - English only; no visuals, diagrams or video links.
   - The GK-01 to GK-14 "Test Your Knowledge" set is only partly present (7 lessons).

Sources: [DMV Section 4 (search snippet)](https://qr.dmv.ca.gov/portal/handbook/commercial-driver-handbook/section-4-transporting-passengers-safely/) · [DMV Section 9 (search snippet)](https://qr.dmv.ca.gov/portal/es/handbook/commercial-driver-handbook/section-9-hazardous-materials/) · [DMV Section 10 (search snippet)](https://www.dmv.ca.gov/portal/handbook/commercial-driver-handbook/section-10-school-buses/) · [CVC 22112](https://california.public.law/codes/vehicle_code_section_22112) · [Fed. Reg. 2002 tire-check revision](https://www.federalregister.gov/documents/2002/10/04/02-25226/revision-to-periodic-tire-check-requirement-for-motor-carriers-transporting-hazardous-materials) · [cdlpracticetest CA (third-party counts)](https://cdlpracticetest.com/california/). dmv.ca.gov and qr.dmv.ca.gov could not be fetched (egress blocked), so everything marked "search snippet" still needs checking against the DL 650 PDF.
