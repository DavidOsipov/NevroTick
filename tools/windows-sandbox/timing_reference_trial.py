# SPDX-License-Identifier: GPL-2.0-only
"""Compare one W11-W01 held-out leg using exact, outward-bounded nanoseconds.

The caller supplies the original capture and an independently paired wire event
in the same UTC epoch. Calibration, association to the captured QPC bracket,
residual independence, validity over the full exchange and aggregate budgets
belong to the bench owner. This comparison grants no mapping authority.
"""

from dataclasses import dataclass
from enum import Enum
import re


_MAX_NS: int = (1 << 63) - 1
_MIN_NS: int = -(1 << 63)
_SHA256: re.Pattern[str] = re.compile(r"[0-9a-f]{64}")


class TrialRefused(ValueError):
    """A malformed, crossed or out-of-budget leg cannot support the trial."""


class ApplicationEvent(Enum):
    """The two application sampling instants that supply local exchange time."""

    PRE_SEND = "pre-send"
    RECEIVE_RETURN = "receive-return"


@dataclass(frozen=True, slots=True)
class Nanoseconds:
    """Closed interval in signed integer nanoseconds, with outward rounding."""

    earliest: int
    latest: int

    def __post_init__(self) -> None:
        if type(self.earliest) is not int or type(self.latest) is not int or not (
            _MIN_NS <= self.earliest <= self.latest <= _MAX_NS
        ):
            raise TrialRefused("invalid nanosecond interval")


@dataclass(frozen=True, slots=True)
class TrialEvent:
    """The original independently paired exchange and one local event."""

    exchange_sha256: str
    event: ApplicationEvent

    def __post_init__(self) -> None:
        if type(self.exchange_sha256) is not str or _SHA256.fullmatch(self.exchange_sha256) is None:
            raise TrialRefused("invalid exchange identity")
        if type(self.event) is not ApplicationEvent:
            raise TrialRefused("invalid application event")


@dataclass(frozen=True, slots=True)
class ModeledLeg:
    """The unenlarged candidate local-wall interval over the original bracket."""

    event: TrialEvent
    application_capture_sha256: str
    local_wall: Nanoseconds


@dataclass(frozen=True, slots=True)
class HeldOutLeg:
    """Traceable packet UTC and independent estimates at the application event.

    Placement is application UTC minus wire UTC. The signed wall residual is
    local wall minus UTC at the application event. Both bounds include their
    justified pairing, drift and representation uncertainties.
    """

    event: TrialEvent
    wire_record_sha256: str
    residual_record_sha256: str
    wire_utc: Nanoseconds
    application_minus_wire: Nanoseconds
    local_wall_minus_utc: Nanoseconds


def compare_held_out_leg(
    modeled: ModeledLeg, reference: HeldOutLeg, *, declared_error_ns: int, leg_budget_ns: int,
) -> int:
    """Return the greatest possible disagreement if it fits both error limits.

    Digests identify original input records supplied by the bench owner. A
    matching digest disqualifies reuse of the candidate as its own reference.
    Physical independence also requires separate acquisition and calibration.
    """
    if type(modeled.event) is not TrialEvent or type(reference.event) is not TrialEvent or (
        modeled.event != reference.event
    ):
        raise TrialRefused("reference event differs")
    records: tuple[str, str, str] = (
        modeled.application_capture_sha256, reference.wire_record_sha256, reference.residual_record_sha256,
    )
    if any(type(record) is not str or _SHA256.fullmatch(record) is None for record in records):
        raise TrialRefused("invalid observation identity")
    if records[0] in records[1:]:
        raise TrialRefused("application capture reused as independent reference")
    if type(declared_error_ns) is not int or type(leg_budget_ns) is not int or not (
        0 <= declared_error_ns <= _MAX_NS and 0 <= leg_budget_ns <= _MAX_NS
    ):
        raise TrialRefused("invalid error limit")
    if declared_error_ns > leg_budget_ns:
        raise TrialRefused("declared error exceeds leg budget")

    components: tuple[Nanoseconds, Nanoseconds, Nanoseconds] = (
        reference.wire_utc, reference.application_minus_wire, reference.local_wall_minus_utc,
    )
    if type(modeled.local_wall) is not Nanoseconds or any(type(term) is not Nanoseconds for term in components):
        raise TrialRefused("invalid comparison coordinates")
    expected_local: Nanoseconds = Nanoseconds(
        sum(term.earliest for term in components), sum(term.latest for term in components),
    )
    required_ns: int = max(
        abs(modeled.local_wall.earliest - expected_local.latest),
        abs(modeled.local_wall.latest - expected_local.earliest),
    )
    if required_ns > declared_error_ns:
        raise TrialRefused("worst-case disagreement exceeds declared error")
    return required_ns
