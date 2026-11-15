// SPDX-License-Identifier: GPL-2.0-only
//! Fixed Windows production-facade refusal consumer, not a provider experiment.
//!
//! Link the normal Engine and Tokio artifacts from the admitted Root graph.
//! The supervisor admits this v3 image and owns its process/output custody.
//! Exactly one closed mode selects fixed Engine arguments; caller text never
//! becomes a destination, identity, operation flag or Engine configuration.
//! Missing arguments reach admission refusal. Valid fixed loopback arguments
//! under an entered current-thread runtime reach caller-context refusal before
//! Engine preparation, worker submission or provider/transport effects.
//! The Engine's actual exit and diagnostic are preserved without substitution.

// Match the standalone launcher and CPU diagnostic's deny policy.
#![deny(
    warnings,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_unsafe,
    unused_must_use,
    let_underscore_drop,
    unused_macro_rules,
    unused_lifetimes,
    unfulfilled_lint_expectations,
    unknown_lints,
    renamed_and_removed_lints,
    improper_ctypes,
    improper_ctypes_definitions,
    unexpected_cfgs,
    missing_docs,
    unreachable_pub,
    unused_qualifications,
    clippy::all,
    clippy::pedantic,
    clippy::missing_docs_in_private_items,
    clippy::undocumented_unsafe_blocks,
    clippy::allow_attributes_without_reason,
    clippy::allow_attributes,
    clippy::multiple_unsafe_ops_per_block,
    clippy::unused_result_ok,
    clippy::string_slice,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    clippy::unreachable,
    clippy::dbg_macro,
    clippy::let_underscore_must_use,
    clippy::as_conversions,
    clippy::mem_forget,
    clippy::exit,
    rustdoc::all,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links
)]

#[cfg(not(all(windows, target_arch = "x86_64")))]
compile_error!("This production-facade consumer is exclusively for x64 Windows.");

use std::{
    env,
    ffi::{OsStr, OsString},
    io::{self, Write},
    process::ExitCode,
};

/// Fixed program atom consumed by the Engine's actual CLI entrypoint.
const PROGRAM: &str = "engine-facade-consumer";

/// Closed setup failures, distinct from Engine refusal exits one and two.
enum SetupFailure {
    /// Missing, unknown or extra consumer mode; no Engine call is permitted.
    Arguments,
    /// The current-thread runtime could not be built; no Engine call occurred.
    Runtime,
}

impl SetupFailure {
    /// Attempt one bounded fixed diagnostic and preserve its distinct setup exit.
    /// Diagnostic failure never replaces the original setup classification.
    fn report(self) -> ExitCode {
        let (stage, code) = match self {
            Self::Arguments => ("arguments", 126),
            Self::Runtime => ("runtime", 125),
        };
        let _diagnostic_result = writeln!(io::stderr().lock(), "facade-consumer-stage={stage}");
        ExitCode::from(code)
    }
}

/// Exactly two fixed calls to the normal production facade, with no raw payload.
enum Mode {
    /// Supply only a program atom, leaving required Engine arguments absent.
    Arguments,
    /// Supply valid fixed arguments while a current-thread runtime is entered.
    EnteredRuntime,
}

impl Mode {
    /// Read at most two consumer atoms and admit exactly one closed mode.
    /// No input text is decoded, logged or forwarded to the Engine.
    /// # Errors
    /// Refuse missing, unknown and extra modes before runtime or Engine work.
    fn from_arguments() -> Result<Self, SetupFailure> {
        let mut arguments = env::args_os().skip(1);
        let mode = match arguments.next().as_deref() {
            Some(value) if value == OsStr::new("arguments") => Self::Arguments,
            Some(value) if value == OsStr::new("entered-runtime") => Self::EnteredRuntime,
            _ => return Err(SetupFailure::Arguments),
        };
        if arguments.next().is_some() {
            return Err(SetupFailure::Arguments);
        }
        Ok(mode)
    }
}

/// Invoke only the production facade and preserve its original exit and stderr.
/// The entered guard remains live throughout that call, then retires before its
/// runtime. No driver is enabled, task spawned or runtime work polled here.
/// # Errors
/// Preserve runtime construction refusal before invoking the Engine.
fn run(mode: &Mode) -> Result<ExitCode, SetupFailure> {
    match mode {
        Mode::Arguments => Ok(nevrotick_engine::experimental_nts_ke_probe([
            OsString::from(PROGRAM),
        ])),
        Mode::EnteredRuntime => {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .build()
                .map_err(|_| SetupFailure::Runtime)?;
            let entered = runtime.enter();
            let code = nevrotick_engine::experimental_nts_ke_probe([
                OsString::from(PROGRAM),
                OsString::from("127.0.0.1:4460"),
                OsString::from("localhost"),
            ]);
            drop(entered);
            drop(runtime);
            Ok(code)
        }
    }
}

/// Preserve Engine exits; only consumer setup failures select 125 or 126.
fn main() -> ExitCode {
    match Mode::from_arguments().and_then(|mode| run(&mode)) {
        Ok(code) => code,
        Err(failure) => failure.report(),
    }
}
