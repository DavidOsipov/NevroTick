// SPDX-License-Identifier: GPL-2.0-only
//! Local application captures at the original datagram adapter boundaries.

#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
use std::num::NonZeroU64;
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

/// Why the original native capture triplet cannot supply an ordered QPC span.
#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtectedNativeCaptureRefusal {
    /// The adapter did not attempt a native read at one of the original points.
    #[cfg(windows)]
    Absent,
    /// An original native read refused with this typed cause.
    #[cfg(windows)]
    ReadRefused(ClockReadError),
    /// An original raw wall reading had no checked software representation.
    #[cfg(windows)]
    UnrepresentableWall,
    /// The original captures report different QPC frequencies.
    FrequencyChanged,
    /// One bracket is reversed or two successive brackets overlap.
    CaptureOrder,
}

/// The original three application brackets from a protected receipt.
///
/// Each bracket is ordered and the three share one reported QPC frequency.
/// Success makes the original application span available for comparison to a
/// mapping's validity. Physical placement, continuity and eligibility need
/// separate evidence.
#[cfg(all(feature = "ke-association-prototype", windows))]
#[must_use = "retain the original checked application capture span"]
pub struct ProtectedNativeCaptureSpan {
    /// Pre-send, send-return and selected receive-return readings, in order.
    captures: [ClockCoordinates; 3],
}

#[cfg(all(feature = "ke-association-prototype", windows))]
impl ProtectedNativeCaptureSpan {
    /// Check the original receipt's native read dispositions in capture order.
    ///
    /// # Errors
    /// Preserve the first absent, failed or unrepresentable read, then refuse
    /// incompatible frequencies or unordered brackets. No clock is read here.
    pub(crate) fn from_observations(
        [pre_send, send_return, receive_return]: [Option<NativeClockObservation>; 3],
    ) -> Result<Self, ProtectedNativeCaptureRefusal> {
        let captures: [ClockCoordinates; 3] = [
            recorded_coordinates(pre_send)?,
            recorded_coordinates(send_return)?,
            recorded_coordinates(receive_return)?,
        ];
        check_qpc_triplet(
            captures.map(|capture: ClockCoordinates| capture.frequency_hz()),
            captures.map(|capture: ClockCoordinates| [capture.qpc_before(), capture.qpc_after()]),
        )?;
        Ok(Self { captures })
    }

    /// Return the original pre-send, send-return and receive-return brackets.
    pub const fn captures(&self) -> [ClockCoordinates; 3] {
        self.captures
    }

    /// Return the shared frequency, in QPC ticks per second.
    #[must_use]
    pub const fn frequency_hz(&self) -> NonZeroU64 {
        let [first, _, _]: [ClockCoordinates; 3] = self.captures;
        first.frequency_hz()
    }

    /// Return the inclusive first and last QPC ticks of the original span.
    #[must_use]
    pub const fn qpc_bounds(&self) -> [i64; 2] {
        let [first, _, last]: [ClockCoordinates; 3] = self.captures;
        [first.qpc_before(), last.qpc_after()]
    }
}

/// Preserve the original disposition when a capture cannot furnish coordinates.
///
/// # Errors
/// Return the typed absence, read refusal or wall representation failure.
#[cfg(all(feature = "ke-association-prototype", windows))]
const fn recorded_coordinates(
    observation: Option<NativeClockObservation>,
) -> Result<ClockCoordinates, ProtectedNativeCaptureRefusal> {
    match observation {
        Some(NativeClockObservation::Recorded(coordinates)) => Ok(coordinates),
        Some(NativeClockObservation::ReadRefused(error)) => {
            Err(ProtectedNativeCaptureRefusal::ReadRefused(error))
        }
        Some(NativeClockObservation::UnrepresentableWall(_)) => {
            Err(ProtectedNativeCaptureRefusal::UnrepresentableWall)
        }
        None => Err(ProtectedNativeCaptureRefusal::Absent),
    }
}

/// Check the whole triplet in its original order without tick arithmetic.
///
/// # Errors
/// Refuse unequal frequencies or any reversed edge. Touching endpoints are valid.
#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
fn check_qpc_triplet(
    [pre_frequency, send_frequency, receive_frequency]: [NonZeroU64; 3],
    [
        [pre_start, pre_end],
        [send_start, send_end],
        [receive_start, receive_end],
    ]: [[i64; 2]; 3],
) -> Result<(), ProtectedNativeCaptureRefusal> {
    if pre_frequency != send_frequency || pre_frequency != receive_frequency {
        return Err(ProtectedNativeCaptureRefusal::FrequencyChanged);
    }
    if pre_start > pre_end
        || pre_end > send_start
        || send_start > send_end
        || send_end > receive_start
        || receive_start > receive_end
    {
        return Err(ProtectedNativeCaptureRefusal::CaptureOrder);
    }
    Ok(())
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
    #[cfg(feature = "ke-association-prototype")]
    use super::{NonZeroU64, ProtectedNativeCaptureRefusal, check_qpc_triplet};

    /// Check inclusive ordering across the full signed tick range.
    /// # Panics
    /// Panics if an equal edge or extreme tick is refused, or an overlap is admitted.
    #[cfg(feature = "ke-association-prototype")]
    #[test]
    fn qpc_triplet_checks_each_one_tick_overlap_beside_touching_edges() {
        let frequency: NonZeroU64 = NonZeroU64::MIN;
        let touching: [[i64; 2]; 3] = [[i64::MIN, -4], [-4, 2], [2, i64::MAX]];
        assert_eq!(check_qpc_triplet([frequency; 3], touching), Ok(()));

        let overlaps: [[[i64; 2]; 3]; 2] = [
            [[i64::MIN, -4], [-5, 2], [2, i64::MAX]],
            [[i64::MIN, -4], [-4, 2], [1, i64::MAX]],
        ];
        for triplet in overlaps {
            assert_eq!(
                check_qpc_triplet([frequency; 3], triplet),
                Err(ProtectedNativeCaptureRefusal::CaptureOrder),
            );
        }
    }

    /// Challenge the second and third frequency independently.
    /// # Panics
    /// Panics if a triplet mixes frequencies or refuses a common nondefault frequency.
    #[cfg(feature = "ke-association-prototype")]
    #[test]
    fn qpc_triplet_rejects_frequency_changes_at_either_later_capture() {
        let alternate: NonZeroU64 = NonZeroU64::MIN;
        let frequency: NonZeroU64 = NonZeroU64::MAX;
        let ticks: [[i64; 2]; 3] = [[1, 2], [3, 4], [5, 6]];
        assert_eq!(check_qpc_triplet([frequency; 3], ticks), Ok(()));
        for frequencies in [
            [frequency, alternate, frequency],
            [frequency, frequency, alternate],
        ] {
            assert_eq!(
                check_qpc_triplet(frequencies, ticks),
                Err(ProtectedNativeCaptureRefusal::FrequencyChanged),
            );
        }
    }

    /// Enumerate six ticks over a small signed range and compare with sorting.
    /// # Panics
    /// Panics if the real comparison disagrees with the independent sorted sequence.
    #[cfg(feature = "ke-association-prototype")]
    #[test]
    fn qpc_triplet_order_matches_sorted_sequence_for_small_signed_ticks() {
        for mut encoding in 0..729_u32 {
            let mut original: [i64; 6] = [0; 6];
            for tick in &mut original {
                *tick = i64::from(encoding % 3) - 1;
                encoding /= 3;
            }
            let mut sorted: [i64; 6] = original;
            sorted.sort_unstable();
            let [first, second, third, fourth, fifth, sixth]: [i64; 6] = original;
            assert_eq!(
                check_qpc_triplet(
                    [NonZeroU64::MIN; 3],
                    [[first, second], [third, fourth], [fifth, sixth]],
                )
                .is_ok(),
                original == sorted,
                "{original:?}",
            );
        }
    }

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
        #[cfg(feature = "ke-association-prototype")]
        {
            use super::ProtectedNativeCaptureSpan;

            assert!(matches!(
                ProtectedNativeCaptureSpan::from_observations([
                    Some(NativeClockObservation::Recorded(coordinates)),
                    Some(NativeClockObservation::UnrepresentableWall(coordinates)),
                    None,
                ]),
                Err(ProtectedNativeCaptureRefusal::UnrepresentableWall)
            ));
        }
        Ok(())
    }
}
