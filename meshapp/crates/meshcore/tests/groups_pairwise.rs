//! Groups, pairwise sessions and QR pairing end to end over the real
//! backend (default features): DESIGN §10 "SKDM atomic check (mismatched
//! x25519 → no store write); pairwise crossed initiation; SPK deletion rule
//! on powered-on time"; BRIEF §3 level 2 pairing; D3 K1–K2; D6 rules 2–3.

#![cfg(feature = "backend-libsignal")]

use std::collections::BTreeMap;

use meshcore::clock::{Clock, ManualClock, ms};
use meshcore::codec::{Encode, decode, encode};
use meshcore::crypto::backend::libsignal::LibsignalBackend;
use meshcore::crypto::backend::{
    Backend, ChainId, GroupId, PairwiseBackend, PairwiseKind, PeerId, PreKeyBundleWire,
    default_backend,
};
use meshcore::crypto::ed25519::IdentitySigner;
use meshcore::crypto::keycard::KeyCard;
use meshcore::groups::group::{GroupError, GroupState, Inbound, Rotation};
use meshcore::objects::chat_post::{PostPlain, TextBody, kind};
use meshcore::objects::skdm_bundle::SkdmEntry;
use meshcore::pairwise::bundles::{BundleError, BundleManager, IssueContext};
use meshcore::pairwise::pairing::{
    Paired, PairingError, PairingMsg, PairingSession, PairingStep, QrCode, QrPayload, link_key,
};
use meshcore::pairwise::session::{
    FirstFlight, PairwiseError, PairwiseManager, Received, SessionState,
};
use meshcore::storage::kv::{KvTxn, MemKv, ns};
use meshcore::transport::link::Role;
use meshcore::wire::cell::Cell;
use meshcore::wire::frame::Class;
use meshcore::wire::inner::{InnerKind, Reassembler, fragment};
use meshcore::wire::noise::{NoiseLink, contact_proof, verify_contact_proof};
use meshcore::wire::recog::TagTable;
use meshcore::wire::seal::{self, EnvelopeKeyPair, Sealed, ff_open_cell, ff_seal_cell};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

fn be() -> Box<dyn Backend> {
    default_backend()
}

/// One mesh node: store, identity, KeyCard, envelope key, managers.
struct Node {
    kv: MemKv,
    signer: IdentitySigner,
    card: KeyCard,
    rng: ChaCha20Rng,
    env: EnvelopeKeyPair,
    pm: PairwiseManager,
    recog: TagTable,
    bm: BundleManager,
    groups: BTreeMap<GroupId, GroupState>,
}

impl Node {
    fn new(seed: u8) -> Node {
        let signer = IdentitySigner::from_seed(&[seed; 32]);
        let mut kv = MemKv::new();
        let mut rng = ChaCha20Rng::seed_from_u64(u64::from(seed));
        let x25519 = be().init_identity(&mut kv, &mut rng).expect("identity");
        let card = KeyCard::issue(&signer, x25519);
        let env = EnvelopeKeyPair::generate(&mut rng);
        Node {
            kv,
            pm: PairwiseManager::new(signer.public()),
            signer,
            card,
            rng,
            env,
            recog: TagTable::new(),
            bm: BundleManager::new(1_000),
            groups: BTreeMap::new(),
        }
    }

    fn id(&self) -> PeerId {
        self.signer.public()
    }

    /// Pin `other`'s key from its roster KeyCard.
    fn pin(&mut self, other: &Node) {
        let card = KeyCard::from_bytes(&other.card.to_bytes()).expect("keycard verifies");
        be().pin_peer_x25519(&mut self.kv, &card.ed, &card.x25519)
            .expect("pin");
    }

    /// Establishes sessions both ways with `other` (bundle inside the group
    /// encryption, a first message and a reply), as co-members do.
    fn connect(&mut self, other: &mut Node, now: u64) {
        self.pin(other);
        other.pin(self);
        let bundle = other
            .bm
            .issue(
                &mut other.kv,
                &*be(),
                IssueContext::InsideGroup(GroupId([0; 16])),
                0,
                &mut other.rng,
            )
            .expect("bundle");
        be().process_prekey_bundle(&mut self.kv, &other.id(), &bundle, now, &mut self.rng)
            .expect("process bundle");
        let hello = text("hello", vec![]).encode();
        let ct = self
            .pm
            .encrypt_to(
                &mut self.kv,
                &*be(),
                &other.id(),
                &hello,
                now,
                &mut self.rng,
            )
            .expect("first");
        assert_eq!(ct.kind, PairwiseKind::PreKey);
        let got = other
            .pm
            .decrypt_from(
                &mut other.kv,
                &*be(),
                &mut other.recog,
                &self.id(),
                &ct,
                &mut other.rng,
            )
            .expect("decrypt first");
        assert!(matches!(got, Received::Post(p) if text_of(&p) == "hello"));
        let reply = other
            .pm
            .encrypt_to(
                &mut other.kv,
                &*be(),
                &self.id(),
                &hello,
                now,
                &mut other.rng,
            )
            .expect("reply");
        self.pm
            .decrypt_from(
                &mut self.kv,
                &*be(),
                &mut self.recog,
                &other.id(),
                &reply,
                &mut self.rng,
            )
            .expect("decrypt reply");
        assert_eq!(self.pm.state(&other.id()), SessionState::Ready);
        assert_eq!(other.pm.state(&self.id()), SessionState::Ready);
        // recognition seeds both ways, so posts can be recognised
        let mut seed_a = [0u8; 32];
        rand::RngCore::fill_bytes(&mut self.rng, &mut seed_a);
        let mut seed_b = [0u8; 32];
        rand::RngCore::fill_bytes(&mut other.rng, &mut seed_b);
        self.pm
            .start_send_epoch(&mut self.recog, &other.id(), &seed_a, 0, 0);
        other
            .pm
            .register_recv_epoch(&mut other.recog, &self.id(), &seed_a, 0);
        other
            .pm
            .start_send_epoch(&mut other.recog, &self.id(), &seed_b, 0, 0);
        self.pm
            .register_recv_epoch(&mut self.recog, &other.id(), &seed_b, 0);
    }

    fn group(&mut self, g: GroupId) -> &mut GroupState {
        let me = self.id();
        self.groups
            .entry(g)
            .or_insert_with(|| GroupState::new(g, me))
    }

    /// Everything this node can read out of `cells` on its 1:1 chains.
    fn receive_1to1(&mut self, cells: &[Cell]) -> Vec<(PeerId, Received)> {
        let mut out = Vec::new();
        for c in cells {
            let hit = self.recog.lookup(&c.tag()).expect("recognised 1:1 cell");
            if let Some(r) = self
                .pm
                .decrypt_cell(&mut self.kv, &*be(), &mut self.recog, hit, c, &mut self.rng)
                .expect("decrypt 1:1 cell")
            {
                out.push(r);
            }
        }
        out
    }

    /// Everything this node can read out of `cells` on group `g`'s chains.
    fn receive_group(&mut self, g: GroupId, cells: &[Cell]) -> Result<Vec<Inbound>, GroupError> {
        let mut pairs = Vec::new();
        for c in cells {
            let hit = self.recog.lookup(&c.tag()).expect("recognised group cell");
            pairs.push((hit, c.clone()));
        }
        let Node {
            kv, recog, groups, ..
        } = self;
        groups
            .get_mut(&g)
            .expect("group")
            .decrypt_cells(kv, &*be(), recog, &pairs)
    }

    fn accept_rotation(
        &mut self,
        from: &Node,
        g: GroupId,
        rot: &Rotation,
    ) -> Result<ChainId, GroupError> {
        let expected = from.card.x25519;
        let Node {
            kv,
            recog,
            groups,
            rng,
            pm,
            ..
        } = self;
        groups.get_mut(&g).expect("group").accept_bundle(
            kv,
            &*be(),
            recog,
            pm,
            from.id(),
            expected,
            &rot.bundle,
            rng,
        )
    }

    fn rotate(&mut self, g: GroupId, members: &[PeerId], now: u64) -> Rotation {
        let Node {
            kv,
            recog,
            groups,
            rng,
            pm,
            ..
        } = self;
        groups
            .get_mut(&g)
            .expect("group")
            .rotate(kv, &*be(), recog, pm, members, now, rng)
            .expect("rotate")
    }

    /// The current chain's SKDM for `to`, as the node re-sends it to a
    /// member reached later.
    fn skdm_entry_for(
        &mut self,
        g: GroupId,
        to: &PeerId,
        now: u64,
    ) -> Result<SkdmEntry, GroupError> {
        let Node {
            kv,
            recog,
            groups,
            rng,
            pm,
            ..
        } = self;
        groups
            .get(&g)
            .expect("group")
            .skdm_entry_for(kv, &*be(), recog, pm, to, now, rng)
    }

    /// Receives one pairwise SKDM entry from `from` the way the node's 1:1
    /// path does: decrypt through the manager, then the atomic acceptance.
    fn accept_entry(
        &mut self,
        from: &Node,
        g: GroupId,
        entry: &SkdmEntry,
    ) -> Result<ChainId, GroupError> {
        let ct: meshcore::crypto::backend::PairwiseCt = decode(&entry.ct).expect("a pairwise ct");
        let got = self
            .pm
            .decrypt_from(
                &mut self.kv,
                &*be(),
                &mut self.recog,
                &from.id(),
                &ct,
                &mut self.rng,
            )
            .expect("entry decrypts");
        let Received::Post(post) = got else {
            panic!("post expected");
        };
        let expected = from.card.x25519;
        let Node {
            kv, recog, groups, ..
        } = self;
        groups.get_mut(&g).expect("group").accept_skdm_post(
            kv,
            &*be(),
            recog,
            from.id(),
            expected,
            &post,
        )
    }

    fn post(&mut self, g: GroupId, plain: &PostPlain) -> Result<Vec<Cell>, GroupError> {
        let Node {
            kv,
            recog,
            groups,
            rng,
            ..
        } = self;
        groups
            .get_mut(&g)
            .expect("group")
            .encrypt_post(kv, &*be(), recog, plain, rng)
    }

    fn send_1to1(
        &mut self,
        to: &PeerId,
        plain: &PostPlain,
        now: u64,
    ) -> Result<Vec<Cell>, PairwiseError> {
        self.pm.send_post(
            &mut self.kv,
            &*be(),
            &mut self.recog,
            to,
            plain,
            now,
            &mut self.rng,
        )
    }

    fn count(&self, namespace: &str) -> usize {
        self.kv.scan_prefix(namespace, &[]).len()
    }
}

fn snapshot(kv: &MemKv) -> Vec<(&'static str, Vec<u8>, Vec<u8>)> {
    [
        ns::IDENTITY,
        ns::SESSIONS,
        ns::PREKEYS,
        ns::SIGNED_PREKEYS,
        ns::KYBER_PREKEYS,
        ns::SENDER_KEYS,
        ns::PEER_IDENTITIES,
    ]
    .into_iter()
    .flat_map(|n| {
        kv.scan_prefix(n, &[])
            .into_iter()
            .map(move |(k, v)| (n, k, v))
    })
    .collect()
}

fn text(s: &str, heads: Vec<meshcore::crypto::hash::Hash32>) -> PostPlain {
    PostPlain::text(heads, &TextBody::new(s, None).unwrap()).unwrap()
}

fn text_of(post: &PostPlain) -> String {
    post.text_body().unwrap().text
}

/// A Noise NN handshake between two links; returns the shared hash.
fn handshake(seed_i: u64, seed_r: u64) -> [u8; 32] {
    let mut ri = ChaCha20Rng::seed_from_u64(seed_i);
    let mut rr = ChaCha20Rng::seed_from_u64(seed_r);
    let mut ni = NoiseLink::initiator(&mut ri).unwrap();
    let mut nr = NoiseLink::responder(&mut rr).unwrap();
    let m1 = ni.write_handshake().unwrap();
    nr.read_handshake(&m1).unwrap();
    let m2 = nr.write_handshake().unwrap();
    ni.read_handshake(&m2).unwrap();
    assert!(ni.is_handshake_finished() && nr.is_handshake_finished());
    assert_eq!(ni.handshake_hash(), nr.handshake_hash());
    ni.handshake_hash()
}

fn qr_payload(n: &mut Node, one_time: u8) -> QrPayload {
    let bundle =
        n.bm.issue(
            &mut n.kv,
            &*be(),
            IssueContext::QrScan,
            one_time,
            &mut n.rng,
        )
        .unwrap();
    QrPayload::new(n.card, n.env.public(), bundle)
}

/// Runs a pairing to completion: `scanner` scanned `displayer`'s code.
/// Returns both outcomes and the transcript of link messages.
fn pair(
    scanner: &mut Node,
    displayer: &mut Node,
    hh: [u8; 32],
) -> (Paired, Paired, Vec<Vec<u8>>, [u8; 32]) {
    let shown = QrCode::generate(qr_payload(displayer, 1), &mut displayer.rng);
    let secret = shown.secret;
    let mine = qr_payload(scanner, 1);
    let mut s =
        PairingSession::scanner(mine, shown.clone(), Role::Central, hh, &mut scanner.rng).unwrap();
    let mut d = PairingSession::displayer(shown, Role::Peripheral, hh, &mut displayer.rng);
    let (mut to_d, mut to_s): (Option<Vec<u8>>, Option<Vec<u8>>) = (None, None);
    let (mut ps, mut pd) = (None, None);
    let mut transcript = Vec::new();
    for round in 0..8 {
        if ps.is_none() {
            match s
                .step(
                    &mut scanner.kv,
                    &*be(),
                    to_s.take().as_deref(),
                    10 + round,
                    &mut scanner.rng,
                )
                .expect("scanner step")
            {
                PairingStep::Send(b) => {
                    transcript.push(b.clone());
                    to_d = Some(b);
                }
                PairingStep::Done(p) => ps = Some(p),
                PairingStep::Wait => {}
            }
        }
        if pd.is_none() {
            match d
                .step(
                    &mut displayer.kv,
                    &*be(),
                    to_d.take().as_deref(),
                    20 + round,
                    &mut displayer.rng,
                )
                .expect("displayer step")
            {
                PairingStep::Send(b) => {
                    transcript.push(b.clone());
                    to_s = Some(b);
                }
                PairingStep::Done(p) => pd = Some(p),
                PairingStep::Wait => {}
            }
        }
        if ps.is_some() && pd.is_some() {
            break;
        }
    }
    let (ps, pd) = (ps.expect("scanner done"), pd.expect("displayer done"));
    assert!(s.is_finished() && d.is_finished());
    scanner.pm.adopt_pairing(&mut scanner.recog, &ps);
    displayer.pm.adopt_pairing(&mut displayer.recog, &pd);
    (ps, pd, transcript, secret)
}

// ---- pairing ----------------------------------------------------------------------

#[test]
fn two_nodes_pair_over_a_direct_link() {
    let (mut a, mut b) = (Node::new(1), Node::new(2));
    let hh = handshake(1, 2);
    let (pa, pb, transcript, secret) = pair(&mut a, &mut b, hh);
    assert_eq!(transcript.len(), 4, "Offer, Accept, Prove, Reply");
    // pinned keys and sessions both ways
    assert_eq!(pa.peer, b.id());
    assert_eq!(pb.peer, a.id());
    assert_eq!(pa.x25519, b.card.x25519);
    assert_eq!(pb.x25519, a.card.x25519);
    assert_eq!(be().remote_session_key(&a.kv, &b.id()), Some(b.card.x25519));
    assert_eq!(be().remote_session_key(&b.kv, &a.id()), Some(a.card.x25519));
    assert!(be().has_session(&a.kv, &b.id()));
    assert!(be().has_session(&b.kv, &a.id()));
    // the same link key on both sides, envelope keys exchanged
    assert_eq!(pa.link_key, pb.link_key);
    assert_eq!(pa.envelope_pub, b.env.public());
    assert_eq!(pb.envelope_pub, a.env.public());
    // recognition epochs: each side's send seed is the other's receive seed
    assert_eq!(pa.send_seed, pb.recv_seed);
    assert_eq!(pb.send_seed, pa.recv_seed);
    assert_ne!(pa.send_seed, pb.send_seed);
    assert_eq!(a.pm.state(&b.id()), SessionState::Ready);
    assert_eq!(b.pm.state(&a.id()), SessionState::Ready);
    assert_eq!(a.pm.send_epoch(&b.id()), Some(0));
    assert_eq!(a.pm.recv_epochs(&b.id()), &[0]);
    assert_eq!(b.recog.chain_count(), 2, "send chain and receive chain");
    // neither the QR secret nor the derived link key ever went over the
    // link (the payloads did: they are public material)
    assert_ne!(secret, pa.link_key);
    for m in &transcript {
        assert!(!m.windows(32).any(|w| w == secret), "secret on the link");
        assert!(
            !m.windows(32).any(|w| w == pa.link_key),
            "link key on the link"
        );
        let parsed: PairingMsg = decode(m).unwrap();
        if let PairingMsg::Accept { payload, .. } = parsed {
            assert_eq!(payload.keycard, b.card);
        }
    }
    // the ratchet works on the session pairing started, in both directions
    let now = 100;
    let cells = a
        .send_1to1(&b.id(), &text("after pairing", vec![]), now)
        .unwrap();
    let got = b.receive_1to1(&cells);
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].0, a.id());
    let Received::Post(p) = &got[0].1 else {
        panic!("post expected");
    };
    assert_eq!(text_of(p), "after pairing");
    let cells = b.send_1to1(&a.id(), &text("back", vec![]), now).unwrap();
    let got = a.receive_1to1(&cells);
    let Received::Post(p) = &got[0].1 else {
        panic!("post expected");
    };
    assert_eq!(text_of(p), "back");
    // the link key proves the contact on a later link, role-bound
    let hh2 = handshake(7, 8);
    let proof = contact_proof(&pa.link_key, Role::Central, &hh2);
    assert!(verify_contact_proof(
        &pb.link_key,
        Role::Central,
        &hh2,
        &proof
    ));
    assert!(!verify_contact_proof(
        &pb.link_key,
        Role::Peripheral,
        &hh2,
        &proof
    ));
}

#[test]
fn a_reflected_contact_proof_fails_the_pairing() {
    let (mut a, mut b) = (Node::new(3), Node::new(4));
    let hh = handshake(3, 4);
    let shown = QrCode::generate(qr_payload(&mut b, 1), &mut b.rng);
    let mine = qr_payload(&mut a, 1);
    let mut s = PairingSession::scanner(mine.clone(), shown.clone(), Role::Central, hh, &mut a.rng)
        .unwrap();
    let PairingStep::Send(offer) = s.step(&mut a.kv, &*be(), None, 0, &mut a.rng).unwrap() else {
        panic!("offer expected");
    };
    let PairingMsg::Offer { link_nonce, .. } = decode::<PairingMsg>(&offer).unwrap() else {
        panic!("offer");
    };
    // The peer knows the secret (it displayed it) but reflects a proof made
    // with the scanner's own role instead of its own.
    let key = link_key(&a.card, &b.card, &shown.secret, &link_nonce, &hh);
    let reflected = PairingMsg::Accept {
        payload: shown.payload.clone(),
        proof: contact_proof(&key, Role::Central, &hh),
    }
    .encode();
    assert_eq!(
        s.step(&mut a.kv, &*be(), Some(&reflected), 0, &mut a.rng)
            .err(),
        Some(PairingError::BadProof)
    );
    assert_eq!(a.count(ns::PEER_IDENTITIES), 0, "nothing pinned");
    assert_eq!(a.count(ns::SESSIONS), 0, "no session started");
    // the correct proof (peer role) is accepted
    let ok = PairingMsg::Accept {
        payload: shown.payload.clone(),
        proof: contact_proof(&key, Role::Peripheral, &hh),
    }
    .encode();
    assert!(matches!(
        s.step(&mut a.kv, &*be(), Some(&ok), 0, &mut a.rng),
        Ok(PairingStep::Send(_))
    ));
    assert_eq!(a.count(ns::PEER_IDENTITIES), 1);
    // and a proof under another handshake hash (another link) is refused by
    // the displayer side too
    let mut d = PairingSession::displayer(shown.clone(), Role::Peripheral, hh, &mut b.rng);
    let PairingStep::Send(_accept) = d
        .step(&mut b.kv, &*be(), Some(&offer), 0, &mut b.rng)
        .unwrap()
    else {
        panic!("accept expected");
    };
    let other_hh = handshake(9, 10);
    let wrong = PairingMsg::Prove {
        proof: contact_proof(&key, Role::Central, &other_hh),
        first: meshcore::crypto::backend::PairwiseCt {
            kind: PairwiseKind::PreKey,
            bytes: vec![0; 8],
        },
    }
    .encode();
    assert_eq!(
        d.step(&mut b.kv, &*be(), Some(&wrong), 0, &mut b.rng).err(),
        Some(PairingError::BadProof)
    );
    assert_eq!(b.count(ns::PEER_IDENTITIES), 0);
}

// ---- 1:1 recognition ----------------------------------------------------------------

#[test]
fn one_to_one_messages_with_recognition_seeds_both_ways() {
    let (mut a, mut b) = (Node::new(5), Node::new(6));
    a.connect(&mut b, 0);
    for i in 0..3u64 {
        let msg = format!("a->b {i}");
        let cells = a.send_1to1(&b.id(), &text(&msg, vec![]), i).unwrap();
        assert_eq!(cells.len(), 1);
        let got = b.receive_1to1(&cells);
        assert_eq!(got.len(), 1);
        let Received::Post(p) = &got[0].1 else {
            panic!("post");
        };
        assert_eq!(text_of(p), msg);
        let cells = b.send_1to1(&a.id(), &text("b->a", vec![]), i).unwrap();
        let got = a.receive_1to1(&cells);
        assert!(matches!(&got[0].1, Received::Post(p) if text_of(p) == "b->a"));
    }
    // a replayed cell is not recognised again
    let cells = a.send_1to1(&b.id(), &text("once", vec![]), 9).unwrap();
    b.receive_1to1(&cells);
    assert_eq!(b.recog.lookup(&cells[0].tag()), None);
    // re-seed on request: the control post goes under the old epoch, the
    // post under the new one, and the receiver registers the new epoch
    a.pm.request_reseed(&b.id());
    let cells = a
        .send_1to1(&b.id(), &text("under epoch 1", vec![]), 10)
        .unwrap();
    assert_eq!(cells.len(), 2, "seed object + post object");
    assert_eq!(a.pm.send_epoch(&b.id()), Some(1));
    let got = b.receive_1to1(&cells);
    assert_eq!(got.len(), 2);
    assert_eq!(got[0].1, Received::RecogSeed { epoch_id: 1 });
    assert!(matches!(&got[1].1, Received::Post(p) if text_of(p) == "under epoch 1"));
    assert_eq!(b.pm.recv_epochs(&a.id()), &[0, 1]);
    // the receiver keeps at most three epochs
    for e in 2..5u32 {
        a.pm.request_reseed(&b.id());
        let cells = a
            .send_1to1(&b.id(), &text("x", vec![]), 10 + u64::from(e))
            .unwrap();
        b.receive_1to1(&cells);
        assert_eq!(a.pm.send_epoch(&b.id()), Some(e));
    }
    assert_eq!(b.pm.recv_epochs(&a.id()), &[2, 3, 4]);
    // a multi-cell object (a 2 kB text is 5 cells) reassembles
    let long = "y".repeat(2000);
    let cells = a.send_1to1(&b.id(), &text(&long, vec![]), 30).unwrap();
    assert!(cells.len() >= 5, "{}", cells.len());
    let got = b.receive_1to1(&cells);
    assert_eq!(got.len(), 1);
    assert!(matches!(&got[0].1, Received::Post(p) if text_of(p) == long));
    // receipts travel the same way and are C1
    let r = meshcore::objects::chat_post::Receipt {
        conv: [1; 16],
        up_to_obj_seq: 7,
        gaps: vec![],
    };
    let rp = PostPlain::receipt(&r).unwrap();
    assert_eq!(rp.class(), Class::C1Control);
    let cells = b.send_1to1(&a.id(), &rp, 31).unwrap();
    let got = a.receive_1to1(&cells);
    assert!(matches!(&got[0].1, Received::Post(p) if p.receipt_body().unwrap() == r));
}

// ---- groups -----------------------------------------------------------------------

#[test]
fn three_node_group_rotation_and_posts() {
    let g = GroupId([7; 16]);
    let (mut a, mut b, mut c) = (Node::new(11), Node::new(12), Node::new(13));
    a.connect(&mut b, 0);
    a.connect(&mut c, 0);
    b.connect(&mut c, 0);
    for n in [&mut a, &mut b, &mut c] {
        n.group(g);
    }
    let members = [a.id(), b.id(), c.id()];
    assert!(a.group(g).needs_rotation());
    let rot = a.rotate(g, &members, 1);
    assert!(rot.cells.is_empty(), "no old chain: pairwise delivery");
    assert!(rot.needs_pairwise_delivery());
    assert_eq!(rot.recipients.len(), 2);
    assert!(rot.without_session.is_empty());
    assert_eq!(rot.bundle.len(), 8, "padded to 8");
    assert_eq!(rot.bundle.chain, rot.chain);
    assert_eq!(a.group(g).own_chain(), Some(rot.chain));
    assert!(!a.group(g).needs_rotation());
    // B and C accept the SKDM: the atomic identity check passes
    assert_eq!(b.accept_rotation(&a, g, &rot), Ok(rot.chain));
    assert_eq!(c.accept_rotation(&a, g, &rot), Ok(rot.chain));
    assert_eq!(b.group(g).member_chains().count(), 1);
    // a re-delivered bundle for the known chain fails to decrypt pairwise
    // (ratchet replay) and registers nothing new
    let before = b.recog.len();
    assert!(b.accept_rotation(&a, g, &rot).is_err());
    assert_eq!(b.recog.len(), before);
    // A posts text, B and C decrypt via the tag table hit
    let post = text(
        "hello group",
        vec![meshcore::crypto::hash::Hash32::from_bytes([1; 32])],
    );
    assert_eq!(post.class(), Class::C2Text);
    let cells = a.post(g, &post).unwrap();
    assert_eq!(cells.len(), 1);
    for n in [&mut b, &mut c] {
        let got = n.receive_group(g, &cells).unwrap();
        assert_eq!(got.len(), 1);
        let Inbound::Post {
            sender,
            chain,
            obj_seq,
            post: p,
        } = &got[0]
        else {
            panic!("post expected");
        };
        assert_eq!(*sender, a.id());
        assert_eq!(*chain, rot.chain);
        assert_eq!(*obj_seq, 0);
        assert_eq!(p, &post);
    }
    // the cell is not recognised twice
    assert_eq!(b.recog.lookup(&cells[0].tag()), None);
    // a second post has obj_seq 1; a long post spans cells
    let long = text(&"z".repeat(1500), vec![]);
    let cells = a.post(g, &long).unwrap();
    assert!(cells.len() >= 3);
    let got = c.receive_group(g, &cells).unwrap();
    assert!(matches!(&got[0], Inbound::Post { obj_seq: 1, post, .. } if post == &long));
    // B and C rotate too; then everyone reads everyone
    let rb = b.rotate(g, &members, 2);
    assert_eq!(a.accept_rotation(&b, g, &rb), Ok(rb.chain));
    assert_eq!(c.accept_rotation(&b, g, &rb), Ok(rb.chain));
    let cells = b.post(g, &text("from b", vec![])).unwrap();
    for n in [&mut a, &mut c] {
        let got = n.receive_group(g, &cells).unwrap();
        assert!(
            matches!(&got[0], Inbound::Post { sender, post, .. } if *sender == b.id() && text_of(post) == "from b")
        );
    }
}

#[test]
fn mismatched_expected_key_stores_nothing() {
    let g = GroupId([8; 16]);
    let (mut a, mut b) = (Node::new(21), Node::new(22));
    a.connect(&mut b, 0);
    a.group(g);
    b.group(g);
    let rot = a.rotate(g, &[a.id(), b.id()], 1);
    let before = snapshot(&b.kv);
    let tags_before = b.recog.len();
    // the roster says A's key is something else: refuse, write nothing
    let mut wrong = a.card.x25519;
    wrong[0] ^= 1;
    let Node {
        kv,
        recog,
        groups,
        rng,
        pm,
        ..
    } = &mut b;
    let err = groups
        .get_mut(&g)
        .unwrap()
        .accept_bundle(kv, &*be(), recog, pm, a.id(), wrong, &rot.bundle, rng)
        .unwrap_err();
    assert_eq!(err, GroupError::IdentityMismatch);
    assert_eq!(snapshot(&b.kv), before, "no store write");
    assert_eq!(b.recog.len(), tags_before, "no epoch registered");
    assert_eq!(b.group(g).member_chains().count(), 0);
    // the decrypted entry with the wrong expectation fails the same way
    let entry = rot.bundle.find_for(&b.id()).unwrap();
    let ct: meshcore::crypto::backend::PairwiseCt = decode(&entry.ct).unwrap();
    let mut probe = b.kv.clone();
    let plain = PairwiseBackend::decrypt(&*be(), &mut probe, &a.id(), &ct, &mut b.rng).unwrap();
    let probe_before = snapshot(&probe);
    let Node { recog, groups, .. } = &mut b;
    assert_eq!(
        groups
            .get_mut(&g)
            .unwrap()
            .accept_skdm(&mut probe, &*be(), recog, a.id(), wrong, &plain),
        Err(GroupError::IdentityMismatch)
    );
    assert_eq!(snapshot(&probe), probe_before);
    // a peer with no session at all is refused before any decrypt
    let mut d = Node::new(23);
    d.group(g);
    let d_before = snapshot(&d.kv);
    assert_eq!(d.accept_rotation(&a, g, &rot), Err(GroupError::NotForMe));
    assert_eq!(snapshot(&d.kv), d_before);
    // with the right key it is accepted and a post decrypts
    assert_eq!(b.accept_rotation(&a, g, &rot), Ok(rot.chain));
    let cells = a.post(g, &text("ok", vec![])).unwrap();
    let got = b.receive_group(g, &cells).unwrap();
    assert!(matches!(&got[0], Inbound::Post { post, .. } if text_of(post) == "ok"));
}

#[test]
fn a_redelivered_skdm_keeps_consumed_counters() {
    // A late copy of the same SKDM entry (the 1:1 delivery path hands the
    // decrypted post to `accept_skdm`) must not re-register the epoch: the
    // tag table would re-accept counters that were already consumed.
    let g = GroupId([11; 16]);
    let (mut a, mut b) = (Node::new(75), Node::new(76));
    a.connect(&mut b, 0);
    a.group(g);
    b.group(g);
    let rot = a.rotate(g, &[a.id(), b.id()], 1);
    let entry = rot.bundle.find_for(&b.id()).unwrap();
    let ct: meshcore::crypto::backend::PairwiseCt = decode(&entry.ct).unwrap();
    let plain = PairwiseBackend::decrypt(&*be(), &mut b.kv, &a.id(), &ct, &mut b.rng).unwrap();
    let expected = a.card.x25519;
    let a_id = a.id();
    let accept = |b: &mut Node| {
        let Node {
            kv, recog, groups, ..
        } = b;
        groups
            .get_mut(&g)
            .unwrap()
            .accept_skdm(kv, &*be(), recog, a_id, expected, &plain)
    };
    assert_eq!(accept(&mut b), Ok(rot.chain));
    let cells = a.post(g, &text("once", vec![])).unwrap();
    let got = b.receive_group(g, &cells).unwrap();
    assert!(matches!(&got[0], Inbound::Post { post, .. } if text_of(post) == "once"));
    assert_eq!(b.recog.lookup(&cells[0].tag()), None, "consumed");
    let tags_before = b.recog.len();
    assert_eq!(accept(&mut b), Ok(rot.chain), "re-delivery is accepted");
    assert_eq!(b.recog.lookup(&cells[0].tag()), None, "still consumed");
    assert_eq!(b.recog.len(), tags_before);
    assert_eq!(b.group(g).member_chains().next().unwrap().1.len(), 1);
    // the next post is still read
    let cells = a.post(g, &text("twice", vec![])).unwrap();
    let got = b.receive_group(g, &cells).unwrap();
    assert!(matches!(&got[0], Inbound::Post { post, .. } if text_of(post) == "twice"));
}

#[test]
fn rotation_after_a_removal_keeps_late_posts_readable() {
    let g = GroupId([9; 16]);
    let (mut a, mut b, mut c) = (Node::new(31), Node::new(32), Node::new(33));
    a.connect(&mut b, 0);
    a.connect(&mut c, 0);
    b.connect(&mut c, 0);
    for n in [&mut a, &mut b, &mut c] {
        n.group(g);
    }
    let all = [a.id(), b.id(), c.id()];
    let r1 = a.rotate(g, &all, 1);
    b.accept_rotation(&a, g, &r1).unwrap();
    c.accept_rotation(&a, g, &r1).unwrap();
    let late = a.post(g, &text("sent before the removal", vec![])).unwrap();
    // A learns that C was removed: the policy sets the flag; no post may go
    // out under the old key
    a.group(g).mark_rotation_pending();
    a.group(g).mark_rotation_pending();
    assert_eq!(
        a.post(g, &text("blocked", vec![])).unwrap_err(),
        GroupError::RotationPending
    );
    let r2 = a.rotate(g, &[a.id(), b.id()], 2);
    assert_ne!(r2.chain, r1.chain);
    assert!(!r2.cells.is_empty(), "bundle under the old chain's tags");
    assert!(!r2.needs_pairwise_delivery());
    assert_eq!(r2.recipients.len(), 1);
    assert!(r2.bundle.find_for(&c.id()).is_none(), "C gets no entry");
    assert!(!a.group(g).rotation_pending());
    // B reads the bundle from the old chain's cells and accepts it
    let got = b.receive_group(g, &r2.cells).unwrap();
    assert_eq!(got.len(), 1);
    let Inbound::Bundle { sender, bundle, .. } = &got[0] else {
        panic!("bundle expected");
    };
    assert_eq!(*sender, a.id());
    assert_eq!(bundle, &r2.bundle);
    let expected = a.card.x25519;
    let Node {
        kv,
        recog,
        groups,
        rng,
        pm,
        ..
    } = &mut b;
    assert_eq!(
        groups.get_mut(&g).unwrap().accept_bundle(
            kv,
            &*be(),
            recog,
            pm,
            a.id(),
            expected,
            bundle,
            rng
        ),
        Ok(r2.chain)
    );
    assert_eq!(b.group(g).member_chains().next().unwrap().1.len(), 2);
    // C also sees the bundle cells (it still holds the old chain) but has
    // no entry in it
    let got = c.receive_group(g, &r2.cells).unwrap();
    assert!(matches!(&got[0], Inbound::Bundle { .. }));
    assert_eq!(c.accept_rotation(&a, g, &r2), Err(GroupError::NotForMe));
    // A posts under the new chain: B reads it, C cannot even recognise it
    let cells = a.post(g, &text("after the removal", vec![])).unwrap();
    let got = b.receive_group(g, &cells).unwrap();
    assert!(
        matches!(&got[0], Inbound::Post { chain, post, .. } if *chain == r2.chain && text_of(post) == "after the removal")
    );
    assert_eq!(
        c.recog.lookup(&cells[0].tag()),
        None,
        "removed member: unrecognised"
    );
    // the late post under the old chain still decrypts at B (and at C)
    let got = b.receive_group(g, &late).unwrap();
    assert!(
        matches!(&got[0], Inbound::Post { chain, post, .. } if *chain == r1.chain && text_of(post) == "sent before the removal")
    );
    let got = c.receive_group(g, &late).unwrap();
    assert!(matches!(&got[0], Inbound::Post { chain, .. } if *chain == r1.chain));
}

// ---- sealed first flight ------------------------------------------------------------

#[test]
fn sealed_first_flight_round_trips_through_cells() {
    let (mut a, mut b) = (Node::new(41), Node::new(42));
    // co-members: pinned from the roster; B's bundle and envelope key came
    // inside group encryption
    a.pin(&b);
    b.pin(&a);
    let bundle_b =
        b.bm.issue(
            &mut b.kv,
            &*be(),
            IssueContext::InsideGroup(GroupId([1; 16])),
            0,
            &mut b.rng,
        )
        .unwrap();
    be().process_prekey_bundle(&mut a.kv, &b.id(), &bundle_b, 0, &mut a.rng)
        .unwrap();
    let post = text("first flight", vec![]);
    let ff =
        a.pm.build_first_flight(
            &mut a.kv,
            &*be(),
            &mut a.recog,
            &b.id(),
            &post,
            a.env.public(),
            1,
            &mut a.rng,
        )
        .unwrap();
    assert_eq!(ff.ct.kind, PairwiseKind::PreKey);
    assert_eq!(a.pm.send_epoch(&b.id()), Some(0));
    // seal to B's envelope key, fragment, random-tag cells
    let sealed = seal::seal(&b.env.public(), &encode(&ff), &mut a.rng).unwrap();
    let object = sealed.to_object();
    let frags = fragment(InnerKind::SealedFirst, 0, &object).unwrap();
    assert!(
        frags.len() >= 4,
        "a PQXDH first flight is ~4 cells: {}",
        frags.len()
    );
    let cells: Vec<Cell> = frags
        .iter()
        .map(|f| {
            ff_seal_cell(
                meshcore::wire::cell::VERSION,
                7,
                &b.env.public(),
                f,
                &mut a.rng,
            )
        })
        .collect();
    let tags: std::collections::BTreeSet<[u8; 16]> = cells.iter().map(|c| c.tag()).collect();
    assert_eq!(tags.len(), cells.len(), "every cell has its own random tag");
    // B: unrecognised by the tag table, opened by trial under its live keys
    let live = [b.env.clone(), EnvelopeKeyPair::generate(&mut b.rng)];
    let mut reassembler = Reassembler::default();
    let mut done = None;
    for c in &cells {
        assert_eq!(b.recog.lookup(&c.tag()), None);
        assert_eq!(a.recog.lookup(&c.tag()), None);
        let (idx, pt) = ff_open_cell(&live, c).expect("outer layer opens");
        assert_eq!(idx, 0);
        if let Some((kind, _, obj)) = reassembler.push(0, &pt).unwrap() {
            assert_eq!(kind, InnerKind::SealedFirst);
            done = Some(obj);
        }
    }
    let obj = done.expect("reassembled");
    let s = Sealed::from_object(&obj).unwrap();
    let (idx, payload) = seal::try_unseal(&live, &s.e_pub, &s.ct).expect("inner seal opens");
    assert_eq!(idx, 0);
    let flight: FirstFlight = decode(&payload).unwrap();
    assert_eq!(flight, ff);
    let (received, env_a) =
        b.pm.accept_first_flight(
            &mut b.kv,
            &*be(),
            &mut b.recog,
            &a.id(),
            &flight,
            &mut b.rng,
        )
        .unwrap();
    assert_eq!(env_a, a.env.public());
    assert!(matches!(&received, Received::Post(p) if text_of(p) == "first flight"));
    assert_eq!(b.pm.recv_epochs(&a.id()), &[0]);
    // a stranger without B's envelope key cannot open the outer layer
    let stranger = [EnvelopeKeyPair::generate(&mut a.rng)];
    assert!(cells.iter().all(|c| ff_open_cell(&stranger, c).is_none()));
    // A's next message is an ordinary recognised chain cell
    let cells = a.send_1to1(&b.id(), &text("second", vec![]), 2).unwrap();
    let got = b.receive_1to1(&cells);
    assert!(matches!(&got[0].1, Received::Post(p) if text_of(p) == "second"));
    // B's reply goes back sealed to A's envelope key with B's seed
    let reply =
        b.pm.build_first_flight(
            &mut b.kv,
            &*be(),
            &mut b.recog,
            &a.id(),
            &text("reply", vec![]),
            b.env.public(),
            3,
            &mut b.rng,
        )
        .unwrap();
    assert_eq!(reply.ct.kind, PairwiseKind::Normal);
    let sealed = seal::seal(&env_a, &encode(&reply), &mut b.rng).unwrap();
    let (_, payload) =
        seal::try_unseal(std::slice::from_ref(&a.env), &sealed.e_pub, &sealed.ct).unwrap();
    let flight: FirstFlight = decode(&payload).unwrap();
    let (received, _) =
        a.pm.accept_first_flight(
            &mut a.kv,
            &*be(),
            &mut a.recog,
            &b.id(),
            &flight,
            &mut a.rng,
        )
        .unwrap();
    assert!(matches!(&received, Received::Post(p) if text_of(p) == "reply"));
    let cells = b.send_1to1(&a.id(), &text("third", vec![]), 4).unwrap();
    let got = a.receive_1to1(&cells);
    assert!(matches!(&got[0].1, Received::Post(p) if text_of(p) == "third"));
    assert_eq!(a.pm.state(&b.id()), SessionState::Ready);
    assert_eq!(b.pm.state(&a.id()), SessionState::Ready);
}

// ---- bundles ------------------------------------------------------------------------

#[test]
fn bundle_issue_contexts() {
    let mut a = Node::new(51);
    let ok = [
        (IssueContext::QrScan, 1u8),
        (IssueContext::DirectEncounter, 10),
        (IssueContext::InsideGroup(GroupId([2; 16])), 0),
    ];
    for (ctx, one_time) in ok {
        let bytes =
            a.bm.issue(&mut a.kv, &*be(), ctx, one_time, &mut a.rng)
                .unwrap_or_else(|e| panic!("{ctx:?}: {e}"));
        let w: PreKeyBundleWire = decode(&bytes).unwrap();
        assert_eq!(w.one_time.len(), usize::from(one_time), "{ctx:?}");
    }
    let before = snapshot(&a.kv);
    assert_eq!(
        a.bm.issue(&mut a.kv, &*be(), IssueContext::Gossip, 0, &mut a.rng),
        Err(BundleError::ContextNotAllowed)
    );
    assert_eq!(
        a.bm.issue(
            &mut a.kv,
            &*be(),
            IssueContext::InsideGroup(GroupId([2; 16])),
            3,
            &mut a.rng
        ),
        Err(BundleError::OneTimeNotAllowed)
    );
    assert_eq!(snapshot(&a.kv), before, "refusals create no prekeys");
    // the replay cache
    let k = [5u8; 33];
    assert!(!a.bm.seen_base_key(&k));
    assert!(a.bm.seen_base_key(&k));
}

#[test]
fn spk_rotation_and_retire_timing_with_a_manual_clock() {
    let mut a = Node::new(52);
    let clock = ManualClock::new(0);
    clock.set_wall_offset_ms(1_700_000_000_000);
    let w0 = clock.wall_for_display();
    a.bm = BundleManager::new(w0);
    let retire_be = LibsignalBackend::new();
    // the lazily created generation 0
    let _ =
        a.bm.issue(&mut a.kv, &*be(), IssueContext::QrScan, 0, &mut a.rng)
            .unwrap();
    assert_eq!(a.count(ns::SIGNED_PREKEYS), 1);
    // no rotation before 24 h of powered-on time
    clock.set(ms::DAY - 1);
    assert_eq!(
        a.bm.maybe_rotate(&mut a.kv, &*be(), &clock, &mut a.rng),
        Ok(false)
    );
    clock.set(ms::DAY);
    assert_eq!(
        a.bm.maybe_rotate(&mut a.kv, &*be(), &clock, &mut a.rng),
        Ok(true)
    );
    assert_eq!(
        a.bm.maybe_rotate(&mut a.kv, &*be(), &clock, &mut a.rng),
        Ok(false)
    );
    assert_eq!(a.count(ns::SIGNED_PREKEYS), 2);
    assert_eq!(a.bm.last_spk_rotation_mono(), ms::DAY);
    // wall time alone does nothing: 30 days of wall, 25 h powered-on
    clock.set(ms::DAY + ms::HOUR);
    clock.set_wall_offset_ms(1_700_000_000_000 + 30 * ms::DAY as i64);
    assert_eq!(
        a.bm.maybe_rotate(&mut a.kv, &*be(), &clock, &mut a.rng),
        Ok(false)
    );
    assert_eq!(a.bm.retire(&mut a.kv, &retire_be, &clock), 0);
    assert_eq!(a.count(ns::SIGNED_PREKEYS), 2);
    // 7 d wall AND 48 h powered-on since generation 0: it goes
    clock.set(2 * ms::DAY);
    assert_eq!(a.bm.retire(&mut a.kv, &retire_be, &clock), 1);
    assert_eq!(a.count(ns::SIGNED_PREKEYS), 1);
    assert_eq!(a.bm.records().len(), 1);
    assert_eq!(a.bm.records()[0].created_mono, ms::DAY);
    // another rotation at 48 h; with the wall clock stuck, the 24 h
    // generation goes only after 7 d of powered-on time
    assert_eq!(
        a.bm.maybe_rotate(&mut a.kv, &*be(), &clock, &mut a.rng),
        Ok(true)
    );
    assert_eq!(a.count(ns::SIGNED_PREKEYS), 2);
    clock.set_wall_offset_ms(1_700_000_000_000 - 100 * ms::DAY as i64);
    clock.set(8 * ms::DAY - 1);
    assert_eq!(a.bm.retire(&mut a.kv, &retire_be, &clock), 0);
    clock.set(8 * ms::DAY);
    assert_eq!(a.bm.retire(&mut a.kv, &retire_be, &clock), 1);
    assert_eq!(
        a.count(ns::SIGNED_PREKEYS),
        1,
        "only the current SPK remains"
    );
    assert_eq!(a.bm.records().len(), 1);
    assert_eq!(a.bm.records()[0].created_mono, 2 * ms::DAY);
    // a bundle issued from the current generation still starts a session
    let bundle =
        a.bm.issue(
            &mut a.kv,
            &*be(),
            IssueContext::DirectEncounter,
            1,
            &mut a.rng,
        )
        .unwrap();
    let mut c = Node::new(53);
    c.pin(&a);
    a.pin(&c);
    be().process_prekey_bundle(&mut c.kv, &a.id(), &bundle, 9 * ms::DAY, &mut c.rng)
        .unwrap();
    let m =
        c.pm.encrypt_to(
            &mut c.kv,
            &*be(),
            &a.id(),
            &text("still works", vec![]).encode(),
            9 * ms::DAY,
            &mut c.rng,
        )
        .unwrap();
    let got =
        a.pm.decrypt_from(&mut a.kv, &*be(), &mut a.recog, &c.id(), &m, &mut a.rng)
            .unwrap();
    assert!(matches!(got, Received::Post(p) if text_of(&p) == "still works"));
    // the manager state persists and restores
    let bytes = encode(&a.bm);
    assert_eq!(decode::<BundleManager>(&bytes), Ok(a.bm.clone()));
}

// ---- crossed initiation and needs-their-key ----------------------------------------

#[test]
fn crossed_initiation_converges_and_missing_session_reports_needs_key() {
    let (mut a, mut b) = (Node::new(61), Node::new(62));
    a.pin(&b);
    b.pin(&a);
    let ba =
        a.bm.issue(
            &mut a.kv,
            &*be(),
            IssueContext::DirectEncounter,
            1,
            &mut a.rng,
        )
        .unwrap();
    let bb =
        b.bm.issue(
            &mut b.kv,
            &*be(),
            IssueContext::DirectEncounter,
            1,
            &mut b.rng,
        )
        .unwrap();
    // before any bundle: "Needs their key"
    let mut n = Node::new(63);
    assert_eq!(
        n.pm.encrypt_to(&mut n.kv, &*be(), &a.id(), b"x", 0, &mut n.rng),
        Err(PairwiseError::NeedsTheirKey)
    );
    assert_eq!(n.pm.state(&a.id()), SessionState::NeedsTheirKey);
    be().process_prekey_bundle(&mut a.kv, &b.id(), &bb, 0, &mut a.rng)
        .unwrap();
    be().process_prekey_bundle(&mut b.kv, &a.id(), &ba, 0, &mut b.rng)
        .unwrap();
    let pa = text("A first", vec![]).encode();
    let pb = text("B first", vec![]).encode();
    let ma =
        a.pm.encrypt_to(&mut a.kv, &*be(), &b.id(), &pa, 1, &mut a.rng)
            .unwrap();
    let mb =
        b.pm.encrypt_to(&mut b.kv, &*be(), &a.id(), &pb, 1, &mut b.rng)
            .unwrap();
    assert_eq!(ma.kind, PairwiseKind::PreKey);
    assert_eq!(a.pm.state(&b.id()), SessionState::Ready);
    let got =
        b.pm.decrypt_from(&mut b.kv, &*be(), &mut b.recog, &a.id(), &ma, &mut b.rng)
            .unwrap();
    assert!(matches!(got, Received::Post(p) if text_of(&p) == "A first"));
    assert_eq!(b.pm.state(&a.id()), SessionState::Crossed);
    let got =
        a.pm.decrypt_from(&mut a.kv, &*be(), &mut a.recog, &b.id(), &mb, &mut a.rng)
            .unwrap();
    assert!(matches!(got, Received::Post(p) if text_of(&p) == "B first"));
    assert_eq!(a.pm.state(&b.id()), SessionState::Crossed);
    // the next exchange converges
    let m =
        a.pm.encrypt_to(&mut a.kv, &*be(), &b.id(), &pa, 2, &mut a.rng)
            .unwrap();
    b.pm.decrypt_from(&mut b.kv, &*be(), &mut b.recog, &a.id(), &m, &mut b.rng)
        .unwrap();
    let m =
        b.pm.encrypt_to(&mut b.kv, &*be(), &a.id(), &pb, 3, &mut b.rng)
            .unwrap();
    assert_eq!(m.kind, PairwiseKind::Normal);
    a.pm.decrypt_from(&mut a.kv, &*be(), &mut a.recog, &b.id(), &m, &mut a.rng)
        .unwrap();
    assert_eq!(a.pm.state(&b.id()), SessionState::Ready);
    assert_eq!(b.pm.state(&a.id()), SessionState::Ready);
    // repeated junk from the peer flips the session to "Needs their key"
    let junk = meshcore::crypto::backend::PairwiseCt {
        kind: PairwiseKind::Normal,
        bytes: vec![0x33; 40],
    };
    for _ in 0..meshcore::pairwise::session::DECRYPT_FAILURES_NEEDS_KEY_ASM {
        assert_eq!(
            a.pm.decrypt_from(&mut a.kv, &*be(), &mut a.recog, &b.id(), &junk, &mut a.rng),
            Err(PairwiseError::Decrypt)
        );
    }
    assert_eq!(a.pm.state(&b.id()), SessionState::NeedsTheirKey);
}

#[test]
fn a_failed_send_after_a_due_reseed_does_not_advance_the_epoch() {
    // The re-seed control post goes out with the post that triggered it; if
    // that post cannot be sent, the new epoch must not start, or the peer
    // never learns the seed and every later cell is unrecognisable.
    let (mut a, mut b) = (Node::new(71), Node::new(72));
    a.connect(&mut b, 0);
    a.pm.request_reseed(&b.id());
    let bad = PostPlain {
        kind: 0x99,
        roster_heads: vec![],
        body: vec![],
    };
    assert!(matches!(
        a.send_1to1(&b.id(), &bad, 1),
        Err(PairwiseError::Codec(_))
    ));
    assert_eq!(a.pm.send_epoch(&b.id()), Some(0), "epoch not advanced");
    // the next good post carries the seed and both objects are read
    let cells = a.send_1to1(&b.id(), &text("ok", vec![]), 2).unwrap();
    assert_eq!(cells.len(), 2, "seed object + post object");
    assert_eq!(a.pm.send_epoch(&b.id()), Some(1));
    let got = b.receive_1to1(&cells);
    assert_eq!(got.len(), 2);
    assert_eq!(got[0].1, Received::RecogSeed { epoch_id: 1 });
    assert!(matches!(&got[1].1, Received::Post(p) if text_of(p) == "ok"));
}

#[test]
fn a_rotation_that_fails_to_cut_cells_keeps_the_own_chain() {
    let g = GroupId([10; 16]);
    let (mut a, mut b) = (Node::new(73), Node::new(74));
    a.connect(&mut b, 0);
    a.group(g);
    b.group(g);
    let r1 = a.rotate(g, &[a.id(), b.id()], 1);
    b.accept_rotation(&a, g, &r1).unwrap();
    // the own chain's sender state is missing from the tag table (a restore
    // that did not replay it): cutting the bundle under the old chain fails
    let key = meshcore::groups::group::group_chain_key(&g, &a.id(), r1.chain);
    assert!(a.recog.forget_chain(key));
    a.group(g).mark_rotation_pending();
    let Node {
        kv,
        recog,
        groups,
        rng,
        pm,
        ..
    } = &mut a;
    let err = groups
        .get_mut(&g)
        .unwrap()
        .rotate(kv, &*be(), recog, pm, &[b.id()], 2, rng)
        .unwrap_err();
    assert!(matches!(err, GroupError::Recog(_)), "{err:?}");
    assert_eq!(
        a.group(g).own_chain(),
        Some(r1.chain),
        "the state still names the chain the (rolled-back) backend holds"
    );
    assert!(a.group(g).rotation_pending(), "the flag is not cleared");
}

#[test]
fn a_peer_that_lost_its_state_is_recognised_again_after_a_new_first_flight() {
    let (mut a, mut b) = (Node::new(42), Node::new(43));
    a.pin(&b);
    b.pin(&a);
    let bundle_a =
        a.bm.issue(
            &mut a.kv,
            &*be(),
            IssueContext::InsideGroup(GroupId([1; 16])),
            0,
            &mut a.rng,
        )
        .unwrap();
    let flight = |b: &mut Node, a: &Node, bundle_a: &[u8], msg: &str, now: u64| {
        be().process_prekey_bundle(&mut b.kv, &a.id(), bundle_a, now, &mut b.rng)
            .unwrap();
        b.pm.build_first_flight(
            &mut b.kv,
            &*be(),
            &mut b.recog,
            &a.id(),
            &text(msg, vec![]),
            b.env.public(),
            now,
            &mut b.rng,
        )
        .unwrap()
    };
    let ff1 = flight(&mut b, &a, &bundle_a, "first", 1);
    a.pm.accept_first_flight(&mut a.kv, &*be(), &mut a.recog, &b.id(), &ff1, &mut a.rng)
        .unwrap();
    let cells = b.send_1to1(&a.id(), &text("one", vec![]), 2).unwrap();
    assert!(matches!(&a.receive_1to1(&cells)[0].1, Received::Post(p) if text_of(p) == "one"));
    // B's phone is wiped and restored with the same identity: a fresh node
    // with the same keys, no session, no epochs
    let mut b2 = Node::new(43);
    assert_eq!(b2.id(), b.id());
    b2.rng = ChaCha20Rng::seed_from_u64(4343);
    b2.pin(&a);
    // every bundle is single-use (its Kyber prekey is one-time): A hands
    // out a fresh one inside the group
    let bundle_a2 =
        a.bm.issue(
            &mut a.kv,
            &*be(),
            IssueContext::InsideGroup(GroupId([1; 16])),
            0,
            &mut a.rng,
        )
        .unwrap();
    let ff2 = flight(&mut b2, &a, &bundle_a2, "again", 3);
    assert_ne!(ff2.recog_seed, ff1.recog_seed);
    let (got, _) =
        a.pm.accept_first_flight(&mut a.kv, &*be(), &mut a.recog, &b2.id(), &ff2, &mut a.rng)
            .unwrap();
    assert!(matches!(&got, Received::Post(p) if text_of(p) == "again"));
    // the new epoch-0 seed is the one that recognises B's cells now
    let cells = b2.send_1to1(&a.id(), &text("two", vec![]), 4).unwrap();
    let got = a.receive_1to1(&cells);
    assert!(matches!(&got[0].1, Received::Post(p) if text_of(p) == "two"));
}

// ---- late SKDMs, first-message rotations, padding ---------------------------------

#[test]
fn a_member_reached_after_the_window_passed_still_reads_the_chain() {
    // B has no session with A when A rotates: it gets no entry. A then
    // issues more than one tag window of cells before B's session exists.
    // The SKDM A sends B afterwards carries A's current counter, so B opens
    // its window there and reads A's next cell; the SKDM at counter 0
    // would have left B unable to recognise anything.
    let g = GroupId([12; 16]);
    let (mut a, mut b) = (Node::new(81), Node::new(82));
    a.group(g);
    b.group(g);
    let rot = a.rotate(g, &[a.id(), b.id()], 1);
    assert_eq!(rot.without_session, vec![b.id()], "no session yet");
    assert!(rot.recipients.is_empty());
    assert_eq!(a.pm.state(&b.id()), SessionState::NeedsTheirKey);
    assert_eq!(
        a.skdm_entry_for(g, &b.id(), 1).unwrap_err(),
        GroupError::Pairwise(PairwiseError::NeedsTheirKey)
    );
    // 120 cells go out before B can be reached
    let mut early = Vec::new();
    for i in 0..120 {
        let cells = a.post(g, &text(&format!("early {i}"), vec![])).unwrap();
        assert_eq!(cells.len(), 1);
        early.extend(cells);
    }
    let key = meshcore::groups::group::group_chain_key(&g, &a.id(), rot.chain);
    assert_eq!(a.recog.sender_next_ctr(key), Some(120));
    // now the session exists (bundle inside the group, first message, reply)
    a.connect(&mut b, 2);
    let entry = a
        .skdm_entry_for(g, &b.id(), 3)
        .expect("entry for the late member");
    assert_eq!(
        entry.hint,
        meshcore::objects::skdm_bundle::hint(&b.id(), rot.chain)
    );
    let ct: meshcore::crypto::backend::PairwiseCt = decode(&entry.ct).unwrap();
    assert_eq!(ct.kind, PairwiseKind::Normal, "the session already exists");
    assert_eq!(b.accept_entry(&a, g, &entry), Ok(rot.chain));
    assert_eq!(b.group(g).member_chains().count(), 1);
    // the window starts at 120: the early cells are not recognised, the
    // next one is and decrypts (the backend skips the 120 message keys)
    assert!(early.iter().all(|c| b.recog.lookup(&c.tag()).is_none()));
    let w = b
        .recog
        .window(key, meshcore::wire::recog::EpochId(rot.chain.0))
        .unwrap();
    assert_eq!(w.base, 120);
    let cells = a.post(g, &text("the 121st", vec![])).unwrap();
    assert!(b.recog.lookup(&cells[0].tag()).is_some());
    let got = b.receive_group(g, &cells).unwrap();
    assert!(
        matches!(&got[0], Inbound::Post { sender, chain, obj_seq: 120, post } if *sender == a.id() && *chain == rot.chain && text_of(post) == "the 121st")
    );
    let cells = a.post(g, &text("the 122nd", vec![])).unwrap();
    let got = b.receive_group(g, &cells).unwrap();
    assert!(matches!(&got[0], Inbound::Post { post, .. } if text_of(post) == "the 122nd"));
}

#[test]
fn a_resent_skdm_after_two_hundred_missed_cells_moves_the_window() {
    // B accepted A's SKDM at counter 0 and read one post; then it missed
    // 200 cells. A re-sends the current chain's SKDM with its counter: the
    // window moves there (the same seed within the window would have been
    // ignored), without re-accepting the counter B already consumed.
    let g = GroupId([13; 16]);
    let (mut a, mut b) = (Node::new(83), Node::new(84));
    a.connect(&mut b, 0);
    a.group(g);
    b.group(g);
    let rot = a.rotate(g, &[a.id(), b.id()], 1);
    assert_eq!(b.accept_rotation(&a, g, &rot), Ok(rot.chain));
    let first = a.post(g, &text("read", vec![])).unwrap();
    let got = b.receive_group(g, &first).unwrap();
    assert!(matches!(&got[0], Inbound::Post { post, .. } if text_of(post) == "read"));
    let key = meshcore::groups::group::group_chain_key(&g, &a.id(), rot.chain);
    let epoch_id = meshcore::wire::recog::EpochId(rot.chain.0);
    // 200 cells B never sees
    for i in 0..200 {
        a.post(g, &text(&format!("missed {i}"), vec![])).unwrap();
    }
    let unread = a.post(g, &text("beyond the window", vec![])).unwrap();
    assert_eq!(
        b.recog.lookup(&unread[0].tag()),
        None,
        "window still near 0"
    );
    assert_eq!(a.recog.sender_next_ctr(key), Some(202));
    // the re-sent entry carries counter 202
    let entry = a.skdm_entry_for(g, &b.id(), 5).unwrap();
    let tags_before = b.recog.len();
    assert_eq!(b.accept_entry(&a, g, &entry), Ok(rot.chain));
    assert_eq!(b.recog.len(), tags_before, "one window replaced another");
    assert_eq!(b.recog.window(key, epoch_id).unwrap().base, 202);
    assert_eq!(
        b.recog.lookup(&first[0].tag()),
        None,
        "consumed stays consumed"
    );
    assert_eq!(
        b.recog.lookup(&unread[0].tag()),
        None,
        "below the new window"
    );
    assert_eq!(b.group(g).member_chains().next().unwrap().1.len(), 1);
    let cells = a.post(g, &text("after the re-send", vec![])).unwrap();
    let got = b.receive_group(g, &cells).unwrap();
    assert!(
        matches!(&got[0], Inbound::Post { obj_seq: 202, post, .. } if text_of(post) == "after the re-send")
    );
    // a second copy of the same entry (counter 202, window now at 202)
    // registers nothing: the just-consumed 202 is not re-accepted
    let ct: meshcore::crypto::backend::PairwiseCt = decode(&entry.ct).unwrap();
    let plain = PairwiseBackend::decrypt(&*be(), &mut b.kv.clone(), &a.id(), &ct, &mut b.rng);
    assert!(
        plain.is_err(),
        "the ratchet refuses the replayed ciphertext"
    );
    let entry2 = a.skdm_entry_for(g, &b.id(), 6).unwrap();
    assert_eq!(b.accept_entry(&a, g, &entry2), Ok(rot.chain));
    assert_eq!(
        b.recog.lookup(&cells[0].tag()),
        None,
        "203 > base: window kept"
    );
    assert_eq!(b.recog.window(key, epoch_id).unwrap().base, 203);
}

#[test]
fn a_rotation_that_is_the_first_message_leaves_the_session_ready() {
    // A holds B's bundle (from inside another group) but never wrote to B:
    // the SKDM entry is A's first message, a PreKey message. Because it
    // goes through the pairwise manager, A's view of the session is Ready
    // (not still "Needs their key") and B's first reply decrypts.
    let g = GroupId([14; 16]);
    let (mut a, mut b) = (Node::new(85), Node::new(86));
    a.pin(&b);
    b.pin(&a);
    let bundle_b =
        b.bm.issue(
            &mut b.kv,
            &*be(),
            IssueContext::InsideGroup(g),
            0,
            &mut b.rng,
        )
        .unwrap();
    be().process_prekey_bundle(&mut a.kv, &b.id(), &bundle_b, 0, &mut a.rng)
        .unwrap();
    assert_eq!(
        a.pm.state(&b.id()),
        SessionState::NeedsTheirKey,
        "nothing sent yet"
    );
    a.group(g);
    b.group(g);
    let rot = a.rotate(g, &[a.id(), b.id()], 1);
    assert!(rot.without_session.is_empty());
    assert_eq!(rot.recipients.len(), 1);
    assert_eq!(rot.recipients[0].0, b.id());
    assert_eq!(rot.recipients[0].1.kind, PairwiseKind::PreKey);
    assert_eq!(
        a.pm.state(&b.id()),
        SessionState::Ready,
        "the manager saw the PreKey"
    );
    assert_eq!(a.pm.contacts().count(), 1);
    // every bundle entry has the PreKey entry's width, dummies included
    let prekey_len = rot.recipients[0].1.encode().len();
    assert_eq!(usize::from(rot.bundle.width), prekey_len);
    let bytes = encode(&rot.bundle);
    assert_eq!(bytes.len(), rot.bundle.wire_len());
    assert_eq!(bytes.len(), 7 + 8 * (10 + prekey_len));
    assert!(rot.bundle.entries.iter().all(|e| e.ct.len() == prekey_len));
    assert!(prekey_len > 1_000, "PQXDH first message: {prekey_len}");
    // B accepts through its manager: got_prekey, Ready
    assert_eq!(b.accept_rotation(&a, g, &rot), Ok(rot.chain));
    assert_eq!(b.pm.state(&a.id()), SessionState::Ready);
    // B's first reply is a Normal message that A decrypts
    let reply =
        b.pm.encrypt_to(
            &mut b.kv,
            &*be(),
            &a.id(),
            &text("got your key", vec![]).encode(),
            2,
            &mut b.rng,
        )
        .unwrap();
    assert_eq!(reply.kind, PairwiseKind::Normal);
    let got =
        a.pm.decrypt_from(&mut a.kv, &*be(), &mut a.recog, &b.id(), &reply, &mut a.rng)
            .unwrap();
    assert!(matches!(got, Received::Post(p) if text_of(&p) == "got your key"));
    assert_eq!(a.pm.state(&b.id()), SessionState::Ready);
    // and A's posts read at B
    let cells = a.post(g, &text("first post", vec![])).unwrap();
    let got = b.receive_group(g, &cells).unwrap();
    assert!(matches!(&got[0], Inbound::Post { post, .. } if text_of(post) == "first post"));
    // the rotation's plaintext is what B's entry decrypted to, with counter 0
    let plain_post: PostPlain = decode(&rot.entry_plain).unwrap();
    assert_eq!(plain_post.kind, kind::POST_SKDM_BUNDLE);
    let payload: meshcore::objects::skdm_bundle::SkdmPayload = decode(&plain_post.body).unwrap();
    assert_eq!(payload.group, g);
    assert_eq!(payload.next_ctr, 0);
    assert!(format!("{rot:?}").contains("redacted"));
}

#[test]
fn post_kinds_and_bundle_kind_constants_match_the_design() {
    assert_eq!(kind::POST_TEXT, 0x30);
    assert_eq!(kind::POST_RECEIPT, 0x35);
    assert_eq!(kind::POST_SKDM_BUNDLE, 0x36);
    assert_eq!(kind::POST_RECOG_SEED, 0x37);
    let p = PostPlain::new(kind::POST_TASK_OP, vec![], vec![1]).unwrap();
    assert_eq!(p.class(), Class::C1Control);
}
