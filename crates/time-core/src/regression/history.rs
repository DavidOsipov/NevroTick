// SPDX-License-Identifier: GPL-2.0-only
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2011-2014, 2016-2018, 2021
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 *
 **********************************************************************
*/
//! Private owner for one failure-atomic, bounded fixture-history transaction.
//!
//! Modified by David Osipov with AI assistance, 30 October 2026. Selected
//! chronological append/capacity context is `SST_AccumulateSample`,
//! `sourcestats.c` 289-329, Chrony 4.9 revision
//! `8df4f1263e206585e6c4e61352d042c2dd12916a`. The pretranslation source ledger
//! and `../../PROVENANCE.md` record the exact scope. Ring indexing, phase sign
//! inversion, extra runs storage, peer/root quality and reset/pruning policy are
//! deliberately excluded. Existing checked normalization remains its sole owner.
//!
//! Append checks the complete current context and exact time order before staging
//! one fixed64-cell observation array. The existing normalizer admits that exact
//! staged prefix once; sufficient input reaches fresh same-input statistics once.
//! Only then does one complete state assignment publish rows/count/report. Any
//! earlier failure preserves the entire old value, including its exact reference.
//! No constructor, raw field or mutable view can bypass this sole state producer.
//! Context labels assert no authentication, native continuity, freshness, source
//! eligibility or clock authority. Storage and staging use fixed arrays rather
//! than heap-backed history. No ambient read, I/O, persistence, callback,
//! asynchronous work or external effect is provided by this owner. Existing
//! scalar-validation owners retain their bounded temporary-report contracts.
//! Source-owned per-append maxima are127 logical observation `Copy` assignments,
//! one normalization, one statistics call and zero direct fits or domain/scalar
//! constructors. The normalizer owns at most64 durations,64 generated points and
//! one checked input. Old64-row storage and one staged64-cell array coexist with
//! that bounded normalization scratch/input and old/new summaries; no additional
//! history identities are retained. These are logical bounds, not measured
//! compiler copies, stack bytes, allocator/RSS or native-resource qualification.

use std::{cmp::Ordering, fmt};

use time_domain::MonotonicNs;

use super::{
    FixtureContext, FixtureNormalization, FixtureObservation, FixtureResidualStatistics,
    MAX_POINTS, NormalizationError, ResidualError, prepare_fixture_input, residual_statistics,
};

/// Closed append rejection, with no raw observation, timestamp or report payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HistoryError {
    /// Incoming labels differ from the immutable history labels; checked first.
    ContextMismatch,
    /// Incoming exact monotonic time equals the committed newest time.
    DuplicateTime,
    /// Incoming exact monotonic time precedes the committed newest time.
    OutOfOrderTime,
    /// The one staged normalization failed before any live-state replacement.
    Normalization {
        /// Actual bounded preparation cause, with staged-prefix row indices.
        source: NormalizationError,
    },
    /// Fresh statistics for the same staged prepared input failed before commit.
    Statistics {
        /// Actual bounded statistics cause, preserved without relabelling.
        source: ResidualError,
    },
    /// A private count/slice relationship failed before any state replacement.
    StateInvariant,
}

impl fmt::Display for HistoryError {
    /// Format closed bounded context for every variant, including forged causes.
    ///
    /// Existing cause owners admit only closed typed metadata, with no raw scalar,
    /// observation, array or validation-report data. This projection has no sink.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContextMismatch => formatter.write_str("history context mismatch"),
            Self::DuplicateTime => formatter.write_str("history duplicate time"),
            Self::OutOfOrderTime => formatter.write_str("history out-of-order time"),
            Self::Normalization { source } => {
                write!(formatter, "history normalization: {source}")
            }
            Self::Statistics { source } => write!(formatter, "history statistics: {source}"),
            Self::StateInvariant => formatter.write_str("history state invariant"),
        }
    }
}

impl std::error::Error for HistoryError {
    /// Borrow the actual retained cause; payload-free rejections have no source.
    ///
    /// The closed projection is total for all public combinations and performs
    /// no parsing, arithmetic, allocation, diagnostic emission or state change.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Normalization { source } => Some(source),
            Self::Statistics { source } => Some(source),
            Self::ContextMismatch
            | Self::DuplicateTime
            | Self::OutOfOrderTime
            | Self::StateInvariant => None,
        }
    }
}

/// Immutable last-committed fixture report, with no freshness or authority claim.
///
/// Only the history transaction may construct this private association of actual
/// prepared context/reference and its same-input statistics. No mixed-input
/// factory, default, deserialization, raw field or mutable inner view is exposed.
/// Count and frame remain owned by the contained statistics, not duplicated here.
pub struct FixtureHistoryReport {
    /// Unchanged complete context of the producer's actual prepared input.
    context: FixtureContext,
    /// Actual exact newest reference used by that same prepared input.
    reference: MonotonicNs,
    /// Owned summary returned by one fresh same-input statistics computation.
    statistics: FixtureResidualStatistics,
}

impl fmt::Debug for FixtureHistoryReport {
    /// Format type/context/count and reference presence; omit all numeric data.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FixtureHistoryReport")
            .field("context", &self.context)
            .field("sample_count", &self.statistics.sample_count())
            .field("reference_present", &true)
            .finish_non_exhaustive()
    }
}

impl FixtureHistoryReport {
    /// Return the unchanged complete prepared context, without new authority.
    #[must_use]
    pub const fn context(&self) -> FixtureContext {
        self.context
    }

    /// Return the exact committed prepared reference, without conversion or reads.
    #[must_use]
    pub const fn reference(&self) -> MonotonicNs {
        self.reference
    }

    /// Borrow the actual owned summary without recomputation, mutation or effects.
    #[must_use]
    pub const fn statistics(&self) -> &FixtureResidualStatistics {
        &self.statistics
    }
}

/// Complete nonempty committed state; one owned value is the sole commit unit.
///
/// The sole append producer establishes a count in `1..=64`, exact chronological
/// context agreement for its prefix and a same-input report iff count is at least
/// three. Slots outside the private admitted prefix are never observable. No
/// destructor, callback, external resource or separately mutable reference is retained.
struct CommittedState {
    /// Fixed storage initialized from actual incoming input before retained copies.
    observations: [FixtureObservation; MAX_POINTS],
    /// Actual admitted prefix length; no extra runs history is retained.
    sample_count: usize,
    /// Exact same-prefix summary, absent only for insufficient counts one or two.
    report: Option<FixtureHistoryReport>,
}

/// Pure bounded fixture history with one immutable context and one state owner.
///
/// Construction is empty; the sole append operation owns exclusive synchronous
/// access. Immutable scalar admission remains with `FixtureObservation::new` and
/// its existing time/phase owners. The history must independently admit changing
/// context/order relationships before staging. No clear, reset, rebind, prune,
/// mutable view, default, deserialization or independent report factory exists.
pub struct FixtureHistory {
    /// Complete immutable caller assertion; no current authority is implied.
    context: FixtureContext,
    /// None is the sole empty state; all mutable associations commit together.
    state: Option<CommittedState>,
}

impl fmt::Debug for FixtureHistory {
    /// Format only permitted context, actual count and reference/report presence.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FixtureHistory")
            .field("context", &self.context)
            .field("sample_count", &self.sample_count())
            .field("reference_present", &self.reference().is_some())
            .field("report_present", &self.report().is_some())
            .finish_non_exhaustive()
    }
}

impl FixtureHistory {
    /// Create an empty history retaining the unchanged complete asserted context.
    ///
    /// Count is zero and reference/report are absent. No observation, retained
    /// prefix, normalization, statistic, fit, allocation or external effect is
    /// created; the type's fixed inline capacity is not an allocation claim.
    #[must_use]
    pub const fn new(context: FixtureContext) -> Self {
        Self {
            context,
            state: None,
        }
    }

    /// Return the unchanged immutable context without checking trust or freshness.
    #[must_use]
    pub const fn context(&self) -> FixtureContext {
        self.context
    }

    /// Return the actual committed prefix length, without scanning or effects.
    #[must_use]
    pub const fn sample_count(&self) -> usize {
        match &self.state {
            Some(state) => state.sample_count,
            None => 0,
        }
    }

    /// Return the actual last committed row's exact time, absent only when empty.
    ///
    /// The reference is read from the held prefix, never reconstructed, refreshed
    /// from rejected input or converted through floating point. No state changes.
    #[must_use]
    pub fn reference(&self) -> Option<MonotonicNs> {
        let state = self.state.as_ref()?;
        let index = state.sample_count.checked_sub(1)?;
        state
            .observations
            .get(index)
            .map(FixtureObservation::observed_at)
    }

    /// Borrow the actual held prefix row; any index outside that prefix is absent.
    ///
    /// This includes count and `usize::MAX`; unused initialized slots are hidden.
    /// The shared borrow remains tied to self. No reconstruction, copying, numeric
    /// work, mutation or external effect occurs.
    #[must_use]
    pub fn observation(&self, index: usize) -> Option<&FixtureObservation> {
        let state = self.state.as_ref()?;
        state.observations.get(..state.sample_count)?.get(index)
    }

    /// Borrow the actual committed report, without recomputation or relabelling.
    ///
    /// Empty and insufficient states expose None. A present report means only
    /// last committed fixture statistics, not current/fresh/authenticated time.
    #[must_use]
    pub const fn report(&self) -> Option<&FixtureHistoryReport> {
        match &self.state {
            Some(state) => state.report.as_ref(),
            None => None,
        }
    }

    /// Admit and prepare one bounded new prefix, then replace complete state once.
    ///
    /// Reuse immutable observation scalar guarantees; check current whole context
    /// before exact time order under exclusive synchronous ownership. One staged
    /// 64-cell array retains the old prefix below capacity or only its newest63
    /// rows when full, then the actual incoming row. No live eviction occurs.
    ///
    /// Normalize that exact prefix once. Only its actual sufficient prepared
    /// input reaches one statistics call; privately pair the returned summary
    /// with that prepared context/reference. Insufficient counts keep no report
    /// only after successful normalizer window checks. Immutable producer-owned
    /// associations are reused without another normalization or independent fit.
    ///
    /// One final complete assignment commits rows/count/report, deriving the
    /// history reference from the actual newest retained row. All fallible work
    /// precedes this write; bounded owned scratch has no fallible destructor or
    /// callback. There is no later expected failure, ambient read or external
    /// effect. Failure leaves every old row/bit/context/count/reference/report
    /// unchanged. Logical `Copy` work is at most64 incoming fills plus63 retained
    /// overwrites; no observation/domain/scalar constructor is invoked here.
    /// # Errors
    /// Context mismatch precedes duplicate/older exact time. Private count/slice
    /// or checked-increment disagreement returns `StateInvariant`. Otherwise
    /// preserve the one normalizer's actual staged-prefix cause, then the one
    /// sufficient statistics computation's actual cause. No partial state or
    /// acknowledgement escapes, and numerical rejection never retries a subset.
    pub fn try_append(&mut self, observation: FixtureObservation) -> Result<(), HistoryError> {
        if observation.context() != self.context {
            return Err(HistoryError::ContextMismatch);
        }
        let retained = if let Some(state) = &self.state {
            let current = state
                .observations
                .get(..state.sample_count)
                .ok_or(HistoryError::StateInvariant)?;
            let newest = current.last().ok_or(HistoryError::StateInvariant)?;
            match observation.observed_at().cmp(&newest.observed_at()) {
                Ordering::Equal => return Err(HistoryError::DuplicateTime),
                Ordering::Less => return Err(HistoryError::OutOfOrderTime),
                Ordering::Greater => {}
            }
            if current.len() == MAX_POINTS {
                current.get(1..).ok_or(HistoryError::StateInvariant)?
            } else {
                current
            }
        } else {
            &[]
        };
        let sample_count = retained
            .len()
            .checked_add(1)
            .ok_or(HistoryError::StateInvariant)?;
        let mut observations = [observation; MAX_POINTS];
        for (target, retained) in observations.iter_mut().zip(retained) {
            *target = *retained;
        }
        let staged = observations
            .get(..sample_count)
            .ok_or(HistoryError::StateInvariant)?;
        let report = match prepare_fixture_input(self.context, staged)
            .map_err(|source| HistoryError::Normalization { source })?
        {
            FixtureNormalization::Insufficient(_) => None,
            FixtureNormalization::Prepared(prepared) => {
                let statistics = residual_statistics(prepared.input())
                    .map_err(|source| HistoryError::Statistics { source })?;
                Some(FixtureHistoryReport {
                    context: prepared.context(),
                    reference: prepared.reference(),
                    statistics,
                })
            }
        };
        self.state = Some(CommittedState {
            observations,
            sample_count,
            report,
        });
        Ok(())
    }
}
