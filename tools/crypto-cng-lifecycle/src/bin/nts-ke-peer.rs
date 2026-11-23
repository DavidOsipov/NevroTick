//! One-connection Windows TLS/KE laboratory peer.
//!
//! Only an independently admitted guest job may execute this fixture. The
//! protected modes perform the admitted authenticated IPv4 loopback exchange.

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
