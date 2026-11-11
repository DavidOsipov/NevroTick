//! Compile-only attempt to mutate the actual private configured peer.
//!
//! A private-type refusal alone is not independent evidence of field visibility.

#![forbid(unsafe_code)]

// The actual library's outer module must reject this external access.
// Deeper constructor/field privacy may be masked by that first barrier.
use nevrotick_engine::nts_ke_experiment as nts_ke;

/// Consumer outside the configuration owner's module subtree.
mod sibling {
    use super::nts_ke;

    /// Type-check an attempted mutation without fabricating an admitted configuration.
    pub(super) fn attack() {
        /// Attempt to replace an actual admitted peer's nonzero port with zero.
        fn mutate(config: &mut nts_ke::Config) {
            config.peer.set_port(0);
        }
        let _mutator = mutate;
    }
}

/// Keep the actual permitted Engine bridge live alongside the forbidden mutation.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())
}
