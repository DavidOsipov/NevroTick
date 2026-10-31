// SPDX-License-Identifier: GPL-2.0-only

//! Direct evidence for the actual raw-u64 checked increment seam.
//! No private state/counter value is forged or externally reset. Four helper
//! calls use primitive inputs; this claims no practical public overflow journey.

use std::io::{self, Write};

use super::{FixtureCombinationLifecycleError, checked_next};

/// Retain the attempted prefix even when an independent assertion unwinds.
/// Its count is local to this four-call test and never changes product state.
struct AttemptedPrefix {
    count: u64,
}

impl Drop for AttemptedPrefix {
    /// Report the precharged count without introducing a teardown panic.
    fn drop(&mut self) {
        let _emission = writeln!(
            io::stderr().lock(),
            "CORE_LIFECYCLE_COUNTER_PROFILE case=lifecycle_counter_success_and_exhaustion attempts={}",
            self.count
        );
    }
}

/// Check the exact successful neighbor and actual exhaustion result.
/// # Errors
/// Propagate an unexpected rejection of an admitted neighboring primitive.
/// # Panics
/// Zero, an ordinary ordinal and the maximum predecessor increment exactly;
/// maximum itself must reject rather than wrapping or reusing an identity.
#[test]
fn lifecycle_counter_success_and_exhaustion() -> Result<(), FixtureCombinationLifecycleError> {
    let mut attempted = AttemptedPrefix { count: 0 };
    (|| {
        attempted.count += 1;
        assert_eq!(checked_next(0)?.get(), 1);
        attempted.count += 1;
        assert_eq!(checked_next(7)?.get(), 8);
        attempted.count += 1;
        assert_eq!(checked_next(u64::MAX - 1)?.get(), u64::MAX);
        attempted.count += 1;
        assert_eq!(
            checked_next(u64::MAX),
            Err(FixtureCombinationLifecycleError::CounterExhausted)
        );
        Ok(())
    })()
}
