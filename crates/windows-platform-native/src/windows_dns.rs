// SPDX-License-Identifier: GPL-2.0-only
//! System DNS address query with one native-owned allocation through completion.
//!
//! Callers can abandon the result receiver. The permit survives submission,
//! callback and any admitted cancellation call. This adapter uses Windows DNS
//! Client policy and exposes no custom server, fallback, cache or interface control.

use std::{
    cell::UnsafeCell,
    ffi::c_void,
    mem::MaybeUninit,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle},
    ptr::null_mut,
    sync::{
        Arc, Weak,
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, SyncSender, TryRecvError, sync_channel},
    },
    thread::JoinHandle,
};

use crate::dns_address::{DnsQueryPermit, wait_for_dns_result};
use crate::{
    DnsAddressError, DnsAddressFamily, DnsAddresses, DnsCancellationRequest, DnsQueryName,
};

/// `DNS_REQUEST_PENDING` is the only result that transfers the callback reference.
const REQUEST_PENDING: i32 = 9_506;
/// `DNS_QUERY_REQUEST_VERSION1` and `DNS_QUERY_RESULTS_VERSION1`.
const STRUCTURE_VERSION: u32 = 1;
/// `DnsFreeRecordList` frees the complete list with matching system ownership.
const FREE_RECORD_LIST: i32 = 1;
/// Absolute name, no `NetBT` and no multicast. Windows still owns DNS policy.
const QUERY_OPTIONS: u64 = 0x1000 | 0x0080 | 0x0800;
/// Answer section bits from the SDK's record flags.
const SECTION_MASK: u32 = 0b11;
/// `DnsSectionAnswer`.
const ANSWER_SECTION: u32 = 1;
/// Maximum inspected nodes, including aliases and non-address records.
const MAX_RECORDS: usize = 32;
/// The thread object has reached its signaled termination state.
const WAIT_OBJECT_0: u32 = 0;
/// At most one actual native operation can retain this process's DNS storage.
static QUERY_ACTIVE: AtomicBool = AtomicBool::new(false);

#[link(name = "kernel32")]
#[expect(
    unsafe_code,
    reason = "Windows nonblocking thread termination observation ABI"
)]
unsafe extern "system" {
    /// Query a live waitable handle, with a zero-millisecond timeout here.
    fn WaitForSingleObject(handle: *mut c_void, milliseconds: u32) -> u32;
}

/// Poll a DNS worker's actual Windows thread termination without waiting.
///
/// A false observation, including a wait error, requires retaining custody.
#[must_use]
#[expect(
    unsafe_code,
    reason = "Poll a borrowed Rust-owned thread handle without waiting"
)]
pub fn dns_worker_thread_exited<T>(thread: &JoinHandle<T>) -> bool {
    let handle: BorrowedHandle<'_> = thread.as_handle();
    // SAFETY: the handle stays open for the call through the borrow. A thread
    // handle is waitable and the zero timeout permits only a readiness query.
    unsafe { WaitForSingleObject(handle.as_raw_handle(), 0) == WAIT_OBJECT_0 }
}

/// Exact x64 `DNS_QUERY_REQUEST` version 1.
#[repr(C)]
struct QueryRequest {
    /// Selected `WinDNS` structure version.
    version: u32,
    /// Owned, terminated UTF-16 name kept through callback.
    name: *const u16,
    /// One selected A or AAAA record type.
    kind: u16,
    /// SDK options for the system-managed DNS path.
    options: u64,
    /// Always null. The caller supplies no custom servers.
    servers: *mut c_void,
    /// Default system interface selection.
    interface_index: u32,
    /// Exact system calling convention with a non-unwinding callback.
    callback: Option<extern "system" fn(*mut c_void, *mut QueryResult)>,
    /// One raw reference to the original query allocation.
    context: *mut c_void,
}

/// Exact x64 `DNS_QUERY_RESULT` version 1.
#[repr(C)]
struct QueryResult {
    /// Result structure version initialized by this caller.
    version: u32,
    /// Exact status supplied by the native completion.
    status: i32,
    /// Options returned by the DNS Client.
    options: u64,
    /// Owned record allocation provided by `WinDNS` on completion.
    records: *mut RecordHeader,
    /// Reserved by `WinDNS` and initialized null.
    reserved: *mut c_void,
}

/// Exact opaque `DNS_QUERY_CANCEL` SDK storage.
#[repr(C)]
struct QueryCancel {
    /// Written and interpreted only by `WinDNS`.
    reserved: [i8; 32],
}

/// Fixed x64 prefix of a `WinDNS` wide record, before its variable data union.
#[repr(C)]
struct RecordHeader {
    /// Next node in the same `WinDNS`-owned allocation.
    next: *mut Self,
    /// Native name pointer. The boundary never dereferences or exports it.
    name: *mut u16,
    /// Exact native RR type.
    kind: u16,
    /// Size of the active union payload in bytes.
    data_len: u16,
    /// Contains the native record section and representation bits.
    flags: u32,
    /// TTL is retained by the DNS Client and supplies no clock authority.
    ttl: u32,
    /// `WinDNS`-owned reserved word.
    reserved: u32,
}

const _: [(); 64] = [(); size_of::<QueryRequest>()];
const _: [(); 32] = [(); size_of::<QueryResult>()];
const _: [(); 32] = [(); size_of::<QueryCancel>()];
const _: [(); 1] = [(); align_of::<QueryCancel>()];
const _: [(); 32] = [(); size_of::<RecordHeader>()];
const _: [(); 48] = [(); std::mem::offset_of!(QueryRequest, callback)];
const _: [(); 56] = [(); std::mem::offset_of!(QueryRequest, context)];
const _: [(); 16] = [(); std::mem::offset_of!(QueryResult, records)];
const _: [(); 16] = [(); std::mem::offset_of!(RecordHeader, kind)];
const _: [(); 20] = [(); std::mem::offset_of!(RecordHeader, flags)];

#[link(name = "dnsapi")]
#[expect(
    unsafe_code,
    reason = "WinDNS import ABI for version 1 address queries"
)]
unsafe extern "system" {
    fn DnsQueryEx(
        request: *const QueryRequest,
        result: *mut QueryResult,
        cancel: *mut QueryCancel,
    ) -> i32;
    fn DnsCancelQuery(cancel: *const QueryCancel) -> i32;
    fn DnsFree(pointer: *mut c_void, free_type: i32);
}

/// Retained by both the submitter and the possible `WinDNS` completion callback.
struct NativeQuery {
    /// Returned only after the last actual native owner has completed.
    _permit: DnsQueryPermit<'static>,
    /// Immutable name and operation selection, held at a stable allocation.
    name: DnsQueryName,
    /// Only this RR family can produce address payloads.
    family: DnsAddressFamily,
    /// Initialized once before `WinDNS` and pinned through its completion path.
    request: UnsafeCell<MaybeUninit<QueryRequest>>,
    /// `WinDNS` writes the initialized v1 result until immediate return or callback.
    result: UnsafeCell<QueryResult>,
    /// Retained native cancellation token at its original address.
    cancel: UnsafeCell<QueryCancel>,
    /// Callback entry or cancellation admission has claimed the pending request.
    completion_or_cancel: AtomicBool,
    /// One bounded publication. Dropping the receiver abandons delivery only.
    completion: SyncSender<Result<DnsAddresses, DnsAddressError>>,
}

#[expect(
    unsafe_code,
    reason = "Pin the exclusive WinDNS result writer across its callback"
)]
// SAFETY: the name, family and once-initialized request are immutable during
// submission. Only the selected completion path reads the native result and
// no published value retains a record pointer. WinDNS alone accesses the
// cancellation bytes, with a single caller after pending submission returns.
unsafe impl Sync for NativeQuery {}

#[expect(
    unsafe_code,
    reason = "Permit callback-side release of the owned DNS allocation"
)]
// SAFETY: the callback may release this allocation on its native thread.
// The result has no thread affinity and the channel is thread-safe.
unsafe impl Send for NativeQuery {}

/// Exactly one owner of a completed native record list.
struct RecordList(
    /// Head accepted from the matching completed `WinDNS` result.
    *mut RecordHeader,
);

impl Drop for RecordList {
    #[expect(
        unsafe_code,
        reason = "Release the completed DNS record list with its matching SDK allocator"
    )]
    fn drop(&mut self) {
        if !self.0.is_null() {
            // SAFETY: the completed result owns the list once, and no copied
            // address or external receiver has borrowed a native node.
            unsafe { DnsFree(self.0.cast(), FREE_RECORD_LIST) };
        }
    }
}

/// Copy the complete bounded family from the actual owned answer section.
/// # Errors
/// Refuse overlong lists, unexpected address sizes or capacity overflow.
#[expect(
    unsafe_code,
    reason = "Read native record prefixes and sized address payloads under list custody"
)]
fn copy_answers(
    list: &RecordList,
    family: DnsAddressFamily,
) -> Result<DnsAddresses, DnsAddressError> {
    let mut current: *mut RecordHeader = list.0;
    let mut answer: DnsAddresses = DnsAddresses::empty();
    for _ in 0..MAX_RECORDS {
        if current.is_null() {
            break;
        }
        let prefix: *const RecordHeader = current.cast_const();
        // SAFETY: this initialized pointer starts the WinDNS-owned fixed prefix.
        let next: *mut RecordHeader = unsafe { prefix.cast::<*mut RecordHeader>().read() };
        let kind: *const u16 = prefix
            .wrapping_byte_add(std::mem::offset_of!(RecordHeader, kind))
            .cast();
        // SAFETY: WinDNS initialized the record type at the checked prefix offset.
        let kind: u16 = unsafe { kind.read() };
        let flags: *const u32 = prefix
            .wrapping_byte_add(std::mem::offset_of!(RecordHeader, flags))
            .cast();
        // SAFETY: only initialized native section bits are read from this field.
        let flags: u32 = unsafe { flags.read() };
        if kind == family.wire_record_type() && flags & SECTION_MASK == ANSWER_SECTION {
            let length: *const u16 = prefix
                .wrapping_byte_add(std::mem::offset_of!(RecordHeader, data_len))
                .cast();
            // SAFETY: the record's fixed prefix contains its active data length.
            let data_len: u16 = unsafe { length.read() };
            let data: *const u8 = prefix.wrapping_byte_add(size_of::<RecordHeader>()).cast();
            let address: IpAddr = match (family, data_len) {
                (DnsAddressFamily::V4, 4) => {
                    // SAFETY: exact matching A type and four-byte union payload.
                    IpAddr::V4(Ipv4Addr::from(unsafe {
                        data.cast::<[u8; 4]>().read_unaligned()
                    }))
                }
                (DnsAddressFamily::V6, 16) => {
                    // SAFETY: exact matching AAAA type and sixteen-byte union payload.
                    IpAddr::V6(Ipv6Addr::from(unsafe {
                        data.cast::<[u8; 16]>().read_unaligned()
                    }))
                }
                _ => return Err(DnsAddressError::Malformed),
            };
            answer.push(address)?;
        }
        current = next;
    }
    if !current.is_null() {
        return Err(DnsAddressError::Capacity);
    }
    if answer.is_empty() {
        return Err(DnsAddressError::Empty);
    }
    Ok(answer)
}

/// Settle one completed result and publish only copied addresses or a safe error.
#[expect(
    unsafe_code,
    reason = "Read the initialized result only at actual immediate or callback completion"
)]
fn complete(query: &NativeQuery, result: *mut QueryResult, immediate: Option<i32>) {
    let outcome: Result<DnsAddresses, DnsAddressError> = if result.is_null() {
        Err(DnsAddressError::Malformed)
    } else {
        let records: *const *mut RecordHeader = result
            .wrapping_byte_add(std::mem::offset_of!(QueryResult, records))
            .cast();
        // SAFETY: both documented completion paths supply the record pointer
        // in the completed result. Read only that field to obtain ownership.
        let records: RecordList = RecordList(unsafe { records.read() });
        if let Some(code) = immediate.filter(|code| *code != 0) {
            Err(DnsAddressError::Native(code))
        } else {
            let version: *const u32 = result.cast();
            // SAFETY: the completed v1 header contains the initialized version.
            let version: u32 = unsafe { version.read() };
            let status: *const i32 = result
                .wrapping_byte_add(std::mem::offset_of!(QueryResult, status))
                .cast();
            // SAFETY: completion supplies this final status and the SDK offset.
            let status: i32 = unsafe { status.read() };
            match status {
                _ if version != STRUCTURE_VERSION || status == REQUEST_PENDING => {
                    Err(DnsAddressError::Malformed)
                }
                0 => copy_answers(&records, query.family),
                code => Err(DnsAddressError::Native(code)),
            }
        }
    };
    let _delivery = query.completion.try_send(outcome);
}

/// Reclaim only the reference transferred on the actual pending path.
#[expect(
    unsafe_code,
    reason = "Recover the one callback reference issued by Arc::into_raw"
)]
extern "system" fn on_complete(context: *mut c_void, result: *mut QueryResult) {
    if context.is_null() {
        return;
    }
    // SAFETY: only DnsQueryEx receives this context. The SDK invokes this
    // callback once for a pending request and never for immediate return.
    let query: Arc<NativeQuery> = unsafe { Arc::from_raw(context.cast::<NativeQuery>()) };
    query.completion_or_cancel.store(true, Ordering::Release);
    complete(&query, result, None);
}

/// One owned receiver for an actual Windows DNS query.
///
/// Drop abandons delivery. Pending `WinDNS` work retains its original storage
/// and the single process permit through actual callback completion.
#[must_use = "retain the receiver to observe the submitted DNS outcome"]
pub struct DnsAddressQuery {
    /// Taken once when the result is delivered or the channel closes.
    receiver: Option<Receiver<Result<DnsAddresses, DnsAddressError>>>,
    /// Upgrade only to attempt cancellation. An unread result cannot retain the permit.
    pending: Option<Weak<NativeQuery>>,
}

impl DnsAddressQuery {
    /// Submit one bounded absolute A or AAAA lookup through Windows DNS Client.
    ///
    /// This is a native call and can complete inline. Callers that need a
    /// nonblocking owner must perform submission on their admitted worker.
    /// # Errors
    /// Refuse before submission while another native query owns the permit.
    /// Native success or failure is available through `try_complete()`.
    #[expect(
        unsafe_code,
        reason = "Submit pinned v1 WinDNS arguments and settle its documented return paths"
    )]
    pub fn start(name: DnsQueryName, family: DnsAddressFamily) -> Result<Self, DnsAddressError> {
        let permit: DnsQueryPermit<'static> = DnsQueryPermit::acquire(&QUERY_ACTIVE)?;
        let (completion, receiver) = sync_channel(1);
        let query: Arc<NativeQuery> = Arc::new(NativeQuery {
            _permit: permit,
            name,
            family,
            request: UnsafeCell::new(MaybeUninit::uninit()),
            result: UnsafeCell::new(QueryResult {
                version: STRUCTURE_VERSION,
                status: 0,
                options: 0,
                records: null_mut(),
                reserved: null_mut(),
            }),
            cancel: UnsafeCell::new(QueryCancel { reserved: [0; 32] }),
            completion_or_cancel: AtomicBool::new(false),
            completion,
        });
        let context: *const NativeQuery = Arc::into_raw(Arc::clone(&query));
        let request: QueryRequest = QueryRequest {
            version: STRUCTURE_VERSION,
            name: query.name.as_ptr(),
            kind: family.wire_record_type(),
            options: QUERY_OPTIONS,
            servers: null_mut(),
            interface_index: 0,
            callback: Some(on_complete),
            context: context.cast_mut().cast(),
        };
        let pinned_request: *mut QueryRequest = query.request.get().cast();
        // SAFETY: this initializes the retained request exactly once, before
        // passing any of its pointers to WinDNS. Nothing writes it afterward.
        unsafe { pinned_request.write(request) };
        let output: *mut QueryResult = query.result.get();
        // SAFETY: the request is initialized, and all three structures retain
        // their original addresses through the selected native completion path.
        let status: i32 = unsafe { DnsQueryEx(pinned_request, output, query.cancel.get()) };
        let pending: Option<Weak<NativeQuery>> = if status == REQUEST_PENDING {
            Some(Arc::downgrade(&query))
        } else {
            // SAFETY: the documented immediate path never invokes a callback.
            // The submission relinquishes its extra raw reference here.
            let immediate: Arc<NativeQuery> = unsafe { Arc::from_raw(context) };
            complete(&immediate, output, Some(status));
            None
        };
        Ok(Self {
            receiver: Some(receiver),
            pending,
        })
    }

    /// Consume the result once when available without waiting or restarting.
    ///
    /// Callers distinguish pending from a consumed handle by retaining their
    /// own operation state. A closed receiver without an outcome is malformed.
    pub fn try_complete(&mut self) -> Option<Result<DnsAddresses, DnsAddressError>> {
        let outcome: Result<DnsAddresses, DnsAddressError> =
            match self.receiver.as_ref()?.try_recv() {
                Ok(result) => result,
                Err(TryRecvError::Empty) => return None,
                Err(TryRecvError::Disconnected) => Err(DnsAddressError::Malformed),
            };
        self.receiver = None;
        self.pending = None;
        Some(outcome)
    }

    /// Wait on an admitted blocking worker for actual completion and consume it.
    ///
    /// A deadline or caller retirement belongs to the worker owner. This method
    /// may wait indefinitely if the native callback never arrives.
    /// # Errors
    /// Preserve the completed DNS error, or refuse an absent outcome.
    pub fn wait(mut self) -> Result<DnsAddresses, DnsAddressError> {
        let receiver: Receiver<Result<DnsAddresses, DnsAddressError>> =
            self.receiver.take().ok_or(DnsAddressError::Malformed)?;
        receiver.recv().map_err(|_| DnsAddressError::Malformed)?
    }

    /// Wait for actual completion on the admitted blocking worker. On retirement,
    /// ask Windows to cancel once when pending. The observation must be nonblocking.
    /// A missing callback can retain the worker indefinitely.
    pub fn wait_with_retirement(
        mut self,
        retired: impl FnMut() -> bool,
    ) -> (
        Result<DnsAddresses, DnsAddressError>,
        DnsCancellationRequest,
    ) {
        let Some(receiver): Option<Receiver<Result<DnsAddresses, DnsAddressError>>> =
            self.receiver.take()
        else {
            return (
                Err(DnsAddressError::Malformed),
                DnsCancellationRequest::Skipped,
            );
        };
        wait_for_dns_result(&receiver, retired, || self.request_cancellation())
    }

    /// Claim the one cancellation admission for this exact pending query.
    #[expect(
        unsafe_code,
        reason = "Submit cancellation using the original WinDNS-owned token"
    )]
    fn request_cancellation(&mut self) -> DnsCancellationRequest {
        let Some(query): Option<Arc<NativeQuery>> = self
            .pending
            .take()
            .and_then(|pending: Weak<NativeQuery>| pending.upgrade())
        else {
            return DnsCancellationRequest::Skipped;
        };
        if query.completion_or_cancel.swap(true, Ordering::AcqRel) {
            return DnsCancellationRequest::Skipped;
        }
        // SAFETY: a pending DnsQueryEx returned the token at this address.
        // Like the SDK DNSAsyncQuery sample, the worker claims cancellation
        // before invoking it and retains the allocation if the callback wins
        // the race to actual completion. The callback independently owns its
        // Arc through cleanup. No Rust code reads or copies the token bytes.
        let status: i32 = unsafe { DnsCancelQuery(query.cancel.get().cast_const()) };
        if status == 0 {
            DnsCancellationRequest::Requested
        } else {
            DnsCancellationRequest::Rejected(status)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        error::Error,
        io,
        sync::mpsc::{self, Receiver, Sender},
        thread,
        time::{Duration, Instant},
    };

    use super::dns_worker_thread_exited;

    /// Both sides have a finite watchdog, including during unwinding.
    const WATCHDOG: Duration = Duration::from_secs(5);

    /// Keep the OS thread alive after the Rust worker function returns.
    struct ExitBarrier {
        /// Notify the test only after thread-local destruction starts.
        entered: Sender<()>,
        /// Wait until the test observes the unsignaled thread object.
        release: Receiver<()>,
    }

    impl Drop for ExitBarrier {
        fn drop(&mut self) {
            if self.entered.send(()).is_ok() {
                let _released: Result<(), mpsc::RecvTimeoutError> =
                    self.release.recv_timeout(WATCHDOG);
            }
        }
    }

    thread_local! {
        /// Install the barrier only inside the test worker.
        static AT_EXIT: RefCell<Option<ExitBarrier>> = const { RefCell::new(None) };
    }

    /// Separate Rust function return from actual Windows thread termination.
    /// # Errors
    /// Report test-worker start, wait or join failure.
    /// # Panics
    /// Reject premature completion or an unsignaled thread after release.
    #[test]
    fn thread_exit_poll_waits_for_thread_local_teardown() -> Result<(), Box<dyn Error>> {
        let (entered, began_exit): (Sender<()>, Receiver<()>) = mpsc::channel();
        let (release, release_exit): (Sender<()>, Receiver<()>) = mpsc::channel();
        let worker: thread::JoinHandle<()> = thread::Builder::new().spawn(move || {
            AT_EXIT.with(|slot| {
                *slot.borrow_mut() = Some(ExitBarrier {
                    entered,
                    release: release_exit,
                });
            });
        })?;
        began_exit.recv_timeout(WATCHDOG)?;
        assert!(worker.is_finished());
        assert!(!dns_worker_thread_exited(&worker));
        release.send(())?;

        let deadline: Instant = Instant::now() + WATCHDOG;
        while !dns_worker_thread_exited(&worker) {
            assert!(Instant::now() < deadline, "released worker never signaled");
            thread::sleep(Duration::from_millis(1));
        }
        worker
            .join()
            .map_err(|_| io::Error::other("test worker panicked"))?;
        Ok(())
    }
}
