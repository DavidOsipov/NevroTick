// SPDX-License-Identifier: GPL-2.0-only
//! Exact-test process custody for the finite N3 async acquisition controls.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

const CHILD_ROLE: &str = "NEVROTICK_N3_SERIES_TEST_CHILD";
const BODY_ENTERED: &str = "N3_SERIES_EXACT_CHILD_ENTERED";
const SEMANTIC_FAILED: &str = "N3_SEMANTIC_CHECK_FAILED";
const CHILD_WALL: Duration = Duration::from_secs(5);
const REAP_WALL: Duration = Duration::from_millis(500);
const OUTPUT_LIMIT: u64 = 16_384;
const COVERAGE_PROFILE: &str = "/work/target/llvm-cov-target/source-%p-%m.profraw";
const CASES: [&str; 12] = [
    "series::tests::single_slot_driver_enforces_receive_cap_with_valid_boundary_neighbor",
    "series::tests::single_slot_driver_enforces_send_cap_with_valid_boundary_neighbor",
    "series::tests::setup_and_registration_failures_preserve_actual_prior_completion",
    "series::tests::final_checkpoint_reads_current_control_without_rewriting_three_completions",
    "series::tests::synchronous_counter_boundaries_do_not_panic_and_preserve_exact_effects",
    "three_real_requests_retain_distinct_identity_and_one_exact_origin",
    "ipv6_real_requests_retain_distinct_identity_and_one_exact_origin",
    "unavailable_measurements_still_consume_once_after_rejected_neighbors",
    "middle_request_timeout_preserves_earlier_facts_and_starts_final_attempt",
    "stop_preserves_first_completion_and_marks_third_not_started",
    "invalidation_preserves_first_completion_and_marks_third_not_started",
    "total_deadline_preserves_first_completion_and_prevents_third_send",
];

/// Run the whole named body in an owned child, or admit that exact child body.
/// The parent returns true after one named pass; the selected child returns false.
/// # Errors
/// Preserve setup, exact-selection, native monitoring and cleanup failures.
/// # Panics
/// Fail a reached case's noncompletion only after its child is killed and reaped.
pub(super) fn parent_or_child(name: &'static str) -> io::Result<bool> {
    let result = dispatch(name);
    if let Err(error) = &result {
        eprintln!("N3_HARNESS_INCOMPLETE");
        eprintln!("N3_HARNESS_CAUSE {error}");
    }
    result
}

/// Record only a failed independent condition with a bounded code-owned fact.
pub(super) fn semantic_check_failed(fact: &'static str) {
    if fact.len() <= 160 && fact.is_ascii() && !fact.bytes().any(|byte| byte.is_ascii_control()) {
        eprintln!("{SEMANTIC_FAILED}");
        eprintln!("N3_SEMANTIC_FACT {fact}");
    } else {
        eprintln!("N3_HARNESS_INCOMPLETE");
        eprintln!("N3_HARNESS_CAUSE invalid static semantic diagnostic");
    }
}

/// Admit only the closed child role or own the exact parent process.
/// # Errors
/// Return setup, role, monitoring or cleanup failure without semantic conversion.
fn dispatch(name: &'static str) -> io::Result<bool> {
    let index = CASES
        .iter()
        .position(|candidate| *candidate == name)
        .ok_or_else(|| io::Error::other("unknown N3 process case"))?;
    match std::env::var_os(CHILD_ROLE) {
        None => {
            OwnedCase::spawn(name, index)?.finish()?;
            Ok(true)
        }
        Some(role) if role == name => {
            eprintln!("{BODY_ENTERED} {name}");
            Ok(false)
        }
        Some(_) => Err(io::Error::other("invalid N3 exact child role")),
    }
}

/// Keep native error category/code without an unbounded localized message.
fn native_error(stage: &'static str, error: &io::Error) -> io::Error {
    io::Error::new(
        error.kind(),
        format!("N3 child {stage}; native_code={:?}", error.raw_os_error()),
    )
}

enum Observed {
    Exit(ExitStatus),
    Deadline,
}

#[derive(Eq, PartialEq)]
enum Completion {
    Passed,
    Semantic(String),
    Deadline,
}

/// One actual child and two exclusively created bounded capture files.
/// No pipe-reader or detached application thread is created in the parent.
struct OwnedCase {
    child: Option<Child>,
    directory: PathBuf,
    name: &'static str,
    started: Instant,
    reap_started: Option<Instant>,
    cleaned: bool,
}

impl OwnedCase {
    /// Spawn only the current test executable and one exact closed test identity.
    /// # Errors
    /// Preserve exclusive capture, executable, coverage admission or spawn failure.
    fn spawn(name: &'static str, index: usize) -> io::Result<Self> {
        let executable = std::env::current_exe()
            .map_err(|error| native_error("executable lookup failed", &error))?;
        let profile = std::env::var_os("LLVM_PROFILE_FILE");
        if profile
            .as_ref()
            .is_some_and(|value| value != COVERAGE_PROFILE)
        {
            return Err(io::Error::other("N3 child coverage path is not admitted"));
        }
        let directory = std::env::temp_dir().join(format!(
            "nevrotick-n3-series-{}-{index}",
            std::process::id()
        ));
        let builder = fs::DirBuilder::new();
        #[cfg(unix)]
        let builder = {
            use std::os::unix::fs::DirBuilderExt;
            let mut builder = builder;
            builder.mode(0o700);
            builder
        };
        builder
            .create(&directory)
            .map_err(|error| native_error("exclusive directory creation failed", &error))?;
        let mut owned = Self {
            child: None,
            directory,
            name,
            started: Instant::now(),
            reap_started: None,
            cleaned: false,
        };
        let capture = |name: &str| {
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(owned.directory.join(name))
                .map_err(|error| native_error("exclusive capture creation failed", &error))
        };
        let stdout = capture("stdout")?;
        let stderr = capture("stderr")?;
        let mut command = Command::new(executable);
        command
            .args([
                "--exact",
                name,
                "--test-threads=1",
                "--nocapture",
                "--color=never",
            ])
            .env_clear()
            .env(CHILD_ROLE, name)
            .current_dir(&owned.directory)
            .stdin(Stdio::null())
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr));
        if let Some(profile) = profile {
            command.env("LLVM_PROFILE_FILE", profile);
        }
        owned.child = Some(
            command
                .spawn()
                .map_err(|error| native_error("spawn failed", &error))?,
        );
        Ok(owned)
    }

    /// Observe exit independently of the child's runtime and expired timer loop.
    /// Apply the sampled wall budget only after observing a still-active child;
    /// this does not measure whether the child completed within that budget.
    /// Output metadata polling is a sampled bound; final reads are bounded too.
    /// # Errors
    /// Reject output overflow or preserve metadata/native wait failures.
    fn wait(&mut self) -> io::Result<Observed> {
        loop {
            // Sample time before exit state: a pause after an earlier None must
            // not invent expiry after a child has already completed.
            let budget_elapsed = self.started.elapsed() >= CHILD_WALL;
            if let Some(status) = self
                .child
                .as_mut()
                .ok_or_else(|| io::Error::other("missing owned N3 child"))?
                .try_wait()
                .map_err(|error| native_error("exit observation failed", &error))?
            {
                return Ok(Observed::Exit(status));
            }
            if budget_elapsed {
                return Ok(Observed::Deadline);
            }
            for name in ["stdout", "stderr"] {
                let bytes = fs::metadata(self.directory.join(name))
                    .map_err(|error| native_error("capture metadata failed", &error))?
                    .len();
                if bytes > OUTPUT_LIMIT {
                    return Err(io::Error::other("N3 child sampled output exceeds cap"));
                }
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    /// Kill/reap only this retained child before releasing any capture ownership.
    /// # Errors
    /// Preserve kill/wait failure. All finish/cleanup/Drop calls share one reap budget.
    fn stop(&mut self) -> io::Result<()> {
        let Some(child) = self.child.as_mut() else {
            return Ok(());
        };
        if child
            .try_wait()
            .map_err(|error| native_error("initial reap observation failed", &error))?
            .is_some()
        {
            return Ok(());
        }
        let began = *self.reap_started.get_or_insert_with(Instant::now);
        if began.elapsed() >= REAP_WALL {
            return Err(io::Error::other(
                "N3 owned child global reap deadline exceeded",
            ));
        }
        if let Err(error) = child.kill()
            && child
                .try_wait()
                .map_err(|wait| native_error("kill-race observation failed", &wait))?
                .is_none()
        {
            return Err(native_error("kill failed", &error));
        }
        loop {
            if child
                .try_wait()
                .map_err(|error| native_error("reap observation failed", &error))?
                .is_some()
            {
                return Ok(());
            }
            if began.elapsed() >= REAP_WALL {
                return Err(io::Error::other(
                    "N3 owned child global reap deadline exceeded",
                ));
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    /// Read a closed capture after actual reap without printing its payload.
    /// # Errors
    /// Reject final overflow/UTF-8 or preserve native open/read failure.
    fn read(&self, name: &str) -> io::Result<String> {
        if !matches!(name, "stdout" | "stderr") {
            return Err(io::Error::other("invalid N3 capture name"));
        }
        let mut bytes = Vec::new();
        File::open(self.directory.join(name))
            .map_err(|error| native_error("capture open failed", &error))?
            .take(OUTPUT_LIMIT + 1)
            .read_to_end(&mut bytes)
            .map_err(|error| native_error("capture read failed", &error))?;
        if u64::try_from(bytes.len()).map_err(|_| io::Error::other("invalid N3 capture size"))?
            > OUTPUT_LIMIT
        {
            return Err(io::Error::other("N3 final capture exceeds cap"));
        }
        String::from_utf8(bytes).map_err(|_| io::Error::other("N3 capture was not UTF-8"))
    }

    /// Reject zero/wrong selection and classify only the exact reached child.
    /// # Errors
    /// Preserve monitoring failure or reject an unproven body/selection/native exit.
    fn inspect(&self, observed: io::Result<Observed>) -> io::Result<Completion> {
        let observed = observed?;
        let stdout = self.read("stdout")?;
        let stderr = self.read("stderr")?;
        let selected_once = stdout
            .lines()
            .filter(|line| line.starts_with("running "))
            .eq(["running 1 test"]);
        let entered_marker = format!("{BODY_ENTERED} {}", self.name);
        let entered = stderr.lines().next() == Some(entered_marker.as_str());
        let expected_prefix = format!("test {} ...", self.name);
        let named_once = stdout
            .lines()
            .filter(|line| line.starts_with(&expected_prefix))
            .count()
            == 1;
        if !selected_once || !entered {
            return Err(io::Error::other(
                "N3 exact child selection/body was not established",
            ));
        }
        let Observed::Exit(status) = observed else {
            return Ok(Completion::Deadline);
        };
        if !named_once {
            return Err(io::Error::other(
                "N3 exact child named outcome was not established",
            ));
        }
        let rows: Vec<_> = stdout
            .lines()
            .filter(|line| line.starts_with("test "))
            .collect();
        if let [test, summary] = rows.as_slice() {
            if status.success()
                && stderr.lines().eq([entered_marker.as_str()])
                && *test == format!("test {} ... ok", self.name)
                && summary
                    .starts_with("test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; ")
                && summary.contains(" filtered out; finished in ")
            {
                return Ok(Completion::Passed);
            }
            if status.code() == Some(101)
                && *test == format!("test {} ... FAILED", self.name)
                && summary
                    .starts_with("test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; ")
                && summary.contains(" filtered out; finished in ")
            {
                if let Some(evidence) = semantic_evidence(&stderr, self.name) {
                    return Ok(Completion::Semantic(evidence));
                }
                return Err(io::Error::other(
                    "N3 named child FAILED/101 lacked assertion or semantic evidence; bounded cause captures retained",
                ));
            }
        }
        Err(io::Error::other(
            "N3 exact child native result was not a named test outcome",
        ))
    }

    /// Remove only the two fixed owned captures and empty directory after reap.
    /// # Errors
    /// Preserve cleanup failure; no recursive or wildcard deletion is performed.
    fn cleanup(&mut self) -> io::Result<()> {
        self.stop()?;
        self.child = None;
        for name in ["stdout", "stderr"] {
            match fs::remove_file(self.directory.join(name)) {
                Ok(()) => (),
                Err(error) if error.kind() == io::ErrorKind::NotFound => (),
                Err(error) => return Err(native_error("capture removal failed", &error)),
            }
        }
        fs::remove_dir(&self.directory)
            .map_err(|error| native_error("owned directory removal failed", &error))?;
        self.cleaned = true;
        Ok(())
    }

    /// Keep bounded cause captures for an unclassified failure after actual reap.
    /// # Errors
    /// Preserve reap/open/truncation failure instead of claiming retained evidence.
    fn retain_captures(&mut self) -> io::Result<()> {
        self.stop()?;
        self.child = None;
        for name in ["stdout", "stderr"] {
            let file = OpenOptions::new()
                .write(true)
                .open(self.directory.join(name))
                .map_err(|error| native_error("cause retention open failed", &error))?;
            if file
                .metadata()
                .map_err(|error| native_error("cause metadata failed", &error))?
                .len()
                > OUTPUT_LIMIT
            {
                file.set_len(OUTPUT_LIMIT)
                    .map_err(|error| native_error("cause retention bound failed", &error))?;
            }
        }
        // Settled ownership: only fixed bounded files remain, within the outer
        // owned namespace. They are deliberately retained, never printed raw.
        self.cleaned = true;
        eprintln!(
            "N3_HARNESS_CAUSE_CAPTURE case={} parent={} limit_bytes={OUTPUT_LIMIT}",
            self.name,
            std::process::id()
        );
        Ok(())
    }

    /// Produce an intended assertion only after exact child exit and cleanup.
    /// # Errors
    /// Keep setup/monitoring/selection/cleanup errors distinct from a reached timeout.
    /// # Panics
    /// Fail reached noncompletion or a qualified independent fact/assertion/product panic.
    fn finish(mut self) -> io::Result<()> {
        let observed = self.wait();
        let stopped = self.stop();
        let inspected = stopped.and_then(|()| self.inspect(observed));
        let cleanup = if inspected.is_err() {
            self.retain_captures()
        } else {
            self.cleanup()
        };
        let completion = match (inspected, cleanup) {
            (Ok(completion), Ok(())) => completion,
            (Err(primary), Ok(())) => return Err(primary),
            (Ok(_), Err(cleanup)) => return Err(cleanup),
            (Err(primary), Err(cleanup)) => {
                return Err(io::Error::other(format!(
                    "{primary}; N3 owned cleanup also failed: {cleanup}"
                )));
            }
        };
        assert!(
            completion != Completion::Deadline,
            "N3 series owner noncompletion assertion: exact child entered, independent watchdog expired, owned child exit confirmed and reaped"
        );
        if let Completion::Semantic(evidence) = &completion {
            eprintln!("N3_SEMANTIC_EVIDENCE {evidence}");
        }
        assert!(
            completion == Completion::Passed,
            "N3 exact child semantic failure assertion"
        );
        Ok(())
    }
}

impl Drop for OwnedCase {
    /// Bound fallback cleanup on partial setup or unwinding. The enclosing owned
    /// PID namespace retains final custody if native kill/reap itself fails.
    fn drop(&mut self) {
        if !self.cleaned && self.cleanup().is_err() {
            eprintln!("N3_HARNESS_INCOMPLETE");
            eprintln!("N3_HARNESS_CAUSE owned process cleanup incomplete");
        }
    }
}

/// Accept only an explicit static fact or this named test's numeric panic site.
/// Native `Result::Err` messages are never interpreted as semantic evidence.
fn semantic_evidence(stderr: &str, name: &str) -> Option<String> {
    if stderr.lines().any(|line| line == "N3_HARNESS_INCOMPLETE") {
        return None;
    }
    let errors: Vec<_> = stderr
        .lines()
        .filter(|line| line.starts_with("Error: "))
        .collect();
    let panics = stderr.matches(" panicked at ").count();
    if stderr
        .lines()
        .filter(|line| *line == SEMANTIC_FAILED)
        .count()
        > 1
    {
        return None;
    }
    let mut lines = stderr.lines();
    while let Some(line) = lines.next() {
        if line == SEMANTIC_FAILED {
            let fact = lines.next()?.strip_prefix("N3_SEMANTIC_FACT ")?;
            let expected_error = format!("Error: Custom {{ kind: Other, error: {fact:?} }}");
            if fact.len() <= 160
                && fact.is_ascii()
                && !fact.bytes().any(|byte| byte.is_ascii_control())
                && stderr
                    .lines()
                    .filter(|line| line.starts_with("N3_SEMANTIC_FACT "))
                    .count()
                    == 1
                && panics == 0
                && errors.as_slice() == [expected_error.as_str()]
            {
                return Some(format!("independent fact: {fact}"));
            }
            return None;
        }
    }
    if !errors.is_empty() || panics != 1 {
        return None;
    }
    let named_thread = format!("thread '{name}'");
    for line in stderr
        .lines()
        .filter(|line| line.starts_with(&named_thread))
    {
        let (_, location) = line.split_once(" panicked at ")?;
        let mut parts = location.trim_end_matches(':').rsplitn(3, ':');
        let column = parts.next()?.parse::<u32>().ok()?;
        let row = parts.next()?.parse::<u32>().ok()?;
        let path = match parts.next()? {
            // This is the one spelling emitted for the path-included private test.
            "crates/nevrotick-engine/src/../tests/private/series.rs" => {
                "crates/nevrotick-engine/tests/private/series.rs"
            }
            path => path,
        };
        let (source, kind) = [
            (
                "crates/nevrotick-engine/tests/private/series.rs",
                "test assertion/panic",
            ),
            (
                "crates/nevrotick-engine/tests/series.rs",
                "test assertion/panic",
            ),
            ("crates/nevrotick-engine/src/lib.rs", "product panic"),
            ("crates/nevrotick-engine/src/series.rs", "product panic"),
            (
                "crates/nevrotick-engine/src/measurement.rs",
                "product panic",
            ),
            ("crates/nevrotick-engine/src/relative.rs", "product panic"),
        ]
        .into_iter()
        .find(|(source, _)| path == *source)?;
        if row != 0 && column != 0 {
            return Some(format!("{kind} at {source}:{row}:{column}"));
        }
    }
    None
}

/// Admit only the named test's exact source spellings and retain incomplete errors.
/// # Panics
/// Require canonical evidence for the two admitted paths and reject every control.
#[test]
fn semantic_evidence_accepts_only_exact_named_admitted_sites() {
    let name = "series::tests::setup_and_registration_failures_preserve_actual_prior_completion";
    let canonical = "crates/nevrotick-engine/tests/private/series.rs";
    let compiler_alias = "crates/nevrotick-engine/src/../tests/private/series.rs";
    let panic_at = |thread: &str, path: &str| {
        format!(
            "thread '{thread}' (7) panicked at {path}:23:7:\nassertion `left == right` failed\n"
        )
    };
    for path in [canonical, compiler_alias] {
        assert_eq!(
            semantic_evidence(&panic_at(name, path), name).as_deref(),
            Some("test assertion/panic at crates/nevrotick-engine/tests/private/series.rs:23:7")
        );
    }
    let fact = "independent product condition changed";
    let semantic = format!(
        "{SEMANTIC_FAILED}\nN3_SEMANTIC_FACT {fact}\nError: Custom {{ kind: Other, error: {fact:?} }}\n"
    );
    assert_eq!(
        semantic_evidence(&semantic, name).as_deref(),
        Some("independent fact: independent product condition changed")
    );
    for rejected in [
        panic_at("foreign::test", canonical),
        panic_at(
            name,
            "/foreign/crates/nevrotick-engine/tests/private/series.rs",
        ),
        panic_at(
            name,
            "crates/nevrotick-engine/src/../../crates/nevrotick-engine/tests/private/series.rs",
        ),
        panic_at(
            name,
            "crates/nevrotick-engine/src/../../tests/private/series.rs",
        ),
        String::from("Error: InternalState\n"),
        String::from("Error: Elapsed(())\n"),
        String::from("Error: Os { code: 5, kind: PermissionDenied, message: \"denied\" }\n"),
        String::from("test result: FAILED. 0 passed; 1 failed; exit code: 101\n"),
        format!("N3_HARNESS_INCOMPLETE\n{}", panic_at(name, canonical)),
        format!("N3_HARNESS_INCOMPLETE\n{}", panic_at(name, compiler_alias)),
        format!("{}Error: Arithmetic\n", panic_at(name, canonical)),
        format!(
            "{}{}",
            panic_at(name, canonical),
            panic_at("foreign::worker", canonical)
        ),
        format!("{semantic}Error: Arithmetic\n"),
        format!("{semantic}{}", panic_at(name, canonical)),
        format!("{semantic}N3_SEMANTIC_FACT foreign fact\n"),
    ] {
        assert!(semantic_evidence(&rejected, name).is_none());
    }
}
