//! Sealed first flights to a gossiped envelope key (docs/DESIGN.md §1 row
//! `wire/seal`, §2.2 `tag` for sealed first flights, §2.4 "Sealed first
//! flight"; DECISIONS.md D6 pick B (2) "strangers' first flights get an
//! outer one-way seal to a gossiped envelope key, with no key id, trial
//! decryption against ≤3 live keys", §3 row (4,5,6,7) "first messages are
//! sealed in both builds ... sealed first messages hide the sender's
//! identity key from relays"; BRIEF_v2 §5.5 "First messages are sealed").
//!
//! A first message carries a `PreKeySignalMessage`, which names the
//! sender's identity key. Before it goes on air the sender wraps it:
//!
//! * `e_s` is a fresh X25519 secret, `e_pub` its public key (sent clear at
//!   the start of the inner object, DESIGN §2.4 "`e_pub [32]` clear");
//! * `shared = X25519(e_s, E_r)` where `E_r` is the recipient's gossiped
//!   envelope public key; an all-zero result (a low-order `E_r` or `e_pub`)
//!   is rejected on both sides, so a relay cannot force a known key;
//! * `k_seal = HKDF-SHA256(ikm = shared, salt = "", info = "meshapp.v1.seal"
//!   ‖ e_pub ‖ E_r)`, binding the key to both public values;
//! * `ct = ChaCha20-Poly1305(k_seal, nonce = 0¹², aad = e_pub, payload)`; a
//!   zero nonce is safe because `k_seal` is used for exactly one message
//!   (fresh `e_s`);
//! * the cell tag of the flight is `Trunc16(SHA256("meshapp.v1.seal.tag" ‖
//!   E_r ‖ e_pub))` ([`seal_tag`]; DESIGN §2.2 offset 4). The tag names no
//!   key: a relay sees 16 bytes it cannot relate to any `E_r` without
//!   `e_pub`, which is inside the cell body.
//!
//! The recipient keeps its current envelope key and the two before it (D6
//! pick B (2) "≤3 live keys", retained 72 h of powered-on time by the
//! caller) and [`try_unseal`]s against each: at most three X25519
//! operations per unrecognised cell, which bounds what junk from a relay
//! can cost. A wrong key, a truncated or tampered ciphertext and a
//! low-order `e_pub` all give `None`; nothing here panics on input.
//!
//! The shared secret and `k_seal` live only for the call and are zeroized
//! when it returns; [`EnvelopeKeyPair`] zeroizes its secret on drop.
//! Nothing here reads a clock.
//!
//! # Sealed-first-flight cells
//!
//! A chain cell's tag is `prf16(k_tag, ctr)` and its body key is derived
//! from the chain, so a receiver recognises it by one table lookup. A first
//! flight has no chain yet, so its cells follow a different rule
//! ([`ff_tag`], [`ff_body_key`], [`ff_seal_cell`], [`ff_open_cell`]):
//!
//! * the cell tag is 16 uniformly random bytes ([`ff_tag`]): nothing a
//!   relay could precompute or relate to any envelope key (DESIGN §2.2's
//!   `Trunc16(SHA256("seal" ‖ E_r ‖ e_pub))` would let anyone who knows a
//!   gossiped `E_r` and reads `e_pub` from the body link the flight to its
//!   recipient; the random tag does not, and the inner [`seal`] already
//!   binds `e_pub` and `E_r` under the AEAD);
//! * the body is ChaCha20-Poly1305 under `ff_body_key(E_r) = HKDF-SHA256(ikm
//!   = E_r, salt = "", info = "meshapp.v1.ff.body")`, nonce = the first 12
//!   bytes of the random tag (fresh per cell: the key is shared by every
//!   flight to `E_r`, so the nonce must not repeat; 96 random bits across
//!   the few thousand cells one envelope key ever receives is far below the
//!   birthday bound), AAD = the clear header without the TTL, like a chain
//!   cell. `E_r` travels only inside group encryption or on a direct
//!   encounter (DECISIONS §3 row (4,5,6,7)), so only envelope-key holders,
//!   not relays, can open the outer layer and see that a cell is a first
//!   flight at all;
//! * a receiver trial-opens every unrecognised cell under its ≤
//!   [`MAX_LIVE_KEYS`] live `ff_body_key`s (one AEAD each, cheap), then
//!   reassembles the [`crate::wire::inner::InnerKind::SealedFirst`] object
//!   and [`try_unseal`]s the inner payload against the same keys.

use core::fmt;

use rand::CryptoRng;
use x25519_dalek::{PublicKey, StaticSecret};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::crypto::aead::{self, Key32, NONCE_LEN, TAG_LEN};
use crate::crypto::hash::hkdf32;
use crate::crypto::hash::sha256_concat;
use crate::wire::cell::{self, BODY_LEN, BODY_PLAINTEXT_LEN, Cell, HeaderAad};
use crate::wire::inner::{E_PUB_LEN, MAX_OBJECT_LEN};

/// Length of an X25519 key, public or secret.
pub const KEY_LEN: usize = 32;
/// Envelope keys a recipient keeps live (the current one and two before).
pub const MAX_LIVE_KEYS: usize = 3;
/// HKDF `info` prefix of `k_seal`, followed by `e_pub ‖ E_r`.
pub const SEAL_INFO: &[u8] = b"meshapp.v1.seal";
/// Domain prefix of the sealed-flight cell tag.
pub const SEAL_TAG_DOMAIN: &[u8] = b"meshapp.v1.seal.tag";
/// The AEAD nonce: constant, because every `k_seal` is single-use.
pub const SEAL_NONCE: [u8; NONCE_LEN] = [0u8; NONCE_LEN];
/// Bytes a seal adds to its payload: the clear `e_pub` and the AEAD tag.
pub const OVERHEAD: usize = E_PUB_LEN + TAG_LEN;
/// Largest payload a sealed first flight can carry, so that `e_pub ‖ ct`
/// still fits the fragment layer's largest object (DESIGN §2.2 "≤ 141
/// fragments").
pub const MAX_PAYLOAD_LEN: usize = MAX_OBJECT_LEN - OVERHEAD;
/// HKDF `info` of the first-flight cell body key (module docs).
pub const FF_BODY_INFO: &[u8] = b"meshapp.v1.ff.body";

/// An envelope key pair. The secret is wiped on drop and never printed.
#[derive(Clone)]
pub struct EnvelopeKeyPair {
    secret: StaticSecret,
    public: [u8; KEY_LEN],
}

/// The only secret field, `StaticSecret`, zeroizes itself on drop
/// (x25519-dalek's `ZeroizeOnDrop`); the public key is not secret.
impl ZeroizeOnDrop for EnvelopeKeyPair {}

impl fmt::Debug for EnvelopeKeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("EnvelopeKeyPair(pub=")?;
        for b in &self.public {
            write!(f, "{b:02x}")?;
        }
        f.write_str(")")
    }
}

impl EnvelopeKeyPair {
    /// A fresh key pair from 32 random bytes.
    pub fn generate(rng: &mut dyn CryptoRng) -> Self {
        let mut bytes = [0u8; KEY_LEN];
        rng.fill_bytes(&mut bytes);
        let pair = Self::from_bytes(bytes);
        bytes.zeroize();
        pair
    }

    /// Rebuilds a pair from its persisted secret ([`to_bytes`](Self::to_bytes)).
    /// Any 32 bytes are a valid secret (X25519 clamps on use); the caller
    /// should zeroize its own copy.
    pub fn from_bytes(secret: [u8; KEY_LEN]) -> Self {
        let secret = StaticSecret::from(secret);
        let public = PublicKey::from(&secret).to_bytes();
        EnvelopeKeyPair { secret, public }
    }

    /// The public key to gossip as `E_r`.
    pub fn public(&self) -> [u8; KEY_LEN] {
        self.public
    }

    /// The secret, for the encrypted store (BRIEF_v2 §5.3). Zeroize the copy.
    pub fn to_bytes(&self) -> [u8; KEY_LEN] {
        self.secret.to_bytes()
    }

    /// `X25519(secret, their_pub)`, or `None` when the result is the
    /// identity (a low-order `their_pub`), in constant time.
    fn shared(&self, their_pub: &[u8; KEY_LEN]) -> Option<Shared> {
        let shared = self.secret.diffie_hellman(&PublicKey::from(*their_pub));
        shared.was_contributory().then(|| Shared(shared.to_bytes()))
    }
}

/// A shared secret that is wiped on drop.
#[derive(Zeroize, ZeroizeOnDrop)]
struct Shared([u8; KEY_LEN]);

/// A sealed first flight: the clear ephemeral key and the ciphertext,
/// which together form the `InnerKind::SealedFirst` object body
/// (DESIGN §2.4 "`e_pub [32]` clear ‖ AEAD(k_seal, ...)").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sealed {
    pub e_pub: [u8; E_PUB_LEN],
    pub ct: Vec<u8>,
}

impl Sealed {
    /// `e_pub ‖ ct`: the bytes handed to the fragmenter.
    pub fn to_object(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(E_PUB_LEN + self.ct.len());
        out.extend_from_slice(&self.e_pub);
        out.extend_from_slice(&self.ct);
        out
    }

    /// Splits a reassembled object into `e_pub` and `ct`. `None` when it is
    /// shorter than `e_pub` plus an AEAD tag, which can never open.
    pub fn from_object(object: &[u8]) -> Option<Self> {
        let (e_pub, ct) = crate::wire::inner::sealed_first_split(object)?;
        if ct.len() < TAG_LEN {
            return None;
        }
        Some(Sealed {
            e_pub: *e_pub,
            ct: ct.to_vec(),
        })
    }

    /// The cell tag of this flight to `recipient_pub` ([`seal_tag`]).
    pub fn tag(&self, recipient_pub: &[u8; KEY_LEN]) -> [u8; cell::TAG_LEN] {
        seal_tag(recipient_pub, &self.e_pub)
    }
}

/// Why a payload could not be sealed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum SealError {
    /// The recipient key is low-order: the shared secret would be the
    /// identity, so anyone could derive `k_seal`.
    #[error("recipient envelope key is low-order (non-contributory)")]
    NonContributory,
    #[error("payload of {0} bytes exceeds the sealed-flight maximum of {MAX_PAYLOAD_LEN}")]
    PayloadTooLong(usize),
}

/// `HKDF-SHA256(ikm = shared, salt = "", info = SEAL_INFO ‖ e_pub ‖ E_r)`.
fn seal_key(shared: &Shared, e_pub: &[u8; KEY_LEN], recipient_pub: &[u8; KEY_LEN]) -> Key32 {
    let mut info = [0u8; SEAL_INFO.len() + 2 * KEY_LEN];
    info[..SEAL_INFO.len()].copy_from_slice(SEAL_INFO);
    info[SEAL_INFO.len()..SEAL_INFO.len() + KEY_LEN].copy_from_slice(e_pub);
    info[SEAL_INFO.len() + KEY_LEN..].copy_from_slice(recipient_pub);
    Key32::from(hkdf32(&shared.0, b"", &info))
}

/// Seals `payload` to the recipient's envelope key `recipient_pub` (module
/// docs). Deterministic for a given RNG state; the only randomness is the
/// ephemeral key.
pub fn seal(
    recipient_pub: &[u8; KEY_LEN],
    payload: &[u8],
    rng: &mut dyn CryptoRng,
) -> Result<Sealed, SealError> {
    if payload.len() > MAX_PAYLOAD_LEN {
        return Err(SealError::PayloadTooLong(payload.len()));
    }
    let ephemeral = EnvelopeKeyPair::generate(rng);
    let e_pub = ephemeral.public();
    let shared = ephemeral
        .shared(recipient_pub)
        .ok_or(SealError::NonContributory)?;
    let k_seal = seal_key(&shared, &e_pub, recipient_pub);
    let ct = aead::seal(&k_seal, &SEAL_NONCE, &e_pub, payload);
    Ok(Sealed { e_pub, ct })
}

/// `Trunc16(SHA256("meshapp.v1.seal.tag" ‖ E_r ‖ e_pub))`: the clear cell
/// tag of a sealed first flight (DESIGN §2.2 offset 4).
pub fn seal_tag(recipient_pub: &[u8; KEY_LEN], e_pub: &[u8; KEY_LEN]) -> [u8; cell::TAG_LEN] {
    sha256_concat(&[SEAL_TAG_DOMAIN, recipient_pub, e_pub]).truncate16()
}

/// Which of the recipient's live keys a flight with clear `e_pub` and cell
/// tag `tag` was addressed to, by recomputing [`seal_tag`] for each. A
/// cheap check before [`try_unseal`]; `None` when the tag matches none,
/// which includes any tag of a chain cell.
pub fn match_tag(
    live_keys: &[EnvelopeKeyPair],
    e_pub: &[u8; KEY_LEN],
    tag: &[u8; cell::TAG_LEN],
) -> Option<usize> {
    live_keys
        .iter()
        .take(MAX_LIVE_KEYS)
        .position(|k| seal_tag(&k.public(), e_pub) == *tag)
}

/// Trial-decrypts `ct` (sealed under some live key with the clear `e_pub`)
/// against each of the recipient's live keys, current first, and returns
/// the index of the key that opened it with the payload. Only the first
/// [`MAX_LIVE_KEYS`] keys are tried. `None` for every failure: no key
/// matches, a tampered or truncated `ct`, or a low-order `e_pub`.
pub fn try_unseal(
    live_keys: &[EnvelopeKeyPair],
    e_pub: &[u8; KEY_LEN],
    ct: &[u8],
) -> Option<(usize, Vec<u8>)> {
    if ct.len() < TAG_LEN {
        return None;
    }
    for (i, key) in live_keys.iter().take(MAX_LIVE_KEYS).enumerate() {
        let Some(shared) = key.shared(e_pub) else {
            continue;
        };
        let k_seal = seal_key(&shared, e_pub, &key.public());
        if let Ok(pt) = aead::open(&k_seal, &SEAL_NONCE, e_pub, ct) {
            return Some((i, pt));
        }
    }
    None
}

// ---- sealed-first-flight cells ----------------------------------------------

/// `HKDF-SHA256(ikm = E_r, salt = "", info = "meshapp.v1.ff.body")`: the
/// body key of every first-flight cell addressed to the envelope key `E_r`
/// (module docs "Sealed-first-flight cells").
pub fn ff_body_key(recipient_envelope_pub: &[u8; KEY_LEN]) -> Key32 {
    Key32::from(hkdf32(recipient_envelope_pub, b"", FF_BODY_INFO))
}

/// A fresh uniformly random cell tag for a first-flight cell: not
/// precomputable by anyone, so a relay cannot tell first flights from chain
/// cells or group them by recipient.
pub fn ff_tag(rng: &mut dyn CryptoRng) -> [u8; cell::TAG_LEN] {
    let mut tag = [0u8; cell::TAG_LEN];
    rng.fill_bytes(&mut tag);
    tag
}

/// The body nonce of a first-flight cell: the first [`NONCE_LEN`] bytes of
/// its random tag.
pub fn ff_nonce(tag: &[u8; cell::TAG_LEN]) -> [u8; NONCE_LEN] {
    let mut nonce = [0u8; NONCE_LEN];
    nonce.copy_from_slice(&tag[..NONCE_LEN]);
    nonce
}

/// Builds one first-flight cell: header `{ver, ttl, 0, 0, ff_tag}` and the
/// body of `plaintext` (an inner fragment of a
/// [`crate::wire::inner::InnerKind::SealedFirst`] object) under
/// [`ff_body_key`]`(E_r)` with nonce [`ff_nonce`] and AAD = the header
/// without its TTL. Deterministic for a given RNG state.
pub fn ff_seal_cell(
    ver: u8,
    ttl: u8,
    recipient_envelope_pub: &[u8; KEY_LEN],
    plaintext: &[u8; BODY_PLAINTEXT_LEN],
    rng: &mut dyn CryptoRng,
) -> Cell {
    let tag = ff_tag(rng);
    let header = HeaderAad::new(ver, tag);
    let key = ff_body_key(recipient_envelope_pub);
    let ct = aead::seal(&key, &ff_nonce(&tag), &header.to_bytes(), plaintext);
    let mut body = [0u8; BODY_LEN];
    body.copy_from_slice(&ct);
    cell::build(ver, ttl, &tag, &body)
}

/// Trial-opens an unrecognised cell under the body keys of the receiver's
/// live envelope keys, current first (at most [`MAX_LIVE_KEYS`] are tried).
/// Returns the index of the key that opened it and the body plaintext, or
/// `None` when the cell is not a first flight to any of them (which
/// includes every chain cell, junk, and a tampered body). Each attempt is
/// one AEAD open; nothing here panics on input.
pub fn ff_open_cell(
    live_keys: &[EnvelopeKeyPair],
    cell: &Cell,
) -> Option<(usize, [u8; BODY_PLAINTEXT_LEN])> {
    let tag = cell.tag();
    let nonce = ff_nonce(&tag);
    let aad = cell.aad_bytes();
    for (i, key) in live_keys.iter().take(MAX_LIVE_KEYS).enumerate() {
        let k = ff_body_key(&key.public());
        if let Ok(pt) = aead::open(&k, &nonce, &aad, cell.body()) {
            let arr: [u8; BODY_PLAINTEXT_LEN] = pt.as_slice().try_into().ok()?;
            return Some((i, arr));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    fn rng(seed: u64) -> ChaCha20Rng {
        ChaCha20Rng::seed_from_u64(seed)
    }

    fn pair(seed: u64) -> EnvelopeKeyPair {
        EnvelopeKeyPair::generate(&mut rng(seed))
    }

    /// Low-order points of Curve25519 in the u-coordinate encoding (RFC 7748
    /// §6.1 lists the all-zero and one; the others are the 8-torsion points).
    const LOW_ORDER: [[u8; 32]; 4] = [
        [0u8; 32],
        {
            let mut p = [0u8; 32];
            p[0] = 1;
            p
        },
        [
            0xe0, 0xeb, 0x7a, 0x7c, 0x3b, 0x41, 0xb8, 0xae, 0x16, 0x56, 0xe3, 0xfa, 0xf1, 0x9f,
            0xc4, 0x6a, 0xda, 0x09, 0x8d, 0xeb, 0x9c, 0x32, 0xb1, 0xfd, 0x86, 0x62, 0x05, 0x16,
            0x5f, 0x49, 0xb8, 0x00,
        ],
        [
            0x5f, 0x9c, 0x95, 0xbc, 0xa3, 0x50, 0x8c, 0x24, 0xb1, 0xd0, 0xb1, 0x55, 0x9c, 0x83,
            0xef, 0x5b, 0x04, 0x44, 0x5c, 0xc4, 0x58, 0x1c, 0x8e, 0x86, 0xd8, 0x22, 0x4e, 0xdd,
            0xd0, 0x9f, 0x11, 0x57,
        ],
    ];

    #[test]
    fn key_pair_round_trips_and_is_redacted() {
        let k = pair(1);
        let again = EnvelopeKeyPair::from_bytes(k.to_bytes());
        assert_eq!(again.public(), k.public());
        assert_eq!(again.to_bytes(), k.to_bytes());
        assert_eq!(
            k.public(),
            x25519_dalek::x25519(k.to_bytes(), x25519_dalek::X25519_BASEPOINT_BYTES)
        );
        let dbg = format!("{k:?}");
        assert!(dbg.starts_with("EnvelopeKeyPair(pub="));
        assert!(!dbg.contains(&hex::encode(k.to_bytes())));
        assert_ne!(pair(1).public(), pair(2).public());
        assert_eq!(pair(1).to_bytes(), pair(1).to_bytes(), "seeded rng");
    }

    #[test]
    fn seal_follows_the_module_docs() {
        let r = pair(1);
        let s = seal(&r.public(), b"first flight", &mut rng(7)).unwrap();
        assert_eq!(s.ct.len(), b"first flight".len() + TAG_LEN);
        // recompute k_seal by hand on the recipient side
        let shared = x25519_dalek::x25519(r.to_bytes(), s.e_pub);
        let mut info = b"meshapp.v1.seal".to_vec();
        info.extend_from_slice(&s.e_pub);
        info.extend_from_slice(&r.public());
        let k = Key32::from(hkdf32(&shared, b"", &info));
        assert_eq!(
            aead::open(&k, &[0u8; 12], &s.e_pub, &s.ct).unwrap(),
            b"first flight"
        );
        // deterministic for a seeded rng, fresh otherwise
        assert_eq!(seal(&r.public(), b"first flight", &mut rng(7)).unwrap(), s);
        assert_ne!(
            seal(&r.public(), b"first flight", &mut rng(8))
                .unwrap()
                .e_pub,
            s.e_pub
        );
    }

    #[test]
    fn unseal_round_trip_and_key_index() {
        let keys = [pair(1), pair(2), pair(3)];
        for (i, k) in keys.iter().enumerate() {
            let s = seal(&k.public(), b"hello", &mut rng(10 + i as u64)).unwrap();
            assert_eq!(
                try_unseal(&keys, &s.e_pub, &s.ct),
                Some((i, b"hello".to_vec()))
            );
            assert_eq!(match_tag(&keys, &s.e_pub, &s.tag(&k.public())), Some(i));
        }
        let s = seal(&keys[2].public(), b"", &mut rng(3)).unwrap();
        assert_eq!(try_unseal(&keys, &s.e_pub, &s.ct), Some((2, Vec::new())));
    }

    #[test]
    fn wrong_key_tamper_and_truncation_fail() {
        let r = pair(1);
        let other = [pair(5), pair(6)];
        let s = seal(&r.public(), b"secret", &mut rng(2)).unwrap();
        assert_eq!(try_unseal(&other, &s.e_pub, &s.ct), None);
        assert_eq!(try_unseal(&[], &s.e_pub, &s.ct), None);
        let mut bad = s.ct.clone();
        bad[0] ^= 1;
        assert_eq!(try_unseal(std::slice::from_ref(&r), &s.e_pub, &bad), None);
        let mut bad_e = s.e_pub;
        bad_e[31] ^= 0x40;
        assert_eq!(try_unseal(std::slice::from_ref(&r), &bad_e, &s.ct), None);
        assert_eq!(
            try_unseal(std::slice::from_ref(&r), &s.e_pub, &s.ct[..s.ct.len() - 1]),
            None
        );
        assert_eq!(try_unseal(std::slice::from_ref(&r), &s.e_pub, &[]), None);
        assert_eq!(match_tag(&other, &s.e_pub, &s.tag(&r.public())), None);
        assert_eq!(
            try_unseal(std::slice::from_ref(&r), &s.e_pub, &s.ct),
            Some((0, b"secret".to_vec()))
        );
    }

    #[test]
    fn low_order_keys_are_rejected_on_both_sides() {
        for p in LOW_ORDER {
            assert_eq!(
                seal(&p, b"x", &mut rng(1)),
                Err(SealError::NonContributory),
                "recipient {}",
                hex::encode(p)
            );
            let r = pair(1);
            let s = seal(&r.public(), b"x", &mut rng(1)).unwrap();
            assert_eq!(
                try_unseal(&[r], &p, &s.ct),
                None,
                "e_pub {}",
                hex::encode(p)
            );
        }
    }

    /// A low-order `e_pub` forces the shared secret to the identity (all
    /// zero), so anyone can derive `k_seal` and forge a ciphertext that
    /// authenticates under the AEAD. Only the receive-side contributory
    /// check rejects it; this test fails if that check is removed from
    /// [`try_unseal`], which the wrong-key tests above cannot detect.
    #[test]
    fn forged_ciphertext_under_a_low_order_e_pub_is_rejected() {
        let r = pair(1);
        for e_pub in LOW_ORDER {
            assert!(r.shared(&e_pub).is_none(), "e_pub {}", hex::encode(e_pub));
            let forged_key = seal_key(&Shared([0u8; KEY_LEN]), &e_pub, &r.public());
            let ct = aead::seal(&forged_key, &SEAL_NONCE, &e_pub, b"forged");
            // the forgery is a valid AEAD under the key anyone can compute
            assert_eq!(
                aead::open(&forged_key, &SEAL_NONCE, &e_pub, &ct).as_deref(),
                Ok(&b"forged"[..])
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
                Some(0),
                "the tag alone cannot tell a forgery apart; try_unseal must"
            );
        }
    }

    #[test]
    fn payload_bound() {
        let r = pair(1);
        let big = vec![0u8; MAX_PAYLOAD_LEN + 1];
        assert_eq!(
            seal(&r.public(), &big, &mut rng(1)),
            Err(SealError::PayloadTooLong(MAX_PAYLOAD_LEN + 1))
        );
        let s = seal(&r.public(), &big[..MAX_PAYLOAD_LEN], &mut rng(1)).unwrap();
        assert_eq!(s.to_object().len(), MAX_OBJECT_LEN);
        assert_eq!(
            try_unseal(&[r], &s.e_pub, &s.ct).map(|(i, pt)| (i, pt.len())),
            Some((0, MAX_PAYLOAD_LEN))
        );
    }

    #[test]
    fn seal_tag_layout_and_distinctness() {
        let r = pair(1);
        let e = pair(2).public();
        let mut bytes = b"meshapp.v1.seal.tag".to_vec();
        bytes.extend_from_slice(&r.public());
        bytes.extend_from_slice(&e);
        assert_eq!(
            seal_tag(&r.public(), &e),
            crate::crypto::hash::sha256(&bytes).truncate16()
        );
        assert_ne!(seal_tag(&r.public(), &e), seal_tag(&e, &r.public()));
        assert_ne!(seal_tag(&r.public(), &e), seal_tag(&pair(3).public(), &e));
    }

    #[test]
    fn object_split_round_trip() {
        let r = pair(1);
        let s = seal(&r.public(), b"abc", &mut rng(4)).unwrap();
        let obj = s.to_object();
        assert_eq!(obj.len(), 3 + OVERHEAD);
        assert_eq!(Sealed::from_object(&obj), Some(s.clone()));
        assert_eq!(Sealed::from_object(&obj[..E_PUB_LEN + TAG_LEN - 1]), None);
        assert_eq!(Sealed::from_object(&[]), None);
        assert!(Sealed::from_object(&obj[..E_PUB_LEN + TAG_LEN]).is_some());
    }

    #[test]
    fn only_the_first_three_keys_are_tried() {
        let keys = [pair(1), pair(2), pair(3), pair(4)];
        let s = seal(&keys[3].public(), b"x", &mut rng(1)).unwrap();
        assert_eq!(try_unseal(&keys, &s.e_pub, &s.ct), None);
        assert_eq!(match_tag(&keys, &s.e_pub, &s.tag(&keys[3].public())), None);
        let s2 = seal(&keys[2].public(), b"x", &mut rng(1)).unwrap();
        assert_eq!(
            try_unseal(&keys, &s2.e_pub, &s2.ct),
            Some((2, b"x".to_vec()))
        );
    }

    #[test]
    fn ff_body_key_is_the_documented_hkdf_and_tags_are_random() {
        let r = pair(1);
        assert_eq!(
            ff_body_key(&r.public()),
            Key32::from(hkdf32(&r.public(), b"", b"meshapp.v1.ff.body"))
        );
        assert_ne!(ff_body_key(&r.public()), ff_body_key(&pair(2).public()));
        let t1 = ff_tag(&mut rng(1));
        let t2 = ff_tag(&mut rng(2));
        assert_ne!(t1, t2);
        assert_eq!(t1, ff_tag(&mut rng(1)), "seeded rng");
        assert_eq!(&ff_nonce(&t1)[..], &t1[..12]);
    }

    #[test]
    fn ff_cell_round_trip_and_rejections() {
        let keys = [pair(1), pair(2), pair(3)];
        let mut pt = [0u8; BODY_PLAINTEXT_LEN];
        pt[0] = 0x03;
        pt[11] = 0xaa;
        for (i, k) in keys.iter().enumerate() {
            let c = ff_seal_cell(cell::VERSION, 7, &k.public(), &pt, &mut rng(10 + i as u64));
            assert_eq!(c.validate(), Ok(()));
            assert_eq!(ff_open_cell(&keys, &c), Some((i, pt)));
            // a relay's TTL decrement does not break it
            let fwd = c.forwarded().unwrap();
            assert_eq!(ff_open_cell(&keys, &fwd), Some((i, pt)));
            // the tag is not derived from anything: two flights differ
            let c2 = ff_seal_cell(cell::VERSION, 7, &k.public(), &pt, &mut rng(99));
            assert_ne!(c.tag(), c2.tag());
        }
        let c = ff_seal_cell(cell::VERSION, 6, &keys[0].public(), &pt, &mut rng(5));
        // wrong keys, tampered tag, tampered body
        assert_eq!(ff_open_cell(&[pair(8), pair(9)], &c), None);
        assert_eq!(ff_open_cell(&[], &c), None);
        let mut bytes = c.into_array();
        bytes[cell::OFF_TAG] ^= 1;
        assert_eq!(ff_open_cell(&keys, &Cell::from_array(bytes)), None);
        bytes[cell::OFF_TAG] ^= 1;
        bytes[cell::OFF_BODY + 3] ^= 1;
        assert_eq!(ff_open_cell(&keys, &Cell::from_array(bytes)), None);
        // a chain cell is never mistaken for a first flight
        let chain = cell::seal_cell(cell::VERSION, 7, &[1u8; 16], &[2u8; 32], 0, 0, &pt);
        assert_eq!(ff_open_cell(&keys, &chain), None);
        // only the first three live keys are tried
        let four = [pair(1), pair(2), pair(3), pair(4)];
        let c4 = ff_seal_cell(cell::VERSION, 7, &four[3].public(), &pt, &mut rng(1));
        assert_eq!(ff_open_cell(&four, &c4), None);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(128))]

        #[test]
        fn arbitrary_inputs_never_panic(
            e_pub in any::<[u8; 32]>(),
            ct in prop::collection::vec(any::<u8>(), 0..80),
            secret in any::<[u8; 32]>(),
            tag in any::<[u8; 16]>(),
        ) {
            let keys = [EnvelopeKeyPair::from_bytes(secret), pair(1)];
            prop_assert_eq!(try_unseal(&keys, &e_pub, &ct), None);
            let _ = match_tag(&keys, &e_pub, &tag);
            let _ = Sealed::from_object(&ct);
            let _ = seal_tag(&e_pub, &secret);
            let mut raw = [0u8; cell::CELL_LEN];
            raw[..32].copy_from_slice(&e_pub);
            raw[32..64].copy_from_slice(&secret);
            prop_assert_eq!(ff_open_cell(&keys, &Cell::from_array(raw)), None);
        }

        #[test]
        fn every_secret_round_trips_unless_low_order(secret in any::<[u8; 32]>(), msg in prop::collection::vec(any::<u8>(), 0..64)) {
            let k = EnvelopeKeyPair::from_bytes(secret);
            match seal(&k.public(), &msg, &mut rng(9)) {
                Ok(s) => prop_assert_eq!(try_unseal(std::slice::from_ref(&k), &s.e_pub, &s.ct), Some((0, msg))),
                Err(SealError::NonContributory) => prop_assert!(LOW_ORDER.contains(&k.public())),
                Err(e) => prop_assert!(false, "{e}"),
            }
        }
    }
}
