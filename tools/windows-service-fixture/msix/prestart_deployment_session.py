# SPDX-License-Identifier: GPL-2.0-only
"""Drive one admitted S0.6 Windows deployment window over private process pipes."""

import base64
import binascii
from collections.abc import Callable
from hashlib import sha256
import json
import math
import os
from pathlib import Path
import re
import select
import stat
from subprocess import PIPE, Popen, TimeoutExpired
import time
from typing import BinaryIO, Generic, Protocol

from check_package import Refused, read_regular, unique_object
from prestart_receipts import (
    MAX_OUTPUT, SHA256, Capture, DeploymentObservation, DeploymentRun, Member, PrestartObservation,
    PrestartProbe, ReceiptRefused, Run, ScmTypeSample, is_dword, require, require_run,
)
from prestart_supervisor import OBSERVER_ARGUMENTS, RunT, supervise_deployment, supervise_prestart


MAX_FRAME = 24000
CODE = re.compile(r"[a-z0-9-]{1,80}")
_CAPTURE_FIELDS: tuple[str, ...] = ("pid", "before", "after", "exit", "stdout", "stderr")


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


def launch_deployment_watchdog(
    run: DeploymentRun, executable: Path, executable_pin: Member,
    guardian_path: str, guardian_pin: Member, power_shell_pin: Member,
    owner_path: str, owner_pin: Member,
    guardian_cutoffs: tuple[int, int], watchdog_cutoffs: tuple[int, int], diagnostics: BinaryIO,
) -> Popen[bytes]:
    """Start the admitted native watchdog with fixed arguments and private pipes.

    The caller owns the protected staging namespace through activation and
    supplies independent pins, including the guardian's transitive inputs.
    It retains the returned process for exit observation and recovery.
    Windows paths and native deadline relationships are checked by the
    native launch boundary. The diagnostic file supplies no authority.
    """
    require(executable.is_absolute() and executable.name == "prestart_deployment_watchdog.exe",
            "invalid deployment watchdog path")
    require(type(executable_pin.size) is int and 0 < executable_pin.size <= 2097152,
            "invalid deployment watchdog extent")
    require(type(executable_pin.sha256) is str and SHA256.fullmatch(executable_pin.sha256) is not None,
            "invalid deployment watchdog digest")
    require(stat.S_ISREG(os.fstat(diagnostics.fileno()).st_mode), "deployment watchdog requires private diagnostics")
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
) -> PrestartObservation:
    """Observe the original full-mode window and two fixed same-actor owners.

    The enclosing owner supplies protected private pipes, independently admitted
    effective actors, and an original process/job per route. The two access actor
    identities in the run are those retained process owners. A failed input
    check leaves custody with the caller. After admission, all three processes
    must confirm retirement before any result can be returned.
    """
    _require_deadlines(deadline, retirement_deadline)
    require_run(run)
    require(run.provisioning_actor is provisioning and run.standard_actor is standard,
            "foreign pre-start access owner")
    window: _PrestartPipe = _PrestartPipe(run, owner, provisioning, standard, deadline, retirement_deadline)
    window.require_separate_pipes()
    return supervise_prestart(window)


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
                 deadline: float, retirement_deadline: float):
        self._run: RunT = run
        self._owner: OwnedDeploymentProcess = owner
        self._stdin: BinaryIO = owner.stdin
        self._stdout: BinaryIO = owner.stdout
        self._deadline: float = deadline
        self._retirement_deadline: float = retirement_deadline
        self._frame: bytearray = bytearray()

    def _send(self, command: bytes, deadline: float) -> None:
        fd: int = self._stdin.fileno()
        ready: list[int]
        _, ready, _ = select.select([], [fd], [], _remaining(deadline))
        if not ready:
            raise TimeoutError("deployment request pipe deadline")
        _remaining(deadline)
        require(os.write(fd, command) == len(command), "short deployment request write")

    def _receive(self, deadline: float) -> dict[str, object]:
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
        except (ValueError, Refused, RecursionError) as error:
            raise ReceiptRefused("invalid deployment frame JSON") from error
        require(type(row) is dict and row.get("run") == str(self._run.id), "foreign deployment frame")
        return row

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
                 deadline: float, retirement_deadline: float):
        super().__init__(run, owner, deadline, retirement_deadline)
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
        return self._capture(b"I", "identity")

    def capture_deployment_observation(self, run: RunT) -> Capture:
        self._require_run(run)
        capture: Capture = self._capture(b"O", "observation")
        self._observation_received = True
        return capture

    def _require_run(self, run: RunT) -> None:
        require(run is self._run and self._taken, "foreign deployment session run")

    def _capture(self, command: bytes, kind: str) -> Capture:
        row: dict[str, object] = self._exchange(command, kind, *_CAPTURE_FIELDS)
        return _captured(row, self._run, self._run.deployment_actor)

    def _exchange(self, command: bytes, kind: str, *fields: str) -> dict[str, object]:
        self._send(command, self._deadline)
        try:
            row: dict[str, object] = self._receive(self._deadline)
        except TimeoutError:
            self._pending_reply = (kind, fields)
            raise
        if row.get("kind") == "refused":
            raise ReceiptRefused("native deployment refused " + _refusal(row))
        _expect(row, kind, *fields)
        # The complete reply has been consumed. A late success still expires,
        # but retirement must not expect that reply again.
        _remaining(self._deadline)
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
                 retirement_deadline: float, probe: PrestartProbe):
        super().__init__(run, owner, deadline, retirement_deadline)
        self.probe: PrestartProbe = probe
        self.command: bytes = b"A" if probe is PrestartProbe.ACCESS else b"U"
        self._kind: str = "access" if probe is PrestartProbe.ACCESS else "standard"
        self.current_kind: str = self._kind + "-current"
        self._input_close_attempted: bool = False
        self._output_close_attempted: bool = False
        self._retirement_attempted: bool = False

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
        result: Capture = _captured(row, self._run, actor)
        require(_read(self._stdout.fileno(), 1, self._deadline) == b"", "trailing native access output")
        self._close_output()
        _remaining(self._deadline)
        self._retirement_attempted = True
        try:
            observed: None = self._owner.wait_retired(self._deadline)
        except (TimeoutError, TimeoutExpired):
            # The one original process remains owned. Retry only the observation
            # after all other owners have received retirement, under its later cutoff.
            self._retirement_attempted = False
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
        failure: BaseException | None = _retire("native access pipe closure failed", (
            self._close_input, self._close_output,
        ))
        if failure is not None:
            raise failure

    def observe_retirement(self) -> None:
        if self._retirement_attempted:
            return
        self._retirement_attempted = True
        self._wait_retired()


class _PrestartPipe(_DeploymentPipe[Run]):
    """Keep the original deployment window live across both fixed access roles."""

    def __init__(self, run: Run, owner: OwnedDeploymentProcess, provisioning: OwnedDeploymentProcess,
                 standard: OwnedDeploymentProcess, deadline: float, retirement_deadline: float):
        super().__init__(run, owner, deadline, retirement_deadline)
        self._access: tuple[_AccessPipe, _AccessPipe] = (
            _AccessPipe(run, provisioning, deadline, retirement_deadline, PrestartProbe.ACCESS),
            _AccessPipe(run, standard, deadline, retirement_deadline, PrestartProbe.STANDARD),
        )
        self._next_access: int = 0
        self._access_ready: bool = False

    def require_separate_pipes(self) -> None:
        """Check the six retained pipe ends before the supervisor consumes a run."""
        pipes: set[tuple[int, int]] = set()
        for process in (self, *self._access):
            for stream in (process._stdin, process._stdout):
                facts: os.stat_result = os.fstat(stream.fileno())
                identity: tuple[int, int] = (facts.st_dev, facts.st_ino)
                require(stat.S_ISFIFO(facts.st_mode) and identity not in pipes,
                        "pre-start owners require separate private pipes")
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
    require(type(value) is str and len(value) <= 4 * ((MAX_OUTPUT + 2) // 3), "invalid native child output")
    try:
        result: bytes = base64.b64decode(value, validate=True)
    except (binascii.Error, ValueError) as error:
        raise ReceiptRefused("invalid native child output") from error
    require(len(result) <= MAX_OUTPUT and base64.b64encode(result).decode("ascii") == value,
            "noncanonical native child output")
    return result


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
