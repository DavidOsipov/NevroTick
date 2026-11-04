// SPDX-License-Identifier: GPL-2.0-only
//! Private chart-combination arithmetic, without state or publication authority.
//!
//! Checked point/error companions bound only the new arithmetic over the exact
//! stored input bits. They do not bound source-fit error, statistical deviation,
//! physical uncertainty or an applied clock effect. Parent integration must keep
//! the original witnessed-population singleton rule and the complete transaction.

mod reduction;
/// Sole owner of the private rounded point/error construction and propagation.
mod roundoff;

pub(super) use reduction::{ChartQuantities, ChartReductionError, ChartSums};
/// Parent-only arithmetic surface; no public checked-value escape is introduced.
pub(super) use roundoff::{Companion, CompanionError};
