// SPDX-License-Identifier: GPL-3.0-only
//! Check typed startup-policy propagation through the real native UI wrapper.

/// The actual outer Windows error retains the typed startup cause.
///
/// # Panics
/// The cause or its bounded display disappears at the host wrapper.
#[cfg(windows)]
#[test]
fn host_error_retains_the_original_strict_handle_cause() {
    use super::{HostError, StrictHandlePolicyError};

    let original: StrictHandlePolicyError = StrictHandlePolicyError::Apply(87);
    let host: HostError = HostError::StrictHandlePolicy(original);
    assert_eq!(
        std::error::Error::source(&host)
            .and_then(|source| source.downcast_ref::<StrictHandlePolicyError>()),
        Some(&original)
    );
    assert_eq!(
        host.to_string(),
        "Cannot set the startup strict-handle policy (Win32 87)."
    );
}

/// The real host wrapper retains policy identity, original stage and native code.
///
/// # Panics
/// A restricted-execution cause is changed, hidden or relabeled.
#[test]
fn host_error_retains_the_original_restricted_execution_cause() {
    use super::{HostError, RestrictedExecutionPolicy, RestrictedExecutionPolicyError};
    let policies: [RestrictedExecutionPolicy; 2] = [
        RestrictedExecutionPolicy::DynamicCode,
        RestrictedExecutionPolicy::ExtensionPoints,
    ];
    for policy in policies {
        let original: RestrictedExecutionPolicyError =
            RestrictedExecutionPolicyError::Apply { policy, code: 87 };
        let host: HostError = HostError::RestrictedExecutionPolicy(original);
        assert_eq!(
            std::error::Error::source(&host)
                .and_then(|source| source.downcast_ref::<RestrictedExecutionPolicyError>()),
            Some(&original)
        );
        assert_eq!(host.to_string(), original.to_string());
    }
}
