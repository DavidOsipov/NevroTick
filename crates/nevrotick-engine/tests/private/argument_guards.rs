// SPDX-License-Identifier: GPL-2.0-only
//! Direct CLI resource-boundary controls; no process, network or OS-failure simulation.

use super::*;

/// The owner admits exactly six pairs and preserves exhausted state on every failure.
/// These calls exercise the resource contract independently of the option grammar.
///
/// # Errors
/// Preserve an unexpected rejection within the legal six-pair allowance.
///
/// # Panics
/// Fail if charging skips, wraps, resets, or admits a seventh raw pair.
#[test]
fn pair_budget_admits_six_and_preserves_exhausted_failure() -> Result<(), CliError> {
    let mut budget = ArgumentBudget::for_cli();
    assert_eq!(budget.remaining_pairs, 6);
    for remaining in [5, 4, 3, 2, 1, 0] {
        budget.charge_pair()?;
        assert_eq!(budget.remaining_pairs, remaining);
    }
    for _ in 0..2 {
        assert!(matches!(budget.charge_pair(), Err(CliError::Arguments)));
        assert_eq!(budget.remaining_pairs, 0);
    }
    Ok(())
}

/// Independently literal byte neighbors preserve the whole original borrowed text.
/// Acceptance here establishes bounded text, not a recognized option or valid endpoint.
///
/// # Errors
/// Preserve an unexpected rejection of a legal text boundary.
///
/// # Panics
/// Fail if a limit is reduced, exceeded, counted as characters or the text is copied/lost.
#[test]
fn argument_text_preserves_exact_byte_boundaries_and_original_borrows() -> Result<(), CliError> {
    for (kind, below, exact, above) in [
        (ArgumentTextKind::Option, 31, 32, 33),
        (ArgumentTextKind::Value, 127, 128, 129),
    ] {
        for length in [0, below, exact] {
            let input = "x".repeat(length);
            let admitted = argument_text(OsStr::new(&input), kind)?;
            assert_eq!(admitted, input.as_str());
            assert!(std::ptr::eq(admitted.as_ptr(), input.as_ptr()));
        }
        let input = "x".repeat(above);
        assert!(matches!(
            argument_text(OsStr::new(&input), kind),
            Err(CliError::Arguments)
        ));
    }
    for (kind, exact_repetitions, above_repetitions) in [
        (ArgumentTextKind::Option, 16, 17),
        (ArgumentTextKind::Value, 64, 65),
    ] {
        let input = "é".repeat(exact_repetitions); // Each scalar occupies two UTF-8 bytes.
        let admitted = argument_text(OsStr::new(&input), kind)?;
        assert_eq!(admitted, input.as_str());
        assert!(std::ptr::eq(admitted.as_ptr(), input.as_ptr()));
        let input = "é".repeat(above_repetitions);
        assert!(matches!(
            argument_text(OsStr::new(&input), kind),
            Err(CliError::Arguments)
        ));
    }
    Ok(())
}

/// Actual Unix non-text OS strings reject in both roles even below their byte caps.
/// This constructs OS-string representations; it induces no native operation failure.
///
/// # Panics
/// Fail if UTF-8 rejection is omitted or conflated with text-length admission.
#[test]
#[cfg(unix)]
fn argument_text_rejects_unix_nontext_in_both_roles() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let input = OsString::from_vec(vec![0xff]);
    for kind in [ArgumentTextKind::Option, ArgumentTextKind::Value] {
        assert!(matches!(
            argument_text(&input, kind),
            Err(CliError::Arguments)
        ));
    }
}
