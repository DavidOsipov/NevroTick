// SPDX-License-Identifier: GPL-2.0-only
//! Private exhaustion, fault-reason, rejection-layout and finite-owner controls.

use std::mem::{align_of, size_of};
use std::panic::catch_unwind;
use std::time::{Duration, Instant};

use super::{
    Candidate, Component, ConsumerState, Counter, DecodeError, DecodedRecord,
    DiagnosticObservation, DiagnosticSnapshot, ImportanceMask, LifecycleObservationClaim,
    MemoryConsumer, MemoryRoutes, ObservationReason, OptionalProfile, PrimaryCause, QueueState,
    RouteHealth, Router, ServiceEvent, WriterRecord,
};
use garde::Validate;

/// Use the real router with the finite ordinary required-route composition.
fn router() -> Router {
    Router::new(
        Component::Engine,
        OptionalProfile::Off,
        ImportanceMask::all(),
        MemoryRoutes::available(),
    )
}

/// Controlled relative observation for sequential resource tests; no ambient clock.
fn at(elapsed_ns: u64) -> DiagnosticObservation {
    DiagnosticObservation {
        reason: ObservationReason::Accepted,
        elapsed_ns,
    }
}

/// Observe the accepted literal last sequence without unwrapping invariant assumptions.
fn sequences(snapshot: &DiagnosticSnapshot) -> Vec<u64> {
    let mut observed: Vec<u64> = Vec::new();
    // Observe one extra record to detect a broken iterator without depending
    // on its reported size or termination.
    for bytes in snapshot.required().history().records().take(17) {
        let mut word: [u8; 8] = [0; 8];
        word.copy_from_slice(&bytes[8..16]);
        observed.push(u64::from_le_bytes(word));
    }
    observed
}

/// One root rule yields one constant-code, depth-zero Report, then closed errors.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn root_garde_rule_has_one_bounded_entry_and_observed_layout() {
    let candidate: Candidate = Candidate { bytes: [0; 128] };
    let result: Result<(), garde::Report> = candidate.validate();
    assert!(result.is_err());
    if let Err(report) = result {
        let entries: Vec<(garde::Path, garde::Error)> = report.into_inner();
        assert_eq!(entries.len(), 1);
        assert!(entries[0].0.is_empty());
        assert_eq!(entries[0].1.message(), "invalid-v2");
        assert_eq!(entries[0].1.message().len(), 10);
        let entry_bytes: usize = size_of::<(garde::Path, garde::Error)>();
        let alignment: usize = align_of::<(garde::Path, garde::Error)>();
        let capacity: usize = entries.capacity();
        let requested: Option<usize> = entry_bytes.checked_mul(capacity);
        // Independent pinned x64 Garde/Vec layout derivation: 40-byte entry,
        // eight-byte alignment and four slots on the first Vec push.
        assert_eq!(entry_bytes, 40);
        assert_eq!(alignment, 8);
        assert_eq!(capacity, 4);
        assert_eq!(requested, Some(160));
        println!(
            "logger-root-report entry_bytes={entry_bytes} alignment={alignment} capacity={capacity} requested_bytes={requested:?}"
        );
    }
    assert!(matches!(
        Candidate::checked([0; 128]),
        Err(DecodeError::InvalidV2)
    ));
    let router_bytes: usize = size_of::<Router>();
    let snapshot_bytes: usize = size_of::<DiagnosticSnapshot>();
    println!(
        "logger-fixed-storage router_bytes={router_bytes} snapshot_bytes={snapshot_bytes} bounded_candidates_bytes=384"
    );
    // The retained receipt and one in-flight snapshot copy may coexist.
    assert!(router_bytes + snapshot_bytes * 2 + 128 * 3 <= 24_576);
}

/// The last legal sequence is emitted once; no wrapping or post-seal restart occurs.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn sequence_maximum_and_permanent_seal_preserve_originals() {
    let mut actual: Router = router();
    actual.next_sequence = Some(u64::MAX);
    actual.emit(ServiceEvent::Finished, DiagnosticObservation::unavailable());
    actual.emit(ServiceEvent::Finished, DiagnosticObservation::unavailable());
    let first: DiagnosticSnapshot = actual.finish();
    assert_eq!(sequences(&first), [u64::MAX]);
    assert_eq!(first.health().emitted().value(), 1);
    assert_eq!(first.health().sequence_loss().value(), 1);
    assert!(first.sealed());
    assert!(!first.history_complete());
    actual.emit(
        ServiceEvent::StartObserved,
        DiagnosticObservation::unavailable(),
    );
    let second: DiagnosticSnapshot = actual.finish();
    assert_eq!(sequences(&second), [u64::MAX]);
    assert_eq!(second.health().sealed_loss().value(), 1);
    assert_eq!(second.required().health().consumer_attempts().value(), 1);
}

/// Saturation is explicit rather than wraparound, including history completeness.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn counter_saturation_is_explicit_and_does_not_wrap() {
    let mut count: Counter = Counter {
        value: u64::MAX - 1,
        saturated: false,
    };
    count.increment();
    assert_eq!(count.value(), u64::MAX);
    assert!(!count.saturated());
    count.increment();
    assert_eq!(count.value(), u64::MAX);
    assert!(count.saturated());
    // Each global flag must affect overall completeness on its own, while the
    // required route remains healthy. Drive the real producer from MAX-1 so
    // the legal MAX neighbor cannot be mistaken for lost counter precision.
    for reason in [
        ObservationReason::NotObserved,
        ObservationReason::Accepted,
        ObservationReason::CheckedElapsedOverflow,
        ObservationReason::OriginOrderFault,
        ObservationReason::ActualClockFault,
    ] {
        for (attempts, saturated) in [(1_u8, false), (2, true)] {
            let mut actual: Router = router();
            let near_limit: Counter = Counter {
                value: u64::MAX - 1,
                saturated: false,
            };
            match reason {
                ObservationReason::NotObserved | ObservationReason::Accepted => {
                    actual.health.emitted = near_limit;
                }
                ObservationReason::CheckedElapsedOverflow => {
                    actual.health.elapsed_overflow = near_limit;
                }
                ObservationReason::OriginOrderFault => {
                    actual.health.origin_order_fault = near_limit;
                }
                ObservationReason::ActualClockFault => {
                    actual.health.actual_clock_fault = near_limit;
                }
            }
            for _ in 0..attempts {
                actual.emit(
                    ServiceEvent::StartObserved,
                    DiagnosticObservation {
                        reason,
                        elapsed_ns: 0,
                    },
                );
            }
            let snapshot: DiagnosticSnapshot = actual.finish();
            let observed: Counter = match reason {
                ObservationReason::NotObserved | ObservationReason::Accepted => {
                    snapshot.health().emitted()
                }
                ObservationReason::CheckedElapsedOverflow => snapshot.health().elapsed_overflow(),
                ObservationReason::OriginOrderFault => snapshot.health().origin_order_fault(),
                ObservationReason::ActualClockFault => snapshot.health().actual_clock_fault(),
            };
            assert_eq!(observed.value(), u64::MAX);
            assert_eq!(observed.saturated(), saturated);
            assert_eq!(
                snapshot.required().health().accepted().value(),
                u64::from(attempts)
            );
            assert!(snapshot.required().history_complete());
            assert_eq!(snapshot.history_complete(), !saturated);
        }
    }
    let mut actual: Router = router();
    actual.required.health.accepted = count;
    let snapshot: DiagnosticSnapshot = actual.finish();
    assert!(!snapshot.required().history_complete());
    assert!(!snapshot.history_complete());
    let mut health: RouteHealth = RouteHealth::EMPTY;
    health.events[0].suppressed = count;
    assert!(health.saturated());
}

/// Real checked origin/order/overflow projection preserves contract10 fault reasons.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn observation_projection_checks_conversion_before_actual_fault() {
    let origin: Instant = Instant::now();
    let equal: DiagnosticObservation = DiagnosticObservation::from_lifecycle(origin, origin, false);
    assert_eq!(equal.reason, ObservationReason::Accepted);
    assert_eq!(equal.elapsed_ns, 0);
    let historical: DiagnosticObservation =
        DiagnosticObservation::from_lifecycle(origin, origin, true);
    assert_eq!(historical.reason, ObservationReason::ActualClockFault);
    assert_eq!(historical.wire_kind(), 2);
    let before: Option<Instant> = origin.checked_sub(Duration::from_nanos(1));
    assert!(before.is_some());
    if let Some(before) = before {
        for faulted in [false, true] {
            let order: DiagnosticObservation =
                DiagnosticObservation::from_lifecycle(origin, before, faulted);
            assert_eq!(order.reason, ObservationReason::OriginOrderFault);
            assert_eq!(order.wire_kind(), 0);
        }
    }
    let future: Option<Instant> = origin.checked_add(Duration::new(18_446_744_073, 709_551_616));
    assert!(future.is_some());
    if let Some(future) = future {
        for faulted in [false, true] {
            let overflow: DiagnosticObservation =
                DiagnosticObservation::from_lifecycle(origin, future, faulted);
            assert_eq!(overflow.reason, ObservationReason::CheckedElapsedOverflow);
            assert_eq!(overflow.wire_kind(), 0);
        }
    }
}

/// Every internal fault reason latches refill refusal despite passive unavailable time.
/// Repeated cached fault facts count observations, never distinct physical incidents.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn internal_fault_reasons_survive_wire_unavailable_and_never_refill() {
    for reason in [
        ObservationReason::CheckedElapsedOverflow,
        ObservationReason::OriginOrderFault,
        ObservationReason::ActualClockFault,
    ] {
        let mut actual: Router = router();
        let observation: DiagnosticObservation = DiagnosticObservation {
            reason,
            elapsed_ns: 0,
        };
        for _ in 0..2 {
            actual.emit(ServiceEvent::StartObserved, observation);
            actual.pump_one();
        }
        for coordinate in [0, 1_000_000_000, 2_000_000_000] {
            actual.emit(ServiceEvent::StartObserved, at(coordinate));
            actual.pump_one();
        }
        let snapshot: DiagnosticSnapshot = actual.finish();
        assert!(snapshot.health().rate_clock_fault());
        assert_eq!(snapshot.required().health().accepted().value(), 4);
        assert_eq!(snapshot.required().health().suppressed().value(), 1);
        assert_eq!(
            snapshot.health().elapsed_overflow().value(),
            u64::from(reason == ObservationReason::CheckedElapsedOverflow) * 2
        );
        assert_eq!(
            snapshot.health().origin_order_fault().value(),
            u64::from(reason == ObservationReason::OriginOrderFault) * 2
        );
        assert_eq!(
            snapshot.health().actual_clock_fault().value(),
            u64::from(reason == ObservationReason::ActualClockFault) * 2
        );
        if let Some(bytes) = snapshot.required().history().get(0) {
            let decoded: Result<DecodedRecord, DecodeError> = DecodedRecord::decode(bytes);
            assert!(decoded.is_ok());
            if let Ok(decoded) = decoded {
                assert_eq!(
                    decoded.observation(),
                    if reason == ObservationReason::ActualClockFault {
                        LifecycleObservationClaim::HistoricalAfterClockFault(0)
                    } else {
                        LifecycleObservationClaim::Unavailable
                    }
                );
            }
        }
    }
}

/// Missing time retains four useful credits; first observed anchor never refunds them.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn unavailable_initial_budget_and_first_anchor_do_not_manufacture_credit() {
    let mut actual: Router = router();
    for _ in 0..4 {
        actual.emit(
            ServiceEvent::StartObserved,
            DiagnosticObservation::unavailable(),
        );
        actual.pump_one();
    }
    actual.emit(ServiceEvent::StartObserved, at(20_000_000_000));
    let snapshot: DiagnosticSnapshot = actual.snapshot();
    assert!(!snapshot.health().rate_clock_fault());
    assert_eq!(snapshot.required().health().accepted().value(), 4);
    assert_eq!(snapshot.required().health().suppressed().value(), 1);
    assert_eq!(actual.window_anchor, Some(20_000_000_000));
    actual.emit(ServiceEvent::Finished, DiagnosticObservation::unavailable());
    let final_view: DiagnosticSnapshot = actual.finish();
    assert_eq!(final_view.required().health().accepted().value(), 5);
}

/// One-second equality refills once, without equal-time or multi-window windfalls.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn refill_has_exact_boundaries_and_no_carryover() {
    let mut actual: Router = router();
    for _ in 0..4 {
        actual.emit(ServiceEvent::Finished, at(0));
        actual.pump_one();
    }
    actual.emit(ServiceEvent::Finished, at(999_999_999));
    actual.emit(ServiceEvent::Finished, at(1_000_000_000));
    actual.pump_one();
    for _ in 0..4 {
        actual.emit(ServiceEvent::Finished, at(1_000_000_000));
        actual.pump_one();
    }
    let middle: DiagnosticSnapshot = actual.snapshot();
    assert_eq!(middle.required().health().accepted().value(), 8);
    assert_eq!(middle.required().health().suppressed().value(), 2);
    for _ in 0..5 {
        actual.emit(ServiceEvent::Finished, at(100_000_000_000));
        actual.pump_one();
    }
    let final_view: DiagnosticSnapshot = actual.finish();
    assert_eq!(final_view.required().health().accepted().value(), 12);
    assert_eq!(final_view.required().health().suppressed().value(), 3);
}

/// Decrease above the current window anchor still faults and cannot later refill.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn nondecreasing_observations_compare_the_last_coordinate_not_only_anchor() {
    let mut actual: Router = router();
    for coordinate in [0, 200, 100, 1_000_000_000, 2_000_000_000] {
        actual.emit(ServiceEvent::StartObserved, at(coordinate));
        actual.pump_one();
    }
    let snapshot: DiagnosticSnapshot = actual.finish();
    assert!(snapshot.health().rate_clock_fault());
    assert_eq!(snapshot.health().origin_order_fault().value(), 1);
    assert_eq!(snapshot.required().health().accepted().value(), 4);
    assert_eq!(snapshot.required().health().suppressed().value(), 1);
}

/// A full pending queue spends credits without overwrite; Full consumer is separate.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn pending_capacity_and_reached_full_consumer_have_distinct_loss() {
    // The live queue, codec and recent history must handle valid emissions
    // without panicking. Check the healthy boundary and the first overwrite.
    let healthy: std::thread::Result<()> = catch_unwind(|| {
        let mut actual: Router = router();
        actual.pump_one();
        for second in 0..17_u64 {
            actual.emit(ServiceEvent::Finished, at(second * 1_000_000_000));
            actual.pump_one();
        }
        let snapshot: DiagnosticSnapshot = actual.finish();
        assert_eq!(snapshot.required().health().accepted().value(), 17);
        assert_eq!(
            snapshot.required().health().history_overwritten().value(),
            1
        );
        assert_eq!(
            sequences(&snapshot),
            [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,]
        );
        let mut records = snapshot.required().history().records();
        for remaining in (1..=16).rev() {
            assert_eq!(records.size_hint(), (remaining, Some(remaining)));
            assert!(records.next().is_some());
        }
        assert_eq!(records.size_hint(), (0, Some(0)));
        assert!(records.next().is_none());
    });
    assert!(healthy.is_ok());
    let routes: MemoryRoutes = MemoryRoutes::new(
        MemoryConsumer::new(ConsumerState::Full),
        MemoryConsumer::new(ConsumerState::Available),
    );
    let mut actual: Router = Router::new(
        Component::Engine,
        OptionalProfile::Off,
        ImportanceMask::all(),
        routes,
    );
    for event in [
        ServiceEvent::StartObserved,
        ServiceEvent::RunningAcknowledged,
        ServiceEvent::AdmissionClosed,
        ServiceEvent::Finished,
        ServiceEvent::PrimaryFailure(PrimaryCause::Native),
    ] {
        for _ in 0..4 {
            actual.emit(event, DiagnosticObservation::unavailable());
        }
    }
    let pending: DiagnosticSnapshot = actual.snapshot();
    assert_eq!(pending.required().pending(), 16);
    assert_eq!(pending.required().health().queued().value(), 16);
    assert_eq!(pending.required().health().queue_full().value(), 4);
    assert_eq!(pending.required().health().consumer_attempts().value(), 0);
    assert!(!pending.required().history_complete());
    let snapshot: DiagnosticSnapshot = actual.finish();
    assert_eq!(snapshot.required().pending(), 0);
    assert_eq!(snapshot.required().health().consumer_attempts().value(), 16);
    assert_eq!(snapshot.required().health().consumer_full().value(), 16);
    assert_eq!(snapshot.required().history().len(), 0);
    assert_eq!(
        snapshot.required().health().history_overwritten().value(),
        0
    );
    assert_eq!(snapshot.required().health().queue_full().value(), 4);
}

/// Eligible originals retain their fields with optional routing Off.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn writer_fields_come_from_original_required_admission() {
    let mut actual: Router = Router::new(
        Component::ClockBroker,
        OptionalProfile::Off,
        ImportanceMask::all(),
        MemoryRoutes::available(),
    );
    assert!(
        actual
            .emit_for_writer(ServiceEvent::StartObserved, at(100))
            .is_none()
    );
    let running: Option<WriterRecord> =
        actual.emit_for_writer(ServiceEvent::RunningAcknowledged, at(200));
    assert!(running.is_some());
    if let Some(record) = running {
        assert_eq!(record.component(), Component::ClockBroker);
        assert_eq!(record.sequence(), 2);
        assert_eq!(
            record.observation(),
            LifecycleObservationClaim::LastAccepted(200)
        );
        assert_eq!(record.event_id(), 2);
        assert_eq!(record.cause_code(), 0);
    }
    let historical: DiagnosticObservation = DiagnosticObservation {
        reason: ObservationReason::ActualClockFault,
        elapsed_ns: 230,
    };
    let failure: Option<WriterRecord> = actual.emit_for_writer(
        ServiceEvent::PrimaryFailure(PrimaryCause::LifecycleReport),
        historical,
    );
    assert!(failure.is_some());
    if let Some(record) = failure {
        assert_eq!(record.component(), Component::ClockBroker);
        assert_eq!(record.sequence(), 3);
        assert_eq!(
            record.observation(),
            LifecycleObservationClaim::HistoricalAfterClockFault(230)
        );
        assert_eq!(record.event_id(), 4);
        assert_eq!(record.cause_code(), 10);
    }
    let pending: DiagnosticSnapshot = actual.snapshot();
    assert_eq!(pending.required().pending(), 3);
    assert_eq!(pending.required().health().consumer_attempts().value(), 0);
    assert_eq!(pending.optional().health().filtered().value(), 3);
    let finished: DiagnosticSnapshot = actual.finish();
    assert_eq!(sequences(&finished), [1, 2, 3]);
    // Inspect the retained wire values without optional-mask arithmetic.
    assert_eq!(
        finished.required().history().get(1).map(|bytes| bytes[5]),
        Some(5)
    );
    assert_eq!(
        finished.required().history().get(2).map(|bytes| bytes[5]),
        Some(2)
    );
    assert_eq!(finished.optional().pending(), 0);
    assert!(finished.optional().history().is_empty());
}

/// Optional admission supplies no writer record when the required queue is absent.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn writer_ignores_optional_admission_after_required_queue_refusal() {
    let routes: MemoryRoutes =
        MemoryRoutes::available().with_queue_states(QueueState::Unavailable, QueueState::Available);
    let mut actual: Router = Router::new(
        Component::Engine,
        OptionalProfile::Info,
        ImportanceMask::all(),
        routes,
    );
    assert!(
        actual
            .emit_for_writer(ServiceEvent::RunningAcknowledged, at(0))
            .is_none()
    );
    let pending: DiagnosticSnapshot = actual.snapshot();
    assert_eq!(pending.required().health().queue_unavailable().value(), 1);
    assert_eq!(pending.required().pending(), 0);
    assert_eq!(pending.optional().pending(), 1);
    let finished: DiagnosticSnapshot = actual.finish();
    assert_eq!(finished.optional().history().len(), 1);
}

/// Writer admission follows queue capacity, its spent credits and the producer seal.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn writer_distinguishes_queue_refusal_from_later_consumer_refusal() {
    let routes: MemoryRoutes = MemoryRoutes::new(
        MemoryConsumer::new(ConsumerState::Full),
        MemoryConsumer::new(ConsumerState::Available),
    );
    let mut actual: Router = Router::new(
        Component::Engine,
        OptionalProfile::Off,
        ImportanceMask::all(),
        routes,
    );
    for event in [
        ServiceEvent::StartObserved,
        ServiceEvent::RunningAcknowledged,
        ServiceEvent::AdmissionClosed,
        ServiceEvent::Finished,
    ] {
        for _ in 0..4 {
            actual.emit(event, at(0));
        }
    }
    let failure: ServiceEvent = ServiceEvent::PrimaryFailure(PrimaryCause::Native);
    assert!(actual.emit_for_writer(failure, at(0)).is_none());
    for sequence in [18_u64, 19, 20] {
        actual.pump_one();
        let record: Option<WriterRecord> = actual.emit_for_writer(failure, at(0));
        assert_eq!(record.as_ref().map(WriterRecord::sequence), Some(sequence));
    }
    actual.pump_one();
    assert!(actual.emit_for_writer(failure, at(0)).is_none());
    let finished: DiagnosticSnapshot = actual.finish();
    assert_eq!(finished.required().health().queue_full().value(), 1);
    assert_eq!(finished.required().health().suppressed().value(), 1);
    assert_eq!(finished.required().health().consumer_full().value(), 19);
    assert!(finished.required().history().is_empty());
    assert!(actual.emit_for_writer(failure, at(1_000_000_000)).is_none());
    assert_eq!(actual.snapshot().health().sealed_loss().value(), 1);
}
