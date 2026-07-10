# COMPETITIVE_TEARDOWN_METHOD.md — reverse-engineer the best, for every feature

_A standing, reusable method: for any feature of the app (past, present, or future), find the products that do it best, analyze HOW they do it (not just what), extract the patterns worth having, filter them for honesty + fit, and ship a "polish pass" for that feature. Generalized from `GRAPH_BEAUTY_PLAN.md` (the graph teardown) so it applies everywhere. **Planning/method only — this document builds nothing. It runs as a BACKGROUND track (Codex, free, owner-paced) that FEEDS the build; it never blocks a slice.**_

## 0. The rule that binds every teardown
- **Analyze, don't steal.** The output is understanding — how a feature behaves mechanically, why the flow feels good — turned into our own design, not copied assets or code.
- **Honesty survives.** No borrowed pattern may fake data, imply structure we don't have, or override an honest label. Anything that does is **rejected**, however slick (the graph's rule: brightness = connectivity, never faked "high-yield").
- **No invented facts.** Every competitor feature is verified hands-on or in that product's own docs, tagged `[observed]` vs `[doc]`; never asserted from memory. Product names below are **seeds** — Codex confirms, replaces, or beats them by research.
- **Background, not blocking.** A teardown is never the main thread. The build proceeds on the plan; teardowns arrive as polish backlogs (`Slice N.5`) the owner schedules when convenient. (This is the correction from earlier — testing/analysis threads must not stall the mainline.)

## 1. The five-step method (same every time)
1. **DISCOVER** — Codex researches which products are currently best at THIS feature (recent reviews, ratings, practitioner/community discussion), lists ~8–10 candidates (one line each: what, famous-for, price, needs-account), ranks the top 5 *for our specific purpose*, and **stops** with a recommendation.
2. **PAUSE FOR OWNER** — owner logs into the recommended 2–3 (only ones they can access), says proceed.
3. **ANALYZE (deep)** — per chosen product: (a) feature census, hands-on + docs, each with what/how-it-mechanically-behaves/when-a-user-needs-it/where-verified `[observed|doc]`; (b) UI-flow walkthroughs of that feature's core journeys, counting clicks/keys, noting friction vs delight; (c) "why it feels good" concretely (timing, physics, spacing, edge cases, accessibility); (d) a 5-line verdict.
4. **SYNTHESIZE** — across products: shared table-stakes patterns (must-haves), unique standouts, and the ~10 patterns most worth adopting with mechanics + a one-line sketch of how each maps onto our app.
5. **FILTER + BACKLOG** — run each pattern through **keep / drop / reject** (keep = improves OUR purpose; drop = not needed now; reject = fights honesty), route each keep to its owning specialist, prioritize by impact×effort, and ship as a `Slice N.5` polish pass with a before/after the owner eyeballs.

## 2. Per-feature competitor SEED map (Codex confirms/replaces by research)
_These are starting suggestions only — [SEED], not verified feature claims. The DISCOVER step may confirm, drop, or beat any of them._

| Feature / Slice | Our purpose to judge against | Seed products to research |
|---|---|---|
| **Review experience** (cross-cutting: the card review screen, rating, flow) | fast, low-friction recall practice; keyboard-first | Anki, Quizlet, INBDE/Dental Bootcamp, UWorld, RemNote |
| **S1 Explainability** (tooltips, guide, onboarding) | teach a non-expert what a number means without a manual | Linear, Stripe (docs/tooltips), Duolingo (onboarding), Notion help, Amboss |
| **S2 Planner / calendar / gating** (day view, next-day plan, prerequisite gating) | honest adaptive daily plan; don't advance on shaky ground | Anki/FSRS, Duolingo (skill path + gating), Amboss/UWorld study planners, Super/Traverse |
| **S3 Reminders / integrations UX** (calendar, Anki push settings) | clear, trustworthy connect-and-forget | Fantastical/Google Calendar, Todoist, Readwise (sync UX) |
| **S5 Model picker** (choose 2 models, token visibility) | pick a model + see cost without confusion | OpenRouter, LM Studio, Cursor/Continue model pickers, Ollama UIs |
| **S6 Knowledge graph** (DONE — the graph teardown) | interactive learning graph | Heptabase, RemNote, Obsidian, XMind/MindNode, Kumu _(already in GRAPH_BEAUTY_PLAN)_ |
| **S7 Media / 3D flashcards** (image-occlusion, 3D anatomy, voice notes) | media that aids learning, rotatable 3D that helps not decorates | Anki image-occlusion, Complete Anatomy, BioDigital, Kenhub, Sketchfab/model-viewer galleries |
| **S8 Managing-agent dashboard** (nightly loop status, approvals, "why") | trustable at-a-glance system status + honest control | Linear/Height dashboards, agent-run UIs, observability dashboards (Grafana-style) |
| **Gamification / progress** (day-box, streak — engage's turf) | motivating without dishonest pressure | Duolingo, Brilliant, Habitica, Streaks |
| **Infra-only slices (S0)** | — | **No teardown** — not user-facing; honesty note: skip, don't manufacture one |

## 3. Applying it to PAST vs FUTURE slices
- **Past/built slices (1,2,3,5,6):** each gets a **retro-teardown** on the owner's schedule → a `Slice N.5` polish backlog. Priority order by owner pain: the owner already flagged the **graph (S6.5)** first; likely next by visibility are the **Review experience** (most-used screen) and **S1 explainability**.
- **Future slices (7,8):** run the DISCOVER+ANALYZE **before or alongside** the build so the build already reflects best-in-class, instead of building plain then polishing. For S7 especially (media/3D), the teardown should precede build — 3D-anatomy UX is a solved problem elsewhere and worth learning first.
- **Proposed specialists as the "how":** the teardown's synthesis is the raw material a domain specialist distills. Graph → the proposed `graphux`. If a feature area's polish proves deep enough (e.g. review UX), a similarly-scoped specialist may be proposed — name only, owner-gated, never auto-built.

## 4. The reusable Codex prompt (fill the one blank)
A single template drives every teardown — the owner sets `<FEATURE>` and `<OUR PURPOSE>`; the rest is fixed (the full two-phase prompt lives in the chat that accompanies this plan and mirrors §1). Phase 1 discovers+ranks+stops; Phase 2 (after owner login) does the deep analysis + cross-tool synthesis; output is one markdown per tool + one synthesis file, saved to a folder.

## 5. What this is NOT
- Not a mandate to copy UIs or assets. Not a reason to delay a build (teardowns are background). Not license to assert a competitor does X without verifying it. Not applied to non-user-facing infra. Not self-executing — every resulting `Slice N.5` is owner-gated like any slice.

_Nothing is built by this document. It defines a repeatable background practice; each teardown produces a plan (a polish backlog), and each backlog becomes an owner-gated slice only when the owner says so._
