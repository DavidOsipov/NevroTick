// SPDX-License-Identifier: GPL-2.0-only
//! S154 chart raw-candidate controls; no checked history is forged.
//!
//! Fixed raw guards: four raw frame cases, six raw domain cases, 13 raw
//! projection candidates, nine raw tracking candidates, six raw affine candidates,
//! eight operation mappings,
//! and bounded closed-error formatting/source controls. Raw candidates are not
//! product state. The literal source ceilings must be reserved separately from
//! the public fixture profiles before these tests are integrated or run.
//! The historical S154 upper was60 validations/162 callbacks/252 predicate visits.
//! S1.4b adds the two bound fields, ten raw-bound neighbors and21 validation
//! requests; the coordinator must extend the source reservation before execution.
//! These declarations are source work, not measured execution counters.

use std::{error::Error, io, num::NonZeroU64};

use garde::Validate;
use time_domain::MonotonicNs;

use super::{
    AffineCandidate, ArithmeticOperation, DomainCandidate, FixtureChartBoundIssue,
    FixtureChartBoundOperation, FixtureChartError, FixtureChartField, FixtureChartIssue,
    FixtureChartStep, FramesCandidate, ProjectionCandidate, TrackingPhaseCandidate, domain_valid,
    frames_valid,
};
use crate::{FixtureEnclosureError, FixtureProjectionError, FixtureTrackingProjectionError};

/// Admit a fixed nonzero test tag without retaining input or effects.
/// # Errors
/// Reject the impossible zero fixture tag.
fn label(value: u64) -> Result<NonZeroU64, io::Error> {
    NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero fixture tag"))
}

/// Independent raw-frame and domain neighbors exercise the actual Garde carriers.
/// # Errors
/// Preserve the fixed test-label construction failure.
/// # Panics
/// Reject a mismatched typed predicate or Garde outcome.
#[test]
fn chart_raw_frame_and_domain_carriers() -> Result<(), io::Error> {
    let a = label(11)?;
    let b = label(17)?;
    let c = label(23)?;
    let d = label(29)?;
    for (labels, legal) in [
        ((a, b, c, d), true),
        ((a, b, a, d), false),
        ((a, b, c, b), false),
        ((c, d, a, b), true),
    ] {
        assert_eq!(frames_valid(&labels), legal);
        assert_eq!(FramesCandidate { labels }.validate().is_ok(), legal);
    }
    for (start, reference, end, legal) in [
        (0, 0, 0, true),
        (0, 0, 1_u64 << 53, true),
        (0, 0, (1_u64 << 53) + 1, false),
        (2, 1, 3, false),
        (0, 4, 3, false),
        (3, 3, 2, false),
    ] {
        let times = (
            MonotonicNs::new(start),
            MonotonicNs::new(reference),
            MonotonicNs::new(end),
        );
        assert_eq!(domain_valid(&times), legal);
        assert_eq!(DomainCandidate { times }.validate().is_ok(), legal);
    }
    Ok(())
}

/// Independently challenge both signed raw descriptor fields without a factory.
/// # Panics
/// Reject an invalid field's acceptance or rejection of either legal signed value.
#[test]
fn chart_raw_affine_carrier_checks_both_signed_fields() {
    for candidate in [
        AffineCandidate {
            phase: f64::NAN,
            rate: 0.0,
            phase_error: 0.0,
            rate_error: 0.0,
        },
        AffineCandidate {
            phase: 0.0,
            rate: f64::INFINITY,
            phase_error: 0.0,
            rate_error: 0.0,
        },
        AffineCandidate {
            phase: f64::from_bits(1),
            rate: 0.0,
            phase_error: 0.0,
            rate_error: 0.0,
        },
        AffineCandidate {
            phase: 0.0,
            rate: -f64::from_bits(1),
            phase_error: 0.0,
            rate_error: 0.0,
        },
    ] {
        assert!(candidate.validate().is_err());
    }
    for candidate in [
        AffineCandidate {
            phase: -1.0,
            rate: -1.0,
            phase_error: 0.0,
            rate_error: 0.0,
        },
        AffineCandidate {
            phase: -0.0,
            rate: 0.0,
            phase_error: 0.0,
            rate_error: 0.0,
        },
    ] {
        assert!(candidate.validate().is_ok());
    }
    affine_bound_neighbors();
}

/// Check both raw coefficient-bound roles and their typed admission precedence.
/// # Panics
/// Reject unsupported raw bounds, a metadata-only Garde rule or lost first cause.
fn affine_bound_neighbors() {
    for (phase_error, rate_error, legal) in [
        (f64::NAN, 0.0, false),
        (f64::INFINITY, 0.0, false),
        (f64::from_bits(1), 0.0, false),
        (-1.0, 0.0, false),
        (0.0, f64::NAN, false),
        (0.0, f64::INFINITY, false),
        (0.0, f64::from_bits(1), false),
        (0.0, -1.0, false),
        (1.0, 1.0, true),
        (-0.0, 0.0, true),
    ] {
        let candidate = AffineCandidate {
            phase: 0.0,
            rate: 0.0,
            phase_error,
            rate_error,
        };
        assert_eq!(candidate.validate().is_ok(), legal);
        let admitted = candidate.admit();
        assert_eq!(admitted.is_ok(), legal);
        if !legal {
            let expected = if phase_error == 0.0 {
                FixtureChartField::RateCoefficientRoundoff
            } else {
                FixtureChartField::PhaseCoefficientRoundoff
            };
            assert!(
                matches!(admitted, Err(FixtureChartError::InvalidScalar { field, .. }) if field == expected)
            );
        }
    }
    assert!(matches!(
        AffineCandidate {
            phase: 0.0,
            rate: 0.0,
            phase_error: -1.0,
            rate_error: f64::NAN
        }
        .admit(),
        Err(FixtureChartError::InvalidScalar {
            field: FixtureChartField::PhaseCoefficientRoundoff,
            issue: FixtureChartIssue::Negative
        })
    ));
}

/// Construct a raw candidate with one of the four projection-role inputs.
///
/// Other field tags leave healthy literals unchanged. No checked value is built.
fn projection(field: FixtureChartField, value: f64) -> ProjectionCandidate {
    ProjectionCandidate {
        center: if field == FixtureChartField::AnchorCenter {
            value
        } else {
            1.0
        },
        rate: if field == FixtureChartField::Rate {
            value
        } else {
            1.0
        },
        center_error: if field == FixtureChartField::CenterRoundoff {
            value
        } else {
            1.0
        },
        rate_error: if field == FixtureChartField::RateRoundoff {
            value
        } else {
            1.0
        },
    }
}

/// Test every raw output role, signed-zero neighbors and first-error precedence.
/// # Panics
/// Reject changed typed role/issue or Garde acceptance of an invalid raw field.
#[test]
fn chart_raw_projection_candidate_roles_and_precedence() {
    use FixtureChartField::{AnchorCenter, CenterRoundoff, Rate, RateRoundoff};
    use FixtureChartIssue::{Negative, NonFinite, Subnormal};
    for (field, value, issue) in [
        (AnchorCenter, f64::NAN, NonFinite),
        (Rate, f64::INFINITY, NonFinite),
        (AnchorCenter, -f64::from_bits(1), Subnormal),
        (Rate, -f64::from_bits(1), Subnormal),
        (CenterRoundoff, -1.0, Negative),
        (RateRoundoff, -1.0, Negative),
        (CenterRoundoff, -f64::from_bits(1), Subnormal),
        (RateRoundoff, f64::NAN, NonFinite),
        (CenterRoundoff, f64::INFINITY, NonFinite),
        (RateRoundoff, f64::from_bits(1), Subnormal),
    ] {
        let candidate = projection(field, value);
        assert!(candidate.validate().is_err());
        assert_eq!(
            candidate.admit(),
            Err(FixtureChartError::InvalidScalar { field, issue })
        );
    }
    for candidate in [
        ProjectionCandidate {
            center: -1.0,
            rate: -1.0,
            center_error: 0.0,
            rate_error: -0.0,
        },
        ProjectionCandidate {
            center: -0.0,
            rate: 0.0,
            center_error: 1.0,
            rate_error: f64::MIN_POSITIVE,
        },
    ] {
        assert!(candidate.validate().is_ok());
        assert_eq!(candidate.admit(), Ok(()));
    }
    let both = ProjectionCandidate {
        center: f64::NAN,
        rate: -f64::from_bits(1),
        center_error: -1.0,
        rate_error: f64::INFINITY,
    };
    assert!(both.validate().is_err());
    assert_eq!(
        both.admit(),
        Err(FixtureChartError::InvalidScalar {
            field: AnchorCenter,
            issue: NonFinite
        })
    );
}

/// Exercise signed tracking phase separately from its nonnegative bound.
/// # Panics
/// Reject changed scalar role, unsupported values, or first-error precedence.
#[test]
fn chart_raw_tracking_candidate_roles_and_precedence() {
    use FixtureChartField::{TrackingPhase, TrackingRoundoff};
    use FixtureChartIssue::{Negative, NonFinite, Subnormal};
    for (value, error, field, issue) in [
        (f64::NAN, 1.0, TrackingPhase, NonFinite),
        (-f64::from_bits(1), 1.0, TrackingPhase, Subnormal),
        (f64::NEG_INFINITY, 1.0, TrackingPhase, NonFinite),
        (1.0, -1.0, TrackingRoundoff, Negative),
        (1.0, -f64::from_bits(1), TrackingRoundoff, Subnormal),
        (1.0, f64::NAN, TrackingRoundoff, NonFinite),
    ] {
        let candidate = TrackingPhaseCandidate { value, error };
        assert!(candidate.validate().is_err());
        assert_eq!(
            candidate.admit(),
            Err(FixtureChartError::InvalidScalar { field, issue })
        );
    }
    for candidate in [
        TrackingPhaseCandidate {
            value: -1.0,
            error: 0.0,
        },
        TrackingPhaseCandidate {
            value: -0.0,
            error: -0.0,
        },
    ] {
        assert!(candidate.validate().is_ok());
        assert_eq!(candidate.admit(), Ok(()));
    }
    let both = TrackingPhaseCandidate {
        value: f64::NAN,
        error: -1.0,
    };
    assert!(both.validate().is_err());
    assert_eq!(
        both.admit(),
        Err(FixtureChartError::InvalidScalar {
            field: TrackingPhase,
            issue: NonFinite
        })
    );
}

/// Check each closed arithmetic step against its independent primitive mapping.
/// # Panics
/// Reject a role mapped to the wrong primitive.
#[test]
fn chart_steps_preserve_actual_operation_mapping() {
    use ArithmeticOperation::{Add, Divide, Multiply, Subtract};
    use FixtureChartStep::{
        AnchorCenter, ComposeDisplacement, ComposeRate, Displacement, Rate, RateTimesElapsed,
        SecondsDivide, TrackingPhase,
    };
    for (step, operation) in [
        (SecondsDivide, Divide),
        (RateTimesElapsed, Multiply),
        (Displacement, Add),
        (AnchorCenter, Subtract),
        (TrackingPhase, Subtract),
        (Rate, Subtract),
        (ComposeDisplacement, Add),
        (ComposeRate, Add),
    ] {
        assert_eq!(step.operation(), operation);
    }
}

/// Retain fixed redacted messages and genuine typed error-chain projections.
/// # Panics
/// Reject a changed closed message, unexpected source, or wrong nested error type.
#[test]
fn chart_errors_preserve_closed_messages_and_typed_sources() {
    for (error, expected) in [
        (FixtureChartError::InvalidFrames, "chart frame relationship"),
        (
            FixtureChartError::InvalidDomain,
            "chart domain relationship",
        ),
        (FixtureChartError::ContextMismatch, "chart context mismatch"),
        (
            FixtureChartError::SourceMappingMismatch,
            "chart source mapping mismatch",
        ),
        (
            FixtureChartError::MissingReport,
            "chart missing fitted report",
        ),
        (FixtureChartError::UnknownOutcome, "chart outcome unknown"),
        (
            FixtureChartError::ValidationInvariant,
            "chart validation invariant",
        ),
        (FixtureChartError::Invariant, "chart producer invariant"),
    ] {
        assert_eq!(error.to_string(), expected);
        assert!(Error::source(&error).is_none());
        assert!(format!("{error:?}").len() < 512);
    }
    let bound = FixtureChartError::Bound {
        operation: FixtureChartBoundOperation::Add,
        issue: FixtureChartBoundIssue::InvalidOperand,
    };
    assert_eq!(bound.to_string(), "chart bound Add: InvalidOperand");
    assert!(Error::source(&bound).is_none());
    let projection = FixtureChartError::Projection {
        source: FixtureProjectionError::MissingAnchor,
    };
    assert_eq!(projection.to_string(), "chart original projection failure");
    assert_eq!(
        Error::source(&projection).and_then(|e| e.downcast_ref::<FixtureProjectionError>()),
        Some(&FixtureProjectionError::MissingAnchor)
    );
    let tracking = FixtureChartError::Tracking {
        source: FixtureTrackingProjectionError::Projection {
            source: FixtureProjectionError::MissingAnchor,
        },
    };
    assert_eq!(tracking.to_string(), "chart original tracking failure");
    assert!(
        Error::source(&tracking)
            .and_then(|e| e.downcast_ref::<FixtureTrackingProjectionError>())
            .is_some()
    );
    let enclosure = FixtureChartError::Enclosure {
        source: FixtureEnclosureError::Unrepresentable,
    };
    assert_eq!(enclosure.to_string(), "chart enclosure failure");
    assert_eq!(
        Error::source(&enclosure).and_then(|e| e.downcast_ref::<FixtureEnclosureError>()),
        Some(&FixtureEnclosureError::Unrepresentable)
    );
}
