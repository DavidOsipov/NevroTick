//! Literal source-to-consumer checks for the compiled reviewed leap dataset.
#![forbid(unsafe_code)]

use std::error::Error;

use time_domain::{
    EraWindow, LeapDateScope, LeapEventKnowledge, LeapOffsetKnowledge, bundled_leap_knowledge,
};

/// Exact Q32 unit fixed independently of the knowledge implementation.
const SECOND: i128 = 4_294_967_296;
/// The actual reviewed source digest, independently pinned before generation.
const SOURCE_DIGEST: [u8; 32] = [
    0xdb, 0x5a, 0x89, 0x5f, 0x16, 0x85, 0x3b, 0x03, 0xbf, 0xc8, 0x65, 0xe8, 0xd6, 0x8f, 0x9f, 0xc8,
    0x71, 0x0e, 0xf1, 0x74, 0x0e, 0x34, 0x00, 0xc7, 0x01, 0xcd, 0x46, 0xa5, 0xbb, 0xbc, 0x34, 0x33,
];

/// Supply a finite test interval, without choosing a production support policy.
///
/// # Errors
/// Propagates unexpected rejection of the literal numeric bounds.
fn test_scope() -> Result<LeapDateScope, Box<dyn Error>> {
    Ok(LeapDateScope::new(EraWindow::new(
        2_272_060_800 * SECOND,
        6_311_433_600 * SECOND,
    )?)?)
}

/// LP-C01/C02: actual compiled data retains literal historical and wide answers.
///
/// # Errors
/// Propagates unexpected rejection of reviewed data or literal queries.
/// # Panics
/// Fails if generation, construction or interpretation loses the source facts.
#[test]
fn compiled_history_preserves_source_and_literal_offsets() -> Result<(), Box<dyn Error>> {
    let knowledge = bundled_leap_knowledge()?;
    let scope = test_scope()?;
    assert_eq!(knowledge.records().len(), 28);
    assert_eq!(knowledge.identity().revision(), 1);
    assert_eq!(
        knowledge.identity().raw_source_sha256().as_slice(),
        SOURCE_DIGEST.as_slice()
    );
    for (seconds, expected) in [
        (2_272_060_800_i128, 10),
        (3_644_697_600, 36),
        (3_692_217_599, 36),
        (3_692_217_600, 37),
        (4_294_967_296, 37),
        (6_311_433_600, 37),
    ] {
        let point = seconds * SECOND;
        let result = knowledge.interpret(EraWindow::new(point, point)?, scope)?;
        assert_eq!(result.offset(), LeapOffsetKnowledge::Known(expected));
    }
    Ok(())
}

/// LP-C03: the selected continuous-rule boundary is not an offset transition.
///
/// # Errors
/// Propagates unexpected rejection of valid metadata or fractional queries.
/// # Panics
/// Fails if the effective rule manufactures an event or an offset impulse.
#[test]
fn compiled_rule_has_no_impulse_and_keeps_distinct_metadata() -> Result<(), Box<dyn Error>> {
    let knowledge = bundled_leap_knowledge()?;
    assert_eq!(knowledge.history_before().ntp_seconds(), 3_992_284_800);
    assert_eq!(
        knowledge
            .expiry()
            .map(time_domain::LeapFutureExpiry::ntp_seconds),
        Some(4_023_129_600)
    );
    assert_eq!(
        knowledge
            .continuous_from()
            .map(time_domain::LeapContinuousFrom::ntp_seconds),
        Some(4_019_760_000)
    );
    let boundary = 4_019_760_000 * SECOND;
    let result = knowledge.interpret(EraWindow::new(boundary - 1, boundary + 1)?, test_scope()?)?;
    assert_eq!(result.offset(), LeapOffsetKnowledge::Known(37));
    assert_eq!(result.events(), LeapEventKnowledge::NoEvents);
    assert_eq!(result.known_transition_count(), 0);
    Ok(())
}
