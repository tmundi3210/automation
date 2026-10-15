//! Commands: objects this node originates (docs/DESIGN.md §1 row
//! `node/outbound.rs`; §3 "Sender Key rotation"; §4 "Ops"; §5 "VoteOpen",
//! "Ballot"; BRIEF_v2 §4, §5.3, §5.4 authority table, §5.9; DECISIONS D3
//! K2, M1–M2, D6 rules 2–3, §3 row (4,5,6,7)).
//!
//! Every command runs in one transaction ([`super::Node::run_txn`]):
//! the object is built with the group's current roster heads
//! (`RosterDag::heads`, D3 M4), encrypted (`GroupState::encrypt_post`,
//! which refuses while a rotation is pending, so the rotation runs first:
//! `new_sender_chain` → SKDM bundle under the **old** chain as one C1
//! object, pairwise first flights to members without a chain), cut into
//! cells, and after the commit the cells are held as this node's own
//! (pinned until delivered, lane L1), deduped and offered to every pushing
//! link; an outbox entry starts at `Waiting`. 1:1 objects go through
//! `PairwiseManager::send_post`, or a sealed first flight when no send
//! epoch exists yet; `NeedsTheirKey` becomes the `NeedsKey` state and event
//! (D6 rule 7).

use std::collections::BTreeSet;

use rand::RngCore;

use crate::clock::Clock;
use crate::codec::Encode;
use crate::crypto::backend::{GroupId, PeerId};
use crate::crypto::ed25519::IdentityKey;
use crate::crypto::hash::Hash32;
use crate::crypto::keycard::KeyCard;
use crate::dtn::store::CellMeta;
use crate::objects::ballot::Ballot;
use crate::objects::chat_post::{Gap, PostPlain, Receipt, TextBody, kind};
use crate::objects::roster_op::{Endorsement, MemberEntry, RosterBody, RosterOp};
use crate::objects::task_op::{TaskId, TaskKind, TaskOp, task_id_for_create};
use crate::objects::vote_open::{VoteOpen, threshold};
use crate::objects::{SignedHeader, kind as okind};
use crate::pairwise::bundles::IssueContext;
use crate::pairwise::pairing::{PairingSession, PairingStep, QrCode, QrPayload};
use crate::pairwise::session::{FirstFlight, PairwiseError};
use crate::people::contacts::{ContactStore, PinSource};
use crate::transport::link::{LinkAuth, LinkId};
use crate::wire::cell::{Cell, VERSION, draw_initial_ttl};
use crate::wire::frame::Class;
use crate::wire::inner::{InnerKind, fragment};
use crate::wire::seal::{ff_seal_cell, seal};

use super::inbound::{FIRST_FLIGHT_OBJ_SEQ, OWN_LANE, ff_obj_seq, ff_payload, task_heads};
use super::node::{After, ContactLite, Ctx, Node, OutboxEntry, Target, dm_conv, group_conv};
use super::persist;
use super::status::DeliveryState;
use super::{AddDraft, Event, ImportBadge, ImportedContact, NodeError, ObjId, VoteId, object_id};

/// One-time prekeys handed over at a QR scan or a direct encounter (D6 rule
/// 2 "hand over 10 reserve OPKs").
pub const OPK_RESERVE_ASM: u8 = 10;

/// Holds the cells of one originated object for the post-commit store.
struct Emitted {
    cells: Vec<Cell>,
    class: Class,
    pinned: bool,
}

fn emit(ctx: &mut Ctx<'_, '_>, e: Emitted) -> Vec<Hash32> {
    let mut ids = Vec::with_capacity(e.cells.len());
    for cell in e.cells {
        ids.push(cell.msg_id());
        let stamp = ctx.t.saturating_add(ctx.seq);
        ctx.seq = ctx.seq.saturating_add(1);
        ctx.after.push(After::Store {
            cell: Box::new(cell),
            meta: CellMeta {
                class: e.class,
                lane: OWN_LANE,
                from_link: None,
                first_seen_mono: stamp,
                pinned: e.pinned,
            },
        });
    }
    ids
}

/// Records a user object in the outbox at `Waiting` (BRIEF_v2 §5.9).
fn track(
    ctx: &mut Ctx<'_, '_>,
    conv: [u8; 16],
    author: PeerId,
    obj_seq: u32,
    target: Target,
    kind: u8,
    cells: Vec<Hash32>,
) -> ObjId {
    let obj = object_id(&conv, &author, obj_seq);
    for c in &cells {
        ctx.proto.own_cells.insert(*c, obj);
    }
    ctx.proto.outbox.insert(
        obj,
        OutboxEntry {
            conv,
            obj_seq,
            target,
            kind,
            created_t: ctx.t,
            state: DeliveryState::Waiting,
            cells,
            passed: false,
        },
    );
    ctx.proto.dirty.outbox = true;
    ctx.events.push(Event::Delivery {
        obj,
        state: DeliveryState::Waiting,
    });
    obj
}

// ---- pairwise sending -------------------------------------------------------------

/// Sends `post` to `peer` on the 1:1 chain, or as a sealed first flight
/// when this node has no send epoch towards the peer yet (DESIGN §2.4
/// "Sealed first flight"; D6 pick B (2)). Returns the cells and the
/// object counter the receiver will see.
pub(crate) fn send_pairwise(
    ctx: &mut Ctx<'_, '_>,
    peer: PeerId,
    post: &PostPlain,
) -> Result<(Vec<Cell>, u32), NodeError> {
    let me = ctx.me()?;
    let has_epoch = ctx
        .proto
        .pm
        .as_ref()
        .is_some_and(|pm| pm.send_epoch(&peer).is_some());
    if has_epoch {
        let next_seq = ctx
            .proto
            .pm
            .as_ref()
            .and_then(|pm| pm.export().into_iter().find(|r| r.peer == peer))
            .map_or(0, |r| r.next_obj_seq);
        let cells = {
            let Ctx {
                txn,
                proto,
                backend,
                rng,
                t,
                ..
            } = ctx;
            let pm = proto.pm.as_mut().ok_or(NodeError::NoIdentity)?;
            let r = pm.send_post(
                &mut **txn,
                *backend,
                &mut proto.recog,
                &peer,
                post,
                *t,
                *rng,
            );
            proto.dirty.pm = true;
            proto.dirty.recog = true;
            r
        };
        return match cells {
            Ok(cells) => {
                // The post's own cells are the last object cut (a re-seed
                // control post may precede it); its counter is the one the
                // manager held before the send, plus one per control object.
                let seq = obj_seq_of_last(ctx, &me, &peer, &cells).unwrap_or(next_seq);
                Ok((cells, seq))
            }
            Err(PairwiseError::NeedsTheirKey) => Err(NodeError::NeedsKey(peer)),
            Err(e) => Err(e.into()),
        };
    }
    // First flight: needs a session from the peer's bundle and its envelope key.
    let envelope_pub = ctx
        .proto
        .contacts
        .get(&peer)
        .and_then(|c| c.envelope_pub)
        .ok_or(NodeError::NeedsKey(peer))?;
    if !ctx.backend.has_session(&*ctx.txn, &peer) {
        return Err(NodeError::NeedsKey(peer));
    }
    let my_envelope = ctx.identity()?.envelope.public();
    let flight = {
        let Ctx {
            txn,
            proto,
            backend,
            rng,
            t,
            ..
        } = ctx;
        let pm = proto.pm.as_mut().ok_or(NodeError::NoIdentity)?;
        let r = pm.build_first_flight(
            &mut **txn,
            *backend,
            &mut proto.recog,
            &peer,
            post,
            my_envelope,
            *t,
            *rng,
        );
        proto.dirty.pm = true;
        proto.dirty.recog = true;
        match r {
            Ok(f) => f,
            Err(PairwiseError::NeedsTheirKey) => return Err(NodeError::NeedsKey(peer)),
            Err(e) => return Err(e.into()),
        }
    };
    let cells = seal_flight(ctx, &me, &envelope_pub, &flight)?;
    ctx.after.push(After::SessionReady(peer));
    Ok((cells, FIRST_FLIGHT_OBJ_SEQ))
}

/// Seals a first flight to `envelope_pub` and cuts it into random-tag cells.
fn seal_flight(
    ctx: &mut Ctx<'_, '_>,
    me: &PeerId,
    envelope_pub: &[u8; 32],
    flight: &FirstFlight,
) -> Result<Vec<Cell>, NodeError> {
    let payload = zeroize::Zeroizing::new(ff_payload(me, flight));
    let sealed =
        seal(envelope_pub, &payload, ctx.rng).map_err(|_| NodeError::Invalid("envelope key"))?;
    let object = sealed.to_object();
    let obj_seq = ff_obj_seq(ctx.rng);
    let frags = fragment(InnerKind::SealedFirst, obj_seq, &object)?;
    Ok(frags
        .iter()
        .map(|f| ff_seal_cell(VERSION, draw_initial_ttl(ctx.rng), envelope_pub, f, ctx.rng))
        .collect())
}

/// The object counter of the last object cut on this node's send chain to
/// `peer`: read back from its cells under the sender epoch (the manager
/// does not expose the counter it used). `cells` are the cells of the send
/// in order, so the last object's first fragment is found by walking back
/// from the end until a body opens as fragment 0 (a body nonce carries the
/// fragment index, so other fragments do not open under index 0); at most
/// one AEAD per cell.
fn obj_seq_of_last(ctx: &Ctx<'_, '_>, me: &PeerId, peer: &PeerId, cells: &[Cell]) -> Option<u32> {
    let key = crate::pairwise::session::pair_chain_key(me, peer);
    let chain = ctx.proto.recog.chain(key)?;
    let (_, epoch) = chain.sender_epoch()?;
    let next = chain.sender_next_ctr()?;
    for (back, cell) in cells.iter().rev().enumerate() {
        let ctr = next.checked_sub(1)?.checked_sub(back as u64)?;
        let Ok(plain) = cell.open_body(epoch.k_hdr().as_bytes(), ctr, 0) else {
            continue;
        };
        let (header, _) = crate::wire::inner::InnerHeader::parse(&plain).ok()?;
        return Some(header.obj_seq);
    }
    None
}

/// Cuts a pairwise ciphertext produced elsewhere (an SKDM entry) into cells
/// for `peer`, as a chain object or a sealed first flight carrying it.
fn send_pairwise_ct(
    ctx: &mut Ctx<'_, '_>,
    peer: PeerId,
    ct: &crate::crypto::backend::PairwiseCt,
) -> Result<Vec<Cell>, NodeError> {
    let me = ctx.me()?;
    let has_epoch = ctx
        .proto
        .pm
        .as_ref()
        .is_some_and(|pm| pm.send_epoch(&peer).is_some());
    if has_epoch {
        let Ctx { proto, rng, .. } = ctx;
        let pm = proto.pm.as_mut().ok_or(NodeError::NoIdentity)?;
        let r = pm.cells_for_ct(&mut proto.recog, &peer, ct, *rng);
        proto.dirty.pm = true;
        proto.dirty.recog = true;
        return r.map_err(Into::into);
    }
    let envelope_pub = ctx
        .proto
        .contacts
        .get(&peer)
        .and_then(|c| c.envelope_pub)
        .ok_or(NodeError::NeedsKey(peer))?;
    let my_envelope = ctx.identity()?.envelope.public();
    let mut seed = zeroize::Zeroizing::new([0u8; 32]);
    ctx.rng.fill_bytes(seed.as_mut());
    {
        let Ctx { proto, .. } = ctx;
        let pm = proto.pm.as_mut().ok_or(NodeError::NoIdentity)?;
        pm.start_send_epoch(&mut proto.recog, &peer, &seed, 0, 0);
        proto.dirty.pm = true;
        proto.dirty.recog = true;
    }
    let flight = FirstFlight {
        ct: ct.clone(),
        recog_seed: *seed,
        envelope_pub: my_envelope,
    };
    let cells = seal_flight(ctx, &me, &envelope_pub, &flight)?;
    ctx.after.push(After::SessionReady(peer));
    Ok(cells)
}

// ---- group sending ------------------------------------------------------------------

/// Rotates this node's Sender Key in `gid` (DESIGN §3 "Sender Key
/// rotation"; D3 K2): the bundle under the old chain to its holders, a
/// pairwise entry to every other member with a session; members without a
/// session are owed one and trigger the next rotation when it appears.
pub(crate) fn rotate(ctx: &mut Ctx<'_, '_>, gid: GroupId) -> Result<(), NodeError> {
    let me = ctx.me()?;
    let members: Vec<PeerId> = ctx
        .proto
        .groups
        .get(&gid)
        .ok_or(NodeError::UnknownGroup)?
        .view
        .members
        .keys()
        .copied()
        .filter(|k| *k != me)
        .collect();
    let (rotation, holders) = {
        let Ctx {
            txn,
            proto,
            backend,
            rng,
            t,
            ..
        } = ctx;
        let g = proto.groups.get_mut(&gid).ok_or(NodeError::UnknownGroup)?;
        let pm = proto.pm.as_mut().ok_or(NodeError::NoIdentity)?;
        let r = g.state.rotate(
            &mut **txn,
            *backend,
            &mut proto.recog,
            pm,
            &members,
            *t,
            *rng,
        )?;
        g.policy.on_rotated(*t);
        proto.dirty.recog = true;
        proto.dirty.pm = true;
        proto.dirty.groups.insert(gid);
        (r, std::mem::take(&mut g.holders))
    };
    let mut new_holders = BTreeSet::new();
    if !rotation.cells.is_empty() {
        for (peer, _) in &rotation.recipients {
            if holders.contains(peer) {
                new_holders.insert(*peer);
            }
        }
        emit(
            ctx,
            Emitted {
                cells: rotation.cells.clone(),
                class: Class::C1Control,
                pinned: false,
            },
        );
    }
    for (peer, ct) in &rotation.recipients {
        if new_holders.contains(peer) {
            continue;
        }
        match send_pairwise_ct(ctx, *peer, ct) {
            Ok(cells) => {
                emit(
                    ctx,
                    Emitted {
                        cells,
                        class: Class::C1Control,
                        pinned: false,
                    },
                );
                new_holders.insert(*peer);
            }
            Err(NodeError::NeedsKey(_)) => {
                ctx.events.push(Event::NeedsKey { peer: *peer });
            }
            Err(e) => return Err(e),
        }
    }
    for peer in &rotation.without_session {
        ctx.events.push(Event::NeedsKey { peer: *peer });
    }
    let g = ctx
        .proto
        .groups
        .get_mut(&gid)
        .ok_or(NodeError::UnknownGroup)?;
    g.holders = new_holders;
    Ok(())
}

/// Encrypts `post` under this node's chain in `gid`, rotating first when
/// due (DESIGN §3). Returns the cells and the object counter.
pub(crate) fn send_group(
    ctx: &mut Ctx<'_, '_>,
    gid: GroupId,
    post: &PostPlain,
) -> Result<(Vec<Cell>, u32, crate::crypto::backend::ChainId), NodeError> {
    let me = ctx.me()?;
    {
        let g = ctx.proto.groups.get(&gid).ok_or(NodeError::UnknownGroup)?;
        if !g.view.is_member(&me) && !ctx.adversary_keep_keys {
            return Err(NodeError::NotMember);
        }
    }
    let due = {
        let g = ctx.proto.groups.get(&gid).ok_or(NodeError::UnknownGroup)?;
        g.state.needs_rotation() || g.policy.due(ctx.t)
    };
    if due {
        rotate(ctx, gid)?;
    }
    let Ctx {
        txn,
        proto,
        backend,
        rng,
        ..
    } = ctx;
    let g = proto.groups.get_mut(&gid).ok_or(NodeError::UnknownGroup)?;
    let obj_seq = g.state.own_next_obj_seq().ok_or(GroupError::NoOwnChain)?;
    let chain = g.state.own_chain().ok_or(GroupError::NoOwnChain)?;
    let cells = g
        .state
        .encrypt_post(&mut **txn, *backend, &mut proto.recog, post, *rng)?;
    proto.dirty.recog = true;
    proto.dirty.groups.insert(gid);
    Ok((cells, obj_seq, chain))
}

use crate::groups::group::GroupError;

/// The next identity-signed header of this node in `gid` (DESIGN §2.4
/// common header; `objects::sender_chain`).
fn next_header(ctx: &mut Ctx<'_, '_>, gid: GroupId, kind: u8) -> Result<SignedHeader, NodeError> {
    let me = ctx.me()?;
    let g = ctx.proto.groups.get(&gid).ok_or(NodeError::UnknownGroup)?;
    let (seq, prev) = g.own_seq;
    let heads = g.dag.heads();
    SignedHeader::new(kind, gid, me, seq, prev, heads).map_err(|_| NodeError::Invalid("heads"))
}

/// Advances this node's identity-signed chain after `hash` was produced.
fn advance_own_seq(ctx: &mut Ctx<'_, '_>, gid: GroupId, hash: Hash32) -> Result<(), NodeError> {
    let g = ctx
        .proto
        .groups
        .get_mut(&gid)
        .ok_or(NodeError::UnknownGroup)?;
    g.own_seq = (g.own_seq.0.saturating_add(1), hash);
    ctx.proto.dirty.groups.insert(gid);
    Ok(())
}

/// Posts a carried roster op in the group and shares the whole roster with
/// `share_with` over 1:1 (the join path, DECISIONS §3 row (4,5,6,7) "the
/// admin who verified a newcomer relays ... in both directions").
fn publish_roster_op(
    ctx: &mut Ctx<'_, '_>,
    gid: GroupId,
    op: &RosterOp,
    share_with: &[PeerId],
) -> Result<(), NodeError> {
    let bytes = op.try_encode()?;
    let heads = ctx
        .proto
        .groups
        .get(&gid)
        .ok_or(NodeError::UnknownGroup)?
        .dag
        .heads();
    let post = PostPlain::new(kind::POST_CARRIED_ROSTER_OP, heads, bytes)?;
    for peer in share_with {
        share_roster_with(ctx, gid, *peer)?;
    }
    let members_besides_me = {
        let me = ctx.me()?;
        ctx.proto
            .groups
            .get(&gid)
            .ok_or(NodeError::UnknownGroup)?
            .view
            .members
            .keys()
            .any(|k| *k != me)
    };
    if members_besides_me {
        match send_group(ctx, gid, &post) {
            Ok((cells, _, _)) => {
                emit(
                    ctx,
                    Emitted {
                        cells,
                        class: Class::C1Control,
                        pinned: true,
                    },
                );
            }
            Err(NodeError::NotMember) => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// Sends every roster op this node holds for `gid` to `peer` over 1:1.
pub(crate) fn share_roster_with(
    ctx: &mut Ctx<'_, '_>,
    gid: GroupId,
    peer: PeerId,
) -> Result<(), NodeError> {
    let ops: Vec<Vec<u8>> = {
        let g = ctx.proto.groups.get(&gid).ok_or(NodeError::UnknownGroup)?;
        let mut ordered: Vec<(&Hash32, &RosterOp)> = g.dag.ops().collect();
        // Genesis first, then in hash order (the receiver parks and unparks).
        ordered.sort_by_key(|(h, op)| (!op.is_genesis(), **h));
        ordered
            .into_iter()
            .filter_map(|(_, op)| op.try_encode().ok())
            .collect()
    };
    for bytes in ops {
        let post = PostPlain::new(kind::POST_CARRIED_ROSTER_OP, Vec::new(), bytes)?;
        match send_pairwise(ctx, peer, &post) {
            Ok((cells, _)) => {
                emit(
                    ctx,
                    Emitted {
                        cells,
                        class: Class::C1Control,
                        pinned: true,
                    },
                );
            }
            Err(NodeError::NeedsKey(p)) => {
                ctx.events.push(Event::NeedsKey { peer: p });
                return Ok(());
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

// ---- SPK maintenance ----------------------------------------------------------------

/// A clock view for the bundle manager: mono = powered-on now; wall = the
/// display hook when configured. The retire step reads wall time only when
/// the powered-on-only clause could already fire, which is the one place the
/// deletion rule needs it (D6 rule 3).
struct MonoClock {
    mono: u64,
    wall: i64,
}

impl Clock for MonoClock {
    fn mono_ms(&self) -> u64 {
        self.mono
    }
    fn wall_for_display(&self) -> i64 {
        self.wall
    }
}

/// SPK rotation and retirement on the powered-on meter (BRIEF_v2 §5.3; D6
/// rule 3). Retirement needs wall time; it is attempted only once a retired
/// SPK is at least 48 h (powered-on) old, through the node's display hook.
pub(crate) fn spk_maintenance(ctx: &mut Ctx<'_, '_>) -> Result<(), NodeError> {
    let clock = MonoClock {
        mono: ctx.t,
        wall: 0,
    };
    let rotated = ctx
        .proto
        .bundles
        .maybe_rotate(&mut *ctx.txn, ctx.backend, &clock, ctx.rng)?;
    if rotated {
        ctx.proto.dirty.bundles = true;
    }
    Ok(())
}

// ---- receipts -------------------------------------------------------------------------

impl Node {
    /// Acknowledges everything received in `conv` to `peer` (DESIGN §2.4
    /// "Receipt"; D4 MERGE "cumulative per recipient per conversation").
    pub(crate) fn send_receipt(
        &mut self,
        peer: PeerId,
        conv: [u8; 16],
        _t: u64,
    ) -> Result<(), NodeError> {
        let seqs = self.proto.received.get(&conv).cloned().unwrap_or_default();
        // Nothing counted yet (only a first flight arrived): a receipt that
        // acknowledges no counter still proves the session and settles the
        // first flight's delivery state.
        let up_to = seqs.iter().next_back().copied().unwrap_or(0);
        let mut gaps = Vec::new();
        let mut expect = 0u32;
        for s in &seqs {
            if *s > expect {
                gaps.push(Gap {
                    from: expect,
                    to: s.saturating_sub(1),
                });
            }
            expect = s.saturating_add(1);
        }
        if seqs.is_empty() {
            gaps.push(Gap { from: 0, to: 0 });
        }
        gaps.truncate(255);
        let receipt = Receipt {
            conv,
            up_to_obj_seq: up_to,
            gaps,
        };
        let post = PostPlain::receipt(&receipt)?;
        self.run_txn(self.last_now_mono(), |ctx| {
            match send_pairwise(ctx, peer, &post) {
                Ok((cells, _)) => {
                    emit(
                        ctx,
                        Emitted {
                            cells,
                            class: Class::C1Control,
                            pinned: false,
                        },
                    );
                    Ok(())
                }
                Err(NodeError::NeedsKey(_)) => Ok(()),
                Err(e) => Err(e),
            }
        })
    }

    /// The last `now_mono` the platform passed in (receipts are scheduled
    /// right after the delivering transaction, at the same instant).
    pub(crate) fn last_now_mono(&self) -> u64 {
        self.last_now
    }

    // ---- public commands ---------------------------------------------------------

    /// Creates a group (DESIGN §2.4 Genesis; D3 M2 "the creator signs the
    /// initial admin set"): this node is an admin and a member; `admins`
    /// are further admins (bootstrapped in the genesis, each also a member
    /// with its name from `members` when listed there), `members` further
    /// members. The genesis is shared over 1:1 with every listed key.
    pub fn create_group(
        &mut self,
        name: &str,
        admins: &[KeyCard],
        members: &[MemberEntry],
        settings: u8,
        now_mono: u64,
    ) -> Result<GroupId, NodeError> {
        self.last_now = now_mono;
        let name = name.to_owned();
        let admins = admins.to_vec();
        let members = members.to_vec();
        self.run_txn(now_mono, move |ctx| {
            let me = ctx.identity()?;
            let signer_pub = me.card.ed;
            let mut all_admins = vec![me.card];
            all_admins.extend(admins.iter().copied().filter(|c| c.ed != signer_pub));
            let mut all_members = vec![MemberEntry {
                keycard: me.card,
                name: me.display_name.clone(),
                in_person: true,
            }];
            all_members.extend(members.iter().filter(|m| m.key() != signer_pub).cloned());
            let mut nonce = [0u8; 16];
            ctx.rng.fill_bytes(&mut nonce);
            let header = SignedHeader::new(
                okind::ROSTER_GENESIS,
                GroupId([0; 16]),
                signer_pub,
                0,
                Hash32::ZERO,
                Vec::new(),
            )
            .map_err(|_| NodeError::Invalid("heads"))?;
            let op = RosterOp::sign(
                &ctx.identity()?.signer,
                header,
                RosterBody::Genesis {
                    nonce,
                    name,
                    admins: all_admins.clone(),
                    members: all_members.clone(),
                    settings,
                },
            )?;
            let gid = op.group_id();
            let hash = op.op_hash();
            super::inbound::apply_roster_op(ctx, gid, op.clone(), None)?;
            advance_own_seq(ctx, gid, hash)?;
            let mut share: Vec<PeerId> = all_admins.iter().map(|c| c.ed).collect();
            share.extend(all_members.iter().map(MemberEntry::key));
            share.sort_unstable();
            share.dedup();
            share.retain(|k| *k != signer_pub);
            for peer in share {
                share_roster_with(ctx, gid, peer)?;
            }
            Ok(gid)
        })
    }

    /// A proposed Add for a second admin to endorse (BRIEF_v2 §5.4).
    pub fn propose_add(&self, group: GroupId, member: MemberEntry) -> Result<AddDraft, NodeError> {
        let me = self.proto.me().ok_or(NodeError::NoIdentity)?;
        let g = self
            .proto
            .groups
            .get(&group)
            .ok_or(NodeError::UnknownGroup)?;
        if !g.view.is_admin(&me) {
            return Err(NodeError::NotAdmin);
        }
        member.keycard.verify()?;
        Ok(AddDraft {
            group,
            subject: member,
        })
    }

    /// The second admin's endorsement of a draft (`in_person`: this admin
    /// verified the newcomer in person).
    pub fn endorse(&self, draft: &AddDraft, in_person: bool) -> Result<Endorsement, NodeError> {
        let id = self.proto.identity.as_ref().ok_or(NodeError::NoIdentity)?;
        let g = self
            .proto
            .groups
            .get(&draft.group)
            .ok_or(NodeError::UnknownGroup)?;
        if !g.view.is_admin(&id.card.ed) {
            return Err(NodeError::NotAdmin);
        }
        Ok(RosterOp::endorse(
            &id.signer,
            &draft.group,
            &draft.subject.key(),
            &draft.subject.name,
            in_person,
        )?)
    }

    /// Adds a member with a second admin's endorsement (BRIEF_v2 §5.4
    /// authority table; the 2-admin rule is judged by `roster::eval`, the
    /// node only builds a well-formed op). The roster is shared with the
    /// newcomer over 1:1 and the Add posted in the group; the next post
    /// rotates so the newcomer receives this node's chain.
    pub fn add_member(
        &mut self,
        group: GroupId,
        member: MemberEntry,
        endorsement: Endorsement,
        now_mono: u64,
    ) -> Result<(), NodeError> {
        self.last_now = now_mono;
        self.run_txn(now_mono, move |ctx| {
            let header = next_header(ctx, group, okind::ROSTER_ADD)?;
            let subject = member.key();
            let op = RosterOp::sign(
                &ctx.identity()?.signer,
                header,
                RosterBody::Add {
                    subject: member,
                    endorsement,
                },
            )?;
            let hash = op.op_hash();
            let applied = super::inbound::apply_roster_op(ctx, group, op.clone(), None)?;
            if !applied {
                return Err(NodeError::Invalid("roster op rejected"));
            }
            advance_own_seq(ctx, group, hash)?;
            publish_roster_op(ctx, group, &op, &[subject])
        })
    }

    /// Removes a member (any one admin, D3 M2). Every sender rotates before
    /// its next post once it learns of the removal (D3 K2), so the op is
    /// also sent to the removed member over 1:1: it learns "[Name] removed"
    /// (D3 UI labels) although it can no longer read the group.
    pub fn remove_member(
        &mut self,
        group: GroupId,
        member: IdentityKey,
        now_mono: u64,
    ) -> Result<(), NodeError> {
        self.last_now = now_mono;
        self.run_txn(now_mono, move |ctx| {
            let header = next_header(ctx, group, okind::ROSTER_REMOVE)?;
            let op = RosterOp::sign(
                &ctx.identity()?.signer,
                header,
                RosterBody::Remove { subject: member },
            )?;
            let hash = op.op_hash();
            let applied = super::inbound::apply_roster_op(ctx, group, op.clone(), None)?;
            if !applied {
                return Err(NodeError::Invalid("roster op rejected"));
            }
            advance_own_seq(ctx, group, hash)?;
            publish_roster_op(ctx, group, &op, &[member])
        })
    }

    /// Posts a text in the group (BRIEF_v2 §5.9 delivery states follow).
    pub fn send_text(
        &mut self,
        group: GroupId,
        text: &str,
        now_mono: u64,
    ) -> Result<ObjId, NodeError> {
        self.last_now = now_mono;
        let body = TextBody::new(text, None)?;
        self.run_txn(now_mono, move |ctx| {
            let me = ctx.me()?;
            let heads = ctx
                .proto
                .groups
                .get(&group)
                .ok_or(NodeError::UnknownGroup)?
                .dag
                .heads();
            let post = PostPlain::text(heads, &body)?;
            let (cells, obj_seq, chain) = send_group(ctx, group, &post)?;
            let conv = group_conv(&group, chain);
            let ids = emit(
                ctx,
                Emitted {
                    cells,
                    class: Class::C2Text,
                    pinned: true,
                },
            );
            let obj = track(
                ctx,
                conv,
                me,
                obj_seq,
                Target::Group(group),
                kind::POST_TEXT,
                ids,
            );
            persist::save_plaintext(
                &mut *ctx.txn,
                &obj,
                &persist::PlaintextRec {
                    conv,
                    author: me,
                    obj_seq,
                    kind: kind::POST_TEXT,
                    group: Some(group),
                    plaintext: post.encode(),
                },
            );
            Ok(obj)
        })
    }

    /// Sends a 1:1 text (D6: a sealed first flight when no epoch exists yet;
    /// `NeedsKey` when the peer's bundle has not arrived).
    pub fn send_dm(
        &mut self,
        peer: IdentityKey,
        text: &str,
        now_mono: u64,
    ) -> Result<ObjId, NodeError> {
        self.last_now = now_mono;
        let body = TextBody::new(text, None)?;
        self.run_txn(now_mono, move |ctx| {
            let me = ctx.me()?;
            let post = PostPlain::text(Vec::new(), &body)?;
            let conv = dm_conv(&me, &peer);
            match send_pairwise(ctx, peer, &post) {
                Ok((cells, obj_seq)) => {
                    let ids = emit(
                        ctx,
                        Emitted {
                            cells,
                            class: Class::C2Text,
                            pinned: true,
                        },
                    );
                    let obj = track(
                        ctx,
                        conv,
                        me,
                        obj_seq,
                        Target::Peer(peer),
                        kind::POST_TEXT,
                        ids,
                    );
                    persist::save_plaintext(
                        &mut *ctx.txn,
                        &obj,
                        &persist::PlaintextRec {
                            conv,
                            author: me,
                            obj_seq,
                            kind: kind::POST_TEXT,
                            group: None,
                            plaintext: post.encode(),
                        },
                    );
                    Ok(obj)
                }
                Err(NodeError::NeedsKey(p)) => {
                    let obj = object_id(&conv, &me, ctx.rng.next_u32());
                    ctx.proto.outbox.insert(
                        obj,
                        OutboxEntry {
                            conv,
                            obj_seq: FIRST_FLIGHT_OBJ_SEQ,
                            target: Target::Peer(peer),
                            kind: kind::POST_TEXT,
                            created_t: ctx.t,
                            state: DeliveryState::NeedsKey,
                            cells: Vec::new(),
                            passed: false,
                        },
                    );
                    ctx.proto.dirty.outbox = true;
                    ctx.events.push(Event::NeedsKey { peer: p });
                    ctx.events.push(Event::Delivery {
                        obj,
                        state: DeliveryState::NeedsKey,
                    });
                    Ok(obj)
                }
                Err(e) => Err(e),
            }
        })
    }

    /// "Resend" (BRIEF_v2 §5.9 "May have expired · Resend"): sends the stored
    /// text again as a new object.
    pub fn resend(&mut self, obj: &ObjId, now_mono: u64) -> Result<ObjId, NodeError> {
        let entry = self
            .proto
            .outbox
            .get(obj)
            .cloned()
            .ok_or(NodeError::UnknownObject)?;
        let text = self.plaintext(obj)?.ok_or(NodeError::UnknownObject)?;
        match entry.target {
            Target::Group(g) => self.send_text(g, &text, now_mono),
            Target::Peer(p) => self.send_dm(p, &text, now_mono),
        }
    }

    /// Creates a task (DESIGN §4; any member, docs/decisions-assumed.md row 5).
    pub fn create_task(
        &mut self,
        group: GroupId,
        title: &str,
        details: &str,
        now_mono: u64,
    ) -> Result<TaskId, NodeError> {
        self.last_now = now_mono;
        let op = TaskOp::create(title, details)?;
        self.run_txn(now_mono, move |ctx| {
            let me = ctx.me()?;
            let heads = ctx
                .proto
                .groups
                .get(&group)
                .ok_or(NodeError::UnknownGroup)?
                .dag
                .heads();
            let post = PostPlain::new(kind::POST_TASK_OP, heads, op.encode())?;
            let task_id = task_id_for_create(&me, &post.encode());
            super::inbound::handle_task_op(ctx, group, me, &post)?;
            let (cells, _, _) = send_group(ctx, group, &post)?;
            emit(
                ctx,
                Emitted {
                    cells,
                    class: Class::C1Control,
                    pinned: false,
                },
            );
            Ok(task_id)
        })
    }

    fn task_op(
        &mut self,
        group: GroupId,
        task_id: TaskId,
        op: TaskKind,
        now_mono: u64,
    ) -> Result<(), NodeError> {
        self.last_now = now_mono;
        self.run_txn(now_mono, move |ctx| {
            let me = ctx.me()?;
            let (heads, task_heads_now) = {
                let g = ctx
                    .proto
                    .groups
                    .get(&group)
                    .ok_or(NodeError::UnknownGroup)?;
                let task = g.tasks.get(&task_id).ok_or(NodeError::UnknownTask)?;
                (g.dag.heads(), task_heads(task))
            };
            let top = TaskOp::on_task(task_id, op, task_heads_now)?;
            let post = PostPlain::new(kind::POST_TASK_OP, heads, top.encode())?;
            super::inbound::handle_task_op(ctx, group, me, &post)?;
            let (cells, _, _) = send_group(ctx, group, &post)?;
            let ids = emit(
                ctx,
                Emitted {
                    cells,
                    class: Class::C1Control,
                    pinned: false,
                },
            );
            if op == TaskKind::Take
                && let Some(task) = ctx
                    .proto
                    .groups
                    .get_mut(&group)
                    .and_then(|g| g.tasks.get_mut(&task_id))
            {
                task.own_take_cells = ids;
                persist::save_task_meta(&mut *ctx.txn, &group, &task_id, task)?;
            }
            Ok(())
        })
    }

    /// "Take" (BRIEF_v2 §7 vocabulary): a local claim, effective at once.
    pub fn take_task(
        &mut self,
        group: GroupId,
        task_id: TaskId,
        now_mono: u64,
    ) -> Result<(), NodeError> {
        self.task_op(group, task_id, TaskKind::Take, now_mono)
    }

    /// "Release".
    pub fn release_task(
        &mut self,
        group: GroupId,
        task_id: TaskId,
        now_mono: u64,
    ) -> Result<(), NodeError> {
        self.task_op(group, task_id, TaskKind::Release, now_mono)
    }

    /// "Done".
    pub fn done_task(
        &mut self,
        group: GroupId,
        task_id: TaskId,
        now_mono: u64,
    ) -> Result<(), NodeError> {
        self.task_op(group, task_id, TaskKind::Done, now_mono)
    }

    /// Opens a vote (DESIGN §5; BRIEF_v2 §4 "Opening"): `N = |members(H)|`
    /// and `T = ⌊(N + f)/2⌋ + 1` from the current heads.
    pub fn open_vote(
        &mut self,
        group: GroupId,
        question: &str,
        options: &[&str],
        f: u8,
        now_mono: u64,
    ) -> Result<VoteId, NodeError> {
        self.last_now = now_mono;
        let question = question.to_owned();
        let options: Vec<String> = options.iter().map(|o| (*o).to_owned()).collect();
        self.run_txn(now_mono, move |ctx| {
            let header = next_header(ctx, group, okind::VOTE_OPEN)?;
            let n = u16::try_from(
                ctx.proto
                    .groups
                    .get(&group)
                    .ok_or(NodeError::UnknownGroup)?
                    .view
                    .member_count(),
            )
            .map_err(|_| NodeError::Invalid("members"))?;
            let open = VoteOpen::sign(
                &ctx.identity()?.signer,
                header,
                question,
                options,
                f,
                threshold(n, f),
                n,
            )?;
            let hash = open.try_op_hash()?;
            let vote_id = open.try_vote_id()?;
            let bytes = open.try_encode()?;
            super::inbound::handle_vote_open(ctx, group, &bytes, None)?;
            advance_own_seq(ctx, group, hash)?;
            if let Some(v) = ctx.proto.votes.get(&vote_id)
                && let Some(r) = v.reject
            {
                return Err(NodeError::OpenRejected(r));
            }
            let heads = ctx
                .proto
                .groups
                .get(&group)
                .ok_or(NodeError::UnknownGroup)?
                .dag
                .heads();
            let post = PostPlain::new(kind::POST_CARRIED_VOTE_OPEN, heads, bytes)?;
            let (cells, _, _) = send_group(ctx, group, &post)?;
            emit(
                ctx,
                Emitted {
                    cells,
                    class: Class::C1Control,
                    pinned: false,
                },
            );
            Ok(vote_id)
        })
    }

    /// Casts a ballot (DESIGN §5 "Ballot": no revotes; a second payload
    /// voids the key).
    pub fn cast_ballot(
        &mut self,
        vote_id: VoteId,
        choice: u8,
        now_mono: u64,
    ) -> Result<(), NodeError> {
        self.last_now = now_mono;
        self.run_txn(now_mono, move |ctx| {
            let group = ctx
                .proto
                .votes
                .get(&vote_id)
                .ok_or(NodeError::UnknownVote)?
                .group;
            let ballot = Ballot::sign(&ctx.identity()?.signer, vote_id, choice);
            let bytes = ballot.encode();
            super::inbound::handle_ballot(ctx, &bytes, None)?;
            let heads = ctx
                .proto
                .groups
                .get(&group)
                .ok_or(NodeError::UnknownGroup)?
                .dag
                .heads();
            let post = PostPlain::new(kind::POST_CARRIED_BALLOT, heads, bytes)?;
            let (cells, _, _) = send_group(ctx, group, &post)?;
            emit(
                ctx,
                Emitted {
                    cells,
                    class: Class::C1Control,
                    pinned: false,
                },
            );
            Ok(())
        })
    }

    /// Provisions a contact (what a completed pairing or a simulator step
    /// yields): verifies the KeyCard, pins its X25519 key (D3 K1), starts the
    /// session from the bundle, stores the contact with its badge, `k_pair`
    /// and envelope key (BRIEF_v2 §3), and updates the same-name index.
    pub fn import_contact(
        &mut self,
        contact: ImportedContact,
        now_mono: u64,
    ) -> Result<(), NodeError> {
        self.last_now = now_mono;
        let at_display = if contact.badge == ImportBadge::VerifiedInPerson {
            self.wall_for_display().unwrap_or(0)
        } else {
            0
        };
        self.run_txn(now_mono, move |ctx| {
            let me = ctx.me()?;
            contact.keycard.verify()?;
            let key = contact.keycard.ed;
            if key == me {
                return Err(NodeError::Invalid("cannot import oneself"));
            }
            ctx.backend
                .pin_peer_x25519(&mut *ctx.txn, &key, &contact.keycard.x25519)?;
            ctx.backend.process_prekey_bundle(
                &mut *ctx.txn,
                &key,
                &contact.bundle,
                ctx.t,
                ctx.rng,
            )?;
            let source = match contact.badge {
                ImportBadge::VerifiedInPerson => PinSource::QrScan { at_mono: ctx.t },
                ImportBadge::Unverified => PinSource::DirectMessage,
            };
            let stored = {
                let mut store = ContactStore::new(&mut *ctx.txn);
                let mut c = store.first_contact(
                    &key,
                    contact.keycard.x25519,
                    &contact.display_name,
                    source,
                    at_display,
                )?;
                if contact.link_key.is_some() {
                    c.link_key = contact.link_key;
                }
                c.envelope_pub = Some(contact.envelope_pub);
                store.upsert(&c)?;
                c
            };
            ctx.proto.contacts.insert(
                key,
                ContactLite {
                    display_name: stored.display_name.clone(),
                    link_key: stored.link_key,
                    envelope_pub: stored.envelope_pub,
                    verified: matches!(
                        stored.badge,
                        crate::people::contacts::Badge::VerifiedInPerson { .. }
                    ),
                },
            );
            ctx.proto.samename.observe(key, &contact.display_name);
            if let Some(marker) = ctx.proto.samename.marker_for(&key) {
                ctx.events.push(Event::SameName {
                    key,
                    text: marker.text(&contact.display_name),
                });
            }
            if let Some(pm) = ctx.proto.pm.as_mut() {
                pm.mark_needs_key(&key);
                ctx.proto.dirty.pm = true;
            }
            ctx.after.push(After::SessionReady(key));
            Ok(())
        })
    }

    /// Issues this node's prekey bundle for `context` (`pairwise::bundles`:
    /// one-time prekeys only at a scan or a direct encounter, D6 rule 2).
    pub fn my_bundle(
        &mut self,
        context: IssueContext,
        now_mono: u64,
    ) -> Result<Vec<u8>, NodeError> {
        self.last_now = now_mono;
        let one_time = match context {
            IssueContext::QrScan | IssueContext::DirectEncounter => OPK_RESERVE_ASM,
            _ => 0,
        };
        self.run_txn(now_mono, move |ctx| {
            if ctx.proto.identity.is_none() {
                return Err(NodeError::NoIdentity);
            }
            let bundle =
                ctx.proto
                    .bundles
                    .issue(&mut *ctx.txn, ctx.backend, context, one_time, ctx.rng)?;
            ctx.proto.dirty.bundles = true;
            Ok(bundle)
        })
    }

    /// The QR code to show for "Verify in person" (`pairwise::pairing`).
    pub fn my_qr_code(&mut self, now_mono: u64) -> Result<QrCode, NodeError> {
        let bundle = self.my_bundle(IssueContext::QrScan, now_mono)?;
        let card = self.my_keycard()?;
        let envelope = self.my_envelope_pub()?;
        Ok(QrCode::generate(
            QrPayload::new(card, envelope, bundle),
            &mut self.rng,
        ))
    }

    /// Starts the scanner side of a pairing over `link` after scanning
    /// `scanned` (`shown` is this node's own displayed code).
    pub fn start_pairing_scanner(
        &mut self,
        link: LinkId,
        shown: &QrCode,
        scanned: QrCode,
    ) -> Result<(), NodeError> {
        let end = self
            .relay
            .links
            .get(&link)
            .ok_or(NodeError::UnknownLink(link))?;
        let hh = end
            .hh
            .ok_or(NodeError::Invalid("link handshake not finished"))?;
        let session =
            PairingSession::scanner(shown.payload.clone(), scanned, end.role, hh, &mut self.rng)?;
        self.pairing.insert(link, session);
        Ok(())
    }

    /// Starts the displayer side of a pairing over `link` for the code shown.
    pub fn start_pairing_displayer(
        &mut self,
        link: LinkId,
        shown: QrCode,
    ) -> Result<(), NodeError> {
        let end = self
            .relay
            .links
            .get(&link)
            .ok_or(NodeError::UnknownLink(link))?;
        let hh = end
            .hh
            .ok_or(NodeError::Invalid("link handshake not finished"))?;
        let session = PairingSession::displayer(shown, end.role, hh, &mut self.rng);
        self.pairing.insert(link, session);
        Ok(())
    }

    /// Advances the pairing on `link` with the peer's message (the caller
    /// ferries the bytes over the direct link). On `Done` the contact is
    /// stored as "Verified in person" with its link key and envelope key,
    /// the session adopted and the link lifted to L2.
    pub fn pairing_step(
        &mut self,
        link: LinkId,
        incoming: Option<&[u8]>,
        now_mono: u64,
    ) -> Result<PairingStep, NodeError> {
        self.last_now = now_mono;
        let mut session = self
            .pairing
            .remove(&link)
            .ok_or(NodeError::Invalid("no pairing on this link"))?;
        let incoming = incoming.map(<[u8]>::to_vec);
        let at_display = self.wall_for_display().unwrap_or(0);
        let result = self.run_txn(now_mono, |ctx| {
            let step = session.step(
                &mut *ctx.txn,
                ctx.backend,
                incoming.as_deref(),
                ctx.t,
                ctx.rng,
            )?;
            if let PairingStep::Done(paired) = &step {
                let peer = paired.peer;
                let name = ctx
                    .proto
                    .contacts
                    .get(&peer)
                    .map(|c| c.display_name.clone())
                    .unwrap_or_default();
                let stored = {
                    let mut store = ContactStore::new(&mut *ctx.txn);
                    let mut c = store.first_contact(
                        &peer,
                        paired.x25519,
                        &name,
                        PinSource::QrScan { at_mono: ctx.t },
                        at_display,
                    )?;
                    c.link_key = Some(paired.link_key);
                    c.envelope_pub = Some(paired.envelope_pub);
                    store.upsert(&c)?;
                    c
                };
                ctx.proto.contacts.insert(
                    peer,
                    ContactLite {
                        display_name: stored.display_name.clone(),
                        link_key: stored.link_key,
                        envelope_pub: stored.envelope_pub,
                        verified: true,
                    },
                );
                let Ctx { proto, .. } = ctx;
                let pm = proto.pm.as_mut().ok_or(NodeError::NoIdentity)?;
                pm.adopt_pairing(&mut proto.recog, paired);
                proto.dirty.pm = true;
                proto.dirty.recog = true;
                ctx.after.push(After::SessionReady(peer));
            }
            Ok(step)
        });
        match &result {
            Ok(PairingStep::Done(paired)) => {
                if let Some(end) = self.relay.links.get_mut(&link) {
                    end.auth = LinkAuth::VerifiedContact(paired.peer);
                    end.platform.set_auth(end.auth);
                }
            }
            Ok(_) => {
                self.pairing.insert(link, session);
            }
            Err(_) => {}
        }
        result
    }
}
