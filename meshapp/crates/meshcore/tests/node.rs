//! Node integration tests over an in-memory byte pipe (docs/DESIGN.md §10
//! "storage atomicity (kill between decrypt and commit via a fault-injection
//! hook → replay yields exactly one delivery)", S0/S1/S3/S4/S-flood in the
//! small; BRIEF_v2 §5.3, §5.4, §5.9; DECISIONS D3, D4, D6). Default features
//! (the real backend); seeds from `rand_chacha`; no OS clock anywhere, and
//! every test ends with `wall_reads() == 0` on every node.

#![cfg(feature = "backend-libsignal")]

use std::collections::BTreeSet;

use meshcore::clock::ms;
use meshcore::crypto::backend::GroupId;
use meshcore::crypto::ed25519::IdentityKey;
use meshcore::crypto::hash::Hash32;
use meshcore::node::{
    DeliveryState, Event, ImportBadge, ImportedContact, Node, NodeConfig, NodeError, ObjId,
};
use meshcore::objects::roster_op::MemberEntry;
use meshcore::pairwise::bundles::IssueContext;
use meshcore::pairwise::session::SessionState;
use meshcore::relay::buffers::BufferPolicy;
use meshcore::storage::Db;
use meshcore::tasks::state::TaskState;
use meshcore::transport::link::{LinkAuth, LinkId, Role};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

const CHUNK: usize = 4096;
/// Simulated time between pump rounds: budgets refill and throttle holds
/// expire (no OS clock: the test is the clock).
const STEP_MS: u64 = 2_000;

fn db(seed: u8) -> Db {
    Db::open_in_memory([seed; 32], [0x77 ^ seed; 32]).expect("db")
}

fn member_cfg(seed: u8, name: &str) -> NodeConfig {
    NodeConfig::member([seed; 32], name)
}

fn open_member(seed: u8, name: &str, now: u64) -> Node {
    Node::open(
        db(seed),
        member_cfg(seed, name),
        now,
        ChaCha20Rng::seed_from_u64(u64::from(seed)),
    )
    .expect("open member")
}

fn open_station(seed: u8, now: u64) -> Node {
    Node::open(
        db(seed),
        NodeConfig::station(),
        now,
        ChaCha20Rng::seed_from_u64(u64::from(seed)),
    )
    .expect("open station")
}

/// A set of nodes joined by byte pipes; the test is the radio.
struct Net {
    nodes: Vec<Node>,
    /// `(i, link at i, j, link at j, up)`.
    pipes: Vec<(usize, LinkId, usize, LinkId, bool)>,
    events: Vec<Vec<Event>>,
    next_link: u32,
    /// The simulated monotonic clock every call is stamped with.
    now: u64,
}

impl Net {
    fn new(nodes: Vec<Node>, now: u64) -> Self {
        let n = nodes.len();
        Net {
            nodes,
            pipes: Vec::new(),
            events: vec![Vec::new(); n],
            next_link: 1,
            now,
        }
    }

    fn now(&self) -> u64 {
        self.now
    }

    fn advance(&mut self, ms: u64) -> u64 {
        self.now += ms;
        self.now
    }

    fn id(&self, i: usize) -> IdentityKey {
        self.nodes[i].identity().expect("member")
    }

    /// Mutual provisioning (what a QR pairing or the sim yields): bundles,
    /// envelope keys, a shared link key, badge "Verified in person".
    fn befriend(&mut self, i: usize, j: usize) {
        let now = self.now;
        let mut link_key = [0u8; 32];
        link_key[0] = i as u8;
        link_key[1] = j as u8;
        link_key[2] = 0xAB;
        let import_i = self.export_contact(i, now, link_key);
        let import_j = self.export_contact(j, now, link_key);
        self.nodes[j].import_contact(import_i, now).expect("import");
        self.nodes[i].import_contact(import_j, now).expect("import");
    }

    fn export_contact(&mut self, i: usize, now: u64, link_key: [u8; 32]) -> ImportedContact {
        let n = &mut self.nodes[i];
        ImportedContact {
            keycard: n.my_keycard().unwrap(),
            display_name: n.display_name().unwrap().to_owned(),
            bundle: n.my_bundle(IssueContext::DirectEncounter, now).unwrap(),
            link_key: Some(link_key),
            envelope_pub: n.my_envelope_pub().unwrap(),
            badge: ImportBadge::VerifiedInPerson,
        }
    }

    /// Brings a link up between `i` (central) and `j` (peripheral), with
    /// identity hints when both are members.
    fn connect(&mut self, i: usize, j: usize) -> usize {
        let now = self.now;
        let li = LinkId(self.next_link);
        let lj = LinkId(self.next_link + 1);
        self.next_link += 2;
        let hint_i = self.nodes[j].identity();
        let hint_j = self.nodes[i].identity();
        self.nodes[i]
            .link_up(li, Role::Central, hint_i, now)
            .unwrap();
        self.nodes[j]
            .link_up(lj, Role::Peripheral, hint_j, now)
            .unwrap();
        self.pipes.push((i, li, j, lj, true));
        self.pipes.len() - 1
    }

    fn disconnect(&mut self, pipe: usize) {
        let now = self.now;
        let (i, li, j, lj, up) = self.pipes[pipe];
        if up {
            self.nodes[i].link_down(li, now);
            self.nodes[j].link_down(lj, now);
            self.pipes[pipe].4 = false;
        }
    }

    fn collect(&mut self) {
        for (i, n) in self.nodes.iter_mut().enumerate() {
            self.events[i].extend(n.poll_events());
        }
    }

    /// Moves bytes over every pipe until nothing flows for a while,
    /// ticking and advancing the clock as it goes (relay budgets refill
    /// with simulated time).
    fn pump(&mut self) {
        let mut idle = 0;
        for _ in 0..4_000 {
            let now = self.advance(STEP_MS);
            let mut moved = false;
            for p in 0..self.pipes.len() {
                let (i, li, j, lj, up) = self.pipes[p];
                if !up {
                    continue;
                }
                let bytes = self.nodes[i].poll_outbound(li, CHUNK, now);
                if !bytes.is_empty() {
                    moved = true;
                    if let Err(NodeError::LinkDead(_)) = self.nodes[j].feed(lj, &bytes, now) {
                        panic!("link died at node {j}");
                    }
                }
                let bytes = self.nodes[j].poll_outbound(lj, CHUNK, now);
                if !bytes.is_empty() {
                    moved = true;
                    if let Err(NodeError::LinkDead(_)) = self.nodes[i].feed(li, &bytes, now) {
                        panic!("link died at node {i}");
                    }
                }
            }
            for (k, n) in self.nodes.iter_mut().enumerate() {
                self.events[k].extend(n.tick(now));
            }
            self.collect();
            if moved {
                idle = 0;
            } else {
                idle += 1;
                if idle >= 40 {
                    return;
                }
            }
        }
        panic!("pump did not settle");
    }

    /// One pump round: every pipe both ways, then a tick everywhere.
    fn round(&mut self) -> bool {
        let now = self.advance(STEP_MS);
        let mut moved = false;
        for p in 0..self.pipes.len() {
            let (i, li, j, lj, up) = self.pipes[p];
            if !up {
                continue;
            }
            let bytes = self.nodes[i].poll_outbound(li, CHUNK, now);
            if !bytes.is_empty() {
                moved = true;
                if let Err(NodeError::LinkDead(_)) = self.nodes[j].feed(lj, &bytes, now) {
                    panic!("link died at node {j}");
                }
            }
            let bytes = self.nodes[j].poll_outbound(lj, CHUNK, now);
            if !bytes.is_empty() {
                moved = true;
                if let Err(NodeError::LinkDead(_)) = self.nodes[i].feed(li, &bytes, now) {
                    panic!("link died at node {i}");
                }
            }
        }
        for (k, n) in self.nodes.iter_mut().enumerate() {
            self.events[k].extend(n.tick(now));
        }
        self.collect();
        moved
    }

    /// Pumps round by round until `done` holds (panics if it never does).
    fn pump_until(&mut self, done: impl Fn(&Net) -> bool) {
        for _ in 0..4_000 {
            if done(self) {
                return;
            }
            self.round();
        }
        panic!("condition never held");
    }

    fn take_events(&mut self, i: usize) -> Vec<Event> {
        std::mem::take(&mut self.events[i])
    }

    fn assert_no_wall_reads(&self) {
        for (i, n) in self.nodes.iter().enumerate() {
            assert_eq!(n.wall_reads(), 0, "node {i} read the wall clock");
        }
    }
}

fn posts(events: &[Event]) -> Vec<(String, String, Vec<String>)> {
    events
        .iter()
        .filter_map(|e| match e {
            Event::Post {
                author_name,
                text,
                labels,
                ..
            } => Some((author_name.clone(), text.clone(), labels.clone())),
            _ => None,
        })
        .collect()
}

fn dms(events: &[Event]) -> Vec<String> {
    events
        .iter()
        .filter_map(|e| match e {
            Event::Dm { text, .. } => Some(text.clone()),
            _ => None,
        })
        .collect()
}

fn entry(net: &Net, i: usize) -> MemberEntry {
    MemberEntry {
        keycard: net.nodes[i].my_keycard().unwrap(),
        name: net.nodes[i].display_name().unwrap().to_owned(),
        in_person: true,
    }
}

fn win_index(net: &Net, winner: &IdentityKey) -> usize {
    if *winner == net.id(0) { 0 } else { 1 }
}

fn ready(state: SessionState) -> bool {
    matches!(state, SessionState::Ready | SessionState::Crossed)
}

/// A (admin), C (second admin, in the genesis), B added by A with C's
/// endorsement; D is a keyless station between them (A–D, B–D, C–D).
fn three_member_group(start: u64) -> (Net, GroupId) {
    let a = open_member(1, "A", start);
    let b = open_member(2, "B", start);
    let c = open_member(3, "C", start);
    let d = open_station(4, start);
    let mut net = Net::new(vec![a, b, c, d], start);
    net.befriend(0, 1);
    net.befriend(0, 2);
    net.befriend(1, 2);
    net.connect(0, 3);
    net.connect(1, 3);
    net.connect(2, 3);
    net.pump();
    let c_card = net.nodes[2].my_keycard().unwrap();
    let c_entry = entry(&net, 2);
    let now = net.now();
    let gid = net.nodes[0]
        .create_group("Shelter", &[c_card], &[c_entry], 0, now)
        .unwrap();
    net.pump();
    assert_eq!(
        net.nodes[2].members(&gid).unwrap().len(),
        2,
        "C holds the genesis"
    );
    let b_entry = entry(&net, 1);
    let draft = net.nodes[0].propose_add(gid, b_entry.clone()).unwrap();
    let endorsement = net.nodes[2].endorse(&draft, true).unwrap();
    let now = net.now();
    net.nodes[0]
        .add_member(gid, b_entry, endorsement, now)
        .unwrap();
    net.pump();
    for i in 0..3 {
        assert_eq!(
            net.nodes[i].members(&gid).unwrap().len(),
            3,
            "node {i} sees three members"
        );
    }
    (net, gid)
}

// ---- (1) DMs over a byte pipe --------------------------------------------------

#[test]
fn two_nodes_exchange_dms_with_receipts() {
    let start = 1_000;
    let mut net = Net::new(
        vec![open_member(1, "A", start), open_member(2, "B", start)],
        start,
    );
    net.befriend(0, 1);
    net.connect(0, 1);
    net.pump();
    let ev_a = net.take_events(0);
    let ev_b = net.take_events(1);
    assert!(
        ev_a.iter().any(|e| matches!(
            e,
            Event::LinkState {
                up: true,
                auth: LinkAuth::VerifiedContact(_),
                ..
            }
        )),
        "A's link is L2: {ev_a:?}"
    );
    assert!(ev_b.iter().any(|e| matches!(
        e,
        Event::LinkState {
            up: true,
            auth: LinkAuth::VerifiedContact(_),
            ..
        }
    )));
    let b_id = net.id(1);
    let a_id = net.id(0);
    let now = net.now();
    let obj = net.nodes[0].send_dm(b_id, "hi B", now).unwrap();
    assert_eq!(net.nodes[0].delivery(&obj).unwrap(), DeliveryState::Waiting);
    net.pump();
    assert_eq!(dms(&net.take_events(1)), vec!["hi B".to_owned()]);
    assert_eq!(
        net.nodes[0].delivery(&obj).unwrap(),
        DeliveryState::Delivered
    );
    let ev = net.take_events(0);
    assert!(ev.iter().any(|e| matches!(
        e,
        Event::Delivery {
            state: DeliveryState::Delivered,
            ..
        }
    )));
    // the reply
    let now = net.now();
    let obj2 = net.nodes[1].send_dm(a_id, "hi A", now).unwrap();
    net.pump();
    assert_eq!(dms(&net.take_events(0)), vec!["hi A".to_owned()]);
    assert_eq!(
        net.nodes[1].delivery(&obj2).unwrap(),
        DeliveryState::Delivered
    );
    assert!(ready(net.nodes[0].session_state(&b_id)));
    assert!(ready(net.nodes[1].session_state(&a_id)));
    // a later message uses the recognition epoch (no first flight)
    let now = net.now();
    let obj3 = net.nodes[0].send_dm(b_id, "again", now).unwrap();
    net.pump();
    assert_eq!(dms(&net.take_events(1)), vec!["again".to_owned()]);
    assert_eq!(
        net.nodes[0].delivery(&obj3).unwrap(),
        DeliveryState::Delivered
    );
    assert_eq!(net.nodes[0].session_state(&b_id), SessionState::Ready);
    let now = net.now();
    assert_eq!(net.nodes[0].status_strip(now).nearby, 1);
    assert_eq!(
        net.nodes[0].plaintext(&obj).unwrap().as_deref(),
        Some("hi B")
    );
    net.assert_no_wall_reads();
}

// ---- (2) group with a relay station -------------------------------------------------

#[test]
fn three_node_group_posts_relayed_by_a_station() {
    let (mut net, gid) = three_member_group(5_000);
    for i in 0..3 {
        net.take_events(i);
    }
    let now = net.now();
    let obj = net.nodes[0].send_text(gid, "hello all", now).unwrap();
    net.pump();
    for i in 1..3 {
        let got = posts(&net.take_events(i));
        assert_eq!(
            got,
            vec![("A".to_owned(), "hello all".to_owned(), vec![])],
            "node {i}"
        );
    }
    assert_eq!(
        net.nodes[0].delivery(&obj).unwrap(),
        DeliveryState::Delivered
    );
    // the station relayed opaque cells only
    assert_eq!(net.nodes[3].stats().decrypted, 0);
    assert!(net.nodes[3].store_len() > 0);
    assert!(net.nodes[3].identity().is_none());
    // B posts; A and C see it with B's name
    let now = net.now();
    net.nodes[1].send_text(gid, "from B", now).unwrap();
    net.pump();
    for i in [0usize, 2] {
        assert_eq!(
            posts(&net.take_events(i)),
            vec![("B".to_owned(), "from B".to_owned(), vec![])],
            "node {i}"
        );
    }
    let now = net.now();
    let strip = net.nodes[3].status_strip(now);
    assert_eq!(strip.nearby, 3);
    assert!(strip.passing_along);
    net.assert_no_wall_reads();
}

// ---- (3) removal: rejected, labelled, undecryptable ----------------------------------

#[test]
fn removal_hides_citing_posts_labels_earlier_ones_and_locks_out_the_removed() {
    let (mut net, gid) = three_member_group(9_000);
    let b_id = net.id(1);
    // B's link goes down; A removes B; A and C apply the removal.
    net.disconnect(1);
    let now = net.now();
    net.nodes[0].remove_member(gid, b_id, now).unwrap();
    net.pump();
    assert!(!net.nodes[2].roster(&gid).unwrap().is_member(&b_id));
    for i in 0..3 {
        net.take_events(i);
    }
    // B, unaware, posts; its heads lack the removal: kept and labelled.
    let now = net.now();
    net.nodes[1].send_text(gid, "late news", now).unwrap();
    net.pipes.remove(1);
    net.connect(1, 3);
    net.pump();
    let expect = vec![(
        "B".to_owned(),
        "late news".to_owned(),
        vec!["Sent by B, who was removed".to_owned()],
    )];
    assert_eq!(
        posts(&net.take_events(0)),
        expect,
        "A labels B's split-time post"
    );
    assert_eq!(
        posts(&net.take_events(2)),
        expect,
        "C labels B's split-time post"
    );
    assert!(
        !net.nodes[1].roster(&gid).unwrap().is_member(&b_id),
        "B learnt its removal"
    );
    // Posts after the removal rotate the chain: B cannot read them.
    let decrypted_before = net.nodes[1].stats().decrypted;
    let now = net.now();
    net.nodes[0].send_text(gid, "after removal", now).unwrap();
    net.nodes[2].send_text(gid, "C after", now).unwrap();
    net.pump();
    assert!(
        posts(&net.take_events(1)).is_empty(),
        "B reads nothing after its removal"
    );
    assert_eq!(net.nodes[1].stats().decrypted, decrypted_before);
    assert_eq!(
        posts(&net.take_events(0)),
        vec![("C".to_owned(), "C after".to_owned(), vec![])]
    );
    assert_eq!(
        posts(&net.take_events(2)),
        vec![("A".to_owned(), "after removal".to_owned(), vec![])]
    );
    let now = net.now();
    assert!(
        net.nodes[1].send_text(gid, "refused", now).is_err(),
        "a stock client refuses"
    );
    net.assert_no_wall_reads();
}

#[test]
fn a_removed_insider_posting_with_the_removal_in_its_heads_is_rejected_everywhere() {
    let (mut net, gid) = three_member_group(11_000);
    let b_id = net.id(1);
    // The insider keeps its keys: it ignores the rotation trigger.
    net.nodes[1].set_adversary_keep_keys(true);
    let now = net.now();
    net.nodes[0].remove_member(gid, b_id, now).unwrap();
    net.pump();
    assert!(!net.nodes[1].roster(&gid).unwrap().is_member(&b_id));
    for i in 0..3 {
        net.take_events(i);
    }
    let rejected_a = net.nodes[0].stats().rejected_posts;
    let rejected_c = net.nodes[2].stats().rejected_posts;
    // Under its old chain, citing heads that include its own removal.
    let now = net.now();
    net.nodes[1].send_text(gid, "sneaky", now).unwrap();
    let penalties_a = net.nodes[0].stats().penalties;
    // DECISIONS §3 row (1,2,3,4,10) (iii): the link the rejected post arrived
    // on is penalised, at once (here A's station link, LinkId(1)).
    net.pump_until(|net| net.nodes[0].stats().rejected_posts > rejected_a);
    assert!(
        net.nodes[0].stats().penalties > penalties_a,
        "A penalised the source link"
    );
    let now = net.now();
    assert!(
        net.nodes[0].link_score(LinkId(1), now).unwrap() < 1.0,
        "A's station link lost score"
    );
    net.pump();
    assert!(posts(&net.take_events(0)).is_empty(), "A hides it");
    assert!(posts(&net.take_events(2)).is_empty(), "C hides it");
    assert_eq!(net.nodes[0].stats().rejected_posts, rejected_a + 1);
    assert_eq!(net.nodes[2].stats().rejected_posts, rejected_c + 1);
    assert!(
        net.nodes[2].stats().penalties >= 1,
        "C penalised the source link"
    );
    // The same verdict covers every post kind: a task op citing the
    // removal is rejected too (DECISIONS §3 row (1,2,3,4,10) (ii) "a post
    // from removed member X whose own carried roster heads include X's
    // removal is rejected everywhere"; only ballots, vote-opens and roster
    // ops are exempt by (i)).
    let now = net.now();
    let task = net.nodes[1]
        .create_task(gid, "Sneaky task", "", now)
        .unwrap();
    net.pump();
    assert!(
        net.nodes[0].task(&gid, &task).is_err(),
        "A refuses the task"
    );
    assert!(
        net.nodes[2].task(&gid, &task).is_err(),
        "C refuses the task"
    );
    assert_eq!(net.nodes[0].stats().rejected_posts, rejected_a + 2);
    net.assert_no_wall_reads();
}

// ---- receipts never settle an object that was never sent --------------------------------

#[test]
fn a_dm_that_needs_a_key_is_never_marked_delivered_by_a_later_receipt() {
    let start = 37_000;
    let mut net = Net::new(
        vec![open_member(1, "A", start), open_member(2, "B", start)],
        start,
    );
    let b_id = net.id(1);
    let now = net.now();
    // No bundle for B yet: the message has no cells and shows "Needs their key".
    let stuck = net.nodes[0].send_dm(b_id, "too early", now).unwrap();
    assert_eq!(
        net.nodes[0].delivery(&stuck).unwrap(),
        DeliveryState::NeedsKey
    );
    net.befriend(0, 1);
    net.connect(0, 1);
    net.pump();
    let now = net.now();
    let real = net.nodes[0].send_dm(b_id, "real", now).unwrap();
    net.pump();
    assert_eq!(dms(&net.take_events(1)), vec!["real".to_owned()]);
    assert_eq!(
        net.nodes[0].delivery(&real).unwrap(),
        DeliveryState::Delivered
    );
    // B's receipt covers what B received; "too early" never left A.
    assert_eq!(
        net.nodes[0].delivery(&stuck).unwrap(),
        DeliveryState::NeedsKey,
        "a message that was never sent cannot be Delivered (BRIEF_v2 §5.9)"
    );
    assert!(
        !net.take_events(0).iter().any(|e| matches!(
            e,
            Event::Delivery { obj, state: DeliveryState::Delivered } if *obj == stuck
        )),
        "no Delivered event for the unsent message"
    );
    net.assert_no_wall_reads();
}

// ---- "Passed along" is committed before it is shown ------------------------------------

#[test]
fn passed_along_is_not_recorded_when_its_commit_fails() {
    let start = 41_000;
    let mut net = Net::new(
        vec![open_member(1, "A", start), open_member(2, "B", start)],
        start,
    );
    net.befriend(0, 1);
    net.connect(0, 1);
    net.pump();
    let b_id = net.id(1);
    let now = net.now();
    let obj = net.nodes[0].send_dm(b_id, "hold on", now).unwrap();
    assert_eq!(net.nodes[0].delivery(&obj).unwrap(), DeliveryState::Waiting);
    net.take_events(0);
    // A's cells go out on the link, but A cannot commit (BRIEF_v2 §5.3): the
    // state it shows must be the last committed one.
    net.nodes[0].set_fail_before_commit(true);
    let (_, li, _, lj, _) = net.pipes[0];
    let now = net.advance(STEP_MS);
    let bytes = net.nodes[0].poll_outbound(li, CHUNK, now);
    assert!(!bytes.is_empty(), "the cells went out");
    net.nodes[1].feed(lj, &bytes, now).unwrap();
    assert_eq!(
        net.nodes[0].delivery(&obj).unwrap(),
        DeliveryState::Waiting,
        "no state change without a commit"
    );
    assert!(
        !net.nodes[0].poll_events().iter().any(|e| matches!(
            e,
            Event::Delivery {
                state: DeliveryState::PassedAlong,
                ..
            }
        )),
        "no event before a commit"
    );
    // Back to normal: the message still gets through and is acknowledged.
    net.nodes[0].set_fail_before_commit(false);
    net.pump();
    assert_eq!(dms(&net.take_events(1)), vec!["hold on".to_owned()]);
    assert_eq!(
        net.nodes[0].delivery(&obj).unwrap(),
        DeliveryState::Delivered
    );
    net.assert_no_wall_reads();
}

// ---- object ids agree across phones for multi-fragment objects --------------------------

#[test]
fn a_multi_fragment_dm_has_the_same_object_id_on_both_phones() {
    let start = 43_000;
    let mut net = Net::new(
        vec![open_member(1, "A", start), open_member(2, "B", start)],
        start,
    );
    net.befriend(0, 1);
    net.connect(0, 1);
    net.pump();
    let b_id = net.id(1);
    let now = net.now();
    // the first flight, then a text that spans several cells
    net.nodes[0].send_dm(b_id, "first", now).unwrap();
    net.pump();
    net.take_events(1);
    let long = "x".repeat(1_900);
    let now = net.now();
    let obj = net.nodes[0].send_dm(b_id, &long, now).unwrap();
    net.pump();
    let got: Vec<ObjId> = net
        .take_events(1)
        .iter()
        .filter_map(|e| match e {
            Event::Dm { obj, text, .. } if *text == long => Some(*obj),
            _ => None,
        })
        .collect();
    assert_eq!(
        got,
        vec![obj],
        "the receiver derives the sender's object id"
    );
    assert_eq!(
        net.nodes[0].delivery(&obj).unwrap(),
        DeliveryState::Delivered
    );
    net.assert_no_wall_reads();
}

// ---- (4) crash replay -----------------------------------------------------------------

#[test]
fn crash_before_commit_replays_to_exactly_one_delivery() {
    let start = 13_000;
    let mut net = Net::new(
        vec![open_member(1, "A", start), open_member(2, "B", start)],
        start,
    );
    net.befriend(0, 1);
    let pipe = net.connect(0, 1);
    net.pump();
    let b_id = net.id(1);
    let a_id = net.id(0);
    let now = net.now();
    net.nodes[0].send_dm(b_id, "warm-up", now).unwrap();
    net.nodes[1].send_dm(a_id, "warm-up back", now).unwrap();
    net.pump();
    for i in 0..2 {
        net.take_events(i);
    }
    let store_before = net.nodes[1].store_len();
    let outbox_before = net.nodes[1].outbox();
    let state_before = net.nodes[1].session_state(&a_id);
    let now = net.now();
    let obj = net.nodes[0].send_dm(b_id, "only once", now).unwrap();
    net.nodes[1].set_fail_before_commit(true);
    net.pump();
    assert!(
        dms(&net.take_events(1)).is_empty(),
        "nothing delivered before the commit"
    );
    assert!(net.nodes[1].stats().rollbacks >= 1);
    assert_eq!(
        net.nodes[1].store_len(),
        store_before,
        "the cell was not stored"
    );
    assert_eq!(net.nodes[1].outbox(), outbox_before);
    assert_eq!(net.nodes[1].session_state(&a_id), state_before);
    assert_ne!(
        net.nodes[0].delivery(&obj).unwrap(),
        DeliveryState::Delivered
    );
    // The phone comes back: a new encounter re-offers the cell from A's store.
    net.nodes[1].set_fail_before_commit(false);
    net.disconnect(pipe);
    net.connect(0, 1);
    net.pump();
    assert_eq!(dms(&net.take_events(1)), vec!["only once".to_owned()]);
    assert_eq!(
        net.nodes[0].delivery(&obj).unwrap(),
        DeliveryState::Delivered
    );
    // and never again
    let last = net.pipes.len() - 1;
    net.disconnect(last);
    net.connect(0, 1);
    net.pump();
    assert!(dms(&net.take_events(1)).is_empty());
    net.assert_no_wall_reads();
}

// ---- (5) task conflict -----------------------------------------------------------------

#[test]
fn concurrent_takes_show_the_same_winner_everywhere() {
    let (mut net, gid) = three_member_group(17_000);
    let now = net.now();
    let task = net.nodes[0]
        .create_task(gid, "Water run", "annex", now)
        .unwrap();
    net.pump();
    for i in 0..3 {
        let (state, ui) = net.nodes[i].task(&gid, &task).unwrap();
        assert_eq!(state, TaskState::Open, "node {i}");
        assert_eq!(ui, "Open");
    }
    // A and B take concurrently (nothing pumped between).
    let now = net.now();
    net.nodes[0].take_task(gid, task, now).unwrap();
    net.nodes[1].take_task(gid, task, now).unwrap();
    assert_eq!(
        net.nodes[0].task(&gid, &task).unwrap().1,
        "Taken by you · Provisional (people near you only)"
    );
    net.pump();
    let states: Vec<TaskState> = (0..3)
        .map(|i| net.nodes[i].task(&gid, &task).unwrap().0)
        .collect();
    let TaskState::Conflict { winner, losers, .. } = &states[0] else {
        panic!("expected a conflict, got {:?}", states[0]);
    };
    assert_eq!(losers.len(), 1);
    assert!(
        states.iter().all(|s| s == &states[0]),
        "same winner everywhere: {states:?}"
    );
    assert!([net.id(0), net.id(1)].contains(winner));
    let (_, ui_c) = net.nodes[2].task(&gid, &task).unwrap();
    assert!(ui_c.contains("Conflict:"), "{ui_c}");
    let (_, ui_w) = net.nodes[win_index(&net, winner)]
        .task(&gid, &task)
        .unwrap();
    assert!(
        ui_w.contains("Conflict:") || ui_w.contains("Taken by you"),
        "{ui_w}"
    );
    // the loser releases; the winner is done
    let loser = if *winner == net.id(0) { 1 } else { 0 };
    let win = 1 - loser;
    let now = net.now();
    net.nodes[loser].release_task(gid, task, now).unwrap();
    net.nodes[win].done_task(gid, task, now).unwrap();
    net.pump();
    for i in 0..3 {
        assert_eq!(
            net.nodes[i].task(&gid, &task).unwrap().0,
            TaskState::Done,
            "node {i}"
        );
    }
    net.assert_no_wall_reads();
}

// ---- (6) vote with an equivocator ------------------------------------------------------

#[test]
fn vote_with_an_equivocator_tallies_identically() {
    let (mut net, gid) = three_member_group(19_000);
    let now = net.now();
    let vote = net.nodes[0]
        .open_vote(gid, "Move the kitchen?", &["Yes", "No"], 1, now)
        .unwrap();
    net.pump();
    for i in 0..3 {
        let (tally, line) = net.nodes[i].vote(&vote).unwrap();
        assert!(tally.is_some(), "node {i} validated the opening: {line}");
        assert_eq!(line, "Y0 N0 · Provisional");
    }
    let now = net.now();
    net.nodes[0].cast_ballot(vote, 0, now).unwrap();
    // C sends conflicting ballots: void (both payloads kept as proof); it
    // counts as heard, not in the tally (BRIEF_v2 §4 "Ballots").
    net.nodes[2].cast_ballot(vote, 0, now).unwrap();
    net.nodes[2].cast_ballot(vote, 1, now).unwrap();
    net.pump();
    let lines: Vec<String> = (0..3)
        .map(|i| net.nodes[i].vote(&vote).unwrap().1)
        .collect();
    assert_eq!(lines[0], "Y1 N0 · 1 void · Provisional");
    assert!(lines.iter().all(|l| l == &lines[0]), "{lines:?}");
    // B's Yes reaches T = 3 with the equivocator counted as a signer: Final
    // everywhere, identical (D1 rule 4).
    let now = net.now();
    net.nodes[1].cast_ballot(vote, 0, now).unwrap();
    net.pump();
    let lines: Vec<String> = (0..3)
        .map(|i| net.nodes[i].vote(&vote).unwrap().1)
        .collect();
    assert_eq!(lines[0], "Y2 N0 · 1 void · Final");
    assert!(lines.iter().all(|l| l == &lines[0]), "{lines:?}");
    let ev = net.take_events(1);
    assert!(ev.iter().any(|e| matches!(e, Event::VoteChanged { .. })));
    net.assert_no_wall_reads();
}

// ---- (7) flood from an anonymous link --------------------------------------------------

#[test]
fn junk_flood_never_evicts_pinned_roster_ops_and_stays_bounded() {
    let start = 23_000;
    let mut cfg = member_cfg(1, "A");
    cfg.buffer = BufferPolicy {
        total_cap_bytes: 64 * 512,
        per_link_cap_bytes: 24 * 512,
        absent_peer_reserve: 0.2,
    };
    let a = Node::open(db(1), cfg, start, ChaCha20Rng::seed_from_u64(1)).unwrap();
    let b = open_member(2, "B", start);
    let e = open_member(9, "E", start);
    let mut net = Net::new(vec![a, b, e], start);
    net.befriend(0, 1);
    net.connect(0, 1);
    net.pump();
    let b_entry = entry(&net, 1);
    let now = net.now();
    net.nodes[0]
        .create_group("Shelter", &[], &[b_entry], 0, now)
        .unwrap();
    net.pump();
    let pinned: BTreeSet<Hash32> = net.nodes[0].pinned_cells().into_iter().collect();
    assert!(!pinned.is_empty(), "roster ops are pinned");
    // E is a stranger: its link is anonymous (L3).
    net.connect(2, 0);
    net.pump();
    let now = net.now();
    let junk = net.nodes[2].adversary_inject_junk(600, now);
    assert_eq!(junk.len(), 600);
    for _ in 0..20 {
        net.pump();
        net.advance(ms::MINUTE);
    }
    let still: BTreeSet<Hash32> = net.nodes[0].pinned_cells().into_iter().collect();
    assert!(pinned.is_subset(&still), "no pinned cell was evicted");
    let junk_held = junk.iter().filter(|id| net.nodes[0].holds(id)).count();
    assert!(
        junk_held <= 24,
        "junk stays within the per-link share: {junk_held}"
    );
    assert!(net.nodes[0].store_len() <= 64);
    assert_eq!(net.nodes[0].stats().decrypted, 0, "junk never decrypts");
    net.assert_no_wall_reads();
}

// ---- (8) persistence -------------------------------------------------------------------

#[test]
fn reopening_the_same_db_restores_state_and_keeps_decrypting() {
    let start = 29_000;
    let mut net = Net::new(
        vec![open_member(1, "A", start), open_member(2, "B", start)],
        start,
    );
    net.befriend(0, 1);
    let pipe = net.connect(0, 1);
    net.pump();
    let b_id = net.id(1);
    let b_entry = entry(&net, 1);
    let now = net.now();
    let gid = net.nodes[0]
        .create_group("Shelter", &[], &[b_entry], 0, now)
        .unwrap();
    net.pump();
    let now = net.now();
    let obj = net.nodes[0].send_text(gid, "before restart", now).unwrap();
    let dm = net.nodes[0].send_dm(b_id, "dm before", now).unwrap();
    net.pump();
    assert_eq!(
        net.nodes[0].delivery(&obj).unwrap(),
        DeliveryState::Delivered
    );
    assert_eq!(
        net.nodes[0].delivery(&dm).unwrap(),
        DeliveryState::Delivered
    );
    net.disconnect(pipe);
    for i in 0..2 {
        net.take_events(i);
    }
    let roster = net.nodes[0].roster(&gid).unwrap();
    let contacts = net.nodes[0].contacts();
    let outbox = net.nodes[0].outbox();
    let card = net.nodes[0].my_keycard().unwrap();
    // Restart A on the same database.
    let a = net.nodes.remove(0);
    let db = a.into_db();
    let now = net.advance(ms::MINUTE);
    let a2 = Node::open(db, member_cfg(1, "A"), now, ChaCha20Rng::seed_from_u64(77)).unwrap();
    net.nodes.insert(0, a2);
    assert_eq!(net.nodes[0].my_keycard().unwrap(), card);
    assert_eq!(net.nodes[0].roster(&gid).unwrap(), roster);
    assert_eq!(net.nodes[0].contacts(), contacts);
    assert_eq!(net.nodes[0].outbox(), outbox);
    assert_eq!(
        net.nodes[0].delivery(&obj).unwrap(),
        DeliveryState::Delivered
    );
    assert_eq!(
        net.nodes[0].plaintext(&obj).unwrap().as_deref(),
        Some("before restart")
    );
    assert!(ready(net.nodes[0].session_state(&b_id)));
    // Both directions keep working with the restored chains and epochs.
    net.pipes.clear();
    net.connect(0, 1);
    net.pump();
    let a_id = net.id(0);
    let now = net.now();
    net.nodes[1]
        .send_text(gid, "to the restarted A", now)
        .unwrap();
    net.nodes[1].send_dm(a_id, "dm after", now).unwrap();
    net.pump();
    let ev = net.take_events(0);
    assert_eq!(
        posts(&ev),
        vec![("B".to_owned(), "to the restarted A".to_owned(), vec![])]
    );
    assert_eq!(dms(&ev), vec!["dm after".to_owned()]);
    let now = net.now();
    let obj2 = net.nodes[0]
        .send_text(gid, "from the restarted A", now)
        .unwrap();
    net.pump();
    assert_eq!(
        posts(&net.take_events(1)),
        vec![("A".to_owned(), "from the restarted A".to_owned(), vec![])]
    );
    assert_eq!(
        net.nodes[0].delivery(&obj2).unwrap(),
        DeliveryState::Delivered
    );
    net.assert_no_wall_reads();
}

// ---- delivery ageing --------------------------------------------------------------------

#[test]
fn an_unacknowledged_object_ages_to_may_have_expired_and_can_be_resent() {
    let start = 31_000;
    let mut net = Net::new(
        vec![open_member(1, "A", start), open_member(2, "B", start)],
        start,
    );
    net.befriend(0, 1);
    let b_id = net.id(1);
    let now = net.now();
    let obj: ObjId = net.nodes[0].send_dm(b_id, "lonely", now).unwrap();
    assert_eq!(net.nodes[0].delivery(&obj).unwrap(), DeliveryState::Waiting);
    let strip = net.nodes[0].status_strip(now);
    assert_eq!(strip.text(), "Offline – will send later");
    let later = net.advance(meshcore::node::status::MAX_AGE_ASM + ms::MINUTE);
    net.nodes[0].tick(later);
    assert_eq!(
        net.nodes[0].delivery(&obj).unwrap(),
        DeliveryState::MayHaveExpired
    );
    assert!(DeliveryState::MayHaveExpired.can_resend());
    let again = net.nodes[0].resend(&obj, later + 1).unwrap();
    assert_ne!(again, obj);
    assert_eq!(
        net.nodes[0].delivery(&again).unwrap(),
        DeliveryState::Waiting
    );
    net.assert_no_wall_reads();
}
