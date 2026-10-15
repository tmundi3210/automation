//! Facade tests (docs/DESIGN.md §1 row `api/*`; §10 "JVM two-node smoke"
//! in the small): two [`MeshNode`]s over a byte pipe exchange a 1:1 text
//! and a group text through the facade only; configuration and type round
//! trips; error mapping; the event sink is invoked outside the node lock
//! (a re-entrant call from inside the sink does not deadlock). Default
//! features (the real backend) for everything that opens a node; no OS
//! clock anywhere (`wall_reads() == 0` at the end of every run).

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use meshcore::api::{
    ApiBranch, ApiConfig, ApiDeliveryState, ApiError, ApiEvent, ApiImportBadge, ApiImportedContact,
    ApiIssueContext, ApiLinkAuth, ApiLinkRole, ApiMemberEntry, ApiRole, MeshNode,
};
use meshcore::node::NodeConfig;

// ---- ungated: types and argument checks --------------------------------------------

#[test]
fn config_round_trips_and_defaults_match_the_core() {
    let api = ApiConfig::member("Ada");
    assert_eq!(
        api.role,
        ApiRole::Member {
            display_name: "Ada".to_owned()
        }
    );
    assert_eq!(api.branch, ApiBranch::A);
    let core = api.to_node_config(&[1; 32]).expect("valid");
    assert_eq!(ApiConfig::from_node_config(&core), api);
    assert_eq!(
        ApiConfig::station(),
        ApiConfig::from_node_config(&NodeConfig::station())
    );
    let core_default = NodeConfig::station();
    assert_eq!(
        ApiConfig::station().dedupe_retention_ms,
        core_default.dedupe_retention_ms
    );
    assert_eq!(ApiConfig::station().max_links, core_default.max_links);
    let mut b = ApiConfig::member("B");
    b.branch = ApiBranch::B;
    b.total_cap_bytes = 1 << 20;
    b.absent_peer_reserve_ppm = 250_000;
    let core = b.to_node_config(&[2; 32]).expect("valid");
    assert_eq!(ApiConfig::from_node_config(&core), b);
}

#[test]
fn open_rejects_bad_arguments_before_touching_storage() {
    let bad_key = MeshNode::open(None, vec![0; 31], vec![0; 32], ApiConfig::station(), 0);
    assert!(matches!(bad_key, Err(ApiError::InvalidArgument { .. })));
    let bad_entropy = MeshNode::open(None, vec![0; 32], vec![0; 16], ApiConfig::station(), 0);
    assert!(matches!(bad_entropy, Err(ApiError::InvalidArgument { .. })));
    let bad_name = MeshNode::open(None, vec![0; 32], vec![0; 32], ApiConfig::member(""), 0);
    assert!(matches!(bad_name, Err(ApiError::InvalidArgument { .. })));
    assert_eq!(
        ApiError::NeedsKey { peer: vec![1; 32] }.message(),
        "Needs their key",
        "§7 vocabulary"
    );
}

// ---- gated: everything that opens a node -------------------------------------------

#[cfg(feature = "backend-libsignal")]
mod with_backend {
    use super::*;

    const CHUNK: u32 = 4096;
    /// Simulated time between pump rounds (budgets refill, holds expire).
    const STEP_MS: u64 = 2_000;

    fn open_member(seed: u8, name: &str, now: u64) -> Rc<MeshNode> {
        Rc::new(
            MeshNode::open(
                None,
                vec![seed; 32],
                vec![seed ^ 0x77; 32],
                ApiConfig::member(name),
                now,
            )
            .expect("open member"),
        )
    }

    fn export(n: &MeshNode, now: u64) -> ApiImportedContact {
        ApiImportedContact {
            keycard: n.my_keycard().unwrap(),
            display_name: n.display_name().unwrap().unwrap(),
            bundle: n.my_bundle(ApiIssueContext::DirectEncounter, now).unwrap(),
            link_key: Some(vec![0xAB; 32]),
            envelope_pub: n.my_envelope_pub().unwrap(),
            badge: ApiImportBadge::VerifiedInPerson,
        }
    }

    /// Two facades joined by one byte pipe; the test is the radio and the
    /// clock.
    struct Pipe {
        a: Rc<MeshNode>,
        b: Rc<MeshNode>,
        now: u64,
        ev_a: Vec<ApiEvent>,
        ev_b: Vec<ApiEvent>,
    }

    const LA: u32 = 1;
    const LB: u32 = 2;

    impl Pipe {
        fn new(start: u64) -> Self {
            Pipe {
                a: open_member(1, "A", start),
                b: open_member(2, "B", start),
                now: start,
                ev_a: Vec::new(),
                ev_b: Vec::new(),
            }
        }

        /// Mutual provisioning (what a QR pairing yields).
        fn befriend(&self) {
            let ca = export(&self.a, self.now);
            let cb = export(&self.b, self.now);
            self.b.import_contact(ca, self.now).expect("import");
            self.a.import_contact(cb, self.now).expect("import");
        }

        fn connect(&self) {
            self.a.start(self.now).unwrap();
            self.b.start(self.now).unwrap();
            self.a
                .link_up(
                    LA,
                    ApiLinkRole::Central,
                    self.b.identity().unwrap(),
                    self.now,
                )
                .unwrap();
            self.b
                .link_up(
                    LB,
                    ApiLinkRole::Peripheral,
                    self.a.identity().unwrap(),
                    self.now,
                )
                .unwrap();
        }

        fn collect(&mut self) {
            self.ev_a.extend(self.a.poll_events());
            self.ev_b.extend(self.b.poll_events());
        }

        fn round(&mut self) -> bool {
            self.now += STEP_MS;
            let now = self.now;
            let mut moved = false;
            let bytes = self.a.poll_outbound(LA, CHUNK, now).unwrap();
            if !bytes.is_empty() {
                moved = true;
                self.b.feed(LB, bytes, now).expect("B's link alive");
            }
            let bytes = self.b.poll_outbound(LB, CHUNK, now).unwrap();
            if !bytes.is_empty() {
                moved = true;
                self.a.feed(LA, bytes, now).expect("A's link alive");
            }
            self.a.tick(now).unwrap();
            self.b.tick(now).unwrap();
            self.collect();
            moved
        }

        fn pump(&mut self) {
            let mut idle = 0;
            for _ in 0..4_000 {
                if self.round() {
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

        fn take_a(&mut self) -> Vec<ApiEvent> {
            std::mem::take(&mut self.ev_a)
        }

        fn take_b(&mut self) -> Vec<ApiEvent> {
            std::mem::take(&mut self.ev_b)
        }

        fn assert_no_wall_reads(&self) {
            assert_eq!(self.a.wall_reads().unwrap(), 0);
            assert_eq!(self.b.wall_reads().unwrap(), 0);
        }
    }

    fn dms(events: &[ApiEvent]) -> Vec<String> {
        events
            .iter()
            .filter_map(|e| match e {
                ApiEvent::Dm { text, .. } => Some(text.clone()),
                _ => None,
            })
            .collect()
    }

    fn posts(events: &[ApiEvent]) -> Vec<(String, String)> {
        events
            .iter()
            .filter_map(|e| match e {
                ApiEvent::Post {
                    author_name, text, ..
                } => Some((author_name.clone(), text.clone())),
                _ => None,
            })
            .collect()
    }

    thread_local! {
        static SINK_NODE: RefCell<Option<Rc<MeshNode>>> = const { RefCell::new(None) };
        static SINK_SEEN: RefCell<Vec<ApiEvent>> = const { RefCell::new(Vec::new()) };
        static SINK_REENTERED: Cell<u32> = const { Cell::new(0) };
    }

    #[test]
    fn two_nodes_exchange_a_dm_and_a_group_text_through_the_facade() {
        let mut p = Pipe::new(1_000);
        p.befriend();
        p.connect();
        p.pump();
        let ev_a = p.take_a();
        assert!(
            ev_a.iter().any(|e| matches!(
                e,
                ApiEvent::LinkState {
                    link: LA,
                    up: true,
                    auth: ApiLinkAuth::VerifiedContact { .. }
                }
            )),
            "A's link is a verified-contact link: {ev_a:?}"
        );
        let _ = p.take_b();
        assert_eq!(p.a.status_strip(p.now).unwrap().nearby, 1);
        let strip = p.a.status_strip(p.now).unwrap();
        assert!(
            strip.text.starts_with("1 nearby") && strip.last_sync_age_ms.is_some(),
            "{strip:?}"
        );
        assert_eq!(strip.last_sync_display_ms, None, "no display clock set");

        // ---- 1:1 text, with B's sink installed (called outside the lock) ----
        SINK_NODE.with(|n| *n.borrow_mut() = Some(p.b.clone()));
        p.b.set_event_sink(Some(Box::new(|e| {
            SINK_NODE.with(|n| {
                let n = n.borrow();
                let n = n.as_ref().unwrap();
                // Re-entrant calls from inside the sink: a query and a poll.
                n.contacts().expect("re-entrant query");
                n.poll_events();
            });
            SINK_REENTERED.with(|r| r.set(r.get() + 1));
            SINK_SEEN.with(|s| s.borrow_mut().push(e));
        })));
        let b_id = p.b.identity().unwrap().unwrap();
        let a_id = p.a.identity().unwrap().unwrap();
        let obj = p.a.send_dm(b_id.clone(), "hi B".to_owned(), p.now).unwrap();
        assert_eq!(obj.len(), 32);
        assert_eq!(
            p.a.delivery(obj.clone()).unwrap(),
            ApiDeliveryState::Waiting
        );
        p.pump();
        let via_sink = SINK_SEEN.with(|s| s.borrow().clone());
        assert_eq!(dms(&via_sink), vec!["hi B".to_owned()], "{via_sink:?}");
        assert!(SINK_REENTERED.with(Cell::get) >= 1);
        assert!(
            dms(&p.take_b()).is_empty(),
            "events went to the sink, not the poll queue"
        );
        assert_eq!(
            p.a.delivery(obj.clone()).unwrap(),
            ApiDeliveryState::Delivered
        );
        assert!(p.take_a().iter().any(|e| matches!(
            e,
            ApiEvent::Delivery {
                state: ApiDeliveryState::Delivered,
                state_text,
                ..
            } if state_text == "Delivered"
        )));
        assert_eq!(p.a.plaintext(obj.clone()).unwrap().as_deref(), Some("hi B"));
        let outbox = p.a.outbox().unwrap();
        assert!(outbox.iter().any(|o| o.obj == obj && o.text == "Delivered"));
        p.b.set_event_sink(None);
        // the reply through the poll queue
        let obj2 = p.b.send_dm(a_id.clone(), "hi A".to_owned(), p.now).unwrap();
        p.pump();
        assert_eq!(dms(&p.take_a()), vec!["hi A".to_owned()]);
        assert_eq!(p.b.delivery(obj2).unwrap(), ApiDeliveryState::Delivered);
        let contacts = p.a.contacts().unwrap();
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].key, b_id);
        assert_eq!(contacts[0].name, "B");
        assert_eq!(p.a.name_of(b_id.clone()).unwrap(), "B");

        // ---- group text ----
        let b_entry = ApiMemberEntry {
            keycard: p.b.my_keycard().unwrap(),
            name: "B".to_owned(),
            in_person: true,
        };
        let gid =
            p.a.create_group("Shelter".to_owned(), vec![], vec![b_entry], 0, p.now)
                .unwrap();
        assert_eq!(gid.len(), 16);
        p.pump();
        assert_eq!(
            p.b.groups().unwrap(),
            vec![gid.clone()],
            "B holds the genesis"
        );
        let roster_b = p.b.roster(gid.clone()).unwrap();
        assert_eq!(roster_b.name, "Shelter");
        assert_eq!(roster_b.members.len(), 2);
        assert_eq!(roster_b.admins, vec![a_id.clone()]);
        assert!(roster_b.unresolved.is_empty());
        assert!(
            roster_b
                .members
                .iter()
                .any(|m| m.key == a_id && m.is_admin && m.name == "A")
        );
        let _ = p.take_b();
        let post =
            p.a.send_text(gid.clone(), "water at the north door".to_owned(), p.now)
                .unwrap();
        p.pump();
        assert_eq!(
            posts(&p.take_b()),
            vec![("A".to_owned(), "water at the north door".to_owned())]
        );
        assert_eq!(
            p.a.delivery(post.clone()).unwrap(),
            ApiDeliveryState::Delivered
        );
        assert_eq!(
            p.a.plaintext(post).unwrap().as_deref(),
            Some("water at the north door")
        );
        // B answers in the group
        let reply =
            p.b.send_text(gid.clone(), "on my way".to_owned(), p.now)
                .unwrap();
        p.pump();
        assert_eq!(
            posts(&p.take_a()),
            vec![("B".to_owned(), "on my way".to_owned())]
        );
        assert_eq!(p.b.delivery(reply).unwrap(), ApiDeliveryState::Delivered);
        assert!(p.a.stats().unwrap().decrypted >= 2);
        p.assert_no_wall_reads();
    }

    #[test]
    fn errors_map_to_the_flat_enum() {
        let station = MeshNode::open(None, vec![3; 32], vec![4; 32], ApiConfig::station(), 0)
            .expect("station");
        assert_eq!(station.identity().unwrap(), None);
        assert_eq!(
            station.send_text(vec![0; 16], "x".to_owned(), 1),
            Err(ApiError::NoIdentity)
        );
        let a = open_member(7, "A", 0);
        assert_eq!(
            a.send_dm(vec![0; 31], "x".to_owned(), 1),
            Err(ApiError::InvalidArgument {
                msg: "identity key must be 32 bytes".to_owned()
            })
        );
        assert_eq!(
            a.send_text(vec![9; 16], "x".to_owned(), 1),
            Err(ApiError::UnknownGroup)
        );
        assert_eq!(a.delivery(vec![1; 32]), Err(ApiError::UnknownObject));
        assert_eq!(
            a.roster(vec![1; 3]).unwrap_err().message().as_str(),
            "invalid argument: group id must be 16 bytes"
        );
        assert_eq!(a.feed(1, vec![1, 2, 3], 1), Err(ApiError::NotRunning));
        a.start(1).unwrap();
        assert_eq!(
            a.feed(1, vec![1, 2, 3], 1),
            Err(ApiError::UnknownLink { link: 1 })
        );
        let gossip = a.my_bundle(ApiIssueContext::Gossip, 2).unwrap_err();
        assert!(
            matches!(&gossip, ApiError::Protocol { msg } if msg.starts_with("bundle: ")),
            "gossiped bundles are refused by the core (D6 rule 2): {gossip:?}"
        );
        // A DM to a stranger is not an error: it waits as "Needs their key".
        let stranger = {
            let other = open_member(8, "S", 0);
            other.identity().unwrap().unwrap()
        };
        let obj = a.send_dm(stranger, "hi".to_owned(), 3).unwrap();
        assert_eq!(a.delivery(obj).unwrap(), ApiDeliveryState::NeedsKey);
        assert_eq!(ApiDeliveryState::NeedsKey.text(), "Needs their key");
        let ev = a.poll_events();
        assert!(ev.iter().any(|e| matches!(e, ApiEvent::NeedsKey { .. })));
        assert_eq!(a.wall_reads().unwrap(), 0);
    }

    /// `Node::tick` *returns* the events it produced (it drains the node's
    /// queue); the facade must hand them on like every other event
    /// (BRIEF_v2 §5.9 "May have expired · Resend" is produced only by
    /// ticks).
    #[test]
    fn tick_events_are_not_lost() {
        let p = Pipe::new(1_000);
        p.befriend();
        p.a.start(p.now).unwrap();
        let b_id = p.b.identity().unwrap().unwrap();
        let obj = p.a.send_dm(b_id, "ages".to_owned(), p.now).unwrap();
        assert_eq!(
            p.a.delivery(obj.clone()).unwrap(),
            ApiDeliveryState::Waiting
        );
        let _ = p.a.poll_events();
        // Past the receiver-side key retention: the object ages out.
        let later = p.now + 8 * 24 * 60 * 60 * 1_000;
        p.a.tick(later).unwrap();
        assert_eq!(
            p.a.delivery(obj.clone()).unwrap(),
            ApiDeliveryState::MayHaveExpired
        );
        let ev = p.a.poll_events();
        assert!(
            ev.iter().any(|e| matches!(
                e,
                ApiEvent::Delivery {
                    obj: o,
                    state: ApiDeliveryState::MayHaveExpired,
                    state_text,
                } if *o == obj && state_text == "May have expired · Resend"
            )),
            "the tick's Delivery event reaches the queue: {ev:?}"
        );
        // ... and the sink, when one is installed (`start` ticks too).
        SINK_SEEN.with(|s| s.borrow_mut().clear());
        p.a.set_event_sink(Some(Box::new(|e| {
            SINK_SEEN.with(|s| s.borrow_mut().push(e));
        })));
        let obj2 =
            p.a.send_dm(
                p.b.identity().unwrap().unwrap(),
                "ages too".to_owned(),
                later,
            )
            .unwrap();
        SINK_SEEN.with(|s| s.borrow_mut().clear());
        p.a.stop(later).unwrap();
        p.a.start(later + 8 * 24 * 60 * 60 * 1_000).unwrap();
        let via_sink = SINK_SEEN.with(|s| s.borrow().clone());
        assert!(
            via_sink.iter().any(|e| matches!(
                e,
                ApiEvent::Delivery {
                    obj: o,
                    state: ApiDeliveryState::MayHaveExpired,
                    ..
                } if *o == obj2
            )),
            "{via_sink:?}"
        );
        assert!(
            p.a.poll_events().is_empty(),
            "sink installed: nothing queued"
        );
        p.a.set_event_sink(None);
        p.assert_no_wall_reads();
    }

    #[test]
    fn stop_takes_every_link_down() {
        let mut p = Pipe::new(1_000);
        p.befriend();
        p.connect();
        p.pump();
        assert_eq!(p.a.links().unwrap().len(), 1);
        p.a.stop(p.now).unwrap();
        assert!(p.a.links().unwrap().is_empty());
        assert!(
            p.a.poll_events()
                .iter()
                .any(|e| matches!(e, ApiEvent::LinkState { up: false, .. }))
        );
        assert_eq!(p.a.feed(LA, vec![0], p.now), Err(ApiError::NotRunning));
        assert_eq!(
            p.a.status_strip(p.now).unwrap().text,
            "Offline – will send later"
        );
        // commands still queue while stopped
        let b_id = p.b.identity().unwrap().unwrap();
        let obj = p.a.send_dm(b_id, "later".to_owned(), p.now).unwrap();
        assert_eq!(p.a.delivery(obj).unwrap(), ApiDeliveryState::Waiting);
        p.assert_no_wall_reads();
    }

    #[test]
    fn reopen_keeps_identity_and_name() {
        let dir =
            std::env::temp_dir().join(format!("meshcore-api-{}-{}", std::process::id(), line!()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("node.db");
        let path_s = path.to_string_lossy().into_owned();
        let (id, card) = {
            let n = MeshNode::open(
                Some(path_s.clone()),
                vec![5; 32],
                vec![6; 32],
                ApiConfig::member("Ada"),
                0,
            )
            .expect("first open");
            (n.identity().unwrap().unwrap(), n.my_keycard().unwrap())
        };
        let n = MeshNode::open(
            Some(path_s),
            vec![5; 32],
            vec![0xEE; 40],
            ApiConfig::member("Someone else"),
            10,
        )
        .expect("reopen");
        assert_eq!(n.identity().unwrap().unwrap(), id, "the stored seed wins");
        assert_eq!(n.my_keycard().unwrap(), card);
        assert_eq!(n.display_name().unwrap().as_deref(), Some("Ada"));
        assert_eq!(
            n.config().unwrap().role,
            ApiRole::Member {
                display_name: "Ada".to_owned()
            },
            "config() reports the effective (stored) name, not the one passed"
        );
        drop(n);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
