//! Test-only requested-payload observer for the S1.1d exchange fixture.
//!
//! This external consumer is not a workspace member or shipping component.
//! Run only through the admitted isolated host analysis runner. The runner binds
//! actual source, compiler, dependency artifacts, commands, executable and exit.
//! No arguments, environment values or observer claims authenticate that binding.
//! All product measurements finish before output and before process teardown.
//! No network, Windows API, thread spawning or ambient clock is used here.
//!
//! Accounting adapts the local GPL-2.0-only time-domain Windows observer at
//! SHA-256 1bc88f146c677281c7b99865825945e9b084a146b6e65dc21fdd352a3258f7dc. Only its System allocator forwarding/accounting is
//! reused; the fixed workload and expected lifecycle results are authored here.
//! Original local authorship: David Osipov with AI assistance, 29 October 2026.
//! Measurements cover reached global-allocator calls and logical live requested
//! payload in this exact host artifact. They do not cover allocator overhead,
//! caches, RSS, native allocation APIs, Windows execution or universal absence
//! of allocations. Compiler allocation elision remains possible; `black_box` is
//! best effort, and non-discriminating positive controls invalidate the result.
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

#[cfg(not(all(target_os = "linux", target_arch = "x86_64", target_env = "gnu")))]
compile_error!("This isolated host observer requires x86_64-unknown-linux-gnu.");

use std::hint::black_box;
use std::io::{self, Write};
use std::net::SocketAddr;
use std::num::NonZeroU64;
use std::process::ExitCode;

use ntp_exchange::{
    Association, AssociationContext, AssociationGeneration, Binding, CancelError, ClockGeneration,
    ConfigurationGeneration, ContextError, ExpireError, FixtureEvent, Generations, PeerEraContext,
    ReceiptFacts, ReceiveError, RequestId, RequestTiming, SocketId, SourceId, WireToken,
};
use ntp_wire::FrameError;
use time_domain::{DomainError, EraWindow, MonotonicNs, UtcCoordinateQ32};

/// Narrow test-only adapter; accounting preserves System's ownership and result.
/// No callback allocates, logs, locks, accesses user memory or unwinds.
mod allocator {
    use core::sync::atomic::{AtomicBool, AtomicU64, Ordering::SeqCst};
    use std::alloc::{GlobalAlloc, Layout, System};

    /// Allocator installed only in this external host test process.
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
        reason = "reviewed test-only host adapter forwards GlobalAlloc unchanged to System"
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

/// System-backed allocator belongs only to this external host fixture.
#[global_allocator]
static ALLOCATOR: allocator::Observer = allocator::Observer;

/// Every coordinate and receipt variant is constructed before measurements.
#[derive(Clone, Copy)]
struct Fixture {
    /// Original immutable source/socket/endpoint identity.
    binding: Binding,
    /// Initial zero-generation context.
    context: AssociationContext,
    /// Checked local and peer coordinate of exactly one Q32 tick.
    utc: UtcCoordinateQ32,
    /// Both peer windows admit only that coordinate.
    eras: PeerEraContext,
    /// Checked send=10, deadline=20 fixture nanoseconds.
    timing: RequestTiming,
    /// Normal arrival=11, processing=12, mapping-generation=0 receipt.
    receipt: ReceiptFacts,
    /// Wrong source, socket, endpoint, mapping, arrival, processing and deadline.
    bad_receipts: [ReceiptFacts; 7],
}

impl Fixture {
    /// Construct all fixed validated dependencies before any accounting interval.
    /// # Errors
    /// Rejects an unexpected constructor failure without producing observations.
    fn new() -> Result<Self, &'static str> {
        let source = SourceId::new(NonZeroU64::MIN);
        let socket = SocketId::new(NonZeroU64::MIN);
        let endpoint = SocketAddr::from(([127, 0, 0, 1], 123));
        let binding = Binding::new(source, socket, endpoint);
        let clock = ClockGeneration::new(0);
        let context = AssociationContext::new(
            binding,
            Generations::new(
                ConfigurationGeneration::new(0),
                AssociationGeneration::new(0),
                clock,
            ),
        );
        let utc = UtcCoordinateQ32::new(1).map_err(|_| "coordinate preparation failed")?;
        let window = EraWindow::new(1, 1).map_err(|_| "window preparation failed")?;
        let eras = PeerEraContext::new(window, window);
        let timing = RequestTiming::new(MonotonicNs::new(10), MonotonicNs::new(20))
            .map_err(|_| "timing preparation failed")?;
        let arrival = MonotonicNs::new(11);
        let processing = MonotonicNs::new(12);
        let receipt = ReceiptFacts::new(binding, clock, arrival, processing, utc);
        let bad_receipts = [
            ReceiptFacts::new(
                Binding::new(SourceId::new(NonZeroU64::MAX), socket, endpoint),
                clock,
                arrival,
                processing,
                utc,
            ),
            ReceiptFacts::new(
                Binding::new(source, SocketId::new(NonZeroU64::MAX), endpoint),
                clock,
                arrival,
                processing,
                utc,
            ),
            ReceiptFacts::new(
                Binding::new(source, socket, SocketAddr::from(([127, 0, 0, 1], 124))),
                clock,
                arrival,
                processing,
                utc,
            ),
            ReceiptFacts::new(binding, ClockGeneration::new(1), arrival, processing, utc),
            ReceiptFacts::new(binding, clock, MonotonicNs::new(9), processing, utc),
            ReceiptFacts::new(binding, clock, arrival, MonotonicNs::new(10), utc),
            ReceiptFacts::new(binding, clock, arrival, MonotonicNs::new(20), utc),
        ];
        Ok(Self {
            binding,
            context,
            utc,
            eras,
            timing,
            receipt,
            bad_receipts,
        })
    }
}

/// Fixed workload identities; only their declared loop bounds are executed.
#[derive(Clone, Copy)]
enum Workload {
    /// Each hostile receive is followed by a valid completion and duplicate.
    HostileReceive,
    /// Alternate time and stratum-zero opaque-control completions.
    Complete,
    /// Reject a stale cancellation, retire the actual request, then rearm.
    Cancel,
    /// Reject early expiry, expire exactly at deadline, then rearm.
    Expire,
    /// Reject unchanged context, retire by generation advance, then rearm.
    Context,
}

impl Workload {
    /// Fixed JSON label without attacker-controlled contents.
    const fn name(self) -> &'static str {
        match self {
            Self::HostileReceive => "hostile_receive",
            Self::Complete => "complete",
            Self::Cancel => "cancel",
            Self::Expire => "expire",
            Self::Context => "context",
        }
    }

    /// Fixed workload size; the four retirement/completion rows total 256 cycles.
    const fn iterations(self) -> u32 {
        match self {
            Self::HostileReceive => 4096,
            Self::Complete | Self::Cancel | Self::Expire | Self::Context => 64,
        }
    }
}

/// Explicit selection order; no iterator/filter can silently skip a workload.
const WORKLOADS: [Workload; 5] = [
    Workload::HostileReceive,
    Workload::Complete,
    Workload::Cancel,
    Workload::Expire,
    Workload::Context,
];

/// Scalar execution accounting; every bound is below 16,384 actual operations.
#[derive(Clone, Copy)]
struct Counts {
    /// Number of actual arm invocations, including any unexpected failure.
    arms: u32,
    /// Nonduplicate receive rejection probes actually delivered.
    rejected_receives: u32,
    /// Observed time events, independently checked for exact values.
    time_events: u32,
    /// Observed opaque-control events, independently checked for exact values.
    control_events: u32,
    /// Successful explicit cancellation, expiry or context retirement events.
    retirements: u32,
    /// Duplicate receive probes delivered after a successful completion.
    duplicates: u32,
    /// Stale-cancel, early-expiry or unchanged-context probes delivered.
    rejected_lifecycle: u32,
    /// All unexpected result, identity, numerical and state observations.
    mismatches: u32,
}

impl Counts {
    /// Empty counter initializer, before any operation is attempted.
    const EMPTY: Self = Self {
        arms: 0,
        rejected_receives: 0,
        time_events: 0,
        control_events: 0,
        retirements: 0,
        duplicates: 0,
        rejected_lifecycle: 0,
        mismatches: 0,
    };

    /// Record a boolean observation without formatting or allocation.
    fn require(&mut self, condition: bool) {
        self.mismatches += u32::from(!condition);
    }
}

/// One completed interval, captured while its empty owner remains alive.
#[derive(Clone, Copy)]
struct Sample {
    /// Selected fixed workload.
    workload: Workload,
    /// Actual iteration count entered, including failed-operation iterations.
    iterations: u32,
    /// Operation and result accounting.
    counts: Counts,
    /// Whether the actual pending projection is empty before fixture teardown.
    pending_empty: bool,
    /// Logical requested-payload baseline before entering the workload.
    before: allocator::Snapshot,
    /// Snapshot after its final live-state check, before owner destruction.
    after: allocator::Snapshot,
}

impl Sample {
    /// Array initializer overwritten by the five actual observations.
    const EMPTY: Self = Self {
        workload: Workload::HostileReceive,
        iterations: 0,
        counts: Counts::EMPTY,
        pending_empty: false,
        before: allocator::Snapshot::EMPTY,
        after: allocator::Snapshot::EMPTY,
    };
}

/// Construct a literal 49-byte stack buffer; callers explicitly select 48 bytes.
/// Peer receive/transmit are one raw Q32 tick; only opaque origin bits vary.
fn packet(token: u64) -> [u8; 49] {
    let mut bytes = [0_u8; 49];
    bytes[0] = 0x24;
    bytes[1] = 1;
    bytes[24..32].copy_from_slice(&token.to_be_bytes());
    bytes[39] = 1;
    bytes[47] = 1;
    bytes
}

/// Attempt a real arm with a distinct fixture token and independent request ID.
/// Failure is recorded; no unchecked request or pending state is manufactured.
fn arm(
    owner: &mut Association,
    fixture: &Fixture,
    token: u64,
    expected_request: u64,
    counts: &mut Counts,
) -> Option<RequestId> {
    let Ok(checked_token) = WireToken::new(black_box(token)) else {
        counts.require(false);
        return None;
    };
    counts.arms += 1;
    if let Ok(request) =
        black_box(owner.arm(fixture.utc, checked_token, fixture.timing, fixture.eras))
    {
        counts.require(request.value().get() == expected_request);
        counts.require(owner.pending_request_id() == Some(request));
        Some(request)
    } else {
        counts.require(false);
        None
    }
}

/// Deliver one rejected packet and inspect the same pending request afterwards.
fn reject(
    owner: &mut Association,
    bytes: &[u8],
    receipt: ReceiptFacts,
    expected: ReceiveError,
    pending: Option<RequestId>,
    counts: &mut Counts,
) {
    counts.rejected_receives += 1;
    counts.require(black_box(owner.receive(black_box(bytes), receipt)) == Err(expected));
    counts.require(owner.pending_request_id() == pending);
}

/// Check completion provenance and retained numeric fields against independent
/// fixture literals; the owner supplies no expected numerical value to this code.
fn complete(
    owner: &mut Association,
    fixture: &Fixture,
    bytes: &[u8],
    request: RequestId,
    control_expected: bool,
    counts: &mut Counts,
) {
    let context = owner.current_context();
    let result = black_box(owner.receive(black_box(bytes), fixture.receipt));
    let facts = match &result {
        Ok(FixtureEvent::Time(value)) => {
            counts.time_events += 1;
            counts.require(!control_expected);
            counts.require(value.saved_real_t1().ticks() == 1);
            counts.require(value.peer_t2().ticks() == 1 && value.peer_t3().ticks() == 1);
            counts.require(value.local_t4().ticks() == 1);
            counts.require(value.offset().numerator() == 0);
            counts.require(value.signed_round_trip().numerator() == 0);
            counts.require(value.leap_indicator() == 0 && value.stratum() == 1);
            Some(value.facts())
        }
        Ok(FixtureEvent::Control(value)) => {
            counts.control_events += 1;
            counts.require(control_expected && value.code_bits() == 0x5241_5445);
            Some(value.facts())
        }
        Err(_) => {
            counts.require(false);
            None
        }
    };
    if let Some(facts) = facts {
        counts.require(facts.request_id() == request);
        counts.require(facts.binding() == fixture.binding);
        counts.require(facts.generations() == context.generations());
        counts.require(facts.mapping_generation() == ClockGeneration::new(0));
        counts.require(facts.send().nanoseconds() == 10);
        counts.require(facts.arrival().nanoseconds() == 11);
        counts.require(facts.processing_now().nanoseconds() == 12);
        counts.require(facts.deadline().nanoseconds() == 20);
        counts.duplicates += 1;
        counts.require(
            owner.receive(black_box(bytes), fixture.receipt) == Err(ReceiveError::NoPending),
        );
    }
    counts.require(owner.pending_request_id().is_none());
}

/// Apply one of twenty independently specified faults to a copied valid packet.
/// Exact framing/profile/currentness/time errors must preserve the pending slot.
fn hostile(
    owner: &mut Association,
    fixture: &Fixture,
    bytes: [u8; 49],
    index: u32,
    request: RequestId,
    counts: &mut Counts,
) {
    let mut bad = bytes;
    let mut length = 48;
    let mut receipt = fixture.receipt;
    let expected = match index % 20 {
        0 => {
            length = 47;
            ReceiveError::Framing(FrameError::HeaderTooShort)
        }
        1 => {
            length = 49;
            ReceiveError::Framing(FrameError::ByteLimit)
        }
        2 => {
            bad[0] = 0x1c;
            ReceiveError::UnsupportedVersion
        }
        3 => {
            bad[0] = 0x23;
            ReceiveError::UnsupportedMode
        }
        4 => {
            receipt = fixture.bad_receipts[0];
            ReceiveError::SourceMismatch
        }
        5 => {
            receipt = fixture.bad_receipts[1];
            ReceiveError::SocketMismatch
        }
        6 => {
            receipt = fixture.bad_receipts[2];
            ReceiveError::EndpointMismatch
        }
        7 => {
            receipt = fixture.bad_receipts[3];
            ReceiveError::MappingGenerationMismatch
        }
        8 => {
            bad[24..32].fill(0);
            ReceiveError::ZeroOrigin
        }
        9 => {
            bad[24..32].fill(0xff);
            ReceiveError::OriginMismatch
        }
        10 => {
            receipt = fixture.bad_receipts[4];
            ReceiveError::ArrivalBeforeSend
        }
        11 => {
            receipt = fixture.bad_receipts[5];
            ReceiveError::ProcessingBeforeArrival
        }
        12 => {
            receipt = fixture.bad_receipts[6];
            ReceiveError::DeadlineReached
        }
        13 => {
            bad[0] = 0xe4;
            ReceiveError::UnsynchronizedLeap
        }
        14 => {
            bad[1] = 16;
            ReceiveError::InvalidStratum
        }
        15 => {
            bad[39] = 0;
            ReceiveError::ZeroReceiveTimestamp
        }
        16 => {
            bad[47] = 0;
            ReceiveError::ZeroTransmitTimestamp
        }
        17 => {
            bad[39] = 2;
            ReceiveError::ReceiveEra(DomainError::NoEraCandidate)
        }
        18 => {
            bad[1] = 0;
            bad[24..32].fill(0xff);
            ReceiveError::OriginMismatch
        }
        _ => {
            bad[1] = 0;
            bad[24..32].fill(0);
            ReceiveError::ZeroOrigin
        }
    };
    reject(
        owner,
        &bad[..length],
        receipt,
        expected,
        Some(request),
        counts,
    );
}

/// Exercise one real trusted retirement and its rejection neighbor before rearm.
fn retire(
    owner: &mut Association,
    fixture: &Fixture,
    workload: Workload,
    request: RequestId,
    iteration: u32,
    counts: &mut Counts,
) {
    counts.rejected_lifecycle += 1;
    let retired = match workload {
        Workload::Cancel => {
            counts.require(
                owner.cancel(RequestId::new(NonZeroU64::MAX)) == Err(CancelError::RequestMismatch),
            );
            counts.require(owner.pending_request_id() == Some(request));
            owner.cancel(request) == Ok(request)
        }
        Workload::Expire => {
            counts.require(owner.expire(MonotonicNs::new(19)) == Err(ExpireError::NotExpired));
            counts.require(owner.pending_request_id() == Some(request));
            owner.expire(MonotonicNs::new(20)) == Ok(request)
        }
        Workload::Context => {
            counts.require(
                owner.advance_context(owner.current_context())
                    == Err(ContextError::NoGenerationAdvance),
            );
            counts.require(owner.pending_request_id() == Some(request));
            let next = AssociationContext::new(
                fixture.binding,
                Generations::new(
                    ConfigurationGeneration::new(0),
                    AssociationGeneration::new(u64::from(iteration) + 1),
                    ClockGeneration::new(0),
                ),
            );
            let result = owner.advance_context(next) == Ok(Some(request));
            counts.require(owner.current_context() == next);
            result
        }
        Workload::HostileReceive | Workload::Complete => false,
    };
    counts.require(retired);
    counts.retirements += u32::from(retired);
    counts.require(owner.pending_request_id().is_none());
}

/// Execute one bounded iteration; fresh tokens are distinct for every real arm.
fn iteration(
    owner: &mut Association,
    fixture: &Fixture,
    workload: Workload,
    index: u32,
    counts: &mut Counts,
) {
    let double_arm = matches!(
        workload,
        Workload::Cancel | Workload::Expire | Workload::Context
    );
    let expected_request = if double_arm {
        u64::from(index) * 2 + 1
    } else {
        u64::from(index) + 1
    };
    let token = expected_request + 100;
    let Some(request) = arm(owner, fixture, token, expected_request, counts) else {
        return;
    };
    let mut bytes = packet(token);
    match workload {
        Workload::HostileReceive => {
            hostile(owner, fixture, bytes, index, request, counts);
            complete(owner, fixture, &bytes[..48], request, false, counts);
        }
        Workload::Complete => {
            let is_control = index % 2 == 1;
            if is_control {
                bytes[0] = 0xe4;
                bytes[1] = 0;
                bytes[12..16].copy_from_slice(&0x5241_5445_u32.to_be_bytes());
                bytes[32..48].fill(0);
            }
            complete(owner, fixture, &bytes[..48], request, is_control, counts);
        }
        Workload::Cancel | Workload::Expire | Workload::Context => {
            retire(owner, fixture, workload, request, index, counts);
            reject(
                owner,
                &bytes[..48],
                fixture.receipt,
                ReceiveError::NoPending,
                None,
                counts,
            );
            if let Some(next) = arm(owner, fixture, token + 1, expected_request + 1, counts) {
                if matches!(workload, Workload::Context) {
                    reject(
                        owner,
                        &bytes[..48],
                        fixture.receipt,
                        ReceiveError::OriginMismatch,
                        Some(next),
                        counts,
                    );
                }
                let fresh = packet(token + 1);
                complete(owner, fixture, &fresh[..48], next, false, counts);
            }
        }
    }
}

/// Snapshot while the actual owner is alive and its pending slot is checked empty.
fn observe(workload: Workload, fixture: &Fixture) -> Sample {
    let before = allocator::begin();
    let mut owner =
        Association::new_plain_header_fixture(fixture.context, RequestId::new(NonZeroU64::MIN));
    let mut counts = Counts::EMPTY;
    for index in 0..workload.iterations() {
        iteration(&mut owner, black_box(fixture), workload, index, &mut counts);
    }
    let pending_empty = owner.pending_request_id().is_none();
    counts.require(pending_empty);
    let after = allocator::snapshot();
    let _observed_owner = black_box(&owner);
    Sample {
        workload,
        iterations: workload.iterations(),
        counts,
        pending_empty,
        before,
        after,
    }
}

/// Safe retained-allocation control, released explicitly before reporting.
#[derive(Clone, Copy)]
struct BoxCalibration {
    /// Baseline before safe allocation.
    before: allocator::Snapshot,
    /// Actual held allocation remains alive at this snapshot.
    held: allocator::Snapshot,
    /// Snapshot after the matching drop, before process teardown.
    dropped: allocator::Snapshot,
    /// Every touched byte matched its fixed initialized value.
    contents_match: bool,
}

/// Calibrate positive allocation and matching release with owned safe storage.
fn calibrate_box() -> BoxCalibration {
    let before = allocator::begin();
    let held_box = black_box(Box::new(black_box([0x5a_u8; 4096])));
    let contents_match = black_box(&held_box).iter().all(|byte| *byte == 0x5a);
    let held = allocator::snapshot();
    drop(black_box(held_box));
    let dropped = allocator::snapshot();
    BoxCalibration {
        before,
        held,
        dropped,
        contents_match,
    }
}

/// Independent zero-initialization and realloc forwarding control.
#[derive(Clone, Copy)]
struct VecCalibration {
    /// Baseline before safe vector allocation.
    before: allocator::Snapshot,
    /// Held zero-initialized vector observation.
    zeroed: allocator::Snapshot,
    /// Observation after safe growth and initialized content checks.
    grown: allocator::Snapshot,
    /// Observation after safe truncation and shrink request.
    shrunk: allocator::Snapshot,
    /// Observation after explicit matching drop.
    dropped: allocator::Snapshot,
    /// Original and grown initialized bytes matched fixed expectations.
    contents_match: bool,
}

/// Safe `Vec` ownership calibrates each forwarded operation without raw pointers.
fn calibrate_vec() -> VecCalibration {
    let before = allocator::begin();
    let mut vector = black_box(vec![0_u8; black_box(128)]);
    let zero_contents = black_box(vector.as_slice()).iter().all(|byte| *byte == 0);
    let zeroed = allocator::snapshot();
    vector.resize(black_box(640), 0x3c);
    let grown_contents = black_box(vector.as_slice())
        .iter()
        .skip(128)
        .all(|byte| *byte == 0x3c);
    let grown = allocator::snapshot();
    vector.truncate(black_box(64));
    vector.shrink_to_fit();
    let shrunk_contents = black_box(vector.as_slice()).iter().all(|byte| *byte == 0);
    let shrunk = allocator::snapshot();
    drop(black_box(vector));
    let dropped = allocator::snapshot();
    VecCalibration {
        before,
        zeroed,
        grown,
        shrunk,
        dropped,
        contents_match: zero_contents && grown_contents && shrunk_contents,
    }
}

/// All snapshots exist before stdout acquisition, formatting or any teardown.
struct Report {
    /// Baseline after dependency preparation and before calibration/workloads.
    initial: allocator::Snapshot,
    /// Five actual fixed workload observations in their declared order.
    samples: [Sample; 5],
    /// Positive retained-allocation and explicit-release control.
    box_control: BoxCalibration,
    /// Positive zeroed/realloc and explicit-release control.
    vec_control: VecCalibration,
    /// Final snapshot before all formatting and process teardown.
    final_state: allocator::Snapshot,
}

/// Run the exact finite workload after all fallible dependency construction.
fn run(fixture: &Fixture) -> Report {
    let initial = allocator::snapshot();
    let box_control = calibrate_box();
    let vec_control = calibrate_vec();
    let mut samples = [Sample::EMPTY; 5];
    for (slot, workload) in samples.iter_mut().zip(WORKLOADS) {
        *slot = observe(workload, fixture);
    }
    let final_state = allocator::snapshot();
    Report {
        initial,
        samples,
        box_control,
        vec_control,
        final_state,
    }
}

/// Emit one complete raw snapshot only after all observed intervals finish.
/// # Errors
/// Propagates actual stdout failure without treating truncation as evidence.
fn write_snapshot(output: &mut impl Write, value: allocator::Snapshot) -> io::Result<()> {
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

/// Emit one sample's actual operation counters and unprocessed snapshots.
/// # Errors
/// Propagates stdout failures; the runner rejects incomplete JSON.
fn write_sample(output: &mut impl Write, sample: Sample) -> io::Result<()> {
    let counts = sample.counts;
    write!(
        output,
        "{{\"name\":\"{}\",\"iterations\":{},\"arms\":{},\"rejected_receives\":{},\"time_events\":{},\"control_events\":{},\"retirements\":{},\"duplicates\":{},\"rejected_lifecycle\":{},\"mismatches\":{},\"pending_empty\":{},\"before\":",
        sample.workload.name(),
        sample.iterations,
        counts.arms,
        counts.rejected_receives,
        counts.time_events,
        counts.control_events,
        counts.retirements,
        counts.duplicates,
        counts.rejected_lifecycle,
        counts.mismatches,
        sample.pending_empty
    )?;
    write_snapshot(output, sample.before)?;
    write!(output, ",\"after\":")?;
    write_snapshot(output, sample.after)?;
    write!(output, "}}")
}

/// Print the completed observations; the runner independently evaluates bounds.
/// # Errors
/// Propagates output failures; no success status is invented by a partial report.
fn write_report(report: &Report) -> io::Result<()> {
    let mut output = io::stdout().lock();
    write!(
        output,
        "{{\"schema_version\":1,\"scope\":\"s1.1d-host-requested-payload\",\"profile\":\"single-thread-fixed-fixture-v1\",\"debug_assertions\":{},\"observation_initial\":",
        cfg!(debug_assertions)
    )?;
    write_snapshot(&mut output, report.initial)?;
    write!(
        output,
        ",\"box_control\":{{\"contents_match\":{},\"before\":",
        report.box_control.contents_match
    )?;
    write_snapshot(&mut output, report.box_control.before)?;
    write!(output, ",\"held\":")?;
    write_snapshot(&mut output, report.box_control.held)?;
    write!(output, ",\"dropped\":")?;
    write_snapshot(&mut output, report.box_control.dropped)?;
    write!(
        output,
        "}},\"vec_control\":{{\"contents_match\":{},\"before\":",
        report.vec_control.contents_match
    )?;
    write_snapshot(&mut output, report.vec_control.before)?;
    write!(output, ",\"zeroed\":")?;
    write_snapshot(&mut output, report.vec_control.zeroed)?;
    write!(output, ",\"grown\":")?;
    write_snapshot(&mut output, report.vec_control.grown)?;
    write!(output, ",\"shrunk\":")?;
    write_snapshot(&mut output, report.vec_control.shrunk)?;
    write!(output, ",\"dropped\":")?;
    write_snapshot(&mut output, report.vec_control.dropped)?;
    write!(output, "}},\"samples\":[")?;
    for (index, sample) in report.samples.iter().enumerate() {
        if index != 0 {
            write!(output, ",")?;
        }
        write_sample(&mut output, *sample)?;
    }
    write!(output, "],\"observation_final\":")?;
    write_snapshot(&mut output, report.final_state)?;
    writeln!(output, "}}")
}

/// Prepare, observe and emit; only complete stdout supplies a candidate report.
fn main() -> ExitCode {
    let Ok(fixture) = Fixture::new() else {
        return ExitCode::FAILURE;
    };
    let report = run(&fixture);
    if write_report(&report).is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
