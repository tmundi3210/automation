//! Delay-tolerant store-and-forward (BRIEF_v2 §5.5, §5.7; DECISIONS D4
//! MERGE, D10 (4)): the cell store, the msg_id dedupe cache, Bloom summary
//! vectors and the per-link anti-entropy state machine.

pub mod dedupe;
pub mod store;
pub mod summary;
pub mod sync;
