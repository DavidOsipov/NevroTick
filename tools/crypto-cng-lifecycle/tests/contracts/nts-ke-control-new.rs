//! Compile-only attempt to call the actual private publication-control constructor.
//!
//! Require the actual private Engine module diagnostic, not a missing import
//! or target failure.

#![forbid(unsafe_code)]

// The actual library's outer module must reject this external access.
// Deeper constructor/field privacy may be masked by that first barrier.
use nevrotick_engine::nts_ke_experiment as nts_ke;

/// Consumer outside the publication-control owner's module subtree.
mod sibling {
    use super::nts_ke;

    /// Attempt to create the actual private deadline/cancellation owner.
    pub(super) fn attack() {
        let _attempt = nts_ke::Control::new(std::time::Instant::now());
    }
}

/// Keep the actual permitted Engine bridge live alongside the forbidden constructor call.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())
}
