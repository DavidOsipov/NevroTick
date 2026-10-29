//! Same-domain clocks retain units, ordering and checked transformation bounds.
#![forbid(unsafe_code)]

use time_domain::{ActiveAwakeNs, DomainError, MonotonicNs};

/// Unit wrappers preserve every representable supplied reading without authority.
/// # Panics
/// Fails if unit tagging changes raw readings or their same-domain ordering.
#[test]
fn clock_unit_wrappers_preserve_u64_extrema() {
    for raw in [0, 1, u64::MAX - 1, u64::MAX] {
        assert_eq!(MonotonicNs::new(raw).nanoseconds(), raw);
        assert_eq!(ActiveAwakeNs::new(raw).nanoseconds(), raw);
    }
    assert!(MonotonicNs::new(0) < MonotonicNs::new(u64::MAX));
    assert!(ActiveAwakeNs::new(0) < ActiveAwakeNs::new(u64::MAX));
}

/// Positive and zero monotonic elapsed results preserve exact same-domain units.
/// # Errors
/// Propagates an unexpected rejection of ordered readings or legal additions.
/// # Panics
/// Fails when elapsed arithmetic or projections differ from independent facts.
#[test]
fn monotonic_elapsed_and_checked_add_healthy_controls() -> Result<(), DomainError> {
    let start = MonotonicNs::new(10);
    let end = MonotonicNs::new(13);
    let three = end.duration_since(start)?;
    let zero = start.duration_since(start)?;
    assert_eq!(three.nanoseconds(), 3);
    assert_eq!(zero.nanoseconds(), 0);
    assert_eq!(start.checked_add(three)?, end);
    assert_eq!(
        MonotonicNs::new(u64::MAX).checked_add(zero)?.nanoseconds(),
        u64::MAX
    );
    assert_eq!(
        MonotonicNs::new(u64::MAX)
            .duration_since(MonotonicNs::new(0))?
            .nanoseconds(),
        u64::MAX
    );
    Ok(())
}

/// Active-awake elapsed results remain separate from monotonic elapsed results.
/// # Errors
/// Propagates unexpected rejection of legal same-domain controls.
/// # Panics
/// Fails when exact elapsed/addition or maximal domain values change.
#[test]
fn active_awake_elapsed_and_checked_add_healthy_controls() -> Result<(), DomainError> {
    let start = ActiveAwakeNs::new(10);
    let end = ActiveAwakeNs::new(13);
    let three = end.duration_since(start)?;
    let zero = start.duration_since(start)?;
    assert_eq!(three.nanoseconds(), 3);
    assert_eq!(zero.nanoseconds(), 0);
    assert_eq!(start.checked_add(three)?, end);
    assert_eq!(
        ActiveAwakeNs::new(u64::MAX)
            .checked_add(zero)?
            .nanoseconds(),
        u64::MAX
    );
    assert_eq!(
        ActiveAwakeNs::new(u64::MAX)
            .duration_since(ActiveAwakeNs::new(0))?
            .nanoseconds(),
        u64::MAX
    );
    Ok(())
}

/// Reverse order is a real rejection, paired with exact healthy elapsed tests.
/// # Panics
/// Fails when reverse readings wrap, clamp or acquire a successful duration.
#[test]
fn reversed_readings_reject_in_both_domains() {
    assert_eq!(
        MonotonicNs::new(10).duration_since(MonotonicNs::new(11)),
        Err(DomainError::DurationOutOfOrder)
    );
    assert_eq!(
        ActiveAwakeNs::new(10).duration_since(ActiveAwakeNs::new(11)),
        Err(DomainError::DurationOutOfOrder)
    );
}

/// Checked addition rejects just-above-u64 output while retaining valid neighbors.
/// # Errors
/// Propagates unexpected rejection of legal elapsed construction or addition.
/// # Panics
/// Fails when overflow saturates/wraps, or legal endpoint addition changes.
#[test]
fn elapsed_addition_overflow_and_neighbor() -> Result<(), DomainError> {
    let monotonic_one = MonotonicNs::new(1).duration_since(MonotonicNs::new(0))?;
    let awake_one = ActiveAwakeNs::new(1).duration_since(ActiveAwakeNs::new(0))?;
    assert_eq!(
        MonotonicNs::new(u64::MAX).checked_add(monotonic_one),
        Err(DomainError::ArithmeticOverflow)
    );
    assert_eq!(
        ActiveAwakeNs::new(u64::MAX).checked_add(awake_one),
        Err(DomainError::ArithmeticOverflow)
    );
    assert_eq!(
        MonotonicNs::new(u64::MAX - 1)
            .checked_add(monotonic_one)?
            .nanoseconds(),
        u64::MAX
    );
    assert_eq!(
        ActiveAwakeNs::new(u64::MAX - 1)
            .checked_add(awake_one)?
            .nanoseconds(),
        u64::MAX
    );
    Ok(())
}
