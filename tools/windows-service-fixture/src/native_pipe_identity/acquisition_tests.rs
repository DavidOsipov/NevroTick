// SPDX-License-Identifier: GPL-2.0-only
//! Exercise primary-token acquisition with scripted statistics and a live query handle.

use std::cell::Cell;
use std::num::NonZeroU32;
use std::os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle, OwnedHandle};

use windows_platform_native::PeerProcess;
use windows_sys::Win32::Foundation::{
    ERROR_ACCESS_DENIED, ERROR_INVALID_HANDLE, GetHandleInformation, HANDLE,
};
use windows_sys::Win32::Security::{TOKEN_STATISTICS, TokenImpersonation};

use super::{
    PipeIdentity, PipeIdentityError, RetainedPrimaryToken, last_error, open_primary_token,
    own_process_identity, read_statistics, sample_identity,
};

/// Next callback expected during the acquisition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AcquisitionStep {
    /// Request the original statistics.
    Original,
    /// Read identity with the original statistics.
    Identity,
    /// Request confirmation statistics after reading identity.
    Confirmation,
    /// Return without further queries.
    Complete,
}

/// Scripted confirmation response for a well-formed first sample.
#[derive(Clone, Copy, Debug)]
enum Confirmation {
    /// Matching generation and object identifier.
    Stable,
    /// Alter only the high half of the modification identifier.
    ChangedModifiedHigh,
    /// Alter only the low half of the modification identifier.
    ChangedModifiedLow,
    /// Return a native query refusal.
    Denied,
    /// Report an extent one byte shorter than the original.
    Short,
    /// Report an extent one byte longer than the original.
    Long,
    /// Report an impersonation token.
    WrongType,
}

impl Confirmation {
    /// Produce the scripted native result without modifying the kernel token.
    /// # Errors
    /// The denied case returns the simulated native refusal.
    fn read(
        self,
        mut statistics: TOKEN_STATISTICS,
        mut written: u32,
    ) -> Result<(TOKEN_STATISTICS, u32), PipeIdentityError> {
        match self {
            Self::Stable => {}
            Self::ChangedModifiedHigh => statistics.ModifiedId.HighPart ^= i32::MIN,
            Self::ChangedModifiedLow => statistics.ModifiedId.LowPart ^= 1_u32 << 31,
            Self::Denied => {
                return Err(PipeIdentityError::Native {
                    operation: "GetTokenInformation(TokenStatistics)",
                    code: ERROR_ACCESS_DENIED,
                });
            }
            Self::Short => written -= 1,
            Self::Long => written += 1,
            Self::WrongType => statistics.TokenType = TokenImpersonation,
        }
        Ok((statistics, written))
    }
}

/// Probe the original numeric handle without taking or borrowing Rust ownership.
/// # Errors
/// Preserve the Win32 error captured immediately after a failed probe.
fn check_handle(handle: HANDLE) -> Result<(), PipeIdentityError> {
    let mut flags: u32 = 0;
    #[expect(
        unsafe_code,
        reason = "observe validity of the original fixture handle"
    )]
    // SAFETY: GetHandleInformation accepts an opaque handle value and reports
    // invalid handles. The output is writable for one u32.
    let success: i32 = unsafe { GetHandleInformation(handle, &raw mut flags) };
    if success == 0 {
        return Err(last_error("GetHandleInformation"));
    }
    Ok(())
}

/// Check read order, generation admission, typed refusals and handle ownership.
/// Run this selector alone so another test cannot reuse the closed handle value.
/// # Errors
/// Preserve failure to retain the self process or query its real token.
/// # Panics
/// Panic on an incorrect read sequence, retained value, or handle lifetime.
#[test]
#[ignore = "requires the admitted offline Windows broker identity fixture"]
fn native_primary_token_acquisition_brackets_identity_and_preserves_handle_custody()
-> Result<(), Box<dyn std::error::Error>> {
    let pid: NonZeroU32 = NonZeroU32::new(std::process::id()).ok_or("fixture PID was zero")?;
    let process: PeerProcess = PeerProcess::open(pid)?;
    let expected_identity: PipeIdentity = own_process_identity()?;

    for confirmation in [
        Confirmation::Stable,
        Confirmation::ChangedModifiedHigh,
        Confirmation::ChangedModifiedLow,
        Confirmation::Denied,
        Confirmation::Short,
        Confirmation::Long,
        Confirmation::WrongType,
    ] {
        let token: OwnedHandle = open_primary_token(process.as_handle().as_raw_handle())?;
        let handle: HANDLE = token.as_raw_handle();
        check_handle(handle)?;
        let (mut original, written): (TOKEN_STATISTICS, u32) = read_statistics(token.as_handle())?;
        // Distinct nonzero halves make a wrong field or a truncated saved
        // generation observable even on a host whose native high half is zero.
        original.ModifiedId.HighPart = -0x0748_51cf;
        original.ModifiedId.LowPart = 0x3f24_8ac7;
        original.TokenId.HighPart = 0x1730_be87;
        original.TokenId.LowPart = 0xa821_06d4;

        let next: Cell<AcquisitionStep> = Cell::new(AcquisitionStep::Original);
        let acquired: Result<RetainedPrimaryToken, PipeIdentityError> =
            RetainedPrimaryToken::from_token(
                token,
                |queried: BorrowedHandle<'_>| {
                    assert!(
                        queried.as_raw_handle() == handle,
                        "statistics handle changed"
                    );
                    if next.get() == AcquisitionStep::Original {
                        next.set(AcquisitionStep::Identity);
                        return Ok((original, written));
                    }
                    assert_eq!(
                        next.replace(AcquisitionStep::Complete),
                        AcquisitionStep::Confirmation,
                        "statistics confirmation must follow identity"
                    );
                    confirmation.read(original, written)
                },
                |queried: BorrowedHandle<'_>, statistics: &TOKEN_STATISTICS| {
                    assert!(queried.as_raw_handle() == handle, "identity handle changed");
                    assert_eq!(
                        next.replace(AcquisitionStep::Confirmation),
                        AcquisitionStep::Identity,
                        "identity must follow the original statistics query"
                    );
                    assert!(
                        statistics.ModifiedId.HighPart == original.ModifiedId.HighPart
                            && statistics.ModifiedId.LowPart == original.ModifiedId.LowPart,
                        "identity must receive the original statistics"
                    );
                    sample_identity(queried, statistics)
                },
            );
        // Probe before any intervening work can open another handle.
        let handle_status: Result<(), PipeIdentityError> = check_handle(handle);
        assert_eq!(next.get(), AcquisitionStep::Complete, "{confirmation:?}");

        match (confirmation, acquired) {
            (Confirmation::Stable, Ok(retained)) => {
                handle_status?;
                assert!(retained.handle.as_raw_handle() == handle, "owner changed");
                assert!(retained.identity == expected_identity, "identity changed");
                assert!(
                    retained.generation.0 == original.ModifiedId.HighPart
                        && retained.generation.1 == original.ModifiedId.LowPart,
                    "the saved generation must contain both halves of the original ModifiedId"
                );
            }
            (
                Confirmation::ChangedModifiedHigh | Confirmation::ChangedModifiedLow,
                Err(PipeIdentityError::ChangedPrimaryToken),
            )
            | (
                Confirmation::Denied,
                Err(PipeIdentityError::Native {
                    operation: "GetTokenInformation(TokenStatistics)",
                    code: ERROR_ACCESS_DENIED,
                }),
            )
            | (
                Confirmation::Short | Confirmation::Long,
                Err(PipeIdentityError::InvalidTokenInformation),
            )
            | (Confirmation::WrongType, Err(PipeIdentityError::UnexpectedTokenType)) => {
                assert!(
                    matches!(
                        handle_status,
                        Err(PipeIdentityError::Native {
                            operation: "GetHandleInformation",
                            code: ERROR_INVALID_HANDLE,
                        })
                    ),
                    "{confirmation:?} must close the original handle"
                );
            }
            (case, _) => return Err(format!("unexpected acquisition result for {case:?}").into()),
        }
    }
    Ok(())
}
