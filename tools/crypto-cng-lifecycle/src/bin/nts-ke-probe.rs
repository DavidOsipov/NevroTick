//! Held loopback-only ordinary-validation NTS-KE/exporter laboratory executable.
//!
//! A successful result is historical laboratory completion, with no UDP, clock,
//! service or Core effect. Native execution requires separate task admission.

#![forbid(unsafe_code)]

#[cfg(any(windows, test))]
mod nts_ke;

/// Use the actual Windows producer, or explicitly refuse another platform.
fn main() -> std::process::ExitCode {
    #[cfg(windows)]
    {
        nts_ke::entry(std::env::args_os())
    }
    #[cfg(not(windows))]
    {
        use std::io::Write;
        // Preserve unsupported-platform failure even if its bounded report fails.
        let _diagnostic_result = writeln!(
            std::io::stderr().lock(),
            "stage=admission cause=unsupported-platform"
        );
        std::process::ExitCode::FAILURE
    }
}
