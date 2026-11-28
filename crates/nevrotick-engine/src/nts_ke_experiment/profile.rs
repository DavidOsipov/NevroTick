// SPDX-License-Identifier: GPL-2.0-only
//! Immutable profile captured from the actual strict Windows TLS completion.
//!
//! The normal constructor is available only at the Windows connection boundary.
//! Test material below is explicitly component-only, never KE evidence.

use std::time::Instant;

#[cfg(windows)]
use super::Failure;
#[cfg(windows)]
use super::transport::ConfiguredCertificateTime;

/// One of the three explicitly offered TLS-1.3 CNG suites.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Suite {
    /// First preference, preserving mandatory AES-128 interoperability.
    Aes128,
    /// Second preference; this choice does not change the NTS packet AEAD.
    Aes256,
    /// Compatible non-AES alternative from the same admitted provider.
    ChaCha,
}

/// Exact accepted full-handshake mode; no resumed mode is representable.
#[derive(Clone, Copy, Eq, PartialEq)]
enum FullHandshake {
    /// Ordinary full handshake.
    Full,
    /// Full handshake after the protocol's bounded hello retry.
    HelloRetry,
}

/// Classification of a protected receipt's original key establishment.
/// The Windows dispositions record which wall input accompanied acceptance by
/// the laboratory verifier. The classification carries no independent date,
/// current trust, or sample authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CertificateTimeProvenance {
    /// The Windows laboratory verifier accepted using ordinary local wall time.
    #[cfg(windows)]
    WindowsLaboratoryLocalWall,
    /// The Windows laboratory verifier accepted using a fixed test wall reading.
    #[cfg(all(windows, test))]
    WindowsLaboratoryFixtureWall,
    /// Component association material without a certificate verification.
    #[cfg(test)]
    Component,
}

/// Inclusive process-local bounds surrounding the original full TLS handshake.
#[derive(Clone, Copy)]
pub(super) struct FullAuthenticationBracket([Instant; 2]);

impl FullAuthenticationBracket {
    /// Retain ordered readings inside the original half-open handshake window.
    /// Return `None` when the historical bounds are unavailable.
    pub(super) fn within(
        started_at: Instant,
        completed_by: Instant,
        origin: Instant,
        deadline: Instant,
    ) -> Option<Self> {
        (origin <= started_at && started_at <= completed_by && completed_by < deadline)
            .then_some(Self([started_at, completed_by]))
    }

    /// Return the before-call and after-return readings in order.
    pub(super) const fn bounds(self) -> [Instant; 2] {
        self.0
    }
}

/// Actual negotiated profile retained beside the same-connection material.
pub(super) struct StrictProfile {
    /// Exact TLS-1.3 suite, captured from the completed connection.
    _suite: Suite,
    /// Full authentication form, with resumption excluded at the producer.
    _handshake: FullHandshake,
    /// Original accepted verifier and time input, or explicit component origin.
    certificate_time_provenance: CertificateTimeProvenance,
    /// Original checked process-local bracket, when its chronological bounds hold.
    full_authentication_bracket: Option<FullAuthenticationBracket>,
}

impl StrictProfile {
    /// Capture only completed TLS 1.3, exact ALPN and a full accepted handshake.
    /// The transport supplies the provenance of the time provider installed on
    /// this exact connection and any checked observations from its initial handshake.
    /// # Errors
    /// Refuse absent/unsupported metadata, resumption, wrong ALPN or protocol.
    #[cfg(windows)]
    pub(super) fn capture(
        connection: &rustls::ClientConnection,
        certificate_time: ConfiguredCertificateTime,
        full_authentication_bracket: Option<FullAuthenticationBracket>,
    ) -> Result<Self, Failure> {
        use rustls::{CipherSuite, HandshakeKind, ProtocolVersion};
        if connection.is_handshaking()
            || connection.protocol_version() != Some(ProtocolVersion::TLSv1_3)
            || connection.alpn_protocol() != Some(b"ntske/1".as_slice())
        {
            return Err(Failure::TlsProfile);
        }
        let suite = match connection
            .negotiated_cipher_suite()
            .map(|value| value.suite())
        {
            Some(CipherSuite::TLS13_AES_128_GCM_SHA256) => Suite::Aes128,
            Some(CipherSuite::TLS13_AES_256_GCM_SHA384) => Suite::Aes256,
            Some(CipherSuite::TLS13_CHACHA20_POLY1305_SHA256) => Suite::ChaCha,
            _ => return Err(Failure::TlsProfile),
        };
        let handshake = match connection.handshake_kind() {
            Some(HandshakeKind::Full) => FullHandshake::Full,
            Some(HandshakeKind::FullWithHelloRetryRequest) => FullHandshake::HelloRetry,
            _ => return Err(Failure::TlsProfile),
        };
        Ok(Self {
            _suite: suite,
            _handshake: handshake,
            certificate_time_provenance: certificate_time.into_provenance(),
            full_authentication_bracket,
        })
    }

    /// Public-material component profile; never used by the actual backend.
    #[cfg(test)]
    pub(super) fn component() -> Self {
        Self {
            _suite: Suite::Aes128,
            _handshake: FullHandshake::Full,
            certificate_time_provenance: CertificateTimeProvenance::Component,
            full_authentication_bracket: None,
        }
    }

    /// Exercise original bracket custody through a synthetic component association.
    #[cfg(test)]
    pub(super) fn component_with_authentication_bracket(
        bracket: FullAuthenticationBracket,
    ) -> Self {
        Self {
            full_authentication_bracket: Some(bracket),
            ..Self::component()
        }
    }

    /// Preserve the original classification without reading time or trust again.
    pub(super) const fn certificate_time_provenance(&self) -> CertificateTimeProvenance {
        self.certificate_time_provenance
    }

    /// Copy the original historical bracket with its immutable KE profile.
    pub(super) const fn full_authentication_bracket(&self) -> Option<FullAuthenticationBracket> {
        self.full_authentication_bracket
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::{
        CertificateTimeProvenance, FullAuthenticationBracket, FullHandshake, StrictProfile, Suite,
    };

    /// Observe exact boundaries of the original handshake window.
    ///
    /// # Errors
    /// Refuse fixture setup when a neighboring reading is unrepresentable.
    /// # Panics
    /// Fail if missing or reversed readings become usable, or an inclusive boundary is lost.
    #[test]
    fn full_authentication_bracket_checks_original_window() -> Result<(), &'static str> {
        let origin: Instant = Instant::now();
        let unit: Duration = Duration::from_millis(1);
        let before_origin: Instant = origin.checked_sub(unit).ok_or("pre-origin reading")?;
        let started: Instant = origin.checked_add(unit).ok_or("start reading")?;
        let completed: Instant = started.checked_add(unit).ok_or("completion reading")?;
        let deadline: Instant = completed.checked_add(unit).ok_or("deadline reading")?;
        let after_deadline: Instant = deadline.checked_add(unit).ok_or("post-deadline reading")?;

        let cases: [([Instant; 2], Option<[Instant; 2]>); 6] = [
            ([started, completed], Some([started, completed])),
            ([origin, origin], Some([origin, origin])),
            ([before_origin, completed], None),
            ([completed, started], None),
            ([started, deadline], None),
            ([started, after_deadline], None),
        ];
        for ([lower, upper], expected) in cases {
            assert_eq!(
                FullAuthenticationBracket::within(lower, upper, origin, deadline)
                    .map(FullAuthenticationBracket::bounds),
                expected,
            );
        }
        Ok(())
    }

    /// Component metadata preserves every selected full-profile alternative.
    /// # Panics
    /// Fail if retained metadata loses a selected suite or full-handshake form.
    #[test]
    fn selected_profile_metadata_is_retained() {
        for suite in [Suite::Aes128, Suite::Aes256, Suite::ChaCha] {
            for handshake in [FullHandshake::Full, FullHandshake::HelloRetry] {
                let profile = StrictProfile {
                    _suite: suite,
                    _handshake: handshake,
                    certificate_time_provenance: CertificateTimeProvenance::Component,
                    full_authentication_bracket: None,
                };
                let StrictProfile {
                    _suite: retained_suite,
                    _handshake: retained_handshake,
                    certificate_time_provenance,
                    full_authentication_bracket,
                } = profile;
                assert!(full_authentication_bracket.is_none());
                assert!(matches!(
                    retained_suite,
                    Suite::Aes128 | Suite::Aes256 | Suite::ChaCha
                ));
                assert!(matches!(
                    retained_handshake,
                    FullHandshake::Full | FullHandshake::HelloRetry
                ));
                assert_eq!(
                    certificate_time_provenance,
                    CertificateTimeProvenance::Component
                );
            }
        }
    }
}
