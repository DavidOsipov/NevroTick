// SPDX-License-Identifier: GPL-2.0-only
//! TCP shutdown and guard refusal controls.

use super::{ExchangeError, PendingTcpExchange, Phase, RootKey, RoundAuthority};
use ed25519_dalek::SigningKey;
use std::{
    error::Error,
    io::{self, Read},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    sync::MutexGuard,
    time::Duration,
};

/// Test-owned gate before the real worker's dispatch clock and claim.
pub(in crate::roughtime) struct BeforeConnectDispatch(
    /// Optional callback consumed immediately before the dispatch claim.
    pub(in crate::roughtime) Option<Box<dyn FnOnce() -> io::Result<()> + Send>>,
);

impl BeforeConnectDispatch {
    /// Apply no fixture gate.
    pub(in crate::roughtime) const READY: Self = Self(None);

    /// Wait for the fixture, then perform the unchanged dispatch transition.
    ///
    /// # Errors
    /// Returns a fixture failure or the actual dispatch refusal.
    pub(super) fn then<T>(
        self,
        dispatch: impl FnOnce() -> Result<T, ExchangeError>,
    ) -> Result<T, ExchangeError> {
        if let Some(wait) = self.0 {
            wait().map_err(|_| ExchangeError::ConnectWorker)?;
        }
        dispatch()
    }
}

/// Exercise a refused authority acquisition on an actual owned stream.
///
/// # Errors
/// Returns an unexpected fixture, authority or socket failure.
///
/// # Panics
/// Panics if the refused poll leaves the connection live or permits a later poll.
#[test]
fn closes_original_stream_when_guard_acquisition_is_refused() -> Result<(), Box<dyn Error>> {
    let budget: Duration = Duration::from_secs(2);
    let listener: TcpListener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
    let endpoint: SocketAddr = listener.local_addr()?;
    let client: TcpStream = TcpStream::connect_timeout(&endpoint, budget)?;
    listener.set_nonblocking(true)?;
    let (mut server, _): (TcpStream, SocketAddr) = listener.accept()?;
    server.set_read_timeout(Some(budget))?;
    let signer: SigningKey = SigningKey::from_bytes(&[39; 32]);
    let root: RootKey = RootKey::from_bytes(signer.verifying_key().as_bytes())
        .map_err(|error| format!("unexpected fixture root refusal: {error:?}"))?;
    let authority: RoundAuthority = RoundAuthority::new();
    let mut pending: PendingTcpExchange<'_> =
        PendingTcpExchange::start(&authority, client, endpoint, &root, budget)
            .map_err(|error| format!("unexpected fixture start refusal: {error:?}"))?;
    let held: MutexGuard<'_, Phase> = authority
        .phase()
        .map_err(|error| format!("unexpected fixture guard refusal: {error:?}"))?;
    assert!(matches!(
        pending.poll(),
        Err(ExchangeError::AuthorityUnavailable)
    ));
    let mut unexpected: [u8; 1] = [0];
    assert_eq!(server.read(&mut unexpected)?, 0);
    drop(held);
    assert!(matches!(pending.poll(), Err(ExchangeError::Spent)));
    Ok(())
}
