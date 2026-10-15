//! Task operation body (docs/DESIGN.md §2.4 "TaskOp body"; BRIEF_v2 §4
//! Tasks).
//!
//! A task op travels as the body of a group post of kind
//! [`kind::POST_TASK_OP`](crate::objects::kind::POST_TASK_OP); the post's
//! Sender Key chain binds the author (docs/decisions-assumed.md row 12), so
//! the body carries no signature of its own. Layout:
//!
//! ```text
//! task_id [16] ‖ op u8 {1 Create, 2 Take, 3 Release, 4 Done} ‖ n_heads u8 ‖
//! task_heads [32]×n (sorted ascending) ‖ (Create only: title str16 ‖ details str16)
//! ```
//!
//! Each task is its own small DAG (DESIGN §4 "Ops"): `task_heads` are the
//! hashes of the task ops the author had seen for this task, so order is
//! causal and no timestamp decides anything (BRIEF_v2 §4 "Model").
//!
//! Identifiers (DESIGN §2.4) are *author-bound*: the post plaintext
//! (`ver ‖ kind ‖ n_heads ‖ roster_heads ‖ body`, `objects/chat_post.rs`)
//! carries no author field, so two members who emit byte-identical ops (the
//! common case: both Take right after the Create, citing the same heads)
//! would otherwise share one hash and merge into one op. Instead
//!
//! ```text
//! op hash = SHA256("meshapp.v1.taskop" ‖ 0x00 ‖ author [32] ‖ post plaintext)
//! task_id = Trunc16(op hash of the Create)
//! ```
//!
//! ([`task_op_hash`], [`task_id_for_create`]), where `author` is the
//! Sender Key chain owner of the post, which the group layer knows for every
//! post it decrypts (DESIGN §2.4 "author = chain owner"). Every task op
//! therefore has exactly one identity per author, and a `task_heads`
//! citation names one member's copy. Because the Create's own id is derived
//! from the bytes that contain it, a Create carries the all-zero `task_id`
//! on the wire and cites no heads (there is nothing yet to cite); decoders
//! enforce both so every task has exactly one Create encoding per author.

use crate::codec::{CodecError, Decode, Encode, ListItem, Reader, Writer, unknown_variant};
use crate::crypto::ed25519::IdentityKey;
use crate::crypto::hash::{Hash32, object_hash};
use crate::objects::MAX_HEADS;

/// A task's 16-byte id: `Trunc16(task_op_hash(author, Create post
/// plaintext))` ([`task_id_for_create`]).
pub type TaskId = [u8; 16];

/// Domain label of the author-bound task op hash (DESIGN §2.4; the same
/// `domain ‖ 0x00 ‖ content` shape as every other object hash, §2.1).
pub const TASK_OP_DOMAIN: &str = "meshapp.v1.taskop";

/// Largest title, in UTF-8 bytes (module spec: ≤ 120 B).
pub const MAX_TITLE_BYTES: usize = 120;
/// Largest details text, in UTF-8 bytes (module spec: ≤ 2000 B, the same
/// cap as a text post, DESIGN §2.4).
pub const MAX_DETAILS_BYTES: usize = 2000;
/// Fixed bytes before the heads: `task_id ‖ op ‖ n_heads`.
pub const TASK_OP_FIXED_LEN: usize = 16 + 1 + 1;

/// The four task operations (DESIGN §4 "Ops"; BRIEF_v2 §7 vocabulary:
/// "Take" and "Release" are the only words for claiming and giving up).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TaskKind {
    /// Opens the task. Any member may create (docs/decisions-assumed.md row 5).
    Create,
    /// Claims the task; takes effect locally at once (BRIEF_v2 §4 "Take task").
    Take,
    /// Gives up the author's claim.
    Release,
    /// Marks the task finished; valid from any current claimant.
    Done,
}

impl TaskKind {
    /// Wire discriminant (DESIGN §2.4 `op u8 {1 Create, 2 Take, 3 Release, 4 Done}`).
    pub const fn to_u8(self) -> u8 {
        match self {
            TaskKind::Create => 1,
            TaskKind::Take => 2,
            TaskKind::Release => 3,
            TaskKind::Done => 4,
        }
    }

    /// Inverse of [`TaskKind::to_u8`]; unknown values are a codec error.
    pub fn from_u8(v: u8) -> Result<Self, CodecError> {
        match v {
            1 => Ok(TaskKind::Create),
            2 => Ok(TaskKind::Take),
            3 => Ok(TaskKind::Release),
            4 => Ok(TaskKind::Done),
            other => Err(unknown_variant("task op", other)),
        }
    }
}

/// Why a task op could not be built.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum TaskOpError {
    #[error("title exceeds {MAX_TITLE_BYTES} bytes: {0}")]
    TitleTooLong(usize),
    #[error("details exceed {MAX_DETAILS_BYTES} bytes: {0}")]
    DetailsTooLong(usize),
    #[error("too many task heads: {0}")]
    TooManyHeads(usize),
    #[error("task heads must be sorted ascending without duplicates")]
    HeadsNotSorted,
    /// A Create must carry the zero `task_id` and no heads; every other op
    /// must carry no title or details.
    #[error("field not allowed for this op: {0}")]
    FieldNotAllowed(&'static str),
}

/// One task operation (module docs).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskOp {
    /// The task this op belongs to; all zero for a Create (module docs).
    pub task_id: TaskId,
    pub op: TaskKind,
    /// Task ops the author had seen for this task: sorted ascending, no
    /// duplicates, at most [`MAX_HEADS`]; empty for a Create.
    pub task_heads: Vec<Hash32>,
    /// Create only (≤ [`MAX_TITLE_BYTES`]); empty otherwise.
    pub title: String,
    /// Create only (≤ [`MAX_DETAILS_BYTES`]); empty otherwise.
    pub details: String,
}

impl ListItem for TaskOp {}

impl TaskOp {
    /// A Create op. The wire `task_id` is zero; the real id is
    /// [`task_id_for_create`] over the author and the post the caller wraps
    /// this in.
    pub fn create(
        title: impl Into<String>,
        details: impl Into<String>,
    ) -> Result<Self, TaskOpError> {
        let op = TaskOp {
            task_id: [0u8; 16],
            op: TaskKind::Create,
            task_heads: Vec::new(),
            title: title.into(),
            details: details.into(),
        };
        op.check()?;
        Ok(op)
    }

    /// A Take, Release or Done op on `task_id`, citing `task_heads`
    /// (normalised: sorted, deduplicated). `Create` is refused here; use
    /// [`TaskOp::create`].
    pub fn on_task(
        task_id: TaskId,
        op: TaskKind,
        mut task_heads: Vec<Hash32>,
    ) -> Result<Self, TaskOpError> {
        if op == TaskKind::Create {
            return Err(TaskOpError::FieldNotAllowed("create via on_task"));
        }
        task_heads.sort_unstable();
        task_heads.dedup();
        let op = TaskOp {
            task_id,
            op,
            task_heads,
            title: String::new(),
            details: String::new(),
        };
        op.check()?;
        Ok(op)
    }

    /// Format rules (module docs). Decoding enforces the same rules, so a
    /// decoded op always passes.
    pub fn check(&self) -> Result<(), TaskOpError> {
        if self.task_heads.len() > MAX_HEADS {
            return Err(TaskOpError::TooManyHeads(self.task_heads.len()));
        }
        if self.task_heads.windows(2).any(|w| w[0] >= w[1]) {
            return Err(TaskOpError::HeadsNotSorted);
        }
        match self.op {
            TaskKind::Create => {
                if self.task_id != [0u8; 16] {
                    return Err(TaskOpError::FieldNotAllowed("create task_id"));
                }
                if !self.task_heads.is_empty() {
                    return Err(TaskOpError::FieldNotAllowed("create task_heads"));
                }
                if self.title.len() > MAX_TITLE_BYTES {
                    return Err(TaskOpError::TitleTooLong(self.title.len()));
                }
                if self.details.len() > MAX_DETAILS_BYTES {
                    return Err(TaskOpError::DetailsTooLong(self.details.len()));
                }
            }
            TaskKind::Take | TaskKind::Release | TaskKind::Done => {
                if !self.title.is_empty() {
                    return Err(TaskOpError::FieldNotAllowed("title"));
                }
                if !self.details.is_empty() {
                    return Err(TaskOpError::FieldNotAllowed("details"));
                }
            }
        }
        Ok(())
    }

    /// Decodes a body produced by [`Encode::encode`].
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CodecError> {
        crate::codec::decode(bytes)
    }
}

fn codec_err(e: TaskOpError) -> CodecError {
    match e {
        TaskOpError::TitleTooLong(_) => CodecError::Invalid("task title length"),
        TaskOpError::DetailsTooLong(_) => CodecError::Invalid("task details length"),
        TaskOpError::TooManyHeads(_) | TaskOpError::HeadsNotSorted => {
            CodecError::Invalid("task heads")
        }
        TaskOpError::FieldNotAllowed(what) => CodecError::Invalid(what),
    }
}

impl Encode for TaskOp {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        self.check().map_err(codec_err)?;
        w.bytes(&self.task_id).u8(self.op.to_u8());
        w.list_u8(&self.task_heads, |w, h| {
            w.bytes(h.as_bytes());
            Ok(())
        })?;
        if self.op == TaskKind::Create {
            w.str16(&self.title)?;
            w.str16(&self.details)?;
        }
        Ok(())
    }
}

impl Decode for TaskOp {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let task_id: TaskId = r.bytes()?;
        let op = TaskKind::from_u8(r.u8()?)?;
        let task_heads: Vec<Hash32> = r.list_u8(|r| Ok(Hash32::from_bytes(r.bytes()?)))?;
        let (title, details) = if op == TaskKind::Create {
            (r.str16()?.to_owned(), r.str16()?.to_owned())
        } else {
            (String::new(), String::new())
        };
        let out = TaskOp {
            task_id,
            op,
            task_heads,
            title,
            details,
        };
        out.check().map_err(codec_err)?;
        Ok(out)
    }
}

/// The author-bound identity of a task op (DESIGN §2.4):
/// `SHA256("meshapp.v1.taskop" ‖ 0x00 ‖ author ‖ post plaintext)`, i.e.
/// [`object_hash`] under [`TASK_OP_DOMAIN`] with the author's identity key
/// prepended to the whole post plaintext. This is the value other ops cite
/// in `task_heads` and the tie-break ranks over. `author` is the chain
/// owner of the post (module docs); the node supplies it.
pub fn task_op_hash(author: &IdentityKey, post_plaintext: &[u8]) -> Hash32 {
    let mut bytes = Vec::with_capacity(author.as_bytes().len() + post_plaintext.len());
    bytes.extend_from_slice(author.as_bytes());
    bytes.extend_from_slice(post_plaintext);
    object_hash(TASK_OP_DOMAIN, &bytes)
}

/// `task_id = Trunc16(task_op_hash(author, Create post plaintext))`
/// (DESIGN §2.4): two members who Create byte-identical tasks get two
/// tasks.
pub fn task_id_for_create(author: &IdentityKey, create_post_plaintext: &[u8]) -> TaskId {
    task_op_hash(author, create_post_plaintext).truncate16()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::{decode, encode};
    use proptest::prelude::*;

    fn h(i: u8) -> Hash32 {
        Hash32::from_bytes([i; 32])
    }

    #[test]
    fn create_known_answer_and_roundtrip() {
        let op = TaskOp::create("Water run", "2 jerry cans to the school").unwrap();
        let bytes = encode(&op);
        let mut expect = vec![0u8; 16];
        expect.push(1);
        expect.push(0);
        expect.extend_from_slice(&[9, 0]);
        expect.extend_from_slice(b"Water run");
        expect.extend_from_slice(&[26, 0]);
        expect.extend_from_slice(b"2 jerry cans to the school");
        assert_eq!(bytes, expect);
        assert_eq!(decode::<TaskOp>(&bytes).unwrap(), op);
    }

    #[test]
    fn take_release_done_roundtrip_with_sorted_heads() {
        for kind in [TaskKind::Take, TaskKind::Release, TaskKind::Done] {
            let op = TaskOp::on_task([7; 16], kind, vec![h(9), h(2), h(2)]).unwrap();
            assert_eq!(op.task_heads, vec![h(2), h(9)]);
            let bytes = encode(&op);
            assert_eq!(bytes.len(), TASK_OP_FIXED_LEN + 64);
            assert_eq!(bytes[16], kind.to_u8());
            assert_eq!(decode::<TaskOp>(&bytes).unwrap(), op);
        }
    }

    #[test]
    fn constructors_enforce_caps_and_field_rules() {
        assert_eq!(
            TaskOp::create("a".repeat(MAX_TITLE_BYTES + 1), "").unwrap_err(),
            TaskOpError::TitleTooLong(MAX_TITLE_BYTES + 1)
        );
        assert_eq!(
            TaskOp::create("", "b".repeat(MAX_DETAILS_BYTES + 1)).unwrap_err(),
            TaskOpError::DetailsTooLong(MAX_DETAILS_BYTES + 1)
        );
        assert!(TaskOp::create("a".repeat(MAX_TITLE_BYTES), "b".repeat(MAX_DETAILS_BYTES)).is_ok());
        assert!(matches!(
            TaskOp::on_task([1; 16], TaskKind::Create, vec![]),
            Err(TaskOpError::FieldNotAllowed(_))
        ));
        let many: Vec<Hash32> = (0..=MAX_HEADS as u8).map(h).collect();
        assert_eq!(
            TaskOp::on_task([1; 16], TaskKind::Take, many).unwrap_err(),
            TaskOpError::TooManyHeads(MAX_HEADS + 1)
        );
        // hand-assembled values are checked at encode time
        let mut bad = TaskOp::on_task([1; 16], TaskKind::Take, vec![]).unwrap();
        bad.title = "x".into();
        assert!(bad.try_encode().is_err());
        let mut bad = TaskOp::create("t", "d").unwrap();
        bad.task_id = [1; 16];
        assert!(bad.try_encode().is_err());
        bad.task_id = [0; 16];
        bad.task_heads = vec![h(1)];
        assert!(bad.try_encode().is_err());
    }

    #[test]
    fn decoder_rejects_malformed_input() {
        let good = encode(&TaskOp::on_task([3; 16], TaskKind::Take, vec![h(1), h(2)]).unwrap());
        let mut bad = good.clone();
        bad[16] = 9;
        assert_eq!(decode::<TaskOp>(&bad), Err(unknown_variant("task op", 9u8)));
        let mut bad = good.clone();
        bad[16] = 0;
        assert!(decode::<TaskOp>(&bad).is_err());
        // unsorted heads
        let mut bad = good.clone();
        let (a, b) = (bad[18..50].to_vec(), bad[50..82].to_vec());
        bad[18..50].copy_from_slice(&b);
        bad[50..82].copy_from_slice(&a);
        assert_eq!(
            decode::<TaskOp>(&bad),
            Err(CodecError::Invalid("task heads"))
        );
        // duplicate heads
        let mut bad = good.clone();
        bad[50..82].copy_from_slice(&a);
        assert_eq!(
            decode::<TaskOp>(&bad),
            Err(CodecError::Invalid("task heads"))
        );
        // every strict prefix is EOF, any extra byte is trailing
        for cut in 0..good.len() {
            assert!(matches!(
                decode::<TaskOp>(&good[..cut]),
                Err(CodecError::UnexpectedEof { .. })
            ));
        }
        let mut longer = good.clone();
        longer.push(0);
        assert_eq!(decode::<TaskOp>(&longer), Err(CodecError::TrailingBytes(1)));
        // a Create with a non-zero id, with heads, or an over-long title
        let mut bad = encode(&TaskOp::create("t", "d").unwrap());
        bad[0] = 1;
        assert_eq!(
            decode::<TaskOp>(&bad),
            Err(CodecError::Invalid("create task_id"))
        );
        let mut bad = vec![0u8; 16];
        bad.extend_from_slice(&[1, 1]);
        bad.extend_from_slice(&[5; 32]);
        bad.extend_from_slice(&[0, 0, 0, 0]);
        assert_eq!(
            decode::<TaskOp>(&bad),
            Err(CodecError::Invalid("create task_heads"))
        );
        let mut bad = vec![0u8; 16];
        bad.extend_from_slice(&[1, 0]);
        bad.extend_from_slice(&((MAX_TITLE_BYTES + 1) as u16).to_le_bytes());
        bad.extend(std::iter::repeat_n(b'a', MAX_TITLE_BYTES + 1));
        bad.extend_from_slice(&[0, 0]);
        assert_eq!(
            decode::<TaskOp>(&bad),
            Err(CodecError::Invalid("task title length"))
        );
    }

    #[test]
    fn ids_are_author_bound_hashes_of_the_post_plaintext() {
        use crate::crypto::ed25519::IdentitySigner;
        use crate::crypto::hash::sha256;
        let a = IdentitySigner::from_seed(&[1; 32]).public();
        let b = IdentitySigner::from_seed(&[2; 32]).public();
        let post = b"pretend post plaintext";
        // known structure: domain ‖ 0x00 ‖ author ‖ plaintext
        let mut pre = TASK_OP_DOMAIN.as_bytes().to_vec();
        pre.push(0);
        pre.extend_from_slice(a.as_bytes());
        pre.extend_from_slice(post);
        assert_eq!(task_op_hash(&a, post), sha256(&pre));
        assert_eq!(
            task_id_for_create(&a, post),
            task_op_hash(&a, post).truncate16()
        );
        // identical bytes, different authors: different op and task identities
        assert_ne!(task_op_hash(&a, post), task_op_hash(&b, post));
        assert_ne!(task_id_for_create(&a, post), task_id_for_create(&b, post));
        // and different bytes under one author differ too
        assert_ne!(task_op_hash(&a, post), task_op_hash(&a, b"other"));
        // no ambiguity between author and plaintext boundaries: the author
        // is fixed-width, so shifting a byte across changes the digest
        let mut shifted = a.as_bytes().to_vec();
        shifted.extend_from_slice(post);
        assert_ne!(task_op_hash(&a, post), sha256(&shifted));
    }

    proptest! {
        #[test]
        fn decoder_never_panics_and_roundtrips(bytes in proptest::collection::vec(any::<u8>(), 0..300)) {
            if let Ok(op) = decode::<TaskOp>(&bytes) {
                prop_assert!(op.check().is_ok());
                prop_assert_eq!(encode(&op), bytes);
            }
        }

        #[test]
        fn roundtrip_from_values(
            kind in 1u8..=4,
            heads in proptest::collection::btree_set(any::<[u8; 32]>(), 0..=MAX_HEADS),
            title in "[a-zA-Z0-9 ]{0,120}",
            details in "[a-zA-Z0-9 ]{0,200}",
            id in any::<[u8; 16]>(),
        ) {
            let kind = TaskKind::from_u8(kind).unwrap();
            let op = if kind == TaskKind::Create {
                TaskOp::create(title, details).unwrap()
            } else {
                TaskOp::on_task(id, kind, heads.into_iter().map(Hash32::from_bytes).collect()).unwrap()
            };
            let bytes = encode(&op);
            prop_assert_eq!(decode::<TaskOp>(&bytes).unwrap(), op);
        }
    }
}
