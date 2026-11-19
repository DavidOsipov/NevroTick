// SPDX-License-Identifier: GPL-2.0-only
//! Source-issued KE controls through the actual worker and joined receiver.
//!
//! Public fixed component material establishes no TLS, certificate, exporter or
//! Windows-provider success. Protected controls use the closed datagram backend.

use std::cell::Cell;
use std::ffi::OsString;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::num::{NonZeroU32, NonZeroU64};
use std::sync::mpsc::{Receiver, SyncSender, TrySendError, sync_channel};
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

use super::association::ledger::{RequestInput, RequestPhase, Status, Ticket};
use super::association::{AssociationCandidate, AssociationOwner};
use super::datagram::tests::{managed_component_exchange, managed_component_keys};
use super::datagram::{ProtectedCompletion, ReceivedCompletion};
use super::handoff::ManagedPreviewHistory;
use super::job::{
    Hooks, Observation, Operation, PreparedInvocation, SubmissionReservation, Termination,
    WorkerSlot,
};
use super::managed_preview::{
    SourceKePreviewFailure, SourceKePreviewReport, SourceProtectedProbeFailure,
    SourceProtectedProbeReport, protected_report,
};
use super::process::{ProcessAdmission, ProcessClaim, ProviderScope};
use super::profile::StrictProfile;
use super::recovery::{End, Invocation, Producer, Snapshot, Step, step_with};
use super::request::DirectionalKeys;
use super::response::fixtures;
use super::transport_fault::TransportIo;
use super::{
    Config, Control, Failed, Failure, RunFailed, StopDisposition, StopHandle, Summary, output,
    run_observing, run_synchronously_with,
};
use crate::source_policy::{FixtureRequest, SourceBinding, fixture_control, fixture_send};
use crate::{
    ExperimentalMeasurement, ExperimentalProfile, KePreviewTarget, KePreviewTargetError,
    MeasurementRefusal, SourceConfigurationSnapshot, SourceHandle, SourcePolicyAuthority,
    SourcePolicyError, SourcePolicyLimits, SourcePolicySnapshot, SourceScale,
};

/// Public LI=1 neighbor of HEALTHY, sealed with cryptography 41.0.7 AESSIV.
/// The independent generator first reproduced HEALTHY with the same public
/// key, nonce, UID and plaintext, then sealed the header with LI=1.
const LEAP_ANNOUNCED_REPLY: &[u8] = &[
    0x64, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa,
    0x01, 0x04, 0x00, 0x24, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
    0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
    0x1c, 0x1d, 0x1e, 0x1f, 0x04, 0x04, 0x00, 0x38, 0x00, 0x10, 0x00, 0x20, 0x10, 0x11, 0x12, 0x13,
    0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0xef, 0x05, 0xee, 0x46,
    0xb2, 0x20, 0x92, 0x67, 0x3b, 0x83, 0x69, 0xf1, 0xdc, 0x44, 0x6f, 0x17, 0xa8, 0xdb, 0x1e, 0xea,
    0x8d, 0x66, 0x67, 0x1a, 0x39, 0xa2, 0xaf, 0x21, 0xf4, 0x11, 0x64, 0x2b,
];

/// Finite component observation only; it supplies no product operation deadline.
const WAIT: Duration = Duration::from_secs(2);

/// Shared UDP peer in the fixed component material.
const COMPONENT_UDP: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 123));

/// Configured neighbor with no outstanding request or retained endpoint slot.
const ALTERNATE_UDP: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 124));

/// Select a fault in component work, never source identity or permission.
#[derive(Clone, Copy)]
enum Scenario {
    /// Complete under the genuine unchanged definition.
    Healthy,
    /// Retain two distinct cookies while sealing only the original first cookie.
    TwoCookies,
    /// Replace the actual issued revision while entropy is outside all guards.
    ReplaceDuringEntropy,
    /// Replace after complete material preparation, before the real publisher.
    ReplaceBeforePublication,
    /// Replace after the successful commit, before worker publication and collection.
    ReplaceAfterCommit,
    /// Offer an otherwise valid endpoint outside the installed UDP set.
    OutsideEndpoint,
    /// Replace a protected source just before the actual send.
    ProtectedBeforeSend,
    /// Replace a protected source after actual exchange retirement.
    ProtectedBeforeReplyCommit,
    /// Replace a protected source after commit, before joined reconstruction.
    ProtectedAfterCommit,
    /// Retain an authenticated nonzero peer leap claim through the same replacement.
    ProtectedLeapWarningAfterCommit,
}

impl Scenario {
    /// The original fixture retains a genuine Preview unless explicitly protected.
    const fn protected(self) -> bool {
        matches!(
            self,
            Self::ProtectedBeforeSend
                | Self::ProtectedBeforeReplyCommit
                | Self::ProtectedAfterCommit
                | Self::ProtectedLeapWarningAfterCommit
        )
    }
}

/// Actual checked source owner and definition before preparation.
struct Issued {
    /// Non-clonable authority used by the same real configuration transaction.
    authority: SourcePolicyAuthority,
    /// Actual catalogue handle; it alone captures the managed Preview context.
    source: SourceHandle,
    /// Original immutable issuer snapshot, used only as historical expectation.
    configuration: SourceConfigurationSnapshot,
    /// Genuine checked TLS target reused by equal-value replacement.
    target: KePreviewTarget,
    /// Sole configured UDP endpoint; Preview must not create its endpoint slot.
    allowed: SocketAddr,
}

/// Admit literal loopback component inputs through the real checked issuers.
/// # Errors
/// Preserve source/configuration refusal or the bounded invalid-target category.
fn issued(scenario: Scenario) -> Result<Issued, Failure> {
    let limits: SourcePolicyLimits =
        SourcePolicyLimits::new(1, 1, Duration::from_millis(1), Duration::from_millis(4))
            .map_err(Failure::SourcePolicy)?;
    let mut authority: SourcePolicyAuthority =
        crate::source_policy::isolated_authority(limits).map_err(Failure::SourcePolicy)?;
    let source: SourceHandle = authority.source(0).map_err(Failure::SourcePolicy)?;
    let tls: SocketAddr = SocketAddr::from((Ipv4Addr::LOCALHOST, 4460));
    let target: KePreviewTarget = KePreviewTarget::new(tls, "Time.Example")
        .map_err(|_: KePreviewTargetError| Failure::Arguments)?;
    let allowed: SocketAddr = SocketAddr::from((
        Ipv4Addr::LOCALHOST,
        if matches!(scenario, Scenario::OutsideEndpoint) {
            124_u16
        } else {
            123_u16
        },
    ));
    let addresses: crate::SourceAddressSet =
        crate::SourceAddressSet::new(&[allowed]).map_err(Failure::SourcePolicy)?;
    let configuration: SourceConfigurationSnapshot = authority
        .install_ke_preview_configuration(&source, addresses, target.clone())
        .map_err(Failure::SourcePolicy)?;
    let configuration: SourceConfigurationSnapshot = if matches!(
        scenario,
        Scenario::ProtectedAfterCommit | Scenario::ProtectedLeapWarningAfterCommit
    ) {
        authority
            .replace_source_scale(&configuration, SourceScale::UnsmearedUtc)
            .map_err(Failure::SourcePolicy)?
    } else {
        configuration
    };
    Ok(Issued {
        authority,
        source,
        configuration,
        target,
        allowed,
    })
}

impl Issued {
    /// Capture a checked KE input for the actual issued revision.
    ///
    /// # Errors
    /// Preserve the real source owner refusal.
    fn transport_config(&self) -> Result<Config, Failure> {
        self.authority
            .authorize_protected_ke(&self.configuration)
            .map(Config::from_managed_protected)
            .map_err(Failure::SourcePolicy)
    }

    /// Replace the actual definition while retaining the TLS and UDP values.
    ///
    /// # Errors
    /// Preserve real policy failure during replacement.
    /// # Panics
    /// Fail if the issuer did not create a distinct same-values revision.
    fn replace_transport_definition(&mut self) -> Result<(), Failure> {
        let replacement: SourceConfigurationSnapshot = self
            .authority
            .replace_ke_preview_configuration(
                &self.configuration,
                self.configuration.addresses().clone(),
                self.target.clone(),
            )
            .map_err(Failure::SourcePolicy)?;
        assert_eq!(replacement.addresses().len(), 1);
        assert!(replacement.addresses().allows(self.allowed));
        assert!(!replacement.same_configuration(&self.configuration));
        self.configuration = replacement;
        Ok(())
    }
}

/// Refuse an obsolete configuration before the connect adapter sees its peer.
///
/// # Errors
/// Preserve real fixture issuance and any rejected healthy connect.
/// # Panics
/// Fail if an equal-target replacement reaches connect or masks prior cancellation.
#[test]
fn managed_ke_transport_refuses_replaced_definition_before_connect() -> Result<(), Failure> {
    let mut fixture: Issued = issued(Scenario::Healthy)?;
    let original: Config = fixture.transport_config()?;
    fixture.replace_transport_definition()?;

    let control: Control = Control::new(Instant::now())?;
    let mut stale: TransportIo<'_> = TransportIo::new(&original, &control);
    let calls: Cell<usize> = Cell::new(0);
    let refused: Result<(), Failure> = stale.connect(|_: SocketAddr, _: Duration| {
        calls.set(calls.get() + 1);
        Ok(())
    });
    assert_eq!(
        refused,
        Err(Failure::SourcePolicy(
            SourcePolicyError::ConfigurationChanged
        ))
    );
    assert_eq!(calls.get(), 0);

    let cancellation: super::CancelOutcome = control.cancel()?;
    assert_eq!(cancellation, super::CancelOutcome::Cancelled);
    let mut stopped: TransportIo<'_> = TransportIo::new(&original, &control);
    assert_eq!(
        stopped.connect(|_: SocketAddr, _: Duration| {
            calls.set(calls.get() + 1);
            Ok(())
        }),
        Err(Failure::Cancelled)
    );
    assert_eq!(calls.get(), 0);

    let expired_origin: Instant = Instant::now()
        .checked_sub(super::HANDSHAKE_TIME)
        .ok_or(Failure::Deadline)?;
    let expired: Control = Control::new(expired_origin)?;
    let mut late: TransportIo<'_> = TransportIo::new(&original, &expired);
    assert_eq!(
        late.connect(|_: SocketAddr, _: Duration| {
            calls.set(calls.get() + 1);
            Ok(())
        }),
        Err(Failure::Deadline)
    );
    assert_eq!(calls.get(), 0);

    let current: Config = fixture.transport_config()?;
    let next: Control = Control::new(Instant::now())?;
    let mut authorized: TransportIo<'_> = TransportIo::new(&current, &next);
    let peer: SocketAddr = authorized.connect(|peer: SocketAddr, timeout: Duration| {
        assert!(!timeout.is_zero());
        calls.set(calls.get() + 1);
        Ok(peer)
    })?;
    assert_eq!(peer, fixture.target.peer());
    assert_eq!(calls.get(), 1);
    Ok(())
}

/// Observe exactly one destruction of a successfully returned adapter object.
struct ConnectedWitness<'a> {
    /// The test's independent destruction count.
    drops: &'a Cell<usize>,
}

impl Drop for ConnectedWitness<'_> {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

/// Retire an owned connect result if its original source changes in flight.
///
/// # Errors
/// Preserve real fixture issuance and any unexpected replacement failure.
/// # Panics
/// Fail if the stale connection escapes, is leaked or is retired more than once.
#[test]
fn managed_ke_transport_drops_owned_connect_on_revision_cutover() -> Result<(), Failure> {
    let mut fixture: Issued = issued(Scenario::Healthy)?;
    let config: Config = fixture.transport_config()?;
    let control: Control = Control::new(Instant::now())?;
    let mut transport: TransportIo<'_> = TransportIo::new(&config, &control);
    let drops: Cell<usize> = Cell::new(0);
    let replacement: Cell<Option<Result<(), Failure>>> = Cell::new(None);
    let connected: Result<ConnectedWitness<'_>, Failure> =
        transport.connect(|peer: SocketAddr, timeout: Duration| {
            assert_eq!(peer, fixture.target.peer());
            assert!(!timeout.is_zero() && timeout <= super::HANDSHAKE_TIME);
            replacement.set(Some(fixture.replace_transport_definition()));
            Ok(ConnectedWitness { drops: &drops })
        });
    replacement.get().ok_or(Failure::Control)??;
    assert!(matches!(
        &connected,
        Err(Failure::SourcePolicy(
            SourcePolicyError::ConfigurationChanged
        ))
    ));
    assert_eq!(
        drops.get(),
        1,
        "post-connect refusal retired the returned object"
    );
    drop(connected);
    assert_eq!(drops.get(), 1);
    Ok(())
}

/// Place a same-values replacement relative to one shared transport transfer.
#[derive(Clone, Copy)]
enum TransportReplacement {
    /// The timeout adapter returns after the source changes.
    Timeout,
    /// The I/O adapter returns after the source changes.
    Operation,
    /// Preserve an ordinary complete transfer.
    None,
}

/// Follow the real transport checkpoints on each side of an admitted adapter call.
///
/// # Errors
/// Preserve real source admission or an unexpected native/checkpoint refusal.
/// # Panics
/// Fail on a stale issued revision, an extra effect or a lost healthy result.
#[test]
fn managed_ke_transport_rechecks_revision_after_each_adapter_return() -> Result<(), Failure> {
    for (replacement, expected_calls) in [
        (TransportReplacement::Timeout, 0_usize),
        (TransportReplacement::Operation, 1),
        (TransportReplacement::None, 1),
    ] {
        let mut fixture: Issued = issued(Scenario::Healthy)?;
        let config: Config = fixture.transport_config()?;
        let control: Control = Control::new(Instant::now())?;
        let mut transport: TransportIo<'_> = TransportIo::new(&config, &control);
        transport.begin_exchange();
        let replacement_result: Cell<Option<Result<(), Failure>>> = Cell::new(None);
        let timeout_calls: Cell<usize> = Cell::new(0);
        let operation_calls: Cell<usize> = Cell::new(0);
        let result: io::Result<usize> = transport.transfer(
            &mut fixture,
            |fixture: &mut Issued, timeout: Duration| {
                assert!(!timeout.is_zero());
                timeout_calls.set(timeout_calls.get() + 1);
                if matches!(replacement, TransportReplacement::Timeout) {
                    replacement_result.set(Some(fixture.replace_transport_definition()));
                }
                Ok(())
            },
            |fixture: &mut Issued| {
                operation_calls.set(operation_calls.get() + 1);
                if matches!(replacement, TransportReplacement::Operation) {
                    replacement_result.set(Some(fixture.replace_transport_definition()));
                }
                Ok(7_usize)
            },
        );
        let expected: Result<usize, Failure> = if let Some(changed) = replacement_result.get() {
            changed?;
            Err(Failure::SourcePolicy(
                SourcePolicyError::ConfigurationChanged,
            ))
        } else {
            assert!(matches!(replacement, TransportReplacement::None));
            Ok(7_usize)
        };
        assert_eq!(
            transport.resolve(
                result.map_err(|error: io::Error| Failure::Request(error.kind())),
                Failure::Request
            ),
            expected
        );
        assert_eq!(timeout_calls.get(), 1);
        assert_eq!(operation_calls.get(), expected_calls);
        if let Err(stale) = expected {
            let refused: io::Result<()> = transport.transfer(
                &mut fixture,
                |_: &mut Issued, _: Duration| {
                    timeout_calls.set(timeout_calls.get() + 1);
                    Ok(())
                },
                |_: &mut Issued| {
                    operation_calls.set(operation_calls.get() + 1);
                    Ok(())
                },
            );
            assert_eq!(
                refused.err().map(|error: io::Error| error.kind()),
                Some(io::ErrorKind::Other)
            );
            assert_eq!(transport.current(Failure::Shutdown), Err(stale));
            assert_eq!(timeout_calls.get(), 1);
            assert_eq!(operation_calls.get(), expected_calls);
        } else {
            fixture.replace_transport_definition()?;
            assert_eq!(
                transport.current(Failure::Response),
                Err(Failure::SourcePolicy(
                    SourcePolicyError::ConfigurationChanged
                ))
            );
            assert_eq!(operation_calls.get(), 1);
        }
    }
    Ok(())
}

/// Exchange work retains its original longer cutoff after the handshake window.
/// # Errors
/// Preserve fixture/source admission and any unexpected transfer refusal.
/// # Panics
/// Fail if the old handshake cutoff survives the phase transition or the
/// exchange timeout is refreshed relative to the original invocation.
#[test]
fn managed_ke_transport_exchange_uses_original_later_cutoff() -> Result<(), Failure> {
    let fixture: Issued = issued(Scenario::Healthy)?;
    let config: Config = fixture.transport_config()?;
    let origin: Instant = Instant::now()
        .checked_sub(super::HANDSHAKE_TIME)
        .ok_or(Failure::Deadline)?;
    let control: Control = Control::new(origin)?;
    let mut handshake: TransportIo<'_> = TransportIo::new(&config, &control);
    assert_eq!(
        handshake.current(Failure::Handshake),
        Err(Failure::Deadline)
    );

    let mut exchange: TransportIo<'_> = TransportIo::new(&config, &control);
    exchange.begin_exchange();
    let limit: Duration = control.expiry.saturating_duration_since(Instant::now());
    let mut configured: Option<Duration> = None;
    let received: io::Result<u8> = exchange.transfer(
        &mut configured,
        |observed: &mut Option<Duration>, timeout: Duration| {
            *observed = Some(timeout);
            Ok(())
        },
        |observed: &mut Option<Duration>| {
            assert!(observed.is_some_and(|value: Duration| !value.is_zero() && value <= limit));
            Ok(7)
        },
    );
    assert_eq!(received.map_err(|error: io::Error| error.kind()), Ok(7));
    Ok(())
}

/// Deadline and Stop keep their initial adapter categories and block later I/O.
/// # Errors
/// Preserve admitted fixture, source or cancellation failures.
/// # Panics
/// Fail if the checker loses the first I/O category or enters either adapter.
#[test]
fn managed_ke_transport_refusal_keeps_initial_io_category() -> Result<(), Failure> {
    let fixture: Issued = issued(Scenario::Healthy)?;
    let config: Config = fixture.transport_config()?;
    let expired_origin: Instant = Instant::now()
        .checked_sub(super::HANDSHAKE_TIME)
        .ok_or(Failure::Deadline)?;
    let stopped: Control = Control::new(Instant::now())?;
    assert_eq!(stopped.cancel()?, super::CancelOutcome::Cancelled);
    for (control, cause, kind) in [
        (
            Control::new(expired_origin)?,
            Failure::Deadline,
            io::ErrorKind::TimedOut,
        ),
        (stopped, Failure::Cancelled, io::ErrorKind::Interrupted),
    ] {
        let mut owner: TransportIo<'_> = TransportIo::new(&config, &control);
        let mut seen: u8 = 0;
        let attempt = |transport: &mut TransportIo<'_>, count: &mut u8| {
            transport
                .transfer(
                    count,
                    |called: &mut u8, _: Duration| {
                        *called += 1;
                        Ok(())
                    },
                    |called: &mut u8| {
                        *called += 1;
                        Ok(())
                    },
                )
                .map_err(|error: io::Error| error.kind())
        };
        assert_eq!(attempt(&mut owner, &mut seen), Err(kind));
        assert_eq!(attempt(&mut owner, &mut seen), Err(io::ErrorKind::Other));
        assert_eq!(owner.current(Failure::Response), Err(cause));
        assert_eq!(seen, 0);
    }
    Ok(())
}

/// A later revision change leaves the independently latched native error primary.
///
/// # Errors
/// Preserve real source issuance, original Control or replacement errors.
/// # Panics
/// Fail if later stale context rewrites the earlier native refusal.
#[test]
fn managed_ke_transport_preserves_first_native_failure_after_replacement() -> Result<(), Failure> {
    let mut fixture: Issued = issued(Scenario::Healthy)?;
    let config: Config = fixture.transport_config()?;
    let control: Control = Control::new(Instant::now())?;
    let mut transport: TransportIo<'_> = TransportIo::new(&config, &control);
    let operation_calls: Cell<usize> = Cell::new(0);
    let changed: Cell<Option<Result<(), Failure>>> = Cell::new(None);
    let result: io::Result<()> = transport.transfer(
        &mut fixture,
        |_: &mut Issued, _: Duration| Ok(()),
        |fixture: &mut Issued| {
            operation_calls.set(operation_calls.get() + 1);
            changed.set(Some(fixture.replace_transport_definition()));
            Err(io::ErrorKind::BrokenPipe.into())
        },
    );
    changed.get().ok_or(Failure::Control)??;
    assert_eq!(
        result.err().map(|error: io::Error| error.kind()),
        Some(io::ErrorKind::BrokenPipe)
    );
    assert_eq!(
        transport.current(Failure::Request),
        Err(Failure::Request(io::ErrorKind::BrokenPipe))
    );
    assert_eq!(operation_calls.get(), 1);
    Ok(())
}

/// One actual component producer; every context still comes from the issuer.
struct PreviewProducer {
    /// Real authority moved to the worker for the entropy-boundary replacement.
    authority: SourcePolicyAuthority,
    /// Actual initial revision against which replacement is conditional.
    configuration: SourceConfigurationSnapshot,
    /// Checked original target; no caller-selected TLS override enters Config.
    target: KePreviewTarget,
    /// Explicit test action, with no authority-bearing data.
    scenario: Scenario,
    /// Observer's original Stop capability, delivered after protected retirement if set.
    stop_before_reply_commit: Option<StopHandle>,
    /// Genuine newer snapshot returned by the actual replacement transaction.
    replacement: Option<SourceConfigurationSnapshot>,
    /// Actual establishment/entropy/protected-operation/jitter entries.
    calls: [usize; 4],
}

impl PreviewProducer {
    /// Replace the actual definition with equal target/address values once.
    /// # Errors
    /// Preserve genuine stale/foreign, contention or exhaustion refusal.
    /// # Panics
    /// Reject an issuer that fails to advance the genuine configuration identity.
    fn replace_definition(&mut self) -> Result<(), Failure> {
        let newer: SourceConfigurationSnapshot = self
            .authority
            .replace_ke_preview_configuration(
                &self.configuration,
                self.configuration.addresses().clone(),
                self.target.clone(),
            )
            .map_err(Failure::SourcePolicy)?;
        assert!(!newer.same_configuration(&self.configuration));
        self.replacement = Some(newer);
        Ok(())
    }
}

impl Producer for PreviewProducer {
    fn establish<'tx>(
        &mut self,
        config: &'tx Config,
        control: &'tx Control,
        provider: &'tx ProviderScope<'tx>,
    ) -> Result<AssociationOwner<'tx>, Failed> {
        self.calls[0] += 1;
        assert_eq!(config.peer, self.target.peer());
        assert_eq!(config.identity, self.target.identity());
        let candidate: AssociationCandidate<'tx> =
            component_candidate(config, control, provider, COMPONENT_UDP, self.scenario);
        if matches!(self.scenario, Scenario::ReplaceBeforePublication) {
            self.replace_definition().map_err(Failed::from)?;
        }
        AssociationOwner::receive(config, control, provider, candidate).map_err(Failed::from)
    }

    fn exchange<'tx>(
        &mut self,
        association: &mut AssociationOwner<'tx>,
        control: &'tx Control,
    ) -> Result<ProtectedCompletion<'tx>, Failure> {
        self.calls[2] += 1;
        if !self.scenario.protected() {
            return Err(Failure::Control);
        }
        let reply: &'static [u8] =
            if matches!(self.scenario, Scenario::ProtectedLeapWarningAfterCommit) {
                LEAP_ANNOUNCED_REPLY
            } else {
                fixtures::HEALTHY
            };
        let completion: ProtectedCompletion<'tx> =
            managed_component_exchange(association, control, reply, || {
                if matches!(self.scenario, Scenario::ProtectedBeforeSend) {
                    self.replace_definition()?;
                }
                Ok(())
            })?;
        assert_eq!(association.cookie_population()?, (0, 0));
        if matches!(self.scenario, Scenario::ProtectedBeforeReplyCommit) {
            self.replacement = Some(
                self.authority
                    .replace_source_scale(&self.configuration, SourceScale::Smeared)
                    .map_err(Failure::SourcePolicy)?,
            );
        }
        if let Some(stop) = self.stop_before_reply_commit.take() {
            assert_eq!(stop.request_stop(), Ok(StopDisposition::Latched));
        }
        Ok(completion)
    }

    fn preview_entropy(&mut self, output: &mut [u8; 48]) -> Result<(), Failure> {
        self.calls[1] += 1;
        if matches!(self.scenario, Scenario::ReplaceDuringEntropy) {
            self.replace_definition()?;
        }
        output.fill(0x42_u8);
        Ok(())
    }

    fn jitter(&mut self, _upper: Duration) -> Result<Duration, Failure> {
        self.calls[3] += 1;
        Err(Failure::Control)
    }
}

/// Supply fixed public component material to the same actual receiving constructor.
/// The candidate claims no TLS or exporter qualification and retains all real owners.
fn component_candidate<'a>(
    config: &'a Config,
    control: &'a Control,
    provider: &'a ProviderScope<'a>,
    udp: SocketAddr,
    scenario: Scenario,
) -> AssociationCandidate<'a> {
    let profile: StrictProfile = StrictProfile::component();
    let keys: DirectionalKeys = if matches!(&config.mode, super::ExchangeMode::ManagedProtected(_))
    {
        managed_component_keys()
    } else {
        DirectionalKeys::new(Zeroizing::new([0x11_u8; 32]), Zeroizing::new([0x22_u8; 32]))
    };
    let cookies: Vec<Zeroizing<Vec<u8>>> = if matches!(scenario, Scenario::TwoCookies) {
        vec![
            Zeroizing::new(vec![0xa5_u8; 12]),
            Zeroizing::new(vec![0xb6_u8; 16]),
        ]
    } else {
        vec![Zeroizing::new(vec![0xa5_u8; 12])]
    };
    AssociationCandidate::new(config, control, provider, udp, profile, keys, cookies)
}

/// Publish one genuine component owner for direct guarded-entry controls.
/// # Errors
/// Preserve actual control/provider/publication and exercised consumer refusals.
fn with_owner(
    config: &Config,
    udp: SocketAddr,
    exercise: impl FnOnce(&mut AssociationOwner<'_>, &Control) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let control: Control = Control::new(Instant::now())?;
    let admission: ProcessAdmission = ProcessAdmission::new();
    let claim: ProcessClaim<'_> = admission.claim()?;
    let provider: ProviderScope<'_> = ProviderScope::component(claim);
    let candidate: AssociationCandidate<'_> =
        component_candidate(config, &control, &provider, udp, Scenario::Healthy);
    let mut owner: AssociationOwner<'_> =
        AssociationOwner::receive(config, &control, &provider, candidate)?;
    exercise(&mut owner, &control)
}

/// Legal input neighbors cover both direct protected reservation representations.
/// # Errors
/// Preserve maintained timestamp decoding or the checked nonzero wire-token refusal.
fn protected_inputs() -> Result<[RequestInput; 2], Failure> {
    let header: ntp_wire::RawHeader =
        ntp_wire::decode_fixed_header(&[0_u8; 48]).map_err(|_| Failure::Control)?;
    let wire: NonZeroU64 = NonZeroU64::new(0x1122_3344_5566_7788).ok_or(Failure::Control)?;
    Ok([
        RequestInput::Observed,
        RequestInput::Packet {
            wire,
            local_t1: header.transmit_timestamp(),
        },
    ])
}

/// Both diagnostic modes retain their existing legal adapter/ledger entries.
/// # Errors
/// Preserve actual admission, publication, adapter context and reservation refusal.
/// # Panics
/// Reject a blanket mode refusal or incorrect original protected checkout accounting.
fn assert_protected_neighbor(config: &Config) -> Result<(), Failure> {
    let inputs: [RequestInput; 2] = protected_inputs()?;
    for input in inputs {
        with_owner(
            config,
            COMPONENT_UDP,
            |owner: &mut AssociationOwner<'_>, control: &Control| {
                let before: Status = owner.status();
                let expected: SocketAddr = SocketAddr::from((Ipv4Addr::LOCALHOST, 123));
                assert_eq!(owner.check_context(control)?, expected);
                assert_eq!(owner.status(), before);
                let _ticket: Ticket<'_> = owner.reserve(input)?;
                let after: Status = owner.status();
                assert_eq!(after.phase, RequestPhase::Reserved);
                assert_eq!(
                    (
                        after.attempts,
                        after.frames,
                        after.bytes,
                        after.available,
                        after.used,
                        after.uids
                    ),
                    (31, 32, 131_072, 0, 0, 0)
                );
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// Post-step observations sent before the real worker records and retires its result.
struct Work {
    /// Original immutable invocation coordinates and initially unspent counts.
    before: Snapshot,
    /// The same invocation after its one actual transaction.
    after: Snapshot,
    /// Original complete failure retained by the invocation, if any.
    last_failure: Option<Failed>,
    /// Actual producer entry counts, including forbidden UDP/jitter controls.
    calls: [usize; 4],
    /// Actual replacement result, never a fabricated revision label.
    replacement: Option<SourceConfigurationSnapshot>,
    /// Weak observation of this actual component provider's destruction.
    provider_lifetime: Weak<()>,
}

/// Use the existing component provider without invoking Windows/CNG effects.
/// # Errors
/// This factory is infallible but retains the real worker's fallible signature.
#[expect(
    clippy::unnecessary_wraps,
    reason = "The actual worker accepts its existing fallible provider-factory type."
)]
fn component_provider(claim: ProcessClaim<'_>) -> Result<ProviderScope<'_>, Failure> {
    Ok(ProviderScope::component(claim))
}

/// Execute the production invocation/association/sealing machinery on the real worker.
/// # Errors
/// Preserve genuine primary/cleanup results; observation failure is a fixture refusal.
/// # Panics
/// Reject a reopened terminal invocation or changed original work accounting.
fn component_operation<'scope>(
    config: &'scope Config,
    provider: &'scope ProviderScope<'scope>,
    control: Control,
    mut producer: PreviewProducer,
    observed: &SyncSender<Work>,
) -> Operation {
    let mut invocation: Invocation<'scope> =
        Invocation::one_shot(config, provider, control).map_err(Failed::from)?;
    let before: Snapshot = invocation.snapshot();
    let provider_lifetime: Weak<()> = provider.retirement_witness();
    let outcome: Result<Step, Failed> = step_with(&mut invocation, &mut producer);
    if matches!(
        producer.scenario,
        Scenario::ReplaceAfterCommit
            | Scenario::ProtectedAfterCommit
            | Scenario::ProtectedLeapWarningAfterCommit
    ) && matches!(&outcome, Ok(Step::Complete(_)))
    {
        producer.replace_definition().map_err(Failed::from)?;
    }
    let after: Snapshot = invocation.snapshot();
    let last_failure: Option<Failed> = invocation.last_failure().copied();
    let terminal: End = after.end.ok_or_else(|| Failed::from(Failure::Control))?;
    let closed: Result<Step, Failed> = step_with(&mut invocation, &mut producer);
    assert!(matches!(closed, Ok(Step::Closed(end)) if end == terminal));
    assert_eq!(invocation.snapshot(), after);
    drop(invocation);
    let operation: Operation = match outcome {
        Ok(Step::Complete(completion)) => Ok(completion),
        Err(failure) => Err(failure),
        Ok(Step::Closed(_) | Step::Deferred(_)) => Err(Failed::from(Failure::Control)),
    };
    let work: Work = Work {
        before,
        after,
        last_failure,
        calls: producer.calls,
        replacement: producer.replacement,
        provider_lifetime,
    };
    observed
        .try_send(work)
        .map_err(|_: TrySendError<Work>| Failed::from(Failure::Control))?;
    operation
}

/// Joined result coupled to independent original-context expectations.
struct Run {
    /// Actual joined receiver result including a possible abnormal termination.
    outcome: Result<ReceivedCompletion, RunFailed>,
    /// Actual completed invocation observations and returned issuer.
    work: Work,
    /// Original source handle permits read-only state checks after real retirement.
    source: SourceHandle,
    /// Original configuration is historical expectation, never fresh permission.
    expected: SourceConfigurationSnapshot,
}

/// Drain unexpected real work before failing a facade-custody assertion.
/// # Errors
/// Preserve unavailable actual collection; never infer completion from observation.
fn drain(slot: &'static WorkerSlot, observation: Observation) -> Result<(), Failure> {
    match observation {
        Observation::Running(_) => {
            slot.collect().map_err(|_| Failure::Control)?;
            drop(slot.take_fixture_result().map_err(|_| Failure::Control)?);
        }
        Observation::Joined => drop(slot.take_fixture_result().map_err(|_| Failure::Control)?),
        Observation::Taken | Observation::Unused | Observation::NotSubmitted(_) => {}
        Observation::Busy | Observation::Joining | Observation::Control | Observation::TimedOut => {
            return Err(Failure::Control);
        }
    }
    Ok(())
}

/// Run one source-issued configuration through the real synchronous facade and slot.
/// # Errors
/// Preserve typed source failure, actual worker/observation refusal and bounded diagnostics.
/// # Panics
/// Reject unjoined custody, retained provider/Config, endpoint-slot creation or work reset.
fn run(
    slot: &'static WorkerSlot,
    admission: &'static ProcessAdmission,
    scenario: Scenario,
) -> Result<Run, Failure> {
    run_with_hooks(slot, admission, scenario, Hooks::none(), None)
}

/// Install the selected worker hook while exercising the real source and joined result.
/// # Errors
/// Preserve source, worker, observation and output refusal.
/// # Panics
/// Reject unjoined or retained real resources and lost policy or work accounting.
fn run_with_hooks(
    slot: &'static WorkerSlot,
    admission: &'static ProcessAdmission,
    scenario: Scenario,
    hooks: Hooks,
    profile: Option<ExperimentalProfile>,
) -> Result<Run, Failure> {
    let fixture: Issued = issued(scenario)?;
    let expected: SourceConfigurationSnapshot = fixture.configuration.clone();
    let config: Config = if scenario.protected() {
        Config::from_managed_protected(
            fixture
                .authority
                .authorize_protected_ke(&expected)
                .map_err(Failure::SourcePolicy)?,
        )
    } else {
        Config::from_managed_preview(
            fixture
                .source
                .capture_ke_preview()
                .map_err(Failure::SourcePolicy)?,
        )
    };
    let config_lifetime: Weak<()> = Arc::downgrade(&config.lifetime);
    let producer: PreviewProducer = PreviewProducer {
        authority: fixture.authority,
        configuration: fixture.configuration,
        target: fixture.target,
        scenario,
        stop_before_reply_commit: None,
        replacement: None,
        calls: [0; 4],
    };
    let (observed, receiver): (SyncSender<Work>, Receiver<Work>) = sync_channel(1);
    let result: Result<ReceivedCompletion, RunFailed> = run_synchronously_with(
        config,
        slot,
        profile,
        move |actual_slot: &'static WorkerSlot, prepared: PreparedInvocation| {
            actual_slot.submit_fixture(
                prepared,
                admission,
                component_provider,
                move |original: &Config, provider: &ProviderScope<'_>, control: Control| {
                    component_operation(original, provider, control, producer, &observed)
                },
                hooks,
            )
        },
    );
    let observation_end: Instant = Instant::now().checked_add(WAIT).ok_or(Failure::Deadline)?;
    let observation: Observation = slot.observer().poll_until(observation_end);
    drain(slot, observation)?;
    if let Err(failure) = &result {
        let mut diagnostic: Vec<u8> = Vec::new();
        output::write_run_failure(failure, &mut diagnostic).map_err(|_| Failure::Control)?;
        eprintln!(
            "KE_MANAGED_PREVIEW_RUN {}",
            String::from_utf8_lossy(&diagnostic)
        );
    }
    assert_eq!(observation, Observation::Taken);
    assert_eq!(config_lifetime.strong_count(), 0);
    assert!(matches!(admission.claim(), Err(Failure::ProcessUsed)));
    let work: Work = receiver.recv_timeout(WAIT).map_err(|_| Failure::Control)?;
    assert_eq!(work.provider_lifetime.strong_count(), 0);
    let checked_peers: [SocketAddr; 2] = [
        fixture.allowed,
        SocketAddr::from((Ipv4Addr::LOCALHOST, 123)),
    ];
    if scenario.protected() {
        let actual: SourcePolicySnapshot = fixture
            .source
            .policy_snapshot(fixture.allowed)
            .map_err(Failure::SourcePolicy)?;
        let should_debit: bool = !matches!(scenario, Scenario::ProtectedBeforeSend);
        assert_eq!(actual.source().floor().is_some(), should_debit);
        assert_eq!(actual.endpoint().floor().is_some(), should_debit);
    } else {
        for peer in checked_peers {
            assert!(matches!(
                fixture.source.policy_snapshot(peer),
                Err(SourcePolicyError::UnknownEndpoint)
            ));
        }
    }
    assert_work_coordinates(&work);
    Ok(Run {
        outcome: result,
        work,
        source: fixture.source,
        expected,
    })
}

/// Keep the normal operation's original cause in a fixture failure.
fn operation_primary(failure: &RunFailed) -> Failure {
    match failure {
        RunFailed::Operation(operation) => operation.primary,
        _ => Failure::Control,
    }
}

/// Compare original coordinates and initially unspent counters without reading fresh time.
/// # Panics
/// Reject a renewed cutoff, uncharged original state or introduced recovery history.
fn assert_work_coordinates(work: &Work) {
    assert_eq!(
        (
            work.before.transactions,
            work.before.refusals,
            work.before.observations,
            work.before.contexts
        ),
        (0, 0, 0, 0)
    );
    assert_eq!(
        (work.after.origin, work.after.cutoff),
        (work.before.origin, work.before.cutoff)
    );
    assert_eq!(
        (
            work.after.not_before,
            work.after.source_not_before,
            work.after.global_not_before,
            work.after.intent_not_before
        ),
        (
            work.before.not_before,
            work.before.source_not_before,
            work.before.global_not_before,
            work.before.intent_not_before
        )
    );
    assert!(!work.after.has_previous && work.after.base.is_none());
}

/// Inspect genuine joined history and independent fixed framing/count expectations.
/// # Errors
/// Preserve the absence of the required managed historical projection.
/// # Panics
/// Reject an unmanaged substitute, replaced original context or UDP effect.
fn assert_history(
    completion: ReceivedCompletion,
    expected: &SourceConfigurationSnapshot,
    counts: (usize, usize, usize),
) -> Result<(), Failure> {
    assert!(completion.protected().is_none());
    assert!(!completion.summary().transmitted);
    let report: SourceKePreviewReport = SourceKePreviewReport::from_run(Ok(completion))
        .map_err(|_: SourceKePreviewFailure| Failure::Control)?;
    assert!(report.configuration().same_configuration(expected));
    assert_eq!(
        (
            report.cookie_count(),
            report.cookie_bytes(),
            report.packet_bytes()
        ),
        counts
    );
    assert_eq!(
        format!("{report:?}"),
        "SourceKePreviewReport { joined: true, transmitted: false, .. }"
    );
    Ok(())
}

/// Inspect one actual healthy joined Preview without deriving its count oracle.
/// # Errors
/// Preserve actual issuer, worker, provider and managed receiving refusal.
/// # Panics
/// Reject a missing healthy completion, changed counts or reset work.
fn assert_healthy_preview(
    slot: &'static WorkerSlot,
    admission: &'static ProcessAdmission,
    scenario: Scenario,
    counts: (usize, usize, usize),
) -> Result<(), Failure> {
    let run: Run = run(slot, admission, scenario)?;
    assert!(
        run.outcome.is_ok(),
        "issued legal Preview must complete through its actual joined receiver"
    );
    let completion: ReceivedCompletion = run
        .outcome
        .map_err(|failure: RunFailed| operation_primary(&failure))?;
    assert_history(completion, &run.expected, counts)?;
    assert_eq!(run.work.calls, [1, 1, 0, 0]);
    assert_eq!(
        (
            run.work.after.transactions,
            run.work.after.refusals,
            run.work.after.observations,
            run.work.after.contexts
        ),
        (1, 0, 2, 1)
    );
    assert_eq!(run.work.after.end, Some(End::Completed));
    assert!(run.work.last_failure.is_none() && run.work.replacement.is_none());
    Ok(())
}

/// Both checked cookie populations cross the actual worker/sealer/joined receiver.
/// # Errors
/// Preserve actual issuer, worker, provider and managed receiving refusal.
/// # Panics
/// Reject a changed population or packet extent and any reset work accounting.
#[test]
fn managed_ke_preview_worker_healthy_preserves_issued_history() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    static TWO_COOKIE_SLOT: WorkerSlot = WorkerSlot::new();
    static TWO_COOKIE_ADMISSION: ProcessAdmission = ProcessAdmission::new();
    assert_healthy_preview(&SLOT, &ADMISSION, Scenario::Healthy, (1, 12, 140))?;
    // The first twelve-byte cookie determines the sealed packet extent; both
    // independently specified bodies remain in the checked population summary.
    assert_healthy_preview(
        &TWO_COOKIE_SLOT,
        &TWO_COOKIE_ADMISSION,
        Scenario::TwoCookies,
        (2, 28, 140),
    )
}

/// A protected source commits under its original definition, then retains that history.
/// # Errors
/// Preserve actual issuer, worker, adapter or joined receiving refusal.
/// # Panics
/// Reject premature cookie installation, absent probing or postcommit relabelling.
#[test]
fn managed_protected_worker_preserves_committed_definition_through_join() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let run: Run = run(&SLOT, &ADMISSION, Scenario::ProtectedLeapWarningAfterCommit)?;
    let completion: ReceivedCompletion = run
        .outcome
        .map_err(|failure: RunFailed| operation_primary(&failure))?;
    let original_packet_bytes: usize = completion.summary().packet_bytes;
    assert!(completion.summary().transmitted);
    let report: SourceProtectedProbeReport = protected_report(completion)?;
    assert!(report.configuration().same_configuration(&run.expected));
    assert_eq!(
        report.configuration().declared_scale(),
        SourceScale::UnsmearedUtc
    );
    assert_eq!(report.leap_indicator(), 1);
    assert_eq!(report.measurement(), Err(MeasurementRefusal::Unavailable));
    assert_eq!(report.packet_bytes(), original_packet_bytes);
    assert_eq!(run.work.calls, [1, 0, 1, 0]);
    assert_eq!(run.work.after.end, Some(End::Completed));
    let newer: &SourceConfigurationSnapshot =
        run.work.replacement.as_ref().ok_or(Failure::Control)?;
    assert!(!newer.same_configuration(report.configuration()));
    assert_eq!(newer.declared_scale(), SourceScale::Unknown);
    Ok(())
}

/// Delivery positions relative to an actual source-issued Protected result.
#[derive(Clone, Copy)]
enum PowerEventTiming {
    /// Retain ordinary joined behavior without event delivery.
    NoEvent,
    /// Deliver after protected adapter retirement, before the actual reply commit.
    BeforeReplyCommit,
    /// Deliver after the actual joined take, while closing the observer.
    AfterJoin,
}

/// Follow the observer's original Stop clone through the real managed worker and report.
///
/// # Errors
/// Preserve source-issued setup and genuine worker or observation failure.
/// # Panics
/// Reject late rewriting, false completion, lost close code or an uncharged send.
#[test]
fn managed_protected_power_event_respects_reply_commit_and_join() -> Result<(), Failure> {
    static SLOTS: [WorkerSlot; 3] = [const { WorkerSlot::new() }; 3];
    static ADMISSIONS: [ProcessAdmission; 3] = [const { ProcessAdmission::new() }; 3];
    let timings: [PowerEventTiming; 3] = [
        PowerEventTiming::NoEvent,
        PowerEventTiming::BeforeReplyCommit,
        PowerEventTiming::AfterJoin,
    ];
    let close_error: NonZeroU32 = NonZeroU32::new(5).ok_or(Failure::Control)?;
    for ((slot, admission), timing) in SLOTS.iter().zip(&ADMISSIONS).zip(timings) {
        let fixture: Issued = issued(Scenario::ProtectedAfterCommit)?;
        let expected: SourceConfigurationSnapshot = fixture.configuration.clone();
        let config: Config = Config::from_managed_protected(
            fixture
                .authority
                .authorize_protected_ke(&expected)
                .map_err(Failure::SourcePolicy)?,
        );
        let stop_at_commit: Cell<Option<StopHandle>> = Cell::new(None);
        let close_state: Cell<Option<Observation>> = Cell::new(None);
        let late_stop: Cell<Option<Result<StopDisposition, Failure>>> = Cell::new(None);
        let (observed, receiver): (SyncSender<Work>, Receiver<Work>) = sync_channel(1);
        let result: Result<SourceProtectedProbeReport, SourceProtectedProbeFailure> =
            SourceProtectedProbeReport::from_observed(run_observing(
                config,
                slot,
                |stop: StopHandle| {
                    if matches!(timing, PowerEventTiming::BeforeReplyCommit) {
                        stop_at_commit.set(Some(stop.clone()));
                    }
                    let closed: &Cell<Option<Observation>> = &close_state;
                    let late_stop: &Cell<Option<Result<StopDisposition, Failure>>> = &late_stop;
                    Ok(move || {
                        closed.set(Some(slot.observer().poll_until(Instant::now() + WAIT)));
                        if matches!(timing, PowerEventTiming::AfterJoin) {
                            late_stop.set(Some(stop.request_stop()));
                        }
                        match timing {
                            PowerEventTiming::NoEvent => Ok(()),
                            PowerEventTiming::BeforeReplyCommit | PowerEventTiming::AfterJoin => {
                                Err(close_error)
                            }
                        }
                    })
                },
                |reservation: SubmissionReservation| {
                    let producer: PreviewProducer = PreviewProducer {
                        authority: fixture.authority,
                        configuration: fixture.configuration,
                        target: fixture.target,
                        scenario: Scenario::ProtectedAfterCommit,
                        stop_before_reply_commit: stop_at_commit.take(),
                        replacement: None,
                        calls: [0; 4],
                    };
                    reservation.submit_fixture(
                        admission,
                        component_provider,
                        move |config: &Config, provider: &ProviderScope<'_>, control: Control| {
                            component_operation(config, provider, control, producer, &observed)
                        },
                        Hooks::none(),
                    )
                },
            ));
        drain(slot, slot.observer().poll_until(Instant::now() + WAIT))?;
        assert_eq!(close_state.get(), Some(Observation::Taken));
        assert_eq!(
            late_stop.get(),
            matches!(timing, PowerEventTiming::AfterJoin).then_some(Ok(StopDisposition::Latched))
        );
        let work: Work = receiver.recv_timeout(WAIT).map_err(|_| Failure::Control)?;
        assert_eq!(work.calls, [1, 0, 1, 0]);
        assert_eq!(work.provider_lifetime.strong_count(), 0);
        let policy: SourcePolicySnapshot = fixture
            .source
            .policy_snapshot(fixture.allowed)
            .map_err(Failure::SourcePolicy)?;
        assert!(policy.source().floor().is_some() && policy.endpoint().floor().is_some());
        if matches!(timing, PowerEventTiming::BeforeReplyCommit) {
            let Err(refusal) = result else {
                return Err(Failure::Control);
            };
            assert_eq!(refusal.labels(), ("currentness", "cancelled"));
            assert_eq!(refusal.power_unregistration_error(), Some(close_error));
            assert!(!refusal.power_stop_failed());
            assert!(refusal.retained_configuration().is_none());
        } else {
            let report: SourceProtectedProbeReport = result.map_err(|_| Failure::Control)?;
            assert!(report.configuration().same_configuration(&expected));
            assert_eq!(report.measurement(), Err(MeasurementRefusal::Unavailable));
            assert_eq!(
                report.power_unregistration_error(),
                matches!(timing, PowerEventTiming::AfterJoin).then_some(close_error)
            );
            let replacement: &SourceConfigurationSnapshot =
                work.replacement.as_ref().ok_or(Failure::Control)?;
            assert!(!replacement.same_configuration(report.configuration()));
        }
    }
    Ok(())
}

/// Preserve exact declared numerical outcomes beside the original managed history.
/// # Errors
/// Preserve actual issuer, worker, packet or checked-profile refusal.
/// # Panics
/// Reject a substituted numerical result, changed age or widened declared dates.
#[test]
fn managed_protected_report_preserves_declared_numerical_result() -> Result<(), Failure> {
    static BROAD_SLOT: WorkerSlot = WorkerSlot::new();
    static BROAD_ADMISSION: ProcessAdmission = ProcessAdmission::new();
    static NARROW_SLOT: WorkerSlot = WorkerSlot::new();
    static NARROW_ADMISSION: ProcessAdmission = ProcessAdmission::new();
    for (slot, admission, latest, refusal) in [
        (&BROAD_SLOT, &BROAD_ADMISSION, u64::from(u32::MAX), None),
        (
            &NARROW_SLOT,
            &NARROW_ADMISSION,
            0,
            Some(MeasurementRefusal::DateScope),
        ),
    ] {
        let profile: ExperimentalProfile =
            ExperimentalProfile::new(0, latest, 30_000_000_000).map_err(|_| Failure::Control)?;
        let run: Run = run_with_hooks(
            slot,
            admission,
            Scenario::ProtectedAfterCommit,
            Hooks::none(),
            Some(profile),
        )?;
        let completion: ReceivedCompletion = run
            .outcome
            .map_err(|failure: RunFailed| operation_primary(&failure))?;
        let protected = completion.protected().ok_or(Failure::Control)?;
        let history = protected.history();
        let accepted_at: Instant = history.receipt().accepted_at();
        let committed_at: Instant = history.committed_at();
        assert!(accepted_at <= committed_at && committed_at < run.work.before.cutoff);
        let original: Result<ExperimentalMeasurement, MeasurementRefusal> =
            *protected.measurement();
        assert_eq!(original.as_ref().err().copied(), refusal);
        let [_, _, read_return]: [crate::SoftwareClock; 3] = history.captures();
        let trailing: Instant = read_return.monotonic_after();
        assert!(trailing <= accepted_at);
        let expected_delay: Option<Duration> = committed_at.checked_duration_since(trailing);
        assert!(expected_delay.is_some());
        if let Ok(measurement) = original {
            assert_eq!(
                Some(measurement.acquisition_age()),
                accepted_at.checked_duration_since(trailing)
            );
        }
        let original_packet_bytes: usize = completion.summary().packet_bytes;
        assert!(completion.summary().transmitted);
        let report: SourceProtectedProbeReport = protected_report(completion)?;
        assert_eq!(report.leap_indicator(), 0);
        assert_eq!(report.measurement().copied(), original);
        assert_eq!(report.committed_at(), committed_at);
        assert_eq!(report.receive_to_commit_delay(), expected_delay);
        assert!(report.configuration().same_configuration(&run.expected));
        assert_eq!(report.packet_bytes(), original_packet_bytes);
        assert_eq!(run.work.calls, [1, 0, 1, 0]);
    }
    Ok(())
}

/// A genuine joined result cannot project through the other report mode.
/// # Errors
/// Preserve the original source-owned worker or Preview refusal.
/// # Panics
/// Reject projecting one genuine joined mode as the other.
#[test]
fn managed_preview_cannot_project_a_protected_report() -> Result<(), Failure> {
    static PREVIEW_SLOT: WorkerSlot = WorkerSlot::new();
    static PREVIEW_ADMISSION: ProcessAdmission = ProcessAdmission::new();
    static PROTECTED_SLOT: WorkerSlot = WorkerSlot::new();
    static PROTECTED_ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let preview: Run = run(&PREVIEW_SLOT, &PREVIEW_ADMISSION, Scenario::Healthy)?;
    let completion: ReceivedCompletion = preview
        .outcome
        .map_err(|failure: RunFailed| operation_primary(&failure))?;
    assert!(matches!(
        protected_report(completion),
        Err(Failure::Control)
    ));
    assert_eq!(preview.work.calls, [1, 1, 0, 0]);
    let protected: Run = run(
        &PROTECTED_SLOT,
        &PROTECTED_ADMISSION,
        Scenario::ProtectedAfterCommit,
    )?;
    let completion: ReceivedCompletion = protected
        .outcome
        .map_err(|failure: RunFailed| operation_primary(&failure))?;
    assert!(completion.protected().is_some());
    assert!(completion.summary().transmitted);
    let Err(refusal): Result<SourceKePreviewReport, SourceKePreviewFailure> =
        SourceKePreviewReport::from_run(Ok(completion))
    else {
        return Err(Failure::Control);
    };
    assert_eq!(refusal.labels(), ("currentness", "control"));
    assert_eq!(refusal.primary_labels(), Some(("currentness", "control")));
    assert!(refusal.retained_configuration().is_none());
    assert_eq!(protected.work.calls, [1, 0, 1, 0]);
    Ok(())
}

/// Preserve a genuine Preview's committed history after actual abnormal retirement.
/// # Errors
/// Preserve unexpected operation, observation or absent sealed history.
/// # Panics
/// Reject normal success, absent original history or substitution of the new revision.
fn assert_preview_abnormal_join_history() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let mut hooks: Hooks = Hooks::none();
    hooks.before_retire = Some(Box::new(|| {
        std::panic::resume_unwind(Box::new("managed Preview fixture after record"));
    }));
    let run: Run = run_with_hooks(&SLOT, &ADMISSION, Scenario::ReplaceAfterCommit, hooks, None)?;
    let Err(failure) = run.outcome else {
        return Err(Failure::Control);
    };
    let RunFailed::Abnormal(joined) = &failure else {
        return Err(Failure::Control);
    };
    assert_eq!(joined.termination, Termination::Panicked);
    assert!(!joined.retired);
    assert_eq!(SLOT.sealed_count().map_err(|_| Failure::Control)?, 1);
    assert_eq!(run.work.calls, [1, 1, 0, 0]);
    assert_eq!(run.work.after.end, Some(End::Completed));
    let Err(projection): Result<SourceKePreviewReport, SourceKePreviewFailure> =
        SourceKePreviewReport::from_run(Err(failure))
    else {
        return Err(Failure::Control);
    };
    assert_eq!(projection.labels(), ("worker-termination", "panicked"));
    assert!(projection.primary_labels().is_none());
    assert_eq!(
        projection.retained_configuration().map(
            |original: &SourceConfigurationSnapshot| original.same_configuration(&run.expected)
        ),
        Some(true),
        "managed Preview retains the original configuration after abnormal join"
    );
    let newer: &SourceConfigurationSnapshot =
        run.work.replacement.as_ref().ok_or(Failure::Control)?;
    let current: SourceConfigurationSnapshot = run
        .source
        .configuration_snapshot()
        .map_err(Failure::SourcePolicy)?
        .ok_or(Failure::Control)?;
    assert!(current.same_configuration(newer));
    assert!(!run.expected.same_configuration(&current));
    Ok(())
}

/// Real post-record panics retain only each mode's already committed source revision.
/// # Errors
/// Preserve unexpected operation, observation or absent sealed history.
/// # Panics
/// Reject joined-success projection, revision substitution or invented retirement.
#[test]
fn managed_protected_abnormal_join_preserves_sealed_configuration() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let mut hooks: Hooks = Hooks::none();
    hooks.before_retire = Some(Box::new(|| {
        std::panic::resume_unwind(Box::new("managed protected fixture after record"));
    }));
    let run: Run = run_with_hooks(
        &SLOT,
        &ADMISSION,
        Scenario::ProtectedAfterCommit,
        hooks,
        None,
    )?;
    let Err(failure) = run.outcome else {
        return Err(Failure::Control);
    };
    let RunFailed::Abnormal(joined) = &failure else {
        return Err(Failure::Control);
    };
    assert_eq!(joined.termination, Termination::Panicked);
    assert!(!joined.retired);
    assert_eq!(SLOT.sealed_count().map_err(|_| Failure::Control)?, 1);
    assert_eq!(run.work.calls, [1, 0, 1, 0]);
    assert_eq!(run.work.after.end, Some(End::Completed));
    let projection: SourceProtectedProbeFailure =
        SourceProtectedProbeFailure::from_run_failure(failure);
    assert_eq!(projection.labels(), ("worker-termination", "panicked"));
    assert!(projection.primary_labels().is_none());
    let original: &SourceConfigurationSnapshot = projection
        .retained_configuration()
        .ok_or(Failure::Control)?;
    assert!(original.same_configuration(&run.expected));
    let newer: &SourceConfigurationSnapshot =
        run.work.replacement.as_ref().ok_or(Failure::Control)?;
    let current: SourceConfigurationSnapshot = run
        .source
        .configuration_snapshot()
        .map_err(Failure::SourcePolicy)?
        .ok_or(Failure::Control)?;
    assert!(current.same_configuration(newer));
    assert!(!original.same_configuration(&current));
    assert_preview_abnormal_join_history()
}

/// A replacement with unchanged endpoints refuses the guarded send or final reply.
/// # Errors
/// Preserve unexpected worker or lack of any failure.
/// # Panics
/// Reject admitted stale effects, a lost polling floor or an automatic retry.
#[test]
fn managed_protected_worker_refuses_replacement_at_both_effects() -> Result<(), Failure> {
    static SEND_SLOT: WorkerSlot = WorkerSlot::new();
    static SEND_ADMISSION: ProcessAdmission = ProcessAdmission::new();
    static REPLY_SLOT: WorkerSlot = WorkerSlot::new();
    static REPLY_ADMISSION: ProcessAdmission = ProcessAdmission::new();
    for (slot, admission, scenario) in [
        (&SEND_SLOT, &SEND_ADMISSION, Scenario::ProtectedBeforeSend),
        (
            &REPLY_SLOT,
            &REPLY_ADMISSION,
            Scenario::ProtectedBeforeReplyCommit,
        ),
    ] {
        let run: Run = run(slot, admission, scenario)?;
        let Err(RunFailed::Operation(failure)) = run.outcome else {
            return Err(Failure::Control);
        };
        assert_eq!(
            failure.primary,
            Failure::SourcePolicy(SourcePolicyError::ConfigurationChanged)
        );
        assert_eq!(run.work.calls, [1, 0, 1, 0]);
        assert_eq!(run.work.after.end, Some(End::Failure));
        let replacement: &SourceConfigurationSnapshot =
            run.work.replacement.as_ref().ok_or(Failure::Control)?;
        let expected_scale: SourceScale =
            if matches!(scenario, Scenario::ProtectedBeforeReplyCommit) {
                SourceScale::Smeared
            } else {
                SourceScale::Unknown
            };
        assert_eq!(replacement.declared_scale(), expected_scale);
        assert!(!replacement.same_configuration(&run.expected));
        let projected: SourceProtectedProbeFailure =
            SourceProtectedProbeFailure::from_run_failure(RunFailed::Operation(failure));
        assert_eq!(
            projected.source_policy_error(),
            Some(SourcePolicyError::ConfigurationChanged)
        );
        assert!(projected.retained_configuration().is_none());
    }
    Ok(())
}

/// Direct Preview callers cannot convert the managed protected association.
/// # Errors
/// Preserve fixture, publication or genuine managed Preview refusal.
/// # Panics
/// Reject entropy or checkout on the wrong mode and any blanket Preview refusal.
#[test]
fn managed_protected_refuses_preview_downgrade_before_charging() -> Result<(), Failure> {
    let fixture: Issued = issued(Scenario::Healthy)?;
    let protected: Config = Config::from_managed_protected(
        fixture
            .authority
            .authorize_protected_ke(&fixture.configuration)
            .map_err(Failure::SourcePolicy)?,
    );
    with_owner(
        &protected,
        COMPONENT_UDP,
        |owner: &mut AssociationOwner<'_>, _control: &Control| {
            let before: Status = owner.status();
            let mut entropy_calls: usize = 0;
            assert!(matches!(
                owner.prepare_preview(|_: &mut [u8; 48]| {
                    entropy_calls += 1;
                    Ok(())
                }),
                Err(Failure::Control)
            ));
            assert_eq!(entropy_calls, 0);
            assert!(matches!(
                owner.preview_owner_for_test(),
                Err(Failure::Control)
            ));
            assert!(matches!(
                owner.reserve(RequestInput::Preview),
                Err(Failure::Control)
            ));
            assert_eq!(owner.status(), before);
            Ok(())
        },
    )?;
    let preview: Config = Config::from_managed_preview(
        fixture
            .source
            .capture_ke_preview()
            .map_err(Failure::SourcePolicy)?,
    );
    with_owner(
        &preview,
        COMPONENT_UDP,
        |owner: &mut AssociationOwner<'_>, _control: &Control| {
            let prepared: super::handoff::PreparedRequest<'_> =
                owner.prepare_preview(|output: &mut [u8; 48]| {
                    output.fill(0x42);
                    Ok(())
                })?;
            let history: ManagedPreviewHistory = prepared.finish_managed()?;
            assert!(
                history
                    .configuration()
                    .same_configuration(&fixture.configuration)
            );
            assert!(!history.summary().transmitted);
            Ok(())
        },
    )
}

/// Replacing the real definition during entropy refuses final publication without refunds.
/// # Errors
/// Preserve actual setup/worker refusal or absence of the intended operation failure.
/// # Panics
/// Reject substituted failure causes, lost charges or fabricated replacement history.
#[test]
fn managed_ke_preview_worker_entropy_replacement_retains_failure_and_charges() -> Result<(), Failure>
{
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let run: Run = run(&SLOT, &ADMISSION, Scenario::ReplaceDuringEntropy)?;
    let Err(RunFailed::Operation(failure)) = &run.outcome else {
        return Err(Failure::Control);
    };
    assert_eq!(
        failure.primary,
        Failure::SourcePolicy(SourcePolicyError::ConfigurationChanged)
    );
    assert!(failure.cleanup.is_none() && failure.admission_cleanup.is_none());
    assert_eq!(run.work.last_failure, Some(*failure));
    let Err(refusal): Result<SourceKePreviewReport, SourceKePreviewFailure> =
        SourceKePreviewReport::from_run(run.outcome)
    else {
        return Err(Failure::Control);
    };
    assert_eq!(
        refusal.source_policy_error(),
        Some(SourcePolicyError::ConfigurationChanged)
    );
    assert_eq!(refusal.labels(), ("source-policy", "refused"));
    assert_eq!(refusal.primary_labels(), Some(("source-policy", "refused")));
    assert!(refusal.cleanup_labels().is_none());
    assert!(refusal.admission_cleanup_labels().is_none());
    assert!(refusal.retained_configuration().is_none());
    assert_eq!(
        format!("{refusal:?}"),
        r#"SourceKePreviewFailure { stage: "source-policy", cause: "refused", .. }"#
    );
    assert_eq!(
        refusal.to_string(),
        "source KE Preview refused: source-policy/refused"
    );
    assert_eq!(run.work.calls, [1, 1, 0, 0]);
    assert_eq!(
        (
            run.work.after.transactions,
            run.work.after.refusals,
            run.work.after.observations,
            run.work.after.contexts
        ),
        (1, 0, 2, 1)
    );
    assert_eq!(run.work.after.end, Some(End::Failure));
    let replacement: &SourceConfigurationSnapshot =
        run.work.replacement.as_ref().ok_or(Failure::Control)?;
    assert!(!replacement.same_configuration(&run.expected));
    assert!(
        run.source
            .configuration_snapshot()
            .map_err(Failure::SourcePolicy)?
            .as_ref()
            .is_some_and(
                |actual: &SourceConfigurationSnapshot| actual.same_configuration(replacement)
            )
    );
    Ok(())
}

/// A genuine association outside the issued UDP set refuses before Preview entropy.
/// # Errors
/// Preserve actual issuer/worker refusal or absence of the intended operation failure.
/// # Panics
/// Reject publication/sealing of the outside endpoint or lost failure/charge facts.
#[test]
fn managed_ke_preview_worker_outside_endpoint_never_enters_sealing() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let run: Run = run(&SLOT, &ADMISSION, Scenario::OutsideEndpoint)?;
    let Err(RunFailed::Operation(failure)) = run.outcome else {
        return Err(Failure::Control);
    };
    assert_eq!(
        failure.primary,
        Failure::SourcePolicy(SourcePolicyError::Foreign)
    );
    assert!(failure.cleanup.is_none() && failure.admission_cleanup.is_none());
    assert_eq!(run.work.last_failure, Some(failure));
    assert_eq!(run.work.calls, [1, 0, 0, 0]);
    assert_eq!(
        (
            run.work.after.transactions,
            run.work.after.refusals,
            run.work.after.observations,
            run.work.after.contexts
        ),
        (1, 0, 2, 0)
    );
    assert_eq!(run.work.after.end, Some(End::Failure));
    Ok(())
}

/// A real replacement after material preparation prevents association publication.
/// # Errors
/// Preserve actual issuer/worker refusal or absence of the intended operation failure.
/// # Panics
/// Reject stale publication, entropy entry, lost work or an unchanged issued revision.
#[test]
fn managed_ke_preview_worker_replacement_before_publication_never_enters_sealing()
-> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let run: Run = run(&SLOT, &ADMISSION, Scenario::ReplaceBeforePublication)?;
    let Err(RunFailed::Operation(failure)) = run.outcome else {
        return Err(Failure::Control);
    };
    assert_eq!(
        failure.primary,
        Failure::SourcePolicy(SourcePolicyError::ConfigurationChanged)
    );
    assert!(failure.cleanup.is_none() && failure.admission_cleanup.is_none());
    assert_eq!(run.work.last_failure, Some(failure));
    assert_eq!(run.work.calls, [1, 0, 0, 0]);
    assert_eq!(
        (
            run.work.after.transactions,
            run.work.after.refusals,
            run.work.after.observations,
            run.work.after.contexts
        ),
        (1, 0, 2, 0)
    );
    assert_eq!(run.work.after.end, Some(End::Failure));
    let replacement: &SourceConfigurationSnapshot =
        run.work.replacement.as_ref().ok_or(Failure::Control)?;
    assert!(!replacement.same_configuration(&run.expected));
    Ok(())
}

/// Replacement before worker publication cannot relabel the subsequent joined history.
/// # Errors
/// Preserve actual issuer, worker, receiver or postcommit replacement refusal.
/// # Panics
/// Reject revised historical context, UDP effects or missing completed state.
#[test]
fn managed_ke_preview_worker_postcommit_replacement_preserves_original_history()
-> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let run: Run = run(&SLOT, &ADMISSION, Scenario::ReplaceAfterCommit)?;
    assert!(run.outcome.is_ok());
    let completion: ReceivedCompletion = run
        .outcome
        .map_err(|failure: RunFailed| operation_primary(&failure))?;
    let newer: &SourceConfigurationSnapshot =
        run.work.replacement.as_ref().ok_or(Failure::Control)?;
    assert!(!newer.same_configuration(&run.expected));
    assert_history(completion, &run.expected, (1, 12, 140))?;
    assert!(
        run.source
            .capture_ke_preview()
            .map_err(Failure::SourcePolicy)?
            .snapshot()
            .same_configuration(newer)
    );
    assert_eq!(run.work.calls, [1, 1, 0, 0]);
    assert_eq!(run.work.after.end, Some(End::Completed));
    Ok(())
}

/// Direct managed protected-entry attempts refuse before adapter setup or ledger spending.
/// # Errors
/// Preserve actual checked issuance, publication and legal input construction refusal.
/// # Panics
/// Reject a bypass through either protected representation or any pre-refusal debit.
#[test]
fn managed_ke_preview_direct_protected_entries_refuse_without_spending() -> Result<(), Failure> {
    let fixture: Issued = issued(Scenario::Healthy)?;
    let config: Config = Config::from_managed_preview(
        fixture
            .source
            .capture_ke_preview()
            .map_err(Failure::SourcePolicy)?,
    );
    with_owner(
        &config,
        COMPONENT_UDP,
        |owner: &mut AssociationOwner<'_>, control: &Control| {
            let before: Status = owner.status();
            assert_eq!(before.phase, RequestPhase::Idle);
            assert_eq!(
                (
                    before.attempts,
                    before.frames,
                    before.bytes,
                    before.available,
                    before.used,
                    before.uids
                ),
                (32, 32, 131_072, 1, 0, 0)
            );
            assert_eq!(owner.check_context(control), Err(Failure::Control));
            assert_eq!(owner.status(), before);
            assert!(matches!(
                owner.reserve(RequestInput::Preview),
                Err(Failure::Control)
            ));
            assert_eq!(owner.status(), before);
            let inputs: [RequestInput; 2] = protected_inputs()?;
            for input in inputs {
                assert!(matches!(owner.reserve(input), Err(Failure::Control)));
                assert_eq!(owner.status(), before);
            }
            Ok(())
        },
    )?;
    assert!(matches!(
        fixture.source.policy_snapshot(fixture.allowed),
        Err(SourcePolicyError::UnknownEndpoint)
    ));
    Ok(())
}

/// Existing diagnostic Preview and explicit protected mode remain legal direct neighbors.
/// # Errors
/// Preserve checked diagnostic configuration and exercised real owner/ledger refusals.
/// # Panics
/// Reject a blanket protected-entry ban or an altered finite checkout charge.
#[test]
fn managed_ke_preview_direct_protected_entries_keep_diagnostic_neighbors() -> Result<(), Failure> {
    let preview: Config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    assert_protected_neighbor(&preview)?;
    let arguments: [OsString; 3] = [
        OsString::from("127.0.0.1:4460"),
        OsString::from("Time.Example"),
        OsString::from("--protected-datagram"),
    ];
    let protected: Config = Config::from_arguments(arguments.into_iter())?;
    assert_protected_neighbor(&protected)
}

/// Exercise same-Control attachment and the distinct managed and diagnostic finalizers.
/// # Errors
/// Preserve actual issuer, publication and preparation refusals before finalization.
/// # Panics
/// Reject foreign attachment, diagnostic publication of managed work or a blanket finalizer ban.
#[test]
fn managed_ke_preview_ordinary_finish_refuses_with_diagnostic_neighbor() -> Result<(), Failure> {
    let fixture: Issued = issued(Scenario::Healthy)?;
    let managed: Config = Config::from_managed_preview(
        fixture
            .source
            .capture_ke_preview()
            .map_err(Failure::SourcePolicy)?,
    );
    with_owner(
        &managed,
        COMPONENT_UDP,
        |owner: &mut AssociationOwner<'_>, control: &Control| {
            let raw_keys: DirectionalKeys =
                DirectionalKeys::new(Zeroizing::new([0x11_u8; 32]), Zeroizing::new([0x22_u8; 32]));
            let raw_cookies: Vec<Zeroizing<Vec<u8>>> = vec![Zeroizing::new(vec![0xa5_u8; 12])];
            let raw: super::handoff::PreparedRequest<'_> = super::handoff::prepare(
                control,
                &raw_keys,
                &raw_cookies,
                |bytes: &mut [u8; 48]| {
                    bytes.fill(0x42_u8);
                    Ok(())
                },
            )?;
            assert!(matches!(raw.finish(), Err(Failure::Control)));

            let foreign_control: Control = Control::new(control.origin)?;
            foreign_control.publish_ke()?;
            let before: Status = owner.status();
            let attachments: [(&Control, bool); 2] = [(&foreign_control, false), (control, true)];
            for (packet_control, same_owner) in attachments {
                let raw: super::handoff::PreparedRequest<'_> = super::handoff::prepare(
                    packet_control,
                    &raw_keys,
                    &raw_cookies,
                    |bytes: &mut [u8; 48]| {
                        bytes.fill(0x42_u8);
                        Ok(())
                    },
                )?;
                let origin: super::association::PreviewOwner<'_> =
                    owner.preview_owner_for_test()?;
                assert!(matches!(
                    (same_owner, raw.bind_owner(origin)),
                    (false, Err(Failure::Control)) | (true, Ok(_))
                ));
            }
            assert_eq!(owner.status(), before);
            assert_eq!(foreign_control.with_published(|_| Ok(())), Ok(()));
            assert_eq!(control.with_published(|_| Ok(())), Ok(()));

            let prepared: super::handoff::PreparedRequest<'_> =
                owner.prepare_preview(|bytes: &mut [u8; 48]| {
                    bytes.fill(0x42_u8);
                    Ok(())
                })?;
            assert!(matches!(prepared.finish(), Err(Failure::Control)));
            assert_eq!(control.with_published(|_| Ok(())), Ok(()));
            Ok(())
        },
    )?;
    let diagnostic: Config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    with_owner(
        &diagnostic,
        COMPONENT_UDP,
        |owner: &mut AssociationOwner<'_>, control: &Control| {
            let duplicate: super::association::PreviewOwner<'_> = owner.preview_owner_for_test()?;
            let prepared: super::handoff::PreparedRequest<'_> =
                owner.prepare_preview(|bytes: &mut [u8; 48]| {
                    bytes.fill(0x42_u8);
                    Ok(())
                })?;
            assert!(matches!(
                prepared.bind_owner(duplicate),
                Err(Failure::Control)
            ));
            assert_eq!(control.with_published(|_| Ok(())), Ok(()));
            Ok(())
        },
    )?;
    with_owner(
        &diagnostic,
        COMPONENT_UDP,
        |owner: &mut AssociationOwner<'_>, _control: &Control| {
            let prepared: super::handoff::PreparedRequest<'_> =
                owner.prepare_preview(|bytes: &mut [u8; 48]| {
                    bytes.fill(0x42_u8);
                    Ok(())
                })?;
            let summary: Summary = prepared.finish()?;
            assert_eq!(
                (summary.cookies, summary.cookie_bytes, summary.packet_bytes),
                (1, 12, 140)
            );
            assert!(!summary.transmitted);
            Ok(())
        },
    )
}

/// Select when the same checked alias request delivers its restriction.
#[derive(Clone, Copy)]
enum AliasRestrictionTiming {
    /// Restrict the endpoint after preparation and before final admission.
    BeforeCommit,
    /// Restrict the endpoint after the actual final admission.
    AfterCommit,
}

/// One actual prepared Preview observes only its selected endpoint at final commit.
/// # Errors
/// Preserve genuine policy/association setup or healthy completion failures.
/// # Panics
/// Fail on a restriction bypass, transitive refusal, renewed budget or rewritten history.
fn check_managed_preview_alias_restriction(
    code: u32,
    udp: SocketAddr,
    timing: AliasRestrictionTiming,
) -> Result<(), Failure> {
    let interval: Duration = Duration::from_millis(1);
    let limits: SourcePolicyLimits =
        SourcePolicyLimits::new(2, 1, interval, Duration::from_millis(4))
            .map_err(Failure::SourcePolicy)?;
    let mut authority: SourcePolicyAuthority =
        crate::source_policy::isolated_authority(limits).map_err(Failure::SourcePolicy)?;
    let source: SourceHandle = authority.source(0).map_err(Failure::SourcePolicy)?;
    let alias: SourceHandle = authority.source(1).map_err(Failure::SourcePolicy)?;
    let tls: SocketAddr = SocketAddr::from((Ipv4Addr::LOCALHOST, 4460));
    let target: KePreviewTarget = KePreviewTarget::new(tls, "Time.Example")
        .map_err(|_: KePreviewTargetError| Failure::Arguments)?;
    let addresses: crate::SourceAddressSet =
        crate::SourceAddressSet::new(&[COMPONENT_UDP, ALTERNATE_UDP])
            .map_err(Failure::SourcePolicy)?;
    let original: SourceConfigurationSnapshot = authority
        .install_ke_preview_configuration(&source, addresses, target)
        .map_err(Failure::SourcePolicy)?;
    let alias_addresses: crate::SourceAddressSet =
        crate::SourceAddressSet::new(&[COMPONENT_UDP]).map_err(Failure::SourcePolicy)?;
    let _alias_configuration: SourceConfigurationSnapshot = authority
        .install_source_configuration(&alias, alias_addresses)
        .map_err(Failure::SourcePolicy)?;
    let binding: SourceBinding = alias.bind(COMPONENT_UDP).map_err(Failure::SourcePolicy)?;
    let now: Instant = Instant::now();
    let floor: Instant = now.checked_add(interval).ok_or(Failure::Deadline)?;
    let request: Arc<()> = Arc::new(());
    let mut pending: FixtureRequest =
        fixture_send(&binding, &request, now).map_err(Failure::SourcePolicy)?;
    let baseline: SourcePolicySnapshot = source
        .policy_snapshot(COMPONENT_UDP)
        .map_err(Failure::SourcePolicy)?;
    assert_eq!(baseline.endpoint().floor(), Some(floor));
    assert!(!baseline.endpoint().refused());

    let config: Config =
        Config::from_managed_preview(source.capture_ke_preview().map_err(Failure::SourcePolicy)?);
    with_owner(
        &config,
        udp,
        |owner: &mut AssociationOwner<'_>, control: &Control| {
            let prepared: super::handoff::PreparedRequest<'_> =
                owner.prepare_preview(|bytes: &mut [u8; 48]| {
                    bytes.fill(0x42_u8);
                    Ok(())
                })?;
            let charged: Status = owner.status();
            assert_eq!(
                (charged.attempts, charged.available, charged.used),
                (31, 1, 0)
            );
            let before_commit: bool = matches!(timing, AliasRestrictionTiming::BeforeCommit);
            if before_commit {
                fixture_control(&binding, &mut pending, code, now)
                    .map_err(Failure::SourcePolicy)?;
            }
            let result: Result<ManagedPreviewHistory, Failure> = prepared.finish_managed();
            if !before_commit {
                fixture_control(&binding, &mut pending, code, now)
                    .map_err(Failure::SourcePolicy)?;
            }
            if before_commit && udp == COMPONENT_UDP {
                assert!(
                    matches!(
                        result,
                        Err(Failure::SourcePolicy(SourcePolicyError::Disabled))
                    ),
                    "current shared-endpoint restriction must veto Preview commit"
                );
                assert_eq!(control.with_published(|_| Ok(())), Ok(()));
            } else {
                let history: ManagedPreviewHistory = result?;
                let summary: &Summary = history.summary();
                assert!(history.configuration().same_configuration(&original));
                assert_eq!(
                    (summary.cookies, summary.cookie_bytes, summary.packet_bytes),
                    (1, 12, 140)
                );
                assert!(!summary.transmitted);
                assert_eq!(control.with_published(|_| Ok(())), Err(Failure::Control));
            }
            assert_eq!(owner.status(), charged);
            Ok(())
        },
    )?;

    assert_alias_restriction_state(
        &source,
        &binding,
        &original,
        baseline.source().revision(),
        floor,
    )
}

/// Verify the source and shared endpoint records after the alias control.
/// # Errors
/// Preserve actual snapshot refusal.
/// # Panics
/// Fail on transitive source refusal, shortened floor or a new independent slot.
fn assert_alias_restriction_state(
    source: &SourceHandle,
    binding: &SourceBinding,
    original: &SourceConfigurationSnapshot,
    original_source_revision: u64,
    floor: Instant,
) -> Result<(), Failure> {
    let current: SourceConfigurationSnapshot = source
        .configuration_snapshot()
        .map_err(Failure::SourcePolicy)?
        .ok_or(Failure::Control)?;
    assert!(current.same_configuration(original));
    let policy: SourcePolicySnapshot = source
        .policy_snapshot(COMPONENT_UDP)
        .map_err(Failure::SourcePolicy)?;
    assert!(!policy.source().refused());
    assert_eq!(policy.source().floor(), None);
    assert_eq!(policy.source().revision(), original_source_revision);
    assert!(policy.endpoint().refused());
    assert_eq!(policy.endpoint().floor(), Some(floor));
    let alias_policy: SourcePolicySnapshot =
        binding.policy_snapshot().map_err(Failure::SourcePolicy)?;
    assert!(alias_policy.source().refused());
    assert_eq!(alias_policy.source().floor(), Some(floor));
    assert!(matches!(
        source.policy_snapshot(ALTERNATE_UDP),
        Err(SourcePolicyError::UnknownEndpoint)
    ));
    Ok(())
}

/// Exercise both restriction codes against both selected endpoints on either side of commit.
/// # Errors
/// Preserve the first unsuccessful component history.
#[test]
fn managed_ke_preview_alias_endpoint_restriction_preserves_commit_history() -> Result<(), Failure> {
    let codes: [u32; 2] = [u32::from_be_bytes(*b"DENY"), u32::from_be_bytes(*b"RSTR")];
    let timings: [AliasRestrictionTiming; 2] = [
        AliasRestrictionTiming::BeforeCommit,
        AliasRestrictionTiming::AfterCommit,
    ];
    for code in codes {
        for udp in [COMPONENT_UDP, ALTERNATE_UDP] {
            for timing in timings {
                check_managed_preview_alias_restriction(code, udp, timing)?;
            }
        }
    }
    Ok(())
}
