//! Codec failure modes.
//!
//! Every way a decoder can refuse input is one variant here, so callers (the
//! relay's "checks format, size and TTL" step, BRIEF_v2 §5.5; object
//! admission, DESIGN.md §2.4) can log or count rejections without string
//! matching. Decoders never panic on malformed input and never allocate from
//! an untrusted length before the bytes it describes are known to exist.

/// Why an encode or decode refused to continue.
///
/// DESIGN.md §2.1: "Decoders reject trailing bytes, over-long lengths and
/// unknown enum values." Each of those rules has its own variant below;
/// [`CodecError::Invalid`] covers object-specific rules (a non-canonical
/// `bool`, an unsorted head list, a zero count where one is required) so that
/// object decoders do not need their own error enum for format checks.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CodecError {
    /// A read of `needed` bytes was attempted with only `remaining` left.
    #[error("unexpected end of input: needed {needed} byte(s), {remaining} remaining")]
    UnexpectedEof { needed: usize, remaining: usize },
    /// The object decoded cleanly but this many bytes were left over. The top-level
    /// [`decode`](super::decode) rejects this so that two byte strings never
    /// decode to the same value (DESIGN.md §2.1 "canonical by construction").
    #[error("{0} trailing byte(s) after the object")]
    TrailingBytes(usize),
    /// A `str16` whose bytes are not valid UTF-8.
    #[error("string is not valid UTF-8")]
    InvalidUtf8,
    /// An encoder was handed a value longer than its length prefix can carry
    /// (`str16`/`bytes16`: 65535 bytes; `list_u8`: 255 items). This is the
    /// only way encoding can fail.
    #[error("{what} too long: max {max}, got {got}")]
    LengthOverflow {
        what: &'static str,
        max: usize,
        got: usize,
    },
    /// An enum discriminant (object `kind`, `ver`, `op`, ...) that this
    /// version of the protocol does not know. Built with [`unknown_variant`].
    #[error("unknown {what} value {value}")]
    UnknownVariant { what: &'static str, value: u64 },
    /// A field decoded but violates a rule of its object, named by `what`.
    #[error("invalid {0}")]
    Invalid(&'static str),
}

/// Builds the error an enum decoder returns for a discriminant it does not
/// recognise. DESIGN.md §2.1: every object carries a `ver` byte and decoders
/// "reject unknown version"; the same applies to `kind`, `op` and setting
/// keys.
///
/// ```
/// use meshcore::codec::{CodecError, unknown_variant};
/// let e = unknown_variant("task op", 9u8);
/// assert_eq!(e, CodecError::UnknownVariant { what: "task op", value: 9 });
/// ```
pub fn unknown_variant(what: &'static str, value: impl Into<u64>) -> CodecError {
    CodecError::UnknownVariant {
        what,
        value: value.into(),
    }
}
