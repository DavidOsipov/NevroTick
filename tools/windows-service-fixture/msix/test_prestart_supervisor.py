# SPDX-License-Identifier: GPL-2.0-only
"""Drive the S0.6 supervisor and its real receipt readers with inert captures."""

from dataclasses import replace
from typing import Generic, TypeVar
import unittest
from unittest.mock import patch
from uuid import UUID

from prestart_receipts import (
    Capture, DeploymentObservation, DeploymentRun, PrestartObservation, ReceiptRefused, Run, ScmTypeSample,
    join_prestart,
)
from prestart_supervisor import supervise_deployment, supervise_prestart
import test_prestart_receipts as receipts


RunT = TypeVar("RunT", bound=DeploymentRun)


class ScriptedDeploymentWindow(Generic[RunT]):
    """Observe the closed call route and inject custody changes at known points."""

    def __init__(self, run: RunT, captures: tuple[Capture, ...]) -> None:
        self.run: RunT = run
        self.initial_types: ScmTypeSample = ScmTypeSample(engine=(528, 528), broker=(528, 16))
        self.captures: list[Capture | BaseException] = list(captures)
        self.calls: list[tuple[object, ...]] = []
        self.at_check: dict[int, RunT | Exception] = {}
        self.checks = 0
        self.capture_count = 0
        self.taken = False
        self.admission_failure: BaseException | None = None
        self.retirement_failure: BaseException | None = None
        self.exhausted = ReceiptRefused("scripted window was already consumed")

    def take_run(self) -> RunT:
        self.calls.append(("take",))
        if self.admission_failure is not None:
            raise self.admission_failure
        if self.taken:
            raise self.exhausted
        self.taken = True
        return self.run

    def current_run(self) -> RunT:
        self.calls.append(("current",))
        self.checks += 1
        outcome: RunT | Exception = self.at_check.get(self.checks, self.run)
        if isinstance(outcome, Exception):
            raise outcome
        return outcome

    def initial_scm_types(self, run: RunT) -> ScmTypeSample:
        self.calls.append(("scm-sample", run))
        return self.initial_types

    def next_capture(self) -> Capture:
        outcome = self.captures[self.capture_count]
        self.capture_count += 1
        if isinstance(outcome, BaseException):
            raise outcome
        return outcome

    def capture_identity(self, run: RunT) -> Capture:
        self.calls.append(("identity", run))
        return self.next_capture()

    def capture_deployment_observation(self, run: RunT) -> Capture:
        self.calls.append(("deployment-observation", run))
        return self.next_capture()

    def retire_run(self, run: RunT) -> None:
        self.calls.append(("retire", run))
        if self.retirement_failure is not None:
            raise self.retirement_failure


class ScriptedWindow(ScriptedDeploymentWindow[Run]):
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

    def deployment_window(self) -> ScriptedDeploymentWindow[DeploymentRun]:
        fixture: receipts.PrestartReceiptTests = self.fixture
        return ScriptedDeploymentWindow(fixture.deployment_run(), (fixture.identity, fixture.observation))

    def assert_retired(self, window: ScriptedDeploymentWindow[RunT]) -> None:
        retirements: list[tuple[object, ...]] = [call for call in window.calls if call[0] == "retire"]
        self.assertEqual(len(retirements), 1)
        self.assertIs(retirements[0][1], window.run)
        self.assertEqual(window.calls[-1][0], "retire")

    def test_deployment_only_publishes_after_the_original_run_retires(self) -> None:
        window: ScriptedDeploymentWindow[DeploymentRun] = self.deployment_window()
        result: DeploymentObservation = supervise_deployment(window)
        self.assertEqual(
            result, DeploymentObservation(window.run.id, "1" * 64, ((528, 528, 528), (528, 16, 16))),
        )
        self.assertEqual([call[0] for call in window.calls], [
            "take", "scm-sample", "current", "identity", "current", "deployment-observation", "current", "retire",
        ])
        for index in (1, 3, 5, 7):
            self.assertIs(window.calls[index][1], window.run)
        self.assert_retired(window)

    def test_deployment_only_checks_original_run_before_each_capture_and_return(self) -> None:
        for check in (1, 2, 3):
            with self.subTest(check=check):
                window: ScriptedDeploymentWindow[DeploymentRun] = self.deployment_window()
                window.at_check[check] = replace(window.run)
                with self.assertRaisesRegex(ReceiptRefused, "pre-start deployment window changed"):
                    supervise_deployment(window)
                self.assertEqual(window.capture_count, check - 1)
                self.assertEqual(window.checks, check)
                self.assert_retired(window)

    def test_deployment_only_retirement_error_withholds_valid_observation(self) -> None:
        window: ScriptedDeploymentWindow[DeploymentRun] = self.deployment_window()
        refusal: OSError = OSError("scripted SCM close unconfirmed")
        window.retirement_failure = refusal
        with self.assertRaises(OSError) as caught:
            supervise_deployment(window)
        self.assertIs(caught.exception, refusal)
        self.assertEqual(window.capture_count, 2)
        self.assertEqual(window.checks, 3)
        self.assert_retired(window)

    def test_routes_fixed_calls_and_publishes_the_real_join_result(self) -> None:
        window = self.window()
        result = supervise_prestart(window)
        self.assertEqual(result.run_id, window.run.id)
        self.assertEqual(result.signed_package_sha256, "1" * 64)
        self.assertEqual(result.service_types, ((528, 528, 528), (528, 16, 16)))
        self.assertEqual([call[0] for call in window.calls], [
            "take", "scm-sample", "current", "identity", "current", "deployment-observation",
            "current", "observer", "current", "observer", "current", "retire",
        ])
        self.assert_retired(window)
        self.assertIs(window.calls[1][1], window.run)
        self.assertIs(window.calls[3][1], window.run)
        self.assertIs(window.calls[5][1], window.run)
        for index, selector, actor in (
            (7, receipts.ACCESS, window.run.provisioning_actor),
            (9, receipts.ORDINARY, window.run.standard_actor),
        ):
            with self.subTest(selector=selector):
                call = window.calls[index]
                self.assertIs(call[1], window.run)
                self.assertIs(call[2], actor)
                self.assertIs(type(call[3]), tuple)
                self.assertEqual(call[3], (selector, "--exact", "--ignored", "--nocapture", "--test-threads=1",
                                           "--format", "pretty", "--color", "never"))

    def test_invalid_identity_receipt_never_reaches_deployment_observation(self) -> None:
        window: ScriptedWindow = self.window()
        identity: Capture = self.fixture.identity
        original_digest: bytes = b'"sha256": "' + b"2" * 64 + b'"'
        different_digest: bytes = b'"sha256": "' + b"f" * 64 + b'"'
        self.assertIn(original_digest, identity.stdout)
        window.captures[0] = replace(identity, stdout=identity.stdout.replace(original_digest, different_digest, 1))
        with self.assertRaisesRegex(ReceiptRefused, "installed identity differs from the original run or package"):
            supervise_prestart(window)
        self.assertEqual(window.calls, [
            ("take",), ("scm-sample", window.run), ("current",), ("identity", window.run), ("retire", window.run),
        ])
        self.assertEqual(window.capture_count, 1)
        self.assert_retired(window)

    def test_agreeing_child_observations_cannot_replace_original_scm_types(self) -> None:
        fixture: receipts.PrestartReceiptTests = self.fixture
        for original, altered, expected in (
            ("configured-type=528", "configured-type=16", (16, 16, 16)),
            ("before-type=16", "before-type=32", (528, 32, 16)),
            ("after-type=16", "after-type=32", (528, 16, 32)),
        ):
            with self.subTest(field=original):
                window: ScriptedWindow = self.window()
                role_row: str = fixture.role_rows[1].replace(original, altered)
                observation_rows: list[str] = [fixture.role_rows[0], role_row, fixture.role_rows[2]]
                access_rows: list[str] = [fixture.access_rows[0], role_row, *fixture.access_rows[2:]]
                observation: Capture = replace(fixture.observation, stdout=receipts.libtest(receipts.READ, observation_rows))
                access: Capture = replace(fixture.access, stdout=receipts.libtest(receipts.ACCESS, access_rows))
                offline: PrestartObservation = join_prestart(
                    window.run, fixture.identity, observation, access, fixture.standard,
                )
                self.assertEqual(offline.service_types, ((528, 528, 528), expected))
                window.captures[1] = observation
                window.captures[2] = access
                with self.assertRaisesRegex(ReceiptRefused, "SCM types differ from the original window"):
                    supervise_prestart(window)
                self.assertEqual(window.capture_count, 2)
                self.assertEqual(window.checks, 2)
                self.assert_retired(window)

    def test_swapped_sample_roles_refuse_before_access_and_preserve_retirement_error(self) -> None:
        window: ScriptedWindow = self.window()
        window.initial_types = ScmTypeSample(engine=(528, 16), broker=(528, 528))
        retirement: OSError = OSError("scripted original SCM close remains unconfirmed")
        window.retirement_failure = retirement
        with self.assertRaises(BaseExceptionGroup) as caught:
            supervise_prestart(window)
        primary, cleanup = caught.exception.exceptions
        self.assertIsInstance(primary, ReceiptRefused)
        self.assertEqual(str(primary), "SCM types differ from the original window")
        self.assertIs(cleanup, retirement)
        self.assertEqual(window.capture_count, 2)
        self.assert_retired(window)

    def test_non_dword_or_noncanonical_baselines_refuse_before_capture(self) -> None:
        class ForeignPair(tuple):
            def __len__(self) -> int:
                raise AssertionError("foreign tuple hook must not run")

        for sample in (
            None,
            ScmTypeSample(engine=(528, True), broker=(528, 16)),
            ScmTypeSample(engine=(528, -1), broker=(528, 16)),
            ScmTypeSample(engine=(528, 0x100000000), broker=(528, 16)),
            ScmTypeSample(engine=(528,), broker=(528, 16)),
            ScmTypeSample(engine=ForeignPair((528, 528)), broker=(528, 16)),
        ):
            with self.subTest(sample=sample):
                window: ScriptedWindow = self.window()
                with patch.object(window, "initial_scm_types", return_value=sample) as sample_method:
                    with self.assertRaisesRegex(ReceiptRefused, "invalid original SCM type sample"):
                        supervise_prestart(window)
                    sample_method.assert_called_once_with(window.run)
                self.assertEqual(window.capture_count, 0)
                self.assertEqual(window.checks, 0)
                self.assert_retired(window)

    def test_provisioning_authority_refusal_skips_standard_user_capture(self) -> None:
        window = self.window()
        # Keep the child's exit and custody apparently healthy to reach the real reader.
        refused = self.fixture.access.stdout.replace(b"opened-and-closed", b"access-denied", 1)
        window.captures[2] = replace(self.fixture.access, stdout=refused)
        with self.assertRaisesRegex(ReceiptRefused, "provisioning access rows differ"):
            supervise_prestart(window)
        self.assertEqual(window.capture_count, 3)
        self.assert_retired(window)
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
                self.assert_retired(window)

    def test_equal_but_different_window_refuses_before_and_after_capture(self) -> None:
        for check in range(1, 6):
            with self.subTest(check=check):
                window = self.window()
                window.at_check[check] = replace(window.run)
                with self.assertRaises(ReceiptRefused):
                    supervise_prestart(window)
                self.assertEqual(window.capture_count, check - 1)
                self.assertEqual(window.checks, check)
                self.assert_retired(window)

    def test_first_adapter_refusal_keeps_its_cause_and_consumes_the_attempt(self) -> None:
        window = self.window()
        first = ReceiptRefused("scripted capture cleanup remains unresolved")
        window.captures[0] = first
        # A later window check must not replace the original adapter refusal.
        window.at_check[2] = ReceiptRefused("scripted later custody failure")
        with self.assertRaises(ReceiptRefused) as caught:
            supervise_prestart(window)
        self.assertIs(caught.exception, first)
        self.assertEqual(window.calls, [
            ("take",), ("scm-sample", window.run), ("current",), ("identity", window.run), ("retire", window.run),
        ])

        with self.assertRaises(ReceiptRefused) as caught:
            supervise_prestart(window)
        self.assertIs(caught.exception, window.exhausted)
        self.assertEqual(window.calls[-1], ("take",))
        self.assertEqual(window.capture_count, 1)
        self.assertEqual([call[0] for call in window.calls].count("retire"), 1)

    def test_failed_child_receipt_precedes_a_later_window_refusal(self) -> None:
        window = self.window()
        window.captures[0] = replace(self.fixture.identity, exit_code=1)
        window.at_check[2] = ReceiptRefused("scripted later window withdrawal")
        with self.assertRaisesRegex(ReceiptRefused, "child did not exit successfully and reap"):
            supervise_prestart(window)
        self.assertEqual(window.capture_count, 1)
        self.assertEqual(window.checks, 1)
        self.assert_retired(window)

    def test_custody_checker_refusal_propagates_at_publication(self) -> None:
        window = self.window()
        refused = ReceiptRefused("scripted custody lost during final publication check")
        window.at_check[5] = refused
        with self.assertRaises(ReceiptRefused) as caught:
            supervise_prestart(window)
        self.assertIs(caught.exception, refused)
        self.assertEqual(window.capture_count, 4)
        self.assert_retired(window)

    def test_failed_admission_keeps_partial_rollback_with_the_owner(self) -> None:
        window = self.window()
        refusal = ReceiptRefused("scripted admission refused")
        window.admission_failure = refusal
        with self.assertRaises(ReceiptRefused) as caught:
            supervise_prestart(window)
        self.assertIs(caught.exception, refusal)
        self.assertEqual(window.calls, [("take",)])

    def test_invalid_issued_run_is_retired_before_any_capture(self) -> None:
        window = self.window()
        window.run = replace(window.run, id=UUID(int=0))
        with self.assertRaisesRegex(ReceiptRefused, "invalid supervisor run"):
            supervise_prestart(window)
        self.assertEqual(window.calls, [("take",), ("retire", window.run)])
        self.assert_retired(window)

    def test_each_receipt_refusal_stops_and_retires_the_run(self) -> None:
        captures = (self.fixture.identity, self.fixture.observation, self.fixture.access, self.fixture.standard)
        for index, capture in enumerate(captures):
            with self.subTest(capture=index):
                window = self.window()
                window.captures[index] = replace(capture, reaped=False)
                with self.assertRaisesRegex(ReceiptRefused, "child did not exit successfully and reap"):
                    supervise_prestart(window)
                self.assertEqual(window.capture_count, index + 1)
                self.assertEqual(window.checks, index + 1)
                self.assert_retired(window)

    def test_each_interrupted_capture_retires_the_run_then_propagates_the_interrupt(self) -> None:
        for index in range(4):
            with self.subTest(capture=index):
                window = self.window()
                interrupt = KeyboardInterrupt()
                window.captures[index] = interrupt
                with self.assertRaises(KeyboardInterrupt) as caught:
                    supervise_prestart(window)
                self.assertIs(caught.exception, interrupt)
                self.assertEqual(window.capture_count, index + 1)
                self.assert_retired(window)

    def test_refused_retirement_withholds_an_otherwise_valid_observation(self) -> None:
        window = self.window()
        refusal = ReceiptRefused("scripted SCM close unconfirmed")
        window.retirement_failure = refusal
        with self.assertRaises(ReceiptRefused) as caught:
            supervise_prestart(window)
        self.assertIs(caught.exception, refusal)
        self.assertEqual(window.capture_count, 4)
        self.assertEqual(window.checks, 5)
        self.assert_retired(window)

    def test_retirement_failure_follows_a_receipt_refusal(self) -> None:
        window = self.window()
        window.captures[0] = replace(self.fixture.identity, reaped=False)
        retirement = ReceiptRefused("scripted SCM close unconfirmed")
        window.retirement_failure = retirement
        with self.assertRaises(BaseExceptionGroup) as caught:
            supervise_prestart(window)
        first, second = caught.exception.exceptions
        self.assertIsInstance(first, ReceiptRefused)
        self.assertEqual(str(first), "child did not exit successfully and reap")
        self.assertIs(second, retirement)
        self.assertEqual(window.capture_count, 1)
        self.assertEqual(window.checks, 1)
        self.assert_retired(window)

    def test_retirement_failure_follows_the_original_interrupt(self) -> None:
        window = self.window()
        interrupt = KeyboardInterrupt()
        retirement = ReceiptRefused("scripted SCM close unconfirmed")
        window.captures[2] = interrupt
        window.retirement_failure = retirement
        with self.assertRaises(BaseExceptionGroup) as caught:
            supervise_prestart(window)
        first, second = caught.exception.exceptions
        self.assertIs(first, interrupt)
        self.assertIs(second, retirement)
        self.assertEqual(window.capture_count, 3)
        self.assert_retired(window)

    def test_retirement_rethrowing_the_original_failure_does_not_duplicate_it(self) -> None:
        window = self.window()
        refusal = ReceiptRefused("scripted custody loss")
        window.at_check[5] = refusal
        window.retirement_failure = refusal
        with self.assertRaises(ReceiptRefused) as caught:
            supervise_prestart(window)
        self.assertIs(caught.exception, refusal)
        self.assertEqual(window.capture_count, 4)
        self.assert_retired(window)

    def test_returned_retirement_error_is_propagated(self) -> None:
        window: ScriptedWindow = self.window()
        failure: OSError = OSError("reported original handle closure failure")
        with patch.object(window, "retire_run", return_value=failure) as retire:
            with self.assertRaises(OSError) as caught:
                supervise_prestart(window)
        self.assertIs(caught.exception, failure)
        retire.assert_called_once_with(window.run)
        self.assertEqual(window.capture_count, 4)

    def test_non_exception_retirement_result_is_refused(self) -> None:
        window: ScriptedWindow = self.window()
        with patch.object(window, "retire_run", return_value=False) as retire:
            with self.assertRaisesRegex(ReceiptRefused, "retirement returned an invalid outcome"):
                supervise_prestart(window)
        retire.assert_called_once_with(window.run)
        self.assertEqual(window.capture_count, 4)
