//! Runtime-neutral time values and split clock ports.
//!
//! Foundation code owns values and dependency-inversion seams only. Concrete
//! operating-system clocks belong in adapter crates.

use core::fmt;

use identus_derive as identus;
use identus_derive::Newtype;

use crate::{CapabilityId, ErrorCode, ErrorKind, IdentusError};

const CAPABILITY: CapabilityId = CapabilityId::new("core");
const CLOCK_UNAVAILABLE_CODE: ErrorCode = ErrorCode::new("core.clock_unavailable");
const TIME_OVERFLOW_CODE: ErrorCode = ErrorCode::new("core.time_overflow");

/// Milliseconds since the Unix epoch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Newtype)]
#[newtype(display, serde)]
pub struct UnixTimestampMillis(u64);

impl UnixTimestampMillis {
    /// Return whole Unix seconds, truncating a partial second.
    #[must_use]
    pub const fn whole_seconds(self) -> u64 {
        self.0 / 1_000
    }

    /// Add a duration without wrapping.
    pub fn checked_add(self, duration: DurationMillis) -> Result<Self, ClockError> {
        self.0
            .checked_add(duration.0)
            .map(Self)
            .ok_or(ClockError::Overflow)
    }
}

/// Milliseconds from an opaque stable origin within one process/boot epoch.
///
/// This value intentionally has no serde implementation: it is not a civil
/// timestamp and must not be persisted across clock epochs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Newtype)]
#[newtype(display)]
pub struct MonotonicTimestampMillis(u64);

impl MonotonicTimestampMillis {
    /// Add a duration without wrapping.
    pub fn checked_add(self, duration: DurationMillis) -> Result<Self, ClockError> {
        self.0
            .checked_add(duration.0)
            .map(Self)
            .ok_or(ClockError::Overflow)
    }

    /// Return elapsed time if `earlier` belongs to the same non-regressed
    /// clock epoch.
    #[must_use]
    pub fn elapsed_since(self, earlier: Self) -> Option<DurationMillis> {
        self.0.checked_sub(earlier.0).map(DurationMillis)
    }
}

/// An unsigned millisecond duration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Newtype)]
#[newtype(display, serde)]
pub struct DurationMillis(u64);

/// A non-sensitive clock or time-arithmetic failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClockError {
    /// The injected clock cannot provide a trustworthy reading.
    Unavailable,
    /// Checked time arithmetic exceeded the supported range.
    Overflow,
}

impl ClockError {
    /// Bridge to the shared redaction-safe SDK error surface.
    pub const fn to_identus_error(self) -> IdentusError {
        match self {
            Self::Unavailable => IdentusError::public(
                CLOCK_UNAVAILABLE_CODE,
                ErrorKind::Internal,
                CAPABILITY,
                "clock is unavailable",
            ),
            Self::Overflow => IdentusError::public(
                TIME_OVERFLOW_CODE,
                ErrorKind::Internal,
                CAPABILITY,
                "time arithmetic overflowed",
            ),
        }
    }
}

impl fmt::Display for ClockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Unavailable => "clock is unavailable",
            Self::Overflow => "time arithmetic overflowed",
        })
    }
}

impl std::error::Error for ClockError {}

/// Supplies civil Unix time without selecting an operating-system adapter.
#[identus::port]
pub trait WallClock: Send + Sync {
    /// Read current Unix epoch milliseconds.
    fn now(&self) -> Result<UnixTimestampMillis, ClockError>;
}

/// Supplies elapsed-time ticks from an opaque stable origin.
#[identus::port]
pub trait MonotonicClock: Send + Sync {
    /// Read the current process/boot-epoch monotonic tick.
    fn now(&self) -> Result<MonotonicTimestampMillis, ClockError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FixedWallClock;

    impl WallClock for FixedWallClock {
        fn now(&self) -> Result<UnixTimestampMillis, ClockError> {
            Ok(UnixTimestampMillis::new(1_234_567))
        }
    }

    struct FixedMonotonicClock;

    impl MonotonicClock for FixedMonotonicClock {
        fn now(&self) -> Result<MonotonicTimestampMillis, ClockError> {
            Ok(MonotonicTimestampMillis::new(500))
        }
    }

    #[test]
    fn clock_ports_are_independent_and_object_safe() {
        let wall: &dyn WallClock = &FixedWallClock;
        let monotonic: &dyn MonotonicClock = &FixedMonotonicClock;
        assert_eq!(wall.now().unwrap().whole_seconds(), 1_234);
        assert_eq!(monotonic.now().unwrap().get(), 500);
    }

    #[test]
    fn checked_time_arithmetic_never_wraps() {
        let duration = DurationMillis::new(50);
        assert_eq!(
            MonotonicTimestampMillis::new(100)
                .checked_add(duration)
                .unwrap()
                .get(),
            150
        );
        assert_eq!(
            MonotonicTimestampMillis::new(150)
                .elapsed_since(MonotonicTimestampMillis::new(100))
                .unwrap()
                .get(),
            50
        );
        assert!(
            MonotonicTimestampMillis::new(100)
                .elapsed_since(MonotonicTimestampMillis::new(150))
                .is_none()
        );
        assert_eq!(
            UnixTimestampMillis::new(u64::MAX)
                .checked_add(DurationMillis::new(1))
                .unwrap_err(),
            ClockError::Overflow
        );
    }

    #[test]
    fn civil_values_serialize_but_monotonic_values_have_no_wire_contract() {
        let unix = UnixTimestampMillis::new(1_234);
        let duration = DurationMillis::new(50);
        assert_eq!(serde_json::to_string(&unix).unwrap(), "1234");
        assert_eq!(
            serde_json::from_str::<DurationMillis>("50").unwrap(),
            duration
        );
    }

    #[test]
    fn serialized_time_values_enforce_u64_json_range() {
        let maximum = u64::MAX.to_string();
        assert_eq!(
            serde_json::from_str::<UnixTimestampMillis>(&maximum)
                .unwrap()
                .get(),
            u64::MAX
        );
        assert_eq!(
            serde_json::from_str::<DurationMillis>(&maximum)
                .unwrap()
                .get(),
            u64::MAX
        );

        for invalid in ["-1", "1.5", "18446744073709551616"] {
            assert!(serde_json::from_str::<UnixTimestampMillis>(invalid).is_err());
            assert!(serde_json::from_str::<DurationMillis>(invalid).is_err());
        }
    }

    #[test]
    fn clock_errors_bridge_without_runtime_detail() {
        let unavailable = ClockError::Unavailable.to_identus_error();
        assert_eq!(unavailable.code().as_str(), "core.clock_unavailable");
        assert_eq!(unavailable.kind(), ErrorKind::Internal);
        assert_eq!(unavailable.capability(), Some(CAPABILITY));

        let overflow = ClockError::Overflow.to_identus_error();
        assert_eq!(overflow.code().as_str(), "core.time_overflow");
        assert_eq!(overflow.kind(), ErrorKind::Internal);
    }
}
