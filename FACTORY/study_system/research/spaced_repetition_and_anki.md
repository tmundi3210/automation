# Spaced Repetition Math + Anki Integration Surface

Machine-facing reference for the scheduler/timeline specialist. Honesty tags: [FACT] = fetched source or true by definition (URL inline); [ESTIMATE] = inference with stated basis; [UNKNOWN] = unverifiable, blocker named.

**Sourcing note (applies throughout):** this session's egress proxy only allowed `github.com`, `raw.githubusercontent.com`, and `pypi.org`. `supermemo.com`/`super-memory.com`, `docs.ankiweb.net`, `faqs.ankiweb.net`, `foosoft.net`, `git.sr.ht`, and Wikipedia/journals were all blocked (CONNECT 403). Everything below is therefore sourced from the **official source repositories** that build those sites (verified: `ankitects/faqs` contains `CNAME = faqs.ankiweb.net`; `ankitects/anki-manual` is the docs.ankiweb.net source), from Anki's own source code, or from verbatim mirrors — each tagged.

---

## 1. SM-2 (Piotr Wozniak, SuperMemo, 1987 method / published on supermemo.com)

Canonical URL: `http://www.supermemo.com/english/ol/sm2.htm` — this is the URL Anki's own FAQ links to as "the SuperMemo 2 algorithm" [FACT — fetched FAQ source: https://github.com/ankitects/faqs/blob/main/src/what-spaced-repetition-algorithm.md]. [UNKNOWN: the supermemo.com page itself was unreachable (egress policy 403); text below is from a verbatim mirror — https://raw.githubusercontent.com/CherokeeLanguage/BoundPronouns/2e621baaaddc375ae09f6c2fea238919d38b1325/docs/sm2-notes.txt — whose formula text matches 26 independent GitHub mirrors found via code search.]

Algorithm as published [FACT — mirror above, quoted near-verbatim]:

1. Split knowledge into smallest possible items.
2. Associate with every item an **E-Factor (EF) = 2.5** initially.
3. Intervals:
   - `I(1) := 1` day
   - `I(2) := 6` days
   - `for n > 2: I(n) := I(n-1) * EF`
   - fractional intervals round **up** to nearest integer.
4. After each repetition, grade response quality `q` on **0–5**:
   | q | meaning |
   |---|---------|
   | 5 | perfect response |
   | 4 | correct response after a hesitation |
   | 3 | correct response recalled with serious difficulty |
   | 2 | incorrect response; correct one seemed easy to recall |
   | 1 | incorrect response; the correct one remembered |
   | 0 | complete blackout |
5. EF update after each repetition:
   ```
   EF' := EF + (0.1 - (5-q) * (0.08 + (5-q) * 0.02))
   if EF' < 1.3 then EF' := 1.3
   ```
6. If `q < 3`: restart repetitions from the beginning (`I(1)`, `I(2)`, …) **without changing EF**.
7. At the end of each day's session, re-repeat all items that scored `< 4` until every one scores `>= 4`.

EF deltas per grade (arithmetic from the formula, [FACT by definition]): q=5 → +0.10; q=4 → 0.00; q=3 → −0.14; q=2 → −0.32; q=1 → −0.54; q=0 → −0.80.

Key structural properties [FACT by definition]: EF is per-item; interval growth is geometric in EF; failure resets the interval ladder but grading a failure still lowers EF (step 5 runs on every repetition, step 6 only resets position); EF floor 1.3 caps minimum growth at 1.3×.

---

## 2. How classic Anki (SM-2 variant) deviates from SM-2

Source of record: Anki FAQ "What spaced repetition algorithm does Anki use?" [FACT — fetched source repo of faqs.ankiweb.net: https://github.com/ankitects/faqs/blob/main/src/what-spaced-repetition-algorithm.md], Anki manual deck-options page [FACT — https://github.com/ankitects/anki-manual/blob/main/src/deck-options.md, source of docs.ankiweb.net/deck-options.html], and Anki scheduler source [FACT — https://github.com/ankitects/anki, `rslib/src/scheduler/states/`].

Deviations (all [FACT] from the FAQ page unless noted):

| # | Deviation | Detail |
|---|-----------|--------|
| 1 | Learning steps replace SM-2's fixed 1d/6d | User-configurable steps (default `1m 10m`); a card "graduates" after the last step. Failures **during learning do not touch ease** ("the initial acquisition process does not influence a card's ease" — avoids "low interval hell"). |
| 2 | 4 answer buttons, not 6 grades | Again / Hard / Good / Easy; only one fail grade. |
| 3 | Ease deltas as constants | Again −0.20, Hard −0.15, Good 0, Easy +0.15 (FAQ: "percentage points"; exact constants in source: `EASE_FACTOR_AGAIN_DELTA = -0.2`, `EASE_FACTOR_HARD_DELTA = -0.15`, `EASE_FACTOR_EASY_DELTA = 0.15` — [FACT https://github.com/ankitects/anki/blob/main/rslib/src/scheduler/states/review.rs lines 16–20]). |
| 4 | Ease floor 1.3 | `MINIMUM_EASE_FACTOR: f32 = 1.3`, initial 2.5 (`INITIAL_EASE_FACTOR`) [FACT — review.rs lines 16–17]. FAQ: "Eases will never be decreased below 130%; SuperMemo's research has shown that eases below 130% tend to result in cards becoming due more often than is useful". |
| 5 | Lapse handling | Again on a review card → relearning steps; interval multiplied by **New Interval** (default 0.00 = full reset; e.g. 0.20 keeps 20% of a 100d interval) [FACT — deck-options.md "New Interval"]. Repeated lapses can flag a card as a **leech** (suspend/tag) [FACT — deck-options.md "Leeches"]. |
| 6 | Late answers get credit | "Answering cards later than scheduled will be factored into the next interval calculation" [FACT — FAQ]. In source: Good uses `(current_interval + days_late/2) * ease_factor`; Easy uses `(current_interval + days_late) * ease_factor * easy_multiplier` [FACT — review.rs lines 241–248]. |
| 7 | Extra multipliers | Hard Interval ×1.2 (of previous interval), Easy Bonus ×1.30, global Interval Modifier ×1.00 default, Maximum Interval cap; all per-preset [FACT — deck-options.md]. New intervals (except Again) always ≥ previous interval + 1 day [FACT — FAQ "Limitations"]. |
| 8 | Fuzz | Random fuzz applied on answering "to prevent cards that were introduced at the same time and given the same ratings from sticking together"; learning cards get up to 5 min extra delay; **cannot be disabled** [FACT — https://github.com/ankitects/anki-manual/blob/main/src/studying.md "Fuzz Factor"]. Exact ranges in source: fuzz delta = 1 day + 0.15×(days in 2.5–7) + 0.10×(days in 7–20) + 0.05×(days > 20); short intervals not fuzzed [FACT — https://github.com/ankitects/anki/blob/main/rslib/src/scheduler/states/fuzz.rs `FUZZ_RANGES`]. |

Interval-modifier ↔ retention identity used by classic Anki tuning: `new modifier = log(desired retention%) / log(current retention%)`; manual example: raising 85% → 90% gives modifier 0.65, i.e. ~35% more frequent study for +5 points of retention — the trade-off is non-linear [FACT — deck-options.md "Interval Modifier"].

---

## 3. FSRS — the open-source scheduler Anki ships

- What it is: FSRS (Free Spaced Repetition Scheduler), a DSR-model scheduler descending from MaiMemo's DHP model, itself a variant of Wozniak's three-component (DSR) memory model [FACT — https://raw.githubusercontent.com/open-spaced-repetition/fsrs-rs/main/README.md].
- Shipped in Anki since 23.10 as the second built-in algorithm [FACT — FAQ page above: "As of Anki 23.10, Anki has two available algorithms"]. Anki's Rust core depends on the `fsrs` crate (`fsrs.workspace = true` in rslib/Cargo.toml) [FACT — https://raw.githubusercontent.com/ankitects/anki/main/rslib/Cargo.toml].
- Repos and licenses:
  - `open-spaced-repetition/fsrs4anki` (helper add-on + wiki) — **MIT** [FACT — https://raw.githubusercontent.com/open-spaced-repetition/fsrs4anki/main/LICENSE].
  - `open-spaced-repetition/fsrs-rs` (Rust impl Anki uses; full training via the Burn framework, simulation, scheduling; crate `fsrs`) — **BSD-3-Clause** [FACT — https://raw.githubusercontent.com/open-spaced-repetition/fsrs-rs/main/LICENSE].
  - `open-spaced-repetition/py-fsrs` — PyPI package `fsrs`, v6.3.1 as of 2026-07-02, MIT [FACT — https://pypi.org/pypi/fsrs/json].

### DSR model [FACT — FAQ page above + algorithm wiki]

- **Retrievability R**: probability of recall now; decays with time since last review; the only component that changes without a review.
- **Stability S**: days for R to fall from 100% to 90% (S = interval at which R = 90%).
- **Difficulty D** ∈ [1, 10]: how hard it is to increase S by reviewing.
- Grades G: 1=again, 2=hard, 3=good, 4=easy. Each card carries its own (D, S) "memory state"; R is computed from S and elapsed time.

### Formulas (FSRS-6, current; 21 parameters w0..w20) [FACT — official algorithm wiki, fetched from the wiki git repo: https://github.com/open-spaced-repetition/awesome-fsrs/wiki/The-Algorithm (moved from https://github.com/open-spaced-repetition/fsrs4anki/wiki/The-Algorithm)]

- **Power forgetting curve** (decay is trainable): `R(t,S) = (1 + factor * t/S)^(-w20)` with `factor = 0.9^(-1/w20) − 1` so that `R(S,S) = 0.9`.
- **Next interval** = inverse of the curve at desired retention r: `I(r,S) = (S/factor) * (r^(-1/w20) − 1)`; `I(0.9, S) = S`. (FSRS-4.5/5 fixed DECAY=−0.5, FACTOR=19/81; FSRS v4 used DECAY=−1, FACTOR=1/9.)
- **Initial state after first rating**: `S0(G) = w[G−1]` (w0..w3 = initial stability for Again/Hard/Good/Easy); `D0(G) = w4 − e^(w5·(G−1)) + 1`, with D0(1)=w4.
- **Difficulty update** (linear damping + mean reversion toward D0(4), against "ease hell"): `ΔD = −w6·(G−3)`; `D' = D + ΔD·(10−D)/9`; `D'' = w7·D0(4) + (1−w7)·D'`; clamped to [1,10].
- **Stability after success** (G ≥ 2): `S'_r = S · (e^{w8} · (11−D) · S^{−w9} · (e^{w10·(1−R)} − 1) · w15[if G=2] · w16[if G=4] + 1)`. Properties: SInc = S'/S ≥ 1; SInc shrinks with higher D and higher S; SInc grows as R drops (the spacing effect); for overdue-but-recalled cards the stability gain converges to an upper bound rather than growing linearly as in SM-2/Anki.
- **Stability after lapse** (G = 1): `S'_f = w11 · D^{−w12} · ((S+1)^{w13} − 1) · e^{w14·(1−R)}` (min with a same-day-derived floor in implementations).
- **Same-day (short-term) review**: `S'(S,G) = S · e^{w17·(G−3+w18)} · S^{−w19}`.
- **FSRS-6 default parameters** [FACT — same wiki page]: `[0.212, 1.2931, 2.3065, 8.2956, 6.4133, 0.8334, 3.0194, 0.001, 1.8722, 0.1666, 0.796, 1.4835, 0.0614, 0.2629, 1.6483, 0.6014, 1.8729, 0.5425, 0.0912, 0.0658, 0.1542]`.

### Per-user parameter optimization [FACT — https://github.com/open-spaced-repetition/awesome-fsrs/wiki/The-mechanism-of-optimization + anki-manual deck-options.md]

- Optimizer fits the w-vector to the user's own `revlog` (review history): revlogs grouped per card → time-series of (rating, interval) pairs → model trained with **maximum likelihood / binary log-loss** on recall-vs-forget outcomes, via backpropagation through time (reference optimizer in PyTorch; Anki's built-in trainer in `fsrs-rs` uses Burn).
- In Anki this is the deck-options **"Optimize"** button (and "Optimize All Presets"). Parameters and desired retention are **per-preset**; manual says users should not hand-edit or copy parameters, once-a-month reoptimization is sufficient, and an "Ignore cards reviewed before" date can exclude old habits/imported logs [FACT — deck-options.md "FSRS parameters"].

---

## 4. Empirical basis (spacing effect + testing effect)

Verified quantitative findings (source actually fetched):

- **Spacing effect, classroom materials**: Rea & Modigliani 1985 — distributed practice 70% correct vs 53% massed, immediately post-training [FACT — quoted in the FSRS author's tutorial, fetched: https://github.com/open-spaced-repetition/awesome-fsrs/wiki/Spaced-Repetition-Algorithm:-A-Three%E2%80%90Day-Journey-from-Novice-to-Expert, which cites gwern.net/doc/psychology/spaced-repetition/1985-rea.pdf].
- **Distribution-of-practice meta-analysis**: Donovan & Radosevich 1999 — mean weighted effect size 0.46 (95% CI 0.42–0.50) favoring spaced over massed practice [FACT — same fetched page].

Standard citations (bibliographic identity cross-checked in multiple independently fetched bibliographies via GitHub code search; **primary texts not fetched** — journals/Wikipedia blocked by egress policy):

- **Ebbinghaus, H. (1885/1913). *Memory: A Contribution to Experimental Psychology*** — origin of the forgetting curve and savings method; self-experiments with nonsense syllables. [FACT for the bibliographic reference (string matched in fetched bibliographies); ESTIMATE (training knowledge, high confidence) for the content description; UNKNOWN for exact figures — primary text unreachable.]
- **Cepeda, N. J., Pashler, H., Vul, E., Wixted, J. T., & Rohrer, D. (2006). "Distributed practice in verbal recall tasks: A review and quantitative synthesis." *Psychological Bulletin*, 132(3), 354–380** — the modern meta-analysis: spaced beats massed across hundreds of comparisons and optimal gap scales with retention interval. [FACT for the reference string; ESTIMATE (training, high confidence) for the one-line finding; UNKNOWN for its effect sizes — paywalled/blocked.]
- **Roediger, H. L., & Karpicke, J. D. (2006). "Test-enhanced learning: Taking memory tests improves long-term retention." *Psychological Science*, 17(3), 249–255** (companion review: "The power of testing memory," *Perspectives on Psychological Science*, 1(3), 181–210) — the testing effect: retrieval practice beats restudy at delayed retention even when restudy wins immediately. [FACT for reference strings; ESTIMATE (training, high confidence) for the finding; UNKNOWN for exact percentages.]
- Design consequence [FACT by definition given the above]: flashcard review is simultaneously a *test* (testing effect) and a *spaced exposure* (spacing effect); the FSRS success-stability formula encodes the spacing effect directly (SInc grows as R falls — wiki The-Algorithm, point 3).

---

## 5. Integration surfaces for an external system

### 5.1 File formats: `.apkg` / `.colpkg` [FACT — https://github.com/ankitects/anki-manual/blob/main/src/exporting.md]

- `.colpkg` = "Anki Collection Package": entire collection, all decks, **scheduling included**; importing replaces the user's whole collection.
- `.apkg` = "Anki Deck Package": one deck (+media); "Include Scheduling Information" optional — if off, review history/scheduling stripped. A file literally named `collection.apkg` is treated as a collection package.
- Inside the package: a SQLite database (`.anki2` naming per AnkiDroid wiki) plus media, zipped [FACT — https://github.com/ankidroid/Anki-Android/wiki/Database-Structure: "Anki uses a single SQLite database … found inside the Anki package file (the .apkg file) with the extension .anki2"]. [ESTIMATE, medium-high: newer exports also use `collection.anki21`/`collection.anki21b` (zstd) variants — seen in ecosystem code but not verified against an official spec page this session.]

### 5.2 Database schema basics [FACT — AnkiDroid wiki page above, which reproduces the CREATE TABLEs]

- `notes`: raw facts; `flds` = field contents; `mid` = model (note type) id; `guid` for dedup/sync.
- `cards`: one per (note, template); `ivl` = interval in days; `factor` = ease in **permille** (2500 = 2.5×); `due`, `queue`, `type`, `lapses`, `reps`; `odue` for filtered decks.
- `col`: single row; collection conf + (legacy) models/decks JSON.
- `revlog`: **one row per review ever** — `(id epoch-ms of review, cid, usn, ease, ivl, lastIvl, factor, time, type)`; `ease` = button pressed (review: 1=wrong, 2=hard, 3=ok, 4=easy); `time` = review duration ms capped at 60000; `type` 0=learn 1=review 2=relearn 3=cram; when FSRS is enabled, `factor` is reused as FSRS difficulty normalized to 100–1100. This table is exactly what the FSRS optimizer consumes.
- `graves`: deletions pending sync.

### 5.3 AnkiConnect (HTTP API into a running Anki)

Add-on code **2055492159**; local HTTP server on **port 8765**, bound to 127.0.0.1 by default; request = JSON `{action, version: 6, params, key?}`; response = `{result, error}` [FACT — AnkiConnect README; canonical repo moved to git.sr.ht/~foosoft/anki-connect (blocked); verbatim pre-move mirror fetched: https://raw.githubusercontent.com/johan456789/anki-connect-mirror/e437f892ff6a44b0da2e8610005503d6d1c57068/README.md]. Actions most relevant to an external study system (all [FACT — same README]):

| Purpose | Actions |
|---|---|
| Decks | `deckNames`, `deckNamesAndIds`, `createDeck`, `changeDeck`, `deleteDecks`, `getDeckConfig`, `saveDeckConfig`, `getDeckStats` |
| Note types | `modelNames`, `createModel`, `modelFieldNames`, `updateModelTemplates`, `updateModelStyling` |
| Notes | `addNote`, `addNotes` (batch), `canAddNotes`, `updateNoteFields`, `updateNote`, `findNotes` (Anki search syntax), `notesInfo`, `deleteNotes`, tags: `addTags`/`removeTags`/`updateNoteTags` |
| Cards/scheduling | `findCards`, `cardsInfo`, `areDue`, `getIntervals`, `getEaseFactors`/`setEaseFactors`, `answerCards` (ease 1=Again..4=Easy; starts timer then answers), `forgetCards`, `relearnCards`, `setDueDate`, `suspend`/`unsuspend` |
| Review history | `cardReviews`, `getReviewsOfCards`, `getLatestReviewID`, `insertReviews` (9-tuples `(reviewTime, cardID, usn, buttonPressed, newInterval, previousInterval, newFactor, reviewDuration, reviewType)` — i.e., direct revlog writes) |
| Media / bulk / misc | `storeMediaFile` (data/path/url), `exportPackage`, `importPackage`, `sync`, `multi` (batch several actions), `requestPermission`, `version` |

`addNote` supports `options.allowDuplicate`, `duplicateScope`, and attaching `audio`/`video`/`picture` by url/path/base64 into named fields [FACT — same README].

### 5.4 genanki (offline .apkg generation, Python) [FACT — https://raw.githubusercontent.com/kerrickstaley/genanki/master/README.md; PyPI `genanki` 0.13.1, MIT — https://pypi.org/pypi/genanki/json]

- Objects: `Model` (fields + card templates `qfmt`/`afmt` + css), `Note(model, fields=[...])`, `Deck(deck_id, name)`, `Package(deck).write_to_file('out.apkg')`; `package.media_files = [paths]`.
- IDs: `model_id` and `deck_id` must be unique, generated once via `random.randrange(1 << 30, 1 << 31)` and **hardcoded**.
- `Note.guid` defaults to a hash of all field values; keep GUIDs **stable** (e.g., hash only the key field) so re-imports update instead of duplicate. Not affiliated with the Anki project.

### 5.5 FSRS parameter export

- Parameters live per deck-options preset; Anki's protobuf `DeckConfig` carries `repeated float fsrs_params_4/5/6` and `desired_retention`, with a per-deck `desired_retention` override field [FACT — https://raw.githubusercontent.com/ankitects/anki/main/proto/anki/deck_config.proto lines 135–137, 188, 222]. So parameters can be read/written wherever deck config is accessible (UI copy-paste field; `getDeckConfig`/`saveDeckConfig` over AnkiConnect exposes the deck config JSON — [ESTIMATE, medium confidence: the fetched AnkiConnect README's getDeckConfig sample predates FSRS; whether current Anki surfaces fsrs params through that call was not directly verified]).
- An external system can also skip Anki's optimizer entirely: feed revlog-shaped history to `py-fsrs`/`fsrs-rs` (both expose optimize + schedule) [FACT — fsrs-rs README: "full training support", examples `schedule`, `optimize`, `migrate`].

---

## 6. Scheduling design implications (for our scheduler/timeline)

1. **A scheduler is exactly a `(grade, elapsed) → next state/interval` function.** Both algorithm families conform: SM-2 state = (EF, n, last interval), transition keyed on q and (implicitly) on-time review; FSRS state = (D, S), transition explicitly takes elapsed time via `R(t,S)` — fsrs-rs's API is literally `next_states(previous_memory_state, desired_retention, elapsed_days)` returning one candidate state+interval per button [FACT — fsrs-rs README quickstart]. Response-dependence (grade) and time-dependence (elapsed) are both mandatory inputs; SM-2's blind spot is that it ignores actual elapsed time, which classic Anki patched ad hoc (days_late credit) and FSRS handles natively with bounded stability gain [FACT — FAQ + The-Algorithm wiki].
2. **Desired retention is the single workload knob.** Interval = inverse forgetting curve at r: `I(r,S) = (S/factor)(r^(1/DECAY) − 1)`; r=0.9 ⇒ interval = S by construction [FACT — The-Algorithm wiki]. Anki manual: default 0.90; "Higher retention leads to shorter intervals and more reviews per day… Above 90% the workload increases very quickly, and above 97% the workload can be overwhelming" [FACT — deck-options.md].
3. **Workload vs retention is U-shaped, not monotone.** Two forces: higher r ⇒ shorter intervals ⇒ more reviews; lower r ⇒ more forgetting ⇒ more relearning. There is a workload-minimizing retention; Anki's CMRR ("Compute minimum recommended retention") searches for it, and going *below* it is dominated (more time AND less knowledge). Knowledge ≈ Σ R_i across cards [FACT — https://github.com/open-spaced-repetition/fsrs4anki/wiki/The-Optimal-Retention + deck-options.md "Compute minimum recommended retention"].
4. **Rough SM-2-world equivalent of the knob**: interval modifier `log(desired)/log(current)`; +5 pts retention (85→90%) ≈ 35% more reviews [FACT — deck-options.md], consistent with the exponential blow-up near r→1 in FSRS.
5. **Add fuzz.** Deterministic schedulers clump cards introduced together; Anki fuzzes ±(1 day + 15%/10%/5% banded) and won't let users disable it [FACT — studying.md + fuzz.rs]. Any homegrown scheduler should randomize similarly.
6. **Log reviews in revlog shape from day one** — `(timestamp_ms, item_id, grade 1–4, interval, last_interval, duration_ms, type)` — because that exact shape is (a) sufficient to train/optimize FSRS parameters per user, (b) importable into Anki via `insertReviews`, and (c) what all benchmark tooling consumes [FACT — mechanism-of-optimization wiki (trains on revlog), AnkiConnect README (`insertReviews`)].
7. **Interop path of least resistance**: generate content as `.apkg` via genanki (stable note GUIDs) for cold delivery; use AnkiConnect for live round-trip (create/update notes, read `getReviewsOfCards` telemetry back). Grade vocabulary should be Anki's 1–4, mapping to SM-2's 0–5 only at import boundaries (Anki's FAQ explains why 4 grades suffice: failures are rare, so ease adjustment via positive grades is enough) [FACT — FAQ page].

---

## Sources

Fetched this session (all verified reachable and read):

1. SM-2 verbatim mirror: https://raw.githubusercontent.com/CherokeeLanguage/BoundPronouns/2e621baaaddc375ae09f6c2fea238919d38b1325/docs/sm2-notes.txt (canonical http://www.supermemo.com/english/ol/sm2.htm — blocked; formula cross-matched against 26 GitHub mirrors via code search)
2. Anki FAQ source (faqs.ankiweb.net): https://github.com/ankitects/faqs — `src/what-spaced-repetition-algorithm.md` (CNAME confirms it builds faqs.ankiweb.net)
3. Anki manual source (docs.ankiweb.net): https://github.com/ankitects/anki-manual — `src/deck-options.md`, `src/studying.md`, `src/exporting.md`
4. Anki scheduler source: https://github.com/ankitects/anki — `rslib/src/scheduler/states/review.rs`, `rslib/src/scheduler/states/fuzz.rs`, `rslib/Cargo.toml`, `proto/anki/deck_config.proto`
5. FSRS algorithm wiki (official): https://github.com/open-spaced-repetition/awesome-fsrs/wiki/The-Algorithm and /The-mechanism-of-optimization (cloned wiki git; fsrs4anki wiki redirects here)
6. FSRS optimal retention: https://github.com/open-spaced-repetition/fsrs4anki/wiki/The-Optimal-Retention (cloned wiki git)
7. FSRS tutorial with spacing-effect studies: https://github.com/open-spaced-repetition/awesome-fsrs/wiki/Spaced-Repetition-Algorithm:-A-Three%E2%80%90Day-Journey-from-Novice-to-Expert
8. fsrs-rs: https://github.com/open-spaced-repetition/fsrs-rs (README + LICENSE = BSD-3-Clause); fsrs4anki LICENSE = MIT
9. py-fsrs metadata: https://pypi.org/pypi/fsrs/json (v6.3.1, MIT)
10. AnkiConnect README (pre-sr.ht-move mirror): https://raw.githubusercontent.com/johan456789/anki-connect-mirror/e437f892ff6a44b0da2e8610005503d6d1c57068/README.md (canonical moved to https://git.sr.ht/~foosoft/anki-connect — blocked)
11. genanki: https://github.com/kerrickstaley/genanki (README) + https://pypi.org/pypi/genanki/json (0.13.1, MIT)
12. Anki DB schema: https://github.com/ankidroid/Anki-Android/wiki/Database-Structure (cloned wiki git)

Not fetchable this session (egress policy): supermemo.com / super-memory.com, docs.ankiweb.net, faqs.ankiweb.net (rendered sites; sources used instead), supermemo.guru, Wikipedia, journal sites for Ebbinghaus/Cepeda/Roediger primary texts.
