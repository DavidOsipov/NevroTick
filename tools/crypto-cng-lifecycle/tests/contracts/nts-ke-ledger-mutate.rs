// SPDX-License-Identifier: GPL-2.0-only
//! Compile-only attempt to reset the actual private original-context ledger.
//! No surrogate ledger or public mutation surface is introduced.

#![forbid(unsafe_code)]

use nevrotick_engine::nts_ke_experiment as nts_ke;

/// Consumer outside the actual request/budget owner's subtree.
mod sibling {
    use super::nts_ke;

    /// Attempt budget renewal and premature release of actual occupied custody.
    pub(super) fn attack() {
        fn mutate(ledger: &mut nts_ke::association::ledger::LedgerState<'_>) {
            ledger.attempts = 32;
            ledger.frames = 32;
            ledger.bytes = 131_072;
            ledger.active = None;
        }
        let _mutator = mutate;
    }
}

/// Retain the actual permitted bridge beside the expected private-module refusal.
fn main() -> std::process::ExitCode {
    sibling::attack();
    nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())
}
