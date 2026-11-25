# SPDX-License-Identifier: GPL-2.0-only
"""Host pipe endpoint for an admitted full-mode Sandbox deployment relay."""

from collections.abc import Callable
from contextlib import ExitStack
from hashlib import sha256
import json
import os
from pathlib import Path
import select
import stat
from threading import Event, Thread
from time import monotonic
from typing import BinaryIO

from check_package import read_regular, unique_object
from prestart_deployment_session import MAX_FRAME, _combined_failure, _remaining, _require_deadlines
from prestart_receipts import ReceiptRefused, Run, require, require_run


ROSTER: bytes = b"SCICOAUFR"
_FRAME_EXTENT: int = MAX_FRAME + 4
_TERMINAL_FIELDS: frozenset[str] = frozenset({
    "run", "commands", "reply_count", "launch_attempted", "transport_ok", "retired",
    "watchdog_pid", "watchdog_exit", "stdout_eof", "stderr_eof", "stdout_sha256", "stderr_sha256",
})


class MappedDeploymentProcess:
    """Bridge one run over protected input/output mappings with private POSIX pipes.

    The transaction owns the admitted guest and the mappings through retirement.
    Its confirmation callback must independently verify the original native
    relay process and job and return the original watchdog PID and diagnostic
    bytes. A local pipe closing or guest record alone does not permit cleanup.
    """

    def __init__(self, run: Run, records_in: Path, records_out: Path, work: float, retirement: float,
                 confirm_native: Callable[[float], tuple[int, bytes]]) -> None:
        require_run(run)
        _require_deadlines(work, retirement)
        _remaining(work)
        for path in (records_in, records_out):
            require(path.is_absolute() and path.resolve(strict=True) == path and path.is_dir(),
                    "invalid deployment relay mapping")
        require(not records_in.samefile(records_out), "deployment relay mappings alias")
        with os.scandir(records_out) as entries:
            require(next(entries, None) is None, "deployment relay output mapping is occupied")
        self._run: Run = run
        self._in: Path = records_in
        self._out: Path = records_out
        self._work: float = work
        self._retirement: float = retirement
        self._confirm_native: Callable[[float], tuple[int, bytes]] = confirm_native
        self._commands = bytearray()
        self._forwarded = bytearray()
        self._terminal: dict[str, object] | None = None
        self._error: BaseException | None = None
        self._work_expired: TimeoutError | None = None
        with ExitStack() as caller:
            worker = ExitStack()
            ready = Event()
            admitted = False

            def drive() -> None:
                ready.wait(timeout=max(0.0, retirement - monotonic()))
                if admitted:
                    with worker:
                        self._bridge(request, reply)

            try:
                request, sender = os.pipe()
                worker.callback(os.close, request)
                try:
                    self.stdin: BinaryIO = os.fdopen(sender, "wb", buffering=0)
                except BaseException:
                    os.close(sender)
                    raise
                caller.enter_context(self.stdin)
                receiver, reply = os.pipe()
                worker.callback(os.close, reply)
                try:
                    self.stdout: BinaryIO = os.fdopen(receiver, "rb", buffering=0)
                except BaseException:
                    os.close(receiver)
                    raise
                caller.enter_context(self.stdout)
                self._thread = Thread(target=drive, daemon=True)
                self._thread.start()
                admitted = True
                caller.pop_all()
            finally:
                try:
                    if not admitted:
                        worker.close()
                finally:
                    ready.set()

    def _committed(self, name: str, extent: int) -> bytes | None:
        try:
            fd: int = os.open(self._out / (name + ".commit"), os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        except FileNotFoundError:
            return None
        with os.fdopen(fd, "rb") as marker:
            info: os.stat_result = os.fstat(marker.fileno())
            require(stat.S_ISREG(info.st_mode) and info.st_size == 0 and marker.read(1) == b"",
                    "invalid deployment relay commit")
        return read_regular(self._out / name, extent, "relay output")

    def _publish(self, command: bytes) -> bool:
        index: int = len(self._commands)
        require(index < len(ROSTER) and not self._commands.endswith(b"R")
                and (command == b"R" or command == ROSTER[index:index + 1]), "invalid deployment relay command")
        cutoff: float = self._retirement if command == b"R" else self._work
        if command == b"R":
            _remaining(cutoff)
        elif monotonic() >= cutoff:
            return False
        leaf: Path = self._in / f"request-{index + 1}.bin"
        with leaf.open("xb") as stream:
            stream.write(command)
            stream.flush()
            os.fsync(stream.fileno())
        if monotonic() >= cutoff:
            # This exclusively created file has no marker. The guest has
            # not accepted the slot, so retirement can still use it.
            leaf.unlink()
            if command == b"R":
                _remaining(cutoff)
            return False
        with leaf.with_suffix(".bin.commit").open("xb"):
            pass
        self._commands.extend(command)
        return True

    def _read_terminal(self) -> dict[str, object] | None:
        raw: bytes | None = self._committed("relay.json", 4096)
        if raw is None:
            return None
        row: object = json.loads(raw.decode("utf-8"), object_pairs_hook=unique_object)
        require(type(row) is dict and row.keys() == _TERMINAL_FIELDS and row["run"] == str(self._run.id)
                and type(row["reply_count"]) is int and 0 <= row["reply_count"] <= len(self._commands),
                "invalid deployment relay terminal record")
        return row

    def _bridge(self, request: int, reply: int) -> None:
        pending = bytearray()
        replies: int = 0
        input_closed: bool = False
        try:
            os.set_blocking(reply, False)
            while True:
                delay: float = min(0.01, _remaining(self._retirement))
                if not pending and replies < len(self._commands):
                    frame: bytes | None = self._committed(f"reply-{replies + 1}.bin", _FRAME_EXTENT)
                    if frame is not None:
                        require(len(frame) >= 5 and int.from_bytes(frame[:4], "little") == len(frame) - 4,
                                "invalid deployment relay frame extent")
                        pending.extend(frame)
                        self._forwarded.extend(frame)
                        replies += 1
                if self._terminal is None:
                    self._terminal = self._read_terminal()
                if self._terminal is not None:
                    require(replies <= self._terminal["reply_count"], "deployment relay frame count differs")
                    if input_closed and replies == self._terminal["reply_count"] and not pending:
                        return
                readable, writable, _ = select.select([] if input_closed else [request],
                                                      [reply] if pending else [], [], delay)
                if readable:
                    command: bytes = os.read(request, 1)
                    if command:
                        if not self._publish(command):
                            self._work_expired = TimeoutError("deployment relay work expired before commit")
                    else:
                        input_closed = True
                        require(self._commands.endswith(b"R"), "deployment relay input closed before retirement")
                if writable:
                    try:
                        sent: int = os.write(reply, pending)
                    except BlockingIOError:
                        continue
                    del pending[:sent]
        except BaseException as failure:
            self._error = failure

    def wait_retired(self, deadline: float) -> None:
        require(self.stdin.closed and self.stdout.closed, "deployment relay pipe custody")
        cutoff: float = min(deadline, self._retirement)
        observed: tuple[int, bytes] | None = None
        errors: list[BaseException] = []
        try:
            _remaining(cutoff)
            observed = self._confirm_native(cutoff)
        except BaseException as problem:
            errors.append(problem)
        self._thread.join(timeout=max(0.0, cutoff - monotonic()))
        if self._thread.is_alive():
            errors.append(ReceiptRefused("deployment relay writer retirement unconfirmed"))
        elif self._error is not None:
            errors.append(self._error)
        if errors and self._work_expired is not None:
            errors.append(self._work_expired)
        failure: BaseException | None = _combined_failure("deployment relay retirement failed", errors)
        if failure is not None:
            raise failure
        require(type(observed) is tuple and len(observed) == 2, "invalid native relay observation")
        assert observed is not None
        observed_pid, observed_diag = observed
        row: dict[str, object] | None = self._terminal
        require(row is not None, "deployment relay terminal record absent")
        assert row is not None
        wire: bytes | None = self._committed("watchdog.stdout.bin", len(ROSTER) * _FRAME_EXTENT)
        diag: bytes | None = self._committed("watchdog.stderr.bin", 32768)
        require(wire is not None and diag is not None, "deployment relay terminal streams absent")
        assert wire is not None and diag is not None
        require(wire == self._forwarded and self._commands.endswith(b"R")
                and row["commands"] == self._commands.decode("ascii") and row["reply_count"] == len(self._commands)
                and row["stdout_sha256"] == sha256(wire).hexdigest()
                and row["stderr_sha256"] == sha256(diag).hexdigest(), "deployment relay transcript differs")
        require(all(row[key] is True for key in ("launch_attempted", "stdout_eof", "stderr_eof", "transport_ok", "retired"))
                and type(row["watchdog_exit"]) is int and row["watchdog_exit"] == 0,
                "deployment relay native settlement unconfirmed")
        pid: object = row["watchdog_pid"]
        require(type(pid) is int and 0 < pid < 2**32 and type(observed_pid) is int and observed_pid == pid
                and type(observed_diag) is bytes and observed_diag == diag, "deployment relay native observation differs")
        _remaining(cutoff)
