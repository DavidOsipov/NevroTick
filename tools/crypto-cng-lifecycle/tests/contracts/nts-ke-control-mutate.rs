//! Compile-only attempt to replace the actual private original-origin expiry.
//!
//! A private-type refusal alone is not independent evidence of field visibility.

#![forbid(unsafe_code)]

/// Actual producer source, with its original private representation unchanged.
#[path = "../../src/bin/nts_ke/mod.rs"]
mod nts_ke;

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

/// Keep the actual permitted CLI path live alongside the forbidden mutation.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nts_ke::entry(std::env::args_os())
}
