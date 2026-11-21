// SPDX-License-Identifier: GPL-2.0-only
//! Read a precise Windows wall-clock coordinate between performance counter reads.

use crate::clock_coordinates::{ClockCalls, ClockCoordinates, ClockReadError, read_coordinates};

/// Win32 `FILETIME` output layout, with its separate low and high DWORDs.
#[repr(C)]
struct FileTime {
    /// Least significant 32 bits of the wall-clock coordinate.
    low: u32,
    /// Most significant 32 bits of the wall-clock coordinate.
    high: u32,
}

#[link(name = "kernel32")]
#[expect(
    unsafe_code,
    reason = "Exact Win32 profileapi.h, sysinfoapi.h and errhandlingapi.h getter ABIs"
)]
unsafe extern "system" {
    /// Read the fixed boot-time performance counter frequency.
    fn QueryPerformanceFrequency(frequency: *mut i64) -> i32;
    /// Read the current performance counter.
    fn QueryPerformanceCounter(counter: *mut i64) -> i32;
    /// Read the current precise Windows wall-clock value.
    fn GetSystemTimePreciseAsFileTime(filetime: *mut FileTime);
    /// Retrieve the most recent error on the calling thread.
    fn GetLastError() -> u32;
}

/// Synchronous Windows inputs for one observation.
struct SystemClockCalls;

impl ClockCalls for SystemClockCalls {
    /// Read the performance counter frequency and preserve a native failure code.
    #[expect(
        unsafe_code,
        reason = "Read into a local frequency output and capture the native error on FALSE"
    )]
    fn frequency(&mut self) -> Result<i64, u32> {
        let mut frequency: i64 = 0;
        // SAFETY: This exclusive local is live, aligned and writable for the
        // LARGE_INTEGER output (the signed 64-bit QuadPart) during the call.
        let status: i32 = unsafe { QueryPerformanceFrequency(&raw mut frequency) };
        if status == 0 {
            // SAFETY: GetLastError takes no arguments. Read this thread's
            // error immediately after the documented failure return.
            let code: u32 = unsafe { GetLastError() };
            return Err(code);
        }
        Ok(frequency)
    }

    /// Read the performance counter and preserve a native failure code.
    #[expect(
        unsafe_code,
        reason = "Read into a local counter output and capture the native error on FALSE"
    )]
    fn counter(&mut self) -> Result<i64, u32> {
        let mut counter: i64 = 0;
        // SAFETY: This exclusive local is live, aligned and writable for the
        // LARGE_INTEGER output (the signed 64-bit QuadPart) during the call.
        let status: i32 = unsafe { QueryPerformanceCounter(&raw mut counter) };
        if status == 0 {
            // SAFETY: GetLastError takes no arguments. Read this thread's
            // error immediately after the documented failure return.
            let code: u32 = unsafe { GetLastError() };
            return Err(code);
        }
        Ok(counter)
    }

    /// Return the exact Windows wall-clock value in 100-nanosecond units.
    #[expect(unsafe_code, reason = "Read into a local FILETIME output")]
    fn filetime(&mut self) -> u64 {
        let mut filetime: FileTime = FileTime { low: 0, high: 0 };
        // SAFETY: This exclusive local has the FILETIME layout and is live,
        // aligned and writable for both DWORD outputs during the call.
        unsafe { GetSystemTimePreciseAsFileTime(&raw mut filetime) };
        let high: u64 = u64::from(filetime.high) << u32::BITS;
        high | u64::from(filetime.low)
    }
}

/// Read the Windows wall-clock coordinate and its performance counter bracket.
///
/// This observation does not establish a calibrated UTC mapping or clock
/// continuity, and leaves the clock and its configuration untouched.
///
/// # Errors
/// Return the original Win32 code for a failed frequency or counter read, or reject an
/// invalid frequency or counter bracket.
pub fn read_clock_coordinates() -> Result<ClockCoordinates, ClockReadError> {
    read_coordinates(&mut SystemClockCalls)
}
