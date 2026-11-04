//! Independent leap-history and admission controls through the public boundary.
#![forbid(unsafe_code)]

#[path = "private/leap_properties.rs"]
mod leap_properties;

use std::error::Error;

use time_domain::{
    EraWindow, LeapContinuousFrom, LeapDataIdentity, LeapDateScope, LeapError, LeapEventKnowledge,
    LeapFutureExpiry, LeapHistoryBefore, LeapInterpretation, LeapKnowledge, LeapOffsetKnowledge,
    MAX_COORDINATE_TICKS,
};

/// Exact Q32 unit, independently fixed by the coordinate representation.
const SECOND: i128 = 4_294_967_296;

/// Real baseline and the first two transitions, read from the pinned IANA input.
const EARLY_HISTORY: [(u64, i32); 3] = [
    (2_272_060_800, 10),
    (2_287_785_600, 11),
    (2_303_683_200, 12),
];

/// Full literal IANA 2026f ledger, independently read from the pinned raw input.
const REAL_HISTORY: [(u64, i32); 28] = [
    (2_272_060_800, 10),
    (2_287_785_600, 11),
    (2_303_683_200, 12),
    (2_335_219_200, 13),
    (2_366_755_200, 14),
    (2_398_291_200, 15),
    (2_429_913_600, 16),
    (2_461_449_600, 17),
    (2_492_985_600, 18),
    (2_524_521_600, 19),
    (2_571_782_400, 20),
    (2_603_318_400, 21),
    (2_634_854_400, 22),
    (2_698_012_800, 23),
    (2_776_982_400, 24),
    (2_840_140_800, 25),
    (2_871_676_800, 26),
    (2_918_937_600, 27),
    (2_950_473_600, 28),
    (2_982_009_600, 29),
    (3_029_443_200, 30),
    (3_076_704_000, 31),
    (3_124_137_600, 32),
    (3_345_062_400, 33),
    (3_439_756_800, 34),
    (3_550_089_600, 35),
    (3_644_697_600, 36),
    (3_692_217_600, 37),
];

/// Explicit reviewed history endpoint, not derived from file update or expiry.
const REVIEWED_BEFORE: u64 = 3_992_284_800;
/// Independently pinned finite IANA expiry, exclusive of the endpoint itself.
const REAL_EXPIRY: u64 = 4_023_129_600;
/// Independent BIPM effective rule date, which is not a leap-event record.
const CONTINUOUS_FROM: u64 = 4_019_760_000;

/// Independent synthetic fixed-width identity labels, never authenticated facts.
///
/// # Errors
/// Propagates unexpected rejection of a nonzero declared revision.
fn identity() -> Result<LeapDataIdentity, LeapError> {
    LeapDataIdentity::new(1, [0x19; 32], [0x72; 32])
}

/// Form an exact whole-second query without consulting production calendar code.
///
/// # Errors
/// Propagates numeric-window rejection for these independently bounded constants.
fn window(earliest: u64, latest: u64) -> Result<EraWindow, Box<dyn Error>> {
    Ok(EraWindow::new(
        i128::from(earliest) * SECOND,
        i128::from(latest) * SECOND,
    )?)
}

/// Admit raw ledger facts through the real public constructors, without clocks.
///
/// # Errors
/// Propagates metadata, identity or ledger rejection without altering raw input.
fn knowledge(
    records: &[(u64, i32)],
    reviewed_before: u64,
    expiry: Option<u64>,
    continuous: Option<u64>,
) -> Result<LeapKnowledge<'_>, LeapError> {
    LeapKnowledge::new(
        records,
        LeapHistoryBefore::new(reviewed_before)?,
        expiry.map(LeapFutureExpiry::new).transpose()?,
        continuous.map(LeapContinuousFrom::new).transpose()?,
        identity()?,
    )
}

/// Compare separately specified offset, event-completeness and membership facts.
///
/// # Panics
/// Fails if any independently expected fact differs from the actual answer.
fn assert_answer(
    answer: LeapInterpretation,
    offset: LeapOffsetKnowledge,
    events: LeapEventKnowledge,
    known_transitions: usize,
) {
    assert_eq!(answer.offset(), offset);
    assert_eq!(answer.events(), events);
    assert_eq!(answer.known_transition_count(), known_transitions);
}

/// LP-C01: a real historical query must return its independently recorded offset.
///
/// # Errors
/// Propagates unexpected constructor or numeric-scope rejection.
/// # Panics
/// Fails when admitted historical knowledge is incorrectly reported as unknown.
#[test]
fn historical_baseline_and_transition_are_known() -> Result<(), Box<dyn Error>> {
    let history = LeapKnowledge::new(
        &EARLY_HISTORY,
        LeapHistoryBefore::new(2_335_219_200)?,
        None,
        None,
        identity()?,
    )?;
    let scope = LeapDateScope::new(window(2_272_060_800, 2_335_219_199)?)?;
    let result = history.interpret(window(2_287_785_600, 2_287_785_600)?, scope)?;
    assert_eq!(result.offset(), LeapOffsetKnowledge::Known(11));
    Ok(())
}

/// Rejected raw construction cannot replace an already admitted healthy ledger.
///
/// # Errors
/// Propagates unexpected rejection of valid source, metadata or explicit scope.
/// # Panics
/// Fails if a malformed baseline escapes admission or retained source changes.
#[test]
fn invalid_constructor_preserves_admitted_healthy_records() -> Result<(), Box<dyn Error>> {
    let endpoint = LeapHistoryBefore::new(2_335_219_200)?;
    let mut retained = LeapKnowledge::new(&EARLY_HISTORY, endpoint, None, None, identity()?)?;
    let before = retained;
    let malformed = [(2_272_060_800, 37), (2_287_785_600, 38)];
    let replacement = LeapKnowledge::new(&malformed, endpoint, None, None, identity()?);
    assert_eq!(replacement, Err(LeapError::InvalidRecords));
    if let Ok(candidate) = replacement {
        retained = candidate;
    }
    assert_eq!(retained, before);
    assert_eq!(retained.records(), &EARLY_HISTORY);
    let scope = LeapDateScope::new(window(2_272_060_800, 2_335_219_199)?)?;
    assert_answer(
        retained.interpret(window(2_287_785_600, 2_287_785_600)?, scope)?,
        LeapOffsetKnowledge::Known(11),
        LeapEventKnowledge::KnownEvents,
        1,
    );
    assert_eq!(
        retained.interpret(window(2_335_219_200, 2_335_219_200)?, scope),
        Err(LeapError::UnsupportedDates)
    );
    Ok(())
}

/// LP-C01: literal historical instants retain exact distinct offsets and identity.
///
/// # Errors
/// Propagates unexpected constructor, scope or interpretation rejection.
/// # Panics
/// Fails on a universal offset, wrong boundary, changed source or dropped labels.
#[test]
fn real_history_offsets_and_declared_identity_are_preserved() -> Result<(), Box<dyn Error>> {
    let history = knowledge(&REAL_HISTORY, REVIEWED_BEFORE, None, None)?;
    let scope = LeapDateScope::new(window(2_272_060_800, REVIEWED_BEFORE)?)?;
    for (second, expected, event_count) in [
        (2_272_060_800, 10, 0),
        (3_644_697_599, 35, 0),
        (3_644_697_600, 36, 1),
        (3_692_217_599, 36, 0),
        (3_692_217_600, 37, 1),
        (3_992_284_799, 37, 0),
    ] {
        let dates = window(second, second)?;
        let answer = history.interpret(dates, scope)?;
        let events = if event_count == 0 {
            LeapEventKnowledge::NoEvents
        } else {
            LeapEventKnowledge::KnownEvents
        };
        assert_answer(
            answer,
            LeapOffsetKnowledge::Known(expected),
            events,
            event_count,
        );
        assert_eq!(answer.dates(), dates);
        assert_eq!(answer.identity(), identity()?);
    }
    assert_eq!(history.records(), REAL_HISTORY);
    assert_eq!(history.identity().revision(), 1);
    assert_eq!(history.identity().raw_source_sha256(), [0x19; 32]);
    assert_eq!(history.identity().provenance_sha256(), [0x72; 32]);
    Ok(())
}

/// Inclusive event membership differs from offset variation at a point boundary.
///
/// # Errors
/// Propagates unexpected rejection of exact Q32 boundary neighbors.
/// # Panics
/// Fails if an endpoint event is lost or a constant after-event interval varies.
#[test]
fn fractional_transition_neighbors_keep_inclusive_event_meaning() -> Result<(), Box<dyn Error>> {
    let history = knowledge(&EARLY_HISTORY, 2_335_219_200, None, None)?;
    let scope = LeapDateScope::new(window(2_272_060_800, 2_335_219_199)?)?;
    let event = 2_287_785_600_i128 * SECOND;
    for (lower, upper, offset, events, count) in [
        (
            event - 1,
            event - 1,
            LeapOffsetKnowledge::Known(10),
            LeapEventKnowledge::NoEvents,
            0,
        ),
        (
            event,
            event,
            LeapOffsetKnowledge::Known(11),
            LeapEventKnowledge::KnownEvents,
            1,
        ),
        (
            event,
            event + 1,
            LeapOffsetKnowledge::Known(11),
            LeapEventKnowledge::KnownEvents,
            1,
        ),
        (
            event + 1,
            event + 1,
            LeapOffsetKnowledge::Known(11),
            LeapEventKnowledge::NoEvents,
            0,
        ),
        (
            event - 1,
            event,
            LeapOffsetKnowledge::Varying,
            LeapEventKnowledge::KnownEvents,
            1,
        ),
    ] {
        let dates = EraWindow::new(lower, upper)?;
        assert_answer(history.interpret(dates, scope)?, offset, events, count);
    }
    Ok(())
}

/// LP-C01: an interior positive/negative pair cannot vanish with equal endpoints.
///
/// # Errors
/// Propagates unexpected rejection of a labelled synthetic negative transition.
/// # Panics
/// Fails if endpoint equality hides either event or the intervening offset.
#[test]
fn cancelling_interior_events_still_vary_and_remain_known() -> Result<(), Box<dyn Error>> {
    let synthetic = [
        (2_272_060_800, 10),
        (2_287_785_600, 11),
        (2_303_683_200, 10),
    ];
    let history = knowledge(&synthetic, 2_335_219_200, None, None)?;
    let scope = LeapDateScope::new(window(2_272_060_800, 2_335_219_200)?)?;
    for second in [2_272_060_801, 2_303_683_201] {
        assert_eq!(
            history.interpret(window(second, second)?, scope)?.offset(),
            LeapOffsetKnowledge::Known(10)
        );
    }
    assert_answer(
        history.interpret(window(2_272_060_801, 2_303_683_201)?, scope)?,
        LeapOffsetKnowledge::Varying,
        LeapEventKnowledge::KnownEvents,
        2,
    );
    assert_answer(
        history.interpret(window(2_287_785_600, 2_335_219_200)?, scope)?,
        LeapOffsetKnowledge::Unknown,
        LeapEventKnowledge::Unknown,
        2,
    );
    Ok(())
}

/// LP-C02: historical and finite future endpoints are exclusive at exact Q32 bits.
///
/// # Errors
/// Propagates unexpected rejection of valid history and finite coverage.
/// # Panics
/// Fails on an inclusive-expiry error, erased history or fabricated infinite tail.
#[test]
fn absent_zero_and_expired_metadata_preserve_historical_knowledge() -> Result<(), Box<dyn Error>> {
    let scope = LeapDateScope::new(window(2_272_060_800, 4_100_000_000)?)?;
    for expiry in [None, Some(0), Some(2_303_683_201)] {
        let history = knowledge(&REAL_HISTORY, REVIEWED_BEFORE, expiry, None)?;
        assert_eq!(history.expiry().map(LeapFutureExpiry::ntp_seconds), expiry);
        assert_eq!(history.history_before().ntp_seconds(), REVIEWED_BEFORE);
        let end = i128::from(REVIEWED_BEFORE) * SECOND;
        assert_answer(
            history.interpret(EraWindow::new(end - 1, end - 1)?, scope)?,
            LeapOffsetKnowledge::Known(37),
            LeapEventKnowledge::NoEvents,
            0,
        );
        for (lower, upper) in [(end, end), (end - 1, end)] {
            assert_answer(
                history.interpret(EraWindow::new(lower, upper)?, scope)?,
                LeapOffsetKnowledge::Unknown,
                LeapEventKnowledge::Unknown,
                0,
            );
        }
    }
    let future = knowledge(&REAL_HISTORY, REVIEWED_BEFORE, Some(REAL_EXPIRY), None)?;
    let end = i128::from(REAL_EXPIRY) * SECOND;
    assert_answer(
        future.interpret(EraWindow::new(end - 1, end - 1)?, scope)?,
        LeapOffsetKnowledge::Known(37),
        LeapEventKnowledge::NoEvents,
        0,
    );
    assert_answer(
        future.interpret(EraWindow::new(end, end)?, scope)?,
        LeapOffsetKnowledge::Unknown,
        LeapEventKnowledge::Unknown,
        0,
    );
    Ok(())
}

/// LP-C02: a wide finite file date is not a wrapping packet-era second or infinity.
///
/// # Errors
/// Propagates unexpected rejection of an explicitly scoped synthetic 2100 bound.
/// # Panics
/// Fails if the bound wraps to 1963, disappears or becomes inclusive/infinite.
#[test]
fn wide_2100_expiry_remains_an_exact_finite_value() -> Result<(), Box<dyn Error>> {
    let expiry = 6_311_433_600;
    let history = knowledge(&REAL_HISTORY, REVIEWED_BEFORE, Some(expiry), None)?;
    let scope = LeapDateScope::new(window(2_272_060_800, expiry + 1)?)?;
    assert_eq!(
        history.expiry().map(LeapFutureExpiry::ntp_seconds),
        Some(expiry)
    );
    assert_answer(
        history.interpret(window(expiry - 1, expiry - 1)?, scope)?,
        LeapOffsetKnowledge::Known(37),
        LeapEventKnowledge::NoEvents,
        0,
    );
    assert_answer(
        history.interpret(window(expiry, expiry)?, scope)?,
        LeapOffsetKnowledge::Unknown,
        LeapEventKnowledge::Unknown,
        0,
    );
    Ok(())
}

/// LP-C03: the selected rule is a no-event boundary, with no phase/offset impulse.
///
/// # Errors
/// Propagates unexpected rejection of pinned history, expiry and effective rule.
/// # Panics
/// Fails if before/at/after or a compatible straddling interval gains an event.
#[test]
fn continuous_rule_has_no_impulse_and_carries_a_known_offset() -> Result<(), Box<dyn Error>> {
    let history = knowledge(
        &REAL_HISTORY,
        REVIEWED_BEFORE,
        Some(REAL_EXPIRY),
        Some(CONTINUOUS_FROM),
    )?;
    let scope = LeapDateScope::new(window(2_272_060_800, 6_311_433_600)?)?;
    let rule = i128::from(CONTINUOUS_FROM) * SECOND;
    assert_eq!(
        history
            .continuous_from()
            .map(LeapContinuousFrom::ntp_seconds),
        Some(CONTINUOUS_FROM)
    );
    for (lower, upper) in [
        (rule - 1, rule - 1),
        (rule, rule),
        (rule + 1, rule + 1),
        (rule - 1, rule + 1),
        (rule, 6_311_433_600_i128 * SECOND),
    ] {
        assert_answer(
            history.interpret(EraWindow::new(lower, upper)?, scope)?,
            LeapOffsetKnowledge::Known(37),
            LeapEventKnowledge::NoEvents,
            0,
        );
    }
    Ok(())
}

/// A touching exclusive/inclusive boundary bridges, but a real earlier gap does not.
///
/// # Errors
/// Propagates unexpected rejection of explicit synthetic completeness intervals.
/// # Panics
/// Fails if a gap invents an offset or adjacency incorrectly loses a known offset.
#[test]
fn continuous_rule_bridges_only_contiguous_offset_coverage() -> Result<(), Box<dyn Error>> {
    let records = [(2_272_060_800, 10)];
    let scope = LeapDateScope::new(window(2_272_060_800, CONTINUOUS_FROM + 1)?)?;
    let boundary = i128::from(CONTINUOUS_FROM) * SECOND;
    for (review, expiry) in [
        (CONTINUOUS_FROM, None),
        (REVIEWED_BEFORE, Some(CONTINUOUS_FROM)),
    ] {
        let contiguous = knowledge(&records, review, expiry, Some(CONTINUOUS_FROM))?;
        for (lower, upper) in [
            (boundary - 1, boundary),
            (boundary, boundary),
            (boundary, boundary + SECOND),
        ] {
            assert_answer(
                contiguous.interpret(EraWindow::new(lower, upper)?, scope)?,
                LeapOffsetKnowledge::Known(10),
                LeapEventKnowledge::NoEvents,
                0,
            );
        }
    }
    for (review, expiry) in [
        (CONTINUOUS_FROM - 1, None),
        (REVIEWED_BEFORE, Some(CONTINUOUS_FROM - 1)),
    ] {
        let gapped = knowledge(&records, review, expiry, Some(CONTINUOUS_FROM))?;
        assert_answer(
            gapped.interpret(EraWindow::new(boundary, boundary + SECOND)?, scope)?,
            LeapOffsetKnowledge::Unknown,
            LeapEventKnowledge::NoEvents,
            0,
        );
        assert_answer(
            gapped.interpret(EraWindow::new(boundary - 1, boundary)?, scope)?,
            LeapOffsetKnowledge::Unknown,
            LeapEventKnowledge::Unknown,
            0,
        );
    }
    Ok(())
}

/// Complete fractional candidate intervals must fit the caller's actual scope.
///
/// # Errors
/// Propagates unexpected numeric or healthy scope-construction rejection.
/// # Panics
/// Fails if either endpoint is narrowed, guessed or allowed outside scope.
#[test]
fn whole_fractional_scope_is_checked_before_interpretation() -> Result<(), Box<dyn Error>> {
    let history = knowledge(&EARLY_HISTORY, 2_335_219_200, None, None)?;
    let beginning = 2_272_060_800_i128 * SECOND;
    assert_eq!(
        LeapDateScope::new(EraWindow::new(beginning - 1, beginning)?),
        Err(LeapError::InvalidScope)
    );
    let scope = LeapDateScope::new(EraWindow::new(beginning + 1, beginning + 9)?)?;
    let good = EraWindow::new(beginning + 1, beginning + 9)?;
    assert_eq!(scope.dates(), good);
    assert_eq!(history.interpret(good, scope)?.dates(), good);
    for (lower, upper) in [
        (beginning, beginning + 1),
        (beginning + 9, beginning + 10),
        (beginning, beginning + 10),
    ] {
        assert_eq!(
            history.interpret(EraWindow::new(lower, upper)?, scope),
            Err(LeapError::UnsupportedDates)
        );
    }
    let point = EraWindow::new(beginning + 3, beginning + 3)?;
    assert_answer(
        history.interpret(point, LeapDateScope::new(point)?)?,
        LeapOffsetKnowledge::Known(10),
        LeapEventKnowledge::NoEvents,
        0,
    );
    Ok(())
}

/// Direct hostile tuples cannot bypass checked baseline, order, calendar or change.
///
/// # Errors
/// Propagates unexpected rejection of healthy neighboring construction.
/// # Panics
/// Fails when an independently invalid tuple sequence is admitted.
#[test]
fn invalid_record_classes_are_rejected_with_healthy_neighbors() -> Result<(), Box<dyn Error>> {
    for records in [
        vec![],
        vec![(2_272_060_801, 10)],
        vec![(2_272_060_800, 37)],
        vec![
            (2_272_060_800, 10),
            (2_287_785_600, 11),
            (2_287_785_600, 11),
        ],
        vec![
            (2_272_060_800, 10),
            (2_287_785_600, 11),
            (2_287_785_600, 12),
        ],
        vec![
            (2_272_060_800, 10),
            (2_303_683_200, 11),
            (2_287_785_600, 12),
        ],
        vec![(2_272_060_800, 10), (2_287_785_600, 10)],
        vec![(2_272_060_800, 10), (2_287_785_600, 12)],
        vec![(2_272_060_800, 10), (2_287_785_601, 11)],
        vec![(2_272_060_800, 10), (2_287_872_000, 11)],
        vec![(2_272_060_800, 10), (u64::MAX, 11)],
        vec![(2_272_060_800, 10), (2_287_785_600, i32::MIN)],
        vec![(2_272_060_800, 10), (2_287_785_600, i32::MAX)],
    ] {
        assert_eq!(
            knowledge(&records, REVIEWED_BEFORE, None, None),
            Err(LeapError::InvalidRecords)
        );
    }
    assert_eq!(
        knowledge(&EARLY_HISTORY, 2_272_060_800, None, None),
        Err(LeapError::InvalidRecords)
    );
    assert_eq!(
        knowledge(&EARLY_HISTORY, 2_303_683_200, None, None),
        Err(LeapError::InvalidRecords)
    );
    let healthy = knowledge(&EARLY_HISTORY, 2_303_683_201, Some(0), None)?;
    assert_eq!(healthy.records(), EARLY_HISTORY);
    Ok(())
}

/// Known event records at or after a continuous rule are contradictory data.
///
/// # Errors
/// Propagates unexpected rejection of a rule after the final known event.
/// # Panics
/// Fails if construction drops or overlooks an event conflicting with the rule.
#[test]
fn continuous_rule_conflicts_reject_instead_of_discarding_events() -> Result<(), Box<dyn Error>> {
    for effective in [2_287_785_599, 2_287_785_600, 2_303_683_200] {
        assert_eq!(
            knowledge(&EARLY_HISTORY, REVIEWED_BEFORE, None, Some(effective)),
            Err(LeapError::InvalidRecords)
        );
    }
    let healthy = knowledge(&EARLY_HISTORY, REVIEWED_BEFORE, None, Some(2_303_683_201))?;
    assert_eq!(healthy.records(), EARLY_HISTORY);
    Ok(())
}

/// Wide numeric metadata and maximum Q32 scope remain bounded, explicit inputs.
///
/// # Errors
/// Propagates unexpected rejection of legal numeric limits and baseline-only data.
/// # Panics
/// Fails on wrapping, hidden zero defaults, revision bypass or width-dependent loss.
#[test]
fn numeric_limits_and_identity_labels_are_checked_without_date_defaults()
-> Result<(), Box<dyn Error>> {
    let maximum = 9_223_372_036_854_775_807;
    for invalid in [maximum + 1, u64::MAX] {
        assert_eq!(
            LeapHistoryBefore::new(invalid),
            Err(LeapError::InvalidMetadata)
        );
        assert_eq!(
            LeapFutureExpiry::new(invalid),
            Err(LeapError::InvalidMetadata)
        );
        assert_eq!(
            LeapContinuousFrom::new(invalid),
            Err(LeapError::InvalidMetadata)
        );
    }
    for value in [0, maximum] {
        assert_eq!(LeapHistoryBefore::new(value)?.ntp_seconds(), value);
        assert_eq!(LeapFutureExpiry::new(value)?.ntp_seconds(), value);
        assert_eq!(LeapContinuousFrom::new(value)?.ntp_seconds(), value);
    }
    assert_eq!(
        LeapDataIdentity::new(0, [0; 32], [0; 32]),
        Err(LeapError::InvalidIdentity)
    );
    assert_eq!(LeapDataIdentity::new(1, [0; 32], [0; 32])?.revision(), 1);
    let records = [(2_272_060_800, 10)];
    let history = knowledge(&records, maximum, Some(0), Some(maximum))?;
    let dates = EraWindow::new(2_272_060_800_i128 * SECOND, MAX_COORDINATE_TICKS)?;
    assert_answer(
        history.interpret(dates, LeapDateScope::new(dates)?)?,
        LeapOffsetKnowledge::Known(10),
        LeapEventKnowledge::NoEvents,
        0,
    );
    Ok(())
}

/// Independently enumerate 129 monthly starts using literal 1972–1982 calendars.
///
/// There is no production calendar call or 400-year quotient formula here.
/// Fixed iteration/allocation bounds make this a maximum-size constructor control.
fn maximum_monthly_records() -> Vec<(u64, i32)> {
    let month_days = [31_u64, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut records = Vec::with_capacity(129);
    let mut second = 2_272_060_800;
    let mut month = 0;
    let mut year = 1972;
    for index in 0..129 {
        records.push((second, if index % 2 == 0 { 10 } else { 11 }));
        let days = if month == 1 && matches!(year, 1972 | 1976 | 1980) {
            29
        } else {
            month_days[month]
        };
        second += days * 86_400;
        month += 1;
        if month == 12 {
            month = 0;
            year += 1;
        }
    }
    records
}

/// The literal 128-record boundary accepts useful data while 129 is rejected.
///
/// # Errors
/// Propagates unexpected rejection of legitimate maximum-size construction.
/// # Panics
/// Fails if count rejection also rejects the legal neighbor or misses the cap.
#[test]
fn record_capacity_has_a_healthy_full_neighbor() -> Result<(), Box<dyn Error>> {
    let records = maximum_monthly_records();
    let healthy = knowledge(&records[..128], 3_000_000_000, None, None)?;
    assert_eq!(healthy.records().len(), 128);
    let dates = window(2_272_060_800, 2_999_999_999)?;
    assert_answer(
        healthy.interpret(dates, LeapDateScope::new(dates)?)?,
        LeapOffsetKnowledge::Varying,
        LeapEventKnowledge::KnownEvents,
        127,
    );
    assert_eq!(
        knowledge(&records, 3_000_000_000, None, None),
        Err(LeapError::InvalidRecords)
    );
    Ok(())
}

/// Independent civil-date literals challenge leap years and century cycle seams.
///
/// # Errors
/// Propagates unexpected rejection of legitimate synthetic monthly boundaries.
/// # Panics
/// Fails if leap days become month starts or valid century neighbors disappear.
#[test]
fn monthly_calendar_rules_cover_leap_and_century_neighbors() -> Result<(), Box<dyn Error>> {
    for (second, is_valid) in [
        (2_274_739_200, true),   // 1972-02-01
        (2_277_158_400, false),  // 1972-02-29
        (2_277_244_800, true),   // 1972-03-01
        (2_308_780_800, true),   // 1973-03-01
        (3_158_352_000, true),   // 2000-02-01
        (3_160_771_200, false),  // 2000-02-29
        (3_160_857_600, true),   // 2000-03-01
        (6_314_112_000, true),   // 2100-02-01
        (6_316_444_800, false),  // 2100-02-28
        (6_316_531_200, true),   // 2100-03-01
        (15_783_552_000, false), // 2400-02-29
        (15_783_638_400, true),  // 2400-03-01
    ] {
        let records = [(2_272_060_800, 10), (second, 11)];
        let answer = knowledge(&records, 20_000_000_000, None, None);
        if is_valid {
            assert_eq!(answer?.records(), records);
        } else {
            assert_eq!(answer, Err(LeapError::InvalidRecords));
        }
    }
    Ok(())
}

/// Finite coverage can admit a known prospective event after reviewed history.
///
/// This synthetic announcement tests the domain contract, not a real calendar
/// prediction. The event is covered only when the finite exclusive end permits it.
///
/// # Errors
/// Propagates unexpected rejection of the legal future-coverage neighbor.
/// # Panics
/// Fails if future records are always rejected, coverage is inclusive or answers
/// lose the distinct historical endpoint and prospective event.
#[test]
fn prospective_event_requires_finite_coverage_beyond_history() -> Result<(), Box<dyn Error>> {
    let records = [(2_272_060_800, 10), (2_287_785_600, 11)];
    let reviewed = 2_287_785_599;
    let expiry = 2_287_785_601;
    for insufficient in [None, Some(2_287_785_600)] {
        assert_eq!(
            knowledge(&records, reviewed, insufficient, None),
            Err(LeapError::InvalidRecords)
        );
    }
    let history = knowledge(&records, reviewed, Some(expiry), None)?;
    assert_eq!(history.history_before().ntp_seconds(), reviewed);
    assert_eq!(
        history.expiry().map(LeapFutureExpiry::ntp_seconds),
        Some(expiry)
    );
    let scope = LeapDateScope::new(window(reviewed - 1, expiry)?)?;
    let event = 2_287_785_600_i128 * SECOND;
    assert_answer(
        history.interpret(EraWindow::new(event - 1, event)?, scope)?,
        LeapOffsetKnowledge::Varying,
        LeapEventKnowledge::KnownEvents,
        1,
    );
    assert_answer(
        history.interpret(EraWindow::new(event + 1, event + 1)?, scope)?,
        LeapOffsetKnowledge::Known(11),
        LeapEventKnowledge::NoEvents,
        0,
    );
    assert_answer(
        history.interpret(window(expiry, expiry)?, scope)?,
        LeapOffsetKnowledge::Unknown,
        LeapEventKnowledge::Unknown,
        0,
    );
    Ok(())
}

/// Distinct declared revisions and digests reach the actual whole-interval result.
///
/// Identity bytes are labels only. These legal neighbors challenge accidental
/// defaulting or aliasing while the same immutable calendar remains unchanged.
///
/// # Errors
/// Propagates unexpected rejection of nonzero identities or healthy knowledge.
/// # Panics
/// Fails if any declared revision/source/provenance label is changed or coalesced.
#[test]
fn distinct_data_identities_reach_interpretation_consumers() -> Result<(), Box<dyn Error>> {
    let records = [(2_272_060_800, 10)];
    let dates = window(2_272_060_801, 2_272_060_802)?;
    let scope = LeapDateScope::new(dates)?;
    let mut previous = None;
    for (revision, source, provenance) in [
        (7, [0x3a; 32], [0x55; 32]),
        (u64::MAX, [0x3a; 32], [0xaa; 32]),
        (7, [0x5c; 32], [0x55; 32]),
    ] {
        let declared = LeapDataIdentity::new(revision, source, provenance)?;
        let history = LeapKnowledge::new(
            &records,
            LeapHistoryBefore::new(REVIEWED_BEFORE)?,
            None,
            None,
            declared,
        )?;
        let answer = history.interpret(dates, scope)?;
        assert_answer(
            answer,
            LeapOffsetKnowledge::Known(10),
            LeapEventKnowledge::NoEvents,
            0,
        );
        assert_eq!(answer.dates(), dates);
        assert_eq!(answer.identity(), declared);
        assert_eq!(answer.identity().revision(), revision);
        assert_eq!(answer.identity().raw_source_sha256(), source);
        assert_eq!(answer.identity().provenance_sha256(), provenance);
        if let Some(prior) = previous {
            assert_ne!(answer.identity(), prior);
        }
        previous = Some(answer.identity());
    }
    Ok(())
}

/// Signed negative-event trajectories can reach zero and negative exact offsets.
///
/// Literal monthly dates were independently calculated with a Gregorian calendar;
/// the synthetic signs are not claims about actual 1972 history.
///
/// # Errors
/// Propagates unexpected rejection of ordered unit negative/positive transitions.
/// # Panics
/// Fails if signed offsets are clamped, rejected or hidden by equal endpoints.
#[test]
fn negative_events_preserve_zero_and_negative_offset_knowledge() -> Result<(), Box<dyn Error>> {
    let records = [
        (2_272_060_800, 10),
        (2_274_739_200, 9),
        (2_277_244_800, 8),
        (2_279_923_200, 7),
        (2_282_515_200, 6),
        (2_285_193_600, 5),
        (2_287_785_600, 4),
        (2_290_464_000, 3),
        (2_293_142_400, 2),
        (2_295_734_400, 1),
        (2_298_412_800, 0),
        (2_301_004_800, -1),
        (2_303_683_200, 0),
    ];
    let history = knowledge(&records, 2_335_219_200, None, None)?;
    let scope = LeapDateScope::new(window(2_272_060_800, 2_335_219_199)?)?;
    for (second, offset) in [
        (2_274_739_200_u64, 9),
        (2_298_412_800, 0),
        (2_301_004_800, -1),
        (2_303_683_200, 0),
    ] {
        let after = i128::from(second) * SECOND + 1;
        assert_answer(
            history.interpret(EraWindow::new(after, after)?, scope)?,
            LeapOffsetKnowledge::Known(offset),
            LeapEventKnowledge::NoEvents,
            0,
        );
    }
    assert_answer(
        history.interpret(
            EraWindow::new(
                2_298_412_800_i128 * SECOND + 1,
                2_303_683_200_i128 * SECOND + 1,
            )?,
            scope,
        )?,
        LeapOffsetKnowledge::Varying,
        LeapEventKnowledge::KnownEvents,
        2,
    );
    Ok(())
}

/// Fixed error messages remain useful and exclude rejected identity-label bytes.
///
/// # Errors
/// Propagates unexpected acceptance of a negative control or healthy rejection.
/// # Panics
/// Fails if any real rejection formats an empty, changed or unbounded message.
#[test]
fn rejected_inputs_have_fixed_redacted_error_messages() -> Result<(), Box<dyn Error>> {
    let history = knowledge(&EARLY_HISTORY, REVIEWED_BEFORE, None, None)?;
    let scope = LeapDateScope::new(window(2_272_060_800, 2_303_683_200)?)?;
    let invalid_metadata = LeapFutureExpiry::new(u64::MAX)
        .err()
        .ok_or("oversized metadata unexpectedly accepted")?;
    let invalid_records = knowledge(&[(2_272_060_800, i32::MIN)], REVIEWED_BEFORE, None, None)
        .err()
        .ok_or("invalid baseline unexpectedly accepted")?;
    let invalid_scope = LeapDateScope::new(window(0, 1)?)
        .err()
        .ok_or("unsupported scope unexpectedly accepted")?;
    let invalid_identity = LeapDataIdentity::new(0, [b'S'; 32], [b'P'; 32])
        .err()
        .ok_or("zero revision unexpectedly accepted")?;
    let unsupported = history
        .interpret(window(2_303_683_201, 2_303_683_201)?, scope)
        .err()
        .ok_or("out-of-scope query unexpectedly accepted")?;
    for (actual, expected) in [
        (invalid_metadata, "invalid leap metadata"),
        (invalid_records, "invalid leap records"),
        (invalid_scope, "invalid leap date scope"),
        (invalid_identity, "invalid leap data identity"),
        (unsupported, "unsupported leap dates"),
    ] {
        let message = actual.to_string();
        assert_eq!(message, expected);
        assert!(message.len() <= 32);
    }
    assert_answer(
        history.interpret(window(2_287_785_600, 2_287_785_600)?, scope)?,
        LeapOffsetKnowledge::Known(11),
        LeapEventKnowledge::KnownEvents,
        1,
    );
    Ok(())
}

/// Deterministic property evidence over a closed, independently labelled domain.
///
/// Exactly five tiny signed ledgers × two history bounds × three expiry states
/// × four rule states produce 120 valid contexts. Their 21 Q32 representatives
/// yield 231 ordered endpoint pairs each: 27,720 accepted interpretations plus
/// 240 rejected scope neighbors. This is deterministic coverage, not fuzzing.
///
/// # Errors
/// Propagates any model-inventory error or unexpected real constructor rejection.
/// # Panics
/// Fails on an interpretation mismatch, vacuous truth category or wrong count.
#[test]
fn generated_intervals_match_independent_segment_truth() -> Result<(), Box<dyn Error>> {
    leap_properties::check_closed_inventory()
}

/// A Gregorian 400-year cycle preserves month starts across the scalar envelope.
///
/// The two literal dates add respectively 40 billion and 400 billion years to
/// 1 January 1972. Both are month starts; only the first fits signed-i64 seconds.
/// # Errors
/// Propagates unexpected metadata, scope or healthy-ledger rejection.
/// # Panics
/// Fails if an out-of-envelope monthly record escapes the checked coverage bound.
#[test]
fn huge_monthly_records_retain_the_checked_numeric_envelope() -> Result<(), Box<dyn Error>> {
    let healthy = [(2_272_060_800, 10), (1_262_278_082_272_060_800, 11)];
    let hostile = [(2_272_060_800, 10), (12_622_780_802_272_060_800, 11)];
    let endpoint = 9_223_372_036_854_775_807;
    let accepted = knowledge(&healthy, endpoint, None, None)?;
    assert_eq!(
        knowledge(&hostile, endpoint, None, None),
        Err(LeapError::InvalidRecords)
    );
    let scope = LeapDateScope::new(window(2_272_060_800, endpoint)?)?;
    let dates = window(1_262_278_082_272_060_800, 1_262_278_082_272_060_800)?;
    assert_answer(
        accepted.interpret(dates, scope)?,
        LeapOffsetKnowledge::Known(11),
        LeapEventKnowledge::KnownEvents,
        1,
    );
    Ok(())
}
