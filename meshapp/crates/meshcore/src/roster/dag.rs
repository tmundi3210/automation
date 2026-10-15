//! Roster DAG storage (docs/DESIGN.md §3 "Node = a roster op; edges =
//! `heads`"; DECISIONS.md D3 M1 "The roster is a hash-linked DAG of ops
//! signed with identity keys ... Roster ops ... are never evicted").
//!
//! [`RosterDag`] holds every op of one group that has been **applied**, i.e.
//! whose signatures verified ([`RosterOp::verify`]) and whose cited heads are
//! all applied themselves. An op whose head is unknown is **parked** and
//! applied automatically when the last missing head arrives; the missing
//! hashes are what anti-entropy asks peers for (DESIGN §3 "ops with missing
//! heads are parked and their parents requested through summary vectors").
//! Rejected ops (bad signature, wrong group, unknown kind) are dropped.
//!
//! The DAG stores and indexes; it never judges authority. `roster::eval`
//! computes `members(H)` from [`RosterDag::closure`], which is the only
//! thing membership depends on (D2/§3-3: "members(H) is a pure function of
//! the op set closure(H)").

use std::collections::{BTreeMap, BTreeSet};

use crate::crypto::backend::GroupId;
use crate::crypto::hash::Hash32;
use crate::objects::roster_op::{RosterOp, RosterOpError};

/// Most ops that may wait for missing heads at once. Beyond this, further
/// ops with unknown heads are rejected until some parked op resolves
/// (bounded memory for untrusted input).
pub const MAX_PARKED: usize = 4096;

/// Why [`RosterDag::insert`] dropped an op.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RejectReason {
    /// Format or signature failure, from [`RosterOp::verify`].
    #[error("invalid roster op: {0}")]
    Invalid(RosterOpError),
    /// The op's group id (derived for a genesis) is not this DAG's group.
    #[error("roster op belongs to another group")]
    WrongGroup,
    /// Too many ops are already waiting for missing heads.
    #[error("too many parked roster ops")]
    ParkedFull,
}

/// Outcome of [`RosterDag::insert`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Insert {
    /// The op (or an identical copy already held) is applied; the hash is
    /// its `op_hash`. Parked ops that this one unblocked are applied too;
    /// see [`RosterDag::drain_applied`].
    Applied(Hash32),
    /// The op verified but cites heads this DAG does not hold yet.
    Parked { missing: Vec<Hash32> },
    /// The op was dropped.
    Rejected(RejectReason),
}

#[derive(Debug, Clone)]
struct Parked {
    op: RosterOp,
    missing: BTreeSet<Hash32>,
}

/// One group's roster DAG (see the module docs).
#[derive(Debug, Clone)]
pub struct RosterDag {
    group: GroupId,
    genesis: Option<Hash32>,
    ops: BTreeMap<Hash32, RosterOp>,
    /// Applied op → applied ops that cite it.
    children: BTreeMap<Hash32, BTreeSet<Hash32>>,
    parked: BTreeMap<Hash32, Parked>,
    /// Missing head → parked ops waiting for it.
    waiting_on: BTreeMap<Hash32, BTreeSet<Hash32>>,
    /// Hashes applied since the last [`RosterDag::drain_applied`].
    applied_log: Vec<Hash32>,
}

impl RosterDag {
    /// An empty DAG for `group`. The genesis arrives through
    /// [`RosterDag::insert`] like any other op (a phone may hold later ops
    /// before it holds the genesis).
    pub fn new(group: GroupId) -> Self {
        Self {
            group,
            genesis: None,
            ops: BTreeMap::new(),
            children: BTreeMap::new(),
            parked: BTreeMap::new(),
            waiting_on: BTreeMap::new(),
            applied_log: Vec::new(),
        }
    }

    /// A DAG created from its genesis op (the group id is derived from it).
    pub fn from_genesis(genesis: RosterOp) -> Result<Self, RejectReason> {
        if !genesis.is_genesis() {
            return Err(RejectReason::WrongGroup);
        }
        genesis.verify().map_err(RejectReason::Invalid)?;
        let mut dag = Self::new(genesis.group_id());
        match dag.insert(genesis) {
            Insert::Applied(_) => Ok(dag),
            Insert::Parked { .. } => Err(RejectReason::WrongGroup),
            Insert::Rejected(r) => Err(r),
        }
    }

    /// The group this DAG belongs to.
    pub fn group(&self) -> GroupId {
        self.group
    }

    /// The genesis op, once applied.
    pub fn genesis(&self) -> Option<&RosterOp> {
        self.genesis.and_then(|h| self.ops.get(&h))
    }

    /// Hash of the genesis op, once applied.
    pub fn genesis_hash(&self) -> Option<Hash32> {
        self.genesis
    }

    /// Verifies and stores an op; see [`Insert`].
    pub fn insert(&mut self, op: RosterOp) -> Insert {
        if let Err(e) = op.verify() {
            return Insert::Rejected(RejectReason::Invalid(e));
        }
        if op.group_id() != self.group {
            return Insert::Rejected(RejectReason::WrongGroup);
        }
        let hash = op.op_hash();
        if self.ops.contains_key(&hash) {
            return Insert::Applied(hash);
        }
        if let Some(p) = self.parked.get(&hash) {
            return Insert::Parked {
                missing: p.missing.iter().copied().collect(),
            };
        }
        let missing: BTreeSet<Hash32> = op
            .header
            .heads
            .iter()
            .copied()
            .filter(|h| !self.ops.contains_key(h))
            .collect();
        if missing.is_empty() {
            self.apply(hash, op);
            self.cascade(hash);
            return Insert::Applied(hash);
        }
        if self.parked.len() >= MAX_PARKED {
            return Insert::Rejected(RejectReason::ParkedFull);
        }
        let out: Vec<Hash32> = missing.iter().copied().collect();
        for m in &missing {
            self.waiting_on.entry(*m).or_default().insert(hash);
        }
        self.parked.insert(hash, Parked { op, missing });
        Insert::Parked { missing: out }
    }

    fn apply(&mut self, hash: Hash32, op: RosterOp) {
        if op.is_genesis() {
            self.genesis = Some(hash);
        }
        for h in &op.header.heads {
            self.children.entry(*h).or_default().insert(hash);
        }
        self.ops.insert(hash, op);
        self.applied_log.push(hash);
    }

    /// Applies every parked op whose last missing head just arrived,
    /// transitively.
    fn cascade(&mut self, arrived: Hash32) {
        let mut queue = vec![arrived];
        while let Some(h) = queue.pop() {
            let Some(waiters) = self.waiting_on.remove(&h) else {
                continue;
            };
            for w in waiters {
                let ready = match self.parked.get_mut(&w) {
                    Some(p) => {
                        p.missing.remove(&h);
                        p.missing.is_empty()
                    }
                    None => false,
                };
                if ready {
                    if let Some(p) = self.parked.remove(&w) {
                        self.apply(w, p.op);
                        queue.push(w);
                    }
                }
            }
        }
    }

    /// Hashes applied (directly or by unparking) since the last call.
    pub fn drain_applied(&mut self) -> Vec<Hash32> {
        std::mem::take(&mut self.applied_log)
    }

    /// An applied op by hash.
    pub fn get(&self, hash: &Hash32) -> Option<&RosterOp> {
        self.ops.get(hash)
    }

    /// Is `hash` applied?
    pub fn contains(&self, hash: &Hash32) -> bool {
        self.ops.contains_key(hash)
    }

    /// Number of applied ops.
    pub fn len(&self) -> usize {
        self.ops.len()
    }

    /// No op applied yet.
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    /// Number of parked ops.
    pub fn parked_len(&self) -> usize {
        self.parked.len()
    }

    /// All applied ops, in hash order.
    pub fn ops(&self) -> impl Iterator<Item = (&Hash32, &RosterOp)> {
        self.ops.iter()
    }

    /// Applied ops that cite `hash` as a head.
    pub fn children(&self, hash: &Hash32) -> Vec<Hash32> {
        self.children
            .get(hash)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default()
    }

    /// The frontier: applied ops not cited by any other applied op, sorted.
    /// This is the head set a new op by this phone cites (DESIGN §2.4
    /// "`heads` are the roster-DAG heads the author had seen").
    pub fn heads(&self) -> Vec<Hash32> {
        self.ops
            .keys()
            .filter(|h| self.children.get(*h).is_none_or(BTreeSet::is_empty))
            .copied()
            .collect()
    }

    /// Heads that parked ops are waiting for and that this DAG holds neither
    /// applied nor parked, sorted: what to request from peers (DESIGN §3
    /// "missing-op requests"). A head that is itself parked is not missing,
    /// only blocked.
    pub fn missing(&self) -> Vec<Hash32> {
        self.waiting_on
            .keys()
            .filter(|h| !self.parked.contains_key(*h))
            .copied()
            .collect()
    }

    /// Every applied op reachable from `heads` (the heads included), computed
    /// only from ops this DAG holds; unknown heads are skipped (see
    /// [`RosterDag::closure_with_unresolved`] to learn which).
    pub fn closure(&self, heads: &[Hash32]) -> BTreeSet<Hash32> {
        self.closure_with_unresolved(heads).0
    }

    /// [`RosterDag::closure`] plus the heads (and only the heads) that this
    /// DAG does not hold. Membership computed from an unresolved head set is
    /// provisional (DESIGN §5 "H unresolved → Waiting · needs this vote's
    /// member list").
    pub fn closure_with_unresolved(&self, heads: &[Hash32]) -> (BTreeSet<Hash32>, Vec<Hash32>) {
        let mut seen = BTreeSet::new();
        let mut unresolved = Vec::new();
        let mut stack: Vec<Hash32> = Vec::new();
        for h in heads {
            if self.ops.contains_key(h) {
                stack.push(*h);
            } else {
                unresolved.push(*h);
            }
        }
        while let Some(h) = stack.pop() {
            if !seen.insert(h) {
                continue;
            }
            if let Some(op) = self.ops.get(&h) {
                for p in &op.header.heads {
                    if !seen.contains(p) {
                        stack.push(*p);
                    }
                }
            }
        }
        unresolved.sort_unstable();
        unresolved.dedup();
        (seen, unresolved)
    }

    /// Are all of `heads` applied?
    pub fn resolves(&self, heads: &[Hash32]) -> bool {
        heads.iter().all(|h| self.ops.contains_key(h))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::ed25519::IdentitySigner;
    use crate::crypto::keycard::KeyCard;
    use crate::objects::roster_op::{MemberEntry, RosterBody};
    use crate::objects::{SignedHeader, kind};

    fn signer(i: u8) -> IdentitySigner {
        IdentitySigner::from_seed(&[i; 32])
    }

    fn card(i: u8) -> KeyCard {
        KeyCard::issue(&signer(i), [i; 32])
    }

    fn genesis() -> RosterOp {
        let s = signer(1);
        RosterOp::sign(
            &s,
            SignedHeader::new(
                kind::ROSTER_GENESIS,
                GroupId([0; 16]),
                s.public(),
                0,
                Hash32::ZERO,
                vec![],
            )
            .unwrap(),
            RosterBody::Genesis {
                nonce: [1; 16],
                name: "g".into(),
                admins: vec![card(1), card(2)],
                members: vec![MemberEntry {
                    keycard: card(3),
                    name: "three".into(),
                    in_person: true,
                }],
                settings: 0,
            },
        )
        .unwrap()
    }

    fn setting(author: u8, seq: u32, gid: GroupId, heads: Vec<Hash32>, value: u8) -> RosterOp {
        let s = signer(author);
        RosterOp::sign(
            &s,
            SignedHeader::new(
                kind::ROSTER_SETTING,
                gid,
                s.public(),
                seq,
                Hash32::ZERO,
                heads,
            )
            .unwrap(),
            RosterBody::Setting { key: 0, value },
        )
        .unwrap()
    }

    #[test]
    fn genesis_then_chain_applies_and_tracks_heads() {
        let g = genesis();
        let gid = g.group_id();
        let gh = g.op_hash();
        let mut dag = RosterDag::new(gid);
        assert!(dag.is_empty());
        assert_eq!(dag.insert(g.clone()), Insert::Applied(gh));
        assert_eq!(dag.insert(g.clone()), Insert::Applied(gh));
        assert_eq!(dag.genesis_hash(), Some(gh));
        assert_eq!(dag.genesis(), Some(&g));
        assert_eq!(dag.heads(), vec![gh]);
        let a = setting(1, 1, gid, vec![gh], 1);
        let b = setting(2, 1, gid, vec![gh], 2);
        let ah = a.op_hash();
        let bh = b.op_hash();
        assert_eq!(dag.insert(a), Insert::Applied(ah));
        assert_eq!(dag.insert(b), Insert::Applied(bh));
        let mut expect = vec![ah, bh];
        expect.sort_unstable();
        assert_eq!(dag.heads(), expect);
        let c = setting(1, 2, gid, vec![ah, bh], 3);
        let ch = c.op_hash();
        assert_eq!(dag.insert(c), Insert::Applied(ch));
        assert_eq!(dag.heads(), vec![ch]);
        assert_eq!(dag.len(), 4);
        assert_eq!(dag.children(&gh).len(), 2);
        assert_eq!(dag.closure(&[ah]), [gh, ah].into_iter().collect());
        assert_eq!(dag.closure(&[ch]).len(), 4);
        assert_eq!(dag.drain_applied(), vec![gh, ah, bh, ch]);
        assert!(dag.drain_applied().is_empty());
        assert_eq!(dag.group(), gid);
        assert!(dag.contains(&ch));
    }

    #[test]
    fn child_before_parent_is_parked_then_applied() {
        let g = genesis();
        let gid = g.group_id();
        let gh = g.op_hash();
        let a = setting(1, 1, gid, vec![gh], 1);
        let ah = a.op_hash();
        let c = setting(1, 2, gid, vec![ah], 2);
        let ch = c.op_hash();
        let mut dag = RosterDag::new(gid);
        assert_eq!(dag.insert(c.clone()), Insert::Parked { missing: vec![ah] });
        assert_eq!(dag.insert(c), Insert::Parked { missing: vec![ah] });
        assert_eq!(dag.parked_len(), 1);
        assert_eq!(dag.missing(), vec![ah]);
        assert_eq!(dag.insert(a), Insert::Parked { missing: vec![gh] });
        assert_eq!(dag.missing(), vec![gh]);
        assert!(dag.heads().is_empty());
        assert_eq!(dag.insert(g), Insert::Applied(gh));
        assert_eq!(dag.parked_len(), 0);
        assert!(dag.missing().is_empty());
        assert_eq!(dag.heads(), vec![ch]);
        assert_eq!(dag.drain_applied(), vec![gh, ah, ch]);
    }

    #[test]
    fn rejects_bad_signature_wrong_group_and_unknown_kind() {
        let g = genesis();
        let gid = g.group_id();
        let gh = g.op_hash();
        let mut dag = RosterDag::from_genesis(g.clone()).unwrap();
        let mut bad = setting(1, 1, gid, vec![gh], 1);
        bad.sig[0] ^= 1;
        assert_eq!(
            dag.insert(bad),
            Insert::Rejected(RejectReason::Invalid(RosterOpError::BadSignature))
        );
        let other = setting(1, 1, GroupId([9; 16]), vec![gh], 1);
        assert_eq!(
            dag.insert(other),
            Insert::Rejected(RejectReason::WrongGroup)
        );
        let mut unknown = setting(1, 1, gid, vec![gh], 1);
        unknown.header.kind = 0x7f;
        assert!(matches!(
            dag.insert(unknown),
            Insert::Rejected(RejectReason::Invalid(RosterOpError::UnknownKind(0x7f)))
        ));
        // a second genesis is a different group
        let s = signer(2);
        let g2 = RosterOp::sign(
            &s,
            SignedHeader::new(
                kind::ROSTER_GENESIS,
                GroupId([0; 16]),
                s.public(),
                0,
                Hash32::ZERO,
                vec![],
            )
            .unwrap(),
            RosterBody::Genesis {
                nonce: [2; 16],
                name: "other".into(),
                admins: vec![card(2)],
                members: vec![],
                settings: 0,
            },
        )
        .unwrap();
        assert_eq!(dag.insert(g2), Insert::Rejected(RejectReason::WrongGroup));
        assert_eq!(dag.len(), 1);
        assert!(RosterDag::from_genesis(setting(1, 1, gid, vec![gh], 1)).is_err());
    }

    #[test]
    fn closure_reports_unresolved_heads() {
        let g = genesis();
        let gh = g.op_hash();
        let dag = RosterDag::from_genesis(g).unwrap();
        let ghost = Hash32::from_bytes([0xee; 32]);
        let (set, unresolved) = dag.closure_with_unresolved(&[gh, ghost, ghost]);
        assert_eq!(set, [gh].into_iter().collect());
        assert_eq!(unresolved, vec![ghost]);
        assert!(!dag.resolves(&[gh, ghost]));
        assert!(dag.resolves(&[gh]));
        assert!(dag.closure(&[]).is_empty());
    }

    #[test]
    fn parking_is_bounded() {
        let g = genesis();
        let gid = g.group_id();
        let mut dag = RosterDag::new(gid);
        for i in 0..MAX_PARKED {
            let mut b = [0u8; 32];
            b[..8].copy_from_slice(&(i as u64).to_le_bytes());
            b[31] = 1;
            let op = setting(1, i as u32, gid, vec![Hash32::from_bytes(b)], 0);
            assert!(matches!(dag.insert(op), Insert::Parked { .. }));
        }
        let op = setting(2, 0, gid, vec![Hash32::from_bytes([0xaa; 32])], 0);
        assert_eq!(dag.insert(op), Insert::Rejected(RejectReason::ParkedFull));
        assert_eq!(dag.parked_len(), MAX_PARKED);
    }
}
