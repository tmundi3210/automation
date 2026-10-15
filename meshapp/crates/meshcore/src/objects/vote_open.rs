//! Vote opening (docs/DESIGN.md §2.4 "VoteOpen", §5; BRIEF_v2 §4 "Votes ·
//! Opening"; DECISIONS D1 rule 1, D2 rule 1).
//!
//! An identity-signed object: the common [`SignedHeader`] (kind
//! [`kind::VOTE_OPEN`], the opener as author, `heads` = `H`, the roster-DAG
//! frontier the opener holds) followed by the body
//!
//! ```text
//! question str16 (≤ 500 B) ‖ n_opt u8 (2..=8) ‖ options str16×n (each ≤ 80 B) ‖
//! f u8 ‖ T u16 ‖ N u16 ‖ sig [64]
//! ```
//!
//! `sig` is the opener's signature over `header ‖ body` under
//! [`domain::VOTE_OPEN`]. `vote_id = SHA256(bytes before sig)` (DESIGN
//! §2.4), so the same opening signed twice (iOS signatures are randomised,
//! D1 rule 2) has one vote id. The header's `author_seq`/`prev_own` put the
//! opening in the opener's sender chain: two openings with the same
//! `(opener, seq)` and different content are an equivocation that voids the
//! vote (D1 rule 1; `votes::open`).
//!
//! `T = ⌊(N + f)/2⌋ + 1` ([`threshold`]) is carried in the body so every
//! phone uses the same value, and recomputed locally by `votes::open`
//! (BRIEF_v2 §4 "f comes from the signed opening, so every phone uses the
//! same T"). The target time ("Target 18:00 · late votes still count") is UI
//! metadata, not part of the signed body (DESIGN §5 last bullet).

use crate::codec::{CodecError, Decode, Encode, ListItem, Reader, Writer, decode};
use crate::crypto::ed25519::{IdentityKey, IdentitySigner, SIGNATURE_LEN};
use crate::crypto::hash::{Hash32, sha256};
use crate::objects::{SignedHeader, domain, kind, object_id, sign_bytes, verify_bytes};

/// Largest question, in UTF-8 bytes.
pub const MAX_QUESTION_BYTES: usize = 500;
/// Largest option label, in UTF-8 bytes.
pub const MAX_OPTION_BYTES: usize = 80;
/// Fewest options an opening may offer (DESIGN §2.4 `n_opt u8 (2..=8)`).
pub const MIN_OPTIONS: usize = 2;
/// Most options an opening may offer.
pub const MAX_OPTIONS: usize = 8;

/// `T = ⌊(N + f)/2⌋ + 1` (BRIEF_v2 §4 "Final rule"; D1 rule 4). Never
/// overflows: `(65535 + 255)/2 + 1` fits a `u16`.
pub fn threshold(n: u16, f: u8) -> u16 {
    let sum = u32::from(n) + u32::from(f);
    // sum / 2 + 1 ≤ 32896
    u16::try_from(sum / 2 + 1).unwrap_or(u16::MAX)
}

/// Why a vote opening was refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum VoteOpenError {
    #[error("vote-open codec: {0}")]
    Codec(#[from] CodecError),
    /// `header.kind` is not [`kind::VOTE_OPEN`].
    #[error("header kind {0:#04x} is not a vote-open")]
    WrongKind(u8),
    /// `sign` was handed a header whose author is not the signer.
    #[error("header author is not the signing key")]
    AuthorMismatch,
    #[error("question exceeds {MAX_QUESTION_BYTES} bytes: {0}")]
    QuestionTooLong(usize),
    #[error("option count {0} outside {MIN_OPTIONS}..={MAX_OPTIONS}")]
    BadOptionCount(usize),
    #[error("option {index} exceeds {MAX_OPTION_BYTES} bytes: {len}")]
    OptionTooLong { index: usize, len: usize },
    /// The opener's signature over `header ‖ body` does not verify.
    #[error("vote-open signature does not verify")]
    BadSignature,
}

/// A signed vote opening (module docs).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoteOpen {
    /// Kind [`kind::VOTE_OPEN`]; `author` is the opener; `heads` is `H`.
    pub header: SignedHeader,
    pub question: String,
    /// 2..=8 labels, each ≤ [`MAX_OPTION_BYTES`]; a ballot's `choice` indexes this list.
    pub options: Vec<String>,
    /// Tolerance for members who send conflicting ballots (≥ `F_MIN` of the branch).
    pub f: u8,
    /// `threshold(n, f)`, carried so every phone uses the same value.
    pub t: u16,
    /// `|members(H)|` as the opener computed it.
    pub n: u16,
    pub sig: [u8; SIGNATURE_LEN],
}

impl ListItem for VoteOpen {}

impl VoteOpen {
    /// Signs an opening. `t` is passed explicitly rather than derived so a
    /// test can produce a correctly signed opening with a wrong `T`;
    /// production callers pass `threshold(n, f)`. Fails if the header is
    /// not a vote-open by this signer or a body cap is exceeded.
    pub fn sign(
        signer: &IdentitySigner,
        header: SignedHeader,
        question: impl Into<String>,
        options: Vec<String>,
        f: u8,
        t: u16,
        n: u16,
    ) -> Result<Self, VoteOpenError> {
        if header.author != signer.public() {
            return Err(VoteOpenError::AuthorMismatch);
        }
        let mut open = VoteOpen {
            header,
            question: question.into(),
            options,
            f,
            t,
            n,
            sig: [0u8; SIGNATURE_LEN],
        };
        open.check_wellformed()?;
        let bytes = open.signature_input()?;
        open.sig = sign_bytes(signer, domain::VOTE_OPEN, &bytes);
        Ok(open)
    }

    /// The opener.
    pub fn opener(&self) -> &IdentityKey {
        &self.header.author
    }

    /// `H`: the roster-DAG heads the opener held (the electorate is `members(H)`).
    pub fn h(&self) -> &[Hash32] {
        &self.header.heads
    }

    /// Format rules (module docs). Decoding enforces the same rules.
    pub fn check_wellformed(&self) -> Result<(), VoteOpenError> {
        if self.header.kind != kind::VOTE_OPEN {
            return Err(VoteOpenError::WrongKind(self.header.kind));
        }
        if self.question.len() > MAX_QUESTION_BYTES {
            return Err(VoteOpenError::QuestionTooLong(self.question.len()));
        }
        if !(MIN_OPTIONS..=MAX_OPTIONS).contains(&self.options.len()) {
            return Err(VoteOpenError::BadOptionCount(self.options.len()));
        }
        for (index, o) in self.options.iter().enumerate() {
            if o.len() > MAX_OPTION_BYTES {
                return Err(VoteOpenError::OptionTooLong {
                    index,
                    len: o.len(),
                });
            }
        }
        Ok(())
    }

    fn body_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.str16(&self.question)?;
        w.list_u8(&self.options, |w, o| w.str16(o))?;
        w.u8(self.f).u16(self.t).u16(self.n);
        Ok(())
    }

    /// `header ‖ body`: the bytes the opener signs and the preimage of the vote id.
    pub fn signature_input(&self) -> Result<Vec<u8>, CodecError> {
        let mut w = Writer::new();
        self.header.encode_to(&mut w)?;
        self.body_to(&mut w)?;
        Ok(w.into_inner())
    }

    /// `vote_id = SHA256(bytes before sig)` (DESIGN §2.4).
    ///
    /// # Panics
    ///
    /// Only if the opening violates a codec cap, which [`VoteOpen::sign`] and
    /// decoding rule out; [`VoteOpen::try_vote_id`] has no panic path.
    pub fn vote_id(&self) -> Hash32 {
        match self.try_vote_id() {
            Ok(id) => id,
            Err(e) => panic!("VoteOpen::vote_id on a malformed opening: {e}"),
        }
    }

    /// [`VoteOpen::vote_id`] without the panic path.
    pub fn try_vote_id(&self) -> Result<Hash32, CodecError> {
        Ok(sha256(&self.signature_input()?))
    }

    /// `SHA256(question)`: the key of the "Duplicate vote" flag (D1 rule 1).
    pub fn question_hash(&self) -> Hash32 {
        sha256(self.question.as_bytes())
    }

    /// `op_hash = SHA256(all bytes including sig)`, the hash the opener's
    /// sender chain records (DESIGN §2.4).
    pub fn try_op_hash(&self) -> Result<Hash32, CodecError> {
        Ok(object_id(&self.try_encode()?))
    }

    /// Format and signature check (DESIGN §2.4 "signature over all bytes
    /// with the domain prefix"; the opener key was admitted when the header
    /// was decoded or built).
    pub fn verify(&self) -> Result<(), VoteOpenError> {
        self.check_wellformed()?;
        let bytes = self.signature_input()?;
        verify_bytes(domain::VOTE_OPEN, &self.header.author, &bytes, &self.sig)
            .map_err(|_| VoteOpenError::BadSignature)
    }

    /// Decodes and fully verifies an opening from its bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VoteOpenError> {
        let open: VoteOpen = decode(bytes)?;
        open.verify()?;
        Ok(open)
    }
}

fn codec_err(e: VoteOpenError) -> CodecError {
    match e {
        VoteOpenError::Codec(c) => c,
        VoteOpenError::WrongKind(_) => CodecError::Invalid("vote-open kind"),
        VoteOpenError::QuestionTooLong(_) => CodecError::Invalid("question length"),
        VoteOpenError::BadOptionCount(_) => CodecError::Invalid("option count"),
        VoteOpenError::OptionTooLong { .. } => CodecError::Invalid("option length"),
        VoteOpenError::AuthorMismatch | VoteOpenError::BadSignature => {
            CodecError::Invalid("vote-open")
        }
    }
}

impl Encode for VoteOpen {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        self.check_wellformed().map_err(codec_err)?;
        self.header.encode_to(w)?;
        self.body_to(w)?;
        w.bytes(&self.sig);
        Ok(())
    }
}

impl Decode for VoteOpen {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let header = SignedHeader::decode_from(r)?;
        let question = r.str16()?.to_owned();
        let options = r.list_u8(|r| Ok(r.str16()?.to_owned()))?;
        let f = r.u8()?;
        let t = r.u16()?;
        let n = r.u16()?;
        let sig: [u8; SIGNATURE_LEN] = r.bytes()?;
        let open = VoteOpen {
            header,
            question,
            options,
            f,
            t,
            n,
            sig,
        };
        open.check_wellformed().map_err(codec_err)?;
        Ok(open)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::encode;
    use crate::crypto::backend::GroupId;
    use proptest::prelude::*;

    fn signer(i: u8) -> IdentitySigner {
        IdentitySigner::from_seed(&[i; 32])
    }

    fn header(signer: &IdentitySigner, k: u8) -> SignedHeader {
        SignedHeader::new(
            k,
            GroupId([3u8; 16]),
            signer.public(),
            4,
            Hash32::from_bytes([8u8; 32]),
            vec![Hash32::from_bytes([1u8; 32])],
        )
        .unwrap()
    }

    fn open(signer: &IdentitySigner) -> VoteOpen {
        VoteOpen::sign(
            signer,
            header(signer, kind::VOTE_OPEN),
            "Move the kitchen?",
            vec!["Yes".into(), "No".into()],
            1,
            threshold(12, 1),
            12,
        )
        .unwrap()
    }

    #[test]
    fn threshold_matches_the_brief_table() {
        // BRIEF_v2 §4: f=1 → 7/12, 8/13, 12/21, 21/40; f=2 → 8/12, 8/13, 12/21, 22/40
        assert_eq!(threshold(12, 1), 7);
        assert_eq!(threshold(13, 1), 8);
        assert_eq!(threshold(21, 1), 12);
        assert_eq!(threshold(40, 1), 21);
        assert_eq!(threshold(12, 2), 8);
        assert_eq!(threshold(13, 2), 8);
        assert_eq!(threshold(21, 2), 12);
        assert_eq!(threshold(40, 2), 22);
        assert_eq!(threshold(u16::MAX, u8::MAX), 32896);
    }

    #[test]
    fn sign_verify_roundtrip_and_ids() {
        let s = signer(1);
        let o = open(&s);
        assert!(o.verify().is_ok());
        let bytes = encode(&o);
        let back = VoteOpen::from_bytes(&bytes).unwrap();
        assert_eq!(back, o);
        assert_eq!(o.vote_id(), sha256(&bytes[..bytes.len() - 64]));
        assert_eq!(o.try_op_hash().unwrap(), object_id(&bytes));
        assert_eq!(o.question_hash(), sha256(b"Move the kitchen?"));
        assert_eq!(*o.opener(), s.public());
        assert_eq!(o.h(), &[Hash32::from_bytes([1u8; 32])][..]);
        // same content, fresh signature → same vote id, different op hash is allowed
        let again = open(&s);
        assert_eq!(again.vote_id(), o.vote_id());
    }

    #[test]
    fn verify_rejects_tampering_and_wrong_domain() {
        let s = signer(1);
        let o = open(&s);
        let mut t = o.clone();
        t.n = 13;
        assert_eq!(t.verify(), Err(VoteOpenError::BadSignature));
        let mut t = o.clone();
        t.options[0] = "Yes!".into();
        assert_eq!(t.verify(), Err(VoteOpenError::BadSignature));
        let mut t = o.clone();
        t.sig = sign_bytes(&s, domain::ROSTER_OP, &o.signature_input().unwrap());
        assert_eq!(t.verify(), Err(VoteOpenError::BadSignature));
        let mut t = o.clone();
        t.header.author = signer(2).public();
        assert_eq!(t.verify(), Err(VoteOpenError::BadSignature));
    }

    #[test]
    fn sign_enforces_header_and_caps() {
        let s = signer(1);
        let two = vec!["a".to_string(), "b".to_string()];
        assert_eq!(
            VoteOpen::sign(
                &signer(2),
                header(&s, kind::VOTE_OPEN),
                "q",
                two.clone(),
                1,
                2,
                2
            )
            .unwrap_err(),
            VoteOpenError::AuthorMismatch
        );
        assert_eq!(
            VoteOpen::sign(&s, header(&s, kind::ROSTER_ADD), "q", two.clone(), 1, 2, 2)
                .unwrap_err(),
            VoteOpenError::WrongKind(kind::ROSTER_ADD)
        );
        assert_eq!(
            VoteOpen::sign(
                &s,
                header(&s, kind::VOTE_OPEN),
                "q".repeat(MAX_QUESTION_BYTES + 1),
                two.clone(),
                1,
                2,
                2
            )
            .unwrap_err(),
            VoteOpenError::QuestionTooLong(MAX_QUESTION_BYTES + 1)
        );
        assert_eq!(
            VoteOpen::sign(
                &s,
                header(&s, kind::VOTE_OPEN),
                "q",
                vec!["a".into()],
                1,
                2,
                2
            )
            .unwrap_err(),
            VoteOpenError::BadOptionCount(1)
        );
        let nine = vec!["x".to_string(); MAX_OPTIONS + 1];
        assert_eq!(
            VoteOpen::sign(&s, header(&s, kind::VOTE_OPEN), "q", nine, 1, 2, 2).unwrap_err(),
            VoteOpenError::BadOptionCount(MAX_OPTIONS + 1)
        );
        let long = vec!["a".to_string(), "b".repeat(MAX_OPTION_BYTES + 1)];
        assert_eq!(
            VoteOpen::sign(&s, header(&s, kind::VOTE_OPEN), "q", long, 1, 2, 2).unwrap_err(),
            VoteOpenError::OptionTooLong {
                index: 1,
                len: MAX_OPTION_BYTES + 1
            }
        );
        let eight = (0..MAX_OPTIONS)
            .map(|i| "o".repeat(MAX_OPTION_BYTES - i))
            .collect();
        assert!(
            VoteOpen::sign(
                &s,
                header(&s, kind::VOTE_OPEN),
                "q".repeat(MAX_QUESTION_BYTES),
                eight,
                3,
                2,
                2
            )
            .is_ok()
        );
    }

    #[test]
    fn decoder_rejects_malformed_input() {
        let s = signer(1);
        let good = encode(&open(&s));
        // kind byte → not a vote-open
        let mut bad = good.clone();
        bad[1] = kind::ROSTER_ADD;
        assert_eq!(
            decode::<VoteOpen>(&bad),
            Err(CodecError::Invalid("vote-open kind"))
        );
        for cut in 0..good.len() {
            assert!(matches!(
                decode::<VoteOpen>(&good[..cut]),
                Err(CodecError::UnexpectedEof { .. })
            ));
        }
        let mut longer = good.clone();
        longer.push(0);
        assert_eq!(
            decode::<VoteOpen>(&longer),
            Err(CodecError::TrailingBytes(1))
        );
        // option count 1
        let hdr_len = 1 + 1 + 16 + 32 + 4 + 32 + 1 + 32;
        let q_len = 2 + "Move the kitchen?".len();
        let mut bad = good.clone();
        bad[hdr_len + q_len] = 1;
        assert!(decode::<VoteOpen>(&bad).is_err());
    }

    proptest! {
        #[test]
        fn decoder_never_panics_and_roundtrips(bytes in proptest::collection::vec(any::<u8>(), 0..400)) {
            if let Ok(o) = decode::<VoteOpen>(&bytes) {
                prop_assert!(o.check_wellformed().is_ok());
                prop_assert_eq!(encode(&o), bytes);
                // a random byte string essentially never carries a valid signature
                prop_assert!(o.verify().is_err());
            }
        }
    }
}
