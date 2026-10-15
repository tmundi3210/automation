//! Bounds-checked decoder for the DESIGN.md §2.1 wire format.

use super::error::CodecError;

/// Upper bound on the capacity a list decoder reserves before it has read
/// any item. The count byte is untrusted, so pre-allocating from it would
/// let a 2-byte input reserve 255 items of whatever size `T` is, at every
/// nesting level; past this bound the `Vec` grows as items actually decode.
/// The reservation is further bounded by the bytes still unread, so a bare
/// count byte reserves nothing (BRIEF_v2 §5.5 "checks format, size").
pub const LIST_PREALLOC_CAP: usize = 64;

/// Reads fixed-field little-endian values from a borrowed byte slice.
///
/// Every read checks that enough bytes remain and fails with
/// [`CodecError::UnexpectedEof`] otherwise; nothing is read past the end,
/// nothing panics on malformed input, and variable-length fields are
/// returned as sub-slices of the input, so no allocation happens before the
/// bytes a length prefix describes are known to exist (BRIEF_v2 §5.5 "every
/// relay checks format, size"; DESIGN.md §2.1).
#[derive(Debug, Clone)]
pub struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    /// A reader positioned at the start of `buf`.
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// Bytes not yet read.
    pub fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    /// Bytes read so far (the offset of the next read).
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Succeeds only if every byte was consumed; otherwise
    /// [`CodecError::TrailingBytes`] with the number left (DESIGN.md §2.1
    /// "decoders reject trailing bytes").
    pub fn finish(self) -> Result<(), CodecError> {
        match self.remaining() {
            0 => Ok(()),
            n => Err(CodecError::TrailingBytes(n)),
        }
    }

    /// Takes the next `n` bytes as a slice of the input.
    pub fn take(&mut self, n: usize) -> Result<&'a [u8], CodecError> {
        let remaining = self.remaining();
        if n > remaining {
            return Err(CodecError::UnexpectedEof {
                needed: n,
                remaining,
            });
        }
        let start = self.pos;
        self.pos += n;
        Ok(&self.buf[start..self.pos])
    }

    /// Takes everything that is left (for "the rest of the buffer is the
    /// AEAD body" layouts). Afterwards [`remaining`](Self::remaining) is 0.
    pub fn rest(&mut self) -> &'a [u8] {
        let start = self.pos;
        self.pos = self.buf.len();
        &self.buf[start..]
    }

    /// One byte.
    pub fn u8(&mut self) -> Result<u8, CodecError> {
        Ok(self.bytes::<1>()?[0])
    }

    /// Two bytes, little-endian.
    pub fn u16(&mut self) -> Result<u16, CodecError> {
        Ok(u16::from_le_bytes(self.bytes()?))
    }

    /// Four bytes, little-endian.
    pub fn u32(&mut self) -> Result<u32, CodecError> {
        Ok(u32::from_le_bytes(self.bytes()?))
    }

    /// Eight bytes, little-endian.
    pub fn u64(&mut self) -> Result<u64, CodecError> {
        Ok(u64::from_le_bytes(self.bytes()?))
    }

    /// Eight bytes, two's complement little-endian.
    pub fn i64(&mut self) -> Result<i64, CodecError> {
        Ok(i64::from_le_bytes(self.bytes()?))
    }

    /// One byte that must be `0` or `1`; anything else is
    /// `Invalid("bool")`, so each `bool` has exactly one encoding.
    pub fn bool(&mut self) -> Result<bool, CodecError> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(CodecError::Invalid("bool")),
        }
    }

    /// A fixed-size field with no length prefix (see [`super::Writer::bytes`]).
    pub fn bytes<const N: usize>(&mut self) -> Result<[u8; N], CodecError> {
        let s = self.take(N)?;
        let mut out = [0u8; N];
        out.copy_from_slice(s);
        Ok(out)
    }

    /// `str16`: u16 LE byte length, then that many bytes, which must be valid
    /// UTF-8 (otherwise [`CodecError::InvalidUtf8`]). The length is checked
    /// against the remaining input before the UTF-8 check.
    pub fn str16(&mut self) -> Result<&'a str, CodecError> {
        let b = self.bytes16()?;
        std::str::from_utf8(b).map_err(|_| CodecError::InvalidUtf8)
    }

    /// `bytes16`: u16 LE length, then that many raw bytes.
    pub fn bytes16(&mut self) -> Result<&'a [u8], CodecError> {
        let n = usize::from(self.u16()?);
        self.take(n)
    }

    /// `list_u8`: u8 item count, then that many items read by `item`. The
    /// count is only a hint for pre-allocation, capped at
    /// [`LIST_PREALLOC_CAP`] and at the number of bytes still unread (every
    /// item occupies at least one byte on the wire, so more items than
    /// bytes cannot decode); a count larger than the input can satisfy
    /// fails with the first item's [`CodecError::UnexpectedEof`].
    pub fn list_u8<T, F>(&mut self, mut item: F) -> Result<Vec<T>, CodecError>
    where
        F: FnMut(&mut Reader<'a>) -> Result<T, CodecError>,
    {
        let n = usize::from(self.u8()?);
        let hint = n.min(LIST_PREALLOC_CAP).min(self.remaining());
        let mut out = Vec::with_capacity(hint);
        for _ in 0..n {
            out.push(item(self)?);
        }
        Ok(out)
    }
}
