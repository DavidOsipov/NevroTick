// SPDX-License-Identifier: GPL-2.0-only
//! Bounded synthetic regression contracts without sample or clock authority.
//!
//! S1.2a supplies mandatory checked point construction, response-independent
//! geometry and a bounded inverse-divisor affine fit. Exact constant phases
//! retain the first admitted phase bits with zero rate; nonconstant response
//! arithmetic is evaluated in the prescribed chronological binary64 order.
//! S1.2b/c add fresh-line fixture residual diagnostics and exact-observation
//! normalization. S1.2d adds one bounded fixture-history transaction: context/order
//! admission, staged exact rows, one normalization and fresh same-input statistics
//! precede complete state replacement. Rejection preserves the prior rows and
//! report. These fixture values supply no physical uncertainty, eligibility,
//! source selection, network, persistence or clock authority.
//!
//! ```
//! # use std::num::NonZeroU64;
//! # use time_core::{FitError, FitInput, FitPoint};
//! # fn main() -> Result<(), FitError> {
//! let points = [
//!     FitPoint::new(-2.0, 0.001, 1.0)?,
//!     FitPoint::new(-1.0, 0.002, 1.0)?,
//!     FitPoint::new(0.0, 0.003, 1.0)?,
//! ];
//! let input = FitInput::new(NonZeroU64::MIN, &points)?;
//! assert_eq!(input.frame(), NonZeroU64::MIN);
//! assert_eq!(input.sample_count(), 3);
//! assert_eq!(points[0].relative_seconds(), -2.0);
//! # Ok(()) }
//! ```
#![forbid(unsafe_code)]

/// Sole checked owner for synthetic point, batch, geometry and result contracts.
mod regression;

/// Checked synthetic intervals and bounded, inert fixture-majority selection.
mod selection;

pub use regression::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FitInput, FitPoint,
    FixtureContext, FixtureHistory, FixtureHistoryReport, FixtureLine, FixtureNormalization,
    FixtureObservation, FixtureResidualStatistics, FixtureRunsError, FixtureRunsSelection,
    FixtureRunsStage, FixtureRunsStop, HistoryError, InsufficientFixture, NormalizationComponent,
    NormalizationError, NormalizationStep, ObservationError, PointField, PointIssue,
    PreparedFixtureInput, ResidualError, ResidualField, ResidualIssue, ResidualStep, fit,
    prepare_fixture_input, residual_statistics, select_fixture_runs,
};

pub use selection::{
    BatchError, FixtureBatch, FixtureInterval, FixtureMinimum, FixtureOutcomeKind,
    FixtureSelection, FixtureWitness, IntervalError, MinimumError, SelectionError,
    SelectionInvariantStage, select_fixture,
};
