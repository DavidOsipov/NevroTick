//! Test-only Windows allocation observer for the admitted `time-domain` library.
//!
//! This is an external consumer, not a workspace member or shipping component.
//! Compile only through the admitted `x86_64` MSVC route with the frozen library,
//! dependency artifacts, target protections and reviewed source. The external
//! runner must independently attach compiler/std/profile/linker/SDK identities,
//! build commands, artifact hashes, Windows/token identity and native exits.
//! After argument validation, an exact UTF-8 BOM plus ASCII `G` gates the workload.
//! The supervisor must inspect the actual child token before sending the final `G`.
//! Waiting occurs before every measured interval and invokes no product function.
//!
//! Four positional arguments are required: candidate SHA-256, observer-source
//! SHA-256, external build-receipt SHA-256, and `debug`, `release` or
//! `verify-release`. These are bounded runner claims, not self-authentication.
//! No compiler identity is inferred from those claims. The runner must verify
//! them and hash the executable before and after the ordinary-token native run.
//!
//! Metrics cover observed global allocator requests and logical live requested
//! payload at completed callbacks in this exact instrumented artifact. They do
//! not cover allocator overhead/caches, native realloc transient peaks, RSS,
//! allocations outside this wrapper, or universal source-level allocation counts.
//! Allocation elision/merging remains permitted; `black_box` is best effort.
//! Calibration failure or inconsistent counters makes the observation inconclusive.
//! No elevation, debugger configuration, registry, file, network or clock API is
//! used. Stdout is written only after all measurement intervals have completed.
//!
//! SPDX-License-Identifier: GPL-2.0-only
#![deny(warnings, unsafe_code, unsafe_op_in_unsafe_fn, missing_docs)]
#![deny(unfulfilled_lint_expectations)]
#![deny(clippy::all, clippy::pedantic)]
#![deny(
    clippy::missing_docs_in_private_items,
    clippy::undocumented_unsafe_blocks
)]
#![deny(clippy::multiple_unsafe_ops_per_block, clippy::as_conversions)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[cfg(not(all(target_os = "windows", target_arch = "x86_64", target_env = "msvc")))]
compile_error!("This test-only observer requires x86_64-pc-windows-msvc.");

use std::hint::black_box;
use std::io::{self, Read, Write};
use std::process::ExitCode;

use time_domain::{DomainError, EraWindow};

/// Narrow Windows-only adapter; accounting never changes System's ownership or
/// allocation outcome. No callback allocates, logs, locks or unwinds.
#[cfg(windows)]
mod windows_allocator {
    use core::sync::atomic::{AtomicBool, AtomicU64, Ordering::SeqCst};
    use std::alloc::{GlobalAlloc, Layout, System};

    /// Allocator installed only in this external Windows test process.
    pub(super) struct Observer;

    /// Number of direct alloc requests, including null results.
    static ALLOC_REQUESTS: AtomicU64 = AtomicU64::new(0);
    /// Number of direct `alloc_zeroed` requests, including null results.
    static ZEROED_REQUESTS: AtomicU64 = AtomicU64::new(0);
    /// Number of direct realloc requests, including null results.
    static REALLOC_REQUESTS: AtomicU64 = AtomicU64::new(0);
    /// Sum of requested sizes across all allocation/reallocation attempts.
    static REQUESTED_BYTES: AtomicU64 = AtomicU64::new(0);
    /// Number of non-null allocation/reallocation results.
    static SUCCESSFUL_REQUESTS: AtomicU64 = AtomicU64::new(0);
    /// Sum of new sizes for non-null allocation/reallocation results.
    static SUCCESSFUL_BYTES: AtomicU64 = AtomicU64::new(0);
    /// Number of null allocation/reallocation results.
    static FAILED_REQUESTS: AtomicU64 = AtomicU64::new(0);
    /// Number of direct dealloc callbacks.
    static DEALLOCATIONS: AtomicU64 = AtomicU64::new(0);
    /// Continuously tracked logical live requested payload, never phase-reset.
    static LIVE_BYTES: AtomicU64 = AtomicU64::new(0);
    /// Largest logical live payload since the latest quiescent begin operation.
    static PEAK_BYTES: AtomicU64 = AtomicU64::new(0);
    /// Sticky accounting failure; never changes delegated allocator behavior.
    static COUNTER_ERROR: AtomicBool = AtomicBool::new(false);

    /// Fixed-size atomic snapshot; the single-threaded fixture supplies quiescence.
    #[derive(Clone, Copy)]
    pub(super) struct Snapshot {
        /// Cumulative direct allocation attempts.
        pub(super) alloc_requests: u64,
        /// Cumulative zero-initialized allocation attempts.
        pub(super) zeroed_requests: u64,
        /// Cumulative reallocation attempts.
        pub(super) realloc_requests: u64,
        /// Cumulative requested bytes, including failed requests.
        pub(super) requested_bytes: u64,
        /// Cumulative requests returning a non-null pointer.
        pub(super) successful_requests: u64,
        /// Cumulative new sizes of successful requests; not net growth.
        pub(super) successful_bytes: u64,
        /// Cumulative requests returning null.
        pub(super) failed_requests: u64,
        /// Cumulative direct dealloc callbacks.
        pub(super) deallocations: u64,
        /// Current logical live requested payload bytes.
        pub(super) live_bytes: u64,
        /// Logical live requested-payload high-water at callback boundaries.
        pub(super) peak_bytes: u64,
        /// Sticky arithmetic/inconsistent-accounting failure.
        pub(super) counter_error: bool,
    }

    impl Snapshot {
        /// Empty stack-array initializer; never used as an observed sample.
        pub(super) const EMPTY: Self = Self {
            alloc_requests: 0,
            zeroed_requests: 0,
            realloc_requests: 0,
            requested_bytes: 0,
            successful_requests: 0,
            successful_bytes: 0,
            failed_requests: 0,
            deallocations: 0,
            live_bytes: 0,
            peak_bytes: 0,
            counter_error: false,
        };
    }

    /// Add without panicking; wrapping invalidates evidence through a sticky flag.
    fn add(counter: &AtomicU64, amount: u64) -> u64 {
        let previous = counter.fetch_add(amount, SeqCst);
        if let Some(current) = previous.checked_add(amount) {
            current
        } else {
            COUNTER_ERROR.store(true, SeqCst);
            0
        }
    }

    /// Subtract without panicking; underflow invalidates the entire observation.
    fn subtract_live(amount: u64) {
        let previous = LIVE_BYTES.fetch_sub(amount, SeqCst);
        if previous < amount {
            COUNTER_ERROR.store(true, SeqCst);
        }
    }

    /// Convert a layout size without a truncating cast or a panic in callbacks.
    fn size_bytes(size: usize) -> u64 {
        if let Ok(value) = u64::try_from(size) {
            value
        } else {
            COUNTER_ERROR.store(true, SeqCst);
            0
        }
    }

    /// Record one request and its requested size before unchanged delegation.
    fn request(counter: &AtomicU64, size: u64) {
        add(counter, 1);
        add(&REQUESTED_BYTES, size);
    }

    /// Record successful request traffic independently from net live-byte change.
    fn success(size: u64) {
        add(&SUCCESSFUL_REQUESTS, 1);
        add(&SUCCESSFUL_BYTES, size);
    }

    /// Increase logical live payload and record its callback-boundary high-water.
    fn grow_live(size: u64) {
        let live = add(&LIVE_BYTES, size);
        let _previous_peak = PEAK_BYTES.fetch_max(live, SeqCst);
    }

    #[expect(
        unsafe_code,
        reason = "reviewed test-only Windows adapter forwards GlobalAlloc unchanged to System"
    )]
    // SAFETY: every method forwards exactly the caller's pointer/layout/size to
    // System and returns its unchanged result. The adapter never accesses user
    // memory, introduces alternate ownership, allocates recursively or unwinds.
    unsafe impl GlobalAlloc for Observer {
        /// Delegate a caller-valid nonzero layout unchanged and count its result.
        /// # Safety
        /// The caller must meet `GlobalAlloc`'s nonzero valid-layout contract.
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let size = size_bytes(layout.size());
            request(&ALLOC_REQUESTS, size);
            // SAFETY: the trait caller supplies System's identical valid layout.
            let pointer = unsafe { System.alloc(layout) };
            if pointer.is_null() {
                add(&FAILED_REQUESTS, 1);
            } else {
                success(size);
                grow_live(size);
            }
            pointer
        }

        /// Preserve zero-initialization by using System's matching operation.
        /// # Safety
        /// The caller must meet `GlobalAlloc`'s nonzero valid-layout contract.
        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            let size = size_bytes(layout.size());
            request(&ZEROED_REQUESTS, size);
            // SAFETY: forwarding the valid layout preserves the zeroed contract.
            let pointer = unsafe { System.alloc_zeroed(layout) };
            if pointer.is_null() {
                add(&FAILED_REQUESTS, 1);
            } else {
                success(size);
                grow_live(size);
            }
            pointer
        }

        /// Release the exact System-owned allocation once, with the same layout.
        /// # Safety
        /// The caller owns the live pointer and supplies its allocation layout.
        unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
            // SAFETY: the caller's pointer/layout are unchanged and owned by System.
            unsafe { System.dealloc(pointer, layout) };
            add(&DEALLOCATIONS, 1);
            subtract_live(size_bytes(layout.size()));
        }

        /// Delegate reallocation unchanged; a null result retains the old block.
        /// # Safety
        /// The caller supplies the live allocation/layout and legal nonzero size.
        unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            let old_bytes = size_bytes(layout.size());
            let new_bytes = size_bytes(new_size);
            request(&REALLOC_REQUESTS, new_bytes);
            // SAFETY: all ownership, layout and size inputs are passed unchanged.
            let replacement = unsafe { System.realloc(pointer, layout, new_size) };
            if replacement.is_null() {
                add(&FAILED_REQUESTS, 1);
            } else {
                success(new_bytes);
                if new_bytes >= old_bytes {
                    grow_live(new_bytes - old_bytes);
                } else {
                    subtract_live(old_bytes - new_bytes);
                }
            }
            replacement
        }
    }

    /// Snapshot only at the fixture's quiescent boundaries; allocate nothing.
    pub(super) fn snapshot() -> Snapshot {
        Snapshot {
            alloc_requests: ALLOC_REQUESTS.load(SeqCst),
            zeroed_requests: ZEROED_REQUESTS.load(SeqCst),
            realloc_requests: REALLOC_REQUESTS.load(SeqCst),
            requested_bytes: REQUESTED_BYTES.load(SeqCst),
            successful_requests: SUCCESSFUL_REQUESTS.load(SeqCst),
            successful_bytes: SUCCESSFUL_BYTES.load(SeqCst),
            failed_requests: FAILED_REQUESTS.load(SeqCst),
            deallocations: DEALLOCATIONS.load(SeqCst),
            live_bytes: LIVE_BYTES.load(SeqCst),
            peak_bytes: PEAK_BYTES.load(SeqCst),
            counter_error: COUNTER_ERROR.load(SeqCst),
        }
    }

    /// Begin a quiescent interval by resetting only its peak to existing live bytes.
    pub(super) fn begin() -> Snapshot {
        PEAK_BYTES.store(LIVE_BYTES.load(SeqCst), SeqCst);
        snapshot()
    }
}

/// System-backed observer is confined to this ordinary Windows test executable.
#[cfg(windows)]
#[global_allocator]
static ALLOCATOR: windows_allocator::Observer = windows_allocator::Observer;

/// One fixed constructor case, with no attacker-sized inputs or allocated metadata.
#[derive(Clone, Copy)]
enum Case {
    /// Ordered in-envelope accepted neighbor.
    Valid,
    /// Earliest endpoint is below the numeric envelope.
    EarliestInvalid,
    /// Latest endpoint is above the numeric envelope.
    LatestInvalid,
    /// Both endpoints violate their actual mandatory garde rules.
    BothInvalid,
    /// Both fields are valid but their cross-field ordering is inverted.
    Inverted,
}

impl Case {
    /// Fixed allowlisted JSON label; no rejected value is formatted.
    const fn name(self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::EarliestInvalid => "earliest_invalid",
            Self::LatestInvalid => "latest_invalid",
            Self::BothInvalid => "both_invalid",
            Self::Inverted => "inverted",
        }
    }

    /// Explicit literal raw inputs; this function never invokes the product.
    const fn inputs(self) -> (i128, i128) {
        match self {
            Self::Valid => (-1, 1),
            Self::EarliestInvalid => (i128::MIN, 0),
            Self::LatestInvalid => (0, i128::MAX),
            Self::BothInvalid => (i128::MIN, i128::MAX),
            Self::Inverted => (1, 0),
        }
    }

    /// Inspect only the real fixed-size result with no allocation or assertion.
    fn result_matches(self, result: Result<EraWindow, DomainError>) -> bool {
        match (self, result) {
            (Self::Valid, Ok(window)) => {
                window.earliest().ticks() == -1 && window.latest().ticks() == 1
            }
            (Self::Valid, Err(_)) | (_, Ok(_)) => false,
            (_, Err(error)) => error == DomainError::InvalidEraWindow,
        }
    }
}

/// Frozen case order; cold observations occur before any product warmup/control.
const CASES: [Case; 5] = [
    Case::Valid,
    Case::EarliestInvalid,
    Case::LatestInvalid,
    Case::BothInvalid,
    Case::Inverted,
];
/// Increasing bounded repeat counts; every raw batch sample is retained on stack.
const BATCHES: [u32; 4] = [1, 32, 256, 4096];
/// Warmup calls per constructor case, after the first observations are captured.
const WARMUPS: u32 = 64;
/// Safe retained-payload control size, never leaked or left to process reclamation.
const BOX_BYTES: usize = 4096;

/// One real constructor interval; cold means first case observation, not a claim
/// of a fresh process per case. Cases execute in the reported shared-process order.
#[derive(Clone, Copy)]
struct Sample {
    /// Frozen case label and independent expected result owner.
    case: Case,
    /// Whether this precedes all product warmups and held-Box product controls.
    cold: bool,
    /// Exact number of real constructor calls in this interval.
    calls: u32,
    /// Cumulative accounting before entry and peak reset.
    before: windows_allocator::Snapshot,
    /// Cumulative accounting immediately after returns/report destruction.
    after: windows_allocator::Snapshot,
    /// Fixed-scalar count of observed unexpected product results.
    mismatches: u32,
}

impl Sample {
    /// Fixed-size array initializer, replaced before any report is emitted.
    const EMPTY: Self = Self {
        case: Case::Valid,
        cold: false,
        calls: 0,
        before: windows_allocator::Snapshot::EMPTY,
        after: windows_allocator::Snapshot::EMPTY,
        mismatches: 0,
    };
}

/// Bounded constructor loop: black-box raw inputs/results and retain only a scalar
/// correctness count. No assertion, formatting, allocation or I/O is performed by
/// the harness inside the loop; the actual garde path is measured unchanged.
fn call_product(case: Case, calls: u32) -> u32 {
    let mut mismatches = 0;
    for _ in 0..calls {
        let (earliest, latest) = black_box(case.inputs());
        let result = black_box(EraWindow::new(black_box(earliest), black_box(latest)));
        mismatches += u32::from(!case.result_matches(result));
    }
    mismatches
}

/// Bracket actual product calls; all report construction/printing happens later.
fn observe(case: Case, calls: u32, cold: bool) -> Sample {
    let before = windows_allocator::begin();
    let mismatches = call_product(case, calls);
    let after = windows_allocator::snapshot();
    Sample {
        case,
        cold,
        calls,
        before,
        after,
        mismatches,
    }
}

/// Safe held-Box calibration including an optional fixed valid product workload.
/// Every Box remains owned across the held snapshot and is explicitly dropped.
#[derive(Clone, Copy)]
struct BoxCalibration {
    /// Baseline before safe allocation.
    before: windows_allocator::Snapshot,
    /// Snapshot while the touched safe Box is still owned by the fixture.
    held: windows_allocator::Snapshot,
    /// Snapshot after optional real product calls, while the Box remains held.
    after_product: windows_allocator::Snapshot,
    /// Snapshot after explicit drop, before any process teardown.
    dropped: windows_allocator::Snapshot,
    /// Whether actual safe content consumption matched the fixed bytes.
    contents_match: bool,
    /// Exact real valid constructor calls while retaining the known allocation.
    product_calls: u32,
    /// Fixed-scalar unexpected result count from that optional workload.
    product_mismatches: u32,
}

impl BoxCalibration {
    /// Require observable positive retention and exact matching release in this
    /// artifact; elided/non-discriminating control is inconclusive, never a pass.
    fn discriminates(&self) -> bool {
        let retained = self.held.live_bytes.checked_sub(self.before.live_bytes);
        self.contents_match
            && retained.is_some_and(|bytes| bytes >= 4096)
            && self.held.successful_requests > self.before.successful_requests
            && self.after_product.live_bytes == self.held.live_bytes
            && self.dropped.live_bytes == self.before.live_bytes
            && self.dropped.deallocations > self.held.deallocations
            && self.product_mismatches == 0
            && !self.dropped.counter_error
    }
}

/// Calibrate only with safe allocation ownership; zero product calls are used
/// before the cold observations and after all primary measurement samples.
fn calibrate_box(product_calls: u32) -> BoxCalibration {
    let before = windows_allocator::begin();
    let held_box = black_box(Box::new(black_box([0x5a_u8; BOX_BYTES])));
    let contents_match = black_box(&held_box).iter().all(|byte| *byte == 0x5a);
    let held = windows_allocator::snapshot();
    let product_mismatches = call_product(Case::Valid, product_calls);
    let after_product = windows_allocator::snapshot();
    drop(black_box(held_box));
    let dropped = windows_allocator::snapshot();
    BoxCalibration {
        before,
        held,
        after_product,
        dropped,
        contents_match,
        product_calls,
        product_mismatches,
    }
}

/// Safe zero/grow/shrink/drop calibration for distinct forwarding operations.
#[derive(Clone, Copy)]
struct VecCalibration {
    /// Baseline before the zero-initialized vector is created.
    before: windows_allocator::Snapshot,
    /// Snapshot after the zero-filled vector is owned and read.
    zeroed: windows_allocator::Snapshot,
    /// Snapshot after a larger initialized vector is owned and read.
    grown: windows_allocator::Snapshot,
    /// Snapshot after safe truncation and a shrink request.
    shrunk: windows_allocator::Snapshot,
    /// Snapshot after explicit vector drop.
    dropped: windows_allocator::Snapshot,
    /// Whether every initialized byte was read and matched its known value.
    contents_match: bool,
}

impl VecCalibration {
    /// Require actual zeroed and realloc observations plus release; a compiler
    /// that removes these calls produces inconclusive forwarding calibration.
    fn discriminates(&self) -> bool {
        self.contents_match
            && self.zeroed.zeroed_requests > self.before.zeroed_requests
            && self.grown.realloc_requests > self.zeroed.realloc_requests
            && self.shrunk.realloc_requests > self.grown.realloc_requests
            && self.zeroed.live_bytes > self.before.live_bytes
            && self.grown.live_bytes > self.zeroed.live_bytes
            && self.shrunk.live_bytes < self.grown.live_bytes
            && self.dropped.live_bytes == self.before.live_bytes
            && !self.dropped.counter_error
    }
}

/// Use only safe Vec operations; no raw allocation/pointer manipulation occurs.
fn calibrate_vec() -> VecCalibration {
    let before = windows_allocator::begin();
    let mut vector = black_box(vec![0_u8; black_box(128)]);
    let zero_contents = black_box(vector.as_slice()).iter().all(|byte| *byte == 0);
    let zeroed = windows_allocator::snapshot();
    vector.resize(black_box(640), 0x3c);
    let grown_contents = black_box(vector.as_slice())
        .iter()
        .skip(128)
        .all(|byte| *byte == 0x3c);
    let grown = windows_allocator::snapshot();
    vector.truncate(black_box(64));
    vector.shrink_to_fit();
    let shrunk_contents = black_box(vector.as_slice()).iter().all(|byte| *byte == 0);
    let shrunk = windows_allocator::snapshot();
    drop(black_box(vector));
    let dropped = windows_allocator::snapshot();
    VecCalibration {
        before,
        zeroed,
        grown,
        shrunk,
        dropped,
        contents_match: zero_contents && grown_contents && shrunk_contents,
    }
}

/// Completed fixed-size evidence, captured entirely before stdout formatting.
struct Report {
    /// Five cold samples followed by four increasing batches for each case.
    samples: [Sample; 25],
    /// Before-product, post-cold held-product, and after-product Box controls.
    boxes: [BoxCalibration; 3],
    /// Independent safe zero/grow/shrink/drop forwarding control.
    vector: VecCalibration,
    /// Unexpected product results across exactly 64 warmups per case.
    warmup_mismatches: u32,
    /// Last counter snapshot before output allocation can occur.
    final_counters: windows_allocator::Snapshot,
}

impl Report {
    /// All controls must discriminate and every interval must retain valid counters.
    fn conclusive(&self) -> bool {
        self.boxes.iter().all(BoxCalibration::discriminates)
            && self.vector.discriminates()
            && !self.final_counters.counter_error
            && self.samples.iter().all(|sample| {
                !sample.before.counter_error
                    && !sample.after.counter_error
                    && sample.after.requested_bytes >= sample.before.requested_bytes
                    && sample.after.successful_bytes >= sample.before.successful_bytes
            })
    }

    /// Correctness is a separate outcome from calibrated accounting validity.
    fn product_matches(&self) -> bool {
        self.warmup_mismatches == 0
            && self.samples.iter().all(|sample| sample.mismatches == 0)
            && self
                .boxes
                .iter()
                .all(|control| control.product_mismatches == 0)
    }

    /// Retention is observed before process exit, independently for every batch.
    fn stable_retention(&self) -> bool {
        self.samples
            .iter()
            .all(|sample| sample.before.live_bytes == sample.after.live_bytes)
    }

    /// Fixed allowlisted classification; missing/failed calibration is inconclusive.
    fn status(&self) -> &'static str {
        if !self.conclusive() {
            "inconclusive"
        } else if !self.product_matches() {
            "product_mismatch"
        } else if !self.stable_retention() {
            "observed_retained_delta"
        } else {
            "observed"
        }
    }
}

/// Complete a bounded single-threaded workload with no product call before cold
/// samples. The known held-Box product control follows all five cold samples.
fn run_observation() -> Report {
    let before_box = calibrate_box(0);
    let vector = calibrate_vec();
    let mut samples = [Sample::EMPTY; 25];
    for (slot, case) in samples.iter_mut().zip(CASES) {
        *slot = observe(case, 1, true);
    }
    let held_product_box = calibrate_box(32);
    let mut warmup_mismatches = 0;
    for case in CASES {
        warmup_mismatches += call_product(case, WARMUPS);
    }
    let mut next = 5;
    for case in CASES {
        for calls in BATCHES {
            samples[next] = observe(case, calls, false);
            next += 1;
        }
    }
    let after_box = calibrate_box(0);
    Report {
        samples,
        boxes: [before_box, held_product_box, after_box],
        vector,
        warmup_mismatches,
        final_counters: windows_allocator::snapshot(),
    }
}

/// Caller-supplied build label; debug-assertion state is separately observed.
enum BuildProfile {
    /// Ordinary non-optimized profile claimed by the external runner.
    Debug,
    /// Optimized profile with actual debug assertions disabled.
    Release,
    /// Optimized verification profile with actual debug assertions enabled.
    VerifyRelease,
}

impl BuildProfile {
    /// Return a fixed JSON-safe label without certifying optimization settings.
    const fn name(&self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Release => "release",
            Self::VerifyRelease => "verify-release",
        }
    }
}

/// Bounded unverified runner claims; only the attached external receipt can bind
/// these values to actual source, compiler, dependency artifacts and executable.
struct Identity {
    /// Frozen product-candidate claim; lower-case SHA-256 only.
    candidate_sha256: String,
    /// Exact observer source claim; lower-case SHA-256 only.
    observer_source_sha256: String,
    /// Hash of the external source/build/artifact identity receipt.
    build_receipt_sha256: String,
    /// Allowlisted claimed profile, cross-checked only against debug assertions.
    profile: BuildProfile,
}

/// Read one bounded Unicode command-line claim before measurement begins.
/// # Errors
/// Rejects missing, non-Unicode or oversized claims without echoing input.
fn argument(arguments: &mut std::env::ArgsOs) -> Result<String, &'static str> {
    let value = arguments.next().ok_or("missing_identity_argument")?;
    let value = value
        .into_string()
        .map_err(|_| "non_unicode_identity_argument")?;
    if value.len() > 64 {
        return Err("oversized_identity_argument");
    }
    Ok(value)
}

/// Admit exactly one JSON-safe hexadecimal SHA-256 claim, without trusting it.
/// # Errors
/// Rejects wrong length, upper-case hex and non-hexadecimal characters.
fn hash_argument(arguments: &mut std::env::ArgsOs) -> Result<String, &'static str> {
    let value = argument(arguments)?;
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err("invalid_sha256_claim");
    }
    Ok(value)
}

/// Validate fixed-size identity labels before any measured product execution.
/// # Errors
/// Rejects malformed claims, extra arguments and mismatched debug-assertion state.
fn parse_identity() -> Result<Identity, &'static str> {
    let mut arguments = std::env::args_os();
    let _program = arguments.next();
    let candidate_sha256 = hash_argument(&mut arguments)?;
    let observer_source_sha256 = hash_argument(&mut arguments)?;
    let build_receipt_sha256 = hash_argument(&mut arguments)?;
    let profile = match argument(&mut arguments)?.as_str() {
        "debug" => BuildProfile::Debug,
        "release" => BuildProfile::Release,
        "verify-release" => BuildProfile::VerifyRelease,
        _ => return Err("invalid_profile_claim"),
    };
    let expected_assertions = !matches!(profile, BuildProfile::Release);
    if expected_assertions != cfg!(debug_assertions) {
        return Err("profile_debug_assertions_mismatch");
    }
    if arguments.next().is_some() {
        return Err("extra_identity_argument");
    }
    Ok(Identity {
        candidate_sha256,
        observer_source_sha256,
        build_receipt_sha256,
        profile,
    })
}

/// Print one raw fixed-size snapshot after all measurement has completed.
/// # Errors
/// Propagates stdout write failure; truncated output remains invalid evidence.
fn write_snapshot(output: &mut impl Write, value: windows_allocator::Snapshot) -> io::Result<()> {
    write!(
        output,
        "{{\"alloc_requests\":{},\"zeroed_requests\":{},\"realloc_requests\":{},\"requested_bytes\":{},\"successful_requests\":{},\"successful_requested_bytes\":{},\"failed_requests\":{},\"deallocations\":{},\"live_payload_bytes\":{},\"peak_live_payload_bytes\":{},\"counter_error\":{}}}",
        value.alloc_requests,
        value.zeroed_requests,
        value.realloc_requests,
        value.requested_bytes,
        value.successful_requests,
        value.successful_bytes,
        value.failed_requests,
        value.deallocations,
        value.live_bytes,
        value.peak_bytes,
        value.counter_error
    )
}

/// Print precise interval differences and raw snapshots; signed deltas cannot
/// wrap. All integer differences are representable in i128 from u64 snapshots.
/// # Errors
/// Propagates output failure, never fabricating a completed JSON receipt.
fn write_interval(
    output: &mut impl Write,
    before: windows_allocator::Snapshot,
    after: windows_allocator::Snapshot,
) -> io::Result<()> {
    write!(
        output,
        "{{\"alloc_requests\":{},\"zeroed_requests\":{},\"realloc_requests\":{},\"requested_bytes\":{},\"successful_requests\":{},\"successful_requested_bytes\":{},\"failed_requests\":{},\"deallocations\":{},\"live_baseline_bytes\":{},\"retained_delta_bytes\":{},\"peak_above_baseline_bytes\":{},\"before\":",
        i128::from(after.alloc_requests) - i128::from(before.alloc_requests),
        i128::from(after.zeroed_requests) - i128::from(before.zeroed_requests),
        i128::from(after.realloc_requests) - i128::from(before.realloc_requests),
        i128::from(after.requested_bytes) - i128::from(before.requested_bytes),
        i128::from(after.successful_requests) - i128::from(before.successful_requests),
        i128::from(after.successful_bytes) - i128::from(before.successful_bytes),
        i128::from(after.failed_requests) - i128::from(before.failed_requests),
        i128::from(after.deallocations) - i128::from(before.deallocations),
        before.live_bytes,
        i128::from(after.live_bytes) - i128::from(before.live_bytes),
        i128::from(after.peak_bytes) - i128::from(before.live_bytes)
    )?;
    write_snapshot(output, before)?;
    write!(output, ",\"after\":")?;
    write_snapshot(output, after)?;
    write!(output, "}}")
}

/// Print each calibration stage and its discriminating outcome without hiding
/// retained payload behind process cleanup or the product batch's own baseline.
/// # Errors
/// Propagates stdout failure.
fn write_box_calibration(output: &mut impl Write, value: &BoxCalibration) -> io::Result<()> {
    write!(
        output,
        "{{\"discriminates\":{},\"contents_match\":{},\"expected_payload_bytes\":4096,\"product_calls\":{},\"product_mismatches\":{},\"held\":",
        value.discriminates(),
        value.contents_match,
        value.product_calls,
        value.product_mismatches
    )?;
    write_interval(output, value.before, value.held)?;
    write!(output, ",\"after_product_while_held\":")?;
    write_interval(output, value.before, value.after_product)?;
    write!(output, ",\"after_explicit_drop\":")?;
    write_interval(output, value.before, value.dropped)?;
    write!(output, "}}")
}

/// Print the independent safe vector forwarding control's complete raw stages.
/// # Errors
/// Propagates stdout failure.
fn write_vec_calibration(output: &mut impl Write, value: &VecCalibration) -> io::Result<()> {
    write!(
        output,
        "{{\"discriminates\":{},\"contents_match\":{},\"zeroed\":",
        value.discriminates(),
        value.contents_match
    )?;
    write_interval(output, value.before, value.zeroed)?;
    write!(output, ",\"grown\":")?;
    write_interval(output, value.before, value.grown)?;
    write!(output, ",\"shrunk\":")?;
    write_interval(output, value.before, value.shrunk)?;
    write!(output, ",\"after_explicit_drop\":")?;
    write_interval(output, value.before, value.dropped)?;
    write!(output, "}}")
}

/// Emit only fixed labels, validated hexadecimal claims, scalar values and raw
/// snapshots. Formatting begins after the final captured allocator snapshot.
/// # Errors
/// Propagates stdout failure; external execution must retain native exit/output.
fn write_report(identity: &Identity, report: &Report) -> io::Result<()> {
    let mut output = io::stdout().lock();
    write!(
        output,
        "{{\"schema_version\":1,\"purpose\":\"test_only_time_domain_allocator_requests\",\"status\":\"{}\",\"runner_claims_unverified\":{{\"candidate_sha256\":\"{}\",\"observer_source_sha256\":\"{}\",\"build_receipt_sha256\":\"{}\",\"profile\":\"{}\"}},\"build_observations\":{{\"target\":\"x86_64-pc-windows-msvc\",\"debug_assertions\":{},\"external_build_identity_required\":true}},\"workload\":{{\"cases\":5,\"cold_calls_per_case\":1,\"warmups_per_case\":64,\"batches\":[1,32,256,4096],\"held_box_product_calls\":32,\"total_product_calls\":22282,\"warmup_mismatches\":{},\"cold_definition\":\"first_observation_per_case_in_shared_process_before_any_product_warmup\"}},\"calibrations\":{{\"before_product_box\":",
        report.status(),
        identity.candidate_sha256,
        identity.observer_source_sha256,
        identity.build_receipt_sha256,
        identity.profile.name(),
        cfg!(debug_assertions),
        report.warmup_mismatches
    )?;
    write_box_calibration(&mut output, &report.boxes[0])?;
    write!(output, ",\"held_box_product_after_cold\":")?;
    write_box_calibration(&mut output, &report.boxes[1])?;
    write!(output, ",\"after_product_box\":")?;
    write_box_calibration(&mut output, &report.boxes[2])?;
    write!(output, ",\"safe_vec\":")?;
    write_vec_calibration(&mut output, &report.vector)?;
    write!(output, "}},\"samples\":[")?;
    for (index, sample) in report.samples.iter().enumerate() {
        if index != 0 {
            write!(output, ",")?;
        }
        write!(
            output,
            "{{\"case\":\"{}\",\"phase\":\"{}\",\"calls\":{},\"product_mismatches\":{},\"metrics\":",
            sample.case.name(),
            if sample.cold { "cold" } else { "warm_batch" },
            sample.calls,
            sample.mismatches
        )?;
        write_interval(&mut output, sample.before, sample.after)?;
        write!(output, "}}")?;
    }
    write!(output, "],\"final_counters_before_output\":")?;
    write_snapshot(&mut output, report.final_counters)?;
    writeln!(
        output,
        ",\"limits\":[\"artifact_specific_observed_global_requests\",\"allocation_elision_and_merging_permitted\",\"black_box_best_effort\",\"logical_payload_peak_at_completed_callbacks\",\"excludes_os_heap_overhead_caches_rss_and_native_realloc_transient_peak\",\"excludes_allocations_outside_this_wrapper\",\"no_universal_zero_allocation_or_absence_of_leaks_claim\",\"no_appverifier_pageheap_or_release_qualification\"]}}"
    )
}

/// Read the exact four-byte fixture gate before any measured/product operation.
///
/// # Errors
/// Rejects a failed/closed stream or any sequence other than UTF-8 BOM plus ASCII
/// `G`. The admitted .NET Framework writer emits the BOM during process startup;
/// only its final `G`, sent after token capture, releases the product workload.
/// The supervisor owns the wait deadline and cleanup; no prefix is skipped.
fn wait_for_start() -> Result<(), &'static str> {
    let mut gate = [0_u8; 4];
    io::stdin()
        .read_exact(&mut gate)
        .map_err(|_| "start_gate_io_failure")?;
    if gate != [0xef, 0xbb, 0xbf, b'G'] {
        return Err("invalid_start_gate");
    }
    Ok(())
}

/// Run one bounded ordinary-user observation. Failure/inconclusive native exits
/// remain failures; no allocator-accounting result controls unsafe correctness.
fn main() -> ExitCode {
    let identity = match parse_identity() {
        Ok(identity) => identity,
        Err(code) => {
            let mut output = io::stdout().lock();
            let _write_result = writeln!(
                output,
                "{{\"schema_version\":1,\"status\":\"inconclusive\",\"error\":\"{code}\"}}"
            );
            return ExitCode::FAILURE;
        }
    };
    if let Err(code) = wait_for_start() {
        let mut output = io::stdout().lock();
        let _write_result = writeln!(
            output,
            "{{\"schema_version\":1,\"status\":\"inconclusive\",\"error\":\"{code}\"}}"
        );
        return ExitCode::FAILURE;
    }
    let report = run_observation();
    if write_report(&identity, &report).is_err() {
        return ExitCode::FAILURE;
    }
    if report.status() == "observed" {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
