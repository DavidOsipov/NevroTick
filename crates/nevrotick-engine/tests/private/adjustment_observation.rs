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
