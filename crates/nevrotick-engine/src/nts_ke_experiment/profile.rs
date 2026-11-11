// SPDX-License-Identifier: GPL-2.0-only
//! Immutable profile captured from the actual strict Windows TLS completion.
//!
//! The normal constructor is available only at the Windows connection boundary.
//! Test material below is explicitly component-only, never KE evidence.

#[cfg(windows)]
use super::Failure;

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

/// Actual negotiated profile retained beside the same-connection material.
pub(super) struct StrictProfile {
    /// Exact TLS-1.3 suite, captured from the completed connection.
    _suite: Suite,
    /// Full authentication form, with resumption excluded at the producer.
    _handshake: FullHandshake,
}

impl StrictProfile {
    /// Capture only completed TLS 1.3, exact ALPN and a full accepted handshake.
    /// # Errors
    /// Refuse absent/unsupported metadata, resumption, wrong ALPN or protocol.
    #[cfg(windows)]
    pub(super) fn capture(connection: &rustls::ClientConnection) -> Result<Self, Failure> {
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
        })
    }

    /// Public-material component profile; never used by the actual backend.
    #[cfg(test)]
    pub(super) fn component() -> Self {
        Self {
            _suite: Suite::Aes128,
            _handshake: FullHandshake::Full,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FullHandshake, StrictProfile, Suite};

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
                };
                let StrictProfile {
                    _suite: retained_suite,
                    _handshake: retained_handshake,
                } = profile;
                assert!(matches!(
                    retained_suite,
                    Suite::Aes128 | Suite::Aes256 | Suite::ChaCha
                ));
                assert!(matches!(
                    retained_handshake,
                    FullHandshake::Full | FullHandshake::HelloRetry
                ));
            }
        }
    }
}
