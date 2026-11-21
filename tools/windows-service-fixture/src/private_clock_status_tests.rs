// SPDX-License-Identifier: GPL-2.0-only
//! Controls through the actual broker owner and the private fixture exchange.

use std::{
    cell::Cell,
    error::Error,
    sync::atomic::{AtomicU32, Ordering::SeqCst},
    time::{Duration, Instant},
};

use windows_platform::service::{ServiceStatus, StatusReporter};

use super::{
    BrokerAvailability, BrokerStatusSession, Clock, Connection, Delivery, InertWorker, LabRole,
    NativeError, Reply, ServiceInvocation, SessionFault, StopNotification,
};
use crate::{
    BrokerClockRead, NativeOperation,
    broker_clock_change::{BrokerClockChanges, NativeBrokerReader, deliver, report_status},
    host::{
        HostReceipt, InvocationPoll, StatusFactory, start_broker_invocation,
        tests::HealthyTokenObserver,
    },
    private_clock_wire::FrameError,
};

/// Each scenario owns one fresh registered mailbox and one independent counter.
static MAILBOXES: [BrokerClockChanges; 7] = [const { BrokerClockChanges::new() }; 7];
/// Calls of the original raw getter, separated from trusted deadline observations.
static GETTERS: [AtomicU32; 7] = [const { AtomicU32::new(0) }; 7];

/// Observe the same Windows coordinate adapter used by the fixed broker.
/// # Errors
/// Preserve the actual Windows adapter's raw failure.
/// # Panics
/// Panics if the selected fixture has no assigned counter.
#[cfg(windows)]
fn sample<const N: usize>()
-> Result<windows_platform_native::ClockCoordinates, windows_platform_native::ClockReadError> {
    let _previous: u32 = GETTERS[N].fetch_add(1, SeqCst);
    windows_platform_native::read_clock_coordinates()
}

/// Observe one known value through the portable broker reader.
/// # Errors
/// Refuses an unscripted repeat getter call.
/// # Panics
/// Panics if the selected fixture has no assigned counter.
#[cfg(not(windows))]
fn sample<const N: usize>() -> Result<u64, u32> {
    let previous: u32 = GETTERS[N].fetch_add(1, SeqCst);
    if previous == 0 { Ok(1) } else { Err(7) }
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
    /// The first observation passes and the following observation triggers the action.
    final_action: Option<(bool, DecisionAction)>,
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
            Some((false, action)) => self.final_action = Some((true, action)),
            Some((true, DecisionAction::Advance(time))) => {
                self.now = time;
                self.reached = true;
            }
            Some((true, DecisionAction::Deliver(control))) => {
                self.reached = deliver(Some(self.mailbox), self.stop, control, 0).wake;
            }
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
    let mailbox: &'static BrokerClockChanges = &MAILBOXES[N];
    let stop: StopNotification = StopNotification::new();
    let mut clock: ScriptClock<'_> = ScriptClock {
        now: Instant::now(),
        final_action: None,
        reached: false,
        mailbox,
        stop: &stop,
    };
    let mut owner: ServiceInvocation<Reporter, InertWorker> = start_broker_invocation(
        0,
        &mut Reporter(mailbox),
        &mut HealthyTokenObserver(LabRole::ClockBroker),
        &mut clock,
        &stop,
        NativeBrokerReader::new(mailbox, sample::<N>).with_sampling(|_, _| None),
    )
    .map_err(|_receipt: HostReceipt| "unexpected broker startup failure")?;
    let first: InvocationPoll = owner.poll(&mut clock, &stop);
    let result: T = exercise(&mut owner, &mut clock, &stop);
    // A refused transition may leave the script unused. Never trigger it during cleanup.
    clock.final_action = None;
    let _stop_disposition = deliver(Some(mailbox), &stop, 1, 0);
    let last: InvocationPoll = owner.poll(&mut clock, &stop);
    let receipt: HostReceipt = owner.take_receipt().ok_or("inert broker did not finish")?;
    assert_eq!(first, InvocationPoll::Pending(Duration::from_secs(1)));
    assert_eq!(last, InvocationPoll::Finished);
    assert_eq!(receipt.result(), &Ok(()));
    assert!(matches!(
        receipt.broker_clock_read(),
        Some(BrokerClockRead::Completed { revision, result: Ok(_), .. })
            if revision.get() == 1
    ));
    assert_eq!(GETTERS[N].load(SeqCst), 1);
    Ok(result)
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
            let pending = sender.request()?;
            let local: Reply = session.receive(pending.delivery())?;
            let foreign: Reply = {
                let other_pending: super::PendingStatus<'_> = other_sender.request()?;
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
        session.clock.final_action = Some((false, action));
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
