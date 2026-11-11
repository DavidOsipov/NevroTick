//! Compile-only attempt to call the actual private configuration constructor.
//!
//! Require the actual private Engine module diagnostic, not a missing import
//! or target failure.

#![forbid(unsafe_code)]

// The actual library's outer module must reject this external access.
// Deeper constructor/field privacy may be masked by that first barrier.
use nevrotick_engine::nts_ke_experiment as nts_ke;

/// Consumer outside the configuration owner's module subtree.
mod sibling {
    use super::nts_ke;

    /// Attempt the real private type and constructor using valid neighboring inputs.
    pub(super) fn attack() {
        let _attempt = nts_ke::Config::new("127.0.0.1:4460".into(), "Time.Example".into());
    }
}

/// Keep the actual permitted Engine bridge live alongside the forbidden constructor call.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())
}
