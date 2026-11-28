// SPDX-License-Identifier: GPL-2.0-only
//! Retry floor and endpoint binding controls.

use super::{ExchangeError, ExchangeTransport, RetryBinding, RetryState, RootKey, minimum_wait};
use ed25519_dalek::SigningKey;
use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

/// Admit both transports at distinct aliases and refuse one alias entered twice.
///
/// # Errors
/// Returns an unexpected fixture root or distinct-alias admission error.
///
/// # Panics
/// Panics if a duplicate enters the binding or either valid alias loses a transport.
#[test]
fn legacy_retry_binding_requires_distinct_aliases() -> Result<(), &'static str> {
    let key: SigningKey = SigningKey::from_bytes(&[61; 32]);
    let root: RootKey =
        RootKey::from_bytes(&key.verifying_key().to_bytes()).map_err(|_| "fixture root")?;
    let first: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 5319));
    let second: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 5320));
    assert!(matches!(
        RetryBinding::new(&root, [first, first]),
        Err(ExchangeError::Configuration)
    ));
    let binding: RetryBinding<'_> =
        RetryBinding::new(&root, [first, second]).map_err(|_| "fixture distinct aliases")?;
    for endpoint in [first, second] {
        assert!(
            binding
                .require_route(endpoint, ExchangeTransport::Udp)
                .is_ok()
        );
        assert!(
            binding
                .require_route(endpoint, ExchangeTransport::Tcp)
                .is_ok()
        );
    }
    Ok(())
}

/// Verify growth, upward rounding, and the boundary on each side of the cap.
///
/// # Panics
/// Panics if a selected floor differs from its independently computed value.
#[test]
fn rational_retry_floor_rounds_up_and_saturates() {
    let expected: [(u8, u64); 9] = [
        (1, 1_000_000_000),
        (2, 1_500_000_000),
        (3, 2_250_000_000),
        (4, 3_375_000_000),
        (11, 57_665_039_063),
        (12, 86_497_558_594),
        (29, 85_222_692_992_393),
        (30, 86_400_000_000_000),
        (u8::MAX, 86_400_000_000_000),
    ];
    for (failures, nanos) in expected {
        assert_eq!(minimum_wait(failures), Duration::from_nanos(nanos));
    }
}

/// Compare the exact retry boundary and refuse a reversed terminal clock.
///
/// # Errors
/// Returns a fixture error if the host clock cannot represent the bounds.
///
/// # Panics
/// Panics if an early or invalid clock observation admits retry work.
#[test]
fn floor_refuses_early_active_and_reversed_observations() -> Result<(), &'static str> {
    let mut state: RetryState = RetryState::default();
    let start: Instant = Instant::now();
    let retired: Instant = start
        .checked_add(Duration::from_millis(50))
        .ok_or("fixture retirement")?;
    let until: Instant = retired
        .checked_add(Duration::from_secs(1))
        .ok_or("fixture floor")?;
    let early: Instant = until
        .checked_sub(Duration::from_nanos(1))
        .ok_or("fixture early")?;
    state.failed(start, retired);
    assert!(
        matches!(state.check(early), Err(ExchangeError::RetryDeferred { until: actual }) if actual == until)
    );
    assert!(state.check(until).is_ok());
    let lease: super::RetryLease<'_> = state.charge(until).map_err(|_| "fixture charge")?;
    let charged: &RetryState = lease.state.as_deref().ok_or("fixture missing owner")?;
    assert!(matches!(
        charged.check(until),
        Err(ExchangeError::RetryInFlight)
    ));
    lease.authenticated();
    state.failed(retired, start);
    assert!(matches!(
        state.check(until),
        Err(ExchangeError::RetryClockUnavailable)
    ));
    Ok(())
}
