//! Construct checked knowledge from the reproducible compiled local dataset.
//!
//! The generator binds reviewed input bytes and metadata; this constructor
//! re-establishes domain invariants. Package authentication, current date scope
//! and clock authority remain with their separate consumers.

use super::generated::{
    CONTINUOUS_FROM, DATA_REVISION, FINITE_FUTURE_BEFORE, HISTORY_COMPLETE_BEFORE, LEAP_RECORDS,
    PROVENANCE_SHA256, RAW_SHA256,
};
use super::{
    LeapContinuousFrom, LeapDataIdentity, LeapError, LeapFutureExpiry, LeapHistoryBefore,
    LeapKnowledge,
};

/// Admit the compiled local records through the mandatory shared constructors.
///
/// The returned immutable knowledge uses no file, network or ambient clock.
/// Interpretation still requires an explicit finite caller date scope. This
/// function grants no authentication, sample eligibility or clock authority.
///
/// # Errors
/// Rejects malformed generated scalar, identity, ledger or rule relationships
/// without publishing partial knowledge. No fallback dataset is substituted.
pub fn bundled_leap_knowledge() -> Result<LeapKnowledge<'static>, LeapError> {
    let history = LeapHistoryBefore::new(HISTORY_COMPLETE_BEFORE)?;
    let expiry = FINITE_FUTURE_BEFORE
        .map(LeapFutureExpiry::new)
        .transpose()?;
    let continuous = CONTINUOUS_FROM.map(LeapContinuousFrom::new).transpose()?;
    let identity = LeapDataIdentity::new(DATA_REVISION, RAW_SHA256, PROVENANCE_SHA256)?;
    LeapKnowledge::new(LEAP_RECORDS, history, expiry, continuous, identity)
}
