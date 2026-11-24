# SPDX-License-Identifier: GPL-2.0-only
"""Read retained W11-W01 capture rows and compare held-out timing exchanges.

The bench owner supplies an independently paired reference, its declared QPC
validity and the complete possible exchange span. Calibration, physical pairing,
residual independence and aggregate budgets belong to that owner. This
comparison grants no mapping authority.
"""

from dataclasses import dataclass
from enum import Enum
from hashlib import sha256
import json
import re


_MAX_I64: int = (1 << 63) - 1
_MIN_I64: int = -(1 << 63)
_MAX_U64: int = (1 << 64) - 1
_SHA256: re.Pattern[str] = re.compile(r"[0-9a-f]{64}")
_RECEIPT_LIMIT: int = 64 * 1024
_STREAM_LIMIT: int = 4096
_CAPTURE_STREAM: str = "healthy-ke-managed-successor-facade-client-stdout.txt"
_CAPTURE_POINTS: tuple[str, str, str] = ("pre-send", "send-return", "receive-return")
_CAPTURE_FIELDS: re.Pattern[bytes] = re.compile(
    rb" client_port=([1-9][0-9]{0,4}) peer_port=([1-9][0-9]{0,4})"
    rb" qpc_before=(0|-?[1-9][0-9]{0,18}) filetime_100ns=(0|[1-9][0-9]{0,19})"
    rb" qpc_after=(0|-?[1-9][0-9]{0,18}) frequency_hz=([1-9][0-9]{0,19})"
)


class TrialRefused(ValueError):
    """A malformed, crossed or out-of-budget observation cannot support the trial."""


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
            _MIN_I64 <= self.earliest <= self.latest <= _MAX_I64
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


@dataclass(frozen=True, slots=True)
class HeldOutComparison:
    """One modeled local-wall reading and its independent reference limits."""

    local_wall: Nanoseconds
    reference: HeldOutLeg
    declared_error_ns: int
    leg_budget_ns: int


@dataclass(frozen=True, slots=True)
class QpcValidityWindow:
    """Bench validity on the chart identified by the original timing receipt."""

    receipt_sha256: str
    start_tick: int
    end_tick_exclusive: int

    def __post_init__(self) -> None:
        if type(self.receipt_sha256) is not str or _SHA256.fullmatch(self.receipt_sha256) is None:
            raise TrialRefused("invalid validity chart identity")
        if type(self.start_tick) is not int or type(self.end_tick_exclusive) is not int or not (
            _MIN_I64 <= self.start_tick < self.end_tick_exclusive <= _MAX_I64
        ):
            raise TrialRefused("invalid QPC validity window")


@dataclass(frozen=True, slots=True)
class PairedExchangeSpan:
    """Closed full possible exchange span supplied by the bench pairing owner."""

    receipt_sha256: str
    exchange_sha256: str
    earliest_tick: int
    latest_tick: int

    def __post_init__(self) -> None:
        if any(type(pin) is not str or _SHA256.fullmatch(pin) is None for pin in (
            self.receipt_sha256, self.exchange_sha256,
        )):
            raise TrialRefused("invalid paired span identity")
        if type(self.earliest_tick) is not int or type(self.latest_tick) is not int or not (
            _MIN_I64 <= self.earliest_tick <= self.latest_tick <= _MAX_I64
        ):
            raise TrialRefused("invalid paired QPC span")


@dataclass(frozen=True, slots=True)
class ApplicationCapture:
    """One original QPC/FILETIME/QPC bracket and its terminated row digest."""

    qpc_before: int
    filetime_100ns: int
    qpc_after: int
    row_sha256: str


@dataclass(frozen=True, slots=True)
class RetainedExchange:
    """Original application rows on the QPC chart of one pinned lab run.

    FILETIME is the raw wall reading in 100 ns units since 1601. The three
    brackets establish application chronology within this run. The caller
    supplies the candidate modeled local-wall interval for comparison.
    """

    receipt_sha256: str
    exchange_sha256: str
    client_port: int
    peer_port: int
    frequency_hz: int
    pre_send: ApplicationCapture
    send_return: ApplicationCapture
    receive_return: ApplicationCapture

    def modeled_leg(self, event: ApplicationEvent, local_wall: Nanoseconds) -> ModeledLeg:
        """Attach the original row identity to the modeled T1 or T4 interval."""
        if event is ApplicationEvent.PRE_SEND:
            capture: ApplicationCapture = self.pre_send
        elif event is ApplicationEvent.RECEIVE_RETURN:
            capture = self.receive_return
        else:
            raise TrialRefused("invalid application event")
        if type(local_wall) is not Nanoseconds:
            raise TrialRefused("invalid comparison coordinates")
        return ModeledLeg(TrialEvent(self.exchange_sha256, event), capture.row_sha256, local_wall)


def _receipt_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    """Decode one receipt object without collapsing duplicated keys."""
    result: dict[str, object] = {}
    key: str
    value: object
    for key, value in pairs:
        if key in result:
            raise TrialRefused("duplicate timing receipt field")
        result[key] = value
    return result


def _receipt_frequency(receipt: bytes, pin: str, stream: bytes) -> int:
    """Select the exact retained stream from a separately pinned terminal receipt."""
    if type(receipt) is not bytes or len(receipt) > _RECEIPT_LIMIT:
        raise TrialRefused("timing receipt exceeds input bounds")
    if type(stream) is not bytes or len(stream) > _STREAM_LIMIT:
        raise TrialRefused("timing stream exceeds input bounds")
    if type(pin) is not str or _SHA256.fullmatch(pin) is None or sha256(receipt).hexdigest() != pin:
        raise TrialRefused("timing receipt digest differs from pin")
    try:
        metadata: object = json.loads(receipt.decode("utf-8"), object_pairs_hook=_receipt_object)
    except TrialRefused:
        raise
    except (ValueError, RecursionError) as error:
        raise TrialRefused("invalid timing receipt") from error
    if type(metadata) is not dict or (
        metadata.get("scope") != "native-authenticated-ordinary-timing"
        or metadata.get("scenario") != "ordinary-timing"
        or metadata.get("status") != "passed-native-managed-successor-facade"
        or metadata.get("retirement") != "exact-one-peer-timing-restoration"
        or metadata.get("empty_mappings") is not True
    ):
        raise TrialRefused("receipt is not a completed timing run")
    outputs: object = metadata.get("retained_output")
    stdout: object = outputs.get(_CAPTURE_STREAM) if type(outputs) is dict else None
    if type(stdout) is not dict or type(stdout.get("bytes")) is not int or (
        stdout["bytes"] != len(stream) or stdout.get("sha256") != sha256(stream).hexdigest()
    ):
        raise TrialRefused("timing stream differs from receipt")
    observation: object = metadata.get("observed")
    relative: object = observation.get("relative_timing") if type(observation) is dict else None
    frequency: object = relative.get("frequency_hz") if type(relative) is dict else None
    # QueryPerformanceFrequency supplies a positive signed LARGE_INTEGER.
    if type(frequency) is not int or not 0 < frequency <= _MAX_I64:
        raise TrialRefused("invalid timing receipt frequency")
    return frequency


def _capture_row(line: bytes, leg: str, point: str, frequency: int) -> tuple[int, int, ApplicationCapture]:
    """Decode one selected producer row while retaining its original byte identity."""
    prefix: bytes = f"scope=managed-protected-successor-timing leg={leg} point={point}".encode("ascii")
    fields: re.Match[bytes] | None = _CAPTURE_FIELDS.fullmatch(line.removesuffix(b"\r"), len(prefix)) if (
        line.startswith(prefix)
    ) else None
    if fields is None:
        raise TrialRefused("invalid capture row")
    client: int
    peer: int
    before: int
    filetime: int
    after: int
    hz: int
    client, peer, before, filetime, after, hz = map(int, fields.groups())
    if client > 65_535 or peer > 65_535 or filetime > _MAX_U64 or not _MIN_I64 <= before <= after <= _MAX_I64:
        raise TrialRefused("invalid capture row bounds")
    if hz != frequency:
        raise TrialRefused("capture frequency differs from receipt")
    return client, peer, ApplicationCapture(before, filetime, after, sha256(line + b"\n").hexdigest())


def read_retained_captures(receipt: bytes, receipt_sha256: str, stream: bytes) -> tuple[RetainedExchange, RetainedExchange]:
    """Bind the two ordered application triplets to their independently pinned run.

    The receipt must already be admitted by the original lab owner. This
    reader uses the retained relative-timing schema and the original complete
    Engine stream. It makes no independent wire pairing or UTC assertion.
    """
    frequency: int = _receipt_frequency(receipt, receipt_sha256, stream)
    rows: list[bytes] = stream.split(b"\n", 6)
    if len(rows) != 7 or rows[-1] != b"":
        raise TrialRefused("timing stream requires exactly six terminated rows")
    exchanges: list[RetainedExchange] = []
    offset: int
    leg: str
    for offset, leg in ((0, "first"), (3, "successor")):
        captures: list[tuple[int, int, ApplicationCapture]] = [
            _capture_row(rows[offset + index], leg, point, frequency) for index, point in enumerate(_CAPTURE_POINTS)
        ]
        client: int
        peer: int
        pre_send: ApplicationCapture
        client, peer, pre_send = captures[0]
        send_return: ApplicationCapture = captures[1][2]
        receive_return: ApplicationCapture = captures[2][2]
        if any((row_client, row_peer) != (client, peer) for row_client, row_peer, _ in captures):
            raise TrialRefused("exchange sockets differ across timing rows")
        if pre_send.qpc_after > send_return.qpc_before or send_return.qpc_after > receive_return.qpc_before:
            raise TrialRefused("invalid application bracket order")
        exchange_sha256: str = sha256(f"{receipt_sha256}:{leg}".encode("ascii")).hexdigest()
        exchanges.append(RetainedExchange(
            receipt_sha256, exchange_sha256, client, peer, frequency, pre_send, send_return, receive_return,
        ))
    first: RetainedExchange
    successor: RetainedExchange
    first, successor = exchanges
    if first.client_port == successor.client_port or first.peer_port != successor.peer_port:
        raise TrialRefused("exchange sockets differ from the ordinary successor shape")
    if first.receive_return.qpc_after > successor.pre_send.qpc_before:
        raise TrialRefused("invalid successor bracket order")
    return first, successor


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
        0 <= declared_error_ns <= _MAX_I64 and 0 <= leg_budget_ns <= _MAX_I64
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


def compare_held_out_exchange(
    receipt: bytes, receipt_sha256: str, stream: bytes, validity: QpcValidityWindow, span: PairedExchangeSpan,
    *, pre_send: HeldOutComparison, receive_return: HeldOutComparison,
) -> tuple[int, int]:
    """Compare T1 and T4 only when the whole exchange fits the declared QPC window.

    Both results are worst-case disagreements in nanoseconds, in T1/T4 order.
    The original application brackets and external paired span are checked
    separately. The bench owner remains responsible for physical validity.
    """
    if type(validity) is not QpcValidityWindow or type(span) is not PairedExchangeSpan:
        raise TrialRefused("invalid exchange validity inputs")
    if validity.receipt_sha256 != receipt_sha256 or span.receipt_sha256 != receipt_sha256:
        raise TrialRefused("exchange validity chart differs from receipt")
    exchanges: tuple[RetainedExchange, RetainedExchange] = read_retained_captures(receipt, receipt_sha256, stream)
    exchange: RetainedExchange
    for exchange in exchanges:
        if exchange.exchange_sha256 == span.exchange_sha256:
            break
    else:
        raise TrialRefused("paired span exchange differs from receipt")

    captures: tuple[ApplicationCapture, ApplicationCapture, ApplicationCapture] = (
        exchange.pre_send, exchange.send_return, exchange.receive_return,
    )
    if any(not validity.start_tick <= capture.qpc_before <= capture.qpc_after < validity.end_tick_exclusive
           for capture in captures):
        raise TrialRefused("application bracket crosses validity window")
    if not validity.start_tick <= span.earliest_tick <= span.latest_tick < validity.end_tick_exclusive:
        raise TrialRefused("paired span crosses validity window")

    app_only_pins: tuple[str, ...] = (receipt_sha256, sha256(stream).hexdigest()) + tuple(
        capture.row_sha256 for retained in exchanges
        for capture in (retained.pre_send, retained.send_return, retained.receive_return)
    )
    results: list[int] = []
    event: ApplicationEvent
    comparison: HeldOutComparison
    for event, comparison in ((ApplicationEvent.PRE_SEND, pre_send), (ApplicationEvent.RECEIVE_RETURN, receive_return)):
        if type(comparison) is not HeldOutComparison or type(comparison.reference) is not HeldOutLeg:
            raise TrialRefused("invalid held-out comparison")
        reference: HeldOutLeg = comparison.reference
        if reference.wire_record_sha256 in app_only_pins or reference.residual_record_sha256 in app_only_pins:
            raise TrialRefused("application capture reused as independent reference")
        results.append(compare_held_out_leg(
            exchange.modeled_leg(event, comparison.local_wall), reference,
            declared_error_ns=comparison.declared_error_ns, leg_budget_ns=comparison.leg_budget_ns,
        ))
    return results[0], results[1]
