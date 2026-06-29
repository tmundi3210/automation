# Make a Specialist — the whole pipeline, the same way this repo does it

This is the build manual. It teaches you to author a **specialist** (and the dense
**knowledge bases** it stands on) exactly the way the specialists in this repository were
made, gate them with the same deterministic checks, and then **use** one as a prompt or
wire many into a self-checking "brain."

Everything is **Python stdlib only** — no pip install, no GPU, no API keys, no network at
build time. If `python3 --version` works, the factory works.

---

## 0. The one idea you must hold

A **specialist is not a trained model.** It is a dense JSON *operating spec* — a role, a
decision procedure, a workflow, escalation triggers, validation checks. You drop that JSON
into a prompt template and any capable model **reasons *as* that specialist**. Nothing here
is fine-tuned. The intelligence is in the spec and the knowledge behind it, not in weights.

Each specialist is **grounded in 3 dense knowledge bases (KBs)**. A KB is a strict-JSON graph
of ~20 "work units" (nodes), their dependencies/conflicts (edges), risk/priority math,
workflow, and competency questions. The specialist is the *distilled operating layer* over
its 3 KBs.

```
   3 dense KBs  ──distill──►  1 specialist spec  ──fill template──►  a model acts as the expert
  (the knowledge)              (the operating layer)                  (the answer)
```

---

## 1. The pipeline at a glance

```
            ┌─────────────────── PATH A: author with an AI ───────────────────┐
 RAW IDEA ─►│  schema/kb_generator_v1.4.1.txt  (paste into a capable model)    │
            │     + labeled blocks (DOMAIN / PURPOSE_HINT / DENSITY_MODE=dense)│
            │              ▼                                                    │
            │      a strict-JSON dense KB                                       │
            └─────────────────────────────┬───────────────────────────────────┘
                                          │
            ┌──────────────── PATH B: author deterministically ───────────────┐
 RAW IDEA ─►│  write a COMPACT spec  (FACTORY/kb_spec.template.json)          │
            │     ▼                                                            │
            │  python3 branches/_forge/kb_forge.py spec.json -o out.kb.json    │
            │     (computes every derived field; reproducible, pure function)  │
            └─────────────────────────────┬───────────────────────────────────┘
                                          │
                 ┌────────────────────────▼────────────────────────┐
   THE GATE  ►   │ python3 validators/kb_validator.py KB --mode dense│  (counts + refs + formulas)
                 │ python3 validators/metrics.py      KB --label ... │  (density/size)
                 └────────────────────────┬────────────────────────┘
                                          │  (do this 3× → 3 gate-passing KBs)
                 ┌────────────────────────▼────────────────────────┐
  DISTILL    ►   │ write <code>.specialist.json grounded in the 3 KBs│
                 │ python3 validators/specialist_validator.py SPEC   │  (structure + grounding)
                 └────────────────────────┬────────────────────────┘
                                          │
   USE IT    ►   fill dist/prompt_template.json  →  send to a model  →  expert answer
                 (or wire many specialists into a brain + gate; see §6)
```

You can mix the paths: generate a KB with Path A, or forge it with Path B — the **gate and
the distillation step are identical** either way.

---

## 2. The specialist schema (what you are producing)

A specialist is one JSON object. The gate (`validators/specialist_validator.py`) **requires**
these 15 keys, and the listed ones must be **non-empty lists**:

| key | required | shape |
|---|---|---|
| `_directive` | ✅ must contain the word "machine" | string |
| `specialist_id` | ✅ | short code, e.g. `movie_scout` |
| `domain` | ✅ | machine slug |
| `domain_label` | ✅ | human name |
| `purpose` | ✅ | one precise "what + when to invoke" sentence |
| `grounded_in_kbs` | ✅ non-empty **and every path must exist on disk** | list of KB file paths |
| `role` | ✅ | the dense system-prompt-style role statement |
| `capabilities` | ✅ non-empty; each item needs (`capability` **or** `method`) **and** `when_to_use` | list of objects |
| `decision_procedure` | ✅ non-empty | ordered steps |
| `workflow` | ✅ non-empty | phases with entry/exit/gates |
| `escalation_triggers` | ✅ non-empty | `{condition, action, reason}` |
| `validation_checklist` | ✅ non-empty | checkable acceptance conditions |
| `conflicts_and_dominance` | ✅ | `{tradeoff, resolution}` |
| `glossary` | ✅ | `{term, definition}` |
| `competency_questions_covered` | ✅ non-empty | questions it can answer |

Also enforced: **no placeholder tokens** may appear anywhere in the file — the gate rejects
the literal strings `TODO`, `placeholder`, `<...>`, `NODE_A`, `SHORT_ID`, `FAMILY_ID`,
`method_catalog`, `concrete_example`, `precise_pro`, `system-prompt-style`. (So when you fill
the template, replace every `REPLACE_…` marker with real content.)

Heavy specialists in `branches/b60_content_intelligence/` add an optional `boundaries` list
(explicit out-of-scope / hard-stop lines). It is not required by the gate but is good practice.

> Blank to copy: **`FACTORY/specialist.template.json`**.
> Real examples to imitate: any `specialists/*.specialist.json` (28 of them) or
> `branches/b60_content_intelligence/*.specialist.heavy.json` (the 5 heavy ones).

---

## 3. The knowledge base (what the specialist stands on)

A dense KB is a strict-JSON object. The gate runs in `--mode dense` and enforces these
**count bands**:

| section | dense band |
|---|---|
| `nodes` | 19–24 |
| `edges` | 32–40 |
| `conflict_axes` | 8–10 |
| `edge_cases` | 10–12 |
| `workflow` | 9–12 |
| `competency_questions` | 10–14 |
| `dominance_rules` | 7–12 |
| `anti_rework_rules` | 7–12 |
| `iteration_protocol` | 6–10 |

Plus reference integrity (every edge endpoint / dependency / CQ-ref points at a real id),
acyclic dependencies, evidence-label correctness, and **formula consistency** (every derived
score recomputes to the stored value).

You almost never hand-compute the formulas. You author **content + base-metric magnitudes**;
the forge computes the rest. That is the whole point of Path B.

---

## 4. PATH B — the deterministic forge (recommended for control & reproducibility)

This is how the 5 heavy specialists in `branches/b60_content_intelligence/` were built. It is
a *pure function*: same spec in → same KB out, no randomness, no time. A spec that builds
clean passes the gate's mechanical checks **by construction**.

**Step 1 — write a compact content spec.** Copy the template and fill it:

```bash
cp FACTORY/kb_spec.template.json my_topic.spec.json
# fill: 19-24 nodes (each: prose + the 15 base metrics in [0,1] + uncertainty_interval),
#       32-40 edges, 8-10 conflict_axes, 10-12 edge_cases, 9-12 workflow steps,
#       10-14 competency_questions, dominance/anti-rework/iteration rules.
```

You supply per node: `criticality, business_value, user_value, technical_complexity,
risk_if_wrong, cross_topic_coupling, irreversibility, confidence, node_conflict_pressure,
acceptance_test_pass_rate, dependency_gate_pass_rate, prior_importance, evidence_confidence,
failure_rate, downside_weight`, an `uncertainty_interval [lo,hi]`, and an `update_signal`.
The forge will *refuse with a precise message* if a metric is out of `[0,1]`, a dependency is
cyclic, a high-risk node lacks acceptance tests, or a conflict edge lacks a resolution rule.

> **Tip (how the repo actually does it):** instead of hand-writing 20 verbose node blocks,
> the b60 authors wrote a tiny Python generator with `node()`/`b()` helpers that apply sane
> defaults, then emit the spec. See `branches/b60_content_intelligence/kb/_forge/gen_*.py`
> for the pattern — it just `json.dump`s the same structure the template shows.

**Step 2 — forge the full KB:**

```bash
python3 branches/_forge/kb_forge.py my_topic.spec.json -o my_topic.kb.json
# prints the computed counts; the build is reproducible
```

**Step 3 — gate it:**

```bash
python3 validators/kb_validator.py my_topic.kb.json --mode dense --report my_topic.validation.json
echo "EXIT=$?"   # 0 = pass
python3 validators/metrics.py     my_topic.kb.json --label run1 --report my_topic.metrics.json
```

Repeat Steps 1–3 **three times** → three gate-passing KBs for one specialist.

---

## 5. PATH A — author with an AI (recommended for breadth & speed)

This is the original "factory" path (`README.md`, `PLAN.md`). You let a capable model write
the dense KB JSON directly from the master generator prompt, then gate it the same way.

**Step 1 — generate the KB.** Paste the full contents of `schema/kb_generator_v1.4.1.txt`
into a strong model, then add labeled input blocks:

```
MODE: KB_GENERATION
RAW_IDEA: <your topic / idea in a sentence or two>
DOMAIN: <the subdomain this KB covers>
DOMAIN_CONTEXT: <audience, constraints, what it must support>
PURPOSE_HINT: <what an expert in this subdomain must be able to decide/do>
DENSITY_MODE: dense
```

The model returns **strict JSON** (first char `{`, last char `}`, nothing else). Save it as
`my_topic.kb.json`.

**Step 2 — gate it** (identical to Path B Step 3):

```bash
python3 validators/kb_validator.py my_topic.kb.json --mode dense --report my_topic.validation.json
```

If it fails, hand the model the failed-check list from the report and ask for one repair pass.
Repeat for the 3 subdomain KBs.

> Faster paste-ready variant: **`FACTORY/GENERATOR_PROMPT.md`** is a single self-contained
> prompt that asks any model to emit *both* a dense KB and the specialist spec in this repo's
> exact schema — use it when you don't want to wrangle the 73 KB master generator.

---

## 6. Distill the specialist (same for both paths)

Once you have **3 gate-passing KBs**, distill them into one operating spec. This mirrors
`prompts/_a5_specialist_job.md`.

**Step 1 — write the spec.** Copy the template and synthesize *across* the 3 KBs (don't copy
them verbatim):

```bash
cp FACTORY/specialist.template.json my.specialist.json
```
- pull **capabilities** from KB nodes (topic + method + when_to_use) — must cover all 3 KBs;
- merge KB **workflow** steps into the specialist `workflow`;
- pull **escalation_triggers** from nodes with `human_review_required` + conflict thresholds;
- pull **validation_checklist** from KB `acceptance_tests` + validation checks;
- pull **conflicts_and_dominance** from `conflict_axes` + `dominance_rules`;
- set `grounded_in_kbs` to the **exact paths** of your 3 KB files (they must exist on disk).

**Step 2 — gate it:**

```bash
python3 validators/specialist_validator.py my.specialist.json --report my.specialist.validation.json
echo "EXIT=$?"   # 0 = pass
```

If it fails, read the report, do one repair pass, re-run. Done: you have a specialist built
the same way every specialist in this repo was built.

---

## 7. How to USE a finished specialist

**Single specialist, one question.** The harness is `dist/prompt_template.json`. Its
`specialist_prompt_template` has three slots — fill them and send to any model:

- `{{DOMAIN_LABEL}}` ← your spec's `domain_label`
- `{{SPECIALIST_SPEC_JSON}}` ← the **entire** specialist JSON, verbatim
- `{{USER_QUESTION}}` ← the end-user's question

The system message tells the model to operate strictly by the spec's `role` /
`decision_procedure` / `workflow`, honor every `escalation_trigger`, and answer only within
the domain. That's it — no training, no setup.

> The repo also ships `dist/specialist_prompts.jsonl` — one *fully assembled* prompt per
> specialist (spec already embedded). Pick a line, drop in your question.

**Pick the right specialist automatically.** `dist/prompt_template.json` also has a
`router_prompt`: give a model the routing table (`{code: route_when[]}` from `MANIFEST.json`
or `specialists/ROUTER.json`) + the question, and it returns the best specialist `code`.

---

## 8. Wire many specialists into a "brain" (the loop)

One specialist answers one question. A **brain** is a thin controller that runs several
specialists in a fixed order with deterministic gates — exactly what
`branches/b60_content_intelligence/` demonstrates:

- **`BRAIN.md`** — the controller that wires the 5 heavy specialists into one
  map → link → signal → generate → **gate** → test loop.
- **`BRAIN_STEP1.md` + `GATE_STEP2.md`** — the two copy-paste prompts that make any model
  *be* the brain (collect → connect → draft + flag) and then a **separate frozen safety gate**
  (return one GREEN / YELLOW / STOP verdict). Splitting "think" from "decide" is the key move:
  the brain never makes the safety call; a frozen gate does.
- **`scaffold/run_pipeline.py`** — the same spine as runnable, offline, stdlib-only code
  (`python3 branches/b60_content_intelligence/scaffold/run_pipeline.py`).
- **`movie/MOVIE_BRAIN.md`** — a worked example of carving a *vertical* (a film studio) out of
  the general brain: same 5 specialists, re-scoped, with its own paste-ready prompt.

To build your own brain: pick the specialists, decide the order, and make the safety/quality
gate the **sole predecessor** of any irreversible step (publish, send, render). Copy the
two-prompt brain/gate split from `BRAIN_STEP1.md` + `GATE_STEP2.md`.

---

## 9. Install & run locally (recap)

```bash
# A) don't have the repo yet:
bash FACTORY/setup_local.sh --clone            # clones, then self-tests all three gates

# B) already cloned — verify from inside the repo:
bash FACTORY/setup_local.sh
```

The script checks `python3`, then proves the factory works on your machine by gating a real
KB, gating a real specialist, and round-tripping a spec → KB → dense gate. All green = you're
ready to author your own.

Manual clone, if you prefer:

```bash
git clone https://github.com/tmundi3210/automation.git
cd automation
python3 --version                              # need 3.8+
python3 validators/kb_validator.py branches/b60_content_intelligence/kb/geo__attention_topology.kb.json --mode dense
```

---

## 10. Cheat sheet (every command, confirmed working)

```bash
# forge a KB from a compact spec
python3 branches/_forge/kb_forge.py SPEC.json -o KB.json [--quiet]

# gate a KB (quality + quantity)
python3 validators/kb_validator.py KB.json --mode dense [--report R.json] [--quiet]
python3 validators/metrics.py      KB.json --label NAME --report M.json

# gate a specialist (structure + grounding)
python3 validators/specialist_validator.py SPEC.json [--report R.json] [--quiet]

# self-test the whole factory on your machine
bash FACTORY/setup_local.sh            # (add --clone if you haven't cloned yet)
```

| You want… | Use |
|---|---|
| a blank specialist to fill | `FACTORY/specialist.template.json` |
| a blank KB content-spec to fill | `FACTORY/kb_spec.template.json` |
| an AI to generate KB + specialist for you | `FACTORY/GENERATOR_PROMPT.md` |
| the master KB generator (full power) | `schema/kb_generator_v1.4.1.txt` |
| the specialist distillation spec | `prompts/_a5_specialist_job.md` |
| real specialists to imitate | `specialists/*.json`, `branches/b60_content_intelligence/*.heavy.json` |
| a multi-specialist brain example | `branches/b60_content_intelligence/BRAIN.md` + `BRAIN_STEP1.md` + `GATE_STEP2.md` |
```
