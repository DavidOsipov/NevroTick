// SPDX-License-Identifier: GPL-2.0-only
//! Read the actual process startup policies across each fixed console entry.

use std::{
    error::Error,
    ffi::{OsStr, OsString, c_void},
    fs::{File, OpenOptions},
    mem::size_of,
    os::windows::{
        ffi::{OsStrExt, OsStringExt},
        fs::OpenOptionsExt,
        io::AsRawHandle,
    },
    path::{Path, PathBuf},
};

use windows_platform::{
    diagnostics::{DecodedRecord, DiagnosticSnapshot, LifecycleObservationClaim},
    service::ServiceRole,
};
use windows_sys::Win32::{
    Foundation::{
        ERROR_FAILED_SERVICE_CONTROLLER_CONNECT, ERROR_FILE_NOT_FOUND, ERROR_MOD_NOT_FOUND,
        GetLastError, HANDLE, HMODULE,
    },
    Storage::FileSystem::{
        FILE_NAME_NORMALIZED, FILE_SHARE_READ, GetFinalPathNameByHandleW, VOLUME_NAME_NT,
    },
    System::Threading::{
        GetCurrentProcess, GetProcessMitigationPolicy, PROCESS_MITIGATION_POLICY,
        ProcessExtensionPointDisablePolicy, ProcessImageLoadPolicy, ProcessStrictHandleCheckPolicy,
    },
};

use crate::{
    DispatchDiagnostic, DispatchFailureDetail, FixtureError, NativeOperation, broker_entry,
    engine_entry, retained_diagnostics, retained_dispatch_diagnostic,
    retained_dispatch_diagnostics,
};

/// The three four-byte mitigation layouts observed by this test.
#[derive(Clone, Copy)]
enum ObservedPolicy {
    /// SDK image-load flags.
    ImageLoad,
    /// SDK extension-point admission flags.
    ExtensionPointDisable,
    /// SDK invalid-handle exception and permanent-enable flags.
    StrictHandleCheck,
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
        ObservedPolicy::StrictHandleCheck => ProcessStrictHandleCheckPolicy,
    };
    let mut flags: u32 = 0;
    // SAFETY: This call returns a borrowed pseudohandle and needs no parameters.
    let process: HANDLE = unsafe { GetCurrentProcess() };
    // SAFETY: Each test-selected policy layout is one DWORD. The local buffer
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
/// image, error-mode, extension or strict-handle readback, or an accepted duplicate entry.
///
/// # Panics
/// Fails if the required diagnostic contradicts the original dispatcher refusal.
fn verify_console_entry(
    enter: fn() -> Result<(), &'static FixtureError>,
    expected_role: ServiceRole,
    expected_role_code: u8,
) -> Result<(), Box<dyn Error>> {
    let required: u32 = 0x07;
    let before: u32 = observed_policy_flags(ObservedPolicy::ImageLoad)?;
    if before & required == required {
        return Err("fixture needs a fresh process with an incomplete image-load policy".into());
    }
    assert!(retained_dispatch_diagnostics().is_none());
    assert!(retained_dispatch_diagnostic().is_none());
    let error_mode_before: u32 = observed_process_error_mode();
    let extension_before: u32 = observed_policy_flags(ObservedPolicy::ExtensionPointDisable)?;
    let strict_handles_before: u32 = observed_policy_flags(ObservedPolicy::StrictHandleCheck)?;
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
    let strict_handles_after: u32 = observed_policy_flags(ObservedPolicy::StrictHandleCheck)?;
    if strict_handles_after != 3 {
        return Err(format!(
            "incomplete permanent strict-handle policy after entry ({strict_handles_after:#010x})"
        )
        .into());
    }
    println!(
        "service_strict_handles_before={strict_handles_before:#010x} service_strict_handles_after={strict_handles_after:#010x}"
    );
    let dispatch_diagnostic: &DispatchDiagnostic = retained_dispatch_diagnostic()
        .ok_or("missing diagnostic from the original dispatcher refusal")?;
    assert_eq!(dispatch_diagnostic.role(), expected_role);
    assert!(matches!(
        dispatch_diagnostic.detail(),
        DispatchFailureDetail::Native(error)
            if error.operation() == NativeOperation::Dispatch && error.code() == 1063
    ));
    let diagnostic: &DiagnosticSnapshot = dispatch_diagnostic.snapshot();
    assert!(retained_dispatch_diagnostics().is_some_and(|legacy| std::ptr::eq(legacy, diagnostic)));
    let raw: &[u8; 128] = diagnostic
        .required()
        .history()
        .records()
        .next()
        .ok_or("missing required record")?;
    let decoded: DecodedRecord = DecodedRecord::decode(raw).map_err(|_| "invalid diagnostic")?;
    assert_eq!(decoded.component().code(), expected_role_code);
    assert_eq!((decoded.event().id(), decoded.event().cause_code()), (4, 9));
    assert_eq!(decoded.sequence(), 1);
    assert_eq!(
        decoded.observation(),
        LifecycleObservationClaim::Unavailable
    );
    assert_eq!(diagnostic.required().history().len(), 1);
    assert_eq!(diagnostic.optional().history().len(), 0);
    assert!(retained_diagnostics().is_none());
    if !matches!(enter(), Err(FixtureError::AlreadyDispatched)) {
        return Err("fixture accepted reuse of the dispatcher attempt".into());
    }
    assert!(retained_dispatch_diagnostics().is_some_and(|after| std::ptr::eq(after, diagnostic)));
    assert!(
        retained_dispatch_diagnostic()
            .is_some_and(|after| std::ptr::eq(after, dispatch_diagnostic))
    );
    Ok(())
}

/// The engine entry establishes the selected startup policies before its SCM attempt.
///
/// # Errors
/// Preserve failed native observations and check refusals.
#[test]
#[ignore = "run alone in a fresh Windows process outside an installed service"]
fn engine_console_entry_enforces_image_load_policy() -> Result<(), Box<dyn Error>> {
    verify_console_entry(engine_entry, ServiceRole::Engine, 1)
}

/// The broker entry establishes the selected startup policies before its SCM attempt.
///
/// # Errors
/// Preserve failed native observations and check refusals.
#[test]
#[ignore = "run alone in a fresh Windows process outside an installed service"]
fn broker_console_entry_enforces_image_load_policy() -> Result<(), Box<dyn Error>> {
    verify_console_entry(broker_entry, ServiceRole::ClockBroker, 2)
}

/// The harness stages these resource-only files under distinct names and paths.
const APPLICATION_CANARY: &str = "nv-application-search-canary.dll";
/// The harness starts the process in a private directory distinct from its image directory.
const WORKING_CANARY: &str = "nv-working-search-canary.dll";
/// A supported System32 resource lookup that must start outside the loaded module list.
const SYSTEM_SAMPLE: &str = "version.dll";
/// Bound all UTF-16 path input and readback in the private fixture.
const PATH_LIMIT: usize = 1024;
/// `LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE` makes a read-only data mapping.
/// No `LOAD_LIBRARY_SEARCH_*` flag may bypass the process default under test.
const DATAFILE_EXCLUSIVE: u32 = 0x40;

#[expect(
    unsafe_code,
    reason = "fixed Kernel32 queries used only by the Windows test image"
)]
#[link(name = "kernel32")]
unsafe extern "system" {
    /// Map a fixed file as data. The only caller passes `DATAFILE_EXCLUSIVE`.
    fn LoadLibraryExW(name: *const u16, reserved: HANDLE, flags: u32) -> HMODULE;
    /// Observe a previously executable-loaded module without acquiring its reference.
    fn GetModuleHandleW(name: *const u16) -> HMODULE;
    /// Release the actual reference returned by `LoadLibraryExW`.
    fn FreeLibrary(module: HMODULE) -> i32;
    /// Observe the device path backing the specific address in the local process.
    fn K32GetMappedFileNameW(
        process: HANDLE,
        address: *mut c_void,
        path: *mut u16,
        capacity: u32,
    ) -> u32;
    /// Read the system directory selected by this Windows process.
    fn GetSystemDirectoryW(path: *mut u16, capacity: u32) -> u32;
}

/// Keep the exact file open read-only, denying write and delete across both lookups.
struct HeldSearchFile {
    /// Prevent replacement while the original fixture path is in use.
    _file: File,
    /// Absolute path explicitly passed to the data loader for the healthy neighbor.
    path: PathBuf,
    /// Native path queried from the held file, independent of the loader result.
    device_path: OsString,
}

/// Encode the complete nonempty path or fixed basename for synchronous Windows queries.
///
/// # Errors
/// Refuse an embedded terminator, empty input or path outside the fixed buffer envelope.
fn wide_path(path: &OsStr) -> Result<Vec<u16>, Box<dyn Error>> {
    let mut wide: Vec<u16> = path.encode_wide().take(PATH_LIMIT).collect();
    if wide.is_empty() || wide.len() == PATH_LIMIT || wide.contains(&0) {
        return Err("unexpected DLL search test path extent".into());
    }
    wide.push(0);
    Ok(wide)
}

/// Read a Windows path whose reported count excludes its terminator.
///
/// # Errors
/// Refuse a failed, truncated or contradictory path observation.
fn observed_path(buffer: &[u16], count: u32) -> Result<OsString, Box<dyn Error>> {
    let count: usize = usize::try_from(count)?;
    if count == 0 || buffer.get(count) != Some(&0) {
        return Err("incomplete DLL search path observation".into());
    }
    let path: &[u16] = buffer.get(..count).ok_or("truncated DLL search path")?;
    if path.contains(&0) {
        return Err("embedded terminator in DLL search path observation".into());
    }
    Ok(OsString::from_wide(path))
}

impl HeldSearchFile {
    /// Hold the expected local file and read its exact Windows device path.
    ///
    /// # Errors
    /// Preserve file/open errors and refuse a non-file, relative path or failed native readback.
    #[expect(
        unsafe_code,
        reason = "query the exact path of the held read-only file"
    )]
    fn open(directory: &Path, name: &str) -> Result<Self, Box<dyn Error>> {
        let path: PathBuf = directory.join(name);
        if !path.is_absolute() || !path.symlink_metadata()?.is_file() {
            return Err("DLL search fixture needs an absolute regular file".into());
        }
        let file: File = OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .open(&path)?;
        let mut buffer: [u16; PATH_LIMIT] = [0; PATH_LIMIT];
        let capacity: u32 = u32::try_from(buffer.len())?;
        // SAFETY: The borrowed file handle and writable path buffer remain live for this query.
        let count: u32 = unsafe {
            GetFinalPathNameByHandleW(
                file.as_raw_handle(),
                buffer.as_mut_ptr(),
                capacity,
                FILE_NAME_NORMALIZED | VOLUME_NAME_NT,
            )
        };
        if count == 0 {
            // SAFETY: Capture the failing query's original error on this thread.
            let code: u32 = unsafe { GetLastError() };
            return Err(format!("held DLL path query failed: {code}").into());
        }
        let device_path: OsString = observed_path(&buffer, count)?;
        Ok(Self {
            _file: file,
            path,
            device_path,
        })
    }
}

/// Read the actual System32 directory in this process before joining the fixed sample name.
///
/// # Errors
/// Preserve the native failure or refuse incomplete directory readback.
#[expect(unsafe_code, reason = "bounded query for the process system directory")]
fn system_directory() -> Result<PathBuf, Box<dyn Error>> {
    let mut buffer: [u16; PATH_LIMIT] = [0; PATH_LIMIT];
    let capacity: u32 = u32::try_from(buffer.len())?;
    // SAFETY: The buffer is writable for the exact capacity for this synchronous call.
    let count: u32 = unsafe { GetSystemDirectoryW(buffer.as_mut_ptr(), capacity) };
    if count == 0 {
        // SAFETY: Capture the failing query's original error on this thread.
        let code: u32 = unsafe { GetLastError() };
        return Err(format!("system directory query failed: {code}").into());
    }
    Ok(PathBuf::from(observed_path(&buffer, count)?))
}

/// Confirm that a fixed basename cannot hit an already executable-loaded module.
///
/// # Errors
/// Refuse an already loaded module or any native outcome other than module not found.
#[expect(
    unsafe_code,
    reason = "observe the process loader list without acquiring a reference"
)]
fn require_unloaded(name: &str) -> Result<(), Box<dyn Error>> {
    let wide: Vec<u16> = wide_path(OsStr::new(name))?;
    // SAFETY: The bounded UTF-16 basename has a terminator and outlives the query.
    let module: HMODULE = unsafe { GetModuleHandleW(wide.as_ptr()) };
    if !module.is_null() {
        return Err(format!("DLL search sample already loaded: {name}").into());
    }
    // SAFETY: Read the error from the null result immediately on this thread.
    let code: u32 = unsafe { GetLastError() };
    if code != ERROR_MOD_NOT_FOUND {
        return Err(format!("unexpected DLL loader-list outcome for {name}: {code}").into());
    }
    Ok(())
}

/// Read the path backing a data-file mapping without dereferencing its tagged handle.
///
/// # Errors
/// Refuse an ordinary module handle, a failed native query, or incomplete path readback.
#[expect(
    unsafe_code,
    reason = "read the backing path of an owned data-file mapping"
)]
fn mapped_datafile_path(module: HMODULE) -> Result<OsString, Box<dyn Error>> {
    let tag: usize = 1;
    if module.addr() & tag != tag {
        return Err("DLL lookup returned a handle without the data-file tag".into());
    }
    // The SDK's LDR_IS_DATAFILE uses bit zero. Use an address in the original
    // mapping for the path query and keep the original tagged handle for release.
    let address: *mut c_void = module.with_addr(module.addr() & !tag);
    let mut buffer: [u16; PATH_LIMIT] = [0; PATH_LIMIT];
    let capacity: u32 = u32::try_from(buffer.len())?;
    // SAFETY: GetCurrentProcess takes no arguments and returns a borrowed pseudohandle.
    let process: HANDLE = unsafe { GetCurrentProcess() };
    // SAFETY: The original mapping remains live and the output buffer owns capacity UTF-16 units.
    let count: u32 =
        unsafe { K32GetMappedFileNameW(process, address, buffer.as_mut_ptr(), capacity) };
    if count == 0 {
        // SAFETY: Capture the failing query's original error on this thread.
        let code: u32 = unsafe { GetLastError() };
        return Err(format!("DLL backing-path query failed: {code}").into());
    }
    observed_path(&buffer, count)
}

/// The bounded observation from a single default-search data-file lookup.
#[derive(Debug, PartialEq, Eq)]
enum DatafileLookup {
    /// Exact native backing path. The original data mapping has been released.
    Mapped(OsString),
    /// Original file-not-found or module-not-found status.
    Missing(u32),
}

/// Map only as data using the process default search and release any returned module.
///
/// # Errors
/// Preserve unexpected lookup, backing-path and release failures. Report a release failure
/// even after a path observation fails.
#[expect(
    unsafe_code,
    reason = "map a read-only data file and confirm release of its exact handle"
)]
fn lookup_datafile(name: &OsStr) -> Result<DatafileLookup, Box<dyn Error>> {
    let wide: Vec<u16> = wide_path(name)?;
    // SAFETY: The complete wide path lives through the call. The exclusive data-file flag
    // maps bytes read-only and neither prepares nor executes the image from any path.
    let module: HMODULE =
        unsafe { LoadLibraryExW(wide.as_ptr(), std::ptr::null_mut(), DATAFILE_EXCLUSIVE) };
    if module.is_null() {
        // SAFETY: Capture the null result's native error before another Win32 call.
        let code: u32 = unsafe { GetLastError() };
        if matches!(code, ERROR_FILE_NOT_FOUND | ERROR_MOD_NOT_FOUND) {
            return Ok(DatafileLookup::Missing(code));
        }
        return Err(format!("data-file DLL lookup failed for {name:?}: {code}").into());
    }
    let observed: Result<OsString, Box<dyn Error>> = mapped_datafile_path(module);
    // SAFETY: Use the exact successful LoadLibraryExW handle once, including any data tag.
    let released: i32 = unsafe { FreeLibrary(module) };
    if released == 0 {
        // SAFETY: Preserve this release operation's immediate native error.
        let code: u32 = unsafe { GetLastError() };
        return Err(match observed {
            Ok(_) => format!("DLL data-file release failed: {code}"),
            Err(error) => format!("{error}. DLL data-file release also failed: {code}"),
        }
        .into());
    }
    observed.map(DatafileLookup::Mapped)
}

/// Verify the resolved mapping identifies the independently held file.
///
/// # Errors
/// Refuse an absent or different backing-file path, or preserve a native lookup failure.
fn require_datafile(name: &OsStr, expected: &HeldSearchFile) -> Result<(), Box<dyn Error>> {
    let observed: DatafileLookup = lookup_datafile(name)?;
    if !matches!(&observed, DatafileLookup::Mapped(path) if path == &expected.device_path) {
        return Err(format!(
            "DLL data-file mapping differs from held file for {name:?}: {observed:?}"
        )
        .into());
    }
    Ok(())
}

/// Exercise process default DLL search before and after the authentic fixed-role entry.
///
/// # Errors
/// Refuse mismatched fixture paths, incomplete baselines, incorrect search behavior or
/// unconfirmed native release. Preserve the role's native admission failures.
fn verify_console_dll_search(
    enter: fn() -> Result<(), &'static FixtureError>,
    role: ServiceRole,
    role_code: u8,
) -> Result<(), Box<dyn Error>> {
    let image: PathBuf = std::env::current_exe()?.canonicalize()?;
    let application: &Path = image
        .parent()
        .ok_or("DLL search fixture image has no parent")?;
    let working: PathBuf = std::env::current_dir()?.canonicalize()?;
    if working == application {
        return Err(
            "DLL search fixture requires distinct application and working directories".into(),
        );
    }

    let application_file: HeldSearchFile = HeldSearchFile::open(application, APPLICATION_CANARY)?;
    let working_file: HeldSearchFile = HeldSearchFile::open(&working, WORKING_CANARY)?;
    let canaries: [(&str, &HeldSearchFile); 2] = [
        (APPLICATION_CANARY, &application_file),
        (WORKING_CANARY, &working_file),
    ];
    for (name, held) in canaries {
        require_unloaded(name)?;
        require_datafile(held.path.as_os_str(), held)?;
        require_datafile(OsStr::new(name), held)?;
    }
    require_unloaded(SYSTEM_SAMPLE)?;
    let system_file: HeldSearchFile = HeldSearchFile::open(&system_directory()?, SYSTEM_SAMPLE)?;
    // The sandbox can back a System32 file with a host-shared mapping. Use the
    // exact backing identity observed through the fully qualified lookup.
    let system_backing: DatafileLookup = lookup_datafile(system_file.path.as_os_str())?;
    if !matches!(system_backing, DatafileLookup::Mapped(_)) {
        return Err(
            format!("the fully qualified System32 sample did not map: {system_backing:?}").into(),
        );
    }

    verify_console_entry(enter, role, role_code)?;

    for (name, held) in canaries {
        let DatafileLookup::Missing(code) = lookup_datafile(OsStr::new(name))? else {
            return Err(format!("default search still reached the staged canary: {name}").into());
        };
        require_datafile(held.path.as_os_str(), held)?;
        println!("service_dll_search excluded_canary={name} status={code}");
    }
    require_unloaded(SYSTEM_SAMPLE)?;
    let system_observed: DatafileLookup = lookup_datafile(OsStr::new(SYSTEM_SAMPLE))?;
    if system_observed != system_backing {
        return Err("default lookup differs from the fully qualified System32 lookup".into());
    }
    println!(
        "service_dll_search canaries=located-before,excluded-after,absolute-retained system32=matched"
    );
    Ok(())
}

/// Observe default search exclusion through the actual Engine entry.
///
/// # Errors
/// Preserve fixture custody, role admission, lookup or retirement failures.
#[test]
#[ignore = "run alone with the staged search canaries in a fresh Windows console process"]
fn engine_console_entry_limits_default_dll_search() -> Result<(), Box<dyn Error>> {
    verify_console_dll_search(engine_entry, ServiceRole::Engine, 1)
}

/// Observe default search exclusion through the actual Broker entry.
///
/// # Errors
/// Preserve fixture custody, role admission, lookup or retirement failures.
#[test]
#[ignore = "run alone with the staged search canaries in a fresh Windows console process"]
fn broker_console_entry_limits_default_dll_search() -> Result<(), Box<dyn Error>> {
    verify_console_dll_search(broker_entry, ServiceRole::ClockBroker, 2)
}
