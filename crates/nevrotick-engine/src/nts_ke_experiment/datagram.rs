// SPDX-License-Identifier: GPL-2.0-only
//! One private protected UDP operation after genuine association publication.
//!
//! The Windows backend owns its socket through synchronous syscall returns. The
//! deterministic backend exercises the same owner without opening a socket.
//! Neither historical completion nor software-clock captures confer eligibility.

use std::io;

/// Closed adapter failures; never retain packet, endpoint or library-error text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DatagramError {
    /// A nested runtime cannot own this synchronous invocation.
    #[cfg(any(windows, test))]
    EnteredRuntime,
    /// Current-thread runtime construction failed.
    Runtime(io::ErrorKind),
    /// One bounded loopback socket could not be bound.
    #[cfg(windows)]
    Bind(io::ErrorKind),
    /// The socket could not become nonblocking.
    #[cfg(windows)]
    Nonblocking(io::ErrorKind),
    /// The actual local endpoint could not be captured/admitted.
    #[cfg(windows)]
    Local(io::ErrorKind),
    /// Tokio could not register the owned nonblocking socket.
    #[cfg(windows)]
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
/// Concrete Windows operation and the identical deterministic receiving path.
mod reached {
    use std::io;
    use std::net::SocketAddr;
    use std::sync::Arc;
    use std::time::Instant;

    #[cfg(windows)]
    use tokio::net::UdpSocket;
    use zeroize::Zeroizing;

    use super::DatagramError;
    use crate::nts_ke_experiment::association::AssociationOwner;
    #[cfg(test)]
    use crate::nts_ke_experiment::association::ledger::{
        AppliedProtectedDeny, StagedProtectedDeny, StagedProtectedSuccessor,
    };
    use crate::nts_ke_experiment::association::ledger::{
        CookieReplacement, PendingSubmission, Receipt, RequestInput, RequestPhase, RetiredCommit,
        SendOutcome, Ticket,
    };
    use crate::nts_ke_experiment::job::ReceivingOutcome;
    use crate::nts_ke_experiment::recovery::BoundProtectedReceiver;
    use crate::nts_ke_experiment::{
        Control, DatagramAuthorityGuard, Failure, InvocationCompletion, IoPhase, RunFailed, Summary,
    };
    use crate::source_policy::{ProtectedEndpoint, ProtectedSendStamp};
    use crate::{
        ExperimentalMeasurement, ExperimentalProfile, MeasurementRefusal, RunClockContext,
        SoftwareClock, SourceConfigurationSnapshot,
    };

    /// One counter covers readiness waits and nonblocking receive attempts.
    const ATTEMPTS: usize = 64;
    /// An extra octet detects every successful receive above the admitted limit.
    const RECEIVE_BYTES: usize = 4097;

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
        pub(super) fn spend(&mut self) -> Result<(), Failure> {
            self.0 = self
                .0
                .checked_sub(1)
                .ok_or(failed(DatagramError::Attempts))?;
            Ok(())
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
        #[cfg(windows)]
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
        pub(super) send: Option<(SoftwareClock, SoftwareClock, usize)>,
    }

    /// Check the actual bound endpoint and selected peer before registration.
    /// # Errors
    /// Refuse a non-loopback local or remote address or a zero bound port.
    #[cfg(any(windows, test))]
    fn check_bound_endpoints(local: SocketAddr, remote: SocketAddr) -> Result<(), Failure> {
        if !local.ip().is_loopback() || local.port() == 0 || !remote.ip().is_loopback() {
            return Err(Failure::Endpoint);
        }
        Ok(())
    }

    impl Backend {
        /// Bind and register exactly one actual same-family loopback socket.
        /// # Errors
        /// Preserve bounded bind, nonblocking, local-endpoint and registration causes.
        #[cfg(windows)]
        fn bind(remote: SocketAddr) -> Result<Self, Failure> {
            use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, UdpSocket as StdSocket};

            let ip = match remote {
                SocketAddr::V4(_) => IpAddr::V4(Ipv4Addr::LOCALHOST),
                SocketAddr::V6(_) => IpAddr::V6(Ipv6Addr::LOCALHOST),
            };
            let socket = StdSocket::bind(SocketAddr::new(ip, 0))
                .map_err(|error| failed(DatagramError::Bind(error.kind())))?;
            socket
                .set_nonblocking(true)
                .map_err(|error| failed(DatagramError::Nonblocking(error.kind())))?;
            let local: SocketAddr = socket
                .local_addr()
                .map_err(|error| failed(DatagramError::Local(error.kind())))?;
            check_bound_endpoints(local, remote)?;
            let socket = UdpSocket::from_std(socket)
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
                #[cfg(windows)]
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
                #[cfg(windows)]
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

        /// Observe one synchronous return before interpreting any packet bytes.
        fn receive(&mut self, buffer: &mut [u8; RECEIVE_BYTES]) -> (Read, SoftwareClock) {
            match &mut self.io {
                #[cfg(windows)]
                Io::Native(socket) => {
                    let result = socket.try_recv_from(buffer);
                    let observed = SoftwareClock::capture();
                    (classify_read(result), observed)
                }
                #[cfg(test)]
                Io::Script(script) => {
                    let result = script.receive(buffer);
                    (result, SoftwareClock::capture())
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
            let before: SoftwareClock = SoftwareClock::capture();
            #[cfg(test)]
            let before: SoftwareClock = match &backend.io {
                #[cfg(windows)]
                Io::Native(_) => before,
                Io::Script(script) => script.observe_send_before(before),
            };
            check_capture_order(before.monotonic(), before.monotonic_after())?;
            guard.control.published(before.monotonic_after())?;
            let stamp: Option<ProtectedSendStamp> =
                guard.reserve(remote, &backend.identity, before.monotonic_after())?;
            let (outcome, cancel_after): (SendOutcome, bool) = match &mut backend.io {
                #[cfg(windows)]
                Io::Native(socket) => (
                    match socket.try_send_to(packet, remote) {
                        Ok(length) => SendOutcome::Completed(length),
                        Err(error) => SendOutcome::Failed(error.kind()),
                    },
                    false,
                ),
                #[cfg(test)]
                Io::Script(script) => script.send(packet, guard.control.control),
            };
            let after: SoftwareClock = SoftwareClock::capture();
            let control: &Control = guard.control.control;
            drop(guard);
            backend.send = Some((before, after, packet.len()));
            // Preserve completed-send custody before any fixture refusal or Stop.
            #[cfg(test)]
            match &backend.io {
                #[cfg(windows)]
                Io::Native(_) => {}
                Io::Script(script) => {
                    let after: SoftwareClock = script.observe_send_after(after)?;
                    backend.send = Some((before, after, packet.len()));
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
        /// Driver credits left by the original runtime; only the fixture can continue.
        #[cfg(test)]
        attempts: Attempts,
    }

    /// The actual scripted DENY transfer after both receive resources retired.
    /// Pending remains with the ledger until its guarded final commit.
    #[cfg(test)]
    #[must_use = "observe the retired DENY through its original owner"]
    pub(in super::super) struct RetiredProtectedDeny<'control>(StagedProtectedDeny<'control>);

    /// Either private successor result, after the shared driver and backend retire.
    #[cfg(test)]
    #[expect(
        clippy::large_enum_variant,
        reason = "keep the bounded returned history and cookie transfer inline after retirement"
    )]
    pub(in super::super) enum RetiredProtectedSuccessor<'control> {
        /// A complete ordinary reply with the existing final commit boundary.
        Ordinary(ProtectedCompletion<'control>),
        /// An exact control with its Pending request still held.
        Deny(RetiredProtectedDeny<'control>),
    }

    #[cfg(test)]
    impl<'control> RetiredProtectedDeny<'control> {
        /// Transfer the exact already-retired request to its receiving ledger.
        pub(in super::super) fn into_staged(self) -> StagedProtectedDeny<'control> {
            self.0
        }

        /// Publish the exact protected denial through its original receiving owner.
        /// # Errors
        /// Preserve any currentness, endpoint or final guarded policy refusal.
        pub(super) fn commit(
            self,
            association: &mut AssociationOwner<'_>,
        ) -> Result<AppliedProtectedDeny, Failure> {
            association.commit_retired_protected_deny(self)
        }

        /// Recheck this exact retired ticket without publishing any control effects.
        /// # Errors
        /// Preserve wrong-owner, stale-currentness, Stop and expiry refusals.
        pub(super) fn check_current(
            &self,
            association: &AssociationOwner<'_>,
        ) -> Result<(), Failure> {
            association.check_protected_deny_current(&self.0)
        }
    }

    /// Keep detached facts and the exact ledger transfer within the retired owner.
    struct RetiredOperation<'control> {
        /// Historical records have no authority to install the replacement.
        history: OperationHistory<'control>,
        /// The actual consumed request and its guarded transfer.
        reply: RetiredReply<'control>,
        /// The original driver's unused allowance.
        #[cfg(test)]
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
        /// Historical managed configuration, checked at both original effects.
        configuration: Option<SourceConfigurationSnapshot>,
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
        /// Checked historical pool entries/bytes, including duplicate entries.
        /// Distinct from initial KE counts; grants no availability or reuse authority.
        population: (usize, usize),
        /// Actual bracket immediately before the successful local send.
        send_before: SoftwareClock,
        /// Actual bracket immediately after that same local send.
        send_after: SoftwareClock,
        /// Original selected receive-return bracket, never refreshed at detachment.
        read_return: SoftwareClock,
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
        #[cfg(test)]
        pub(in super::super) const fn summary(&self) -> &Summary {
            self.history.summary()
        }
    }

    /// One normally joined result; numerical refusal is retained data, never a retry.
    /// Its private representation prevents promoting caller-supplied summaries.
    pub(in super::super) struct ReceivedCompletion {
        /// Exactly one original state admitted by the normal-join guard.
        inner: ReceivedState,
    }

    /// A protected denial obtained solely through normal joined custody.
    #[cfg(test)]
    pub(in super::super) struct ReceivedProtectedDeny {
        /// Already applied effect, detached from every live authority owner.
        history: AppliedProtectedDeny,
    }

    #[cfg(test)]
    impl ReceivedProtectedDeny {
        /// Borrow the retained historical effect without deriving permission.
        pub(in super::super) const fn history(&self) -> &AppliedProtectedDeny {
            &self.history
        }

        /// Recover the detached history without its normal-join witness.
        pub(in super::super) fn into_history(self) -> AppliedProtectedDeny {
            self.history
        }
    }

    /// One normally joined operation, with a distinct control-only case.
    #[cfg_attr(
        test,
        expect(
            clippy::large_enum_variant,
            reason = "the existing fixed-size numerical probe remains inline across joined receiving"
        )
    )]
    pub(in super::super) enum ReceivedInvocation {
        /// Existing preview and ordinary protected result modes.
        Probe(ReceivedCompletion),
        /// Historical denial with no numerical or summary projection.
        #[cfg(test)]
        ProtectedDeny(ReceivedProtectedDeny),
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

    impl ReceivedCompletion {
        /// Preserve the existing CLI projection without publishing numerical authority.
        pub(in super::super) const fn summary(&self) -> &Summary {
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
        ) -> Result<(SourceConfigurationSnapshot, Summary), Failure> {
            let ReceivedState::Protected(received) = self.inner else {
                return Err(Failure::Control);
            };
            let JoinedProtectedHistory::Managed(managed) = received.history else {
                return Err(Failure::Control);
            };
            let ManagedProtectedHistory {
                history,
                configuration,
            }: ManagedProtectedHistory = *managed;
            Ok((configuration, *history.summary()))
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
        Managed(Box<ManagedProtectedHistory>),
    }

    impl JoinedProtectedHistory {
        /// Borrow the original captures and packet facts for either protected mode.
        const fn observation(&self) -> &ProtectedHistory {
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
        pub(in super::super) const fn summary(&self) -> &Summary {
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
        pub(in super::super) const fn history(&self) -> &ProtectedHistory {
            self.history.observation()
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
        let history: JoinedProtectedHistory = match joined
            .into_normal()
            .map_err(RunFailed::Abnormal)?
            .map_err(RunFailed::Operation)?
        {
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
            #[cfg(test)]
            InvocationCompletion::ManagedProtectedDeny(history) => {
                return Ok(ReceivedInvocation::ProtectedDeny(ReceivedProtectedDeny {
                    history,
                }));
            }
        };
        let original: &ProtectedHistory = history.observation();
        let measurement: Result<ExperimentalMeasurement, MeasurementRefusal> =
            crate::measurement::measure(
                profile,
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
        /// Borrow only the unchanged CLI count projection of this owning history.
        pub(in super::super) const fn summary(&self) -> &Summary {
            &self.summary
        }

        /// Inspect original authenticated bits/time for actual-owner component controls.
        #[cfg(test)]
        pub(in super::super) const fn receipt(&self) -> Receipt {
            self.receipt
        }

        /// Inspect only the original checked post-admission pool population.
        #[cfg(test)]
        pub(in super::super) const fn population(&self) -> (usize, usize) {
            self.population
        }

        /// Copy only the three original brackets; create no new acquisition fact.
        #[cfg(test)]
        pub(in super::super) const fn captures(&self) -> [SoftwareClock; 3] {
            [self.send_before, self.send_after, self.read_return]
        }

        /// Inspect actual historical endpoints without reissuing socket custody.
        #[cfg(test)]
        pub(in super::super) const fn endpoints(&self) -> (SocketAddr, SocketAddr) {
            (self.local, self.remote)
        }

        /// Borrow original socket identity without retaining a live resource.
        #[cfg(test)]
        pub(in super::super) const fn identity(&self) -> &Arc<()> {
            &self.identity
        }

        /// Borrow the invocation's actual pre-acquisition chart for identity controls.
        #[cfg(test)]
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

        /// Commit the first genuine retired protected request for a private successor.
        /// # Errors
        /// Preserve the shared final receipt and currentness refusals.
        #[cfg(test)]
        pub(in super::super) fn commit_for_successor(
            self,
            association: &mut AssociationOwner<'_>,
        ) -> Result<(), Failure> {
            let RetiredOperation {
                history: _,
                reply,
                attempts,
            }: RetiredOperation<'_> = self.into_retired();
            association
                .commit_retired_reply(reply, RetiredCommit::FirstRequest(attempts))
                .map(drop)
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
            let RetiredOperation {
                history,
                reply,
                #[cfg(test)]
                    attempts: _,
            }: RetiredOperation<'control> = self.into_retired();
            let configuration: Option<SourceConfigurationSnapshot> = association
                .protected_endpoint()?
                .map(|lease: &ProtectedEndpoint| lease.configuration().ok_or(Failure::Control))
                .transpose()?
                .cloned();
            let identity: Arc<()> =
                association.commit_retired_reply(reply, RetiredCommit::Terminal)?;
            Ok(CommittedProtected {
                operation: history,
                identity,
                configuration,
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
                #[cfg(test)]
                attempts: self.attempts,
            }
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
                local,
                remote,
                self.identity,
            ));
            summary
        }

        /// Detach only immutable history from this genuinely consumed final commit.
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
                control: _,
                receipt,
                send_before,
                send_after,
                read_return,
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
                population: (cookies, cookie_bytes),
                send_before,
                send_after,
                read_return,
                local,
                remote,
                identity: self.identity,
                run_context,
            };
            Ok(match self.configuration {
                Some(configuration) => {
                    InvocationCompletion::ManagedProtected(Box::new(ManagedProtectedHistory {
                        history,
                        configuration,
                    }))
                }
                None => InvocationCompletion::Protected(Box::new(history)),
            })
        }
    }

    /// Register the shared wake before reading actual scope and local state.
    /// A harmless wake re-waits within the same nonrenewable attempt allowance.
    /// # Errors
    /// Preserve cancellation/expiry/readiness failure with the original deadline.
    async fn wait(
        backend: &mut Backend,
        control: &Control,
        interest: Interest,
        attempts: &mut Attempts,
    ) -> Result<(), Failure> {
        loop {
            attempts.spend()?;
            let cancelled = control.cancelled.notified();
            tokio::pin!(cancelled);
            cancelled.as_mut().enable();
            drop(control.published_guard()?);
            let ready = tokio::select! {
                biased;
                () = &mut cancelled => false,
                () = tokio::time::sleep_until(tokio::time::Instant::from_std(control.expiry)) => return Err(Failure::Deadline),
                ready = backend.ready(interest, control) => {
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
            ReceiveMode::Ordinary,
        )
        .await
        .and_then(SelectedOperation::into_ordinary)
    }

    /// One actual datagram and its original adapter/capture observations.
    struct ObservedReply<'a> {
        /// Original selected bytes, excluding unused buffer capacity.
        packet: &'a [u8],
        /// Peer returned by the actual selector.
        remote: SocketAddr,
        /// Exact concrete socket/back-end identity.
        backend: &'a Backend,
        /// Capture preceding the actual synchronous send.
        send_before: SoftwareClock,
        /// Capture following the same synchronous send.
        send_after: SoftwareClock,
        /// Capture before any interpretation of the selected packet.
        read_return: SoftwareClock,
        /// Local packet extent, independent of either received length or peer fields.
        packet_bytes: usize,
    }

    /// The caller selects whether the already-authenticated scripted leg can return a control.
    #[derive(Clone, Copy)]
    enum ReceiveMode {
        /// Existing ordinary one-shot and component paths.
        Ordinary,
        /// The closed scripted successor permits both admitted dispositions.
        #[cfg(test)]
        ProtectedSuccessor,
    }

    /// One concrete async driver result shared by both receive modes.
    #[cfg_attr(
        test,
        expect(
            clippy::large_enum_variant,
            reason = "keep the bounded selected reply and transfer inline before resource retirement"
        )
    )]
    enum SelectedOperation<'control> {
        /// Ordinary timing and replacement obtained through the original ledger.
        Ordinary(OperationCompletion<'control>),
        /// The charged control evidence cannot produce ordinary reply history.
        #[cfg(test)]
        Deny(StagedProtectedDeny<'control>),
    }

    #[cfg(test)]
    impl<'control> SelectedOperation<'control> {
        /// Preserve the existing ordinary result type at its callers.
        /// # Errors
        /// Refuse a closed control disposition if an ordinary caller receives one.
        fn into_ordinary(self) -> Result<OperationCompletion<'control>, Failure> {
            match self {
                Self::Ordinary(operation) => Ok(operation),
                Self::Deny(_) => Err(Failure::Control),
            }
        }
    }

    /// Admit one charged disposition through the selected path in the same ledger.
    /// # Errors
    /// Preserve original endpoint, identity, budget, time and authentication refusals.
    fn stage_observed_reply<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        reply: &ObservedReply<'_>,
        mode: ReceiveMode,
    ) -> Result<SelectedOperation<'control>, Failure> {
        let arrival: Instant = reply.read_return.monotonic();
        let identity: &Arc<()> = &reply.backend.identity;
        let (receipt, replacement, submission): (Receipt, CookieReplacement, PendingSubmission) =
            match mode {
                ReceiveMode::Ordinary => association.stage_observed_reply(
                    reply.packet,
                    reply.remote,
                    arrival,
                    identity,
                )?,
                #[cfg(test)]
                ReceiveMode::ProtectedSuccessor => match association.stage_protected_successor(
                    control,
                    reply.packet,
                    reply.remote,
                    arrival,
                    identity,
                )? {
                    StagedProtectedSuccessor::Ordinary(receipt, replacement, submission) => {
                        (receipt, replacement, submission)
                    }
                    StagedProtectedSuccessor::Deny(staged) => {
                        return Ok(SelectedOperation::Deny(staged));
                    }
                },
            };
        let (cookies, cookie_bytes): (usize, usize) = replacement.population();
        Ok(SelectedOperation::Ordinary(OperationCompletion {
            history: OperationHistory {
                control,
                receipt,
                send_before: reply.send_before,
                send_after: reply.send_after,
                read_return: reply.read_return,
                local: reply.backend.local,
                remote: reply.backend.remote,
                packet_bytes: reply.packet_bytes,
                cookies,
                cookie_bytes,
            },
            replacement,
            submission,
        }))
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
        mode: ReceiveMode,
    ) -> Result<SelectedOperation<'control>, Failure> {
        if association.check_context(control)? != backend.remote {
            return Err(Failure::Endpoint);
        }
        let ticket: Ticket<'_> = association.reserve(RequestInput::Observed)?;
        association.prepare_observed(&ticket, fill)?;
        drive_prepared(association, control, backend, attempts, ticket, mode).await
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
        mode: ReceiveMode,
    ) -> Result<SelectedOperation<'control>, Failure> {
        wait(backend, control, Interest::Write, attempts).await?;
        backend.send_prepared(association, ticket, control)?;
        let (send_before, send_after, packet_bytes) =
            backend.send.ok_or(failed(DatagramError::Capture))?;
        check_capture_order(send_before.monotonic_after(), send_after.monotonic())?;
        let mut buffer = Zeroizing::new([0; RECEIVE_BYTES]);
        wait(backend, control, Interest::Read, attempts).await?;
        loop {
            attempts.spend()?;
            drop(control.published_guard()?);
            let (read, observed) = backend.receive(&mut buffer);
            match read {
                Read::WouldBlock => wait(backend, control, Interest::Read, attempts).await?,
                Read::Failed(kind) => return Err(failed(DatagramError::Receive(kind))),
                Read::ConsumedUnknown => {
                    association.charge_consumed_unknown()?;
                    return Err(failed(DatagramError::Capture));
                }
                Read::Datagram(length, remote) => {
                    check_capture_order(send_after.monotonic_after(), observed.monotonic())?;
                    let packet = buffer.get(..length).ok_or(failed(DatagramError::Capture))?;
                    let reply: ObservedReply<'_> = ObservedReply {
                        packet,
                        remote,
                        backend,
                        send_before,
                        send_after,
                        read_return: observed,
                        packet_bytes,
                    };
                    match stage_observed_reply(association, control, &reply, mode) {
                        Ok(selected) => return Ok(selected),
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
        #[cfg(test)]
        association.check_fresh_exchange()?;
        exchange_with_attempts(association, control, fill, make_backend, Attempts(ATTEMPTS))
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
            ReceiveMode::Ordinary,
        )?;
        #[cfg(test)]
        let operation: OperationCompletion<'control> = selected.into_ordinary()?;
        #[cfg(not(test))]
        let SelectedOperation::Ordinary(operation) = selected;
        Ok(ProtectedCompletion {
            operation,
            #[cfg(test)]
            attempts,
        })
    }

    /// Drive only the closed Script backend for either private successor disposition.
    /// # Errors
    /// Preserve original continuation, adapter, charged receive and policy refusals.
    #[cfg(test)]
    pub(super) fn exchange_successor_mixed<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
        make_script: impl FnOnce(SocketAddr) -> Script,
    ) -> Result<RetiredProtectedSuccessor<'control>, Failure> {
        let mut attempts: Attempts = association.take_authenticated_successor(control)?;
        let selected: SelectedOperation<'control> = retire_operation(
            association,
            control,
            fill,
            |remote: SocketAddr| Ok(super::tests::backend(remote, make_script(remote))),
            &mut attempts,
            ReceiveMode::ProtectedSuccessor,
        )?;
        match selected {
            SelectedOperation::Ordinary(operation) => {
                Ok(RetiredProtectedSuccessor::Ordinary(ProtectedCompletion {
                    operation,
                    attempts,
                }))
            }
            SelectedOperation::Deny(staged) => Ok(RetiredProtectedSuccessor::Deny(
                RetiredProtectedDeny(staged),
            )),
        }
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
        mode: ReceiveMode,
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
                operation_with_attempts(association, control, fill, &mut backend, attempts, mode)
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
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

        for (loopback, foreign) in [
            (
                IpAddr::V4(Ipv4Addr::LOCALHOST),
                IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1)),
            ),
            (
                IpAddr::V6(Ipv6Addr::LOCALHOST),
                IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1)),
            ),
        ] {
            let local: SocketAddr = SocketAddr::new(loopback, 49000);
            let remote: SocketAddr = SocketAddr::new(loopback, 4460);
            assert_eq!(check_bound_endpoints(local, remote), Ok(()));
            assert_eq!(
                check_bound_endpoints(SocketAddr::new(foreign, local.port()), remote),
                Err(Failure::Endpoint),
            );
            assert_eq!(
                check_bound_endpoints(SocketAddr::new(loopback, 0), remote),
                Err(Failure::Endpoint),
            );
            assert_eq!(
                check_bound_endpoints(local, SocketAddr::new(foreign, remote.port())),
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
        use std::sync::Weak;
        use std::time::Duration;

        let loopback: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 4460));
        let foreign: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 4460));
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
            assert_eq!(registered.ip(), loopback.ip());
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
pub(super) use reached::exchange;

#[cfg(test)]
pub(super) use reached::{Attempts, RetiredProtectedDeny, RetiredProtectedSuccessor};

#[cfg(any(windows, test))]
pub(super) use reached::{
    CommittedProtected, CompletionProjection, GuardedSender, ManagedProtectedHistory,
    ProtectedCompletion, ProtectedHistory, ReceivedCompletion, ReceivedInvocation, RetiredReply,
    RetiredReplyParts, receive_joined,
};

#[cfg(test)]
#[path = "datagram/tests.rs"]
pub(super) mod tests;
