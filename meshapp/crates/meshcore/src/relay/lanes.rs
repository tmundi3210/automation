//! Lanes (BRIEF_v2 §5.5 "Rate limits per inbound link"): what a relay can
//! vouch for about a cell it is asked to carry. Relays never see senders, so
//! the lane depends only on the inbound link and on whether this phone could
//! validate the cell itself (DECISIONS D4 LANES; DESIGN §1 `relay/lanes.rs`).
//!
//! [`classify`] is the one place the lane rule lives;
//! [`crate::transport::link::Link::lane_for`] delegates to it.

use crate::transport::link::{LinkAuth, Validation};

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

    /// [`Lane::L3_SHARE_ASM`] as an exact integer (per mille), so the token
    /// buckets in `relay/limits` never touch floating point and every phone
    /// computes the same allowance (determinism rule, DESIGN §1 conventions).
    pub const L3_SHARE_PERMILLE_ASM: u64 = 250;

    /// Whether the new-link ramp (DECISIONS D4 PER INBOUND LINK "new links
    /// start at 25 % of quota and ramp up over 10 min") applies to this lane.
    /// BRIEF_v2 §5.5: the ramp "limits only traffic the receiver cannot
    /// validate", so it applies to L3 alone; DECISIONS §3 row (4,8,9,10):
    /// "a locked iPhone's own L1 traffic arriving from a station is never
    /// ramp-limited".
    pub const fn ramp_applies(self) -> bool {
        matches!(self, Lane::L3)
    }

    /// `true` when the receiver could vouch for the cell itself (L1) or for
    /// the link it came over (L2).
    pub const fn is_vouched(self) -> bool {
        !matches!(self, Lane::L3)
    }

    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

// The two spellings of the share must agree.
const _: () = assert!((Lane::L3_SHARE_ASM * 1000.0) as u64 == Lane::L3_SHARE_PERMILLE_ASM);

/// Lane of a cell received on a link (BRIEF_v2 §5.5 lanes; DESIGN §7
/// `lane_for`): L1 if this phone validated the cell, L2 if the link is to a
/// QR-verified contact, else L3. Validation wins over link auth because a
/// validated cell is vouched for by the roster, not by who carried it.
pub fn classify(validated: Validation, auth: LinkAuth) -> Lane {
    match (validated, auth) {
        (Validation::Validated, _) => Lane::L1,
        (Validation::Opaque, LinkAuth::VerifiedContact(_)) => Lane::L2,
        (Validation::Opaque, LinkAuth::Anonymous) => Lane::L3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::ed25519::IdentitySigner;

    fn contact() -> LinkAuth {
        let signer = IdentitySigner::from_seed(&[7u8; 32]);
        LinkAuth::VerifiedContact(signer.public())
    }

    #[test]
    fn lane_order_is_trust_order() {
        assert!(Lane::L1 < Lane::L2 && Lane::L2 < Lane::L3);
    }

    #[test]
    fn classify_follows_design_section_7() {
        assert_eq!(
            classify(Validation::Validated, LinkAuth::Anonymous),
            Lane::L1
        );
        assert_eq!(classify(Validation::Validated, contact()), Lane::L1);
        assert_eq!(classify(Validation::Opaque, contact()), Lane::L2);
        assert_eq!(classify(Validation::Opaque, LinkAuth::Anonymous), Lane::L3);
    }

    #[test]
    fn ramp_applies_to_opaque_traffic_only() {
        assert!(!Lane::L1.ramp_applies());
        assert!(!Lane::L2.ramp_applies());
        assert!(Lane::L3.ramp_applies());
        assert!(Lane::L1.is_vouched() && Lane::L2.is_vouched() && !Lane::L3.is_vouched());
    }
}
