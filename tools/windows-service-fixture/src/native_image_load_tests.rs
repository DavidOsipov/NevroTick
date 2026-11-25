// SPDX-License-Identifier: GPL-2.0-only
//! Read the actual process startup policies across each fixed console entry.

use std::{error::Error, ffi::c_void, mem::size_of};

use windows_sys::Win32::{
    Foundation::{ERROR_FAILED_SERVICE_CONTROLLER_CONNECT, GetLastError, HANDLE},
    System::Threading::{
        GetCurrentProcess, GetProcessMitigationPolicy, PROCESS_MITIGATION_POLICY,
        ProcessExtensionPointDisablePolicy, ProcessImageLoadPolicy,
    },
};

use crate::{FixtureError, NativeOperation, broker_entry, engine_entry};

/// The only two four-byte mitigation layouts observed by this test.
#[derive(Clone, Copy)]
enum ObservedPolicy {
    /// SDK image-load flags.
    ImageLoad,
    /// SDK extension-point admission flags.
    ExtensionPointDisable,
}

/// Snapshot a selected one-DWORD SDK policy through the independent binding.
///
/// # Errors
/// Preserve a failed query's exact thread-local Windows error.
#[expect(
    unsafe_code,
    reason = "independent query through the pinned Windows binding"
)]
fn observed_policy_flags(policy: ObservedPolicy) -> Result<u32, Box<dyn Error>> {
    let selector: PROCESS_MITIGATION_POLICY = match policy {
        ObservedPolicy::ImageLoad => ProcessImageLoadPolicy,
        ObservedPolicy::ExtensionPointDisable => ProcessExtensionPointDisablePolicy,
    };
    let mut flags: u32 = 0;
    // SAFETY: This call returns a borrowed pseudohandle and needs no parameters.
    let process: HANDLE = unsafe { GetCurrentProcess() };
    // SAFETY: Both test-selected policy layouts are one DWORD. The local buffer
    // remains live and writable during the synchronous query.
    let result: i32 = unsafe {
        GetProcessMitigationPolicy(
            process,
            selector,
            (&raw mut flags).cast::<c_void>(),
            size_of::<u32>(),
        )
    };
    if result == 0 {
        // SAFETY: Read the original query failure immediately on this thread.
        let code: u32 = unsafe { GetLastError() };
        return Err(format!("policy {selector} readback failed (Win32 {code})").into());
    }
    Ok(flags)
}

/// Read the current process error mode directly from Kernel32.
#[expect(unsafe_code, reason = "independent process error-mode observation")]
fn observed_process_error_mode() -> u32 {
    #[link(name = "kernel32")]
    unsafe extern "system" {
        /// The SDK current-process getter returns a mode, including valid zero.
        fn GetErrorMode() -> u32;
    }
    // SAFETY: The SDK current-process query takes no parameters.
    unsafe { GetErrorMode() }
}

/// Observe policy admission through the real executable entry in a fresh process.
///
/// # Errors
/// Refuse an already protected setup, unexpected SCM result, incomplete
/// image, error-mode or extension policy readback, or an accepted duplicate entry.
fn verify_console_entry(
    enter: fn() -> Result<(), &'static FixtureError>,
) -> Result<(), Box<dyn Error>> {
    let required: u32 = 0x07;
    let before: u32 = observed_policy_flags(ObservedPolicy::ImageLoad)?;
    if before & required == required {
        return Err("fixture needs a fresh process with an incomplete image-load policy".into());
    }
    let error_mode_before: u32 = observed_process_error_mode();
    let extension_before: u32 = observed_policy_flags(ObservedPolicy::ExtensionPointDisable)?;
    let original: Result<(), &'static FixtureError> = enter();
    if !matches!(
        original,
        Err(FixtureError::Native(error))
            if error.operation() == NativeOperation::Dispatch
                && error.code() == ERROR_FAILED_SERVICE_CONTROLLER_CONNECT
    ) {
        return Err(format!("unexpected standalone SCM outcome {original:?}").into());
    }
    let after: u32 = observed_policy_flags(ObservedPolicy::ImageLoad)?;
    if after & required != required {
        return Err(format!("image-load enforcement absent after entry ({after:#010x})").into());
    }
    let error_mode_after: u32 = observed_process_error_mode();
    let expected: u32 = error_mode_before | 0x0001;
    if error_mode_after & expected != expected {
        return Err(format!(
            "incomplete process error mode ({error_mode_after:#010x}, expected {expected:#010x})"
        )
        .into());
    }
    println!(
        "service_error_mode_before={error_mode_before:#010x} service_error_mode_after={error_mode_after:#010x}"
    );
    let extension_after: u32 = observed_policy_flags(ObservedPolicy::ExtensionPointDisable)?;
    if extension_after != 1 {
        return Err(format!(
            "unexpected extension-point policy after entry ({extension_after:#010x})"
        )
        .into());
    }
    println!(
        "service_extension_points_before={extension_before:#010x} service_extension_points_after={extension_after:#010x}"
    );
    if !matches!(enter(), Err(FixtureError::AlreadyDispatched)) {
        return Err("fixture accepted reuse of the dispatcher attempt".into());
    }
    Ok(())
}

/// The engine entry establishes the selected startup policies before its SCM attempt.
///
/// # Errors
/// Preserve failed native observations and check refusals.
#[test]
#[ignore = "run alone in a fresh Windows process outside an installed service"]
fn engine_console_entry_enforces_image_load_policy() -> Result<(), Box<dyn Error>> {
    verify_console_entry(engine_entry)
}

/// The broker entry establishes the selected startup policies before its SCM attempt.
///
/// # Errors
/// Preserve failed native observations and check refusals.
#[test]
#[ignore = "run alone in a fresh Windows process outside an installed service"]
fn broker_console_entry_enforces_image_load_policy() -> Result<(), Box<dyn Error>> {
    verify_console_entry(broker_entry)
}
