//! Enforce the finite numeric envelope of immutable NTP-epoch Q32 coordinates.
//! No supported date interval, clock mapping, trust or native effect is implied.

use crate::DomainError;

/// Smallest coordinate: signed-i64 minimum whole seconds in Q32 ticks.
pub const MIN_COORDINATE_TICKS: i128 = -(1_i128 << 95);
/// Largest coordinate: signed-i64 maximum seconds plus maximal u32 fraction.
pub const MAX_COORDINATE_TICKS: i128 = (1_i128 << 95) - 1;

/// Claimed unfolded UTC coordinate, in 1/2^32 seconds from the NTP epoch.
///
/// Private state admits no raw-field construction or mutation:
/// ```compile_fail,E0451
/// use time_domain::UtcCoordinateQ32;
/// let _ = UtcCoordinateQ32 { ticks: i128::MAX };
/// ```
/// ```compile_fail,E0616
/// # use time_domain::{DomainError, UtcCoordinateQ32};
/// # fn main() -> Result<(), DomainError> {
/// let mut coordinate = UtcCoordinateQ32::new(0)?;
/// coordinate.ticks = i128::MAX;
/// # Ok(()) }
/// ```
/// Legal construction retains the exact coordinate:
/// ```
/// # use time_domain::{DomainError, UtcCoordinateQ32};
/// # fn main() -> Result<(), DomainError> {
/// let coordinate = UtcCoordinateQ32::new(-1)?;
/// assert_eq!(coordinate.ticks(), -1);
/// # Ok(()) }
/// ```
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UtcCoordinateQ32 {
    /// Checked numeric coordinate, never an authentication or provenance marker.
    ticks: i128,
}

impl UtcCoordinateQ32 {
    /// Admit only the inclusive numeric envelope, without allocation or effects.
    ///
    /// # Errors
    /// Rejects raw ticks outside -2^95 through 2^95-1.
    pub fn new(ticks: i128) -> Result<Self, DomainError> {
        if !coordinate_in_range(ticks) {
            return Err(DomainError::CoordinateOutOfRange);
        }
        Ok(Self { ticks })
    }

    /// Return the immutable exact Q32 coordinate; no conversion or effect occurs.
    #[must_use]
    pub const fn ticks(self) -> i128 {
        self.ticks
    }
}

/// Single allocation-free endpoint rule shared by scalar and garde admission.
#[must_use]
pub(crate) fn coordinate_in_range(ticks: i128) -> bool {
    (MIN_COORDINATE_TICKS..=MAX_COORDINATE_TICKS).contains(&ticks)
}
