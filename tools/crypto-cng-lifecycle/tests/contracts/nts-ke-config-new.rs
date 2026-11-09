//! Compile-only attempt to call the actual private configuration constructor.
//!
//! Require a private `Config` diagnostic, not an import or target failure.

#![forbid(unsafe_code)]

/// Actual producer source, with its original visibility unchanged.
#[path = "../../src/bin/nts_ke/mod.rs"]
mod nts_ke;

/// Consumer outside the configuration owner's module subtree.
mod sibling {
    use super::nts_ke;

    /// Attempt the real private type and constructor using valid neighboring inputs.
    pub(super) fn attack() {
        let _attempt = nts_ke::Config::new("127.0.0.1:4460".into(), "Time.Example".into());
    }
}

/// Keep the actual permitted CLI path live alongside the forbidden constructor call.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nts_ke::entry(std::env::args_os())
}
