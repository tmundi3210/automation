//! Relay hardening (BRIEF_v2 §5.5–5.6, DECISIONS D4 and §3 rows): lanes,
//! per-link limits, class scheduler, bounded buffers and link-slot policy.

pub mod buffers;
pub mod lanes;
pub mod limits;
pub mod sched;
pub mod slots;

pub use lanes::Lane;
