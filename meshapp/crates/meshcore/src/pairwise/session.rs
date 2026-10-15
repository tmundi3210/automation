//! Pairwise (1:1) sessions over the backend: session state per contact,
//! crossed initiation, decrypt-failure detection, and the 1:1 recognition
//! epochs (docs/DESIGN.md §1 row `pairwise/session.rs`; DECISIONS D6 rules
//! 2, 5 and 7; §3 row (3,4,6,10) "1:1: Double Ratchet counter tags,
//! re-seeded at each DH step"; BRIEF_v2 §5.1 "1:1 messages use Double
//! Ratchet message authentication").
//!
//! # Session state (D6 rule 7, BRIEF §5.9: never "Failed")
//!
//! * [`SessionState::NeedsTheirKey`]: no bundle yet, the pin changed, or
//!   the backend reported `NoSession`/`UntrustedIdentity`, or
//!   [`DECRYPT_FAILURES_NEEDS_KEY_ASM`] consecutive messages from the peer
//!   failed to decrypt (D6 remaining risk: "decrypt-failure detection,
//!   automatic re-establishment with 'Needs their key'"). The caller re-runs
//!   the bundle.
//! * [`SessionState::Crossed`]: both sides sent a PreKey message before
//!   hearing the other (D6 rule 5). The backend keeps every session for
//!   decryption and converges on its own (`tests/backend.rs`
//!   `crossed_initiation_converges`); the state goes back to `Ready` on the
//!   first Normal message either way.
//! * [`SessionState::Ready`]: messages flow.
//!
//! # Recognition epochs (DECISIONS §3 row (3,4,6,10))
//!
//! Each side owns its **send** epoch: a chain named
//! [`pair_chain_key`]`(me, peer)` in the shared [`TagTable`], seeded by a
//! random 32-byte seed. The seed reaches the peer *inside* the encryption:
//! the first one during pairing (`pairwise::pairing`) or inside a sealed
//! first flight ([`FirstFlight`]), later ones as a control post of kind
//! [`kind::POST_RECOG_SEED`] (`seed ‖ LE32(epoch_id)`) sent under the
//! epoch it replaces. The receiver registers `(pair_chain_key(peer, me),
//! epoch_id)` at counter 0; the table keeps ≤ 3 epochs per chain
//! ([`crate::wire::recog::MAX_EPOCHS`]), so cells delayed across a re-seed
//! still match. A sender re-seeds every [`RECOG_RESEED_MESSAGES_ASM`]
//! messages or when asked ([`PairwiseManager::request_reseed`], e.g. after a
//! "phone was out of my hands" report). A relay never sees a seed.
//!
//! # SKDM entries (DECISIONS D3 K1–K2)
//!
//! Group Sender Key distribution (`groups::group`) encrypts and decrypts
//! its per-member entries through [`PairwiseManager::encrypt_to`] and
//! [`PairwiseManager::decrypt_from`], never through the backend directly:
//! an SKDM is often the first message to a co-member (a PreKey message),
//! and only this manager's path records `sent_prekey`/`got_prekey`, turns
//! the state `Ready` (or `Crossed`) and clears the decrypt-failure count,
//! so the session the SKDM started behaves like one started by a text.
//!
//! # Cells
//!
//! A 1:1 object is the [`PairwiseCt`] encoding, fragmented with
//! [`InnerKind::PairwiseDr`] under the send epoch's `(ctr, tag, cell key)`;
//! the body nonce uses fragment index 0 for the reason given in
//! `groups::group` (the counter already keys each cell). Receiving reverses
//! it through a per-chain [`Reassembler`].
//!
//! Nothing here reads a clock; `now_mono` goes to the backend.

use std::collections::BTreeMap;

use rand::CryptoRng;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

use crate::codec::{CodecError, Decode, Encode, Reader, Writer, decode};
use crate::crypto::backend::{
    Backend, BackendError, PairwiseBackend, PairwiseCt, PairwiseKind, PeerId,
};
use crate::crypto::hash::sha256_concat;
use crate::objects::chat_post::{PostPlain, kind};
use crate::storage::kv::KvTxn;
use crate::wire::cell::{self, Cell, CellReject, VERSION};
use crate::wire::inner::{self, InnerError, InnerKind, Reassembler};
use crate::wire::recog::{ChainKey, EpochId, Hit, RecogEpoch, RecogError, SEED_LEN, TagTable};

/// Messages a send epoch carries before the sender re-seeds (assumption;
/// DECISIONS §3 row (3,4,6,10) says "re-seeded at each DH step", which the
/// backend does not expose, so a message count stands in for it).
pub const RECOG_RESEED_MESSAGES_ASM: u32 = 1000;
/// Consecutive decrypt failures from one peer after which the session is
/// treated as desynchronised and shown as "Needs their key" (assumption).
pub const DECRYPT_FAILURES_NEEDS_KEY_ASM: u32 = 3;
/// Domain of the 1:1 chain-key derivation.
pub const PAIR_CHAIN_KEY_DOMAIN: &[u8] = b"meshapp.v1.chainkey.pair";

/// The local [`ChainKey`] of the direction `sender → receiver`:
/// `LE64(Trunc8(SHA256("meshapp.v1.chainkey.pair" ‖ sender.ed ‖
/// receiver.ed)))`. Both sides derive the same key for the same direction.
pub fn pair_chain_key(sender: &PeerId, receiver: &PeerId) -> ChainKey {
    let h = sha256_concat(&[
        PAIR_CHAIN_KEY_DOMAIN,
        sender.as_bytes(),
        receiver.as_bytes(),
    ]);
    ChainKey(u64::from_le_bytes(h.truncate8()))
}

/// Per-contact session state (module docs). The UI maps `NeedsTheirKey` to
/// the §7 term "Needs their key".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionState {
    NeedsTheirKey,
    Ready,
    Crossed,
}

/// Why a pairwise operation did not apply.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PairwiseError {
    /// No usable session with the peer: get its bundle (D6 rule 7).
    #[error("needs their key")]
    NeedsTheirKey,
    /// The ciphertext did not decrypt; no detail (BRIEF §5.9).
    #[error("message did not decrypt")]
    Decrypt,
    /// This node has no send epoch for the peer yet (pairing or a first
    /// flight delivers the first seed).
    #[error("no recognition send epoch for this peer")]
    NoSendEpoch,
    /// A cell was recognised under a chain no contact owns.
    #[error("recognised chain is not a 1:1 chain")]
    UnknownChain,
    /// The object is not a pairwise message.
    #[error("unexpected object kind")]
    WrongKind,
    #[error("object counter exhausted")]
    ObjSeqExhausted,
    #[error("backend: {0}")]
    Backend(BackendError),
    #[error("codec: {0}")]
    Codec(#[from] CodecError),
    #[error("fragment layer: {0}")]
    Inner(#[from] InnerError),
    #[error("recognition: {0}")]
    Recog(#[from] RecogError),
    #[error("cell: {0}")]
    Cell(#[from] CellReject),
}

/// What a decrypted 1:1 message was.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Received {
    /// A post for the application (text, receipt, SKDM entry, carried op).
    Post(PostPlain),
    /// A recognition re-seed; the new receive epoch is already registered.
    RecogSeed { epoch_id: u32 },
}

/// The payload of a sealed first flight (DESIGN §2.4 "Sealed first flight":
/// `PreKeySignalMessage bytes ‖ recog_seed [32] ‖ envelope_pub [32]`):
/// the PreKey message as a [`PairwiseCt`], the sender's first recognition
/// seed (epoch 0) and the sender's envelope public key so the recipient can
/// seal a reply. Sealed as a whole by `wire::seal::seal` to the recipient's
/// envelope key and carried in `InnerKind::SealedFirst` cells
/// (`wire::seal` "Sealed-first-flight cells"). The seed is wiped on drop.
#[derive(Clone, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct FirstFlight {
    #[zeroize(skip)]
    pub ct: PairwiseCt,
    pub recog_seed: [u8; SEED_LEN],
    #[zeroize(skip)]
    pub envelope_pub: [u8; 32],
}

impl core::fmt::Debug for FirstFlight {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "FirstFlight({:?}, {} bytes, <seed redacted>, envelope {})",
            self.ct.kind,
            self.ct.bytes.len(),
            hex::encode(self.envelope_pub)
        )
    }
}

impl Encode for FirstFlight {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        self.ct.encode_to(w)?;
        w.bytes(&self.recog_seed).bytes(&self.envelope_pub);
        Ok(())
    }
}

impl Decode for FirstFlight {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(FirstFlight {
            ct: PairwiseCt::decode_from(r)?,
            recog_seed: r.bytes()?,
            envelope_pub: r.bytes()?,
        })
    }
}

#[derive(Debug)]
struct SendEpoch {
    key: ChainKey,
    epoch_id: u32,
    epoch: RecogEpoch,
    /// Messages sent under this epoch.
    sent: u32,
}

#[derive(Debug)]
struct Contact {
    state: SessionState,
    send: Option<SendEpoch>,
    /// Receive epochs registered, oldest first (mirrors the table's ≤ 3).
    recv_epochs: Vec<u32>,
    next_obj_seq: u32,
    reseed_requested: bool,
    sent_prekey: bool,
    got_prekey: bool,
    decrypt_failures: u32,
}

impl Default for Contact {
    fn default() -> Self {
        Contact {
            state: SessionState::NeedsTheirKey,
            send: None,
            recv_epochs: Vec::new(),
            next_obj_seq: 0,
            reseed_requested: false,
            sent_prekey: false,
            got_prekey: false,
            decrypt_failures: 0,
        }
    }
}

/// The persistable form of one contact's session bookkeeping
/// ([`PairwiseManager::export`] / [`PairwiseManager::restore_contact`]):
/// everything the manager keeps in memory per peer. The send epoch's keys
/// are included because the seed is not retained after it was sent; the
/// receive windows live in the shared [`TagTable`], which the node persists
/// separately. The node stores this in the encrypted `chains` namespace.
#[derive(Debug, Clone)]
pub struct ContactRecord {
    pub peer: PeerId,
    pub state: SessionState,
    /// `(epoch id, epoch keys, messages sent under it)`.
    pub send: Option<(u32, RecogEpoch, u32)>,
    pub recv_epochs: Vec<u32>,
    pub next_obj_seq: u32,
    pub reseed_requested: bool,
    pub sent_prekey: bool,
    pub got_prekey: bool,
    pub decrypt_failures: u32,
}

/// All of this node's 1:1 sessions (module docs).
#[derive(Debug)]
pub struct PairwiseManager {
    me: PeerId,
    contacts: BTreeMap<PeerId, Contact>,
    /// Receive chain → peer.
    recv_chains: BTreeMap<ChainKey, PeerId>,
    reassemblers: BTreeMap<ChainKey, Reassembler>,
}

impl PairwiseManager {
    pub fn new(me: PeerId) -> Self {
        PairwiseManager {
            me,
            contacts: BTreeMap::new(),
            recv_chains: BTreeMap::new(),
            reassemblers: BTreeMap::new(),
        }
    }

    pub fn me(&self) -> &PeerId {
        &self.me
    }

    /// The session state with `peer` (`NeedsTheirKey` for an unknown peer).
    pub fn state(&self, peer: &PeerId) -> SessionState {
        self.contacts
            .get(peer)
            .map_or(SessionState::NeedsTheirKey, |c| c.state)
    }

    /// Contacts with any state, in key order.
    pub fn contacts(&self) -> impl Iterator<Item = (&PeerId, SessionState)> {
        self.contacts.iter().map(|(p, c)| (p, c.state))
    }

    /// The current send epoch id for `peer`, if one exists.
    pub fn send_epoch(&self, peer: &PeerId) -> Option<u32> {
        self.contacts.get(peer)?.send.as_ref().map(|s| s.epoch_id)
    }

    /// Receive epoch ids registered for `peer`, oldest first.
    pub fn recv_epochs(&self, peer: &PeerId) -> &[u32] {
        self.contacts
            .get(peer)
            .map_or(&[], |c| c.recv_epochs.as_slice())
    }

    /// The peer owning a recognised receive chain, if it is a 1:1 chain.
    pub fn chain_peer(&self, chain: ChainKey) -> Option<PeerId> {
        self.recv_chains.get(&chain).copied()
    }

    /// Asks for a fresh send seed before the next message to `peer`.
    pub fn request_reseed(&mut self, peer: &PeerId) {
        self.contacts.entry(*peer).or_default().reseed_requested = true;
    }

    /// Marks the pin as changed or the session as lost: the next send
    /// reports "Needs their key" until a new bundle is processed.
    pub fn mark_needs_key(&mut self, peer: &PeerId) {
        let c = self.contacts.entry(*peer).or_default();
        c.state = SessionState::NeedsTheirKey;
        c.sent_prekey = false;
        c.got_prekey = false;
    }

    /// Starts (or restores) this node's send epoch towards `peer` with
    /// `seed`, replacing any current one. The seed must already be on its
    /// way to the peer (pairing, first flight, or a re-seed post sent under
    /// the previous epoch).
    pub fn start_send_epoch(
        &mut self,
        recog: &mut TagTable,
        peer: &PeerId,
        seed: &[u8; SEED_LEN],
        epoch_id: u32,
        next_ctr: u64,
    ) {
        let key = pair_chain_key(&self.me, peer);
        let epoch = RecogEpoch::from_seed(seed);
        recog.register_sender(key, EpochId(epoch_id), epoch.clone(), next_ctr);
        let c = self.contacts.entry(*peer).or_default();
        c.send = Some(SendEpoch {
            key,
            epoch_id,
            epoch,
            sent: 0,
        });
        c.reseed_requested = false;
    }

    /// Registers a receive epoch from `peer` (its seed arrived inside the
    /// encryption). A known `(chain, epoch)` with the *same* seed is not
    /// re-registered (a re-delivery must not re-accept consumed counters);
    /// a different seed under a known epoch id replaces it: a peer that
    /// lost its state starts over at epoch 0 (`build_first_flight`, a
    /// re-scan), and its new cells must be recognised.
    pub fn register_recv_epoch(
        &mut self,
        recog: &mut TagTable,
        peer: &PeerId,
        seed: &[u8; SEED_LEN],
        epoch_id: u32,
    ) -> ChainKey {
        let key = pair_chain_key(peer, &self.me);
        let epoch = RecogEpoch::from_seed(seed);
        if recog.epoch(key, EpochId(epoch_id)) != Some(&epoch) {
            recog.register(key, EpochId(epoch_id), epoch, 0);
        }
        self.recv_chains.insert(key, *peer);
        let c = self.contacts.entry(*peer).or_default();
        if !c.recv_epochs.contains(&epoch_id) {
            c.recv_epochs.push(epoch_id);
            while c.recv_epochs.len() > crate::wire::recog::MAX_EPOCHS {
                c.recv_epochs.remove(0);
            }
        }
        key
    }

    /// Encrypts `plain` to `peer`, updating the session state (module docs).
    pub fn encrypt_to(
        &mut self,
        txn: &mut dyn KvTxn,
        backend: &dyn Backend,
        peer: &PeerId,
        plain: &[u8],
        now_mono: u64,
        rng: &mut dyn CryptoRng,
    ) -> Result<PairwiseCt, PairwiseError> {
        let c = self.contacts.entry(*peer).or_default();
        match PairwiseBackend::encrypt(backend, txn, peer, plain, now_mono, rng) {
            Ok(ct) => {
                match ct.kind {
                    PairwiseKind::PreKey => {
                        c.sent_prekey = true;
                        c.state = if c.got_prekey {
                            SessionState::Crossed
                        } else {
                            SessionState::Ready
                        };
                    }
                    PairwiseKind::Normal => c.state = SessionState::Ready,
                }
                Ok(ct)
            }
            Err(BackendError::NoSession | BackendError::UntrustedIdentity) => {
                c.state = SessionState::NeedsTheirKey;
                Err(PairwiseError::NeedsTheirKey)
            }
            Err(e) => Err(PairwiseError::Backend(e)),
        }
    }

    /// Decrypts `ct` from `peer`, updating the session state, and registers
    /// a carried recognition seed (module docs).
    pub fn decrypt_from(
        &mut self,
        txn: &mut dyn KvTxn,
        backend: &dyn Backend,
        recog: &mut TagTable,
        peer: &PeerId,
        ct: &PairwiseCt,
        rng: &mut dyn CryptoRng,
    ) -> Result<Received, PairwiseError> {
        // Wiped on drop: a 1:1 plaintext may be an SKDM entry (a chain key
        // and an epoch seed, `groups::group`).
        let plain = match PairwiseBackend::decrypt(backend, txn, peer, ct, rng) {
            Ok(p) => Zeroizing::new(p),
            Err(BackendError::NoSession | BackendError::UntrustedIdentity) => {
                let c = self.contacts.entry(*peer).or_default();
                c.state = SessionState::NeedsTheirKey;
                return Err(PairwiseError::NeedsTheirKey);
            }
            Err(BackendError::Decrypt | BackendError::Encoding) => {
                let c = self.contacts.entry(*peer).or_default();
                c.decrypt_failures = c.decrypt_failures.saturating_add(1);
                if c.decrypt_failures >= DECRYPT_FAILURES_NEEDS_KEY_ASM {
                    c.state = SessionState::NeedsTheirKey;
                }
                return Err(PairwiseError::Decrypt);
            }
            Err(e) => return Err(PairwiseError::Backend(e)),
        };
        {
            let c = self.contacts.entry(*peer).or_default();
            c.decrypt_failures = 0;
            match ct.kind {
                PairwiseKind::PreKey => {
                    c.got_prekey = true;
                    c.state = if c.sent_prekey {
                        SessionState::Crossed
                    } else {
                        SessionState::Ready
                    };
                }
                PairwiseKind::Normal => c.state = SessionState::Ready,
            }
        }
        let post: PostPlain = decode(&plain)?;
        if post.kind == kind::POST_RECOG_SEED {
            let body = post.recog_seed_body()?;
            self.register_recv_epoch(recog, peer, &body.seed, body.epoch_id);
            return Ok(Received::RecogSeed {
                epoch_id: body.epoch_id,
            });
        }
        Ok(Received::Post(post))
    }

    /// Sends `post` to `peer` as cells under the send epoch, re-seeding
    /// first when due (the re-seed control post goes out under the old
    /// epoch, then the post under the new one; all cells are returned in
    /// order). The relay class is [`PostPlain::class`] (the re-seed is C1).
    /// The post is validated before the re-seed starts, so a post that
    /// cannot be sent never advances the epoch (the peer would never learn
    /// the seed and every later cell would be unrecognisable).
    #[allow(clippy::too_many_arguments)] // txn, backend, recog, peer, post, time and rng are all inputs
    pub fn send_post(
        &mut self,
        txn: &mut dyn KvTxn,
        backend: &dyn Backend,
        recog: &mut TagTable,
        peer: &PeerId,
        post: &PostPlain,
        now_mono: u64,
        rng: &mut dyn CryptoRng,
    ) -> Result<Vec<Cell>, PairwiseError> {
        let bytes = post.try_encode()?;
        let (due, next_epoch) = {
            let c = self.contacts.get(peer).ok_or(PairwiseError::NoSendEpoch)?;
            let s = c.send.as_ref().ok_or(PairwiseError::NoSendEpoch)?;
            (
                c.reseed_requested || s.sent >= RECOG_RESEED_MESSAGES_ASM,
                s.epoch_id.wrapping_add(1),
            )
        };
        let mut cells = Vec::new();
        if due {
            let mut seed = Zeroizing::new([0u8; SEED_LEN]);
            rng.fill_bytes(seed.as_mut());
            let control = PostPlain::recog_seed(&seed, next_epoch);
            let control_bytes = Zeroizing::new(control.encode());
            let ct = self.encrypt_to(txn, backend, peer, &control_bytes, now_mono, rng)?;
            cells.extend(self.cells_for_ct(recog, peer, &ct, rng)?);
            let next_ctr = 0;
            self.start_send_epoch(recog, peer, &seed, next_epoch, next_ctr);
        }
        let ct = self.encrypt_to(txn, backend, peer, &bytes, now_mono, rng)?;
        cells.extend(self.cells_for_ct(recog, peer, &ct, rng)?);
        Ok(cells)
    }

    /// Cuts one pairwise ciphertext (ours, or an SKDM bundle entry produced
    /// by `groups::group`) into cells under the send epoch towards `peer`.
    pub fn cells_for_ct(
        &mut self,
        recog: &mut TagTable,
        peer: &PeerId,
        ct: &PairwiseCt,
        rng: &mut dyn CryptoRng,
    ) -> Result<Vec<Cell>, PairwiseError> {
        let c = self
            .contacts
            .get_mut(peer)
            .ok_or(PairwiseError::NoSendEpoch)?;
        let s = c.send.as_mut().ok_or(PairwiseError::NoSendEpoch)?;
        let next = c
            .next_obj_seq
            .checked_add(1)
            .ok_or(PairwiseError::ObjSeqExhausted)?;
        let frags = inner::fragment(InnerKind::PairwiseDr, c.next_obj_seq, &ct.encode())?;
        let mut cells = Vec::with_capacity(frags.len());
        for frag in &frags {
            let issued = recog.sender_next(s.key)?;
            let ttl = cell::draw_initial_ttl(rng);
            cells.push(cell::seal_cell(
                VERSION,
                ttl,
                &issued.tag,
                s.epoch.k_hdr().as_bytes(),
                issued.ctr,
                0,
                frag,
            ));
        }
        c.next_obj_seq = next;
        s.sent = s.sent.saturating_add(1);
        Ok(cells)
    }

    /// Opens one recognised 1:1 cell, reassembles and decrypts (module docs
    /// "Cells"). `Ok(None)` while fragments are missing.
    pub fn decrypt_cell(
        &mut self,
        txn: &mut dyn KvTxn,
        backend: &dyn Backend,
        recog: &mut TagTable,
        hit: Hit,
        cell: &Cell,
        rng: &mut dyn CryptoRng,
    ) -> Result<Option<(PeerId, Received)>, PairwiseError> {
        let peer = *self
            .recv_chains
            .get(&hit.chain)
            .ok_or(PairwiseError::UnknownChain)?;
        let epoch = recog
            .epoch(hit.chain, hit.epoch_id)
            .ok_or(RecogError::UnknownEpoch)?;
        let plain = cell.open_body(epoch.k_hdr().as_bytes(), hit.ctr, 0)?;
        recog.consume(hit)?;
        let reassembler = self.reassemblers.entry(hit.chain).or_default();
        let Some((kind, _obj_seq, object)) = reassembler.push(hit.chain.0, &plain)? else {
            return Ok(None);
        };
        if kind != InnerKind::PairwiseDr {
            return Err(PairwiseError::WrongKind);
        }
        let ct: PairwiseCt = decode(&object)?;
        let received = self.decrypt_from(txn, backend, recog, &peer, &ct, rng)?;
        Ok(Some((peer, received)))
    }

    /// Every contact's bookkeeping, in key order, for persistence.
    pub fn export(&self) -> Vec<ContactRecord> {
        self.contacts
            .iter()
            .map(|(peer, c)| ContactRecord {
                peer: *peer,
                state: c.state,
                send: c
                    .send
                    .as_ref()
                    .map(|s| (s.epoch_id, s.epoch.clone(), s.sent)),
                recv_epochs: c.recv_epochs.clone(),
                next_obj_seq: c.next_obj_seq,
                reseed_requested: c.reseed_requested,
                sent_prekey: c.sent_prekey,
                got_prekey: c.got_prekey,
                decrypt_failures: c.decrypt_failures,
            })
            .collect()
    }

    /// Restores one contact from a [`ContactRecord`]: the send epoch is
    /// re-registered in the table at `send_next_ctr` (the persisted sender
    /// counter, so no cell key is ever reused), the receive chain mapping is
    /// recorded (its windows are restored into the table by the caller), and
    /// the session flags are set as persisted. Replaces any state for the
    /// peer.
    pub fn restore_contact(
        &mut self,
        recog: &mut TagTable,
        record: ContactRecord,
        send_next_ctr: u64,
    ) {
        let key = pair_chain_key(&self.me, &record.peer);
        let send = record.send.map(|(epoch_id, epoch, sent)| {
            recog.register_sender(key, EpochId(epoch_id), epoch.clone(), send_next_ctr);
            SendEpoch {
                key,
                epoch_id,
                epoch,
                sent,
            }
        });
        if !record.recv_epochs.is_empty() {
            self.recv_chains
                .insert(pair_chain_key(&record.peer, &self.me), record.peer);
        }
        self.contacts.insert(
            record.peer,
            Contact {
                state: record.state,
                send,
                recv_epochs: record.recv_epochs,
                next_obj_seq: record.next_obj_seq,
                reseed_requested: record.reseed_requested,
                sent_prekey: record.sent_prekey,
                got_prekey: record.got_prekey,
                decrypt_failures: record.decrypt_failures,
            },
        );
    }

    /// Adopts a completed QR pairing (`pairwise::pairing`): the session
    /// exists in the backend already (first DR round trip on the direct
    /// link), so the state is `Ready`; this node's send epoch 0 and the
    /// peer's receive epoch 0 are registered from the exchanged seeds.
    pub fn adopt_pairing(
        &mut self,
        recog: &mut TagTable,
        paired: &crate::pairwise::pairing::Paired,
    ) {
        self.start_send_epoch(recog, &paired.peer, &paired.send_seed, 0, 0);
        self.register_recv_epoch(recog, &paired.peer, &paired.recv_seed, 0);
        let c = self.contacts.entry(paired.peer).or_default();
        c.state = SessionState::Ready;
        c.sent_prekey = false;
        c.got_prekey = false;
        c.decrypt_failures = 0;
    }

    /// Builds the payload of a sealed first flight to `peer` (DESIGN §2.4):
    /// starts this node's send epoch 0 towards the peer with a fresh seed
    /// and encrypts `post` (normally a text or receipt) as the PreKey
    /// message. The caller seals the encoding to the peer's envelope key.
    #[allow(clippy::too_many_arguments)] // as send_post, plus the envelope key to advertise
    pub fn build_first_flight(
        &mut self,
        txn: &mut dyn KvTxn,
        backend: &dyn Backend,
        recog: &mut TagTable,
        peer: &PeerId,
        post: &PostPlain,
        my_envelope_pub: [u8; 32],
        now_mono: u64,
        rng: &mut dyn CryptoRng,
    ) -> Result<FirstFlight, PairwiseError> {
        let bytes = post.try_encode()?;
        let ct = self.encrypt_to(txn, backend, peer, &bytes, now_mono, rng)?;
        let mut seed = Zeroizing::new([0u8; SEED_LEN]);
        rng.fill_bytes(seed.as_mut());
        self.start_send_epoch(recog, peer, &seed, 0, 0);
        Ok(FirstFlight {
            ct,
            recog_seed: *seed,
            envelope_pub: my_envelope_pub,
        })
    }

    /// Accepts an unsealed first flight from `from`: decrypts the PreKey
    /// message (which starts the responder side of the session; the
    /// backend refuses it unless `from`'s key is pinned, BRIEF §5.2) and
    /// registers the sender's receive epoch 0. Returns the post and the
    /// sender's envelope key for the reply.
    pub fn accept_first_flight(
        &mut self,
        txn: &mut dyn KvTxn,
        backend: &dyn Backend,
        recog: &mut TagTable,
        from: &PeerId,
        flight: &FirstFlight,
        rng: &mut dyn CryptoRng,
    ) -> Result<(Received, [u8; 32]), PairwiseError> {
        let received = self.decrypt_from(txn, backend, recog, from, &flight.ct, rng)?;
        self.register_recv_epoch(recog, from, &flight.recog_seed, 0);
        Ok((received, flight.envelope_pub))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::encode;
    use crate::crypto::ed25519::IdentitySigner;
    use proptest::prelude::*;

    fn peer(seed: u8) -> PeerId {
        IdentitySigner::from_seed(&[seed; 32]).public()
    }

    #[test]
    fn pair_chain_key_is_directional_and_documented() {
        let (a, b) = (peer(1), peer(2));
        let mut bytes = b"meshapp.v1.chainkey.pair".to_vec();
        bytes.extend_from_slice(a.as_bytes());
        bytes.extend_from_slice(b.as_bytes());
        let expect = u64::from_le_bytes(crate::crypto::hash::sha256(&bytes).truncate8());
        assert_eq!(pair_chain_key(&a, &b), ChainKey(expect));
        assert_ne!(pair_chain_key(&a, &b), pair_chain_key(&b, &a));
    }

    #[test]
    fn epochs_register_and_bookkeeping_is_bounded() {
        let (me, them) = (peer(1), peer(2));
        let mut m = PairwiseManager::new(me);
        let mut recog = TagTable::new();
        assert_eq!(m.state(&them), SessionState::NeedsTheirKey);
        assert_eq!(m.send_epoch(&them), None);
        m.start_send_epoch(&mut recog, &them, &[1; 32], 4, 10);
        assert_eq!(m.send_epoch(&them), Some(4));
        assert_eq!(recog.sender_next_ctr(pair_chain_key(&me, &them)), Some(10));
        for e in 0..5u32 {
            let key = m.register_recv_epoch(&mut recog, &them, &[e as u8; 32], e);
            assert_eq!(key, pair_chain_key(&them, &me));
            assert_eq!(m.chain_peer(key), Some(them));
        }
        assert_eq!(m.recv_epochs(&them), &[2, 3, 4]);
        let key = pair_chain_key(&them, &me);
        assert!(recog.epoch(key, EpochId(1)).is_none(), "table keeps 3");
        assert!(recog.epoch(key, EpochId(4)).is_some());
        // re-registering a known epoch keeps its window
        let hit = recog
            .lookup(&RecogEpoch::from_seed(&[4; 32]).tag_for(0))
            .unwrap();
        recog.consume(hit).unwrap();
        m.register_recv_epoch(&mut recog, &them, &[4; 32], 4);
        assert_eq!(
            recog.lookup(&RecogEpoch::from_seed(&[4; 32]).tag_for(0)),
            None,
            "consumed counter stays consumed"
        );
        m.mark_needs_key(&them);
        assert_eq!(m.state(&them), SessionState::NeedsTheirKey);
        assert_eq!(m.contacts().count(), 1);
        assert_eq!(m.me(), &me);
    }

    #[test]
    fn a_new_seed_under_a_reused_epoch_id_replaces_the_old_epoch() {
        // A peer that lost its state re-establishes with a fresh epoch-0
        // seed (`build_first_flight` and a re-scan both start at epoch 0):
        // the new seed must be recognised, while a replay of the same seed
        // keeps the window (consumed counters stay consumed).
        let (me, them) = (peer(1), peer(2));
        let mut m = PairwiseManager::new(me);
        let mut recog = TagTable::new();
        let key = m.register_recv_epoch(&mut recog, &them, &[1; 32], 0);
        let hit = recog
            .lookup(&RecogEpoch::from_seed(&[1; 32]).tag_for(0))
            .unwrap();
        recog.consume(hit).unwrap();
        m.register_recv_epoch(&mut recog, &them, &[1; 32], 0);
        assert_eq!(
            recog.lookup(&RecogEpoch::from_seed(&[1; 32]).tag_for(0)),
            None,
            "same seed: window kept"
        );
        m.register_recv_epoch(&mut recog, &them, &[2; 32], 0);
        let hit2 = recog
            .lookup(&RecogEpoch::from_seed(&[2; 32]).tag_for(0))
            .expect("the new seed is recognised");
        assert_eq!(hit2.chain, key);
        assert_eq!(hit2.epoch_id, EpochId(0));
        assert_eq!(
            recog.lookup(&RecogEpoch::from_seed(&[1; 32]).tag_for(1)),
            None,
            "the old seed's tags are gone"
        );
        assert_eq!(m.recv_epochs(&them), &[0]);
    }

    #[test]
    fn seed_carriers_are_zeroized_on_drop() {
        fn assert_zeroize_on_drop<T: zeroize::ZeroizeOnDrop>() {}
        assert_zeroize_on_drop::<FirstFlight>();
    }

    #[test]
    fn first_flight_layout_roundtrip_and_redaction() {
        let ff = FirstFlight {
            ct: PairwiseCt {
                kind: PairwiseKind::PreKey,
                bytes: vec![1, 2, 3],
            },
            recog_seed: [5; 32],
            envelope_pub: [6; 32],
        };
        let bytes = encode(&ff);
        assert_eq!(bytes.len(), 1 + 2 + 3 + 32 + 32);
        assert_eq!(decode::<FirstFlight>(&bytes), Ok(ff.clone()));
        let dbg = format!("{ff:?}");
        assert!(!dbg.contains("0505"));
        assert!(dbg.contains(&hex::encode([6u8; 32])));
    }

    #[test]
    #[cfg(feature = "backend-libsignal")] // needs a real identity in the backend
    fn sending_without_a_session_or_epoch_reports_needs_key() {
        let backend = crate::crypto::backend::default_backend();
        let mut kv = crate::storage::kv::MemKv::new();
        let mut recog = TagTable::new();
        let mut rng = <rand_chacha::ChaCha20Rng as rand::SeedableRng>::seed_from_u64(1);
        let (me, them) = (peer(1), peer(2));
        backend.init_identity(&mut kv, &mut rng).unwrap();
        let mut m = PairwiseManager::new(me);
        let post = PostPlain::recog_seed(&[0; 32], 0);
        assert_eq!(
            m.send_post(&mut kv, &*backend, &mut recog, &them, &post, 0, &mut rng)
                .err(),
            Some(PairwiseError::NoSendEpoch)
        );
        assert_eq!(
            m.encrypt_to(&mut kv, &*backend, &them, b"x", 0, &mut rng)
                .err(),
            Some(PairwiseError::NeedsTheirKey)
        );
        assert_eq!(m.state(&them), SessionState::NeedsTheirKey);
        let ct = PairwiseCt {
            kind: PairwiseKind::Normal,
            bytes: vec![0; 10],
        };
        // malformed bytes are refused as an opaque decrypt failure
        assert_eq!(
            m.decrypt_from(&mut kv, &*backend, &mut recog, &them, &ct, &mut rng)
                .err(),
            Some(PairwiseError::Decrypt)
        );
        assert_eq!(m.state(&them), SessionState::NeedsTheirKey);
        let hit = Hit {
            chain: ChainKey(9),
            epoch_id: EpochId(0),
            ctr: 0,
        };
        let cell = Cell::from_array([0u8; cell::CELL_LEN]);
        assert_eq!(
            m.decrypt_cell(&mut kv, &*backend, &mut recog, hit, &cell, &mut rng)
                .err(),
            Some(PairwiseError::UnknownChain)
        );
        assert!(
            kv.scan_prefix(crate::storage::kv::ns::SESSIONS, &[])
                .is_empty()
        );
    }

    proptest! {
        #[test]
        fn first_flight_decoder_survives_random_bytes(bytes in prop::collection::vec(any::<u8>(), 0..200)) {
            if let Ok(ff) = decode::<FirstFlight>(&bytes) {
                prop_assert_eq!(encode(&ff), bytes);
            }
        }
    }
}
