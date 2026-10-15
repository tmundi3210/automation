//! Post plaintext: what travels inside a group SenderKeyMessage or a 1:1
//! Double Ratchet message (docs/DESIGN.md §2.4 row "Post plaintext";
//! §1 row `objects/chat_post.rs`; BRIEF_v2 §5.4 "Each post carries the
//! sender's roster-head hashes inside the ciphertext" (D3 M4)).
//!
//! Layout: `ver u8 ‖ kind u8 ‖ n_heads u8 ‖ roster_heads [32]×n ‖ body`
//! where the body is the rest of the bytes (no length prefix: the
//! surrounding ciphertext bounds it). The heads are sorted ascending without
//! duplicates and at most [`MAX_HEADS`] (the same rule as the identity-signed
//! header, so one normaliser serves both). 1:1 posts carry `n_heads = 0`.
//!
//! Kinds (DESIGN §2.4; [`kind`]): text, task op, carried vote-open / ballot
//! / roster op, receipt, SKDM bundle, plus the 1:1 control kind
//! [`kind::POST_RECOG_SEED`] that re-seeds the recipient's recognition
//! epoch (DECISIONS §3 row (3,4,6,10): "1:1: Double Ratchet counter tags,
//! re-seeded at each DH step"; carried inside the encryption so a relay
//! never sees a seed).
//!
//! Typed bodies: [`TextBody`] (text ≤ [`MAX_TEXT_BYTES`] plus a preview
//! hash), [`Receipt`] (cumulative receipt with gaps, feeding "N messages
//! from X not here yet", BRIEF §5.9) and [`RecogSeedBody`]. Task ops and
//! carried objects are encoded by their own modules and travel here as
//! opaque bytes.
//!
//! Author binding: in a group the author is the chain owner (bound by the
//! SKDM atomic check, D3 K1); in 1:1 it is the session peer. Nothing in a
//! post is identity-signed (docs/decisions-assumed.md row 12).

use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::codec::{CodecError, Decode, Encode, ListItem, Reader, Writer, unknown_variant};
use crate::crypto::hash::Hash32;
use crate::objects::{MAX_HEADS, OBJ_VERSION};
use crate::wire::frame::Class;

/// Post kind discriminants (DESIGN §2.4 "Post plaintext" row), re-exported
/// from [`crate::objects::kind`] plus the 1:1 control kind defined here.
pub mod kind {
    pub use crate::objects::kind::{
        POST_CARRIED_BALLOT, POST_CARRIED_ROSTER_OP, POST_CARRIED_VOTE_OPEN, POST_RECEIPT,
        POST_SKDM_BUNDLE, POST_TASK_OP, POST_TEXT,
    };

    /// 1:1 control post: the sender's next recognition-epoch seed,
    /// `seed [32] ‖ LE32(epoch_id)` ([`super::RecogSeedBody`]). Never
    /// shown; registered in the receiver's tag table (DECISIONS §3 row
    /// (3,4,6,10)).
    pub const POST_RECOG_SEED: u8 = 0x37;

    /// Every kind a post may carry, ascending.
    pub const ALL: [u8; 8] = [
        POST_TEXT,
        POST_TASK_OP,
        POST_CARRIED_VOTE_OPEN,
        POST_CARRIED_BALLOT,
        POST_CARRIED_ROSTER_OP,
        POST_RECEIPT,
        POST_SKDM_BUNDLE,
        POST_RECOG_SEED,
    ];

    /// `true` for the kinds this version of the protocol knows.
    pub fn is_known(k: u8) -> bool {
        ALL.contains(&k)
    }
}

/// Largest text body, in UTF-8 bytes (DESIGN §2.4 "text str16 (≤2000 B)";
/// BRIEF §5.6 "C2 text ≤2 kB").
pub const MAX_TEXT_BYTES: usize = 2000;
/// Length of a conversation id in a [`Receipt`] (a group id or a 1:1
/// conversation id, 16 bytes).
pub const CONV_LEN: usize = 16;
/// Length of a recognition-epoch seed ([`crate::wire::recog::SEED_LEN`]).
pub const SEED_LEN: usize = 32;
/// Fixed bytes before the body: `ver ‖ kind ‖ n_heads`.
pub const POST_FIXED_LEN: usize = 3;

/// Why a post could not be built.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum PostError {
    #[error("unknown post kind {0:#04x}")]
    UnknownKind(u8),
    #[error("too many roster heads: {0}")]
    TooManyHeads(usize),
    #[error("text exceeds {MAX_TEXT_BYTES} bytes: {0}")]
    TextTooLong(usize),
    #[error("receipt gap list is not ascending and disjoint")]
    BadGaps,
}

/// One post's plaintext (module docs).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PostPlain {
    /// One of [`kind::ALL`].
    pub kind: u8,
    /// Sorted ascending, no duplicates, at most [`MAX_HEADS`]; empty for
    /// 1:1 posts.
    pub roster_heads: Vec<Hash32>,
    /// The kind's body; opaque at this layer.
    pub body: Vec<u8>,
}

impl PostPlain {
    /// Builds a post, normalising the heads (sort, dedupe) and rejecting an
    /// unknown kind or too many heads.
    pub fn new(kind: u8, mut roster_heads: Vec<Hash32>, body: Vec<u8>) -> Result<Self, PostError> {
        if !kind::is_known(kind) {
            return Err(PostError::UnknownKind(kind));
        }
        roster_heads.sort_unstable();
        roster_heads.dedup();
        if roster_heads.len() > MAX_HEADS {
            return Err(PostError::TooManyHeads(roster_heads.len()));
        }
        Ok(PostPlain {
            kind,
            roster_heads,
            body,
        })
    }

    /// A text post.
    pub fn text(roster_heads: Vec<Hash32>, body: &TextBody) -> Result<Self, PostError> {
        body.check()?;
        Self::new(kind::POST_TEXT, roster_heads, body.encode())
    }

    /// A receipt post (`n_heads = 0`: receipts are pairwise, DESIGN §2.4).
    pub fn receipt(receipt: &Receipt) -> Result<Self, PostError> {
        receipt.check()?;
        Self::new(kind::POST_RECEIPT, Vec::new(), receipt.encode())
    }

    /// The 1:1 recognition re-seed control post.
    pub fn recog_seed(seed: &[u8; SEED_LEN], epoch_id: u32) -> Self {
        let body = RecogSeedBody {
            seed: *seed,
            epoch_id,
        };
        PostPlain {
            kind: kind::POST_RECOG_SEED,
            roster_heads: Vec::new(),
            body: body.encode(),
        }
    }

    /// Relay class of the object carrying this post (BRIEF §5.6; DESIGN §1
    /// `groups/group.rs`): text is C2, everything else (task ops, receipts,
    /// carried control objects, SKDM bundles, control seeds) is C1.
    pub fn class(&self) -> Class {
        if self.kind == kind::POST_TEXT {
            Class::C2Text
        } else {
            Class::C1Control
        }
    }

    /// The typed text body, for a [`kind::POST_TEXT`] post.
    pub fn text_body(&self) -> Result<TextBody, CodecError> {
        if self.kind != kind::POST_TEXT {
            return Err(CodecError::Invalid("post kind is not text"));
        }
        crate::codec::decode(&self.body)
    }

    /// The typed receipt, for a [`kind::POST_RECEIPT`] post.
    pub fn receipt_body(&self) -> Result<Receipt, CodecError> {
        if self.kind != kind::POST_RECEIPT {
            return Err(CodecError::Invalid("post kind is not receipt"));
        }
        crate::codec::decode(&self.body)
    }

    /// The typed seed body, for a [`kind::POST_RECOG_SEED`] post.
    pub fn recog_seed_body(&self) -> Result<RecogSeedBody, CodecError> {
        if self.kind != kind::POST_RECOG_SEED {
            return Err(CodecError::Invalid("post kind is not recog seed"));
        }
        crate::codec::decode(&self.body)
    }

    fn check_heads(heads: &[Hash32]) -> Result<(), CodecError> {
        if heads.len() > MAX_HEADS {
            return Err(CodecError::Invalid("heads"));
        }
        if heads.windows(2).any(|w| w[0] >= w[1]) {
            return Err(CodecError::Invalid("heads"));
        }
        Ok(())
    }
}

impl Encode for PostPlain {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        if !kind::is_known(self.kind) {
            return Err(unknown_variant("post kind", self.kind));
        }
        Self::check_heads(&self.roster_heads)?;
        w.u8(OBJ_VERSION).u8(self.kind);
        w.list_u8(&self.roster_heads, |w, h| {
            w.bytes(h.as_bytes());
            Ok(())
        })?;
        w.raw(&self.body);
        Ok(())
    }
}

impl Decode for PostPlain {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let ver = r.u8()?;
        if ver != OBJ_VERSION {
            return Err(unknown_variant("post version", ver));
        }
        let kind = r.u8()?;
        if !kind::is_known(kind) {
            return Err(unknown_variant("post kind", kind));
        }
        let roster_heads: Vec<Hash32> = r.list_u8(|r| Ok(Hash32::from_bytes(r.bytes()?)))?;
        Self::check_heads(&roster_heads)?;
        let body = r.rest().to_vec();
        Ok(PostPlain {
            kind,
            roster_heads,
            body,
        })
    }
}

/// Text body: `text str16 (≤2000 B) ‖ preview_hash [32] (zero if none)`
/// (DESIGN §2.4).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextBody {
    pub text: String,
    /// Hash of the attached preview object, [`Hash32::ZERO`] when none.
    pub preview_hash: Hash32,
}

impl TextBody {
    pub fn new(text: impl Into<String>, preview_hash: Option<Hash32>) -> Result<Self, PostError> {
        let body = TextBody {
            text: text.into(),
            preview_hash: preview_hash.unwrap_or(Hash32::ZERO),
        };
        body.check()?;
        Ok(body)
    }

    pub fn check(&self) -> Result<(), PostError> {
        if self.text.len() > MAX_TEXT_BYTES {
            return Err(PostError::TextTooLong(self.text.len()));
        }
        Ok(())
    }

    /// The preview hash, `None` when the zero sentinel is set.
    pub fn preview(&self) -> Option<Hash32> {
        (!self.preview_hash.is_zero()).then_some(self.preview_hash)
    }
}

impl Encode for TextBody {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        if self.text.len() > MAX_TEXT_BYTES {
            return Err(CodecError::LengthOverflow {
                what: "text",
                max: MAX_TEXT_BYTES,
                got: self.text.len(),
            });
        }
        w.str16(&self.text)?;
        w.bytes(self.preview_hash.as_bytes());
        Ok(())
    }
}

impl Decode for TextBody {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let text = r.str16()?;
        if text.len() > MAX_TEXT_BYTES {
            return Err(CodecError::Invalid("text length"));
        }
        let preview_hash = Hash32::from_bytes(r.bytes()?);
        Ok(TextBody {
            text: text.to_owned(),
            preview_hash,
        })
    }
}

/// One missing counter range `from..=to` in a [`Receipt`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Gap {
    pub from: u32,
    pub to: u32,
}

impl Encode for Gap {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u32(self.from).u32(self.to);
        Ok(())
    }
}

impl Decode for Gap {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(Gap {
            from: r.u32()?,
            to: r.u32()?,
        })
    }
}

impl ListItem for Gap {}

/// Cumulative receipt: `conv [16] ‖ up_to_obj_seq u32 ‖ n_gaps u8 ‖ (from
/// u32 ‖ to u32)×n` (DESIGN §2.4 "Receipt"). "Everything up to
/// `up_to_obj_seq` arrived except the gaps", which feed "N messages from X
/// not here yet" (BRIEF §5.9) and let the sender show "May have expired ·
/// Resend" (D6 rule 7). Gaps are ascending, disjoint, each `from ≤ to ≤
/// up_to_obj_seq`, so equal receipts are byte-identical.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Receipt {
    pub conv: [u8; CONV_LEN],
    pub up_to_obj_seq: u32,
    pub gaps: Vec<Gap>,
}

impl Receipt {
    pub fn check(&self) -> Result<(), PostError> {
        let mut prev_end: Option<u32> = None;
        for g in &self.gaps {
            if g.from > g.to || g.to > self.up_to_obj_seq {
                return Err(PostError::BadGaps);
            }
            if let Some(end) = prev_end {
                if g.from <= end {
                    return Err(PostError::BadGaps);
                }
            }
            prev_end = Some(g.to);
        }
        Ok(())
    }

    /// Number of objects the receipt reports missing.
    pub fn missing(&self) -> u64 {
        self.gaps
            .iter()
            .map(|g| u64::from(g.to.saturating_sub(g.from)).saturating_add(1))
            .fold(0u64, u64::saturating_add)
    }
}

impl Encode for Receipt {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        self.check().map_err(|_| CodecError::Invalid("gaps"))?;
        w.bytes(&self.conv).u32(self.up_to_obj_seq);
        self.gaps.encode_to(w)
    }
}

impl Decode for Receipt {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let receipt = Receipt {
            conv: r.bytes()?,
            up_to_obj_seq: r.u32()?,
            gaps: Vec::decode_from(r)?,
        };
        receipt.check().map_err(|_| CodecError::Invalid("gaps"))?;
        Ok(receipt)
    }
}

/// Body of a [`kind::POST_RECOG_SEED`] post: `seed [32] ‖ LE32(epoch_id)`.
/// The receiver registers `(chain = the 1:1 direction key, epoch_id)` with
/// [`crate::wire::recog::RecogEpoch::from_seed`] at counter 0. The seed is
/// wiped on drop.
#[derive(Clone, PartialEq, Eq, Hash, Zeroize, ZeroizeOnDrop)]
pub struct RecogSeedBody {
    pub seed: [u8; SEED_LEN],
    pub epoch_id: u32,
}

impl core::fmt::Debug for RecogSeedBody {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RecogSeedBody(epoch {}, <seed redacted>)", self.epoch_id)
    }
}

impl Encode for RecogSeedBody {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.bytes(&self.seed).u32(self.epoch_id);
        Ok(())
    }
}

impl Decode for RecogSeedBody {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(RecogSeedBody {
            seed: r.bytes()?,
            epoch_id: r.u32()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::{decode, encode};
    use proptest::prelude::*;

    fn h(b: u8) -> Hash32 {
        Hash32::from_bytes([b; 32])
    }

    #[test]
    fn text_post_known_layout_and_roundtrip() {
        let body = TextBody::new("hi", Some(h(9))).unwrap();
        let p = PostPlain::text(vec![h(5), h(2), h(5)], &body).unwrap();
        assert_eq!(p.roster_heads, vec![h(2), h(5)], "sorted and deduped");
        let bytes = encode(&p);
        let mut expect = vec![1u8, 0x30, 2];
        expect.extend_from_slice(&[2u8; 32]);
        expect.extend_from_slice(&[5u8; 32]);
        expect.extend_from_slice(&[2, 0, b'h', b'i']);
        expect.extend_from_slice(&[9u8; 32]);
        assert_eq!(bytes, expect);
        assert_eq!(decode::<PostPlain>(&bytes), Ok(p.clone()));
        assert_eq!(p.text_body(), Ok(body));
        assert_eq!(p.class(), Class::C2Text);
        assert!(p.receipt_body().is_err());
    }

    #[test]
    fn one_to_one_posts_have_no_heads_and_control_kinds_are_c1() {
        let r = Receipt {
            conv: [7; 16],
            up_to_obj_seq: 10,
            gaps: vec![Gap { from: 2, to: 3 }, Gap { from: 7, to: 7 }],
        };
        let p = PostPlain::receipt(&r).unwrap();
        assert!(p.roster_heads.is_empty());
        assert_eq!(p.class(), Class::C1Control);
        assert_eq!(encode(&p)[2], 0, "n_heads = 0");
        assert_eq!(p.receipt_body(), Ok(r.clone()));
        assert_eq!(r.missing(), 3);
        let s = PostPlain::recog_seed(&[4u8; 32], 3);
        assert_eq!(s.kind, 0x37);
        assert_eq!(s.class(), Class::C1Control);
        let b = s.recog_seed_body().unwrap();
        assert_eq!(b.seed, [4u8; 32]);
        assert_eq!(b.epoch_id, 3);
        assert_eq!(&encode(&b)[32..], &[3, 0, 0, 0]);
        assert!(!format!("{b:?}").contains("04040404"));
    }

    #[test]
    fn decoder_rejects_version_kind_heads_and_text_length() {
        let p = PostPlain::new(kind::POST_TASK_OP, vec![h(1)], vec![1, 2, 3]).unwrap();
        let good = encode(&p);
        let mut bad = good.clone();
        bad[0] = 2;
        assert_eq!(
            decode::<PostPlain>(&bad),
            Err(unknown_variant("post version", 2u8))
        );
        let mut bad = good.clone();
        bad[1] = 0x38;
        assert_eq!(
            decode::<PostPlain>(&bad),
            Err(unknown_variant("post kind", 0x38u8))
        );
        assert_eq!(
            PostPlain::new(0x29, vec![], vec![]),
            Err(PostError::UnknownKind(0x29))
        );
        // unsorted heads
        let mut two = vec![1u8, 0x30, 2];
        two.extend_from_slice(&[5u8; 32]);
        two.extend_from_slice(&[2u8; 32]);
        assert_eq!(decode::<PostPlain>(&two), Err(CodecError::Invalid("heads")));
        // too many heads
        let many: Vec<Hash32> = (0..=MAX_HEADS as u8).map(h).collect();
        assert_eq!(
            PostPlain::new(kind::POST_TEXT, many, vec![]),
            Err(PostError::TooManyHeads(MAX_HEADS + 1))
        );
        // text too long
        let long = "x".repeat(MAX_TEXT_BYTES + 1);
        assert_eq!(
            TextBody::new(long.clone(), None),
            Err(PostError::TextTooLong(MAX_TEXT_BYTES + 1))
        );
        let mut w = Writer::new();
        w.str16(&long).unwrap();
        w.bytes(&[0u8; 32]);
        assert_eq!(
            decode::<TextBody>(w.as_bytes()),
            Err(CodecError::Invalid("text length"))
        );
        let ok = "x".repeat(MAX_TEXT_BYTES);
        assert!(TextBody::new(ok, None).is_ok());
        // empty body decodes (opaque kinds may be empty at this layer)
        assert_eq!(
            decode::<PostPlain>(&[1, 0x31, 0]),
            Ok(PostPlain {
                kind: 0x31,
                roster_heads: vec![],
                body: vec![]
            })
        );
        assert!(decode::<PostPlain>(&[1, 0x31]).is_err());
    }

    #[test]
    fn seed_body_is_zeroized_on_drop() {
        fn assert_zeroize_on_drop<T: zeroize::ZeroizeOnDrop>() {}
        assert_zeroize_on_drop::<RecogSeedBody>();
    }

    #[test]
    fn receipt_gap_rules() {
        let base = Receipt {
            conv: [0; 16],
            up_to_obj_seq: 5,
            gaps: vec![],
        };
        let bad = [
            vec![Gap { from: 3, to: 2 }],
            vec![Gap { from: 1, to: 6 }],
            vec![Gap { from: 1, to: 2 }, Gap { from: 2, to: 3 }],
            vec![Gap { from: 3, to: 3 }, Gap { from: 1, to: 1 }],
        ];
        for gaps in bad {
            let r = Receipt {
                gaps: gaps.clone(),
                ..base.clone()
            };
            assert_eq!(r.check(), Err(PostError::BadGaps), "{gaps:?}");
            assert!(r.try_encode().is_err());
            let mut w = Writer::new();
            w.bytes(&[0u8; 16]).u32(5);
            gaps.encode_to(&mut w).unwrap();
            assert_eq!(
                decode::<Receipt>(w.as_bytes()),
                Err(CodecError::Invalid("gaps"))
            );
        }
        let ok = Receipt {
            gaps: vec![Gap { from: 0, to: 1 }, Gap { from: 5, to: 5 }],
            ..base
        };
        assert_eq!(decode::<Receipt>(&encode(&ok)), Ok(ok));
    }

    proptest! {
        #[test]
        fn decoders_survive_random_bytes(bytes in prop::collection::vec(any::<u8>(), 0..300)) {
            if let Ok(p) = decode::<PostPlain>(&bytes) {
                prop_assert_eq!(encode(&p), bytes.clone());
            }
            if let Ok(t) = decode::<TextBody>(&bytes) {
                prop_assert_eq!(encode(&t), bytes.clone());
            }
            if let Ok(r) = decode::<Receipt>(&bytes) {
                prop_assert_eq!(encode(&r), bytes.clone());
            }
            if let Ok(s) = decode::<RecogSeedBody>(&bytes) {
                prop_assert_eq!(encode(&s), bytes.clone());
            }
        }

        #[test]
        fn posts_roundtrip(kind_idx in 0usize..8, heads in prop::collection::vec(any::<u8>(), 0..40), body in prop::collection::vec(any::<u8>(), 0..64)) {
            let heads: Vec<Hash32> = heads.into_iter().map(h).collect();
            let kind = kind::ALL[kind_idx];
            let n = heads.iter().collect::<std::collections::BTreeSet<_>>().len();
            match PostPlain::new(kind, heads, body) {
                Ok(p) => {
                    prop_assert!(n <= MAX_HEADS);
                    prop_assert_eq!(decode::<PostPlain>(&encode(&p)), Ok(p));
                }
                Err(e) => prop_assert_eq!(e, PostError::TooManyHeads(n)),
            }
        }
    }
}
