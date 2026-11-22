// SPDX-License-Identifier: GPL-2.0-only
//! Actual observe projection with synthetic read outcomes. No Windows clock is changed.

use std::{
    cell::Cell,
    error::Error,
    str::{self, Split},
};

use super::adjustment_observation::{AdjustmentObservation, AdjustmentState, observe};

/// Bounded test failure or the independently checked record.
type Check<T = ()> = Result<T, Box<dyn Error>>;
/// One synthetic adapter result before conversion to the native observation shape.
type Reading = Result<(u64, u64, bool), u32>;

/// Exact diagnostic fields for the ordinary nonzero disabled observation.
const OBSERVED: &str = "status=observed adjustment=21 increment=7 periodic_disabled=1";
/// Exact diagnostic fields for the widest adjustment and the enabled mode.
const MAXIMUM: &str =
    "status=observed adjustment=18446744073709551615 increment=0 periodic_disabled=0";

/// Populate only the raw fields supplied by this test adapter.
fn state(adjustment: u64, increment: u64, periodic_adjustment_disabled: bool) -> AdjustmentState {
    AdjustmentState {
        adjustment,
        increment,
        periodic_adjustment_disabled,
    }
}

/// Parse only the decimal byte representation emitted by the observer.
/// # Errors
/// Reject an overflowing nanosecond field.
/// # Panics
/// Reject a missing or nondecimal field.
fn decimal_nanoseconds(value: &str) -> Check<u128> {
    assert!(!value.is_empty() && value.bytes().all(|byte: u8| byte.is_ascii_digit()));
    Ok(value.parse::<u128>()?)
}

/// Read one exact record without accepting unexpected fields or spacing.
/// # Errors
/// Reject missing structure or a malformed decimal.
/// # Panics
/// Reject substituted observation fields.
fn assert_read(line: &str, point: &str, expected: &str) -> Check<(u128, u128)> {
    let values: &str = line
        .strip_prefix("event=clock_adjustment point=")
        .and_then(|line: &str| line.strip_prefix(point))
        .and_then(|line: &str| line.strip_prefix(" bracket_start_ns="))
        .ok_or("missing exact bracket prefix")?;
    let (start, remaining): (&str, &str) = values
        .split_once(" bracket_end_ns=")
        .ok_or("missing bracket end")?;
    let (end, fields): (&str, &str) = remaining
        .split_once(' ')
        .ok_or("missing adjustment fields")?;
    assert_eq!(fields, expected);
    Ok((decimal_nanoseconds(start)?, decimal_nanoseconds(end)?))
}

/// Require all three ordered records with their unchanged mapping disposition.
/// # Errors
/// Preserve UTF-8, record boundary and decimal failures.
/// # Panics
/// Reject a reordered interval, unexpected output, or invented mapping status.
fn assert_projection(output: &[u8], before: &str, after: &str, summary: &str) -> Check {
    // The two bracket rows contain only fixed labels and bounded decimal fields.
    assert!(output.len() <= 640);
    let text: &str = str::from_utf8(output)?;
    let mut lines: Split<'_, char> = text
        .strip_suffix('\n')
        .ok_or("missing final record delimiter")?
        .split('\n');
    let (before_start, before_end): (u128, u128) =
        assert_read(lines.next().ok_or("missing before row")?, "before", before)?;
    let (after_start, after_end): (u128, u128) =
        assert_read(lines.next().ok_or("missing after row")?, "after", after)?;
    // Order only. No minimum gap or assumed resolution is needed.
    assert!(before_start <= before_end && before_end <= after_start && after_start <= after_end);
    assert_eq!(
        lines.next().ok_or("missing summary")?,
        format!(
            "event=clock_adjustment_summary state={summary} ordering=ordered mapping=unavailable"
        )
    );
    assert!(lines.next().is_none());
    Ok(())
}

/// Drive two supplied reads through the actual writer while preserving the result.
/// # Errors
/// Forward the actual writer failure.
/// # Panics
/// Reject changed operation results or a skipped read.
fn project_pair(readings: [Reading; 2]) -> Check<Vec<u8>> {
    let mut readings: std::array::IntoIter<Reading, 2> = readings.into_iter();
    let (result, observation): (u16, AdjustmentObservation) = observe(
        || {
            readings.next().ok_or(19_u32)?.map(
                |(adjustment, increment, disabled): (u64, u64, bool)| {
                    state(adjustment, increment, disabled)
                },
            )
        },
        || 73_u16,
    );
    assert_eq!(result, 73);
    assert_eq!(readings.len(), 0);
    let mut output: Vec<u8> = Vec::new();
    observation.write(&mut output)?;
    Ok(output)
}

/// Neither read can move across the operation, including a failed operation.
/// # Errors
/// Preserve a writer or observation decoding failure.
/// # Panics
/// Reject changed call order, acquisition error or reported observations.
#[test]
fn adjustment_observation_reads_bracket_the_failed_operation() -> Check {
    let position: Cell<u8> = Cell::new(0);
    let (result, observation): (Result<u16, &str>, AdjustmentObservation) = observe(
        || match position.get() {
            0 => {
                position.set(1);
                Ok(state(21, 7, true))
            }
            2 => {
                position.set(3);
                Ok(state(22, 7, true))
            }
            _ => {
                position.set(u8::MAX);
                Err(19)
            }
        },
        || {
            assert_eq!(position.replace(2), 1);
            Err("original acquisition failure")
        },
    );
    assert_eq!(result, Err("original acquisition failure"));
    assert_eq!(position.get(), 3);
    let mut output: Vec<u8> = Vec::new();
    observation.write(&mut output)?;
    assert_projection(
        &output,
        OBSERVED,
        "status=observed adjustment=22 increment=7 periodic_disabled=1",
        "changed",
    )
}

/// Each native word and the periodic mode independently determine equality.
/// # Errors
/// Preserve a writer or observation decoding failure.
/// # Panics
/// Reject truncation, mode erasure or any incorrect equality disposition.
#[test]
fn adjustment_observation_compares_every_raw_word_and_mode() -> Check {
    let initial: Reading = Ok((u64::MAX, 0, false));
    let cases: [(Reading, &str, &str); 4] = [
        (initial, MAXIMUM, "equal"),
        (
            Ok((0, 0, false)),
            "status=observed adjustment=0 increment=0 periodic_disabled=0",
            "changed",
        ),
        (
            Ok((u64::MAX, u64::MAX, false)),
            "status=observed adjustment=18446744073709551615 increment=18446744073709551615 periodic_disabled=0",
            "changed",
        ),
        (
            Ok((u64::MAX, 0, true)),
            "status=observed adjustment=18446744073709551615 increment=0 periodic_disabled=1",
            "changed",
        ),
    ];
    for (later, expected, summary) in cases {
        let output: Vec<u8> = project_pair([initial, later])?;
        assert_projection(&output, MAXIMUM, expected, summary)?;
    }
    Ok(())
}

/// Either missing input makes the comparison unavailable and keeps the raw errors.
/// # Errors
/// Preserve a writer or observation decoding failure.
/// # Panics
/// Reject fabricated values, a false comparison or changed acquisition result.
#[test]
fn adjustment_observation_read_failures_keep_results_and_bounded_output() -> Check {
    let observed: Reading = Ok((21, 7, true));
    let cases: [([Reading; 2], &str, &str); 3] = [
        (
            [Err(u32::MAX), observed],
            "status=unavailable error=4294967295",
            OBSERVED,
        ),
        ([observed, Err(0)], OBSERVED, "status=unavailable error=0"),
        (
            [Err(17), Err(23)],
            "status=unavailable error=17",
            "status=unavailable error=23",
        ),
    ];
    for (reads, before, after) in cases {
        let output: Vec<u8> = project_pair(reads)?;
        assert_projection(&output, before, after, "unavailable")?;
    }
    Ok(())
}

/// Sink refusal chosen independently from CLI error translation.
pub(super) const REFUSAL_CODE: i32 = 65_237;

/// Reject all writes in the selected record, retaining writes that did arrive.
#[derive(Default)]
pub(super) struct RecordSink {
    /// Previously accepted output.
    pub(super) bytes: Vec<u8>,
    /// Number of failed calls, including accidental attempts beyond the first.
    pub(super) failures: usize,
    /// The number of completed rows before the selected refusal.
    refused_row: Option<usize>,
}

impl RecordSink {
    /// Accept the specified number of complete rows, then refuse.
    pub(super) fn refusing(refused_row: usize) -> Self {
        Self {
            refused_row: Some(refused_row),
            ..Self::default()
        }
    }
}

impl std::io::Write for RecordSink {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let completed: usize = self
            .bytes
            .split_inclusive(|byte: &u8| *byte == b'\n')
            .filter(|row: &&[u8]| row.ends_with(b"\n"))
            .count();
        if self.refused_row == Some(completed) {
            self.failures += 1;
            return Err(std::io::Error::from_raw_os_error(REFUSAL_CODE));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Check precedence and stopped, healthy neighbor through the actual CLI kernel.
///
/// # Errors
/// Preserve host preparation, binding or output decoding failures.
/// # Panics
/// Reject omitted bookends, premature output, rewritten errors or false completion.
#[test]
fn adjustment_cli_keeps_attempted_reads_and_first_failure_through_close() -> Check {
    assert_composed_attempt(false, false, None, None)?;
    assert_composed_attempt(true, false, None, Some("entered_runtime"))?;
    assert_composed_attempt(false, true, None, Some("resume_unregistration_failed"))?;
    assert_composed_attempt(true, true, None, Some("entered_runtime"))?;
    for row in 0..3 {
        assert_composed_attempt(false, false, Some(row), Some("diagnostic_output_failed"))?;
        assert_composed_attempt(true, true, Some(row), Some("entered_runtime"))?;
    }
    Ok(())
}

/// Drive real bound publication, Engine execution or entered-runtime refusal,
/// native-close disposition and sink ordering through the shared CLI kernel.
///
/// # Errors
/// Preserve unexpected socket/projection/fixture setup failure.
/// # Panics
/// Reject incorrect read, cleanup, terminal output or retained error semantics.
fn assert_composed_attempt(
    entered: bool,
    close_fails: bool,
    refused_row: Option<usize>,
    expected_error: Option<&str>,
) -> Check {
    use super::{CliError, observe, write_bound};
    use nevrotick_engine::{Config, ControlHandle, ControlSchedule, PreparedPilot, prepare};
    use std::net::{SocketAddr, UdpSocket};
    use std::time::Duration;

    let first: UdpSocket = UdpSocket::bind("127.0.0.1:0")?;
    let second: UdpSocket = UdpSocket::bind("127.0.0.1:0")?;
    let endpoints: [SocketAddr; 2] = [first.local_addr()?, second.local_addr()?];
    let config: Config = Config::new(endpoints, Duration::from_secs(1))?;
    let (pilot, control): (PreparedPilot, ControlHandle) = prepare(&config)?;
    control.stop();
    let schedule: ControlSchedule = ControlSchedule::new(None, None)?;
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread().build()?;
    let entered_guard: Option<tokio::runtime::EnterGuard<'_>> = entered.then(|| runtime.enter());
    let mut sink: RecordSink = refused_row.map_or_else(RecordSink::default, RecordSink::refusing);
    let mut bound_output: Vec<u8> = Vec::new();
    let reads: Cell<usize> = Cell::new(0);
    let closed: Cell<bool> = Cell::new(false);
    let result: Result<(), CliError> = observe(
        pilot,
        schedule,
        |pilot: &PreparedPilot| write_bound(pilot, &mut bound_output),
        || {
            assert_eq!(reads.get(), 2);
            assert!(!closed.replace(true));
            if close_fails { Err(()) } else { Ok(()) }
        },
        || {
            assert!(closed.get());
            &mut sink
        },
        Some(|| {
            reads.set(reads.get() + 1);
            Ok(state(21, 7, true))
        }),
        None::<fn() -> super::leap_presentation::Observation>,
    );
    drop(entered_guard);
    assert!(closed.get());
    assert_eq!(str::from_utf8(&bound_output)?.lines().count(), 2);
    assert_eq!(result.as_ref().err().map(CliError::as_str), expected_error);
    if close_fails {
        let expected_primary: Option<&'static str> = entered.then_some("entered_runtime");
        assert!(
            matches!(result, Err(CliError::ResumeUnregistration { primary }) if primary == expected_primary)
        );
    } else if entered {
        assert!(matches!(
            result,
            Err(CliError::Pilot(
                nevrotick_engine::PilotError::EnteredRuntime
            ))
        ));
    }
    assert_attempt_output(&sink, refused_row, expected_error.is_none())
}

/// Distinguish attempted records from successful result/done publication.
///
/// # Errors
/// Preserve malformed stream errors.
/// # Panics
/// Reject missing surviving reads, wrong record counts or incomplete output as success.
fn assert_attempt_output(sink: &RecordSink, refused_row: Option<usize>, success: bool) -> Check {
    let text: &str = str::from_utf8(&sink.bytes)?;
    if let Some(row) = refused_row {
        assert_eq!(sink.failures, 1);
        assert_eq!(text.lines().count(), row);
        assert!(
            text.lines()
                .all(|line: &str| line.starts_with("event=clock_adjustment "))
        );
        return Ok(());
    }
    assert_eq!(sink.failures, 0);
    if success {
        let (observation, normal): (&str, &str) = text
            .split_once("event=result")
            .ok_or("missing normal report")?;
        assert_projection(observation.as_bytes(), OBSERVED, OBSERVED, "equal")?;
        assert_eq!(normal.matches(" status=stopped ").count(), 2);
        assert_eq!(normal.matches("event=done elapsed_ns=").count(), 1);
    } else {
        assert_projection(text.as_bytes(), OBSERVED, OBSERVED, "equal")?;
    }
    Ok(())
}
