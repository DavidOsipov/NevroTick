// SPDX-License-Identifier: GPL-2.0-only
//! Shared checked TLS destination and verification name for the laboratory KE paths.

use garde::Validate;
use rustls_pki_types::ServerName;
use std::error::Error;
use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;

/// Retain the laboratory CLI's original verification-name bound in bytes.
const MAX_VERIFICATION_NAME_BYTES: usize = 253;

/// A TLS destination or verification name failed the bounded Preview contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KePreviewTargetError;

impl fmt::Display for KePreviewTargetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid KE preview target")
    }
}

impl Error for KePreviewTargetError {}

/// An explicit numeric loopback TLS destination and checked verification name.
///
/// Cloning shares the immutable target without allocating under source-policy
/// custody. This definition carries no source identity, live permission or TLS
/// completion.
///
/// External callers cannot manufacture another target from private fields.
/// ```compile_fail,E0451
/// use nevrotick_engine::KePreviewTarget;
/// fn bypass(target: KePreviewTarget) -> KePreviewTarget {
///     KePreviewTarget { ..target }
/// }
/// ```
#[derive(Clone)]
#[must_use = "retain the checked target for the actual Preview owner"]
pub struct KePreviewTarget {
    /// One immutable allocation shared across issued contexts and historical copies.
    inner: Arc<CheckedTarget>,
}

/// Inputs kept together after complete target and TLS name admission.
struct CheckedTarget {
    /// Numeric TLS connection destination with no alternate route.
    peer: SocketAddr,
    /// Original bounded spelling used by the laboratory endpoint policy.
    identity: String,
    /// DNS or IP name accepted by the TLS library and supplied to verification.
    server_name: ServerName<'static>,
}

/// Borrowed raw inputs validated before any owned accepted target is allocated.
#[derive(Validate)]
struct TargetCandidate<'a> {
    /// The shared rule admits the complete raw target before owned name parsing.
    #[garde(custom(target_rule))]
    fields: (SocketAddr, &'a str),
}

/// Admit the bounded numeric peer and literal name envelope.
///
/// # Errors
/// Reject non-loopback, zero-port, scoped or flow-labelled peers and empty,
/// non-ASCII or oversized names.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow the raw tuple and its unit context"
)]
fn target_rule(fields: &(SocketAddr, &str), (): &()) -> garde::Result {
    let (peer, identity): (SocketAddr, &str) = *fields;
    let scoped: bool = matches!(peer, SocketAddr::V6(address)
        if address.scope_id() != 0 || address.flowinfo() != 0);
    if !peer.ip().is_loopback()
        || peer.port() == 0
        || scoped
        || identity.is_empty()
        || identity.len() > MAX_VERIFICATION_NAME_BYTES
        || !identity.is_ascii()
    {
        return Err(garde::Error::new("invalid loopback KE configuration"));
    }
    Ok(())
}

impl KePreviewTarget {
    /// Admit an explicit peer and the exact name passed to TLS verification.
    ///
    /// # Errors
    /// Refuse an invalid peer, name envelope or TLS verification name without
    /// producing an accepted target. No provider, socket or policy is touched.
    pub fn new(peer: SocketAddr, identity: &str) -> Result<Self, KePreviewTargetError> {
        let candidate: TargetCandidate<'_> = TargetCandidate {
            fields: (peer, identity),
        };
        candidate.validate().map_err(|_| KePreviewTargetError)?;
        let identity: String = identity.to_owned();
        let server_name: ServerName<'static> =
            ServerName::try_from(identity.clone()).map_err(|_| KePreviewTargetError)?;
        let inner: Arc<CheckedTarget> = Arc::new(CheckedTarget {
            peer,
            identity,
            server_name,
        });
        Ok(Self { inner })
    }

    /// Return the original checked numeric TLS destination.
    #[must_use]
    pub fn peer(&self) -> SocketAddr {
        self.inner.peer
    }

    /// Borrow the original bounded verification identity for laboratory policy.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.inner.identity
    }

    /// Borrow the checked name used by the actual TLS verification constructor.
    #[must_use]
    pub fn server_name(&self) -> &ServerName<'static> {
        &self.inner.server_name
    }
}

impl fmt::Debug for KePreviewTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("KePreviewTarget { .. }")
    }
}

#[cfg(test)]
mod tests {
    use super::{KePreviewTarget, KePreviewTargetError, TargetCandidate};
    use garde::Validate;
    use rustls_pki_types::ServerName;
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV6};

    /// Keep the typed entry point inside the same envelope as the diagnostic parser.
    ///
    /// # Errors
    /// Return the actual refusal if a known valid peer and name cannot be admitted.
    ///
    /// # Panics
    /// Assert independent input refusal and checked identity retention.
    #[test]
    fn preview_target_checks_typed_peer_and_verification_name() -> Result<(), KePreviewTargetError>
    {
        let peer: SocketAddr = SocketAddr::from((Ipv4Addr::LOCALHOST, 4460));
        let target: KePreviewTarget = KePreviewTarget::new(peer, "localhost")?;
        assert_eq!(target.peer(), peer);
        assert_eq!(target.identity(), "localhost");
        assert!(
            matches!(target.server_name(), ServerName::DnsName(name) if name.as_ref() == "localhost")
        );
        assert_eq!(format!("{target:?}"), "KePreviewTarget { .. }");

        let refused_peers: [SocketAddr; 4] = [
            SocketAddr::from((Ipv4Addr::UNSPECIFIED, 4460)),
            SocketAddr::from((Ipv4Addr::LOCALHOST, 0)),
            SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 4460, 0, 1)),
            SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 4460, 1, 0)),
        ];
        for refused in refused_peers {
            assert!(KePreviewTarget::new(refused, "localhost").is_err());
        }
        // Each label fits DNS syntax, so the length specimens isolate the
        // whole-name bound in the raw validator before TLS name conversion.
        let label: String = "a".repeat(63);
        let last: String = "b".repeat(61);
        let maximum: String = format!("{label}.{label}.{label}.{last}");
        let oversized: String = format!("{maximum}b");
        assert_eq!((maximum.len(), oversized.len()), (253, 254));
        let boundary: TargetCandidate<'_> = TargetCandidate {
            fields: (peer, &maximum),
        };
        assert!(boundary.validate().is_ok());
        assert!(KePreviewTarget::new(peer, &maximum).is_ok());
        for identity in ["", "locálhost", oversized.as_str()] {
            let raw: TargetCandidate<'_> = TargetCandidate {
                fields: (peer, identity),
            };
            assert!(raw.validate().is_err(), "raw verification-name envelope");
            assert!(KePreviewTarget::new(peer, identity).is_err());
        }
        assert!(KePreviewTarget::new(peer, "host/name").is_err());
        Ok(())
    }
}
