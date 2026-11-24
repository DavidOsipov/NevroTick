# SPDX-License-Identifier: GPL-2.0-only
"""Exercise the deployment session over anonymous POSIX pipes with inert receipts."""

from base64 import b64encode
from collections.abc import Callable
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
from types import SimpleNamespace, TracebackType
from typing import BinaryIO, Self
import unittest
from unittest.mock import patch
from uuid import UUID

import prestart_deployment_session as session
from prestart_deployment_session import (
    GuardedDeploymentProcess, launch_deployment_watchdog, supervise_deployment_session, supervise_prestart_session,
)
from prestart_receipts import (
    Capture, DeploymentObservation, DeploymentRun, Member, PrestartObservation, ReceiptRefused, Run,
)
import test_prestart_receipts as receipts


PipeScript = tuple[tuple[bytes, bytes], ...]


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
        for status in (0, 23):
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
                  work_deadline: float | None = None, retirement_deadline: float | None = None) -> PrestartObservation:
        now: float = monotonic()
        if work_deadline is None:
            work_deadline = now + 3.0
        if retirement_deadline is None:
            retirement_deadline = now + 4.0
        try:
            return supervise_prestart_session(
                self.bind(provisioning, standard), owner, provisioning, standard,
                work_deadline, retirement_deadline,
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
        for label, response, reason in (
            ("crossed selector", crossed, r"libtest"),
            ("crossed role frame", self.standard, r"unexpected deployment frame"),
            ("foreign run", foreign, r"foreign"),
            ("trailing output", self.access + b"X", r"trailing native access output"),
            ("native refusal", self.deployment.reply("refused", code="access-capture-refused", cleanup=[]),
             r"native access refused access-capture-refused"),
        ):
            with self.subTest(label=label):
                with (
                    ScriptedProcess(self.script[:6], self.deployment.retired) as owner,
                    self.actor(b"A", response) as provisioning,
                    self.actor(b"") as standard,
                ):
                    with self.assertRaisesRegex(ReceiptRefused, reason):
                        self.supervise(owner, provisioning, standard)
                    self.deployment.assert_requests(owner, b"SCICOA")
                    self.assertEqual(provisioning.requests, [b"A", b""])
                    self.assertEqual(standard.requests, [b""])

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
                    wait_retired: Callable[[float], None] = standard.wait_retired

                    def finish_at_cutoff(deadline: float) -> None:
                        nonlocal now
                        wait_retired(deadline)
                        now = work_deadline + delta

                    with (
                        patch.object(session, "time", SimpleNamespace(monotonic=lambda: now)),
                        patch.object(standard, "wait_retired", side_effect=finish_at_cutoff),
                    ):
                        if delta < 0:
                            self.assertEqual(self.supervise(
                                owner, provisioning, standard, work_deadline=work_deadline,
                                retirement_deadline=retirement_deadline,
                            ).run_id, self.fixture.run.id)
                        else:
                            with self.assertRaises(TimeoutError):
                                self.supervise(
                                    owner, provisioning, standard, work_deadline=work_deadline,
                                    retirement_deadline=retirement_deadline,
                                )
                    self.deployment.assert_requests(owner, b"SCICOAUF" if delta < 0 else b"SCICOAU")
                    self.assertEqual(provisioning.requests, [b"A", b""])
                    self.assertEqual(standard.requests, [b"U", b""])

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
            with self.assertRaises(ExceptionGroup) as caught:
                self.supervise(owner, provisioning, standard)
            observation, retirement = caught.exception.exceptions
            self.assertIsInstance(observation, ReceiptRefused)
            self.assertEqual(str(observation), "trailing native access output")
            self.assertIs(retirement, refusal)
            self.deployment.assert_requests(owner, b"SCICOA")
            self.assertEqual(provisioning.requests, [b"A", b""])
            self.assertEqual(standard.requests, [b""])

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

    def launch(self, executable: Path, pin: Member, diagnostics: BinaryIO) -> Popen[bytes]:
        return launch_deployment_watchdog(
            self.run, executable, pin,
            r"C:\Staging & Data\prestart_deployment_guardian.ps1", Member(101, "a" * 64), Member(202, "b" * 64),
            r"C:\Staging & Data\prestart_deployment_owner.ps1", Member(303, "c" * 64),
            (1000, 2000), (3000, 4000), diagnostics,
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


if __name__ == "__main__":
    unittest.main()
