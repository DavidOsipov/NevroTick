//! Compile-only attempt to replace the actual private original-origin expiry.
//!
//! A private-type refusal alone is not independent evidence of field visibility.

#![forbid(unsafe_code)]

// The actual library's outer module must reject this external access.
// Deeper constructor/field privacy may be masked by that first barrier.
use nevrotick_engine::nts_ke_experiment as nts_ke;

/// Consumer outside the publication-control owner's module subtree.
mod sibling {
    use super::nts_ke;

    /// Type-check an expiry rewrite without fabricating a reachable control value.
    pub(super) fn attack() {
        /// Attempt to replace the actual immutable invocation expiry.
        fn mutate(control: &mut nts_ke::Control) {
            control.expiry = std::time::Instant::now();
        }
        let _mutator = mutate;
    }
}

/// Keep the actual permitted Engine bridge live alongside the forbidden mutation.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())
}
