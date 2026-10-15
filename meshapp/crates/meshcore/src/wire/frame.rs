//! Per-hop link frame carried inside the Noise NN transport message
//! (docs/DESIGN.md §2.3) and the message classes (BRIEF_v2 §5.6).
//!
//! This file fixes the [`Class`] type that the DTN store, relay limits and
//! scheduler key on. The frame encoding and the link control messages
//! (Hello, Summary, Throttle, ContactProof) are completed in milestone 1
//! wave 2 in this same file.

use crate::codec::{CodecError, unknown_variant};

/// Message class (BRIEF_v2 §5.6). Lower discriminant = higher priority.
/// Strict priority on every queue: C0 > C1 > C2 > C3 > C4; the scheduler
/// additionally splits C2 and C3 by lane (DECISIONS §3 row 4,8,9,10).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Class {
    /// SOS, at most 256 B. Reserved at P0 (owner decision 5: not built).
    C0Sos = 0,
    /// Control: roster ops, Sender Key bundles, vote-opens, ballots, task
    /// Take/Release/Done, revocations. Segmented, objects up to 64 kB.
    C1Control = 1,
    /// Text, at most 2 kB.
    C2Text = 2,
    /// News (P2), at most 4 kB.
    C3News = 3,
    /// Media preview, at most 16 kB (B build: 8–16 kB cap, assumption).
    C4Preview = 4,
}

impl Class {
    pub const ALL: [Class; 5] = [
        Class::C0Sos,
        Class::C1Control,
        Class::C2Text,
        Class::C3News,
        Class::C4Preview,
    ];

    pub fn from_u8(v: u8) -> Result<Self, CodecError> {
        Ok(match v {
            0 => Class::C0Sos,
            1 => Class::C1Control,
            2 => Class::C2Text,
            3 => Class::C3News,
            4 => Class::C4Preview,
            other => return Err(unknown_variant("class", other)),
        })
    }

    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Largest object this class may carry, in bytes of plaintext before
    /// fragmentation (BRIEF_v2 §5.6; C1's 64 kB is the segmentation ceiling).
    pub const fn max_object_bytes(self) -> usize {
        match self {
            Class::C0Sos => 256,
            Class::C1Control => 64 * 1024,
            Class::C2Text => 2 * 1024,
            Class::C3News => 4 * 1024,
            Class::C4Preview => 16 * 1024,
        }
    }

    /// `true` if `self` has strictly higher priority than `other`.
    pub fn outranks(self, other: Class) -> bool {
        self < other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_order_is_strict_priority() {
        assert!(Class::C0Sos.outranks(Class::C1Control));
        assert!(Class::C1Control.outranks(Class::C2Text));
        assert!(Class::C2Text.outranks(Class::C3News));
        assert!(Class::C3News.outranks(Class::C4Preview));
        assert!(!Class::C4Preview.outranks(Class::C4Preview));
    }

    #[test]
    fn class_round_trips_and_rejects_unknown() {
        for c in Class::ALL {
            assert_eq!(Class::from_u8(c.as_u8()).unwrap(), c);
        }
        assert!(Class::from_u8(5).is_err());
        assert!(Class::from_u8(255).is_err());
    }

    #[test]
    fn object_caps_match_the_brief() {
        assert_eq!(Class::C0Sos.max_object_bytes(), 256);
        assert_eq!(Class::C1Control.max_object_bytes(), 65_536);
        assert_eq!(Class::C2Text.max_object_bytes(), 2_048);
        assert_eq!(Class::C3News.max_object_bytes(), 4_096);
        assert_eq!(Class::C4Preview.max_object_bytes(), 16_384);
    }
}
