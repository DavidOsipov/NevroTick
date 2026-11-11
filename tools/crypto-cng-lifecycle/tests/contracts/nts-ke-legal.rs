//! Compile-only legal external caller of the Engine-owned KE prototype.
//!
//! Require the admitted feature and Windows target; never execute this fixture.

#![forbid(unsafe_code)]

/// Type-check the actual public bridge without importing private source modules.
fn main() -> std::process::ExitCode {
    nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())
}
