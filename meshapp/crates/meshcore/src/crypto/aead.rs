//! AEAD wrappers (docs/DESIGN.md §1 row `crypto/aead`; BRIEF_v2 §5.5).
//!
//! Two constructions, chosen by nonce size:
//!
//! * [`seal`] / [`open`]: ChaCha20-Poly1305 (RFC 8439) with a 12-byte nonce,
//!   for cell bodies where the protocol controls the nonce as a counter
//!   (DESIGN §2.2: key = `HKDF(k_hdr, "cell", ctr)`, nonce =
//!   `LE32(frag_idx) ‖ 0⁸`, AAD = the clear header bytes).
//! * [`seal_x`] / [`open_x`]: XChaCha20-Poly1305 with a 24-byte nonce, for
//!   storage blobs under the long-lived `db_key`, where nonces are drawn at
//!   random and a 96-bit nonce would be unsafe at scale (DESIGN §1
//!   `storage/kv.rs` "XChaCha20-Poly1305 under `db_key`", §6).
//!
//! `open*` returns one opaque error, [`AeadError::Failed`], for every failure
//! (bad tag, truncated input, wrong key, nonce or AAD): a relay must reject
//! malformed input (BRIEF_v2 §5.5 "checks format, size and TTL"), and a
//! distinguishable error would be an oracle. Tag comparison is constant-time
//! inside the `chacha20poly1305` crate.
//!
//! Nonce uniqueness is the caller's responsibility; nothing here draws
//! randomness or reads a clock.

use core::fmt;

use chacha20poly1305::aead::{Aead, Payload};
use chacha20poly1305::{ChaCha20Poly1305, KeyInit, XChaCha20Poly1305};
use subtle::ConstantTimeEq;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Key length of both constructions.
pub const KEY_LEN: usize = 32;
/// Nonce length of [`seal`] / [`open`] (ChaCha20-Poly1305).
pub const NONCE_LEN: usize = 12;
/// Nonce length of [`seal_x`] / [`open_x`] (XChaCha20-Poly1305).
pub const XNONCE_LEN: usize = 24;
/// Poly1305 tag appended to every ciphertext (DESIGN §2.2 "476 B plaintext +
/// 16 B auth tag").
pub const TAG_LEN: usize = 16;

/// A 32-byte symmetric key that is wiped when dropped and never printed.
///
/// `from_bytes` copies the array; the caller still owns (and should zeroize)
/// its own copy.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct Key32([u8; KEY_LEN]);

impl Key32 {
    pub fn from_bytes(bytes: [u8; KEY_LEN]) -> Self {
        Key32(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; KEY_LEN] {
        &self.0
    }
}

impl From<[u8; KEY_LEN]> for Key32 {
    fn from(bytes: [u8; KEY_LEN]) -> Self {
        Key32(bytes)
    }
}

impl PartialEq for Key32 {
    /// Constant-time comparison.
    fn eq(&self, other: &Self) -> bool {
        self.0.ct_eq(&other.0).into()
    }
}

impl Eq for Key32 {}

impl fmt::Debug for Key32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Key32(<redacted>)")
    }
}

/// The only error `open*` reports. Deliberately carries no detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum AeadError {
    #[error("authenticated decryption failed")]
    Failed,
}

/// ChaCha20-Poly1305: `ciphertext ‖ tag` (`plaintext.len() + 16` bytes).
pub fn seal(key: &Key32, nonce: &[u8; NONCE_LEN], aad: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new(key.as_bytes().into());
    cipher
        .encrypt(
            nonce.into(),
            Payload {
                msg: plaintext,
                aad,
            },
        )
        .expect("ChaCha20-Poly1305 encryption only fails for inputs above 2^38 bytes")
}

/// Inverse of [`seal`]. Any failure, including a ciphertext shorter than the
/// tag, is [`AeadError::Failed`].
pub fn open(
    key: &Key32,
    nonce: &[u8; NONCE_LEN],
    aad: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, AeadError> {
    let cipher = ChaCha20Poly1305::new(key.as_bytes().into());
    cipher
        .decrypt(
            nonce.into(),
            Payload {
                msg: ciphertext,
                aad,
            },
        )
        .map_err(|_| AeadError::Failed)
}

/// XChaCha20-Poly1305 with a 24-byte nonce: `ciphertext ‖ tag`.
pub fn seal_x(key: &Key32, nonce: &[u8; XNONCE_LEN], aad: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let cipher = XChaCha20Poly1305::new(key.as_bytes().into());
    cipher
        .encrypt(
            nonce.into(),
            Payload {
                msg: plaintext,
                aad,
            },
        )
        .expect("XChaCha20-Poly1305 encryption only fails for inputs above 2^38 bytes")
}

/// Inverse of [`seal_x`]. Any failure is [`AeadError::Failed`].
pub fn open_x(
    key: &Key32,
    nonce: &[u8; XNONCE_LEN],
    aad: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, AeadError> {
    let cipher = XChaCha20Poly1305::new(key.as_bytes().into());
    cipher
        .decrypt(
            nonce.into(),
            Payload {
                msg: ciphertext,
                aad,
            },
        )
        .map_err(|_| AeadError::Failed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h(s: &str) -> Vec<u8> {
        hex::decode(s).expect("valid hex in test")
    }

    fn key() -> Key32 {
        Key32::from_bytes(core::array::from_fn(|i| i as u8))
    }

    const RFC8439_PT: &[u8] = b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";
    const RFC8439_AAD: &str = "50515253c0c1c2c3c4c5c6c7";

    fn rfc8439_key() -> Key32 {
        Key32::from_bytes(core::array::from_fn(|i| 0x80 + i as u8))
    }

    #[test]
    fn chacha20poly1305_rfc8439_known_answer() {
        // RFC 8439 §2.8.2.
        let nonce: [u8; 12] = h("070000004041424344454647").try_into().unwrap();
        let aad = h(RFC8439_AAD);
        let expected = h(concat!(
            "d31a8d34648e60db7b86afbc53ef7ec2a4aded51296e08fea9e2b5a736ee62d6",
            "3dbea45e8ca9671282fafb69da92728b1a71de0a9e060b2905d6a5b67ecd3b36",
            "92ddbd7f2d778b8c9803aee328091b58fab324e4fad675945585808b4831d7bc",
            "3ff4def08e4b7a9de576d26586cec64b6116",
            "1ae10b594f09e26a7e902ecbd0600691"
        ));
        let ct = seal(&rfc8439_key(), &nonce, &aad, RFC8439_PT);
        assert_eq!(ct, expected);
        assert_eq!(open(&rfc8439_key(), &nonce, &aad, &ct).unwrap(), RFC8439_PT);
    }

    #[test]
    fn xchacha20poly1305_draft_known_answer() {
        // draft-irtf-cfrg-xchacha-03 §A.3.1.
        let nonce: [u8; 24] = h("404142434445464748494a4b4c4d4e4f5051525354555657")
            .try_into()
            .unwrap();
        let aad = h(RFC8439_AAD);
        let expected = h(concat!(
            "bd6d179d3e83d43b9576579493c0e939572a1700252bfaccbed2902c21396cbb",
            "731c7f1b0b4aa6440bf3a82f4eda7e39ae64c6708c54c216cb96b72e1213b452",
            "2f8c9ba40db5d945b11b69b982c1bb9e3f3fac2bc369488f76b2383565d3fff9",
            "21f9664c97637da9768812f615c68b13b52e",
            "c0875924c1c7987947deafd8780acf49"
        ));
        let ct = seal_x(&rfc8439_key(), &nonce, &aad, RFC8439_PT);
        assert_eq!(ct, expected);
        assert_eq!(
            open_x(&rfc8439_key(), &nonce, &aad, &ct).unwrap(),
            RFC8439_PT
        );
    }

    #[test]
    fn round_trip_with_aad() {
        let k = key();
        let nonce = [9u8; NONCE_LEN];
        let ct = seal(&k, &nonce, b"header", b"body bytes");
        assert_eq!(ct.len(), b"body bytes".len() + TAG_LEN);
        assert_eq!(open(&k, &nonce, b"header", &ct).unwrap(), b"body bytes");
        // Deterministic for identical inputs (the nonce is the only freshness).
        assert_eq!(ct, seal(&k, &nonce, b"header", b"body bytes"));
    }

    #[test]
    fn round_trip_x_with_aad() {
        let k = key();
        let nonce = [3u8; XNONCE_LEN];
        let ct = seal_x(&k, &nonce, b"blob:1", b"ratchet state");
        assert_eq!(ct.len(), b"ratchet state".len() + TAG_LEN);
        assert_eq!(
            open_x(&k, &nonce, b"blob:1", &ct).unwrap(),
            b"ratchet state"
        );
        assert_eq!(ct, seal_x(&k, &nonce, b"blob:1", b"ratchet state"));
    }

    #[test]
    fn empty_plaintext_is_a_bare_tag() {
        let k = key();
        let nonce = [0u8; NONCE_LEN];
        let ct = seal(&k, &nonce, b"", b"");
        assert_eq!(ct.len(), TAG_LEN);
        assert_eq!(open(&k, &nonce, b"", &ct).unwrap(), Vec::<u8>::new());
        let ctx = seal_x(&k, &[0u8; XNONCE_LEN], b"", b"");
        assert_eq!(ctx.len(), TAG_LEN);
        assert_eq!(
            open_x(&k, &[0u8; XNONCE_LEN], b"", &ctx).unwrap(),
            Vec::<u8>::new()
        );
    }

    #[test]
    fn wrong_key_nonce_or_aad_fails() {
        let k = key();
        let nonce = [1u8; NONCE_LEN];
        let ct = seal(&k, &nonce, b"aad", b"secret");
        let mut other_key = k.as_bytes().to_owned();
        other_key[31] ^= 0x80;
        assert_eq!(
            open(&Key32::from_bytes(other_key), &nonce, b"aad", &ct),
            Err(AeadError::Failed)
        );
        let mut other_nonce = nonce;
        other_nonce[0] ^= 1;
        assert_eq!(open(&k, &other_nonce, b"aad", &ct), Err(AeadError::Failed));
        assert_eq!(open(&k, &nonce, b"aad!", &ct), Err(AeadError::Failed));
        assert_eq!(open(&k, &nonce, b"", &ct), Err(AeadError::Failed));
        assert_eq!(open(&k, &nonce, b"aad", &ct).unwrap(), b"secret");
    }

    #[test]
    fn wrong_key_nonce_or_aad_fails_x() {
        let k = key();
        let nonce = [1u8; XNONCE_LEN];
        let ct = seal_x(&k, &nonce, b"aad", b"secret");
        let mut other_key = k.as_bytes().to_owned();
        other_key[0] ^= 1;
        assert_eq!(
            open_x(&Key32::from_bytes(other_key), &nonce, b"aad", &ct),
            Err(AeadError::Failed)
        );
        let mut other_nonce = nonce;
        other_nonce[23] ^= 1;
        assert_eq!(
            open_x(&k, &other_nonce, b"aad", &ct),
            Err(AeadError::Failed)
        );
        assert_eq!(open_x(&k, &nonce, b"", &ct), Err(AeadError::Failed));
    }

    #[test]
    fn every_single_bit_flip_is_rejected() {
        let k = key();
        let nonce = [2u8; NONCE_LEN];
        let ct = seal(&k, &nonce, b"a", b"short message");
        for byte in 0..ct.len() {
            for bit in 0..8 {
                let mut bad = ct.clone();
                bad[byte] ^= 1 << bit;
                assert_eq!(
                    open(&k, &nonce, b"a", &bad),
                    Err(AeadError::Failed),
                    "flipped byte {byte} bit {bit}"
                );
            }
        }
        let ctx = seal_x(&k, &[2u8; XNONCE_LEN], b"a", b"short message");
        for byte in 0..ctx.len() {
            let mut bad = ctx.clone();
            bad[byte] ^= 0x01;
            assert_eq!(
                open_x(&k, &[2u8; XNONCE_LEN], b"a", &bad),
                Err(AeadError::Failed)
            );
        }
    }

    #[test]
    fn truncated_or_too_short_ciphertext_fails() {
        let k = key();
        let nonce = [4u8; NONCE_LEN];
        let ct = seal(&k, &nonce, b"", b"payload");
        assert_eq!(
            open(&k, &nonce, b"", &ct[..ct.len() - 1]),
            Err(AeadError::Failed)
        );
        assert_eq!(open(&k, &nonce, b"", &ct[1..]), Err(AeadError::Failed));
        assert_eq!(
            open(&k, &nonce, b"", &ct[..TAG_LEN - 1]),
            Err(AeadError::Failed)
        );
        assert_eq!(open(&k, &nonce, b"", &[]), Err(AeadError::Failed));
        assert_eq!(
            open_x(&k, &[4u8; XNONCE_LEN], b"", &[]),
            Err(AeadError::Failed)
        );
        assert_eq!(
            open_x(&k, &[4u8; XNONCE_LEN], b"", &[0u8; TAG_LEN - 1]),
            Err(AeadError::Failed)
        );
    }

    #[test]
    fn constructions_do_not_cross_open() {
        let k = key();
        let ct = seal(&k, &[0u8; NONCE_LEN], b"", b"x");
        assert_eq!(
            open_x(&k, &[0u8; XNONCE_LEN], b"", &ct),
            Err(AeadError::Failed)
        );
        let ctx = seal_x(&k, &[0u8; XNONCE_LEN], b"", b"x");
        assert_eq!(
            open(&k, &[0u8; NONCE_LEN], b"", &ctx),
            Err(AeadError::Failed)
        );
        assert_ne!(ct, ctx);
    }

    #[test]
    fn key32_is_redacted_and_compares_by_value() {
        let k = key();
        assert_eq!(format!("{k:?}"), "Key32(<redacted>)");
        assert!(!format!("{k:?}").contains("00"));
        assert_eq!(k, Key32::from(k.as_bytes().to_owned()));
        assert_ne!(k, Key32::from_bytes([0u8; KEY_LEN]));
        assert_eq!(
            AeadError::Failed.to_string(),
            "authenticated decryption failed"
        );
    }
}
