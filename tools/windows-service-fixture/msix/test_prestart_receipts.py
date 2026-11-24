# SPDX-License-Identifier: GPL-2.0-only
"""Exercise the actual join with inert process and signed-artifact observations."""

from dataclasses import replace
import json
import unittest
from uuid import UUID

from prestart_receipts import (
    Capture, DeploymentObservation, DeploymentRun, Member, PrestartProbe, ReceiptRefused, Run,
    ScmTypeSample, join_prestart, read_deployment, read_prestart,
)


ENGINE = "NevroTick-Lab-S02-Engine"
BROKER = "NevroTick-Lab-S02-Broker"
READ = "scm_observation_tests::native_installed_roles_prestart_observation"
ACCESS = "scm_access_probe::windows::native_installed_roles_prestart_access"
ORDINARY = "scm_access_probe::windows::native_standard_user_prestart_change_config_refused"


def libtest(test, lines):
    """Render the existing single, exact, uncaptured `libtest` transcript shape."""
    rows = "\r\n".join(lines)
    return (f"\r\nrunning 1 test\r\ntest {test} ... {rows}\r\nok\r\n\r\n"
            "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 195 filtered out; "
            "finished in 0.00s\r\n\r\n").encode("ascii")


class PrestartReceiptTests(unittest.TestCase):
    def setUp(self):
        self.run = Run(UUID(int=123), "NevroTick.S06.ScmFixture_1.0.0.0_x64__0123456789abc", "1" * 64,
                       Member(125, "2" * 64), Member(256, "3" * 64), "4" * 64, "5" * 64,
                       object(), object(), object())
        self.report = {
            "schema_version": 2,
            "outcome": "observed-installed-images",
            "run_id": str(self.run.id),
            "signed_package_sha256": "1" * 64,
            "package_full_name": "NevroTick.S06.ScmFixture_1.0.0.0_x64__0123456789abc",
            "roles": [
                {"key": ENGINE, "member": "nevrotick-lab-s02-engine.exe", "bytes": 125, "sha256": "2" * 64},
                {"key": BROKER, "member": "nevrotick-lab-s02-broker.exe", "bytes": 256, "sha256": "3" * 64},
            ],
        }
        self.role_rows = [
            f"scm-prestart {ENGINE} configured-type=528 start=4 error=1 before-type=528 before-state=1 "
            "after-type=528 after-state=1 sid-type=0 path-units=Some(135) account-units=Some(24) "
            "privilege-units=None security-control=32772 owner-bytes=12 dacl=present dacl-bytes=140",
            f"scm-prestart {BROKER} configured-type=528 start=4 error=1 before-type=16 before-state=1 "
            "after-type=16 after-state=1 sid-type=0 path-units=Some(135) account-units=Some(24) "
            "privilege-units=Some(14) security-control=32768 owner-bytes=12 dacl=absent dacl-bytes=0",
            "scm-prestart observed-rows=2",
        ]
        self.access_rows = [*self.role_rows, "scm-prestart-access invoking-pid=303"]
        for role in (ENGINE, BROKER):
            for mask in ("0x4", "0x1", "0x20000", "0x2", "0x10", "0x20"):
                self.access_rows.append(f"scm-prestart-access {role} mask={mask} opened-and-closed")
        self.access_rows.append("scm-prestart-access observed-rows=12")
        self.standard_rows = [
            "scm-prestart-standard-user invoking-pid=404",
            f"scm-prestart-standard-user {ENGINE} status-opened-and-closed configuration-denied code=5",
            f"scm-prestart-standard-user {BROKER} status-opened-and-closed configuration-denied code=5",
            "scm-prestart-standard-user observed-rows=2",
        ]
        self.identity = self.capture(101, self.run.deployment_actor, "4" * 64, self.identity_bytes())
        self.observation = self.capture(202, self.run.deployment_actor, "5" * 64, libtest(READ, self.role_rows))
        self.access = self.capture(303, self.run.provisioning_actor, "5" * 64, libtest(ACCESS, self.access_rows))
        self.standard = self.capture(404, self.run.standard_actor, "5" * 64, libtest(ORDINARY, self.standard_rows))

    def identity_bytes(self):
        return json.dumps(self.report).encode("ascii")

    def capture(self, pid, actor, image, output):
        return Capture(self.run.id, actor, pid, image, image, 0, True, output, b"")

    def join(self, **override):
        arguments = dict(run=self.run, identity=self.identity, observation=self.observation,
                         access=self.access, standard=self.standard)
        return join_prestart(**(arguments | override))

    def test_joins_exact_observations_without_normalizing_raw_service_types(self):
        result = self.join()
        self.assertEqual(result.run_id, self.run.id)
        self.assertEqual(result.signed_package_sha256, "1" * 64)
        self.assertEqual(result.service_types, ((528, 528, 528), (528, 16, 16)))

    def deployment_run(self) -> DeploymentRun:
        return DeploymentRun(self.run.id, self.run.package_full_name, self.run.signed_package_sha256,
                             self.run.engine, self.run.broker, self.run.companion_sha256,
                             self.run.observer_sha256, self.run.deployment_actor)

    def test_deployment_only_reads_the_two_fixed_captures(self) -> None:
        run: DeploymentRun = self.deployment_run()
        baseline: ScmTypeSample = ScmTypeSample(engine=(528, 528), broker=(528, 16))
        order: list[PrestartProbe] = []
        captures: dict[PrestartProbe, Capture] = {
            PrestartProbe.IDENTITY: self.identity, PrestartProbe.OBSERVATION: self.observation,
        }

        def collect(probe: PrestartProbe) -> Capture:
            order.append(probe)
            return captures[probe]

        result: DeploymentObservation = read_deployment(run, collect, baseline)
        self.assertEqual(result, DeploymentObservation(self.run.id, "1" * 64, ((528, 528, 528), (528, 16, 16))))
        self.assertEqual(order, [PrestartProbe.IDENTITY, PrestartProbe.OBSERVATION])

        order.clear()
        with self.assertRaisesRegex(ReceiptRefused, "actor contexts are incomplete"):
            read_prestart(run, collect, baseline)
        self.assertEqual(order, [])

    def test_deployment_identity_refusal_prevents_the_observer(self) -> None:
        run: DeploymentRun = self.deployment_run()
        baseline: ScmTypeSample = ScmTypeSample(engine=(528, 528), broker=(528, 16))
        order: list[PrestartProbe] = []
        wrong: bytes = self.identity.stdout.replace(b'"sha256": "' + b"2" * 64, b'"sha256": "' + b"9" * 64, 1)
        self.assertNotEqual(wrong, self.identity.stdout)

        def collect(probe: PrestartProbe) -> Capture:
            order.append(probe)
            self.assertIs(probe, PrestartProbe.IDENTITY)
            return replace(self.identity, stdout=wrong)

        with self.assertRaisesRegex(ReceiptRefused, "identity differs"):
            read_deployment(run, collect, baseline)
        self.assertEqual(order, [PrestartProbe.IDENTITY])

    def test_deployment_requires_original_scm_sample(self) -> None:
        run: DeploymentRun = self.deployment_run()
        malformed: ScmTypeSample = ScmTypeSample(engine=(528, 528), broker=(528, True))

        def unexpected(probe: PrestartProbe) -> Capture:
            self.fail(f"invalid baseline dispatched {probe}")

        with self.assertRaisesRegex(ReceiptRefused, "invalid original SCM type sample"):
            read_deployment(run, unexpected, malformed)

        forged_row: str = self.role_rows[1].replace("before-type=16", "before-type=32")
        forged: Capture = replace(self.observation, stdout=libtest(READ, [self.role_rows[0], forged_row, self.role_rows[2]]))
        baseline: ScmTypeSample = ScmTypeSample(engine=(528, 528), broker=(528, 16))
        captures: dict[PrestartProbe, Capture] = {
            PrestartProbe.IDENTITY: self.identity, PrestartProbe.OBSERVATION: forged,
        }
        with self.assertRaisesRegex(ReceiptRefused, "SCM types differ from the original window"):
            read_deployment(run, captures.__getitem__, baseline)

    def test_supervised_child_custody_has_no_stdout_fallback(self):
        for change in (
            {"run_id": UUID(int=124)}, {"actor": self.run.standard_actor}, {"pid": 304},
            {"input_sha256_before": "0" * 64}, {"input_sha256_after": "0" * 64},
            {"exit_code": 1}, {"exit_code": False}, {"reaped": False}, {"stderr": b"warning"},
            {"stdout": b"x" * 8193},
        ):
            with self.subTest(change=next(iter(change))), self.assertRaises(ReceiptRefused):
                self.join(access=replace(self.access, **change))
        with self.assertRaisesRegex(ReceiptRefused, "conflated"):
            self.join(run=replace(self.run, standard_actor=self.run.provisioning_actor))
        self.join()

    def test_identity_rejects_valid_shaped_different_release_and_members(self):
        for key, wrong in (
            ("package_full_name", "NevroTick.S06.ScmFixture_1.0.0.0_x64__fedcba9876543"),
            ("run_id", str(UUID(int=124))), ("signed_package_sha256", "9" * 64),
            ("roles", list(reversed(self.report["roles"]))), ("schema_version", 1), ("schema_version", 2.0),
        ):
            with self.subTest(key=key), self.assertRaisesRegex(ReceiptRefused, "identity differs"):
                supplied = json.dumps(self.report | {key: wrong}).encode()
                self.join(identity=replace(self.identity, stdout=supplied))
        wrong_member = self.report["roles"][0] | {"sha256": "f" * 64}
        supplied = json.dumps(self.report | {"roles": [wrong_member, self.report["roles"][1]]}).encode()
        with self.assertRaisesRegex(ReceiptRefused, "identity differs"):
            self.join(identity=replace(self.identity, stdout=supplied))
        duplicated = self.identity_bytes().replace(b'"schema_version": 2', b'"schema_version": 2, "schema_version": 2')
        with self.assertRaisesRegex(ReceiptRefused, "malformed installed identity"):
            self.join(identity=replace(self.identity, stdout=duplicated))
        self.join()

    def test_different_libtest_outcomes_and_discovery_only_refuse(self):
        for wrong in (
            f"{READ}: test\n\n1 test, 0 benchmarks\n".encode(),
            libtest(ACCESS, self.role_rows),
            self.observation.stdout.removesuffix(b"\r\n"),
            self.observation.stdout.replace(b"1 passed; 0 failed", b"0 passed; 1 failed"),
            self.observation.stdout + b"scm-prestart observed-rows=2\n",
        ):
            with self.subTest(wrong=wrong[:30]), self.assertRaisesRegex(ReceiptRefused, "unexpected selector"):
                self.join(observation=replace(self.observation, stdout=wrong))
        self.join()

    def test_prestart_fields_and_order_require_real_complete_rows(self):
        for wrong in (
            [self.role_rows[1], self.role_rows[0], self.role_rows[2]],
            [self.role_rows[0].replace("after-state=1", "after-state=4"), *self.role_rows[1:]],
            [self.role_rows[0].replace("start=4", "start=3"), *self.role_rows[1:]],
            [self.role_rows[0].replace("dacl-bytes=140", "dacl-bytes=0"), *self.role_rows[1:]],
            [self.role_rows[0].replace("before-type=528", "before-type=4294967296"), *self.role_rows[1:]],
        ):
            with self.subTest(wrong=wrong[0]), self.assertRaises(ReceiptRefused):
                self.join(observation=replace(self.observation, stdout=libtest(READ, wrong)))
        self.join()

    def test_access_requires_original_observation_order_and_closes(self):
        for index, mutated in (
            (0, self.role_rows[0].replace("sid-type=0", "sid-type=1")),
            (4, self.access_rows[5]),
            (5, self.access_rows[5].replace("opened-and-closed", "close-failed")),
            (16, "scm-prestart-access observed-rows=11"),
        ):
            wrong = self.access_rows.copy()
            wrong[index] = mutated
            with self.subTest(index=index), self.assertRaisesRegex(ReceiptRefused, "access rows differ"):
                self.join(access=replace(self.access, stdout=libtest(ACCESS, wrong)))
        wrong = self.standard_rows.copy()
        wrong[2] = wrong[2].replace("code=5", "code=1060")
        with self.assertRaisesRegex(ReceiptRefused, "standard-user access or denial"):
            self.join(standard=replace(self.standard, stdout=libtest(ORDINARY, wrong)))
        self.join()

    def test_agreeing_impossible_descriptor_projections_refuse(self):
        for field, invalid in (
            ("security-control=32772", "security-control=4"),
            ("security-control=32772", "security-control=32768"),
            ("security-control=32772", "security-control=98308"),
            ("owner-bytes=12", "owner-bytes=9"),
            ("dacl-bytes=140", "dacl-bytes=9"),
            ("dacl-bytes=140", "dacl-bytes=8180"),
        ):
            changed = self.role_rows[0].replace(field, invalid)
            observation = libtest(READ, [changed, *self.role_rows[1:]])
            access = libtest(ACCESS, [changed, *self.access_rows[1:]])
            with self.subTest(invalid=invalid), self.assertRaisesRegex(ReceiptRefused, "invalid SCM"):
                self.join(observation=replace(self.observation, stdout=observation),
                          access=replace(self.access, stdout=access))
        self.join()

    def join_with_engine_extent(self, before, after):
        row = self.role_rows[0].replace(before, after)
        observation = libtest(READ, [row, *self.role_rows[1:]])
        access = libtest(ACCESS, [row, *self.access_rows[1:]])
        return self.join(observation=replace(self.observation, stdout=observation),
                         access=replace(self.access, stdout=access))

    def test_agreeing_impossible_configuration_extents_refuse(self):
        for before, after in (
            ("path-units=Some(135)", "path-units=Some(4064)"),
            ("account-units=Some(24)", "account-units=Some(4064)"),
            ("privilege-units=None", "privilege-units=Some(0)"),
            ("privilege-units=None", "privilege-units=Some(2)"),
            ("privilege-units=None", "privilege-units=Some(4093)"),
        ):
            with self.subTest(field=after), self.assertRaisesRegex(ReceiptRefused, "invalid SCM"):
                self.join_with_engine_extent(before, after)

    def test_configuration_extent_edges_and_empty_forms_join(self):
        for before, after in (
            ("path-units=Some(135)", "path-units=None"),
            ("path-units=Some(135)", "path-units=Some(4062)"),
            ("path-units=Some(135)", "path-units=Some(4063)"),
            ("account-units=Some(24)", "account-units=Some(0)"),
            ("account-units=Some(24)", "account-units=Some(4063)"),
            ("privilege-units=None", "privilege-units=Some(1)"),
            ("privilege-units=None", "privilege-units=Some(3)"),
            ("privilege-units=None", "privilege-units=Some(4091)"),
            ("privilege-units=None", "privilege-units=Some(4092)"),
        ):
            with self.subTest(field=after):
                self.assertEqual(self.join_with_engine_extent(before, after).service_types,
                                 ((528, 528, 528), (528, 16, 16)))
