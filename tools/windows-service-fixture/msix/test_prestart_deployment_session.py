# SPDX-License-Identifier: GPL-2.0-only
"""Exercise the deployment session over anonymous POSIX pipes with inert receipts."""

from base64 import b64encode
from collections.abc import Callable
from contextlib import ExitStack
from dataclasses import replace
from hashlib import sha256
import json
import os
from pathlib import Path
import select
from subprocess import DEVNULL, PIPE, Popen, TimeoutExpired
import sys
from tempfile import TemporaryDirectory, TemporaryFile
from threading import Event, Thread
from time import monotonic
from traceback import format_exception
from types import SimpleNamespace, TracebackType
from typing import BinaryIO, Generic, Self, TypeVar
import unittest
from unittest.mock import patch
from uuid import UUID

import prestart_deployment_session as session
from prestart_deployment_session import (
    GuardedDeploymentProcess, PrivateAccessDiagnostic, launch_deployment_watchdog, read_private_access_diagnostic,
    supervise_deployment_session, supervise_prestart_session,
)
from prestart_receipts import (
    Capture, DeploymentObservation, DeploymentRun, Member, PrestartObservation, PrestartProbe, ReceiptRefused, Run,
)
import test_prestart_receipts as receipts


PipeScript = tuple[tuple[bytes, bytes], ...]


class PrivateAccessDiagnosticTests(unittest.TestCase):
    failure: bytes = b"nevrotick-deployment-owner failure=session cause=none cleanup=actor-close\r\n"
    prefix: bytes = b"nevrotick-prestart-access-capture "

    def setUp(self) -> None:
        fixture: receipts.PrestartReceiptTests = receipts.PrestartReceiptTests()
        fixture.setUp()
        self.run: DeploymentRun = fixture.deployment_run()
        self.payload: dict[str, object] = {
            "run": str(self.run.id), "role": "access", "expected": self.run.observer_sha256,
            "pid": 413, "exit": 101, "before": self.run.observer_sha256, "after": self.run.observer_sha256,
            "stdout": "/wo=", "stderr": "ZXJyb3I=",
        }

    def line(self, **changes: object) -> bytes:
        return self.prefix + json.dumps(self.payload | changes, separators=(",", ":")).encode("ascii") + b"\n"

    def read(self, data: bytes, probe: PrestartProbe = PrestartProbe.ACCESS) -> PrivateAccessDiagnostic | None:
        original: BinaryIO
        with TemporaryFile() as original:
            original.write(data)
            original.flush()
            original.seek(2)
            result: PrivateAccessDiagnostic | None = read_private_access_diagnostic(self.run, probe, original)
            self.assertEqual(original.tell(), 2)
            return result

    def test_original_bytes_and_unconfirmed_input(self) -> None:
        result: PrivateAccessDiagnostic | None = self.read(self.failure + self.line() + b"watchdog exited 1\r\n")
        self.assertEqual(result, PrivateAccessDiagnostic(self.run.id, PrestartProbe.ACCESS, 413, 101,
                                                        self.run.observer_sha256, b"\xff\n", b"error"))
        self.assertNotIn("error", repr(result))
        self.assertEqual(self.read(self.failure + self.line(role="standard", before=None, after=None),
                                   PrestartProbe.STANDARD),
                         PrivateAccessDiagnostic(self.run.id, PrestartProbe.STANDARD, 413, 101,
                                                 None, b"\xff\n", b"error"))

    def test_no_record_and_stream_limits(self) -> None:
        raw: bytes
        for raw in (b"", self.failure, self.failure + b"nevrotick-prestart-access-capture-unavailable\r\n"):
            with self.subTest(size=len(raw)):
                self.assertIsNone(self.read(raw))
        stream: bytes = b"\x00" * 8192
        encoded: str = b64encode(stream).decode("ascii")
        result: PrivateAccessDiagnostic | None = self.read(self.failure + self.line(stdout=encoded, stderr=encoded))
        self.assertEqual(result, PrivateAccessDiagnostic(self.run.id, PrestartProbe.ACCESS, 413, 101,
                                                        self.run.observer_sha256, stream, stream))
        with self.assertRaisesRegex(ReceiptRefused, "native child output"):
            self.read(self.failure + self.line(stdout=b64encode(stream + b"\0").decode("ascii")))

    def test_crossed_or_malformed_record_refused(self) -> None:
        cases: tuple[tuple[dict[str, object], str], ...] = (
            ({"run": str(UUID(int=99))}, "foreign private access diagnostic"),
            ({"role": "standard"}, "foreign private access diagnostic"),
            ({"expected": "6" * 64}, "foreign private access diagnostic"),
            ({"before": None}, "invalid private access diagnostic inputs"),
            ({"before": "6" * 64, "after": "6" * 64}, "invalid private access diagnostic inputs"),
            ({"pid": 1 << 31}, "invalid private access diagnostic PID"),
            ({"pid": True}, "invalid private access diagnostic PID"),
            ({"exit": -(1 << 31) - 1}, "invalid private access diagnostic exit"),
            ({"exit": False}, "invalid private access diagnostic exit"),
            ({"stdout": "/wp="}, "noncanonical native child output"),
            ({"reaped": True}, "invalid private access diagnostic record"),
        )
        changes: dict[str, object]
        error: str
        for changes, error in cases:
            with self.subTest(field=next(iter(changes))), self.assertRaisesRegex(ReceiptRefused, error):
                self.read(self.failure + self.line(**changes))

    def test_partial_repeated_and_oversized_file_refused(self) -> None:
        line: bytes = self.line()
        cases: tuple[tuple[bytes, str], ...] = (
            (self.failure + line[:-1], "incomplete private access diagnostic file"),
            (line, "missing owner failure"),
            (b"nevrotick-deployment-owner failure=session\r\n" + line, "missing owner failure"),
            (self.failure + line + self.failure + line, "repeated private access diagnostic"),
            (self.failure + line + self.failure + b"nevrotick-prestart-access-capture-unavailable\n",
             "repeated private access diagnostic"),
            (self.failure + line[:-2] + b',"\\u0070id":413}\n', "invalid private access diagnostic record"),
            (self.failure + self.prefix + b" " * 24000 + b"\n", "oversized private access diagnostic record"),
            (b"x" * (32 * 1024) + b"\n", "invalid private access diagnostic file"),
        )
        raw: bytes
        error: str
        for raw, error in cases:
            with self.subTest(error=error), self.assertRaisesRegex(ReceiptRefused, error):
                self.read(raw)

    def test_requires_an_original_regular_file(self) -> None:
        read_end: int
        write_end: int
        pipe: BinaryIO
        original: BinaryIO
        read_end, write_end = os.pipe()
        try:
            with os.fdopen(read_end, "rb") as pipe, self.assertRaisesRegex(ReceiptRefused, "diagnostic file"):
                read_private_access_diagnostic(self.run, PrestartProbe.ACCESS, pipe)
        finally:
            os.close(write_end)
        with TemporaryFile() as original, self.assertRaisesRegex(ReceiptRefused, "diagnostic probe"):
            read_private_access_diagnostic(self.run, PrestartProbe.OBSERVATION, original)

    def test_refuses_growth_during_read(self) -> None:
        def read_then_grow(fd: int, size: int, offset: int) -> bytes:
            data: bytes = pread(fd, size, offset)
            os.pwrite(fd, b"late diagnostic\n", len(data))
            return data

        pread: Callable[[int, int, int], bytes] = os.pread
        with patch.object(session.os, "pread", read_then_grow):
            with self.assertRaisesRegex(ReceiptRefused, "diagnostic file changed"):
                self.read(self.failure + self.line())


def frame(payload: bytes) -> bytes:
    """Encode the independent wire envelope without the session implementation."""
    return len(payload).to_bytes(4, "little") + payload


class ScriptedProcess:
    """Stand in for the admitted native process and observe its actual pipe traffic."""

    def __init__(self, script: PipeScript, terminal: bytes, *,
                 wait_failure: OSError | None = None, keep_stdout_open: bool = False,
                 retirement_requests: tuple[bytes, ...] = (b"R", b""),
                 observe_request: Callable[[bytes], None] | None = None) -> None:
        requests, stdin = os.pipe()
        stdout, responses = os.pipe()
        self.stdin: BinaryIO = os.fdopen(stdin, "wb", buffering=0)
        self.stdout: BinaryIO = os.fdopen(stdout, "rb", buffering=0)
        self.requests: list[bytes] = []
        self.waits: list[tuple[float, bool, bool]] = []
        self.failure: BaseException | None = None
        self.wait_failure = wait_failure
        self.release = Event()
        self.retired_exit = Event()
        self.observe_request = observe_request
        self.deadline = monotonic() + 6.0
        self.producer = Thread(
            target=self._serve,
            args=(requests, responses, script, terminal, keep_stdout_open, retirement_requests), daemon=True,
        )
        self.producer.start()

    def __enter__(self) -> Self:
        return self

    def __exit__(self, _type: type[BaseException] | None, _value: BaseException | None,
                 _traceback: TracebackType | None) -> None:
        self.stdin.close()
        self.stdout.close()
        self.release.set()
        self.producer.join(timeout=max(0.0, self.deadline - monotonic()))
        if self.producer.is_alive():
            raise AssertionError("scripted producer did not settle")
        if self.failure is not None:
            raise AssertionError("scripted producer did not complete its pipe transcript") from self.failure

    def _request(self, requests: int, expected: bytes) -> None:
        ready, _, _ = select.select([requests], [], [], max(0.0, self.deadline - monotonic()))
        if not ready:
            raise AssertionError("scripted producer did not receive a request before its deadline")
        observed: bytes = os.read(requests, 1)
        self.requests.append(observed)
        if observed != expected:
            raise AssertionError(f"expected request {expected!r}, received {observed!r}")
        if self.observe_request is not None:
            self.observe_request(observed)

    def _reply(self, responses: int, reply: bytes) -> None:
        # Small non-blocking writes keep the fixture bounded even if the consumer stalls.
        remaining = memoryview(reply)
        while remaining:
            _, ready, _ = select.select([], [responses], [], max(0.0, self.deadline - monotonic()))
            if not ready:
                raise AssertionError("scripted producer could not deliver response before its deadline")
            try:
                written = os.write(responses, remaining[:4096])
            except BlockingIOError:
                continue
            remaining = remaining[written:]

    def _serve(self, requests: int, responses: int, script: PipeScript, terminal: bytes,
               keep_stdout_open: bool, retirement_requests: tuple[bytes, ...]) -> None:
        try:
            os.set_blocking(responses, False)
            for command, reply in script:
                self._request(requests, command)
                self._reply(responses, reply)
            for command in retirement_requests:
                self._request(requests, command)
            self._reply(responses, terminal)
            if keep_stdout_open and not self.release.wait(timeout=max(0.0, self.deadline - monotonic())):
                raise AssertionError("scripted terminal stream was left open")
        except BaseException as error:
            self.failure = error
        finally:
            os.close(requests)
            os.close(responses)

    def wait_retired(self, deadline: float) -> None:
        self.waits.append((deadline, self.stdin.closed, self.stdout.closed))
        if self.wait_failure is not None:
            raise self.wait_failure
        self.producer.join(timeout=max(0.0, deadline - monotonic()))
        if self.producer.is_alive():
            raise TimeoutError("scripted process exit was unconfirmed")
        self.retired_exit.set()


class DeploymentSessionTests(unittest.TestCase):
    def setUp(self) -> None:
        self.fixture = receipts.PrestartReceiptTests()
        self.fixture.setUp()
        self.run: DeploymentRun = self.fixture.deployment_run()
        self.script: PipeScript = (
            (b"S", self.reply("sample", engine=[528, 528], broker=[528, 16])),
            (b"C", self.reply("current")),
            (b"I", self.capture("identity", self.fixture.identity)),
            (b"C", self.reply("current")),
            (b"O", self.capture("observation", self.fixture.observation)),
            (b"F", self.reply("confirmed")),
        )
        self.retired: bytes = self.reply("retired", cleanup=[])

    def reply(self, kind: str, **fields: object) -> bytes:
        record: dict[str, object] = {"run": str(self.run.id), "kind": kind}
        return frame(json.dumps(record | fields, ensure_ascii=True).encode("ascii"))

    def capture(self, kind: str, capture: Capture, **override: object) -> bytes:
        fields: dict[str, object] = {
            "pid": capture.pid, "before": capture.input_sha256_before, "after": capture.input_sha256_after,
            "exit": capture.exit_code, "stdout": b64encode(capture.stdout).decode("ascii"),
            "stderr": b64encode(capture.stderr).decode("ascii"),
        }
        return self.reply(kind, **(fields | override))

    def supervise(self, owner: ScriptedProcess, *, observation_seconds: float = 3.0,
                  retirement_seconds: float = 4.0) -> DeploymentObservation:
        now: float = monotonic()
        retirement_deadline: float = now + retirement_seconds
        try:
            return supervise_deployment_session(self.run, owner, now + observation_seconds, retirement_deadline)
        finally:
            self.assertEqual(owner.waits, [(retirement_deadline, True, True)])

    def assert_requests(self, owner: ScriptedProcess, commands: bytes) -> None:
        self.assertEqual(owner.requests, [bytes([command]) for command in commands] + [b"R", b""])

    def test_publishes_only_after_ordered_capture_final_confirmation_and_retirement(self) -> None:
        with ScriptedProcess(self.script, self.retired) as owner:
            result: DeploymentObservation = self.supervise(owner)
            self.assertEqual(result, DeploymentObservation(self.run.id, "1" * 64, ((528, 528, 528), (528, 16, 16))))
            self.assert_requests(owner, b"SCICOF")
            self.assertFalse(owner.producer.is_alive())

    def test_complete_confirmation_must_still_be_inside_the_work_deadline(self) -> None:
        work_deadline: float = monotonic() + 3.0
        retirement_deadline: float = work_deadline + 1.0
        expect: Callable[..., None] = session._expect

        for delta in (-0.125, 0.0, 0.125):
            with self.subTest(seconds_from_cutoff=delta):
                now: float = work_deadline - 1.0

                def validate(row: dict[str, object], kind: str, *fields: str) -> None:
                    nonlocal now
                    expect(row, kind, *fields)
                    if kind == "confirmed":
                        # The complete original reply was read and checked before rescheduling.
                        now = work_deadline + delta

                with (
                    ScriptedProcess(self.script, self.retired) as owner,
                    patch.object(session, "time", SimpleNamespace(monotonic=lambda: now)),
                    patch.object(session, "_expect", validate),
                ):
                    if delta < 0.0:
                        actual: DeploymentObservation = supervise_deployment_session(
                            self.run, owner, work_deadline, retirement_deadline,
                        )
                        self.assertEqual(actual.run_id, self.run.id)
                    else:
                        with self.assertRaisesRegex(TimeoutError, "deployment pipe deadline"):
                            supervise_deployment_session(self.run, owner, work_deadline, retirement_deadline)
                    self.assert_requests(owner, b"SCICOF")
                    self.assertEqual(owner.waits, [(retirement_deadline, True, True)])

    def test_reads_the_inclusive_frame_size_limit(self) -> None:
        sample: bytes = self.script[0][1][4:]
        padded: bytes = frame(sample.ljust(24_000, b" "))
        with ScriptedProcess(((b"S", padded), *self.script[1:]), self.retired) as owner:
            self.assertEqual(self.supervise(owner).run_id, self.run.id)
            self.assert_requests(owner, b"SCICOF")

    def test_invalid_receipt_stops_before_the_next_native_command(self) -> None:
        wrong_identity: dict[str, object] = self.fixture.report | {"signed_package_sha256": "9" * 64}
        identity: str = b64encode(json.dumps(wrong_identity).encode("ascii")).decode("ascii")
        wrong_rows: list[str] = [*self.fixture.role_rows]
        wrong_rows[1] = wrong_rows[1].replace("before-type=16", "before-type=32")
        observation: str = b64encode(receipts.libtest(receipts.READ, wrong_rows)).decode("ascii")
        for index, reply, commands, reason in (
            (2, self.capture("identity", self.fixture.identity, stdout=identity), b"SCI", "installed identity differs"),
            (4, self.capture("observation", self.fixture.observation, stdout=observation), b"SCICO",
             "SCM types differ from the original window"),
        ):
            with self.subTest(commands=commands):
                script: PipeScript = (*self.script[:index], (commands[-1:], reply))
                with ScriptedProcess(script, self.retired) as owner:
                    with self.assertRaisesRegex(ReceiptRefused, reason):
                        self.supervise(owner)
                    self.assert_requests(owner, commands)

    def test_rejects_malformed_or_foreign_sample_before_currentness_check(self) -> None:
        sample: bytes = self.script[0][1][4:]
        duplicate: bytes = b'{"kind":"sample",' + sample[1:]
        for label, response, reason in (
            ("empty", b"\x00" * 4, "frame length"),
            ("oversized", (24_001).to_bytes(4, "little"), "frame length"),
            ("invalid UTF-8", frame(b"\xff"), "frame JSON"),
            ("duplicate", frame(duplicate), "frame JSON"),
            ("foreign", self.reply("sample", run=str(UUID(int=124)), engine=[528, 528], broker=[528, 16]),
             "foreign deployment frame"),
            ("extra field", self.reply("sample", engine=[528, 528], broker=[528, 16], extra=1), "unexpected deployment"),
            ("boolean", self.reply("sample", engine=[528, False], broker=[528, 16]), "invalid native SCM sample"),
        ):
            with self.subTest(label=label):
                with ScriptedProcess(((b"S", response),), self.retired) as owner:
                    with self.assertRaisesRegex(ReceiptRefused, reason):
                        self.supervise(owner)
                    self.assert_requests(owner, b"S")

    def test_rejects_invalid_capture_encoding_before_sending_the_next_command(self) -> None:
        for output, reason in (("!", "invalid native child output"), ("Zh==", "noncanonical native child output")):
            with self.subTest(output=output):
                identity: bytes = self.capture("identity", self.fixture.identity, stdout=output)
                with ScriptedProcess((*self.script[:2], (b"I", identity)), self.retired) as owner:
                    with self.assertRaisesRegex(ReceiptRefused, reason):
                        self.supervise(owner)
                    self.assert_requests(owner, b"SCI")

    def test_preserves_final_native_refusal_and_cleanup_failure_in_order(self) -> None:
        script: PipeScript = (*self.script[:5], (b"F", self.reply("refused", code="window-withdrawn")))
        terminal: bytes = self.reply("retired", cleanup=["archive-close-failed", "scm-close-failed"])
        with ScriptedProcess(script, terminal) as owner:
            with self.assertRaises(ExceptionGroup) as caught:
                self.supervise(owner)
            self.assertEqual([type(error) for error in caught.exception.exceptions], [ReceiptRefused, ReceiptRefused])
            self.assertEqual([str(error) for error in caught.exception.exceptions], [
                "native deployment refused window-withdrawn",
                "native deployment cleanup refused archive-close-failed, scm-close-failed",
            ])
            self.assert_requests(owner, b"SCICOF")

    def test_replayed_current_reply_cannot_confirm_final_window(self) -> None:
        stale_current: bytes = self.script[3][1]
        script: PipeScript = (*self.script[:5], (b"F", stale_current))
        with ScriptedProcess(script, self.retired) as owner:
            with self.assertRaisesRegex(ReceiptRefused, "unexpected deployment frame"):
                self.supervise(owner)
            self.assert_requests(owner, b"SCICOF")

    def test_keeps_terminal_cleanup_codes_when_output_trails_the_frame(self) -> None:
        terminal: bytes = self.reply("retired", cleanup=["archive-close-failed", "scm-close-failed"]) + b"X"
        with ScriptedProcess(self.script, terminal) as owner:
            with self.assertRaises(ExceptionGroup) as caught:
                self.supervise(owner)
            self.assertEqual([type(error) for error in caught.exception.exceptions], [ReceiptRefused, ReceiptRefused])
            self.assertEqual([str(error) for error in caught.exception.exceptions], [
                "native deployment cleanup refused archive-close-failed, scm-close-failed",
                "trailing native deployment output",
            ])
            self.assert_requests(owner, b"SCICOF")

    def test_withholds_checked_receipts_on_invalid_terminal_or_trailing_output(self) -> None:
        for label, terminal, reason in (
            ("foreign", self.reply("retired", run=str(UUID(int=124)), cleanup=[]), "foreign deployment frame"),
            ("missing body", b"\x01\x00\x00\x00", "incomplete deployment frame"),
            ("trailing", self.retired + b"X", "trailing native deployment output"),
            ("over-budget cleanup", self.reply("retired", cleanup=["close-failed"] * 9), "invalid deployment cleanup list"),
            ("unsafe code", self.reply("retired", cleanup=["private path"]), "invalid native deployment refusal code"),
        ):
            with self.subTest(label=label):
                with ScriptedProcess(self.script, terminal) as owner:
                    with self.assertRaisesRegex(ReceiptRefused, reason) as caught:
                        self.supervise(owner)
                    self.assertNotIn("private path", str(caught.exception))
                    self.assert_requests(owner, b"SCICOF")

    def test_owner_exit_failure_blocks_return_even_after_terminal_and_eof(self) -> None:
        error = OSError("owned job settlement unconfirmed")
        with ScriptedProcess(self.script, self.retired, wait_failure=error) as owner:
            with self.assertRaises(OSError) as caught:
                self.supervise(owner)
            self.assertIs(caught.exception, error)
            self.assert_requests(owner, b"SCICOF")

    def test_timed_out_observation_still_uses_separate_retirement_deadline(self) -> None:
        with ScriptedProcess(((b"S", b""),), self.retired) as owner:
            with self.assertRaisesRegex(TimeoutError, "deployment response pipe deadline"):
                self.supervise(owner, observation_seconds=0.25)
            self.assert_requests(owner, b"S")

    def test_timed_out_final_confirmation_drains_at_most_one_late_reply(self) -> None:
        confirmed: bytes = self.script[5][1]
        retired: bytes = self.reply("retired", cleanup=["scm-close-failed"])
        for prefix_size, repeated in ((0, False), (2, False), (9, False), (0, True)):
            with self.subTest(prefix_size=prefix_size, repeated=repeated):
                script: PipeScript = (*self.script[:5], (b"F", confirmed[:prefix_size]))
                # The producer sends the remaining reply and retirement only after R and stdin EOF.
                response: bytes = confirmed[prefix_size:] + (confirmed if repeated else b"") + retired
                with ScriptedProcess(script, response) as owner:
                    with self.assertRaises(ExceptionGroup) as caught:
                        self.supervise(owner, observation_seconds=0.75)
                    timeout, retirement = caught.exception.exceptions
                    self.assertIsInstance(timeout, TimeoutError)
                    self.assertRegex(str(timeout), r"deployment (?:response )?pipe deadline")
                    self.assertIsInstance(retirement, ReceiptRefused)
                    reason: str = ("unexpected deployment frame" if repeated
                                   else "native deployment cleanup refused scm-close-failed")
                    self.assertEqual(str(retirement), reason)
                    self.assert_requests(owner, b"SCICOF")

    def test_valid_terminal_without_eof_cannot_publish_and_owner_stays_responsible(self) -> None:
        error = OSError("owned process is still running")
        with ScriptedProcess(self.script, self.retired, wait_failure=error, keep_stdout_open=True) as owner:
            with self.assertRaises(ExceptionGroup) as caught:
                self.supervise(owner, observation_seconds=0.5, retirement_seconds=0.75)
            timeout, wait_failure = caught.exception.exceptions
            self.assertIsInstance(timeout, TimeoutError)
            self.assertRegex(str(timeout), "deployment response pipe deadline")
            self.assertIs(wait_failure, error)
            self.assertTrue(owner.producer.is_alive())
            self.assert_requests(owner, b"SCICOF")


    def test_guardian_wait_uses_actual_exit_after_pipe_custody(self) -> None:
        for status in (0, 3, 23):
            with self.subTest(status=status):
                args: list[str] = [sys.executable, "-I", "-B", "-c", f"raise SystemExit({status})"]
                deadline: float = monotonic() + 4
                with Popen(args, stdin=PIPE, stdout=PIPE, stderr=DEVNULL, bufsize=0) as process:
                    guardian = GuardedDeploymentProcess(process)
                    with self.assertRaisesRegex(ReceiptRefused, "pipes still owned"):
                        guardian.wait_retired(deadline)
                    guardian.stdin.close()
                    guardian.stdout.close()
                    if status == 0:
                        self.assertIsNone(guardian.wait_retired(deadline))
                    else:
                        with self.assertRaisesRegex(ReceiptRefused, "guardian refused retirement"):
                            guardian.wait_retired(deadline)
                    self.assertEqual(process.returncode, status)


class FullPrestartSessionTests(unittest.TestCase):
    def setUp(self) -> None:
        self.deployment = DeploymentSessionTests()
        self.deployment.setUp()
        self.fixture = self.deployment.fixture
        self.script: PipeScript = (
            *self.deployment.script[:5],
            (b"A", self.deployment.reply("access-current")),
            (b"U", self.deployment.reply("standard-current")),
            self.deployment.script[5],
        )
        self.access: bytes = self.deployment.capture("access", self.fixture.access)
        self.standard: bytes = self.deployment.capture("standard", self.fixture.standard)

    def bind(self, provisioning: ScriptedProcess, standard: ScriptedProcess) -> Run:
        return replace(self.fixture.run, provisioning_actor=provisioning, standard_actor=standard)

    def supervise(self, owner: ScriptedProcess, provisioning: ScriptedProcess, standard: ScriptedProcess, *,
                  work_deadline: float | None = None, retirement_deadline: float | None = None,
                  captures: session.PrestartCaptures | None = None) -> PrestartObservation:
        now: float = monotonic()
        if work_deadline is None:
            work_deadline = now + 3.0
        if retirement_deadline is None:
            retirement_deadline = now + 4.0
        try:
            return supervise_prestart_session(
                self.bind(provisioning, standard), owner, provisioning, standard,
                work_deadline, retirement_deadline, captures=captures,
            )
        finally:
            self.assertEqual(owner.waits, [(retirement_deadline, True, True)])
            for process in (provisioning, standard):
                self.assertEqual(len(process.waits), 1)
                deadline, input_closed, output_closed = process.waits[0]
                self.assertIn(deadline, (work_deadline, retirement_deadline))
                self.assertTrue(input_closed and output_closed)

    def actor(self, command: bytes, response: bytes = b"", *,
              wait_failure: OSError | None = None) -> ScriptedProcess:
        requests: tuple[bytes, ...] = (command, b"") if command else (b"",)
        return ScriptedProcess((), response, retirement_requests=requests, wait_failure=wait_failure)

    def assert_captures(self, captures: session.PrestartCaptures, deployment: ScriptedProcess,
                        *access: tuple[PrestartProbe, ScriptedProcess, Capture]) -> None:
        expected: tuple[tuple[PrestartProbe, ScriptedProcess, Capture], ...] = (
            (PrestartProbe.IDENTITY, deployment, self.fixture.identity),
            (PrestartProbe.OBSERVATION, deployment, self.fixture.observation),
            *access,
        )
        records: tuple[session.CapturedProbe, ...] = captures.records
        self.assertIsInstance(records, tuple)
        self.assertEqual(len(records), len(expected))
        for record, (probe, owner, capture) in zip(records, expected, strict=True):
            self.assertIs(record.probe, probe)
            self.assertIs(record.owner, owner)
            self.assertIs(record.capture.actor, capture.actor)
            self.assertEqual(record.capture, capture)

    def test_joins_only_after_both_original_captures_and_all_retirements(self) -> None:
        with self.actor(b"A", self.access) as provisioning, self.actor(b"U", self.standard) as standard:
            def require_prior_retirement(command: bytes) -> None:
                if command == b"U":
                    self.assertTrue(provisioning.retired_exit.is_set())
                if command == b"F":
                    self.assertTrue(standard.retired_exit.is_set())

            with ScriptedProcess(
                self.script, self.deployment.retired, observe_request=require_prior_retirement,
            ) as owner:
                result: PrestartObservation = self.supervise(owner, provisioning, standard)
                self.assertEqual(result, PrestartObservation(
                    self.fixture.run.id, "1" * 64, ((528, 528, 528), (528, 16, 16)),
                ))
                self.deployment.assert_requests(owner, b"SCICOAUF")
                self.assertEqual(provisioning.requests, [b"A", b""])
                self.assertEqual(standard.requests, [b"U", b""])
                self.assertTrue(all(process.retired_exit.is_set() for process in (owner, provisioning, standard)))

    def test_refuses_crossed_role_owners_before_taking_custody(self) -> None:
        with self.actor(b"") as owner, self.actor(b"") as provisioning, self.actor(b"") as standard:
            now: float = monotonic()
            with self.assertRaisesRegex(ReceiptRefused, "foreign pre-start access owner"):
                supervise_prestart_session(
                    self.bind(provisioning, standard), owner, standard, provisioning, now + 3.0, now + 4.0,
                )
            for process in (owner, provisioning, standard):
                self.assertFalse(process.stdin.closed)
                self.assertFalse(process.stdout.closed)
                self.assertEqual(process.requests, [])
                self.assertEqual(process.waits, [])

    def test_refuses_a_shared_underlying_pipe_before_consuming_the_run(self) -> None:
        with self.actor(b"") as owner, self.actor(b"") as provisioning, self.actor(b"") as standard:
            with os.fdopen(os.dup(provisioning.stdin.fileno()), "wb", buffering=0) as alias:
                crossed: SimpleNamespace = SimpleNamespace(
                    stdin=alias, stdout=standard.stdout, wait_retired=standard.wait_retired,
                )
                run: Run = replace(self.fixture.run, provisioning_actor=provisioning, standard_actor=crossed)
                now: float = monotonic()
                with self.assertRaisesRegex(ReceiptRefused, "separate private pipes"):
                    supervise_prestart_session(run, owner, provisioning, crossed, now + 3.0, now + 4.0)
                self.assertFalse(alias.closed)
                for process in (owner, provisioning, standard):
                    self.assertFalse(process.stdin.closed)
                    self.assertFalse(process.stdout.closed)
                    self.assertEqual(process.requests, [])
                    self.assertEqual(process.waits, [])

    def test_refuses_a_deployment_actor_reused_for_an_access_role_before_custody(self) -> None:
        with self.actor(b"") as owner, self.actor(b"") as provisioning, self.actor(b"") as standard:
            now: float = monotonic()
            for actor in (provisioning, standard):
                run: Run = replace(self.bind(provisioning, standard), deployment_actor=actor)
                with self.assertRaisesRegex(ReceiptRefused, "actor contexts are incomplete or conflated"):
                    supervise_prestart_session(run, owner, provisioning, standard, now + 3.0, now + 4.0)
            for process in (owner, provisioning, standard):
                self.assertFalse(process.stdin.closed)
                self.assertFalse(process.stdout.closed)
                self.assertEqual(process.requests, [])
                self.assertEqual(process.waits, [])

    def test_deployment_currentness_refusal_sends_no_role_command(self) -> None:
        for guard, index in ((b"A", 5), (b"U", 6)):
            with self.subTest(guard=guard):
                script: PipeScript = (
                    *self.script[:index], (guard, self.deployment.reply("refused", code="window-withdrawn")),
                )
                command: bytes = b"A" if guard == b"U" else b""
                with (
                    ScriptedProcess(script, self.deployment.retired) as owner,
                    self.actor(command, self.access if command else b"") as provisioning,
                    self.actor(b"") as standard,
                ):
                    with self.assertRaisesRegex(ReceiptRefused, "native deployment refused window-withdrawn"):
                        self.supervise(owner, provisioning, standard)
                    self.deployment.assert_requests(owner, b"SCICOAU" if command else b"SCICOA")
                    self.assertEqual(provisioning.requests, [b"A", b""] if command else [b""])
                    self.assertEqual(standard.requests, [b""])

    def test_invalid_provisioning_frames_block_the_standard_probe_and_confirmation(self) -> None:
        foreign: bytes = self.deployment.capture("access", self.fixture.access, run=str(UUID(int=124)))
        crossed: bytes = self.deployment.capture("access", self.fixture.standard)
        invalid_base64: bytes = self.deployment.capture("access", self.fixture.access, stdout="?!")
        for label, response, reason, decoded in (
            ("crossed selector", crossed, r"libtest", self.fixture.standard),
            ("crossed role frame", self.standard, r"unexpected deployment frame", None),
            ("foreign run", foreign, r"foreign", None),
            ("malformed private JSON", frame(b'{"private-access-capture":'), r"invalid deployment frame JSON", None),
            ("malformed capture encoding", invalid_base64, r"invalid native child output", None),
            ("trailing output", self.access + b"X", r"trailing native access output", self.fixture.access),
            ("native refusal", self.deployment.reply("refused", code="access-capture-refused", cleanup=[]),
             r"native access refused access-capture-refused", None),
        ):
            with self.subTest(label=label):
                with (
                    ScriptedProcess(self.script[:6], self.deployment.retired) as owner,
                    self.actor(b"A", response) as provisioning,
                    self.actor(b"") as standard,
                ):
                    captures: session.PrestartCaptures = session.PrestartCaptures()
                    with self.assertRaisesRegex(ReceiptRefused, reason) as caught:
                        self.supervise(owner, provisioning, standard, captures=captures)
                    self.assertIsNone(caught.exception.__cause__)
                    self.assertIsNone(caught.exception.__context__)
                    self.deployment.assert_requests(owner, b"SCICOA")
                    self.assertEqual(provisioning.requests, [b"A", b""])
                    self.assertEqual(standard.requests, [b""])
                    if decoded is None:
                        self.assert_captures(captures, owner)
                    else:
                        self.assert_captures(captures, owner, (
                            PrestartProbe.ACCESS, provisioning, replace(decoded, actor=provisioning),
                        ))

    def test_invalid_standard_observation_blocks_final_confirmation(self) -> None:
        stdout: bytes = receipts.libtest(receipts.ORDINARY, [
            row.replace("configuration-denied code=5", "configuration-denied code=6")
            for row in self.fixture.standard_rows
        ])
        invalid: bytes = self.deployment.capture("standard", replace(self.fixture.standard, stdout=stdout))
        with (
            ScriptedProcess(self.script[:7], self.deployment.retired) as owner,
            self.actor(b"A", self.access) as provisioning,
            self.actor(b"U", invalid) as standard,
        ):
            with self.assertRaisesRegex(ReceiptRefused, "standard-user access or denial rows differ"):
                self.supervise(owner, provisioning, standard)
            self.deployment.assert_requests(owner, b"SCICOAU")
            self.assertEqual(provisioning.requests, [b"A", b""])
            self.assertEqual(standard.requests, [b"U", b""])

    def test_stale_window_acknowledgement_cannot_spend_an_access_owner(self) -> None:
        script: PipeScript = (*self.script[:5], (b"A", self.deployment.reply("current")))
        with (
            ScriptedProcess(script, self.deployment.retired) as owner,
            self.actor(b"") as provisioning,
            self.actor(b"") as standard,
        ):
            wait: Callable[[float], None] = owner.wait_retired

            def observe_after_fanout(deadline: float) -> None:
                for access in (provisioning, standard):
                    self.assertTrue(access.stdin.closed and access.stdout.closed)
                wait(deadline)

            with patch.object(owner, "wait_retired", side_effect=observe_after_fanout):
                with self.assertRaisesRegex(ReceiptRefused, "unexpected deployment frame"):
                    self.supervise(owner, provisioning, standard)
            self.deployment.assert_requests(owner, b"SCICOA")
            self.assertEqual(provisioning.requests, [b""])
            self.assertEqual(standard.requests, [b""])

    def test_identity_frame_decoded_at_work_cutoff_is_retained_without_acceptance(self) -> None:
        work_deadline: float = monotonic() + 3.0
        retirement_deadline: float = work_deadline + 1.0
        now: float = work_deadline - 1.0
        expect: Callable[..., None] = session._expect
        failed_identity: Capture = replace(self.fixture.identity, exit_code=19)

        def expire_after_identity_frame(row: dict[str, object], kind: str, *fields: str) -> None:
            nonlocal now
            expect(row, kind, *fields)
            if kind == "identity":
                now = work_deadline

        with (
            ScriptedProcess((
                *self.script[:2], (b"I", self.deployment.capture("identity", failed_identity)),
            ), self.deployment.retired) as owner,
            self.actor(b"") as provisioning,
            self.actor(b"") as standard,
            patch.object(session, "time", SimpleNamespace(monotonic=lambda: now)),
            patch.object(session, "_expect", expire_after_identity_frame),
        ):
            captures: session.PrestartCaptures = session.PrestartCaptures()
            with self.assertRaises(TimeoutError):
                self.supervise(owner, provisioning, standard, work_deadline=work_deadline,
                               retirement_deadline=retirement_deadline, captures=captures)
            records: tuple[session.CapturedProbe, ...] = captures.records
            self.assertEqual(records, (session.CapturedProbe(PrestartProbe.IDENTITY, owner, failed_identity),))
            self.deployment.assert_requests(owner, b"SCI")
            self.assertEqual(provisioning.requests, [b""])
            self.assertEqual(standard.requests, [b""])

    def test_complete_access_capture_expires_while_observing_its_retirement(self) -> None:
        work_deadline: float = monotonic() + 3.0
        retirement_deadline: float = work_deadline + 1.0
        for delta in (-0.125, 0.0, 0.125):
            with self.subTest(seconds_from_cutoff=delta):
                now: float = work_deadline - 1.0
                with (
                    ScriptedProcess(self.script if delta < 0 else self.script[:7], self.deployment.retired) as owner,
                    self.actor(b"A", self.access) as provisioning,
                    self.actor(b"U", self.standard) as standard,
                ):
                    captures: session.PrestartCaptures = session.PrestartCaptures()
                    wait_retired: Callable[[float], None] = standard.wait_retired
                    expected: tuple[tuple[PrestartProbe, ScriptedProcess, Capture], ...] = (
                        (PrestartProbe.ACCESS, provisioning, replace(self.fixture.access, actor=provisioning)),
                        (PrestartProbe.STANDARD, standard, replace(self.fixture.standard, actor=standard)),
                    )

                    def finish_at_cutoff(deadline: float) -> None:
                        nonlocal now
                        self.assert_captures(captures, owner, *expected)
                        wait_retired(deadline)
                        now = work_deadline + delta

                    with (
                        patch.object(session, "time", SimpleNamespace(monotonic=lambda: now)),
                        patch.object(standard, "wait_retired", side_effect=finish_at_cutoff),
                    ):
                        if delta < 0:
                            self.assertEqual(self.supervise(
                                owner, provisioning, standard, work_deadline=work_deadline,
                                retirement_deadline=retirement_deadline, captures=captures,
                            ).run_id, self.fixture.run.id)
                        else:
                            with self.assertRaises(TimeoutError):
                                self.supervise(
                                    owner, provisioning, standard, work_deadline=work_deadline,
                                    retirement_deadline=retirement_deadline, captures=captures,
                                )
                    self.deployment.assert_requests(owner, b"SCICOAUF" if delta < 0 else b"SCICOAU")
                    self.assertEqual(provisioning.requests, [b"A", b""])
                    self.assertEqual(standard.requests, [b"U", b""])
                    self.assert_captures(captures, owner, *expected)

    def test_unconfirmed_provisioning_retirement_blocks_the_next_probe(self) -> None:
        refusal = OSError("original provisioning process exit was unconfirmed")
        with (
            ScriptedProcess(self.script[:6], self.deployment.retired) as owner,
            self.actor(b"A", self.access, wait_failure=refusal) as provisioning,
            self.actor(b"") as standard,
        ):
            with self.assertRaises(OSError) as caught:
                self.supervise(owner, provisioning, standard)
            self.assertIs(caught.exception, refusal)
            self.deployment.assert_requests(owner, b"SCICOA")
            self.assertEqual(provisioning.requests, [b"A", b""])
            self.assertEqual(standard.requests, [b""])

    def test_work_exit_timeout_reobserves_the_original_role_after_all_retirement_signals(self) -> None:
        work_deadline: float = monotonic() + 3.0
        retirement_deadline: float = work_deadline + 1.0
        expiry: TimeoutError = TimeoutError("original provisioning completion unobserved by work cutoff")
        attempts: list[float] = []
        with (
            ScriptedProcess(self.script[:6], self.deployment.retired) as owner,
            self.actor(b"A", self.access) as provisioning,
            self.actor(b"") as standard,
        ):
            wait: Callable[[float], None] = provisioning.wait_retired

            def pause_on_first_wait(deadline: float) -> None:
                attempts.append(deadline)
                if deadline == work_deadline:
                    raise expiry
                self.assertTrue(owner.retired_exit.is_set())
                self.assertTrue(standard.stdin.closed and standard.stdout.closed)
                wait(deadline)

            with patch.object(provisioning, "wait_retired", side_effect=pause_on_first_wait):
                with self.assertRaises(TimeoutError) as caught:
                    supervise_prestart_session(
                        self.bind(provisioning, standard), owner, provisioning, standard,
                        work_deadline, retirement_deadline,
                    )
            self.assertIs(caught.exception, expiry)
            self.assertEqual(attempts, [work_deadline, retirement_deadline])
            self.deployment.assert_requests(owner, b"SCICOA")
            self.assertEqual(provisioning.requests, [b"A", b""])
            self.assertEqual(standard.requests, [b""])

    def test_retains_the_first_capture_refusal_and_unused_role_retirement_failure(self) -> None:
        refusal = OSError("original unused standard process exit was unconfirmed")
        with (
            ScriptedProcess(self.script[:6], self.deployment.retired) as owner,
            self.actor(b"A", self.access + b"X") as provisioning,
            self.actor(b"", wait_failure=refusal) as standard,
        ):
            captures: session.PrestartCaptures = session.PrestartCaptures()
            with self.assertRaises(ExceptionGroup) as caught:
                self.supervise(owner, provisioning, standard, captures=captures)
            observation, retirement = caught.exception.exceptions
            self.assertIsInstance(observation, ReceiptRefused)
            self.assertEqual(str(observation), "trailing native access output")
            self.assertIs(retirement, refusal)
            self.deployment.assert_requests(owner, b"SCICOA")
            self.assertEqual(provisioning.requests, [b"A", b""])
            self.assertEqual(standard.requests, [b""])
            self.assertTrue(owner.retired_exit.is_set() and provisioning.retired_exit.is_set())
            self.assertFalse(standard.retired_exit.is_set())
            self.assert_captures(captures, owner, (
                PrestartProbe.ACCESS, provisioning, replace(self.fixture.access, actor=provisioning),
            ))

    def test_preserves_the_access_failure_and_its_observed_child_cleanup(self) -> None:
        response: bytes = self.deployment.reply(
            "refused", code="access-failed", cleanup=["child-termination-failed", "actor-dispose-failed"],
        )
        with (
            ScriptedProcess(self.script[:6], self.deployment.retired) as owner,
            self.actor(b"A", response) as provisioning,
            self.actor(b"") as standard,
        ):
            with self.assertRaises(ExceptionGroup) as caught:
                self.supervise(owner, provisioning, standard)
            self.assertEqual([str(error) for error in caught.exception.exceptions], [
                "native access refused access-failed",
                "native access cleanup refused child-termination-failed, actor-dispose-failed",
            ])
            self.deployment.assert_requests(owner, b"SCICOA")
            self.assertEqual(standard.requests, [b""])


LaunchRunT = TypeVar("LaunchRunT", bound=DeploymentRun)
LaunchEvent = tuple[str, str, float]


class ScriptedOwnerAttempt(Generic[LaunchRunT]):
    """Retain the original process even when start cannot return it."""

    def __init__(self, role: str, events: list[LaunchEvent], process: ScriptedProcess | None = None, *,
                 result: session.OwnedDeploymentProcess | None = None,
                 on_start: Callable[[], None] | None = None) -> None:
        self.role: str = role
        self.events: list[LaunchEvent] = events
        self.process: ScriptedProcess | None = process
        self.result: session.OwnedDeploymentProcess | None = result
        self.on_start: Callable[[], None] | None = on_start
        self.starts: list[tuple[LaunchRunT, float, float]] = []

    def start(self, run: LaunchRunT, deadline: float, retirement_deadline: float) -> session.OwnedDeploymentProcess:
        self.events.append(("start", self.role, deadline))
        self.starts.append((run, deadline, retirement_deadline))
        if self.on_start is not None:
            self.on_start()
        if self.result is not None:
            return self.result
        if self.process is None:
            raise AssertionError("scripted attempt did not acquire an original process")
        return self.process

    def request_retirement(self, deadline: float) -> None:
        self.events.append(("request", self.role, deadline))
        if self.process is not None:
            self.process.stdin.close()
            self.process.stdout.close()

    def wait_retired(self, deadline: float) -> None:
        self.events.append(("wait", self.role, deadline))
        if self.process is not None:
            self.process.wait_retired(deadline)


class FullPrestartLaunchTests(unittest.TestCase):
    def setUp(self) -> None:
        self.full: FullPrestartSessionTests = FullPrestartSessionTests()
        self.full.setUp()
        self.run: DeploymentRun = self.full.deployment.run
        self.work_deadline: float = monotonic() + 3.0
        self.retirement_deadline: float = self.work_deadline + 1.0
        self.events: list[LaunchEvent] = []

    def launch(self, provisioning: ScriptedOwnerAttempt[DeploymentRun],
               standard: ScriptedOwnerAttempt[DeploymentRun],
               deployment: ScriptedOwnerAttempt[Run], *,
               captures: session.PrestartCaptures | None = None) -> PrestartObservation:
        if captures is None:
            captures = session.PrestartCaptures()
        return session.launch_prestart_session(
            self.run, provisioning, standard, deployment, self.work_deadline, self.retirement_deadline,
            captures=captures,
        )

    def assert_rollback(self, *roles: str) -> None:
        count: int = len(roles)
        self.assertEqual(self.events[:count], [("start", role, self.work_deadline) for role in roles])
        self.assertCountEqual(
            self.events[count:2 * count], [("request", role, self.retirement_deadline) for role in roles],
        )
        self.assertCountEqual(
            self.events[2 * count:], [("wait", role, self.retirement_deadline) for role in roles],
        )

    def assert_unused_retired(self, *processes: ScriptedProcess) -> None:
        for process in processes:
            self.assertEqual(process.requests, [b""])
            self.assertEqual(process.waits, [(self.retirement_deadline, True, True)])
            self.assertTrue(process.retired_exit.is_set())

    def test_launches_the_original_roles_then_supervises_their_pipes_once(self) -> None:
        captures: session.PrestartCaptures = session.PrestartCaptures()
        empty: tuple[session.CapturedProbe, ...] = captures.records
        with (
            self.full.actor(b"A", self.full.access) as access,
            self.full.actor(b"U", self.full.standard) as ordinary,
            ScriptedProcess(self.full.script, self.full.deployment.retired) as owner,
        ):
            provisioning: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt(
                "provisioning", self.events, access,
            )
            standard: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt("standard", self.events, ordinary)
            deployment: ScriptedOwnerAttempt[Run] = ScriptedOwnerAttempt("deployment", self.events, owner)

            result: PrestartObservation = self.launch(provisioning, standard, deployment, captures=captures)
            self.assertEqual(result, PrestartObservation(
                self.run.id, "1" * 64, ((528, 528, 528), (528, 16, 16)),
            ))
            self.assertEqual(self.events, [("start", role, self.work_deadline) for role in (
                "provisioning", "standard", "deployment",
            )])
            for attempt in (provisioning, standard):
                [(base, deadline, retirement)] = attempt.starts
                self.assertIs(base, self.run)
                self.assertEqual((deadline, retirement), (self.work_deadline, self.retirement_deadline))
            [(bound, deadline, retirement)] = deployment.starts
            self.assertEqual((deadline, retirement), (self.work_deadline, self.retirement_deadline))
            self.assertEqual(bound, self.full.bind(access, ordinary))
            self.assertIs(bound.deployment_actor, self.run.deployment_actor)
            self.assertIs(bound.provisioning_actor, access)
            self.assertIs(bound.standard_actor, ordinary)
            self.full.deployment.assert_requests(owner, b"SCICOAUF")
            self.assertEqual(access.requests, [b"A", b""])
            self.assertEqual(ordinary.requests, [b"U", b""])
            self.assertEqual(owner.waits, [(self.retirement_deadline, True, True)])
            for process in (access, ordinary):
                self.assertEqual(process.waits, [(self.work_deadline, True, True)])
            self.assertTrue(all(process.retired_exit.is_set() for process in (access, ordinary, owner)))
            self.full.assert_captures(
                captures, owner,
                (PrestartProbe.ACCESS, access, replace(self.full.fixture.access, actor=access)),
                (PrestartProbe.STANDARD, ordinary, replace(self.full.fixture.standard, actor=ordinary)),
            )
            self.assertEqual(empty, ())

    def test_launch_retains_raw_provisioning_failure_and_rejects_capture_store_reuse(self) -> None:
        marker: str = "synthetic-private-child-marker"
        stdout: bytes = (
            f"\r\nrunning 1 test\r\ntest {receipts.ACCESS} ... "
            f"{self.full.fixture.role_rows[0]}\r\n"
            f"{self.full.fixture.role_rows[1]}\r\n"
            "scm-prestart observed-rows=2\r\n"
            "scm-prestart-access invoking-pid=303\r\n"
            "FAILED\r\n"
        ).encode("ascii")
        stderr: bytes = (
            f'Error: "scm-prestart-access {receipts.ENGINE} mask=0x4 access-denied '
            f'stage=OpenService code=5 additional-manager-close=None"\r\n{marker}\r\n'
        ).encode("ascii")
        failed: Capture = replace(self.full.fixture.access, stdout=stdout, stderr=stderr, exit_code=101)
        reply: bytes = self.full.deployment.capture("access", failed)
        captures: session.PrestartCaptures = session.PrestartCaptures()
        with (
            self.full.actor(b"A", reply) as access,
            self.full.actor(b"") as ordinary,
            ScriptedProcess(self.full.script[:6], self.full.deployment.retired) as owner,
        ):
            provisioning: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt(
                "provisioning", self.events, access,
            )
            standard: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt("standard", self.events, ordinary)
            deployment: ScriptedOwnerAttempt[Run] = ScriptedOwnerAttempt("deployment", self.events, owner)
            with self.assertRaisesRegex(ReceiptRefused, "child did not exit successfully and reap") as caught:
                self.launch(provisioning, standard, deployment, captures=captures)
            self.full.assert_captures(captures, owner, (PrestartProbe.ACCESS, access, replace(failed, actor=access)))
            self.assertNotIn(marker, "".join(format_exception(caught.exception)))
            self.full.deployment.assert_requests(owner, b"SCICOA")
            self.assertEqual(access.requests, [b"A", b""])
            self.assert_unused_retired(ordinary)
            self.assertTrue(all(process.retired_exit.is_set() for process in (access, ordinary, owner)))

        retained: tuple[session.CapturedProbe, ...] = captures.records
        with self.assertRaises(AttributeError):
            setattr(retained[-1], "probe", PrestartProbe.STANDARD)
        with self.assertRaises(AttributeError):
            setattr(retained[-1].capture, "stdout", b"")
        self.events.clear()
        self.run = replace(self.run, id=UUID(int=124))
        with self.assertRaises(ReceiptRefused):
            self.launch(
                ScriptedOwnerAttempt("provisioning", self.events),
                ScriptedOwnerAttempt("standard", self.events),
                ScriptedOwnerAttempt("deployment", self.events),
                captures=captures,
            )
        self.assertEqual(self.events, [])
        self.assertEqual(captures.records, retained)
        self.full.assert_captures(captures, owner, (PrestartProbe.ACCESS, access, replace(failed, actor=access)))

    def test_deployment_render_refusal_retires_both_unused_access_owners(self) -> None:
        refusal: ReceiptRefused = ReceiptRefused("full deployment entry failed independent pin admission")

        def reject_render() -> None:
            raise refusal

        with self.full.actor(b"") as access, self.full.actor(b"") as ordinary:
            provisioning: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt(
                "provisioning", self.events, access,
            )
            standard: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt("standard", self.events, ordinary)
            deployment: ScriptedOwnerAttempt[Run] = ScriptedOwnerAttempt("deployment", self.events, on_start=reject_render)
            with self.assertRaises(ReceiptRefused) as caught:
                self.launch(provisioning, standard, deployment)
            self.assertIs(caught.exception, refusal)
            self.assert_rollback("provisioning", "standard", "deployment")
            self.assert_unused_retired(access, ordinary)

    def test_interrupted_second_launch_and_unconfirmed_retirement_preserve_both_failures(self) -> None:
        interruption: KeyboardInterrupt = KeyboardInterrupt("interrupted after acquiring the original process")
        unconfirmed: TimeoutError = TimeoutError("the interrupted original did not confirm exit")
        with ExitStack() as stack:
            access: ScriptedProcess = stack.enter_context(self.full.actor(b""))

            def interrupt_after_activation() -> None:
                standard.process = stack.enter_context(self.full.actor(b"", wait_failure=unconfirmed))
                raise interruption

            provisioning: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt(
                "provisioning", self.events, access,
            )
            standard: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt(
                "standard", self.events, on_start=interrupt_after_activation,
            )
            deployment: ScriptedOwnerAttempt[Run] = ScriptedOwnerAttempt("deployment", self.events)
            with self.assertRaises(BaseExceptionGroup) as caught:
                self.launch(provisioning, standard, deployment)
            self.assertEqual(caught.exception.exceptions, (interruption, unconfirmed))
            self.assert_rollback("provisioning", "standard")
            self.assert_unused_retired(access)
            original: ScriptedProcess | None = standard.process
            assert original is not None
            self.assertEqual(original.waits, [(self.retirement_deadline, True, True)])
            self.assertFalse(original.retired_exit.is_set())

    def test_shared_attempt_is_refused_without_launching(self) -> None:
        shared: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt("shared", self.events)
        deployment: ScriptedOwnerAttempt[Run] = ScriptedOwnerAttempt("deployment", self.events)
        with self.assertRaises(ReceiptRefused):
            self.launch(shared, shared, deployment)
        self.assertEqual(self.events, [])

    def test_unavailable_retirement_observer_does_not_prevent_other_signals_or_waits(self) -> None:
        primary: OSError = OSError("deployment preparation failed")
        unavailable: OSError = OSError("original retirement observer unavailable")

        class UnavailableWaitAttempt(ScriptedOwnerAttempt[DeploymentRun]):
            @property
            def wait_retired(self) -> Callable[[float], None]:
                raise unavailable

        def fail_preparation() -> None:
            raise primary

        with self.full.actor(b"") as access, self.full.actor(b"") as ordinary:
            provisioning: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt("provisioning", self.events, access)
            standard: UnavailableWaitAttempt = UnavailableWaitAttempt("standard", self.events, ordinary)
            deployment: ScriptedOwnerAttempt[Run] = ScriptedOwnerAttempt(
                "deployment", self.events, on_start=fail_preparation,
            )
            with self.assertRaises(ExceptionGroup) as caught:
                self.launch(provisioning, standard, deployment)
            self.assertEqual(caught.exception.exceptions, (primary, unavailable))
            self.assertEqual(self.events[3:6], [
                ("request", role, self.retirement_deadline) for role in ("provisioning", "standard", "deployment")
            ])
            self.assertEqual(self.events[6:], [
                ("wait", role, self.retirement_deadline) for role in ("provisioning", "deployment")
            ])
            self.assert_unused_retired(access)
            self.assertTrue(ordinary.stdin.closed and ordinary.stdout.closed)
            self.assertEqual(ordinary.waits, [])

    def test_crossed_pipe_and_shared_process_are_refused_before_any_protocol_request(self) -> None:
        fault: str
        for fault in ("aliased-pipe", "shared-process", "reversed-directions"):
            self.events.clear()
            with (
                self.subTest(fault=fault),
                self.full.actor(b"") as access,
                self.full.actor(b"") as ordinary,
                self.full.actor(b"") as owner,
                os.fdopen(os.dup(access.stdin.fileno()), "wb", buffering=0) as alias,
            ):
                crossed: session.OwnedDeploymentProcess
                if fault == "shared-process":
                    crossed = access
                elif fault == "reversed-directions":
                    crossed = SimpleNamespace(stdin=owner.stdout, stdout=owner.stdin, wait_retired=owner.wait_retired)
                else:
                    crossed = SimpleNamespace(stdin=alias, stdout=owner.stdout, wait_retired=owner.wait_retired)
                provisioning: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt(
                    "provisioning", self.events, access,
                )
                standard: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt("standard", self.events, ordinary)
                deployment: ScriptedOwnerAttempt[Run] = ScriptedOwnerAttempt(
                    "deployment", self.events, owner, result=crossed,
                )
                # The independently retained deployment provider closes its own alias on rollback.
                request_retirement: Callable[[float], None] = deployment.request_retirement

                def retire_alias(deadline: float) -> None:
                    alias.close()
                    request_retirement(deadline)

                with patch.object(deployment, "request_retirement", side_effect=retire_alias):
                    with self.assertRaises(ReceiptRefused):
                        self.launch(provisioning, standard, deployment)
                self.assert_rollback("provisioning", "standard", "deployment")
                self.assert_unused_retired(access, ordinary, owner)

    def test_elapsed_work_deadline_prevents_the_first_launch(self) -> None:
        provisioning: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt("provisioning", self.events)
        standard: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt("standard", self.events)
        deployment: ScriptedOwnerAttempt[Run] = ScriptedOwnerAttempt("deployment", self.events)
        with patch.object(session, "time", SimpleNamespace(monotonic=lambda: self.work_deadline)):
            with self.assertRaises(TimeoutError):
                self.launch(provisioning, standard, deployment)
        self.assertEqual(self.events, [])

    def test_work_deadline_reached_during_launch_retires_original_without_starting_another(self) -> None:
        now: float = self.work_deadline - 1.0

        def reach_deadline() -> None:
            nonlocal now
            now = self.work_deadline

        with self.full.actor(b"") as access:
            provisioning: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt(
                "provisioning", self.events, access, on_start=reach_deadline,
            )
            standard: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt("standard", self.events)
            deployment: ScriptedOwnerAttempt[Run] = ScriptedOwnerAttempt("deployment", self.events)
            with patch.object(session, "time", SimpleNamespace(monotonic=lambda: now)):
                with self.assertRaises(TimeoutError):
                    self.launch(provisioning, standard, deployment)
            self.assert_rollback("provisioning")
            self.assert_unused_retired(access)

    def test_work_cutoff_during_pipe_admission_settles_all_roles_without_protocol_traffic(self) -> None:
        now: float = self.work_deadline - 1.0
        check_pipes: Callable[[session._PrestartPipe], None] = session._PrestartPipe.require_separate_pipes

        def reach_deadline(window: session._PrestartPipe) -> None:
            nonlocal now
            check_pipes(window)
            now = self.work_deadline

        with self.full.actor(b"") as access, self.full.actor(b"") as ordinary, self.full.actor(b"") as owner:
            provisioning: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt("provisioning", self.events, access)
            standard: ScriptedOwnerAttempt[DeploymentRun] = ScriptedOwnerAttempt("standard", self.events, ordinary)
            deployment: ScriptedOwnerAttempt[Run] = ScriptedOwnerAttempt("deployment", self.events, owner)
            with (patch.object(session, "time", SimpleNamespace(monotonic=lambda: now)),
                  patch.object(session._PrestartPipe, "require_separate_pipes", reach_deadline)):
                with self.assertRaises(TimeoutError):
                    self.launch(provisioning, standard, deployment)
            self.assert_rollback("provisioning", "standard", "deployment")
            self.assert_unused_retired(access, ordinary, owner)


class DeploymentWatchdogLaunchTests(unittest.TestCase):
    def setUp(self) -> None:
        fixture = receipts.PrestartReceiptTests()
        fixture.setUp()
        self.run: DeploymentRun = fixture.deployment_run()
        directory = TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.executable: Path = Path(directory.name) / "prestart_deployment_watchdog.exe"
        image: bytes = os.fsencode(f"#!{sys.executable} -I\n") + (
            b"import json, os, stat, sys\n"
            b"data = sys.stdin.buffer.read()\n"
            b"pipes = [stat.S_ISFIFO(os.fstat(fd).st_mode) for fd in (0, 1, 2)]\n"
            b"print(json.dumps([sys.argv[1:], data.hex(), pipes]))\n"
            b"sys.stderr.buffer.write(b'watchdog diagnostic\\n')\n"
        )
        self.executable.write_bytes(image)
        self.executable.chmod(0o700)
        self.pin = Member(len(image), sha256(image).hexdigest())

    def launch(self, executable: Path, pin: Member, diagnostics: BinaryIO,
               *, observe_access_settlement: bool = False) -> Popen[bytes]:
        return launch_deployment_watchdog(
            self.run, executable, pin,
            r"C:\Staging & Data\prestart_deployment_guardian.ps1", Member(101, "a" * 64), Member(202, "b" * 64),
            r"C:\Staging & Data\prestart_deployment_owner.ps1", Member(303, "c" * 64),
            (1000, 2000), (3000, 4000), diagnostics,
            observe_access_settlement=observe_access_settlement,
        )

    def test_wrong_watchdog_digest_is_refused_before_process_creation(self) -> None:
        with TemporaryFile() as diagnostics, patch("prestart_deployment_session.Popen") as popen:
            with self.assertRaisesRegex(ReceiptRefused, "watchdog image differs"):
                self.launch(self.executable, Member(self.pin.size, "0" * 64), diagnostics)
            popen.assert_not_called()

    def test_nonregular_watchdog_path_is_refused_before_process_creation(self) -> None:
        directory: Path = self.executable.parent / "directory" / self.executable.name
        directory.mkdir(parents=True)
        symlink: Path = self.executable.parent / "symlink" / self.executable.name
        symlink.parent.mkdir()
        symlink.symlink_to(self.executable)
        for path in (directory, symlink):
            with (
                self.subTest(shape=path.parent.name),
                TemporaryFile() as diagnostics,
                patch("prestart_deployment_session.Popen") as popen,
            ):
                with self.assertRaisesRegex(ReceiptRefused, "watchdog read refused"):
                    self.launch(path, self.pin, diagnostics)
                popen.assert_not_called()

    def test_exact_watchdog_pin_starts_process_with_fixed_arguments_and_private_pipes(self) -> None:
        with TemporaryFile() as diagnostics, self.launch(self.executable, self.pin, diagnostics) as process:
            try:
                stdout, stderr = process.communicate(b"\x00R\xff", timeout=4)
            except TimeoutExpired:
                process.kill()
                process.communicate(timeout=4)
                raise
            self.assertEqual(process.returncode, 0)
            self.assertIsNone(stderr)
            self.assertEqual(json.loads(stdout), [
                [r"C:\Staging & Data\prestart_deployment_guardian.ps1", "101", "a" * 64, "202", "b" * 64,
                 r"C:\Staging & Data\prestart_deployment_owner.ps1", "303", "c" * 64,
                 "00000000-0000-0000-0000-00000000007b", "1000", "2000", "3000", "4000"],
                "0052ff", [True, True, False],
            ])
            diagnostics.seek(0)
            self.assertEqual(diagnostics.read(), b"watchdog diagnostic\n")

    def test_access_settlement_opt_in_reaches_the_original_watchdog(self) -> None:
        diagnostics: BinaryIO
        with TemporaryFile() as diagnostics, self.launch(self.executable, self.pin, diagnostics,
                                                         observe_access_settlement=True) as process:
            stdout: bytes
            stderr: None
            try:
                stdout, stderr = process.communicate(b"A", timeout=4)
            except TimeoutExpired:
                process.kill()
                process.communicate(timeout=4)
                raise
            arguments: list[str]
            request: str
            pipes: list[bool]
            arguments, request, pipes = json.loads(stdout)
            self.assertEqual(process.returncode, 0)
            self.assertIsNone(stderr)
            self.assertEqual(len(arguments), 14)
            self.assertEqual(arguments[-1], "--observe-access-settlement")
            self.assertEqual((request, pipes), ("41", [True, True, False]))
            diagnostics.seek(0)
            self.assertEqual(diagnostics.read(), b"watchdog diagnostic\n")

    def test_access_settlement_requires_a_fresh_read_write_diagnostic_file(self) -> None:
        diagnostics: BinaryIO
        with TemporaryFile() as diagnostics, patch("prestart_deployment_session.Popen") as popen:
            diagnostics.seek(1)
            with self.assertRaisesRegex(ReceiptRefused, "diagnostic file at offset zero"):
                self.launch(self.executable, self.pin, diagnostics, observe_access_settlement=True)
            diagnostics.seek(0)
            diagnostics.write(b"earlier output\n")
            diagnostics.seek(0)
            with self.assertRaisesRegex(ReceiptRefused, "diagnostic file at offset zero"):
                self.launch(self.executable, self.pin, diagnostics, observe_access_settlement=True)
            popen.assert_not_called()
        with TemporaryFile(mode="wb") as diagnostics, patch("prestart_deployment_session.Popen") as popen:
            with self.assertRaisesRegex(ReceiptRefused, "diagnostic file at offset zero"):
                self.launch(self.executable, self.pin, diagnostics, observe_access_settlement=True)
            popen.assert_not_called()


if __name__ == "__main__":
    unittest.main()
