// SPDX-License-Identifier: GPL-2.0-only
//! One private protected UDP operation and its earned same-association successor.
//!
//! The native backend owns its socket through synchronous syscall returns. A Linux
//! loopback fixture and the deterministic backend use the same receiving owner.
//! Neither historical completion nor software-clock captures confer eligibility.

use std::io;

#[cfg(any(windows, test))]
#[cfg(test)]
use crate::clock_capture;

/// Closed adapter failures; never retain packet, endpoint or library-error text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DatagramError {
    /// A nested runtime cannot own this synchronous invocation.
    #[cfg(any(windows, test))]
    EnteredRuntime,
    /// Current-thread runtime construction failed.
    Runtime(io::ErrorKind),
    /// One bounded loopback socket could not be bound.
    #[cfg(any(windows, all(test, target_os = "linux")))]
    Bind(io::ErrorKind),
    /// The socket could not become nonblocking.
    #[cfg(any(windows, all(test, target_os = "linux")))]
    Nonblocking(io::ErrorKind),
    /// The actual local endpoint could not be captured/admitted.
    #[cfg(any(windows, all(test, target_os = "linux")))]
    Local(io::ErrorKind),
    /// Tokio could not register the owned nonblocking socket.
    #[cfg(any(windows, all(test, target_os = "linux")))]
    Registration(io::ErrorKind),
    /// A readiness wait failed without establishing datagram consumption.
    Readiness(io::ErrorKind),
    /// A receive failed without establishing datagram consumption.
    Receive(io::ErrorKind),
    /// The original finite readiness/receive-attempt allowance is exhausted.
    Attempts,
    /// Actual operation captures could not form a coherent historical record.
    Capture,
}

#[cfg(any(windows, test))]
/// Concrete socket operation and its deterministic receiving neighbors.
mod reached {
    use std::io;
    use std::net::SocketAddr;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, OnceLock};
    use std::time::{Duration, Instant};

    #[cfg(any(windows, all(test, target_os = "linux")))]
    use tokio::net::UdpSocket;
    use zeroize::Zeroizing;

    use super::DatagramError;
    #[cfg(windows)]
    use crate::ProtectedNativeClockObservation;
    use crate::clock_capture::ApplicationClockCapture as ProtectedClockCapture;
    use crate::nts_ke_experiment::association::AssociationOwner;
    #[cfg(test)]
    use crate::nts_ke_experiment::association::ledger::AppliedProtectedControl;
    use crate::nts_ke_experiment::association::ledger::{
        CookieReplacement, LedgerError, PendingSubmission, ProtectedControlLeg, Receipt,
        RequestInput, RequestPhase, RetiredCommit, SendOutcome, StagedProtectedControl,
        StagedProtectedSuccessor, Ticket,
    };
    use crate::nts_ke_experiment::job::{NormalOperation, ReceivingOutcome};
    use crate::nts_ke_experiment::profile::CertificateTimeProvenance;
    use crate::nts_ke_experiment::recovery::BoundProtectedReceiver;
    use crate::nts_ke_experiment::{
        Control, DatagramAuthorityGuard, Failure, InvocationCompletion, IoPhase,
        ManagedSuccessorTerminal, RunFailed, Summary,
    };
    use crate::source_policy::{LivePolicy, ProtectedEndpoint, ProtectedSendStamp};
    use crate::{
        ExperimentalMeasurement, ExperimentalProfile, MeasurementRefusal, RunClockContext,
        SoftwareClock, SourceConfigurationSnapshot,
    };

    /// One counter covers policy waits, readiness waits and nonblocking receive attempts.
    const ATTEMPTS: usize = 64;
    /// An extra octet detects every successful receive above the admitted limit.
    const RECEIVE_BYTES: usize = 4097;

    /// Local send and shared driver observations carried through the worker join.
    /// The sole admitted worker records effects. The joined reader can inspect the
    /// final observations after synchronization, including after worker unwind.
    pub(in super::super) struct ProtectedDatagramWork {
        /// Entries into the concrete send adapter, including failed or short returns.
        attempted_sends: AtomicUsize,
        /// Adapter returns that report the entire requested datagram length.
        fully_submitted_sends: AtomicUsize,
        /// Most recent unused allowance from the actual shared driver owner.
        remaining_driver_work: AtomicUsize,
        /// First admitted NAK observed in the actual pending receive path.
        first_nak_at: OnceLock<Instant>,
    }

    impl ProtectedDatagramWork {
        /// Record the admitted starting state before any backend or driver work.
        pub(in super::super) const fn new() -> Self {
            Self {
                attempted_sends: AtomicUsize::new(0),
                fully_submitted_sends: AtomicUsize::new(0),
                remaining_driver_work: AtomicUsize::new(ATTEMPTS),
                first_nak_at: OnceLock::new(),
            }
        }

        /// Read attempted sends, full local submissions and unused driver credits, in order.
        /// The caller must already have joined the worker for a final observation.
        pub(in super::super) fn snapshot(&self) -> (usize, usize, usize) {
            (
                self.attempted_sends.load(Ordering::Relaxed),
                self.fully_submitted_sends.load(Ordering::Relaxed),
                self.remaining_driver_work.load(Ordering::Relaxed),
            )
        }

        /// Preserve the first admitted mark after an actual join, including unwind.
        pub(in super::super) fn first_nak_at(&self) -> Option<Instant> {
            self.first_nak_at.get().copied()
        }
    }

    /// Preserve a closed adapter failure without formatting its source.
    const fn failed(error: DatagramError) -> Failure {
        Failure::Datagram(error)
    }

    /// No reset is possible after a wake, refusal or received packet.
    pub(in super::super) struct Attempts(usize);

    impl Attempts {
        /// Consume before the bounded operation, including a spurious wake.
        /// # Errors
        /// Refuse attempt 65 without performing it.
        fn spend(&mut self, control: &Control) -> Result<(), Failure> {
            self.0 = self.after_one()?;
            if let Some(work) = control.datagram_work.as_deref() {
                work.remaining_driver_work.store(self.0, Ordering::Relaxed);
            }
            Ok(())
        }

        /// Check that one operation can run without spending its credit.
        /// # Errors
        /// Refuse an exhausted original allowance.
        fn after_one(&self) -> Result<usize, Failure> {
            self.0.checked_sub(1).ok_or(failed(DatagramError::Attempts))
        }

        /// Inspect the exact unused original driver credits in component controls.
        #[cfg(test)]
        const fn remaining(&self) -> usize {
            self.0
        }
    }

    /// The only two readiness classes in this one-operation owner.
    #[derive(Clone, Copy)]
    pub(super) enum Interest {
        /// Preparation has finished but no adapter has been invoked.
        Write,
        /// Exact full local submission established Pending.
        Read,
    }

    /// One selected native return; unknown extents never expose a prefix.
    pub(super) enum Read {
        /// Complete selected bytes/source, still requiring all packet guards.
        Datagram(usize, SocketAddr),
        /// Readiness was spurious; no datagram consumption is established.
        WouldBlock,
        /// A datagram was consumed but its complete extent is unavailable.
        ConsumedUnknown,
        /// No datagram consumption is established by this error.
        Failed(io::ErrorKind),
    }

    /// A native socket is never cloned, exported or reconstructed from endpoints.
    pub(super) enum Io {
        /// Sole runtime-registered native socket.
        #[cfg(any(windows, all(test, target_os = "linux")))]
        Native(UdpSocket),
        /// Closed public-material component cases; never a TLS/native substitute.
        #[cfg(test)]
        Script(Box<Script>),
    }

    /// Socket and immutable endpoint identity stay together through retirement.
    pub(super) struct Backend {
        /// Sole owned native socket or closed component script, never cloned.
        pub(super) io: Io,
        /// Captured actual local endpoint; it supplies no independent socket identity.
        pub(super) local: SocketAddr,
        /// Immutable association-admitted destination for this whole operation.
        pub(super) remote: SocketAddr,
        /// Original socket-instance identity retained by historical completion.
        pub(super) identity: Arc<()>,
        /// Exact before/after software captures and expected complete packet length.
        pub(super) send: Option<(ProtectedClockCapture, ProtectedClockCapture, usize)>,
    }

    /// Bind the reported local endpoint to the original ephemeral bind request.
    /// The allocated port must be nonzero, and the peer must remain loopback.
    /// # Errors
    /// Refuse a substituted local address, zero bound port, or non-loopback peer.
    #[cfg(any(windows, test))]
    fn check_bound_endpoints(
        requested_local: SocketAddr,
        local: SocketAddr,
        remote: SocketAddr,
    ) -> Result<SocketAddr, Failure> {
        if !remote.ip().is_loopback() {
            return Err(Failure::Endpoint);
        }
        crate::checked_local_endpoint(requested_local, local).map_err(|_| Failure::Endpoint)
    }

    impl Backend {
        /// Bind and register exactly one actual same-family loopback socket.
        /// # Errors
        /// Preserve bounded bind, nonblocking, local-endpoint and registration causes.
        #[cfg(any(windows, all(test, target_os = "linux")))]
        pub(super) fn bind(remote: SocketAddr) -> Result<Self, Failure> {
            use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, UdpSocket as StdSocket};

            let ip: IpAddr = match remote {
                SocketAddr::V4(_) => IpAddr::V4(Ipv4Addr::LOCALHOST),
                SocketAddr::V6(_) => IpAddr::V6(Ipv6Addr::LOCALHOST),
            };
            let requested_local: SocketAddr = SocketAddr::new(ip, 0);
            let socket: StdSocket = StdSocket::bind(requested_local)
                .map_err(|error| failed(DatagramError::Bind(error.kind())))?;
            socket
                .set_nonblocking(true)
                .map_err(|error| failed(DatagramError::Nonblocking(error.kind())))?;
            let observed: SocketAddr = socket
                .local_addr()
                .map_err(|error| failed(DatagramError::Local(error.kind())))?;
            let local: SocketAddr = check_bound_endpoints(requested_local, observed, remote)?;
            let socket: UdpSocket = UdpSocket::from_std(socket)
                .map_err(|error| failed(DatagramError::Registration(error.kind())))?;
            Ok(Self {
                io: Io::Native(socket),
                local,
                remote,
                identity: Arc::new(()),
                send: None,
            })
        }

        /// Wait only for readiness; the caller retains currentness/deadline checks.
        /// # Errors
        /// Preserve readiness failure without fabricating a selected datagram.
        async fn ready(&mut self, interest: Interest, control: &Control) -> Result<(), Failure> {
            control.remaining(IoPhase::Exchange)?;
            match &mut self.io {
                #[cfg(any(windows, all(test, target_os = "linux")))]
                Io::Native(socket) => match interest {
                    Interest::Write => socket.writable().await,
                    Interest::Read => socket.readable().await,
                }
                .map_err(|error| failed(DatagramError::Readiness(error.kind()))),
                #[cfg(test)]
                Io::Script(script) => script.ready(interest, control).await,
            }
        }

        /// Route one prepared ledger request to this concrete synchronous backend.
        /// # Errors
        /// Preserve the ledger, actual authority and adapter refusals.
        pub(super) fn send_prepared(
            &mut self,
            association: &mut AssociationOwner<'_>,
            ticket: Ticket<'_>,
            control: &Control,
        ) -> Result<(), Failure> {
            #[cfg(test)]
            association.check_send_owner(&ticket, control)?;
            #[cfg(test)]
            match &mut self.io {
                #[cfg(any(windows, all(test, target_os = "linux")))]
                Io::Native(_) => {}
                Io::Script(script) => script.before_guard(control)?,
            }
            let lease: Option<ProtectedEndpoint> = association.protected_endpoint()?.cloned();
            let guard: DatagramAuthorityGuard<'_> =
                DatagramAuthorityGuard::lock(control, lease.as_ref())?;
            let sender: GuardedSender<'_, '_> = GuardedSender {
                backend: self,
                guard,
            };
            association.invoke_guarded(ticket, sender)
        }

        /// Capture the original send boundary using the selected actual adapter.
        fn capture_send_clock(&self) -> ProtectedClockCapture {
            match &self.io {
                #[cfg(any(windows, all(test, target_os = "linux")))]
                Io::Native(_) => ProtectedClockCapture::capture(),
                #[cfg(test)]
                Io::Script(_) => ProtectedClockCapture::software_only(SoftwareClock::capture()),
            }
        }

        /// Observe one synchronous return before interpreting any packet bytes.
        fn receive(&mut self, buffer: &mut [u8; RECEIVE_BYTES]) -> (Read, ProtectedClockCapture) {
            match &mut self.io {
                #[cfg(any(windows, all(test, target_os = "linux")))]
                Io::Native(socket) => {
                    let result = socket.try_recv_from(buffer);
                    let observed: ProtectedClockCapture = ProtectedClockCapture::capture();
                    (classify_read(result), observed)
                }
                #[cfg(test)]
                Io::Script(script) => {
                    let result: Read = script.receive(buffer);
                    let observed: SoftwareClock = SoftwareClock::capture();
                    let capture: ProtectedClockCapture =
                        ProtectedClockCapture::software_only(script.observe_read_return(observed));
                    #[cfg(windows)]
                    let capture: ProtectedClockCapture =
                        capture.with_scripted_refusal(script.clock_refusals[2]);
                    (result, capture)
                }
            }
        }
    }

    /// Actual backend and scope-then-phase custody for one consuming send.
    /// Only the concrete datagram backend constructs this value.
    pub(in super::super) struct GuardedSender<'backend, 'control> {
        /// Exclusive original socket owner, including any retained send capture.
        backend: &'backend mut Backend,
        /// The same actual gates remain held through archive, send and return capture.
        guard: DatagramAuthorityGuard<'control>,
    }

    impl GuardedSender<'_, '_> {
        /// Borrow the actual Control for the ledger's same-owner check.
        #[must_use]
        pub(in super::super) const fn control(&self) -> &Control {
            self.guard.control.control
        }

        /// Borrow the actual socket instance for Pending correlation.
        pub(in super::super) const fn request_identity(&self) -> &Arc<()> {
            &self.backend.identity
        }

        /// Check the actual original authority at a fresh archive observation.
        /// # Errors
        /// Preserve stopped, non-Published and expired authority refusal.
        pub(in super::super) fn archive_entry(&self) -> Result<Instant, Failure> {
            let now: Instant = Instant::now();
            self.guard.control.published(now)?;
            self.guard.endpoint(self.backend.remote)?;
            Ok(now)
        }

        /// Consume this admitted adapter once after the owning ledger archives.
        ///
        /// No arbitrary callback, allocation, formatter, crypto or await runs
        /// under this guard. The closed test branch returns predeclared facts.
        /// # Errors
        /// Preserve endpoint, repeat, capture and original-deadline refusal
        /// before the syscall, without reversing the ledger's earlier archive.
        pub(in super::super) fn submit(
            self,
            packet: &[u8],
            remote: SocketAddr,
        ) -> Result<(SendOutcome, Option<ProtectedSendStamp>), Failure> {
            let Self { backend, mut guard } = self;
            if remote != backend.remote || backend.send.is_some() {
                return Err(Failure::Endpoint);
            }
            let before: ProtectedClockCapture = backend.capture_send_clock();
            #[cfg(test)]
            let before: ProtectedClockCapture = match &backend.io {
                #[cfg(any(windows, all(test, target_os = "linux")))]
                Io::Native(_) => before,
                Io::Script(script) => {
                    let capture: ProtectedClockCapture = ProtectedClockCapture::software_only(
                        script.observe_send_before(before.software()),
                    );
                    #[cfg(windows)]
                    let capture: ProtectedClockCapture =
                        capture.with_scripted_refusal(script.clock_refusals[0]);
                    capture
                }
            };
            let bracket: SoftwareClock = before.software();
            check_capture_order(bracket.monotonic(), bracket.monotonic_after())?;
            guard.control.published(bracket.monotonic_after())?;
            let stamp: Option<ProtectedSendStamp> =
                guard.reserve(remote, &backend.identity, bracket.monotonic_after())?;
            let work: Option<&ProtectedDatagramWork> =
                guard.control.control.datagram_work.as_deref();
            if let Some(work) = work {
                // The initial operation and single earned continuation bound this addition.
                let _previous: usize = work.attempted_sends.fetch_add(1, Ordering::Relaxed);
            }
            let (outcome, cancel_after, native_after): (
                SendOutcome,
                bool,
                Option<ProtectedClockCapture>,
            ) = match &mut backend.io {
                #[cfg(any(windows, all(test, target_os = "linux")))]
                Io::Native(socket) => {
                    let result: io::Result<usize> = socket.try_send_to(packet, remote);
                    let after: ProtectedClockCapture = ProtectedClockCapture::capture();
                    let outcome: SendOutcome = match result {
                        Ok(length) => SendOutcome::Completed(length),
                        Err(error) => SendOutcome::Failed(error.kind()),
                    };
                    (outcome, false, Some(after))
                }
                #[cfg(test)]
                Io::Script(script) => {
                    let (outcome, cancel_after): (SendOutcome, bool) =
                        script.send(packet, guard.control.control);
                    (outcome, cancel_after, None)
                }
            };
            if let Some(work) = work
                && matches!(outcome, SendOutcome::Completed(length) if length == packet.len())
            {
                let _previous: usize = work.fully_submitted_sends.fetch_add(1, Ordering::Relaxed);
            }
            // The scripted backend keeps its original capture after send accounting.
            let after: ProtectedClockCapture =
                native_after.unwrap_or_else(|| backend.capture_send_clock());
            let control: &Control = guard.control.control;
            drop(guard);
            backend.send = Some((before, after, packet.len()));
            // Preserve completed-send custody before any fixture refusal or Stop.
            #[cfg(test)]
            match &backend.io {
                #[cfg(any(windows, all(test, target_os = "linux")))]
                Io::Native(_) => {}
                Io::Script(script) => {
                    let after: SoftwareClock = script.observe_send_after(after.software())?;
                    let capture: ProtectedClockCapture =
                        ProtectedClockCapture::software_only(after);
                    #[cfg(windows)]
                    let capture: ProtectedClockCapture =
                        capture.with_scripted_refusal(script.clock_refusals[1]);
                    backend.send = Some((before, capture, packet.len()));
                }
            }
            if cancel_after {
                control.cancel()?;
            }
            Ok((outcome, stamp))
        }
    }

    /// Translate a native Windows receive return without observing the socket.
    /// Synthetic returns can exercise the same decision on other test hosts.
    pub(super) fn classify_read(result: io::Result<(usize, SocketAddr)>) -> Read {
        match result {
            Ok((RECEIVE_BYTES, _)) => Read::ConsumedUnknown,
            Ok((length, source)) => Read::Datagram(length, source),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => Read::WouldBlock,
            Err(error) if error.raw_os_error() == Some(10040) => Read::ConsumedUnknown,
            Err(error) => Read::Failed(error.kind()),
        }
    }

    /// Check inclusive ordering of two already captured monotonic coordinates.
    ///
    /// Callers check the pre-send bracket and the gaps between send and read
    /// captures. They separately enforce the original deadline and Control.
    /// # Errors
    /// Refuse reversed observations; equal coordinates remain a valid neighbor.
    pub(super) fn check_capture_order(
        before_end: Instant,
        after_start: Instant,
    ) -> Result<(), Failure> {
        if before_end > after_start {
            return Err(failed(DatagramError::Capture));
        }
        Ok(())
    }

    /// Original packet admission and captures, with no cookie-transfer authority.
    pub(super) struct OperationHistory<'control> {
        /// Same live control checked against the association before any packet work.
        control: &'control Control,
        /// Genuine once-consumed authenticated receipt, without eligibility authority.
        pub(super) receipt: Receipt,
        /// Software observations immediately before the concrete send attempt.
        pub(super) send_before: SoftwareClock,
        /// Software observations immediately after that synchronous return.
        pub(super) send_after: SoftwareClock,
        /// Actual selected receive-return observations, captured before parsing.
        pub(super) read_return: SoftwareClock,
        /// Native read outcomes at the same three original adapter boundaries.
        #[cfg(windows)]
        native_clock: [Option<ProtectedNativeClockObservation>; 3],
        /// Captured bound endpoint from the original owned socket.
        pub(super) local: SocketAddr,
        /// Same immutable association endpoint that received the submitted packet.
        pub(super) remote: SocketAddr,
        /// Actual complete protected request extent, never a fixed 48-byte assumption.
        pub(super) packet_bytes: usize,
        /// Checked resulting pool entries for this reply's replacement.
        pub(super) cookies: usize,
        /// Corresponding checked aggregate opaque pool payload, without copying bytes.
        pub(super) cookie_bytes: usize,
    }

    /// Raw operation result. Only the retiring wrapper can issue a protected completion.
    pub(super) struct OperationCompletion<'control> {
        /// Original packet admission, even if final acceptance later fails.
        pub(super) history: OperationHistory<'control>,
        /// Private preallocated pool transfer, unavailable to a raw history observer.
        replacement: CookieReplacement,
        /// Original Pending submission, surviving socket/runtime retirement.
        submission: PendingSubmission,
    }

    /// Opaque receiving input, constructed only after backend/runtime retirement.
    /// Neither historical receipts nor raw operation results can construct it.
    pub(in super::super) struct ProtectedCompletion<'control> {
        /// Same genuine operation, with its original context and complete history.
        operation: OperationCompletion<'control>,
        /// Driver credits left by the original runtime for one earned continuation.
        attempts: Attempts,
    }

    /// The actual control transfer after both receive resources retired.
    /// Pending remains with the ledger until its guarded final commit.
    #[must_use = "observe the retired control through its original owner"]
    pub(in super::super) struct RetiredProtectedControl<'control>(StagedProtectedControl<'control>);

    /// A protected response after the shared driver and backend retire.
    #[must_use = "consume the response through its original association"]
    #[expect(
        clippy::large_enum_variant,
        reason = "keep the bounded returned history and cookie transfer inline after retirement"
    )]
    pub(in super::super) enum RetiredProtectedResponse<'control> {
        /// A complete ordinary reply with the existing final commit boundary.
        Ordinary(ProtectedCompletion<'control>),
        /// An exact control with its Pending request still held.
        Control(RetiredProtectedControl<'control>),
    }

    /// Retired candidate and independent receive observation, awaiting final commit.
    pub(in super::super) struct ObservedRetiredSuccessor<'control> {
        /// Exact retired candidate from the original pending operation.
        candidate: RetiredProtectedResponse<'control>,
        /// Original first NAK, retained if final commit later refuses.
        first_nak_at: Option<Instant>,
    }

    impl<'control> ObservedRetiredSuccessor<'control> {
        /// Transfer the actual candidate and its separate observation to the receiver.
        pub(in super::super) fn into_parts(
            self,
        ) -> (RetiredProtectedResponse<'control>, Option<Instant>) {
            (self.candidate, self.first_nak_at)
        }
    }

    /// Original successor refusal beside an independent, historical recovery observation.
    pub(in super::super) struct SuccessorFailure {
        /// The actual terminal cause of the selected operation.
        cause: Failure,
        /// First current NTSN during the pending opportunity. The one-KE plan cannot recover.
        first_nak_at: Option<Instant>,
    }

    impl SuccessorFailure {
        /// Retain the original cause even when a NAK was present.
        pub(in super::super) const fn cause(&self) -> Failure {
            self.cause
        }

        /// Give the original NAK arrival without scheduling recovery or rewriting failure.
        pub(in super::super) const fn first_nak_at(&self) -> Option<Instant> {
            self.first_nak_at
        }
    }

    impl<'control> RetiredProtectedControl<'control> {
        /// Transfer the exact already-retired request to its receiving ledger.
        pub(in super::super) fn into_staged(self) -> StagedProtectedControl<'control> {
            self.0
        }

        /// Publish the exact protected control through its original receiving owner.
        /// # Errors
        /// Preserve any currentness, endpoint or final guarded policy refusal.
        #[cfg(test)]
        pub(super) fn commit(
            self,
            association: &mut AssociationOwner<'_>,
        ) -> Result<AppliedProtectedControl, Failure> {
            association.commit_retired_protected_control(self)
        }

        /// Recheck this exact retired ticket without publishing any control effects.
        /// # Errors
        /// Preserve wrong-owner, stale-currentness, Stop and expiry refusals.
        #[cfg(test)]
        pub(super) fn check_current(
            &self,
            association: &AssociationOwner<'_>,
        ) -> Result<(), Failure> {
            association.check_protected_control_current(&self.0)
        }
    }

    /// Keep detached facts and the exact ledger transfer within the retired owner.
    struct RetiredOperation<'control> {
        /// Historical records have no authority to install the replacement.
        history: OperationHistory<'control>,
        /// The actual consumed request and its guarded transfer.
        reply: RetiredReply<'control>,
        /// The original driver's unused allowance.
        attempts: Attempts,
    }

    /// One ledger transfer obtainable only from a genuinely retired operation.
    pub(in super::super) struct RetiredReply<'control> {
        /// Original invocation authority kept through the retired transfer.
        control: &'control Control,
        /// Exact original packet-admission history and spent request ordinal.
        receipt: Receipt,
        /// Prepared cookie installation held until final guarded commit.
        replacement: CookieReplacement,
        /// Original matched submission and its managed policy correlation, if issued.
        submission: PendingSubmission,
    }

    /// The components of one exclusively consumed retired reply.
    pub(in super::super) type RetiredReplyParts<'control> = (
        &'control Control,
        Receipt,
        CookieReplacement,
        PendingSubmission,
    );

    impl<'control> RetiredReply<'control> {
        /// Move the sole transfer into the ledger's original-owner transaction.
        pub(in super::super) fn into_parts(self) -> RetiredReplyParts<'control> {
            (
                self.control,
                self.receipt,
                self.replacement,
                self.submission,
            )
        }
    }

    /// One committed protected exchange, not live permission or Core eligibility.
    /// No Clone/Copy or public raw constructor can duplicate this fact.
    pub(in super::super) struct CommittedProtected<'control> {
        /// Actual history survives until its one consuming receiver projection.
        operation: OperationHistory<'control>,
        /// Original submitted socket identity, taken from the guarded ledger transfer.
        identity: Arc<()>,
        /// The exact guarded terminal processing observation.
        committed_at: Instant,
        /// Historical managed configuration, checked at both original effects.
        configuration: Option<SourceConfigurationSnapshot>,
        /// The actual publishing association's original accepted KE provenance.
        certificate_time_provenance: CertificateTimeProvenance,
    }

    /// Genuine first-leg commit that left the original invocation published.
    pub(in super::super) struct CommittedFirstProtected<'control>(CommittedProtected<'control>);

    /// Detached first history bound to the original KE Control before worker storage.
    pub(in super::super) struct BoundFirstHistory {
        /// Original managed first receipt with its pre-exchange population.
        history: Arc<ManagedProtectedHistory>,
    }

    impl BoundFirstHistory {
        /// Detach the payload only for the worker that owns this invocation.
        /// # Errors
        /// Refuse a different worker Control without recording.
        pub(in super::super) fn into_worker_history(
            self,
            control_identity: &Arc<()>,
        ) -> Result<Arc<ManagedProtectedHistory>, Failure> {
            if !Arc::ptr_eq(
                &self.history.history.ke_association_identity,
                control_identity,
            ) {
                return Err(Failure::Control);
            }
            Ok(self.history)
        }
    }

    /// Owned historical protected exchange; only consuming genuine commit creates it.
    /// Unlike the plain pilot's `TransportFacts`, peer timestamp bits may be zero.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the remaining immutable historical identities await the production Core receiver"
        )
    )]
    pub(in super::super) struct ProtectedHistory {
        /// Checked pre-operation population and actual send counts.
        summary: Summary,
        /// Genuine original authenticated header, ordinal and packet-admission time.
        receipt: Receipt,
        /// Historical identity of the exact Control that published this association.
        ke_association_identity: Arc<()>,
        /// Original KE verifier and time input, with no current trust or date claim.
        certificate_time_provenance: CertificateTimeProvenance,
        /// The guarded terminal decision, separate from the receipt age reference.
        committed_at: Instant,
        /// Checked historical pool entries/bytes, including duplicate entries.
        /// Distinct from initial KE counts; grants no availability or reuse authority.
        population: (usize, usize),
        /// Actual bracket immediately before the successful local send.
        send_before: SoftwareClock,
        /// Actual bracket immediately after that same local send.
        send_after: SoftwareClock,
        /// Original selected receive-return bracket, never refreshed at detachment.
        read_return: SoftwareClock,
        /// Native read outcomes ordered as send-before, send-return and read-return.
        #[cfg(windows)]
        native_clock: [Option<ProtectedNativeClockObservation>; 3],
        /// Actual socket's original bound endpoint.
        local: SocketAddr,
        /// Actual checked peer endpoint, without independent-source authority.
        remote: SocketAddr,
        /// Historical socket lifetime identity; this retains no handle.
        identity: Arc<()>,
        /// Original invocation chart, shared across any actual replacement context.
        run_context: Arc<RunClockContext>,
    }

    /// Keep the exact committed managed revision with its packet history in one allocation.
    pub(in super::super) struct ManagedProtectedHistory {
        /// Same genuine packet history as the unmanaged protected result.
        history: ProtectedHistory,
        /// Immutable revision checked by both the sender and final reply consumer.
        configuration: SourceConfigurationSnapshot,
    }

    impl ManagedProtectedHistory {
        /// Borrow the already committed revision, without deriving current authority.
        pub(in super::super) const fn configuration(&self) -> &SourceConfigurationSnapshot {
            &self.configuration
        }

        /// Keep the same bounded pre-operation counts and observed submission.
        pub(in super::super) const fn summary(&self) -> &Summary {
            self.history.summary()
        }

        /// Inspect original committed facts through the actual joined control.
        pub(in super::super) const fn history(&self) -> &ProtectedHistory {
            &self.history
        }
    }

    /// One normally joined result; numerical refusal is retained data, never a retry.
    /// Its private representation prevents promoting caller-supplied summaries.
    pub(in super::super) struct ReceivedCompletion {
        /// Exactly one original state admitted by the normal-join guard.
        inner: ReceivedState,
    }

    /// A protected control obtained solely through normal joined custody.
    #[cfg(test)]
    pub(in super::super) struct ReceivedProtectedControl {
        /// Already applied effect, detached from every live authority owner.
        history: AppliedProtectedControl,
    }

    #[cfg(test)]
    impl ReceivedProtectedControl {
        /// Borrow the retained historical effect without deriving permission.
        pub(in super::super) const fn history(&self) -> &AppliedProtectedControl {
            &self.history
        }

        /// Recover the detached history without its normal-join witness.
        pub(in super::super) fn into_history(self) -> AppliedProtectedControl {
            self.history
        }
    }

    /// The selected bounded path's normal retirement, original first history and work.
    pub(in super::super) struct ReceivedManagedSuccessor {
        /// Present only after a genuine first commit.
        first: Option<Arc<ManagedProtectedHistory>>,
        /// Actual terminal result with the original NAK mark.
        terminal: ManagedSuccessorTerminal,
        /// Original checked declaration shared by the two distinct ordinary receipts.
        profile: Option<ExperimentalProfile>,
        /// Attempted sends, full sends, and the original driver's remaining work.
        work: (usize, usize, usize),
        /// Raw clock reads around this original submission and joined take.
        #[cfg(windows)]
        clock_reads: Option<[super::super::ManagedClockRead; 2]>,
    }

    impl ReceivedManagedSuccessor {
        /// Retain the private observation enclosing this worker submission and take.
        #[cfg(windows)]
        pub(in super::super) const fn retain_clock_reads(
            &mut self,
            reads: [super::super::ManagedClockRead; 2],
        ) {
            self.clock_reads = Some(reads);
        }

        /// Return the exact optional pair without querying the process again.
        #[cfg(windows)]
        pub(in super::super) const fn clock_reads(
            &self,
        ) -> Option<[super::super::ManagedClockRead; 2]> {
            self.clock_reads
        }

        /// Observe the exact first packet history in joined component assertions.
        #[cfg(test)]
        pub(in super::super) fn first_history(&self) -> Option<&ManagedProtectedHistory> {
            self.first.as_deref()
        }

        /// Observe the exact terminal ordinary history in joined component assertions.
        #[cfg(all(test, any(windows, target_os = "linux")))]
        pub(in super::super) fn terminal_history(&self) -> Option<&ManagedProtectedHistory> {
            use crate::nts_ke_experiment::ManagedSuccessorEffect;

            match &self.terminal.result {
                Ok(ManagedSuccessorEffect::Ordinary(history)) => Some(history),
                Ok(ManagedSuccessorEffect::Restriction(_)) | Err(_) => None,
            }
        }

        /// Return the declaration received with the joined operation.
        pub(in super::super) const fn profile(&self) -> Option<ExperimentalProfile> {
            self.profile
        }

        /// Move the normally retired facts together, with no renewed permission.
        pub(in super::super) fn into_parts(
            self,
        ) -> (
            Option<Arc<ManagedProtectedHistory>>,
            ManagedSuccessorTerminal,
            (usize, usize, usize),
        ) {
            (self.first, self.terminal, self.work)
        }
    }

    /// One normally joined operation, with a distinct control-only case.
    #[expect(
        clippy::large_enum_variant,
        reason = "the existing fixed-size numerical probe remains inline across joined receiving"
    )]
    pub(in super::super) enum ReceivedInvocation {
        /// Existing preview and ordinary protected result modes.
        Probe(ReceivedCompletion),
        /// Historical control with no numerical or summary projection.
        #[cfg(test)]
        ProtectedControl(ReceivedProtectedControl),
        /// Explicit bounded API's distinct normally joined result.
        ManagedSuccessor(Box<ReceivedManagedSuccessor>),
    }

    /// Private mode-specific state issued only after the normal-join guard.
    #[expect(
        clippy::large_enum_variant,
        reason = "One consume-once result keeps fixed-size numerics inline without adding an allocation after completed work."
    )]
    enum ReceivedState {
        /// The actual never-send operation supplied no protected history to adapt.
        Preview(Summary),
        /// The same never-send result retains its genuine original source definition.
        ManagedPreview(Box<crate::nts_ke_experiment::handoff::ManagedPreviewHistory>),
        /// Original owning history coupled to its exact optional model and result.
        Protected(ReceivedProtectedNumerics),
    }

    /// Transfer the admitted fields needed by the joined managed report.
    pub(in super::super) type ManagedProtectedProjection = (
        SourceConfigurationSnapshot,
        Summary,
        u8,
        Instant,
        Option<Duration>,
        Option<[Duration; 2]>,
        Result<ExperimentalMeasurement, MeasurementRefusal>,
    );

    impl ReceivedCompletion {
        /// Preserve the existing CLI projection without publishing numerical authority.
        pub(in super::super) fn summary(&self) -> &Summary {
            match &self.inner {
                ReceivedState::Preview(summary) => summary,
                ReceivedState::ManagedPreview(history) => history.summary(),
                ReceivedState::Protected(received) => received.history.observation().summary(),
            }
        }

        /// Borrow the same joined owner without detaching or recomputing facts.
        pub(in super::super) const fn projection(&self) -> CompletionProjection<'_> {
            CompletionProjection { completion: self }
        }

        /// Consume exactly the managed result; diagnostic counts cannot substitute.
        /// # Errors
        /// Refuse another completed operation selection without manufacturing history.
        pub(in super::super) fn into_managed_preview(
            self,
        ) -> Result<Box<crate::nts_ke_experiment::handoff::ManagedPreviewHistory>, Failure>
        {
            match self.inner {
                ReceivedState::ManagedPreview(history) => Ok(history),
                ReceivedState::Preview(_) | ReceivedState::Protected(_) => Err(Failure::Control),
            }
        }

        /// Project only a genuine joined source-owned protected result.
        /// # Errors
        /// Refuse an unmanaged or no-send result.
        pub(in super::super) fn into_managed_protected(
            self,
        ) -> Result<ManagedProtectedProjection, Failure> {
            let ReceivedState::Protected(received) = self.inner else {
                return Err(Failure::Control);
            };
            let JoinedProtectedHistory::Managed(managed) = received.history else {
                return Err(Failure::Control);
            };
            let history: &ProtectedHistory = managed.history();
            Ok((
                managed.configuration().clone(),
                *history.summary(),
                history.receipt.fields().header().leap_indicator(),
                history.committed_at(),
                history.receive_to_commit_delay(),
                history.application_wall_elapsed_bounds(),
                received.measurement,
            ))
        }

        /// Borrow only genuine protected history for actual-owner component controls.
        #[cfg(test)]
        pub(in super::super) const fn protected(&self) -> Option<&ReceivedProtectedNumerics> {
            match &self.inner {
                ReceivedState::Preview(_) | ReceivedState::ManagedPreview(_) => None,
                ReceivedState::Protected(received) => Some(received),
            }
        }
    }

    /// Move the one original operation allocation through normal receiving.
    enum JoinedProtectedHistory {
        /// Original diagnostic protected operation.
        Diagnostic(Box<ProtectedHistory>),
        /// Explicit source-issued operation with an exact committed revision.
        Managed(Arc<ManagedProtectedHistory>),
    }

    impl JoinedProtectedHistory {
        /// Borrow the original captures and packet facts for either protected mode.
        fn observation(&self) -> &ProtectedHistory {
            match self {
                Self::Diagnostic(history) => history,
                Self::Managed(managed) => &managed.history,
            }
        }
    }

    /// Opaque diagnostic view borrowing one normally joined immutable completion.
    /// Codes describe historical experimental arithmetic, never freshness or eligibility.
    pub(in super::super) struct CompletionProjection<'a> {
        /// The sole original owner; callers cannot substitute its constituent facts.
        completion: &'a ReceivedCompletion,
    }

    impl CompletionProjection<'_> {
        /// Borrow the original mode-specific counts without changing their meaning.
        pub(in super::super) fn summary(&self) -> &Summary {
            self.completion.summary()
        }

        /// Select only the finite state of the retained original numerical result.
        pub(in super::super) const fn numerical(&self) -> &'static str {
            match &self.completion.inner {
                ReceivedState::Preview(_) | ReceivedState::ManagedPreview(_) => "not_attempted",
                ReceivedState::Protected(received) => match &received.measurement {
                    Ok(_) => "nominal_software",
                    Err(_) => "unavailable",
                },
            }
        }

        /// Return only an existing allowlisted refusal code, without error formatting.
        pub(in super::super) const fn numerical_reason(&self) -> &'static str {
            match &self.completion.inner {
                ReceivedState::Preview(_) | ReceivedState::ManagedPreview(_) => "none",
                ReceivedState::Protected(received) => match &received.measurement {
                    Ok(_) => "none",
                    Err(reason) => reason.as_str(),
                },
            }
        }
    }

    /// Immutable historical calculation, constructible only at the normal-join guard.
    /// This retains no native owner and supplies no current freshness or eligibility.
    pub(in super::super) struct ReceivedProtectedNumerics {
        /// Same complete history moved from the actual retired worker result.
        history: JoinedProtectedHistory,
        /// Exact checked declaration; no missing declaration is inferred or replaced.
        #[cfg_attr(
            not(test),
            expect(
                dead_code,
                reason = "the private first caller retains its absent model declaration"
            )
        )]
        profile: Option<ExperimentalProfile>,
        /// Success or complete refusal at original publication, without discarding history.
        measurement: Result<ExperimentalMeasurement, MeasurementRefusal>,
    }

    #[cfg(test)]
    impl ReceivedProtectedNumerics {
        /// Borrow unchanged genuine history; never split off or rebind its provenance.
        pub(in super::super) fn history(&self) -> &ProtectedHistory {
            self.history.observation()
        }

        /// Borrow the source-issued history only from a genuine joined result.
        pub(in super::super) fn managed_history(&self) -> Option<&Arc<ManagedProtectedHistory>> {
            match &self.history {
                JoinedProtectedHistory::Managed(managed) => Some(managed),
                JoinedProtectedHistory::Diagnostic(_) => None,
            }
        }

        /// Borrow only the original optional checked declaration.
        pub(in super::super) const fn profile(&self) -> Option<&ExperimentalProfile> {
            self.profile.as_ref()
        }

        /// Borrow the complete conditional result, with its original historical age.
        pub(in super::super) const fn measurement(
            &self,
        ) -> &Result<ExperimentalMeasurement, MeasurementRefusal> {
            &self.measurement
        }
    }

    /// Project only an actually collected result through the mandatory retirement guard.
    /// The facade supplies its one take after actual join. No bare-history constructor
    /// can promote abnormal retained facts. Numerical work is finite and has no effect
    /// on acquisition, spent budgets, cookies, history or the original operation result.
    /// # Errors
    /// Preserve normal operation failure or the complete abnormal joined outcome.
    pub(in super::super) fn receive_joined(
        joined: ReceivingOutcome,
        profile: Option<ExperimentalProfile>,
    ) -> Result<ReceivedInvocation, RunFailed> {
        let NormalOperation {
            operation,
            first_receipt,
            datagram_work,
        } = joined.into_normal().map_err(RunFailed::Abnormal)?;
        if let Some(work) = datagram_work {
            let terminal: ManagedSuccessorTerminal = match operation {
                Ok(InvocationCompletion::ManagedSuccessor(terminal)) => *terminal,
                Err(failure) => ManagedSuccessorTerminal {
                    result: Err(failure),
                    first_nak_at: None,
                    remaining_association_work: None,
                },
                Ok(_) => return Err(RunFailed::Operation(Failure::Control.into())),
            };
            return Ok(ReceivedInvocation::ManagedSuccessor(Box::new(
                ReceivedManagedSuccessor {
                    first: first_receipt,
                    terminal,
                    profile,
                    work,
                    #[cfg(windows)]
                    clock_reads: None,
                },
            )));
        }
        if first_receipt.is_some() {
            return Err(RunFailed::Operation(Failure::Control.into()));
        }
        let history: JoinedProtectedHistory = match operation.map_err(RunFailed::Operation)? {
            InvocationCompletion::Preview(summary) => {
                return Ok(ReceivedInvocation::Probe(ReceivedCompletion {
                    inner: ReceivedState::Preview(summary),
                }));
            }
            InvocationCompletion::ManagedPreview(history) => {
                return Ok(ReceivedInvocation::Probe(ReceivedCompletion {
                    inner: ReceivedState::ManagedPreview(history),
                }));
            }
            InvocationCompletion::Protected(history) => JoinedProtectedHistory::Diagnostic(history),
            InvocationCompletion::ManagedProtected(history) => {
                JoinedProtectedHistory::Managed(history)
            }
            InvocationCompletion::ManagedSuccessor(_) => {
                return Err(RunFailed::Operation(Failure::Control.into()));
            }
            #[cfg(test)]
            InvocationCompletion::ManagedProtectedControl(history) => {
                return Ok(ReceivedInvocation::ProtectedControl(
                    ReceivedProtectedControl { history },
                ));
            }
        };
        let original: &ProtectedHistory = history.observation();
        let configuration: Option<&SourceConfigurationSnapshot> = match &history {
            JoinedProtectedHistory::Diagnostic(_) => None,
            JoinedProtectedHistory::Managed(managed) => Some(managed.configuration()),
        };
        let measurement: Result<ExperimentalMeasurement, MeasurementRefusal> =
            crate::measurement::measure_for_source(
                profile,
                configuration,
                original.send_before,
                original.send_after,
                original.read_return,
                original.receipt.accepted_at(),
                original.receipt.fields().header(),
            );
        Ok(ReceivedInvocation::Probe(ReceivedCompletion {
            inner: ReceivedState::Protected(ReceivedProtectedNumerics {
                history,
                profile,
                measurement,
            }),
        }))
    }

    impl ProtectedHistory {
        /// Compare the original one-publication KE Control identities.
        pub(in super::super) fn same_ke_association(&self, other: &Self) -> bool {
            Arc::ptr_eq(
                &self.ke_association_identity,
                &other.ke_association_identity,
            )
        }

        /// Return the original KE provenance retained at the protected commit.
        pub(in super::super) const fn certificate_time_provenance(
            &self,
        ) -> CertificateTimeProvenance {
            self.certificate_time_provenance
        }

        /// Borrow only the unchanged CLI count projection of this owning history.
        pub(in super::super) const fn summary(&self) -> &Summary {
            &self.summary
        }

        /// Return the original guarded processing observation.
        pub(in super::super) const fn committed_at(&self) -> Instant {
            self.committed_at
        }

        /// Return the original trailing wall-read bracket's monotonic coordinate.
        pub(in super::super) fn received_at(&self) -> Instant {
            self.read_return.monotonic_after()
        }

        /// Derive only the original monotonic receive-trailing to commit interval.
        /// A refused derivation leaves the committed historical result intact.
        pub(in super::super) fn receive_to_commit_delay(&self) -> Option<Duration> {
            let trailing: Instant = self.received_at();
            if self.committed_at < trailing {
                return None;
            }
            self.committed_at.checked_duration_since(trailing)
        }

        /// Derive the inclusive elapsed bounds from this operation's original wall reads.
        /// Unavailable bounds do not change the receipt or its original timing.
        pub(in super::super) fn application_wall_elapsed_bounds(&self) -> Option<[Duration; 2]> {
            crate::measurement::application_elapsed_durations(
                self.send_before,
                self.send_after,
                self.read_return,
                self.committed_at,
            )
            .ok()
        }

        /// Inspect original authenticated bits/time for actual-owner component controls.
        pub(in super::super) const fn receipt(&self) -> Receipt {
            self.receipt
        }

        /// Inspect only the original checked post-admission pool population.
        #[cfg(test)]
        pub(in super::super) const fn population(&self) -> (usize, usize) {
            self.population
        }

        /// Copy the original pre-send, send-return and selected receive-return brackets.
        pub(in super::super) const fn captures(&self) -> [SoftwareClock; 3] {
            [self.send_before, self.send_after, self.read_return]
        }

        /// Copy the original native read outcomes without a new clock read.
        #[cfg(windows)]
        pub(in super::super) const fn native_captures(
            &self,
        ) -> [Option<ProtectedNativeClockObservation>; 3] {
            self.native_clock
        }

        /// Inspect actual historical endpoints without reissuing socket custody.
        pub(in super::super) const fn endpoints(&self) -> (SocketAddr, SocketAddr) {
            (self.local, self.remote)
        }

        /// Borrow original socket identity without retaining a live resource.
        #[cfg(test)]
        pub(in super::super) const fn identity(&self) -> &Arc<()> {
            &self.identity
        }

        /// Borrow the original pre-acquisition chart for immutable receipt projection.
        pub(in super::super) const fn run_context(&self) -> &Arc<RunClockContext> {
            &self.run_context
        }
    }

    impl<'control> ProtectedCompletion<'control> {
        /// Inspect historical component facts without duplicating commit custody.
        #[cfg(test)]
        pub(super) const fn history(&self) -> &OperationHistory<'control> {
            &self.operation.history
        }

        /// Borrow the exact backend identity retained by the submission.
        #[cfg(test)]
        pub(super) const fn identity(&self) -> &Arc<()> {
            self.operation.submission.identity()
        }

        /// Observe the existing budget without letting the fixture replace it.
        #[cfg(test)]
        pub(super) const fn remaining_attempts(&self) -> usize {
            self.attempts.remaining()
        }

        /// Commit the first retired request and retain its original historical facts.
        ///
        /// The ledger leaves the original Control published and owns the remaining
        /// driver allowance. The returned history grants no continuation authority.
        /// # Errors
        /// Preserve the original retired-reply and currentness refusals.
        pub(in super::super) fn commit_first_for_successor(
            self,
            association: &mut AssociationOwner<'_>,
        ) -> Result<CommittedFirstProtected<'control>, Failure> {
            let RetiredOperation {
                history,
                reply,
                attempts,
            }: RetiredOperation<'control> = self.into_retired();
            let (identity, committed_at): (Arc<()>, Instant) = association.commit_retired_reply(
                reply,
                RetiredCommit::FirstRequest(attempts),
                Instant::now,
            )?;
            // The ledger already checked this immutable lease in its original
            // refusal order. A component lease can lack an installed snapshot.
            let configuration: Option<SourceConfigurationSnapshot> = association
                .protected_endpoint()
                .ok()
                .flatten()
                .and_then(ProtectedEndpoint::configuration)
                .cloned();
            Ok(CommittedFirstProtected(CommittedProtected {
                operation: history,
                identity,
                committed_at,
                configuration,
                certificate_time_provenance: association.certificate_time_provenance(),
            }))
        }

        /// Exercise continuation in fixtures that only need the ledger transition.
        /// # Errors
        /// Preserve the shared final receipt and currentness refusals.
        #[cfg(test)]
        pub(in super::super) fn commit_for_successor(
            self,
            association: &mut AssociationOwner<'_>,
        ) -> Result<(), Failure> {
            self.commit_first_for_successor(association).map(drop)
        }

        /// Consume this retired operation at its actual current receiving owner.
        ///
        /// The original Control serializes final validation and cookie transfer.
        /// A stop or expiry before the decision leaves the live pool intact.
        /// # Errors
        /// Refuse foreign/stale owner, later charged work, stop or expiry. Refusal
        /// creates no fact and restores no request, cookie, deadline or work credit.
        pub(in super::super) fn commit(
            self,
            association: &mut AssociationOwner<'_>,
        ) -> Result<CommittedProtected<'control>, Failure> {
            self.commit_observing(association, Instant::now)
        }

        /// Sample the final processing clock through the actual guarded commit.
        ///
        /// The observer only supplies one clock reading and performs no other work.
        /// # Errors
        /// Preserve currentness and all refusals of the original retired commit.
        pub(super) fn commit_observing(
            self,
            association: &mut AssociationOwner<'_>,
            observe_now: impl FnOnce() -> Instant,
        ) -> Result<CommittedProtected<'control>, Failure> {
            let RetiredOperation {
                history,
                reply,
                attempts: _,
            }: RetiredOperation<'control> = self.into_retired();
            let configuration: Option<SourceConfigurationSnapshot> = association
                .protected_endpoint()?
                .map(|lease: &ProtectedEndpoint| lease.configuration().ok_or(Failure::Control))
                .transpose()?
                .cloned();
            let (identity, committed_at): (Arc<()>, Instant) =
                association.commit_retired_reply(reply, RetiredCommit::Terminal, observe_now)?;
            Ok(CommittedProtected {
                operation: history,
                identity,
                committed_at,
                configuration,
                certificate_time_provenance: association.certificate_time_provenance(),
            })
        }
        /// Only completed socket/runtime retirement permits making the ledger transfer.
        fn into_retired(self) -> RetiredOperation<'control> {
            let OperationCompletion {
                history,
                replacement,
                submission,
            }: OperationCompletion<'control> = self.operation;
            let reply: RetiredReply<'control> = RetiredReply {
                control: history.control,
                receipt: history.receipt,
                replacement,
                submission,
            };
            RetiredOperation {
                history,
                reply,
                attempts: self.attempts,
            }
        }
    }

    impl CommittedFirstProtected<'_> {
        /// Bind original first history to the actual receiver and worker Control.
        /// # Errors
        /// Refuse a foreign receiver or absent installed configuration.
        pub(in super::super) fn into_first_history(
            self,
            receiving: &BoundProtectedReceiver<'_, '_, '_, '_>,
        ) -> Result<BoundFirstHistory, Failure> {
            let InvocationCompletion::ManagedProtected(history) = self.0.into_history(receiving)?
            else {
                return Err(Failure::Control);
            };
            Ok(BoundFirstHistory { history })
        }

        /// Borrow the committed fact for the actual-owner continuation control.
        #[cfg(test)]
        pub(super) const fn observation(
            &self,
        ) -> (
            &OperationHistory<'_>,
            &Arc<()>,
            Instant,
            Option<&SourceConfigurationSnapshot>,
        ) {
            (
                &self.0.operation,
                &self.0.identity,
                self.0.committed_at,
                self.0.configuration.as_ref(),
            )
        }
    }

    impl CommittedProtected<'_> {
        /// Consume the historical fact once into its checked post-admission counts.
        /// This test-only projection attaches no invocation chart. No Published
        /// check follows commit, and later stop cannot rewrite history.
        #[cfg(test)]
        pub(in super::super) fn into_summary(self) -> Summary {
            let OperationHistory {
                control,
                receipt,
                send_before,
                send_after,
                read_return,
                #[cfg(windows)]
                native_clock,
                local,
                remote,
                packet_bytes,
                cookies,
                cookie_bytes,
            } = self.operation;
            let summary = Summary {
                cookies,
                cookie_bytes,
                packet_bytes,
                transmitted: true,
            };
            // Retain the complete historical header and actual capture/identity
            // through projection, with no numerical or fresh-sync admission.
            drop((
                control,
                receipt,
                send_before,
                send_after,
                read_return,
                #[cfg(windows)]
                native_clock,
                local,
                remote,
                self.identity,
            ));
            summary
        }

        /// Detach only immutable history from this genuine guarded commit.
        /// The consuming operation binds the actual receiving Control, obtains only
        /// that invocation's chart and the bound checked pre-operation population.
        /// No live owner, secret payload, numerical measurement or renewed timestamp
        /// is returned. Later cancellation never reopens the completed decision.
        /// # Errors
        /// Refuse a foreign receiving owner without creating or rebinding history.
        pub(in super::super) fn into_history(
            self,
            receiving: &BoundProtectedReceiver<'_, '_, '_, '_>,
        ) -> Result<InvocationCompletion, Failure> {
            let (run_context, population) = receiving.historical_context(self.operation.control)?;
            let OperationHistory {
                control,
                receipt,
                send_before,
                send_after,
                read_return,
                #[cfg(windows)]
                native_clock,
                local,
                remote,
                packet_bytes,
                cookies,
                cookie_bytes,
            } = self.operation;
            let (initial_cookies, initial_cookie_bytes) = population.counts();
            let history: ProtectedHistory = ProtectedHistory {
                summary: Summary {
                    cookies: initial_cookies,
                    cookie_bytes: initial_cookie_bytes,
                    packet_bytes,
                    transmitted: true,
                },
                receipt,
                ke_association_identity: Arc::clone(&control.first_receipt_identity),
                certificate_time_provenance: self.certificate_time_provenance,
                committed_at: self.committed_at,
                population: (cookies, cookie_bytes),
                send_before,
                send_after,
                read_return,
                #[cfg(windows)]
                native_clock,
                local,
                remote,
                identity: self.identity,
                run_context,
            };
            Ok(match self.configuration {
                Some(configuration) => {
                    InvocationCompletion::ManagedProtected(Arc::new(ManagedProtectedHistory {
                        history,
                        configuration,
                    }))
                }
                None => InvocationCompletion::Protected(Box::new(history)),
            })
        }
    }

    /// Register the shared wake before checking actual scope and pending correlation.
    /// A timer wake rechecks Control so a Stop latched before it retains precedence.
    /// A harmless wake re-waits within the same nonrenewable attempt allowance.
    /// Socket readiness remains necessary. Policy contention waits for release.
    /// # Errors
    /// Preserve cancellation, expiry, pending-policy and genuine readiness failure.
    async fn wait(
        backend: &mut Backend,
        control: &Control,
        interest: Interest,
        attempts: &mut Attempts,
        mut pending_policy: Option<&mut LivePolicy>,
    ) -> Result<(), Failure> {
        loop {
            attempts.spend(control)?;
            let cancelled = control.cancelled.notified();
            tokio::pin!(cancelled);
            cancelled.as_mut().enable();
            drop(control.published_guard()?);
            let socket_allowed: bool = if let Some(policy) = pending_policy.as_deref_mut() {
                let refreshed: Result<bool, Failure> = policy
                    .refresh_pending(&backend.identity)
                    .map_err(Failure::SourcePolicy);
                drop(control.published_guard()?);
                refreshed?
            } else {
                true
            };
            let ready = tokio::select! {
                biased;
                () = &mut cancelled => false,
                () = tokio::time::sleep_until(tokio::time::Instant::from_std(control.expiry)) => false,
                () = async {
                    match pending_policy.as_deref_mut() {
                        Some(policy) => policy.wake().await,
                        None => std::future::pending::<()>().await,
                    }
                } => false,
                ready = backend.ready(interest, control), if socket_allowed => {
                    ready?;
                    true
                },
            };
            drop(control.published_guard()?);
            if ready {
                return Ok(());
            }
        }
    }

    /// Exercise the ordinary fresh driver allowance in direct component tests.
    /// # Errors
    /// Preserve actual request, adapter and receive refusals.
    #[cfg(test)]
    pub(super) async fn operation<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
        backend: &mut Backend,
    ) -> Result<OperationCompletion<'control>, Failure> {
        association.check_fresh_exchange()?;
        let mut attempts: Attempts = Attempts(ATTEMPTS);
        operation_with_attempts(
            association,
            control,
            fill,
            backend,
            &mut attempts,
            &mut ReceiveState::Ordinary,
        )
        .await
        .and_then(SelectedOperation::into_ordinary)
    }

    /// One concrete consumption on the exact pending backend.
    #[must_use = "account for this consumed datagram exactly once"]
    pub(in super::super) struct ConsumedDatagram<'backend> {
        /// Identity of the adapter that actually returned this read.
        backend: &'backend Arc<()>,
        /// Exact selected extent. An unknown extent has no exposable prefix.
        packet: Option<&'backend [u8]>,
    }

    impl<'backend> ConsumedDatagram<'backend> {
        /// Give the ledger exclusive custody of this observation.
        pub(in super::super) fn into_parts(self) -> (&'backend Arc<()>, Option<&'backend [u8]>) {
            (self.backend, self.packet)
        }
    }

    /// One actual datagram and its original adapter/capture observations.
    struct ObservedReply<'a> {
        /// Exclusive accounting for the original concrete read.
        consumed: ConsumedDatagram<'a>,
        /// Peer returned by the actual selector.
        remote: SocketAddr,
        /// Exact concrete socket/back-end identity.
        backend: &'a Backend,
        /// Capture preceding the actual synchronous send.
        send_before: ProtectedClockCapture,
        /// Capture following the same synchronous send.
        send_after: ProtectedClockCapture,
        /// Capture before any interpretation of the selected packet.
        read_return: ProtectedClockCapture,
        /// Local packet extent, independent of either received length or peer fields.
        packet_bytes: usize,
    }

    /// The receive selection and the bounded observation owned by that request.
    enum ReceiveState {
        /// Existing ordinary one-shot and component paths.
        Ordinary,
        /// The bounded first leg additionally admits an S2C-authenticated restriction.
        ProtectedInitial,
        /// The admitted successor can retain one nonterminal NAK before completing.
        ProtectedSuccessor {
            /// The actual first NAK arrival. Later reads still spend credits.
            first_nak_at: Option<Instant>,
        },
    }

    /// One concrete async driver result shared by both receive modes.
    #[expect(
        clippy::large_enum_variant,
        reason = "keep the bounded selected reply and transfer inline before resource retirement"
    )]
    enum SelectedOperation<'control> {
        /// Ordinary timing and replacement obtained through the original ledger.
        Ordinary(OperationCompletion<'control>),
        /// The charged control evidence cannot produce ordinary reply history.
        Control(StagedProtectedControl<'control>),
    }

    impl<'control> SelectedOperation<'control> {
        /// Preserve the existing ordinary result type at its callers.
        /// # Errors
        /// Refuse a closed control disposition if an ordinary caller receives one.
        fn into_ordinary(self) -> Result<OperationCompletion<'control>, Failure> {
            match self {
                Self::Ordinary(operation) => Ok(operation),
                Self::Control(_) => Err(Failure::Control),
            }
        }

        /// Detach the candidate only after the caller has retired both I/O owners.
        fn into_retired(self, attempts: Attempts) -> RetiredProtectedResponse<'control> {
            match self {
                Self::Ordinary(operation) => {
                    RetiredProtectedResponse::Ordinary(ProtectedCompletion {
                        operation,
                        attempts,
                    })
                }
                Self::Control(staged) => {
                    RetiredProtectedResponse::Control(RetiredProtectedControl(staged))
                }
            }
        }
    }

    /// Admit one charged disposition through the selected path in the same ledger.
    /// A current NAK records only the observation and can leave the operation pending.
    /// # Errors
    /// Preserve original endpoint, identity, budget, time and authentication refusals.
    fn stage_observed_reply<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        reply: ObservedReply<'_>,
        state: &mut ReceiveState,
    ) -> Result<Option<SelectedOperation<'control>>, Failure> {
        let read_return: SoftwareClock = reply.read_return.software();
        let (receipt, replacement, submission): (Receipt, CookieReplacement, PendingSubmission) =
            match state {
                ReceiveState::Ordinary => {
                    association.stage_concrete_reply(reply.consumed, reply.remote, &read_return)?
                }
                ReceiveState::ProtectedInitial | ReceiveState::ProtectedSuccessor { .. } => {
                    let leg: ProtectedControlLeg =
                        if matches!(state, ReceiveState::ProtectedInitial) {
                            ProtectedControlLeg::Initial
                        } else {
                            ProtectedControlLeg::Successor
                        };
                    match association.stage_concrete_protected_response(
                        control,
                        reply.consumed,
                        reply.remote,
                        &read_return,
                        leg,
                    )? {
                        StagedProtectedSuccessor::Ordinary(receipt, replacement, submission) => {
                            (receipt, replacement, submission)
                        }
                        StagedProtectedSuccessor::Control(staged) => {
                            return Ok(Some(SelectedOperation::Control(staged)));
                        }
                        StagedProtectedSuccessor::NtsNak(observation) => {
                            let ReceiveState::ProtectedSuccessor { first_nak_at } = state else {
                                return Err(Failure::Control);
                            };
                            let (arrived_at, exhausted): (Instant, bool) = observation.into_parts();
                            if first_nak_at.is_none() {
                                *first_nak_at = Some(arrived_at);
                                if let Some(work) = control.datagram_work.as_deref() {
                                    let _recorded: &Instant =
                                        work.first_nak_at.get_or_init(|| arrived_at);
                                }
                            }
                            if exhausted {
                                return Err(Failure::Ledger(LedgerError::ReceiveWork));
                            }
                            return Ok(None);
                        }
                    }
                }
            };
        let (cookies, cookie_bytes): (usize, usize) = replacement.population();
        Ok(Some(SelectedOperation::Ordinary(OperationCompletion {
            history: OperationHistory {
                control,
                receipt,
                send_before: reply.send_before.software(),
                send_after: reply.send_after.software(),
                read_return,
                #[cfg(windows)]
                native_clock: [
                    reply.send_before.native(),
                    reply.send_after.native(),
                    reply.read_return.native(),
                ],
                local: reply.backend.local,
                remote: reply.backend.remote,
                packet_bytes: reply.packet_bytes,
                cookies,
                cookie_bytes,
            },
            replacement,
            submission,
        })))
    }

    /// Wait for the original managed floor before checking out a request cookie.
    /// Charge each park to the original driver allowance. Eligibility is only
    /// a readiness hint. The final sender rechecks the same records with the
    /// original Control before it invokes the adapter.
    /// # Errors
    /// Preserve the original Stop, expiry, policy or shared work refusal.
    async fn wait_for_send_eligibility(
        control: &Control,
        backend_identity: &Arc<()>,
        mut policy: LivePolicy,
        attempts: &mut Attempts,
    ) -> Result<(), Failure> {
        loop {
            let cancelled = control.cancelled.notified();
            tokio::pin!(cancelled);
            cancelled.as_mut().enable();
            drop(control.published_guard()?);
            let refreshed: Result<(), Failure> = policy
                .refresh(backend_identity)
                .map_err(Failure::SourcePolicy);
            drop(control.published_guard()?);
            refreshed?;
            if policy.wants_writable() {
                // Checkout must leave a credit for the required write readiness wait.
                return attempts.after_one().map(|_| ());
            }
            attempts.spend(control)?;
            // A timer is only a wake. The original guard can observe an already
            // latched Stop even before its notification reaches this task.
            tokio::select! {
                biased;
                () = &mut cancelled => {},
                () = tokio::time::sleep_until(tokio::time::Instant::from_std(control.expiry)) => {},
                () = policy.wake() => {},
            }
        }
    }

    /// Drive the same bounded packet owner with either concrete backend.
    /// # Errors
    /// Retain original adapter, currentness and packet refusals; never retry a send.
    async fn operation_with_attempts<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
        backend: &mut Backend,
        attempts: &mut Attempts,
        state: &mut ReceiveState,
    ) -> Result<SelectedOperation<'control>, Failure> {
        if association.check_context(control)? != backend.remote {
            return Err(Failure::Endpoint);
        }
        if let Some(lease) = association.protected_endpoint()? {
            let policy: LivePolicy = lease.observe_send_eligibility();
            wait_for_send_eligibility(control, &backend.identity, policy, attempts).await?;
        }
        let ticket: Ticket<'_> = association.reserve(RequestInput::Observed)?;
        association.prepare_observed(&ticket, fill)?;
        drive_prepared(association, control, backend, attempts, ticket, state).await
    }

    /// Send once and run the same concrete readiness/receive future in both modes.
    /// # Errors
    /// Retain original readiness, capture, completion and selected packet refusals.
    async fn drive_prepared<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        backend: &mut Backend,
        attempts: &mut Attempts,
        ticket: Ticket<'_>,
        state: &mut ReceiveState,
    ) -> Result<SelectedOperation<'control>, Failure> {
        wait(backend, control, Interest::Write, attempts, None).await?;
        backend.send_prepared(association, ticket, control)?;
        let (send_before, send_after, packet_bytes): (
            ProtectedClockCapture,
            ProtectedClockCapture,
            usize,
        ) = backend.send.ok_or(failed(DatagramError::Capture))?;
        let send_after_bracket: SoftwareClock = send_after.software();
        check_capture_order(
            send_before.software().monotonic_after(),
            send_after_bracket.monotonic(),
        )?;
        let mut buffer = Zeroizing::new([0; RECEIVE_BYTES]);
        let mut pending_policy: Option<LivePolicy> =
            association.observe_pending_policy(&backend.identity)?;
        wait(
            backend,
            control,
            Interest::Read,
            attempts,
            pending_policy.as_mut(),
        )
        .await?;
        loop {
            attempts.spend(control)?;
            drop(control.published_guard()?);
            let (read, observed): (Read, ProtectedClockCapture) = backend.receive(&mut buffer);
            match read {
                Read::WouldBlock => {
                    wait(
                        backend,
                        control,
                        Interest::Read,
                        attempts,
                        pending_policy.as_mut(),
                    )
                    .await?;
                }
                Read::Failed(kind) => return Err(failed(DatagramError::Receive(kind))),
                Read::ConsumedUnknown => {
                    association.charge_discarded_reply(ConsumedDatagram {
                        backend: &backend.identity,
                        packet: None,
                    })?;
                    return Err(failed(DatagramError::Capture));
                }
                Read::Datagram(length, remote) => {
                    let packet: &[u8] =
                        buffer.get(..length).ok_or(failed(DatagramError::Capture))?;
                    let consumed: ConsumedDatagram<'_> = ConsumedDatagram {
                        backend: &backend.identity,
                        packet: Some(packet),
                    };
                    if let Err(error) = check_capture_order(
                        send_after_bracket.monotonic_after(),
                        observed.software().monotonic(),
                    ) {
                        association.charge_discarded_reply(consumed)?;
                        return Err(error);
                    }
                    let reply: ObservedReply<'_> = ObservedReply {
                        consumed,
                        remote,
                        backend,
                        send_before,
                        send_after,
                        read_return: observed,
                        packet_bytes,
                    };
                    match stage_observed_reply(association, control, reply, state) {
                        Ok(Some(selected)) => return Ok(selected),
                        Ok(None) => {}
                        Err(error) => {
                            if !matches!(association.status().phase, RequestPhase::Pending) {
                                return Err(error);
                            }
                            if matches!(
                                error,
                                Failure::Cancelled
                                    | Failure::Deadline
                                    | Failure::Control
                                    | Failure::SourcePolicy(_)
                            ) {
                                return Err(error);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Own one active current-thread driver through socket retirement.
    /// # Errors
    /// Refuse nested runtime, socket setup and the genuine bounded operation.
    #[cfg(windows)]
    pub(in super::super) fn exchange<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
    ) -> Result<ProtectedCompletion<'control>, Failure> {
        exchange_with(association, control, fill, Backend::bind)
    }

    /// Own the actual driver and selected backend through their real retirement.
    ///
    /// The native caller supplies only its fixed bind function; Script controls
    /// create their closed backend through the same path inside this driver.
    /// Only this return boundary constructs a committable completion. Raw
    /// operation output remains historical and has no commit method.
    /// # Errors
    /// Preserve nested-context, association, driver/backend setup or operation
    /// refusal; retire owned resources before returning either success or failure.
    pub(super) fn exchange_with<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
        make_backend: impl FnOnce(SocketAddr) -> Result<Backend, Failure>,
    ) -> Result<ProtectedCompletion<'control>, Failure> {
        association.check_fresh_exchange()?;
        exchange_with_attempts(association, control, fill, make_backend, Attempts(ATTEMPTS))
    }

    /// Retire the bounded first native request and its selected protected response.
    /// # Errors
    /// Preserve the original admission, socket, driver and receive refusals.
    #[cfg(windows)]
    pub(in super::super) fn exchange_initial<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
    ) -> Result<RetiredProtectedResponse<'control>, Failure> {
        exchange_initial_with(association, control, fill, Backend::bind)
    }

    /// Start the single bounded managed first leg with its original driver credits.
    /// # Errors
    /// Refuse another mode or previously earned continuation before any I/O.
    pub(super) fn exchange_initial_with<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
        make_backend: impl FnOnce(SocketAddr) -> Result<Backend, Failure>,
    ) -> Result<RetiredProtectedResponse<'control>, Failure> {
        if control.datagram_work.is_none() {
            return Err(Failure::Control);
        }
        association.check_fresh_exchange()?;
        let mut attempts: Attempts = Attempts(ATTEMPTS);
        let selected: SelectedOperation<'control> = retire_operation(
            association,
            control,
            fill,
            make_backend,
            &mut attempts,
            &mut ReceiveState::ProtectedInitial,
        )?;
        Ok(selected.into_retired(attempts))
    }

    /// Exercise a second genuine retired request under the original association.
    /// # Errors
    /// Refuse missing or stale authentication before any new backend or entropy.
    #[cfg(test)]
    pub(super) fn exchange_successor_with<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
        make_backend: impl FnOnce(SocketAddr) -> Result<Backend, Failure>,
    ) -> Result<ProtectedCompletion<'control>, Failure> {
        let attempts: Attempts = association.take_authenticated_successor(control)?;
        exchange_with_attempts(association, control, fill, make_backend, attempts)
    }

    /// Retire both resources while retaining only the completed packet and its credits.
    /// # Errors
    /// Preserve nested driver, context, setup and request refusal.
    fn exchange_with_attempts<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
        make_backend: impl FnOnce(SocketAddr) -> Result<Backend, Failure>,
        mut attempts: Attempts,
    ) -> Result<ProtectedCompletion<'control>, Failure> {
        let selected: SelectedOperation<'control> = retire_operation(
            association,
            control,
            fill,
            make_backend,
            &mut attempts,
            &mut ReceiveState::Ordinary,
        )?;
        let operation: OperationCompletion<'control> = selected.into_ordinary()?;
        Ok(ProtectedCompletion {
            operation,
            attempts,
        })
    }

    /// Keep the fixed script constructor confined to the existing component controls.
    /// # Errors
    /// Preserve the common driver's admission, operation and retirement refusals.
    #[cfg(test)]
    pub(super) fn exchange_successor_mixed<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
        make_script: impl FnOnce(SocketAddr) -> Script,
    ) -> Result<RetiredProtectedResponse<'control>, Failure> {
        exchange_successor_mixed_with(association, control, fill, |remote: SocketAddr| {
            Ok(super::tests::backend(remote, make_script(remote)))
        })
    }

    /// Drive the selected bounded fixture backend for either private successor disposition.
    /// # Errors
    /// Preserve original continuation, adapter, charged receive and policy refusals.
    #[cfg(test)]
    pub(super) fn exchange_successor_mixed_with<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
        make_backend: impl FnOnce(SocketAddr) -> Result<Backend, Failure>,
    ) -> Result<RetiredProtectedResponse<'control>, Failure> {
        exchange_successor_observing(association, control, fill, make_backend)
            .map(|observed: ObservedRetiredSuccessor<'control>| observed.into_parts().0)
            .map_err(|failure: SuccessorFailure| failure.cause())
    }

    /// Spend the earned continuation and retire one actual native successor socket.
    /// # Errors
    /// Preserve successor admission and socket/driver failures beside any observed NAK.
    #[cfg(windows)]
    pub(in super::super) fn exchange_successor<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
    ) -> Result<ObservedRetiredSuccessor<'control>, SuccessorFailure> {
        exchange_successor_observing(association, control, fill, Backend::bind)
    }

    /// Retire the actual successor, retaining a NAK beside its candidate or failure.
    /// # Errors
    /// Preserve continuation, setup, charged receive and policy refusals. A NAK
    /// never starts replacement under this one-KE selection.
    pub(super) fn exchange_successor_observing<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
        make_backend: impl FnOnce(SocketAddr) -> Result<Backend, Failure>,
    ) -> Result<ObservedRetiredSuccessor<'control>, SuccessorFailure> {
        let mut attempts: Attempts =
            association
                .take_authenticated_successor(control)
                .map_err(|cause: Failure| SuccessorFailure {
                    cause,
                    first_nak_at: None,
                })?;
        let mut state: ReceiveState = ReceiveState::ProtectedSuccessor { first_nak_at: None };
        let outcome: Result<SelectedOperation<'control>, Failure> = retire_operation(
            association,
            control,
            fill,
            make_backend,
            &mut attempts,
            &mut state,
        );
        let first_nak_at: Option<Instant> = match state {
            ReceiveState::ProtectedSuccessor { first_nak_at } => first_nak_at,
            ReceiveState::Ordinary | ReceiveState::ProtectedInitial => None,
        };
        let selected: SelectedOperation<'control> =
            outcome.map_err(|cause: Failure| SuccessorFailure {
                cause,
                first_nak_at,
            })?;
        Ok(ObservedRetiredSuccessor {
            candidate: selected.into_retired(attempts),
            first_nak_at,
        })
    }

    /// Own the actual adapter/runtime until this selected outcome can detach.
    /// # Errors
    /// Refuse nested runtime, invalid association, setup, send or receive work.
    fn retire_operation<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
        make_backend: impl FnOnce(SocketAddr) -> Result<Backend, Failure>,
        attempts: &mut Attempts,
        state: &mut ReceiveState,
    ) -> Result<SelectedOperation<'control>, Failure> {
        if tokio::runtime::Handle::try_current().is_ok() {
            return Err(failed(DatagramError::EnteredRuntime));
        }
        let remote = association.check_context(control)?;
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .max_blocking_threads(2)
            .build()
            .map_err(|error| failed(DatagramError::Runtime(error.kind())))?;
        let result = runtime.block_on(async {
            let mut backend = make_backend(remote)?;
            let completion: Result<SelectedOperation<'control>, Failure> =
                operation_with_attempts(association, control, fill, &mut backend, attempts, state)
                    .await;
            drop(backend);
            completion
        });
        drop(runtime);
        result
    }

    /// Cover each bound-address refusal independently for both address families.
    /// # Panics
    /// Fail if any single invalid coordinate is accepted or the valid pair is refused.
    #[cfg(test)]
    #[test]
    fn bound_endpoint_check_refuses_each_invalid_coordinate() {
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV6};

        for (loopback, foreign, other_family) in [
            (
                IpAddr::V4(Ipv4Addr::LOCALHOST),
                IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1)),
                IpAddr::V6(Ipv6Addr::LOCALHOST),
            ),
            (
                IpAddr::V6(Ipv6Addr::LOCALHOST),
                IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1)),
                IpAddr::V4(Ipv4Addr::LOCALHOST),
            ),
        ] {
            let requested: SocketAddr = SocketAddr::new(loopback, 0);
            let local: SocketAddr = SocketAddr::new(loopback, 49000);
            let remote: SocketAddr = SocketAddr::new(loopback, 4460);
            assert_eq!(check_bound_endpoints(requested, local, remote), Ok(local));
            assert_eq!(
                check_bound_endpoints(requested, SocketAddr::new(foreign, local.port()), remote),
                Err(Failure::Endpoint),
            );
            assert_eq!(
                check_bound_endpoints(
                    requested,
                    SocketAddr::new(other_family, local.port()),
                    remote
                ),
                Err(Failure::Endpoint),
            );
            assert_eq!(
                check_bound_endpoints(requested, requested, remote),
                Err(Failure::Endpoint),
            );
            assert_eq!(
                check_bound_endpoints(requested, local, SocketAddr::new(foreign, remote.port())),
                Err(Failure::Endpoint),
            );
        }

        let ipv4_bind: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 0));
        let ipv4_peer: SocketAddr = SocketAddr::from(([127, 0, 0, 2], 4460));
        let ipv4_bound: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 49000));
        let ipv4_alias: SocketAddr = SocketAddr::from(([127, 0, 0, 2], 49000));
        assert_eq!(
            check_bound_endpoints(ipv4_bind, ipv4_bound, ipv4_peer),
            Ok(ipv4_bound)
        );
        assert_eq!(
            check_bound_endpoints(ipv4_bind, ipv4_alias, ipv4_peer),
            Err(Failure::Endpoint),
        );

        let ipv6_bind: SocketAddr = SocketAddr::from((Ipv6Addr::LOCALHOST, 0));
        let ipv6_peer: SocketAddr = SocketAddr::from((Ipv6Addr::LOCALHOST, 4460));
        for (flow, scope) in [(0, 1), (1, 0)] {
            let ipv6_changed: SocketAddr =
                SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 49000, flow, scope));
            assert_eq!(
                check_bound_endpoints(ipv6_bind, ipv6_changed, ipv6_peer),
                Err(Failure::Endpoint),
            );
        }
    }

    /// Exercise the actual IPv4 loopback binding, registration and backend drop.
    /// # Errors
    /// Preserve runtime setup, native binding, endpoint observation and readiness failures.
    /// # Panics
    /// Fail if the live socket and backend disagree, the raw remote is admitted,
    /// or the backend's identity is retained after its owner is dropped.
    #[cfg(all(test, windows))]
    #[test]
    fn native_ipv4_loopback_bind_refuses_foreign_remote_and_retires_owner() -> Result<(), Failure> {
        verify_native_loopback_bind(
            SocketAddr::from(([127, 0, 0, 1], 4460)),
            SocketAddr::from(([0, 0, 0, 0], 4460)),
        )
    }

    /// Exercise the actual IPv6 loopback binding, registration and backend drop.
    /// # Errors
    /// Preserve runtime setup, native binding, endpoint observation and readiness failures.
    #[cfg(all(test, windows))]
    #[test]
    fn native_ipv6_loopback_bind_refuses_foreign_remote_and_retires_owner() -> Result<(), Failure> {
        use std::net::Ipv6Addr;

        verify_native_loopback_bind(
            SocketAddr::from((Ipv6Addr::LOCALHOST, 4460)),
            SocketAddr::from((Ipv6Addr::UNSPECIFIED, 4460)),
        )
    }

    /// Check the registered socket and retirement for one named loopback family.
    /// # Errors
    /// Preserve runtime, bind, local endpoint and readiness failures.
    /// # Panics
    /// Fail if the socket was substituted, used before readiness, left live, or
    /// the same family's foreign peer was admitted.
    #[cfg(all(test, windows))]
    fn verify_native_loopback_bind(
        loopback: SocketAddr,
        foreign: SocketAddr,
    ) -> Result<(), Failure> {
        use std::sync::Weak;
        use std::time::Duration;

        let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .map_err(|error| failed(DatagramError::Runtime(error.kind())))?;
        runtime.block_on(async {
            let backend: Backend = Backend::bind(loopback)?;
            let Io::Native(socket) = &backend.io else {
                return Err(Failure::Control);
            };
            let registered: SocketAddr = socket
                .local_addr()
                .map_err(|error| failed(DatagramError::Local(error.kind())))?;
            assert_eq!(
                registered,
                SocketAddr::new(loopback.ip(), registered.port())
            );
            assert_ne!(registered.port(), 0);
            assert_eq!((backend.local, backend.remote), (registered, loopback));
            tokio::time::timeout(Duration::from_secs(2), socket.writable())
                .await
                .map_err(|_| Failure::Deadline)?
                .map_err(|error| failed(DatagramError::Readiness(error.kind())))?;
            assert!(backend.send.is_none());
            assert!(matches!(Backend::bind(foreign), Err(Failure::Endpoint)));
            let identity: Weak<()> = Arc::downgrade(&backend.identity);
            assert_eq!(identity.strong_count(), 1);
            drop(backend);
            assert_eq!(identity.strong_count(), 0);
            Ok(())
        })
    }

    #[cfg(test)]
    use super::tests::Script;
}

#[cfg(windows)]
pub(super) use reached::{exchange, exchange_initial, exchange_successor};

#[cfg(any(windows, test))]
pub(super) use reached::{
    Attempts, BoundFirstHistory, CommittedFirstProtected, CommittedProtected, CompletionProjection,
    ConsumedDatagram, GuardedSender, ManagedProtectedHistory, ManagedProtectedProjection,
    ObservedRetiredSuccessor, ProtectedCompletion, ProtectedDatagramWork, ProtectedHistory,
    ReceivedCompletion, ReceivedInvocation, RetiredProtectedControl, RetiredProtectedResponse,
    RetiredReply, RetiredReplyParts, SuccessorFailure, receive_joined,
};

#[cfg(test)]
#[path = "datagram/tests.rs"]
pub(super) mod tests;
