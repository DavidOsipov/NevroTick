# SPDX-License-Identifier: GPL-2.0-only
"""Run the fixed S0.6 observation sequence in one admitted deployment window.

The native window owner supplies separately verified signed package inputs,
effective actor contexts and retained child observations. This module orders
their use and withholds the joined result on an incomplete window. It does
not load a window or establish native admission from untrusted files or data.
"""

from collections.abc import Callable
from typing import Protocol, TypeVar

from prestart_receipts import (
    Capture, DeploymentObservation, DeploymentRun, PrestartObservation, PrestartProbe, Run, ScmTypeSample,
    read_deployment, read_prestart, require, require_run,
)


OBSERVER_ARGUMENTS = (
    "--exact", "--ignored", "--nocapture", "--test-threads=1",
    "--format", "pretty", "--color", "never",
)


RunT = TypeVar("RunT", bound=DeploymentRun)
ObservationT = TypeVar("ObservationT", bound=DeploymentObservation)


class DeploymentWindow(Protocol[RunT]):
    """Native owner of one admitted signed-artifact observation attempt.

    The owner keeps its original artifact, actor and quiescent window
    identities live. It retains each child and actual input through bounded
    collection and retirement, preserving the initiating and cleanup failures
    in its own record. None of the captured bytes supplies those facts.
    """

    def take_run(self) -> RunT:
        """Consume this window's one observation attempt or refuse."""
        ...

    def current_run(self) -> RunT | None:
        """Return the same still-current admission or report permanent withdrawal."""
        ...

    def initial_scm_types(self, run: RunT) -> ScmTypeSample:
        """Return this run's immutable baseline from the original retained SCM handles."""
        ...

    def capture_identity(self, run: RunT) -> Capture:
        """Collect the fixed identity companion in the deployment context."""
        ...

    def capture_deployment_observation(self, run: RunT) -> Capture:
        """Collect the fixed observation through the original SCM window and verified archive."""
        ...

    def retire_run(self, run: RunT) -> BaseException | None:
        """Settle the consumed run. Return None only for confirmed retirement.

        Return or raise the original failure when retirement is refused.

        A refusal retains unresolved resources with the native owner for
        recovery. If take_run fails, it owns rollback of partial admission.
        """
        ...


class PrestartWindow(DeploymentWindow[Run], Protocol):
    """Deployment window with separately admitted access observers."""

    def capture_observer(self, run: Run, actor: object, arguments: tuple[str, ...]) -> Capture:
        """Collect the fixed access probe under its separately admitted actor."""
        ...


def supervise_deployment(window: DeploymentWindow[DeploymentRun]) -> DeploymentObservation:
    """Check installed identity and point SCM observations in the original window.

    Each capture must pass its receipt checks before the next step. Only a
    current run with confirmed retirement can return its checked observation.
    The result establishes neither access-probe outcomes nor authority to change
    the deployment.
    """
    def observe(run: DeploymentRun) -> DeploymentObservation:
        def collect(probe: PrestartProbe) -> Capture:
            _require_current(window, run)
            if probe is PrestartProbe.IDENTITY:
                return window.capture_identity(run)
            require(probe is PrestartProbe.OBSERVATION, "invalid deployment observation probe")
            return window.capture_deployment_observation(run)

        scm_types: ScmTypeSample = window.initial_scm_types(run)
        observation: DeploymentObservation = read_deployment(run, collect, scm_types)
        _require_current(window, run)
        return observation

    return _observe_and_retire(window, observe)


def supervise_prestart(window: PrestartWindow) -> PrestartObservation:
    """Observe and join the four required captures in one live, owned run.

    Each child must settle and pass its receipt checks before the next starts.
    The next window check follows those checks, preserving an earlier child
    refusal. Once the run is handed off, the observation block attempts its
    retirement on success or failure. The native owner remains responsible for
    interruption during admission or cleanup. Return the observation only after
    the final currentness check and confirmed retirement. If both attempts fail,
    retain the first cause followed by the retirement failure.
    A completed observation conveys no
    authority for an installation, configuration change or service start.
    """
    def observe(run: Run) -> PrestartObservation:
        actors: dict[PrestartProbe, object] = {
            PrestartProbe.ACCESS: run.provisioning_actor,
            PrestartProbe.STANDARD: run.standard_actor,
        }

        def collect(probe: PrestartProbe) -> Capture:
            _require_current(window, run)
            if probe is PrestartProbe.IDENTITY:
                return window.capture_identity(run)
            if probe is PrestartProbe.OBSERVATION:
                return window.capture_deployment_observation(run)
            return window.capture_observer(run, actors[probe], (probe.value, *OBSERVER_ARGUMENTS))

        require_run(run)
        scm_types: ScmTypeSample = window.initial_scm_types(run)
        observation: PrestartObservation = read_prestart(run, collect, scm_types)
        _require_current(window, run)
        return observation

    return _observe_and_retire(window, observe)


def _require_current(window: DeploymentWindow[RunT], run: RunT) -> None:
    """A value-equal run from a different window supplies no current authority."""
    require(window.current_run() is run, "pre-start deployment window changed")


def _observe_and_retire(
    window: DeploymentWindow[RunT], observe: Callable[[RunT], ObservationT],
) -> ObservationT:
    """Settle the original consumed run and preserve independent failures in order."""
    primary: BaseException | None = None
    run: RunT = window.take_run()
    try:
        return observe(run)
    except BaseException as error:
        primary = error
        raise
    finally:
        try:
            result: BaseException | None = window.retire_run(run)
            if result is not None:
                require(isinstance(result, BaseException), "pre-start retirement returned an invalid outcome")
                raise result
        except BaseException as retirement:
            if primary is None:
                raise
            if retirement is not primary:
                raise BaseExceptionGroup(
                    "pre-start observation and retirement failed", [primary, retirement]
                ) from None
