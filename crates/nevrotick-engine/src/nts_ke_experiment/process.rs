// SPDX-License-Identifier: GPL-2.0-only
//! One process claim and its actual outer provider shutdown scope.
//!
//! Admission never resets. The CNG provider caches handles globally and shutdown
//! does not reset those caches; this owner is neither a pool nor a refcount.

use std::sync::atomic::{AtomicBool, Ordering};

use super::Failure;

/// One permanent process-use latch, claimed before any provider operation.
pub(super) struct ProcessAdmission {
    /// False only before the first successful claim; no reset API exists.
    used: AtomicBool,
}

impl ProcessAdmission {
    /// Construct an unclaimed latch without native work.
    pub(super) const fn new() -> Self {
        Self {
            used: AtomicBool::new(false),
        }
    }

    /// Claim the sole invocation without waiting or creating background work.
    /// # Errors
    /// Refuse every second/reentrant invocation, even after prior retirement.
    pub(super) fn claim(&self) -> Result<ProcessClaim<'_>, Failure> {
        self.used
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| Failure::ProcessUsed)?;
        Ok(ProcessClaim { _admission: self })
    }
}

/// Uncloneable evidence of the real process admission, with no reset-on-drop.
pub(super) struct ProcessClaim<'a> {
    /// Original latch remains borrowed throughout the invocation scope.
    _admission: &'a ProcessAdmission,
}

/// Actual provider shutdown owner, borrowed by every dependent association.
pub(super) struct ProviderScope<'a> {
    /// Keep process admission alive until all dependent custody has retired.
    _claim: &'a ProcessClaim<'a>,
    /// This is the one actual global shutdown, never a per-request lease.
    #[cfg(windows)]
    _shutdown: Option<rustls_cng_crypto::ShutdownHandle>,
}

impl<'a> ProviderScope<'a> {
    /// Establish the sole actual provider scope after process admission.
    #[cfg(windows)]
    pub(super) fn new(claim: &'a ProcessClaim<'a>) -> Self {
        Self {
            _claim: claim,
            _shutdown: Some(rustls_cng_crypto::ShutdownHandle {}),
        }
    }

    /// Component-only lifetime owner; performs no Windows/provider operation.
    #[cfg(test)]
    pub(super) fn component(claim: &'a ProcessClaim<'a>) -> Self {
        Self {
            _claim: claim,
            #[cfg(windows)]
            _shutdown: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Failure, ProcessAdmission, ProviderScope};

    /// A live claim and its retirement both preserve permanent refusal.
    /// # Errors
    /// Propagate unexpected refusal of the first component claim.
    /// # Panics
    /// Fail if a concurrent claim or post-scope reinitialization is allowed.
    #[test]
    fn claim_is_permanent_before_and_after_provider_scope() -> Result<(), Failure> {
        let admission = ProcessAdmission::new();
        {
            let claim = admission.claim()?;
            let _provider = ProviderScope::component(&claim);
            assert!(matches!(admission.claim(), Err(Failure::ProcessUsed)));
        }
        assert!(matches!(admission.claim(), Err(Failure::ProcessUsed)));
        Ok(())
    }
}
