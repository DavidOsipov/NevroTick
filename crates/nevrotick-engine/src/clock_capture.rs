// SPDX-License-Identifier: GPL-2.0-only
//! Local application captures at the original datagram adapter boundaries.

#[cfg(windows)]
use std::time::Instant;
#[cfg(any(windows, test))]
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(windows)]
use windows_platform_native::{ClockCoordinates, ClockReadError, read_clock_coordinates};

use crate::SoftwareClock;

/// The native clock-read outcome retained at an original adapter boundary.
///
/// Recorded coordinates identify the original local QPC and FILETIME reading.
/// They convey no clock continuity, UTC accuracy, or eligibility.
#[cfg(windows)]
#[must_use = "preserve the original native read disposition"]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum NativeClockObservation {
    /// The exact raw read also supplied this point's software wall value.
    Recorded(ClockCoordinates),
    /// The original clock adapter refused; transport used its fallback wall read.
    ReadRefused(ClockReadError),
    /// The exact raw read was retained but cannot supply a checked software wall value.
    UnrepresentableWall(ClockCoordinates),
}

/// The original software bracket and, on Windows, the result of its native read.
#[must_use = "retain the original adapter capture"]
#[derive(Clone, Copy)]
pub(super) struct ApplicationClockCapture {
    /// This point's process-local scheduling bracket and wall observation.
    software: SoftwareClock,
    /// Absence identifies an alternate backend that did not invoke Windows.
    #[cfg(windows)]
    native: Option<NativeClockObservation>,
}

impl ApplicationClockCapture {
    /// Retain a compatibility observation without any native evidence.
    #[cfg(any(not(windows), test))]
    pub(super) const fn software_only(software: SoftwareClock) -> Self {
        Self {
            software,
            #[cfg(windows)]
            native: None,
        }
    }

    /// Read immediately at the original datagram socket boundary.
    ///
    /// The Windows path uses a single QPC/`FILETIME`/QPC read for the retained
    /// coordinates and wall observation. A refused native read preserves its
    /// typed result and takes an ordinary wall reading inside the original
    /// scheduling bracket. Such a wall value supplies no native evidence.
    pub(super) fn capture() -> Self {
        #[cfg(not(windows))]
        {
            Self::software_only(SoftwareClock::capture())
        }
        #[cfg(windows)]
        {
            let mono: Instant = Instant::now();
            let (utc, observation): (SystemTime, NativeClockObservation) =
                select_native_wall(read_clock_coordinates(), SystemTime::now);
            Self {
                software: SoftwareClock {
                    mono,
                    utc,
                    mono_after: Instant::now(),
                },
                native: Some(observation),
            }
        }
    }

    /// Keep a controlled read refusal alongside the script's original compatibility reading.
    #[cfg(all(windows, test))]
    pub(super) fn with_scripted_refusal(self, error: Option<ClockReadError>) -> Self {
        let Some(error) = error else {
            return self;
        };
        let (utc, observation): (SystemTime, NativeClockObservation) =
            select_native_wall(Err(error), || self.software.utc);
        Self {
            software: SoftwareClock {
                utc,
                ..self.software
            },
            native: Some(observation),
        }
    }

    /// Copy the original software bracket for the bounded transport consumers.
    pub(super) const fn software(self) -> SoftwareClock {
        self.software
    }

    /// Copy the original raw native read outcome, when this backend attempted it.
    #[cfg(windows)]
    pub(super) const fn native(self) -> Option<NativeClockObservation> {
        self.native
    }
}

/// Keep the native read's wall when representable, or evaluate one fallback.
#[cfg(windows)]
fn select_native_wall(
    read: Result<ClockCoordinates, ClockReadError>,
    fallback: impl FnOnce() -> SystemTime,
) -> (SystemTime, NativeClockObservation) {
    match read {
        Ok(coordinates) => match wall_from_filetime(coordinates.filetime_100ns()) {
            Some(utc) => (utc, NativeClockObservation::Recorded(coordinates)),
            None => (
                fallback(),
                NativeClockObservation::UnrepresentableWall(coordinates),
            ),
        },
        Err(error) => (fallback(), NativeClockObservation::ReadRefused(error)),
    }
}

/// UTC-coordinate epoch distance in 100 ns ticks. It establishes no UTC truth.
#[cfg(any(windows, test))]
const UNIX_EPOCH_FILETIME: u64 = 116_444_736_000_000_000;
/// Number of `FILETIME` ticks in one whole second.
#[cfg(any(windows, test))]
const FILETIME_TICKS_PER_SECOND: u64 = 10_000_000;

/// Convert the original raw wall value. A host representation refusal cannot
/// become a successful native timing observation. Dividing before conversion
/// keeps the subsecond scaling independent of the full raw tick magnitude.
#[cfg(any(windows, test))]
fn wall_from_filetime(filetime: u64) -> Option<SystemTime> {
    let elapsed: u64 = filetime.abs_diff(UNIX_EPOCH_FILETIME);
    let subsecond: u32 = u32::try_from((elapsed % FILETIME_TICKS_PER_SECOND) * 100).ok()?;
    let offset: Duration = Duration::new(elapsed / FILETIME_TICKS_PER_SECOND, subsecond);
    if filetime < UNIX_EPOCH_FILETIME {
        UNIX_EPOCH.checked_sub(offset)
    } else {
        UNIX_EPOCH.checked_add(offset)
    }
}

#[cfg(test)]
mod tests {
    use super::{Duration, UNIX_EPOCH, wall_from_filetime};

    /// Independent epoch neighbors and mixed seconds/fractions on both sides.
    ///
    /// # Panics
    /// Panics if conversion changes an anchor's epoch, direction or 100 ns precision.
    #[test]
    fn filetime_conversion_preserves_direction_and_100ns_precision() {
        assert_eq!(
            wall_from_filetime(116_444_736_000_000_000),
            Some(UNIX_EPOCH)
        );
        assert_eq!(
            wall_from_filetime(116_444_735_989_999_999),
            UNIX_EPOCH.checked_sub(Duration::new(1, 100)),
        );
        assert_eq!(
            wall_from_filetime(116_444_736_010_000_001),
            UNIX_EPOCH.checked_add(Duration::new(1, 100)),
        );
        assert_eq!(
            wall_from_filetime(0),
            UNIX_EPOCH.checked_sub(Duration::from_hours(3_234_576)),
        );
    }

    /// Maximum unsigned value neither scales the full value nor truncates it.
    ///
    /// # Panics
    /// Panics if the maximum tick value yields a different time or representability result.
    #[test]
    fn filetime_conversion_preserves_maximum_tick_representation_result() {
        assert_eq!(
            wall_from_filetime(u64::MAX),
            UNIX_EPOCH.checked_add(Duration::new(1_833_029_933_770, 955_161_500)),
        );
    }

    /// The pinned Windows time representation admits the signed boundary tick.
    ///
    /// # Panics
    /// Panics if the highest signed tick refuses or its unsigned neighbor wraps.
    #[cfg(windows)]
    #[test]
    fn windows_filetime_signed_boundary_refuses_the_next_tick() {
        assert!(wall_from_filetime(9_223_372_036_854_775_807).is_some());
        assert!(wall_from_filetime(9_223_372_036_854_775_808).is_none());
    }

    /// Observe one real read-only Windows capture through the application capture adapter.
    ///
    /// # Errors
    /// Returns an error if the native coordinate or the independent epoch comparison is unavailable.
    ///
    /// # Panics
    /// Panics if the software wall differs from the retained raw tick value or the
    /// scheduling bracket falls outside the surrounding monotonic observations.
    #[cfg(windows)]
    #[test]
    fn native_capture_retains_its_original_wall_and_scheduling_bracket() -> Result<(), String> {
        use std::time::{Instant, SystemTime};
        use windows_platform_native::ClockCoordinates;

        use super::{ApplicationClockCapture, NativeClockObservation};
        use crate::SoftwareClock;

        let before: Instant = Instant::now();
        let capture: ApplicationClockCapture = ApplicationClockCapture::capture();
        let after: Instant = Instant::now();
        let coordinates: ClockCoordinates = match capture.native() {
            Some(NativeClockObservation::Recorded(coordinates)) => coordinates,
            Some(NativeClockObservation::ReadRefused(error)) => {
                return Err(format!("native clock read refused: {error:?}"));
            }
            Some(NativeClockObservation::UnrepresentableWall(_)) => {
                return Err(String::from("native wall coordinate was not representable"));
            }
            None => return Err(String::from("native clock read was absent")),
        };
        let software: SoftwareClock = capture.software();
        assert!(before <= software.monotonic());
        assert!(software.monotonic() <= software.monotonic_after());
        assert!(software.monotonic_after() <= after);

        // Measure from the Windows epoch and widen the entire raw tick value.
        let filetime_epoch: SystemTime = UNIX_EPOCH
            .checked_sub(Duration::from_hours(3_234_576))
            .ok_or_else(|| String::from("Windows epoch was not representable"))?;
        let observed: Duration = software
            .utc()
            .duration_since(filetime_epoch)
            .map_err(|_| String::from("software wall preceded the Windows epoch"))?;
        let expected_ns: u128 = u128::from(coordinates.filetime_100ns()) * 100;
        assert_eq!(observed.as_nanos(), expected_ns);
        Ok(())
    }
}
