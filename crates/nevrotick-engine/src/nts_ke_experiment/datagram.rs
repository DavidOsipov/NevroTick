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
    use crate::nts_ke_experiment::association::ledger::{
        Receipt, RequestInput, RequestPhase, SendOutcome,
    };
    use crate::nts_ke_experiment::job::ReceivingOutcome;
    use crate::nts_ke_experiment::{
        Control, Failure, InvocationCompletion, IoPhase, RunFailed, Summary,
    };
    use crate::{
        ExperimentalMeasurement, ExperimentalProfile, MeasurementRefusal, RunClockContext,
        SoftwareClock,
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
    pub(super) struct Attempts(usize);

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
            let local = socket
                .local_addr()
                .map_err(|error| failed(DatagramError::Local(error.kind())))?;
            if !local.ip().is_loopback() || local.port() == 0 || !remote.ip().is_loopback() {
                return Err(Failure::Endpoint);
            }
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

        /// Serialize actual cancellation acknowledgement with one concrete send.
        ///
        /// No arbitrary callback, allocation, formatter, crypto or await runs
        /// under this guard. The closed test branch returns predeclared facts.
        /// # Errors
        /// Refuse stale authority before the syscall and retain the original cause.
        pub(super) fn send_guarded(
            &mut self,
            control: &Control,
            packet: &[u8],
            remote: SocketAddr,
        ) -> Result<SendOutcome, Failure> {
            if remote != self.remote || self.send.is_some() {
                return Err(Failure::Endpoint);
            }
            #[cfg(test)]
            match &mut self.io {
                #[cfg(windows)]
                Io::Native(_) => {}
                Io::Script(script) => script.before_guard(control)?,
            }
            let guard = control.published_guard()?;
            let before = SoftwareClock::capture();
            guard.remaining(IoPhase::Exchange, before.monotonic_after())?;
            let (outcome, cancel_after) = match &mut self.io {
                #[cfg(windows)]
                Io::Native(socket) => (
                    match socket.try_send_to(packet, remote) {
                        Ok(length) => SendOutcome::Completed(length),
                        Err(error) => SendOutcome::Failed(error.kind()),
                    },
                    false,
                ),
                #[cfg(test)]
                Io::Script(script) => script.send(packet),
            };
            let after = SoftwareClock::capture();
            drop(guard);
            self.send = Some((before, after, packet.len()));
            if cancel_after {
                control.cancel()?;
            }
            Ok(outcome)
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

    /// Translate only the compiled platform's consumed-oversize code.
    pub(super) fn classify_read(result: io::Result<(usize, SocketAddr)>) -> Read {
        match result {
            Ok((RECEIVE_BYTES, _)) => Read::ConsumedUnknown,
            Ok((length, source)) => Read::Datagram(length, source),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => Read::WouldBlock,
            Err(error)
                if (cfg!(windows) && error.raw_os_error() == Some(10040))
                    || (cfg!(target_os = "linux") && error.raw_os_error() == Some(90)) =>
            {
                Read::ConsumedUnknown
            }
            Err(error) => Read::Failed(error.kind()),
        }
    }

    /// Check inclusive ordering of two already captured monotonic coordinates.
    ///
    /// This pure helper grants no clock provenance or current authority. Its
    /// callers supply the actual send-end/send-return or send-return/read-return
    /// captures and retain their separate original-deadline/control checks.
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

    /// Historical operation result; no final-commit API exists on this raw value.
    pub(super) struct OperationCompletion<'control> {
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
        /// Same socket instance as the operation, not inferred from numeric endpoints.
        pub(super) identity: Arc<()>,
        /// Actual complete protected request extent, never a fixed 48-byte assumption.
        pub(super) packet_bytes: usize,
        /// Post-admission pool entries retained under the original context's budgets.
        pub(super) cookies: usize,
        /// Corresponding checked aggregate opaque pool payload, without copying bytes.
        pub(super) cookie_bytes: usize,
    }

    /// Opaque receiving input, constructed only after backend/runtime retirement.
    /// Neither historical receipts nor raw operation results can construct it.
    pub(in super::super) struct ProtectedCompletion<'control> {
        /// Same genuine operation, with its original context and complete history.
        operation: OperationCompletion<'control>,
    }

    /// One committed protected exchange, not live permission or Core eligibility.
    /// No Clone/Copy or public raw constructor can duplicate this fact.
    pub(in super::super) struct CommittedProtected<'control> {
        /// Actual history survives until its one consuming receiver projection.
        operation: OperationCompletion<'control>,
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
        /// Existing bounded initial-KE population and actual send counts.
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

    /// One normally joined result; numerical refusal is retained data, never a retry.
    #[expect(
        clippy::large_enum_variant,
        reason = "One consume-once result keeps fixed-size numerics inline without adding an allocation after completed work."
    )]
    pub(in super::super) enum ReceivedCompletion {
        /// The actual never-send operation supplied no protected history to adapt.
        Preview(Summary),
        /// Original owning history coupled to its exact optional model and result.
        Protected(ReceivedProtectedNumerics),
    }

    impl ReceivedCompletion {
        /// Preserve the existing CLI projection without publishing numerical authority.
        pub(in super::super) const fn summary(&self) -> &Summary {
            match self {
                Self::Preview(summary) => summary,
                Self::Protected(received) => received.history.summary(),
            }
        }
    }

    /// Immutable historical calculation, constructible only at the normal-join guard.
    /// This retains no native owner and supplies no current freshness or eligibility.
    pub(in super::super) struct ReceivedProtectedNumerics {
        /// Same complete history moved from the actual retired worker result.
        history: Box<ProtectedHistory>,
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
        #[cfg_attr(
            not(test),
            expect(
                dead_code,
                reason = "the private result awaits a qualified production numerical consumer"
            )
        )]
        measurement: Result<ExperimentalMeasurement, MeasurementRefusal>,
    }

    #[cfg(test)]
    impl ReceivedProtectedNumerics {
        /// Borrow unchanged genuine history; never split off or rebind its provenance.
        pub(in super::super) const fn history(&self) -> &ProtectedHistory {
            &self.history
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
    ) -> Result<ReceivedCompletion, RunFailed> {
        match joined
            .into_normal()
            .map_err(RunFailed::Abnormal)?
            .map_err(RunFailed::Operation)?
        {
            InvocationCompletion::Preview(summary) => Ok(ReceivedCompletion::Preview(summary)),
            InvocationCompletion::Protected(history) => {
                let measurement = crate::measurement::measure(
                    profile,
                    history.send_before,
                    history.send_after,
                    history.read_return,
                    history.receipt.accepted_at(),
                    history.receipt.fields().header(),
                );
                Ok(ReceivedCompletion::Protected(ReceivedProtectedNumerics {
                    history,
                    profile,
                    measurement,
                }))
            }
        }
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
        pub(super) const fn history(&self) -> &OperationCompletion<'control> {
            &self.operation
        }

        /// Consume this retired operation at its actual current receiving owner.
        ///
        /// Exclusive owner borrowing spans validation and the final commit. There
        /// is no callback, await or retained lock between the two fresh guards.
        /// A late stop/expiry still wins the existing final Control decision.
        /// # Errors
        /// Refuse foreign/stale owner, later charged work, stop or expiry. Refusal
        /// creates no fact and restores no request, cookie, deadline or work credit.
        pub(in super::super) fn commit(
            self,
            association: &mut AssociationOwner<'_>,
        ) -> Result<CommittedProtected<'control>, Failure> {
            association.check_completed(self.operation.control, &self.operation.receipt)?;
            self.operation.control.commit()?;
            Ok(CommittedProtected {
                operation: self.operation,
            })
        }
    }

    impl CommittedProtected<'_> {
        /// Bind this historical fact to the receiving transaction without reopening it.
        /// # Errors
        /// Refuse another actual Control; no Published check follows commit.
        pub(in super::super) fn check_control(&self, control: &Control) -> Result<(), Failure> {
            if std::ptr::eq(self.operation.control, control) {
                Ok(())
            } else {
                Err(Failure::Control)
            }
        }

        /// Consume the historical fact once; return only existing bounded counts.
        /// No Published check follows commit, and later stop cannot rewrite history.
        #[cfg(test)]
        pub(in super::super) fn into_summary(self) -> Summary {
            let OperationCompletion {
                control,
                receipt,
                send_before,
                send_after,
                read_return,
                local,
                remote,
                identity,
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
                identity,
            ));
            summary
        }

        /// Detach only immutable history from this genuinely consumed final commit.
        /// The actual receiver supplies its original chart and already checked initial
        /// population after binding this fact to its current Control. No live owner,
        /// secret payload, numerical measurement or renewed timestamp is returned.
        pub(in super::super) fn into_history(
            self,
            run_context: Arc<RunClockContext>,
            population: (usize, usize),
        ) -> ProtectedHistory {
            let OperationCompletion {
                control: _,
                receipt,
                send_before,
                send_after,
                read_return,
                local,
                remote,
                identity,
                packet_bytes,
                cookies,
                cookie_bytes,
            } = self.operation;
            ProtectedHistory {
                summary: Summary {
                    cookies: population.0,
                    cookie_bytes: population.1,
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
                identity,
                run_context,
            }
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

    /// Drive the same bounded packet owner with either concrete backend.
    /// # Errors
    /// Retain original adapter, currentness and packet refusals; never retry a send.
    pub(super) async fn operation<'control>(
        association: &mut AssociationOwner<'_>,
        control: &'control Control,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
        backend: &mut Backend,
    ) -> Result<OperationCompletion<'control>, Failure> {
        if association.check_context(control)? != backend.remote {
            return Err(Failure::Endpoint);
        }
        let ticket = association.reserve(RequestInput::Observed)?;
        association.prepare_observed(&ticket, fill)?;
        let mut attempts = Attempts(ATTEMPTS);
        wait(backend, control, Interest::Write, &mut attempts).await?;
        let mut submission_failure = None;
        let submitted = association.invoke_sender(ticket, |packet, remote| {
            match backend.send_guarded(control, packet, remote) {
                Ok(outcome) => outcome,
                Err(error) => {
                    submission_failure = Some(error);
                    SendOutcome::Unknown
                }
            }
        });
        if let Some(error) = submission_failure {
            return Err(error);
        }
        submitted?;
        let (send_before, send_after, packet_bytes) =
            backend.send.ok_or(failed(DatagramError::Capture))?;
        check_capture_order(send_before.monotonic_after(), send_after.monotonic())?;
        let mut buffer = Zeroizing::new([0; RECEIVE_BYTES]);
        wait(backend, control, Interest::Read, &mut attempts).await?;
        loop {
            attempts.spend()?;
            drop(control.published_guard()?);
            let (read, observed) = backend.receive(&mut buffer);
            match read {
                Read::WouldBlock => wait(backend, control, Interest::Read, &mut attempts).await?,
                Read::Failed(kind) => return Err(failed(DatagramError::Receive(kind))),
                Read::ConsumedUnknown => {
                    association.charge_consumed_unknown()?;
                    return Err(failed(DatagramError::Capture));
                }
                Read::Datagram(length, remote) => {
                    check_capture_order(send_after.monotonic_after(), observed.monotonic())?;
                    let packet = buffer.get(..length).ok_or(failed(DatagramError::Capture))?;
                    match association.receive_reply(packet, remote, observed.monotonic()) {
                        Ok(receipt) => {
                            let (cookies, cookie_bytes) = association.cookie_population()?;
                            return Ok(OperationCompletion {
                                control,
                                receipt,
                                send_before,
                                send_after,
                                read_return: observed,
                                local: backend.local,
                                remote: backend.remote,
                                identity: Arc::clone(&backend.identity),
                                packet_bytes,
                                cookies,
                                cookie_bytes,
                            });
                        }
                        Err(error) => {
                            if !matches!(association.status().phase, RequestPhase::Pending) {
                                return Err(error);
                            }
                            if matches!(
                                error,
                                Failure::Cancelled | Failure::Deadline | Failure::Control
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
            let completion = operation(association, control, fill, &mut backend).await;
            drop(backend);
            completion
        });
        drop(runtime);
        result.map(|operation| ProtectedCompletion { operation })
    }

    #[cfg(test)]
    use super::tests::Script;
}

#[cfg(windows)]
pub(super) use reached::exchange;

#[cfg(any(windows, test))]
pub(super) use reached::{
    CommittedProtected, ProtectedCompletion, ProtectedHistory, ReceivedCompletion, receive_joined,
};

#[cfg(test)]
#[path = "datagram/tests.rs"]
mod tests;
