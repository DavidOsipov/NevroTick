//! Exact arithmetic and boundary controls with synthetic monotonic captures.

use super::{ExactInterval, IntervalError, IntervalPolicy, project};
use std::{
    num::NonZeroU32,
    time::{Duration, Instant},
};
use time_domain::EraWindow;

/// Keep fixture setup failures distinct from expected projection refusals.
type TestResult = Result<(), &'static str>;

/// Configure both exact rate factors with zero capture margin.
///
/// # Errors
/// Returns an unexpected zero rate or invalid policy separately.
fn policy(
    rates: [u32; 2],
    max_age: Duration,
    max_width: Duration,
) -> Result<IntervalPolicy, &'static str> {
    let lower: NonZeroU32 = NonZeroU32::new(rates[0]).ok_or("zero fixture lower rate")?;
    let upper: NonZeroU32 = NonZeroU32::new(rates[1]).ok_or("zero fixture upper rate")?;
    IntervalPolicy::new([lower, upper], Duration::ZERO, max_age, max_width)
        .map_err(|_| "unexpected fixture policy refusal")
}

/// Place a fixture observation at a checked offset from one monotonic origin.
///
/// # Errors
/// Returns a setup error if the host cannot represent the fixture instant.
fn at(origin: Instant, elapsed: Duration) -> Result<Instant, &'static str> {
    origin
        .checked_add(elapsed)
        .ok_or("fixture capture out of range")
}

/// Preserve every possible processing instant from send through receive.
///
/// # Errors
/// Returns an unexpected setup or projection failure.
///
/// # Panics
/// Panics if projection drops transport uncertainty or uses the wrong time units.
#[test]
fn projects_the_entire_processing_bracket_at_exact_rate() -> TestResult {
    let origin: Instant = Instant::now();
    let send: Instant = at(origin, Duration::from_secs(10))?;
    let receive: Instant = at(origin, Duration::from_secs(14))?;
    let evaluated_at: Instant = at(origin, Duration::from_secs(16))?;
    let limits: IntervalPolicy = policy(
        [1_000_000_000; 2],
        Duration::from_secs(7),
        Duration::from_secs(9),
    )?;
    let projected: ExactInterval = project((998, 1_002), send, receive, evaluated_at, &limits)
        .map_err(|_| "unexpected exact-rate projection refusal")?;

    assert_eq!(projected.lower, 1_000_000_000_000_000_000_000);
    assert_eq!(projected.upper, 1_008_000_000_000_000_000_000);
    assert_eq!(
        projected.outward_ns(),
        (1_000_000_000_000, 1_008_000_000_000)
    );
    let padding: IntervalPolicy = IntervalPolicy::new(
        limits.rates,
        Duration::from_millis(250),
        Duration::from_secs(7),
        Duration::from_secs(9),
    )
    .map_err(|_| "unexpected padded fixture policy refusal")?;
    let extended: ExactInterval = project((998, 1_002), send, receive, evaluated_at, &padding)
        .map_err(|_| "unexpected padded projection refusal")?;
    assert_eq!(extended.outward_ns(), (999_750_000_000, 1_008_250_000_000));
    Ok(())
}

/// Preserve exact subnanosecond bounds and round outward only at publication.
///
/// # Errors
/// Returns an unexpected setup or projection failure.
///
/// # Panics
/// Panics if the implementation truncates either rate contribution too early.
#[test]
fn projects_fractional_rate_and_rounds_both_endpoints_outward() -> TestResult {
    let origin: Instant = Instant::now();
    let receive: Instant = at(origin, Duration::from_nanos(1))?;
    let evaluated_at: Instant = at(origin, Duration::from_nanos(2))?;
    let limits: IntervalPolicy = policy(
        [999_999_999, 1_000_000_001],
        Duration::from_secs(1),
        Duration::from_secs(3),
    )?;
    let projected: ExactInterval = project((100, 102), origin, receive, evaluated_at, &limits)
        .map_err(|_| "unexpected fractional-rate projection refusal")?;

    // The possible advances are 0.999999999 ns through 2.000000002 ns.
    assert_eq!(projected.lower, 100_000_000_000_999_999_999);
    assert_eq!(projected.upper, 102_000_000_002_000_000_002);
    assert_eq!(projected.outward_ns(), (100_000_000_000, 102_000_000_003));
    Ok(())
}

/// Compare strict exact intersection near an outward-rounded overlap.
///
/// # Errors
/// Returns a missing positive-width exact intersection.
///
/// # Panics
/// Panics if touching or disjoint intervals acquire apparent common support.
#[test]
fn touching_and_disjoint_intervals_cannot_gain_support_from_rounding() -> TestResult {
    const LEFT: ExactInterval = ExactInterval {
        lower: 1_800_000_001,
        upper: 2_400_000_001,
    };
    const TOUCHING: ExactInterval = ExactInterval {
        lower: 2_400_000_001,
        upper: 3_300_000_001,
    };
    const DISJOINT: ExactInterval = ExactInterval {
        lower: 2_400_000_002,
        upper: 3_300_000_001,
    };
    const OVERLAPPING: ExactInterval = ExactInterval {
        lower: 2_400_000_000,
        upper: 3_300_000_001,
    };
    assert_eq!(LEFT.outward_ns(), (1, 3));
    assert_eq!(TOUCHING.outward_ns(), (2, 4));
    assert_eq!(DISJOINT.outward_ns(), (2, 4));
    assert!(LEFT.intersect(TOUCHING).is_none());
    assert!(LEFT.intersect(DISJOINT).is_none());
    let common: ExactInterval = LEFT
        .intersect(OVERLAPPING)
        .ok_or("lost a positive exact intersection")?;
    assert_eq!(common.lower, 2_400_000_000);
    assert_eq!(common.upper, 2_400_000_001);
    Ok(())
}

/// Refuse reordered captures and expire age at the original send.
///
/// # Errors
/// Returns an unexpected setup failure.
///
/// # Panics
/// Panics if projection renews age from the receive observation or accepts future captures.
#[test]
fn checks_capture_order_and_exclusive_age_from_send() -> TestResult {
    let origin: Instant = Instant::now();
    let receive: Instant = at(origin, Duration::from_secs(2))?;
    let healthy: Instant = at(origin, Duration::from_secs(3))?;
    let one_tick_future: Instant = at(healthy, Duration::from_nanos(1))?;
    let expired: Instant = at(origin, Duration::from_secs(4))?;
    let limits: IntervalPolicy = policy(
        [1_000_000_000; 2],
        Duration::from_secs(4),
        Duration::from_secs(5),
    )?;

    assert!(project((100, 102), origin, receive, healthy, &limits).is_ok());
    assert!(project((100, 102), origin, receive, receive, &limits).is_ok());
    assert!(matches!(
        project((100, 102), origin, one_tick_future, healthy, &limits),
        Err(IntervalError::CaptureOrder)
    ));
    assert!(matches!(
        project((100, 102), receive, origin, healthy, &limits),
        Err(IntervalError::CaptureOrder)
    ));
    assert!(matches!(
        project((100, 102), origin, expired, healthy, &limits),
        Err(IntervalError::CaptureOrder)
    ));
    assert!(matches!(
        project((100, 102), origin, receive, expired, &limits),
        Err(IntervalError::Stale)
    ));
    Ok(())
}

/// Preserve useful-width admission at the inclusive threshold.
///
/// # Errors
/// Returns an unexpected setup failure.
///
/// # Panics
/// Panics if elapsed uncertainty is excluded from the width check.
#[test]
fn checks_width_after_transport_from_the_original_bracket() -> TestResult {
    let origin: Instant = Instant::now();
    let receive: Instant = at(origin, Duration::from_secs(2))?;
    let evaluated_at: Instant = at(origin, Duration::from_secs(3))?;
    let limits: IntervalPolicy = policy(
        [1_000_000_000; 2],
        Duration::from_secs(4),
        Duration::from_secs(4),
    )?;

    // Both signed spans fit alone. Transport adds two seconds of uncertainty.
    assert!(project((100, 102), origin, receive, evaluated_at, &limits).is_ok());
    assert!(matches!(
        project((99, 102), origin, receive, evaluated_at, &limits),
        Err(IntervalError::TooWide)
    ));
    Ok(())
}

/// Check the exported upper endpoint at the unsigned Unix boundary.
///
/// # Errors
/// Returns an unexpected setup or neighboring projection failure.
///
/// # Panics
/// Panics if ceiling rounding can publish an invalid Unix endpoint.
#[test]
fn refuses_unix_overflow_caused_only_by_outward_rounding() -> TestResult {
    let origin: Instant = Instant::now();
    let last_representable: Instant = at(origin, Duration::from_nanos(999_999_998))?;
    let outside_after_rounding: Instant = at(origin, Duration::from_nanos(999_999_999))?;
    let limits: IntervalPolicy = policy(
        [1_000_000_001; 2],
        Duration::from_secs(2),
        Duration::from_secs(3),
    )?;
    let signed: (u64, u64) = (u64::MAX - 2, u64::MAX);
    let projected: ExactInterval = project(signed, origin, origin, last_representable, &limits)
        .map_err(|_| "lost the last representable outward bound")?;
    assert_eq!(
        projected.outward_ns().1,
        18_446_744_073_709_551_615_999_999_999
    );
    assert!(matches!(
        project(signed, origin, origin, outside_after_rounding, &limits),
        Err(IntervalError::Arithmetic)
    ));
    Ok(())
}

/// Preserve the full unfolded coordinates across the NTP era boundary.
///
/// # Errors
/// Returns an unexpected refusal of either exact anchor interval.
///
/// # Panics
/// Panics if the conversion uses the wrong epoch, units or era.
#[test]
fn anchors_unix_epoch_and_preserves_the_ntp_era_crossing() -> TestResult {
    const UNIX_EPOCH: ExactInterval = ExactInterval {
        lower: 0,
        upper: 500_000_000_000_000_000,
    };
    // A half second on each side of the first 32-bit NTP seconds rollover.
    const ERA_CROSSING: ExactInterval = ExactInterval {
        lower: 2_085_978_495_500_000_000_000_000_000,
        upper: 2_085_978_496_500_000_000_000_000_000,
    };
    let epoch: EraWindow = UNIX_EPOCH
        .ntp_window()
        .map_err(|_| "lost the Unix epoch anchor interval")?;
    assert_eq!(epoch.earliest().ticks(), 9_487_534_653_230_284_800);
    assert_eq!(epoch.latest().ticks(), 9_487_534_655_377_768_448);

    let crossing: EraWindow = ERA_CROSSING
        .ntp_window()
        .map_err(|_| "lost the exact 2036 era-crossing interval")?;
    assert_eq!(crossing.earliest().ticks(), 18_446_744_071_562_067_968);
    assert_eq!(crossing.latest().ticks(), 18_446_744_075_857_035_264);
    Ok(())
}

/// Enclose both fractional endpoints in the immediately adjacent Q32 ticks.
///
/// # Errors
/// Returns an unexpected refusal of the positive sub-tick interval.
///
/// # Panics
/// Panics for inward rounding or extra widening via integer-nanosecond conversion.
#[test]
fn converts_sub_tick_endpoints_directly_and_outward() -> TestResult {
    // Both endpoints are just above the exact half-second tick at the Unix epoch.
    const ABOVE_HALF_SECOND: ExactInterval = ExactInterval {
        lower: 500_000_000_000_000_001,
        upper: 500_000_000_000_000_002,
    };
    let enclosure: EraWindow = ABOVE_HALF_SECOND
        .ntp_window()
        .map_err(|_| "lost the positive sub-tick interval")?;
    assert_eq!(enclosure.earliest().ticks(), 9_487_534_655_377_768_448);
    assert_eq!(enclosure.latest().ticks(), 9_487_534_655_377_768_449);
    Ok(())
}

/// Check the outward upper tick against the signed whole-second envelope.
///
/// # Errors
/// Returns an unexpected refusal of the last representable neighbor.
///
/// # Panics
/// Panics if ceiling rounding admits an unsupported tick or rejects the neighbor.
#[test]
fn refuses_outward_ceiling_past_the_last_ntp_tick() -> TestResult {
    // These adjacent upper inputs straddle the last representable Q32 tick.
    const IN_RANGE: ExactInterval = ExactInterval {
        lower: 9_223_372_034_645_787_007_000_000_000_000_000_000,
        upper: 9_223_372_034_645_787_007_999_999_999_767_169_356,
    };
    const ABOVE_RANGE: ExactInterval = ExactInterval {
        lower: IN_RANGE.lower,
        upper: 9_223_372_034_645_787_007_999_999_999_767_169_357,
    };
    let window: EraWindow = IN_RANGE
        .ntp_window()
        .map_err(|_| "lost the last representable NTP tick")?;
    assert_eq!(
        window.earliest().ticks(),
        39_614_081_257_132_168_792_477_007_872
    );
    assert_eq!(
        window.latest().ticks(),
        39_614_081_257_132_168_796_771_975_167
    );
    assert_eq!(ABOVE_RANGE.ntp_window(), Err(IntervalError::Arithmetic));
    Ok(())
}
