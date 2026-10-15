//! Integration tests for the crypto backend through the public API
//! (docs/DESIGN.md §1 "Backend trait", §10 "backends"; BRIEF_v2 §5.2–5.3;
//! DECISIONS D3 K1–K2, D6 rules 1–7). Default features only: the vodozemac
//! stub has nothing to exercise (docs/testing.md "Backend stub").
//!
//! Every node is a `MemKv` plus an Ed25519 signer; keys are pinned from the
//! peer's `KeyCard` exactly as the roster layer will do. RNGs are seeded so
//! every run is reproducible.
#![cfg(feature = "backend-libsignal")]

use std::sync::OnceLock;

use meshcore::codec;
use meshcore::crypto::backend::libsignal::LibsignalBackend;
use meshcore::crypto::backend::{
    Backend, BackendError, ChainId, GroupBackend, GroupId, PairwiseBackend, PairwiseCt,
    PairwiseKind, PeerId, PreKeyBundleWire, default_backend,
};
use meshcore::crypto::ed25519::IdentitySigner;
use meshcore::crypto::keycard::KeyCard;
use meshcore::storage::kv::{KvTxn, MemKv, ns};
use proptest::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

const HOUR: u64 = 60 * 60 * 1000;
const DAY: u64 = 24 * HOUR;

fn rng(seed: u64) -> ChaCha20Rng {
    ChaCha20Rng::seed_from_u64(seed)
}

fn be() -> Box<dyn Backend> {
    default_backend()
}

/// One mesh node: its transaction store, Ed25519 identity, KeyCard and RNG.
struct Node {
    kv: MemKv,
    signer: IdentitySigner,
    card: KeyCard,
    rng: ChaCha20Rng,
}

impl Node {
    fn new(seed: u8) -> Node {
        let signer = IdentitySigner::from_seed(&[seed; 32]);
        let mut kv = MemKv::new();
        let mut rng = rng(u64::from(seed));
        let x25519 = be()
            .init_identity(&mut kv, &mut rng)
            .expect("init identity");
        let card = KeyCard::issue(&signer, x25519);
        card.verify().expect("own keycard verifies");
        Node {
            kv,
            signer,
            card,
            rng,
        }
    }

    fn id(&self) -> PeerId {
        self.signer.public()
    }

    /// Pin `other`'s X25519 key from its KeyCard bytes, as the roster does.
    fn pin(&mut self, other: &Node) {
        let card = KeyCard::from_bytes(&other.card.to_bytes()).expect("peer keycard verifies");
        be().pin_peer_x25519(&mut self.kv, &card.ed, &card.x25519)
            .expect("pin");
    }

    fn bundle(&mut self, one_time: u8) -> Vec<u8> {
        be().make_prekey_bundle(&mut self.kv, one_time, &mut self.rng)
            .expect("bundle")
    }

    fn process_bundle(&mut self, from: &Node, bundle: &[u8], now: u64) -> Result<(), BackendError> {
        be().process_prekey_bundle(&mut self.kv, &from.id(), bundle, now, &mut self.rng)
    }

    fn send(&mut self, to: &Node, plain: &[u8], now: u64) -> PairwiseCt {
        PairwiseBackend::encrypt(&*be(), &mut self.kv, &to.id(), plain, now, &mut self.rng)
            .expect("encrypt")
    }

    fn recv(&mut self, from: &Node, ct: &PairwiseCt) -> Result<Vec<u8>, BackendError> {
        PairwiseBackend::decrypt(&*be(), &mut self.kv, &from.id(), ct, &mut self.rng)
    }

    fn new_chain(&mut self, group: &GroupId) -> (ChainId, Vec<u8>) {
        be().new_sender_chain(&mut self.kv, group, &mut self.rng)
            .expect("new sender chain")
    }

    fn accept_skdm(
        &mut self,
        group: &GroupId,
        from: &Node,
        skdm: &[u8],
    ) -> Result<ChainId, BackendError> {
        be().process_skdm(&mut self.kv, group, &from.id(), skdm)
    }

    fn post(&mut self, group: &GroupId, plain: &[u8]) -> Vec<u8> {
        GroupBackend::encrypt(&*be(), &mut self.kv, group, plain, &mut self.rng)
            .expect("group encrypt")
    }

    fn read(
        &mut self,
        group: &GroupId,
        from: &Node,
        ct: &[u8],
    ) -> Result<(ChainId, Vec<u8>), BackendError> {
        GroupBackend::decrypt(&*be(), &mut self.kv, group, &from.id(), ct)
    }

    fn count(&self, namespace: &str) -> usize {
        self.kv.scan_prefix(namespace, &[]).len()
    }
}

/// Two nodes that pinned each other.
fn pair(seed_a: u8, seed_b: u8) -> (Node, Node) {
    let mut a = Node::new(seed_a);
    let mut b = Node::new(seed_b);
    a.pin(&b);
    b.pin(&a);
    (a, b)
}

fn decode_bundle(bytes: &[u8]) -> PreKeyBundleWire {
    codec::decode(bytes).expect("bundle decodes")
}

/// Every (namespace, key, value) of a store, for byte-level comparison.
fn snapshot(kv: &MemKv) -> Vec<(&'static str, Vec<u8>, Vec<u8>)> {
    [
        ns::IDENTITY,
        ns::SESSIONS,
        ns::PREKEYS,
        ns::SIGNED_PREKEYS,
        ns::KYBER_PREKEYS,
        ns::SENDER_KEYS,
        ns::PEER_IDENTITIES,
    ]
    .into_iter()
    .flat_map(|n| {
        kv.scan_prefix(n, &[])
            .into_iter()
            .map(move |(k, v)| (n, k, v))
    })
    .collect()
}

// ---- (1) PQXDH session start and Double Ratchet both ways -------------------

#[test]
fn two_nodes_start_a_session_and_ratchet_both_ways() {
    let (mut a, mut b) = pair(1, 2);
    let be = be();

    let bundle = a.bundle(2);
    let wire = decode_bundle(&bundle);
    assert_eq!(wire.one_time.len(), 2);
    assert_eq!(wire.identity_key[0], 0x05);
    assert_eq!(
        &wire.identity_key[1..],
        &a.card.x25519,
        "bundle identity = KeyCard x25519"
    );
    assert_eq!(a.count(ns::PREKEYS), 2);
    assert_eq!(a.count(ns::KYBER_PREKEYS), 1);
    assert_eq!(a.count(ns::SIGNED_PREKEYS), 1);
    assert_eq!(
        bundle,
        codec::encode(&wire),
        "canonical: re-encoding is identical"
    );

    b.process_bundle(&a, &bundle, 1_000)
        .expect("B processes A's bundle");
    assert!(be.has_session(&b.kv, &a.id()));
    assert!(!be.has_session(&a.kv, &b.id()));
    assert_eq!(be.remote_session_key(&b.kv, &a.id()), Some(a.card.x25519));
    assert_eq!(be.remote_session_key(&a.kv, &b.id()), None);

    let m1 = b.send(&a, b"hello from B", 2_000);
    assert_eq!(
        m1.kind,
        PairwiseKind::PreKey,
        "first flight carries the handshake"
    );
    assert_eq!(a.recv(&b, &m1).unwrap(), b"hello from B");
    assert_eq!(
        a.count(ns::PREKEYS),
        1,
        "the one-time prekey B used was deleted"
    );
    assert_eq!(
        a.count(ns::KYBER_PREKEYS),
        0,
        "the one-time Kyber prekey was deleted"
    );
    assert_eq!(
        a.count(ns::SIGNED_PREKEYS),
        1,
        "the signed prekey stays for other bundles"
    );
    assert!(be.has_session(&a.kv, &b.id()));
    assert_eq!(be.remote_session_key(&a.kv, &b.id()), Some(b.card.x25519));

    let r1 = a.send(&b, b"hi B, this is A", 3_000);
    assert_eq!(
        r1.kind,
        PairwiseKind::Normal,
        "the responder never sends a prekey message"
    );
    assert_eq!(b.recv(&a, &r1).unwrap(), b"hi B, this is A");

    for i in 0..5u8 {
        let text = format!("B#{i}");
        let ct = b.send(&a, text.as_bytes(), 4_000 + u64::from(i));
        assert_eq!(
            ct.kind,
            PairwiseKind::Normal,
            "acknowledged session sends normal messages"
        );
        assert_eq!(a.recv(&b, &ct).unwrap(), text.as_bytes());
        let text = format!("A#{i}");
        let ct = a.send(&b, text.as_bytes(), 5_000 + u64::from(i));
        assert_eq!(ct.kind, PairwiseKind::Normal);
        assert_eq!(b.recv(&a, &ct).unwrap(), text.as_bytes());
    }
    // same inputs, different ciphertexts: the ratchet moved
    let c1 = b.send(&a, b"same", 6_000);
    let c2 = b.send(&a, b"same", 6_000);
    assert_ne!(c1.bytes, c2.bytes);
    assert_eq!(a.recv(&b, &c1).unwrap(), b"same");
    assert_eq!(a.recv(&b, &c2).unwrap(), b"same");
}

// ---- (2) out-of-order delivery uses skipped message keys ---------------------

#[test]
fn out_of_order_delivery_uses_skipped_keys() {
    let (mut a, mut b) = pair(3, 4);
    let bundle = a.bundle(1);
    b.process_bundle(&a, &bundle, 0).unwrap();

    let msgs: Vec<PairwiseCt> = (1..=5u8)
        .map(|i| b.send(&a, &[i; 20], u64::from(i)))
        .collect();
    assert!(
        msgs.iter().all(|m| m.kind == PairwiseKind::PreKey),
        "all before A replies"
    );

    assert_eq!(a.recv(&b, &msgs[2]).unwrap(), [3; 20], "#3 first");
    assert_eq!(a.count(ns::PREKEYS), 0, "#3 consumed the one-time prekey");
    for i in [0usize, 1, 3, 4] {
        assert_eq!(
            a.recv(&b, &msgs[i]).unwrap(),
            [i as u8 + 1; 20],
            "#{}",
            i + 1
        );
    }
    assert_eq!(
        a.recv(&b, &msgs[2]),
        Err(BackendError::Decrypt),
        "a replay is refused"
    );
    assert_eq!(
        a.recv(&b, &msgs[0]),
        Err(BackendError::Decrypt),
        "skipped keys are deleted on use"
    );

    // the same on normal messages in the other direction
    let replies: Vec<PairwiseCt> = (1..=4u8).map(|i| a.send(&b, &[0x40 + i; 8], 10)).collect();
    assert!(replies.iter().all(|m| m.kind == PairwiseKind::Normal));
    assert_eq!(b.recv(&a, &replies[3]).unwrap(), [0x44; 8]);
    assert_eq!(b.recv(&a, &replies[0]).unwrap(), [0x41; 8]);
    assert_eq!(b.recv(&a, &replies[2]).unwrap(), [0x43; 8]);
    assert_eq!(b.recv(&a, &replies[1]).unwrap(), [0x42; 8]);
    assert_eq!(b.recv(&a, &replies[1]), Err(BackendError::Decrypt));
}

// ---- (3) TOFU is off: unpinned or mis-pinned peers are refused ---------------

#[test]
fn unpinned_or_mispinned_peer_is_refused_without_storing_anything() {
    let mut a = Node::new(5);
    let mut c = Node::new(6);
    let d = Node::new(7);
    let bundle = a.bundle(1);

    // C never pinned A
    assert_eq!(
        c.process_bundle(&a, &bundle, 0),
        Err(BackendError::UntrustedIdentity)
    );
    assert_eq!(c.count(ns::SESSIONS), 0, "nothing stored");
    assert!(!be().has_session(&c.kv, &a.id()));

    // C pinned the wrong key for A
    be().pin_peer_x25519(&mut c.kv, &a.id(), &d.card.x25519)
        .unwrap();
    assert_eq!(
        c.process_bundle(&a, &bundle, 0),
        Err(BackendError::UntrustedIdentity)
    );
    assert_eq!(c.count(ns::SESSIONS), 0);

    // a bundle whose identity key is not the one that signed it is malformed
    let mut forged = decode_bundle(&bundle);
    forged.identity_key[1..].copy_from_slice(&d.card.x25519);
    be().pin_peer_x25519(&mut c.kv, &a.id(), &d.card.x25519)
        .unwrap();
    assert_eq!(
        c.process_bundle(&a, &codec::encode(&forged), 0),
        Err(BackendError::Encoding),
        "pinned key matches the forged identity, but the prekey signatures do not verify"
    );

    // correct pin: works
    c.pin(&a);
    c.process_bundle(&a, &bundle, 0).unwrap();
    let m = c.send(&a, b"from C", 1);
    assert_eq!(m.kind, PairwiseKind::PreKey);

    // A never pinned C: the first flight is refused and consumes nothing
    let prekeys_before = a.count(ns::PREKEYS);
    assert_eq!(a.recv(&c, &m), Err(BackendError::UntrustedIdentity));
    assert_eq!(
        a.count(ns::PREKEYS),
        prekeys_before,
        "one-time prekey not consumed"
    );
    assert_eq!(a.count(ns::KYBER_PREKEYS), 1, "kyber prekey not consumed");
    assert_eq!(a.count(ns::SESSIONS), 0, "no session stored");

    // A pins C: the same first flight now decrypts
    a.pin(&c);
    assert_eq!(a.recv(&c, &m).unwrap(), b"from C");
    assert_eq!(a.count(ns::PREKEYS), prekeys_before - 1);

    // encrypting to an unpinned peer with no session: no session, not a crash
    assert_eq!(
        PairwiseBackend::encrypt(&*be(), &mut a.kv, &d.id(), b"x", 0, &mut a.rng),
        Err(BackendError::NoSession)
    );
    let normal = PairwiseCt {
        kind: PairwiseKind::Normal,
        bytes: m.bytes.clone(),
    };
    assert_eq!(
        a.recv(&d, &normal),
        Err(BackendError::Encoding),
        "a prekey message is not a normal one"
    );
}

// ---- (4) group sender keys -----------------------------------------------------

#[test]
fn group_sender_key_round_trip() {
    let (mut a, mut b) = pair(8, 9);
    let g = GroupId([0x11; 16]);
    let other = GroupId([0x22; 16]);
    let be = be();

    assert_eq!(be.own_chain(&a.kv, &g), None);
    let (chain, skdm) = a.new_chain(&g);
    assert_eq!(be.own_chain(&a.kv, &g), Some(chain));
    assert_eq!(b.accept_skdm(&g, &a, &skdm).unwrap(), chain);
    assert_eq!(
        b.accept_skdm(&other, &a, &skdm),
        Err(BackendError::Encoding),
        "SKDM for another group"
    );

    let post = a.post(&g, b"first post");
    assert_eq!(
        b.read(&g, &a, &post).unwrap(),
        (chain, b"first post".to_vec())
    );
    assert_eq!(
        b.read(&g, &a, &post),
        Err(BackendError::Decrypt),
        "message key deleted on use"
    );
    assert_eq!(b.read(&other, &a, &post), Err(BackendError::Encoding));

    // B has no chain of its own yet; A has no SKDM from B
    assert_eq!(
        GroupBackend::encrypt(&*be, &mut b.kv, &g, b"x", &mut b.rng),
        Err(BackendError::NoSenderKey)
    );
    let (bchain, bskdm) = b.new_chain(&g);
    let bpost = b.post(&g, b"B speaks");
    assert_eq!(
        a.read(&g, &b, &bpost),
        Err(BackendError::NoSenderKey),
        "SKDM not yet received"
    );
    a.accept_skdm(&g, &b, &bskdm).unwrap();
    assert_eq!(
        a.read(&g, &b, &bpost).unwrap(),
        (bchain, b"B speaks".to_vec())
    );

    // out of order within a chain
    let p1 = a.post(&g, b"one");
    let p2 = a.post(&g, b"two");
    let p3 = a.post(&g, b"three");
    assert_eq!(b.read(&g, &a, &p3).unwrap().1, b"three");
    assert_eq!(b.read(&g, &a, &p1).unwrap().1, b"one");
    assert_eq!(b.read(&g, &a, &p2).unwrap().1, b"two");

    // a re-delivered SKDM does not reset the chain
    assert_eq!(b.accept_skdm(&g, &a, &skdm).unwrap(), chain);
    let p4 = a.post(&g, b"four");
    assert_eq!(b.read(&g, &a, &p4).unwrap().1, b"four");
    assert_eq!(
        b.read(&g, &a, &p1),
        Err(BackendError::Decrypt),
        "still a duplicate"
    );
}

// ---- (5) rotation: old chains stay readable up to the library's cap ---------

#[test]
fn rotation_keeps_previous_chains_up_to_five_states() {
    let (mut a, mut b) = pair(10, 11);
    let g = GroupId([0x33; 16]);

    let (chain0, skdm0) = a.new_chain(&g);
    b.accept_skdm(&g, &a, &skdm0).unwrap();
    let late0 = a.post(&g, b"late under chain 0");

    let (chain1, skdm1) = a.new_chain(&g);
    assert_ne!(chain1, chain0, "rotation gives a new chain id");
    assert_eq!(be().own_chain(&a.kv, &g), Some(chain1));
    let early1 = a.post(&g, b"first under chain 1");
    // the late message from the old chain arrives after the new SKDM
    b.accept_skdm(&g, &a, &skdm1).unwrap();
    assert_eq!(
        b.read(&g, &a, &early1).unwrap(),
        (chain1, b"first under chain 1".to_vec())
    );
    assert_eq!(
        b.read(&g, &a, &late0).unwrap(),
        (chain0, b"late under chain 0".to_vec())
    );

    // keep one late post per chain, rotate on
    let mut chains = vec![chain0, chain1];
    let mut late = vec![a.post(&g, b"x"), a.post(&g, b"x")]; // chain0's is a duplicate position; replaced below
    late[0] = late0.clone();
    late[1] = a.post(&g, b"late under chain 1");
    for _ in 2..=6 {
        let (chain, skdm) = a.new_chain(&g);
        b.accept_skdm(&g, &a, &skdm).unwrap();
        late.push(a.post(&g, b"late"));
        chains.push(chain);
        if chains.len() == 5 {
            // after 4 rotations every chain so far is still readable
            let mut probe = b.kv.clone();
            for (i, ct) in late.iter().enumerate() {
                let r = GroupBackend::decrypt(&*be(), &mut probe, &g, &a.id(), ct);
                assert!(
                    r.is_ok() || r == Err(BackendError::Decrypt),
                    "chain {i} still present (Decrypt = duplicate counter)"
                );
                assert_ne!(
                    r,
                    Err(BackendError::NoSenderKey),
                    "chain {i} evicted too early"
                );
            }
        }
    }
    assert_eq!(chains.len(), 7, "chain 0 plus six rotations");
    assert_eq!(
        chains
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        7
    );

    // libsignal keeps MAX_SENDER_KEY_STATES = 5 states per (sender, group):
    // chains 2..=6 remain, chains 0 and 1 are gone
    assert_eq!(
        b.read(&g, &a, &late[0]),
        Err(BackendError::NoSenderKey),
        "chain 0 evicted"
    );
    assert_eq!(
        b.read(&g, &a, &late[1]),
        Err(BackendError::NoSenderKey),
        "chain 1 evicted"
    );
    for i in 2..=6 {
        assert_eq!(
            b.read(&g, &a, &late[i]).unwrap(),
            (chains[i], b"late".to_vec()),
            "chain {i} retained"
        );
    }
    assert_eq!(be().own_chain(&a.kv, &g), Some(chains[6]));
    // the sender only keeps its current chain
    let (final_chain, _) = a.new_chain(&g);
    assert_eq!(be().own_chain(&a.kv, &g), Some(final_chain));
    assert_eq!(a.count(ns::SENDER_KEYS), 1);
}

// ---- (6) persistence through a cloned store ------------------------------------

#[test]
fn state_round_trips_through_a_cloned_store() {
    let (mut a, mut b) = pair(12, 13);
    let g = GroupId([0x44; 16]);
    let bundle = a.bundle(1);
    b.process_bundle(&a, &bundle, 0).unwrap();
    let m1 = b.send(&a, b"one", 1);
    assert_eq!(a.recv(&b, &m1).unwrap(), b"one");
    let r1 = a.send(&b, b"ack", 2);
    assert_eq!(b.recv(&a, &r1).unwrap(), b"ack");
    let (chain, skdm) = a.new_chain(&g);
    b.accept_skdm(&g, &a, &skdm).unwrap();

    // "restart": a fresh backend instance over a copy of the bytes
    let snapshot_a = a.kv.clone();
    let snapshot_b = b.kv.clone();
    let fresh = LibsignalBackend::new();
    let mut a2 = snapshot_a;
    let mut b2 = snapshot_b;
    let mut rng_a = rng(1200);
    let mut rng_b = rng(1300);

    let m2 = PairwiseBackend::encrypt(&fresh, &mut b2, &a.id(), b"two", 3, &mut rng_b).unwrap();
    assert_eq!(m2.kind, PairwiseKind::Normal);
    assert_eq!(
        PairwiseBackend::decrypt(&fresh, &mut a2, &b.id(), &m2, &mut rng_a).unwrap(),
        b"two"
    );
    let r2 = PairwiseBackend::encrypt(&fresh, &mut a2, &b.id(), b"ack two", 4, &mut rng_a).unwrap();
    assert_eq!(
        PairwiseBackend::decrypt(&fresh, &mut b2, &a.id(), &r2, &mut rng_b).unwrap(),
        b"ack two"
    );
    assert_eq!(fresh.own_chain(&a2, &g), Some(chain));
    let post = GroupBackend::encrypt(&fresh, &mut a2, &g, b"post", &mut rng_a).unwrap();
    assert_eq!(
        GroupBackend::decrypt(&fresh, &mut b2, &g, &a.id(), &post).unwrap(),
        (chain, b"post".to_vec())
    );
    assert_eq!(fresh.local_x25519(&a2), Some(a.card.x25519));
    assert_eq!(fresh.remote_session_key(&b2, &a.id()), Some(a.card.x25519));
    // the originals, untouched by the copies, still work on their own timeline
    let m3 = b.send(&a, b"three", 5);
    assert_eq!(a.recv(&b, &m3).unwrap(), b"three");
}

// ---- (7) a dropped transaction does not advance the ratchet ------------------

#[test]
fn dropped_transaction_does_not_advance_the_ratchet() {
    let (mut a, mut b) = pair(14, 15);
    let g = GroupId([0x55; 16]);
    let bundle = a.bundle(1);
    b.process_bundle(&a, &bundle, 0).unwrap();
    let (_, skdm) = b.new_chain(&g);
    a.accept_skdm(&g, &b, &skdm).unwrap();
    let ct = b.send(&a, b"atomic", 1);
    let post = b.post(&g, b"atomic post");

    let before = a.kv.clone();
    {
        let mut work = a.kv.clone();
        assert_eq!(
            PairwiseBackend::decrypt(&*be(), &mut work, &b.id(), &ct, &mut rng(1)).unwrap(),
            b"atomic"
        );
        assert_eq!(
            GroupBackend::decrypt(&*be(), &mut work, &g, &b.id(), &post)
                .unwrap()
                .1,
            b"atomic post"
        );
        drop(work); // the transaction is discarded: nothing was committed
    }
    assert_eq!(
        snapshot(&a.kv),
        snapshot(&before),
        "the original store is byte-identical"
    );
    let mut again = before.clone();
    assert_eq!(
        PairwiseBackend::decrypt(&*be(), &mut again, &b.id(), &ct, &mut rng(2)).unwrap(),
        b"atomic",
        "the pre-transaction copy still decrypts the same ciphertext"
    );
    assert_eq!(
        GroupBackend::decrypt(&*be(), &mut again, &g, &b.id(), &post)
            .unwrap()
            .1,
        b"atomic post"
    );

    // commit = decrypt on the real store; a second attempt is refused
    assert_eq!(a.recv(&b, &ct).unwrap(), b"atomic");
    assert_ne!(
        snapshot(&a.kv),
        snapshot(&before),
        "the committed store advanced"
    );
    assert_eq!(a.recv(&b, &ct), Err(BackendError::Decrypt));
    assert_eq!(a.read(&g, &b, &post).unwrap().1, b"atomic post");
    assert_eq!(a.read(&g, &b, &post), Err(BackendError::Decrypt));
}

// ---- (8) signed prekey rotation -----------------------------------------------

#[test]
fn signed_prekey_rotation_keeps_old_sessions_working() {
    let (mut a, mut b) = pair(16, 17);
    let mut c = Node::new(18);
    let mut d = Node::new(19);
    let mut e = Node::new(20);
    for n in [&mut c, &mut d, &mut e] {
        n.pin(&a);
        a.pin(n);
    }

    let bundle_b = a.bundle(1);
    let bundle_d = a.bundle(1);
    let bundle_e = a.bundle(0);
    let w_b = decode_bundle(&bundle_b);
    let w_d = decode_bundle(&bundle_d);
    assert_eq!(
        w_b.signed_prekey_id, w_d.signed_prekey_id,
        "same generation, same SPK"
    );
    assert_ne!(
        w_b.kyber_prekey_id, w_d.kyber_prekey_id,
        "fresh Kyber prekey per bundle"
    );
    assert_ne!(w_b.one_time, w_d.one_time);
    b.process_bundle(&a, &bundle_b, 0).unwrap();

    be().rotate_signed_prekey(&mut a.kv, DAY, &mut a.rng)
        .unwrap();
    let bundle_c = a.bundle(1);
    let w_c = decode_bundle(&bundle_c);
    assert_ne!(
        w_c.signed_prekey_id, w_b.signed_prekey_id,
        "bundle after rotation uses the new SPK"
    );
    assert_ne!(w_c.signed_prekey, w_b.signed_prekey);
    assert_eq!(w_c.identity_key, w_b.identity_key);
    assert_eq!(a.count(ns::SIGNED_PREKEYS), 2);

    // session started before rotation completes afterwards
    let m_b = b.send(&a, b"B started before rotation", DAY + 1);
    assert_eq!(m_b.kind, PairwiseKind::PreKey);
    assert_eq!(a.recv(&b, &m_b).unwrap(), b"B started before rotation");
    // a bundle issued before rotation still starts a session afterwards
    d.process_bundle(&a, &bundle_d, DAY + 2).unwrap();
    let m_d = d.send(&a, b"D used an old bundle", DAY + 3);
    assert_eq!(a.recv(&d, &m_d).unwrap(), b"D used an old bundle");
    // the post-rotation bundle works too
    c.process_bundle(&a, &bundle_c, DAY + 4).unwrap();
    let m_c = c.send(&a, b"C used the new bundle", DAY + 5);
    assert_eq!(a.recv(&c, &m_c).unwrap(), b"C used the new bundle");

    // the caller's deletion rule retires the old SPK; a first flight that still
    // references it is then refused (D6 rule 7 "May have expired · Resend")
    assert_eq!(
        LibsignalBackend::new().retire_signed_prekeys(&mut a.kv, DAY),
        1
    );
    assert_eq!(a.count(ns::SIGNED_PREKEYS), 1);
    e.process_bundle(&a, &bundle_e, 2 * DAY).unwrap();
    let m_e = e.send(&a, b"E is too late", 2 * DAY + 1);
    assert_eq!(a.recv(&e, &m_e), Err(BackendError::Decrypt));
    // established sessions are untouched by SPK deletion (D6 rule 6)
    let r = a.send(&b, b"still fine", 2 * DAY + 2);
    assert_eq!(b.recv(&a, &r).unwrap(), b"still fine");
    let m_b2 = b.send(&a, b"and back", 2 * DAY + 3);
    assert_eq!(m_b2.kind, PairwiseKind::Normal);
    assert_eq!(a.recv(&b, &m_b2).unwrap(), b"and back");
}

// ---- (9) message sizes --------------------------------------------------------

#[test]
fn message_sizes_for_a_100_byte_plaintext() {
    let (mut a, mut b) = pair(21, 22);
    let g = GroupId([0x66; 16]);
    let plain = [0x5a; 100];

    let bundle0 = a.bundle(0);
    let bundle1 = a.bundle(1);
    b.process_bundle(&a, &bundle1, 0).unwrap();
    let prekey = b.send(&a, &plain, 1);
    assert_eq!(prekey.kind, PairwiseKind::PreKey);
    a.recv(&b, &prekey).unwrap();
    let normal = a.send(&b, &plain, 2);
    assert_eq!(normal.kind, PairwiseKind::Normal);
    b.recv(&a, &normal).unwrap();
    let normal2 = b.send(&a, &plain, 3);
    assert_eq!(normal2.kind, PairwiseKind::Normal);
    let (_, skdm) = a.new_chain(&g);
    b.accept_skdm(&g, &a, &skdm).unwrap();
    let skm = a.post(&g, &plain);

    println!("bundle (0 one-time prekeys): {} B", bundle0.len());
    println!("bundle (1 one-time prekey):  {} B", bundle1.len());
    println!(
        "PreKey message, 100 B plaintext:   {} B",
        prekey.bytes.len()
    );
    println!(
        "Normal message, 100 B plaintext:   {} B (responder)",
        normal.bytes.len()
    );
    println!(
        "Normal message, 100 B plaintext:   {} B (initiator)",
        normal2.bytes.len()
    );
    println!("SKDM:                              {} B", skdm.len());
    println!("SenderKeyMessage, 100 B plaintext: {} B", skm.len());

    assert!(prekey.bytes.len() > normal.bytes.len());
    assert!(
        (1500..=3000).contains(&prekey.bytes.len()),
        "PreKey {}",
        prekey.bytes.len()
    );
    assert!(
        (120..=700).contains(&normal.bytes.len()),
        "Normal {}",
        normal.bytes.len()
    );
    assert!((60..=200).contains(&skdm.len()), "SKDM {}", skdm.len());
    assert!((150..=300).contains(&skm.len()), "SKM {}", skm.len());
    assert_eq!(bundle1.len(), bundle0.len() + 4 + 33);
}

// ---- parsers survive arbitrary bytes -------------------------------------------

struct Fixture {
    a_kv: MemKv,
    a_id: PeerId,
    b_id: PeerId,
    group: GroupId,
    prekey_ct: Vec<u8>,
    normal_ct: Vec<u8>,
    skdm: Vec<u8>,
    skm: Vec<u8>,
    bundle: Vec<u8>,
}

/// A's store with a session from B, a chain from B, and real objects to mutate.
fn fixture() -> &'static Fixture {
    static FX: OnceLock<Fixture> = OnceLock::new();
    FX.get_or_init(|| {
        let (mut a, mut b) = pair(30, 31);
        let group = GroupId([0x77; 16]);
        let bundle = a.bundle(1);
        b.process_bundle(&a, &bundle, 0).unwrap();
        let first = b.send(&a, b"first", 1);
        a.recv(&b, &first).unwrap();
        let reply = a.send(&b, b"reply", 2);
        b.recv(&a, &reply).unwrap();
        let prekey_ct = first.bytes;
        let normal_ct = b.send(&a, b"normal", 3).bytes;
        let (_, skdm) = b.new_chain(&group);
        a.accept_skdm(&group, &b, &skdm).unwrap();
        let skm = b.post(&group, b"post");
        Fixture {
            a_kv: a.kv,
            a_id: a.signer.public(),
            b_id: b.signer.public(),
            group,
            prekey_ct,
            normal_ct,
            skdm,
            skm,
            bundle,
        }
    })
}

fn feed(kv: &mut MemKv, fx: &Fixture, bytes: &[u8]) {
    let be = be();
    let mut r = rng(9);
    for kind in [PairwiseKind::PreKey, PairwiseKind::Normal] {
        let ct = PairwiseCt {
            kind,
            bytes: bytes.to_vec(),
        };
        let _ = PairwiseBackend::decrypt(&*be, kv, &fx.b_id, &ct, &mut r);
        let _ = PairwiseBackend::decrypt(&*be, kv, &fx.a_id, &ct, &mut r);
    }
    let _ = be.process_prekey_bundle(kv, &fx.b_id, bytes, 0, &mut r);
    let _ = be.process_skdm(kv, &fx.group, &fx.b_id, bytes);
    let _ = GroupBackend::decrypt(&*be, kv, &fx.group, &fx.b_id, bytes);
    let _ = codec::decode::<PreKeyBundleWire>(bytes);
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 64, ..ProptestConfig::default() })]

    #[test]
    fn parsers_survive_random_bytes(bytes in proptest::collection::vec(any::<u8>(), 0..2048)) {
        let fx = fixture();
        let mut kv = fx.a_kv.clone();
        feed(&mut kv, fx, &bytes);
    }

    #[test]
    fn parsers_survive_corrupted_real_objects(which in 0u8..5, pos in any::<usize>(), flip in 1u8..=255, cut in any::<bool>()) {
        let fx = fixture();
        let original: &[u8] = match which {
            0 => &fx.prekey_ct,
            1 => &fx.normal_ct,
            2 => &fx.skdm,
            3 => &fx.skm,
            _ => &fx.bundle,
        };
        let pos = pos % original.len();
        let mut bytes = original.to_vec();
        if cut {
            bytes.truncate(pos);
        } else {
            bytes[pos] ^= flip;
        }
        let mut kv = fx.a_kv.clone();
        feed(&mut kv, fx, &bytes);
        // the untouched store still reads the genuine objects
        let be = be();
        let ct = PairwiseCt { kind: PairwiseKind::Normal, bytes: fx.normal_ct.clone() };
        let mut fresh = fx.a_kv.clone();
        prop_assert_eq!(PairwiseBackend::decrypt(&*be, &mut fresh, &fx.b_id, &ct, &mut rng(1)).unwrap(), b"normal");
        prop_assert_eq!(GroupBackend::decrypt(&*be, &mut fresh, &fx.group, &fx.b_id, &fx.skm).unwrap().1, b"post");
    }
}

// ---- reviewer additions ---------------------------------------------------------

/// Same seeds, same inputs: every byte the backend emits is identical across two
/// independent runs (DESIGN §10 "Determinism"; the simulator relies on it).
#[test]
fn whole_flow_is_deterministic_for_the_same_rng_streams() {
    fn run() -> Vec<Vec<u8>> {
        let (mut a, mut b) = pair(40, 41);
        let g = GroupId([0x88; 16]);
        let bundle = a.bundle(2);
        b.process_bundle(&a, &bundle, 7).unwrap();
        let prekey = b.send(&a, b"first", 8);
        a.recv(&b, &prekey).unwrap();
        let normal = a.send(&b, b"reply", 9);
        b.recv(&a, &normal).unwrap();
        let (_, skdm) = a.new_chain(&g);
        b.accept_skdm(&g, &a, &skdm).unwrap();
        let skm = a.post(&g, b"post");
        let snap = snapshot(&a.kv)
            .into_iter()
            .flat_map(|(n, k, v)| [n.as_bytes().to_vec(), k, v].concat())
            .collect();
        vec![bundle, prekey.bytes, normal.bytes, skdm, skm, snap]
    }
    assert_eq!(run(), run());
}

/// D6 rule 5 "crossed initiation": both sides process each other's bundle and
/// both send a first flight; libsignal's built-in promotion converges on one
/// session and nothing is lost.
#[test]
fn crossed_initiation_converges() {
    let (mut a, mut b) = pair(50, 51);
    let bundle_a = a.bundle(1);
    let bundle_b = b.bundle(1);
    a.process_bundle(&b, &bundle_b, 0).unwrap();
    b.process_bundle(&a, &bundle_a, 0).unwrap();
    let ma = a.send(&b, b"A first", 1);
    let mb = b.send(&a, b"B first", 1);
    assert_eq!(ma.kind, PairwiseKind::PreKey);
    assert_eq!(mb.kind, PairwiseKind::PreKey);
    // a second first flight from each side, sent before anything crossed
    let ma2 = a.send(&b, b"A second", 2);
    let mb2 = b.send(&a, b"B second", 2);
    assert_eq!(b.recv(&a, &ma).unwrap(), b"A first");
    assert_eq!(a.recv(&b, &mb).unwrap(), b"B first");
    for i in 0..4u8 {
        let ca = a.send(&b, &[0xa0 + i; 10], 10 + u64::from(i));
        assert_eq!(b.recv(&a, &ca).unwrap(), [0xa0 + i; 10], "A->B round {i}");
        let cb = b.send(&a, &[0xb0 + i; 10], 20 + u64::from(i));
        assert_eq!(a.recv(&b, &cb).unwrap(), [0xb0 + i; 10], "B->A round {i}");
    }
    // the stragglers from the other initiation still decrypt (archived states)
    assert_eq!(b.recv(&a, &ma2).unwrap(), b"A second");
    assert_eq!(a.recv(&b, &mb2).unwrap(), b"B second");
    let ca = a.send(&b, b"after stragglers", 30);
    assert_eq!(ca.kind, PairwiseKind::Normal);
    assert_eq!(b.recv(&a, &ca).unwrap(), b"after stragglers");
    let cb = b.send(&a, b"and back", 31);
    assert_eq!(cb.kind, PairwiseKind::Normal);
    assert_eq!(a.recv(&b, &cb).unwrap(), b"and back");
}

/// A first flight whose handshake is well formed but whose body does not
/// authenticate (anyone holding A's gossiped bundle and B's public KeyCard can
/// build one) must not consume A's one-time or Kyber prekeys and must leave no
/// session: otherwise a stranger could burn every bundle A ever handed out.
#[test]
fn unauthenticated_first_flight_consumes_no_prekey() {
    let (mut a, mut b) = pair(52, 53);
    let bundle = a.bundle(1);
    b.process_bundle(&a, &bundle, 0).unwrap();
    let genuine = b.send(&a, b"genuine", 1);
    let before = snapshot(&a.kv);
    // corrupt the Kyber ciphertext: the handshake parses and derives a session,
    // the shared secret differs, the MAC fails
    let mut forged = genuine.clone();
    let n = forged.bytes.len();
    forged.bytes[n - 200] ^= 0x01;
    assert_eq!(a.recv(&b, &forged), Err(BackendError::Decrypt));
    assert_eq!(snapshot(&a.kv), before, "nothing consumed, nothing stored");
    assert_eq!(a.count(ns::PREKEYS), 1);
    assert_eq!(a.count(ns::KYBER_PREKEYS), 1);
    assert!(!be().has_session(&a.kv, &b.id()));
    // the genuine one still works afterwards
    assert_eq!(a.recv(&b, &genuine).unwrap(), b"genuine");
}

/// Re-pinning a peer to a different key (the roster's D5 same-name decision)
/// makes the established session unusable in both directions without touching
/// it, and `has_session` reports that so the pairwise layer re-runs the bundle;
/// pinning the right key back restores it with no message key lost.
#[test]
fn repin_makes_the_session_unusable_and_has_session_false() {
    let (mut a, mut b) = pair(54, 55);
    let d = Node::new(56);
    let bundle = a.bundle(1);
    b.process_bundle(&a, &bundle, 0).unwrap();
    let m1 = b.send(&a, b"one", 1);
    assert_eq!(a.recv(&b, &m1).unwrap(), b"one");
    let r1 = a.send(&b, b"ack", 2);
    assert_eq!(b.recv(&a, &r1).unwrap(), b"ack");
    assert!(be().has_session(&a.kv, &b.id()));

    let m2 = b.send(&a, b"two", 3);
    let before = snapshot(&a.kv);
    be().pin_peer_x25519(&mut a.kv, &b.id(), &d.card.x25519)
        .unwrap();
    assert!(
        !be().has_session(&a.kv, &b.id()),
        "the session's key no longer matches the pin: it cannot send"
    );
    assert_eq!(
        PairwiseBackend::encrypt(&*be(), &mut a.kv, &b.id(), b"x", 4, &mut a.rng),
        Err(BackendError::UntrustedIdentity)
    );
    assert_eq!(a.recv(&b, &m2), Err(BackendError::UntrustedIdentity));
    let mut after = snapshot(&a.kv);
    after.retain(|(n, _, _)| *n != ns::PEER_IDENTITIES);
    let mut before_no_pin = before.clone();
    before_no_pin.retain(|(n, _, _)| *n != ns::PEER_IDENTITIES);
    assert_eq!(after, before_no_pin, "refusal touches nothing but the pin");

    a.pin(&b);
    assert!(be().has_session(&a.kv, &b.id()));
    assert_eq!(
        a.recv(&b, &m2).unwrap(),
        b"two",
        "message key was not consumed"
    );
    let r2 = a.send(&b, b"ack two", 5);
    assert_eq!(b.recv(&a, &r2).unwrap(), b"ack two");
}

/// `retire_signed_prekeys` must also drop the identity-signed Kyber prekeys
/// issued under the retired signed prekeys: they are useless once their SPK is
/// gone (a first flight needs both) and otherwise accumulate ~3 KB of
/// post-quantum secret per bundle ever handed out, past the deletion horizon
/// (BRIEF §5.3 "deleted by a powered-on-time rule", D6 rule 3).
#[test]
fn retiring_signed_prekeys_also_retires_their_kyber_prekeys() {
    let mut a = Node::new(57);
    let b = Node::new(58);
    a.pin(&b);
    let _ = a.bundle(0);
    let _ = a.bundle(0);
    let _ = a.bundle(1);
    assert_eq!(a.count(ns::KYBER_PREKEYS), 3);
    be().rotate_signed_prekey(&mut a.kv, DAY, &mut a.rng)
        .unwrap();
    let current = a.bundle(0);
    assert_eq!(a.count(ns::KYBER_PREKEYS), 4);
    assert_eq!(a.count(ns::SIGNED_PREKEYS), 2);

    assert_eq!(
        LibsignalBackend::new().retire_signed_prekeys(&mut a.kv, DAY),
        1
    );
    assert_eq!(a.count(ns::SIGNED_PREKEYS), 1);
    assert_eq!(
        a.count(ns::KYBER_PREKEYS),
        1,
        "only the current generation's Kyber prekey remains"
    );
    assert_eq!(
        a.count(ns::PREKEYS),
        1,
        "one-time prekeys are not time-bound"
    );
    // the surviving Kyber prekey is the one the post-rotation bundle references
    let w = decode_bundle(&current);
    assert!(
        a.kv.get(ns::KYBER_PREKEYS, &w.kyber_prekey_id.to_be_bytes())
            .is_some()
    );
    // and that bundle still starts a session
    let mut c = Node::new(59);
    c.pin(&a);
    a.pin(&c);
    c.process_bundle(&a, &current, 2 * DAY).unwrap();
    let m = c.send(&a, b"still works", 2 * DAY + 1);
    assert_eq!(a.recv(&c, &m).unwrap(), b"still works");
    assert_eq!(a.count(ns::KYBER_PREKEYS), 0);
}

/// Observed retention detail (test 5 measures the window; this pins its
/// order): libsignal keeps the five states ordered by the *last SKDM
/// delivery*, not by creation. Re-delivering an old chain's SKDM moves that
/// chain to the front, so the next eviction takes the next-oldest chain
/// instead. The groups layer should therefore not re-send old SKDMs casually.
#[test]
fn redelivered_skdm_moves_its_chain_to_the_front_of_the_retention_window() {
    let (mut a, mut b) = pair(60, 61);
    let g = GroupId([0x99; 16]);
    let mut skdms = Vec::new();
    let mut posts = Vec::new();
    let mut chains = Vec::new();
    for _ in 0..5 {
        let (c, s) = a.new_chain(&g);
        b.accept_skdm(&g, &a, &s).unwrap();
        posts.push(a.post(&g, b"p"));
        skdms.push(s);
        chains.push(c);
    }
    assert_eq!(b.accept_skdm(&g, &a, &skdms[0]).unwrap(), chains[0]);
    let (c5, s5) = a.new_chain(&g);
    b.accept_skdm(&g, &a, &s5).unwrap();
    let p5 = a.post(&g, b"p");
    assert_eq!(
        b.read(&g, &a, &posts[1]),
        Err(BackendError::NoSenderKey),
        "chain 1 was evicted"
    );
    assert_eq!(
        b.read(&g, &a, &posts[0]).unwrap(),
        (chains[0], b"p".to_vec()),
        "chain 0 survived because its SKDM was re-delivered"
    );
    for i in 2..5 {
        assert_eq!(b.read(&g, &a, &posts[i]).unwrap().0, chains[i]);
    }
    assert_eq!(b.read(&g, &a, &p5).unwrap().0, c5);
}
