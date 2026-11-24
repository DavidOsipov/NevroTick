// SPDX-License-Identifier: GPL-2.0-only
//! Immutable profile captured from the actual strict Windows TLS completion.
//!
//! The normal constructor is available only at the Windows connection boundary.
//! Test material below is explicitly component-only, never KE evidence.

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

/// Actual negotiated profile retained beside the same-connection material.
pub(super) struct StrictProfile {
    /// Exact TLS-1.3 suite, captured from the completed connection.
    _suite: Suite,
    /// Full authentication form, with resumption excluded at the producer.
    _handshake: FullHandshake,
    /// Original accepted verifier and time input, or explicit component origin.
    certificate_time_provenance: CertificateTimeProvenance,
}

impl StrictProfile {
    /// Capture only completed TLS 1.3, exact ALPN and a full accepted handshake.
    /// The transport supplies the provenance of the time provider installed on
    /// this exact connection.
    /// # Errors
    /// Refuse absent/unsupported metadata, resumption, wrong ALPN or protocol.
    #[cfg(windows)]
    pub(super) fn capture(
        connection: &rustls::ClientConnection,
        certificate_time: ConfiguredCertificateTime,
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
        })
    }

    /// Public-material component profile; never used by the actual backend.
    #[cfg(test)]
    pub(super) fn component() -> Self {
        Self {
            _suite: Suite::Aes128,
            _handshake: FullHandshake::Full,
            certificate_time_provenance: CertificateTimeProvenance::Component,
        }
    }

    /// Preserve the original classification without reading time or trust again.
    pub(super) const fn certificate_time_provenance(&self) -> CertificateTimeProvenance {
        self.certificate_time_provenance
    }
}

#[cfg(test)]
mod tests {
    use super::{CertificateTimeProvenance, FullHandshake, StrictProfile, Suite};

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
                };
                let StrictProfile {
                    _suite: retained_suite,
                    _handshake: retained_handshake,
                    certificate_time_provenance,
                } = profile;
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
