//! Compile-only attempt to bypass actual configuration admission with raw fields.
//!
//! Record the private type and any emitted private-field diagnostics separately.

#![forbid(unsafe_code)]

/// Actual producer source, without replacement types or widened visibility.
#[path = "../../src/bin/nts_ke/mod.rs"]
mod nts_ke;

/// Consumer outside the configuration owner's module subtree.
mod sibling {
    use super::nts_ke;

    /// Attempt raw construction with actual fields and a legal typed verification name.
    pub(super) fn attack() {
        let Ok(server_name) = rustls_pki_types::ServerName::try_from(String::from("Time.Example"))
        else {
            return;
        };
        let _forged = nts_ke::Config {
            peer: std::net::SocketAddr::from(([127, 0, 0, 1], 4460)),
            identity: String::from("Time.Example"),
            server_name,
        };
    }
}

/// Keep the actual permitted CLI path live alongside the forbidden raw construction.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nts_ke::entry(std::env::args_os())
}
