// SPDX-License-Identifier: GPL-2.0-only
//! Windows DNS submission for the source-owned loopback round.

use std::{
    net::SocketAddr,
    num::NonZeroU16,
    time::{Duration, Instant},
};

use tokio::sync::watch;
use windows_platform_native::{DnsAddressFamily, DnsAddressQuery, DnsAddresses, DnsQueryName};

use super::{
    AddressRoundConfig, AddressRoundConfigError, AddressRoundEnd, AddressRoundRefreshOutcome,
    CAPACITY, Control, ControlHandle, PACKET_BYTES, PilotError, PreparedAddressRound, QueryPlan,
    RefreshPlan, RoundBindings, end_at, native_job::NativeResolverJob, policy_refusal,
    prepare_with_budget,
};
use crate::{SourceHandle, SourcePolicyAuthority, SourcePolicyError};

/// Checked query definition and locally reserved successor work.
pub(in super::super) struct WindowsRefreshPlan {
    /// Absolute name supplied by this source authority.
    name: DnsQueryName,
    /// Exactly one address family per operation.
    family: DnsAddressFamily,
    /// Caller-selected UDP service port; DNS supplies only addresses.
    port: NonZeroU16,
    /// Original per-request allowance.
    request_timeout: Duration,
    /// Original whole-round allowance.
    round_timeout: Duration,
    /// Successor tokens reserved before either round starts.
    packets: [[u8; PACKET_BYTES]; CAPACITY],
}

impl SourcePolicyAuthority {
    /// Prepare a loopback round with one optional Windows system DNS successor.
    ///
    /// Execution can query the supplied name through the Windows DNS Client.
    /// Published destinations still require the original source envelope and
    /// current membership authority. The port comes from this declaration.
    /// # Errors
    /// Refuse foreign source ownership, missing membership, insufficient budget,
    /// an entered runtime, or entropy failure.
    pub fn prepare_windows_address_round_refresh(
        &self,
        source: &SourceHandle,
        initial: AddressRoundConfig,
        name: DnsQueryName,
        family: DnsAddressFamily,
        port: NonZeroU16,
    ) -> Result<(PreparedAddressRound, ControlHandle), PilotError> {
        let _target = self
            .source_target(source)
            .map_err(PilotError::SourcePolicy)?;
        let request_timeout: Duration = initial.request_timeout;
        let round_timeout: Duration = initial.round_timeout;
        let (mut prepared, handle, packets) = prepare_with_budget(source, initial, CAPACITY)?;
        prepared.refresh = Some(QueryPlan::Windows(Box::new(WindowsRefreshPlan {
            name,
            family,
            port,
            request_timeout,
            round_timeout,
            packets,
        })));
        Ok((prepared, handle))
    }
}

/// Hand blocking submission and actual callback wait to the retained worker.
/// # Errors
/// Refuse occupied process custody or failure to start the worker.
pub(super) fn start(
    plan: WindowsRefreshPlan,
    source: RoundBindings,
    control: watch::Receiver<Control>,
    deadline: Instant,
) -> Result<NativeResolverJob, PilotError> {
    NativeResolverJob::start(move |dispatch| {
        {
            let current: watch::Ref<'_, Control> = control.borrow();
            if let Some(end) = end_at(*current, Instant::now(), deadline) {
                return Err(AddressRoundRefreshOutcome::Retired(end));
            }
            source
                .with_current_resolution(|| {
                    if Instant::now() >= deadline {
                        return Err(AddressRoundRefreshOutcome::Retired(
                            AddressRoundEnd::Deadline,
                        ));
                    }
                    if dispatch.admit() {
                        Ok(())
                    } else {
                        Err(AddressRoundRefreshOutcome::NotNeeded)
                    }
                })
                .map_err(policy_refusal)??;
        }
        let addresses: DnsAddresses = DnsAddressQuery::start(plan.name, plan.family)
            .and_then(DnsAddressQuery::wait)
            .map_err(|error| {
                AddressRoundRefreshOutcome::Failed(PilotError::NativeResolution(error))
            })?;
        let peers: Vec<SocketAddr> = addresses
            .iter()
            .map(|address| SocketAddr::new(address, plan.port.get()))
            .collect();
        let config: AddressRoundConfig =
            AddressRoundConfig::new(&peers, plan.request_timeout, plan.round_timeout).map_err(
                |error| match error {
                    AddressRoundConfigError::InvalidTimeout => {
                        AddressRoundRefreshOutcome::Failed(PilotError::InvalidTimeout)
                    }
                    _ => policy_refusal(SourcePolicyError::InvalidSelection),
                },
            )?;
        Ok(RefreshPlan {
            config,
            packets: plan.packets,
        })
    })
}
