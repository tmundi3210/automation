//! Pure roster evaluation: `members(H)`, `admins(H)` and the per-op
//! verdicts of docs/DESIGN.md §3 ("Validity, evaluated against each op's own
//! causal past"), DECISIONS.md D3 M2 (authority) and M3 (merge rules),
//! BRIEF_v2 §5.4 ("Removals win, and no timestamps are used").
//!
//! [`evaluate`] is a function of the op *set* `S = closure(heads)` only
//! (DESIGN §3 "`members(H)` is a pure function of the op set `closure(H)`
//! (D2/§3-3): later ops never change it"). Every op is judged against its
//! own causal past `P(op) = closure(op.heads)` for authority, and against
//! `S` for the merge rules (remove-wins, "Changed after reconnect",
//! equivocation). Nothing here reads a clock or depends on insertion order:
//! the DAG is walked in a topological order with ties broken by op hash, and
//! every decision is a set query, so any order yields the same view.
//!
//! Rules implemented, by DESIGN §3 table row:
//!
//! * **Genesis** — exactly one per group (the group id is its hash, so the
//!   DAG holds at most one); its admins are the first admin set and its
//!   members (and admins) the first member set.
//! * **Remove(m)** — author ∈ `admins(P)`, m ∉ `admins(P)` (demote first, D3
//!   Q5); **monotone**: a valid removal stands even if its author is removed
//!   or demoted concurrently or later (D3 M3).
//! * **Add(m)** — author ∈ `admins(P)`, endorser ∈ `admins(P)`, endorser ≠
//!   author, `in_person` on at least one of the two (BRIEF_v2 §5.4,
//!   decisions-assumed #8), m ∉ `admins(P)`; **remove-wins**: every valid
//!   `Remove(m)` in `S` must be in `P` (a re-add is valid only causally after
//!   the removal); **voided** ("Changed after reconnect", D3 M3) when `S`
//!   holds a valid Remove or demotion of the author or the endorser that is
//!   *concurrent* with the Add (see "Decisions" below).
//! * **AdminSet(v)** — `v = 1 + |applied AdminSets in P|`, cosigners ⊆
//!   `admins(P)`, author ∈ cosigners, strict majority (`2·|cosigners| >
//!   |admins(P)|`), every named admin ∈ `members(P)`; **equivocation**: two
//!   otherwise-valid AdminSets with the same `v` sharing a cosigner anywhere
//!   in `S` void each other (D3 M2 "Each admin device signs at most one change
//!   per admin-set version").
//! * **Setting** — author ∈ `admins(P)`; last writer wins, concurrent writers
//!   ordered by op hash.
//! * **any** — sender-chain equivocation (same `(author, author_seq)`,
//!   different hash, both in `S`) voids both objects (DESIGN §3 "any" row);
//!   see "Decisions" for the two exemptions.
//!
//! `admins(P)` = genesis admins, then the applied AdminSets of `P` in version
//! order; it is memoised per op hash ([`RosterView::admins_at`]) as DESIGN
//! §3 asks ("Memoise `admins(P)` per op hash").
//!
//! # Decisions the spec left open (recorded here, see docs/decisions-assumed.md)
//!
//! 1. "Changed after reconnect" voids an Add only for a Remove/demotion of
//!    its author or endorser that is **concurrent** with the Add (neither
//!    in the other's past). DESIGN §3 says "not in P", which read literally
//!    would also void every Add an admin ever made once that admin is
//!    legitimately removed later; D3 M3 names the label "Changed after
//!    reconnect", i.e. a merge of concurrent fragments, and every S3 attack
//!    (an Add that omits the removal from its heads) is concurrent, so the
//!    narrower reading keeps the security property without evicting half a
//!    group when an admin leaves.
//! 2. Sender-chain equivocation never voids a Remove or the Genesis.
//!    Removals are monotone (D3 M3) and must not be undone by their author's
//!    later misbehaviour (a removed admin could otherwise resurrect everyone
//!    he removed by re-signing their sequence numbers), and voiding the
//!    Genesis would destroy the group irrecoverably. Add, AdminSet and
//!    Setting objects of an equivocating slot are both ignored, as the table
//!    says; the damage (an evicted member, a reverted admin set) is
//!    reversible by a re-add or a new AdminSet.
//! 3. AdminSet equivocation counts only between AdminSets that pass their own
//!    authority checks (version, cosigners, majority). A lone admin signing a
//!    bogus second AdminSet could otherwise retroactively void a majority's
//!    change and, because every later majority intersects the voided one's
//!    cosigners, block admin-set changes forever. The twin set is computed
//!    to a fixpoint (voiding a pair can change the version numbering for
//!    later AdminSets), which is still a pure function of `S`.
//! 4. An AdminSet may only name members of its causal past. Admins are
//!    members (DESIGN §3 "admins are members"), and a key without a KeyCard
//!    in the roster could not be addressed by anyone.
//! 5. A genesis admin that is not also listed under `members` has an empty
//!    display name and `in_person = false` (nothing in the op says who
//!    verified them; the UI shows "Unverified" until an Add says otherwise).

use std::collections::{BTreeMap, BTreeSet};

use crate::crypto::ed25519::IdentityKey;
use crate::crypto::hash::{Hash32, sha256_concat};
use crate::crypto::keycard::KeyCard;
use crate::objects::roster_op::{RosterBody, RosterOp, setting};
use crate::roster::dag::RosterDag;

/// Domain label of [`members_of_head_set_hash`].
pub const HEAD_SET_DOMAIN: &str = "meshapp.v1.head-set";

/// One current member (DESIGN §2.4 member entry: KeyCard binding the
/// identity to its session key, display name, in-person flag).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberInfo {
    pub keycard: KeyCard,
    pub name: String,
    /// At least one admin verified this person in person (BRIEF_v2 §5.4).
    pub in_person: bool,
}

impl MemberInfo {
    /// The member's identity key.
    pub fn key(&self) -> IdentityKey {
        self.keycard.ed
    }
}

/// Why an op in `S` is not applied (DESIGN §3 table; the storage schema's
/// `roster_ops.void_reason`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VoidReason {
    /// Sender-chain equivocation: another object by the same author with
    /// the same `author_seq` and a different hash is in `S` (DESIGN §3 "any").
    Equivocation,
    /// The author is not in `admins(P)`.
    AuthorNotAdmin,
    /// Remove/Add of a current admin (DESIGN §3: "m ∉ admins(P)").
    SubjectIsAdmin,
    /// The Add's endorsing admin is not in `admins(P)`.
    EndorserNotAdmin,
    /// The Add's endorser is its author (two *distinct* admins, BRIEF §5.4).
    EndorserIsAuthor,
    /// Neither the author nor the endorser verified the newcomer in person.
    NoInPersonVerification,
    /// Remove-wins: a valid Remove of the subject is concurrent with this
    /// Add (D3 M3 "a removal beats any add of the same member that is not
    /// causally after it").
    RemoveWins,
    /// "Changed after reconnect": a valid Remove of the author is
    /// concurrent with this Add (D3 M3).
    AuthorRemovedConcurrently,
    /// "Changed after reconnect": a valid Remove of the endorser is
    /// concurrent with this Add.
    EndorserRemovedConcurrently,
    /// "Changed after reconnect": a valid AdminSet demoting the author is
    /// concurrent with this Add.
    AuthorDemotedConcurrently,
    /// "Changed after reconnect": a valid AdminSet demoting the endorser is
    /// concurrent with this Add.
    EndorserDemotedConcurrently,
    /// `version ≠ 1 + |applied AdminSets in P|`.
    AdminSetVersion { expected: u32 },
    /// A cosigner is not in `admins(P)`.
    CosignerNotAdmin,
    /// The AdminSet's author did not cosign it.
    AuthorNotCosigner,
    /// Fewer than a strict majority of `admins(P)` cosigned.
    NoMajority,
    /// A named admin is not a member of the op's causal past.
    AdminNotMember,
    /// An admin cosigned two different AdminSets with this version (D3 M2).
    AdminSetEquivocation,
}

impl VoidReason {
    /// Is this one of the merge-time voids the UI shows as "Changed after
    /// reconnect" (D3 M3, UI labels)?
    pub fn is_changed_after_reconnect(&self) -> bool {
        matches!(
            self,
            VoidReason::RemoveWins
                | VoidReason::AuthorRemovedConcurrently
                | VoidReason::EndorserRemovedConcurrently
                | VoidReason::AuthorDemotedConcurrently
                | VoidReason::EndorserDemotedConcurrently
        )
    }
}

/// `members(H)`, `admins(H)` and everything else derived from `S =
/// closure(H)` (DESIGN §1 `roster/eval.rs`). Two views over the same op set
/// are equal whatever order the ops arrived in.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RosterView {
    /// Group name from the Genesis (empty until the Genesis is held).
    pub name: String,
    /// The head set this view was computed for (sorted, deduplicated).
    pub heads: Vec<Hash32>,
    /// Heads the DAG does not hold; non-empty means this view is provisional
    /// (DESIGN §5 "H unresolved → Waiting · needs this vote's member list").
    pub unresolved: Vec<Hash32>,
    /// `S`: every applied op reachable from `heads`.
    pub ops: BTreeSet<Hash32>,
    /// `members(H)`.
    pub members: BTreeMap<IdentityKey, MemberInfo>,
    /// `admins(H)`: the genesis admins or the latest applied AdminSet.
    pub admins: BTreeSet<IdentityKey>,
    /// Version of the latest applied AdminSet (0 = genesis admins).
    pub admin_version: u32,
    /// Keys with a valid Remove in `S` that are not members.
    pub removed: BTreeSet<IdentityKey>,
    /// Subjects of Adds voided at merge time (reasons with
    /// [`VoidReason::is_changed_after_reconnect`]) that are not members.
    pub changed_after_reconnect: BTreeSet<IdentityKey>,
    /// Every op in `S` that is not applied, with its reason, sorted by hash.
    pub voided: Vec<(Hash32, VoidReason)>,
    /// Current settings (`setting::FLAGS` is always present once the
    /// Genesis is held).
    pub settings: BTreeMap<u8, u8>,
    admins_at: BTreeMap<Hash32, u32>,
    admin_sets: Vec<BTreeSet<IdentityKey>>,
}

impl RosterView {
    /// Is `key` a current member?
    pub fn is_member(&self, key: &IdentityKey) -> bool {
        self.members.contains_key(key)
    }

    /// Is `key` a current admin?
    pub fn is_admin(&self, key: &IdentityKey) -> bool {
        self.admins.contains(key)
    }

    /// Is the op in `S` and applied?
    pub fn is_valid(&self, op: &Hash32) -> bool {
        self.ops.contains(op) && self.void_reason(op).is_none()
    }

    /// Why the op is not applied, if it is in `S` and void.
    pub fn void_reason(&self, op: &Hash32) -> Option<VoidReason> {
        self.voided
            .binary_search_by(|(h, _)| h.cmp(op))
            .ok()
            .and_then(|i| self.voided.get(i))
            .map(|(_, r)| *r)
    }

    /// `admins(P(op))`: the admin set the op was judged against (memoised
    /// per op hash, DESIGN §3). `None` if the op is not in `S`.
    pub fn admins_at(&self, op: &Hash32) -> Option<&BTreeSet<IdentityKey>> {
        let i = *self.admins_at.get(op)?;
        self.admin_sets.get(usize::try_from(i).ok()?)
    }

    /// The `setting::FLAGS` bitfield (0 when no Genesis is held).
    pub fn flags(&self) -> u8 {
        self.settings.get(&setting::FLAGS).copied().unwrap_or(0)
    }

    /// Cache key for this view: [`members_of_head_set_hash`] of its heads.
    pub fn head_set_hash(&self) -> Hash32 {
        members_of_head_set_hash(&self.heads)
    }

    /// Number of members.
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Is this view complete (every head held)?
    pub fn is_resolved(&self) -> bool {
        self.unresolved.is_empty()
    }
}

/// Cache key of `members(H)`: `SHA256(domain ‖ 0x00 ‖ sorted distinct
/// heads)` (DESIGN §3 "the UI cache is keyed by the sorted head-set hash").
pub fn members_of_head_set_hash(heads: &[Hash32]) -> Hash32 {
    let mut sorted = heads.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let mut parts: Vec<&[u8]> = Vec::with_capacity(sorted.len() + 2);
    parts.push(HEAD_SET_DOMAIN.as_bytes());
    parts.push(&[0u8]);
    for h in &sorted {
        parts.push(h.as_bytes());
    }
    sha256_concat(&parts)
}

// ---- bit sets over the topologically indexed op set -----------------------

#[derive(Clone, Debug)]
struct BitSet(Vec<u64>);

impl BitSet {
    fn new(n: usize) -> Self {
        BitSet(vec![0u64; n.div_ceil(64)])
    }

    fn set(&mut self, i: usize) {
        if let Some(w) = self.0.get_mut(i / 64) {
            *w |= 1u64 << (i % 64);
        }
    }

    fn get(&self, i: usize) -> bool {
        self.0
            .get(i / 64)
            .is_some_and(|w| w & (1u64 << (i % 64)) != 0)
    }

    fn union_with(&mut self, other: &BitSet) {
        for (a, b) in self.0.iter_mut().zip(other.0.iter()) {
            *a |= *b;
        }
    }
}

// ---- evaluation context ----------------------------------------------------

/// Where a member's current entry comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Source {
    Genesis,
    Add(usize),
}

/// What `S` (or a sub-set of it) says about a base-valid Add.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AddStatus {
    /// The subject is a member through this Add.
    Live,
    /// A valid Remove of the subject is causally after the Add: an ordinary
    /// removal, not a void.
    Superseded,
    Void(VoidReason),
}

struct Ctx<'a> {
    ops: Vec<&'a RosterOp>,
    hashes: Vec<Hash32>,
    /// `past[i]` = indices of `P(op_i) = closure(op_i.heads)` (excludes `i`).
    past: Vec<BitSet>,
    genesis_idx: usize,
    genesis_admins: BTreeSet<IdentityKey>,
    genesis_entries: BTreeMap<IdentityKey, MemberInfo>,
    /// Ops whose `(author, author_seq)` slot holds two different hashes.
    sender_equiv: Vec<bool>,
    /// Remove ops by subject, in topological order.
    removes_by_subject: BTreeMap<IdentityKey, Vec<usize>>,
    /// Add ops by subject, in topological order.
    adds_by_subject: BTreeMap<IdentityKey, Vec<usize>>,
    adds: Vec<usize>,
    setting_ops: Vec<usize>,
    /// Cosigner keys per AdminSet op index.
    cosigners: BTreeMap<usize, BTreeSet<IdentityKey>>,
}

/// One topological pass: verdicts and the memoised `admins(P)`.
struct Pass {
    /// `None` = applied (for Adds: passes every P-only check).
    verdict: Vec<Option<VoidReason>>,
    /// Index into `admin_sets` per op.
    admins_at: Vec<usize>,
    /// `[0]` = genesis admins, then one entry per applied AdminSet.
    admin_sets: Vec<BTreeSet<IdentityKey>>,
    /// `(version, op index, admin_sets index)` of applied AdminSets, in
    /// topological order.
    applied_admin_sets: Vec<(u32, usize, usize)>,
}

impl<'a> Ctx<'a> {
    /// Indexes `set` in a topological order (parents first, ties by hash).
    fn build(dag: &'a RosterDag, set: &BTreeSet<Hash32>, genesis_hash: Hash32) -> Option<Self> {
        // Kahn's algorithm with a sorted ready queue: deterministic for a
        // given set whatever order the ops were inserted in.
        let mut indegree: BTreeMap<Hash32, usize> = BTreeMap::new();
        for h in set {
            let op = dag.get(h)?;
            let n = op.header.heads.iter().filter(|p| set.contains(*p)).count();
            indegree.insert(*h, n);
        }
        let mut ready: BTreeSet<Hash32> = indegree
            .iter()
            .filter(|(_, n)| **n == 0)
            .map(|(h, _)| *h)
            .collect();
        let mut order: Vec<Hash32> = Vec::with_capacity(set.len());
        while let Some(h) = ready.pop_first() {
            order.push(h);
            for c in dag.children(&h) {
                if let Some(n) = indegree.get_mut(&c) {
                    *n = n.saturating_sub(1);
                    if *n == 0 {
                        ready.insert(c);
                    }
                }
            }
        }
        if order.len() != set.len() {
            // A cycle is impossible (hashes), but never trust the input.
            return None;
        }
        let index: BTreeMap<Hash32, usize> =
            order.iter().enumerate().map(|(i, h)| (*h, i)).collect();
        let n = order.len();
        let mut ops: Vec<&RosterOp> = Vec::with_capacity(n);
        let mut past: Vec<BitSet> = Vec::with_capacity(n);
        for h in &order {
            let op = dag.get(h)?;
            let mut p = BitSet::new(n);
            for head in &op.header.heads {
                if let Some(&j) = index.get(head) {
                    if let Some(pj) = past.get(j) {
                        p.union_with(pj);
                    }
                    p.set(j);
                }
            }
            ops.push(op);
            past.push(p);
        }
        let genesis_idx = *index.get(&genesis_hash)?;
        let (genesis_admins, genesis_entries) = match &ops.get(genesis_idx)?.body {
            RosterBody::Genesis {
                admins, members, ..
            } => {
                let mut entries: BTreeMap<IdentityKey, MemberInfo> = BTreeMap::new();
                for kc in admins {
                    entries.insert(
                        kc.ed,
                        MemberInfo {
                            keycard: *kc,
                            name: String::new(),
                            in_person: false,
                        },
                    );
                }
                for m in members {
                    entries.insert(
                        m.key(),
                        MemberInfo {
                            keycard: m.keycard,
                            name: m.name.clone(),
                            in_person: m.in_person,
                        },
                    );
                }
                (admins.iter().map(|kc| kc.ed).collect(), entries)
            }
            _ => return None,
        };

        // Sender-chain equivocation within S (DESIGN §3 "any" row).
        let mut slots: BTreeMap<(IdentityKey, u32), Vec<usize>> = BTreeMap::new();
        for (i, op) in ops.iter().enumerate() {
            slots
                .entry((op.header.author, op.header.author_seq))
                .or_default()
                .push(i);
        }
        let mut sender_equiv = vec![false; n];
        for idxs in slots.values() {
            if idxs.len() > 1 {
                for &i in idxs {
                    if let Some(s) = sender_equiv.get_mut(i) {
                        *s = true;
                    }
                }
            }
        }

        let mut removes_by_subject: BTreeMap<IdentityKey, Vec<usize>> = BTreeMap::new();
        let mut adds_by_subject: BTreeMap<IdentityKey, Vec<usize>> = BTreeMap::new();
        let mut adds = Vec::new();
        let mut setting_ops = Vec::new();
        let mut cosigners = BTreeMap::new();
        for (i, op) in ops.iter().enumerate() {
            match &op.body {
                RosterBody::Remove { subject } => {
                    removes_by_subject.entry(*subject).or_default().push(i);
                }
                RosterBody::Add { subject, .. } => {
                    adds_by_subject.entry(subject.key()).or_default().push(i);
                    adds.push(i);
                }
                RosterBody::AdminSet { cosigs, .. } => {
                    cosigners.insert(i, cosigs.iter().map(|(k, _)| *k).collect());
                }
                RosterBody::Setting { .. } => setting_ops.push(i),
                RosterBody::Genesis { .. } => {}
            }
        }
        Some(Ctx {
            ops,
            hashes: order,
            past,
            genesis_idx,
            genesis_admins,
            genesis_entries,
            sender_equiv,
            removes_by_subject,
            adds_by_subject,
            adds,
            setting_ops,
            cosigners,
        })
    }

    fn in_past(&self, of: usize, candidate: usize) -> bool {
        self.past.get(of).is_some_and(|p| p.get(candidate))
    }

    /// Neither op is in the other's causal past.
    fn concurrent(&self, a: usize, b: usize) -> bool {
        a != b && !self.in_past(a, b) && !self.in_past(b, a)
    }

    fn author(&self, i: usize) -> Option<IdentityKey> {
        self.ops.get(i).map(|op| op.header.author)
    }

    fn is_valid_remove_of(&self, pass: &Pass, i: usize, subject: &IdentityKey) -> bool {
        pass.verdict.get(i).is_some_and(Option::is_none)
            && matches!(self.ops.get(i).map(|o| &o.body), Some(RosterBody::Remove { subject: s }) if s == subject)
    }

    /// Does the applied AdminSet `y` demote `who` (in `admins(P(y))`, not in
    /// `y.admins`)?
    fn demotes(&self, pass: &Pass, y: usize, who: &IdentityKey) -> bool {
        let Some(RosterBody::AdminSet { admins, .. }) = self.ops.get(y).map(|o| &o.body) else {
            return false;
        };
        let before = pass.admins_at.get(y).and_then(|&k| pass.admin_sets.get(k));
        before.is_some_and(|b| b.contains(who)) && admins.binary_search(who).is_err()
    }

    /// Status of the base-valid Add `a` within the op set `contains`
    /// (`S` itself, or `P(Y)` for an AdminSet's member check).
    fn add_status(&self, pass: &Pass, a: usize, contains: &dyn Fn(usize) -> bool) -> AddStatus {
        let Some(RosterBody::Add {
            subject,
            endorsement,
        }) = self.ops.get(a).map(|o| &o.body)
        else {
            return AddStatus::Void(VoidReason::AuthorNotAdmin);
        };
        let Some(author) = self.author(a) else {
            return AddStatus::Void(VoidReason::AuthorNotAdmin);
        };
        let endorser = endorsement.admin;
        // "Changed after reconnect": concurrent removal or demotion of the
        // author or the endorser (D3 M3; module decision 1).
        for (who, removed, demoted) in [
            (
                author,
                VoidReason::AuthorRemovedConcurrently,
                VoidReason::AuthorDemotedConcurrently,
            ),
            (
                endorser,
                VoidReason::EndorserRemovedConcurrently,
                VoidReason::EndorserDemotedConcurrently,
            ),
        ] {
            if let Some(rs) = self.removes_by_subject.get(&who) {
                if rs.iter().any(|&r| {
                    contains(r) && self.is_valid_remove_of(pass, r, &who) && self.concurrent(a, r)
                }) {
                    return AddStatus::Void(removed);
                }
            }
            if pass.applied_admin_sets.iter().any(|&(_, y, _)| {
                contains(y) && self.concurrent(a, y) && self.demotes(pass, y, &who)
            }) {
                return AddStatus::Void(demoted);
            }
        }
        // Remove-wins: every valid Remove(m) in the set must be in P(a).
        let m = subject.key();
        let mut superseded = false;
        if let Some(rs) = self.removes_by_subject.get(&m) {
            for &r in rs {
                if !contains(r) || !self.is_valid_remove_of(pass, r, &m) || self.in_past(a, r) {
                    continue;
                }
                if self.in_past(r, a) {
                    superseded = true;
                } else {
                    return AddStatus::Void(VoidReason::RemoveWins);
                }
            }
        }
        if superseded {
            AddStatus::Superseded
        } else {
            AddStatus::Live
        }
    }

    /// `members(set)`: genesis entries without a valid Remove in the set,
    /// plus every subject with a live Add; the entry comes from the
    /// causally latest live Add (ties by op hash), else the Genesis.
    fn members_in(
        &self,
        pass: &Pass,
        contains: &dyn Fn(usize) -> bool,
    ) -> BTreeMap<IdentityKey, Source> {
        let mut out: BTreeMap<IdentityKey, Source> = BTreeMap::new();
        for key in self.genesis_entries.keys() {
            let removed = self.removes_by_subject.get(key).is_some_and(|rs| {
                rs.iter()
                    .any(|&r| contains(r) && self.is_valid_remove_of(pass, r, key))
            });
            if !removed {
                out.insert(*key, Source::Genesis);
            }
        }
        for (m, adds) in &self.adds_by_subject {
            let live: Vec<usize> = adds
                .iter()
                .copied()
                .filter(|&a| {
                    contains(a)
                        && pass.verdict.get(a).is_some_and(Option::is_none)
                        && self.add_status(pass, a, contains) == AddStatus::Live
                })
                .collect();
            // Causally maximal live Adds, then the highest hash.
            let best = live
                .iter()
                .copied()
                .filter(|&a| !live.iter().any(|&b| b != a && self.in_past(b, a)))
                .max_by_key(|&a| self.hashes.get(a).copied().unwrap_or(Hash32::ZERO));
            if let Some(a) = best {
                out.insert(*m, Source::Add(a));
            }
        }
        out
    }

    /// One topological pass with `forced_void` AdminSets (equivocation
    /// twins) treated as void.
    fn pass(&self, forced_void: &BTreeSet<usize>) -> Pass {
        let n = self.ops.len();
        let mut pass = Pass {
            verdict: vec![None; n],
            admins_at: vec![0; n],
            admin_sets: vec![self.genesis_admins.clone()],
            applied_admin_sets: Vec::new(),
        };
        for i in 0..n {
            let Some(op) = self.ops.get(i) else { continue };
            // admins(P(op)): the highest-version applied AdminSet in P.
            let governing = pass
                .applied_admin_sets
                .iter()
                .filter(|&&(_, y, _)| self.in_past(i, y))
                .max_by_key(|&&(v, y, _)| (v, self.hashes.get(y).copied().unwrap_or(Hash32::ZERO)))
                .map_or(0, |&(_, _, k)| k);
            if let Some(slot) = pass.admins_at.get_mut(i) {
                *slot = governing;
            }
            let admins = pass.admin_sets.get(governing).cloned().unwrap_or_default();
            let author = op.header.author;
            let equivocated = self.sender_equiv.get(i).copied().unwrap_or(false);
            let verdict: Option<VoidReason> = match &op.body {
                RosterBody::Genesis { .. } => {
                    // The DAG holds one genesis per group; anything else
                    // reachable can only be that one.
                    (i != self.genesis_idx).then_some(VoidReason::AuthorNotAdmin)
                }
                RosterBody::Remove { subject } => {
                    // Monotone and exempt from sender-chain voids (decision 2).
                    if !admins.contains(&author) {
                        Some(VoidReason::AuthorNotAdmin)
                    } else if admins.contains(subject) {
                        Some(VoidReason::SubjectIsAdmin)
                    } else {
                        None
                    }
                }
                RosterBody::Add {
                    subject,
                    endorsement,
                } => {
                    if equivocated {
                        Some(VoidReason::Equivocation)
                    } else if !admins.contains(&author) {
                        Some(VoidReason::AuthorNotAdmin)
                    } else if !admins.contains(&endorsement.admin) {
                        Some(VoidReason::EndorserNotAdmin)
                    } else if endorsement.admin == author {
                        Some(VoidReason::EndorserIsAuthor)
                    } else if !(subject.in_person || endorsement.in_person) {
                        Some(VoidReason::NoInPersonVerification)
                    } else if admins.contains(&subject.key()) {
                        Some(VoidReason::SubjectIsAdmin)
                    } else {
                        None
                    }
                }
                RosterBody::AdminSet {
                    version,
                    admins: new_admins,
                    ..
                } => {
                    let cosigners = self.cosigners.get(&i).cloned().unwrap_or_default();
                    let applied_in_past = pass
                        .applied_admin_sets
                        .iter()
                        .filter(|&&(_, y, _)| self.in_past(i, y))
                        .count();
                    let expected = u32::try_from(applied_in_past)
                        .ok()
                        .and_then(|k| k.checked_add(1))
                        .unwrap_or(u32::MAX);
                    if equivocated {
                        Some(VoidReason::Equivocation)
                    } else if forced_void.contains(&i) {
                        Some(VoidReason::AdminSetEquivocation)
                    } else if *version != expected {
                        Some(VoidReason::AdminSetVersion { expected })
                    } else if !cosigners.is_subset(&admins) {
                        Some(VoidReason::CosignerNotAdmin)
                    } else if !cosigners.contains(&author) {
                        Some(VoidReason::AuthorNotCosigner)
                    } else if cosigners.len().saturating_mul(2) <= admins.len() {
                        Some(VoidReason::NoMajority)
                    } else {
                        let members = self.members_in(&pass, &|j| self.in_past(i, j));
                        if new_admins.iter().all(|k| members.contains_key(k)) {
                            None
                        } else {
                            Some(VoidReason::AdminNotMember)
                        }
                    }
                }
                RosterBody::Setting { .. } => {
                    if equivocated {
                        Some(VoidReason::Equivocation)
                    } else if !admins.contains(&author) {
                        Some(VoidReason::AuthorNotAdmin)
                    } else {
                        None
                    }
                }
            };
            if verdict.is_none() {
                if let RosterBody::AdminSet {
                    version,
                    admins: new_admins,
                    ..
                } = &op.body
                {
                    let k = pass.admin_sets.len();
                    pass.admin_sets.push(new_admins.iter().copied().collect());
                    pass.applied_admin_sets.push((*version, i, k));
                }
            }
            if let Some(slot) = pass.verdict.get_mut(i) {
                *slot = verdict;
            }
        }
        pass
    }

    /// Pairs of applied AdminSets with the same version that share a
    /// cosigner (D3 M2 equivocation).
    fn admin_twins(&self, pass: &Pass) -> BTreeSet<usize> {
        let mut by_version: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
        for &(v, y, _) in &pass.applied_admin_sets {
            by_version.entry(v).or_default().push(y);
        }
        let mut twins = BTreeSet::new();
        for ys in by_version.values() {
            for (ai, &a) in ys.iter().enumerate() {
                for &b in ys.iter().skip(ai.saturating_add(1)) {
                    let (Some(ca), Some(cb)) = (self.cosigners.get(&a), self.cosigners.get(&b))
                    else {
                        continue;
                    };
                    if !ca.is_disjoint(cb) {
                        twins.insert(a);
                        twins.insert(b);
                    }
                }
            }
        }
        twins
    }

    /// Runs passes until the AdminSet twin set stops growing (decision 3).
    fn evaluate(&self) -> Pass {
        let mut forced: BTreeSet<usize> = BTreeSet::new();
        loop {
            let pass = self.pass(&forced);
            let twins = self.admin_twins(&pass);
            let before = forced.len();
            forced.extend(twins);
            if forced.len() == before {
                return pass;
            }
        }
    }
}

/// Evaluates `S = closure(heads)` into a [`RosterView`] (see the module
/// docs). Heads the DAG does not hold are skipped and reported in
/// [`RosterView::unresolved`]; a DAG without its Genesis yields an empty
/// view.
pub fn evaluate(dag: &RosterDag, heads: &[Hash32]) -> RosterView {
    let mut sorted = heads.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let (set, unresolved) = dag.closure_with_unresolved(&sorted);
    let mut view = RosterView {
        heads: sorted,
        unresolved,
        ops: set.clone(),
        ..RosterView::default()
    };
    let Some(genesis_hash) = dag.genesis_hash() else {
        return view;
    };
    if !set.contains(&genesis_hash) {
        return view;
    }
    let Some(ctx) = Ctx::build(dag, &set, genesis_hash) else {
        return view;
    };
    let pass = ctx.evaluate();
    let all = |_: usize| true;

    if let Some(RosterBody::Genesis { name, settings, .. }) =
        ctx.ops.get(ctx.genesis_idx).map(|o| &o.body)
    {
        view.name = name.clone();
        view.settings.insert(setting::FLAGS, *settings);
    }

    // members(S)
    for (key, source) in ctx.members_in(&pass, &all) {
        let info = match source {
            Source::Genesis => ctx.genesis_entries.get(&key).cloned(),
            Source::Add(a) => match ctx.ops.get(a).map(|o| &o.body) {
                Some(RosterBody::Add {
                    subject,
                    endorsement,
                }) => Some(MemberInfo {
                    keycard: subject.keycard,
                    name: subject.name.clone(),
                    in_person: subject.in_person || endorsement.in_person,
                }),
                _ => None,
            },
        };
        if let Some(info) = info {
            view.members.insert(key, info);
        }
    }

    // admins(S) = the latest applied AdminSet, else the genesis admins.
    let latest = pass
        .applied_admin_sets
        .iter()
        .max_by_key(|&&(v, y, _)| (v, ctx.hashes.get(y).copied().unwrap_or(Hash32::ZERO)));
    match latest {
        Some(&(v, _, k)) => {
            view.admin_version = v;
            view.admins = pass.admin_sets.get(k).cloned().unwrap_or_default();
        }
        None => view.admins = ctx.genesis_admins.clone(),
    }

    // Voided ops: P-only verdicts, then the merge-time Add voids.
    let mut voided: Vec<(Hash32, VoidReason)> = Vec::new();
    for (i, v) in pass.verdict.iter().enumerate() {
        if let (Some(r), Some(h)) = (v, ctx.hashes.get(i)) {
            voided.push((*h, *r));
        }
    }
    for &a in &ctx.adds {
        if pass.verdict.get(a).is_some_and(Option::is_some) {
            continue;
        }
        if let AddStatus::Void(r) = ctx.add_status(&pass, a, &all) {
            if let Some(h) = ctx.hashes.get(a) {
                voided.push((*h, r));
            }
            if let Some(RosterBody::Add { subject, .. }) = ctx.ops.get(a).map(|o| &o.body) {
                let m = subject.key();
                if r.is_changed_after_reconnect() && !view.members.contains_key(&m) {
                    view.changed_after_reconnect.insert(m);
                }
            }
        }
    }
    voided.sort_unstable();
    voided.dedup();
    view.voided = voided;

    // Keys with a valid Remove that are not members.
    for (subject, rs) in &ctx.removes_by_subject {
        if view.members.contains_key(subject) {
            continue;
        }
        if rs
            .iter()
            .any(|&r| ctx.is_valid_remove_of(&pass, r, subject))
        {
            view.removed.insert(*subject);
        }
    }

    // Settings: last writer per key, concurrent writers by op hash.
    let mut by_key: BTreeMap<u8, Vec<usize>> = BTreeMap::new();
    for &s in &ctx.setting_ops {
        if pass.verdict.get(s).is_some_and(Option::is_none) {
            if let Some(RosterBody::Setting { key, .. }) = ctx.ops.get(s).map(|o| &o.body) {
                by_key.entry(*key).or_default().push(s);
            }
        }
    }
    for (key, writers) in by_key {
        let winner = writers
            .iter()
            .copied()
            .filter(|&s| !writers.iter().any(|&t| t != s && ctx.in_past(t, s)))
            .max_by_key(|&s| ctx.hashes.get(s).copied().unwrap_or(Hash32::ZERO));
        if let Some(RosterBody::Setting { value, .. }) =
            winner.and_then(|s| ctx.ops.get(s)).map(|o| &o.body)
        {
            view.settings.insert(key, *value);
        }
    }

    // Memoised admins(P) per op hash.
    view.admins_at = ctx
        .hashes
        .iter()
        .enumerate()
        .filter_map(|(i, h)| {
            let k = *pass.admins_at.get(i)?;
            Some((*h, u32::try_from(k).ok()?))
        })
        .collect();
    view.admin_sets = pass.admin_sets;
    view
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::backend::GroupId;
    use crate::crypto::ed25519::IdentitySigner;
    use crate::objects::roster_op::MemberEntry;
    use crate::objects::{SignedHeader, kind};
    use crate::roster::dag::Insert;

    fn signer(i: u8) -> IdentitySigner {
        IdentitySigner::from_seed(&[i; 32])
    }

    fn key(i: u8) -> IdentityKey {
        signer(i).public()
    }

    fn card(i: u8) -> KeyCard {
        KeyCard::issue(&signer(i), [0x80 ^ i; 32])
    }

    fn entry(i: u8, in_person: bool) -> MemberEntry {
        MemberEntry {
            keycard: card(i),
            name: format!("n{i}"),
            in_person,
        }
    }

    /// Builds ops with honest per-author sequence numbers.
    struct Fx {
        gid: GroupId,
        seq: BTreeMap<u8, (u32, Hash32)>,
        dag: RosterDag,
    }

    impl Fx {
        fn new(admins: &[u8], members: &[u8]) -> (Self, Hash32) {
            let s = signer(admins[0]);
            let g = RosterOp::sign(
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
                    nonce: [3; 16],
                    name: "Shelter".into(),
                    admins: admins.iter().map(|&a| card(a)).collect(),
                    members: members.iter().map(|&m| entry(m, true)).collect(),
                    settings: 0,
                },
            )
            .unwrap();
            let gh = g.op_hash();
            let dag = RosterDag::from_genesis(g).unwrap();
            let mut seq = BTreeMap::new();
            seq.insert(admins[0], (1, gh));
            (
                Fx {
                    gid: dag.group(),
                    seq,
                    dag,
                },
                gh,
            )
        }

        fn header(&mut self, author: u8, k: u8, heads: Vec<Hash32>) -> SignedHeader {
            let (seq, prev) = self.seq.get(&author).copied().unwrap_or((0, Hash32::ZERO));
            SignedHeader::new(k, self.gid, key(author), seq, prev, heads).unwrap()
        }

        fn finish(&mut self, author: u8, op: RosterOp) -> Hash32 {
            let h = op.op_hash();
            let e = self.seq.entry(author).or_insert((0, Hash32::ZERO));
            e.0 += 1;
            e.1 = h;
            assert!(matches!(self.dag.insert(op), Insert::Applied(_)));
            h
        }

        fn add(&mut self, author: u8, endorser: u8, subject: u8, heads: Vec<Hash32>) -> Hash32 {
            let hdr = self.header(author, kind::ROSTER_ADD, heads);
            let e = RosterOp::endorse(
                &signer(endorser),
                &self.gid,
                &key(subject),
                &format!("n{subject}"),
                true,
            )
            .unwrap();
            let op = RosterOp::sign(
                &signer(author),
                hdr,
                RosterBody::Add {
                    subject: entry(subject, false),
                    endorsement: e,
                },
            )
            .unwrap();
            self.finish(author, op)
        }

        fn add_with(
            &mut self,
            author: u8,
            endorser: u8,
            subject: u8,
            heads: Vec<Hash32>,
            author_in_person: bool,
            endorser_in_person: bool,
        ) -> Hash32 {
            let hdr = self.header(author, kind::ROSTER_ADD, heads);
            let e = RosterOp::endorse(
                &signer(endorser),
                &self.gid,
                &key(subject),
                &format!("n{subject}"),
                endorser_in_person,
            )
            .unwrap();
            let op = RosterOp::sign(
                &signer(author),
                hdr,
                RosterBody::Add {
                    subject: entry(subject, author_in_person),
                    endorsement: e,
                },
            )
            .unwrap();
            self.finish(author, op)
        }

        fn remove(&mut self, author: u8, subject: u8, heads: Vec<Hash32>) -> Hash32 {
            let hdr = self.header(author, kind::ROSTER_REMOVE, heads);
            let op = RosterOp::sign(
                &signer(author),
                hdr,
                RosterBody::Remove {
                    subject: key(subject),
                },
            )
            .unwrap();
            self.finish(author, op)
        }

        fn admin_set(
            &mut self,
            author: u8,
            cosigners: &[u8],
            admins: &[u8],
            version: u32,
            heads: Vec<Hash32>,
        ) -> Hash32 {
            let hdr = self.header(author, kind::ROSTER_ADMIN_SET, heads);
            let mut new: Vec<IdentityKey> = admins.iter().map(|&a| key(a)).collect();
            new.sort_unstable();
            new.dedup();
            let mut cosigs: Vec<(IdentityKey, [u8; 64])> = cosigners
                .iter()
                .map(|&c| RosterOp::cosign(&signer(c), &self.gid, version, &new))
                .collect();
            cosigs.sort_by_key(|c| c.0);
            cosigs.dedup_by(|a, b| a.0 == b.0);
            let op = RosterOp::sign(
                &signer(author),
                hdr,
                RosterBody::AdminSet {
                    version,
                    admins: new,
                    cosigs,
                },
            )
            .unwrap();
            self.finish(author, op)
        }

        fn setting(&mut self, author: u8, k: u8, value: u8, heads: Vec<Hash32>) -> Hash32 {
            let hdr = self.header(author, kind::ROSTER_SETTING, heads);
            let op = RosterOp::sign(&signer(author), hdr, RosterBody::Setting { key: k, value })
                .unwrap();
            self.finish(author, op)
        }

        fn view(&self) -> RosterView {
            evaluate(&self.dag, &self.dag.heads())
        }
    }

    fn keys(v: &[u8]) -> BTreeSet<IdentityKey> {
        v.iter().map(|&i| key(i)).collect()
    }

    #[test]
    fn genesis_alone_lists_admins_and_members() {
        let (fx, gh) = Fx::new(&[1, 2], &[2, 3]);
        let v = fx.view();
        assert_eq!(v.heads, vec![gh]);
        assert!(v.is_resolved());
        assert_eq!(v.admins, keys(&[1, 2]));
        assert_eq!(
            v.members.keys().copied().collect::<BTreeSet<_>>(),
            keys(&[1, 2, 3])
        );
        assert_eq!(v.members[&key(2)].name, "n2");
        assert_eq!(v.members[&key(1)].name, "");
        assert!(v.voided.is_empty());
        assert_eq!(v.name, "Shelter");
        assert_eq!(v.flags(), 0);
        assert_eq!(v.admin_version, 0);
        assert_eq!(v.admins_at(&gh), Some(&keys(&[1, 2])));
        assert_eq!(v.head_set_hash(), members_of_head_set_hash(&[gh]));
    }

    #[test]
    fn empty_or_unknown_heads_give_empty_view() {
        let (fx, gh) = Fx::new(&[1], &[]);
        let v = evaluate(&fx.dag, &[]);
        assert!(v.members.is_empty() && v.admins.is_empty());
        let ghost = Hash32::from_bytes([9; 32]);
        let v = evaluate(&fx.dag, &[ghost]);
        assert_eq!(v.unresolved, vec![ghost]);
        assert!(v.members.is_empty());
        let v = evaluate(&fx.dag, &[gh, ghost]);
        assert!(!v.is_resolved());
        assert_eq!(v.members.len(), 1);
        let empty = RosterDag::new(fx.dag.group());
        assert!(evaluate(&empty, &[gh]).members.is_empty());
    }

    #[test]
    fn add_then_remove_then_readd() {
        let (mut fx, gh) = Fx::new(&[1, 2], &[]);
        let a = fx.add(1, 2, 5, vec![gh]);
        assert!(fx.view().is_member(&key(5)));
        let r = fx.remove(2, 5, vec![a]);
        let v = fx.view();
        assert!(!v.is_member(&key(5)));
        assert!(v.removed.contains(&key(5)));
        // superseded, not voided
        assert!(v.voided.is_empty());
        let a2 = fx.add(2, 1, 5, vec![r]);
        let v = fx.view();
        assert!(v.is_member(&key(5)));
        assert!(!v.removed.contains(&key(5)));
        assert!(v.is_valid(&a2));
    }

    #[test]
    fn remove_wins_over_concurrent_add() {
        let (mut fx, gh) = Fx::new(&[1, 2, 3], &[5]);
        let r = fx.remove(1, 5, vec![gh]);
        let a = fx.add(2, 3, 5, vec![gh]);
        let v = fx.view();
        assert!(!v.is_member(&key(5)));
        assert_eq!(v.void_reason(&a), Some(VoidReason::RemoveWins));
        assert!(v.is_valid(&r));
        assert!(v.changed_after_reconnect.contains(&key(5)));
    }

    #[test]
    fn concurrent_add_by_removed_author_is_void_but_later_removal_is_not() {
        let (mut fx, gh) = Fx::new(&[1, 2, 3], &[]);
        // 3 is demoted and removed in one fragment ...
        let d = fx.admin_set(1, &[1, 2], &[1, 2], 1, vec![gh]);
        let r = fx.remove(1, 3, vec![d]);
        // ... while 3 adds 7 (endorsed by 2) concurrently
        let a = fx.add(3, 2, 7, vec![gh]);
        let v = fx.view();
        assert!(matches!(
            v.void_reason(&a),
            Some(VoidReason::AuthorDemotedConcurrently | VoidReason::AuthorRemovedConcurrently)
        ));
        assert!(!v.is_member(&key(7)));
        assert!(!v.is_member(&key(3)));
        assert_eq!(v.admins, keys(&[1, 2]));
        assert!(v.changed_after_reconnect.contains(&key(7)));
        // An Add made causally before the demotion stands after it.
        let (mut fx2, gh2) = Fx::new(&[1, 2, 3], &[]);
        let a2 = fx2.add(3, 2, 7, vec![gh2]);
        let d2 = fx2.admin_set(1, &[1, 2], &[1, 2], 1, vec![a2]);
        let r2 = fx2.remove(1, 3, vec![d2]);
        let v2 = fx2.view();
        assert!(v2.is_valid(&a2));
        assert!(v2.is_member(&key(7)));
        assert!(!v2.is_member(&key(3)));
        assert!(v2.is_valid(&r2));
        assert!(v.is_valid(&r));
    }

    #[test]
    fn endorser_removed_concurrently_voids_add() {
        let (mut fx, gh) = Fx::new(&[1, 2, 3], &[]);
        let d = fx.admin_set(1, &[1, 2], &[1, 2], 1, vec![gh]);
        let _ = fx.remove(2, 3, vec![d]);
        let a = fx.add(2, 3, 7, vec![gh]);
        let v = fx.view();
        assert!(matches!(
            v.void_reason(&a),
            Some(VoidReason::EndorserDemotedConcurrently | VoidReason::EndorserRemovedConcurrently)
        ));
        // demotion alone (no removal) is reported as such
        let (mut fx3, gh3) = Fx::new(&[1, 2, 3], &[]);
        let _ = fx3.admin_set(1, &[1, 2], &[1, 2], 1, vec![gh3]);
        let a3 = fx3.add(2, 3, 7, vec![gh3]);
        assert_eq!(
            fx3.view().void_reason(&a3),
            Some(VoidReason::EndorserDemotedConcurrently)
        );
        // A plain Remove of a current admin is invalid (demote first).
        let (mut fx, gh) = Fx::new(&[1, 2, 3], &[]);
        let bad = fx.remove(1, 3, vec![gh]);
        assert_eq!(
            fx.view().void_reason(&bad),
            Some(VoidReason::SubjectIsAdmin)
        );
    }

    #[test]
    fn add_rules_negative_cases() {
        let (mut fx, gh) = Fx::new(&[1, 2], &[5]);
        let by_member = fx.add(5, 1, 7, vec![gh]);
        let endorser_not_admin = fx.add(1, 5, 7, vec![gh]);
        let self_endorsed = fx.add(1, 1, 7, vec![gh]);
        let nobody_in_person = fx.add_with(1, 2, 7, vec![gh], false, false);
        let add_admin = fx.add(1, 2, 2, vec![gh]);
        let v = fx.view();
        assert_eq!(v.void_reason(&by_member), Some(VoidReason::AuthorNotAdmin));
        assert_eq!(
            v.void_reason(&endorser_not_admin),
            Some(VoidReason::EndorserNotAdmin)
        );
        assert_eq!(
            v.void_reason(&self_endorsed),
            Some(VoidReason::EndorserIsAuthor)
        );
        assert_eq!(
            v.void_reason(&nobody_in_person),
            Some(VoidReason::NoInPersonVerification)
        );
        assert_eq!(v.void_reason(&add_admin), Some(VoidReason::SubjectIsAdmin));
        assert!(!v.is_member(&key(7)));
        let ok = fx.add_with(2, 1, 7, vec![gh], true, false);
        let v = fx.view();
        assert!(v.is_valid(&ok));
        assert!(v.members[&key(7)].in_person);
    }

    #[test]
    fn admin_set_rules() {
        let (mut fx, gh) = Fx::new(&[1, 2, 3], &[4]);
        let no_majority = fx.admin_set(1, &[1], &[1, 2, 3, 4], 1, vec![gh]);
        let wrong_version = fx.admin_set(1, &[1, 2], &[1, 2, 3, 4], 2, vec![gh]);
        let outsider_cosig = fx.admin_set(1, &[1, 4], &[1, 2, 3, 4], 1, vec![gh]);
        let author_not_cosigner = fx.admin_set(1, &[2, 3], &[1, 2, 3, 4], 1, vec![gh]);
        let not_member = fx.admin_set(1, &[1, 2], &[1, 2, 3, 9], 1, vec![gh]);
        let v = fx.view();
        assert_eq!(v.void_reason(&no_majority), Some(VoidReason::NoMajority));
        assert_eq!(
            v.void_reason(&wrong_version),
            Some(VoidReason::AdminSetVersion { expected: 1 })
        );
        assert_eq!(
            v.void_reason(&outsider_cosig),
            Some(VoidReason::CosignerNotAdmin)
        );
        assert_eq!(
            v.void_reason(&author_not_cosigner),
            Some(VoidReason::AuthorNotCosigner)
        );
        assert_eq!(v.void_reason(&not_member), Some(VoidReason::AdminNotMember));
        assert_eq!(v.admins, keys(&[1, 2, 3]));
        let good = fx.admin_set(1, &[1, 2], &[1, 2, 3, 4], 1, vec![gh]);
        let v = fx.view();
        assert!(v.is_valid(&good));
        assert_eq!(v.admins, keys(&[1, 2, 3, 4]));
        assert_eq!(v.admin_version, 1);
        // the next version needs 3 of 4
        let two_of_four = fx.admin_set(4, &[4, 1], &[4], 2, vec![good]);
        let three_of_four = fx.admin_set(4, &[4, 1, 2], &[4, 1], 2, vec![good]);
        let v = fx.view();
        assert_eq!(v.void_reason(&two_of_four), Some(VoidReason::NoMajority));
        assert!(v.is_valid(&three_of_four));
        assert_eq!(v.admins, keys(&[1, 4]));
        assert_eq!(v.admin_version, 2);
        assert_eq!(v.admins_at(&three_of_four), Some(&keys(&[1, 2, 3, 4])));
    }

    #[test]
    fn admin_set_equivocation_voids_both_and_is_repairable() {
        let (mut fx, gh) = Fx::new(&[1, 2, 3], &[4, 5]);
        let x = fx.admin_set(1, &[1, 2], &[1, 2, 4], 1, vec![gh]);
        let y = fx.admin_set(2, &[2, 3], &[2, 3, 5], 1, vec![gh]);
        let v = fx.view();
        assert_eq!(v.void_reason(&x), Some(VoidReason::AdminSetEquivocation));
        assert_eq!(v.void_reason(&y), Some(VoidReason::AdminSetEquivocation));
        assert_eq!(v.admins, keys(&[1, 2, 3]));
        // a lone bogus twin does not void a majority's change
        let (mut fx2, gh2) = Fx::new(&[1, 2, 3], &[4, 5]);
        let x2 = fx2.admin_set(1, &[1, 2], &[1, 2, 4], 1, vec![gh2]);
        let bogus = fx2.admin_set(1, &[1], &[1], 1, vec![gh2]);
        let v2 = fx2.view();
        assert!(v2.is_valid(&x2));
        assert_eq!(v2.void_reason(&bogus), Some(VoidReason::NoMajority));
        // repair after a real equivocation: a fresh v=1 citing both
        let z = fx.admin_set(3, &[3, 1], &[1, 3], 1, vec![x, y]);
        let v = fx.view();
        assert!(v.is_valid(&z));
        assert_eq!(v.admins, keys(&[1, 3]));
    }

    #[test]
    fn sender_chain_equivocation_voids_adds_but_not_removes() {
        let (mut fx, gh) = Fx::new(&[1, 2], &[5, 6]);
        let a = fx.add(1, 2, 7, vec![gh]);
        // re-sign seq 1 with different content
        let (seq, prev) = fx.seq[&1];
        let hdr = SignedHeader::new(
            kind::ROSTER_ADD,
            fx.gid,
            key(1),
            seq - 1,
            Hash32::ZERO,
            vec![gh],
        )
        .unwrap();
        let _ = prev;
        let e = RosterOp::endorse(&signer(2), &fx.gid, &key(8), "n8", true).unwrap();
        let twin = RosterOp::sign(
            &signer(1),
            hdr,
            RosterBody::Add {
                subject: entry(8, false),
                endorsement: e,
            },
        )
        .unwrap();
        let th = twin.op_hash();
        assert!(matches!(fx.dag.insert(twin), Insert::Applied(_)));
        let v = fx.view();
        assert_eq!(v.void_reason(&a), Some(VoidReason::Equivocation));
        assert_eq!(v.void_reason(&th), Some(VoidReason::Equivocation));
        assert!(!v.is_member(&key(7)) && !v.is_member(&key(8)));
        // Removes stand (decision 2)
        let r = fx.remove(2, 5, vec![gh]);
        let hdr = SignedHeader::new(
            kind::ROSTER_REMOVE,
            fx.gid,
            key(2),
            0,
            Hash32::ZERO,
            vec![gh],
        )
        .unwrap();
        let twin = RosterOp::sign(&signer(2), hdr, RosterBody::Remove { subject: key(6) }).unwrap();
        let th2 = twin.op_hash();
        assert!(matches!(fx.dag.insert(twin), Insert::Applied(_)));
        let v = fx.view();
        assert!(v.is_valid(&r) && v.is_valid(&th2));
        assert!(!v.is_member(&key(5)) && !v.is_member(&key(6)));
    }

    #[test]
    fn settings_last_writer_and_hash_tiebreak() {
        let (mut fx, gh) = Fx::new(&[1, 2], &[5]);
        let s1 = fx.setting(1, setting::FLAGS, 1, vec![gh]);
        let by_member = fx.setting(5, setting::FLAGS, 9, vec![gh]);
        let s2 = fx.setting(2, setting::FLAGS, 2, vec![gh]);
        let v = fx.view();
        assert_eq!(v.void_reason(&by_member), Some(VoidReason::AuthorNotAdmin));
        let expect = if s1 > s2 { 1 } else { 2 };
        assert_eq!(v.flags(), expect);
        let s3 = fx.setting(1, setting::FLAGS, 3, vec![s1, s2]);
        let v = fx.view();
        assert!(v.is_valid(&s3));
        assert_eq!(v.flags(), 3);
        let _ = fx.setting(2, 7, 1, vec![s3]);
        assert_eq!(fx.view().settings.get(&7), Some(&1));
    }

    #[test]
    fn insertion_order_does_not_matter() {
        let (mut fx, gh) = Fx::new(&[1, 2, 3], &[5]);
        let r = fx.remove(1, 5, vec![gh]);
        let a = fx.add(2, 3, 5, vec![gh]);
        let d = fx.admin_set(1, &[1, 2], &[1, 2], 1, vec![r, a]);
        let a2 = fx.add(3, 2, 6, vec![a]);
        let _ = fx.remove(1, 6, vec![d]);
        let ops: Vec<RosterOp> = fx.dag.ops().map(|(_, o)| o.clone()).collect();
        let reference = fx.view();
        let n = ops.len();
        for rot in 0..n {
            let mut dag = RosterDag::new(fx.gid);
            for i in 0..n {
                let _ = dag.insert(ops[(i * 7 + rot) % n].clone());
            }
            assert_eq!(dag.len(), n);
            assert_eq!(evaluate(&dag, &dag.heads()), reference);
        }
        assert_eq!(
            reference.void_reason(&a2),
            Some(VoidReason::AuthorDemotedConcurrently)
        );
    }

    #[test]
    fn head_set_hash_is_order_independent() {
        let a = Hash32::from_bytes([1; 32]);
        let b = Hash32::from_bytes([2; 32]);
        assert_eq!(
            members_of_head_set_hash(&[a, b]),
            members_of_head_set_hash(&[b, a, a])
        );
        assert_ne!(
            members_of_head_set_hash(&[a]),
            members_of_head_set_hash(&[b])
        );
    }

    #[test]
    fn bitset_basics() {
        let mut b = BitSet::new(130);
        assert!(!b.get(129));
        b.set(129);
        b.set(0);
        assert!(b.get(129) && b.get(0) && !b.get(64));
        b.set(1000);
        assert!(!b.get(1000));
        let mut c = BitSet::new(130);
        c.set(64);
        c.union_with(&b);
        assert!(c.get(64) && c.get(0) && c.get(129));
    }
}
