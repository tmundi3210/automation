//! Lanes (BRIEF_v2 §5.5 "Rate limits per inbound link"): what a relay can
//! vouch for about a cell it is asked to carry. Relays never see senders, so
//! the lane depends only on the inbound link and on whether this phone could
//! validate the cell itself.
//!
//! The lane type is fixed here; `classify` and its tests are completed in
//! milestone 1 wave 2 in this same file.

/// Lane of a cell on a given inbound link. Lower = more trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Lane {
    /// Member-validated: this phone recognised the cell (tag matched a chain
    /// it holds) and the inner signature/roster check passed.
    L1 = 1,
    /// Received over a link to a QR-verified contact (ContactProof matched).
    L2 = 2,
    /// Opaque: nothing could be validated. Capped at 25 % of a link's budget
    /// and evicted first (except on stations, where it is the whole budget).
    L3 = 3,
}

impl Lane {
    pub const ALL: [Lane; 3] = [Lane::L1, Lane::L2, Lane::L3];

    /// Share of a non-station relay's per-link budget that opaque traffic may
    /// use (BRIEF_v2 §5.5; assumption from DECISIONS D4).
    pub const L3_SHARE_ASM: f32 = 0.25;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lane_order_is_trust_order() {
        assert!(Lane::L1 < Lane::L2 && Lane::L2 < Lane::L3);
    }
}
