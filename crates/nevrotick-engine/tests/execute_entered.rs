//! One public preparation followed by an entered-runtime execution rejection.
//!
//! This executable owns one prepared pilot and its loopback prerequisites;
//! it establishes no successful query, nested runtime or clock effect.

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
