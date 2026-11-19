// SPDX-License-Identifier: GPL-2.0-only
//! Controlled registration status and retirement ownership checks.

use std::num::NonZeroU32;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::{
    ResumeRegistrationError, ResumeUnregistrationError, complete_registration, retire_registration,
};
use crate::resume_slot::ResumeSlot;

/// An owned fixture token that cannot be passed to a native release call.
#[derive(Debug, Eq, PartialEq)]
struct FixtureRegistration(#[doc = "Independent token identity."] u8);

/// Bind an inert action whose delivery and ownership can be observed separately.
///
/// # Panics
/// Fail if the test does not supply a fresh local slot.
fn bind_action(slot: &ResumeSlot) -> Arc<AtomicBool> {
    let delivered: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let capture: Arc<AtomicBool> = Arc::clone(&delivered);
    assert!(slot.bind(Box::new(move || {
        capture.store(true, Ordering::Relaxed);
        Ok(())
    })));
    delivered
}

/// A nonzero status takes precedence over the handle output, and zero still requires a handle.
///
/// # Errors
/// Fail if the literal nonzero fixture status cannot be constructed.
///
/// # Panics
/// Fail if registration refusal loses its cause, retains a capture or reopens the slot.
#[test]
fn registration_failures_disarm_without_reopening_slot() -> Result<(), ()> {
    let code: NonZeroU32 = NonZeroU32::new(5).ok_or(())?;
    let cases: [(u32, Option<FixtureRegistration>, ResumeRegistrationError); 2] = [
        (
            5,
            Some(FixtureRegistration(7)),
            ResumeRegistrationError::Native(code),
        ),
        (0, None, ResumeRegistrationError::MissingHandle),
    ];
    for (status, handle, expected) in cases {
        let slot: ResumeSlot = ResumeSlot::new();
        let delivered: Arc<AtomicBool> = bind_action(&slot);
        assert_eq!(complete_registration(&slot, status, handle), Err(expected));
        assert_eq!(Arc::strong_count(&delivered), 1);
        assert_eq!(slot.notify(18), Ok(()));
        assert!(!delivered.load(Ordering::Relaxed));
        assert!(!slot.bind(Box::new(|| Ok(()))));
    }
    Ok(())
}

/// Success retains the exact token and leaves the original action available for delivery.
///
/// # Panics
/// Fail if the token changes or registration prematurely releases the action.
#[test]
fn registration_success_preserves_live_capture() {
    let slot: ResumeSlot = ResumeSlot::new();
    let delivered: Arc<AtomicBool> = bind_action(&slot);
    assert_eq!(
        complete_registration(&slot, 0, Some(FixtureRegistration(7))),
        Ok(FixtureRegistration(7))
    );
    assert_eq!(Arc::strong_count(&delivered), 2);
    assert_eq!(slot.notify(4), Ok(()));
    assert!(delivered.load(Ordering::Relaxed));
    assert_eq!(Arc::strong_count(&delivered), 1);
}

/// Observe serialized disarm, exact status and single-attempt custody for both outcomes.
///
/// # Panics
/// Fail if retirement invokes the callback, loses the status, leaks or retries the token.
#[test]
fn retirement_disarms_before_attempt_and_never_retries() {
    let cases: [(u32, Result<(), u32>); 2] = [(0, Ok(())), (6, Err(6))];
    for (status, expected) in cases {
        let slot: ResumeSlot = ResumeSlot::new();
        let delivered: Arc<AtomicBool> = bind_action(&slot);
        let mut handle: Option<FixtureRegistration> = Some(FixtureRegistration(7));
        let mut attempt: Option<(FixtureRegistration, Result<(), ()>, bool, usize)> = None;
        let result: Result<(), ResumeUnregistrationError> =
            retire_registration(&slot, &mut handle, |token: FixtureRegistration| {
                attempt = Some((
                    token,
                    slot.notify(4),
                    delivered.load(Ordering::Relaxed),
                    Arc::strong_count(&delivered),
                ));
                status
            });
        assert_eq!(attempt, Some((FixtureRegistration(7), Ok(()), false, 2)));
        assert_eq!(
            result.map_err(|error: ResumeUnregistrationError| error.code().get()),
            expected
        );
        assert!(handle.is_none());
        assert_eq!(Arc::strong_count(&delivered), 1);
        assert_eq!(slot.notify(18), Ok(()));
        assert!(!delivered.load(Ordering::Relaxed));
        let mut retried: bool = false;
        assert_eq!(
            retire_registration(&slot, &mut handle, |_token: FixtureRegistration| {
                retried = true;
                0
            }),
            Ok(())
        );
        assert!(!retried);
    }
}
