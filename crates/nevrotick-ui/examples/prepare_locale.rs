//! Offline unapproved-review producer. Only stdin/stdout/stderr are used.
//!
//! This tool cannot activate a catalog or populate the semantic-review ledger.
//! Invoke inside the admitted credential-free, network-isolated bounded runner.
//! The caller owns an exclusive staged output and removes it on any failure;
//! stdout is not an atomic filesystem publication or an approved package asset.
#![forbid(unsafe_code)]

use std::io::{self, Read, Write};
use std::process::ExitCode;

use nevrotick_ui::localization::{self, CBOR_BYTES, JSON_BYTES};

/// Prepare the complete bounded candidate before writing any output bytes.
///
/// # Errors
/// Returns only a fixed static reason on argument, IO or technical admission
/// failure. A failed output write may leave incomplete unapproved stdout bytes;
/// the external runner must discard its exclusive temporary output.
fn prepare() -> Result<(), &'static str> {
    if std::env::args_os().nth(1).is_some() {
        return Err("This review tool accepts no arguments or file paths.");
    }
    let maximum = JSON_BYTES
        .checked_add(1)
        .ok_or("The fixed source bound is invalid.")?;
    let read_limit = u64::try_from(maximum).map_err(|_| "The fixed source bound is invalid.")?;
    let mut retained = Vec::with_capacity(maximum);
    io::stdin()
        .lock()
        .take(read_limit)
        .read_to_end(&mut retained)
        .map_err(|_| "The retained source could not be read.")?;
    if retained.len() > JSON_BYTES {
        return Err("The retained source exceeds the fixed review limit.");
    }
    let candidate = localization::prepare_unapproved_json(&retained)
        .map_err(|_| "The interface catalog could not be technically admitted.")?;
    let bytes = candidate.bytes_for_review();
    if bytes.len() > CBOR_BYTES {
        return Err("The unapproved output exceeds the fixed review limit.");
    }
    io::stdout()
        .lock()
        .write_all(bytes)
        .map_err(|_| "The unapproved candidate output could not be completed.")?;
    Ok(())
}

/// Emit only a fixed failure reason; never echo contributor data or IO paths.
fn main() -> ExitCode {
    match prepare() {
        Ok(()) => ExitCode::SUCCESS,
        Err(reason) => {
            eprintln!("{reason}");
            ExitCode::FAILURE
        }
    }
}
