//! One ordinary public preparation and socket-release control per test executable.
//!
//! Owned loopback sockets and rebind observations establish only this setup/drop
//! boundary, without protocol traffic or complete native resource qualification.

use std::error::Error;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use nevrotick_engine::{Config, prepare};

/// Keep two distinct controlled peers alive while testing actual native setup.
///
/// # Errors
/// Return host socket setup errors without substituting a protocol outcome.
fn controlled_peers() -> Result<([UdpSocket; 2], [SocketAddr; 2]), Box<dyn Error>> {
    let first = UdpSocket::bind("127.0.0.1:0")?;
    let second = UdpSocket::bind("127.0.0.1:0")?;
    let endpoints = [first.local_addr()?, second.local_addr()?];
    Ok(([first, second], endpoints))
}

/// Healthy control: valid ordinary startup obtains actual private ephemeral sockets.
///
/// # Errors
/// Preserve unavailable host entropy/socket setup as a failed prerequisite.
///
/// # Panics
/// Fail the independent assertion if native setup reports aliased sockets,
/// a non-loopback local address or an unassigned local port.
#[test]
fn prepare_outside_runtime_owns_and_releases_standard_sockets() -> Result<(), Box<dyn Error>> {
    let (_peers, endpoints) = controlled_peers()?;
    let config = Config::new(endpoints, Duration::from_secs(1))?;
    let (pilot, control) = prepare(&config)?;
    let locals = pilot.local_endpoints();
    assert_ne!(locals[0], locals[1]);
    for local in locals {
        assert!(local.ip().is_loopback());
        assert_ne!(local.port(), 0);
    }
    drop(pilot);
    drop(control);
    let _released_first = UdpSocket::bind(locals[0])?;
    let _released_second = UdpSocket::bind(locals[1])?;
    Ok(())
}
