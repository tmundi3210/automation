# SLICE 1 — Explainability layer (ships early, zero dependencies)

_Repo-driven mode: obey `docs/plan/PROTOCOL.md`. Any specialist path like `FACTORY/study_system/specialists/<code>/<code>.specialist.json` in the directive block resolves HERE to `docs/plan/specialists/<code>.specialist.json`._

## SLICE 1 — Explainability layer (ships early, zero dependencies)
**Goal.** Every non-obvious label, number and code explains itself honestly at four depths. Scope: WHAT each surface must say and at which disclosure depth — not pixels, not scheduling math. The app today has zero tooltips and no guide layer of any kind [FACT — APP_RECON §5.9], so this is pure addition. No new entities; persistence of seen/dismissed hints may use one small field on the existing Settings entity or client storage (builder's choice; must survive reload).

**What to build.**

**1. The four-rung disclosure ladder.** One ladder, single-sourced: every concept is authored ONCE as a Guide-page entry; rungs 1–3 quote or deep-link into it so they can never contradict each other.
- **Rung 1 — inline caption**: a tiny always-visible line under the label. Essential operating meaning lives HERE, never behind a reveal.
- **Rung 2 — hover card ("thinking cloud")**: 2–3 sentences; accessible spec in item 4.
- **Rung 3 — "more" icon panel**: in-context expandable panel, paragraph-plus with a concrete example; stays open until closed.
- **Rung 4 — Guide page**: canonical catalog entry with a stable anchor per concept; searchable.
Every non-obvious element carries visible information scent (small "i"/"?" or dotted underline). Depth budget is proportionate: trivial controls get rung 1 only; honesty-critical numbers earn all four.

Surface inventory (which rungs, and the one honesty fact each MUST carry):

| Surface | Rungs | Non-negotiable honest content |
|---|---|---|
| Dashboard "Due Reviews" | 1–2 | cards whose review time has arrived (due ≤ now, not new-state) [FACT — APP_RECON §4b] |
| Dashboard "New Cards" | 1–3 | a per-day allowance (min of new-due count and `new_per_day`), NOT deck size; today the review queue does not actually enforce it [FACT — APP_RECON §4b, risk 7] |
| Dashboard "Today %" | 1–2 | done/planned units of today's plan; day boundary currently computed in UTC, so evening work can count toward the wrong day until Slice 0/2 fixes it [FACT — APP_RECON risk 5] |
| Dashboard "Streak" | 1–3 | days meeting the `day_done_threshold` (default 0.8); up to 2 slack tokens auto-cover missed days [FACT — APP_RECON §4b] |
| DayBox calendar fill | 1–2 | green fill = plan completion in 5% steps; a tick = done (engage specialist's wording is binding) |
| Weakness list (Dashboard + Learner Model) | 1–4 | staleness banner: "as of last analysis run <date> — runs only when you press the button" [FACT — APP_RECON §3b]; "Mark Resolved" only changes the label — no scheduling effect, and re-running today's analysis can undo it [FACT — APP_RECON §4a, risk 8] |
| E1–E4 cause label | 1–4 | plain decode of the four causes (slip / mixed-up idea / missing groundwork / faded memory); letter↔cause mapping is assumed enum order — verify before shipping [ESTIMATE — APP_RECON §4a] |
| Weakness "confidence %" | 1–4 | it is the fired rule's fixed built-in constant (0.4–0.8), a rule-strength label — NEVER worded as a computed probability [FACT — APP_RECON §4a] |
| BKT mastery bars/trend | 1–4 | "our best guess from your answers," display-only — it does not drive scheduling or card choice [FACT — APP_RECON §3d, risk 9]; trend updates only on manual runs (staleness); counts read at most 500 rows and may under-count at scale [FACT — risk 3] |
| Elo (θ / item difficulty) | 2–4 | bounded chess-rating analogy; display-only; its stated uncertainty shrinks by rule (×0.97 toward a 0.3 floor) regardless of surprises, so treat it as decoration, not certainty [FACT — APP_RECON §3a] |
| Planner "Budget share" | 1–3 | this exam's slice of the shared daily minutes, prorated by card count [FACT — APP_RECON §4b] |
| Planner "Total cards" | 1–2 | the number YOU entered, not a measured deck size [FACT — APP_RECON §4b] |
| Planner "New / day", "Last day to learn" | 1–2 | recommended pace; last day = exam date minus 14 headroom days [FACT — APP_RECON §4b] |
| Planner go / tight / no_go | 1–4 | a rough forward simulation (interval ladder), not the real FSRS engine — treat as an estimate, not a verdict [FACT — APP_RECON risk 12] |
| Review "AI suggests rating N" | 1–3 | a suggestion from an AI grader; YOU pick the final 1–4 and only your click is recorded; the AI's error tags are shown but not saved [FACT — APP_RECON §4d] |
| Settings: every field | 1–2 each | `desired_retention` alone gets 1–4: the single workload dial ("remember more ↔ review more"), warn above 0.90 (sched doctrine); fields with no code behind them (`stt_engine`, `anki_mode`/`anki_endpoint`, `burnout_threshold`, `w_source` optimized) must SAY "not active yet" [FACT — APP_RECON §5.1, §5.11] |

**2. Guide page + glossary.** New route `/guide`. Task-oriented sections answering the learner's real questions ("What do the dashboard numbers mean?", "How does scheduling decide?", "What is a weakness?"), plus an A–Z glossary. Each entry: plain definition (short active sentences, ESL-friendly), one concrete example, the honest caveat, and its stable anchor. All hover cards and panels deep-link here.
- **Glossary must cover**: FK1–FK10 (Foundation Knowledge areas; in-app short labels like "FK6 Pathology" are hardcoded UI strings [FACT — APP_RECON §4c]; official long wording, e.g. FK6 "general and disease-specific pathology", is [ESTIMATE — verify against the JCNDE Item Development Guide PDF, [METHOD] fetch-enabled session, before it is canon]); CC = Clinical Content, shipped now as the 3 sections DTP/OHM/PP [FACT — APP_RECON §4c] with the 56-item list marked "pending official source" [UNKNOWN in-repo]; BKT mastery ("best guess from your answers", not a measurement); Elo (bounded chess-rating analogy, leakage capped: no ladders, no opponents); desired retention (thermostat analogy); FSRS (the spacing engine); retrievability/stability in one line each; E1–E4 causes; the confidence constant; streak & slack tokens; new-card allowance; planned units.
- **Honest wording rules (binding on every rung)**: confidence % is a fixed rule constant, worded as rule strength [FACT — APP_RECON §4a]; E1–E4 lettering is assumed enum order — a code read binds the letters before ship [ESTIMATE]; display-only numbers are said to be display-only; pipeline-dependent state discloses manual-run staleness; precision is capped (no decimals on heuristic values); one registered term per concept — "mastery" (never "proficiency"), "review workload" (for the retention dial's effect), "rating" (never "grade"). Simplify wording, never meaning.
- **Onboarding**: no launch tour. Each empty state explains itself for a user with zero cards (Library: "add or generate your first cards"; Learner Model: "appears after your first analysis run"; trend chart: "needs 2+ run dates" [FACT — APP_RECON §4c]). Contextual one-time hints at first arrival on a feature; dismissed hints never re-nag and stay permanently re-reachable from a "Hints" section of the Guide (no onboarding cliff).

**3. WHY surfaces — faithful, provenance-carrying, never invented.** Each answers with the rule that actually fired and the fields it read. If the mechanism is ambiguous, the copy stops and routes to a code read — no plausible narrative.
- **Why this card now** (Review): "It came due <time> ago; the queue shows the 30 earliest-due cards in order" [FACT — APP_RECON §2]. Expansion: FSRS set the date from your last rating and your desired-retention setting. Contrast honestly: NOT chosen for weakness or difficulty — the queue ignores mastery, Elo, and weaknesses today [FACT — APP_RECON §3d]. New cards mix in by due order [FACT — risk 7].
- **Why this weakness** (each Weakness row): names the fired rule (of the 5) and the concrete evidence: e.g. "You picked answers tagged '<misconception_tag>' twice today" (rule 1), or "groundwork concept <name> is at <mastery> < 0.5" (rule 2), with detected-date + staleness line [FACT — APP_RECON §4a]. The fallback rule (confidence 0.4) is worded as "not enough evidence yet — a placeholder diagnosis to probe further," never as a finding. The prereq rule's "block new cards" text is advice-only today — nothing enforces it [FACT — APP_RECON §4a rule 2]; the WHY card says so until Slice 2 gating ships.
- **Why the plan changed** (DailyPlan numbers): explain the mechanism — tomorrow's planned units = cards due by tomorrow, capped at `new_per_day` (pipeline stage 4); today's number is recomputed when the Dashboard loads and increments as you review [FACT — APP_RECON §3b, §3c]. The app records WHICH writer last changed it nowhere [FACT — no audit field], so the copy lists what can change the number rather than pretending to know which did; per-change provenance arrives when Slice 2 consolidates the plan writers (handshake noted there).

**4. Hover behavior (owner-dictated) + viz handshake.**
- **Thinking cloud**: hovering a marked term pops a small definition card anchored to it; it disappears on mouse-out (owner's dictate) — except the pointer may travel INTO the card without dismissing it (hoverable). Accessibility is non-negotiable: the same card opens persistently via keyboard focus or tap (never hover-only), is dismissible with Esc/close without moving the pointer.
- **Small icon in the cloud** opens depth: the rung-3 in-context panel, which stays until closed.
- **Click-through**: a "full entry" link opens the rung-4 Guide catalog entry at its anchor, either by navigating with guaranteed back-navigation that restores the originating screen's state, or by opening in a side column where layout allows (builder picks per screen; behavior contract is identical).
- **Handshake with viz (Slice 6)**: this exact cloud → icon → catalog-with-back pattern is the shared inspection idiom for the knowledge-graph view's nodes. uxguide owns the rung copy and honesty rules; the viz specialist owns rendering/layout. This is a forward contract only, no Slice 1 work depends on it.

**Acceptance checks** (this slice's test = uxguide validation checklist, concretely):
1. Every surface in the table shows its rung-1 caption and scent; no essential meaning is hover-only.
2. Nowhere does confidence % read as a probability; grep of shipped copy for "probability/likelihood" near confidence returns zero.
3. Every pipeline-derived surface shows the staleness line; Mark Resolved discloses its no-op.
4. BKT/Elo copy contains "display-only" phrasing; no decision narrative they don't drive.
5. Tooltips pass: hoverable, dismissible, keyboard/tap reachable; dismissed hints don't re-nag and are listed in the Guide.
6. Every rung's text is verbatim-derived from its single Guide entry (spot-check 5 concepts).
7. E1–E4 letter binding and official FK wording are either verified [FACT] or shipped with a visible "wording pending verification" note — never silently asserted.

**Residual before ship**: E1–E4 letter mapping [ESTIMATE → code read]; official FK area wording + 56-CC list [UNKNOWN → JCNDE Item Development Guide PDF, fetch-enabled session].

**Owner approval gate (phase end)**: the owner reads the Guide page and hover cards and confirms tone + depth ("this is how I want the app to talk to me") and approves the FK/CC glossary wording before it is canon. No external actions in this slice — nothing is sent, spent, hosted, credentialed, or exported; the JCNDE PDF check is a read-only fetch flagged [METHOD].

```
Base44 agent directive
> Load FACTORY/study_system/specialists/uxguide/uxguide.specialist.json (paste the full JSON into your planning context). Design strictly by its role + decision_procedure; treat its escalation_triggers as stop-and-ask rules and its validation_checklist as this slice's acceptance test. Do not act outside its boundaries: it specifies what each surface says and at which disclosure rung — visual/layout engineering and algorithm changes belong to other specialists. Never render the weakness confidence constant as a probability, never invent a WHY narrative the code did not compute, and mark unverified wording (E1–E4 letters, official FK/CC text) as pending rather than asserting it.
> Consult /home/user/automation/FACTORY/study_system/specialists/dental/dental.specialist.json for FK/CC content truth and /home/user/automation/FACTORY/study_system/specialists/toefl/toefl.specialist.json for TOEFL glossary entries.
```

---
