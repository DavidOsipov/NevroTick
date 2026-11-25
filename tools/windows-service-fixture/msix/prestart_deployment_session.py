# SPDX-License-Identifier: GPL-2.0-only
"""Drive one admitted S0.6 Windows deployment window over private process pipes."""

from _thread import LockType
import base64
import binascii
from collections.abc import Callable
from dataclasses import dataclass, field
from hashlib import sha256
import json
import math
import os
from pathlib import Path
import re
import select
import stat
from subprocess import PIPE, Popen, TimeoutExpired
from threading import Lock
import time
from typing import BinaryIO, Generic, Protocol, TypeVar, runtime_checkable
from uuid import UUID

from check_package import Refused, read_regular, unique_object
from prestart_receipts import (
    MAX_OUTPUT, SHA256, Capture, DeploymentObservation, DeploymentRun, Member, PrestartObservation,
    PrestartProbe, ReceiptRefused, Run, ScmTypeSample, is_dword, require, require_deployment_run, require_run,
)
from prestart_supervisor import OBSERVER_ARGUMENTS, RunT, supervise_deployment, supervise_prestart


MAX_FRAME = 24000
CODE = re.compile(r"[a-z0-9-]{1,80}")
_CAPTURE_FIELDS: tuple[str, ...] = ("pid", "before", "after", "exit", "stdout", "stderr")
_ACCESS_DIAGNOSTIC_PREFIX: bytes = b"nevrotick-prestart-access-capture "
_ACCESS_DIAGNOSTIC_UNAVAILABLE: bytes = b"nevrotick-prestart-access-capture-unavailable"
_ACCESS_DIAGNOSTIC_FILE_LIMIT: int = 32 * 1024
_ACCESS_OWNER_FAILURE: re.Pattern[bytes] = re.compile(
    rb"nevrotick-deployment-owner failure=[a-z/-]+ cause=[a-z0-9:-]+ cleanup=[a-z-]+(?:,[a-z-]+)*"
    rb"(?: transport=reply-transport)?\r?"
)


class OwnedDeploymentProcess(Protocol):
    """Private pipe ends from the original process and its external job owner.

    The caller has already admitted the process, pinned inputs, original run
    and native job. This adapter assumes sole use of these unbuffered POSIX
    pipe ends, including when using WSL interoperability. The external owner
    keeps custody of process/job/diagnostic streams even when the pipe fails.
    """

    stdin: BinaryIO
    stdout: BinaryIO

    def wait_retired(self, deadline: float) -> None:
        """Require original owner exit and job/stream settlement by the absolute deadline.

        Raise if cleanup is uncertain. This observation is independent of the
        terminal frame. The original owner retains unresolved work for recovery.
        """
        ...


@runtime_checkable
class AccessWriterSettlement(Protocol):
    """Original access owner with separate observation of refusal cleanup."""

    def wait_access_settled(self, deadline: float) -> None:
        """Require actual access process and writer retirement after success or refusal."""
        ...


class AccessStatusOverflow(ReceiptRefused):
    """Refuse a bounded status read while the original relay still needs retirement."""


@dataclass(frozen=True, slots=True)
class CapturedProbe:
    """Original bounded capture associated with its fixed probe and pipe owner."""

    probe: PrestartProbe
    owner: OwnedDeploymentProcess = field(repr=False)
    capture: Capture = field(repr=False)


@dataclass(frozen=True, slots=True)
class PrivateAccessDiagnostic:
    """Historical child bytes from the original access owner's private stderr."""

    run_id: UUID
    probe: PrestartProbe
    pid: int
    exit_code: int
    confirmed_input_sha256: str | None
    stdout: bytes = field(repr=False)
    stderr: bytes = field(repr=False)


def read_private_access_diagnostic(
    run: DeploymentRun, probe: PrestartProbe, diagnostics: BinaryIO,
) -> PrivateAccessDiagnostic | None:
    """Read the original retained stderr file after the role's writers retire.

    The launcher supplies its original private regular file and independently
    confirms writer settlement. Reading preserves the caller's offset and rejects
    incomplete, oversized or crossed records. This result is diagnostic only.
    """
    require_deployment_run(run)
    require(probe is PrestartProbe.ACCESS or probe is PrestartProbe.STANDARD,
            "invalid private access diagnostic probe")
    try:
        fd: int = diagnostics.fileno()
        before: os.stat_result = os.fstat(fd)
    except (OSError, ValueError):
        raise ReceiptRefused("private access diagnostic descriptor refused") from None
    require(stat.S_ISREG(before.st_mode) and 0 <= before.st_size <= _ACCESS_DIAGNOSTIC_FILE_LIMIT,
            "invalid private access diagnostic file")
    try:
        raw: bytes = os.pread(fd, before.st_size + 1, 0)
        after: os.stat_result = os.fstat(fd)
    except OSError:
        raise ReceiptRefused("private access diagnostic read refused") from None
    require(len(raw) == before.st_size == after.st_size
            and before.st_mtime_ns == after.st_mtime_ns and before.st_ctime_ns == after.st_ctime_ns,
            "private access diagnostic file changed during read")
    return _decode_private_access_diagnostic(run, probe, raw)


def _decode_private_access_diagnostic(
    run: DeploymentRun, probe: PrestartProbe, raw: bytes,
) -> PrivateAccessDiagnostic | None:
    """Decode the exact byte snapshot from the independently settled writer."""
    require(probe is PrestartProbe.ACCESS or probe is PrestartProbe.STANDARD,
            "invalid private access diagnostic probe")
    require(len(raw) <= _ACCESS_DIAGNOSTIC_FILE_LIMIT, "invalid private access diagnostic file")
    require(not raw or raw.endswith(b"\n"), "incomplete private access diagnostic file")

    diagnostic: bytes | None = None
    unavailable: bool = False
    previous: bytes = b""
    for line in raw.split(b"\n"):
        is_capture: bool = line.startswith(_ACCESS_DIAGNOSTIC_PREFIX)
        if is_capture or line.rstrip(b"\r") == _ACCESS_DIAGNOSTIC_UNAVAILABLE:
            require(diagnostic is None and not unavailable, "repeated private access diagnostic")
            require(_ACCESS_OWNER_FAILURE.fullmatch(previous) is not None,
                    "private access diagnostic missing owner failure")
            unavailable = not is_capture
            if is_capture:
                require(len(line) < MAX_FRAME, "oversized private access diagnostic record")
                diagnostic = line[len(_ACCESS_DIAGNOSTIC_PREFIX):]
        previous = line
    if diagnostic is None:
        return None

    row: object = None
    try:
        row = json.loads(diagnostic.decode("ascii"), object_pairs_hook=unique_object)
    except (ValueError, RecursionError):
        pass
    require(type(row) is dict and set(row) == {"run", "role", "expected", *_CAPTURE_FIELDS},
            "invalid private access diagnostic record")
    role: str = "access" if probe is PrestartProbe.ACCESS else "standard"
    require(row["run"] == str(run.id) and row["role"] == role and row["expected"] == run.observer_sha256,
            "foreign private access diagnostic")
    observed_input: object = row["before"]
    require(observed_input == row["after"] and (observed_input is None or observed_input == run.observer_sha256),
            "invalid private access diagnostic inputs")
    pid: object = row["pid"]
    exit_code: object = row["exit"]
    require(type(pid) is int and 0 < pid < (1 << 31), "invalid private access diagnostic PID")
    require(type(exit_code) is int and -(1 << 31) <= exit_code < (1 << 31),
            "invalid private access diagnostic exit")
    confirmed_input: str | None = run.observer_sha256 if observed_input is not None else None
    return PrivateAccessDiagnostic(run.id, probe, pid, exit_code, confirmed_input,
                                   _output(row["stdout"]), _output(row["stderr"]))


class PrestartCaptures:
    """Private in-memory observations for one S0.6 attempt, including refusals.

    The lab owner retains this object separately from the session result.
    Each fixed probe contributes at most one structurally decoded capture.
    Access through `records` supplies raw evidence for that owner. It conveys
    no semantic receipt admission, completion or permission to repeat a probe.
    """

    def __init__(self) -> None:
        self._lock: LockType = Lock()
        self._claimed: bool = False
        self._records: dict[PrestartProbe, CapturedProbe] = {}

    @property
    def records(self) -> tuple[CapturedProbe, ...]:
        """Return an immutable snapshot in capture order for the private lab owner."""
        with self._lock:
            return tuple(self._records.values())

    def _claim(self) -> None:
        """Spend this collection on its one attempt before any launch or request."""
        with self._lock:
            require(not self._claimed, "pre-start capture collection already used")
            self._claimed = True

    def _retain(self, probe: PrestartProbe, owner: OwnedDeploymentProcess, capture: Capture) -> None:
        """Keep the first complete, bounded capture from the admitted pipe."""
        with self._lock:
            require(self._claimed and probe not in self._records, "unexpected repeated pre-start capture")
            self._records[probe] = CapturedProbe(probe, owner, capture)


LaunchRunT = TypeVar("LaunchRunT", bound=DeploymentRun, contravariant=True)


class PrestartOwnerAttempt(Protocol[LaunchRunT]):
    """One fixed actor's retained launcher and independent recovery owner.

    The lab caller supplies this one-shot attempt from an independently
    admitted effective context. It holds the protected inputs, actual process,
    job and diagnostics, including after an interrupted start with no returned
    process. Successful start returns the original process with its private
    pipes, ready for supervision. A failed observation leaves any unresolved
    original with the external custodian.
    """

    def start(self, run: LaunchRunT, deadline: float, retirement_deadline: float) -> OwnedDeploymentProcess:
        """Start only the admitted role for this run under the original absolute cutoffs.

        Include launch and idle time. The provider independently enforces
        the cutoffs even if this host stops observing startup.
        """
        ...

    def request_retirement(self, retirement_deadline: float) -> None:
        """Signal the original attempt without waiting, even on startup failure or expiry."""
        ...

    def wait_retired(self, retirement_deadline: float) -> None:
        """Confirm settlement or raise by the original cutoff, retaining uncertain custody."""
        ...


class PrestartAccessLaunchAttempt(PrestartOwnerAttempt[DeploymentRun], Protocol):
    """One fixed access attempt with a separately staged role entry."""

    def check_launch(self, run: DeploymentRun, probe: PrestartProbe) -> Path:
        """Check the exact run, role and staged entry before starting a process."""
        ...


def launch_deployment_watchdog(
    run: DeploymentRun, executable: Path, executable_pin: Member,
    guardian_path: str, guardian_pin: Member, power_shell_pin: Member,
    owner_path: str, owner_pin: Member,
    guardian_cutoffs: tuple[int, int], watchdog_cutoffs: tuple[int, int], diagnostics: BinaryIO,
    *, observe_access_settlement: bool = False,
) -> Popen[bytes]:
    """Start the admitted native watchdog with fixed arguments and private pipes.

    The caller owns the protected staging namespace through activation and
    supplies independent pins, including the guardian's transitive inputs.
    It retains the returned process for exit observation and recovery.
    Windows paths and native deadline relationships are checked by the
    native launch boundary. The settlement option requires a new private
    diagnostic file. Its consumer must independently observe full native
    status and diagnostic writer retirement before readback.
    """
    require(executable.is_absolute() and executable.name == "prestart_deployment_watchdog.exe",
            "invalid deployment watchdog path")
    require(type(executable_pin.size) is int and 0 < executable_pin.size <= 2097152,
            "invalid deployment watchdog extent")
    require(type(executable_pin.sha256) is str and SHA256.fullmatch(executable_pin.sha256) is not None,
            "invalid deployment watchdog digest")
    facts: os.stat_result = os.fstat(diagnostics.fileno())
    require(stat.S_ISREG(facts.st_mode), "deployment watchdog requires private diagnostics")
    require(type(observe_access_settlement) is bool, "invalid watchdog settlement selection")
    if observe_access_settlement:
        require(diagnostics.readable() and diagnostics.writable() and facts.st_size == 0 and diagnostics.tell() == 0,
                "access watchdog needs an empty private diagnostic file at offset zero")
    try:
        image: bytes = read_regular(executable, executable_pin.size, "deployment watchdog")
    except (OSError, Refused) as error:
        raise ReceiptRefused("deployment watchdog read refused") from error
    require(len(image) == executable_pin.size and sha256(image).hexdigest() == executable_pin.sha256,
            "deployment watchdog image differs")
    arguments: list[str] = [
        str(executable), guardian_path, str(guardian_pin.size), guardian_pin.sha256,
        str(power_shell_pin.size), power_shell_pin.sha256, owner_path, str(owner_pin.size), owner_pin.sha256,
        str(run.id), *(str(value) for value in guardian_cutoffs), *(str(value) for value in watchdog_cutoffs),
    ]
    if observe_access_settlement:
        arguments.append("--observe-access-settlement")
    return Popen(arguments, stdin=PIPE, stdout=PIPE, stderr=diagnostics, bufsize=0)


def supervise_deployment_session(
    run: DeploymentRun, owner: OwnedDeploymentProcess, deadline: float, retirement_deadline: float,
) -> DeploymentObservation:
    """Validate the admitted run, then finish its live native window and retirement.

    Deadlines are absolute monotonic seconds on this host. The external owner
    supplies and independently enforces both, and handles launch/admission
    rollback. A refused endpoint, trailing output or unconfirmed retirement
    withholds the result. This function transfers no installation authority.
    """
    _require_deadlines(deadline, retirement_deadline)
    return supervise_deployment(_DeploymentPipe(run, owner, deadline, retirement_deadline))


def supervise_prestart_session(
    run: Run, owner: OwnedDeploymentProcess, provisioning: OwnedDeploymentProcess,
    standard: OwnedDeploymentProcess, deadline: float, retirement_deadline: float,
    *, captures: PrestartCaptures | None = None,
) -> PrestartObservation:
    """Observe the original full-mode window and two fixed same-actor owners.

    The enclosing owner supplies protected private pipes, independently admitted
    effective actors, and an original process/job per route. The two access actor
    identities in the run are those retained process owners. A failed input
    check leaves custody with the caller. After admission, all three processes
    must confirm retirement before any result can be returned. An optional
    private collection retains decoded captures independently of that result.
    """
    _require_deadlines(deadline, retirement_deadline)
    if captures is not None:
        captures._claim()
    return supervise_prestart(_admit_prestart_pipe(
        run, owner, provisioning, standard, deadline, retirement_deadline, captures,
    ))


def launch_prestart_session(
    run: DeploymentRun, provisioning: PrestartAccessLaunchAttempt, standard: PrestartAccessLaunchAttempt,
    deployment: PrestartOwnerAttempt[Run], deadline: float, retirement_deadline: float,
    *, captures: PrestartCaptures,
) -> PrestartObservation:
    """Acquire the original role owners and pass them to the fixed supervisor once.

    The lab owner supplies the signed-artifact run and its one-shot role
    providers after actor admission and under a continuously held quiescent
    window. Both cutoffs are absolute host monotonic seconds from before the
    first launch. Both access entries must match their original role, run and
    separate staging before the first launch. The coordinator checks the owners
    and their private pipes before sending any command. On a launch or handoff
    failure, it signals and observes settlement of every attempted owner under the
    original retirement cutoff. The external custodian keeps recovery
    authority across handoff and interruption of this coordinator. The caller
    owns the capture collection, including when result admission fails.
    """
    _require_deadlines(deadline, retirement_deadline)
    require(type(run) is DeploymentRun, "pre-start launches require the base deployment run")
    require_deployment_run(run)
    require(provisioning is not standard and provisioning is not deployment and standard is not deployment,
            "pre-start roles require distinct launch attempts")
    captures._claim()
    _remaining(deadline)
    access_stage: Path = provisioning.check_launch(run, PrestartProbe.ACCESS)
    _remaining(deadline)
    standard_stage: Path = standard.check_launch(run, PrestartProbe.STANDARD)
    require(access_stage != standard_stage, "pre-start access roles require distinct staging")
    attempted: list[PrestartOwnerAttempt[Run]] = []
    try:
        _remaining(deadline)
        attempted.append(provisioning)
        provisioning_owner: OwnedDeploymentProcess = provisioning.start(run, deadline, retirement_deadline)
        _remaining(deadline)
        attempted.append(standard)
        standard_owner: OwnedDeploymentProcess = standard.start(run, deadline, retirement_deadline)
        full: Run = Run(
            run.id, run.package_full_name, run.signed_package_sha256, run.engine, run.broker,
            run.companion_sha256, run.observer_sha256, run.deployment_actor, provisioning_owner, standard_owner,
        )
        require_run(full)
        _remaining(deadline)
        attempted.append(deployment)
        owner: OwnedDeploymentProcess = deployment.start(full, deadline, retirement_deadline)
        _remaining(deadline)
        window: _PrestartPipe = _admit_prestart_pipe(
            full, owner, provisioning_owner, standard_owner, deadline, retirement_deadline, captures,
        )
        _remaining(deadline)
    except BaseException as primary:
        failure: BaseException | None = _retire_unadmitted_prestart(attempted, retirement_deadline)
        if failure is None:
            raise
        raise BaseExceptionGroup("pre-start launch and retirement failed", [primary, failure]) from None
    return supervise_prestart(window)


def _admit_prestart_pipe(
    run: Run, owner: OwnedDeploymentProcess, provisioning: OwnedDeploymentProcess,
    standard: OwnedDeploymentProcess, deadline: float, retirement_deadline: float,
    captures: PrestartCaptures | None,
) -> "_PrestartPipe":
    """Check the original process identities and pipe separation before sending any command."""
    require_run(run)
    require(run.provisioning_actor is provisioning and run.standard_actor is standard,
            "foreign pre-start access owner")
    require(owner is not provisioning and owner is not standard, "pre-start roles require distinct processes")
    window: _PrestartPipe = _PrestartPipe(run, owner, provisioning, standard, deadline, retirement_deadline, captures)
    window.require_separate_pipes()
    return window


def _retire_unadmitted_prestart(attempted: list[PrestartOwnerAttempt[Run]], deadline: float) -> BaseException | None:
    """Signal every original before waiting, preserving independent refusal and expiry."""
    if not attempted:
        return None
    errors: list[BaseException] = []
    requesting: bool
    attempt: PrestartOwnerAttempt[Run]
    for requesting in (True, False):
        for attempt in attempted:
            try:
                outcome: None = attempt.request_retirement(deadline) if requesting else attempt.wait_retired(deadline)
                require(outcome is None, "invalid pre-start launch cleanup result")
            except BaseException as error:
                errors.append(error)
    try:
        _remaining(deadline)
    except TimeoutError as error:
        errors.append(error)
    return _combined_failure("pre-start launch retirement failed", errors)


class GuardedDeploymentProcess:
    """Observe the original native owner launched by an admitted WSL caller.

    The admitted guardian script and the compiled deployment watchdog supply
    this exit contract. The caller keeps the original Popen and its private
    diagnostic file. Native job recovery continues if a pipe closes or
    observing the launcher exit times out.
    """

    def __init__(self, process: Popen[bytes]):
        stdin: BinaryIO | None = process.stdin
        stdout: BinaryIO | None = process.stdout
        require(stdin is not None and stdout is not None and process.stderr is None,
                "deployment guardian requires private pipes and separate diagnostics")
        self.stdin: BinaryIO = stdin
        self.stdout: BinaryIO = stdout
        self._process = process

    def wait_retired(self, deadline: float) -> None:
        require(self.stdin.closed and self.stdout.closed, "deployment guardian pipes still owned by supervisor")
        status: int = self._process.wait(timeout=_remaining(deadline))
        _remaining(deadline)
        require(status == 0, "native deployment guardian refused retirement")


class _ProcessPipe(Generic[RunT]):
    """Bounded framing on one retained process's original pipe ends."""

    def __init__(self, run: RunT, owner: OwnedDeploymentProcess,
                 deadline: float, retirement_deadline: float, captures: PrestartCaptures | None = None):
        self._run: RunT = run
        self._owner: OwnedDeploymentProcess = owner
        self._stdin: BinaryIO = owner.stdin
        self._stdout: BinaryIO = owner.stdout
        self._deadline: float = deadline
        self._retirement_deadline: float = retirement_deadline
        self._captures: PrestartCaptures | None = captures
        self._frame: bytearray = bytearray()

    def _capture_record(self, row: dict[str, object], probe: PrestartProbe, actor: object) -> Capture:
        """Save the decoded private frame before semantic validation and pipe retirement."""
        capture: Capture = _captured(row, self._run, actor)
        if self._captures is not None:
            self._captures._retain(probe, self._owner, capture)
        return capture

    def _send(self, command: bytes, deadline: float) -> None:
        fd: int = self._stdin.fileno()
        ready: list[int]
        _, ready, _ = select.select([], [fd], [], _remaining(deadline))
        if not ready:
            raise TimeoutError("deployment request pipe deadline")
        _remaining(deadline)
        require(os.write(fd, command) == len(command), "short deployment request write")

    def _receive(self, deadline: float) -> dict[str, object]:
        """Read a bounded private frame for the original run with redacted decoding failures."""
        fd: int = self._stdout.fileno()
        self._fill(fd, 4, deadline)
        length: int = int.from_bytes(self._frame[:4], "little")
        if not 0 < length <= MAX_FRAME:
            self._frame.clear()
            raise ReceiptRefused("invalid deployment frame length")
        self._fill(fd, length + 4, deadline)
        payload: bytes = bytes(self._frame[4:])
        self._frame.clear()
        try:
            row: object = json.loads(payload.decode("utf-8"), object_pairs_hook=unique_object)
        except (ValueError, Refused, RecursionError):
            pass
        else:
            require(type(row) is dict and row.get("run") == str(self._run.id), "foreign deployment frame")
            return row
        raise ReceiptRefused("invalid deployment frame JSON")

    def _fill(self, fd: int, length: int, deadline: float) -> None:
        # Retain at most one bounded frame if observation expires mid-read.
        while len(self._frame) < length:
            chunk: bytes = _read(fd, length - len(self._frame), deadline)
            require(bool(chunk), "incomplete deployment frame")
            self._frame.extend(chunk)

    def _wait_retired(self) -> None:
        result: None = self._owner.wait_retired(self._retirement_deadline)
        require(result is None, "invalid native owner retirement result")
        _remaining(self._retirement_deadline)


class _DeploymentPipe(_ProcessPipe[RunT]):
    def __init__(self, run: RunT, owner: OwnedDeploymentProcess,
                 deadline: float, retirement_deadline: float, captures: PrestartCaptures | None = None):
        super().__init__(run, owner, deadline, retirement_deadline, captures)
        self._taken = False
        self._observation_received = False
        self._pending_reply: tuple[str, tuple[str, ...]] | None = None

    def take_run(self) -> RunT:
        require(not self._taken, "deployment session already taken")
        self._taken = True
        return self._run

    def current_run(self) -> RunT:
        if self._observation_received:
            self._exchange(b"F", "confirmed")
        else:
            self._exchange(b"C", "current")
        return self._run

    def initial_scm_types(self, run: RunT) -> ScmTypeSample:
        self._require_run(run)
        row: dict[str, object] = self._exchange(b"S", "sample", "engine", "broker")
        return ScmTypeSample(_sample_pair(row["engine"]), _sample_pair(row["broker"]))

    def capture_identity(self, run: RunT) -> Capture:
        self._require_run(run)
        return self._capture(b"I", "identity", PrestartProbe.IDENTITY)

    def capture_deployment_observation(self, run: RunT) -> Capture:
        self._require_run(run)
        capture: Capture = self._capture(b"O", "observation", PrestartProbe.OBSERVATION)
        self._observation_received = True
        return capture

    def _require_run(self, run: RunT) -> None:
        require(run is self._run and self._taken, "foreign deployment session run")

    def _capture(self, command: bytes, kind: str, probe: PrestartProbe) -> Capture:
        """Retain the decoded capture even if elapsed work time precludes admission."""
        row: dict[str, object] = self._request_reply(command, kind, *_CAPTURE_FIELDS)
        try:
            capture: Capture = self._capture_record(row, probe, self._run.deployment_actor)
        except ReceiptRefused:
            _remaining(self._deadline)
            raise
        _remaining(self._deadline)
        return capture

    def _exchange(self, command: bytes, kind: str, *fields: str) -> dict[str, object]:
        row: dict[str, object] = self._request_reply(command, kind, *fields)
        _remaining(self._deadline)
        return row

    def _request_reply(self, command: bytes, kind: str, *fields: str) -> dict[str, object]:
        """Consume one matching frame. A timeout while reading leaves one pending reply."""
        self._send(command, self._deadline)
        try:
            row: dict[str, object] = self._receive(self._deadline)
        except TimeoutError:
            self._pending_reply = (kind, fields)
            raise
        if row.get("kind") == "refused":
            raise ReceiptRefused("native deployment refused " + _refusal(row))
        _expect(row, kind, *fields)
        return row

    def _read_retirement(self) -> None:
        deadline: float = self._retirement_deadline
        row: dict[str, object] = self._receive(deadline)
        if self._pending_reply is not None and row.get("kind") != "retired":
            # A late reply cannot recover observation. Drain just the prior
            # response so the independent retirement frame can still be read.
            kind: str
            fields: tuple[str, ...]
            kind, fields = self._pending_reply
            if row.get("kind") == "refused":
                _refusal(row)
            else:
                _expect(row, kind, *fields)
            row = self._receive(deadline)
        _expect(row, "retired", "cleanup")
        codes: tuple[str, ...] = _cleanup_codes(row["cleanup"])
        native_failure: ReceiptRefused | None = None
        if codes:
            native_failure = ReceiptRefused("native deployment cleanup refused " + ", ".join(codes))
        try:
            require(_read(self._stdout.fileno(), 1, deadline) == b"", "trailing native deployment output")
        except BaseException as error:
            if native_failure is not None:
                raise BaseExceptionGroup("native deployment cleanup and stream failed", [native_failure, error]) from None
            raise
        if native_failure is not None:
            raise native_failure

    def retire_run(self, run: RunT) -> BaseException | None:
        self._require_run(run)
        return _retire("native deployment retirement failed", (
            lambda: self._send(b"R", self._retirement_deadline),
            self._stdin.close,
            self._read_retirement,
            self._stdout.close,
            self._wait_retired,
        ))


class _AccessPipe(_ProcessPipe[Run]):
    """Consume one pre-admitted same-actor observation and its original process."""

    def __init__(self, run: Run, owner: OwnedDeploymentProcess, deadline: float,
                 retirement_deadline: float, probe: PrestartProbe, captures: PrestartCaptures | None):
        super().__init__(run, owner, deadline, retirement_deadline, captures)
        self.probe: PrestartProbe = probe
        self.command: bytes = b"A" if probe is PrestartProbe.ACCESS else b"U"
        self._kind: str = "access" if probe is PrestartProbe.ACCESS else "standard"
        self.current_kind: str = self._kind + "-current"
        self._input_close_attempted: bool = False
        self._output_close_attempted: bool = False
        self._retirement_attempted: bool = False
        self._capture_status_refusal: AccessStatusOverflow | None = None

    def capture(self, actor: object) -> Capture:
        require(actor is self._owner and not self._retirement_attempted, "access owner is foreign or retired")
        self._send(self.command, self._deadline)
        self._close_input()
        row: dict[str, object] = self._receive(self._deadline)
        if row.get("kind") == "refused":
            _expect(row, "refused", "code", "cleanup")
            cause: ReceiptRefused = ReceiptRefused("native access refused " + _code(row["code"]))
            codes: tuple[str, ...] = _cleanup_codes(row["cleanup"])
            if codes:
                cleanup: ReceiptRefused = ReceiptRefused("native access cleanup refused " + ", ".join(codes))
                raise ExceptionGroup("native access observation and cleanup failed", [cause, cleanup])
            raise cause
        _expect(row, self._kind, *_CAPTURE_FIELDS)
        result: Capture = self._capture_record(row, self.probe, actor)
        require(_read(self._stdout.fileno(), 1, self._deadline) == b"", "trailing native access output")
        self._close_output()
        _remaining(self._deadline)
        self._retirement_attempted = True
        try:
            observed: None = self._owner.wait_retired(self._deadline)
        except (TimeoutError, TimeoutExpired, AccessStatusOverflow) as error:
            # The one original process remains owned. Retry only the observation
            # after all other owners have received retirement, under its later cutoff.
            self._retirement_attempted = False
            if isinstance(error, AccessStatusOverflow):
                self._capture_status_refusal = error
            raise
        require(observed is None, "invalid native owner retirement result")
        _remaining(self._deadline)
        return result

    def _close_input(self) -> None:
        if not self._input_close_attempted:
            self._input_close_attempted = True
            self._stdin.close()

    def _close_output(self) -> None:
        if not self._output_close_attempted:
            self._output_close_attempted = True
            self._stdout.close()

    def close_pipes(self) -> None:
        if isinstance(self._owner, AccessWriterSettlement):
            # Signal first. The WSL relay keeps its original output writer
            # until the owner drains the bounded remainder to actual EOF.
            self._close_input()
            return
        failure: BaseException | None = _retire("native access pipe closure failed", (
            self._close_input, self._close_output,
        ))
        if failure is not None:
            raise failure

    def observe_retirement(self) -> None:
        if self._retirement_attempted:
            return
        self._retirement_attempted = True
        if isinstance(self._owner, AccessWriterSettlement):
            try:
                observed: None = self._owner.wait_access_settled(self._retirement_deadline)
            except AccessStatusOverflow as error:
                # The relay has exited. The original capture refusal still
                # records the missing native witness, even if another owner fails.
                if error is not self._capture_status_refusal:
                    raise
            else:
                require(observed is None, "invalid native access writer retirement result")
            _remaining(self._retirement_deadline)
        else:
            self._wait_retired()


class _PrestartPipe(_DeploymentPipe[Run]):
    """Keep the original deployment window live across both fixed access roles."""

    def __init__(self, run: Run, owner: OwnedDeploymentProcess, provisioning: OwnedDeploymentProcess,
                 standard: OwnedDeploymentProcess, deadline: float, retirement_deadline: float,
                 captures: PrestartCaptures | None):
        super().__init__(run, owner, deadline, retirement_deadline, captures)
        self._access: tuple[_AccessPipe, _AccessPipe] = (
            _AccessPipe(run, provisioning, deadline, retirement_deadline, PrestartProbe.ACCESS, captures),
            _AccessPipe(run, standard, deadline, retirement_deadline, PrestartProbe.STANDARD, captures),
        )
        self._next_access: int = 0
        self._access_ready: bool = False

    def require_separate_pipes(self) -> None:
        """Check the six retained pipe ends before the supervisor consumes a run."""
        from fcntl import F_GETFL, fcntl

        pipes: set[tuple[int, int]] = set()
        stream: BinaryIO
        direction: int
        for process in (self, *self._access):
            for stream, direction in ((process._stdin, os.O_WRONLY), (process._stdout, os.O_RDONLY)):
                fd: int = stream.fileno()
                facts: os.stat_result = os.fstat(fd)
                identity: tuple[int, int] = (facts.st_dev, facts.st_ino)
                require(stat.S_ISFIFO(facts.st_mode) and identity not in pipes,
                        "pre-start owners require separate private pipes")
                access_mode: int = fcntl(fd, F_GETFL) & os.O_ACCMODE
                require(access_mode == direction, "pre-start owner pipe has wrong direction")
                pipes.add(identity)

    def current_run(self) -> Run:
        if not self._observation_received or self._next_access == len(self._access):
            return super().current_run()
        require(not self._access_ready, "pre-start access check already spent")
        selected: _AccessPipe = self._access[self._next_access]
        self._exchange(selected.command, selected.current_kind)
        self._access_ready = True
        return self._run

    def capture_observer(self, run: Run, actor: object, arguments: tuple[str, ...]) -> Capture:
        self._require_run(run)
        require(self._access_ready, "pre-start access capture without a current window check")
        selected: _AccessPipe = self._access[self._next_access]
        require(arguments == (selected.probe.value, *OBSERVER_ARGUMENTS), "unexpected fixed pre-start access selector")
        self._access_ready = False
        self._next_access += 1
        return selected.capture(actor)

    def retire_run(self, run: Run) -> BaseException | None:
        self._require_run(run)
        # Signal both access owners before a blocked deployment wait can use up
        # their common cutoff. The deployment owner still receives original R/EOF.
        provisioning, standard = self._access
        closed: BaseException | None = _retire("pre-start access pipe closure failed", (
            provisioning.close_pipes, standard.close_pipes,
        ))
        original: BaseException | None = super().retire_run(run)
        observed: BaseException | None = _retire("pre-start access retirement failed", (
            provisioning.observe_retirement, standard.observe_retirement,
        ))
        errors: list[BaseException] = [error for error in (closed, original, observed) if error is not None]
        return _combined_failure("pre-start owner retirement failed", errors)


def _captured(row: dict[str, object], run: DeploymentRun, actor: object) -> Capture:
    """Decode the bounded child record from its original actor's fixed owner."""
    pid: object = row["pid"]
    exit_code: object = row["exit"]
    before: object = row["before"]
    after: object = row["after"]
    require(type(pid) is int and 0 < pid <= 0xFFFFFFFF, "invalid native child PID")
    require(type(exit_code) is int and -(1 << 31) <= exit_code < (1 << 31), "invalid native child exit")
    require(type(before) is str and SHA256.fullmatch(before) is not None
            and type(after) is str and SHA256.fullmatch(after) is not None, "invalid native input digest")
    return Capture(run.id, actor, pid, before, after, exit_code, True, _output(row["stdout"]), _output(row["stderr"]))


def _retire(message: str, operations: tuple[Callable[[], None], ...]) -> BaseException | None:
    errors: list[BaseException] = []
    for operation in operations:
        try:
            operation()
        except BaseException as error:
            errors.append(error)
    return _combined_failure(message, errors)


def _combined_failure(message: str, errors: list[BaseException]) -> BaseException | None:
    if not errors:
        return None
    if len(errors) == 1:
        return errors[0]
    return BaseExceptionGroup(message, errors)


def _expect(row: dict[str, object], kind: str, *fields: str) -> None:
    require(set(row) == {"run", "kind", *fields} and row["kind"] == kind, "unexpected deployment frame")


def _refusal(row: dict[str, object]) -> str:
    _expect(row, "refused", "code")
    return _code(row["code"])


def _sample_pair(value: object) -> tuple[int, int]:
    require(type(value) is list and len(value) == 2 and all(is_dword(item) for item in value),
            "invalid native SCM sample")
    return value[0], value[1]


def _output(value: object) -> bytes:
    """Decode a bounded canonical stream without exposing its bytes in errors."""
    require(type(value) is str and len(value) <= 4 * ((MAX_OUTPUT + 2) // 3), "invalid native child output")
    try:
        result: bytes = base64.b64decode(value, validate=True)
    except (binascii.Error, ValueError):
        pass
    else:
        require(len(result) <= MAX_OUTPUT and base64.b64encode(result).decode("ascii") == value,
                "noncanonical native child output")
        return result
    raise ReceiptRefused("invalid native child output")


def _code(value: object) -> str:
    require(type(value) is str and CODE.fullmatch(value) is not None, "invalid native deployment refusal code")
    return value


def _cleanup_codes(value: object) -> tuple[str, ...]:
    require(type(value) is list and len(value) <= 8, "invalid deployment cleanup list")
    return tuple(_code(code) for code in value)


def _require_deadlines(deadline: float, retirement_deadline: float) -> None:
    require(math.isfinite(deadline) and math.isfinite(retirement_deadline) and deadline <= retirement_deadline,
            "invalid deployment session deadlines")


def _remaining(deadline: float) -> float:
    remaining: float = deadline - time.monotonic()
    if remaining <= 0:
        raise TimeoutError("deployment pipe deadline")
    return remaining


def _read(fd: int, length: int, deadline: float) -> bytes:
    ready: list[int]
    ready, _, _ = select.select([fd], [], [], _remaining(deadline))
    if not ready:
        raise TimeoutError("deployment response pipe deadline")
    return os.read(fd, length)
