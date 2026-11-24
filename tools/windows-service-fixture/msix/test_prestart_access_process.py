# SPDX-License-Identifier: GPL-2.0-only
"""Use real POSIX pipes to exercise the watchdog-parent consumer and fixed supervisor."""

from contextlib import ExitStack
from hashlib import sha256
import json
from pathlib import Path
from subprocess import Popen, TimeoutExpired
from tempfile import TemporaryDirectory
import time
from typing import BinaryIO, Callable
import unittest
from unittest.mock import patch
from uuid import UUID

import prestart_deployment_session as session
from prestart_access_process import (
    AccessWatchdogInputs, PrestartAccessAttempt, _limited_budgets, _remaining, _windows_path,
)
from prestart_receipts import DeploymentRun, Member, PrestartObservation, PrestartProbe, ReceiptRefused, Run
import test_prestart_deployment_session as fixtures


# Independent inert producer with the parent's fixed argv and transport. A, U and
# EOF are observed in the spawned process; its POSIX exit is independent of the
# native DWORD the fixture puts on its dedicated status pipe.
_PARENT: bytes = b"""#!/usr/bin/python3
from hashlib import sha256
from pathlib import Path
import json, sys
mode, parent, parent_size, parent_pin, watchdog, size, pin, role, nonce, diagnostic, guardian, gs, gh, ps, ph, owner, osz, oh, run, *budgets = sys.argv[1:]
assert mode == '--supervise-access-parent' and parent == sys.argv[0]
parent_bytes = Path(parent).read_bytes()
assert int(parent_size) == len(parent_bytes) and parent_pin == sha256(parent_bytes).hexdigest()
root = Path(diagnostic).parent
case = json.loads((root / 'case.json').read_bytes())
assert Path(watchdog).name == 'prestart_deployment_watchdog.exe'
assert Path(guardian).name == 'prestart_deployment_guardian.ps1'
assert Path(owner).name == 'prestart_deployment_owner.ps1'
assert 0 < int(budgets[0]) < int(budgets[1]) <= 4000
assert 0 < int(budgets[2]) < int(budgets[3]) <= 4000
assert int(budgets[3]) - int(budgets[2]) >= 4
command = sys.stdin.buffer.read(2)
assert command == case['command'].encode()
if command:
    response = (root / 'wire.bin').read_bytes()
    if response:
        sys.stdout.buffer.write(response)
sys.stdout.buffer.flush()
diagnostics = (root / 'diagnostics.bin').read_bytes()
Path(diagnostic).write_bytes(diagnostics)
if case['fault'] == 'different-diagnostics':
    diagnostics = diagnostics + b'X'
if case['fault'] == 'foreign-nonce':
    nonce = 'e' * 32 if nonce != 'e' * 32 else 'f' * 32
if case['fault'] == 'foreign-run':
    run = '00000000-0000-0000-0000-000000000001'
if case['fault'] == 'foreign-role':
    role = 'standard' if role == 'access' else 'access'
if case['fault'] == 'foreign-image':
    pin = '0' * 64
line = f"nevrotick-prestart-access-exit-v1 {run} {role} {nonce} {pin} 413 {case['native_exit']} {len(diagnostics)} {sha256(diagnostics).hexdigest()}".encode()
if case['fault'] == 'extra-record':
    line += b'\\n' + line
if case['fault'] != 'missing-record':
    sys.stderr.buffer.write(line + b'\\n')
sys.stderr.buffer.flush()
raise SystemExit(case['parent_exit'])
"""


class NativeAccessProcessTests(unittest.TestCase):
    def setUp(self) -> None:
        self.full: fixtures.FullPrestartSessionTests = fixtures.FullPrestartSessionTests()
        self.full.setUp()
        self.run: DeploymentRun = self.full.fixture.deployment_run()
        self.files: ExitStack = ExitStack()
        self.addCleanup(self.files.close)
        self.root: Path = Path(self.files.enter_context(TemporaryDirectory()))
        self.files.enter_context(patch("prestart_access_process._windows_path", side_effect=str))
        self.work: float = time.monotonic() + 4.0
        self.retire: float = self.work + 1.0

    def prepare(self, name: str, *, probe: PrestartProbe = PrestartProbe.ACCESS,
                command: str = "", response: bytes = b"", diagnostics: bytes = b"",
                native_exit: int = 0, parent_exit: int = 0, fault: str = "") -> tuple[PrestartAccessAttempt, Path, BinaryIO]:
        shelf: Path = self.root / name
        shelf.mkdir()
        parent: Path = shelf / "prestart_access_parent.exe"
        watchdog: Path = shelf / "prestart_deployment_watchdog.exe"
        guardian: Path = shelf / "prestart_deployment_guardian.ps1"
        entry: Path = shelf / "prestart_deployment_owner.ps1"
        parent.write_bytes(_PARENT)
        parent.chmod(0o700)
        watchdog.write_bytes(b"fixed watchdog bytes checked by the launch consumer")
        diag: Path = shelf / "prestart-access-diagnostics.log"
        diag.touch(mode=0o600)
        stream: BinaryIO = self.files.enter_context(diag.open("rb", buffering=0))
        (shelf / "wire.bin").write_bytes(response)
        (shelf / "diagnostics.bin").write_bytes(diagnostics)
        (shelf / "case.json").write_text(json.dumps({
            "command": command, "native_exit": native_exit, "parent_exit": parent_exit, "fault": fault,
        }), encoding="ascii")
        inputs: AccessWatchdogInputs = AccessWatchdogInputs(
            parent, Member(len(_PARENT), sha256(_PARENT).hexdigest()),
            watchdog, Member(watchdog.stat().st_size, sha256(watchdog.read_bytes()).hexdigest()),
            guardian, Member(1, "1" * 64), Member(2, "2" * 64), entry, Member(3, "3" * 64),
        )
        result: PrestartAccessAttempt = PrestartAccessAttempt(
            inputs, probe, diag, stream, (1800, 2300), (2600, 3400))

        def reap_control() -> None:
            process = result._process
            if process is not None:
                for pipe in (process.stdin, process.stdout, process.stderr):
                    if pipe is not None:
                        pipe.close()
                process.wait(timeout=3.0)

        self.files.callback(reap_control)
        return result, diag, stream

    def test_both_original_access_pipes_through_real_prestart_launch_consumer(self) -> None:
        access, _, _ = self.prepare("access", command="A", response=self.full.access)
        standard, _, _ = self.prepare("standard", probe=PrestartProbe.STANDARD, command="U",
                                      response=self.full.standard)
        captures: session.PrestartCaptures = session.PrestartCaptures()
        with fixtures.ScriptedProcess(self.full.script, self.full.deployment.retired) as old_window:
            deployment: fixtures.ScriptedOwnerAttempt[Run] = fixtures.ScriptedOwnerAttempt("deployment", [], old_window)
            result: PrestartObservation = session.launch_prestart_session(
                self.run, access, standard, deployment, self.work, self.retire, captures=captures,
            )
            self.full.deployment.assert_requests(old_window, b"SCICOAUF")
        self.assertEqual(result, PrestartObservation(self.run.id, "1" * 64, ((528, 528, 528), (528, 16, 16))))
        self.assertEqual(tuple(row.probe for row in captures.records),
                         (PrestartProbe.IDENTITY, PrestartProbe.OBSERVATION, PrestartProbe.ACCESS, PrestartProbe.STANDARD))
        with self.assertRaisesRegex(ReceiptRefused, "already used"):
            access.start(self.run, self.work, self.retire)

    def test_settled_native_three_retains_private_bytes_and_remains_refusal(self) -> None:
        line: bytes = b"nevrotick-deployment-owner failure=session cause=none cleanup=actor-close\r\n"
        data: bytes = line + b"nevrotick-prestart-access-capture " + json.dumps({
            "run": str(self.run.id), "role": "access", "expected": self.run.observer_sha256,
            "pid": 414, "before": None, "after": None, "exit": 44, "stdout": "/w==", "stderr": "",
        }).encode("ascii") + b"\n"
        attempt, diag, original = self.prepare("settled", native_exit=3, diagnostics=data)
        owner: session.OwnedDeploymentProcess = attempt.start(self.run, self.work, self.retire)
        with self.assertRaisesRegex(ReceiptRefused, "not settled"):
            attempt.private_diagnostic()
        attempt.request_retirement(self.retire)
        self.assertEqual(owner.stdout.read(1), b"")
        owner.stdout.close()
        with self.assertRaisesRegex(ReceiptRefused, "watchdog refused operation"):
            owner.wait_retired(self.work)
        attempt.wait_retired(self.retire)
        original.seek(1)
        result: session.PrivateAccessDiagnostic | None = attempt.private_diagnostic()
        self.assertEqual(result, session.PrivateAccessDiagnostic(self.run.id, PrestartProbe.ACCESS, 414, 44,
                                                                None, b"\xff", b""))
        self.assertEqual(original.tell(), 1)
        diag.rename(diag.with_name("retired-original"))
        diag.write_bytes(data)
        with self.assertRaisesRegex(ReceiptRefused, "foreign private native access diagnostics"):
            attempt.private_diagnostic()

    def test_rejects_coarse_or_foreign_status_and_changed_private_data(self) -> None:
        cases: tuple[tuple[int, int, str, str], ...] = (
            (0xC0000000, 0, "", "watchdog refused operation"),
            (0x10003, 0, "", "watchdog refused operation"),
            (1, 0, "", "watchdog refused operation"),
            (3, 1, "", "parent refused settlement"),
            (0, 0, "foreign-nonce", "foreign native access settlement"),
            (0, 0, "foreign-role", "foreign native access settlement"),
            (0, 0, "foreign-run", "foreign native access settlement"),
            (0, 0, "foreign-image", "foreign native access settlement"),
            (0, 0, "missing-record", "invalid native access settlement"),
            (0, 0, "extra-record", "oversized native access settlement"),
            (3, 0, "different-diagnostics", "diagnostic extent differs"),
        )
        index: int
        choice: tuple[int, int, str, str]
        for index, choice in enumerate(cases):
            native, parent, fault, expected = choice
            with self.subTest(native=native, parent=parent, fault=fault):
                attempt, _, _ = self.prepare(str(index), native_exit=native, parent_exit=parent, fault=fault)
                owner: session.OwnedDeploymentProcess = attempt.start(self.run, self.work, self.retire)
                attempt.request_retirement(self.retire)
                self.assertEqual(owner.stdout.read(1), b"")
                owner.stdout.close()
                with self.assertRaisesRegex(ReceiptRefused, expected):
                    owner.wait_retired(self.work)
                with self.assertRaisesRegex(ReceiptRefused, "not settled"):
                    attempt.private_diagnostic()

    def test_refused_access_drains_original_output_and_does_not_report_failed_cleanup(self) -> None:
        refused: bytes = self.full.deployment.reply("refused", code="actor-close", cleanup=[])
        access, _, _ = self.prepare("refused", command="A", response=refused + b"pending relay bytes",
                                    native_exit=3)
        ordinary, _, _ = self.prepare("unused", probe=PrestartProbe.STANDARD)
        with fixtures.ScriptedProcess(self.full.script[:6], self.full.deployment.retired) as window:
            deployment: fixtures.ScriptedOwnerAttempt[Run] = fixtures.ScriptedOwnerAttempt("deployment", [], window)
            with self.assertRaisesRegex(ReceiptRefused, "^native access refused actor-close$"):
                session.launch_prestart_session(self.run, access, ordinary, deployment, self.work, self.retire,
                                                captures=session.PrestartCaptures())
            self.full.deployment.assert_requests(window, b"SCICOA")
        self.assertIsNone(access.private_diagnostic())

    def test_complete_success_frame_with_native_three_cannot_advance_to_standard(self) -> None:
        access, _, _ = self.prepare("false-success", command="A", response=self.full.access, native_exit=3)
        ordinary, _, _ = self.prepare("unused", probe=PrestartProbe.STANDARD)
        with fixtures.ScriptedProcess(self.full.script[:6], self.full.deployment.retired) as window:
            deployment: fixtures.ScriptedOwnerAttempt[Run] = fixtures.ScriptedOwnerAttempt("deployment", [], window)
            with self.assertRaisesRegex(ReceiptRefused, "^native access watchdog refused operation$"):
                session.launch_prestart_session(self.run, access, ordinary, deployment, self.work, self.retire,
                                                captures=session.PrestartCaptures())
            self.full.deployment.assert_requests(window, b"SCICOA")

    def test_complete_success_bytes_cannot_survive_outer_owner_refusal(self) -> None:
        access: PrestartAccessAttempt
        ordinary: PrestartAccessAttempt
        access, _, _ = self.prepare("outer-refusal", command="A", response=self.full.access, parent_exit=1)
        ordinary, _, _ = self.prepare("unused", probe=PrestartProbe.STANDARD)
        with fixtures.ScriptedProcess(self.full.script[:6], self.full.deployment.retired) as window:
            deployment: fixtures.ScriptedOwnerAttempt[Run] = fixtures.ScriptedOwnerAttempt("deployment", [], window)
            with self.assertRaisesRegex(ReceiptRefused, "^native access parent refused settlement$"):
                session.launch_prestart_session(self.run, access, ordinary, deployment, self.work, self.retire,
                                                captures=session.PrestartCaptures())
            self.full.deployment.assert_requests(window, b"SCICOA")
        with self.assertRaisesRegex(ReceiptRefused, "native access diagnostics not settled"):
            access.private_diagnostic()

    def test_oversized_status_refuses_capture_and_reaps_original_relay(self) -> None:
        access: PrestartAccessAttempt
        ordinary: PrestartAccessAttempt
        access, _, _ = self.prepare("oversized", command="A", response=self.full.access, fault="extra-record")
        ordinary, _, _ = self.prepare("unused", probe=PrestartProbe.STANDARD)
        with fixtures.ScriptedProcess(self.full.script[:6], self.full.deployment.retired) as window:
            deployment: fixtures.ScriptedOwnerAttempt[Run] = fixtures.ScriptedOwnerAttempt("deployment", [], window)
            with self.assertRaisesRegex(ReceiptRefused, "^oversized native access settlement$"):
                session.launch_prestart_session(self.run, access, ordinary, deployment, self.work, self.retire,
                                                captures=session.PrestartCaptures())
            self.full.deployment.assert_requests(window, b"SCICOA")
        process: Popen[bytes] | None = access._process
        assert process is not None
        # Do not poll or wait here. Observe the exit already collected by the
        # supervisor, before the test custodian can reap the original relay.
        self.assertEqual(process.returncode, 0)

    def test_oversized_status_preserves_refusal_when_original_relay_exceeds_retirement(self) -> None:
        access: PrestartAccessAttempt
        ordinary: PrestartAccessAttempt
        access, _, _ = self.prepare("late-status", command="A", response=self.full.access, fault="extra-record")
        ordinary, _, _ = self.prepare("unused", probe=PrestartProbe.STANDARD)
        original_wait: Callable[[Popen[bytes], float | None], int] = Popen.wait
        budgets: list[tuple[float, float]] = []
        waits: list[float] = []

        def remaining(deadline: float) -> float:
            budget: float = _remaining(deadline)
            budgets.append((deadline, budget))
            return budget

        def wait(process: Popen[bytes], timeout: float | None = None) -> int:
            if process is access._process:
                deadline: float
                budget: float
                deadline, budget = budgets[-1]
                self.assertEqual(budget, timeout)
                waits.append(deadline)
                raise TimeoutExpired(process.args, timeout)
            return original_wait(process, timeout)

        with fixtures.ScriptedProcess(self.full.script[:6], self.full.deployment.retired) as window:
            deployment: fixtures.ScriptedOwnerAttempt[Run] = fixtures.ScriptedOwnerAttempt("deployment", [], window)
            with (patch.object(Popen, "wait", wait), patch("prestart_access_process._remaining", remaining),
                  self.assertRaises(ExceptionGroup) as refused):
                session.launch_prestart_session(self.run, access, ordinary, deployment, self.work, self.retire,
                                                captures=session.PrestartCaptures())
            self.full.deployment.assert_requests(window, b"SCICOA")
        primary: BaseException
        retirement: BaseException
        primary, retirement = refused.exception.exceptions
        self.assertIsInstance(primary, ReceiptRefused)
        self.assertEqual(str(primary), "oversized native access settlement")
        self.assertIsInstance(retirement, TimeoutExpired)
        self.assertEqual(waits, [self.retire])
        process: Popen[bytes] | None = access._process
        assert process is not None and process.stderr is not None
        self.assertIsNone(process.returncode)
        self.assertFalse(process.stderr.closed)

    def test_oversized_status_and_distinct_cleanup_refusal_each_report_once(self) -> None:
        access: PrestartAccessAttempt
        ordinary: PrestartAccessAttempt
        access, _, _ = self.prepare("oversized", command="A", response=self.full.access, fault="extra-record")
        ordinary, _, _ = self.prepare("refused", probe=PrestartProbe.STANDARD, parent_exit=1)
        with fixtures.ScriptedProcess(self.full.script[:6], self.full.deployment.retired) as window:
            deployment: fixtures.ScriptedOwnerAttempt[Run] = fixtures.ScriptedOwnerAttempt("deployment", [], window)
            with self.assertRaises(ExceptionGroup) as refused:
                session.launch_prestart_session(self.run, access, ordinary, deployment, self.work, self.retire,
                                                captures=session.PrestartCaptures())
            self.full.deployment.assert_requests(window, b"SCICOA")
        primary: BaseException
        retirement: BaseException
        primary, retirement = refused.exception.exceptions
        self.assertIsInstance(primary, ReceiptRefused)
        self.assertEqual(str(primary), "oversized native access settlement")
        self.assertIsInstance(retirement, ReceiptRefused)
        self.assertEqual(str(retirement), "native access parent refused settlement")
        process: Popen[bytes] | None = access._process
        other: Popen[bytes] | None = ordinary._process
        assert process is not None and other is not None
        self.assertEqual((process.returncode, other.returncode), (0, 1))

    def test_oversized_status_first_seen_during_cleanup_is_reported(self) -> None:
        response: bytes = self.full.deployment.reply("refused", code="actor-close", cleanup=[])
        access: PrestartAccessAttempt
        ordinary: PrestartAccessAttempt
        access, _, _ = self.prepare("refused", command="A", response=response, fault="extra-record")
        ordinary, _, _ = self.prepare("unused", probe=PrestartProbe.STANDARD)
        with fixtures.ScriptedProcess(self.full.script[:6], self.full.deployment.retired) as window:
            deployment: fixtures.ScriptedOwnerAttempt[Run] = fixtures.ScriptedOwnerAttempt("deployment", [], window)
            with self.assertRaises(ExceptionGroup) as refused:
                session.launch_prestart_session(self.run, access, ordinary, deployment, self.work, self.retire,
                                                captures=session.PrestartCaptures())
            self.full.deployment.assert_requests(window, b"SCICOA")
        primary: BaseException
        retirement: BaseException
        primary, retirement = refused.exception.exceptions
        self.assertIsInstance(primary, ReceiptRefused)
        self.assertEqual(str(primary), "native access refused actor-close")
        self.assertIsInstance(retirement, ReceiptRefused)
        self.assertEqual(str(retirement), "oversized native access settlement")
        process: Popen[bytes] | None = access._process
        assert process is not None
        self.assertEqual(process.returncode, 0)

    def test_cleanup_status_overflow_survives_an_independent_relay_exit_failure(self) -> None:
        response: bytes = self.full.deployment.reply("refused", code="actor-close", cleanup=[])
        access: PrestartAccessAttempt
        ordinary: PrestartAccessAttempt
        access, _, _ = self.prepare("refused", command="A", response=response, fault="extra-record", parent_exit=1)
        ordinary, _, _ = self.prepare("unused", probe=PrestartProbe.STANDARD)
        with fixtures.ScriptedProcess(self.full.script[:6], self.full.deployment.retired) as window:
            deployment: fixtures.ScriptedOwnerAttempt[Run] = fixtures.ScriptedOwnerAttempt("deployment", [], window)
            with self.assertRaises(ExceptionGroup) as refused:
                session.launch_prestart_session(self.run, access, ordinary, deployment, self.work, self.retire,
                                                captures=session.PrestartCaptures())
            self.full.deployment.assert_requests(window, b"SCICOA")
        primary: BaseException
        retirement: BaseException
        primary, retirement = refused.exception.exceptions
        self.assertEqual(str(primary), "native access refused actor-close")
        self.assertIsInstance(retirement, ExceptionGroup)
        assert isinstance(retirement, ExceptionGroup)
        self.assertEqual(tuple(map(str, retirement.exceptions)), (
            "oversized native access settlement", "native access parent refused settlement",
        ))
        process: Popen[bytes] | None = access._process
        assert process is not None
        self.assertEqual(process.returncode, 1)

    def test_exhausted_output_drain_refuses_observation_and_reaps_original_relay(self) -> None:
        response: bytes = bytes(4) + bytes(session.MAX_FRAME + 5)
        access: PrestartAccessAttempt
        ordinary: PrestartAccessAttempt
        access, _, _ = self.prepare("excess-wire", command="A", response=response)
        ordinary, _, _ = self.prepare("unused", probe=PrestartProbe.STANDARD)
        with fixtures.ScriptedProcess(self.full.script[:6], self.full.deployment.retired) as window:
            deployment: fixtures.ScriptedOwnerAttempt[Run] = fixtures.ScriptedOwnerAttempt("deployment", [], window)
            with self.assertRaises(ExceptionGroup) as refused:
                session.launch_prestart_session(self.run, access, ordinary, deployment, self.work, self.retire,
                                                captures=session.PrestartCaptures())
            self.full.deployment.assert_requests(window, b"SCICOA")
        self.assertEqual(tuple(map(str, refused.exception.exceptions)), (
            "invalid deployment frame length", "excess native access output during retirement",
        ))
        process: Popen[bytes] | None = access._process
        assert process is not None and process.stdout is not None and process.stderr is not None
        self.assertEqual(process.returncode, 0)
        self.assertTrue(process.stdout.closed and process.stderr.closed)

    def test_exhausted_output_drain_retains_unconfirmed_original_relay(self) -> None:
        response: bytes = bytes(4) + bytes(session.MAX_FRAME + 5)
        access: PrestartAccessAttempt
        ordinary: PrestartAccessAttempt
        access, _, _ = self.prepare("excess-wire", command="A", response=response)
        ordinary, _, _ = self.prepare("unused", probe=PrestartProbe.STANDARD)
        original_wait: Callable[[Popen[bytes], float | None], int] = Popen.wait
        deadlines: list[float] = []
        waits: list[float] = []

        def remaining(deadline: float) -> float:
            deadlines.append(deadline)
            return _remaining(deadline)

        def wait(process: Popen[bytes], timeout: float | None = None) -> int:
            if process is access._process:
                waits.append(deadlines[-1])
                raise TimeoutExpired(process.args, timeout)
            return original_wait(process, timeout)

        with fixtures.ScriptedProcess(self.full.script[:6], self.full.deployment.retired) as window:
            deployment: fixtures.ScriptedOwnerAttempt[Run] = fixtures.ScriptedOwnerAttempt("deployment", [], window)
            with (patch.object(Popen, "wait", wait), patch("prestart_access_process._remaining", remaining),
                  self.assertRaises(ExceptionGroup) as refused):
                session.launch_prestart_session(self.run, access, ordinary, deployment, self.work, self.retire,
                                                captures=session.PrestartCaptures())
            self.full.deployment.assert_requests(window, b"SCICOA")
        primary: BaseException
        retirement: BaseException
        primary, retirement = refused.exception.exceptions
        self.assertEqual(str(primary), "invalid deployment frame length")
        self.assertIsInstance(retirement, ExceptionGroup)
        assert isinstance(retirement, ExceptionGroup)
        drain: BaseException
        reap: BaseException
        drain, reap = retirement.exceptions
        self.assertEqual(str(drain), "excess native access output during retirement")
        self.assertIsInstance(reap, TimeoutExpired)
        self.assertEqual(waits, [self.retire])
        process: Popen[bytes] | None = access._process
        assert process is not None and process.stdout is not None and process.stderr is not None
        self.assertIsNone(process.returncode)
        self.assertFalse(process.stdout.closed or process.stderr.closed)

    def test_rollback_closes_original_pipes_after_late_launch_return(self) -> None:
        attempt, _, _ = self.prepare("late")
        now: float = time.monotonic()
        with patch("prestart_access_process._remaining", side_effect=[1.0, TimeoutError("injected late launch")]):
            with self.assertRaisesRegex(TimeoutError, "late launch"):
                attempt.start(self.run, now + 1.0, now + 3.0)
        with self.assertRaisesRegex(ReceiptRefused, "changed native access retirement cutoff"):
            attempt.request_retirement(now + 4.0)
        attempt.request_retirement(now + 3.0)
        attempt.wait_retired(now + 3.0)

    def test_fixed_drive_translation(self) -> None:
        self.assertEqual(_windows_path(Path("/mnt/c/Owned Space/input.exe")), "C:\\Owned Space\\input.exe")
        bad: str
        for bad in ("/mnt/wsl/temp", "/mnt/c/../x", "/mnt/c/x:stream", "/mnt/c/trailing.", "/tmp/a"):
            with self.subTest(path=bad), self.assertRaisesRegex(ReceiptRefused, "local Windows staging path"):
                _windows_path(Path(bad))

    def test_host_clipping_preserves_positive_cooperative_and_recovery_allowances(self) -> None:
        self.assertEqual(_limited_budgets((8, 20), 0.0, 0.002, 0.006, minimum_grace_ms=4), (2, 6))
        self.assertEqual(_limited_budgets((8, 20), 0.0, 0.002, 0.003), (2, 3))
        with self.assertRaisesRegex(ReceiptRefused, "insufficient pre-start access launch allowance"):
            _limited_budgets((8, 20), 0.0, 0.002, 0.005, minimum_grace_ms=4)


if __name__ == "__main__":
    unittest.main()
