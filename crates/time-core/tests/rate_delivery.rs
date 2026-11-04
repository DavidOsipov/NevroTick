// SPDX-License-Identifier: GPL-2.0-only
//! Seven finite public-owner families for settled modeled rate delivery.
//!
//! The arithmetic expectations are literal dyadic facts. Two genuine three-row
//! histories retain their raw observations and fitted reports across a settled
//! counterfactual translation, including rows before and inside the interval.
//! Monotonic and active-awake coordinates are supplied separately; these tests
//! establish no native delivery, ongoing trajectory or durable broker identity.
//! All loops have literal finite cases; execution/admission belongs to Root.
//!
//! Static authored census: 27 span constructions (two reversed), 33 interval
//! constructions (four unsupported rates), 16 cursors and 24 reference owners.
//! The 38 receiver calls reserve 15 known terminals, three no-effect terminals,
//! one Unknown and 19 outer rejections. Known terminals include three admitted
//! numerical failures and twelve replacement charts. The genuine consumer owns
//! two histories/six appends, four chart projections/enclosures, two raw
//! enclosures, one raw seed/batch and two chart batches/advances. These are fixed
//! source reservations, not executed counters or refunds for early failures.
#![forbid(unsafe_code)]

use std::{error::Error, io, num::NonZeroU64};

use time_core::{
    FixtureAdaptiveHistory, FixtureAffineChart, FixtureChartBatch, FixtureChartCombinationOutcome,
    FixtureChartDomain, FixtureChartError, FixtureChartFrames, FixtureChartInterval,
    FixtureChartOutcome, FixtureChartStep, FixtureCombinationParameters,
    FixtureCombinationSourceState, FixtureCombinationState, FixtureContext, FixtureMinimum,
    FixtureObservation, FixturePrincipalOutcomeKind, FixturePrincipalParameters,
    FixturePrincipalState, FixturePrincipalStratum, FixtureProjectedBatch, FixtureProjectionTerms,
    FixtureRateClock, FixtureRateClockContext, FixtureRateCursorStatus, FixtureRateDeliveryError,
    FixtureRateSettlement, FixtureRateSettlementCompletion, FixtureRateSettlementCursor,
    FixtureRateSettlementKind, FixtureRateSpan, FixtureReferenceInputError,
    FixtureReferenceInputState, FixtureReferenceReadiness, FixtureSettledRateInterval,
    enclose_fixture_chart_projection, enclose_fixture_projection,
};
use time_domain::{ActiveAwakeNs, MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

/// A genuine fixture-construction or missing-result failure.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Exact nanoseconds in one supplied fixture second.
const SECOND: u64 = 1_000_000_000;
/// Complete observable cursor state, without access to its private mapping.
type CursorSnapshot = (
    FixtureRateCursorStatus,
    Option<NonZeroU64>,
    Option<ActiveAwakeNs>,
    Option<MonotonicNs>,
);

/// Admit one fixed nonzero fixture label without substituting another value.
/// # Errors
/// Reject an accidental zero before constructing a contextual value.
fn label(value: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero rate fixture label").into())
}

/// Return independent supplied clock-origin and generation assertions.
/// # Errors
/// Preserve nonzero-label construction failures.
/// # Panics
/// Reject diagnostics that expose the supplied clock labels.
fn clock_context() -> TestResult<FixtureRateClockContext> {
    let context = FixtureRateClockContext::new(label(101)?, label(103)?, label(107)?);
    assert_eq!(format!("{context:?}"), "FixtureRateClockContext { .. }");
    Ok(context)
}

/// Admit the unchanged complete retained-history domain and initial A-to-B chart.
/// # Errors
/// Preserve actual frame, domain and coefficient admission failures.
fn chart(phase: f64, rate: f64) -> TestResult<FixtureAffineChart> {
    Ok(FixtureAffineChart::new(
        FixtureChartFrames::new(label(17)?, label(23)?, label(19)?, label(29)?)?,
        FixtureChartDomain::new(
            MonotonicNs::new(0),
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(8 * SECOND),
        )?,
        phase,
        rate,
    )?)
}

/// Admit a distinct next mapping from the actual predecessor's target.
/// # Errors
/// Preserve nonzero labels and checked frame-relationship rejection.
fn transition(map: &FixtureAffineChart, target: (u64, u64)) -> TestResult<FixtureChartFrames> {
    let old = map.frames();
    Ok(FixtureChartFrames::new(
        old.target_frame(),
        old.target_generation(),
        label(target.0)?,
        label(target.1)?,
    )?)
}

/// Construct ordered spans without deriving either clock from the other.
/// # Errors
/// Preserve the actual independent span constructor's rejection.
/// # Panics
/// Reject an incorrect same-domain elapsed getter or exposed endpoint diagnostics.
fn span(
    active: (u64, u64),
    monotonic: (u64, u64),
) -> Result<FixtureRateSpan, FixtureRateDeliveryError> {
    let checked = FixtureRateSpan::new(
        (ActiveAwakeNs::new(active.0), ActiveAwakeNs::new(active.1)),
        (MonotonicNs::new(monotonic.0), MonotonicNs::new(monotonic.1)),
    )?;
    assert_eq!(checked.active_elapsed().nanoseconds(), active.1 - active.0);
    assert_eq!(format!("{checked:?}"), "FixtureRateSpan { .. }");
    Ok(checked)
}

/// Build a checked interval for one explicit actual chart relationship.
/// # Errors
/// Preserve ordinal, frame, domain, span and rate admission failures.
/// # Panics
/// Reject diagnostics that expose retained rate, endpoint or identity values.
fn interval(
    map: &FixtureAffineChart,
    context: FixtureRateClockContext,
    ordinal: u64,
    times: FixtureRateSpan,
    rate: f64,
    target: (u64, u64),
) -> TestResult<FixtureSettledRateInterval> {
    let interval = FixtureSettledRateInterval::new(
        context,
        label(ordinal)?,
        times,
        rate,
        transition(map, target)?,
        map.domain(),
    )?;
    assert_eq!(format!("{interval:?}"), "FixtureSettledRateInterval { .. }");
    Ok(interval)
}

/// Capture all public mutable cursor observations before a rejected attempt.
/// # Panics
/// Reject diagnostics that expose cursor labels, ordinals or endpoint high-water marks.
fn cursor_snapshot(cursor: &FixtureRateSettlementCursor) -> CursorSnapshot {
    assert_eq!(format!("{cursor:?}"), "FixtureRateSettlementCursor { .. }");
    (
        cursor.status(),
        cursor.next_ordinal(),
        cursor.last_active_end(),
        cursor.last_monotonic_end(),
    )
}

/// Consume a known successful completion without inventing an absent replacement.
/// # Errors
/// Preserve the stored chart cause or reject a missing known replacement.
/// # Panics
/// Reject unredacted completion diagnostics or disagreement in borrowed presence.
fn replacement(completion: FixtureRateSettlementCompletion) -> TestResult<FixtureAffineChart> {
    assert_eq!(
        format!("{completion:?}"),
        "FixtureRateSettlementCompletion { .. }"
    );
    assert!(completion.replacement()?.is_some());
    completion
        .into_replacement()?
        .ok_or_else(|| io::Error::other("missing settled replacement").into())
}

/// Five literal signed/partial facts use active time despite much larger monotonic elapsed.
/// # Errors
/// Preserve real checked chart, interval, receiving and replacement failures.
/// # Panics
/// Reject wrong sign, requested-duration substitution, rate changes or missing consumption.
#[test]
fn dyadic_delivery_and_cancellation_use_active_time_only() -> TestResult {
    for (rate, seconds, cancelled, expected_phase) in [
        (1.0 / 1024.0, 2, false, 3.0 / 512.0),
        (-1.0 / 1024.0, 2, false, 1.0 / 512.0),
        (0.0, 2, false, 1.0 / 256.0),
        (-0.0, 2, false, 1.0 / 256.0),
        (1.0 / 1024.0, 1, true, 5.0 / 1024.0),
    ] {
        let map = chart(1.0 / 256.0, -0.0)?;
        let context = clock_context()?;
        let mut cursor = FixtureRateSettlementCursor::new(&map, context, label(1)?);
        let mut owner = FixtureReferenceInputState::new(&map);
        let value = interval(
            &map,
            context,
            1,
            span((0, seconds * SECOND), (0, 102 * SECOND))?,
            rate,
            (31, 7),
        )?;
        let event = if cancelled {
            FixtureRateSettlement::Cancelled(value)
        } else {
            FixtureRateSettlement::Completed(value)
        };
        assert_eq!(format!("{event:?}"), "FixtureRateSettlement { .. }");
        let result = owner.receive_rate_settlement(&mut cursor, &map, event)?;
        assert_eq!(
            result.kind(),
            if cancelled {
                FixtureRateSettlementKind::Cancelled
            } else {
                FixtureRateSettlementKind::Completed
            }
        );
        let changed = replacement(result)?;
        assert_eq!(changed.phase_at_reference_seconds(), expected_phase);
        assert_eq!(
            changed.rate_seconds_per_second().to_bits(),
            (-0.0_f64).to_bits()
        );
        assert_eq!(changed.domain(), map.domain());
        assert_eq!(
            owner.readiness(),
            FixtureReferenceReadiness::CoordinateChanged
        );
        assert_eq!(cursor.next_ordinal(), Some(label(2)?));
        assert_eq!(
            cursor.last_active_end(),
            Some(ActiveAwakeNs::new(seconds * SECOND))
        );
        assert_eq!(
            cursor.last_monotonic_end(),
            Some(MonotonicNs::new(102 * SECOND))
        );
    }
    Ok(())
}

/// Independent reversal checks have zero-width and finite-normal neighboring controls.
/// # Errors
/// Preserve the legitimate chart, span, interval and receiving construction paths.
/// # Panics
/// Reject a conflated clock error, unsupported scalar acceptance or zero-width rejection.
#[test]
fn span_and_rate_admission_preserve_valid_neighbors() -> TestResult {
    assert_error_diagnostics();
    assert!(matches!(
        span((2, 1), (0, 1)),
        Err(FixtureRateDeliveryError::InvalidSpan {
            clock: FixtureRateClock::ActiveAwake,
        })
    ));
    assert!(matches!(
        span((0, 1), (2, 1)),
        Err(FixtureRateDeliveryError::InvalidSpan {
            clock: FixtureRateClock::Monotonic,
        })
    ));
    let map = chart(1.0 / 256.0, 1.0 / 1024.0)?;
    let frames = transition(&map, (31, 7))?;
    let times = span((0, 0), (0, 0))?;
    for bad in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(1),
    ] {
        assert!(matches!(
            FixtureSettledRateInterval::new(
                clock_context()?,
                label(1)?,
                times,
                bad,
                frames,
                map.domain(),
            ),
            Err(FixtureRateDeliveryError::Chart {
                source: FixtureChartError::InvalidScalar { .. },
            })
        ));
    }
    let _normal = FixtureSettledRateInterval::new(
        clock_context()?,
        label(1)?,
        times,
        f64::MIN_POSITIVE,
        frames,
        map.domain(),
    )?;
    let mut cursor = FixtureRateSettlementCursor::new(&map, clock_context()?, label(1)?);
    let mut owner = FixtureReferenceInputState::new(&map);
    let value = interval(&map, clock_context()?, 1, times, 1.0 / 1024.0, (31, 7))?;
    let changed = replacement(owner.receive_rate_settlement(
        &mut cursor,
        &map,
        FixtureRateSettlement::Completed(value),
    )?)?;
    assert_eq!(
        changed.phase_at_reference_seconds(),
        map.phase_at_reference_seconds()
    );
    assert_eq!(
        owner.readiness(),
        FixtureReferenceReadiness::CoordinateChanged
    );
    assert_eq!(cursor.last_active_end(), Some(ActiveAwakeNs::new(0)));
    assert_eq!(cursor.last_monotonic_end(), Some(MonotonicNs::new(0)));
    Ok(())
}

/// Check every bounded error family without constructing or inspecting rejected payloads.
/// # Panics
/// Reject exposed values, wrong public family text, missing typed causes or invented causes.
fn assert_error_diagnostics() {
    let reference_cause = FixtureReferenceInputError::NotReady;
    let chart_cause = FixtureChartError::Invariant;
    let reference = FixtureRateDeliveryError::Reference {
        source: reference_cause,
    };
    let chart = FixtureRateDeliveryError::Chart {
        source: chart_cause,
    };
    for (error, expected) in [
        (reference, "fixture rate delivery: reference rejected"),
        (
            FixtureRateDeliveryError::CursorUnavailable {
                status: FixtureRateCursorStatus::Unresolved,
            },
            "fixture rate delivery: cursor unavailable",
        ),
        (
            FixtureRateDeliveryError::ClockContextMismatch,
            "fixture rate delivery: clock context mismatch",
        ),
        (
            FixtureRateDeliveryError::OrdinalMismatch,
            "fixture rate delivery: ordinal mismatch",
        ),
        (
            FixtureRateDeliveryError::CursorMappingMismatch,
            "fixture rate delivery: cursor mapping mismatch",
        ),
        (
            FixtureRateDeliveryError::InvalidSpan {
                clock: FixtureRateClock::ActiveAwake,
            },
            "fixture rate delivery: reversed clock span",
        ),
        (
            FixtureRateDeliveryError::ActiveOverlap,
            "fixture rate delivery: active span overlaps",
        ),
        (
            FixtureRateDeliveryError::MonotonicOverlap,
            "fixture rate delivery: monotonic span overlaps",
        ),
        (chart, "fixture rate delivery: chart rejected"),
        (
            FixtureRateDeliveryError::ValidationInvariant,
            "fixture rate delivery: validation invariant",
        ),
    ] {
        assert_eq!(format!("{error}"), expected);
        assert_eq!(format!("{error:?}"), "FixtureRateDeliveryError { .. }");
        assert_eq!(
            error.source().is_some(),
            matches!(
                error,
                FixtureRateDeliveryError::Reference { .. } | FixtureRateDeliveryError::Chart { .. }
            )
        );
    }
    assert_eq!(
        reference
            .source()
            .and_then(|cause| cause.downcast_ref::<FixtureReferenceInputError>()),
        Some(&reference_cause)
    );
    assert_eq!(
        chart
            .source()
            .and_then(|cause| cause.downcast_ref::<FixtureChartError>()),
        Some(&chart_cause)
    );
}

/// No-effect consumes without endpoints; Unknown blocks a fresh wrapper on the same chart.
/// # Errors
/// Preserve valid constructor, known-neighbor and actual terminal receiving failures.
/// # Panics
/// Reject invented endpoint/delivery credit, restored readiness or reusable Unknown state.
#[test]
fn noeffect_and_unknown_have_distinct_terminal_behavior() -> TestResult {
    let map = chart(1.0 / 256.0, 1.0 / 1024.0)?;
    let context = clock_context()?;
    let mut cursor = FixtureRateSettlementCursor::new(&map, context, label(1)?);
    let mut owner = FixtureReferenceInputState::new(&map);
    let none = owner.receive_rate_settlement(
        &mut cursor,
        &map,
        FixtureRateSettlement::ProvenNoEffect {
            context,
            ordinal: label(1)?,
        },
    )?;
    assert_eq!(none.kind(), FixtureRateSettlementKind::ProvenNoEffect);
    assert!(none.replacement()?.is_none());
    assert!(none.into_replacement()?.is_none());
    assert_eq!(owner.readiness(), FixtureReferenceReadiness::Ready);
    assert_eq!(
        cursor_snapshot(&cursor),
        (
            FixtureRateCursorStatus::Pending,
            Some(label(2)?),
            None,
            None
        )
    );
    let known = interval(
        &map,
        context,
        2,
        span((0, SECOND), (0, SECOND))?,
        1.0 / 1024.0,
        (31, 7),
    )?;
    let changed = replacement(owner.receive_rate_settlement(
        &mut cursor,
        &map,
        FixtureRateSettlement::Completed(known),
    )?)?;
    let mut next_owner = FixtureReferenceInputState::new(&changed);
    let endpoints = (cursor.last_active_end(), cursor.last_monotonic_end());
    let none = next_owner.receive_rate_settlement(
        &mut cursor,
        &changed,
        FixtureRateSettlement::ProvenNoEffect {
            context,
            ordinal: label(3)?,
        },
    )?;
    assert!(none.into_replacement()?.is_none());
    assert_eq!(next_owner.readiness(), FixtureReferenceReadiness::Ready);
    assert_eq!(
        (cursor.last_active_end(), cursor.last_monotonic_end()),
        endpoints
    );
    let unknown = next_owner.receive_rate_settlement(
        &mut cursor,
        &changed,
        FixtureRateSettlement::Unknown {
            context,
            ordinal: label(4)?,
        },
    )?;
    assert_eq!(unknown.kind(), FixtureRateSettlementKind::Unknown);
    assert!(unknown.into_replacement()?.is_none());
    assert_eq!(next_owner.readiness(), FixtureReferenceReadiness::Unknown);
    assert_eq!(cursor.status(), FixtureRateCursorStatus::Unresolved);
    assert_eq!(cursor.next_ordinal(), None);
    assert_eq!(
        (cursor.last_active_end(), cursor.last_monotonic_end()),
        endpoints
    );
    let mut fresh_wrapper = FixtureReferenceInputState::new(&changed);
    assert!(matches!(
        fresh_wrapper.receive_rate_settlement(
            &mut cursor,
            &changed,
            FixtureRateSettlement::ProvenNoEffect {
                context,
                ordinal: label(5)?
            }
        ),
        Err(FixtureRateDeliveryError::CursorUnavailable {
            status: FixtureRateCursorStatus::Unresolved
        })
    ));
    assert_eq!(fresh_wrapper.readiness(), FixtureReferenceReadiness::Ready);
    Ok(())
}

/// One independently specified rejected next-interval shape.
type NextIntervalCase = (
    u64,
    FixtureRateClockContext,
    u64,
    u64,
    FixtureRateDeliveryError,
);

/// Supply seven literal identity/context/overlap defects with valid neighboring endpoints.
/// # Errors
/// Preserve accidental nonzero-tag construction failures.
fn rejected_next_intervals(context: FixtureRateClockContext) -> TestResult<[NextIntervalCase; 7]> {
    let foreign = FixtureRateClockContext::new(label(101)?, label(103)?, label(109)?);
    let foreign_active = FixtureRateClockContext::new(label(101)?, label(109)?, label(107)?);
    let foreign_monotonic = FixtureRateClockContext::new(label(109)?, label(103)?, label(107)?);
    Ok([
        (
            1,
            context,
            12,
            102,
            FixtureRateDeliveryError::OrdinalMismatch,
        ),
        (
            3,
            context,
            12,
            102,
            FixtureRateDeliveryError::OrdinalMismatch,
        ),
        (
            2,
            foreign,
            12,
            102,
            FixtureRateDeliveryError::ClockContextMismatch,
        ),
        (
            2,
            foreign_active,
            12,
            102,
            FixtureRateDeliveryError::ClockContextMismatch,
        ),
        (
            2,
            foreign_monotonic,
            12,
            102,
            FixtureRateDeliveryError::ClockContextMismatch,
        ),
        (2, context, 11, 102, FixtureRateDeliveryError::ActiveOverlap),
        (
            2,
            context,
            12,
            101,
            FixtureRateDeliveryError::MonotonicOverlap,
        ),
    ])
}

/// Seven rejected next intervals preserve the same new owner; adjacency then succeeds.
/// # Errors
/// Preserve actual chart, interval and current-owner construction failures.
/// # Panics
/// Reject forgotten cross-owner replay state, clock context or either high-water mark.
#[test]
fn cursor_carries_identity_and_nonoverlap_across_replacement_owners() -> TestResult {
    let map = chart(1.0 / 256.0, 1.0 / 1024.0)?;
    let context = clock_context()?;
    let mut cursor = FixtureRateSettlementCursor::new(&map, context, label(1)?);
    let mut owner = FixtureReferenceInputState::new(&map);
    let first = interval(
        &map,
        context,
        1,
        span((10 * SECOND, 12 * SECOND), (100 * SECOND, 102 * SECOND))?,
        1.0 / 1024.0,
        (31, 7),
    )?;
    let changed = replacement(owner.receive_rate_settlement(
        &mut cursor,
        &map,
        FixtureRateSettlement::Completed(first),
    )?)?;
    reject_single_target_tag_changes(&changed, &mut cursor, context, label(2)?)?;
    let mut next_owner = FixtureReferenceInputState::new(&changed);
    for (ordinal, clocks, active_start, mono_start, expected) in rejected_next_intervals(context)? {
        let before = cursor_snapshot(&cursor);
        let value = interval(
            &changed,
            clocks,
            ordinal,
            span(
                (active_start * SECOND, 13 * SECOND),
                (mono_start * SECOND, 103 * SECOND),
            )?,
            1.0 / 1024.0,
            (37, 11),
        )?;
        assert_eq!(
            next_owner
                .receive_rate_settlement(
                    &mut cursor,
                    &changed,
                    FixtureRateSettlement::Completed(value)
                )
                .err(),
            Some(expected)
        );
        assert_eq!(next_owner.readiness(), FixtureReferenceReadiness::Ready);
        assert_eq!(cursor_snapshot(&cursor), before);
    }
    let adjacent = interval(
        &changed,
        context,
        2,
        span((12 * SECOND, 13 * SECOND), (102 * SECOND, 103 * SECOND))?,
        1.0 / 1024.0,
        (37, 11),
    )?;
    let latest = replacement(next_owner.receive_rate_settlement(
        &mut cursor,
        &changed,
        FixtureRateSettlement::Cancelled(adjacent),
    )?)?;
    assert_eq!(latest.phase_at_reference_seconds(), 7.0 / 1024.0);
    assert_eq!(cursor.next_ordinal(), Some(label(3)?));
    assert_eq!(
        cursor.last_active_end(),
        Some(ActiveAwakeNs::new(13 * SECOND))
    );
    assert_eq!(
        cursor.last_monotonic_end(),
        Some(MonotonicNs::new(103 * SECOND))
    );
    Ok(())
}

/// Reject each independent target tag mismatch on the actual advanced cursor.
///
/// The supplied predecessor and cursor are the first completed31/7 transition.
/// No-effect events isolate cursor target admission from known-interval guards.
/// # Errors
/// Preserve legitimate frame/chart construction failures in both neighboring views.
/// # Panics
/// Reject one-field acceptance or any owner/cursor change before admission.
fn reject_single_target_tag_changes(
    predecessor: &FixtureAffineChart,
    cursor: &mut FixtureRateSettlementCursor,
    context: FixtureRateClockContext,
    ordinal: NonZeroU64,
) -> TestResult {
    let frames = predecessor.frames();
    assert_eq!(
        (
            frames.target_frame().get(),
            frames.target_generation().get()
        ),
        (31, 7)
    );
    for (target_frame, target_generation) in [
        (frames.target_frame(), label(13)?),
        (label(41)?, frames.target_generation()),
    ] {
        let foreign = FixtureAffineChart::new(
            FixtureChartFrames::new(
                frames.source_frame(),
                frames.source_generation(),
                target_frame,
                target_generation,
            )?,
            predecessor.domain(),
            predecessor.phase_at_reference_seconds(),
            predecessor.rate_seconds_per_second(),
        )?;
        let mut foreign_owner = FixtureReferenceInputState::new(&foreign);
        let before = cursor_snapshot(cursor);
        assert_eq!(
            foreign_owner
                .receive_rate_settlement(
                    cursor,
                    &foreign,
                    FixtureRateSettlement::ProvenNoEffect { context, ordinal },
                )
                .err(),
            Some(FixtureRateDeliveryError::CursorMappingMismatch)
        );
        assert_eq!(foreign_owner.readiness(), FixtureReferenceReadiness::Ready);
        assert_eq!(cursor_snapshot(cursor), before);
    }
    Ok(())
}

/// Exact owner identity and chart/domain relationships reject beside one healthy call.
/// # Errors
/// Preserve admitted fixture descriptors and the final valid receiving neighbor.
/// # Panics
/// Reject equality-as-identity, foreign cursor reuse or pre-admission mutation.
#[test]
fn current_owner_and_chart_relationships_are_required_before_consumption() -> TestResult {
    let map = chart(1.0 / 256.0, 1.0 / 1024.0)?;
    let equal_foreign = chart(1.0 / 256.0, 1.0 / 1024.0)?;
    let context = clock_context()?;
    let mut cursor = FixtureRateSettlementCursor::new(&map, context, label(1)?);
    let mut owner = FixtureReferenceInputState::new(&map);
    let times = span((0, SECOND), (0, SECOND))?;
    let valid = interval(&map, context, 1, times, 1.0 / 1024.0, (31, 7))?;
    let before = cursor_snapshot(&cursor);
    assert_eq!(
        owner
            .receive_rate_settlement(
                &mut cursor,
                &equal_foreign,
                FixtureRateSettlement::Completed(valid)
            )
            .err(),
        Some(FixtureRateDeliveryError::Reference {
            source: FixtureReferenceInputError::ForeignChartView,
        })
    );
    assert_eq!(cursor_snapshot(&cursor), before);
    let foreign_map = FixtureAffineChart::new(
        FixtureChartFrames::new(label(17)?, label(23)?, label(41)?, label(43)?)?,
        map.domain(),
        0.0,
        0.0,
    )?;
    let mut foreign_cursor = FixtureRateSettlementCursor::new(&foreign_map, context, label(1)?);
    let foreign_before = cursor_snapshot(&foreign_cursor);
    assert_eq!(
        owner
            .receive_rate_settlement(
                &mut foreign_cursor,
                &map,
                FixtureRateSettlement::Completed(valid)
            )
            .err(),
        Some(FixtureRateDeliveryError::CursorMappingMismatch)
    );
    assert_eq!(cursor_snapshot(&foreign_cursor), foreign_before);
    reject_chart_relationships(&mut owner, &mut cursor, &map, times)?;
    assert_eq!(owner.readiness(), FixtureReferenceReadiness::Ready);
    assert_eq!(cursor_snapshot(&cursor), before);
    let _changed = replacement(owner.receive_rate_settlement(
        &mut cursor,
        &map,
        FixtureRateSettlement::Completed(valid),
    )?)?;
    Ok(())
}

/// Three literal domain, intermediate-map and final-endpoint defects reject unchanged.
/// # Errors
/// Preserve legitimate raw descriptor/interval construction failures.
/// # Panics
/// Reject a lost typed cause or state/cursor mutation before admission.
fn reject_chart_relationships(
    owner: &mut FixtureReferenceInputState<'_>,
    cursor: &mut FixtureRateSettlementCursor,
    map: &FixtureAffineChart,
    times: FixtureRateSpan,
) -> TestResult {
    let other_domain = FixtureChartDomain::new(
        MonotonicNs::new(0),
        MonotonicNs::new(2 * SECOND),
        MonotonicNs::new(9 * SECOND),
    )?;
    for (frames, domain, expected) in [
        (
            transition(map, (31, 7))?,
            other_domain,
            FixtureRateDeliveryError::Reference {
                source: FixtureReferenceInputError::Chart {
                    source: FixtureChartError::CompositionDomainMismatch,
                },
            },
        ),
        (
            FixtureChartFrames::new(label(97)?, label(101)?, label(31)?, label(7)?)?,
            map.domain(),
            FixtureRateDeliveryError::Reference {
                source: FixtureReferenceInputError::Chart {
                    source: FixtureChartError::CompositionMappingMismatch,
                },
            },
        ),
        (
            transition(map, (17, 23))?,
            map.domain(),
            FixtureRateDeliveryError::Chart {
                source: FixtureChartError::InvalidFrames,
            },
        ),
    ] {
        let before = cursor_snapshot(cursor);
        let value = FixtureSettledRateInterval::new(
            clock_context()?,
            label(1)?,
            times,
            1.0 / 1024.0,
            frames,
            domain,
        )?;
        assert_eq!(
            owner
                .receive_rate_settlement(cursor, map, FixtureRateSettlement::Completed(value))
                .err(),
            Some(expected)
        );
        assert_eq!(owner.readiness(), FixtureReferenceReadiness::Ready);
        assert_eq!(cursor_snapshot(cursor), before);
    }
    Ok(())
}

/// An admitted failure consumes its ordinal and target mapping before returning its cause.
/// # Errors
/// Preserve real fixture admission or reject an unexpected successful replacement.
/// # Panics
/// Reject an unconsumed failure or an old-B wrapper borrowing the advanced cursor.
fn admitted_failure(
    map: &FixtureAffineChart,
    active_end: u64,
    rate: f64,
) -> TestResult<FixtureChartError> {
    let context = clock_context()?;
    let mut cursor = FixtureRateSettlementCursor::new(map, context, label(1)?);
    let mut owner = FixtureReferenceInputState::new(map);
    let value = interval(
        map,
        context,
        1,
        span((0, active_end), (0, SECOND))?,
        rate,
        (31, 7),
    )?;
    let result =
        owner.receive_rate_settlement(&mut cursor, map, FixtureRateSettlement::Completed(value))?;
    assert_eq!(result.kind(), FixtureRateSettlementKind::Completed);
    let cause = result
        .replacement()
        .err()
        .ok_or_else(|| io::Error::other("expected admitted arithmetic failure"))?;
    assert_eq!(result.into_replacement().err(), Some(cause));
    assert_eq!(
        owner.readiness(),
        FixtureReferenceReadiness::CoordinateChanged
    );
    assert_eq!(cursor.next_ordinal(), Some(label(2)?));
    assert_eq!(
        cursor.last_active_end(),
        Some(ActiveAwakeNs::new(active_end))
    );
    assert_eq!(cursor.last_monotonic_end(), Some(MonotonicNs::new(SECOND)));
    let mut old_wrapper = FixtureReferenceInputState::new(map);
    let next = interval(
        map,
        context,
        2,
        span((active_end, active_end), (SECOND, SECOND))?,
        0.0,
        (37, 11),
    )?;
    assert_eq!(
        old_wrapper
            .receive_rate_settlement(&mut cursor, map, FixtureRateSettlement::Completed(next))
            .err(),
        Some(FixtureRateDeliveryError::CursorMappingMismatch)
    );
    Ok(cause)
}

/// Arithmetic/representation failures consume; exact support and ordinal exhaustion have neighbors.
/// # Errors
/// Preserve legitimate fixture construction and receiving failures.
/// # Panics
/// Reject a lost arithmetic cause, unchecked duration conversion or ordinal wrap.
#[test]
fn admitted_failures_and_exhaustion_cannot_be_retried() -> TestResult {
    let map = chart(1.0 / 256.0, 0.0)?;
    assert!(matches!(
        admitted_failure(&map, 2 * SECOND, f64::MAX)?,
        FixtureChartError::Arithmetic { .. }
    ));
    assert!(matches!(
        admitted_failure(&map, (1_u64 << 53) + 1, 1.0 / 1024.0)?,
        FixtureChartError::Normalization { .. }
    ));
    let huge = chart(f64::MAX, 0.0)?;
    assert!(matches!(
        admitted_failure(&huge, SECOND, f64::MAX / 4.0)?,
        FixtureChartError::Arithmetic {
            step: FixtureChartStep::ComposeDisplacement,
            ..
        }
    ));
    let context = clock_context()?;
    let mut cursor = FixtureRateSettlementCursor::new(&map, context, label(1)?);
    let mut owner = FixtureReferenceInputState::new(&map);
    let supported = interval(
        &map,
        context,
        1,
        span((0, 1_u64 << 53), (0, SECOND))?,
        1.0 / 1024.0,
        (31, 7),
    )?;
    let _changed = replacement(owner.receive_rate_settlement(
        &mut cursor,
        &map,
        FixtureRateSettlement::Completed(supported),
    )?)?;
    let mut last = FixtureRateSettlementCursor::new(&map, context, label(u64::MAX)?);
    let mut ready = FixtureReferenceInputState::new(&map);
    let result = ready.receive_rate_settlement(
        &mut last,
        &map,
        FixtureRateSettlement::ProvenNoEffect {
            context,
            ordinal: label(u64::MAX)?,
        },
    )?;
    assert!(result.into_replacement()?.is_none());
    assert_eq!(ready.readiness(), FixtureReferenceReadiness::Ready);
    assert_eq!(last.status(), FixtureRateCursorStatus::Exhausted);
    assert_eq!(last.next_ordinal(), None);
    assert!(matches!(
        ready.receive_rate_settlement(
            &mut last,
            &map,
            FixtureRateSettlement::ProvenNoEffect {
                context,
                ordinal: label(u64::MAX)?
            }
        ),
        Err(FixtureRateDeliveryError::CursorUnavailable {
            status: FixtureRateCursorStatus::Exhausted
        })
    ));
    Ok(())
}

/// Genuine raw source state, never a detached fitted payload.
struct Source {
    /// Original asserted source/frame/generation of all three observations.
    context: FixtureContext,
    /// Sole owner of actual exchange-derived observations and the fitted report.
    history: FixtureAdaptiveHistory,
}

impl Source {
    /// Build three chronological rows using the established nonconstant dyadic fixture.
    /// # Errors
    /// Preserve actual domain, observation, append and adaptive fitting failures.
    fn new(id: u64, values: [i32; 3]) -> TestResult<Self> {
        let context = FixtureContext::new(label(id)?, label(17)?, label(23)?);
        let mut history = FixtureAdaptiveHistory::new(context, 3)?;
        for ((second, value), divisor) in [0, 1, 2].into_iter().zip(values).zip([1.0, 1.0, 4.0]) {
            let zero = UtcCoordinateQ32::new(0)?;
            let shifted = UtcCoordinateQ32::new(i128::from(value) << 28)?;
            let phase = exchange_arithmetic(zero, shifted, shifted, zero)?.offset();
            history.try_append(FixtureObservation::new(
                context,
                MonotonicNs::new(second * SECOND),
                phase,
                divisor,
            )?)?;
        }
        Ok(Self { context, history })
    }

    /// Project the real latest fitted row and enclose the genuine tracking result.
    /// # Errors
    /// Preserve public chart-view, original producer and enclosure failures.
    fn entry<'chart>(
        &self,
        map: &'chart FixtureAffineChart,
    ) -> TestResult<FixtureChartInterval<'chart>> {
        let view = self
            .history
            .prepare_chart(self.context, FixtureChartOutcome::Known(map))?
            .ok_or_else(|| io::Error::other("missing rate fixture chart view"))?;
        let reference = self
            .history
            .reference()
            .ok_or_else(|| io::Error::other("missing rate fixture reference"))?;
        let projection = view.project_tracking(
            view.context(),
            2,
            reference,
            MonotonicNs::new(4 * SECOND),
            FixtureProjectionTerms::new(0.25, 0.0, 0.125)?,
            0.125,
        )?;
        Ok(enclose_fixture_chart_projection(&projection)?)
    }
}

/// Original raw rows and fitted coefficient bits, including every pre-interval row.
#[derive(Debug, Eq, PartialEq)]
struct SourceSnapshot {
    /// Original raw context, never replaced by target-chart metadata.
    context: FixtureContext,
    /// Actual times, Q33 numerators and divisor bits for exactly three rows.
    rows: [(u64, i128, u64); 3],
    /// Actual fitted and auxiliary occupancy.
    counts: (usize, usize),
    /// Original fitted phase and rate stored bits.
    coefficients: (u64, u64),
}

/// Capture original history evidence without copying it into another history owner.
/// # Errors
/// Reject a missing genuine row/report or unexpected fixture occupancy.
fn source_snapshot(source: &Source) -> TestResult<SourceSnapshot> {
    let counts = (
        source.history.sample_count(),
        source.history.auxiliary_count(),
    );
    if counts != (3, 0) {
        return Err(io::Error::other("rate fixture occupancy").into());
    }
    let mut rows = [(0, 0, 0); 3];
    for (index, target) in rows.iter_mut().enumerate() {
        let row = source
            .history
            .decision_observation(index)
            .ok_or_else(|| io::Error::other("missing rate fixture row"))?;
        *target = (
            row.observed_at().nanoseconds(),
            row.phase().numerator(),
            row.variance_divisor().to_bits(),
        );
    }
    let line = source
        .history
        .report()
        .ok_or_else(|| io::Error::other("missing rate fixture report"))?
        .selection()
        .statistics()
        .line();
    Ok(SourceSnapshot {
        context: source.context,
        rows,
        counts,
        coefficients: (
            line.phase_seconds().to_bits(),
            line.phase_rate_seconds_per_second().to_bits(),
        ),
    })
}

/// Seed the existing raw roster from genuine original projections and explicit minimum two.
/// # Errors
/// Preserve actual enclosures, batch, parameters and complete-owner seed failures.
fn seed(
    entries: &[FixtureChartInterval<'_>; 2],
    strata: &[FixturePrincipalStratum; 2],
) -> TestResult<FixtureCombinationState> {
    let raw = [
        enclose_fixture_projection(entries[0].projection().source_projection())?,
        enclose_fixture_projection(entries[1].projection().source_projection())?,
    ];
    let batch =
        FixtureProjectedBatch::new(label(17)?, label(23)?, MonotonicNs::new(4 * SECOND), &raw)?;
    Ok(FixtureCombinationState::seed(
        &[label(11)?, label(22)?],
        FixturePrincipalParameters::new(1.0 / 1024.0, 0.0)?,
        FixtureCombinationParameters::new(2.0, 0.0)?,
        FixtureMinimum::new(2)?,
        &batch,
        strata,
    )?)
}

/// Compare actual raw evidence and statistics while independently checking the dyadic shift.
/// # Errors
/// Reject a missing genuine tracking attachment.
/// # Panics
/// Reject a raw rewrite, changed uncertainty/rate or a lost integration bound.
fn check_remapped(
    old: &FixtureChartInterval<'_>,
    new: &FixtureChartInterval<'_>,
    bound: f64,
) -> TestResult {
    let old = old.projection();
    let new = new.projection();
    let raw = old.source_projection();
    let remapped_raw = new.source_projection();
    assert_eq!(remapped_raw.context(), raw.context());
    assert_eq!(remapped_raw.anchor_observed_at(), raw.anchor_observed_at());
    assert_eq!(remapped_raw.latest_observed_at(), raw.latest_observed_at());
    assert_eq!(remapped_raw.fit_reference(), raw.fit_reference());
    assert_eq!(remapped_raw.projected_at(), raw.projected_at());
    assert_eq!(
        remapped_raw.center_seconds().to_bits(),
        raw.center_seconds().to_bits()
    );
    assert_eq!(
        remapped_raw.distance_seconds().to_bits(),
        raw.distance_seconds().to_bits()
    );
    assert_eq!(new.center_seconds(), old.center_seconds() - 1.0 / 512.0);
    assert_eq!(
        new.phase_rate_seconds_per_second().to_bits(),
        old.phase_rate_seconds_per_second().to_bits()
    );
    assert!(new.center_roundoff_seconds() >= bound && bound > 0.0);
    let old_tracking = old
        .tracking()
        .ok_or_else(|| io::Error::other("missing original rate tracking"))?;
    let new_tracking = new
        .tracking()
        .ok_or_else(|| io::Error::other("missing remapped rate tracking"))?;
    assert!(old_tracking.phase_standard_deviation_seconds() > 0.0);
    assert_eq!(
        new_tracking.phase_standard_deviation_seconds().to_bits(),
        old_tracking.phase_standard_deviation_seconds().to_bits()
    );
    assert_eq!(
        new_tracking
            .rate_standard_deviation_seconds_per_second()
            .to_bits(),
        old_tracking
            .rate_standard_deviation_seconds_per_second()
            .to_bits()
    );
    assert_eq!(
        new_tracking.tracking_skew_seconds_per_second().to_bits(),
        old_tracking.tracking_skew_seconds_per_second().to_bits()
    );
    Ok(())
}

/// Actual credited owner state captured before a coordinate-only replacement.
struct RetainedCredit {
    /// Genuine principal lifecycle snapshot, without manufacturing an emission.
    principal: FixturePrincipalState,
    /// Both actual combination source counters.
    sources: [Option<FixtureCombinationSourceState>; 2],
}

/// Capture genuine principal and source credit before the remapping transition.
/// # Errors
/// Preserve fixed source-label construction failures.
fn retained_credit(owner: &FixtureCombinationState) -> TestResult<RetainedCredit> {
    Ok(RetainedCredit {
        principal: owner.principal_state().clone(),
        sources: [
            owner.source_state(label(11)?).copied(),
            owner.source_state(label(22)?).copied(),
        ],
    })
}

/// A remapped retained emission has neither a fresh round nor changed credit.
/// # Errors
/// Preserve fixed source-label construction failures.
/// # Panics
/// Reject a manufactured estimate, observation update or lifecycle change.
fn check_retained(
    owner: &FixtureCombinationState,
    outcome: &FixtureChartCombinationOutcome<'_, '_, '_>,
    before: &RetainedCredit,
) -> TestResult {
    assert_eq!(
        outcome.principal().kind(),
        FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
    );
    assert_eq!(outcome.principal().updated_source(), None);
    assert!(outcome.combined().is_none());
    assert_eq!(outcome.emitted_round(), None);
    assert_eq!(owner.emitted_rounds(), 1);
    assert_eq!(owner.principal_state(), &before.principal);
    assert_eq!(
        [
            owner.source_state(label(11)?).copied(),
            owner.source_state(label(22)?).copied()
        ],
        before.sources
    );
    Ok(())
}

/// Original rows at 0/1/2 seconds survive an interval at 1..3 without new evidence credit.
/// # Errors
/// Preserve every genuine history, chart, enclosure, batch and receiving failure.
/// # Panics
/// Reject cessation-domain truncation, rewritten evidence or remap-created readiness.
#[test]
fn settled_translation_preserves_genuine_retained_histories_and_credit() -> TestResult {
    let a = Source::new(11, [0, 2, 0])?;
    let b = Source::new(22, [1, 3, 1])?;
    let before = [source_snapshot(&a)?, source_snapshot(&b)?];
    let map = chart(1.0 / 256.0, 1.0 / 1024.0)?;
    let original = [a.entry(&map)?, b.entry(&map)?];
    let strata = [
        FixturePrincipalStratum::new(label(11)?, 2)?,
        FixturePrincipalStratum::new(label(22)?, 2)?,
    ];
    let batch = FixtureChartBatch::new(&map, MonotonicNs::new(4 * SECOND), &original)?;
    let mut combined = seed(&original, &strata)?;
    let first = combined.advance_chart(&map, &batch, &strata)?;
    assert_eq!(first.emitted_round(), Some(label(1)?));
    let mut owner = FixtureReferenceInputState::new(&map);
    assert_eq!(
        owner.admit_reference_input(&first)?.emitted_round(),
        label(1)?
    );
    let credit = retained_credit(&combined)?;
    let mut cursor = FixtureRateSettlementCursor::new(&map, clock_context()?, label(1)?);
    let value = interval(
        &map,
        clock_context()?,
        1,
        span((10 * SECOND, 12 * SECOND), (SECOND, 3 * SECOND))?,
        1.0 / 1024.0,
        (31, 7),
    )?;
    let changed = replacement(owner.receive_rate_settlement(
        &mut cursor,
        &map,
        FixtureRateSettlement::Completed(value),
    )?)?;
    assert_eq!(changed.domain(), map.domain());
    assert_eq!(changed.phase_at_reference_seconds(), 3.0 / 512.0);
    let remapped = [a.entry(&changed)?, b.entry(&changed)?];
    // These independently specified binades make the literal 1/512 subtraction
    // exact on the admitted binary64 grids; the original fit is not a new oracle.
    assert!(
        (9.0 / 128.0..15.0 / 128.0).contains(
            &original[0]
                .projection()
                .source_projection()
                .center_seconds(),
        )
    );
    assert!(
        (17.0 / 128.0..31.0 / 128.0).contains(
            &original[1]
                .projection()
                .source_projection()
                .center_seconds(),
        )
    );
    check_remapped(
        &original[0],
        &remapped[0],
        changed.phase_coefficient_roundoff_seconds(),
    )?;
    check_remapped(
        &original[1],
        &remapped[1],
        changed.phase_coefficient_roundoff_seconds(),
    )?;
    let remapped_batch = FixtureChartBatch::new(&changed, MonotonicNs::new(4 * SECOND), &remapped)?;
    let retained = combined.advance_chart(&changed, &remapped_batch, &strata)?;
    check_retained(&combined, &retained, &credit)?;
    assert_eq!([source_snapshot(&a)?, source_snapshot(&b)?], before);
    assert!(matches!(
        owner.admit_reference_input(&first),
        Err(FixtureReferenceInputError::Unavailable {
            readiness: FixtureReferenceReadiness::CoordinateChanged,
        })
    ));
    let next_owner = FixtureReferenceInputState::new(&changed);
    assert!(matches!(
        next_owner.admit_reference_input(&retained),
        Err(FixtureReferenceInputError::NotReady)
    ));
    Ok(())
}
