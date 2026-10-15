//! Protocol objects (docs/DESIGN.md §2.4): the canonical encodings of
//! everything that travels inside the encryption. Identity-signed objects
//! (roster operations, vote openings) share [`SignedHeader`]; ballots are
//! standalone signatures; posts and bundles are plaintexts of group or
//! pairwise messages.
//!
//! Signing convention (crypto::ed25519): `sig = Ed25519(sk, domain ‖ 0x00 ‖
//! bytes)`; the object hash is `SHA256(all bytes including sig)`.

pub mod ballot;
pub mod chat_post;
pub mod roster_op;
pub mod sender_chain;
pub mod skdm_bundle;
pub mod task_op;
pub mod vote_open;

use crate::codec::{CodecError, Decode, Encode, ListItem, Reader, Writer};
use crate::crypto::backend::GroupId;
use crate::crypto::ed25519::{
    IdentityKey, IdentitySigner, SigError, admit_public_key, sign, verify,
};
use crate::crypto::hash::{Hash32, sha256};

/// Version byte every object starts with. Decoders reject anything else.
pub const OBJ_VERSION: u8 = 1;

/// Largest number of causal heads an identity-signed object may cite.
pub const MAX_HEADS: usize = 32;

/// Object kind discriminants (DESIGN §2.4).
pub mod kind {
    pub const ROSTER_GENESIS: u8 = 0x10;
    pub const ROSTER_ADD: u8 = 0x11;
    pub const ROSTER_REMOVE: u8 = 0x12;
    pub const ROSTER_ADMIN_SET: u8 = 0x13;
    pub const ROSTER_SETTING: u8 = 0x14;
    pub const VOTE_OPEN: u8 = 0x20;
    pub const BALLOT: u8 = 0x21;
    pub const POST_TEXT: u8 = 0x30;
    pub const POST_TASK_OP: u8 = 0x31;
    pub const POST_CARRIED_VOTE_OPEN: u8 = 0x32;
    pub const POST_CARRIED_BALLOT: u8 = 0x33;
    pub const POST_CARRIED_ROSTER_OP: u8 = 0x34;
    pub const POST_RECEIPT: u8 = 0x35;
    pub const POST_SKDM_BUNDLE: u8 = 0x36;
}

/// Signature domains, one per identity-signed object family.
pub mod domain {
    pub const ROSTER_OP: &str = "meshapp.v1.roster-op";
    pub const ENDORSE: &str = "meshapp.v1.endorse";
    pub const ADMIN_SET: &str = "meshapp.v1.adminset";
    pub const VOTE_OPEN: &str = "meshapp.v1.vote-open";
    pub const BALLOT: &str = "meshapp.v1.ballot";
}

/// Common header of identity-signed objects (DESIGN §2.4 "Common header"):
/// `ver u8 ‖ kind u8 ‖ group_id [16] ‖ author [32] ‖ author_seq u32 ‖
/// prev_own [32] ‖ n_heads u8 ‖ heads [32]×n (sorted ascending)`.
///
/// `author_seq`/`prev_own` form the author's per-sender hash chain
/// (`objects::sender_chain`): `prev_own` is the hash of the author's previous
/// identity-signed object, `Hash32::ZERO` for the first. `heads` are the
/// roster-DAG heads the author had seen (causal order, no timestamps,
/// BRIEF_v2 §4).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SignedHeader {
    pub kind: u8,
    pub group_id: GroupId,
    pub author: IdentityKey,
    pub author_seq: u32,
    pub prev_own: Hash32,
    /// Sorted ascending, no duplicates, at most [`MAX_HEADS`].
    pub heads: Vec<Hash32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum HeaderError {
    #[error("object version {0} is not supported")]
    BadVersion(u8),
    #[error("heads must be sorted ascending without duplicates")]
    HeadsNotSorted,
    #[error("too many heads: {0}")]
    TooManyHeads(usize),
}

impl SignedHeader {
    /// Normalises `heads` (sort, dedupe) and checks the count.
    pub fn new(
        kind: u8,
        group_id: GroupId,
        author: IdentityKey,
        author_seq: u32,
        prev_own: Hash32,
        mut heads: Vec<Hash32>,
    ) -> Result<Self, HeaderError> {
        heads.sort_unstable();
        heads.dedup();
        if heads.len() > MAX_HEADS {
            return Err(HeaderError::TooManyHeads(heads.len()));
        }
        Ok(Self {
            kind,
            group_id,
            author,
            author_seq,
            prev_own,
            heads,
        })
    }

    fn check_heads(heads: &[Hash32]) -> Result<(), HeaderError> {
        if heads.len() > MAX_HEADS {
            return Err(HeaderError::TooManyHeads(heads.len()));
        }
        if heads.windows(2).any(|w| w[0] >= w[1]) {
            return Err(HeaderError::HeadsNotSorted);
        }
        Ok(())
    }
}

impl Encode for SignedHeader {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        Self::check_heads(&self.heads).map_err(|_| CodecError::Invalid("heads"))?;
        w.u8(OBJ_VERSION).u8(self.kind);
        self.group_id.encode_to(w)?;
        w.bytes(self.author.as_bytes())
            .u32(self.author_seq)
            .bytes(self.prev_own.as_bytes());
        w.list_u8(&self.heads, |w, h| {
            w.bytes(h.as_bytes());
            Ok(())
        })
    }
}

impl Decode for SignedHeader {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let ver = r.u8()?;
        if ver != OBJ_VERSION {
            return Err(CodecError::UnknownVariant {
                what: "object version",
                value: ver as u64,
            });
        }
        let kind = r.u8()?;
        let group_id = GroupId::decode_from(r)?;
        let author_bytes: [u8; 32] = r.bytes()?;
        let author =
            admit_public_key(&author_bytes).map_err(|_| CodecError::Invalid("author key"))?;
        let author_seq = r.u32()?;
        let prev_own = Hash32::from_bytes(r.bytes()?);
        let heads: Vec<Hash32> = r.list_u8(|r| Ok(Hash32::from_bytes(r.bytes()?)))?;
        Self::check_heads(&heads).map_err(|_| CodecError::Invalid("heads"))?;
        Ok(Self {
            kind,
            group_id,
            author,
            author_seq,
            prev_own,
            heads,
        })
    }
}

impl ListItem for Hash32 {}

/// Signs `bytes` (a header plus body, already encoded) under `domain`.
pub fn sign_bytes(signer: &IdentitySigner, domain: &str, bytes: &[u8]) -> [u8; 64] {
    sign(signer, domain, bytes)
}

/// Verifies `sig` over `bytes` under `domain` with the strict verifier.
pub fn verify_bytes(
    domain: &str,
    key: &IdentityKey,
    bytes: &[u8],
    sig: &[u8; 64],
) -> Result<(), SigError> {
    verify(domain, key, bytes, sig)
}

/// `op_hash = SHA256(all bytes including sig)` (DESIGN §2.4).
pub fn object_id(encoded_with_sig: &[u8]) -> Hash32 {
    sha256(encoded_with_sig)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::decode;

    fn header() -> SignedHeader {
        let signer = IdentitySigner::from_seed(&[1u8; 32]);
        SignedHeader::new(
            kind::ROSTER_REMOVE,
            GroupId([7u8; 16]),
            signer.public(),
            3,
            Hash32::from_bytes([9u8; 32]),
            vec![Hash32::from_bytes([5u8; 32]), Hash32::from_bytes([2u8; 32])],
        )
        .unwrap()
    }

    #[test]
    fn header_round_trips_with_sorted_heads() {
        let h = header();
        assert!(h.heads[0] < h.heads[1]);
        let bytes = h.encode();
        assert_eq!(bytes.len(), 1 + 1 + 16 + 32 + 4 + 32 + 1 + 64);
        let back: SignedHeader = decode(&bytes).unwrap();
        assert_eq!(back, h);
    }

    #[test]
    fn header_rejects_bad_version_unsorted_heads_and_weak_author() {
        let h = header();
        let mut bytes = h.encode();
        bytes[0] = 2;
        assert!(decode::<SignedHeader>(&bytes).is_err());

        let mut bytes = h.encode();
        // swap the two heads in place → unsorted
        let off = 1 + 1 + 16 + 32 + 4 + 32 + 1;
        let (a, b) = (
            bytes[off..off + 32].to_vec(),
            bytes[off + 32..off + 64].to_vec(),
        );
        bytes[off..off + 32].copy_from_slice(&b);
        bytes[off + 32..off + 64].copy_from_slice(&a);
        assert!(decode::<SignedHeader>(&bytes).is_err());

        let mut bytes = h.encode();
        bytes[18..50].copy_from_slice(&[0u8; 32]); // the identity point: small order
        assert!(decode::<SignedHeader>(&bytes).is_err());
    }

    #[test]
    fn new_normalises_heads_and_caps_count() {
        let signer = IdentitySigner::from_seed(&[1u8; 32]);
        let dup = vec![Hash32::from_bytes([5u8; 32]); 3];
        let h = SignedHeader::new(
            kind::VOTE_OPEN,
            GroupId([0; 16]),
            signer.public(),
            0,
            Hash32::ZERO,
            dup,
        )
        .unwrap();
        assert_eq!(h.heads.len(), 1);
        let many: Vec<Hash32> = (0..=MAX_HEADS as u8)
            .map(|i| Hash32::from_bytes([i; 32]))
            .collect();
        assert!(matches!(
            SignedHeader::new(
                kind::VOTE_OPEN,
                GroupId([0; 16]),
                signer.public(),
                0,
                Hash32::ZERO,
                many
            ),
            Err(HeaderError::TooManyHeads(_))
        ));
    }

    #[test]
    fn sign_and_verify_helpers_agree_and_bind_domain() {
        let signer = IdentitySigner::from_seed(&[4u8; 32]);
        let bytes = header().encode();
        let sig = sign_bytes(&signer, domain::ROSTER_OP, &bytes);
        assert!(verify_bytes(domain::ROSTER_OP, &signer.public(), &bytes, &sig).is_ok());
        assert!(verify_bytes(domain::VOTE_OPEN, &signer.public(), &bytes, &sig).is_err());
        let mut with_sig = bytes.clone();
        with_sig.extend_from_slice(&sig);
        assert_ne!(object_id(&with_sig), object_id(&bytes));
    }
}
