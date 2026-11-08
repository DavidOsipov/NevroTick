// SPDX-License-Identifier: GPL-2.0-only
//! Independent exact-real and numerical-bound controls for conditional frequency.

use std::error::Error;

use garde::Validate;
use time_domain::{DomainError, MonotonicDurationNs, MonotonicNs};

use super::{
    FixtureFrequencyNumericError, FixtureFrequencyNumericField, FixtureFrequencyNumericIssue,
    FixtureFrequencyNumericStep, FrequencyAnchor, FrequencyPrediction, FrequencyPrior, Interval,
    RawAnchor, RawPrediction, RawPrior, Sample, gain_interval,
};

/// Preserve unexpected typed failures instead of converting them into test success.
type TestResult = Result<(), Box<dyn Error>>;

/// Assert a known-valid result before preserving its original typed outcome.
///
/// Callers supply the closed bounded numeric or time-domain error types; this
/// helper adds no fallback value, error conversion or changed success payload.
/// # Errors
/// Return the original result without erasing or replacing its error type.
/// # Panics
/// Reject refusal of the independently supported literal product trajectory.
#[track_caller]
fn product_ok<T, E: std::fmt::Debug>(value: Result<T, E>) -> Result<T, E> {
    assert!(
        value.is_ok(),
        "valid frequency numeric fixture was refused: {:?}",
        value.as_ref().err()
    );
    value
}

/// Obtain elapsed time from the actual monotonic owner, without raw duration fields.
/// # Errors
/// Preserve a genuine time-domain failure.
fn elapsed(nanoseconds: u64) -> Result<MonotonicDurationNs, DomainError> {
    MonotonicNs::new(nanoseconds).duration_since(MonotonicNs::new(0))
}

/// First intake retains the fresh mean error and exact correlated-zero residual.
/// # Errors
/// Preserve an unexpected checked-construction rejection.
/// # Panics
/// Detect inventing a zero-mean prior or charging the same error twice to `f-f`.
#[test]
fn initialization_preserves_fresh_error_and_correlated_zero() -> TestResult {
    let prior = product_ok(FrequencyPrior::initialize(
        -1.0 / 512.0,
        1.0 / 4096.0,
        1.0 / 1024.0,
    ))?;
    assert_eq!(prior.mean(), -1.0 / 512.0);
    assert_eq!(prior.mean_error(), 1.0 / 4096.0);
    assert_eq!(prior.skew(), 1.0 / 1024.0);
    assert_eq!(prior.skew_error(), 0.0);
    assert_eq!(prior.residual().to_bits(), 0);
    assert_eq!(prior.residual_error(), 0.0);
    Ok(())
}

/// A literal integer-square bracket independently encloses the irrational skew.
/// # Errors
/// Preserve an unexpected mean/skew/enclosure refusal.
/// # Panics
/// Detect a wrong gain, missing disagreement, wrong residual or zero error bound.
#[test]
fn equal_skew_fusion_matches_dyadic_and_integer_square_truth() -> TestResult {
    let prior = product_ok(FrequencyPrior::initialize(0.0, 0.0, 1.0 / 1024.0))?;
    let updated = product_ok(prior.assimilate(1.0 / 512.0, 0.0, 1.0 / 1024.0))?;
    assert_eq!(updated.mean().to_bits(), 0x3f58_0000_0000_0000);
    assert_eq!(updated.residual(), 1.0 / 2048.0);
    // L/2^52 < sqrt(3) < (L+1)/2^52 by these exact integer squares.
    let lower = 7_800_463_371_553_962_u128;
    let radicand = 3_u128 << 104;
    assert!(lower * lower < radicand);
    assert!((lower + 1) * (lower + 1) > radicand);
    // Consequently (2+sqrt(3))/2048 lies strictly between these literals.
    let skew_lower = f64::from_bits(0x3f5d_db3d_742c_2655);
    let skew_upper = f64::from_bits(0x3f5d_db3d_742c_2656);
    assert_eq!(updated.skew().to_bits(), skew_lower.to_bits());
    assert!(updated.skew() - updated.skew_error() <= skew_lower);
    assert!(updated.skew() + updated.skew_error() >= skew_upper);
    assert!(updated.mean_error() > 0.0);
    assert!(updated.residual_error() > 0.0);
    assert_eq!(prior.mean(), 0.0);
    Ok(())
}

/// Same-chart translation and sign reversal preserve the real gain/disagreement.
/// # Errors
/// Preserve an unexpected checked update refusal.
/// # Panics
/// Detect resetting a nonzero prior, reversing the wrong sign or moving skew.
#[test]
fn nonzero_prior_translation_and_sign_are_preserved() -> TestResult {
    let skew = 1.0 / 1024.0;
    let positive = product_ok(
        product_ok(FrequencyPrior::initialize(0.0, 0.0, skew))?.assimilate(1.0 / 512.0, 0.0, skew),
    )?;
    let shifted = product_ok(
        product_ok(FrequencyPrior::initialize(7.0 / 1024.0, 0.0, skew))?.assimilate(
            9.0 / 1024.0,
            0.0,
            skew,
        ),
    )?;
    let negative = product_ok(
        product_ok(FrequencyPrior::initialize(0.0, 0.0, skew))?.assimilate(-1.0 / 512.0, 0.0, skew),
    )?;
    let retained = product_ok(
        product_ok(FrequencyPrior::initialize(1.0 / 2048.0, 0.0, skew))?.assimilate(
            1.0 / 1024.0,
            0.0,
            skew,
        ),
    )?;
    assert_eq!(shifted.mean(), 17.0 / 2048.0);
    assert_eq!(shifted.residual(), positive.residual());
    assert_eq!(shifted.skew(), positive.skew());
    assert_eq!(negative.mean(), -positive.mean());
    assert_eq!(negative.residual(), -positive.residual());
    assert_eq!(negative.skew(), positive.skew());
    assert_eq!(retained.mean(), 7.0 / 8192.0);
    assert_eq!(retained.residual(), 1.0 / 8192.0);
    Ok(())
}

/// A weak zero-innovation fit increases skew without replacing the prior mean.
/// # Errors
/// Preserve an unexpected checked refusal.
/// # Panics
/// Detect treating skew as an inverse linear weight or replacing the prior.
#[test]
fn weaker_fit_keeps_zero_innovation_and_expected_skew() -> TestResult {
    let prior = product_ok(FrequencyPrior::initialize(1.0 / 512.0, 0.0, 1.0 / 1024.0))?;
    let next = product_ok(prior.assimilate(1.0 / 512.0, 0.0, 3.0 / 1024.0))?;
    assert_eq!(next.mean(), 1.0 / 512.0);
    assert_eq!(next.residual(), 0.0);
    assert_eq!(next.skew(), 3.0 / 2048.0);
    Ok(())
}

/// Scaling both finite skews avoids otherwise overflowing/underflowing squares.
/// # Errors
/// Preserve unsupported weight arithmetic instead of hiding it as a skip.
/// # Panics
/// Detect raw absolute-skew quadratics or a changed normalized point tree.
#[test]
fn large_and_tiny_equal_skews_use_normalized_weights() -> TestResult {
    for bits in [0x6570_0000_0000_0000_u64, 0x1a70_0000_0000_0000] {
        // Exact normal powers 2^600 and 2^-600; neither value is a default.
        let skew = f64::from_bits(bits);
        let prior = product_ok(FrequencyPrior::initialize(0.0, 0.0, skew))?;
        let next = product_ok(prior.assimilate(1.0 / 512.0, 0.0, skew))?;
        assert_eq!(next.mean(), 3.0 / 2048.0);
        assert_eq!(next.residual(), 1.0 / 2048.0);
        assert!(next.skew().is_normal());
        assert!(next.skew_error().is_finite());
    }
    Ok(())
}

/// Both-zero refusal and the two single-zero mathematical neighbors are distinct.
/// # Errors
/// Preserve an unexpected checked neighboring refusal.
/// # Panics
/// Detect importing NaN/clamp behavior or refusing valid gain-zero/gain-one cases.
#[test]
fn zero_pair_refuses_but_single_zero_cases_are_supported() -> TestResult {
    let zero = product_ok(FrequencyPrior::initialize(1.0 / 1024.0, 0.0, -0.0))?;
    assert_eq!(
        zero.assimilate(1.0 / 512.0, 0.0, 0.0).err(),
        Some(FixtureFrequencyNumericError::IndeterminateWeights)
    );
    let gain_zero = product_ok(zero.assimilate(1.0 / 512.0, 0.0, 1.0 / 1024.0))?;
    assert_eq!(gain_zero.mean(), 1.0 / 1024.0);
    assert_eq!(gain_zero.residual(), 1.0 / 1024.0);
    assert_eq!(gain_zero.skew(), 0.0);
    let prior = product_ok(FrequencyPrior::initialize(1.0 / 1024.0, 0.0, 1.0 / 1024.0))?;
    let gain_one = product_ok(prior.assimilate(1.0 / 512.0, 0.0, 0.0))?;
    assert_eq!(gain_one.mean(), 1.0 / 512.0);
    assert_eq!(gain_one.residual(), 0.0);
    assert_eq!(gain_one.skew(), 0.0);
    assert_eq!(gain_one.skew_error(), 0.0);
    assert_eq!(zero.mean(), 1.0 / 1024.0);
    Ok(())
}

/// Independent interval endpoints require inherited and fresh mean errors.
/// # Errors
/// Preserve a genuine checked update/enclosure failure.
/// # Panics
/// Detect discarding inherited input error or treating model skew as rate SD.
#[test]
fn inherited_mean_and_skew_errors_survive_successive_updates() -> TestResult {
    let old_error = 1.0 / 128.0;
    let new_error = 1.0 / 256.0;
    let prior = product_ok(FrequencyPrior::initialize(0.0, old_error, 1.0 / 1024.0))?;
    let next = product_ok(prior.assimilate(0.0, new_error, 1.0 / 1024.0))?;
    // For exact g=3/4, the mean interval radius is (old/4)+(3*new/4)=5/1024.
    assert!(next.mean_error() >= 5.0 / 1024.0);
    // The extreme innovation old+new adds sqrt(3)*(old+new)/4 to exact skew.
    assert!(next.skew_error() >= (old_error + new_error) / 4.0);
    // The residual is (f-p)/4, so its exact input-error radius is 3/1024.
    assert!(next.residual_error() >= 3.0 / 1024.0);
    let again = product_ok(next.assimilate(0.0, 0.0, 1.0 / 1024.0))?;
    assert!(again.mean_error() > 0.0);
    assert!(again.skew_error() > 0.0);
    assert_eq!(prior.mean_error(), old_error);
    Ok(())
}

/// A reachable inherited skew interval may make a zero-new-skew weight singular.
/// # Errors
/// Preserve healthy setup failures for the runner.
/// # Panics
/// Detect forcing gain one despite an inherited interval containing zero.
#[test]
fn singular_weight_enclosure_retains_the_old_model() -> TestResult {
    let old = product_ok(
        product_ok(FrequencyPrior::initialize(0.0, 1.0, 1.0 / 1024.0))?.assimilate(
            0.0,
            1.0,
            1.0 / 1024.0,
        ),
    )?;
    assert!(old.skew_error() > old.skew());
    assert_eq!(
        old.assimilate(0.0, 0.0, 0.0).err(),
        Some(FixtureFrequencyNumericError::SingularWeights)
    );
    assert_eq!(old.mean(), 0.0);
    assert_eq!(old.skew(), 1.0 / 1024.0);
    Ok(())
}

/// Full C02 trajectory uses its disagreement-increased skew in later prediction.
/// # Errors
/// Preserve unexpected numeric or duration refusals.
/// # Panics
/// Detect using the standalone C07 skew or integrating a new slope from stale time.
#[test]
fn actual_updated_model_growth_includes_disagreement() -> TestResult {
    let prior = product_ok(
        product_ok(FrequencyPrior::initialize(0.0, 0.0, 1.0 / 1024.0))?.assimilate(
            1.0 / 512.0,
            0.0,
            1.0 / 1024.0,
        ),
    )?;
    let anchor = product_ok(FrequencyAnchor::new(1.0 / 32.0, 0.0))?;
    let prediction = product_ok(prior.project(&anchor, elapsed(8_000_000_000)?, 1.0 / 1024.0))?;
    assert_eq!(prediction.phase(), 11.0 / 256.0);
    // Exact growth is (5+sqrt(3))/256, strictly above standalone 5/256.
    // The integer sqrt(3) bracket above places (5+sqrt(3))/256 strictly
    // between these adjacent binary64 literals, independent of this point tree.
    let lower = f64::from_bits(0x3f9a_ed9e_ba16_132a);
    let upper = f64::from_bits(0x3f9a_ed9e_ba16_132b);
    assert!(prediction.growth() > 5.0 / 256.0);
    assert!(prediction.growth_upper() >= lower);
    assert!(prediction.growth_upper() >= upper);
    assert!(prediction.phase_error() >= anchor.error());
    Ok(())
}

/// Projection encloses exact rational elapsed seconds instead of only RN seconds.
/// # Errors
/// Preserve duration/conversion or numeric failures.
/// # Panics
/// Detect dropping the noninteger denominator residual or inherited phase/rate errors.
#[test]
fn noninteger_elapsed_and_inherited_projection_errors_are_enclosed() -> TestResult {
    let prior = product_ok(FrequencyPrior::initialize(
        1.0 / 1024.0,
        1.0 / 4096.0,
        1.0 / 1024.0,
    ))?;
    let anchor = product_ok(FrequencyAnchor::new(0.0, 1.0 / 8192.0))?;
    let prediction = product_ok(prior.project(&anchor, elapsed(1_000_000_004)?, 1.0 / 4096.0))?;
    assert_eq!(prediction.phase().to_bits(), 0x3f50_0000_0112_e0bf);
    // Independent real mean is 250000001/256000000000; inherited radius alone
    // exceeds 1/8192+1/4096 because exact elapsed is strictly greater than one.
    assert!(prediction.phase_error() > 3.0 / 8192.0);
    assert!(prediction.growth_upper() >= prediction.growth());
    let zero = product_ok(prior.project(&anchor, elapsed(0)?, 0.0))?;
    assert_eq!(zero.phase(), anchor.phase());
    assert_eq!(zero.phase_error(), anchor.error());
    assert_eq!(zero.growth(), 0.0);
    assert_eq!(zero.growth_upper(), 0.0);
    Ok(())
}

/// Exact-total anchor rebase is distinct from baseline-relative phase debt.
/// # Errors
/// Preserve an unexpected phase/bound refusal.
/// # Panics
/// Detect adding baseline back, using a request, or dropping inherited error.
#[test]
fn actual_total_anchor_rebase_preserves_both_error_channels() -> TestResult {
    let anchor = product_ok(FrequencyAnchor::new(1.0 / 32.0, 1.0 / 8192.0))?;
    let rebased = product_ok(anchor.rebase(5.0 / 512.0, 1.0 / 16384.0))?;
    assert_eq!(rebased.phase(), 11.0 / 512.0);
    assert_ne!(rebased.phase(), 3.0 / 128.0);
    assert!(rebased.error() >= 3.0 / 16384.0);
    assert_eq!(anchor.phase(), 1.0 / 32.0);
    let unchanged = product_ok(anchor.rebase(-0.0, 0.0))?;
    assert_eq!(unchanged.phase(), anchor.phase());
    assert_eq!(unchanged.error(), anchor.error());

    // 1 - 2^-54 is exactly halfway between 1 and its lower binary64 neighbor.
    // Ties-to-even returns 1, whose exact-real error is the literal 2^-54.
    let exact = product_ok(FrequencyAnchor::new(1.0, 0.0))?;
    let half_lower_ulp = f64::from_bits(0x3c90_0000_0000_0000);
    let result = exact.rebase(half_lower_ulp, 0.0);
    assert!(
        result.is_ok(),
        "the exact midpoint rebase must be supported"
    );
    let rounded = result?;
    assert_eq!(rounded.phase().to_bits(), 0x3ff0_0000_0000_0000);
    assert!(rounded.error() >= half_lower_ulp);
    // A zero nominal effect with nonzero inherited effect error is not identity.
    let result = exact.rebase(0.0, half_lower_ulp);
    assert!(
        result.is_ok(),
        "the uncertain zero effect must be supported"
    );
    let uncertain = result?;
    assert_eq!(uncertain.phase().to_bits(), 0x3ff0_0000_0000_0000);
    assert!(uncertain.error() >= half_lower_ulp);
    assert_eq!(exact.phase(), 1.0);
    assert_eq!(exact.error(), 0.0);
    Ok(())
}

/// Unsupported raw input reaches checked constructors and every exposed consumer.
/// # Errors
/// Preserve a healthy control refusal.
/// # Panics
/// Detect bypassing sample/maximum/effect field admission or accepting negatives.
#[test]
fn raw_inputs_have_typed_rejections_and_healthy_neighbors() -> TestResult {
    for (rate, error, skew, field, issue) in [
        (
            f64::NAN,
            0.0,
            0.0,
            FixtureFrequencyNumericField::Mean,
            FixtureFrequencyNumericIssue::NonFinite,
        ),
        (
            f64::from_bits(1),
            0.0,
            0.0,
            FixtureFrequencyNumericField::Mean,
            FixtureFrequencyNumericIssue::Subnormal,
        ),
        (
            0.0,
            -1.0,
            0.0,
            FixtureFrequencyNumericField::MeanError,
            FixtureFrequencyNumericIssue::Negative,
        ),
        (
            0.0,
            0.0,
            -1.0,
            FixtureFrequencyNumericField::Skew,
            FixtureFrequencyNumericIssue::Negative,
        ),
    ] {
        assert_eq!(
            FrequencyPrior::initialize(rate, error, skew).err(),
            Some(FixtureFrequencyNumericError::Input { field, issue })
        );
        assert!(Sample::new(rate, error, skew).is_err());
    }
    let prior = product_ok(FrequencyPrior::initialize(-0.0, 0.0, 0.0))?;
    let anchor = product_ok(FrequencyAnchor::new(-0.0, 0.0))?;
    assert!(prior.project(&anchor, elapsed(1)?, -1.0).is_err());
    assert!(anchor.rebase(f64::INFINITY, 0.0).is_err());
    assert!(FrequencyAnchor::new(0.0, -1.0).is_err());
    assert_eq!(prior.mean().to_bits(), (-0.0_f64).to_bits());
    Ok(())
}

/// Finite operands may fail arithmetic/enclosure; failures never overwrite owners.
/// # Errors
/// Preserve useful setup failures.
/// # Panics
/// Detect finite overflow being silently zeroed or replaced with a fresh prior.
#[test]
fn finite_overflow_and_duration_limits_preserve_prior_and_anchor() -> TestResult {
    let prior = product_ok(FrequencyPrior::initialize(-f64::MAX, 0.0, 1.0))?;
    assert!(matches!(
        prior.assimilate(f64::MAX, 0.0, 1.0),
        Err(FixtureFrequencyNumericError::Arithmetic {
            step: FixtureFrequencyNumericStep::Innovation,
            ..
        })
    ));
    assert_eq!(prior.mean(), -f64::MAX);
    let anchor = product_ok(FrequencyAnchor::new(f64::MAX, 0.0))?;
    assert!(anchor.rebase(-f64::MAX, 0.0).is_err());
    assert_eq!(anchor.phase(), f64::MAX);
    let healthy = product_ok(FrequencyPrior::initialize(0.0, 0.0, 0.0))?;
    let origin = product_ok(FrequencyAnchor::new(0.0, 0.0))?;
    assert!(healthy.project(&origin, elapsed(1_u64 << 53)?, 0.0).is_ok());
    assert!(matches!(
        healthy.project(&origin, elapsed((1_u64 << 53) + 1)?, 0.0),
        Err(FixtureFrequencyNumericError::Duration { .. })
    ));
    Ok(())
}

/// Transformed-field validators cover fields a healthy point path cannot forge.
/// # Errors
/// Preserve healthy construction failures.
/// # Panics
/// Detect missing output admission or a whole-object Garde delegation omission.
#[test]
fn transformed_outputs_delegate_complete_field_rules() -> TestResult {
    let prior = product_ok(FrequencyPrior::initialize(0.0, 0.0, 1.0))?;
    let invalid = FrequencyPrior {
        skew_error: -1.0,
        ..prior
    };
    assert!(RawPrior { value: invalid }.validate().is_err());
    assert_eq!(
        invalid.admit().err(),
        Some(FixtureFrequencyNumericError::Input {
            field: FixtureFrequencyNumericField::SkewError,
            issue: FixtureFrequencyNumericIssue::Negative
        })
    );
    let invalid_anchor = FrequencyAnchor {
        phase: 0.0,
        error: f64::NAN,
    };
    assert!(
        RawAnchor {
            value: invalid_anchor
        }
        .validate()
        .is_err()
    );
    assert!(invalid_anchor.admit().is_err());
    let invalid_prediction = FrequencyPrediction {
        phase: 0.0,
        phase_error: 0.0,
        growth: 1.0,
        growth_upper: 0.0,
    };
    assert!(
        RawPrediction {
            value: invalid_prediction
        }
        .validate()
        .is_err()
    );
    assert_eq!(
        invalid_prediction.admit().err(),
        Some(FixtureFrequencyNumericError::ValidationInvariant)
    );
    Ok(())
}

/// Signed endpoint operations use exact real interval order and zero identities.
/// # Errors
/// Preserve endpoint refusals instead of discarding them.
/// # Panics
/// Detect reversing signed endpoints, missing zero products or a negative root.
#[test]
fn closed_interval_sign_and_endpoint_neighbors() -> TestResult {
    let step = FixtureFrequencyNumericStep::Prediction;
    let signed = Interval {
        low: -2.0,
        high: 3.0,
    };
    let negative = Interval {
        low: -4.0,
        high: -1.0,
    };
    let product = product_ok(signed.multiply(negative, step))?;
    assert!(product.low <= -12.0 && product.high >= 8.0);
    let abs = negative.absolute();
    assert_eq!(abs.low, 1.0);
    assert_eq!(abs.high, 4.0);
    assert!(signed.absolute().high >= 3.0);
    assert!(
        signed
            .divide(
                Interval {
                    low: 0.0,
                    high: 1.0
                },
                step
            )
            .is_err()
    );
    assert!(signed.square_root(step).is_err());
    assert!(Interval::point(f64::MAX).add(Interval::ONE, step).is_err());
    assert!(product_ok(Interval::ZERO.multiply(signed, step))?.is_zero());
    assert!(product_ok(Interval::point(2.0).subtract(Interval::point(2.0), step))?.is_zero());
    Ok(())
}

/// Checked numeric containers and bounded errors do not reveal source numbers.
/// # Errors
/// Preserve healthy model setup failures.
/// # Panics
/// Detect accidental derived numeric Debug or missing typed error delegation.
#[test]
fn diagnostics_redact_numeric_state_and_preserve_error_source() -> TestResult {
    let prior = product_ok(FrequencyPrior::initialize(1.0 / 1024.0, 0.0, 1.0 / 1024.0))?;
    let anchor = product_ok(FrequencyAnchor::new(1.0 / 32.0, 0.0))?;
    let prediction = product_ok(prior.project(&anchor, elapsed(1_000_000_000)?, 0.0))?;
    assert_eq!(format!("{prior:?}"), "FrequencyPrior { checked: true, .. }");
    assert_eq!(
        format!("{anchor:#?}"),
        "FrequencyAnchor { checked: true, .. }"
    );
    assert_eq!(
        format!("{prediction:?}"),
        "FrequencyPrediction { checked: true, .. }"
    );
    let expected_error = product_ok(FrequencyPrior::initialize(-f64::MAX, 0.0, 1.0))?
        .assimilate(f64::MAX, 0.0, 1.0)
        .err();
    assert!(
        expected_error.is_some(),
        "the independent overflow fixture must return its arithmetic error"
    );
    let error = expected_error.ok_or("missing arithmetic rejection")?;
    assert!(error.source().is_some());
    assert!(error.to_string().starts_with("frequency numeric"));
    assert!(!format!("{error:?}").contains("179769"));
    Ok(())
}

/// Zero-old-skew gain distinguishes uncertain zero support from positive support.
///
/// Each literal interval is finite, ordered and nonnegative. Scale one is also
/// the maximum endpoint in both non-singleton cases; these controls do not
/// depend on allowing a scale larger than the represented skews. The helper
/// admits inherited intervals, independently of today's exact-fresh caller.
/// # Errors
/// Preserve an unexpected finite enclosure refusal on the supported neighbors.
/// # Panics
/// Detect accepting a potentially singular gain or refusing exact gain zero.
#[test]
fn gain_interval_zero_support_preserves_singular_and_exact_cases() -> TestResult {
    for zero in [0.0, -0.0] {
        // The fresh interval includes zero: at (old,fresh)=(0,0), gain is 0/0.
        assert_eq!(
            gain_interval(
                Interval::point(zero),
                Interval {
                    low: 0.0,
                    high: 1.0
                },
                Interval::ONE,
            )
            .err(),
            Some(FixtureFrequencyNumericError::SingularWeights)
        );
        // Every member is strictly positive, so 3*0^2/(3*0^2+v^2)=0 exactly.
        // Squaring the lower endpoint in binary64 would underflow, but this
        // exact one-zero identity does not need a rounded quadratic at all.
        for fresh in [
            Interval {
                low: f64::MIN_POSITIVE,
                high: 1.0,
            },
            Interval::ONE,
        ] {
            let result = gain_interval(Interval::point(zero), fresh, Interval::ONE);
            assert!(
                result.is_ok(),
                "positive fresh support must preserve exact gain zero"
            );
            let gain = result?;
            assert_eq!(gain.low.to_bits(), 0);
            assert_eq!(gain.high.to_bits(), 0);
        }
        let result = gain_interval(Interval::ONE, Interval::point(zero), Interval::ONE);
        assert!(
            result.is_ok(),
            "positive old support must preserve exact gain one"
        );
        let one = result?;
        assert_eq!(one.low, 1.0);
        assert_eq!(one.high, 1.0);
    }
    Ok(())
}

/// Around encloses exact dyadic endpoints and preserves either zero's bits.
/// # Errors
/// Preserve a refusal of independently supported phase/error literals.
/// # Panics
/// Detect losing a true endpoint or changing the declared outward successor.
#[test]
fn around_intervals_enclose_independent_dyadic_endpoints() -> TestResult {
    let step = FixtureFrequencyNumericStep::Prediction;
    for (value, exact_low, exact_high, low_bits, high_bits) in [
        (0.75, 0.5, 1.0, 0x3fdf_ffff_ffff_ffff, 0x3ff0_0000_0000_0001),
        (
            -0.75,
            -1.0,
            -0.5,
            0xbff0_0000_0000_0001,
            0xbfdf_ffff_ffff_ffff,
        ),
    ] {
        let result = Interval::around(value, 0.25, step);
        assert!(
            result.is_ok(),
            "the dyadic around interval must be supported"
        );
        let interval = result?;
        assert!(interval.low <= exact_low);
        assert!(interval.high >= exact_high);
        assert_eq!(interval.low.to_bits(), low_bits);
        assert_eq!(interval.high.to_bits(), high_bits);
    }
    for (value, error) in [(0.75, 0.0), (-0.0, 0.0), (-0.0, -0.0)] {
        let result = Interval::around(value, error, step);
        assert!(
            result.is_ok(),
            "zero error must preserve the exact stored point"
        );
        let interval = result?;
        assert_eq!(interval.low.to_bits(), value.to_bits());
        assert_eq!(interval.high.to_bits(), value.to_bits());
    }
    Ok(())
}

/// Asymmetric cross-zero ranges retain the dominant absolute endpoint.
/// # Errors
/// Preserve a refusal of supported exact dyadic range inputs.
/// # Panics
/// Detect dropping the negative-dominant magnitude or reversing its neighbor.
#[test]
fn absolute_value_preserves_dominant_cross_zero_magnitude() -> TestResult {
    let step = FixtureFrequencyNumericStep::Prediction;
    for (value, low_bits, high_bits) in [
        (-0.75, 0xbffc_0000_0000_0001, 0x3fd0_0000_0000_0001),
        (0.75, 0xbfd0_0000_0000_0001, 0x3ffc_0000_0000_0001),
    ] {
        // The exact intervals are [-7/4,1/4] and [-1/4,7/4].
        let result = Interval::around(value, 1.0, step);
        assert!(
            result.is_ok(),
            "the cross-zero dyadic interval must be supported"
        );
        let signed = result?;
        assert_eq!(signed.low.to_bits(), low_bits);
        assert_eq!(signed.high.to_bits(), high_bits);
        let absolute = signed.absolute();
        assert_eq!(absolute.low.to_bits(), 0);
        assert_eq!(absolute.high.to_bits(), 0x3ffc_0000_0000_0001);
        assert!(absolute.high >= 1.75);
    }
    Ok(())
}

/// A zero endpoint remains exact without making its whole interval zero.
/// # Errors
/// Preserve a refusal of supported around or multiplication inputs.
/// # Panics
/// Detect widening an exact zero product or erasing the nonzero upper endpoint.
#[test]
fn multiplication_preserves_exact_zero_endpoints() -> TestResult {
    let step = FixtureFrequencyNumericStep::Prediction;
    let result = Interval::around(0.75, 1.0, step);
    assert!(
        result.is_ok(),
        "the positive clipped interval must be supported"
    );
    let signed = result?;
    assert_eq!(signed.low.to_bits(), 0xbfd0_0000_0000_0001);
    assert_eq!(signed.high.to_bits(), 0x3ffc_0000_0000_0001);
    let positive = signed.nonnegative();
    assert_eq!(positive.low.to_bits(), 0);
    assert_eq!(positive.high.to_bits(), 0x3ffc_0000_0000_0001);
    assert!(!positive.is_zero());
    for (left, right) in [(positive, Interval::ONE), (Interval::ONE, positive)] {
        let result = left.multiply(right, step);
        assert!(
            result.is_ok(),
            "the zero-endpoint product must be supported"
        );
        let product = result?;
        assert_eq!(product.low.to_bits(), 0);
        assert_eq!(product.high.to_bits(), 0x3ffc_0000_0000_0002);
    }
    Ok(())
}

/// Whole-zero identities preserve canonical bits beside nonzero normal products.
/// # Errors
/// Preserve a refusal of supported finite interval construction or products.
/// # Panics
/// Detect noncanonical zero results or erasing the adjacent nonzero product.
#[test]
fn whole_zero_products_keep_canonical_bits_and_nonzero_neighbors() -> TestResult {
    let step = FixtureFrequencyNumericStep::Prediction;
    let result = Interval::around(0.75, 1.0, step);
    assert!(
        result.is_ok(),
        "the mixed-sign zero-product peer must be supported"
    );
    let signed = result?;
    for zero in [
        Interval::point(0.0),
        Interval::point(-0.0),
        Interval {
            low: -0.0,
            high: 0.0,
        },
        Interval {
            low: 0.0,
            high: -0.0,
        },
    ] {
        for peer in [
            signed,
            Interval::point(f64::MAX),
            Interval::point(-f64::MAX),
        ] {
            for (left, right) in [(zero, peer), (peer, zero)] {
                let result = left.multiply(right, step);
                assert!(result.is_ok(), "the whole-zero product must be supported");
                let product = result?;
                assert_eq!(product.low.to_bits(), 0);
                assert_eq!(product.high.to_bits(), 0);
            }
        }
    }
    let result = Interval::point(f64::MIN_POSITIVE).multiply(Interval::ONE, step);
    assert!(
        result.is_ok(),
        "the minimum-normal nonzero product must be supported"
    );
    let nonzero = result?;
    assert!(nonzero.low > 0.0);
    assert!(nonzero.high >= f64::MIN_POSITIVE);
    assert!(!nonzero.is_zero());
    Ok(())
}
