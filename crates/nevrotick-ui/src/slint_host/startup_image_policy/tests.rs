// SPDX-License-Identifier: GPL-3.0-only
//! Independent flag and call-order expectations for the owning startup guard.

use super::{ImageLoadPolicyError, PolicyCalls, enforce_with};

/// One call observed at the scripted native boundary.
#[derive(Debug, Eq, PartialEq)]
enum Call {
    /// Read the current policy flags.
    Read,
    /// Request this exact policy word.
    Apply(u32),
}

/// Two bounded observations and one setter result, captured in call order.
struct ScriptedCalls {
    /// Current flags or failure before any write.
    initial: Result<u32, u32>,
    /// Effective flags or failure after the setter returns successfully.
    effective: Result<u32, u32>,
    /// Native return for the single expected setter call.
    setter: Result<(), u32>,
    /// Exact effects and queries attempted by the startup guard.
    events: Vec<Call>,
}

impl ScriptedCalls {
    /// Start with the given reads and an accepted setter.
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
    /// Take the initial result on the first call and the effective result later.
    fn read(&mut self) -> Result<u32, u32> {
        let is_initial: bool = self.events.is_empty();
        self.events.push(Call::Read);
        if is_initial {
            self.initial
        } else {
            self.effective
        }
    }

    /// Record the actual request before returning the declared setter result.
    fn apply(&mut self, flags: u32) -> Result<(), u32> {
        self.events.push(Call::Apply(flags));
        self.setter
    }
}

/// Add exactly the required three bits and retain both inherited audit bits.
///
/// # Panics
/// The requested flags or call order differ.
#[test]
fn requests_all_enforcement_and_preserves_inherited_audit() {
    let mut calls: ScriptedCalls = ScriptedCalls::new(Ok(0x18), Ok(0x1f));
    assert_eq!(enforce_with(&mut calls), Ok(()));
    assert_eq!(calls.events, [Call::Read, Call::Apply(0x1f), Call::Read]);
}

/// An already enforced process needs only the successful initial observation.
///
/// # Panics
/// The already enforced state is rejected or the setter is called.
#[test]
fn recognizes_existing_enforcement_without_a_setter() {
    let mut calls: ScriptedCalls = ScriptedCalls::new(Ok(0x17), Err(123));
    assert_eq!(enforce_with(&mut calls), Ok(()));
    assert_eq!(calls.events, [Call::Read]);
}

/// Accept all required flags from a zero initial word, including extra audit.
///
/// # Panics
/// The effective state is rejected or the requested flags differ.
#[test]
fn accepts_enforcement_with_additional_recognized_effective_flags() {
    let mut calls: ScriptedCalls = ScriptedCalls::new(Ok(0), Ok(0x0f));
    assert_eq!(enforce_with(&mut calls), Ok(()));
    assert_eq!(calls.events, [Call::Read, Call::Apply(7), Call::Read]);
}

/// Each missing enforcement bit and audit-only readback prevent admission.
///
/// # Panics
/// Any incomplete state admits startup or reports a different call order.
#[test]
fn refuses_each_missing_effective_enforcement_bit_and_audit_only() {
    for observed in [0x06_u32, 0x05, 0x03, 0x18] {
        let mut calls: ScriptedCalls = ScriptedCalls::new(Ok(0), Ok(observed));
        assert_eq!(
            enforce_with(&mut calls),
            Err(ImageLoadPolicyError::MissingEffectiveFlags {
                requested: 7,
                observed
            })
        );
        assert_eq!(calls.events, [Call::Read, Call::Apply(7), Call::Read]);
    }
}

/// Loss of a recognized inherited bit is visible even with enforcement active.
///
/// # Panics
/// Discarded audit bits admit startup or the request drops inherited bits.
#[test]
fn refuses_lost_inherited_audit_bits() {
    let mut calls: ScriptedCalls = ScriptedCalls::new(Ok(0x18), Ok(0x07));
    assert_eq!(
        enforce_with(&mut calls),
        Err(ImageLoadPolicyError::MissingEffectiveFlags {
            requested: 0x1f,
            observed: 7
        })
    );
    assert_eq!(calls.events, [Call::Read, Call::Apply(0x1f), Call::Read]);
}

/// Refuse reserved bits at either observation, never rewriting unknown input.
///
/// # Panics
/// Either unrecognized policy word admits startup or the call order differs.
#[test]
fn rejects_unknown_flags_on_either_read() {
    let mut initial: ScriptedCalls = ScriptedCalls::new(Ok(0x27), Ok(0x07));
    assert_eq!(
        enforce_with(&mut initial),
        Err(ImageLoadPolicyError::UnexpectedInitialFlags(0x27))
    );
    assert_eq!(initial.events, [Call::Read]);

    let mut effective: ScriptedCalls = ScriptedCalls::new(Ok(0), Ok(0x27));
    assert_eq!(
        enforce_with(&mut effective),
        Err(ImageLoadPolicyError::UnexpectedEffectiveFlags(0x27))
    );
    assert_eq!(effective.events, [Call::Read, Call::Apply(7), Call::Read]);
}

/// The native error and attempted calls identify the exact failing stage.
///
/// # Panics
/// Any native error loses its cause, reaches a later stage or admits startup.
#[test]
fn returns_each_native_failure_at_its_own_stage() {
    let mut initial: ScriptedCalls = ScriptedCalls::new(Err(5), Ok(7));
    assert_eq!(
        enforce_with(&mut initial),
        Err(ImageLoadPolicyError::ReadInitial(5))
    );
    assert_eq!(initial.events, [Call::Read]);

    let mut setter: ScriptedCalls = ScriptedCalls::new(Ok(0), Ok(7));
    setter.setter = Err(87);
    assert_eq!(
        enforce_with(&mut setter),
        Err(ImageLoadPolicyError::Apply(87))
    );
    assert_eq!(setter.events, [Call::Read, Call::Apply(7)]);

    let mut effective: ScriptedCalls = ScriptedCalls::new(Ok(0), Err(50));
    assert_eq!(
        enforce_with(&mut effective),
        Err(ImageLoadPolicyError::ReadEffective(50))
    );
    assert_eq!(effective.events, [Call::Read, Call::Apply(7), Call::Read]);
}

/// Observe the before and after state on Windows without making a UI window.
///
/// # Errors
/// A native read or the production guard fails.
///
/// # Panics
/// The setter would be skipped or readback lacks required or inherited flags.
#[cfg(windows)]
#[test]
#[ignore = "Run only this exact selector in a fresh, owned Windows test process"]
fn native_applies_and_reads_back_image_policy_in_fresh_process() -> Result<(), ImageLoadPolicyError>
{
    use super::{enforce, windows::SystemPolicyCalls};

    let mut calls: SystemPolicyCalls = SystemPolicyCalls;
    let initial: u32 = calls.read().map_err(ImageLoadPolicyError::ReadInitial)?;
    assert_ne!(
        initial & 7,
        7,
        "the host must leave at least one enforcement bit for the setter"
    );
    enforce()?;
    let effective: u32 = calls.read().map_err(ImageLoadPolicyError::ReadEffective)?;
    assert_eq!(effective & 7, 7);
    assert_eq!(effective & initial, initial);
    Ok(())
}
