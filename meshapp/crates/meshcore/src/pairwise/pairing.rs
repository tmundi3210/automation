//! QR pairing over a direct link: "Verified in person" that also starts the
//! encrypted session (docs/DESIGN.md §1 row `pairwise/pairing.rs`;
//! BRIEF_v2 §3 level 2 "Mutual QR scan (Briar-style); this also starts the
//! encrypted session", §6 step 2 "Verify one person now"; DECISIONS D6 rule
//! 2 "Verified pairs: an ephemeral-ephemeral handshake while both phones are
//! present at the QR scan. No PreKey message ever goes over the air for
//! them"; §3 row (4,5,6,7) "Bundles and envelope keys travel only: at the
//! QR scan ...").
//!
//! # What the QR code carries
//!
//! [`QrCode`] = [`QrPayload`] `{ver, keycard, envelope_pub, bundle}` ‖
//! `secret [32]`. The payload is public material (it is also sent over the
//! link); the secret is fresh per display and reaches the scanner only
//! through the camera. It is what makes the pairing *mutual* with one scan:
//! the displayer learns that the device it is talking to is the one that
//! physically read its screen, and the scanner already holds the
//! displayer's KeyCard from the same screen.
//!
//! # Flow (one scanner S, one displayer D, over a Noise NN link whose
//! handshake hash `hh` both sides pass in)
//!
//! ```text
//! S → D  Offer  { payload_S, link_nonce }
//! D → S  Accept { payload_D, proof_D }      S checks payload_D == scanned
//! S → D  Prove  { proof_S, first: PreKey DR message carrying S's recog seed }
//! D → S  Reply  { reply: Normal DR message carrying D's recog seed }
//! ```
//!
//! * `link_key = HKDF-SHA256(ikm = secret ‖ link_nonce, salt =
//!   SHA256(min(card_S, card_D) ‖ max(card_S, card_D)), info =
//!   "meshapp.v1.pairing.link" ‖ hh)` ([`link_key`]). It needs the
//!   displayer's fresh randomness (`secret`, off-air) and the scanner's
//!   (`link_nonce`), binds both KeyCards (the salt) and this very link
//!   session (`hh`). A radio attacker, even one in the middle of the Noise
//!   NN link, never learns `secret`, so it can neither forge a proof nor
//!   swap a KeyCard: a swapped card changes the salt and the proof fails.
//! * The proofs are [`contact_proof`]`(link_key, own role, hh)`, verified
//!   with the peer's role, so a proof reflected back never verifies.
//!   `link_key` is then stored on the contact as `k_pair` for L2 contact
//!   proofs on later links (DESIGN §2.3).
//! * After the proofs, S pins D's X25519 key (from the scanned KeyCard),
//!   processes D's bundle and sends the first Double Ratchet message (a
//!   PreKey message, over the direct link, never on air) carrying its
//!   recognition seed; D pins S's key (from the card the proof bound),
//!   decrypts it, which starts its side of the session, and replies with
//!   its own seed. Both end with [`Paired`], which the node stores on the
//!   contact with the badge **Verified in person** and hands to
//!   `PairwiseManager::adopt_pairing` to register the recognition epochs.
//!
//! Every step is [`PairingSession::step`]; it never reads a clock
//! (`now_mono` is passed through to the backend).

use rand::CryptoRng;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::codec::{CodecError, Decode, Encode, Reader, Writer, decode, unknown_variant};
use crate::crypto::backend::{
    Backend, BackendError, PairwiseBackend, PairwiseCt, PairwiseKind, PeerId,
};
use crate::crypto::hash::{hkdf32, sha256_concat};
use crate::crypto::keycard::{KEYCARD_LEN, KeyCard};
use crate::objects::chat_post::{PostPlain, kind};
use crate::storage::kv::KvTxn;
use crate::transport::link::Role;
use crate::wire::frame::CONTACT_PROOF_LEN;
use crate::wire::noise::{HANDSHAKE_HASH_LEN, contact_proof, verify_contact_proof};
use crate::wire::recog::SEED_LEN;

/// Version byte of [`QrPayload`] and of every pairing message.
pub const QR_VERSION: u8 = 1;
/// Length of the QR secret and the link nonce.
pub const NONCE_LEN: usize = 32;
/// HKDF `info` prefix of [`link_key`], followed by the handshake hash.
pub const LINK_KEY_INFO: &[u8] = b"meshapp.v1.pairing.link";

/// What a QR code carries besides the secret (DESIGN §1 pairing row): the
/// KeyCard, the envelope public key for sealed first flights (D6 pick B
/// (2)) and a prekey bundle ([`crate::pairwise::bundles::IssueContext::QrScan`]).
/// Layout: `ver u8 ‖ keycard [128] ‖ envelope_pub [32] ‖ bundle bytes16`.
/// Decoding verifies the KeyCard (admission and signature).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QrPayload {
    pub ver: u8,
    pub keycard: KeyCard,
    pub envelope_pub: [u8; 32],
    pub bundle: Vec<u8>,
}

impl QrPayload {
    pub fn new(keycard: KeyCard, envelope_pub: [u8; 32], bundle: Vec<u8>) -> Self {
        QrPayload {
            ver: QR_VERSION,
            keycard,
            envelope_pub,
            bundle,
        }
    }
}

impl Encode for QrPayload {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        if self.ver != QR_VERSION {
            return Err(unknown_variant("qr version", self.ver));
        }
        w.u8(self.ver)
            .bytes(&self.keycard.to_bytes())
            .bytes(&self.envelope_pub);
        w.bytes16(&self.bundle)
    }
}

impl Decode for QrPayload {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let ver = r.u8()?;
        if ver != QR_VERSION {
            return Err(unknown_variant("qr version", ver));
        }
        let card: [u8; KEYCARD_LEN] = r.bytes()?;
        let keycard = KeyCard::from_bytes(&card).map_err(|_| CodecError::Invalid("keycard"))?;
        Ok(QrPayload {
            ver,
            keycard,
            envelope_pub: r.bytes()?,
            bundle: r.bytes16()?.to_vec(),
        })
    }
}

/// What the screen shows: the payload and the fresh per-display secret.
/// Layout: `QrPayload ‖ secret [32]`. The secret is wiped on drop.
#[derive(Clone, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct QrCode {
    #[zeroize(skip)]
    pub payload: QrPayload,
    pub secret: [u8; NONCE_LEN],
}

impl core::fmt::Debug for QrCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "QrCode({:?}, <secret redacted>)", self.payload)
    }
}

impl QrCode {
    /// A code for this display with a fresh secret.
    pub fn generate(payload: QrPayload, rng: &mut dyn CryptoRng) -> Self {
        let mut secret = [0u8; NONCE_LEN];
        rng.fill_bytes(&mut secret);
        QrCode { payload, secret }
    }
}

impl Encode for QrCode {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        self.payload.encode_to(w)?;
        w.bytes(&self.secret);
        Ok(())
    }
}

impl Decode for QrCode {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(QrCode {
            payload: QrPayload::decode_from(r)?,
            secret: r.bytes()?,
        })
    }
}

/// `HKDF-SHA256(ikm = secret ‖ link_nonce, salt = SHA256(min(card_a, card_b)
/// ‖ max(card_a, card_b)), info = "meshapp.v1.pairing.link" ‖ hh)` (module
/// docs). Symmetric in the two cards.
pub fn link_key(
    card_a: &KeyCard,
    card_b: &KeyCard,
    secret: &[u8; NONCE_LEN],
    link_nonce: &[u8; NONCE_LEN],
    handshake_hash: &[u8; HANDSHAKE_HASH_LEN],
) -> [u8; 32] {
    let (a, b) = (card_a.to_bytes(), card_b.to_bytes());
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    let salt = sha256_concat(&[&lo, &hi]);
    let mut ikm = [0u8; 2 * NONCE_LEN];
    ikm[..NONCE_LEN].copy_from_slice(secret);
    ikm[NONCE_LEN..].copy_from_slice(link_nonce);
    let mut info = [0u8; LINK_KEY_INFO.len() + HANDSHAKE_HASH_LEN];
    info[..LINK_KEY_INFO.len()].copy_from_slice(LINK_KEY_INFO);
    info[LINK_KEY_INFO.len()..].copy_from_slice(handshake_hash);
    hkdf32(&ikm, salt.as_bytes(), &info)
}

/// The other side's role on the link.
pub const fn peer_role(mine: Role) -> Role {
    match mine {
        Role::Central => Role::Peripheral,
        Role::Peripheral => Role::Central,
    }
}

/// Messages on the direct link (module docs "Flow"), each `ver u8 ‖ tag u8 ‖
/// fields`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairingMsg {
    Offer {
        payload: QrPayload,
        link_nonce: [u8; NONCE_LEN],
    },
    Accept {
        payload: QrPayload,
        proof: [u8; CONTACT_PROOF_LEN],
    },
    Prove {
        proof: [u8; CONTACT_PROOF_LEN],
        first: PairwiseCt,
    },
    Reply {
        reply: PairwiseCt,
    },
}

impl PairingMsg {
    const OFFER: u8 = 1;
    const ACCEPT: u8 = 2;
    const PROVE: u8 = 3;
    const REPLY: u8 = 4;
}

impl Encode for PairingMsg {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(QR_VERSION);
        match self {
            PairingMsg::Offer {
                payload,
                link_nonce,
            } => {
                w.u8(Self::OFFER);
                payload.encode_to(w)?;
                w.bytes(link_nonce);
            }
            PairingMsg::Accept { payload, proof } => {
                w.u8(Self::ACCEPT);
                payload.encode_to(w)?;
                w.bytes(proof);
            }
            PairingMsg::Prove { proof, first } => {
                w.u8(Self::PROVE).bytes(proof);
                first.encode_to(w)?;
            }
            PairingMsg::Reply { reply } => {
                w.u8(Self::REPLY);
                reply.encode_to(w)?;
            }
        }
        Ok(())
    }
}

impl Decode for PairingMsg {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let ver = r.u8()?;
        if ver != QR_VERSION {
            return Err(unknown_variant("pairing version", ver));
        }
        Ok(match r.u8()? {
            Self::OFFER => PairingMsg::Offer {
                payload: QrPayload::decode_from(r)?,
                link_nonce: r.bytes()?,
            },
            Self::ACCEPT => PairingMsg::Accept {
                payload: QrPayload::decode_from(r)?,
                proof: r.bytes()?,
            },
            Self::PROVE => PairingMsg::Prove {
                proof: r.bytes()?,
                first: PairwiseCt::decode_from(r)?,
            },
            Self::REPLY => PairingMsg::Reply {
                reply: PairwiseCt::decode_from(r)?,
            },
            other => return Err(unknown_variant("pairing message", other)),
        })
    }
}

/// Why pairing failed. The UI says what to do next ("Scan again"), never
/// "Failed" (BRIEF §7).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PairingError {
    #[error("pairing message: {0}")]
    Codec(#[from] CodecError),
    /// A message that this state does not expect.
    #[error("unexpected pairing message")]
    UnexpectedMessage,
    #[error("pairing already finished")]
    Finished,
    /// The device on the link did not present the scanned code.
    #[error("the link peer is not the scanned device")]
    PayloadMismatch,
    /// The peer could not prove knowledge of the link key.
    #[error("contact proof did not verify")]
    BadProof,
    /// The first ratchet message was not the expected kind or did not carry
    /// a recognition seed.
    #[error("unexpected first ratchet message")]
    BadFirstMessage,
    /// The scanned card is this node's own.
    #[error("cannot pair with oneself")]
    SelfPairing,
    #[error("backend: {0}")]
    Backend(#[from] BackendError),
}

/// The result of a completed pairing: what the node stores on the contact
/// (badge **Verified in person**, BRIEF §3) and hands to the pairwise
/// manager. The link key and the seeds are wiped on drop.
#[derive(Clone, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct Paired {
    #[zeroize(skip)]
    pub peer: PeerId,
    /// The peer's X25519 session key (now pinned in the backend).
    pub x25519: [u8; 32],
    /// `k_pair` for L2 contact proofs on later links (DESIGN §2.3).
    pub link_key: [u8; 32],
    /// The peer's envelope public key (D6 pick B (2)).
    pub envelope_pub: [u8; 32],
    /// Seed of this node's send epoch 0 towards the peer.
    pub send_seed: [u8; SEED_LEN],
    /// Seed of the peer's send epoch 0 towards this node.
    pub recv_seed: [u8; SEED_LEN],
}

impl core::fmt::Debug for Paired {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Paired(peer {}, x25519 {}, envelope {}, <keys redacted>)",
            self.peer,
            hex::encode(self.x25519),
            hex::encode(self.envelope_pub)
        )
    }
}

/// What the caller does after a [`PairingSession::step`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairingStep {
    /// Send these bytes over the direct link.
    Send(Vec<u8>),
    /// Pairing completed.
    Done(Paired),
    /// Nothing to send; wait for the peer's next message.
    Wait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Scanner,
    Displayer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    /// Scanner: about to send the Offer.
    Start,
    /// Scanner: Offer sent, waiting for Accept.
    AwaitAccept,
    /// Scanner: Prove sent, waiting for Reply.
    AwaitReply,
    /// Displayer: waiting for the Offer.
    AwaitOffer,
    /// Displayer: Accept sent, waiting for Prove.
    AwaitProve,
    /// Displayer: Reply sent; the next step reports Done.
    ReplySent,
    Finished,
}

/// The pairing state machine for one side (module docs). Its secrets (the
/// QR secret, the link nonce, the link key and both seeds) are wiped on
/// drop.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct PairingSession {
    #[zeroize(skip)]
    side: Side,
    #[zeroize(skip)]
    stage: Stage,
    #[zeroize(skip)]
    role: Role,
    #[zeroize(skip)]
    hh: [u8; HANDSHAKE_HASH_LEN],
    #[zeroize(skip)]
    mine: QrPayload,
    /// Scanner: the scanned code. Displayer: its own displayed secret.
    secret: [u8; NONCE_LEN],
    /// Scanner: the scanned payload; displayer: filled from the Offer.
    #[zeroize(skip)]
    theirs: Option<QrPayload>,
    link_nonce: [u8; NONCE_LEN],
    link_key: Option<[u8; 32]>,
    send_seed: [u8; SEED_LEN],
    recv_seed: Option<[u8; SEED_LEN]>,
}

impl core::fmt::Debug for PairingSession {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "PairingSession({:?}, {:?}, {:?}, <secrets redacted>)",
            self.side, self.stage, self.role
        )
    }
}

impl PairingSession {
    /// The scanner's session after scanning `scanned` (the peer's code),
    /// on a link in which this node has `role` and whose Noise handshake
    /// hash is `handshake_hash`. `mine` is this node's own payload, sent
    /// back over the link.
    pub fn scanner(
        mine: QrPayload,
        scanned: QrCode,
        role: Role,
        handshake_hash: [u8; HANDSHAKE_HASH_LEN],
        rng: &mut dyn CryptoRng,
    ) -> Result<Self, PairingError> {
        if scanned.payload.keycard.ed == mine.keycard.ed {
            return Err(PairingError::SelfPairing);
        }
        let mut link_nonce = [0u8; NONCE_LEN];
        rng.fill_bytes(&mut link_nonce);
        let mut send_seed = [0u8; SEED_LEN];
        rng.fill_bytes(&mut send_seed);
        Ok(PairingSession {
            side: Side::Scanner,
            stage: Stage::Start,
            role,
            hh: handshake_hash,
            mine,
            secret: scanned.secret,
            theirs: Some(scanned.payload.clone()),
            link_nonce,
            link_key: None,
            send_seed,
            recv_seed: None,
        })
    }

    /// The displayer's session for the code `shown` on its screen.
    pub fn displayer(
        shown: QrCode,
        role: Role,
        handshake_hash: [u8; HANDSHAKE_HASH_LEN],
        rng: &mut dyn CryptoRng,
    ) -> Self {
        let mut send_seed = [0u8; SEED_LEN];
        rng.fill_bytes(&mut send_seed);
        PairingSession {
            side: Side::Displayer,
            stage: Stage::AwaitOffer,
            role,
            hh: handshake_hash,
            mine: shown.payload.clone(),
            secret: shown.secret,
            theirs: None,
            link_nonce: [0u8; NONCE_LEN],
            link_key: None,
            send_seed,
            recv_seed: None,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.stage == Stage::Finished
    }

    /// Advances the machine with the peer's next message (or none).
    pub fn step(
        &mut self,
        txn: &mut dyn KvTxn,
        backend: &dyn Backend,
        incoming: Option<&[u8]>,
        now_mono: u64,
        rng: &mut dyn CryptoRng,
    ) -> Result<PairingStep, PairingError> {
        let msg = match incoming {
            Some(bytes) => Some(decode::<PairingMsg>(bytes)?),
            None => None,
        };
        match (self.stage, msg) {
            (Stage::Finished, _) => Err(PairingError::Finished),
            (Stage::Start, None) => {
                self.stage = Stage::AwaitAccept;
                Ok(PairingStep::Send(
                    PairingMsg::Offer {
                        payload: self.mine.clone(),
                        link_nonce: self.link_nonce,
                    }
                    .encode(),
                ))
            }
            (Stage::AwaitAccept, Some(PairingMsg::Accept { payload, proof })) => {
                let theirs = self.theirs.clone().ok_or(PairingError::UnexpectedMessage)?;
                if payload != theirs {
                    return Err(PairingError::PayloadMismatch);
                }
                let key = self.derive_link_key(&theirs);
                if !verify_contact_proof(&key, peer_role(self.role), &self.hh, &proof) {
                    return Err(PairingError::BadProof);
                }
                self.link_key = Some(key);
                // Pin, start the session from the scanned bundle, first message.
                backend.pin_peer_x25519(txn, &theirs.keycard.ed, &theirs.keycard.x25519)?;
                backend.process_prekey_bundle(
                    txn,
                    &theirs.keycard.ed,
                    &theirs.bundle,
                    now_mono,
                    rng,
                )?;
                let seed_post = PostPlain::recog_seed(&self.send_seed, 0).encode();
                let first = PairwiseBackend::encrypt(
                    backend,
                    txn,
                    &theirs.keycard.ed,
                    &seed_post,
                    now_mono,
                    rng,
                )?;
                if first.kind != PairwiseKind::PreKey {
                    return Err(PairingError::BadFirstMessage);
                }
                let proof = contact_proof(&key, self.role, &self.hh);
                self.stage = Stage::AwaitReply;
                Ok(PairingStep::Send(
                    PairingMsg::Prove { proof, first }.encode(),
                ))
            }
            (Stage::AwaitReply, Some(PairingMsg::Reply { reply })) => {
                let theirs = self.theirs.clone().ok_or(PairingError::UnexpectedMessage)?;
                let plain =
                    PairwiseBackend::decrypt(backend, txn, &theirs.keycard.ed, &reply, rng)?;
                let seed = Self::seed_from(&plain)?;
                self.recv_seed = Some(seed);
                self.stage = Stage::Finished;
                Ok(PairingStep::Done(self.paired(&theirs)?))
            }
            (
                Stage::AwaitOffer,
                Some(PairingMsg::Offer {
                    payload,
                    link_nonce,
                }),
            ) => {
                if payload.keycard.ed == self.mine.keycard.ed {
                    return Err(PairingError::SelfPairing);
                }
                self.link_nonce = link_nonce;
                let key = self.derive_link_key(&payload);
                self.link_key = Some(key);
                self.theirs = Some(payload);
                let proof = contact_proof(&key, self.role, &self.hh);
                self.stage = Stage::AwaitProve;
                Ok(PairingStep::Send(
                    PairingMsg::Accept {
                        payload: self.mine.clone(),
                        proof,
                    }
                    .encode(),
                ))
            }
            (Stage::AwaitProve, Some(PairingMsg::Prove { proof, first })) => {
                let theirs = self.theirs.clone().ok_or(PairingError::UnexpectedMessage)?;
                let key = self.link_key.ok_or(PairingError::UnexpectedMessage)?;
                if !verify_contact_proof(&key, peer_role(self.role), &self.hh, &proof) {
                    return Err(PairingError::BadProof);
                }
                if first.kind != PairwiseKind::PreKey {
                    return Err(PairingError::BadFirstMessage);
                }
                backend.pin_peer_x25519(txn, &theirs.keycard.ed, &theirs.keycard.x25519)?;
                let plain =
                    PairwiseBackend::decrypt(backend, txn, &theirs.keycard.ed, &first, rng)?;
                let seed = Self::seed_from(&plain)?;
                self.recv_seed = Some(seed);
                let seed_post = PostPlain::recog_seed(&self.send_seed, 0).encode();
                let reply = PairwiseBackend::encrypt(
                    backend,
                    txn,
                    &theirs.keycard.ed,
                    &seed_post,
                    now_mono,
                    rng,
                )?;
                self.stage = Stage::ReplySent;
                Ok(PairingStep::Send(PairingMsg::Reply { reply }.encode()))
            }
            (Stage::ReplySent, None) => {
                let theirs = self.theirs.clone().ok_or(PairingError::UnexpectedMessage)?;
                self.stage = Stage::Finished;
                Ok(PairingStep::Done(self.paired(&theirs)?))
            }
            (
                Stage::AwaitAccept | Stage::AwaitReply | Stage::AwaitOffer | Stage::AwaitProve,
                None,
            ) => Ok(PairingStep::Wait),
            (_, Some(_)) => Err(PairingError::UnexpectedMessage),
        }
    }

    fn derive_link_key(&self, theirs: &QrPayload) -> [u8; 32] {
        link_key(
            &self.mine.keycard,
            &theirs.keycard,
            &self.secret,
            &self.link_nonce,
            &self.hh,
        )
    }

    fn seed_from(plain: &[u8]) -> Result<[u8; SEED_LEN], PairingError> {
        let post: PostPlain = decode(plain).map_err(|_| PairingError::BadFirstMessage)?;
        if post.kind != kind::POST_RECOG_SEED {
            return Err(PairingError::BadFirstMessage);
        }
        let body = post
            .recog_seed_body()
            .map_err(|_| PairingError::BadFirstMessage)?;
        if body.epoch_id != 0 {
            return Err(PairingError::BadFirstMessage);
        }
        Ok(body.seed)
    }

    fn paired(&self, theirs: &QrPayload) -> Result<Paired, PairingError> {
        Ok(Paired {
            peer: theirs.keycard.ed,
            x25519: theirs.keycard.x25519,
            link_key: self.link_key.ok_or(PairingError::UnexpectedMessage)?,
            envelope_pub: theirs.envelope_pub,
            send_seed: self.send_seed,
            recv_seed: self.recv_seed.ok_or(PairingError::UnexpectedMessage)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::encode;
    use crate::crypto::ed25519::IdentitySigner;
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    fn card(seed: u8) -> KeyCard {
        KeyCard::issue(&IdentitySigner::from_seed(&[seed; 32]), [seed ^ 0xa5; 32])
    }

    fn payload(seed: u8) -> QrPayload {
        QrPayload::new(card(seed), [seed; 32], vec![seed; 5])
    }

    #[test]
    fn qr_payload_layout_roundtrip_and_keycard_check() {
        let p = payload(1);
        let bytes = encode(&p);
        assert_eq!(bytes.len(), 1 + 128 + 32 + 2 + 5);
        assert_eq!(bytes[0], 1);
        assert_eq!(decode::<QrPayload>(&bytes), Ok(p.clone()));
        let mut bad = bytes.clone();
        bad[0] = 2;
        assert_eq!(
            decode::<QrPayload>(&bad),
            Err(unknown_variant("qr version", 2u8))
        );
        let mut bad = bytes.clone();
        bad[1 + 64] ^= 1; // signature byte
        assert_eq!(
            decode::<QrPayload>(&bad),
            Err(CodecError::Invalid("keycard"))
        );
        let code = QrCode::generate(p.clone(), &mut ChaCha20Rng::seed_from_u64(1));
        let cb = encode(&code);
        assert_eq!(cb.len(), bytes.len() + 32);
        assert_eq!(decode::<QrCode>(&cb), Ok(code.clone()));
        assert!(!format!("{code:?}").contains(&hex::encode(code.secret)));
        let mut wrong_ver = p;
        wrong_ver.ver = 3;
        assert!(wrong_ver.try_encode().is_err());
    }

    #[test]
    fn secret_carriers_are_zeroized_on_drop() {
        fn assert_zeroize_on_drop<T: zeroize::ZeroizeOnDrop>() {}
        assert_zeroize_on_drop::<QrCode>();
        assert_zeroize_on_drop::<Paired>();
        assert_zeroize_on_drop::<PairingSession>();
    }

    #[test]
    fn messages_roundtrip_and_reject_unknown() {
        let msgs = [
            PairingMsg::Offer {
                payload: payload(1),
                link_nonce: [9; 32],
            },
            PairingMsg::Accept {
                payload: payload(2),
                proof: [8; 16],
            },
            PairingMsg::Prove {
                proof: [7; 16],
                first: PairwiseCt {
                    kind: PairwiseKind::PreKey,
                    bytes: vec![1, 2],
                },
            },
            PairingMsg::Reply {
                reply: PairwiseCt {
                    kind: PairwiseKind::Normal,
                    bytes: vec![],
                },
            },
        ];
        for (i, m) in msgs.iter().enumerate() {
            let bytes = encode(m);
            assert_eq!(bytes[0], 1);
            assert_eq!(bytes[1], i as u8 + 1);
            assert_eq!(decode::<PairingMsg>(&bytes), Ok(m.clone()));
        }
        assert_eq!(
            decode::<PairingMsg>(&[1, 5]),
            Err(unknown_variant("pairing message", 5u8))
        );
        assert_eq!(
            decode::<PairingMsg>(&[2, 1]),
            Err(unknown_variant("pairing version", 2u8))
        );
    }

    #[test]
    fn link_key_is_symmetric_and_needs_every_input() {
        let (a, b) = (card(1), card(2));
        let k = link_key(&a, &b, &[1; 32], &[2; 32], &[3; 32]);
        assert_eq!(k, link_key(&b, &a, &[1; 32], &[2; 32], &[3; 32]));
        assert_ne!(k, link_key(&a, &b, &[9; 32], &[2; 32], &[3; 32]));
        assert_ne!(k, link_key(&a, &b, &[1; 32], &[9; 32], &[3; 32]));
        assert_ne!(k, link_key(&a, &b, &[1; 32], &[2; 32], &[9; 32]));
        assert_ne!(k, link_key(&a, &card(3), &[1; 32], &[2; 32], &[3; 32]));
        // documented derivation
        let (lo, hi) = if a.to_bytes() <= b.to_bytes() {
            (a.to_bytes(), b.to_bytes())
        } else {
            (b.to_bytes(), a.to_bytes())
        };
        let mut salt_in = lo.to_vec();
        salt_in.extend_from_slice(&hi);
        let salt = crate::crypto::hash::sha256(&salt_in);
        let mut ikm = [1u8; 32].to_vec();
        ikm.extend_from_slice(&[2; 32]);
        let mut info = b"meshapp.v1.pairing.link".to_vec();
        info.extend_from_slice(&[3; 32]);
        assert_eq!(k, hkdf32(&ikm, salt.as_bytes(), &info));
        assert_eq!(peer_role(Role::Central), Role::Peripheral);
        assert_eq!(peer_role(Role::Peripheral), Role::Central);
    }

    #[test]
    fn state_machine_guards() {
        let mut rng = ChaCha20Rng::seed_from_u64(1);
        let backend = crate::crypto::backend::default_backend();
        let mut kv = crate::storage::kv::MemKv::new();
        let own = payload(1);
        let code = QrCode::generate(payload(1), &mut rng);
        assert_eq!(
            PairingSession::scanner(own.clone(), code, Role::Central, [0; 32], &mut rng).err(),
            Some(PairingError::SelfPairing)
        );
        let code = QrCode::generate(payload(2), &mut rng);
        let mut s =
            PairingSession::scanner(own.clone(), code, Role::Central, [0; 32], &mut rng).unwrap();
        assert!(!s.is_finished());
        assert!(format!("{s:?}").contains("Scanner"));
        // a message before the offer went out is unexpected
        let reply = PairingMsg::Reply {
            reply: PairwiseCt {
                kind: PairwiseKind::Normal,
                bytes: vec![],
            },
        }
        .encode();
        assert_eq!(
            s.step(&mut kv, &*backend, Some(&reply), 0, &mut rng).err(),
            Some(PairingError::UnexpectedMessage)
        );
        assert!(matches!(
            s.step(&mut kv, &*backend, None, 0, &mut rng),
            Ok(PairingStep::Send(_))
        ));
        assert_eq!(
            s.step(&mut kv, &*backend, None, 0, &mut rng),
            Ok(PairingStep::Wait)
        );
        // an Accept with another device's payload is a mismatch
        let other = PairingMsg::Accept {
            payload: payload(3),
            proof: [0; 16],
        }
        .encode();
        assert_eq!(
            s.step(&mut kv, &*backend, Some(&other), 0, &mut rng).err(),
            Some(PairingError::PayloadMismatch)
        );
        // the right payload with a bad proof
        let bad = PairingMsg::Accept {
            payload: payload(2),
            proof: [0; 16],
        }
        .encode();
        assert_eq!(
            s.step(&mut kv, &*backend, Some(&bad), 0, &mut rng).err(),
            Some(PairingError::BadProof)
        );
        assert!(kv.is_empty(), "nothing pinned before the proof verified");
        assert_eq!(
            s.step(&mut kv, &*backend, Some(&[1, 9]), 0, &mut rng).err(),
            Some(PairingError::Codec(unknown_variant("pairing message", 9u8)))
        );
        // displayer: self offer refused, waits otherwise
        let shown = QrCode::generate(payload(2), &mut rng);
        let mut d = PairingSession::displayer(shown, Role::Peripheral, [0; 32], &mut rng);
        assert_eq!(
            d.step(&mut kv, &*backend, None, 0, &mut rng),
            Ok(PairingStep::Wait)
        );
        let self_offer = PairingMsg::Offer {
            payload: payload(2),
            link_nonce: [0; 32],
        }
        .encode();
        assert_eq!(
            d.step(&mut kv, &*backend, Some(&self_offer), 0, &mut rng)
                .err(),
            Some(PairingError::SelfPairing)
        );
    }

    proptest! {
        #[test]
        fn decoders_survive_random_bytes(bytes in prop::collection::vec(any::<u8>(), 0..400)) {
            if let Ok(m) = decode::<PairingMsg>(&bytes) {
                prop_assert_eq!(encode(&m), bytes.clone());
            }
            if let Ok(p) = decode::<QrPayload>(&bytes) {
                prop_assert_eq!(encode(&p), bytes.clone());
            }
            if let Ok(c) = decode::<QrCode>(&bytes) {
                prop_assert_eq!(encode(&c), bytes.clone());
            }
        }
    }
}
