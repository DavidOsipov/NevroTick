// SPDX-License-Identifier: GPL-2.0-only
//! Restore bounded transport causes without losing an earlier TLS refusal.

use std::io;

use super::Failure;

/// Actual I/O projection site; it selects both native failure and public label.
#[cfg(any(windows, test))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TlsIoStage {
    /// Initial TLS completion.
    Handshake,
    /// KE request write or flush.
    Request,
    /// KE response or already-buffered plaintext read.
    Response,
    /// Local close-notify write or flush.
    Shutdown,
}

#[cfg(any(windows, test))]
impl TlsIoStage {
    /// Map every typed stage to its existing fixed CLI field, without I/O.
    const fn label(self) -> &'static str {
        match self {
            Self::Handshake => "handshake",
            Self::Request => "request",
            Self::Response => "response",
            Self::Shutdown => "shutdown",
        }
    }

    /// Preserve ordinary native I/O categories at the same actual stage.
    #[cfg(windows)]
    const fn native(self, kind: io::ErrorKind) -> Failure {
        match self {
            Self::Handshake => Failure::Handshake(kind),
            Self::Request => Failure::Request(kind),
            Self::Response => Failure::Response(kind),
            Self::Shutdown => Failure::Shutdown(kind),
        }
    }
}

/// Closed certificate facts; context names, dates, OIDs and text never survive.
#[cfg(any(windows, test))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CertificateCause {
    /// The peer supplied no certificate.
    Missing,
    /// Certificate encoding was rejected.
    Encoding,
    /// The verifier reported expiry; this supplies no independent true date.
    Expired,
    /// The verifier reported that validity has not begun.
    NotYetValid,
    /// The certificate name does not match the requested identity.
    Name,
    /// The issuer is not accepted by the actual verifier.
    Issuer,
    /// The verifier reported revocation.
    Revoked,
    /// The certificate's permitted usage does not cover this operation.
    Purpose,
    /// A certificate or handshake signature was rejected.
    Signature,
    /// The typed certificate refusal has an opaque or unrecognized reason.
    Unclassified,
}

/// Closed facts, not retry eligibility, peer fault or certificate-date authority.
#[cfg(any(windows, test))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TlsCause {
    /// A recognized certificate-verification refusal, with all context discarded.
    Certificate(CertificateCause),
    /// A recognized TLS protocol or received-record refusal.
    Protocol,
    /// A peer alert was reported; neither its payload nor a retry rule is retained.
    PeerAlert,
    /// Rustls explicitly reported unavailable current time.
    LocalClock,
    /// Rustls explicitly reported unavailable random bytes.
    LocalEntropy,
    /// An opaque or unrecognized typed TLS cause remains unavailable to policy.
    Unclassified,
}

/// Provenance from the pinned direct Rustls I/O wrapper, never an I/O kind alone.
/// Fields stay private; only the actual projector constructs production values.
#[cfg(any(windows, test))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TlsFailure {
    /// Actual receiving site; cannot disagree with its native-error conversion.
    stage: TlsIoStage,
    /// Fixed family only; no library payload, name, certificate or date survives.
    cause: TlsCause,
}

#[cfg(any(windows, test))]
impl TlsFailure {
    /// Retain existing fixed CLI labels, independently of the internal cause.
    pub(super) const fn labels(self) -> (&'static str, &'static str) {
        (self.stage.label(), "invalid-data")
    }
}

/// Exact local setup site; CNG initialization/panic policy remains independent.
#[cfg(any(windows, test))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SetupStage {
    /// The actual provider's suite inventory differs from the admitted profile.
    ProviderSuites,
    /// Ordinary platform-verifier construction failed.
    Verifier,
    /// TLS protocol-version configuration failed.
    ProtocolVersions,
    /// Client-connection construction failed.
    ClientConnection,
}

/// Borrow and project only known typed families; never format or follow sources.
/// Recognized contexts lose all fields; opaque/future reasons stay unclassified.
#[cfg(windows)]
fn classify_tls(error: &rustls::Error) -> TlsCause {
    use rustls::{CertificateError, Error};
    match error {
        Error::InvalidCertificate(certificate) => {
            TlsCause::Certificate(match certificate {
                CertificateError::BadEncoding => CertificateCause::Encoding,
                CertificateError::Expired | CertificateError::ExpiredContext { .. } => {
                    CertificateCause::Expired
                }
                CertificateError::NotValidYet | CertificateError::NotValidYetContext { .. } => {
                    CertificateCause::NotYetValid
                }
                CertificateError::Revoked => CertificateCause::Revoked,
                CertificateError::UnknownIssuer => CertificateCause::Issuer,
                CertificateError::BadSignature => CertificateCause::Signature,
                CertificateError::NotValidForName
                | CertificateError::NotValidForNameContext { .. } => CertificateCause::Name,
                CertificateError::InvalidPurpose
                | CertificateError::InvalidPurposeContext { .. } => CertificateCause::Purpose,
                _ => CertificateCause::Unclassified,
            })
        }
        Error::NoCertificatesPresented => TlsCause::Certificate(CertificateCause::Missing),
        Error::InappropriateMessage { .. }
        | Error::InappropriateHandshakeMessage { .. }
        | Error::InvalidMessage(_)
        | Error::PeerIncompatible(_)
        | Error::PeerMisbehaved(_)
        | Error::PeerSentOversizedRecord
        | Error::NoApplicationProtocol
        | Error::DecryptError => TlsCause::Protocol,
        Error::AlertReceived(_) => TlsCause::PeerAlert,
        Error::FailedToGetCurrentTime => TlsCause::LocalClock,
        Error::FailedToGetRandomBytes => TlsCause::LocalEntropy,
        _ => TlsCause::Unclassified,
    }
}

/// Project the pinned direct TLS wrapper before discarding the I/O error.
/// Bare/wrapped-other-source/wrong-kind input preserves only its native category.
/// No allocation, formatting, source traversal, effect or retry authorization.
#[cfg(windows)]
pub(super) fn project_io(stage: TlsIoStage, error: &io::Error) -> Failure {
    if error.kind() == io::ErrorKind::InvalidData
        && let Some(tls) = error
            .get_ref()
            .and_then(|source| source.downcast_ref::<rustls::Error>())
    {
        return Failure::Tls(TlsFailure {
            stage,
            cause: classify_tls(tls),
        });
    }
    stage.native(error.kind())
}

/// Preserve a response reader's actual error without granting TLS provenance.
/// Windows recognizes only the pinned direct typed wrapper; portable readers
/// retain their native category without importing a TLS implementation.
pub(super) fn response_error(error: &io::Error) -> Failure {
    #[cfg(windows)]
    {
        project_io(TlsIoStage::Response, error)
    }
    #[cfg(not(windows))]
    {
        Failure::Response(error.kind())
    }
}

/// The first transport refusal survives libraries that retry or suppress errors.
#[derive(Clone, Copy)]
pub(super) enum TransportFault {
    /// The original monotonic/cancellation guard closed this job.
    Control(Failure),
    /// One native operation failed; no later native operation is admitted.
    Io(io::ErrorKind),
}

/// Resolve a library result against the independently retained transport fault.
///
/// # Errors
/// Preserve a genuine typed TLS refusal before its last-gasp alert failure.
/// An I/O kind alone proves no TLS precedence. Otherwise restore the first
/// fault, including faults obscured by retries or nominal library success.
pub(super) fn resolve<T>(
    fault: Option<TransportFault>,
    result: Result<T, Failure>,
    stage: fn(io::ErrorKind) -> Failure,
) -> Result<T, Failure> {
    #[cfg(any(windows, test))]
    if matches!(result, Err(Failure::Tls(_))) {
        return result;
    }
    match fault {
        Some(TransportFault::Control(failure)) => Err(failure),
        Some(TransportFault::Io(kind)) => Err(stage(kind)),
        None => result,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CertificateCause, Failure, SetupStage, TlsCause, TlsFailure, TlsIoStage, TransportFault,
        io, resolve,
    };

    /// An untyped I/O category cannot prove that TLS failed before cancellation.
    ///
    /// # Panics
    /// Panics if bare invalid-data input masks the independently retained stop.
    #[test]
    fn bare_invalid_data_does_not_claim_tls_precedence() {
        let result: Result<(), Failure> = Err(Failure::Response(io::ErrorKind::InvalidData));
        assert_eq!(
            resolve(
                Some(TransportFault::Control(Failure::Cancelled)),
                result,
                Failure::Response,
            ),
            Err(Failure::Cancelled),
        );
    }

    /// A known protocol refusal precedes a failing best-effort TLS alert write.
    ///
    /// # Panics
    /// Panics if alert I/O replaces the earlier TLS invalid-data category.
    #[test]
    fn tls_refusal_precedes_alert_write_failure() {
        let result: Result<(), Failure> = Err(Failure::Tls(TlsFailure {
            stage: TlsIoStage::Response,
            cause: TlsCause::Protocol,
        }));
        assert_eq!(
            resolve(
                Some(TransportFault::Io(io::ErrorKind::BrokenPipe)),
                result,
                Failure::Response,
            ),
            result,
        );
    }

    /// A deadline reached during the alert cannot replace its earlier TLS cause.
    ///
    /// # Panics
    /// Panics if a later alert deadline replaces the primary TLS refusal.
    #[test]
    fn tls_refusal_precedes_alert_deadline() {
        let result: Result<(), Failure> = Err(Failure::Tls(TlsFailure {
            stage: TlsIoStage::Response,
            cause: TlsCause::Protocol,
        }));
        assert_eq!(
            resolve(
                Some(TransportFault::Control(Failure::Deadline)),
                result,
                Failure::Response,
            ),
            result,
        );
    }

    /// Nominal library success cannot hide any independently observed fault.
    ///
    /// # Panics
    /// Panics if nominal success escapes a retained transport/control refusal.
    #[test]
    fn suppressed_fault_never_becomes_success() {
        for (fault, expected) in [
            (
                TransportFault::Io(io::ErrorKind::BrokenPipe),
                Failure::Response(io::ErrorKind::BrokenPipe),
            ),
            (
                TransportFault::Control(Failure::Cancelled),
                Failure::Cancelled,
            ),
            (
                TransportFault::Control(Failure::Deadline),
                Failure::Deadline,
            ),
        ] {
            assert_eq!(
                resolve(Some(fault), Ok(7_u8), Failure::Response),
                Err(expected)
            );
        }
    }

    /// A library retry's generic error still restores the actual first refusal.
    ///
    /// # Panics
    /// Panics if retry-obscured errors lose the original cancellation/I/O cause.
    #[test]
    fn retry_obscured_fault_keeps_original_category() {
        for kind in [io::ErrorKind::Other, io::ErrorKind::Interrupted] {
            let result: Result<(), Failure> = Err(Failure::Response(kind));
            assert_eq!(
                resolve(
                    Some(TransportFault::Control(Failure::Cancelled)),
                    result,
                    Failure::Response,
                ),
                Err(Failure::Cancelled),
            );
            assert_eq!(
                resolve(
                    Some(TransportFault::Io(io::ErrorKind::TimedOut)),
                    result,
                    Failure::Response,
                ),
                Err(Failure::Response(io::ErrorKind::TimedOut)),
            );
        }
    }

    /// Without an independent fault, neither success nor rejection is rewritten.
    ///
    /// # Panics
    /// Panics if an unfaulted success or original failure is changed.
    #[test]
    fn unfaulted_results_are_preserved() {
        assert_eq!(resolve(None, Ok(7_u8), Failure::Response), Ok(7));
        for error in [
            Failure::Cookies,
            Failure::Response(io::ErrorKind::InvalidData),
        ] {
            let result: Result<(), Failure> = Err(error);
            assert_eq!(resolve(None, result, Failure::Response), result);
        }
    }

    /// Unknown typed TLS reasons retain provenance without becoming retry rules.
    /// # Panics
    /// Fail if a later alert refusal masks genuine but unclassified TLS evidence.
    #[test]
    fn typed_unclassified_cause_retains_primary_precedence() {
        let result: Result<(), Failure> = Err(Failure::Tls(TlsFailure {
            stage: TlsIoStage::Response,
            cause: TlsCause::Unclassified,
        }));
        for fault in [
            TransportFault::Control(Failure::Cancelled),
            TransportFault::Control(Failure::Deadline),
            TransportFault::Io(io::ErrorKind::BrokenPipe),
        ] {
            assert_eq!(resolve(Some(fault), result, Failure::Response), result);
        }
    }

    /// Frozen pre-existing public I/O labels; unknown categories stay generic.
    const LEGACY_IO_LABELS: [(io::ErrorKind, &str); 7] = [
        (io::ErrorKind::TimedOut, "timeout"),
        (io::ErrorKind::Interrupted, "interrupted"),
        (io::ErrorKind::ConnectionRefused, "refused"),
        (io::ErrorKind::UnexpectedEof, "unexpected-eof"),
        (io::ErrorKind::InvalidData, "invalid-data"),
        (io::ErrorKind::Other, "io"),
        (io::ErrorKind::PermissionDenied, "io"),
    ];

    /// Every legacy portable failure keeps its independently specified CLI code.
    /// These closed error values contain no trusted owner or authenticated state.
    /// # Panics
    /// Fail if an existing category, stage or bounded I/O label changes.
    fn assert_legacy_failure_labels() {
        use super::super::{
            association::ledger::LedgerError, datagram::DatagramError, request::RequestError,
        };
        use ntp_wire::{NtsKeAccumulatorError, NtsKeNegotiationError, NtsKeRequestEncodeError};

        for (failure, expected) in [
            (Failure::Arguments, ("admission", "arguments")),
            (Failure::Allocation, ("resources", "allocation")),
            (Failure::Deadline, ("currentness", "deadline")),
            (Failure::Cancelled, ("currentness", "cancelled")),
            (Failure::Control, ("currentness", "control")),
            (Failure::ProcessUsed, ("admission", "process-used")),
            (Failure::PreviewSpent, ("resources", "preview-spent")),
            (
                Failure::Ledger(LedgerError::Busy),
                ("protected-request", "refused"),
            ),
            (
                Failure::Datagram(DatagramError::Attempts),
                ("protected-datagram", "refused"),
            ),
            (
                Failure::Offer(NtsKeRequestEncodeError::EmptyAeadOffer),
                ("request", "parameters"),
            ),
            (Failure::Limits, ("request", "parameters")),
            (
                Failure::Frame(NtsKeAccumulatorError::StorageTooSmall),
                ("response", "framing"),
            ),
            (
                Failure::Negotiation(NtsKeNegotiationError::MissingCookie),
                ("response", "negotiation"),
            ),
            (Failure::Cookies, ("response", "cookie-budget")),
            (Failure::Entropy, ("request-sealing", "entropy")),
            (
                Failure::Packet(RequestError::EntropyWidth),
                ("request-sealing", "encoding"),
            ),
            (Failure::Endpoint, ("response", "endpoint")),
        ] {
            assert_eq!(super::super::failure_labels(failure), expected);
        }
        for (kind, label) in LEGACY_IO_LABELS {
            assert_eq!(
                super::super::failure_labels(Failure::Response(kind)),
                ("response", label)
            );
        }
    }

    /// Native-only legacy variants retain the same fixed stage and cause fields.
    /// Constructing these bounded errors performs no Windows or provider work.
    /// # Panics
    /// Fail if any native stage, profile or native I/O label changes.
    #[cfg(windows)]
    fn assert_legacy_native_failure_labels() {
        for (failure, expected) in [
            (Failure::Peer, ("connect", "peer-mismatch")),
            (Failure::TlsProfile, ("handshake", "profile")),
            (Failure::Exporter, ("exporter", "export")),
        ] {
            assert_eq!(super::super::failure_labels(failure), expected);
        }
        for (kind, label) in LEGACY_IO_LABELS {
            for (failure, stage) in [
                (Failure::Connect(kind), "connect"),
                (Failure::Handshake(kind), "handshake"),
                (Failure::Request(kind), "request"),
                (Failure::Shutdown(kind), "shutdown"),
            ] {
                assert_eq!(super::super::failure_labels(failure), (stage, label));
            }
        }
    }

    /// Internal typed detail cannot change the established public CLI fields.
    /// # Panics
    /// Fail if any cause/stage changes its fixed, payload-free public projection.
    #[test]
    fn typed_failure_and_setup_labels_remain_compatible() {
        assert_legacy_failure_labels();
        #[cfg(windows)]
        assert_legacy_native_failure_labels();
        let mut causes = Vec::from([
            TlsCause::Protocol,
            TlsCause::PeerAlert,
            TlsCause::LocalClock,
            TlsCause::LocalEntropy,
            TlsCause::Unclassified,
        ]);
        for certificate in [
            CertificateCause::Missing,
            CertificateCause::Encoding,
            CertificateCause::Expired,
            CertificateCause::NotYetValid,
            CertificateCause::Name,
            CertificateCause::Issuer,
            CertificateCause::Revoked,
            CertificateCause::Purpose,
            CertificateCause::Signature,
            CertificateCause::Unclassified,
        ] {
            causes.push(TlsCause::Certificate(certificate));
        }
        for (stage, label) in [
            (TlsIoStage::Handshake, "handshake"),
            (TlsIoStage::Request, "request"),
            (TlsIoStage::Response, "response"),
            (TlsIoStage::Shutdown, "shutdown"),
        ] {
            for cause in &causes {
                assert_eq!(
                    super::super::failure_labels(Failure::Tls(TlsFailure {
                        stage,
                        cause: *cause
                    })),
                    (label, "invalid-data"),
                );
            }
        }
        for stage in [
            SetupStage::ProviderSuites,
            SetupStage::Verifier,
            SetupStage::ProtocolVersions,
            SetupStage::ClientConnection,
        ] {
            assert_eq!(
                super::super::failure_labels(Failure::TlsSetup(stage)),
                ("tls", "setup")
            );
        }
    }
}

/// Windows-only controls reach the actual direct Rustls error projector.
#[cfg(test)]
#[cfg(windows)]
mod windows_tests {
    use super::{
        CertificateCause, Failure, TlsCause, TlsFailure, TlsIoStage, TransportFault, project_io,
        resolve,
    };
    use std::{
        error::Error,
        fmt, io,
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
        time::Duration,
    };

    /// Fail if classification formats, debugs or traverses this private payload.
    struct PayloadSpy(Arc<AtomicUsize>);

    impl PayloadSpy {
        /// This fixture has no legal inspection; its counter begins at zero.
        /// # Panics
        /// Panic if an error payload is inspected instead of merely downcast.
        fn touched(&self) {
            assert_eq!(
                self.0.fetch_add(1, Ordering::SeqCst),
                usize::MAX,
                "private error payload was inspected"
            );
        }
    }

    impl fmt::Debug for PayloadSpy {
        /// Reject inspection before producing any payload text.
        /// # Errors
        /// Preserve a sink error if the unreachable inspection sentinel is met.
        /// # Panics
        /// Fail on any attempted debug formatting of this fixture.
        fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.touched();
            output.write_str("unreachable private fixture")
        }
    }

    impl fmt::Display for PayloadSpy {
        /// Reject inspection before producing any payload text.
        /// # Errors
        /// Preserve a sink error if the unreachable inspection sentinel is met.
        /// # Panics
        /// Fail on any attempted display formatting of this fixture.
        fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.touched();
            output.write_str("unreachable private fixture")
        }
    }

    impl Error for PayloadSpy {
        /// Reject source traversal rather than revealing a nested payload.
        /// # Panics
        /// Fail if classification follows a private error source.
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.touched();
            None
        }
    }

    /// A different direct error type must not confer nested Rustls provenance.
    struct NestedTls {
        /// A genuine error behind a wrapper outside the pinned I/O contract.
        inner: rustls::Error,
        /// Detect every attempted traversal or formatting operation.
        spy: PayloadSpy,
    }

    impl fmt::Debug for NestedTls {
        /// Reject formatting before touching the nested error.
        /// # Errors
        /// Propagate the spy's formatting result.
        /// # Panics
        /// Fail on any attempted private payload inspection.
        fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.spy, output)
        }
    }

    impl fmt::Display for NestedTls {
        /// Reject formatting before touching the nested error.
        /// # Errors
        /// Propagate the spy's formatting result.
        /// # Panics
        /// Fail on any attempted private payload inspection.
        fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.spy, output)
        }
    }

    impl Error for NestedTls {
        /// A source exists, but the direct-only projector must never inspect it.
        /// # Panics
        /// Fail if classification follows this unrelated wrapper.
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.spy.touched();
            Some(&self.inner)
        }
    }

    /// Check every actual site using one real direct TLS wrapper and native neighbor.
    /// # Panics
    /// Fail if a typed reason, stage or ordinary native category is changed.
    fn assert_direct_cause(raw: rustls::Error, cause: TlsCause) {
        let error = io::Error::new(io::ErrorKind::InvalidData, raw);
        for (stage, expected_native) in [
            (
                TlsIoStage::Handshake,
                Failure::Handshake(io::ErrorKind::ConnectionRefused),
            ),
            (
                TlsIoStage::Request,
                Failure::Request(io::ErrorKind::ConnectionRefused),
            ),
            (
                TlsIoStage::Response,
                Failure::Response(io::ErrorKind::ConnectionRefused),
            ),
            (
                TlsIoStage::Shutdown,
                Failure::Shutdown(io::ErrorKind::ConnectionRefused),
            ),
        ] {
            assert_eq!(
                project_io(stage, &error),
                Failure::Tls(TlsFailure { stage, cause })
            );
            assert_eq!(
                project_io(stage, &io::ErrorKind::ConnectionRefused.into()),
                expected_native
            );
        }
    }

    /// Every actual site retains its native category beside direct TLS facts.
    /// # Panics
    /// Fail if a recognized cause, current stage or ordinary neighbor is lost.
    #[test]
    fn direct_rustls_causes_and_native_neighbors_are_distinct() {
        use rustls::{CertificateError, Error as TlsError};
        for (raw, cause) in [
            (
                TlsError::NoCertificatesPresented,
                TlsCause::Certificate(CertificateCause::Missing),
            ),
            (
                TlsError::InvalidCertificate(CertificateError::BadEncoding),
                TlsCause::Certificate(CertificateCause::Encoding),
            ),
            (
                TlsError::InvalidCertificate(CertificateError::Expired),
                TlsCause::Certificate(CertificateCause::Expired),
            ),
            (
                TlsError::InvalidCertificate(CertificateError::NotValidYet),
                TlsCause::Certificate(CertificateCause::NotYetValid),
            ),
            (
                TlsError::InvalidCertificate(CertificateError::NotValidForName),
                TlsCause::Certificate(CertificateCause::Name),
            ),
            (
                TlsError::InvalidCertificate(CertificateError::UnknownIssuer),
                TlsCause::Certificate(CertificateCause::Issuer),
            ),
            (
                TlsError::InvalidCertificate(CertificateError::Revoked),
                TlsCause::Certificate(CertificateCause::Revoked),
            ),
            (
                TlsError::InvalidCertificate(CertificateError::InvalidPurpose),
                TlsCause::Certificate(CertificateCause::Purpose),
            ),
            (
                TlsError::InvalidCertificate(CertificateError::BadSignature),
                TlsCause::Certificate(CertificateCause::Signature),
            ),
            (TlsError::PeerSentOversizedRecord, TlsCause::Protocol),
            (
                TlsError::PeerIncompatible(rustls::PeerIncompatible::Tls12NotOffered),
                TlsCause::Protocol,
            ),
            (
                TlsError::PeerMisbehaved(rustls::PeerMisbehaved::BadCertChainExtensions),
                TlsCause::Protocol,
            ),
            (
                TlsError::AlertReceived(rustls::AlertDescription::HandshakeFailure),
                TlsCause::PeerAlert,
            ),
            (TlsError::FailedToGetCurrentTime, TlsCause::LocalClock),
            (TlsError::FailedToGetRandomBytes, TlsCause::LocalEntropy),
        ] {
            assert_direct_cause(raw, cause);
        }
    }

    /// Inspect only projected facts, fixed labels and genuine primary precedence.
    /// # Panics
    /// Fail if a secret survives, projected facts differ, or a later fault wins.
    fn assert_private_projection(raw: rustls::Error, cause: TlsCause) {
        let error = io::Error::new(io::ErrorKind::InvalidData, raw);
        let projected = project_io(TlsIoStage::Response, &error);
        assert_eq!(
            projected,
            Failure::Tls(TlsFailure {
                stage: TlsIoStage::Response,
                cause
            })
        );
        assert!(!format!("{projected:?}").contains("synthetic-secret"));
        assert_eq!(
            super::super::failure_labels(projected),
            ("response", "invalid-data")
        );
        assert_eq!(
            resolve(
                Some(TransportFault::Control(Failure::Deadline)),
                Err::<(), _>(projected),
                Failure::Response
            ),
            Err(projected)
        );
    }

    /// Context forms lose all names/dates; opaque payloads remain unclassified.
    /// # Errors
    /// Preserve failure to construct the literal synthetic server name.
    /// # Panics
    /// Fail if payload inspection occurs, text survives, or TLS provenance is lost.
    #[test]
    fn context_and_opaque_payloads_are_not_retained_or_inspected() -> Result<(), Failure> {
        use rustls::{CertificateError, Error as TlsError};
        use rustls_pki_types::{ServerName, UnixTime};
        let touches = Arc::new(AtomicUsize::new(0));
        let expected =
            ServerName::try_from("synthetic-secret.invalid").map_err(|_| Failure::Control)?;
        for (raw, cause) in [
            (
                TlsError::InvalidCertificate(CertificateError::NotValidForNameContext {
                    expected,
                    presented: vec![String::from("synthetic-secret-presented.invalid")],
                }),
                TlsCause::Certificate(CertificateCause::Name),
            ),
            (
                TlsError::InvalidCertificate(CertificateError::ExpiredContext {
                    time: UnixTime::since_unix_epoch(Duration::from_secs(9)),
                    not_after: UnixTime::since_unix_epoch(Duration::from_secs(1)),
                }),
                TlsCause::Certificate(CertificateCause::Expired),
            ),
            (
                TlsError::InvalidCertificate(CertificateError::NotValidYetContext {
                    time: UnixTime::since_unix_epoch(Duration::from_secs(1)),
                    not_before: UnixTime::since_unix_epoch(Duration::from_secs(9)),
                }),
                TlsCause::Certificate(CertificateCause::NotYetValid),
            ),
            (
                TlsError::General(String::from("synthetic-secret-general")),
                TlsCause::Unclassified,
            ),
            (
                TlsError::Other(rustls::OtherError(Arc::new(PayloadSpy(Arc::clone(
                    &touches,
                ))))),
                TlsCause::Unclassified,
            ),
            (
                TlsError::InvalidCertificate(CertificateError::Other(rustls::OtherError(
                    Arc::new(PayloadSpy(Arc::clone(&touches))),
                ))),
                TlsCause::Certificate(CertificateCause::Unclassified),
            ),
        ] {
            assert_private_projection(raw, cause);
        }
        assert_eq!(touches.load(Ordering::SeqCst), 0);
        Ok(())
    }

    /// No native kind, unrelated wrapper or wrong kind proves TLS provenance.
    /// # Panics
    /// Fail if the classifier follows a source or invents TLS precedence.
    #[test]
    fn native_nested_and_wrong_kind_errors_remain_native() {
        let touches = Arc::new(AtomicUsize::new(0));
        for error in [
            io::ErrorKind::InvalidData.into(),
            io::Error::new(
                io::ErrorKind::InvalidData,
                NestedTls {
                    inner: rustls::Error::General(String::from("synthetic-secret-nested")),
                    spy: PayloadSpy(Arc::clone(&touches)),
                },
            ),
            io::Error::other(rustls::Error::NoCertificatesPresented),
        ] {
            let projected = project_io(TlsIoStage::Response, &error);
            assert_eq!(projected, Failure::Response(error.kind()));
            assert_eq!(
                resolve(
                    Some(TransportFault::Control(Failure::Cancelled)),
                    Err::<(), _>(projected),
                    Failure::Response
                ),
                Err(Failure::Cancelled)
            );
        }
        assert_eq!(touches.load(Ordering::SeqCst), 0);
    }

    /// One raw failure from the actual generic response reader; retry is visible.
    struct FailingReader {
        /// Original native or direct TLS error, consumed on the first call.
        error: Option<io::Error>,
        /// Actual receiving calls, independently checked after refusal.
        calls: usize,
    }

    impl io::Read for FailingReader {
        /// Refuse immediately; an unwanted retry yields a different cause.
        /// # Errors
        /// Return the original failure once, then an unexpected-EOF refusal.
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            self.calls += 1;
            Err(self
                .error
                .take()
                .unwrap_or_else(|| io::ErrorKind::UnexpectedEof.into()))
        }
    }

    /// The actual response consumer projects before its generic I/O kind is lost.
    /// # Errors
    /// Preserve fixture configuration/offer refusal or an unexpected success.
    /// # Panics
    /// Fail if the real receiving boundary erases TLS facts or retries the reader.
    #[test]
    fn actual_response_reader_preserves_typed_cause_and_native_neighbor() -> Result<(), Failure> {
        use super::super::{Config, encode_offer, read_response};
        use std::ffi::OsString;
        let config = Config::new(
            OsString::from("127.0.0.1:4460"),
            OsString::from("time.example"),
        )?;
        let mut storage = [0; 16];
        let offer = encode_offer(&mut storage)?;
        for (error, expected) in [
            (
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    rustls::Error::InvalidCertificate(rustls::CertificateError::UnknownIssuer),
                ),
                Failure::Tls(TlsFailure {
                    stage: TlsIoStage::Response,
                    cause: TlsCause::Certificate(CertificateCause::Issuer),
                }),
            ),
            (
                io::ErrorKind::TimedOut.into(),
                Failure::Response(io::ErrorKind::TimedOut),
            ),
        ] {
            let mut reader = FailingReader {
                error: Some(error),
                calls: 0,
            };
            let Err(failure) = read_response(&mut reader, &config, &offer) else {
                return Err(Failure::Control);
            };
            assert_eq!(failure, expected);
            assert_eq!(reader.calls, 1);
        }
        Ok(())
    }
}
