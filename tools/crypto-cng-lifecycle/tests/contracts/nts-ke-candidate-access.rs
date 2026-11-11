//! Compile-only attempt to reach and mutate the actual private transport candidate.
//!
//! The transport privacy barrier supplies no candidate move or TLS-success evidence.

#![forbid(unsafe_code)]

// The actual library's outer module must reject this external access.
// Deeper constructor/field privacy may be masked by that first barrier.
use nevrotick_engine::nts_ke_experiment as nts_ke;

/// Consumer outside the transport candidate owner's module subtree.
mod sibling {
    use super::nts_ke;

    /// Type-check a forbidden candidate path without constructing fake exporter state.
    pub(super) fn attack() {
        /// Attempt to rewrite the actual candidate's selected AEAD.
        fn mutate(candidate: &mut nts_ke::transport::InternalCandidate<'_>) {
            candidate.aead = 0;
        }
        let _mutator = mutate;
    }
}

/// Keep the actual permitted Engine bridge live alongside the forbidden candidate access.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())
}
