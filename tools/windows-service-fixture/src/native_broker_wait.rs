// SPDX-License-Identifier: GPL-2.0-only
//! Scheduling on the existing broker owner using only owned event handles.

use std::{
    os::windows::io::{AsRawHandle, BorrowedHandle, FromRawHandle, OwnedHandle},
    ptr,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use windows_sys::{
    Win32::{
        Foundation::{GetLastError, HANDLE, WAIT_FAILED, WAIT_OBJECT_0, WAIT_TIMEOUT},
        System::Threading::{CreateEventW, INFINITE, SetEvent, WaitForMultipleObjects},
    },
    core::BOOL,
};

use crate::{NativeError, NativeOperation};

/// Callback-safe process-owned channel carrying no payload or authority.
pub(super) struct BrokerControlWake {
    /// The unnamed auto-reset event coalesces notifications for one owner.
    event: OwnedHandle,
    /// Zero means no failure. Other values preserve the first exact code plus one.
    signal_failure: AtomicU64,
}

impl BrokerControlWake {
    /// Create a private auto-reset event before publishing the native callback.
    ///
    /// # Errors
    /// Return the original native creation failure without admitting a handler.
    pub(super) fn new() -> Result<Self, NativeError> {
        #[expect(unsafe_code, reason = "create a private coalescing control event")]
        // SAFETY: Null attributes and name select noninheritance and an unnamed
        // event. The other arguments select auto reset and initially unsignaled.
        let handle: HANDLE = unsafe { CreateEventW(ptr::null(), 0, 0, ptr::null()) };
        if handle.is_null() {
            return Err(last_error(NativeOperation::BrokerWakePrepare));
        }
        #[expect(unsafe_code, reason = "adopt the successful control event handle")]
        // SAFETY: Successful creation returns a unique handle released by CloseHandle.
        let event: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(handle) };
        Ok(Self {
            event,
            signal_failure: AtomicU64::new(0),
        })
    }

    /// Signal after the callback has published the original sticky control.
    pub(super) fn signal(&self) {
        #[expect(
            unsafe_code,
            reason = "wake the existing broker owner without blocking"
        )]
        // SAFETY: The event remains open through the borrow, with modify rights
        // granted by creation. SetEvent retains no Rust pointer.
        let signaled: BOOL = unsafe { SetEvent(self.event.as_raw_handle()) };
        if signaled == 0 {
            self.remember_signal_failure(last_error(NativeOperation::BrokerWakeSignal).code());
        }
    }

    /// Supply a live event for a scripted host handoff without executing a wait.
    #[cfg(test)]
    pub(super) fn test_event(&self) -> BorrowedHandle<'_> {
        std::os::windows::io::AsHandle::as_handle(&self.event)
    }

    /// Retain the first callback-side failure without locking or allocating.
    fn remember_signal_failure(&self, code: u32) {
        let encoded: u64 = u64::from(code) + 1;
        let _earlier_failure: Result<u64, u64> =
            self.signal_failure
                .compare_exchange(0, encoded, Ordering::Release, Ordering::Relaxed);
    }

    /// Recover the exact recorded code, including zero.
    fn signal_failure(&self) -> Option<NativeError> {
        let encoded: u64 = self.signal_failure.load(Ordering::Acquire);
        let code: u32 = u32::try_from(encoded.checked_sub(1)?).ok()?;
        Some(NativeError::new(NativeOperation::BrokerWakeSignal, code))
    }
}

/// Which scheduling hint the owner observed. Its lifecycle guards still decide.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WakeSource {
    /// An accepted control signaled the coalescing event.
    Control,
    /// The exact queued accept's event won its one scheduling observation.
    Completion,
    /// The bounded native wait elapsed.
    Timeout,
    /// A prior wake failure requires the original bounded thread wait.
    Fallback,
}

/// One scheduling hint owned by an exact pending submission.
pub(super) struct CompletionWakeState {
    /// A manual-reset completion grants at most one scheduling hint.
    observed: bool,
}

impl CompletionWakeState {
    /// Initialize the hint for a newly submitted operation.
    pub(super) const fn new() -> Self {
        Self { observed: false }
    }

    /// Borrow this submission's state with the event held for that submission.
    pub(super) const fn lend<'a>(
        &'a mut self,
        event: BorrowedHandle<'a>,
    ) -> PendingCompletionWake<'a> {
        PendingCompletionWake { event, state: self }
    }
}

/// Synchronous borrow of the exact event and its submission's one-shot hint.
pub(super) struct PendingCompletionWake<'a> {
    /// The original manual-reset event, held by the native operation's owner.
    event: BorrowedHandle<'a>,
    /// Serialized mutation applies only to this exact pending submission.
    state: &'a mut CompletionWakeState,
}

impl PendingCompletionWake<'_> {
    /// Offer the original event only before its hint has been observed.
    fn eligible_event(&self) -> Option<BorrowedHandle<'_>> {
        (!self.state.observed).then_some(self.event)
    }
}

/// Native wait state for the serialized broker owner.
pub(super) struct BrokerWait {
    /// A native wake failure is returned once and selects bounded fallback.
    failed: bool,
}

impl BrokerWait {
    /// Start with the native wait enabled.
    pub(super) const fn new() -> Self {
        Self { failed: false }
    }

    /// Wait for control or exact accept scheduling with the current finite budget.
    ///
    /// # Errors
    /// Return an original signaling or wait failure once, then select fallback.
    pub(super) fn observe(
        &mut self,
        control: &BrokerControlWake,
        timeout: Duration,
        completion: Option<PendingCompletionWake<'_>>,
    ) -> Result<WakeSource, NativeError> {
        if self.failed {
            return Ok(WakeSource::Fallback);
        }
        let outcome: Result<WakeSource, NativeError> = Self::wait(control, timeout, completion);
        if outcome.is_err() {
            self.failed = true;
        }
        outcome
    }

    /// Wait synchronously while both event owners remain borrowed.
    ///
    /// # Errors
    /// Return the immediately available failure from the native wake channel.
    fn wait(
        control: &BrokerControlWake,
        timeout: Duration,
        completion: Option<PendingCompletionWake<'_>>,
    ) -> Result<WakeSource, NativeError> {
        if let Some(failure) = control.signal_failure() {
            return Err(failure);
        }
        let control_handle: HANDLE = control.event.as_raw_handle();
        let event: Option<BorrowedHandle<'_>> = completion
            .as_ref()
            .and_then(PendingCompletionWake::eligible_event);
        let (handles, count): ([HANDLE; 2], u32) = match event {
            Some(event) => ([control_handle, event.as_raw_handle()], 2),
            None => ([control_handle, ptr::null_mut()], 1),
        };
        let timeout_ms: u32 = finite_wait_millis(timeout);
        #[expect(unsafe_code, reason = "wait non-alertably on the two borrowed events")]
        // SAFETY: Only the first count entries are used. They borrow distinct
        // live waitable events for the entire synchronous call. Control is
        // first so WaitForMultipleObjects prefers it when both are signaled.
        let status: u32 = unsafe { WaitForMultipleObjects(count, handles.as_ptr(), 0, timeout_ms) };
        let wake: WakeSource = match status {
            WAIT_OBJECT_0 => WakeSource::Control,
            selected if count == 2 && selected == WAIT_OBJECT_0 + 1 => WakeSource::Completion,
            WAIT_TIMEOUT => WakeSource::Timeout,
            WAIT_FAILED => return Err(last_error(NativeOperation::BrokerWakeWait)),
            unexpected => {
                return Err(NativeError::new(
                    NativeOperation::BrokerWakeWait,
                    unexpected,
                ));
            }
        };
        // SetEvent failure cannot wake a pending native wait. Recheck the
        // bounded mailbox immediately after that finite wait returns.
        if let Some(failure) = control.signal_failure() {
            return Err(failure);
        }
        if wake == WakeSource::Completion
            && let Some(hint) = completion
        {
            hint.state.observed = true;
        }
        Ok(wake)
    }
}

/// Round a positive fraction up to one millisecond, preserving a finite wait.
fn finite_wait_millis(timeout: Duration) -> u32 {
    const MAX_FINITE: u32 = INFINITE - 1;
    let rounded: u128 = timeout.as_nanos().div_ceil(1_000_000);
    u32::try_from(rounded).unwrap_or(MAX_FINITE).min(MAX_FINITE)
}

/// Capture the immediately preceding Win32 failure on the same thread.
fn last_error(operation: NativeOperation) -> NativeError {
    #[expect(unsafe_code, reason = "retain this thread's immediate native error")]
    // SAFETY: GetLastError has no pointer or lifetime preconditions.
    let code: u32 = unsafe { GetLastError() };
    NativeError::new(operation, code)
}

#[cfg(test)]
mod tests {
    use std::{
        os::windows::io::{AsHandle, AsRawHandle, FromRawHandle, OwnedHandle},
        ptr,
        time::Duration,
    };

    use windows_sys::{
        Win32::{
            Foundation::HANDLE,
            System::Threading::{CreateEventW, INFINITE, ResetEvent, SetEvent},
        },
        core::BOOL,
    };

    use super::{
        BrokerControlWake, BrokerWait, CompletionWakeState, WakeSource, finite_wait_millis,
        last_error,
    };
    use crate::{NativeError, NativeOperation};

    /// A notification sent ahead of a wait remains observable and coalesces.
    ///
    /// # Errors
    /// Reports a native event creation or wait failure.
    ///
    /// # Panics
    /// Panics if the wait loses an early signal or observes an extra wake.
    #[test]
    fn broker_control_wake_retains_and_coalesces_early_signal() -> Result<(), NativeError> {
        let control: BrokerControlWake = BrokerControlWake::new()?;
        let mut wait: BrokerWait = BrokerWait::new();
        assert_eq!(
            wait.observe(&control, Duration::ZERO, None)?,
            WakeSource::Timeout
        );
        control.signal();
        control.signal();
        assert_eq!(
            wait.observe(&control, Duration::ZERO, None)?,
            WakeSource::Control
        );
        assert_eq!(
            wait.observe(&control, Duration::ZERO, None)?,
            WakeSource::Timeout
        );
        Ok(())
    }

    /// Control wins a tie and each submission gets one completion hint.
    /// # Errors
    /// Reports a native event creation or wait failure.
    /// # Panics
    /// Panics if control loses priority, a repeated hint arrives, or a
    /// successor using the same event loses its first hint.
    #[test]
    fn broker_wait_prioritizes_control_and_suppresses_a_repeated_completion_hint()
    -> Result<(), NativeError> {
        let control: BrokerControlWake = BrokerControlWake::new()?;
        let completion_event: OwnedHandle = signaled_completion_event()?;
        let mut wait: BrokerWait = BrokerWait::new();
        {
            let mut first: CompletionWakeState = CompletionWakeState::new();
            control.signal();
            for expected in [
                WakeSource::Control,
                WakeSource::Completion,
                WakeSource::Timeout,
            ] {
                assert_eq!(
                    wait.observe(
                        &control,
                        Duration::ZERO,
                        Some(first.lend(completion_event.as_handle()))
                    )?,
                    expected
                );
            }
        }

        // Simulate the kernel clearing the event at successor submission.
        // The test has no pending kernel operation on this private handle.
        #[expect(
            unsafe_code,
            reason = "clear the owned fixture event before submission"
        )]
        // SAFETY: The manual-reset fixture event stays open with modification access.
        let reset: BOOL = unsafe { ResetEvent(completion_event.as_raw_handle()) };
        if reset == 0 {
            return Err(last_error(NativeOperation::BrokerWakeWait));
        }
        let mut successor: CompletionWakeState = CompletionWakeState::new();
        assert_eq!(
            wait.observe(
                &control,
                Duration::ZERO,
                Some(successor.lend(completion_event.as_handle()))
            )?,
            WakeSource::Timeout
        );
        #[expect(unsafe_code, reason = "signal the private successor fixture event")]
        // SAFETY: This is the same open fixture event with modification access.
        let signaled: BOOL = unsafe { SetEvent(completion_event.as_raw_handle()) };
        if signaled == 0 {
            return Err(last_error(NativeOperation::BrokerWakeSignal));
        }
        for expected in [WakeSource::Completion, WakeSource::Timeout] {
            assert_eq!(
                wait.observe(
                    &control,
                    Duration::ZERO,
                    Some(successor.lend(completion_event.as_handle()))
                )?,
                expected
            );
        }
        Ok(())
    }

    /// Supply a real manual-reset event that stays signaled across observations.
    /// # Errors
    /// Preserve the native failure if the fixture cannot allocate its event.
    fn signaled_completion_event() -> Result<OwnedHandle, NativeError> {
        #[expect(
            unsafe_code,
            reason = "create a private signaled manual-reset fixture event"
        )]
        // SAFETY: Null attributes and name select a noninherited, unnamed event.
        // Both scalar flags are true for manual reset and initially signaled.
        let event: HANDLE = unsafe { CreateEventW(ptr::null(), 1, 1, ptr::null()) };
        if event.is_null() {
            return Err(last_error(NativeOperation::BrokerEndpointAcceptEvent));
        }
        #[expect(unsafe_code, reason = "take ownership of the fixture event")]
        // SAFETY: The successful creation produced a unique CloseHandle-owned event.
        Ok(unsafe { OwnedHandle::from_raw_handle(event) })
    }

    /// First signal failure keeps its exact code and selects bounded fallback.
    ///
    /// # Errors
    /// Reports a native event creation or subsequent wait failure.
    ///
    /// # Panics
    /// Panics if the first failure is replaced or fallback is lost.
    #[test]
    fn broker_wait_projects_first_signal_failure_once() -> Result<(), NativeError> {
        let control: BrokerControlWake = BrokerControlWake::new()?;
        let mut wait: BrokerWait = BrokerWait::new();
        control.remember_signal_failure(0);
        control.remember_signal_failure(u32::MAX);
        assert_eq!(
            wait.observe(&control, Duration::ZERO, None),
            Err(NativeError::new(NativeOperation::BrokerWakeSignal, 0))
        );
        assert_eq!(
            wait.observe(&control, Duration::ZERO, None)?,
            WakeSource::Fallback
        );
        Ok(())
    }

    /// Native wait conversion keeps zero and uses no infinite sentinel.
    ///
    /// # Panics
    /// Panics if the wait rounds down or reaches the infinite sentinel.
    #[test]
    fn broker_wait_duration_rounds_up_but_never_reaches_infinite() {
        assert_eq!(finite_wait_millis(Duration::ZERO), 0);
        assert_eq!(finite_wait_millis(Duration::from_nanos(1)), 1);
        assert_eq!(finite_wait_millis(Duration::from_millis(3)), 3);
        assert_eq!(finite_wait_millis(Duration::from_nanos(3_000_001)), 4);
        assert_eq!(
            finite_wait_millis(Duration::from_millis(u64::from(INFINITE) - 1)),
            INFINITE - 1
        );
        assert_eq!(
            finite_wait_millis(Duration::from_millis(u64::from(INFINITE))),
            INFINITE - 1
        );
        assert_eq!(finite_wait_millis(Duration::MAX), INFINITE - 1);
    }
}
