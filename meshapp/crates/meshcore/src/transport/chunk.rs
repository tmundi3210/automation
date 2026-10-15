//! Stream framing of Noise messages (docs/DESIGN.md §1 `transport/chunk.rs`
//! "stream framing/reassembly of length-prefixed Noise messages
//! (MTU-agnostic)"; §2.2 "the link carries a *byte stream* of
//! length-prefixed Noise frames (frame = 2 + 4 + 512 + 16 = 534 B) chunked
//! at MTU−3 by the platform"; §10 fuzz target `noise_stream`).
//!
//! On the wire every Noise message is `len u16 LE ‖ bytes` ([`frame`]). The
//! platform splits that stream at whatever its radio takes (ATT MTU−3: 17,
//! 182, 244, 514 bytes...) and [`StreamFramer`] puts it back together from
//! any chunk boundaries, including a prefix split across two chunks.
//!
//! The framer does not look inside a frame: everything it yields goes to
//! [`NoiseLink::decrypt_frame`](crate::wire::noise::NoiseLink::decrypt_frame),
//! which rejects anything that is not a message of this session.
//!
//! Hard caps on untrusted input:
//!
//! - a frame is at most [`MAX_FRAME_LEN`] (the prefix cannot express more,
//!   which matches the Noise 65535-byte message limit);
//! - a frame is at least [`MIN_FRAME_LEN`] = 16 bytes: no Noise message is
//!   shorter than its Poly1305 tag (handshake messages are 32 and 48
//!   bytes), so a smaller length prefix is a protocol violation. Refusing it
//!   at the parser also keeps a flood of tiny frames from costing more queue
//!   memory than the byte caps account for (each queued frame has a fixed
//!   overhead well above its payload when that payload is near empty);
//! - the reassembly buffer holds at most [`MAX_BUFFERED_BYTES`] (four
//!   maximal frames); more means the consumer is not draining or the peer
//!   is flooding, and [`StreamFramer::push`] fails.
//!
//! After any error the framer is poisoned and the link must be reset
//! (DESIGN §7: a broken stream tears the link down and a new link
//! re-handshakes); [`StreamFramer::reset`] is for a fresh link.

use core::fmt;

use crate::wire::frame::{NOISE_LENGTH_PREFIX_LEN, NOISE_TAG_LEN};

/// The `u16 LE` length prefix (DESIGN §1 `wire/noise.rs` "2-B length framing").
pub const LENGTH_PREFIX_LEN: usize = NOISE_LENGTH_PREFIX_LEN;
/// Largest frame the prefix can express; equal to the Noise message limit.
pub const MAX_FRAME_LEN: usize = u16::MAX as usize;
/// Smallest frame the parser accepts: a Noise message is never shorter than
/// its tag (module doc).
pub const MIN_FRAME_LEN: usize = NOISE_TAG_LEN;
/// Most bytes a [`StreamFramer`] holds before it fails: four maximal
/// prefixed frames.
pub const MAX_BUFFERED_BYTES: usize = 4 * (MAX_FRAME_LEN + LENGTH_PREFIX_LEN);

const _: () = assert!(MAX_FRAME_LEN == 65535);
const _: () = assert!(LENGTH_PREFIX_LEN == 2);
const _: () = assert!(MIN_FRAME_LEN == 16);

/// Errors of the stream framer. `TooLarge` comes only from [`frame`]; every
/// other variant means the framer is poisoned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum FrameError {
    /// [`frame`] was given more than [`MAX_FRAME_LEN`] bytes.
    #[error("frame of {0} bytes exceeds the 65535-byte limit")]
    TooLarge(usize),
    /// The stream announced a frame shorter than [`MIN_FRAME_LEN`].
    #[error("frame of {0} bytes is shorter than a Noise tag")]
    TooShort(usize),
    /// Accepting the chunk would hold more than [`MAX_BUFFERED_BYTES`];
    /// carries the number of bytes that would have been pending.
    #[error("stream framer overflow: {0} bytes pending")]
    Overflow(usize),
    /// An earlier error poisoned the framer; reset the link.
    #[error("stream framer poisoned by an earlier error")]
    Poisoned,
}

/// Prefixes `bytes` with their `u16 LE` length (the outbound half of the
/// framing). `TooLarge` above [`MAX_FRAME_LEN`]. The encoder does not
/// enforce [`MIN_FRAME_LEN`]: the core only ever sends Noise messages, and
/// the peer's parser is the one that refuses a short frame.
pub fn frame(bytes: &[u8]) -> Result<Vec<u8>, FrameError> {
    let len = u16::try_from(bytes.len()).map_err(|_| FrameError::TooLarge(bytes.len()))?;
    let mut out = Vec::with_capacity(LENGTH_PREFIX_LEN + bytes.len());
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(bytes);
    Ok(out)
}

/// Bytes `frame` would produce for a payload of `payload_len` bytes (no
/// check that it fits; for sizing queues).
pub const fn framed_len(payload_len: usize) -> usize {
    LENGTH_PREFIX_LEN + payload_len
}

/// Reassembles length-prefixed frames from arbitrary chunks of the inbound
/// byte stream (module doc).
#[derive(Clone, Default)]
pub struct StreamFramer {
    buf: Vec<u8>,
    poisoned: bool,
}

impl fmt::Debug for StreamFramer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamFramer")
            .field("pending_bytes", &self.buf.len())
            .field("poisoned", &self.poisoned)
            .finish()
    }
}

impl StreamFramer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends one chunk of the stream, of any size including zero.
    /// `Overflow` if the pending bytes would exceed [`MAX_BUFFERED_BYTES`]
    /// (nothing consumed), `TooShort` if the frame now announced at the head
    /// of the stream is shorter than [`MIN_FRAME_LEN`], `Poisoned` after any
    /// earlier error. Every error poisons the framer.
    pub fn push(&mut self, chunk: &[u8]) -> Result<(), FrameError> {
        if self.poisoned {
            return Err(FrameError::Poisoned);
        }
        let pending = self.buf.len().saturating_add(chunk.len());
        if pending > MAX_BUFFERED_BYTES {
            self.poison();
            return Err(FrameError::Overflow(pending));
        }
        self.buf.extend_from_slice(chunk);
        self.check_head()
    }

    /// The next complete frame (without its prefix), if one is pending, and
    /// `None` once the framer is poisoned. A too-short prefix behind the
    /// frame being drained poisons the framer here, silently: the frame is
    /// still returned, the next call answers `None`, the next
    /// [`push`](Self::push) answers `Poisoned` and
    /// [`is_poisoned`](Self::is_poisoned) says so.
    pub fn next_frame(&mut self) -> Option<Vec<u8>> {
        if self.poisoned {
            return None;
        }
        let len = self.head_len()?;
        let end = LENGTH_PREFIX_LEN + len;
        let frame = self.buf.get(LENGTH_PREFIX_LEN..end)?.to_vec();
        self.buf.drain(..end);
        // The frame just drained is valid whatever follows it; a too-short
        // prefix behind it poisons the framer for the *next* call.
        let _ = self.check_head();
        Some(frame)
    }

    /// Bytes received but not yet returned as a frame (partial frame or
    /// frames not yet pulled with [`next_frame`](Self::next_frame)).
    pub fn pending_bytes(&self) -> usize {
        self.buf.len()
    }

    /// `true` after an error: the stream is unusable until
    /// [`reset`](Self::reset).
    pub fn is_poisoned(&self) -> bool {
        self.poisoned
    }

    /// Forgets everything, including the poison, for a fresh link.
    pub fn reset(&mut self) {
        self.buf.clear();
        self.poisoned = false;
    }

    /// The length the prefix at the head announces, once both prefix bytes
    /// are in.
    fn head_len(&self) -> Option<usize> {
        let prefix = self.buf.get(..LENGTH_PREFIX_LEN)?;
        Some(usize::from(u16::from_le_bytes([prefix[0], prefix[1]])))
    }

    /// Refuses a head prefix below [`MIN_FRAME_LEN`] as soon as it is
    /// visible, poisoning the framer.
    fn check_head(&mut self) -> Result<(), FrameError> {
        match self.head_len() {
            Some(len) if len < MIN_FRAME_LEN => {
                self.poison();
                Err(FrameError::TooShort(len))
            }
            _ => Ok(()),
        }
    }

    fn poison(&mut self) {
        self.buf.clear();
        self.poisoned = true;
    }
}

// ---- tests ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn frame_prefixes_little_endian_and_refuses_oversize() {
        assert_eq!(frame(&[]).unwrap(), vec![0, 0]);
        assert_eq!(frame(&[9]).unwrap(), vec![1, 0, 9]);
        let f = frame(&[7u8; 516]).unwrap();
        assert_eq!(&f[..2], &[0x04, 0x02]);
        assert_eq!(f.len(), framed_len(516));
        let max = frame(&[1u8; MAX_FRAME_LEN]).unwrap();
        assert_eq!(&max[..2], &[0xFF, 0xFF]);
        assert_eq!(max.len(), MAX_FRAME_LEN + 2);
        assert_eq!(
            frame(&[1u8; MAX_FRAME_LEN + 1]),
            Err(FrameError::TooLarge(MAX_FRAME_LEN + 1))
        );
    }

    #[test]
    fn reassembles_across_every_split_point_of_two_frames() {
        let a = frame(b"alpha, sixteen b").unwrap();
        let b = frame(&[0u8; 300]).unwrap();
        let stream: Vec<u8> = a.iter().chain(b.iter()).copied().collect();
        for split in 0..=stream.len() {
            let mut fr = StreamFramer::new();
            fr.push(&stream[..split]).unwrap();
            let mut got = Vec::new();
            while let Some(f) = fr.next_frame() {
                got.push(f);
            }
            fr.push(&stream[split..]).unwrap();
            while let Some(f) = fr.next_frame() {
                got.push(f);
            }
            assert_eq!(
                got,
                vec![b"alpha, sixteen b".to_vec(), vec![0u8; 300]],
                "split {split}"
            );
            assert_eq!(fr.pending_bytes(), 0);
        }
    }

    #[test]
    fn one_byte_chunks_and_a_maximal_frame() {
        let payload: Vec<u8> = (0..MAX_FRAME_LEN).map(|i| (i % 251) as u8).collect();
        let stream = frame(&payload).unwrap();
        let mut fr = StreamFramer::new();
        for b in &stream[..stream.len() - 1] {
            fr.push(&[*b]).unwrap();
            assert!(fr.next_frame().is_none());
        }
        fr.push(&stream[stream.len() - 1..]).unwrap();
        assert_eq!(fr.next_frame().unwrap(), payload);
        assert!(fr.next_frame().is_none());
        // the minimum is accepted
        fr.push(&frame(&[3u8; MIN_FRAME_LEN]).unwrap()).unwrap();
        assert_eq!(fr.next_frame().unwrap(), vec![3u8; MIN_FRAME_LEN]);
    }

    #[test]
    fn short_frames_poison_as_soon_as_the_prefix_is_visible() {
        for len in [0u16, 1, 15] {
            let mut fr = StreamFramer::new();
            fr.push(&len.to_le_bytes()[..1]).unwrap();
            assert_eq!(
                fr.push(&len.to_le_bytes()[1..]),
                Err(FrameError::TooShort(usize::from(len)))
            );
            assert!(fr.is_poisoned());
            assert_eq!(fr.pending_bytes(), 0);
            assert!(fr.next_frame().is_none());
            assert_eq!(fr.push(&[0u8; 20]), Err(FrameError::Poisoned));
        }
        // behind a good frame: only the head is judged on push, so the good
        // frame still comes out, then the poison shows
        let mut stream = frame(&[1u8; 16]).unwrap();
        stream.extend_from_slice(&[0, 0, 9, 9]);
        for split in [5usize, 18] {
            let mut fr = StreamFramer::new();
            fr.push(&stream[..split]).unwrap();
            fr.push(&stream[split..]).unwrap();
            assert!(!fr.is_poisoned(), "split {split}: the head is fine");
            assert_eq!(fr.next_frame().unwrap(), vec![1u8; 16]);
            assert!(fr.is_poisoned(), "split {split}");
            assert_eq!(fr.next_frame(), None);
            assert_eq!(fr.push(&[1]), Err(FrameError::Poisoned));
        }
        // one byte of the bad prefix behind the frame is not a verdict yet
        let mut fr = StreamFramer::new();
        fr.push(&stream[..19]).unwrap();
        assert_eq!(fr.next_frame().unwrap(), vec![1u8; 16]);
        assert!(!fr.is_poisoned());
        assert_eq!(fr.push(&stream[19..]), Err(FrameError::TooShort(0)));
        // drained in next_frame: poisoned silently, reported by the next push
        let mut fr = StreamFramer::new();
        fr.push(&frame(&[1u8; 16]).unwrap()).unwrap();
        fr.push(&[2u8]).unwrap();
        fr.push(&[0u8]).unwrap();
        assert_eq!(fr.next_frame().unwrap(), vec![1u8; 16]);
        assert!(fr.is_poisoned());
        assert_eq!(fr.next_frame(), None);
        assert_eq!(fr.push(&[1]), Err(FrameError::Poisoned));
    }

    #[test]
    fn overflow_poisons_until_reset() {
        let mut fr = StreamFramer::new();
        // a prefix promising 65535 bytes, never completed: three fit...
        for _ in 0..3 {
            fr.push(&[0xFF, 0xFF]).unwrap();
            fr.push(&[0u8; MAX_FRAME_LEN]).unwrap();
        }
        assert_eq!(fr.pending_bytes(), 3 * (MAX_FRAME_LEN + 2));
        // ...and the fourth plus one byte does not
        fr.push(&[0xFF, 0xFF]).unwrap();
        fr.push(&[0u8; MAX_FRAME_LEN - 1]).unwrap();
        assert_eq!(fr.pending_bytes(), MAX_BUFFERED_BYTES - 1);
        assert_eq!(
            fr.push(&[0, 0]),
            Err(FrameError::Overflow(MAX_BUFFERED_BYTES + 1))
        );
        assert!(fr.is_poisoned());
        assert_eq!(fr.pending_bytes(), 0);
        assert_eq!(fr.push(&[1]), Err(FrameError::Poisoned));
        assert!(fr.next_frame().is_none());
        fr.reset();
        assert!(!fr.is_poisoned());
        fr.push(&frame(&[7u8; 16]).unwrap()).unwrap();
        assert_eq!(fr.next_frame().unwrap(), vec![7u8; 16]);
        // exactly the cap is accepted
        let mut fr = StreamFramer::new();
        fr.push(&vec![0xFFu8; MAX_BUFFERED_BYTES]).unwrap();
        assert!(!fr.is_poisoned());
        let dbg = format!("{fr:?}");
        assert!(dbg.contains("pending_bytes"));
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(128))]

        /// Random splits of a frame sequence reassemble identically.
        #[test]
        fn random_splits_reassemble_identically(
            frames in prop::collection::vec(
                prop::collection::vec(any::<u8>(), MIN_FRAME_LEN..1500), 0..10),
            seed in any::<u64>(),
            max_chunk in 1usize..700,
        ) {
            let stream: Vec<u8> = frames
                .iter()
                .flat_map(|f| frame(f).unwrap())
                .collect();
            let mut rng = ChaCha20Rng::seed_from_u64(seed);
            let mut fr = StreamFramer::new();
            let mut got = Vec::new();
            let mut pos = 0;
            while pos < stream.len() {
                let n = rng.random_range(1..=max_chunk).min(stream.len() - pos);
                fr.push(&stream[pos..pos + n]).unwrap();
                pos += n;
                while let Some(f) = fr.next_frame() {
                    got.push(f);
                }
            }
            prop_assert_eq!(got, frames);
            prop_assert_eq!(fr.pending_bytes(), 0);
            prop_assert!(!fr.is_poisoned());
        }

        /// DESIGN §10 `noise_stream`: random garbage never panics and never
        /// yields anything outside the caps.
        #[test]
        fn random_garbage_never_panics(
            chunks in prop::collection::vec(prop::collection::vec(any::<u8>(), 0..3000), 0..30),
        ) {
            let mut fr = StreamFramer::new();
            for c in &chunks {
                let res = fr.push(c);
                prop_assert!(fr.pending_bytes() <= MAX_BUFFERED_BYTES);
                if res.is_err() {
                    prop_assert!(fr.is_poisoned());
                }
                while let Some(f) = fr.next_frame() {
                    prop_assert!((MIN_FRAME_LEN..=MAX_FRAME_LEN).contains(&f.len()));
                }
            }
        }
    }
}
