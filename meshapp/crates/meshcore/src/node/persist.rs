//! Persisted layout of the node's protocol state (docs/DESIGN.md §6
//! "Tables", §1 row `storage/kv.rs`; BRIEF_v2 §5.3 "State is committed
//! atomically"; D6 rule 4). One loader ([`load`]) serves both the restart
//! and the rollback after a failed transaction (see [`super`]).
//!
//! Namespaces (`storage::kv::ns` names the secret ones, encrypted at rest):
//!
//! | namespace | key | value |
//! |---|---|---|
//! | `identity` (secret) | `node/identity` | `ver ‖ seed [32] ‖ name str16 ‖ envelope secret [32]` |
//! | `chains` (secret) | `recog` | every recognition chain: receive epochs (id, keys, window) and the sender epoch (id, keys, next counter) |
//! | `chains` (secret) | `pairwise` | one record per 1:1 contact (`PairwiseManager::export`) |
//! | `chains` (secret) | `bundles` | `BundleManager` (SPK generations, base-key cache) |
//! | `chains` (secret) | `group/<id>` | own chain keys and counters, members' chain ids, rotation flag and policy, SKDM holders, own identity-signed sequence |
//! | `chains` (secret) | `parked/<group><peer><seq>` | a group object awaiting its sender's SKDM |
//! | `plaintext` (secret) | obj id | `conv ‖ author ‖ obj_seq ‖ kind ‖ group? ‖ post plaintext` |
//! | `outbox` (secret) | obj id | delivery record of an object this node sent |
//! | `roster_ops/<group hex>` | op hash | roster op bytes |
//! | `tasks/<group hex>` | task id ‖ op hash | `author ‖ post plaintext` |
//! | `task_meta/<group hex>` | task id | `taken_at ‖ seen_beyond ‖ own Take cells` |
//! | `votes` | `o` ‖ vote id / `x` ‖ op hash / `b` ‖ vote id ‖ key ‖ choice | opening (with its group) / rejected opening / ballot |
//! | `dedupe` | msg id | `expires_mono ‖ rejected` |
//! | `receipts` | conv | received object counters as ranges |
//! | `heard` | peer | powered-on time last heard |
//!
//! Every blob starts with a version byte and is decoded with the canonical
//! codec (trailing bytes rejected). Sizes are bounded on decode so a
//! corrupt row cannot allocate without limit.

use std::collections::{BTreeMap, BTreeSet};

use zeroize::Zeroizing;

use crate::codec::{CodecError, Decode, Encode, Reader, Writer, decode};
use crate::crypto::aead::Key32;
use crate::crypto::backend::{Backend, ChainId, GroupId, PeerId};
use crate::crypto::ed25519::{IdentityKey, IdentitySigner, admit_public_key};
use crate::crypto::hash::Hash32;
use crate::crypto::keycard::KeyCard;
use crate::groups::group::{GroupState, group_chain_key};
use crate::objects::ballot::Ballot;
use crate::objects::chat_post::PostPlain;
use crate::objects::roster_op::RosterOp;
use crate::objects::sender_chain::SenderChainIndex;
use crate::objects::task_op::{TaskId, TaskOp};
use crate::objects::vote_open::VoteOpen;
use crate::pairwise::bundles::BundleManager;
use crate::pairwise::session::{ContactRecord, PairwiseManager, SessionState, pair_chain_key};
use crate::people::contacts::{Badge, ContactStore};
use crate::people::samename::SameNameIndex;
use crate::roster::dag::RosterDag;
use crate::roster::eval::evaluate;
use crate::roster::rotation::RotationPolicy;
use crate::storage::db::Db;
use crate::storage::kv::{KvTxn, ns};
use crate::wire::recog::{ChainKey, EpochId, RecogEpoch, TagTable, WindowState};
use crate::wire::seal::EnvelopeKeyPair;

use super::node::{
    ContactLite, Dirty, GroupCtx, Identity, OutboxEntry, Proto, Target, TaskCtx, VoteCtx,
};
use super::reasm::Reasm;
use super::status::DeliveryState;
use super::{NodeConfig, NodeError, NodeRole, ObjId, VoteId};

/// Version byte of every node blob.
pub const BLOB_VERSION: u8 = 1;
/// Most items any one list may hold on decode (corruption bound).
pub const MAX_ITEMS: usize = 1 << 20;

pub const K_IDENTITY: &[u8] = b"node/identity";
pub const K_RECOG: &[u8] = b"recog";
pub const K_PAIRWISE: &[u8] = b"pairwise";
pub const K_BUNDLES: &[u8] = b"bundles";
pub const K_GROUP_PREFIX: &[u8] = b"group/";
pub const K_PARKED_PREFIX: &[u8] = b"parked/";
pub const K_SKDM_PREFIX: &[u8] = b"skdm/";
pub const NS_VOTES: &str = "votes";
pub const NS_DEDUPE: &str = "dedupe";
pub const NS_RECEIPTS: &str = "receipts";
pub const NS_HEARD: &str = "heard";

/// Namespace of a group's roster ops.
pub fn ns_roster_ops(group: &GroupId) -> String {
    format!("roster_ops/{}", hex::encode(group.as_bytes()))
}

/// Namespace of a group's task ops.
pub fn ns_tasks(group: &GroupId) -> String {
    format!("tasks/{}", hex::encode(group.as_bytes()))
}

/// Namespace of a group's task bookkeeping.
pub fn ns_task_meta(group: &GroupId) -> String {
    format!("task_meta/{}", hex::encode(group.as_bytes()))
}

fn corrupt(what: &'static str) -> NodeError {
    NodeError::Codec(CodecError::Invalid(what))
}

fn ver(r: &mut Reader<'_>) -> Result<(), CodecError> {
    let v = r.u8()?;
    if v != BLOB_VERSION {
        return Err(CodecError::Invalid("node blob version"));
    }
    Ok(())
}

fn count(r: &mut Reader<'_>) -> Result<usize, CodecError> {
    let n = usize::try_from(r.u32()?).map_err(|_| CodecError::Invalid("count"))?;
    if n > MAX_ITEMS || n > r.remaining() {
        return Err(CodecError::Invalid("count"));
    }
    Ok(n)
}

fn write_count(w: &mut Writer, n: usize) -> Result<(), CodecError> {
    w.u32(u32::try_from(n).map_err(|_| CodecError::Invalid("count"))?);
    Ok(())
}

fn key32(r: &mut Reader<'_>) -> Result<IdentityKey, CodecError> {
    admit_public_key(&r.bytes()?).map_err(|_| CodecError::Invalid("identity key"))
}

fn epoch_to(w: &mut Writer, e: &RecogEpoch) {
    w.bytes(e.k_tag().as_bytes()).bytes(e.k_hdr().as_bytes());
}

fn epoch_from(r: &mut Reader<'_>) -> Result<RecogEpoch, CodecError> {
    let k_tag = Key32::from_bytes(r.bytes()?);
    let k_hdr = Key32::from_bytes(r.bytes()?);
    Ok(RecogEpoch::from_keys(k_tag, k_hdr))
}

// ---- identity -----------------------------------------------------------------

pub(crate) struct IdentityRec {
    pub seed: Zeroizing<[u8; 32]>,
    pub display_name: String,
    pub envelope: Zeroizing<[u8; 32]>,
}

pub(crate) fn save_identity(
    txn: &mut dyn KvTxn,
    seed: &[u8; 32],
    display_name: &str,
    envelope: &EnvelopeKeyPair,
) {
    let mut w = Writer::new();
    w.u8(BLOB_VERSION).bytes(seed);
    let _ = w.str16(display_name);
    let mut secret = envelope.to_bytes();
    w.bytes(&secret);
    zeroize::Zeroize::zeroize(&mut secret);
    let bytes = Zeroizing::new(w.into_inner());
    txn.put(ns::IDENTITY, K_IDENTITY, &bytes);
}

pub(crate) fn load_identity(txn: &dyn KvTxn) -> Result<Option<IdentityRec>, NodeError> {
    let Some(bytes) = txn.get(ns::IDENTITY, K_IDENTITY) else {
        return Ok(None);
    };
    let bytes = Zeroizing::new(bytes);
    let mut r = Reader::new(&bytes);
    ver(&mut r)?;
    let seed = Zeroizing::new(r.bytes::<32>()?);
    let display_name = r.str16()?.to_owned();
    let envelope = Zeroizing::new(r.bytes::<32>()?);
    r.finish()?;
    Ok(Some(IdentityRec {
        seed,
        display_name,
        envelope,
    }))
}

// ---- recognition table ----------------------------------------------------------

pub(crate) fn save_recog(txn: &mut dyn KvTxn, recog: &TagTable) -> Result<(), NodeError> {
    let mut w = Writer::new();
    w.u8(BLOB_VERSION);
    write_count(&mut w, recog.chain_count())?;
    for (key, chain) in recog.chains() {
        w.u64(key.0);
        let epochs: Vec<_> = chain.epochs().collect();
        w.u8(u8::try_from(epochs.len()).map_err(|_| CodecError::Invalid("epochs"))?);
        for (id, epoch) in epochs {
            let win = chain.window(id).unwrap_or_default();
            w.u32(id.0);
            epoch_to(&mut w, epoch);
            w.u64(win.base).u64(win.used);
        }
        match chain.sender_epoch() {
            Some((id, epoch)) => {
                w.u8(1).u32(id.0);
                epoch_to(&mut w, epoch);
                w.u64(chain.sender_next_ctr().unwrap_or(0));
            }
            None => {
                w.u8(0);
            }
        }
    }
    let bytes = Zeroizing::new(w.into_inner());
    txn.put(ns::CHAINS, K_RECOG, &bytes);
    Ok(())
}

pub(crate) fn load_recog(txn: &dyn KvTxn) -> Result<TagTable, NodeError> {
    let mut table = TagTable::new();
    let Some(bytes) = txn.get(ns::CHAINS, K_RECOG) else {
        return Ok(table);
    };
    let bytes = Zeroizing::new(bytes);
    let mut r = Reader::new(&bytes);
    ver(&mut r)?;
    let n = count(&mut r)?;
    for _ in 0..n {
        let key = ChainKey(r.u64()?);
        let n_epochs = r.u8()?;
        for _ in 0..n_epochs {
            let id = EpochId(r.u32()?);
            let epoch = epoch_from(&mut r)?;
            let base = r.u64()?;
            let used = r.u64()?;
            table.restore(key, id, epoch, WindowState { base, used });
        }
        if r.u8()? == 1 {
            let id = EpochId(r.u32()?);
            let epoch = epoch_from(&mut r)?;
            let next = r.u64()?;
            table.register_sender(key, id, epoch, next);
        }
    }
    r.finish()?;
    Ok(table)
}

// ---- pairwise manager -------------------------------------------------------------

fn state_byte(s: SessionState) -> u8 {
    match s {
        SessionState::NeedsTheirKey => 0,
        SessionState::Ready => 1,
        SessionState::Crossed => 2,
    }
}

fn state_from(b: u8) -> Result<SessionState, CodecError> {
    Ok(match b {
        0 => SessionState::NeedsTheirKey,
        1 => SessionState::Ready,
        2 => SessionState::Crossed,
        other => return Err(crate::codec::unknown_variant("session state", other)),
    })
}

pub(crate) fn save_pairwise(txn: &mut dyn KvTxn, pm: &PairwiseManager) -> Result<(), NodeError> {
    let records = pm.export();
    let mut w = Writer::new();
    w.u8(BLOB_VERSION);
    write_count(&mut w, records.len())?;
    for rec in &records {
        w.bytes(rec.peer.as_bytes()).u8(state_byte(rec.state));
        match &rec.send {
            Some((id, epoch, sent)) => {
                w.u8(1).u32(*id);
                epoch_to(&mut w, epoch);
                w.u32(*sent);
            }
            None => {
                w.u8(0);
            }
        }
        w.u8(u8::try_from(rec.recv_epochs.len()).map_err(|_| CodecError::Invalid("epochs"))?);
        for e in &rec.recv_epochs {
            w.u32(*e);
        }
        w.u32(rec.next_obj_seq)
            .bool(rec.reseed_requested)
            .bool(rec.sent_prekey)
            .bool(rec.got_prekey)
            .u32(rec.decrypt_failures);
    }
    let bytes = Zeroizing::new(w.into_inner());
    txn.put(ns::CHAINS, K_PAIRWISE, &bytes);
    Ok(())
}

pub(crate) fn load_pairwise(
    txn: &dyn KvTxn,
    me: PeerId,
    recog: &mut TagTable,
) -> Result<PairwiseManager, NodeError> {
    let mut pm = PairwiseManager::new(me);
    let Some(bytes) = txn.get(ns::CHAINS, K_PAIRWISE) else {
        return Ok(pm);
    };
    let bytes = Zeroizing::new(bytes);
    let mut r = Reader::new(&bytes);
    ver(&mut r)?;
    let n = count(&mut r)?;
    for _ in 0..n {
        let peer = key32(&mut r)?;
        let state = state_from(r.u8()?)?;
        let send = if r.u8()? == 1 {
            let id = r.u32()?;
            let epoch = epoch_from(&mut r)?;
            let sent = r.u32()?;
            Some((id, epoch, sent))
        } else {
            None
        };
        let n_recv = r.u8()?;
        let mut recv_epochs = Vec::with_capacity(usize::from(n_recv));
        for _ in 0..n_recv {
            recv_epochs.push(r.u32()?);
        }
        let next_obj_seq = r.u32()?;
        let reseed_requested = r.bool()?;
        let sent_prekey = r.bool()?;
        let got_prekey = r.bool()?;
        let decrypt_failures = r.u32()?;
        let next_ctr = recog
            .sender_next_ctr(pair_chain_key(&me, &peer))
            .unwrap_or(0);
        pm.restore_contact(
            recog,
            ContactRecord {
                peer,
                state,
                send,
                recv_epochs,
                next_obj_seq,
                reseed_requested,
                sent_prekey,
                got_prekey,
                decrypt_failures,
            },
            next_ctr,
        );
    }
    r.finish()?;
    Ok(pm)
}

// ---- bundles ----------------------------------------------------------------------

pub(crate) fn save_bundles(txn: &mut dyn KvTxn, bm: &BundleManager) {
    if let Ok(bytes) = bm.try_encode() {
        txn.put(ns::CHAINS, K_BUNDLES, &bytes);
    }
}

pub(crate) fn load_bundles(txn: &dyn KvTxn) -> Result<BundleManager, NodeError> {
    match txn.get(ns::CHAINS, K_BUNDLES) {
        Some(bytes) => Ok(decode::<BundleManager>(&bytes)?),
        None => Ok(BundleManager::new(0)),
    }
}

// ---- groups -----------------------------------------------------------------------

fn group_key(group: &GroupId) -> Vec<u8> {
    let mut k = K_GROUP_PREFIX.to_vec();
    k.extend_from_slice(group.as_bytes());
    k
}

pub(crate) fn save_group(
    txn: &mut dyn KvTxn,
    group: &GroupId,
    g: &GroupCtx,
    recog: &TagTable,
    me: &PeerId,
) -> Result<(), NodeError> {
    let mut w = Writer::new();
    w.u8(BLOB_VERSION);
    match (
        g.state.own_chain(),
        g.state.own_epoch(),
        g.state.own_next_obj_seq(),
    ) {
        (Some(chain), Some(epoch), Some(next_obj_seq)) => {
            w.u8(1).u32(chain.0);
            epoch_to(&mut w, epoch);
            let next_ctr = recog
                .sender_next_ctr(group_chain_key(group, me, chain))
                .unwrap_or(0);
            w.u64(next_ctr).u32(next_obj_seq);
        }
        _ => {
            w.u8(0);
        }
    }
    let members: Vec<_> = g.state.member_chains().collect();
    write_count(&mut w, members.len())?;
    for (peer, chains) in members {
        w.bytes(peer.as_bytes());
        write_count(&mut w, chains.len())?;
        for chain in chains.keys() {
            w.u32(chain.0);
        }
    }
    w.bool(g.state.rotation_pending())
        .u64(g.policy.last_rotation_mono())
        .bool(g.policy.is_pending());
    write_count(&mut w, g.holders.len())?;
    for h in &g.holders {
        w.bytes(h.as_bytes());
    }
    w.u32(g.own_seq.0).bytes(g.own_seq.1.as_bytes());
    let bytes = Zeroizing::new(w.into_inner());
    txn.put(ns::CHAINS, &group_key(group), &bytes);
    Ok(())
}

struct GroupBlob {
    own: Option<(ChainId, RecogEpoch, u64, u32)>,
    members: Vec<(PeerId, Vec<ChainId>)>,
    pending: bool,
    last_rotation: u64,
    policy_pending: bool,
    holders: BTreeSet<PeerId>,
    own_seq: (u32, Hash32),
}

fn decode_group(bytes: &[u8]) -> Result<GroupBlob, NodeError> {
    let mut r = Reader::new(bytes);
    ver(&mut r)?;
    let own = if r.u8()? == 1 {
        let chain = ChainId(r.u32()?);
        let epoch = epoch_from(&mut r)?;
        let next_ctr = r.u64()?;
        let next_obj_seq = r.u32()?;
        Some((chain, epoch, next_ctr, next_obj_seq))
    } else {
        None
    };
    let n = count(&mut r)?;
    let mut members = Vec::with_capacity(n.min(64));
    for _ in 0..n {
        let peer = key32(&mut r)?;
        let m = count(&mut r)?;
        let mut chains = Vec::with_capacity(m.min(8));
        for _ in 0..m {
            chains.push(ChainId(r.u32()?));
        }
        members.push((peer, chains));
    }
    let pending = r.bool()?;
    let last_rotation = r.u64()?;
    let policy_pending = r.bool()?;
    let n = count(&mut r)?;
    let mut holders = BTreeSet::new();
    for _ in 0..n {
        holders.insert(key32(&mut r)?);
    }
    let seq = r.u32()?;
    let prev = Hash32::from_bytes(r.bytes()?);
    r.finish()?;
    Ok(GroupBlob {
        own,
        members,
        pending,
        last_rotation,
        policy_pending,
        holders,
        own_seq: (seq, prev),
    })
}

/// Stores one roster op's bytes (append-only).
pub(crate) fn save_roster_op(txn: &mut dyn KvTxn, group: &GroupId, op: &RosterOp) {
    if let Ok(bytes) = op.try_encode() {
        txn.put(
            &ns_roster_ops(group),
            crate::objects::object_id(&bytes).as_bytes(),
            &bytes,
        );
    }
}

/// Stores one task op record (append-only).
pub(crate) fn save_task_op(
    txn: &mut dyn KvTxn,
    group: &GroupId,
    task_id: &TaskId,
    op_hash: &Hash32,
    author: &IdentityKey,
    plaintext: &[u8],
) {
    let mut key = task_id.to_vec();
    key.extend_from_slice(op_hash.as_bytes());
    let mut value = author.as_bytes().to_vec();
    value.extend_from_slice(plaintext);
    txn.put(&ns_tasks(group), &key, &value);
}

pub(crate) fn save_task_meta(
    txn: &mut dyn KvTxn,
    group: &GroupId,
    task_id: &TaskId,
    task: &TaskCtx,
) -> Result<(), NodeError> {
    let mut w = Writer::new();
    w.u8(BLOB_VERSION)
        .u64(task.taken_at.unwrap_or(u64::MAX))
        .bool(task.seen_beyond_reach);
    write_count(&mut w, task.own_take_cells.len())?;
    for c in &task.own_take_cells {
        w.bytes(c.as_bytes());
    }
    txn.put(&ns_task_meta(group), task_id, &w.into_inner());
    Ok(())
}

fn parked_key(group: &GroupId, peer: &PeerId, seq: u32) -> Vec<u8> {
    let mut k = K_PARKED_PREFIX.to_vec();
    k.extend_from_slice(group.as_bytes());
    k.extend_from_slice(peer.as_bytes());
    k.extend_from_slice(&seq.to_le_bytes());
    k
}

pub(crate) fn save_parked(
    txn: &mut dyn KvTxn,
    group: &GroupId,
    peer: &PeerId,
    seq: u32,
    object: &[u8],
) {
    txn.put(ns::CHAINS, &parked_key(group, peer, seq), object);
}

pub(crate) fn delete_parked(txn: &mut dyn KvTxn, group: &GroupId, peer: &PeerId, seq: u32) {
    txn.delete(ns::CHAINS, &parked_key(group, peer, seq));
}

fn skdm_key(group: &GroupId, peer: &PeerId) -> Vec<u8> {
    let mut k = K_SKDM_PREFIX.to_vec();
    k.extend_from_slice(group.as_bytes());
    k.extend_from_slice(peer.as_bytes());
    k
}

/// An SKDM entry plaintext waiting for its sender's roster card.
pub(crate) fn save_pending_skdm(txn: &mut dyn KvTxn, group: &GroupId, peer: &PeerId, plain: &[u8]) {
    txn.put(ns::CHAINS, &skdm_key(group, peer), plain);
}

pub(crate) fn delete_pending_skdm(txn: &mut dyn KvTxn, group: &GroupId, peer: &PeerId) {
    txn.delete(ns::CHAINS, &skdm_key(group, peer));
}

// ---- votes ------------------------------------------------------------------------

pub(crate) fn save_vote_open(
    txn: &mut dyn KvTxn,
    group: &GroupId,
    open: &VoteOpen,
    accepted: bool,
) {
    let Ok(bytes) = open.try_encode() else {
        return;
    };
    let mut value = group.as_bytes().to_vec();
    value.extend_from_slice(&bytes);
    let mut key = Vec::with_capacity(33);
    if accepted {
        key.push(b'o');
        key.extend_from_slice(open.vote_id().as_bytes());
    } else {
        key.push(b'x');
        key.extend_from_slice(crate::objects::object_id(&bytes).as_bytes());
    }
    txn.put(NS_VOTES, &key, &value);
}

pub(crate) fn save_ballot(txn: &mut dyn KvTxn, ballot: &Ballot) {
    let mut key = Vec::with_capacity(66);
    key.push(b'b');
    key.extend_from_slice(ballot.vote_id.as_bytes());
    key.extend_from_slice(ballot.key.as_bytes());
    key.push(ballot.choice);
    txn.put(NS_VOTES, &key, &ballot.encode());
}

// ---- outbox and plaintext ----------------------------------------------------------

fn state_to_u8(s: DeliveryState) -> u8 {
    match s {
        DeliveryState::Waiting => 0,
        DeliveryState::WaitingNetworkBusy => 1,
        DeliveryState::PassedAlong => 2,
        DeliveryState::Delivered => 3,
        DeliveryState::MayHaveExpired => 4,
        DeliveryState::NeedsKey => 5,
    }
}

fn state_from_u8(b: u8) -> Result<DeliveryState, CodecError> {
    Ok(match b {
        0 => DeliveryState::Waiting,
        1 => DeliveryState::WaitingNetworkBusy,
        2 => DeliveryState::PassedAlong,
        3 => DeliveryState::Delivered,
        4 => DeliveryState::MayHaveExpired,
        5 => DeliveryState::NeedsKey,
        other => return Err(crate::codec::unknown_variant("delivery state", other)),
    })
}

impl Encode for OutboxEntry {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(BLOB_VERSION).bytes(&self.conv).u32(self.obj_seq);
        match self.target {
            Target::Group(g) => {
                w.u8(0).bytes(g.as_bytes());
            }
            Target::Peer(p) => {
                w.u8(1).bytes(p.as_bytes());
            }
        }
        w.u8(self.kind)
            .u64(self.created_t)
            .u8(state_to_u8(self.state))
            .bool(self.passed);
        write_count(w, self.cells.len())?;
        for c in &self.cells {
            w.bytes(c.as_bytes());
        }
        Ok(())
    }
}

impl Decode for OutboxEntry {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        ver(r)?;
        let conv = r.bytes()?;
        let obj_seq = r.u32()?;
        let target = match r.u8()? {
            0 => Target::Group(GroupId(r.bytes()?)),
            1 => Target::Peer(key32(r)?),
            other => return Err(crate::codec::unknown_variant("target", other)),
        };
        let kind = r.u8()?;
        let created_t = r.u64()?;
        let state = state_from_u8(r.u8()?)?;
        let passed = r.bool()?;
        let n = count(r)?;
        let mut cells = Vec::with_capacity(n.min(256));
        for _ in 0..n {
            cells.push(Hash32::from_bytes(r.bytes()?));
        }
        Ok(OutboxEntry {
            conv,
            obj_seq,
            target,
            kind,
            created_t,
            state,
            cells,
            passed,
        })
    }
}

/// A stored decrypted object.
#[derive(Debug, Clone)]
pub struct PlaintextRec {
    pub conv: [u8; 16],
    pub author: IdentityKey,
    pub obj_seq: u32,
    pub kind: u8,
    pub group: Option<GroupId>,
    pub plaintext: Vec<u8>,
}

impl Encode for PlaintextRec {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(BLOB_VERSION)
            .bytes(&self.conv)
            .bytes(self.author.as_bytes())
            .u32(self.obj_seq)
            .u8(self.kind);
        match self.group {
            Some(g) => {
                w.u8(1).bytes(g.as_bytes());
            }
            None => {
                w.u8(0);
            }
        }
        w.raw(&self.plaintext);
        Ok(())
    }
}

impl Decode for PlaintextRec {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        ver(r)?;
        let conv = r.bytes()?;
        let author = key32(r)?;
        let obj_seq = r.u32()?;
        let kind = r.u8()?;
        let group = match r.u8()? {
            0 => None,
            1 => Some(GroupId(r.bytes()?)),
            other => return Err(crate::codec::unknown_variant("group flag", other)),
        };
        let plaintext = r.rest().to_vec();
        Ok(PlaintextRec {
            conv,
            author,
            obj_seq,
            kind,
            group,
            plaintext,
        })
    }
}

pub(crate) fn save_plaintext(txn: &mut dyn KvTxn, obj: &ObjId, rec: &PlaintextRec) {
    let bytes = Zeroizing::new(rec.encode());
    txn.put(ns::PLAINTEXT, obj.as_bytes(), &bytes);
}

pub(crate) fn load_plaintext(
    txn: &dyn KvTxn,
    obj: &ObjId,
) -> Result<Option<PlaintextRec>, NodeError> {
    match txn.get(ns::PLAINTEXT, obj.as_bytes()) {
        Some(bytes) => {
            let bytes = Zeroizing::new(bytes);
            Ok(Some(decode::<PlaintextRec>(&bytes)?))
        }
        None => Ok(None),
    }
}

/// Dedupe marks for the `dedupe` namespace.
pub(crate) fn save_dedupe(txn: &mut dyn KvTxn, id: &Hash32, expires: u64, rejected: bool) {
    let mut v = expires.to_le_bytes().to_vec();
    v.push(u8::from(rejected));
    txn.put(NS_DEDUPE, id.as_bytes(), &v);
}

/// Every persisted dedupe mark `(id, expires_mono, rejected)`.
pub(crate) fn load_dedupe(txn: &dyn KvTxn) -> Vec<(Hash32, u64, bool)> {
    txn.scan_prefix(NS_DEDUPE, &[])
        .into_iter()
        .filter_map(|(k, v)| {
            let id = Hash32::try_from(k.as_slice()).ok()?;
            let (exp, rej) = v.split_at_checked(8)?;
            let exp = u64::from_le_bytes(<[u8; 8]>::try_from(exp).ok()?);
            Some((id, exp, rej.first().is_some_and(|b| *b == 1)))
        })
        .collect()
}

fn save_received(
    txn: &mut dyn KvTxn,
    conv: &[u8; 16],
    seqs: &BTreeSet<u32>,
) -> Result<(), NodeError> {
    let ranges = ranges_of(seqs);
    let mut w = Writer::new();
    w.u8(BLOB_VERSION);
    write_count(&mut w, ranges.len())?;
    for (a, b) in ranges {
        w.u32(a).u32(b);
    }
    txn.put(NS_RECEIPTS, conv, &w.into_inner());
    Ok(())
}

/// Maximal runs `(from, to)` of consecutive counters.
pub(crate) fn ranges_of(seqs: &BTreeSet<u32>) -> Vec<(u32, u32)> {
    let mut out: Vec<(u32, u32)> = Vec::new();
    for &s in seqs {
        match out.last_mut() {
            Some((_, to)) if to.checked_add(1) == Some(s) => *to = s,
            _ => out.push((s, s)),
        }
    }
    out
}

// ---- flush ---------------------------------------------------------------------------

/// Writes every dirty blob of `proto` into `txn` and clears the flags.
pub(crate) fn flush(txn: &mut dyn KvTxn, proto: &mut Proto) -> Result<(), NodeError> {
    let dirty = std::mem::take(&mut proto.dirty);
    if dirty.recog {
        save_recog(txn, &proto.recog)?;
    }
    if dirty.pm
        && let Some(pm) = proto.pm.as_ref()
    {
        save_pairwise(txn, pm)?;
    }
    if dirty.bundles {
        save_bundles(txn, &proto.bundles);
    }
    if let Some(me) = proto.me() {
        for gid in &dirty.groups {
            if let Some(g) = proto.groups.get(gid) {
                save_group(txn, gid, g, &proto.recog, &me)?;
            }
        }
    }
    if dirty.outbox {
        for (obj, e) in &proto.outbox {
            let bytes = Zeroizing::new(e.try_encode()?);
            txn.put(ns::OUTBOX, obj.as_bytes(), &bytes);
        }
    }
    if dirty.received {
        for (conv, seqs) in &proto.received {
            save_received(txn, conv, seqs)?;
        }
    }
    if dirty.heard {
        for (peer, t) in &proto.last_heard {
            txn.put(NS_HEARD, peer.as_bytes(), &t.to_le_bytes());
        }
    }
    for (id, expires, rejected) in proto.pending_dedupe.drain(..) {
        save_dedupe(txn, &id, expires, rejected);
    }
    Ok(())
}

// ---- load ----------------------------------------------------------------------------

/// Rebuilds the whole protocol state from the database (module docs). The
/// same loader serves the restart and the rollback after a failed
/// transaction, so what it produces is exactly the last committed state.
pub(crate) fn load(
    db: &mut Db,
    _backend: &dyn Backend,
    cfg: &NodeConfig,
    t: u64,
) -> Result<Proto, NodeError> {
    let txn = db.begin()?;
    let proto = load_in(&txn, cfg, t)?;
    drop(txn);
    Ok(proto)
}

fn load_in(txn: &dyn KvTxn, cfg: &NodeConfig, t: u64) -> Result<Proto, NodeError> {
    let identity = match &cfg.role {
        NodeRole::Member { .. } => {
            let rec = load_identity(txn)?.ok_or_else(|| corrupt("identity missing"))?;
            let signer = IdentitySigner::from_seed(&rec.seed);
            let card = load_keycard(txn, &signer)?;
            Some(Identity {
                signer,
                card,
                display_name: rec.display_name,
                envelope: EnvelopeKeyPair::from_bytes(*rec.envelope),
            })
        }
        NodeRole::Station => None,
    };
    let me = identity.as_ref().map(|i| i.card.ed);
    let mut recog = load_recog(txn)?;
    let pm = match me {
        Some(me) => Some(load_pairwise(txn, me, &mut recog)?),
        None => None,
    };
    let bundles = load_bundles(txn)?;
    let mut chains = SenderChainIndex::new();
    let mut groups = BTreeMap::new();
    let mut pending_skdm: BTreeMap<GroupId, BTreeMap<PeerId, Zeroizing<Vec<u8>>>> = BTreeMap::new();
    if let Some(me) = me {
        for (key, bytes) in txn.scan_prefix(ns::CHAINS, K_GROUP_PREFIX) {
            let gid_bytes = key
                .get(K_GROUP_PREFIX.len()..)
                .ok_or_else(|| corrupt("group key"))?;
            let gid = GroupId(<[u8; 16]>::try_from(gid_bytes).map_err(|_| corrupt("group id"))?);
            let bytes = Zeroizing::new(bytes);
            let blob = decode_group(&bytes)?;
            let mut state = GroupState::new(gid, me);
            if let Some((chain, epoch, next_ctr, next_obj_seq)) = blob.own {
                let next_ctr = recog
                    .sender_next_ctr(group_chain_key(&gid, &me, chain))
                    .unwrap_or(next_ctr);
                state.restore_own_epoch(&mut recog, chain, epoch, next_ctr, next_obj_seq);
            }
            for (peer, chain_ids) in &blob.members {
                for c in chain_ids {
                    state.restore_member_chain(*peer, *c);
                }
            }
            if blob.pending {
                state.mark_rotation_pending();
            }
            let policy = RotationPolicy::restore(
                cfg.rotation_interval_ms,
                blob.last_rotation,
                blob.policy_pending,
            );
            let mut dag = RosterDag::new(gid);
            let mut pending_ops: Vec<RosterOp> = Vec::new();
            for (_, op_bytes) in txn.scan_prefix(&ns_roster_ops(&gid), &[]) {
                if let Ok(op) = RosterOp::from_bytes(&op_bytes) {
                    pending_ops.push(op);
                }
            }
            // Insert in hash order; ops whose parents come later are parked
            // and unparked by the DAG when the parents arrive.
            for op in &pending_ops {
                let _ = dag.insert(op.clone());
            }
            dag.drain_applied();
            for (hash, op) in dag.ops() {
                chains.observe(
                    op.header.author,
                    op.header.author_seq,
                    *hash,
                    op.header.prev_own,
                    &op.header.heads,
                );
            }
            let heads = dag.heads();
            let view = evaluate(&dag, &heads);
            let mut ctx = GroupCtx {
                state,
                dag,
                policy,
                view,
                holders: blob.holders,
                own_seq: blob.own_seq,
                tasks: BTreeMap::new(),
                parked: BTreeMap::new(),
            };
            load_tasks(txn, &gid, &mut ctx, &me, t)?;
            groups.insert(gid, ctx);
        }
        // Pending pairwise SKDMs.
        for (key, plain) in txn.scan_prefix(ns::CHAINS, K_SKDM_PREFIX) {
            let rest = key
                .get(K_SKDM_PREFIX.len()..)
                .ok_or_else(|| corrupt("skdm key"))?;
            if rest.len() != 16 + 32 {
                continue;
            }
            let gid = GroupId(<[u8; 16]>::try_from(&rest[..16]).map_err(|_| corrupt("skdm"))?);
            let Ok(peer) = admit_public_key(
                &<[u8; 32]>::try_from(&rest[16..48]).map_err(|_| corrupt("skdm"))?,
            ) else {
                continue;
            };
            pending_skdm
                .entry(gid)
                .or_default()
                .insert(peer, Zeroizing::new(plain));
        }
        // Parked objects.
        for (key, object) in txn.scan_prefix(ns::CHAINS, K_PARKED_PREFIX) {
            let rest = key
                .get(K_PARKED_PREFIX.len()..)
                .ok_or_else(|| corrupt("parked key"))?;
            if rest.len() != 16 + 32 + 4 {
                continue;
            }
            let gid = GroupId(<[u8; 16]>::try_from(&rest[..16]).map_err(|_| corrupt("parked"))?);
            let peer = admit_public_key(
                &<[u8; 32]>::try_from(&rest[16..48]).map_err(|_| corrupt("parked"))?,
            )
            .map_err(|_| corrupt("parked peer"))?;
            let seq = u32::from_le_bytes(
                <[u8; 4]>::try_from(&rest[48..52]).map_err(|_| corrupt("parked"))?,
            );
            if let Some(g) = groups.get_mut(&gid) {
                g.parked.insert((peer, seq), object);
            }
        }
    }
    // Votes.
    let mut votes: BTreeMap<VoteId, VoteCtx> = BTreeMap::new();
    let mut pending_ballots: BTreeMap<VoteId, BTreeMap<IdentityKey, Vec<Ballot>>> = BTreeMap::new();
    let mut ballots: Vec<Ballot> = Vec::new();
    for (key, value) in txn.scan_prefix(NS_VOTES, &[]) {
        match key.first() {
            Some(b'o') | Some(b'x') => {
                let Some((gid, bytes)) = value.split_at_checked(16) else {
                    continue;
                };
                let gid = GroupId(<[u8; 16]>::try_from(gid).map_err(|_| corrupt("vote group"))?);
                let Ok(open) = VoteOpen::from_bytes(bytes) else {
                    continue;
                };
                let accepted = key.first() == Some(&b'o');
                if let Ok(hash) = open.try_op_hash() {
                    chains.observe(
                        open.header.author,
                        open.header.author_seq,
                        hash,
                        open.header.prev_own,
                        &open.header.heads,
                    );
                }
                if accepted {
                    let vote_id = open.vote_id();
                    votes.insert(
                        vote_id,
                        VoteCtx {
                            group: gid,
                            open,
                            valid: None,
                            reject: None,
                            members_of_h: None,
                            ballots: BTreeMap::new(),
                            tally: None,
                        },
                    );
                }
            }
            Some(b'b') => {
                if let Ok(b) = Ballot::from_bytes(&value) {
                    ballots.push(b);
                }
            }
            _ => {}
        }
    }
    // Re-validate every opening against the loaded DAGs and chain index.
    for v in votes.values_mut() {
        if let Some(g) = groups.get(&v.group) {
            super::inbound::revalidate_vote(v, &g.dag, &chains, cfg.branch);
        }
    }
    for b in ballots {
        let slot = match votes.get_mut(&b.vote_id) {
            Some(v) => v.ballots.entry(b.key).or_default(),
            None => pending_ballots
                .entry(b.vote_id)
                .or_default()
                .entry(b.key)
                .or_default(),
        };
        if slot.len() < 2 && !slot.iter().any(|x| x.payload_key() == b.payload_key()) {
            slot.push(b);
        }
    }
    for v in votes.values_mut() {
        super::inbound::retally(v);
    }
    // Outbox.
    let mut outbox = BTreeMap::new();
    let mut own_cells = BTreeMap::new();
    for (key, value) in txn.scan_prefix(ns::OUTBOX, &[]) {
        let obj = Hash32::try_from(key.as_slice()).map_err(|_| corrupt("outbox key"))?;
        let value = Zeroizing::new(value);
        let entry: OutboxEntry = decode(&value)?;
        for c in &entry.cells {
            own_cells.insert(*c, obj);
        }
        outbox.insert(obj, entry);
    }
    // Received counters.
    let mut received = BTreeMap::new();
    for (key, value) in txn.scan_prefix(NS_RECEIPTS, &[]) {
        let conv = <[u8; 16]>::try_from(key.as_slice()).map_err(|_| corrupt("receipt key"))?;
        let mut r = Reader::new(&value);
        ver(&mut r)?;
        let n = count(&mut r)?;
        let mut set = BTreeSet::new();
        for _ in 0..n {
            let a = r.u32()?;
            let b = r.u32()?;
            if b < a || b.saturating_sub(a) > 1 << 16 {
                return Err(corrupt("receipt range"));
            }
            for s in a..=b {
                set.insert(s);
            }
        }
        r.finish()?;
        received.insert(conv, set);
    }
    // Last heard.
    let mut last_heard = BTreeMap::new();
    for (key, value) in txn.scan_prefix(NS_HEARD, &[]) {
        if let (Ok(k), Ok(v)) = (
            <[u8; 32]>::try_from(key.as_slice()),
            <[u8; 8]>::try_from(value.as_slice()),
        ) && let Ok(peer) = admit_public_key(&k)
        {
            last_heard.insert(peer, u64::from_le_bytes(v));
        }
    }
    // Contacts cache and the same-name index.
    let mut contacts = BTreeMap::new();
    let mut samename = SameNameIndex::new();
    if me.is_some() {
        let mut tmp = TxnView(txn);
        let store = ContactStore::new(&mut tmp);
        for c in store.list()? {
            samename.observe(c.key, &c.display_name);
            if !c.petname.is_empty() {
                samename.set_petname(c.key, Some(&c.petname));
            }
            contacts.insert(
                c.key,
                ContactLite {
                    display_name: c.display_name.clone(),
                    link_key: c.link_key,
                    envelope_pub: c.envelope_pub,
                    verified: matches!(c.badge, Badge::VerifiedInPerson { .. }),
                },
            );
        }
    }
    Ok(Proto {
        identity,
        recog,
        pm,
        groups,
        chains,
        votes,
        pending_ballots,
        outbox,
        own_cells,
        received,
        bundles,
        reasm: Reasm::new(),
        last_heard,
        contacts,
        samename,
        pending_skdm,
        pending_dedupe: Vec::new(),
        dirty: Dirty::default(),
    })
}

/// A read-only view of a transaction as a `KvTxn` (writes are refused:
/// the loader never writes).
struct TxnView<'a>(&'a dyn KvTxn);

impl KvTxn for TxnView<'_> {
    fn get(&self, ns: &str, key: &[u8]) -> Option<Vec<u8>> {
        self.0.get(ns, key)
    }
    fn put(&mut self, _ns: &str, _key: &[u8], _value: &[u8]) {}
    fn delete(&mut self, _ns: &str, _key: &[u8]) {}
    fn scan_prefix(&self, ns: &str, prefix: &[u8]) -> Vec<(Vec<u8>, Vec<u8>)> {
        self.0.scan_prefix(ns, prefix)
    }
}

/// The KeyCard, re-issued from the signer and the backend's stored X25519
/// public key (`PairwiseBackend::local_x25519`; the signature input is
/// deterministic, so every open yields the same card bytes).
fn load_keycard(txn: &dyn KvTxn, signer: &IdentitySigner) -> Result<KeyCard, NodeError> {
    let x25519 = crate::crypto::backend::default_backend()
        .local_x25519(txn)
        .ok_or_else(|| corrupt("backend identity missing"))?;
    Ok(KeyCard::issue(signer, x25519))
}

fn load_tasks(
    txn: &dyn KvTxn,
    gid: &GroupId,
    g: &mut GroupCtx,
    me: &PeerId,
    _t: u64,
) -> Result<(), NodeError> {
    for (key, value) in txn.scan_prefix(&ns_tasks(gid), &[]) {
        if key.len() != 48 || value.len() < 32 {
            continue;
        }
        let task_id: TaskId = <[u8; 16]>::try_from(&key[..16]).map_err(|_| corrupt("task key"))?;
        let op_hash = Hash32::try_from(&key[16..48]).map_err(|_| corrupt("task key"))?;
        let Ok(author) = admit_public_key(
            &<[u8; 32]>::try_from(&value[..32]).map_err(|_| corrupt("task author"))?,
        ) else {
            continue;
        };
        let plaintext = &value[32..];
        let Ok(post) = decode::<PostPlain>(plaintext) else {
            continue;
        };
        let Ok(op) = TaskOp::from_bytes(&post.body) else {
            continue;
        };
        g.tasks
            .entry(task_id)
            .or_default()
            .records
            .push((op_hash, author, op));
    }
    for (key, value) in txn.scan_prefix(&ns_task_meta(gid), &[]) {
        let Ok(task_id) = <[u8; 16]>::try_from(key.as_slice()) else {
            continue;
        };
        let mut r = Reader::new(&value);
        ver(&mut r)?;
        let taken = r.u64()?;
        let seen = r.bool()?;
        let n = count(&mut r)?;
        let mut cells = Vec::with_capacity(n.min(16));
        for _ in 0..n {
            cells.push(Hash32::from_bytes(r.bytes()?));
        }
        r.finish()?;
        let task = g.tasks.entry(task_id).or_default();
        task.taken_at = (taken != u64::MAX).then_some(taken);
        task.seen_beyond_reach = seen;
        task.own_take_cells = cells;
    }
    let name_of = |k: &IdentityKey| -> String {
        g.view
            .members
            .get(k)
            .map(|m| m.name.clone())
            .unwrap_or_else(|| crate::roster::labels::UNNAMED.to_owned())
    };
    for (task_id, task) in g.tasks.iter_mut() {
        super::inbound::refold_task(task_id, task, me, &name_of);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemKv;
    use proptest::prelude::*;

    #[test]
    fn recog_blob_round_trips_windows_and_senders() {
        let mut table = TagTable::new();
        let e = RecogEpoch::from_seed(&[1; 32]);
        table.register(ChainKey(5), EpochId(2), e.clone(), 7);
        table.register_sender(ChainKey(9), EpochId(0), e.clone(), 42);
        let hit = table.lookup(&e.tag_for(9)).unwrap();
        table.consume(hit).unwrap();
        let mut kv = MemKv::new();
        save_recog(&mut kv, &table).unwrap();
        let back = load_recog(&kv).unwrap();
        assert_eq!(
            back.window(ChainKey(5), EpochId(2)),
            table.window(ChainKey(5), EpochId(2))
        );
        assert_eq!(back.sender_next_ctr(ChainKey(9)), Some(42));
        assert_eq!(back.lookup(&e.tag_for(9)), None, "consumed stays consumed");
        assert!(back.lookup(&e.tag_for(10)).is_some());
    }

    #[test]
    fn outbox_and_plaintext_round_trip() {
        let e = OutboxEntry {
            conv: [3; 16],
            obj_seq: 4,
            target: Target::Group(GroupId([1; 16])),
            kind: 0x30,
            created_t: 99,
            state: DeliveryState::PassedAlong,
            cells: vec![Hash32::from_bytes([7; 32])],
            passed: true,
        };
        let bytes = e.encode();
        let back: OutboxEntry = decode(&bytes).unwrap();
        assert_eq!(back.cells, e.cells);
        assert_eq!(back.state, DeliveryState::PassedAlong);
        assert_eq!(
            ranges_of(&BTreeSet::from([1, 2, 3, 7, 9, 10])),
            vec![(1, 3), (7, 7), (9, 10)]
        );
    }

    proptest! {
        #[test]
        fn decoders_survive_random_bytes(bytes in prop::collection::vec(any::<u8>(), 0..300)) {
            let _ = decode::<OutboxEntry>(&bytes);
            let _ = decode::<PlaintextRec>(&bytes);
            let _ = decode_group(&bytes);
            let mut kv = MemKv::new();
            kv.put(ns::CHAINS, K_RECOG, &bytes);
            let _ = load_recog(&kv);
            kv.put(ns::IDENTITY, K_IDENTITY, &bytes);
            let _ = load_identity(&kv);
            let me = crate::crypto::ed25519::IdentitySigner::from_seed(&[1; 32]).public();
            let mut recog = TagTable::new();
            kv.put(ns::CHAINS, K_PAIRWISE, &bytes);
            let _ = load_pairwise(&kv, me, &mut recog);
            kv.put(ns::CHAINS, K_BUNDLES, &bytes);
            let _ = load_bundles(&kv);
        }

        #[test]
        fn the_loader_survives_random_rows(
            key in prop::collection::vec(any::<u8>(), 0..70),
            value in prop::collection::vec(any::<u8>(), 0..300),
        ) {
            // Every plain namespace the loader scans, with one random row
            // each: a corrupt row never panics (it is skipped or reported).
            let mut kv = MemKv::new();
            kv.put(NS_VOTES, &key, &value);
            kv.put(NS_RECEIPTS, &key, &value);
            kv.put(NS_HEARD, &key, &value);
            kv.put(NS_DEDUPE, &key, &value);
            kv.put(ns::OUTBOX, &key, &value);
            let gid = GroupId([7; 16]);
            kv.put(&ns_tasks(&gid), &key, &value);
            kv.put(&ns_task_meta(&gid), &key, &value);
            let _ = load_in(&kv, &NodeConfig::station(), 0);
            let _ = load_dedupe(&kv);
            let me = crate::crypto::ed25519::IdentitySigner::from_seed(&[1; 32]).public();
            let dag = RosterDag::new(gid);
            let view = evaluate(&dag, &[]);
            let mut g = GroupCtx {
                state: GroupState::new(gid, me),
                dag,
                policy: RotationPolicy::new(1, 0),
                view,
                holders: BTreeSet::new(),
                own_seq: (0, Hash32::ZERO),
                tasks: BTreeMap::new(),
                parked: BTreeMap::new(),
            };
            let _ = load_tasks(&kv, &gid, &mut g, &me, 0);
        }
    }
}
