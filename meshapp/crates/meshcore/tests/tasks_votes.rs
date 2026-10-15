//! Tasks and votes integration tests (docs/DESIGN.md §10: the `final_rule`
//! and `task_tiebreak` property tests, the brief's threshold table, the S4
//! walkthrough, task fold scenarios and decoder fuzzing).
//!
//! `PROPTEST_CASES=10000` runs the long version locally; the default is 512.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use meshcore::codec::{decode, encode};
use meshcore::crypto::backend::GroupId;
use meshcore::crypto::ed25519::{IdentityKey, IdentitySigner};
use meshcore::crypto::hash::{Hash32, sha256};
use meshcore::objects::ballot::Ballot;
use meshcore::objects::chat_post::PostPlain;
use meshcore::objects::task_op::{TaskId, TaskKind, TaskOp, task_id_for_create, task_op_hash};
use meshcore::objects::vote_open::{VoteOpen, threshold};
use meshcore::objects::{SignedHeader, kind};
use meshcore::tasks::state::{Fold, OpRecord, TaskState, fold};
use meshcore::tasks::tiebreak;
use meshcore::votes::open::{Branch, ChainCheck, ValidOpen, validate_open};
use meshcore::votes::state::{notification, state_label, tally_line};
use meshcore::votes::tally::{Tally, VoteState, tally, tally_payloads};
use proptest::prelude::*;

// ---- fixture ----------------------------------------------------------------

const MAX_N: usize = 60;

fn signers() -> &'static Vec<IdentitySigner> {
    static S: OnceLock<Vec<IdentitySigner>> = OnceLock::new();
    S.get_or_init(|| {
        (0..MAX_N + 4)
            .map(|i| {
                let mut seed = [0u8; 32];
                seed[0] = i as u8;
                seed[1] = 0x5A;
                IdentitySigner::from_seed(&seed)
            })
            .collect()
    })
}

fn signer(i: usize) -> &'static IdentitySigner {
    &signers()[i]
}

fn key(i: usize) -> IdentityKey {
    signer(i).public()
}

fn members(n: usize) -> BTreeSet<IdentityKey> {
    (0..n).map(key).collect()
}

fn open_vote(n: usize, f: u8, options: &[&str]) -> (VoteOpen, ValidOpen) {
    let s = signer(0);
    let header = SignedHeader::new(
        kind::VOTE_OPEN,
        GroupId([0x33; 16]),
        s.public(),
        1,
        Hash32::from_bytes([1; 32]),
        vec![Hash32::from_bytes([0xaa; 32])],
    )
    .unwrap();
    let n16 = u16::try_from(n).unwrap();
    let open = VoteOpen::sign(
        s,
        header,
        "Move the kitchen to the school?",
        options.iter().map(|o| o.to_string()).collect(),
        f,
        threshold(n16, f),
        n16,
    )
    .unwrap();
    let valid = validate_open(&open, &members(n), Branch::A, false, ChainCheck::Ok).unwrap();
    (open, valid)
}

fn proptest_cases() -> u32 {
    std::env::var("PROPTEST_CASES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(512)
}

// ---- threshold table (BRIEF_v2 §4 "Final rule") --------------------------------

#[test]
fn fixed_threshold_table() {
    let table: &[(u8, u16, u16)] = &[
        (1, 12, 7),
        (1, 13, 8),
        (1, 21, 12),
        (1, 40, 21),
        (2, 12, 8),
        (2, 13, 8),
        (2, 21, 12),
        (2, 40, 22),
    ];
    for &(f, n, t) in table {
        assert_eq!(threshold(n, f), t, "f={f} n={n}");
        let (open, valid) = open_vote(usize::from(n), f, &["Yes", "No"]);
        assert_eq!(valid.t(), t);
        assert_eq!(open.t, t);
        let m = members(usize::from(n));
        // t−1 signers for Yes: not Final; t signers: Final
        let below: Vec<(IdentityKey, u8)> = (0..usize::from(t) - 1).map(|i| (key(i), 0)).collect();
        assert!(
            !tally_payloads(&valid, &m, below).state.is_final(),
            "f={f} n={n}: {} is below T",
            t - 1
        );
        let at: Vec<(IdentityKey, u8)> = (0..usize::from(t)).map(|i| (key(i), 0)).collect();
        assert_eq!(
            tally_payloads(&valid, &m, at).state,
            VoteState::Final { option: 0 },
            "f={f} n={n}"
        );
        // the brief's f_eff is always ≥ f
        assert!(valid.f_eff() >= i64::from(f));
    }
}

// ---- final_rule property test (DESIGN §10) -----------------------------------

/// Per key: no ballot, one option, or two different options.
#[derive(Debug, Clone, Copy)]
enum Assign {
    None,
    One(u8),
    Two(u8, u8),
}

fn assign_strategy(n_opt: u8) -> impl Strategy<Value = Assign> {
    prop_oneof![
        2 => Just(Assign::None),
        5 => (0..n_opt).prop_map(Assign::One),
        1 => (0..n_opt, 1..n_opt).prop_map(move |(a, d)| Assign::Two(a, (a + d) % n_opt)),
    ]
}

fn payloads_of(assigns: &[Assign]) -> Vec<(IdentityKey, u8)> {
    let mut out = Vec::new();
    for (i, a) in assigns.iter().enumerate() {
        match *a {
            Assign::None => {}
            Assign::One(x) => out.push((key(i), x)),
            Assign::Two(x, y) => {
                out.push((key(i), x));
                out.push((key(i), y));
            }
        }
    }
    out
}

fn subset(payloads: &[(IdentityKey, u8)], mask: &[bool]) -> Vec<(IdentityKey, u8)> {
    payloads
        .iter()
        .zip(mask.iter().chain(std::iter::repeat(&false)))
        .filter(|(_, m)| **m)
        .map(|(p, _)| *p)
        .collect()
}

proptest! {
    #![proptest_config(ProptestConfig { cases: proptest_cases(), ..ProptestConfig::default() })]

    #[test]
    fn final_rule(
        n in 3usize..=MAX_N,
        f in 1u8..=3,
        n_opt in 2u8..=3,
        assigns in proptest::collection::vec(assign_strategy(3), MAX_N),
        mask_a in proptest::collection::vec(any::<bool>(), 2 * MAX_N),
        mask_b in proptest::collection::vec(any::<bool>(), 2 * MAX_N),
        grow in proptest::collection::vec(any::<bool>(), 2 * MAX_N),
    ) {
        let options: Vec<&str> = ["A", "B", "C"][..usize::from(n_opt)].to_vec();
        let (_, valid) = open_vote(n, f, &options);
        let m = members(n);
        // clamp assignments to the option count
        let assigns: Vec<Assign> = assigns[..n].iter().map(|a| match *a {
            Assign::One(x) => Assign::One(x % n_opt),
            Assign::Two(x, y) => { let (x, y) = (x % n_opt, y % n_opt); if x == y { Assign::One(x) } else { Assign::Two(x, y) } }
            Assign::None => Assign::None,
        }).collect();
        let full = payloads_of(&assigns);
        let f_eff = valid.f_eff();
        prop_assert!(f_eff >= i64::from(f));

        // (1) any two subsets that both yield Final agree on the option
        let ta = tally_payloads(&valid, &m, subset(&full, &mask_a));
        let tb = tally_payloads(&valid, &m, subset(&full, &mask_b));
        if let (VoteState::Final { option: x }, VoteState::Final { option: y }) = (&ta.state, &tb.state) {
            prop_assert_eq!(x, y, "two Final subsets disagree");
        }

        // (2) adding ballots to a Final(X) set yields Final(X) or Disputed
        if let VoteState::Final { option: x } = ta.state {
            let grown: Vec<bool> = mask_a.iter().zip(grow.iter()).map(|(a, g)| *a || *g).collect();
            let tg = tally_payloads(&valid, &m, subset(&full, &grown));
            match tg.state {
                VoteState::Final { option } => prop_assert_eq!(option, x, "Final flipped"),
                VoteState::Disputed { .. } => {}
                ref other => prop_assert!(false, "Final regressed to {other:?}"),
            }
            let tf = tally_payloads(&valid, &m, full.clone());
            let full_ok = matches!(tf.state, VoteState::Final { option } if option == x) || tf.state.is_disputed();
            prop_assert!(full_ok, "full set regressed");
        }

        // (3) with void > f_eff the full set is Disputed and names the equivocators
        let tf = tally_payloads(&valid, &m, full.clone());
        let voids = assigns.iter().filter(|a| matches!(a, Assign::Two(..))).count();
        prop_assert_eq!(tf.void_keys.len(), voids);
        if i64::try_from(voids).unwrap() > f_eff {
            prop_assert!(tf.state.is_disputed(), "void {voids} > f_eff {f_eff} but {:?}", tf.state);
            if let VoteState::Disputed { members } = &tf.state {
                prop_assert_eq!(members.len(), voids);
            }
        } else {
            prop_assert!(!tf.state.is_disputed());
        }

        // (4) bookkeeping: heard + not_heard = N; s ≥ displayed; order-independence
        prop_assert_eq!(tf.heard + tf.not_heard.len(), n);
        for (s, d) in tf.s.iter().zip(tf.displayed.iter()) {
            prop_assert!(s >= d);
        }
        let mut rev = full.clone();
        rev.reverse();
        rev.extend_from_slice(&full); // duplicates collapse
        prop_assert_eq!(tally_payloads(&valid, &m, rev), tf);
    }
}

// ---- S4 walkthrough (DESIGN §10 S4; DECISIONS D1 "Deciding walkthrough") -----

fn ballots(open: &ValidOpen, yes: &[usize], no: &[usize]) -> Vec<Ballot> {
    let vid = open.vote_id();
    yes.iter()
        .map(|&i| Ballot::sign(signer(i), vid, 0))
        .chain(no.iter().map(|&i| Ballot::sign(signer(i), vid, 1)))
        .collect()
}

#[test]
fn s4_courier_with_two_payloads_is_void_and_never_final() {
    // N = 21, f = 1, T = 12, f_eff = 2. Keys 1..=10 Yes, 11..=20 No, Z = key 0.
    let (_, valid) = open_vote(21, 1, &["Yes", "No"]);
    assert_eq!(valid.t(), 12);
    assert_eq!(valid.f_eff(), 2);
    let m = members(21);
    let yes: Vec<usize> = (1..=10).collect();
    let no: Vec<usize> = (11..=20).collect();
    let mut b = ballots(&valid, &yes, &no);
    // Z sent No in F2, then swapped to Yes in the bundle carried to F1
    b.push(Ballot::sign(signer(0), valid.vote_id(), 1));
    b.push(Ballot::sign(signer(0), valid.vote_id(), 0));
    // ballots arrive in any order on any node
    for rotate in [0usize, 5, 13, 21] {
        let mut order = b.clone();
        order.rotate_left(rotate);
        let t = tally(&valid, &m, &order);
        assert_eq!(tally_line(&valid, &t), "Y10 N10 · 1 void · Provisional");
        assert_eq!(t.s, vec![11, 11]);
        assert_eq!(t.displayed, vec![10, 10]);
        assert_eq!(t.void_keys, BTreeSet::from([key(0)]));
        assert_eq!(t.heard, 21);
        assert_eq!(
            t.state,
            VoteState::ResultProvisional {
                leading: None,
                needs_t: 12,
                k_not_heard: 0
            }
        );
        assert_eq!(
            state_label(&t),
            "Result (provisional) · needs 12 for Final · 0 not heard from"
        );
        assert_eq!(notification(None, &t.state), None);
    }
    // F1 before the merge: 11 Yes signers (Z's Yes included) < 12 → never Final
    let mut f1 = ballots(&valid, &yes, &[]);
    f1.push(Ballot::sign(signer(0), valid.vote_id(), 0));
    let t = tally(&valid, &m, &f1);
    assert!(!t.state.is_final());
    assert_eq!(t.s, vec![11, 0]);
}

#[test]
fn s4_honest_courier_stays_provisional_and_lopsided_is_final_forever() {
    let (_, valid) = open_vote(21, 1, &["Yes", "No"]);
    let m = members(21);
    // honest courier: 11–10
    let t = tally(
        &valid,
        &m,
        &ballots(
            &valid,
            &(0..=10).collect::<Vec<_>>(),
            &(11..=20).collect::<Vec<_>>(),
        ),
    );
    assert_eq!(tally_line(&valid, &t), "Y11 N10 · Provisional");
    assert_eq!(
        t.state,
        VoteState::ResultProvisional {
            leading: Some(0),
            needs_t: 12,
            k_not_heard: 0
        }
    );
    // lopsided: 14–7 → Final(Yes) on every node after the merge, and never changes after
    let yes: Vec<usize> = (0..14).collect();
    let no: Vec<usize> = (14..21).collect();
    let all = ballots(&valid, &yes, &no);
    let t = tally(&valid, &m, &all);
    assert_eq!(t.state, VoteState::Final { option: 0 });
    assert_eq!(tally_line(&valid, &t), "Y14 N7 · Final");
    assert_eq!(
        notification(None, &t.state),
        Some(meshcore::votes::state::Notification::Final { option: 0 })
    );
    // the first 12 Yes alone already make it Final; later ballots (a +25 min clock
    // is irrelevant: nothing reads a clock) only add to the count
    let twelve = tally(&valid, &m, &all[..12]);
    assert_eq!(twelve.state, VoteState::Final { option: 0 });
    for cut in 12..=all.len() {
        assert_eq!(
            tally(&valid, &m, &all[..cut]).state,
            VoteState::Final { option: 0 }
        );
    }
    // one of the No voters later equivocates: still Final (void 1 ≤ f_eff 2)
    let mut more = all.clone();
    more.push(Ballot::sign(signer(14), valid.vote_id(), 0));
    assert_eq!(
        tally(&valid, &m, &more).state,
        VoteState::Final { option: 0 }
    );
    // three equivocators (> f_eff): Disputed, visibly, naming them
    more.push(Ballot::sign(signer(15), valid.vote_id(), 0));
    more.push(Ballot::sign(signer(16), valid.vote_id(), 0));
    let t = tally(&valid, &m, &more);
    assert_eq!(
        t.state,
        VoteState::Disputed {
            members: vec![key(14), key(15), key(16)]
                .into_iter()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect()
        }
    );
    assert_eq!(
        state_label(&t),
        "Disputed · 3 members sent conflicting ballots"
    );
}

#[test]
fn ballots_from_outside_members_of_h_are_dropped_and_payloads_collapse() {
    let (_, valid) = open_vote(12, 1, &["Yes", "No"]);
    let m = members(12);
    let mut b = ballots(&valid, &(0..7).collect::<Vec<_>>(), &[]);
    // a key added after the opening (not in members(H)) cannot vote
    b.push(Ballot::sign(signer(MAX_N + 1), valid.vote_id(), 1));
    // the same payload signed twice (randomised signatures) counts once
    b.push(Ballot::sign(signer(1), valid.vote_id(), 0));
    let t = tally(&valid, &m, &b);
    assert_eq!(t.rejected, 1);
    assert_eq!(t.s, vec![7, 0]);
    assert_eq!(t.state, VoteState::Final { option: 0 });
}

// ---- tie-break property test (DESIGN §10 task_tiebreak) ----------------------

proptest! {
    #![proptest_config(ProptestConfig { cases: proptest_cases().min(2000), ..ProptestConfig::default() })]

    #[test]
    fn tiebreak_is_permutation_invariant_and_rerandomises(
        task_id in any::<[u8; 16]>(),
        hashes in proptest::collection::vec(any::<[u8; 32]>(), 2..=6),
        perm_seed in any::<u64>(),
        which in 0usize..6,
        new_hash in any::<[u8; 32]>(),
    ) {
        let set: Vec<(Hash32, IdentityKey)> = hashes.iter().enumerate().map(|(i, h)| (Hash32::from_bytes(*h), key(i))).collect();
        let w = tiebreak::winner(&task_id, &set).unwrap();
        prop_assert!(set.iter().any(|(_, k)| *k == w));
        // any permutation (a deterministic shuffle from perm_seed)
        let mut perm = set.clone();
        let mut x = perm_seed;
        for i in (1..perm.len()).rev() {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let j = usize::try_from(x >> 33).unwrap() % (i + 1);
            perm.swap(i, j);
        }
        prop_assert_eq!(tiebreak::winner(&task_id, &perm), Some(w));
        // changing any one op hash re-randomises the ranking: over 64 variants
        // of that hash the winner is not constant
        let which = which % set.len();
        let mut winners = BTreeSet::new();
        for v in 0u8..64 {
            let mut changed = set.clone();
            let mut h = new_hash;
            h[0] ^= v;
            changed[which].0 = Hash32::from_bytes(h);
            winners.insert(tiebreak::winner(&task_id, &changed).unwrap());
        }
        prop_assert!(winners.len() >= 2, "64 different rival hashes never changed the winner");
    }
}

#[test]
fn tiebreak_is_fair_over_many_tasks() {
    // the same 4 keys conflict on 2000 tasks; no key wins more than 1/|C| + 0.1
    const TASKS: usize = 2000;
    let keys: Vec<IdentityKey> = (0..4).map(key).collect();
    let mut wins: BTreeMap<IdentityKey, usize> = BTreeMap::new();
    for t in 0..TASKS {
        let task_id: TaskId = sha256(&(t as u64).to_le_bytes()).truncate16();
        let set: Vec<(Hash32, IdentityKey)> = keys
            .iter()
            .enumerate()
            .map(|(i, k)| (sha256(&[task_id.as_slice(), &[i as u8]].concat()), *k))
            .collect();
        let w = tiebreak::winner(&task_id, &set).unwrap();
        *wins.entry(w).or_default() += 1;
    }
    for (k, n) in &wins {
        let share = *n as f64 / TASKS as f64;
        assert!(share <= 0.25 + 0.1, "{k} won {share}");
    }
    assert_eq!(wins.len(), 4, "every key wins sometimes");
}

// ---- task fold scenarios (DESIGN §4 "Fold"; BRIEF_v2 §4 Tasks) ---------------

/// A tiny post pipeline over the real post layout: each op travels as the
/// body of a `PostPlain` of kind 0x31 (`ver ‖ kind ‖ n_heads ‖ roster_heads
/// ‖ body`, DESIGN §2.4) and its identity is the author-bound
/// `task_op_hash(author, post plaintext)`. The plaintext has no author
/// field, so two members who emit the same op citing the same heads produce
/// byte-identical posts; the author binding keeps them two ops.
struct Pipeline {
    task_id: TaskId,
    ops: Vec<OpRecord>,
}

/// Every member cites the same roster heads, as members of one group in one
/// roster state would.
fn roster_heads() -> Vec<Hash32> {
    vec![
        Hash32::from_bytes([0xaa; 32]),
        Hash32::from_bytes([0x0b; 32]),
    ]
}

fn post_plaintext(op: &TaskOp) -> Vec<u8> {
    let post = PostPlain::new(kind::POST_TASK_OP, roster_heads(), encode(op)).unwrap();
    let bytes = encode(&post);
    // the layout the hash covers: ver, kind, two sorted heads, the body
    assert_eq!(&bytes[..3], &[1u8, kind::POST_TASK_OP, 2]);
    assert_eq!(&bytes[3..35], &[0x0b; 32]);
    assert_eq!(&bytes[35..67], &[0xaa; 32]);
    assert_eq!(&bytes[67..], &encode(op)[..]);
    bytes
}

fn create_post() -> (TaskOp, Vec<u8>) {
    let create = TaskOp::create("Water run", "2 jerry cans to the school").unwrap();
    let post = post_plaintext(&create);
    (create, post)
}

impl Pipeline {
    fn new() -> Self {
        Self::created_by(0)
    }

    fn created_by(author: usize) -> Self {
        let (create, post) = create_post();
        let task_id = task_id_for_create(&key(author), &post);
        let mut p = Pipeline {
            task_id,
            ops: Vec::new(),
        };
        p.ops
            .push((task_op_hash(&key(author), &post), key(author), create));
        p
    }

    fn emit(&mut self, author: usize, kind: TaskKind, heads: &[Hash32]) -> Hash32 {
        let op = TaskOp::on_task(self.task_id, kind, heads.to_vec()).unwrap();
        let h = task_op_hash(&key(author), &post_plaintext(&op));
        self.ops.push((h, key(author), op));
        h
    }

    /// The post plaintext of the record at `i`, as its author sent it.
    fn plaintext(&self, i: usize) -> Vec<u8> {
        post_plaintext(&self.ops[i].2)
    }

    fn fold(&self) -> Fold {
        fold(&self.task_id, &self.ops)
    }

    fn fold_reversed(&self) -> Fold {
        let mut ops = self.ops.clone();
        ops.reverse();
        fold(&self.task_id, &ops)
    }
}

#[test]
fn second_claim_versus_conflict_versus_release_versus_done() {
    let mut p = Pipeline::new();
    let create = p.ops[0].0;
    assert_eq!(p.fold().state, TaskState::Open);

    // Ana takes
    let take_a = p.emit(1, TaskKind::Take, &[create]);
    assert_eq!(
        p.fold().state,
        TaskState::Taken {
            holders: vec![key(1)]
        }
    );

    // Ravi saw Ana's Take and took anyway: second claim, "2 people on this"
    let take_r = p.emit(2, TaskKind::Take, &[take_a]);
    assert_eq!(
        p.fold().state,
        TaskState::Taken {
            holders: vec![key(1), key(2)]
        }
    );
    assert_eq!(p.fold_reversed(), p.fold());

    // Mo, in another fragment, took concurrently with Ana, citing only the
    // Create: Mo's post bytes equal Ana's, but the op hash binds the author
    // (DESIGN §2.4), so Mo's Take is its own op. Ravi's Take has Ana's live
    // Take in its causal past, so it is a second claim, not a conflict
    // (DESIGN §4 "Fold"; docs/decisions-assumed.md row 6), even though Ravi
    // never saw Mo: the conflict set is {Ana, Mo} and Ravi stays on the
    // task as "2 people on this" helper.
    let take_m = p.emit(3, TaskKind::Take, &[create]);
    assert_eq!(p.plaintext(1), p.plaintext(3), "byte-identical Take posts");
    assert_ne!(take_m, take_a, "but two ops");
    let f = p.fold();
    let TaskState::Conflict {
        winner,
        losers,
        second,
    } = &f.state
    else {
        panic!("expected a conflict, got {:?}", f.state);
    };
    let conflict_set = [(take_a, key(1)), (take_m, key(3))];
    assert_eq!(
        *winner,
        tiebreak::winner(&p.task_id, &conflict_set).unwrap()
    );
    assert_eq!(losers.len(), 1);
    assert!([key(1), key(3)].contains(&losers[0]));
    assert_eq!(*second, vec![key(2)]);
    assert_eq!(p.fold_reversed(), f);
    for rotate in 0..p.ops.len() {
        let mut ops = p.ops.clone();
        ops.rotate_left(rotate);
        assert_eq!(fold(&p.task_id, &ops), f);
    }

    // Mo releases after the merge → the conflict dissolves, Ana and Ravi remain
    let rel_m = p.emit(3, TaskKind::Release, &[take_r, take_m]);
    assert_eq!(
        p.fold().state,
        TaskState::Taken {
            holders: vec![key(1), key(2)]
        }
    );

    // Done by Ravi (a current claimant) closes it, idempotently
    let done = p.emit(2, TaskKind::Done, &[rel_m]);
    assert_eq!(p.fold().state, TaskState::Done);
    p.emit(1, TaskKind::Done, &[done]);
    assert_eq!(p.fold().state, TaskState::Done);
    assert_eq!(p.fold_reversed().state, TaskState::Done);
    assert!(p.fold().parked.is_empty());
}

#[test]
fn two_authors_creating_byte_identical_tasks_get_different_task_ids() {
    // DESIGN §2.4: the post plaintext carries no author, so Ana's and
    // Ravi's Create posts are byte-identical; the author-bound hash keeps
    // them two tasks with two ids, and either task folds on its own.
    let (_, post) = create_post();
    let ana = Pipeline::created_by(1);
    let ravi = Pipeline::created_by(2);
    assert_eq!(ana.plaintext(0), ravi.plaintext(0));
    assert_eq!(ana.plaintext(0), post);
    assert_ne!(ana.task_id, ravi.task_id);
    assert_ne!(ana.ops[0].0, ravi.ops[0].0);
    assert_eq!(ana.task_id, task_op_hash(&key(1), &post).truncate16());
    assert_eq!(ana.fold().state, TaskState::Open);
    assert_eq!(ravi.fold().state, TaskState::Open);
    // the same author creating the same bytes twice is one task (one op)
    let again = Pipeline::created_by(1);
    assert_eq!(again.task_id, ana.task_id);
    let both: Vec<OpRecord> = [ana.ops.clone(), again.ops.clone()].concat();
    assert_eq!(fold(&ana.task_id, &both), ana.fold());
}

#[test]
fn two_takes_with_identical_post_bytes_are_two_claims_and_conflict() {
    // Ana and Ravi both Take right after the Create: same task_id, same
    // heads, same roster heads → byte-identical post plaintext. The
    // author-bound hash makes them two ops; both claims survive, conflict,
    // and resolve the same way whichever copy a phone received first.
    let mut p = Pipeline::new();
    let create = p.ops[0].0;
    let take_a = p.emit(1, TaskKind::Take, &[create]);
    let take_r = p.emit(2, TaskKind::Take, &[create]);
    assert_eq!(p.plaintext(1), p.plaintext(2), "identical plaintext");
    assert_ne!(take_a, take_r, "distinct author-bound hashes");
    assert_eq!(take_a, task_op_hash(&key(1), &p.plaintext(1)));
    assert_eq!(take_r, task_op_hash(&key(2), &p.plaintext(2)));
    let f = p.fold();
    let expect = tiebreak::winner(&p.task_id, &[(take_a, key(1)), (take_r, key(2))]).unwrap();
    let loser = if expect == key(1) { key(2) } else { key(1) };
    assert_eq!(
        f.state,
        TaskState::Conflict {
            winner: expect,
            losers: vec![loser],
            second: vec![]
        }
    );
    // every arrival order on every phone
    for rotate in 0..p.ops.len() {
        let mut ops = p.ops.clone();
        ops.rotate_left(rotate);
        assert_eq!(fold(&p.task_id, &ops), f);
        ops.reverse();
        assert_eq!(fold(&p.task_id, &ops), f);
    }
    // Mo saw both Takes and took anyway: after both, a second claim
    p.emit(3, TaskKind::Take, &[take_a, take_r]);
    let f = p.fold();
    assert_eq!(
        f.state,
        TaskState::Conflict {
            winner: expect,
            losers: vec![loser],
            second: vec![key(3)]
        }
    );
    assert_eq!(p.fold_reversed(), f);
    // the loser releases (its Release bytes equal any other Release citing
    // Mo's Take, but the author is bound into the hash) → winner and Mo remain
    let last = p.ops.last().unwrap().0;
    let loser_idx = if loser == key(1) { 1 } else { 2 };
    let rel = p.emit(loser_idx, TaskKind::Release, &[last]);
    let other_rel = TaskOp::on_task(p.task_id, TaskKind::Release, vec![last]).unwrap();
    assert_eq!(p.plaintext(p.ops.len() - 1), post_plaintext(&other_rel));
    assert_ne!(rel, task_op_hash(&key(3), &post_plaintext(&other_rel)));
    let f = p.fold();
    assert_eq!(
        f.state,
        TaskState::Taken {
            holders: vec![expect, key(3)]
        }
    );
    assert_eq!(p.fold_reversed(), f);
    assert!(f.parked.is_empty());
}

#[test]
fn three_identical_takes_resolve_the_same_way_in_every_permutation() {
    // Three members Take right after the Create with byte-identical posts:
    // three ops, one conflict set, one winner, in all 24 arrival orders.
    let mut p = Pipeline::new();
    let create = p.ops[0].0;
    let hs: Vec<Hash32> = (1..=3)
        .map(|i| p.emit(i, TaskKind::Take, &[create]))
        .collect();
    assert_eq!(p.plaintext(1), p.plaintext(2));
    assert_eq!(p.plaintext(2), p.plaintext(3));
    let distinct: BTreeSet<Hash32> = hs.iter().copied().collect();
    assert_eq!(distinct.len(), 3);
    let set: Vec<(Hash32, IdentityKey)> = hs
        .iter()
        .enumerate()
        .map(|(i, h)| (*h, key(i + 1)))
        .collect();
    let winner = tiebreak::winner(&p.task_id, &set).unwrap();
    let mut losers: Vec<IdentityKey> = (1..=3).map(key).filter(|k| *k != winner).collect();
    losers.sort_unstable();
    let expect = Fold {
        state: TaskState::Conflict {
            winner,
            losers,
            second: vec![],
        },
        parked: vec![],
    };
    // all permutations of 4 records via Heap's algorithm
    let mut ops = p.ops.clone();
    let n = ops.len();
    let mut c = vec![0usize; n];
    assert_eq!(fold(&p.task_id, &ops), expect);
    let mut i = 0;
    let mut count = 1;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                ops.swap(0, i);
            } else {
                ops.swap(c[i], i);
            }
            assert_eq!(fold(&p.task_id, &ops), expect);
            count += 1;
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    assert_eq!(count, 24);
}

#[test]
fn done_by_a_non_claimant_is_ignored_and_release_by_loser_resolves_conflict() {
    let mut p = Pipeline::new();
    let create = p.ops[0].0;
    // a Done by someone who never took it does nothing
    p.emit(5, TaskKind::Done, &[create]);
    assert_eq!(p.fold().state, TaskState::Open);
    // two concurrent Takes → the loser releases → the winner holds alone
    let a = p.emit(1, TaskKind::Take, &[create]);
    let b = p.emit(2, TaskKind::Take, &[create]);
    let f = p.fold();
    let TaskState::Conflict { winner, losers, .. } = f.state.clone() else {
        panic!("{:?}", f.state);
    };
    let loser = losers[0];
    let loser_idx = if loser == key(1) { 1 } else { 2 };
    p.emit(loser_idx, TaskKind::Release, &[a, b]);
    assert_eq!(
        p.fold().state,
        TaskState::Taken {
            holders: vec![winner]
        }
    );
    // the winner finishes
    let last = p.ops.last().unwrap().0;
    p.emit(if loser_idx == 1 { 2 } else { 1 }, TaskKind::Done, &[last]);
    assert_eq!(p.fold().state, TaskState::Done);
}

#[test]
fn ops_citing_missing_heads_are_parked_until_they_arrive() {
    let mut p = Pipeline::new();
    let create = p.ops[0].0;
    let take = p.emit(1, TaskKind::Take, &[create]);
    let mut without_take = p.clone_ops();
    without_take.retain(|(h, _, _)| *h != take);
    let done = p.emit(1, TaskKind::Done, &[take]);
    let mut partial = without_take;
    partial.push(p.ops.last().unwrap().clone());
    let f = fold(&p.task_id, &partial);
    assert_eq!(f.state, TaskState::Open);
    assert_eq!(f.parked, vec![done]);
    assert_eq!(p.fold().state, TaskState::Done);
}

impl Pipeline {
    fn clone_ops(&self) -> Vec<OpRecord> {
        self.ops.clone()
    }
}

proptest! {
    #![proptest_config(ProptestConfig { cases: proptest_cases().min(1000), ..ProptestConfig::default() })]

    /// Random op DAGs: the fold is independent of input order, every parked
    /// op really cites a missing head, and holders are always distinct members
    /// who issued a Take.
    #[test]
    fn fold_is_order_independent(
        steps in proptest::collection::vec((0usize..5, 1u8..=4, any::<u8>(), any::<u8>()), 0..24),
        shuffle in any::<u64>(),
    ) {
        let mut p = Pipeline::new();
        for (author, kind, pick_a, pick_b) in steps {
            let kind = TaskKind::from_u8(kind).unwrap();
            if kind == TaskKind::Create { continue; }
            let n = p.ops.len();
            let mut heads = vec![p.ops[usize::from(pick_a) % n].0];
            if pick_b % 3 == 0 { heads.push(p.ops[usize::from(pick_b) % n].0); }
            p.emit(author + 1, kind, &heads);
        }
        let f = p.fold();
        let mut ops = p.ops.clone();
        let mut x = shuffle;
        for i in (1..ops.len()).rev() {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let j = usize::try_from(x >> 33).unwrap() % (i + 1);
            ops.swap(i, j);
        }
        prop_assert_eq!(fold(&p.task_id, &ops), f.clone());
        prop_assert!(f.parked.is_empty());
        let holders = f.state.holders();
        let distinct: BTreeSet<IdentityKey> = holders.iter().copied().collect();
        prop_assert_eq!(distinct.len(), holders.len());
        for h in &holders {
            prop_assert!(p.ops.iter().any(|(_, a, o)| a == h && o.op == TaskKind::Take));
        }
    }
}

// ---- decoder fuzzing for all three objects -----------------------------------

proptest! {
    #![proptest_config(ProptestConfig { cases: proptest_cases(), ..ProptestConfig::default() })]

    #[test]
    fn decoders_never_panic_on_random_bytes(bytes in proptest::collection::vec(any::<u8>(), 0..600)) {
        if let Ok(op) = decode::<TaskOp>(&bytes) {
            prop_assert_eq!(encode(&op), bytes.clone());
        }
        if let Ok(o) = decode::<VoteOpen>(&bytes) {
            prop_assert_eq!(encode(&o), bytes.clone());
            prop_assert!(o.verify().is_err());
        }
        if let Ok(b) = decode::<Ballot>(&bytes) {
            prop_assert_eq!(encode(&b), bytes.clone());
            prop_assert!(b.verify().is_err());
        }
    }

    #[test]
    fn mutated_valid_objects_never_panic_and_never_verify(flip in any::<u16>(), byte in any::<u8>()) {
        let (open, valid) = open_vote(12, 1, &["Yes", "No"]);
        let mut o = encode(&open);
        let i = usize::from(flip) % o.len();
        if o[i] == byte { return Ok(()); }
        o[i] = byte;
        if let Ok(back) = decode::<VoteOpen>(&o) {
            prop_assert!(back.verify().is_err() || i >= o.len() - 64 && back.sig != open.sig && back.verify().is_err());
        }
        let ballot = Ballot::sign(signer(1), valid.vote_id(), 0);
        let mut b = encode(&ballot);
        let j = usize::from(flip) % b.len();
        if b[j] != byte {
            b[j] = byte;
            if let Ok(back) = decode::<Ballot>(&b) {
                prop_assert!(back.verify().is_err());
            }
        }
        let task = TaskOp::on_task([1; 16], TaskKind::Take, vec![Hash32::from_bytes([2; 32])]).unwrap();
        let mut t = encode(&task);
        let k = usize::from(flip) % t.len();
        t[k] = byte;
        let _ = decode::<TaskOp>(&t);
    }
}

// ---- opening validity end to end --------------------------------------------

#[test]
fn opening_round_trips_and_validates_with_the_recomputed_t() {
    let (open, valid) = open_vote(13, 2, &["Yes", "No", "Later"]);
    let bytes = encode(&open);
    let back = VoteOpen::from_bytes(&bytes).unwrap();
    assert_eq!(back.vote_id(), valid.vote_id());
    assert_eq!(
        validate_open(&back, &members(13), Branch::B, false, ChainCheck::Ok).unwrap(),
        valid
    );
    // branch B with f = 1 is refused, a stale N is refused, a fork is invalid everywhere
    let (one, _) = open_vote(13, 1, &["Yes", "No"]);
    assert!(validate_open(&one, &members(13), Branch::B, false, ChainCheck::Ok).is_err());
    assert!(validate_open(&back, &members(14), Branch::B, false, ChainCheck::Ok).is_err());
    assert!(validate_open(&back, &members(13), Branch::B, true, ChainCheck::Ok).is_err());
    let t: Tally = tally(&valid, &members(13), &[]);
    assert_eq!(state_label(&t), "Voting open");
}
