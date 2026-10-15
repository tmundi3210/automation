//! The pure tally (docs/DESIGN.md §5 "Tally"; BRIEF_v2 §4 "Final rule",
//! "States"; DECISIONS D1 rules 2–5; §3 row (1,2,5,6)).
//!
//! Inputs are a validated opening, `members(H)` and a list of ballots; the
//! output is a function of the *set* of distinct `(key, choice)` payloads
//! among them, so order, duplicates and re-sends never matter and no clock
//! is read. Per key, the distinct choices it signed form its payload set:
//!
//! - `s_X`       = keys whose set contains X (equivocators included);
//! - `displayed` = keys whose set is exactly `{X}` (equivocators excluded);
//! - `void`      = keys with ≥ 2 distinct payloads (D1 rule 3);
//! - `heard`     = keys with any payload (void keys count as heard).
//!
//! State (D1 rules 4–5; BRIEF_v2 §4):
//! - `Disputed { members }` iff `|void| > f_eff = 2T − N − 1`;
//! - else `Final(X)` iff `s_X ≥ T` and, with more than two options,
//!   `s_X > N/2` (absolute majority; already implied by T, checked anyway);
//! - else `LeadingProvisional` when `heard < ⌊N/2⌋ + 1`, otherwise
//!   `ResultProvisional { needs_t }`.
//!
//! The signer-count form is used deliberately: "margin > not yet voted" is
//! non-monotone under a later void and lets one member who signs in two
//! fragments produce opposite Finals (BRIEF_v2 §4 "Do not implement"; D1
//! rule 4). `tests/tasks_votes.rs::final_rule` checks that Final is
//! monotone: adding ballots to a Final(X) set yields Final(X) or Disputed.
//!
//! Ballots are dropped (counted in [`Tally::rejected`]) when the key is
//! outside `members(H)` (D2 rule 2), the vote id differs, the choice is
//! out of range, or the strict signature fails. The "keep ≤ 2 payloads as
//! proof" rule is storage's; the tally only needs the set.

use std::collections::{BTreeMap, BTreeSet};

use crate::crypto::ed25519::IdentityKey;
use crate::objects::ballot::Ballot;
use crate::votes::open::ValidOpen;

/// The vote's state (BRIEF_v2 §4 "States", D1 rule 5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoteState {
    /// No ballot still out can change it (BRIEF_v2 §4).
    Final { option: u8 },
    /// More than `2T − N − 1` void keys; `members` names them (sorted).
    Disputed { members: Vec<IdentityKey> },
    /// Turnout below `⌊N/2⌋ + 1`. `leading` is `None` on a tie or with no
    /// displayed ballots.
    LeadingProvisional {
        leading: Option<u8>,
        k_not_heard: u16,
    },
    /// Turnout at or above `⌊N/2⌋ + 1` but no option at T.
    ResultProvisional {
        leading: Option<u8>,
        needs_t: u16,
        k_not_heard: u16,
    },
}

impl VoteState {
    pub fn is_final(&self) -> bool {
        matches!(self, VoteState::Final { .. })
    }

    pub fn is_disputed(&self) -> bool {
        matches!(self, VoteState::Disputed { .. })
    }

    pub fn is_provisional(&self) -> bool {
        matches!(
            self,
            VoteState::LeadingProvisional { .. } | VoteState::ResultProvisional { .. }
        )
    }

    /// The provisional lead, if this is a provisional state with one.
    pub fn leading(&self) -> Option<u8> {
        match self {
            VoteState::LeadingProvisional { leading, .. }
            | VoteState::ResultProvisional { leading, .. } => *leading,
            VoteState::Final { .. } | VoteState::Disputed { .. } => None,
        }
    }
}

/// The counted vote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tally {
    /// Per option: distinct keys with ≥ 1 valid ballot for it (equivocators included).
    pub s: Vec<u32>,
    /// Per option: `s` without the void keys; what the card shows.
    pub displayed: Vec<u32>,
    /// Keys with two or more distinct payloads (D1 rule 3).
    pub void_keys: BTreeSet<IdentityKey>,
    /// Distinct member keys with any valid ballot (void keys included).
    pub heard: usize,
    /// `members(H)` with no valid ballot, sorted (the tappable "k not heard from").
    pub not_heard: Vec<IdentityKey>,
    /// Ballots dropped: non-member key, other vote, bad choice or bad signature.
    pub rejected: usize,
    pub state: VoteState,
}

impl Tally {
    /// `displayed` for one option, 0 when out of range.
    pub fn displayed_for(&self, option: u8) -> u32 {
        self.displayed
            .get(usize::from(option))
            .copied()
            .unwrap_or(0)
    }
}

/// The option with the highest displayed count, or `None` when the top two
/// are tied or nothing has been counted. Lowest index wins among equals
/// only if it is strictly ahead of every other option.
pub fn leading(displayed: &[u32]) -> Option<u8> {
    let (best_i, best) = displayed.iter().enumerate().max_by_key(|(_, c)| **c)?;
    if *best == 0 {
        return None;
    }
    let ties = displayed.iter().filter(|c| **c == *best).count();
    if ties > 1 {
        return None;
    }
    u8::try_from(best_i).ok()
}

/// Gap between the two highest displayed counts (0 with one option or a tie).
pub fn top_two_gap(displayed: &[u32]) -> u32 {
    let mut sorted: Vec<u32> = displayed.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    match (sorted.first(), sorted.get(1)) {
        (Some(a), Some(b)) => a.saturating_sub(*b),
        _ => 0,
    }
}

/// Verifies each ballot against the opening and `members(H)`, then counts
/// (module docs).
pub fn tally(open: &ValidOpen, members_of_h: &BTreeSet<IdentityKey>, ballots: &[Ballot]) -> Tally {
    let vote_id = open.vote_id();
    let n_opt = open.options().len();
    let mut rejected: usize = 0;
    let mut payloads: Vec<(IdentityKey, u8)> = Vec::with_capacity(ballots.len());
    for b in ballots {
        let ok = b.vote_id == vote_id
            && members_of_h.contains(&b.key)
            && usize::from(b.choice) < n_opt
            && b.verify().is_ok();
        if ok {
            payloads.push((b.key, b.choice));
        } else {
            rejected = rejected.saturating_add(1);
        }
    }
    let mut t = tally_payloads(open, members_of_h, payloads);
    t.rejected = rejected;
    t
}

/// Counts already-verified `(key, choice)` payloads (module docs). Pure:
/// this is the function the `final_rule` property test drives. Payloads
/// from keys outside `members(H)` or with an out-of-range choice are
/// dropped here too, so a wrong list cannot inflate a count.
pub fn tally_payloads(
    open: &ValidOpen,
    members_of_h: &BTreeSet<IdentityKey>,
    payloads: impl IntoIterator<Item = (IdentityKey, u8)>,
) -> Tally {
    let n_opt = open.options().len();
    let mut per_key: BTreeMap<IdentityKey, BTreeSet<u8>> = BTreeMap::new();
    let mut rejected: usize = 0;
    for (k, c) in payloads {
        if members_of_h.contains(&k) && usize::from(c) < n_opt {
            per_key.entry(k).or_default().insert(c);
        } else {
            rejected = rejected.saturating_add(1);
        }
    }

    let mut s = vec![0u32; n_opt];
    let mut displayed = vec![0u32; n_opt];
    let mut void_keys = BTreeSet::new();
    for (k, choices) in &per_key {
        let void = choices.len() >= 2;
        if void {
            void_keys.insert(*k);
        }
        for &c in choices {
            if let Some(slot) = s.get_mut(usize::from(c)) {
                *slot = slot.saturating_add(1);
            }
            if !void && let Some(slot) = displayed.get_mut(usize::from(c)) {
                *slot = slot.saturating_add(1);
            }
        }
    }
    let heard = per_key.len();
    let not_heard: Vec<IdentityKey> = members_of_h
        .iter()
        .filter(|k| !per_key.contains_key(k))
        .copied()
        .collect();
    let k_not_heard = u16::try_from(not_heard.len()).unwrap_or(u16::MAX);

    let n = open.n();
    let t = open.t();
    let f_eff = open.f_eff();
    let state = if i64::try_from(void_keys.len()).unwrap_or(i64::MAX) > f_eff {
        VoteState::Disputed {
            members: void_keys.iter().copied().collect(),
        }
    } else if let Some(option) = s.iter().enumerate().find_map(|(i, &sx)| {
        let reaches_t = u64::from(sx) >= u64::from(t);
        let majority = n_opt <= 2 || u64::from(sx) * 2 > u64::from(n);
        (reaches_t && majority).then_some(i)
    }) {
        VoteState::Final {
            option: u8::try_from(option).unwrap_or(u8::MAX),
        }
    } else {
        let lead = leading(&displayed);
        if heard < usize::from(open.quorum()) {
            VoteState::LeadingProvisional {
                leading: lead,
                k_not_heard,
            }
        } else {
            VoteState::ResultProvisional {
                leading: lead,
                needs_t: t,
                k_not_heard,
            }
        }
    };

    Tally {
        s,
        displayed,
        void_keys,
        heard,
        not_heard,
        rejected,
        state,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::backend::GroupId;
    use crate::crypto::ed25519::IdentitySigner;
    use crate::crypto::hash::Hash32;
    use crate::objects::vote_open::{VoteOpen, threshold};
    use crate::objects::{SignedHeader, kind};

    fn signer(i: u8) -> IdentitySigner {
        IdentitySigner::from_seed(&[i; 32])
    }

    fn key(i: u8) -> IdentityKey {
        signer(i).public()
    }

    fn members(n: u8) -> BTreeSet<IdentityKey> {
        (1..=n).map(key).collect()
    }

    fn open(n: u16, f: u8, options: &[&str]) -> ValidOpen {
        let s = signer(1);
        let header = SignedHeader::new(
            kind::VOTE_OPEN,
            GroupId([1; 16]),
            s.public(),
            0,
            Hash32::ZERO,
            vec![],
        )
        .unwrap();
        let o = VoteOpen::sign(
            &s,
            header,
            "q",
            options.iter().map(|o| o.to_string()).collect(),
            f,
            threshold(n, f),
            n,
        )
        .unwrap();
        ValidOpen::trusted(o)
    }

    #[test]
    fn leading_and_gap_helpers() {
        assert_eq!(leading(&[]), None);
        assert_eq!(leading(&[0, 0]), None);
        assert_eq!(leading(&[3, 3]), None);
        assert_eq!(leading(&[2, 5, 1]), Some(1));
        assert_eq!(leading(&[5, 2]), Some(0));
        assert_eq!(top_two_gap(&[5, 2]), 3);
        assert_eq!(top_two_gap(&[2, 5, 4]), 1);
        assert_eq!(top_two_gap(&[7]), 0);
        assert_eq!(top_two_gap(&[]), 0);
    }

    #[test]
    fn empty_vote_is_leading_provisional_with_everyone_not_heard() {
        let o = open(12, 1, &["Yes", "No"]);
        let t = tally(&o, &members(12), &[]);
        assert_eq!(t.s, vec![0, 0]);
        assert_eq!(t.heard, 0);
        assert_eq!(t.not_heard.len(), 12);
        assert_eq!(
            t.state,
            VoteState::LeadingProvisional {
                leading: None,
                k_not_heard: 12
            }
        );
        assert!(t.state.is_provisional());
        assert_eq!(t.state.leading(), None);
    }

    #[test]
    fn signed_path_drops_bad_ballots_and_collapses_duplicates() {
        let o = open(12, 1, &["Yes", "No"]);
        let m = members(12);
        let vid = o.vote_id();
        let mut ballots = vec![
            Ballot::sign(&signer(1), vid, 0),
            Ballot::sign(&signer(1), vid, 0),
        ];
        ballots.push(Ballot::sign(&signer(2), vid, 1));
        ballots.push(Ballot::sign(&signer(30), vid, 0)); // not a member
        ballots.push(Ballot::sign(&signer(3), Hash32::from_bytes([9; 32]), 0)); // other vote
        ballots.push(Ballot::sign(&signer(4), vid, 2)); // bad choice
        let mut forged = Ballot::sign(&signer(5), vid, 0);
        forged.choice = 1;
        ballots.push(forged); // bad signature
        let t = tally(&o, &m, &ballots);
        assert_eq!(t.rejected, 4);
        assert_eq!(t.s, vec![1, 1]);
        assert_eq!(t.displayed, vec![1, 1]);
        assert_eq!(t.heard, 2);
        assert_eq!(t.not_heard.len(), 10);
        assert!(!t.not_heard.contains(&key(1)));
        assert_eq!(t.displayed_for(0), 1);
        assert_eq!(t.displayed_for(7), 0);
    }

    #[test]
    fn final_disputed_and_provisional_boundaries_n12_f1() {
        // T = 7, f_eff = 1, quorum = 7
        let o = open(12, 1, &["Yes", "No"]);
        let m = members(12);
        let yes = |n: u8| (1..=n).map(|i| (key(i), 0u8)).collect::<Vec<_>>();
        let t = tally_payloads(&o, &m, yes(6));
        assert_eq!(
            t.state,
            VoteState::LeadingProvisional {
                leading: Some(0),
                k_not_heard: 6
            }
        );
        let mut p = yes(6);
        p.push((key(7), 1));
        let t = tally_payloads(&o, &m, p);
        assert_eq!(
            t.state,
            VoteState::ResultProvisional {
                leading: Some(0),
                needs_t: 7,
                k_not_heard: 5
            }
        );
        let t = tally_payloads(&o, &m, yes(7));
        assert_eq!(t.state, VoteState::Final { option: 0 });
        // one equivocator among the seven still counts toward s, not displayed
        let mut p = yes(7);
        p.push((key(7), 1));
        let t = tally_payloads(&o, &m, p);
        assert_eq!(t.s, vec![7, 1]);
        assert_eq!(t.displayed, vec![6, 0]);
        assert_eq!(t.void_keys, BTreeSet::from([key(7)]));
        assert_eq!(t.state, VoteState::Final { option: 0 });
        // two void keys > f_eff → Disputed, naming them
        let mut p = yes(7);
        p.push((key(7), 1));
        p.push((key(6), 1));
        let t = tally_payloads(&o, &m, p);
        assert_eq!(
            t.state,
            VoteState::Disputed {
                members: vec![key(6), key(7)]
                    .into_iter()
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect()
            }
        );
        assert!(t.state.is_disputed());
    }

    #[test]
    fn more_than_two_options_need_an_absolute_majority() {
        // N = 12, f = 1, T = 7 > N/2: a plurality with full turnout stays
        // Provisional (BRIEF_v2 §4 "With more than 2 options, Final needs an
        // absolute majority"); the majority guard is implied by T for every
        // valid opening and only matters for a hand-built one.
        let o = open(12, 1, &["A", "B", "C"]);
        let m = members(12);
        let p: Vec<(IdentityKey, u8)> = (1..=5)
            .map(|i| (key(i), 0u8))
            .chain((6..=9).map(|i| (key(i), 1u8)))
            .chain((10..=12).map(|i| (key(i), 2u8)))
            .collect();
        let t = tally_payloads(&o, &m, p);
        assert_eq!(
            t.state,
            VoteState::ResultProvisional {
                leading: Some(0),
                needs_t: 7,
                k_not_heard: 0
            }
        );
        let p: Vec<(IdentityKey, u8)> = (1..=7)
            .map(|i| (key(i), 0u8))
            .chain((8..=12).map(|i| (key(i), 2u8)))
            .collect();
        assert_eq!(
            tally_payloads(&o, &m, p).state,
            VoteState::Final { option: 0 }
        );
    }

    #[test]
    fn a_plurality_is_never_final_whatever_t_the_opening_carries() {
        // BRIEF_v2 §4 "With more than 2 options, Final needs an absolute
        // majority; otherwise the result stays Provisional". The guard is
        // implied twice over: `validate_open` fixes T = ⌊(N+f)/2⌋+1 > N/2,
        // and even a hand-built (trusted) opening with T ≤ N/2 has
        // f_eff = 2T−N−1 < 0, so `void ≤ f_eff` can never hold and nothing
        // is Final. This pins that a plurality cannot sneak through.
        let s = signer(1);
        let m = members(12);
        for t_carried in [1u16, 5, 6] {
            let header = SignedHeader::new(
                kind::VOTE_OPEN,
                GroupId([1; 16]),
                s.public(),
                u32::from(t_carried),
                Hash32::ZERO,
                vec![],
            )
            .unwrap();
            let o = ValidOpen::trusted(
                VoteOpen::sign(
                    &s,
                    header,
                    "q",
                    vec!["A".into(), "B".into(), "C".into()],
                    1,
                    t_carried,
                    12,
                )
                .unwrap(),
            );
            assert!(o.f_eff() < 0);
            let p: Vec<(IdentityKey, u8)> = (1..=5)
                .map(|i| (key(i), 0u8))
                .chain((6..=9).map(|i| (key(i), 1u8)))
                .chain((10..=12).map(|i| (key(i), 2u8)))
                .collect();
            let t = tally_payloads(&o, &m, p);
            assert_eq!(t.s, vec![5, 4, 3]);
            assert!(!t.state.is_final(), "T={t_carried}: {:?}", t.state);
            let p: Vec<(IdentityKey, u8)> = (1..=6)
                .map(|i| (key(i), 0u8))
                .chain((7..=12).map(|i| (key(i), 1u8)))
                .collect();
            assert!(!tally_payloads(&o, &m, p).state.is_final());
        }
        // the smallest T a valid opening can carry (N=12, f=1 → 7) is a majority
        let o = open(12, 1, &["A", "B", "C"]);
        assert_eq!(o.t(), 7);
        assert!(u32::from(o.t()) * 2 > u32::from(o.n()));
        assert!(o.f_eff() >= 0);
    }

    #[test]
    fn payload_path_rejects_non_members_and_bad_choices() {
        let o = open(3, 1, &["Yes", "No"]);
        let t = tally_payloads(&o, &members(3), [(key(9), 0), (key(1), 5), (key(1), 0)]);
        assert_eq!(t.rejected, 2);
        assert_eq!(t.s, vec![1, 0]);
    }
}
