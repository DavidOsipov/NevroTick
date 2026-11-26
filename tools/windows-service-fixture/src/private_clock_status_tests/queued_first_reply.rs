// SPDX-License-Identifier: GPL-2.0-only
//! Backpressure on the original Windows broker exchange.

use std::{cell::RefCell, num::NonZeroU32, rc::Rc};

use windows_platform::service::ServiceState;
use windows_platform_native::PeerProcess;

use super::{
    BrokerControlWake, BrokerWait, Duration, Error, FirstReplyCase, Instant, InvocationPoll,
    LabPipeReader, MAX_FRAME_LEN, NativeCancel, NativeError, NativePipeRead, NativeReadPoll,
    NativeReadStart, NativeWritePoll, NativeWriteStart, ServiceInvocation, StopNotification,
    WakeSource, Worker, deliver_first_reply_suspend, finish_native_first_reply,
    native_fixture_result, observe_first_reply_bytes, retire_pending_peer_read,
    wait_for_first_reply_write, wait_for_native_read, with_broker_worker_reads,
};
use super::{Reporter, ScriptClock};
use crate::{
    broker_endpoint::BrokerEndpointWorker,
    native_broker_endpoint::{
        FIRST_REPLY_PREFILL, FirstReplyControl, FirstReplyIo, NativeBrokerEndpoint,
    },
    native_pipe_identity::RetainedFixtureClient,
    native_pipe_pair::{
        LabRequestWriter, open_first_exchange_client, prepare_first_exchange_fixture,
    },
};

/// A bounded native observation of the one submitted status reply.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Step {
    /// The syscall retained the exact OVERLAPPED storage.
    Queued,
    /// The first reply completed inline.
    Immediate,
    /// Windows has yet to settle the write.
    Pending,
    /// Windows returned a native cancellation request outcome.
    Cancel(NativeCancel),
    /// Windows returned an exact terminal byte count.
    Written(usize),
    /// Windows completed the write by cancelling it.
    Cancelled,
    /// Windows observed the peer disconnect before reporting a count.
    Disconnected,
}

impl From<FirstReplyIo<'_>> for Step {
    /// Record only native facts, leaving the actual write owner in control.
    fn from(io: FirstReplyIo<'_>) -> Self {
        match io {
            FirstReplyIo::Submitted(NativeWriteStart::Queued) => Self::Queued,
            FirstReplyIo::Submitted(NativeWriteStart::Immediate) => Self::Immediate,
            FirstReplyIo::Observed(NativeWritePoll::Pending) => Self::Pending,
            FirstReplyIo::Observed(NativeWritePoll::Written(bytes)) => Self::Written(*bytes),
            FirstReplyIo::Observed(NativeWritePoll::Cancelled) => Self::Cancelled,
            FirstReplyIo::Observed(NativeWritePoll::Disconnected) => Self::Disconnected,
            FirstReplyIo::Cancel(cancel) => Self::Cancel(*cancel),
        }
    }
}

/// Exercise an actual pending first status write across both decisions.
///
/// # Errors
/// Returns an unmet backpressure setup condition or a lease, wire or cleanup failure.
#[test]
#[ignore = "requires the admitted offline Windows first-exchange fixture supervisor"]
fn native_first_reply_retires_after_revision_withdrawal_while_pending() -> Result<(), Box<dyn Error>>
{
    observe_pending_reply::<45>(FirstReplyCase::Available)?;
    observe_pending_reply::<46>(FirstReplyCase::SuspendedWhileQueued)
}

/// Leave the original client read idle through native submission and the first observation.
///
/// # Errors
/// Refuses incorrect admission, physical operation history or terminal cleanup.
fn observe_pending_reply<const CASE: usize>(
    scenario: FirstReplyCase,
) -> Result<(), Box<dyn Error>> {
    let pid: NonZeroU32 = NonZeroU32::new(std::process::id()).ok_or("fixture PID was zero")?;
    let peer: RetainedFixtureClient = RetainedFixtureClient::new(PeerProcess::open(pid)?)?;
    let pipe_case: u32 = u32::try_from(CASE)?;
    let events: Rc<RefCell<Vec<Step>>> = Rc::default();
    let sink: Rc<RefCell<Vec<Step>>> = Rc::clone(&events);
    let probe: FirstReplyControl = FirstReplyControl::Backpressure(Rc::new(move |io| {
        sink.borrow_mut().push(Step::from(io));
    }));
    let acquire = move || -> Result<NativeBrokerEndpoint, NativeError> {
        prepare_first_exchange_fixture(pipe_case, peer, Some(probe))
    };
    with_broker_worker_reads::<CASE, 1, _, _>(
        BrokerEndpointWorker::new(acquire),
        |owner, clock, stop| {
            let cutoff: Instant = owner.connection_cutoff.ok_or("no original accept cutoff")?;
            let (client_reader, mut client_writer): (LabPipeReader, LabRequestWriter) =
                native_fixture_result(open_first_exchange_client(pipe_case))?;
            let mut reader: NativePipeRead =
                native_fixture_result(NativePipeRead::new(client_reader))?;
            let control: BrokerControlWake = BrokerControlWake::new()?;
            let mut waiter: BrokerWait = BrokerWait::new();
            let result: Result<(), Box<dyn Error>> = (|| {
                poll_on_completion(owner, clock, stop, &control, &mut waiter)?;
                let request: &[u8] =
                    b"\x01\x00\x01\x00\x02\x00\x00\x00\x2a\x00\x00\x00\x00\x00\x00\x00{}";
                native_fixture_result(client_writer.write(request))?;
                poll_on_completion(owner, clock, stop, &control, &mut waiter)?;
                if !matches!(owner.poll(clock, stop), InvocationPoll::Pending(_))
                    || *events.borrow() != [Step::Queued, Step::Pending]
                    || owner.native_status.reply_written
                    || owner.outstanding_work() != 1
                    || clock.now >= cutoff
                    || stop.stop_requested()
                {
                    return Err(format!(
                        "first write did not reach native backpressure: {:?}",
                        events.borrow()
                    )
                    .into());
                }
                if matches!(scenario, FirstReplyCase::SuspendedWhileQueued) {
                    withdraw_queued_reply(owner, clock, stop, cutoff, &events)?;
                }
                read_prefill(&mut reader)?;
                if matches!(scenario, FirstReplyCase::Available) {
                    let _: NativeReadStart = native_fixture_result(reader.begin(MAX_FRAME_LEN))?;
                    wait_for_first_reply_write(
                        owner,
                        clock,
                        stop,
                        &control,
                        &mut waiter,
                        Instant::now() + Duration::from_secs(2),
                    )?;
                    observe_first_reply_bytes(
                        &mut reader,
                        scenario.expected_reply(),
                        wait_for_native_read,
                    )?;
                    check_completion(
                        &events.borrow(),
                        Step::Written(scenario.expected_reply().len()),
                    )?;
                }
                if native_fixture_result(reader.begin(MAX_FRAME_LEN))? != NativeReadStart::Queued {
                    return Err("unexpected bytes follow the selected pipe history".into());
                }
                finish_native_first_reply(owner, clock, stop, &mut reader, cutoff, scenario)
            })();
            if result.is_err() {
                retire_pending_peer_read(&mut reader);
            }
            result
        },
    )?
}

/// Read only the remaining inert prefix, even if the status write completes meanwhile.
///
/// # Errors
/// Refuses a missing fragment or bytes differing from the exact prefill.
fn read_prefill(reader: &mut NativePipeRead) -> Result<(), Box<dyn Error>> {
    let mut remaining: &[u8] = &FIRST_REPLY_PREFILL;
    while !remaining.is_empty() {
        let _: NativeReadStart = native_fixture_result(reader.begin(remaining.len()))?;
        let NativeReadPoll::Data(completed) = wait_for_native_read(reader)? else {
            return Err("the original peer did not receive the prefill".into());
        };
        let bytes: &[u8] = completed.as_bytes();
        if bytes.is_empty() {
            return Err("the original peer received an empty prefill fragment".into());
        }
        remaining = remaining
            .strip_prefix(bytes)
            .ok_or("the original peer received wrong prefill bytes")?;
    }
    Ok(())
}

/// Advance the original accept or synchronized request after its native wake.
///
/// # Errors
/// Refuses a missing completion signal or premature service retirement.
fn poll_on_completion<P: Worker>(
    owner: &mut ServiceInvocation<Reporter, P>,
    clock: &mut ScriptClock<'_>,
    stop: &StopNotification,
    control: &BrokerControlWake,
    waiter: &mut BrokerWait,
) -> Result<(), Box<dyn Error>> {
    if waiter.observe(
        control,
        Duration::from_secs(2),
        owner.worker.completion_wake(),
    )? != WakeSource::Completion
        || !matches!(owner.poll(clock, stop), InvocationPoll::Pending(_))
    {
        return Err("the original accept or request did not reach its owner".into());
    }
    Ok(())
}

/// Cancel after native pending and collect before the original client frees any buffer space.
///
/// # Errors
/// Refuses a wrong cancel result, lost Running ownership or missing terminal collection.
fn withdraw_queued_reply<P: Worker>(
    owner: &mut ServiceInvocation<Reporter, P>,
    clock: &mut ScriptClock<'_>,
    stop: &StopNotification,
    cutoff: Instant,
    events: &RefCell<Vec<Step>>,
) -> Result<(), Box<dyn Error>> {
    deliver_first_reply_suspend(clock, stop, cutoff)?;
    if !matches!(owner.poll(clock, stop), InvocationPoll::Pending(_))
        || *events.borrow()
            != [
                Step::Queued,
                Step::Pending,
                Step::Cancel(NativeCancel::Requested),
            ]
        || owner.state() != Some(ServiceState::Running)
        || owner.take_receipt().is_some()
        || owner.outstanding_work() != 1
        || owner.native_status.reply_written
    {
        return Err(format!(
            "withdrawal did not cancel the original queued write: {:?}",
            events.borrow()
        )
        .into());
    }
    let control: BrokerControlWake = BrokerControlWake::new()?;
    let mut waiter: BrokerWait = BrokerWait::new();
    wait_for_first_reply_write(
        owner,
        clock,
        stop,
        &control,
        &mut waiter,
        Instant::now() + Duration::from_secs(2),
    )?;
    check_completion(&events.borrow(), Step::Cancelled)
}

/// Check the terminal result without conflating pending observations with extra writes.
///
/// # Errors
/// Refuses an extra submit, an unexpected cancellation request or a wrong terminal result.
fn check_completion(steps: &[Step], terminal: Step) -> Result<(), Box<dyn Error>> {
    let rest: &[Step] = steps
        .strip_prefix(&[Step::Queued, Step::Pending])
        .ok_or("status write did not become physically pending")?;
    let rest: &[Step] = if terminal == Step::Cancelled {
        rest.strip_prefix(&[Step::Cancel(NativeCancel::Requested)])
            .ok_or("the original pending write did not receive exactly one cancellation request")?
    } else {
        rest
    };
    if !rest
        .strip_suffix(&[terminal])
        .is_some_and(|pending: &[Step]| pending.iter().all(|step: &Step| *step == Step::Pending))
    {
        return Err(format!("unexpected terminal history: {steps:?}").into());
    }
    Ok(())
}
