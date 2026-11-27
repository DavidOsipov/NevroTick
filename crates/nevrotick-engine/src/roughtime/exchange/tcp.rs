// SPDX-License-Identifier: GPL-2.0-only
//! One bounded Roughtime request on a selected loopback TCP connection.

use super::retry::{RetryBinding, RetryLease, RetryState};
use super::{
    Capture, ExchangeError, ExchangeTransport, OriginalRequest, Phase, RECEIVE_BYTES, Receipt,
    RoundAuthority, StagedReceipt, exchange_cutoff, prepare, require_phase, socket_error,
};
use crate::{
    RECEIVE_CAP,
    native_network_job::{DispatchAdmission, JoinWitness, NativeJob, NativeJobError},
    roughtime::{RootKey, VerifyError},
};
use ntp_wire::roughtime::{DecodeError, FRAME_HEADER_BYTES, response_frame_length};
use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream},
    sync::MutexGuard,
    task::Poll,
    time::{Duration, Instant},
};

/// One bounded stream request and at most one response.
#[must_use = "drive or retire the original Roughtime stream attempt"]
pub struct PendingTcpExchange<'a> {
    /// Original one-shot publication and withdrawal owner.
    authority: &'a RoundAuthority,
    /// Exclusive deadline of the surrendered connection's one request.
    cutoff: Instant,
    /// Owned until a complete response or a terminal result.
    connection: Option<Connection<'a>>,
}

impl<'a> PendingTcpExchange<'a> {
    /// Take a fresh exclusively owned connected loopback stream without writing yet.
    ///
    /// The caller owns connection establishment and supplies a stream with no
    /// prior application traffic, retained clone or other user. Use the default
    /// disabled linger setting. Each poll performs at most one nonblocking write
    /// or read. A terminal result closes the stream.
    ///
    /// # Errors
    /// Refuses an invalid budget, unselected/non-loopback stream, native or
    /// entropy failure, or unavailable, expired or withdrawn authority.
    pub fn start(
        authority: &'a RoundAuthority,
        stream: TcpStream,
        peer: SocketAddr,
        root: &'a RootKey,
        budget: Duration,
    ) -> Result<Self, ExchangeError> {
        let cutoff: Instant = exchange_cutoff(budget)?;
        if !peer.ip().is_loopback() || peer.port() == 0 {
            return Err(ExchangeError::Configuration);
        }
        configure_connection(&stream, peer)?;
        let request: OriginalRequest<'a> =
            prepare(authority, peer, root, cutoff, &[], ExchangeTransport::Tcp)?;
        Ok(Self::adopt(authority, cutoff, stream, request, None))
    }

    /// Connect to one selected controlled alias and retain its original retry lease.
    ///
    /// Call from a blocking controlled owner. The native connector gets only the
    /// time remaining under the original cutoff. Its original authority stays
    /// held during connection establishment. Contention on concurrent withdrawal
    /// is undelivered and must be settled by its caller. Connection success leaves
    /// the retry lease charged until current signed publication or retirement.
    ///
    /// # Errors
    /// Refuses the binding or original cutoff, authority, entropy, connect or setup failure.
    pub fn connect_with_retry(
        authority: &'a RoundAuthority,
        binding: &'a mut RetryBinding<'_>,
        peer: SocketAddr,
        budget: Duration,
    ) -> Result<Self, ExchangeError> {
        let cutoff: Instant = exchange_cutoff(budget)?;
        let (root, state): (&RootKey, &mut RetryState) =
            binding.admission(peer, ExchangeTransport::Tcp)?;
        Self::connect_bound(authority, peer, root, cutoff, &[], state)
    }

    /// Connect one bound leg and derive its nonce from the original round prefix.
    ///
    /// # Errors
    /// Refuses the original authority, deadline, retry floor or native work.
    pub(in crate::roughtime) fn connect_bound(
        authority: &'a RoundAuthority,
        peer: SocketAddr,
        root: &'a RootKey,
        cutoff: Instant,
        history: &[Receipt],
        state: &'a mut RetryState,
    ) -> Result<Self, ExchangeError> {
        let request: OriginalRequest<'a> = prepare(
            authority,
            peer,
            root,
            cutoff,
            history,
            ExchangeTransport::Tcp,
        )?;
        let phase: MutexGuard<'_, Phase> = authority.phase()?;
        require_phase(*phase, Phase::Preparing)?;
        let before: Instant = Instant::now();
        if before >= cutoff {
            return Err(ExchangeError::Expired);
        }
        let lease: RetryLease<'a> = state.charge(before)?;
        let stream: TcpStream = connect_checked(peer, cutoff, before, connect_native)?;
        Ok(Self::adopt(authority, cutoff, stream, request, Some(lease)))
    }

    /// Transfer the original configured stream and optional charged retry into the caller.
    fn adopt(
        authority: &'a RoundAuthority,
        cutoff: Instant,
        stream: TcpStream,
        request: OriginalRequest<'a>,
        retry: Option<RetryLease<'a>>,
    ) -> Self {
        Self {
            authority,
            cutoff,
            connection: Some(Connection {
                stream,
                request,
                written: 0,
                sent: None,
                response: [0; RECEIVE_BYTES],
                received_bytes: 0,
                received: None,
                target: FRAME_HEADER_BYTES,
                stalls_left: RECEIVE_CAP,
                retry,
            }),
        }
    }

    /// Advance one nonblocking operation on the original stream.
    ///
    /// Positive I/O is bounded by the request and response lengths. Idle calls
    /// share one finite allowance. The complete frame then reaches the same
    /// signature verification and publication guard as a datagram.
    ///
    /// # Errors
    /// Refuses withdrawn, expired or exhausted work, incomplete or oversized
    /// frames, malformed or unauthenticated input, and native failures.
    pub fn poll(&mut self) -> Result<Option<StagedReceipt<'a>>, ExchangeError> {
        let mut phase: MutexGuard<'_, Phase> = match self.authority.phase() {
            Ok(phase) => phase,
            Err(error) => {
                drop(self.connection.take());
                return Err(error);
            }
        };
        let connection: &mut Connection<'a> =
            self.connection.as_mut().ok_or(ExchangeError::Spent)?;
        match connection.advance(&mut phase, self.cutoff) {
            Ok(false) => Ok(None),
            Err(error) => {
                if !matches!(*phase, Phase::Withdrawn(_)) {
                    *phase = Phase::Spent;
                }
                drop(phase);
                drop(self.connection.take());
                Err(error)
            }
            Ok(true) => {
                *phase = Phase::Verifying;
                let complete: Connection<'a> =
                    self.connection.take().ok_or(ExchangeError::Spent)?;
                drop(phase);
                complete.verify(self.authority, self.cutoff).map(Some)
            }
        }
    }
}

/// Fixture hook for the reservation before the actual worker dispatch claim.
#[cfg(test)]
pub(in crate::roughtime) type BeforeConnectDispatch = Box<dyn FnOnce() -> io::Result<()> + Send>;

/// The real connector and its optional per-attempt fixture hook.
#[cfg(test)]
pub(in crate::roughtime) struct ConnectFixture<C> {
    /// Where the worker would actually connect.
    pub(in crate::roughtime) connect: C,
    /// Gate before the worker's dispatch clock sample and claim.
    pub(in crate::roughtime) before_dispatch: Option<BeforeConnectDispatch>,
}

/// Borrow the round inputs while one owned native job establishes its stream.
pub(in crate::roughtime) struct PendingConnect<'a> {
    /// Process-owned thread and exclusive destination for its returned stream.
    job: NativeJob<Result<TcpStream, ExchangeError>>,
    /// Original attempt whose preparation is already charged.
    authority: &'a RoundAuthority,
    /// Same absolute cutoff on both sides of the worker handoff.
    cutoff: Instant,
    /// Request prepared from the preceding accepted reply before dispatch.
    request: OriginalRequest<'a>,
    /// Exact logical peer floor that observes this worker's settlement.
    retry: &'a mut RetryState,
}

impl<'a> PendingConnect<'a> {
    /// Start one bounded connector. The caller retains its outer effect guards.
    ///
    /// # Errors
    /// Refuses unready retry state, original preparation, or native lane admission.
    pub(in crate::roughtime) fn start(
        authority: &'a RoundAuthority,
        peer: SocketAddr,
        root: &'a RootKey,
        cutoff: Instant,
        history: &[Receipt],
        retry: &'a mut RetryState,
        #[cfg(not(test))] connect: impl FnOnce(SocketAddr, Duration) -> io::Result<TcpStream>
        + Send
        + 'static,
        #[cfg(test)] fixture: ConnectFixture<
            impl FnOnce(SocketAddr, Duration) -> io::Result<TcpStream> + Send + 'static,
        >,
    ) -> Result<Self, ExchangeError> {
        #[cfg(test)]
        let ConnectFixture {
            connect,
            before_dispatch,
        } = fixture;
        retry.prepare_worker(Instant::now())?;
        let request: OriginalRequest<'a> = prepare(
            authority,
            peer,
            root,
            cutoff,
            history,
            ExchangeTransport::Tcp,
        )?;
        let job: NativeJob<Result<TcpStream, ExchangeError>> = NativeJob::start(
            "nevrotick-rgt-connect",
            move |dispatch: DispatchAdmission| {
                #[cfg(test)]
                if let Some(wait) = before_dispatch {
                    wait().map_err(|_| ExchangeError::ConnectWorker)?;
                }
                let before: Instant = Instant::now();
                if before >= cutoff {
                    return Err(ExchangeError::Expired);
                }
                if !dispatch.admit_at(before) {
                    return Err(ExchangeError::Spent);
                }
                connect_checked(peer, cutoff, before, connect)
            },
        )
        .map_err(worker_error)?;
        retry.track_worker(job.witness());
        Ok(Self {
            job,
            authority,
            cutoff,
            request,
            retry,
        })
    }

    /// Give the original invocation and round authority the same dispatch latch.
    pub(in crate::roughtime) fn witness(&self) -> JoinWitness {
        self.job.witness()
    }

    /// Collect only after the process owner has joined the exact native worker.
    pub(in crate::roughtime) fn try_join(&mut self) -> Poll<Result<TcpStream, ExchangeError>> {
        self.job.try_join().map(|outcome| {
            outcome
                .map_err(worker_error)
                .and_then(std::convert::identity)
        })
    }

    /// Adopt the joined stream while the caller holds the original outer guards.
    ///
    /// # Errors
    /// Refuses original attempt authority, cutoff or missing retry lease.
    pub(in crate::roughtime) fn adopt(
        self,
        stream: TcpStream,
    ) -> Result<PendingTcpExchange<'a>, ExchangeError> {
        let retry: RetryLease<'a> = self.retry.adopt_worker()?;
        let current: Result<(), ExchangeError> = self.authority.phase().and_then(|phase| {
            require_phase(*phase, Phase::Preparing)?;
            if Instant::now() >= self.cutoff {
                return Err(ExchangeError::Expired);
            }
            Ok(())
        });
        if let Err(error) = current {
            drop(stream);
            drop(retry);
            return Err(error);
        }
        Ok(PendingTcpExchange::adopt(
            self.authority,
            self.cutoff,
            stream,
            self.request,
            Some(retry),
        ))
    }
}

/// Map worker admission and join failures without exposing native panic data.
fn worker_error(error: NativeJobError) -> ExchangeError {
    match error {
        NativeJobError::Busy => ExchangeError::ConnectBusy,
        NativeJobError::Failed => ExchangeError::ConnectWorker,
    }
}

/// Connect one exact numeric alias with the native timeout.
///
/// # Errors
/// Returns the original native socket or timeout failure.
pub(in crate::roughtime) fn connect_native(
    peer: SocketAddr,
    remaining: Duration,
) -> io::Result<TcpStream> {
    TcpStream::connect_timeout(&peer, remaining)
}

/// Submit a charged connect and configure the exact returned stream before transfer.
///
/// # Errors
/// Refuses native failure, capture disorder, late return or a foreign connection.
fn connect_checked(
    peer: SocketAddr,
    cutoff: Instant,
    submitted_at: Instant,
    connect: impl FnOnce(SocketAddr, Duration) -> io::Result<TcpStream>,
) -> Result<TcpStream, ExchangeError> {
    let before: Instant = Instant::now();
    if before < submitted_at {
        return Err(ExchangeError::CaptureOrder);
    }
    let remaining: Duration = cutoff
        .checked_duration_since(before)
        .filter(|duration| !duration.is_zero())
        .ok_or(ExchangeError::Expired)?;
    let connected: io::Result<TcpStream> = connect(peer, remaining);
    let after: Instant = Instant::now();
    if after < before {
        return Err(ExchangeError::CaptureOrder);
    }
    if after >= cutoff {
        return Err(ExchangeError::Expired);
    }
    let stream: TcpStream = connected.map_err(|error| socket_error(&error))?;
    configure_connection(&stream, peer)?;
    let ready_at: Instant = Instant::now();
    if ready_at < after {
        return Err(ExchangeError::CaptureOrder);
    }
    if ready_at >= cutoff {
        return Err(ExchangeError::Expired);
    }
    Ok(stream)
}

/// Check the actual peer and enable nonblocking stream I/O.
///
/// # Errors
/// Refuses a foreign route or unavailable native socket setting.
fn configure_connection(stream: &TcpStream, peer: SocketAddr) -> Result<(), ExchangeError> {
    let remote: SocketAddr = stream.peer_addr().map_err(|error| socket_error(&error))?;
    let local: SocketAddr = stream.local_addr().map_err(|error| socket_error(&error))?;
    if remote != peer || !local.ip().is_loopback() {
        return Err(ExchangeError::ForeignPeer);
    }
    stream
        .set_nonblocking(true)
        .map_err(|error| socket_error(&error))
}

/// Exclusive stream, immutable original request, and bounded I/O progress.
struct Connection<'a> {
    /// Selected loopback connection taken until the one response completes.
    stream: TcpStream,
    /// Original authenticated-response correlation input.
    request: OriginalRequest<'a>,
    /// Count of successfully submitted request bytes.
    written: usize,
    /// Bracket of actual successful request writes.
    sent: Option<Capture>,
    /// Exactly one response and no subsequent frame bytes.
    response: [u8; RECEIVE_BYTES],
    /// Count of bytes actually read into the response.
    received_bytes: usize,
    /// Bracket of actual successful response reads.
    received: Option<Capture>,
    /// First the fixed prefix, then the checked length declared in that prefix.
    target: usize,
    /// Original aggregate allowance for `WouldBlock` and `Interrupted` calls.
    stalls_left: u16,
    /// Original connection charge, released after the stream on Drop.
    retry: Option<RetryLease<'a>>,
}

impl<'a> Connection<'a> {
    /// Perform one active I/O step. True indicates a complete frame.
    ///
    /// # Errors
    /// Refuses the current authority, expired or disordered captures, I/O errors,
    /// exhausted idle allowance, premature EOF or an invalid frame prefix.
    fn advance(&mut self, phase: &mut Phase, cutoff: Instant) -> Result<bool, ExchangeError> {
        let writing: bool = self.written < self.request.frame.len();
        require_phase(
            *phase,
            if writing {
                Phase::Preparing
            } else {
                Phase::Awaiting
            },
        )?;
        let prior: Option<Capture> = if writing {
            self.sent
        } else {
            self.received.or(self.sent)
        };
        let before: Instant = Instant::now();
        if prior.is_some_and(|capture| before < capture.after) {
            return Err(ExchangeError::CaptureOrder);
        }
        if before >= cutoff {
            return Err(ExchangeError::Expired);
        }
        let result: io::Result<usize> = if writing {
            self.stream.write(&self.request.frame[self.written..])
        } else {
            self.stream
                .read(&mut self.response[self.received_bytes..self.target])
        };
        let after: Instant = Instant::now();
        if after < before {
            return Err(ExchangeError::CaptureOrder);
        }
        if after >= cutoff {
            return Err(ExchangeError::Expired);
        }
        let count: usize = match result {
            Ok(0) => return Err(ExchangeError::IncompleteFrame),
            Ok(count) => count,
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                ) =>
            {
                self.stalls_left -= 1;
                return if self.stalls_left == 0 {
                    Err(ExchangeError::ReceiveBudget)
                } else {
                    Ok(false)
                };
            }
            Err(error) => return Err(socket_error(&error)),
        };
        if writing {
            self.sent = Some(Capture {
                before: self.sent.map_or(before, |span| span.before),
                after,
            });
            self.written += count;
            if self.written == self.request.frame.len() {
                *phase = Phase::Awaiting;
            }
            return Ok(false);
        }
        self.received = Some(Capture {
            before: self.received.map_or(before, |span| span.before),
            after,
        });
        self.received_bytes += count;
        if self.received_bytes == FRAME_HEADER_BYTES {
            let header: &[u8; FRAME_HEADER_BYTES] = self
                .response
                .first_chunk()
                .ok_or(ExchangeError::Truncated)?;
            self.target = response_frame_length(header).map_err(|error| match error {
                DecodeError::FrameLimit => ExchangeError::Truncated,
                error => ExchangeError::Verification(VerifyError::Response(error)),
            })?;
        }
        Ok(self.received_bytes == self.target)
    }

    /// Retire the original connection before verifying the complete frame.
    ///
    /// # Errors
    /// Refuses incomplete captures, stale or unauthenticated evidence.
    fn verify(
        self,
        authority: &'a RoundAuthority,
        cutoff: Instant,
    ) -> Result<StagedReceipt<'a>, ExchangeError> {
        drop(self.stream);
        let mut staged: StagedReceipt<'a> = StagedReceipt::verify(
            authority,
            cutoff,
            &self.request,
            self.sent.ok_or(ExchangeError::CaptureOrder)?,
            &self.response,
            self.received_bytes,
            self.received.ok_or(ExchangeError::CaptureOrder)?,
        )?;
        staged.retry = self.retry;
        Ok(staged)
    }
}

#[cfg(test)]
mod tests {
    use super::{ExchangeError, PendingTcpExchange, Phase, RootKey, RoundAuthority};
    use ed25519_dalek::SigningKey;
    use std::{
        error::Error,
        io::Read,
        net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
        sync::MutexGuard,
        time::Duration,
    };

    /// Exercise a refused authority acquisition on an actual owned stream.
    ///
    /// # Errors
    /// Returns an unexpected fixture, authority or socket failure.
    ///
    /// # Panics
    /// Panics if the refused poll leaves the connection live or permits a later poll.
    #[test]
    fn closes_original_stream_when_guard_acquisition_is_refused() -> Result<(), Box<dyn Error>> {
        let budget: Duration = Duration::from_secs(2);
        let listener: TcpListener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
        let endpoint: SocketAddr = listener.local_addr()?;
        let client: TcpStream = TcpStream::connect_timeout(&endpoint, budget)?;
        listener.set_nonblocking(true)?;
        let (mut server, _): (TcpStream, SocketAddr) = listener.accept()?;
        server.set_read_timeout(Some(budget))?;
        let signer: SigningKey = SigningKey::from_bytes(&[39; 32]);
        let root: RootKey = RootKey::from_bytes(signer.verifying_key().as_bytes())
            .map_err(|error| format!("unexpected fixture root refusal: {error:?}"))?;
        let authority: RoundAuthority = RoundAuthority::new();
        let mut pending: PendingTcpExchange<'_> =
            PendingTcpExchange::start(&authority, client, endpoint, &root, budget)
                .map_err(|error| format!("unexpected fixture start refusal: {error:?}"))?;
        let held: MutexGuard<'_, Phase> = authority
            .phase()
            .map_err(|error| format!("unexpected fixture guard refusal: {error:?}"))?;
        assert!(matches!(
            pending.poll(),
            Err(ExchangeError::AuthorityUnavailable)
        ));
        let mut unexpected: [u8; 1] = [0];
        assert_eq!(server.read(&mut unexpected)?, 0);
        drop(held);
        assert!(matches!(pending.poll(), Err(ExchangeError::Spent)));
        Ok(())
    }
}
