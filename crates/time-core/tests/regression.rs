// SPDX-License-Identifier: GPL-2.0-only
//! Independent external S1.2a consumers of the reviewed public API.
//!
//! Truth: activation-facts.json SHA256
//! 9eaa3271413e21e22597326856e23ae7fabfaefedf2bea309590769bf53d9d38,
//! original independent-oracles.json and constant-witness.json SHA256
//! c2d608c6b3c4cca99fbf8d7bf84a6ab29cafdac486204e4b28f186b56cf393b5.
//! No implementation body, product arithmetic helper or source-string assertion
//! supplies an expectation. All arrays/loops are bounded; there are no retries,
//! network/native effects, ambient clocks, global counters or new dependencies.
//! One complete profile has 18 test scopes, at most 285 fit attempts and exactly
//! 1,329 point attempts when all paths complete. The 256 analytic cases account
//! for 256 fits/1,024 points; two underflow cases are deliberately observed twice
//! each (four attempts), without a retry or changed input. Early RED emits only
//! its actual executed prefix through each scope's direct-stderr Drop record.
#![forbid(unsafe_code)]

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use time_core::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FitInput, FitPoint,
    FixtureLine, PointField, PointIssue, fit,
};

/// Harness/setup errors propagate separately from product assertion failures.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Frozen absolute intercept tolerance, 2^-36 seconds.
const PHASE_TOLERANCE: f64 = 1.0 / 68_719_476_736.0;
/// Frozen absolute rate tolerance, 2^-32 seconds/second.
const RATE_TOLERANCE: f64 = 1.0 / 4_294_967_296.0;
/// Frozen relative tolerance, 2^-40, applied to each exact expected coefficient.
const RELATIVE_TOLERANCE: f64 = 1.0 / 1_099_511_627_776.0;

/// Per-test public-entry attempts, including rejected calls; never shared globally.
struct Profile {
    /// Number of actual fit attempts made by this test scope.
    fits: Cell<u64>,
    /// Number of actual point-constructor attempts made by this test scope.
    points: Cell<u64>,
}

impl Profile {
    /// Begin a fresh counter scope without constructing a product value.
    const fn new() -> Self {
        Self {
            fits: Cell::new(0),
            points: Cell::new(0),
        }
    }

    /// Charge before the actual sole public point-constructor call.
    /// # Errors
    /// Returns a harness error on count overflow; preserves the typed product result.
    fn point(&self, x: f64, y: f64, divisor: f64) -> TestResult<Result<FitPoint, FitError>> {
        charge(&self.points, 65_536)?;
        Ok(FitPoint::new(x, y, divisor))
    }

    /// Charge before the actual sole public fit call, without retries.
    /// # Errors
    /// Returns a harness error on count overflow; preserves the typed product result.
    fn fit(&self, input: &FitInput) -> TestResult<Result<FixtureLine, FitError>> {
        charge(&self.fits, 512)?;
        Ok(fit(input))
    }
}

impl Drop for Profile {
    /// Best-effort bounded stderr accounting also runs during assertion unwinding.
    /// A missing record is an incomplete observation for the external supervisor.
    fn drop(&mut self) {
        let record = format!(
            "CORE_PROFILE_COUNTS fits={} points={}\n",
            self.fits.get(),
            self.points.get()
        );
        let _emission_result = io::stderr().lock().write_all(record.as_bytes());
    }
}

/// Increment a test-local attempt counter before its public operation.
/// # Errors
/// Rejects overflow or a per-scope ceiling before performing the counted operation.
fn charge(counter: &Cell<u64>, ceiling: u64) -> TestResult {
    let next = counter
        .get()
        .checked_add(1)
        .ok_or_else(|| io::Error::other("profile count overflow"))?;
    if next > ceiling {
        return Err(io::Error::other("profile count ceiling").into());
    }
    counter.set(next);
    Ok(())
}

/// Construct a nonzero asserted fixture frame; it grants no native/trust authority.
/// # Errors
/// Rejects a zero test identifier without inventing a default.
fn frame(value: u64) -> TestResult<NonZeroU64> {
    Ok(NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero test frame"))?)
}

/// Admit exactly N raw rows once each, retaining fixed storage and chronological order.
/// # Errors
/// Rejects an empty helper input or a real point-constructor failure.
fn admit<const N: usize>(profile: &Profile, rows: [[f64; 3]; N]) -> TestResult<[FitPoint; N]> {
    let first = rows
        .first()
        .ok_or_else(|| io::Error::other("empty admission helper"))?;
    let initial = profile.point(first[0], first[1], first[2])??;
    let mut points = [initial; N];
    for (target, row) in points.iter_mut().zip(rows).skip(1) {
        *target = profile.point(row[0], row[1], row[2])??;
    }
    Ok(points)
}

/// Observe all immutable scalar bits, including either signed zero.
fn point_bits(point: &FitPoint) -> [u64; 3] {
    [
        point.relative_seconds().to_bits(),
        point.phase_seconds().to_bits(),
        point.variance_divisor().to_bits(),
    ]
}

/// Compare finite coefficients against predeclared numerical tolerances only.
fn coefficients_match(actual_phase: f64, actual_rate: f64, phase: f64, rate: f64) -> bool {
    actual_phase.is_finite()
        && actual_rate.is_finite()
        && (actual_phase - phase).abs() <= PHASE_TOLERANCE + RELATIVE_TOLERANCE * phase.abs()
        && (actual_rate - rate).abs() <= RATE_TOLERANCE + RELATIVE_TOLERANCE * rate.abs()
}

/// Exercise one actual fit and all public result/frame/source preservation facts.
/// # Errors
/// Propagates constructor/geometry/fit failure; `NotImplemented` is never accepted.
/// # Panics
/// Asserts independently expected coefficients, unchanged point bits and frame/count.
fn fit_rows<const N: usize>(
    profile: &Profile,
    identity: u64,
    rows: [[f64; 3]; N],
    phase: f64,
    rate: f64,
) -> TestResult<FixtureLine> {
    let points = admit(profile, rows)?;
    let before = points.map(|point| point_bits(&point));
    let expected_frame = frame(identity)?;
    let input = FitInput::new(expected_frame, &points)?;
    assert_eq!(input.frame(), expected_frame);
    assert_eq!(input.sample_count(), N);
    let line = profile.fit(&input)??;
    assert_eq!(line.frame(), expected_frame);
    assert!(
        coefficients_match(
            line.phase_seconds(),
            line.phase_rate_seconds_per_second(),
            phase,
            rate
        ),
        "actual=({:?},{:?}) expected=({phase:?},{rate:?})",
        line.phase_seconds(),
        line.phase_rate_seconds_per_second()
    );
    assert_eq!(points.map(|point| point_bits(&point)), before);
    assert_eq!(input.frame(), expected_frame);
    assert_eq!(input.sample_count(), N);
    Ok(line)
}

/// One exact-rational three-point fact, authored before Rust implementation.
struct LiteralCase {
    /// Stable independent fixture identity for diagnostic context.
    name: &'static str,
    /// Relative seconds, positive-behind phase seconds and variance divisor.
    rows: [[f64; 3]; 3],
    /// Exact rational intercept expressed as its independently rounded scalar.
    phase: f64,
    /// Exact rational phase derivative, seconds/second.
    rate: f64,
}

/// First twelve activation facts; the thirteenth has its own 64-point consumer.
const LITERALS: [LiteralCase; 12] = [
    LiteralCase {
        name: "constant",
        rows: [
            [-2.0, 1.0 / 500.0, 1.0],
            [-1.0, 1.0 / 500.0, 1.0],
            [0.0, 1.0 / 500.0, 4.0],
        ],
        phase: 1.0 / 500.0,
        rate: 0.0,
    },
    LiteralCase {
        name: "linear",
        rows: [
            [-2.0, 1.0 / 200.0, 1.0],
            [-1.0, 7.0 / 1000.0, 1.0],
            [0.0, 9.0 / 1000.0, 4.0],
        ],
        phase: 9.0 / 1000.0,
        rate: 1.0 / 500.0,
    },
    LiteralCase {
        name: "variance_divisor_direction",
        rows: [
            [-2.0, -1.0 / 1000.0, 1.0],
            [-1.0, 0.0, 1.0],
            [0.0, 3.0 / 1000.0, 4.0],
        ],
        phase: 19.0 / 9000.0,
        rate: 1.0 / 600.0,
    },
    LiteralCase {
        name: "equal_divisor_neighbor",
        rows: [
            [-2.0, -1.0 / 1000.0, 1.0],
            [-1.0, 0.0, 1.0],
            [0.0, 3.0 / 1000.0, 1.0],
        ],
        phase: 1.0 / 375.0,
        rate: 1.0 / 500.0,
    },
    LiteralCase {
        name: "wrong_multiplying_direction",
        rows: [
            [-2.0, -1.0 / 1000.0, 1.0],
            [-1.0, 0.0, 1.0],
            [0.0, 3.0 / 1000.0, 1.0 / 4.0],
        ],
        phase: 61.0 / 21000.0,
        rate: 3.0 / 1400.0,
    },
    LiteralCase {
        name: "newest_reference_origin",
        rows: [
            [-2.0, -1.0 / 1000.0, 1.0],
            [-1.0, 0.0, 1.0],
            [0.0, 3.0 / 1000.0, 4.0],
        ],
        phase: 19.0 / 9000.0,
        rate: 1.0 / 600.0,
    },
    LiteralCase {
        name: "scaled_divisors",
        rows: [
            [-2.0, -1.0 / 1000.0, 4.0],
            [-1.0, 0.0, 4.0],
            [0.0, 3.0 / 1000.0, 16.0],
        ],
        phase: 19.0 / 9000.0,
        rate: 1.0 / 600.0,
    },
    LiteralCase {
        name: "opposite_sign",
        rows: [
            [-2.0, 1.0 / 1000.0, 1.0],
            [-1.0, 0.0, 1.0],
            [0.0, -3.0 / 1000.0, 4.0],
        ],
        phase: -19.0 / 9000.0,
        rate: -1.0 / 600.0,
    },
    LiteralCase {
        name: "output_above_input_phase_bound",
        rows: [[-2.0, -1.0, 1.0], [-1.0, 1.0, 1.0], [0.0, 1.0, 1.0]],
        phase: 4.0 / 3.0,
        rate: 1.0,
    },
    LiteralCase {
        name: "zero_constant",
        rows: [[-2.0, 0.0, 1.0], [-1.0, 0.0, 16.0], [0.0, 0.0, 16.0]],
        phase: 0.0,
        rate: 0.0,
    },
    LiteralCase {
        name: "minimum_span",
        rows: [
            [-1.0 / 16.0, -1.0 / 1000.0, 1.0],
            [-1.0 / 32.0, 0.0, 1.0],
            [0.0, 3.0 / 1000.0, 4.0],
        ],
        phase: 19.0 / 9000.0,
        rate: 4.0 / 75.0,
    },
    LiteralCase {
        name: "maximum_span",
        rows: [
            [-4096.0, -1.0 / 1000.0, 1.0],
            [-2048.0, 0.0, 1.0],
            [0.0, 3.0 / 1000.0, 4.0],
        ],
        phase: 19.0 / 9000.0,
        rate: 1.0 / 1_228_800.0,
    },
];

/// Execute every three-point activation fact once, preserving the published inventory.
/// # Errors
/// Propagates any constructor or actual fit failure.
/// # Panics
/// Asserts the independent coefficients and distinguishing 4/3-second output.
#[test]
fn twelve_three_point_activation_facts() -> TestResult {
    let profile = Profile::new();
    for (index, case) in LITERALS.iter().enumerate() {
        let identity = u64::try_from(index)?
            .checked_add(1)
            .ok_or_else(|| io::Error::other("frame overflow"))?;
        let line = fit_rows(&profile, identity, case.rows, case.phase, case.rate)?;
        if case.name == "output_above_input_phase_bound" {
            assert!(line.phase_seconds() > 1.0);
        }
    }
    Ok(())
}

/// Consume activation fact 13 using its exact dyadic line and alternating divisors.
/// # Errors
/// Propagates admission, frame or fit failure.
/// # Panics
/// Asserts the line at frame 13, 64-point count and exact-rational coefficients.
#[test]
fn sixty_four_point_activation_fact() -> TestResult {
    let profile = Profile::new();
    let mut rows = [[0.0; 3]; 64];
    for (index, row) in rows.iter_mut().enumerate() {
        let integer = i32::try_from(index)?;
        let x = f64::from(integer - 63);
        let divisor = match index % 3 {
            0 => 1.0 / 16.0,
            1 => 1.0,
            _ => 16.0,
        };
        *row = [x, 1.0 / 8.0 + x / 1024.0, divisor];
    }
    fit_rows(&profile, 13, rows, 1.0 / 8.0, 1.0 / 1024.0)?;
    Ok(())
}

/// Preserve exact constants before any potentially underflowing response reduction.
/// # Errors
/// Propagates public admission/fit failure; no fallback result is accepted.
/// # Panics
/// Asserts original constant bits, numerical zero rates and preserved signed-zero inputs.
#[test]
fn uneven_constants_minimum_normal_and_signed_zeros_are_exact() -> TestResult {
    let profile = Profile::new();
    for phase in [0.1_f64, f64::MIN_POSITIVE, -f64::MIN_POSITIVE, 0.0, -0.0] {
        let line = fit_rows(
            &profile,
            20,
            [[-2.0, phase, 1.0], [-1.0, phase, 16.0], [0.0, phase, 16.0]],
            phase,
            0.0,
        )?;
        assert_eq!(line.phase_seconds().to_bits(), phase.to_bits());
        assert_eq!(
            line.phase_rate_seconds_per_second().to_bits() & 0x7fff_ffff_ffff_ffff,
            0
        );
    }
    let line = fit_rows(
        &profile,
        21,
        [[-2.0, -0.0, 1.0], [-1.0, 0.0, 16.0], [-0.0, -0.0, 16.0]],
        0.0,
        0.0,
    )?;
    assert_eq!(line.phase_seconds().to_bits(), (-0.0_f64).to_bits());
    assert_eq!(
        line.phase_rate_seconds_per_second().to_bits() & 0x7fff_ffff_ffff_ffff,
        0
    );
    Ok(())
}

/// Compare unit-only field codes without requiring an unadvertised equality trait.
fn field_matches(actual: PointField, expected: PointField) -> bool {
    matches!(
        (actual, expected),
        (PointField::RelativeSeconds, PointField::RelativeSeconds)
            | (PointField::PhaseSeconds, PointField::PhaseSeconds)
            | (PointField::VarianceDivisor, PointField::VarianceDivisor)
    )
}

/// Compare unit-only issue codes, retaining nonfinite/subnormal/range distinctions.
fn issue_matches(actual: PointIssue, expected: PointIssue) -> bool {
    matches!(
        (actual, expected),
        (PointIssue::NonFinite, PointIssue::NonFinite)
            | (PointIssue::Subnormal, PointIssue::Subnormal)
            | (PointIssue::OutOfRange, PointIssue::OutOfRange)
    )
}

/// Reach the actual point constructor and assert its first useful typed issue.
/// # Errors
/// Propagates attempt-counter failure only.
/// # Panics
/// Rejects acceptance, wrong field/issue or a generic validation fallback.
fn reject_point(
    profile: &Profile,
    raw: [f64; 3],
    field: PointField,
    issue: PointIssue,
) -> TestResult {
    let result = profile.point(raw[0], raw[1], raw[2])?;
    let matches = match result.as_ref() {
        Err(FitError::Point {
            field: actual,
            issue: reason,
        }) => field_matches(*actual, field) && issue_matches(*reason, issue),
        _ => false,
    };
    assert!(
        matches,
        "expected field/issue, got {:?}",
        result.as_ref().err()
    );
    Ok(())
}

/// Challenge all three public argument positions with nonfinite raw values.
/// # Errors
/// Propagates test-counter failure only.
/// # Panics
/// Asserts that every constructor call returns its actual typed nonfinite issue.
fn reject_nonfinite_fields(profile: &Profile) -> TestResult {
    for field in 0..3 {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut raw = [-1.0, 0.25, 1.0];
            raw[field] = value;
            let expected = match field {
                0 => PointField::RelativeSeconds,
                1 => PointField::PhaseSeconds,
                _ => PointField::VarianceDivisor,
            };
            reject_point(profile, raw, expected, PointIssue::NonFinite)?;
        }
    }
    Ok(())
}

/// Challenge smallest/largest nonzero subnormals at both signs in every field.
/// # Errors
/// Propagates test-counter failure only.
/// # Panics
/// Asserts subnormal classification precedes each affected field's range rejection.
fn reject_subnormal_fields(profile: &Profile) -> TestResult {
    for field in 0..3 {
        for bits in [
            1,
            0x8000_0000_0000_0001,
            0x000f_ffff_ffff_ffff,
            0x800f_ffff_ffff_ffff,
        ] {
            let mut raw = [-1.0, 0.25, 1.0];
            raw[field] = f64::from_bits(bits);
            let expected = match field {
                0 => PointField::RelativeSeconds,
                1 => PointField::PhaseSeconds,
                _ => PointField::VarianceDivisor,
            };
            reject_point(profile, raw, expected, PointIssue::Subnormal)?;
        }
    }
    Ok(())
}

/// Challenge every field's finite/subnormal admission and cross-field precedence.
/// # Errors
/// Propagates counter or useful healthy-neighbor admission failure.
/// # Panics
/// Asserts typed first-field errors and exact healthy getter bits.
#[test]
fn point_nonfinite_subnormal_and_precedence_controls() -> TestResult {
    let profile = Profile::new();
    reject_nonfinite_fields(&profile)?;
    reject_subnormal_fields(&profile)?;
    reject_point(
        &profile,
        [f64::NAN, f64::INFINITY, -1.0],
        PointField::RelativeSeconds,
        PointIssue::NonFinite,
    )?;
    reject_point(
        &profile,
        [1.0, f64::NAN, 0.0],
        PointField::RelativeSeconds,
        PointIssue::OutOfRange,
    )?;
    reject_point(
        &profile,
        [-1.0, f64::NAN, 0.0],
        PointField::PhaseSeconds,
        PointIssue::NonFinite,
    )?;
    reject_point(
        &profile,
        [-1.0, f64::from_bits(1), f64::NAN],
        PointField::PhaseSeconds,
        PointIssue::Subnormal,
    )?;
    reject_point(
        &profile,
        [-1.0, 2.0, f64::NAN],
        PointField::PhaseSeconds,
        PointIssue::OutOfRange,
    )?;
    for raw in [
        [-f64::MIN_POSITIVE, f64::MIN_POSITIVE, 1.0],
        [-0.0, -0.0, 1.0],
        [0.0, 0.0, 1.0],
    ] {
        let point = profile.point(raw[0], raw[1], raw[2])??;
        assert_eq!(point_bits(&point), raw.map(f64::to_bits));
    }
    Ok(())
}

/// Challenge inclusive range endpoints and independently encoded adjacent scalars.
/// # Errors
/// Propagates counter or accepted-neighbor admission failure.
/// # Panics
/// Asserts exact typed rejection or bit-preserving acceptance for each neighbor.
#[test]
fn point_range_endpoints_and_adjacent_values() -> TestResult {
    let profile = Profile::new();
    for raw in [
        [-4096.0, -1.0, 1.0 / 16.0],
        [0.0, 1.0, 16.0],
        [f64::from_bits(0xc0af_ffff_ffff_ffff), 0.0, 1.0],
        [-1.0, f64::from_bits(0x3fef_ffff_ffff_ffff), 1.0],
        [-1.0, -f64::from_bits(0x3fef_ffff_ffff_ffff), 1.0],
        [-1.0, 0.0, f64::from_bits(0x3fb0_0000_0000_0001)],
        [-1.0, 0.0, f64::from_bits(0x402f_ffff_ffff_ffff)],
    ] {
        let point = profile.point(raw[0], raw[1], raw[2])??;
        assert_eq!(point_bits(&point), raw.map(f64::to_bits));
    }
    reject_point(
        &profile,
        [f64::from_bits(0xc0b0_0000_0000_0001), 0.0, 1.0],
        PointField::RelativeSeconds,
        PointIssue::OutOfRange,
    )?;
    reject_point(
        &profile,
        [f64::MIN_POSITIVE, 0.0, 1.0],
        PointField::RelativeSeconds,
        PointIssue::OutOfRange,
    )?;
    for phase in [
        f64::from_bits(0x3ff0_0000_0000_0001),
        -f64::from_bits(0x3ff0_0000_0000_0001),
    ] {
        reject_point(
            &profile,
            [-1.0, phase, 1.0],
            PointField::PhaseSeconds,
            PointIssue::OutOfRange,
        )?;
    }
    for divisor in [
        f64::from_bits(0x3faf_ffff_ffff_ffff),
        f64::from_bits(0x4030_0000_0000_0001),
        0.0,
        -0.0,
        -1.0,
    ] {
        reject_point(
            &profile,
            [-1.0, 0.0, divisor],
            PointField::VarianceDivisor,
            PointIssue::OutOfRange,
        )?;
    }
    Ok(())
}

/// The exact batch rejection categories independently required by the contract.
enum BatchIssue {
    /// Raw cardinality is outside the inclusive 3..=64 envelope.
    Count,
    /// First nonincreasing-time index, independently fixed below 64.
    Order(usize),
    /// The final relative timestamp is not numerically zero.
    Reference,
    /// The admitted point coordinates have too little total span.
    Span,
    /// Supported calculated geometry is weaker than the exact cutoff.
    Spread,
}

/// Retain at most 65 source-point bit records without allocating.
/// # Errors
/// Rejects an oversized test helper input before indexing its fixed storage.
fn snapshot(points: &[FitPoint]) -> TestResult<[[u64; 3]; 65]> {
    if points.len() > 65 {
        return Err(io::Error::other("snapshot test capacity").into());
    }
    let mut records = [[0; 3]; 65];
    for (record, point) in records.iter_mut().zip(points) {
        *record = point_bits(point);
    }
    Ok(records)
}

/// Call the actual batch constructor and retain its source facts on rejection.
/// # Errors
/// Propagates frame/snapshot helper failure only.
/// # Panics
/// Asserts the exact typed batch guard and unchanged source bits.
fn reject_batch(points: &[FitPoint], expected: BatchIssue) -> TestResult {
    let before = snapshot(points)?;
    let result = FitInput::new(frame(30)?, points);
    let matches = match (result.as_ref(), expected) {
        (Err(FitError::CountOutOfRange), BatchIssue::Count)
        | (Err(FitError::ReferenceNotZero), BatchIssue::Reference)
        | (Err(FitError::SpanOutOfRange), BatchIssue::Span)
        | (Err(FitError::InsufficientSpread), BatchIssue::Spread) => true,
        (Err(FitError::NonIncreasingTime { index }), BatchIssue::Order(expected_index)) => {
            *index == expected_index
        }
        _ => false,
    };
    assert!(
        matches,
        "unexpected batch result {:?}",
        result.as_ref().err()
    );
    assert_eq!(snapshot(points)?, before);
    Ok(())
}

/// Preserve count-first admission, valid 3/64 controls and immutable frame metadata.
/// # Errors
/// Propagates helpers or healthy constructor failure.
/// # Panics
/// Asserts count errors before invalid ordering and accepts useful count endpoints.
#[test]
fn batch_count_guards_and_healthy_three_and_sixty_four() -> TestResult {
    let profile = Profile::new();
    let point = profile.point(-1.0, 0.0, 1.0)??;
    let over = [point; 65];
    reject_batch(&[], BatchIssue::Count)?;
    reject_batch(&over[..1], BatchIssue::Count)?;
    reject_batch(&over[..2], BatchIssue::Count)?;
    reject_batch(&over, BatchIssue::Count)?;
    let three = admit(
        &profile,
        [[-2.0, 0.0, 1.0], [-1.0, 0.0, 1.0], [-0.0, 0.0, 1.0]],
    )?;
    let input = FitInput::new(frame(31)?, &three)?;
    assert_eq!(input.sample_count(), 3);
    assert_eq!(input.frame(), frame(31)?);
    let mut rows = [[0.0; 3]; 64];
    for (index, row) in rows.iter_mut().enumerate() {
        *row = [f64::from(i32::try_from(index)? - 63), 0.0, 1.0];
    }
    let points = admit(&profile, rows)?;
    let input = FitInput::new(frame(u64::MAX)?, &points)?;
    assert_eq!(input.sample_count(), 64);
    assert_eq!(input.frame(), frame(u64::MAX)?);
    Ok(())
}

/// Verify batch guard precedence, span neighbors and geometry-only weak spread.
/// # Errors
/// Propagates point/helper or healthy batch construction failure.
/// # Panics
/// Asserts exact guards and preserves legal minimum/maximum-span neighbors.
#[test]
fn batch_order_reference_span_and_spread_guards() -> TestResult {
    let profile = Profile::new();
    let duplicate = admit(
        &profile,
        [[-2.0, 0.0, 1.0], [-2.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
    )?;
    reject_batch(&duplicate, BatchIssue::Order(1))?;
    let descending = admit(
        &profile,
        [[-2.0, 0.0, 1.0], [0.0, 0.0, 1.0], [-1.0, 0.0, 1.0]],
    )?;
    reject_batch(&descending, BatchIssue::Order(2))?;
    let reference = admit(
        &profile,
        [[-2.0, 0.0, 1.0], [-1.0, 0.0, 1.0], [-0.5, 0.0, 1.0]],
    )?;
    reject_batch(&reference, BatchIssue::Reference)?;
    let short = f64::from_bits(0x3faf_ffff_ffff_ffff);
    let short_span = admit(
        &profile,
        [
            [-short, 0.0, 1.0],
            [-short / 2.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    )?;
    reject_batch(&short_span, BatchIssue::Span)?;
    let weak = admit(
        &profile,
        [[-2.0, 0.0, 16.0], [-1.0, 0.0, 1.0 / 16.0], [0.0, 0.0, 16.0]],
    )?;
    reject_batch(&weak, BatchIssue::Spread)?;
    for span in [1.0 / 16.0, 4096.0] {
        let points = admit(
            &profile,
            [[-span, 0.0, 1.0], [-span / 2.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
        )?;
        let input = FitInput::new(frame(32)?, &points)?;
        assert_eq!(input.sample_count(), 3);
    }
    Ok(())
}

/// Check transformed geometry even when all response values are exact zero.
/// # Errors
/// Propagates point/counter failure or the independently healthy geometry failure.
/// # Panics
/// Asserts normal input can reject at the actual geometry divide/multiply owner.
#[test]
fn geometry_underflow_is_checked_before_constant_response() -> TestResult {
    let profile = Profile::new();
    let cases = [
        (-f64::MIN_POSITIVE, 1.0),
        (-2.0 * f64::MIN_POSITIVE, 16.0),
        (-2.0 * f64::MIN_POSITIVE, 1.0),
    ];
    for (index, (middle, divisor)) in cases.into_iter().enumerate() {
        let points = admit(
            &profile,
            [[-2.0, 0.0, 1.0], [middle, 0.0, divisor], [0.0, 0.0, 1.0]],
        )?;
        let before = snapshot(&points)?;
        let result = FitInput::new(frame(33)?, &points);
        if index == 2 {
            let input = result?;
            assert_eq!(input.sample_count(), 3);
        } else {
            let matches = matches!(
                (&result, index),
                (
                    Err(FitError::Arithmetic {
                        stage: ArithmeticStage::Geometry,
                        operation: ArithmeticOperation::Divide,
                        issue: ArithmeticIssue::Subnormal
                    }),
                    0
                ) | (
                    Err(FitError::Arithmetic {
                        stage: ArithmeticStage::Geometry,
                        operation: ArithmeticOperation::Multiply,
                        issue: ArithmeticIssue::Subnormal
                    }),
                    1
                )
            );
            assert!(
                matches,
                "unexpected geometry result {:?}",
                result.as_ref().err()
            );
        }
        assert_eq!(snapshot(&points)?, before);
    }
    Ok(())
}

/// Freeze calculated-binary64 cutoff semantics separately from exact-real geometry.
/// # Errors
/// Propagates useful constructor/fit failure in the two admitted neighbors.
/// # Panics
/// Asserts reject/accept/accept for the three consecutive frozen divisor bit patterns.
#[test]
fn calculated_spread_cutoff_neighbors_are_exact() -> TestResult {
    let profile = Profile::new();
    for (bits, accepted) in [
        (0x3fc0_4104_1041_040f, false),
        (0x3fc0_4104_1041_0410, true),
        (0x3fc0_4104_1041_0411, true),
    ] {
        let points = admit(
            &profile,
            [
                [-2.0, 0.0, 16.0],
                [-1.0, 0.0, f64::from_bits(bits)],
                [0.0, 0.0, 16.0],
            ],
        )?;
        if accepted {
            let input = FitInput::new(frame(40)?, &points)?;
            let line = profile.fit(&input)??;
            assert_eq!(line.phase_seconds().to_bits() & 0x7fff_ffff_ffff_ffff, 0);
            assert_eq!(
                line.phase_rate_seconds_per_second().to_bits() & 0x7fff_ffff_ffff_ffff,
                0
            );
        } else {
            reject_batch(&points, BatchIssue::Spread)?;
        }
    }
    Ok(())
}

/// Exercise unsupported nonconstant operations without mislabelling raw input errors.
/// # Errors
/// Propagates admitted geometry or harness failure; fit must reach response arithmetic.
/// # Panics
/// Asserts typed subnormal/zero-underflow, repeatable rejection and preserved source/frame.
#[test]
fn nonconstant_response_underflow_is_typed_and_preserves_inputs() -> TestResult {
    let profile = Profile::new();
    let next_above_minus_one = f64::from_bits(0xbfef_ffff_ffff_ffff);
    let rows = [
        [
            [-2.0, 0.0, 1.0],
            [-1.0, f64::MIN_POSITIVE, 16.0],
            [0.0, 0.0, 16.0],
        ],
        [
            [-2.0, -0.5, 1.0],
            [next_above_minus_one, f64::MIN_POSITIVE, 1.0],
            [0.0, 0.5, 1.0],
        ],
    ];
    for (index, raw) in rows.into_iter().enumerate() {
        let points = admit(&profile, raw)?;
        let before = snapshot(&points)?;
        let input = FitInput::new(frame(50)?, &points)?;
        for _ in 0..2 {
            let result = profile.fit(&input)?;
            let matches = matches!(
                (&result, index),
                (
                    Err(FitError::Arithmetic {
                        stage: ArithmeticStage::Response,
                        operation: ArithmeticOperation::Multiply,
                        issue: ArithmeticIssue::Subnormal
                    }),
                    0
                ) | (
                    Err(FitError::Arithmetic {
                        stage: ArithmeticStage::Response,
                        operation: ArithmeticOperation::Multiply,
                        issue: ArithmeticIssue::ZeroUnderflow
                    }),
                    1
                )
            );
            assert!(
                matches,
                "unexpected response result {:?}",
                result.as_ref().err()
            );
            assert_eq!(snapshot(&points)?, before);
            assert_eq!(input.frame(), frame(50)?);
            assert_eq!(input.sample_count(), 3);
        }
    }
    Ok(())
}

/// Prove owned batch data, a distinct result envelope and unchanged public state.
/// # Errors
/// Propagates admission or either actual fit failure.
/// # Panics
/// Asserts the 4/3-second result remains legal after the caller replaces a source point.
#[test]
fn result_envelope_frame_and_owned_input_are_preserved() -> TestResult {
    let profile = Profile::new();
    let mut points = admit(
        &profile,
        [[-2.0, -1.0, 1.0], [-1.0, 1.0, 1.0], [0.0, 1.0, 1.0]],
    )?;
    let before = snapshot(&points)?;
    let input = FitInput::new(frame(u64::MAX)?, &points)?;
    let first = profile.fit(&input)??;
    assert!(coefficients_match(
        first.phase_seconds(),
        first.phase_rate_seconds_per_second(),
        4.0 / 3.0,
        1.0
    ));
    assert!(first.phase_seconds() > 1.0);
    assert_eq!(snapshot(&points)?, before);
    points[0] = profile.point(-2.0, 0.0, 1.0)??;
    let repeated = profile.fit(&input)??;
    assert!(coefficients_match(
        repeated.phase_seconds(),
        repeated.phase_rate_seconds_per_second(),
        4.0 / 3.0,
        1.0
    ));
    assert_eq!(first.frame(), frame(u64::MAX)?);
    assert_eq!(repeated.frame(), frame(u64::MAX)?);
    assert_eq!(input.frame(), frame(u64::MAX)?);
    assert_eq!(input.sample_count(), 3);
    assert_eq!(points[0].phase_seconds().to_bits(), 0.0_f64.to_bits());
    reject_point(
        &profile,
        [-1.0, 4.0 / 3.0, 1.0],
        PointField::PhaseSeconds,
        PointIssue::OutOfRange,
    )?;
    Ok(())
}

/// Derive 256 distinct lines from a 16-by-16 dyadic a/b grid and y=a+b*x.
/// Neither expected coefficient comes from a regression solver.
/// # Errors
/// Propagates checked fixture conversions, admission or actual fit failure.
/// # Panics
/// Asserts both phase signs, zero/nonzero rates, divisor rotations and frame preservation.
#[test]
fn bounded_256_analytic_lines_have_known_coefficients() -> TestResult {
    let profile = Profile::new();
    let divisors = [1.0 / 16.0, 1.0, 4.0, 16.0];
    for index in 0_u16..256 {
        let phase = f64::from(i32::from(index % 16) - 8) / 64.0;
        let rate = f64::from(i32::from(index / 16) - 8) / 2048.0;
        let mut rows = [[0.0; 3]; 4];
        for (position, row) in rows.iter_mut().enumerate() {
            let x = f64::from(i32::try_from(position)? - 3);
            let rotation = usize::from(index)
                .checked_add(position)
                .ok_or_else(|| io::Error::other("rotation overflow"))?
                % 4;
            let divisor = *divisors
                .get(rotation)
                .ok_or_else(|| io::Error::other("divisor index"))?;
            *row = [x, phase + rate * x, divisor];
        }
        let identity = u64::from(index)
            .checked_add(1000)
            .ok_or_else(|| io::Error::other("analytic frame overflow"))?;
        fit_rows(&profile, identity, rows, phase, rate)?;
    }
    Ok(())
}

/// A changed semantic sentinel must fail the exact comparison owner used by fit tests.
/// # Panics
/// Asserts healthy/changed phase/changed rate controls without any product call.
#[test]
fn coefficient_comparator_detects_independent_sentinels() {
    let _profile = Profile::new();
    assert!(coefficients_match(4.0 / 3.0, 1.0, 4.0 / 3.0, 1.0));
    assert!(!coefficients_match(
        4.0 / 3.0 + 1.0 / 1024.0,
        1.0,
        4.0 / 3.0,
        1.0
    ));
    assert!(!coefficients_match(
        4.0 / 3.0,
        1.0 + 1.0 / 1024.0,
        4.0 / 3.0,
        1.0
    ));
    assert!(!coefficients_match(f64::NAN, 1.0, 4.0 / 3.0, 1.0));
}

/// Repeat admission through the actual public path in the optimized profile.
/// # Errors
/// Propagates counter/healthy-constructor or explicit stderr evidence failure.
/// # Panics
/// Asserts rejection survives `debug_assertions=false`; fit success is not required.
#[test]
fn release_build_preserves_regression_admission() -> TestResult {
    let profile = Profile::new();
    let record = format!(
        "CORE_REGRESSION_ADMISSION debug_assertions={}\n",
        cfg!(debug_assertions)
    );
    io::stderr().lock().write_all(record.as_bytes())?;
    reject_point(
        &profile,
        [f64::NAN, 0.0, 1.0],
        PointField::RelativeSeconds,
        PointIssue::NonFinite,
    )?;
    reject_point(
        &profile,
        [-1.0, 0.0, 0.0],
        PointField::VarianceDivisor,
        PointIssue::OutOfRange,
    )?;
    let valid = admit(
        &profile,
        [[-2.0, 0.0, 1.0], [-1.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
    )?;
    let input = FitInput::new(frame(60)?, &valid)?;
    assert_eq!(input.sample_count(), 3);
    reject_batch(&valid[..2], BatchIssue::Count)?;
    let duplicate = admit(
        &profile,
        [[-2.0, 0.0, 1.0], [-2.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
    )?;
    reject_batch(&duplicate, BatchIssue::Order(1))?;
    Ok(())
}

/// Observe bounded useful type context without exposing the numeric sentinels.
/// # Panics
/// Asserts the declared type-only or identity-only Debug surface, in either style.
fn assert_redacted_debug(rendered: &str, type_name: &str, sentinels: &[&str]) {
    assert!(
        rendered.starts_with(type_name),
        "missing Debug type context"
    );
    assert!(rendered.len() <= 256, "unbounded Debug context");
    for sentinel in sentinels {
        assert!(
            !rendered.contains(sentinel),
            "numeric Debug payload escaped"
        );
    }
    for field in [
        "relative_seconds",
        "phase_seconds",
        "variance_divisor",
        "phase_rate",
        "points",
        "geometry",
    ] {
        assert!(!rendered.contains(field), "private Debug field escaped");
    }
}

/// Compare semantic typed codes while leaving the formatter's prose unconstrained.
/// # Panics
/// Asserts useful context through Error, a finite diagnostic bound and no raw values.
fn assert_display_context(error: &FitError, expected_codes: &[&str]) {
    let typed_error: &dyn Error = error;
    let rendered = format!("{typed_error}");
    assert!(!rendered.is_empty(), "missing error context");
    assert!(rendered.len() <= 256, "unbounded error context");
    let codes: String = rendered
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|character| character.to_ascii_lowercase())
        .collect();
    for expected in expected_codes {
        assert!(
            codes.contains(expected),
            "missing typed error context: {expected}"
        );
    }
    for sentinel in ["317.25", "1.23456789", "3.5"] {
        assert!(!rendered.contains(sentinel), "raw error payload escaped");
    }
}

/// Point Debug is type-only even when all three admitted numeric values differ.
/// # Errors
/// Propagates bounded admission failures only.
/// # Panics
/// Asserts useful type context, absence of raw values/fields and value independence.
#[test]
fn point_debug_is_type_only() -> TestResult {
    let profile = Profile::new();
    let first = profile.point(-317.25, 0.3125, 1.125)??;
    let second = profile.point(-159.125, -0.1875, 7.25)??;
    let sentinels = ["317.25", "0.3125", "1.125", "159.125", "0.1875", "7.25"];
    for (left, right) in [
        (format!("{first:?}"), format!("{second:?}")),
        (format!("{first:#?}"), format!("{second:#?}")),
    ] {
        assert_redacted_debug(&left, "FitPoint", &sentinels);
        assert_redacted_debug(&right, "FitPoint", &sentinels);
        assert_eq!(left, right, "point Debug depends on numeric contents");
        assert!(!left.chars().any(|character| character.is_ascii_digit()));
    }
    Ok(())
}

/// Batch Debug may expose frame/count, while differing points and geometry stay private.
/// # Errors
/// Propagates bounded point/batch admission failures only.
/// # Panics
/// Asserts numeric redaction and equal output for equal permitted identity/count context.
#[test]
fn input_debug_excludes_points_and_geometry() -> TestResult {
    let profile = Profile::new();
    let first_points = admit(
        &profile,
        [
            [-317.25, 0.3125, 1.125],
            [-159.125, -0.1875, 3.5],
            [0.0, 0.4375, 7.25],
        ],
    )?;
    let second_points = admit(
        &profile,
        [
            [-285.5, -0.5625, 2.25],
            [-89.25, 0.6875, 5.5],
            [0.0, -0.8125, 11.25],
        ],
    )?;
    let identity = frame(8_640_123_457)?;
    let first = FitInput::new(identity, &first_points)?;
    let second = FitInput::new(identity, &second_points)?;
    assert_eq!(first.frame(), second.frame());
    assert_eq!(first.sample_count(), second.sample_count());
    let sentinels = [
        "317.25", "0.3125", "1.125", "159.125", "0.1875", "3.5", "0.4375", "7.25", "285.5",
        "0.5625", "2.25", "89.25", "0.6875", "5.5", "0.8125", "11.25",
    ];
    for (left, right) in [
        (format!("{first:?}"), format!("{second:?}")),
        (format!("{first:#?}"), format!("{second:#?}")),
    ] {
        assert_redacted_debug(&left, "FitInput", &sentinels);
        assert_redacted_debug(&right, "FitInput", &sentinels);
        assert_eq!(left, right, "batch Debug depends on points or geometry");
        assert!(!left.contains("FitPoint"));
    }
    Ok(())
}

/// Result Debug may expose frame, while actual input and coefficient changes stay private.
/// # Errors
/// Propagates bounded admission or either actual fit failure.
/// # Panics
/// Asserts healthy independent coefficients and numeric-independent Debug in both styles.
#[test]
fn line_debug_excludes_numeric_inputs_and_outputs() -> TestResult {
    let profile = Profile::new();
    let first = fit_rows(
        &profile,
        8_640_123_457,
        [
            [-317.25, 0.3125, 1.125],
            [-159.125, 0.3125, 3.5],
            [0.0, 0.3125, 7.25],
        ],
        0.3125,
        0.0,
    )?;
    let second = fit_rows(
        &profile,
        8_640_123_457,
        [
            [-2.0, -0.1875, 1.125],
            [-1.0, 0.125, 3.5],
            [0.0, 0.4375, 7.25],
        ],
        0.4375,
        0.3125,
    )?;
    assert_ne!(first.phase_seconds(), second.phase_seconds());
    assert_ne!(
        first.phase_rate_seconds_per_second(),
        second.phase_rate_seconds_per_second()
    );
    let sentinels = [
        "317.25", "159.125", "0.3125", "1.125", "3.5", "7.25", "0.1875", "0.4375",
    ];
    for (left, right) in [
        (format!("{first:?}"), format!("{second:?}")),
        (format!("{first:#?}"), format!("{second:#?}")),
    ] {
        assert_redacted_debug(&left, "FixtureLine", &sentinels);
        assert_redacted_debug(&right, "FixtureLine", &sentinels);
        assert_eq!(left, right, "result Debug depends on numeric contents");
    }
    Ok(())
}

/// Every declared error variant retains bounded typed context without raw payloads.
/// # Errors
/// Propagates the attempt counter or healthy neighboring point admission failure.
/// # Panics
/// Asserts all field/issue/stage/operation codes and a real rejection's redaction.
#[test]
fn error_display_preserves_bounded_typed_context() -> TestResult {
    let profile = Profile::new();
    for (field, field_code) in [
        (PointField::RelativeSeconds, "relativeseconds"),
        (PointField::PhaseSeconds, "phaseseconds"),
        (PointField::VarianceDivisor, "variancedivisor"),
    ] {
        for (issue, issue_code) in [
            (PointIssue::NonFinite, "nonfinite"),
            (PointIssue::Subnormal, "subnormal"),
            (PointIssue::OutOfRange, "outofrange"),
        ] {
            assert_display_context(&FitError::Point { field, issue }, &[field_code, issue_code]);
        }
    }
    for (stage, stage_code) in [
        (ArithmeticStage::Geometry, "geometry"),
        (ArithmeticStage::Response, "response"),
        (ArithmeticStage::Coefficients, "coefficients"),
        (ArithmeticStage::Result, "result"),
    ] {
        for (operation, operation_code) in [
            (ArithmeticOperation::Add, "add"),
            (ArithmeticOperation::Subtract, "subtract"),
            (ArithmeticOperation::Multiply, "multiply"),
            (ArithmeticOperation::Divide, "divide"),
        ] {
            for (issue, issue_code) in [
                (ArithmeticIssue::NonFinite, "nonfinite"),
                (ArithmeticIssue::Subnormal, "subnormal"),
                (ArithmeticIssue::ZeroUnderflow, "zerounderflow"),
                (
                    ArithmeticIssue::NonPositiveDenominator,
                    "nonpositivedenominator",
                ),
            ] {
                assert_display_context(
                    &FitError::Arithmetic {
                        stage,
                        operation,
                        issue,
                    },
                    &[stage_code, operation_code, issue_code],
                );
            }
        }
    }
    for (error, codes) in [
        (FitError::CountOutOfRange, &["count"][..]),
        (
            FitError::NonIncreasingTime { index: 1 },
            &["increasing", "1"][..],
        ),
        (
            FitError::NonIncreasingTime { index: 63 },
            &["increasing", "63"][..],
        ),
        (FitError::ReferenceNotZero, &["reference", "zero"][..]),
        (FitError::SpanOutOfRange, &["span"][..]),
        (FitError::InsufficientSpread, &["spread"][..]),
        (
            FitError::ValidationInvariant,
            &["validation", "invariant"][..],
        ),
    ] {
        assert_display_context(&error, codes);
    }
    let rejected = profile.point(-317.25, 1.234_567_89, 3.5)?;
    assert!(
        matches!(
            rejected.as_ref(),
            Err(FitError::Point {
                field: PointField::PhaseSeconds,
                issue: PointIssue::OutOfRange
            })
        ),
        "missing typed sentinel rejection"
    );
    if let Err(error) = rejected {
        assert_display_context(&error, &["phaseseconds", "outofrange"]);
    }
    let valid = profile.point(-317.25, 0.3125, 3.5)??;
    assert_eq!(valid.phase_seconds(), 0.3125);
    Ok(())
}
