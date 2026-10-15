//! Link-slot policy (DECISIONS D4 LINK SLOTS: "keep at least 50 % of slots
//! for outbound links the relay chooses itself (GossipSub D_out analogue,
//! Q5); when inbound slots fill, Android switches to setConnectable(false)
//! and iOS calls stopAdvertising"; DESIGN §1 `relay/slots.rs`, §7
//! "non-connectable advertising when `SlotPolicy` says inbound slots are
//! full").
//!
//! The platform reports every link up/down with its [`Role`]; the policy
//! answers with a [`PlatformHint`] only when the answer changes, so the
//! platform flips advertising at most once per transition. A link the peer
//! opened (this phone is the GATT peripheral) is *inbound*; one this phone
//! dialled (central) is *outbound-chosen*.

use crate::transport::link::Role;

/// Share of slots kept for outbound-chosen links: D4 LINK SLOTS "at least
/// 50 %" (assumption).
pub const MIN_OUTBOUND_SHARE_ASM: f32 = 0.5;

/// What the platform should do with inbound connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlatformHint {
    /// Android `setConnectable(false)` / iOS `stopAdvertising`.
    StopAcceptingInbound,
    /// Advertise connectable again.
    ResumeInbound,
}

/// Counts links by role and reserves outbound slots.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlotPolicy {
    max_links: u32,
    min_outbound_share: f32,
    inbound: u32,
    outbound: u32,
    accepting: bool,
}

impl SlotPolicy {
    /// `max_links`: device-wide GATT link cap the platform reports (Android
    /// 14: 7; 15/16: 8–16; iOS unpublished, D4 Q4/Q9).
    pub fn new(max_links: u32) -> Self {
        Self::with_share(max_links, MIN_OUTBOUND_SHARE_ASM)
    }

    /// As [`SlotPolicy::new`] with another outbound share in [0, 1].
    pub fn with_share(max_links: u32, min_outbound_share: f32) -> Self {
        let mut p = SlotPolicy {
            max_links,
            min_outbound_share: min_outbound_share.clamp(0.0, 1.0),
            inbound: 0,
            outbound: 0,
            accepting: true,
        };
        p.accepting = p.would_accept();
        p
    }

    pub fn max_links(&self) -> u32 {
        self.max_links
    }

    /// Slots kept for links this phone dials: `ceil(max_links × share)`.
    pub fn outbound_reserved(&self) -> u32 {
        let share = if self.min_outbound_share.is_nan() {
            0.0
        } else {
            self.min_outbound_share
        };
        // share ∈ [0, 1], so the product is within [0, max_links] and the
        // rounded value converts exactly.
        let reserved = (f64::from(self.max_links) * f64::from(share)).ceil();
        (reserved as u32).min(self.max_links)
    }

    /// Slots inbound links may take.
    pub fn inbound_cap(&self) -> u32 {
        self.max_links - self.outbound_reserved()
    }

    /// `(inbound, outbound)` links currently up.
    pub fn counts(&self) -> (u32, u32) {
        (self.inbound, self.outbound)
    }

    pub fn total(&self) -> u32 {
        self.inbound.saturating_add(self.outbound)
    }

    /// Whether the platform should currently accept inbound connections.
    pub fn accepting_inbound(&self) -> bool {
        self.accepting
    }

    /// Whether a slot is free for a link this phone would dial.
    pub fn can_dial(&self) -> bool {
        self.total() < self.max_links
    }

    /// Hint for a platform that is starting up with no links (the policy may
    /// refuse inbound from the start when no slots are reserved for it).
    pub fn initial_hint(&self) -> Option<PlatformHint> {
        if self.accepting {
            None
        } else {
            Some(PlatformHint::StopAcceptingInbound)
        }
    }

    /// A link came up in `role`. Links are counted even when the platform
    /// raced past a stop hint: they are real and hold a slot.
    pub fn on_link_up(&mut self, role: Role) -> Option<PlatformHint> {
        match role {
            Role::Peripheral => self.inbound = self.inbound.saturating_add(1),
            Role::Central => self.outbound = self.outbound.saturating_add(1),
        }
        self.transition()
    }

    /// A link in `role` went down.
    pub fn on_link_down(&mut self, role: Role) -> Option<PlatformHint> {
        match role {
            Role::Peripheral => self.inbound = self.inbound.saturating_sub(1),
            Role::Central => self.outbound = self.outbound.saturating_sub(1),
        }
        self.transition()
    }

    fn would_accept(&self) -> bool {
        self.inbound < self.inbound_cap() && self.total() < self.max_links
    }

    fn transition(&mut self) -> Option<PlatformHint> {
        let now = self.would_accept();
        if now == self.accepting {
            return None;
        }
        self.accepting = now;
        Some(if now {
            PlatformHint::ResumeInbound
        } else {
            PlatformHint::StopAcceptingInbound
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn android_14_seven_slots_keeps_four_for_outbound() {
        let mut p = SlotPolicy::new(7);
        assert_eq!(p.outbound_reserved(), 4);
        assert_eq!(p.inbound_cap(), 3);
        assert_eq!(p.initial_hint(), None);
        assert_eq!(p.on_link_up(Role::Peripheral), None);
        assert_eq!(p.on_link_up(Role::Peripheral), None);
        assert_eq!(
            p.on_link_up(Role::Peripheral),
            Some(PlatformHint::StopAcceptingInbound)
        );
        assert!(!p.accepting_inbound());
        // A raced fourth inbound is counted but changes no hint.
        assert_eq!(p.on_link_up(Role::Peripheral), None);
        assert_eq!(p.counts(), (4, 0));
        assert_eq!(p.on_link_down(Role::Peripheral), None);
        assert_eq!(
            p.on_link_down(Role::Peripheral),
            Some(PlatformHint::ResumeInbound)
        );
        assert!(p.accepting_inbound() && p.can_dial());
    }

    #[test]
    fn outbound_links_fill_the_device_cap_too() {
        let mut p = SlotPolicy::new(4);
        assert_eq!(p.inbound_cap(), 2);
        assert_eq!(p.on_link_up(Role::Peripheral), None);
        for _ in 0..2 {
            assert_eq!(p.on_link_up(Role::Central), None);
        }
        assert_eq!(
            p.on_link_up(Role::Central),
            Some(PlatformHint::StopAcceptingInbound)
        );
        assert!(!p.can_dial());
        assert_eq!(
            p.on_link_down(Role::Central),
            Some(PlatformHint::ResumeInbound)
        );
        assert_eq!(p.on_link_down(Role::Central), None);
        // Going below zero is ignored.
        let mut q = SlotPolicy::new(2);
        assert_eq!(q.on_link_down(Role::Peripheral), None);
        assert_eq!(q.counts(), (0, 0));
    }

    #[test]
    fn degenerate_caps() {
        let p = SlotPolicy::new(0);
        assert_eq!(p.initial_hint(), Some(PlatformHint::StopAcceptingInbound));
        assert!(!p.can_dial());
        let p = SlotPolicy::new(1);
        assert_eq!(p.inbound_cap(), 0);
        assert_eq!(p.initial_hint(), Some(PlatformHint::StopAcceptingInbound));
        let p = SlotPolicy::with_share(u32::MAX, f32::NAN);
        assert_eq!(p.inbound_cap(), u32::MAX);
        let p = SlotPolicy::with_share(5, 3.0);
        assert_eq!(p.outbound_reserved(), 5);
    }
}
