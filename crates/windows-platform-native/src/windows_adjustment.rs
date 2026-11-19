// SPDX-License-Identifier: GPL-2.0-only
//! Read the precise adjustment state of the local Windows wall clock.

#[link(name = "mincore")]
#[expect(unsafe_code, reason = "Exact Win32 sysinfoapi.h getter ABI")]
unsafe extern "system" {
    fn GetSystemTimeAdjustmentPrecise(
        adjustment: *mut u64,
        increment: *mut u64,
        adjustment_disabled: *mut i32,
    ) -> i32;
}

#[link(name = "kernel32")]
#[expect(unsafe_code, reason = "Exact Win32 errhandlingapi.h last-error ABI")]
unsafe extern "system" {
    fn GetLastError() -> u32;
}

/// Exact raw adjustment state returned by one successful Windows read.
///
/// The observations confer no clock ownership or continuity guarantee.
#[must_use = "retain the actual clock adjustment observation"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClockAdjustment {
    /// Raw adjusted clock update value returned by Windows.
    adjustment: u64,
    /// Raw clock update increment returned by the same Windows read.
    increment: u64,
    /// Whether Windows reported periodic programmed adjustment disabled.
    periodic_adjustment_disabled: bool,
}

impl ClockAdjustment {
    /// Return the observed raw adjusted clock update value.
    #[must_use]
    pub const fn adjustment(&self) -> u64 {
        self.adjustment
    }

    /// Return the observed raw clock update increment.
    #[must_use]
    pub const fn increment(&self) -> u64 {
        self.increment
    }

    /// Return whether Windows reported periodic programmed adjustment disabled.
    #[must_use]
    pub const fn periodic_adjustment_disabled(&self) -> bool {
        self.periodic_adjustment_disabled
    }
}

/// Read the local Windows adjustment state once, without changing it.
///
/// # Errors
/// Return the original current-thread Win32 error on native failure, including zero.
#[expect(
    unsafe_code,
    reason = "Read into writable local outputs and preserve the original failure code"
)]
pub fn read_clock_adjustment() -> Result<ClockAdjustment, u32> {
    let mut adjustment: u64 = 0;
    let mut increment: u64 = 0;
    let mut adjustment_disabled: i32 = 0;
    // SAFETY: The three distinct locals are live, aligned, writable
    // DWORD64, DWORD64 and BOOL output slots for this synchronous call.
    let status: i32 = unsafe {
        GetSystemTimeAdjustmentPrecise(
            &raw mut adjustment,
            &raw mut increment,
            &raw mut adjustment_disabled,
        )
    };
    if status == 0 {
        // SAFETY: GetLastError has no arguments and reads this thread's error.
        // Observe it immediately on FALSE with no intervening Windows call.
        let code: u32 = unsafe { GetLastError() };
        return Err(code);
    }
    Ok(ClockAdjustment {
        adjustment,
        increment,
        periodic_adjustment_disabled: adjustment_disabled != 0,
    })
}
