// SPDX-License-Identifier: GPL-2.0-only
//! Process custody of one actual blocking resolver thread.

use std::{task::Poll, time::Duration};

use super::{AddressRoundRefreshOutcome, RefreshPlan};
use crate::{
    PilotError,
    native_network_job::{DispatchAdmission, NativeJob, NativeJobError},
};

/// Timer interval for checking actual completion without blocking the Engine.
const COMPLETION_CHECK: Duration = Duration::from_millis(5);

/// One outcome produced by the selected resolver operation.
type Answer = Result<RefreshPlan, AddressRoundRefreshOutcome>;

/// Resolver access to the shared process-owned native network lane.
pub(super) struct NativeResolverJob {
    /// Retains the original typed answer until actual join or retirement.
    job: NativeJob<Answer>,
}

impl NativeResolverJob {
    /// Reserve the shared process slot and hand it the actual resolver thread.
    ///
    /// # Errors
    /// Report occupied custody or a failure to start the owned resolver thread.
    pub(super) fn start(
        operation: impl FnOnce(DispatchAdmission) -> Answer + Send + 'static,
    ) -> Result<Self, PilotError> {
        NativeJob::start("nevrotick-dns", operation)
            .map(|job| Self { job })
            .map_err(|error| match error {
                NativeJobError::Busy => PilotError::ResolutionBusy,
                NativeJobError::Failed => PilotError::ResolverTask,
            })
    }

    /// Await and consume the result from this actual thread. Dropping the
    /// wait future leaves custody with the job until its owner retires it.
    ///
    /// # Errors
    /// Return the resolver disposition, or report failed or absent custody.
    pub(super) async fn wait(&mut self) -> Answer {
        loop {
            if let Poll::Ready(outcome) = self.job.try_join() {
                return match outcome {
                    Ok(answer) => answer,
                    Err(_) => Err(resolver_task()),
                };
            }
            tokio::time::sleep(COMPLETION_CHECK).await;
        }
    }

    /// Retire this invocation and report whether actual thread custody remains.
    pub(super) fn retire(self) -> bool {
        self.job.retire()
    }
}

/// Report a custody failure without exposing the worker's private panic payload.
fn resolver_task() -> AddressRoundRefreshOutcome {
    AddressRoundRefreshOutcome::Failed(PilotError::ResolverTask)
}

#[cfg(test)]
mod tests {
    use std::{
        error::Error,
        io,
        net::{SocketAddr, TcpStream},
        sync::mpsc::{self, Receiver, Sender},
    };

    use super::super::{AddressRoundConfig, series_packets};
    use super::{
        AddressRoundRefreshOutcome, COMPLETION_CHECK, Duration, NativeJob, NativeJobError,
        NativeResolverJob, PilotError, RefreshPlan, resolver_task,
    };

    /// Finite watchdog for both coordinator-side and worker-side waits.
    const WATCHDOG: Duration = Duration::from_secs(5);

    /// Prepare a checked result whose peer identifies the actual producing thread.
    /// # Errors
    /// Return a configuration or fixture-packet refusal.
    fn answer(peer: SocketAddr) -> Result<RefreshPlan, Box<dyn Error>> {
        let config: AddressRoundConfig =
            AddressRoundConfig::new(&[peer], Duration::from_millis(1), Duration::from_secs(1))?;
        let entropy: [u8; 24] = [
            1, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 3,
        ];
        let packets = series_packets(&entropy)?;
        Ok(RefreshPlan { config, packets })
    }

    /// Reap a retired thread through normal admission and hold the next admitted dispatch.
    /// The worker reports a live dispatch, then its retirement state on the first
    /// advance, and returns on the second advance.
    /// # Errors
    /// Return a fixture preparation or worker start failure.
    async fn start_successor(
        peer: SocketAddr,
    ) -> Result<(NativeResolverJob, Sender<()>, Receiver<bool>), Box<dyn Error>> {
        loop {
            let prepared: RefreshPlan = answer(peer)?;
            let (report, observed): (Sender<bool>, Receiver<bool>) = mpsc::channel();
            let (advance, advanced): (Sender<()>, Receiver<()>) = mpsc::channel();
            match NativeResolverJob::start(move |admission| {
                let dispatched_once: bool = admission.admit() && !admission.admit();
                let live_dispatch: bool = dispatched_once && !admission.retirement_requested();
                if report.send(live_dispatch).is_err()
                    || !live_dispatch
                    || advanced.recv_timeout(WATCHDOG).is_err()
                    || report.send(admission.retirement_requested()).is_err()
                    || advanced.recv_timeout(WATCHDOG).is_err()
                {
                    return Err(resolver_task());
                }
                Ok(prepared)
            }) {
                Ok(job) => return Ok((job, advance, observed)),
                Err(PilotError::ResolutionBusy) => tokio::time::sleep(COMPLETION_CHECK).await,
                Err(error) => return Err(error.into()),
            }
        }
    }

    /// Exercise the one process slot serially with actual blocking threads.
    ///
    /// # Errors
    /// Return failed fixture preparation or a bounded thread/watchdog failure.
    /// # Panics
    /// Detect invalid dispatch, missed retirement, premature slot release, or
    /// lost or reused results.
    #[test]
    fn retirement_closes_dispatch_and_preserves_actual_thread_custody() -> Result<(), Box<dyn Error>>
    {
        #[cfg(feature = "roughtime-verification")]
        let _fixture: std::sync::MutexGuard<'_, ()> = crate::native_network_job::CUSTODY_FIXTURE
            .lock()
            .map_err(|_| "native worker fixture unavailable")?;
        let old_peer: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 41001));
        let current_peer: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 41002));
        let old_answer: RefreshPlan = answer(old_peer)?;
        let (entered, started): (Sender<()>, Receiver<()>) = mpsc::channel();
        let (release, released): (Sender<()>, Receiver<()>) = mpsc::channel();
        let (dispatched, observed): (Sender<bool>, Receiver<bool>) = mpsc::channel();
        let original: NativeResolverJob = NativeResolverJob::start(move |admission| {
            if entered.send(()).is_err() || released.recv_timeout(WATCHDOG).is_err() {
                return Err(resolver_task());
            }
            let _reported: Result<(), mpsc::SendError<bool>> = dispatched.send(admission.admit());
            // Offer a late answer anyway to ensure successor custody discards it.
            Ok(old_answer)
        })?;
        started.recv_timeout(WATCHDOG)?;
        assert!(original.retire());
        assert!(matches!(
            NativeJob::<io::Result<TcpStream>>::start("nevrotick-tcp", |_| {
                Err(io::Error::other("unexpected TCP worker"))
            }),
            Err(NativeJobError::Busy)
        ));
        assert!(matches!(
            NativeResolverJob::start(|_| Err(resolver_task())),
            Err(PilotError::ResolutionBusy)
        ));
        release.send(())?;
        assert!(!observed.recv_timeout(WATCHDOG)?);

        let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()?;
        for explicit_retirement in [true, false] {
            let (retiring, advance, observed): (NativeResolverJob, Sender<()>, Receiver<bool>) =
                runtime.block_on(async {
                    tokio::time::timeout(WATCHDOG, start_successor(old_peer)).await?
                })?;
            assert!(observed.recv_timeout(WATCHDOG)?);
            if explicit_retirement {
                assert!(retiring.retire());
            } else {
                drop(retiring);
            }
            advance.send(())?;
            assert!(observed.recv_timeout(WATCHDOG)?);
            assert!(matches!(
                NativeResolverJob::start(|_| Err(resolver_task())),
                Err(PilotError::ResolutionBusy)
            ));
            advance.send(())?;
        }
        let (mut current, advance_current, observed_current): (
            NativeResolverJob,
            Sender<()>,
            Receiver<bool>,
        ) = runtime.block_on(async {
            tokio::time::timeout(WATCHDOG, start_successor(current_peer)).await?
        })?;
        assert!(observed_current.recv_timeout(WATCHDOG)?);
        assert!(
            runtime
                .block_on(async { tokio::time::timeout(Duration::ZERO, current.wait()).await })
                .is_err()
        );
        assert!(matches!(
            NativeResolverJob::start(|_| Err(resolver_task())),
            Err(PilotError::ResolutionBusy)
        ));
        advance_current.send(())?;
        assert!(!observed_current.recv_timeout(WATCHDOG)?);
        advance_current.send(())?;
        let joined: Result<RefreshPlan, AddressRoundRefreshOutcome> =
            runtime.block_on(async { tokio::time::timeout(WATCHDOG, current.wait()).await })?;
        let Ok(joined) = joined else {
            return Err(PilotError::ResolverTask.into());
        };
        assert_eq!(joined.config.peers, [current_peer]);
        assert!(matches!(
            runtime.block_on(async { tokio::time::timeout(WATCHDOG, current.wait()).await })?,
            Err(AddressRoundRefreshOutcome::Failed(PilotError::ResolverTask))
        ));
        assert!(!current.retire());
        Ok(())
    }
}
