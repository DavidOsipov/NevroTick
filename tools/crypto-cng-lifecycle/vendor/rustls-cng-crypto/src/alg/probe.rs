//! Owned, feature-gated observation of private and public cached initialization.
//!
//! The original direct probe keeps its bounded cleanup and recovery contract.
//! The separate cache session tracks actual native statuses and one shutdown close.
//! Faults substitute a static invalid input into a real native call. Observation
//! never authorizes or vetoes a native close. One fresh process, one thread, two
//! successful opens and eight events bound both diagnostic surfaces.

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use windows::Win32::Foundation::NTSTATUS;
use windows::Win32::Security::Cryptography::{
    BCRYPT_ALG_HANDLE, BCRYPT_ECC_CURVE_25519, BCRYPT_ECC_CURVE_NAME, BCRYPT_ECDH_ALGORITHM,
    BCRYPT_HANDLE, BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS, BCryptCloseAlgorithmProvider,
    BCryptOpenAlgorithmProvider, BCryptSetProperty,
};
use windows::core::{PCWSTR, w};

/// Maximum native observations: the fixed cases need at most six.
const EVENT_LIMIT: usize = 8;
/// Maximum successful native algorithm opens in a fixed case.
const HANDLE_LIMIT: usize = 2;
/// Process-wide admission; a second caller cannot silently start another probe.
static USED: AtomicBool = AtomicBool::new(false);

thread_local! {
    /// State is confined to the one synchronous diagnostic caller thread.
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

/// Closed input selecting a single bounded diagnostic.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProbeCase {
    /// One normal initializer call and returned-owner close.
    HealthyLoad,
    /// One real open with an invalid algorithm, then a normal neighbor.
    OpenInvalidOnce,
    /// One real property call with an invalid name, then a normal neighbor.
    PropertyInvalidOnce,
}

/// Initial fault/control phase or subsequent fault-off healthy neighbor.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// The first actual call to the private initializer.
    Initial,
    /// A second actual initializer call after fault injection is disarmed.
    Neighbor,
}

/// Native call roles; fixture cleanup never masquerades as provider cleanup.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    /// Actual `BCryptOpenAlgorithmProvider` result.
    Open,
    /// Actual `BCryptSetProperty` result.
    SetProperty,
    /// Provider guard cleanup before the initializer propagates an error.
    ProviderCloseError,
    /// Caller close of a handle returned successfully by the initializer.
    CloseReturned,
    /// Diagnostic close of an error-path orphan after the saved snapshot.
    CloseRecovery,
}

/// Redacted native observation: ordinal and raw status, never a pointer.
#[derive(Clone, Copy)]
pub struct NativeEvent {
    /// The operation actually completed.
    operation: Operation,
    /// The initializer phase that owns this operation's handle.
    phase: Phase,
    /// Successful-open ordinal, absent when no tracked handle was obtained.
    ordinal: Option<u8>,
    /// Actual returned signed `NTSTATUS`; no synthetic statuses are stored.
    status: i32,
}

impl NativeEvent {
    /// Return the actual native operation role.
    pub fn operation(&self) -> Operation {
        self.operation
    }
    /// Return its owning initializer phase.
    pub fn phase(&self) -> Phase {
        self.phase
    }
    /// Return a bounded opaque ordinal instead of the raw native handle.
    pub fn ordinal(&self) -> Option<u8> {
        self.ordinal
    }
    /// Return the exact signed status.
    pub fn status(&self) -> i32 {
        self.status
    }
    /// Return the same raw bits for hexadecimal projection.
    pub fn status_bits(&self) -> u32 {
        u32::from_ne_bytes(self.status.to_ne_bytes())
    }
    /// Apply the pinned Windows status convention: nonnegative means success.
    pub fn succeeded(&self) -> bool {
        self.status >= 0
    }
}

/// Interpretation of the actual fixed observations, not a provider qualification.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Conclusion {
    /// The normal initializer and its caller close both completed successfully.
    Healthy,
    /// Actual open failure propagated without an allocated successful handle.
    OpenFailureObserved,
    /// Failed property initialization closed its provider handle before return.
    PropertyFailureCleaned,
    /// An error-path orphan remained native-closeable after the saved snapshot.
    CleanupGap,
    /// A required native failure, neighbor or cleanup fact was not established.
    Incomplete,
}

/// Safe entry-admission error, occurring before this caller starts native work.
pub enum ProbeError {
    /// Another call already consumed the process-wide diagnostic admission.
    AlreadyUsed,
    /// Internal observation state was unexpectedly absent; no success is inferred.
    StateUnavailable,
}

/// Closed cases for one fresh-process public cache diagnostic.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CacheCase {
    /// Substitute the first actual native property name once.
    CachedPropertyFailure,
    /// Observe a sole shutdown owner without prior provider use.
    UnusedShutdown,
    /// Observe healthy public exchanges and their eventual shutdown.
    CachedHealthy,
}

/// Actual native call roles in the public-cache diagnostic.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CacheOperation {
    /// Actual `BCryptOpenAlgorithmProvider` result.
    Open,
    /// Actual `BCryptSetProperty` result.
    SetProperty,
    /// Actual close performed by the initializer's error guard.
    ProviderCloseError,
    /// Actual close performed by the sole shutdown owner.
    ShutdownClose,
}

/// Redacted actual native result, with no pointer or input bytes.
#[derive(Clone, Copy)]
pub struct CacheEvent {
    operation: CacheOperation,
    ordinal: Option<u8>,
    status: i32,
}

impl CacheEvent {
    /// Return the actual native operation role.
    pub fn operation(&self) -> CacheOperation {
        self.operation
    }
    /// Return the successful-open ordinal, if one exists.
    pub fn ordinal(&self) -> Option<u8> {
        self.ordinal
    }
    /// Return the signed native status.
    pub fn status(&self) -> i32 {
        self.status
    }
    /// Return the native status bit pattern.
    pub fn status_bits(&self) -> u32 {
        u32::from_ne_bytes(self.status.to_ne_bytes())
    }
    /// Return whether the native status satisfies `NT_SUCCESS`.
    pub fn succeeded(&self) -> bool {
        self.status >= 0
    }
}

/// Immutable, bounded observation; it conveys no provider qualification.
pub struct CacheSnapshot {
    events: [Option<CacheEvent>; EVENT_LIMIT],
    valid: bool,
    fault_consumed: bool,
    open_count: usize,
    outstanding: usize,
}

impl CacheSnapshot {
    /// Iterate actual calls in completion order, without exposing handles.
    pub fn events(&self) -> impl Iterator<Item = &CacheEvent> {
        self.events.iter().flatten()
    }
    /// Return whether observation remained complete and unambiguous.
    pub fn valid(&self) -> bool {
        self.valid
    }
    /// Return whether the selected invalid-name input reached its wrapper.
    pub fn fault_consumed(&self) -> bool {
        self.fault_consumed
    }
    /// Return the number of recorded successful native opens.
    pub fn open_count(&self) -> usize {
        self.open_count
    }
    /// Return the number of successful opens not yet closed successfully.
    pub fn outstanding(&self) -> usize {
        self.outstanding
    }
}

#[derive(Clone, Copy)]
struct CacheHandle {
    handle: BCRYPT_ALG_HANDLE,
    transferred: bool,
    close_attempted: bool,
    closed: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CachePhase {
    Collecting,
    Snapshotted,
    Disarmed,
    ShutdownEntered,
}

struct CacheState {
    case: CacheCase,
    phase: CachePhase,
    shutdown_entries: u8,
    shutdown_cache_accesses: u8,
    armed: bool,
    consumed: bool,
    valid: bool,
    events: [Option<CacheEvent>; EVENT_LIMIT],
    event_count: usize,
    handles: [Option<CacheHandle>; HANDLE_LIMIT],
    open_count: usize,
}

impl CacheState {
    fn new(case: CacheCase) -> Self {
        Self {
            case,
            phase: CachePhase::Collecting,
            shutdown_entries: 0,
            shutdown_cache_accesses: 0,
            armed: true,
            consumed: false,
            valid: true,
            events: [None; EVENT_LIMIT],
            event_count: 0,
            handles: [None; HANDLE_LIMIT],
            open_count: 0,
        }
    }

    fn push(&mut self, operation: CacheOperation, ordinal: Option<u8>, status: NTSTATUS) {
        if let Some(slot) = self.events.get_mut(self.event_count) {
            *slot = Some(CacheEvent {
                operation,
                ordinal,
                status: status.0,
            });
            self.event_count += 1;
        } else {
            self.valid = false;
        }
    }

    fn ordinal(&mut self, handle: BCRYPT_ALG_HANDLE) -> Option<usize> {
        let mut found = None;
        for (index, tracked) in self.handles.iter().enumerate() {
            if tracked.is_some_and(|value| value.handle == handle && !value.close_attempted) {
                if found.is_some() {
                    self.valid = false;
                    return None;
                }
                found = Some(index);
            }
        }
        if found.is_none() {
            self.valid = false;
        }
        found
    }

    fn snapshot(&self) -> CacheSnapshot {
        CacheSnapshot {
            events: self.events,
            valid: self.valid && !CACHE_BAD.load(Ordering::Acquire),
            fault_consumed: self.consumed,
            open_count: self.open_count,
            outstanding: self
                .handles
                .iter()
                .flatten()
                .filter(|handle| !handle.closed)
                .count(),
        }
    }
}

thread_local! {
    static CACHE_STATE: RefCell<Option<CacheState>> = const { RefCell::new(None) };
}
// One word couples the active bit and in-flight observation callbacks. The
// high bit denotes an active session; all lower bits count entered callbacks.
const CACHE_ACTIVE_BIT: usize = 1_usize << (usize::BITS - 1);
static CACHE_LIFECYCLE: AtomicUsize = AtomicUsize::new(0);
static CACHE_ACCESSED: AtomicBool = AtomicBool::new(false);
static CACHE_BAD: AtomicBool = AtomicBool::new(false);
// Independent disarm remains available even if the thread-local state cannot be borrowed.
static CACHE_FAULT_ARMED: AtomicBool = AtomicBool::new(false);

/// A single-threaded observation session; only the closed cases are selectable.
///
/// Begin before any cached use and keep the sole shutdown owner inside the session.
/// A before-drop snapshot, disarm and exactly one observed shutdown entry precede
/// finish. Upstream's public `ShutdownHandle` permits additional owners; a fresh
/// single-threaded process and exclusive owner remain external fixture obligations.
/// Dropping this value stops observation and performs no native cleanup.
pub struct CacheSession {
    _same_thread: std::marker::PhantomData<std::rc::Rc<()>>,
}

impl CacheSession {
    /// Admit exactly one diagnostic session in a fresh process.
    ///
    /// # Errors
    /// Rejects a previously used diagnostic or unavailable internal state.
    pub fn begin(case: CacheCase) -> Result<Self, ProbeError> {
        if USED
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(ProbeError::AlreadyUsed);
        }
        CACHE_STATE
            .try_with(|cell| {
                let mut slot = cell
                    .try_borrow_mut()
                    .map_err(|_| ProbeError::StateUnavailable)?;
                if slot.is_some() {
                    return Err(ProbeError::StateUnavailable);
                }
                *slot = Some(CacheState::new(case));
                Ok::<(), ProbeError>(())
            })
            .map_err(|_| ProbeError::StateUnavailable)??;
        CACHE_BAD.store(false, Ordering::SeqCst);
        CACHE_FAULT_ARMED.store(true, Ordering::SeqCst);
        // Atomic activation returns any already-entered callback; the access
        // marker is published before each callback releases its count. Thus
        // an access straddling admission is rejected or invalidates the session.
        let previous = CACHE_LIFECYCLE.fetch_or(CACHE_ACTIVE_BIT, Ordering::SeqCst);
        if previous != 0 || CACHE_ACCESSED.load(Ordering::SeqCst) {
            CACHE_FAULT_ARMED.store(false, Ordering::SeqCst);
            CACHE_LIFECYCLE.fetch_and(!CACHE_ACTIVE_BIT, Ordering::SeqCst);
            let _ = CACHE_STATE.try_with(|cell| {
                if let Ok(mut slot) = cell.try_borrow_mut() {
                    *slot = None;
                }
            });
            return Err(ProbeError::AlreadyUsed);
        }
        Ok(Self {
            _same_thread: std::marker::PhantomData,
        })
    }

    /// Copy the current bounded facts without changing or closing any handle.
    ///
    /// # Errors
    /// Rejects missing or currently borrowed observation state.
    pub fn snapshot(&self) -> Result<CacheSnapshot, ProbeError> {
        let result = CACHE_STATE
            .try_with(|cell| {
                let mut slot = cell
                    .try_borrow_mut()
                    .map_err(|_| ProbeError::StateUnavailable)?;
                let state = slot.as_mut().ok_or(ProbeError::StateUnavailable)?;
                if state.phase != CachePhase::Collecting {
                    state.valid = false;
                    return Err(ProbeError::StateUnavailable);
                }
                state.phase = CachePhase::Snapshotted;
                Ok(state.snapshot())
            })
            .map_err(|_| ProbeError::StateUnavailable)?;
        if result.is_err() {
            CACHE_BAD.store(true, Ordering::SeqCst);
        }
        result
    }

    /// Disarm the one selected fault before dropping the sole shutdown owner.
    ///
    /// # Errors
    /// Rejects missing or currently borrowed observation state.
    pub fn disarm(&self) -> Result<(), ProbeError> {
        // Even an invalid phase or inaccessible observer must not leave the
        // diagnostic fault armed during the owner's native cleanup.
        CACHE_FAULT_ARMED.store(false, Ordering::SeqCst);
        let result = CACHE_STATE
            .try_with(|cell| {
                let mut slot = cell
                    .try_borrow_mut()
                    .map_err(|_| ProbeError::StateUnavailable)?;
                let state = slot.as_mut().ok_or(ProbeError::StateUnavailable)?;
                state.armed = false;
                if state.phase != CachePhase::Snapshotted {
                    state.valid = false;
                    return Err(ProbeError::StateUnavailable);
                }
                state.phase = CachePhase::Disarmed;
                Ok(())
            })
            .map_err(|_| ProbeError::StateUnavailable)?;
        if result.is_err() {
            CACHE_BAD.store(true, Ordering::SeqCst);
        }
        result
    }

    /// Stop observation after shutdown and return its last bounded snapshot.
    ///
    /// # Errors
    /// Rejects missing or currently borrowed observation state.
    pub fn finish(self) -> Result<CacheSnapshot, ProbeError> {
        CACHE_FAULT_ARMED.store(false, Ordering::SeqCst);
        // Deactivate first, then reject any callback that overlapped deactivation.
        // This keeps a delayed foreign-thread invalidation from following snapshot.
        let previous = CACHE_LIFECYCLE.fetch_and(!CACHE_ACTIVE_BIT, Ordering::SeqCst);
        if previous & !CACHE_ACTIVE_BIT != 0 {
            CACHE_BAD.store(true, Ordering::SeqCst);
        }
        let result = CACHE_STATE
            .try_with(|cell| {
                let mut slot = cell
                    .try_borrow_mut()
                    .map_err(|_| ProbeError::StateUnavailable)?;
                let state = slot.as_mut().ok_or(ProbeError::StateUnavailable)?;
                if state.phase != CachePhase::ShutdownEntered || state.shutdown_entries != 1 {
                    state.valid = false;
                    return Err(ProbeError::StateUnavailable);
                }
                Ok(state.snapshot())
            })
            .map_err(|_| ProbeError::StateUnavailable)?;
        if result.is_err() {
            CACHE_BAD.store(true, Ordering::SeqCst);
        }
        let cleared = CACHE_STATE
            .try_with(|cell| {
                let mut slot = cell
                    .try_borrow_mut()
                    .map_err(|_| ProbeError::StateUnavailable)?;
                *slot = None;
                Ok::<(), ProbeError>(())
            })
            .map_err(|_| ProbeError::StateUnavailable)?;
        cleared?;
        result
    }
}

impl Drop for CacheSession {
    fn drop(&mut self) {
        CACHE_FAULT_ARMED.store(false, Ordering::SeqCst);
        CACHE_LIFECYCLE.fetch_and(!CACHE_ACTIVE_BIT, Ordering::SeqCst);
        let _ = CACHE_STATE.try_with(|cell| {
            if let Ok(mut slot) = cell.try_borrow_mut() {
                *slot = None;
            }
        });
    }
}

/// Observe only while the one diagnostic session is active; never veto native work.
fn observe_cache(f: impl FnOnce(&mut CacheState)) {
    let observed = CACHE_STATE
        .try_with(|cell| {
            if let Ok(mut slot) = cell.try_borrow_mut() {
                if let Some(state) = slot.as_mut() {
                    f(state);
                    return true;
                }
            }
            false
        })
        .unwrap_or(false);
    if !observed {
        CACHE_BAD.store(true, Ordering::SeqCst);
    }
}

fn with_cache(f: impl FnOnce(&mut CacheState)) {
    let prior = CACHE_LIFECYCLE.fetch_add(1, Ordering::SeqCst);
    if prior & CACHE_ACTIVE_BIT != 0 {
        observe_cache(f);
    }
    CACHE_LIFECYCLE.fetch_sub(1, Ordering::SeqCst);
}

/// Record cache entry, detecting prior use and foreign-thread access during a session.
pub(super) fn mark_cache_access() {
    let prior = CACHE_LIFECYCLE.fetch_add(1, Ordering::SeqCst);
    CACHE_ACCESSED.store(true, Ordering::SeqCst);
    if prior & CACHE_ACTIVE_BIT != 0 {
        observe_cache(|state| match state.phase {
            CachePhase::Collecting => {}
            CachePhase::ShutdownEntered => {
                state.shutdown_cache_accesses = state.shutdown_cache_accesses.saturating_add(1);
                if state.shutdown_cache_accesses != 1 {
                    state.valid = false;
                }
            }
            CachePhase::Snapshotted | CachePhase::Disarmed => state.valid = false,
        });
    }
    CACHE_LIFECYCLE.fetch_sub(1, Ordering::SeqCst);
}

/// Observe a shutdown entry without authorizing or suppressing its native work.
pub(super) fn mark_shutdown_entry() {
    // Never inject a diagnostic fault during native shutdown, even if the
    // caller violated phase order or thread-local observation is unavailable.
    CACHE_FAULT_ARMED.store(false, Ordering::SeqCst);
    with_cache(|state| {
        state.armed = false;
        state.shutdown_entries = state.shutdown_entries.saturating_add(1);
        if state.phase != CachePhase::Disarmed || state.shutdown_entries != 1 {
            state.valid = false;
        } else {
            state.phase = CachePhase::ShutdownEntered;
        }
    });
}

fn cache_property_override() -> bool {
    let mut override_name = false;
    with_cache(|state| {
        if CACHE_FAULT_ARMED.load(Ordering::SeqCst)
            && state.case == CacheCase::CachedPropertyFailure
            && state.armed
            && !state.consumed
        {
            state.consumed = true;
            override_name = true;
        }
    });
    override_name
}

fn record_cache_open(handle: BCRYPT_ALG_HANDLE, status: NTSTATUS) {
    with_cache(|state| {
        let mut ordinal = None;
        if status.is_ok() && !handle.is_invalid() {
            if let Some(slot) = state.handles.get_mut(state.open_count) {
                *slot = Some(CacheHandle {
                    handle,
                    transferred: false,
                    close_attempted: false,
                    closed: false,
                });
                ordinal = u8::try_from(state.open_count).ok();
                state.open_count += 1;
            } else {
                state.valid = false;
            }
        } else if status.is_ok() || !handle.is_invalid() {
            state.valid = false;
        }
        state.push(CacheOperation::Open, ordinal, status);
    });
}

fn record_cache_property(handle: BCRYPT_HANDLE, status: NTSTATUS) {
    with_cache(|state| {
        let index = state.ordinal(BCRYPT_ALG_HANDLE(handle.0));
        state.push(
            CacheOperation::SetProperty,
            index.and_then(|i| u8::try_from(i).ok()),
            status,
        );
    });
}

/// Record transfer to the actual caller; this never changes native ownership.
pub(super) fn record_cache_transfer(handle: BCRYPT_ALG_HANDLE) {
    with_cache(|state| {
        if let Some(index) = state.ordinal(handle) {
            if let Some(tracked) = state.handles[index].as_mut() {
                tracked.transferred = true;
            }
        }
    });
}

fn record_cache_close(handle: BCRYPT_ALG_HANDLE, operation: CacheOperation, status: NTSTATUS) {
    with_cache(|state| {
        let index = state.ordinal(handle);
        if let Some(index) = index {
            if let Some(tracked) = state.handles[index].as_mut() {
                if tracked.transferred != (operation == CacheOperation::ShutdownClose) {
                    state.valid = false;
                }
                tracked.close_attempted = true;
                tracked.closed = status.is_ok();
            }
        }
        state.push(operation, index.and_then(|i| u8::try_from(i).ok()), status);
    });
}

/// Perform the same one native close as `Free`, preserving its actual status.
///
/// # Safety
/// The caller supplies the cached handle under the sole shutdown-owner contract.
pub(super) unsafe fn close_shutdown(handle: BCRYPT_ALG_HANDLE) {
    if handle.is_invalid() {
        with_cache(|state| state.valid = false);
        return;
    }
    // SAFETY: This replaces the single `Free` close on the same cached handle.
    let status = unsafe { BCryptCloseAlgorithmProvider(handle, 0) };
    record_cache_close(handle, CacheOperation::ShutdownClose, status);
}

/// Immutable saved facts with no exposed native handle or error payload string.
pub struct ProbeReport {
    /// Observed native calls in their actual completion order.
    events: [Option<NativeEvent>; EVENT_LIMIT],
    /// Number of recorded events before any orphan-recovery close.
    before_recovery_events: usize,
    /// Whether the first private initializer returned ownership to its caller.
    initial_returned: bool,
    /// Healthy neighbor return, absent for the single-call healthy control.
    neighbor_returned: Option<bool>,
    /// Successful-open handles with no returned ownership before fixture recovery.
    orphans_before_recovery: u8,
    /// Whether the planned one-shot override actually reached a native call.
    fault_consumed: bool,
    /// Interpretation under the strict actual-status/cleanup conditions.
    conclusion: Conclusion,
}

impl ProbeReport {
    /// Iterate the fixed retained native observations without native work.
    pub fn events(&self) -> impl Iterator<Item = &NativeEvent> {
        self.events.iter().flatten()
    }
    /// Return the saved event boundary preceding fixture recovery.
    pub fn before_recovery_events(&self) -> usize {
        self.before_recovery_events
    }
    /// Return actual successful ownership transfer from the first call.
    pub fn initial_returned(&self) -> bool {
        self.initial_returned
    }
    /// Return actual neighbor ownership transfer or its meaningful absence.
    pub fn neighbor_returned(&self) -> Option<bool> {
        self.neighbor_returned
    }
    /// Return the saved bounded error-path orphan count.
    pub fn orphans_before_recovery(&self) -> u8 {
        self.orphans_before_recovery
    }
    /// Return whether the deterministic input override was consumed once.
    pub fn fault_consumed(&self) -> bool {
        self.fault_consumed
    }
    /// Return the interpretation; `CleanupGap` is an actual baseline RED.
    pub fn conclusion(&self) -> Conclusion {
        self.conclusion
    }
}

/// Private successful-open ownership observation; raw handles never leave here.
#[derive(Clone, Copy)]
struct Tracked {
    /// Actual handle from one successful native open.
    handle: BCRYPT_ALG_HANDLE,
    /// Successful-open ordinal, at most one.
    ordinal: u8,
    /// Initial or neighbor call owning this successful open.
    phase: Phase,
    /// Whether the private initializer returned this handle successfully.
    returned: bool,
    /// Close admission is one-way, including a native close that does not return.
    close_attempted: bool,
    /// A close was attempted exactly once; failure is not silently retried.
    close_status: Option<i32>,
}

/// Fixed-capacity observation state; it is never borrowed across native calls.
struct State {
    /// Closed case admitted by the public diagnostic entry point.
    case: ProbeCase,
    /// Phase currently calling the real initializer.
    phase: Phase,
    /// Fault input may be substituted only while this flag is set.
    armed: bool,
    /// The selected native call actually consumed its override.
    consumed: bool,
    /// Any capacity, ownership, close or native-output ambiguity clears validity.
    valid: bool,
    /// Successful-open ledger, bounded independently of event storage.
    handles: [Option<Tracked>; HANDLE_LIMIT],
    /// Number of successful opens already stored.
    handle_count: usize,
    /// Current successful-open ordinal for matching a returned native handle.
    current: Option<u8>,
    /// Bounded actual native results.
    events: [Option<NativeEvent>; EVENT_LIMIT],
    /// Number of populated event slots.
    event_count: usize,
    /// Actual first-call ownership result.
    initial_returned: Option<bool>,
    /// Actual second-call ownership result, if a neighbor was selected.
    neighbor_returned: Option<bool>,
}

impl State {
    /// Construct empty fixed storage without submitting native work.
    fn new(case: ProbeCase) -> Self {
        Self {
            case,
            phase: Phase::Initial,
            armed: true,
            consumed: false,
            valid: true,
            handles: [None; HANDLE_LIMIT],
            handle_count: 0,
            current: None,
            events: [None; EVENT_LIMIT],
            event_count: 0,
            initial_returned: None,
            neighbor_returned: None,
        }
    }

    /// Append an actual status, marking an overflow incomplete rather than inventing data.
    fn push(&mut self, operation: Operation, phase: Phase, ordinal: Option<u8>, status: NTSTATUS) {
        if let Some(slot) = self.events.get_mut(self.event_count) {
            *slot = Some(NativeEvent {
                operation,
                phase,
                ordinal,
                status: status.0,
            });
            self.event_count += 1;
        } else {
            self.valid = false;
        }
    }

    /// Find one retained event without native effects.
    fn event(&self, operation: Operation, phase: Phase) -> Option<&NativeEvent> {
        self.events
            .iter()
            .flatten()
            .find(|event| event.operation == operation && event.phase == phase)
    }

    /// Check complete actual success and normal caller cleanup for one phase.
    fn healthy(&self, phase: Phase) -> bool {
        let returned = match phase {
            Phase::Initial => self.initial_returned,
            Phase::Neighbor => self.neighbor_returned,
        };
        returned == Some(true)
            && [
                Operation::Open,
                Operation::SetProperty,
                Operation::CloseReturned,
            ]
            .iter()
            .all(|operation| {
                self.event(*operation, phase)
                    .is_some_and(NativeEvent::succeeded)
            })
    }
}

/// Invoke the actual native open, optionally substituting one static invalid name.
///
/// # Safety
/// The caller supplies the original initializer's valid output reference and
/// valid UTF-16 identifier; substituted input is static, terminated UTF-16.
pub(super) unsafe fn open_algorithm(
    out: &mut BCRYPT_ALG_HANDLE,
    id: PCWSTR,
    flags: BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS,
) -> NTSTATUS {
    let actual = STATE
        .try_with(|cell| {
            let mut state = cell.try_borrow_mut().ok()?;
            if let Some(state) = state.as_mut() {
                state.current = None;
                if state.armed && !state.consumed && state.case == ProbeCase::OpenInvalidOnce {
                    state.consumed = true;
                    return Some(w!("NEVROTICK_FIXTURE_NO_SUCH_ALGORITHM_V1"));
                }
            }
            Some(id)
        })
        .ok()
        .flatten()
        .unwrap_or(id);
    // SAFETY: The output reference is live; both possible identifiers are valid
    // terminated UTF-16. Provider selection and flags are unchanged.
    let status = unsafe { BCryptOpenAlgorithmProvider(out, actual, None, flags) };
    let _ = STATE.try_with(|cell| {
        if let Ok(mut slot) = cell.try_borrow_mut() {
            if let Some(state) = slot.as_mut() {
                if status.is_ok() && !out.is_invalid() {
                    let ordinal = match state.handle_count {
                        0 => Some(0),
                        1 => Some(1),
                        _ => None,
                    };
                    if let Some(ordinal) = ordinal {
                        state.handles[state.handle_count] = Some(Tracked {
                            handle: *out,
                            ordinal,
                            phase: state.phase,
                            returned: false,
                            close_attempted: false,
                            close_status: None,
                        });
                        state.handle_count += 1;
                        state.current = Some(ordinal);
                    } else {
                        state.valid = false;
                    }
                } else if status.is_ok() || !out.is_invalid() {
                    // Failed opens do not grant ownership of an unexpected output.
                    state.valid = false;
                }
                state.push(Operation::Open, state.phase, state.current, status);
            }
        }
    });
    record_cache_open(*out, status);
    status
}

/// Invoke the actual property call, optionally substituting one static invalid name.
///
/// # Safety
/// The caller retains the original live algorithm handle and property-value
/// buffer through this call. The original or substituted name is valid UTF-16.
pub(super) unsafe fn set_property(
    handle: BCRYPT_HANDLE,
    property: PCWSTR,
    value: &[u8],
) -> NTSTATUS {
    let override_cache = cache_property_override();
    let actual = STATE
        .try_with(|cell| {
            if override_cache {
                return Some(w!("NEVROTICK_FIXTURE_NO_SUCH_PROPERTY_V1"));
            }
            if let Ok(mut slot) = cell.try_borrow_mut() {
                if let Some(state) = slot.as_mut() {
                    if state.armed
                        && !state.consumed
                        && state.case == ProbeCase::PropertyInvalidOnce
                    {
                        state.consumed = true;
                        return Some(w!("NEVROTICK_FIXTURE_NO_SUCH_PROPERTY_V1"));
                    }
                }
            }
            Some(property)
        })
        .ok()
        .flatten()
        .unwrap_or(property);
    // SAFETY: Caller-owned handle/value remain live, static substituted names
    // are valid, and the original flags and value bytes are unchanged.
    let status = unsafe { BCryptSetProperty(handle, actual, value, 0) };
    let _ = STATE.try_with(|cell| {
        if let Ok(mut slot) = cell.try_borrow_mut() {
            if let Some(state) = slot.as_mut() {
                state.push(Operation::SetProperty, state.phase, state.current, status);
            }
        }
    });
    record_cache_property(handle, status);
    status
}

/// Observe provider cleanup without letting ledger ambiguity veto native cleanup.
///
/// # Safety
/// The private guard owns a real successful open and disarms before this call.
/// No observation-state borrow may cross the native call.
pub(super) unsafe fn close_algorithm(handle: BCRYPT_ALG_HANDLE) -> NTSTATUS {
    let observed = STATE
        .try_with(|cell| {
            let mut state = cell.try_borrow_mut().ok()?;
            let state = state.as_mut()?;
            let mut ordinal = None;
            for tracked in state
                .handles
                .iter_mut()
                .flatten()
                .filter(|tracked| tracked.handle == handle && !tracked.close_attempted)
            {
                if tracked.returned || tracked.phase != state.phase || ordinal.is_some() {
                    state.valid = false;
                }
                tracked.close_attempted = true;
                ordinal = Some(tracked.ordinal);
            }
            if ordinal.is_none() {
                state.valid = false;
            }
            Some((state.phase, ordinal))
        })
        .ok()
        .flatten();
    // SAFETY: The guard's ownership, not the diagnostic ledger, authorizes this
    // matching native close. No state borrow remains and the guard is disarmed.
    let status = unsafe { BCryptCloseAlgorithmProvider(handle, 0) };
    let recorded = STATE.try_with(|cell| {
        if let Ok(mut slot) = cell.try_borrow_mut() {
            if let Some(state) = slot.as_mut() {
                for tracked in state.handles.iter_mut().flatten().filter(|tracked| {
                    tracked.handle == handle
                        && tracked.close_attempted
                        && tracked.close_status.is_none()
                }) {
                    tracked.close_status = Some(status.0);
                }
                if status.is_err() || observed.is_none() {
                    state.valid = false;
                }
                let (phase, ordinal) = match observed {
                    Some(observed) => observed,
                    None => (state.phase, None),
                };
                state.push(Operation::ProviderCloseError, phase, ordinal, status);
            }
        }
    });
    if recorded.is_err() {
        CACHE_BAD.store(true, Ordering::Release);
    }
    record_cache_close(handle, CacheOperation::ProviderCloseError, status);
    status
}

/// Close a proven successful-open handle once and retain the actual status.
fn close_owned(handle: BCRYPT_ALG_HANDLE, ordinal: Option<u8>, phase: Phase, operation: Operation) {
    let admitted = STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        let Some(state) = state.as_mut() else {
            return false;
        };
        let Some(tracked) = ordinal
            .and_then(|value| state.handles.get_mut(usize::from(value)))
            .and_then(Option::as_mut)
        else {
            state.valid = false;
            return false;
        };
        let role_matches = match operation {
            Operation::CloseReturned => tracked.returned,
            Operation::CloseRecovery => !tracked.returned,
            _ => false,
        };
        if handle.is_invalid()
            || tracked.handle != handle
            || tracked.phase != phase
            || tracked.close_attempted
            || !role_matches
        {
            state.valid = false;
            return false;
        }
        tracked.close_attempted = true;
        true
    });
    if !admitted {
        return;
    }
    // SAFETY: Only actual successful-open handles reach this private function:
    // returned ownership or an unreturned orphan. Each ledger slot is selected
    // for one close attempt; the original cached initializer is never called.
    let status = unsafe { BCryptCloseAlgorithmProvider(handle, 0) };
    STATE.with(|cell| {
        if let Some(state) = cell.borrow_mut().as_mut() {
            if let Some(tracked) = ordinal
                .and_then(|value| state.handles.get_mut(usize::from(value)))
                .and_then(Option::as_mut)
            {
                tracked.close_status = Some(status.0);
            } else {
                state.valid = false;
            }
            if status.is_err() {
                state.valid = false;
            }
            state.push(operation, phase, ordinal, status);
        }
    });
}

/// Call the private initializer and close only ownership it returns.
fn call_initializer(phase: Phase) {
    let result = super::load_algorithm(
        BCRYPT_ECDH_ALGORITHM,
        BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS::default(),
        Some((BCRYPT_ECC_CURVE_NAME, BCRYPT_ECC_CURVE_25519)),
    );
    let ordinal = STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        let Some(state) = state.as_mut() else {
            return None;
        };
        let returned = result.is_ok();
        match phase {
            Phase::Initial => state.initial_returned = Some(returned),
            Phase::Neighbor => state.neighbor_returned = Some(returned),
        }
        if let Ok(handle) = &result {
            if let Some(tracked) = state
                .current
                .and_then(|value| state.handles.get_mut(usize::from(value)))
                .and_then(Option::as_mut)
            {
                if tracked.handle == *handle && tracked.phase == phase {
                    tracked.returned = true;
                } else {
                    state.valid = false;
                }
            } else {
                state.valid = false;
            }
        }
        state.current
    });
    if let Ok(handle) = result {
        close_owned(handle, ordinal, phase, Operation::CloseReturned);
    }
}

/// Determine the interpretation without converting an absent fact into success.
fn conclusion(state: &State, orphan_count: u8) -> Conclusion {
    if !state.valid {
        return Conclusion::Incomplete;
    }
    if state.case == ProbeCase::HealthyLoad {
        return if state.healthy(Phase::Initial) && orphan_count == 0 {
            Conclusion::Healthy
        } else {
            Conclusion::Incomplete
        };
    }
    if !state.consumed || !state.healthy(Phase::Neighbor) || state.initial_returned != Some(false) {
        return Conclusion::Incomplete;
    }
    let Some(open) = state.event(Operation::Open, Phase::Initial) else {
        return Conclusion::Incomplete;
    };
    match state.case {
        ProbeCase::OpenInvalidOnce
            if !open.succeeded()
                && state
                    .event(Operation::SetProperty, Phase::Initial)
                    .is_none()
                && orphan_count == 0 =>
        {
            Conclusion::OpenFailureObserved
        }
        ProbeCase::PropertyInvalidOnce
            if open.succeeded()
                && orphan_count == 0
                && state
                    .event(Operation::SetProperty, Phase::Initial)
                    .is_some_and(|event| !event.succeeded())
                && state
                    .event(Operation::ProviderCloseError, Phase::Initial)
                    .is_some_and(NativeEvent::succeeded)
                && state
                    .event(Operation::CloseRecovery, Phase::Initial)
                    .is_none() =>
        {
            Conclusion::PropertyFailureCleaned
        }
        ProbeCase::PropertyInvalidOnce
            if open.succeeded()
                && orphan_count == 1
                && state
                    .event(Operation::SetProperty, Phase::Initial)
                    .is_some_and(|event| !event.succeeded())
                && state
                    .event(Operation::CloseRecovery, Phase::Initial)
                    .is_some_and(NativeEvent::succeeded) =>
        {
            Conclusion::CleanupGap
        }
        _ => Conclusion::Incomplete,
    }
}

/// Observe one fixed private-initialization case and its explicit fixture cleanup.
///
/// The snapshot is saved after provider error cleanup and before fixture recovery.
/// A cleanup-gap regression requires a real failed property status and successful
/// later orphan close. No `ShutdownHandle` or cached initializer is constructed.
/// Native calls are synchronous; an admitted parent must bound/reap this process.
///
/// # Errors
/// Rejects a second process-wide call before native work, or unavailable internal
/// state. Unexpected native outcomes/cleanup failures produce an incomplete report.
pub fn run(case: ProbeCase) -> Result<ProbeReport, ProbeError> {
    if USED
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return Err(ProbeError::AlreadyUsed);
    }
    STATE.with(|cell| *cell.borrow_mut() = Some(State::new(case)));
    call_initializer(Phase::Initial);
    STATE.with(|cell| {
        if let Some(state) = cell.borrow_mut().as_mut() {
            state.armed = false;
        }
    });
    let (before_recovery_events, initial_returned, fault_consumed, pending) = STATE.with(|cell| {
        let state = cell.borrow();
        match state.as_ref() {
            Some(state) => (
                state.event_count,
                state.initial_returned == Some(true),
                state.consumed,
                state.handles.map(|slot| {
                    slot.filter(|tracked| !tracked.returned && !tracked.close_attempted)
                }),
            ),
            None => (0, false, false, [None; HANDLE_LIMIT]),
        }
    });
    let mut orphan_count = 0;
    for tracked in pending.into_iter().flatten() {
        orphan_count += 1;
        close_owned(
            tracked.handle,
            Some(tracked.ordinal),
            tracked.phase,
            Operation::CloseRecovery,
        );
    }
    if case != ProbeCase::HealthyLoad {
        STATE.with(|cell| {
            if let Some(state) = cell.borrow_mut().as_mut() {
                state.phase = Phase::Neighbor;
                state.current = None;
            }
        });
        call_initializer(Phase::Neighbor);
    }
    let state = STATE
        .with(|cell| cell.borrow_mut().take())
        .ok_or(ProbeError::StateUnavailable)?;
    let result = conclusion(&state, orphan_count);
    Ok(ProbeReport {
        events: state.events,
        before_recovery_events,
        initial_returned,
        neighbor_returned: state.neighbor_returned,
        orphans_before_recovery: orphan_count,
        fault_consumed,
        conclusion: result,
    })
}
