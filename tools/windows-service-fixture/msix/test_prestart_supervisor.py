# SPDX-License-Identifier: GPL-2.0-only
"""Drive the S0.6 supervisor and its real receipt readers with inert captures."""

from dataclasses import replace
import unittest
from uuid import UUID

from prestart_receipts import Capture, ReceiptRefused, Run
from prestart_supervisor import supervise_prestart
import test_prestart_receipts as receipts


class ScriptedWindow:
    """Observe the closed call route and inject custody changes at known points."""

    def __init__(self, run: Run, captures: tuple[Capture, ...]) -> None:
        self.run = run
        self.captures: list[Capture | Exception] = list(captures)
        self.calls: list[tuple[object, ...]] = []
        self.at_check: dict[int, Run | Exception] = {}
        self.checks = 0
        self.capture_count = 0
        self.taken = False
        self.exhausted = ReceiptRefused("scripted window was already consumed")

    def take_run(self) -> Run:
        self.calls.append(("take",))
        if self.taken:
            raise self.exhausted
        self.taken = True
        return self.run

    def current_run(self) -> Run:
        self.calls.append(("current",))
        self.checks += 1
        outcome = self.at_check.get(self.checks, self.run)
        if isinstance(outcome, Exception):
            raise outcome
        return outcome

    def next_capture(self) -> Capture:
        outcome = self.captures[self.capture_count]
        self.capture_count += 1
        if isinstance(outcome, Exception):
            raise outcome
        return outcome

    def capture_identity(self, run: Run) -> Capture:
        self.calls.append(("identity", run))
        return self.next_capture()

    def capture_observer(self, run: Run, actor: object, arguments: tuple[str, ...]) -> Capture:
        self.calls.append(("observer", run, actor, arguments))
        return self.next_capture()


class PrestartSupervisorTests(unittest.TestCase):
    def setUp(self) -> None:
        self.fixture = receipts.PrestartReceiptTests()
        self.fixture.setUp()

    def window(self) -> ScriptedWindow:
        fixture = self.fixture
        return ScriptedWindow(fixture.run, (fixture.identity, fixture.observation, fixture.access, fixture.standard))

    def test_routes_fixed_calls_and_publishes_the_real_join_result(self) -> None:
        window = self.window()
        result = supervise_prestart(window)
        self.assertEqual(result.run_id, window.run.id)
        self.assertEqual(result.signed_package_sha256, "1" * 64)
        self.assertEqual(result.service_types, ((528, 528, 528), (528, 16, 16)))
        self.assertEqual([call[0] for call in window.calls], [
            "take", "current", "identity", "current", "observer",
            "current", "observer", "current", "observer", "current",
        ])
        self.assertIs(window.calls[2][1], window.run)
        for index, selector, actor in (
            (4, receipts.READ, window.run.deployment_actor),
            (6, receipts.ACCESS, window.run.provisioning_actor),
            (8, receipts.ORDINARY, window.run.standard_actor),
        ):
            with self.subTest(selector=selector):
                call = window.calls[index]
                self.assertIs(call[1], window.run)
                self.assertIs(call[2], actor)
                self.assertIs(type(call[3]), tuple)
                self.assertEqual(call[3], (selector, "--exact", "--ignored", "--nocapture", "--test-threads=1",
                                           "--format", "pretty", "--color", "never"))

    def test_provisioning_authority_refusal_skips_standard_user_capture(self) -> None:
        window = self.window()
        # Keep the child's exit and custody apparently healthy to reach the real reader.
        refused = self.fixture.access.stdout.replace(b"opened-and-closed", b"access-denied", 1)
        window.captures[2] = replace(self.fixture.access, stdout=refused)
        with self.assertRaisesRegex(ReceiptRefused, "provisioning access rows differ"):
            supervise_prestart(window)
        self.assertEqual(window.capture_count, 3)
        self.assertEqual(supervise_prestart(self.window()).service_types, ((528, 528, 528), (528, 16, 16)))

    def test_wrong_run_actor_or_process_custody_skips_later_captures(self) -> None:
        for changes in (
            {"run_id": UUID(int=124)},
            {"actor": self.fixture.run.provisioning_actor},
            {"input_sha256_after": "0" * 64},
            {"exit_code": 1},
            {"reaped": False},
        ):
            with self.subTest(changes=changes):
                window = self.window()
                window.captures[1] = replace(self.fixture.observation, **changes)
                with self.assertRaises(ReceiptRefused):
                    supervise_prestart(window)
                self.assertEqual(window.capture_count, 2)

    def test_equal_but_different_window_refuses_before_and_after_capture(self) -> None:
        for check, capture_count in ((1, 0), (2, 1), (5, 4)):
            with self.subTest(check=check):
                window = self.window()
                window.at_check[check] = replace(window.run)
                with self.assertRaises(ReceiptRefused):
                    supervise_prestart(window)
                self.assertEqual(window.capture_count, capture_count)
                self.assertEqual(window.checks, check)

    def test_first_adapter_refusal_keeps_its_cause_and_consumes_the_attempt(self) -> None:
        window = self.window()
        first = ReceiptRefused("scripted capture cleanup remains unresolved")
        window.captures[0] = first
        # A later window check must not replace the original adapter refusal.
        window.at_check[2] = ReceiptRefused("scripted later custody failure")
        with self.assertRaises(ReceiptRefused) as caught:
            supervise_prestart(window)
        self.assertIs(caught.exception, first)
        self.assertEqual(window.calls, [("take",), ("current",), ("identity", window.run)])

        with self.assertRaises(ReceiptRefused) as caught:
            supervise_prestart(window)
        self.assertIs(caught.exception, window.exhausted)
        self.assertEqual(window.calls[-1], ("take",))
        self.assertEqual(window.capture_count, 1)

    def test_failed_child_receipt_precedes_a_later_window_refusal(self) -> None:
        window = self.window()
        window.captures[0] = replace(self.fixture.identity, exit_code=1)
        window.at_check[2] = ReceiptRefused("scripted later window withdrawal")
        with self.assertRaisesRegex(ReceiptRefused, "child did not exit successfully and reap"):
            supervise_prestart(window)
        self.assertEqual(window.capture_count, 1)
        self.assertEqual(window.checks, 1)

    def test_custody_checker_refusal_propagates_at_publication(self) -> None:
        window = self.window()
        refused = ReceiptRefused("scripted custody lost during final publication check")
        window.at_check[5] = refused
        with self.assertRaises(ReceiptRefused) as caught:
            supervise_prestart(window)
        self.assertIs(caught.exception, refused)
        self.assertEqual(window.capture_count, 4)
