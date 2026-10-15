//! Public-API tests for `meshcore::wire` (docs/DESIGN.md §2.2–2.3; §10
//! "cell build/parse incl. TTL draw ∈ {6,7,8}, msg_id excludes TTL, token
//! slot rejection", the `cell_roundtrip` property and the stable-proptest
//! versions of the `cell_parse`, `frame_parse` and `inner_decode` fuzz
//! targets).

use meshcore::codec::{Decode, Encode, decode, encode};
use meshcore::crypto::hash::hkdf32;
use meshcore::wire::cell::{
    BODY_LEN, BODY_PLAINTEXT_LEN, CELL_LEN, Cell, CellReject, HEADER_LEN, HeaderAad, OFF_BODY,
    OFF_TAG, OFF_TOKEN_LEN, OFF_TOKEN_TYPE, OFF_VER, TAG_LEN, TTL_INITIAL_CHOICES, VERSION,
    draw_initial_ttl, open_body, seal_body, seal_cell,
};
use meshcore::wire::frame::{
    CELL_FRAME_LEN, Class, ContactProof, FRAME_HEADER_LEN, Frame, FrameType, Hello, LinkFrame,
    MAX_FRAME_LEN, MAX_SUMMARY_BITS_LEN, SummaryChunk, Throttle,
};
use meshcore::wire::inner::{
    DATA_LEN, INNER_HEADER_LEN, InnerError, InnerHeader, InnerKind, MAX_FRAGS, MAX_OBJECT_LEN,
    Reassembler, fragment,
};
use proptest::prelude::*;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha20Rng;

// ---- helpers ----------------------------------------------------------------

/// A header key the way a chain would derive one: 32 bytes of HKDF output.
fn k_hdr(label: &str) -> [u8; 32] {
    hkdf32(b"wire test chain secret", b"", label.as_bytes())
}

fn plaintext(seed: u8) -> [u8; BODY_PLAINTEXT_LEN] {
    let mut p = [0u8; BODY_PLAINTEXT_LEN];
    for (i, b) in p.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(31).wrapping_add(seed);
    }
    p
}

fn object(len: usize, seed: u64) -> Vec<u8> {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut v = vec![0u8; len];
    rand::RngCore::fill_bytes(&mut rng, &mut v);
    v
}

// ---- cell -------------------------------------------------------------------

#[test]
fn cell_seal_and_open_round_trip_with_a_real_header_key() {
    let k = k_hdr("chain A");
    let tag = [0x7Eu8; TAG_LEN];
    let pt = plaintext(1);
    let c = seal_cell(VERSION, 7, &tag, &k, 1_000, 3, &pt);
    assert_eq!(c.as_bytes().len(), CELL_LEN);
    assert_eq!(c.ver(), VERSION);
    assert_eq!(c.ttl(), 7);
    assert_eq!(c.tag(), tag);
    assert_eq!((c.token_type(), c.token_len()), (0, 0));
    assert!(c.validate().is_ok());
    assert_eq!(open_body(&k, 1_000, 3, &c), Ok(pt));
    assert_eq!(c.open_body(&k, 1_000, 3), Ok(pt));
    // deterministic: same inputs, same bytes
    assert_eq!(c, seal_cell(VERSION, 7, &tag, &k, 1_000, 3, &pt));
    // the body alone is seal_body under the same header
    assert_eq!(
        c.body(),
        &seal_body(&k, 1_000, 3, &HeaderAad::v1(tag), &pt)[..]
    );
    // the ciphertext hides the plaintext and differs per counter/index
    assert_ne!(&c.body()[..BODY_PLAINTEXT_LEN], &pt[..]);
    assert_ne!(
        seal_cell(VERSION, 7, &tag, &k, 1_001, 3, &pt).body(),
        c.body()
    );
    assert_ne!(
        seal_cell(VERSION, 7, &tag, &k, 1_000, 4, &pt).body(),
        c.body()
    );
    // a relayed copy (TTL decremented) still opens and keeps its msg_id
    let relayed = c.forwarded().expect("ttl 7 forwards");
    assert_eq!(relayed.ttl(), 6);
    assert_eq!(open_body(&k, 1_000, 3, &relayed), Ok(pt));
    assert_eq!(relayed.msg_id(), c.msg_id());
    // bytes-level round trip through from_bytes
    let parsed = Cell::from_bytes(c.as_bytes()).expect("512 bytes");
    assert_eq!(parsed, c);
    assert_eq!(open_body(&k, 1_000, 3, &parsed), Ok(pt));
}

#[test]
fn cell_open_fails_on_wrong_ctr_wrong_frag_idx_wrong_key_or_tampering() {
    let k = k_hdr("chain B");
    let tag = [0x42u8; TAG_LEN];
    let pt = plaintext(2);
    let c = seal_cell(VERSION, 8, &tag, &k, 5, 1, &pt);
    assert_eq!(open_body(&k, 6, 1, &c), Err(CellReject::BodyAuthFailed));
    assert_eq!(open_body(&k, 4, 1, &c), Err(CellReject::BodyAuthFailed));
    assert_eq!(open_body(&k, 5, 0, &c), Err(CellReject::BodyAuthFailed));
    assert_eq!(open_body(&k, 5, 2, &c), Err(CellReject::BodyAuthFailed));
    assert_eq!(
        open_body(&k_hdr("chain C"), 5, 1, &c),
        Err(CellReject::BodyAuthFailed)
    );
    // every header byte except the TTL is authenticated
    for off in (0..HEADER_LEN).filter(|&o| o != 1) {
        let mut raw = *c.as_bytes();
        raw[off] ^= 0x01;
        let t = Cell::from_array(raw);
        assert_eq!(
            open_body(&k, 5, 1, &t),
            Err(CellReject::BodyAuthFailed),
            "header byte {off}"
        );
    }
    // and so is every body byte
    for off in [OFF_BODY, OFF_BODY + 100, CELL_LEN - 17, CELL_LEN - 1] {
        let mut raw = *c.as_bytes();
        raw[off] ^= 0x01;
        assert_eq!(
            open_body(&k, 5, 1, &Cell::from_array(raw)),
            Err(CellReject::BodyAuthFailed),
            "body byte {off}"
        );
    }
    // a seal with the right key but a different header AAD does not open
    // as this cell: the receiver's AAD comes from the cell itself
    let other_header = HeaderAad::new(2, tag);
    let body = seal_body(&k, 5, 1, &other_header, &pt);
    let mixed = meshcore::wire::cell::build(VERSION, 8, &tag, &body);
    assert_eq!(open_body(&k, 5, 1, &mixed), Err(CellReject::BodyAuthFailed));
    let honest = meshcore::wire::cell::build(2, 8, &tag, &body);
    assert_eq!(open_body(&k, 5, 1, &honest), Ok(pt));
}

#[test]
fn cell_msg_id_excludes_only_the_ttl_and_validate_applies_v1_rules() {
    let k = k_hdr("chain D");
    let c = seal_cell(VERSION, 6, &[1u8; TAG_LEN], &k, 0, 0, &plaintext(3));
    let mut lower = c.clone();
    lower.set_ttl(1);
    assert_eq!(lower.msg_id(), c.msg_id());
    assert_ne!(lower, c);
    for off in [
        OFF_VER,
        OFF_TOKEN_TYPE,
        OFF_TOKEN_LEN,
        OFF_TAG,
        OFF_BODY,
        CELL_LEN - 1,
    ] {
        let mut raw = *c.as_bytes();
        raw[off] ^= 1;
        assert_ne!(Cell::from_array(raw).msg_id(), c.msg_id(), "byte {off}");
    }
    let mut token = *c.as_bytes();
    token[OFF_TOKEN_TYPE] = 1;
    assert_eq!(
        Cell::from_array(token).validate(),
        Err(CellReject::TokenNotEmpty)
    );
    let mut ttl = *c.as_bytes();
    ttl[1] = 9;
    assert_eq!(Cell::from_array(ttl).validate(), Err(CellReject::BadTtl(9)));
    assert_eq!(
        Cell::from_bytes(&[0u8; CELL_LEN + 1]),
        Err(CellReject::BadLength(CELL_LEN + 1))
    );
    assert_eq!(BODY_LEN, 492);
    assert_eq!(BODY_PLAINTEXT_LEN, 476);
}

#[test]
fn ttl_draw_distribution_over_3000_draws_covers_only_6_7_8() {
    let mut rng = ChaCha20Rng::seed_from_u64(0x7718_2026);
    let mut counts = [0usize; 3];
    for _ in 0..3000 {
        let t = draw_initial_ttl(&mut rng);
        let i = TTL_INITIAL_CHOICES
            .iter()
            .position(|&c| c == t)
            .unwrap_or_else(|| panic!("ttl {t} is not in {TTL_INITIAL_CHOICES:?}"));
        counts[i] += 1;
    }
    assert_eq!(counts.iter().sum::<usize>(), 3000);
    for (i, &n) in counts.iter().enumerate() {
        // expected 1000 each, σ ≈ 26; the seed is fixed so this is exact,
        // and a modulo bias of 1/3 would show up as ~1333 vs ~833
        assert!(
            (850..=1150).contains(&n),
            "ttl {} drawn {n} times of 3000: {counts:?}",
            TTL_INITIAL_CHOICES[i]
        );
    }
    // the draw is a pure function of the RNG state
    let mut a = ChaCha20Rng::seed_from_u64(9);
    let mut b = ChaCha20Rng::seed_from_u64(9);
    let xs: Vec<u8> = (0..50).map(|_| draw_initial_ttl(&mut a)).collect();
    let ys: Vec<u8> = (0..50).map(|_| draw_initial_ttl(&mut b)).collect();
    assert_eq!(xs, ys);
}

// ---- inner: fragment / reassemble -------------------------------------------

#[test]
fn fragment_sizes_and_the_64k_cap() {
    assert_eq!(DATA_LEN, 465);
    assert_eq!(MAX_FRAGS, 141);
    assert_eq!(MAX_OBJECT_LEN, 65_565);
    for (len, n) in [(1, 1), (465, 1), (466, 2), (10_000, 22), (65_565, 141)] {
        let frags = fragment(InnerKind::SenderKey, 1, &object(len, 1)).expect("fits");
        assert_eq!(frags.len(), n, "{len} bytes");
    }
    assert_eq!(
        fragment(InnerKind::SenderKey, 1, &object(65_566, 1)),
        Err(InnerError::ObjectTooLarge {
            len: 65_566,
            max: 65_565
        })
    );
    assert_eq!(
        fragment(InnerKind::SenderKey, 1, &[]),
        Err(InnerError::EmptyObject)
    );
}

/// Pushes `order` (indices into `frags`, duplicates allowed) and checks that
/// the object comes out exactly when the last distinct fragment arrives and
/// byte-identical; a duplicate before that is ignored. The reassembler is
/// not a dedupe layer, so once an object is complete a second full set of
/// copies completes it again, and a one-fragment object completes on every
/// copy; the dedupe cache in front of it is what stops that.
fn push_in_order(
    frags: &[[u8; BODY_PLAINTEXT_LEN]],
    order: &[usize],
    chain: u64,
    kind: InnerKind,
    obj_seq: u32,
    expect: &[u8],
) {
    let mut r = Reassembler::default();
    let mut seen = vec![false; frags.len()];
    let mut distinct = 0;
    let mut completions = 0;
    for &i in order {
        let res = r.push(chain, &frags[i]).expect("fragment is valid");
        if !seen[i] {
            seen[i] = true;
            distinct += 1;
        }
        if distinct == frags.len() {
            assert_eq!(res, Some((kind, obj_seq, expect.to_vec())));
            assert_eq!(r.in_progress(), 0);
            completions += 1;
            seen.fill(false);
            distinct = 0;
        } else {
            assert_eq!(res, None, "fragment {i} of {}", frags.len());
            assert_eq!(r.in_progress(), 1);
        }
    }
    assert!(completions >= 1, "the object never completed");
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(96))]

    /// DESIGN §10 `cell_roundtrip`: objects fragment and reassemble through
    /// random cell orderings and duplicates.
    #[test]
    fn fragment_reassemble_round_trip_through_random_order_and_duplicates(
        len in prop::sample::select(vec![1usize, 465, 466, 10_000, 65_565]),
        seed in any::<u64>(),
        kind in prop::sample::select(InnerKind::ALL.to_vec()),
        obj_seq in any::<u32>(),
        chain in any::<u64>(),
        n_dups in 0usize..8,
    ) {
        let obj = object(len, seed);
        let frags = fragment(kind, obj_seq, &obj).expect("fits");
        let mut rng = ChaCha20Rng::seed_from_u64(seed ^ 0xD1CE);
        let mut order: Vec<usize> = (0..frags.len()).collect();
        order.shuffle(&mut rng);
        for _ in 0..n_dups {
            let pick = order[rand::Rng::random_range(&mut rng, 0..order.len())];
            let at = rand::Rng::random_range(&mut rng, 0..=order.len());
            order.insert(at, pick);
        }
        push_in_order(&frags, &order, chain, kind, obj_seq, &obj);
        // fragmentation is deterministic
        prop_assert_eq!(fragment(kind, obj_seq, &obj).unwrap(), frags);
    }

    #[test]
    fn objects_between_sizes_round_trip_in_order(
        len in 1usize..=3000,
        seed in any::<u64>(),
    ) {
        let obj = object(len, seed);
        let frags = fragment(InnerKind::PairwiseDr, 7, &obj).unwrap();
        prop_assert_eq!(frags.len(), len.div_ceil(DATA_LEN));
        let order: Vec<usize> = (0..frags.len()).collect();
        push_in_order(&frags, &order, 3, InnerKind::PairwiseDr, 7, &obj);
    }
}

#[test]
fn reassembler_bounds_in_progress_objects_and_evicts_oldest() {
    let mut r = Reassembler::new(3);
    assert_eq!(r.max_in_progress(), 3);
    let firsts: Vec<Vec<[u8; BODY_PLAINTEXT_LEN]>> = (0..5u32)
        .map(|s| fragment(InnerKind::SkdmBundle, s, &object(1000, u64::from(s))).unwrap())
        .collect();
    for (s, f) in firsts.iter().enumerate() {
        assert_eq!(r.push(1, &f[0]), Ok(None));
        assert!(r.in_progress() <= 3, "after object {s}");
    }
    assert_eq!(r.in_progress(), 3);
    assert!(!r.contains(1, 0) && !r.contains(1, 1), "oldest two evicted");
    assert!(r.contains(1, 2) && r.contains(1, 3) && r.contains(1, 4));
    // the surviving objects still complete
    for s in 2..5u32 {
        let f = &firsts[s as usize];
        assert_eq!(r.push(1, &f[1]), Ok(None));
        assert_eq!(
            r.push(1, &f[2]),
            Ok(Some((InnerKind::SkdmBundle, s, object(1000, u64::from(s)))))
        );
    }
    assert_eq!(r.in_progress(), 0);
    // a flood of first fragments never grows past the bound
    let mut flood = Reassembler::new(16);
    for s in 0..2000u32 {
        let f = fragment(InnerKind::Receipt, s, &object(466, 0)).unwrap();
        assert_eq!(flood.push(s as u64 % 7, &f[0]), Ok(None));
        assert!(flood.in_progress() <= 16);
    }
    assert_eq!(flood.in_progress(), 16);
    // and a 141-fragment object held open costs only what has arrived
    let big = fragment(InnerKind::SenderKey, 1, &object(MAX_OBJECT_LEN, 5)).unwrap();
    let mut one = Reassembler::new(1);
    for f in &big[..140] {
        assert_eq!(one.push(9, f), Ok(None));
    }
    assert_eq!(
        one.push(9, &big[140]).map(|o| o.map(|(_, _, b)| b.len())),
        Ok(Some(MAX_OBJECT_LEN))
    );
}

#[test]
fn reassembler_rejects_inconsistent_fragments_and_bad_headers() {
    let mut r = Reassembler::default();
    let a = fragment(InnerKind::SenderKey, 1, &object(1000, 1)).unwrap(); // 3 frags
    let b = fragment(InnerKind::SenderKey, 1, &object(2000, 2)).unwrap(); // 5 frags
    assert_eq!(r.push(1, &a[0]), Ok(None));
    assert_eq!(r.push(1, &b[1]), Err(InnerError::Mismatch));
    assert!(!r.contains(1, 1));
    let k = fragment(InnerKind::Receipt, 1, &object(1000, 1)).unwrap();
    assert_eq!(r.push(1, &a[0]), Ok(None));
    assert_eq!(r.push(1, &k[1]), Err(InnerError::Mismatch));
    assert!(!r.contains(1, 1));
    // hand-written bad headers
    let hdr = |kind: u8, idx: u16, total: u16, len: u16| {
        let mut p = [0u8; BODY_PLAINTEXT_LEN];
        p[0] = kind;
        p[1..5].copy_from_slice(&1u32.to_le_bytes());
        p[5..7].copy_from_slice(&idx.to_le_bytes());
        p[7..9].copy_from_slice(&total.to_le_bytes());
        p[9..11].copy_from_slice(&len.to_le_bytes());
        p
    };
    assert!(matches!(
        r.push(1, &hdr(0x00, 0, 1, 1)),
        Err(InnerError::Codec(_))
    ));
    assert_eq!(
        r.push(1, &hdr(0x01, 0, 0, 1)),
        Err(InnerError::BadFragTotal(0))
    );
    assert_eq!(
        r.push(1, &hdr(0x01, 0, 142, 465)),
        Err(InnerError::BadFragTotal(142))
    );
    assert_eq!(
        r.push(1, &hdr(0x01, 2, 2, 1)),
        Err(InnerError::BadFragIdx { idx: 2, total: 2 })
    );
    assert_eq!(
        r.push(1, &hdr(0x01, 0, 2, 464)),
        Err(InnerError::BadLen {
            len: 464,
            is_final: false
        })
    );
    assert_eq!(
        r.push(1, &hdr(0x01, 1, 2, 0)),
        Err(InnerError::BadLen {
            len: 0,
            is_final: true
        })
    );
    assert_eq!(
        r.push(1, &hdr(0x01, 0, 1, 466)),
        Err(InnerError::BadLen {
            len: 466,
            is_final: true
        })
    );
    let mut padded = hdr(0x01, 0, 1, 1);
    padded[INNER_HEADER_LEN + 5] = 1;
    assert_eq!(r.push(1, &padded), Err(InnerError::NonZeroPadding));
    assert_eq!(r.in_progress(), 0);
}

#[test]
fn reassembler_rejects_a_single_fragment_object_that_contradicts_an_open_partial() {
    // "mismatched frag_total/kind within one object → reject that object"
    // applies to a frag_total of 1 as well: the key (chain, obj_seq) is the
    // object, whatever its fragment count.
    let mut r = Reassembler::default();
    let multi = fragment(InnerKind::Receipt, 42, &object(1500, 3)).unwrap(); // 4 frags
    let single = fragment(InnerKind::Receipt, 42, &object(64, 4)).unwrap(); // 1 frag
    assert_eq!(r.push(9, &multi[3]), Ok(None));
    assert_eq!(r.push(9, &multi[0]), Ok(None));
    assert_eq!(r.push(9, &single[0]), Err(InnerError::Mismatch));
    assert!(!r.contains(9, 42));
    assert_eq!(r.in_progress(), 0);
    // a kind mismatch on a single fragment is a mismatch too
    let other_kind = fragment(InnerKind::Sos, 42, &object(64, 4)).unwrap();
    assert_eq!(r.push(9, &multi[1]), Ok(None));
    assert_eq!(r.push(9, &other_kind[0]), Err(InnerError::Mismatch));
    assert_eq!(r.in_progress(), 0);
    // nothing open: the single-fragment object completes
    assert_eq!(
        r.push(9, &single[0]),
        Ok(Some((InnerKind::Receipt, 42, object(64, 4))))
    );
}

/// Pins the body AEAD derivation to fixed bytes so that an accidental change
/// to the key info, the nonce layout or the AAD (which `wire/recog.rs` and
/// any second implementation must match byte for byte) fails here and not
/// only in a cross-module test.
#[test]
fn cell_body_known_answer() {
    use meshcore::wire::cell::{body_nonce, cell_key};
    let k: [u8; 32] = core::array::from_fn(|i| i as u8);
    assert_eq!(
        hex::encode(cell_key(&k, 0x0102_0304_0506_0708).as_bytes()),
        hex::encode(hkdf32(
            &k,
            b"",
            b"meshapp.v1.cell\x08\x07\x06\x05\x04\x03\x02\x01"
        ))
    );
    assert_eq!(
        body_nonce(0xBEEF),
        [0xEF, 0xBE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    let pt: [u8; BODY_PLAINTEXT_LEN] = core::array::from_fn(|i| (i % 7) as u8);
    let c = seal_cell(VERSION, 6, &[0xA5; TAG_LEN], &k, 1, 2, &pt);
    assert_eq!(
        hex::encode(&c.as_bytes()[..HEADER_LEN]),
        "01060000a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5"
    );
    assert_eq!(hex::encode(c.msg_id()), MSG_ID_KAT);
    assert_eq!(hex::encode(&c.body()[..16]), BODY_PREFIX_KAT);
    assert_eq!(hex::encode(&c.body()[BODY_LEN - 16..]), BODY_TAG_KAT);
}
const MSG_ID_KAT: &str = "fd385eaf5d852f38bed992cdcf2673062181e08cd94c19058a0044952fd5a1a5";
const BODY_PREFIX_KAT: &str = "dabad83dcfe05c1f9916a96845246670";
const BODY_TAG_KAT: &str = "8879326ed5f3b31dd2f62d24510ecd44";

// ---- frames ------------------------------------------------------------------

#[test]
fn frames_round_trip_through_bytes_and_the_typed_enum() {
    let c = seal_cell(
        VERSION,
        7,
        &[3u8; TAG_LEN],
        &k_hdr("f"),
        1,
        0,
        &plaintext(4),
    );
    let frames = vec![
        Frame::Cell {
            class: Class::C1Control,
            cell: c.clone(),
        },
        Frame::Hello(Hello {
            proto: 1,
            caps: 0b11,
            is_station: false,
            buffer_free: 5_000_000,
        }),
        Frame::Summary(SummaryChunk {
            salt: 0xDEAD_BEEF,
            n_bits: 1 << 20,
            k: 7,
            chunk_idx: 3,
            chunk_total: 32,
            bits: vec![0x55; MAX_SUMMARY_BITS_LEN],
        }),
        Frame::Throttle(Throttle::for_class(Class::C4Preview, 30_000)),
        Frame::ContactProof(ContactProof {
            proofs: vec![[9u8; 16]; 3],
        }),
    ];
    for f in &frames {
        let bytes = encode(f);
        assert_eq!(bytes[2], 0, "flags");
        assert_eq!(bytes[3], 0, "reserved");
        assert_eq!(bytes[0], f.frame_type().as_u8());
        assert!(bytes.len() <= MAX_FRAME_LEN);
        let raw: LinkFrame = decode(&bytes).expect("raw decode");
        assert_eq!(raw.encoded_len(), bytes.len());
        assert_eq!(encode(&raw), bytes);
        let typed: Frame = decode(&bytes).expect("typed decode");
        assert_eq!(&typed, f);
        assert_eq!(Frame::from_link_frame(&raw), Ok(typed));
        assert_eq!(f.to_link_frame(), Ok(raw));
        // same value, same bytes
        assert_eq!(encode(f), bytes);
    }
    let cell_bytes = encode(&frames[0]);
    assert_eq!(cell_bytes.len(), CELL_FRAME_LEN);
    assert_eq!(&cell_bytes[FRAME_HEADER_LEN..], c.as_bytes());
    assert_eq!(cell_bytes[1], Class::C1Control.as_u8());
    assert_eq!(FrameType::ALL.len(), 5);
}

// ---- parsers survive arbitrary bytes ----------------------------------------

/// Decodes `T` from `bytes` and, if it decodes, checks that the encoding is
/// canonical (re-encoding gives the same bytes).
fn parse_canonical<T: Encode + Decode>(bytes: &[u8]) {
    if let Ok(v) = decode::<T>(bytes) {
        assert_eq!(encode(&v), bytes, "non-canonical encoding accepted");
    }
}

fn random_cell_frame(bytes: &[u8], class: u8, flags: u8, reserved: u8) -> Vec<u8> {
    let mut v = vec![0u8, class, flags, reserved];
    v.extend_from_slice(bytes);
    v
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    #[test]
    fn every_parser_survives_arbitrary_bytes(bytes in prop::collection::vec(any::<u8>(), 0..600)) {
        parse_canonical::<LinkFrame>(&bytes);
        parse_canonical::<Frame>(&bytes);
        parse_canonical::<Hello>(&bytes);
        parse_canonical::<SummaryChunk>(&bytes);
        parse_canonical::<Throttle>(&bytes);
        parse_canonical::<ContactProof>(&bytes);
        parse_canonical::<InnerHeader>(&bytes);
        parse_canonical::<FrameType>(&bytes);
        parse_canonical::<Class>(&bytes);
        if let Ok(c) = Cell::from_bytes(&bytes) {
            prop_assert_eq!(bytes.len(), CELL_LEN);
            let _ = c.validate();
            let _ = c.msg_id();
            let _ = c.forwarded();
            let _ = c.header_aad();
            let _ = open_body(&[0u8; 32], 0, 0, &c);
        } else {
            prop_assert_ne!(bytes.len(), CELL_LEN);
        }
    }

    /// Frames with a plausible header and a random payload, up to the cap
    /// and a little past it.
    #[test]
    fn link_frame_parser_survives_random_headers_and_payloads(
        frame_type in 0u8..8,
        class in 0u8..8,
        flags in prop::sample::select(vec![0u8, 1, 2, 0x80]),
        reserved in prop::sample::select(vec![0u8, 1, 0xFF]),
        payload in prop::collection::vec(any::<u8>(), 0..(MAX_FRAME_LEN + 64)),
    ) {
        let mut bytes = vec![frame_type, class, flags, reserved];
        bytes.extend_from_slice(&payload);
        parse_canonical::<LinkFrame>(&bytes);
        parse_canonical::<Frame>(&bytes);
        let accepted = decode::<LinkFrame>(&bytes).is_ok();
        if flags != 0 || reserved != 0 || frame_type > 4 || bytes.len() > MAX_FRAME_LEN {
            prop_assert!(!accepted);
        }
        if accepted && frame_type != 0 {
            prop_assert_eq!(class, 0);
        }
    }

    #[test]
    fn cell_frames_with_random_cells_decode_only_when_well_formed(
        cell in prop::collection::vec(any::<u8>(), CELL_LEN - 1..=CELL_LEN + 1),
        class in 0u8..6,
        flags in 0u8..2,
        reserved in 0u8..2,
    ) {
        let bytes = random_cell_frame(&cell, class, flags, reserved);
        let ok = cell.len() == CELL_LEN && class <= 4 && flags == 0 && reserved == 0;
        let raw = decode::<LinkFrame>(&bytes);
        prop_assert_eq!(raw.is_ok(), ok);
        let typed = decode::<Frame>(&bytes);
        prop_assert_eq!(typed.is_ok(), ok);
        if let Ok(Frame::Cell { class: c, cell: parsed }) = typed {
            prop_assert_eq!(c.as_u8(), class);
            prop_assert_eq!(parsed.as_bytes().to_vec(), cell);
            let _ = parsed.validate();
        }
    }

    #[test]
    fn inner_parser_and_reassembler_survive_arbitrary_plaintexts(
        plain in prop::collection::vec(any::<u8>(), BODY_PLAINTEXT_LEN),
        chain in any::<u64>(),
    ) {
        let arr: [u8; BODY_PLAINTEXT_LEN] = plain.as_slice().try_into().unwrap();
        let _ = InnerHeader::parse(&arr);
        let mut r = Reassembler::new(4);
        let _ = r.push(chain, &arr);
        prop_assert!(r.in_progress() <= 4);
    }

    /// Plaintexts whose header is in range exercise the reassembler's
    /// bookkeeping (mismatches, duplicates, eviction) with random data.
    #[test]
    fn reassembler_survives_plausible_headers(
        frags in prop::collection::vec(
            (
                prop::sample::select(vec![1u8, 2, 3, 4, 5, 6, 7]),
                0u32..3,
                0u16..4,
                0u16..4,
                prop::sample::select(vec![0u16, 1, 100, 464, 465, 466]),
                any::<u8>(),
                0u64..2,
            ),
            1..40,
        ),
    ) {
        let mut r = Reassembler::new(3);
        for (kind, seq, idx, total, len, fill, chain) in frags {
            let mut p = [fill; BODY_PLAINTEXT_LEN];
            p[0] = kind;
            p[1..5].copy_from_slice(&seq.to_le_bytes());
            p[5..7].copy_from_slice(&idx.to_le_bytes());
            p[7..9].copy_from_slice(&total.to_le_bytes());
            p[9..11].copy_from_slice(&len.to_le_bytes());
            let data_end = INNER_HEADER_LEN + usize::from(len).min(DATA_LEN);
            for b in &mut p[data_end..] {
                *b = 0;
            }
            if let Ok(Some((k, s, bytes))) = r.push(chain, &p) {
                prop_assert_eq!(k.as_u8(), kind);
                prop_assert_eq!(s, seq);
                prop_assert!(!bytes.is_empty());
                prop_assert!(bytes.len() <= usize::from(total) * DATA_LEN);
            }
            prop_assert!(r.in_progress() <= 3);
        }
    }

    #[test]
    fn fragment_never_panics_on_any_length(
        len in prop::sample::select(vec![0usize, 1, 464, 465, 466, 65_564, 65_565, 65_566, 70_000]),
        kind in prop::sample::select(InnerKind::ALL.to_vec()),
        obj_seq in any::<u32>(),
    ) {
        let obj = vec![0xA5u8; len];
        match fragment(kind, obj_seq, &obj) {
            Ok(frags) => {
                prop_assert!((1..=MAX_OBJECT_LEN).contains(&len));
                prop_assert_eq!(frags.len(), len.div_ceil(DATA_LEN));
            }
            Err(InnerError::EmptyObject) => prop_assert_eq!(len, 0),
            Err(InnerError::ObjectTooLarge { .. }) => prop_assert!(len > MAX_OBJECT_LEN),
            Err(e) => prop_assert!(false, "unexpected error {e}"),
        }
    }

    #[test]
    fn control_payloads_round_trip(
        hello in (any::<u8>(), any::<u16>(), any::<bool>(), any::<u32>()),
        throttle in (any::<u8>(), any::<u32>()),
        summary in (any::<u32>(), any::<u32>(), any::<u8>(), 0u16..8, prop::collection::vec(any::<u8>(), 0..64)),
        proofs in prop::collection::vec(any::<[u8; 16]>(), 0..=255),
    ) {
        let h = Hello { proto: hello.0, caps: hello.1, is_station: hello.2, buffer_free: hello.3 };
        prop_assert_eq!(decode::<Frame>(&encode(&LinkFrame::hello(&h))), Ok(Frame::Hello(h)));
        let t = Throttle { class: throttle.0, retry_ms: throttle.1 };
        prop_assert_eq!(decode::<Frame>(&encode(&LinkFrame::throttle(&t))), Ok(Frame::Throttle(t)));
        let s = SummaryChunk {
            salt: summary.0,
            n_bits: summary.1,
            k: summary.2,
            chunk_idx: summary.3,
            chunk_total: summary.3 + 1,
            bits: summary.4,
        };
        let sf = LinkFrame::summary(&s).unwrap();
        prop_assert_eq!(decode::<Frame>(&encode(&sf)), Ok(Frame::Summary(s)));
        let p = ContactProof { proofs };
        let pf = LinkFrame::contact_proof(&p).unwrap();
        prop_assert_eq!(decode::<Frame>(&encode(&pf)), Ok(Frame::ContactProof(p)));
    }

    #[test]
    fn sealed_cells_round_trip_for_any_key_counter_and_index(
        k in any::<[u8; 32]>(),
        ctr in any::<u64>(),
        frag_idx in any::<u16>(),
        ttl in prop::sample::select(TTL_INITIAL_CHOICES.to_vec()),
        tag in any::<[u8; 16]>(),
        fill in any::<u8>(),
    ) {
        let pt = plaintext(fill);
        let c = seal_cell(VERSION, ttl, &tag, &k, ctr, frag_idx, &pt);
        prop_assert!(c.validate().is_ok());
        prop_assert_eq!(open_body(&k, ctr, frag_idx, &c), Ok(pt));
        prop_assert_eq!(open_body(&k, ctr.wrapping_add(1), frag_idx, &c), Err(CellReject::BodyAuthFailed));
        prop_assert_eq!(open_body(&k, ctr, frag_idx.wrapping_add(1), &c), Err(CellReject::BodyAuthFailed));
    }
}
