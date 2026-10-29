//! Explicit fixture-origin monotonic and active-awake units, with typed elapsed
//! results. Callers must establish shared origin/generation and native semantics;
//! plain numbers cannot certify continuity, suspend behavior or provenance.

use crate::DomainError;

/// Nanosecond reading in one explicitly supplied monotonic fixture domain.
///
/// Cross-domain subtraction and raw-field mutation cannot compile:
/// ```compile_fail,E0308
/// use time_domain::{ActiveAwakeNs, MonotonicNs};
/// let _ = MonotonicNs::new(1).duration_since(ActiveAwakeNs::new(0));
/// ```
/// ```compile_fail,E0616
/// use time_domain::MonotonicNs;
/// let mut reading = MonotonicNs::new(0);
/// reading.nanoseconds = u64::MAX;
/// ```
/// Durations from another clock domain cannot extend a monotonic reading:
/// ```compile_fail,E0308
/// # use time_domain::{ActiveAwakeNs, DomainError, MonotonicNs};
/// # fn main() -> Result<(), DomainError> {
/// let awake = ActiveAwakeNs::new(1).duration_since(ActiveAwakeNs::new(0))?;
/// let _ = MonotonicNs::new(0).checked_add(awake);
/// # Ok(()) }
/// ```
/// Legal callers retain the elapsed domain:
/// ```
/// # use time_domain::{DomainError, MonotonicNs};
/// # fn main() -> Result<(), DomainError> {
/// let start = MonotonicNs::new(10);
/// let elapsed = MonotonicNs::new(13).duration_since(start)?;
/// assert_eq!(start.checked_add(elapsed)?.nanoseconds(), 13);
/// # Ok(()) }
/// ```
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MonotonicNs {
    /// Unit-tagged supplied reading; all u64 values are representable.
    nanoseconds: u64,
}

/// Nonnegative elapsed nanoseconds confined to the monotonic domain.
///
/// ```compile_fail,E0451
/// use time_domain::MonotonicDurationNs;
/// let _ = MonotonicDurationNs { nanoseconds: u64::MAX };
/// ```
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MonotonicDurationNs {
    /// Difference produced only by the checked same-domain operation.
    nanoseconds: u64,
}

/// Nanosecond reading in one explicitly supplied active-awake fixture domain.
///
/// ```compile_fail,E0308
/// use time_domain::{ActiveAwakeNs, MonotonicNs};
/// let _ = ActiveAwakeNs::new(1).duration_since(MonotonicNs::new(0));
/// ```
/// ```compile_fail,E0451
/// use time_domain::ActiveAwakeNs;
/// let _ = ActiveAwakeNs { nanoseconds: 0 };
/// ```
/// ```compile_fail,E0308
/// # use time_domain::{DomainError, MonotonicNs, UtcCoordinateQ32};
/// # fn main() -> Result<(), DomainError> {
/// let _ = MonotonicNs::new(0).duration_since(UtcCoordinateQ32::new(0)?);
/// # Ok(()) }
/// ```
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ActiveAwakeNs {
    /// Unit-tagged supplied reading, not a native suspend-semantics guarantee.
    nanoseconds: u64,
}

/// Nonnegative elapsed nanoseconds confined to the active-awake domain.
///
/// ```compile_fail,E0451
/// use time_domain::ActiveAwakeDurationNs;
/// let _ = ActiveAwakeDurationNs { nanoseconds: u64::MAX };
/// ```
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ActiveAwakeDurationNs {
    /// Difference produced only by the checked same-domain operation.
    nanoseconds: u64,
}

impl MonotonicNs {
    /// Tag a supplied reading without certifying its origin or generation.
    #[must_use]
    pub const fn new(nanoseconds: u64) -> Self {
        Self { nanoseconds }
    }

    /// Return the immutable unit-tagged reading without effects.
    #[must_use]
    pub const fn nanoseconds(self) -> u64 {
        self.nanoseconds
    }

    /// Compute an ordered same-origin/generation elapsed reading without effects.
    ///
    /// The caller supplies both readings from one origin/generation.
    /// # Errors
    /// Rejects reversed readings rather than wrapping or clamping.
    pub fn duration_since(self, earlier: Self) -> Result<MonotonicDurationNs, DomainError> {
        let nanoseconds = self
            .nanoseconds
            .checked_sub(earlier.nanoseconds)
            .ok_or(DomainError::DurationOutOfOrder)?;
        Ok(MonotonicDurationNs { nanoseconds })
    }

    /// Add a same-domain elapsed amount with an exact checked u64 result.
    ///
    /// # Errors
    /// Rejects overflow without saturating, changing another reading or effects.
    pub fn checked_add(self, elapsed: MonotonicDurationNs) -> Result<Self, DomainError> {
        let nanoseconds = self
            .nanoseconds
            .checked_add(elapsed.nanoseconds)
            .ok_or(DomainError::ArithmeticOverflow)?;
        Ok(Self { nanoseconds })
    }
}

impl MonotonicDurationNs {
    /// Return the immutable nonnegative monotonic difference without effects.
    #[must_use]
    pub const fn nanoseconds(self) -> u64 {
        self.nanoseconds
    }
}

impl ActiveAwakeNs {
    /// Tag a supplied reading without certifying its origin or native semantics.
    #[must_use]
    pub const fn new(nanoseconds: u64) -> Self {
        Self { nanoseconds }
    }

    /// Return the immutable unit-tagged reading without effects.
    #[must_use]
    pub const fn nanoseconds(self) -> u64 {
        self.nanoseconds
    }

    /// Compute ordered elapsed readings from the caller's same origin/generation.
    ///
    /// # Errors
    /// Rejects reversed readings without wrapping, clamping or effects.
    pub fn duration_since(self, earlier: Self) -> Result<ActiveAwakeDurationNs, DomainError> {
        let nanoseconds = self
            .nanoseconds
            .checked_sub(earlier.nanoseconds)
            .ok_or(DomainError::DurationOutOfOrder)?;
        Ok(ActiveAwakeDurationNs { nanoseconds })
    }

    /// Add same-domain elapsed nanoseconds with an exact checked u64 result.
    ///
    /// # Errors
    /// Rejects overflow before returning any reading; performs no effects.
    pub fn checked_add(self, elapsed: ActiveAwakeDurationNs) -> Result<Self, DomainError> {
        let nanoseconds = self
            .nanoseconds
            .checked_add(elapsed.nanoseconds)
            .ok_or(DomainError::ArithmeticOverflow)?;
        Ok(Self { nanoseconds })
    }
}

impl ActiveAwakeDurationNs {
    /// Return the immutable nonnegative active-awake difference without effects.
    #[must_use]
    pub const fn nanoseconds(self) -> u64 {
        self.nanoseconds
    }
}
