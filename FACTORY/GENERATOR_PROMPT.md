# GENERATOR_PROMPT — paste this into any capable AI to mint a new specialist

This produces, in this repo's exact schema, a new **specialist** plus the **3 forge-ready KB
specs** it stands on. The AI writes the content; **you** run the local forge + gates (which
compute all formulas deterministically), so nothing depends on the model doing math.

**How to use:**
1. Copy everything in the fenced block below into a strong model (Claude, etc.).
2. Replace the `IDEA:` / `AUDIENCE:` lines at the top with your topic.
3. Save each `=== FILE: name ===` block the AI returns to that filename.
4. Run the forge + gates locally (commands are inside the prompt and in §10 of
   `FACTORY/MAKE_A_SPECIALIST.md`).

Source files this prompt mirrors (open them to verify the AI followed the schema):
- Build manual: `FACTORY/MAKE_A_SPECIALIST.md`
  https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/MAKE_A_SPECIALIST.md
- KB spec template: `FACTORY/kb_spec.template.json`
  https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/kb_spec.template.json
- Specialist template: `FACTORY/specialist.template.json`
  https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/FACTORY/specialist.template.json
- The deterministic forge: `branches/_forge/kb_forge.py`
  https://github.com/tmundi3210/automation/blob/claude/eager-wozniak-74rlgj/branches/_forge/kb_forge.py
- The gates: `validators/kb_validator.py`, `validators/specialist_validator.py`

---

```
# ============================ PASTE FROM HERE ============================
# SOURCE: github.com/tmundi3210/automation — FACTORY/GENERATOR_PROMPT.md
# You are building one EXPERT SPECIALIST and the 3 dense knowledge bases (KBs) it
# stands on, in a fixed JSON schema. A "specialist" is NOT a trained model — it is a
# JSON operating spec that a model later loads to reason AS that expert. Be dense,
# concrete, and machine-facing. Output JSON only inside the marked file blocks.

IDEA: <replace with your topic — e.g. "spotting under-served regional film markets">
AUDIENCE: <replace — or write "general" if none>

# ---- WHAT TO PRODUCE -------------------------------------------------------------
# Pick 3 distinct SUBDOMAINS of the IDEA (the 3 KBs). For EACH subdomain, output one
# forge-ready compact KB spec. Then output ONE specialist spec grounded in all three.
# Emit exactly these four blocks, each as STRICT JSON (first char {, last char }):
#
#   === FILE: kb1.spec.json ===   { ...compact KB spec for subdomain 1... }
#   === FILE: kb2.spec.json ===   { ...compact KB spec for subdomain 2... }
#   === FILE: kb3.spec.json ===   { ...compact KB spec for subdomain 3... }
#   === FILE: my.specialist.json ===  { ...specialist spec grounded in the 3 KBs... }

# ---- KB SPEC SHAPE (per file; the local forge computes every derived score) -------
# Top-level keys: domain, domain_label, purpose, assumptions[], exclusions[],
#   competency_questions[], glossary[], nodes[], edges[], conflict_axes[],
#   edge_cases[], workflow[], dominance_rules[], anti_rework_rules[],
#   iteration_protocol[], priority_rationale, eval_objective.
#
# COUNT BANDS (must land in-band or the dense gate fails):
#   nodes 19-24 | edges 32-40 | conflict_axes 8-10 | edge_cases 10-12 |
#   workflow 9-12 | competency_questions 10-14 | dominance_rules 7-12 |
#   anti_rework_rules 7-12 | iteration_protocol 6-10.
#
# Each competency_question: { "id":"CQ_01", "question":"...", "acceptance":"..." }
#
# Each node:
#   { "id":"UPPER_SNAKE_UNIQUE", "topic":"snake_topic", "definition":"dense paragraph",
#     "group":"foundations|core|...", "dependencies":[node ids, ACYCLIC],
#     "competency_question_refs":["CQ_0x"],
#     "scope_boundary":{"included":[...],"excluded":[...]},
#     "pros":[{"claim":"...","example":"..."}], "cons":[{"claim":"...","example":"..."}],
#     "failure_modes":["..."], "acceptance_tests":[...], "revisit_triggers":[...],
#     "base":{ all 15 metrics below, each a number in [0,1] }, }
#   base metrics: criticality, business_value, user_value, technical_complexity,
#     risk_if_wrong, cross_topic_coupling, irreversibility, confidence,
#     node_conflict_pressure, acceptance_test_pass_rate, dependency_gate_pass_rate,
#     prior_importance, evidence_confidence, failure_rate, downside_weight,
#     plus "uncertainty_interval":[lo,hi]  and  "update_signal":"...".
#   RULES the forge enforces:
#     - if risk_if_wrong >= 0.80  => acceptance_tests MUST be non-empty.
#     - if cross_topic_coupling >= 0.75 => revisit_triggers MUST be non-empty.
#
# Each edge: { "from":nodeid, "to":nodeid,
#   "edge_type":"dependency|constraint|conflict|causal|sequence|feedback|similarity",
#   "why_related":"...", "relation_strength":0.0-1.0 }
#   - a "conflict" edge MUST also supply "resolution_rule":"...".
#   - every from/to must be a real node id (the forge checks this); dependency edges must
#     not form a cycle (the dense gate checks this — keep the graph acyclic).

# ---- SPECIALIST SPEC SHAPE (15 required keys) ------------------------------------
#   { "_directive":"machine-facing specialist operating spec distilled from 3 dense KBs",
#     "specialist_id":"short_code", "domain":"slug", "domain_label":"Human Name",
#     "purpose":"one precise what+when sentence",
#     "grounded_in_kbs":["kb1.kb.json","kb2.kb.json","kb3.kb.json"],  # the FORGED outputs
#     "role":"dense system-prompt-style role; scope + invariants + what it must NEVER do",
#     "capabilities":[ {"capability":"...","subdomain":"which KB","method":"...",
#                       "when_to_use":"...","inputs":[...],"outputs":[...]} ],  # cover all 3 KBs
#     "decision_procedure":["ordered steps"], "workflow":[{"phase","objective","entry","exit","gates"}],
#     "escalation_triggers":[{"condition","action":"human_review|defer|flag","reason"}],
#     "validation_checklist":["checkable conditions"],
#     "conflicts_and_dominance":[{"tradeoff","resolution"}],
#     "glossary":[{"term","definition"}],
#     "competency_questions_covered":["questions it can answer"] }
#   - _directive MUST contain the word "machine".
#   - capabilities, decision_procedure, workflow, escalation_triggers,
#     validation_checklist, competency_questions_covered, grounded_in_kbs: all NON-EMPTY.
#   - NEVER output the literal strings TODO, placeholder, <...>, NODE_A, SHORT_ID — the
#     gate rejects them. Replace every field with real content.

# ---- THEN TELL THE USER TO RUN (locally; pure stdlib, no install) ----------------
#   for k in kb1 kb2 kb3; do
#     python3 branches/_forge/kb_forge.py $k.spec.json -o $k.kb.json
#     python3 validators/kb_validator.py $k.kb.json --mode dense   # must exit 0
#   done
#   python3 validators/specialist_validator.py my.specialist.json  # must exit 0
# (Set grounded_in_kbs to the kbN.kb.json paths that exist after forging.)
# Output the four JSON file blocks now. JSON only inside them. No prose inside the blocks.
# ============================= PASTE TO HERE =============================
```

> Why it outputs *forge-ready specs* (not finished KBs): the dense gate checks that every
> risk/priority score recomputes exactly. Letting the model guess those numbers fails the
> gate. Instead the model writes content + base magnitudes, and `kb_forge.py` computes the
> formulas deterministically — so the result passes "by construction." If you'd rather have
> the model emit a *finished* KB directly, use the full master generator
> `schema/kb_generator_v1.4.1.txt` instead and gate its output the same way.
