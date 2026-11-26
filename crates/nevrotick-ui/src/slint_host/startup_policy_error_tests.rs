// SPDX-License-Identifier: GPL-3.0-only
//! Check typed startup-policy propagation through the real native UI wrapper.

/// The actual outer Windows error retains the typed startup cause.
///
/// # Panics
/// The cause or its bounded display disappears at the host wrapper.
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
