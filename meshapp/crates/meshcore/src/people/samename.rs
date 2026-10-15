//! Same-name marker: name normalisation and marker state (BRIEF_v2 §3
//! "Same-name alert"; DESIGN.md §1 row `people/samename.rs`; DECISIONS D5
//! rules R3, R5, R6, R9; owner default #9 in docs/decisions-assumed.md).
//!
//! The P0 key-change signal is *not* a "Ravi's identity changed" banner. When
//! a key delivers a display name (inside an encrypted payload: a roster
//! entry or a 1:1 session, D5 R3) that normalises to the same string as the
//! display name or petname of a **different pinned key**, every entry with
//! that name shows the same inline marker, "2 phones use the name Ravi"
//! (BRIEF §7 row "Two keys, one name"). The marker never links the keys, never
//! says which one is "new", "old" or "changed", and never singles one out
//! (D5 R5). It is a speed bump; the `Unverified` badge is the real control
//! (D5 "Remaining risk").
//!
//! **Normalisation** ([`normalise`]; BRIEF §3; D5 R3): NFKC → casefold →
//! strip Default_Ignorable code points, whitespace and punctuation/symbols →
//! UTS #39 skeleton. The confusable data is the `unicode-security` crate's
//! bundled table; its version is pinned per release as
//! [`SKELETON_DATA_VERSION`], because skeletons are unstable across Unicode
//! versions (D5 R3, V5). Phones on different releases may disagree about a
//! marker; that affects only local display (D5 "Remaining risk").
//!
//! **State** ([`SameNameIndex`]; D5 R9): the marker is a function of (keys
//! seen with their names, local pins and petnames, silenced pairs), so every
//! phone shows the same thing after a rejoin, whatever the arrival order.
//! Observing a name has **zero network side effect** (D5 R8): nothing here
//! sends, signs or changes delivery behaviour.
//!
//! **Interrupts** (D5 R6; BRIEF §3 "Interruptions"): one in-app card per
//! normalised name per phone, never a push notification; later claimants
//! join the same card. Beyond [`CARD_CAP_PER_HOUR_ASM`] distinct names in an
//! hour of powered-on time, cards aggregate. "These are different people"
//! silences one pair ([`SameNameIndex::silence_pair`]).

use std::collections::{BTreeMap, BTreeSet};

use unicode_normalization::UnicodeNormalization;

use crate::crypto::ed25519::IdentityKey;

/// Version of the `unicode-security` crate whose confusables table produces
/// the UTS #39 skeletons, exactly as pinned in `Cargo.lock`. Stored in
/// `meta(skeleton_data_version)` (DESIGN §6) so a release that bumps the
/// table can re-normalise the index (BRIEF §3 "with the data version pinned
/// per release"; D5 R3, V5).
pub const SKELETON_DATA_VERSION: &str = "0.1.2";

/// The Unicode version behind [`SKELETON_DATA_VERSION`]'s table, as the
/// crate reports it (`(major, minor, patch)`).
pub const SKELETON_UNICODE_VERSION: (u64, u64, u64) = unicode_security::UNICODE_VERSION;

/// Distinct names that may each get their own in-app card per hour of
/// powered-on time before cards aggregate (D5 R6 "assumption: 3 names per
/// hour"; BRIEF §3 "about 3 names/h, then aggregated"). `_ASM` marks it as
/// an owner-adjustable assumption; the B build may lower it (BRIEF §3 "a
/// lower card cap").
pub const CARD_CAP_PER_HOUR_ASM: usize = 3;

/// The card window, in powered-on milliseconds (never wall time, BRIEF §4).
pub const CARD_WINDOW_MS: u64 = 60 * 60 * 1000;

// ---- normalisation ----------------------------------------------------------

/// Default_Ignorable_Code_Point ranges (Unicode DerivedCoreProperties),
/// listed by hand so no new dependency is needed. Inclusive.
const DEFAULT_IGNORABLE: &[(u32, u32)] = &[
    (0x00AD, 0x00AD),   // SOFT HYPHEN
    (0x034F, 0x034F),   // COMBINING GRAPHEME JOINER
    (0x061C, 0x061C),   // ARABIC LETTER MARK
    (0x115F, 0x1160),   // HANGUL FILLERS
    (0x17B4, 0x17B5),   // KHMER VOWEL INHERENT
    (0x180B, 0x180F),   // MONGOLIAN FREE VARIATION SELECTORS, VOWEL SEPARATOR
    (0x200B, 0x200F),   // ZWSP, ZWNJ, ZWJ, LRM, RLM
    (0x202A, 0x202E),   // bidi embedding/override controls
    (0x2060, 0x206F),   // WJ, invisible operators, bidi isolates, deprecated
    (0x3164, 0x3164),   // HANGUL FILLER
    (0xFE00, 0xFE0F),   // VARIATION SELECTORS
    (0xFEFF, 0xFEFF),   // ZERO WIDTH NO-BREAK SPACE (BOM)
    (0xFFA0, 0xFFA0),   // HALFWIDTH HANGUL FILLER
    (0xFFF0, 0xFFF8),   // unassigned default ignorables
    (0x1BCA0, 0x1BCA3), // SHORTHAND FORMAT CONTROLS
    (0x1D173, 0x1D17A), // MUSICAL SYMBOL controls
    (0xE0000, 0xE0FFF), // TAGS, VARIATION SELECTORS SUPPLEMENT
];

/// Punctuation and symbol blocks recognised without a Unicode-category
/// dependency: the common blocks named by the spec plus the Latin-1 and
/// ASCII-adjacent ones. Inclusive. (ª U+00AA and º U+00BA are letters that
/// NFKC already folds to `a`/`o`, so the Latin-1 range may include them.)
const PUNCT_SYMBOL_BLOCKS: &[(u32, u32)] = &[
    (0x00A1, 0x00BF), // Latin-1 punctuation and symbols (¡ ¿ « » § © ® ° ± · ...)
    (0x00D7, 0x00D7), // MULTIPLICATION SIGN
    (0x00F7, 0x00F7), // DIVISION SIGN
    (0x2000, 0x206F), // General Punctuation (dashes, quotes, bullets, ‰ † ...)
    (0x2070, 0x209F), // Super/subscripts (NFKC folds most; the rest are symbols)
    (0x20A0, 0x20CF), // Currency Symbols
    (0x2100, 0x214F), // Letterlike Symbols (NFKC folds ℝ → R; the rest stay)
    (0x2190, 0x23FF), // Arrows, Mathematical Operators, Misc Technical
    (0x2400, 0x27BF), // Control Pictures ... Dingbats
    (0x27C0, 0x2BFF), // Misc Math Symbols, Arrows, Misc Symbols and Arrows
    (0x2E00, 0x2E7F), // Supplemental Punctuation
    (0x3000, 0x303F), // CJK Symbols and Punctuation
    (0xFE10, 0xFE1F), // Vertical Forms
    (0xFE30, 0xFE4F), // CJK Compatibility Forms
    (0xFE50, 0xFE6F), // Small Form Variants
    (0xFF00, 0xFF0F), // Fullwidth ASCII punctuation (NFKC folds these first)
    (0xFF1A, 0xFF20),
    (0xFF3B, 0xFF40),
    (0xFF5B, 0xFF65),
    (0x1F000, 0x1FAFF), // Mahjong ... Symbols and Pictographs Extended-A (emoji)
];

fn in_ranges(c: char, ranges: &[(u32, u32)]) -> bool {
    let cp = u32::from(c);
    ranges.iter().any(|&(lo, hi)| lo <= cp && cp <= hi)
}

/// A code point that carries no name content: whitespace, controls,
/// punctuation, symbols, or Default_Ignorable.
fn is_stripped(c: char) -> bool {
    c.is_whitespace()
        || c.is_control()
        || c.is_ascii_punctuation()
        || in_ranges(c, DEFAULT_IGNORABLE)
        || in_ranges(c, PUNCT_SYMBOL_BLOCKS)
}

/// Casefold then strip, as one pass. Casefolding is `char::to_lowercase`
/// (simple, context-free lowercase mapping) rather than full Unicode
/// case folding: `std` has no `to_casefold`, so a few folds differ, e.g. `ß`
/// lowercases to itself while full folding gives `ss`, and final-sigma
/// context is ignored. Both phones in a pair run the same code, so the
/// result is still identical on every phone (D5 R9); the only cost is a
/// missed match for those few letters (D5 "Remaining risk": the alert is a
/// speed bump).
fn fold_and_strip(chars: impl Iterator<Item = char>) -> String {
    chars
        .flat_map(char::to_lowercase)
        .filter(|c| !is_stripped(*c))
        .collect()
}

/// Normalises a display name or petname for the same-name match (BRIEF §3
/// "Normalisation: NFKC casefold, strip default-ignorable characters,
/// whitespace and punctuation, then the UTS #39 skeleton"; D5 R3).
///
/// Steps, in order:
/// 1. NFKC (compatibility composition: `ℝ` → `R`, fullwidth → ASCII,
///    ligatures expanded);
/// 2. casefold via `char::to_lowercase` (see `fold_and_strip` for the
///    limitation);
/// 3. strip whitespace, controls, ASCII punctuation, the General
///    Punctuation (U+2000–206F), CJK Symbols (U+3000–303F) and other
///    symbol blocks listed in `PUNCT_SYMBOL_BLOCKS`, and every
///    Default_Ignorable code point (ZWJ, ZWNJ, ZWSP, soft hyphen, variation
///    selectors, BOM, ...);
/// 4. UTS #39 skeleton (`unicode_security::skeleton`, data version
///    [`SKELETON_DATA_VERSION`]), which maps confusables such as Cyrillic
///    `а` to Latin `a`;
/// 5. NFKC again (the skeleton works on NFD and may leave combining marks
///    in non-canonical order), then steps 2–3 once more, because a
///    confusable prototype may itself be an upper-case letter or a symbol.
///
/// Deterministic and total: any `&str` gives one `String`; an empty result
/// means the name had no content to match on and never collides. It is
/// applied exactly once, to the raw delivered name or petname, and the
/// result is compared for equality only: the bundled confusables table is
/// not transitively closed for every code point (e.g. `𐊣 → δ` while `δ →
/// ẟ`), so `normalise(normalise(x))` can differ from `normalise(x)` and
/// callers must never re-normalise a stored normalised string.
pub fn normalise(name: &str) -> String {
    let folded = fold_and_strip(name.nfkc());
    // The skeleton is built on NFD and can leave combining marks in
    // non-canonical order; recompose (NFKC) so equal names give equal bytes
    // and the result is stable under a second pass.
    let skeleton: String = unicode_security::skeleton(&folded).nfkc().collect();
    fold_and_strip(skeleton.chars())
}

// ---- marker -----------------------------------------------------------------

/// What an entry shows when its name is shared (D5 R5): the number of pinned
/// keys using the name, this one included, and the other keys. The others
/// are listed only so the tap-through can show each entry's **own** facts
/// ("You scanned this phone's code on 12 Oct", "Not verified", "In group
/// G"); the marker itself never links them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Marker {
    /// Keys using the name, this entry's included (always ≥ 2).
    pub count: usize,
    /// The other keys, sorted.
    pub others: Vec<IdentityKey>,
}

impl Marker {
    /// The inline marker text: "2 phones use the name Ravi" (BRIEF §7 row
    /// "Two keys, one name", "Use" column; D5 R5). `display_name` is the
    /// entry's own shown name. The count updates as more claimants arrive.
    /// No "changed", "new", "old" or "impostor" wording, ever.
    pub fn text(&self, display_name: &str) -> String {
        format!("{} phones use the name {display_name}", self.count)
    }
}

/// Whether a same-name card may interrupt (D5 R6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardDecision {
    /// Show (or update) this name's own in-app card.
    Show,
    /// The hourly cap is reached: fold this name into the aggregate card.
    Aggregate,
}

/// The aggregate card's text: "N phones are using names of your contacts"
/// (D5 R6).
pub fn aggregate_text(phones: usize) -> String {
    format!("{phones} phones are using names of your contacts")
}

/// The card's call to action (D5 R6): nothing is required of the user.
pub const CARD_CALL_TO_ACTION: &str = "Check when you next see them";

/// Per-name card throttle (D5 R6). Pure in `now_mono`; never reads a clock.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct CardThrottle {
    /// First decision made for each normalised name. A name's card never
    /// changes kind: later claimants join it (R6 "an attacker cannot
    /// pre-burn it").
    decided: BTreeMap<String, CardDecision>,
    /// Powered-on times at which a *new* name got its own card.
    shown_at: Vec<u64>,
}

impl CardThrottle {
    fn decide(&mut self, name: &str, now_mono: u64) -> CardDecision {
        if let Some(d) = self.decided.get(name) {
            return *d;
        }
        let window_start = now_mono.saturating_sub(CARD_WINDOW_MS);
        self.shown_at.retain(|&t| t > window_start && t <= now_mono);
        let decision = if self.shown_at.len() < CARD_CAP_PER_HOUR_ASM {
            self.shown_at.push(now_mono);
            CardDecision::Show
        } else {
            CardDecision::Aggregate
        };
        self.decided.insert(name.to_owned(), decision);
        decision
    }
}

/// Marker state as `f(keys seen with names, pins, petnames, silenced pairs)`
/// (DESIGN §1 `people/samename.rs`; D5 R9).
///
/// The node feeds it only names that arrived **inside encrypted payloads**
/// from keys it pins under D5 R2 (roster entries of shared groups, 1:1
/// sessions): BRIEF §3 B "names and keys never appear in adverts, scan
/// responses or cleartext relay headers", so a name seen anywhere else is
/// never observed here. Every method is deterministic; iteration is over
/// `BTreeMap`/`BTreeSet`, so `others` lists and counts are the same on every
/// phone with the same inputs.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SameNameIndex {
    /// Delivered display name per key, normalised (D5 R3 "norm(name)").
    display: BTreeMap<IdentityKey, String>,
    /// Local petname per key, normalised (D5 R3 "norm(petname)").
    petname: BTreeMap<IdentityKey, String>,
    /// Normalised name → keys carrying it as display name or petname.
    by_name: BTreeMap<String, BTreeSet<IdentityKey>>,
    /// "These are different people", stored as `(min, max)`.
    silenced: BTreeSet<(IdentityKey, IdentityKey)>,
    cards: CardThrottle,
}

fn pair(a: IdentityKey, b: IdentityKey) -> (IdentityKey, IdentityKey) {
    if a <= b { (a, b) } else { (b, a) }
}

impl SameNameIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Rebuilds an index from persisted silenced pairs (D5 R6 "permanently
    /// silences that pair"); names are re-observed from the pin store so a
    /// release with a new [`SKELETON_DATA_VERSION`] re-normalises them.
    pub fn with_silenced(pairs: impl IntoIterator<Item = (IdentityKey, IdentityKey)>) -> Self {
        let mut idx = Self::new();
        for (a, b) in pairs {
            idx.silence_pair(&a, &b);
        }
        idx
    }

    fn unindex(&mut self, key: &IdentityKey, name: &str) {
        if name.is_empty() {
            return;
        }
        // Keep the key under `name` if its other name is the same string.
        let still_used = self.display.get(key).is_some_and(|n| n == name)
            || self.petname.get(key).is_some_and(|n| n == name);
        if still_used {
            return;
        }
        if let Some(set) = self.by_name.get_mut(name) {
            set.remove(key);
            if set.is_empty() {
                self.by_name.remove(name);
            }
        }
    }

    fn index(&mut self, key: IdentityKey, name: &str) {
        if name.is_empty() {
            return;
        }
        self.by_name.entry(name.to_owned()).or_default().insert(key);
    }

    /// Records that pinned `key` delivered `display_name` inside an
    /// encrypted payload (D5 R3). Replaces the key's previous display name.
    /// Has no network side effect (D5 R8).
    pub fn observe(&mut self, key: IdentityKey, display_name: &str) {
        let norm = normalise(display_name);
        if let Some(old) = self.display.insert(key, norm.clone()) {
            self.unindex(&key, &old);
        }
        self.index(key, &norm);
    }

    /// Records the local petname of `key` (`None` or empty clears it).
    /// Petnames take part in the match (D5 R3 "norm(name or petname)").
    pub fn set_petname(&mut self, key: IdentityKey, petname: Option<&str>) {
        let norm = petname.map(normalise).unwrap_or_default();
        let old = if norm.is_empty() {
            self.petname.remove(&key)
        } else {
            self.petname.insert(key, norm.clone())
        };
        if let Some(old) = old {
            self.unindex(&key, &old);
        }
        self.index(key, &norm);
    }

    /// Drops `key` entirely (the contact was removed). Its names leave the
    /// index, so markers on the keys it collided with disappear or count
    /// down; silenced pairs mentioning it are kept, since silence is
    /// permanent (D5 R6).
    pub fn forget(&mut self, key: &IdentityKey) {
        let display = self.display.remove(key);
        let petname = self.petname.remove(key);
        for name in [display, petname].into_iter().flatten() {
            self.unindex(key, &name);
        }
    }

    /// Whether `key` is in the index (has a display name or petname).
    pub fn contains(&self, key: &IdentityKey) -> bool {
        self.display.contains_key(key) || self.petname.contains_key(key)
    }

    /// The normalised names `key` currently carries (display name and
    /// petname, deduplicated, sorted; empty strings excluded).
    pub fn names_of(&self, key: &IdentityKey) -> BTreeSet<String> {
        self.display
            .get(key)
            .into_iter()
            .chain(self.petname.get(key))
            .filter(|n| !n.is_empty())
            .cloned()
            .collect()
    }

    /// The other pinned keys that share any of `key`'s normalised names,
    /// minus pairs the user silenced; sorted.
    fn collisions(&self, key: &IdentityKey) -> BTreeSet<IdentityKey> {
        let mut others = BTreeSet::new();
        for name in self.names_of(key) {
            if let Some(set) = self.by_name.get(&name) {
                others.extend(
                    set.iter()
                        .filter(|k| *k != key && !self.silenced.contains(&pair(*key, **k)))
                        .copied(),
                );
            }
        }
        others
    }

    /// The marker `key`'s entry shows, if ≥ 2 pinned keys (this one
    /// included) share a normalised display name or petname (D5 R3, R5).
    /// Symmetric by construction: if `b` is among `a`'s `others`, `a` is
    /// among `b`'s. Shown everywhere the name appears (BRIEF §3).
    pub fn marker_for(&self, key: &IdentityKey) -> Option<Marker> {
        let others = self.collisions(key);
        if others.is_empty() {
            return None;
        }
        Some(Marker {
            count: others.len().saturating_add(1),
            others: others.into_iter().collect(),
        })
    }

    /// "These are different people" (BRIEF §3; D5 R6): permanently silences
    /// the marker between `a` and `b`, in both directions. Other keys
    /// sharing the name still mark both. Setting petnames to tell them apart
    /// is the caller's job ([`super::contacts::ContactStore::set_petname`]).
    pub fn silence_pair(&mut self, a: &IdentityKey, b: &IdentityKey) {
        if a != b {
            self.silenced.insert(pair(*a, *b));
        }
    }

    /// Whether the pair is silenced (for persistence and the detail view).
    pub fn is_silenced(&self, a: &IdentityKey, b: &IdentityKey) -> bool {
        self.silenced.contains(&pair(*a, *b))
    }

    /// The silenced pairs, sorted, for persistence.
    pub fn silenced(&self) -> impl Iterator<Item = (IdentityKey, IdentityKey)> + '_ {
        self.silenced.iter().copied()
    }

    /// Every normalised name currently shared by ≥ 2 keys, with those keys.
    /// Silenced pairs are not applied here (a name with exactly one silenced
    /// pair still appears); use [`SameNameIndex::marker_for`] per entry.
    pub fn shared_names(&self) -> BTreeMap<String, BTreeSet<IdentityKey>> {
        self.by_name
            .iter()
            .filter(|(_, keys)| keys.len() >= 2)
            .map(|(n, k)| (n.clone(), k.clone()))
            .collect()
    }

    /// Whether the card for `normalised_name` may be shown on its own or
    /// must join the aggregate (D5 R6; BRIEF §3 "Interruptions"). The first
    /// call for a name decides for good; later claimants join the same
    /// card. At most [`CARD_CAP_PER_HOUR_ASM`] new names get their own card
    /// in any [`CARD_WINDOW_MS`] of powered-on time (`now_mono`). Never a
    /// push notification: the UI maps `Show` to an in-app card only.
    pub fn should_show_card(&mut self, normalised_name: &str, now_mono: u64) -> CardDecision {
        self.cards.decide(normalised_name, now_mono)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::ed25519::IdentitySigner;
    use proptest::prelude::*;

    fn key(i: u8) -> IdentityKey {
        let mut seed = [0u8; 32];
        seed[0] = i;
        seed[1] = 0x5A;
        IdentitySigner::from_seed(&seed).public()
    }

    #[test]
    fn data_version_matches_cargo_lock() {
        // The literal is checked against Cargo.lock by tests/people.rs; here
        // we only pin the shape.
        assert_eq!(SKELETON_DATA_VERSION, "0.1.2");
        assert!(SKELETON_UNICODE_VERSION.0 >= 15);
    }

    #[test]
    fn normalise_steps() {
        assert_eq!(normalise("Ravi"), "ravi");
        assert_eq!(normalise("  Ravi.  "), "ravi");
        assert_eq!(normalise("R A V I"), "ravi");
        assert_eq!(normalise("ℝavi"), "ravi");
        assert_eq!(normalise("Ｒａｖｉ"), "ravi"); // fullwidth
        assert_eq!(normalise("Ra\u{200D}vi"), "ravi"); // ZWJ
        assert_eq!(
            normalise("Ra\u{200C}\u{200B}\u{00AD}vi\u{FE0F}\u{FEFF}"),
            "ravi"
        );
        assert_eq!(normalise("Rаvi"), "ravi"); // Cyrillic а
        assert_eq!(normalise("ravi‼"), "ravi");
        assert_eq!(normalise("“Ravi”"), "ravi");
        assert_eq!(normalise("Ravi\u{3000}。"), "ravi");
        assert_eq!(normalise("Ravi 🙂"), "ravi");
        assert_eq!(normalise(""), "");
        assert_eq!(normalise(" . - "), "");
        assert_ne!(normalise("Ravi"), normalise("Asha"));
        assert_ne!(normalise("Ravi"), normalise("Ravi K"));
    }

    #[test]
    fn normalise_edge_cases_are_deterministic() {
        // Found by proptest: combining-mark reordering ("ÀÐ") and a
        // non-closed confusable chain ("𐊣" → δ → ẟ). Both must be total and
        // deterministic; idempotence is not promised (see `normalise`).
        for s in ["ÀÐ", "𐊣", "\u{300}\u{335}", "\u{FFFD}", "\u{10FFFF}"] {
            assert_eq!(normalise(s), normalise(s), "{s}");
            assert!(!normalise(s).chars().any(|c| c.is_whitespace()), "{s}");
        }
        assert_eq!(normalise("ÀÐ"), normalise("àð"));
    }

    #[test]
    fn normalise_is_idempotent() {
        for s in [
            "Ravi",
            "ℝavi",
            "Rаvi",
            "ﬁona",
            "Straße",
            "İstanbul",
            "Ωmega",
            "ǅ",
        ] {
            let once = normalise(s);
            assert_eq!(normalise(&once), once, "{s}");
        }
    }

    #[test]
    fn marker_is_symmetric_and_counts() {
        let mut idx = SameNameIndex::new();
        idx.observe(key(1), "Ravi");
        assert_eq!(idx.marker_for(&key(1)), None);
        idx.observe(key(2), "ravi ");
        let m1 = idx.marker_for(&key(1)).unwrap();
        let m2 = idx.marker_for(&key(2)).unwrap();
        assert_eq!(m1.count, 2);
        assert_eq!(m1.others, vec![key(2)]);
        assert_eq!(m2.others, vec![key(1)]);
        assert_eq!(m1.text("Ravi"), "2 phones use the name Ravi");
        idx.observe(key(3), "Rаvi"); // Cyrillic а
        assert_eq!(idx.marker_for(&key(1)).unwrap().count, 3);
        assert_eq!(
            idx.marker_for(&key(3)).unwrap().text("Rаvi"),
            "3 phones use the name Rаvi"
        );
        // unrelated name: no marker
        idx.observe(key(4), "Asha");
        assert_eq!(idx.marker_for(&key(4)), None);
        assert_eq!(idx.shared_names().len(), 1);
    }

    #[test]
    fn petname_matches_and_rename_unindexes() {
        let mut idx = SameNameIndex::new();
        idx.observe(key(1), "Ravi");
        idx.observe(key(2), "R. Kumar");
        assert_eq!(idx.marker_for(&key(1)), None);
        idx.set_petname(key(2), Some("Ravi"));
        assert_eq!(idx.marker_for(&key(1)).unwrap().others, vec![key(2)]);
        // same string as display name and petname: unindexing one keeps the other
        idx.set_petname(key(1), Some("ravi"));
        idx.set_petname(key(1), None);
        assert_eq!(idx.marker_for(&key(2)).unwrap().others, vec![key(1)]);
        idx.set_petname(key(2), Some(""));
        assert_eq!(idx.marker_for(&key(1)), None);
        // a new display name replaces the old one
        idx.observe(key(2), "Ravi");
        idx.observe(key(2), "Rav");
        assert_eq!(idx.marker_for(&key(1)), None);
        assert_eq!(
            idx.names_of(&key(2)).into_iter().collect::<Vec<_>>(),
            vec!["rav"]
        );
    }

    #[test]
    fn forget_removes_marker() {
        let mut idx = SameNameIndex::new();
        idx.observe(key(1), "Ravi");
        idx.observe(key(2), "Ravi");
        assert!(idx.marker_for(&key(1)).is_some());
        idx.forget(&key(2));
        assert_eq!(idx.marker_for(&key(1)), None);
        assert!(!idx.contains(&key(2)));
        assert!(idx.contains(&key(1)));
    }

    #[test]
    fn silence_is_pairwise_and_persisted() {
        let mut idx = SameNameIndex::new();
        idx.observe(key(1), "Ravi");
        idx.observe(key(2), "Ravi");
        idx.observe(key(3), "Ravi");
        idx.silence_pair(&key(2), &key(1));
        assert!(idx.is_silenced(&key(1), &key(2)));
        assert_eq!(idx.marker_for(&key(1)).unwrap().others, vec![key(3)]);
        assert_eq!(idx.marker_for(&key(2)).unwrap().others, vec![key(3)]);
        assert_eq!(idx.marker_for(&key(3)).unwrap().count, 3);
        idx.silence_pair(&key(1), &key(3));
        assert_eq!(idx.marker_for(&key(1)), None);
        idx.silence_pair(&key(1), &key(1)); // no-op
        let pairs: Vec<_> = idx.silenced().collect();
        assert_eq!(pairs.len(), 2);
        let rebuilt = SameNameIndex::with_silenced(pairs);
        assert!(rebuilt.is_silenced(&key(3), &key(1)));
    }

    #[test]
    fn arrival_order_does_not_matter() {
        let mut a = SameNameIndex::new();
        let mut b = SameNameIndex::new();
        a.observe(key(1), "Ravi");
        a.observe(key(2), "ravi");
        a.set_petname(key(3), Some("RAVI"));
        b.set_petname(key(3), Some("RAVI"));
        b.observe(key(2), "ravi");
        b.observe(key(1), "Ravi");
        for k in [key(1), key(2), key(3)] {
            assert_eq!(a.marker_for(&k), b.marker_for(&k));
        }
    }

    #[test]
    fn card_cap_then_aggregate() {
        let mut idx = SameNameIndex::new();
        let t0 = 10 * CARD_WINDOW_MS;
        assert_eq!(idx.should_show_card("ravi", t0), CardDecision::Show);
        assert_eq!(idx.should_show_card("asha", t0 + 1), CardDecision::Show);
        assert_eq!(idx.should_show_card("marco", t0 + 2), CardDecision::Show);
        assert_eq!(
            idx.should_show_card("priya", t0 + 3),
            CardDecision::Aggregate
        );
        // a name already decided keeps its decision
        assert_eq!(idx.should_show_card("ravi", t0 + 4), CardDecision::Show);
        assert_eq!(
            idx.should_show_card("priya", t0 + CARD_WINDOW_MS * 2),
            CardDecision::Aggregate
        );
        // the window slides: the card shown at t0 leaves the window exactly
        // one window later, so the cap frees one slot
        assert_eq!(
            idx.should_show_card("zed", t0 + CARD_WINDOW_MS - 1),
            CardDecision::Aggregate
        );
        assert_eq!(
            idx.should_show_card("yan", t0 + CARD_WINDOW_MS),
            CardDecision::Show
        );
        assert_eq!(
            idx.should_show_card("xi", t0 + CARD_WINDOW_MS),
            CardDecision::Aggregate
        );
        assert_eq!(
            aggregate_text(4),
            "4 phones are using names of your contacts"
        );
        // time running backwards (restart without persisted meter) never panics
        assert_eq!(idx.should_show_card("q", 0), CardDecision::Show);
    }

    proptest! {
        #[test]
        fn normalise_never_panics_and_is_deterministic(s in "\\PC{0,40}") {
            let n = normalise(&s);
            prop_assert_eq!(normalise(&s), n.clone());
            // output carries none of the stripped classes
            prop_assert!(!n.chars().any(|c| c.is_whitespace() || c.is_ascii_punctuation()));
        }

        #[test]
        fn marker_symmetry(names in proptest::collection::vec(0u8..4, 1..8)) {
            let mut idx = SameNameIndex::new();
            let pool = ["Ravi", "ravi.", "Asha", "Rаvi"];
            for (i, n) in names.iter().enumerate() {
                let i = u8::try_from(i).unwrap();
                idx.observe(key(i), pool[usize::from(*n)]);
            }
            for i in 0..names.len() {
                let i = u8::try_from(i).unwrap();
                if let Some(m) = idx.marker_for(&key(i)) {
                    prop_assert_eq!(m.count, m.others.len() + 1);
                    for o in &m.others {
                        let back = idx.marker_for(o).unwrap();
                        prop_assert!(back.others.contains(&key(i)));
                        prop_assert_eq!(back.count, m.count);
                    }
                }
            }
        }
    }
}
