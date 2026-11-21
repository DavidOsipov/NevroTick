// SPDX-License-Identifier: GPL-2.0-only
//! Test-only controller and file consumer for an in-process fixture trace.

#![expect(
    unsafe_code,
    reason = "the fixture owns the ETW controller and callback boundaries"
)]

use std::{
    error::Error,
    fs, io,
    mem::{offset_of, size_of},
    os::windows::ffi::OsStrExt,
    path::{Component, Path, PathBuf, Prefix},
    ptr::null,
    sync::atomic::{AtomicU32, Ordering},
};

use windows_sys::Win32::{
    Foundation::{ERROR_MORE_DATA, ERROR_SUCCESS, WIN32_ERROR},
    System::Diagnostics::Etw::{
        CONTROLTRACE_HANDLE, CloseTrace, ControlTraceW, EVENT_CONTROL_CODE_ENABLE_PROVIDER,
        EVENT_RECORD, EVENT_TRACE_CONTROL_STOP, EVENT_TRACE_FILE_MODE_SEQUENTIAL,
        EVENT_TRACE_LOGFILEW, EVENT_TRACE_NO_PER_PROCESSOR_BUFFERING, EVENT_TRACE_PRIVATE_IN_PROC,
        EVENT_TRACE_PRIVATE_LOGGER_MODE, EVENT_TRACE_PROPERTIES, EnableTraceEx2, OpenTraceW,
        PROCESS_TRACE_MODE_EVENT_RECORD, PROCESSTRACE_HANDLE, ProcessTrace, StartTraceW,
        WNODE_FLAG_TRACED_GUID,
    },
};
use windows_sys::core::GUID;

use crate::diagnostics::threaded::EventDescriptor;

/// The fixed fixture publisher, independently matched by the consumer.
const FIXTURE_PROVIDER: GUID = GUID::from_u128(0x407b_0d7f_0220_4ca1_9e6b_d0a6_c0d3_3380);
/// ETW properties permit up to 1,024 characters for each trailing string.
const NAME_CAPACITY: usize = 1_024;
/// The maximum log size requested from ETW, in MiB.
const MAX_FILE_MIB: u32 = 1;
/// The session is a local file trace confined to the calling process.
const LOG_MODE: u32 = EVENT_TRACE_PRIVATE_LOGGER_MODE
    | EVENT_TRACE_PRIVATE_IN_PROC
    | EVENT_TRACE_FILE_MODE_SEQUENTIAL
    | EVENT_TRACE_NO_PER_PROCESSOR_BUFFERING;
/// Distinguishes repeated captures in one test process.
static NEXT_CAPTURE: AtomicU32 = AtomicU32::new(0);

/// Bounded, test-local error reporting without retaining payloads outside this test.
type TestResult = Result<(), Box<dyn Error>>;

/// Capture exactly one independently expected fixture event from the current process.
/// The action must finish the writer's lifecycle before it returns. The test runner
/// owns the process deadline and a local temporary directory.
///
/// # Errors
///
/// Returns an error on scratch, controller, action, consumer or cleanup failure,
/// trace loss, or any mismatch in the fixture event count, provenance or content.
pub(crate) fn capture(
    action: impl FnOnce() -> TestResult,
    descriptor: EventDescriptor,
    expected_fields: [u8; 19],
) -> TestResult {
    let pid: u32 = std::process::id();
    let ordinal: u32 = NEXT_CAPTURE.fetch_add(1, Ordering::Relaxed);
    let name: String = format!("NevroTick-Writer-ETW-{pid}-{ordinal}");
    let temp: PathBuf = std::env::temp_dir();
    if !temp.is_absolute()
        || !matches!(
            temp.components().next(),
            Some(Component::Prefix(prefix)) if matches!(prefix.kind(), Prefix::Disk(_) | Prefix::VerbatimDisk(_))
        )
    {
        return Err(io::Error::other("ETW fixture needs a local absolute scratch root").into());
    }

    let directory: PathBuf = temp.join(&name);
    // A collision fails without adopting an existing directory or its contents.
    fs::create_dir(&directory)?;
    let mut scratch: Scratch = Scratch {
        file: directory.join("fixture.etl"),
        suffixed_file: directory.join(format!("fixture.etl_{pid}")),
        directory,
        removed: false,
        trace_retired: true,
    };
    let result: TestResult = capture_to_file(
        &mut scratch,
        &name,
        action,
        pid,
        descriptor,
        expected_fields,
    );
    let cleanup: TestResult = if scratch.trace_retired {
        scratch.remove()
    } else {
        Err(io::Error::other("trace retirement unconfirmed, retain fixture files").into())
    };
    finish(result, cleanup)
}

/// Owns the newly created directory containing this capture's ETL.
struct Scratch {
    /// Directory allocated by this call.
    directory: PathBuf,
    /// Direct file path used by the in-process controller.
    file: PathBuf,
    /// Alternative name used when ETW appends the private logger PID.
    suffixed_file: PathBuf,
    /// Successful removal retires ownership.
    removed: bool,
    /// File cleanup requires a confirmed controller stop when a trace started.
    trace_retired: bool,
}

impl Scratch {
    /// Remove this invocation's scratch after its controller and consumer retire.
    ///
    /// # Errors
    ///
    /// Returns the filesystem error if removal fails.
    fn remove(&mut self) -> TestResult {
        let files: TestResult = finish(
            remove_if_present(&self.file),
            remove_if_present(&self.suffixed_file),
        );
        let directory: TestResult = fs::remove_dir(&self.directory).map_err(Into::into);
        self.removed = directory.is_ok();
        finish(files, directory)
    }
}

impl Drop for Scratch {
    /// Attempt cleanup on unwind or after a reported removal failure.
    fn drop(&mut self) {
        if !self.removed && self.trace_retired {
            drop(self.remove());
        }
    }
}

/// Remove only a named fixture artifact, including one ETW never created.
///
/// # Errors
///
/// Returns a filesystem error for a failed removal of a present artifact.
fn remove_if_present(path: &Path) -> TestResult {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}

/// Run the complete native capture before the owning directory is removed.
///
/// # Errors
///
/// Returns a controller, action, file, consumer or expectation failure. Stop is
/// attempted even if enable or the action fails.
fn capture_to_file(
    scratch: &mut Scratch,
    name: &str,
    action: impl FnOnce() -> TestResult,
    pid: u32,
    descriptor: EventDescriptor,
    expected_fields: [u8; 19],
) -> TestResult {
    let mut properties: TraceProperties = TraceProperties::new()?;
    let wide: Vec<u16> = scratch.file.as_os_str().encode_wide().collect();
    // A zeroed tail supplies the terminator and room for a possible PID suffix.
    if wide.len() > NAME_CAPACITY - 12 || wide.contains(&0) {
        return Err(io::Error::other("ETW fixture trace path is too long or contains NUL").into());
    }
    let target: &mut [u16] = properties
        .file_name
        .get_mut(..wide.len())
        .ok_or_else(|| io::Error::other("ETW fixture trace path exceeds its buffer"))?;
    target.copy_from_slice(&wide);

    let mut session: Session = Session::start(name, &mut properties)?;
    scratch.trace_retired = false;
    let result: TestResult = session.enable().and_then(|()| action());
    let stopped: TestResult = session.stop();
    scratch.trace_retired = !session.active;
    finish(result, stopped)?;
    let path: &Path = match (
        bounded_file(&scratch.file)?,
        bounded_file(&scratch.suffixed_file)?,
    ) {
        (true, false) => &scratch.file,
        (false, true) => &scratch.suffixed_file,
        _ => return Err(io::Error::other("expected exactly one fixture ETL file").into()),
    };
    read_trace(path, pid, descriptor, expected_fields)
}

/// Recognize only an owned candidate name with a finite regular-file size.
///
/// # Errors
///
/// Returns an error for a path other than a finite nonempty regular file or
/// absent candidate, including an unexpected symlink.
fn bounded_file(path: &Path) -> Result<bool, io::Error> {
    match fs::symlink_metadata(path) {
        Ok(metadata)
            if metadata.is_file()
                && (1..=u64::from(MAX_FILE_MIB) * 1024 * 1024).contains(&metadata.len()) =>
        {
            Ok(true)
        }
        Ok(_) => Err(io::Error::other(
            "fixture ETL is not a bounded regular file",
        )),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}

/// ABI-aligned properties and output strings, all in one live allocation.
#[repr(C)]
struct TraceProperties {
    /// Header passed to the Windows controller.
    event: EVENT_TRACE_PROPERTIES,
    /// Storage for the controller's session name.
    logger_name: [u16; NAME_CAPACITY],
    /// Input and output storage for the controller's ETL path.
    file_name: [u16; NAME_CAPACITY],
}

impl TraceProperties {
    /// Initialize the common controller buffer and its checked native offsets.
    ///
    /// # Errors
    ///
    /// Returns a conversion error if the ABI layout does not fit the API fields.
    fn new() -> Result<Self, Box<dyn Error>> {
        let mut event: EVENT_TRACE_PROPERTIES = EVENT_TRACE_PROPERTIES::default();
        event.Wnode.BufferSize = u32::try_from(size_of::<Self>())?;
        event.Wnode.Flags = WNODE_FLAG_TRACED_GUID;
        event.LoggerNameOffset = u32::try_from(offset_of!(Self, logger_name))?;
        event.LogFileNameOffset = u32::try_from(offset_of!(Self, file_name))?;
        event.LogFileMode = LOG_MODE;
        Ok(Self {
            event,
            logger_name: [0; NAME_CAPACITY],
            file_name: [0; NAME_CAPACITY],
        })
    }
}

/// Owned controller handle for the private session.
struct Session {
    /// Handle obtained from the successful `StartTraceW` call.
    handle: CONTROLTRACE_HANDLE,
    /// GUID returned in the controller properties.
    guid: GUID,
    /// Whether stopping this specific controller handle is still required.
    active: bool,
}

impl Session {
    /// Start this capture's single private file session.
    ///
    /// # Errors
    ///
    /// Returns the `StartTraceW` failure status without adopting a session.
    fn start(name: &str, properties: &mut TraceProperties) -> Result<Self, Box<dyn Error>> {
        let name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
        properties.event.Wnode.ClientContext = 1; // QPC timestamps.
        properties.event.BufferSize = 64;
        properties.event.MinimumBuffers = 4;
        properties.event.MaximumBuffers = 4;
        properties.event.MaximumFileSize = MAX_FILE_MIB;
        let mut handle: CONTROLTRACE_HANDLE = CONTROLTRACE_HANDLE::default();
        // SAFETY: The valid terminated name and full mutable properties buffer
        // live through the call. Its two offsets are measured within that buffer.
        let code: WIN32_ERROR = unsafe {
            StartTraceW(
                &raw mut handle,
                name.as_ptr(),
                (&raw mut *properties).cast(),
            )
        };
        status("StartTraceW", code)?;
        Ok(Self {
            handle,
            guid: properties.event.Wnode.Guid,
            active: true,
        })
    }

    /// Enable the one fixture provider before the action may emit records.
    ///
    /// # Errors
    ///
    /// Returns the `EnableTraceEx2` status, including the finite timeout.
    fn enable(&self) -> TestResult {
        // SAFETY: The controller owns this live same-process session and the GUID
        // is a live constant. No callback or filter pointer is registered here.
        let code: WIN32_ERROR = unsafe {
            EnableTraceEx2(
                self.handle,
                &FIXTURE_PROVIDER,
                EVENT_CONTROL_CODE_ENABLE_PROVIDER,
                u8::MAX,
                u64::MAX,
                0,
                2_000,
                null(),
            )
        };
        status("EnableTraceEx2", code)
    }

    /// Stop this controller's session then inspect ETW's final loss counters.
    ///
    /// # Errors
    ///
    /// Returns the controller failure status or reports lost events or buffers.
    fn stop(&mut self) -> TestResult {
        if !self.active {
            return Ok(());
        }
        let mut properties: TraceProperties = TraceProperties::new()?;
        properties.event.Wnode.Guid = self.guid;
        // SAFETY: The handle is this owned session's controller. The properties
        // and its output strings are live, disjoint fields in one mutable buffer.
        let code: WIN32_ERROR = unsafe {
            ControlTraceW(
                self.handle,
                null(),
                (&raw mut properties).cast(),
                EVENT_TRACE_CONTROL_STOP,
            )
        };
        // ERROR_MORE_DATA reports that STOP happened but its readback did not fit.
        if matches!(code, ERROR_SUCCESS | ERROR_MORE_DATA) {
            self.active = false;
        }
        status("ControlTraceW(STOP)", code)?;
        let lost: u32 = properties.event.EventsLost;
        let buffers_lost: u32 = properties.event.LogBuffersLost;
        if lost != 0 || buffers_lost != 0 {
            return Err(io::Error::other(format!(
                "ETW lost {lost} events and {buffers_lost} log buffers"
            ))
            .into());
        }
        Ok(())
    }
}

impl Drop for Session {
    /// Attempt to stop only the owned session if explicit stop did not retire it.
    fn drop(&mut self) {
        if self.active {
            drop(self.stop());
        }
    }
}

/// One raw fixture event observed after reopening the stopped ETL file.
#[derive(Debug, PartialEq, Eq)]
struct ObservedEvent {
    /// Original PID from the ETW header.
    pid: u32,
    /// Descriptor from the ETW header, compared in full.
    descriptor: EventDescriptor,
    /// Exact fixed-size event data, with absent or malformed payload distinguished.
    fields: Option<[u8; 19]>,
}

/// The bounded callback state, holding only the first matching provider event.
#[derive(Default)]
struct Observation {
    /// Saturating count so duplicate events always fail the one-event check.
    count: u32,
    /// First event from the fixture provider regardless of event ID or PID.
    first: Option<ObservedEvent>,
}

/// Match the provider and copy only the first fixture event during replay.
///
/// # Safety
///
/// ETW supplies a live event record and invokes this callback synchronously
/// during replay. Its user context is the exclusively owned Observation passed
/// to `OpenTraceW`. No pointers are kept, and the callback does no fallible work.
unsafe extern "system" fn observe(record: *mut EVENT_RECORD) {
    // SAFETY: The record is borrowed only for the ETW callback.
    let Some(record) = (unsafe { record.as_ref() }) else {
        return;
    };
    let provider: GUID = record.EventHeader.ProviderId;
    if provider.data1 != FIXTURE_PROVIDER.data1
        || provider.data2 != FIXTURE_PROVIDER.data2
        || provider.data3 != FIXTURE_PROVIDER.data3
        || provider.data4 != FIXTURE_PROVIDER.data4
    {
        return;
    }

    // SAFETY: ProcessTrace serially invokes the callback over the live, otherwise
    // unborrowed Observation in read_trace. This borrow ends on return.
    let Some(observation) = (unsafe { record.UserContext.cast::<Observation>().as_mut() }) else {
        return;
    };
    observation.count = observation.count.saturating_add(1);
    if observation.count != 1 {
        return;
    }
    let fields: Option<[u8; 19]> = if record.UserDataLength == 19 && !record.UserData.is_null() {
        // SAFETY: The native record promises this many readable bytes through
        // the callback. A byte array has alignment one and is copied by value.
        Some(unsafe { record.UserData.cast::<[u8; 19]>().read() })
    } else {
        None
    };
    observation.first = Some(ObservedEvent {
        pid: record.EventHeader.ProcessId,
        descriptor: EventDescriptor {
            id: record.EventHeader.EventDescriptor.Id,
            version: record.EventHeader.EventDescriptor.Version,
            channel: record.EventHeader.EventDescriptor.Channel,
            level: record.EventHeader.EventDescriptor.Level,
            opcode: record.EventHeader.EventDescriptor.Opcode,
            task: record.EventHeader.EventDescriptor.Task,
            keyword: record.EventHeader.EventDescriptor.Keyword,
        },
        fields,
    });
}

/// Consume the stopped file and verify the independent fixture expectation.
///
/// # Errors
///
/// Returns file-open, process or close errors, header loss or an event mismatch.
fn read_trace(path: &Path, pid: u32, descriptor: EventDescriptor, fields: [u8; 19]) -> TestResult {
    let mut observation: Observation = Observation::default();
    let mut file_name: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let invalid_handle: u64 = u64::try_from(usize::MAX)?;
    let mut logfile: EVENT_TRACE_LOGFILEW = EVENT_TRACE_LOGFILEW {
        LogFileName: file_name.as_mut_ptr(),
        Context: (&raw mut observation).cast(),
        ..EVENT_TRACE_LOGFILEW::default()
    };
    logfile.Anonymous1.ProcessTraceMode = PROCESS_TRACE_MODE_EVENT_RECORD;
    logfile.Anonymous2.EventRecordCallback = Some(observe);

    // SAFETY: The logfile, its terminated path, and exclusive context live
    // until the consumer closes. The event-record mode matches the callback ABI.
    let handle: PROCESSTRACE_HANDLE = unsafe { OpenTraceW(&raw mut logfile) };
    if handle.Value == invalid_handle {
        return Err(
            io::Error::other(format!("OpenTraceW failed: {}", io::Error::last_os_error())).into(),
        );
    }
    let buffers_lost: u32 = logfile.LogfileHeader.BuffersLost;
    // SAFETY: One open file handle and its context remain live and unborrowed
    // through the synchronous replay. Both optional time filters are absent.
    let process: WIN32_ERROR = unsafe { ProcessTrace(&raw const handle, 1, null(), null()) };
    // SAFETY: Replay has returned and this is the one owned consumer handle.
    let close: WIN32_ERROR = unsafe { CloseTrace(handle) };
    finish(status("ProcessTrace", process), status("CloseTrace", close))?;

    if buffers_lost != 0 {
        return Err(
            io::Error::other(format!("fixture ETL reports {buffers_lost} lost buffers")).into(),
        );
    }
    let expected: ObservedEvent = ObservedEvent {
        pid,
        descriptor,
        fields: Some(fields),
    };
    if observation.count != 1 || observation.first.as_ref() != Some(&expected) {
        return Err(io::Error::other(format!(
            "expected one fixture ETW event {expected:?}, observed {} with first {:?}",
            observation.count, observation.first
        ))
        .into());
    }
    println!(
        "observed one fixture ETW event {:?} with zero file buffers lost",
        observation.first
    );
    Ok(())
}

/// Preserve the raw Win32 status and its operation name.
///
/// # Errors
///
/// Returns a diagnostic for any status other than `ERROR_SUCCESS`.
fn status(operation: &str, code: WIN32_ERROR) -> TestResult {
    if code == ERROR_SUCCESS {
        Ok(())
    } else {
        Err(io::Error::other(format!("{operation} returned Win32 status {code}")).into())
    }
}

/// Preserve operation failure precedence while surfacing a cleanup failure.
///
/// # Errors
///
/// Returns the primary failure first, followed by any cleanup failure.
fn finish(primary: TestResult, cleanup: TestResult) -> TestResult {
    match (primary, cleanup) {
        (Err(primary), Err(cleanup)) => {
            Err(io::Error::other(format!("{primary}. Cleanup also failed: {cleanup}")).into())
        }
        (Err(error), _) | (_, Err(error)) => Err(error),
        (Ok(()), Ok(())) => Ok(()),
    }
}
