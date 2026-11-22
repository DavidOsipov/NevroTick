// SPDX-License-Identifier: GPL-2.0-only
//! Compare retained process and primary-token objects in the native peer fixture.

use std::os::windows::io::{AsRawHandle, BorrowedHandle};

use windows_sys::{Win32::Foundation::CompareObjectHandles, core::BOOL};

/// Observe equality of two borrowed kernel objects. Zero supplies no match.
pub(super) fn same_kernel_object(left: BorrowedHandle<'_>, right: BorrowedHandle<'_>) -> bool {
    #[expect(
        unsafe_code,
        reason = "compare two borrowed handles by their kernel objects"
    )]
    // SAFETY: Both handles stay borrowed through the call, which neither closes
    // the handles nor retains their raw values.
    let same: BOOL = unsafe { CompareObjectHandles(left.as_raw_handle(), right.as_raw_handle()) };
    same != 0
}
