// SPDX-License-Identifier: GPL-2.0-only
//! Bound-consuming Q33 geometry for one immutable mathematical chart projection.
//!
//! The chart owner supplies its transformed raw-anchor center and the separate
//! upper bound on new arithmetic error. Outward positive addition widens the
//! original geometry radius before the existing exact endpoint owner runs.
//! Retaining the complete chart projection preserves the actual descriptor,
//! original provenance and arithmetic bounds. Statistical and physical
//! uncertainty, source eligibility and current clock authority are not inferred.

use std::fmt;

use crate::{
    FixtureChartError, FixtureChartProjection, FixtureEndpointRounding, FixtureInterval,
    projection_enclosure::enclose_labeled_scalars,
};

/// Complete chart provenance and the checked bound-consuming fixture geometry.
///
/// No conversion into a legacy projected interval discards the chart identity.
/// Keep the descriptor lifetime and complete chart provenance in the consumer:
///
/// ```
/// use time_core::{FixtureChartError, FixtureChartInterval, FixtureChartProjection,
///     enclose_fixture_chart_projection};
/// fn keep_chart<'chart>(projection: &FixtureChartProjection<'chart>)
///     -> Result<FixtureChartInterval<'chart>, FixtureChartError>
/// {
///     enclose_fixture_chart_projection(projection)
/// }
/// ```
///
/// A chart projection cannot discard its bounds through a legacy conversion:
///
/// ```compile_fail,E0277
/// use time_core::{FixtureAnchorProjection, FixtureChartProjection};
/// fn discard_chart(projection: FixtureChartProjection<'_>) -> FixtureAnchorProjection {
///     projection.into()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FixtureChartInterval<'chart> {
    /// Same opaque checked chart projection, still borrowing its exact descriptor.
    projection: FixtureChartProjection<'chart>,
    /// Completed exact Q33 enclosure of the widened represented geometry.
    interval: FixtureInterval,
    /// Exact represented lower endpoint's relation to the Q33 lattice.
    lower_rounding: FixtureEndpointRounding,
    /// Exact represented upper endpoint's relation to the Q33 lattice.
    upper_rounding: FixtureEndpointRounding,
}

impl fmt::Debug for FixtureChartInterval<'_> {
    /// Reveal only the type, without labels, times, map parameters or geometry.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureChartInterval { .. }")
    }
}

impl<'chart> FixtureChartInterval<'chart> {
    /// Borrow unchanged complete chart provenance without recalculation or effects.
    #[must_use]
    pub const fn projection(&self) -> &FixtureChartProjection<'chart> {
        &self.projection
    }

    /// Borrow completed fixture geometry without asserting physical uncertainty.
    #[must_use]
    pub const fn interval(&self) -> &FixtureInterval {
        &self.interval
    }

    /// Return lower endpoint rounding without changing geometry or provenance.
    #[must_use]
    pub const fn lower_rounding(&self) -> FixtureEndpointRounding {
        self.lower_rounding
    }

    /// Return upper endpoint rounding without changing geometry or provenance.
    #[must_use]
    pub const fn upper_rounding(&self) -> FixtureEndpointRounding {
        self.upper_rounding
    }
}

/// Enclose one chart projection after outwardly consuming its raw-center bound.
///
/// The original represented radius plus new arithmetic error is rounded upward
/// by the chart's bound-only addition owner. The existing exact endpoint owner
/// floors/ceils the resulting represented center/radius onto Q33. Work and
/// scalar scratch are constant; no new observation, fit, state or effect occurs.
/// This is mathematical fixture geometry, not a physical UTC confidence bound.
/// # Errors
/// Preserve radius-upper failure before the actual exact-enclosure cause. Publish
/// no interval or partial provenance on either failure.
pub fn enclose_fixture_chart_projection<'chart>(
    projection: &FixtureChartProjection<'chart>,
) -> Result<FixtureChartInterval<'chart>, FixtureChartError> {
    let radius = projection.enclosure_distance_seconds()?;
    let (interval, lower_rounding, upper_rounding) = enclose_labeled_scalars(
        projection.context().source_id().get(),
        projection.center_seconds(),
        radius,
    )
    .map_err(|source| FixtureChartError::Enclosure { source })?;
    Ok(FixtureChartInterval {
        projection: *projection,
        interval,
        lower_rounding,
        upper_rounding,
    })
}
