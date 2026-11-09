//! Compile-only attempt to reach the actual private directional key owner.
//! This challenges a real privacy barrier without fabricating a TLS result.

#![forbid(unsafe_code)]

/// Actual laboratory owner, with its original private child declarations.
#[path = "../../src/bin/nts_ke/mod.rs"]
mod nts_ke;

/// Caller outside the directional-key owner's subtree.
mod sibling {
    use super::nts_ke;

    /// A caller cannot expose or replace either real private directional field.
    pub(super) fn attack() {
        /// Challenge the actual field rather than inventing a raw-key API.
        fn expose(keys: &nts_ke::request::DirectionalKeys) -> &[u8; 32] {
            &keys.c2s_key
        }
        let _getter = expose;
    }
}

/// Keep the legal CLI call graph live beside the expected privacy refusal.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nts_ke::entry(std::env::args_os())
}
