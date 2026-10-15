//! Task state: folding a task's op DAG (docs/DESIGN.md §4 "Fold"; BRIEF_v2
//! §4 Tasks; docs/decisions-assumed.md rows 5–6).
//!
//! Inputs are the task's ops with their hashes and chain-bound authors;
//! nothing here reads a clock (BRIEF_v2 §4 "no timestamps decide
//! anything"). The fold is deterministic: the same op set in any order gives
//! the same [`TaskState`], so every phone that holds the same ops agrees.
//!
//! Rules (DESIGN §4 "Fold"):
//! - ops are walked in topological order of `task_heads`, ties by op hash;
//! - a Take is a live claim unless the same author issued a Release that is
//!   not in the Take's causal past ("later-or-concurrent Release");
//! - a live Take whose causal past contains another live Take is a *second
//!   claim* ("2 people on this"), not a conflict;
//! - live Takes that are concurrent (neither in the other's past) form the
//!   conflict set, resolved by [`crate::tasks::tiebreak`];
//! - Done by any current claimant closes the task (idempotent; concurrent
//!   Dones are fine); a Done from someone without a live claim at that point
//!   is ignored;
//! - Create may be issued by any member (row 5), so a Create carries no
//!   authority and the fold does not require one;
//! - an op citing a head that is not in the set (or that is itself parked)
//!   is parked and reported, so the caller can ask for the missing ops;
//! - an op's identity is `(op_hash, author)`: the hash alone is not unique
//!   per member because the post plaintext it digests has no author field
//!   (see [`fold`]).

use std::collections::{BTreeMap, BTreeSet};

use crate::crypto::ed25519::IdentityKey;
use crate::crypto::hash::Hash32;
use crate::objects::task_op::{TaskId, TaskKind, TaskOp};
use crate::tasks::tiebreak;

/// The folded state of one task (BRIEF_v2 §4 "States in plain UI").
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskState {
    Open,
    /// One or more live claims, none concurrent with another. More than one
    /// holder is shown as "N people on this".
    Taken {
        /// In claim order (topological order of the claiming Takes).
        holders: Vec<IdentityKey>,
    },
    /// Concurrent live claims. `winner` is the tie-break pick, `losers` the
    /// rest of the conflict set (offered "Keep helping" or "Release"), and
    /// `second` the live claims that saw the conflict and took anyway
    /// (second claims, still on the task).
    Conflict {
        winner: IdentityKey,
        /// Sorted ascending.
        losers: Vec<IdentityKey>,
        /// Sorted ascending.
        second: Vec<IdentityKey>,
    },
    Done,
}

impl TaskState {
    /// Everyone currently on the task, in display order: holders, or the
    /// winner, losers and second claimants; empty for Open and Done.
    pub fn holders(&self) -> Vec<IdentityKey> {
        match self {
            TaskState::Open | TaskState::Done => Vec::new(),
            TaskState::Taken { holders } => holders.clone(),
            TaskState::Conflict {
                winner,
                losers,
                second,
            } => {
                let mut v = Vec::with_capacity(1 + losers.len() + second.len());
                v.push(*winner);
                v.extend_from_slice(losers);
                v.extend_from_slice(second);
                v
            }
        }
    }
}

/// Result of [`fold`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fold {
    pub state: TaskState,
    /// Hashes of ops that cite a head not in the input (directly or through
    /// another parked op), or that are part of a reference cycle. They did
    /// not take part in the fold; re-run when the missing ops arrive.
    pub parked: Vec<Hash32>,
}

/// One op as the fold sees it: `(op_hash, author, op)`. The author is the
/// post's chain owner (docs/decisions-assumed.md row 12).
pub type OpRecord = (Hash32, IdentityKey, TaskOp);

/// Folds `ops` (any order) into the task's state. `task_id` seeds the
/// tie-break.
///
/// An op's identity is `(op_hash, author)`, not the hash alone: the op hash
/// is `SHA256(post plaintext)` (DESIGN §2.4) and the post plaintext carries
/// no author, so two members who Take the same task citing the same heads
/// (the common concurrent-Take case) emit byte-identical posts with one
/// hash. Those are two claims. A head cited by hash therefore resolves to
/// every record carrying that hash (the citer saw a post with those bytes,
/// whichever member sent it), and a repeated `(hash, author)` record
/// collapses to the first seen.
pub fn fold(task_id: &TaskId, ops: &[OpRecord]) -> Fold {
    // ---- nodes: one per distinct (hash, author), in (hash, author) order --
    let mut seen_pair: BTreeSet<(Hash32, IdentityKey)> = BTreeSet::new();
    let mut nodes: Vec<usize> = Vec::new();
    for (i, (h, a, _)) in ops.iter().enumerate() {
        if seen_pair.insert((*h, *a)) {
            nodes.push(i);
        }
    }
    nodes.sort_by_key(|&i| (ops[i].0, ops[i].1));
    let record = |nid: usize| -> Option<&OpRecord> { nodes.get(nid).and_then(|&i| ops.get(i)) };
    // hash → node ids carrying it (ascending)
    let mut by_hash: BTreeMap<Hash32, Vec<usize>> = BTreeMap::new();
    for nid in 0..nodes.len() {
        if let Some((h, _, _)) = record(nid) {
            by_hash.entry(*h).or_default().push(nid);
        }
    }

    // ---- park ops with unresolved heads (fixpoint) ----------------------
    let mut parked: BTreeSet<usize> = BTreeSet::new();
    loop {
        let mut changed = false;
        for nid in 0..nodes.len() {
            if parked.contains(&nid) {
                continue;
            }
            let Some((_, _, op)) = record(nid) else {
                continue;
            };
            let unresolved = op.task_heads.iter().any(|hd| {
                by_hash
                    .get(hd)
                    .is_none_or(|ps| ps.iter().any(|p| parked.contains(p)))
            });
            if unresolved {
                parked.insert(nid);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    // ---- topological order: Kahn's algorithm, ready set by (hash, author) --
    let resolved: Vec<usize> = (0..nodes.len()).filter(|n| !parked.contains(n)).collect();
    let mut indeg: BTreeMap<usize, usize> = BTreeMap::new();
    let mut children: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &nid in &resolved {
        let Some((_, _, op)) = record(nid) else {
            continue;
        };
        let mut d = 0usize;
        for hd in &op.task_heads {
            for &p in by_hash.get(hd).map(Vec::as_slice).unwrap_or(&[]) {
                children.entry(p).or_default().push(nid);
                d = d.saturating_add(1);
            }
        }
        indeg.insert(nid, d);
    }
    let mut ready: BTreeSet<usize> = indeg
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(n, _)| *n)
        .collect();
    let mut order: Vec<usize> = Vec::with_capacity(resolved.len());
    while let Some(nid) = ready.pop_first() {
        order.push(nid);
        if let Some(cs) = children.get(&nid) {
            for c in cs {
                if let Some(d) = indeg.get_mut(c) {
                    *d = d.saturating_sub(1);
                    if *d == 0 {
                        ready.insert(*c);
                    }
                }
            }
        }
    }
    // anything never emitted sits on a reference cycle → park it too
    if order.len() < resolved.len() {
        let emitted: BTreeSet<usize> = order.iter().copied().collect();
        for &nid in &resolved {
            if !emitted.contains(&nid) {
                parked.insert(nid);
            }
        }
    }
    let parked: Vec<Hash32> = {
        let hs: BTreeSet<Hash32> = parked
            .iter()
            .filter_map(|&n| record(n).map(|(h, _, _)| *h))
            .collect();
        hs.into_iter().collect()
    };

    // ---- causal pasts ---------------------------------------------------
    let pos: BTreeMap<usize, usize> = order.iter().enumerate().map(|(p, n)| (*n, p)).collect();
    let mut anc: Vec<BTreeSet<usize>> = Vec::with_capacity(order.len());
    for &nid in &order {
        let mut set = BTreeSet::new();
        if let Some((_, _, op)) = record(nid) {
            for hd in &op.task_heads {
                for p in by_hash.get(hd).map(Vec::as_slice).unwrap_or(&[]) {
                    if let Some(&pp) = pos.get(p) {
                        set.insert(pp);
                        if let Some(a) = anc.get(pp) {
                            set.extend(a.iter().copied());
                        }
                    }
                }
            }
        }
        anc.push(set);
    }
    let rec = |p: usize| -> Option<&OpRecord> { record(*order.get(p)?) };

    // ---- releases per author, in topo positions -------------------------
    let mut releases: BTreeMap<IdentityKey, Vec<usize>> = BTreeMap::new();
    for p in 0..order.len() {
        if let Some((_, author, op)) = rec(p)
            && op.op == TaskKind::Release
        {
            releases.entry(*author).or_default().push(p);
        }
    }
    let in_past = |what: usize, of: usize| anc.get(of).is_some_and(|a| a.contains(&what));
    // a Take at `t` by `author` is live within the past of `scope` (or
    // globally when scope is None) iff no Release by the author lies
    // outside the Take's past (within that scope)
    let take_live = |t: usize, author: &IdentityKey, scope: Option<usize>| -> bool {
        releases.get(author).is_none_or(|rs| {
            rs.iter().all(|&r| {
                let considered = scope.is_none_or(|s| in_past(r, s));
                !considered || in_past(r, t)
            })
        })
    };

    // ---- done? ----------------------------------------------------------
    for d in 0..order.len() {
        let Some((_, author, op)) = rec(d) else {
            continue;
        };
        if op.op != TaskKind::Done {
            continue;
        }
        let claimant = (0..d).any(|t| {
            rec(t).is_some_and(|(_, a, o)| o.op == TaskKind::Take && a == author && in_past(t, d))
                && take_live(t, author, Some(d))
        });
        if claimant {
            return Fold {
                state: TaskState::Done,
                parked: parked.clone(),
            };
        }
    }

    // ---- live claims: first live Take per author, in topo order ---------
    let mut claims: Vec<(usize, Hash32, IdentityKey)> = Vec::new();
    let mut seen: BTreeSet<IdentityKey> = BTreeSet::new();
    for t in 0..order.len() {
        let Some((h, author, op)) = rec(t) else {
            continue;
        };
        if op.op == TaskKind::Take && !seen.contains(author) && take_live(t, author, None) {
            seen.insert(*author);
            claims.push((t, *h, *author));
        }
    }
    if claims.is_empty() {
        return Fold {
            state: TaskState::Open,
            parked: parked.clone(),
        };
    }

    // ---- conflict set: claims concurrent with at least one other --------
    let concurrent = |a: usize, b: usize| !in_past(a, b) && !in_past(b, a);
    let conflict: Vec<(Hash32, IdentityKey)> = claims
        .iter()
        .filter(|(p, _, _)| claims.iter().any(|(q, _, _)| q != p && concurrent(*p, *q)))
        .map(|(_, h, k)| (*h, *k))
        .collect();
    let state = match tiebreak::winner(task_id, &conflict) {
        Some(winner) if conflict.len() >= 2 => {
            let mut losers: Vec<IdentityKey> = conflict
                .iter()
                .map(|(_, k)| *k)
                .filter(|k| *k != winner)
                .collect();
            losers.sort_unstable();
            let in_conflict: BTreeSet<IdentityKey> = conflict.iter().map(|(_, k)| *k).collect();
            let mut second: Vec<IdentityKey> = claims
                .iter()
                .map(|(_, _, k)| *k)
                .filter(|k| !in_conflict.contains(k))
                .collect();
            second.sort_unstable();
            TaskState::Conflict {
                winner,
                losers,
                second,
            }
        }
        _ => TaskState::Taken {
            holders: claims.iter().map(|(_, _, k)| *k).collect(),
        },
    };
    Fold { state, parked }
}

/// What this phone knows about how far its own claim has travelled
/// (BRIEF_v2 §4 "Taken by you · Provisional (people near you only)" versus
/// "· no conflict seen"). Transport evidence, not protocol state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReachEvidence {
    /// The claim has only been seen by people near you.
    pub local_only: bool,
    /// The claim has reached people beyond those near you when you took it
    /// (after a reconnect), so a conflict would have had a chance to arrive.
    pub seen_beyond_reach: bool,
    /// A competing claim has arrived (informational; the state carries it).
    pub conflict_seen: bool,
}

/// The brief's state strings (BRIEF_v2 §4 "States in plain UI", §7
/// vocabulary: "Take", "Provisional" + "people near you only"). `name_of`
/// renders another member's key for display.
pub fn ui_state(
    state: &TaskState,
    me: &IdentityKey,
    reach: ReachEvidence,
    name_of: &dyn Fn(&IdentityKey) -> String,
) -> String {
    let people = |n: usize| format!("{n} people on this");
    match state {
        TaskState::Open => "Open".to_string(),
        TaskState::Done => "Done".to_string(),
        TaskState::Taken { holders } => {
            let mut s = if holders.contains(me) {
                if reach.local_only || !reach.seen_beyond_reach {
                    "Taken by you · Provisional (people near you only)".to_string()
                } else {
                    "Taken by you · no conflict seen".to_string()
                }
            } else {
                match holders.first() {
                    Some(k) => format!("Taken by {}", name_of(k)),
                    None => "Open".to_string(),
                }
            };
            if holders.len() > 1 {
                s.push_str(" · ");
                s.push_str(&people(holders.len()));
            }
            s
        }
        TaskState::Conflict {
            winner,
            losers,
            second,
        } => {
            let total = 1 + losers.len() + second.len();
            let mut s = if me == winner {
                match losers.first() {
                    Some(l) => format!("Conflict: {} also took this", name_of(l)),
                    None => "Taken by you · no conflict seen".to_string(),
                }
            } else if losers.contains(me) || second.contains(me) {
                format!("Conflict: {} also took this", name_of(winner))
            } else {
                match losers.first() {
                    Some(l) => format!(
                        "Taken by {} · Conflict: {} also took this",
                        name_of(winner),
                        name_of(l)
                    ),
                    None => format!("Taken by {}", name_of(winner)),
                }
            };
            if total > 2 {
                s.push_str(" · ");
                s.push_str(&people(total));
            }
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::ed25519::IdentitySigner;

    fn key(i: u8) -> IdentityKey {
        IdentitySigner::from_seed(&[i; 32]).public()
    }

    fn h(i: u8) -> Hash32 {
        Hash32::from_bytes([i; 32])
    }

    const TID: TaskId = [0x42; 16];

    fn op(i: u8, who: u8, kind: TaskKind, heads: &[u8]) -> OpRecord {
        let heads = heads.iter().map(|&x| h(x)).collect();
        let t = match kind {
            TaskKind::Create => TaskOp::create("t", "d").unwrap(),
            k => TaskOp::on_task(TID, k, heads).unwrap(),
        };
        (h(i), key(who), t)
    }

    #[test]
    fn empty_and_create_only_are_open() {
        assert_eq!(fold(&TID, &[]).state, TaskState::Open);
        let f = fold(&TID, &[op(1, 1, TaskKind::Create, &[])]);
        assert_eq!(f.state, TaskState::Open);
        assert!(f.parked.is_empty());
    }

    #[test]
    fn take_then_second_claim_then_release_then_done() {
        let create = op(1, 1, TaskKind::Create, &[]);
        let take_a = op(2, 2, TaskKind::Take, &[1]);
        let f = fold(&TID, &[create.clone(), take_a.clone()]);
        assert_eq!(
            f.state,
            TaskState::Taken {
                holders: vec![key(2)]
            }
        );
        // B saw A's Take and took anyway: second claim, "2 people on this"
        let take_b = op(3, 3, TaskKind::Take, &[2]);
        let f = fold(&TID, &[take_b.clone(), create.clone(), take_a.clone()]);
        assert_eq!(
            f.state,
            TaskState::Taken {
                holders: vec![key(2), key(3)]
            }
        );
        // A releases → only B holds
        let rel_a = op(4, 2, TaskKind::Release, &[3]);
        let f = fold(
            &TID,
            &[
                create.clone(),
                take_a.clone(),
                take_b.clone(),
                rel_a.clone(),
            ],
        );
        assert_eq!(
            f.state,
            TaskState::Taken {
                holders: vec![key(3)]
            }
        );
        // a Release from a non-holder changes nothing
        let rel_x = op(5, 9, TaskKind::Release, &[4]);
        let f = fold(
            &TID,
            &[
                create.clone(),
                take_a.clone(),
                take_b.clone(),
                rel_a.clone(),
                rel_x,
            ],
        );
        assert_eq!(
            f.state,
            TaskState::Taken {
                holders: vec![key(3)]
            }
        );
        // Done by B (a claimant) closes it; Done by A after release is ignored
        let done_a = op(6, 2, TaskKind::Done, &[4]);
        let f = fold(
            &TID,
            &[
                create.clone(),
                take_a.clone(),
                take_b.clone(),
                rel_a.clone(),
                done_a,
            ],
        );
        assert_eq!(
            f.state,
            TaskState::Taken {
                holders: vec![key(3)]
            }
        );
        let done_b = op(7, 3, TaskKind::Done, &[4]);
        let f = fold(
            &TID,
            &[
                create.clone(),
                take_a.clone(),
                take_b.clone(),
                rel_a.clone(),
                done_b.clone(),
            ],
        );
        assert_eq!(f.state, TaskState::Done);
        // Done is idempotent (a repeated hash collapses, a concurrent Done
        // is fine) and survives later Takes
        let take_late = op(8, 5, TaskKind::Take, &[7]);
        let done_b2 = op(9, 3, TaskKind::Done, &[4]);
        let f = fold(
            &TID,
            &[
                done_b.clone(),
                create,
                take_a,
                take_b,
                rel_a,
                done_b,
                take_late,
                done_b2,
            ],
        );
        assert_eq!(f.state, TaskState::Done);
        assert!(f.parked.is_empty());
    }

    #[test]
    fn concurrent_takes_conflict_and_resolve_by_tiebreak() {
        let create = op(1, 1, TaskKind::Create, &[]);
        let take_a = op(2, 2, TaskKind::Take, &[1]);
        let take_b = op(3, 3, TaskKind::Take, &[1]);
        let f = fold(&TID, &[create.clone(), take_a.clone(), take_b.clone()]);
        let expect = tiebreak::winner(&TID, &[(h(2), key(2)), (h(3), key(3))]).unwrap();
        let loser = if expect == key(2) { key(3) } else { key(2) };
        assert_eq!(
            f.state,
            TaskState::Conflict {
                winner: expect,
                losers: vec![loser],
                second: vec![]
            }
        );
        // input order does not matter
        let g = fold(&TID, &[take_b.clone(), take_a.clone(), create.clone()]);
        assert_eq!(g, f);
        // C saw both and took anyway: second claim, not in the conflict set
        let take_c = op(4, 4, TaskKind::Take, &[2, 3]);
        let f = fold(
            &TID,
            &[create.clone(), take_a.clone(), take_b.clone(), take_c],
        );
        assert_eq!(
            f.state,
            TaskState::Conflict {
                winner: expect,
                losers: vec![loser],
                second: vec![key(4)]
            }
        );
        // the loser releases → the winner holds alone
        let loser_id = if loser == key(2) { 2 } else { 3 };
        let rel = op(5, loser_id, TaskKind::Release, &[2, 3]);
        let f = fold(&TID, &[create.clone(), take_a.clone(), take_b.clone(), rel]);
        assert_eq!(
            f.state,
            TaskState::Taken {
                holders: vec![expect]
            }
        );
        // the loser's "Keep helping" emits nothing: still a conflict, and a
        // Done by the loser (a current claimant) closes the task
        let done = op(6, loser_id, TaskKind::Done, &[2, 3]);
        let f = fold(&TID, &[create, take_a, take_b, done]);
        assert_eq!(f.state, TaskState::Done);
    }

    #[test]
    fn release_then_retake_is_a_fresh_claim() {
        let take = op(2, 2, TaskKind::Take, &[]);
        let rel = op(3, 2, TaskKind::Release, &[2]);
        let retake = op(4, 2, TaskKind::Take, &[3]);
        assert_eq!(
            fold(&TID, &[take.clone(), rel.clone()]).state,
            TaskState::Open
        );
        assert_eq!(
            fold(&TID, &[take.clone(), rel.clone(), retake]).state,
            TaskState::Taken {
                holders: vec![key(2)]
            }
        );
        // a Release concurrent with the Take also cancels it (later-or-concurrent)
        let rel_conc = op(5, 2, TaskKind::Release, &[]);
        assert_eq!(fold(&TID, &[take, rel_conc]).state, TaskState::Open);
    }

    #[test]
    fn unknown_heads_and_cycles_are_parked() {
        let take = op(2, 2, TaskKind::Take, &[99]);
        let f = fold(&TID, std::slice::from_ref(&take));
        assert_eq!(f.state, TaskState::Open);
        assert_eq!(f.parked, vec![h(2)]);
        // transitively parked
        let child = op(3, 3, TaskKind::Take, &[2]);
        let f = fold(&TID, &[take, child]);
        assert_eq!(f.parked, vec![h(2), h(3)]);
        assert_eq!(f.state, TaskState::Open);
        // a reference cycle (impossible with honest hashes) never loops
        let a = op(4, 4, TaskKind::Take, &[5]);
        let b = op(5, 5, TaskKind::Take, &[4]);
        let ok = op(6, 6, TaskKind::Take, &[]);
        let f = fold(&TID, &[a, b, ok]);
        assert_eq!(f.parked, vec![h(4), h(5)]);
        assert_eq!(
            f.state,
            TaskState::Taken {
                holders: vec![key(6)]
            }
        );
    }

    #[test]
    fn identical_post_bytes_from_two_authors_are_two_claims() {
        // DESIGN §2.4: `op hash = SHA256(post plaintext)` and the post
        // plaintext carries no author field, so two members who Take right
        // after the Create (same task_id, same heads, same roster heads)
        // emit byte-identical posts with one hash. They are two concurrent
        // claims and must conflict, whichever copy a phone saw first.
        let create = op(1, 1, TaskKind::Create, &[]);
        let take_a = op(2, 2, TaskKind::Take, &[1]);
        let take_b = op(2, 3, TaskKind::Take, &[1]);
        let expect = tiebreak::winner(&TID, &[(h(2), key(2)), (h(2), key(3))]).unwrap();
        let loser = if expect == key(2) { key(3) } else { key(2) };
        let f = fold(&TID, &[create.clone(), take_a.clone(), take_b.clone()]);
        assert_eq!(
            f.state,
            TaskState::Conflict {
                winner: expect,
                losers: vec![loser],
                second: vec![]
            }
        );
        let g = fold(&TID, &[take_b.clone(), create.clone(), take_a.clone()]);
        assert_eq!(g, f);
        // a later op citing that hash is after both copies
        let take_c = op(4, 4, TaskKind::Take, &[2]);
        let f = fold(
            &TID,
            &[create.clone(), take_a.clone(), take_b.clone(), take_c],
        );
        assert_eq!(
            f.state,
            TaskState::Conflict {
                winner: expect,
                losers: vec![loser],
                second: vec![key(4)]
            }
        );
        // the loser's Release (same bytes as any other Release citing [2]
        // would have) cancels only the loser's claim
        let loser_id = if loser == key(2) { 2 } else { 3 };
        let rel = op(5, loser_id, TaskKind::Release, &[2]);
        let f = fold(&TID, &[create.clone(), take_a.clone(), take_b.clone(), rel]);
        assert_eq!(
            f.state,
            TaskState::Taken {
                holders: vec![expect]
            }
        );
        // the exact same record twice is still one op; parked hashes are listed once
        let f = fold(&TID, &[take_a.clone(), take_a.clone(), create.clone()]);
        assert_eq!(
            f.state,
            TaskState::Taken {
                holders: vec![key(2)]
            }
        );
        let dangling_a = op(7, 2, TaskKind::Take, &[99]);
        let dangling_b = op(7, 3, TaskKind::Take, &[99]);
        let f = fold(&TID, &[dangling_a, dangling_b, create, take_a]);
        assert_eq!(f.parked, vec![h(7)]);
        assert_eq!(
            f.state,
            TaskState::Taken {
                holders: vec![key(2)]
            }
        );
    }

    #[test]
    fn ui_strings_follow_the_brief() {
        let me = key(1);
        let names = |k: &IdentityKey| {
            if *k == key(2) {
                "Ravi".to_string()
            } else {
                "Ana".to_string()
            }
        };
        let local = ReachEvidence {
            local_only: true,
            ..Default::default()
        };
        let beyond = ReachEvidence {
            seen_beyond_reach: true,
            ..Default::default()
        };
        assert_eq!(ui_state(&TaskState::Open, &me, local, &names), "Open");
        assert_eq!(ui_state(&TaskState::Done, &me, local, &names), "Done");
        let mine = TaskState::Taken { holders: vec![me] };
        assert_eq!(
            ui_state(&mine, &me, local, &names),
            "Taken by you · Provisional (people near you only)"
        );
        assert_eq!(
            ui_state(&mine, &me, beyond, &names),
            "Taken by you · no conflict seen"
        );
        let two = TaskState::Taken {
            holders: vec![key(2), me],
        };
        assert_eq!(
            ui_state(&two, &me, beyond, &names),
            "Taken by you · no conflict seen · 2 people on this"
        );
        assert_eq!(
            ui_state(&two, &key(3), beyond, &names),
            "Taken by Ravi · 2 people on this"
        );
        let conflict = TaskState::Conflict {
            winner: me,
            losers: vec![key(2)],
            second: vec![],
        };
        assert_eq!(
            ui_state(&conflict, &me, beyond, &names),
            "Conflict: Ravi also took this"
        );
        assert_eq!(
            ui_state(&conflict, &key(2), beyond, &names),
            "Conflict: Ana also took this"
        );
        assert_eq!(
            ui_state(&conflict, &key(3), beyond, &names),
            "Taken by Ana · Conflict: Ravi also took this"
        );
        let three = TaskState::Conflict {
            winner: key(2),
            losers: vec![key(3)],
            second: vec![me],
        };
        assert_eq!(
            ui_state(&three, &me, beyond, &names),
            "Conflict: Ravi also took this · 3 people on this"
        );
        assert_eq!(three.holders(), vec![key(2), key(3), me]);
    }
}
