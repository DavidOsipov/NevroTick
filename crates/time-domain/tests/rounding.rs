//! Independent exact rounding literals and rational reconstruction observations.
#![forbid(unsafe_code)]

use time_domain::{DomainError, ExchangeArithmetic, UtcCoordinateQ32, exchange_arithmetic};

/// Produce both relative numerators n through actual coordinate arithmetic.
/// T1=T3=T4=0 and T2=n gives phase numerator n and raw RTT numerator n.
/// # Errors
/// Propagates unexpected legal coordinate or arithmetic rejection.
fn relative(numerator: i128) -> Result<ExchangeArithmetic, DomainError> {
    let zero = UtcCoordinateQ32::new(0)?;
    exchange_arithmetic(zero, UtcCoordinateQ32::new(numerator)?, zero, zero)
}

/// Q33 literals discriminate rounded zero, both tie signs/even directions and
/// one-input-tick neighbors. Expectations were independently rationally derived.
/// # Errors
/// Propagates unexpected rejection by any real construction/rounding entry.
/// # Panics
/// Fails on wrong nearest/tie rule, sign, denominator or residual reconstruction.
#[test]
fn phase_ties_near_ties_zero_and_signed_residuals() -> Result<(), DomainError> {
    for (numerator, nanoseconds, residual) in [
        (0_i128, 0_i64, 0_i128),
        (1, 0, 1_000_000_000),
        (-1, 0, -1_000_000_000),
        (8_388_607, 976_562, 3_294_967_296),
        (8_388_608, 976_562, 4_294_967_296),
        (8_388_609, 976_563, -3_294_967_296),
        (25_165_824, 2_929_688, -4_294_967_296),
        (-8_388_607, -976_562, -3_294_967_296),
        (-8_388_608, -976_562, -4_294_967_296),
        (-8_388_609, -976_563, 3_294_967_296),
        (-25_165_824, -2_929_688, 4_294_967_296),
    ] {
        let result = relative(numerator)?
            .offset()
            .round_nanoseconds_ties_even()?;
        assert_eq!(result.nanoseconds(), nanoseconds);
        assert_eq!(result.residual_numerator(), residual);
        assert_eq!(result.residual_denominator(), 8_589_934_592);
        assert_eq!(
            i128::from(result.nanoseconds()) * 8_589_934_592 + result.residual_numerator(),
            numerator * 1_000_000_000
        );
    }
    Ok(())
}

/// Q32 has its own residual denominator and distinct exact half cases.
/// # Errors
/// Propagates unexpected actual constructor/arithmetic/rounding rejection.
/// # Panics
/// Fails on denominator confusion, wrong tie parity or residual sign.
#[test]
fn round_trip_ties_near_ties_zero_and_signed_residuals() -> Result<(), DomainError> {
    for (numerator, nanoseconds, residual) in [
        (0_i128, 0_i64, 0_i128),
        (1, 0, 1_000_000_000),
        (-1, 0, -1_000_000_000),
        (4_194_303, 976_562, 1_147_483_648),
        (4_194_304, 976_562, 2_147_483_648),
        (4_194_305, 976_563, -1_147_483_648),
        (12_582_912, 2_929_688, -2_147_483_648),
        (-4_194_303, -976_562, -1_147_483_648),
        (-4_194_304, -976_562, -2_147_483_648),
        (-4_194_305, -976_563, 1_147_483_648),
        (-12_582_912, -2_929_688, 2_147_483_648),
    ] {
        let result = relative(numerator)?
            .round_trip()
            .round_nanoseconds_ties_even()?;
        assert_eq!(result.nanoseconds(), nanoseconds);
        assert_eq!(result.residual_numerator(), residual);
        assert_eq!(result.residual_denominator(), 4_294_967_296);
        assert_eq!(
            i128::from(result.nanoseconds()) * 4_294_967_296 + result.residual_numerator(),
            numerator * 1_000_000_000
        );
    }
    Ok(())
}

/// Phase destination endpoint literals are inside the rounding cell; adjacent
/// outside-cell inputs must fail rather than clamp or cast modulo i64.
/// # Errors
/// Propagates unexpected rejection of legal coordinate/arithmetic/endpoints.
/// # Panics
/// Fails when signed-i64 phase destination enforcement or residual changes.
#[test]
fn phase_i64_destination_endpoints_and_overflow() -> Result<(), DomainError> {
    for (numerator, expected, residual) in [
        (79_228_162_514_264_337_585_i128, i64::MAX, 45_984_256_i128),
        (79_228_162_514_264_337_589, i64::MAX, 4_045_984_256),
        (-79_228_162_514_264_337_594, i64::MIN, -456_049_664),
        (-79_228_162_514_264_337_597, i64::MIN, -3_456_049_664),
    ] {
        let result = relative(numerator)?
            .offset()
            .round_nanoseconds_ties_even()?;
        assert_eq!(result.nanoseconds(), expected);
        assert_eq!(result.residual_numerator(), residual);
    }
    for numerator in [79_228_162_514_264_337_590_i128, -79_228_162_514_264_337_598] {
        assert_eq!(
            relative(numerator)?.offset().round_nanoseconds_ties_even(),
            Err(DomainError::NanosecondsOutOfRange)
        );
    }
    Ok(())
}

/// RTT endpoint literals include exact last accepted and first rejected Q32 ticks.
/// # Errors
/// Propagates unexpected legal coordinate/arithmetic/rounding rejection.
/// # Panics
/// Fails on signed-i64 output narrowing, saturation or asymmetric negative bounds.
#[test]
fn round_trip_i64_destination_endpoints_and_overflow() -> Result<(), DomainError> {
    for (numerator, expected, residual) in [
        (
            39_614_081_257_132_168_794_i128,
            i64::MAX,
            1_522_992_128_i128,
        ),
        (-39_614_081_257_132_168_798, i64::MIN, -1_228_024_832),
    ] {
        let result = relative(numerator)?
            .round_trip()
            .round_nanoseconds_ties_even()?;
        assert_eq!(result.nanoseconds(), expected);
        assert_eq!(result.residual_numerator(), residual);
    }
    for numerator in [39_614_081_257_132_168_795_i128, -39_614_081_257_132_168_799] {
        assert_eq!(
            relative(numerator)?
                .round_trip()
                .round_nanoseconds_ties_even(),
            Err(DomainError::NanosecondsOutOfRange)
        );
    }
    Ok(())
}

/// Legal extreme relative results must reject conversion whose rounded value
/// exceeds i64; the operation itself remains exact within the i128 envelope.
/// # Errors
/// Propagates unexpected rejection of legal coordinates or exact arithmetic.
/// # Panics
/// Fails when a huge valid relative result is silently clamped/rounded to success.
#[test]
fn extreme_relative_results_reject_nanosecond_destination() -> Result<(), DomainError> {
    let low = UtcCoordinateQ32::new(-(1_i128 << 95))?;
    let high = UtcCoordinateQ32::new((1_i128 << 95) - 1)?;
    for result in [
        exchange_arithmetic(low, high, high, low)?,
        exchange_arithmetic(high, low, low, high)?,
    ] {
        assert_eq!(
            result.offset().round_nanoseconds_ties_even(),
            Err(DomainError::NanosecondsOutOfRange)
        );
    }
    for result in [
        exchange_arithmetic(low, high, low, high)?,
        exchange_arithmetic(high, low, high, low)?,
    ] {
        assert_eq!(
            result.round_trip().round_nanoseconds_ties_even(),
            Err(DomainError::NanosecondsOutOfRange)
        );
    }
    Ok(())
}

/// Finite small numerators observe exact residual reconstruction, nearest bounds
/// and even destinations at ties. These necessary postconditions discriminate
/// wrong rounding without using a production rounding helper as the oracle.
/// # Errors
/// Propagates unexpected rejection along real input/arithmetic/rounding paths.
/// # Panics
/// Fails when reconstruction, nearest bound, parity or signed symmetry is broken.
#[test]
fn bounded_generated_rounding_reconstruction_and_nearest_bounds() -> Result<(), DomainError> {
    for numerator in -4_096_i128..=4_096 {
        let relative = relative(numerator)?;
        let phase = relative.offset().round_nanoseconds_ties_even()?;
        let delay = relative.round_trip().round_nanoseconds_ties_even()?;
        for (rounded, residual, denominator) in [
            (
                phase.nanoseconds(),
                phase.residual_numerator(),
                8_589_934_592_i128,
            ),
            (
                delay.nanoseconds(),
                delay.residual_numerator(),
                4_294_967_296_i128,
            ),
        ] {
            assert_eq!(
                i128::from(rounded) * denominator + residual,
                numerator * 1_000_000_000
            );
            assert!(residual.abs() <= denominator / 2);
            if residual.abs() == denominator / 2 {
                assert_eq!(rounded % 2, 0);
            }
        }
    }
    Ok(())
}
