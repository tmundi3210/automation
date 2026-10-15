//! People integration tests (DESIGN.md §10 "same-name normalisation
//! (confusables, casefold, Default_Ignorable)"; BRIEF_v2 §3; DECISIONS D5
//! R1–R9).

use meshcore::codec::{CodecError, Decode, Encode, decode};
use meshcore::crypto::backend::GroupId;
use meshcore::crypto::ed25519::{IdentityKey, IdentitySigner};
use meshcore::people::contacts::{
    Badge, CONTACT_VER, Contact, ContactError, ContactStore, MAX_PETNAME_BYTES, NS_CONTACTS,
    PinSource,
};
use meshcore::people::samename::{
    CARD_CAP_PER_HOUR_ASM, CARD_WINDOW_MS, CardDecision, Marker, SKELETON_DATA_VERSION,
    SameNameIndex, normalise,
};
use meshcore::storage::kv::{KvTxn, MemKv};
use proptest::prelude::*;

// ---- fixture ----------------------------------------------------------------

fn signer(i: u8) -> IdentitySigner {
    let mut seed = [0u8; 32];
    seed[0] = i;
    seed[1] = 0x77;
    IdentitySigner::from_seed(&seed)
}

fn key(i: u8) -> IdentityKey {
    signer(i).public()
}

fn contact(i: u8, name: &str) -> Contact {
    Contact::new(key(i), [i ^ 0x55; 32], name).expect("short name")
}

// ---- normalisation ----------------------------------------------------------

#[test]
fn skeleton_data_version_is_the_cargo_lock_pin() {
    // BRIEF §3 "with the data version pinned per release": the literal must
    // equal the unicode-security version this build resolves.
    let lock = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../Cargo.lock"))
        .expect("workspace Cargo.lock");
    let pinned = lock
        .split("[[package]]")
        .find(|p| p.contains("name = \"unicode-security\""))
        .and_then(|p| {
            p.lines()
                .find_map(|l| l.strip_prefix("version = \""))
                .map(|v| v.trim_end_matches('"').to_owned())
        })
        .expect("unicode-security in Cargo.lock");
    assert_eq!(SKELETON_DATA_VERSION, pinned);
}

#[test]
fn confusables_collapse() {
    let base = normalise("Ravi");
    // Cyrillic а (U+0430) vs Latin a
    assert_eq!(normalise("R\u{0430}vi"), base);
    // ℝ (U+211D) vs R
    assert_eq!(normalise("ℝavi"), base);
    // zero-width joiner inside the name
    assert_eq!(normalise("Ra\u{200D}vi"), base);
    // other Default_Ignorable: ZWNJ, ZWSP, soft hyphen, variation selector, BOM
    assert_eq!(
        normalise("R\u{200C}a\u{200B}v\u{00AD}i\u{FE0F}\u{FEFF}"),
        base
    );
    // case
    assert_eq!(normalise("RAVI"), base);
    assert_eq!(normalise("rAvI"), base);
    // trailing spaces and punctuation
    assert_eq!(normalise("Ravi   "), base);
    assert_eq!(normalise("Ravi."), base);
    assert_eq!(normalise("Ravi!!!"), base);
    assert_eq!(normalise("  Ravi … "), base);
    assert_eq!(normalise("Ravi\u{3000}\u{3002}"), base);
    // fullwidth letters (NFKC)
    assert_eq!(normalise("\u{FF32}\u{FF41}\u{FF56}\u{FF49}"), base);
    // Latin l vs digit 1 (UTS #39). Capital I is casefolded to i *before*
    // the skeleton (BRIEF §3 order), so it no longer maps to l: a known,
    // deliberate limit of folding first.
    assert_eq!(normalise("Ravil"), normalise("Ravi1"));
    assert_eq!(normalise("RaviI"), normalise("ravii"));
    assert_ne!(normalise("RaviI"), normalise("Ravil"));
}

#[test]
fn unrelated_names_do_not_collapse() {
    assert_ne!(normalise("Ravi"), normalise("Asha"));
    assert_ne!(normalise("Ravi"), normalise("Ravi K"));
    assert_ne!(normalise("Ravi"), normalise("Ravina"));
    assert_ne!(normalise("Marco"), normalise("Marc"));
    assert_ne!(normalise("Priya"), normalise("Priyanka"));
    assert_ne!(normalise("Ravi"), normalise("Rav"));
    assert_ne!(normalise("Ravi"), normalise(""));
}

// ---- marker -----------------------------------------------------------------

#[test]
fn marker_is_symmetric_and_removed_when_a_key_is_dropped() {
    let mut idx = SameNameIndex::new();
    idx.observe(key(1), "Ravi");
    idx.observe(key(2), "R\u{0430}vi");
    idx.observe(key(3), "Asha");
    let m1 = idx.marker_for(&key(1)).expect("marker on 1");
    let m2 = idx.marker_for(&key(2)).expect("marker on 2");
    assert_eq!(
        m1,
        Marker {
            count: 2,
            others: vec![key(2)]
        }
    );
    assert_eq!(m2.count, 2);
    assert_eq!(m2.others, vec![key(1)]);
    assert_eq!(idx.marker_for(&key(3)), None);
    // every entry with the name shows the same text (D5 R5), built from its
    // own display name
    assert_eq!(m1.text("Ravi"), "2 phones use the name Ravi");
    assert_eq!(m2.text("R\u{0430}vi"), "2 phones use the name R\u{0430}vi");
    for word in ["changed", "new", "old", "impostor", "Impostor"] {
        assert!(!m1.text("Ravi").contains(word), "{word}");
    }
    // a third claimant joins; count updates everywhere
    idx.observe(key(4), "ravi");
    assert_eq!(idx.marker_for(&key(1)).unwrap().count, 3);
    assert_eq!(idx.marker_for(&key(4)).unwrap().count, 3);
    // dropping a key removes it from every marker
    idx.forget(&key(4));
    assert_eq!(idx.marker_for(&key(1)).unwrap().count, 2);
    idx.forget(&key(2));
    assert_eq!(idx.marker_for(&key(1)), None);
    assert_eq!(idx.marker_for(&key(2)), None);
}

#[test]
fn petname_collision_marks_both() {
    let mut idx = SameNameIndex::new();
    idx.observe(key(1), "Ravi");
    idx.observe(key(2), "Kumar");
    assert_eq!(idx.marker_for(&key(1)), None);
    idx.set_petname(key(2), Some("ravi"));
    assert_eq!(idx.marker_for(&key(1)).unwrap().others, vec![key(2)]);
    assert_eq!(idx.marker_for(&key(2)).unwrap().others, vec![key(1)]);
    idx.set_petname(key(2), None);
    assert_eq!(idx.marker_for(&key(1)), None);
}

#[test]
fn silence_pair_is_these_are_different_people() {
    let mut idx = SameNameIndex::new();
    idx.observe(key(1), "Ravi");
    idx.observe(key(2), "Ravi");
    idx.observe(key(3), "Ravi");
    idx.silence_pair(&key(1), &key(2));
    // the silenced pair stops marking each other but a third key still marks both
    assert_eq!(idx.marker_for(&key(1)).unwrap().others, vec![key(3)]);
    assert_eq!(idx.marker_for(&key(2)).unwrap().others, vec![key(3)]);
    assert_eq!(idx.marker_for(&key(3)).unwrap().count, 3);
    idx.forget(&key(3));
    assert_eq!(idx.marker_for(&key(1)), None);
    assert_eq!(idx.marker_for(&key(2)), None);
    // silence survives a rebuild and is symmetric
    let rebuilt = SameNameIndex::with_silenced(idx.silenced());
    assert!(rebuilt.is_silenced(&key(2), &key(1)));
}

#[test]
fn marker_state_is_independent_of_arrival_order() {
    let names = [(1u8, "Ravi"), (2, "ravi."), (3, "Asha"), (4, "R\u{0430}vi")];
    let mut forward = SameNameIndex::new();
    let mut backward = SameNameIndex::new();
    for (i, n) in names {
        forward.observe(key(i), n);
    }
    for (i, n) in names.iter().rev() {
        backward.observe(key(*i), n);
    }
    assert_eq!(forward, backward);
    for (i, _) in names {
        assert_eq!(forward.marker_for(&key(i)), backward.marker_for(&key(i)));
    }
}

// ---- cards ------------------------------------------------------------------

#[test]
fn card_cap_three_names_per_hour_then_aggregate() {
    let mut idx = SameNameIndex::new();
    let t0 = 5 * CARD_WINDOW_MS;
    let mut shown = 0;
    for i in 0..10u32 {
        let name = format!("name{i}");
        if idx.should_show_card(&name, t0 + u64::from(i)) == CardDecision::Show {
            shown += 1;
        }
    }
    assert_eq!(shown, CARD_CAP_PER_HOUR_ASM);
    // later claimants of a shown name join its card
    assert_eq!(idx.should_show_card("name0", t0 + 100), CardDecision::Show);
    // an aggregated name stays aggregated
    assert_eq!(
        idx.should_show_card("name9", t0 + 100),
        CardDecision::Aggregate
    );
    // after an hour of powered-on time a new name gets a card again
    assert_eq!(
        idx.should_show_card("fresh", t0 + CARD_WINDOW_MS + 10),
        CardDecision::Show
    );
}

// ---- pin store --------------------------------------------------------------

#[test]
fn pin_rule_only_scan_message_or_group() {
    let mut kv = MemKv::new();
    let mut store = ContactStore::new(&mut kv);
    let g = GroupId([4; 16]);
    // first contact via a shared group: Unverified
    let c = store
        .first_contact(&key(1), [1; 32], "Ravi", PinSource::SharedGroup(g), 0)
        .unwrap();
    assert_eq!(c.badge, Badge::Unverified);
    assert_eq!(c.badge.text(), "Unverified");
    // 1:1 message: still Unverified
    let c = store.pin(&key(1), PinSource::DirectMessage, 0).unwrap();
    assert_eq!(c.badge, Badge::Unverified);
    assert_eq!(
        c.pins,
        vec![PinSource::SharedGroup(g), PinSource::DirectMessage]
    );
    // scan: Verified in person
    let c = store
        .pin(
            &key(1),
            PinSource::QrScan { at_mono: 12 },
            1_728_000_000_000,
        )
        .unwrap();
    assert_eq!(c.badge.text(), "Verified in person");
    assert!(c.has_scan());
    // an unpinned key cannot receive a pin: the entry must come from one of
    // the three sources, never from hearing the key on the radio
    assert_eq!(
        store.pin(&key(9), PinSource::DirectMessage, 0),
        Err(ContactError::UnknownContact(key(9)))
    );
    assert_eq!(store.list().unwrap().len(), 1);
    // the namespace holds the key bytes, nothing name-derived
    assert!(kv.get(NS_CONTACTS, key(1).as_bytes()).is_some());
    assert!(kv.get(NS_CONTACTS, b"Ravi").is_none());
}

#[test]
fn vouched_is_not_built_at_p0() {
    let mut kv = MemKv::new();
    let mut store = ContactStore::new(&mut kv);
    store.upsert(&contact(1, "Ravi")).unwrap();
    assert_eq!(
        store.vouch(&key(1), &[key(2)]),
        Err(ContactError::NotBuiltAtP0)
    );
    // the type still exists for the §3 table
    let b = Badge::Vouched { via: vec![key(2)] };
    assert_eq!(b.text(), "Vouched");
}

#[test]
fn repin_flow_is_local_and_inherits_nothing() {
    let mut kv = MemKv::new();
    let mut store = ContactStore::new(&mut kv);
    let mut idx = SameNameIndex::new();
    let g = GroupId([8; 16]);
    // Ravi's old phone: verified, petnamed, in group g
    let old = store
        .first_contact(&key(1), [1; 32], "Ravi", PinSource::SharedGroup(g), 0)
        .unwrap();
    store
        .pin(&key(1), PinSource::QrScan { at_mono: 1 }, 1_000)
        .unwrap();
    store.set_petname(&key(1), "Ravi (shelter)").unwrap();
    idx.observe(old.key, "Ravi");
    idx.set_petname(old.key, Some("Ravi (shelter)"));
    // a second key delivers "Ravi" inside a roster update: same-name marker
    let other = store
        .first_contact(&key(2), [2; 32], "Ravi", PinSource::SharedGroup(g), 0)
        .unwrap();
    idx.observe(other.key, "Ravi");
    assert_eq!(idx.marker_for(&key(1)).unwrap().count, 2);
    assert_eq!(idx.marker_for(&key(2)).unwrap().count, 2);
    // the user scans the new phone in person
    let mut scanned = store.get(&key(2)).unwrap().unwrap();
    scanned.pins.push(PinSource::QrScan { at_mono: 500 });
    let out = store
        .repin_after_scan(&key(1), &key(2), scanned, 2_000)
        .unwrap();
    assert_eq!(
        out.new_entry.badge,
        Badge::VerifiedInPerson { at_display: 2_000 }
    );
    assert_eq!(out.new_entry.petname, "", "new entry inherits nothing");
    assert_eq!(out.old_entry.petname, "Ravi (shelter)");
    assert!(out.old_entry.stop_trusting);
    assert_eq!(
        out.old_entry.badge,
        Badge::VerifiedInPerson { at_display: 1_000 }
    );
    assert_eq!(
        out.text(),
        "This is Ravi (shelter)'s phone now (you scanned it today)"
    );
    assert_eq!(
        out.stop_trusting_offer_text(),
        "Stop trusting Ravi (shelter)'s other phone"
    );
    // nothing else in the store changed; two separate entries exist
    let all = store.list().unwrap();
    assert_eq!(all.len(), 2);
    assert_eq!(store.get(&key(1)).unwrap().unwrap(), out.old_entry);
    assert_eq!(store.get(&key(2)).unwrap().unwrap(), out.new_entry);
    // the marker is still symmetric: re-pin does not single out an entry
    assert_eq!(idx.marker_for(&key(1)).unwrap().count, 2);
    assert_eq!(idx.marker_for(&key(2)).unwrap().count, 2);
    // the store knows no network: it only touched the contacts namespace
    assert_eq!(kv.scan_prefix(NS_CONTACTS, &[]).len(), 2);
    assert_eq!(kv.len(), 2);
}

#[test]
fn repin_requires_the_users_own_scan() {
    let mut kv = MemKv::new();
    let mut store = ContactStore::new(&mut kv);
    store.upsert(&contact(1, "Ravi")).unwrap();
    let mut new = contact(2, "Ravi");
    new.pins.push(PinSource::DirectMessage);
    new.pins.push(PinSource::SharedGroup(GroupId([1; 16])));
    assert_eq!(
        store.repin_after_scan(&key(1), &key(2), new, 0),
        Err(ContactError::RequiresScan)
    );
    assert_eq!(store.get(&key(2)).unwrap(), None);
    assert!(!store.get(&key(1)).unwrap().unwrap().stop_trusting);
}

// ---- codec ------------------------------------------------------------------

#[test]
fn contact_codec_round_trip() {
    let mut c = contact(3, "Ravi");
    c.petname = "p".repeat(MAX_PETNAME_BYTES);
    c.badge = Badge::VerifiedInPerson { at_display: -5 };
    c.pins = vec![
        PinSource::QrScan { at_mono: u64::MAX },
        PinSource::DirectMessage,
        PinSource::SharedGroup(GroupId([0xAB; 16])),
    ];
    c.link_key = Some([1; 32]);
    c.envelope_pub = Some([2; 32]);
    c.stop_trusting = true;
    let bytes = c.encode();
    assert_eq!(bytes[0], CONTACT_VER);
    let back: Contact = decode(&bytes).unwrap();
    assert_eq!(back, c);
    assert_eq!(back.encode(), bytes);
    // truncations never panic, always Err
    for n in 0..bytes.len() {
        assert!(decode::<Contact>(&bytes[..n]).is_err(), "prefix {n}");
    }
    let mut t = bytes.clone();
    t.push(0);
    assert_eq!(decode::<Contact>(&t), Err(CodecError::TrailingBytes(1)));
    // a non-admitted key is refused
    let mut bad = bytes.clone();
    bad[1..33].copy_from_slice(&[0xFF; 32]);
    assert!(decode::<Contact>(&bad).is_err());
    // the every-badge table encodes
    for b in [
        Badge::Unverified,
        Badge::Vouched {
            via: {
                let mut v = vec![key(1), key(2), key(3)];
                v.sort();
                v
            },
        },
        Badge::VerifiedInPerson { at_display: 0 },
        Badge::Official,
    ] {
        let bytes = b.encode();
        assert_eq!(decode::<Badge>(&bytes).unwrap(), b);
    }
}

#[test]
fn contact_debug_never_prints_the_link_key_and_is_wiped_on_drop() {
    // `link_key` is `k_pair` (DESIGN §2.3), a symmetric secret: the crate
    // convention for secret-bearing values (pairwise::Paired, SkdmPayload)
    // is a redacting Debug plus ZeroizeOnDrop.
    fn assert_zeroize_on_drop<T: zeroize::ZeroizeOnDrop>() {}
    assert_zeroize_on_drop::<Contact>();
    let mut c = contact(1, "Ravi");
    c.link_key = Some([0xC7; 32]);
    c.envelope_pub = Some([0x3D; 32]);
    let dbg = format!("{c:?}");
    assert!(!dbg.contains("199"), "{dbg}");
    assert!(!dbg.to_lowercase().contains("c7c7"), "{dbg}");
    // public facts stay visible
    assert!(dbg.contains("Ravi"), "{dbg}");
    assert!(dbg.contains("3d3d"), "{dbg}");
}

#[test]
fn duplicate_pins_are_not_a_valid_contact() {
    // Contact doc: pins are "each at most once"; one encoding per value.
    let mut c = contact(2, "Ravi");
    c.pins = vec![PinSource::DirectMessage, PinSource::DirectMessage];
    assert_eq!(c.try_encode(), Err(CodecError::Invalid("pins (duplicate)")));
    // the same blob hand-built: `ver ‖ key ‖ x25519 ‖ petname ‖ display_name ‖
    // badge ‖ pins ‖ link_key ‖ envelope_pub ‖ stop_trusting`
    let mut w = meshcore::codec::Writer::new();
    w.u8(CONTACT_VER);
    w.bytes(c.key.as_bytes());
    w.bytes(&c.x25519);
    w.str16("").unwrap();
    w.str16("Ravi").unwrap();
    w.u8(0); // Unverified
    w.u8(2); // two pins
    w.u8(1); // DirectMessage
    w.u8(1); // DirectMessage again
    w.u8(0); // no link key
    w.u8(0); // no envelope key
    w.bool(false);
    assert_eq!(
        decode::<Contact>(w.as_bytes()),
        Err(CodecError::Invalid("pins (duplicate)"))
    );
    // with one pin the same blob decodes
    let mut w = meshcore::codec::Writer::new();
    w.u8(CONTACT_VER);
    w.bytes(c.key.as_bytes());
    w.bytes(&c.x25519);
    w.str16("").unwrap();
    w.str16("Ravi").unwrap();
    w.u8(0);
    w.u8(1);
    w.u8(1);
    w.u8(0);
    w.u8(0);
    w.bool(false);
    c.pins = vec![PinSource::DirectMessage];
    assert_eq!(decode::<Contact>(w.as_bytes()), Ok(c.clone()));
    c.pins = vec![PinSource::DirectMessage, PinSource::DirectMessage];
    let mut kv = MemKv::new();
    let mut store = ContactStore::new(&mut kv);
    assert_eq!(store.upsert(&c), Err(ContactError::DuplicatePin));
    assert_eq!(store.list().unwrap().len(), 0);
}

fn arb_key() -> impl Strategy<Value = IdentityKey> {
    any::<u8>().prop_map(key)
}

fn arb_pin() -> impl Strategy<Value = PinSource> {
    prop_oneof![
        any::<u64>().prop_map(|at_mono| PinSource::QrScan { at_mono }),
        Just(PinSource::DirectMessage),
        any::<[u8; 16]>().prop_map(|g| PinSource::SharedGroup(GroupId(g))),
    ]
}

fn arb_badge() -> impl Strategy<Value = Badge> {
    prop_oneof![
        Just(Badge::Unverified),
        proptest::collection::btree_set(arb_key(), 0..4).prop_map(|via| Badge::Vouched {
            via: via.into_iter().collect()
        }),
        any::<i64>().prop_map(|at_display| Badge::VerifiedInPerson { at_display }),
        Just(Badge::Official),
    ]
}

fn arb_contact() -> impl Strategy<Value = Contact> {
    (
        arb_key(),
        any::<[u8; 32]>(),
        "[a-zA-Z ]{0,64}",
        "\\PC{0,20}",
        arb_badge(),
        proptest::collection::btree_set(arb_pin(), 0..6).prop_map(|p| p.into_iter().collect()),
        proptest::option::of(any::<[u8; 32]>()),
        proptest::option::of(any::<[u8; 32]>()),
        any::<bool>(),
    )
        .prop_filter("display name within limit", |t| t.3.len() <= 64)
        .prop_map(
            |(
                key,
                x25519,
                petname,
                display_name,
                badge,
                pins,
                link_key,
                envelope_pub,
                stop_trusting,
            )| {
                Contact {
                    key,
                    x25519,
                    petname,
                    display_name,
                    badge,
                    pins,
                    link_key,
                    envelope_pub,
                    stop_trusting,
                }
            },
        )
}

proptest! {
    #[test]
    fn contact_round_trips_for_any_value(c in arb_contact()) {
        let bytes = c.try_encode().unwrap();
        let back: Contact = decode(&bytes).unwrap();
        prop_assert_eq!(&back, &c);
        prop_assert_eq!(back.try_encode().unwrap(), bytes);
    }

    #[test]
    fn contact_decoder_never_panics(bytes in proptest::collection::vec(any::<u8>(), 0..512)) {
        let _ = decode::<Contact>(&bytes);
        let mut r = meshcore::codec::Reader::new(&bytes);
        let _ = Contact::decode_from(&mut r);
    }

    #[test]
    fn normalise_never_panics_and_is_deterministic(s in "\\PC{0,64}") {
        let n = normalise(&s);
        prop_assert_eq!(normalise(&s), n.clone());
        prop_assert!(!n.chars().any(|c| c.is_whitespace() || c.is_ascii_punctuation()));
    }

    #[test]
    fn normalise_is_total_on_any_string(s in any::<String>()) {
        // includes controls, unassigned and noncharacter code points
        let n = normalise(&s);
        prop_assert_eq!(normalise(&s), n.clone());
        prop_assert!(!n.chars().any(|c| c.is_whitespace() || c.is_control()));
    }
}
