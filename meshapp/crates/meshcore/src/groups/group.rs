//! One group on one node: the own sender chain with its recognition epoch,
//! every member's chains, rotation, SKDM acceptance bound to the roster
//! KeyCard, and post encode/decode (docs/DESIGN.md §1 row `groups/group.rs`,
//! §3 "Sender Key rotation"; BRIEF_v2 §5.2 third bullet, §5.4 "Rotation";
//! DECISIONS D3 K1–K2, §3 rows (3,4,6,10) and (3,4,6,9,10)).
//!
//! # What lives where
//!
//! * The backend ([`crate::crypto::backend::GroupBackend`]) holds the Sender
//!   Key records (own and received) in the caller's transaction.
//! * The shared [`TagTable`] holds the recognition state of every chain this
//!   node sends on or listens to; this module names group chains with
//!   [`group_chain_key`] and uses the chain id as the epoch id (a rotation
//!   is a new chain, so a group chain has exactly one epoch).
//! * [`GroupState`] holds the mapping from recognised chains back to
//!   `(member, chain id)`, the own chain's object counter, a reassembler per
//!   chain and the pending-rotation flag.
//!
//! # Rotation (D3 K2; DESIGN §3)
//!
//! `pending_rotation` is set by the roster/rotation policy (any valid Remove
//! entering the DAG, or 24 h of powered-on time: `roster::rotation`); this
//! module only consumes the flag. [`GroupState::rotate`] runs before the
//! next post: `new_sender_chain` → a fresh epoch seed → one SKDM entry per
//! current member (pairwise-encrypted, DESIGN §2.4 "one bundle of n−1
//! pairwise ciphertexts"), padded ([`SkdmBundle`]) and, when an old chain
//! exists, already cut into cells under the **old** chain's tags as one C1
//! [`InnerKind::SkdmBundle`] object (the entries are pairwise ciphertexts,
//! so no group encryption is needed and the old Sender Key, which the
//! backend has just replaced, is not required). A sender with no old chain,
//! or whose bundle exceeds one 64 kB object (§3 row (3,4,6,9,10)), sends the
//! entries pairwise ([`Rotation::recipients`]). Rotations are coalesced:
//! every trigger between two posts sets the same flag, one rotation clears
//! it, and [`GroupState::encrypt_post`] refuses to post while it is set, so
//! "once a sender knows about a removal, none of its new posts uses a key
//! the removed member holds".
//!
//! # SKDM acceptance (BRIEF §5.2 "accepted only over a pairwise session whose
//! key matches the sender's roster entry, checked and stored in one atomic
//! step"; D3 K1)
//!
//! [`GroupState::accept_skdm`] compares the X25519 key of the pairwise
//! session the SKDM arrived on (`remote_session_key`) with the key the
//! node took from the sender's roster KeyCard, and only then stores the
//! chain (`process_skdm`) and registers its recognition epoch, all in the
//! same transaction. On a mismatch nothing is written. A re-delivered SKDM
//! for a known `(chain, epoch)` with the same seed is not re-registered (the
//! tag table would re-accept consumed counters; [`TagTable::register`]
//! docs); a different seed for a known chain replaces the epoch.
//!
//! # Posts
//!
//! A post is a [`PostPlain`] → `GroupBackend::encrypt` (SenderKeyMessage,
//! per-chain signature inside) → [`inner::fragment`] with
//! [`InnerKind::SenderKey`] and the own chain's object counter → one cell
//! per fragment with `(ctr, tag, cell key)` from the own chain's sender
//! epoch. Each cell's body nonce uses fragment index 0: the recognition
//! counter already gives every cell its own key, and a receiver cannot
//! know the fragment index before opening the body (it is inside; the same
//! convention as `tests/recog_seal.rs`). Receiving reverses it through a
//! per-chain [`Reassembler`]; a post under a retained previous chain still
//! decrypts because the backend keeps the sender's earlier chains (BRIEF
//! §5.4) and the tag table keeps their epochs.
//!
//! Nothing here reads a clock; `now_mono` is passed through to the backend.

use std::collections::BTreeMap;

use rand::CryptoRng;
use zeroize::Zeroizing;

use crate::codec::{CodecError, Encode, decode};
use crate::crypto::backend::{
    Backend, BackendError, ChainId, GroupBackend, GroupId, PairwiseBackend, PairwiseCt, PeerId,
};
use crate::crypto::hash::sha256_concat;
use crate::objects::chat_post::{PostError, PostPlain, kind};
use crate::objects::skdm_bundle::{SkdmBundle, SkdmEntry, SkdmError, SkdmPayload};
use crate::wire::cell::{self, Cell, CellReject, VERSION};
use crate::wire::inner::{self, InnerError, InnerKind, Reassembler};
use crate::wire::recog::{ChainKey, EpochId, Hit, RecogEpoch, RecogError, SEED_LEN, TagTable};

/// Domain of the local chain-key derivation.
pub const CHAIN_KEY_DOMAIN: &[u8] = b"meshapp.v1.chainkey.group";

/// The local [`ChainKey`] of `(group, chain owner, chain id)`:
/// `LE64(Trunc8(SHA256("meshapp.v1.chainkey.group" ‖ group ‖ owner.ed ‖
/// LE32(chain_id))))`. Never on the wire; only a name inside the shared
/// tag table, distinct from 1:1 keys by its domain.
pub fn group_chain_key(group: &GroupId, owner: &PeerId, chain: ChainId) -> ChainKey {
    let h = sha256_concat(&[
        CHAIN_KEY_DOMAIN,
        group.as_bytes(),
        owner.as_bytes(),
        &chain.0.to_le_bytes(),
    ]);
    ChainKey(u64::from_le_bytes(h.truncate8()))
}

/// The epoch id of a group chain: the chain id itself (one epoch per chain).
pub fn group_epoch_id(chain: ChainId) -> EpochId {
    EpochId(chain.0)
}

/// Why a group operation did not apply.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum GroupError {
    /// This node has no sender chain yet: call [`GroupState::rotate`] first.
    #[error("no own sender chain; rotate first")]
    NoOwnChain,
    /// A rotation is pending (a removal was learned or the interval passed):
    /// no post may go out under the current key (D3 K2).
    #[error("rotation pending; rotate before posting")]
    RotationPending,
    /// The pairwise session the SKDM arrived on is not bound to the X25519
    /// key of the sender's roster KeyCard, or there is no session. Nothing
    /// was stored (D3 K1).
    #[error("sender's session key does not match its roster KeyCard")]
    IdentityMismatch,
    /// The bundle has no entry for this node, or the SKDM is this node's own.
    #[error("the SKDM is not addressed to this node")]
    NotForMe,
    /// The SKDM payload names another group.
    #[error("SKDM is for another group")]
    WrongGroup,
    /// An object of a kind this module does not carry.
    #[error("unexpected object kind")]
    WrongKind,
    /// A cell was recognised under a chain this group does not know.
    #[error("recognised chain is not a chain of this group")]
    UnknownChain,
    /// The own chain's object counter reached `u32::MAX`; rotate.
    #[error("object counter exhausted")]
    ObjSeqExhausted,
    #[error("backend: {0}")]
    Backend(#[from] BackendError),
    #[error("codec: {0}")]
    Codec(#[from] CodecError),
    #[error("fragment layer: {0}")]
    Inner(#[from] InnerError),
    #[error("recognition: {0}")]
    Recog(#[from] RecogError),
    #[error("cell: {0}")]
    Cell(#[from] CellReject),
    #[error("bundle: {0}")]
    Skdm(#[from] SkdmError),
    #[error("post: {0}")]
    Post(#[from] PostError),
}

/// This node's current sender chain in the group.
#[derive(Debug)]
struct OwnChain {
    chain: ChainId,
    key: ChainKey,
    epoch: RecogEpoch,
    /// Per-chain object counter (DESIGN §2.2 `obj_seq`).
    next_obj_seq: u32,
}

/// What [`GroupState::rotate`] produced.
#[derive(Debug, Clone)]
pub struct Rotation {
    /// The new chain.
    pub chain: ChainId,
    /// The padded bundle (one real entry per member with a session).
    pub bundle: SkdmBundle,
    /// The bundle as one C1 object under the **old** chain's tags, or empty
    /// when there was no old chain or the bundle does not fit one object:
    /// then the caller delivers [`Rotation::recipients`] pairwise instead.
    pub cells: Vec<Cell>,
    /// Every real entry as `(member, pairwise ciphertext)`, for pairwise
    /// delivery when `cells` is empty.
    pub recipients: Vec<(PeerId, PairwiseCt)>,
    /// Members that got no entry because no pairwise session with them
    /// exists yet ("Needs their key", D6 rule 7); they receive the SKDM once
    /// a session is established and the caller re-sends it to them.
    pub without_session: Vec<PeerId>,
}

impl Rotation {
    /// Whether the caller must deliver the entries pairwise.
    pub fn needs_pairwise_delivery(&self) -> bool {
        self.cells.is_empty() && !self.recipients.is_empty()
    }
}

/// A decrypted object received on one of the group's chains.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inbound {
    /// A post, with the chain it was sent under (a retained previous chain
    /// included) and its per-chain object counter.
    Post {
        sender: PeerId,
        chain: ChainId,
        obj_seq: u32,
        post: PostPlain,
    },
    /// An SKDM bundle sent under `sender`'s old chain; the caller finds its
    /// entry, decrypts it pairwise and calls [`GroupState::accept_skdm`]
    /// with the sender's roster KeyCard key.
    Bundle {
        sender: PeerId,
        obj_seq: u32,
        bundle: SkdmBundle,
    },
}

/// One group's state on this node (module docs).
#[derive(Debug)]
pub struct GroupState {
    group: GroupId,
    me: PeerId,
    own: Option<OwnChain>,
    /// Member → its chains → epoch id, as registered in the tag table.
    members: BTreeMap<PeerId, BTreeMap<ChainId, EpochId>>,
    /// Recognised chain → `(owner, chain id)`.
    chain_keys: BTreeMap<ChainKey, (PeerId, ChainId)>,
    reassemblers: BTreeMap<ChainKey, Reassembler>,
    pending_rotation: bool,
}

impl GroupState {
    /// A group this node is a member of, with no chain yet: the first
    /// [`rotate`](Self::rotate) creates one.
    pub fn new(group: GroupId, me: PeerId) -> Self {
        GroupState {
            group,
            me,
            own: None,
            members: BTreeMap::new(),
            chain_keys: BTreeMap::new(),
            reassemblers: BTreeMap::new(),
            pending_rotation: false,
        }
    }

    pub fn group(&self) -> &GroupId {
        &self.group
    }

    pub fn me(&self) -> &PeerId {
        &self.me
    }

    /// This node's current chain, if any.
    pub fn own_chain(&self) -> Option<ChainId> {
        self.own.as_ref().map(|o| o.chain)
    }

    /// The next object counter of the own chain, for persistence.
    pub fn own_next_obj_seq(&self) -> Option<u32> {
        self.own.as_ref().map(|o| o.next_obj_seq)
    }

    /// Whether the rotation policy asked for a rotation before the next post.
    pub fn rotation_pending(&self) -> bool {
        self.pending_rotation
    }

    /// Input from the rotation policy (`roster::rotation`): a removal was
    /// learned or the interval passed. Idempotent; cleared by
    /// [`rotate`](Self::rotate) (coalescing, D3 K2).
    pub fn mark_rotation_pending(&mut self) {
        self.pending_rotation = true;
    }

    /// `true` when the next post must be preceded by a rotation: no chain
    /// yet, or the flag is set.
    pub fn needs_rotation(&self) -> bool {
        self.own.is_none() || self.pending_rotation
    }

    /// Members whose chains this node can read, with their chain ids.
    pub fn member_chains(&self) -> impl Iterator<Item = (&PeerId, &BTreeMap<ChainId, EpochId>)> {
        self.members.iter()
    }

    /// `(owner, chain id)` of a recognised chain, if it is one of this group's.
    pub fn chain_owner(&self, chain: ChainKey) -> Option<(PeerId, ChainId)> {
        self.chain_keys.get(&chain).copied()
    }

    /// Re-registers the own chain after a restart from persisted state: the
    /// chain id (still the backend's own chain), its epoch seed, the next
    /// recognition counter and the next object counter.
    pub fn restore_own(
        &mut self,
        recog: &mut TagTable,
        chain: ChainId,
        seed: &[u8; SEED_LEN],
        next_ctr: u64,
        next_obj_seq: u32,
    ) {
        let key = group_chain_key(&self.group, &self.me, chain);
        let epoch = RecogEpoch::from_seed(seed);
        recog.register_sender(key, group_epoch_id(chain), epoch.clone(), next_ctr);
        self.own = Some(OwnChain {
            chain,
            key,
            epoch,
            next_obj_seq,
        });
    }

    /// Re-registers a member's chain after a restart (the backend record is
    /// in the store; the recognition window comes from persisted state via
    /// `recog.restore`, which the caller does; this only records the
    /// mapping).
    pub fn restore_member_chain(&mut self, owner: PeerId, chain: ChainId) -> ChainKey {
        let key = group_chain_key(&self.group, &owner, chain);
        self.members
            .entry(owner)
            .or_default()
            .insert(chain, group_epoch_id(chain));
        self.chain_keys.insert(key, (owner, chain));
        key
    }

    /// Drops a member's chains from this state and the tag table (a removal
    /// learned long ago whose retained chains are no longer wanted). The
    /// backend record is left to the library's own cap.
    pub fn forget_member(&mut self, recog: &mut TagTable, member: &PeerId) {
        if let Some(chains) = self.members.remove(member) {
            for chain in chains.keys() {
                let key = group_chain_key(&self.group, member, *chain);
                recog.forget_chain(key);
                self.chain_keys.remove(&key);
                self.reassemblers.remove(&key);
            }
        }
    }

    /// Rotates this node's Sender Key (module docs "Rotation"). `members` is
    /// the current roster view (`members(current heads)`); this node is
    /// skipped if listed. Clears the pending flag.
    pub fn rotate(
        &mut self,
        txn: &mut dyn crate::storage::kv::KvTxn,
        backend: &dyn Backend,
        recog: &mut TagTable,
        members: &[PeerId],
        now_mono: u64,
        rng: &mut dyn CryptoRng,
    ) -> Result<Rotation, GroupError> {
        let (chain, skdm) = backend.new_sender_chain(txn, &self.group, rng)?;
        let mut seed = Zeroizing::new([0u8; SEED_LEN]);
        rng.fill_bytes(seed.as_mut());
        let epoch = RecogEpoch::from_seed(&seed);
        let payload = SkdmPayload {
            group: self.group,
            skdm,
            seed: *seed,
        };
        let post = PostPlain::new(kind::POST_SKDM_BUNDLE, Vec::new(), payload.encode())?;
        let plain = Zeroizing::new(post.encode());

        let mut sorted: Vec<PeerId> = members.iter().copied().filter(|m| *m != self.me).collect();
        sorted.sort_unstable();
        sorted.dedup();
        let mut entries = Vec::with_capacity(sorted.len());
        let mut recipients = Vec::with_capacity(sorted.len());
        let mut without_session = Vec::new();
        for peer in sorted {
            match PairwiseBackend::encrypt(backend, txn, &peer, &plain, now_mono, rng) {
                Ok(ct) => {
                    entries.push(SkdmEntry::new(&peer, chain, ct.encode()));
                    recipients.push((peer, ct));
                }
                Err(BackendError::NoSession | BackendError::UntrustedIdentity) => {
                    without_session.push(peer);
                }
                Err(e) => return Err(e.into()),
            }
        }
        let bundle = SkdmBundle::build(chain, entries, rng)?;

        // Under the old chain's tags, as one C1 object, if there is an old
        // chain and the object fits. The old chain is replaced only once
        // its cells exist: on an error the state still names the chain the
        // caller's rolled-back transaction holds, and the pending flag stays.
        let mut cells = Vec::new();
        if let Some(old) = self.own.as_mut() {
            if !recipients.is_empty() {
                let object = bundle.encode();
                // A bundle above one 64 kB object (PreKey entries at first
                // join) falls back to pairwise delivery.
                if let Ok(frags) = inner::fragment(InnerKind::SkdmBundle, old.next_obj_seq, &object)
                {
                    cells = Self::cells_for(recog, old, &frags, rng)?;
                }
            }
        }
        if let Some(old) = self.own.take() {
            recog.forget_chain(old.key);
            self.reassemblers.remove(&old.key);
        }

        let key = group_chain_key(&self.group, &self.me, chain);
        recog.register_sender(key, group_epoch_id(chain), epoch.clone(), 0);
        self.own = Some(OwnChain {
            chain,
            key,
            epoch,
            next_obj_seq: 0,
        });
        self.pending_rotation = false;
        Ok(Rotation {
            chain,
            bundle,
            cells,
            recipients,
            without_session,
        })
    }

    /// Accepts one SKDM entry's decrypted plaintext from `from` (module docs
    /// "SKDM acceptance"): the atomic identity check against
    /// `expected_x25519` (the X25519 key of `from`'s roster KeyCard, supplied
    /// by the node) precedes every write; on success the chain is stored and
    /// its recognition epoch registered in the same transaction. Returns the
    /// chain id.
    pub fn accept_skdm(
        &mut self,
        txn: &mut dyn crate::storage::kv::KvTxn,
        backend: &dyn Backend,
        recog: &mut TagTable,
        from: PeerId,
        expected_x25519: [u8; 32],
        entry_plain: &[u8],
    ) -> Result<ChainId, GroupError> {
        if from == self.me {
            return Err(GroupError::NotForMe);
        }
        let post: PostPlain = decode(entry_plain)?;
        if post.kind != kind::POST_SKDM_BUNDLE {
            return Err(GroupError::WrongKind);
        }
        let payload: SkdmPayload = decode(&post.body)?;
        if payload.group != self.group {
            return Err(GroupError::WrongGroup);
        }
        self.check_identity(txn, backend, &from, &expected_x25519)?;
        let chain = backend.process_skdm(txn, &self.group, &from, &payload.skdm)?;
        let key = group_chain_key(&self.group, &from, chain);
        let epoch_id = group_epoch_id(chain);
        let epoch = RecogEpoch::from_seed(&payload.seed);
        if recog.epoch(key, epoch_id) != Some(&epoch) {
            recog.register(key, epoch_id, epoch, 0);
        }
        self.members
            .entry(from)
            .or_default()
            .insert(chain, epoch_id);
        self.chain_keys.insert(key, (from, chain));
        Ok(chain)
    }

    /// Finds this node's entry in `bundle`, decrypts it on the pairwise
    /// session with `from` and accepts it ([`accept_skdm`](Self::accept_skdm)).
    /// The identity check runs before the pairwise decrypt, so a mismatch
    /// consumes no ratchet step and writes nothing.
    #[allow(clippy::too_many_arguments)] // txn, backend, recog and the identity inputs are all required
    pub fn accept_bundle(
        &mut self,
        txn: &mut dyn crate::storage::kv::KvTxn,
        backend: &dyn Backend,
        recog: &mut TagTable,
        from: PeerId,
        expected_x25519: [u8; 32],
        bundle: &SkdmBundle,
        rng: &mut dyn CryptoRng,
    ) -> Result<ChainId, GroupError> {
        if from == self.me {
            return Err(GroupError::NotForMe);
        }
        let entry = bundle.find_for(&self.me).ok_or(GroupError::NotForMe)?;
        let ct: PairwiseCt = decode(&entry.ct)?;
        self.check_identity(txn, backend, &from, &expected_x25519)?;
        let plain = PairwiseBackend::decrypt(backend, txn, &from, &ct, rng)?;
        self.accept_skdm(txn, backend, recog, from, expected_x25519, &plain)
    }

    fn check_identity(
        &self,
        txn: &dyn crate::storage::kv::KvTxn,
        backend: &dyn Backend,
        from: &PeerId,
        expected_x25519: &[u8; 32],
    ) -> Result<(), GroupError> {
        match backend.remote_session_key(txn, from) {
            Some(k) if k == *expected_x25519 => Ok(()),
            _ => Err(GroupError::IdentityMismatch),
        }
    }

    /// Encrypts a post under the own chain and cuts it into cells (module
    /// docs "Posts"). The relay class is [`PostPlain::class`]. Refused while
    /// a rotation is pending or before the first rotation.
    pub fn encrypt_post(
        &mut self,
        txn: &mut dyn crate::storage::kv::KvTxn,
        backend: &dyn Backend,
        recog: &mut TagTable,
        plain: &PostPlain,
        rng: &mut dyn CryptoRng,
    ) -> Result<Vec<Cell>, GroupError> {
        if self.pending_rotation {
            return Err(GroupError::RotationPending);
        }
        let bytes = plain.try_encode()?;
        let own = self.own.as_mut().ok_or(GroupError::NoOwnChain)?;
        let ct = GroupBackend::encrypt(backend, txn, &self.group, &bytes, rng)?;
        let frags = inner::fragment(InnerKind::SenderKey, own.next_obj_seq, &ct)?;
        Self::cells_for(recog, own, &frags, rng)
    }

    /// Issues one cell per fragment under the own chain and advances the
    /// object counter.
    fn cells_for(
        recog: &mut TagTable,
        own: &mut OwnChain,
        frags: &[[u8; cell::BODY_PLAINTEXT_LEN]],
        rng: &mut dyn CryptoRng,
    ) -> Result<Vec<Cell>, GroupError> {
        let next = own
            .next_obj_seq
            .checked_add(1)
            .ok_or(GroupError::ObjSeqExhausted)?;
        let mut cells = Vec::with_capacity(frags.len());
        for frag in frags {
            let issued = recog.sender_next(own.key)?;
            let ttl = cell::draw_initial_ttl(rng);
            cells.push(cell::seal_cell(
                VERSION,
                ttl,
                &issued.tag,
                own.epoch.k_hdr().as_bytes(),
                issued.ctr,
                0,
                frag,
            ));
        }
        own.next_obj_seq = next;
        Ok(cells)
    }

    /// Opens one recognised cell, feeds the chain's reassembler and, when an
    /// object completes, decrypts it (module docs "Posts"). The hit is
    /// consumed only after the body authenticated. `Ok(None)` while
    /// fragments are missing or for a duplicate fragment.
    pub fn decrypt_cell(
        &mut self,
        txn: &mut dyn crate::storage::kv::KvTxn,
        backend: &dyn Backend,
        recog: &mut TagTable,
        hit: Hit,
        cell: &Cell,
    ) -> Result<Option<Inbound>, GroupError> {
        let (sender, _chain) = *self
            .chain_keys
            .get(&hit.chain)
            .ok_or(GroupError::UnknownChain)?;
        let epoch = recog
            .epoch(hit.chain, hit.epoch_id)
            .ok_or(RecogError::UnknownEpoch)?;
        let plain = cell.open_body(epoch.k_hdr().as_bytes(), hit.ctr, 0)?;
        recog.consume(hit)?;
        let reassembler = self.reassemblers.entry(hit.chain).or_default();
        let Some((kind, obj_seq, object)) = reassembler.push(hit.chain.0, &plain)? else {
            return Ok(None);
        };
        match kind {
            InnerKind::SenderKey => {
                let (chain, bytes) =
                    GroupBackend::decrypt(backend, txn, &self.group, &sender, &object)?;
                let post: PostPlain = decode(&bytes)?;
                Ok(Some(Inbound::Post {
                    sender,
                    chain,
                    obj_seq,
                    post,
                }))
            }
            InnerKind::SkdmBundle => {
                let bundle: SkdmBundle = decode(&object)?;
                Ok(Some(Inbound::Bundle {
                    sender,
                    obj_seq,
                    bundle,
                }))
            }
            _ => Err(GroupError::WrongKind),
        }
    }

    /// [`decrypt_cell`](Self::decrypt_cell) over several recognised cells in
    /// order; stops at the first error.
    pub fn decrypt_cells(
        &mut self,
        txn: &mut dyn crate::storage::kv::KvTxn,
        backend: &dyn Backend,
        recog: &mut TagTable,
        cells: &[(Hit, Cell)],
    ) -> Result<Vec<Inbound>, GroupError> {
        let mut out = Vec::new();
        for (hit, cell) in cells {
            if let Some(obj) = self.decrypt_cell(txn, backend, recog, *hit, cell)? {
                out.push(obj);
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::ed25519::IdentitySigner;

    fn peer(seed: u8) -> PeerId {
        IdentitySigner::from_seed(&[seed; 32]).public()
    }

    #[test]
    fn chain_key_is_the_documented_hash_and_distinct() {
        let g = GroupId([1; 16]);
        let p = peer(1);
        let mut bytes = b"meshapp.v1.chainkey.group".to_vec();
        bytes.extend_from_slice(&[1; 16]);
        bytes.extend_from_slice(p.as_bytes());
        bytes.extend_from_slice(&7u32.to_le_bytes());
        let expect = u64::from_le_bytes(crate::crypto::hash::sha256(&bytes).truncate8());
        assert_eq!(group_chain_key(&g, &p, ChainId(7)), ChainKey(expect));
        assert_ne!(
            group_chain_key(&g, &p, ChainId(7)),
            group_chain_key(&g, &p, ChainId(8))
        );
        assert_ne!(
            group_chain_key(&g, &p, ChainId(7)),
            group_chain_key(&g, &peer(2), ChainId(7))
        );
        assert_ne!(
            group_chain_key(&g, &p, ChainId(7)),
            group_chain_key(&GroupId([2; 16]), &p, ChainId(7))
        );
        assert_eq!(group_epoch_id(ChainId(9)), EpochId(9));
    }

    #[test]
    fn flags_and_restore_bookkeeping() {
        let mut g = GroupState::new(GroupId([3; 16]), peer(1));
        assert!(g.needs_rotation());
        assert!(!g.rotation_pending());
        g.mark_rotation_pending();
        g.mark_rotation_pending();
        assert!(g.rotation_pending());
        let mut recog = TagTable::new();
        g.restore_own(&mut recog, ChainId(5), &[9u8; 32], 12, 3);
        assert_eq!(g.own_chain(), Some(ChainId(5)));
        assert_eq!(g.own_next_obj_seq(), Some(3));
        let key = group_chain_key(&GroupId([3; 16]), &peer(1), ChainId(5));
        assert_eq!(recog.sender_next_ctr(key), Some(12));
        assert!(g.needs_rotation(), "flag still set");
        let k2 = g.restore_member_chain(peer(2), ChainId(8));
        assert_eq!(g.chain_owner(k2), Some((peer(2), ChainId(8))));
        assert_eq!(g.member_chains().count(), 1);
        recog.register(k2, EpochId(8), RecogEpoch::from_seed(&[1; 32]), 0);
        g.forget_member(&mut recog, &peer(2));
        assert_eq!(g.chain_owner(k2), None);
        assert!(recog.chain(k2).is_none());
        // posting is refused while the flag is set, before touching the backend
        let backend = crate::crypto::backend::default_backend();
        let mut kv = crate::storage::kv::MemKv::new();
        let mut rng = <rand_chacha::ChaCha20Rng as rand::SeedableRng>::seed_from_u64(1);
        let post = PostPlain::recog_seed(&[0; 32], 0);
        assert_eq!(
            g.encrypt_post(&mut kv, &*backend, &mut recog, &post, &mut rng)
                .err(),
            Some(GroupError::RotationPending)
        );
        let mut fresh = GroupState::new(GroupId([3; 16]), peer(1));
        assert_eq!(
            fresh
                .encrypt_post(&mut kv, &*backend, &mut recog, &post, &mut rng)
                .err(),
            Some(GroupError::NoOwnChain)
        );
        // an unknown chain hit is refused without consuming anything
        let hit = Hit {
            chain: ChainKey(1),
            epoch_id: EpochId(0),
            ctr: 0,
        };
        let cell = Cell::from_array([0u8; cell::CELL_LEN]);
        assert_eq!(
            fresh
                .decrypt_cell(&mut kv, &*backend, &mut recog, hit, &cell)
                .err(),
            Some(GroupError::UnknownChain)
        );
        assert!(kv.is_empty());
    }

    #[test]
    fn accept_skdm_rejects_malformed_and_foreign_entries_before_any_write() {
        let backend = crate::crypto::backend::default_backend();
        let mut kv = crate::storage::kv::MemKv::new();
        let mut recog = TagTable::new();
        let mut g = GroupState::new(GroupId([3; 16]), peer(1));
        let from = peer(2);
        assert_eq!(
            g.accept_skdm(&mut kv, &*backend, &mut recog, from, [0; 32], &[1]),
            Err(GroupError::Codec(CodecError::UnexpectedEof {
                needed: 1,
                remaining: 0
            }))
        );
        let text = PostPlain::recog_seed(&[0; 32], 0).encode();
        assert_eq!(
            g.accept_skdm(&mut kv, &*backend, &mut recog, from, [0; 32], &text),
            Err(GroupError::WrongKind)
        );
        let other = SkdmPayload {
            group: GroupId([4; 16]),
            skdm: vec![],
            seed: [0; 32],
        };
        let post = PostPlain::new(kind::POST_SKDM_BUNDLE, vec![], other.encode())
            .unwrap()
            .encode();
        assert_eq!(
            g.accept_skdm(&mut kv, &*backend, &mut recog, from, [0; 32], &post),
            Err(GroupError::WrongGroup)
        );
        let mine = SkdmPayload {
            group: GroupId([3; 16]),
            skdm: vec![],
            seed: [0; 32],
        };
        let post = PostPlain::new(kind::POST_SKDM_BUNDLE, vec![], mine.encode())
            .unwrap()
            .encode();
        // no session with `from`: the identity check fails first
        assert_eq!(
            g.accept_skdm(&mut kv, &*backend, &mut recog, from, [0; 32], &post),
            Err(GroupError::IdentityMismatch)
        );
        assert_eq!(
            g.accept_skdm(&mut kv, &*backend, &mut recog, peer(1), [0; 32], &post),
            Err(GroupError::NotForMe)
        );
        assert!(kv.is_empty());
        assert!(recog.is_empty());
        assert_eq!(g.member_chains().count(), 0);
    }
}
