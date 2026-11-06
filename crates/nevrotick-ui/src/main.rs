//! Windows fixture observer entry; backend/service/clock effects stay unavailable.
//! Effective launch-token and activation qualification remain open.
#![cfg_attr(windows, windows_subsystem = "windows")]

/// Launch the fixture window, propagating startup and lifetime errors.
///
/// # Errors
/// Returns the native adapter's startup, control or message-loop failure after
/// releasing its owned resources.
#[cfg(windows)]
fn main() -> Result<(), nevrotick_ui::slint_host::HostError> {
    nevrotick_ui::slint_host::run_native()
}

/// Reject execution on unsupported hosts; portable model tests remain available.
#[cfg(not(windows))]
fn main() -> std::process::ExitCode {
    eprintln!("UI-M0 requires Windows; run the portable model tests on this host.");
    std::process::ExitCode::FAILURE
}
