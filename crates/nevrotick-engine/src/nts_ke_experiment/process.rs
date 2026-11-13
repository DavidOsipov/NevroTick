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
        Ok(ProcessClaim { admission: self })
    }
}

/// Uncloneable evidence of one permanent admission, consumed into its provider scope.
pub(super) struct ProcessClaim<'a> {
    /// Original latch remains borrowed throughout the invocation scope.
    admission: &'a ProcessAdmission,
}

/// Consume only the fixed authoritative claim before invoking the Native effect factory.
/// Independent checked admissions never authorize global provider effects.
/// # Errors
/// Refuse foreign origins locally without invoking the factory or resetting admission.
pub(super) fn native_scope_with<'a>(
    claim: ProcessClaim<'a>,
    factory: impl FnOnce(ProcessClaim<'a>) -> ProviderScope<'a>,
) -> Result<ProviderScope<'a>, Failure> {
    if !std::ptr::eq(
        std::ptr::from_ref(claim.admission),
        std::ptr::from_ref(&super::PROCESS_ADMISSION),
    ) {
        return Err(Failure::Control);
    }
    Ok(factory(claim))
}

/// Actual provider shutdown owner, borrowed by every dependent association.
pub(super) struct ProviderScope<'a> {
    /// Own the sole claim until all dependent custody has retired.
    _claim: ProcessClaim<'a>,
    /// This is the one actual global shutdown, never a per-request lease.
    #[cfg(windows)]
    _shutdown: Option<rustls_cng_crypto::ShutdownHandle>,
    /// Test-only destruction witness; this actual scope alone owns its strong reference.
    /// Both real constructors establish it; it conveys no provider or validation fact.
    #[cfg(test)]
    lifetime: std::sync::Arc<()>,
}

impl<'a> ProviderScope<'a> {
    /// Consume the sole authoritative claim into the actual global provider owner.
    /// # Errors
    /// Propagate local admission refusal before constructing the shutdown handle.
    #[cfg(windows)]
    pub(super) fn new(claim: ProcessClaim<'a>) -> Result<Self, Failure> {
        native_scope_with(claim, |claim| Self {
            _claim: claim,
            _shutdown: Some(rustls_cng_crypto::ShutdownHandle {}),
            #[cfg(test)]
            lifetime: std::sync::Arc::new(()),
        })
    }

    /// Consume one genuine component claim; perform no Windows/provider operation.
    #[cfg(test)]
    pub(super) fn component(claim: ProcessClaim<'a>) -> Self {
        Self {
            _claim: claim,
            #[cfg(windows)]
            _shutdown: None,
            lifetime: std::sync::Arc::new(()),
        }
    }

    /// Observe this actual scope's ordinary destruction without retaining it.
    #[cfg(test)]
    pub(super) fn retirement_witness(&self) -> std::sync::Weak<()> {
        std::sync::Arc::downgrade(&self.lifetime)
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
            let _provider = ProviderScope::component(claim);
            assert!(matches!(admission.claim(), Err(Failure::ProcessUsed)));
        }
        assert!(matches!(admission.claim(), Err(Failure::ProcessUsed)));
        Ok(())
    }
}
