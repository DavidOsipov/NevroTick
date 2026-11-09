//! Pure Windows laboratory-profile regressions, not certificate/authentication tests.
//!
//! These cases inspect the actual private helpers used by the OS call sites.
//! They invoke no chain builder, verifier, network operation or trust-store edit.
//! Their source is unexecuted until a separate dependency/test graph is admitted;
//! the retained upstream fake-root/AWS-LC test graph is not such an admission.

use std::{error::Error, mem};

use super::{
    laboratory_chain_parameters, laboratory_filetime, laboratory_policy_status,
    CERT_CHAIN_POLICY_STATUS, LABORATORY_CHAIN_FLAGS, LABORATORY_POLICY_FLAGS,
};

/// Preserve unexpected checked helper failures after a primary assertion.
type TestResult = Result<(), Box<dyn Error>>;

/// Exact integer oracles cover the epoch, a low-word carry and both upper neighbors.
/// # Errors
/// Preserve refusal of an independently supported literal input.
/// # Panics
/// Reject a wrong epoch, unit, carry, truncation or inclusive upper boundary.
#[test]
fn laboratory_filetime_matches_literal_ticks_and_words() -> TestResult {
    let cases: [(u64, u64, u32, u32); 6] = [
        (0, 116_444_736_000_000_000, 0xd53e_8000, 0x019d_b1de),
        (1, 116_444_736_010_000_000, 0xd5d7_1680, 0x019d_b1de),
        (71, 116_444_736_710_000_000, 0xff90_3d80, 0x019d_b1de),
        (72, 116_444_736_720_000_000, 0x0028_d400, 0x019d_b1df),
        (
            4_102_444_799,
            157_469_183_990_000_000,
            0x76cb_6980,
            0x022f_7163,
        ),
        (
            4_102_444_800,
            157_469_184_000_000_000,
            0x7764_0000,
            0x022f_7163,
        ),
    ];
    for (seconds, ticks, low, high) in cases {
        let result = laboratory_filetime(seconds);
        assert!(result.is_ok(), "supported laboratory date was refused");
        let time = result?;
        assert_eq!(time.dwLowDateTime, low);
        assert_eq!(time.dwHighDateTime, high);
        assert_eq!(
            (u64::from(time.dwHighDateTime) << 32) | u64::from(time.dwLowDateTime),
            ticks
        );
    }
    Ok(())
}

/// Out-of-envelope values refuse rather than clamp, wrap or substitute a date.
/// # Panics
/// Reject acceptance or diagnostic disclosure of an unsupported raw timestamp.
#[test]
fn laboratory_filetime_refuses_outside_dates_with_bounded_errors() {
    for seconds in [4_102_444_801, 6_802_270_474, u64::MAX] {
        match laboratory_filetime(seconds) {
            Err(rustls::Error::General(message)) => {
                assert!(message.len() <= 80);
                assert!(!message.contains(&seconds.to_string()));
            }
            _ => panic!("unsupported laboratory date did not produce its bounded refusal"),
        }
    }
}

/// Independent Wincrypt bit values distinguish both retrieval controls and full scope.
/// # Errors
/// Preserve an unrepresentable ABI-size observation.
/// # Panics
/// Reject leaf-only revocation, unknown suppression, fetch permission or wrong timeout.
#[test]
fn laboratory_chain_settings_match_the_call_site_contract() -> TestResult {
    // Cache end cert=1, chain URL cache=4, cumulative timeout=0x08000000,
    // revocation chain excluding root=0x40000000, revocation cache=0x80000000.
    assert_eq!(LABORATORY_CHAIN_FLAGS, 0xc800_0005);
    assert_eq!(LABORATORY_POLICY_FLAGS, 0);
    let parameters = laboratory_chain_parameters();
    assert_eq!(parameters.dwUrlRetrievalTimeout, 2_000);
    assert_eq!(
        parameters.cbSize,
        u32::try_from(mem::size_of_val(&parameters))?
    );
    Ok(())
}

/// Every policy-status field is initialized before a possible native write.
/// # Errors
/// Preserve an unexpected status-size refusal or ABI-size conversion failure.
/// # Panics
/// Reject a missing size, uninitialized output, nonzero error/index or extra pointer.
#[test]
fn laboratory_policy_status_has_complete_initialized_inputs() -> TestResult {
    let result = laboratory_policy_status();
    assert!(
        result.is_ok(),
        "laboratory policy status must be constructible"
    );
    let status = result?;
    assert_eq!(
        status.cbSize,
        u32::try_from(mem::size_of::<CERT_CHAIN_POLICY_STATUS>())?
    );
    // Wincrypt's documented layout has four 32-bit fields followed by one pointer.
    assert_eq!(
        status.cbSize,
        16 + u32::try_from(mem::size_of::<*mut std::ffi::c_void>())?
    );
    assert_eq!(status.dwError, 0);
    assert_eq!(status.lChainIndex, 0);
    assert_eq!(status.lElementIndex, 0);
    assert!(status.pvExtraPolicyStatus.is_null());
    Ok(())
}
