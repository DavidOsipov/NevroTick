// SPDX-License-Identifier: GPL-2.0-only
//! Real GNU executable entry points reject service emulation without OS effects.
#![cfg(not(windows))]

use std::{error::Error, process::Command};

use nevrotick_windows_service_fixture::FixtureError;

/// Test both typed public entries and their actually executed GNU binaries.
///
/// # Panics
/// Fails if either GNU entry succeeds or emits a console service imitation.
///
/// # Errors
/// Propagate actual executable launch failure.
#[test]
fn gnu_entries_fail_windows_only_without_a_console_service_fallback() -> Result<(), Box<dyn Error>>
{
    assert!(matches!(
        nevrotick_windows_service_fixture::engine_entry(),
        Err(FixtureError::WindowsOnly)
    ));
    assert!(matches!(
        nevrotick_windows_service_fixture::broker_entry(),
        Err(FixtureError::WindowsOnly)
    ));
    for executable in [
        env!("CARGO_BIN_EXE_nevrotick-lab-s02-engine"),
        env!("CARGO_BIN_EXE_nevrotick-lab-s02-broker"),
    ] {
        let result = Command::new(executable).output()?;
        assert_eq!(result.status.code(), Some(1));
        assert_eq!(result.stdout.as_slice(), b"");
        assert_eq!(result.stderr.as_slice(), b"");
    }
    Ok(())
}
