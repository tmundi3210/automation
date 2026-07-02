# T13_REANALYSIS.md — capstone of the T13 re-analysis (critique -> self-improvement -> pro test)

_Written 2026-07-02 by the T13 phase-3 final synthesizer, fresh context. Every claim below
traces to an artifact in this repo (or is explicitly flagged as ephemeral/unreproducible).
Inputs: `ORIGIN_EXCHANGE.md`, `ORIGIN_CRITIQUE.md`, `IMPROVEMENT_PLAN.md` (this directory),
the three phase-3 test-suite reports, and an independent refuter pass over all 31 attacked
verdicts. Honesty rules applied throughout: a miss is reported as a miss; the one red test
is left red on purpose._

---

## 1. What the first message and first answer were

The project's true first exchange — the core motto — is archived with full provenance
honesty in [`ORIGIN_EXCHANGE.md`](ORIGIN_EXCHANGE.md) (the later sweater-vertical kickoff
is separately archived in [`INITIAL_EXCHANGE.md`](INITIAL_EXCHANGE.md)). The verbatim
wording of the owner's first chat message did not survive compaction and is unrecoverable
— nothing in T13 pretends otherwise — but what does survive verbatim is stronger evidence:
the owner's own kernel (`schema/kb_generator_v1.4.1.txt`, 1,924 lines, vendored in the
first scaffold commit), the machine-facing directive (`schema/HEADER.txt`), and the first
answer itself — scaffold commit `0330acf` (2026-06-23), containing `PLAN.md`, `README.md`,
`validators/kb_validator.py`, `validators/metrics.py`, the prompts, and the taxonomy seed.
In substance ([ESTIMATE], high confidence, zero claim on wording) the first message asked:
take this KB-generator schema and build an orchestrated automation around it — an
orchestrator that plans and gates but never generates in-context, helper sub-agents that
each run one schema job, a deterministic quality/quantity gate, then a reusable pipeline
that turns any raw idea into neutralized, academically-grounded, dense, gated KBs and
distills a specialist per KB. The first answer encoded exactly that as five load-bearing
decisions (roles, header reconciliation, deterministic gate, calibration-before-mass,
Phase B as the reusable product), which every later stage of the factory kept.

## 2. The verdict: was the answer correct and most intelligent?

**Correct: yes. Most intelligent: near — merged score 7.3/10** (top-decile competent; a
strictly better first answer was ~one day of foreseeable work away). Full verdict, per-lens
scores, and the deduped gap table are in [`ORIGIN_CRITIQUE.md`](ORIGIN_CRITIQUE.md),
synthesized from three independent fresh-context critics (design-correctness 8.0,
intelligence 6.5, hindsight 7.5) with every citation re-verified against
`git show 0330acf:*`.

All five day-1 architectural choices were validated unanimously — most of all the decision
to refuse to trust the KB's self-reported validation and independently recompute everything
in a deterministic stdlib gate. But four HIGH gaps clustered exactly where determinism ends
and trust begins, and none was even named as a known limitation:

- **G1** — prose/content truthfulness ungated AND unacknowledged (a fabricated-but-well-formed KB exits 0);
- **G2** — the terminal artifact (the specialist, the declared product) had no contract, while the intermediate KB got 1,924 schema lines and a 466-line gate;
- **G3** — the gate itself was never tested: zero fixtures, no fault injection, the whole trust chain bottoming out in one unproven script;
- **G4** — fabricated sources pass the gate: an invented book/DOI resolves and passes every check.

Plus five MED (G5 gate custody, G6 no schema-version pin, G7 shelved self-improvement
mode, G8 granularity coupled to GROUP_SIZE, and the G5-adjacent custody leak) and four LOW
(G9–G12). The critique's one-line diagnosis: *the scaffold is excellent wherever a check
can be deterministic, and silent wherever it cannot.*

## 3. How the system improved ITSELF using its own specialists

T13 phase 2 ([`IMPROVEMENT_PLAN.md`](IMPROVEMENT_PLAN.md)) had four of the factory's own
Phase-A specialists — built by the very scaffold under critique — audit the factory core,
each as a fresh context against the origin critique:

| Auditor (own specialist) | Surface audited | Patches |
|---|---|---|
| `kr` (knowledge representation) | schema/contract: source reality, version pin, specialist contract | P4, P5, P14(a,c), P15, parts of P12 |
| `method` (methodology/measurement) | measurement/determinism: boundary naming, custody, near-miss | P1, P2, P3, P14(b), P10 (part) |
| `loops` (iteration/self-improvement) | feedback loops: gate-failure ledger, bounded self-repair, loop generalization | P6, P7, P11, P10 (part) |
| `infosci` (organization/retrieval) | manifests, granularity, input wiring | P8, P9, P12 (part), P13 |

A neutral improvement synthesizer merged the four audits into **15 patches (P1–P15)**, all
already applied to the working tree and independently re-verified: PLAN.md gains normative
sections §2a and §3a–§3g and §6a (the determinism boundary named, gate custody, source
reality, version pin, gate-failure feedback loop, bounded self-repair, run manifest, unit
and boundary rules); `FACTORY/README.md` gains the operator-facing boundary statement;
`FACTORY/SELF_LOOP.md` (new) generalizes the T12 loop to any vertical;
`FACTORY/MAKE_A_SPECIALIST.md` gains the boundary rule, OUTPUT_BUDGET_HINT, and the CQ
acceptance test; `validators/kb_validator.py` gains the FAIL-level schema-version pin
(verified against all 204 in-repo KBs BEFORE adding) plus warn-only source_registry and
density near-miss checks; `validators/specialist_validator.py` gets a reproduced
standalone-load bug FIXED (cwd-independent KB path resolution) plus warn-level groundedness
and CQ-floor checks. Six auditor conflicts were resolved with the kernel's guarantee order
as tiebreak (IMPROVEMENT_PLAN.md "Conflicts resolved"), and nine correct-but-not-now ideas
were parked as backlog B1–B9 rather than force-landed.

**Independent re-gate result** (IMPROVEMENT_PLAN.md, bottom): 204/204 KBs with exit-code
parity to the pre-patch validator (195 pass, 7 pass_with_warnings, 2 pre-existing draft
fails — see §5); 67/67 specialists exit-parity, zero fails, 17 advisory-warn-only; 20/20
FACTORY spec dirs through `build.sh` ALL GREEN; forge round-trip byte-identical. Every
patch additive; no validated content rewritten; the honesty discipline strengthened in doc
form and weakened nowhere.

## 4. The pro test (T13 phase 3)

### 4a. Fault injection against the gate — 15/16, and the gate is NOT fully proven

The suite exists at `validators/test_kb_validator.py` (stdlib-only, no pytest; run as
`python3 validators/test_kb_validator.py`; exit 0 only when all cases pass). Method: a
pristine known-green KB is copied per case, exactly ONE mutation applied, the validator
invoked exactly as `FACTORY/build.sh` does (`--mode dense`), and the assertion is BOTH the
exit code AND the targeted `check_id` status in the machine report — stronger than
exit-code-only, since it proves the intended defect class tripped, not a side effect.
Control confirmed green before every case; per-case restore prevents cross-contamination.

**Result: 15 of 16 cases pass — including both new T13 additions** (the FAIL-level
schema-version pin and the warn-level source_registry checks with exit 0 preserved, both
also hand-reproduced by the refuter at `kb_validator.py:460–478` and line 356 for the
signed-tension bound). **One case fails, deliberately left red: a real gate miss.** The
placeholder check's `PLACEHOLDERS` set (`validators/kb_validator.py` lines 87–94) contains
only kernel template tokens (`precise_pro`, `NODE_A`, `[DOMAIN]`, ...); a KB containing
`TODO lorem ipsum placeholder text` in any prose field exits 0 with
`placeholders.none_leaked=pass`. Reproduced standalone outside the suite. This is a scope
gap, not a broken check (the kernel-token case passes). So the honest statement of G3's
closure is: **the gate is now TESTED, and testing it immediately found a miss** — which is
the fault-injection suite doing precisely its job. The suggested fix (case-insensitive
regex for TODO/FIXME/TBD/lorem/XXX in `check_placeholders`) was NOT applied — phase-3
instructions forbade touching the validator — and is queued for the next patch cycle,
after which the suite goes 16/16 with no test change.

### 4b. End-to-end mini-build — 8/8, ALL GREEN on attempt 1 of 5

A fresh-context operator, following only `FACTORY/MAKE_A_SPECIALIST.md` + the two templates
+ one real example spec, took a neutral micro-domain (espresso) from idea -> compact spec
-> forged KB -> gated KB -> grounded specialist -> gated specialist through the single
`build.sh` command: **exit 0, zero validator failures, zero warnings, first attempt** (the
5-attempt budget untouched; the only pre-build edit was a self-caught stray non-ASCII
character in the operator's own spec). The T13 hardening was exercised live: the FAIL-level
version pin passed on a forge-built KB, all six warn-level source_registry checks passed,
and `specialist_validator.py` passed identically from `cwd=/` — the P15 cwd-independence
fix confirmed in anger. Honest caveats carried verbatim from the suite: (1) this proves the
deterministic lane only — the KB's prose has NOT been through the §6 Step 4 LLM CQ
acceptance test, which is process, not gate; (2) the package is a legitimate 1-KB minimal
build, not the house-default 3; (3) one fresh-operator friction point: in-band node-count
planning (19–24) is entirely on the author pre-forge. **Evidence caveat (refuter):** the
e2e artifacts live only in the session scratchpad, not the repo — the artifact-level claims
were all re-verified by the refuter (build re-run ALL GREEN, honesty audit reproduced to
the digit), but the evidence is ephemeral and the "self-caught non-ASCII fix" is an
unreproducible process claim.

### 4c. Brain probes — 8/8

Eight probes routed via `FACTORY/sweater_vertical/specialists/BRAIN/router.json`
(routes_when + dominance rules) and answered strictly from the owning specialist file(s)
only. All 8 correct: honesty tags right, gates G1–G5 surfaced by the owning files
themselves, every fabrication path blocked at two layers (the file's own
boundaries/escalations AND the N1–N7 neutralize gate). The T12/T13 fixes proved
load-bearing standalone — the yarn USTER demotion, the India-vs-US-6110 duty separation,
the build-vs-buy anchor set, and the pilling four-way lane split all answer correctly from
a fresh context loading only the routed file. No cross-context rescue needed; no fabricated
number derivable from any routed file.

### 4d. Refuter — 31 attacked, 31 upheld, 0 overturned

An independent refuter attacked all 31 verdicts across the three suites and reproduced
every one: re-ran the fault-injection suite (confirming 15/16 and confirming the miss was
pre-disclosed, not hidden), hand-mutated KBs to spot-check the version pin and the
signed-tension bound (targeted check fires; clean control 43 pass / 0 fail / 0 warn),
re-ran the e2e build ALL GREEN with the honesty audit exact to the digit (0 brand hits,
63/16 ESTIMATE tags, est_tokens=37987), and verified every probe citation against the
actual specialist files, router rules 1/2/4/6/11, and `DECISION_MEMO.md` (G1–G4 table,
GATED verdict, memo line 17). One attribution nuance recorded on P4: the verbatim pilling
"FLOOR" phrase sits in `quality.specialist.json` / router rule 11 while
`yarn.specialist.json` carries the equivalent substance — substance upheld, quote-to-file
mapping slightly loose. Nothing overturned.

## 5. Honest residuals — what stays open

1. **The placeholder scope gap (NEW, found by this pro test).** The gate misses generic
   draft markers (TODO/FIXME/TBD/lorem/XXX) in prose fields. The suite carries it as a
   deliberately red case; fix queued for the next patch cycle. Until then, "exit 0" does
   not certify the absence of draft-marker prose — consistent with the §3a boundary but now
   with a concrete known instance.
2. **The proposed backlog B1–B9** (IMPROVEMENT_PLAN.md, "PROPOSED backlog"): kernel 1.4
   structured locators (B1), groundedness warn->fail promotion after triaging the 8 flagged
   specialists (B2), the executable source-verification job (B3), the cq_acceptance
   template block (B4), the G9 waiver path (B5), job-template harmonization (B6), the
   GATE_LEDGER writer tool (B7), one proving self-loop round on a ROOT specialist (B8), and
   the calibration-rerun provenance rule (B9). Each has a stated reason it cannot land
   today; none is forgotten.
3. **Two draft KBs fail at baseline** —
   `phase_b/scaffold/ingest/_drafts/ftune__demo_ingest.draft.kb.json` and
   `phase_b/scaffold/ingest/_drafts/ftune__smoke.draft.kb.json`. They failed before T13's
   patches and fail identically after (exit-code parity confirmed in the re-gate); they are
   known drafts, not regressions, and remain honestly red.
4. **G1 remains structurally open by design.** Prose truth is still not deterministically
   gateable; T13 closed the *silence* (the boundary is now named normatively in PLAN §3a
   and FACTORY/README) and specified the process lane, but exit 0 remains schema-true,
   never content-true. The e2e KB's prose specifically has not been CQ-acceptance-tested.
5. **Ephemeral e2e evidence.** The mini-domain artifacts live in the session scratchpad,
   not the repo. If the owner wants the e2e run to be a durable regression fixture, the
   spec should be committed under a FACTORY vertical dir (it would then also ride the
   IMPROVEMENT_PLAN re-gate loop #3 automatically).
6. **Overturned passes: none.** The refuter attacked 31 and upheld 31; the only findings in
   the whole pro test are the pre-disclosed placeholder miss (item 1) and the minor P4
   quote-attribution looseness (substance upheld).

## Bottom line

The first answer was **correct and near-most-intelligent (7.3/10)**; its four HIGH gaps all
sat at the determinism boundary. The system then **improved itself with its own
specialists** — 15 additive patches, independently re-gated with zero regressions across
204 KBs, 67 specialists, and 20 build dirs. The pro test **proved the gate by trying to
break it**: 15/16 injected defects caught including both new T13 checks, one real scope
miss found and honestly left red, the full pipeline green end-to-end from a fresh operator
on the first attempt, 8/8 brain probes correct from standalone files, and a refuter unable
to overturn a single verdict. The factory is measurably harder than it was at T12 — and it
now knows, in writing, exactly where it is still soft.

---

## Closure addendum (orchestrator, 2026-07-02, same day)

The one red case did not wait for a "next patch cycle":

- **The placeholder gate miss is FIXED** — `validators/kb_validator.py` now carries
  `DRAFT_MARKER_RES` (word-boundary `TODO`/`FIXME`/`TBD`/`XXX` uppercase-only so prose
  words like "autodoc" can never match, plus case-insensitive `lorem ipsum`), folded into
  the same `placeholders.none_leaked` check the suite asserts on.
- **Proven safe before applying:** the exact regex set was swept across all 204 repo KBs
  first — 0 hits, so 0 possible regressions.
- **Suite re-run: 16/16, exit 0.** The previously-red `placeholder_todo_lorem` case now
  catches (`exit=1, placeholders.none_leaked=fail`).
- **Full parity re-sweep after the fix:** all 204 KBs re-validated against the pre-T13
  HEAD validator — 0 exit-code mismatches (the 2 baseline-failing `_drafts` unchanged).

With this, the fault-injection suite stands at **16/16 defect classes proven caught**, and
the only remaining open items are the process-lane residuals and the B1–B9 backlog listed
above.
