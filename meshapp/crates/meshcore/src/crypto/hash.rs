//! Hashing and key derivation (docs/DESIGN.md §1 row `crypto/hash`).
//!
//! Everything here is a pure function of its inputs: no randomness, no clock,
//! no allocation beyond the output. The protocol uses these in three places:
//!
//! * `object_hash(domain, bytes)` for object identifiers that must never
//!   collide across object kinds (BRIEF_v2 §5.5 "message id"; DESIGN §2.1
//!   domain separation): `SHA256(domain ‖ 0x00 ‖ bytes)`.
//! * `prf16(key, label, ctr)` for the per-cell recognition tags
//!   (BRIEF_v2 §5.5 "a tag = PRF(tag key, counter)"; DESIGN §2.2 `tag`,
//!   §2.3 `ContactProof`): the first 16 bytes of
//!   `HMAC-SHA256(key, label ‖ 0x00 ‖ LE64(ctr))`.
//! * `hkdf32(ikm, salt, info)` for key derivation (DESIGN §2.2 cell key
//!   `HKDF(k_hdr, "cell", ctr)`): HKDF-SHA256 with a 32-byte output.
//!
//! Layouts in DESIGN §2 that specify a bare concatenation under SHA-256
//! (for example `msg_id = SHA256(cell[0] ‖ cell[2..512])` or
//! `Trunc16(SHA256("seal" ‖ E_r ‖ e_pub))`) use [`sha256`] or
//! [`sha256_concat`] directly; [`object_hash`] is only for the
//! `domain ‖ 0x00 ‖ content` form.

use core::fmt;

use hkdf::Hkdf;
use hmac::{Hmac, KeyInit, Mac};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

/// Length of every hash in the protocol (DESIGN §2.1 "hashes 32 B").
pub const HASH_LEN: usize = 32;

/// A 32-byte SHA-256 output. Ordered and hashable so it can key maps and be
/// sorted (DESIGN §2.4 "heads [32]×n (sorted ascending)").
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Hash32([u8; HASH_LEN]);

impl Hash32 {
    /// The all-zero hash, used where the design says `0³²` (DESIGN §2.4
    /// `prev_own` of the first object, `preview_hash` when there is none).
    pub const ZERO: Hash32 = Hash32([0u8; HASH_LEN]);

    pub const fn from_bytes(bytes: [u8; HASH_LEN]) -> Self {
        Hash32(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; HASH_LEN] {
        &self.0
    }

    pub const fn to_bytes(self) -> [u8; HASH_LEN] {
        self.0
    }

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    /// First 16 bytes, for `Trunc16` fields (DESIGN §2.4: group id, task id;
    /// §2.2 sealed-flight tags).
    pub fn truncate16(&self) -> [u8; 16] {
        let mut out = [0u8; 16];
        out.copy_from_slice(&self.0[..16]);
        out
    }

    /// First 8 bytes, for `Trunc8` fields (DESIGN §2.4 SKDM `hint`).
    pub fn truncate8(&self) -> [u8; 8] {
        let mut out = [0u8; 8];
        out.copy_from_slice(&self.0[..8]);
        out
    }
}

impl From<[u8; HASH_LEN]> for Hash32 {
    fn from(bytes: [u8; HASH_LEN]) -> Self {
        Hash32(bytes)
    }
}

impl From<Hash32> for [u8; HASH_LEN] {
    fn from(h: Hash32) -> Self {
        h.0
    }
}

impl AsRef<[u8]> for Hash32 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&[u8]> for Hash32 {
    type Error = core::array::TryFromSliceError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        <[u8; HASH_LEN]>::try_from(bytes).map(Hash32)
    }
}

fn write_hex(f: &mut fmt::Formatter<'_>, bytes: &[u8]) -> fmt::Result {
    for b in bytes {
        write!(f, "{b:02x}")?;
    }
    Ok(())
}

impl fmt::Display for Hash32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_hex(f, &self.0)
    }
}

impl fmt::Debug for Hash32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Hash32(")?;
        write_hex(f, &self.0)?;
        f.write_str(")")
    }
}

/// `true` when `domain` is usable as a domain-separation label: non-empty
/// ASCII without NUL (the NUL byte is the separator between label and
/// content, so a label containing it would be ambiguous; DESIGN §2.1).
pub(crate) fn is_valid_domain(domain: &str) -> bool {
    !domain.is_empty() && domain.bytes().all(|b| b.is_ascii() && b != 0)
}

/// Plain SHA-256.
pub fn sha256(bytes: &[u8]) -> Hash32 {
    Hash32(Sha256::digest(bytes).into())
}

/// SHA-256 over the concatenation of `parts`, without copying them together.
/// Identical to `sha256(parts.concat())`.
pub fn sha256_concat(parts: &[&[u8]]) -> Hash32 {
    let mut h = Sha256::new();
    for p in parts {
        h.update(p);
    }
    Hash32(h.finalize().into())
}

/// Domain-separated object hash: `SHA256(domain ‖ 0x00 ‖ bytes)`
/// (DESIGN §2.1; BRIEF_v2 §5.5 tags and message ids).
///
/// `domain` must be non-empty ASCII without NUL; this is a programming
/// error, not an input error, so it is a `debug_assert`.
pub fn object_hash(domain: &str, bytes: &[u8]) -> Hash32 {
    debug_assert!(
        is_valid_domain(domain),
        "object_hash: invalid domain {domain:?}"
    );
    sha256_concat(&[domain.as_bytes(), &[0u8], bytes])
}

/// HMAC-SHA256 over the concatenation of `parts` under `key` (any key length;
/// HMAC pads or hashes it).
pub fn hmac_sha256(key: &[u8], parts: &[&[u8]]) -> [u8; 32] {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC accepts keys of any length");
    for p in parts {
        mac.update(p);
    }
    mac.finalize().into_bytes().into()
}

/// Recognition-tag PRF (BRIEF_v2 §5.5 "tag = PRF(tag key, counter)";
/// DESIGN §1 `prf16(key,label,ctr) = Trunc16(HMAC-SHA256)`):
/// the first 16 bytes of `HMAC-SHA256(key, label ‖ 0x00 ‖ LE64(ctr))`.
///
/// The label separates the tag stream (`"tag"`) from other uses of the same
/// key (DESIGN §2.3 `"l2"` contact proofs), and the counter is little-endian
/// like every other integer on the wire (DESIGN §2.1).
pub fn prf16(key: &[u8; 32], label: &str, ctr: u64) -> [u8; 16] {
    debug_assert!(is_valid_domain(label), "prf16: invalid label {label:?}");
    let full = hmac_sha256(key, &[label.as_bytes(), &[0u8], &ctr.to_le_bytes()]);
    let mut out = [0u8; 16];
    out.copy_from_slice(&full[..16]);
    out
}

/// HKDF-SHA256 (RFC 5869) with a 32-byte output: `Expand(Extract(salt, ikm), info)`.
/// An empty `salt` is equivalent to the RFC's default of `HashLen` zero bytes.
pub fn hkdf32(ikm: &[u8], salt: &[u8], info: &[u8]) -> [u8; 32] {
    let hk = Hkdf::<Sha256>::new(Some(salt), ikm);
    let mut okm = [0u8; 32];
    hk.expand(info, &mut okm)
        .expect("32 bytes is below the HKDF-SHA256 output limit of 255*32");
    okm
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h(s: &str) -> Vec<u8> {
        hex::decode(s).expect("valid hex in test")
    }

    #[test]
    fn sha256_known_answer() {
        // FIPS 180-4 example: SHA-256("abc").
        assert_eq!(
            sha256(b"abc").to_string(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        // SHA-256 of the empty string.
        assert_eq!(
            sha256(b"").to_string(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn sha256_concat_equals_sha256_of_concatenation() {
        let parts: [&[u8]; 4] = [b"a", b"", b"bcd", b"ef"];
        assert_eq!(sha256_concat(&parts), sha256(b"abcdef"));
        assert_eq!(sha256_concat(&[]), sha256(b""));
    }

    #[test]
    fn object_hash_layout_and_domain_separation() {
        let msg = b"payload";
        assert_eq!(
            object_hash("meshapp.v1.x", msg),
            sha256(b"meshapp.v1.x\x00payload")
        );
        assert_ne!(
            object_hash("meshapp.v1.x", msg),
            object_hash("meshapp.v1.y", msg)
        );
        // The NUL separator makes (domain, content) unambiguous: moving a
        // byte across the boundary changes the hash.
        assert_ne!(object_hash("ab", b"c"), object_hash("a", b"bc"));
        assert_ne!(object_hash("ab", b"c"), sha256(b"abc"));
    }

    #[test]
    fn domain_validation() {
        assert!(is_valid_domain("meshapp.v1.keycard"));
        assert!(!is_valid_domain(""));
        assert!(!is_valid_domain("a\0b"));
        assert!(!is_valid_domain("naïve"));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "invalid domain")]
    fn object_hash_rejects_empty_domain_in_debug() {
        let _ = object_hash("", b"x");
    }

    #[test]
    fn hmac_sha256_known_answer() {
        // RFC 4231 test case 2.
        let mac = hmac_sha256(b"Jefe", &[b"what do ya want for nothing?"]);
        assert_eq!(
            mac.to_vec(),
            h("5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843")
        );
        // Splitting the data across parts does not change the MAC.
        assert_eq!(
            mac,
            hmac_sha256(b"Jefe", &[b"what do ya ", b"want for nothing?"])
        );
        // RFC 4231 test case 6: a 131-byte key, longer than the SHA-256 block,
        // is hashed first rather than rejected (the `expect` in `hmac_sha256`
        // relies on HMAC accepting any key length).
        let long_key = [0xaau8; 131];
        assert_eq!(
            hmac_sha256(
                &long_key,
                &[b"Test Using Larger Than Block-Size Key - Hash Key First"]
            )
            .to_vec(),
            h("60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54")
        );
    }

    #[test]
    fn prf16_layout() {
        let key = [7u8; 32];
        let full = hmac_sha256(&key, &[b"tag", &[0u8], &5u64.to_le_bytes()]);
        assert_eq!(prf16(&key, "tag", 5), full[..16]);
        // Little-endian counter: ctr=1 is 01 00 00 00 00 00 00 00.
        let full1 = hmac_sha256(&key, &[b"tag\x00\x01\x00\x00\x00\x00\x00\x00\x00"]);
        assert_eq!(prf16(&key, "tag", 1), full1[..16]);
    }

    #[test]
    fn prf16_distinct_across_key_label_counter() {
        let k1 = [1u8; 32];
        let k2 = [2u8; 32];
        let base = prf16(&k1, "tag", 0);
        assert_eq!(base, prf16(&k1, "tag", 0), "deterministic");
        assert_ne!(base, prf16(&k2, "tag", 0));
        assert_ne!(base, prf16(&k1, "l2", 0));
        assert_ne!(base, prf16(&k1, "tag", 1));
        assert_ne!(prf16(&k1, "tag", u64::MAX), prf16(&k1, "tag", u64::MAX - 1));
    }

    #[test]
    fn hkdf32_known_answer() {
        // RFC 5869 test case 1, first 32 of the 42 output bytes.
        let ikm = [0x0bu8; 22];
        let salt = h("000102030405060708090a0b0c");
        let info = h("f0f1f2f3f4f5f6f7f8f9");
        assert_eq!(
            hkdf32(&ikm, &salt, &info).to_vec(),
            h("3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf")
        );
    }

    #[test]
    fn hkdf32_empty_salt_matches_zero_salt() {
        // RFC 5869 §2.2: "if not provided, [salt] is set to a string of
        // HashLen zeros". HMAC zero-pads keys, so an empty salt is the same.
        let ikm = b"input keying material";
        assert_eq!(hkdf32(ikm, b"", b"info"), hkdf32(ikm, &[0u8; 32], b"info"));
        assert_ne!(hkdf32(ikm, b"", b"info"), hkdf32(ikm, b"", b"other"));
        assert_ne!(hkdf32(ikm, b"", b"info"), hkdf32(ikm, b"salt", b"info"));
    }

    #[test]
    fn hash32_conversions_and_formatting() {
        let hsh = sha256(b"abc");
        assert_eq!(hsh.truncate16(), hsh.as_bytes()[..16]);
        assert_eq!(hsh.truncate8(), hsh.as_bytes()[..8]);
        assert_eq!(Hash32::from(hsh.to_bytes()), hsh);
        assert_eq!(Hash32::try_from(&hsh.to_bytes()[..]).unwrap(), hsh);
        assert!(Hash32::try_from(&hsh.to_bytes()[..31]).is_err());
        assert!(Hash32::ZERO.is_zero());
        assert_eq!(Hash32::default(), Hash32::ZERO);
        assert!(!hsh.is_zero());
        assert_eq!(format!("{hsh:?}"), format!("Hash32({hsh})"));
        assert_eq!(hsh.to_string().len(), 64);
        // Byte-lexicographic order, as the design's "sorted ascending" lists need.
        let lo = Hash32::from([0u8; 32]);
        let mut hi = [0u8; 32];
        hi[0] = 1;
        assert!(lo < Hash32::from(hi));
    }
}
