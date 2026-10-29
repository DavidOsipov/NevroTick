//! Public scalar admission boundaries, independent of implementation helpers.
#![forbid(unsafe_code)]

use time_domain::{DomainError, MAX_COORDINATE_TICKS, MIN_COORDINATE_TICKS, UtcCoordinateQ32};

/// Exact numeric endpoints and legal neighboring values must remain unchanged.
/// # Errors
/// Propagates any unexpected rejection of a legal numeric coordinate.
/// # Panics
/// Fails when the independent engineering envelope or projection changes.
#[test]
fn coordinate_exact_endpoints_and_healthy_neighbors() -> Result<(), DomainError> {
    let minimum = -(1_i128 << 95);
    let maximum = (1_i128 << 95) - 1;
    assert_eq!(MIN_COORDINATE_TICKS, minimum);
    assert_eq!(MAX_COORDINATE_TICKS, maximum);
    for ticks in [minimum, minimum + 1, -1, 0, 1, maximum - 1, maximum] {
        assert_eq!(UtcCoordinateQ32::new(ticks)?.ticks(), ticks);
    }
    Ok(())
}

/// Both immediate invalid neighbors and primitive extrema reject explicitly.
/// # Panics
/// Fails if constructor wraps, clamps or admits an out-of-envelope value.
#[test]
fn coordinate_invalid_neighbors_and_i128_extrema_reject() {
    for ticks in [-(1_i128 << 95) - 1, 1_i128 << 95, i128::MIN, i128::MAX] {
        assert_eq!(
            UtcCoordinateQ32::new(ticks),
            Err(DomainError::CoordinateOutOfRange)
        );
    }
}

/// Every public error remains a distinguishable bounded redacted diagnostic.
/// No error carries rejected values; its display must contain only ASCII words.
/// # Panics
/// Fails for empty, oversized, duplicate or non-allowlisted diagnostic text.
#[test]
fn every_domain_error_has_bounded_redacted_display() {
    let errors = [
        DomainError::CoordinateOutOfRange,
        DomainError::InvalidEraWindow,
        DomainError::NoEraCandidate,
        DomainError::AmbiguousEra,
        DomainError::ArithmeticOverflow,
        DomainError::DurationOutOfOrder,
        DomainError::NanosecondsOutOfRange,
        DomainError::ResidualOutOfRange,
    ];
    let messages = errors.map(|error| error.to_string());
    for (index, message) in messages.iter().enumerate() {
        assert!(!message.is_empty() && message.len() <= 32);
        assert!(
            message
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte == b' ')
        );
        assert!(!messages[..index].contains(message));
    }
}

/// Whole signed-i64 seconds and maximal fractions retain exact Q32 values.
/// # Errors
/// Propagates unexpected rejection of an exactly representable coordinate.
/// # Panics
/// Fails if whole-second and fractional endpoints are conflated.
#[test]
fn signed_whole_second_envelope_is_exact() -> Result<(), DomainError> {
    let minimum_seconds = i128::from(i64::MIN) * 4_294_967_296;
    let maximum_seconds = i128::from(i64::MAX) * 4_294_967_296;
    assert_eq!(
        UtcCoordinateQ32::new(minimum_seconds)?.ticks(),
        -(1_i128 << 95)
    );
    assert_eq!(
        UtcCoordinateQ32::new(maximum_seconds + i128::from(u32::MAX))?.ticks(),
        (1_i128 << 95) - 1
    );
    Ok(())
}

/// Optimized runtime must reject invalid construction and a transformed overflow,
/// beside legal neighboring construction/transform paths. This is executed only
/// in the actual release lane and is required by the reviewed runner inventory.
/// # Errors
/// Propagates unexpected rejection of legal constructor or elapsed controls.
/// # Panics
/// Fails if release-only admission or checked addition loses its enforcement.
#[cfg(not(debug_assertions))]
#[test]
fn release_build_preserves_checked_admission() -> Result<(), DomainError> {
    use time_domain::MonotonicNs;

    assert_eq!(
        UtcCoordinateQ32::new(1_i128 << 95),
        Err(DomainError::CoordinateOutOfRange)
    );
    assert_eq!(
        UtcCoordinateQ32::new((1_i128 << 95) - 1)?.ticks(),
        (1_i128 << 95) - 1
    );
    let one = MonotonicNs::new(1).duration_since(MonotonicNs::new(0))?;
    assert_eq!(
        MonotonicNs::new(u64::MAX).checked_add(one),
        Err(DomainError::ArithmeticOverflow)
    );
    assert_eq!(
        MonotonicNs::new(u64::MAX - 1)
            .checked_add(one)?
            .nanoseconds(),
        u64::MAX
    );
    Ok(())
}
