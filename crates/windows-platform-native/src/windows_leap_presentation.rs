// SPDX-License-Identifier: GPL-2.0-only
//! Read the Windows leap-second presentation of the current process.

use std::ffi::c_void;

use crate::leap_presentation::{LeapPresentation, LeapPresentationError, classify};

/// SDK `ProcessLeapSecondInfo` value in `PROCESS_INFORMATION_CLASS`.
const PROCESS_LEAP_SECOND_INFO: i32 = 8;
/// Exact size passed to the Windows query in bytes.
const REPLY_BYTES: u32 = 8;

/// SDK `PROCESS_LEAP_SECOND_INFO` output layout with two `ULONG` words.
#[repr(C)]
struct ProcessLeapSecondInfo {
    /// Process presentation flags.
    flags: u32,
    /// Reserved output, required to be clear for supported observations.
    reserved: u32,
}

const _: [(); 8] = [(); size_of::<ProcessLeapSecondInfo>()];

#[link(name = "kernel32")]
#[expect(
    unsafe_code,
    reason = "Exact Win32 processthreadsapi.h and errhandlingapi.h query ABIs"
)]
unsafe extern "system" {
    /// Borrow the current process pseudohandle without taking ownership.
    fn GetCurrentProcess() -> *mut c_void;
    /// Fill the output buffer corresponding to the selected process information class.
    fn GetProcessInformation(
        process: *mut c_void,
        class: i32,
        information: *mut c_void,
        information_bytes: u32,
    ) -> i32;
    /// Observe the failing thread's Win32 error before another OS call.
    fn GetLastError() -> u32;
}

/// Query the current process's selected leap-second presentation once.
/// The result records a process setting without altering it or granting time authority.
///
/// # Errors
/// Returns the original native failure code or both invalid output words.
#[expect(
    unsafe_code,
    reason = "Use the current process pseudohandle for a bounded read"
)]
pub fn read_leap_presentation() -> Result<LeapPresentation, LeapPresentationError> {
    let mut information: ProcessLeapSecondInfo = ProcessLeapSecondInfo {
        flags: 0,
        reserved: 0,
    };
    // SAFETY: GetCurrentProcess has no pointer inputs and returns the current
    // process pseudohandle. The adapter neither closes nor retains it.
    let process: *mut c_void = unsafe { GetCurrentProcess() };
    // SAFETY: The pseudohandle is valid for the current process. The exclusive
    // local is aligned, writable and lives through the synchronous call. Its
    // exact C layout and byte length match ProcessLeapSecondInfo.
    let status: i32 = unsafe {
        GetProcessInformation(
            process,
            PROCESS_LEAP_SECOND_INFO,
            (&raw mut information).cast::<c_void>(),
            REPLY_BYTES,
        )
    };
    if status == 0 {
        // SAFETY: No intervening Windows call can replace this thread's error
        // from the failing query. GetLastError takes no pointer arguments.
        let code: u32 = unsafe { GetLastError() };
        return Err(LeapPresentationError::QueryFailed(code));
    }
    classify(information.flags, information.reserved)
}
