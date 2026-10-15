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
//! * The [`PairwiseManager`] owns the 1:1 session bookkeeping with every
//!   member: SKDM entries are encrypted and decrypted **through it**, never
//!   through the backend directly, so an SKDM that is the first message to
//!   a member (a PreKey message) leaves that contact's state (`Ready`,
//!   `sent_prekey`/`got_prekey`, crossed-initiation detection) correct, as
//!   any other first message would (D6 rules 5 and 7).
//! * [`GroupState`] holds the mapping from recognised chains back to
//!   `(member, chain id)`, the own chain's object counter and SKDM
//!   material, a reassembler per chain and the pending-rotation flag.
//!
//! # Rotation (D3 K2; DESIGN §3)
//!
//! `pending_rotation` is set by the roster/rotation policy (any valid Remove
//! entering the DAG, or 24 h of powered-on time: `roster::rotation`); this
//! module only consumes the flag. [`GroupState::rotate`] runs before the
//! next post: `new_sender_chain` → a fresh epoch seed → one SKDM entry per
//! current member (pairwise-encrypted, DESIGN §2.4 "one bundle of n−1
//! pairwise ciphertexts"), padded ([`SkdmBundle`]: every entry to the
//! longest ciphertext, the count to 8/16/32/64) and, when an old chain
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
//! # Late members (DECISIONS §3 row (3,4,6,10) "a 50-tag window per chain")
//!
//! The SKDM payload carries the sender's **current** recognition counter
//! (`next_ctr`, [`SkdmPayload`]) and the recipient opens its window there.
//! A member that had no session at rotation time ([`Rotation::without_session`],
//! "Needs their key", D6 rule 7) gets the current chain's SKDM later from
//! [`GroupState::skdm_entry_for`], with the counter the sender is at by
//! then, so it recognises the sender's next cell even after the sender has
//! issued more than [`WINDOW`] cells; the same call re-sends the SKDM to a
//! member that lost it. The chain's SKDM bytes and epoch seed are kept for
//! this ([`GroupState::own_skdm`]) and restored after a restart with
//! [`GroupState::restore_own_skdm`]; without them `skdm_entry_for` reports
//! [`GroupError::SkdmUnavailable`] and the caller rotates instead.
//!
//! # SKDM acceptance (BRIEF §5.2 "accepted only over a pairwise session whose
//! key matches the sender's roster entry, checked and stored in one atomic
//! step"; D3 K1)
//!
//! [`GroupState::accept_skdm`] compares the X25519 key of the pairwise
//! session the SKDM arrived on (`remote_session_key`) with the key the
//! node took from the sender's roster KeyCard, and only then stores the
//! chain (`process_skdm`) and registers its recognition epoch at the
//! payload's `next_ctr`, all in the same transaction. On a mismatch nothing
//! is written. [`GroupState::accept_bundle`] runs that check before the
//! pairwise decrypt when the entry is a Normal message (a mismatch costs no
//! ratchet step) and right after it when the entry is a PreKey message,
//! the sender's first message to this node, whose session exists only once
//! the decrypt established it. A re-delivered SKDM for a known `(chain, epoch)` with the
//! same seed is not re-registered while its window still reaches the
//! payload's counter (the tag table would re-accept consumed counters;
//! [`TagTable::register`] docs); when the counter lies at or beyond the
//! window's end (the member missed more than a window of cells and the
//! sender re-sent the SKDM), the window is moved there, which re-accepts
//! nothing: every counter consumed so far is below it. A different seed
//! for a known chain replaces the epoch.
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
use zeroize::{Zeroize, Zeroizing};

use crate::codec::{CodecError, Encode, decode};
use crate::crypto::backend::{
    Backend, BackendError, ChainId, GroupBackend, GroupId, PairwiseCt, PairwiseKind, PeerId,
};
use crate::crypto::hash::sha256_concat;
use crate::objects::chat_post::{PostError, PostPlain, kind};
use crate::objects::skdm_bundle::{SkdmBundle, SkdmEntry, SkdmError, SkdmPayload};
use crate::pairwise::session::{PairwiseError, PairwiseManager, Received};
use crate::wire::cell::{self, Cell, CellReject, VERSION};
use crate::wire::inner::{self, InnerError, InnerKind, Reassembler};
use crate::wire::recog::{
    ChainKey, EpochId, Hit, RecogEpoch, RecogError, SEED_LEN, TagTable, WINDOW,
};

/// Domain of the local chain-key derivation.
pub const CHAIN_KEY_DOMAIN: &[u8] = b"meshapp.v1.chainkey.group";

/// [`WINDOW`] as a counter span.
const WINDOW_SPAN: u64 = WINDOW as u64;

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
    /// The own chain's SKDM bytes or epoch seed are not held (restored from
    /// the epoch keys alone): no entry can be re-sent, rotate instead.
    #[error("the own chain's SKDM is not held; rotate instead")]
    SkdmUnavailable,
    /// [`GroupState::restore_own_skdm`] was given a seed that does not derive
    /// the own chain's live epoch keys: handing it to a late member would
    /// let it compute tags this node never issues. Nothing was kept.
    #[error("the restored seed does not derive the own chain's epoch keys")]
    SeedMismatch,
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
    #[error("pairwise: {0}")]
    Pairwise(#[from] PairwiseError),
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
    /// The chain's SKDM bytes and epoch seed, for late SKDM entries
    /// (module docs "Late members"); `None` after a restore without them.
    skdm: Option<Zeroizing<Vec<u8>>>,
    seed: Option<Zeroizing<[u8; SEED_LEN]>>,
}

/// What [`GroupState::rotate`] produced.
#[derive(Clone)]
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
    /// a session is established, from [`GroupState::skdm_entry_for`].
    pub without_session: Vec<PeerId>,
    /// The plaintext every real entry encrypts: the
    /// [`kind::POST_SKDM_BUNDLE`] post around the [`SkdmPayload`] with
    /// `next_ctr = 0` (the new chain has issued nothing yet). Secret (it
    /// holds the chain key and the epoch seed); wiped on drop.
    pub entry_plain: Zeroizing<Vec<u8>>,
}

impl core::fmt::Debug for Rotation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Rotation")
            .field("chain", &self.chain)
            .field("bundle", &self.bundle)
            .field("cells", &self.cells.len())
            .field("recipients", &self.recipients)
            .field("without_session", &self.without_session)
            .field("entry_plain", &"<redacted>")
            .finish()
    }
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
    /// recognition counter and the next object counter. The seed is kept
    /// for late SKDM entries; the SKDM bytes come from
    /// [`restore_own_skdm`](Self::restore_own_skdm).
    pub fn restore_own(
        &mut self,
        recog: &mut TagTable,
        chain: ChainId,
        seed: &[u8; SEED_LEN],
        next_ctr: u64,
        next_obj_seq: u32,
    ) {
        self.restore_own_epoch(
            recog,
            chain,
            RecogEpoch::from_seed(seed),
            next_ctr,
            next_obj_seq,
        );
        if let Some(own) = self.own.as_mut() {
            own.seed = Some(Zeroizing::new(*seed));
        }
    }

    /// [`restore_own`](Self::restore_own) from the persisted epoch *keys*
    /// rather than the seed: the seed is not retained after a rotation
    /// (it travels inside the SKDM and is wiped), so the node persists
    /// [`own_epoch`](Self::own_epoch) and restores from it. Until
    /// [`restore_own_skdm`](Self::restore_own_skdm) follows,
    /// [`skdm_entry_for`](Self::skdm_entry_for) reports
    /// [`GroupError::SkdmUnavailable`].
    pub fn restore_own_epoch(
        &mut self,
        recog: &mut TagTable,
        chain: ChainId,
        epoch: RecogEpoch,
        next_ctr: u64,
        next_obj_seq: u32,
    ) {
        let key = group_chain_key(&self.group, &self.me, chain);
        recog.register_sender(key, group_epoch_id(chain), epoch.clone(), next_ctr);
        self.own = Some(OwnChain {
            chain,
            key,
            epoch,
            next_obj_seq,
            skdm: None,
            seed: None,
        });
    }

    /// The own chain's recognition epoch keys, for persistence (the node
    /// stores them in the encrypted `chains` namespace and restores with
    /// [`restore_own_epoch`](Self::restore_own_epoch)).
    pub fn own_epoch(&self) -> Option<&RecogEpoch> {
        self.own.as_ref().map(|o| &o.epoch)
    }

    /// The own chain's SKDM bytes and epoch seed, for persistence (secret:
    /// the encrypted `chains` namespace) and for
    /// [`skdm_entry_for`](Self::skdm_entry_for). `None` without an own
    /// chain or after a restore that did not bring them back.
    pub fn own_skdm(&self) -> Option<(&[u8], &[u8; SEED_LEN])> {
        let own = self.own.as_ref()?;
        Some((own.skdm.as_deref()?.as_slice(), own.seed.as_deref()?))
    }

    /// Restores the own chain's SKDM bytes and epoch seed after a restart
    /// (persisted from [`own_skdm`](Self::own_skdm)). The seed must be the
    /// one the current epoch keys were derived from: one that is not is
    /// refused ([`GroupError::SeedMismatch`]) rather than handed to late
    /// members, whose tags would then never match this node's cells.
    /// Fails without an own chain. The SKDM bytes are wiped either way.
    pub fn restore_own_skdm(
        &mut self,
        skdm: Vec<u8>,
        seed: &[u8; SEED_LEN],
    ) -> Result<(), GroupError> {
        let skdm = Zeroizing::new(skdm);
        let own = self.own.as_mut().ok_or(GroupError::NoOwnChain)?;
        if RecogEpoch::from_seed(seed) != own.epoch {
            return Err(GroupError::SeedMismatch);
        }
        own.skdm = Some(skdm);
        own.seed = Some(Zeroizing::new(*seed));
        Ok(())
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

    /// The plaintext of one SKDM entry: the [`kind::POST_SKDM_BUNDLE`] post
    /// around the [`SkdmPayload`] (DESIGN §2.4).
    fn entry_plain(
        group: &GroupId,
        skdm: &[u8],
        seed: &[u8; SEED_LEN],
        next_ctr: u32,
    ) -> Result<Zeroizing<Vec<u8>>, GroupError> {
        let payload = SkdmPayload {
            group: *group,
            skdm: skdm.to_vec(),
            seed: *seed,
            next_ctr,
        };
        let mut post = PostPlain::new(kind::POST_SKDM_BUNDLE, Vec::new(), payload.try_encode()?)?;
        let plain = post.try_encode().map(Zeroizing::new);
        // the post's body is a second copy of the payload: wipe it too
        post.body.zeroize();
        Ok(plain?)
    }

    /// Rotates this node's Sender Key (module docs "Rotation"). `members` is
    /// the current roster view (`members(current heads)`); this node is
    /// skipped if listed. Every entry goes through `pairwise`
    /// ([`PairwiseManager::encrypt_to`]), so a member whose first message
    /// this is ends up `Ready` like after any first message; a member with
    /// no session ("Needs their key") is listed in
    /// [`Rotation::without_session`]. Clears the pending flag.
    #[allow(clippy::too_many_arguments)] // txn, backend, recog, pairwise, members, time and rng are all inputs
    pub fn rotate(
        &mut self,
        txn: &mut dyn crate::storage::kv::KvTxn,
        backend: &dyn Backend,
        recog: &mut TagTable,
        pairwise: &mut PairwiseManager,
        members: &[PeerId],
        now_mono: u64,
        rng: &mut dyn CryptoRng,
    ) -> Result<Rotation, GroupError> {
        let (chain, skdm) = backend.new_sender_chain(txn, &self.group, rng)?;
        let skdm = Zeroizing::new(skdm);
        let mut seed = Zeroizing::new([0u8; SEED_LEN]);
        rng.fill_bytes(seed.as_mut());
        let epoch = RecogEpoch::from_seed(&seed);
        let entry_plain = Self::entry_plain(&self.group, &skdm, &seed, 0)?;

        let mut sorted: Vec<PeerId> = members.iter().copied().filter(|m| *m != self.me).collect();
        sorted.sort_unstable();
        sorted.dedup();
        let mut entries = Vec::with_capacity(sorted.len());
        let mut recipients = Vec::with_capacity(sorted.len());
        let mut without_session = Vec::new();
        for peer in sorted {
            match pairwise.encrypt_to(txn, backend, &peer, &entry_plain, now_mono, rng) {
                Ok(ct) => {
                    entries.push(SkdmEntry::new(&peer, chain, ct.encode()));
                    recipients.push((peer, ct));
                }
                Err(PairwiseError::NeedsTheirKey) => without_session.push(peer),
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
                let object = bundle.try_encode()?;
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
            skdm: Some(skdm),
            seed: Some(seed),
        });
        self.pending_rotation = false;
        Ok(Rotation {
            chain,
            bundle,
            cells,
            recipients,
            without_session,
            entry_plain,
        })
    }

    /// The current chain's SKDM as one entry for `peer`, encrypted through
    /// `pairwise` with the recognition counter this node is at **now**
    /// (module docs "Late members"): for a member that gained a session
    /// after the rotation, or a re-send. The caller delivers the ciphertext
    /// pairwise ([`PairwiseManager::cells_for_ct`]); the recipient's
    /// [`accept_skdm`](Self::accept_skdm) opens its window at that counter.
    /// Errors: [`GroupError::NoOwnChain`], [`GroupError::SkdmUnavailable`]
    /// (rotate instead), [`PairwiseError::NeedsTheirKey`] (still no
    /// session), [`RecogError::NoSender`] (the own chain is not in the tag
    /// table), [`RecogError::CounterExhausted`] (the counter does not fit
    /// the payload's `u32`; rotate).
    #[allow(clippy::too_many_arguments)] // txn, backend, recog, pairwise, peer, time and rng are all inputs
    pub fn skdm_entry_for(
        &self,
        txn: &mut dyn crate::storage::kv::KvTxn,
        backend: &dyn Backend,
        recog: &TagTable,
        pairwise: &mut PairwiseManager,
        peer: &PeerId,
        now_mono: u64,
        rng: &mut dyn CryptoRng,
    ) -> Result<SkdmEntry, GroupError> {
        if *peer == self.me {
            return Err(GroupError::NotForMe);
        }
        let own = self.own.as_ref().ok_or(GroupError::NoOwnChain)?;
        let (skdm, seed) = match (&own.skdm, &own.seed) {
            (Some(skdm), Some(seed)) => (skdm, seed),
            _ => return Err(GroupError::SkdmUnavailable),
        };
        let next_ctr = recog.sender_next_ctr(own.key).ok_or(RecogError::NoSender)?;
        let next_ctr = u32::try_from(next_ctr).map_err(|_| RecogError::CounterExhausted)?;
        let plain = Self::entry_plain(&self.group, skdm, seed, next_ctr)?;
        let ct = pairwise.encrypt_to(txn, backend, peer, &plain, now_mono, rng)?;
        Ok(SkdmEntry::new(peer, own.chain, ct.encode()))
    }

    /// Accepts one SKDM entry's decrypted plaintext from `from` (module docs
    /// "SKDM acceptance"): the atomic identity check against
    /// `expected_x25519` (the X25519 key of `from`'s roster KeyCard, supplied
    /// by the node) precedes every write; on success the chain is stored and
    /// its recognition epoch registered at the payload's counter in the
    /// same transaction. Returns the chain id.
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
        let mut post: PostPlain = decode(entry_plain)?;
        let r = self.accept_skdm_post(txn, backend, recog, from, expected_x25519, &post);
        // the body holds the SKDM and the seed: wipe this copy
        post.body.zeroize();
        r
    }

    /// [`accept_skdm`](Self::accept_skdm) for an already decoded post (what
    /// the 1:1 receive path hands over).
    pub fn accept_skdm_post(
        &mut self,
        txn: &mut dyn crate::storage::kv::KvTxn,
        backend: &dyn Backend,
        recog: &mut TagTable,
        from: PeerId,
        expected_x25519: [u8; 32],
        post: &PostPlain,
    ) -> Result<ChainId, GroupError> {
        if from == self.me {
            return Err(GroupError::NotForMe);
        }
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
        let next_ctr = u64::from(payload.next_ctr);
        if window_needs_register(recog, key, epoch_id, &epoch, next_ctr) {
            recog.register(key, epoch_id, epoch, next_ctr);
        }
        self.members
            .entry(from)
            .or_default()
            .insert(chain, epoch_id);
        self.chain_keys.insert(key, (from, chain));
        Ok(chain)
    }

    /// Finds this node's entry in `bundle`, decrypts it on the pairwise
    /// session with `from` through `pairwise` (so the contact's state
    /// follows a PreKey entry like any first message) and accepts it
    /// ([`accept_skdm_post`](Self::accept_skdm_post)). For a Normal entry
    /// the identity check runs before the pairwise decrypt, so a mismatch
    /// consumes no ratchet step and writes nothing. For a PreKey entry (the
    /// sender's first message to this node) the session exists only once
    /// the decrypt established it, so the check runs right after the
    /// decrypt and before any store write, in the same transaction (D3 K1);
    /// the backend itself refuses a PreKey message from an unpinned or
    /// re-keyed identity (BRIEF §5.2). On a mismatch after a PreKey decrypt
    /// nothing of the group is written, but the 1:1 session stays in the
    /// caller's transaction (it is bound to the pinned key, which is what
    /// disagreed with the roster); a caller that wants none of it rolls
    /// the transaction back.
    #[allow(clippy::too_many_arguments)] // txn, backend, recog, pairwise and the identity inputs are all required
    pub fn accept_bundle(
        &mut self,
        txn: &mut dyn crate::storage::kv::KvTxn,
        backend: &dyn Backend,
        recog: &mut TagTable,
        pairwise: &mut PairwiseManager,
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
        if ct.kind == PairwiseKind::Normal {
            self.check_identity(txn, backend, &from, &expected_x25519)?;
        }
        // `accept_skdm_post` repeats the check after the decrypt, before
        // the store: the atomic step for both kinds.
        match pairwise.decrypt_from(txn, backend, recog, &from, &ct, rng)? {
            Received::Post(mut post) => {
                let r = self.accept_skdm_post(txn, backend, recog, from, expected_x25519, &post);
                // the body holds the SKDM and the seed: wipe this copy
                post.body.zeroize();
                r
            }
            Received::RecogSeed { .. } => Err(GroupError::WrongKind),
        }
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

/// Whether an SKDM naming `(chain, epoch)` with `epoch` keys and sender
/// counter `next_ctr` must (re-)register the receive window (module docs
/// "SKDM acceptance"): yes for a new epoch or a different seed; for the
/// same seed only when the current window ends at or before `next_ctr`,
/// where a fresh window re-accepts no consumed counter (all of them lie
/// below the old window's end).
pub fn window_needs_register(
    recog: &TagTable,
    chain: ChainKey,
    epoch_id: EpochId,
    epoch: &RecogEpoch,
    next_ctr: u64,
) -> bool {
    match (recog.epoch(chain, epoch_id), recog.window(chain, epoch_id)) {
        (Some(known), Some(w)) if known == epoch => next_ctr >= w.base.saturating_add(WINDOW_SPAN),
        _ => true,
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
        assert_eq!(
            g.restore_own_skdm(vec![1], &[0; 32]),
            Err(GroupError::NoOwnChain)
        );
        g.restore_own(&mut recog, ChainId(5), &[9u8; 32], 12, 3);
        assert_eq!(g.own_chain(), Some(ChainId(5)));
        assert_eq!(g.own_next_obj_seq(), Some(3));
        assert_eq!(g.own_epoch(), Some(&RecogEpoch::from_seed(&[9u8; 32])));
        assert_eq!(g.own_skdm(), None, "the seed alone is not enough");
        // a seed that does not derive the live epoch keys is refused: a
        // late member would get a seed whose tags the sender never issues
        assert_eq!(
            g.restore_own_skdm(vec![1, 2], &[8u8; 32]),
            Err(GroupError::SeedMismatch)
        );
        assert_eq!(g.own_skdm(), None, "nothing kept from the refusal");
        g.restore_own_skdm(vec![1, 2], &[9u8; 32]).unwrap();
        assert_eq!(g.own_skdm(), Some((&[1u8, 2][..], &[9u8; 32])));
        let key = group_chain_key(&GroupId([3; 16]), &peer(1), ChainId(5));
        assert_eq!(recog.sender_next_ctr(key), Some(12));
        assert!(g.needs_rotation(), "flag still set");
        // the keys-only restore drops the SKDM material
        g.restore_own_epoch(
            &mut recog,
            ChainId(5),
            RecogEpoch::from_seed(&[9u8; 32]),
            12,
            3,
        );
        assert_eq!(g.own_skdm(), None);
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
        // no late entry without an own chain, or without its SKDM
        let mut pm = PairwiseManager::new(peer(1));
        assert_eq!(
            fresh
                .skdm_entry_for(&mut kv, &*backend, &recog, &mut pm, &peer(2), 0, &mut rng)
                .err(),
            Some(GroupError::NoOwnChain)
        );
        assert_eq!(
            g.skdm_entry_for(&mut kv, &*backend, &recog, &mut pm, &peer(2), 0, &mut rng)
                .err(),
            Some(GroupError::SkdmUnavailable)
        );
        assert_eq!(
            g.skdm_entry_for(&mut kv, &*backend, &recog, &mut pm, &peer(1), 0, &mut rng)
                .err(),
            Some(GroupError::NotForMe)
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
    fn a_window_is_moved_only_past_its_end_and_never_over_consumed_counters() {
        let mut recog = TagTable::new();
        let (key, id) = (ChainKey(7), EpochId(7));
        let epoch = RecogEpoch::from_seed(&[4; 32]);
        assert!(window_needs_register(&recog, key, id, &epoch, 0));
        assert!(window_needs_register(&recog, key, id, &epoch, 120));
        recog.register(key, id, epoch.clone(), 0);
        // the same seed within the window: keep it (a replayed SKDM must
        // not re-accept consumed counters)
        for ctr in 0..WINDOW_SPAN {
            assert!(
                !window_needs_register(&recog, key, id, &epoch, ctr),
                "{ctr}"
            );
        }
        // at or beyond its end: move it
        assert!(window_needs_register(&recog, key, id, &epoch, WINDOW_SPAN));
        assert!(window_needs_register(&recog, key, id, &epoch, 200));
        // a different seed under the same id: replace
        let other = RecogEpoch::from_seed(&[5; 32]);
        assert!(window_needs_register(&recog, key, id, &other, 3));
        // after consuming, the window slides and the rule follows its base
        let hit = recog.lookup(&epoch.tag_for(40)).unwrap();
        recog.consume(hit).unwrap();
        let w = recog.window(key, id).unwrap();
        assert!(w.base > 0);
        assert!(!window_needs_register(&recog, key, id, &epoch, w.base + 49));
        assert!(window_needs_register(&recog, key, id, &epoch, w.base + 50));
        // moving it there re-accepts nothing: every consumed counter is
        // below the new base
        recog.register(key, id, epoch.clone(), w.base + 50);
        assert_eq!(recog.lookup(&epoch.tag_for(40)), None);
        assert!(recog.lookup(&epoch.tag_for(w.base + 50)).is_some());
        assert!(window_needs_register(&recog, key, id, &epoch, u64::MAX));
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
            next_ctr: 0,
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
            next_ctr: 0,
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
        // a bundle without my entry, or addressed by me: nothing touched
        let mut pm = PairwiseManager::new(peer(1));
        let mut rng = <rand_chacha::ChaCha20Rng as rand::SeedableRng>::seed_from_u64(1);
        let bundle = SkdmBundle::build(ChainId(1), vec![], &mut rng).unwrap();
        assert_eq!(
            g.accept_bundle(
                &mut kv, &*backend, &mut recog, &mut pm, from, [0; 32], &bundle, &mut rng
            ),
            Err(GroupError::NotForMe)
        );
        assert_eq!(
            g.accept_bundle(
                &mut kv,
                &*backend,
                &mut recog,
                &mut pm,
                peer(1),
                [0; 32],
                &bundle,
                &mut rng
            ),
            Err(GroupError::NotForMe)
        );
        assert!(kv.is_empty());
        assert!(recog.is_empty());
        assert_eq!(g.member_chains().count(), 0);
        assert_eq!(pm.contacts().count(), 0);
    }
}
