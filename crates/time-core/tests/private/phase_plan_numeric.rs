// SPDX-License-Identifier: GPL-2.0-only
//! Private finite phase-plan numerical controls, retaining their source-module identity.

use std::{error::Error, io};

use garde::Validate;
use time_domain::{ActiveAwakeDurationNs, ActiveAwakeNs, DomainError};

use super::{
    FixturePhaseNumericError, FixturePhaseNumericField, FixturePhaseNumericIssue,
    FixturePhaseNumericStep, PhaseDebt, PhaseRates, ProposalInputs, RateCandidate, RawProposal,
    RawRates, RawSettlement, SettlementInputs, delivery_error,
};
use crate::regression::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FixtureChartBoundIssue,
    FixtureChartBoundOperation, FixtureChartError, NormalizationComponent, NormalizationError,
};

/// Numerical controls preserve an unexpected typed rejection for the runner.
type TestResult = Result<(), Box<dyn Error>>;

/// Construct a duration through its actual typed owner, without raw fields.
/// # Errors
/// Preserve a time-domain rejection instead of fabricating an interval.
fn duration(nanoseconds: u64) -> Result<ActiveAwakeDurationNs, DomainError> {
    ActiveAwakeNs::new(nanoseconds).duration_since(ActiveAwakeNs::new(0))
}

/// Assert success of an independently specified supported numerical trajectory.
/// # Errors
/// Preserve the original typed error after its semantic success assertion.
/// # Panics
/// Reject refusal of the unchanged healthy numerical fixture.
#[track_caller]
fn product_ok<T, E: Error + 'static>(value: Result<T, E>) -> Result<T, Box<dyn Error>> {
    assert!(
        value.is_ok(),
        "valid phase-numerical fixture was refused: {:?}",
        value.as_ref().err().map(ToString::to_string)
    );
    value.map_err(Into::into)
}

/// Two actual two-second deliveries retain the original phase obligation.
/// # Errors
/// Preserve an unexpected checked arithmetic or typed-duration rejection.
/// # Panics
/// Detect dropping the baseline, resetting debt or reusing the initial horizon.
#[test]
fn successive_baseline_relative_dyadic_deliveries() -> TestResult {
    let initial = product_ok(PhaseDebt::new(1.0 / 32.0, 0.0))?;
    let first = product_ok(initial.settled(5.0 / 1024.0, 1.0 / 1024.0, duration(2_000_000_000)?))?;
    assert_eq!(first.point().to_bits(), 0x3f98_0000_0000_0000);
    let first_rates = product_ok(first.proposal(
        1.0 / 1024.0,
        duration(6_000_000_000)?,
        1.0 / 128.0,
        1.0 / 64.0,
    ))?;
    assert_eq!(
        first_rates.transient_rate().to_bits(),
        0x3f70_0000_0000_0000
    );
    assert_eq!(
        first_rates.aggregate_rate().to_bits(),
        (5.0_f64 / 1024.0).to_bits()
    );
    assert!(first_rates.has_useful_transient());
    let second = product_ok(first.settled(5.0 / 1024.0, 1.0 / 1024.0, duration(2_000_000_000)?))?;
    assert_eq!(second.point().to_bits(), 0x3f90_0000_0000_0000);
    let second_rates = product_ok(second.proposal(
        1.0 / 1024.0,
        duration(4_000_000_000)?,
        1.0 / 128.0,
        1.0 / 64.0,
    ))?;
    assert_eq!(
        second_rates.transient_rate().to_bits(),
        0x3f70_0000_0000_0000
    );
    assert!(second.error() > first.error());
    assert!(first_rates.transient_error() > 0.0);
    assert!(first_rates.aggregate_error() >= first_rates.transient_error());
    assert_eq!(initial.point().to_bits(), (1.0_f64 / 32.0).to_bits());
    Ok(())
}

/// The inherited enclosure is a debt input, not a replaceable local scratch.
/// # Errors
/// Preserve an unexpected checked constructor or interval failure.
/// # Panics
/// Detect either settlement or quotient dropping the inherited error.
#[test]
fn prior_error_survives_successive_settlement_and_division() -> TestResult {
    let initial_error = 1.0 / 4096.0;
    let first = product_ok(
        product_ok(PhaseDebt::new(1.0 / 32.0, initial_error))?.settled(
            5.0 / 1024.0,
            1.0 / 1024.0,
            duration(2_000_000_000)?,
        ),
    )?;
    let second = product_ok(first.settled(5.0 / 1024.0, 1.0 / 1024.0, duration(2_000_000_000)?))?;
    assert!(first.error() >= initial_error);
    assert!(second.error() >= first.error());
    let rates = product_ok(second.proposal(
        1.0 / 1024.0,
        duration(4_000_000_000)?,
        1.0 / 128.0,
        1.0 / 64.0,
    ))?;
    assert!(rates.transient_error() >= initial_error / 4.0);
    Ok(())
}

/// Actual quantization leaves exactly 25/1024 before a separately RN divide.
/// The exact error of RN(25/6144) is 1/3458764513820540928.
/// # Errors
/// Preserve any unexpected checked arithmetic rejection.
/// # Panics
/// Detect using a requested rate, the old horizon or a false exact quotient.
#[test]
fn quantized_actual_rate_has_independent_non_dyadic_quotient() -> TestResult {
    let debt = product_ok(product_ok(PhaseDebt::new(1.0 / 32.0, 0.0))?.settled(
        9.0 / 2048.0,
        1.0 / 1024.0,
        duration(2_000_000_000)?,
    ))?;
    assert_eq!(debt.point().to_bits(), 0x3f99_0000_0000_0000);
    let rates = product_ok(debt.proposal(
        1.0 / 1024.0,
        duration(6_000_000_000)?,
        1.0 / 128.0,
        1.0 / 64.0,
    ))?;
    assert_eq!(rates.transient_rate().to_bits(), 0x3f70_aaaa_aaaa_aaab);
    assert!(rates.transient_error() >= 2.891_205_793_294_679e-19);
    Ok(())
}

/// Exact H is 1000000001/1000000000, not its rounded seconds approximation.
/// The exact quotient is 1953125/2000000002; expected RN bits and outward
/// error are independently retained Fraction controls, not encoder output.
/// # Errors
/// Preserve an unexpected typed-duration or quotient rejection.
/// # Panics
/// Detect integer-second truncation or an error bound below the exact error.
#[test]
fn noninteger_remaining_seconds_keep_independent_point_and_error() -> TestResult {
    let rates = product_ok(product_ok(PhaseDebt::new(1.0 / 1024.0, 0.0))?.proposal(
        0.0,
        duration(1_000_000_001)?,
        1.0 / 128.0,
        1.0 / 64.0,
    ))?;
    assert_eq!(rates.transient_rate().to_bits(), 0x3f4f_ffff_ff76_8fa0);
    assert!(rates.transient_error() >= 8.177_770_605_282_264e-20);
    Ok(())
}

/// Transient direction follows baseline-relative debt, not aggregate sign.
/// # Errors
/// Preserve an unexpected supported arithmetic rejection.
/// # Panics
/// Detect a positive aggregate being misread as a positive transient.
#[test]
fn negative_transient_can_have_positive_aggregate() -> TestResult {
    let debt = product_ok(product_ok(PhaseDebt::new(-1.0 / 256.0, 0.0))?.settled(
        1.0 / 2048.0,
        1.0 / 1024.0,
        duration(2_000_000_000)?,
    ))?;
    assert_eq!(debt.point().to_bits(), 0xbf68_0000_0000_0000);
    let rates = product_ok(debt.proposal(
        1.0 / 1024.0,
        duration(6_000_000_000)?,
        1.0 / 128.0,
        1.0 / 64.0,
    ))?;
    assert_eq!(rates.transient_rate().to_bits(), 0xbf40_0000_0000_0000);
    assert_eq!(rates.aggregate_rate().to_bits(), 0x3f40_0000_0000_0000);
    Ok(())
}

/// Known complete delivery and overshoot retain their actual signed debt.
/// # Errors
/// Preserve an unexpected supported settlement rejection.
/// # Panics
/// Detect subtracting aggregate without baseline or erasing an overshoot.
#[test]
fn completion_and_overshoot_preserve_actual_debt() -> TestResult {
    let initial = product_ok(PhaseDebt::new(1.0 / 32.0, 0.0))?;
    let complete =
        product_ok(initial.settled(5.0 / 1024.0, 1.0 / 1024.0, duration(8_000_000_000)?))?;
    assert_eq!(complete.point().to_bits(), 0);
    assert!(complete.error() > 0.0);
    let rates =
        product_ok(complete.proposal(-0.0, duration(1_000_000_000)?, 1.0 / 128.0, 1.0 / 64.0))?;
    assert!(!rates.has_useful_transient());
    assert_eq!(rates.aggregate_rate().to_bits(), (-0.0_f64).to_bits());
    let overshoot =
        product_ok(initial.settled(6.0 / 1024.0, 1.0 / 1024.0, duration(8_000_000_000)?))?;
    assert_eq!(overshoot.point().to_bits(), 0xbf80_0000_0000_0000);
    Ok(())
}

/// A non-strict sign enclosure issues an exact baseline-only request.
/// # Errors
/// Preserve an unexpected legal numerical admission rejection.
/// # Panics
/// Detect sign invention, negative transient zero or baseline normalization.
#[test]
fn zero_and_ambiguous_sign_preserve_baseline_bits() -> TestResult {
    for (point, error) in [
        (0.0, 0.0),
        (-0.0, 0.0),
        (1.0 / 1024.0, 1.0 / 1024.0),
        (-1.0 / 1024.0, 1.0 / 512.0),
    ] {
        for baseline in [0.0_f64, -0.0, 1.0 / 1024.0, -1.0 / 1024.0] {
            let rates = product_ok(product_ok(PhaseDebt::new(point, error))?.proposal(
                baseline,
                duration(1)?,
                1.0 / 128.0,
                1.0 / 64.0,
            ))?;
            assert!(!rates.has_useful_transient());
            assert_eq!(rates.transient_rate().to_bits(), 0);
            assert_eq!(rates.transient_error().to_bits(), 0);
            assert_eq!(rates.aggregate_rate().to_bits(), baseline.to_bits());
            assert_eq!(rates.aggregate_error().to_bits(), 0);
        }
    }
    Ok(())
}

/// Explicit symmetric transient and aggregate point caps remain independent.
/// # Errors
/// Preserve unexpected rejection of the healthy exact cap neighbors.
/// # Panics
/// Detect clamp asymmetry or aggregate-cap enforcement before adding baseline.
#[test]
fn explicit_caps_clamp_transient_then_check_aggregate() -> TestResult {
    for sign in [1.0, -1.0] {
        let rates = product_ok(product_ok(PhaseDebt::new(sign / 32.0, 0.0))?.proposal(
            0.0,
            duration(1_000_000_000)?,
            1.0 / 128.0,
            1.0 / 64.0,
        ))?;
        assert_eq!(
            rates.transient_rate().to_bits(),
            (sign / 128.0_f64).to_bits()
        );
    }
    let debt = product_ok(PhaseDebt::new(1.0 / 32.0, 0.0))?;
    assert_eq!(
        debt.proposal(
            1.0 / 64.0,
            duration(1_000_000_000)?,
            1.0 / 128.0,
            1.0 / 64.0
        )
        .err(),
        Some(FixturePhaseNumericError::AggregateLimit)
    );
    assert_eq!(debt.point().to_bits(), (1.0_f64 / 32.0).to_bits());
    Ok(())
}

/// Complete raw admission rejects unsupported values before arithmetic.
/// # Panics
/// Detect a constructor bypass, negative uncertainty or unsupported scalars.
#[test]
fn debt_constructor_rejects_all_unsupported_scalar_classes() {
    for (point, error, field, issue) in [
        (
            f64::NAN,
            0.0,
            FixturePhaseNumericField::Debt,
            FixturePhaseNumericIssue::NonFinite,
        ),
        (
            f64::INFINITY,
            0.0,
            FixturePhaseNumericField::Debt,
            FixturePhaseNumericIssue::NonFinite,
        ),
        (
            f64::from_bits(1),
            0.0,
            FixturePhaseNumericField::Debt,
            FixturePhaseNumericIssue::Subnormal,
        ),
        (
            0.0,
            f64::NEG_INFINITY,
            FixturePhaseNumericField::DebtRoundoff,
            FixturePhaseNumericIssue::NonFinite,
        ),
        (
            0.0,
            f64::from_bits(1),
            FixturePhaseNumericField::DebtRoundoff,
            FixturePhaseNumericIssue::Subnormal,
        ),
        (
            0.0,
            -1.0,
            FixturePhaseNumericField::DebtRoundoff,
            FixturePhaseNumericIssue::Negative,
        ),
    ] {
        assert_eq!(
            PhaseDebt::new(point, error).err(),
            Some(FixturePhaseNumericError::Input { field, issue })
        );
    }
}

/// Typed intervals retain the existing exact integer conversion envelope.
/// # Errors
/// Preserve a legal typed-duration or numerical construction rejection.
/// # Panics
/// Detect silently rounded integer durations or an accepted empty horizon.
#[test]
fn zero_and_integer_duration_boundaries_are_explicit() -> TestResult {
    let debt = product_ok(PhaseDebt::new(1.0 / 1024.0, 0.0))?;
    assert_eq!(
        debt.proposal(0.0, duration(0)?, 1.0, 1.0).err(),
        Some(FixturePhaseNumericError::EmptyHorizon)
    );
    assert!(debt.settled(0.0, 0.0, duration(1_u64 << 53)?).is_ok());
    assert_eq!(
        debt.settled(0.0, 0.0, duration((1_u64 << 53) + 1)?).err(),
        Some(FixturePhaseNumericError::Duration {
            step: FixturePhaseNumericStep::ElapsedSeconds,
            source: NormalizationError::IntegerRange {
                component: NormalizationComponent::Time
            },
        })
    );
    assert_eq!(
        debt.proposal(0.0, duration((1_u64 << 53) + 1)?, 1.0, 1.0)
            .err(),
        Some(FixturePhaseNumericError::Duration {
            step: FixturePhaseNumericStep::RemainingSeconds,
            source: NormalizationError::IntegerRange {
                component: NormalizationComponent::Time
            },
        })
    );
    Ok(())
}

/// Supported neighboring inputs do not license overflow or hidden underflow.
/// # Errors
/// Preserve a legal constructor or interval rejection.
/// # Panics
/// Detect nonfinite/subnormal results being exposed as checked debt/rates.
#[test]
fn transformed_unsupported_results_are_rejected_atomically() -> TestResult {
    let debt = product_ok(PhaseDebt::new(1.0 / 1024.0, 0.0))?;
    assert!(matches!(
        debt.settled(f64::MAX, 0.0, duration(2_000_000_000)?),
        Err(FixturePhaseNumericError::Arithmetic {
            step: FixturePhaseNumericStep::AggregateDelivery,
            ..
        })
    ));
    assert!(matches!(
        debt.settled(f64::MIN_POSITIVE, 0.0, duration(1)?),
        Err(FixturePhaseNumericError::Arithmetic {
            step: FixturePhaseNumericStep::AggregateDelivery,
            ..
        })
    ));
    assert!(matches!(
        product_ok(PhaseDebt::new(f64::MAX, 0.0))?.proposal(0.0, duration(1)?, 1.0, 1.0),
        Err(FixturePhaseNumericError::Arithmetic {
            step: FixturePhaseNumericStep::TransientDivide,
            ..
        })
    ));
    assert_eq!(debt.point().to_bits(), (1.0_f64 / 1024.0).to_bits());
    assert_eq!(debt.error().to_bits(), 0);
    Ok(())
}

/// Obtain a real checked rejection without allowing an unexpected success to pass.
/// # Errors
/// Reject a malformed negative fixture instead of inventing a numerical error.
/// # Panics
/// Reject an invalid numerical fixture that the product unexpectedly admits.
fn failure<T>(
    result: Result<T, FixturePhaseNumericError>,
) -> Result<FixturePhaseNumericError, Box<dyn Error>> {
    assert!(
        result.is_err(),
        "invalid phase-numerical fixture was unexpectedly admitted"
    );
    result
        .err()
        .ok_or_else(|| io::Error::other("invalid numerical fixture unexpectedly admitted").into())
}

/// Both unavoidable result admission and its actual Garde rule reject the fault.
/// # Panics
/// Detect a missing output rule or a typed refusal from an unrelated field.
fn reject_rates(rates: PhaseRates, inputs: ProposalInputs, expected: FixturePhaseNumericError) {
    let raw = RawRates {
        value: RateCandidate { rates, inputs },
    };
    assert!(raw.validate().is_err());
    assert_eq!(PhaseRates::admit(rates, inputs).err(), Some(expected));
}

/// Reachable error families redact values while preserving actual typed causes.
/// # Errors
/// Preserve an unexpected checked constructor, duration or rejection failure.
/// # Panics
/// Detect payload-dependent diagnostics, missing causes or changed cause types.
#[test]
fn numeric_error_families_redact_and_delegate_typed_sources() -> TestResult {
    let debt = product_ok(PhaseDebt::new(1.0 / 32.0, 0.0))?;
    let input = failure(PhaseDebt::new(f64::NAN, 0.0))?;
    let other_input = failure(PhaseDebt::new(f64::INFINITY, 0.0))?;
    assert_eq!(input.to_string(), other_input.to_string());
    let conversion = failure(debt.settled(0.0, 0.0, duration((1_u64 << 53) + 1)?))?;
    let arithmetic = failure(debt.settled(f64::MAX, 0.0, duration(2_000_000_000)?))?;
    let bound =
        failure(product_ok(PhaseDebt::new(1.0, f64::MAX))?.settled(0.0, 0.0, duration(0)?))?;
    let empty = failure(debt.proposal(0.0, duration(0)?, 1.0, 1.0))?;
    let cap = failure(debt.proposal(
        1.0 / 64.0,
        duration(1_000_000_000)?,
        1.0 / 128.0,
        1.0 / 64.0,
    ))?;
    for error in [input, conversion, arithmetic, bound, empty, cap] {
        let diagnostic = error.to_string();
        assert!(diagnostic.starts_with("phase numeric"));
        assert!(
            diagnostic
                .chars()
                .all(|character| character.is_ascii_graphic() || character == ' ')
        );
        for payload in ["NaN", "inf", "1.7976931348623157", "9007199254740993"] {
            assert!(!diagnostic.contains(payload));
        }
    }
    for error in [input, empty, cap] {
        assert!(error.source().is_none());
    }
    assert_eq!(
        conversion
            .source()
            .and_then(<dyn Error>::downcast_ref::<NormalizationError>),
        Some(&NormalizationError::IntegerRange {
            component: NormalizationComponent::Time
        })
    );
    assert_eq!(
        arithmetic
            .source()
            .and_then(<dyn Error>::downcast_ref::<FitError>),
        Some(&FitError::Arithmetic {
            stage: ArithmeticStage::Coefficients,
            operation: ArithmeticOperation::Multiply,
            issue: ArithmeticIssue::NonFinite
        })
    );
    assert_eq!(
        bound
            .source()
            .and_then(<dyn Error>::downcast_ref::<FixtureChartError>),
        Some(&FixtureChartError::Bound {
            operation: FixtureChartBoundOperation::Add,
            issue: FixtureChartBoundIssue::NonFiniteResult
        })
    );
    Ok(())
}

/// Checked Debug is an opaque classification, independent of point/error payload.
/// # Errors
/// Preserve an unexpected checked numeric or typed-duration rejection.
/// # Panics
/// Detect either checked type exposing changed values or bounds through Debug.
#[test]
fn checked_debt_and_rates_debug_are_opaque_over_distinct_values() -> TestResult {
    let first = product_ok(PhaseDebt::new(1.0 / 32.0, 0.0))?;
    let second = product_ok(PhaseDebt::new(-1.0 / 64.0, 1.0 / 4096.0))?;
    assert_ne!(first.point().to_bits(), second.point().to_bits());
    assert_ne!(first.error().to_bits(), second.error().to_bits());
    assert_eq!(format!("{first:?}"), format!("{second:?}"));
    assert_eq!(format!("{first:?}"), "PhaseDebt { checked: true, .. }");
    let first_rates =
        product_ok(first.proposal(0.0, duration(8_000_000_000)?, 1.0 / 128.0, 1.0 / 64.0))?;
    let second_rates = product_ok(second.proposal(
        1.0 / 1024.0,
        duration(4_000_000_000)?,
        1.0 / 128.0,
        1.0 / 64.0,
    ))?;
    assert_ne!(
        first_rates.transient_rate().to_bits(),
        second_rates.transient_rate().to_bits()
    );
    assert_ne!(
        first_rates.aggregate_rate().to_bits(),
        second_rates.aggregate_rate().to_bits()
    );
    assert_eq!(format!("{first_rates:?}"), format!("{second_rates:?}"));
    assert_eq!(
        format!("{first_rates:?}"),
        "PhaseRates { checked: true, .. }"
    );
    Ok(())
}

/// Each transformed scalar retains its own mandatory rejection and field code.
/// # Errors
/// Preserve an unexpected typed-duration failure in the valid cap context.
/// # Panics
/// Detect a Garde/output-constructor bypass or a lost first-field diagnostic.
#[test]
fn transformed_scalar_faults_fail_raw_and_checked_rate_admission() -> TestResult {
    let inputs = ProposalInputs {
        baseline: -0.0,
        remaining: duration(1_000_000_000)?,
        transient_cap: 1.0 / 128.0,
        aggregate_cap: 1.0 / 64.0,
    };
    assert!(RawProposal { value: inputs }.validate().is_ok());
    let good = PhaseRates {
        transient: 1.0 / 256.0,
        transient_error: 0.0,
        aggregate: 1.0 / 256.0,
        aggregate_error: 0.0,
        useful: true,
    };
    assert!(PhaseRates::admit(good, inputs).is_ok());
    for (candidate, field, issue) in [
        (
            PhaseRates {
                transient: f64::NAN,
                ..good
            },
            FixturePhaseNumericField::TransientRate,
            FixturePhaseNumericIssue::NonFinite,
        ),
        (
            PhaseRates {
                transient_error: -1.0,
                ..good
            },
            FixturePhaseNumericField::TransientRoundoff,
            FixturePhaseNumericIssue::Negative,
        ),
        (
            PhaseRates {
                aggregate: f64::INFINITY,
                ..good
            },
            FixturePhaseNumericField::AggregateRate,
            FixturePhaseNumericIssue::NonFinite,
        ),
        (
            PhaseRates {
                aggregate_error: -1.0,
                ..good
            },
            FixturePhaseNumericField::AggregateRoundoff,
            FixturePhaseNumericIssue::Negative,
        ),
        (
            PhaseRates {
                aggregate_error: f64::from_bits(1),
                ..good
            },
            FixturePhaseNumericField::AggregateRoundoff,
            FixturePhaseNumericIssue::Subnormal,
        ),
    ] {
        reject_rates(
            candidate,
            inputs,
            FixturePhaseNumericError::Input { field, issue },
        );
    }
    Ok(())
}

/// Useful and baseline-only results retain different exact branch obligations.
/// # Errors
/// Preserve an unexpected typed-duration failure in an admitted fixture context.
/// # Panics
/// Detect cap excess, a fabricated useful zero or loss of baseline signed-zero bits.
#[test]
fn transformed_branch_faults_fail_raw_and_checked_rate_admission() -> TestResult {
    let inputs = ProposalInputs {
        baseline: -0.0,
        remaining: duration(1_000_000_000)?,
        transient_cap: 1.0 / 128.0,
        aggregate_cap: 1.0 / 64.0,
    };
    assert!(RawProposal { value: inputs }.validate().is_ok());
    let useful = PhaseRates {
        transient: 1.0 / 256.0,
        transient_error: 0.0,
        aggregate: 1.0 / 256.0,
        aggregate_error: 0.0,
        useful: true,
    };
    let baseline_only = PhaseRates {
        transient: 0.0,
        transient_error: 0.0,
        aggregate: -0.0,
        aggregate_error: 0.0,
        useful: false,
    };
    assert!(PhaseRates::admit(useful, inputs).is_ok());
    assert!(PhaseRates::admit(baseline_only, inputs).is_ok());
    let boundary = PhaseRates {
        transient: inputs.transient_cap,
        aggregate: inputs.transient_cap,
        ..useful
    };
    assert!(PhaseRates::admit(boundary, inputs).is_ok());
    for candidate in [
        PhaseRates {
            transient: 0.0,
            ..useful
        },
        PhaseRates {
            transient: inputs.transient_cap.next_up(),
            ..useful
        },
        PhaseRates {
            transient: -0.0,
            ..baseline_only
        },
        PhaseRates {
            transient_error: 1.0 / 1024.0,
            ..baseline_only
        },
        PhaseRates {
            aggregate: 0.0,
            ..baseline_only
        },
        PhaseRates {
            aggregate_error: 1.0 / 1024.0,
            ..baseline_only
        },
    ] {
        reject_rates(
            candidate,
            inputs,
            FixturePhaseNumericError::ValidationInvariant,
        );
    }
    reject_rates(
        PhaseRates {
            aggregate: inputs.aggregate_cap.next_up(),
            ..useful
        },
        inputs,
        FixturePhaseNumericError::AggregateLimit,
    );
    Ok(())
}

/// Raw settlement rules apply even when exact duration is zero.
/// # Errors
/// Preserve an unexpected checked debt or typed interval construction failure.
/// # Panics
/// Detect zero shortcuts bypassing raw validation or mutation of prior debt.
#[test]
fn settlement_raw_rules_reject_before_zero_duration_shortcuts() -> TestResult {
    let debt = product_ok(PhaseDebt::new(1.0 / 32.0, 1.0 / 4096.0))?;
    let original = (debt.point().to_bits(), debt.error().to_bits());
    let elapsed = duration(0)?;
    for (aggregate, baseline, field, issue) in [
        (
            f64::NAN,
            0.0,
            FixturePhaseNumericField::AggregateRate,
            FixturePhaseNumericIssue::NonFinite,
        ),
        (
            f64::from_bits(1),
            0.0,
            FixturePhaseNumericField::AggregateRate,
            FixturePhaseNumericIssue::Subnormal,
        ),
        (
            0.0,
            f64::INFINITY,
            FixturePhaseNumericField::BaselineRate,
            FixturePhaseNumericIssue::NonFinite,
        ),
        (
            0.0,
            -f64::from_bits(1),
            FixturePhaseNumericField::BaselineRate,
            FixturePhaseNumericIssue::Subnormal,
        ),
        (
            f64::NAN,
            f64::INFINITY,
            FixturePhaseNumericField::AggregateRate,
            FixturePhaseNumericIssue::NonFinite,
        ),
    ] {
        let raw = RawSettlement {
            value: SettlementInputs {
                aggregate,
                baseline,
                duration: elapsed,
            },
        };
        assert!(raw.validate().is_err());
        assert_eq!(
            debt.settled(aggregate, baseline, elapsed).err(),
            Some(FixturePhaseNumericError::Input { field, issue })
        );
        assert_eq!((debt.point().to_bits(), debt.error().to_bits()), original);
    }
    assert!(
        debt.settled(
            f64::MIN_POSITIVE,
            -f64::MIN_POSITIVE,
            duration(1_000_000_000)?
        )
        .is_ok()
    );
    Ok(())
}

/// Known zero delivery preserves its point; exhausted numerical bounds refuse.
/// # Errors
/// Preserve any unexpected checked owner, typed interval or healthy-neighbor failure.
/// # Panics
/// Detect lost inherited error, fabricated finite bounds or mutation on refusal.
#[test]
fn zero_delivery_and_bound_exhaustion_preserve_prior_debt() -> TestResult {
    let debt = product_ok(PhaseDebt::new(1.0 / 32.0, 1.0 / 4096.0))?;
    let zero = product_ok(debt.settled(5.0 / 1024.0, 1.0 / 1024.0, duration(0)?))?;
    assert_eq!(zero.point().to_bits(), debt.point().to_bits());
    assert!(zero.error() >= debt.error());
    let large = product_ok(PhaseDebt::new(1.0, f64::MAX))?;
    let original = (large.point().to_bits(), large.error().to_bits());
    assert_eq!(
        large.settled(0.0, 0.0, duration(0)?).err(),
        Some(FixturePhaseNumericError::Bound {
            step: FixturePhaseNumericStep::DebtBound,
            source: FixtureChartError::Bound {
                operation: FixtureChartBoundOperation::Add,
                issue: FixtureChartBoundIssue::NonFiniteResult
            }
        })
    );
    assert_eq!((large.point().to_bits(), large.error().to_bits()), original);
    let neighbor = product_ok(product_ok(PhaseDebt::new(1.0, f64::MAX / 4.0))?.settled(
        0.0,
        0.0,
        duration(0)?,
    ))?;
    assert_eq!(neighbor.point().to_bits(), 1.0_f64.to_bits());
    assert!(neighbor.error().is_finite() && neighbor.error() >= f64::MAX / 4.0);
    Ok(())
}

/// Exact P=33/32768 and H=1000000004/1000000000 distinguish denominator error.
/// RN quotient error is exactly 152851525/1152921509218532994427387904,
/// above the rounding-only upper 1.1180834859033424e-19. Both literals below
/// are independent exact-rational/separate-RN facts, not production outputs.
/// # Errors
/// Preserve an unexpected checked owner, active interval or quotient failure.
/// # Panics
/// Detect an altered RN point or omission of the exact-duration denominator error.
#[test]
fn noninteger_horizon_error_includes_denominator_conversion() -> TestResult {
    let rates = product_ok(product_ok(PhaseDebt::new(33.0 / 32768.0, 0.0))?.proposal(
        0.0,
        duration(1_000_000_004)?,
        1.0 / 128.0,
        1.0 / 64.0,
    ))?;
    assert_eq!(rates.transient_rate().to_bits(), 0x3f50_7fff_fee4_883b);
    let exact_error_upper = f64::from_bits(0x3c03_90a3_8532_45df);
    assert!(rates.transient_error() >= exact_error_upper);
    assert!(rates.has_useful_transient());
    Ok(())
}

/// Separately RN aggregate/base products precede the ordered debt subtraction/addition.
/// The independent exact debt is -125000001/128000000000. The selected point's
/// exact absolute error is 37039/9007199254740992000000000, whose least binary64
/// upper is fixed below. Collapsing aggregate-minus-baseline instead changes the
/// point by one ULP, to 0xbf5000000225c17e.
/// # Errors
/// Preserve an unexpected checked debt, typed duration or settlement rejection.
/// # Panics
/// Detect reassociated settlement arithmetic or a bound below its exact error.
#[test]
fn settlement_rounding_order_preserves_noninteger_duration() -> TestResult {
    let debt = product_ok(product_ok(PhaseDebt::new(1.0 / 1024.0, 0.0))?.settled(
        3.0 / 1024.0,
        1.0 / 1024.0,
        duration(1_000_000_004)?,
    ))?;
    assert_eq!(debt.point().to_bits(), 0xbf50_0000_0225_c17d);
    assert!(debt.error() >= f64::from_bits(0x3bb3_6b4a_5973_1681));
    Ok(())
}

/// Numeric proposal inputs own cap admission independently of the public profile.
/// # Errors
/// Preserve unexpected checked debt, typed duration or legal cap-neighbor failures.
/// # Panics
/// Detect invalid caps, baseline magnitude loss or refusal of exact cap equality.
#[test]
fn proposal_input_rules_preserve_caps_and_baseline_boundaries() -> TestResult {
    let debt = product_ok(PhaseDebt::new(1.0 / 1024.0, 0.0))?;
    let remaining = duration(1_000_000_000)?;
    for (bad, issue) in [
        (0.0, FixturePhaseNumericIssue::NonPositive),
        (-0.0, FixturePhaseNumericIssue::NonPositive),
        (-1.0, FixturePhaseNumericIssue::NonPositive),
        (f64::from_bits(1), FixturePhaseNumericIssue::Subnormal),
        (f64::NAN, FixturePhaseNumericIssue::NonFinite),
        (f64::INFINITY, FixturePhaseNumericIssue::NonFinite),
    ] {
        for (transient_cap, aggregate_cap, field) in [
            (bad, 1.0 / 64.0, FixturePhaseNumericField::TransientCap),
            (1.0 / 128.0, bad, FixturePhaseNumericField::AggregateCap),
        ] {
            let inputs = ProposalInputs {
                baseline: 0.0,
                remaining,
                transient_cap,
                aggregate_cap,
            };
            assert!(RawProposal { value: inputs }.validate().is_err());
            assert_eq!(
                debt.proposal(0.0, remaining, transient_cap, aggregate_cap)
                    .err(),
                Some(FixturePhaseNumericError::Input { field, issue })
            );
        }
    }
    let aggregate_cap = 1.0_f64 / 64.0;
    for baseline in [aggregate_cap.next_up(), -aggregate_cap.next_up()] {
        let inputs = ProposalInputs {
            baseline,
            remaining,
            transient_cap: 1.0 / 128.0,
            aggregate_cap,
        };
        assert!(RawProposal { value: inputs }.validate().is_err());
        assert_eq!(
            debt.proposal(baseline, remaining, inputs.transient_cap, aggregate_cap)
                .err(),
            Some(FixturePhaseNumericError::AggregateLimit)
        );
    }
    for baseline in [aggregate_cap, -aggregate_cap] {
        let inputs = ProposalInputs {
            baseline,
            remaining,
            transient_cap: 1.0 / 128.0,
            aggregate_cap,
        };
        assert!(RawProposal { value: inputs }.validate().is_ok());
        for (point, error) in [(0.0, 0.0), (1.0 / 1024.0, 1.0 / 1024.0)] {
            let rates = product_ok(product_ok(PhaseDebt::new(point, error))?.proposal(
                baseline,
                remaining,
                inputs.transient_cap,
                aggregate_cap,
            ))?;
            assert!(!rates.has_useful_transient());
            assert_eq!(rates.transient_rate().to_bits(), 0);
            assert_eq!(rates.aggregate_rate().to_bits(), baseline.to_bits());
        }
    }
    Ok(())
}

/// An unclamped explicit debt interval [6,10] around point8 needs error at least2.
/// These finite parameters exercise the numeric owner, not a shipping phase cap.
/// # Errors
/// Preserve an unexpected checked debt, active interval or proposal rejection.
/// # Panics
/// Detect discarded inherited uncertainty or a fabricated constant small bound.
#[test]
fn proposal_error_encloses_inherited_debt_interval() -> TestResult {
    let debt = product_ok(PhaseDebt::new(8.0, 2.0))?;
    let rates = product_ok(debt.proposal(0.0, duration(1_000_000_000)?, 16.0, 16.0))?;
    assert_eq!(rates.transient_rate().to_bits(), 8.0_f64.to_bits());
    assert_eq!(rates.aggregate_rate().to_bits(), 8.0_f64.to_bits());
    assert!(rates.transient_error() >= 2.0);
    assert!(rates.aggregate_error() >= 2.0);
    assert!(rates.has_useful_transient());
    assert_eq!(debt.point().to_bits(), 8.0_f64.to_bits());
    assert_eq!(debt.error().to_bits(), 2.0_f64.to_bits());
    Ok(())
}

/// Exact zero-factor products need zero error, separately from nonzero products.
/// For rate3/1024 and exact duration1000000004/1000000000, the separately RN
/// product has absolute error230251/2251799813685248000000000; its least
/// binary64 upper is fixed below, independently of the production bound.
/// # Errors
/// Preserve an unexpected duration-conversion or valid product-bound failure.
/// # Panics
/// Detect spurious zero-product error or omission of nonzero conversion/product error.
#[test]
fn delivery_error_preserves_zero_factor_identity() -> TestResult {
    assert_eq!(product_ok(delivery_error(0.0, 1.0, 0.0))?.to_bits(), 0);
    assert_eq!(product_ok(delivery_error(-0.0, 1.0, 0.0))?.to_bits(), 0);
    assert_eq!(
        product_ok(delivery_error(3.0 / 1024.0, 0.0, 0.0))?.to_bits(),
        0
    );
    let (seconds, time_error) = product_ok(super::duration_seconds(
        duration(1_000_000_004)?,
        FixturePhaseNumericStep::ElapsedSeconds,
    ))?;
    assert_eq!(seconds.to_bits(), 0x3ff0_0000_0112_e0bf);
    assert_eq!((3.0 / 1024.0 * seconds).to_bits(), 0x3f68_0000_019c_511e);
    let error = product_ok(delivery_error(3.0 / 1024.0, seconds, time_error))?;
    assert!(error >= f64::from_bits(0x3bfe_2df1_079d_35e4));
    Ok(())
}
