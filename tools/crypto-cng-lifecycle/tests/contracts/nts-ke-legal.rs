//! Compile-only legal caller of the actual Windows laboratory CLI module.
//!
//! Check this fixture for the admitted Windows target; never execute it.

#![forbid(unsafe_code)]

/// Actual producer source, with its original private child modules unchanged.
#[path = "../../src/bin/nts_ke/mod.rs"]
mod nts_ke;

/// Type-check the actual CLI entry and complete producer call graph without execution.
fn main() -> std::process::ExitCode {
    nts_ke::entry(std::env::args_os())
}
