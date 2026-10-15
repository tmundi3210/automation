//! One strict Ed25519 verifier for both OSes (docs/DESIGN.md §1 row
//! `crypto/ed25519`; BRIEF_v2 §5.1 last bullet: "One strict signature
//! verifier behaves identically on iOS and Android. Non-canonical and
//! small-order keys are rejected when admitted."; D1 rule 2; §3-22).
//!
//! Two rules apply, in this order, wherever an identity key or signature
//! enters the core:
//!
//! 1. **Admission** ([`admit_public_key`]). Every 32-byte key that enters a
//!    KeyCard, roster op, ballot or vote-open passes here *before any
//!    signature check* (DESIGN §2.4 "Admission"). Rejected, in this order:
//!    * [`KeyReject::NotOnCurve`]: the y coordinate is not on the curve.
//!    * [`KeyReject::NonCanonical`]: the bytes are not the unique canonical
//!      encoding of the point: y ≥ p, or the sign bit is set while x = 0.
//!      `curve25519-dalek` (and so `ed25519-dalek::VerifyingKey::from_bytes`)
//!      silently reduces such encodings, so without this check two different
//!      byte strings would name one key. We decompress and recompress and
//!      require equality with the input.
//!    * [`KeyReject::SmallOrder`]: the point has order ≤ 8
//!      (`VerifyingKey::is_weak`). For such a key there are signatures that a
//!      lax verifier accepts for every message.
//! 2. **Verification** ([`verify`]). The key is re-admitted (so a key stored
//!    before a rule change is re-checked), then `VerifyingKey::verify_strict`
//!    of `ed25519-dalek` 3 is applied. It rejects `S ≥ L` (a non-canonical
//!    scalar), an `R` that is not on the curve or has small order, and a weak
//!    `A`. The signed message is `domain ‖ 0x00 ‖ msg` (DESIGN §2.1 "all
//!    identity signatures use domain separation").
//!
//!    Two library facts this rule leans on, both pinned by tests: the strict
//!    small-order-`R` check is the *only* observable difference from the
//!    crate's non-strict `verify` once `A` is admitted (test
//!    `small_order_r_is_rejected_even_when_the_equation_holds` and the table
//!    case `sig_r_identity_s_consistent`), and the `S < L` check is weakened
//!    to `S < 2^253` by the `legacy_compatibility` cargo feature of
//!    `ed25519-dalek`, which feature unification would switch on for the
//!    whole build if any dependency enabled it (test
//!    `non_canonical_s_is_rejected` and the table case `sig_s_plus_l` fail
//!    the moment that happens).
//!
//! The Kotlin and Swift apps never verify anything themselves; they call this
//! crate. `tests/vectors/ed25519_strict.json` is the verdict table that the
//! JVM smoke test re-runs (DESIGN §10 "ed25519 strict"), so any change to the
//! rules above must change that table too.
//!
//! The private half ([`IdentitySigner`]) is built from a 32-byte seed, is
//! wiped on drop and never printed. Signing is deterministic (RFC 8032): the
//! same seed and message always give the same signature, so nothing here
//! needs randomness or a clock.

use core::fmt;

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use zeroize::{ZeroizeOnDrop, Zeroizing};

use super::hash::is_valid_domain;

/// Ed25519 public key length (DESIGN §2.1 "keys 32 B").
pub const PUBLIC_KEY_LEN: usize = 32;
/// Seed length of [`IdentitySigner::from_seed`].
pub const SEED_LEN: usize = 32;
/// Ed25519 signature length (DESIGN §2.1 "signatures 64 B").
pub const SIGNATURE_LEN: usize = 64;

/// An *admitted* Ed25519 public key: the only way to build one from bytes is
/// [`admit_public_key`] (or `TryFrom`, which calls it), so holding an
/// `IdentityKey` means the key passed the admission rules at least once.
///
/// `PeerId` in DESIGN §1 is this type. Ordered byte-lexicographically so
/// lists of keys can be sorted (DESIGN §2.4 `admins [32]×n (sorted)`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdentityKey([u8; PUBLIC_KEY_LEN]);

impl IdentityKey {
    pub const fn as_bytes(&self) -> &[u8; PUBLIC_KEY_LEN] {
        &self.0
    }

    pub const fn to_bytes(self) -> [u8; PUBLIC_KEY_LEN] {
        self.0
    }
}

impl TryFrom<[u8; PUBLIC_KEY_LEN]> for IdentityKey {
    type Error = KeyReject;

    fn try_from(bytes: [u8; PUBLIC_KEY_LEN]) -> Result<Self, KeyReject> {
        admit_public_key(&bytes)
    }
}

impl TryFrom<&[u8; PUBLIC_KEY_LEN]> for IdentityKey {
    type Error = KeyReject;

    fn try_from(bytes: &[u8; PUBLIC_KEY_LEN]) -> Result<Self, KeyReject> {
        admit_public_key(bytes)
    }
}

impl AsRef<[u8]> for IdentityKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

fn write_hex(f: &mut fmt::Formatter<'_>, bytes: &[u8]) -> fmt::Result {
    for b in bytes {
        write!(f, "{b:02x}")?;
    }
    Ok(())
}

impl fmt::Display for IdentityKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_hex(f, &self.0)
    }
}

impl fmt::Debug for IdentityKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("IdentityKey(")?;
        write_hex(f, &self.0)?;
        f.write_str(")")
    }
}

/// Why a 32-byte string was not admitted as an identity key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum KeyReject {
    #[error("public key is not a point on the curve")]
    NotOnCurve,
    #[error("public key encoding is not canonical")]
    NonCanonical,
    #[error("public key has small order")]
    SmallOrder,
}

/// Why a signature was not accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum SigError {
    /// The key failed re-admission; no signature check was attempted.
    #[error("signer key rejected: {0}")]
    Key(#[from] KeyReject),
    /// Malformed (`S ≥ L`, bad `R`) or simply wrong. Deliberately one case:
    /// callers treat both the same (BRIEF_v2 §5.5 relay penalties key on "bad
    /// signature"), and the vector table documents which inputs fall here.
    #[error("signature does not verify")]
    Invalid,
}

/// Admission (rule 1 of the module docs). Decompress, require the canonical
/// re-encoding, reject small order.
fn admit_verifying_key(bytes: &[u8; PUBLIC_KEY_LEN]) -> Result<VerifyingKey, KeyReject> {
    let vk = VerifyingKey::from_bytes(bytes).map_err(|_| KeyReject::NotOnCurve)?;
    // `from_bytes` reduces y mod p and ignores a sign bit on x = 0; only the
    // canonical encoding survives a decompress → recompress round trip.
    if vk.to_edwards().compress().to_bytes() != *bytes {
        return Err(KeyReject::NonCanonical);
    }
    if vk.is_weak() {
        return Err(KeyReject::SmallOrder);
    }
    Ok(vk)
}

/// Admit a 32-byte Ed25519 public key (DESIGN §2.4 "Admission: every Ed25519
/// key entering a KeyCard, roster op, ballot or vote-open passes
/// `admit_public_key` (rejects small-order and non-canonical) before any
/// signature check").
pub fn admit_public_key(bytes: &[u8; PUBLIC_KEY_LEN]) -> Result<IdentityKey, KeyReject> {
    admit_verifying_key(bytes).map(|_| IdentityKey(*bytes))
}

/// `domain ‖ 0x00 ‖ msg` (DESIGN §2.1).
fn domain_message(domain: &str, msg: &[u8]) -> Vec<u8> {
    debug_assert!(
        is_valid_domain(domain),
        "ed25519: invalid domain {domain:?}"
    );
    let mut out = Vec::with_capacity(domain.len() + 1 + msg.len());
    out.extend_from_slice(domain.as_bytes());
    out.push(0);
    out.extend_from_slice(msg);
    out
}

/// Strict verification of a signature over `domain ‖ 0x00 ‖ msg` (rule 2 of
/// the module docs). The key is re-admitted first.
pub fn verify(
    domain: &str,
    key: &IdentityKey,
    msg: &[u8],
    sig: &[u8; SIGNATURE_LEN],
) -> Result<(), SigError> {
    verify_raw(key, &domain_message(domain, msg), sig)
}

/// Strict verification over the raw `msg` with no domain prefix.
///
/// Protocol code must use [`verify`]; this exists so the conformance table
/// can hold standard RFC 8032 vectors, which prove the verifier is plain
/// Ed25519 (and therefore the same on every platform).
pub fn verify_raw(
    key: &IdentityKey,
    msg: &[u8],
    sig: &[u8; SIGNATURE_LEN],
) -> Result<(), SigError> {
    let vk = admit_verifying_key(key.as_bytes())?;
    vk.verify_strict(msg, &Signature::from_bytes(sig))
        .map_err(|_| SigError::Invalid)
}

/// The private half of an identity. Built from a 32-byte seed (RFC 8032 §5.1.5),
/// zeroized on drop, and `Debug` shows only the public key.
#[derive(Clone, ZeroizeOnDrop)]
pub struct IdentitySigner {
    key: SigningKey,
    #[zeroize(skip)]
    public: IdentityKey,
}

impl IdentitySigner {
    pub fn from_seed(seed: &[u8; SEED_LEN]) -> Self {
        let key = SigningKey::from_bytes(seed);
        // A key derived from a seed is [a]B for a clamped scalar a, so it is
        // canonical (compressed by the library) and in the prime-order
        // subgroup; it is never the identity because a clamped a lies in
        // [2^254, 2^255) with its low three bits clear and no such value is a
        // multiple of the odd L (kL for k in 4..8 is never divisible by 8).
        // Hence every seed key admits; `admission_accepts_seed_keys` and the
        // proptest in tests/crypto.rs check this.
        let public = IdentityKey(key.verifying_key().to_bytes());
        Self { key, public }
    }

    pub fn public(&self) -> IdentityKey {
        self.public
    }

    /// The seed, for the identity store (DESIGN §6 `identity` table, secret).
    /// The returned buffer is zeroized on drop.
    pub fn seed(&self) -> Zeroizing<[u8; SEED_LEN]> {
        Zeroizing::new(self.key.to_bytes())
    }

    /// Sign `domain ‖ 0x00 ‖ msg` (DESIGN §2.1). Deterministic.
    pub fn sign(&self, domain: &str, msg: &[u8]) -> [u8; SIGNATURE_LEN] {
        self.key.sign(&domain_message(domain, msg)).to_bytes()
    }
}

impl fmt::Debug for IdentitySigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IdentitySigner")
            .field("public", &self.public)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOMAIN: &str = "meshapp.v1.test";

    /// The group order L, little-endian (RFC 8032 §5.1).
    const L_LE: [u8; 32] = [
        0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde,
        0x14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10,
    ];

    /// The eight small-order points (canonical encodings), order 1, 2, 4, 4, 8, 8, 8, 8.
    const SMALL_ORDER: [&str; 8] = [
        "0100000000000000000000000000000000000000000000000000000000000000",
        "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
        "0000000000000000000000000000000000000000000000000000000000000080",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
        "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
        "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
        "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
    ];

    fn key32(s: &str) -> [u8; 32] {
        hex::decode(s).unwrap().try_into().unwrap()
    }

    const SEED: [u8; 32] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];

    fn signer() -> IdentitySigner {
        IdentitySigner::from_seed(&SEED)
    }

    /// A signature with an attacker-chosen `R` encoding and `S = k·a mod L`
    /// where `k = SHA-512(R ‖ A ‖ M)` and `a` is the private scalar.
    ///
    /// When `R` decodes to the identity, `[S]B = [k]A = R + [k]A`, so the
    /// bare verification equation holds and a verifier that checks nothing
    /// else accepts it (such an `R` also leaks `a = S/k`). RFC 8032-strict
    /// verification must reject every small-order `R`, which is what
    /// distinguishes `verify_strict` from `verify` in `ed25519-dalek`.
    fn forged_r_signature(seed: &[u8; 32], r_bytes: &[u8; 32], msg: &[u8]) -> [u8; 64] {
        use ed25519_dalek::hazmat::ExpandedSecretKey;
        use sha2::{Digest, Sha512};

        let esk = ExpandedSecretKey::from_bytes(&Sha512::digest(seed).into());
        let a = esk.scalar;
        let pk = SigningKey::from_bytes(seed).verifying_key().to_bytes();
        let k_hash: [u8; 64] = Sha512::new()
            .chain_update(r_bytes)
            .chain_update(pk)
            .chain_update(msg)
            .finalize()
            .into();
        // k = k_hash mod L by Horner over 128-bit little-endian limbs; the
        // `From<u128>` conversion reduces mod L. `like` only pins the type.
        fn scalar_like<S: From<u128>>(_like: &S, v: u128) -> S {
            S::from(v)
        }
        let two_128 = scalar_like(&a, 1u128 << 127) * scalar_like(&a, 2);
        let mut k = scalar_like(&a, 0);
        for limb in k_hash.chunks(16).rev() {
            let limb = u128::from_le_bytes(limb.try_into().unwrap());
            k = k * two_128 + scalar_like(&a, limb);
        }
        let s = k * a;
        let mut sig = [0u8; 64];
        sig[..32].copy_from_slice(r_bytes);
        sig[32..].copy_from_slice(&s.to_bytes());
        sig
    }

    /// `s + L` as 32 little-endian bytes (no overflow since s < L < 2^253).
    fn add_l(s: &[u8; 32]) -> [u8; 32] {
        let mut out = [0u8; 32];
        let mut carry = 0u16;
        for i in 0..32 {
            let v = s[i] as u16 + L_LE[i] as u16 + carry;
            out[i] = v as u8;
            carry = v >> 8;
        }
        assert_eq!(carry, 0);
        out
    }

    #[test]
    fn sign_verify_round_trip_and_determinism() {
        let s = signer();
        let sig = s.sign(DOMAIN, b"hello");
        assert_eq!(verify(DOMAIN, &s.public(), b"hello", &sig), Ok(()));
        assert_eq!(
            sig,
            s.sign(DOMAIN, b"hello"),
            "RFC 8032 signing is deterministic"
        );
        let again = IdentitySigner::from_seed(&s.seed());
        assert_eq!(again.public(), s.public());
        assert_eq!(again.sign(DOMAIN, b"hello"), sig);
    }

    #[test]
    fn wrong_message_domain_key_or_signature_is_invalid() {
        let s = signer();
        let sig = s.sign(DOMAIN, b"hello");
        assert_eq!(
            verify(DOMAIN, &s.public(), b"hellp", &sig),
            Err(SigError::Invalid)
        );
        assert_eq!(
            verify("meshapp.v1.other", &s.public(), b"hello", &sig),
            Err(SigError::Invalid)
        );
        let other = IdentitySigner::from_seed(&[0x42u8; 32]);
        assert_eq!(
            verify(DOMAIN, &other.public(), b"hello", &sig),
            Err(SigError::Invalid)
        );
        for byte in [0usize, 31, 32, 63] {
            let mut bad = sig;
            bad[byte] ^= 1;
            assert_eq!(
                verify(DOMAIN, &s.public(), b"hello", &bad),
                Err(SigError::Invalid)
            );
        }
        assert_eq!(
            verify(DOMAIN, &s.public(), b"hello", &[0u8; 64]),
            Err(SigError::Invalid)
        );
    }

    #[test]
    fn domain_separation_is_the_nul_prefixed_layout() {
        // verify(domain, msg) is verify_raw(domain ‖ 0x00 ‖ msg): moving a
        // byte across the separator must not verify.
        let s = signer();
        let sig = s.sign("ab", b"c");
        assert_eq!(verify_raw(&s.public(), b"ab\x00c", &sig), Ok(()));
        assert_eq!(
            verify_raw(&s.public(), b"abc", &sig),
            Err(SigError::Invalid)
        );
        assert_eq!(
            verify("a", &s.public(), b"bc", &sig),
            Err(SigError::Invalid)
        );
    }

    #[test]
    fn non_canonical_s_is_rejected() {
        let s = signer();
        let sig = s.sign(DOMAIN, b"hello");
        let mut bad = sig;
        let s_plus_l = add_l(sig[32..].try_into().unwrap());
        bad[32..].copy_from_slice(&s_plus_l);
        assert_ne!(bad, sig);
        // Mathematically the same scalar mod L; a lax verifier accepts it.
        assert_eq!(
            verify(DOMAIN, &s.public(), b"hello", &bad),
            Err(SigError::Invalid)
        );
        // S = L exactly is also out of range.
        let mut at_l = sig;
        at_l[32..].copy_from_slice(&L_LE);
        assert_eq!(
            verify(DOMAIN, &s.public(), b"hello", &at_l),
            Err(SigError::Invalid)
        );
    }

    #[test]
    fn small_order_r_is_rejected() {
        let s = signer();
        let mut sig = s.sign(DOMAIN, b"hello");
        sig[..32].copy_from_slice(&key32(SMALL_ORDER[0]));
        assert_eq!(
            verify(DOMAIN, &s.public(), b"hello", &sig),
            Err(SigError::Invalid)
        );
    }

    #[test]
    fn small_order_r_is_rejected_even_when_the_equation_holds() {
        // `small_order_r_is_rejected` above keeps the original S, so *any*
        // verifier rejects it. This one builds S to match R = identity: the
        // cofactorless equation holds and ed25519-dalek's non-strict `verify`
        // accepts, so only the strict small-order-R rule rejects it. (The
        // mutation `verify_strict` -> `verify` passed every other test.)
        use ed25519_dalek::Verifier;
        let s = signer();
        let signed = domain_message(DOMAIN, b"hello");
        let identity = key32(SMALL_ORDER[0]);
        let sig = forged_r_signature(&SEED, &identity, &signed);
        let lax = VerifyingKey::from_bytes(s.public().as_bytes()).unwrap();
        assert!(
            lax.verify(&signed, &Signature::from_bytes(&sig)).is_ok(),
            "the bare equation holds, so a lax verifier accepts this signature"
        );
        assert_eq!(
            verify(DOMAIN, &s.public(), b"hello", &sig),
            Err(SigError::Invalid),
            "strict verification rejects small-order R"
        );
        assert_eq!(
            verify_raw(&s.public(), &signed, &sig),
            Err(SigError::Invalid)
        );

        // The same construction with every other small-order R (the equation
        // cannot hold for torsion R ≠ identity under cofactorless
        // verification, but a cofactored verifier would accept) and with the
        // non-canonical encodings of the identity.
        let others = [
            SMALL_ORDER[1],
            SMALL_ORDER[2],
            SMALL_ORDER[3],
            SMALL_ORDER[4],
            SMALL_ORDER[5],
            SMALL_ORDER[6],
            SMALL_ORDER[7],
            "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
            "0100000000000000000000000000000000000000000000000000000000000080",
        ];
        for r in others {
            let sig = forged_r_signature(&SEED, &key32(r), &signed);
            assert_eq!(
                verify(DOMAIN, &s.public(), b"hello", &sig),
                Err(SigError::Invalid),
                "R = {r}"
            );
        }
    }

    #[test]
    fn admission_accepts_seed_keys() {
        for seed in [[0u8; 32], [0xffu8; 32], core::array::from_fn(|i| i as u8)] {
            let s = IdentitySigner::from_seed(&seed);
            assert_eq!(admit_public_key(s.public().as_bytes()), Ok(s.public()));
            assert_eq!(IdentityKey::try_from(s.public().to_bytes()), Ok(s.public()));
            assert_eq!(IdentityKey::try_from(s.public().as_bytes()), Ok(s.public()));
        }
    }

    #[test]
    fn admission_rejects_all_eight_small_order_keys() {
        for enc in SMALL_ORDER {
            assert_eq!(
                admit_public_key(&key32(enc)),
                Err(KeyReject::SmallOrder),
                "{enc}"
            );
        }
    }

    #[test]
    fn admission_rejects_non_canonical_encodings() {
        // y = p (reduces to the y = 0 point) and y = p + 1 (reduces to the identity).
        let y_eq_p = key32("edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f");
        let y_eq_p1 = key32("eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f");
        assert_eq!(admit_public_key(&y_eq_p), Err(KeyReject::NonCanonical));
        assert_eq!(admit_public_key(&y_eq_p1), Err(KeyReject::NonCanonical));
        // The identity with the sign bit set: x = 0 so the sign must be 0.
        let identity_neg =
            key32("0100000000000000000000000000000000000000000000000000000000000080");
        assert_eq!(
            admit_public_key(&identity_neg),
            Err(KeyReject::NonCanonical)
        );
        // A valid key with its sign bit flipped is a different valid key, not
        // a non-canonical one.
        let mut flipped = signer().public().to_bytes();
        flipped[31] ^= 0x80;
        assert!(admit_public_key(&flipped).is_ok());
    }

    #[test]
    fn admission_rejects_points_off_the_curve() {
        // Some small y have no square root for x²; find the first one.
        let found = (2u8..=40).find(|&y| {
            let mut b = [0u8; 32];
            b[0] = y;
            admit_public_key(&b) == Err(KeyReject::NotOnCurve)
        });
        assert!(
            found.is_some(),
            "expected a y in 2..=40 that is not on the curve"
        );
    }

    #[test]
    fn weak_key_never_becomes_an_identity_key() {
        // An IdentityKey can only be built through admission, so verification
        // with a weak key is impossible by construction and admission is what
        // rejects it. (The re-admission inside `verify_raw` is defence in
        // depth and cannot be observed through the public API: removing it
        // fails no test, because `verify_strict` also rejects a weak A and no
        // `IdentityKey` can hold a non-canonical encoding.)
        let s = signer();
        let sig = s.sign(DOMAIN, b"x");
        assert_eq!(verify(DOMAIN, &s.public(), b"x", &sig), Ok(()));
        assert_eq!(
            admit_public_key(&key32(SMALL_ORDER[4])).map(|k| verify(DOMAIN, &k, b"x", &sig)),
            Err(KeyReject::SmallOrder)
        );
    }

    #[test]
    fn signer_debug_is_redacted_and_key_formats_as_hex() {
        let s = signer();
        let dbg = format!("{s:?}");
        assert!(dbg.starts_with("IdentitySigner { public: IdentityKey("));
        assert!(!dbg.contains(&hex::encode(s.seed().as_ref())));
        let pk = s.public();
        assert_eq!(pk.to_string(), hex::encode(pk.as_bytes()));
        assert_eq!(format!("{pk:?}"), format!("IdentityKey({pk})"));
    }

    #[test]
    fn identity_keys_order_by_bytes() {
        let a = IdentitySigner::from_seed(&[1u8; 32]).public();
        let b = IdentitySigner::from_seed(&[2u8; 32]).public();
        assert_eq!(a < b, a.as_bytes() < b.as_bytes());
        assert_eq!(a.cmp(&b), a.as_bytes().cmp(b.as_bytes()));
    }

    #[test]
    fn error_messages() {
        assert_eq!(
            SigError::from(KeyReject::SmallOrder).to_string(),
            "signer key rejected: public key has small order"
        );
        assert_eq!(SigError::Invalid.to_string(), "signature does not verify");
        assert_eq!(
            KeyReject::NonCanonical.to_string(),
            "public key encoding is not canonical"
        );
        assert_eq!(
            KeyReject::NotOnCurve.to_string(),
            "public key is not a point on the curve"
        );
    }
}
