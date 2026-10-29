//! Construct explicit numeric windows and resolve raw NTP eras in constant work.
//! Windows supply no anchor provenance, local-date fallback or authority.

use garde::Validate;

use crate::{DomainError, UtcCoordinateQ32, coordinate::coordinate_in_range};

/// One NTP era in Q32 ticks; a positive fixed divisor for every raw timestamp.
const ERA_TICKS: i128 = 1_i128 << 64;

/// Fixed two-field admission candidate; every field executes the shared rule.
#[derive(Validate)]
struct EraWindowCandidate {
    /// Inclusive raw lower endpoint in Q32 ticks.
    #[garde(custom(endpoint_rule))]
    earliest: i128,
    /// Inclusive raw upper endpoint in Q32 ticks.
    #[garde(custom(endpoint_rule))]
    latest: i128,
}

/// Apply the sole numeric endpoint predicate through garde's borrowed signature.
///
/// # Errors
/// Returns only a fixed redacted error for an out-of-envelope endpoint.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules require borrowed values and context"
)]
fn endpoint_rule(value: &i128, (): &()) -> garde::Result {
    if coordinate_in_range(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid coordinate endpoint"))
    }
}

/// Single pure cross-field ordering rule; fixed work and no effects.
fn ordered_endpoints(earliest: i128, latest: i128) -> bool {
    earliest <= latest
}

/// Inclusive checked numeric interval, carrying no trust in its supplied anchor.
///
/// ```compile_fail,E0451
/// # use time_domain::{DomainError, EraWindow, UtcCoordinateQ32};
/// # fn main() -> Result<(), DomainError> {
/// let endpoint = UtcCoordinateQ32::new(0)?;
/// let _ = EraWindow { earliest: endpoint, latest: endpoint };
/// # Ok(()) }
/// ```
/// ```
/// # use time_domain::{DomainError, EraWindow, unfold_ntp};
/// # fn main() -> Result<(), DomainError> {
/// let window = EraWindow::new(0, 1)?;
/// assert_eq!(unfold_ntp(0, 1, &window)?.ticks(), 1);
/// # Ok(()) }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EraWindow {
    /// Inclusive checked lower endpoint.
    earliest: UtcCoordinateQ32,
    /// Inclusive checked upper endpoint.
    latest: UtcCoordinateQ32,
}

impl EraWindow {
    /// Validate both raw endpoints through garde and the sole ordering rule.
    ///
    /// Only two scalar fields and fixed redacted report paths are processed.
    /// A garde report is dropped before returning; no input/report is retained.
    ///
    /// # Errors
    /// Rejects out-of-envelope or inverted endpoints without publishing a window.
    pub fn new(earliest: i128, latest: i128) -> Result<Self, DomainError> {
        let candidate = EraWindowCandidate { earliest, latest };
        candidate
            .validate()
            .map_err(|_| DomainError::InvalidEraWindow)?;
        if !ordered_endpoints(earliest, latest) {
            return Err(DomainError::InvalidEraWindow);
        }
        Ok(Self {
            earliest: UtcCoordinateQ32::new(earliest)?,
            latest: UtcCoordinateQ32::new(latest)?,
        })
    }

    /// Return the immutable inclusive lower coordinate without effects.
    #[must_use]
    pub const fn earliest(self) -> UtcCoordinateQ32 {
        self.earliest
    }

    /// Return the immutable inclusive upper coordinate without effects.
    #[must_use]
    pub const fn latest(self) -> UtcCoordinateQ32 {
        self.latest
    }
}

/// Resolve raw seconds/fraction to the unique coordinate in the explicit window.
///
/// Required work is constant in interval width; all raw bits remain exact.
/// There is no ambient pivot, nearest-era tie break, allocation or side effect.
///
/// # Errors
/// Distinguishes no candidate from multiple candidates and rejects checked
/// arithmetic/output failures before publishing a coordinate.
pub fn unfold_ntp(
    seconds: u32,
    fraction: u32,
    window: &EraWindow,
) -> Result<UtcCoordinateQ32, DomainError> {
    let raw = i128::from((u64::from(seconds) << 32) | u64::from(fraction));
    let earliest = window.earliest().ticks();
    let latest = window.latest().ticks();
    let lower_delta = earliest
        .checked_sub(raw)
        .ok_or(DomainError::ArithmeticOverflow)?;
    let upper_delta = latest
        .checked_sub(raw)
        .ok_or(DomainError::ArithmeticOverflow)?;
    // Euclidean quotient/remainder implement ceil for either sign, without
    // negating a bound or adding a whole era to the potentially large dividend.
    let first_era = lower_delta
        .div_euclid(ERA_TICKS)
        .checked_add(i128::from(lower_delta.rem_euclid(ERA_TICKS) != 0))
        .ok_or(DomainError::ArithmeticOverflow)?;
    let last_era = upper_delta.div_euclid(ERA_TICKS);
    if first_era > last_era {
        return Err(DomainError::NoEraCandidate);
    }
    if first_era < last_era {
        return Err(DomainError::AmbiguousEra);
    }
    let ticks = first_era
        .checked_mul(ERA_TICKS)
        .and_then(|base| base.checked_add(raw))
        .ok_or(DomainError::ArithmeticOverflow)?;
    if ticks < earliest || ticks > latest {
        return Err(DomainError::NoEraCandidate);
    }
    UtcCoordinateQ32::new(ticks)
}
