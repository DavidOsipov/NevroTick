// SPDX-License-Identifier: GPL-2.0-only
//! Ordinary-user native subscription lifetime control in a separate test binary.

#![cfg(windows)]
#![forbid(unsafe_code)]

use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use windows_platform_native::{ResumeRegistrationError, ResumeWatch};

/// The capture remains available to the installed callback.
const ARMED: u8 = 0;
/// An actual external notification entered the action.
const DELIVERED: u8 = 1;
/// The capture was released without entering the action.
const RETIRED: u8 = 2;

/// Observe the release of the one capture owned by the installed callback.
struct Capture {
    /// Distinguish an actual invocation from release of an undelivered action.
    state: Arc<AtomicU8>,
}

impl Drop for Capture {
    fn drop(&mut self) {
        let _previous: Result<u8, u8> =
            self.state
                .compare_exchange(ARMED, RETIRED, Ordering::AcqRel, Ordering::Acquire);
    }
}

/// Observe the captured action around implicit drop of a real native watch.
///
/// # Errors
/// Report registration refusal or interruption by a delivered power event.
///
/// # Panics
/// Fail if admission releases the capture early or implicit drop leaves it armed.
#[test]
fn implicit_drop_disarms_undelivered_capture() -> io::Result<()> {
    let state: Arc<AtomicU8> = Arc::new(AtomicU8::new(ARMED));
    let capture: Capture = Capture {
        state: Arc::clone(&state),
    };
    let before_drop: u8 = {
        let _watch: ResumeWatch = ResumeWatch::subscribe(move || {
            capture.state.store(DELIVERED, Ordering::Release);
            drop(capture);
            Ok(())
        })
        .map_err(|error: ResumeRegistrationError| {
            io::Error::other(format!("native resume registration refused: {error:?}"))
        })?;
        state.load(Ordering::Acquire)
    };
    let after_drop: u8 = state.load(Ordering::Acquire);
    if before_drop == DELIVERED || after_drop == DELIVERED {
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "external power event interrupted the implicit Drop observation",
        ));
    }
    assert_eq!(
        before_drop, ARMED,
        "subscription released the capture early"
    );
    assert_eq!(
        after_drop, RETIRED,
        "implicit Drop left the undelivered callback capture armed"
    );
    assert_eq!(
        Arc::strong_count(&state),
        1,
        "callback capture still retained"
    );
    Ok(())
}
