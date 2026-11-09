//! Compile-only attempt to reach and rebind the actual private packet owner.
//! Windows-target compiler controls must reject its real private module/fields.

#![forbid(unsafe_code)]

/// Actual laboratory owner, including the newly integrated private receiver.
#[path = "../../src/bin/nts_ke/mod.rs"]
mod nts_ke;

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

/// Keep the legal CLI call graph live beside the expected privacy refusal.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nts_ke::entry(std::env::args_os())
}
