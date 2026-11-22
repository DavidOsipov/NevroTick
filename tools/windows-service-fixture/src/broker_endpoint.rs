// SPDX-License-Identifier: GPL-2.0-only
//! One accept and its physical retirement under the broker's service lifecycle.

use std::{mem, task::Poll};

use crate::{
    NativeError,
    error::WorkerError,
    host::{Worker, WorkerCompletion, WorkerFailure, WorkerFinish, WorkerWake},
};

/// Whether the original accept submission requires a separate observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "retain the original endpoint while its accept is queued"]
pub(super) enum AcceptStart {
    /// The exact native acceptance record remains charged.
    Queued,
    /// An unexpected connection completed during submission.
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

/// An owned endpoint with one preallocated accept and explicit close.
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

    /// Consume this handle in one release attempt.
    ///
    /// # Errors
    /// Preserve the native failure. An error leaves release unconfirmed.
    fn close(self) -> Result<(), NativeError>;
}

/// Legal observations under the acknowledged Running permit.
enum RunningAccept {
    /// The prepared endpoint has not submitted its one accept.
    Unsubmitted,
    /// A native operation is still charged.
    Queued,
    /// A Running observation failed with native completion unconfirmed.
    FencedQueued,
    /// Submission was refused or a terminal result was observed.
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
    /// An acknowledged service lifetime with tracked accept custody.
    Running(E, RunningAccept),
    /// Admission is closed. No accept is outstanding on this endpoint.
    Closing(Option<E>),
    /// Admission is closed and exactly one queued accept needs retirement.
    Draining(E, Option<WorkerError>),
    /// A consumed close did not confirm physical release.
    CloseUnconfirmed(WorkerError),
    /// No endpoint was acquired or the original close completed.
    Finished(WorkerCompletion),
}

/// The fixed broker owns its reservation and accept under one work permit.
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

    /// Attempt release only with no outstanding accept. Preserve the first
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
    fn activate(&mut self) -> Result<(), WorkerError> {
        let previous: State<A, E> = mem::replace(&mut self.state, State::Refused);
        let State::Prepared(endpoint) = previous else {
            self.state = previous;
            return Err(WorkerError::Activation);
        };
        self.state = State::Running(endpoint, RunningAccept::Unsubmitted);
        Ok(())
    }

    /// Submit once and observe the queued accept during admitted host polls.
    /// No connection is dispatched from the laboratory reservation.
    /// # Errors
    /// Refuses a poll outside Running or following retirement. A submission
    /// failure or unexpected terminal result fences service operation. A
    /// failed observation leaves the pending accept charged through Stop.
    fn poll_running(&mut self, _wake: &WorkerWake) -> Result<(), WorkerFailure> {
        let State::Running(endpoint, accept) = &mut self.state else {
            return Err(WorkerFailure::Operation(WorkerError::Running));
        };
        match accept {
            RunningAccept::Unsubmitted => {
                *accept = RunningAccept::Retired;
                match endpoint.begin_accept() {
                    Ok(AcceptStart::Queued) => {
                        *accept = RunningAccept::Queued;
                        Ok(())
                    }
                    Ok(AcceptStart::Connected) => Err(WorkerFailure::Operation(
                        WorkerError::EndpointUnexpectedCompletion,
                    )),
                    Err(error) => Err(WorkerFailure::Operation(WorkerError::EndpointAccept(error))),
                }
            }
            RunningAccept::Queued => match endpoint.poll_accept() {
                Ok(Poll::Pending) => Ok(()),
                Ok(Poll::Ready(AcceptCompletion::Connected | AcceptCompletion::Cancelled)) => {
                    *accept = RunningAccept::Retired;
                    Err(WorkerFailure::Operation(
                        WorkerError::EndpointUnexpectedCompletion,
                    ))
                }
                Err(error) => {
                    *accept = RunningAccept::FencedQueued;
                    Err(WorkerFailure::Operation(WorkerError::EndpointAccept(error)))
                }
            },
            RunningAccept::FencedQueued | RunningAccept::Retired => {
                Err(WorkerFailure::Operation(WorkerError::Running))
            }
        }
    }

    /// Fence preparation and activation, cancel an outstanding accept once,
    /// then signal the owning thread.
    fn request_cancel(&mut self, wake: &WorkerWake) {
        let previous: State<A, E> = mem::replace(&mut self.state, State::Refused);
        self.state = match previous {
            State::Dormant(_) | State::Refused => State::Closing(None),
            State::Prepared(endpoint)
            | State::Running(endpoint, RunningAccept::Unsubmitted | RunningAccept::Retired) => {
                State::Closing(Some(endpoint))
            }
            State::Running(mut endpoint, RunningAccept::Queued | RunningAccept::FencedQueued) => {
                let cleanup: Option<WorkerError> = endpoint
                    .request_accept_cancel()
                    .err()
                    .map(WorkerError::EndpointAcceptCleanup);
                State::Draining(endpoint, cleanup)
            }
            retained @ (State::Closing(_)
            | State::Draining(..)
            | State::CloseUnconfirmed(_)
            | State::Finished(_)) => retained,
        };
        wake.notify();
    }

    /// Discard the original accept's terminal result, then release the
    /// endpoint. Pending work and ambiguous observations retain custody.
    fn poll_finished(&mut self, _wake: &WorkerWake) -> WorkerFinish {
        let previous: State<A, E> = mem::replace(&mut self.state, State::Refused);
        self.state = match previous {
            State::Closing(endpoint) => Self::release(endpoint, None),
            State::Draining(mut endpoint, cleanup) => match endpoint.poll_accept() {
                Ok(Poll::Pending) => State::Draining(endpoint, cleanup),
                Ok(Poll::Ready(AcceptCompletion::Connected | AcceptCompletion::Cancelled)) => {
                    Self::release(Some(endpoint), cleanup)
                }
                Err(error) => State::Draining(
                    endpoint,
                    cleanup.or(Some(WorkerError::EndpointAcceptCleanup(error))),
                ),
            },
            retained @ (State::Dormant(_)
            | State::Refused
            | State::Prepared(_)
            | State::Running(..)
            | State::CloseUnconfirmed(_)
            | State::Finished(_)) => retained,
        };
        match self.state {
            State::Finished(completion) => WorkerFinish::Finished(completion),
            State::Draining(_, cleanup_failure) => {
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
}
