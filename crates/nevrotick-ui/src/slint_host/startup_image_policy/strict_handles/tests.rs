// SPDX-License-Identifier: GPL-3.0-only
//! Literal policy words and ordered observations independent of production masks.

use super::super::PolicyCalls;
use super::{StrictHandlePolicyError, enforce_with};

/// One attempted synchronous call at the fixed native-policy boundary.
#[derive(Debug, Eq, PartialEq)]
enum Call {
    /// Query the original process policy.
    Read,
    /// Set this exact policy word.
    Apply(u32),
}

/// Exactly two available observations and one setter result.
struct ScriptedCalls {
    /// Word or code from the first query.
    initial: Result<u32, u32>,
    /// Word or code from the post-set query.
    effective: Result<u32, u32>,
    /// Success or original failure from the one setter.
    setter: Result<(), u32>,
    /// Observable calls, including any unexpected later attempt.
    events: Vec<Call>,
}

impl ScriptedCalls {
    /// Supply the two reads and an initially successful setter.
    fn new(initial: Result<u32, u32>, effective: Result<u32, u32>) -> Self {
        Self {
            initial,
            effective,
            setter: Ok(()),
            events: Vec::new(),
        }
    }
}

impl PolicyCalls for ScriptedCalls {
    /// Record each query and return its corresponding fixed observation.
    fn read(&mut self) -> Result<u32, u32> {
        let initial: bool = self.events.is_empty();
        self.events.push(Call::Read);
        if initial {
            self.initial
        } else {
            self.effective
        }
    }

    /// Record the actual requested word before reporting the fixed setter result.
    fn apply(&mut self, flags: u32) -> Result<(), u32> {
        self.events.push(Call::Apply(flags));
        self.setter
    }
}

/// Missing or temporary initial fields require the same matching permanent word.
///
/// # Panics
/// A valid tightening fails, sets a different word or has different call order.
#[test]
fn tightens_each_incomplete_initial_word_to_both_permanent_fields() {
    for initial in [0_u32, 1, 2] {
        let mut calls: ScriptedCalls = ScriptedCalls::new(Ok(initial), Ok(3));
        assert_eq!(enforce_with(&mut calls), Ok(()));
        assert_eq!(calls.events, [Call::Read, Call::Apply(3), Call::Read]);
    }
}

/// An inherited complete permanent word needs no further query or setting.
///
/// # Panics
/// Complete inherited enforcement refuses or attempts a setter/readback.
#[test]
fn accepts_only_the_complete_inherited_word_without_a_setter() {
    let mut calls: ScriptedCalls = ScriptedCalls::new(Ok(3), Err(123));
    calls.setter = Err(456);
    assert_eq!(enforce_with(&mut calls), Ok(()));
    assert_eq!(calls.events, [Call::Read]);
}

/// Every incomplete effective word refuses, including verifier-only exceptions.
///
/// # Panics
/// Either missing field admits startup or a weaker retry is attempted.
#[test]
fn refuses_each_incomplete_effective_word_without_retry() {
    for observed in [0_u32, 1, 2] {
        let mut calls: ScriptedCalls = ScriptedCalls::new(Ok(0), Ok(observed));
        assert_eq!(
            enforce_with(&mut calls),
            Err(StrictHandlePolicyError::MissingEffectiveFlags(observed))
        );
        assert_eq!(calls.events, [Call::Read, Call::Apply(3), Call::Read]);
    }
}

/// Each of the SDK's 30 reserved positions independently refuses at either read.
///
/// # Panics
/// A reserved bit admits startup, is rewritten or produces a later retry.
#[test]
fn refuses_every_reserved_bit_at_its_original_observation() {
    for bit in 2_u32..32 {
        let flags: u32 = 3 | (1_u32 << bit);
        let mut initial: ScriptedCalls = ScriptedCalls::new(Ok(flags), Ok(3));
        assert_eq!(
            enforce_with(&mut initial),
            Err(StrictHandlePolicyError::UnexpectedInitialFlags(flags))
        );
        assert_eq!(initial.events, [Call::Read]);
        let mut effective: ScriptedCalls = ScriptedCalls::new(Ok(0), Ok(flags));
        assert_eq!(
            enforce_with(&mut effective),
            Err(StrictHandlePolicyError::UnexpectedEffectiveFlags(flags))
        );
        assert_eq!(effective.events, [Call::Read, Call::Apply(3), Call::Read]);
    }
}

/// The first failed native call preserves its stage and exact original code.
///
/// # Panics
/// A failed call is lost, another call follows or startup is accepted.
#[test]
fn preserves_each_native_failure_and_stops_at_that_call() {
    let mut initial: ScriptedCalls = ScriptedCalls::new(Err(5), Ok(3));
    assert_eq!(
        enforce_with(&mut initial),
        Err(StrictHandlePolicyError::ReadInitial(5))
    );
    assert_eq!(initial.events, [Call::Read]);
    let mut setter: ScriptedCalls = ScriptedCalls::new(Ok(0), Ok(3));
    setter.setter = Err(87);
    assert_eq!(
        enforce_with(&mut setter),
        Err(StrictHandlePolicyError::Apply(87))
    );
    assert_eq!(setter.events, [Call::Read, Call::Apply(3)]);
    let mut effective: ScriptedCalls = ScriptedCalls::new(Ok(0), Err(50));
    assert_eq!(
        enforce_with(&mut effective),
        Err(StrictHandlePolicyError::ReadEffective(50))
    );
    assert_eq!(effective.events, [Call::Read, Call::Apply(3), Call::Read]);
}

/// All outcome stages retain a bounded useful description without native text.
///
/// # Panics
/// A message omits its fixed stage, changes meaning or manufactures a source.
#[test]
fn formats_each_typed_first_cause_without_an_untyped_source() {
    let cases: [(StrictHandlePolicyError, &str); 6] = [
        (
            StrictHandlePolicyError::ReadInitial(5),
            "Cannot read the startup strict-handle policy (Win32 5).",
        ),
        (
            StrictHandlePolicyError::UnexpectedInitialFlags(4),
            "Unexpected startup strict-handle policy flags (0x00000004).",
        ),
        (
            StrictHandlePolicyError::Apply(87),
            "Cannot set the startup strict-handle policy (Win32 87).",
        ),
        (
            StrictHandlePolicyError::ReadEffective(50),
            "Cannot verify the effective strict-handle policy (Win32 50).",
        ),
        (
            StrictHandlePolicyError::UnexpectedEffectiveFlags(4),
            "Unexpected effective strict-handle policy flags (0x00000004).",
        ),
        (
            StrictHandlePolicyError::MissingEffectiveFlags(1),
            "Incomplete effective strict-handle policy (0x00000001, required 0x00000003).",
        ),
    ];
    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
        assert!(std::error::Error::source(&error).is_none());
    }
}

/// Query strict handles independently of the production adapter's struct/selector.
///
/// The selected SDK and `windows-sys` 0.61.2 declare enum value 3 and a single
/// DWORD union. These exact Kernel32 declarations add no product Cargo feature.
///
/// # Errors
/// Preserve the independent current-process query's immediate Win32 code.
#[cfg(windows)]
#[expect(
    unsafe_code,
    reason = "Independent SDK-ABI query of the original process's strict-handle DWORD"
)]
fn independent_strict_handle_word() -> Result<u32, u32> {
    use std::{ffi::c_void, mem::size_of};

    #[link(name = "kernel32")]
    unsafe extern "system" {
        /// Borrow the current process pseudohandle; do not close it.
        fn GetCurrentProcess() -> *mut c_void;
        /// Read the selected policy into the exact writable DWORD extent.
        fn GetProcessMitigationPolicy(
            process: *mut c_void,
            policy: i32,
            buffer: *mut c_void,
            length: usize,
        ) -> i32;
        /// Read the failed query's error on this thread before intervening work.
        fn GetLastError() -> u32;
    }

    // SAFETY: This argument-free call borrows this process's pseudohandle;
    // it remains valid for the synchronous query and has no close obligation.
    let process: *mut c_void = unsafe { GetCurrentProcess() };
    let mut flags: u32 = 0;
    // SAFETY: The exact DWORD is initialized, writable and aligned, and its
    // extent and lifetime cover the synchronous query on the borrowed process.
    let status: i32 = unsafe {
        GetProcessMitigationPolicy(
            process,
            3,
            (&raw mut flags).cast::<c_void>(),
            size_of::<u32>(),
        )
    };
    if status == 0 {
        // SAFETY: This reads the failed BOOL query's error immediately.
        return Err(unsafe { GetLastError() });
    }
    Ok(flags)
}

/// Exercise the actual production wrapper in a fresh owned Windows process.
///
/// # Errors
/// A production guard or independent Win32 observation fails.
///
/// # Panics
/// The process is not fresh for tightening or the effective word is not exact.
#[cfg(windows)]
#[test]
#[ignore = "Run only this exact selector in a fresh, owned Windows test process"]
fn native_applies_and_reads_back_strict_handles_in_fresh_process()
-> Result<(), StrictHandlePolicyError> {
    let initial: u32 =
        independent_strict_handle_word().map_err(StrictHandlePolicyError::ReadInitial)?;
    assert_ne!(initial, 3, "the selected child must exercise the setter");
    super::super::enforce_strict_handles()?;
    let effective: u32 =
        independent_strict_handle_word().map_err(StrictHandlePolicyError::ReadEffective)?;
    println!("ui_strict_handles_before={initial:#010x} ui_strict_handles_after={effective:#010x}");
    assert_eq!(effective, 3);
    Ok(())
}
