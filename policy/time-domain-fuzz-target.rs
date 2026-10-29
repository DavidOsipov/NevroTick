//! Independent S1.1c integer models, authored for the reviewed task contract.
//! No product implementation helper or upstream output supplies expected values.
//! Test-only GPL-2.0-only harness, David Osipov with AI assistance, 29 October 2026.
#![no_main]
#![forbid(unsafe_code)]

use libfuzzer_sys::fuzz_target;
use time_domain::{
    ActiveAwakeNs, DomainError, EraWindow, MonotonicNs, UtcCoordinateQ32,
    exchange_arithmetic, unfold_ntp,
};

const LOWER: i128 = -(1_i128 << 95);
const UPPER: i128 = (1_i128 << 95) - 1;
const MODULUS: i128 = 1_i128 << 64;

/// A short input is zero-padded, so every invocation reaches all numeric owners.
fn octets<const N: usize>(data: &[u8], start: usize) -> [u8; N] {
    let mut result = [0; N];
    for (index, byte) in result.iter_mut().enumerate() {
        *byte = data.get(start + index).copied().unwrap_or(0);
    }
    result
}

/// Decode full hostile i128 input independently of the product constructor.
fn raw(data: &[u8], start: usize) -> i128 {
    i128::from_be_bytes(octets(data, start))
}

/// Interpret the low 96 bits as signed, reaching the full coordinate envelope.
fn coordinate(data: &[u8], start: usize) -> i128 {
    let mut value: [u8; 16] = octets(data, start);
    let sign = if value[4] & 128 == 0 { 0 } else { 255 };
    value[..4].fill(sign);
    i128::from_be_bytes(value)
}

/// Model a numeric interval by its first congruent point plus a second witness.
/// This does not reuse the product's ceil/floor era-index bounds or era iteration.
fn era_reference(bits: u64, lower: i128, upper: i128) -> Result<i128, DomainError> {
    let distance = (i128::from(bits) - lower.rem_euclid(MODULUS)).rem_euclid(MODULUS);
    let first = lower + distance;
    if first > upper {
        Err(DomainError::NoEraCandidate)
    } else if first + MODULUS <= upper {
        Err(DomainError::AmbiguousEra)
    } else {
        Ok(first)
    }
}

/// Compare the two adjacent integer candidates by exact residual distance/parity.
/// All operation-produced numerators satisfy |n| <= 2^97-2; n*10^9 fits i128.
fn rounding_reference(numerator: i128, scale: i128) -> Option<(i64, i128)> {
    let scaled = numerator * 1_000_000_000;
    let floor = scaled.div_euclid(scale);
    let ceiling = floor + 1;
    let floor_distance = scaled - floor * scale;
    let ceiling_distance = ceiling * scale - scaled;
    let nearest = if floor_distance < ceiling_distance
        || (floor_distance == ceiling_distance && floor.rem_euclid(2) == 0)
    {
        floor
    } else {
        ceiling
    };
    i64::try_from(nearest).ok().map(|value| (value, scaled - nearest * scale))
}

/// Check actual unavoidable coordinate and garde-window construction first.
fn check_construction(data: &[u8]) {
    let value = raw(data, 0);
    let actual = UtcCoordinateQ32::new(value);
    if (LOWER..=UPPER).contains(&value) {
        assert_eq!(actual.expect("independent admitted coordinate").ticks(), value);
    } else {
        assert_eq!(actual, Err(DomainError::CoordinateOutOfRange));
    }
    let lower = raw(data, 16);
    let upper = raw(data, 32);
    let actual = EraWindow::new(lower, upper);
    if (LOWER..=UPPER).contains(&lower) && (LOWER..=UPPER).contains(&upper) && lower <= upper {
        let accepted = actual.expect("independent admitted scalar window");
        assert_eq!(accepted.earliest().ticks(), lower);
        assert_eq!(accepted.latest().ticks(), upper);
    } else {
        assert_eq!(actual, Err(DomainError::InvalidEraWindow));
    }
}

/// Check positive, negative, absent and ambiguous eras within a supplied window.
fn check_era(data: &[u8]) {
    let a = coordinate(data, 120);
    let b = coordinate(data, 136);
    let lower = a.min(b);
    let upper = a.max(b);
    let bits = u64::from_be_bytes(octets(data, 112));
    let parts: [u8; 8] = bits.to_be_bytes();
    let seconds = u32::from_be_bytes(parts[..4].try_into().expect("fixed seconds extent"));
    let fraction = u32::from_be_bytes(parts[4..].try_into().expect("fixed fraction extent"));
    let window = EraWindow::new(lower, upper).expect("independent valid fixture window");
    assert_eq!(
        unfold_ntp(seconds, fraction, &window).map(UtcCoordinateQ32::ticks),
        era_reference(bits, lower, upper),
        "independent next-congruent era oracle"
    );
}

/// Compare actual arithmetic/rounding with independent linear/rational models.
fn check_exchange(data: &[u8]) {
    let values = [coordinate(data, 48), coordinate(data, 64), coordinate(data, 80), coordinate(data, 96)];
    let [a, b, c, d] = values.map(|value| UtcCoordinateQ32::new(value).expect("valid 96-bit fixture"));
    let exact = exchange_arithmetic(a, b, c, d).expect("independent bounded arithmetic");
    let phase = -values[0] + values[1] + values[2] - values[3];
    let delay = -values[0] + values[1] - values[2] + values[3];
    // This separately built fault perturbs only the observation after the real
    // arithmetic path. It must fail this exact numeric assertion, not setup.
    let observed = exact.offset().numerator() + i128::from(cfg!(feature = "seeded-defect"));
    assert_eq!(observed, phase, "independent exact phase-offset oracle");
    assert_eq!(exact.round_trip().numerator(), delay, "independent signed RTT oracle");
    let phase_result = exact.offset().round_nanoseconds_ties_even();
    match rounding_reference(phase, 1_i128 << 33) {
        Some((nanoseconds, residual)) => {
            let result = phase_result.expect("phase destination fits independently");
            assert_eq!(result.nanoseconds(), nanoseconds);
            assert_eq!(result.residual_numerator(), residual);
            assert_eq!(result.residual_denominator(), 1_i128 << 33);
            assert_eq!(i128::from(result.nanoseconds()) * result.residual_denominator()
                + result.residual_numerator(), phase * 1_000_000_000);
        }
        None => assert_eq!(phase_result, Err(DomainError::NanosecondsOutOfRange)),
    }
    let delay_result = exact.round_trip().round_nanoseconds_ties_even();
    match rounding_reference(delay, 1_i128 << 32) {
        Some((nanoseconds, residual)) => {
            let result = delay_result.expect("RTT destination fits independently");
            assert_eq!(result.nanoseconds(), nanoseconds);
            assert_eq!(result.residual_numerator(), residual);
            assert_eq!(result.residual_denominator(), 1_i128 << 32);
            assert_eq!(i128::from(result.nanoseconds()) * result.residual_denominator()
                + result.residual_numerator(), delay * 1_000_000_000);
        }
        None => assert_eq!(delay_result, Err(DomainError::NanosecondsOutOfRange)),
    }
}

/// Wider integer expectations check both distinct same-domain clock APIs.
fn check_clocks(data: &[u8]) {
    let start = u64::from_be_bytes(octets(data, 152));
    let end = u64::from_be_bytes(octets(data, 160));
    let base = u64::from_be_bytes(octets(data, 168));
    let monotonic = MonotonicNs::new(end).duration_since(MonotonicNs::new(start));
    let awake = ActiveAwakeNs::new(end).duration_since(ActiveAwakeNs::new(start));
    if end < start {
        assert_eq!(monotonic, Err(DomainError::DurationOutOfOrder));
        assert_eq!(awake, Err(DomainError::DurationOutOfOrder));
    } else {
        let expected = u128::from(end) - u128::from(start);
        let monotonic = monotonic.expect("independent ordered monotonic readings");
        let awake = awake.expect("independent ordered active-awake readings");
        assert_eq!(u128::from(monotonic.nanoseconds()), expected);
        assert_eq!(u128::from(awake.nanoseconds()), expected);
        let total = u128::from(base) + expected;
        let m = MonotonicNs::new(base).checked_add(monotonic);
        let a = ActiveAwakeNs::new(base).checked_add(awake);
        if total > u128::from(u64::MAX) {
            assert_eq!(m, Err(DomainError::ArithmeticOverflow));
            assert_eq!(a, Err(DomainError::ArithmeticOverflow));
        } else {
            assert_eq!(u128::from(m.expect("independent sum fits").nanoseconds()), total);
            assert_eq!(u128::from(a.expect("independent sum fits").nanoseconds()), total);
        }
    }
}

fuzz_target!(|data: &[u8]| {
    check_construction(data);
    check_era(data);
    check_exchange(data);
    check_clocks(data);
});
