//! KeyCard: binds the Ed25519 identity to the backend's X25519 session key
//! (docs/DESIGN.md §1 row `crypto/keycard`, §2.4 row `KeyCard`; BRIEF_v2 §3
//! "one contact = one key"; D3 K1).
//!
//! Why it exists: the backend (libsignal or vodozemac) holds only X25519 keys,
//! while the identity people verify by QR and that signs roster ops is
//! Ed25519. A Sender Key is accepted only over a pairwise session whose X25519
//! key matches the sender's roster KeyCard, checked and stored in one atomic
//! step (BRIEF_v2 §5.2 third bullet; DESIGN §1 `groups/group.rs`
//! `accept_skdm`). This signature is what ties "the session I am talking to"
//! to "the member in the roster". It is not a second identity (DESIGN §12).
//!
//! Layout (DESIGN §2.4), fixed 128 bytes, written by hand so this module does
//! not depend on `codec`:
//!
//! | off | len | field |
//! |---|---|---|
//! | 0 | 32 | `ed` (admitted Ed25519 public key) |
//! | 32 | 32 | `x25519` (backend session public key) |
//! | 64 | 64 | `sig = Ed25519(ed_sk, "meshapp.v1.keycard" ‖ 0x00 ‖ x25519)` |
//!
//! [`KeyCard::from_bytes`] admits `ed` and verifies `sig` (DESIGN §2.4
//! "Admission: every Ed25519 key entering a KeyCard ... before any signature
//! check"), so a `KeyCard` value that came off the wire is always valid.

use super::ed25519::{
    IdentityKey, IdentitySigner, KeyReject, PUBLIC_KEY_LEN, SIGNATURE_LEN, SigError,
    admit_public_key, verify,
};

/// Domain string of the KeyCard signature (DESIGN §2.4).
pub const KEYCARD_DOMAIN: &str = "meshapp.v1.keycard";
/// X25519 public key length.
pub const X25519_LEN: usize = 32;
/// Encoded size: `ed [32] ‖ x25519 [32] ‖ sig [64]`.
pub const KEYCARD_LEN: usize = PUBLIC_KEY_LEN + X25519_LEN + SIGNATURE_LEN;

/// An identity's statement "my backend session key is `x25519`".
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct KeyCard {
    pub ed: IdentityKey,
    pub x25519: [u8; X25519_LEN],
    pub sig: [u8; SIGNATURE_LEN],
}

/// Why a KeyCard was not accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum KeyCardError {
    /// `ed` failed admission; the signature was not checked.
    #[error("keycard identity key rejected: {0}")]
    Key(#[from] KeyReject),
    #[error("keycard signature does not verify")]
    BadSignature,
}

impl From<SigError> for KeyCardError {
    fn from(e: SigError) -> Self {
        match e {
            SigError::Key(k) => KeyCardError::Key(k),
            SigError::Invalid => KeyCardError::BadSignature,
        }
    }
}

impl KeyCard {
    /// Sign `x25519` with the identity. The result always verifies.
    pub fn issue(signer: &IdentitySigner, x25519: [u8; X25519_LEN]) -> Self {
        let sig = signer.sign(KEYCARD_DOMAIN, &x25519);
        KeyCard {
            ed: signer.public(),
            x25519,
            sig,
        }
    }

    /// Re-admit `ed` and check `sig` over `KEYCARD_DOMAIN ‖ 0x00 ‖ x25519`.
    pub fn verify(&self) -> Result<(), KeyCardError> {
        verify(KEYCARD_DOMAIN, &self.ed, &self.x25519, &self.sig)?;
        Ok(())
    }

    pub fn to_bytes(&self) -> [u8; KEYCARD_LEN] {
        let mut out = [0u8; KEYCARD_LEN];
        out[..PUBLIC_KEY_LEN].copy_from_slice(self.ed.as_bytes());
        out[PUBLIC_KEY_LEN..PUBLIC_KEY_LEN + X25519_LEN].copy_from_slice(&self.x25519);
        out[PUBLIC_KEY_LEN + X25519_LEN..].copy_from_slice(&self.sig);
        out
    }

    /// Parse and verify. Admission of `ed` happens before the signature check.
    pub fn from_bytes(bytes: &[u8; KEYCARD_LEN]) -> Result<Self, KeyCardError> {
        let ed_bytes: [u8; PUBLIC_KEY_LEN] = bytes[..PUBLIC_KEY_LEN]
            .try_into()
            .expect("slice length is PUBLIC_KEY_LEN");
        let x25519: [u8; X25519_LEN] = bytes[PUBLIC_KEY_LEN..PUBLIC_KEY_LEN + X25519_LEN]
            .try_into()
            .expect("slice length is X25519_LEN");
        let sig: [u8; SIGNATURE_LEN] = bytes[PUBLIC_KEY_LEN + X25519_LEN..]
            .try_into()
            .expect("slice length is SIGNATURE_LEN");
        let ed = admit_public_key(&ed_bytes)?;
        let card = KeyCard { ed, x25519, sig };
        card.verify()?;
        Ok(card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signer() -> IdentitySigner {
        IdentitySigner::from_seed(&[7u8; 32])
    }

    fn x25519() -> [u8; 32] {
        core::array::from_fn(|i| 0xa0 ^ i as u8)
    }

    #[test]
    fn issue_verify_round_trip() {
        let card = KeyCard::issue(&signer(), x25519());
        assert_eq!(card.verify(), Ok(()));
        assert_eq!(card.ed, signer().public());
        let bytes = card.to_bytes();
        assert_eq!(bytes.len(), 128);
        assert_eq!(&bytes[..32], card.ed.as_bytes());
        assert_eq!(&bytes[32..64], &card.x25519);
        assert_eq!(&bytes[64..], &card.sig);
        assert_eq!(KeyCard::from_bytes(&bytes), Ok(card));
        // Issuing is deterministic (Ed25519 signing is).
        assert_eq!(KeyCard::issue(&signer(), x25519()), card);
    }

    #[test]
    fn signature_is_over_domain_nul_x25519() {
        let card = KeyCard::issue(&signer(), x25519());
        assert_eq!(
            verify(KEYCARD_DOMAIN, &card.ed, &card.x25519, &card.sig),
            Ok(())
        );
        assert_eq!(
            verify("meshapp.v1.endorse", &card.ed, &card.x25519, &card.sig),
            Err(SigError::Invalid)
        );
        assert_eq!(KEYCARD_DOMAIN, "meshapp.v1.keycard");
    }

    #[test]
    fn tampered_x25519_fails() {
        let card = KeyCard::issue(&signer(), x25519());
        let mut bytes = card.to_bytes();
        bytes[32] ^= 1;
        assert_eq!(KeyCard::from_bytes(&bytes), Err(KeyCardError::BadSignature));
        let mut tampered = card;
        tampered.x25519[31] ^= 0x80;
        assert_eq!(tampered.verify(), Err(KeyCardError::BadSignature));
    }

    #[test]
    fn tampered_signature_fails() {
        let card = KeyCard::issue(&signer(), x25519());
        for byte in [64usize, 95, 96, 127] {
            let mut bytes = card.to_bytes();
            bytes[byte] ^= 1;
            assert_eq!(
                KeyCard::from_bytes(&bytes),
                Err(KeyCardError::BadSignature),
                "{byte}"
            );
        }
        let mut zero_sig = card;
        zero_sig.sig = [0u8; 64];
        assert_eq!(zero_sig.verify(), Err(KeyCardError::BadSignature));
    }

    #[test]
    fn substituted_identity_fails() {
        // A card for the same x25519 signed by someone else, with the ed field
        // swapped to the victim's key, must not verify.
        let victim = signer();
        let attacker = IdentitySigner::from_seed(&[9u8; 32]);
        let forged = KeyCard {
            ed: victim.public(),
            x25519: x25519(),
            sig: KeyCard::issue(&attacker, x25519()).sig,
        };
        assert_eq!(forged.verify(), Err(KeyCardError::BadSignature));
    }

    #[test]
    fn weak_identity_fails_admission_before_signature_check() {
        let card = KeyCard::issue(&signer(), x25519());
        let mut bytes = card.to_bytes();
        // Order-8 point in place of the identity key.
        let weak = hex::decode("c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a")
            .unwrap();
        bytes[..32].copy_from_slice(&weak);
        assert_eq!(
            KeyCard::from_bytes(&bytes),
            Err(KeyCardError::Key(KeyReject::SmallOrder))
        );
        // The identity point.
        bytes[..32].copy_from_slice(&[0u8; 32]);
        bytes[0] = 1;
        assert_eq!(
            KeyCard::from_bytes(&bytes),
            Err(KeyCardError::Key(KeyReject::SmallOrder))
        );
    }

    #[test]
    fn non_canonical_or_off_curve_identity_fails_admission() {
        let card = KeyCard::issue(&signer(), x25519());
        let mut bytes = card.to_bytes();
        let non_canonical =
            hex::decode("eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f")
                .unwrap();
        bytes[..32].copy_from_slice(&non_canonical);
        assert_eq!(
            KeyCard::from_bytes(&bytes),
            Err(KeyCardError::Key(KeyReject::NonCanonical))
        );
        let off_curve = (2u8..=40)
            .map(|y| {
                let mut b = [0u8; 32];
                b[0] = y;
                b
            })
            .find(|b| admit_public_key(b) == Err(KeyReject::NotOnCurve))
            .expect("a y in 2..=40 that is not on the curve");
        bytes[..32].copy_from_slice(&off_curve);
        assert_eq!(
            KeyCard::from_bytes(&bytes),
            Err(KeyCardError::Key(KeyReject::NotOnCurve))
        );
    }

    #[test]
    fn error_messages() {
        assert_eq!(
            KeyCardError::Key(KeyReject::SmallOrder).to_string(),
            "keycard identity key rejected: public key has small order"
        );
        assert_eq!(
            KeyCardError::BadSignature.to_string(),
            "keycard signature does not verify"
        );
        assert_eq!(
            KeyCardError::from(SigError::Invalid),
            KeyCardError::BadSignature
        );
        assert_eq!(
            KeyCardError::from(SigError::Key(KeyReject::NonCanonical)),
            KeyCardError::Key(KeyReject::NonCanonical)
        );
    }
}
