# GRAPH_BEAUTY_PLAN.md — making the knowledge-graph view genuinely beautiful

_Planning only. The Slice 6 graph works and is HONEST (hue = node type, brightness = connectivity, negated edges distinct) but looks plain. This plans the beauty pass: one PROPOSED specialist (name + scope only — NOT built), a Codex-driven teardown of the best mind-map / knowledge-graph tools to extract their features, and a method to transfer the good ones into the app. Nothing is built by this document._

## 0. The one honest constraint (binds everything below)
Beauty must not lie. The existing honest encoding is **kept**: color still means node-type, brightness still means connectivity (not verified high-yield), null definitions still say "no definition in source", negated edges still render distinctly. "Pretty" here = **layout, interaction, motion, spacing, typography, and delight** — never decoration that implies structure the data doesn't have. Any borrowed feature that would fake certainty or hide honesty is rejected, however slick.

## 1. Proposed specialist — `graphux` (knowledge-graph interaction & aesthetic designer)
**NAME AND SCOPE ONLY — not created. Owner-gated, like the other proposed specialists were before build.**

- **Scope:** how the graph *looks and feels* as an interactive artifact — aesthetic layout tuning (force / hierarchical / radial choice for beauty + readability, node spacing, edge routing/bundling for legibility), interaction gestures (pan/zoom feel, expand-collapse, focus+context / fisheye, hover-to-highlight-neighborhood, drag), motion design (easing, staged transitions on zoom/expand, reduce-motion fallback), typography and whitespace on a dense canvas, and the competitive-teardown discipline that mines best-in-class tools for patterns.
- **Grounding fields (fields → topics → subtopics for its dense KBs):**
  1. Interaction design / HCI — focus+context (fisheye, degree-of-interest trees, overview+detail), direct manipulation, navigation gestures, expand/collapse of dense structure.
  2. Graph-drawing aesthetics — aesthetic criteria (edge-crossing minimization, symmetry, node distribution, angular resolution), edge routing & bundling, layout-family selection for legibility.
  3. Motion / animation design — easing and staging, transition choreography for zoom/expand/focus, continuity, `prefers-reduced-motion` accessibility.
  4. Visual-design craft / information aesthetics — typography in dense canvases, whitespace, visual hierarchy, restraint.
  5. Competitive UX analysis method — heuristic teardown, feature inventory, pattern extraction from tools + their manuals.
- **Boundary (crisp, so it doesn't overlap the roster):**
  - `viz` owns the **encoding grammar** (what channel carries what datum, the color/brightness system, what to show/hide/aggregate at each zoom). `graphux` **consumes viz's encoding** and decides how it *looks and moves*. On any meaning-vs-beauty conflict, **viz wins**.
  - `uxguide` owns **explanation** (hover-cloud content, the tooltip ladder, why-surfaces). `graphux` owns the *interaction* that triggers them, not their words.
  - `engage` still owns the day-box calendar visuals; `kgraph` still owns graph data/schema/terminology.
  - Honesty stops: never assert a tool's exact feature/behavior without a source (Codex extracts from the actual tool + manual); never let aesthetics override honest encoding; library/performance limits stay [UNKNOWN] until prototyped in-app.
- **Merge alternative (honest option):** if this feels too narrow, its content could instead become a **4th KB added to `viz`** rather than a standalone specialist. Recommendation: standalone, because interaction/motion/competitive-craft is a distinct skill from perceptual encoding — but the owner decides at build time.

## 2. Competitor teardown (Codex-driven — the owner logs in, Codex extracts)
Goal: mine the best tools for every feature worth stealing, sourced from hands-on use **and** their official manuals/support — no invented features.

**Tool selection — DISCOVERED, not assumed [AMENDED per owner direction]:** Codex first RESEARCHES which tools are currently the best (recent reviews, app-store ratings, PKM-community discussions), builds a ~8–10 candidate list, ranks a top 5 for our purpose (interactive knowledge graph for LEARNING: polish, navigation depth, learnability-from), and STOPS with a recommendation. The owner logs into the chosen 2–3, then Phase 2 (the deep analysis) proceeds. My prior shortlist (Heptabase/RemNote/Obsidian; XMind/MindNode; Kumu/Infranodus) stands only as a seed the research may confirm or beat.

**What to extract, per tool (the feature-inventory taxonomy):**
1. Layout — available layouts, defaults, auto-arrange behavior, spacing/whitespace feel.
2. Navigation & zoom — pan/zoom feel, semantic zoom / level-of-detail, minimap/overview, "frame/fit", breadcrumb.
3. Interaction gestures — expand/collapse, focus mode (isolate a node's neighborhood), hover-highlight, drag, multi-select, right-click menus.
4. Node & edge styling — shapes, sizing rules, icons/images on nodes, edge styles/curves/labels, grouping/cluster hulls.
5. Motion — what animates, transitions on expand/zoom/filter, speed.
6. Search / filter — find-a-node, filter by type/tag, dim-others, path-between-two-nodes.
7. Reading affordances — hover cards, side panel on select, click-through to full note, back-navigation, open-in-new-pane.
8. Delight & polish — the small touches that make it feel premium; keyboard shortcuts; theming/dark mode.
9. Export / share (note only; likely out of scope for us).

**Method (for each chosen tool) [AMENDED — analysis, not inventory]:** (a) hands-on end-to-end on a small real map, THEN the official manual / help center / shortcut reference for non-obvious features; (b) for EACH feature: what it does, HOW it behaves mechanically (timings, hop-depths, exit gestures — marked [observed] vs [doc]), and when a learner would use it; (c) UI-FLOW walkthroughs of the core journeys (first-open orientation; create-connect-arrange; find-a-node; read-deeply-and-return; overview↔detail zoom) counting clicks/keys and noting friction vs delight; (d) why-it-feels-good observations (zoom-to-cursor, inertia, easing, dense-map handling at 100+ nodes, keyboard/undo/dark-mode/reduced-motion). Concrete observations, never invented features; ends in a cross-tool synthesis: shared table-stakes patterns, unique standouts, and the ~10 patterns most worth adopting with mechanics + app-mapping sketch.

**Output:** one structured markdown inventory per tool (the taxonomy above), plus a short "what makes this one feel premium" note. These become the raw material `graphux` (once built) distills, and the transfer backlog below.

## 3. Transfer method (extracted features → app backlog)
1. **Filter each feature into keep / drop / reject:**
   - _keep_ — improves reading/navigating a **learning** knowledge graph (focus mode, neighborhood highlight, smooth semantic zoom, path-between-concepts, side-panel-on-select, minimap).
   - _drop_ — mind-map-*authoring* features we don't need (freehand drawing, presentation mode, collaborative cursors) unless a later slice wants them.
   - _reject_ — anything that fights honesty (auto-prettify that hides missing data, "importance" glow untied to a real metric, fake hierarchy).
2. **Route each kept feature to an owner:** `viz` (if it's an encoding/LOD change), `graphux` (interaction/motion/aesthetic craft), `uxguide` (explanation), or straight to the Slice 6/6.5 build if it's a small render tweak.
3. **Prioritize** keeps by impact × effort into a short backlog (the 5–8 that most change the feel come first).
4. **Ship as a Slice 6.5 (graph beauty pass)** — a follow-on to the honest foundation already built, owner-gated, with a before/after the owner eyeballs. It does not block Slice 7/8.

## 4. What NOT to copy (honesty carry-over)
- No beautifier that implies structure the data lacks; no color/brightness meaning beyond what `viz` sanctioned; no invented "definition" to fill a hover; negated edges never smoothed into positive-looking links; no performance claim ("handles 10k nodes") adopted without measuring in-app.

## 5. Next actions
1. Owner picks the 2–3 tools (log in to the ones you have).
2. Run the Codex teardown (paste prompt provided alongside this plan) → get the feature inventories.
3. Bring the inventories back → the transfer filter + backlog is authored (this is where `graphux` would be built, if approved, to do it properly).
4. Slice 6.5 builds the prioritized beauty backlog.

_Nothing is built here. `graphux` is a name + scope only; the teardown reads external tools read-only; the app is untouched until an owner-gated Slice 6.5._
