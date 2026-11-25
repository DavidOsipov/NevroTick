# SPDX-License-Identifier: GPL-2.0-only

from hashlib import sha256
import json
from pathlib import Path
from tempfile import TemporaryDirectory
from threading import Thread
from time import monotonic, sleep
import unittest
from unittest.mock import patch

from prestart_deployment_session import supervise_prestart_session
from prestart_mapped_deployment import MappedDeploymentProcess
from prestart_receipts import ReceiptRefused
import test_prestart_deployment_session as fixtures
import prestart_mapped_deployment as mapped


def commit(folder: Path, name: str, data: bytes) -> None:
    with (folder / name).open("xb") as stream:
        stream.write(data)
    with (folder / (name + ".commit")).open("xb"):
        pass


class MappedDeploymentTests(unittest.TestCase):
    def exercise(self, *, crossed_retirement: bool) -> None:
        fixture = fixtures.FullPrestartSessionTests()
        fixture.setUp()
        work = monotonic() + 3
        retirement = work + 1
        native_observations: list[float] = []
        commands: list[bytes] = []
        failure: list[BaseException] = []
        diagnostic = b"private inert watchdog diagnostic\n"
        result = None
        with (
            TemporaryDirectory() as temp,
            fixture.actor(b"A", fixture.access) as access,
            fixture.actor(b"U", fixture.standard) as standard,
        ):
            root = Path(temp)
            inbox, outbox = root / "in", root / "out"
            inbox.mkdir()
            outbox.mkdir()
            run = fixture.bind(access, standard)
            responses = (*fixture.script, (b"R", fixture.deployment.retired))

            def observe_original(cutoff: float) -> tuple[int, bytes]:
                native_observations.append(cutoff)
                return (882 if crossed_retirement else 881), diagnostic

            def peer() -> None:
                try:
                    for index, (command, frame) in enumerate(responses, 1):
                        leaf = inbox / f"request-{index}.bin"
                        while not leaf.with_suffix(".bin.commit").exists():
                            if monotonic() >= retirement:
                                raise TimeoutError("inert mapped request missing")
                            sleep(0.002)
                        observed = leaf.read_bytes()
                        commands.append(observed)
                        if command != observed:
                            raise AssertionError((command, observed))
                        commit(outbox, f"reply-{index}.bin", frame)
                    wire = b"".join(frame for _, frame in responses)
                    commit(outbox, "watchdog.stdout.bin", wire)
                    commit(outbox, "watchdog.stderr.bin", diagnostic)
                    commit(outbox, "relay.json", json.dumps({
                        "run": str(run.id), "commands": b"".join(commands).decode("ascii"),
                        "reply_count": len(responses), "launch_attempted": True, "transport_ok": True,
                        "retired": True, "watchdog_pid": 881, "watchdog_exit": 0, "stdout_eof": True,
                        "stderr_eof": True, "stdout_sha256": sha256(wire).hexdigest(),
                        "stderr_sha256": sha256(diagnostic).hexdigest(),
                    }).encode("utf-8"))
                except BaseException as error:
                    failure.append(error)

            owner = MappedDeploymentProcess(run, inbox, outbox, work, retirement, observe_original)
            remote = Thread(target=peer)
            remote.start()
            try:
                if crossed_retirement:
                    with self.assertRaisesRegex(ReceiptRefused, "native observation differs"):
                        supervise_prestart_session(run, owner, access, standard, work, retirement)
                else:
                    result = supervise_prestart_session(run, owner, access, standard, work, retirement)
            finally:
                owner.stdin.close()
                owner.stdout.close()
                remote.join(timeout=max(0.0, retirement - monotonic()))
                self.assertFalse(remote.is_alive())
        self.assertEqual(failure, [])
        self.assertEqual(b"".join(commands), b"SCICOAUFR")
        self.assertEqual(native_observations, [retirement])
        self.assertEqual(access.requests, [b"A", b""])
        self.assertEqual(standard.requests, [b"U", b""])
        if not crossed_retirement:
            self.assertEqual(result, fixtures.PrestartObservation(
                run.id, "1" * 64, ((528, 528, 528), (528, 16, 16)),
            ))

    def test_full_supervisor_joins_both_access_owners_and_mapped_deployment_reply(self) -> None:
        self.exercise(crossed_retirement=False)

    def test_native_watchdog_identity_must_match_even_with_complete_mapped_transcript(self) -> None:
        self.exercise(crossed_retirement=True)

    def test_unread_reply_larger_than_pipe_capacity_cannot_hold_the_bridge_past_deadline(self) -> None:
        from fcntl import F_SETPIPE_SZ, fcntl

        fixture = fixtures.FullPrestartSessionTests()
        fixture.setUp()
        with TemporaryDirectory() as temp:
            root = Path(temp)
            inbox, outbox = root / "in", root / "out"
            inbox.mkdir()
            outbox.mkdir()
            work = monotonic() + 1
            retirement = work + 0.5
            owner = MappedDeploymentProcess(fixture.bind(object(), object()), inbox, outbox, work, retirement,
                                             lambda _cutoff: (881, b""))
            try:
                fcntl(owner.stdout.fileno(), F_SETPIPE_SZ, 4096)
                owner.stdin.write(b"S")
                while not (inbox / "request-1.bin.commit").exists():
                    self.assertLess(monotonic(), work, "request was not committed")
                    sleep(0.002)
                commit(outbox, "reply-1.bin", fixtures.frame(b"x" * 24000))
                owner._thread.join(timeout=max(0.0, retirement + 1 - monotonic()))
                self.assertFalse(owner._thread.is_alive())
                self.assertIsInstance(owner._error, TimeoutError)
                self.assertEqual(len(owner._forwarded), 24004)
            finally:
                owner.stdin.close()
                owner.stdout.close()

    def test_expired_publication_leaves_the_same_slot_available_for_retirement(self) -> None:
        fixture = fixtures.FullPrestartSessionTests()
        fixture.setUp()
        with TemporaryDirectory() as temp:
            root = Path(temp)
            inbox, outbox = root / "in", root / "out"
            inbox.mkdir()
            outbox.mkdir()
            run = fixture.bind(object(), object())
            work = monotonic() + 5
            retirement = work + 2
            leaf = inbox / "request-1.bin"
            samples: list[float] = []
            forced_expiry = False
            wire = fixture.deployment.retired
            diagnostic = b"original terminal diagnostic"
            owner = MappedDeploymentProcess(run, inbox, outbox, work, retirement, lambda _cutoff: (881, diagnostic))

            def sample_time() -> float:
                nonlocal forced_expiry
                if not forced_expiry and leaf.exists():
                    forced_expiry = True
                    samples.append(work)
                    return work
                return monotonic()

            try:
                with patch.object(mapped, "monotonic", side_effect=sample_time):
                    owner.stdin.write(b"S")
                    while owner._work_expired is None:
                        self.assertLess(monotonic(), work, "work expiry was not observed")
                        sleep(0.002)
                    self.assertFalse(leaf.exists())
                    self.assertFalse(leaf.with_suffix(".bin.commit").exists())
                    owner.stdin.write(b"R")
                    owner.stdin.close()
                    while not leaf.with_suffix(".bin.commit").exists():
                        self.assertLess(monotonic(), work, "retirement slot was not committed")
                        sleep(0.002)
                self.assertEqual(leaf.read_bytes(), b"R")
                commit(outbox, "reply-1.bin", wire)
                commit(outbox, "watchdog.stdout.bin", wire)
                commit(outbox, "watchdog.stderr.bin", diagnostic)
                commit(outbox, "relay.json", json.dumps({
                    "run": str(run.id), "commands": "R", "reply_count": 1, "launch_attempted": True,
                    "transport_ok": True, "retired": True, "watchdog_pid": 881, "watchdog_exit": 0,
                    "stdout_eof": True, "stderr_eof": True, "stdout_sha256": sha256(wire).hexdigest(),
                    "stderr_sha256": sha256(diagnostic).hexdigest(),
                }).encode())
                self.assertEqual(owner.stdout.readall(), wire)
                owner.stdout.close()
                owner.wait_retired(retirement)
                self.assertIsInstance(owner._work_expired, TimeoutError)
                self.assertEqual(samples, [work])
            finally:
                owner.stdin.close()
                owner.stdout.close()

    def test_interrupted_start_after_thread_creation_never_transfers_descriptors(self) -> None:
        fixture = fixtures.FullPrestartSessionTests()
        fixture.setUp()
        threads: list[Thread] = []
        start = Thread.start

        def interrupted(thread: Thread) -> None:
            start(thread)
            threads.append(thread)
            raise KeyboardInterrupt

        with TemporaryDirectory() as temp:
            root = Path(temp)
            inbox, outbox = root / "in", root / "out"
            inbox.mkdir()
            outbox.mkdir()
            work = monotonic() + 1
            with (
                patch.object(Thread, "start", interrupted),
                patch.object(MappedDeploymentProcess, "_bridge") as bridge,
                self.assertRaises(KeyboardInterrupt),
            ):
                MappedDeploymentProcess(fixture.bind(object(), object()), inbox, outbox, work, work + 1,
                                        lambda _cutoff: (881, b""))
            self.assertEqual(len(threads), 1)
            threads[0].join(timeout=1)
            self.assertFalse(threads[0].is_alive())
            bridge.assert_not_called()

    def test_preserves_native_and_local_retirement_failures(self) -> None:
        fixture = fixtures.FullPrestartSessionTests()
        fixture.setUp()
        native_problem = OSError("inert native retirement refusal")

        def confirm(_cutoff: float) -> tuple[int, bytes]:
            raise native_problem

        with TemporaryDirectory() as temp:
            root = Path(temp)
            inbox, outbox = root / "in", root / "out"
            inbox.mkdir()
            outbox.mkdir()
            work = monotonic() + 2
            owner = MappedDeploymentProcess(fixture.bind(object(), object()), inbox, outbox, work, work + 1, confirm)
            try:
                owner.stdin.write(b"X")
                owner.stdin.close()
                self.assertEqual(owner.stdout.readall(), b"")
                owner.stdout.close()
                with self.assertRaises(ExceptionGroup) as refused:
                    owner.wait_retired(work + 1)
                observed, bridge = refused.exception.exceptions
                self.assertIs(observed, native_problem)
                self.assertIsInstance(bridge, ReceiptRefused)
                self.assertEqual(str(bridge), "invalid deployment relay command")
                self.assertFalse(owner._thread.is_alive())
            finally:
                owner.stdin.close()
                owner.stdout.close()
