//! Roster integration tests (docs/DESIGN.md §10 "roster rules (every row of
//! the §3 table has a positive and a negative test, plus the S3 walkthrough
//! as a fixture)" and the `roster_convergence` property test).

use std::collections::{BTreeMap, BTreeSet};

use meshcore::crypto::backend::GroupId;
use meshcore::crypto::ed25519::{IdentityKey, IdentitySigner};
use meshcore::crypto::hash::Hash32;
use meshcore::crypto::keycard::KeyCard;
use meshcore::objects::roster_op::{MemberEntry, RosterBody, RosterOp, RosterOpError, setting};
use meshcore::objects::{MAX_HEADS, SignedHeader, kind};
use meshcore::roster::labels::{LabelKind, labels};
use meshcore::roster::rotation::{PostVerdict, RotationPolicy, post_verdict};
use meshcore::roster::{Insert, RejectReason, RosterDag, RosterView, VoidReason, evaluate};
use proptest::prelude::*;

// ---- fixture ----------------------------------------------------------------

fn signer(i: u8) -> IdentitySigner {
    let mut seed = [0u8; 32];
    seed[0] = i;
    seed[1] = 0xA5;
    IdentitySigner::from_seed(&seed)
}

fn key(i: u8) -> IdentityKey {
    signer(i).public()
}

fn card(i: u8) -> KeyCard {
    KeyCard::issue(&signer(i), [0x30 ^ i; 32])
}

fn entry(i: u8, in_person: bool) -> MemberEntry {
    MemberEntry {
        keycard: card(i),
        name: format!("p{i}"),
        in_person,
    }
}

fn keys(v: &[u8]) -> BTreeSet<IdentityKey> {
    v.iter().map(|&i| key(i)).collect()
}

/// Builds signed ops with honest per-author sequence numbers, keeps every
/// op produced and a reference DAG holding them all.
struct Fixture {
    gid: GroupId,
    seq: BTreeMap<u8, (u32, Hash32)>,
    dag: RosterDag,
    ops: Vec<RosterOp>,
}

impl Fixture {
    fn new(creator: u8, admins: &[u8], members: &[u8]) -> (Self, Hash32) {
        let s = signer(creator);
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
                nonce: [0x5a; 16],
                name: "Shelter A".into(),
                admins: admins.iter().map(|&a| card(a)).collect(),
                members: members.iter().map(|&m| entry(m, true)).collect(),
                settings: 0,
            },
        )
        .unwrap();
        let gh = g.op_hash();
        let dag = RosterDag::from_genesis(g.clone()).unwrap();
        let mut seq = BTreeMap::new();
        seq.insert(creator, (1, gh));
        (
            Fixture {
                gid: dag.group(),
                seq,
                dag,
                ops: vec![g],
            },
            gh,
        )
    }

    fn next_seq(&self, author: u8) -> (u32, Hash32) {
        self.seq.get(&author).copied().unwrap_or((0, Hash32::ZERO))
    }

    fn header(&self, author: u8, k: u8, heads: Vec<Hash32>, seq: Option<u32>) -> SignedHeader {
        let (next, prev) = self.next_seq(author);
        let (seq, prev) = match seq {
            Some(s) => (s, prev),
            None => (next, prev),
        };
        SignedHeader::new(k, self.gid, key(author), seq, prev, heads).unwrap()
    }

    /// Records and applies a finished op; `honest` advances the author's
    /// chain, an equivocating re-sign does not.
    fn push(&mut self, author: u8, op: RosterOp, honest: bool) -> Hash32 {
        let h = op.op_hash();
        if honest {
            let e = self.seq.entry(author).or_insert((0, Hash32::ZERO));
            e.0 += 1;
            e.1 = h;
        }
        assert!(matches!(self.dag.insert(op.clone()), Insert::Applied(_)));
        self.ops.push(op);
        h
    }

    fn add_op(
        &self,
        author: u8,
        endorser: u8,
        subject: u8,
        heads: Vec<Hash32>,
        (author_ip, endorser_ip): (bool, bool),
        seq: Option<u32>,
    ) -> RosterOp {
        let hdr = self.header(author, kind::ROSTER_ADD, heads, seq);
        let e = RosterOp::endorse(
            &signer(endorser),
            &self.gid,
            &key(subject),
            &format!("p{subject}"),
            endorser_ip,
        )
        .unwrap();
        RosterOp::sign(
            &signer(author),
            hdr,
            RosterBody::Add {
                subject: entry(subject, author_ip),
                endorsement: e,
            },
        )
        .unwrap()
    }

    fn add(&mut self, author: u8, endorser: u8, subject: u8, heads: Vec<Hash32>) -> Hash32 {
        let op = self.add_op(author, endorser, subject, heads, (true, false), None);
        self.push(author, op, true)
    }

    fn add_ip(
        &mut self,
        author: u8,
        endorser: u8,
        subject: u8,
        heads: Vec<Hash32>,
        author_ip: bool,
        endorser_ip: bool,
    ) -> Hash32 {
        let op = self.add_op(
            author,
            endorser,
            subject,
            heads,
            (author_ip, endorser_ip),
            None,
        );
        self.push(author, op, true)
    }

    fn remove_op(&self, author: u8, subject: u8, heads: Vec<Hash32>, seq: Option<u32>) -> RosterOp {
        let hdr = self.header(author, kind::ROSTER_REMOVE, heads, seq);
        RosterOp::sign(
            &signer(author),
            hdr,
            RosterBody::Remove {
                subject: key(subject),
            },
        )
        .unwrap()
    }

    fn remove(&mut self, author: u8, subject: u8, heads: Vec<Hash32>) -> Hash32 {
        let op = self.remove_op(author, subject, heads, None);
        self.push(author, op, true)
    }

    fn admin_set_op(
        &self,
        author: u8,
        cosigners: &[u8],
        admins: &[u8],
        version: u32,
        heads: Vec<Hash32>,
    ) -> RosterOp {
        let hdr = self.header(author, kind::ROSTER_ADMIN_SET, heads, None);
        let mut new: Vec<IdentityKey> = admins.iter().map(|&a| key(a)).collect();
        new.sort_unstable();
        new.dedup();
        let mut cosigs: Vec<(IdentityKey, [u8; 64])> = cosigners
            .iter()
            .map(|&c| RosterOp::cosign(&signer(c), &self.gid, version, &new))
            .collect();
        cosigs.sort_by_key(|c| c.0);
        cosigs.dedup_by(|a, b| a.0 == b.0);
        RosterOp::sign(
            &signer(author),
            hdr,
            RosterBody::AdminSet {
                version,
                admins: new,
                cosigs,
            },
        )
        .unwrap()
    }

    fn admin_set(
        &mut self,
        author: u8,
        cosigners: &[u8],
        admins: &[u8],
        version: u32,
        heads: Vec<Hash32>,
    ) -> Hash32 {
        let op = self.admin_set_op(author, cosigners, admins, version, heads);
        self.push(author, op, true)
    }

    fn setting_op(
        &self,
        author: u8,
        k: u8,
        value: u8,
        heads: Vec<Hash32>,
        seq: Option<u32>,
    ) -> RosterOp {
        let hdr = self.header(author, kind::ROSTER_SETTING, heads, seq);
        RosterOp::sign(&signer(author), hdr, RosterBody::Setting { key: k, value }).unwrap()
    }

    fn setting(&mut self, author: u8, k: u8, value: u8, heads: Vec<Hash32>) -> Hash32 {
        let op = self.setting_op(author, k, value, heads, None);
        self.push(author, op, true)
    }

    fn view(&self) -> RosterView {
        evaluate(&self.dag, &self.dag.heads())
    }

    fn member_keys(&self) -> BTreeSet<IdentityKey> {
        self.view().members.keys().copied().collect()
    }
}

/// Deterministic xorshift for insertion-order shuffles (no clock, no OS RNG).
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = (self.next() % (i as u64 + 1)) as usize;
            v.swap(i, j);
        }
    }
}

/// Inserts `ops` into a fresh DAG in a shuffled order (children often
/// arrive before parents, so parking is exercised) and checks that nothing
/// stays parked.
fn rebuild_shuffled(gid: GroupId, ops: &[RosterOp], rng: &mut Rng) -> RosterDag {
    let mut order: Vec<usize> = (0..ops.len()).collect();
    rng.shuffle(&mut order);
    let mut dag = RosterDag::new(gid);
    for i in order {
        match dag.insert(ops[i].clone()) {
            Insert::Applied(_) | Insert::Parked { .. } => {}
            Insert::Rejected(r) => panic!("op rejected: {r:?}"),
        }
    }
    assert_eq!(dag.parked_len(), 0, "ops left parked");
    assert!(dag.missing().is_empty());
    assert_eq!(dag.len(), ops.len());
    dag
}

// ---- §3 table rows: one positive and one negative test each -----------------

#[test]
fn row_genesis_valid() {
    let (fx, gh) = Fixture::new(0, &[0, 1], &[1, 2]);
    let v = fx.view();
    assert_eq!(v.heads, vec![gh]);
    assert_eq!(v.admins, keys(&[0, 1]));
    assert_eq!(fx.member_keys(), keys(&[0, 1, 2]));
    assert_eq!(v.members[&key(1)].name, "p1");
    assert!(v.members[&key(1)].in_person);
    assert_eq!(v.name, "Shelter A");
    assert_eq!(v.settings.get(&setting::FLAGS), Some(&0));
    assert!(v.voided.is_empty());
    assert!(v.is_resolved());
    assert_eq!(fx.gid, GroupId(gh.truncate16()));
    assert!(fx.dag.genesis().is_some_and(|g| g.op_hash() == gh));
}

#[test]
fn row_genesis_invalid() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1], &[]);
    // a genesis that cites heads cannot even be signed
    let s = signer(0);
    let err = RosterOp::sign(
        &s,
        SignedHeader::new(
            kind::ROSTER_GENESIS,
            GroupId([0; 16]),
            s.public(),
            9,
            Hash32::ZERO,
            vec![gh],
        )
        .unwrap(),
        RosterBody::Genesis {
            nonce: [1; 16],
            name: "x".into(),
            admins: vec![card(0)],
            members: vec![],
            settings: 0,
        },
    )
    .unwrap_err();
    assert!(matches!(err, RosterOpError::Malformed(_)));
    // a genesis without admins, or whose author is not an admin, is malformed
    let hdr = || {
        SignedHeader::new(
            kind::ROSTER_GENESIS,
            GroupId([0; 16]),
            s.public(),
            0,
            Hash32::ZERO,
            vec![],
        )
        .unwrap()
    };
    assert!(matches!(
        RosterOp::sign(
            &s,
            hdr(),
            RosterBody::Genesis {
                nonce: [1; 16],
                name: "x".into(),
                admins: vec![],
                members: vec![],
                settings: 0
            }
        ),
        Err(RosterOpError::Malformed(_))
    ));
    assert!(matches!(
        RosterOp::sign(
            &s,
            hdr(),
            RosterBody::Genesis {
                nonce: [1; 16],
                name: "x".into(),
                admins: vec![card(1)],
                members: vec![],
                settings: 0
            }
        ),
        Err(RosterOpError::Malformed(_))
    ));
    // a second genesis is another group: "exactly one genesis per group"
    let other = RosterOp::sign(
        &s,
        hdr(),
        RosterBody::Genesis {
            nonce: [2; 16],
            name: "other".into(),
            admins: vec![card(0)],
            members: vec![],
            settings: 0,
        },
    )
    .unwrap();
    assert_ne!(other.group_id(), fx.gid);
    assert_eq!(
        fx.dag.insert(other),
        Insert::Rejected(RejectReason::WrongGroup)
    );
    assert_eq!(fx.dag.len(), 1);
}

#[test]
fn row_remove_valid() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1, 2], &[3, 4]);
    let r = fx.remove(1, 3, vec![gh]);
    let v = fx.view();
    assert!(v.is_valid(&r));
    assert_eq!(fx.member_keys(), keys(&[0, 1, 2, 4]));
    assert!(v.removed.contains(&key(3)));
    // monotone: the removal stands although its author is demoted and
    // removed concurrently
    let d = fx.admin_set(0, &[0, 2], &[0, 2], 1, vec![gh]);
    let r1 = fx.remove(0, 1, vec![d]);
    let v = fx.view();
    assert!(v.is_valid(&r) && v.is_valid(&r1) && v.is_valid(&d));
    assert_eq!(fx.member_keys(), keys(&[0, 2, 4]));
    assert_eq!(v.admins, keys(&[0, 2]));
    // ... and a removal made after the author's own demotion is invalid
    let late = fx.remove(1, 4, vec![r1]);
    assert_eq!(
        fx.view().void_reason(&late),
        Some(VoidReason::AuthorNotAdmin)
    );
    assert!(fx.view().is_member(&key(4)));
}

#[test]
fn row_remove_invalid() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1], &[3, 4]);
    let by_member = fx.remove(3, 4, vec![gh]);
    let by_outsider = fx.remove(9, 4, vec![gh]);
    let of_admin = fx.remove(0, 1, vec![gh]);
    let of_self = fx.remove(0, 0, vec![gh]);
    let v = fx.view();
    assert_eq!(v.void_reason(&by_member), Some(VoidReason::AuthorNotAdmin));
    assert_eq!(
        v.void_reason(&by_outsider),
        Some(VoidReason::AuthorNotAdmin)
    );
    assert_eq!(v.void_reason(&of_admin), Some(VoidReason::SubjectIsAdmin));
    assert_eq!(v.void_reason(&of_self), Some(VoidReason::SubjectIsAdmin));
    assert_eq!(fx.member_keys(), keys(&[0, 1, 3, 4]));
    assert_eq!(v.admins, keys(&[0, 1]));
    assert!(v.removed.is_empty());
}

#[test]
fn row_add_valid() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1, 2], &[3]);
    let a = fx.add_ip(0, 1, 5, vec![gh], false, true);
    let v = fx.view();
    assert!(v.is_valid(&a));
    assert!(v.members[&key(5)].in_person);
    assert_eq!(v.members[&key(5)].name, "p5");
    assert_eq!(v.members[&key(5)].keycard, card(5));
    // re-add causally after the removal
    let r = fx.remove(2, 5, vec![a]);
    assert!(!fx.view().is_member(&key(5)));
    let a2 = fx.add(1, 2, 5, vec![r]);
    let v = fx.view();
    assert!(v.is_valid(&a2));
    assert!(v.is_member(&key(5)));
    assert!(v.voided.is_empty());
    // an Add made before an unrelated later removal of its author stands
    let a3 = fx.add(2, 0, 6, vec![a2]);
    let d = fx.admin_set(0, &[0, 1], &[0, 1], 1, vec![a3]);
    let _ = fx.remove(0, 2, vec![d]);
    let v = fx.view();
    assert!(v.is_valid(&a3) && v.is_member(&key(6)) && !v.is_member(&key(2)));
}

#[test]
fn row_add_invalid() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1, 2], &[3]);
    let author_not_admin = fx.add(3, 0, 5, vec![gh]);
    let endorser_not_admin = fx.add(0, 3, 5, vec![gh]);
    let endorser_is_author = fx.add(0, 0, 5, vec![gh]);
    let no_in_person = fx.add_ip(0, 1, 5, vec![gh], false, false);
    let subject_is_admin = fx.add(0, 1, 2, vec![gh]);
    let v = fx.view();
    assert_eq!(
        v.void_reason(&author_not_admin),
        Some(VoidReason::AuthorNotAdmin)
    );
    assert_eq!(
        v.void_reason(&endorser_not_admin),
        Some(VoidReason::EndorserNotAdmin)
    );
    assert_eq!(
        v.void_reason(&endorser_is_author),
        Some(VoidReason::EndorserIsAuthor)
    );
    assert_eq!(
        v.void_reason(&no_in_person),
        Some(VoidReason::NoInPersonVerification)
    );
    assert_eq!(
        v.void_reason(&subject_is_admin),
        Some(VoidReason::SubjectIsAdmin)
    );
    assert!(!v.is_member(&key(5)));
    // an endorsement whose flag was changed after the endorser signed does
    // not verify at all (the author signed the tampered body, so only the
    // endorsement check catches it)
    let hdr = fx.header(0, kind::ROSTER_ADD, vec![gh], None);
    let mut e = RosterOp::endorse(&signer(1), &fx.gid, &key(5), "p5", true).unwrap();
    e.in_person = false;
    let bad = RosterOp::sign(
        &signer(0),
        hdr,
        RosterBody::Add {
            subject: entry(5, false),
            endorsement: e,
        },
    )
    .unwrap();
    assert!(matches!(bad.verify(), Err(RosterOpError::BadEndorsement)));
    assert!(matches!(
        fx.dag.insert(bad),
        Insert::Rejected(RejectReason::Invalid(RosterOpError::BadEndorsement))
    ));
    // remove-wins: concurrent Add loses
    let r = fx.remove(2, 3, vec![gh]);
    let concurrent = fx.add(0, 1, 3, vec![gh]);
    let v = fx.view();
    assert!(v.is_valid(&r));
    assert_eq!(v.void_reason(&concurrent), Some(VoidReason::RemoveWins));
    assert!(!v.is_member(&key(3)));
    assert!(v.changed_after_reconnect.contains(&key(3)));
    // a re-add that does not cite the removal is void too
    let not_after = fx.add(1, 2, 3, vec![concurrent]);
    assert_eq!(
        fx.view().void_reason(&not_after),
        Some(VoidReason::RemoveWins)
    );
    // "Changed after reconnect": author demoted / removed concurrently
    let d = fx.admin_set(0, &[0, 1], &[0, 1], 1, vec![r]);
    let r2 = fx.remove(0, 2, vec![d]);
    let by_demoted = fx.add(2, 1, 6, vec![gh]);
    let endorsed_by_demoted = fx.add(1, 2, 7, vec![gh]);
    let v = fx.view();
    assert!(v.is_valid(&r2));
    assert!(matches!(
        v.void_reason(&by_demoted),
        Some(VoidReason::AuthorRemovedConcurrently | VoidReason::AuthorDemotedConcurrently)
    ));
    assert!(matches!(
        v.void_reason(&endorsed_by_demoted),
        Some(VoidReason::EndorserRemovedConcurrently | VoidReason::EndorserDemotedConcurrently)
    ));
    assert!(!v.is_member(&key(6)) && !v.is_member(&key(7)));
    assert!(v.changed_after_reconnect.contains(&key(6)));
}

#[test]
fn row_adminset_valid() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1, 2], &[3, 4]);
    let v1 = fx.admin_set(0, &[0, 1], &[0, 1, 3], 1, vec![gh]);
    let v = fx.view();
    assert!(v.is_valid(&v1));
    assert_eq!(v.admins, keys(&[0, 1, 3]));
    assert_eq!(v.admin_version, 1);
    assert_eq!(v.admins_at(&v1), Some(&keys(&[0, 1, 2])));
    // 2 of 3 again for version 2; a demoted admin can then be removed
    let v2 = fx.admin_set(3, &[3, 1], &[1, 3, 4], 2, vec![v1]);
    let r = fx.remove(3, 0, vec![v2]);
    let v = fx.view();
    assert!(v.is_valid(&v2) && v.is_valid(&r));
    assert_eq!(v.admins, keys(&[1, 3, 4]));
    assert_eq!(v.admin_version, 2);
    assert_eq!(fx.member_keys(), keys(&[1, 2, 3, 4]));
    assert_eq!(v.admins_at(&r), Some(&keys(&[1, 3, 4])));
}

#[test]
fn row_adminset_invalid() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1, 2, 5], &[3, 4]);
    let half = fx.admin_set(0, &[0, 1], &[0, 1], 1, vec![gh]); // 2 of 4 is not strict
    let version = fx.admin_set(0, &[0, 1, 2], &[0, 1], 2, vec![gh]);
    let outsider = fx.admin_set(0, &[0, 1, 3], &[0, 1], 1, vec![gh]);
    let no_author = fx.admin_set(0, &[1, 2, 5], &[0, 1], 1, vec![gh]);
    let stranger = fx.admin_set(0, &[0, 1, 2], &[0, 1, 9], 1, vec![gh]);
    let v = fx.view();
    assert_eq!(v.void_reason(&half), Some(VoidReason::NoMajority));
    assert_eq!(
        v.void_reason(&version),
        Some(VoidReason::AdminSetVersion { expected: 1 })
    );
    assert_eq!(v.void_reason(&outsider), Some(VoidReason::CosignerNotAdmin));
    assert_eq!(
        v.void_reason(&no_author),
        Some(VoidReason::AuthorNotCosigner)
    );
    assert_eq!(v.void_reason(&stranger), Some(VoidReason::AdminNotMember));
    assert_eq!(v.admins, keys(&[0, 1, 2, 5]));
    assert_eq!(v.admin_version, 0);
    // equivocation: 1 cosigns two different version-1 sets → both void
    let x = fx.admin_set(0, &[0, 1, 2], &[0, 1, 2], 1, vec![gh]);
    let y = fx.admin_set(5, &[5, 1, 2], &[1, 2, 5], 1, vec![gh]);
    let v = fx.view();
    assert_eq!(v.void_reason(&x), Some(VoidReason::AdminSetEquivocation));
    assert_eq!(v.void_reason(&y), Some(VoidReason::AdminSetEquivocation));
    assert_eq!(v.admins, keys(&[0, 1, 2, 5]));
    // a version-2 op built on the voided x is now at the wrong version
    let z = fx.admin_set(0, &[0, 1], &[0, 1], 2, vec![x]);
    assert_eq!(
        fx.view().void_reason(&z),
        Some(VoidReason::AdminSetVersion { expected: 1 })
    );
}

#[test]
fn row_setting_valid() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1], &[3]);
    let s1 = fx.setting(0, setting::FLAGS, setting::FLAG_STRAW_MODE, vec![gh]);
    let v = fx.view();
    assert!(v.is_valid(&s1));
    assert_eq!(v.flags(), setting::FLAG_STRAW_MODE);
    let s2 = fx.setting(1, setting::FLAGS, setting::FLAG_MEDIA_OFF, vec![s1]);
    assert!(fx.view().is_valid(&s2));
    assert_eq!(fx.view().flags(), setting::FLAG_MEDIA_OFF);
    // concurrent writers: the higher op hash wins, whatever the order
    let a = fx.setting(0, 7, 1, vec![s2]);
    let b = fx.setting(1, 7, 2, vec![s2]);
    let expect = if a > b { 1 } else { 2 };
    assert_eq!(fx.view().settings.get(&7), Some(&expect));
    let mut rng = Rng(11);
    for _ in 0..5 {
        let dag = rebuild_shuffled(fx.gid, &fx.ops, &mut rng);
        assert_eq!(evaluate(&dag, &dag.heads()).settings.get(&7), Some(&expect));
    }
}

#[test]
fn row_setting_invalid() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1], &[3]);
    let by_member = fx.setting(3, setting::FLAGS, 0xff, vec![gh]);
    let by_outsider = fx.setting(8, setting::FLAGS, 0xff, vec![gh]);
    let v = fx.view();
    assert_eq!(v.void_reason(&by_member), Some(VoidReason::AuthorNotAdmin));
    assert_eq!(
        v.void_reason(&by_outsider),
        Some(VoidReason::AuthorNotAdmin)
    );
    assert_eq!(v.flags(), 0);
    // a demoted admin's later setting is invalid
    let d = fx.admin_set(0, &[0, 1], &[0], 1, vec![gh]);
    let late = fx.setting(1, setting::FLAGS, 0x7f, vec![d]);
    assert_eq!(
        fx.view().void_reason(&late),
        Some(VoidReason::AuthorNotAdmin)
    );
}

#[test]
fn row_any_equivocation_valid() {
    // an honest chain: distinct sequence numbers, every op applied
    let (mut fx, gh) = Fixture::new(0, &[0, 1], &[]);
    let a = fx.add(0, 1, 5, vec![gh]);
    let b = fx.add(0, 1, 6, vec![a]);
    let c = fx.setting(0, setting::FLAGS, 1, vec![b]);
    let v = fx.view();
    assert!(v.is_valid(&a) && v.is_valid(&b) && v.is_valid(&c));
    assert_eq!(fx.member_keys(), keys(&[0, 1, 5, 6]));
    // the same op twice is not an equivocation
    assert_eq!(fx.dag.insert(fx.ops[1].clone()), Insert::Applied(a));
    assert_eq!(fx.view(), v);
}

#[test]
fn row_any_equivocation_invalid() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1], &[3]);
    let a = fx.add(0, 1, 5, vec![gh]);
    let (next, _) = fx.next_seq(0);
    // re-sign the Add's sequence number with different content
    let twin = fx.add_op(0, 1, 6, vec![gh], (true, false), Some(next - 1));
    let th = fx.push(0, twin, false);
    let v = fx.view();
    assert_eq!(v.void_reason(&a), Some(VoidReason::Equivocation));
    assert_eq!(v.void_reason(&th), Some(VoidReason::Equivocation));
    assert_eq!(fx.member_keys(), keys(&[0, 1, 3]));
    // settings and admin sets equivocated the same way are ignored too
    let s = fx.setting(1, setting::FLAGS, 1, vec![gh]);
    let (next1, _) = fx.next_seq(1);
    let s_twin = fx.setting_op(1, setting::FLAGS, 2, vec![gh], Some(next1 - 1));
    let sth = fx.push(1, s_twin, false);
    let v = fx.view();
    assert_eq!(v.void_reason(&s), Some(VoidReason::Equivocation));
    assert_eq!(v.void_reason(&sth), Some(VoidReason::Equivocation));
    assert_eq!(v.flags(), 0);
    // removals are monotone: an equivocated Remove still stands
    let r = fx.remove(0, 3, vec![gh]);
    let (next0, _) = fx.next_seq(0);
    let r_twin = fx.setting_op(0, 9, 9, vec![gh], Some(next0 - 1));
    let rth = fx.push(0, r_twin, false);
    let v = fx.view();
    assert!(v.is_valid(&r));
    assert_eq!(v.void_reason(&rth), Some(VoidReason::Equivocation));
    assert!(!v.is_member(&key(3)));
}

// ---- parking ----------------------------------------------------------------

#[test]
fn parking_child_before_parent() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1], &[]);
    let a = fx.add(0, 1, 5, vec![gh]);
    let b = fx.add(1, 0, 6, vec![a]);
    let c = fx.remove(0, 5, vec![b]);
    let reference = fx.view();
    let mut dag = RosterDag::new(fx.gid);
    assert_eq!(
        dag.insert(fx.ops[3].clone()),
        Insert::Parked { missing: vec![b] }
    );
    assert_eq!(
        dag.insert(fx.ops[2].clone()),
        Insert::Parked { missing: vec![a] }
    );
    assert_eq!(dag.parked_len(), 2);
    assert_eq!(dag.missing(), vec![a]);
    assert!(dag.heads().is_empty());
    assert!(evaluate(&dag, &[c]).members.is_empty());
    assert_eq!(
        dag.insert(fx.ops[1].clone()),
        Insert::Parked { missing: vec![gh] }
    );
    assert_eq!(dag.missing(), vec![gh]);
    assert_eq!(dag.insert(fx.ops[0].clone()), Insert::Applied(gh));
    assert_eq!(dag.parked_len(), 0);
    assert_eq!(dag.heads(), vec![c]);
    assert_eq!(dag.drain_applied(), vec![gh, a, b, c]);
    assert_eq!(evaluate(&dag, &dag.heads()), reference);
    assert_eq!(
        reference.members.keys().copied().collect::<BTreeSet<_>>(),
        keys(&[0, 1, 6])
    );
}

// ---- S3 walkthrough -----------------------------------------------------------

/// Scenario S3 (DESIGN §10): 40 members, admins {0,1,2,18,19,25}. Mo = 25 is
/// demoted (4 of 6 cosign) and removed by 0 in fragment A while, in fragment
/// B, he signs Adds of three sock-puppet keys endorsed by 19.
struct S3 {
    fx: Fixture,
    gh: Hash32,
    frag_a: Vec<Hash32>,
    frag_b: Vec<Hash32>,
    puppets: Vec<Hash32>,
    remove_mo: Hash32,
    honest_add: Hash32,
}

const MO: u8 = 25;

fn s3() -> S3 {
    let members: Vec<u8> = (0..40).collect();
    let (mut fx, gh) = Fixture::new(0, &[0, 1, 2, 18, 19, MO], &members);
    // fragment A: demote Mo, remove him, add an honest newcomer
    let demote = fx.admin_set(0, &[0, 1, 2, 18], &[0, 1, 2, 18, 19], 1, vec![gh]);
    let remove_mo = fx.remove(0, MO, vec![demote]);
    let honest_add = fx.add(1, 2, 43, vec![remove_mo]);
    let s = fx.setting(2, setting::FLAGS, setting::FLAG_MEDIA_OFF, vec![honest_add]);
    // fragment B: Mo adds three sock puppets, endorsed by 19
    let p0 = fx.add_ip(MO, 19, 40, vec![gh], true, false);
    let p1 = fx.add_ip(MO, 19, 41, vec![p0], true, false);
    let p2 = fx.add_ip(MO, 19, 42, vec![p1], true, false);
    S3 {
        fx,
        gh,
        frag_a: vec![s],
        frag_b: vec![p2],
        puppets: vec![p0, p1, p2],
        remove_mo,
        honest_add,
    }
}

#[test]
fn s3_walkthrough_voids_sock_puppets_after_merge() {
    let s = s3();
    let expect_members: BTreeSet<IdentityKey> = (0..40u8)
        .filter(|&i| i != MO)
        .chain([43])
        .map(key)
        .collect();

    // before the merge, fragment B believes the puppets are in
    let before = evaluate(&s.fx.dag, &s.frag_b);
    for p in &s.puppets {
        assert!(before.is_valid(p));
    }
    assert!(before.is_member(&key(40)) && before.is_member(&key(MO)));
    assert_eq!(before.admins.len(), 6);

    // fragment A has Mo out already
    let a = evaluate(&s.fx.dag, &s.frag_a);
    assert!(!a.is_member(&key(MO)) && a.is_member(&key(43)));
    assert_eq!(a.admins, keys(&[0, 1, 2, 18, 19]));

    // after the merge
    let heads: Vec<Hash32> = s.frag_a.iter().chain(s.frag_b.iter()).copied().collect();
    assert_eq!(s.fx.dag.heads(), {
        let mut h = heads.clone();
        h.sort_unstable();
        h
    });
    let after = evaluate(&s.fx.dag, &heads);
    assert_eq!(
        after.members.keys().copied().collect::<BTreeSet<_>>(),
        expect_members
    );
    assert_eq!(after.admins, keys(&[0, 1, 2, 18, 19]));
    for p in &s.puppets {
        assert!(matches!(
            after.void_reason(p),
            Some(VoidReason::AuthorRemovedConcurrently | VoidReason::AuthorDemotedConcurrently)
        ));
    }
    assert_eq!(after.voided.len(), 3);
    assert!(after.removed.contains(&key(MO)));
    assert_eq!(after.changed_after_reconnect, keys(&[40, 41, 42]));
    assert_eq!(after.flags(), setting::FLAG_MEDIA_OFF);
    assert!(after.is_valid(&s.remove_mo) && after.is_valid(&s.honest_add));

    // labels: "Changed after reconnect" once per puppet, Mo removed, 43 added
    let ls = labels(&before, &after);
    let kinds: BTreeMap<IdentityKey, LabelKind> = ls.iter().map(|l| (l.key, l.kind)).collect();
    assert_eq!(kinds[&key(40)], LabelKind::ChangedAfterReconnect);
    assert_eq!(kinds[&key(41)], LabelKind::ChangedAfterReconnect);
    assert_eq!(kinds[&key(42)], LabelKind::ChangedAfterReconnect);
    assert_eq!(kinds[&key(MO)], LabelKind::RemovedWaiting);
    assert_eq!(kinds[&key(43)], LabelKind::AddedProvisional);
    assert_eq!(ls.len(), 5);
    let mo = ls.iter().find(|l| l.key == key(MO)).unwrap();
    assert_eq!(mo.text, "p25 removed · Waiting for people out of range");
    // the label is stable: the same merge again yields nothing new
    assert!(labels(&after, &after).is_empty());

    // members(H) is identical for every insertion order, with parking
    let mut rng = Rng(0x5eed_5eed);
    for _ in 0..50 {
        let dag = rebuild_shuffled(s.fx.gid, &s.fx.ops, &mut rng);
        assert_eq!(dag.heads(), s.fx.dag.heads());
        let v = evaluate(&dag, &dag.heads());
        assert_eq!(v, after);
        assert_eq!(evaluate(&dag, &s.frag_b), before);
    }
}

#[test]
fn s3_posts_by_mo_are_rejected_or_labelled_and_rotation_is_triggered() {
    let s = s3();
    let current = s.fx.view();
    let mo = key(MO);
    // (c) a post whose heads include his removal: hidden everywhere
    assert_eq!(
        post_verdict(&s.fx.dag, &mo, &[s.remove_mo], &current),
        PostVerdict::RejectRemovedInHeads
    );
    assert_eq!(
        post_verdict(&s.fx.dag, &mo, &s.fx.dag.heads(), &current),
        PostVerdict::RejectRemovedInHeads
    );
    // split-time posts (heads before the removal): kept, "Sent by Mo, who was removed"
    assert_eq!(
        post_verdict(&s.fx.dag, &mo, &[s.gh], &current),
        PostVerdict::KeepLabelledRemoved
    );
    assert_eq!(
        post_verdict(&s.fx.dag, &mo, &s.frag_b, &current),
        PostVerdict::KeepLabelledRemoved
    );
    assert_eq!(
        meshcore::roster::labels::sent_by_removed(&current.members.get(&key(1)).unwrap().name),
        "Sent by p1, who was removed"
    );
    // honest members everywhere: accepted
    assert_eq!(
        post_verdict(&s.fx.dag, &key(19), &s.frag_b, &current),
        PostVerdict::Accept
    );
    assert_eq!(
        post_verdict(&s.fx.dag, &key(43), &s.frag_a, &current),
        PostVerdict::Accept
    );
    // a puppet is never a member at the merged heads: labelled, not accepted
    assert_eq!(
        post_verdict(&s.fx.dag, &key(40), &s.frag_b, &current),
        PostVerdict::KeepLabelledRemoved
    );
    // the removal reaching a phone sets the rotation flag
    let mut dag = RosterDag::new(s.fx.gid);
    let mut policy = RotationPolicy::build_a(0);
    for op in &s.fx.ops {
        let _ = dag.insert(op.clone());
        let applied = dag.drain_applied();
        let view = evaluate(&dag, &dag.heads());
        let saw = policy.observe_applied(&dag, &applied, &view);
        assert_eq!(saw, applied.contains(&s.remove_mo));
    }
    assert!(policy.is_pending() && policy.due(0));
    policy.on_rotated(5);
    assert!(!policy.due(5));
}

// ---- roster_convergence property test ------------------------------------------

/// Keys 0..N_KEYS; genesis admins {0,1,2}, members {3,4}; 5,6,7 outsiders.
const N_KEYS: u8 = 8;
const N_FRAGS: usize = 3;

#[derive(Debug, Clone)]
enum Instr {
    Add {
        frag: u8,
        author: u8,
        endorser: u8,
        subject: u8,
        in_person: u8,
        stale: bool,
    },
    Remove {
        frag: u8,
        author: u8,
        subject: u8,
        stale: bool,
    },
    AdminSet {
        frag: u8,
        author: u8,
        cosign_mask: u8,
        admin_mask: u8,
        bad_version: bool,
        stale: bool,
    },
    Setting {
        frag: u8,
        author: u8,
        value: u8,
    },
    /// The author re-signs its last sequence number with other content.
    Equivocate {
        frag: u8,
        author: u8,
        value: u8,
    },
    Merge {
        a: u8,
        b: u8,
    },
}

fn instr() -> impl Strategy<Value = Instr> {
    let k = 0..N_KEYS;
    let f = 0..N_FRAGS as u8;
    prop_oneof![
        4 => (f.clone(), k.clone(), k.clone(), k.clone(), 0..4u8, any::<bool>()).prop_map(
            |(frag, author, endorser, subject, in_person, stale)| Instr::Add {
                frag,
                author,
                endorser,
                subject,
                in_person,
                stale
            }
        ),
        3 => (f.clone(), k.clone(), k.clone(), any::<bool>()).prop_map(|(frag, author, subject, stale)| {
            Instr::Remove {
                frag,
                author,
                subject,
                stale,
            }
        }),
        2 => (f.clone(), k.clone(), any::<u8>(), 1..=255u8, prop::bool::weighted(0.2), any::<bool>()).prop_map(
            |(frag, author, cosign_mask, admin_mask, bad_version, stale)| Instr::AdminSet {
                frag,
                author,
                cosign_mask,
                admin_mask,
                bad_version,
                stale
            }
        ),
        1 => (f.clone(), k.clone(), any::<u8>()).prop_map(|(frag, author, value)| Instr::Setting {
            frag,
            author,
            value
        }),
        1 => (f.clone(), k.clone(), any::<u8>()).prop_map(|(frag, author, value)| Instr::Equivocate {
            frag,
            author,
            value
        }),
        2 => (f.clone(), f).prop_map(|(a, b)| Instr::Merge { a, b }),
    ]
}

fn mask_keys(mask: u8) -> Vec<u8> {
    (0..N_KEYS).filter(|i| mask & (1 << i) != 0).collect()
}

/// Interprets an instruction stream into signed ops over simulated
/// fragments (each with its own head set).
struct Sim {
    fx: Fixture,
    heads: Vec<Vec<Hash32>>,
    prev_heads: Vec<Vec<Hash32>>,
}

impl Sim {
    fn new() -> Self {
        let (fx, gh) = Fixture::new(0, &[0, 1, 2], &[3, 4]);
        Sim {
            fx,
            heads: vec![vec![gh]; N_FRAGS],
            prev_heads: vec![vec![gh]; N_FRAGS],
        }
    }

    fn cite(&self, frag: usize, stale: bool) -> Vec<Hash32> {
        let mut h = if stale {
            self.prev_heads[frag].clone()
        } else {
            self.heads[frag].clone()
        };
        h.sort_unstable();
        h.dedup();
        h.truncate(MAX_HEADS);
        h
    }

    fn advance(&mut self, frag: usize, cited: &[Hash32], new: Hash32) {
        self.prev_heads[frag] = self.heads[frag].clone();
        let mut h: Vec<Hash32> = self.heads[frag]
            .iter()
            .copied()
            .filter(|x| !cited.contains(x))
            .collect();
        h.push(new);
        h.sort_unstable();
        h.dedup();
        self.heads[frag] = h;
    }

    fn run(&mut self, program: &[Instr]) {
        for ins in program {
            match *ins {
                Instr::Add {
                    frag,
                    author,
                    endorser,
                    subject,
                    in_person,
                    stale,
                } => {
                    let f = frag as usize;
                    let cited = self.cite(f, stale);
                    let op = self.fx.add_op(
                        author,
                        endorser,
                        subject,
                        cited.clone(),
                        (in_person & 1 != 0, in_person & 2 != 0),
                        None,
                    );
                    let h = self.fx.push(author, op, true);
                    self.advance(f, &cited, h);
                }
                Instr::Remove {
                    frag,
                    author,
                    subject,
                    stale,
                } => {
                    let f = frag as usize;
                    let cited = self.cite(f, stale);
                    let op = self.fx.remove_op(author, subject, cited.clone(), None);
                    let h = self.fx.push(author, op, true);
                    self.advance(f, &cited, h);
                }
                Instr::AdminSet {
                    frag,
                    author,
                    cosign_mask,
                    admin_mask,
                    bad_version,
                    stale,
                } => {
                    let f = frag as usize;
                    let cited = self.cite(f, stale);
                    let mut cosigners = mask_keys(cosign_mask);
                    if !cosigners.contains(&author) {
                        cosigners.push(author);
                    }
                    let admins = mask_keys(admin_mask);
                    let base = evaluate(&self.fx.dag, &cited).admin_version;
                    let version = if bad_version { base + 2 } else { base + 1 };
                    let op =
                        self.fx
                            .admin_set_op(author, &cosigners, &admins, version, cited.clone());
                    let h = self.fx.push(author, op, true);
                    self.advance(f, &cited, h);
                }
                Instr::Setting {
                    frag,
                    author,
                    value,
                } => {
                    let f = frag as usize;
                    let cited = self.cite(f, false);
                    let op = self
                        .fx
                        .setting_op(author, setting::FLAGS, value, cited.clone(), None);
                    let h = self.fx.push(author, op, true);
                    self.advance(f, &cited, h);
                }
                Instr::Equivocate {
                    frag,
                    author,
                    value,
                } => {
                    let f = frag as usize;
                    let (next, _) = self.fx.next_seq(author);
                    if next == 0 {
                        continue;
                    }
                    let cited = self.cite(f, false);
                    let op = self
                        .fx
                        .setting_op(author, 0xee, value, cited.clone(), Some(next - 1));
                    if self.fx.dag.contains(&op.op_hash()) {
                        continue;
                    }
                    let h = self.fx.push(author, op, false);
                    self.advance(f, &cited, h);
                }
                Instr::Merge { a, b } => {
                    let (a, b) = (a as usize, b as usize);
                    let mut h = self.heads[a].clone();
                    h.extend(self.heads[b].iter().copied());
                    h.sort_unstable();
                    h.dedup();
                    h.truncate(MAX_HEADS);
                    self.prev_heads[a] = self.heads[a].clone();
                    self.prev_heads[b] = self.heads[b].clone();
                    self.heads[a] = h.clone();
                    self.heads[b] = h;
                }
            }
        }
    }
}

fn body(op: &RosterOp) -> &RosterBody {
    &op.body
}

/// The §10 invariants, checked from the op set with the DAG's own closure.
fn check_invariants(dag: &RosterDag, view: &RosterView, has_equivocation: bool) {
    let valid_removes: Vec<(&RosterOp, Hash32)> = dag
        .ops()
        .filter(|(h, op)| matches!(body(op), RosterBody::Remove { .. }) && view.is_valid(h))
        .map(|(h, op)| (op, *h))
        .collect();
    let valid_adds: Vec<(&RosterOp, Hash32)> = dag
        .ops()
        .filter(|(h, op)| matches!(body(op), RosterBody::Add { .. }) && view.is_valid(h))
        .map(|(h, op)| (op, *h))
        .collect();
    let in_past = |op: &RosterOp, h: &Hash32| dag.closure(&op.header.heads).contains(h);

    // Every op in S is either applied or voided; nothing outside S is listed.
    for (h, _) in &view.voided {
        assert!(view.ops.contains(h));
    }

    // Removal monotone / re-add only causally after the removal: a member
    // with a valid Remove must have a valid Add that cites it causally.
    for m in view.members.keys() {
        for (_, rh) in valid_removes
            .iter()
            .filter(|(r, _)| matches!(body(r), RosterBody::Remove { subject } if subject == m))
        {
            let readded = valid_adds.iter().any(|(a, _)| {
                matches!(body(a), RosterBody::Add { subject, .. } if subject.key() == *m)
                    && in_past(a, rh)
            });
            assert!(readded, "member {m} has an un-superseded valid removal");
        }
    }
    // Conversely, a key whose every valid Add is voided or superseded by a
    // valid Remove is not a member.
    for (r, rh) in &valid_removes {
        let RosterBody::Remove { subject } = body(r) else {
            unreachable!()
        };
        let any_add_after = valid_adds.iter().any(|(a, _)| {
            matches!(body(a), RosterBody::Add { subject: s, .. } if s.key() == *subject)
                && in_past(a, rh)
        });
        if !any_add_after {
            assert!(!view.is_member(subject), "removed {subject} still a member");
        }
    }

    // Concurrent add by a removed/demoted admin is void.
    for (h, op) in dag.ops() {
        let RosterBody::Add { endorsement, .. } = body(op) else {
            continue;
        };
        if !view.ops.contains(h) {
            continue;
        }
        for who in [op.header.author, endorsement.admin] {
            let removed_concurrently = valid_removes.iter().any(|(r, rh)| {
                matches!(body(r), RosterBody::Remove { subject } if *subject == who)
                    && !in_past(op, rh)
                    && !in_past(r, h)
            });
            let demoted_concurrently = dag.ops().any(|(yh, y)| {
                let RosterBody::AdminSet { admins, .. } = body(y) else {
                    return false;
                };
                view.is_valid(yh)
                    && !in_past(op, yh)
                    && !in_past(y, h)
                    && view.admins_at(yh).is_some_and(|b| b.contains(&who))
                    && !admins.contains(&who)
            });
            if removed_concurrently || demoted_concurrently {
                assert!(
                    view.void_reason(h).is_some(),
                    "concurrent add {h} by removed/demoted {who} not void"
                );
            }
        }
    }

    // Admin set never changes without a strict majority: the applied
    // AdminSets form a chain 1..k, each cosigned by a strict majority of the
    // previous set, and the view's admins are the last one's.
    let mut chain: Vec<(u32, &RosterOp, Hash32)> = dag
        .ops()
        .filter_map(|(h, op)| match body(op) {
            RosterBody::AdminSet { version, .. } if view.is_valid(h) => Some((*version, op, *h)),
            _ => None,
        })
        .collect();
    chain.sort_by_key(|(v, _, _)| *v);
    let genesis_admins = keys(&[0, 1, 2]);
    let mut prev: BTreeSet<IdentityKey> = genesis_admins.clone();
    for (i, (v, op, h)) in chain.iter().enumerate() {
        assert_eq!(*v as usize, i + 1, "applied AdminSet versions are not 1..k");
        let RosterBody::AdminSet { admins, cosigs, .. } = body(op) else {
            unreachable!()
        };
        let cosigners: BTreeSet<IdentityKey> = cosigs.iter().map(|(k, _)| *k).collect();
        assert!(
            cosigners.is_subset(&prev),
            "cosigners outside the previous admin set"
        );
        assert!(cosigners.len() * 2 > prev.len(), "no strict majority");
        assert!(cosigners.contains(&op.header.author));
        assert_eq!(view.admins_at(h), Some(&prev));
        if !has_equivocation {
            // without equivocation the memo equals the view at the op's own
            // heads, and every named admin is a member of that past
            let own = evaluate(dag, &op.header.heads);
            assert_eq!(own.admins, prev);
            for a in admins {
                assert!(
                    own.is_member(a),
                    "named admin {a} not a member of the op's past"
                );
            }
        }
        prev = admins.iter().copied().collect();
    }
    assert_eq!(view.admins, prev);
    assert_eq!(view.admin_version as usize, chain.len());
    // every member has a KeyCard bound to its key
    for (k, info) in &view.members {
        assert_eq!(info.keycard.ed, *k);
        assert!(info.keycard.verify().is_ok());
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: std::env::var("PROPTEST_CASES").ok().and_then(|v| v.parse().ok()).unwrap_or(256),
        ..ProptestConfig::default()
    })]

    #[test]
    fn roster_convergence(program in prop::collection::vec(instr(), 1..28), seed in any::<u64>()) {
        let mut sim = Sim::new();
        sim.run(&program);
        let has_equivocation = program.iter().any(|i| matches!(i, Instr::Equivocate { .. }));
        let gid = sim.fx.gid;
        let ops = sim.fx.ops.clone();
        let reference = sim.fx.view();
        let ref_heads = sim.fx.dag.heads();
        let frag_views: Vec<RosterView> = sim.heads.iter().map(|h| evaluate(&sim.fx.dag, h)).collect();
        check_invariants(&sim.fx.dag, &reference, has_equivocation);
        for fv in &frag_views {
            check_invariants(&sim.fx.dag, fv, has_equivocation);
        }

        let mut rng = Rng(seed | 1);
        for _ in 0..20 {
            let dag = rebuild_shuffled(gid, &ops, &mut rng);
            prop_assert_eq!(dag.heads(), ref_heads.clone());
            let v = evaluate(&dag, &dag.heads());
            prop_assert_eq!(&v, &reference);
            for (h, fv) in sim.heads.iter().zip(frag_views.iter()) {
                prop_assert_eq!(&evaluate(&dag, h), fv);
            }
        }
        // the cache key is a function of the head set only
        prop_assert_eq!(reference.head_set_hash(), meshcore::roster::members_of_head_set_hash(&ref_heads));
    }
}

// ---- adversarial review -------------------------------------------------------

/// D3 UI labels: "[Name] removed · Waiting for people out of range" is the
/// label for a *removal*. A member who drops out of the view because the
/// Add that admitted them was voided by a sender-chain equivocation was not
/// removed by anyone, so that string must not be shown; the generic merge
/// alert "Changed after reconnect" is.
#[test]
fn review_equivocation_voided_add_is_not_labelled_removed() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1], &[]);
    let _a = fx.add(0, 1, 5, vec![gh]);
    let before = fx.view();
    assert!(before.is_member(&key(5)));
    let (next, _) = fx.next_seq(0);
    let twin = fx.add_op(0, 1, 6, vec![gh], (true, false), Some(next - 1));
    fx.push(0, twin, false);
    let after = fx.view();
    assert!(!after.is_member(&key(5)));
    assert!(!after.removed.contains(&key(5)));
    let ls = labels(&before, &after);
    assert_eq!(ls.len(), 1);
    assert_eq!(ls[0].key, key(5));
    assert_eq!(ls[0].kind, LabelKind::ChangedAfterReconnect);
    assert_eq!(ls[0].text, "Changed after reconnect");
    assert!(ls[0].detail.is_none());
    // a real removal still gets the removal label
    let r = fx.remove(1, 0, vec![gh]); // 0 is an admin: invalid, nothing changes
    assert_eq!(fx.view().void_reason(&r), Some(VoidReason::SubjectIsAdmin));
    let (mut fx2, gh2) = Fixture::new(0, &[0, 1], &[3]);
    let before2 = fx2.view();
    fx2.remove(0, 3, vec![gh2]);
    let after2 = fx2.view();
    let ls2 = labels(&before2, &after2);
    assert_eq!(ls2.len(), 1);
    assert_eq!(ls2[0].kind, LabelKind::RemovedWaiting);
    assert_eq!(ls2[0].text, "p3 removed · Waiting for people out of range");
}

/// The creator's genesis has `author_seq = 0`; a later object by the same
/// author re-using seq 0 (e.g. after a reinstall that lost the chain state)
/// is ignored, but the genesis itself never is (voiding it would destroy the
/// group): the group keeps its admins, members and name.
#[test]
fn review_genesis_survives_a_seq_zero_equivocation() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1], &[3]);
    let twin = fx.setting_op(0, setting::FLAGS, 0xff, vec![gh], Some(0));
    let th = fx.push(0, twin, false);
    let v = fx.view();
    assert_eq!(v.void_reason(&th), Some(VoidReason::Equivocation));
    assert!(v.is_valid(&gh));
    assert_eq!(v.admins, keys(&[0, 1]));
    assert_eq!(fx.member_keys(), keys(&[0, 1, 3]));
    assert_eq!(v.flags(), 0);
    assert_eq!(v.name, "Shelter A");
}

/// An Add made honestly in one fragment is void when its author is demoted
/// concurrently, even if the removal that follows the demotion is causally
/// *after* the Add (the demotion alone is the concurrent power change).
#[test]
fn review_concurrent_demotion_voids_add_even_if_removal_is_later() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1, 2], &[]);
    let a = fx.add(2, 1, 5, vec![gh]);
    let d = fx.admin_set(0, &[0, 1], &[0, 1], 1, vec![gh]);
    let r = fx.remove(0, 2, vec![a, d]);
    let v = fx.view();
    assert!(v.is_valid(&d) && v.is_valid(&r));
    assert_eq!(
        v.void_reason(&a),
        Some(VoidReason::AuthorDemotedConcurrently)
    );
    assert!(!v.is_member(&key(5)) && !v.is_member(&key(2)));
    // the demotion alone, cited by the Add, makes the Add plainly unauthorised
    let late = fx.add(2, 1, 6, vec![d]);
    assert_eq!(
        fx.view().void_reason(&late),
        Some(VoidReason::AuthorNotAdmin)
    );
}

/// A re-added member's posts: one citing heads that hold only the removal
/// is rejected; one citing the re-add is accepted; one citing heads before
/// the removal is kept and labelled only while the author is out.
#[test]
fn review_post_verdict_respects_a_causal_readd() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1], &[3]);
    let three = key(3);
    let r = fx.remove(0, 3, vec![gh]);
    let out = fx.view();
    assert_eq!(
        post_verdict(&fx.dag, &three, &[gh], &out),
        PostVerdict::KeepLabelledRemoved
    );
    assert_eq!(
        post_verdict(&fx.dag, &three, &[r], &out),
        PostVerdict::RejectRemovedInHeads
    );
    let a = fx.add(1, 0, 3, vec![r]);
    let back = fx.view();
    assert!(back.is_member(&three));
    assert_eq!(
        post_verdict(&fx.dag, &three, &[a], &back),
        PostVerdict::Accept
    );
    assert_eq!(
        post_verdict(&fx.dag, &three, &[gh], &back),
        PostVerdict::Accept
    );
    // a post from the removed interval stays rejected everywhere, forever
    assert_eq!(
        post_verdict(&fx.dag, &three, &[r], &back),
        PostVerdict::RejectRemovedInHeads
    );
}

/// Remove-wins also protects against a key that was never a member: an Add
/// concurrent with a (valid) pre-emptive Remove of that key loses, and a
/// re-add citing the Remove wins.
#[test]
fn review_remove_of_a_never_member_still_wins_over_a_concurrent_add() {
    let (mut fx, gh) = Fixture::new(0, &[0, 1], &[]);
    let r = fx.remove(0, 9, vec![gh]);
    let a = fx.add(1, 0, 9, vec![gh]);
    let v = fx.view();
    assert!(v.is_valid(&r));
    assert_eq!(v.void_reason(&a), Some(VoidReason::RemoveWins));
    assert!(!v.is_member(&key(9)));
    let a2 = fx.add(1, 0, 9, vec![r, a]);
    let v = fx.view();
    assert!(v.is_valid(&a2) && v.is_member(&key(9)));
    assert!(!v.removed.contains(&key(9)));
}
