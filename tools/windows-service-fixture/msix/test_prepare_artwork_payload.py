# SPDX-License-Identifier: GPL-2.0-only
"""Check ArtworkLab selection and publication through the staging command."""

from collections.abc import Callable
from contextlib import redirect_stderr, redirect_stdout
from hashlib import sha256
import io
import json
from pathlib import Path
import sys
from tempfile import TemporaryDirectory
import unittest
from unittest.mock import patch
import xml.etree.ElementTree as ET

import prepare_artwork_payload as producer

FOUNDATION: str = "http://schemas.microsoft.com/appx/manifest/foundation/windows10"
TRUSTED_LAUNCH: str = "http://schemas.microsoft.com/appx/manifest/trustedlaunch/windows10"
PROPERTIES: bytes = (
    '<?xml version="1.0" encoding="utf-8"?>\n'
    f'<Properties xmlns="{FOUNDATION}" xmlns:trustedlaunch="{TRUSTED_LAUNCH}">\n'
    '  <DisplayName>Fixture display name</DisplayName>\n'
    '  <Logo>Assets\\PackageLogo.png</Logo>\n'
    '  <trustedlaunch:TrustedLaunch>true</trustedlaunch:TrustedLaunch>\n'
    '</Properties>\n'
).encode("utf-8")


class ArtworkPayloadTests(unittest.TestCase):
    def setUp(self) -> None:
        self.root: Path = Path(self.enterContext(TemporaryDirectory()))
        self.handoff: Path = self.root / "handoff"
        self.pri_dir: Path = self.root / "pri"
        self.inert: Path = self.root / "inert.exe"
        self.output: Path = self.root / "payload"
        self.handoff.mkdir(mode=0o700)
        self.pri_dir.mkdir(mode=0o700)
        self.inert.write_bytes(b"fixture executable")
        self.inert_pin: str = sha256(self.inert.read_bytes()).hexdigest()
        self.handoff_pin: str = "9" * 64
        self.artwork: dict[str, bytes] = {
            "Assets/" + name: name.encode("ascii")
            for name in producer.verify.MSIX_BASE_ARTWORK.keys() | producer.verify.MSIX_VARIANT_ARTWORK.keys()
        }

        # Supply an already-inspected result. The command still reads real leaves,
        # checks their bytes, compares independently prepared PRI records and publishes.
        properties: Path = self.handoff / producer.PROPERTIES
        properties.parent.mkdir(parents=True)
        properties.write_bytes(PROPERTIES)
        files: dict[str, dict[str, str]] = {
            producer.PROPERTIES: {
                "disposition": "private-authoring", "sha256": sha256(PROPERTIES).hexdigest(),
            },
        }
        for name, data in self.artwork.items():
            relative: str = "msix-inputs/" + name
            path: Path = self.handoff / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
            files[relative] = {"disposition": "unadmitted-artwork", "sha256": sha256(data).hexdigest()}

        self.handoff_report: dict = {
            "assembly_ready": False, "package_identity": None,
            "files": files, "generation_receipt_sha256": "7" * 64,
        }
        self.pri: bytes = b"complete native PRI fixture"
        dump: bytes = b"<PriInfo/>"
        (self.pri_dir / "complete.pri").write_bytes(self.pri)
        (self.pri_dir / "complete.xml").write_bytes(dump)
        self.pri_report: dict = {
            "status": "passed", "map": "NevroTick.S06.ArtworkLab", "candidate_count": 51,
            "groups": {"PackageLogo.png": 3, "Square150x150Logo.png": 3, "Square44x44Logo.png": 45},
            "rows": {
                name: {"bytes": len(data), "sha256": sha256(data).hexdigest()}
                for name, data in self.artwork.items()
            },
            "pri_sha256": sha256(self.pri).hexdigest(), "dump_sha256": sha256(dump).hexdigest(),
        }
        self.pri_pin: str = self.write_pri_receipt()

    def write_pri_receipt(self) -> str:
        raw: bytes = json.dumps(self.pri_report).encode("utf-8")
        (self.pri_dir / "readback.json").write_bytes(raw)
        return sha256(raw).hexdigest()

    def invoke(self, *, inert_pin: str | None = None) -> tuple[int, str, str]:
        out: io.StringIO = io.StringIO()
        err: io.StringIO = io.StringIO()
        arguments: list[str] = [
            "prepare_artwork_payload", "--handoff", str(self.handoff),
            "--handoff-receipt-sha256", self.handoff_pin, "--pri-dir", str(self.pri_dir),
            "--pri-receipt-sha256", self.pri_pin, "--inert", str(self.inert),
            "--inert-sha256", self.inert_pin if inert_pin is None else inert_pin,
            "--output", str(self.output),
        ]
        with (
            patch.object(producer, "INERT_SHA256", self.inert_pin),
            patch.object(producer.verify, "inspect_msix_handoff", return_value=self.handoff_report) as inspect,
            patch.object(sys, "argv", arguments),
            redirect_stdout(out), redirect_stderr(err),
        ):
            status: int = producer.main()
            if inspect.called:
                inspect.assert_called_once_with(producer.stage.ROOT, self.handoff, self.handoff_pin)
        return status, out.getvalue(), err.getvalue()

    def assert_refused(self, diagnostic: str, **kwargs: str) -> None:
        status: int
        stdout: str
        stderr: str
        status, stdout, stderr = self.invoke(**kwargs)
        self.assertEqual(status, 1, stderr)
        self.assertIn(diagnostic, stderr)
        self.assertEqual(stdout, "")
        self.assertFalse(self.output.exists())
        self.assertEqual(set(self.root.iterdir()), {self.handoff, self.pri_dir, self.inert})

    def test_stages_only_exact_artwork_and_fixed_lab_payload(self) -> None:
        status: int
        stdout: str
        stderr: str
        status, stdout, stderr = self.invoke()
        self.assertEqual((status, stderr), (0, ""))
        report: dict = json.loads(stdout)
        self.assertEqual(report["status"], "unsigned-artwork-lab-inputs-staged")
        self.assertIs(report["assembly_ready"], False)
        self.assertEqual(report["handoff_receipt_sha256"], self.handoff_pin)
        self.assertEqual(report["generation_receipt_sha256"], "7" * 64)
        self.assertEqual(report["pri_readback_sha256"], self.pri_pin)
        self.assertEqual(report["properties_sha256"], sha256(PROPERTIES).hexdigest())

        actual: dict[str, bytes] = {
            path.relative_to(self.output).as_posix(): path.read_bytes()
            for path in self.output.rglob("*") if path.is_file()
        }
        self.assertEqual(len(actual), 54)
        self.assertEqual(report["payload"], {name: sha256(data).hexdigest() for name, data in actual.items()})
        manifest: ET.Element = ET.fromstring(actual.pop("AppxManifest.xml"))
        self.assertEqual(manifest.find(f"{{{FOUNDATION}}}Identity").attrib, {
            "Name": "NevroTick.S06.ArtworkLab", "Publisher": "CN=NevroTick S06 Laboratory",
            "Version": "1.0.0.0", "ProcessorArchitecture": "x64",
        })
        self.assertEqual(manifest.findtext(f"{{{FOUNDATION}}}Properties/{{{TRUSTED_LAUNCH}}}TrustedLaunch"), "true")
        self.assertEqual(manifest.findtext(f"{{{FOUNDATION}}}Properties/{{{FOUNDATION}}}DisplayName"),
                         "Fixture display name")
        self.assertEqual(actual, {**self.artwork, "Inert.exe": b"fixture executable", "resources.pri": self.pri})

    def test_rejects_changed_selected_artwork_and_properties(self) -> None:
        for relative in ("msix-inputs/Assets/PackageLogo.png", producer.PROPERTIES):
            with self.subTest(relative=relative):
                source: Path = self.handoff / relative
                original: bytes = source.read_bytes()
                source.write_bytes(b"changed after inspection")
                self.assert_refused("differs from the checked generation")
                source.write_bytes(original)

    def test_rejects_missing_or_extra_artwork_and_assembled_handoff(self) -> None:
        required: str = "msix-inputs/Assets/PackageLogo.png"
        roster: dict = self.handoff_report["files"]
        original: dict = roster.pop(required)
        self.assert_refused("handoff selection differs")
        roster[required] = original
        roster["msix-inputs/Assets/extra.png"] = original
        self.assert_refused("handoff selection differs")
        del roster["msix-inputs/Assets/extra.png"]
        self.handoff_report["assembly_ready"] = True
        self.assert_refused("handoff selection differs")

    def test_rejects_changed_pri_receipt_and_crossed_readbacks_with_matching_pins(self) -> None:
        original_map: str = self.pri_report["map"]
        self.pri_report["map"] = "Other.Lab"
        changed_pin: str = self.write_pri_receipt()
        self.assert_refused("differs from the checked generation")
        self.pri_pin = changed_pin
        self.assert_refused("differs from the exact ArtworkLab cohort")
        self.pri_report["map"] = original_map
        self.pri_report["rows"]["Assets/PackageLogo.png"]["sha256"] = "2" * 64
        self.pri_pin = self.write_pri_receipt()
        self.assert_refused("differs from the exact ArtworkLab cohort")

    def test_rejects_unmatched_pri_bytes_and_detailed_dump(self) -> None:
        for name in ("complete.pri", "complete.xml"):
            with self.subTest(name=name):
                path: Path = self.pri_dir / name
                original: bytes = path.read_bytes()
                path.write_bytes(b"different native output")
                self.assert_refused("PRI or detailed dump differs from the pinned readback")
                path.write_bytes(original)

    def test_rejects_substituted_inert_bytes_even_with_their_own_pin(self) -> None:
        other: bytes = b"different executable"
        self.inert.write_bytes(other)
        self.assert_refused("requires the reviewed inert image", inert_pin=sha256(other).hexdigest())
        self.assert_refused("Fixture input hash or size differs")

    def test_preserves_occupied_output(self) -> None:
        self.output.mkdir(mode=0o700)
        marker: Path = self.output / "prior"
        marker.write_bytes(b"unchanged")
        status: int
        stdout: str
        stderr: str
        status, stdout, stderr = self.invoke()
        self.assertEqual((status, stdout), (1, ""))
        self.assertIn("File exists", stderr)
        self.assertEqual(list(self.output.iterdir()), [marker])
        self.assertEqual(marker.read_bytes(), b"unchanged")

    def test_reports_primary_write_and_independent_cleanup_failure(self) -> None:
        status: int
        stdout: str
        stderr: str
        with (
            patch.object(producer.stage, "_write_payload", side_effect=OSError("fixture write failure")),
            patch.object(producer.stage, "_remove_staging", side_effect=OSError("fixture cleanup failure")),
        ):
            status, stdout, stderr = self.invoke()
        self.assertEqual((status, stdout), (1, ""))
        self.assertEqual(stderr.splitlines(), [
            "ArtworkLab staging refused: fixture write failure",
            "Private staging cleanup failed: fixture cleanup failure",
        ])
        self.assertFalse(self.output.exists())
        retained: set[Path] = set(self.root.iterdir()) - {self.handoff, self.pri_dir, self.inert}
        self.assertEqual(len(retained), 1)
        self.assertTrue(next(iter(retained)).is_dir())

    def test_changed_final_slot_and_failed_close_keep_uncertainty_without_success_receipt(self) -> None:
        renamed: Path = self.root / "committed"
        publish: Callable[[int, str, str], None] = producer.stage.publish_new
        close: Callable[[int], None] = producer.stage.os.close
        original_parent: int | None = None

        def publish_then_move(parent: int, temporary: str, name: str) -> None:
            nonlocal original_parent
            original_parent = parent
            publish(parent, temporary, name)
            self.output.rename(renamed)

        def close_then_fail(descriptor: int) -> None:
            close(descriptor)
            if descriptor == original_parent:
                raise OSError("fixture parent close failure")

        status: int
        stdout: str
        stderr: str
        with (
            patch.object(producer.stage, "publish_new", publish_then_move),
            patch.object(producer.stage.os, "close", close_then_fail),
        ):
            status, stdout, stderr = self.invoke()
        self.assertEqual((status, stdout), (3, ""))
        self.assertIn("Publication durability, path or cleanup is unconfirmed", stderr)
        self.assertIn("fixture parent close failure", stderr)
        self.assertFalse(self.output.exists())
        self.assertEqual(sum(path.is_file() for path in renamed.rglob("*")), 54)


if __name__ == "__main__":
    unittest.main()
