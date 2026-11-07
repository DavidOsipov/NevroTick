// SPDX-License-Identifier: GPL-2.0-only
//! Fixed inert S0.2 engine-role composition; no console service fallback.

use std::process::ExitCode;

/// Enter the fixed laboratory role; all non-Windows invocations fail.
fn main() -> ExitCode {
    if nevrotick_windows_service_fixture::engine_entry().is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
