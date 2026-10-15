//! Public-API tests for `meshcore::codec` (docs/DESIGN.md §2.1, §10 "codec
//! round-trip + rejection of trailing/overlong input").

use meshcore::codec::{
    CodecError, Decode, Encode, LIST_PREALLOC_CAP, ListItem, MAX_LEN16, MAX_LIST_U8, Reader,
    Writer, decode, encode, try_encode, unknown_variant,
};
use proptest::prelude::*;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

// ---- allocation observer ---------------------------------------------------

/// Counts allocations of at least [`BIG_ALLOC`] bytes made on a thread that
/// has switched [`WATCH`] on, so a test can check that a decoder never
/// reserves memory the input could not fill (BRIEF_v2 §5.5 "checks format,
/// size"; DESIGN.md §2.1 fuzzable parser).
struct CountingAlloc;

const BIG_ALLOC: usize = 1 << 20;
static BIG_ALLOCS: AtomicUsize = AtomicUsize::new(0);
thread_local! {
    static WATCH: Cell<bool> = const { Cell::new(false) };
}

// Every call is forwarded to `System` unchanged; only a counter is touched.
// `unsafe` is required by the trait, not by anything done here.
unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() >= BIG_ALLOC && WATCH.with(Cell::get) {
            BIG_ALLOCS.fetch_add(1, Ordering::SeqCst);
        }
        // SAFETY: same contract as the caller's.
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: `ptr` came from `System.alloc` with this `layout`.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: CountingAlloc = CountingAlloc;

/// Runs `f` with big-allocation counting on for this thread and returns how
/// many allocations of at least [`BIG_ALLOC`] bytes it made.
fn big_allocs_during(f: impl FnOnce()) -> usize {
    let before = BIG_ALLOCS.load(Ordering::SeqCst);
    WATCH.with(|w| w.set(true));
    f();
    WATCH.with(|w| w.set(false));
    BIG_ALLOCS.load(Ordering::SeqCst) - before
}

// ---- a sample object family in the §2.4 style -----------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Create,
    Take,
    Release,
    Done,
}

impl Op {
    fn to_u8(self) -> u8 {
        match self {
            Op::Create => 1,
            Op::Take => 2,
            Op::Release => 3,
            Op::Done => 4,
        }
    }
    fn from_u8(v: u8) -> Result<Self, CodecError> {
        Ok(match v {
            1 => Op::Create,
            2 => Op::Take,
            3 => Op::Release,
            4 => Op::Done,
            other => return Err(unknown_variant("task op", other)),
        })
    }
}

/// `admin [32] ‖ sig [64]` style entry.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CoSig {
    admin: [u8; 32],
    sig: [u8; 64],
}

impl Encode for CoSig {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.bytes(&self.admin).bytes(&self.sig);
        Ok(())
    }
}
impl Decode for CoSig {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(CoSig {
            admin: r.bytes()?,
            sig: r.bytes()?,
        })
    }
}
impl ListItem for CoSig {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Envelope {
    ver: u8,
    op: Op,
    task_id: [u8; 16],
    seq: u32,
    heads: Vec<[u8; 32]>,
    title: String,
    details: String,
    ct: Vec<u8>,
    cosigs: Vec<CoSig>,
    labels: Vec<String>,
    delta: i64,
    stamp: u64,
    flag: bool,
}

impl Encode for Envelope {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(self.ver)
            .u8(self.op.to_u8())
            .bytes(&self.task_id)
            .u32(self.seq);
        self.heads.encode_to(w)?;
        w.str16(&self.title)?;
        self.details.encode_to(w)?;
        self.ct.encode_to(w)?;
        self.cosigs.encode_to(w)?;
        self.labels.encode_to(w)?;
        w.i64(self.delta).u64(self.stamp).bool(self.flag);
        Ok(())
    }
}

impl Decode for Envelope {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let ver = r.u8()?;
        if ver != 1 {
            return Err(unknown_variant("envelope ver", ver));
        }
        Ok(Envelope {
            ver,
            op: Op::from_u8(r.u8()?)?,
            task_id: r.bytes()?,
            seq: r.u32()?,
            heads: Decode::decode_from(r)?,
            title: r.str16()?.to_owned(),
            details: Decode::decode_from(r)?,
            ct: Decode::decode_from(r)?,
            cosigs: Decode::decode_from(r)?,
            labels: Decode::decode_from(r)?,
            delta: r.i64()?,
            stamp: r.u64()?,
            flag: r.bool()?,
        })
    }
}

fn op_strategy() -> impl Strategy<Value = Op> {
    prop_oneof![
        Just(Op::Create),
        Just(Op::Take),
        Just(Op::Release),
        Just(Op::Done)
    ]
}

fn string_strategy(max_chars: usize) -> impl Strategy<Value = String> {
    prop::collection::vec(any::<char>(), 0..=max_chars).prop_map(String::from_iter)
}

fn cosig_strategy() -> impl Strategy<Value = CoSig> {
    (any::<[u8; 32]>(), prop::collection::vec(any::<u8>(), 64)).prop_map(|(admin, sig)| CoSig {
        admin,
        sig: sig.try_into().unwrap(),
    })
}

fn envelope_strategy() -> impl Strategy<Value = Envelope> {
    (
        op_strategy(),
        any::<[u8; 16]>(),
        any::<u32>(),
        prop::collection::vec(any::<[u8; 32]>(), 0..=6),
        string_strategy(64),
        string_strategy(300),
        prop::collection::vec(any::<u8>(), 0..=1024),
        prop::collection::vec(cosig_strategy(), 0..=4),
        prop::collection::vec(string_strategy(8), 0..=8),
        (any::<i64>(), any::<u64>(), any::<bool>()),
    )
        .prop_map(
            |(
                op,
                task_id,
                seq,
                heads,
                title,
                details,
                ct,
                cosigs,
                labels,
                (delta, stamp, flag),
            )| {
                Envelope {
                    ver: 1,
                    op,
                    task_id,
                    seq,
                    heads,
                    title,
                    details,
                    ct,
                    cosigs,
                    labels,
                    delta,
                    stamp,
                    flag,
                }
            },
        )
}

fn fixed_envelope() -> Envelope {
    Envelope {
        ver: 1,
        op: Op::Take,
        task_id: [0x42; 16],
        seq: 7,
        heads: vec![[1; 32]],
        title: "Water run".to_string(),
        details: "north gate, 2 people".to_string(),
        ct: vec![0xde, 0xad],
        cosigs: vec![CoSig {
            admin: [3; 32],
            sig: [4; 64],
        }],
        labels: vec!["a".to_string(), "b".to_string()],
        delta: -5,
        stamp: 123_456,
        flag: false,
    }
}

// ---- deterministic tests ---------------------------------------------------

#[test]
fn envelope_round_trips_and_is_deterministic() {
    let e = fixed_envelope();
    let bytes = encode(&e);
    assert_eq!(bytes, e.clone().try_encode().unwrap());
    assert_eq!(bytes, encode(&e.clone()));
    let back: Envelope = decode(&bytes).unwrap();
    assert_eq!(back, e);
    assert_eq!(encode(&back), bytes);
}

#[test]
fn envelope_known_answer() {
    let bytes = encode(&fixed_envelope());
    let mut expect = vec![1u8, 2];
    expect.extend_from_slice(&[0x42; 16]);
    expect.extend_from_slice(&7u32.to_le_bytes());
    expect.push(1);
    expect.extend_from_slice(&[1; 32]);
    expect.extend_from_slice(&[9, 0]);
    expect.extend_from_slice(b"Water run");
    expect.extend_from_slice(&[20, 0]);
    expect.extend_from_slice(b"north gate, 2 people");
    expect.extend_from_slice(&[2, 0, 0xde, 0xad]);
    expect.push(1);
    expect.extend_from_slice(&[3; 32]);
    expect.extend_from_slice(&[4; 64]);
    expect.extend_from_slice(&[2, 1, 0, b'a', 1, 0, b'b']);
    expect.extend_from_slice(&(-5i64).to_le_bytes());
    expect.extend_from_slice(&123_456u64.to_le_bytes());
    expect.push(0);
    assert_eq!(bytes, expect);
}

#[test]
fn primitives_round_trip_through_public_api() {
    macro_rules! rt {
        ($($v:expr),* $(,)?) => {$(
            let v = $v;
            let b = encode(&v);
            assert_eq!(decode(&b), Ok(v.clone()));
        )*};
    }
    rt!(
        0u8,
        255u8,
        0x1234u16,
        0xdead_beefu32,
        u64::MAX,
        i64::MIN,
        true,
        [7u8; 32],
        [0u8; 64],
        String::from("ünïcödé"),
        vec![1u8, 2, 3],
        vec![1u16, 2, 3],
        vec![String::from("x"), String::new()],
        vec![vec![1u8], vec![]],
        vec![vec![[1u8; 32]], vec![]],
    );
}

#[test]
fn malformed_envelope_inputs_are_rejected() {
    let good = encode(&fixed_envelope());
    assert_eq!(
        decode::<Envelope>(&[]),
        Err(CodecError::UnexpectedEof {
            needed: 1,
            remaining: 0
        })
    );
    let mut bad = good.clone();
    bad[1] = 5;
    assert_eq!(
        decode::<Envelope>(&bad),
        Err(CodecError::UnknownVariant {
            what: "task op",
            value: 5
        })
    );
    let mut bad = good.clone();
    bad[0] = 0;
    assert_eq!(
        decode::<Envelope>(&bad),
        Err(unknown_variant("envelope ver", 0u8))
    );
    // corrupt the title's UTF-8 (title starts after 1+1+16+4+1+32 bytes, 2-byte len)
    let title_off = 1 + 1 + 16 + 4 + 1 + 32 + 2;
    let mut bad = good.clone();
    bad[title_off] = 0xff;
    assert_eq!(decode::<Envelope>(&bad), Err(CodecError::InvalidUtf8));
    // inflate the title length so it runs past the end
    let mut bad = good.clone();
    bad[title_off - 2] = 0xff;
    bad[title_off - 1] = 0xff;
    assert!(matches!(
        decode::<Envelope>(&bad),
        Err(CodecError::UnexpectedEof { needed: 65535, .. })
    ));
    // trailing byte
    let mut bad = good.clone();
    bad.push(0);
    assert_eq!(decode::<Envelope>(&bad), Err(CodecError::TrailingBytes(1)));
    // non-canonical bool at the end
    let mut bad = good;
    *bad.last_mut().unwrap() = 7;
    assert_eq!(decode::<Envelope>(&bad), Err(CodecError::Invalid("bool")));
}

#[test]
fn encoder_limits_are_enforced() {
    let mut e = fixed_envelope();
    e.title = "x".repeat(MAX_LEN16 + 1);
    assert_eq!(
        try_encode(&e),
        Err(CodecError::LengthOverflow {
            what: "str16",
            max: MAX_LEN16,
            got: MAX_LEN16 + 1
        })
    );
    let mut e = fixed_envelope();
    e.ct = vec![0; MAX_LEN16 + 1];
    assert_eq!(
        try_encode(&e),
        Err(CodecError::LengthOverflow {
            what: "bytes16",
            max: MAX_LEN16,
            got: MAX_LEN16 + 1
        })
    );
    let mut e = fixed_envelope();
    e.heads = vec![[0; 32]; MAX_LIST_U8 + 1];
    assert_eq!(
        try_encode(&e),
        Err(CodecError::LengthOverflow {
            what: "list_u8",
            max: MAX_LIST_U8,
            got: MAX_LIST_U8 + 1
        })
    );
    // at the limits everything round-trips
    let mut e = fixed_envelope();
    e.title = "x".repeat(MAX_LEN16);
    e.ct = vec![0; MAX_LEN16];
    e.heads = vec![[0; 32]; MAX_LIST_U8];
    let b = encode(&e);
    assert_eq!(decode::<Envelope>(&b), Ok(e));
}

#[test]
fn list_counts_larger_than_input_fail_fast() {
    // 255 cosigs claimed, 0 bytes of cosig present
    assert_eq!(
        decode::<Vec<CoSig>>(&[255]),
        Err(CodecError::UnexpectedEof {
            needed: 32,
            remaining: 0
        })
    );
    // deeply nested "255 lists" bomb must fail with EOF, not exhaust memory
    let bomb = vec![0xffu8; 64 * 1024];
    assert!(matches!(
        decode::<Vec<Vec<Vec<Vec<Vec<CoSig>>>>>>(&bomb),
        Err(CodecError::UnexpectedEof { .. })
    ));
}

// The pre-allocation cap is a hint below the hard list limit, not a limit.
const _: () = assert!(LIST_PREALLOC_CAP <= MAX_LIST_U8);

#[test]
fn list_prealloc_never_exceeds_input() {
    // A 32 KiB item type makes any up-front reservation visible: reserving
    // even 32 items for a count byte that no bytes follow is a 1 MiB
    // allocation driven by a 1-byte input (the original `min(count, 64)`
    // rule reserved 2 MiB). The number of items that can still decode is
    // bounded by the bytes remaining, since an item occupies at least one
    // byte on the wire, so no large allocation may happen here.
    type Big = [u8; 32 << 10];
    let big = big_allocs_during(|| {
        assert!(matches!(
            decode::<Vec<Big>>(&[255]),
            Err(CodecError::UnexpectedEof { needed, remaining: 0 }) if needed == 32 << 10
        ));
        // two levels: outer count 255 with one byte left, inner count 255 with none
        assert!(matches!(
            decode::<Vec<Vec<Big>>>(&[255, 255]),
            Err(CodecError::UnexpectedEof { remaining: 0, .. })
        ));
        // through the raw reader as well, with a closure item
        let mut r = Reader::new(&[200]);
        assert!(r.list_u8(|r| r.bytes::<{ 32 << 10 }>()).is_err());
    });
    assert_eq!(
        big, 0,
        "decoding a bare count byte must not reserve memory the input cannot fill"
    );
    // the hint still applies when the input could hold the items
    let mut good = vec![3u8];
    good.extend_from_slice(&[7u8; 3 * (32 << 10)]);
    let v: Vec<Big> = decode(&good).unwrap();
    assert_eq!(v.len(), 3);
    assert!(v.iter().all(|b| b.iter().all(|&x| x == 7)));
}

// ---- property tests --------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn envelope_round_trip(e in envelope_strategy()) {
        let bytes = encode(&e);
        let back: Envelope = decode(&bytes).unwrap();
        prop_assert_eq!(&back, &e);
        prop_assert_eq!(encode(&back), bytes);
    }

    #[test]
    fn equal_envelopes_encode_identically(e in envelope_strategy()) {
        let a = encode(&e);
        let b = encode(&e.clone());
        prop_assert_eq!(a, b);
    }

    #[test]
    fn every_strict_prefix_is_eof(e in envelope_strategy()) {
        let bytes = encode(&e);
        // check a spread of cut points, including every one near the end
        let n = bytes.len();
        let cuts = (0..n).filter(|c| c % 7 == 0 || n - c <= 12);
        for cut in cuts {
            prop_assert!(
                matches!(decode::<Envelope>(&bytes[..cut]), Err(CodecError::UnexpectedEof { .. })),
                "prefix of {} bytes", cut
            );
        }
    }

    #[test]
    fn extra_bytes_are_trailing(e in envelope_strategy(), extra in prop::collection::vec(any::<u8>(), 1..=16)) {
        let mut bytes = encode(&e);
        bytes.extend_from_slice(&extra);
        prop_assert_eq!(decode::<Envelope>(&bytes), Err(CodecError::TrailingBytes(extra.len())));
    }

    #[test]
    fn arbitrary_bytes_never_panic(bytes in prop::collection::vec(any::<u8>(), 0..=2048)) {
        // whatever comes back, the decoder must return rather than panic
        let _ = decode::<Envelope>(&bytes);
        let _ = decode::<Vec<Vec<String>>>(&bytes);
        let _ = decode::<Vec<CoSig>>(&bytes);
        let _ = decode::<String>(&bytes);
    }

    #[test]
    fn primitive_strings_and_blobs_round_trip(
        s in string_strategy(512),
        b in prop::collection::vec(any::<u8>(), 0..=4096),
        l in prop::collection::vec(string_strategy(16), 0..=32),
    ) {
        prop_assert_eq!(decode::<String>(&encode(&s)), Ok(s.clone()));
        prop_assert_eq!(decode::<Vec<u8>>(&encode(&b)), Ok(b.clone()));
        prop_assert_eq!(decode::<Vec<String>>(&encode(&l)), Ok(l.clone()));
        // str16 length prefix is the UTF-8 byte length
        let sb = encode(&s);
        prop_assert_eq!(u16::from_le_bytes([sb[0], sb[1]]) as usize, s.len());
    }
}
