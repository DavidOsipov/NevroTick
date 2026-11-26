// SPDX-License-Identifier: GPL-3.0-only
//! Require both documented strict-handle fields before native UI startup.
//!
//! `winnt.h` declares two one-bit fields followed by 30 reserved bits. The
//! exception and permanent fields must match when setting the policy. A
//! debugger/verifier's temporary exception field alone cannot admit this UI.
//! Queries and at most one tightening are synchronous on the current process;
//! rejection never retries weaker or pretends that a successful set rolled back.

use super::PolicyCalls;
use std::fmt;

/// Matching `RaiseExceptionOnInvalidHandleReference` and permanent-enable bits.
const REQUIRED_FLAGS: u32 = 0x01 | 0x02;

/// First bounded reason permanent strict handles could not admit the UI.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "A strict-handle refusal must prevent native backend startup"]
pub enum StrictHandlePolicyError {
    /// The original current-process query failed with this Win32 code.
    ReadInitial(u32),
    /// The initial word contains at least one of the 30 reserved bits.
    UnexpectedInitialFlags(u32),
    /// Applying both matching fields failed with this original Win32 code.
    Apply(u32),
    /// The query after the accepted setter failed with this Win32 code.
    ReadEffective(u32),
    /// The effective word contains at least one reserved bit.
    UnexpectedEffectiveFlags(u32),
    /// The effective word omits the exception or permanent-enable field.
    MissingEffectiveFlags(u32),
}

impl fmt::Display for StrictHandlePolicyError {
    /// Format only a fixed stage and bounded native code or policy word.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadInitial(code) => write!(
                formatter,
                "Cannot read the startup strict-handle policy (Win32 {code})."
            ),
            Self::UnexpectedInitialFlags(flags) => write!(
                formatter,
                "Unexpected startup strict-handle policy flags ({flags:#010x})."
            ),
            Self::Apply(code) => write!(
                formatter,
                "Cannot set the startup strict-handle policy (Win32 {code})."
            ),
            Self::ReadEffective(code) => write!(
                formatter,
                "Cannot verify the effective strict-handle policy (Win32 {code})."
            ),
            Self::UnexpectedEffectiveFlags(flags) => write!(
                formatter,
                "Unexpected effective strict-handle policy flags ({flags:#010x})."
            ),
            Self::MissingEffectiveFlags(flags) => write!(
                formatter,
                "Incomplete effective strict-handle policy ({flags:#010x}, required 0x00000003)."
            ),
        }
    }
}

impl std::error::Error for StrictHandlePolicyError {}

/// Admit an inherited permanent word or set and read back exactly both fields.
///
/// # Errors
/// Preserve the first failed call or refused word. Unknown initial flags cause
/// no setter; an accepted setter has no rollback even if later readback refuses.
pub(super) fn enforce_with(calls: &mut impl PolicyCalls) -> Result<(), StrictHandlePolicyError> {
    let initial: u32 = calls.read().map_err(StrictHandlePolicyError::ReadInitial)?;
    if initial & !REQUIRED_FLAGS != 0 {
        return Err(StrictHandlePolicyError::UnexpectedInitialFlags(initial));
    }
    if initial == REQUIRED_FLAGS {
        return Ok(());
    }
    calls
        .apply(REQUIRED_FLAGS)
        .map_err(StrictHandlePolicyError::Apply)?;
    let observed: u32 = calls
        .read()
        .map_err(StrictHandlePolicyError::ReadEffective)?;
    if observed & !REQUIRED_FLAGS != 0 {
        return Err(StrictHandlePolicyError::UnexpectedEffectiveFlags(observed));
    }
    if observed != REQUIRED_FLAGS {
        return Err(StrictHandlePolicyError::MissingEffectiveFlags(observed));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
