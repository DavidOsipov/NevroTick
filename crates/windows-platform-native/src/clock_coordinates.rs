// SPDX-License-Identifier: GPL-2.0-only
//! Validate one read-only local Windows clock bracket.

use std::num::NonZeroU64;

/// Raw Windows clock coordinates from one completed read sequence.
///
/// The QPC ticks bracket the FILETIME call in call order. Consumers need
/// separate evidence of the UTC mapping, its uncertainty, and its validity
/// at the time of use. QPC ticks do not establish active-awake elapsed time.
#[must_use = "retain the actual clock coordinates"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClockCoordinates {
    /// Positive QPC frequency returned for this read, in ticks per second.
    frequency_hz: NonZeroU64,
    /// Raw signed QPC tick read before FILETIME.
    qpc_before: i64,
    /// Raw Windows FILETIME value, in 100-nanosecond units.
    filetime_100ns: u64,
    /// Raw signed QPC tick read after FILETIME, at least `qpc_before`.
    qpc_after: i64,
}

impl ClockCoordinates {
    /// Return the QPC frequency in ticks per second.
    #[must_use]
    pub const fn frequency_hz(&self) -> NonZeroU64 {
        self.frequency_hz
    }

    /// Return the first raw signed QPC tick.
    #[must_use]
    pub const fn qpc_before(&self) -> i64 {
        self.qpc_before
    }

    /// Return the raw FILETIME value in 100-nanosecond units.
    #[must_use]
    pub const fn filetime_100ns(&self) -> u64 {
        self.filetime_100ns
    }

    /// Return the final raw signed QPC tick.
    #[must_use]
    pub const fn qpc_after(&self) -> i64 {
        self.qpc_after
    }
}

/// Earliest failure while reading or validating a local clock bracket.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClockReadError {
    /// QPC frequency read failed with this original Win32 error code.
    Frequency(u32),
    /// QPC frequency read succeeded with a zero or negative value.
    InvalidFrequency,
    /// The first QPC tick read failed with this original Win32 error code.
    Before(u32),
    /// The final QPC tick read failed with this original Win32 error code.
    After(u32),
    /// The final signed QPC tick preceded the first.
    Reversed,
}

/// Synchronous read-only clock calls used for one bracket.
pub(crate) trait ClockCalls {
    /// Read the raw signed QPC frequency in ticks per second.
    ///
    /// # Errors
    /// Return the original current-thread Win32 error code on failure, including zero.
    fn frequency(&mut self) -> Result<i64, u32>;

    /// Read one raw signed QPC tick.
    ///
    /// # Errors
    /// Return the original current-thread Win32 error code on failure, including zero.
    fn counter(&mut self) -> Result<i64, u32>;

    /// Read one raw Windows FILETIME value in 100-nanosecond units.
    fn filetime(&mut self) -> u64;
}

/// Sample frequency, QPC, FILETIME, and QPC in that order, once each.
///
/// # Errors
/// Stop at the first failing call or invalid value. Preserve the original
/// Win32 failure code, or report a nonpositive frequency or reversed bracket.
pub(crate) fn read_coordinates(
    calls: &mut impl ClockCalls,
) -> Result<ClockCoordinates, ClockReadError> {
    let raw_frequency: i64 = calls.frequency().map_err(ClockReadError::Frequency)?;
    let frequency: u64 =
        u64::try_from(raw_frequency).map_err(|_| ClockReadError::InvalidFrequency)?;
    let frequency_hz: NonZeroU64 =
        NonZeroU64::new(frequency).ok_or(ClockReadError::InvalidFrequency)?;

    let qpc_before: i64 = calls.counter().map_err(ClockReadError::Before)?;
    let filetime_100ns: u64 = calls.filetime();
    let qpc_after: i64 = calls.counter().map_err(ClockReadError::After)?;
    if qpc_after < qpc_before {
        return Err(ClockReadError::Reversed);
    }

    Ok(ClockCoordinates {
        frequency_hz,
        qpc_before,
        filetime_100ns,
        qpc_after,
    })
}

#[cfg(test)]
mod tests {
    use super::{ClockCalls, ClockCoordinates, ClockReadError, read_coordinates};

    /// Operation tags recorded independently of the returned clock values.
    #[derive(Debug, Eq, PartialEq)]
    enum Call {
        /// QPC frequency.
        Frequency,
        /// QPC tick.
        Counter,
        /// Windows FILETIME.
        Filetime,
    }

    /// Expected call history for a fully attempted bracket.
    const FULL_BRACKET: [Call; 4] = [
        Call::Frequency,
        Call::Counter,
        Call::Filetime,
        Call::Counter,
    ];

    /// Fixed call replies and a record of each operation the reader attempted.
    struct ScriptedCalls {
        /// Frequency reply.
        frequency: Result<i64, u32>,
        /// Reply to the first QPC call, taken when used.
        first_counter: Option<Result<i64, u32>>,
        /// Reply to subsequent QPC calls.
        next_counter: Result<i64, u32>,
        /// Raw FILETIME reply.
        filetime: u64,
        /// Record of all attempts, including failed or extra calls.
        observed: Vec<Call>,
    }

    impl ScriptedCalls {
        /// Start with a fixed valid bracket and an empty call history.
        fn valid() -> Self {
            Self {
                frequency: Ok(10_000_000),
                first_counter: Some(Ok(21)),
                next_counter: Ok(25),
                filetime: 132_223_104_000_000_000,
                observed: Vec::new(),
            }
        }
    }

    impl ClockCalls for ScriptedCalls {
        fn frequency(&mut self) -> Result<i64, u32> {
            self.observed.push(Call::Frequency);
            self.frequency
        }

        fn counter(&mut self) -> Result<i64, u32> {
            self.observed.push(Call::Counter);
            self.first_counter.take().unwrap_or(self.next_counter)
        }

        fn filetime(&mut self) -> u64 {
            self.observed.push(Call::Filetime);
            self.filetime
        }
    }

    /// Keep the largest positive native frequency, signed ticks and raw FILETIME.
    ///
    /// # Errors
    /// Return an unexpected bracket refusal.
    ///
    /// # Panics
    /// Fail on a changed call sequence or any changed raw coordinate.
    #[test]
    fn successful_read_keeps_raw_limits_in_call_order() -> Result<(), ClockReadError> {
        let mut calls: ScriptedCalls = ScriptedCalls {
            frequency: Ok(i64::MAX),
            first_counter: Some(Ok(i64::MIN)),
            next_counter: Ok(i64::MAX),
            filetime: u64::MAX,
            observed: Vec::new(),
        };
        let reading: ClockCoordinates = read_coordinates(&mut calls)?;
        assert_eq!(calls.observed, FULL_BRACKET);
        assert_eq!(reading.frequency_hz().get(), 9_223_372_036_854_775_807);
        assert_eq!(reading.qpc_before(), i64::MIN);
        assert_eq!(reading.filetime_100ns(), u64::MAX);
        assert_eq!(reading.qpc_after(), i64::MAX);
        Ok(())
    }

    /// Accept an equal negative bracket, the smallest valid frequency and FILETIME zero.
    ///
    /// # Errors
    /// Return an unexpected bracket refusal.
    ///
    /// # Panics
    /// Fail on a changed call sequence or any changed raw coordinate.
    #[test]
    fn equal_ticks_at_the_smallest_positive_frequency_are_valid() -> Result<(), ClockReadError> {
        let mut calls: ScriptedCalls = ScriptedCalls {
            frequency: Ok(1),
            first_counter: Some(Ok(-1)),
            next_counter: Ok(-1),
            filetime: 0,
            observed: Vec::new(),
        };
        let reading: ClockCoordinates = read_coordinates(&mut calls)?;
        assert_eq!(calls.observed, FULL_BRACKET);
        assert_eq!(reading.frequency_hz().get(), 1);
        assert_eq!(reading.qpc_before(), -1);
        assert_eq!(reading.filetime_100ns(), 0);
        assert_eq!(reading.qpc_after(), -1);
        Ok(())
    }

    /// Reject nonpositive frequencies before reading either clock coordinate.
    ///
    /// # Panics
    /// Fail if an invalid frequency is accepted or any later call is attempted.
    #[test]
    fn nonpositive_frequency_stops_after_frequency_call() {
        for raw_frequency in [i64::MIN, -1, 0] {
            let mut calls: ScriptedCalls = ScriptedCalls {
                frequency: Ok(raw_frequency),
                ..ScriptedCalls::valid()
            };
            assert_eq!(
                read_coordinates(&mut calls),
                Err(ClockReadError::InvalidFrequency)
            );
            assert_eq!(calls.observed, [Call::Frequency]);
        }
    }

    /// Preserve the native frequency error code and skip all later calls.
    ///
    /// # Panics
    /// Fail on an incorrect error or any extra call.
    #[test]
    fn frequency_failure_stops_after_frequency_call() {
        for code in [0, u32::MAX] {
            let mut calls: ScriptedCalls = ScriptedCalls {
                frequency: Err(code),
                ..ScriptedCalls::valid()
            };
            assert_eq!(
                read_coordinates(&mut calls),
                Err(ClockReadError::Frequency(code))
            );
            assert_eq!(calls.observed, [Call::Frequency]);
        }
    }

    /// Preserve the first tick's native error code and skip FILETIME and the final tick.
    ///
    /// # Panics
    /// Fail on an incorrect error or any extra call.
    #[test]
    fn before_failure_stops_before_filetime_call() {
        for code in [0, u32::MAX] {
            let mut calls: ScriptedCalls = ScriptedCalls {
                first_counter: Some(Err(code)),
                ..ScriptedCalls::valid()
            };
            assert_eq!(
                read_coordinates(&mut calls),
                Err(ClockReadError::Before(code))
            );
            assert_eq!(calls.observed, [Call::Frequency, Call::Counter]);
        }
    }

    /// Preserve the final tick's native error code without producing a partial bracket.
    ///
    /// # Panics
    /// Fail on an incorrect error or a changed call sequence.
    #[test]
    fn after_failure_returns_its_own_code_after_full_call_sequence() {
        for code in [0, u32::MAX] {
            let mut calls: ScriptedCalls = ScriptedCalls {
                next_counter: Err(code),
                ..ScriptedCalls::valid()
            };
            assert_eq!(
                read_coordinates(&mut calls),
                Err(ClockReadError::After(code))
            );
            assert_eq!(calls.observed, FULL_BRACKET);
        }
    }

    /// Refuse reversed signed ticks even when their difference cannot fit in an `i64`.
    ///
    /// # Panics
    /// Fail if either reversed bracket is accepted or if the call sequence changes.
    #[test]
    fn reversed_ticks_are_rejected_after_full_call_sequence() {
        for (qpc_before, qpc_after) in [(0, -1), (i64::MAX, i64::MIN)] {
            let mut calls: ScriptedCalls = ScriptedCalls {
                first_counter: Some(Ok(qpc_before)),
                next_counter: Ok(qpc_after),
                ..ScriptedCalls::valid()
            };
            assert_eq!(read_coordinates(&mut calls), Err(ClockReadError::Reversed));
            assert_eq!(calls.observed, FULL_BRACKET);
        }
    }
}
