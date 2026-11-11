// SPDX-License-Identifier: GPL-2.0-only
//! Compile-only attempt to forge the actual private same-owner request ticket.
//! The outer privacy refusal does not independently prove deeper field privacy.

#![forbid(unsafe_code)]

use nevrotick_engine::nts_ke_experiment as nts_ke;

/// Consumer outside the actual ledger owner's subtree.
mod sibling {
    use super::nts_ke;

    /// Challenge the real ticket fields without fabricating a TLS producer.
    pub(super) fn attack() {
        fn forge(control: &nts_ke::Control) -> nts_ke::association::ledger::Ticket<'_> {
            nts_ke::association::ledger::Ticket { control, serial: 1 }
        }
        let _constructor = forge;
    }
}

/// Retain the actual permitted bridge beside the expected private-module refusal.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())
}
