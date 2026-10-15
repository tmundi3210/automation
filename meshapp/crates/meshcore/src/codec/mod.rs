//! Canonical fixed-field little-endian codec (docs/DESIGN.md §2.1).
//!
//! Every protocol object is a fixed sequence of fields, each with exactly one
//! encoding: integers are fixed-width little-endian, hashes/keys/signatures
//! are fixed arrays, strings are `str16` (u16 LE byte length + UTF-8), raw
//! blobs are `bytes16` (u16 LE length + bytes) and lists are `list_u8` (u8
//! count + items). There are no floats, no varints, no optional fields and no
//! maps, so equal values give byte-identical output and hashes and signatures
//! need no canonicalisation pass (DESIGN.md §2.1 reason 1). Decoders reject
//! trailing bytes, over-long lengths and unknown enum values (§2.1), and
//! never panic or over-allocate on malformed input (the relay's "checks
//! format, size" step, BRIEF_v2 §5.5; parser in the §5.8 audit scope).
//!
//! Objects implement [`Encode`] and [`Decode`] by writing their fields in
//! layout order through [`Writer`] and [`Reader`]; [`encode`] and [`decode`]
//! are the whole-object entry points. Enum decoders report unknown
//! discriminants with [`unknown_variant`].

mod error;
mod reader;
mod writer;

pub use error::{CodecError, unknown_variant};
pub use reader::{LIST_PREALLOC_CAP, Reader};
pub use writer::{MAX_LEN16, MAX_LIST_U8, Writer};

/// A value with exactly one wire encoding.
pub trait Encode {
    /// Appends this value's fields in layout order. The only failure is a
    /// [`CodecError::LengthOverflow`] from a `str16`/`bytes16`/`list_u8`
    /// field whose value exceeds its prefix.
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError>;

    /// This value's encoding, or the limit it violates.
    fn try_encode(&self) -> Result<Vec<u8>, CodecError> {
        let mut w = Writer::new();
        self.encode_to(&mut w)?;
        Ok(w.into_inner())
    }

    /// This value's encoding (`fn encode<T:Encode>(&T)->Vec<u8>` in the
    /// DESIGN.md §1 module table).
    ///
    /// # Panics
    ///
    /// If a field exceeds its length prefix. Object constructors enforce the
    /// §2.4 limits (text ≤ 2000 B, ≤ 50 members, 2..=8 options, ...), which
    /// are all far below the codec's 65535-byte / 255-item ceilings, so a
    /// panic here is a broken invariant, not bad input. Code that encodes
    /// values it did not validate uses [`Encode::try_encode`].
    fn encode(&self) -> Vec<u8> {
        match self.try_encode() {
            Ok(bytes) => bytes,
            Err(e) => panic!("Encode::encode on a value that violates a codec limit: {e}"),
        }
    }
}

/// A value that can be read back from its wire encoding.
pub trait Decode: Sized {
    /// Reads this value's fields in layout order, leaving the reader just
    /// after them. Whole-object checks (trailing bytes) belong to [`decode`].
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError>;
}

/// Marker for types that may be the item type of a `list_u8` (`Vec<T>`,
/// `[T]`).
///
/// It exists so that `Vec<u8>` and `[u8]` can mean `bytes16` (u16 length +
/// raw bytes) while `Vec<T>` for every other `T` means `list_u8` (u8 count +
/// items); without it a ciphertext field typed `Vec<u8>` would silently
/// encode as a list capped at 255 entries. `u8` is therefore the one codec
/// primitive that deliberately does not implement it; a list of bytes-as-
/// items is written with [`Writer::list_u8`] explicitly. Protocol object
/// types opt in with `impl ListItem for Thing {}`.
pub trait ListItem {}

/// Encodes `value`. See [`Encode::encode`] for the panic condition.
pub fn encode<T: Encode + ?Sized>(value: &T) -> Vec<u8> {
    value.encode()
}

/// Encodes `value`, or reports the length limit it violates.
pub fn try_encode<T: Encode + ?Sized>(value: &T) -> Result<Vec<u8>, CodecError> {
    value.try_encode()
}

/// Decodes one whole `T` from `bytes`, rejecting trailing bytes
/// ([`CodecError::TrailingBytes`]) so that every value has exactly one
/// accepted encoding (DESIGN.md §2.1).
pub fn decode<T: Decode>(bytes: &[u8]) -> Result<T, CodecError> {
    let mut r = Reader::new(bytes);
    let value = T::decode_from(&mut r)?;
    r.finish()?;
    Ok(value)
}

// ---- primitives -----------------------------------------------------------

macro_rules! fixed_width {
    ($($ty:ident),* $(,)?) => {$(
        impl Encode for $ty {
            fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
                w.$ty(*self);
                Ok(())
            }
        }
        impl Decode for $ty {
            fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
                r.$ty()
            }
        }
    )*};
}
fixed_width!(u8, u16, u32, u64, i64, bool);

impl ListItem for u16 {}
impl ListItem for u32 {}
impl ListItem for u64 {}
impl ListItem for i64 {}
impl ListItem for bool {}

impl<const N: usize> Encode for [u8; N] {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.bytes(self);
        Ok(())
    }
}
impl<const N: usize> Decode for [u8; N] {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        r.bytes()
    }
}
impl<const N: usize> ListItem for [u8; N] {}

// ---- str16 ----------------------------------------------------------------

impl Encode for str {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.str16(self)
    }
}
impl Encode for String {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.str16(self)
    }
}
impl Decode for String {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(r.str16()?.to_owned())
    }
}
impl ListItem for String {}

// ---- bytes16 --------------------------------------------------------------

impl Encode for [u8] {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.bytes16(self)
    }
}
impl Encode for Vec<u8> {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.bytes16(self)
    }
}
impl Decode for Vec<u8> {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(r.bytes16()?.to_vec())
    }
}
impl ListItem for Vec<u8> {}

// ---- list_u8 --------------------------------------------------------------

impl<T: ListItem + Encode> Encode for [T] {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.list_u8(self, |w, item| item.encode_to(w))
    }
}
impl<T: ListItem + Encode> Encode for Vec<T> {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        self.as_slice().encode_to(w)
    }
}
impl<T: ListItem + Decode> Decode for Vec<T> {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        r.list_u8(T::decode_from)
    }
}
impl<T: ListItem> ListItem for Vec<T> {}

// ---- references -----------------------------------------------------------

impl<T: Encode + ?Sized> Encode for &T {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        (**self).encode_to(w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip<T: Encode + Decode + PartialEq + core::fmt::Debug>(v: T) -> Vec<u8> {
        let bytes = encode(&v);
        let back: T = decode(&bytes).expect("decode");
        assert_eq!(back, v);
        assert_eq!(encode(&back), bytes, "re-encoding is byte-identical");
        bytes
    }

    #[test]
    fn integers_are_little_endian_fixed_width() {
        assert_eq!(roundtrip(0x01u8), [0x01]);
        assert_eq!(roundtrip(0x0102u16), [0x02, 0x01]);
        assert_eq!(roundtrip(0x0102_0304u32), [0x04, 0x03, 0x02, 0x01]);
        assert_eq!(
            roundtrip(0x0102_0304_0506_0708u64),
            [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]
        );
        assert_eq!(roundtrip(-1i64), [0xff; 8]);
        assert_eq!(roundtrip(i64::MIN), [0, 0, 0, 0, 0, 0, 0, 0x80]);
        assert_eq!(
            roundtrip(i64::MAX),
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]
        );
        for v in [0u64, 1, u64::MAX] {
            roundtrip(v);
        }
        for v in [0u32, u32::MAX] {
            roundtrip(v);
        }
        for v in [0u16, u16::MAX] {
            roundtrip(v);
        }
        for v in [0u8, u8::MAX] {
            roundtrip(v);
        }
    }

    #[test]
    fn bool_has_exactly_one_encoding_per_value() {
        assert_eq!(roundtrip(false), [0]);
        assert_eq!(roundtrip(true), [1]);
        assert_eq!(decode::<bool>(&[2]), Err(CodecError::Invalid("bool")));
        assert_eq!(decode::<bool>(&[0xff]), Err(CodecError::Invalid("bool")));
    }

    #[test]
    fn fixed_arrays_have_no_prefix() {
        assert_eq!(roundtrip([0u8; 0]), Vec::<u8>::new());
        assert_eq!(roundtrip([7u8; 16]).len(), 16);
        assert_eq!(roundtrip([9u8; 32]).len(), 32);
        assert_eq!(roundtrip([3u8; 64]).len(), 64);
        let mut arr = [0u8; 32];
        for (i, b) in arr.iter_mut().enumerate() {
            *b = i as u8;
        }
        assert_eq!(roundtrip(arr), arr.to_vec());
    }

    #[test]
    fn str16_is_u16_len_plus_utf8() {
        assert_eq!(roundtrip(String::new()), [0, 0]);
        assert_eq!(roundtrip("hi".to_string()), [2, 0, b'h', b'i']);
        // multi-byte UTF-8: length prefix counts bytes, not chars
        let s = "héllo 🌍".to_string();
        let bytes = roundtrip(s.clone());
        assert_eq!(bytes.len(), 2 + s.len());
        assert_eq!(&bytes[..2], &(s.len() as u16).to_le_bytes());
        // the longest allowed string
        let long = "a".repeat(MAX_LEN16);
        assert_eq!(roundtrip(long).len(), 2 + MAX_LEN16);
        // borrowed str encodes identically to String
        assert_eq!(encode("hi"), encode(&"hi".to_string()));
    }

    #[test]
    fn bytes16_is_u16_len_plus_raw() {
        assert_eq!(roundtrip(Vec::<u8>::new()), [0, 0]);
        assert_eq!(roundtrip(vec![0xaau8, 0xbb]), [2, 0, 0xaa, 0xbb]);
        let long = vec![0x5au8; MAX_LEN16];
        assert_eq!(roundtrip(long).len(), 2 + MAX_LEN16);
        assert_eq!(encode(&[1u8, 2][..]), encode(&vec![1u8, 2]));
    }

    #[test]
    fn list_u8_is_u8_count_plus_items() {
        assert_eq!(roundtrip(Vec::<u16>::new()), [0]);
        assert_eq!(
            roundtrip(vec![0x0102u16, 0x0304]),
            [2, 0x02, 0x01, 0x04, 0x03]
        );
        assert_eq!(
            roundtrip(vec!["a".to_string(), "bc".to_string()]),
            [2, 1, 0, b'a', 2, 0, b'b', b'c']
        );
        assert_eq!(roundtrip(vec![[1u8; 32], [2u8; 32]]).len(), 1 + 64);
        // list of bytes16 blobs
        assert_eq!(roundtrip(vec![vec![1u8], vec![]]), [2, 1, 0, 1, 0, 0]);
        // nested lists
        assert_eq!(
            roundtrip(vec![vec![1u16], vec![2, 3]]),
            [2, 1, 1, 0, 2, 2, 0, 3, 0]
        );
        // the longest allowed list
        assert_eq!(roundtrip(vec![true; MAX_LIST_U8]).len(), 1 + MAX_LIST_U8);
        // slices encode like vectors
        assert_eq!(encode(&[1u16, 2][..]), encode(&vec![1u16, 2]));
    }

    #[test]
    fn encoder_rejects_values_that_exceed_their_prefix() {
        let s = "a".repeat(MAX_LEN16 + 1);
        assert_eq!(
            try_encode(&s),
            Err(CodecError::LengthOverflow {
                what: "str16",
                max: MAX_LEN16,
                got: MAX_LEN16 + 1
            })
        );
        let b = vec![0u8; MAX_LEN16 + 1];
        assert_eq!(
            try_encode(&b),
            Err(CodecError::LengthOverflow {
                what: "bytes16",
                max: MAX_LEN16,
                got: MAX_LEN16 + 1
            })
        );
        let l = vec![0u16; MAX_LIST_U8 + 1];
        assert_eq!(
            try_encode(&l),
            Err(CodecError::LengthOverflow {
                what: "list_u8",
                max: MAX_LIST_U8,
                got: MAX_LIST_U8 + 1
            })
        );
        // a failed list write leaves the buffer untouched
        let mut w = Writer::new();
        w.u8(0xee);
        assert!(w.list_u8(&l, |w, v| v.encode_to(w)).is_err());
        assert_eq!(w.as_bytes(), [0xee]);
    }

    #[test]
    #[should_panic(expected = "violates a codec limit")]
    fn encode_panics_on_limit_violation() {
        let _ = encode(&"a".repeat(MAX_LEN16 + 1));
    }

    #[test]
    fn reader_reports_eof_with_sizes() {
        assert_eq!(
            decode::<u8>(&[]),
            Err(CodecError::UnexpectedEof {
                needed: 1,
                remaining: 0
            })
        );
        assert_eq!(
            decode::<u16>(&[1]),
            Err(CodecError::UnexpectedEof {
                needed: 2,
                remaining: 1
            })
        );
        assert_eq!(
            decode::<u32>(&[1, 2, 3]),
            Err(CodecError::UnexpectedEof {
                needed: 4,
                remaining: 3
            })
        );
        assert_eq!(
            decode::<u64>(&[0; 7]),
            Err(CodecError::UnexpectedEof {
                needed: 8,
                remaining: 7
            })
        );
        assert_eq!(
            decode::<i64>(&[0; 7]),
            Err(CodecError::UnexpectedEof {
                needed: 8,
                remaining: 7
            })
        );
        assert_eq!(
            decode::<[u8; 32]>(&[0; 31]),
            Err(CodecError::UnexpectedEof {
                needed: 32,
                remaining: 31
            })
        );
        // str16/bytes16: length prefix claims more than is there
        assert_eq!(
            decode::<String>(&[5, 0, b'a', b'b', b'c']),
            Err(CodecError::UnexpectedEof {
                needed: 5,
                remaining: 3
            })
        );
        assert_eq!(
            decode::<Vec<u8>>(&[0xff, 0xff]),
            Err(CodecError::UnexpectedEof {
                needed: 65535,
                remaining: 0
            })
        );
        // truncated prefix itself
        assert_eq!(
            decode::<String>(&[5]),
            Err(CodecError::UnexpectedEof {
                needed: 2,
                remaining: 1
            })
        );
    }

    #[test]
    fn reader_rejects_trailing_bytes() {
        assert_eq!(decode::<u8>(&[1, 2]), Err(CodecError::TrailingBytes(1)));
        assert_eq!(
            decode::<String>(&[1, 0, b'a', 0, 0, 0]),
            Err(CodecError::TrailingBytes(3))
        );
        let mut r = Reader::new(&[1, 2, 3]);
        assert_eq!(r.u8(), Ok(1));
        assert_eq!(r.remaining(), 2);
        assert_eq!(r.position(), 1);
        assert_eq!(r.clone().finish(), Err(CodecError::TrailingBytes(2)));
        assert_eq!(r.take(2), Ok(&[2u8, 3][..]));
        assert_eq!(r.finish(), Ok(()));
    }

    #[test]
    fn reader_rejects_invalid_utf8() {
        assert_eq!(
            decode::<String>(&[2, 0, 0xff, 0xfe]),
            Err(CodecError::InvalidUtf8)
        );
        // overlong encoding of '/' (C0 AF) is invalid UTF-8
        assert_eq!(
            decode::<String>(&[2, 0, 0xc0, 0xaf]),
            Err(CodecError::InvalidUtf8)
        );
        // a lone surrogate (ED A0 80) is invalid UTF-8
        assert_eq!(
            decode::<String>(&[3, 0, 0xed, 0xa0, 0x80]),
            Err(CodecError::InvalidUtf8)
        );
        // the same bytes are fine as bytes16
        assert_eq!(decode::<Vec<u8>>(&[2, 0, 0xff, 0xfe]), Ok(vec![0xff, 0xfe]));
    }

    #[test]
    fn list_count_beyond_input_fails_at_first_missing_item() {
        assert_eq!(
            decode::<Vec<[u8; 32]>>(&[255, 0, 0]),
            Err(CodecError::UnexpectedEof {
                needed: 32,
                remaining: 2
            })
        );
        assert_eq!(
            decode::<Vec<u64>>(&[200]),
            Err(CodecError::UnexpectedEof {
                needed: 8,
                remaining: 0
            })
        );
        // a nested-list bomb: every byte says "255 more lists"
        let bomb = [0xffu8; 4096];
        assert!(matches!(
            decode::<Vec<Vec<Vec<Vec<u16>>>>>(&bomb),
            Err(CodecError::UnexpectedEof { .. })
        ));
        // the count is only a hint: 255 zero-byte items decode to 255 items
        let mut r = Reader::new(&[255]);
        let items = r.list_u8(|_| Ok(())).expect("zero-size items");
        assert_eq!(items.len(), 255);
        assert_eq!(r.finish(), Ok(()));
    }

    #[test]
    fn reader_raw_helpers() {
        let mut r = Reader::new(&[1, 2, 3, 4, 5]);
        assert_eq!(r.bytes::<2>(), Ok([1, 2]));
        assert_eq!(r.rest(), &[3, 4, 5]);
        assert_eq!(r.remaining(), 0);
        assert!(r.rest().is_empty());
        assert_eq!(r.take(0), Ok(&[][..]));
        assert_eq!(
            r.take(1),
            Err(CodecError::UnexpectedEof {
                needed: 1,
                remaining: 0
            })
        );
        assert_eq!(r.finish(), Ok(()));
    }

    #[test]
    fn writer_chaining_and_accessors() {
        let mut w = Writer::with_capacity(16);
        assert!(w.is_empty());
        w.u8(1)
            .u16(2)
            .u32(3)
            .u64(4)
            .i64(-1)
            .bool(true)
            .bytes(&[9u8; 2])
            .raw(&[8, 8]);
        assert_eq!(w.len(), 1 + 2 + 4 + 8 + 8 + 1 + 2 + 2);
        assert_eq!(
            w.as_bytes(),
            [
                1, 2, 0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255,
                255, 1, 9, 9, 8, 8
            ]
        );
        let bytes = w.clone().into_inner();
        assert_eq!(bytes, w.as_bytes());
        assert!(Writer::new().is_empty());
    }

    #[test]
    fn unknown_variant_helper_and_display() {
        let e = unknown_variant("post kind", 0x99u8);
        assert_eq!(
            e,
            CodecError::UnknownVariant {
                what: "post kind",
                value: 0x99
            }
        );
        assert_eq!(e.to_string(), "unknown post kind value 153");
        assert_eq!(
            CodecError::UnexpectedEof {
                needed: 4,
                remaining: 1
            }
            .to_string(),
            "unexpected end of input: needed 4 byte(s), 1 remaining"
        );
        assert_eq!(
            CodecError::TrailingBytes(3).to_string(),
            "3 trailing byte(s) after the object"
        );
        assert_eq!(
            CodecError::InvalidUtf8.to_string(),
            "string is not valid UTF-8"
        );
        assert_eq!(
            CodecError::LengthOverflow {
                what: "str16",
                max: 65535,
                got: 70000
            }
            .to_string(),
            "str16 too long: max 65535, got 70000"
        );
        assert_eq!(CodecError::Invalid("bool").to_string(), "invalid bool");
        // thiserror gives us std::error::Error
        let _: &dyn std::error::Error = &e;
    }

    // A sample object in the DESIGN.md §2.4 style: ver, kind enum, fixed
    // ids, a string, a blob, a list of fixed arrays and a nested list.
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Kind {
        Text,
        TaskOp,
    }

    impl Kind {
        fn to_u8(&self) -> u8 {
            match self {
                Kind::Text => 0x30,
                Kind::TaskOp => 0x31,
            }
        }
        fn from_u8(v: u8) -> Result<Self, CodecError> {
            match v {
                0x30 => Ok(Kind::Text),
                0x31 => Ok(Kind::TaskOp),
                other => Err(unknown_variant("post kind", other)),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Sample {
        ver: u8,
        kind: Kind,
        group_id: [u8; 16],
        author_seq: u32,
        heads: Vec<[u8; 32]>,
        text: String,
        blob: Vec<u8>,
        options: Vec<String>,
        delta: i64,
        in_person: bool,
    }

    impl Encode for Sample {
        fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
            w.u8(self.ver)
                .u8(self.kind.to_u8())
                .bytes(&self.group_id)
                .u32(self.author_seq);
            self.heads.encode_to(w)?;
            w.str16(&self.text)?;
            w.bytes16(&self.blob)?;
            w.list_u8(&self.options, |w, o| w.str16(o))?;
            w.i64(self.delta).bool(self.in_person);
            Ok(())
        }
    }

    impl Decode for Sample {
        fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
            let ver = r.u8()?;
            if ver != 1 {
                return Err(unknown_variant("sample ver", ver));
            }
            Ok(Sample {
                ver,
                kind: Kind::from_u8(r.u8()?)?,
                group_id: r.bytes()?,
                author_seq: r.u32()?,
                heads: Vec::decode_from(r)?,
                text: r.str16()?.to_owned(),
                blob: r.bytes16()?.to_vec(),
                options: r.list_u8(|r| Ok(r.str16()?.to_owned()))?,
                delta: r.i64()?,
                in_person: r.bool()?,
            })
        }
    }

    fn sample() -> Sample {
        Sample {
            ver: 1,
            kind: Kind::TaskOp,
            group_id: [0x11; 16],
            author_seq: 0x0102_0304,
            heads: vec![[0xaa; 32], [0xbb; 32]],
            text: "héllo".to_string(),
            blob: vec![1, 2, 3],
            options: vec!["x".to_string(), String::new()],
            delta: -2,
            in_person: true,
        }
    }

    #[test]
    fn sample_struct_known_answer_and_roundtrip() {
        let bytes = roundtrip(sample());
        let mut expect = vec![1, 0x31];
        expect.extend_from_slice(&[0x11; 16]);
        expect.extend_from_slice(&[0x04, 0x03, 0x02, 0x01]);
        expect.push(2);
        expect.extend_from_slice(&[0xaa; 32]);
        expect.extend_from_slice(&[0xbb; 32]);
        expect.extend_from_slice(&[6, 0]);
        expect.extend_from_slice("héllo".as_bytes());
        expect.extend_from_slice(&[3, 0, 1, 2, 3]);
        expect.extend_from_slice(&[2, 1, 0, b'x', 0, 0]);
        expect.extend_from_slice(&(-2i64).to_le_bytes());
        expect.push(1);
        assert_eq!(bytes, expect);
    }

    #[test]
    fn sample_struct_rejects_malformed_input() {
        let good = encode(&sample());
        // unknown version / kind
        let mut bad = good.clone();
        bad[0] = 2;
        assert_eq!(
            decode::<Sample>(&bad),
            Err(unknown_variant("sample ver", 2u8))
        );
        let mut bad = good.clone();
        bad[1] = 0x7f;
        assert_eq!(
            decode::<Sample>(&bad),
            Err(unknown_variant("post kind", 0x7fu8))
        );
        // every strict prefix is EOF
        for cut in 0..good.len() {
            assert!(
                matches!(
                    decode::<Sample>(&good[..cut]),
                    Err(CodecError::UnexpectedEof { .. })
                ),
                "prefix of {cut} bytes"
            );
        }
        // any extra byte is trailing
        let mut longer = good.clone();
        longer.push(0);
        assert_eq!(decode::<Sample>(&longer), Err(CodecError::TrailingBytes(1)));
        // non-canonical bool
        let mut bad = good.clone();
        *bad.last_mut().unwrap() = 2;
        assert_eq!(decode::<Sample>(&bad), Err(CodecError::Invalid("bool")));
    }

    #[test]
    fn equal_values_encode_identically() {
        let a = sample();
        let b = a.clone();
        assert_eq!(encode(&a), encode(&b));
        assert_eq!(a.try_encode().unwrap(), encode(&&a));
        let mut w1 = Writer::new();
        let mut w2 = Writer::with_capacity(1024);
        a.encode_to(&mut w1).unwrap();
        b.encode_to(&mut w2).unwrap();
        assert_eq!(w1.into_inner(), w2.into_inner());
    }
}
