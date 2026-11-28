// SPDX-License-Identifier: GPL-2.0-only
//! Scheduler-owned retry floor for one controlled Roughtime peer and root.

use super::{ExchangeError, ExchangeTransport, RootKey};
use crate::native_network_job::{JoinWitness, Settlement};
use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

/// Saturation begins at the 30th failure for the selected 3/2 profile.
const CAP_FAILURE: u8 = 30;
/// Exact maximum wait of 24 hours, in nanoseconds.
const CAP_NANOS: u64 = 86_400_000_000_000;

/// One exact loopback endpoint and transport selected for a controlled peer.
///
/// ```compile_fail,E0451
/// use std::net::SocketAddr;
/// use nevrotick_engine::roughtime::exchange::{ExchangeTransport, retry::RetryRoute};
/// let _route = RetryRoute {
///     endpoint: SocketAddr::from(([0, 0, 0, 0], 5319)),
///     transport: ExchangeTransport::Udp,
/// };
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryRoute {
    /// Checked numeric endpoint before any native work.
    endpoint: SocketAddr,
    /// Transport permitted on that endpoint.
    transport: ExchangeTransport,
}

impl RetryRoute {
    /// Admit a bounded numeric route for one transport.
    ///
    /// # Errors
    /// Refuses non-loopback or noncanonical endpoints and zero ports.
    pub fn new(endpoint: SocketAddr, transport: ExchangeTransport) -> Result<Self, ExchangeError> {
        require_loopback_endpoint(endpoint)?;
        Ok(Self {
            endpoint,
            transport,
        })
    }
}

/// Complete route set of one controlled binding.
enum RetryRoutes {
    /// The original fixture constructor explicitly allows both transports at each alias.
    LoopbackAliases([SocketAddr; 2]),
    /// The exact route or pair selected before any native effect.
    Selected {
        /// First selected route.
        primary: RetryRoute,
        /// Optional independently selected second route.
        alternate: Option<RetryRoute>,
    },
}

/// One controlled peer/root with a retry floor shared by its selected routes.
///
/// The scheduler retains this same binding across attempts and round changes.
/// The exclusive attempt borrow prevents overlap. Installed catalogue and
/// persisted floor continuity are separate admission responsibilities.
///
/// ```compile_fail,E0499
/// use std::{net::SocketAddr, time::Duration};
/// use nevrotick_engine::roughtime::exchange::{ExchangeError, PendingExchange, RoundAuthority};
/// use nevrotick_engine::roughtime::exchange::retry::RetryBinding;
/// fn overlap(binding: &mut RetryBinding<'_>, aliases: [SocketAddr; 2],
///            left: &RoundAuthority, right: &RoundAuthority) -> Result<(), ExchangeError> {
///     let budget = Duration::from_secs(2);
///     let first = PendingExchange::send_with_retry(left, binding, aliases[0], budget)?;
///     let second = PendingExchange::send_with_retry(right, binding, aliases[1], budget)?;
///     drop((first, second));
///     Ok(())
/// }
/// ```
pub struct RetryBinding<'a> {
    /// Exact previously selected root used by every route.
    root: &'a RootKey,
    /// Complete selected route set.
    routes: RetryRoutes,
    /// Floor shared by all selected routes.
    state: RetryState,
}

impl<'a> RetryBinding<'a> {
    /// Bind a controlled fixture root to two distinct addresses for UDP and TCP.
    ///
    /// # Errors
    /// Refuses duplicate, non-loopback or noncanonical addresses and zero ports.
    pub fn new(root: &'a RootKey, aliases: [SocketAddr; 2]) -> Result<Self, ExchangeError> {
        if aliases[0] == aliases[1] {
            return Err(ExchangeError::Configuration);
        }
        for endpoint in aliases {
            require_loopback_endpoint(endpoint)?;
        }
        Ok(Self {
            root,
            routes: RetryRoutes::LoopbackAliases(aliases),
            state: RetryState::default(),
        })
    }

    /// Bind the exact controlled route or pair to one root and original retry state.
    ///
    /// UDP and TCP may share a numeric endpoint. Both retain the same retry floor.
    ///
    /// # Errors
    /// Refuses two copies of the same endpoint and transport.
    pub fn new_for_routes(
        root: &'a RootKey,
        primary: RetryRoute,
        alternate: Option<RetryRoute>,
    ) -> Result<Self, ExchangeError> {
        if alternate == Some(primary) {
            return Err(ExchangeError::Configuration);
        }
        Ok(Self {
            root,
            routes: RetryRoutes::Selected { primary, alternate },
            state: RetryState::default(),
        })
    }

    /// Lend the exact root and its floor for one selected route.
    ///
    /// # Errors
    /// Refuses an unselected route or a floor that cannot yet admit native work.
    pub(super) fn admission(
        &mut self,
        peer: SocketAddr,
        transport: ExchangeTransport,
    ) -> Result<(&RootKey, &mut RetryState), ExchangeError> {
        let (root, state): (&RootKey, &mut RetryState) = self.selection(peer, transport)?;
        state.check(Instant::now())?;
        Ok((root, state))
    }

    /// Lend the selected route and state for evaluation at a later native send.
    ///
    /// # Errors
    /// Refuses an endpoint or transport outside the original binding.
    pub(in crate::roughtime) fn selection(
        &mut self,
        peer: SocketAddr,
        transport: ExchangeTransport,
    ) -> Result<(&RootKey, &mut RetryState), ExchangeError> {
        self.require_route(peer, transport)?;
        Ok((self.root, &mut self.state))
    }

    /// Check whether the original owner selected this exact endpoint and transport.
    ///
    /// # Errors
    /// Refuses an unselected endpoint or a transport absent for that endpoint.
    pub(in crate::roughtime) fn require_route(
        &self,
        peer: SocketAddr,
        transport: ExchangeTransport,
    ) -> Result<(), ExchangeError> {
        match &self.routes {
            RetryRoutes::LoopbackAliases(aliases) if aliases.contains(&peer) => Ok(()),
            RetryRoutes::LoopbackAliases(_) => Err(ExchangeError::ForeignPeer),
            RetryRoutes::Selected { primary, alternate } => {
                let matches_route =
                    |route: RetryRoute| route.endpoint == peer && route.transport == transport;
                if matches_route(*primary) || alternate.is_some_and(matches_route) {
                    Ok(())
                } else if primary.endpoint == peer
                    || alternate.is_some_and(|route| route.endpoint == peer)
                {
                    Err(ExchangeError::UnselectedTransport)
                } else {
                    Err(ExchangeError::ForeignPeer)
                }
            }
        }
    }
}

/// Apply the controlled endpoint rule consistently to both constructors.
///
/// # Errors
/// Refuses non-loopback endpoints, zero ports or a noncanonical IPv6 representation.
fn require_loopback_endpoint(endpoint: SocketAddr) -> Result<(), ExchangeError> {
    if !endpoint.ip().is_loopback()
        || endpoint.port() == 0
        || matches!(endpoint, SocketAddr::V6(v6) if v6.scope_id() != 0 || v6.flowinfo() != 0)
    {
        return Err(ExchangeError::Configuration);
    }
    Ok(())
}

/// Current earliest retry, including fail-closed monotonic arithmetic.
#[derive(Clone, Copy, Default)]
enum Floor {
    /// No outstanding failure floor.
    #[default]
    Ready,
    /// The original attempt is charged and has no observed retirement.
    InFlight,
    /// The original binding may retry at this instant or later.
    After(Instant),
    /// The original time frame could not represent a safe new deadline.
    Unavailable,
}

/// Exclusive state of one controlled binding.
#[derive(Default)]
pub(in crate::roughtime) struct RetryState {
    /// Consecutive charged failures, capped after growth reaches its limit.
    failures: u8,
    /// Current retry gate, unchanged by uncharged attempts.
    floor: Floor,
    /// An unsettled owned connect, which can outlive the borrowed round.
    connect: Option<JoinWitness>,
}

impl RetryState {
    /// Check the configured floor against this monotonic observation.
    ///
    /// # Errors
    /// Refuses early observation or an unavailable floor.
    pub(in crate::roughtime) fn check(&self, now: Instant) -> Result<(), ExchangeError> {
        let (floor, _): (Floor, u8) = self.resolved_floor()?;
        Self::check_floor(floor, now)
    }

    /// Resolve a discarded connect only through its exact joined worker.
    ///
    /// # Errors
    /// Refuses a connect whose native retirement has not been observed.
    fn resolved_floor(&self) -> Result<(Floor, u8), ExchangeError> {
        let Some(connect): Option<&JoinWitness> = self.connect.as_ref() else {
            return Ok((self.floor, self.failures));
        };
        let settled: Settlement = connect.settled().ok_or(ExchangeError::RetryInFlight)?;
        let Some(submitted_at): Option<Instant> = settled.submitted_at else {
            return Ok((self.floor, self.failures));
        };
        let failures: u8 = self.failures.saturating_add(1).min(CAP_FAILURE);
        Ok((
            Self::failure_floor(submitted_at, settled.joined_at, failures),
            failures,
        ))
    }

    /// Compare the original typed floor with the current observation.
    ///
    /// # Errors
    /// Refuses pending work, an early retry or unavailable clock evidence.
    fn check_floor(floor: Floor, now: Instant) -> Result<(), ExchangeError> {
        match floor {
            Floor::InFlight => Err(ExchangeError::RetryInFlight),
            Floor::After(until) if now < until => Err(ExchangeError::RetryDeferred { until }),
            Floor::Ready | Floor::After(_) => Ok(()),
            Floor::Unavailable => Err(ExchangeError::RetryClockUnavailable),
        }
    }

    /// Preserve settled debt before a caller attempts a new reservation.
    ///
    /// # Errors
    /// Refuses pending native work or an ineligible floor.
    pub(super) fn prepare_worker(&mut self, now: Instant) -> Result<(), ExchangeError> {
        let (floor, failures): (Floor, u8) = self.resolved_floor()?;
        Self::check_floor(floor, now)?;
        self.floor = floor;
        self.failures = failures;
        self.connect = None;
        Ok(())
    }

    /// Retain the exact recently reserved connect until its native retirement.
    pub(super) fn track_worker(&mut self, connect: JoinWitness) {
        self.connect = Some(connect);
    }

    /// Move the joined and current connect into the original stream's retry lease.
    ///
    /// # Errors
    /// Refuses an absent or unjoined worker, or a missing or disordered dispatch.
    pub(super) fn adopt_worker(&mut self) -> Result<RetryLease<'_>, ExchangeError> {
        let settled: Settlement = self
            .connect
            .as_ref()
            .and_then(JoinWitness::settled)
            .ok_or(ExchangeError::RetryInFlight)?;
        let submitted_at: Instant = settled.submitted_at.ok_or(ExchangeError::ConnectWorker)?;
        if settled.joined_at < submitted_at {
            return Err(ExchangeError::CaptureOrder);
        }
        self.connect = None;
        Ok(self.lease(submitted_at))
    }

    /// Charge the next native attempt under the original floor and borrow.
    ///
    /// # Errors
    /// Refuses a floor that cannot admit this exact native attempt.
    pub(super) fn charge(
        &mut self,
        submitted_at: Instant,
    ) -> Result<RetryLease<'_>, ExchangeError> {
        self.prepare_worker(submitted_at)?;
        Ok(self.lease(submitted_at))
    }

    /// Lend one exclusive charge after eligibility or original worker adoption.
    fn lease(&mut self, submitted_at: Instant) -> RetryLease<'_> {
        let prior_floor: Floor = self.floor;
        self.floor = Floor::InFlight;
        RetryLease {
            state: Some(self),
            submitted_at,
            prior_floor,
        }
    }

    /// Clear the floor after admission by the original complete-chain owner.
    pub(in crate::roughtime) fn reset(&mut self) {
        *self = Self::default();
    }

    /// Retire failed or discarded native work and advance the original floor.
    fn failed(&mut self, submitted_at: Instant, retired_at: Instant) {
        self.failures = self.failures.saturating_add(1).min(CAP_FAILURE);
        self.floor = Self::failure_floor(submitted_at, retired_at, self.failures);
    }

    /// Derive the next floor from the witnessed retirement of this charged work.
    fn failure_floor(submitted_at: Instant, retired_at: Instant, failures: u8) -> Floor {
        if retired_at < submitted_at {
            Floor::Unavailable
        } else {
            retired_at
                .checked_add(minimum_wait(failures))
                .map_or(Floor::Unavailable, Floor::After)
        }
    }
}

/// Exclusive custody of one native attempt through its definitive result.
pub(in crate::roughtime) struct RetryLease<'a> {
    /// Present until the current signed publication or terminal failure.
    state: Option<&'a mut RetryState>,
    /// Monotonic observation immediately before the original submitted effect.
    submitted_at: Instant,
    /// Original already-due floor preserved for a provisional signed leg.
    prior_floor: Floor,
}

impl<'a> RetryLease<'a> {
    /// Commit the reset from one actual current single-exchange publication.
    pub(super) fn authenticated(mut self) {
        if let Some(state) = self.state.take() {
            state.reset();
        }
    }

    /// Retire the accepted leg and restore its prior debt without resetting.
    ///
    /// # Errors
    /// Refuses an attempt without its original state.
    pub(in crate::roughtime) fn tentative(mut self) -> Result<&'a mut RetryState, ExchangeError> {
        let state: &mut RetryState = self.state.take().ok_or(ExchangeError::Spent)?;
        state.floor = self.prior_floor;
        Ok(state)
    }
}

impl Drop for RetryLease<'_> {
    /// Conservatively advance the floor if this attempt did not publish.
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            state.failed(self.submitted_at, Instant::now());
        }
    }
}

/// Compute the exact rational growth and round upward once at nanosecond precision.
fn minimum_wait(failures: u8) -> Duration {
    let exponent: u32 = u32::from(failures.clamp(1, CAP_FAILURE) - 1);
    // The largest exponent is 29, so the scaled numerator fits within 77 bits.
    let nanos: u128 = (1_000_000_000 * 3_u128.pow(exponent)).div_ceil(1_u128 << exponent);
    let capped: u128 = nanos.min(u128::from(CAP_NANOS));
    Duration::from_nanos(u64::try_from(capped).unwrap_or(CAP_NANOS))
}

#[cfg(test)]
mod tests;
