//! Real caller-context controls for the synchronous transport admission boundary.
//!
//! These tests bind owned loopback sockets; they send no packets and change no
//! clock or service state. Standard-socket release is narrower than Tokio/AFD
//! operation teardown and supplies no native Windows qualification.

use std::error::Error;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use nevrotick_engine::{Config, ControlSchedule, PilotError, execute, prepare};

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

/// Direct-call bypass: synchronous native startup must reject an entered runtime.
///
/// # Errors
/// Return setup failure or the independently specified missing rejection.
#[test]
fn prepare_rejects_entered_runtime_before_native_startup() -> Result<(), Box<dyn Error>> {
    let (_peers, endpoints) = controlled_peers()?;
    let config = Config::new(endpoints, Duration::from_secs(1))?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()?;
    let result = runtime.block_on(async { prepare(&config) });
    match result {
        Err(PilotError::EnteredRuntime) => Ok(()),
        Err(other) => Err(format!("wrong admission failure: {}", other.as_str()).into()),
        Ok(_) => Err("native startup was admitted from an entered runtime".into()),
    }
}

/// Neighboring enforced boundary: execute returns a typed error instead of nesting runtimes.
///
/// # Errors
/// Return setup failure or a violation of the independent context contract.
#[test]
fn execute_rejects_entered_runtime_without_nested_runtime_creation() -> Result<(), Box<dyn Error>> {
    let (_peers, endpoints) = controlled_peers()?;
    let (pilot, _control) = prepare(&Config::new(endpoints, Duration::from_secs(1))?)?;
    let schedule = ControlSchedule::new(None, None)?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()?;
    let result = runtime.block_on(async { execute(pilot, schedule) });
    match result {
        Err(PilotError::EnteredRuntime) => Ok(()),
        Err(other) => Err(format!("wrong execution failure: {}", other.as_str()).into()),
        Ok(_) => Err("nested runtime execution was admitted".into()),
    }
}
