// SPDX-License-Identifier: GPL-2.0-only
//! One bounded laboratory KE job; this module grants no product authority.
//!
//! CLI construction checks immutable endpoint/identity inputs before effects.
//! Response helpers produce only inert negotiation/cookie data. Only the Windows
//! transport child can complete the real TLS/exporter operation. Cancellation
//! and the final time observation share a serialized, consume-once commit.

use std::ffi::OsString;
use std::io::{self, Read};
use std::net::{IpAddr, SocketAddr};
#[cfg(any(windows, test))]
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

#[cfg(test)]
use garde::Validate;
use ntp_wire::{
    NtsKeAccumulator, NtsKeAccumulatorError, NtsKeExporterContext, NtsKeExporterDirection,
    NtsKeFrameLimits, NtsKeNegotiationError, NtsKeNtpv4Offer, NtsKeProgress,
    NtsKeRequestEncodeError, encode_nts_ke_ntpv4_request, negotiate_nts_ke_ntpv4,
};
use rustls_pki_types::ServerName;
#[cfg(windows)]
use windows_platform_native::{ResumeUnregistrationError, ResumeWatch};
use zeroize::Zeroizing;

mod association;
mod datagram;
#[cfg(any(windows, test))]
mod job;
#[cfg(any(windows, test))]
mod power_target;
mod process;
mod profile;
#[cfg(any(windows, test))]
mod recovery;

#[cfg(windows)]
use association::AssociationCandidate;
#[cfg(any(windows, test))]
use association::AssociationOwner;
pub(crate) use association::ledger::{ControlSubmission, VerifiedRetiredControl};
#[cfg(any(windows, test))]
use process::ProcessAdmission;
#[cfg(any(windows, test))]
use process::ProviderScope;
pub(crate) use response::ProtectedControlKind;

/// Process-wide one-shot admission; cached CNG providers cannot be reinitialized.
#[cfg(any(windows, test))]
static PROCESS_ADMISSION: ProcessAdmission = ProcessAdmission::new();

/// Permanent custody of the one submitted laboratory worker and its sealed outcome.
#[cfg(windows)]
static PROCESS_WORKER: job::WorkerSlot = job::WorkerSlot::new();

mod handoff;
#[cfg(any(windows, test))]
pub(super) mod managed_preview;
#[cfg(test)]
mod managed_preview_tests;
mod request;
mod response;
#[cfg(test)]
mod tests;
#[cfg(windows)]
mod transport;
mod transport_fault;

/// Maximum complete plaintext response, including record framing.
const RESPONSE_LIMIT: usize = 131_072;
/// Maximum record count, including the sole final EOM.
const RECORD_LIMIT: usize = 32_768;
/// Maximum application-owned read scratch, wiped on ordinary drop.
const SCRATCH_LIMIT: usize = 4_096;
/// Maximum opaque initial cookie count; even empty cookies consume a slot.
const COOKIE_LIMIT: usize = 8;
/// Independent defensive aggregate limit on copied cookie payload.
const COOKIE_BYTES_LIMIT: usize = 131_072;
/// Connect and full handshake share this duration from the original origin.
const HANDSHAKE_TIME: Duration = Duration::from_secs(20);
/// KE publication and ephemeral expiry share this original-origin duration.
const EXCHANGE_TIME: Duration = Duration::from_secs(60);

/// Closed failures contain no identity, endpoint, packet or library-error text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Failure {
    /// Raw CLI input fails exact admission.
    Arguments,
    /// A bounded allocation was refused.
    Allocation,
    /// Fresh monotonic time is outside a strict-before deadline.
    Deadline,
    /// Cancellation won before commit.
    Cancelled,
    /// The control lock or consume-once state is unavailable.
    Control,
    /// A process invocation was already claimed, including after its retirement.
    ProcessUsed,
    /// The invocation already spent its sole never-send preview allowance.
    PreviewSpent,
    /// The actual managed source or installed Preview definition refused admission.
    #[cfg(any(windows, test))]
    SourcePolicy(crate::SourcePolicyError),
    /// Closed request-custody or protected-response refusal; no secret payload.
    Ledger(association::ledger::LedgerError),
    /// Closed protected-datagram adapter failure, without packet or endpoint text.
    Datagram(datagram::DatagramError),
    /// Fixed encoder parameters or the actual request were rejected.
    Offer(NtsKeRequestEncodeError),
    /// Fixed framing limits were rejected before parsing.
    Limits,
    /// A response read failed with this payload-free typed category.
    Response(io::ErrorKind),
    /// The actual direct TLS wrapper supplied a bounded typed refusal.
    #[cfg(any(windows, test))]
    Tls(transport_fault::TlsFailure),
    /// The actual incremental framing operation refused its input.
    Frame(NtsKeAccumulatorError),
    /// Actual offer/response negotiation refused its input.
    Negotiation(NtsKeNegotiationError),
    /// Cookie count, aggregate size or a checked copy exceeded its budget.
    Cookies,
    /// The actual provider refused the one bounded UID/nonce entropy request.
    Entropy,
    /// The private fixed-shape request receiver refused its exact input.
    Packet(request::RequestError),
    /// An unadmitted endpoint hint or zero port was supplied.
    Endpoint,
    /// Connecting the one numeric peer failed.
    #[cfg(any(windows, test))]
    Connect(io::ErrorKind),
    /// The actual connected endpoint differs from the admitted endpoint.
    #[cfg(windows)]
    Peer,
    /// Explicit provider, verifier or TLS configuration construction failed.
    #[cfg(any(windows, test))]
    TlsSetup(transport_fault::SetupStage),
    /// The real handshake failed; arbitrary error text is not retained.
    #[cfg(any(windows, test))]
    Handshake(io::ErrorKind),
    /// Negotiated protocol, ALPN or full-handshake requirements did not hold.
    #[cfg(windows)]
    TlsProfile,
    /// Complete request write or its mandatory flush failed.
    #[cfg(any(windows, test))]
    Request(io::ErrorKind),
    /// A regular same-connection exporter failed.
    #[cfg(windows)]
    Exporter,
    /// Required local close-notify flush failed.
    #[cfg(any(windows, test))]
    Shutdown(io::ErrorKind),
}

/// Admitted immutable inputs for one numeric, loopback-only laboratory job.
struct Config {
    /// Actual connection destination; no DNS or address alternative is selected.
    peer: SocketAddr,
    /// Separate, bounded original verification identity and admitted hint mapping.
    identity: String,
    /// The standard checked DNS/IP identity used by the actual TLS constructor.
    server_name: ServerName<'static>,
    /// Explicit one-shot selection; two-argument compatibility never sends UDP.
    mode: ExchangeMode,
    /// Test-only destruction witness owned by this actual checked Config.
    /// The private constructor creates it; no validation or authority is delegated.
    #[cfg(test)]
    lifetime: Arc<()>,
}

/// Closed experimental operation selection, never a production acquisition mode.
enum ExchangeMode {
    /// Default: seal the laboratory preview and expose no packet to a sender.
    Preview,
    /// Explicit request for one bounded protected loopback datagram operation.
    ProtectedDatagram,
    /// A genuine installed definition permits only the never-send Preview path.
    #[cfg(any(windows, test))]
    ManagedPreview(crate::source_policy::ConfiguredKePreview),
    /// An explicit same-process source authority permits one managed protected operation.
    ManagedProtected(crate::source_policy::ProtectedKeSource),
}

impl ExchangeMode {
    /// Borrow checked shared TLS/UDP inputs while preserving each closed operation selection.
    fn source_definition(&self) -> Option<&crate::source_policy::ConfiguredKePreview> {
        match self {
            Self::ManagedPreview(definition) => Some(definition),
            Self::ManagedProtected(source) => Some(source.definition()),
            Self::Preview | Self::ProtectedDatagram => None,
        }
    }
}

impl std::fmt::Debug for ExchangeMode {
    /// Keep managed configuration, endpoint and verification-name payloads redacted.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Preview => "Preview",
            Self::ProtectedDatagram => "ProtectedDatagram",
            #[cfg(any(windows, test))]
            Self::ManagedPreview(_) => "ManagedPreview { .. }",
            Self::ManagedProtected(_) => "ManagedProtected { .. }",
        })
    }
}

impl PartialEq for ExchangeMode {
    /// A managed mode compares its actual issued immutable configuration identity.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Preview, Self::Preview) | (Self::ProtectedDatagram, Self::ProtectedDatagram) => {
                true
            }
            (Self::ManagedPreview(left), Self::ManagedPreview(right)) => {
                left.snapshot().same_configuration(right.snapshot())
            }
            (Self::ManagedProtected(left), Self::ManagedProtected(right)) => left
                .definition()
                .snapshot()
                .same_configuration(right.definition().snapshot()),
            _ => false,
        }
    }
}

impl Eq for ExchangeMode {}

impl Config {
    /// Admit two required arguments and at most one exact operation flag.
    /// # Errors
    /// Refuse missing/extra arguments and every unrecognized mode before effects.
    fn from_arguments(mut arguments: impl Iterator<Item = OsString>) -> Result<Self, Failure> {
        let peer = arguments.next().ok_or(Failure::Arguments)?;
        let identity = arguments.next().ok_or(Failure::Arguments)?;
        let mode = match arguments.next() {
            None => ExchangeMode::Preview,
            Some(flag) if flag == "--protected-datagram" => ExchangeMode::ProtectedDatagram,
            Some(_) => return Err(Failure::Arguments),
        };
        if arguments.next().is_some() {
            return Err(Failure::Arguments);
        }
        let mut config = Self::new(peer, identity)?;
        config.mode = mode;
        Ok(config)
    }

    /// Parse exact bounded inputs before any provider or connection is touched.
    ///
    /// Invalid Unicode, names, ports, non-loopback peers and scoped IPv6 refuse.
    ///
    /// # Errors
    /// Refuses invalid Unicode, lengths, address/name syntax, port zero, non-loopback peers
    /// and every scoped IPv6 spelling before effects.
    fn new(peer: OsString, identity: OsString) -> Result<Self, Failure> {
        if peer.len() > 128 || identity.len() > 253 {
            return Err(Failure::Arguments);
        }
        let peer: String = peer.into_string().map_err(|_| Failure::Arguments)?;
        if peer.contains('%') {
            return Err(Failure::Arguments);
        }
        let peer: SocketAddr = peer.parse().map_err(|_| Failure::Arguments)?;
        let identity: String = identity.into_string().map_err(|_| Failure::Arguments)?;
        let target: crate::KePreviewTarget =
            crate::KePreviewTarget::new(peer, &identity).map_err(|_| Failure::Arguments)?;
        Ok(Self {
            peer: target.peer(),
            identity: target.identity().to_owned(),
            server_name: target.server_name().clone(),
            mode: ExchangeMode::Preview,
            #[cfg(test)]
            lifetime: Arc::new(()),
        })
    }

    /// Derive every connection input from the actual source-issued Preview definition.
    /// Allocation is bounded by the already checked verification-name envelope.
    #[cfg(any(windows, test))]
    fn from_managed_preview(context: crate::source_policy::ConfiguredKePreview) -> Self {
        let target: crate::KePreviewTarget = context.target().clone();
        Self::from_managed_target(&target, ExchangeMode::ManagedPreview(context))
    }

    /// Derive the TLS target exclusively from the separately authorized protected selection.
    fn from_managed_protected(source: crate::source_policy::ProtectedKeSource) -> Self {
        let target: crate::KePreviewTarget = source.definition().target().clone();
        Self::from_managed_target(&target, ExchangeMode::ManagedProtected(source))
    }

    /// Keep a single checked TLS-input projection. Each managed guard verifies
    /// these fields against its own originally issued definition before use.
    fn from_managed_target(target: &crate::KePreviewTarget, mode: ExchangeMode) -> Self {
        Self {
            peer: target.peer(),
            identity: target.identity().to_owned(),
            server_name: target.server_name().clone(),
            mode,
            #[cfg(test)]
            lifetime: Arc::new(()),
        }
    }
}

/// One maintained structured-input rule runs inside the private constructor.
#[derive(Validate)]
#[cfg(test)]
struct ConfigCandidate<'a> {
    /// Parsed endpoint and exact bounded verification identity; no effects occur.
    #[garde(custom(config_rule))]
    values: (SocketAddr, &'a str),
}

/// Enforce the loopback and literal-identity contract before provider admission.
/// # Errors
/// Return a fixed redacted violation for invalid endpoint/name envelope.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow fields and their unit context"
)]
#[cfg(test)]
fn config_rule(values: &(SocketAddr, &str), (): &()) -> garde::Result {
    let (peer, identity): (SocketAddr, &str) = *values;
    match crate::KePreviewTarget::new(peer, identity) {
        Ok(_) => Ok(()),
        Err(_) => Err(garde::Error::new("invalid loopback KE configuration")),
    }
}

/// Socket phase chooses a deadline, never a fresh relative timeout budget.
#[derive(Clone, Copy)]
enum IoPhase {
    /// Connect plus TLS must both finish before origin plus twenty seconds.
    Handshake,
    /// All subsequent KE work remains before origin plus sixty seconds.
    Exchange,
}

/// Serialized terminal phase; historical commit cannot be rewritten by stop.
#[derive(Clone, Copy)]
enum Phase {
    /// The sole job still has permission to prepare and commit.
    Open,
    /// Genuine KE completion was consumed; invocation currentness remains live.
    Published,
    /// Cancellation has irrevocably closed publication.
    Cancelled,
    /// Complete publication already won its final guard.
    Committed,
}

/// Explicit result of cancellation competing with the one commit point.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CancelOutcome {
    /// This call closed publication.
    Cancelled,
    /// Publication was already closed by an earlier cancellation.
    AlreadyCancelled,
    /// Historical completion already committed; this stop is too late.
    AlreadyCommitted,
}

/// One invocation's irreversible admission latch, independent of local cleanup.
struct StopScope {
    /// Only the actual stop endpoint may change this value, from false to true.
    stopped: Mutex<bool>,
    /// A wake carries no cancellation or actual-completion acknowledgement.
    wake: Arc<tokio::sync::Notify>,
}

impl StopScope {
    /// Create one fresh scope before any context can receive its custody.
    fn new() -> Self {
        Self {
            stopped: Mutex::new(false),
            wake: Arc::new(tokio::sync::Notify::new()),
        }
    }

    /// Retain this actual scope's gate without granting admission by a snapshot.
    /// # Errors
    /// Refuse a poisoned gate with the existing closed control cause.
    fn lock(&self) -> Result<ScopeGuard<'_>, Failure> {
        Ok(ScopeGuard {
            scope: self,
            stopped: self.stopped.lock().map_err(|_| Failure::Control)?,
        })
    }
}

/// Serialized access to one actual invocation's irreversible admission latch.
struct ScopeGuard<'scope> {
    /// Actual scope owning the mutex, retained for mandatory Control binding.
    scope: &'scope StopScope,
    /// The held gate prevents a concurrent stop from crossing an admitted effect.
    stopped: MutexGuard<'scope, bool>,
}

impl ScopeGuard<'_> {
    /// Inspect only the actual latch while this scope's gate remains held.
    fn is_stopped(&self) -> bool {
        *self.stopped
    }

    /// Check the actual latch while retaining the admission gate.
    /// # Errors
    /// Refuse an already stopped invocation without reopening it.
    fn ensure_open(&self) -> Result<(), Failure> {
        if self.is_stopped() {
            Err(Failure::Cancelled)
        } else {
            Ok(())
        }
    }
}

/// A stop observation closes admission; it never acknowledges work completion.
#[cfg(any(windows, test))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StopDisposition {
    /// This request irreversibly changed the invocation's admission latch.
    Latched,
    /// An earlier request already closed this same invocation's admission.
    AlreadyLatched,
}

/// A bounded capability can only stop its actual original invocation scope.
#[cfg(any(windows, test))]
#[derive(Clone)]
struct StopHandle {
    /// Keep the same latch alive without retaining or replacing any Control.
    scope: Arc<StopScope>,
}

#[cfg(any(windows, test))]
impl StopHandle {
    /// Latch stop once without claiming that any admitted operation returned.
    /// # Errors
    /// Refuse an unavailable scope lock with the existing closed control cause.
    fn request_stop(&self) -> Result<StopDisposition, Failure> {
        let mut stopped = self.scope.stopped.lock().map_err(|_| Failure::Control)?;
        let disposition = if *stopped {
            StopDisposition::AlreadyLatched
        } else {
            *stopped = true;
            StopDisposition::Latched
        };
        drop(stopped);
        if disposition == StopDisposition::Latched {
            self.scope.wake.notify_waiters();
        }
        Ok(disposition)
    }
}

/// One transaction's immutable time bounds and serialized publication authority.
struct Control {
    /// Exact instance identity for the worker's write-only first-receipt checkpoint.
    first_receipt_identity: Arc<()>,
    /// Selected successor accounting, written by the actual datagram adapter.
    datagram_work: Option<Arc<datagram::ProtectedDatagramWork>>,
    /// Original monotonic origin; earlier supplied times cannot gain lifetime.
    origin: Instant,
    /// Connect/handshake deadline from this transaction's original origin.
    handshake_deadline: Instant,
    /// Full-job deadline and candidate expiry, never reset by progress.
    expiry: Instant,
    /// Every replacement shares this actual original invocation's stop latch.
    stop_scope: Arc<StopScope>,
    /// Cancellation and commit serialize with the concrete nonblocking UDP send.
    /// General state effects remain non-I/O; no blocking operation or await is allowed.
    phase: Mutex<Phase>,
    /// Wake registered readiness waiters when cancellation closes admission.
    cancelled: Arc<tokio::sync::Notify>,
}

/// Actual scope and transaction guards retained in the sole permitted lock order.
struct ControlGuard<'control, 'scope> {
    /// This reference, not a copied phase, identifies the guarded transaction.
    control: &'control Control,
    /// Only this actual transaction's phase can be checked or changed here.
    phase: MutexGuard<'control, Phase>,
    /// Acquire this first and drop it after the local phase guard.
    scope: ScopeGuard<'scope>,
}

impl ControlGuard<'_, '_> {
    /// Check this actual owner's scope, phase and immutable deadline under both gates.
    /// # Errors
    /// Refuse stop, local cancellation/commit, pre-origin time or expired work.
    fn remaining(&self, phase: IoPhase, now: Instant) -> Result<Duration, Failure> {
        self.scope.ensure_open()?;
        match *self.phase {
            Phase::Cancelled => return Err(Failure::Cancelled),
            Phase::Committed => return Err(Failure::Control),
            Phase::Open | Phase::Published => {}
        }
        if now < self.control.origin {
            return Err(Failure::Deadline);
        }
        let deadline = match phase {
            IoPhase::Handshake => self.control.handshake_deadline,
            IoPhase::Exchange => self.control.expiry,
        };
        deadline
            .checked_duration_since(now)
            .filter(|value| !value.is_zero())
            .ok_or(Failure::Deadline)
    }

    /// Require current published authority from this same guarded transaction.
    /// # Errors
    /// Preserve stop/time/phase refusal before any effect is invoked.
    fn published(&self, now: Instant) -> Result<(), Failure> {
        self.remaining(IoPhase::Exchange, now)?;
        if matches!(*self.phase, Phase::Published) {
            Ok(())
        } else {
            Err(Failure::Control)
        }
    }

    /// Publish KE once under the actual original scope and phase guards.
    /// # Errors
    /// Refuse closed or expired authority without changing the local phase.
    fn publish_ke(&mut self, now: Instant) -> Result<(), Failure> {
        self.remaining(IoPhase::Exchange, now)?;
        if !matches!(*self.phase, Phase::Open) {
            return Err(Failure::Control);
        }
        *self.phase = Phase::Published;
        Ok(())
    }

    /// Preserve the one historical commit when it wins before invocation stop.
    /// # Errors
    /// Refuse unavailable published authority without changing its phase.
    fn commit(&mut self, now: Instant) -> Result<(), Failure> {
        self.published(now)?;
        *self.phase = Phase::Committed;
        Ok(())
    }
}

/// Hold the actual scope/phase before nonblocking source-policy custody.
/// Declaration order releases Control first, including check refusal and unwind.
#[cfg(any(windows, test))]
struct ConfigurationGuard<'a> {
    /// Actual cancellation and phase owner, released before policy notifications.
    control: ControlGuard<'a, 'a>,
    /// Present only for source-issued modes that require a current definition.
    policy: Option<crate::source_policy::KePreviewPolicyGuard<'a>>,
}

#[cfg(any(windows, test))]
impl ConfigurationGuard<'_> {
    /// Observe the unchanged original deadline under every applicable owner.
    /// # Errors
    /// Preserve stopped, consumed or expired Control refusal.
    fn remaining(&self, phase: IoPhase, now: Instant) -> Result<Duration, Failure> {
        self.control.remaining(phase, now)
    }

    /// Publish the same candidate while its original source definition is current.
    /// # Errors
    /// Preserve actual phase, stop and original expiry refusal without publication.
    fn publish_ke(&mut self, now: Instant) -> Result<(), Failure> {
        self.control.publish_ke(now)
    }

    /// Seal one historical result without a callback or allocation under custody.
    /// # Errors
    /// Preserve original published/stop/deadline refusal without a completion.
    fn commit(&mut self, now: Instant) -> Result<(), Failure> {
        self.control.commit(now)
    }
}

#[cfg(any(windows, test))]
impl Config {
    /// Acquire scope, phase, then the source policy and check the actual selected endpoint.
    /// No provider, entropy, callback, allocation, retirement or join occurs here.
    /// # Errors
    /// Preserve source/revision/endpoint refusal and reject detached connection inputs.
    fn guard<'a>(
        &'a self,
        control: &'a Control,
        endpoint: Option<SocketAddr>,
    ) -> Result<ConfigurationGuard<'a>, Failure> {
        self.bind_guard(control.guard()?, endpoint)
    }

    /// Use the invocation's already-held original scope without recursive locking.
    /// # Errors
    /// Preserve original scope/phase and source-currentness refusal before producer work.
    fn guard_in<'a>(
        &'a self,
        control: &'a Control,
        scope: ScopeGuard<'a>,
        endpoint: Option<SocketAddr>,
    ) -> Result<ConfigurationGuard<'a>, Failure> {
        self.bind_guard(control.guard_in(scope)?, endpoint)
    }

    /// Couple an actual Control guard to the context owned by this closed mode.
    /// # Errors
    /// Refuse detached connection fields or current source/endpoint admission.
    fn bind_guard<'a>(
        &'a self,
        control: ControlGuard<'a, 'a>,
        endpoint: Option<SocketAddr>,
    ) -> Result<ConfigurationGuard<'a>, Failure> {
        let policy: Option<crate::source_policy::KePreviewPolicyGuard<'a>> =
            if let Some(context) = self.mode.source_definition() {
                let target: &crate::KePreviewTarget = context.target();
                if self.peer != target.peer()
                    || self.identity != target.identity()
                    || &self.server_name != target.server_name()
                {
                    return Err(Failure::Control);
                }
                Some(context.lock().map_err(Failure::SourcePolicy)?)
            } else {
                None
            };
        let guard: ConfigurationGuard<'a> = ConfigurationGuard { control, policy };
        if let Some(policy) = &guard.policy {
            policy.check(endpoint).map_err(Failure::SourcePolicy)?;
        }
        Ok(guard)
    }

    /// Publish only the actual same-owner UDP selection after complete preparation.
    /// # Errors
    /// Preserve current source/configuration and original Control refusal atomically.
    fn publish_ke(
        &self,
        control: &Control,
        endpoint: SocketAddr,
    ) -> Result<Option<crate::source_policy::ProtectedEndpoint>, Failure> {
        let mut guard: ConfigurationGuard<'_> = self.guard(control, Some(endpoint))?;
        if let ExchangeMode::ManagedProtected(source) = &self.mode {
            let policy: &mut crate::source_policy::KePreviewPolicyGuard<'_> =
                guard.policy.as_mut().ok_or(Failure::Control)?;
            let staged: crate::source_policy::PreparedProtectedEndpoint<'_> = source
                .prepare_endpoint(policy, endpoint)
                .map_err(Failure::SourcePolicy)?;
            guard.control.publish_ke(Instant::now())?;
            return Ok(Some(staged.publish()));
        }
        guard.publish_ke(Instant::now())?;
        Ok(None)
    }
}

/// Both original Control gates are released before the source lock publishes wakes.
struct DatagramAuthorityGuard<'a> {
    /// Original `StopScope` followed by its invocation `Control`.
    control: ControlGuard<'a, 'a>,
    /// Present only with an actual issued endpoint lease, released last.
    policy: Option<crate::source_policy::ProtectedEndpointGuard<'a>>,
}

impl<'a> DatagramAuthorityGuard<'a> {
    /// Obtain currentness custody in order without waiting for policy.
    /// # Errors
    /// Preserve original Control or policy custody refusal.
    fn lock(
        control: &'a Control,
        endpoint: Option<&'a crate::source_policy::ProtectedEndpoint>,
    ) -> Result<Self, Failure> {
        let control: ControlGuard<'_, '_> = control.published_guard()?;
        let policy: Option<crate::source_policy::ProtectedEndpointGuard<'_>> = endpoint
            .map(crate::source_policy::ProtectedEndpoint::lock)
            .transpose()
            .map_err(Failure::SourcePolicy)?;
        Ok(Self { control, policy })
    }

    /// Bind the original association destination before using it at an effect.
    /// # Errors
    /// Preserve destination, configuration or retained restriction refusal.
    fn endpoint(&self, peer: SocketAddr) -> Result<(), Failure> {
        self.policy.as_ref().map_or(Ok(()), |policy| {
            policy.check(peer).map_err(Failure::SourcePolicy)
        })
    }

    /// Charge managed source and endpoint floors at the actual send boundary.
    /// # Errors
    /// Preserve a disabled, stale, premature or unavailable managed send refusal.
    fn reserve(
        &mut self,
        peer: SocketAddr,
        request: &Arc<()>,
        now: Instant,
    ) -> Result<Option<crate::source_policy::ProtectedSendStamp>, Failure> {
        self.policy
            .as_mut()
            .map(|policy| policy.reserve(peer, request, now))
            .transpose()
            .map_err(Failure::SourcePolicy)
    }

    /// Publish one verified protected control while the original Control and policy remain held.
    /// Consume the authorization issued by the exact retired receiving ledger.
    /// # Errors
    /// Preserve Stop, deadline, binding and stamp refusals before either policy record changes.
    fn commit_protected_control(
        &mut self,
        authorization: VerifiedRetiredControl<'_>,
    ) -> Result<(), Failure> {
        if !authorization.matches_control(self.control.control) {
            return Err(Failure::Control);
        }
        self.control.published(authorization.decided_at())?;
        self.policy
            .as_mut()
            .ok_or(Failure::Control)?
            .apply_control(authorization)
            .map_err(Failure::SourcePolicy)?;
        *self.control.phase = Phase::Committed;
        Ok(())
    }

    /// Check the same concrete request at Pending publication and final consumption.
    /// # Errors
    /// Refuse missing/mismatched authority, obsolete definition or policy epochs.
    fn receiving(
        &self,
        provenance: Option<(&crate::source_policy::ProtectedSendStamp, &Arc<()>)>,
    ) -> Result<(), Failure> {
        match (&self.policy, provenance) {
            (Some(policy), Some((stamp, request))) => policy
                .receiving(stamp, request)
                .map_err(Failure::SourcePolicy),
            (None, None) => Ok(()),
            _ => Err(Failure::Control),
        }
    }
}

impl Control {
    /// Establish both finite deadlines once; arithmetic failure creates no job.
    ///
    /// # Errors
    /// Refuses monotonic deadline arithmetic overflow; creates no usable control on
    /// failure.
    fn new(start: Instant) -> Result<Self, Failure> {
        Self::new_in(start, Arc::new(StopScope::new()))
    }

    /// Bind one immutable scope while establishing the original finite deadlines.
    /// # Errors
    /// Refuse monotonic deadline arithmetic overflow without a usable Control.
    fn new_in(start: Instant, stop_scope: Arc<StopScope>) -> Result<Self, Failure> {
        let cancelled = Arc::clone(&stop_scope.wake);
        Ok(Self {
            first_receipt_identity: Arc::new(()),
            datagram_work: None,
            origin: start,
            handshake_deadline: start.checked_add(HANDSHAKE_TIME).ok_or(Failure::Deadline)?,
            expiry: start.checked_add(EXCHANGE_TIME).ok_or(Failure::Deadline)?,
            stop_scope,
            phase: Mutex::new(Phase::Open),
            cancelled,
        })
    }

    /// Preserve the original phase maxima inside one immutable invocation window.
    /// The receiving owner supplies its internally captured origin and cutoff;
    /// ordinary effect guards still observe fresh time under the phase lock.
    /// # Errors
    /// Refuse deadline overflow and an empty or reversed window before effects.
    #[cfg(any(windows, test))]
    fn clamped(start: Instant, cutoff: Instant) -> Result<Self, Failure> {
        Self::clamped_in(start, cutoff, Arc::new(StopScope::new()))
    }

    /// Bind a replacement to the same actual scope without renewing its deadline.
    /// This constructor performs no admission and acquires no scope or phase lock.
    /// # Errors
    /// Refuse deadline overflow and an empty or reversed window before effects.
    #[cfg(any(windows, test))]
    fn clamped_in(
        start: Instant,
        cutoff: Instant,
        stop_scope: Arc<StopScope>,
    ) -> Result<Self, Failure> {
        if start >= cutoff {
            return Err(Failure::Deadline);
        }
        let mut control = Self::new_in(start, stop_scope)?;
        control.handshake_deadline = control.handshake_deadline.min(cutoff);
        control.expiry = control.expiry.min(cutoff);
        Ok(control)
    }

    /// Retain both actual guards in scope-then-phase order.
    /// # Errors
    /// Refuse either unavailable lock without granting current authority.
    fn guard(&self) -> Result<ControlGuard<'_, '_>, Failure> {
        self.guard_in(self.stop_scope.lock()?)
    }

    /// Consume an already-held same-scope gate without recursively locking it.
    /// # Errors
    /// Refuse a foreign scope or an unavailable actual transaction lock.
    fn guard_in<'scope>(
        &self,
        scope: ScopeGuard<'scope>,
    ) -> Result<ControlGuard<'_, 'scope>, Failure> {
        if !std::ptr::eq(scope.scope, self.stop_scope.as_ref()) {
            return Err(Failure::Control);
        }
        let phase = self.phase.lock().map_err(|_| Failure::Control)?;
        Ok(ControlGuard {
            control: self,
            scope,
            phase,
        })
    }

    /// Observe actual fresh time while holding both current-authority guards.
    /// # Errors
    /// Refuse stop, unavailable/closed local phase or an expired original deadline.
    fn remaining(&self, phase: IoPhase) -> Result<Duration, Failure> {
        self.guard()?.remaining(phase, Instant::now())
    }

    /// Retain actual scope and phase for the bounded concrete UDP send.
    /// No allocation, arbitrary callback, blocking operation or await may occur.
    /// # Errors
    /// Refuse unavailable, stopped, unpublished or expired authority.
    fn published_guard(&self) -> Result<ControlGuard<'_, '_>, Failure> {
        let guard = self.guard()?;
        guard.published(Instant::now())?;
        Ok(guard)
    }

    /// Run a bounded, non-I/O state commit under fresh published authority.
    /// Stage allocations and fallible preparation first. Explicit work charges,
    /// retirement and recovery may survive refusal; never refund them or renew
    /// authority. The effect must not re-enter either held guard.
    /// # Errors
    /// Preserve currentness or the effect's closed refusal without changing phase.
    fn with_published<T>(
        &self,
        effect: impl FnOnce(Instant) -> Result<T, Failure>,
    ) -> Result<T, Failure> {
        self.with_published_observing(Instant::now, effect)
    }

    /// Sample the trusted local clock after acquiring both actual owner guards.
    /// # Errors
    /// Preserve currentness refusal and the bounded non-reentrant effect's failure.
    fn with_published_observing<T>(
        &self,
        observe: impl FnOnce() -> Instant,
        effect: impl FnOnce(Instant) -> Result<T, Failure>,
    ) -> Result<T, Failure> {
        let guard = self.guard()?;
        let now = observe();
        guard.published(now)?;
        let result = effect(now);
        drop(guard);
        result
    }

    /// Testable time observation under the same mandatory actual guards.
    /// # Errors
    /// Refuse stopped/closed authority, pre-origin time or an expired deadline.
    #[cfg(test)]
    fn remaining_at(&self, phase: IoPhase, now: Instant) -> Result<Duration, Failure> {
        self.guard()?.remaining(phase, now)
    }

    /// Consume one genuine KE completion under both still-live actual guards.
    /// # Errors
    /// Refuse stop, expiry, duplicate publication or unavailable control.
    #[cfg(test)]
    fn publish_ke(&self) -> Result<(), Failure> {
        self.publish_ke_observing(Instant::now)
    }

    /// Sample publication time only within the held actual scope and phase.
    /// # Errors
    /// Preserve currentness/duplicate refusal without partial publication.
    #[cfg(test)]
    fn publish_ke_observing(&self, observe: impl FnOnce() -> Instant) -> Result<(), Failure> {
        let mut guard = self.guard()?;
        guard.publish_ke(observe())
    }

    /// Exercise the same publication rule at an independently selected test time.
    /// # Errors
    /// Refuse stopped, closed or expired authority without changing local phase.
    #[cfg(test)]
    fn publish_ke_at(&self, now: Instant) -> Result<(), Failure> {
        self.guard()?.publish_ke(now)
    }

    /// Commit after a fresh time observation under both actual authority guards.
    /// # Errors
    /// Refuse stopped, closed or expired authority without creating a commit.
    fn commit(&self) -> Result<(), Failure> {
        self.guard()?.commit(Instant::now())
    }

    /// Exercise the same commit rule at an independent boundary time.
    /// # Errors
    /// Refuse stopped, closed or expired authority without changing local phase.
    #[cfg(test)]
    fn commit_at(&self, now: Instant) -> Result<(), Failure> {
        self.guard()?.commit(now)
    }

    /// Close new publication, explicitly distinguishing a stop after commit.
    ///
    /// # Errors
    /// Refuses a poisoned control lock; an existing commit remains explicitly distinguished
    /// from cancellation.
    fn cancel(&self) -> Result<CancelOutcome, Failure> {
        let mut state = self.phase.lock().map_err(|_| Failure::Control)?;
        let outcome = match *state {
            Phase::Open | Phase::Published => {
                *state = Phase::Cancelled;
                CancelOutcome::Cancelled
            }
            Phase::Cancelled => CancelOutcome::AlreadyCancelled,
            Phase::Committed => CancelOutcome::AlreadyCommitted,
        };
        drop(state);
        if outcome != CancelOutcome::AlreadyCommitted {
            self.cancelled.notify_waiters();
        }
        Ok(outcome)
    }
}

/// Inert owned negotiation result; this type proves no successful TLS operation.
struct Reply {
    /// Opaque initial cookies, copied only within the count/aggregate limits.
    cookies: Vec<Zeroizing<Vec<u8>>>,
    /// Prospective admitted UDP endpoint; no UDP operation occurs here.
    udp: SocketAddr,
    /// Literal protocol/AEAD/direction metadata for the actual C2S exporter.
    c2s_context: NtsKeExporterContext,
    /// Separate direction metadata for the actual S2C exporter.
    s2c_context: NtsKeExporterContext,
}

/// Encode the actual sole offer into exactly sixteen caller-owned bytes.
///
/// # Errors
/// Propagates rejection of fixed limits or the actual request encoder; no unchecked offer
/// is returned.
fn encode_offer(storage: &mut [u8; 16]) -> Result<NtsKeNtpv4Offer<'_>, Failure> {
    let limits = NtsKeFrameLimits::new(16, 3).map_err(|_| Failure::Limits)?;
    encode_nts_ke_ntpv4_request(&[15], storage, limits).map_err(Failure::Offer)
}

/// Admit hints only against this invocation's existing identity-to-peer mapping.
///
/// # Errors
/// Refuses an unmapped server hint or explicit zero port; returns no alternative address or
/// DNS effect.
fn endpoint(
    config: &Config,
    server: Option<&str>,
    port: Option<u16>,
) -> Result<SocketAddr, Failure> {
    if let Some(server) = server {
        let admitted = match server.parse::<IpAddr>() {
            Ok(address) => address == config.peer.ip(),
            Err(_) => server.eq_ignore_ascii_case(&config.identity),
        };
        if !admitted {
            return Err(Failure::Endpoint);
        }
    }
    let port = port.unwrap_or(123);
    if port == 0 {
        return Err(Failure::Endpoint);
    }
    Ok(SocketAddr::new(config.peer.ip(), port))
}

/// Copy at most eight cookies with checked aggregate size and zeroizing owners.
///
/// # Errors
/// Refuses a ninth cookie, aggregate overflow/excess or allocation failure; partial copies
/// are wiped on ordinary drop.
fn copy_cookies<'a>(
    cookies: impl Iterator<Item = &'a [u8]>,
) -> Result<Vec<Zeroizing<Vec<u8>>>, Failure> {
    let mut owned = Vec::new();
    owned
        .try_reserve_exact(COOKIE_LIMIT)
        .map_err(|_| Failure::Allocation)?;
    let mut total = 0_usize;
    for cookie in cookies {
        if owned.len() == COOKIE_LIMIT {
            return Err(Failure::Cookies);
        }
        total = total.checked_add(cookie.len()).ok_or(Failure::Cookies)?;
        if total > COOKIE_BYTES_LIMIT {
            return Err(Failure::Cookies);
        }
        let mut copy = Zeroizing::new(Vec::new());
        copy.try_reserve_exact(cookie.len())
            .map_err(|_| Failure::Allocation)?;
        copy.extend_from_slice(cookie);
        owned.push(copy);
    }
    Ok(owned)
}

/// Read one real response without conflating EOM with EOF or reading past EOM.
///
/// The reader owns live time/cancellation admission. Every I/O error, including
/// interruption, refuses without retry. This helper never creates TLS authority.
///
/// # Errors
/// Refuses allocation, limits, read/interruption, incomplete or invalid framing,
/// negotiation, cookie-copy and endpoint failures.
fn read_response(
    reader: &mut impl Read,
    config: &Config,
    offer: &NtsKeNtpv4Offer<'_>,
) -> Result<Reply, Failure> {
    let mut storage = Zeroizing::new(Vec::new());
    storage
        .try_reserve_exact(RESPONSE_LIMIT)
        .map_err(|_| Failure::Allocation)?;
    storage.resize(RESPONSE_LIMIT, 0);
    let mut scratch = Zeroizing::new([0_u8; SCRATCH_LIMIT]);
    let limits =
        NtsKeFrameLimits::new(RESPONSE_LIMIT, RECORD_LIMIT).map_err(|_| Failure::Limits)?;
    let mut accumulator = NtsKeAccumulator::new(&mut storage, limits).map_err(Failure::Frame)?;
    loop {
        let count = reader
            .read(scratch.as_mut())
            .map_err(|error| transport_fault::response_error(&error))?;
        if count == 0 {
            break;
        }
        let bytes = scratch.get(..count).ok_or(Failure::Control)?;
        if accumulator.push(bytes).map_err(Failure::Frame)? == NtsKeProgress::EndOfMessage {
            break;
        }
    }
    let message = accumulator.finish().map_err(Failure::Frame)?;
    let negotiation = negotiate_nts_ke_ntpv4(offer, message).map_err(Failure::Negotiation)?;
    Ok(Reply {
        cookies: copy_cookies(negotiation.cookies())?,
        udp: endpoint(config, negotiation.server_hint(), negotiation.port_hint())?,
        c2s_context: negotiation.exporter_context(NtsKeExporterDirection::ClientToServer),
        s2c_context: negotiation.exporter_context(NtsKeExporterDirection::ServerToClient),
    })
}

/// Payload-free historical completion after all secret/native owners drop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Summary {
    /// Checked pool entries immediately before the selected operation.
    cookies: usize,
    /// Actual aggregate opaque cookie payload bytes.
    cookie_bytes: usize,
    /// Complete request length; transmission is reported separately and explicitly.
    packet_bytes: usize,
    /// A complete local datagram send returned; this asserts no peer delivery.
    transmitted: bool,
}

/// One actual terminal mode; protected provenance moves with its success.
/// Neither variant grants numerical eligibility, source agreement or clock authority.
#[cfg(any(windows, test))]
enum InvocationCompletion {
    /// Preparation completed without a protected send or authenticated observation.
    Preview(Summary),
    /// Same never-send result with its actual source definition retained after commit.
    ManagedPreview(Box<handoff::ManagedPreviewHistory>),
    /// One genuine committed exchange, detached from all live native/secret owners.
    Protected(Box<datagram::ProtectedHistory>),
    /// The actual managed protected result and its original committed source revision.
    ManagedProtected(Box<datagram::ManagedProtectedHistory>),
    /// A committed, control-only historical effect from the private scripted worker.
    #[cfg(test)]
    ManagedProtectedControl(association::ledger::AppliedProtectedControl),
    /// Closed second operation, with the original correlation-only NAK observation.
    ManagedSuccessor(Box<ManagedSuccessorTerminal>),
}

/// Terminal historical effect issued after the second adapter retires.
enum ManagedSuccessorEffect {
    /// Genuine authenticated ordinary commit.
    Ordinary(Box<datagram::ManagedProtectedHistory>),
    /// The exact restriction committed by the source and endpoint policy owner.
    Restriction(association::ledger::AppliedProtectedControl),
}

/// Terminal operation result and the independently observed recovery indication.
struct ManagedSuccessorTerminal {
    /// Preserve original operation and cleanup failures beside the first receipt.
    result: Result<ManagedSuccessorEffect, Failed>,
    /// Earliest admitted NAK within the original successor receive window.
    first_nak_at: Option<Instant>,
    /// Unspent checkout, frame and byte budgets from this actual association.
    remaining_association_work: Option<(usize, usize, usize)>,
}

#[cfg(test)]
impl InvocationCompletion {
    /// Borrow only the existing bounded CLI counts without exposing or copying history.
    fn summary(&self) -> Option<&Summary> {
        match self {
            Self::Preview(summary) => Some(summary),
            Self::ManagedPreview(history) => Some(history.summary()),
            Self::Protected(history) => Some(history.summary()),
            Self::ManagedProtected(history) => Some(history.summary()),
            Self::ManagedProtectedControl(_) | Self::ManagedSuccessor(_) => None,
        }
    }
}

/// Primary operation failure and a separately retained cancellation failure.
#[cfg(any(windows, test))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Failed {
    /// Original operation failure, never replaced by cleanup.
    primary: Failure,
    /// Secondary failure flushing TLS closure after a primary operation failed.
    cleanup: Option<Failure>,
    /// A separate failure closing publication admission after the operation.
    admission_cleanup: Option<Failure>,
}

#[cfg(any(windows, test))]
impl From<Failure> for Failed {
    /// Retain one original failure before any additional cleanup was attempted.
    fn from(primary: Failure) -> Self {
        Self {
            primary,
            cleanup: None,
            admission_cleanup: None,
        }
    }
}

/// Drive the actual one-shot invocation with its original Control and selected producer.
/// All association/invocation custody retires before the outer provider/Config owner.
/// # Errors
/// Preserve original operation and secondary cleanup facts without a retry or new Control.
#[cfg(any(windows, test))]
fn drive_invocation_with<'scope, P: recovery::Producer>(
    config: &'scope Config,
    provider: &'scope ProviderScope<'scope>,
    control: Control,
    producer: &mut P,
) -> Result<InvocationCompletion, Failed> {
    let mut invocation =
        recovery::Invocation::one_shot(config, provider, control).map_err(Failed::from)?;
    drive_prepared_invocation(&mut invocation, producer)
}

/// Run the selected successor through the actual worker's first-history checkpoint.
/// # Errors
/// Preserve original admission, operation and retirement refusal from the shared drive.
#[cfg(test)]
fn drive_successor_fixture<'scope, P: recovery::Producer>(
    config: &'scope Config,
    provider: &'scope ProviderScope<'scope>,
    control: Control,
    producer: &mut P,
    writer: job::FirstReceiptWriter<'scope>,
) -> job::Operation {
    drive_with_first_receipt(config, provider, control, producer, writer)
}

/// Drive the one-KE route with the exact original worker receipt destination.
/// # Errors
/// Preserve original preparation and adapter refusals.
fn drive_with_first_receipt<'scope, P: recovery::Producer>(
    config: &'scope Config,
    provider: &'scope ProviderScope<'scope>,
    control: Control,
    producer: &mut P,
    writer: job::FirstReceiptWriter<'scope>,
) -> job::Operation {
    let mut invocation: recovery::Invocation<'scope> =
        recovery::Invocation::one_shot(config, provider, control).map_err(Failed::from)?;
    invocation.retain_first_with(writer).map_err(Failed::from)?;
    drive_prepared_invocation(&mut invocation, producer)
}

/// Drive one checked invocation without substituting a renewed Control or result.
/// # Errors
/// Preserve original operation and secondary cleanup facts.
#[cfg(any(windows, test))]
fn drive_prepared_invocation<P: recovery::Producer>(
    invocation: &mut recovery::Invocation<'_>,
    producer: &mut P,
) -> job::Operation {
    match recovery::step_with(invocation, producer) {
        Ok(recovery::Step::Complete(summary)) => Ok(summary),
        Err(failure) => Err(failure),
        Ok(recovery::Step::Closed(end)) => Err(match invocation.last_failure() {
            Some(failure) => *failure,
            None => Failed::from(match end {
                recovery::End::Window | recovery::End::Arithmetic => Failure::Deadline,
                recovery::End::Context(_, cause) => cause,
                recovery::End::Stopped => Failure::Cancelled,
                recovery::End::Control
                | recovery::End::Completed
                | recovery::End::Failure
                | recovery::End::Transactions
                | recovery::End::Contexts
                | recovery::End::Refusals
                | recovery::End::Observations
                | recovery::End::Jitter(_) => Failure::Control,
            }),
        }),
        Ok(recovery::Step::Deferred(_)) => Err(match invocation.last_failure() {
            Some(failure) => *failure,
            None => Failed::from(Failure::Control),
        }),
    }
}

/// Wire the observed Windows invocation to its original one-shot stop scope.
/// # Errors
/// Preserve caller-context, preparation and registration refusal before submission.
#[cfg(windows)]
fn run_with_power_observer(config: Config) -> Result<ObservedInvocation, ObserverStartError> {
    run_observing(
        config,
        &PROCESS_WORKER,
        subscribe_power_observer,
        job::SubmissionReservation::try_submit,
    )
}

/// One checked lease and reservation for the same prepared invocation.
#[cfg(any(windows, test))]
struct ManagedObservedJob<'target> {
    /// Original prepared Control and actual worker reservation.
    reservation: job::SubmissionReservation,
    /// Keeps the power delivery target bound until collection finishes.
    active: power_target::ActivePower<'target>,
    /// The exact Stop handle for an unconfirmed collection.
    stop: StopHandle,
}

/// Use the observer's epoch across original Control preparation and reservation.
/// # Errors
/// Preserve caller, deadline, slot, source and power binding refusals before spawn.
#[cfg(any(windows, test))]
fn prepare_managed_observed<'target>(
    config: Config,
    cutoff: Option<Instant>,
    worker: &job::ManagedWorker,
    target: &'target power_target::PowerTarget,
) -> Result<ManagedObservedJob<'target>, ObserverStartError> {
    let power_refusal =
        |failure: Failed| ObserverStartError::Preparation(RunFailed::Preparation(failure));
    let attempt: power_target::PreparedPower<'_> = target
        .prepare()
        .map_err(Failed::from)
        .map_err(power_refusal)?;
    let prepared: job::PreparedInvocation =
        prepare_synchronous(config, cutoff).map_err(ObserverStartError::Preparation)?;
    let reservation: job::SubmissionReservation =
        worker
            .try_reserve(prepared)
            .map_err(|error: job::SubmitError| {
                ObserverStartError::Preparation(RunFailed::Submission(error))
            })?;
    let (active, stop): (power_target::ActivePower<'_>, StopHandle) =
        attempt.activate(&reservation).map_err(power_refusal)?;
    Ok(ManagedObservedJob {
        reservation,
        active,
        stop,
    })
}

/// One exact current-process presentation query outcome.
#[cfg(windows)]
type ProcessLeapPresentationRead = Result<
    windows_platform_native::LeapPresentation,
    windows_platform_native::LeapPresentationError,
>;

/// Enclose the original submission and normal joined take in process-local reads.
/// Callers retain the original power owner across this operation. An alternate
/// backend can omit the reads. The observations grant no mapping or continuity.
/// # Errors
/// Preserve outer submission, collection and abnormal-join refusals without a closing read.
/// A normally joined successor retains both reads alongside its own terminal outcome.
#[cfg(any(windows, test))]
fn receive_managed_submission(
    submit: impl FnOnce() -> Result<job::SubmittedJob, job::SubmitError>,
    #[cfg(windows)] read_presentation: &mut Option<&mut dyn FnMut() -> ProcessLeapPresentationRead>,
) -> Result<datagram::ReceivedInvocation, RunFailed> {
    #[cfg(windows)]
    let finish_presentation = read_presentation.as_mut().map(|read| {
        let before: ProcessLeapPresentationRead = read();
        move || [before, read()]
    });
    let joined: job::ReceivingOutcome = submit()
        .map_err(RunFailed::Submission)?
        .receive()
        .map_err(RunFailed::Collection)?;
    let received: datagram::ReceivedInvocation = datagram::receive_joined(joined, None)?;
    #[cfg(windows)]
    let received: datagram::ReceivedInvocation = match (received, finish_presentation) {
        (datagram::ReceivedInvocation::ManagedSuccessor(mut result), Some(mut finish)) => {
            let reads: [ProcessLeapPresentationRead; 2] = finish();
            result.retain_process_leap_presentation(reads);
            datagram::ReceivedInvocation::ManagedSuccessor(result)
        }
        (other, _) => other,
    };
    Ok(received)
}

/// Select the explicit bounded successor before any Native invocation effects.
/// # Errors
/// Preserve the same checked preparation and registration refusals as the probe.
#[cfg(windows)]
fn run_managed_successor_with_power_observer(
    config: Config,
    owner_cutoff: Option<Instant>,
) -> Result<ObservedInvocation, ObserverStartError> {
    let worker: job::ManagedWorker = job::ManagedWorker::new(&PROCESS_WORKER);
    let target: Arc<power_target::PowerTarget> = Arc::new(power_target::PowerTarget::new());
    let ManagedObservedJob {
        reservation,
        active,
        stop,
    } = prepare_managed_observed(config, owner_cutoff, &worker, &target)?;
    let close = subscribe_managed_power(Arc::clone(&target))
        .map_err(|()| ObserverStartError::Registration)?;
    let result: Result<datagram::ReceivedInvocation, RunFailed> = receive_managed_submission(
        || {
            let submitted: Result<job::SubmittedJob, job::SubmitError> =
                reservation.try_submit_successor();
            // This facade owns one job. The slot retains provider custody until actual join.
            drop(worker);
            submitted
        },
        &mut Some(&mut windows_platform_native::read_leap_presentation),
    );
    Ok(finish_observing(result, &stop, move || {
        drop(active);
        close()
    }))
}

/// Register one persistent native callback for the target's owning scope.
/// # Errors
/// Refuse unavailable registration before any worker runs.
#[cfg(windows)]
fn subscribe_managed_power(
    target: Arc<power_target::PowerTarget>,
) -> Result<impl FnOnce() -> Result<(), NonZeroU32>, ()> {
    let watch: ResumeWatch =
        ResumeWatch::subscribe_persistent(move || target.deliver().map_err(|_| ()))
            .map_err(|_| ())?;
    Ok(move || watch.close().map_err(ResumeUnregistrationError::code))
}

/// Subscribe the actual Windows observer to this invocation's original Stop scope.
/// The returned consuming close reports the native unregistration result.
/// # Errors
/// Refuse an unavailable subscription before any worker is submitted.
#[cfg(windows)]
fn subscribe_power_observer(
    stop: StopHandle,
) -> Result<impl FnOnce() -> Result<(), NonZeroU32>, ()> {
    let watch: ResumeWatch =
        ResumeWatch::subscribe(move || stop.request_stop().map(|_| ()).map_err(|_| ()))
            .map_err(|_| ())?;
    Ok(move || watch.close().map_err(ResumeUnregistrationError::code))
}

/// Keep the joined or unconfirmed invocation separately from observer failures.
#[cfg(any(windows, test))]
struct ObservedInvocation {
    /// Existing worker result. Cancellation and observer close never alter it.
    result: Result<datagram::ReceivedInvocation, RunFailed>,
    /// Observer results, independent of the worker's retained result.
    cleanup: PowerObserverCleanup,
}

/// Closed observer cleanup facts, retaining the exact native status on refusal.
#[cfg(any(windows, test))]
#[derive(Clone, Copy, Default)]
struct PowerObserverCleanup {
    /// The actual stop scope could not be latched after collection refusal.
    stop_failed: bool,
    /// Exact nonzero status from consuming close after disarm.
    unregistration_error: Option<NonZeroU32>,
}

/// Failures before an observed invocation can submit the actual worker.
#[cfg(any(windows, test))]
enum ObserverStartError {
    /// Caller context, original checked preparation or slot reservation refused.
    Preparation(RunFailed),
    /// The power observer did not register.
    Registration,
}

/// Register only after reserving the actual worker slot and release it on refusal.
/// The observer and worker share the original Stop scope. Collection refusal
/// requests Stop before disarm while the slot retains the actual worker.
///
/// # Errors
/// Preserve caller, preparation and slot refusal before registration, or an
/// observer registration refusal before submission.
#[cfg(any(windows, test))]
fn run_observing<Close: FnOnce() -> Result<(), NonZeroU32>>(
    config: Config,
    slot: &'static job::WorkerSlot,
    subscribe: impl FnOnce(StopHandle) -> Result<Close, ()>,
    submit: impl FnOnce(job::SubmissionReservation) -> Result<(), job::SubmitError>,
) -> Result<ObservedInvocation, ObserverStartError> {
    run_observing_with_cutoff(config, None, slot, subscribe, submit)
}

/// Keep any caller cutoff in the original Control through observer registration and join.
/// # Errors
/// Preserve caller, preparation, reservation and registration refusals without renewal.
#[cfg(any(windows, test))]
fn run_observing_with_cutoff<Close: FnOnce() -> Result<(), NonZeroU32>>(
    config: Config,
    owner_cutoff: Option<Instant>,
    slot: &'static job::WorkerSlot,
    subscribe: impl FnOnce(StopHandle) -> Result<Close, ()>,
    submit: impl FnOnce(job::SubmissionReservation) -> Result<(), job::SubmitError>,
) -> Result<ObservedInvocation, ObserverStartError> {
    let prepared: job::PreparedInvocation =
        prepare_synchronous(config, owner_cutoff).map_err(ObserverStartError::Preparation)?;
    let reservation: job::SubmissionReservation =
        slot.try_reserve(prepared)
            .map_err(|error: job::SubmitError| {
                ObserverStartError::Preparation(RunFailed::Submission(error))
            })?;
    run_reserved_observing(reservation, subscribe, submit)
}

/// Recheck the original startup authority at the reserved registration edge.
/// # Errors
/// Release the unspent reservation on currentness or registration refusal.
#[cfg(any(windows, test))]
fn run_reserved_observing<Close: FnOnce() -> Result<(), NonZeroU32>>(
    reservation: job::SubmissionReservation,
    subscribe: impl FnOnce(StopHandle) -> Result<Close, ()>,
    submit: impl FnOnce(job::SubmissionReservation) -> Result<(), job::SubmitError>,
) -> Result<ObservedInvocation, ObserverStartError> {
    let (slot, stop): (&'static job::WorkerSlot, StopHandle) =
        reservation.observer_entry().map_err(|failure: Failed| {
            ObserverStartError::Preparation(RunFailed::Preparation(failure))
        })?;
    let close: Close = subscribe(stop.clone()).map_err(|()| ObserverStartError::Registration)?;
    let result: Result<datagram::ReceivedInvocation, RunFailed> = submit(reservation)
        .map_err(RunFailed::Submission)
        .and_then(|()| receive_submitted(slot, None));
    Ok(finish_observing(result, &stop, close))
}

/// Keep actual collection refusal and independent observer teardown together.
#[cfg(any(windows, test))]
fn finish_observing(
    result: Result<datagram::ReceivedInvocation, RunFailed>,
    stop: &StopHandle,
    close: impl FnOnce() -> Result<(), NonZeroU32>,
) -> ObservedInvocation {
    let stop_failed: bool =
        matches!(&result, Err(RunFailed::Collection(_))) && stop.request_stop().is_err();
    let unregistration_error: Option<NonZeroU32> = close().err();
    ObservedInvocation {
        result,
        cleanup: PowerObserverCleanup {
            stop_failed,
            unregistration_error,
        },
    }
}

/// Own synchronous preparation, real submission, collection and result projection.
/// The trusted adapter submits to this actual slot; no collector or result is injected.
/// Entered Tokio callers refuse before Control preparation or adapter invocation.
/// Explicit collection can wait indefinitely; a collection refusal leaves completion
/// unconfirmed and retains the actual handle in permanent custody.
/// The exact optional model follows only normal joined protected history; the actual
/// Windows caller supplies no declaration and receives numerical unavailability.
/// # Errors
/// Keep pre-submit caller refusal distinct from submission, unconfirmed collection,
/// normally retired operation failure and actual joined abnormal termination.
#[cfg(test)]
fn run_synchronously_with(
    config: Config,
    slot: &'static job::WorkerSlot,
    profile: Option<crate::ExperimentalProfile>,
    submit: impl FnOnce(
        &'static job::WorkerSlot,
        job::PreparedInvocation,
    ) -> Result<(), job::SubmitError>,
) -> Result<datagram::ReceivedCompletion, RunFailed> {
    receive_probe(run_joined_with(config, slot, profile, submit))
}

/// Project a normal probe from the joined invocation under its existing contract.
/// # Errors
/// Preserve original failure or refuse a fixture-only protected control.
#[cfg(any(windows, test))]
fn receive_probe(
    received: Result<datagram::ReceivedInvocation, RunFailed>,
) -> Result<datagram::ReceivedCompletion, RunFailed> {
    match received? {
        datagram::ReceivedInvocation::Probe(completion) => Ok(completion),
        datagram::ReceivedInvocation::ManagedSuccessor(_) => {
            Err(RunFailed::Operation(Failure::Control.into()))
        }
        #[cfg(test)]
        datagram::ReceivedInvocation::ProtectedControl(_) => {
            Err(RunFailed::Operation(Failure::Control.into()))
        }
    }
}

/// Share actual preparation, collect, and sealed take with the private control fixture.
/// # Errors
/// Preserve caller-context, submission, collection, operation or abnormal retirement failure.
#[cfg(test)]
fn run_joined_with(
    config: Config,
    slot: &'static job::WorkerSlot,
    profile: Option<crate::ExperimentalProfile>,
    submit: impl FnOnce(
        &'static job::WorkerSlot,
        job::PreparedInvocation,
    ) -> Result<(), job::SubmitError>,
) -> Result<datagram::ReceivedInvocation, RunFailed> {
    let prepared: job::PreparedInvocation = prepare_synchronous(config, None)?;
    collect_prepared(prepared, slot, profile, submit)
}

/// Prepare one checked invocation and its own stop endpoint outside Tokio.
/// # Errors
/// Refuse an entered runtime or the original preparation failure.
#[cfg(any(windows, test))]
fn prepare_synchronous(
    config: Config,
    owner_cutoff: Option<Instant>,
) -> Result<job::PreparedInvocation, RunFailed> {
    if tokio::runtime::Handle::try_current().is_ok() {
        return Err(RunFailed::CallerContext);
    }
    job::PreparedInvocation::prepare_with_cutoff(config, owner_cutoff)
        .map_err(RunFailed::Preparation)
}

/// Submit and consume only after the existing worker confirms actual join.
/// # Errors
/// Preserve submission, collection, operation or abnormal retirement failure.
#[cfg(test)]
fn collect_prepared(
    prepared: job::PreparedInvocation,
    slot: &'static job::WorkerSlot,
    profile: Option<crate::ExperimentalProfile>,
    submit: impl FnOnce(
        &'static job::WorkerSlot,
        job::PreparedInvocation,
    ) -> Result<(), job::SubmitError>,
) -> Result<datagram::ReceivedInvocation, RunFailed> {
    submit(slot, prepared).map_err(RunFailed::Submission)?;
    receive_submitted(slot, profile)
}

/// Receive only after the actual worker submission adapter succeeds.
///
/// # Errors
/// Preserve collection and joined operation or retirement failures.
#[cfg(any(windows, test))]
fn receive_submitted(
    slot: &'static job::WorkerSlot,
    profile: Option<crate::ExperimentalProfile>,
) -> Result<datagram::ReceivedInvocation, RunFailed> {
    slot.collect().map_err(RunFailed::Collection)?;
    let joined: job::ReceivingOutcome = slot.take_for_receiving().map_err(RunFailed::Collection)?;
    datagram::receive_joined(joined, profile)
}

/// Closed outer failure projection keeps submission, operation and joined termination distinct.
#[cfg(any(windows, test))]
enum RunFailed {
    /// Entered Tokio context refused before preparation or any actual submission.
    CallerContext,
    /// No submission occurred because original checked preparation failed.
    Preparation(Failed),
    /// No worker was submitted because power observation could not be established.
    PowerRegistration,
    /// No completed job is inferred from submission refusal.
    Submission(job::SubmitError),
    /// Completion/retirement is unconfirmed; permanent actual job custody remains.
    Collection(job::CollectError),
    /// Normally retired operation failure retains all original secondary facts.
    Operation(Failed),
    /// Actual joined abnormal termination retains any recorded historical facts.
    Abnormal(job::JoinedOutcome),
}

/// Report bounded closed worker categories without replacing original failure facts.
#[cfg(windows)]
fn report_run_failure(failure: &RunFailed) -> std::process::ExitCode {
    let _diagnostic_result = output::write_run_failure(failure, &mut io::stderr().lock());
    std::process::ExitCode::FAILURE
}

/// The same one-shot owner captures initial counts before unsendable preview work.
/// All inputs are the actual checked association and its one bounded entropy fill.
/// # Errors
/// Preserve population, preview/preparation and final currentness refusal; no
/// protected-completion fact is constructed on this branch.
#[cfg(any(windows, test))]
fn preview_once(
    association: &mut AssociationOwner<'_>,
    fill: impl FnOnce(&mut [u8; 48]) -> Result<(), Failure>,
) -> Result<Summary, Failure> {
    let population = association.cookie_population()?;
    let mut summary = association.prepare_preview(fill)?.finish()?;
    summary.cookies = population.0;
    summary.cookie_bytes = population.1;
    Ok(summary)
}

/// Preserve the original definition and population through the actual unsendable seal.
/// # Errors
/// Preserve charged work, source/endpoint/stop refusal and normal packet destruction.
#[cfg(any(windows, test))]
fn managed_preview_once(
    association: &mut AssociationOwner<'_>,
    fill: impl FnOnce(&mut [u8; 48]) -> Result<(), Failure>,
) -> Result<Box<handoff::ManagedPreviewHistory>, Failure> {
    association
        .prepare_preview(fill)?
        .finish_managed()
        .map(Box::new)
}

/// Receive only a retired opaque operation before committing/projecting its facts.
/// The private producer is called once before either final guard; no callback or
/// await occurs between association validation and the final Control commit.
/// # Errors
/// Preserve actual producer, population, wrong/stale context or commit refusal.
/// Every rejected operation creates no fact or success summary and refunds no work.
#[cfg(test)]
fn protected_once<'association, 'control>(
    association: &mut AssociationOwner<'association>,
    exchange: impl FnOnce(
        &mut AssociationOwner<'association>,
    ) -> Result<datagram::ProtectedCompletion<'control>, Failure>,
) -> Result<Summary, Failure> {
    let population = association.cookie_population()?;
    protected_receiving(association, exchange, |committed| {
        let mut summary = committed.into_summary();
        summary.cookies = population.0;
        summary.cookie_bytes = population.1;
        Ok(summary)
    })
}

/// Share actual commit/count handling with the invocation's exclusive receiver.
/// The receiver consumes the committed fact before any terminal projection.
/// # Errors
/// Preserve producer, association, commit and receiver rejection without refunds.
#[cfg(any(windows, test))]
fn protected_receiving<'association, 'control, T>(
    association: &mut AssociationOwner<'association>,
    exchange: impl FnOnce(
        &mut AssociationOwner<'association>,
    ) -> Result<datagram::ProtectedCompletion<'control>, Failure>,
    receive: impl FnOnce(datagram::CommittedProtected<'control>) -> Result<T, Failure>,
) -> Result<T, Failure> {
    let completion = exchange(association)?;
    let committed = completion.commit(association)?;
    receive(committed)
}

/// Project only closed, bounded failure labels; payload-bearing errors stay out.
#[cfg(any(windows, test))]
fn failure_labels(failure: Failure) -> (&'static str, &'static str) {
    match failure {
        Failure::Arguments => ("admission", "arguments"),
        Failure::Allocation => ("resources", "allocation"),
        Failure::Deadline => ("currentness", "deadline"),
        Failure::Cancelled => ("currentness", "cancelled"),
        Failure::Control => ("currentness", "control"),
        Failure::ProcessUsed => ("admission", "process-used"),
        Failure::PreviewSpent => ("resources", "preview-spent"),
        Failure::SourcePolicy(_) => ("source-policy", "refused"),
        Failure::Ledger(_) => ("protected-request", "refused"),
        Failure::Datagram(_) => ("protected-datagram", "refused"),
        Failure::Offer(_) | Failure::Limits => ("request", "parameters"),
        Failure::Response(kind) => ("response", io_label(kind)),
        Failure::Tls(failure) => failure.labels(),
        Failure::Frame(_) => ("response", "framing"),
        Failure::Negotiation(_) => ("response", "negotiation"),
        Failure::Cookies => ("response", "cookie-budget"),
        Failure::Entropy => ("request-sealing", "entropy"),
        Failure::Packet(_) => ("request-sealing", "encoding"),
        Failure::Endpoint => ("response", "endpoint"),
        #[cfg(any(windows, test))]
        Failure::Connect(kind) => ("connect", io_label(kind)),
        #[cfg(windows)]
        Failure::Peer => ("connect", "peer-mismatch"),
        Failure::TlsSetup(_) => ("tls", "setup"),
        #[cfg(any(windows, test))]
        Failure::Handshake(kind) => ("handshake", io_label(kind)),
        #[cfg(windows)]
        Failure::TlsProfile => ("handshake", "profile"),
        #[cfg(any(windows, test))]
        Failure::Request(kind) => ("request", io_label(kind)),
        #[cfg(windows)]
        Failure::Exporter => ("exporter", "export"),
        #[cfg(any(windows, test))]
        Failure::Shutdown(kind) => ("shutdown", io_label(kind)),
    }
}

/// Classify a typed I/O cause without exposing its original error or source.
#[cfg(any(windows, test))]
fn io_label(kind: io::ErrorKind) -> &'static str {
    match kind {
        io::ErrorKind::TimedOut => "timeout",
        io::ErrorKind::Interrupted => "interrupted",
        io::ErrorKind::ConnectionRefused => "refused",
        io::ErrorKind::UnexpectedEof => "unexpected-eof",
        io::ErrorKind::InvalidData => "invalid-data",
        _ => "io",
    }
}

/// Attempt the bounded failure report while preserving its original exit class.
#[cfg(windows)]
fn report_failure(failure: &Failed, code: std::process::ExitCode) -> std::process::ExitCode {
    let _diagnostic_result = output::write_failure(failure, &mut io::stderr().lock());
    code
}

/// Typed, bounded prototype diagnostics after actual operation retirement.
/// No diagnostic value supplies freshness, eligibility or a cleanup acknowledgement.
#[cfg(any(windows, test))]
mod output {
    use std::io::{self, Cursor, Write};

    use super::{Failed, RunFailed, datagram, failure_labels, io_label, job};

    /// Maximum encoded bytes per complete record, including its terminal LF.
    const RECORD_BYTES: usize = 384;

    /// Stack-only record minted from the actual joined projection or typed failure.
    struct Record {
        /// Fixed storage for the complete encoded row, including its terminal LF.
        bytes: [u8; RECORD_BYTES],
        /// Checked initialized prefix that belongs to this one record.
        used: usize,
    }

    impl Record {
        /// Encode only immutable counts and closed codes from the same joined owner.
        /// # Errors
        /// Reject an unrepresentable or oversized record before any sink output.
        fn completion(projection: &datagram::CompletionProjection<'_>) -> io::Result<Self> {
            let summary = projection.summary();
            let mut bytes = [0; RECORD_BYTES];
            let used = {
                let mut record = Cursor::new(bytes.as_mut_slice());
                writeln!(
                    record,
                    "scope=engine-ke-prototype eligible=0 fresh_sync=0 completed=1 cookies={} cookie_bytes={} sealed_packet_bytes={} transmitted={} native_cleanup_qualified=0 numerical={} numerical_reason={}",
                    summary.cookies,
                    summary.cookie_bytes,
                    summary.packet_bytes,
                    u8::from(summary.transmitted),
                    projection.numerical(),
                    projection.numerical_reason()
                )?;
                usize::try_from(record.position())
                    .map_err(|_| io::Error::other("bounded CLI record length is unrepresentable"))?
            };
            Ok(Self { bytes, used })
        }

        /// Encode permitted operation and secondary failure labels, never their payloads.
        /// # Errors
        /// Reject an unrepresentable or oversized record without emitting partial data.
        fn failure(failure: &Failed) -> io::Result<Self> {
            let (stage, cause) = failure_labels(failure.primary);
            let (cleanup_stage, cleanup_cause) =
                failure.cleanup.map_or(("none", "none"), failure_labels);
            let (admission_stage, admission_cause) = failure
                .admission_cleanup
                .map_or(("none", "none"), failure_labels);
            let mut bytes = [0; RECORD_BYTES];
            let used = {
                let mut record = Cursor::new(bytes.as_mut_slice());
                writeln!(
                    record,
                    "scope=engine-ke-prototype eligible=0 fresh_sync=0 stage={stage} cause={cause} cleanup_stage={cleanup_stage} cleanup_cause={cleanup_cause} admission_stage={admission_stage} admission_cause={admission_cause}"
                )?;
                usize::try_from(record.position())
                    .map_err(|_| io::Error::other("bounded CLI record length is unrepresentable"))?
            };
            Ok(Self { bytes, used })
        }

        /// Encode only the actual worker category and its recorded retirement observation.
        /// # Errors
        /// Reject an unrepresentable or oversized record before any sink output.
        fn worker(failure: &RunFailed) -> io::Result<Self> {
            let (stage, cause, retired) = match failure {
                RunFailed::CallerContext => ("caller-context", "entered-runtime", false),
                RunFailed::PowerRegistration => ("power-registration", "unavailable", false),
                RunFailed::Preparation(operation) | RunFailed::Operation(operation) => {
                    return Self::failure(operation);
                }
                RunFailed::Submission(submission) => (
                    "worker-submission",
                    match submission {
                        job::SubmitError::Busy => "busy",
                        job::SubmitError::Used => "used",
                        job::SubmitError::Control => "control",
                        job::SubmitError::Spawn(kind) => io_label(*kind),
                    },
                    false,
                ),
                RunFailed::Collection(collection) => (
                    "worker-collection",
                    match collection {
                        job::CollectError::Busy => "busy",
                        job::CollectError::NotReady => "not-ready",
                        job::CollectError::SelfJoin => "self-join",
                        job::CollectError::EnteredRuntime => "entered-runtime",
                        job::CollectError::Control => "control",
                    },
                    false,
                ),
                RunFailed::Abnormal(joined) => (
                    "worker-termination",
                    match joined.termination {
                        job::Termination::Panicked => "panicked",
                        job::Termination::Contract(_) => "contract",
                        job::Termination::Returned => "unconfirmed-retirement",
                    },
                    joined.retired,
                ),
            };
            Self::worker_status(stage, cause, retired)
        }

        /// Encode one fixed worker or pre-submission observation status.
        /// # Errors
        /// Reject an oversized or unrepresentable record before writing to a sink.
        fn worker_status(stage: &str, cause: &str, retired: bool) -> io::Result<Self> {
            let mut bytes: [u8; RECORD_BYTES] = [0; RECORD_BYTES];
            let used: usize = {
                let mut record: Cursor<&mut [u8]> = Cursor::new(bytes.as_mut_slice());
                writeln!(
                    record,
                    "scope=engine-ke-prototype eligible=0 fresh_sync=0 stage={stage} cause={cause} retirement_confirmed={}",
                    u8::from(retired)
                )?;
                usize::try_from(record.position())
                    .map_err(|_| io::Error::other("bounded CLI record length is unrepresentable"))?
            };
            Ok(Self { bytes, used })
        }

        /// Write this one record with at most one call per encoded byte, then flush once.
        /// Each short write must make progress; errors, including Interrupted, are final.
        /// # Errors
        /// Preserve sink errors; reject zero progress or an impossible reported extent.
        fn write_to(&self, output: &mut impl Write) -> io::Result<()> {
            let mut remaining = self
                .bytes
                .get(..self.used)
                .ok_or_else(|| io::Error::other("bounded CLI record extent is invalid"))?;
            while !remaining.is_empty() {
                let written = output.write(remaining)?;
                if written == 0 {
                    return Err(io::Error::from(io::ErrorKind::WriteZero));
                }
                remaining = remaining
                    .get(written..)
                    .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
            }
            output.flush()
        }
    }

    /// Emit one actual joined completion without reconstructing or refreshing its facts.
    /// # Errors
    /// Refuse encoding/write/flush failure; never retry acquisition or emit another record.
    pub(super) fn write_completion(
        completion: &datagram::CompletionProjection<'_>,
        output: &mut impl Write,
    ) -> io::Result<()> {
        Record::completion(completion)?.write_to(output)
    }

    /// Best-effort typed diagnostics retain the caller's original failure and exit class.
    /// # Errors
    /// Preserve encoding/write/flush failure without formatting the sink error itself.
    pub(super) fn write_failure(failure: &Failed, output: &mut impl Write) -> io::Result<()> {
        Record::failure(failure)?.write_to(output)
    }

    /// Preserve normal and abnormal failure facts, with no completion-success record.
    /// An abnormal operation's retained primary row follows its worker row only if that
    /// row was fully written; an I/O error never initiates fallback/recursive reporting.
    /// # Errors
    /// Preserve the first encoding/write/flush failure while retaining the input unchanged.
    pub(super) fn write_run_failure(
        failure: &RunFailed,
        output: &mut impl Write,
    ) -> io::Result<()> {
        Record::worker(failure)?.write_to(output)?;
        if let RunFailed::Abnormal(job::JoinedOutcome {
            operation: Some(Err(operation)),
            ..
        }) = failure
        {
            write_failure(operation, output)?;
        }
        Ok(())
    }

    /// Report the observer's registration refusal before any worker submission.
    /// # Errors
    /// Preserve encoding/write/flush failure.
    pub(super) fn write_power_registration_failure(output: &mut impl Write) -> io::Result<()> {
        Record::worker_status("power-registration", "unavailable", false)?.write_to(output)
    }

    /// Report observer teardown separately from actual worker completion.
    /// # Errors
    /// Preserve the original write/flush failure without any retry.
    pub(super) fn write_power_cleanup(
        stop_failed: bool,
        close_failed: bool,
        output: &mut impl Write,
    ) -> io::Result<()> {
        let mut bytes: [u8; RECORD_BYTES] = [0; RECORD_BYTES];
        let used: usize = {
            let mut record: Cursor<&mut [u8]> = Cursor::new(bytes.as_mut_slice());
            writeln!(
                record,
                "scope=engine-ke-prototype eligible=0 fresh_sync=0 stage=power-cleanup stop_failed={} unregistration_failed={}",
                u8::from(stop_failed),
                u8::from(close_failed)
            )?;
            usize::try_from(record.position())
                .map_err(|_| io::Error::other("bounded CLI record length is unrepresentable"))?
        };
        Record { bytes, used }.write_to(output)
    }
}

/// Project the existing result and a separate observer failure, if observed.
/// Return whether observation and output both completed successfully. A refused
/// sink never causes a retry or changes the already-owned acquisition result.
#[cfg(any(windows, test))]
fn write_observed_run(
    observed: ObservedInvocation,
    output: &mut impl io::Write,
    diagnostics: &mut impl io::Write,
) -> bool {
    let ObservedInvocation { result, cleanup }: ObservedInvocation = observed;
    let completed: bool = match receive_probe(result) {
        Ok(completion) => output::write_completion(&completion.projection(), output).is_ok(),
        Err(failure) => {
            if output::write_run_failure(&failure, diagnostics).is_err() {
                return false;
            }
            false
        }
    };
    let close_failed: bool = cleanup.unregistration_error.is_some();
    if cleanup.stop_failed || close_failed {
        let _diagnostic: io::Result<()> =
            output::write_power_cleanup(cleanup.stop_failed, close_failed, diagnostics);
        return false;
    }
    completed
}

/// Admit two bounded CLI arguments and an optional exact protected-datagram flag.
///
/// Successful counts and ordinary operation failures follow actual joined retirement.
/// Post-submission collection refusals report unconfirmed completion while permanent
/// custody retains the actual job; native cleanup remains separately unqualified.
#[cfg(windows)]
pub(super) fn entry(mut arguments: impl Iterator<Item = OsString>) -> std::process::ExitCode {
    use std::process::ExitCode;
    let _program = arguments.next();
    let config = match Config::from_arguments(arguments) {
        Ok(config) => config,
        Err(primary) => return report_failure(&Failed::from(primary), ExitCode::from(2)),
    };
    let observed: ObservedInvocation = match run_with_power_observer(config) {
        Ok(observed) => observed,
        Err(ObserverStartError::Preparation(failure)) => return report_run_failure(&failure),
        Err(ObserverStartError::Registration) => {
            let _diagnostic: io::Result<()> =
                output::write_power_registration_failure(&mut io::stderr().lock());
            return ExitCode::FAILURE;
        }
    };
    if write_observed_run(observed, &mut io::stdout().lock(), &mut io::stderr().lock()) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
