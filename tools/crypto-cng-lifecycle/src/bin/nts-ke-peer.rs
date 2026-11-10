//! One-connection Windows TLS/KE laboratory peer; not a product time server.
//!
//! Only an independently admitted guest job may execute this fixture. No trust
//! store, clock, service, UDP or external address is changed by this executable.

#![forbid(unsafe_code)]

#[cfg(any(windows, test))]
mod nts_ke_peer;

/// Run the fixed Windows peer or refuse another platform without file effects.
fn main() -> std::process::ExitCode {
    #[cfg(windows)]
    {
        nts_ke_peer::entry(std::env::args_os())
    }
    #[cfg(not(windows))]
    {
        use std::io::Write;
        let _diagnostic_result = writeln!(
            std::io::stderr().lock(),
            "stage=admission cause=unsupported-platform cleanup_cause=none"
        );
        std::process::ExitCode::FAILURE
    }
}
