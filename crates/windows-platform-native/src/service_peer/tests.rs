// SPDX-License-Identifier: GPL-2.0-only
//! Independent ordered examples for the shared observation kernel.

use core::num::NonZeroU32;
use std::{cell::RefCell, collections::VecDeque};

use super::{
    check_process, check_token_generation, observe_between_pipe_samples, observe_process,
    observe_service_peer,
};
use crate::service_role_sid::{
    TokenInformation, TokenInformationBuffer, TokenModifiedId, read_modified_id,
};
use crate::{ReportedServiceStatus, RunningServiceRefusal};

/// Observed calls, including the PID supplied to the open and liveness operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Call {
    /// Observe the original pipe.
    Pipe,
    /// Observe the primary token.
    Token,
    /// Compare the freshly acquired object with the retained token.
    CompareToken,
    /// Sample the retained token's modification identifier.
    TokenStatistics,
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
    /// The original pipe query returned this native refusal.
    PipeQuery(u32),
    /// Pipe server and service process disagreement.
    WrongPipe,
    /// The fresh primary-token object or retained generation differed.
    ChangedToken,
    /// Querying the current primary token failed.
    TokenQuery,
    /// The original token refused its statistics query.
    TokenStatistics(u32),
    /// The successful statistics response reported an invalid extent.
    MalformedTokenStatistics,
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
            None,
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
/// A separate process identifier observed on the wrong pipe.
const SECOND: NonZeroU32 = FIRST.saturating_add(1);
/// Literal Windows own-process Running status for the first candidate.
const RUNNING: ReportedServiceStatus = ReportedServiceStatus::new(0x10, 4, FIRST.get());
/// A stopped report that still mentions that PID.
const STOPPED: ReportedServiceStatus = ReportedServiceStatus::new(0x10, 1, FIRST.get());
/// A Running report for a different eligible PID.
const REPLACED: ReportedServiceStatus = ReportedServiceStatus::new(0x10, 4, 2);

/// Refuse a wrong initial pipe PID before any candidate process open.
/// # Panics
/// Fail if the open guard accepts a mismatch or prevents the matching control.
#[test]
fn pipe_pid_is_checked_before_process_open() {
    for (pipe, expected) in [(SECOND, Err(Fault::WrongPipe)), (FIRST, Ok((FIRST, 7_u32)))] {
        let probe: Probe = Probe::new([Ok(RUNNING); 3], Ok(()), [Ok(()); 4]);
        let mut pipe_pids: std::array::IntoIter<NonZeroU32, 3> = [pipe, FIRST, FIRST].into_iter();
        let observed: Result<(NonZeroU32, u32), Fault> = observe_service_peer(
            || {
                probe.calls.borrow_mut().push(Call::Pipe);
                pipe_pids.next().ok_or(Fault::Unexpected)
            },
            |original: NonZeroU32| {
                observe_process(
                    || probe.status(),
                    |reported: NonZeroU32| probe.open(reported),
                    |pid: &NonZeroU32| probe.live(*pid),
                    Some((original, Fault::WrongPipe)),
                    Fault::Changed,
                )
            },
            |_pid: &NonZeroU32| {
                probe.calls.borrow_mut().push(Call::Token);
                Ok(7_u32)
            },
            |pid: &NonZeroU32| probe.reobserve(*pid),
            Fault::WrongPipe,
        );
        assert_eq!(observed, expected);
        if expected.is_err() {
            assert_eq!(probe.calls.borrow().as_slice(), [Call::Pipe, Call::Status]);
        } else {
            assert_eq!(
                probe.calls.borrow().as_slice(),
                [
                    Call::Pipe,
                    Call::Status,
                    Call::Open(FIRST),
                    Call::Live(FIRST),
                    Call::Status,
                    Call::Live(FIRST),
                    Call::Pipe,
                    Call::Token,
                    Call::Live(FIRST),
                    Call::Status,
                    Call::Live(FIRST),
                    Call::Pipe,
                ]
            );
        }
    }
}

/// Keep raw pipe-query failure distinct from a successful but mismatched PID.
/// # Panics
/// Fail if the first query error or the observed call prefix changes.
#[test]
fn native_pipe_query_refusal_stops_at_its_original_stage() {
    let healthy: [Call; 12] = [
        Call::Pipe,
        Call::Status,
        Call::Open(FIRST),
        Call::Live(FIRST),
        Call::Status,
        Call::Live(FIRST),
        Call::Pipe,
        Call::Token,
        Call::Live(FIRST),
        Call::Status,
        Call::Live(FIRST),
        Call::Pipe,
    ];
    for (samples, expected, calls) in [
        (
            [Err(Fault::PipeQuery(6)), Ok(FIRST), Ok(FIRST)],
            Err(Fault::PipeQuery(6)),
            1,
        ),
        (
            [Ok(FIRST), Err(Fault::PipeQuery(5)), Ok(FIRST)],
            Err(Fault::PipeQuery(5)),
            7,
        ),
        (
            [Ok(FIRST), Ok(FIRST), Err(Fault::PipeQuery(109))],
            Err(Fault::PipeQuery(109)),
            12,
        ),
        ([Ok(FIRST); 3], Ok((FIRST, 7)), 12),
    ] {
        let probe: Probe = Probe::new([Ok(RUNNING); 3], Ok(()), [Ok(()); 4]);
        let mut samples: std::array::IntoIter<Result<NonZeroU32, Fault>, 3> = samples.into_iter();
        let actual: Result<(NonZeroU32, u32), Fault> = observe_service_peer(
            || {
                probe.calls.borrow_mut().push(Call::Pipe);
                samples.next().ok_or(Fault::Unexpected)?
            },
            |original: NonZeroU32| {
                observe_process(
                    || probe.status(),
                    |reported: NonZeroU32| probe.open(reported),
                    |pid: &NonZeroU32| probe.live(*pid),
                    Some((original, Fault::WrongPipe)),
                    Fault::Changed,
                )
            },
            |_pid: &NonZeroU32| {
                probe.calls.borrow_mut().push(Call::Token);
                Ok(7)
            },
            |pid: &NonZeroU32| probe.reobserve(*pid),
            Fault::WrongPipe,
        );
        assert_eq!(actual, expected);
        assert_eq!(probe.calls.borrow().as_slice(), &healthy[..calls]);
    }
}

/// Exercise the actual sequencing around the original pipe without native effects.
/// # Errors
/// Preserve the scripted pipe, token or service refusal.
fn sample_pipe_and_token(
    probe: &Probe,
    pipe_pids: [NonZeroU32; 2],
    token: Result<u32, Fault>,
) -> Result<u32, Fault> {
    let mut pids: std::array::IntoIter<NonZeroU32, 2> = pipe_pids.into_iter();
    observe_between_pipe_samples(
        FIRST,
        || {
            probe.calls.borrow_mut().push(Call::Pipe);
            pids.next().ok_or(Fault::Unexpected)
        },
        || {
            probe.calls.borrow_mut().push(Call::Token);
            token
        },
        || probe.reobserve(FIRST),
        Fault::WrongPipe,
    )
}

/// Only the original pipe may bracket the token and SCM observations.
/// # Panics
/// Fail on hidden work after an early refusal or publication after a late mismatch.
#[test]
fn peer_observation_brackets_current_process_with_original_pipe() {
    let healthy: Probe = Probe::new([Ok(RUNNING)], Ok(()), [Ok(()); 2]);
    assert_eq!(sample_pipe_and_token(&healthy, [FIRST; 2], Ok(7)), Ok(7));
    let checked: [Call; 6] = [
        Call::Pipe,
        Call::Token,
        Call::Live(FIRST),
        Call::Status,
        Call::Live(FIRST),
        Call::Pipe,
    ];
    assert_eq!(healthy.calls.borrow().as_slice(), checked);

    let early: Probe = Probe::new([], Err(Fault::Open), []);
    assert_eq!(
        sample_pipe_and_token(&early, [SECOND, FIRST], Ok(7)),
        Err(Fault::WrongPipe)
    );
    assert_eq!(early.calls.borrow().as_slice(), [Call::Pipe]);

    let late: Probe = Probe::new([Ok(RUNNING)], Err(Fault::Open), [Ok(()); 2]);
    assert_eq!(
        sample_pipe_and_token(&late, [FIRST, SECOND], Ok(7)),
        Err(Fault::WrongPipe)
    );
    assert_eq!(late.calls.borrow().as_slice(), checked);
}

/// Preserve the first token or service turnover instead of yielding an observation.
/// # Panics
/// Fail on a substituted refusal or an observation after either turnover.
#[test]
fn token_and_service_turnover_prevent_peer_observation() {
    let token_changed: Probe = Probe::new([], Err(Fault::Open), []);
    assert_eq!(
        sample_pipe_and_token(&token_changed, [FIRST; 2], Err(Fault::ChangedToken)),
        Err(Fault::ChangedToken)
    );
    assert_eq!(
        token_changed.calls.borrow().as_slice(),
        [Call::Pipe, Call::Token]
    );

    let replaced: Probe = Probe::new([Ok(REPLACED)], Err(Fault::Open), [Ok(())]);
    assert_eq!(
        sample_pipe_and_token(&replaced, [FIRST; 2], Ok(7)),
        Err(Fault::Changed)
    );
    assert_eq!(
        replaced.calls.borrow().as_slice(),
        [Call::Pipe, Call::Token, Call::Live(FIRST), Call::Status]
    );
}

/// Build a modification sample from independently arranged x64 SDK output.
/// # Errors
/// Preserve the scripted statistics refusal or reject any extent other than 56 bytes.
/// # Panics
/// Fail if the statistics read requests the wrong information class, alignment or capacity.
fn observed_token_generation(
    modified_id: [u8; 8],
    extent: Result<u32, u32>,
) -> Result<TokenModifiedId, Fault> {
    read_modified_id(
        &mut |information: TokenInformation, buffer: TokenInformationBuffer<'_>| {
            let (output, capacity): (&mut [u8], u32) = buffer.into_parts();
            assert_eq!(information, TokenInformation::Statistics);
            assert_eq!(output.len(), 56);
            assert_eq!(output.as_ptr().addr() % 8, 0);
            assert_eq!(capacity, 56);
            output[48..56].copy_from_slice(&modified_id);
            extent
        },
        Fault::TokenStatistics,
        Fault::MalformedTokenStatistics,
    )
}

/// Check object and generation before the remaining SCM and pipe observations.
/// # Errors
/// Return an unexpected refusal of the acquisition statistics fixture.
/// # Panics
/// Fail on a skipped/disordered check, an altered refusal or use of the fresh handle for statistics.
#[test]
fn requires_the_original_token_generation_before_later_peer_observations() -> Result<(), Fault> {
    let unchanged: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let changed_low: [u8; 8] = [1, 3, 3, 4, 5, 6, 7, 8];
    let changed_high: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 9];
    let generation: TokenModifiedId = observed_token_generation(unchanged, Ok(56))?;
    let original: u32 = 81;
    let fresh: u32 = 23;
    let healthy: [Call; 8] = [
        Call::Pipe,
        Call::Token,
        Call::CompareToken,
        Call::TokenStatistics,
        Call::Live(FIRST),
        Call::Status,
        Call::Live(FIRST),
        Call::Pipe,
    ];
    for (object, current_id, extent, outcome, count) in [
        (Ok(true), unchanged, Ok(56), Ok(()), 8),
        (Ok(true), changed_low, Ok(56), Err(Fault::ChangedToken), 4),
        (Ok(true), changed_high, Ok(56), Err(Fault::ChangedToken), 4),
        (Ok(false), unchanged, Err(5), Err(Fault::ChangedToken), 3),
        (
            Err(Fault::TokenQuery),
            unchanged,
            Err(5),
            Err(Fault::TokenQuery),
            2,
        ),
        (
            Ok(true),
            unchanged,
            Err(5),
            Err(Fault::TokenStatistics(5)),
            4,
        ),
        (
            Ok(true),
            unchanged,
            Ok(55),
            Err(Fault::MalformedTokenStatistics),
            4,
        ),
        (
            Ok(true),
            unchanged,
            Ok(57),
            Err(Fault::MalformedTokenStatistics),
            4,
        ),
    ] {
        let probe: Probe = Probe::new([Ok(RUNNING)], Err(Fault::Open), [Ok(()); 2]);
        let observed: Result<(), Fault> = observe_between_pipe_samples(
            FIRST,
            || {
                probe.calls.borrow_mut().push(Call::Pipe);
                Ok(FIRST)
            },
            || {
                check_token_generation(
                    &original,
                    generation,
                    || {
                        probe.calls.borrow_mut().push(Call::Token);
                        object.map(|_: bool| fresh)
                    },
                    |retained: &u32, current: &u32| {
                        probe.calls.borrow_mut().push(Call::CompareToken);
                        assert_eq!((*retained, *current), (original, fresh));
                        object == Ok(true)
                    },
                    |token: &u32| {
                        probe.calls.borrow_mut().push(Call::TokenStatistics);
                        assert_eq!(*token, original);
                        observed_token_generation(current_id, extent)
                    },
                    Fault::ChangedToken,
                )
            },
            || probe.reobserve(FIRST),
            Fault::WrongPipe,
        );
        assert_eq!(observed, outcome);
        assert_eq!(probe.calls.borrow().as_slice(), &healthy[..count]);
    }
    Ok(())
}

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
