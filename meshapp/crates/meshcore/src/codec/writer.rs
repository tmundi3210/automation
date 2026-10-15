//! Append-only encoder for the DESIGN.md §2.1 wire format.

use super::error::CodecError;

/// Largest payload a `str16`/`bytes16` field can carry (u16 length prefix).
pub const MAX_LEN16: usize = u16::MAX as usize;
/// Largest item count a `list_u8` field can carry (u8 count prefix).
pub const MAX_LIST_U8: usize = u8::MAX as usize;

/// Appends fixed-field little-endian values to an owned buffer.
///
/// The writer has no choices to make: every value has exactly one encoding,
/// so equal values produce byte-identical output and hashes/signatures need
/// no canonicalisation pass (DESIGN.md §2.1 reason 1). The fixed-width
/// methods cannot fail and return `&mut Self` so headers can be written in
/// one chain; the length-prefixed ones return `Err(LengthOverflow)` when the
/// value does not fit its prefix.
#[derive(Debug, Default, Clone)]
pub struct Writer {
    buf: Vec<u8>,
}

impl Writer {
    /// An empty writer.
    pub fn new() -> Self {
        Self::default()
    }

    /// An empty writer with room for `capacity` bytes.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
        }
    }

    /// Bytes written so far.
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// `true` when nothing has been written.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// The bytes written so far.
    pub fn as_bytes(&self) -> &[u8] {
        &self.buf
    }

    /// Takes the encoded bytes.
    pub fn into_inner(self) -> Vec<u8> {
        self.buf
    }

    /// One byte.
    pub fn u8(&mut self, v: u8) -> &mut Self {
        self.buf.push(v);
        self
    }

    /// Two bytes, little-endian.
    pub fn u16(&mut self, v: u16) -> &mut Self {
        self.raw(&v.to_le_bytes())
    }

    /// Four bytes, little-endian.
    pub fn u32(&mut self, v: u32) -> &mut Self {
        self.raw(&v.to_le_bytes())
    }

    /// Eight bytes, little-endian.
    pub fn u64(&mut self, v: u64) -> &mut Self {
        self.raw(&v.to_le_bytes())
    }

    /// Eight bytes, two's complement little-endian.
    pub fn i64(&mut self, v: i64) -> &mut Self {
        self.raw(&v.to_le_bytes())
    }

    /// A `bool` as one byte, `0` or `1`. The reader accepts no other value,
    /// so the field stays canonical.
    pub fn bool(&mut self, v: bool) -> &mut Self {
        self.u8(u8::from(v))
    }

    /// A fixed-size field with no length prefix: hashes (32 B), keys (32 B),
    /// signatures (64 B), ids (16 B), tags (16 B) in DESIGN.md §2.
    pub fn bytes<const N: usize>(&mut self, v: &[u8; N]) -> &mut Self {
        self.raw(v)
    }

    /// Raw bytes with no length prefix. The caller is responsible for the
    /// reader knowing how many to take back (a fixed layout, or the rest of
    /// the buffer, as in an AEAD body).
    pub fn raw(&mut self, v: &[u8]) -> &mut Self {
        self.buf.extend_from_slice(v);
        self
    }

    /// `str16`: u16 LE byte length followed by the UTF-8 bytes (DESIGN.md
    /// §2.1). No normalisation happens here; the §3 name rules of the brief
    /// are applied by the objects before encoding.
    pub fn str16(&mut self, s: &str) -> Result<(), CodecError> {
        self.len16("str16", s.len())?;
        self.raw(s.as_bytes());
        Ok(())
    }

    /// `bytes16`: u16 LE length followed by the raw bytes (`ct_len u16 ‖ ct`
    /// in the SKDM bundle, DESIGN.md §2.4).
    pub fn bytes16(&mut self, b: &[u8]) -> Result<(), CodecError> {
        self.len16("bytes16", b.len())?;
        self.raw(b);
        Ok(())
    }

    /// `list_u8`: u8 item count followed by each item written by `item`
    /// (DESIGN.md §2.1 "lists = u8 count + items"). More than 255 items is
    /// detected before anything is written and fails with
    /// [`CodecError::LengthOverflow`]; an error from `item` leaves the count
    /// and the items written so far in the buffer, so callers treat any
    /// `Err` as "discard this writer".
    pub fn list_u8<T, F>(&mut self, items: &[T], mut item: F) -> Result<(), CodecError>
    where
        F: FnMut(&mut Writer, &T) -> Result<(), CodecError>,
    {
        let count = u8::try_from(items.len()).map_err(|_| CodecError::LengthOverflow {
            what: "list_u8",
            max: MAX_LIST_U8,
            got: items.len(),
        })?;
        self.u8(count);
        for it in items {
            item(self, it)?;
        }
        Ok(())
    }

    fn len16(&mut self, what: &'static str, len: usize) -> Result<(), CodecError> {
        let prefix = u16::try_from(len).map_err(|_| CodecError::LengthOverflow {
            what,
            max: MAX_LEN16,
            got: len,
        })?;
        self.u16(prefix);
        Ok(())
    }
}
