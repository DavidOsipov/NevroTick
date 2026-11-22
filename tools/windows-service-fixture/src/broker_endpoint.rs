// SPDX-License-Identifier: GPL-2.0-only
//! Endpoint ownership under the broker's single service lifecycle.

use std::{mem, task::Poll};

use crate::{
    NativeError,
    error::WorkerError,
    host::{Worker, WorkerCompletion, WorkerFailure, WorkerFinish, WorkerWake},
};

/// An owned endpoint whose explicit close establishes physical release.
pub(super) trait Reservation: Sized {
    /// Consume this handle in one release attempt.
    ///
    /// # Errors
    /// Preserve the native failure. An error leaves release unconfirmed.
    fn close(self) -> Result<(), NativeError>;
}

/// One ownership path from dormant preparation through physical release.
enum State<A, E> {
    /// The only factory, which does no work before the preparation permit.
    Dormant(A),
    /// Preparation failed before acquiring an endpoint.
    Refused,
    /// The endpoint is held before the host acknowledges Running.
    Prepared(E),
    /// The endpoint remains held for the acknowledged service lifetime.
    Running(E),
    /// Admission is closed. A previously acquired endpoint still belongs here.
    Stopping(Option<E>),
    /// A consumed close did not confirm physical release.
    CloseUnconfirmed(WorkerError),
    /// No endpoint was acquired or the original close completed.
    Finished,
}

/// The fixed broker owns the reservation under its existing work permit.
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
        self.state = State::Running(endpoint);
        Ok(())
    }

    /// The reservation has no accept, frame, or independent work to dispatch.
    /// # Errors
    /// A host poll outside Running reports a lifecycle refusal.
    fn poll_running(&mut self, _wake: &WorkerWake) -> Result<(), WorkerFailure> {
        if matches!(self.state, State::Running(_)) {
            Ok(())
        } else {
            Err(WorkerFailure::Operation(WorkerError::Running))
        }
    }

    /// Fence preparation and activation before signalling the owning thread.
    fn request_cancel(&mut self, wake: &WorkerWake) {
        let previous: State<A, E> = mem::replace(&mut self.state, State::Refused);
        self.state = match previous {
            State::Dormant(_) | State::Refused => State::Stopping(None),
            State::Prepared(endpoint) | State::Running(endpoint) => State::Stopping(Some(endpoint)),
            retained => retained,
        };
        wake.notify();
    }

    /// Complete only after the original endpoint is observably released.
    fn poll_finished(&mut self, _wake: &WorkerWake) -> WorkerFinish {
        if let State::Stopping(endpoint) = &mut self.state {
            self.state = match endpoint.take().map_or(Ok(()), Reservation::close) {
                Ok(()) => State::Finished,
                Err(error) => State::CloseUnconfirmed(WorkerError::EndpointClose(error)),
            };
        }
        match self.state {
            State::Finished => WorkerFinish::Finished(WorkerCompletion {
                operation_failure: None,
                cleanup_failure: None,
            }),
            State::CloseUnconfirmed(error) => WorkerFinish::NativeDrainPending {
                cleanup_failure: Some(error),
            },
            _ => WorkerFinish::Pending,
        }
    }
}
