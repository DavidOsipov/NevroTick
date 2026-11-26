// SPDX-License-Identifier: GPL-2.0-only
//! Broker accept and fixture read custody through physical retirement.

use std::{mem, task::Poll, time::Instant};

#[cfg(test)]
use crate::broker_clock_change::ReplyRevision;
#[cfg(windows)]
use crate::native_broker_wait::PendingCompletionWake;

use crate::{
    NativeError, NativeOperation,
    error::WorkerError,
    host::{
        Clock, ConnectionEdge, ConnectionWait, RunningConnection, Worker, WorkerCompletion,
        WorkerFailure, WorkerFinish, WorkerWake,
    },
};

/// Whether the original accept submission requires a separate observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "retain the original endpoint while its accept is queued"]
pub(super) enum AcceptStart {
    /// The exact native acceptance record remains charged.
    Queued,
    /// A connection completed during submission.
    Connected,
}

/// A terminal disposition for the exact queued accept.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "distinguish a connection from acknowledged cancellation"]
pub(super) enum AcceptCompletion {
    /// A connection completed. The laboratory reservation admits no dispatch.
    Connected,
    /// Native cancellation completed.
    Cancelled,
}

/// Submission of the single preallocated fixture read.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "retain even an inline read until its result is collected"]
pub(super) enum FirstReadStart {
    /// This reservation admits no reads.
    Unarmed,
    /// Admission closed before a read was submitted.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "only the sealed test reservation implements a first read"
        )
    )]
    Refused,
    /// The original read record awaits native completion.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "only the sealed test reservation implements a first read"
        )
    )]
    Queued,
    /// The call completed inline. The original record still requires collection.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "only the sealed test reservation implements a first read"
        )
    )]
    Immediate,
}

/// An authoritative terminal result, carrying no application bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "distinguish settled custody from an invalid terminal output"]
pub(super) enum FirstReadCompletion {
    /// The native read settled and its bytes, if any, were discarded.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "only the sealed test reservation implements a first read"
        )
    )]
    Settled,
    /// The slot settled but its native result was invalid.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "only the sealed test reservation implements a first read"
        )
    )]
    Invalid(NativeError),
}

/// Native submission of the fixture's one bounded reply.
#[cfg(test)]
pub(super) enum FirstReplyStart {
    /// This endpoint reads only.
    Unarmed,
    /// Final admission closed after preparation without submitting a write.
    Refused,
    /// Windows still owns the physical operation.
    Queued,
    /// Windows already completed the write. The count still needs collection.
    Immediate,
}

/// Terminal result of the original reply write.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum FirstReplyCompletion {
    /// Win32 reports the complete input extent.
    Written,
    /// Windows confirms cancellation or disconnection.
    Abandoned,
    /// The operation settled without a valid complete count.
    Invalid(NativeError),
}

/// Invalid operation on an unarmed reservation. This is Win32 `ERROR_INVALID_PARAMETER`.
const UNARMED_READ: u32 = 87;

/// An owned endpoint with one accept, optional fixture read and explicit close.
pub(super) trait Reservation: Sized {
    /// Submit the one accept under the acknowledged Running permit.
    ///
    /// # Errors
    /// A submission refusal owns no queued accept.
    fn begin_accept(&mut self) -> Result<AcceptStart, NativeError>;

    /// Request cancellation of the previously queued accept, retaining it.
    ///
    /// # Errors
    /// Preserve the first native cancellation refusal. Neither request
    /// success nor refusal supplies a terminal observation.
    fn request_accept_cancel(&mut self) -> Result<(), NativeError>;

    /// Poll the exact queued accept without waiting.
    ///
    /// # Errors
    /// An observation failure leaves completion unconfirmed.
    fn poll_accept(&mut self) -> Result<Poll<AcceptCompletion>, NativeError>;

    /// Start one read only on the same, uncancelled fixture accept.
    /// Recheck the original connection after preparation, at submission.
    ///
    /// # Errors
    /// A failed submission queues no new read.
    fn begin_first_read<C: Clock>(
        &mut self,
        _connection: &mut RunningConnection<'_, C>,
    ) -> Result<FirstReadStart, NativeError> {
        Ok(FirstReadStart::Unarmed)
    }

    /// Poll the exact read. An error leaves physical completion unconfirmed.
    ///
    /// # Errors
    /// Preserve a native observation failure without releasing the read.
    fn poll_first_read(&mut self) -> Result<Poll<FirstReadCompletion>, NativeError> {
        Err(NativeError::new(
            NativeOperation::BrokerEndpointReadCompletion,
            UNARMED_READ,
        ))
    }

    /// Observe a first read that may contribute to the admitted fixture receiver.
    ///
    /// # Errors
    /// The original read remains charged if native completion is unconfirmed.
    #[cfg(all(windows, test))]
    fn poll_running_first_read<C: Clock>(
        &mut self,
        _connection: &mut RunningConnection<'_, C>,
    ) -> Result<Poll<FirstReadCompletion>, NativeError> {
        self.poll_first_read()
    }

    /// Request cancellation of the exact read without releasing its resources.
    ///
    /// # Errors
    /// Preserve a cancellation refusal. Completion is still owed.
    fn request_first_read_cancel(&mut self) -> Result<(), NativeError> {
        Err(NativeError::new(
            NativeOperation::BrokerEndpointReadCancel,
            UNARMED_READ,
        ))
    }

    /// Reobserve the original peer for this read's one reply without submitting I/O.
    /// A refusal revokes reply admission for this exchange and cannot replace its peer.
    #[cfg(test)]
    fn reply_peer_current(&mut self) -> bool {
        false
    }

    /// Submit the only bounded fixture reply on the original accepted pipe.
    ///
    /// # Errors
    /// Refuses native submission without acquiring new work.
    #[cfg(test)]
    fn begin_first_reply<C: Clock>(
        &mut self,
        _bytes: &[u8],
        _connection: &mut RunningConnection<'_, C>,
        _cutoff: Instant,
        _positive_revision: Option<ReplyRevision>,
    ) -> Result<FirstReplyStart, NativeError> {
        Ok(FirstReplyStart::Unarmed)
    }

    /// Observe the exact write. An error leaves physical completion unconfirmed.
    ///
    /// # Errors
    /// Refuses an unarmed slot or retains a native observation failure.
    #[cfg(test)]
    fn poll_first_reply(&mut self) -> Result<Poll<FirstReplyCompletion>, NativeError> {
        Err(NativeError::new(
            NativeOperation::BrokerEndpointReplyCompletion,
            UNARMED_READ,
        ))
    }

    /// Request cancellation once without releasing write custody.
    ///
    /// # Errors
    /// Preserves an unarmed slot or a native cancellation failure.
    #[cfg(test)]
    fn request_first_reply_cancel(&mut self) -> Result<(), NativeError> {
        Err(NativeError::new(
            NativeOperation::BrokerEndpointReplyCancel,
            UNARMED_READ,
        ))
    }

    /// Borrow only the pending operation's event and one-shot wake state.
    #[cfg(windows)]
    fn completion_wake(&mut self) -> Option<PendingCompletionWake<'_>> {
        None
    }

    /// Consume this handle in one release attempt.
    ///
    /// # Errors
    /// Preserve the native failure. An error leaves release unconfirmed.
    fn close(self) -> Result<(), NativeError>;
}

/// The one native operation that may still require completion.
#[derive(Clone, Copy)]
enum Operation {
    /// The first connection has yet to settle.
    Accept,
    /// The armed fixture has queued its first read.
    QueuedRead,
    /// `ReadFile` reported inline success. The result still requires collection.
    InlineRead,
    /// The exact queued reply retains the original connection cutoff and decision.
    #[cfg(test)]
    QueuedReply(ReplyContext),
    /// The inline reply count still requires collection.
    #[cfg(test)]
    InlineReply(ReplyContext),
}

/// The original context required to keep a write queued or confirm its completion.
#[cfg(test)]
#[derive(Clone, Copy)]
struct ReplyContext {
    /// The unchanged cutoff of this connection.
    cutoff: Instant,
    /// The exact awake revision behind a positive decision.
    positive_revision: Option<ReplyRevision>,
}

/// Poll outcome after a native operation's resources have settled.
enum Completed {
    /// The exact native operation settled and forwarded no application bytes.
    Settled,
    /// Physical completion occurred, but its output was invalid.
    Invalid(NativeError),
}

impl Operation {
    /// Observe only the operation that is still charged.
    ///
    /// # Errors
    /// Preserve an observation failure with physical completion unconfirmed.
    fn poll(self, endpoint: &mut impl Reservation) -> Result<Poll<Completed>, NativeError> {
        match self {
            Self::Accept => endpoint
                .poll_accept()
                .map(|poll| poll.map(|_| Completed::Settled)),
            Self::QueuedRead | Self::InlineRead => endpoint.poll_first_read().map(|poll| {
                poll.map(|complete| match complete {
                    FirstReadCompletion::Settled => Completed::Settled,
                    FirstReadCompletion::Invalid(error) => Completed::Invalid(error),
                })
            }),
            #[cfg(test)]
            Self::QueuedReply(_) | Self::InlineReply(_) => {
                endpoint.poll_first_reply().map(|poll| {
                    poll.map(|complete| match complete {
                        FirstReplyCompletion::Written | FirstReplyCompletion::Abandoned => {
                            Completed::Settled
                        }
                        FirstReplyCompletion::Invalid(error) => Completed::Invalid(error),
                    })
                })
            }
        }
    }

    /// Target cancellation at the selected operation without releasing it.
    ///
    /// # Errors
    /// Preserve the first native refusal and its retirement phase.
    fn cancel(self, endpoint: &mut impl Reservation) -> Result<(), WorkerError> {
        match self {
            Self::Accept => endpoint.request_accept_cancel(),
            Self::QueuedRead => endpoint.request_first_read_cancel(),
            // A reported inline completion requires collection, not another effect.
            Self::InlineRead => Ok(()),
            #[cfg(test)]
            Self::QueuedReply(_) => endpoint.request_first_reply_cancel(),
            #[cfg(test)]
            Self::InlineReply(_) => Ok(()),
        }
        .map_err(|error| self.cleanup_error(error))
    }

    /// Keep the operation's typed failure for service reporting.
    const fn running_error(self, error: NativeError) -> WorkerError {
        match self {
            Self::Accept => WorkerError::EndpointAccept(error),
            Self::QueuedRead | Self::InlineRead => WorkerError::EndpointRead(error),
            #[cfg(test)]
            Self::QueuedReply(_) | Self::InlineReply(_) => WorkerError::EndpointReply(error),
        }
    }

    /// Retain the original native retirement failure.
    const fn cleanup_error(self, error: NativeError) -> WorkerError {
        match self {
            Self::Accept => WorkerError::EndpointAcceptCleanup(error),
            Self::QueuedRead | Self::InlineRead => WorkerError::EndpointReadCleanup(error),
            #[cfg(test)]
            Self::QueuedReply(_) | Self::InlineReply(_) => WorkerError::EndpointReplyCleanup(error),
        }
    }
}

/// Legal observations under the acknowledged Running permit.
enum Running {
    /// The prepared endpoint has not submitted its one accept.
    Unsubmitted,
    /// A native operation is still charged.
    Queued(Operation),
    /// A Running observation failed with native completion unconfirmed.
    FencedQueued(Operation),
    /// Local expiry fenced admission and requested cancellation once.
    Retiring(Operation, Option<WorkerError>),
    /// Its first read settled. Only this poll's guarded reply may use it.
    #[cfg(test)]
    ReadSettled,
    /// The connection has settled. Keep the pipe until service Stop.
    Parked,
    /// A terminal failure has fenced further work.
    Retired,
}

/// One ownership path from dormant preparation through physical release.
enum State<A, E> {
    /// The only factory, which does no work before the preparation permit.
    Dormant(A),
    /// Preparation failed before acquiring an endpoint.
    Refused,
    /// The endpoint is held before the host acknowledges Running.
    Prepared(E),
    /// An acknowledged service lifetime with tracked native I/O custody.
    Running(E, Running),
    /// Admission is closed. No operation is outstanding on this endpoint.
    Closing(Option<E>),
    /// Admission is closed and exactly one queued operation needs retirement.
    Draining(E, Operation, Option<WorkerError>),
    /// A consumed close did not confirm physical release.
    CloseUnconfirmed(WorkerError),
    /// No endpoint was acquired or the original close completed.
    Finished(WorkerCompletion),
}

/// The broker owns its reservation and only active operation under one work permit.
pub(super) struct BrokerEndpointWorker<A, E> {
    /// All native acquisition and release state, confined to the host thread.
    state: State<A, E>,
}

impl<A, E> BrokerEndpointWorker<A, E>
where
    A: FnOnce() -> Result<E, NativeError>,
    E: Reservation,
{
    /// Retain a one-shot factory without acquiring the endpoint.
    pub(super) const fn new(acquire: A) -> Self {
        Self {
            state: State::Dormant(acquire),
        }
    }

    /// Attempt release only with no outstanding operation. Preserve the first
    /// cleanup refusal even if the terminal release also fails.
    fn release(endpoint: Option<E>, cleanup_failure: Option<WorkerError>) -> State<A, E> {
        match endpoint.map_or(Ok(()), Reservation::close) {
            Ok(()) => State::Finished(WorkerCompletion {
                operation_failure: None,
                cleanup_failure,
            }),
            Err(error) => State::CloseUnconfirmed(
                cleanup_failure.unwrap_or(WorkerError::EndpointClose(error)),
            ),
        }
    }

    /// Cancel one locally refused operation and keep its exact resources until settled.
    ///
    /// # Errors
    /// Keep the first cancellation refusal while native completion is still owed.
    fn retire(
        endpoint: &mut E,
        state: &mut Running,
        operation: Operation,
    ) -> Result<ConnectionWait, WorkerFailure> {
        let cleanup: Option<WorkerError> = operation.cancel(endpoint).err();
        *state = Running::Retiring(operation, cleanup);
        cleanup.map_or(Ok(ConnectionWait::Drain), |error| {
            Err(WorkerFailure::Cleanup(error))
        })
    }

    /// Wait within the original cutoff or start local retirement once.
    ///
    /// # Errors
    /// Preserve a cancellation refusal while the original operation is still owned.
    fn pending(
        endpoint: &mut E,
        state: &mut Running,
        operation: Operation,
        edge: &ConnectionEdge,
    ) -> Result<ConnectionWait, WorkerFailure> {
        match edge {
            ConnectionEdge::Open(cutoff) => Ok(ConnectionWait::Cutoff(*cutoff)),
            ConnectionEdge::Expired => Self::retire(endpoint, state, operation),
            ConnectionEdge::Closed => Ok(ConnectionWait::Service),
        }
    }

    /// Hand the original accepted reservation to its optional first read at a fresh edge.
    ///
    /// # Errors
    /// Refuse a live connection on the fixed unarmed endpoint or a failed submission.
    fn accepted<C: Clock>(
        endpoint: &mut E,
        state: &mut Running,
        connection: &mut RunningConnection<'_, C>,
        edge: &ConnectionEdge,
    ) -> Result<ConnectionWait, WorkerFailure> {
        if !matches!(edge, ConnectionEdge::Open(_)) {
            *state = Running::Parked;
            return Ok(ConnectionWait::Service);
        }
        let start: Result<FirstReadStart, NativeError> = endpoint.begin_first_read(connection);
        let after: ConnectionEdge = connection.edge();
        match start {
            Ok(FirstReadStart::Unarmed) if matches!(after, ConnectionEdge::Open(_)) => {
                Self::unexpected(state)
            }
            Ok(FirstReadStart::Unarmed | FirstReadStart::Refused) => {
                *state = Running::Parked;
                Ok(ConnectionWait::Service)
            }
            Ok(FirstReadStart::Queued) => {
                let operation: Operation = Operation::QueuedRead;
                *state = Running::Queued(operation);
                Self::pending(endpoint, state, operation, &after)
            }
            Ok(FirstReadStart::Immediate) => {
                let operation: Operation = Operation::InlineRead;
                *state = Running::Queued(operation);
                Self::poll_read(endpoint, state, connection, operation, &after)
            }
            Err(error) => {
                *state = Running::Retired;
                Err(WorkerFailure::Operation(WorkerError::EndpointRead(error)))
            }
        }
    }

    /// Fence an unexpected eligible terminal accept on the reservation.
    ///
    /// # Errors
    /// Always reports the unexpected native completion to the Running owner.
    fn unexpected(state: &mut Running) -> Result<ConnectionWait, WorkerFailure> {
        *state = Running::Retired;
        Err(WorkerFailure::Operation(
            WorkerError::EndpointUnexpectedCompletion,
        ))
    }

    /// Collect the one read under the original cutoff. The armed fixture can
    /// stage a sealed status candidate for the serialized host.
    ///
    /// # Errors
    /// Fence a failed observation, retaining native custody if still outstanding.
    fn poll_read<C: Clock>(
        endpoint: &mut E,
        state: &mut Running,
        connection: &mut RunningConnection<'_, C>,
        operation: Operation,
        edge: &ConnectionEdge,
    ) -> Result<ConnectionWait, WorkerFailure> {
        // Collect a reported inline completion even if admission closed while
        // submitting it. This observation never forwards bytes or starts I/O.
        if !matches!(operation, Operation::InlineRead) && !matches!(edge, ConnectionEdge::Open(_)) {
            return Self::pending(endpoint, state, operation, edge);
        }
        #[cfg(all(windows, test))]
        let result: Result<Poll<FirstReadCompletion>, NativeError> =
            endpoint.poll_running_first_read(connection);
        #[cfg(not(all(windows, test)))]
        let result: Result<Poll<FirstReadCompletion>, NativeError> = endpoint.poll_first_read();
        let after: ConnectionEdge = connection.edge();
        match result {
            Ok(Poll::Pending) => Self::pending(endpoint, state, operation, &after),
            Ok(Poll::Ready(FirstReadCompletion::Settled)) => {
                #[cfg(test)]
                {
                    *state = Running::ReadSettled;
                }
                #[cfg(not(test))]
                {
                    *state = Running::Parked;
                }
                Ok(ConnectionWait::Service)
            }
            Ok(Poll::Ready(FirstReadCompletion::Invalid(error))) => {
                *state = Running::Retired;
                Err(WorkerFailure::Operation(operation.running_error(error)))
            }
            Err(error) => {
                *state = Running::FencedQueued(operation);
                Err(WorkerFailure::Operation(operation.running_error(error)))
            }
        }
    }

    /// Recheck the host and peer admission, then the original positive revision.
    /// A refusal retires this connection locally while Stop retains service precedence.
    #[cfg(test)]
    fn reply_edge<C: Clock>(
        endpoint: &mut E,
        connection: &mut RunningConnection<'_, C>,
        reply: ReplyContext,
    ) -> ConnectionEdge {
        connection.reply_edge_with_revision(reply.cutoff, reply.positive_revision, || {
            endpoint.reply_peer_current()
        })
    }

    /// Start the serialized fixture reply on the worker whose original read settled.
    ///
    /// # Errors
    /// Keeps native refusal distinct from a queued operation requiring retirement.
    #[cfg(test)]
    pub(super) fn begin_reply<C: Clock>(
        &mut self,
        connection: &mut RunningConnection<'_, C>,
        bytes: &[u8],
        cutoff: Instant,
        positive_revision: Option<ReplyRevision>,
    ) -> Result<ConnectionWait, WorkerFailure> {
        let State::Running(endpoint, state @ Running::ReadSettled) = &mut self.state else {
            return Ok(ConnectionWait::Service);
        };
        *state = Running::Parked;
        if !matches!(
            connection.reply_edge_with_peer(cutoff, || endpoint.reply_peer_current()),
            ConnectionEdge::Open(_)
        ) {
            return Ok(ConnectionWait::Service);
        }
        let start: Result<FirstReplyStart, NativeError> =
            endpoint.begin_first_reply(bytes, connection, cutoff, positive_revision);
        let reply: ReplyContext = ReplyContext {
            cutoff,
            positive_revision,
        };
        match start {
            Ok(FirstReplyStart::Refused) => Ok(ConnectionWait::Service),
            Ok(FirstReplyStart::Unarmed) => {
                let _edge: ConnectionEdge = connection.reply_edge(cutoff);
                Ok(ConnectionWait::Service)
            }
            Ok(FirstReplyStart::Queued) => {
                let operation: Operation = Operation::QueuedReply(reply);
                *state = Running::Queued(operation);
                let after: ConnectionEdge = Self::reply_edge(endpoint, connection, reply);
                Self::pending(endpoint, state, operation, &after)
            }
            Ok(FirstReplyStart::Immediate) => {
                let operation: Operation = Operation::InlineReply(reply);
                *state = Running::Queued(operation);
                Self::poll_reply(endpoint, state, connection, operation, reply)
            }
            Err(error) => {
                let _edge: ConnectionEdge = connection.reply_edge(cutoff);
                *state = Running::Retired;
                Err(WorkerFailure::Operation(WorkerError::EndpointReply(error)))
            }
        }
    }

    /// Collect the actual write count and confirm it only while the original context is live.
    ///
    /// # Errors
    /// A terminal count refusal fences activity. An unconfirmed observation
    /// still owns the native operation and must be drained.
    #[cfg(test)]
    fn poll_reply<C: Clock>(
        endpoint: &mut E,
        state: &mut Running,
        connection: &mut RunningConnection<'_, C>,
        operation: Operation,
        reply: ReplyContext,
    ) -> Result<ConnectionWait, WorkerFailure> {
        let cutoff: Instant = reply.cutoff;
        // An inline completion is already terminal. Always collect its original slot.
        if matches!(operation, Operation::QueuedReply(_)) {
            let edge: ConnectionEdge = Self::reply_edge(endpoint, connection, reply);
            if !matches!(edge, ConnectionEdge::Open(_)) {
                return Self::pending(endpoint, state, operation, &edge);
            }
        }
        let result: Result<Poll<FirstReplyCompletion>, NativeError> = endpoint.poll_first_reply();
        let after: ConnectionEdge = match result {
            Ok(Poll::Ready(FirstReplyCompletion::Written) | Poll::Pending) => {
                Self::reply_edge(endpoint, connection, reply)
            }
            _ => connection.reply_edge(cutoff),
        };
        match result {
            Ok(Poll::Pending) => Self::pending(endpoint, state, operation, &after),
            Ok(Poll::Ready(
                complete @ (FirstReplyCompletion::Written | FirstReplyCompletion::Abandoned),
            )) => {
                *state = Running::Parked;
                if matches!(complete, FirstReplyCompletion::Written)
                    && matches!(after, ConnectionEdge::Open(_))
                {
                    connection.record_reply_written();
                }
                Ok(ConnectionWait::Service)
            }
            Ok(Poll::Ready(FirstReplyCompletion::Invalid(error))) => {
                *state = Running::Retired;
                Err(WorkerFailure::Operation(operation.running_error(error)))
            }
            Err(error) => {
                *state = Running::FencedQueued(operation);
                Err(WorkerFailure::Operation(operation.running_error(error)))
            }
        }
    }

    /// Collect a queued accept and start the read only after the fresh observation.
    ///
    /// # Errors
    /// A refused observation preserves the charge, and an unexpected result fences activity.
    fn poll_accept<C: Clock>(
        endpoint: &mut E,
        state: &mut Running,
        connection: &mut RunningConnection<'_, C>,
    ) -> Result<ConnectionWait, WorkerFailure> {
        let operation: Operation = Operation::Accept;
        let edge: ConnectionEdge = connection.edge();
        if !matches!(edge, ConnectionEdge::Open(_)) {
            return Self::pending(endpoint, state, operation, &edge);
        }
        let result: Result<Poll<AcceptCompletion>, NativeError> = endpoint.poll_accept();
        let after: ConnectionEdge = connection.edge();
        match result {
            Ok(Poll::Pending) => Self::pending(endpoint, state, operation, &after),
            Ok(Poll::Ready(AcceptCompletion::Connected)) => {
                Self::accepted(endpoint, state, connection, &after)
            }
            Ok(Poll::Ready(AcceptCompletion::Cancelled))
                if matches!(after, ConnectionEdge::Open(_)) =>
            {
                Self::unexpected(state)
            }
            Ok(Poll::Ready(AcceptCompletion::Cancelled)) => {
                *state = Running::Parked;
                Ok(ConnectionWait::Service)
            }
            Err(error) => {
                *state = Running::FencedQueued(operation);
                Err(WorkerFailure::Operation(operation.running_error(error)))
            }
        }
    }
}

impl<A, E> Worker for BrokerEndpointWorker<A, E>
where
    A: FnOnce() -> Result<E, NativeError>,
    E: Reservation,
{
    /// Acquire once on the preparation permit. Readiness retains the handle.
    fn poll_prepare(&mut self, _wake: &WorkerWake) -> Poll<Result<(), WorkerError>> {
        let previous: State<A, E> = mem::replace(&mut self.state, State::Refused);
        let State::Dormant(acquire) = previous else {
            self.state = previous;
            return Poll::Ready(Err(WorkerError::Preparation));
        };
        match acquire() {
            Ok(endpoint) => {
                self.state = State::Prepared(endpoint);
                Poll::Ready(Ok(()))
            }
            Err(error) => Poll::Ready(Err(WorkerError::EndpointPreparation(error))),
        }
    }

    /// Cross to Running only once and only with the prepared original handle.
    /// # Errors
    /// Refuses activation without a prepared endpoint.
    fn activate(&mut self, _cutoff: Instant) -> Result<(), WorkerError> {
        let previous: State<A, E> = mem::replace(&mut self.state, State::Refused);
        let State::Prepared(endpoint) = previous else {
            self.state = previous;
            return Err(WorkerError::Activation);
        };
        self.state = State::Running(endpoint, Running::Unsubmitted);
        Ok(())
    }

    /// Advance the exact accept or armed first read under the host's fresh clock and Stop checks.
    ///
    /// # Errors
    /// A refused operation fences Running. Unconfirmed work stays charged through Stop.
    fn poll_running_with_connection<C: Clock>(
        &mut self,
        _wake: &WorkerWake,
        connection: &mut RunningConnection<'_, C>,
    ) -> Result<ConnectionWait, WorkerFailure> {
        let State::Running(endpoint, state) = &mut self.state else {
            return Err(WorkerFailure::Operation(WorkerError::Running));
        };
        match state {
            Running::Unsubmitted => {
                if !matches!(connection.edge(), ConnectionEdge::Open(_)) {
                    return Ok(ConnectionWait::Service);
                }
                let operation: Operation = Operation::Accept;
                let start: Result<AcceptStart, NativeError> = endpoint.begin_accept();
                let after: ConnectionEdge = connection.edge();
                match start {
                    Ok(AcceptStart::Queued) => {
                        *state = Running::Queued(operation);
                        Self::pending(endpoint, state, operation, &after)
                    }
                    Ok(AcceptStart::Connected) => {
                        Self::accepted(endpoint, state, connection, &after)
                    }
                    Err(error) => {
                        *state = Running::Retired;
                        Err(WorkerFailure::Operation(operation.running_error(error)))
                    }
                }
            }
            Running::Queued(operation) => {
                let operation: Operation = *operation;
                match operation {
                    Operation::Accept => Self::poll_accept(endpoint, state, connection),
                    Operation::QueuedRead | Operation::InlineRead => {
                        let edge: ConnectionEdge = connection.edge();
                        Self::poll_read(endpoint, state, connection, operation, &edge)
                    }
                    #[cfg(test)]
                    Operation::QueuedReply(reply) | Operation::InlineReply(reply) => {
                        Self::poll_reply(endpoint, state, connection, operation, reply)
                    }
                }
            }
            Running::Retiring(operation, cleanup) => match operation.poll(endpoint) {
                Ok(Poll::Pending) => Ok(ConnectionWait::Drain),
                Ok(Poll::Ready(Completed::Invalid(error))) => {
                    let error: WorkerError = cleanup.unwrap_or(operation.cleanup_error(error));
                    *state = Running::Parked;
                    Err(WorkerFailure::Cleanup(error))
                }
                Ok(Poll::Ready(Completed::Settled)) => {
                    *state = Running::Parked;
                    Ok(ConnectionWait::Service)
                }
                Err(error) => {
                    let error: WorkerError = *cleanup.get_or_insert(operation.cleanup_error(error));
                    Err(WorkerFailure::Cleanup(error))
                }
            },
            #[cfg(test)]
            Running::ReadSettled => {
                *state = Running::Parked;
                Ok(ConnectionWait::Service)
            }
            Running::Parked => Ok(ConnectionWait::Service),
            Running::FencedQueued(_) | Running::Retired => {
                Err(WorkerFailure::Operation(WorkerError::Running))
            }
        }
    }

    /// Respond only to the guarded result passed back by this Running host call.
    #[cfg(all(windows, test))]
    fn submit_status_reply<C: Clock>(
        &mut self,
        connection: &mut RunningConnection<'_, C>,
        reply: &crate::host::NativeStatusReply,
    ) -> Result<ConnectionWait, WorkerFailure> {
        self.begin_reply(
            connection,
            reply.frame().as_bytes(),
            reply.cutoff(),
            reply.positive_revision(),
        )
    }

    /// Fence preparation and activation, cancel an outstanding operation once,
    /// then signal the owning thread.
    fn request_cancel(&mut self, wake: &WorkerWake) {
        let previous: State<A, E> = mem::replace(&mut self.state, State::Refused);
        self.state = match previous {
            State::Dormant(_) | State::Refused => State::Closing(None),
            #[cfg(test)]
            State::Running(endpoint, Running::ReadSettled) => State::Closing(Some(endpoint)),
            State::Prepared(endpoint)
            | State::Running(endpoint, Running::Unsubmitted | Running::Parked | Running::Retired) => {
                State::Closing(Some(endpoint))
            }
            State::Running(
                mut endpoint,
                Running::Queued(operation) | Running::FencedQueued(operation),
            ) => {
                let cleanup: Option<WorkerError> = operation.cancel(&mut endpoint).err();
                State::Draining(endpoint, operation, cleanup)
            }
            State::Running(endpoint, Running::Retiring(operation, cleanup)) => {
                State::Draining(endpoint, operation, cleanup)
            }
            retained @ (State::Closing(_)
            | State::Draining(..)
            | State::CloseUnconfirmed(_)
            | State::Finished(_)) => retained,
        };
        wake.notify();
    }

    /// Discard the original operation's terminal result, then release the
    /// endpoint. Pending work and ambiguous observations retain custody.
    fn poll_finished(&mut self, _wake: &WorkerWake) -> WorkerFinish {
        let previous: State<A, E> = mem::replace(&mut self.state, State::Refused);
        self.state = match previous {
            State::Closing(endpoint) => Self::release(endpoint, None),
            State::Draining(mut endpoint, operation, cleanup) => {
                match operation.poll(&mut endpoint) {
                    Ok(Poll::Pending) => State::Draining(endpoint, operation, cleanup),
                    Ok(Poll::Ready(Completed::Invalid(error))) => Self::release(
                        Some(endpoint),
                        cleanup.or(Some(operation.cleanup_error(error))),
                    ),
                    Ok(Poll::Ready(Completed::Settled)) => Self::release(Some(endpoint), cleanup),
                    Err(error) => State::Draining(
                        endpoint,
                        operation,
                        cleanup.or(Some(operation.cleanup_error(error))),
                    ),
                }
            }
            retained @ (State::Dormant(_)
            | State::Refused
            | State::Prepared(_)
            | State::Running(..)
            | State::CloseUnconfirmed(_)
            | State::Finished(_)) => retained,
        };
        match self.state {
            State::Finished(completion) => WorkerFinish::Finished(completion),
            State::Draining(_, _, cleanup_failure) => {
                WorkerFinish::NativeDrainPending { cleanup_failure }
            }
            State::CloseUnconfirmed(error) => WorkerFinish::NativeDrainPending {
                cleanup_failure: Some(error),
            },
            State::Dormant(_)
            | State::Refused
            | State::Prepared(_)
            | State::Running(..)
            | State::Closing(_) => WorkerFinish::Pending,
        }
    }

    /// Expose a borrowed hint only while the exact operation is charged.
    #[cfg(windows)]
    fn completion_wake(&mut self) -> Option<PendingCompletionWake<'_>> {
        match &mut self.state {
            State::Running(
                endpoint,
                Running::Queued(_) | Running::FencedQueued(_) | Running::Retiring(..),
            )
            | State::Draining(endpoint, ..) => endpoint.completion_wake(),
            _ => None,
        }
    }
}
