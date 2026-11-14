// SPDX-License-Identifier: GPL-2.0-only
//! Test-only console launcher; no product startup or authorization authority.
//!
//! Compile only this std-only diagnostic for baseline x86-64; Engine stays v3.
//! The ordinary-user supervisor must lock/admit this image and its selected
//! sibling image, install its non-breakaway kill-on-close job, inspect the
//! launcher token, and only then send exactly UTF-8 BOM, `G`, and EOF on stdin.
//! Before that gate this process reads only its argument and at most five stdin
//! bytes. It never spawns a child on invalid arguments, gate or I/O failure.
//! Refusal emits only a fixed stage and optional numeric OS error on stderr.
//!
//! After admission, one fixed sibling runs with inherited primary token, job,
//! environment and output streams; stdin is null. The supervisor owns timeout,
//! output bounds, job accounting and cleanup. This launcher waits for that child
//! and preserves its full Windows exit code. Launcher/setup failures return 126.
//! No shell, elevation, breakaway, service, clock or configuration API is used.

// Match the CPU diagnostic's standalone Rust/Clippy/rustdoc deny policy.
#![deny(
    warnings,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_unsafe,
    unused_must_use,
    let_underscore_drop,
    unused_macro_rules,
    unused_lifetimes,
    unfulfilled_lint_expectations,
    unknown_lints,
    renamed_and_removed_lints,
    improper_ctypes,
    improper_ctypes_definitions,
    unexpected_cfgs,
    missing_docs,
    unreachable_pub,
    unused_qualifications,
    clippy::all,
    clippy::pedantic,
    clippy::missing_docs_in_private_items,
    clippy::undocumented_unsafe_blocks,
    clippy::allow_attributes_without_reason,
    clippy::allow_attributes,
    clippy::multiple_unsafe_ops_per_block,
    clippy::unused_result_ok,
    clippy::string_slice,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    clippy::unreachable,
    clippy::dbg_macro,
    clippy::let_underscore_must_use,
    clippy::as_conversions,
    clippy::mem_forget,
    clippy::exit,
    rustdoc::all,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links
)]

#[cfg(not(all(windows, target_arch = "x86_64")))]
compile_error!("This launcher is exclusively for x64 Windows test supervision.");

use std::{
    env,
    ffi::OsStr,
    io::{self, Read, Write},
    os::windows::process::CommandExt,
    process::{self, Command, Stdio},
    thread,
    time::Duration,
};

/// Diagnostic-only observation opportunity; the supervisor still owns deadlines.
/// Sleeping is neither metadata-read acknowledgement nor completion evidence.
const METADATA_GRACE: Duration = Duration::from_millis(250);
/// Suppress a console window without breakaway, elevation or other custom flags.
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Closed launcher stages; none contains caller input, paths or error text.
enum Stage {
    /// Validate the sole closed mode argument.
    Arguments,
    /// Read and validate the bounded supervisor gate.
    Gate,
    /// Locate the launcher's admitted sibling directory.
    Location,
    /// Create the selected fixed child process.
    Spawn,
    /// Report the actual child's identity; returned only after its grace elapsed.
    Report,
    /// Wait for the actual child process and obtain its native exit status.
    Wait,
}

impl Stage {
    /// Return a fixed diagnostic field, at most eight ASCII bytes, without I/O.
    fn label(&self) -> &'static str {
        match self {
            Self::Arguments => "args",
            Self::Gate => "gate",
            Self::Location => "location",
            Self::Spawn => "spawn",
            Self::Report => "report",
            Self::Wait => "wait",
        }
    }
}

/// Bounded diagnostic facts only; native error text and paths are not retained.
struct Failure {
    /// Fixed phase at which launcher setup or child custody failed.
    stage: Stage,
    /// Exact optional native error number supplied by the failed OS operation.
    os_error: Option<i32>,
}

impl Failure {
    /// Capture a typed OS code from any I/O error, without formatting its payload.
    fn capture(stage: Stage, error: &io::Error) -> Self {
        Self {
            stage,
            os_error: error.raw_os_error(),
        }
    }

    /// Emit one bounded ASCII line, under 64 bytes, with no paths or error text.
    /// Output failure is returned; it cannot turn the original refusal into success.
    /// # Errors
    /// Preserve the diagnostic sink's write failure without replacing the refusal.
    fn emit(&self) -> io::Result<()> {
        let mut sink = io::stderr().lock();
        match self.os_error {
            Some(code) => writeln!(
                sink,
                "launcher-stage={};os-error={code}",
                self.stage.label()
            ),
            None => writeln!(sink, "launcher-stage={};os-error=none", self.stage.label()),
        }
    }
}

/// Closed selection of admitted sibling images and fixed argument sequences.
enum Mode {
    /// Observe CPU/OS v3 support before starting the v3 Engine image.
    Cpu,
    /// Enumerate the actual Windows Engine libtest roster without running tests.
    List,
    /// Run only the scripted KE component cohort, serially.
    Ke,
    /// Run the supervisor's bounded successful synthetic neighbor.
    ControlHealthy,
    /// Preserve the synthetic child's deliberate exit code 7.
    ControlFail,
    /// Keep the synthetic child alive until the supervisor's deadline cleanup.
    ControlWait,
    /// Run the exact process-global foreign-job claim test in isolation.
    JobNativeForeign,
    /// Run the exact authoritative provider claim test in isolation.
    ProviderNativeOnce,
    /// Run the exact provider foreign-origin test in isolation.
    ProviderNativeForeign,
    /// Observe one finite successful same-image synthetic descendant.
    ControlNestedHealthy,
    /// Preserve failure seven from one same-image synthetic descendant.
    ControlNestedFail,
    /// Leave both synthetic descendants live for owned timeout cleanup.
    ControlNestedWait,
}

impl Mode {
    /// Accept exactly one named mode; no caller-supplied path or child arguments.
    /// Reads only process arguments and returns a setup error without spawning.
    /// # Errors
    /// Refuse missing, unknown or extra arguments before child creation.
    fn from_arguments() -> io::Result<Self> {
        let mut arguments = env::args_os().skip(1);
        let mode = match arguments.next().as_deref() {
            Some(value) if value == OsStr::new("cpu") => Self::Cpu,
            Some(value) if value == OsStr::new("list") => Self::List,
            Some(value) if value == OsStr::new("ke") => Self::Ke,
            Some(value) if value == OsStr::new("control-healthy") => Self::ControlHealthy,
            Some(value) if value == OsStr::new("control-fail") => Self::ControlFail,
            Some(value) if value == OsStr::new("control-wait") => Self::ControlWait,
            Some(value) if value == OsStr::new("job-native-foreign") => Self::JobNativeForeign,
            Some(value) if value == OsStr::new("provider-native-once") => Self::ProviderNativeOnce,
            Some(value) if value == OsStr::new("provider-native-foreign") => {
                Self::ProviderNativeForeign
            }
            Some(value) if value == OsStr::new("control-nested-healthy") => {
                Self::ControlNestedHealthy
            }
            Some(value) if value == OsStr::new("control-nested-fail") => Self::ControlNestedFail,
            Some(value) if value == OsStr::new("control-nested-wait") => Self::ControlNestedWait,
            _ => return Err(io::Error::other("Unadmitted launcher mode.")),
        };
        if arguments.next().is_some() {
            return Err(io::Error::other("Extra launcher argument."));
        }
        Ok(mode)
    }

    /// Every typed mode maps to one fixed sibling basename and finite arguments.
    /// Pure and constant work; cannot select a shell or arbitrary command.
    fn child(&self) -> (&'static str, &'static [&'static str]) {
        match self {
            Self::Cpu => ("cpu-v3-probe.exe", &[]),
            Self::List => ("engine-tests.exe", &["--list", "--format", "terse"]),
            // These required nested-child controls await their separate custody
            // qualification; the flat supervisor keeps its existing process cap.
            Self::Ke => (
                "engine-tests.exe",
                &[
                    "nts_ke_experiment::",
                    "--test-threads=1",
                    "--skip",
                    "nts_ke_experiment::job::tests::job_native_foreign_origin_records_control_before_actual_retirement",
                    "--skip",
                    "nts_ke_experiment::job::tests::provider_native_authoritative_once_preserves_permanent_admission",
                    "--skip",
                    "nts_ke_experiment::job::tests::provider_native_foreign_origins_refuse_before_effect_factory",
                ],
            ),
            Self::ControlHealthy => ("gate-control.exe", &["healthy"]),
            Self::ControlFail => ("gate-control.exe", &["fail"]),
            Self::ControlWait => ("gate-control.exe", &["wait"]),
            Self::JobNativeForeign => (
                "engine-tests.exe",
                &[
                    "--exact",
                    "nts_ke_experiment::job::tests::job_native_foreign_origin_records_control_before_actual_retirement",
                    "--nocapture",
                    "--test-threads=1",
                ],
            ),
            Self::ProviderNativeOnce => (
                "engine-tests.exe",
                &[
                    "--exact",
                    "nts_ke_experiment::job::tests::provider_native_authoritative_once_preserves_permanent_admission",
                    "--nocapture",
                    "--test-threads=1",
                ],
            ),
            Self::ProviderNativeForeign => (
                "engine-tests.exe",
                &[
                    "--exact",
                    "nts_ke_experiment::job::tests::provider_native_foreign_origins_refuse_before_effect_factory",
                    "--nocapture",
                    "--test-threads=1",
                ],
            ),
            Self::ControlNestedHealthy => ("gate-control.exe", &["nested-healthy"]),
            Self::ControlNestedFail => ("gate-control.exe", &["nested-fail"]),
            Self::ControlNestedWait => ("gate-control.exe", &["nested-wait"]),
        }
    }
}

/// Admit exactly BOM + `G` + EOF, reading at most five bytes from raw stdin.
/// Missing, extra or wrong bytes and read failures return an error; no spawn.
/// Waiting for the final EOF is bounded by the external supervisor's custody.
/// # Errors
/// Preserve read failure or refuse any gate other than the exact four-byte value.
fn admit_gate(input: impl Read) -> io::Result<()> {
    let mut bytes = Vec::with_capacity(5);
    input.take(5).read_to_end(&mut bytes)?;
    if bytes.as_slice() != b"\xEF\xBB\xBFG" {
        return Err(io::Error::other("Unadmitted launcher gate."));
    }
    Ok(())
}

/// Emit the actual created child's PID once, before waiting for completion.
/// The fixed prefix, at most ten decimal digits and LF occupy at most 30 bytes.
/// # Errors
/// Preserve the stderr write failure without changing child completion custody.
fn report_child_pid(pid: u32) -> io::Result<()> {
    writeln!(io::stderr().lock(), "launcher-child-pid={pid}")
}

/// Requires a supervisor-owned locked image directory and non-breakaway job.
/// Admit the sole mode and gate before resolving or starting one sibling child.
/// Wait for actual child exit; propagate its full code or a bounded typed error.
/// The child inherits only supervisor-established context and output streams.
/// Attempt its actual PID report before waiting; a write failure still waits.
/// A nonzero child exit takes precedence over report failure; a zero exit cannot.
/// After actual wait, retain the Child handle for one fixed metadata grace before
/// releasing it. A missed or stale observer read still authorizes no success.
/// Wait failures leave real child completion and cleanup with the supervisor job.
/// # Errors
/// Preserve the fixed stage and optional native code on admission/spawn/wait
/// refusal, or report failure after an otherwise successful child completion.
fn run() -> Result<i32, Failure> {
    let mode =
        Mode::from_arguments().map_err(|error| Failure::capture(Stage::Arguments, &error))?;
    admit_gate(io::stdin().lock()).map_err(|error| Failure::capture(Stage::Gate, &error))?;
    let launcher = env::current_exe().map_err(|error| Failure::capture(Stage::Location, &error))?;
    let directory = launcher.parent().ok_or(Failure {
        stage: Stage::Location,
        os_error: None,
    })?;
    let (image, arguments) = mode.child();
    let mut child = Command::new(directory.join(image))
        .args(arguments)
        .current_dir(directory)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|error| Failure::capture(Stage::Spawn, &error))?;
    let reported =
        report_child_pid(child.id()).map_err(|error| Failure::capture(Stage::Report, &error));
    let code = child
        .wait()
        .map_err(|error| Failure::capture(Stage::Wait, &error))?
        .code()
        .ok_or(Failure {
            stage: Stage::Wait,
            os_error: None,
        })?;
    thread::sleep(METADATA_GRACE);
    drop(child);
    if code == 0 {
        reported?;
    }
    Ok(code)
}

/// Waited child code is preserved; argument/gate/setup failures return 126.
/// Failure stderr contains only a fixed stage and optional native numeric code.
/// A failed diagnostic write still returns 126. Every created child adds one
/// bounded PID marker to stderr; the child's own stderr remains inherited.
/// Refusal emits its stage before one grace for parent metadata inspection;
/// a report refusal has already held the child grace and does not sleep twice.
/// All `run` locals have retired before this fixture-only process termination.
fn main() {
    let code = match run() {
        Ok(code) => code,
        Err(failure) => {
            let needs_grace = !matches!(&failure.stage, Stage::Report);
            // Preserve refusal even if its bounded diagnostic sink is unavailable.
            failure.emit().unwrap_or(());
            if needs_grace {
                thread::sleep(METADATA_GRACE);
            }
            126
        }
    };
    // The stable Windows ExitCode API exposes only u8 construction. Preserve
    // the full i32 native status here after all run-local resources retired.
    process::exit(code);
}
