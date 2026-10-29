//! Literal explicit-window era fixtures, including negative and hostile widths.
#![forbid(unsafe_code)]

use time_domain::{DomainError, EraWindow, unfold_ntp};

/// One full raw-timestamp period in Q32 ticks, independently specified as 2^64.
const ERA: i128 = 18_446_744_073_709_551_616;

/// Both endpoints take the actual mandatory garde and ordering construction path.
/// # Errors
/// Propagates unexpected rejection of ordered/equal healthy windows.
/// # Panics
/// Fails when a raw endpoint bypasses validation or exact endpoints change.
#[test]
fn era_window_raw_fields_ordering_and_healthy_neighbors() -> Result<(), DomainError> {
    for (lower, upper) in [(0, 0), (-1, 1), (-(1_i128 << 95), (1_i128 << 95) - 1)] {
        let window = EraWindow::new(lower, upper)?;
        assert_eq!(window.earliest().ticks(), lower);
        assert_eq!(window.latest().ticks(), upper);
    }
    for (lower, upper) in [
        (1, 0),
        (i128::MIN, 0),
        (0, i128::MAX),
        (-(1_i128 << 95) - 1, 0),
        (0, 1_i128 << 95),
    ] {
        assert_eq!(
            EraWindow::new(lower, upper),
            Err(DomainError::InvalidEraWindow)
        );
    }
    Ok(())
}

/// Exact inclusive edge matches and arbitrary raw fractions must be preserved.
/// # Errors
/// Propagates unexpected rejection of explicit unique in-window coordinates.
/// # Panics
/// Fails on an off-by-one, fraction truncation or guessed nearest-era result.
#[test]
fn exact_interval_edges_and_raw_parts_are_preserved() -> Result<(), DomainError> {
    for (seconds, fraction, expected) in [
        (0, 0, 0),
        (0, 1, 1),
        (0, u32::MAX, 4_294_967_295),
        (u32::MAX, u32::MAX, ERA - 1),
    ] {
        let exact = EraWindow::new(expected, expected)?;
        assert_eq!(unfold_ntp(seconds, fraction, &exact)?.ticks(), expected);
        let left_edge = EraWindow::new(expected, expected + 1)?;
        let right_edge = EraWindow::new(expected - 1, expected)?;
        assert_eq!(unfold_ntp(seconds, fraction, &left_edge)?.ticks(), expected);
        assert_eq!(
            unfold_ntp(seconds, fraction, &right_edge)?.ticks(),
            expected
        );
    }
    Ok(())
}

/// No candidate and two inclusive candidates are distinct fixed failures.
/// # Errors
/// Propagates unexpected failure to construct valid numeric windows.
/// # Panics
/// Fails if an unanchored/half-era-shaped window falls back to a guessed pivot.
#[test]
fn absent_ambiguous_and_half_era_windows_are_distinct() -> Result<(), DomainError> {
    assert_eq!(
        unfold_ntp(0, 0, &EraWindow::new(1, ERA - 1)?),
        Err(DomainError::NoEraCandidate)
    );
    assert_eq!(
        unfold_ntp(0, 0, &EraWindow::new(0, ERA)?),
        Err(DomainError::AmbiguousEra)
    );
    assert_eq!(
        unfold_ntp(0, 0, &EraWindow::new(-ERA / 2, ERA + ERA / 2)?),
        Err(DomainError::AmbiguousEra)
    );
    assert_eq!(unfold_ntp(0, 0, &EraWindow::new(0, ERA - 1)?)?.ticks(), 0);
    Ok(())
}

/// Negative intervals discriminate Euclidean bounds from truncation toward zero.
/// # Errors
/// Propagates unexpected rejection of unique negative-era coordinates/windows.
/// # Panics
/// Fails on negative ceil/floor errors or a nonnegative fallback era.
#[test]
fn negative_eras_use_euclidean_bounds() -> Result<(), DomainError> {
    assert_eq!(
        unfold_ntp(0, 0, &EraWindow::new(-ERA - 1, -1)?)?.ticks(),
        -ERA
    );
    assert_eq!(
        unfold_ntp(0, 1, &EraWindow::new(-ERA, -1)?)?.ticks(),
        -ERA + 1
    );
    assert_eq!(
        unfold_ntp(u32::MAX, u32::MAX, &EraWindow::new(-2, -1)?)?.ticks(),
        -1
    );
    assert_eq!(
        unfold_ntp(0, 0, &EraWindow::new(-ERA + 1, -1)?),
        Err(DomainError::NoEraCandidate)
    );
    assert_eq!(
        unfold_ntp(0, 0, &EraWindow::new(-2 * ERA, -ERA)?),
        Err(DomainError::AmbiguousEra)
    );
    Ok(())
}

/// Extreme eras and the whole engineering envelope require fixed-width work.
/// # Errors
/// Propagates unexpected rejection of huge explicit unique-era windows.
/// # Panics
/// Fails on era narrowing, endpoint overflow or wide-window false uniqueness.
#[test]
fn huge_positive_negative_eras_and_wide_window() -> Result<(), DomainError> {
    for expected in [-(1_i128 << 95), -(1_i128 << 94), 1_i128 << 94] {
        assert_eq!(
            unfold_ntp(0, 0, &EraWindow::new(expected, expected)?)?.ticks(),
            expected
        );
    }
    let maximum = (1_i128 << 95) - 1;
    assert_eq!(
        unfold_ntp(u32::MAX, u32::MAX, &EraWindow::new(maximum, maximum)?)?.ticks(),
        maximum
    );
    let widest = EraWindow::new(-(1_i128 << 95), maximum)?;
    for _ in 0..64 {
        assert_eq!(unfold_ntp(0, 0, &widest), Err(DomainError::AmbiguousEra));
    }
    Ok(())
}

/// A bounded independent finite candidate enumeration checks 2,112 small windows.
/// Production helpers and bound formulas are not reused as expected results.
/// # Errors
/// Propagates unexpected valid-window construction or unique unfolding failure.
/// # Panics
/// Fails when real resolution differs from independently enumerated candidates.
#[test]
fn bounded_generated_windows_match_independent_candidate_enumeration() -> Result<(), DomainError> {
    for raw in [0_i128, 1, 4_294_967_295, 4_294_967_296] {
        let seconds =
            u32::try_from(raw / 4_294_967_296).map_err(|_| DomainError::ArithmeticOverflow)?;
        let fraction =
            u32::try_from(raw % 4_294_967_296).map_err(|_| DomainError::ArithmeticOverflow)?;
        for lower_step in -8_i128..=7 {
            for width_step in 0_i128..=32 {
                let lower = lower_step * ERA / 4;
                let upper = lower + width_step * ERA / 4;
                let mut count = 0_u32;
                let mut unique = 0_i128;
                for era in -4_i128..=12 {
                    let candidate = raw + era * ERA;
                    if (lower..=upper).contains(&candidate) {
                        count += 1;
                        unique = candidate;
                    }
                }
                let actual = unfold_ntp(seconds, fraction, &EraWindow::new(lower, upper)?);
                match count {
                    0 => assert_eq!(actual, Err(DomainError::NoEraCandidate)),
                    1 => assert_eq!(actual?.ticks(), unique),
                    _ => assert_eq!(actual, Err(DomainError::AmbiguousEra)),
                }
            }
        }
    }
    Ok(())
}

/// Rejection and accepted-neighbor construction alternate for a finite workload.
/// # Errors
/// Propagates unexpected rejection of healthy windows after invalid construction.
/// # Panics
/// Fails if errors leak rejected values or alter subsequent legal construction.
#[test]
fn repeated_invalid_valid_constructor_cycles_are_redacted() -> Result<(), DomainError> {
    for _ in 0..256 {
        assert_eq!(
            EraWindow::new(i128::MIN, i128::MAX),
            Err(DomainError::InvalidEraWindow)
        );
        assert_eq!(
            DomainError::InvalidEraWindow.to_string(),
            "invalid era window"
        );
        assert_eq!(EraWindow::new(-1, 1)?.earliest().ticks(), -1);
    }
    assert!(!std::mem::needs_drop::<EraWindow>());
    Ok(())
}
