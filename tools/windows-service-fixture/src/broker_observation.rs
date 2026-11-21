// SPDX-License-Identifier: GPL-2.0-only
//! A test-selected, one-shot read owned by the broker invocation.

use std::time::Instant;

use windows_platform::{
    scm::StopNotification,
    service::{ServiceRole, ServiceState, ServiceStatus, StatusReporter},
};

use super::{
    Clock, EventWriter, InertWorker, NativeError, ServiceInvocation, Worker, WorkerCustody,
    WorkerPhase,
};

#[path = "broker_power.rs"]
mod power;

use power::BrokerPowerFence;
#[cfg(test)]
pub(super) use power::BrokerPowerGeneration;
#[cfg(windows)]
pub(super) use power::BrokerPowerWatch;

/// Raw words and mode from the same successful read, with no rate or mapping claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::host) struct RawAdjustment {
    /// Raw adjusted clock update value.
    pub(in crate::host) adjustment: u64,
    /// Raw clock update increment.
    pub(in crate::host) increment: u64,
    /// Whether Windows reports periodic programmed adjustment disabled.
    pub(in crate::host) periodic_adjustment_disabled: bool,
}

/// Distinct observations retained by the same private broker probe slot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::host) enum BrokerReading {
    /// Native adjustment words and programmed mode.
    Adjustment(RawAdjustment),
    /// The QPC span enclosing the raw system wall-clock reading.
    #[cfg(windows)]
    ClockCoordinates(windows_platform_native::ClockCoordinates),
}

/// Native refusals retain the operation and its exact error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::host) enum BrokerReadFailure {
    /// The adjustment getter failed with the original Win32 error.
    Adjustment(u32),
    /// A staged coordinate read or its validation failed.
    #[cfg(windows)]
    ClockCoordinates(windows_platform_native::ClockReadError),
}

pub(in crate::host) use crate::ClockReadRetirement as Retirement;

/// Historical result of the selected fixture probe, independent of final service status.
#[must_use = "retain the actual broker read or its refusal"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::host) enum BrokerObservation {
    /// A single successful read bracketed by admitted owner-clock observations.
    Observed {
        /// Owner time observed before the getter.
        before: Instant,
        /// Owner time observed after the getter returned.
        after: Instant,
        /// Exact raw readout with no inferred units or clock generation.
        raw: BrokerReading,
    },
    /// The getter failed while the enclosing lifecycle remained admissible.
    NativeFailure(BrokerReadFailure),
    /// The enclosing lifecycle refused the call or its returned result.
    Retired {
        /// The first applicable retirement at the bracket boundary.
        reason: Retirement,
        /// Original native failure if the getter had already returned one.
        native_failure: Option<BrokerReadFailure>,
    },
}

/// Once selected, the probe retains either its sole callable or its final result.
pub(super) enum BrokerProbe {
    /// Dormant read awaiting the existing Running charge.
    Pending {
        /// The sole selected getter, consumed after acknowledged Running.
        read: Box<dyn FnOnce() -> Result<BrokerReading, BrokerReadFailure>>,
        /// Coordinates require the generation of the original subscribed observer.
        power: Option<BrokerPowerFence>,
    },
    /// Immutable result carried to the host receipt; no callable remains.
    Finished(BrokerObservation),
}

impl<R: StatusReporter<Error = NativeError>, W: EventWriter> ServiceInvocation<R, InertWorker, W> {
    /// Select a dormant probe once on an admitted broker invocation. Selection
    /// does not call the getter, acquire work capacity, or change service status.
    #[must_use = "only dormant, admitted broker invocations can select a read"]
    pub(super) fn select_broker_probe(
        &mut self,
        read: impl FnOnce() -> Result<RawAdjustment, u32> + 'static,
    ) -> bool {
        self.select_adjustment_probe(read, None)
    }

    /// Use the actual receiving gate with a portable controlled invalidation signal.
    #[cfg(test)]
    #[must_use = "only dormant, admitted broker invocations can select a read"]
    pub(super) fn select_power_scoped_probe(
        &mut self,
        generation: &BrokerPowerGeneration,
        read: impl FnOnce() -> Result<RawAdjustment, u32> + 'static,
    ) -> bool {
        self.select_adjustment_probe(read, Some(generation.fence()))
    }

    /// Project only the raw adjustment or its original native error.
    fn select_adjustment_probe(
        &mut self,
        read: impl FnOnce() -> Result<RawAdjustment, u32> + 'static,
        power: Option<BrokerPowerFence>,
    ) -> bool {
        self.select_probe(power, move || {
            read()
                .map(BrokerReading::Adjustment)
                .map_err(BrokerReadFailure::Adjustment)
        })
    }

    /// Select one actual Windows coordinate capture through the same broker guard.
    #[cfg(windows)]
    #[must_use = "only dormant, admitted broker invocations can select a read"]
    pub(super) fn select_clock_coordinates(&mut self, watch: &BrokerPowerWatch) -> bool {
        self.select_probe(Some(watch.fence()), || {
            windows_platform_native::read_clock_coordinates()
                .map(BrokerReading::ClockCoordinates)
                .map_err(BrokerReadFailure::ClockCoordinates)
        })
    }

    /// Admit either read in the same initially empty one-shot slot.
    fn select_probe(
        &mut self,
        power: Option<BrokerPowerFence>,
        read: impl FnOnce() -> Result<BrokerReading, BrokerReadFailure> + 'static,
    ) -> bool {
        if self.broker_probe.is_some()
            || self.phase != WorkerPhase::Preparing
            || self.custody != WorkerCustody::Dormant
            || self.admission_closed
            || self.owner.role() != ServiceRole::ClockBroker
        {
            return false;
        }
        self.broker_probe = Some(BrokerProbe::Pending {
            read: Box::new(read),
            power,
        });
        true
    }
}

impl<R: StatusReporter<Error = NativeError>, P: Worker, W: EventWriter> ServiceInvocation<R, P, W> {
    /// Take the sole selected read under the invocation's active Running charge.
    /// An in-flight read is synchronous and retains that charge until return.
    /// Only an admissible bracket can publish raw words to the historical receipt.
    pub(super) fn poll_broker_probe(
        &mut self,
        clock: &mut impl Clock,
        notification: &StopNotification,
    ) {
        if self.phase != WorkerPhase::Running
            || self.custody != WorkerCustody::Active
            || self.permit.is_none()
            || self.owner.state() != Some(ServiceState::Running)
            || self.owner.acknowledged().map(ServiceStatus::state) != Some(ServiceState::Running)
        {
            return;
        }
        let Some(deadline) = self.run_deadline else {
            return;
        };
        let Some(BrokerProbe::Pending { read, power }) = self
            .broker_probe
            .take_if(|probe: &mut BrokerProbe| matches!(probe, BrokerProbe::Pending { .. }))
        else {
            return;
        };

        let observation: BrokerObservation =
            match self.broker_bracket_edge(clock, notification, deadline) {
                Err(reason) => BrokerObservation::Retired {
                    reason,
                    native_failure: None,
                },
                Ok(_)
                    if power
                        .as_ref()
                        .is_some_and(|fence: &BrokerPowerFence| !fence.can_start()) =>
                {
                    BrokerObservation::Retired {
                        reason: Retirement::PowerContext,
                        native_failure: None,
                    }
                }
                Ok(before) => {
                    let result: Result<BrokerReading, BrokerReadFailure> = read();
                    // The power fence's successful compare-and-exchange is the
                    // publication decision. Later delivery leaves history intact.
                    let after: Result<Instant, Retirement> = self
                        .broker_bracket_edge(clock, notification, deadline)
                        .and_then(|after: Instant| {
                            if power.is_none_or(BrokerPowerFence::report) {
                                Ok(after)
                            } else {
                                Err(Retirement::PowerContext)
                            }
                        });
                    match after {
                        Err(reason) => BrokerObservation::Retired {
                            reason,
                            native_failure: result.err(),
                        },
                        Ok(after) => match result {
                            Ok(raw) => BrokerObservation::Observed { before, after, raw },
                            Err(code) => BrokerObservation::NativeFailure(code),
                        },
                    }
                }
            };
        self.broker_probe = Some(BrokerProbe::Finished(observation));
    }

    /// Copy the sole historical probe result, without completing work or changing state.
    pub(super) const fn observed_broker_probe(&self) -> Option<BrokerObservation> {
        match &self.broker_probe {
            Some(BrokerProbe::Finished(observation)) => Some(*observation),
            Some(BrokerProbe::Pending { .. }) | None => None,
        }
    }
}

/// Invoke the existing Windows-only leaf and project only its three raw outputs.
///
/// # Errors
/// Return the original Win32 code captured by the native leaf, including zero.
#[cfg(windows)]
pub(in crate::host) fn read_native_adjustment() -> Result<RawAdjustment, u32> {
    let native: windows_platform_native::ClockAdjustment =
        windows_platform_native::read_clock_adjustment()?;
    Ok(RawAdjustment {
        adjustment: native.adjustment(),
        increment: native.increment(),
        periodic_adjustment_disabled: native.periodic_adjustment_disabled(),
    })
}
