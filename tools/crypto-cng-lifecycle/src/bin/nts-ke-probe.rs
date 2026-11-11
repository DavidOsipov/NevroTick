// SPDX-License-Identifier: GPL-2.0-only
//! Live caller of the Engine-owned, controlled-loopback KE prototype.
//!
//! The Engine owns the strict producer, association and default never-send preview.
//! Its explicit protected-datagram flag selects a separate bounded source path.
//! Native execution and provider qualification remain separately admitted.

#![forbid(unsafe_code)]

/// Delegate actual arguments to the feature-selected Engine owner.
fn main() -> std::process::ExitCode {
    nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())
}
