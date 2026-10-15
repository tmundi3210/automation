//! Per-inbound-link rate limits (BRIEF_v2 §5.5 "Rate limits per inbound
//! link, not per sender"; DECISIONS D4 PER INBOUND LINK and LANES; DESIGN §1
//! `relay/limits.rs`).
//!
//! One [`LinkBudget`] per connection (D4: "limits are keyed per connection,
//! because CBPeer.identifier stability is undocumented"). It holds one token
//! bucket per class group, an opaque-lane (L3) sub-bucket inside each with
//! the 25 % cap and the 10-minute new-link ramp, and a GossipSub-P4-style
//! link score that halves the budget when it drops.
//!
//! All arithmetic is integer: tokens are counted in [`UNITS_PER_TOKEN`]
//! sub-units (one per millisecond of a minute) so a bucket of `r` tokens per
//! minute refills exactly `r` units per millisecond and every phone reaches
//! the same decision from the same inputs. Time comes in as `now_mono`
//! (powered-on ms, [`crate::clock::Clock::mono_ms`]); a `now_mono` that goes
//! backwards counts as no time passed.

use crate::clock::ms;
use crate::relay::Lane;
use crate::wire::cell::CELL_LEN;
use crate::wire::frame::Class;

// ---- assumptions ------------------------------------------------------------

/// Control cells (C0 SOS and C1 control share one bucket) per minute per
/// link: DECISIONS D4 PER INBOUND LINK "control 20/min" (assumption).
pub const CONTROL_CELLS_PER_MIN_ASM: u64 = 20;
/// Text cells (C2) per minute per link: D4 "chat 60/min" (assumption).
pub const TEXT_CELLS_PER_MIN_ASM: u64 = 60;
/// News cells (C3, P2) per minute per link; D4 gives News its own 10/min
/// reservation only for the P1 L0 lane, so at P0 it shares the chat rate
/// (assumption).
pub const NEWS_CELLS_PER_MIN_ASM: u64 = 60;
/// Bulk (C4 preview) bytes per minute per BLE link: D4 "BLE bulk 256 KB/min"
/// read as 256 × 1024 bytes (assumption).
pub const BULK_BYTES_PER_MIN_ASM: u64 = 256 * 1024;
/// New-link ramp length: D4 "ramp up over 10 min" (assumption). The ramp
/// applies to L3 only ([`Lane::ramp_applies`]).
pub const RAMP_MS_ASM: u64 = 10 * ms::MINUTE;
/// Where the ramp starts, per mille of the lane's full share: D4 "new links
/// start at 25 % of quota" (assumption).
pub const RAMP_START_PERMILLE_ASM: u64 = 250;
/// Quota multiplier for L2 (QR-verified contact) links: D4 LANES "L2 … gets
/// 2x quota" (assumption). Implemented as each L2 cell costing `1/mult` of
/// a token.
pub const L2_QUOTA_MULT_ASM: u64 = 2;
/// Link score below which the link's budget halves (GossipSub
/// `publishThreshold` analogue; D4 "GossipSub-P4-style penalty", Q5)
/// (assumption).
pub const THROTTLE_SCORE_ASM: f32 = 0.5;
/// Link score below which nothing is admitted (GossipSub `graylistThreshold`
/// analogue) (assumption).
pub const REJECT_SCORE_ASM: f32 = 0.1;
/// Time for a link score to recover from 0 to 1 with no further penalties
/// (linear recovery; GossipSub decays its P4 counter over minutes)
/// (assumption).
pub const SCORE_RECOVERY_MS_ASM: u64 = 10 * ms::MINUTE;
/// Score removed by one bad inner signature (DECISIONS §3 row (1,2,3,4,10)
/// (iii)) (assumption).
pub const PENALTY_BAD_SIGNATURE_ASM: f32 = 0.25;
/// Score removed by one post from a key that was never a member (§3 row
/// (1,2,3,4,10) (iii)) (assumption).
pub const PENALTY_NEVER_MEMBER_ASM: f32 = 0.25;
/// Score removed by one post whose own heads include the author's removal
/// (§3 row (1,2,3,4,10) (iii)) (assumption).
pub const PENALTY_REMOVED_IN_HEADS_ASM: f32 = 0.25;
/// Score removed when a link keeps sending through a `Throttle` (buffer or
/// budget overflow; D4 "in the peripheral role the penalty is an ATT error
/// plus no more notifications") (assumption).
pub const PENALTY_OVERFLOW_ASM: f32 = 0.10;

/// Sub-units per token: one per millisecond of a minute, so a bucket of `r`
/// tokens per minute refills `r` units per millisecond.
pub const UNITS_PER_TOKEN: u64 = ms::MINUTE;

/// Score 1.0 in integer units. Chosen so that linear recovery over
/// [`SCORE_RECOVERY_MS_ASM`] is a whole number of units per millisecond.
const SCORE_ONE: u64 = SCORE_RECOVERY_MS_ASM * 1000;
const SCORE_RECOVERY_PER_MS: u64 = SCORE_ONE / SCORE_RECOVERY_MS_ASM;
const SCORE_ONE_PERMILLE: u64 = SCORE_ONE / 1000;
const _: () = assert!(SCORE_RECOVERY_PER_MS * SCORE_RECOVERY_MS_ASM == SCORE_ONE);
const _: () = assert!(RAMP_START_PERMILLE_ASM > 0 && RAMP_START_PERMILLE_ASM <= 1000);
const _: () = assert!(L2_QUOTA_MULT_ASM >= 1);
// One token must split into whole units for an L2 request (`token_cost_units`).
const _: () = assert!(UNITS_PER_TOKEN % L2_QUOTA_MULT_ASM == 0);

/// A share given as a compile-time `f32` fraction, in per mille.
const fn permille(fraction: f32) -> u64 {
    // Only ever applied to the `*_ASM` constants above; the `as` is exact
    // for the values they hold (checked by the asserts below).
    (fraction * 1000.0) as u64
}
const THROTTLE_SCORE_PERMILLE: u64 = permille(THROTTLE_SCORE_ASM);
const REJECT_SCORE_PERMILLE: u64 = permille(REJECT_SCORE_ASM);
const _: () = assert!(THROTTLE_SCORE_PERMILLE <= 1000 && REJECT_SCORE_PERMILLE <= 1000);
const _: () = assert!(REJECT_SCORE_PERMILLE <= THROTTLE_SCORE_PERMILLE);

// ---- public types -------------------------------------------------------------

/// Decision for one cell (or one object's worth of cells) on a link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Admit {
    /// Within budget; the tokens are consumed.
    Accept,
    /// Over budget now; nothing consumed. `retry_ms` is when the same request
    /// would fit if nothing else is admitted meanwhile (the value a
    /// `Throttle` frame carries, DESIGN §2.3).
    Throttle { retry_ms: u32 },
    /// Never admissible on this link: larger than the whole bucket, or the
    /// link's score is below [`REJECT_SCORE_ASM`].
    Reject,
}

/// Why a link is penalised (DECISIONS §3 row (1,2,3,4,10) (iii): the drop
/// plus penalty "applies only to bad signatures, keys that were never
/// members, or posts whose heads include the author's removal"; it keys on
/// the envelope sender's link, so a member re-sending another member's
/// ballot is never penalised). `Overflow` is the budget/buffer overrun
/// penalty from D4 LINK SLOTS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Penalty {
    BadSignature,
    NeverMember,
    RemovedInHeads,
    Overflow,
}

impl Penalty {
    /// Score removed, per mille of 1.0.
    const fn permille(self) -> u64 {
        match self {
            Penalty::BadSignature => permille(PENALTY_BAD_SIGNATURE_ASM),
            Penalty::NeverMember => permille(PENALTY_NEVER_MEMBER_ASM),
            Penalty::RemovedInHeads => permille(PENALTY_REMOVED_IN_HEADS_ASM),
            Penalty::Overflow => permille(PENALTY_OVERFLOW_ASM),
        }
    }
}

/// The four bucket groups (BRIEF_v2 §5.6 classes; C0 and C1 share the
/// control bucket, D4 "control 20/min").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Group {
    Control,
    Text,
    News,
    Bulk,
}

impl Group {
    const ALL: [Group; 4] = [Group::Control, Group::Text, Group::News, Group::Bulk];

    const fn of(class: Class) -> Group {
        match class {
            Class::C0Sos | Class::C1Control => Group::Control,
            Class::C2Text => Group::Text,
            Class::C3News => Group::News,
            Class::C4Preview => Group::Bulk,
        }
    }

    const fn index(self) -> usize {
        match self {
            Group::Control => 0,
            Group::Text => 1,
            Group::News => 2,
            Group::Bulk => 3,
        }
    }

    /// Tokens per minute: cells for the cell-counted groups, bytes for bulk.
    const fn rate_per_min(self) -> u64 {
        match self {
            Group::Control => CONTROL_CELLS_PER_MIN_ASM,
            Group::Text => TEXT_CELLS_PER_MIN_ASM,
            Group::News => NEWS_CELLS_PER_MIN_ASM,
            Group::Bulk => BULK_BYTES_PER_MIN_ASM,
        }
    }

    /// Burst = one minute's worth (D4 PER INBOUND LINK, assumption).
    const fn capacity_units(self) -> u64 {
        self.rate_per_min() * UNITS_PER_TOKEN
    }
}

/// One token bucket with its opaque-lane sub-bucket.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bucket {
    group: Group,
    /// Units available to any lane.
    tokens: u64,
    /// Units available to L3, drawn in addition to `tokens`. Capped at the
    /// ramped L3 share of the bucket.
    l3_tokens: u64,
    last_refill: u64,
}

impl Bucket {
    fn new(group: Group, now_mono: u64, l3_cap_now: u64) -> Self {
        Bucket {
            group,
            tokens: group.capacity_units(),
            l3_tokens: l3_cap_now,
            last_refill: now_mono,
        }
    }

    /// Credits the elapsed time. The L3 sub-bucket refills at the ramped
    /// share of the rate and is capped at the ramped share of the capacity,
    /// both evaluated at `now` (the ramp only grows, so this is the share
    /// in force when the tokens could first be used).
    fn refill(&mut self, now_mono: u64, l3: &L3Share) {
        let elapsed = now_mono.saturating_sub(self.last_refill);
        self.last_refill = self.last_refill.max(now_mono);
        if elapsed == 0 {
            // Still clamp: the ramp may have grown since the last call.
            self.l3_tokens = self.l3_tokens.min(l3.cap_units(self.group));
            return;
        }
        let cap = self.group.capacity_units();
        let rate = self.group.rate_per_min();
        self.tokens = self
            .tokens
            .saturating_add(elapsed.saturating_mul(rate))
            .min(cap);
        let l3_cap = l3.cap_units(self.group);
        self.l3_tokens = self
            .l3_tokens
            .saturating_add(l3.refill_units(self.group, elapsed))
            .min(l3_cap);
    }
}

/// The L3 share in force at one instant: `share × ramp`, both per mille.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct L3Share {
    /// Numerator and denominator of the effective fraction of the bucket.
    num: u128,
    den: u128,
}

impl L3Share {
    fn at(created_mono: u64, now_mono: u64, is_station: bool) -> Self {
        // Share of the bucket L3 may use at full ramp (BRIEF_v2 §5.5: 25 %,
        // or the whole budget on a station that can validate nothing).
        let share_permille: u128 = if is_station {
            1000
        } else {
            u128::from(Lane::L3_SHARE_PERMILLE_ASM)
        };
        // Linear ramp from RAMP_START to 100 % of that share over RAMP_MS.
        let age = u128::from(now_mono.saturating_sub(created_mono).min(RAMP_MS_ASM));
        let ramp_ms = u128::from(RAMP_MS_ASM);
        let start = u128::from(RAMP_START_PERMILLE_ASM);
        // ramp = (start·RAMP + (1000−start)·age) / (1000·RAMP)
        let ramp_num = start * ramp_ms + (1000 - start) * age;
        let ramp_den = 1000 * ramp_ms;
        L3Share {
            num: share_permille * ramp_num,
            den: 1000 * ramp_den,
        }
    }

    /// Share at the end of the ramp.
    fn full(is_station: bool) -> Self {
        L3Share::at(0, RAMP_MS_ASM, is_station)
    }

    fn apply(&self, units: u128) -> u64 {
        u64::try_from(units * self.num / self.den).unwrap_or(u64::MAX)
    }

    fn cap_units(&self, group: Group) -> u64 {
        self.apply(u128::from(group.capacity_units()))
    }

    fn refill_units(&self, group: Group, elapsed_ms: u64) -> u64 {
        self.apply(u128::from(elapsed_ms) * u128::from(group.rate_per_min()))
    }

    /// Milliseconds until `deficit` units accrue at this share of the rate.
    fn wait_ms(&self, group: Group, deficit: u64) -> u64 {
        let rate_num = u128::from(group.rate_per_min()) * self.num;
        if rate_num == 0 {
            return u64::MAX;
        }
        let needed = u128::from(deficit) * self.den;
        u64::try_from(needed.div_ceil(rate_num)).unwrap_or(u64::MAX)
    }
}

/// Per-link budget: token buckets, L3 cap and ramp, link score.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkBudget {
    created_mono: u64,
    is_station: bool,
    buckets: [Bucket; 4],
    /// Link score in [0, SCORE_ONE] as of `score_mono`.
    score_units: u64,
    score_mono: u64,
}

impl LinkBudget {
    /// A fresh link at `now_mono`. `is_station`: this relay validates nothing
    /// (DECISIONS §3 row (4,8,9,10): "On relays that can validate nothing
    /// (stations), L3's cap is the whole link budget").
    pub fn new(now_mono: u64, is_station: bool) -> Self {
        let l3 = L3Share::at(now_mono, now_mono, is_station);
        LinkBudget {
            created_mono: now_mono,
            is_station,
            buckets: Group::ALL.map(|g| Bucket::new(g, now_mono, l3.cap_units(g))),
            score_units: SCORE_ONE,
            score_mono: now_mono,
        }
    }

    pub fn is_station(&self) -> bool {
        self.is_station
    }

    /// Powered-on instant the link was created (start of the ramp).
    pub fn created_mono(&self) -> u64 {
        self.created_mono
    }

    /// `true` while the new-link ramp is still rising (L3 only).
    pub fn in_ramp(&self, now_mono: u64) -> bool {
        now_mono.saturating_sub(self.created_mono) < RAMP_MS_ASM
    }

    /// Asks to admit `bytes` of `class` traffic on `lane` at `now_mono`.
    /// Cell-counted classes (C0–C3) charge one token per cell the bytes fill
    /// (at least one); C4 charges one token per byte. `Accept` consumes the
    /// tokens; the other outcomes consume nothing.
    pub fn admit(&mut self, class: Class, lane: Lane, bytes: usize, now_mono: u64) -> Admit {
        let group = Group::of(class);
        let l3_now = L3Share::at(self.created_mono, now_mono, self.is_station);
        let score = self.score_units_at(now_mono);
        self.score_units = score;
        self.score_mono = self.score_mono.max(now_mono);
        let bucket = &mut self.buckets[group.index()];
        bucket.refill(now_mono, &l3_now);

        if score < REJECT_SCORE_PERMILLE * SCORE_ONE_PERMILLE {
            return Admit::Reject;
        }

        let Some(cost) = Self::cost_units(group, lane, bytes, score) else {
            return Admit::Reject;
        };
        if cost == 0 {
            return Admit::Accept;
        }
        let cap = group.capacity_units();
        if cost > cap {
            return Admit::Reject;
        }
        if lane.ramp_applies() {
            // L3 may never use more than its share, even at full ramp.
            let full = L3Share::full(self.is_station);
            if cost > full.cap_units(group) {
                return Admit::Reject;
            }
        }

        let mut wait: u64 = 0;
        if bucket.tokens < cost {
            wait = wait.max(L3Share { num: 1, den: 1 }.wait_ms(group, cost - bucket.tokens));
        }
        if lane.ramp_applies() {
            if bucket.l3_tokens < cost {
                wait = wait.max(l3_now.wait_ms(group, cost - bucket.l3_tokens));
            }
            // Even with tokens accrued, the ramped cap must have grown to
            // hold `cost`.
            wait = wait.max(self.wait_for_ramp_cap(group, cost, now_mono));
        }
        if wait > 0 {
            return Admit::Throttle {
                retry_ms: u32::try_from(wait.max(1)).unwrap_or(u32::MAX),
            };
        }

        let bucket = &mut self.buckets[group.index()];
        bucket.tokens -= cost;
        if lane.ramp_applies() {
            bucket.l3_tokens -= cost;
        }
        Admit::Accept
    }

    /// Lowers the link score (DECISIONS D4 LANES "drop plus a
    /// GossipSub-P4-style penalty on the link"). Only the caller decides
    /// when: validated member traffic re-sent by a relay is never penalised
    /// (§3 row (1,2,3,4,10) (iii)).
    pub fn penalise(&mut self, reason: Penalty, now_mono: u64) {
        let current = self.score_units_at(now_mono);
        self.score_units = current.saturating_sub(reason.permille() * SCORE_ONE_PERMILLE);
        self.score_mono = self.score_mono.max(now_mono);
    }

    /// Link score in [0, 1] as of the last `admit`/`penalise` call.
    pub fn score(&self) -> f32 {
        Self::score_to_f32(self.score_units)
    }

    /// Link score in [0, 1] at `now_mono`, after recovery.
    pub fn score_at(&self, now_mono: u64) -> f32 {
        Self::score_to_f32(self.score_units_at(now_mono))
    }

    /// `true` while the score halves the budget (below [`THROTTLE_SCORE_ASM`]).
    pub fn is_throttled_by_score(&self, now_mono: u64) -> bool {
        self.score_units_at(now_mono) < THROTTLE_SCORE_PERMILLE * SCORE_ONE_PERMILLE
    }

    /// Tokens (cells, or bytes for C4) the given lane could draw right now
    /// without a throttle, for tests and the status strip. Exactly what a
    /// run of one-token `admit` calls at `now_mono` would accept: it counts
    /// the L2 quota multiplier, the halved budget of a throttled score and
    /// the graylist (score below [`REJECT_SCORE_ASM`] draws nothing).
    pub fn available(&self, class: Class, lane: Lane, now_mono: u64) -> u64 {
        let group = Group::of(class);
        let score = self.score_units_at(now_mono);
        if score < REJECT_SCORE_PERMILLE * SCORE_ONE_PERMILLE {
            return 0;
        }
        let l3 = L3Share::at(self.created_mono, now_mono, self.is_station);
        let mut bucket = self.buckets[group.index()];
        bucket.refill(now_mono, &l3);
        let units = if lane.ramp_applies() {
            bucket.tokens.min(bucket.l3_tokens)
        } else {
            bucket.tokens
        };
        units / Self::token_cost_units(lane, score)
    }

    fn score_units_at(&self, now_mono: u64) -> u64 {
        let elapsed = now_mono.saturating_sub(self.score_mono);
        self.score_units
            .saturating_add(elapsed.saturating_mul(SCORE_RECOVERY_PER_MS))
            .min(SCORE_ONE)
    }

    fn score_to_f32(units: u64) -> f32 {
        // Both values are small (≤ 6e8), so the conversion is exact enough
        // for a display/threshold value.
        (units.min(SCORE_ONE) as f64 / SCORE_ONE as f64) as f32
    }

    /// Units one token (a cell, or a byte of C4) costs on `lane` at a given
    /// score: the one cost model behind `admit` and `available`. Never 0.
    fn token_cost_units(lane: Lane, score_units: u64) -> u64 {
        let mut cost = UNITS_PER_TOKEN;
        // Below THROTTLE_SCORE the budget halves: every request costs double.
        if score_units < THROTTLE_SCORE_PERMILLE * SCORE_ONE_PERMILLE {
            cost *= 2;
        }
        // L2 gets L2_QUOTA_MULT× quota: each request costs 1/mult.
        if lane == Lane::L2 {
            cost /= L2_QUOTA_MULT_ASM;
        }
        cost
    }

    /// Units charged for `bytes` of a group on a lane at a given score, or
    /// `None` if the request cannot be represented (absurd `bytes`).
    fn cost_units(group: Group, lane: Lane, bytes: usize, score_units: u64) -> Option<u64> {
        let bytes = u64::try_from(bytes).ok()?;
        let tokens = match group {
            Group::Bulk => bytes,
            Group::Control | Group::Text | Group::News => bytes.div_ceil(CELL_LEN as u64).max(1),
        };
        tokens.checked_mul(Self::token_cost_units(lane, score_units))
    }

    /// Milliseconds until the ramped L3 cap of `group` reaches `cost`
    /// (0 if it already does).
    fn wait_for_ramp_cap(&self, group: Group, cost: u64, now_mono: u64) -> u64 {
        let now_share = L3Share::at(self.created_mono, now_mono, self.is_station);
        if now_share.cap_units(group) >= cost {
            return 0;
        }
        // cap(age) = C·share·(start·R + (1000−start)·age) / (1000·1000·R).
        // Solve for the smallest age with cap(age) ≥ cost, then subtract
        // the current age. Falls back to the end of the ramp on any
        // degenerate input.
        let c = u128::from(group.capacity_units());
        let share: u128 = if self.is_station {
            1000
        } else {
            u128::from(Lane::L3_SHARE_PERMILLE_ASM)
        };
        let r = u128::from(RAMP_MS_ASM);
        let start = u128::from(RAMP_START_PERMILLE_ASM);
        let slope = 1000 - start;
        let lhs = u128::from(cost) * 1000 * 1000 * r;
        let base = c * share * start * r;
        let denom = c * share * slope;
        let age_needed = if denom == 0 {
            r
        } else {
            lhs.saturating_sub(base).div_ceil(denom).min(r)
        };
        let age_now = u128::from(now_mono.saturating_sub(self.created_mono));
        u64::try_from(age_needed.saturating_sub(age_now)).unwrap_or(u64::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const T0: u64 = 1_000_000;

    fn admit_n(b: &mut LinkBudget, class: Class, lane: Lane, n: usize, now: u64) -> usize {
        (0..n)
            .filter(|_| b.admit(class, lane, CELL_LEN, now) == Admit::Accept)
            .count()
    }

    #[test]
    fn control_bucket_is_exactly_twenty_cells_then_throttles_with_refill_time() {
        let mut b = LinkBudget::new(T0, false);
        assert_eq!(admit_n(&mut b, Class::C1Control, Lane::L1, 20, T0), 20);
        assert_eq!(
            b.admit(Class::C1Control, Lane::L1, CELL_LEN, T0),
            Admit::Throttle { retry_ms: 3_000 }
        );
        // Exactly one cell's worth has accrued after 3 s.
        assert_eq!(
            b.admit(Class::C1Control, Lane::L1, CELL_LEN, T0 + 3_000),
            Admit::Accept
        );
        assert!(matches!(
            b.admit(Class::C1Control, Lane::L1, CELL_LEN, T0 + 3_000),
            Admit::Throttle { .. }
        ));
        // A full minute later the burst is back.
        assert_eq!(
            admit_n(&mut b, Class::C0Sos, Lane::L1, 20, T0 + 3_000 + ms::MINUTE),
            20
        );
    }

    #[test]
    fn bulk_bucket_counts_bytes() {
        let mut b = LinkBudget::new(T0, false);
        let cap = usize::try_from(BULK_BYTES_PER_MIN_ASM).unwrap_or(usize::MAX);
        assert_eq!(b.admit(Class::C4Preview, Lane::L1, cap, T0), Admit::Accept);
        assert_eq!(b.admit(Class::C4Preview, Lane::L1, 0, T0), Admit::Accept);
        assert_eq!(
            b.admit(Class::C4Preview, Lane::L1, 1, T0),
            Admit::Throttle { retry_ms: 1 }
        );
        // Larger than the whole bucket: never.
        assert_eq!(
            b.admit(Class::C4Preview, Lane::L1, cap + 1, T0 + ms::MINUTE),
            Admit::Reject
        );
    }

    #[test]
    fn multi_cell_objects_charge_per_cell() {
        let mut b = LinkBudget::new(T0, false);
        assert_eq!(
            b.admit(Class::C1Control, Lane::L1, 19 * CELL_LEN + 1, T0),
            Admit::Accept
        );
        assert_eq!(b.available(Class::C1Control, Lane::L1, T0), 0);
    }

    #[test]
    fn l3_share_and_ramp() {
        let t_full = T0 + RAMP_MS_ASM;
        let mut b = LinkBudget::new(T0, false);
        // Age 0: 25 % share × 25 % ramp = 1.25 cells.
        assert_eq!(admit_n(&mut b, Class::C1Control, Lane::L3, 5, T0), 1);
        // L1 on the same link is untouched by the ramp.
        let mut b = LinkBudget::new(T0, false);
        assert_eq!(admit_n(&mut b, Class::C1Control, Lane::L1, 25, T0), 20);
        // Half way: ramp = 0.625 → 3.125 cells.
        let mut b = LinkBudget::new(T0, false);
        assert_eq!(
            admit_n(&mut b, Class::C1Control, Lane::L3, 10, T0 + RAMP_MS_ASM / 2),
            3
        );
        // Full ramp: 5 cells, and the rest of the bucket is still open to L1.
        let mut b = LinkBudget::new(T0, false);
        assert_eq!(admit_n(&mut b, Class::C1Control, Lane::L3, 10, t_full), 5);
        assert_eq!(admit_n(&mut b, Class::C1Control, Lane::L1, 20, t_full), 15);
        // Station: the whole bucket, still ramped.
        let mut s = LinkBudget::new(T0, true);
        assert_eq!(admit_n(&mut s, Class::C1Control, Lane::L3, 25, T0), 5);
        let mut s = LinkBudget::new(T0, true);
        assert_eq!(admit_n(&mut s, Class::C1Control, Lane::L3, 25, t_full), 20);
    }

    #[test]
    fn l3_throttle_waits_for_the_ramp_cap() {
        let mut b = LinkBudget::new(T0, false);
        let cap = usize::try_from(BULK_BYTES_PER_MIN_ASM).unwrap_or(usize::MAX);
        // At age 0 L3 bulk is 1/16 of the bucket = 16 kB.
        assert_eq!(
            b.admit(Class::C4Preview, Lane::L3, cap / 16, T0),
            Admit::Accept
        );
        let mut b = LinkBudget::new(T0, false);
        let Admit::Throttle { retry_ms } = b.admit(Class::C4Preview, Lane::L3, cap / 8, T0) else {
            panic!("expected a throttle");
        };
        // 1/8 of the bucket needs ramp ≥ 0.5, i.e. a third of the ramp.
        assert_eq!(u64::from(retry_ms), RAMP_MS_ASM / 3);
        assert_eq!(
            b.admit(
                Class::C4Preview,
                Lane::L3,
                cap / 8,
                T0 + u64::from(retry_ms)
            ),
            Admit::Accept
        );
        // More than the full L3 share is never admissible on L3.
        assert_eq!(
            b.admit(Class::C4Preview, Lane::L3, cap / 4 + 1, T0 + RAMP_MS_ASM),
            Admit::Reject
        );
    }

    #[test]
    fn l2_gets_double_quota() {
        let mut b = LinkBudget::new(T0, false);
        assert_eq!(admit_n(&mut b, Class::C2Text, Lane::L2, 200, T0), 120);
    }

    #[test]
    fn penalties_halve_then_reject_and_recover() {
        let mut b = LinkBudget::new(T0, false);
        b.penalise(Penalty::BadSignature, T0);
        b.penalise(Penalty::NeverMember, T0);
        assert_eq!(b.score(), 0.5);
        assert!(!b.is_throttled_by_score(T0));
        b.penalise(Penalty::RemovedInHeads, T0);
        assert_eq!(b.score(), 0.25);
        assert!(b.is_throttled_by_score(T0));
        assert_eq!(admit_n(&mut b, Class::C1Control, Lane::L1, 20, T0), 10);
        b.penalise(Penalty::Overflow, T0);
        b.penalise(Penalty::Overflow, T0);
        assert_eq!(b.score(), 0.05);
        assert_eq!(
            b.admit(Class::C2Text, Lane::L1, CELL_LEN, T0),
            Admit::Reject
        );
        // Full recovery after SCORE_RECOVERY_MS: 1.0, no throttle, full burst.
        let later = T0 + SCORE_RECOVERY_MS_ASM;
        assert_eq!(b.score_at(later), 1.0);
        assert_eq!(admit_n(&mut b, Class::C1Control, Lane::L1, 25, later), 20);
        assert_eq!(b.score(), 1.0);
    }

    /// `available` must say exactly what a run of one-token admits grants:
    /// L2's double quota, the halved budget of a throttled score and the
    /// graylist (reviewer regression: it used to ignore all three).
    #[test]
    fn available_matches_what_admit_grants() {
        let t = T0 + RAMP_MS_ASM;
        for penalties in [0usize, 3, 4] {
            for lane in Lane::ALL {
                for class in Class::ALL {
                    let mut b = LinkBudget::new(T0, false);
                    for _ in 0..penalties {
                        b.penalise(Penalty::BadSignature, T0);
                    }
                    let said = b.available(class, lane, t);
                    let unit = if class == Class::C4Preview {
                        1
                    } else {
                        CELL_LEN
                    };
                    let got = (0..600_000)
                        .take_while(|_| b.admit(class, lane, unit, t) == Admit::Accept)
                        .count();
                    assert_eq!(
                        u64::try_from(got).unwrap_or(u64::MAX),
                        said,
                        "{class:?} {lane:?} after {penalties} penalties"
                    );
                }
            }
        }
        let mut b = LinkBudget::new(T0, false);
        assert_eq!(b.available(Class::C2Text, Lane::L2, T0), 120);
        b.penalise(Penalty::BadSignature, T0);
        b.penalise(Penalty::BadSignature, T0);
        b.penalise(Penalty::BadSignature, T0);
        assert_eq!(b.available(Class::C1Control, Lane::L1, T0), 10);
        assert_eq!(b.available(Class::C1Control, Lane::L2, T0), 20);
        b.penalise(Penalty::BadSignature, T0);
        assert_eq!(b.available(Class::C1Control, Lane::L1, T0), 0);
    }

    #[test]
    fn time_never_goes_backwards() {
        let mut b = LinkBudget::new(T0, false);
        assert_eq!(admit_n(&mut b, Class::C2Text, Lane::L1, 60, T0), 60);
        assert!(matches!(
            b.admit(Class::C2Text, Lane::L1, CELL_LEN, 0),
            Admit::Throttle { .. }
        ));
        assert_eq!(b.available(Class::C2Text, Lane::L1, 0), 0);
        assert_eq!(b.available(Class::C2Text, Lane::L1, T0 + 1_000), 1);
    }

    #[test]
    fn absurd_sizes_are_rejected_not_panicked() {
        let mut b = LinkBudget::new(T0, false);
        assert_eq!(
            b.admit(Class::C4Preview, Lane::L1, usize::MAX, u64::MAX),
            Admit::Reject
        );
        assert_eq!(
            b.admit(Class::C2Text, Lane::L3, usize::MAX, u64::MAX),
            Admit::Reject
        );
        assert_eq!(
            b.admit(Class::C2Text, Lane::L1, CELL_LEN, u64::MAX),
            Admit::Accept
        );
    }
}
