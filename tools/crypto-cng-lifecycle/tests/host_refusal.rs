// SPDX-License-Identifier: GPL-2.0-only
//! Actual Linux CLI refusal and failed-stderr controls; no Windows or TLS claim.
//!
//! Children inherit the admitted namespace and environment. Regular files avoid
//! blocked pipe drains. Output-size polling is sampled; the admitting runner's
//! file-size cap supplies the separate hard storage bound.

#![cfg(target_os = "linux")]
#![forbid(unsafe_code)]

use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{self, Read, Seek, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Process-local serials distinguish parallel tests without global environment changes.
static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);
/// Maximum observed length of either regular output file before refusal.
const OUTPUT_LIMIT: u64 = 4_096;
/// Refuse observations completing at or after this original-start deadline.
const CHILD_TIME: Duration = Duration::from_secs(5);
/// Finite polling delay; cleanup still retains custody through actual reap.
const POLL_TIME: Duration = Duration::from_millis(5);

/// Choose the actual access mode of the file supplied as the child's stderr.
#[derive(Clone, Copy)]
enum StderrMode {
    /// Ordinary writable regular file admits the bounded refusal diagnostic.
    Writable,
    /// A read-only regular-file descriptor makes the diagnostic write fail.
    ReadOnly,
}

/// Facts read only after the actual child was reaped, before owned-file cleanup.
struct Observation {
    /// Actual process status, including signal termination when present.
    status: ExitStatus,
    /// Bounded bytes emitted by the actual stdout descriptor.
    stdout: Vec<u8>,
    /// Bounded bytes present in the actual stderr file.
    stderr: Vec<u8>,
}

/// Own exactly one private directory and any child that has not yet been reaped.
struct Fixture {
    /// Present until successful removal; no other directory is eligible for cleanup.
    directory: Option<PathBuf>,
    /// Armed immediately after spawn and cleared only after actual process reap.
    child: Option<Child>,
}

impl Fixture {
    /// Atomically create one private directory with finitely many collision attempts.
    ///
    /// # Errors
    /// Preserve serial exhaustion, temporary-directory admission or creation failure.
    fn new() -> io::Result<Self> {
        for _ in 0..16 {
            let serial = NEXT_DIRECTORY
                .try_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                    value.checked_add(1)
                })
                .map_err(|_| io::Error::other("temporary directory serial exhausted"))?;
            let path = std::env::temp_dir().join(format!(
                "nevrotick-host-refusal-{}-{serial}",
                std::process::id()
            ));
            match DirBuilder::new().mode(0o700).create(&path) {
                Ok(()) => {
                    return Ok(Self {
                        directory: Some(path),
                        child: None,
                    });
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error),
            }
        }
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "temporary directory collisions",
        ))
    }

    /// Launch Cargo's actual executable without arguments and observe only host refusal.
    ///
    /// # Errors
    /// Preserve file setup, spawn, deadline, output-budget, wait or bounded-read failure.
    fn observe(&mut self, mode: StderrMode) -> io::Result<Observation> {
        let directory = self
            .directory
            .as_ref()
            .ok_or_else(|| io::Error::other("closed fixture"))?;
        let stdout = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(directory.join("stdout"))?;
        let stderr_path = directory.join("stderr");
        let stderr = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&stderr_path)?;
        let stderr_descriptor = match mode {
            StderrMode::Writable => stderr.try_clone()?,
            StderrMode::ReadOnly => File::open(stderr_path)?,
        };
        let deadline = Instant::now()
            .checked_add(CHILD_TIME)
            .ok_or_else(|| io::Error::other("unrepresentable child deadline"))?;
        self.child = Some(
            Command::new(env!("CARGO_BIN_EXE_nevrotick-nts-ke-probe"))
                .stdin(Stdio::null())
                .stdout(Stdio::from(stdout.try_clone()?))
                .stderr(Stdio::from(stderr_descriptor))
                .spawn()?,
        );
        let status = self.wait(deadline, &stdout, &stderr)?;
        Ok(Observation {
            status,
            stdout: read_bounded(stdout)?,
            stderr: read_bounded(stderr)?,
        })
    }

    /// Poll time and regular-file lengths while retaining the actual child owner.
    ///
    /// # Errors
    /// Refuse a missing child, sampled output excess, late completion or OS wait failure.
    fn wait(&mut self, deadline: Instant, stdout: &File, stderr: &File) -> io::Result<ExitStatus> {
        loop {
            if Instant::now() >= deadline {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "host refusal deadline",
                ));
            }
            for file in [stdout, stderr] {
                if file.metadata()?.len() > OUTPUT_LIMIT {
                    return Err(io::Error::other("host refusal output limit"));
                }
            }
            let child = self
                .child
                .as_mut()
                .ok_or_else(|| io::Error::other("missing child"))?;
            if let Some(status) = child.try_wait()? {
                self.child = None;
                if Instant::now() >= deadline {
                    return Err(io::Error::new(io::ErrorKind::TimedOut, "late host refusal"));
                }
                return Ok(status);
            }
            std::thread::sleep(POLL_TIME);
        }
    }

    /// Kill and reap any outstanding child before removing only the owned directory.
    ///
    /// # Errors
    /// Report kill, reap and directory-removal failures together. A failed reap keeps
    /// the child and directory owned for the Drop fallback rather than claiming cleanup.
    fn cleanup(&mut self) -> io::Result<()> {
        let mut failures = Vec::new();
        if let Some(child) = self.child.as_mut() {
            if let Err(error) = child.kill() {
                failures.push(format!("kill: {error}"));
            }
            match child.wait() {
                Ok(_) => self.child = None,
                Err(error) => failures.push(format!("reap: {error}")),
            }
        }
        if self.child.is_none()
            && let Some(directory) = self.directory.as_ref()
        {
            match fs::remove_dir_all(directory) {
                Ok(()) => self.directory = None,
                Err(error) => failures.push(format!("remove owned directory: {error}")),
            }
        }
        if failures.is_empty() {
            Ok(())
        } else {
            Err(io::Error::other(failures.join("; ")))
        }
    }
}

impl Drop for Fixture {
    /// Preserve custody during unwinding; normal cleanup errors are returned separately.
    fn drop(&mut self) {
        if let Err(error) = self.cleanup() {
            // Preserve the original panic/error; diagnostic delivery is best effort.
            let _diagnostic_result = writeln!(
                io::stderr().lock(),
                "host_refusal cleanup failure: {:?}",
                error.kind()
            );
        }
    }
}

/// Read no more than the admitted output budget plus one excess-detection byte.
///
/// # Errors
/// Preserve seek/read failure or refuse a file exceeding the 4,096-byte budget.
fn read_bounded(mut file: File) -> io::Result<Vec<u8>> {
    file.rewind()?;
    let mut bytes = Vec::new();
    file.take(4_097).read_to_end(&mut bytes)?;
    if bytes.len() > 4_096 {
        return Err(io::Error::other("host refusal output limit"));
    }
    Ok(bytes)
}

/// Complete explicit cleanup before returning any facts or propagating a primary error.
///
/// # Errors
/// Preserve setup/observation failure and attach any separate cleanup failure.
fn observe(mode: StderrMode) -> io::Result<Observation> {
    let mut fixture = Fixture::new()?;
    let observation = fixture.observe(mode);
    let cleanup = fixture.cleanup();
    match (observation, cleanup) {
        (Ok(observation), Ok(())) => Ok(observation),
        (Err(error), Ok(())) | (Ok(_), Err(error)) => Err(error),
        (Err(primary), Err(cleanup)) => Err(io::Error::other(format!(
            "observation: {primary}; cleanup: {cleanup}"
        ))),
    }
}

/// Actual unsupported-platform execution emits exactly the independent 43-byte refusal.
///
/// # Errors
/// Preserve actual executable, output, time-bound or cleanup failure.
///
/// # Panics
/// Fail if the actual process succeeds, writes stdout or changes the literal stderr bytes.
#[test]
fn linux_cli_refuses_with_literal_diagnostic() -> io::Result<()> {
    let result = observe(StderrMode::Writable)?;
    assert_eq!(result.status.code(), Some(1));
    assert_eq!(result.stdout, b"");
    assert_eq!(
        result.stderr,
        b"stage=admission cause=unsupported-platform\n"
    );
    Ok(())
}

/// A real read-only stderr descriptor cannot turn unsupported-platform refusal into success.
///
/// # Errors
/// Preserve actual executable, descriptor, output, time-bound or cleanup failure.
///
/// # Panics
/// Fail if the process succeeds, emits stdout or changes the read-only stderr file.
#[test]
fn linux_cli_refuses_when_stderr_is_read_only() -> io::Result<()> {
    let result = observe(StderrMode::ReadOnly)?;
    assert_eq!(result.status.code(), Some(1));
    assert_eq!(result.stdout, b"");
    assert_eq!(result.stderr, b"");
    Ok(())
}
