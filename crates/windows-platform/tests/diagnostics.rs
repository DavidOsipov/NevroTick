// SPDX-License-Identifier: GPL-2.0-only
//! Independent public-producer, literal decoder, policy and bounded-history controls.

use std::time::{Duration, Instant};

use windows_platform::diagnostics::{
    Component, ConsumerState, DecodeError, DecodedRecord, DiagnosticObservation,
    DiagnosticSnapshot, Importance, ImportanceMask, LifecycleCause, LifecycleObservationClaim,
    MemoryConsumer, MemoryRoutes, MetadataClaim, OptionalProfile, PrimaryCause, QueueState,
    RECORD_BYTES, Router, ServiceEvent, TechnicalLevel, UtcClaim, WorkerCause, optional_enabled,
};

/// Independent literal v2 `StartObserved` record; never calls the production encoder.
fn literal() -> [u8; 128] {
    let mut bytes: [u8; 128] = [0; 128];
    bytes[..9].copy_from_slice(&[2, 1, 1, 1, 3, 5, 1, 0, 1]);
    bytes
}

/// Construct real routes while preserving each concrete consumer outcome.
fn router(required: ConsumerState, optional: ConsumerState, profile: OptionalProfile) -> Router {
    Router::new(
        Component::Engine,
        profile,
        ImportanceMask::all(),
        MemoryRoutes::new(MemoryConsumer::new(required), MemoryConsumer::new(optional)),
    )
}

/// Decode one expected passive result and assert its independent literal fields.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
fn assert_start(bytes: &[u8; RECORD_BYTES], sequence: u64) {
    let decoded: Result<DecodedRecord, DecodeError> = DecodedRecord::decode(bytes);
    assert!(decoded.is_ok());
    if let Ok(decoded) = decoded {
        assert_eq!(decoded.component().code(), 1);
        assert_eq!(decoded.event().id(), 1);
        assert_eq!(decoded.event().cause_code(), 0);
        assert_eq!(decoded.sequence(), sequence);
        assert_eq!(
            decoded.observation(),
            LifecycleObservationClaim::Unavailable
        );
        assert_eq!(decoded.process_instance(), MetadataClaim::Unavailable);
        assert_eq!(decoded.policy_generation(), MetadataClaim::Unavailable);
        assert_eq!(
            decoded.configuration_generation(),
            MetadataClaim::Unavailable
        );
        assert_eq!(decoded.association_generation(), MetadataClaim::Unavailable);
        assert_eq!(decoded.clock_generation(), MetadataClaim::Unavailable);
        assert_eq!(decoded.correlation(), MetadataClaim::Unavailable);
        assert_eq!(decoded.utc(), UtcClaim::Unavailable);
        assert_eq!(decoded.bytes(), bytes);
    }
}

/// Required INFO reaches actual consumption with all optional output off.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn required_info_off_has_literal_original_fields_and_real_consumer_acceptance() {
    let mut actual: Router = router(
        ConsumerState::Available,
        ConsumerState::Reject,
        OptionalProfile::Off,
    );
    actual.emit(
        ServiceEvent::StartObserved,
        DiagnosticObservation::unavailable(),
    );
    let queued: DiagnosticSnapshot = actual.snapshot();
    assert_eq!(queued.required().history().len(), 0);
    assert_eq!(queued.required().pending(), 1);
    assert!(!queued.history_complete());
    actual.pump_one();
    let snapshot: DiagnosticSnapshot = actual.finish();
    assert_eq!(snapshot.required().history().get(0), Some(&literal()));
    assert_start(&literal(), 1);
    assert!(snapshot.required().history_complete());
    assert!(snapshot.history_complete());
    assert!(!snapshot.native_delivery_qualified());
    assert_eq!(snapshot.required().health().queued().value(), 1);
    assert_eq!(snapshot.required().health().consumer_attempts().value(), 1);
    assert_eq!(snapshot.required().health().accepted().value(), 1);
    assert_eq!(snapshot.optional().health().filtered().value(), 1);
    assert_eq!(snapshot.optional().health().queued().value(), 0);
    assert_eq!(snapshot.optional().health().consumer_attempts().value(), 0);
    assert!(snapshot.optional().history().is_empty());
    assert_eq!(snapshot.required().history().records().len(), 1);
    assert_eq!(snapshot.required().history().get(1), None);
}

/// Exact empty/version/length precedence preserves the original historical v1 bytes.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn decoder_versions_and_complete_length_have_no_fallback_or_partial_acceptance() {
    let historical_v1: [u8; 9] = [1, 1, 1, 1, 1, 0, 0, 0, 0];
    assert_eq!(historical_v1, [1, 1, 1, 1, 1, 0, 0, 0, 0]);
    assert_eq!(
        DecodedRecord::decode(&historical_v1),
        Err(DecodeError::UnsupportedVersion(1))
    );
    assert_eq!(DecodedRecord::decode(&[]), Err(DecodeError::Empty));
    for version in [0u8, 1, 3, 255] {
        assert_eq!(
            DecodedRecord::decode(&[version]),
            Err(DecodeError::UnsupportedVersion(version))
        );
    }
    let bytes: [u8; 128] = literal();
    for len in 1..128 {
        assert_eq!(
            DecodedRecord::decode(&bytes[..len]),
            Err(DecodeError::InvalidLength { actual: len })
        );
    }
    let mut extra: [u8; 129] = [0; 129];
    extra[..128].copy_from_slice(&bytes);
    assert_eq!(
        DecodedRecord::decode(&extra),
        Err(DecodeError::InvalidLength { actual: 129 })
    );
    assert!(DecodedRecord::decode(&bytes).is_ok());
}

/// Single semantic mutations reach the real root constructor beside a healthy literal.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn decoder_rejects_classification_tags_canonical_payloads_and_reserved_bytes() {
    let cases: [(usize, u8); 30] = [
        (1, 0),
        (1, 2),
        (2, 0),
        (2, 3),
        (3, 0),
        (3, 10),
        (4, 1),
        (4, 2),
        (4, 4),
        (4, 5),
        (5, 1),
        (5, 2),
        (5, 6),
        (6, 0),
        (6, 2),
        (7, 1),
        (16, 3),
        (17, 1),
        (25, 2),
        (26, 1),
        (42, 2),
        (43, 1),
        (51, 2),
        (52, 1),
        (60, 2),
        (61, 1),
        (69, 2),
        (70, 1),
        (78, 2),
        (79, 1),
    ];
    for (offset, value) in cases {
        let mut bytes: [u8; 128] = literal();
        bytes[offset] = value;
        assert_eq!(
            DecodedRecord::decode(&bytes),
            Err(DecodeError::InvalidV2),
            "offset {offset} value {value}"
        );
    }
    for offset in 108..128 {
        let mut bytes: [u8; 128] = literal();
        bytes[offset] = 1;
        assert_eq!(DecodedRecord::decode(&bytes), Err(DecodeError::InvalidV2));
    }
    for tag in [25usize, 42, 51, 60, 69, 78] {
        let mut bytes: [u8; 128] = literal();
        bytes[tag] = 1;
        assert_eq!(DecodedRecord::decode(&bytes), Err(DecodeError::InvalidV2));
    }
    let mut zero_sequence: [u8; 128] = literal();
    zero_sequence[8] = 0;
    assert_eq!(
        DecodedRecord::decode(&zero_sequence),
        Err(DecodeError::InvalidV2)
    );
    assert!(DecodedRecord::decode(&literal()).is_ok());
}

/// Passive nonzero metadata and pre-epoch UTC remain untrusted claims, never issuers.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn passive_claims_preserve_exact_integer_and_utc_boundary_meaning() {
    let mut bytes: [u8; 128] = literal();
    bytes[2] = 2;
    bytes[8..16].copy_from_slice(&u64::MAX.to_le_bytes());
    bytes[16] = 2;
    bytes[25] = 1;
    bytes[26..42].copy_from_slice(&u128::MAX.to_le_bytes());
    for (tag, offset) in [(42usize, 43usize), (51, 52), (60, 61), (69, 70)] {
        bytes[tag] = 1;
        bytes[offset..offset + 8].copy_from_slice(&u64::MAX.to_le_bytes());
    }
    bytes[78] = 1;
    bytes[79..95].copy_from_slice(&1u128.to_le_bytes());
    bytes[95] = 1;
    bytes[96..104].copy_from_slice(&(-1i64).to_le_bytes());
    bytes[104..108].copy_from_slice(&999_999_999u32.to_le_bytes());
    let decoded: Result<DecodedRecord, DecodeError> = DecodedRecord::decode(&bytes);
    assert!(decoded.is_ok());
    if let Ok(decoded) = decoded {
        assert_eq!(decoded.component().code(), 2);
        assert_eq!(decoded.sequence(), u64::MAX);
        assert_eq!(
            decoded.observation(),
            LifecycleObservationClaim::HistoricalAfterClockFault(0)
        );
        assert_eq!(
            decoded.process_instance(),
            MetadataClaim::ObservedClaim(u128::MAX)
        );
        assert_eq!(
            decoded.policy_generation(),
            MetadataClaim::ObservedClaim(u64::MAX)
        );
        assert_eq!(
            decoded.configuration_generation(),
            MetadataClaim::ObservedClaim(u64::MAX)
        );
        assert_eq!(
            decoded.association_generation(),
            MetadataClaim::ObservedClaim(u64::MAX)
        );
        assert_eq!(
            decoded.clock_generation(),
            MetadataClaim::ObservedClaim(u64::MAX)
        );
        assert_eq!(decoded.correlation(), MetadataClaim::ObservedClaim(1));
        assert_eq!(
            decoded.utc(),
            UtcClaim::SystemWallUnqualified {
                unix_seconds: -1,
                nanoseconds: 999_999_999
            }
        );
    }
    bytes[104..108].copy_from_slice(&1_000_000_000u32.to_le_bytes());
    assert_eq!(DecodedRecord::decode(&bytes), Err(DecodeError::InvalidV2));
    bytes[104..108].fill(0);
    bytes[95] = 0;
    assert_eq!(DecodedRecord::decode(&bytes), Err(DecodeError::InvalidV2));
    bytes[96..104].fill(0);
    assert!(DecodedRecord::decode(&bytes).is_ok());
    bytes[95] = 2;
    assert_eq!(DecodedRecord::decode(&bytes), Err(DecodeError::InvalidV2));
}

/// Every raw cause is admitted by event-specific closed conversion, never a byte wrapper.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn raw_cause_domain_properties_cover_all_256_values_and_event_neighbours() {
    for code in 0u8..=255 {
        let primary: Result<PrimaryCause, DecodeError> = PrimaryCause::try_from(code);
        let lifecycle: Result<LifecycleCause, DecodeError> = LifecycleCause::try_from(code);
        let worker: Result<WorkerCause, DecodeError> = WorkerCause::try_from(code);
        assert_eq!(primary.is_ok(), (1..=28).contains(&code));
        assert_eq!(lifecycle.is_ok(), (10..=19).contains(&code));
        assert_eq!(worker.is_ok(), (20..=23).contains(&code));
        if let Ok(cause) = primary {
            assert_eq!(cause.code(), code);
        }
        if let Ok(cause) = lifecycle {
            assert_eq!(cause.code(), code);
        }
        if let Ok(cause) = worker {
            assert_eq!(cause.code(), code);
        }
    }
    for (id, low, high) in [
        (4u8, 1u8, 28u8),
        (5, 20, 23),
        (6, 10, 19),
        (7, 10, 19),
        (9, 20, 23),
    ] {
        for cause in 0u8..=29 {
            let mut bytes: [u8; 128] = literal();
            bytes[3] = id;
            bytes[4] = 1;
            bytes[5] = 2;
            bytes[7] = cause;
            assert_eq!(
                DecodedRecord::decode(&bytes).is_ok(),
                cause >= low && cause <= high,
                "id {id}, cause {cause}"
            );
        }
    }
}

/// Code-owned nine-event classes/IDs reach real routes; no synthetic live events exist.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn all_live_events_have_independent_literal_classes_and_exact_causes() {
    let events: [(ServiceEvent, u8, u8); 9] = [
        (ServiceEvent::StartObserved, 1, 0),
        (ServiceEvent::RunningAcknowledged, 2, 0),
        (ServiceEvent::AdmissionClosed, 3, 0),
        (
            ServiceEvent::PrimaryFailure(PrimaryCause::NestedCleanupNotTraversed),
            4,
            28,
        ),
        (ServiceEvent::CompletionFailure(WorkerCause::Cleanup), 5, 23),
        (
            ServiceEvent::StopReportFailure(LifecycleCause::Report),
            6,
            10,
        ),
        (
            ServiceEvent::TerminalReportFailure(LifecycleCause::FailureOutcomeRequired),
            7,
            19,
        ),
        (ServiceEvent::Finished, 8, 0),
        (ServiceEvent::OperationFailure(WorkerCause::Running), 9, 22),
    ];
    let mut actual: Router = router(
        ConsumerState::Available,
        ConsumerState::Available,
        OptionalProfile::Info,
    );
    for (event, id, cause) in events {
        assert_eq!(event.id(), id);
        assert_eq!(event.cause_code(), cause);
        assert_eq!(event.descriptor().id(), id);
        assert_eq!(event.descriptor().version(), 2);
        assert!(event.descriptor().required());
        assert!(event.descriptor().optional_duplicate_eligible());
        assert!(event.descriptor().permits_cause(cause));
        assert_eq!(
            event.descriptor().level(),
            if cause == 0 {
                TechnicalLevel::Info
            } else {
                TechnicalLevel::Error
            }
        );
        assert_eq!(
            event.descriptor().importance(),
            if cause == 0 {
                Importance::Info
            } else {
                Importance::High
            }
        );
        actual.emit(event, DiagnosticObservation::unavailable());
        actual.pump_one();
    }
    let snapshot: DiagnosticSnapshot = actual.finish();
    assert_eq!(snapshot.required().history().len(), 9);
    assert_eq!(snapshot.optional().history().len(), 9);
    for (ordinal, (_, id, cause)) in events.into_iter().enumerate() {
        let bytes: Option<&[u8; 128]> = snapshot.required().history().get(ordinal);
        assert!(bytes.is_some());
        if let Some(bytes) = bytes {
            assert_eq!(
                &bytes[..8],
                &[
                    2,
                    1,
                    1,
                    id,
                    if cause == 0 { 3 } else { 1 },
                    if cause == 0 { 5 } else { 2 },
                    1,
                    cause
                ]
            );
            let decoded: Result<DecodedRecord, DecodeError> = DecodedRecord::decode(bytes);
            assert!(decoded.is_ok());
            if let Ok(decoded) = decoded {
                assert_eq!(decoded.event().id(), id);
                assert_eq!(decoded.event().cause_code(), cause);
                assert_eq!(decoded.event().descriptor().id(), id);
            }
        }
    }
}

/// Synthetic 5x5 policy controls prove technical levels and importance are independent.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn optional_profile_by_importance_matrix_and_checked_mask_are_independent() {
    let levels: [TechnicalLevel; 5] = [
        TechnicalLevel::Error,
        TechnicalLevel::Warn,
        TechnicalLevel::Info,
        TechnicalLevel::Debug,
        TechnicalLevel::Trace,
    ];
    let bands: [Importance; 5] = [
        Importance::Critical,
        Importance::High,
        Importance::Medium,
        Importance::Low,
        Importance::Info,
    ];
    let profiles: [(OptionalProfile, [bool; 5]); 4] = [
        (OptionalProfile::Off, [false, false, false, false, false]),
        (OptionalProfile::Error, [true, false, false, false, false]),
        (OptionalProfile::Warn, [true, true, false, false, false]),
        (OptionalProfile::Info, [true, true, true, false, false]),
    ];
    for bits in 0u8..=255 {
        let mask: Result<ImportanceMask, DecodeError> = ImportanceMask::new(bits);
        assert_eq!(mask.is_ok(), bits <= 31);
        if let Ok(mask) = mask {
            assert_eq!(mask.bits(), bits);
            for (profile, admitted) in profiles {
                for (level_index, level) in levels.into_iter().enumerate() {
                    for (band_index, band) in bands.into_iter().enumerate() {
                        let selected: bool = bits & (1u8 << band_index) != 0;
                        assert_eq!(
                            optional_enabled(profile, mask, level, band),
                            admitted[level_index] && selected
                        );
                    }
                }
            }
        }
    }
}

/// Queue absence, reached rejection/fullness and consumer acceptance remain separate.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn concrete_consumer_failures_preserve_required_route_and_explicit_health() {
    for state in [
        ConsumerState::Available,
        ConsumerState::Full,
        ConsumerState::Reject,
        ConsumerState::Unavailable,
    ] {
        let mut actual: Router = router(state, ConsumerState::Available, OptionalProfile::Info);
        actual.emit(
            ServiceEvent::StartObserved,
            DiagnosticObservation::unavailable(),
        );
        let snapshot: DiagnosticSnapshot = actual.finish();
        let accepted: bool = state == ConsumerState::Available;
        assert_eq!(snapshot.required().health().queued().value(), 1);
        assert_eq!(snapshot.required().health().consumer_attempts().value(), 1);
        assert_eq!(
            snapshot.required().health().accepted().value(),
            u64::from(accepted)
        );
        assert_eq!(
            snapshot.required().health().consumer_full().value(),
            u64::from(state == ConsumerState::Full)
        );
        assert_eq!(
            snapshot.required().health().consumer_rejected().value(),
            u64::from(state == ConsumerState::Reject)
        );
        assert_eq!(
            snapshot.required().health().consumer_unavailable().value(),
            u64::from(state == ConsumerState::Unavailable)
        );
        assert_eq!(snapshot.required().history().len(), usize::from(accepted));
        assert_eq!(snapshot.required().history_complete(), accepted);
        assert_eq!(snapshot.history_complete(), accepted);
        assert_eq!(snapshot.optional().health().accepted().value(), 1);
        assert_eq!(snapshot.optional().history().get(0), Some(&literal()));
    }
    for state in [ConsumerState::Available, ConsumerState::Reject] {
        let mut actual: Router = router(ConsumerState::Available, state, OptionalProfile::Info);
        actual.emit(
            ServiceEvent::StartObserved,
            DiagnosticObservation::unavailable(),
        );
        let snapshot: DiagnosticSnapshot = actual.finish();
        let accepted: bool = state == ConsumerState::Available;
        assert_eq!(snapshot.required().history().get(0), Some(&literal()));
        assert!(snapshot.required().history_complete());
        assert_eq!(snapshot.optional().health().consumer_attempts().value(), 1);
        assert_eq!(
            snapshot.optional().health().consumer_rejected().value(),
            u64::from(!accepted)
        );
        assert_eq!(snapshot.optional().history_complete(), accepted);
        assert_eq!(snapshot.history_complete(), accepted);
    }
    let routes: MemoryRoutes =
        MemoryRoutes::available().with_queue_states(QueueState::Unavailable, QueueState::Available);
    let mut absent: Router = Router::new(
        Component::Engine,
        OptionalProfile::Info,
        ImportanceMask::all(),
        routes,
    );
    for _ in 0..5 {
        absent.emit(
            ServiceEvent::StartObserved,
            DiagnosticObservation::unavailable(),
        );
    }
    let snapshot: DiagnosticSnapshot = absent.finish();
    assert_eq!(snapshot.required().health().queue_unavailable().value(), 4);
    assert_eq!(snapshot.required().health().suppressed().value(), 1);
    assert_eq!(snapshot.required().health().consumer_attempts().value(), 0);
    assert_eq!(snapshot.optional().health().accepted().value(), 4);
}

/// Accepted records retain original coordinates across delay; restart has local ordering.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn original_observation_is_frozen_before_delayed_consumption_and_restart() {
    let origin: Instant = Instant::now();
    let last: Option<Instant> = origin.checked_add(Duration::from_nanos(1234));
    assert!(last.is_some());
    if let Some(last) = last {
        let mut actual: Router = router(
            ConsumerState::Available,
            ConsumerState::Available,
            OptionalProfile::Off,
        );
        actual.emit(
            ServiceEvent::StartObserved,
            DiagnosticObservation::from_lifecycle(origin, last, false),
        );
        actual.emit(
            ServiceEvent::Finished,
            DiagnosticObservation::from_lifecycle(origin, last, true),
        );
        let snapshot: DiagnosticSnapshot = actual.finish();
        for (index, expected) in [
            (0usize, LifecycleObservationClaim::LastAccepted(1234)),
            (
                1,
                LifecycleObservationClaim::HistoricalAfterClockFault(1234),
            ),
        ] {
            let bytes: Option<&[u8; 128]> = snapshot.required().history().get(index);
            assert!(bytes.is_some());
            if let Some(bytes) = bytes {
                let decoded: Result<DecodedRecord, DecodeError> = DecodedRecord::decode(bytes);
                assert!(decoded.is_ok());
                if let Ok(decoded) = decoded {
                    assert_eq!(decoded.observation(), expected);
                }
            }
        }
        assert_eq!(snapshot.health().actual_clock_fault().value(), 1);
    }
    for _ in 0..2 {
        let mut restarted: Router = router(
            ConsumerState::Available,
            ConsumerState::Available,
            OptionalProfile::Off,
        );
        restarted.emit(
            ServiceEvent::StartObserved,
            DiagnosticObservation::unavailable(),
        );
        let snapshot: DiagnosticSnapshot = restarted.finish();
        assert_eq!(snapshot.required().history().get(0), Some(&literal()));
    }
}

/// Reserved first-ID credits survive repeated floods; recent overwrite is explicit.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn aggregate_model_preserves_first_id_reservations_and_latest_sixteen_originals() {
    let events: [ServiceEvent; 9] = [
        ServiceEvent::StartObserved,
        ServiceEvent::RunningAcknowledged,
        ServiceEvent::AdmissionClosed,
        ServiceEvent::PrimaryFailure(PrimaryCause::Native),
        ServiceEvent::CompletionFailure(WorkerCause::Cleanup),
        ServiceEvent::StopReportFailure(LifecycleCause::Report),
        ServiceEvent::TerminalReportFailure(LifecycleCause::Report),
        ServiceEvent::Finished,
        ServiceEvent::OperationFailure(WorkerCause::Preparation),
    ];
    let mut actual: Router = router(
        ConsumerState::Available,
        ConsumerState::Available,
        OptionalProfile::Info,
    );
    for _ in 0..64 {
        actual.emit(
            ServiceEvent::StartObserved,
            DiagnosticObservation::unavailable(),
        );
        actual.pump_one();
    }
    let first: DiagnosticSnapshot = actual.snapshot();
    assert_eq!(first.required().health().accepted().value(), 4);
    assert_eq!(first.required().health().suppressed().value(), 60);
    for event in events.into_iter().skip(1) {
        actual.emit(event, DiagnosticObservation::unavailable());
        actual.pump_one();
    }
    let reserved: DiagnosticSnapshot = actual.snapshot();
    assert_eq!(reserved.required().health().accepted().value(), 12);
    assert_eq!(
        reserved
            .required()
            .health()
            .per_event(1)
            .map(|event| event.suppressed().value()),
        Some(60)
    );
    assert_eq!(reserved.required().health().per_event(0), None);
    assert_eq!(reserved.required().health().per_event(10), None);
    for event in events.into_iter().skip(1) {
        for _ in 0..3 {
            actual.emit(event, DiagnosticObservation::unavailable());
            actual.pump_one();
        }
    }
    let snapshot: DiagnosticSnapshot = actual.finish();
    assert_eq!(snapshot.required().health().accepted().value(), 32);
    assert_eq!(snapshot.required().health().suppressed().value(), 64);
    assert_eq!(snapshot.required().history().len(), 16);
    assert_eq!(
        snapshot.required().health().history_overwritten().value(),
        16
    );
    assert!(!snapshot.history_complete());
    let sequences: Vec<u64> = snapshot
        .required()
        .history()
        .records()
        .map(|bytes: &[u8; 128]| {
            let mut word: [u8; 8] = [0; 8];
            word.copy_from_slice(&bytes[8..16]);
            u64::from_le_bytes(word)
        })
        .collect();
    assert_eq!(sequences, (77u64..=92).collect::<Vec<u64>>());
    let optional_sequences: Vec<&[u8; 128]> = snapshot.optional().history().records().collect();
    let required_sequences: Vec<&[u8; 128]> = snapshot.required().history().records().collect();
    assert_eq!(optional_sequences, required_sequences);
}

/// Different closed causes coalesce under one ID; rejected text is never retained.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn cause_variation_and_synthetic_secret_rejection_keep_useful_closed_diagnostics() {
    let mut actual: Router = router(
        ConsumerState::Available,
        ConsumerState::Available,
        OptionalProfile::Info,
    );
    assert!(actual.snapshot().required().history().is_empty());
    for (cause, varied) in [
        (PrimaryCause::Native, false),
        (PrimaryCause::Native, false),
        (PrimaryCause::Budget, true),
        (PrimaryCause::ClockReversed, true),
        (PrimaryCause::OutcomeUnavailable, true),
    ] {
        actual.emit(
            ServiceEvent::PrimaryFailure(cause),
            DiagnosticObservation::unavailable(),
        );
        actual.pump_one();
        let observed: DiagnosticSnapshot = actual.snapshot();
        assert_eq!(
            observed
                .required()
                .health()
                .per_event(4)
                .map(|event: &windows_platform::diagnostics::EventHealth| event.cause_variation()),
            Some(varied)
        );
    }
    let before: DiagnosticSnapshot = actual.snapshot();
    let secret: &[u8] = b"SYNTHETIC-COOKIE-SECRET";
    let mut hostile: [u8; 128] = literal();
    hostile[96..96 + secret.len()].copy_from_slice(secret);
    assert_eq!(DecodedRecord::decode(&hostile), Err(DecodeError::InvalidV2));
    let snapshot: DiagnosticSnapshot = actual.finish();
    let history = snapshot.required().history();
    assert!(!history.is_empty());
    let mut retained = history.records();
    for (index, expected) in [9, 9, 8, 5].into_iter().enumerate() {
        let remaining: usize = 4 - index;
        assert_eq!(retained.size_hint(), (remaining, Some(remaining)));
        assert_eq!(
            retained.next().map(|bytes: &[u8; RECORD_BYTES]| bytes[7]),
            Some(expected)
        );
    }
    assert_eq!(retained.size_hint(), (0, Some(0)));
    assert!(retained.next().is_none());
    assert_eq!(
        snapshot.health().emitted().value(),
        before.health().emitted().value()
    );
    let health: Option<&windows_platform::diagnostics::EventHealth> =
        snapshot.required().health().per_event(4);
    assert!(health.is_some());
    if let Some(health) = health {
        assert_eq!(health.first_cause(), Some(9));
        assert!(health.cause_variation());
        assert_eq!(health.suppressed().value(), 1);
    }
    for bytes in snapshot
        .required()
        .history()
        .records()
        .chain(snapshot.optional().history().records())
    {
        assert!(
            !bytes
                .windows(secret.len())
                .any(|window: &[u8]| window == secret)
        );
        assert_eq!(&bytes[25..], &[0; 103]);
    }
}

/// Finite replay compares actual FIFO, losses and consumed originals after every action.
/// Eligibility is independent arithmetic for four complete nine-ID rounds: attempts
/// 1..32 have credits and 33..36 are coalesced; no production reducer computes truth.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn bounded_history_model_checks_every_emit_and_pump_transition() {
    let events: [ServiceEvent; 9] = [
        ServiceEvent::StartObserved,
        ServiceEvent::RunningAcknowledged,
        ServiceEvent::AdmissionClosed,
        ServiceEvent::PrimaryFailure(PrimaryCause::Budget),
        ServiceEvent::CompletionFailure(WorkerCause::Cleanup),
        ServiceEvent::StopReportFailure(LifecycleCause::Report),
        ServiceEvent::TerminalReportFailure(LifecycleCause::Report),
        ServiceEvent::Finished,
        ServiceEvent::OperationFailure(WorkerCause::Preparation),
    ];
    for pump_period in [1u64, 2, 5, 37] {
        let mut actual: Router = router(
            ConsumerState::Available,
            ConsumerState::Available,
            OptionalProfile::Off,
        );
        let mut pending: std::collections::VecDeque<u64> = std::collections::VecDeque::new();
        let mut consumed: Vec<u64> = Vec::new();
        let mut admitted: u64 = 0;
        let mut queue_full: u64 = 0;
        let mut attempts: u64 = 0;
        for event in events.into_iter().cycle().take(36) {
            attempts += 1;
            actual.emit(event, DiagnosticObservation::unavailable());
            if attempts <= 32 {
                if pending.len() == 16 {
                    queue_full += 1;
                } else {
                    pending.push_back(attempts);
                    admitted += 1;
                }
            }
            let queued: DiagnosticSnapshot = actual.snapshot();
            assert_eq!(queued.required().pending(), pending.len());
            assert_eq!(queued.required().health().queued().value(), admitted);
            assert_eq!(queued.required().health().queue_full().value(), queue_full);
            assert_eq!(
                queued.required().health().suppressed().value(),
                attempts.saturating_sub(32)
            );
            if attempts.is_multiple_of(pump_period) {
                actual.pump_one();
                if let Some(sequence) = pending.pop_front() {
                    consumed.push(sequence);
                }
            }
            let stepped: DiagnosticSnapshot = actual.snapshot();
            assert_eq!(stepped.required().pending(), pending.len());
            assert_eq!(
                usize::try_from(stepped.required().health().accepted().value()),
                Ok(consumed.len())
            );
            let expected: Vec<u64> = consumed
                .iter()
                .copied()
                .skip(consumed.len().saturating_sub(16))
                .collect();
            let observed: Vec<u64> = stepped
                .required()
                .history()
                .records()
                .map(|bytes: &[u8; 128]| {
                    let mut word: [u8; 8] = [0; 8];
                    word.copy_from_slice(&bytes[8..16]);
                    u64::from_le_bytes(word)
                })
                .collect();
            assert_eq!(observed, expected);
        }
        let final_view: DiagnosticSnapshot = actual.finish();
        assert_eq!(final_view.required().pending(), 0);
        assert_eq!(final_view.required().health().accepted().value(), admitted);
    }
}

/// Accepted opaque metadata cannot make default Debug format its underlying bytes.
/// Exact bounded text is independent of the encoder, not only a substring check.
///
/// # Panics
/// Fails if the independent oracle observes a contract violation.
#[test]
fn passive_debug_omits_accepted_opaque_synthetic_secret_claims() {
    let secret: [u8; 16] = *b"COOKIE-SECRET-12";
    let mut bytes: [u8; 128] = literal();
    bytes[25] = 1;
    bytes[26..42].copy_from_slice(&secret);
    let decoded: Result<DecodedRecord, DecodeError> = DecodedRecord::decode(&bytes);
    assert!(decoded.is_ok());
    if let Ok(decoded) = decoded {
        assert_eq!(
            decoded.process_instance(),
            MetadataClaim::ObservedClaim(u128::from_le_bytes(secret))
        );
        let formatted: String = format!("{decoded:?}");
        assert_eq!(
            formatted,
            "DecodedRecord { provenance: \"untrusted passive claims\", component: ComponentClaim { code: 1 }, event: ServiceEventClaim { id: 1, cause_code: 0 }, local_sequence: 1, .. }"
        );
        assert!(!formatted.contains("COOKIE-SECRET-12"));
    }
}
