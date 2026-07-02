# IMPROVEMENT_PLAN.md — T13 phase-2 merged patch plan (the factory improved by its own specialists)

_Synthesized 2026-07-02 by a neutral improvement synthesizer from four own-specialist audits
(`kr` schema/contract, `method` measurement/determinism, `loops` feedback/self-improvement,
`infosci` organization/manifests/granularity), each run as a fresh context against
`ORIGIN_CRITIQUE.md` / `ORIGIN_EXCHANGE.md`. Every claim below was independently re-verified
against the live working tree before this plan was written; the full re-gate results are at
the bottom._

## Verdict-driven story — which origin-critique gap each patch closes

The origin critique's one-line diagnosis: *the scaffold is excellent wherever a check can be
deterministic, and silent wherever it cannot* — and none of the silent zones was even named.
This plan closes every YES-fixable gap along that boundary, in three moves:

1. **Name the boundary, then govern it** (G1, G5). PLAN.md now says in normative text what
   was only ever practiced: *exit 0 = schema-true, never content-true* (§3a), and *helper
   validation is advisory; the orchestrator always re-executes the gate* (§3b). The same
   boundary statement is now in the FACTORY kit's README, where external vertical builders
   read first (P1, P2, P10).
2. **Push the deterministic frontier outward — additively, never acceptance-changing**
   (G4, G6, G9, G2). The KB gate gains a FAIL-level schema-version pin (verified to hold for
   all 204 in-repo KBs before it was added), a WARN-only six-check source_registry entry
   contract (zero fires on all gated KBs), and a WARN-only density near-miss annotation
   (fires only beside an already-failed count check). The specialist gate becomes
   cwd-independent (a real standalone-load bug, reproduced and fixed), gains a warn channel,
   KB-node groundedness resolution, and a CQ-coverage floor (P14, P15). What determinism
   cannot reach is specified as named process protocols: sampled source-existence
   verification (§3d), and the fresh-context CQ acceptance test
   (MAKE_A_SPECIALIST.md §6 Steps 3–4).
3. **Close the open loops** (G7, G8, G11, G12). The error signal the templates always
   collected but `FAIL -> rm` destroyed now has a recorded, damped, thresholded,
   human-gated feedback path (§3f), the "self-repair once" rule gets its previously
   unwritten and unbounded orchestrator side (§3g), the day-1 dangling "run manifest"
   promise is implemented rather than un-promised (§2a + `RUN_MANIFEST.jsonl`), the kernel's
   long-unwired `OUTPUT_BUDGET_HINT` input is wired into the live templates, specialist
   granularity is decoupled from the GROUP_SIZE batching constant (§6a), the neutralize
   step's self-contradiction of the never-ingest axiom is fixed (§6.2 + §6a), and the T12
   self-improvement loop is generalized to any vertical (`FACTORY/SELF_LOOP.md`).

Gaps with no remaining in-repo fix: **G3** (gate fault-injection suite) and the proving
runs are scheduled as T13 phase 3 ("pro test"); **G10** (calibration single-sampling) is
closed history — its lesson is encoded as §3f's aggregation/damping rule instead.

---

## APPLY_NOW — merged, ordered patch list (P1–P15; docs first, validators last)

**All fifteen patches are ALREADY APPLIED to the working tree** by the four auditors and
were re-verified by this synthesizer (file-by-file diff inspection + independent full
re-gate; results at bottom). Order below is the safe application order for replaying them
onto a clean checkout.

| # | File | Where (exact insertion point) | Change | Gap | Auditor |
|---|---|---|---|---|---|
| P1 | `PLAN.md` | New `### 3a. The determinism boundary (gate limits — named)` immediately after §3's closing line "Exit code 0 = accept; non-zero = reject -> helper self-repairs once, else escalate." | Names the boundary normatively: gate proves STRUCTURE + MATH only; "Exit 0 = schema-true, never content-true"; enumerates the process lane gating prose truth with the exact file each piece lives in (N1 honesty tags + N1–N7 neutralize gate in `FACTORY/sweater_vertical/specialists/BRAIN/BRAIN.md` + `router.json`; T12 fresh-context loops in `BRAIN/SELF_LOOP.md` + `self_improvement_loop.json`; the deterministic evidence-label slice already in `kb_validator.py`); prose accepted only after the process lane or human review (`risk_if_wrong >= 0.80`), never on exit code alone. | G1 | method |
| P2 | `PLAN.md` | New `### 3b. Gate custody (normative)` immediately after §3a. | Helper-run validation in `prompts/*.md` is ADVISORY ONLY (enables the one self-repair pass); the ORCHESTRATOR ALWAYS RE-EXECUTES `kb_validator.py`/`specialist_validator.py` on the written file before accepting/committing/building; a helper-reported `validator_overall_status` is never grounds for acceptance. Makes §7 OPERATIONAL METHOD 6 practice normative. | G5 | method |
| P3 | `PLAN.md` | New `### 3c. Density-band near-miss annotation (warn-level, additive)` after §3b. | Documents the P14(b) warn: kernel guarantee #9 (density, last) justifies NAMING a <=10% band miss machine-readably; bands stay calibrated cut scores; NO automatic waiver — acceptance change = standard-setting review, not code. | G9 | method |
| P4 | `PLAN.md` | New `### 3d. Source reality (fabricated-sources lane — warn-level, additive)` after §3c. | Two-layer G4 response: names the six deterministic WARN checks (P14(c)) incl. the statement that empty locators on heuristic priors are kernel-legal; plus the process layer — orchestrator samples >=1 non-heuristic source entry per accepted batch for fresh-context/human existence verification; invented citation = HARD reject + recorded fabrication incident. | G4 | kr |
| P5 | `PLAN.md` | New `### 3e. Schema-version pin (gate <-> kernel desync guard)` after §3d. | Documents the P14(a) FAIL-level pin and the rule: a kernel bump must extend `SUPPORTED_KB_SCHEMA_VERSIONS` together with re-derived bands/formulas — fail loudly, never silently gate a new schema with old math. | G6 | kr |
| P6 | `PLAN.md` | New `### 3f. Gate-failure feedback loop (closes the shelved PROMPT_SECTION_REFACTOR path)` after §3e. | Normative 4-step loop: RECORD (orchestrator appends terminal gate outcomes to `calibration/results/GATE_LEDGER.jsonl` BEFORE any `rm`; rm rule unchanged) -> AGGREGATE per batch by check-id family (damping; the G10 lesson in reverse) -> THRESHOLD (>=3 units or >=20% of batch = deadband) -> HUMAN-GATED ACTUATION (proposals to `calibration/results/REFACTOR_PROPOSALS/`, never auto-applied; kernel's PROMPT_SECTION_REFACTOR un-shelved as escalation-only target). | G7 | loops |
| P7 | `PLAN.md` | New `### 3g. Bounded self-repair + escalation (normative; the "once" rule kept)` after §3f. | Keeps the one-self-repair cap with three loop-stability reasons; writes the previously missing uniform escalation path: helper STOPs and reports fail (never loops, never deletes); orchestrator records the §3f ledger line, respawns at most 2 FRESH contexts, then PARKs the unit into §3f aggregation. Bounds the formerly unbounded orchestrator retry. | G7 / failure-path | loops |
| P8 | `PLAN.md` + `calibration/results/RUN_MANIFEST.jsonl` | §2 last bullet amended ("+ run manifest (§2a)"); new `### 2a. Run manifest` inserted between §2 and §3; new one-line JSONL file created. | Implements (rather than un-promises) the day-1 dangling `HEADER.txt`/"§2 run manifest" reference: path `calibration/results/RUN_MANIFEST.jsonl`, orchestrator-only writer at terminal ACCEPT (§3b custody), record `{artifact, kind, schema_version, mode, directive, validator, est_tokens, ts}` (directive = the promised provenance; est_tokens = the missing cost ledger); explicitly complementary to §3f's GATE_LEDGER (provenance vs outcomes); NO-BACKFILL clause protects the honesty discipline; file starts with one honest adoption-event line. | G11a | infosci |
| P9 | `PLAN.md` | §6 step 2 corrected inline ("ONE helper job; orchestrator only routes RAW_IDEA in and gates the result — §6a"); steps 6–7 annotated; new `### 6a. Unit & boundary rules (normative; T13/G8 + G12)` after step 7. | (a) KB unit rule: `domains_per_kb=1` (DECISION.json) + the §A.3 natural-fill grain rule; GROUP_SIZE named as a throughput knob that "sets the boundary of NOTHING downstream". (b) Specialist boundary = DOMAIN COHERENCE over ALL (and only) the domain's gate-passing KBs, never batch size or token ceiling — with the verified in-repo histogram (root 28/28x3; phase_b 11/12x2 + 1/12x3; sweater 9/9x3); step 7's "per validated KB" marked as the GROUP_SIZE=1 coincidence. (c) Neutralize custody: neutralization is a GENERATION task -> ONE fresh-context helper job under [FACT]/[ESTIMATE]/[UNKNOWN] + N1–N7; orchestrator only routes and gates — fixing the single step that contradicted the never-ingest axiom. | G8 + G12 | infosci |
| P10 | `FACTORY/README.md` | New `## The determinism boundary — read before trusting a green gate` after the Easy-mode paragraph ("...passes from any directory."); plus one new "Start here" table row for `SELF_LOOP.md` and a parenthetical in item 3 of the new list pointing at `FACTORY/SELF_LOOP.md`. | Operator-facing boundary statement for the self-contained kit: "ALL GREEN = schema-true, never content-true"; names the three process-lane mechanisms with their real file homes; closes "check its tag and its critique trail — not its exit code"; wires discoverability of P11. (Two auditors' edits to one file, verified composed cleanly.) | G1 (FACTORY audience) | method + loops (merged) |
| P11 | `FACTORY/SELF_LOOP.md` | New file (59 lines; did not exist). | Generalizes the T12 sweater self-improvement loop to ANY vertical as a POINTER + slot-binding spec: canonical protocol stays in the sweater files (nothing copied/moved); slot table (targets, slugs, scope doc, load-bearing joints, re-gate command, output dirs, rollups, residue greps); NOT-re-parameterizable invariants (fresh context per role; honesty discipline factory-invariant; apply_now = mechanical + still ALL GREEN; orchestrator re-gates; bounded rounds with honest NOT-CONVERGED exit); feed-forward note distinguishing self-loop (improves one artifact) from §3f (improves the machine). | loop generalization (G7 family) | loops |
| P12 | `FACTORY/MAKE_A_SPECIALIST.md` | Blockquote "Boundary rule" after the §0 "grounded in 3 dense KBs" paragraph; `OUTPUT_BUDGET_HINT` line + note in the §5 Path A labeled-input example; new Steps 3–4 appended in §6 after the existing Step 2 gate command. | (a) Domain-coherence boundary rule where external builders read it (3 KBs = house default, not gate requirement; never split/merge to chase a count/ceiling). (b) `OUTPUT_BUDGET_HINT` (<= ~45k est tokens, calibrated ceiling) added to the Path A kernel-input example. (c) Step 3: how to read the new advisory warns (`grounding.node_ids_resolve`, `coverage.cq_floor`, cwd-independent paths). Step 4: the CQ acceptance test as an LLM-lane protocol — a fresh context loading ONLY the specialist JSON answers >=3 sampled CQs per grounded KB, judged by a second fresh context holding the KB; failure = under-distilled, back to Step 1; record sample + verdict beside `.validation.json`. | G2 (process half) + G8 + G11b | kr + infosci (merged) |
| P13 | `prompts/kb_generation_job.md` + `prompts/_a4_subdomain_job.md` | kb_generation_job.md: new `OUTPUT_BUDGET_HINT` line in "Inputs the orchestrator fills" + the literal hint line in the injected step-2 input block after DOMAIN; _a4_subdomain_job.md: one new input line after DOMAIN_CONTEXT. | Wires the kernel's optional `OUTPUT_BUDGET_HINT` input (recognized since v1.4.1, consumed only in PASS 0 density-safety) into both live kernel-feeding templates, defaulted from the calibrated ~45k ceiling. Verified gate-inert: 45k exceeds every observed dense output (max 40.6k), so it cannot push generations below count bands; affects only FUTURE runs. Deliberate non-edits: `_a4_phaseb_job.md` (doesn't feed the kernel as a prompt) and `calibration_runner.md` (archival record of a completed experiment — see backlog B9). | G11b | infosci |
| P14 | `validators/kb_validator.py` | (a) `SUPPORTED_KB_SCHEMA_VERSIONS = {"1.3"}` constant after `LEGAL_EDGE_TYPES` + `check_schema_version()` wired via `_guard` immediately after `check_top_level`; (b) near-miss warn inside `check_counts()` density loop, immediately after the existing ok/fail line; (c) `SOURCE_ENTRY_FIELDS`/`INTERNAL_SOURCE_TYPES`/`STRONG_EVIDENCE_LABELS`/`DOI_RE`/`URL_RE` constants + `check_source_registry()` wired after `check_formulas`. | ONE merged validator patch, three additive check families: (a) FAIL-level `version.schema_matches_metadata` + `version.supported` — verified to hold for 204/204 in-repo KBs BEFORE adding; (b) WARN-level `count.<key>.near_miss` (<=10% band-edge miss, fires ONLY beside an already-failed count check — can never appear on a passing KB); (c) six WARN-only source-entry checks: `entry_shape`, `external_locator_present` (heuristic priors legally keep empty locators), `locator_format` (claimed DOI/URL/ISBN must parse), `label_eligibility_nonempty`, `unreferenced` (decorative sources), `strong_label_source_eligible`. Exit-code semantics untouched (warns exit 0). | G6 (fail) + G9 (warn) + G4 (warn) | kr + method (merged) |
| P15 | `validators/specialist_validator.py` | New `resolve_kb_path()` + `kb_id_universe()` helpers + regex constants after `PLACEHOLDERS`; `grounded_kbs_exist` body rewritten to use `resolve_kb_path`; `warn()` channel beside ok/fail; two warn checks after `grounded_kbs_exist`; report gains `warnings` count + `pass_with_warnings` status; tag bumped to `specialist_validator/1.1`; exit code still computed from fails only. | (a) FIXES a reproduced standalone-load bug: `grounded_in_kbs` paths now resolve cwd-independently (as given, then spec dir, then each ancestor — a strict superset of the old lookup; `kr.specialist.json` failed from `cwd=/` under the old gate, passes now). (b) WARN-level `grounding.node_ids_resolve` (UPPER_SNAKE id-like tokens in spec text resolved against the grounded KBs' full id universe) and `coverage.cq_floor` (>=1 covered CQ per grounded KB). Warn-level is forced by measurement: FAIL groundedness would flip 8/48 in-scope specialists; FAIL full-CQ coverage would flip all 48 (floor ratio 0.28). | G2 (deterministic half) | kr |

---

## Conflicts resolved (merger decisions, with the kernel's CORE GUARANTEE ORDER as tiebreak)

1. **G9 near-miss: annotate vs waive** (method M4 vs M5; kr silent). Resolved by guarantee
   order: "Useful density" is #9 — LAST — which justifies *naming* the near-miss
   machine-readably, but guarantees #1–8 plus the calibrated cut-score status of the bands
   (DECISION.json; natural_fill >= 19 nodes) dominate any acceptance change. Since warnings
   already exit 0, any fail->warn conversion flips exit 1->0 for `build.sh` and the §7
   harvest loop — acceptance-changing by construction, hence NOT provably non-regressive.
   Applied: the warn annotation (P14b). Demoted: the `--waive-near-miss` path (backlog B5).
2. **Prompt-template edit scope** (infosci applied `OUTPUT_BUDGET_HINT` to two templates;
   method M6 and loops P2 withheld their template edits citing constraint 3). Resolved by
   effect class: pure INPUT wiring of a kernel-recognized optional input is mechanical,
   additive, verified gate-inert (the kernel consumes the hint only in PASS 0
   density-safety; 45k > max observed 40.6k — it cannot touch guarantees #1–8 and can only
   serve #9), and nothing re-gates prompt files — kept as applied (P13). Edits that change
   helper REPORTING/ESCALATION semantics (advisory-status lines, terminal-escalation
   lines) alter helper behavior and need a sample helper run first — merged into ONE
   proposed backlog item (B6). PLAN §3b/§3g already govern future runs normatively either way.
3. **GATE_LEDGER vs RUN_MANIFEST** (loops §3f vs infosci §2a — two new JSONL ledgers in the
   same directory). Resolved as complementary, not duplicate: GATE_LEDGER = gate OUTCOME
   events (all terminal runs, accepts AND rejects, with failed_check_ids); RUN_MANIFEST =
   accepted-artifact PROVENANCE records (the HEADER.txt promise + cost ledger). §2a states
   the relation explicitly; both share the same writer (orchestrator only, §3b custody) and
   the same write moment for accepts.
4. **PLAN.md concurrent sectioning** (four auditors inserting into one file). Verified: no
   numbering collisions — §2a, §3a–3g, §6a land sequentially; kb_validator.py's docstring
   cross-reference "PLAN.md section 3d" resolves; the custody clause every other section
   leans on (§3b) is referenced consistently by §2a, §3f and `FACTORY/SELF_LOOP.md`
   invariant 4.
5. **FACTORY/README.md dual edit** (method's boundary section + loops' pointer edits into
   that section's list). Verified composed: loops edited AFTER re-reading the live tree;
   the parenthetical sits inside method's item 3 without altering its claim.
6. **CQ acceptance test ownership** (kr KR-6 wrote it into MAKE_A_SPECIALIST.md §6 Step 4 as
   an LLM-lane protocol; the origin critique framed it as a "free acceptance test derivable
   from the kernel"). Resolved: it CANNOT be gate-level today (fail-level full coverage
   flips 48/48 validated specialists — measured), so the deterministic gate carries only the
   `coverage.cq_floor` warn (P15) and the semantic test lives as process doc (P12),
   consistent with the §3a boundary. No auditor disagreed; recorded to prevent future
   "promote to gate" drift without the B2 triage.

---

## PROPOSED backlog (correct ideas the hard constraints forbid landing today)

| # | Origin | What | Why not now |
|---|---|---|---|
| B1 | kr KR-P1 | Kernel `kb_schema_version` 1.4: structured locator `{scheme, value}` + `verification_status` on source records, enabling a HARD locator requirement for external-authority source types. | The kernel is the owner's vendored artifact; editing it silently desynchronizes every consumer and trips the P14(a) version pin by design. Needs an explicit schema migration + band/formula re-derivation + `SUPPORTED_KB_SCHEMA_VERSIONS` extension. |
| B2 | kr KR-P2 | Promote `grounding.node_ids_resolve` warn -> fail after triaging the 8 flagged specialists (fix true dangling refs like ir's `CA_*` / security's `ESC_*`; allowlist legitimate artifact names like `DECISION_MEMO`, `SCOPE_GLOSSARY` via an explicit `external_artifacts` field). | Fail-level today flips 8/48 validated specialists red (measured); cleaning them = rewriting validated content. Both banned. |
| B3 | kr KR-P3 | `prompts/source_verification_job.md` — executable form of §3d's process layer (fresh-context helper verifies one sampled non-heuristic source entry exists and supports its claims; NOT_FOUND/MISATTRIBUTED = hard reject + fabrication incident). | Needs network/human resources + orchestration wiring; not mechanically verifiable as non-regressive from inside an audit. Protocol already normative in §3d. |
| B4 | kr KR-P4 | Optional `cq_acceptance: {sampled, verdict, judged_by}` block in `FACTORY/specialist.template.json` so Step 4 results have a canonical in-artifact home. | Changes the authored contract surface all future specialists copy; should ride with the B2 review, not land mid-phase. |
| B5 | method M5 | The actual G9 waiver: `--waive-near-miss` token downgrading a <=10% band near-miss fail->warn, recorded with rationale in the sidecar. | Acceptance-altering by construction (exit 1->0 for the waived class); the bands are calibrated cut scores anchoring the natural-fill grain rule; a code-defaulted tolerance invites criterion drift. Requires standard-setting review + human sign-off. |
| B6 | method M6 + loops L-P2 (merged) | Job-template harmonization, one line each across the five `prompts/*.md` templates: (a) "status is ADVISORY; the orchestrator re-executes the validator (PLAN.md §3b)" beside each report-format status field; (b) explicit terminal-escalation instruction ("after the one self-repair pass: STOP, do not delete the file, return fail — this is your escalation") in `_a4_subdomain`/`_a4_phaseb`/`_a5_specialist`, matching kb_generation_job.md's `needs_orchestrator` contract. | Templates are injected verbatim into helpers — any change alters helper behavior; re-gate a sample helper run after editing before adopting. Some are also archival records of completed runs (versioning decision belongs to the parent orchestrator). |
| B7 | loops P1 | `tools/append_gate_ledger.py` — ~30-line stdlib writer making §3f step 1 mechanical (validation report + metrics sidecar -> one GATE_LEDGER.jsonl line). | New executable surface that no batch exercises in T13 phase 2; shipping untested loop instrumentation would repeat the G3 pattern (an unproven trust anchor). |
| B8 | loops P3 | One proving self-loop round (R1–R4) on one ROOT specialist (suggest `specialists/loops.specialist.json`) using the `FACTORY/SELF_LOOP.md` slot bindings, gate = raw validators. | A never-run protocol generalization is an unvalidated claim; belongs in T13 phase 3 ("pro test", task #35) as an activity, not a patch. |
| B9 | infosci P8 | If calibration is ever re-run (kernel bump per §3e, or a G10-style confirmatory replication): the re-run's job spec carries the P13 `OUTPUT_BUDGET_HINT` line, and each accepted run appends a RUN_MANIFEST.jsonl line. Do NOT retro-edit `prompts/calibration_runner.md` now. | The file is the archival record of a COMPLETED experiment whose committed outputs (DECISION.json, GRAIN_*) cite it; editing it mutates the provenance chain of a locked decision. |

---

## Verification the orchestrator MUST run after applying (full re-gate)

All four commands below were already run by this synthesizer against the patched working
tree on 2026-07-02; re-run them after any replay/merge/commit of these patches.

```bash
cd /home/user/automation

# 1. Every KB in the repo through the KB gate (expect: only the 2 pre-existing
#    phase_b/scaffold/ingest/_drafts/*.draft.kb.json fail — they failed at baseline too)
find . -name '*.kb.json' | while read -r k; do
  python3 validators/kb_validator.py "$k" --mode dense --quiet || echo "FAIL: $k"
done

# 2. Every specialist in the repo through the specialist gate (expect: zero fails)
find . -name '*.specialist.json' | while read -r s; do
  python3 validators/specialist_validator.py "$s" --quiet || echo "FAIL: $s"
done

# 3. Every FACTORY vertical dir with specs through build.sh (expect: ALL GREEN, exit 0, each)
for d in $(find FACTORY -name '*.specialist.json' -not -path '*/BRAIN/*' -exec dirname {} \; | sort -u); do
  ls "$d"/*.spec.json >/dev/null 2>&1 && { bash FACTORY/build.sh "$d" >/dev/null || echo "FAIL: $d"; }
done

# 4. Forge round-trip must be byte-identical (expect: git status shows NO *.kb.json changes)
git status --short | grep -E '\.kb\.json' && echo "REGRESSION: forge not byte-identical" || echo "forge round-trip clean"
```

### Results of this synthesizer's independent run (2026-07-02, patched tree)

- **KB gate:** 204/204 `*.kb.json` — exit codes IDENTICAL to the pre-edit (HEAD) validator
  file-by-file (zero diffs). Status: 195 `pass`, 7 `pass_with_warnings` (all in
  `phase_b/knowledge_base/`, `source_registry.unreferenced` only, exit still 0),
  2 `fail` (the two pre-existing `_drafts`, failing at baseline). All 84 root
  `knowledge_base/` KBs and all 60 FACTORY KBs: clean `pass`, ZERO warns.
- **Specialist gate:** 67/67 `*.specialist.json` — exit-code parity with the pre-edit
  validator (zero diffs), zero fails; 17 get advisory warns only. Standalone-load fix
  confirmed: `specialists/kr.specialist.json` from `cwd=/` — OLD validator exit 1,
  NEW validator exit 0.
- **build.sh:** 20/20 FACTORY spec dirs (9 sweater + 10 turlock_business + turlock_income)
  exit 0, ALL GREEN; `git status` shows no `*.kb.json` modified (byte-identical forge
  round-trip).
- **Cross-references:** `kb_validator.py` -> "PLAN.md section 3d" resolves; PLAN §2a/§3a–3g/
  §6a numbering collision-free; `_a4_phaseb_job.md` and `calibration_runner.md` confirmed
  untouched (deliberate non-edits).

### Constraint compliance statement

Every P-patch is additive; no validated KB or specialist content was rewritten; the only
FAIL-level additions (P14a version pin) were verified to hold for ALL 204 in-repo KBs
before being added; every other validator addition is warn-level and exit-code-inert; all
doc edits went to PLAN.md / FACTORY docs (plus gate-inert input wiring in two live
templates, per conflict resolution 2); the honesty discipline is strengthened in doc form
(§3a, §6a, SELF_LOOP.md invariant 2, README boundary section) and weakened nowhere.
