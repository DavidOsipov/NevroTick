// SPDX-License-Identifier: GPL-2.0-only
//! Independent literal arithmetic and invalid/valid neighbors at the consumer.

use std::time::{Duration, Instant, UNIX_EPOCH};

use garde::Validate;
use ntp_wire::{RawHeader, decode_fixed_header};
use time_domain::{
    MAX_COORDINATE_TICKS, MIN_COORDINATE_TICKS, UtcCoordinateQ32, exchange_arithmetic,
};

use super::{
    ExperimentalProfile, MAX_DECLARED_RESIDUAL_NS, MeasurementRefusal, NANOS_PER_SECOND,
    NOMINAL_NUMERATOR_LIMIT, NTP_UNIX_OFFSET_SECONDS, NominalResultCandidate, Q32_SCALE,
    SoftwareClock, coherence_residual, convert_wall, measure, nominal_arithmetic, require_in_scope,
};

/// Nonzero NTP-era-zero base, independent of the host's current UTC reading.
const BASE: u64 = 4_000_000_000;
/// Exact whole-second raw timestamp for the common fixture base.
const BASE_BITS: u64 = BASE << 32;

/// Build captures on the Windows-compatible 100 ns wall-representation grid.
/// Fixture construction reads no wall clock and declares no physical accuracy.
///
/// # Errors
/// Rejects a noncanonical or non-100-ns-aligned fraction, or an unrepresentable
/// checked monotonic/system-wall fixture time on the current platform.
fn capture(
    base: Instant,
    mono_ns: u64,
    ntp_seconds: u64,
    nanos: u64,
) -> Result<SoftwareClock, MeasurementRefusal> {
    if nanos >= 1_000_000_000 || !nanos.is_multiple_of(100) {
        return Err(MeasurementRefusal::Arithmetic);
    }
    let mono = base
        .checked_add(Duration::from_nanos(mono_ns))
        .ok_or(MeasurementRefusal::Arithmetic)?;
    let wall = if let Some(seconds) = ntp_seconds.checked_sub(2_208_988_800) {
        UNIX_EPOCH.checked_add(Duration::from_secs(seconds))
    } else {
        UNIX_EPOCH.checked_sub(Duration::from_secs(2_208_988_800 - ntp_seconds))
    }
    .and_then(|value| value.checked_add(Duration::from_nanos(nanos)))
    .ok_or(MeasurementRefusal::Arithmetic)?;
    Ok(SoftwareClock {
        mono,
        utc: wall,
        mono_after: mono,
    })
}

/// Encode exact inert peer bits and an arbitrary independent matching token.
///
/// # Errors
/// Returns `Protocol` for an unexpected decoder rejection of the complete
/// fixed-size 48-byte fixture.
fn header(receive: u64, transmit: u64, wire_token: u64) -> Result<RawHeader, MeasurementRefusal> {
    let mut bytes = [0; 48];
    bytes[0] = 0x24;
    bytes[1] = 1;
    bytes[24..32].copy_from_slice(&wire_token.to_be_bytes());
    bytes[32..40].copy_from_slice(&receive.to_be_bytes());
    bytes[40..48].copy_from_slice(&transmit.to_be_bytes());
    decode_fixed_header(&bytes).map_err(|_| MeasurementRefusal::Protocol)
}

/// Every refusal has one literal redacted code shared by both projections.
///
/// # Panics
/// Panics if a variant changes its exact code or its display adds other data.
#[test]
fn refusal_codes_and_display_remain_literal_and_redacted() {
    for (reason, code) in [
        (MeasurementRefusal::Unavailable, "mapping_unavailable"),
        (MeasurementRefusal::InvalidProfile, "invalid_profile"),
        (MeasurementRefusal::CaptureOrder, "capture_order"),
        (
            MeasurementRefusal::MappingDiscontinuity,
            "mapping_discontinuity",
        ),
        (MeasurementRefusal::DateScope, "date_scope"),
        (MeasurementRefusal::EraNoCandidate, "era_no_candidate"),
        (MeasurementRefusal::EraAmbiguous, "era_ambiguous"),
        (MeasurementRefusal::Arithmetic, "arithmetic"),
        (MeasurementRefusal::Protocol, "not_time_reply"),
    ] {
        assert_eq!(reason.as_str(), code);
        assert_eq!(reason.to_string(), code);
    }
}

/// Each of the five nominal coordinates can own either software-window end.
/// The explicit tolerance admits these software readings without asserting
/// physical continuity; monotonic captures retain chronological order.
///
/// # Errors
/// Propagates fixture/profile construction or unexpected measurement refusal.
///
/// # Panics
/// Panics if any local or peer point is omitted from the literal window ends.
#[test]
fn software_window_retains_each_local_and_peer_extremum() -> Result<(), MeasurementRefusal> {
    let start = Instant::now();
    let profile = Some(ExperimentalProfile::new(BASE, BASE + 5, 30_000_000_000)?);
    // Columns are T1, send-after, T2, T3, T4, in whole seconds above BASE.
    // Each column supplies the unique zero and unique four in one row.
    for [t1, after, t2, t3, t4] in [
        [0, 1, 2, 3, 4],
        [4, 0, 1, 2, 3],
        [3, 4, 0, 1, 2],
        [2, 3, 4, 0, 1],
        [1, 2, 3, 4, 0],
    ] {
        let before = capture(start, 0, BASE + t1, 0)?;
        let after = capture(start, 1_000_000_000, BASE + after, 0)?;
        let read = capture(start, 2_000_000_000, BASE + t4, 0)?;
        let result = measure(
            profile,
            before,
            after,
            read,
            read.mono_after,
            header(BASE_BITS + (t2 << 32), BASE_BITS + (t3 << 32), 1)?,
        )?;
        assert_eq!(
            result.software_window().earliest().ticks(),
            17_179_869_184_000_000_000,
        );
        assert_eq!(
            result.software_window().latest().ticks(),
            17_179_869_201_179_869_184,
        );
    }
    Ok(())
}

/// The supplied dyadic exchange has literal +1/64-second phase and 5/64 RTT.
///
/// # Errors
/// Propagates fixture/profile construction failures or an unexpected refusal
/// of either healthy dyadic measurement.
///
/// # Panics
/// Panics if changing the token changes arithmetic, or literal phase, RTT,
/// residual and acquisition-age expectations differ from the result.
#[test]
fn exact_dyadic_results_ignore_the_wire_token() -> Result<(), MeasurementRefusal> {
    let start = Instant::now();
    let before = capture(start, 0, BASE, 0)?;
    let after = capture(start, 1_000_000, BASE, 1_000_000)?;
    let read = capture(start, 85_937_500, BASE, 85_937_500)?;
    let processing = start
        .checked_add(Duration::from_millis(100))
        .ok_or(MeasurementRefusal::Arithmetic)?;
    let profile = Some(ExperimentalProfile::new(BASE, BASE + 1, 0)?);
    let first = measure(
        profile,
        before,
        after,
        read,
        processing,
        header(BASE_BITS + (7 << 25), BASE_BITS + (8 << 25), 1)?,
    )?;
    let second = measure(
        profile,
        before,
        after,
        read,
        processing,
        header(BASE_BITS + (7 << 25), BASE_BITS + (8 << 25), u64::MAX)?,
    )?;
    assert_eq!(first, second);
    assert_eq!(first.arithmetic().offset().numerator(), 134_217_728);
    assert_eq!(first.arithmetic().round_trip().numerator(), 335_544_320);
    assert_eq!(first.local_fractional_remainders(), [0, 0]);
    assert_eq!(first.observed_residual_ns(), 0);
    assert_eq!(first.acquisition_age(), Duration::from_nanos(14_062_500));
    Ok(())
}

/// Equal peer shifts reverse phase without altering the independent RTT.
///
/// # Errors
/// Propagates fixture/profile construction failures or an unexpected refusal
/// of the shifted healthy measurement.
///
/// # Panics
/// Panics if the common peer shift fails to reverse phase or changes raw RTT.
#[test]
fn peer_shift_changes_sign_without_changing_rtt() -> Result<(), MeasurementRefusal> {
    let start = Instant::now();
    let before = capture(start, 0, BASE, 0)?;
    let after = capture(start, 0, BASE, 0)?;
    let read = capture(start, 85_937_500, BASE, 85_937_500)?;
    let result = measure(
        Some(ExperimentalProfile::new(BASE, BASE + 1, 0)?),
        before,
        after,
        read,
        read.mono_after,
        header(BASE_BITS + (3 << 25), BASE_BITS + (4 << 25), 7)?,
    )?;
    assert_eq!(result.arithmetic().offset().numerator(), -134_217_728);
    assert_eq!(result.arithmetic().round_trip().numerator(), 335_544_320);
    Ok(())
}

/// Short local elapsed time preserves a negative signed raw network RTT.
///
/// # Errors
/// Propagates fixture/profile construction failures or an unexpected refusal
/// of the negative-RTT or half-tick measurement.
///
/// # Panics
/// Panics if negative raw RTT or the odd phase numerator is clamped or changed.
#[test]
fn negative_raw_rtt_and_half_q32_phase_are_preserved() -> Result<(), MeasurementRefusal> {
    let start = Instant::now();
    let before = capture(start, 0, BASE, 0)?;
    let read = capture(start, 7_812_500, BASE, 7_812_500)?;
    let profile = Some(ExperimentalProfile::new(BASE, BASE + 1, 0)?);
    let negative = measure(
        profile,
        before,
        before,
        read,
        read.mono_after,
        header(BASE_BITS + (7 << 25), BASE_BITS + (9 << 25), 8)?,
    )?;
    assert_eq!(negative.arithmetic().round_trip().numerator(), -33_554_432);
    let half = measure(
        profile,
        before,
        before,
        before,
        before.mono_after,
        header(BASE_BITS, BASE_BITS + 1, 8)?,
    )?;
    assert_eq!(half.arithmetic().offset().numerator(), 1);
    assert_eq!(half.arithmetic().round_trip().numerator(), -1);
    Ok(())
}

/// Actual measurements distinguish a zero-remainder control from a 100 ns source.
/// Both peer timestamps are the local Q32 floor, so only retained decimal
/// remainders account for the nonzero exact nominal phase.
///
/// # Errors
/// Propagates fixture construction or unexpected healthy measurement refusal.
///
/// # Panics
/// Panics if units change or the real producer drops its local conversion terms.
#[test]
fn decimal_remainders_change_the_exact_nominal_phase() -> Result<(), MeasurementRefusal> {
    let start = Instant::now();
    let profile = Some(ExperimentalProfile::new(BASE, BASE + 1, 0)?);
    let whole = capture(start, 0, BASE, 0)?;
    let healthy = measure(
        profile,
        whole,
        whole,
        whole,
        start,
        header(BASE_BITS, BASE_BITS, 1)?,
    )?;
    assert_eq!(healthy.nominal_phase().numerator(), 0);
    assert_eq!(healthy.nominal_round_trip().numerator(), 0);
    assert_eq!(healthy.nominal_phase().denominator(), 8_589_934_592);
    assert_eq!(healthy.nominal_round_trip().denominator(), 4_294_967_296);

    let fractional = capture(start, 0, BASE, 100)?;
    let result = measure(
        profile,
        fractional,
        fractional,
        fractional,
        start,
        header(BASE_BITS + 429, BASE_BITS + 429, 2)?,
    )?;
    assert_eq!(result.arithmetic().offset().numerator(), 0);
    assert_eq!(result.local_fractional_remainders(), [496_729_600; 2]);
    assert_eq!(
        result.nominal_phase().numerator(),
        -993_459_200,
        "exact nominal phase dropped local conversion remainders"
    );
    assert_eq!(result.nominal_phase().denominator(), 8_589_934_592);
    assert_eq!(result.nominal_round_trip().numerator(), 0);
    assert_eq!(result.nominal_round_trip().denominator(), 4_294_967_296);
    Ok(())
}

/// Literal direct-timestamp facts retain asymmetric remainder and raw-RTT signs.
/// Monotonic progress is zero; the declared 200 ns tolerance admits either
/// direction of these synthetic wall readings without physical accuracy claims.
///
/// # Errors
/// Propagates checked fixture/profile construction or unexpected refusal.
///
/// # Panics
/// Panics if an exact relative value loses sign, scale or decimal precision.
#[test]
fn asymmetric_decimal_captures_keep_exact_signed_nominal_values() -> Result<(), MeasurementRefusal>
{
    let start = Instant::now();
    let profile = Some(ExperimentalProfile::new(BASE, BASE + 1, 200)?);
    // T1/T4 are integer nanoseconds above BASE; T2/T3 are exact Q32 fractions.
    // Expected values were independently reduced from those original units.
    for (t1_ns, t4_ns, t2_fraction, t3_fraction, phase, round_trip) in [
        (0, 100, 429, 429, 428_503_270_400, 429_496_729_600),
        (100, 200, 429, 429, -430_490_188_800, 429_496_729_600),
        (200, 100, 858, 858, 427_509_811_200, -429_496_729_600),
        (100, 200, 429, 858, -1_490_188_800, 496_729_600),
        (200, 100, 429, 858, -1_490_188_800, -858_496_729_600),
    ] {
        let before = capture(start, 0, BASE, t1_ns)?;
        let read = capture(start, 0, BASE, t4_ns)?;
        let result = measure(
            profile,
            before,
            before,
            read,
            start,
            header(BASE_BITS + t2_fraction, BASE_BITS + t3_fraction, 3)?,
        )?;
        assert_eq!(result.nominal_phase().numerator(), phase);
        assert_eq!(result.nominal_phase().denominator(), 8_589_934_592);
        assert_eq!(result.nominal_round_trip().numerator(), round_trip);
        assert_eq!(result.nominal_round_trip().denominator(), 4_294_967_296);
    }
    Ok(())
}

/// Measurements immediately before/after Unix retain different floor residuals.
/// The returned phase compares the floor-Q32 peer with the exact local wall time.
///
/// # Errors
/// Propagates checked epoch-neighbor capture/profile construction or refusal.
///
/// # Panics
/// Panics if epoch association or a remainder's signed contribution changes.
#[test]
fn pre_unix_nominal_remainders_keep_epoch_and_sign_association() -> Result<(), MeasurementRefusal> {
    let start = Instant::now();
    let epoch_bits = 2_208_988_800_u64 << 32;
    let profile = Some(ExperimentalProfile::new(2_208_988_799, 2_208_988_801, 0)?);
    for (seconds, nanos, peer_bits, remainder, phase) in [
        (
            2_208_988_799,
            999_999_900,
            epoch_bits - 430,
            503_270_400,
            -1_006_540_800,
        ),
        (
            2_208_988_800,
            100,
            epoch_bits + 429,
            496_729_600,
            -993_459_200,
        ),
    ] {
        let clock = capture(start, 0, seconds, nanos)?;
        let result = measure(
            profile,
            clock,
            clock,
            clock,
            start,
            header(peer_bits, peer_bits, 4)?,
        )?;
        assert_eq!(result.arithmetic().offset().numerator(), 0);
        assert_eq!(result.local_fractional_remainders(), [remainder; 2]);
        assert_eq!(result.nominal_phase().numerator(), phase);
        assert_eq!(result.nominal_phase().denominator(), 8_589_934_592);
        assert_eq!(result.nominal_round_trip().numerator(), 0);
        assert_eq!(result.nominal_round_trip().denominator(), 4_294_967_296);
    }
    Ok(())
}

/// Raw-helper range checks run before publishing either typed result.
/// These abstract remainder boundary cases are not claims of actual captures.
///
/// # Errors
/// Propagates legitimate zero-coordinate arithmetic or a valid helper refusal.
///
/// # Panics
/// Panics if an invalid pair is accepted or a valid neighbor is changed/rejected.
#[test]
fn nominal_arithmetic_rejects_each_remainder_boundary_without_partial_result()
-> Result<(), MeasurementRefusal> {
    let zero = UtcCoordinateQ32::new(0).map_err(|_| MeasurementRefusal::Arithmetic)?;
    let arithmetic =
        exchange_arithmetic(zero, zero, zero, zero).map_err(|_| MeasurementRefusal::Arithmetic)?;
    for (remainders, phase, round_trip) in [
        ([0, 0], 0, 0),
        ([999_999_999, 999_999_999], -1_999_999_998, 0),
        ([999_999_999, 0], -999_999_999, -999_999_999),
        ([0, 999_999_999], -999_999_999, 999_999_999),
    ] {
        let (actual_phase, actual_round_trip) = nominal_arithmetic(arithmetic, remainders)?;
        assert_eq!(actual_phase.numerator(), phase);
        assert_eq!(actual_phase.denominator(), 8_589_934_592);
        assert_eq!(actual_round_trip.numerator(), round_trip);
        assert_eq!(actual_round_trip.denominator(), 4_294_967_296);
    }
    for remainders in [
        [1_000_000_000, 0],
        [0, 1_000_000_000],
        [u32::MAX, 0],
        [0, u32::MAX],
    ] {
        assert_eq!(
            nominal_arithmetic(arithmetic, remainders),
            Err(MeasurementRefusal::Arithmetic)
        );
    }
    assert_eq!(nominal_arithmetic(arithmetic, [0, 0])?.0.numerator(), 0);
    Ok(())
}

/// The actual output validator owns both strict signed-numerator bounds.
/// Raw candidate tests establish that guard, not an impossible public capture.
///
/// # Panics
/// Panics if either field escapes its own guard or valid boundary neighbors fail.
#[test]
fn nominal_result_garde_enforces_both_strict_numerator_bounds() {
    for valid in [0, -NOMINAL_NUMERATOR_LIMIT + 1, NOMINAL_NUMERATOR_LIMIT - 1] {
        assert!(
            NominalResultCandidate {
                phase: valid,
                round_trip: valid,
            }
            .validate()
            .is_ok()
        );
    }
    for invalid in [
        i128::MIN,
        -NOMINAL_NUMERATOR_LIMIT,
        NOMINAL_NUMERATOR_LIMIT,
        i128::MAX,
    ] {
        for candidate in [
            NominalResultCandidate {
                phase: invalid,
                round_trip: 0,
            },
            NominalResultCandidate {
                phase: 0,
                round_trip: invalid,
            },
        ] {
            assert!(candidate.validate().is_err());
        }
    }
}

/// The checked component preserves the complete coordinate arithmetic envelope.
/// These are raw numerical-helper controls, not supported-date or wire admission.
///
/// # Errors
/// Propagates valid coordinate/arithmetic construction or helper rejection.
///
/// # Panics
/// Panics if an intermediate i64 projection, sign swap or extra scaling loses data.
#[test]
fn nominal_arithmetic_retains_full_coordinate_domain_without_i64_projection()
-> Result<(), MeasurementRefusal> {
    let low =
        UtcCoordinateQ32::new(MIN_COORDINATE_TICKS).map_err(|_| MeasurementRefusal::Arithmetic)?;
    let high =
        UtcCoordinateQ32::new(MAX_COORDINATE_TICKS).map_err(|_| MeasurementRefusal::Arithmetic)?;
    for (coordinates, remainders, phase, round_trip) in [
        (
            [low, high, high, low],
            [0, 0],
            158_456_325_028_528_675_187_087_900_670_000_000_000,
            0,
        ),
        (
            [high, low, low, high],
            [999_999_999, 999_999_999],
            -158_456_325_028_528_675_187_087_900_671_999_999_998,
            0,
        ),
        (
            [low, high, low, high],
            [0, 999_999_999],
            -999_999_999,
            158_456_325_028_528_675_187_087_900_670_999_999_999,
        ),
        (
            [high, low, high, low],
            [999_999_999, 0],
            -999_999_999,
            -158_456_325_028_528_675_187_087_900_670_999_999_999,
        ),
    ] {
        let [t1, t2, t3, t4] = coordinates;
        let arithmetic =
            exchange_arithmetic(t1, t2, t3, t4).map_err(|_| MeasurementRefusal::Arithmetic)?;
        let (actual_phase, actual_round_trip) = nominal_arithmetic(arithmetic, remainders)?;
        assert_eq!(actual_phase.numerator(), phase);
        assert_eq!(actual_phase.denominator(), 8_589_934_592);
        assert_eq!(actual_round_trip.numerator(), round_trip);
        assert_eq!(actual_round_trip.denominator(), 4_294_967_296);
    }
    Ok(())
}

/// Portable 100/200 ns inputs retain local residuals beyond their Q32 floors.
///
/// # Errors
/// Propagates fixture/profile construction failures or an unexpected refusal
/// of the nanosecond-input measurement.
///
/// # Panics
/// Panics if either remainder or exact integer reconstruction disagrees with
/// the independently specified nanosecond values.
#[test]
fn fractional_conversion_residuals_reconstruct_nominal_values() -> Result<(), MeasurementRefusal> {
    let start = Instant::now();
    let before = capture(start, 0, BASE, 100)?;
    let read = capture(start, 100, BASE, 200)?;
    let result = measure(
        Some(ExperimentalProfile::new(BASE, BASE, 0)?),
        before,
        before,
        read,
        read.mono_after,
        header(BASE_BITS, BASE_BITS, 9)?,
    )?;
    let [r1, r4] = result.local_fractional_remainders();
    assert_eq!([r1, r4], [496_729_600, 993_459_200]);
    assert_eq!(result.arithmetic().offset().numerator(), -1_287);
    assert_eq!(result.arithmetic().round_trip().numerator(), 429);
    assert_eq!(
        result.arithmetic().offset().numerator() * NANOS_PER_SECOND
            - i128::from(r1)
            - i128::from(r4),
        -300 * Q32_SCALE
    );
    assert_eq!(
        result.arithmetic().round_trip().numerator() * NANOS_PER_SECOND + i128::from(r4)
            - i128::from(r1),
        100 * Q32_SCALE
    );
    Ok(())
}

/// Pre/post-Unix 100 ns wall neighbors retain their literal binary residuals.
///
/// # Errors
/// Propagates unavailable checked pre/post-epoch `SystemTime` construction
/// or conversion of those otherwise healthy 100-nanosecond neighbors.
///
/// # Panics
/// Panics if either neighbor loses its literal Q32 floor or remainder.
#[test]
fn pre_unix_and_post_unix_100_ns_have_literal_residuals() -> Result<(), MeasurementRefusal> {
    let before = UNIX_EPOCH
        .checked_sub(Duration::from_nanos(100))
        .ok_or(MeasurementRefusal::Arithmetic)?;
    let after = UNIX_EPOCH
        .checked_add(Duration::from_nanos(100))
        .ok_or(MeasurementRefusal::Arithmetic)?;
    let old = convert_wall(before)?;
    let new = convert_wall(after)?;
    let epoch_ticks = NTP_UNIX_OFFSET_SECONDS * Q32_SCALE;
    assert_eq!(old.coordinate.ticks(), epoch_ticks - 430);
    assert_eq!(old.remainder, 503_270_400);
    assert_eq!(new.coordinate.ticks(), epoch_ticks + 429);
    assert_eq!(new.remainder, 496_729_600);
    Ok(())
}

/// Missing explicit assumptions preserve absence rather than supplying zero.
///
/// # Errors
/// Propagates failure to construct the supplied capture or nonzero peer header.
///
/// # Panics
/// Panics if absent context produces anything other than `Unavailable`.
#[test]
fn absent_profile_is_a_distinct_transport_only_reason() -> Result<(), MeasurementRefusal> {
    let now = Instant::now();
    let clock = capture(now, 0, BASE, 0)?;
    assert_eq!(
        measure(
            None,
            clock,
            clock,
            clock,
            now,
            header(BASE_BITS, BASE_BITS, 1)?
        ),
        Err(MeasurementRefusal::Unavailable)
    );
    Ok(())
}

/// Exact inclusive allowance succeeds while its immediate lower neighbor fails.
///
/// # Errors
/// Propagates fixture/profile construction failures or an unexpected refusal
/// at the exact inclusive coherence allowance.
///
/// # Panics
/// Panics if the accepted residual/allowance differs or the lower allowance
/// fails to produce `MappingDiscontinuity`.
#[test]
fn declared_coherence_limit_has_an_exact_healthy_neighbor() -> Result<(), MeasurementRefusal> {
    let start = Instant::now();
    let before = capture(start, 0, BASE, 0)?;
    let read = capture(start, 10_000_000, BASE, 20_000_000)?;
    let raw = header(BASE_BITS, BASE_BITS, 2)?;
    let accepted = measure(
        Some(ExperimentalProfile::new(BASE, BASE, 10_000_000)?),
        before,
        before,
        read,
        read.mono_after,
        raw,
    )?;
    assert_eq!(accepted.observed_residual_ns(), 10_000_000);
    assert_eq!(accepted.declared_limit_ns(), 10_000_000);
    assert_eq!(
        measure(
            Some(ExperimentalProfile::new(BASE, BASE, 9_999_999)?),
            before,
            before,
            read,
            read.mono_after,
            raw
        ),
        Err(MeasurementRefusal::MappingDiscontinuity)
    );
    Ok(())
}

/// Portable capture brackets admit elapsed wall progress in [600, 1600] ns.
///
/// # Errors
/// Propagates fixture/profile construction failures or refusal of a specified
/// inside, exact-boundary or 100-nanosecond-allowance healthy measurement.
///
/// # Panics
/// Panics if endpoints or 100-nanosecond-grid neighbors fail their literal
/// residuals, including the one-nanosecond allowance boundary at 99/100 ns.
#[test]
fn nonzero_capture_intervals_keep_inside_and_outside_neighbors() -> Result<(), MeasurementRefusal> {
    let start = Instant::now();
    let mut before = capture(start, 1_000, BASE, 0)?;
    before.mono_after = start
        .checked_add(Duration::from_nanos(1_400))
        .ok_or(MeasurementRefusal::Arithmetic)?;
    let mut read = capture(start, 3_000, BASE, 2_000)?;
    read.mono_after = start
        .checked_add(Duration::from_nanos(3_600))
        .ok_or(MeasurementRefusal::Arithmetic)?;
    let raw = header(BASE_BITS, BASE_BITS, 11)?;
    let strict = Some(ExperimentalProfile::new(BASE, BASE, 0)?);
    let just_below = Some(ExperimentalProfile::new(BASE, BASE, 99)?);
    let neighbor = Some(ExperimentalProfile::new(BASE, BASE, 100)?);
    for wall_delta in [600, 1_200, 1_600] {
        let mut after = capture(start, 2_000, BASE, wall_delta)?;
        after.mono_after = start
            .checked_add(Duration::from_nanos(2_600))
            .ok_or(MeasurementRefusal::Arithmetic)?;
        let accepted = measure(strict, before, after, read, read.mono_after, raw)?;
        assert_eq!(accepted.observed_residual_ns(), 0);
    }
    for wall_delta in [500, 1_700] {
        let mut after = capture(start, 2_000, BASE, wall_delta)?;
        after.mono_after = start
            .checked_add(Duration::from_nanos(2_600))
            .ok_or(MeasurementRefusal::Arithmetic)?;
        assert_eq!(
            measure(strict, before, after, read, read.mono_after, raw),
            Err(MeasurementRefusal::MappingDiscontinuity)
        );
        assert_eq!(
            measure(just_below, before, after, read, read.mono_after, raw),
            Err(MeasurementRefusal::MappingDiscontinuity)
        );
        let accepted = measure(neighbor, before, after, read, read.mono_after, raw)?;
        assert_eq!(accepted.observed_residual_ns(), 100);
        assert_eq!(accepted.declared_limit_ns(), 100);
    }
    Ok(())
}

/// The residual helper rejects either reversed bracket before interval arithmetic.
/// A direct call must not depend on the separate measurement caller's admission.
///
/// # Errors
/// Propagates checked capture construction or exact wall conversion failures.
///
/// # Panics
/// Panics if either reversed bracket is admitted or the ordered zero-gap
/// healthy control changes its independently specified residual.
#[test]
fn coherence_helper_rejects_each_reversed_bracket() -> Result<(), MeasurementRefusal> {
    let start = Instant::now();
    let before = capture(start, 0, BASE, 0)?;
    let later = capture(start, 2_000, BASE, 2_000)?;
    let before_wall = convert_wall(before.utc)?;
    let later_wall = convert_wall(later.utc)?;
    assert_eq!(
        coherence_residual(before, later, before_wall, later_wall)?,
        0
    );

    let mut reversed_base = capture(start, 1_000, BASE, 0)?;
    reversed_base.mono_after = before.mono;
    let mut reversed_later = later;
    reversed_later.mono_after = reversed_base.mono;
    let results = [(reversed_base, later), (before, reversed_later)]
        .map(|(first, second)| coherence_residual(first, second, before_wall, later_wall));
    assert_eq!(
        results,
        [Err(MeasurementRefusal::CaptureOrder); 2],
        "coherence helper admitted a reversed bracket"
    );
    assert_eq!(
        coherence_residual(before, later, before_wall, later_wall)?,
        0
    );
    Ok(())
}

/// Bad bracket/send/processing order rejects beside current ordered captures.
///
/// # Errors
/// Propagates failure to construct the capture, profile or header neighbors.
///
/// # Panics
/// Panics if reversed capture/processing order is accepted or the ordered
/// healthy neighbor is refused.
#[test]
fn reversed_brackets_and_processing_are_not_ages() -> Result<(), MeasurementRefusal> {
    let start = Instant::now();
    let first = capture(start, 0, BASE, 0)?;
    let later = capture(start, 100, BASE, 100)?;
    let profile = Some(ExperimentalProfile::new(BASE, BASE, 0)?);
    let raw = header(BASE_BITS, BASE_BITS, 3)?;
    assert_eq!(
        measure(profile, first, later, first, later.mono_after, raw),
        Err(MeasurementRefusal::CaptureOrder)
    );
    assert_eq!(
        measure(profile, first, first, later, start, raw),
        Err(MeasurementRefusal::CaptureOrder)
    );
    assert!(measure(profile, first, first, later, later.mono_after, raw).is_ok());
    Ok(())
}

/// Host values cannot enlarge the independent date scope or select an era.
///
/// # Errors
/// Propagates failure to construct the independent scope, checked Q32 neighbors
/// or supplied fixture.
///
/// # Panics
/// Panics if the outside local value is rebased, inclusive Q32 endpoints reject,
/// or any coordinate's one-tick outside neighbor yields the wrong refusal.
#[test]
fn out_of_scope_local_coordinate_is_not_rebased() -> Result<(), MeasurementRefusal> {
    let dates = ExperimentalProfile::new(BASE, BASE, 0)?.dates();
    let outside_ticks = [
        dates
            .earliest()
            .ticks()
            .checked_sub(1)
            .ok_or(MeasurementRefusal::Arithmetic)?,
        dates
            .latest()
            .ticks()
            .checked_add(1)
            .ok_or(MeasurementRefusal::Arithmetic)?,
    ];
    for position in 0..3 {
        for endpoint in [dates.earliest(), dates.latest()] {
            let mut coordinates = [dates.earliest(); 3];
            coordinates[position] = endpoint;
            assert_eq!(require_in_scope(dates, &coordinates), Ok(()));
        }
        for ticks in outside_ticks {
            let mut coordinates = [dates.earliest(); 3];
            coordinates[position] =
                UtcCoordinateQ32::new(ticks).map_err(|_| MeasurementRefusal::Arithmetic)?;
            assert_eq!(
                require_in_scope(dates, &coordinates),
                Err(MeasurementRefusal::DateScope)
            );
        }
    }
    let start = Instant::now();
    let clock = capture(start, 0, BASE - 1, 0)?;
    assert_eq!(
        measure(
            Some(ExperimentalProfile::new(BASE, BASE, 0)?),
            clock,
            clock,
            clock,
            start,
            header(BASE_BITS, BASE_BITS, 4)?
        ),
        Err(MeasurementRefusal::DateScope)
    );
    Ok(())
}

/// Send-after remains part of date admission even when nominal endpoints fit.
///
/// # Errors
/// Propagates capture/header construction or explicit profile setup failures.
///
/// # Panics
/// Panics if the send-after excursion disappears from date-scope admission.
#[test]
fn send_after_outside_scope_cannot_hide_between_valid_endpoints() -> Result<(), MeasurementRefusal>
{
    let start = Instant::now();
    let before = capture(start, 0, BASE, 0)?;
    let after = capture(start, 100, BASE + 2, 0)?;
    let read = capture(start, 200, BASE, 200)?;
    assert_eq!(
        measure(
            Some(ExperimentalProfile::new(BASE, BASE, 3_000_000_000)?),
            before,
            after,
            read,
            read.mono_after,
            header(BASE_BITS, BASE_BITS, 5)?
        ),
        Err(MeasurementRefusal::DateScope)
    );
    Ok(())
}

/// Independent windows retain both absent and ambiguous peer-era outcomes.
///
/// # Errors
/// Propagates construction failures for the supplied captures, peer headers
/// or independent narrow/wide date profiles.
///
/// # Panics
/// Panics if zero and multiple candidate outcomes merge or a date fallback
/// replaces either required era refusal.
#[test]
fn era_zero_and_multiple_candidates_remain_distinct() -> Result<(), MeasurementRefusal> {
    let start = Instant::now();
    let clock = capture(start, 0, BASE, 0)?;
    assert_eq!(
        measure(
            Some(ExperimentalProfile::new(BASE, BASE, 0)?),
            clock,
            clock,
            clock,
            start,
            header((BASE + 2) << 32, (BASE + 2) << 32, 6)?
        ),
        Err(MeasurementRefusal::EraNoCandidate)
    );
    assert_eq!(
        measure(
            Some(ExperimentalProfile::new(0, 1_u64 << 32, 0)?),
            clock,
            clock,
            clock,
            start,
            header(1, 1, 6)?
        ),
        Err(MeasurementRefusal::EraAmbiguous)
    );
    Ok(())
}

/// Direct numerical callers cannot admit controls, bad modes or zero stamps.
///
/// # Errors
/// Propagates fixture decoding or capture/profile setup failures.
///
/// # Panics
/// Panics if a control, bad version/mode/LI or zero timestamp is measured,
/// or if the ordinary healthy neighbor is refused.
#[test]
fn inert_header_does_not_bypass_the_time_reply_guard() -> Result<(), MeasurementRefusal> {
    let start = Instant::now();
    let clock = capture(start, 0, BASE, 0)?;
    let profile = Some(ExperimentalProfile::new(BASE, BASE, 0)?);
    for (flags, stratum, receive, transmit) in [
        (0x24, 0, BASE_BITS, BASE_BITS),
        (0x24, 16, BASE_BITS, BASE_BITS),
        (0xe4, 1, BASE_BITS, BASE_BITS),
        (0x1c, 1, BASE_BITS, BASE_BITS),
        (0x23, 1, BASE_BITS, BASE_BITS),
        (0x24, 1, 0, BASE_BITS),
        (0x24, 1, BASE_BITS, 0),
    ] {
        let raw = header(receive, transmit, 7)?;
        let mut bytes = [0; 48];
        bytes[0] = flags;
        bytes[1] = stratum;
        bytes[32..40].copy_from_slice(&raw.receive_timestamp().bits().to_be_bytes());
        bytes[40..48].copy_from_slice(&raw.transmit_timestamp().bits().to_be_bytes());
        let invalid = decode_fixed_header(&bytes).map_err(|_| MeasurementRefusal::Protocol)?;
        assert_eq!(
            measure(profile, clock, clock, clock, start, invalid),
            Err(MeasurementRefusal::Protocol)
        );
    }
    assert!(
        measure(
            profile,
            clock,
            clock,
            clock,
            start,
            header(BASE_BITS, BASE_BITS, 7)?
        )
        .is_ok()
    );
    Ok(())
}

/// Whole-second endpoints retain all fractions and exact configured bounds.
///
/// # Errors
/// Propagates an unexpected refusal of the legal maximum-allowance profile.
///
/// # Panics
/// Panics if original bounds/full fractions change or an invalid bound or
/// excessive-allowance neighbor is admitted.
#[test]
fn profile_bounds_and_maximum_allowance_are_checked() -> Result<(), MeasurementRefusal> {
    let profile = ExperimentalProfile::new(BASE, BASE, MAX_DECLARED_RESIDUAL_NS)?;
    assert_eq!(profile.earliest_ntp_seconds(), BASE);
    assert_eq!(profile.latest_ntp_seconds(), BASE);
    assert_eq!(profile.maximum_residual_ns(), MAX_DECLARED_RESIDUAL_NS);
    assert_eq!(profile.dates().earliest().ticks(), i128::from(BASE_BITS));
    assert_eq!(
        profile.dates().latest().ticks(),
        i128::from(BASE_BITS) + Q32_SCALE - 1
    );
    assert_eq!(
        ExperimentalProfile::new(2, 1, 0),
        Err(MeasurementRefusal::InvalidProfile)
    );
    assert_eq!(
        ExperimentalProfile::new(0, 1, MAX_DECLARED_RESIDUAL_NS + 1),
        Err(MeasurementRefusal::InvalidProfile)
    );
    assert_eq!(
        ExperimentalProfile::new(i64::MAX.unsigned_abs() + 1, i64::MAX.unsigned_abs() + 1, 0),
        Err(MeasurementRefusal::InvalidProfile)
    );
    Ok(())
}

/// Final age replacement preserves the checked coordinates and model evidence.
///
/// # Errors
/// Propagates fixture/profile construction or unexpected refusal of the
/// initial healthy measurement.
///
/// # Panics
/// Panics if the fresh age differs or stamping alters the checked coordinates,
/// arithmetic or nominal software window.
#[test]
fn fresh_owner_age_stamp_preserves_the_actual_arithmetic() -> Result<(), MeasurementRefusal> {
    let start = Instant::now();
    let clock = capture(start, 0, BASE, 100)?;
    let staged = measure(
        Some(ExperimentalProfile::new(BASE, BASE, 0)?),
        clock,
        clock,
        clock,
        start,
        header(BASE_BITS + 429, BASE_BITS + 429, 1)?,
    )?;
    let committed = staged.with_acquisition_age(Duration::from_micros(25));
    assert_eq!(committed.acquisition_age(), Duration::from_micros(25));
    assert_eq!(committed.coordinates(), staged.coordinates());
    assert_eq!(committed.arithmetic(), staged.arithmetic());
    assert_eq!(staged.nominal_phase().numerator(), -993_459_200);
    assert_eq!(committed.nominal_phase(), staged.nominal_phase());
    assert_eq!(committed.nominal_round_trip(), staged.nominal_round_trip());
    assert_eq!(
        committed.local_fractional_remainders(),
        staged.local_fractional_remainders()
    );
    assert_eq!(committed.software_window(), staged.software_window());
    Ok(())
}
