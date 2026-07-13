# SCORING — trust & quality scoring for the trio (v1 scaffold)

PROTOCOL-VERSION: 3.3

Single writer: integrator (Claude). This is the v1 scaffold; the `agenteval`
specialist (TASK-008, Codex) will deepen the rubric and Goodhart-resistance
math, after which this file is revised. Design principle from the research
(RESEARCH_DIGEST §B, §D): **weight our own observed track record over any
published benchmark**, and make every score resist gaming.

## 1. Two things get scored

### 1a. Artifact quality (per delivered work product) — 0..10

Composite, computed by the integrator at verdict time:

| Component | Weight | Source | Gaming-resistance |
|---|---|---|---|
| Gate result (exit 0, mid-band counts, zero warnings) | pass/FAIL gate | deterministic | can't be judged around — necessary precondition |
| Independent re-forge byte-identical | pass/FAIL gate | deterministic | detects hand-edited KBs |
| Content-review score (blind, de-biased panel §3) | 0.55 | LLM judges | position-swap + odd jury + no self-grade |
| Verified factual errors found (count, severity-weighted) | −0.25 each major | reviewer + fact-check | rewards finding, not hiding |
| Revise rounds to acceptance | 0.20 (fewer = higher) | ledger | can't pad — more rounds lowers it |

Gate + re-forge are pass/FAIL preconditions; the 0..10 is the judge composite,
docked for confirmed errors and revise churn. A score is only recorded after
`VERDICT: ACCEPTED`.

### 1b. Agent trust (per agent, per task-type) — a rolling profile

Not one global number — a **per-task-type** competence vector (content-author,
code-build, verify, judge), because the benchmarks show competence is lane-
specific. For each (agent, task-type):

```
trust = 0.50 * accept_rate_first_pass      # delivered and accepted with <=1 revise
      + 0.25 * mean_artifact_quality/10     # from 1a
      + 0.15 * xverify_catch_rate           # real defects it caught in OTHERS' work
      - 0.10 * self_report_gap              # (self-claimed pass) minus (integrator-verified pass)
```

`self_report_gap` is the Goodhart guard: an agent that over-claims is penalized
exactly where reward-hacking shows up (RESEARCH_DIGEST §B). `xverify_catch_rate`
rewards being a good skeptic, not just a good builder — so a cautious verifier
(e.g. Codex's onboarding conduct) scores well even before it builds anything.

## 2. Scores drive routing (not identity)

- Assignment prior = published-benchmark lane (ORCHESTRATION §1) ONLY until we
  have ≥3 observed tasks in a lane; after that, OBSERVED trust dominates.
- **Exploration floor**: never route 0% of a lane's tasks to a recovering
  agent — reserve ≥1 in 5 for the lower-scored ACTIVE agent so a bad streak
  can be recovered from (exploration-vs-exploitation; don't starve).
- A `DEGRADED` agent (2 consecutive timeouts, ORCHESTRATION §8) scores no new
  tasks until a canary passes; the canary result updates trust.

## 3. Judge de-bias rules (mandatory, from RESEARCH_DIGEST §B)

- **Blind**: authorship stripped before judging.
- **Position-swap**: every pairwise comparison run in BOTH orders, scores
  averaged — neutralizes the up-to-75% first-slot bias.
- **No self-grading**: an agent never judges its own output; with 3 families
  there is always a non-author judge.
- **Odd jury** for high-stakes (≥3 judges, ≥2 families, majority + tie-break).
- **Gates first**: deterministic checks rule before any LLM judge is invoked —
  closes the ~21% verification-gap cheaply.
- **Diversity over redundancy**: prefer different-family or different-lens
  judges over N identical ones (homogeneous panels converge sycophantically).

## 4. What resists gaming (explicit)

- No score without a passed deterministic gate + integrator re-verification.
- Self-reported success never raises a score; the self_report_gap term docks it.
- Finding real defects (in own or others' work) raises score; hiding them,
  when later caught by the canary re-judge, retroactively docks it.
- Revise-count and error-count are ledger facts, not judge opinions.

## 5. Where the numbers live

`EXCHANGE/scores.json` (single writer: integrator) — appended at each verdict;
`git log -p` is the audit trail. `STATUS.md` renders the current profiles in
plain language. v2 (post-agenteval) adds calibration and inter-judge agreement.
