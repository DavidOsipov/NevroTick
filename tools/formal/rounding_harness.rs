//! Full-domain source-bound rounding specification. No executable subject is modelled.

/// Full raw i128 domain for which multiplication by one billion fits i128.
/// Symmetry follows because one billion cannot divide 2^127 exactly.
const SCALE_LIMIT: i128 = 170_141_183_460_469_231_731_687_303_715;

/// Independently derived numerator endpoints for a rounded signed-i64 result.
/// The lower destination is even; the upper is odd, hence asymmetric ties.
fn admissible(phase: bool) -> (i128, i128, u32) {
    if phase {
        (-79_228_162_514_264_337_597, 79_228_162_514_264_337_589, 33)
    } else {
        (-39_614_081_257_132_168_798, 39_614_081_257_132_168_794, 32)
    }
}

/// Exercise the actual module's existing crate-private wrapper and public getters.
fn observe(n: i128, phase: bool) -> Result<(i64, i128, i128), DomainError> {
    if phase {
        rounding::round_offset(n).map(|v| {
            (
                v.nanoseconds(),
                v.residual_numerator(),
                v.residual_denominator(),
            )
        })
    } else {
        rounding::round_round_trip(n).map(|v| {
            (
                v.nanoseconds(),
                v.residual_numerator(),
                v.residual_denominator(),
            )
        })
    }
}

/// Complete success/error classification and the unique nearest-even answer.
///
/// An exact residual of magnitude at most half the denominator characterizes a
/// nearest integer. At either tie only an even integer is permitted. This oracle
/// uses reconstruction and literal domain endpoints, not the subject's division.
fn complete_contract(n: i128, phase: bool) {
    let observed = observe(n, phase);
    let (lower, upper, k) = admissible(phase);
    if n < -SCALE_LIMIT || n > SCALE_LIMIT {
        assert_eq!(
            observed,
            Err(DomainError::ArithmeticOverflow),
            "scale classification"
        );
    } else if n < lower || n > upper {
        assert_eq!(
            observed,
            Err(DomainError::NanosecondsOutOfRange),
            "destination classification"
        );
    } else {
        let (q, r, denominator) = match observed {
            Ok(value) => value,
            Err(_) => {
                assert!(false, "required success");
                return;
            }
        };
        // 10^9=512*1953125. The successful input interval makes every
        // specification intermediate fit i128 independently of the subject.
        let reduced_denominator = 1_i128 << (k - 9);
        assert_eq!(denominator, 1_i128 << k, "exact residual units");
        let exact_remainder = 512 * (n * 1_953_125 - i128::from(q) * reduced_denominator);
        assert_eq!(r, exact_remainder, "exact signed reconstruction");
        let halfway = 1_i128 << (k - 1);
        assert!(r >= -halfway && r <= halfway, "nearest integer");
        assert!(r != -halfway && r != halfway || q % 2 == 0, "even ties");
    }
}

/// Cover independently chosen inputs only in symbolic full-domain proofs.
fn witnesses(n: i128, phase: bool) {
    let (lower, upper, _) = admissible(phase);
    kani::cover!(n == 0, "zero succeeds");
    kani::cover!(n == lower, "negative destination endpoint succeeds");
    kani::cover!(n == upper, "positive destination endpoint succeeds");
    kani::cover!(n == lower - 1, "negative destination neighbor is rejected");
    kani::cover!(n == upper + 1, "positive destination neighbor is rejected");
    kani::cover!(n == i128::MIN, "raw signed minimum is covered");
}

#[kani::proof]
fn phase_full_domain() {
    let n = kani::any();
    complete_contract(n, true);
    witnesses(n, true);
}

#[kani::proof]
fn round_trip_full_domain() {
    let n = kani::any();
    complete_contract(n, false);
    witnesses(n, false);
}

/// Concrete controls distinguish specification sensitivity from global SAT cost.
#[kani::proof]
fn control_zero() {
    complete_contract(0, true);
}

#[kani::proof]
fn control_negative_tie() {
    // Exact -2,929,687.5 ns has even answer -2,929,688 ns.
    complete_contract(-25_165_824, true);
}

#[kani::proof]
fn control_scale_overflow() {
    complete_contract(i128::MAX, true);
}

#[kani::proof]
fn control_destination_overflow() {
    complete_contract(79_228_162_514_264_337_590, true);
}
