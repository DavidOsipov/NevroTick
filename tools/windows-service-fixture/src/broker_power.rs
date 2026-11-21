// SPDX-License-Identifier: GPL-2.0-only
//! One broker observation generation, retired by the first delivered power notice.

use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering::SeqCst},
};

/// The sole native observation is still eligible to begin or publish.
const PENDING: u8 = 0;
/// The final observation decision preceded any delivered invalidation.
const REPORTED: u8 = 1;
/// This generation can no longer publish a live observation.
const RETIRED: u8 = 2;

/// Own one power context. Its retirement cannot be undone or transferred.
pub(in crate::host) struct BrokerPowerGeneration(Arc<AtomicU8>);

impl BrokerPowerGeneration {
    /// Establish an initially pending context before arming the callback.
    pub(in crate::host) fn new() -> Self {
        Self(Arc::new(AtomicU8::new(PENDING)))
    }

    /// Bind the broker's one pending read to this exact context.
    pub(super) fn fence(&self) -> BrokerPowerFence {
        BrokerPowerFence(Arc::clone(&self.0))
    }

    /// Prepare a callback that only publishes sticky invalidation.
    pub(in crate::host) fn invalidator(&self) -> impl FnOnce() + Send + 'static {
        let state: Arc<AtomicU8> = Arc::clone(&self.0);
        move || state.store(RETIRED, SeqCst)
    }
}

impl Drop for BrokerPowerGeneration {
    /// Withdraw the context before dropping the native registration.
    fn drop(&mut self) {
        self.0.store(RETIRED, SeqCst);
    }
}

/// The read's single opportunity to commit within the issued generation.
pub(in crate::host) struct BrokerPowerFence(Arc<AtomicU8>);

impl BrokerPowerFence {
    /// Sample eligibility at the pre-call edge without blocking power delivery.
    pub(super) fn can_start(&self) -> bool {
        self.0.load(SeqCst) == PENDING
    }

    /// Consume the fence and order the final decision against invalidation.
    pub(super) fn report(self) -> bool {
        self.0
            .compare_exchange(PENDING, REPORTED, SeqCst, SeqCst)
            .is_ok()
    }
}

/// Keep the callback and its single generation under the subscribing thread.
/// Field order also retires the generation before fallback native cleanup.
#[cfg(windows)]
#[must_use = "keep the broker power observer alive through the read and close it explicitly"]
pub(in crate::host) struct BrokerPowerWatch {
    /// The current observation generation, revoked before unregistering.
    generation: BrokerPowerGeneration,
    /// The original one-shot native registration bound to this context.
    watch: windows_platform_native::ResumeWatch,
}

#[cfg(windows)]
impl BrokerPowerWatch {
    /// Arm the first delivered suspend or automatic-resume fence in this process.
    ///
    /// # Errors
    /// Preserve the native observer's registration refusal. A callback arriving
    /// during registration can already retire the new generation.
    pub(in crate::host) fn subscribe()
    -> Result<Self, windows_platform_native::ResumeRegistrationError> {
        let generation: BrokerPowerGeneration = BrokerPowerGeneration::new();
        let invalidate = generation.invalidator();
        let watch: windows_platform_native::ResumeWatch =
            windows_platform_native::ResumeWatch::subscribe(move || {
                invalidate();
                Ok(())
            })?;
        Ok(Self { generation, watch })
    }

    /// Bind a pending coordinate read to the subscribed generation.
    pub(super) fn fence(&self) -> BrokerPowerFence {
        self.generation.fence()
    }

    /// Retire eligibility before asking Windows to release the subscription.
    ///
    /// # Errors
    /// Preserve an original unregister error after the context is irrevocably closed.
    pub(in crate::host) fn close(
        self,
    ) -> Result<(), windows_platform_native::ResumeUnregistrationError> {
        let Self { generation, watch } = self;
        drop(generation);
        watch.close()
    }
}
