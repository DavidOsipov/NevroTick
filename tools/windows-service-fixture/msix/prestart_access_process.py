# SPDX-License-Identifier: GPL-2.0-only
"""Carry one admitted access watchdog's live pipes and separate native status over WSL."""

from dataclasses import dataclass, field
from hashlib import sha256
import os
from pathlib import Path
import re
from secrets import token_hex
import stat
from subprocess import PIPE, Popen
import time
from typing import BinaryIO

from check_package import Refused, read_regular
from prestart_deployment_session import (
    MAX_FRAME, PrivateAccessDiagnostic, _decode_private_access_diagnostic, _read, _remaining, _require_deadlines,
)
from prestart_receipts import DeploymentRun, Member, PrestartProbe, ReceiptRefused, SHA256, require, require_deployment_run


_MAX_STATUS: int = 271
_MAX_DIAGNOSTICS: int = 32768
_MAX_NATIVE_MS: int = 120000
_MAX_RETIREMENT_MS: int = 30000
_STATUS: re.Pattern[bytes] = re.compile(
    rb"nevrotick-prestart-access-exit-v1 ([0-9a-f-]{36}) (access|standard) ([0-9a-f]{32}) ([0-9a-f]{64}) "
    rb"([1-9][0-9]{0,9}) (0|[1-9][0-9]{0,9}) (0|[1-9][0-9]{0,4}) ([0-9a-f]{64})\n"
)


@dataclass(frozen=True, slots=True)
class AccessWatchdogInputs:
    """Fixed files in one protected native staging directory and original pins."""

    parent: Path
    parent_pin: Member
    watchdog: Path
    watchdog_pin: Member
    guardian: Path
    guardian_pin: Member
    powershell_pin: Member
    entry: Path
    entry_pin: Member


@dataclass(frozen=True, slots=True)
class _Settlement:
    """Result observed by the retained Windows parent after diagnostic writer settlement."""

    native_pid: int
    native_exit: int
    size: int
    digest: str


def _windows_path(path: Path) -> str:
    """Translate an admitted local drive path; refuse other WSL namespaces."""
    components: tuple[str, ...] = path.parts
    require(len(components) >= 4 and components[:2] == ("/", "mnt")
            and re.fullmatch("[a-z]", components[2]) is not None
            and all(part not in {".", ".."} and not re.search(r'["<>:|?*\\/\x00-\x1f]', part)
                    and not part.endswith((".", " ")) for part in components[3:]),
            "access watchdog requires a local Windows staging path")
    return components[2].upper() + ":\\" + "\\".join(components[3:])


def _pinned_image(path: Path, pin: Member) -> None:
    """Check a fixed executable before the admitted caller activates it by path."""
    require(type(pin.size) is int and 0 < pin.size <= 2097152
            and type(pin.sha256) is str and SHA256.fullmatch(pin.sha256) is not None,
            "invalid access watchdog image pin")
    try:
        image: bytes = read_regular(path, pin.size, "pre-start access image")
    except (OSError, Refused) as error:
        raise ReceiptRefused("pre-start access image read refused") from error
    require(len(image) == pin.size and sha256(image).hexdigest() == pin.sha256,
            "pre-start access image differs")


def _valid_budgets(budgets: tuple[int, int], *, minimum_grace_ms: int = 1) -> bool:
    work, retirement = budgets
    return (type(work) is int and type(retirement) is int and 0 < work < retirement <= _MAX_NATIVE_MS
            and minimum_grace_ms <= retirement - work <= _MAX_RETIREMENT_MS)


def _limited_budgets(budgets: tuple[int, int], now: float, deadline: float,
                     retirement_deadline: float, *, minimum_grace_ms: int = 1) -> tuple[int, int]:
    """Intersect the admitted native budgets with the original host allowance."""
    work: int = min(budgets[0], int((deadline - now) * 1000))
    retirement: int = min(budgets[1], int((retirement_deadline - now) * 1000), work + _MAX_RETIREMENT_MS)
    require(_valid_budgets((work, retirement), minimum_grace_ms=minimum_grace_ms),
            "insufficient pre-start access launch allowance")
    return work, retirement


def _decode_status(raw: bytes, run: DeploymentRun, role: str, nonce: str, watchdog: str) -> _Settlement:
    """Bind one fixed settlement record to the launch and keep the full native DWORD."""
    match: re.Match[bytes] | None = _STATUS.fullmatch(raw)
    require(match is not None, "invalid native access settlement")
    assert match is not None
    require(match.groups()[:4] == tuple(value.encode("ascii") for value in (str(run.id), role, nonce, watchdog)),
            "foreign native access settlement")
    pid: int = int(match[5])
    native_exit: int = int(match[6])
    size: int = int(match[7])
    require(0 < pid <= 0xFFFFFFFF and 0 <= native_exit <= 0xFFFFFFFF and 0 <= size <= _MAX_DIAGNOSTICS,
            "native access settlement outside bounds")
    return _Settlement(pid, native_exit, size, match[8].decode("ascii"))


def _diagnostic_identity(path: Path, stream: BinaryIO) -> os.stat_result:
    """Require the held single-link diagnostic file to remain the staged object."""
    held: os.stat_result = os.fstat(stream.fileno())
    named: os.stat_result = path.stat(follow_symlinks=False)
    require(stat.S_ISREG(held.st_mode) and held.st_nlink == 1 and os.path.samestat(held, named),
            "foreign private native access diagnostics")
    return held


@dataclass(slots=True)
class _AccessProcess:
    """The original protocol and status pipes. A timeout does not replace them."""

    stdin: BinaryIO
    stdout: BinaryIO
    _status: BinaryIO = field(repr=False)
    _process: Popen[bytes] = field(repr=False)
    _run: DeploymentRun
    _probe: PrestartProbe
    _role: str
    _nonce: str
    _watchdog: str
    _diagnostic_path: Path
    _diagnostics: BinaryIO = field(repr=False)
    _retirement_deadline: float
    _buffer: bytearray = field(default_factory=bytearray, init=False, repr=False)
    _status_eof: bool = field(default=False, init=False)
    _settlement: _Settlement | None = field(default=None, init=False)
    _discarded_wire_bytes: int = field(default=0, init=False)

    def _wait_settlement(self, deadline: float) -> _Settlement:
        """Wait for the original WSL pipe EOF and parent exit, then verify native bytes."""
        require(self.stdin.closed and self.stdout.closed, "native access pipes still owned by supervisor")
        _require_deadlines(deadline, self._retirement_deadline)
        _remaining(deadline)
        if self._settlement is not None:
            return self._settlement
        while not self._status_eof:
            chunk: bytes = _read(self._status.fileno(), _MAX_STATUS + 1 - len(self._buffer), deadline)
            if not chunk:
                self._status_eof = True
                self._status.close()
                break
            self._buffer.extend(chunk)
            if len(self._buffer) > _MAX_STATUS:
                self._status_eof = True
                self._status.close()
            require(len(self._buffer) <= _MAX_STATUS, "oversized native access settlement")
        status: int = self._process.wait(timeout=_remaining(deadline))
        _remaining(deadline)
        require(status == 0, "native access parent refused settlement")
        observed: _Settlement = _decode_status(bytes(self._buffer), self._run, self._role, self._nonce, self._watchdog)
        self._check_diagnostics(observed)
        _remaining(deadline)
        self._settlement = observed
        return observed

    def _check_diagnostics(self, status: _Settlement) -> bytes:
        """Read only the settled original file and check its parent's byte witness."""
        before: os.stat_result = _diagnostic_identity(self._diagnostic_path, self._diagnostics)
        require(before.st_size == status.size, "native access diagnostic extent differs")
        raw: bytes = os.pread(self._diagnostics.fileno(), status.size + 1, 0)
        after: os.stat_result = _diagnostic_identity(self._diagnostic_path, self._diagnostics)
        require(len(raw) == before.st_size == after.st_size
                and before.st_mtime_ns == after.st_mtime_ns and before.st_ctime_ns == after.st_ctime_ns
                and sha256(raw).hexdigest() == status.digest, "native access diagnostic bytes differ")
        return raw

    def wait_retired(self, deadline: float) -> None:
        """Require native success. Exact exit 3 permits separate diagnostic readback only."""
        status: _Settlement = self._wait_settlement(deadline)
        require(status.native_exit == 0, "native access watchdog refused operation")

    def wait_access_settled(self, deadline: float) -> None:
        """Confirm cleanup of the original access owner independently of its result."""
        require(self.stdin.closed, "native access input still owned by supervisor")
        _require_deadlines(deadline, self._retirement_deadline)
        while not self.stdout.closed:
            # Read no more than the remainder of one admitted frame plus a
            # sentinel byte. Closing a still-written POSIX relay could affect
            # the independent Windows status channel.
            limit: int = MAX_FRAME + 4
            require(self._discarded_wire_bytes <= limit, "excess native access output during retirement")
            chunk: bytes = _read(self.stdout.fileno(), min(4096, limit + 1 - self._discarded_wire_bytes), deadline)
            if not chunk:
                self.stdout.close()
            self._discarded_wire_bytes += len(chunk)
        status: _Settlement = self._wait_settlement(deadline)
        require(status.native_exit in {0, 3}, "native access retirement unconfirmed")

    def private_diagnostic(self) -> PrivateAccessDiagnostic | None:
        """Read a captured refusal only after the exact full status 3 and writer settlement."""
        status: _Settlement | None = self._settlement
        require(status is not None and status.native_exit == 3, "native access diagnostics not settled")
        assert status is not None
        raw: bytes = self._check_diagnostics(status)
        return _decode_private_access_diagnostic(self._run, self._probe, raw)


class PrestartAccessAttempt:
    """One admitted actor's one-shot launch provider for launch_prestart_session.

    The caller owns the primary-token context, protected Windows staging and
    original diagnostic descriptor for the lifetime of the attempt. The
    Windows parent enforces its finite native allowance independently. Host
    observation still uses the original absolute deadlines. This provider
    remains the recovery owner if start or settlement is interrupted.
    """

    def __init__(self, inputs: AccessWatchdogInputs, probe: PrestartProbe,
                 diagnostic_path: Path, diagnostics: BinaryIO,
                 guardian_cutoffs: tuple[int, int], watchdog_cutoffs: tuple[int, int]):
        require(probe is PrestartProbe.ACCESS or probe is PrestartProbe.STANDARD,
                "invalid native access role")
        require(inputs.parent.name == "prestart_access_parent.exe"
                and inputs.watchdog.name == "prestart_deployment_watchdog.exe"
                and inputs.guardian.name == "prestart_deployment_guardian.ps1"
                and inputs.entry.name == "prestart_deployment_owner.ps1"
                and diagnostic_path.name == "prestart-access-diagnostics.log"
                and all(path.parent == inputs.parent.parent for path in
                        (inputs.watchdog, inputs.guardian, inputs.entry, diagnostic_path)),
                "invalid fixed native access staging")
        require(_valid_budgets(guardian_cutoffs) and _valid_budgets(watchdog_cutoffs, minimum_grace_ms=2),
                "invalid native access budgets")
        self._inputs: AccessWatchdogInputs = inputs
        self._probe: PrestartProbe = probe
        self._role: str = "access" if probe is PrestartProbe.ACCESS else "standard"
        self._diagnostic_path: Path = diagnostic_path
        self._diagnostics: BinaryIO = diagnostics
        self._guardian_cutoffs: tuple[int, int] = guardian_cutoffs
        self._watchdog_cutoffs: tuple[int, int] = watchdog_cutoffs
        self._spent: bool = False
        self._attempted: bool = False
        self._process: Popen[bytes] | None = None
        self._owner: _AccessProcess | None = None
        self._retirement_deadline: float | None = None

    def start(self, run: DeploymentRun, deadline: float, retirement_deadline: float) -> _AccessProcess:
        """Start the fixed watchdog parent once, keeping its original Popen on return."""
        require(not self._spent, "native access attempt already used")
        self._spent = True
        self._retirement_deadline = retirement_deadline
        require_deployment_run(run)
        _require_deadlines(deadline, retirement_deadline)
        _remaining(deadline)
        inputs: AccessWatchdogInputs = self._inputs
        _windows_path(inputs.parent)
        _pinned_image(inputs.parent, inputs.parent_pin)
        _pinned_image(inputs.watchdog, inputs.watchdog_pin)
        initial: os.stat_result = _diagnostic_identity(self._diagnostic_path, self._diagnostics)
        require(self._diagnostics.readable() and initial.st_size == 0 and self._diagnostics.tell() == 0,
                "native access requires an empty held diagnostic file")
        nonce: str = token_hex(16)
        arguments: list[str] = [
            str(inputs.parent), _windows_path(inputs.watchdog), str(inputs.watchdog_pin.size),
            inputs.watchdog_pin.sha256, self._role, nonce, _windows_path(self._diagnostic_path),
            _windows_path(inputs.guardian), str(inputs.guardian_pin.size), inputs.guardian_pin.sha256,
            str(inputs.powershell_pin.size), inputs.powershell_pin.sha256, _windows_path(inputs.entry),
            str(inputs.entry_pin.size), inputs.entry_pin.sha256, str(run.id),
        ]
        now: float = time.monotonic()
        guardian: tuple[int, int] = _limited_budgets(self._guardian_cutoffs, now, deadline, retirement_deadline)
        # The parent must be able to reserve time for both cooperative exit and forced recovery.
        watchdog: tuple[int, int] = _limited_budgets(
            self._watchdog_cutoffs, now, deadline, retirement_deadline, minimum_grace_ms=2)
        arguments.extend(str(value) for value in (*guardian, *watchdog))
        self._attempted = True
        self._process = Popen(arguments, stdin=PIPE, stdout=PIPE, stderr=PIPE, bufsize=0)
        stdin: BinaryIO | None = self._process.stdin
        stdout: BinaryIO | None = self._process.stdout
        status: BinaryIO | None = self._process.stderr
        require(stdin is not None and stdout is not None and status is not None, "missing native access pipes")
        assert stdin is not None and stdout is not None and status is not None
        self._owner = _AccessProcess(stdin, stdout, status, self._process, run, self._probe,
                                     self._role, nonce, inputs.watchdog_pin.sha256,
                                     self._diagnostic_path, self._diagnostics, retirement_deadline)
        _remaining(deadline)
        return self._owner

    def request_retirement(self, retirement_deadline: float) -> None:
        """Signal by closing input, keeping both original output streams for settlement."""
        require(self._retirement_deadline == retirement_deadline, "changed native access retirement cutoff")
        if self._process is not None and self._process.stdin is not None:
            self._process.stdin.close()

    def wait_retired(self, retirement_deadline: float) -> None:
        """For launch rollback, require settlement independently of access success."""
        require(self._retirement_deadline == retirement_deadline, "changed native access retirement cutoff")
        if self._attempted:
            owner: _AccessProcess | None = self._owner
            require(owner is not None, "native access launch custody unconfirmed")
            assert owner is not None
            owner.wait_access_settled(retirement_deadline)

    def private_diagnostic(self) -> PrivateAccessDiagnostic | None:
        """Expose the fixed role's original private failure after actual settlement."""
        owner: _AccessProcess | None = self._owner
        require(owner is not None, "native access launch custody unconfirmed")
        assert owner is not None
        return owner.private_diagnostic()
