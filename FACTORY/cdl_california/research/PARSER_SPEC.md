# Lesson parser spec, inventory, topic maps, handbook-vs-other conflicts

_Research sweep 2026-09-27 (read-only agents; web access was search-index only — dmv.ca.gov, youtube.com, app stores blocked from the build container). Tags: [FACT src] / [ESTIMATE basis] / [UNKNOWN]. Source agent: `content-new`. Kept verbatim for provenance; corrections from the plan review are in `../BUILD_PLAN.md` and `REVIEW_FINDINGS.md`._

# CA CDL lesson files: parser spec, inventory, topic maps and handbook-vs-other-sources conflicts

Sources: 8 archive members, every line read with `unzip -p`. Tags used below:
- `[F:file]` means the file says it. "Handbook" claims are as the file states them; I did not check them against DL 650 itself.
- `[D]` means I derived the number with awk counts over the file text.

## 1) Parser spec

**Encoding** `[D]`
- UTF-8, LF line endings, no BOM.
- Characters to normalize: `·` (U+00B7), `—` (em dash), `–` (en dash), `→`, `½ ¼ ¾ ≈ § × ≤`, `−` (U+2212 minus; GK-03 answer key line 17).
- The apostrophe in "What you'll" is ASCII.

**Section order** (fixed in all 7 lessons):
- H1 → meta blockquote → `## What you'll be able to answer` → `## Learn it` → `## Numbers & terms to memorize` → `## Exam traps` → `## Handbook review questions — answered` → `## Flashcards` → `## Practice test (real-exam style)` → `### Answer key` (an H3 inside Practice test) → `## One-minute recap`.
- Always scope element regexes to their H2 section. Learn it contains numbered lists (for example GK-02 "1. Driving a CMV at .04…") that look like practice questions.

| Element | Grammar / regex | Literal example |
|---|---|---|
| H1 | `^# (GK\|CV)-(\d\d) · (.+)$` | `# GK-04 · Vehicle inspection` |
| Meta | `^> \*\*Handbook:\*\* (.+), pages (\d+-\d+)–(\d+-\d+) · \*\*Test:\*\* (.+?) · \*\*Exam weight:\*\* (High\|Medium) · \*\*Study time:\*\* ~(\d+) min$`. The en dash sits directly next to hyphenated page numbers, so split on `–`. | `> **Handbook:** Section 2.1.1–2.1.7, pages 2-1–2-9 · **Test:** General Knowledge · **Exam weight:** High · **Study time:** ~45 min` |
| Objectives | `^- ` bullets, 5–6 per file | `- The **3 kinds** of required emergency equipment.` |
| Learn-it subsection | `^### (?:(\d+(?:\.\d+)*) )?(?:\[CA\] )?(.+?)(?: \((.+)\))?(?: \[CA\])?$` | `### 1.3.8 [CA] Violation point counts (p. 1-15)` / `### Width (p. 1-22) [CA]` |
| [CA] tag | `\[CA\]`, optionally wrapped in `**` | `- **[CA]** A **3-axle** vehicle that weighs **more than 6,000 lb**.` |
| Callout (blockquote) | `^> (\*\*)?(Note\|Note — handbook conflict\|Handbook conflict\|Handbook vs other sources\|Beyond the handbook — not on the test\|Wording note — beyond the handbook\|Other California rules live…)` | `> Handbook vs other sources: a few practice websites say "5 times". The handbook says **10 times** — use 10 on the test.` |
| Why | `(^\|\s)\*?Why[^:]*:\*?` and `\*\*Why you need it.\*\*`; lowercase `(*why:* …)` also appears inline | `*Why:* a train cannot stop quickly.` |
| Worked example | `(\*{1,2})?(Worked examples?\|Examples?)( —[^*]+\| \([^)]*\))?:?(\*{1,2})?`; plus `Memory hook`, `Timeline:` | `*Worked example:* a spring has 8 leaves. ¼ of 8 = 2, so 2 or more missing = out-of-service.` |
| Learn-it tables | 2–5 columns, free headers (6/4/6/4/3/3/1 tables per file `[D]`) | `\| Step \| What you do \| Engine \|` |
| Numbers table | Header is always `\| Item \| Value / meaning \| Page \|`; every row has exactly 3 cells `[D]` | `\| Front tire tread \| **4/32 inch** in every major groove \| 2-2 \|` |
| Exam trap | `^- \*\*Trap:\*\* (.+?) → \*\*Correct(?: for [^:]+)?:\*\* (.+) \(((?:pp?)\. [^)]+)\)$`. The reason is merged into the Correct text with no separator. All 97 traps end with a page reference `[D]`. | `- **Trap:** pump 5 times, hold 3 seconds → **Correct:** pump **3**, hold **5**. (p. 2-8)` |
| Handbook Q | Either the placeholder `_This handbook section has no review box — see Flashcards and Practice test._`, or `^\*\*TYK (.+?) #(\d+)\*\* (.+)$` followed by `^→ (.+)$` | `**TYK 6.1 #2** You are pulling doubles and turn suddenly. Which trailer is most likely to overturn?` / `→ The rear (last) trailer. … (p. 6-1)` |
| Flashcards | `\| # \| Question \| Answer \|`; rows `^\| (\d+) \| (.+) \| (.+) \|$`. **No page column.** | `\| 16 \| Leaf spring out-of-service when? \| ¼ or more leaves missing \|` |
| Practice Q | stem `^(\d+)\. (.+)$`, options `^\s{3,4}([abc])\) (.+)$`. Always exactly 3 options: 393 options = 131 × 3 `[D]`. | `7. How many red reflective triangles must you carry?` / `   b) 3` |
| Answer key | `^(\d+)\. ([abc]) — (.+) \(((?:pp?)\. [^)]+)\)$`. All 131 lines match `[D]`. **It includes the explanation and the page**, and some explanations say why the distractors are wrong. | `10. b — 102 in is the limit; 96 in is what local roads may post; 108 in is wheel to wheel. (p. 1-22)` |
| Recap | `^- ` bullets, 8 per file `[D]` | `- Retarders: 4 types (exhaust, engine, hydraulic, electric), drive wheels only, **OFF on wet, icy, or snowy roads**.` |

**Inconsistencies that break a naive regex parser** `[D]`
1. **Blank lines between practice questions.** GK-01, GK-04, GK-05 and GK-08 have them; GK-02, GK-03 and CV-01 do not. Split on `^\d+\. `, not on blank-line blocks.
2. **Option indent** is 3 spaces for Q1–9 and 4 spaces for Q10+.
3. **Numbers "Page" column formats vary.**
   - GK-03 writes `p. 1-16`; the other files write bare `1-16`.
   - Also seen: `2-19 (Fig. 2.12)`, `2-20, Fig. 2.13`, `2-19 – 2-20`, `1-4, 1-16`.
   - Fix: pull every `\d+-\d+` token out of the cell.
4. **Page references in prose vary.**
   - Forms: `pp. X to Y`, `pp. 2-1 – 2-2` (spaced en dash), `pp. 1-6 to 1-7, 1-11`, `(Figure 1.1, p. 1-8)`, `(p. 6-1; Figure 6.1 on p. 6-2)`.
   - Anomalies: `p. X to Y` with "p." on a range (GK-05 `### 2.3.1 … (p. 2-10 to 2-11)`), and `(p. 6-3, 6-4)` or `(p. 2-10, 2-11)` with "p." on a list (CV-01 traps and answer key, GK-05 answer key).
5. **Position of the [CA] tag** changes:
   - suffix on a heading (GK-01, GK-03);
   - after the section number (GK-02 `### 1.3.8 [CA] …`);
   - bold inline `**[CA]**`;
   - `[CA]` prefix in Numbers items, flashcard questions and practice stems (GK-02 Q13–15);
   - suffix in GK-03 Numbers items (`Single vehicle length [CA]`);
   - `**[CA]** Triples` (CV-01);
   - `**[CA] Related California rule:**` (GK-08).
6. **TYK labels differ by file:**
   - `TYK Subsection 2.1 #n` (GK-04)
   - `TYK 2.2–2.3 #n` (GK-05)
   - `TYK 2.7–2.8 #n` (GK-08; the label goes beyond the lesson's own scope of 2.7)
   - `TYK 6.1 #n` (CV-01)
   - GK-01, GK-02 and GK-03 use the placeholder instead.
7. **Trap variants.**
   - The label `**Correct for section 2.1:**` appears once (GK-04).
   - Trap text is sometimes in quotes.
   - Sometimes there is a `.` before ` → ` (GK-03, GK-05, GK-08, CV-01).
8. **The key-words preamble has 6 forms:**
   - GK-01: `### Key words first` (an H3 with no page, so it counts as a subsection)
   - GK-02: `**Key words.**`
   - CV-01: `**Key words**`
   - GK-03: `Words used a lot:`
   - GK-04: `*Terms used below:*`
   - GK-05 and GK-08: none.
9. **Bold "pseudo-subheadings" that are not headings:**
   - GK-04: `**Tires** (p. 2-2).` and `**Step 2 – Engine compartment** (p. 2-4):` (en dash)
   - GK-05: `**Rule 1 — …**` (em dash)
   - GK-03: `**Who to tell, and how fast**`
   - GK-08: `**Right turns** (p. 2-20; Figure 2.13 on p. 2-21)`
   - GK-04 §2.1.5 alone holds 7 steps, 5 tables and the walk order.
10. **A "Beyond the handbook" note that is not a blockquote:** GK-04 line 176 `*Beyond the handbook — not on the test:* …`.
11. **Nested bullets** use a 2-space indent (GK-02 `  *Why:*`, GK-03 phone list and worked-example sub-bullets).
12. **Flashcard prompt styles are mixed:** questions, statements without "?" (all of GK-02), and cloze with `___` (GK-08 #11). Flashcards have no page, so a page must be inferred by joining to the Numbers table or Learn it.

## 2) Inventory `[D]`

| File | H3 subs | Numbers rows | Traps | TYK Qs | Flashcards | Practice Qs | Pages | [CA] total / in Learn it | Learn-it tables / callouts | Weight, time |
|---|---|---|---|---|---|---|---|---|---|---|
| GK-01 | 21 | 28 | 14 | 0 (placeholder) | 35 | 18 | 1-1–1-13 | 24 / 17 | 6 / 4 | Med, 40 |
| GK-02 | 10 | 21 | 12 | 0 (placeholder) | 30 | 15 | 1-14–1-16 | 25 / 3 | 4 / 1 | High, 35 |
| GK-03 | 27 | 49 | 22 | 0 (placeholder) | 40 | 25 | 1-16–1-30 | 39 / 24 | 6 / 6 | Med, 40 |
| GK-04 | 7 | 21 | 13 | 11 | 41 | 19 | 2-1–2-9 | 0 / 0 | 4 / 0 (+1 italic) | High, 45 |
| GK-05 | 11 | 25 | 11 | 8 | 34 | 18 | 2-9–2-11 | 0 / 0 | 3 / 1 | Med, 25 |
| GK-08 | 8 | 23 | 13 | 4 | 30 | 20 | 2-19–2-21 | 1 / 1 | 3 / 0 | High, 35 |
| CV-01 | 7 | 18 | 12 | 6 | 35 | 16 | 6-1–6-4 | 3 / 1 | 1 / 2 | High, 35 |
| **Σ** | 91 | 185 | 97 | 29 | 245 | 131 | | 92 | | |

- **Answer-letter spread (a/b/c):** GK-01 6/6/6, GK-02 4/7/4, GK-03 9/7/9, GK-04 6/6/7, GK-05 6/7/5, GK-08 6/7/7, CV-01 6/5/5. Fairly balanced.
- **Pages cited outside each lesson's own range** (these are cross-links the app should follow):
  - GK-02 → 2-35
  - GK-03 → 1-4, 2-13, 2-16, 2-18, 2-31, 3-5, 7-1
  - GK-04 → 5-9
  - GK-05 → 6-5
  - GK-08 → 2-18
  - CV-01 → 1-5
- **Other zip contents:** there are also 10 older files (`part-01…part-10`). START-HERE says to drop parts 01–04, 06 and 07 because they follow federal answers `[F:START-HERE]`. By title, part-02, 03, 04 and 06 cover the missing GK-06…14 and CV-02…04 topics `[ESTIMATE from titles]`. They could be interim content, but only after applying §5 as overrides. Part-06's opening lines match CV-01's facts (10×, 2.0, release the brakes) `[F:part-06]`.

## 3) Topic maps

Type codes: NUM = numeric-fact, DEF = definition, SEQ = procedure-sequence, SPA = spatial-visual, CAU = causal-system, DEC = decision-rule, LEG = legal-table, CMP = comparison.

I'm proposing 7 reusable engines, each skinned per topic:
- **E1** threshold slider
- **E2** decision-tree router
- **E3** timeline / rolling-window
- **E4** SVG hotspot map
- **E5** sequence ordering
- **E6** 2D kinematic rig sim
- **E7** calculator with a "whichever-first / subtract-allowance" mode

### GK-01

| Sub-topic | Type | Best modality → topic-specific mechanic |
|---|---|---|
| Key words (GVWR/GCWR, endorsement vs restriction) | DEF | Labelled silhouette. **"Rating Tag":** drag GVWR onto a single truck and GCWR onto a combination; wrong drop plays a short animation of the correct one. |
| Who needs a CDL | NUM, DEC | E1. **"One-Pound Gate":** weight slider; the gate opens only at 26,001 / 10,001; round-number decoys at 26,000 / 10,000; [CA] 3-axle >6,000 lb bonus gate. |
| Passenger vans [CA] | DEC | E2 with 4 ordered nodes. **"Van Triage":** route church, vanpool and for-pay shuttle cards; points lost for checking out of order (vanpool must come before >15). |
| Horse trailer / R88 [CA] | DEC, NUM | E7. **"GCWR Builder":** pickup GVWR + loaded trailer → A, A-88 or none. |
| Class selection (Fig 1.1) | DEC | E2. **"Class Sorter":** compose power unit + towed unit, predict A / A-88 / B / C / none; also feeds the app's class onboarding. |
| CLP rules | SEQ, NUM | E3. **"CLP Calendar":** place Day 0, 14, 180 and the 1-year cap; endorsement picker allows only N/P/S; CLP-N → restriction X, CLP-P/S → restriction P. |
| Age rules [CA] | DEC | **"Job Board at 19":** accept or reject gigs (Fresno→LA ok; Reno no; placarded no). |
| Documents to bring | SEQ | **"DMV Backpack":** pack valid docs; photocopy birth certificate and non-SS-card proof bounce back; DL 44C consent pop-up. |
| Test tries / time limits | NUM | **"3 Hearts":** one shared life bar across inspection, basic control and road; knowledge retakes cost no wait; fee lasts 12 months. |
| Driver duties | LEG | **"Notify Matrix":** match event → recipient → deadline (30 d DMV; 30 d employer on DL 535; next business day for suspension; 10-yr history). |
| Endorsements | DEF | **"License Card Builder":** stamp letters to fit a job; T+triples in CA is blocked; N tank sliders 119 / 1,000 gal. |
| Restrictions | DEF, DEC | **"Test-Truck → Code":** configure the skills-test vehicle (automatic, air-over-hydraulic, pintle hook) and predict E/L/Z/O; "same letter, different meaning" duel cards for N, P, X. |
| No-CDL exemptions | DEF | Swipe **"Exempt or Not"** (vanpool, motorhome 40–45 ft, fifth-wheel >15,000 lb not for pay). |
| Special certificates [CA] | LEG | **"Certificate Desk":** sort each job to DMV or CHP; number chips GPPV 24, Youth Bus 16, HAM 21 yrs / 50 mi. |
| General / 1.1.1 knowledge tests | DEF | **Personal test-list generator** driven by the chosen class and endorsements; 12-month waiver rule. |
| Testing rules | DEC | E4. **"Spot the Auto-Fail":** hotspot scene (phone, tape mark on curb, dash cam, bystander signalling). |
| 1.1.2 skills tests | SEQ, NUM | Timeline 40/30/45–60 min; **"Road test required?"** rapid cards (expired >2 yrs, P/S, remove restriction). |
| What to study (Fig 1.2) | DEC | Powers the **app's own path builder** (A → sections 1, 2, 3, 6, 11–13, +5 with air). |
| 1.2 Medical | SEQ, DEF | **"MER vs MEC"** drag; expiry clock (2 yrs; 65+ school bus yearly; mail 4 wks ahead); "no CA variances" myth-buster. |
| 1.2.1 Inter/intrastate | DEC | **"Shipment Tracer":** map animation of cargo origin decides NI or NA (40/K); overseas-port case. |

### GK-02

| Sub-topic | Type | Mechanic |
|---|---|---|
| 1.3.1 General | DEF | One card |
| 1.3.2 Alcohol / major offenses | NUM, LEG | **"BAC Meter":** slider from .00 to .10; any measurable level → 24 h out of service; ≥.04 → 1 yr (3 yrs placarded); **"Two-Strike Ladder"** where the 2nd offense = life; drug felony = instant life. |
| 1.3.3 Serious violations | DEF, NUM | **"Radar Gun":** posted limit vs clocked speed, flag ≥15 over; E3 3-year window where the 2nd token triggers 60 d and the 3rd 120 d. |
| 1.3.4 OOS order | NUM | E3 **10-year** window (contrast with 3-year); "handbook vs federal" badge. |
| 1.3.5 RR crossing (6 offenses) | DEC | Sort offenses by "who" (always-stop drivers / others / all); 60 d → 120 d → 1 yr ladder. |
| 1.3.6 HazMat TSA | DEF | **"TSA Desk":** approve or deny applicant files |
| 1.3.7 Personal-car spillover | CAU | Chain diagram: car suspension → CDL; "no hardship CMV" lock |
| 1.3.8 Points [CA] | NUM | E7 **"Point Calculator":** 1 or 2 points, ×1.5 in a CMV, rolling 4/12, 6/24, 8/36 meters, hearing +2; **"Record Shelf-Life"** ordering 55-15-10-4-3 |
| 1.3.9 Hands-free [CA] | NUM | Strike counter that works in any vehicle |
| Summary table | LEG | **"Penalty Matrix"** fill-in grid used as the boss level |

### GK-03

| Sub-topic | Type | Mechanic |
|---|---|---|
| 1.4 Who to tell | LEG | Shares the Notify Matrix: 30 d / 2 business days / 24 h; phone rule "one-button" tap test; seat belt 4× stat |
| Disqualification summary tables | LEG | Adds a HazMat / 16+ passengers row to the matrix (180 d–2 yr) |
| 1.4.1 State laws | DEC | "Must stop?" scale-sign cards; liability split between owner and driver |
| CARB | LEG, NUM | **"Idle Timer"** 5:00 countdown, 0 at a school; match rule to vehicle (10,001 / 14,000 / 6,000 lb) |
| Length (single, combination, exceptions) | NUM, SPA | **"Rig Builder":** snap units together, live tape reads 40 / 65 / 75, each trailer ≤28′6″, 80-ft pole dolly |
| Overhang | NUM, SPA | E7 wheelbase slider → 2/3 rear limit; front 3 ft (4 ft if the load is vehicles) |
| Width | NUM, SPA | Top-down gauge 96 / 102 / 108 / 120; mirror 10 in, handles 3 in |
| Farm equipment | DEF | Card |
| Height | NUM, SPA | Bridge gate 14′ (14′3″ for double-deck bus) |
| Weight limits (general) | DEC | "Posted-limit street: may I enter?" for direct-route pickup, delivery, construction, utility |
| Axle weights | NUM | **"Axle Scale":** drag load onto axles; 20,000/10,500, 18,000/9,500, steer 12,500; wording-note flag |
| Logs / weight-to-axle ratio | CAU | Bridge-formula spacing slider: closer axles → less weight; 34k ×2 = 68k at ≥36 ft |
| Permit routes / loading | DEC | Map detour limited to nonresidential streets; misdemeanor trap |
| CHP scale | NUM | E7 **"Scale House":** (reading − limit − 200) ≥ 100 → cite; HazMat and livestock (1,000 / 2,000) exception cards |
| Oversize permits, MCP, UCR | DEF, DEC | Sorters: Caltrans vs city/county; MCP needed? (2+ axles and >10,000 lb) |
| Speed limits [CA] | DEC | **"55 Club"** bouncer: vehicle cards (pickup towing, 2-axle not towing) |
| Right lane / designated system | SPA | Top-down freeway lane picker (4+ lanes each way unlocks the 2nd lane); 1-mile off-route radius map |
| Slow-vehicle turnout | NUM | Queue builds behind the truck; pull out at 5 |
| HOS: which rules | DEC | Shipment Tracer (reused) |
| HOS: federal vs CA | NUM, CMP | **"Duty Clock":** 24-h log grid, on duty at 5:00 → last drive 9 pm (CA) vs 7 pm (fed); 12/16/80 vs 11/14/60–70 |
| Record of duty status | DEF | Card: who may demand the log |
| SR 1 accident report | DEC, NUM | **"Crash Desk":** scenarios (parking lot, not at fault, $1,500) → SR 1 to DMV in 10 d; employer 5 d |
| Financial responsibility | LEG | Cargo → coverage match ($300k / $750k / $1M / $5M) |
| IRP/IFTA/IVDR | SEQ, DEC | **"Trip Log":** record the odometer at start, each state line and end; qualify-vehicle check; keep 4 yrs, file quarterly |

### GK-04

| Sub-topic | Type | Mechanic |
|---|---|---|
| 2.1.1 Why inspect | DEF | "#1 reason" (safety) and "who is responsible" (you) cards |
| 2.1.2 Pre / during / after | SEQ | E3 trip timeline; "sign the report only if certified" rule gate |
| 2.1.3 Defects | SPA, NUM | E4 **"Defect Hunt"** on tire, wheel, brake, steering, suspension and exhaust close-ups; **tread-gauge slider** (4/32 front, 2/32 others); **steering-play protractor** (10° ≈ 2 in on a 20-in wheel); leaf-count ¼ calculator; **"Loadout"** of the 3 required kinds (chains are a decoy) |
| 2.1.4 Inspection test | SEQ | "Point · Name · Explain" typed or voice rubric |
| 2.1.5 7-step method | SEQ, SPA | E5 order the steps with an engine On/Off toggle per step; **walk-path tracer** (left front → front → right side → right rear → rear → left side); dashboard sim (oil normal in seconds, air 50→90 in 3 min fast-forward, ABS light on→off); **light-color painter** (amber front, red rear); hydraulic test "pump 3 / hold 5" rhythm; key-in-pocket check |
| 2.1.6 Cargo checks en route | NUM | E7 speed slider picks whichever limit comes first (150 mi vs 3 h) after the 50-mile check |
| 2.1.7 After-trip report | NUM | Card: copy stays 1 day |

### GK-05

| Sub-topic | Type | Mechanic |
|---|---|---|
| 2.2 Four skills | DEF | Card |
| 2.2.1 Accelerating | SEQ, CAU | **"Hill Start"** pedal-timing sim with a rollback meter; wheel-spin → lift throttle |
| 2.2.2 Steering / 2.2.3 Stopping | CAU, SEQ | Pothole wheel-jerk demo; tach needle "clutch in near idle" |
| 2.2.4 Six backing rules | SEQ, SPA | E5 order the rules; **"Dock Planner"** where the player chooses the around-the-block route for a driver-side back; helper-placement drag |
| 2.2.5 Trailer backing | SPA | E6 **"Reverse-Steer"** kinematic mini-game: steer opposite, correct drift toward it, pull-ups budget |
| 2.3 / 2.3.1 Shifting, double clutch | SEQ | **"Double-Clutch Rhythm":** 5-step pedal and lever sequence with rpm matching (upshift lets rpm fall, downshift blips); progressive-shift chart; "never force it" recovery |
| 2.3.2 Multi-speed axles / 2.3.3 Automatic | DEF, DEC | Card; "downgrade → low range" choice |
| 2.3.4 Retarders | CAU, DEC | **"Retarder Switch":** road-surface cards (dry / wet / ice / snow) → on or off; highlight that it brakes the drive wheels only; match the 4 types |

### GK-08

| Sub-topic | Type | Mechanic |
|---|---|---|
| 2.7 / 2.7.1 Space ahead | NUM | **"Following-Distance Slider"** (length × speed → seconds, +1 over 40 once); **landmark count** timing game (tap as the lead car and then your bumper pass a shadow) |
| 2.7.2 Space behind | DEC | **"Tailgater"** response picker: 4 correct moves vs "tricks" |
| 2.7.3 Sides / wind | SPA | Top-down "escape the pack" positioning; gust event at a tunnel exit |
| 2.7.4 Overhead | CAU, SPA | **"Clearance Check":** empty vs loaded ride height vs a repaved or snowy bridge; tilted-road lean |
| 2.7.5 Below | SPA | Side-profile hang-up on drainage channels and rail humps |
| 2.7.6 Turns | SPA | E6 **Button hook vs jug handle** with a swept-path overlay and a car sneaking up on the right; pick the lane when there are 2 left-turn lanes; start the left turn at the center |
| 2.7.7 Entering traffic | NUM, DEC | Gap-acceptance game; a loaded truck accelerates more slowly |

### CV-01

| Sub-topic | Type | Mechanic |
|---|---|---|
| 6.1.1 Rollover | CAU | **"CG Stacker":** stack cargo high or low, centered or off-center, then set curve speed and watch for a tip; 10× loaded badge |
| 6.1.2 Rearward amplification | CAU, CMP | E6 **"Crack-the-Whip"** lane-change sim on rigs rated 1.0 / 2.0 / 3.5; predict which unit tips (the last one) |
| 6.1.3 Brake early | CMP | Stopping race: empty vs loaded vs bobtail (loaded wins) |
| 6.1.4 RR crossings | SPA, SEQ | Pick low-slung trailers from a side profile; E5 "stuck on tracks" order (leave → sign → 911 → DOT #) |
| 6.1.5 Trailer skid | CAU, DEC | **"Mirror Watch"** reaction game: brake hard, glance at the mirror, trailer swings, release brakes; using the hand valve = fail |
| 6.1.6 Off-tracking | SPA | E6 swept path with doubles; the last trailer's rear wheels cut in the most; button hook |
| 6.1.7 Backing | SPA | E6 Reverse-Steer framed as "top of wheel"; bonus for setting up a straight line |

## 4) Missing lessons (in START-HERE, not in the zip) `[F:START-HERE]`

The zip has 7 of the 18 lessons. Totals derived from START-HERE's pack totals minus what the 7 present lessons contain `[D]`:
- 207 of 338 practice questions are missing.
- 412 of 657 flashcards are missing.
- 69 of 98 TYK questions are missing.
- The readiness check implies GK-07 + GK-10 + GK-12 + GK-14 = 78 practice questions, and CV-02 + CV-03 + CV-04 = 52.

| Lesson | Pages | Weight / time | Must cover (START-HERE most-missed list; cross-refs from other lessons) |
|---|---|---|---|
| GK-06 Seeing, mirrors, signals, warning devices | 2-12–2-15 | High / 30 | #7 convex mirrors make things look smaller and farther away. #8 triangles within 10 min: divided highway 10, 100, 200 ft behind; two-lane road within 10 ft of the vehicle, 100 ft behind, 100 ft ahead; hill or curve 100–500 ft. [CA] two mirrors with a 200-ft rear view (p. 2-13, via GK-03). |
| GK-07 Speed and stopping distance | 2-15–2-18 | High / 45 | #1 an empty truck needs longer to stop. #2 at 55 mph = 419 ft (142 perception + 61 reaction + 216 braking). #3 double the speed → about 4× braking distance. [CA] service-brake stop from 20 mph (p. 2-16); passing on grades; 300-ft following rule (p. 2-18). |
| GK-09 Hazards, distracted and aggressive drivers | 2-21–2-28 | Med / 35 | No most-missed item listed |
| GK-10 Night, fog, winter, hot weather | 2-28–2-34 | High / 50 | #14 high beams whenever safe; dim within 500 ft. #15 never let air out of hot tires; open the radiator cap only when cool enough to touch bare-handed. [CA] when headlights must be on (p. 2-31). |
| GK-11 Railroad crossings and mountain driving | 2-35–2-38 | High / 35 | #11 pick the downhill gear before the grade, usually lower. #12 brake to 5 mph below the safe speed, release, repeat. #13 stop 15–50 ft from the rail, never shift on the tracks; a single track takes ≥14 s to clear, a double >15 s. Vehicles that must always stop (p. 2-35, via GK-02). |
| GK-12 Emergencies, ABS, skids | 2-38–2-43 | High / 45 | #16 blowout: hold the wheel, stay off the brake. #17 steer to the right; off the road, avoid braking until ~20 mph. #18 with ABS brake normally, no stab braking, not shorter stops. #19 drive-wheel skid: stop braking, countersteer. |
| GK-13 Accidents, fires, alcohol/drugs, hazmat basics | 2-44–2-49 | High / 45 | #20 protect the area first. #21 fire: no service station, hood shut, cargo doors shut, aim at the base; B:C = electrical and liquids. #22 .04 limit, any alcohol = 24 h, only time sobers. #30 no H endorsement needed only for unplacarded loads. Placard 9.84 in. |
| GK-14 Transporting cargo | 3-1–3-5 | High / 45 | #23 cargo checks at 50 mi, then every 3 h or 150 mi, and after every break. #24 tie-downs 1 per 10 ft, minimum 2. #25 bridge formula: closer axles → less weight. WLL ≥ ½ cargo weight. [CA] flags and lights on projecting loads (p. 3-5). |
| CV-02 Combination air brakes, trailer ABS | 6-5–6-8 | High / 40 | CV #5–13: hand valve only for testing; emergency line loses air → tractor protection valve closes; red octagon knob / yellow diamond; red = supply line, blue = service line; service line apart → nothing until you brake; crossed lines; shut-off valves closed only at the rear of the last trailer; dummy couplers; trailer ABS lamp yellow, left side, required from 3/1/1998. |
| CV-03 Coupling and uncoupling | 6-9–6-15 | High / 45 | CV #14–18: inspect the fifth wheel → chock → position the tractor; lock trailer brakes before backing under; tug test; trailer height raised slightly; no gap, jaws around the shank; landing gear fully up, handle secured; uncouple with the frame still under the trailer; fifth wheel tilted down toward the rear. |
| CV-04 Inspecting a combination | 6-16–6-17 | High / 30 | CV #11, 16, 19, 20: shut-off valves; coupling checks; tractor protection valve test (engine off, pump, pops at 20–45 psi); trailer emergency brake (pull knob, tug) and service brake (hand valve) tests. |

## 5) Handbook vs other sources — conflicts

Format: `id | topic | answer the test wants (handbook) | competing answer (source) | page | lesson | kind`. Kind is EXT (handbook vs outside source) or INT (handbook vs itself).

```
C01|OOS order 1st violation|>=90d; 2nd/10y >=1y; 3rd+/10y >=3y|180d/2y/3y (federal 49 CFR 383.51; old guide)|1-14; table p.1-19 gives 90d-1y non-HazMat but 180d-2y HazMat or 16+ passengers|GK-02,GK-03|EXT (180d is correct ONLY for HazMat/16+ pax)
C02|Dim high beams|within 500 ft of oncoming AND when following within 500 ft|300 ft following (CA car handbook; old guide)|page not given in file|GK-10 (missing)|EXT
C03|Hazmat placard size|>=9.84 in (250 mm) square, on point|10 3/4 in (older manuals; old guide)|page not given|GK-13 (missing)|EXT
C04|Tell employer of suspension|end of next business day (p.1-4) OR 2 business days (p.1-16); pick whichever is offered; never 30 d|30 d (that is for convictions)|1-4,1-16|GK-01,GK-03|INT
C05|Tie-down strength|total WLL >= 1/2 cargo weight|1.5x|page not given|GK-14 (missing)|EXT
C06|Passenger CDL [CA]|more than 10 incl. driver|16+ (federal; old guide)|1-1|GK-01|EXT
C06b|P endorsement wording|"10 or more" (p.1-5) vs "more than 10" (p.1-1); match the question's wording|-|1-1,1-5|GK-01|INT
C07|Fifth wheel before coupling|tilted down toward rear of tractor|"level"|page not given|CV-03 (missing)|EXT
C08|Tractor protection valve / red knob pops|20-45 psi|25-40 psi|page not given|CV-02,CV-04 (missing)|EXT
C09|Trailer hand valve|GK test: may hold rig from rolling back (p.2-9); CV test: only for testing trailer brakes; never park, never drive, never straighten a jackknife (6.2, p.6-5)|context-dependent|2-9,6-5|GK-05,CV-01,CV-02|INT (answer depends on which test)
C10|3-axle vehicle >6,000 lb [CA]|needs CDL (Class B)|missing from old guide|1-1|GK-01|EXT
C11|Loaded vs empty rollover|10x|5x (some practice websites)|6-1|CV-01|EXT
C12|Straighten a jackknife with the hand valve|never; release the brakes|some experienced drivers do it|6-3|CV-01|EXT
C13|Air pressure build-up|section 2.1 answer: 50->90 psi within 3 min|dual system ~85-100 psi in 45 s (p.5-9)|2-5,5-9|GK-04|INT
C14|Single-vehicle axle limits|give the handbook's number if the wording is exact; remember the pair 20,000/10,500|CVC 35550 puts 20,000 on the whole axle, 10,500 on one end; handbook sentences look swapped|1-23|GK-03|INT/EXT wording
C15|Federal hours of service|use handbook table: 11h / 14th hour / 60-7 or 70-8 / 34h restart|federal rules updated since (sleeper split, rest break)|1-27|GK-03|EXT (off-test)
C16|Inspection report copy|1 day in vehicle|federal: carrier keeps original 3 months|2-9|GK-04|EXT (off-test)
C17|"6 fuses" in the handbook's list|means 6 fusees (road flares)|electrical fuses|2-4|GK-04|errata
C18|Knowledge-test languages|English, Arabic, Chinese, Punjabi, Russian, Spanish|federal English-proficiency tightening since 2025; GK-01 says "2026 federal officials announced English-only CDL testing"|1-10|GK-01,START-HERE|EXT [UNKNOWN current status; not checked externally]
C19|CARB rules and deadlines|as printed|change often (arb.ca.gov)|1-20|GK-03|EXT (off-test)
C20|Following distance|seconds rule (1 s per 10 ft, +1 s over 40 mph)|[CA] 300-ft rule (p.2-18) is a separate law, not a replacement|2-19,2-18|GK-08,GK-07|confusion pair
```

**Other items the item generator should use** `[F:START-HERE]`:
- Distractors must be real values taken from the same source ("neighbouring numbers or the common mistake named in Exam traps"), never invented. I suggest turning this rule into the app's question-generator constraint.
- START-HERE also cites a DMV study figure: 39.5% fail GK and 43.9% fail CV on the first try. I did not verify it.
