// SPDX-License-Identifier: GPL-2.0-only
//! Direct raw validation controls; public construction still owns admission.

use std::{error::Error, num::NonZeroU64};

use garde::Validate;
use time_domain::MonotonicNs;

use super::{
    ChartBatchCandidate, ChartExpectation, FixtureAffineChart, FixtureChartBatch,
    FixtureChartBatchError, FixtureChartDomain, FixtureChartFrames,
};

/// Check raw Garde output against literal inclusive-domain facts and neighbors.
///
/// Public rejection alone would also succeed if the custom rule accepted
/// everything: the constructor repeats the typed predicate. This observation
/// therefore consumes the actual raw carrier's validation result separately.
/// All chart components use their real fallible checked constructors.
/// # Errors
/// Preserve a genuine checked descriptor-construction failure.
/// # Panics
/// Reject changed inclusive endpoints or a raw rule that silently accepts all.
#[test]
fn raw_chart_batch_header_checks_inclusive_domain() -> Result<(), Box<dyn Error>> {
    let two = NonZeroU64::new(2).ok_or(FixtureChartBatchError::Invariant)?;
    let frames = FixtureChartFrames::new(NonZeroU64::MIN, NonZeroU64::MIN, two, two)?;
    let domain = FixtureChartDomain::new(
        MonotonicNs::new(5),
        MonotonicNs::new(6),
        MonotonicNs::new(7),
    )?;
    let chart = FixtureAffineChart::new(frames, domain, 0.0, 0.0)?;
    let expected = |at| ChartExpectation {
        frames: chart.frames(),
        domain: chart.domain(),
        phase_bits: chart.phase_at_reference_seconds().to_bits(),
        rate_bits: chart.rate_seconds_per_second().to_bits(),
        at: MonotonicNs::new(at),
    };
    let raw = ChartBatchCandidate { values: &[] };
    for at in [5, 6, 7] {
        assert!(raw.validate_with(&expected(at)).is_ok());
    }
    for at in [4, 8] {
        assert!(raw.validate_with(&expected(at)).is_err());
        assert!(matches!(
            FixtureChartBatch::new(&chart, MonotonicNs::new(at), &[]),
            Err(FixtureChartBatchError::EvaluationOutsideDomain)
        ));
    }
    Ok(())
}
