//! Compile-only attempt to reach and mutate the actual private transport candidate.
//!
//! The transport privacy barrier supplies no candidate move or TLS-success evidence.

#![forbid(unsafe_code)]

/// Actual producer source, including its unchanged private transport child.
#[path = "../../src/bin/nts_ke/mod.rs"]
mod nts_ke;

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

/// Keep the actual permitted CLI path live alongside the forbidden candidate access.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nts_ke::entry(std::env::args_os())
}
