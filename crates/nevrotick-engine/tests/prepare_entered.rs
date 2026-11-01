//! One direct public preparation attempt from an entered runtime per executable.
//!
//! The expected early rejection prevents entropy and engine socket startup;
//! separately owned loopback peers remain fixture prerequisites.

use std::error::Error;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use nevrotick_engine::{Config, PilotError, prepare};

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
