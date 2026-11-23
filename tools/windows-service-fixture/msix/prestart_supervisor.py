# SPDX-License-Identifier: GPL-2.0-only
"""Run the fixed S0.6 observation sequence in one admitted deployment window.

The native window owner supplies separately verified signed package inputs,
effective actor contexts and retained child observations. This module orders
their use and withholds the joined result on an incomplete window. It does
not load a window or establish native admission from untrusted files or data.
"""

from typing import Protocol

from prestart_receipts import Capture, PrestartObservation, PrestartProbe, Run, read_prestart, require


OBSERVER_ARGUMENTS = (
    "--exact", "--ignored", "--nocapture", "--test-threads=1",
    "--format", "pretty", "--color", "never",
)


class PrestartWindow(Protocol):
    """Native owner of one admitted signed-artifact observation attempt.

    The owner keeps its original artifact, actor and quiescent window
    identities live. It retains each child and actual input through bounded
    collection and retirement, preserving the initiating and cleanup failures
    in its own record. None of the captured bytes supplies those facts.
    """

    def take_run(self) -> Run:
        """Consume this window's one observation attempt or refuse."""
        ...

    def current_run(self) -> Run | None:
        """Return the same still-current admission or report permanent withdrawal."""
        ...

    def capture_identity(self, run: Run) -> Capture:
        """Collect the fixed identity companion in the deployment context."""
        ...

    def capture_observer(self, run: Run, actor: object, arguments: tuple[str, ...]) -> Capture:
        """Collect the one exact observer selector under its admitted actor."""
        ...


def supervise_prestart(window: PrestartWindow) -> PrestartObservation:
    """Observe and join the four required captures in one live, owned run.

    Each child must settle and pass its receipt checks before the next starts.
    The next window check follows those checks, preserving an earlier child
    refusal. A native refusal propagates unchanged so its owner retains the actual
    failure and any unresolved cleanup. A completed observation conveys no
    authority for an installation, configuration change or service start.
    """
    run: Run = window.take_run()
    actors: dict[PrestartProbe, object] = {
        PrestartProbe.OBSERVATION: run.deployment_actor,
        PrestartProbe.ACCESS: run.provisioning_actor,
        PrestartProbe.STANDARD: run.standard_actor,
    }

    def require_current() -> None:
        require(window.current_run() is run, "pre-start deployment window changed")

    def collect(probe: PrestartProbe) -> Capture:
        require_current()
        if probe is PrestartProbe.IDENTITY:
            return window.capture_identity(run)
        return window.capture_observer(run, actors[probe], (probe.value, *OBSERVER_ARGUMENTS))

    observation: PrestartObservation = read_prestart(run, collect)
    require_current()
    return observation
