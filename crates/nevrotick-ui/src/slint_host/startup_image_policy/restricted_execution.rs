// SPDX-License-Identifier: GPL-3.0-only
//! Fixed dynamic-code and extension-point admission before native UI startup.
//!
//! Dynamic-code admission requires prohibition without thread opt-out or remote
//! downgrade. Inherited audit state is preserved, and additional known audit
//! state at readback is accepted. Extension-point admission requires its single
//! disable bit. Each check permits one tightening and one effective query;
//! refusal never retries weaker or claims to undo a successful setting.

use std::fmt;

/// The only restricted-execution policies selected by the desktop startup owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RestrictedExecutionPolicy {
    /// Prohibit dynamic code without thread opt-out or remote downgrade.
    DynamicCode,
    /// Disable the documented legacy process extension points.
    ExtensionPoints,
}

impl fmt::Display for RestrictedExecutionPolicy {
    /// Format a fixed policy label without native or caller-supplied text.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DynamicCode => formatter.write_str("dynamic-code"),
            Self::ExtensionPoints => formatter.write_str("extension-point"),
        }
    }
}

/// The first bounded refusal from one fixed restricted-execution policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "A restricted-execution refusal must prevent native backend startup"]
pub enum RestrictedExecutionPolicyError {
    /// The initial query failed before any tightening was attempted.
    ReadInitial {
        /// The fixed policy selected before the query.
        policy: RestrictedExecutionPolicy,
        /// The original Win32 error from the query.
        code: u32,
    },
    /// The initial policy word contains reserved fields.
    UnexpectedInitialFlags {
        /// The fixed policy whose initial word was refused.
        policy: RestrictedExecutionPolicy,
        /// The complete original word, including its reserved fields.
        flags: u32,
    },
    /// The single tightening attempt failed.
    Apply {
        /// The fixed policy whose setter failed.
        policy: RestrictedExecutionPolicy,
        /// The original Win32 error from the setter.
        code: u32,
    },
    /// The effective query failed after the setter reported success.
    ReadEffective {
        /// The fixed policy whose readback failed.
        policy: RestrictedExecutionPolicy,
        /// The original Win32 error from the effective query.
        code: u32,
    },
    /// The effective word contains reserved fields.
    UnexpectedEffectiveFlags {
        /// The fixed policy whose effective word was refused.
        policy: RestrictedExecutionPolicy,
        /// The complete original word, including its reserved fields.
        flags: u32,
    },
    /// Known effective fields do not meet the requested enforcement contract.
    MissingEffectiveFlags {
        /// The fixed policy whose recognized effective fields were refused.
        policy: RestrictedExecutionPolicy,
        /// The requested enforcement fields and any inherited audit field.
        requested: u32,
        /// The complete recognized effective word.
        observed: u32,
    },
}

impl fmt::Display for RestrictedExecutionPolicyError {
    /// Format only the fixed policy/stage and bounded native codes or words.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadInitial { policy, code } => write!(
                formatter,
                "Cannot read the startup {policy} policy (Win32 {code})."
            ),
            Self::UnexpectedInitialFlags { policy, flags } => write!(
                formatter,
                "Unexpected startup {policy} policy flags ({flags:#010x})."
            ),
            Self::Apply { policy, code } => write!(
                formatter,
                "Cannot set the startup {policy} policy (Win32 {code})."
            ),
            Self::ReadEffective { policy, code } => write!(
                formatter,
                "Cannot verify the effective {policy} policy (Win32 {code})."
            ),
            Self::UnexpectedEffectiveFlags { policy, flags } => write!(
                formatter,
                "Unexpected effective {policy} policy flags ({flags:#010x})."
            ),
            Self::MissingEffectiveFlags {
                policy,
                requested,
                observed,
            } => write!(
                formatter,
                "Incomplete effective {policy} policy ({observed:#010x}, required {requested:#010x})."
            ),
        }
    }
}

impl std::error::Error for RestrictedExecutionPolicyError {}

/// Synchronous current-process calls bound to one closed policy selection.
pub(super) trait PolicyCalls {
    /// Return the fixed policy owned by this adapter, before its first read.
    fn policy(&self) -> RestrictedExecutionPolicy;

    /// Query this adapter's current-process DWORD without substituting failures.
    ///
    /// # Errors
    /// Return the original Win32 code if the query fails.
    fn read(&mut self) -> Result<u32, u32>;

    /// Apply the recognized fixed-policy word synchronously to this process.
    ///
    /// # Errors
    /// Return the original Win32 code if the setter fails.
    fn apply(&mut self, flags: u32) -> Result<(), u32>;
}

/// Admit inherited enforcement or tighten once and inspect its actual readback.
///
/// Dynamic-code words `1` and `9`, or extension-point word `1`, need no setter.
/// Other recognized words require one tightening; dynamic-code tightening keeps
/// inherited audit while clearing opt-out/downgrade. Newly observed audit is
/// accepted, but lost inherited audit refuses. Native effects are confined to
/// the supplied synchronous adapter; this check creates no backend or worker.
///
/// # Errors
/// Preserve the first failed call or reserved/incomplete word with its policy.
/// Unknown initial fields cause no write. A successful write is not rolled back
/// after a later refusal, and no path retries a weaker setting.
pub(super) fn enforce_with(
    calls: &mut impl PolicyCalls,
) -> Result<(), RestrictedExecutionPolicyError> {
    let policy: RestrictedExecutionPolicy = calls.policy();
    let known: u32 = match policy {
        RestrictedExecutionPolicy::DynamicCode => 0x0f,
        RestrictedExecutionPolicy::ExtensionPoints => 0x01,
    };
    let initial: u32 = calls
        .read()
        .map_err(|code| RestrictedExecutionPolicyError::ReadInitial { policy, code })?;
    if initial & !known != 0 {
        return Err(RestrictedExecutionPolicyError::UnexpectedInitialFlags {
            policy,
            flags: initial,
        });
    }
    let requested: u32 = match policy {
        RestrictedExecutionPolicy::DynamicCode => 0x01 | (initial & 0x08),
        RestrictedExecutionPolicy::ExtensionPoints => 0x01,
    };
    if initial == requested {
        return Ok(());
    }
    calls
        .apply(requested)
        .map_err(|code| RestrictedExecutionPolicyError::Apply { policy, code })?;
    let observed: u32 = calls
        .read()
        .map_err(|code| RestrictedExecutionPolicyError::ReadEffective { policy, code })?;
    if observed & !known != 0 {
        return Err(RestrictedExecutionPolicyError::UnexpectedEffectiveFlags {
            policy,
            flags: observed,
        });
    }
    let forbidden: u32 = match policy {
        RestrictedExecutionPolicy::DynamicCode => 0x06,
        RestrictedExecutionPolicy::ExtensionPoints => 0x00,
    };
    if observed & requested != requested || observed & forbidden != 0 {
        return Err(RestrictedExecutionPolicyError::MissingEffectiveFlags {
            policy,
            requested,
            observed,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests;
