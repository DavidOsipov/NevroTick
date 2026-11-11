// SPDX-License-Identifier: GPL-2.0-only
//! Live caller of the Engine-owned, controlled-loopback KE prototype.
//!
//! The Engine owns the sole strict producer, association and never-send consumer.
//! Native execution and provider qualification remain separately admitted.

#![forbid(unsafe_code)]

/// Delegate actual arguments to the feature-selected Engine owner.
fn main() -> std::process::ExitCode {
    nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())
}
