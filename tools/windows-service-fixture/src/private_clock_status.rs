// SPDX-License-Identifier: GPL-2.0-only
//! First private status exchange through the serialized laboratory broker.
//! The composed in-memory endpoints model connection custody. They establish
//! no native identity, IPC transport privileges or authority to change time.

use std::{
    cell::Cell,
    rc::Rc,
    time::{Duration, Instant},
};

use super::{
    BrokerAvailability, BrokerQueryFault, Clock, InertWorker, InvocationPoll, PrivateStatusIssue,
    ServiceInvocation, StatusReporter, StopNotification,
};
use crate::{
    LabRole, NativeError,
    private_clock_wire::{
        AssemblyProgress, Frame, FrameAssembly, FrameError, RequestId, decode_reply,
        decode_request, encode_reply, encode_request,
    },
};

/// Finite lifetime of the selected in-memory fixture connection.
const SESSION_SPAN: Duration = Duration::from_secs(5);
/// All inbound frames, including rejected ones, consume this allowance.
const FRAME_BUDGET: u8 = 4;

/// Local connection refusal without raw wire, endpoint or clock details.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SessionFault {
    /// No live connection exists at this admission point.
    Closed,
    /// The fixture principal is outside the private Engine role.
    WrongRole,
    /// A packet arrived with another connection's transport identity.
    ForeignConnection,
    /// The fixed session cutoff elapsed.
    Expired,
    /// The valid request ID repeats or precedes one already consumed.
    Replayed,
    /// The sender has no unused representable next ID.
    IdExhausted,
    /// Bounded wire admission or reply correlation refused.
    Frame(FrameError),
}

/// Exact identity and monotonic open state shared by the two fixture endpoints.
type Connection = Rc<Cell<bool>>;

/// One borrowed inbound frame along with its out-of-band connection identity.
#[derive(Clone, Copy)]
struct Delivery<'a> {
    /// The transport-supplied connection, never parsed from JSON.
    connection: &'a Connection,
    /// Complete bounded frame candidate checked at the receiving endpoint.
    bytes: &'a [u8],
}

/// A reply published by the actual receiver with its original transport identity.
struct Reply {
    /// Connection retained independently of the encoded request ID.
    connection: Connection,
    /// The immutable bounded response.
    frame: Frame,
}

/// Fixed Engine sender for one in-memory private connection.
struct EngineStatusClient {
    /// The connection supplied by the fixture composer.
    connection: Connection,
    /// Positive ordered ID to issue once, or permanent numeric exhaustion.
    next: Option<u64>,
}

impl EngineStatusClient {
    /// Complete the ordinary exchange without releasing its intermediate reply.
    /// The result describes the broker's decision and carries no freshness lease.
    /// # Errors
    /// Refuses failed delivery, broker admission or reply pairing.
    fn read_availability<R: StatusReporter<Error = NativeError>, C: Clock>(
        &mut self,
        receiver: &mut BrokerStatusSession<'_, R, C>,
    ) -> Result<BrokerAvailability, SessionFault> {
        let pending: PendingStatus<'_> = self.request()?;
        let reply: Reply = receiver.receive(pending.delivery())?;
        Ok(if pending.resolve(reply)? {
            BrokerAvailability::Available
        } else {
            BrokerAvailability::Unavailable
        })
    }

    /// Encode one actual request and retain exclusive custody until its reply resolves.
    /// # Errors
    /// Refuses closed custody or exhausted/invalid local request identity.
    fn request(&mut self) -> Result<PendingStatus<'_>, SessionFault> {
        if !self.connection.get() {
            return Err(SessionFault::Closed);
        }
        let next: u64 = self.next.ok_or(SessionFault::IdExhausted)?;
        let id: RequestId = RequestId::new(next).map_err(SessionFault::Frame)?;
        self.next = next.checked_add(1);
        Ok(PendingStatus {
            client: self,
            id,
            frame: encode_request(id),
        })
    }
}

/// One outstanding request. Resolving or discarding it never refunds its ID.
struct PendingStatus<'a> {
    /// Exclusive sender borrow forbids a second simultaneous pending request.
    client: &'a mut EngineStatusClient,
    /// The expected same-connection correlation ID.
    id: RequestId,
    /// Actual bytes emitted by the Engine sender.
    frame: Frame,
}

impl PendingStatus<'_> {
    /// Carry this request's bytes over the same local connection.
    fn delivery(&self) -> Delivery<'_> {
        Delivery {
            connection: &self.client.connection,
            bytes: self.frame.as_bytes(),
        }
    }

    /// Consume the result once and check actual transport identity and the enclosed ID.
    /// # Errors
    /// Refuses foreign, closed, malformed or miscorrelated replies.
    fn resolve(self, reply: Reply) -> Result<bool, SessionFault> {
        let Reply { connection, frame } = reply;
        if !self.client.connection.get() {
            return Err(SessionFault::Closed);
        }
        if !Rc::ptr_eq(&self.client.connection, &connection) {
            return Err(SessionFault::ForeignConnection);
        }
        decode_reply(frame.as_bytes(), self.id).map_err(SessionFault::Frame)
    }
}

/// Receiver owns one exclusive borrow of the broker, original clock and Stop source.
struct BrokerStatusSession<'a, R: StatusReporter<Error = NativeError>, C: Clock> {
    /// Independent identity of this in-memory transport.
    connection: Connection,
    /// Real acknowledged and serialized broker owner.
    owner: &'a mut ServiceInvocation<R, InertWorker>,
    /// The same monotonic observation provider the invocation uses.
    clock: &'a mut C,
    /// The original fixture notification bound at the composition boundary.
    stop: &'a StopNotification,
    /// Absolute initial cutoff from the owner's admitted time.
    cutoff: Instant,
    /// Monotonically consumed valid request IDs.
    last: Option<RequestId>,
    /// Charged for every delivered frame before its bytes are interpreted.
    remaining: u8,
}

impl<'a, R: StatusReporter<Error = NativeError>, C: Clock> BrokerStatusSession<'a, R, C> {
    /// Compose one Engine sender and one real broker receiver under this exact owner.
    /// The in-memory fixture role is selected by trusted test composition.
    /// # Errors
    /// Refuses wrong role, reused ownership, unavailable owner or unrepresentable cutoff.
    fn open(
        owner: &'a mut ServiceInvocation<R, InertWorker>,
        clock: &'a mut C,
        stop: &'a StopNotification,
        peer: LabRole,
    ) -> Result<(Self, EngineStatusClient), SessionFault> {
        if peer != LabRole::Engine {
            return Err(SessionFault::WrongRole);
        }
        if owner.private_status_issue == PrivateStatusIssue::Issued {
            return Err(SessionFault::Closed);
        }
        let now: Instant = owner
            .broker_status_edge(clock, stop, None)
            .map_err(Self::owner_fault)?;
        let cutoff: Instant = now.checked_add(SESSION_SPAN).ok_or(SessionFault::Expired)?;
        let connection: Connection = Rc::new(Cell::new(true));
        let client: EngineStatusClient = EngineStatusClient {
            connection: Rc::clone(&connection),
            next: Some(1),
        };
        owner.private_status_issue = PrivateStatusIssue::Issued;
        Ok((
            Self {
                connection,
                owner,
                clock,
                stop,
                cutoff,
                last: None,
                remaining: FRAME_BUDGET,
            },
            client,
        ))
    }

    /// Deliver one complete record through the same owner as fragmented transfers.
    /// # Errors
    /// Refuses wrong owner, malformed input, replay, closed admission or expired cutoff.
    fn receive(&mut self, delivery: Delivery<'_>) -> Result<Reply, SessionFault> {
        let mut inbound: InboundStatus<'_, 'a, R, C> = self.begin_receive(delivery.connection)?;
        inbound
            .complete_fragment(delivery.bytes)?
            .ok_or(SessionFault::Frame(FrameError::Length))
    }

    /// Reserve one receive under the original transport identity.
    /// The receiver stays exclusively borrowed until this frame resolves or is abandoned.
    /// # Errors
    /// Refuses a closed, exhausted or foreign connection.
    fn begin_receive(
        &mut self,
        connection: &Connection,
    ) -> Result<InboundStatus<'_, 'a, R, C>, SessionFault> {
        if !self.connection.get() {
            return Err(SessionFault::Closed);
        }
        let Some(remaining) = self.remaining.checked_sub(1) else {
            self.close();
            return Err(SessionFault::Closed);
        };
        self.remaining = remaining;
        if !Rc::ptr_eq(&self.connection, connection) {
            return Err(SessionFault::ForeignConnection);
        }
        Ok(InboundStatus {
            receiver: self,
            assembly: Some(FrameAssembly::request()),
        })
    }

    /// Check the original time and control owners before taking more completed bytes.
    /// # Errors
    /// Retires the connection and returns the original service or session refusal.
    fn check_receive_edge(&mut self) -> Result<(), SessionFault> {
        if let Err(fault) = self
            .owner
            .broker_status_edge(self.clock, self.stop, Some(self.cutoff))
        {
            self.close();
            return Err(Self::owner_fault(fault));
        }
        Ok(())
    }

    /// Check the completed frame and take the final guarded availability decision.
    /// # Errors
    /// Refuses invalid schema, replay or loss of the original currentness guard.
    fn dispatch(&mut self, frame: &Frame) -> Result<Reply, SessionFault> {
        let id: RequestId = decode_request(frame.as_bytes()).map_err(SessionFault::Frame)?;
        if self.last.is_some_and(|previous: RequestId| id <= previous) {
            return Err(SessionFault::Replayed);
        }
        self.last = Some(id);
        let availability: BrokerAvailability =
            match self
                .owner
                .broker_availability_before(self.clock, self.stop, Some(self.cutoff))
            {
                Ok(availability) => availability,
                Err(fault) => {
                    self.close();
                    return Err(Self::owner_fault(fault));
                }
            };
        Ok(Reply {
            connection: Rc::clone(&self.connection),
            frame: encode_reply(id, availability == BrokerAvailability::Available),
        })
    }

    /// Keep connection expiry distinct from service lifecycle closure.
    fn owner_fault(fault: BrokerQueryFault) -> SessionFault {
        match fault {
            BrokerQueryFault::ServiceClosed => SessionFault::Closed,
            BrokerQueryFault::SessionExpired => SessionFault::Expired,
        }
    }

    /// Retire delivery and any unconsumed reply without touching worker custody.
    fn close(&self) {
        self.connection.set(false);
    }
}

/// One admitted receive. The trusted fixture transport supplies only bytes
/// already delivered on the connection admitted by the session.
struct InboundStatus<'session, 'owner, R: StatusReporter<Error = NativeError>, C: Clock> {
    /// The exact receiving session remains exclusively reserved during assembly.
    receiver: &'session mut BrokerStatusSession<'owner, R, C>,
    /// Incomplete frame storage. Its absence marks a resolved receive.
    assembly: Option<FrameAssembly>,
}

impl<R: StatusReporter<Error = NativeError>, C: Clock> InboundStatus<'_, '_, R, C> {
    /// Drive one service step while this receive retains exclusive custody.
    /// A live connection can shorten a Running wait. Closure retires just the
    /// local input and preserves the service's own progress or cleanup result.
    fn poll_broker(&mut self) -> InvocationPoll {
        let receiver: &mut BrokerStatusSession<'_, R, C> = self.receiver;
        let poll: InvocationPoll = receiver.owner.poll(receiver.clock, receiver.stop);
        if receiver.connection.get()
            && let InvocationPoll::Pending(wait) = poll
            && let Some(remaining) = receiver
                .owner
                .broker_status_wait(receiver.stop, receiver.cutoff)
        {
            return InvocationPoll::Pending(wait.min(remaining));
        }
        receiver.close();
        self.assembly = None;
        poll
    }

    /// Record one nonempty, contiguous completion from the admitted fixture connection.
    /// A partial frame provides no reply and keeps the original session cutoff.
    /// # Errors
    /// Refuses a resolved receive, invalid framing, changed control, or failed dispatch.
    /// Empty or ambiguous framing retires the connection. A schema refusal after
    /// an exact envelope leaves the next boundary known.
    fn complete_fragment(&mut self, bytes: &[u8]) -> Result<Option<Reply>, SessionFault> {
        let assembly: FrameAssembly = self.assembly.take().ok_or(SessionFault::Closed)?;
        self.receiver.check_receive_edge()?;
        match assembly.push(bytes) {
            Ok(AssemblyProgress::Pending(assembly)) => {
                self.assembly = Some(assembly);
                Ok(None)
            }
            Ok(AssemblyProgress::Complete(frame)) => self.receiver.dispatch(&frame).map(Some),
            Err(error) => {
                self.receiver.close();
                Err(SessionFault::Frame(error))
            }
        }
    }
}

impl<R: StatusReporter<Error = NativeError>, C: Clock> Drop for InboundStatus<'_, '_, R, C> {
    /// Retire undecided boundaries rather than restarting a partial request.
    fn drop(&mut self) {
        if self.assembly.is_some() {
            self.receiver.close();
        }
    }
}

impl<R: StatusReporter<Error = NativeError>, C: Clock> Drop for BrokerStatusSession<'_, R, C> {
    /// Loss of the local receiver also retires the shared transport binding.
    fn drop(&mut self) {
        self.close();
    }
}

#[path = "private_clock_status_tests.rs"]
mod tests;
