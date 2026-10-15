//! Injected time. Protocol code never reads the OS clock (BRIEF_v2 §4: "no
//! timestamps decide anything"; D6 rule 3). Everything that expires or rotates
//! is measured in *powered-on* milliseconds, which a platform persists across
//! restarts. The wall clock exists only for display strings.

use std::cell::Cell;

/// Time source handed to a node. Platforms implement it from the OS; the
/// simulator implements it from its event loop.
pub trait Clock {
    /// Milliseconds of powered-on time. Monotonic within a process; the
    /// platform persists it so the counter continues across restarts.
    fn mono_ms(&self) -> u64;
    /// Unix milliseconds for display only. User-settable, may jump, never
    /// used for any protocol decision.
    fn wall_for_display(&self) -> i64;
}

/// A clock that only moves when told to. Used by tests and the simulator.
#[derive(Debug, Default)]
pub struct ManualClock {
    mono: Cell<u64>,
    wall_offset_ms: Cell<i64>,
}

impl ManualClock {
    pub fn new(mono_ms: u64) -> Self {
        Self {
            mono: Cell::new(mono_ms),
            wall_offset_ms: Cell::new(0),
        }
    }
    /// Advance powered-on time by `ms`.
    pub fn advance(&self, ms: u64) {
        self.mono.set(self.mono.get().saturating_add(ms));
    }
    pub fn set(&self, mono_ms: u64) {
        self.mono.set(mono_ms);
    }
    /// Offset between this node's wall clock and powered-on time, so that
    /// simulated phones can disagree about the time of day (scenario S4).
    pub fn set_wall_offset_ms(&self, offset: i64) {
        self.wall_offset_ms.set(offset);
    }
}

impl Clock for ManualClock {
    fn mono_ms(&self) -> u64 {
        self.mono.get()
    }
    fn wall_for_display(&self) -> i64 {
        self.mono.get() as i64 + self.wall_offset_ms.get()
    }
}

/// Accumulates powered-on time across restarts. A platform stores `total_ms()`
/// periodically and rebuilds the meter with it at the next start.
#[derive(Debug, Clone, Copy)]
pub struct PoweredOnMeter {
    persisted_base_ms: u64,
    session_start_mono_ms: u64,
}

impl PoweredOnMeter {
    /// `persisted_base_ms`: powered-on total saved by the previous session.
    /// `session_start_mono_ms`: the platform's monotonic reading at this start.
    pub fn resume(persisted_base_ms: u64, session_start_mono_ms: u64) -> Self {
        Self {
            persisted_base_ms,
            session_start_mono_ms,
        }
    }
    /// Powered-on total given the platform's current monotonic reading.
    pub fn total_ms(&self, now_mono_ms: u64) -> u64 {
        self.persisted_base_ms
            .saturating_add(now_mono_ms.saturating_sub(self.session_start_mono_ms))
    }
}

/// Convenience durations in milliseconds, so rules read like the brief.
pub mod ms {
    pub const SECOND: u64 = 1_000;
    pub const MINUTE: u64 = 60 * SECOND;
    pub const HOUR: u64 = 60 * MINUTE;
    pub const DAY: u64 = 24 * HOUR;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manual_clock_advances_and_offsets_display_only() {
        let c = ManualClock::new(10);
        c.set_wall_offset_ms(-(ms::MINUTE as i64) * 20);
        c.advance(5);
        assert_eq!(c.mono_ms(), 15);
        assert_eq!(c.wall_for_display(), 15 - 20 * ms::MINUTE as i64);
    }

    #[test]
    fn powered_on_meter_survives_restart() {
        let m = PoweredOnMeter::resume(1_000, 500);
        assert_eq!(m.total_ms(500), 1_000);
        assert_eq!(m.total_ms(1_700), 2_200);
        // a platform monotonic clock that restarted below the session start is clamped
        assert_eq!(m.total_ms(100), 1_000);
    }
}
