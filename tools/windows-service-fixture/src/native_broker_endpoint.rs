// SPDX-License-Identifier: GPL-2.0-only
//! Fixed broker reservation and sealed fixture exchange with retained native completion.

use std::{
    cell::UnsafeCell,
    marker::PhantomPinned,
    os::windows::io::{
        AsHandle, AsRawHandle, BorrowedHandle, FromRawHandle, IntoRawHandle, OwnedHandle,
    },
    pin::Pin,
    ptr,
    task::Poll,
};

use windows_sys::{
    Win32::{
        Foundation::{
            CloseHandle, ERROR_INVALID_HANDLE, ERROR_INVALID_PARAMETER, ERROR_IO_INCOMPLETE,
            ERROR_IO_PENDING, ERROR_NOT_FOUND, ERROR_OPERATION_ABORTED, ERROR_PIPE_CONNECTED,
            GetLastError, HANDLE, INVALID_HANDLE_VALUE,
        },
        Security::{
            ACL, ACL_REVISION, InitializeAcl, InitializeSecurityDescriptor, PSECURITY_DESCRIPTOR,
            SE_DACL_PROTECTED, SECURITY_ATTRIBUTES, SECURITY_DESCRIPTOR,
            SetSecurityDescriptorControl, SetSecurityDescriptorDacl,
        },
        Storage::FileSystem::{
            FILE_FLAG_FIRST_PIPE_INSTANCE, FILE_FLAG_OVERLAPPED, PIPE_ACCESS_DUPLEX,
        },
        System::{
            IO::{CancelIoEx, GetOverlappedResult, OVERLAPPED},
            Pipes::{ConnectNamedPipe, CreateNamedPipeW, PIPE_REJECT_REMOTE_CLIENTS},
            Threading::CreateEventW,
        },
    },
    core::BOOL,
};

#[cfg(test)]
use std::time::Instant;

use crate::{
    NativeError, NativeOperation,
    broker_endpoint::{AcceptCompletion, AcceptStart, Reservation},
    native_broker_wait::{CompletionWakeState, PendingCompletionWake},
};

#[cfg(test)]
use crate::{
    broker_clock_change::ReplyRevision,
    broker_endpoint::{FirstReadCompletion, FirstReadStart, FirstReplyCompletion, FirstReplyStart},
    host::{Clock, ConnectionEdge, RunningConnection},
    native_pipe_identity::{PipeIdentity, RetainedFixtureClient},
    native_pipe_pair::LabUnconnectedDuplexPipe,
    native_pipe_read::{
        AttributedStatusRead, CompletedFirstRead, IdentifiedReadError, NativePipeRead,
        NativeReadError, NativeReadStart,
    },
    native_pipe_write::{NativePipeWrite, NativeWriteError, NativeWritePoll, NativeWriteStart},
    private_clock_wire::MAX_FRAME_LEN,
};

/// UTF-16 projection of `\\.\pipe\NevroTick-Lab-S02-Broker-Reservation` and its terminator.
const RESERVATION_NAME: [u16; 46] = [
    92, 92, 46, 92, 112, 105, 112, 101, 92, 78, 101, 118, 114, 111, 84, 105, 99, 107, 45, 76, 97,
    98, 45, 83, 48, 50, 45, 66, 114, 111, 107, 101, 114, 45, 82, 101, 115, 101, 114, 118, 97, 116,
    105, 111, 110, 0,
];

/// The SDK descriptor revision used by `InitializeSecurityDescriptor`.
const DESCRIPTOR_REVISION: u32 = 1;
/// A descriptor storage length outside the Win32 argument range.
const INVALID_DESCRIPTOR_SIZE: NativeError = NativeError::new(
    NativeOperation::BrokerEndpointDescriptor,
    ERROR_INVALID_PARAMETER,
);

/// DWORD-aligned storage for the empty ACL referenced by the absolute descriptor.
#[repr(C, align(4))]
struct EmptyAcl(ACL);

const _: () = assert!(size_of::<EmptyAcl>() == 8);
const _: () = assert!(size_of::<SECURITY_ATTRIBUTES>() == 24);

/// Submission and cancellation history of the sole exact native record.
enum AcceptState {
    /// The event and record are initialized; no accept has been attempted.
    Ready,
    /// An accept may still be outstanding on the original pipe.
    Pending {
        /// The same hint remains charged from queueing through cancellation.
        wake: CompletionWakeState,
        /// The one cancellation request was attempted. Completion is still owed.
        cancellation_requested: bool,
    },
    /// The original accept connected without an attempted cancellation.
    Connected,
    /// Submission failed, or a pending accept settled after cancellation.
    Retired,
}

/// Pinned, single-use storage for the only kernel-written record.
struct AcceptStorage {
    /// Interior mutability permits native writes while the Rust owner is retained.
    overlapped: UnsafeCell<OVERLAPPED>,
    /// Prevent moving the submitted storage out of its allocation.
    _pin: PhantomPinned,
}

/// Own the allocation independently of the Rust state used to decide release.
struct AcceptRecord {
    /// Keep the original allocation alive and still during all pending work.
    storage: Pin<Box<AcceptStorage>>,
    /// Rust-only state independent of the record written by Windows.
    state: AcceptState,
}

impl AcceptRecord {
    /// Borrow the exact pointer for submission, cancellation, or observation.
    fn pointer(&self) -> *mut OVERLAPPED {
        self.storage.as_ref().get_ref().overlapped.get()
    }
}

impl Drop for AcceptRecord {
    /// Fail fast on invalid destruction while the kernel may still write.
    fn drop(&mut self) {
        if matches!(self.state, AcceptState::Pending { .. }) {
            // The service owner retains this allocation during ordinary Stop.
            // An invalid drop cannot release pending storage or claim Stopped.
            std::process::abort();
        }
    }
}

/// One reply right, issued by attribution and spent before any native submission.
#[cfg(test)]
enum FirstReplyState {
    /// The exact first read has yet to produce a matched, reverted identity.
    WaitingForAttribution,
    /// The worker may submit its admitted reply once.
    Ready,
    /// The exact submitted byte count requires one terminal observation.
    Pending(usize),
    /// No further write is authorized on this connection.
    Retired,
}

/// Separate deliberate final refusal from a failed native or slot operation.
#[cfg(test)]
enum ReplySubmitError {
    /// Admission closed before Windows acquired the prepared write.
    Refused,
    /// Slot preparation or native submission failed.
    Write(NativeWriteError),
}

#[cfg(test)]
impl From<NativeWriteError> for ReplySubmitError {
    fn from(error: NativeWriteError) -> Self {
        Self::Write(error)
    }
}

/// The write and read records are bound to the same original server owner.
#[cfg(test)]
struct FirstExchange {
    write: NativePipeWrite<NativePipeRead>,
    reply: FirstReplyState,
    /// Retain the original client identity through the read and the one write.
    peer: RetainedFixtureClient,
    /// A fixture action after the exact terminal count is collected, before confirmation.
    observe_full_write: Option<fn()>,
}

/// The original pipe has exactly one native handle owner through its operations.
enum EndpointPipe {
    /// Fixed reservation and existing unarmed fixtures.
    Reservation(OwnedHandle),
    /// Sealed fixture server plus the already prepared read record and event.
    #[cfg(test)]
    FirstRead(NativePipeRead),
    /// Sealed fixture prepared for one guarded reply on its original server.
    #[cfg(test)]
    FirstExchange(Box<FirstExchange>),
}

impl EndpointPipe {
    /// Borrow the live pipe without transferring its native ownership.
    fn pipe(&self) -> BorrowedHandle<'_> {
        match self {
            Self::Reservation(handle) => handle.as_handle(),
            #[cfg(test)]
            Self::FirstRead(read) => read.pipe(),
            #[cfg(test)]
            Self::FirstExchange(exchange) => exchange.write.pipe(),
        }
    }

    /// Consume the pipe and its owned events after physical completion.
    ///
    /// # Errors
    /// Preserve the first explicit native release failure.
    fn close(self) -> Result<(), NativeError> {
        match self {
            Self::Reservation(handle) => close_handle(handle),
            #[cfg(test)]
            Self::FirstRead(read) => close_first_read(read),
            #[cfg(test)]
            Self::FirstExchange(exchange) => {
                let (read, write_event): (NativePipeRead, OwnedHandle) =
                    exchange.write.into_first_reply_parts();
                let read_result: Result<(), NativeError> = close_first_read(read);
                let write_event_result: Result<(), NativeError> = close_handle(write_event);
                read_result.and(write_event_result)
            }
        }
    }
}

/// Unique custody of the pipe and its sole accept record and event.
#[must_use = "retain the original reservation and any queued accept through explicit release"]
pub(super) struct NativeBrokerEndpoint {
    /// Drop the record first so invalid destruction cannot release its handles.
    accept: AcceptRecord,
    /// The noninherited manual-reset completion event belongs to this accept.
    event: OwnedHandle,
    /// The creation result, with best-effort release if explicit close is bypassed.
    pipe: EndpointPipe,
    /// Only the sealed test pipe can hand its connection to the fixture transport.
    #[cfg(test)]
    fixture_pipe: bool,
    /// Expected identity of the one fixture client, selected before acquisition.
    #[cfg(test)]
    status_client: Option<Box<PipeIdentity>>,
}

/// Refusal to hand the original server handle to the duplex fixture.
#[cfg(test)]
pub(super) enum FixtureConnectionError {
    /// Retains the original endpoint and any unsettled accept.
    Ineligible(NativeBrokerEndpoint),
    /// The accept event close failed. The idle pipe also received one close attempt.
    AcceptEventClose(NativeError),
}

impl NativeBrokerEndpoint {
    /// Prepare a local overlapped instance and its independent completion event.
    ///
    /// # Errors
    /// Reject an unrepresentable storage size and preserve the first descriptor
    /// or creation failure. A collision leaves the existing object untouched,
    /// and the idle provisional event drops without a submitted operation.
    pub(super) fn acquire() -> Result<Self, NativeError> {
        let acl_bytes: u32 =
            u32::try_from(size_of::<EmptyAcl>()).map_err(|_| INVALID_DESCRIPTOR_SIZE)?;
        let attributes_bytes: u32 =
            u32::try_from(size_of::<SECURITY_ATTRIBUTES>()).map_err(|_| INVALID_DESCRIPTOR_SIZE)?;
        let mut acl: EmptyAcl = EmptyAcl(ACL::default());
        let mut descriptor: SECURITY_DESCRIPTOR = SECURITY_DESCRIPTOR::default();
        let descriptor_ptr: PSECURITY_DESCRIPTOR = (&raw mut descriptor).cast();

        #[expect(unsafe_code, reason = "initialize one aligned empty ACL header")]
        // SAFETY: The aligned field provides a writable DWORD-sized ACL header
        // buffer. No ACE storage is needed, and the buffer outlives the descriptor.
        let initialized: BOOL = unsafe { InitializeAcl(&raw mut acl.0, acl_bytes, ACL_REVISION) };
        if initialized == 0 {
            return Err(last_error(NativeOperation::BrokerEndpointDescriptor));
        }

        #[expect(unsafe_code, reason = "initialize one stack-owned absolute descriptor")]
        // SAFETY: The pointer addresses a writable, aligned SECURITY_DESCRIPTOR.
        let initialized: BOOL =
            unsafe { InitializeSecurityDescriptor(descriptor_ptr, DESCRIPTOR_REVISION) };
        if initialized == 0 {
            return Err(last_error(NativeOperation::BrokerEndpointDescriptor));
        }

        #[expect(unsafe_code, reason = "install only the present empty DACL")]
        // SAFETY: The descriptor is an initialized absolute descriptor, and the
        // referenced ACL remains initialized and unmoved through creation.
        let assigned: BOOL =
            unsafe { SetSecurityDescriptorDacl(descriptor_ptr, 1, &raw const acl.0, 0) };
        if assigned == 0 {
            return Err(last_error(NativeOperation::BrokerEndpointDescriptor));
        }

        #[expect(unsafe_code, reason = "prevent inherited ACEs on the reservation")]
        // SAFETY: The descriptor is initialized and SE_DACL_PROTECTED is a
        // supported control mask for this operation.
        let protected: BOOL = unsafe {
            SetSecurityDescriptorControl(descriptor_ptr, SE_DACL_PROTECTED, SE_DACL_PROTECTED)
        };
        if protected == 0 {
            return Err(last_error(NativeOperation::BrokerEndpointDescriptor));
        }

        let attributes: SECURITY_ATTRIBUTES = SECURITY_ATTRIBUTES {
            nLength: attributes_bytes,
            lpSecurityDescriptor: descriptor_ptr,
            bInheritHandle: 0,
        };
        let (accept, event): (AcceptRecord, OwnedHandle) = prepare_accept()?;
        #[expect(unsafe_code, reason = "reserve only the fixed local lab pipe name")]
        // SAFETY: The name has a terminating NUL. The initialized attributes,
        // descriptor and ACL remain valid through this synchronous call.
        let handle: HANDLE = unsafe {
            CreateNamedPipeW(
                RESERVATION_NAME.as_ptr(),
                PIPE_ACCESS_DUPLEX | FILE_FLAG_FIRST_PIPE_INSTANCE | FILE_FLAG_OVERLAPPED,
                PIPE_REJECT_REMOTE_CLIENTS,
                1,
                0,
                0,
                0,
                &raw const attributes,
            )
        };
        if handle == INVALID_HANDLE_VALUE {
            return Err(last_error(NativeOperation::BrokerEndpointCreate));
        }
        if handle.is_null() {
            return Err(NativeError::new(
                NativeOperation::BrokerEndpointCreate,
                ERROR_INVALID_HANDLE,
            ));
        }

        #[expect(unsafe_code, reason = "adopt the private native server handle")]
        // SAFETY: A successful creation returns a uniquely owned real handle
        // that must be released with CloseHandle.
        let handle: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(handle) };
        Ok(Self {
            accept,
            event,
            pipe: EndpointPipe::Reservation(handle),
            #[cfg(test)]
            fixture_pipe: false,
            #[cfg(test)]
            status_client: None,
        })
    }

    /// Bind the original unconnected test server to its one native accept.
    ///
    /// # Errors
    /// Preserve event creation failure. The idle fixture server has no queued
    /// accept and drops on refusal.
    #[cfg(test)]
    pub(super) fn for_duplex_fixture(
        server: LabUnconnectedDuplexPipe,
    ) -> Result<Self, NativeError> {
        let (accept, event): (AcceptRecord, OwnedHandle) = prepare_accept()?;
        Ok(Self {
            accept,
            event,
            pipe: EndpointPipe::Reservation(server.into_handle()),
            fixture_pipe: true,
            status_client: None,
        })
    }

    /// Prepare one read on the original sealed fixture server before readiness.
    ///
    /// # Errors
    /// Preserves preparation failure with no submitted native storage.
    #[cfg(test)]
    pub(super) fn for_first_read_fixture(
        server: LabUnconnectedDuplexPipe,
        status_client: Option<PipeIdentity>,
    ) -> Result<Self, NativeError> {
        let (accept, event): (AcceptRecord, OwnedHandle) = prepare_accept()?;
        let read: NativePipeRead = NativePipeRead::for_first_read(server)
            .map_err(|error| read_error(NativeOperation::BrokerEndpointReadEvent, error))?;
        Ok(Self {
            accept,
            event,
            pipe: EndpointPipe::FirstRead(read),
            fixture_pipe: false,
            status_client: status_client.map(Box::new),
        })
    }

    /// Prepare separate read and write records while retaining one original server.
    /// # Errors
    /// Preserve preparation failure without submitting any native operation.
    #[cfg(test)]
    pub(super) fn for_first_exchange_fixture(
        server: LabUnconnectedDuplexPipe,
        peer: RetainedFixtureClient,
        observe_full_write: Option<fn()>,
    ) -> Result<Self, NativeError> {
        let (accept, event): (AcceptRecord, OwnedHandle) = prepare_accept()?;
        let read: NativePipeRead = NativePipeRead::for_first_read(server)
            .map_err(|error| read_error(NativeOperation::BrokerEndpointReadEvent, error))?;
        let write: NativePipeWrite<NativePipeRead> = NativePipeWrite::for_first_reply(read)
            .map_err(|error| write_error(NativeOperation::BrokerEndpointReplyEvent, error))?;
        Ok(Self {
            accept,
            event,
            pipe: EndpointPipe::FirstExchange(Box::new(FirstExchange {
                write,
                reply: FirstReplyState::WaitingForAttribution,
                peer,
                observe_full_write,
            })),
            fixture_pipe: false,
            status_client: None,
        })
    }

    /// Hand off exactly the connected fixture handle after retiring its accept.
    ///
    /// # Errors
    /// Return ineligible endpoints intact, including pending or cancelled work.
    /// If the accept event close fails, attempt one pipe close and retain the
    /// earlier error without handing off a connection.
    #[cfg(test)]
    pub(super) fn into_connected_fixture(self) -> Result<OwnedHandle, FixtureConnectionError> {
        if !self.fixture_pipe || !matches!(self.accept.state, AcceptState::Connected) {
            return Err(FixtureConnectionError::Ineligible(self));
        }
        let Self {
            accept,
            event,
            pipe: EndpointPipe::Reservation(handle),
            fixture_pipe: _,
            status_client: _,
        } = self
        else {
            return Err(FixtureConnectionError::Ineligible(self));
        };
        drop(accept);
        if let Err(error) = close_handle(event) {
            let _pipe_result: Result<(), NativeError> = close_handle(handle);
            return Err(FixtureConnectionError::AcceptEventClose(error));
        }
        Ok(handle)
    }

    /// Release the original pipe and owned events once, with no outstanding native I/O.
    ///
    /// # Errors
    /// Preserve the first close failure with release unconfirmed. Consuming the
    /// handles before the calls prevents repeating an uncertain release.
    pub(super) fn close(self) -> Result<(), NativeError> {
        let Self {
            accept,
            event,
            pipe,
            #[cfg(test)]
                fixture_pipe: _,
            #[cfg(test)]
                status_client: _,
        } = self;
        drop(accept);
        let pipe_result: Result<(), NativeError> = pipe.close();
        let event_result: Result<(), NativeError> = close_handle(event);
        pipe_result.and(event_result)
    }
}

impl Reservation for NativeBrokerEndpoint {
    /// Submit once using the prepared pipe and event. Keep storage on pending.
    ///
    /// # Errors
    /// Refuses a repeated submission and preserves any native submission error.
    fn begin_accept(&mut self) -> Result<AcceptStart, NativeError> {
        let operation: NativeOperation = NativeOperation::BrokerEndpointAcceptSubmit;
        if !matches!(self.accept.state, AcceptState::Ready) {
            return Err(NativeError::new(operation, ERROR_INVALID_PARAMETER));
        }
        self.accept.state = AcceptState::Pending {
            wake: CompletionWakeState::new(),
            cancellation_requested: false,
        };
        #[expect(
            unsafe_code,
            reason = "submit one accept with the owned overlapped record"
        )]
        // SAFETY: The pipe was opened overlapped. The record is pinned, is
        // exclusively reserved for this operation and retains its manual-reset
        // event with the original pipe until observed completion.
        let connected: BOOL =
            unsafe { ConnectNamedPipe(self.pipe.pipe().as_raw_handle(), self.accept.pointer()) };
        if connected != 0 {
            self.accept.state = AcceptState::Connected;
            return Ok(AcceptStart::Connected);
        }
        let error: NativeError = last_error(operation);
        if error.code() == ERROR_IO_PENDING {
            return Ok(AcceptStart::Queued);
        }
        if error.code() == ERROR_PIPE_CONNECTED {
            self.accept.state = AcceptState::Connected;
            return Ok(AcceptStart::Connected);
        }
        self.accept.state = AcceptState::Retired;
        Err(error)
    }

    /// Request cancellation once for the exact queued accept. Retain custody.
    ///
    /// # Errors
    /// Refuses an ineligible request or reports its original native error.
    fn request_accept_cancel(&mut self) -> Result<(), NativeError> {
        let operation: NativeOperation = NativeOperation::BrokerEndpointAcceptCancel;
        let AcceptState::Pending {
            cancellation_requested,
            ..
        } = &mut self.accept.state
        else {
            return Err(NativeError::new(operation, ERROR_INVALID_PARAMETER));
        };
        if *cancellation_requested {
            return Err(NativeError::new(operation, ERROR_INVALID_PARAMETER));
        }
        *cancellation_requested = true;
        #[expect(
            unsafe_code,
            reason = "target the exact original accept without freeing it"
        )]
        // SAFETY: This original pipe and nonnull record remain owned and pinned.
        // Cancellation requests no other operation and does not acknowledge release.
        let requested: BOOL =
            unsafe { CancelIoEx(self.pipe.pipe().as_raw_handle(), self.accept.pointer()) };
        if requested != 0 {
            return Ok(());
        }
        let error: NativeError = last_error(operation);
        if error.code() == ERROR_NOT_FOUND {
            return Ok(());
        }
        Err(error)
    }

    /// Observe the exact queued accept without waiting or reading its byte count.
    ///
    /// # Errors
    /// Refuses an unqueued observation and preserves custody on unknown failures.
    fn poll_accept(&mut self) -> Result<Poll<AcceptCompletion>, NativeError> {
        let operation: NativeOperation = NativeOperation::BrokerEndpointAcceptCompletion;
        let AcceptState::Pending {
            cancellation_requested,
            ..
        } = self.accept.state
        else {
            return Err(NativeError::new(operation, ERROR_INVALID_PARAMETER));
        };
        let mut ignored: u32 = 0;
        #[expect(
            unsafe_code,
            reason = "observe only the exact pending accept without waiting"
        )]
        // SAFETY: The same handle, event and pinned record from submission are
        // held. The scratch count is writable for this synchronous observation.
        let complete: BOOL = unsafe {
            GetOverlappedResult(
                self.pipe.pipe().as_raw_handle(),
                self.accept.pointer(),
                &raw mut ignored,
                0,
            )
        };
        if complete != 0 {
            self.accept.state = if cancellation_requested {
                AcceptState::Retired
            } else {
                AcceptState::Connected
            };
            return Ok(Poll::Ready(AcceptCompletion::Connected));
        }
        let error: NativeError = last_error(operation);
        match error.code() {
            ERROR_IO_INCOMPLETE => Ok(Poll::Pending),
            ERROR_OPERATION_ABORTED => {
                self.accept.state = AcceptState::Retired;
                Ok(Poll::Ready(AcceptCompletion::Cancelled))
            }
            _ => Err(error),
        }
    }

    /// Arm data transport only on the exact completed, uncancelled fixture server.
    ///
    /// # Errors
    /// A refused submission leaves no read charged.
    #[cfg(test)]
    fn begin_first_read(&mut self) -> Result<FirstReadStart, NativeError> {
        let operation: NativeOperation = NativeOperation::BrokerEndpointReadSubmit;
        let start: Result<NativeReadStart, NativeReadError> = match &mut self.pipe {
            EndpointPipe::Reservation(_) => return Ok(FirstReadStart::Unarmed),
            _ if !matches!(self.accept.state, AcceptState::Connected) => {
                return Err(NativeError::new(operation, ERROR_INVALID_PARAMETER));
            }
            EndpointPipe::FirstRead(read) => {
                self.accept.state = AcceptState::Retired;
                read.begin(MAX_FRAME_LEN)
            }
            EndpointPipe::FirstExchange(exchange) => {
                self.accept.state = AcceptState::Retired;
                exchange.write.begin_first_read(MAX_FRAME_LEN)
            }
        };
        start
            .map(|start| match start {
                NativeReadStart::Queued => FirstReadStart::Queued,
                NativeReadStart::Immediate => FirstReadStart::Immediate,
            })
            .map_err(|error| read_error(operation, error))
    }

    /// Observe the charged read, suppressing both fresh and late bytes.
    ///
    /// # Errors
    /// An unconfirmed native result retains its exact read resources.
    #[cfg(test)]
    fn poll_first_read(&mut self) -> Result<Poll<FirstReadCompletion>, NativeError> {
        self.observe_first_read(|_expected, _peer, _completed| {})
    }

    /// Only an eligible Running result can capture client identity. Retirement
    /// uses the collection-only path even when actual data won cancellation.
    ///
    /// # Errors
    /// Preserves the original native observation failure and its custody.
    #[cfg(test)]
    fn poll_running_first_read<C: Clock>(
        &mut self,
        connection: &mut RunningConnection<'_, C>,
    ) -> Result<Poll<FirstReadCompletion>, NativeError> {
        let mut staged_match: bool = false;
        let result: Result<Poll<FirstReadCompletion>, NativeError> =
            self.observe_first_read(|expected, peer, completed| {
                if (expected.is_none() && peer.is_none())
                    || !matches!(connection.edge(), ConnectionEdge::Open(_))
                {
                    return;
                }
                let attributed: Result<AttributedStatusRead, IdentifiedReadError> =
                    match (peer, expected) {
                        (Some(peer), _) => completed.attribute_peer(peer),
                        (None, Some(expected)) => completed.attribute(expected),
                        (None, None) => return,
                    };
                let matched: bool = attributed.is_ok();
                staged_match = connection.record_status_read(attributed) && matched;
            });
        if staged_match && let EndpointPipe::FirstExchange(exchange) = &mut self.pipe {
            exchange.reply = FirstReplyState::Ready;
        }
        result
    }

    /// Target the original outstanding read record. Neither outcome releases it.
    ///
    /// # Errors
    /// Preserve the native cancellation refusal while its read remains charged.
    #[cfg(test)]
    fn request_first_read_cancel(&mut self) -> Result<(), NativeError> {
        let operation: NativeOperation = NativeOperation::BrokerEndpointReadCancel;
        match &mut self.pipe {
            EndpointPipe::Reservation(_) => {
                return Err(NativeError::new(operation, ERROR_INVALID_PARAMETER));
            }
            EndpointPipe::FirstRead(read) => read.request_cancel(),
            EndpointPipe::FirstExchange(exchange) => exchange.write.request_first_read_cancel(),
        }
        .map(|_| ())
        .map_err(|error| read_error(operation, error))
    }

    /// Check the original peer while preserving all queued native ownership.
    #[cfg(test)]
    fn reply_peer_current(&mut self) -> bool {
        let EndpointPipe::FirstExchange(exchange) = &mut self.pipe else {
            return false;
        };
        exchange.peer.observe(exchange.write.pipe())
    }

    /// Spend the reply right and revalidate the peer, cutoff and positive revision at submission.
    /// # Errors
    /// Refuse a repeated submission or preserve the native submission failure.
    #[cfg(test)]
    fn begin_first_reply<C: Clock>(
        &mut self,
        bytes: &[u8],
        connection: &mut RunningConnection<'_, C>,
        cutoff: Instant,
        positive_revision: Option<ReplyRevision>,
    ) -> Result<FirstReplyStart, NativeError> {
        let EndpointPipe::FirstExchange(exchange) = &mut self.pipe else {
            return Ok(FirstReplyStart::Unarmed);
        };
        let operation: NativeOperation = NativeOperation::BrokerEndpointReplySubmit;
        if !matches!(exchange.reply, FirstReplyState::Ready) {
            return Err(NativeError::new(operation, ERROR_INVALID_PARAMETER));
        }
        exchange.reply = FirstReplyState::Retired;
        let submission: Result<NativeWriteStart, ReplySubmitError> =
            exchange
                .write
                .begin_guarded(bytes, |pipe: BorrowedHandle<'_>| {
                    match connection.reply_edge_with_revision(cutoff, positive_revision, || {
                        exchange.peer.observe(pipe)
                    }) {
                        ConnectionEdge::Open(_) => Ok(()),
                        ConnectionEdge::Closed | ConnectionEdge::Expired => {
                            Err(ReplySubmitError::Refused)
                        }
                    }
                });
        let start: NativeWriteStart = match submission {
            Ok(start) => start,
            Err(ReplySubmitError::Refused) => return Ok(FirstReplyStart::Refused),
            Err(ReplySubmitError::Write(error)) => return Err(write_error(operation, error)),
        };
        exchange.reply = FirstReplyState::Pending(bytes.len());
        Ok(match start {
            NativeWriteStart::Queued => FirstReplyStart::Queued,
            NativeWriteStart::Immediate => FirstReplyStart::Immediate,
        })
    }

    /// Collect the original reply's result on the exact accepted server.
    /// # Errors
    /// Preserve unconfirmed completion and distinguish terminal count refusal.
    #[cfg(test)]
    fn poll_first_reply(&mut self) -> Result<Poll<FirstReplyCompletion>, NativeError> {
        let operation: NativeOperation = NativeOperation::BrokerEndpointReplyCompletion;
        let EndpointPipe::FirstExchange(exchange) = &mut self.pipe else {
            return Err(NativeError::new(operation, ERROR_INVALID_PARAMETER));
        };
        let FirstReplyState::Pending(expected) = exchange.reply else {
            return Err(NativeError::new(operation, ERROR_INVALID_PARAMETER));
        };
        let completion: FirstReplyCompletion = match exchange.write.poll() {
            Ok(NativeWritePoll::Pending) => return Ok(Poll::Pending),
            Ok(NativeWritePoll::Written(written)) if written == expected => {
                FirstReplyCompletion::Written
            }
            Ok(NativeWritePoll::Cancelled | NativeWritePoll::Disconnected) => {
                FirstReplyCompletion::Abandoned
            }
            Ok(NativeWritePoll::Written(_)) | Err(NativeWriteError::Count) => {
                FirstReplyCompletion::Invalid(write_error(operation, NativeWriteError::Count))
            }
            Err(error) => return Err(write_error(operation, error)),
        };
        exchange.reply = FirstReplyState::Retired;
        if matches!(completion, FirstReplyCompletion::Written)
            && let Some(observe) = exchange.observe_full_write.take()
        {
            observe();
        }
        Ok(Poll::Ready(completion))
    }

    /// Target the one charged write record without releasing any resources.
    /// # Errors
    /// Preserve the cancellation refusal while actual completion remains owed.
    #[cfg(test)]
    fn request_first_reply_cancel(&mut self) -> Result<(), NativeError> {
        let operation: NativeOperation = NativeOperation::BrokerEndpointReplyCancel;
        let EndpointPipe::FirstExchange(exchange) = &mut self.pipe else {
            return Err(NativeError::new(operation, ERROR_INVALID_PARAMETER));
        };
        exchange
            .write
            .request_cancel()
            .map(|_| ())
            .map_err(|error| write_error(operation, error))
    }

    /// Lend only the current native submission’s event and one-shot wake state.
    fn completion_wake(&mut self) -> Option<PendingCompletionWake<'_>> {
        if let AcceptState::Pending { wake, .. } = &mut self.accept.state {
            return Some(wake.lend(self.event.as_handle()));
        }
        #[cfg(test)]
        match &mut self.pipe {
            EndpointPipe::Reservation(_) => {}
            EndpointPipe::FirstRead(read) => return read.completion_wake(),
            EndpointPipe::FirstExchange(exchange) => return exchange.write.completion_wake(),
        }
        None
    }

    /// Preserve the actual release result under the serialized host.
    fn close(self) -> Result<(), NativeError> {
        Self::close(self)
    }
}

#[cfg(test)]
impl NativeBrokerEndpoint {
    /// Collect the original read and lend its exact successful prefix once.
    ///
    /// # Errors
    /// Keep an unconfirmed native result distinct from an invalid terminal count.
    fn observe_first_read(
        &mut self,
        data: impl FnOnce(
            Option<&PipeIdentity>,
            Option<&mut RetainedFixtureClient>,
            CompletedFirstRead<'_>,
        ),
    ) -> Result<Poll<FirstReadCompletion>, NativeError> {
        let operation: NativeOperation = NativeOperation::BrokerEndpointReadCompletion;
        let expected: Option<&PipeIdentity> = self.status_client.as_deref();
        let collected: Result<Poll<()>, NativeReadError> = match &mut self.pipe {
            EndpointPipe::Reservation(_) => {
                return Err(NativeError::new(operation, ERROR_INVALID_PARAMETER));
            }
            EndpointPipe::FirstRead(read) => {
                read.collect_first_read(|completed| data(expected, None, completed))
            }
            EndpointPipe::FirstExchange(exchange) => {
                let peer: &mut RetainedFixtureClient = &mut exchange.peer;
                exchange
                    .write
                    .collect_first_read(|completed| data(expected, Some(peer), completed))
            }
        };
        match collected {
            Ok(poll) => Ok(poll.map(|()| FirstReadCompletion::Settled)),
            Err(error @ NativeReadError::Count) => Ok(Poll::Ready(FirstReadCompletion::Invalid(
                read_error(operation, error),
            ))),
            Err(error) => Err(read_error(operation, error)),
        }
    }
}

/// Preserve the native read code or mark a local representation refusal.
#[cfg(test)]
const fn read_error(operation: NativeOperation, error: NativeReadError) -> NativeError {
    let code: u32 = match error {
        NativeReadError::Event(code)
        | NativeReadError::Submit(code)
        | NativeReadError::Completion(code)
        | NativeReadError::Cancel(code) => code,
        NativeReadError::Busy
        | NativeReadError::Size
        | NativeReadError::Idle
        | NativeReadError::Count => ERROR_INVALID_PARAMETER,
    };
    NativeError::new(operation, code)
}

/// Preserve the native write code or mark a local representation refusal.
#[cfg(test)]
const fn write_error(operation: NativeOperation, error: NativeWriteError) -> NativeError {
    let code: u32 = match error {
        NativeWriteError::Event(code)
        | NativeWriteError::Submit(code)
        | NativeWriteError::Completion(code)
        | NativeWriteError::Cancel(code) => code,
        NativeWriteError::Busy
        | NativeWriteError::Size
        | NativeWriteError::Idle
        | NativeWriteError::Count => ERROR_INVALID_PARAMETER,
    };
    NativeError::new(operation, code)
}

/// Close the exact accepted server and read event after its charge has retired.
/// # Errors
/// Refuses a foreign pipe role and preserves the first explicit close failure.
#[cfg(test)]
fn close_first_read(read: NativePipeRead) -> Result<(), NativeError> {
    let (handle, event): (OwnedHandle, OwnedHandle) =
        read.into_first_read_handles().ok_or(NativeError::new(
            NativeOperation::BrokerEndpointClose,
            ERROR_INVALID_PARAMETER,
        ))?;
    let pipe_result: Result<(), NativeError> = close_handle(handle);
    let read_event_result: Result<(), NativeError> = close_handle(event);
    pipe_result.and(read_event_result)
}

/// Prepare the single pinned accept record and its original completion event.
///
/// # Errors
/// Preserve the event creation failure. No native accept is submitted here.
fn prepare_accept() -> Result<(AcceptRecord, OwnedHandle), NativeError> {
    let event: OwnedHandle = create_accept_event()?;
    let accept: AcceptRecord = AcceptRecord {
        storage: Box::pin(AcceptStorage {
            overlapped: UnsafeCell::new(OVERLAPPED {
                hEvent: event.as_raw_handle(),
                ..OVERLAPPED::default()
            }),
            _pin: PhantomPinned,
        }),
        state: AcceptState::Ready,
    };
    Ok((accept, event))
}

/// Create the independent noninherited manual-reset event for one accept.
///
/// # Errors
/// Preserve the original failure and acquire no event on refusal.
fn create_accept_event() -> Result<OwnedHandle, NativeError> {
    #[expect(unsafe_code, reason = "create an unnamed private completion event")]
    // SAFETY: Null attributes and name select noninheritance and an unnamed
    // event. The other arguments select manual reset and initially unsignaled.
    let handle: HANDLE = unsafe { CreateEventW(ptr::null(), 1, 0, ptr::null()) };
    if handle.is_null() {
        return Err(last_error(NativeOperation::BrokerEndpointAcceptEvent));
    }
    #[expect(unsafe_code, reason = "adopt the successful event creation handle")]
    // SAFETY: Successful creation returns a unique handle released by CloseHandle.
    let event: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(handle) };
    Ok(event)
}

/// Attempt one explicit native release after there is no possible pending use.
///
/// # Errors
/// Preserve the exact error. A failed attempt cannot establish release or retry.
fn close_handle(owner: OwnedHandle) -> Result<(), NativeError> {
    let handle: HANDLE = owner.into_raw_handle();
    #[expect(
        unsafe_code,
        reason = "observe explicit release of each original handle"
    )]
    // SAFETY: The private caller already retired all native work involving this
    // real handle. Consuming the sole owner leaves exactly one close attempt.
    let closed: BOOL = unsafe { CloseHandle(handle) };
    if closed == 0 {
        return Err(last_error(NativeOperation::BrokerEndpointClose));
    }
    Ok(())
}

/// Capture the immediately preceding Win32 failure before intervening calls.
fn last_error(operation: NativeOperation) -> NativeError {
    #[expect(
        unsafe_code,
        reason = "retain the native failure on the calling thread"
    )]
    // SAFETY: GetLastError has no pointer or lifetime preconditions.
    let code: u32 = unsafe { GetLastError() };
    NativeError::new(operation, code)
}

#[cfg(test)]
mod tests {
    use std::{
        error::Error,
        os::windows::io::{FromRawHandle, OwnedHandle},
        ptr,
        sync::{Mutex, MutexGuard},
        task::Poll,
        time::Duration,
    };

    use windows_sys::Win32::{
        Foundation::{ERROR_ACCESS_DENIED, GetLastError, HANDLE, INVALID_HANDLE_VALUE},
        Storage::FileSystem::{
            CreateFileW, FILE_READ_DATA, FILE_WRITE_DATA, OPEN_EXISTING, SECURITY_IDENTIFICATION,
            SECURITY_SQOS_PRESENT, SYNCHRONIZE,
        },
    };

    use super::NativeBrokerEndpoint;
    use crate::{
        NativeError, NativeOperation,
        broker_endpoint::{AcceptCompletion, AcceptStart, Reservation},
        native_broker_wait::{BrokerControlWake, BrokerWait, WakeSource},
    };

    /// Serialize the only tests using the same process's fixed reservation name.
    static RESERVATION_TEST: Mutex<()> = Mutex::new(());

    /// Observe denial on the actual reserved name, then explicitly retire and reacquire it.
    ///
    /// # Errors
    /// Propagate failed acquisition or close, and refuse unexpected client access.
    /// # Panics
    /// Panics on any error code other than the expected access or collision refusal.
    #[test]
    fn reservation_denies_data_and_releases_the_fixed_name() -> Result<(), Box<dyn Error>> {
        let _name: MutexGuard<'_, ()> = RESERVATION_TEST
            .lock()
            .map_err(|_| "reservation test lock poisoned")?;
        let endpoint: NativeBrokerEndpoint = NativeBrokerEndpoint::acquire()?;
        let name: Vec<u16> = r"\\.\pipe\NevroTick-Lab-S02-Broker-Reservation"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        for access in [FILE_READ_DATA, FILE_WRITE_DATA] {
            #[expect(unsafe_code, reason = "probe only data access on the owned lab object")]
            // SAFETY: The independent local name is NUL-terminated and valid
            // throughout the call. No handle inheritance or data I/O is requested.
            let opened: HANDLE = unsafe {
                CreateFileW(
                    name.as_ptr(),
                    access | SYNCHRONIZE,
                    0,
                    ptr::null(),
                    OPEN_EXISTING,
                    SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION,
                    ptr::null_mut(),
                )
            };
            if opened != INVALID_HANDLE_VALUE {
                if !opened.is_null() {
                    #[expect(unsafe_code, reason = "release any unexpectedly granted probe handle")]
                    // SAFETY: The successful call returned a unique real handle.
                    let unexpected: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(opened) };
                    drop(unexpected);
                }
                return Err("reservation allowed a client data open".into());
            }
            #[expect(unsafe_code, reason = "observe the direct client refusal")]
            // SAFETY: GetLastError takes no arguments and follows the failed call.
            let refused: u32 = unsafe { GetLastError() };
            assert_eq!(refused, ERROR_ACCESS_DENIED);
        }

        assert_eq!(
            NativeBrokerEndpoint::acquire().err(),
            Some(NativeError::new(
                NativeOperation::BrokerEndpointCreate,
                ERROR_ACCESS_DENIED,
            )),
        );

        endpoint.close()?;
        let successor: NativeBrokerEndpoint = NativeBrokerEndpoint::acquire()?;
        successor.close()?;
        Ok(())
    }

    /// Wake the same broker wait for two sequential accepts, releasing each.
    ///
    /// # Errors
    /// Propagates native failures. If completion is still unconfirmed, the
    /// fixture's invalid-drop safeguard terminates the isolated test process.
    /// # Panics
    /// Fails on unexpected readiness, a wrong wake source or a wrong outcome.
    #[test]
    fn queued_accept_cancels_to_completion_before_release() -> Result<(), Box<dyn Error>> {
        let _name: MutexGuard<'_, ()> = RESERVATION_TEST
            .lock()
            .map_err(|_| "reservation test lock poisoned")?;
        let control: BrokerControlWake = BrokerControlWake::new()?;
        let mut wait: BrokerWait = BrokerWait::new();
        for _ in 0..2 {
            let mut endpoint: NativeBrokerEndpoint = NativeBrokerEndpoint::acquire()?;
            assert!(endpoint.completion_wake().is_none());
            assert!(matches!(endpoint.begin_accept()?, AcceptStart::Queued));
            assert!(matches!(endpoint.poll_accept()?, Poll::Pending));
            let before_cancel: WakeSource =
                wait.observe(&control, Duration::ZERO, endpoint.completion_wake())?;
            endpoint.request_accept_cancel()?;

            let observed: Result<WakeSource, NativeError> =
                wait.observe(&control, Duration::from_secs(2), endpoint.completion_wake());
            let repeated: Result<WakeSource, NativeError> =
                wait.observe(&control, Duration::ZERO, endpoint.completion_wake());
            let completion: Poll<AcceptCompletion> = endpoint.poll_accept()?;
            assert!(
                completion.is_ready(),
                "accept still pending after broker wait"
            );
            assert!(endpoint.completion_wake().is_none());
            endpoint.close()?;
            assert_eq!(before_cancel, WakeSource::Timeout);
            assert_eq!(observed?, WakeSource::Completion);
            assert_eq!(repeated?, WakeSource::Timeout);
            assert!(matches!(
                completion,
                Poll::Ready(AcceptCompletion::Cancelled)
            ));
        }
        Ok(())
    }
}
