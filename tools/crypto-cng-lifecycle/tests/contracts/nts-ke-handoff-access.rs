//! Compile-only attempt to reach and rebind the actual private packet owner.
//! Windows-target compiler controls must reject the real private Engine module.

#![forbid(unsafe_code)]

// The actual library's outer module must reject this external access.
// Deeper constructor/field privacy may be masked by that first barrier.
use nevrotick_engine::nts_ke_experiment as nts_ke;

/// Caller outside the receiver owner's subtree.
mod sibling {
    use super::nts_ke;

    /// A different caller cannot redirect a prepared packet's original control.
    pub(super) fn attack() {
        /// Challenge the actual source type instead of a surrogate declaration.
        fn rebind<'a>(
            request: &mut nts_ke::handoff::PreparedRequest<'a>,
            control: &'a nts_ke::Control,
        ) {
            request.control = control;
        }
        let _mutator = rebind;
    }
}

/// Keep the legal public Engine bridge live beside the expected privacy refusal.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())
}
