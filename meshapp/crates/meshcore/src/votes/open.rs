//! Vote-opening validity (docs/DESIGN.md §5 "VoteOpen"; BRIEF_v2 §4
//! "Votes · Opening"; DECISIONS D1 rule 1, D2 rules 1, 4–5).
//!
//! Pure: inputs are the signed opening, `members(H)` as the roster module
//! evaluated it over `closure(H)` (DECISIONS §3 row (1,2,3,7): "members(H)
//! = M3 evaluated over closure(H) only, so later concurrent ops never
//! change it"), the build's branch, and two verdicts the sender-chain index
//! supplies. Nothing here reads a clock.
//!
//! An opening is valid everywhere iff all of:
//! - its signature verifies (`VoteOpen::verify`);
//! - it is not a provable fork: no earlier signed object by the opener
//!   cites a head outside `closure(H)` (DESIGN §3 "Vote-open fork rule
//!   hook"; D2 rule 5) — the caller passes that evidence as `fork_evidence`;
//! - it is not an equivocation: no other opening by the same opener with
//!   the same `author_seq` and different content (D1 rule 1; voids the
//!   vote with the alert icon);
//! - the opener ∈ `members(H)` (D2 rule 1);
//! - `N == |members(H)|` and `T == threshold(N, f)` recomputed locally
//!   (DESIGN §5), so every phone uses the same T;
//! - `f ≥ F_MIN` of the branch: 1 under A, 2 under B (D1; §3 row (1,2,5,6)).
//!
//! `H` unresolved is not a rejection: the caller shows "Waiting · needs
//! this vote's member list" (`votes::state`) and retries when the ops
//! arrive. "Duplicate vote" (same opener, same question hash, another vote
//! still open) is a flag, not a rejection ([`duplicate_question`]).

use std::collections::BTreeSet;

use crate::crypto::ed25519::IdentityKey;
use crate::crypto::hash::Hash32;
use crate::objects::sender_chain::ChainVerdict;
use crate::objects::vote_open::{MIN_OPTIONS, VoteOpen, threshold};

/// Which build this phone runs (BRIEF_v2 §0; D1 "f_min=1 under A, 2 under B").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Branch {
    A,
    B,
}

/// Smallest tolerance an opening may carry under branch A.
pub const F_MIN_A: u8 = 1;
/// Smallest tolerance an opening may carry under branch B.
pub const F_MIN_B: u8 = 2;

/// `F_MIN` of a branch.
pub fn f_min(branch: Branch) -> u8 {
    match branch {
        Branch::A => F_MIN_A,
        Branch::B => F_MIN_B,
    }
}

/// What the sender-chain index says about the opening's `(opener,
/// author_seq)`: the `ChainVerdict` collapsed to the two cases that matter
/// here (a `Gap` records the object and is treated as `Ok`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChainCheck {
    Ok,
    /// Another opening by the same opener with the same seq and different
    /// content is known.
    Equivocation,
}

impl From<ChainVerdict> for ChainCheck {
    fn from(v: ChainVerdict) -> Self {
        match v {
            ChainVerdict::Ok | ChainVerdict::Gap { .. } => ChainCheck::Ok,
            ChainVerdict::Equivocation { .. } => ChainCheck::Equivocation,
        }
    }
}

impl From<&ChainVerdict> for ChainCheck {
    fn from(v: &ChainVerdict) -> Self {
        ChainCheck::from(*v)
    }
}

/// Why an opening is invalid (or void) on this phone, and on every other
/// phone that holds the same inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum OpenReject {
    #[error("opener is not in members(H)")]
    OpenerNotMember,
    #[error("N {got} does not equal |members(H)| = {expected}")]
    BadN { expected: u16, got: u16 },
    #[error("T {got} does not equal threshold(N, f) = {expected}")]
    BadT { expected: u16, got: u16 },
    #[error("f {got} is below F_MIN {min}")]
    FBelowMin { min: u8, got: u8 },
    #[error("fewer than {MIN_OPTIONS} options")]
    FewOptions,
    /// Provable fork: invalid everywhere once seen (D2 rule 5).
    #[error("opening cites a roster head older than one the opener already carried")]
    Fork,
    /// Same opener and seq, different content: void with the alert icon (D1 rule 1).
    #[error("two openings with the same opener and sequence number")]
    Equivocated,
    #[error("opening signature does not verify")]
    Signature,
}

/// An opening that passed [`validate_open`]: the only input `tally` accepts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidOpen {
    open: VoteOpen,
    vote_id: Hash32,
}

impl ValidOpen {
    /// Wraps an opening without re-running the checks. For replaying an
    /// opening this phone already validated when it was stored, and for
    /// tests of the pure tally; never for an opening fresh off the air.
    pub fn trusted(open: VoteOpen) -> Self {
        let vote_id = open.try_vote_id().unwrap_or(Hash32::ZERO);
        ValidOpen { open, vote_id }
    }

    pub fn open(&self) -> &VoteOpen {
        &self.open
    }

    pub fn vote_id(&self) -> Hash32 {
        self.vote_id
    }

    pub fn opener(&self) -> &IdentityKey {
        self.open.opener()
    }

    pub fn n(&self) -> u16 {
        self.open.n
    }

    pub fn t(&self) -> u16 {
        self.open.t
    }

    pub fn f(&self) -> u8 {
        self.open.f
    }

    pub fn options(&self) -> &[String] {
        &self.open.options
    }

    pub fn question(&self) -> &str {
        &self.open.question
    }

    /// `f_eff = 2T − N − 1` (D1 rule 4), the most void keys a Final
    /// tolerates. Signed so a hand-built opening with a bad `T` cannot
    /// underflow; `≥ f ≥ 1` for every valid opening.
    pub fn f_eff(&self) -> i64 {
        2 * i64::from(self.open.t) - i64::from(self.open.n) - 1
    }

    /// `⌊N/2⌋ + 1`: the turnout that separates "Leading" from "Result
    /// (provisional)" (D1 rule 5).
    pub fn quorum(&self) -> u16 {
        self.open.n / 2 + 1
    }
}

/// Validates an opening (module docs). Checks run in the order listed so
/// the most severe verdict (signature, fork, equivocation) wins.
pub fn validate_open(
    open: &VoteOpen,
    members_of_h: &BTreeSet<IdentityKey>,
    branch: Branch,
    fork_evidence: bool,
    chain: ChainCheck,
) -> Result<ValidOpen, OpenReject> {
    if open.verify().is_err() {
        return Err(OpenReject::Signature);
    }
    if fork_evidence {
        return Err(OpenReject::Fork);
    }
    if chain == ChainCheck::Equivocation {
        return Err(OpenReject::Equivocated);
    }
    if !members_of_h.contains(open.opener()) {
        return Err(OpenReject::OpenerNotMember);
    }
    if open.options.len() < MIN_OPTIONS {
        return Err(OpenReject::FewOptions);
    }
    let min = f_min(branch);
    if open.f < min {
        return Err(OpenReject::FBelowMin { min, got: open.f });
    }
    let expected_n = u16::try_from(members_of_h.len()).unwrap_or(u16::MAX);
    if open.n != expected_n {
        return Err(OpenReject::BadN {
            expected: expected_n,
            got: open.n,
        });
    }
    let expected_t = threshold(open.n, open.f);
    if open.t != expected_t {
        return Err(OpenReject::BadT {
            expected: expected_t,
            got: open.t,
        });
    }
    let vote_id = open.try_vote_id().map_err(|_| OpenReject::Signature)?;
    Ok(ValidOpen {
        open: open.clone(),
        vote_id,
    })
}

/// "Duplicate vote" flag (D1 rule 1; DESIGN §5): `open` repeats the
/// question hash of another vote by the same opener that is still open.
/// The caller passes the opener's other open votes; a different opener or
/// a different question never matches, and the opening itself (same vote
/// id) is skipped.
pub fn duplicate_question<'a>(
    open: &VoteOpen,
    open_votes_by_same_opener: impl IntoIterator<Item = &'a VoteOpen>,
) -> bool {
    let qh = open.question_hash();
    let own_id = open.try_vote_id().ok();
    open_votes_by_same_opener.into_iter().any(|other| {
        other.opener() == open.opener()
            && other.question_hash() == qh
            && other.try_vote_id().ok() != own_id
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::backend::GroupId;
    use crate::crypto::ed25519::IdentitySigner;
    use crate::objects::{SignedHeader, kind};

    fn signer(i: u8) -> IdentitySigner {
        IdentitySigner::from_seed(&[i; 32])
    }

    fn members(n: u8) -> BTreeSet<IdentityKey> {
        (1..=n).map(|i| signer(i).public()).collect()
    }

    fn open_with(s: &IdentitySigner, seq: u32, q: &str, f: u8, t: u16, n: u16) -> VoteOpen {
        let header = SignedHeader::new(
            kind::VOTE_OPEN,
            GroupId([1; 16]),
            s.public(),
            seq,
            Hash32::ZERO,
            vec![Hash32::from_bytes([2; 32])],
        )
        .unwrap();
        VoteOpen::sign(s, header, q, vec!["Yes".into(), "No".into()], f, t, n).unwrap()
    }

    #[test]
    fn valid_opening_passes_and_exposes_parameters() {
        let s = signer(1);
        let o = open_with(&s, 0, "q", 1, threshold(12, 1), 12);
        let v = validate_open(&o, &members(12), Branch::A, false, ChainCheck::Ok).unwrap();
        assert_eq!(v.n(), 12);
        assert_eq!(v.t(), 7);
        assert_eq!(v.f(), 1);
        assert_eq!(v.f_eff(), 1);
        assert_eq!(v.quorum(), 7);
        assert_eq!(v.vote_id(), o.vote_id());
        assert_eq!(v.options(), &["Yes".to_string(), "No".to_string()]);
        assert_eq!(v.question(), "q");
        assert_eq!(*v.opener(), s.public());
        assert_eq!(ValidOpen::trusted(o.clone()), v);
        assert_eq!(v.open(), &o);
    }

    #[test]
    fn every_rejection_in_severity_order() {
        let s = signer(1);
        let m = members(12);
        let good = open_with(&s, 0, "q", 1, 7, 12);
        let mut forged = good.clone();
        forged.n = 11;
        assert_eq!(
            validate_open(&forged, &m, Branch::A, true, ChainCheck::Equivocation),
            Err(OpenReject::Signature)
        );
        assert_eq!(
            validate_open(&good, &m, Branch::A, true, ChainCheck::Equivocation),
            Err(OpenReject::Fork)
        );
        assert_eq!(
            validate_open(&good, &m, Branch::A, false, ChainCheck::Equivocation),
            Err(OpenReject::Equivocated)
        );
        assert_eq!(
            validate_open(&good, &members(11), Branch::A, false, ChainCheck::Ok),
            Err(OpenReject::BadN {
                expected: 11,
                got: 12
            })
        );
        let outsider = open_with(&signer(40), 0, "q", 1, 7, 12);
        assert_eq!(
            validate_open(&outsider, &m, Branch::A, false, ChainCheck::Ok),
            Err(OpenReject::OpenerNotMember)
        );
        let low_f = open_with(&s, 0, "q", 1, 7, 12);
        assert_eq!(
            validate_open(&low_f, &m, Branch::B, false, ChainCheck::Ok),
            Err(OpenReject::FBelowMin { min: 2, got: 1 })
        );
        let zero_f = open_with(&s, 0, "q", 0, 7, 12);
        assert_eq!(
            validate_open(&zero_f, &m, Branch::A, false, ChainCheck::Ok),
            Err(OpenReject::FBelowMin { min: 1, got: 0 })
        );
        let bad_t = open_with(&s, 0, "q", 1, 8, 12);
        assert_eq!(
            validate_open(&bad_t, &m, Branch::A, false, ChainCheck::Ok),
            Err(OpenReject::BadT {
                expected: 7,
                got: 8
            })
        );
        // raising f is allowed, in the opening only
        let raised = open_with(&s, 0, "q", 3, threshold(12, 3), 12);
        assert!(validate_open(&raised, &m, Branch::B, false, ChainCheck::Ok).is_ok());
        let b_ok = open_with(&s, 0, "q", 2, 8, 12);
        assert!(validate_open(&b_ok, &m, Branch::B, false, ChainCheck::Ok).is_ok());
    }

    #[test]
    fn chain_verdict_collapses() {
        assert_eq!(ChainCheck::from(ChainVerdict::Ok), ChainCheck::Ok);
        assert_eq!(
            ChainCheck::from(ChainVerdict::Gap { expected_seq: 3 }),
            ChainCheck::Ok
        );
        assert_eq!(
            ChainCheck::from(&ChainVerdict::Equivocation {
                other: Hash32::ZERO
            }),
            ChainCheck::Equivocation
        );
        assert_eq!(f_min(Branch::A), 1);
        assert_eq!(f_min(Branch::B), 2);
    }

    #[test]
    fn duplicate_question_flag() {
        let s = signer(1);
        let a = open_with(&s, 0, "Move the kitchen?", 1, 7, 12);
        let b = open_with(&s, 1, "Move the kitchen?", 1, 7, 12);
        let c = open_with(&s, 2, "Buy a tent?", 1, 7, 12);
        let other = open_with(&signer(2), 0, "Move the kitchen?", 1, 7, 12);
        assert!(duplicate_question(&b, [&a, &c]));
        assert!(!duplicate_question(&c, [&a, &b]));
        assert!(!duplicate_question(&a, [&a]));
        assert!(!duplicate_question(&other, [&a, &b]));
        assert!(!duplicate_question(&a, []));
    }
}
