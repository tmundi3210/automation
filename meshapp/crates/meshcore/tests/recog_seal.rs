//! Public-API tests for `meshcore::wire::recog` and `meshcore::wire::seal`
//! (docs/DESIGN.md §10 "recog tag windows and epoch rollover; seal/unseal
//! with ≤3 keys and wrong-key rejection"; DECISIONS §3 row (3,4,6,10);
//! BRIEF_v2 §5.5). Includes the fixed vectors that `wire/cell.rs` and the
//! platform ports assert against.

use meshcore::crypto::aead::{self, Key32};
use meshcore::crypto::hash::hkdf32;
use meshcore::wire::cell::{self, BODY_PLAINTEXT_LEN, TAG_LEN, VERSION};
use meshcore::wire::recog::{
    ChainKey, EpochId, Hit, LOOKAHEAD, MAX_EPOCHS, RecogEpoch, RecogError, TagTable, WINDOW,
    WindowState,
};
use meshcore::wire::seal::{
    EnvelopeKeyPair, MAX_LIVE_KEYS, MAX_PAYLOAD_LEN, OVERHEAD, SealError, Sealed, match_tag, seal,
    seal_tag, try_unseal,
};
use proptest::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

// ---- helpers ----------------------------------------------------------------

/// The fixed-vector seed: bytes 0x00..=0x1f.
fn vector_seed() -> [u8; 32] {
    core::array::from_fn(|i| i as u8)
}

fn epoch(n: u8) -> RecogEpoch {
    RecogEpoch::from_seed(&hkdf32(b"recog_seal test seed", b"", &[n]))
}

fn rng(seed: u64) -> ChaCha20Rng {
    ChaCha20Rng::seed_from_u64(seed)
}

fn pair(seed: u64) -> EnvelopeKeyPair {
    EnvelopeKeyPair::generate(&mut rng(seed))
}

const A: ChainKey = ChainKey(10);
const B: ChainKey = ChainKey(20);
const E0: EpochId = EpochId(0);
const E1: EpochId = EpochId(1);
const E2: EpochId = EpochId(2);
const E3: EpochId = EpochId(3);

fn hit(chain: ChainKey, epoch_id: EpochId, ctr: u64) -> Hit {
    Hit {
        chain,
        epoch_id,
        ctr,
    }
}

// ---- fixed vectors ----------------------------------------------------------

/// `seed = 00 01 .. 1f`; `k_tag = HKDF(seed, "", "meshapp.v1.recog.tag")`,
/// `k_hdr = HKDF(seed, "", "meshapp.v1.recog.hdr")`,
/// `tag(ctr) = prf16(k_tag, "tag", ctr)`,
/// `cell_key(ctr) = HKDF(k_hdr, "", "meshapp.v1.cell" ‖ LE64(ctr))`.
/// Computed once from the implementation, cross-checked with an independent
/// HKDF/HMAC (Python hashlib) and frozen here; `wire/cell.rs` asserts the
/// same `cell_key` values from `k_hdr`.
const VEC_K_TAG: &str = "e6247c7fc4082e59fdd965ac6509e026598ccb7270cb7a55b822e7b4f54acbe4";
const VEC_K_HDR: &str = "6c81058be75cb16d4d3c9846f07cf2d89a9c6469924376ada2297cf99f0e6f36";
const VEC_TAG_0: &str = "782b7071e54a7227c1a6570e60db1676";
const VEC_TAG_7: &str = "089df4860699dc056b8a9bb08bb5d68e";
const VEC_CELL_KEY_0: &str = "cff98b505f6144d3fec0839055002c5cf3774458eac41c93bba9ba3fa871a292";
const VEC_CELL_KEY_7: &str = "506e478cde341e37b8906f12072dbce851ecb612598f3c082b4ee0a7831e54f9";
const VEC_CELL_KEY_MAX: &str = "8f81ab2be91bd375a05c791ccf01cc4d66105cb46d3e9164d7f94a4ccb782a1e";

#[test]
fn fixed_vector_cell_key_from_seed() {
    let e = RecogEpoch::from_seed(&vector_seed());
    assert_eq!(hex::encode(e.k_tag().as_bytes()), VEC_K_TAG);
    assert_eq!(hex::encode(e.k_hdr().as_bytes()), VEC_K_HDR);
    assert_eq!(hex::encode(e.tag_for(0)), VEC_TAG_0);
    assert_eq!(hex::encode(e.tag_for(7)), VEC_TAG_7);
    assert_eq!(hex::encode(e.cell_key(0).as_bytes()), VEC_CELL_KEY_0);
    assert_eq!(hex::encode(e.cell_key(7).as_bytes()), VEC_CELL_KEY_7);
    assert_eq!(
        hex::encode(e.cell_key(u64::MAX).as_bytes()),
        VEC_CELL_KEY_MAX
    );
    // wire/cell.rs derives the same body key from k_hdr
    let k_hdr: [u8; 32] = hex::decode(VEC_K_HDR).unwrap().try_into().unwrap();
    assert_eq!(
        hex::encode(cell::cell_key(&k_hdr, 0).as_bytes()),
        VEC_CELL_KEY_0
    );
    assert_eq!(
        hex::encode(cell::cell_key(&k_hdr, 7).as_bytes()),
        VEC_CELL_KEY_7
    );
    assert_eq!(
        hex::encode(cell::cell_key(&k_hdr, u64::MAX).as_bytes()),
        VEC_CELL_KEY_MAX
    );
    // and the derivation is what the design says, spelled out
    let mut info = b"meshapp.v1.cell".to_vec();
    info.extend_from_slice(&7u64.to_le_bytes());
    assert_eq!(hex::encode(hkdf32(&k_hdr, b"", &info)), VEC_CELL_KEY_7);
    assert_eq!(
        hex::encode(hkdf32(&vector_seed(), b"", b"meshapp.v1.recog.hdr")),
        VEC_K_HDR
    );
    assert_eq!(
        hex::encode(hkdf32(&vector_seed(), b"", b"meshapp.v1.recog.tag")),
        VEC_K_TAG
    );
}

const VEC_ENV_PUB: &str = "8f40c5adb68f25624ae5b214ea767a6ec94d829d3d7b5e1ad1ba6f3e2138285f";
const VEC_E_PUB: &str = "358072d6365880d1aeea329adf9121383851ed21a28e3b75e965d0d2cd166254";
const VEC_SEAL_TAG: &str = "c6d0d53360aa72d890ddd76f71166c0c";

#[test]
fn fixed_vector_seal_tag() {
    let r = EnvelopeKeyPair::from_bytes(vector_seed());
    assert_eq!(hex::encode(r.public()), VEC_ENV_PUB);
    let e_pub = EnvelopeKeyPair::from_bytes(core::array::from_fn(|i| 0x20 + i as u8)).public();
    assert_eq!(hex::encode(e_pub), VEC_E_PUB);
    assert_eq!(hex::encode(seal_tag(&r.public(), &e_pub)), VEC_SEAL_TAG);
}

/// Recipient secret `00..1f` (`E_r` = [`VEC_ENV_PUB`]), ephemeral secret
/// `20..3f` (`e_pub` = [`VEC_E_PUB`]), payload `"first flight"`:
/// `shared = X25519(e_s, E_r)`, `k_seal = HKDF(shared, "", "meshapp.v1.seal"
/// ‖ e_pub ‖ E_r)`, `ct = ChaCha20-Poly1305(k_seal, nonce 0¹², aad = e_pub,
/// payload)`. Computed independently (Python `cryptography` X25519 and
/// ChaCha20-Poly1305, hashlib HKDF) and frozen; the platform ports assert
/// the same bytes, and `try_unseal` must open the frozen ciphertext.
const VEC_SEAL_SHARED: &str = "9663aa1da97e848a914a436d04163dfbb89178f107f1b5b77ed3854203382854";
const VEC_K_SEAL: &str = "dc85ef4a07b31212b819ca1a3a4cf4581f30d4e2f57a8af557b789e0ce33ba80";
const VEC_SEAL_CT: &str = "4cc49acecfe162b03a2c8be5c4fc2d18cb9a0c31e2d0b921f72d9e72";

#[test]
fn fixed_vector_seal_ciphertext() {
    let r = EnvelopeKeyPair::from_bytes(vector_seed());
    let e_pub: [u8; 32] = hex::decode(VEC_E_PUB).unwrap().try_into().unwrap();
    let ct = hex::decode(VEC_SEAL_CT).unwrap();
    assert_eq!(ct.len(), b"first flight".len() + 16);
    assert_eq!(
        try_unseal(std::slice::from_ref(&r), &e_pub, &ct),
        Some((0, b"first flight".to_vec()))
    );
    // the derivation, spelled out against the frozen intermediates
    let shared = x25519_dalek::x25519(r.to_bytes(), e_pub);
    assert_eq!(hex::encode(shared), VEC_SEAL_SHARED);
    let mut info = b"meshapp.v1.seal".to_vec();
    info.extend_from_slice(&e_pub);
    info.extend_from_slice(&r.public());
    let k_seal = Key32::from(hkdf32(&shared, b"", &info));
    assert_eq!(hex::encode(k_seal.as_bytes()), VEC_K_SEAL);
    assert_eq!(
        hex::encode(aead::seal(&k_seal, &[0u8; 12], &e_pub, b"first flight")),
        VEC_SEAL_CT
    );
    // a different recipient secret does not open it
    let other = EnvelopeKeyPair::from_bytes(core::array::from_fn(|i| 0x40 + i as u8));
    assert_eq!(try_unseal(&[other], &e_pub, &ct), None);
}

// ---- recog: tags ------------------------------------------------------------

#[test]
fn tags_are_deterministic_and_unique_across_counters_and_epochs() {
    let e = epoch(1);
    let f = epoch(2);
    let mut seen = std::collections::BTreeSet::new();
    for ctr in 0..1_000u64 {
        assert_eq!(e.tag_for(ctr), e.tag_for(ctr));
        assert!(seen.insert(e.tag_for(ctr)), "counter {ctr} repeats a tag");
        assert!(seen.insert(f.tag_for(ctr)), "epochs share a tag at {ctr}");
    }
    assert_eq!(seen.len(), 2_000);
    assert_ne!(e.cell_key(1), e.cell_key(2));
    assert_ne!(e.cell_key(1), f.cell_key(1));
    assert_eq!(e.cell_key(1), e.cell_key(1));
}

// ---- recog: window ----------------------------------------------------------

#[test]
fn window_lookup_consume_and_replay_rejection() {
    let mut t = TagTable::new();
    let e = epoch(1);
    t.register(A, E0, e.clone(), 0);
    assert_eq!(t.len(), WINDOW);
    // the window starts at the next expected counter
    assert_eq!(t.lookup(&e.tag_for(0)), Some(hit(A, E0, 0)));
    assert_eq!(
        t.lookup(&e.tag_for(WINDOW as u64 - 1)),
        Some(hit(A, E0, WINDOW as u64 - 1))
    );
    assert_eq!(t.lookup(&e.tag_for(WINDOW as u64)), None);
    // a foreign tag is unknown
    assert_eq!(t.lookup(&epoch(2).tag_for(0)), None);
    assert_eq!(t.lookup(&[0u8; TAG_LEN]), None);
    // consume, then the same tag never hits again
    let h = t.lookup(&e.tag_for(4)).unwrap();
    assert_eq!(t.consume(h), Ok(()));
    assert_eq!(t.lookup(&e.tag_for(4)), None);
    assert_eq!(t.consume(h), Err(RecogError::AlreadyUsed));
    assert_eq!(
        t.window(A, E0),
        Some(WindowState {
            base: 0,
            used: 1 << 4
        })
    );
    // the counters before it are still recognisable (reordering)
    for ctr in 0..4 {
        assert_eq!(t.lookup(&e.tag_for(ctr)), Some(hit(A, E0, ctr)));
    }
    // errors for things that are not in any window
    assert_eq!(t.consume(hit(B, E0, 0)), Err(RecogError::UnknownChain));
    assert_eq!(t.consume(hit(A, E1, 0)), Err(RecogError::UnknownEpoch));
    assert_eq!(t.consume(hit(A, E0, 10_000)), Err(RecogError::OutOfWindow));
}

#[test]
fn window_slides_and_stays_bounded() {
    let mut t = TagTable::new();
    let e = epoch(1);
    t.register(A, E0, e.clone(), 0);
    let mut delivered = 0u64;
    // 2 000 cells with mild reordering: every block of 8 arrives reversed
    for block in 0..250u64 {
        for ctr in (block * 8..block * 8 + 8).rev() {
            let h = t
                .lookup(&e.tag_for(ctr))
                .expect("inside the reordering tolerance");
            t.consume(h).unwrap();
            delivered += 1;
            assert!(t.len() <= WINDOW, "bounded: {}", t.len());
        }
        assert_eq!(
            t.window(A, E0),
            Some(WindowState {
                base: block * 8 + 8,
                used: 0
            })
        );
        assert_eq!(t.len(), WINDOW, "fully precomputed after each block");
    }
    assert_eq!(delivered, 2_000);
    // a far-ahead counter is outside the window until the window gets there
    assert_eq!(t.lookup(&e.tag_for(2_000 + WINDOW as u64)), None);
    // a hole (lost cell) is tolerated: the window follows the newest hit
    let hole = 2_000u64;
    for ctr in hole + 1..hole + 300 {
        let h = t
            .lookup(&e.tag_for(ctr))
            .expect("window followed the sender past the hole");
        t.consume(h).unwrap();
        let w = t.window(A, E0).unwrap();
        assert!(w.base + WINDOW as u64 > ctr + LOOKAHEAD as u64);
        assert!(t.len() <= WINDOW);
    }
    assert_eq!(t.lookup(&e.tag_for(hole)), None, "given up");
}

#[test]
fn three_epochs_are_retained_per_chain() {
    let mut t = TagTable::new();
    t.register(A, E0, epoch(0), 0);
    t.register(A, E1, epoch(1), 0);
    t.register(A, E2, epoch(2), 0);
    assert_eq!(t.len(), MAX_EPOCHS * WINDOW);
    for (id, n) in [(E0, 0u8), (E1, 1), (E2, 2)] {
        assert_eq!(t.lookup(&epoch(n).tag_for(3)), Some(hit(A, id, 3)));
    }
    // a fourth epoch evicts the oldest; the table does not grow
    t.register(A, E3, epoch(3), 0);
    assert_eq!(t.len(), MAX_EPOCHS * WINDOW);
    assert_eq!(t.lookup(&epoch(0).tag_for(3)), None);
    assert_eq!(t.lookup(&epoch(3).tag_for(3)), Some(hit(A, E3, 3)));
    assert_eq!(
        t.chain(A)
            .unwrap()
            .epochs()
            .map(|(id, _)| id)
            .collect::<Vec<_>>(),
        [E1, E2, E3]
    );
    // forgetting an epoch early (a confirmed rotation) frees its tags
    assert!(t.forget_epoch(A, E1));
    assert_eq!(t.len(), 2 * WINDOW);
    assert_eq!(t.lookup(&epoch(1).tag_for(3)), None);
    // several chains: the bound is chains × epochs × WINDOW
    t.register(B, E0, epoch(7), 0);
    t.register(B, E1, epoch(8), 0);
    t.register(B, E2, epoch(9), 0);
    assert_eq!(t.len(), 5 * WINDOW);
    assert!(t.len() <= t.chain_count() * MAX_EPOCHS * WINDOW);
}

#[test]
fn restore_from_persisted_window_state() {
    let mut t = TagTable::new();
    let e = epoch(1);
    t.register(A, E0, e.clone(), 100);
    for ctr in [100u64, 101, 103] {
        t.consume(t.lookup(&e.tag_for(ctr)).unwrap()).unwrap();
    }
    let saved = t.window(A, E0).unwrap();
    assert_eq!(
        saved,
        WindowState {
            base: 102,
            used: 0b10
        }
    );
    let mut again = TagTable::new();
    again.restore(A, E0, e.clone(), saved);
    assert_eq!(again.window(A, E0), Some(saved));
    assert_eq!(again.len(), WINDOW - 1);
    assert_eq!(
        again.lookup(&e.tag_for(103)),
        None,
        "consumed before the restart"
    );
    assert_eq!(again.lookup(&e.tag_for(102)), Some(hit(A, E0, 102)));
}

// ---- recog: sender / receiver agreement --------------------------------------

#[test]
fn sender_and_receiver_agree_on_counter_tag_and_cell_key() {
    let e = epoch(5);
    let mut sender = TagTable::new();
    sender.register_sender(A, E1, e.clone(), 0);
    let mut receiver = TagTable::new();
    receiver.register(B, E1, e.clone(), 0);
    let mut pt = [0u8; BODY_PLAINTEXT_LEN];
    for ctr in 0..120u64 {
        let issued = sender.sender_next(A).unwrap();
        assert_eq!(issued.ctr, ctr, "monotone");
        assert_eq!(issued.epoch_id, E1);
        pt[0] = ctr as u8;
        // the sender seals a real cell under its header key and counter
        let c = cell::seal_cell(
            VERSION,
            7,
            &issued.tag,
            e.k_hdr().as_bytes(),
            issued.ctr,
            0,
            &pt,
        );
        assert_eq!(
            issued.cell_key,
            cell::cell_key(e.k_hdr().as_bytes(), issued.ctr)
        );
        // the receiver recognises the clear tag, opens, then consumes
        let h = receiver.lookup(&c.tag()).expect("recognised");
        assert_eq!(h, hit(B, E1, ctr));
        assert_eq!(receiver.cell_key(&h), Some(issued.cell_key.clone()));
        let k_hdr = receiver.epoch(h.chain, h.epoch_id).unwrap().k_hdr().clone();
        assert_eq!(c.open_body(k_hdr.as_bytes(), h.ctr, 0), Ok(pt));
        receiver.consume(h).unwrap();
        // a relayed copy of the same cell is not recognised again
        assert_eq!(receiver.lookup(&c.forwarded().unwrap().tag()), None);
    }
    assert_eq!(sender.sender_next_ctr(A), Some(120));
    // the sender persists its counter and continues after a restart
    let mut restarted = TagTable::new();
    restarted.register_sender(A, E1, e.clone(), sender.sender_next_ctr(A).unwrap());
    let next = restarted.sender_next(A).unwrap();
    assert_eq!(next.ctr, 120);
    assert_eq!(receiver.lookup(&next.tag), Some(hit(B, E1, 120)));
}

#[test]
fn sender_epoch_change_is_a_new_registration_on_both_sides() {
    let e_old = epoch(1);
    let e_new = epoch(2);
    let mut sender = TagTable::new();
    sender.register_sender(A, E0, e_old.clone(), 0);
    let old_issue = sender.sender_next(A).unwrap();
    sender.register_sender(A, E1, e_new.clone(), 0);
    let new_issue = sender.sender_next(A).unwrap();
    assert_eq!(new_issue.ctr, 0);
    assert_ne!(old_issue.tag, new_issue.tag);
    let mut receiver = TagTable::new();
    receiver.register(A, E0, e_old, 0);
    receiver.register(A, E1, e_new, 0);
    assert_eq!(receiver.lookup(&old_issue.tag), Some(hit(A, E0, 0)));
    assert_eq!(receiver.lookup(&new_issue.tag), Some(hit(A, E1, 0)));
}

// ---- seal -------------------------------------------------------------------

#[test]
fn seal_unseal_round_trip_with_three_live_keys() {
    let keys = [pair(1), pair(2), pair(3)];
    assert_eq!(keys.len(), MAX_LIVE_KEYS);
    let payload = b"PreKeySignalMessage || recog_seed || envelope_pub";
    for (i, k) in keys.iter().enumerate() {
        let s = seal(&k.public(), payload, &mut rng(100 + i as u64)).unwrap();
        assert_eq!(s.ct.len(), payload.len() + OVERHEAD - 32);
        let obj = s.to_object();
        assert_eq!(obj.len(), payload.len() + OVERHEAD);
        let parsed = Sealed::from_object(&obj).unwrap();
        assert_eq!(parsed, s);
        assert_eq!(
            try_unseal(&keys, &parsed.e_pub, &parsed.ct),
            Some((i, payload.to_vec()))
        );
        assert_eq!(match_tag(&keys, &s.e_pub, &s.tag(&k.public())), Some(i));
        assert_eq!(s.tag(&k.public()), seal_tag(&k.public(), &s.e_pub));
    }
}

#[test]
fn seal_is_fresh_per_message_and_hides_nothing_reusable() {
    let r = pair(1);
    let a = seal(&r.public(), b"same", &mut rng(1)).unwrap();
    let b = seal(&r.public(), b"same", &mut rng(2)).unwrap();
    assert_ne!(a.e_pub, b.e_pub);
    assert_ne!(a.ct, b.ct);
    assert_ne!(a.tag(&r.public()), b.tag(&r.public()));
    // same rng state, same bytes: the only randomness is the ephemeral key
    assert_eq!(seal(&r.public(), b"same", &mut rng(1)).unwrap(), a);
}

#[test]
fn wrong_recipient_key_fails() {
    let r = pair(1);
    let others = [pair(2), pair(3), pair(4)];
    let s = seal(&r.public(), b"to r only", &mut rng(5)).unwrap();
    assert_eq!(try_unseal(&others, &s.e_pub, &s.ct), None);
    assert_eq!(match_tag(&others, &s.e_pub, &s.tag(&r.public())), None);
    // the same key pair rebuilt from its bytes still opens it
    let rebuilt = EnvelopeKeyPair::from_bytes(r.to_bytes());
    assert_eq!(
        try_unseal(&[rebuilt], &s.e_pub, &s.ct),
        Some((0, b"to r only".to_vec()))
    );
}

#[test]
fn tampered_ciphertext_fails() {
    let r = pair(1);
    let s = seal(&r.public(), b"integrity", &mut rng(6)).unwrap();
    let keys = [r];
    for i in 0..s.ct.len() {
        let mut bad = s.ct.clone();
        bad[i] ^= 0x01;
        assert_eq!(try_unseal(&keys, &s.e_pub, &bad), None, "byte {i}");
    }
    for i in 0..32 {
        let mut bad_e = s.e_pub;
        bad_e[i] ^= 0x01;
        assert_eq!(try_unseal(&keys, &bad_e, &s.ct), None, "e_pub byte {i}");
    }
    assert_eq!(try_unseal(&keys, &s.e_pub, &s.ct[..s.ct.len() - 1]), None);
    assert_eq!(try_unseal(&keys, &s.e_pub, &s.ct[1..]), None);
    assert_eq!(try_unseal(&keys, &s.e_pub, &[]), None);
    assert_eq!(
        try_unseal(&keys, &s.e_pub, &s.ct),
        Some((0, b"integrity".to_vec()))
    );
}

#[test]
fn low_order_recipient_key_is_rejected() {
    let zero = [0u8; 32];
    assert_eq!(
        seal(&zero, b"x", &mut rng(1)),
        Err(SealError::NonContributory)
    );
    let mut one = [0u8; 32];
    one[0] = 1;
    assert_eq!(
        seal(&one, b"x", &mut rng(1)),
        Err(SealError::NonContributory)
    );
    // and a low-order ephemeral key on the receiving side opens nothing
    let r = pair(1);
    let s = seal(&r.public(), b"x", &mut rng(1)).unwrap();
    assert_eq!(try_unseal(std::slice::from_ref(&r), &zero, &s.ct), None);
    // not even a forgery: a low-order e_pub forces the shared secret to all
    // zero, so anyone can compute k_seal and produce a ciphertext that
    // authenticates under the AEAD; only the contributory check rejects it
    for e_pub in [zero, one] {
        let mut info = b"meshapp.v1.seal".to_vec();
        info.extend_from_slice(&e_pub);
        info.extend_from_slice(&r.public());
        let forged_key = Key32::from(hkdf32(&[0u8; 32], b"", &info));
        let ct = aead::seal(&forged_key, &[0u8; 12], &e_pub, b"forged");
        assert_eq!(
            aead::open(&forged_key, &[0u8; 12], &e_pub, &ct).as_deref(),
            Ok(&b"forged"[..]),
            "the forgery is a valid AEAD"
        );
        assert_eq!(
            try_unseal(std::slice::from_ref(&r), &e_pub, &ct),
            None,
            "e_pub {}",
            hex::encode(e_pub)
        );
        assert_eq!(
            match_tag(
                std::slice::from_ref(&r),
                &e_pub,
                &seal_tag(&r.public(), &e_pub)
            ),
            Some(0)
        );
    }
}

#[test]
fn payload_too_long_is_an_error_not_a_panic() {
    let r = pair(1);
    assert_eq!(
        seal(&r.public(), &vec![0u8; MAX_PAYLOAD_LEN + 1], &mut rng(1)),
        Err(SealError::PayloadTooLong(MAX_PAYLOAD_LEN + 1))
    );
    assert!(seal(&r.public(), &vec![0u8; MAX_PAYLOAD_LEN], &mut rng(1)).is_ok());
}

// ---- parsers survive arbitrary bytes -----------------------------------------

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    #[test]
    fn unseal_survives_arbitrary_bytes(
        e_pub in any::<[u8; 32]>(),
        ct in prop::collection::vec(any::<u8>(), 0..200),
        tag in any::<[u8; 16]>(),
    ) {
        let keys = [pair(1), pair(2), pair(3)];
        prop_assert_eq!(try_unseal(&keys, &e_pub, &ct), None);
        prop_assert_eq!(match_tag(&keys, &e_pub, &tag), None);
        let _ = Sealed::from_object(&ct);
    }

    #[test]
    fn table_survives_arbitrary_tags_and_hits(
        tags in prop::collection::vec(any::<[u8; 16]>(), 0..50),
        hits in prop::collection::vec((any::<u64>(), any::<u32>(), any::<u64>()), 0..50),
    ) {
        let mut t = TagTable::new();
        t.register(A, E0, epoch(1), 0);
        t.register_sender(A, E0, epoch(1), 0);
        for tag in &tags {
            prop_assert_eq!(t.lookup(tag), None);
        }
        for (c, e, ctr) in hits {
            let _ = t.consume(hit(ChainKey(c), EpochId(e), ctr));
        }
        prop_assert!(t.len() <= WINDOW);
    }

    #[test]
    fn every_counter_round_trips_through_a_real_cell(ctr in any::<u64>(), fill in any::<u8>()) {
        let e = epoch(3);
        let mut t = TagTable::new();
        t.register(A, E0, e.clone(), ctr);
        let pt = [fill; BODY_PLAINTEXT_LEN];
        let c = cell::seal_cell(VERSION, 6, &e.tag_for(ctr), e.k_hdr().as_bytes(), ctr, 1, &pt);
        let h = t.lookup(&c.tag()).unwrap();
        prop_assert_eq!(h, hit(A, E0, ctr));
        let key: Key32 = t.cell_key(&h).unwrap();
        prop_assert_eq!(&key, &e.cell_key(ctr));
        prop_assert_eq!(c.open_body(e.k_hdr().as_bytes(), h.ctr, 1), Ok(pt));
        prop_assert_eq!(t.consume(h), Ok(()));
        prop_assert_eq!(t.lookup(&c.tag()), None);
    }
}
