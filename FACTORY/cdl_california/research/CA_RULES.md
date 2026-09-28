# California CDL rules, tests, 2025-26 changes, legal posture

_Research sweep 2026-09-27 (read-only agents; web access was search-index only — dmv.ca.gov, youtube.com, app stores blocked from the build container). Tags: [FACT src] / [ESTIMATE basis] / [UNKNOWN]. Source agent: `ca-rules`. Kept verbatim for provenance; corrections from the plan review are in `../BUILD_PLAN.md` and `REVIEW_FINDINGS.md`._

# CA CDL research brief for workshop planning (as of 2026-09-27)

**Access caveats. Read these first.**
- WebFetch returned `EGRESS_BLOCKED` on every domain I tried: dmv.ca.gov, ecfr.gov, fmcsa.dot.gov, law.cornell.edu, en.wikipedia.org, chp.ca.gov, startcdl.com and cdlpassmaster.com. I did not check the proxy-status endpoint because that call was denied.
- Every "FACT" below therefore comes from WebSearch result summaries, not from reading the page. Tags used:
  - `[FACT-s src]` = search summary of the named official page
  - `[FACT-3p src]` = third-party site
  - `[ARCH file]` = the user's zip lessons, which cite DL 650 page numbers
  - `[ESTIMATE basis]` and `[UNKNOWN]` as defined in the task
- Anything marked "volatile" should be re-verified against the live DMV pages before shipping.

**Archive inventory problem (affects planning).**
- `00-START-HERE.md` describes 18 lessons: GK-01…GK-14 and CV-01…CV-04.
- The zip contains only 7 of them: GK-01, 02, 03, 04, 05, 08 and CV-01.
- It also contains an older 10-part guide (`part-01…part-10`). START-HERE itself says that guide follows federal rules in places, which conflicts with the handbook.
- Missing lessons: GK-06, 07, 09–14 and CV-02, 03, 04. These cover handbook pages 2-12–2-18, 2-21–2-49, 3-1–3-5 and 6-5–6-17.

---

## 1. Class definitions (CA)

| Class | Definition | Src |
|---|---|---|
| Commercial A | Any legal combination with GCWR ≥26,001 lb where the towed unit(s) GVWR is >10,000 lb | [FACT-s dmv.ca.gov/portal/es/handbook/commercial-driver-handbook/section-1-introduction/] [ARCH GK-01 p.1-1] |
| Commercial B | Single vehicle GVWR >26,000 lb; or such a vehicle towing ≤10,000 lb GVWR; **or [CA] a 3-axle vehicle >6,000 lb** | same |
| Commercial C | Any Class C vehicle with one or more of these endorsements: HazMat (H), Passenger (P), Tank (N) | [FACT-s dmv.ca.gov/.../commercial-driver-license-classes-certifications/] |
| Class A can also drive B and C vehicles | | [ESTIMATE: standard hierarchy; not confirmed in any snippet] |

**CA-specific rules**
- **Passengers:** a CDL is needed above 10 people *including the driver* [ARCH GK-01 p.1-1]. Federal rule is 16 or more.
  - The handbook's P-endorsement paragraph says "10 or more". The handbook contradicts itself here; flag it.
  - A "passenger transportation vehicle" (>10 incl. driver) needs at least a Class C CDL with P. CA began issuing P and N on Class C CDLs on 2017-08-01 [FACT-s chp.ca.gov IB Class C CDL PDF, via dmv search].
- **Class B cases (Figure 1.1)** [ARCH GK-01 p.1-8]:
  - farm labor vehicle for ≥10 people incl. driver
  - vehicle for >10 incl. driver used for pay, profit or by a nonprofit
- **Horse trailers:** a horse trailer with living quarters and GVWR >10,000 lb needs Commercial A. If GCWR is <26,001, DMV adds **Restriction 88**: combinations with GCWR <26,001 whose towed unit is >10,000 [FACT-s dmv.ca.gov section-1 + bchcsjsu.org CHP PDF] [ARCH GK-01 pp.1-1–1-2].
- **Passenger-van decision chart** (vanpool exempt; >15 → CDL; >10 → CDL only if for pay or a nonprofit) [ARCH GK-01 p.1-1].
- **Restriction 41** is noncommercial: Class C towing a fifth-wheel trailer of 10,001–15,000 lb GVWR [ESTIMATE: forum sources ford-trucks.com, forestriverforums.com]. Not a CDL code; don't teach it as one.
- **Triples are illegal in CA** even with the T endorsement [FACT-s dmv.ca.gov section-7 snippet] [ARCH GK-01 p.1-5].
- **No CDL needed** for: vanpool drivers; non-commercial fifth-wheel travel trailers >15,000 lb or trailer coach >10,000 lb (these need noncommercial Class A); motorhomes 40–45 ft (need an endorsement); military; at a peace officer's direction [ARCH GK-01 p.1-7].
- **Minimum age:** 18 for intrastate. 21 for interstate or HazMat [FACT-s dmv.ca.gov truck-drivers/commercial-driver-information]. A CA driver license is required before a CLP [FACT-3p multiple].

## 2. Tests, endorsements, restrictions, skills

### 2a. Knowledge tests
All pass marks are 80%. Answer format is 3 choices, on the Automated Knowledge Testing Equipment (touchscreen).

| Test | Qs / pass | Handbook section(s) | Src |
|---|---|---|---|
| General Knowledge (everyone) | 50 / 40 | 1, 2, 3 | [FACT-3p driving-tests.org, dmv-written-test.com] [ARCH START-HERE] |
| Air Brakes (incl. air-over-hydraulic) | 25 / 20 | 5 | [FACT-3p driving-tests.org/california/cdl] [ARCH part-10] |
| Combination (Class A) | 20 / 16 | 6 | same |
| Doubles/Triples (T) | 20 / 16 | 7 | same |
| Tank (N) | 20 / 16 | 8 (Fig 1.2 also lists 9; add 6 for Class B/C) | same |
| HazMat (H) | 30 / 24 | 9, plus TSA check, age 21 | [FACT-3p epermittest, driving-tests.org] [ARCH part-09] |
| Passenger (P) | 20 / 16 | 4 | [ARCH part-10] [FACT-3p] |
| School Bus (S) | 20 / 16 | 10, plus CHP school-bus certificate | same |
| X endorsement | Tank + HazMat, both passed | — | [ARCH GK-01 p.1-5] |
| Firefighter (F) [CA] | knowledge test; optional for A/B | ≥14 h hands-on training incl. supervised behind-the-wheel (BTW); needed for Class C drivers of fire equipment | [FACT-s dmv.ca.gov firefighter-endorsement-training-requirements] |

**Study mapping (Figure 1.2)** [ARCH GK-01 p.1-12]:
- Class A: Sections 1, 2, 3, 6, 11–13, plus 5 if the vehicle has air brakes.
- Class B/C: the same without 6.
- Endorsements add: P→4, T→7, N→8 (and 9; plus 6 for B/C), H→9, S→10.

**Attempts and retakes**
- 3 attempts per test per application. Application fee is valid 12 months [ARCH GK-01 p.1-4].
- Retake wait after a failed knowledge test: [UNKNOWN]. The lesson (citing the handbook) says there is no wait. One third-party site claims a 7-day wait (epermittest snippet). Conflict.
- Knowledge tests are waived when adding an endorsement within 12 months of the last renewal or original issue [ARCH p.1-10].

**Other CA certificates (CHP or DMV issued)** [ARCH GK-01 pp.1-7–1-9] [FACT-s dmv.ca.gov driver-license-certificates-and-endorsements]:
- Ambulance (DMV DL 61; medical exam report + medical certificate every 2 years)
- HAM (hazardous agricultural materials)
- VTT (transit training)
- GPPV (paratransit)
- School Bus (medical every year at age 65+)
- SPAB (school pupil activity bus)
- Farm Labor Vehicle
- Youth Bus
- Tow Truck (freeway service patrol)
- VDDP (developmentally disabled persons)

I found no "tour bus" certificate [UNKNOWN].

### 2b. Restrictions
Source: [ARCH GK-01 pp.1-6–1-7, 1-11]; the E/K/L/O meanings also match a third-party list.

| Code | Meaning |
|---|---|
| E | No manual transmission (tested in an automatic) |
| K (CA shows 40/K) | Intrastate only; also used for 18–20-year-olds |
| L | No air brakes (skipped/failed Air Brakes, or tested without air brakes) |
| Z | No full air brakes (tested with air-over-hydraulic) |
| O | No tractor-trailer (Class A tested with a pintle hook or non-fifth-wheel) |
| M | No Class A passenger vehicle |
| N | No Class A or B passenger vehicle |
| P | No passengers (CLP with P or S) |
| X | No cargo in a tank (CLP with N) |
| V | Medical variance |
| 88 [CA] | See section 1 |

- Numeric CA codes such as 46 (automatic) and 50 (lenses) appear on one school blog only [UNKNOWN; verify].
- Design note: the same letter can mean different things as an endorsement vs. a restriction (N, P, X). This is a known trap and worth a dedicated drill.

### 2c. CLP
- Valid 180 days. One renewal of 180 days, as long as the end date is no later than 1 year from the application [FACT-3p dmvquestionbank, getlicensemap; consistent across sources].
- Wait ≥14 days after the CLP is first issued before the skills test. Only N, P and S can go on a CLP. A CDL holder must ride along. Skills parts you pass count only during the first 180 days [ARCH GK-01 p.1-2].

### 2d. Skills test
All of it is in English with no interpreter [ARCH p.1-11]. Federal rule 49 CFR 383.133 bans interpreters on skills tests [FACT-3p tatransinc/startcdl quoting the CFR].

- **Vehicle Inspection** (Section 11, about 40 minutes)
  - Failing it postpones the other parts; there is a free retake on the same application [ARCH].
  - The only aid allowed is the Section 11 memory aid [FACT-s dmv.ca.gov sec11].
  - Hydraulic brake check: pump 3 times, hold 5 seconds; the pedal must not sink [FACT-s sec11].
- **Basic Control Skills** (Section 12, about 30 minutes)
  - Exercises are a subset of: straight-line backing, offset back left/right, parallel park (driver side and conventional), alley dock.
  - Scored on encroachments, outside looks and pull-ups. The first pull-ups are free; too many count as errors.
  - When getting out for a look: neutral, parking brake set, 3 points of contact [FACT-s dmv.ca.gov/portal/es/handbook/commercial-driver-handbook/section-12-basic-control-skills-test/].
  - Failing above 12 points [ESTIMATE: third-party/federal model; not confirmed for CA].
- **Road Test / DPE** (Section 13, 45–60 minutes)
  - Pass = no more than 30 errors and no critical driving error [FACT-s dmv.ca.gov section-13-road-test].
  - Coasting out of gear for more than one vehicle length counts as unsafe coasting.
  - Never shift, pass or change lanes on railroad tracks. Buses and placarded vehicles stop 15–50 ft from the nearest rail [FACT-s same].
- **Retests:** a fee for basic control and road retests. The road test is required for an original CDL, removing a restriction, adding P or S, and renewing a CDL expired more than 2 years. Troops-to-Trucks waiver (DL 963/964) does not apply to S or P [ARCH p.1-11].

## 3. DL 650 Commercial Driver Handbook

**Edition** [UNKNOWN]. Third-party sites say it is "updated roughly yearly". The user's lessons reference a fee table from 2020. PDF copies exist at:
- `dmv.ca.gov/web/eng_pdf/comlhdbk.pdf`
- `dmv.ca.gov/portal/uploads/2020/06/comlhdbk.pdf`
- startcdl.com (2024-04 upload)

**The task's URL slug is wrong.** Correct: `https://www.dmv.ca.gov/portal/file/california-commercial-driver-handbook-pdf/` ("driver", not "drivers") [FACT-s].

**HTML version exists, one page per section**, at `dmv.ca.gov/portal/handbook/commercial-driver-handbook/section-N-<slug>/`. There is a Spanish mirror at `/portal/es/...` and a glossary at `/glossary/`. None of it was fetchable here.

| # | Title | User page range [ARCH] |
|---|---|---|
| 1 | Introduction (classes, tests, medical, disqualifications, CA laws) | 1-1–1-30 |
| 2 | Driving Safely (2.8 Seeing Hazards, 2.9 Distracted Driving, 2.10 Aggressive Drivers/Road Rage, 2.11 Night [FACT-s]; the rest follow the federal model [ESTIMATE]) | 2-1–2-49 |
| 3 | Transporting Cargo Safely | 3-1–3-5 |
| 4 | Transporting Passengers Safely | — |
| 5 | Air Brakes | — |
| 6 | Combination Vehicles (6.1 Driving Safely, 6.2 Air Brakes, 6.3 ABS, 6.4 Coupling/Uncoupling, 6.5 Inspecting) [FACT-s]. There is no 6.6 [ESTIMATE]. | 6-1–6-17 |
| 7 | Doubles and Triples | — |
| 8 | Tank Vehicles | — |
| 9 | Hazardous Materials | — |
| 10 | School Buses | — |
| 11 | Vehicle Inspection Test | — |
| 12 | Basic Control Skills Test | — |
| 13 | Road Test | — |

Page counts for sections 4–13 [UNKNOWN]. A legacy URL says "Section 2 of 15", so an older HTML edition likely had 15 parts including a glossary.

**Sample facts from sections not in the current lessons** (search summaries of dmv.ca.gov section pages)
- **Sec 3:** overloading hurts steering, braking and speed control and can cause brake failure; a high center of gravity means higher rollover risk.
- **Sec 4:**
  - A 2-inch standee line is required; no rider may stand ahead of the rear of the driver's seat.
  - Don't let riders carry car batteries or gasoline.
  - More than 100 lb of solid Class 6 poisons is banned on a bus.
  - No carry-on baggage in the aisle or doorway.
- **Sec 5:** the test is required for any air brakes, including air-over-hydraulic. The user has already passed Air Brakes; `part-05` covers it.
- **Sec 7:**
  - Put the heaviest trailer first, behind the tractor.
  - Rearward amplification: the last trailer of a triple is about 3.5× more likely to roll over than a 5-axle tractor-trailer.
  - Converter dolly: pintle-eye seated, pintle hook latched, safety chains secured.
  - "Pull doubles" means a tug test against the pin of the second trailer.
- **Sec 8:**
  - N endorsement applies to any tank >119 gal *and* ≥1,000 gal total.
  - Baffles control front-to-back surge only; side-to-side surge can still roll the tank.
  - Smooth-bore (unbaffled) tanks carry food such as milk, because sanitation rules forbid baffles.
  - Outage = never load a tank completely full. Start and stop very smoothly.
- **Sec 9:**
  - TSA security threat assessment and age 21 required.
  - Hazmat test content: CFR hazmat rules, placards, shipping papers, emergency procedures, CA route and permit rules, CHP and DTSC.
  - **Placard conflict:** the "DANGEROUS" placard single-class limit is 2,205 lb (1,000 kg) in 49 CFR 172.504(b), but older handbook editions print 5,000 lb [ARCH part-09]. Under the "handbook way" rule, the answer key must match the edition in use [UNKNOWN which].
- **Sec 10:**
  - Danger zone reaches up to 30 ft in front (the first 10 ft are the worst) and 10 ft on the sides and rear; the left side is always dangerous.
  - Crossview mirrors cover the front bumper and the side danger zones.
  - The rear blind spot can reach 400 ft or more.
- **Sec 11–13:** see 2d.

## 4. Changes in 2025–2026 (volatile; show the as-of date in the app)

**English-language proficiency enforcement**
- Executive Order 14286 (April 2025).
- CVSA made ELP failure an out-of-service violation from 2025-06-25 [FACT-3p cvsa.org/news/elp-oosc-06252025].
- FMCSA revised its guidance 2026-04-16 [FACT-3p ecfr snippets/tatransinc].
- NPRM on 2026-08-10 to write that out-of-service rule into the regulations; 60-day comment period [FACT-3p federalregister.gov/documents/2026/08/10/2026-16288, cdllife].
- Dalilah's Law (would require English-only tests) passed House T&I 35–26 on 2026-03-18; not enacted as far as I found [FACT-3p truckingdive, trucksafe].

**English-only testing**
- Secretary Duffy announced rulemaking for English-only knowledge and skills tests on 2026-02-20 [FACT-3p foxbusiness, pbs.org, cdllife]. He said CA offered about 20 languages.
- Whether a final federal rule on *knowledge* tests is in force is [UNKNOWN]. Sources conflict: effective 2026-04-17, 2027-01-01, or not finalized.
- Florida went English-only 2026-02-06 and Texas 2026-06-01 [FACT-3p dps.texas.gov].
- CA's current CDL knowledge-test languages [UNKNOWN]:
  - Handbook: English, Arabic, Chinese, Punjabi, Russian, Spanish [ARCH p.1-10].
  - Third-party claims range from 7 to 35 languages.
  - The 2021 plan to cut to 7 languages applied to Class C tests [FACT-3p governing.com].
- **Design implication:** multilingual *teaching* with an English term alongside every translated term, and an English-mode mock test by default.

**Non-domiciled CDLs**
- Interim final rule 2025-09-29, later stayed by the D.C. Circuit.
- Final Rule published 2026-02-13, effective 2026-03-16. Eligibility is limited to H-2A, H-2B and E-2 visa holders; further court petitions are pending [FACT-s federalregister.gov 2026-02965, beneschlaw].
- CA cancelled about 13,000 non-domiciled CDLs on 2026-03-06. FMCSA plans to withhold about $158M starting FY2027 (final determination 2026-01-07).
- A CA court allowed re-application, but CA DMV says it is not currently issuing non-domiciled CLPs/CDLs [FACT-3p truckingdive, seyfarth; dmv.ca.gov news page].
- **Design implication:** an eligibility pre-check screen that links to official sources, with no legal advice.

**ELDT (entry-level driver training)**
- Required since 2022-02-07 for: a first Class A or B CDL, a B→A upgrade, and a first S, P or H endorsement.
- Theory and BTW must come from a provider on the Training Provider Registry (tpr.fmcsa.dot.gov) *before* the skills test, or before the H knowledge test.
- Theory assessment needs ≥80%. There are no federal minimum hours; BTW is proficiency-based [FACT-s fmcsa ELDT page, 49 CFR 380.715 snippets].
- CA lets you book a skills-test appointment before ELDT, but ELDT must be finished by the test date [FACT-s dmv.ca.gov CDL page].
- **CA extra:** at least 15 h BTW, at least 10 on public roads, with 50 minutes counting as an hour. Submitted on form DL 1236 via the DMV Virtual Field Office before a Class A/B CDL is issued. Legal basis: CVC §15250.1, SB 1236 (2018), 13 CCR §26.03 [FACT-s dmv.ca.gov/portal/dmv-virtual-office/submit-dl-1236-vfo/].
- 2025 crackdown: about 3,000 inactive providers removed, more than 110 emergency removals, about 4,000 put on notice [FACT-3p jjkeller, aamva].

**Medical**
- National Registry II took effect federally 2025-06-23: examiners report results electronically and the paper certificate goes away, with temporary waivers (60-day paper-copy grace, most recently through 2026-04-10) [FACT-s nationalregistry.fmcsa.dot.gov].
- CA DMV implementation date shows as "July 27, 2026" in a dmv.ca.gov snippet [UNKNOWN; the year may be wrong].
- Medical exam report and certificate dated within 2 years. CA has no medical variances. Intrastate restricted certificate DL 51B. Self-certification form DL 694: CA issues only NI or NA (NA → 40/K) [ARCH] [FACT-3p].

**Drug & Alcohol Clearinghouse II**
- Since 2024-11-18, the DMV must check the Clearinghouse before issuing, renewing, upgrading or transferring a CLP or CDL.
- A "prohibited" status leads to a downgrade until the driver completes return-to-duty [FACT-s clearinghouse.fmcsa.dot.gov].

**Appointments and fees**
- Commercial drive-test booking online: `dmv.ca.gov/wasapp/foa/startCommDriveTest.do` [FACT-s]; phone 1-800-777-0133.
- One third-party site claims CDL road tests can't be booked online. That conflicts with the booking page above.
- Waits of 6–10 weeks in metro areas [ESTIMATE: third-party].
- Fees:
  - $100 original Class A/B, $46 skills retest [FACT-3p driving-tests.org, getlicensemap; one other third-party source says $85].
  - The handbook's 2020 table says $82.
  - **Don't hardcode fees; link to** `dmv.ca.gov/portal/driver-licenses-identification-cards/licensing-fees/`.

**Official practice material:** DMV "Sample Commercial Drivers Written Test 1 and 2" at `dmv.ca.gov/portal/driver-education-and-safety/educational-materials/sample-driver-license-dl-knowledge-tests/sample-commercial-drivers-written-test-1/` (and `-2/`) [FACT-s].

**First-attempt fail rates** (39.5% GK, 43.9% Combination) [ARCH START-HERE]. Likely from DMV report RSS-08-225 (2008) [UNKNOWN; verify before displaying].

**YouTube**
- I found no official CA DMV or FMCSA CDL instructional video IDs. FMCSA's "Our Roads, Our Safety" PSAs are general road-safety spots, not CDL instruction [FACT-s fmcsa.dot.gov/ourroads].
- Video IDs [UNKNOWN]. Use human-curated link slots with a "verified on" date. Do not generate IDs.

## 5. Legal and disclaimer implications
1. **Not an ELDT provider.** Only TPR-listed providers can certify theory or BTW training [FACT-s TPR]. The app must never claim ELDT credit, never imply it replaces DL 1236 BTW hours, and should link to tpr.fmcsa.dot.gov.
2. **No DMV branding.** AB 1272 (2025, Ch. 68) extends Vehicle Code §25: from 2026-01-01, occupational licensees may not use "DMV" or "Department of Motor Vehicles" in business names, phone numbers or **domain names**. Violations are misdemeanors [FACT-s dmv.ca.gov 26olin03]. Whether this binds a non-licensee is [UNKNOWN], but avoid "DMV" in the product name and domain anyway, and add a "Not affiliated with CA DMV, CHP or FMCSA" disclaimer.
3. **Copyright.** The CA *Driver's* Handbook (DL 600) is licensed CC BY-NC 4.0, and commercial use needs written permission [FACT-s dmv.ca.gov/portal/handbook/california-driver-handbook/copyright/]. The license for DL 650 is [UNKNOWN]. Safer approach: paraphrase, cite page numbers, and link to the official PDF rather than embedding verbatim text, especially if the app is monetized. DMV forms need written permission to reproduce [FACT-s dmv.ca.gov 29.015].
4. **Accuracy.** Show "Rules change; confirm with DMV" on every rule that could change (languages, fees, ELDT, non-domiciled, ELP, medical), each with a source URL and last-verified date. No legal or immigration advice. Where the handbook and federal rules differ, show both and label which one is "the test answer".
5. **Privacy and accessibility.** Learner-data handling (CCPA if it reaches that scale) and WCAG compliance [ESTIMATE: general best practice; not researched here].

## 6. Handbook-vs-other conflicts the answer-key logic must handle

| Topic | Handbook | Other | Src |
|---|---|---|---|
| Passenger CDL threshold | >10 incl. driver (p.1-1); P endorsement text says "10 or more" (p.1-5) | Federal: 16+ | [ARCH] |
| Out-of-service order, 1st violation | 90 days | Federal: 180 days | [ARCH] |
| Dim high beams when following | 500 ft | Car handbook: 300 ft | [ARCH] |
| Placard size | 9.84 in | Older manuals: 10¾ in | [ARCH] |
| DANGEROUS placard one-class limit | edition-dependent, 5,000 lb? | CFR: 2,205 lb | [ARCH part-09] |
| Tell employer of suspension | next business day (p.1-4) | 2 business days (p.1-16) — both in handbook | [ARCH] |
| Tie-down strength | ≥½ cargo weight | — | [ARCH] |
| Tractor protection valve pops | 20–45 psi | — | [ARCH] |
| Trailer hand valve | GK section: may hold against rollback | Combination section: testing only | [ARCH] |
| Knowledge-test languages | handbook list | federal English-only push | see §4 |

The **`part-01…07` files must not feed the answer key** where they follow federal rules. START-HERE itself tells the user to drop parts 01–04, 06 and 07.
