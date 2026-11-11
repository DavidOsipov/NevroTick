//! Compile-only attempt to reach the actual private directional key owner.
//! This challenges a real privacy barrier without fabricating a TLS result.

#![forbid(unsafe_code)]

// The actual library's outer module must reject this external access.
// Deeper constructor/field privacy may be masked by that first barrier.
use nevrotick_engine::nts_ke_experiment as nts_ke;

/// Caller outside the directional-key owner's subtree.
mod sibling {
    use super::nts_ke;

    /// A caller cannot expose or replace either real private directional field.
    pub(super) fn attack() {
        /// Challenge the actual field rather than inventing a raw-key API.
        fn expose(keys: &nts_ke::request::DirectionalKeys) -> &[u8; 32] {
            &keys.c2s_key
        }
        fn expose_response(keys: &nts_ke::request::DirectionalKeys) -> &[u8; 32] {
            &keys.s2c_key
        }
        let _getter = expose;
        let _response_getter = expose_response;
    }
}

/// Keep the legal public Engine bridge live beside the expected privacy refusal.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())
}
