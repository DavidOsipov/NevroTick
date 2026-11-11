//! Compile-only attempt to bypass actual configuration admission with raw fields.
//!
//! Record the private type and any emitted private-field diagnostics separately.

#![forbid(unsafe_code)]

// The actual library's outer module must reject this external access.
// Deeper constructor/field privacy may be masked by that first barrier.
use nevrotick_engine::nts_ke_experiment as nts_ke;

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

/// Keep the actual permitted Engine bridge live alongside the forbidden raw construction.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())
}
