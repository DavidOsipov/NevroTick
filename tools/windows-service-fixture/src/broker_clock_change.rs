// SPDX-License-Identifier: GPL-2.0-only
//! Broker-only SCM clock and power delivery with capture revision checks.

use std::{
    num::NonZeroU64,
    sync::atomic::{AtomicU64, Ordering::SeqCst},
    time::{Duration, Instant},
};

use windows_platform::{scm::ScmControl, scm::StopNotification, service::ServiceState};

#[path = "broker_clock_continuity.rs"]
mod continuity;

pub use continuity::SampledClockDiscontinuity;

/// Initial state before the first attempted Running report.
const UNARMED: u64 = 0;
/// Sticky retirement, including counter exhaustion.
const CLOSED: u64 = 1;
/// The low bit of a live state records a delivered suspend awaiting resume.
const SUSPENDED: u64 = 1;
/// First awake revision, encoded with the low bit clear.
const FIRST_AWAKE: u64 = 2;
/// Extended SCM control number from `winsvc.h`.
const TIME_CHANGE: u32 = 0x10;
/// Accepted-control bit corresponding to the extended time-change callback.
const ACCEPT_TIME_CHANGE: u32 = 0x200;
/// Extended SCM power-control number from `winsvc.h`.
const POWER_EVENT: u32 = 0x0d;
/// Accepted-control bit for the extended power callback.
const ACCEPT_POWER_EVENT: u32 = 0x40;
/// `PBT_APMSUSPEND` identifies impending suspension.
const SUSPENDING: u32 = 0x04;
/// `PBT_APMRESUMEAUTOMATIC` identifies a resumed execution context.
const AUTOMATIC_RESUME: u32 = 0x12;

/// Only the events that can change this broker's observation context.
#[derive(Clone, Copy)]
enum ClockNotice {
    /// The system wall clock changed.
    TimeChange,
    /// Withdraw observation admission until an automatic resume is delivered.
    Suspend,
    /// Renew observation admission after invalidating the preceding revision.
    Resume,
}

/// Action to take after a fixed handler branch has handled an SCM control.
pub(super) struct ControlDisposition {
    /// Return this acknowledgement to SCM without inferring owner completion.
    pub(super) status: u32,
    /// Publish the existing bounded wake after the mailbox is updated.
    pub(super) wake: bool,
}

/// Decode the designated handler's controls without inspecting any event data.
pub(super) fn deliver(
    changes: Option<&BrokerClockChanges>,
    stop: &StopNotification,
    raw: u32,
    event: u32,
) -> ControlDisposition {
    let notice: Option<ClockNotice> = match (raw, event) {
        (TIME_CHANGE, _) => Some(ClockNotice::TimeChange),
        (POWER_EVENT, SUSPENDING) => Some(ClockNotice::Suspend),
        (POWER_EVENT, AUTOMATIC_RESUME) => Some(ClockNotice::Resume),
        _ => None,
    };
    if let Some((changes, notice)) = changes.zip(notice)
        && changes.notify(notice)
    {
        return ControlDisposition {
            status: 0,
            wake: true,
        };
    }
    if raw == 1
        && let Some(changes) = changes
    {
        // Order closure against a final raw read decision before waking for Stop.
        changes.close();
    }
    let decoded: ScmControl = stop.handle_control(raw);
    ControlDisposition {
        status: decoded.return_code(),
        wake: decoded == ScmControl::Stop,
    }
}

/// Choose the designated broker's extended bit at the actual status boundary.
fn report_controls(changes: Option<&BrokerClockChanges>, state: ServiceState) -> u32 {
    let Some(changes) = changes else {
        return 0;
    };
    match state {
        ServiceState::Running if changes.arm() || changes.is_live() => {
            ACCEPT_TIME_CHANGE | ACCEPT_POWER_EVENT
        }
        ServiceState::StopPending | ServiceState::Stopped => {
            changes.close();
            0
        }
        ServiceState::Running | ServiceState::StartPending => 0,
    }
}

/// Arm before the actual acknowledgement attempt, and revoke on any refusal.
///
/// # Errors
/// Return the unchanged error from the status owner after closing broker admission.
pub(super) fn report_status<E>(
    changes: Option<&BrokerClockChanges>,
    state: ServiceState,
    send: impl FnOnce(u32) -> Result<(), E>,
) -> Result<(), E> {
    let result: Result<(), E> = send(report_controls(changes, state));
    if result.is_err()
        && let Some(changes) = changes
    {
        changes.close();
    }
    result
}

/// Process-lifetime mailbox for one registered broker invocation.
pub(super) struct BrokerClockChanges {
    /// Sentinels or a positive revision in the upper bits with one suspension bit.
    revision: AtomicU64,
}

impl BrokerClockChanges {
    /// Prepare the mailbox without enabling callback delivery.
    pub(super) const fn new() -> Self {
        Self {
            revision: AtomicU64::new(UNARMED),
        }
    }

    /// Arm before submitting the first Running mask so early delivery is retained.
    /// The actual observation still needs the lifecycle's Running acknowledgement.
    pub(super) fn arm(&self) -> bool {
        self.revision
            .compare_exchange(UNARMED, FIRST_AWAKE, SeqCst, SeqCst)
            .is_ok()
    }

    /// Keep both notification classes advertised while a live broker is suspended.
    fn is_live(&self) -> bool {
        self.revision.load(SeqCst) > CLOSED
    }

    /// Invalidate the previous revision and change phase in one atomic update.
    /// Exhaustion closes the mailbox rather than reusing an observation revision.
    fn notify(&self, notice: ClockNotice) -> bool {
        self.revision
            .try_update(SeqCst, SeqCst, |state: u64| match state {
                UNARMED | CLOSED => None,
                current => {
                    let phase: u64 = match notice {
                        ClockNotice::TimeChange => current & SUSPENDED,
                        ClockNotice::Suspend => SUSPENDED,
                        ClockNotice::Resume => 0,
                    };
                    Some(
                        (current & !SUSPENDED)
                            .checked_add(2)
                            .map_or(CLOSED, |next: u64| next | phase),
                    )
                }
            })
            .is_ok()
    }

    /// Revoke before Stop delivery, a failing status handoff, or terminal reporting.
    pub(super) fn close(&self) {
        self.revision.store(CLOSED, SeqCst);
    }

    /// Bind one read decision to this exact awake mailbox and observed revision.
    pub(super) fn revision(&self) -> Option<ClockRevision<'_>> {
        let state: u64 = self.revision.load(SeqCst);
        if state & SUSPENDED != 0 {
            return None;
        }
        NonZeroU64::new(state >> 1).map(|number: NonZeroU64| ClockRevision {
            owner: self,
            number,
        })
    }
}

/// A single observation's original, private process-local revision.
pub(super) struct ClockRevision<'a> {
    /// The actual mailbox, regardless of numeric equality with another invocation.
    owner: &'a BrokerClockChanges,
    /// Positive revision observed before the read.
    number: NonZeroU64,
}

impl ClockRevision<'_> {
    /// Retain the original number in this invocation's historical report.
    pub(super) const fn number(&self) -> NonZeroU64 {
        self.number
    }

    /// Order the decision against notification and shutdown on the original mailbox.
    pub(super) fn admits(&self) -> bool {
        let expected: u64 = self.number.get() << 1;
        self.owner
            .revision
            .compare_exchange(expected, expected, SeqCst, SeqCst)
            .is_ok()
    }

    /// Invalidate only the awake context whose own samples disagree.
    ///
    /// # Errors
    /// Return refusal when a concurrent notification or shutdown already won.
    fn invalidate(&self) -> Result<Option<NonZeroU64>, ()> {
        let expected: u64 = self.number.get() << 1;
        let replacement: u64 = expected.checked_add(2).unwrap_or(CLOSED);
        self.owner
            .revision
            .compare_exchange(expected, replacement, SeqCst, SeqCst)
            .map_err(|_| ())?;
        Ok(NonZeroU64::new(replacement >> 1))
    }
}

/// Why a broker clock read could not enter historical capture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClockReadRetirement {
    /// A clock change, power notice or receiver close crossed admission.
    Generation,
    /// Sticky Stop closed admission.
    Stop,
    /// The original Running deadline was reached.
    Deadline,
    /// Monotonic time became unusable for ordinary work.
    Clock,
    /// The separate one-shot power observer retired the selected fixture read.
    PowerContext,
    /// The original raw bracket disagreed with its preceding same-revision bracket.
    Discontinuity(SampledClockDiscontinuity),
}

/// One historical read attempt from the selected broker's fixed slot.
#[must_use = "retain the actual observation or its refusal"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BrokerClockRead<T, E> {
    /// The original bracket and result reached the serialized decision together.
    Completed {
        /// Process-local historical revision number.
        revision: NonZeroU64,
        /// Owner time before the native call.
        before: Instant,
        /// Owner time after synchronous return.
        after: Instant,
        /// Checked raw sample or exact staged native error.
        result: Result<T, E>,
    },
    /// The bracket was denied before it could publish a raw success.
    Retired {
        /// Original attempted revision, even if another revision has since arrived.
        revision: NonZeroU64,
        /// Original typed reason the owner refused the read.
        reason: ClockReadRetirement,
        /// Exact native error when the getter had already returned a failure.
        native_failure: Option<E>,
    },
}

/// Latest observed sampled clock break retained independently of later reads.
/// This historical evidence identifies a contradiction in raw local readings.
#[must_use = "retain the observed reason for clock context invalidation"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SampledClockBreak {
    /// Original process-local revision invalidated by the comparison.
    pub revision: NonZeroU64,
    /// Owner clock observed after the conflicting synchronous read.
    pub observed_at: Instant,
    /// Exact raw coordinate contradiction.
    pub reason: SampledClockDiscontinuity,
}

/// Read-only cadence of the fixed lab broker, under its original Running limit.
const SAMPLE_INTERVAL: Duration = Duration::from_secs(1);

/// One prior sample and one absolute due time for the quiet observation path.
struct Sampling<T> {
    /// Previous successful comparison input in the current sampled revision.
    previous: Option<T>,
    /// Next allowed quiet read measured from the last observed completion.
    due: Option<Instant>,
    /// Pure comparison of consecutive checked coordinates.
    compare: fn(&T, &T) -> Option<SampledClockDiscontinuity>,
    /// Last sampled invalidation, retaining evidence after reacquisition.
    last_break: Option<SampledClockBreak>,
}

/// Serialize at most one attempted sample per owner poll without retaining a queue.
pub(super) struct BrokerClockReader<T, E> {
    /// This configured service's single registered mailbox.
    changes: &'static BrokerClockChanges,
    /// Sole owner-thread raw observation adapter.
    getter: fn() -> Result<T, E>,
    /// Last attempted revision, including retired and native-failed reads.
    attempted: Option<NonZeroU64>,
    /// Most recent attempt. Subsequent events alone never rewrite it.
    history: Option<BrokerClockRead<T, E>>,
    /// Enabled only in the read-only broker composition and its scoped tests.
    sampling: Option<Sampling<T>>,
}

impl<T: Copy, E: Copy> BrokerClockReader<T, E> {
    /// Bind the configured getter without attempting a read or arming its mailbox.
    pub(super) const fn new(
        changes: &'static BrokerClockChanges,
        getter: fn() -> Result<T, E>,
    ) -> Self {
        Self {
            changes,
            getter,
            attempted: None,
            history: None,
            sampling: None,
        }
    }

    /// Select the lab cadence and pure comparison before entering the host.
    pub(super) fn with_sampling(
        mut self,
        compare: fn(&T, &T) -> Option<SampledClockDiscontinuity>,
    ) -> Self {
        self.sampling = Some(Sampling {
            previous: None,
            due: None,
            compare,
            last_break: None,
        });
        self
    }

    /// Attempt at most one due observation within the caller's Running charge.
    /// Each edge freshly applies the owning invocation's Stop and time guards.
    pub(super) fn poll(
        &mut self,
        now: Instant,
        mut edge: impl FnMut() -> Result<Instant, ClockReadRetirement>,
    ) {
        let changes: &'static BrokerClockChanges = self.changes;
        let Some(revision) = changes.revision() else {
            return;
        };
        let number: NonZeroU64 = revision.number();
        if self.attempted == Some(number) {
            if self
                .sampling
                .as_ref()
                .and_then(|sampling: &Sampling<T>| sampling.due)
                .is_none_or(|due: Instant| now < due)
            {
                return;
            }
        } else if let Some(sampling) = &mut self.sampling {
            sampling.previous = None;
        }
        self.attempted = Some(number);
        self.history = Some(self.capture(&revision, &mut edge));
    }

    /// Limit an ordinary Running park without replacing the owner's cutoff.
    pub(super) fn sample_wait(&self, now: Instant) -> Option<Duration> {
        let sampling: &Sampling<T> = self.sampling.as_ref()?;
        let revision: ClockRevision<'_> = self.changes.revision()?;
        if self.attempted != Some(revision.number()) {
            return Some(Duration::ZERO);
        }
        sampling
            .due
            .map(|due: Instant| due.checked_duration_since(now).unwrap_or(Duration::ZERO))
    }

    /// Keep the native result bound to the original revision and both owner edges.
    fn capture(
        &mut self,
        revision: &ClockRevision<'_>,
        edge: &mut impl FnMut() -> Result<Instant, ClockReadRetirement>,
    ) -> BrokerClockRead<T, E> {
        let before: Instant = match edge().and_then(|time: Instant| {
            if revision.admits() {
                Ok(time)
            } else {
                Err(ClockReadRetirement::Generation)
            }
        }) {
            Ok(before) => before,
            Err(reason) => return self.retire(revision.number(), reason, None),
        };
        let number: NonZeroU64 = revision.number();
        let result: Result<T, E> = (self.getter)();
        let after: Instant = match edge() {
            Ok(after) => after,
            Err(reason) => return self.retire(number, reason, result.err()),
        };
        if let Some(sampling) = &mut self.sampling {
            sampling.due = after.checked_add(SAMPLE_INTERVAL);
            if let (Some(previous), Ok(sample)) = (&sampling.previous, &result)
                && let Some(reason) = (sampling.compare)(previous, sample)
            {
                match revision.invalidate() {
                    Ok(replacement) => {
                        self.attempted = replacement;
                        sampling.last_break = Some(SampledClockBreak {
                            revision: number,
                            observed_at: after,
                            reason,
                        });
                        return self.retire(
                            number,
                            ClockReadRetirement::Discontinuity(reason),
                            None,
                        );
                    }
                    Err(()) => return self.retire(number, ClockReadRetirement::Generation, None),
                }
            }
        }
        if !revision.admits() {
            return self.retire(number, ClockReadRetirement::Generation, result.err());
        }
        if let Some(sampling) = &mut self.sampling {
            sampling.previous = result.ok();
        }
        BrokerClockRead::Completed {
            revision: number,
            before,
            after,
            result,
        }
    }

    /// Preserve the original refusal without carrying a baseline across the gap.
    fn retire(
        &mut self,
        revision: NonZeroU64,
        reason: ClockReadRetirement,
        native_failure: Option<E>,
    ) -> BrokerClockRead<T, E> {
        if let Some(sampling) = &mut self.sampling {
            sampling.previous = None;
        }
        BrokerClockRead::Retired {
            revision,
            reason,
            native_failure,
        }
    }

    /// Keep the last actual historical attempt without observing a new revision.
    pub(super) const fn history(&self) -> Option<BrokerClockRead<T, E>> {
        self.history
    }

    /// Select a successful observation from this reader's own awake revision.
    /// The owner must recheck the returned revision at its live decision edge.
    #[cfg(test)]
    pub(super) fn observed_awake_revision(&self) -> Option<ClockRevision<'static>> {
        let changes: &'static BrokerClockChanges = self.changes;
        let current: ClockRevision<'static> = changes.revision()?;
        matches!(
            self.history,
            Some(BrokerClockRead::Completed {
                revision,
                result: Ok(_),
                ..
            }) if revision == current.number()
        )
        .then_some(current)
    }

    /// Observe whether the original registered mailbox still admits status reads.
    #[cfg(test)]
    pub(super) fn status_source_open(&self) -> bool {
        self.changes.is_live()
    }

    /// Preserve the latest sampled reason when a fresh read replaces the receipt.
    pub(super) fn last_discontinuity(&self) -> Option<SampledClockBreak> {
        self.sampling
            .as_ref()
            .and_then(|sampling: &Sampling<T>| sampling.last_break)
    }

    /// Close the read source before the owner proceeds to `StopPending`.
    pub(super) fn close(&self) {
        self.changes.close();
    }
}

/// The native broker's checked read with its original staged Win32 failure.
#[cfg(windows)]
pub(super) type NativeBrokerReader = BrokerClockReader<
    windows_platform_native::ClockCoordinates,
    windows_platform_native::ClockReadError,
>;

/// Select the real Windows reader and comparison for the fixed broker.
#[cfg(windows)]
pub(super) fn sampled_native_reader(changes: &'static BrokerClockChanges) -> NativeBrokerReader {
    NativeBrokerReader::new(changes, windows_platform_native::read_clock_coordinates)
        .with_sampling(continuity::compare_native)
}

/// Host-only scripted read data, using the same lifecycle and receiving code.
#[cfg(all(not(windows), test))]
pub(super) type NativeBrokerReader = BrokerClockReader<u64, u32>;

#[cfg(test)]
mod tests {
    use std::{
        num::NonZeroU64,
        sync::atomic::{AtomicU32, Ordering::SeqCst},
        time::{Duration, Instant},
    };

    use windows_platform::{scm::StopNotification, service::ServiceState};

    use super::{
        BrokerClockChanges, BrokerClockRead, BrokerClockReader, ClockNotice, ClockReadRetirement,
        SAMPLE_INTERVAL, SampledClockDiscontinuity, deliver, report_status,
    };

    /// A bound reporter advertises only its live mask and retains early delivery.
    ///
    /// # Panics
    /// Fails if an unsupported input is accepted, retirement reopens, or counters wrap.
    #[test]
    fn time_change_controls_are_role_and_lifecycle_bound() {
        let changes: BrokerClockChanges = BrokerClockChanges::new();
        let stop: StopNotification = StopNotification::new();
        assert_eq!(deliver(Some(&changes), &stop, 16, 0).status, 120);
        assert_eq!(deliver(None, &stop, 16, 0).status, 120);
        let start: Result<(), ()> =
            report_status(Some(&changes), ServiceState::StartPending, |extra: u32| {
                assert_eq!(extra, 0);
                assert!(!deliver(Some(&changes), &stop, 16, 0).wake);
                Ok(())
            });
        assert_eq!(start, Ok(()));
        let failed: Result<(), u32> =
            report_status(Some(&changes), ServiceState::Running, |extra: u32| {
                assert_eq!(extra, 0x240);
                assert!(deliver(Some(&changes), &stop, 16, 0).wake);
                assert_eq!(changes.revision().map(|r| r.number()), NonZeroU64::new(2));
                Err(0)
            });
        assert_eq!(failed, Err(0));
        assert!(changes.revision().is_none());
        assert!(!changes.arm());
        assert_eq!(deliver(Some(&changes), &stop, 16, 0).status, 120);
        let last: BrokerClockChanges = BrokerClockChanges::new();
        last.revision.store(u64::MAX - 1, SeqCst);
        assert!(last.notify(ClockNotice::TimeChange));
        assert!(last.revision().is_none());
        assert!(!last.notify(ClockNotice::TimeChange));
        assert!(!last.arm());
    }

    /// One independent fixture source dedicated to the read-failure control.
    static CROSSING: BrokerClockChanges = BrokerClockChanges::new();

    /// Return a typed zero error while a new notice crosses the original call.
    ///
    /// # Errors
    /// Always return the original scripted error code zero.
    ///
    /// # Panics
    /// Fails if the synthetic notification does not reach the original mailbox.
    fn crossing_failure() -> Result<u64, u32> {
        assert!(CROSSING.notify(ClockNotice::TimeChange));
        Err(0)
    }

    /// A crossing read preserves its actual error and spent revision.
    ///
    /// # Panics
    /// Fails if read completion or history is invented after replacement or close.
    #[test]
    fn crossing_read_keeps_the_original_failure_under_its_revision() {
        let mut reader: BrokerClockReader<u64, u32> =
            BrokerClockReader::new(&CROSSING, crossing_failure);
        assert!(CROSSING.arm());
        let now: Instant = Instant::now();
        reader.poll(now, || Ok(now));
        let historical: Option<BrokerClockRead<u64, u32>> = Some(BrokerClockRead::Retired {
            revision: NonZeroU64::MIN,
            reason: ClockReadRetirement::Generation,
            native_failure: Some(0),
        });
        assert_eq!(reader.history(), historical);
        reader.close();
        reader.poll(now, || Ok(now));
        assert_eq!(reader.history(), historical);
    }

    /// Separate registered state for a concurrent-notice fixture.
    static NOTICE_RACE: BrokerClockChanges = BrokerClockChanges::new();
    /// Mark the actual two native-like calls in the concurrently replaced revision.
    static RACE_READS: AtomicU32 = AtomicU32::new(0);

    /// Supply ordered synthetic wall ticks with an observed regression.
    /// # Errors
    /// Reject a read outside the two scripted getter calls.
    fn race_read() -> Result<u64, u32> {
        match RACE_READS.fetch_add(1, SeqCst) {
            0 => Ok(80),
            1 => Ok(70),
            _ => Err(99),
        }
    }

    /// A concurrent real notice wins over a sampled invalidation at commit.
    /// # Panics
    /// Fails if the sampler replaces a delivered revision or records an uncommitted break.
    #[test]
    fn newer_notice_wins_over_a_sampled_contradiction() {
        assert!(NOTICE_RACE.arm());
        let now: Instant = Instant::now();
        let due: Instant = now + SAMPLE_INTERVAL;
        let mut reader: BrokerClockReader<u64, u32> =
            BrokerClockReader::new(&NOTICE_RACE, race_read).with_sampling(|a: &u64, b: &u64| {
                (b < a).then_some(SampledClockDiscontinuity::FiletimeReversed)
            });
        reader.poll(now, || Ok(now));
        let mut edges: u32 = 0;
        reader.poll(due, || {
            edges += 1;
            if edges == 2 {
                assert!(NOTICE_RACE.notify(ClockNotice::TimeChange));
            }
            Ok(due)
        });
        assert_eq!(RACE_READS.load(SeqCst), 2);
        assert_eq!(
            NOTICE_RACE.revision().map(|revision| revision.number()),
            NonZeroU64::new(2)
        );
        assert_eq!(reader.last_discontinuity(), None);
        assert_eq!(reader.sample_wait(due), Some(Duration::ZERO));
        assert_eq!(
            reader.history(),
            Some(BrokerClockRead::Retired {
                revision: NonZeroU64::MIN,
                reason: ClockReadRetirement::Generation,
                native_failure: None
            })
        );
    }

    /// Separate live owner for the gap case.
    static GAP: BrokerClockChanges = BrokerClockChanges::new();
    /// Keep the marker sequence local to that owner.
    static GAP_READS: AtomicU32 = AtomicU32::new(0);

    /// A failure interrupts comparable captures even when later values are lower.
    /// # Errors
    /// Preserve the original zero from the second attempted call.
    fn gap_read() -> Result<u64, u32> {
        match GAP_READS.fetch_add(1, SeqCst) {
            0 => Ok(100),
            1 => Err(0),
            _ => Ok(99),
        }
    }

    /// A failed getter clears the comparison baseline and preserves its own error.
    /// # Panics
    /// Fails if comparison crosses the failed sample or the failure is replaced early.
    #[test]
    fn native_failure_breaks_the_sample_pair() {
        assert!(GAP.arm());
        let now: Instant = Instant::now();
        let second: Instant = now + SAMPLE_INTERVAL;
        let third: Instant = second + SAMPLE_INTERVAL;
        let mut reader: BrokerClockReader<u64, u32> = BrokerClockReader::new(&GAP, gap_read)
            .with_sampling(|a: &u64, b: &u64| {
                (b < a).then_some(SampledClockDiscontinuity::FiletimeReversed)
            });
        reader.poll(now, || Ok(now));
        reader.poll(second, || Ok(second));
        assert!(matches!(
            reader.history(),
            Some(BrokerClockRead::Completed { result: Err(0), .. })
        ));
        reader.poll(third, || Ok(third));
        assert!(matches!(
            reader.history(),
            Some(BrokerClockRead::Completed {
                revision: NonZeroU64::MIN,
                result: Ok(99),
                ..
            })
        ));
        assert_eq!(reader.last_discontinuity(), None);
    }

    /// The last valid awake revision can retire without wrapping.
    /// # Errors
    /// Returns the missing-fence assertion if the largest legal state is unavailable.
    /// # Panics
    /// Fails if the penultimate revision cannot advance or the final revision reopens.
    #[test]
    fn sampled_invalidation_closes_at_revision_exhaustion() -> Result<(), &'static str> {
        let changes: BrokerClockChanges = BrokerClockChanges::new();
        changes.revision.store(u64::MAX - 3, SeqCst);
        let previous = changes.revision().ok_or("missing penultimate revision")?;
        assert_eq!(previous.invalidate(), Ok(NonZeroU64::new(u64::MAX >> 1)));
        let last = changes.revision().ok_or("missing last revision")?;
        assert_eq!(last.invalidate(), Ok(None));
        assert!(!last.admits());
        assert!(!changes.arm());
        assert!(changes.revision().is_none());

        Ok(())
    }
}

#[cfg(test)]
#[path = "broker_clock_power_tests.rs"]
mod power_tests;
