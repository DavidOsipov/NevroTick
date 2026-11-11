// SPDX-License-Identifier: GPL-2.0-only
//! Compile-only attempt to construct actual staged response output without AEAD.
//! An outer privacy refusal proves no independent deeper constructor restriction.

#![forbid(unsafe_code)]

use nevrotick_engine::nts_ke_experiment as nts_ke;

/// Consumer outside the actual authenticated-response staging subtree.
mod sibling {
    use super::nts_ke;

    /// Bypass the real staged-output fields rather than inventing an auth flag.
    pub(super) fn attack() {
        fn forge(fields: nts_ke::response::ReplyFields) -> nts_ke::response::StagedReply {
            nts_ke::response::StagedReply {
                fields,
                cookies: Vec::new(),
                cookie_bytes: 0,
            }
        }
        let _constructor = forge;
    }
}

/// Retain the actual permitted bridge beside the expected private-module refusal.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())
}
