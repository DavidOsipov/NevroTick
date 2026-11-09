//! Compile-only attempt to call the actual private publication-control constructor.
//!
//! Require a private `Control` diagnostic, not an import or target failure.

#![forbid(unsafe_code)]

/// Actual producer source, with its original visibility unchanged.
#[path = "../../src/bin/nts_ke/mod.rs"]
mod nts_ke;

/// Consumer outside the publication-control owner's module subtree.
mod sibling {
    use super::nts_ke;

    /// Attempt to create the actual private deadline/cancellation owner.
    pub(super) fn attack() {
        let _attempt = nts_ke::Control::new(std::time::Instant::now());
    }
}

/// Keep the actual permitted CLI path live alongside the forbidden constructor call.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nts_ke::entry(std::env::args_os())
}
