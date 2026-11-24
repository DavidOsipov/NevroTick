// SPDX-License-Identifier: GPL-2.0-only
//! Controls through the actual broker owner and the private fixture exchange.

use std::{
    cell::Cell,
    error::Error,
    num::NonZeroUsize,
    sync::atomic::{AtomicU32, Ordering::SeqCst},
    time::{Duration, Instant},
};

#[cfg(windows)]
use std::io::Write;
#[cfg(windows)]
use windows_sys::Win32::Foundation::ERROR_CANT_OPEN_ANONYMOUS;

use windows_platform::service::{ServiceStatus, StatusReporter};

use super::{
    BrokerAvailability, BrokerStatusSession, Clock, Connection, Delivery, EngineStatusClient,
    InboundStatus, InertWorker, LabRole, NativeError, OutboundStatus, PendingStatus, Reply,
    ServiceInvocation, SessionFault, StopNotification, Worker,
};
use crate::{
    BrokerClockRead, NativeOperation,
    broker_clock_change::{
        BrokerClockChanges, ControlDisposition, NativeBrokerReader, deliver, report_status,
    },
    host::{
        HostReceipt, InvocationPoll, StatusFactory, start_broker_worker,
        tests::HealthyTokenObserver,
    },
    private_clock_wire::{
        AssemblyProgress, Frame, FrameAssembly, FrameError, MAX_FRAME_LEN, encode_reply,
    },
};

#[cfg(windows)]
use crate::{
    broker_clock_change::ClockRevision,
    native_broker_wait::{BrokerControlWake, BrokerWait, WakeSource},
    native_pipe_identity::{
        PipeIdentity, PipeIdentityError, ensure_unimpersonated, own_process_identity,
    },
    native_pipe_pair::{
        LabClientContext, LabDuplexPipe, LabIdentificationPipe, LabOverlappedWriter, LabPipeError,
        LabPipeReader, LabPipeWriter, open_lab_duplex_pipe, open_lab_identification_pipe,
        open_lab_pipe, open_lab_reply_pipe,
    },
    native_pipe_read::{
        IdentifiedReadError, IdentifiedRequestRead, NativeCancel, NativePipeRead, NativeReadBytes,
        NativeReadError, NativeReadPoll, NativeReadStart, RetiredRead,
    },
    native_pipe_worker::{
        ActiveRequestRead, NativePipeWorker, NativePipeWorkerError, PreparedDuplex, ReadRetirement,
        RetiringRequestRead,
    },
    native_pipe_write::{NativeWritePoll, NativeWriteStart},
};

/// Stable slots include the native pipe histories and the split power-cycle case.
const BROKER_CASES: usize = 43;
/// Each scenario owns one fresh registered mailbox and one independent counter.
static MAILBOXES: [BrokerClockChanges; BROKER_CASES] =
    [const { BrokerClockChanges::new() }; BROKER_CASES];
/// Calls of the original raw getter, separated from trusted deadline observations.
static GETTERS: [AtomicU32; BROKER_CASES] = [const { AtomicU32::new(0) }; BROKER_CASES];

/// Observe the same paired Windows clock sample used by the fixed broker.
/// # Errors
/// Preserve the actual Windows adapter's raw failure.
/// # Panics
/// Panics if the selected fixture has no assigned counter.
#[cfg(windows)]
fn sample<const N: usize>() -> Result<crate::BrokerClockSample, crate::BrokerClockSampleError> {
    let _previous: u32 = GETTERS[N].fetch_add(1, SeqCst);
    crate::BrokerClockSample::read()
}

/// Observe one known value through the portable broker reader.
/// # Errors
/// Refuses a call past the selected scenario's observed read allowance.
/// # Panics
/// Panics if the selected fixture has no assigned counter.
#[cfg(not(windows))]
fn sample<const N: usize, const READS: u32>() -> Result<u64, u32> {
    let previous: u32 = GETTERS[N].fetch_add(1, SeqCst);
    if previous < READS { Ok(1) } else { Err(7) }
}

/// Successful status acknowledgement through the actual registered mailbox.
#[derive(Clone, Copy)]
struct Reporter(&'static BrokerClockChanges);

impl StatusFactory for Reporter {
    type Reporter = Self;

    /// Keep the same registered mailbox with the fixed broker role.
    /// # Errors
    /// Reject an unexpected role selection.
    fn register(&mut self, role: LabRole) -> Result<Self::Reporter, NativeError> {
        if role == LabRole::ClockBroker {
            Ok(*self)
        } else {
            Err(NativeError::new(NativeOperation::Register, 5))
        }
    }
}

impl StatusReporter for Reporter {
    type Error = NativeError;

    /// Acknowledge the selected portable status report and its mailbox effect.
    /// # Errors
    /// The fixture's status acknowledgement always succeeds.
    fn report(&mut self, status: ServiceStatus) -> Result<(), Self::Error> {
        report_status(Some(self.0), status.state(), |_mask: u32| Ok(()))
    }
}

/// A deterministic change scheduled at the final monotonic observation.
#[derive(Clone, Copy)]
enum DecisionAction {
    /// Advance to the exact initial session cutoff.
    Advance(Instant),
    /// Deliver an actual registered control at this observation.
    Deliver(u32),
}

/// Independent time script consumed by the same owner for both observations.
struct ScriptClock<'a> {
    /// Current monotonic time.
    now: Instant,
    /// Number of quiet observations before the selected transition.
    final_action: Option<(u8, DecisionAction)>,
    /// Positive evidence that the selected final edge was reached.
    reached: bool,
    /// Original control ownership.
    mailbox: &'static BrokerClockChanges,
    /// Original sticky notification source.
    stop: &'a StopNotification,
}

impl Clock for ScriptClock<'_> {
    /// Supply the current time and deliver a scheduled transition exactly once.
    fn now(&mut self) -> Instant {
        match self.final_action.take() {
            Some((0, DecisionAction::Advance(time))) => {
                self.now = time;
                self.reached = true;
            }
            Some((0, DecisionAction::Deliver(control))) => {
                self.reached = deliver(Some(self.mailbox), self.stop, control, 0).wake;
            }
            Some((quiet, action)) => self.final_action = Some((quiet - 1, action)),
            None => {}
        }
        self.now
    }
}

/// Drive the selected broker through startup, one real sampling poll and Stop.
/// The exercise returns observations before any assertion on its outcome.
/// # Errors
/// Returns an unexpected startup or incomplete terminal observation.
/// # Panics
/// Fails if the inert fixture returned a different poll or terminal result.
fn with_broker<const N: usize, T>(
    exercise: impl FnOnce(
        &mut ServiceInvocation<Reporter, InertWorker>,
        &mut ScriptClock<'_>,
        &StopNotification,
    ) -> T,
) -> Result<T, Box<dyn Error>> {
    with_broker_reads::<N, 1, T>(exercise)
}

/// Keep the same scripted broker owner with a scenario-specific read count.
/// The ordinary one-read controls retain their original allowance.
/// # Errors
/// Returns an unexpected startup or incomplete terminal observation.
/// # Panics
/// Fails if polling, cleanup or the actual getter count differs.
fn with_broker_reads<const N: usize, const READS: u32, T>(
    exercise: impl FnOnce(
        &mut ServiceInvocation<Reporter, InertWorker>,
        &mut ScriptClock<'_>,
        &StopNotification,
    ) -> T,
) -> Result<T, Box<dyn Error>> {
    with_broker_worker_reads::<N, READS, T, InertWorker>(InertWorker::new(), exercise)
}

/// Use the real broker invocation with its selected dormant worker.
/// # Errors
/// Returns an unexpected startup or incomplete terminal observation.
/// # Panics
/// Fails if startup, terminal status or actual getter count differs.
fn with_broker_worker_reads<const N: usize, const READS: u32, T, P: Worker>(
    worker: P,
    exercise: impl FnOnce(
        &mut ServiceInvocation<Reporter, P>,
        &mut ScriptClock<'_>,
        &StopNotification,
    ) -> T,
) -> Result<T, Box<dyn Error>> {
    let (result, receipt): (T, HostReceipt) = run_broker_case::<N, READS, T, P>(worker, exercise)?;
    assert!(matches!(
        receipt.broker_clock_read(),
        Some(BrokerClockRead::Completed { revision, result: Ok(_), .. })
            if revision.get() == 1
    ));
    assert_eq!(GETTERS[N].load(SeqCst), READS);
    Ok(result)
}

/// Return the exercise result and terminal receipt after retiring the owner.
/// # Errors
/// Returns an unexpected startup or incomplete terminal observation.
/// # Panics
/// Fails if the original owner loses its startup or terminal result.
fn run_broker_case<const N: usize, const READS: u32, T, P: Worker>(
    worker: P,
    exercise: impl FnOnce(
        &mut ServiceInvocation<Reporter, P>,
        &mut ScriptClock<'_>,
        &StopNotification,
    ) -> T,
) -> Result<(T, HostReceipt), Box<dyn Error>> {
    let mailbox: &'static BrokerClockChanges = &MAILBOXES[N];
    let stop: StopNotification = StopNotification::new();
    let mut clock: ScriptClock<'_> = ScriptClock {
        now: Instant::now(),
        final_action: None,
        reached: false,
        mailbox,
        stop: &stop,
    };
    #[cfg(windows)]
    let reader: NativeBrokerReader = NativeBrokerReader::new(mailbox, sample::<N>);
    #[cfg(not(windows))]
    let reader: NativeBrokerReader = NativeBrokerReader::new(mailbox, sample::<N, READS>);
    let mut owner: ServiceInvocation<Reporter, P> = start_broker_worker(
        0,
        &mut Reporter(mailbox),
        &mut HealthyTokenObserver(LabRole::ClockBroker),
        &mut clock,
        &stop,
        reader.with_sampling(|_, _| None),
        worker,
    )
    .map_err(|_receipt: HostReceipt| "unexpected broker startup failure")?;
    let first: InvocationPoll = owner.poll(&mut clock, &stop);
    let result: T = exercise(&mut owner, &mut clock, &stop);
    // A refused transition may leave the script unused. Never trigger it during cleanup.
    clock.final_action = None;
    let _stop_disposition = deliver(Some(mailbox), &stop, 1, 0);
    let started: Instant = Instant::now();
    let last: InvocationPoll = loop {
        let poll: InvocationPoll = owner.poll(&mut clock, &stop);
        if let InvocationPoll::NativeDrain(wait) = poll {
            if started.elapsed() >= Duration::from_secs(2) {
                #[cfg(windows)]
                let _diagnostic: std::io::Result<()> = writeln!(
                    std::io::stderr().lock(),
                    "broker cleanup could not observe native completion"
                );
                return Err("broker cleanup could not observe native completion".into());
            }
            std::thread::sleep(wait);
        } else {
            break poll;
        }
    };
    let receipt: HostReceipt = owner.take_receipt().ok_or("broker did not finish")?;
    assert_eq!(first, InvocationPoll::Pending(Duration::from_secs(1)));
    assert_eq!(last, InvocationPoll::Finished);
    assert_eq!(receipt.result(), &Ok(()));
    Ok((result, receipt))
}

/// Valid shape cannot substitute a role or connection, and all four actual
/// arrivals spend their allowance regardless of the decode or replay result.
/// # Errors
/// Returns an unexpected broker startup, cleanup or exchange failure.
/// # Panics
/// Fails if the receiver launders an inadmissible request or renews its charge.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn private_status_requires_its_role_and_charges_every_frame() -> Result<(), Box<dyn Error>> {
    let actual: Result<_, SessionFault> = with_broker::<0, _>(|owner, clock, stop| {
        let wrong_role: Option<SessionFault> =
            BrokerStatusSession::open(owner, clock, stop, LabRole::ClockBroker).err();
        let (mut session, mut client): (BrokerStatusSession<'_, Reporter, ScriptClock<'_>>, _) =
            BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
        let pending = client.request()?;
        let foreign_connection: Connection = Connection::new(Cell::new(true));
        let foreign: Option<SessionFault> = session
            .receive(Delivery {
                connection: &foreign_connection,
                bytes: pending.delivery().bytes,
            })
            .err();
        // Correct v1 request envelope for ID 1 with an inadmissible array body.
        let invalid: [u8; 18] = [1, 0, 1, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, b'[', b']'];
        let malformed: Option<SessionFault> = session
            .receive(Delivery {
                connection: pending.delivery().connection,
                bytes: &invalid,
            })
            .err();
        let reply: Reply = session.receive(pending.delivery())?;
        let repeated: Option<SessionFault> = session.receive(pending.delivery()).err();
        let available: bool = pending.resolve(reply)?;
        let following = client.request()?;
        let exhausted: Option<SessionFault> = session.receive(following.delivery()).err();
        let closed: Option<SessionFault> = session.receive(following.delivery()).err();
        Ok((
            wrong_role, foreign, malformed, repeated, available, exhausted, closed,
        ))
    })?;
    assert_eq!(
        actual,
        Ok((
            Some(SessionFault::WrongRole),
            Some(SessionFault::ForeignConnection),
            Some(SessionFault::Frame(FrameError::Body)),
            Some(SessionFault::Replayed),
            true,
            Some(SessionFault::Closed),
            Some(SessionFault::Closed),
        )),
    );
    Ok(())
}

/// Split each request through the real receiver. A prefix produces no reply
/// and several completions consume exactly one of the four request attempts.
/// # Errors
/// Returns fixture startup, cleanup or transfer failures.
/// # Panics
/// Fails on premature publication, stale completion reuse or renewed allowance.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn split_requests_publish_once_and_charge_one_attempt() -> Result<(), Box<dyn Error>> {
    let actual: Result<_, SessionFault> = with_broker::<7, _>(|owner, clock, stop| {
        let (mut session, mut client) =
            BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
        let mut accepted: [bool; 4] = [false; 4];
        for slot in &mut accepted {
            let pending = client.request()?;
            let delivery: Delivery<'_> = pending.delivery();
            let (first, rest): (&[u8], &[u8]) = delivery.bytes.split_at(7);
            let (second, last): (&[u8], &[u8]) = rest.split_at(10);
            let mut request = session.begin_receive(delivery.connection)?;
            assert!(request.complete_fragment(first)?.is_none());
            assert!(request.complete_fragment(second)?.is_none());
            let reply: Reply = request
                .complete_fragment(last)?
                .ok_or(SessionFault::Frame(FrameError::Length))?;
            assert_eq!(
                request.complete_fragment(last).err(),
                Some(SessionFault::Closed)
            );
            *slot = pending.resolve(reply)?;
        }
        let next = client.request()?;
        let refused: Option<SessionFault> = session.receive(next.delivery()).err();
        Ok((accepted, refused))
    })?;
    assert_eq!(actual, Ok(([true; 4], Some(SessionFault::Closed))));
    Ok(())
}

/// A confirmed short write advances by its actual count. An ambiguous or
/// expired transfer cannot reopen its connection or complete the request.
/// # Errors
/// Returns an unexpected fixture startup or cleanup failure.
/// # Panics
/// Fails when delivery, physical progress or original currentness differs.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn reply_write_retains_offset_and_original_cutoff() -> Result<(), Box<dyn Error>> {
    let observed: Result<(), SessionFault> = with_broker::<17, _>(|owner, clock, stop| {
        let (mut session, mut client) =
            BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
        let pending: PendingStatus<'_> = client.request()?;
        let reply: Reply = session.receive(pending.delivery())?;
        let len: usize = reply.frame.as_bytes().len();
        let mut sent: [u8; MAX_FRAME_LEN] = [0; MAX_FRAME_LEN];
        let header: NonZeroUsize = NonZeroUsize::new(16).ok_or(SessionFault::WriteProgress)?;
        let mut transfer: OutboundStatus<'_, '_, Reporter, ScriptClock<'_>> =
            session.begin_reply(reply)?;
        sent[..6].copy_from_slice(&transfer.next_write(header)?[..6]);
        assert_eq!(
            transfer.next_write(header),
            Err(SessionFault::WriteProgress)
        );
        assert!(!transfer.complete_write(6)?);
        assert!(transfer.write.pending_bytes().is_none());
        assert!(transfer.write.can_reserve());
        let limit: NonZeroUsize = NonZeroUsize::new(len).ok_or(SessionFault::WriteProgress)?;
        sent[6..len].copy_from_slice(transfer.next_write(limit)?);
        assert!(transfer.complete_write(len - 6)?);
        assert!(transfer.write.pending_bytes().is_none());
        let AssemblyProgress::Pending(partial) = FrameAssembly::reply()
            .push(&sent[..16])
            .map_err(SessionFault::Frame)?
        else {
            return Err(SessionFault::WriteProgress);
        };
        let AssemblyProgress::Complete(received) =
            partial.push(&sent[16..len]).map_err(SessionFault::Frame)?
        else {
            return Err(SessionFault::WriteProgress);
        };
        assert!(transfer.resolve_received(pending, &received)?);

        let pending: PendingStatus<'_> = client.request()?;
        let reply: Reply = session.receive(pending.delivery())?;
        let mut transfer: OutboundStatus<'_, '_, Reporter, ScriptClock<'_>> =
            session.begin_reply(reply)?;
        let count: usize = transfer.next_write(header)?.len();
        transfer.receiver.clock.now = transfer.receiver.cutoff;
        assert_eq!(transfer.complete_write(count), Err(SessionFault::Expired));
        assert_eq!(transfer.next_write(header), Err(SessionFault::Closed));
        let late: Frame = encode_reply(pending.id, true);
        assert_eq!(
            transfer.resolve_received(pending, &late),
            Err(SessionFault::Closed)
        );
        assert_eq!(client.request().err(), Some(SessionFault::Closed));
        Ok(())
    })?;
    assert_eq!(observed, Ok(()));

    let oversize: Result<(), SessionFault> = with_broker::<18, _>(|owner, clock, stop| {
        let (mut session, mut client) =
            BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
        let pending: PendingStatus<'_> = client.request()?;
        let reply: Reply = session.receive(pending.delivery())?;
        let mut transfer: OutboundStatus<'_, '_, Reporter, ScriptClock<'_>> =
            session.begin_reply(reply)?;
        let header: NonZeroUsize = NonZeroUsize::new(16).ok_or(SessionFault::WriteProgress)?;
        assert_eq!(transfer.next_write(header)?.len(), 16);
        assert_eq!(
            transfer.complete_write(17),
            Err(SessionFault::WriteProgress)
        );
        assert!(!transfer.write.can_reserve());
        assert!(transfer.write.pending_bytes().is_none());
        assert_eq!(transfer.next_write(header), Err(SessionFault::Closed));
        let late: Frame = encode_reply(pending.id, true);
        assert_eq!(
            transfer.resolve_received(pending, &late),
            Err(SessionFault::Closed)
        );
        assert_eq!(client.request().err(), Some(SessionFault::Closed));
        Ok(())
    })?;
    assert_eq!(oversize, Ok(()));
    Ok(())
}

/// Keep one initial cutoff and the original Stop source as fragments arrive.
/// # Errors
/// Returns unexpected fixture startup, cleanup or prefix refusal.
/// # Panics
/// Fails when the last fragment publishes after either disqualifying transition.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn split_request_respects_cutoff_and_stop() -> Result<(), Box<dyn Error>> {
    let cutoff: Result<_, SessionFault> = with_broker::<8, _>(|owner, clock, stop| {
        let (mut session, mut client) =
            BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
        let pending = client.request()?;
        let delivery: Delivery<'_> = pending.delivery();
        let (header, body): (&[u8], &[u8]) = delivery.bytes.split_at(16);
        let mut request = session.begin_receive(delivery.connection)?;
        assert!(request.complete_fragment(header)?.is_none());
        request.receiver.clock.now = request.receiver.cutoff;
        // One further byte would still leave an incomplete frame. The
        // receiving guard must refuse it without relying on final dispatch.
        let (next_byte, _tail): (&[u8], &[u8]) = body.split_at(1);
        Ok(request.complete_fragment(next_byte).err())
    })?;
    assert_eq!(cutoff, Ok(Some(SessionFault::Expired)));

    let stopped: Result<_, SessionFault> = with_broker::<9, _>(|owner, clock, stop| {
        let (mut session, mut client) =
            BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
        let pending = client.request()?;
        let delivery: Delivery<'_> = pending.delivery();
        let (header, body): (&[u8], &[u8]) = delivery.bytes.split_at(16);
        let mut request = session.begin_receive(delivery.connection)?;
        assert!(request.complete_fragment(header)?.is_none());
        let _stop_disposition = deliver(Some(&MAILBOXES[9]), stop, 1, 0);
        Ok(request.complete_fragment(body).err())
    })?;
    assert_eq!(stopped, Ok(Some(SessionFault::Closed)));
    Ok(())
}

/// A due broker sample progresses while the receiver retains a split request.
/// # Errors
/// Returns startup, receive or reply refusal from the actual fixture.
/// # Panics
/// Fails if the due sample or the original request is lost.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn broker_poll_samples_during_a_partial_frame() -> Result<(), Box<dyn Error>> {
    let actual: Result<_, SessionFault> = with_broker_reads::<12, 2, _>(|owner, clock, stop| {
        let (mut session, mut client): (
            BrokerStatusSession<'_, Reporter, ScriptClock<'_>>,
            EngineStatusClient,
        ) = BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
        let pending = client.request()?;
        let delivery: Delivery<'_> = pending.delivery();
        let (header, body): (&[u8], &[u8]) = delivery.bytes.split_at(16);
        let mut request = session.begin_receive(delivery.connection)?;
        let partial: bool = request.complete_fragment(header)?.is_none();
        let before: u32 = GETTERS[12].load(SeqCst);
        request.receiver.clock.now += Duration::from_secs(1);
        let poll: InvocationPoll = request.poll_broker();
        let after: u32 = GETTERS[12].load(SeqCst);
        let reply: Reply = request
            .complete_fragment(body)?
            .ok_or(SessionFault::Frame(FrameError::Length))?;
        Ok((partial, before, poll, after, pending.resolve(reply)?))
    })?;
    assert_eq!(
        actual,
        Ok((
            true,
            1,
            InvocationPoll::Pending(Duration::from_secs(1)),
            2,
            true
        ))
    );
    Ok(())
}

/// The delivered event and receiver observations kept through actual owner cleanup.
struct SplitPowerObservation {
    /// Trusted owner coordinate at original connection admission.
    start: Instant,
    /// Independently configured limits observed after the sample renews.
    session_cutoff: Instant,
    /// Original service lifetime through the fresh request.
    run_cutoff: Option<Instant>,
    /// Actual getter calls after the suspended poll, crossed reply and fresh poll.
    reads: [u32; 3],
    /// Actual admitted wait while suspended and after the fresh poll.
    waits: [InvocationPoll; 2],
    /// Correlated old, crossed and fresh replies.
    replies: [bool; 3],
    /// Remaining charge after each of the four original deliveries.
    remaining: [u8; 4],
    /// Original outstanding service work after the new clock sample.
    work: u32,
    /// Callback status and wake after suspend, resume, Stop and late resume.
    controls: [(u32, bool); 4],
    /// Exact rejection of the late body and the connection state it leaves.
    stopped: (Option<SessionFault>, bool),
}

/// Drive one complete power cycle across a split request on the original session.
/// # Errors
/// Returns the actual startup, cleanup, receive or reply refusal when reached.
fn split_status_across_power_cycle<const N: usize>(
    owner: &mut ServiceInvocation<Reporter, InertWorker>,
    clock: &mut ScriptClock<'_>,
    stop: &StopNotification,
) -> Result<SplitPowerObservation, SessionFault> {
    let start: Instant = clock.now;
    let (mut session, mut client): (
        BrokerStatusSession<'_, Reporter, ScriptClock<'_>>,
        EngineStatusClient,
    ) = BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
    let original: bool = client.read_availability(&mut session)? == BrokerAvailability::Available;
    let after_original: u8 = session.remaining;

    let pending: PendingStatus<'_> = client.request()?;
    let delivery: Delivery<'_> = pending.delivery();
    let (header, body): (&[u8], &[u8]) = delivery.bytes.split_at(16);
    let mut request: InboundStatus<'_, '_, Reporter, ScriptClock<'_>> =
        session.begin_receive(delivery.connection)?;
    if request.complete_fragment(header)?.is_some() {
        return Err(SessionFault::Frame(FrameError::Length));
    }
    let suspended: ControlDisposition = deliver(Some(&MAILBOXES[N]), stop, 13, 4);
    request.receiver.clock.now += Duration::from_secs(1);
    let suspend_wait: InvocationPoll = request.poll_broker();
    let suspend_reads: u32 = GETTERS[N].load(SeqCst);

    let resumed: ControlDisposition = deliver(Some(&MAILBOXES[N]), stop, 13, 18);
    let crossing_reply: Reply = request
        .complete_fragment(body)?
        .ok_or(SessionFault::Frame(FrameError::Length))?;
    let crossing: bool = pending.resolve(crossing_reply)?;
    drop(request);
    let crossing_reads: u32 = GETTERS[N].load(SeqCst);
    let after_crossing: u8 = session.remaining;

    let fresh_wait: InvocationPoll = session.owner.poll(session.clock, stop);
    let fresh_reads: u32 = GETTERS[N].load(SeqCst);
    let fresh: bool = client.read_availability(&mut session)? == BrokerAvailability::Available;
    let after_fresh: u8 = session.remaining;
    let session_cutoff: Instant = session.cutoff;
    let run_cutoff: Option<Instant> = session.owner.run_deadline;
    let work: u32 = session.owner.outstanding_work();

    let pending: PendingStatus<'_> = client.request()?;
    let delivery: Delivery<'_> = pending.delivery();
    let (header, body): (&[u8], &[u8]) = delivery.bytes.split_at(16);
    let mut request: InboundStatus<'_, '_, Reporter, ScriptClock<'_>> =
        session.begin_receive(delivery.connection)?;
    if request.complete_fragment(header)?.is_some() {
        return Err(SessionFault::Frame(FrameError::Length));
    }
    let stopped: ControlDisposition = deliver(Some(&MAILBOXES[N]), stop, 1, 0);
    let late_resume: ControlDisposition = deliver(Some(&MAILBOXES[N]), stop, 13, 18);
    let refused_body: Option<SessionFault> = request.complete_fragment(body).err();
    Ok(SplitPowerObservation {
        start,
        session_cutoff,
        run_cutoff,
        reads: [suspend_reads, crossing_reads, fresh_reads],
        waits: [suspend_wait, fresh_wait],
        replies: [original, crossing, fresh],
        remaining: [
            after_original,
            after_crossing,
            after_fresh,
            request.receiver.remaining,
        ],
        work,
        controls: [suspended, resumed, stopped, late_resume]
            .map(|control: ControlDisposition| (control.status, control.wake)),
        stopped: (refused_body, !request.receiver.connection.get()),
    })
}

/// The crossed reply cannot report an old sample, and the fresh neighbor succeeds.
/// # Errors
/// Returns an unexpected broker startup, cleanup or receive failure.
/// # Panics
/// Fails on a stale reply, an extra getter call, renewed admission or lost Stop.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn split_status_reports_fresh_sample_after_power_cycle() -> Result<(), Box<dyn Error>> {
    const CASE: usize = 40;
    let (actual, receipt): (Result<SplitPowerObservation, SessionFault>, HostReceipt) =
        run_broker_case::<CASE, 2, _, InertWorker>(
            InertWorker::new(),
            split_status_across_power_cycle::<CASE>,
        )?;
    let observed: SplitPowerObservation = actual
        .map_err(|fault: SessionFault| format!("split power-cycle request refused: {fault:?}"))?;
    assert_eq!(
        observed.controls,
        [(0, true), (0, true), (0, true), (120, false)]
    );
    assert_eq!(observed.reads, [1, 1, 2]);
    assert_eq!(
        observed.waits,
        [
            InvocationPoll::Pending(Duration::from_secs(4)),
            InvocationPoll::Pending(Duration::from_secs(1))
        ]
    );
    assert_eq!(observed.replies, [true, false, true]);
    assert_eq!(observed.remaining, [3, 2, 1, 0]);
    assert_eq!(
        observed.session_cutoff,
        observed.start + Duration::from_secs(5)
    );
    assert_eq!(
        observed.run_cutoff,
        Some(observed.start + Duration::from_secs(60))
    );
    assert_eq!(observed.work, 1);
    assert_eq!(observed.stopped, (Some(SessionFault::Closed), true));
    assert_eq!(GETTERS[CASE].load(SeqCst), 2);
    assert!(matches!(
        receipt.broker_clock_read(),
        Some(BrokerClockRead::Completed { revision, result: Ok(_), .. })
            if revision.get() == 3
    ));
    Ok(())
}

/// The original session cutoff limits a live wait and retires idle partial input.
/// # Errors
/// Returns startup, receive or clock setup failure from the actual fixture.
/// # Panics
/// Fails if expiry waits past the cutoff, stops the broker or accepts late bytes.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn broker_poll_retires_an_idle_frame_at_the_original_cutoff() -> Result<(), Box<dyn Error>> {
    let actual: Result<_, SessionFault> = with_broker_reads::<13, 2, _>(|owner, clock, stop| {
        let cutoff: Instant = clock
            .now
            .checked_add(Duration::from_secs(5))
            .ok_or(SessionFault::Expired)?;
        let (mut session, mut client): (
            BrokerStatusSession<'_, Reporter, ScriptClock<'_>>,
            EngineStatusClient,
        ) = BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
        let pending = client.request()?;
        let delivery: Delivery<'_> = pending.delivery();
        let (header, body): (&[u8], &[u8]) = delivery.bytes.split_at(16);
        let mut request = session.begin_receive(delivery.connection)?;
        let partial: bool = request.complete_fragment(header)?.is_none();
        let remaining: Duration = Duration::from_millis(200);
        request.receiver.clock.now = cutoff.checked_sub(remaining).ok_or(SessionFault::Expired)?;
        let early: InvocationPoll = request.poll_broker();
        let open: bool = request.receiver.connection.get();
        request.receiver.clock.now = cutoff;
        let boundary: InvocationPoll = request.poll_broker();
        let closed: bool = !request.receiver.connection.get();
        let broker_open: bool = !request.receiver.owner.admission_closed();
        let late: Option<SessionFault> = request.complete_fragment(body).err();
        Ok((partial, early, open, boundary, closed, broker_open, late))
    })?;
    assert_eq!(
        actual,
        Ok((
            true,
            InvocationPoll::Pending(Duration::from_millis(200)),
            true,
            InvocationPoll::Pending(Duration::from_millis(800)),
            true,
            true,
            Some(SessionFault::Closed)
        ))
    );
    Ok(())
}

/// Preserve the typed fixture failure in the selected Windows test report.
/// # Errors
/// Returns the observed failure with its debug representation.
#[cfg(windows)]
fn native_fixture_result<T>(result: Result<T, impl std::fmt::Debug>) -> Result<T, Box<dyn Error>> {
    result.map_err(|error| {
        // Preserve the first error even if later fixture cleanup checks fail.
        let _diagnostic: std::io::Result<()> = writeln!(
            std::io::stderr().lock(),
            "native pipe fixture refused: {error:?}"
        );
        format!("{error:?}").into()
    })
}

/// Wait for the native terminal result under a separate real elapsed bound.
/// # Errors
/// Returns an unrecognized observation or a two-second completion timeout.
#[cfg(windows)]
fn wait_for_native_read(reader: &mut NativePipeRead) -> Result<NativeReadPoll, Box<dyn Error>> {
    let started: Instant = Instant::now();
    loop {
        match reader.poll() {
            Ok(NativeReadPoll::Pending) if started.elapsed() < Duration::from_secs(2) => {
                std::thread::sleep(Duration::from_millis(1));
            }
            Ok(NativeReadPoll::Pending) => {
                let message: &str = "native pipe read still pending after two seconds";
                // An unresolved read triggers fatal Drop. Write outside libtest's capture.
                let _diagnostic: std::io::Result<()> =
                    writeln!(std::io::stderr().lock(), "{message}");
                return Err(message.into());
            }
            Ok(completion) => return Ok(completion),
            Err(error) => {
                let _diagnostic: std::io::Result<()> = writeln!(
                    std::io::stderr().lock(),
                    "native pipe completion observation failed: {error:?}"
                );
                return Err(format!("{error:?}").into());
            }
        }
    }
}

/// Wait at most two real seconds for the actual ingress and identity observation.
/// # Errors
/// Returns the original native transport or capture refusal.
#[cfg(windows)]
fn wait_for_identified_read<E>(
    mut poll: impl FnMut() -> Result<NativeReadPoll, E>,
) -> Result<Option<NativeReadPoll>, E> {
    let started: Instant = Instant::now();
    loop {
        match poll()? {
            NativeReadPoll::Pending if started.elapsed() < Duration::from_secs(2) => {
                std::thread::sleep(Duration::from_millis(1));
            }
            NativeReadPoll::Pending => return Ok(None),
            terminal => return Ok(Some(terminal)),
        }
    }
}

/// Retain the exact ingress operation while attempting bounded failed-read cleanup.
#[cfg(windows)]
fn retire_pending_identified_read(reader: &mut IdentifiedRequestRead) {
    if !reader.is_pending() {
        return;
    }
    if let Err(error) = reader.retire() {
        let _diagnostic: std::io::Result<()> = writeln!(
            std::io::stderr().lock(),
            "identified read cancellation refused: {error:?}"
        );
    }
    if wait_for_retired_read(|| reader.poll_retirement()).is_err() && reader.is_pending() {
        let _diagnostic: std::io::Result<()> = writeln!(
            std::io::stderr().lock(),
            "identified ingress still owns its native read"
        );
    }
}

/// One paired native ingress bound by the lab composer to its logical connection.
#[cfg(windows)]
struct BoundRequestPipe {
    connection: Connection,
    request_id: super::RequestId,
    submitted: usize,
}

/// Preserve logical refusal separately from native observations and failures.
#[cfg(windows)]
enum RequestSubmissionError {
    Session(SessionFault),
    Pipe(LabPipeError),
}

#[cfg(windows)]
impl BoundRequestPipe {
    /// Bind the paired endpoints to one issued request on the original connection.
    fn new(pending: &PendingStatus<'_>) -> Self {
        Self {
            connection: std::rc::Rc::clone(pending.delivery().connection),
            request_id: pending.id,
            submitted: 0,
        }
    }

    /// Submit the pending request's exact extent on the original paired handles.
    /// # Errors
    /// Refuses a foreign, inactive or expired request, extent or native peer.
    fn submit(
        &mut self,
        ingress: &mut InboundStatus<'_, '_, Reporter, ScriptClock<'_>, NativePipeWorker>,
        pending: &PendingStatus<'_>,
    ) -> Result<(), RequestSubmissionError> {
        if !std::rc::Rc::ptr_eq(&self.connection, pending.delivery().connection) {
            return Err(RequestSubmissionError::Session(
                SessionFault::ForeignConnection,
            ));
        }
        if ingress.assembly.is_none() {
            return Err(RequestSubmissionError::Session(SessionFault::Closed));
        }
        if pending.id != self.request_id {
            return Err(RequestSubmissionError::Session(SessionFault::Frame(
                FrameError::Correlation,
            )));
        }
        let length = || RequestSubmissionError::Session(SessionFault::Frame(FrameError::Length));
        let closed = || RequestSubmissionError::Session(SessionFault::Closed);
        let read: ActiveRequestRead<'_> = ingress
            .receiver
            .owner
            .worker
            .request_read()
            .ok_or_else(closed)?;
        let count: usize = read
            .client_write_span()
            .and_then(|span: u32| usize::try_from(span).ok())
            .ok_or_else(length)?;
        let end: usize = self.submitted.checked_add(count).ok_or_else(length)?;
        let bytes: &[u8] = pending
            .delivery()
            .bytes
            .get(self.submitted..end)
            .ok_or_else(length)?;
        read.check_peers().map_err(RequestSubmissionError::Pipe)?;
        ingress
            .receiver
            .check_client_edge(&self.connection)
            .map_err(RequestSubmissionError::Session)?;
        let mut read: ActiveRequestRead<'_> = ingress
            .receiver
            .owner
            .worker
            .request_read()
            .ok_or_else(closed)?;
        self.submitted = end;
        if let Err(error) = read.write_from_client(bytes) {
            ingress.receiver.close();
            return Err(RequestSubmissionError::Pipe(error));
        }
        Ok(())
    }
}

/// Lend the ingress slot from the same active owner that receives the request.
/// # Errors
/// Refuses a resolved receive, expired session, Stop or absent worker ingress.
#[cfg(windows)]
fn request_slot<'borrow>(
    ingress: &'borrow mut InboundStatus<'_, '_, Reporter, ScriptClock<'_>, NativePipeWorker>,
) -> Result<ActiveRequestRead<'borrow>, Box<dyn Error>> {
    if ingress.assembly.is_none() || !ingress.receiver.connection.get() {
        return native_fixture_result(Err(SessionFault::Closed));
    }
    native_fixture_result(ingress.receiver.check_receive_edge())?;
    ingress
        .receiver
        .owner
        .worker
        .request_read()
        .ok_or_else(|| "broker request read is inactive".into())
}

/// Write one fragment and return actual bytes only after successful native identification.
/// # Errors
/// Returns the original write, identity, transport or bounded wait failure.
#[cfg(windows)]
fn write_identified_fragment(
    pipe: &mut BoundRequestPipe,
    ingress: &mut InboundStatus<'_, '_, Reporter, ScriptClock<'_>, NativePipeWorker>,
    pending: &PendingStatus<'_>,
) -> Result<NativeReadBytes, Box<dyn Error>> {
    if let Err(error) = pipe.submit(ingress, pending) {
        return match error {
            RequestSubmissionError::Session(cause) => native_fixture_result(Err(cause)),
            RequestSubmissionError::Pipe(cause) => native_fixture_result(Err(cause)),
        };
    }
    let completed: Option<NativeReadPoll> =
        wait_for_identified_read(|| native_fixture_result(request_slot(ingress)?.poll()))?;
    if let Some(NativeReadPoll::Data(completed)) = completed {
        native_fixture_result(ensure_unimpersonated())?;
        Ok(completed)
    } else {
        native_fixture_result(Err("identified read did not deliver a fragment"))
    }
}

/// Observe a physical write terminal result within the fixture's real time bound.
/// # Errors
/// Returns the original observation failure or a finite timeout.
#[cfg(windows)]
fn wait_for_native_write(writer: &mut NativePipeWorker) -> Result<NativeWritePoll, Box<dyn Error>> {
    let start: Instant = Instant::now();
    loop {
        let poll: NativeWritePoll = native_fixture_result(writer.poll_write())?;
        if poll != NativeWritePoll::Pending {
            return Ok(poll);
        }
        if start.elapsed() >= Duration::from_secs(2) {
            let _diagnostic: std::io::Result<()> = writeln!(
                std::io::stderr().lock(),
                "native reply write still pending after two seconds"
            );
            return Err("native reply write did not reach completion".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// Deliver an actual broker reply to its original pending Engine receiver.
/// # Errors
/// Returns native, framing, currentness or correlation refusal.
/// # Panics
/// Fails when the selected empty byte pipe publishes a partial or absent response.
#[cfg(windows)]
fn receive_native_reply(
    transfer: OutboundStatus<'_, '_, Reporter, ScriptClock<'_>, NativePipeWorker>,
    pending: PendingStatus<'_>,
    reader: &mut NativePipeRead,
) -> Result<bool, Box<dyn Error>> {
    // Only an admitted, idle reader can enter this transfer's cleanup scope.
    transfer.receiver.owner.worker.check_reply_reader(reader)?;
    if reader.is_pending() {
        return native_fixture_result(Err(NativeReadError::Busy));
    }
    native_fixture_result(
        transfer
            .receiver
            .check_client_edge(pending.delivery().connection),
    )?;
    let connection: Connection = std::rc::Rc::clone(&transfer.connection);
    let result: Result<bool, Box<dyn Error>> = transfer_native_reply(transfer, pending, reader);
    if result.is_err() {
        connection.set(false);
        retire_pending_peer_read(reader);
    }
    result
}

/// Transfer one complete native response before releasing either logical owner.
/// # Errors
/// Returns native, framing, currentness or correlation refusal.
/// # Panics
/// Fails when the selected empty byte pipe publishes a partial or absent response.
#[cfg(windows)]
fn transfer_native_reply(
    mut transfer: OutboundStatus<'_, '_, Reporter, ScriptClock<'_>, NativePipeWorker>,
    pending: PendingStatus<'_>,
    reader: &mut NativePipeRead,
) -> Result<bool, Box<dyn Error>> {
    let limit: NonZeroUsize = NonZeroUsize::new(16).ok_or("invalid fragment cap")?;
    let mut assembly: FrameAssembly = FrameAssembly::reply();
    loop {
        let extent: NonZeroUsize = native_fixture_result(transfer.reserve_write(limit))?;
        let span: usize = extent.get();
        let read_start: NativeReadStart = native_fixture_result(reader.begin(span))?;
        native_fixture_result(transfer.receiver.check_receive_edge())?;
        let bytes: &[u8] = transfer
            .write
            .pending_bytes()
            .ok_or("reserved reply fragment is absent")?;
        let writer: &mut NativePipeWorker = &mut transfer.receiver.owner.worker;
        let _write_start: NativeWriteStart = native_fixture_result(writer.begin(bytes))?;
        let write: NativeWritePoll = wait_for_native_write(writer)?;
        let NativeWritePoll::Written(count) = write else {
            let _diagnostic: std::io::Result<()> = writeln!(
                std::io::stderr().lock(),
                "native reply write ended with {write:?}"
            );
            return Err("native reply ended without a successful write".into());
        };
        let NativeReadPoll::Data(received) = wait_for_native_read(reader)? else {
            return Err("native reply ended without bytes".into());
        };
        transfer
            .receiver
            .owner
            .worker
            .check_reply_completion(&received)?;
        let complete: bool = native_fixture_result(transfer.complete_write(count))?;
        assert_eq!(read_start, NativeReadStart::Queued);
        assert_eq!(received.as_bytes().len(), count);
        match native_fixture_result(assembly.push(received.as_bytes()))? {
            AssemblyProgress::Pending(partial) => {
                assert!(!complete);
                assembly = partial;
            }
            AssemblyProgress::Complete(frame) => {
                assert!(complete);
                let available: bool =
                    native_fixture_result(transfer.decode_received(pending, &frame))?;
                transfer.receiver.owner.worker.check_reply_reader(reader)?;
                native_fixture_result(transfer.receiver.check_receive_edge())?;
                return Ok(available);
            }
        }
    }
}

/// Request cancellation once for the peer's already-posted read and try to
/// observe completion. Keep its first failure visible beside the caller's.
#[cfg(windows)]
fn retire_pending_peer_read(reader: &mut NativePipeRead) {
    if !reader.is_pending() {
        return;
    }
    if let Err(error) = reader.request_cancel() {
        let _diagnostic: std::io::Result<()> = writeln!(
            std::io::stderr().lock(),
            "native peer read cleanup could not request cancellation: {error:?}"
        );
    }
    let _completion: Result<_, Box<dyn Error>> = wait_for_native_read(reader);
}

/// Write one exact encoded fragment and observe its actual native terminal read.
/// # Errors
/// Returns a write failure, a completion observation failure or a non-data terminal result.
#[cfg(windows)]
fn write_completed_fragment(
    reader: &mut NativePipeRead,
    writer: &LabPipeWriter,
    bytes: &[u8],
) -> Result<NativeReadBytes, Box<dyn Error>> {
    if let Err(error) = writer.write(bytes) {
        let _diagnostic: std::io::Result<()> = writeln!(
            std::io::stderr().lock(),
            "native fixture peer write failed: {error:?}"
        );
        retire_pending_peer_read(reader);
        return Err(format!("{error:?}").into());
    }
    if let NativeReadPoll::Data(completed) = wait_for_native_read(reader)? {
        Ok(completed)
    } else {
        Err("native pipe read terminated without data".into())
    }
}

/// Distinguish service admission refusal from failed native slot preparation.
#[cfg(windows)]
enum DuplexAdmissionError {
    /// The original session expired or stopped before adopting a connection.
    Session(SessionFault),
    /// The worker could not attach the admitted native transport.
    Worker(NativePipeWorkerError),
}

/// Transfer the already-connected transport under this exact session and worker.
/// # Errors
/// Refuses a stale session or unable worker without creating a request or reply.
#[cfg(windows)]
fn attach_duplex(
    session: &mut BrokerStatusSession<'_, Reporter, ScriptClock<'_>, NativePipeWorker>,
    ready: PreparedDuplex,
) -> Result<NativePipeRead, DuplexAdmissionError> {
    session
        .check_receive_edge()
        .map_err(DuplexAdmissionError::Session)?;
    session
        .owner
        .worker
        .attach_duplex(ready)
        .map_err(DuplexAdmissionError::Worker)
}

/// Acquire the original accept under Running and attach its prepared slots at the live edge.
/// # Errors
/// Retains native failure or reports unexpected refusal or admission.
/// # Panics
/// Fails if Running lacks its permit or a refused handoff issued a request.
#[cfg(windows)]
fn connect_for_session(
    session: &mut BrokerStatusSession<'_, Reporter, ScriptClock<'_>, NativePipeWorker>,
    client: &EngineStatusClient,
    pipe_instance: u32,
    peer: PipeIdentity,
    exit: NativeIngressExit,
) -> Result<Option<NativePipeRead>, Box<dyn Error>> {
    native_fixture_result(session.check_client_edge(&client.connection))?;
    assert!(session.owner.permit.is_some());
    let connected: LabDuplexPipe = native_fixture_result(open_lab_duplex_pipe(pipe_instance))?;
    native_fixture_result(session.check_receive_edge())?;
    let ready: PreparedDuplex = native_fixture_result(PreparedDuplex::new(connected, peer))?;
    let expected: Option<SessionFault> = match exit {
        NativeIngressExit::StopBeforeAttach => {
            let _notice: ControlDisposition =
                deliver(Some(session.clock.mailbox), session.stop, 1, 0);
            Some(SessionFault::Closed)
        }
        NativeIngressExit::ExpireBeforeAttach => {
            session.clock.now = session.cutoff;
            Some(SessionFault::Expired)
        }
        _ => None,
    };
    match attach_duplex(session, ready) {
        Err(DuplexAdmissionError::Session(error)) if Some(error) == expected => {
            assert!(session.owner.worker.request_read().is_none());
            assert_eq!(client.next, Some(1));
            assert!(session.last.is_none());
            Ok(None)
        }
        Ok(reader) if expected.is_none() => Ok(Some(reader)),
        Ok(_) => Err("broker adopted the connected pipe after losing admission".into()),
        Err(DuplexAdmissionError::Session(error)) => native_fixture_result(Err(error)),
        Err(DuplexAdmissionError::Worker(error)) => native_fixture_result(Err(error)),
    }
}

/// Prove the original broker still samples while an actual Windows read waits.
/// The broker responds on that same duplex pipe or stops with the body read uncollected.
/// # Errors
/// Returns native pipe, invocation, frame or reply failure.
/// # Panics
/// Fails if an idle read completes early or successful data fails to reach the broker.
#[cfg(windows)]
fn native_read_feeds_broker<const N: usize, const READS: u32>(
    exit: NativeIngressExit,
) -> Result<(), Box<dyn Error>> {
    let pipe_instance: u32 = u32::try_from(N)?;
    let process_identity: PipeIdentity = native_fixture_result(own_process_identity())?;
    let worker: NativePipeWorker = NativePipeWorker::awaiting_connection();
    with_broker_worker_reads::<N, READS, _, NativePipeWorker>(worker, |owner, clock, stop| {
        let (mut session, mut client): (
            BrokerStatusSession<'_, Reporter, ScriptClock<'_>, NativePipeWorker>,
            EngineStatusClient,
        ) = native_fixture_result(BrokerStatusSession::open(
            owner,
            clock,
            stop,
            LabRole::Engine,
        ))?;
        let Some(mut reply_read) =
            connect_for_session(&mut session, &client, pipe_instance, process_identity, exit)?
        else {
            return Ok(());
        };
        let pending: PendingStatus<'_> = native_fixture_result(client.request())?;
        let mut pipe: BoundRequestPipe = BoundRequestPipe::new(&pending);
        let delivery: Delivery<'_> = pending.delivery();
        let (header, body): (&[u8], &[u8]) = delivery.bytes.split_at(16);
        let mut request: InboundStatus<'_, '_, Reporter, ScriptClock<'_>, NativePipeWorker> =
            native_fixture_result(session.begin_receive(delivery.connection))?;
        let due: Instant = request.receiver.clock.now + Duration::from_secs(1);

        let header_start: NativeReadStart =
            native_fixture_result(request_slot(&mut request)?.begin(header.len()))?;
        let header_idle: bool = matches!(
            request_slot(&mut request)?.poll(),
            Ok(NativeReadPoll::Pending)
        );
        let before: u32 = GETTERS[N].load(SeqCst);
        request.receiver.clock.now = due;
        let broker_poll: InvocationPoll = request.poll_broker();
        let after: u32 = GETTERS[N].load(SeqCst);
        let native_header: NativeReadBytes =
            write_identified_fragment(&mut pipe, &mut request, &pending)?;
        assert!(matches!(header_start, NativeReadStart::Queued));
        assert!(header_idle);
        assert_eq!((before, after), (1, 2));
        assert_eq!(broker_poll, InvocationPoll::Pending(Duration::from_secs(1)));
        assert_eq!(native_header.as_bytes(), header);
        assert!(
            native_fixture_result(request.complete_fragment(native_header.as_bytes()))?.is_none()
        );

        let body_start: NativeReadStart =
            native_fixture_result(request_slot(&mut request)?.begin(body.len()))?;
        let body_idle: bool = matches!(
            native_fixture_result(request_slot(&mut request)?.poll())?,
            NativeReadPoll::Pending
        );
        if body_start != NativeReadStart::Queued || !body_idle {
            return native_fixture_result(Err("body read was not queued on the empty pipe"));
        }
        match exit {
            NativeIngressExit::CompleteThenStop => {
                return stop_after_native_read_completion(&mut pipe, &mut request, &pending, body);
            }
            NativeIngressExit::Stop => {
                return stop_with_native_read(
                    &mut request,
                    body,
                    NativeCancel::Requested,
                    RetiredRead::Cancelled,
                );
            }
            NativeIngressExit::ExpireThenStop => {
                return stop_after_native_read_expiry(&mut request, body);
            }
            NativeIngressExit::Reply => {}
            NativeIngressExit::StopBeforeAttach | NativeIngressExit::ExpireBeforeAttach => {
                return Err("closed broker reached request dispatch".into());
            }
        }
        let native_body: NativeReadBytes =
            write_identified_fragment(&mut pipe, &mut request, &pending)?;
        assert!(!request_slot(&mut request)?.is_pending());
        assert_eq!(native_body.as_bytes(), body);
        let reply: Reply =
            native_fixture_result(request.complete_fragment(native_body.as_bytes()))?
                .ok_or("the broker did not reply to a complete native frame")?;
        drop(request);
        let transfer: OutboundStatus<'_, '_, Reporter, ScriptClock<'_>, NativePipeWorker> =
            native_fixture_result(session.begin_reply(reply))?;
        assert!(receive_native_reply(transfer, pending, &mut reply_read)?);
        Ok(())
    })?
}

/// Selected terminal action for the same actual request and native ingress.
#[cfg(windows)]
#[derive(Clone, Copy)]
enum NativeIngressExit {
    /// Stop after operation preparation and before the worker attaches the slots.
    StopBeforeAttach,
    /// Let the original connection cutoff reach equality before slot attachment.
    ExpireBeforeAttach,
    /// Complete the identified request and deliver the broker reply.
    Reply,
    /// Stop after the body read signals, while the result still awaits collection.
    CompleteThenStop,
    /// Stop the broker while its original body read is still outstanding.
    Stop,
    /// Expire the receive and request cancellation before the broker receives Stop.
    ExpireThenStop,
}

/// Publish Stop after physical body completion while the original read is still owned.
/// # Errors
/// Preserves submission, completion-event or Stop failure.
/// # Panics
/// Fails if the complete request lacked custody, dispatched or escaped discard.
#[cfg(windows)]
fn stop_after_native_read_completion(
    pipe: &mut BoundRequestPipe,
    request: &mut InboundStatus<'_, '_, Reporter, ScriptClock<'_>, NativePipeWorker>,
    pending: &PendingStatus<'_>,
    body: &[u8],
) -> Result<(), Box<dyn Error>> {
    if let Err(error) = pipe.submit(request, pending) {
        return match error {
            RequestSubmissionError::Session(failure) => native_fixture_result(Err(failure)),
            RequestSubmissionError::Pipe(failure) => native_fixture_result(Err(failure)),
        };
    }
    let read: ActiveRequestRead<'_> = request_slot(request)?;
    native_fixture_result(read.wait_for_read_signal())?;
    let uncollected: bool = read.is_pending();
    let permit_held: bool = request.receiver.owner.permit.is_some();
    let undispatched: bool = request.receiver.last.is_none();
    stop_with_native_read(
        request,
        body,
        NativeCancel::NotFound,
        RetiredRead::Discarded { len: body.len() },
    )?;
    assert_eq!(pipe.submitted, pending.delivery().bytes.len());
    assert!(uncollected && permit_held && undispatched);
    Ok(())
}

/// Retire the original read at the receive cutoff, then publish Stop on the same owner.
/// # Errors
/// Preserves operation retirement or Stop failure.
/// # Panics
/// Fails if expiry admits the read or Stop loses the original cancellation result.
#[cfg(windows)]
fn stop_after_native_read_expiry(
    request: &mut InboundStatus<'_, '_, Reporter, ScriptClock<'_>, NativePipeWorker>,
    body: &[u8],
) -> Result<(), Box<dyn Error>> {
    request.receiver.clock.now = request.receiver.cutoff;
    let expired: Result<(), SessionFault> = request.receiver.check_receive_edge();
    let refused: bool = request_slot(request).is_err();
    let mut read: RetiringRequestRead<'_> = request
        .receiver
        .owner
        .worker
        .retiring_request_read()
        .ok_or("broker cannot retire its original read")?;
    let original: NativeCancel = native_fixture_result(read.retire())?;
    let uncollected: bool = read.is_pending();
    stop_with_native_read(
        request,
        body,
        NativeCancel::Requested,
        RetiredRead::Cancelled,
    )?;
    assert_eq!(expired, Err(SessionFault::Expired));
    assert!(refused && uncollected);
    assert_eq!(original, NativeCancel::Requested);
    Ok(())
}

/// Drive the actual service owner through Stop with its original uncollected ingress.
/// # Errors
/// Returns if the bounded native drain fails or the calling thread retains impersonation.
/// # Panics
/// Fails on premature worker settlement, admission, dispatch or wrong native retirement.
#[cfg(windows)]
fn stop_with_native_read(
    request: &mut InboundStatus<'_, '_, Reporter, ScriptClock<'_>, NativePipeWorker>,
    body: &[u8],
    original: NativeCancel,
    terminal: RetiredRead,
) -> Result<(), Box<dyn Error>> {
    let _stop_disposition = deliver(
        Some(request.receiver.clock.mailbox),
        request.receiver.stop,
        1,
        0,
    );
    let started: Instant = Instant::now();
    let mut poll: InvocationPoll = request.poll_broker();
    let admission_closed: bool = request.receiver.owner.worker.request_read().is_none();
    let late_fragment: Option<SessionFault> = request.complete_fragment(body).err();
    let undispatched: bool = request.receiver.last.is_none();
    let mut drain_held: bool = true;
    while let InvocationPoll::NativeDrain(wait) = poll {
        let owner: &mut ServiceInvocation<Reporter, NativePipeWorker> =
            &mut *request.receiver.owner;
        drain_held &= owner.owner.state()
            == Some(windows_platform::service::ServiceState::StopPending)
            && owner.permit.is_some()
            && owner.take_receipt().is_none();
        if started.elapsed() >= Duration::from_secs(2) {
            return native_fixture_result(Err(
                "broker Stop did not observe native read completion",
            ));
        }
        std::thread::sleep(wait);
        poll = request.poll_broker();
    }
    if poll != InvocationPoll::Finished {
        return native_fixture_result(Err(poll));
    }
    assert!(admission_closed && undispatched && drain_held);
    assert_eq!(late_fragment, Some(SessionFault::Closed));
    assert!(request.receiver.owner.permit.is_none());
    let retirement: &ReadRetirement = request.receiver.owner.worker.read_retirement();
    assert_eq!(retirement.cancellation(), Some(Ok(original)));
    assert_eq!(retirement.terminal(), Some(Ok(terminal)));
    assert!(request.receiver.last.is_none());
    native_fixture_result(ensure_unimpersonated())
}

/// Present a valid same-process reader from another pair to the reply consumer.
/// # Errors
/// Preserves setup, request, native operation or cleanup failure.
/// # Panics
/// Fails if a foreign pair gains admission or the transfer cancels its read.
#[cfg(windows)]
fn native_reply_refuses_foreign_reader() -> Result<(), Box<dyn Error>> {
    let (reader, writer): (LabPipeReader, LabOverlappedWriter) =
        native_fixture_result(open_lab_reply_pipe(24))?;
    let original: NativePipeRead = native_fixture_result(NativePipeRead::new(reader))?;
    let worker: NativePipeWorker = native_fixture_result(NativePipeWorker::new(writer))?;
    let (other, other_writer): (LabPipeReader, LabOverlappedWriter) =
        native_fixture_result(open_lab_reply_pipe(25))?;
    let mut foreign: NativePipeRead = native_fixture_result(NativePipeRead::new(other))?;
    foreign.check_reply_writer(&other_writer)?;

    let observed: Result<_, Box<dyn Error>> = with_broker_worker_reads::<24, 1, _, NativePipeWorker>(
        worker,
        move |owner, clock, stop| {
            let (mut session, mut client) = native_fixture_result(BrokerStatusSession::open(
                owner,
                clock,
                stop,
                LabRole::Engine,
            ))?;
            session.owner.worker.check_reply_reader(&original)?;
            let pending: PendingStatus<'_> = native_fixture_result(client.request())?;
            let reply: Reply = native_fixture_result(session.receive(pending.delivery()))?;
            let transfer = native_fixture_result(session.begin_reply(reply))?;
            let started: NativeReadStart = native_fixture_result(foreign.begin(16))?;
            let result: Result<bool, Box<dyn Error>> =
                receive_native_reply(transfer, pending, &mut foreign);
            let untouched: bool = matches!(foreign.poll(), Ok(NativeReadPoll::Pending));
            let cancellation: Result<NativeCancel, NativeReadError> = foreign.request_cancel();
            let terminal: Result<NativeReadPoll, Box<dyn Error>> =
                wait_for_native_read(&mut foreign);
            retire_pending_peer_read(&mut foreign);
            let refusal: Option<&LabPipeError> =
                result.as_ref().err().and_then(|error| error.downcast_ref());
            Ok((
                matches!(refusal, Some(LabPipeError::ForeignReplyPair)),
                started == NativeReadStart::Queued && untouched,
                matches!(
                    native_fixture_result(cancellation)?,
                    NativeCancel::Requested
                ),
                matches!(terminal?, NativeReadPoll::Cancelled) && !foreign.is_pending(),
                !session.connection.get(),
                session.remaining == super::FRAME_BUDGET - 1 && client.next == Some(2),
                matches!(
                    session.owner.worker.poll_write(),
                    Err(NativePipeWorkerError::Native(
                        crate::native_pipe_write::NativeWriteError::Idle
                    ))
                ),
            ))
        },
    )?;
    assert_eq!(observed?, (true, true, true, true, true, true, true));
    Ok(())
}

/// Supply a complete valid request from a real anonymous pipe client.
/// # Errors
/// Returns setup, native submission, write or thread-context failure.
/// # Panics
/// Fails if a read without a proven client context can deliver bytes or reopen.
#[cfg(windows)]
fn native_anonymous_request_refuses_before_dispatch() -> Result<(), Box<dyn Error>> {
    let pipe: LabIdentificationPipe = native_fixture_result(open_lab_identification_pipe(
        21,
        LabClientContext::Anonymous,
    ))?;
    let process_identity: PipeIdentity = native_fixture_result(own_process_identity())?;
    let mut read: IdentifiedRequestRead =
        native_fixture_result(IdentifiedRequestRead::new(pipe, process_identity))?;
    with_broker::<20, _>(|owner, clock, stop| {
        let (mut session, mut client): (
            BrokerStatusSession<'_, Reporter, ScriptClock<'_>>,
            EngineStatusClient,
        ) = native_fixture_result(BrokerStatusSession::open(
            owner,
            clock,
            stop,
            LabRole::Engine,
        ))?;
        let pending: PendingStatus<'_> = native_fixture_result(client.request())?;
        let delivery: Delivery<'_> = pending.delivery();
        let length: usize = delivery.bytes.len();
        let request: InboundStatus<'_, '_, Reporter, ScriptClock<'_>> =
            native_fixture_result(session.begin_receive(delivery.connection))?;
        let start: NativeReadStart = native_fixture_result(read.begin(length))?;
        if let Err(error) = native_fixture_result(read.write_from_client(delivery.bytes)) {
            retire_pending_identified_read(&mut read);
            return Err(error);
        }
        let observed: Result<Option<NativeReadPoll>, IdentifiedReadError> =
            wait_for_identified_read(|| read.poll());
        retire_pending_identified_read(&mut read);
        let reverted: Result<(), _> = ensure_unimpersonated();
        let retired: bool = matches!(read.begin(1), Err(IdentifiedReadError::Retired));
        drop(request);
        let late: Frame = encode_reply(pending.id, true);
        let refused: Result<bool, SessionFault> = pending.resolve(Reply {
            connection: std::rc::Rc::clone(&session.connection),
            frame: late,
            positive_revision: None,
        });
        assert_eq!(start, NativeReadStart::Queued);
        assert!(
            matches!(
                observed.as_ref(),
                Err(IdentifiedReadError::Identity {
                    cause: PipeIdentityError::Native {
                        operation: "OpenThreadToken",
                        code: ERROR_CANT_OPEN_ANONYMOUS,
                } | PipeIdentityError::AnonymousUser,
                    received,
                }) if *received == length
            ),
            "the complete anonymous request did not reach identity refusal: {:?}",
            observed.as_ref().err()
        );
        native_fixture_result(reverted)?;
        assert!(retired);
        assert_eq!(session.last, None);
        assert_eq!(refused, Err(SessionFault::Closed));
        Ok(())
    })?
}

/// Force an actual queued write under bounded same-process backpressure.
/// # Errors
/// Returns setup or unexpected submission, cancellation or completion results.
/// # Panics
/// Fails if a cancellation request releases the original operation early.
#[cfg(windows)]
fn native_reply_drains_pending_write() -> Result<(), Box<dyn Error>> {
    let (_held_reader, writer): (LabPipeReader, LabOverlappedWriter) =
        native_fixture_result(open_lab_reply_pipe(18))?;
    let worker: NativePipeWorker = native_fixture_result(NativePipeWorker::new(writer))?;
    with_broker_worker_reads::<19, 1, _, NativePipeWorker>(worker, |owner, clock, stop| {
        let bytes: [u8; MAX_FRAME_LEN] = [0; MAX_FRAME_LEN];
        for _ in 0..512 {
            native_fixture_result(owner.broker_status_edge(clock, stop, None))?;
            let start: NativeWriteStart = native_fixture_result(owner.worker.begin(&bytes))?;
            match native_fixture_result(owner.worker.poll_write())? {
                NativeWritePoll::Written(count) if count == bytes.len() => {}
                NativeWritePoll::Pending if start == NativeWriteStart::Queued => {
                    return stop_with_native_write(owner, clock, stop, &bytes);
                }
                observed => {
                    return Err(format!("unexpected backpressure completion: {observed:?}").into());
                }
            }
        }
        Err("native reply write did not queue under the bounded backpressure control".into())
    })?
}

/// Drive the original broker Stop owner while it retains a real submitted write.
/// # Errors
/// Returns any wrong wait or unconfirmed cleanup after two real seconds.
/// # Panics
/// Fails on premature acknowledgement, renewed admission or a wrong terminal fact.
#[cfg(windows)]
fn stop_with_native_write(
    owner: &mut ServiceInvocation<Reporter, NativePipeWorker>,
    clock: &mut ScriptClock<'_>,
    stop: &StopNotification,
    bytes: &[u8],
) -> Result<(), Box<dyn Error>> {
    let _stop_disposition = deliver(Some(clock.mailbox), stop, 1, 0);
    let began: Instant = Instant::now();
    let mut poll: InvocationPoll = owner.poll(clock, stop);
    let refusals: Option<NativePipeWorkerError> = owner.worker.begin(bytes).err();
    let mut held_until_terminal: bool = true;
    while let InvocationPoll::NativeDrain(wait) = poll {
        held_until_terminal &= owner.owner.state()
            == Some(windows_platform::service::ServiceState::StopPending)
            && owner.permit.is_some()
            && owner.take_receipt().is_none();
        if began.elapsed() >= Duration::from_secs(2) {
            let _diagnostic: std::io::Result<()> = writeln!(
                std::io::stderr().lock(),
                "broker Stop still holds the original native write"
            );
            return Err("broker Stop did not observe native write completion".into());
        }
        std::thread::sleep(wait);
        poll = owner.poll(clock, stop);
    }
    assert!(held_until_terminal);
    assert_eq!(poll, InvocationPoll::Finished);
    assert_eq!(refusals, Some(NativePipeWorkerError::Inactive));
    assert!(owner.permit.is_none());
    assert_eq!(
        owner.worker.cancellation(),
        Some(&Ok(NativeCancel::Requested))
    );
    assert_eq!(
        owner.worker.terminal(),
        Some(&Ok(NativeWritePoll::Cancelled))
    );
    Ok(())
}

/// Let the real receiver expire while an actual body read remains pending.
/// Observe native cancellation before challenging the closed receiver with late bytes.
/// # Errors
/// Returns native pipe, invocation, frame or cancellation observation failure.
/// # Panics
/// Fails if expiry retires pending native work early or admits late input.
#[cfg(windows)]
fn native_read_drains_after_expiry() -> Result<(), Box<dyn Error>> {
    let (reader, writer): (LabPipeReader, LabPipeWriter) =
        native_fixture_result(open_lab_pipe(16))?;
    let mut read: NativePipeRead = native_fixture_result(NativePipeRead::new(reader))?;
    with_broker_reads::<16, 2, _>(move |owner, clock, stop| {
        let cutoff: Instant = clock.now + Duration::from_secs(5);
        let remaining: Duration = Duration::from_millis(200);
        let before_cutoff: Instant = cutoff.checked_sub(remaining).ok_or("cutoff underflow")?;
        let (mut session, mut client): (
            BrokerStatusSession<'_, Reporter, ScriptClock<'_>>,
            EngineStatusClient,
        ) = native_fixture_result(BrokerStatusSession::open(
            owner,
            clock,
            stop,
            LabRole::Engine,
        ))?;
        let pending: PendingStatus<'_> = native_fixture_result(client.request())?;
        let delivery: Delivery<'_> = pending.delivery();
        let (header, body): (&[u8], &[u8]) = delivery.bytes.split_at(16);
        let mut request: InboundStatus<'_, '_, Reporter, ScriptClock<'_>> =
            native_fixture_result(session.begin_receive(delivery.connection))?;

        let header_start: NativeReadStart = native_fixture_result(read.begin(header.len()))?;
        let native_header: NativeReadBytes = write_completed_fragment(&mut read, &writer, header)?;
        assert!(matches!(header_start, NativeReadStart::Queued));
        assert_eq!(native_header.as_bytes(), header);
        assert!(
            native_fixture_result(request.complete_fragment(native_header.as_bytes()))?.is_none()
        );

        let body_start: NativeReadStart = native_fixture_result(read.begin(body.len()))?;
        let body_idle: bool = matches!(read.poll(), Ok(NativeReadPoll::Pending));
        request.receiver.clock.now = before_cutoff;
        let early: InvocationPoll = request.poll_broker();
        let open: bool = request.receiver.connection.get();
        request.receiver.clock.now = cutoff;
        let boundary: InvocationPoll = request.poll_broker();
        let retired: bool = request.assembly.is_none() && !request.receiver.connection.get();
        let broker_open: bool = !request.receiver.owner.admission_closed();
        let pending_at_cutoff: bool = read.is_pending();
        let cancellation: Result<NativeCancel, NativeReadError> = read.request_cancel();
        let pending_after_cancel: bool = read.is_pending();
        let completion: NativeReadPoll = wait_for_native_read(&mut read)?;

        assert!(matches!(body_start, NativeReadStart::Queued));
        assert!(body_idle);
        assert_eq!(early, InvocationPoll::Pending(remaining));
        assert!(open);
        assert_eq!(
            boundary,
            InvocationPoll::Pending(Duration::from_millis(800))
        );
        assert!(retired && broker_open);
        assert!(pending_at_cutoff && pending_after_cancel);
        assert!(matches!(
            native_fixture_result(cancellation)?,
            NativeCancel::Requested
        ));
        assert!(matches!(completion, NativeReadPoll::Cancelled));
        assert!(!read.is_pending());
        assert_eq!(
            request.complete_fragment(body).err(),
            Some(SessionFault::Closed)
        );
        Ok(())
    })?
}

/// Let the actual paired sender reach cutoff after an admitted header.
/// # Errors
/// Returns setup, authorized header transfer or terminal cancellation failure.
/// # Panics
/// Fails if the body is submitted, its native read is released early or an ID is refunded.
#[cfg(windows)]
fn native_request_refuses_body_at_cutoff() -> Result<(), Box<dyn Error>> {
    let pair: LabIdentificationPipe = native_fixture_result(open_lab_identification_pipe(
        23,
        LabClientContext::Identification,
    ))?;
    let identity: PipeIdentity = native_fixture_result(own_process_identity())?;
    let (_reader, writer): (LabPipeReader, LabOverlappedWriter) =
        native_fixture_result(open_lab_reply_pipe(0x100 + 23))?;
    let worker: NativePipeWorker =
        native_fixture_result(NativePipeWorker::with_request_read(writer, pair, identity))?;
    let observed: Result<_, Box<dyn Error>> =
        with_broker_worker_reads::<23, 1, _, NativePipeWorker>(worker, |owner, clock, stop| {
            let (mut session, mut client) = native_fixture_result(BrokerStatusSession::open(
                owner,
                clock,
                stop,
                LabRole::Engine,
            ))?;
            native_fixture_result(session.check_client_edge(&client.connection))?;
            let pending: PendingStatus<'_> = native_fixture_result(client.request())?;
            let mut pipe: BoundRequestPipe = BoundRequestPipe::new(&pending);
            let delivery: Delivery<'_> = pending.delivery();
            let (header, body): (&[u8], &[u8]) = delivery.bytes.split_at(16);
            let mut ingress = native_fixture_result(session.begin_receive(delivery.connection))?;
            // A second write of this size would still fit inside the frame. Its
            // refusal therefore exercises the spent read slot, even before poll.
            let (prefix, suffix): (&[u8], &[u8]) = header.split_at(8);
            let _prefix_start: NativeReadStart =
                native_fixture_result(request_slot(&mut ingress)?.begin(prefix.len()))?;
            let first: Result<(), RequestSubmissionError> = pipe.submit(&mut ingress, &pending);
            let repeated: bool = matches!(
                pipe.submit(&mut ingress, &pending),
                Err(RequestSubmissionError::Session(SessionFault::Frame(
                    FrameError::Length
                )))
            );
            let header_poll: Result<Option<NativeReadPoll>, Box<dyn Error>> =
                wait_for_identified_read(|| {
                    native_fixture_result(request_slot(&mut ingress)?.poll())
                });
            if let Err(error) = first {
                return match error {
                    RequestSubmissionError::Session(cause) => native_fixture_result(Err(cause)),
                    RequestSubmissionError::Pipe(cause) => native_fixture_result(Err(cause)),
                };
            }
            let Some(NativeReadPoll::Data(native_header)) = header_poll? else {
                return Err("authorized header reached no data".into());
            };
            let admitted_prefix: bool = native_header.as_bytes() == prefix
                && native_fixture_result(ingress.complete_fragment(native_header.as_bytes()))?
                    .is_none();
            let _suffix_start: NativeReadStart =
                native_fixture_result(request_slot(&mut ingress)?.begin(suffix.len()))?;
            let native_suffix: NativeReadBytes =
                write_identified_fragment(&mut pipe, &mut ingress, &pending)?;
            let admitted_header: bool = admitted_prefix
                && native_suffix.as_bytes() == suffix
                && native_fixture_result(ingress.complete_fragment(native_suffix.as_bytes()))?
                    .is_none();
            let body_start: NativeReadStart =
                native_fixture_result(request_slot(&mut ingress)?.begin(body.len()))?;
            ingress.receiver.clock.now = ingress.receiver.cutoff;
            let refusal: bool = matches!(
                pipe.submit(&mut ingress, &pending),
                Err(RequestSubmissionError::Session(SessionFault::Expired))
            );
            assert!(request_slot(&mut ingress).is_err());
            let mut read: RetiringRequestRead<'_> = ingress
                .receiver
                .owner
                .worker
                .retiring_request_read()
                .ok_or("broker cannot retire its expired read")?;
            let pending_at_refusal: bool = read.is_pending();
            let cancel: Result<NativeCancel, NativeReadError> = read.retire();
            let pending_after_cancel: bool = read.is_pending();
            let terminal: RetiredRead = wait_for_retired_read(|| read.poll_retirement())?;
            Ok((
                admitted_header,
                repeated,
                body_start == NativeReadStart::Queued,
                refusal,
                pending_at_refusal && pending_after_cancel,
                matches!(native_fixture_result(cancel)?, NativeCancel::Requested),
                terminal == RetiredRead::Cancelled,
                !read.is_pending(),
                !ingress.receiver.connection.get() && ingress.receiver.last.is_none(),
                ingress.receiver.remaining == super::FRAME_BUDGET - 1
                    && pending.client.next == Some(2)
                    && pipe.submitted == header.len(),
            ))
        })?;
    assert_eq!(
        observed?,
        (true, true, true, true, true, true, true, true, true, true)
    );
    Ok(())
}

/// Observe a retired session read while the broker itself remains Running.
/// # Errors
/// Returns the original native failure or a bounded wait refusal.
#[cfg(windows)]
fn wait_for_retired_read(
    mut poll: impl FnMut() -> Result<std::task::Poll<RetiredRead>, NativeReadError>,
) -> Result<RetiredRead, Box<dyn Error>> {
    let started: Instant = Instant::now();
    loop {
        if let std::task::Poll::Ready(terminal) = native_fixture_result(poll())? {
            return Ok(terminal);
        }
        if started.elapsed() >= Duration::from_secs(2) {
            return Err("retired session read did not complete".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// Exercise real native completion and the expiry/cancellation neighbor.
/// # Errors
/// Returns unexpected native pipe, invocation or exchange failure.
/// # Panics
/// Fails if actual completion custody or broker progress differs from the oracle.
#[cfg(windows)]
#[test]
#[ignore = "requires an explicitly admitted offline Windows pipe and read-only clock fixture"]
fn native_read_completes_or_drains_while_broker_polls() -> Result<(), Box<dyn Error>> {
    native_read_feeds_broker::<15, 2>(NativeIngressExit::Reply)?;
    native_read_feeds_broker::<25, 2>(NativeIngressExit::Stop)?;
    native_read_feeds_broker::<26, 2>(NativeIngressExit::ExpireThenStop)?;
    native_read_feeds_broker::<27, 2>(NativeIngressExit::CompleteThenStop)?;
    native_read_feeds_broker::<28, 1>(NativeIngressExit::StopBeforeAttach)?;
    native_read_feeds_broker::<29, 1>(NativeIngressExit::ExpireBeforeAttach)?;
    native_reply_refuses_foreign_reader()?;
    native_request_refuses_body_at_cutoff()?;
    native_read_drains_after_expiry()?;
    native_reply_drains_pending_write()?;
    native_anonymous_request_refuses_before_dispatch()
}

/// The original Stop runs actual worker cleanup even while a request is incomplete.
/// # Errors
/// Returns startup or receive refusal from the actual fixture.
/// # Panics
/// Fails if the lifecycle or diagnostic pump is skipped or late input is admitted.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn broker_poll_finishes_stop_during_a_partial_frame() -> Result<(), Box<dyn Error>> {
    let actual: Result<_, SessionFault> = with_broker::<14, _>(|owner, clock, stop| {
        let (mut session, mut client): (
            BrokerStatusSession<'_, Reporter, ScriptClock<'_>>,
            EngineStatusClient,
        ) = BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
        let pending = client.request()?;
        let delivery: Delivery<'_> = pending.delivery();
        let (header, body): (&[u8], &[u8]) = delivery.bytes.split_at(16);
        let mut request = session.begin_receive(delivery.connection)?;
        let partial: bool = request.complete_fragment(header)?.is_none();
        let attempts: u64 = request
            .receiver
            .owner
            .diagnostics
            .snapshot()
            .required()
            .health()
            .consumer_attempts()
            .value();
        let wake: bool = deliver(Some(&MAILBOXES[14]), stop, 1, 0).wake;
        let poll: InvocationPoll = request.poll_broker();
        let after: u64 = request
            .receiver
            .owner
            .diagnostics
            .snapshot()
            .required()
            .health()
            .consumer_attempts()
            .value();
        let late: Option<SessionFault> = request.complete_fragment(body).err();
        Ok((partial, wake, poll, after.checked_sub(attempts), late))
    })?;
    assert_eq!(
        actual,
        Ok((
            true,
            true,
            InvocationPoll::Finished,
            Some(1),
            Some(SessionFault::Closed)
        ))
    );
    Ok(())
}

/// Discard abandoned or unframeable input. The same connection cannot reframe its tail.
/// # Errors
/// Returns unexpected fixture startup, cleanup or prefix refusal.
/// # Panics
/// Fails if abandonment or an invalid extent permits the same transport to submit another frame.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn abandoned_frame_retires_the_connection() -> Result<(), Box<dyn Error>> {
    let actual: Result<_, SessionFault> = with_broker::<10, _>(|owner, clock, stop| {
        let (mut session, mut client) =
            BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
        let pending = client.request()?;
        let delivery: Delivery<'_> = pending.delivery();
        let (prefix, _rest): (&[u8], &[u8]) = delivery.bytes.split_at(10);
        {
            let mut request = session.begin_receive(delivery.connection)?;
            assert!(request.complete_fragment(prefix)?.is_none());
        }
        Ok(session.receive(delivery).err())
    })?;
    assert_eq!(actual, Ok(Some(SessionFault::Closed)));

    let malformed: Result<_, SessionFault> = with_broker::<11, _>(|owner, clock, stop| {
        let (mut session, mut client) =
            BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
        let pending = client.request()?;
        let delivery: Delivery<'_> = pending.delivery();
        let (head, _rest): (&[u8], &[u8]) = delivery.bytes.split_at(16);
        let extent: Option<SessionFault> = {
            let mut request = session.begin_receive(delivery.connection)?;
            assert!(request.complete_fragment(head)?.is_none());
            request.complete_fragment(b"{}x").err()
        };
        Ok((extent, session.receive(delivery).err()))
    })?;
    assert_eq!(
        malformed,
        Ok((
            Some(SessionFault::Frame(FrameError::Length)),
            Some(SessionFault::Closed),
        )),
    );
    Ok(())
}

/// Correlation requires both a correct in-band ID and the originally composed connection.
/// # Errors
/// Returns an unexpected broker startup, cleanup or exchange failure.
/// # Panics
/// Fails if another connection, old ID, late result or reopened session is accepted.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn private_status_refuses_foreign_mismatched_and_late_replies() -> Result<(), Box<dyn Error>> {
    let actual = with_broker::<1, _>(|owner, clock, stop| {
        with_broker::<2, _>(|other, other_clock, other_stop| {
            let (mut session, mut sender) =
                BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
            let (mut other_session, mut other_sender) =
                BrokerStatusSession::open(other, other_clock, other_stop, LabRole::Engine)?;
            let wrong_receiver: Result<BrokerAvailability, SessionFault> =
                sender.read_availability(&mut other_session);
            let unspent: bool =
                sender.next == Some(1) && other_session.remaining == super::FRAME_BUDGET;
            let pending = sender.request()?;
            let local: Reply = session.receive(pending.delivery())?;
            let foreign: Reply = {
                let other_pending: PendingStatus<'_> = other_sender.request()?;
                other_session.receive(other_pending.delivery())?
            };
            let foreign_result: Result<bool, SessionFault> = pending.resolve(foreign);
            let other_live: BrokerAvailability =
                other_sender.read_availability(&mut other_session)?;
            let earlier: [u8; 18] = [1, 0, 1, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, b'{', b'}'];
            let decreasing: Option<SessionFault> = other_session
                .receive(Delivery {
                    connection: &other_sender.connection,
                    bytes: &earlier,
                })
                .err();
            let wrong_id: Result<bool, SessionFault> = sender.request()?.resolve(local);
            let late_pending = sender.request()?;
            let late_reply: Reply = session.receive(late_pending.delivery())?;
            drop(session);
            let late: Result<bool, SessionFault> = late_pending.resolve(late_reply);
            let reopened: Option<SessionFault> =
                BrokerStatusSession::open(owner, clock, stop, LabRole::Engine).err();
            Ok::<_, SessionFault>((
                wrong_receiver,
                unspent,
                foreign_result,
                wrong_id,
                late,
                reopened,
                other_live,
                decreasing,
            ))
        })
    })??;
    assert_eq!(
        actual,
        Ok((
            Err(SessionFault::ForeignConnection),
            true,
            Err(SessionFault::ForeignConnection),
            Err(SessionFault::Frame(FrameError::Correlation)),
            Err(SessionFault::Closed),
            Some(SessionFault::Closed),
            BrokerAvailability::Available,
            Some(SessionFault::Replayed),
        )),
    );
    Ok(())
}

/// Named lifecycle cases whose independent observations are checked after Stop.
#[derive(Clone, Copy)]
enum Crossing {
    /// A clock change revokes the old sample at the final decision.
    TimeChange,
    /// Stop revokes the connection before publication.
    Stop,
    /// A healthy first observation crosses the session cutoff.
    Deadline,
    /// The same crossing occurs when no current sample is available.
    DeadlineWithoutSample,
}

/// The reply, the injected edge and the remaining connection state are independent observations.
#[derive(Debug, Eq, PartialEq)]
struct DecisionObservation {
    /// Admitted observation immediately preceding the injected transition.
    before: BrokerAvailability,
    /// Actual consumed decision or transport refusal.
    response: Result<BrokerAvailability, SessionFault>,
    /// Whether the scripted transition reached the second clock observation.
    fired: bool,
    /// Whether the session remained admitted after the transition.
    open: bool,
}

/// Supply two distinct admission observations around the selected control.
/// # Errors
/// Returns an unexpected broker startup, cleanup or exchange failure.
fn crossing<const N: usize>(
    case: Crossing,
) -> Result<Result<DecisionObservation, SessionFault>, Box<dyn Error>> {
    with_broker::<N, _>(|owner, clock, stop| {
        let (mut session, mut client) =
            BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
        let admitted: Instant = session.clock.now;
        let before: BrokerAvailability = client.read_availability(&mut session)?;
        if matches!(case, Crossing::DeadlineWithoutSample) {
            let _disposition = deliver(Some(session.clock.mailbox), stop, 16, 0);
        }
        let action: DecisionAction = match case {
            Crossing::TimeChange => DecisionAction::Deliver(16),
            Crossing::Stop => DecisionAction::Deliver(1),
            Crossing::Deadline | Crossing::DeadlineWithoutSample => {
                session.clock.now = admitted + Duration::from_nanos(4_999_999_999);
                DecisionAction::Advance(admitted + Duration::from_secs(5))
            }
        };
        // The added client admission is quiet. Trigger at the broker decision,
        // after the receiver has also admitted the first complete fragment.
        session.clock.final_action = Some((2, action));
        let response: Result<BrokerAvailability, SessionFault> =
            client.read_availability(&mut session);
        Ok(DecisionObservation {
            before,
            response,
            fired: session.clock.reached,
            open: session.connection.get(),
        })
    })
}

/// The final edge rejects Stop or cutoff and reports false for a valid withdrawn sample.
/// # Errors
/// Returns an unexpected broker startup, cleanup or exchange failure.
/// # Panics
/// Fails if first-edge admission can publish after the decisive later transition.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn private_status_rechecks_the_final_transition_and_cutoff() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        crossing::<3>(Crossing::TimeChange)?,
        Ok(DecisionObservation {
            before: BrokerAvailability::Available,
            response: Ok(BrokerAvailability::Unavailable),
            fired: true,
            open: true
        })
    );
    assert_eq!(
        crossing::<4>(Crossing::Stop)?,
        Ok(DecisionObservation {
            before: BrokerAvailability::Available,
            response: Err(SessionFault::Closed),
            fired: true,
            open: false
        })
    );
    let expired: Result<_, SessionFault> = Ok(DecisionObservation {
        before: BrokerAvailability::Available,
        response: Err(SessionFault::Expired),
        fired: true,
        open: false,
    });
    assert_eq!(crossing::<5>(Crossing::Deadline)?, expired);
    assert_eq!(crossing::<6>(Crossing::DeadlineWithoutSample)?, expired);
    Ok(())
}

/// An expired client must refuse before consuming an ID or delivering a frame.
/// # Errors
/// Preserves a fixture startup, shutdown or unexpected healthy exchange failure.
/// # Panics
/// Fails if live or expired exchanges produce the wrong observation or accounting.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn private_status_client_refuses_send_at_original_cutoff() -> Result<(), Box<dyn Error>> {
    let observed: Result<_, SessionFault> = with_broker::<21, _>(|owner, clock, stop| {
        let (mut session, mut client) =
            BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
        session.clock.now = session
            .cutoff
            .checked_sub(Duration::from_nanos(1))
            .ok_or(SessionFault::Expired)?;
        let live: BrokerAvailability = client.read_availability(&mut session)?;
        let before: (Option<u64>, u8, Option<super::RequestId>) =
            (client.next, session.remaining, session.last);
        session.clock.now = session.cutoff;
        let refused: Result<BrokerAvailability, SessionFault> =
            client.read_availability(&mut session);
        Ok((
            live,
            refused,
            (client.next, session.remaining, session.last) == before,
            session.connection.get(),
        ))
    })?;
    assert_eq!(
        observed,
        Ok((
            BrokerAvailability::Available,
            Err(SessionFault::Expired),
            true,
            false
        ))
    );
    Ok(())
}

/// A broker reply admitted before expiry cannot be newly published at expiry.
/// # Errors
/// Preserves a fixture startup or shutdown failure.
/// # Panics
/// Fails if the completed broker dispatch or the later client refusal is lost.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn private_status_client_rechecks_cutoff_after_broker_reply() -> Result<(), Box<dyn Error>> {
    let observed: Result<_, SessionFault> = with_broker::<22, _>(|owner, clock, stop| {
        let (mut session, mut client) =
            BrokerStatusSession::open(owner, clock, stop, LabRole::Engine)?;
        let exchange_cutoff: Instant = session.cutoff;
        // Send admission, request receipt and broker decision all see the
        // earlier time. Only the receiving client's final observation expires.
        session.clock.final_action = Some((3, DecisionAction::Advance(exchange_cutoff)));
        let response: Result<BrokerAvailability, SessionFault> =
            client.read_availability(&mut session);
        Ok((
            response,
            session.clock.reached,
            client.next,
            session.remaining,
            session.last,
            session.connection.get(),
        ))
    })?;
    assert_eq!(
        observed,
        Ok((
            Err(SessionFault::Expired),
            true,
            Some(2),
            super::FRAME_BUDGET - 1,
            super::RequestId::new(1).ok(),
            false
        ))
    );
    Ok(())
}

/// Drive the original accepted pipe into the real availability receiver.
/// # Errors
/// Returns native setup, wake, lifecycle or reversion observation failure.
/// # Panics
/// Panics if a selected exact history changes availability or cleanup.
#[cfg(windows)]
#[test]
#[ignore = "requires the admitted offline Windows first-read fixture supervisor"]
fn native_first_read_requires_attribution_and_original_cutoff() -> Result<(), Box<dyn Error>> {
    use super::NativeStatusOutcome;
    use crate::private_clock_wire::{RequestId, decode_reply};

    let id: RequestId = native_fixture_result(RequestId::new(42))?;
    let outcome: Option<NativeStatusOutcome> =
        observed_first_status::<30>(LabClientContext::Identification, FirstStatusCase::Full)?;
    let Some(NativeStatusOutcome::Reply(frame)) = outcome else {
        return Err("identified current first read did not produce its status reply".into());
    };
    assert_eq!(decode_reply(frame.as_bytes(), id), Ok(true));
    assert!(matches!(
        observed_first_status::<31>(LabClientContext::Anonymous, FirstStatusCase::Full)?,
        Some(NativeStatusOutcome::Identity(
            IdentifiedReadError::Identity {
                cause: PipeIdentityError::Native {
                    operation: "OpenThreadToken",
                    code: ERROR_CANT_OPEN_ANONYMOUS,
                } | PipeIdentityError::AnonymousUser,
                received: 18,
            }
        ))
    ));
    assert!(matches!(
        observed_first_status::<32>(LabClientContext::Identification, FirstStatusCase::Partial)?,
        Some(NativeStatusOutcome::Receive(SessionFault::Frame(
            FrameError::Length
        )))
    ));
    assert!(
        observed_first_status::<33>(LabClientContext::Identification, FirstStatusCase::Cutoff,)?
            .is_none()
    );
    assert!(
        observed_first_status::<34>(LabClientContext::Identification, FirstStatusCase::Stopped,)?
            .is_none()
    );
    assert!(
        observed_first_status::<35>(
            LabClientContext::Identification,
            FirstStatusCase::CutoffAfterAttribution,
        )?
        .is_none()
    );
    assert!(
        observed_first_status::<36>(
            LabClientContext::Identification,
            FirstStatusCase::StoppedAfterAttribution,
        )?
        .is_none()
    );
    Ok(())
}

/// Independent fixture input or control change at completion.
#[cfg(windows)]
#[derive(Clone, Copy)]
enum FirstStatusCase {
    /// Complete request and a live context just before the original cutoff.
    Full,
    /// Original valid envelope missing its final byte.
    Partial,
    /// The original cutoff is reached as native collection finishes.
    Cutoff,
    /// Stop becomes visible as native collection finishes.
    Stopped,
    /// The same cutoff is reached after the capture and reversion return.
    CutoffAfterAttribution,
    /// Stop becomes visible after the capture and reversion return.
    StoppedAfterAttribution,
}

/// Submit a manual request into the actual Running worker on its original pipe.
/// # Errors
/// Returns the original native or service failure after bounded cleanup.
/// # Panics
/// Panics if a wake does not belong to an outstanding operation or the scripted
/// time change was never consumed.
#[cfg(windows)]
fn observed_first_status<const N: usize>(
    context: LabClientContext,
    scenario: FirstStatusCase,
) -> Result<Option<super::NativeStatusOutcome>, Box<dyn Error>> {
    use crate::{
        broker_endpoint::BrokerEndpointWorker,
        native_broker_endpoint::NativeBrokerEndpoint,
        native_broker_wait::{BrokerControlWake, BrokerWait, WakeSource},
        native_pipe_pair::{open_first_read_writer, prepare_first_read_fixture},
    };

    const REQUEST: [u8; 18] = [1, 0, 1, 0, 2, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, b'{', b'}'];
    let case: u32 = u32::try_from(N)?;
    let expected: PipeIdentity = own_process_identity()?;
    let acquire = move || -> Result<NativeBrokerEndpoint, NativeError> {
        prepare_first_read_fixture(case, Some(expected))
    };
    let outcome: Result<Option<super::NativeStatusOutcome>, Box<dyn Error>> =
        with_broker_worker_reads::<N, 1, _, _>(
            BrokerEndpointWorker::new(acquire),
            |owner, clock, stop| {
                let cutoff: Instant = owner.connection_cutoff.ok_or("no original accept cutoff")?;
                let writer: LabPipeWriter =
                    native_fixture_result(open_first_read_writer(case, context))?;
                let control: BrokerControlWake = BrokerControlWake::new()?;
                let mut waiter: BrokerWait = BrokerWait::new();
                assert_eq!(
                    waiter.observe(
                        &control,
                        Duration::from_secs(2),
                        owner.worker.completion_wake()
                    )?,
                    WakeSource::Completion
                );
                let _: InvocationPoll = owner.poll(clock, stop);
                let request: &[u8] = if matches!(scenario, FirstStatusCase::Partial) {
                    &REQUEST[..17]
                } else {
                    &REQUEST
                };
                native_fixture_result(writer.write(request))?;
                assert_eq!(
                    waiter.observe(
                        &control,
                        Duration::from_secs(2),
                        owner.worker.completion_wake()
                    )?,
                    WakeSource::Completion
                );
                // Keep the original time for the Running sampler's due decision.
                // The worker's before-read edge comes next, then its post-collection
                // and post-attribution checks. Sampling takes no extra observation.
                clock.final_action = Some(match scenario {
                    FirstStatusCase::Cutoff => (2, DecisionAction::Advance(cutoff)),
                    FirstStatusCase::Stopped => (2, DecisionAction::Deliver(1)),
                    FirstStatusCase::CutoffAfterAttribution => (3, DecisionAction::Advance(cutoff)),
                    FirstStatusCase::StoppedAfterAttribution => (3, DecisionAction::Deliver(1)),
                    FirstStatusCase::Full | FirstStatusCase::Partial => (
                        1,
                        DecisionAction::Advance(
                            cutoff
                                .checked_sub(Duration::from_nanos(1))
                                .ok_or("first-read cutoff underflow")?,
                        ),
                    ),
                });
                let expects_stop: bool = matches!(
                    scenario,
                    FirstStatusCase::Stopped | FirstStatusCase::StoppedAfterAttribution
                );
                let _: InvocationPoll = owner.poll(clock, stop);
                assert!(clock.reached);
                assert_eq!(
                    owner.native_status.capture_observed,
                    !matches!(scenario, FirstStatusCase::Cutoff | FirstStatusCase::Stopped)
                );
                assert_eq!(owner.connection_cutoff, Some(cutoff));
                assert_eq!(owner.outstanding_work(), u32::from(!expects_stop));
                native_fixture_result(ensure_unimpersonated())?;
                Ok(owner.native_status.outcome.take())
            },
        )?;
    outcome
}

/// Receive the broker's actual first reply on the client that sent the request.
/// # Errors
/// Returns a setup, native progress, byte comparison or cleanup failure.
/// # Panics
/// Fails if the broker helper observes a startup, sampling or terminal mismatch.
#[cfg(windows)]
#[test]
#[ignore = "requires the admitted offline Windows first-exchange fixture supervisor"]
fn native_first_read_returns_reply_on_the_original_connection() -> Result<(), Box<dyn Error>> {
    observe_native_first_reply::<37>(None)
}

/// Expire the original connection after reply construction and before native submission.
/// # Errors
/// Returns a setup, unexpected reply transfer or cleanup failure.
#[cfg(windows)]
#[test]
#[ignore = "requires the admitted offline Windows first-exchange fixture supervisor"]
fn native_first_read_refuses_reply_at_submission_cutoff() -> Result<(), Box<dyn Error>> {
    observe_native_first_reply::<41>(Some(FirstReplyRefusal::Cutoff))
}

/// Retire the original positive revision after encoding and before the native write.
/// # Errors
/// Returns a setup, unexpected reply transfer or cleanup failure.
#[cfg(windows)]
#[test]
#[ignore = "requires the admitted offline Windows first-exchange fixture supervisor"]
fn native_first_read_refuses_reply_after_revision_changes() -> Result<(), Box<dyn Error>> {
    observe_native_first_reply::<42>(Some(FirstReplyRefusal::Revision))
}

/// The independently scripted cause of a final positive reply refusal.
#[cfg(windows)]
#[derive(Clone, Copy)]
enum FirstReplyRefusal {
    /// The original connection cutoff is reached.
    Cutoff,
    /// The original awake revision retires, without closing the mailbox or pipe.
    Revision,
}

/// Observe the original client read before Stop and again after pipe retirement.
/// # Errors
/// Returns a refusal at setup, reply observation or cleanup.
#[cfg(windows)]
fn observe_native_first_reply<const CASE: usize>(
    refusal: Option<FirstReplyRefusal>,
) -> Result<(), Box<dyn Error>> {
    use std::num::NonZeroU32;
    use windows_platform_native::PeerProcess;

    use crate::{
        broker_endpoint::BrokerEndpointWorker,
        host::{PrivateStatusIssue, private_clock_status::NativeStatusOutcome},
        native_broker_endpoint::NativeBrokerEndpoint,
        native_pipe_identity::RetainedFixtureClient,
        native_pipe_pair::{
            LabRequestWriter, open_first_exchange_client, prepare_first_exchange_fixture,
        },
    };

    const WAIT: Duration = Duration::from_secs(2);
    const REQUEST: &[u8] = b"\x01\x00\x01\x00\x02\x00\x00\x00\x2a\x00\x00\x00\x00\x00\x00\x00{}";
    const REPLY: &[u8] = b"\x01\x00\x02\x00\x12\x00\x00\x00\x2a\x00\x00\x00\x00\x00\x00\x00\
                           {\"available\":true}";
    let pid: NonZeroU32 = NonZeroU32::new(std::process::id()).ok_or("fixture PID was zero")?;
    let peer: RetainedFixtureClient = RetainedFixtureClient::new(PeerProcess::open(pid)?)?;
    let pipe_case: u32 = u32::try_from(CASE)?;
    let acquire = move || -> Result<NativeBrokerEndpoint, NativeError> {
        prepare_first_exchange_fixture(pipe_case, peer)
    };
    with_broker_worker_reads::<CASE, 1, _, _>(
        BrokerEndpointWorker::new(acquire),
        |owner, clock, stop| {
            let cutoff: Instant = owner.connection_cutoff.ok_or("no original accept cutoff")?;
            let (client_read, mut client_write): (LabPipeReader, LabRequestWriter) =
                native_fixture_result(open_first_exchange_client(pipe_case))?;
            let mut reader: NativePipeRead =
                native_fixture_result(NativePipeRead::new(client_read))?;
            let control: BrokerControlWake = BrokerControlWake::new()?;
            let mut waiter: BrokerWait = BrokerWait::new();
            if waiter.observe(&control, WAIT, owner.worker.completion_wake())?
                != WakeSource::Completion
                || !matches!(owner.poll(clock, stop), InvocationPoll::Pending(_))
                || owner.outstanding_work() != 1
                || owner.native_status.reply_written
            {
                return Err("the original accept did not advance to its owned first read".into());
            }
            let observed: Result<(), Box<dyn Error>> = (|| {
                if native_fixture_result(reader.begin(MAX_FRAME_LEN))? != NativeReadStart::Queued {
                    return Err("the broker replied before its client sent the request".into());
                }
                native_fixture_result(client_write.write(REQUEST))?;
                if waiter.observe(&control, WAIT, owner.worker.completion_wake())?
                    != WakeSource::Completion
                {
                    return Err("the original broker request read did not wake".into());
                }

                if let Some(refusal) = refusal {
                    observe_prepared_reply_refusal(owner, clock, stop, cutoff, refusal)?;
                } else {
                    wait_for_first_reply_write(
                        owner,
                        clock,
                        stop,
                        &control,
                        &mut waiter,
                        Instant::now() + WAIT,
                    )?;
                    observe_first_reply_bytes(&mut reader, REPLY, wait_for_native_read)?;
                    native_fixture_result(ensure_unimpersonated())?;
                    if clock.now >= cutoff {
                        return Err("the valid reply completed outside its original cutoff".into());
                    }
                    if native_fixture_result(reader.begin(MAX_FRAME_LEN))?
                        != NativeReadStart::Queued
                        || !matches!(owner.poll(clock, stop), InvocationPoll::Pending(_))
                    {
                        return Err(
                            "the settled exchange produced a second reply or operation".into()
                        );
                    }
                }
                native_fixture_result(ensure_unimpersonated())?;
                if !owner.native_status.capture_observed
                    || !matches!(&owner.native_status.outcome,
                        Some(NativeStatusOutcome::Reply(frame)) if frame.as_bytes() == REPLY)
                    || owner.private_status_issue != PrivateStatusIssue::Issued
                    || owner.connection_cutoff != Some(cutoff)
                    || owner.outstanding_work() != 1
                    || owner.worker.completion_wake().is_some()
                {
                    return Err("the admitted reply lost its original Running custody".into());
                }
                stop_after_pending_peer_read(owner, clock, stop, &mut reader)
            })();
            if observed.is_err() {
                retire_pending_peer_read(&mut reader);
            }
            observed
        },
    )?
}

/// Drive the existing write edge after the encoded positive decision is in custody.
/// # Errors
/// Fails if the selected control misses its edge or a write is confirmed.
#[cfg(windows)]
fn observe_prepared_reply_refusal<P: Worker>(
    owner: &mut ServiceInvocation<Reporter, P>,
    clock: &mut ScriptClock<'_>,
    stop: &StopNotification,
    cutoff: Instant,
    refusal: FirstReplyRefusal,
) -> Result<(), Box<dyn Error>> {
    let original: ClockRevision<'static> = clock
        .mailbox
        .revision()
        .ok_or("the original awake revision was unavailable")?;
    let action: DecisionAction = match refusal {
        FirstReplyRefusal::Cutoff => DecisionAction::Advance(cutoff),
        FirstReplyRefusal::Revision => DecisionAction::Deliver(0x10),
    };
    // Read attribution, receiver admission and the original preflight take
    // eleven clock observations before the prepared write's final guard.
    clock.final_action = Some((11, action));
    if !matches!(owner.poll(clock, stop), InvocationPoll::Pending(_))
        || !clock.reached
        || owner.native_status.reply_written
    {
        return Err("the prepared reply did not reach submission refusal".into());
    }
    match refusal {
        FirstReplyRefusal::Cutoff if clock.now == cutoff => Ok(()),
        FirstReplyRefusal::Revision
            if clock.now < cutoff
                && !stop.stop_requested()
                && !original.admits()
                && clock
                    .mailbox
                    .revision()
                    .is_some_and(|current: ClockRevision<'_>| {
                        current.number().get() == 2 && current.admits()
                    }) =>
        {
            Ok(())
        }
        _ => Err("the selected final refusal did not preserve its independent observations".into()),
    }
}

/// Require the original read to remain pending until a clean Stop disconnects it.
/// # Errors
/// Returns unexpected bytes, a native observation failure or unconfirmed retirement.
#[cfg(windows)]
fn stop_after_pending_peer_read<P: Worker>(
    owner: &mut ServiceInvocation<Reporter, P>,
    clock: &mut ScriptClock<'_>,
    stop: &StopNotification,
    reader: &mut NativePipeRead,
) -> Result<(), Box<dyn Error>> {
    if !matches!(
        native_fixture_result(reader.poll())?,
        NativeReadPoll::Pending
    ) {
        return Err("the original client received unadmitted reply bytes".into());
    }
    let _stop_disposition: ControlDisposition = deliver(Some(clock.mailbox), stop, 1, 0);
    if owner.poll(clock, stop) != InvocationPoll::Finished || owner.outstanding_work() != 0 {
        return Err("Stop did not retire the settled broker connection".into());
    }
    if !matches!(wait_for_native_read(reader)?, NativeReadPoll::Disconnected) {
        return Err("the original client saw extra bytes or no disconnect on Stop".into());
    }
    Ok(())
}

/// Give the original broker owner bounded turns to collect its native write.
/// # Errors
/// Refuses premature service retirement, a wrong wake or unconfirmed completion.
#[cfg(windows)]
fn wait_for_first_reply_write<P: Worker>(
    owner: &mut ServiceInvocation<Reporter, P>,
    clock: &mut ScriptClock<'_>,
    stop: &StopNotification,
    control: &BrokerControlWake,
    waiter: &mut BrokerWait,
    deadline: Instant,
) -> Result<(), Box<dyn Error>> {
    loop {
        if Instant::now() >= deadline {
            return Err("the broker never confirmed the original reply write".into());
        }
        if !matches!(owner.poll(clock, stop), InvocationPoll::Pending(_)) {
            return Err("the broker stopped before confirming the reply".into());
        }
        let remaining: Duration = deadline
            .checked_duration_since(Instant::now())
            .filter(|left: &Duration| !left.is_zero())
            .ok_or("the broker never confirmed the original reply write")?;
        if owner.native_status.reply_written {
            return Ok(());
        }
        if let Some(wake) = owner.worker.completion_wake() {
            if waiter.observe(control, remaining, Some(wake))? != WakeSource::Completion {
                return Err("the original broker reply completion did not wake".into());
            }
        } else {
            std::thread::sleep(Duration::from_millis(1).min(remaining));
        }
    }
}

/// Compare bounded native read completions to the exact wire bytes.
/// # Errors
/// Refuses a missing, empty, mismatched or excess reply prefix.
#[cfg(windows)]
fn observe_first_reply_bytes(
    reader: &mut NativePipeRead,
    mut remaining: &[u8],
    mut observe: impl FnMut(&mut NativePipeRead) -> Result<NativeReadPoll, Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    loop {
        let NativeReadPoll::Data(completed) = observe(reader)? else {
            return Err("the original client did not receive reply bytes".into());
        };
        let bytes: &[u8] = completed.as_bytes();
        if bytes.is_empty() {
            return Err("the original client received an empty reply fragment".into());
        }
        remaining = remaining
            .strip_prefix(bytes)
            .ok_or("the original client received unexpected reply bytes")?;
        if remaining.is_empty() {
            return Ok(());
        }
        let _: NativeReadStart = native_fixture_result(reader.begin(MAX_FRAME_LEN))?;
    }
}

#[cfg(windows)]
mod distinct_peer {
    //! Run one actual first exchange with the independently retained child.

    use std::{
        cell::{Cell, RefCell},
        env,
        ffi::OsString,
        io::{self, Read as _, Write as _},
        num::NonZeroU32,
        os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle, RawHandle},
        path::PathBuf,
        process::{self, Child, ChildStderr, Command, ExitStatus, Stdio},
        sync::mpsc,
        thread,
    };

    use windows_platform_native::PeerProcess;
    use windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND;

    use super::{
        Duration, Error, GETTERS, IdentifiedReadError, Instant, InvocationPoll, LabPipeError,
        LabPipeReader, MAX_FRAME_LEN, NativePipeRead, NativeReadPoll, NativeReadStart, SeqCst,
        Worker, deliver, ensure_unimpersonated, native_fixture_result, observe_first_reply_bytes,
        retire_pending_peer_read, wait_for_first_reply_write, with_broker_worker_reads,
    };
    use crate::{
        NativeError,
        broker_endpoint::BrokerEndpointWorker,
        host::{PrivateStatusIssue, private_clock_status::NativeStatusOutcome},
        native_broker_endpoint::NativeBrokerEndpoint,
        native_broker_wait::{BrokerControlWake, BrokerWait, WakeSource},
        native_peer_process::same_kernel_object,
        native_pipe_identity::RetainedFixtureClient,
        native_pipe_pair::{
            LabOverlappedWriter, LabRequestWriter, check_fixture_peer_before_cutoff,
            check_supervised_first_exchange_peer, open_first_exchange_client, open_lab_reply_pipe,
            open_supervised_first_exchange_client, peek_fixture_child_stderr,
            prepare_first_exchange_fixture,
        },
        native_pipe_write::{NativePipeWrite, NativeWritePoll, confirm_fixture_write},
    };

    /// The only exact libtest function the supervisor spawns.
    const CHILD_TEST: &str =
        "host::private_clock_status::tests::distinct_peer::run_broker_first_exchange_child";
    /// Private process selector for the two fixed inert child roles.
    const CHILD_ROLE: &str = "NEVROTICK_BROKER_LAB_CHILD";
    /// Supervisor PID used to locate and check the fixed local pipe.
    const PARENT_PID: &str = "NEVROTICK_BROKER_LAB_PARENT";
    /// Unique pipe and broker clock slot for the selected child.
    const CHILD_CASE: u32 = 38;
    /// Unique pipe and broker clock slot for the refused connector.
    const FOREIGN_CASE: u32 = 39;
    /// Unique pipe slot for the deterministic submission control.
    const GUARD_CASE: u32 = 40;
    /// Allowance before the idle child acknowledges its private hold.
    const SETUP_WAIT: Duration = Duration::from_secs(5);
    /// Limit on the isolated child exchange or private hold.
    const CHILD_WAIT: Duration = Duration::from_secs(12);
    /// Separate allowance to observe original resources retire.
    const RETIRE_WAIT: Duration = Duration::from_secs(2);
    /// Maximum pause between bounded process and pipe observations.
    const POLL: Duration = Duration::from_millis(1);
    /// The sole private marker on the retained child stderr pipe.
    const READY: u8 = b'R';
    /// Independent literal availability request with correlation value 42.
    const REQUEST: &[u8] = b"\x01\x00\x01\x00\x02\x00\x00\x00\x2a\x00\x00\x00\x00\x00\x00\x00{}";
    /// Independent literal reply for the original available status and correlation.
    const REPLY: &[u8] = b"\x01\x00\x02\x00\x12\x00\x00\x00\x2a\x00\x00\x00\x00\x00\x00\x00\
                           {\"available\":true}";

    /// Preserve the original fixture failure without admitting a second attempt.
    type Check<T> = Result<T, Box<dyn Error>>;

    /// The original spawn handle owns recovery if a test exits early.
    struct FixtureChild(Child);

    impl FixtureChild {
        /// Spawn only the exact ignored role in the current admitted test image.
        /// # Errors
        /// Preserve executable discovery or process creation failure.
        fn spawn(role: &str) -> Check<Self> {
            let executable: PathBuf = env::current_exe()?;
            let child: Child = Command::new(executable)
                .args([
                    "--exact",
                    CHILD_TEST,
                    "--ignored",
                    "--nocapture",
                    "--test-threads=1",
                ])
                .env_clear()
                .env(CHILD_ROLE, role)
                .env(PARENT_PID, process::id().to_string())
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .spawn()?;
            Ok(Self(child))
        }

        /// Compare the limited process query to the independently held spawn object.
        /// # Errors
        /// Refuse a zero PID, the parent process or different native process object.
        fn process(&self) -> Check<PeerProcess> {
            let pid: NonZeroU32 =
                NonZeroU32::new(self.0.id()).ok_or("fixture child PID was zero")?;
            let peer: PeerProcess = PeerProcess::open(pid)?;
            if pid.get() == process::id()
                || !same_kernel_object(peer.as_handle(), self.0.as_handle())
            {
                return Err("fixture peer did not match the original child process".into());
            }
            peer.check_live()?;
            Ok(peer)
        }

        /// Wait for the one private marker under the caller's existing deadline.
        /// # Errors
        /// Preserve an early child exit, unexpected marker or observation failure.
        fn wait_ready(&mut self, deadline: Instant) -> Check<()> {
            loop {
                let wait: Duration = remaining(deadline)?;
                let stderr: &ChildStderr =
                    self.0.stderr.as_ref().ok_or("child marker pipe missing")?;
                let marker: Option<u8> = peek_fixture_child_stderr(stderr)?;
                if let Some(byte) = marker {
                    return if byte == READY && self.0.try_wait()?.is_none() {
                        let _time: Duration = remaining(deadline)?;
                        Ok(())
                    } else {
                        Err("fixture child readiness was invalid or retired".into())
                    };
                }
                if self.0.try_wait()?.is_some() {
                    return Err("fixture child exited before readiness".into());
                }
                thread::sleep(wait.min(POLL));
            }
        }

        /// Release the private hold and observe this exact child's normal exit.
        /// # Errors
        /// Return abnormal status or elapsed cleanup with the original child retained.
        fn finish(&mut self) -> Check<()> {
            drop(self.0.stdin.take());
            let status: ExitStatus = self.reap(Instant::now() + RETIRE_WAIT)?;
            if status.success() {
                Ok(())
            } else {
                Err(format!("fixture child exited with {status}").into())
            }
        }

        /// Poll the original spawn handle until it reports actual exit.
        /// # Errors
        /// Preserve the wait failure or elapsed deadline.
        fn reap(&mut self, deadline: Instant) -> Check<ExitStatus> {
            loop {
                if let Some(status) = self.0.try_wait()? {
                    let _time: Duration = remaining(deadline)?;
                    return Ok(status);
                }
                thread::sleep(remaining(deadline)?.min(POLL));
            }
        }
    }

    impl Drop for FixtureChild {
        /// Contain only the original child on failure or unwind.
        fn drop(&mut self) {
            drop(self.0.stdin.take());
            if matches!(self.0.try_wait(), Ok(Some(_))) {
                return;
            }
            let killed: io::Result<()> = self.0.kill();
            let reaped: Check<ExitStatus> = self.reap(Instant::now() + RETIRE_WAIT);
            if reaped.is_err() {
                let _diagnostic: io::Result<()> = writeln!(
                    io::stderr().lock(),
                    "fixture child cleanup failed: kill={killed:?}, reap={reaped:?}"
                );
            }
        }
    }

    /// Reuse one absolute cutoff through polling and native wait calls.
    /// # Errors
    /// Refuse an elapsed original deadline.
    fn remaining(deadline: Instant) -> Check<Duration> {
        remaining_at(deadline, Instant::now())
    }

    /// Check the fixed cutoff at one clock observation.
    /// # Errors
    /// Refuse elapsed time, including equality with the cutoff.
    fn remaining_at(deadline: Instant, observed: Instant) -> Check<Duration> {
        deadline
            .checked_duration_since(observed)
            .filter(|time: &Duration| !time.is_zero())
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::TimedOut, "fixture deadline elapsed").into()
            })
    }

    /// Bound each fixture effect by a fresh supervisor sample on the original client.
    /// # Errors
    /// Preserve a clock, peer or effect failure without attempting a successor.
    fn after_current_supervisor_observation(
        deadline: Instant,
        now: impl FnMut() -> Instant,
        observe: impl FnOnce() -> Result<(), LabPipeError>,
        effect: impl FnOnce() -> Check<()>,
    ) -> Check<()> {
        check_fixture_peer_before_cutoff(deadline, now, observe)?;
        effect()
    }

    /// Exercise the guard used at both the request and the correlated reply edge.
    /// # Errors
    /// Fail if a refusal was missing or the healthy neighbor failed.
    /// # Panics
    /// Fail if the clock, peer and effect sequence differs from the expected one.
    #[test]
    fn supervised_fixture_rechecks_after_peer_observation() -> Check<()> {
        let before: Instant = Instant::now();
        let cutoff: Instant = before
            .checked_add(Duration::from_nanos(1))
            .ok_or("fixture cutoff unrepresentable")?;
        let at: Cell<Instant> = Cell::new(before);
        let steps: RefCell<Vec<&'static str>> = RefCell::new(Vec::new());
        let attempt = |after: Instant, observed: Result<(), LabPipeError>| -> Check<()> {
            after_current_supervisor_observation(
                cutoff,
                || {
                    steps.borrow_mut().push("clock");
                    at.get()
                },
                || {
                    steps.borrow_mut().push("peer");
                    at.set(after);
                    observed
                },
                || {
                    steps.borrow_mut().push("effect");
                    Ok(())
                },
            )
        };
        let late: Box<dyn Error> = attempt(cutoff, Ok(()))
            .err()
            .ok_or("fixture effect was admitted at its cutoff")?;
        assert!(matches!(
            late.downcast_ref::<LabPipeError>(),
            Some(LabPipeError::SupervisorCutoff)
        ));
        assert_eq!(steps.borrow().as_slice(), ["clock", "peer", "clock"]);

        at.set(before);
        steps.borrow_mut().clear();
        let foreign: Box<dyn Error> = attempt(before, Err(LabPipeError::ForeignPeer))
            .err()
            .ok_or("fixture effect was admitted for a foreign peer")?;
        assert!(matches!(
            foreign.downcast_ref::<LabPipeError>(),
            Some(LabPipeError::ForeignPeer)
        ));
        assert_eq!(steps.borrow().as_slice(), ["clock", "peer"]);

        steps.borrow_mut().clear();
        attempt(before, Ok(()))?;
        assert_eq!(
            steps.borrow().as_slice(),
            ["clock", "peer", "clock", "effect"]
        );

        assert!(matches!(
            confirm_fixture_write(NativeWritePoll::Written(2), 2, Some(cutoff), || cutoff),
            Err(LabPipeError::WriteTimeout)
        ));
        confirm_fixture_write(NativeWritePoll::Written(2), 2, Some(cutoff), || before)?;
        assert!(matches!(
            confirm_fixture_write(NativeWritePoll::Written(1), 2, Some(cutoff), || cutoff),
            Err(LabPipeError::ShortWrite {
                expected: 2,
                written: 1
            })
        ));
        supervise_write_refusal_and_reuse()
    }

    /// Refuse one write on its exact guarded handle, then observe only the healthy bytes.
    /// # Errors
    /// Preserves native setup, submission, retirement and byte-oracle failures.
    fn supervise_write_refusal_and_reuse() -> Check<()> {
        let (client, pipe): (LabPipeReader, LabOverlappedWriter) = open_lab_reply_pipe(GUARD_CASE)?;
        let handle: RawHandle = pipe.as_handle().as_raw_handle();
        let mut writer: NativePipeWrite = native_fixture_result(NativePipeWrite::new(pipe))?;
        let mut reader: NativePipeRead = native_fixture_result(NativePipeRead::new(client))?;
        let deadline: Instant = Instant::now() + SETUP_WAIT;
        let result: Check<()> = (|| {
            if native_fixture_result(reader.begin(MAX_FRAME_LEN))? != NativeReadStart::Queued {
                return Err("write control's empty pipe did not queue a read".into());
            }
            let refusal: Result<(), LabPipeError> = writer.write_fixture_fragment_before(
                b"refused",
                deadline,
                |observed: BorrowedHandle<'_>| {
                    if observed.as_raw_handle() == handle {
                        Err(LabPipeError::ForeignPeer)
                    } else {
                        Err(LabPipeError::ForeignReplyPair)
                    }
                },
            );
            if !matches!(refusal, Err(LabPipeError::ForeignPeer)) || writer.is_pending() {
                return Err("the guard did not refuse while keeping the write slot idle".into());
            }
            writer.write_fixture_fragment_before(
                b"accepted",
                deadline,
                |observed: BorrowedHandle<'_>| {
                    if observed.as_raw_handle() == handle {
                        Ok(())
                    } else {
                        Err(LabPipeError::ForeignReplyPair)
                    }
                },
            )?;
            observe_first_reply_bytes(&mut reader, b"accepted", |reader: &mut NativePipeRead| {
                read_until(reader, deadline)
            })
        })();
        if result.is_err() {
            retire_pending_peer_read(&mut reader);
        }
        result
    }

    /// Collect the exact client read without starting a fresh observation allowance.
    /// # Errors
    /// Preserve native observation failure or elapsed deadline.
    fn read_until(reader: &mut NativePipeRead, deadline: Instant) -> Check<NativeReadPoll> {
        loop {
            let wait: Duration = remaining(deadline)?;
            let result: NativeReadPoll = native_fixture_result(reader.poll())?;
            if !matches!(result, NativeReadPoll::Pending) {
                let _time: Duration = remaining(deadline)?;
                return Ok(result);
            }
            thread::sleep(wait.min(POLL));
        }
    }

    /// Keep the intended foreign-case child alive until private stdin EOF.
    /// # Errors
    /// Refuse a release byte, read failure or an elapsed child watchdog.
    fn hold_child() -> Check<()> {
        let (send, receive): (
            mpsc::SyncSender<io::Result<usize>>,
            mpsc::Receiver<io::Result<usize>>,
        ) = mpsc::sync_channel(1);
        let reader: thread::JoinHandle<()> = thread::Builder::new().spawn(move || {
            let _release: Result<(), mpsc::SendError<io::Result<usize>>> =
                send.send(io::stdin().lock().read(&mut [0_u8; 1]));
        })?;
        io::stderr().lock().write_all(&[READY])?;
        if receive.recv_timeout(CHILD_WAIT)?? != 0 {
            return Err("fixture child release was not private stdin EOF".into());
        }
        reader
            .join()
            .map_err(|_cause| "fixture child release reader failed")?;
        Ok(())
    }

    /// Send the fixed request and observe the exact reply on the original client.
    /// # Errors
    /// Preserve open, I/O, byte-oracle or Stop-disconnect failure.
    fn exchange_as_child(parent: &PeerProcess) -> Check<()> {
        let deadline: Instant = Instant::now() + CHILD_WAIT;
        let (client, mut writer): (LabPipeReader, LabRequestWriter) = loop {
            match open_supervised_first_exchange_client(CHILD_CASE, parent) {
                Ok(pair) => break pair,
                Err(LabPipeError::Native {
                    operation: "CreateFileW",
                    code: ERROR_FILE_NOT_FOUND,
                }) => {
                    thread::sleep(remaining(deadline)?.min(POLL));
                }
                Err(error) => return Err(error.into()),
            }
        };
        let mut reader: NativePipeRead = native_fixture_result(NativePipeRead::new(client))?;
        let observed: Check<()> = (|| {
            if native_fixture_result(reader.begin(MAX_FRAME_LEN))? != NativeReadStart::Queued {
                return Err("reply was available before the original request".into());
            }
            writer.write_supervised_first_exchange(REQUEST, parent, deadline)?;
            observe_first_reply_bytes(&mut reader, REPLY, |reader: &mut NativePipeRead| {
                read_until(reader, deadline)
            })?;
            if native_fixture_result(reader.begin(MAX_FRAME_LEN))? != NativeReadStart::Queued {
                return Err("the original client already received a second reply".into());
            }
            after_current_supervisor_observation(
                deadline,
                Instant::now,
                || check_supervised_first_exchange_peer(parent, reader.pipe()),
                || Ok(io::stderr().lock().write_all(&[READY])?),
            )?;
            if !matches!(
                read_until(&mut reader, deadline)?,
                NativeReadPoll::Disconnected
            ) {
                return Err("the original client did not disconnect without further data".into());
            }
            Ok(())
        })();
        if observed.is_err() {
            retire_pending_peer_read(&mut reader);
        }
        observed
    }

    /// Dispatch only the two fixed fixture roles from the private supervisor.
    /// # Errors
    /// Preserve bad setup or an unsuccessful child observation.
    #[test]
    #[ignore = "selected by the admitted distinct-process broker fixture supervisor"]
    fn run_broker_first_exchange_child() -> Check<()> {
        let Some(role): Option<OsString> = env::var_os(CHILD_ROLE) else {
            return Ok(());
        };
        match role.to_str() {
            Some("hold") => hold_child(),
            Some("exchange") => {
                let pid: NonZeroU32 = env::var(PARENT_PID)?.parse()?;
                let parent: PeerProcess = PeerProcess::open(pid)?;
                exchange_as_child(&parent)
            }
            _ => Err("fixture child role was unsupported".into()),
        }
    }

    /// Admit the real selected child through the original native endpoint worker.
    /// # Errors
    /// Preserve setup, attribution, reply, Stop or child cleanup failure.
    #[test]
    #[ignore = "requires the admitted offline Windows distinct-process broker fixture"]
    fn native_first_exchange_accepts_retained_child() -> Check<()> {
        let mut child: FixtureChild = FixtureChild::spawn("exchange")?;
        let peer: RetainedFixtureClient = RetainedFixtureClient::new(child.process()?)?;
        let acquire = move || -> Result<NativeBrokerEndpoint, NativeError> {
            prepare_first_exchange_fixture(CHILD_CASE, peer)
        };
        let observed: Check<()> = with_broker_worker_reads::<38, 1, _, _>(
            BrokerEndpointWorker::new(acquire),
            |owner, clock, stop| {
                let cutoff: Instant = owner.connection_cutoff.ok_or("no original accept cutoff")?;
                let control: BrokerControlWake = BrokerControlWake::new()?;
                let mut waiter: BrokerWait = BrokerWait::new();
                // The sampler keeps its scripted time. The actual harness also
                // refuses progress at the original cutoff on the real clock.
                wait_for_first_reply_write(owner, clock, stop, &control, &mut waiter, cutoff)?;
                child.wait_ready(cutoff)?;
                if !matches!(&owner.native_status.outcome,
                    Some(NativeStatusOutcome::Reply(frame)) if frame.as_bytes() == REPLY)
                    || owner.private_status_issue != PrivateStatusIssue::Issued
                    || owner.connection_cutoff != Some(cutoff)
                    || owner.outstanding_work() != 1
                    || owner.worker.completion_wake().is_some()
                {
                    return Err("the selected child lost its original reply or connection".into());
                }
                native_fixture_result(ensure_unimpersonated())?;
                let _time: Duration = remaining(cutoff)?;
                let _stop = deliver(Some(clock.mailbox), stop, 1, 0);
                if owner.poll(clock, stop) != InvocationPoll::Finished
                    || owner.outstanding_work() != 0
                {
                    return Err("Stop did not retire the selected child's connection".into());
                }
                Ok(())
            },
        )?;
        observed?;
        child.finish()
    }

    /// Refuse the same-actor parent on the one connection reserved for the live child.
    /// # Errors
    /// Preserve setup, native progress, refusal, zero-byte or cleanup failure.
    #[test]
    #[ignore = "requires the admitted offline Windows distinct-process broker fixture"]
    fn native_first_exchange_refuses_foreign_same_actor() -> Check<()> {
        let mut child: FixtureChild = FixtureChild::spawn("hold")?;
        child.wait_ready(Instant::now() + SETUP_WAIT)?;
        let peer: RetainedFixtureClient = RetainedFixtureClient::new(child.process()?)?;
        if peer.identity() != &super::own_process_identity()? {
            return Err("fixture foreign connector did not share the expected actor".into());
        }
        let acquire = move || -> Result<NativeBrokerEndpoint, NativeError> {
            prepare_first_exchange_fixture(FOREIGN_CASE, peer)
        };
        let observed: Check<()> = with_broker_worker_reads::<39, 1, _, _>(
            BrokerEndpointWorker::new(acquire),
            |owner, clock, stop| {
                let cutoff: Instant = owner.connection_cutoff.ok_or("no original accept cutoff")?;
                let (client, mut writer): (LabPipeReader, LabRequestWriter) =
                    open_first_exchange_client(FOREIGN_CASE)?;
                let mut reader: NativePipeRead =
                    native_fixture_result(NativePipeRead::new(client))?;
                let control: BrokerControlWake = BrokerControlWake::new()?;
                let mut waiter: BrokerWait = BrokerWait::new();
                let observed: Check<()> = (|| {
                    if waiter.observe(
                        &control,
                        remaining(cutoff)?,
                        owner.worker.completion_wake(),
                    )? != WakeSource::Completion
                        || !matches!(owner.poll(clock, stop), InvocationPoll::Pending(_))
                        || native_fixture_result(reader.begin(MAX_FRAME_LEN))?
                            != NativeReadStart::Queued
                    {
                        return Err(
                            "foreign connector did not reach the original queued read".into()
                        );
                    }
                    let getters: u32 = GETTERS[39].load(SeqCst);
                    writer.write(REQUEST)?;
                    if waiter.observe(
                        &control,
                        remaining(cutoff)?,
                        owner.worker.completion_wake(),
                    )? != WakeSource::Completion
                        || !matches!(owner.poll(clock, stop), InvocationPoll::Pending(_))
                    {
                        return Err(
                            "foreign connector's valid frame did not reach attribution".into()
                        );
                    }
                    let _remaining: Duration = remaining(cutoff)?;
                    if !matches!(
                        &owner.native_status.outcome,
                        Some(NativeStatusOutcome::Identity(
                            IdentifiedReadError::Unattributed
                        ))
                    ) || owner.private_status_issue != PrivateStatusIssue::Unissued
                        || owner.native_status.reply_written
                        || owner.connection_cutoff != Some(cutoff)
                        || owner.outstanding_work() != 1
                        || owner.worker.completion_wake().is_some()
                        || GETTERS[39].load(SeqCst) != getters
                        || !matches!(
                            native_fixture_result(reader.poll())?,
                            NativeReadPoll::Pending
                        )
                        || child.0.try_wait()?.is_some()
                    {
                        return Err("the valid foreign frame was not refused on the live child's connection".into());
                    }
                    native_fixture_result(ensure_unimpersonated())?;
                    let _time: Duration = remaining(cutoff)?;
                    let _stop = deliver(Some(clock.mailbox), stop, 1, 0);
                    if owner.poll(clock, stop) != InvocationPoll::Finished
                        || owner.outstanding_work() != 0
                        || !matches!(
                            read_until(&mut reader, Instant::now() + RETIRE_WAIT)?,
                            NativeReadPoll::Disconnected
                        )
                    {
                        return Err(
                            "Stop did not retire the foreign connection without reply bytes".into(),
                        );
                    }
                    Ok(())
                })();
                if observed.is_err() {
                    retire_pending_peer_read(&mut reader);
                }
                observed
            },
        )?;
        observed?;
        child.finish()
    }
}
