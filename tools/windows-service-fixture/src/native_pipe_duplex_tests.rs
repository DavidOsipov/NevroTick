// SPDX-License-Identifier: GPL-2.0-only
//! Exercise both native directions of one admitted test-only duplex pipe.

use std::{
    error::Error,
    fmt::Debug,
    io::{self, Write},
    num::NonZeroU32,
    os::windows::io::{AsHandle, BorrowedHandle, OwnedHandle},
    rc::{Rc, Weak},
    task::Poll,
    thread,
    time::{Duration, Instant},
};

use windows_platform_native::{PeerPrimaryToken, PeerPrimaryTokenError, PeerProcess};

use super::{
    LabDuplexPipe, LabIdentificationPipe, LabOverlappedWriter, LabPipeError, LabPipeReader,
    ReplyPipePair, open_lab_duplex_pipe, open_signaled_duplex_accept, retire_duplex_accept,
};
use crate::{
    NativeError,
    broker_endpoint::{AcceptCompletion, Reservation},
    native_broker_endpoint::{FixtureConnectionError, NativeBrokerEndpoint},
    native_pipe_identity::{
        RetainedPrimaryToken, ensure_unimpersonated, observe_peer_token_during_identification,
        own_process_identity,
    },
    native_pipe_read::{
        IdentifiedRequestRead, NativeCancel, NativePipeRead, NativeReadBytes, NativeReadError,
        NativeReadPoll, NativeReadStart, RetiredRead,
    },
    native_pipe_write::NativePipeWrite,
};

/// Bound each actual completion observation and each cancellation drain.
const READ_WAIT: Duration = Duration::from_secs(2);

/// One same-process name reused only after terminal close by the supervised test.
const DUPLEX_ROUND_TRIP_CASE: u32 = 0x4455_504c;

/// Preserve the fixture's original failure in the test result.
type Check<T> = Result<T, Box<dyn Error>>;

/// An identified request and its reply cross the original connection in opposite directions.
/// # Errors
/// Refuses failed setup, mismatched direction, incomplete delivery, or changed bytes.
#[test]
#[ignore = "requires the admitted offline Windows duplex fixture supervisor"]
fn identified_request_and_reply_share_one_duplex_pipe() -> Check<()> {
    /// Independent expected request bytes.
    const REQUEST: &[u8] = b"duplex request";
    /// Independent expected reply bytes.
    const REPLY: &[u8] = b"duplex reply";

    reject_unobserved_and_cancelled_accept(DUPLEX_ROUND_TRIP_CASE)?;
    let pair: LabDuplexPipe = open_lab_duplex_pipe(DUPLEX_ROUND_TRIP_CASE)?;
    let request_pair: &ReplyPipePair = pair
        .request
        .reader
        .reply_pair()
        .ok_or("duplex request has no connection binding")?;
    let reply_pair: &ReplyPipePair = &pair.reply_writer.reply_pair;
    let client: Weak<OwnedHandle> = Rc::downgrade(&pair.reply_reader.handle);
    if !request_pair.reader.ptr_eq(&reply_pair.writer)
        || !request_pair.writer.ptr_eq(&reply_pair.reader)
        || !Rc::ptr_eq(&pair.request.reader.handle, &pair.reply_writer.handle)
        || !client.ptr_eq(&request_pair.writer)
    {
        return Err("the two directions do not share both original endpoint owners".into());
    }
    let witness_server: Rc<OwnedHandle> = Rc::clone(&pair.request.reader.handle);
    let (request, reply_reader, reply_writer): (
        LabIdentificationPipe,
        LabPipeReader,
        LabOverlappedWriter,
    ) = pair.into_transport();
    let mut request: IdentifiedRequestRead =
        observed(IdentifiedRequestRead::new(request, own_process_identity()?))?;
    let mut reply_reader: NativePipeRead = observed(NativePipeRead::new(reply_reader))?;
    let mut reply_writer: NativePipeWrite = observed(NativePipeWrite::new(reply_writer))?;
    request.check_peers()?;
    reply_writer.check_reply_reader(&reply_reader)?;

    let received_request: NativeReadBytes = transfer_request(&mut request, REQUEST)?;
    ensure_unimpersonated()?;
    if received_request.as_bytes() != REQUEST
        || !matches!(
            reply_writer.check_reply_completion(&received_request),
            Err(LabPipeError::ForeignReplyPair)
        )
    {
        return Err("the duplex request lost its bytes or directional identity".into());
    }
    request.check_peers()?;
    check_peer_token_refusal_and_reversion(witness_server.as_handle())?;
    drop(witness_server);
    let received_reply: NativeReadBytes =
        transfer_reply(&mut reply_reader, &mut reply_writer, REPLY)?;
    reply_writer.check_reply_completion(&received_reply)?;
    if received_reply.as_bytes() != REPLY
        || request.is_pending()
        || reply_reader.is_pending()
        || reply_writer.is_pending()
    {
        return Err("the duplex reply lost its bytes or left a native operation pending".into());
    }
    drop(request);
    drop(reply_reader);
    drop(reply_writer);
    let successor: LabDuplexPipe = open_lab_duplex_pipe(DUPLEX_ROUND_TRIP_CASE)?;
    if !matches!(
        successor
            .reply_writer
            .require_reply_pair(received_reply.reply_pair()),
        Err(LabPipeError::ForeignReplyPair)
    ) {
        return Err("the new connection accepted a completion from its predecessor".into());
    }
    Ok(())
}

/// Check the actual ambient-token refusal and then query the same retained process.
///
/// The server is the identified duplex pair's original endpoint and its latest
/// message has completed. The current process is the checked client of that pair.
///
/// # Errors
/// Preserve a setup or native query failure, or refuse the wrong ambient-token
/// outcome or a changed process token after reversion.
fn check_peer_token_refusal_and_reversion(server: BorrowedHandle<'_>) -> Check<()> {
    let pid: NonZeroU32 =
        NonZeroU32::new(std::process::id()).ok_or("fixture process has no PID")?;
    let peer: PeerProcess = PeerProcess::open(pid)?;
    peer.check_live()?;
    let expected: RetainedPrimaryToken = RetainedPrimaryToken::open(peer.as_handle())?;
    let observed: Result<PeerPrimaryToken, PeerPrimaryTokenError> =
        observe_peer_token_during_identification(server, &peer)?;
    ensure_unimpersonated()?;
    match observed {
        Err(PeerPrimaryTokenError::AmbientThreadToken) => {}
        Err(error) => return Err(error.into()),
        Ok(_) => return Err("peer token query accepted ambient impersonation".into()),
    }
    let recovered: PeerPrimaryToken = peer.retain_primary_token()?;
    if !expected.matches_observation(&recovered) {
        return Err("peer token changed across the ambient refusal and reversion".into());
    }
    peer.check_live()?;
    println!("peer-primary-token ambient=refused post-reversion=matched");
    Ok(())
}

/// Reject a wake without completion and a connected result after cancellation was attempted.
/// The subsequent healthy case must be able to reacquire the same name.
///
/// # Errors
/// Returns native failure or an accepted handoff without an eligible terminal observation.
fn reject_unobserved_and_cancelled_accept(pipe_instance: u32) -> Check<()> {
    let (accept, _client): (NativeBrokerEndpoint, OwnedHandle) =
        open_signaled_duplex_accept(pipe_instance)?;
    let mut accept: NativeBrokerEndpoint = match accept.into_connected_fixture() {
        Err(FixtureConnectionError::Ineligible(endpoint)) => endpoint,
        Err(FixtureConnectionError::AcceptEventClose(cause)) => return Err(cause.into()),
        Ok(_server) => return Err("duplex accept transferred after a wake alone".into()),
    };
    let cancellation: Result<(), NativeError> = accept.request_accept_cancel();
    if let Err(cause) = cancellation {
        return Err(retire_duplex_accept(
            accept,
            LabPipeError::AcceptNative(cause),
            Some(cancellation),
        )
        .into());
    }
    let completion: Result<Poll<AcceptCompletion>, NativeError> = accept.poll_accept();
    match completion {
        Ok(Poll::Ready(AcceptCompletion::Connected)) => {}
        Ok(Poll::Ready(AcceptCompletion::Cancelled) | Poll::Pending) => {
            return Err(retire_duplex_accept(
                accept,
                LabPipeError::AcceptHistory("cancelled duplex accept lost its signaled completion"),
                Some(cancellation),
            )
            .into());
        }
        Err(cause) => {
            return Err(retire_duplex_accept(
                accept,
                LabPipeError::AcceptNative(cause),
                Some(cancellation),
            )
            .into());
        }
    }
    match accept.into_connected_fixture() {
        Err(FixtureConnectionError::Ineligible(endpoint)) => observed(endpoint.close()),
        Err(FixtureConnectionError::AcceptEventClose(cause)) => Err(cause.into()),
        Ok(_server) => Err("duplex accept transferred after cancellation was attempted".into()),
    }
}

/// Drive the paired sender through the identified read, retiring incomplete custody on error.
/// # Errors
/// Retains the original request failure.
fn transfer_request(request: &mut IdentifiedRequestRead, bytes: &[u8]) -> Check<NativeReadBytes> {
    let result: Check<NativeReadBytes> = (|| {
        let start: NativeReadStart = observed(request.begin(bytes.len()))?;
        if start != NativeReadStart::Queued {
            return Err("the empty duplex request pipe did not queue its read".into());
        }
        request.write_from_client(bytes)?;
        observe_read(|| observed(request.poll()))
    })();
    if let Err(original) = &result
        && request.is_pending()
    {
        let cancellation: Result<NativeCancel, NativeReadError> = request.retire();
        let completion: Check<RetiredRead> =
            observe_terminal(|| observed(request.poll_retirement()));
        if request.is_pending() {
            report_unconfirmed_drain("request", original.as_ref(), cancellation, &completion);
        }
    }
    result
}

/// Drive the reverse direction through the physically bound reader and writer.
/// # Errors
/// Retains the original reply failure and settles the read after losing admission.
fn transfer_reply(
    reader: &mut NativePipeRead,
    writer: &mut NativePipeWrite,
    bytes: &[u8],
) -> Check<NativeReadBytes> {
    let result: Check<NativeReadBytes> = (|| {
        let start: NativeReadStart = observed(reader.begin(bytes.len()))?;
        if start != NativeReadStart::Queued {
            return Err("the empty duplex reply pipe did not queue its read".into());
        }
        writer.write_fixture_fragment(bytes)?;
        observe_read(|| observed(reader.poll()))
    })();
    if let Err(original) = &result
        && reader.is_pending()
    {
        let cancellation: Result<NativeCancel, NativeReadError> = reader.request_cancel();
        let completion: Check<()> = observe_terminal(|| match observed(reader.poll())? {
            NativeReadPoll::Pending => Ok(Poll::Pending),
            NativeReadPoll::Data(_) | NativeReadPoll::Cancelled | NativeReadPoll::Disconnected => {
                Ok(Poll::Ready(()))
            }
        });
        if reader.is_pending() {
            report_unconfirmed_drain("reply", original.as_ref(), cancellation, &completion);
        }
    }
    result
}

/// Preserve the original failure and cleanup causes before invalid-drop abort.
fn report_unconfirmed_drain<T: Debug>(
    direction: &'static str,
    original: &dyn Error,
    cancellation: Result<NativeCancel, NativeReadError>,
    completion: &Check<T>,
) {
    let _diagnostic: io::Result<()> = writeln!(
        io::stderr().lock(),
        "duplex {direction} read still pending after {original}; cancel {cancellation:?}; drain {completion:?}",
    );
}

/// Observe a successful read without treating cancellation or disconnect as bytes.
/// # Errors
/// Retains a failed observation, expiry, or a terminal refusal.
fn observe_read(mut poll: impl FnMut() -> Check<NativeReadPoll>) -> Check<NativeReadBytes> {
    observe_terminal(|| match poll()? {
        NativeReadPoll::Pending => Ok(Poll::Pending),
        NativeReadPoll::Data(bytes) => Ok(Poll::Ready(bytes)),
        NativeReadPoll::Cancelled | NativeReadPoll::Disconnected => {
            Err("the duplex read ended without bytes".into())
        }
    })
}

/// Poll one exact operation with a fixed finite observation allowance.
/// # Errors
/// Preserves a polling failure or reports expiry.
fn observe_terminal<T>(mut poll: impl FnMut() -> Check<Poll<T>>) -> Check<T> {
    let started: Instant = Instant::now();
    loop {
        let remaining: Duration = READ_WAIT.saturating_sub(started.elapsed());
        if remaining.is_zero() {
            return Err(io::Error::new(io::ErrorKind::TimedOut, "duplex read timed out").into());
        }
        if let Poll::Ready(result) = poll()? {
            return Ok(result);
        }
        thread::sleep(remaining.min(Duration::from_millis(1)));
    }
}

/// Project a local typed fixture failure into the libtest result.
/// # Errors
/// Preserves the exact debug representation of an observation failure.
fn observed<T>(result: Result<T, impl Debug>) -> Check<T> {
    result.map_err(|error| format!("{error:?}").into())
}
