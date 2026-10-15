//! The inbound pipeline (docs/DESIGN.md §1 row `node/inbound.rs`: stream →
//! frame → cell (format/size/TTL) → dedupe → store + relay admit →
//! recognise (tag table / seal trial) → reassemble → decrypt in a
//! transaction → dispatch; §6 "Atomic commit"; BRIEF_v2 §5.5 "Every
//! relay", §5.3; DECISIONS D3 K1 (atomic SKDM identity check), D4 lanes and
//! limits, §3 rows (1,2,3,4,10) and (3,4,6,10)).
//!
//! Order per cell ([`Node::on_cell`]):
//! 1. format, size and TTL (`Cell::validate`), the link's anti-entropy
//!    bookkeeping, the dedupe cache (an unexpired id is dropped);
//! 2. recognition: one tag-table lookup, else a trial open under the live
//!    envelope keys (≤ 3 AEADs, `wire::seal::ff_open_cell`); a recognised
//!    cell is lane L1, otherwise the link decides (L2 contact, L3 opaque);
//! 3. the inbound budget (`relay::limits`): `Throttle` goes back as a
//!    frame, `Reject` drops;
//! 4. an unrecognised cell is stored (`BufferPolicy::fits`, then
//!    `CellStore::put`, which evicts L3 first and never a pinned roster op)
//!    and offered to the other links: opaque relay traffic, no transaction;
//! 5. a recognised cell is handled inside one transaction: body open,
//!    counter consumed, reassembly, decrypt (ratchet advance + message-key
//!    deletion in the same transaction), dispatch by post kind, plaintext
//!    stored; the store/dedupe/relay effects and the events are deferred to
//!    after the commit ([`super::node::After`]), so a rolled-back cell is
//!    neither deduped nor stored and a re-fed copy yields exactly one
//!    delivery.
//!
//! Dispatch (DESIGN §2.4 post kinds): text → roster verdict
//! (`roster::rotation::post_verdict`) → stored and shown, labelled or
//! rejected everywhere; SKDM bundles → `GroupState::accept_bundle` with the
//! sender's **roster** KeyCard key (never the session's own claim); carried
//! roster ops → `RosterDag::insert`, re-evaluation, labels, rotation
//! trigger; task ops → the task's fold; vote-opens and ballots → the tally;
//! receipts → delivery states; recognition seeds → the pairwise manager.

use std::collections::BTreeSet;

use rand::RngCore;

use crate::codec::{Encode, decode, encode};
use crate::crypto::backend::{ChainId, GroupBackend, GroupId, PairwiseCt, PeerId};
use crate::crypto::ed25519::{IdentityKey, admit_public_key};
use crate::crypto::hash::Hash32;
use crate::dtn::store::{CellMeta, Put};
use crate::groups::group::{GroupError, GroupState};
use crate::objects::ballot::Ballot;
use crate::objects::chat_post::{PostPlain, Receipt, kind};
use crate::objects::roster_op::RosterOp;
use crate::objects::sender_chain::{ChainVerdict, SenderChainIndex};
use crate::objects::skdm_bundle::{SkdmBundle, SkdmPayload};
use crate::objects::task_op::{TaskId, TaskKind, TaskOp, task_id_for_create};
use crate::objects::vote_open::VoteOpen;
use crate::pairwise::session::{FirstFlight, PairwiseError, Received};
use crate::people::contacts::{ContactStore, PinSource};
use crate::relay::Lane;
use crate::relay::limits::{Admit, Penalty};
use crate::roster::dag::{Insert, RejectReason, RosterDag};
use crate::roster::eval::evaluate;
use crate::roster::labels::{labels, sent_by_removed};
use crate::roster::rotation::{PostVerdict, RotationPolicy, post_verdict};
use crate::tasks::state::{ReachEvidence, fold, ui_state};
use crate::votes::open::{Branch, ChainCheck, validate_open};
use crate::votes::tally::tally;
use crate::wire::cell::{CELL_LEN, Cell};
use crate::wire::frame::{Class, LinkFrame, Throttle};
use crate::wire::inner::InnerKind;
use crate::wire::recog::{Hit, RecogError};
use crate::wire::seal::{Sealed, ff_open_cell, try_unseal};

use super::links::LinkEnd;
use super::node::{
    After, Ctx, GroupCtx, Node, TaskCtx, VoteCtx, dm_conv, group_conv, task_op_hash,
};
use super::persist;
use super::status::DeliveryState;
use super::{Event, NodeError, object_id};

/// Pseudo chain keys of first-flight reassembly, one per live envelope key.
const FF_CHAIN_BASE: u64 = 0xFF00_0000_0000_0000;
/// Parked objects kept per sender while its SKDM is missing (bound).
pub const MAX_PARKED_PER_SENDER_ASM: usize = 256;
/// Object counter a sealed first flight carries in a 1:1 conversation
/// (outside the ratchet's counter space; receipts never cover it).
pub const FIRST_FLIGHT_OBJ_SEQ: u32 = u32::MAX;
/// Votes whose ballots arrived before their opening that are kept at once
/// (DESIGN §5 "holders re-send ballots after rejoin"; a bound so a member
/// cannot grow memory with ballots for votes that never open; assumption).
pub const MAX_PENDING_VOTES_ASM: usize = 1024;

/// What a recognised cell resolved to.
enum Recognised {
    Hit(Hit),
    FirstFlight(usize, Box<[u8; crate::wire::cell::BODY_PLAINTEXT_LEN]>),
}

impl Node {
    /// The inbound pipeline of one cell (module docs).
    pub(crate) fn on_cell(&mut self, end: &mut LinkEnd, class: Class, cell: Cell, now_mono: u64) {
        let t = self.t(now_mono);
        self.stats.cells_received = self.stats.cells_received.saturating_add(1);
        if cell.validate().is_err() {
            self.stats.malformed = self.stats.malformed.saturating_add(1);
            return;
        }
        let id = cell.msg_id();
        end.sync.on_cell_received(id);
        if self.relay.dedupe.expires_at(&id).is_some_and(|exp| exp > t) {
            self.stats.duplicates = self.stats.duplicates.saturating_add(1);
            return;
        }
        // Recognition before admission: a recognised cell is member-validated
        // traffic (lane L1) and is never ramp-limited (DECISIONS §3 row (4,8,9,10)).
        let recognised = match self.proto.recog.lookup(&cell.tag()) {
            Some(hit) => Some(Recognised::Hit(hit)),
            None => self.proto.identity.as_ref().and_then(|me| {
                ff_open_cell(std::slice::from_ref(&me.envelope), &cell)
                    .map(|(i, plain)| Recognised::FirstFlight(i, Box::new(plain)))
            }),
        };
        let lane = Node::lane_for(end, recognised.is_some());
        // In-person setup (`Node::set_provisioning`) admits everything.
        let verdict = if self.provisioning {
            Admit::Accept
        } else {
            end.inbound_budget.admit(class, lane, CELL_LEN, t)
        };
        match verdict {
            Admit::Accept => {}
            Admit::Throttle { retry_ms } => {
                self.stats.throttled_in = self.stats.throttled_in.saturating_add(1);
                let _ =
                    end.stage_frame(&LinkFrame::throttle(&Throttle::for_class(class, retry_ms)));
                return;
            }
            Admit::Reject => {
                self.stats.rejected_in = self.stats.rejected_in.saturating_add(1);
                return;
            }
        }
        let meta = CellMeta {
            class,
            lane,
            from_link: Some(end.id),
            first_seen_mono: t,
            pinned: false,
        };
        let Some(recognised) = recognised else {
            self.store_opaque(cell, meta, t);
            return;
        };
        let link = end.id;
        let result = self.run_txn(now_mono, |ctx| {
            handle_recognised(ctx, link, recognised, &cell, meta)
        });
        if result.is_err() {
            self.stats.decrypt_failures = self.stats.decrypt_failures.saturating_add(1);
        }
    }

    /// A held cell whose tag became recognisable (`After::Rescan`): the
    /// recognised path, with the cell's stored provenance.
    pub(crate) fn reprocess_stored(&mut self, cell: Cell, meta: CellMeta, now_mono: u64) {
        let Some(hit) = self.proto.recog.lookup(&cell.tag()) else {
            return;
        };
        let link = meta
            .from_link
            .unwrap_or(crate::transport::link::LinkId(u32::MAX));
        let _ = self.run_txn(now_mono, |ctx| {
            handle_recognised(ctx, link, Recognised::Hit(hit), &cell, meta)
        });
    }

    /// Step 4: an opaque cell is held for others (BRIEF_v2 §5.7 DTN).
    fn store_opaque(&mut self, cell: Cell, meta: CellMeta, t: u64) {
        let id = cell.msg_id();
        let total = self.relay.store.bytes();
        let link_bytes = meta
            .from_link
            .map_or(0, |l| self.relay.store.bytes_for_link(l));
        // Over the general cap the store's policy evicts L3 first and never
        // a pinned roster op or a higher class; own cells are pinned while
        // undelivered, which is how the absent-peer reserve is kept
        // (`relay::buffers`).
        let _fits = self.relay.buffer.fits(total, link_bytes, CELL_LEN);
        match self.relay.store.put(cell, meta) {
            Put::Stored => {
                self.relay.store_version = self.relay.store_version.wrapping_add(1);
                self.relay.dedupe.seen_or_insert(id, t);
                self.proto.pending_dedupe.push((
                    id,
                    t.saturating_add(self.relay.dedupe.retention_ms()),
                    false,
                ));
                self.forward_live(id, meta.class, meta.lane, meta.from_link);
            }
            Put::Duplicate => {
                self.relay.dedupe.seen_or_insert(id, t);
                self.stats.duplicates = self.stats.duplicates.saturating_add(1);
            }
            Put::NoRoom => self.stats.no_room = self.stats.no_room.saturating_add(1),
        }
    }

    /// A pairwise session with `peer` became usable: every group in which
    /// the peer is a member but does not hold this node's chain rotates
    /// before the next post, so the peer gets an SKDM entry (D3 K2).
    pub(crate) fn on_session_ready(&mut self, peer: PeerId, now_mono: u64) {
        let mut owed = Vec::new();
        for (gid, g) in self.proto.groups.iter() {
            if g.view.is_member(&peer)
                && !g.holders.contains(&peer)
                && g.state.own_chain().is_some()
            {
                owed.push(*gid);
            }
        }
        if owed.is_empty() {
            return;
        }
        let _ = self.run_txn(now_mono, |ctx| {
            for gid in &owed {
                if let Some(g) = ctx.proto.groups.get_mut(gid) {
                    g.state.mark_rotation_pending();
                    ctx.proto.dirty.groups.insert(*gid);
                }
            }
            Ok(())
        });
    }
}

/// Step 5: a recognised cell inside the transaction (module docs).
fn handle_recognised(
    ctx: &mut Ctx<'_, '_>,
    link: crate::transport::link::LinkId,
    recognised: Recognised,
    cell: &Cell,
    meta: CellMeta,
) -> Result<(), NodeError> {
    let id = cell.msg_id();
    match recognised {
        Recognised::Hit(hit) => {
            let group = ctx
                .proto
                .groups
                .iter()
                .find(|(_, g)| g.state.chain_owner(hit.chain).is_some())
                .map(|(gid, _)| *gid);
            if let Some(gid) = group {
                handle_group_cell(ctx, link, gid, hit, cell, meta)
            } else if ctx
                .proto
                .pm
                .as_ref()
                .and_then(|pm| pm.chain_peer(hit.chain))
                .is_some()
            {
                handle_pairwise_cell(ctx, hit, cell, meta)
            } else {
                // A chain the table knows but no manager claims: hold the
                // cell as opaque.
                ctx.after.push(After::Store {
                    cell: Box::new(cell.clone()),
                    meta,
                });
                Ok(())
            }
        }
        Recognised::FirstFlight(idx, plain) => {
            let chain = FF_CHAIN_BASE.wrapping_add(idx as u64);
            let Some((kind, _obj_seq, object)) = ctx.proto.reasm.push(chain, &plain)? else {
                ctx.after.push(After::Store {
                    cell: Box::new(cell.clone()),
                    meta,
                });
                return Ok(());
            };
            if kind != InnerKind::SealedFirst {
                return Ok(());
            }
            accept_first_flight(ctx, &object)?;
            ctx.after.push(After::Store {
                cell: Box::new(cell.clone()),
                meta,
            });
            let _ = id;
            Ok(())
        }
    }
}

/// Opens a recognised cell's body and consumes its counter.
fn open_recognised(
    ctx: &mut Ctx<'_, '_>,
    hit: Hit,
    cell: &Cell,
) -> Result<Option<[u8; crate::wire::cell::BODY_PLAINTEXT_LEN]>, NodeError> {
    let Some(epoch) = ctx.proto.recog.epoch(hit.chain, hit.epoch_id) else {
        return Ok(None);
    };
    let Ok(plain) = cell.open_body(epoch.k_hdr().as_bytes(), hit.ctr, 0) else {
        // Junk wearing a real tag: the counter is not burnt.
        ctx.stats.malformed = ctx.stats.malformed.saturating_add(1);
        return Ok(None);
    };
    match ctx.proto.recog.consume(hit) {
        Ok(()) | Err(RecogError::AlreadyUsed) => {}
        Err(e) => return Err(GroupError::Recog(e).into()),
    }
    ctx.proto.dirty.recog = true;
    Ok(Some(plain))
}

fn handle_group_cell(
    ctx: &mut Ctx<'_, '_>,
    link: crate::transport::link::LinkId,
    gid: GroupId,
    hit: Hit,
    cell: &Cell,
    meta: CellMeta,
) -> Result<(), NodeError> {
    let Some(plain) = open_recognised(ctx, hit, cell)? else {
        return Ok(());
    };
    let (sender, _chain_id) = ctx
        .proto
        .groups
        .get(&gid)
        .and_then(|g| g.state.chain_owner(hit.chain))
        .ok_or(NodeError::UnknownGroup)?;
    heard(ctx, sender);
    let Some((kind, obj_seq, object)) = ctx.proto.reasm.push(hit.chain.0, &plain)? else {
        ctx.after.push(After::Store {
            cell: Box::new(cell.clone()),
            meta,
        });
        return Ok(());
    };
    match kind {
        InnerKind::SenderKey => {
            let res = GroupBackend::decrypt(ctx.backend, &mut *ctx.txn, &gid, &sender, &object);
            match res {
                Ok((chain, bytes)) => {
                    let post: PostPlain = decode(&bytes)?;
                    ctx.stats.decrypted = ctx.stats.decrypted.saturating_add(1);
                    let outcome = dispatch_group_post(
                        ctx,
                        gid,
                        sender,
                        chain,
                        obj_seq,
                        &post,
                        cell.msg_id(),
                        Some(link),
                    )?;
                    if outcome.store {
                        ctx.after.push(After::Store {
                            cell: Box::new(cell.clone()),
                            meta,
                        });
                        if outcome.pin {
                            ctx.after.push(After::Pin(cell.msg_id()));
                        }
                    }
                }
                Err(crate::crypto::backend::BackendError::NoSenderKey) => {
                    park(ctx, gid, sender, obj_seq, object);
                    ctx.after.push(After::Store {
                        cell: Box::new(cell.clone()),
                        meta,
                    });
                }
                Err(_) => {
                    ctx.stats.decrypt_failures = ctx.stats.decrypt_failures.saturating_add(1);
                    ctx.after.push(After::Store {
                        cell: Box::new(cell.clone()),
                        meta,
                    });
                }
            }
        }
        InnerKind::SkdmBundle => {
            let bundle: SkdmBundle = decode(&object)?;
            accept_bundle(ctx, gid, sender, &bundle);
            ctx.after.push(After::Store {
                cell: Box::new(cell.clone()),
                meta,
            });
        }
        _ => {
            ctx.stats.malformed = ctx.stats.malformed.saturating_add(1);
        }
    }
    Ok(())
}

fn handle_pairwise_cell(
    ctx: &mut Ctx<'_, '_>,
    hit: Hit,
    cell: &Cell,
    meta: CellMeta,
) -> Result<(), NodeError> {
    let Some(plain) = open_recognised(ctx, hit, cell)? else {
        return Ok(());
    };
    let peer = ctx
        .proto
        .pm
        .as_ref()
        .and_then(|pm| pm.chain_peer(hit.chain))
        .ok_or(NodeError::NoIdentity)?;
    heard(ctx, peer);
    let Some((kind, obj_seq, object)) = ctx.proto.reasm.push(hit.chain.0, &plain)? else {
        ctx.after.push(After::Store {
            cell: Box::new(cell.clone()),
            meta,
        });
        return Ok(());
    };
    if kind != InnerKind::PairwiseDr {
        ctx.stats.malformed = ctx.stats.malformed.saturating_add(1);
        return Ok(());
    }
    let ct: PairwiseCt = decode(&object)?;
    let received = {
        let Ctx {
            txn,
            proto,
            backend,
            rng,
            ..
        } = ctx;
        let pm = proto.pm.as_mut().ok_or(NodeError::NoIdentity)?;
        let r = pm.decrypt_from(&mut **txn, *backend, &mut proto.recog, &peer, &ct, *rng);
        proto.dirty.pm = true;
        proto.dirty.recog = true;
        r
    };
    match received {
        Ok(Received::Post(post)) => {
            ctx.stats.decrypted = ctx.stats.decrypted.saturating_add(1);
            dispatch_dm_post(ctx, peer, obj_seq, &post)?;
            ctx.after.push(After::Store {
                cell: Box::new(cell.clone()),
                meta,
            });
        }
        Ok(Received::RecogSeed { .. }) => {
            ctx.after.push(After::Rescan);
            ctx.after.push(After::Store {
                cell: Box::new(cell.clone()),
                meta,
            });
        }
        Err(PairwiseError::NeedsTheirKey) => {
            ctx.events.push(Event::NeedsKey { peer });
            ctx.after.push(After::Store {
                cell: Box::new(cell.clone()),
                meta,
            });
        }
        Err(_) => {
            ctx.stats.decrypt_failures = ctx.stats.decrypt_failures.saturating_add(1);
            ctx.after.push(After::Store {
                cell: Box::new(cell.clone()),
                meta,
            });
        }
    }
    Ok(())
}

/// A reassembled sealed first flight (DESIGN §2.4; `wire::seal`): the
/// payload is `sender.ed [32] ‖ FirstFlight` (the sender names itself
/// inside the seal; the ratchet decrypt authenticates the claim, and the
/// backend refuses an unpinned sender).
fn accept_first_flight(ctx: &mut Ctx<'_, '_>, object: &[u8]) -> Result<(), NodeError> {
    let Some(sealed) = Sealed::from_object(object) else {
        ctx.stats.malformed = ctx.stats.malformed.saturating_add(1);
        return Ok(());
    };
    let me = ctx.identity()?;
    let Some((_, payload)) = try_unseal(
        std::slice::from_ref(&me.envelope),
        &sealed.e_pub,
        &sealed.ct,
    ) else {
        ctx.stats.decrypt_failures = ctx.stats.decrypt_failures.saturating_add(1);
        return Ok(());
    };
    let Some((sender, rest)) = payload.split_at_checked(32) else {
        ctx.stats.malformed = ctx.stats.malformed.saturating_add(1);
        return Ok(());
    };
    let Ok(sender_bytes) = <[u8; 32]>::try_from(sender) else {
        return Ok(());
    };
    let Ok(from) = admit_public_key(&sender_bytes) else {
        ctx.stats.malformed = ctx.stats.malformed.saturating_add(1);
        return Ok(());
    };
    if !ctx.proto.contacts.contains_key(&from) {
        // BRIEF_v2 §3 "Pinning": a stranger's first flight is refused; the
        // backend would refuse it too (TOFU off).
        ctx.stats.decrypt_failures = ctx.stats.decrypt_failures.saturating_add(1);
        return Ok(());
    }
    let flight: FirstFlight = decode(rest)?;
    let result = {
        let Ctx {
            txn,
            proto,
            backend,
            rng,
            ..
        } = ctx;
        let pm = proto.pm.as_mut().ok_or(NodeError::NoIdentity)?;
        let r =
            pm.accept_first_flight(&mut **txn, *backend, &mut proto.recog, &from, &flight, *rng);
        proto.dirty.pm = true;
        proto.dirty.recog = true;
        r
    };
    match result {
        Ok((received, envelope_pub)) => {
            heard(ctx, from);
            // Remember the sender's envelope key for the sealed reply.
            {
                let mut store = ContactStore::new(&mut *ctx.txn);
                if let Ok(Some(mut c)) = store.get(&from) {
                    c.envelope_pub = Some(envelope_pub);
                    let _ = store.upsert(&c);
                }
                if let Some(c) = ctx.proto.contacts.get_mut(&from) {
                    c.envelope_pub = Some(envelope_pub);
                }
            }
            ctx.after.push(After::SessionReady(from));
            ctx.after.push(After::Rescan);
            if let Received::Post(post) = received {
                ctx.stats.decrypted = ctx.stats.decrypted.saturating_add(1);
                dispatch_dm_post(ctx, from, FIRST_FLIGHT_OBJ_SEQ, &post)?;
            }
        }
        Err(PairwiseError::NeedsTheirKey) => ctx.events.push(Event::NeedsKey { peer: from }),
        Err(_) => {
            ctx.stats.decrypt_failures = ctx.stats.decrypt_failures.saturating_add(1);
        }
    }
    Ok(())
}

/// Powered-on time this node last recognised anything from `peer`
/// (`tasks::pool` evidence).
fn heard(ctx: &mut Ctx<'_, '_>, peer: PeerId) {
    ctx.proto.last_heard.insert(peer, ctx.t);
    ctx.proto.dirty.heard = true;
}

/// Parks an object whose sender chain is unknown ("Waiting · can't read
/// yet", DESIGN §3), bounded per sender.
fn park(ctx: &mut Ctx<'_, '_>, gid: GroupId, sender: PeerId, obj_seq: u32, object: Vec<u8>) {
    let Some(g) = ctx.proto.groups.get_mut(&gid) else {
        return;
    };
    let count = g.parked.keys().filter(|(p, _)| *p == sender).count();
    if count >= MAX_PARKED_PER_SENDER_ASM {
        return;
    }
    persist::save_parked(&mut *ctx.txn, &gid, &sender, obj_seq, &object);
    g.parked.insert((sender, obj_seq), object);
    ctx.stats.parked = ctx.stats.parked.saturating_add(1);
}

/// Retries parked objects of `sender` after its chain arrived.
fn retry_parked(ctx: &mut Ctx<'_, '_>, gid: GroupId, sender: PeerId) -> Result<(), NodeError> {
    let keys: Vec<(PeerId, u32)> = ctx
        .proto
        .groups
        .get(&gid)
        .map(|g| {
            g.parked
                .keys()
                .filter(|(p, _)| *p == sender)
                .copied()
                .collect()
        })
        .unwrap_or_default();
    for key in keys {
        let Some(object) = ctx
            .proto
            .groups
            .get(&gid)
            .and_then(|g| g.parked.get(&key).cloned())
        else {
            continue;
        };
        match GroupBackend::decrypt(ctx.backend, &mut *ctx.txn, &gid, &sender, &object) {
            Ok((chain, bytes)) => {
                if let Some(g) = ctx.proto.groups.get_mut(&gid) {
                    g.parked.remove(&key);
                }
                persist::delete_parked(&mut *ctx.txn, &gid, &sender, key.1);
                let post: PostPlain = decode(&bytes)?;
                dispatch_group_post(ctx, gid, sender, chain, key.1, &post, Hash32::ZERO, None)?;
            }
            Err(crate::crypto::backend::BackendError::NoSenderKey) => {}
            Err(_) => {
                if let Some(g) = ctx.proto.groups.get_mut(&gid) {
                    g.parked.remove(&key);
                }
                persist::delete_parked(&mut *ctx.txn, &gid, &sender, key.1);
            }
        }
    }
    Ok(())
}

/// The X25519 key of `key`'s roster KeyCard: from the current view, else
/// from any applied op of the DAG that carries the card (a removed member's
/// split-time posts must still decrypt so the verdict can be judged,
/// DESIGN §3 "Posts by removed members"). Never the session's own claim.
pub(crate) fn roster_x25519(g: &GroupCtx, key: &IdentityKey) -> Option<[u8; 32]> {
    if let Some(m) = g.view.members.get(key) {
        return Some(m.keycard.x25519);
    }
    for (_, op) in g.dag.ops() {
        match &op.body {
            crate::objects::roster_op::RosterBody::Genesis {
                admins, members, ..
            } => {
                if let Some(c) = admins.iter().find(|c| c.ed == *key) {
                    return Some(c.x25519);
                }
                if let Some(m) = members.iter().find(|m| m.key() == *key) {
                    return Some(m.keycard.x25519);
                }
            }
            crate::objects::roster_op::RosterBody::Add { subject, .. } if subject.key() == *key => {
                return Some(subject.keycard.x25519);
            }
            _ => {}
        }
    }
    None
}

/// The atomic SKDM acceptance (D3 K1): the expected X25519 key is the
/// sender's **roster** KeyCard key, never the session's own claim.
fn accept_bundle(ctx: &mut Ctx<'_, '_>, gid: GroupId, sender: PeerId, bundle: &SkdmBundle) {
    let expected = ctx
        .proto
        .groups
        .get(&gid)
        .and_then(|g| roster_x25519(g, &sender));
    let Some(expected) = expected else {
        ctx.stats.decrypt_failures = ctx.stats.decrypt_failures.saturating_add(1);
        return;
    };
    let res = {
        let Ctx {
            txn,
            proto,
            backend,
            rng,
            ..
        } = ctx;
        let Some(g) = proto.groups.get_mut(&gid) else {
            return;
        };
        let Some(pm) = proto.pm.as_mut() else {
            return;
        };
        let r = g.state.accept_bundle(
            &mut **txn,
            *backend,
            &mut proto.recog,
            pm,
            sender,
            expected,
            bundle,
            *rng,
        );
        proto.dirty.recog = true;
        proto.dirty.pm = true;
        proto.dirty.groups.insert(gid);
        r
    };
    match res {
        Ok(_) => {
            ctx.after.push(After::Rescan);
            let _ = retry_parked(ctx, gid, sender);
        }
        Err(GroupError::NotForMe) => {}
        Err(_) => {
            ctx.stats.decrypt_failures = ctx.stats.decrypt_failures.saturating_add(1);
        }
    }
}

/// An SKDM entry that arrived pairwise (a sender with no old chain, or a
/// newcomer): routed to its group by the payload; parked until the roster
/// names the sender when the group or the sender's card is unknown.
fn accept_pairwise_skdm(
    ctx: &mut Ctx<'_, '_>,
    from: PeerId,
    post: &PostPlain,
) -> Result<(), NodeError> {
    let payload: SkdmPayload = decode(&post.body)?;
    let gid = payload.group;
    // The entry plaintext holds the chain key and the epoch seed.
    let plain = zeroize::Zeroizing::new(post.encode());
    let expected = ctx
        .proto
        .groups
        .get(&gid)
        .and_then(|g| roster_x25519(g, &from));
    let Some(expected) = expected else {
        persist::save_pending_skdm(&mut *ctx.txn, &gid, &from, &plain);
        ctx.proto
            .pending_skdm
            .entry(gid)
            .or_default()
            .insert(from, plain);
        return Ok(());
    };
    let res = {
        let Ctx {
            txn,
            proto,
            backend,
            ..
        } = ctx;
        let g = proto.groups.get_mut(&gid).ok_or(NodeError::UnknownGroup)?;
        let r = g.state.accept_skdm(
            &mut **txn,
            *backend,
            &mut proto.recog,
            from,
            expected,
            &plain,
        );
        proto.dirty.recog = true;
        proto.dirty.groups.insert(gid);
        r
    };
    match res {
        Ok(_) => {
            ctx.after.push(After::Rescan);
            retry_parked(ctx, gid, from)
        }
        Err(GroupError::IdentityMismatch) => {
            // No session yet (the SKDM outran the bundle): keep it.
            persist::save_pending_skdm(&mut *ctx.txn, &gid, &from, &plain);
            ctx.proto
                .pending_skdm
                .entry(gid)
                .or_default()
                .insert(from, plain);
            Ok(())
        }
        Err(GroupError::NotForMe | GroupError::WrongGroup | GroupError::WrongKind) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

/// Retries pending pairwise SKDMs of a group after its roster changed.
pub(crate) fn retry_pending_skdm(ctx: &mut Ctx<'_, '_>, gid: GroupId) -> Result<(), NodeError> {
    let Some(pending) = ctx.proto.pending_skdm.get(&gid).cloned() else {
        return Ok(());
    };
    for (from, plain) in pending {
        let expected = ctx
            .proto
            .groups
            .get(&gid)
            .and_then(|g| roster_x25519(g, &from));
        let Some(expected) = expected else {
            continue;
        };
        let res = {
            let Ctx {
                txn,
                proto,
                backend,
                ..
            } = ctx;
            let Some(g) = proto.groups.get_mut(&gid) else {
                continue;
            };
            let r = g.state.accept_skdm(
                &mut **txn,
                *backend,
                &mut proto.recog,
                from,
                expected,
                &plain,
            );
            proto.dirty.recog = true;
            proto.dirty.groups.insert(gid);
            r
        };
        match res {
            Ok(_) => {
                if let Some(m) = ctx.proto.pending_skdm.get_mut(&gid) {
                    m.remove(&from);
                }
                persist::delete_pending_skdm(&mut *ctx.txn, &gid, &from);
                ctx.after.push(After::Rescan);
                retry_parked(ctx, gid, from)?;
            }
            Err(GroupError::IdentityMismatch) => {}
            Err(_) => {
                if let Some(m) = ctx.proto.pending_skdm.get_mut(&gid) {
                    m.remove(&from);
                }
                persist::delete_pending_skdm(&mut *ctx.txn, &gid, &from);
            }
        }
    }
    Ok(())
}

/// What a dispatched group post asks the relay side to do.
struct Outcome {
    store: bool,
    pin: bool,
}

/// Dispatches a decrypted 1:1 post (DESIGN §2.4 kinds).
fn dispatch_dm_post(
    ctx: &mut Ctx<'_, '_>,
    peer: PeerId,
    obj_seq: u32,
    post: &PostPlain,
) -> Result<(), NodeError> {
    let me = ctx.me()?;
    let conv = dm_conv(&me, &peer);
    if obj_seq != FIRST_FLIGHT_OBJ_SEQ {
        ctx.proto.received.entry(conv).or_default().insert(obj_seq);
        ctx.proto.dirty.received = true;
    }
    match post.kind {
        kind::POST_TEXT => {
            let body = post.text_body()?;
            let obj = object_id(&conv, &peer, obj_seq);
            persist::save_plaintext(
                &mut *ctx.txn,
                &obj,
                &persist::PlaintextRec {
                    conv,
                    author: peer,
                    obj_seq,
                    kind: post.kind,
                    group: None,
                    plaintext: post.encode(),
                },
            );
            ctx.events.push(Event::Dm {
                peer,
                obj,
                text: body.text,
            });
            ctx.after.push(After::Receipt { peer, conv });
            Ok(())
        }
        kind::POST_CARRIED_ROSTER_OP => handle_roster_op(ctx, &post.body, None).map(|_| ()),
        kind::POST_SKDM_BUNDLE => accept_pairwise_skdm(ctx, peer, post),
        kind::POST_RECEIPT => {
            let receipt = post.receipt_body()?;
            handle_receipt(ctx, peer, &receipt);
            Ok(())
        }
        kind::POST_CARRIED_VOTE_OPEN | kind::POST_CARRIED_BALLOT | kind::POST_TASK_OP => {
            // Group objects never travel 1:1 (their author binding is the
            // group chain).
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Dispatches a decrypted group post (module docs). `cell_id` is the
/// carrier of the completing fragment (for pinning), `link` its source.
#[allow(clippy::too_many_arguments)]
fn dispatch_group_post(
    ctx: &mut Ctx<'_, '_>,
    gid: GroupId,
    sender: PeerId,
    chain: ChainId,
    obj_seq: u32,
    post: &PostPlain,
    _cell_id: Hash32,
    link: Option<crate::transport::link::LinkId>,
) -> Result<Outcome, NodeError> {
    let conv = group_conv(&gid, chain);
    ctx.proto.received.entry(conv).or_default().insert(obj_seq);
    ctx.proto.dirty.received = true;
    let mut outcome = Outcome {
        store: true,
        pin: false,
    };
    // The removal verdict (DECISIONS §3 row (1,2,3,4,10) (ii)) covers every
    // post by a member: texts and task ops. Ballots, vote-opens and roster
    // ops are validated only by their own rules (row (i)) and are exempt.
    let verdict = match post.kind {
        kind::POST_TEXT | kind::POST_TASK_OP => {
            let g = ctx.proto.groups.get(&gid).ok_or(NodeError::UnknownGroup)?;
            Some(post_verdict(&g.dag, &sender, &post.roster_heads, &g.view))
        }
        _ => None,
    };
    if verdict == Some(PostVerdict::RejectRemovedInHeads) {
        // Rejected everywhere: not stored, remembered as rejected so peers
        // stop re-offering it (row (iv)), the source link penalised (row (iii)).
        ctx.stats.rejected_posts = ctx.stats.rejected_posts.saturating_add(1);
        if let Some(l) = link {
            ctx.after.push(After::Penalise(l, Penalty::RemovedInHeads));
        }
        ctx.after.push(After::Dedupe {
            id: _cell_id,
            rejected: true,
        });
        outcome.store = false;
        return Ok(outcome);
    }
    match post.kind {
        kind::POST_TEXT => {
            let name = member_name(ctx, &gid, &sender);
            let labels = if verdict == Some(PostVerdict::KeepLabelledRemoved) {
                vec![sent_by_removed(&name)]
            } else {
                Vec::new()
            };
            let body = post.text_body()?;
            let obj = object_id(&conv, &sender, obj_seq);
            persist::save_plaintext(
                &mut *ctx.txn,
                &obj,
                &persist::PlaintextRec {
                    conv,
                    author: sender,
                    obj_seq,
                    kind: post.kind,
                    group: Some(gid),
                    plaintext: post.encode(),
                },
            );
            ctx.events.push(Event::Post {
                group: gid,
                author: sender,
                author_name: name,
                obj,
                text: body.text,
                labels,
            });
            ctx.after.push(After::Receipt { peer: sender, conv });
        }
        kind::POST_CARRIED_ROSTER_OP => {
            let applied = handle_roster_op(ctx, &post.body, link)?;
            outcome.pin = applied;
        }
        kind::POST_TASK_OP => handle_task_op(ctx, gid, sender, post)?,
        kind::POST_CARRIED_VOTE_OPEN => handle_vote_open(ctx, gid, &post.body, link)?,
        kind::POST_CARRIED_BALLOT => handle_ballot(ctx, &post.body, link)?,
        _ => {}
    }
    Ok(outcome)
}

fn member_name(ctx: &Ctx<'_, '_>, gid: &GroupId, key: &IdentityKey) -> String {
    ctx.proto
        .groups
        .get(gid)
        .and_then(|g| g.view.members.get(key))
        .map(|m| m.name.clone())
        .filter(|n| !n.is_empty())
        .or_else(|| {
            ctx.proto
                .contacts
                .get(key)
                .map(|c| c.display_name.clone())
                .filter(|n| !n.is_empty())
        })
        .unwrap_or_else(|| crate::roster::labels::UNNAMED.to_owned())
}

/// A roster op from a carried post, a 1:1 share or this node's own command
/// (DESIGN §3; D3 M1–M3). Returns whether it was applied or parked (the
/// carrier cell is then pinned).
pub(crate) fn handle_roster_op(
    ctx: &mut Ctx<'_, '_>,
    bytes: &[u8],
    link: Option<crate::transport::link::LinkId>,
) -> Result<bool, NodeError> {
    let op = match RosterOp::from_bytes(bytes) {
        Ok(op) => op,
        Err(_) => {
            if let Some(l) = link {
                ctx.after.push(After::Penalise(l, Penalty::BadSignature));
            }
            return Ok(false);
        }
    };
    let gid = op.group_id();
    apply_roster_op(ctx, gid, op, link)
}

/// Inserts an already-verified op into its group's DAG and re-evaluates.
pub(crate) fn apply_roster_op(
    ctx: &mut Ctx<'_, '_>,
    gid: GroupId,
    op: RosterOp,
    link: Option<crate::transport::link::LinkId>,
) -> Result<bool, NodeError> {
    let me = ctx.me()?;
    ensure_group(ctx, gid);
    let (insert, applied) = {
        let g = ctx
            .proto
            .groups
            .get_mut(&gid)
            .ok_or(NodeError::UnknownGroup)?;
        let insert = g.dag.insert(op.clone());
        let applied = g.dag.drain_applied();
        (insert, applied)
    };
    match insert {
        Insert::Rejected(RejectReason::Invalid(_)) => {
            if let Some(l) = link {
                ctx.after.push(After::Penalise(l, Penalty::BadSignature));
            }
            return Ok(false);
        }
        Insert::Rejected(_) => return Ok(false),
        Insert::Parked { .. } => {
            // Kept for the parents (requested through the summary vectors,
            // DESIGN §3): persisted so a restart re-parks it.
            persist::save_roster_op(&mut *ctx.txn, &gid, &op);
            return Ok(true);
        }
        Insert::Applied(_) => {}
    }
    persist::save_roster_op(&mut *ctx.txn, &gid, &op);
    for h in &applied {
        let (author, seq, prev, heads) = {
            let g = ctx.proto.groups.get(&gid).ok_or(NodeError::UnknownGroup)?;
            let Some(o) = g.dag.get(h) else {
                continue;
            };
            (
                o.header.author,
                o.header.author_seq,
                o.header.prev_own,
                o.header.heads.clone(),
            )
        };
        ctx.proto.chains.observe(author, seq, *h, prev, &heads);
        if author != me {
            heard(ctx, author);
        }
    }
    reevaluate(ctx, gid, &applied)?;
    Ok(true)
}

/// Creates the group's context when the first op of a group arrives.
pub(crate) fn ensure_group(ctx: &mut Ctx<'_, '_>, gid: GroupId) {
    if ctx.proto.groups.contains_key(&gid) {
        return;
    }
    let Some(me) = ctx.proto.me() else {
        return;
    };
    ctx.proto.groups.insert(
        gid,
        GroupCtx {
            state: GroupState::new(gid, me),
            dag: RosterDag::new(gid),
            policy: RotationPolicy::new(ctx.cfg.rotation_interval_ms, ctx.t),
            view: evaluate(&RosterDag::new(gid), &[]),
            holders: BTreeSet::new(),
            own_seq: (0, Hash32::ZERO),
            tasks: Default::default(),
            parked: Default::default(),
        },
    );
    ctx.proto.dirty.groups.insert(gid);
}

/// Re-evaluates `members(H)` at the current heads, emits the UI labels,
/// feeds the rotation trigger, pins the members' session keys and records
/// them as contacts (BRIEF_v2 §3 "shares a group with it").
fn reevaluate(ctx: &mut Ctx<'_, '_>, gid: GroupId, applied: &[Hash32]) -> Result<(), NodeError> {
    let me = ctx.me()?;
    let (before, after, removal_seen) = {
        let g = ctx
            .proto
            .groups
            .get_mut(&gid)
            .ok_or(NodeError::UnknownGroup)?;
        let before = std::mem::take(&mut g.view);
        let heads = g.dag.heads();
        let after = evaluate(&g.dag, &heads);
        g.view = after.clone();
        let removal_seen = g.policy.observe_applied(&g.dag, applied, &after);
        if removal_seen && !ctx.adversary_keep_keys {
            g.state.mark_rotation_pending();
        }
        (before, after, removal_seen)
    };
    let _ = removal_seen;
    let changed = labels(&before, &after);
    // Pins and contacts for every member (D3 K1: the roster KeyCard is the
    // pinned key); newcomers without this node's chain trigger a rotation.
    let mut same_name = Vec::new();
    for (key, m) in &after.members {
        if *key == me {
            continue;
        }
        let x25519 = m.keycard.x25519;
        let _ = ctx.backend.pin_peer_x25519(&mut *ctx.txn, key, &x25519);
        let mut store = ContactStore::new(&mut *ctx.txn);
        if let Ok(c) = store.first_contact(key, x25519, &m.name, PinSource::SharedGroup(gid), 0) {
            let lite = ctx
                .proto
                .contacts
                .entry(*key)
                .or_insert(super::node::ContactLite {
                    display_name: m.name.clone(),
                    link_key: None,
                    envelope_pub: None,
                    verified: false,
                });
            lite.display_name = c.display_name.clone();
            lite.verified |= matches!(
                c.badge,
                crate::people::contacts::Badge::VerifiedInPerson { .. }
            );
            ctx.proto.samename.observe(*key, &m.name);
            if let Some(marker) = ctx.proto.samename.marker_for(key) {
                same_name.push(Event::SameName {
                    key: *key,
                    text: marker.text(&m.name),
                });
            }
        }
        if !before.members.contains_key(key) {
            let g = ctx
                .proto
                .groups
                .get_mut(&gid)
                .ok_or(NodeError::UnknownGroup)?;
            if !g.holders.contains(key) && g.state.own_chain().is_some() {
                g.state.mark_rotation_pending();
            }
        }
    }
    ctx.proto.dirty.groups.insert(gid);
    if !changed.is_empty() {
        ctx.events.push(Event::RosterChanged {
            group: gid,
            labels: changed,
        });
    }
    ctx.events.extend(same_name);
    retry_pending_skdm(ctx, gid)?;
    // Openings waiting for this roster's heads.
    revalidate_votes_of(ctx, gid);
    Ok(())
}

/// A task op inside a group post (DESIGN §4; BRIEF_v2 §4 Tasks).
pub(crate) fn handle_task_op(
    ctx: &mut Ctx<'_, '_>,
    gid: GroupId,
    author: PeerId,
    post: &PostPlain,
) -> Result<(), NodeError> {
    let me = ctx.me()?;
    let op = TaskOp::from_bytes(&post.body)?;
    let plaintext = post.encode();
    let hash = task_op_hash(&author, &plaintext);
    let task_id: TaskId = if op.op == TaskKind::Create {
        task_id_for_create(&author, &plaintext)
    } else {
        op.task_id
    };
    persist::save_task_op(&mut *ctx.txn, &gid, &task_id, &hash, &author, &plaintext);
    let name_of = |k: &IdentityKey| member_name(ctx, &gid, k);
    let names: std::collections::BTreeMap<IdentityKey, String> = ctx
        .proto
        .groups
        .get(&gid)
        .map(|g| g.view.members.keys().map(|k| (*k, name_of(k))).collect())
        .unwrap_or_default();
    let t = ctx.t;
    let g = ctx
        .proto
        .groups
        .get_mut(&gid)
        .ok_or(NodeError::UnknownGroup)?;
    let task = g.tasks.entry(task_id).or_default();
    if task
        .records
        .iter()
        .any(|(h, a, _)| *h == hash && *a == author)
    {
        return Ok(());
    }
    task.records.push((hash, author, op.clone()));
    if op.op == TaskKind::Take && task.taken_at.is_none() {
        task.taken_at = Some(t);
    }
    let before = task.ui.clone();
    let lookup = |k: &IdentityKey| -> String {
        names
            .get(k)
            .cloned()
            .unwrap_or_else(|| crate::roster::labels::UNNAMED.to_owned())
    };
    refold_task(&task_id, task, &me, &lookup);
    let ui = task.ui.clone();
    persist::save_task_meta(&mut *ctx.txn, &gid, &task_id, task)?;
    if ui != before {
        ctx.events.push(Event::TaskChanged {
            group: gid,
            task_id,
            ui,
        });
    }
    Ok(())
}

/// Folds a task's records and renders the brief's state string.
pub(crate) fn refold_task(
    task_id: &TaskId,
    task: &mut TaskCtx,
    me: &IdentityKey,
    name_of: &dyn Fn(&IdentityKey) -> String,
) {
    let folded = fold(task_id, &task.records);
    let reach = ReachEvidence {
        local_only: !task.seen_beyond_reach,
        seen_beyond_reach: task.seen_beyond_reach,
        conflict_seen: matches!(
            folded.state,
            crate::tasks::state::TaskState::Conflict { .. }
        ),
    };
    task.ui = ui_state(&folded.state, me, reach, name_of);
    task.state = Some(folded.state);
}

/// The hashes other task ops cite: records not cited by any other record.
pub(crate) fn task_heads(task: &TaskCtx) -> Vec<Hash32> {
    let cited: BTreeSet<Hash32> = task
        .records
        .iter()
        .flat_map(|(_, _, op)| op.task_heads.iter().copied())
        .collect();
    let mut heads: Vec<Hash32> = task
        .records
        .iter()
        .map(|(h, _, _)| *h)
        .filter(|h| !cited.contains(h))
        .collect();
    heads.sort_unstable();
    heads.dedup();
    heads
}

/// A carried vote opening (DESIGN §5; `votes::open`).
pub(crate) fn handle_vote_open(
    ctx: &mut Ctx<'_, '_>,
    gid: GroupId,
    bytes: &[u8],
    link: Option<crate::transport::link::LinkId>,
) -> Result<(), NodeError> {
    let open = match VoteOpen::from_bytes(bytes) {
        Ok(o) => o,
        Err(_) => {
            if let Some(l) = link {
                ctx.after.push(After::Penalise(l, Penalty::BadSignature));
            }
            return Ok(());
        }
    };
    let vote_id = open.try_vote_id()?;
    if ctx.proto.votes.contains_key(&vote_id) {
        return Ok(());
    }
    let hash = open.try_op_hash()?;
    ctx.proto.chains.observe(
        open.header.author,
        open.header.author_seq,
        hash,
        open.header.prev_own,
        &open.header.heads,
    );
    if open.header.author != ctx.me()? {
        heard(ctx, open.header.author);
    }
    persist::save_vote_open(&mut *ctx.txn, &gid, &open, true);
    let mut v = VoteCtx {
        group: gid,
        open,
        valid: None,
        reject: None,
        members_of_h: None,
        ballots: ctx
            .proto
            .pending_ballots
            .remove(&vote_id)
            .unwrap_or_default(),
        tally: None,
    };
    if let Some(g) = ctx.proto.groups.get(&gid) {
        revalidate_vote(&mut v, &g.dag, &ctx.proto.chains, ctx.cfg.branch);
    }
    retally(&mut v);
    let line = super::node::vote_line(&v);
    let state = v.tally.as_ref().map(|t| t.state.clone());
    ctx.proto.votes.insert(vote_id, v);
    ctx.events.push(Event::VoteChanged {
        vote_id,
        line,
        state,
    });
    Ok(())
}

/// Re-runs `validate_open` for a stored opening against the group's DAG
/// and the chain index (fork rule: an earlier object by the opener cites a
/// head outside `closure(H)`).
pub(crate) fn revalidate_vote(
    v: &mut VoteCtx,
    dag: &RosterDag,
    chains: &SenderChainIndex,
    branch: Branch,
) {
    let heads = v.open.h();
    if !dag.resolves(heads) {
        v.valid = None;
        v.members_of_h = None;
        return;
    }
    let view = evaluate(dag, heads);
    let members: BTreeSet<IdentityKey> = view.members.keys().copied().collect();
    let closure = dag.closure(heads);
    let fork = chains
        .heads_cited_before(v.open.opener(), v.open.header.author_seq)
        .iter()
        .any(|h| !closure.contains(h));
    let chain = if chains.is_equivocator(v.open.opener())
        && chains
            .equivocations(v.open.opener())
            .iter()
            .any(|(s, _, _)| *s == v.open.header.author_seq)
    {
        ChainCheck::Equivocation
    } else {
        ChainCheck::from(ChainVerdict::Ok)
    };
    match validate_open(&v.open, &members, branch, fork, chain) {
        Ok(valid) => {
            v.valid = Some(valid);
            v.reject = None;
        }
        Err(e) => {
            v.valid = None;
            v.reject = Some(e);
        }
    }
    v.members_of_h = Some(members);
}

/// Recounts a vote from its retained ballots.
pub(crate) fn retally(v: &mut VoteCtx) {
    let (Some(open), Some(members)) = (v.valid.as_ref(), v.members_of_h.as_ref()) else {
        v.tally = None;
        return;
    };
    let ballots: Vec<Ballot> = v.ballots.values().flatten().cloned().collect();
    v.tally = Some(tally(open, members, &ballots));
}

/// Re-validates the openings of a group whose roster changed.
fn revalidate_votes_of(ctx: &mut Ctx<'_, '_>, gid: GroupId) {
    let ids: Vec<Hash32> = ctx
        .proto
        .votes
        .iter()
        .filter(|(_, v)| v.group == gid)
        .map(|(id, _)| *id)
        .collect();
    for id in ids {
        let Some(mut v) = ctx.proto.votes.remove(&id) else {
            continue;
        };
        let before = super::node::vote_line(&v);
        if let Some(g) = ctx.proto.groups.get(&gid) {
            revalidate_vote(&mut v, &g.dag, &ctx.proto.chains, ctx.cfg.branch);
        }
        retally(&mut v);
        let line = super::node::vote_line(&v);
        let state = v.tally.as_ref().map(|t| t.state.clone());
        ctx.proto.votes.insert(id, v);
        if line != before {
            ctx.events.push(Event::VoteChanged {
                vote_id: id,
                line,
                state,
            });
        }
    }
}

/// A carried ballot (DESIGN §5 "Ballot": ≤ 2 payloads kept per key).
pub(crate) fn handle_ballot(
    ctx: &mut Ctx<'_, '_>,
    bytes: &[u8],
    link: Option<crate::transport::link::LinkId>,
) -> Result<(), NodeError> {
    let ballot = match Ballot::from_bytes(bytes) {
        Ok(b) => b,
        Err(_) => {
            if let Some(l) = link {
                ctx.after.push(After::Penalise(l, Penalty::BadSignature));
            }
            return Ok(());
        }
    };
    let vote_id = ballot.vote_id;
    let slot = match ctx.proto.votes.get_mut(&vote_id) {
        Some(v) => v.ballots.entry(ballot.key).or_default(),
        None => {
            let pending = &mut ctx.proto.pending_ballots;
            if !pending.contains_key(&vote_id) && pending.len() >= MAX_PENDING_VOTES_ASM {
                return Ok(());
            }
            pending
                .entry(vote_id)
                .or_default()
                .entry(ballot.key)
                .or_default()
        }
    };
    if slot.iter().any(|b| b.payload_key() == ballot.payload_key()) || slot.len() >= 2 {
        return Ok(());
    }
    slot.push(ballot);
    persist::save_ballot(&mut *ctx.txn, &ballot);
    if let Some(v) = ctx.proto.votes.get_mut(&vote_id) {
        let before = super::node::vote_line(v);
        retally(v);
        let line = super::node::vote_line(v);
        let state = v.tally.as_ref().map(|t| t.state.clone());
        if line != before {
            ctx.events.push(Event::VoteChanged {
                vote_id,
                line,
                state,
            });
        }
    }
    Ok(())
}

/// A cumulative receipt from `peer` (DESIGN §2.4): everything up to
/// `up_to_obj_seq` in `conv` except the gaps is Delivered (BRIEF_v2 §5.9).
fn handle_receipt(ctx: &mut Ctx<'_, '_>, peer: PeerId, receipt: &Receipt) {
    let mut unpin = Vec::new();
    for (obj, e) in ctx.proto.outbox.iter_mut() {
        // A "Needs their key" object has no cells and never left this
        // phone: no receipt can cover it (BRIEF_v2 §5.9 "Delivered means
        // ... on the recipient's phone").
        if e.conv != receipt.conv
            || e.state == DeliveryState::Delivered
            || e.state == DeliveryState::NeedsKey
        {
            continue;
        }
        let addressed = match e.target {
            super::node::Target::Peer(p) => p == peer,
            super::node::Target::Group(_) => true,
        };
        if !addressed {
            continue;
        }
        let acked = e.obj_seq == FIRST_FLIGHT_OBJ_SEQ
            || (e.obj_seq <= receipt.up_to_obj_seq
                && !receipt
                    .gaps
                    .iter()
                    .any(|g| g.from <= e.obj_seq && e.obj_seq <= g.to));
        if acked {
            e.state = DeliveryState::Delivered;
            ctx.events.push(Event::Delivery {
                obj: *obj,
                state: DeliveryState::Delivered,
            });
            unpin.extend(e.cells.iter().copied());
            ctx.proto.dirty.outbox = true;
        }
    }
    for c in unpin {
        ctx.after.push(After::Unpin(c));
    }
}

/// Lane of a cell this node originated: member-validated.
pub(crate) const OWN_LANE: Lane = Lane::L1;

/// Draws the object counter of a sealed first flight's cells.
pub(crate) fn ff_obj_seq(rng: &mut dyn RngCore) -> u32 {
    rng.next_u32()
}

/// The encoding of a first flight's sealed payload: `sender.ed ‖ FirstFlight`.
pub(crate) fn ff_payload(sender: &IdentityKey, flight: &FirstFlight) -> Vec<u8> {
    let mut out = sender.as_bytes().to_vec();
    out.extend_from_slice(&encode(flight));
    out
}
