//! Bounded whole-interval knowledge, without date guesses or clock authority.

use super::{
    EraWindow, LeapDateScope, LeapError, LeapEventKnowledge, LeapInterpretation, LeapKnowledge,
    LeapOffsetKnowledge, coordinate_ticks, second_ticks,
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
    let earliest = coordinate_ticks(dates.earliest());
    let latest = coordinate_ticks(dates.latest());
    let historical_before = knowledge.history_before.ntp_seconds();
    let known_before = second_ticks(knowledge.expiry.map_or(historical_before, |expiry| {
        historical_before.max(expiry.ntp_seconds())
    }));
    let continuous_from = knowledge
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
    })
}
