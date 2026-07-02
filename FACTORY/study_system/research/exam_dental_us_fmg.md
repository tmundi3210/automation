# US Dental Licensure for Foreign-Trained Graduates — Exam Ground Truth

Scope: identify and fully specify the exam(s) a foreign-trained (non-CODA) dental
graduate must pass for US dental licensure. Primary knowledge exam is the **INBDE**.
This file is ground truth for downstream dental content KBs.

Research constraint (read first): During compilation, direct fetches of the official
JCNDE/ADA PDFs (Candidate Guide, Item Development Guide, Domain of Dentistry Model)
and several university mirrors returned **HTTP 403 under the session egress policy**
(WebFetch and curl both blocked for `jcnde.ada.org`, `dental.ufl.edu`, `ada.org`,
`ece.org`, and general hosts). Facts below were reconstructed from search-engine
snippets of those official pages plus corroborating secondary aggregators. Every
claim that could NOT be confirmed against a directly-read official page is tagged
`[ESTIMATE]` or `[UNKNOWN]`. The official document URLs are cited so a downstream
run with fetch access can verify verbatim — this is the top-priority follow-up
(the full 56-item Clinical Content enumeration in particular).

---

## 1. Exam identity (pinned)

- **INBDE = Integrated National Board Dental Examination.** [FACT] It is the single
  written/computer-based national board exam required across US licensing
  jurisdictions. Multiple official-page snippets confirm it "was launched in August
  2020 and replaced the NBDE Part I and Part II."
  Official landing: https://jcnde.ada.org/inbde
- **Runner / owner:** the **Joint Commission on National Dental Examinations (JCNDE)**,
  an agency operating under the **American Dental Association (ADA)**. [FACT]
  https://jcnde.ada.org/inbde
- **Delivery vendor:** administered at **Prometric** test centers. [ESTIMATE — stated
  by multiple secondary guides; not confirmed against a directly-read official page.]
- The INBDE is a **knowledge exam only**. It does NOT satisfy the separate **clinical
  (hands-on) exam** requirement that most states impose — see §6 (ADEX/CDCA/DLOSCE). [FACT]

---

## 2. Licensure pathway for internationally trained dentists

To be licensed as a general dentist in the US, a foreign-trained (non-CODA/CDAC)
graduate generally must satisfy **three** independent gates. Requirements are set
per-jurisdiction (state board), so the exact combination varies by state. [FACT —
ADA: https://www.ada.org/resources/careers/licensure/licensure-for-the-international-dentists
and ADA licensure-by-state map: https://www.ada.org/resources/careers/licensure/dental-licensure-by-state-map]

### 2.1 Education gate — almost always a CODA "advanced standing" program
- Most US states require a **DDS or DMD from a CODA-accredited US dental school** for
  full licensure. A degree from a non-CODA foreign school is, by itself, not accepted
  in most states. [FACT — ADEA:
  https://www.adea.org/godental/Apply/admissions-process/nontraditional-applicants/foreign-educated-dentists]
- Foreign dentists close this gap via an **International Advanced Standing / Advanced
  Placement DDS/DMD program** (typically **~2–3 years**, entering with credit for prior
  training). [FACT] Examples of CODA advanced-standing programs: Columbia (~2.5 yr),
  NYU (28 months), Harvard DMD track, Pitt, UMKC, University of Colorado (2-yr DDS).
  [FACT — program pages, e.g. https://www.dental.columbia.edu/education/dds-program/advanced-standing-program-internationally-trained-dentists
  , https://www.hsdm.harvard.edu/advanced-standing-international-dentists-dmd-track]
- **Application service: ADEA CAAPID** — ADEA Centralized Application for Advanced
  Placement for International Dentists — a single application to multiple advanced-standing
  programs. Cycle runs **March through January**. [FACT — ADEA:
  https://www.adea.org/home/application-services/international-dentists ;
  program directory: https://programs.adea.org/CAAPID/]
- A minority of states offer alternative routes (e.g. licensure by credential/experience,
  or accepting a non-CODA degree if other conditions are met). State-by-state; treat as
  variable. [ESTIMATE — asserted by ADA state map; specifics not enumerated here.] [UNKNOWN:
  exact list of states allowing non-CODA degrees for full licensure — verify per state board.]

### 2.2 Credential evaluation — ECE (for exam eligibility)
- Non-CODA graduates/students must have their foreign dental credentials evaluated
  before JCNDE will grant INBDE eligibility. JCNDE designates **Educational Credential
  Evaluators (ECE)** for this. [FACT — ECE:
  https://www.ece.org/ECE/Credential-Evaluations/US-Institutions/ADA-JCNDE]
- Report types: a **U.S. General Report** typically suffices if applying only for the
  exam; a **Course-by-Course Report** is what most US dental schools (and accepted by
  ADA/JCNDE) require if also applying to advanced-standing programs. [FACT — ECE page above.]
- Sequence: obtain a **DENTPIN** (ADA's personal ID number) → ECE sends evaluation to
  JCNDE → JCNDE emails eligibility confirmation → candidate may schedule INBDE. [FACT/ESTIMATE
  — sequence from ECE + JCNDE snippets; DENTPIN requirement is [FACT].]
- Current non-CODA students (not yet graduated) must also submit a **Certification of
  Eligibility** signed by their dean/registrar; graduates do not. [ESTIMATE — from JCNDE/ECE
  snippet; confirm in current Candidate Guide.]

### 2.3 Examination gate — INBDE (knowledge) + a clinical exam
- **INBDE** (this document's core) — required by all US jurisdictions. [FACT]
- **A separate clinical/hands-on licensure exam** — required by most states; commonly the
  **ADEX** exam and/or the **DLOSCE**. See §6. [FACT]

---

## 3. INBDE — the knowledge exam in full

### 3.1 Who runs it / eligibility for foreign graduates
- Run by **JCNDE (ADA)**. [FACT]
- **Eligibility for foreign graduates:** established via ECE credential evaluation +
  DENTPIN (§2.2). Once a non-CODA graduate is confirmed eligible, they take the *same*
  INBDE as CODA candidates — there is no separate "foreign" version. [FACT — JCNDE/ECE snippets.]

### 3.2 Format, item counts, timing
[ESTIMATE — high confidence; consistent across multiple secondary guides quoting the
JCNDE Candidate Guide, but not confirmed against a directly-read official Candidate Guide.]

| Element | Value |
|---|---|
| Total items | **500** multiple-choice |
| Structure | Delivered over **2 days** (may be scheduled on non-consecutive days) |
| Day 1 | **~8 hours**, **360 items** (≈300 standalone/discipline-based + ≈60 case-based) |
| Day 2 | **~4 hours**, **140 items**, all **case-based** (patient case sets) |
| Case sets | Series of items about one patient; include charts — perio charts, odontograms, radiographs, clinical photos |
| Tutorials/breaks | Optional ~15-min tutorial at start of each day; optional survey at end of Day 2 |
| Scoring items | Some unscored pretest items are embedded (standard board practice) [ESTIMATE] |

Official format source to verify: 2026 INBDE Candidate Guide (PDF)
https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_candidate_guide.pdf

### 3.3 Official content blueprint — the "Domain of Dentistry"
The INBDE blueprint is a **matrix**: **10 Foundation Knowledge (FK) areas × 56 Clinical
Content (CC) areas**. Every item is written to integrate ≥1 FK area applied within a CC
area (this is what "Integrated" in INBDE means). [FACT — confirmed across official-page
snippets and JCNDE Item Development Guide references.]

Authoritative blueprint documents (verify verbatim — all currently 403 under egress policy):
- **Model Domain of Dentistry (INBDE):**
  https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_domain_dentistry_model.pdf
- **INBDE Item Development Guide** (defines FK + CC, with Figures 1–4):
  https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_item_development_guide.pdf
- **Item Development Guide — Appendices A–E:**
  https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde-item_development_guide_appendices_a-e.pdf
- **Domain of Dentistry (July 2018):**
  https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/domain_of_dentistry_july2018.pdf
- University mirror (Item Dev Guide): https://dental.ufl.edu/wordpress/files/2025/03/INBDE_Item_Development_Guide.pdf
- Penn Libraries prep guide (paraphrases official areas):
  https://guides.library.upenn.edu/INBDE/domain and .../INBDE/found

#### 3.3.1 Foundation Knowledge (FK) areas — all 10
[ESTIMATE — high confidence. Enumeration corroborated across multiple independent
secondary sources quoting the JCNDE Domain of Dentistry model (incl. Penn Libraries,
UOP Dugoni, dentalproguide). Wording is close to official; verify exact phrasing in
the Item Development Guide PDF. No official per-FK item weight is published as a fixed
percentage — items are distributed across FK areas but JCNDE does not publish a public
fixed FK weighting table.] [UNKNOWN: official FK-by-FK item percentages.]

| # | Foundation Knowledge area |
|---|---|
| FK1 | Molecular, biochemical, cellular, and systems-level development, structure, and function |
| FK2 | Physics and chemistry to explain normal biology and pathobiology |
| FK3 | Physics and chemistry to explain the characteristics and use of technologies and materials |
| FK4 | Principles of genetic, congenital, and developmental diseases and conditions and their clinical features to understand patient risk |
| FK5 | Cellular and molecular bases of immune and non-immune host defense mechanisms |
| FK6 | General and disease-specific pathology to assess patient risk |
| FK7 | The biology of microorganisms in physiology and pathology |
| FK8 | Pharmacology |
| FK9 | Behavioral sciences, ethics, and jurisprudence |
| FK10 | Research methodology and analysis, and informatics tools |

#### 3.3.2 Clinical Content (CC) areas — 56 total, in 3 groups
The 56 CC areas are grouped into **three component sections**. Group membership counts
and published item-percentage weights: [ESTIMATE — counts (13/23/20) and weights come
from official-page snippets + secondary aggregators; the two figures describe different
things (number of CC areas vs. share of scored items), which is why counts and percentages
do not line up.]

| Group | # of CC areas | Share of scored items |
|---|---|---|
| A. Diagnosis and Treatment Planning | 13 [ESTIMATE] | ~36.2% [ESTIMATE] |
| B. Oral Health Management | 23 [ESTIMATE] | ~42.0% [ESTIMATE] |
| C. Practice and Profession | 20 [ESTIMATE] | ~21.8% [ESTIMATE] |
| **Total** | **56** [FACT] | **100%** |

> COMPLETENESS GAP (flagged as the #1 follow-up): The **full verbatim enumeration of all
> 56 CC areas with official numbering** could not be extracted — the JCNDE PDFs present
> them as Figures 1–4 (images/tables) and every direct fetch returned 403. Below is a
> **partial, non-authoritative** reconstruction from search snippets. Treat item text as
> approximate and the lists as **incomplete**. Do NOT build a KB claiming this is the
> complete official 56 without verifying against the Item Development Guide PDF.

**A. Diagnosis and Treatment Planning (13 areas)** — [UNKNOWN: complete verbatim list].
Confirmed themes/partial items from official snippets [ESTIMATE]:
- Recognize the normal range of clinical findings and significant deviations that require monitoring, treatment, or management.
- Select, obtain, and interpret diagnostic images for the individual patient.
- Obtain and interpret the patient's history (medical, dental, social, behavioral) and chief complaint.
- Evaluate structure, appearance, and function of orofacial/oral tissues.
- Interpret laboratory and other diagnostic test results.
- Synthesize findings into an accurate diagnosis.
- Develop a comprehensive, evidence-based treatment plan including reasonable, safe alternatives.
- (Remaining ~6 areas not verified.) [UNKNOWN]

**B. Oral Health Management (23 areas)** — [UNKNOWN: complete verbatim list].
Confirmed themes/partial items from official snippets [ESTIMATE]:
- Manage the unique oral health care needs of **infants**.
- Manage the unique oral health care needs of **children**.
- Manage the unique oral health care needs of **adolescents**.
- Manage the oral health care of **adults**, including the unique needs of **women**.
- Manage the unique oral health care needs of **geriatric** patients.
- Manage the unique oral health care needs of **special-needs** patients.
- Select and administer or prescribe **pharmacological agents** in the treatment of dental patients.
- Anticipate, prevent, and manage **complications** from therapeutic/pharmacological agents.
- Prevent, recognize, and manage **medical and dental emergencies**.
- (Plus CC areas covering the operative/restorative, periodontal, endodontic, prosthodontic,
  oral surgery, orthodontic, and pain/anxiety-control management tasks. Individual verbatim
  items and count reconciliation not verified.) [UNKNOWN]

**C. Practice and Profession (20 areas)** — [UNKNOWN: complete verbatim list].
Confirmed themes/partial items from official snippets [ESTIMATE]:
- Practice within the general dentist's scope of competence; consult with or refer to colleagues when indicated.
- Evaluate and use available and emerging resources (lab, clinical, information technology) to support patient care, practice management, and professional development.
- Conduct practice activities that manage risk and are consistent with jurisprudence and ethical requirements.
- Recognize and respond to situations involving ethical and jurisprudence considerations.
- Maintain patient records per jurisprudence and ethical requirements.
- Conduct practice-related business/financial operations per sound business practices and jurisprudence (e.g., OSHA, HIPAA).
- Develop a catastrophe-preparedness plan for the dental practice.
- Manage, coordinate, and supervise the activity of allied dental health personnel.
- Assess one's own level of skills and knowledge relative to dental practice.
- Adhere to standard precautions for infection control for all clinical procedures.
- Use prevention, intervention, and patient-education strategies to maximize oral health.
- Collaborate with dental team members and other health professionals to promote health and manage community disease.
- Evaluate and implement systems of oral health care management and delivery for the populations served.
- Apply quality assurance, assessment, and improvement concepts to improve outcomes.
- Communicate case design to laboratory technicians and evaluate the resultant restoration or prosthesis.
- (Remaining areas not verified; note some prevention/community items may officially sit in group B — grouping unverified.) [UNKNOWN]

---

## 4. Scoring, pass rules, retake, cost

### 4.1 Scoring & pass standard
- **Scaled score range 49–99; passing standard = 75.** [FACT — JCNDE INBDE Scores page
  snippet: https://jcnde.ada.org/inbde/inbde-results]
- Result to candidate/board is reported **pass/fail** ("pass" only for passers); failing
  candidates receive **diagnostic performance feedback** (by FK and CC areas) for
  remediation. Passing candidates do NOT get a numeric score on the report. [FACT — same page.]
- The passing **standard was raised effective June 2024**, increasing the failure rate.
  [ESTIMATE — reported by secondary sources (e.g. cited ~16.1% failure rate); exact figure
  not confirmed against an official statistics release.] [UNKNOWN: official current pass-rate stats.]
- Standard-setting / technical detail: INBDE Technical Report:
  https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_technical_report.pdf

### 4.2 Retake policy
[ESTIMATE — from JCNDE Scores/Results page snippets; confirm current numbers in the
Candidate Guide, as waiting periods have changed historically.]
- Minimum **60 days** between unsuccessful attempts. (Some older/secondary sources say 90 days —
  treat as changed; verify.) [ESTIMATE/UNKNOWN]
- Must pass within **5 years of first attempt OR 5 attempts, whichever comes first.** [ESTIMATE]
- A candidate who has **passed** may not retake to improve a score. [ESTIMATE]

### 4.3 Cost
[ESTIMATE — 2026 figures from multiple secondary guides citing the Candidate Guide; not
confirmed against a directly-read official fee page.] [UNKNOWN: verify current exact fee.]
- INBDE exam fee: **~$890** (2026).
- Additional **non-CODA processing fee: ~$435** for internationally trained applicants.
- ECE credential evaluation: additional cost (report-type dependent), paid to ECE separately. [ESTIMATE]
- Fees stated as **non-refundable / non-transferable**. [ESTIMATE]
- Official application/fee page to verify: https://jcnde.ada.org/inbde/inbde-apply

---

## 5. Official prep materials published by the runner (JCNDE/ADA)

[FACT for existence — these are JCNDE-published resources; URLs may 403 under egress policy
but are the correct official artifacts.]
- **INBDE Candidate Guide (annual PDF)** — format, rules, scheduling, policies:
  https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_candidate_guide.pdf
  (prior-year guides also public: 2024/2025 versions under the same /files/ path.)
- **INBDE Item Development Guide** (+ Appendices A–E) — the blueprint/FK+CC definitions and
  sample item construction (URLs in §3.3).
- **Model Domain of Dentistry** PDF (URL in §3.3).
- **INBDE Technical Report** — psychometrics/standard setting (URL in §4.1).
- **JCNDE "INBDE facts" one-pagers** (for students and for state boards):
  https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_facts_students.pdf
  https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_facts_state_boards.pdf
- **JCNDE "Prepare for the INBDE" page:** https://jcnde.ada.org/inbde/inbde-prepare
- [UNKNOWN]: whether JCNDE currently publishes a free official *practice test / sample item
  set* separate from the Item Development Guide's sample items — not confirmed. Some university
  sites host "INBDE practice questions" PDFs (e.g. UOP Dugoni) but those are school-hosted,
  not necessarily JCNDE-official.

---

## 6. The separate CLINICAL exam (not INBDE) — required by most states

The INBDE is knowledge-only. For a license, most states additionally require a **clinical
(hands-on) examination**, administered by third-party regional testing agencies. [FACT —
ADEA / ADA licensure pages.]

- **ADEX Dental Examination** — the dominant clinical exam; developed by the **American
  Board of Dental Examiners (ABDE)** and administered by **CDCA-WREB-CITA** (the merged
  regional agency). Recognized/required in nearly every US jurisdiction. [FACT —
  https://adextesting.org/adex-dental/ ; https://adextesting.org/]
  - Components: hand-skill/restorative components + periodontal scaling + the ADA's
    **DLOSCE**. Increasingly delivered on **manikins / CompeDont typodont teeth** instead
    of live patients. [FACT — ADEX + Dental Board of California:
    https://www.dbc.ca.gov/applicants/licensure_by_adex.shtml]
  - **Foreign-trained access:** Only students/graduates of **CODA/CDAC-accredited** schools
    may take the ADEX directly. All others (international graduates) **must apply through the
    specific state dental board** for permission to sit a licensure exam in that jurisdiction.
    [FACT — ADEX/ABDE candidate materials.] This is a major reason foreign dentists route
    through a CODA advanced-standing program (§2.1).
- **DLOSCE** — Dental Licensure Objective Structured Clinical Examination — an ADA-developed,
  standardized, non-patient clinical-judgment exam; used as/within the clinical requirement
  in some states. [FACT — referenced by ADEX and state boards.]
- **CDCA / WREB / CITA** — the regional clinical-exam agencies (now merged) that administer
  ADEX; state boards choose which exams they accept. [FACT]
- **State variation is the rule, not the exception.** The exact clinical exam accepted, the
  manikin-vs-patient policy, PGY-1 residency alternatives, and jurisprudence exams differ by
  state. Use the ADA licensure-by-state map as the authority per state. [FACT —
  https://www.ada.org/resources/careers/licensure/dental-licensure-by-state-map]
- Many states also require a **state/regional jurisprudence exam** and background checks. [ESTIMATE]

---

## 7. Well-known public / free study resources (existence only — NO quality claims)

[FACT — for existence. Tagged for existence of the resource only; this file makes no claim
about accuracy, quality, or endorsement. Several are commercial with free tiers.]
- **JCNDE official guides** (free PDFs) — see §5. Primary source of truth.
- **Penn Libraries INBDE Prep Guide** (free library guide summarizing FK/CC areas):
  https://guides.library.upenn.edu/INBDE
- **dentalcare.com (Dentsply Sirona)** — free CE courses mapped to INBDE CC areas; the mapping
  guide exists: https://www.dentalcare.com
- **INBDE Bootcamp** — blog/free articles (commercial product, free content tier):
  https://bootcamp.com/blog (multiple INBDE explainer articles)
- **Kaplan INBDE** free explainer articles: https://www.kaptest.com/inbde/about-the-inbde
- **BoardVitals INBDE blog**: https://www.boardvitals.com/blog/things-to-know-about-the-inbde-dental-board-exam/
- **ADEA GoDental** (free applicant guidance for international dentists):
  https://www.adea.org/godental/
- **exam-review.com free INBDE practice test**: https://www.exam-review.com/inbde-practice-tests
- [UNKNOWN]: named free video series / open-courseware specifically branded for INBDE could
  not be verified as reputable/official; not listing unverified YouTube channels to avoid
  fabricating names. Follow-up: verify with fetch access before adding any.

---

## 8. Confidence summary & verification backlog

| Claim class | Status |
|---|---|
| INBDE is the required knowledge exam; run by JCNDE/ADA; replaced NBDE I&II (Aug 2020) | [FACT] |
| Foreign path = ECE eval + DENTPIN → INBDE; CODA advanced-standing via ADEA CAAPID; separate clinical exam | [FACT] |
| Blueprint = 10 FK × 56 CC matrix; 3 CC groups | [FACT] |
| FK1–FK10 exact wording | [ESTIMATE — high confidence; verify Item Dev Guide] |
| CC group counts 13 / 23 / 20 and weights 36.2 / 42.0 / 21.8% | [ESTIMATE] |
| **Full verbatim 56 CC enumeration with numbering** | **[UNKNOWN — TOP FOLLOW-UP]** |
| 500 items over 2 days (360 + 140) | [ESTIMATE — high confidence] |
| Pass = scaled 75 (range 49–99); pass/fail reporting + diagnostic feedback | [FACT] |
| Standard raised June 2024; current pass rates | [ESTIMATE / UNKNOWN] |
| Retake: 60-day gap; 5 attempts / 5 years | [ESTIMATE — verify current] |
| Fees ~$890 + ~$435 non-CODA (2026) | [ESTIMATE — verify] |
| ADEX clinical exam; CODA-only direct access; state variation | [FACT] |

**Priority verification backlog (needs a run with working WebFetch/HTTP to the JCNDE host):**
1. Extract all **56 CC areas verbatim with official numbering** from the Item Development
   Guide / Domain of Dentistry Model PDFs (§3.3 URLs). This is the highest-value gap.
2. Confirm **exact FK wording** and any published FK/CC item-weight tables.
3. Confirm current **fee**, **retake interval**, and **attempt/time limits** from the live
   Candidate Guide and Apply page.
4. Confirm whether a **free official JCNDE practice test** exists separate from the guide.

---

## Sources

Official (JCNDE / ADA / ADEA / ECE / ABDE) — canonical, several 403 under this session's egress policy:
- INBDE landing — https://jcnde.ada.org/inbde
- INBDE Apply — https://jcnde.ada.org/inbde/inbde-apply
- INBDE Prepare — https://jcnde.ada.org/inbde/inbde-prepare
- INBDE Scores/Results — https://jcnde.ada.org/inbde/inbde-results
- INBDE History — https://jcnde.ada.org/inbde/inbde-history
- INBDE Candidate Guide (2026 PDF) — https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_candidate_guide.pdf
- INBDE Item Development Guide (PDF) — https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_item_development_guide.pdf
- Item Dev Guide Appendices A–E — https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde-item_development_guide_appendices_a-e.pdf
- Model Domain of Dentistry — https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_domain_dentistry_model.pdf
- Domain of Dentistry (July 2018) — https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/domain_of_dentistry_july2018.pdf
- INBDE Technical Report — https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_technical_report.pdf
- INBDE facts (students) — https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_facts_students.pdf
- INBDE facts (state boards) — https://jcnde.ada.org/-/media/project/ada-organization/ada/jcnde/files/inbde_facts_state_boards.pdf
- ADA — Licensure for International Dentists — https://www.ada.org/resources/careers/licensure/licensure-for-the-international-dentists
- ADA — Dental Licensure by State map — https://www.ada.org/resources/careers/licensure/dental-licensure-by-state-map
- ADEA — Foreign-Educated Dentists — https://www.adea.org/godental/Apply/admissions-process/nontraditional-applicants/foreign-educated-dentists
- ADEA — International Dentists / CAAPID — https://www.adea.org/home/application-services/international-dentists
- ADEA CAAPID program directory — https://programs.adea.org/CAAPID/
- ECE — ADA/JCNDE evaluations — https://www.ece.org/ECE/Credential-Evaluations/US-Institutions/ADA-JCNDE
- ABDE / ADEX — https://adextesting.org/ and https://adextesting.org/adex-dental/
- Dental Board of California — Licensure by ADEX — https://www.dbc.ca.gov/applicants/licensure_by_adex.shtml

University / secondary (corroborating; used where official was unreachable):
- Penn Libraries INBDE guide — https://guides.library.upenn.edu/INBDE
- UF mirror, Item Dev Guide — https://dental.ufl.edu/wordpress/files/2025/03/INBDE_Item_Development_Guide.pdf
- UOP Dugoni FK areas PDF — https://dental.pacific.edu/sites/default/files/users/user244/INDBE%20Foundational%20Knowledge%20Areas.pdf
- Columbia advanced standing — https://www.dental.columbia.edu/education/dds-program/advanced-standing-program-internationally-trained-dentists
- Harvard advanced standing DMD — https://www.hsdm.harvard.edu/advanced-standing-international-dentists-dmd-track
- dentalproguide (INBDE pattern) — https://dentalproguide.com/pattern-of-inbde-exam/
- Kaplan INBDE — https://www.kaptest.com/inbde/about-the-inbde
- BoardVitals — https://www.boardvitals.com/blog/things-to-know-about-the-inbde-dental-board-exam/
