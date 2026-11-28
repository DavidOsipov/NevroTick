// SPDX-License-Identifier: GPL-3.0-only
//! Literal field partitions and ordered controls independent of production masks.

use super::{PolicyCalls, RestrictedExecutionPolicy, RestrictedExecutionPolicyError, enforce_with};

/// One observed synchronous operation at the fixed-policy boundary.
#[derive(Debug, Eq, PartialEq)]
enum Call {
    /// Query the selected policy word.
    Read,
    /// Request this exact selected-policy word.
    Apply(u32),
}

/// Two fixed observations and one fixed setter outcome with visible call order.
struct ScriptedCalls {
    /// The sole policy supplied for this operation.
    policy: RestrictedExecutionPolicy,
    /// The original query's word or native error.
    initial: Result<u32, u32>,
    /// The query after an accepted setter's word or native error.
    effective: Result<u32, u32>,
    /// Success or native error from the one setter.
    setter: Result<(), u32>,
    /// Actual calls, including any forbidden extra operation.
    events: Vec<Call>,
}

impl ScriptedCalls {
    /// Bind one closed policy to both reads and an initially successful setter.
    fn new(
        policy: RestrictedExecutionPolicy,
        initial: Result<u32, u32>,
        effective: Result<u32, u32>,
    ) -> Self {
        Self {
            policy,
            initial,
            effective,
            setter: Ok(()),
            events: Vec::new(),
        }
    }
}

impl PolicyCalls for ScriptedCalls {
    /// Return the immutable policy selected by this script's constructor.
    fn policy(&self) -> RestrictedExecutionPolicy {
        self.policy
    }

    /// Record the query before returning its independently supplied observation.
    fn read(&mut self) -> Result<u32, u32> {
        let first: bool = self.events.is_empty();
        self.events.push(Call::Read);
        if first { self.initial } else { self.effective }
    }

    /// Record the actual write before returning the original setter result.
    fn apply(&mut self, flags: u32) -> Result<(), u32> {
        self.events.push(Call::Apply(flags));
        self.setter
    }
}

/// Every recognized initial dynamic-code combination has a fixed stricter target.
///
/// # Panics
/// A known word refuses, audit is cleared, weakening survives or calls differ.
#[test]
fn partitions_all_sixteen_initial_dynamic_code_words() {
    let cases: [(u32, u32); 16] = [
        (0, 1),
        (1, 1),
        (2, 1),
        (3, 1),
        (4, 1),
        (5, 1),
        (6, 1),
        (7, 1),
        (8, 9),
        (9, 9),
        (10, 9),
        (11, 9),
        (12, 9),
        (13, 9),
        (14, 9),
        (15, 9),
    ];
    for (initial, requested) in cases {
        let mut calls: ScriptedCalls = ScriptedCalls::new(
            RestrictedExecutionPolicy::DynamicCode,
            Ok(initial),
            Ok(requested),
        );
        assert_eq!(enforce_with(&mut calls), Ok(()));
        if initial == 1 || initial == 9 {
            assert_eq!(calls.events, [Call::Read]);
        } else {
            assert_eq!(
                calls.events,
                [Call::Read, Call::Apply(requested), Call::Read]
            );
        }
    }
}

/// Inherited complete enforcement never reaches a setter or second query.
///
/// # Panics
/// Any accepted inherited policy attempts an unavailable later native call.
#[test]
fn inherited_enforcement_needs_only_the_original_query() {
    let cases: [(RestrictedExecutionPolicy, u32); 3] = [
        (RestrictedExecutionPolicy::DynamicCode, 1),
        (RestrictedExecutionPolicy::DynamicCode, 9),
        (RestrictedExecutionPolicy::ExtensionPoints, 1),
    ];
    for (policy, initial) in cases {
        let mut calls: ScriptedCalls = ScriptedCalls::new(policy, Ok(initial), Err(50));
        calls.setter = Err(87);
        assert_eq!(enforce_with(&mut calls), Ok(()));
        assert_eq!(calls.events, [Call::Read]);
    }
}

/// Extension-point absence requires one exact disable setting and readback.
///
/// # Panics
/// Tightening fails, the requested word differs or another operation is tried.
#[test]
fn tightens_disabled_extension_points_once() {
    let mut calls: ScriptedCalls =
        ScriptedCalls::new(RestrictedExecutionPolicy::ExtensionPoints, Ok(0), Ok(1));
    assert_eq!(enforce_with(&mut calls), Ok(()));
    assert_eq!(calls.events, [Call::Read, Call::Apply(1), Call::Read]);
}

/// The entire recognized readback partition independently distinguishes weakening.
///
/// # Panics
/// A missing field, opt-out, downgrade or lost inherited audit admits startup.
#[test]
fn partitions_all_dynamic_code_readbacks_with_and_without_inherited_audit() {
    let refused_without_audit: [u32; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15];
    let refused_with_audit: [u32; 15] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15];
    let cases: [(u32, u32, &[u32], &[u32]); 2] = [
        (0, 1, &refused_without_audit, &[1, 9]),
        (8, 9, &refused_with_audit, &[9]),
    ];
    for (initial, requested, refused, accepted) in cases {
        for observed in refused {
            let mut calls: ScriptedCalls = ScriptedCalls::new(
                RestrictedExecutionPolicy::DynamicCode,
                Ok(initial),
                Ok(*observed),
            );
            assert_eq!(
                enforce_with(&mut calls),
                Err(RestrictedExecutionPolicyError::MissingEffectiveFlags {
                    policy: RestrictedExecutionPolicy::DynamicCode,
                    requested,
                    observed: *observed,
                })
            );
            assert_eq!(
                calls.events,
                [Call::Read, Call::Apply(requested), Call::Read]
            );
        }
        for observed in accepted {
            let mut calls: ScriptedCalls = ScriptedCalls::new(
                RestrictedExecutionPolicy::DynamicCode,
                Ok(initial),
                Ok(*observed),
            );
            assert_eq!(enforce_with(&mut calls), Ok(()));
            assert_eq!(
                calls.events,
                [Call::Read, Call::Apply(requested), Call::Read]
            );
        }
    }
}

/// Setter success cannot replace effective extension-point enforcement.
///
/// # Panics
/// The recognized zero word admits startup or a second tightening is attempted.
#[test]
fn refuses_missing_effective_extension_point_disabling_without_retry() {
    let mut calls: ScriptedCalls =
        ScriptedCalls::new(RestrictedExecutionPolicy::ExtensionPoints, Ok(0), Ok(0));
    assert_eq!(
        enforce_with(&mut calls),
        Err(RestrictedExecutionPolicyError::MissingEffectiveFlags {
            policy: RestrictedExecutionPolicy::ExtensionPoints,
            requested: 1,
            observed: 0,
        })
    );
    assert_eq!(calls.events, [Call::Read, Call::Apply(1), Call::Read]);
}

/// Every reserved field refuses at its original observation for both policies.
///
/// # Panics
/// A reserved field is ignored, rewritten, attributed elsewhere or retried.
#[test]
fn refuses_each_reserved_bit_before_any_later_operation() {
    let cases: [(RestrictedExecutionPolicy, u32); 2] = [
        (RestrictedExecutionPolicy::DynamicCode, 4),
        (RestrictedExecutionPolicy::ExtensionPoints, 1),
    ];
    for (policy, first_reserved_bit) in cases {
        for bit in first_reserved_bit..32 {
            let flags: u32 = 1 | (1_u32 << bit);
            let mut initial: ScriptedCalls = ScriptedCalls::new(policy, Ok(flags), Ok(1));
            assert_eq!(
                enforce_with(&mut initial),
                Err(RestrictedExecutionPolicyError::UnexpectedInitialFlags { policy, flags })
            );
            assert_eq!(initial.events, [Call::Read]);
            let mut effective: ScriptedCalls = ScriptedCalls::new(policy, Ok(0), Ok(flags));
            assert_eq!(
                enforce_with(&mut effective),
                Err(RestrictedExecutionPolicyError::UnexpectedEffectiveFlags { policy, flags })
            );
            assert_eq!(effective.events, [Call::Read, Call::Apply(1), Call::Read]);
        }
    }
}

/// The first native failure retains its exact policy, stage and code.
///
/// # Panics
/// A refusal loses its cause, admits startup or allows a later native operation.
#[test]
fn retains_each_native_failure_for_both_policies_without_retry() {
    for policy in [
        RestrictedExecutionPolicy::DynamicCode,
        RestrictedExecutionPolicy::ExtensionPoints,
    ] {
        let mut initial: ScriptedCalls = ScriptedCalls::new(policy, Err(5), Err(50));
        initial.setter = Err(87);
        assert_eq!(
            enforce_with(&mut initial),
            Err(RestrictedExecutionPolicyError::ReadInitial { policy, code: 5 })
        );
        assert_eq!(initial.events, [Call::Read]);
        let mut setter: ScriptedCalls = ScriptedCalls::new(policy, Ok(0), Err(50));
        setter.setter = Err(87);
        assert_eq!(
            enforce_with(&mut setter),
            Err(RestrictedExecutionPolicyError::Apply { policy, code: 87 })
        );
        assert_eq!(setter.events, [Call::Read, Call::Apply(1)]);
        let mut effective: ScriptedCalls = ScriptedCalls::new(policy, Ok(0), Err(50));
        assert_eq!(
            enforce_with(&mut effective),
            Err(RestrictedExecutionPolicyError::ReadEffective { policy, code: 50 })
        );
        assert_eq!(effective.events, [Call::Read, Call::Apply(1), Call::Read]);
    }
}

/// All error stages render bounded fixed meanings with the original scalar facts.
///
/// # Panics
/// Formatting omits a stage/policy/cause or manufactures an underlying source.
#[test]
fn formats_each_typed_first_cause_without_native_text() {
    let policy: RestrictedExecutionPolicy = RestrictedExecutionPolicy::DynamicCode;
    let cases: [(RestrictedExecutionPolicyError, &str); 6] = [
        (
            RestrictedExecutionPolicyError::ReadInitial { policy, code: 5 },
            "Cannot read the startup dynamic-code policy (Win32 5).",
        ),
        (
            RestrictedExecutionPolicyError::UnexpectedInitialFlags { policy, flags: 16 },
            "Unexpected startup dynamic-code policy flags (0x00000010).",
        ),
        (
            RestrictedExecutionPolicyError::Apply { policy, code: 87 },
            "Cannot set the startup dynamic-code policy (Win32 87).",
        ),
        (
            RestrictedExecutionPolicyError::ReadEffective { policy, code: 50 },
            "Cannot verify the effective dynamic-code policy (Win32 50).",
        ),
        (
            RestrictedExecutionPolicyError::UnexpectedEffectiveFlags { policy, flags: 16 },
            "Unexpected effective dynamic-code policy flags (0x00000010).",
        ),
        (
            RestrictedExecutionPolicyError::MissingEffectiveFlags {
                policy,
                requested: 9,
                observed: 1,
            },
            "Incomplete effective dynamic-code policy (0x00000001, required 0x00000009).",
        ),
    ];
    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
        assert!(std::error::Error::source(&error).is_none());
    }
    let extension: RestrictedExecutionPolicyError = RestrictedExecutionPolicyError::Apply {
        policy: RestrictedExecutionPolicy::ExtensionPoints,
        code: 87,
    };
    assert_eq!(
        extension.to_string(),
        "Cannot set the startup extension-point policy (Win32 87)."
    );
}
