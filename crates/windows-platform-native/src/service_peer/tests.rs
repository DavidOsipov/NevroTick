// SPDX-License-Identifier: GPL-2.0-only
//! Independent ordered examples for the shared observation kernel.

use core::num::NonZeroU32;
use std::{cell::RefCell, collections::VecDeque};

use super::{check_process, observe_process};
use crate::{ReportedServiceStatus, RunningServiceRefusal};

/// Observed calls, including the PID supplied to the open and liveness operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Call {
    /// Read the retained service.
    Status,
    /// Open the reported PID.
    Open(NonZeroU32),
    /// Poll the process already opened.
    Live(NonZeroU32),
}

/// Distinct source faults and refusals for deterministic controls.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Fault {
    /// Status query refusal.
    Status,
    /// Native process open refusal.
    Open,
    /// Process reaped before this wait.
    Exited,
    /// Report did not describe an eligible process.
    Ineligible(RunningServiceRefusal),
    /// Different eligible PID was observed.
    Changed,
    /// An unplanned call exhausted its fixture input.
    Unexpected,
}

/// A caller-observed sequence independent of the production ordering.
struct Probe {
    /// Actual calls in order.
    calls: RefCell<Vec<Call>>,
    /// Next independent SCM record or refusal.
    statuses: RefCell<VecDeque<Result<ReportedServiceStatus, Fault>>>,
    /// The setup result of the only permitted open.
    open: Result<(), Fault>,
    /// Next independent liveness result.
    live: RefCell<VecDeque<Result<(), Fault>>>,
}

impl Probe {
    /// Set the complete status and liveness sequences without performing I/O.
    fn new(
        statuses: impl IntoIterator<Item = Result<ReportedServiceStatus, Fault>>,
        open: Result<(), Fault>,
        live: impl IntoIterator<Item = Result<(), Fault>>,
    ) -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
            statuses: RefCell::new(statuses.into_iter().collect()),
            open,
            live: RefCell::new(live.into_iter().collect()),
        }
    }

    /// Supply one actual checked status projection at a time.
    /// # Errors
    /// Propagate the planned status/refusal or flag an unexpected call.
    fn status(&self) -> Result<NonZeroU32, Fault> {
        self.calls.borrow_mut().push(Call::Status);
        let status: ReportedServiceStatus = self
            .statuses
            .borrow_mut()
            .pop_front()
            .ok_or(Fault::Unexpected)??;
        status.reported_running_process().map_err(Fault::Ineligible)
    }

    /// Record which process was opened on this attempt.
    /// # Errors
    /// Return the configured open refusal.
    fn open(&self, pid: NonZeroU32) -> Result<NonZeroU32, Fault> {
        self.calls.borrow_mut().push(Call::Open(pid));
        self.open.map(|()| pid)
    }

    /// Record the observed process and supply its next liveness result.
    /// # Errors
    /// Propagate the planned exit or flag an unexpected call.
    fn live(&self, pid: NonZeroU32) -> Result<(), Fault> {
        self.calls.borrow_mut().push(Call::Live(pid));
        self.live
            .borrow_mut()
            .pop_front()
            .ok_or(Fault::Unexpected)?
    }

    /// Run acquisition using the same sequence kernel as the native owner.
    /// # Errors
    /// Preserve the observed status, open or liveness refusal.
    fn observe(&self) -> Result<NonZeroU32, Fault> {
        observe_process(
            || self.status(),
            |pid: NonZeroU32| self.open(pid),
            |pid: &NonZeroU32| self.live(*pid),
            Fault::Changed,
        )
    }

    /// Run the later observation on the retained candidate.
    /// # Errors
    /// Preserve the observed status or liveness refusal.
    fn reobserve(&self, pid: NonZeroU32) -> Result<(), Fault> {
        check_process(
            pid,
            &pid,
            || self.status(),
            |pid: &NonZeroU32| self.live(*pid),
            Fault::Changed,
        )
    }
}

/// The first valid nonzero process identifier.
const FIRST: NonZeroU32 = NonZeroU32::MIN;
/// Literal Windows own-process Running status for the first candidate.
const RUNNING: ReportedServiceStatus = ReportedServiceStatus::new(0x10, 4, FIRST.get());
/// A stopped report that still mentions that PID.
const STOPPED: ReportedServiceStatus = ReportedServiceStatus::new(0x10, 1, FIRST.get());
/// A Running report for a different eligible PID.
const REPLACED: ReportedServiceStatus = ReportedServiceStatus::new(0x10, 4, 2);

/// Retain one open process through initial and later checks.
/// # Panics
/// Fail on changed order, an extra open or use of a different process.
#[test]
fn one_open_with_fresh_status_between_live_observations() {
    let probe: Probe = Probe::new([Ok(RUNNING); 3], Ok(()), [Ok(()); 4]);
    let retained: Result<NonZeroU32, Fault> = probe.observe();
    assert_eq!(retained, Ok(FIRST));
    assert_eq!(retained.and_then(|pid| probe.reobserve(pid)), Ok(()));
    assert_eq!(
        probe.calls.borrow().as_slice(),
        [
            Call::Status,
            Call::Open(FIRST),
            Call::Live(FIRST),
            Call::Status,
            Call::Live(FIRST),
            Call::Live(FIRST),
            Call::Status,
            Call::Live(FIRST),
        ]
    );
}

/// A stopped, shared or zero-PID report must never lead to opening a process.
/// # Panics
/// Fail on an open or on any substituted refusal.
#[test]
fn ineligible_status_never_opens_a_process() {
    let cases: [(ReportedServiceStatus, RunningServiceRefusal); 3] = [
        (STOPPED, RunningServiceRefusal::NotRunning),
        (
            ReportedServiceStatus::new(0x20, 4, FIRST.get()),
            RunningServiceRefusal::NotOwnProcess,
        ),
        (
            ReportedServiceStatus::new(0x10, 4, 0),
            RunningServiceRefusal::MissingProcessId,
        ),
    ];
    for (status, reason) in cases {
        let probe: Probe = Probe::new([Ok(status)], Ok(()), []);
        assert_eq!(probe.observe(), Err(Fault::Ineligible(reason)));
        assert_eq!(probe.calls.borrow().as_slice(), [Call::Status]);
    }
}

/// Refuse a replacement or stopped report without polling or reopening the replacement.
/// # Panics
/// Fail if the first observed difference is missed at either receiving check.
#[test]
fn changed_status_refuses_initial_and_retained_process() {
    let cases: [(ReportedServiceStatus, Fault); 2] = [
        (REPLACED, Fault::Changed),
        (
            STOPPED,
            Fault::Ineligible(RunningServiceRefusal::NotRunning),
        ),
    ];
    for (status, refusal) in cases {
        let initial: Probe = Probe::new([Ok(RUNNING), Ok(status)], Ok(()), [Ok(())]);
        assert_eq!(initial.observe(), Err(refusal));
        assert_eq!(
            initial.calls.borrow().as_slice(),
            [
                Call::Status,
                Call::Open(FIRST),
                Call::Live(FIRST),
                Call::Status
            ]
        );

        let later: Probe = Probe::new([Ok(RUNNING), Ok(RUNNING), Ok(status)], Ok(()), [Ok(()); 3]);
        assert_eq!(
            later.observe().and_then(|pid| later.reobserve(pid)),
            Err(refusal)
        );
        assert_eq!(
            later.calls.borrow().as_slice(),
            [
                Call::Status,
                Call::Open(FIRST),
                Call::Live(FIRST),
                Call::Status,
                Call::Live(FIRST),
                Call::Live(FIRST),
                Call::Status
            ]
        );
    }
}

/// Refuse an exited process both before and after the status sample.
/// # Panics
/// Fail if either liveness refusal is missed or an extra observation occurs.
#[test]
fn process_exit_invalidates_the_binding() {
    let early: Probe = Probe::new([Ok(RUNNING)], Ok(()), [Err(Fault::Exited)]);
    assert_eq!(early.observe(), Err(Fault::Exited));
    assert_eq!(
        early.calls.borrow().as_slice(),
        [Call::Status, Call::Open(FIRST), Call::Live(FIRST)]
    );

    let late: Probe = Probe::new([Ok(RUNNING); 2], Ok(()), [Ok(()), Err(Fault::Exited)]);
    assert_eq!(late.observe(), Err(Fault::Exited));
    assert_eq!(
        late.calls.borrow().as_slice(),
        [
            Call::Status,
            Call::Open(FIRST),
            Call::Live(FIRST),
            Call::Status,
            Call::Live(FIRST)
        ]
    );
}

/// Preserve an actual source failure rather than observing another object.
/// # Panics
/// Fail on a substituted result or activity after the first refusal.
#[test]
fn original_query_and_open_refusals_stop_observation() {
    let query: Probe = Probe::new([Err(Fault::Status)], Ok(()), []);
    assert_eq!(query.observe(), Err(Fault::Status));
    assert_eq!(query.calls.borrow().as_slice(), [Call::Status]);

    let open: Probe = Probe::new([Ok(RUNNING)], Err(Fault::Open), []);
    assert_eq!(open.observe(), Err(Fault::Open));
    assert_eq!(
        open.calls.borrow().as_slice(),
        [Call::Status, Call::Open(FIRST)]
    );

    let confirm: Probe = Probe::new([Ok(RUNNING), Err(Fault::Status)], Ok(()), [Ok(())]);
    assert_eq!(confirm.observe(), Err(Fault::Status));
    assert_eq!(
        confirm.calls.borrow().as_slice(),
        [
            Call::Status,
            Call::Open(FIRST),
            Call::Live(FIRST),
            Call::Status
        ]
    );
}
