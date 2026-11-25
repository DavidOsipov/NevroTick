# SPDX-License-Identifier: GPL-2.0-only
"""Independent literal and refusal controls for the fixed deployment renderer."""

from collections.abc import Iterator, Mapping
from dataclasses import FrozenInstanceError, replace
from hashlib import sha256
from pathlib import Path
from tempfile import TemporaryDirectory
import unittest
from uuid import UUID

from prestart_deployment_owner import RenderedAccessOwner, render_deployment_owner, render_prestart_access_owner
from prestart_receipts import DeploymentRun, Member, PrestartProbe, ReceiptRefused, Run


class ChangedPin(Mapping[str, Member]):
    """Supply a different valid-looking pin after the first observation."""

    def __init__(self, pins: dict[str, Member], label: str = "archive") -> None:
        self.pins = pins
        self.label = label
        self.reads: int = 0

    def __getitem__(self, label: str) -> Member:
        value: Member = self.pins[label]
        if label == self.label:
            self.reads += 1
            if self.reads >= 3:
                return Member(value.size, "d" * 64)
        return value

    def __iter__(self) -> Iterator[str]:
        return iter(self.pins)

    def __len__(self) -> int:
        return len(self.pins)


class DeploymentOwnerRenderTests(unittest.TestCase):
    def setUp(self) -> None:
        self.directory: TemporaryDirectory[str] = TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.access_stage: Path = Path(self.directory.name) / "access-stage"
        self.template: Path = Path(self.directory.name) / "prestart_deployment_owner.ps1.in"
        self.original: bytes = Path(__file__).with_name(self.template.name).read_bytes()
        self.template.write_bytes(self.original)
        self.template_pin: Member = Member(len(self.original), sha256(self.original).hexdigest())
        self.pins: dict[str, Member] = {
            "interpreter": Member(111, "1" * 64),
            "child_module": Member(112, "2" * 64),
            "session_module": Member(113, "3" * 64),
            "deployment_helper": Member(114, "4" * 64),
            "archive": Member(115, "5" * 64),
            "identity_entry": Member(116, "6" * 64),
            "identity_module": Member(117, "7" * 64),
            "identity_helper": Member(118, "8" * 64),
            "observer": Member(119, "9" * 64),
        }
        self.run: DeploymentRun = DeploymentRun(
            UUID("e43449b8-3be7-49a2-90df-6451ef7cdfb3"),
            "NevroTick.S06.ScmFixture_1.0.0.0_x64__0123456789abc",
            "5" * 64, Member(7, "e" * 64), Member(8, "f" * 64), "7" * 64, "9" * 64, object(),
        )
        self.sid: str = "S-1-5-21-1337-8128-9999-1001"
        self.signer: str = "a" * 64
        self.payload_sha256: str = "0123456789abcdef" * 4

    def render(self) -> tuple[bytes, Member]:
        return render_deployment_owner(
            self.template, self.template_pin, self.run, self.sid, self.signer, self.pins, 8000, 12000,
            expected_payload_sha256=self.payload_sha256,
        )

    def test_exact_literal_boundary_and_watchdog_pin(self) -> None:
        entry: bytes
        pin: Member
        entry, pin = self.render()
        prefix: bytes
        delimiter: bytes
        suffix: bytes
        prefix, delimiter, suffix = self.original.partition(b"# PRESTART_DEPLOYMENT_BINDINGS\n")
        self.assertTrue(delimiter)
        self.assertTrue(entry.startswith(prefix))
        self.assertTrue(entry.endswith(suffix))
        self.assertNotIn(delimiter, entry)
        self.assertTrue(entry.isascii())
        middle: str = entry[len(prefix):len(entry) - len(suffix)].decode("ascii")
        expected: list[str] = [
            "[guid]$expectedRunId = 'e43449b8-3be7-49a2-90df-6451ef7cdfb3'",
            "[string]$ownerRole = 'deployment'",
            f"[string]$expectedActorSid = '{self.sid}'",
            f"[string]$signerSha256 = '{self.signer}'",
            f"[string]$expectedPayloadSha256 = '{self.payload_sha256}'",
            "[int]$workMilliseconds = 8000",
            "[int]$retirementMilliseconds = 12000",
            "[bool]$fullPrestart = $false",
            "[hashtable]$pins = @{",
        ]
        for key, expected_member in self.pins.items():
            expected.append(f"    {key} = @{{ Bytes = [long]{expected_member.size}; Sha256 = '{expected_member.sha256}' }}")
        expected.append("}")
        self.assertEqual(middle.splitlines(), expected)
        self.assertEqual(pin, Member(len(entry), sha256(entry).hexdigest()))
        self.pins = dict(reversed(tuple(self.pins.items())))
        self.assertEqual(self.render(), (entry, pin))
        self.assertEqual(self.template.read_bytes(), self.original)

    def test_full_mode_is_bound_to_the_admitted_run_in_the_rendered_entry(self) -> None:
        with self.assertRaisesRegex(ReceiptRefused, "full pre-start requires both admitted access actors"):
            render_deployment_owner(
                self.template, self.template_pin, self.run, self.sid, self.signer, self.pins, 8000, 12000,
                expected_payload_sha256=self.payload_sha256,
                full_prestart=True,
            )
        run: Run = Run(**vars(self.run), provisioning_actor=object(), standard_actor=object())
        entry: bytes
        pin: Member
        entry, pin = render_deployment_owner(
            self.template, self.template_pin, run, self.sid, self.signer, self.pins, 8000, 12000,
            expected_payload_sha256=self.payload_sha256,
            full_prestart=True,
        )
        self.assertEqual(entry.count(b"[bool]$fullPrestart = $true\n"), 1)
        self.assertIn(b"-FullPrestart:$fullPrestart", entry)
        self.assertEqual(pin, Member(len(entry), sha256(entry).hexdigest()))

    def access_pins(self) -> dict[str, Member]:
        return {key: self.pins[key] for key in ("child_module", "session_module", "deployment_helper", "observer")}

    def test_both_access_roles_render_only_fixed_literal_and_member_pins(self) -> None:
        prefix, delimiter, suffix = self.original.partition(b"# PRESTART_DEPLOYMENT_BINDINGS\n")
        self.assertTrue(delimiter)
        self.assertIn(b"param([Parameter(Mandatory)][guid]$RunId)", prefix)
        for probe, role in ((PrestartProbe.ACCESS, "access"), (PrestartProbe.STANDARD, "standard")):
            with self.subTest(probe=probe):
                pins: dict[str, Member] = self.access_pins()
                stage: Path = self.access_stage / role
                rendered: RenderedAccessOwner = render_prestart_access_owner(
                    self.template, self.template_pin, self.run, probe, self.sid, pins, 8000, stage=stage,
                )
                self.assertIs(rendered.run, self.run)
                self.assertIs(rendered.probe, probe)
                self.assertEqual(rendered.expected_actor_sid, self.sid)
                self.assertEqual(rendered.path, stage / "prestart_deployment_owner.ps1")
                self.assertFalse(stage.exists())
                self.assertNotIn(self.sid, repr(rendered))
                self.assertNotIn("contents=", repr(rendered))
                with self.assertRaises(FrozenInstanceError):
                    setattr(rendered, "path", self.access_stage)
                entry: bytes = rendered.contents
                self.assertTrue(entry.startswith(prefix))
                self.assertTrue(entry.endswith(suffix))
                middle: str = entry[len(prefix):len(entry) - len(suffix)].decode("ascii")
                expected: list[str] = [
                    "[guid]$expectedRunId = 'e43449b8-3be7-49a2-90df-6451ef7cdfb3'",
                    f"[string]$ownerRole = '{role}'",
                    f"[string]$expectedActorSid = '{self.sid}'",
                    "[int]$workMilliseconds = 8000",
                    "[bool]$privateCaptureDiagnostics = $false",
                    "[hashtable]$pins = @{",
                ]
                for key, member in pins.items():
                    expected.append(f"    {key} = @{{ Bytes = [long]{member.size}; Sha256 = '{member.sha256}' }}")
                self.assertEqual(middle.splitlines(), [*expected, "}"])
                self.assertEqual(rendered.pin, Member(len(entry), sha256(entry).hexdigest()))
                self.assertEqual(render_prestart_access_owner(
                    self.template, self.template_pin, self.run, probe, self.sid,
                    dict(reversed(tuple(pins.items()))), 8000, stage=stage,
                ), rendered)
        self.assertEqual(self.template.read_bytes(), self.original)

    def test_private_capture_diagnostics_must_be_selected_in_the_pinned_access_entry(self) -> None:
        rendered: RenderedAccessOwner = render_prestart_access_owner(
            self.template, self.template_pin, self.run, PrestartProbe.ACCESS, self.sid, self.access_pins(), 8000,
            stage=self.access_stage, private_capture_diagnostics=True,
        )
        self.assertEqual(rendered.contents.count(b"[bool]$privateCaptureDiagnostics = $true\n"), 1)
        self.assertEqual(rendered.pin, Member(len(rendered.contents), sha256(rendered.contents).hexdigest()))
        with self.assertRaisesRegex(ReceiptRefused, "invalid pre-start access capture diagnostics selection"):
            render_prestart_access_owner(
                self.template, self.template_pin, self.run, PrestartProbe.ACCESS, self.sid, self.access_pins(), 8000,
                stage=self.access_stage,
                private_capture_diagnostics=1,
            )

    def test_access_role_rejects_other_probes_as_well_as_role_aliases(self) -> None:
        for probe in (PrestartProbe.IDENTITY, PrestartProbe.OBSERVATION, PrestartProbe.ACCESS.value,
                      "access", "standard", None):
            with self.subTest(probe=probe), self.assertRaisesRegex(ReceiptRefused, "invalid pre-start access probe"):
                render_prestart_access_owner(
                    self.template, self.template_pin, self.run, probe, self.sid, self.access_pins(), 8000,
                    stage=self.access_stage,
                )

    def test_access_role_refuses_foreign_run_sid_and_deadline_literals(self) -> None:
        with self.assertRaisesRegex(ReceiptRefused, "invalid pre-start access owner run"):
            render_prestart_access_owner(
                self.template, self.template_pin, replace(self.run, id=UUID(int=0)),
                PrestartProbe.ACCESS, self.sid, self.access_pins(), 8000,
                stage=self.access_stage,
            )
        for sid in ("S-1-5-21-1'; throw 'injected", "S-1-05-1", "S-1-5" + "-1" * 16):
            with self.subTest(sid=sid), self.assertRaisesRegex(ReceiptRefused, "invalid pre-start access owner SID"):
                render_prestart_access_owner(
                    self.template, self.template_pin, self.run, PrestartProbe.STANDARD, sid,
                    self.access_pins(), 8000,
                    stage=self.access_stage,
                )
        for work in (True, 0, -1, 120001):
            with self.subTest(work=work), self.assertRaisesRegex(ReceiptRefused, "invalid pre-start access owner deadline"):
                render_prestart_access_owner(
                    self.template, self.template_pin, self.run, PrestartProbe.ACCESS, self.sid,
                    self.access_pins(), work,
                    stage=self.access_stage,
                )
        for work in (1, 120000):
            rendered: RenderedAccessOwner = render_prestart_access_owner(
                self.template, self.template_pin, self.run, PrestartProbe.ACCESS, self.sid, self.access_pins(), work,
                stage=self.access_stage,
            )
            self.assertIn(f"[int]$workMilliseconds = {work}\n".encode("ascii"), rendered.contents)

    def test_access_role_checks_exact_pin_roster_observer_and_one_snapshot(self) -> None:
        pins: dict[str, Member] = self.access_pins()
        for foreign in ({key: pin for key, pin in pins.items() if key != "child_module"}, self.pins):
            with self.assertRaisesRegex(ReceiptRefused, "pin roster"):
                render_prestart_access_owner(
                    self.template, self.template_pin, self.run, PrestartProbe.ACCESS, self.sid, foreign, 8000,
                    stage=self.access_stage,
                )
        with self.assertRaisesRegex(ReceiptRefused, "observer pin differs from the admitted run"):
            render_prestart_access_owner(
                self.template, self.template_pin, self.run, PrestartProbe.STANDARD, self.sid,
                {**pins, "observer": Member(119, "d" * 64)}, 8000,
                stage=self.access_stage,
            )
        with self.assertRaisesRegex(ReceiptRefused, "pin: deployment_helper"):
            render_prestart_access_owner(
                self.template, self.template_pin, self.run, PrestartProbe.STANDARD, self.sid,
                {**pins, "deployment_helper": Member(2097153, "4" * 64)}, 8000,
                stage=self.access_stage,
            )
        changing: ChangedPin = ChangedPin(pins, "observer")
        rendered: RenderedAccessOwner = render_prestart_access_owner(
            self.template, self.template_pin, self.run, PrestartProbe.ACCESS, self.sid, changing, 8000,
            stage=self.access_stage,
        )
        self.assertEqual(changing.reads, 1)
        self.assertIn(b"observer = @{ Bytes = [long]119; Sha256 = '" + b"9" * 64 + b"'", rendered.contents)

    def test_access_owner_requires_an_absolute_stage(self) -> None:
        with self.assertRaisesRegex(ReceiptRefused, "invalid pre-start access staging directory"):
            render_prestart_access_owner(
                self.template, self.template_pin, self.run, PrestartProbe.ACCESS, self.sid, self.access_pins(), 8000,
                stage=Path("relative/stage"),
            )

    def test_access_owner_requires_the_independent_template_pin(self) -> None:
        with self.assertRaisesRegex(ReceiptRefused, "deployment owner template differs"):
            render_prestart_access_owner(
                self.template, Member(len(self.original), "0" * 64), self.run, PrestartProbe.ACCESS,
                self.sid, self.access_pins(), 8000, stage=self.access_stage,
            )

    def test_crossed_pins_refuse_even_if_individually_well_formed(self) -> None:
        original: dict[str, Member] = self.pins.copy()
        for role in ("archive", "identity_module", "observer"):
            with self.subTest(role=role):
                self.pins = {**original, role: Member(original[role].size, "c" * 64)}
                with self.assertRaisesRegex(ReceiptRefused, "pins differ from the admitted run"):
                    self.render()

    def test_render_uses_the_same_pin_values_it_validates(self) -> None:
        changing: ChangedPin = ChangedPin(self.pins)
        result: bytes = render_deployment_owner(
            self.template, self.template_pin, self.run, self.sid, self.signer, changing, 8000, 12000,
            expected_payload_sha256=self.payload_sha256,
        )[0]
        self.assertIn(b"archive = @{ Bytes = [long]115; Sha256 = '" + b"5" * 64 + b"'", result)
        self.assertEqual(changing.reads, 1)

    def test_missing_extra_and_malformed_member_pins_refuse(self) -> None:
        original: dict[str, Member] = self.pins.copy()
        del self.pins["archive"]
        with self.assertRaisesRegex(ReceiptRefused, "pin roster"):
            self.render()
        self.pins = {**original, "commands": Member(1, "1" * 64)}
        with self.assertRaisesRegex(ReceiptRefused, "pin roster"):
            self.render()
        for member in (Member(True, "8" * 64), Member(65537, "8" * 64), Member(1, "o" * 64)):
            with self.subTest(member=member):
                self.pins = {**original, "identity_helper": member}
                with self.assertRaisesRegex(ReceiptRefused, "pin: identity_helper"):
                    self.render()
        self.pins = {**original, "identity_helper": Member(65536, "8" * 64)}
        self.assertIn(b"Bytes = [long]65536", self.render()[0])

    def test_run_identity_sid_and_signer_are_literal_only(self) -> None:
        original_run: DeploymentRun = self.run
        self.run = replace(original_run, id=UUID(int=0))
        with self.assertRaisesRegex(ReceiptRefused, "invalid deployment owner run"):
            self.render()
        self.run = original_run
        original_sid: str = self.sid
        self.sid = "S-1-5-21-1'; throw 'injected"
        with self.assertRaisesRegex(ReceiptRefused, "invalid deployment owner SID"):
            self.render()
        self.sid = original_sid
        self.signer = "A" * 64
        with self.assertRaisesRegex(ReceiptRefused, "invalid deployment owner signer digest"):
            self.render()

    def test_payload_digest_is_required_and_literal_only(self) -> None:
        with self.assertRaisesRegex(TypeError, "expected_payload_sha256"):
            render_deployment_owner(
                self.template, self.template_pin, self.run, self.sid, self.signer, self.pins, 8000, 12000,
            )
        healthy: str = self.payload_sha256
        for malformed in ("", healthy[:-1], healthy.upper(), healthy + "\n", "'" + healthy[1:]):
            with self.subTest(malformed=malformed):
                self.payload_sha256 = malformed
                with self.assertRaisesRegex(ReceiptRefused, "invalid deployment owner payload digest"):
                    self.render()

    def test_owner_deadline_relationships_and_native_limits(self) -> None:
        for work, retirement in ((True, 2), (0, 1), (500, 500), (500, 499), (500, 30501), (119999, 120001)):
            with self.subTest(work=work, retirement=retirement):
                with self.assertRaisesRegex(ReceiptRefused, "invalid deployment owner deadlines"):
                    render_deployment_owner(
                        self.template, self.template_pin, self.run, self.sid, self.signer, self.pins, work, retirement,
                        expected_payload_sha256=self.payload_sha256,
                    )
        result: bytes = render_deployment_owner(
            self.template, self.template_pin, self.run, self.sid, self.signer, self.pins, 90000, 120000,
            expected_payload_sha256=self.payload_sha256,
        )[0]
        self.assertIn(b"$workMilliseconds = 90000\n[int]$retirementMilliseconds = 120000", result)

    def test_template_must_match_independent_bytes_and_one_binding_marker(self) -> None:
        changed: bytes = self.original.replace(b"# PRESTART_DEPLOYMENT_BINDINGS", b"# NO_DEPLOYMENT_BINDINGS      ")
        self.assertEqual(len(changed), len(self.original))
        self.template.write_bytes(changed)
        with self.assertRaisesRegex(ReceiptRefused, "template differs"):
            self.render()
        self.template_pin = Member(len(changed), sha256(changed).hexdigest())
        with self.assertRaisesRegex(ReceiptRefused, "binding marker"):
            self.render()
        doubled: bytes = self.original + b"\n# PRESTART_DEPLOYMENT_BINDINGS\n"
        self.template.write_bytes(doubled)
        self.template_pin = Member(len(doubled), sha256(doubled).hexdigest())
        with self.assertRaisesRegex(ReceiptRefused, "binding marker"):
            self.render()

    def test_template_link_refuses_even_when_target_has_matching_bytes(self) -> None:
        destination: Path = self.template.with_name("matching.ps1.in")
        self.template.rename(destination)
        self.template.symlink_to(destination)
        with self.assertRaisesRegex(ReceiptRefused, "template read refused"):
            self.render()


if __name__ == "__main__":
    unittest.main()
