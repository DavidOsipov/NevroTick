// SPDX-License-Identifier: GPL-2.0-only
//! Deterministic bookend permutations and record-writer custody.

use std::time::{Duration, Instant};

use crate::adjustment_observation_tests::{REFUSAL_CODE, RecordSink};

use super::{AdjustmentObservation, AdjustmentState, observe_with_clock};

/// Independent literal oracle for equal instants.
const EQUAL: &str = concat!(
    "event=clock_adjustment point=before bracket_start_ns=0 bracket_end_ns=0 status=observed adjustment=21 increment=7 periodic_disabled=1\n",
    "event=clock_adjustment point=after bracket_start_ns=0 bracket_end_ns=0 status=unavailable error=0\n",
    "event=clock_adjustment_summary state=unavailable ordering=ordered mapping=unavailable\n",
);
/// Any ordering refusal removes both offset pairs and retains original facts.
const UNORDERED: &str = concat!(
    "event=clock_adjustment point=before status=observed adjustment=21 increment=7 periodic_disabled=1\n",
    "event=clock_adjustment point=after status=unavailable error=0\n",
    "event=clock_adjustment_summary state=unavailable ordering=unavailable mapping=unavailable\n",
);

/// Run the same capture code as production with four independent clock observations.
///
/// # Errors
/// Refuse an unrepresentable fixture clock before capturing.
/// # Panics
/// Fail if the capture path uses any different number of reads or time observations.
fn observe_at(coordinates: [u64; 4]) -> Result<AdjustmentObservation, &'static str> {
    let origin: Instant = Instant::now();
    let [a, b, c, d]: [Option<Instant>; 4] =
        coordinates.map(|offset: u64| origin.checked_add(Duration::from_nanos(offset)));
    let [Some(a), Some(b), Some(c), Some(d)] = [a, b, c, d] else {
        return Err("fixture-coordinate");
    };
    let mut times: std::array::IntoIter<Instant, 4> = [a, b, c, d].into_iter();
    let mut reads: std::array::IntoIter<Result<AdjustmentState, u32>, 2> = [
        Ok(AdjustmentState {
            adjustment: 21,
            increment: 7,
            periodic_adjustment_disabled: true,
        }),
        Err(0),
    ]
    .into_iter();
    let (operation, observation): (u16, AdjustmentObservation) = observe_with_clock(
        || reads.next().unwrap_or(Err(19)),
        || 73_u16,
        || {
            assert_ne!(times.len(), 0);
            times.next().unwrap_or(origin)
        },
    );
    assert_eq!(operation, 73);
    assert_eq!(reads.len(), 0);
    assert_eq!(times.len(), 0);
    Ok(observation)
}

/// Refuse individually reversed brackets or overlap while admitting equality.
///
/// # Errors
/// Preserve fixture, capture or writer refusal.
/// # Panics
/// Fail if any invalid history exposes even one offset, or loses raw facts.
#[test]
fn adjustment_order_rejects_each_inversion_and_admits_equal_instants()
-> Result<(), Box<dyn std::error::Error>> {
    for (sequence, expected) in [
        ([1, 0, 2, 3], UNORDERED),
        ([0, 2, 1, 3], UNORDERED),
        ([0, 1, 3, 2], UNORDERED),
        ([0, 0, 0, 0], EQUAL),
    ] {
        let mut output: Vec<u8> = Vec::new();
        observe_at(sequence)?.write(&mut output)?;
        assert_eq!(output, expected.as_bytes());
    }
    Ok(())
}

/// An error in each actual record preserves its exact source code and stops output.
///
/// # Errors
/// Preserve unexpected setup or incorrect successful publication.
/// # Panics
/// Reject a replacement error, changed earlier bytes or any write after refusal.
#[test]
fn adjustment_writer_preserves_each_record_refusal_and_stops()
-> Result<(), Box<dyn std::error::Error>> {
    let observation: AdjustmentObservation = observe_at([0; 4])?;
    for refused_row in 0..3 {
        let mut sink: RecordSink = RecordSink::refusing(refused_row);
        let result: std::io::Error = observation
            .write(&mut sink)
            .err()
            .ok_or("sink refusal lost")?;
        assert_eq!(result.raw_os_error(), Some(REFUSAL_CODE));
        assert_eq!(sink.failures, 1);
        let prefix: String = EQUAL.split_inclusive('\n').take(refused_row).collect();
        assert_eq!(sink.bytes, prefix.as_bytes());
    }
    Ok(())
}
