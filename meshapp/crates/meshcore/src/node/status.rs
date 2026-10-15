//! Delivery states and the status strip (docs/DESIGN.md §1 row
//! `node/status.rs`; BRIEF_v2 §2 "Global status strip", §5.9 "Delivery
//! states in the UI", §7 vocabulary "Waiting / Passed along / Delivered",
//! "Offline – will send later"; DECISIONS D4 UI, D6 rule 7).
//!
//! Rules (BRIEF_v2 §5.9):
//! * **Waiting** until a link other than the one the object came from
//!   accepted its cells ("Waiting for someone nearby" is never shown while
//!   neighbours are connected: the strip says "N nearby" then);
//! * **Waiting · network busy** when a relay limit has held the object's
//!   cells for more than [`NETWORK_BUSY_MS_ASM`];
//! * **Passed along** once every cell of the object went out on some link;
//! * **Delivered** when a receipt acknowledges the object (D6 rule 7:
//!   "Delivered means decrypted and committed" at the recipient);
//! * **May have expired · Resend** after [`MAX_AGE_ASM`] of powered-on time
//!   without Delivered (the receiver's key cap, D6 rule 7); "Resend" is
//!   [`Node::resend`](super::Node::resend);
//! * **Needs their key** for a 1:1 message with no usable session.
//!
//! Hollow (can still change) versus solid (won't change) follows BRIEF_v2
//! §8: only Delivered is solid here. Nothing reads a clock: ages are
//! powered-on milliseconds the node passes in.

use crate::clock::ms;

/// Battery floor below which a phone stops forwarding others' cells
/// (BRIEF_v2 §5.7 "about 30%, assumption"), in parts per million.
pub const BATTERY_FLOOR_PPM_ASM: u32 = 300_000;
/// How long a relay limit may hold an object before the sender sees
/// "Waiting · network busy" (DECISIONS D4 UI; assumption).
pub const NETWORK_BUSY_MS_ASM: u64 = 30 * ms::SECOND;
/// Powered-on age after which an unacknowledged object shows "May have
/// expired · Resend": the receiver-side key retention (D6 rule 7;
/// `dtn::dedupe::KEY_RETENTION_ASM_MS`; assumption).
pub const MAX_AGE_ASM: u64 = crate::dtn::dedupe::KEY_RETENTION_ASM_MS;

/// Delivery state of an object this phone sent (BRIEF_v2 §5.9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DeliveryState {
    Waiting,
    WaitingNetworkBusy,
    PassedAlong,
    Delivered,
    MayHaveExpired,
    NeedsKey,
}

impl DeliveryState {
    /// The §7 vocabulary string (icon + word in the UI).
    pub const fn text(self) -> &'static str {
        match self {
            DeliveryState::Waiting => "Waiting for someone nearby",
            DeliveryState::WaitingNetworkBusy => "Waiting · network busy",
            DeliveryState::PassedAlong => "Passed along",
            DeliveryState::Delivered => "Delivered",
            DeliveryState::MayHaveExpired => "May have expired · Resend",
            DeliveryState::NeedsKey => "Needs their key",
        }
    }

    /// Solid states never revert (BRIEF_v2 §8): only Delivered here.
    pub const fn is_solid(self) -> bool {
        matches!(self, DeliveryState::Delivered)
    }

    /// Whether "Resend" is offered.
    pub const fn can_resend(self) -> bool {
        matches!(self, DeliveryState::MayHaveExpired)
    }
}

/// The next state of an undelivered object given what the node observed
/// (module docs). `held` means some link's scheduler is holding queued
/// cells behind a relay limit. Delivered and NeedsKey are set by the
/// receipt and the session paths, never here.
pub fn age_transition(
    current: DeliveryState,
    age_ms: u64,
    passed_along: bool,
    held: bool,
) -> DeliveryState {
    match current {
        DeliveryState::Delivered | DeliveryState::NeedsKey | DeliveryState::MayHaveExpired => {
            current
        }
        _ if age_ms >= MAX_AGE_ASM => DeliveryState::MayHaveExpired,
        _ if passed_along => DeliveryState::PassedAlong,
        _ if held && age_ms >= NETWORK_BUSY_MS_ASM => DeliveryState::WaitingNetworkBusy,
        _ => DeliveryState::Waiting,
    }
}

/// The global status strip (BRIEF_v2 §2): "6 nearby · last sync 2 min ago"
/// or "Offline – will send later", with the relay and battery hints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusStrip {
    /// Links whose handshake finished.
    pub nearby: usize,
    /// This phone forwards others' cells (not under the battery floor, not
    /// a station without links).
    pub passing_along: bool,
    /// Wall-clock millisecond of the last digest match, display only; `None`
    /// without a display clock or before any sync.
    pub last_sync_display: Option<i64>,
    /// Powered-on milliseconds since the last digest match.
    pub last_sync_age_ms: Option<u64>,
    /// Below the battery floor (BRIEF_v2 §5.7).
    pub battery_floor: bool,
    /// A relay limit is holding traffic ("Waiting · network busy" hint,
    /// DECISIONS D4 UI).
    pub network_busy: bool,
}

impl StatusStrip {
    /// The strip text (BRIEF_v2 §2, §7 "Offline – will send later").
    pub fn text(&self) -> String {
        if self.nearby == 0 {
            return "Offline – will send later".to_owned();
        }
        let mut s = format!("{} nearby", self.nearby);
        if let Some(age) = self.last_sync_age_ms {
            s.push_str(" · last sync ");
            s.push_str(&age_text(age));
        }
        if self.network_busy {
            s.push_str(" · network busy");
        }
        s
    }
}

/// "just now", "2 min ago", "3 h ago": a powered-on age for the strip.
pub fn age_text(age_ms: u64) -> String {
    if age_ms < ms::MINUTE {
        "just now".to_owned()
    } else if age_ms < ms::HOUR {
        format!("{} min ago", age_ms / ms::MINUTE)
    } else {
        format!("{} h ago", age_ms / ms::HOUR)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transitions_follow_the_brief() {
        assert_eq!(
            age_transition(DeliveryState::Waiting, 0, false, false),
            DeliveryState::Waiting
        );
        assert_eq!(
            age_transition(DeliveryState::Waiting, NETWORK_BUSY_MS_ASM, false, true),
            DeliveryState::WaitingNetworkBusy
        );
        assert_eq!(
            age_transition(DeliveryState::WaitingNetworkBusy, 1, true, true),
            DeliveryState::PassedAlong
        );
        assert_eq!(
            age_transition(DeliveryState::PassedAlong, MAX_AGE_ASM, true, false),
            DeliveryState::MayHaveExpired
        );
        assert_eq!(
            age_transition(DeliveryState::Delivered, MAX_AGE_ASM * 2, true, true),
            DeliveryState::Delivered,
            "solid never reverts"
        );
        assert!(DeliveryState::Delivered.is_solid());
        assert!(DeliveryState::MayHaveExpired.can_resend());
        assert_eq!(DeliveryState::PassedAlong.text(), "Passed along");
    }

    #[test]
    fn strip_text_uses_the_vocabulary() {
        let offline = StatusStrip {
            nearby: 0,
            passing_along: false,
            last_sync_display: None,
            last_sync_age_ms: None,
            battery_floor: false,
            network_busy: false,
        };
        assert_eq!(offline.text(), "Offline – will send later");
        let on = StatusStrip {
            nearby: 6,
            last_sync_age_ms: Some(2 * ms::MINUTE),
            ..offline
        };
        assert_eq!(on.text(), "6 nearby · last sync 2 min ago");
        assert_eq!(age_text(5 * ms::HOUR), "5 h ago");
    }
}
