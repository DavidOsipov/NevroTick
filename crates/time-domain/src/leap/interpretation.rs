//! Bounded whole-interval knowledge, without date guesses or clock authority.

use super::{
    EraWindow, LeapDateScope, LeapError, LeapEventKnowledge, LeapInterpretation, LeapKnowledge,
    LeapOffsetKnowledge, UtcCoordinateQ32, coordinate_ticks, second_ticks,
    validation::is_month_start,
};

/// Interpret only a wholly scoped interval, scanning each event record once.
///
/// Checked construction owns record and metadata invariants. This operation
/// performs no allocation, mutation, ambient reads or effects. Inclusive event
/// membership and strictly interior offset changes deliberately differ.
///
/// # Errors
/// Rejects the complete candidate interval if either endpoint is outside scope.
pub(super) fn interpret(
    knowledge: &LeapKnowledge<'_>,
    dates: EraWindow,
    scope: LeapDateScope,
) -> Result<LeapInterpretation, LeapError> {
    if dates.earliest() < scope.dates.earliest() || dates.latest() > scope.dates.latest() {
        return Err(LeapError::UnsupportedDates);
    }
    let earliest: i128 = coordinate_ticks(dates.earliest());
    let latest: i128 = coordinate_ticks(dates.latest());
    let historical_before = knowledge.history_before.ntp_seconds();
    let known_before: i128 = second_ticks(knowledge.expiry.map_or(historical_before, |expiry| {
        historical_before.max(expiry.ntp_seconds())
    }));
    let continuous_from: Option<i128> = knowledge
        .continuous
        .map(|rule| second_ticks(rule.ntp_seconds()));
    let bridged = continuous_from.is_some_and(|start| start <= known_before);
    let offsets_complete = latest < known_before || bridged;
    let events_complete = offsets_complete
        || continuous_from.is_some_and(|continuous_start| earliest >= continuous_start);

    let mut initial_offset = 10;
    let mut known_transition_count = 0;
    let mut offset_varies = false;
    for &(seconds, offset) in knowledge.records.iter().skip(1) {
        let instant = second_ticks(seconds);
        if instant <= earliest {
            initial_offset = offset;
        }
        if earliest <= instant && instant <= latest {
            known_transition_count += 1;
            if earliest < instant {
                offset_varies = true;
            }
        }
    }
    let offset = if !offsets_complete {
        LeapOffsetKnowledge::Unknown
    } else if offset_varies {
        LeapOffsetKnowledge::Varying
    } else {
        LeapOffsetKnowledge::Known(initial_offset)
    };
    let events = if !events_complete {
        LeapEventKnowledge::Unknown
    } else if known_transition_count == 0 {
        LeapEventKnowledge::NoEvents
    } else {
        LeapEventKnowledge::KnownEvents
    };
    Ok(LeapInterpretation {
        dates,
        identity: knowledge.identity,
        offset,
        events,
        known_transition_count,
        first_uncovered_month_boundary: first_uncovered_month_boundary(
            earliest.max(known_before),
            latest,
            continuous_from,
        )?,
    })
}

/// Search the intersection of the inclusive date window with uncovered UTC.
/// `lower` is already clipped to the exclusive coverage end. A continuous
/// rule covers its own first tick. All arguments are admitted Q32 ticks.
///
/// # Errors
/// Returns `UnsupportedDates` if a candidate fails checked coordinate conversion.
fn first_uncovered_month_boundary(
    lower: i128,
    latest: i128,
    continuous_from: Option<i128>,
) -> Result<Option<UtcCoordinateQ32>, LeapError> {
    let day: i128 = second_ticks(86_400);
    // The scoped coordinates leave room in i128 to round up, even when the
    // next midnight exceeds the latest representable date.
    let mut midnight: i128 = lower + (-lower).rem_euclid(day);
    // Every Gregorian month has at most 31 days. From any UTC midnight the
    // next eligible month start is at most 30 further day steps away.
    for _ in 0..31 {
        if midnight > latest || continuous_from.is_some_and(|start| midnight >= start) {
            return Ok(None);
        }
        let seconds: u64 =
            u64::try_from(midnight / second_ticks(1)).map_err(|_| LeapError::UnsupportedDates)?;
        if is_month_start(seconds) {
            return UtcCoordinateQ32::new(midnight)
                .map(Some)
                .map_err(|_| LeapError::UnsupportedDates);
        }
        midnight += day;
    }
    Ok(None)
}
