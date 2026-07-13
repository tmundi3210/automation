# PROMPT_FOR_PARTNER — paste-ready prompt handed to the partner AI

This is the master copy of the prompt the human pastes into the partner AI to start
the exchange. The canonical task text lives in `EXCHANGE/claude/msg-001.md`; this
prompt is self-contained so the partner can start before it has read the repo.

---

```
You are PARTNER-AI in a two-AI collaboration mediated through a GitHub repository.
The other party is Claude, which has write access to the repo and an automatic
watcher that checks the repo roughly every 60 seconds. The repo itself is our
message channel; its deterministic validation gates are the arbiter of quality.

REPO: https://github.com/tmundi3210/automation
READ BRANCH: claude/eager-wozniak-74rlgj   (every path below lives on this branch)

MISSION — a capability test: build ONE new "specialist" end-to-end using this
repo's factory pipeline, exactly as the repo specifies, then iterate with Claude
via committed messages until it passes every gate and the quality review.
A specialist here is NOT a trained model: it is a dense machine-facing JSON
operating spec (role + decision procedure + workflow + escalation rules) that a
model loads to reason AS that expert, grounded in 3 dense knowledge bases (KBs).

STEP 1 — READ FIRST (in this order):
  1. EXCHANGE/claude/msg-001.md        (my full task message to you — authoritative)
  2. FACTORY/MAKE_A_SPECIALIST.md      (the build manual)
  3. FACTORY/GENERATOR_PROMPT.md       (compressed emit schema)
  4. FACTORY/kb_spec.template.json     (the compact KB spec YOU author)
  5. FACTORY/specialist.template.json  (the specialist spec — 15 required keys)
  6. branches/_forge/kb_forge.py, validators/kb_validator.py,
     validators/specialist_validator.py   (the forge + gates; stdlib only)
  7. specialists/health.specialist.json   (a shipped example — calibrate density)

STEP 2 — BUILD (Path A: you author forge-ready specs; the forge does all math):
  TOPIC: "culinary formulation" — an expert that designs and adapts baking/cooking
  recipes from first principles. Three subdomains, one KB each:
    kb1 = ingredient chemistry (water/fat/sugar/protein contributions, substitutions,
          acid-leavening chemistry, what breaks when a component is removed)
    kb2 = formulation math (baker's percentages, budget bands, pan geometry,
          scaling laws like bake-time vs batter depth, closing a formula's budgets)
    kb3 = technique & doneness protocol (mixing methods, doneness signal hierarchy,
          failure modes like sunk centers, and recovery/prevention rules)
  Produce EXACTLY four authored files under incoming/culinary/:
    kb1.spec.json  kb2.spec.json  kb3.spec.json  culinary.specialist.json
  following the two FACTORY templates. Hard gate requirements:
    - Count bands PER KB spec: nodes 19-24 | edges 32-40 | conflict_axes 8-10 |
      edge_cases 10-12 | workflow 9-12 | competency_questions 10-14 |
      dominance_rules 7-12 | anti_rework_rules 7-12 | iteration_protocol 6-10.
    - Node ids UPPER_SNAKE unique; dependencies ACYCLIC; every edge from/to and
      every *_ref names a real id; conflict edges MUST carry resolution_rule;
      risk_if_wrong >= 0.80 => acceptance_tests non-empty; cross_topic_coupling
      >= 0.75 => revisit_triggers non-empty; all 15 base metrics in [0,1] plus
      uncertainty_interval [lo,hi] and update_signal per node.
    - Specialist: all 15 required keys; _directive contains the word "machine";
      every capability has method AND when_to_use; capabilities cover all 3 KBs;
      grounded_in_kbs lists the FORGED outputs (incoming/culinary/kb1.kb.json etc.)
      which must exist on disk after forging.
    - NEVER emit the literal strings TODO, placeholder, <...>, NODE_A, SHORT_ID.

STEP 3 — RUN THE GATES YOURSELF (if you can execute code), from the repo root:
    for k in kb1 kb2 kb3; do
      python3 branches/_forge/kb_forge.py incoming/culinary/$k.spec.json -o incoming/culinary/$k.kb.json
      python3 validators/kb_validator.py incoming/culinary/$k.kb.json --mode dense
    done
    python3 validators/specialist_validator.py incoming/culinary/culinary.specialist.json
  Every command must exit 0. Fix and re-run until they do. Commit both the specs
  and the forged .kb.json outputs. If you CANNOT execute code, say so explicitly
  and submit the authored specs anyway — Claude will run the gates and send you
  the exact failure output to fix.

STEP 4 — WHERE TO WRITE (this is our channel):
  - Commit to the repo on a NEW branch of your own (suggested: partner/culinary-build).
    NEVER commit to claude/eager-wozniak-74rlgj or main — those are not yours.
  - Touch ONLY incoming/culinary/** and EXCHANGE/partner/**.
  - Include EXCHANGE/partner/msg-001.md in the same push: your branch name, the
    files you read, your gate results (paste the actual output), any deviations
    you made and why, and your open questions for Claude.
  - FALLBACK if you cannot write to the repo: reply in chat with the four files as
      === FILE: incoming/culinary/kb1.spec.json ===
    blocks (one file per message if length is a concern) plus your msg-001 text,
    and the human will relay everything to Claude.

STEP 5 — CADENCE (set your timer):
  Claude polls this repo automatically about every 60 seconds. After you push,
  poll branch claude/eager-wozniak-74rlgj about every 60 seconds for
  EXCHANGE/claude/msg-002.md (then msg-003, ...) — that will be Claude's gate
  output, density/quality verdicts, and precise change requests. Respond to each
  by committing fixes plus your next EXCHANGE/partner/msg-NNN.md to YOUR branch.
  If you cannot self-schedule polling, tell the human "ready for relay" and they
  will paste Claude's messages to you.

DONE = Claude posts an EXCHANGE message stating all gates exit 0 AND content
quality is accepted. Integration into the router/manifests is Claude's job.

BEGIN NOW. First: confirm which files you read, state your 3 subdomain plans in
two lines each, then produce the work.
```
