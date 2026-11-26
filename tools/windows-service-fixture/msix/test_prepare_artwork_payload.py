# SPDX-License-Identifier: GPL-2.0-only
"""Check ArtworkLab selection and publication through the staging command."""

from collections.abc import Callable
from contextlib import redirect_stderr, redirect_stdout
from hashlib import sha256
import io
import json
from pathlib import Path
import struct
import sys
from tempfile import TemporaryDirectory
import unittest
from unittest.mock import call, patch
import xml.etree.ElementTree as ET

import prepare_artwork_payload as producer
from test_prepare_payload import write_guarded_role_export

sys.path.insert(0, str(producer.stage.ROOT / "scripts/tests"))
from test_windows_pe import _image, _OBSERVE_GUID, _OPTIONAL, _pdb, _UI_GUID, _ui_resource_image

FOUNDATION: str = "http://schemas.microsoft.com/appx/manifest/foundation/windows10"
TRUSTED_LAUNCH: str = "http://schemas.microsoft.com/appx/manifest/trustedlaunch/windows10"
UAP: str = "http://schemas.microsoft.com/appx/manifest/uap/windows10"
UAP10: str = "http://schemas.microsoft.com/appx/manifest/uap/windows10/10"
DESKTOP6: str = "http://schemas.microsoft.com/appx/manifest/desktop/windows10/6"
RESCAP: str = "http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities"
PROPERTIES: bytes = (
    '<?xml version="1.0" encoding="utf-8"?>\n'
    f'<Properties xmlns="{FOUNDATION}" xmlns:trustedlaunch="{TRUSTED_LAUNCH}">\n'
    '  <DisplayName>Fixture display name</DisplayName>\n'
    '  <Logo>Assets\\PackageLogo.png</Logo>\n'
    '  <trustedlaunch:TrustedLaunch>true</trustedlaunch:TrustedLaunch>\n'
    '</Properties>\n'
).encode("utf-8")
SHARED_GUI_BUILD_SOURCES: dict[str, str] = {
    name: sha256(name.encode("ascii")).hexdigest() for name in (
        "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", ".cargo/config.toml",
        "policy/ui-m0-dependencies.json", "crates/nevrotick-ui/Cargo.toml",
        "crates/nevrotick-ui/src/native_tray.rs", "crates/nevrotick-ui/startup/cpu_entry.c",
    )
}
GUARDED_ROLE_SOURCES: dict[str, str] = {
    **SHARED_GUI_BUILD_SOURCES, "crates/nevrotick-ui/startup/cpu_entry_control.c": "7" * 64,
}


class ArtworkPayloadInputs(unittest.TestCase):
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
        root: ET.Element = ET.Element("PriInfo")
        mapping: ET.Element = ET.SubElement(root, "ResourceMap", {
            "name": "NevroTick.S06.ArtworkLab", "primary": "true",
            "uniqueName": "ms-appx://NevroTick.S06.ArtworkLab/",
        })
        files_tree: ET.Element = ET.SubElement(mapping, "ResourceMapSubtree", {"name": "Files"})
        assets_tree: ET.Element = ET.SubElement(files_tree, "ResourceMapSubtree", {"name": "Assets"})
        resource_nodes: dict[str, ET.Element] = {}
        name: str
        for name in ("PackageLogo.png", "Square150x150Logo.png", "Square44x44Logo.png"):
            resource_nodes[name] = ET.SubElement(assets_tree, "NamedResource", {
                "name": name, "uri": "ms-resource://NevroTick.S06.ArtworkLab/Files/Assets/" + name,
            })
        qualifiers_by_path: dict[str, dict[str, str]] = {}
        for name in sorted(self.artwork):
            parts: list[str] = Path(name).stem.split(".")
            candidate: ET.Element = ET.SubElement(resource_nodes[parts[0] + ".png"], "Candidate", {"type": "Path"})
            qualifiers: dict[str, str] = {}
            if len(parts) == 2:
                qualifier_set: ET.Element = ET.SubElement(candidate, "QualifierSet")
                part: str
                for part in parts[1].split("_"):
                    key: str
                    value: str
                    key, value = part.split("-", 1)
                    key = "AlternateForm" if key == "altform" else key.title()
                    qualifiers[key.lower()] = value
                    ET.SubElement(qualifier_set, "Qualifier", {"name": key, "value": value.upper()})
            ET.SubElement(candidate, "Value").text = name.replace("/", "\\")
            qualifiers_by_path[name] = qualifiers
        dump: bytes = ET.tostring(root, encoding="utf-8", xml_declaration=True)
        (self.pri_dir / "complete.pri").write_bytes(self.pri)
        (self.pri_dir / "complete.xml").write_bytes(dump)
        self.pri_report: dict = {
            "status": "passed", "map": "NevroTick.S06.ArtworkLab", "candidate_count": 51,
            "groups": {"PackageLogo.png": 3, "Square150x150Logo.png": 3, "Square44x44Logo.png": 45},
            "rows": {
                name: {"bytes": len(data), "sha256": sha256(data).hexdigest(), "qualifiers": qualifiers_by_path[name]}
                for name, data in self.artwork.items()
            },
            "pri_sha256": sha256(self.pri).hexdigest(), "dump_sha256": sha256(dump).hexdigest(),
        }
        self.pri_pin: str = self.write_pri_receipt()

    def write_pri_receipt(self) -> str:
        raw: bytes = json.dumps(self.pri_report).encode("utf-8")
        (self.pri_dir / "readback.json").write_bytes(raw)
        return sha256(raw).hexdigest()


class ArtworkPayloadTests(ArtworkPayloadInputs):
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
        self.assertNotIn("role_link_receipt_sha256", report)

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

    def assert_refuses_pri_dump_byte_change(self, original: bytes, replacement: bytes, diagnostic: str) -> None:
        path: Path = self.pri_dir / "complete.xml"
        dump: bytes = path.read_bytes()
        changed: bytes = dump.replace(original, replacement, 1)
        self.assertNotEqual(changed, dump)
        path.write_bytes(changed)
        self.pri_report["dump_sha256"] = sha256(changed).hexdigest()
        self.pri_pin = self.write_pri_receipt()
        self.assert_refused(diagnostic)

    def test_rejects_pri_dump_wrong_alternate_form_under_matching_pins(self) -> None:
        self.assert_refuses_pri_dump_byte_change(
            b'value="LIGHTUNPLATED"', b'value="UNPLATED"', "PRI candidate qualifiers differ",
        )

    def test_rejects_pri_dump_wrong_path_under_matching_pins(self) -> None:
        self.assert_refuses_pri_dump_byte_change(
            b"Assets\\PackageLogo.scale-200.png", b"Assets\\Extra.scale-200.png", "PRI candidate path differs",
        )

    def test_rejects_pri_dump_encoding_mismatch_under_matching_pins(self) -> None:
        self.assert_refuses_pri_dump_byte_change(
            b"encoding='utf-8'", b"encoding='utf-16'", "PRI dump must use UTF-8 XML encoding",
        )

    def test_rejects_pri_receipt_qualifiers_conflicting_with_dump(self) -> None:
        self.pri_report["rows"]["Assets/PackageLogo.scale-200.png"]["qualifiers"] = {"scale": "400"}
        self.pri_pin = self.write_pri_receipt()
        self.assert_refused("PRI candidate qualifiers differ")

    def test_rejects_pri_observations_agreeing_on_wrong_scale(self) -> None:
        self.pri_report["rows"]["Assets/PackageLogo.scale-200.png"]["qualifiers"] = {"scale": "400"}
        self.assert_refuses_pri_dump_byte_change(
            b'<Qualifier name="Scale" value="200" /></QualifierSet><Value>Assets\\PackageLogo.scale-200.png',
            b'<Qualifier name="Scale" value="400" /></QualifierSet><Value>Assets\\PackageLogo.scale-200.png',
            "PRI candidate qualifiers differ",
        )

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


class CombinedArtworkPayloadTests(ArtworkPayloadInputs):
    def setUp(self) -> None:
        super().setUp()
        self.role_directory: Path = self.root / "roles"
        self.role_directory.mkdir()
        self.role_receipt: Path = self.role_directory / "cross-roles.json"
        self.role_receipt.write_bytes(b"two-role link receipt")
        self.role_pin: str = sha256(self.role_receipt.read_bytes()).hexdigest()
        self.role_args: tuple[str, ...] = (
            "--role-link-receipt", str(self.role_receipt), "--role-link-receipt-sha256", self.role_pin,
        )
        self.ui: bytes = b"linked UI fixture"
        self.ui_path: Path = self.handoff / "nevrotick-ui.exe"
        self.ui_path.write_bytes(self.ui)
        self.handoff_report["files"][self.ui_path.name] = {
            "disposition": "unadmitted-ui-image", "sha256": sha256(self.ui).hexdigest(),
        }
        self.properties: bytes = PROPERTIES.replace(
            b"<Properties ", f'<Properties xmlns:uap10="{UAP10}" '.encode("ascii"),
        ).replace(
            b"  <trustedlaunch:TrustedLaunch>",
            b'  <uap10:PackageIntegrity><uap10:Content Enforcement="on" /></uap10:PackageIntegrity>\n'
            b"  <trustedlaunch:TrustedLaunch>",
        )
        (self.handoff / producer.PROPERTIES).write_bytes(self.properties)
        self.handoff_report["files"][producer.PROPERTIES]["sha256"] = sha256(self.properties).hexdigest()

    def invoke(self, arguments: tuple[str, ...], *, parsed: bool = True) -> tuple[int, str, str]:
        out: io.StringIO = io.StringIO()
        err: io.StringIO = io.StringIO()
        argv: list[str] = [
            "prepare_artwork_payload", "--handoff", str(self.handoff),
            "--handoff-receipt-sha256", self.handoff_pin, "--pri-dir", str(self.pri_dir),
            "--pri-receipt-sha256", self.pri_pin, "--output", str(self.output), *arguments,
        ]
        with (
            patch.object(producer.verify, "inspect_msix_handoff", return_value=self.handoff_report) as inspect,
            patch.object(sys, "argv", argv), redirect_stdout(out), redirect_stderr(err),
        ):
            if parsed:
                status: int = producer.main()
                inspect.assert_called_once_with(producer.stage.ROOT, self.handoff, self.handoff_pin)
            else:
                with self.assertRaises(SystemExit) as error:
                    producer.main()
                status = error.exception.code
                inspect.assert_not_called()
        return status, out.getvalue(), err.getvalue()

    def test_stages_the_fixed_ui_and_both_disabled_roles_with_the_checked_resources(self) -> None:
        role_names: dict[str, str] = {
            "nevrotick-lab-s02-engine.exe": "NevroTick-Lab-S02-Engine",
            "nevrotick-lab-s02-broker.exe": "NevroTick-Lab-S02-Broker",
        }
        role_images: dict[str, bytes] = {image: name.encode("ascii") for image, name in role_names.items()}
        with patch.object(producer.stage, "role_images", return_value=role_images) as read_roles:
            status, stdout, stderr = self.invoke(self.role_args)
            read_roles.assert_called_once_with(self.role_receipt, self.role_pin)
        self.assertEqual((status, stderr), (0, ""))
        actual: dict[str, bytes] = {
            path.relative_to(self.output).as_posix(): path.read_bytes()
            for path in self.output.rglob("*") if path.is_file()
        }
        self.assertEqual(json.loads(stdout), {
            "status": "unsigned-combined-artwork-lab-inputs-staged", "assembly_ready": False,
            "lab_identity": "NevroTick.S06.ArtworkLab", "handoff_receipt_sha256": self.handoff_pin,
            "generation_receipt_sha256": "7" * 64, "pri_readback_sha256": self.pri_pin,
            "properties_sha256": sha256(self.properties).hexdigest(), "role_link_receipt_sha256": self.role_pin,
            "payload": {name: sha256(data).hexdigest() for name, data in actual.items()},
        })
        manifest: ET.Element = ET.fromstring(actual.pop("AppxManifest.xml"))
        self.assertEqual(actual, {
            **self.artwork, **role_images, "nevrotick-ui.exe": self.ui, "resources.pri": self.pri,
        })
        ns: dict[str, str] = {"f": FOUNDATION, "uap": UAP, "uap10": UAP10, "d": DESKTOP6, "r": RESCAP}
        self.assertEqual(manifest.find("f:Identity", ns).attrib, {
            "Name": "NevroTick.S06.ArtworkLab", "Publisher": "CN=NevroTick S06 Laboratory",
            "Version": "1.0.1.0", "ProcessorArchitecture": "x64",
        })
        self.assertEqual(manifest.findtext(f"f:Properties/{{{TRUSTED_LAUNCH}}}TrustedLaunch", namespaces=ns), "true")
        self.assertEqual(manifest.find("f:Properties/uap10:PackageIntegrity/uap10:Content", ns).attrib,
                         {"Enforcement": "on"})
        app: ET.Element
        app, = manifest.findall("f:Applications/f:Application", ns)
        self.assertEqual(app.attrib, {
            "Id": "Observe", "Executable": "nevrotick-ui.exe",
            f"{{{UAP10}}}RuntimeBehavior": "packagedClassicApp", f"{{{UAP10}}}TrustLevel": "mediumIL",
        })
        self.assertEqual(app.find("uap:VisualElements", ns).get("AppListEntry"), "none")
        extensions: list[ET.Element] = app.findall("f:Extensions/d:Extension", ns)
        self.assertEqual(extensions, manifest.findall(".//d:Extension", ns))
        self.assertEqual(len(extensions), 2)
        self.assertEqual({row.get("Executable") for row in extensions}, set(role_names))
        for extension in extensions:
            executable: str = extension.attrib["Executable"]
            self.assertEqual(extension.attrib, {
                "Category": "windows.service", "Executable": executable, "EntryPoint": "Windows.FullTrustApplication",
            })
            service: ET.Element
            service, = extension
            self.assertEqual((service.tag, service.attrib), (f"{{{DESKTOP6}}}Service", {
                "Name": role_names[executable], "StartupType": "disabled", "StartAccount": "localService",
            }))
        capabilities: ET.Element = manifest.find("f:Capabilities", ns)
        self.assertCountEqual([(capability.tag, capability.attrib) for capability in capabilities], [
            (f"{{{RESCAP}}}Capability", {"Name": "runFullTrust"}),
            (f"{{{RESCAP}}}Capability", {"Name": "packagedServices"}),
        ])

    def test_rejects_missing_and_crossed_profile_arguments_during_parsing(self) -> None:
        inert_path: tuple[str, ...] = ("--inert", str(self.inert))
        inert_pin: tuple[str, ...] = ("--inert-sha256", self.inert_pin)
        role_path: tuple[str, ...] = self.role_args[:2]
        role_pin: tuple[str, ...] = self.role_args[2:]
        for arguments in ((), inert_pin, inert_path, role_pin, role_path, inert_path + role_pin,
                          role_path + inert_pin, inert_path + inert_pin + self.role_args):
            with self.subTest(arguments=arguments), patch.object(producer.stage, "role_images") as read_roles:
                status, stdout, stderr = self.invoke(arguments, parsed=False)
                self.assertEqual((status, stdout), (2, ""))
                self.assertIn("error:", stderr)
                self.assertFalse(self.output.exists())
                read_roles.assert_not_called()

    def guarded_fixture(self, *, version: bytes = b"exact version resource", generation: str | None = None,
                        missing_guard: bool = False, stale_tray: bool = False) -> tuple[tuple[str, ...], bytes]:
        """Write inert PE/PDB and linker map inputs for the actual guarded reader."""
        export: Path = self.root / "guarded-ui"
        export.mkdir(mode=0o700, exist_ok=True)
        expected_version: bytes = b"exact version resource"
        manifest: bytes = b"exact manifest resource"
        self.ui = bytes(_ui_resource_image(expected_version, manifest))
        self.ui_path.write_bytes(self.ui)
        self.handoff_report["files"][self.ui_path.name]["sha256"] = sha256(self.ui).hexdigest()
        image: bytearray = _ui_resource_image(version, manifest)
        struct.pack_into("<I", image, _OPTIONAL + 16, 0x1010)
        guarded: bytes = bytes(image)
        symbols: bytes = bytes(_pdb())
        (export / self.ui_path.name).write_bytes(guarded)
        (export / "nevrotick_ui.pdb").write_bytes(symbols)
        object_hash: str = "b" * 64
        link_map: bytes = (
            " Preferred load address is 0000000140000000\n"
            "  Address         Publics by Value              Rva+Base               Lib:Object\n"
            f" 0001:00000010 nevrotick_ui_entry 0000000140001010 nevrotick-ui-entry-{object_hash}.obj\n"
            " 0001:00000050 mainCRTStartup 0000000140001050 msvcrt:exe_main.obj\n"
            " entry point at         0001:00000010\n"
        ).encode("ascii")
        map_path: Path = self.root / "guarded-ui.map"
        map_path.write_bytes(link_map)
        generation = self.handoff_report["generation_receipt_sha256"] if generation is None else generation
        image_hash: str = sha256(guarded).hexdigest()
        ui_only_inputs: dict[str, str] = {
            "scripts/windows_pe.py": "8" * 64,
            "metadata/product.toml": "9" * 64,
        }
        ordinary_receipt: bytes = json.dumps({
            "source_sha256": {**SHARED_GUI_BUILD_SOURCES, **ui_only_inputs}, "dependency_profile": "default",
        }).encode("utf-8")
        (self.handoff / "verification.json").write_bytes(ordinary_receipt)
        self.handoff_pin = sha256(ordinary_receipt).hexdigest()
        guarded_inputs: dict[str, str] = {**GUARDED_ROLE_SOURCES, **ui_only_inputs}
        if stale_tray:
            guarded_inputs["crates/nevrotick-ui/src/native_tray.rs"] = "a" * 64
        receipt: dict = {
            "status": "passed", "native_windows_execution": "not performed",
            "source_sha256": guarded_inputs, "dependency_profile": "default",
            "artifact": {
                "path": "original/nevrotick-ui.exe", "sha256": image_hash, "profile": "release",
                "target": "x86_64-pc-windows-msvc", "component_role": "ui", "source_state": "fixture",
                "symbols": {"path": "original/nevrotick_ui.pdb", "sha256": sha256(symbols).hexdigest(),
                            "guid": "12345678-9abc-def0-1234-56789abcdef0", "age": 1},
            },
            "ui_versioninfo": {"image_sha256": image_hash, "generation_sha256": generation,
                               "strings": {"OriginalFilename": "nevrotick-ui.exe"}},
            "msix_assembly_inputs": {"generation_receipt_sha256": generation, "assembly_ready": False},
            "ui_preentry": {
                "entry_rva": 0x1010, "image_base": 0x140000000, "crt_startup_rva": 0x1050,
                "tls_callback_table_rva": 0, "direct_import_modules": [], "image_sha256": image_hash,
                "object_sha256": object_hash, "map_sha256": sha256(link_map).hexdigest(),
            },
        }
        if missing_guard:
            del receipt["ui_preentry"]
        raw: bytes = json.dumps(receipt).encode("utf-8")
        (export / "verification.json").write_bytes(raw)
        return (
            "--guarded-ui-export", str(export), "--guarded-ui-receipt-sha256", sha256(raw).hexdigest(),
            "--guarded-ui-link-map", str(map_path),
        ), guarded

    def test_guarded_ui_stages_exact_selected_bytes_and_reconciled_receipts(self) -> None:
        options: tuple[str, ...]
        guarded: bytes
        options, guarded = self.guarded_fixture()
        self.assertNotEqual(guarded, self.ui)
        role_images: dict[str, bytes] = {
            "nevrotick-lab-s02-engine.exe": b"engine", "nevrotick-lab-s02-broker.exe": b"broker",
        }
        with patch.object(producer.stage, "role_images", return_value=role_images) as read_roles:
            status, stdout, stderr = self.invoke(self.role_args + options)
            read_roles.assert_called_once_with(
                self.role_receipt, self.role_pin, require_baseline_entry=True, matching_sources=GUARDED_ROLE_SOURCES,
            )
        self.assertEqual((status, stderr), (0, ""))
        report: dict = json.loads(stdout)
        self.assertEqual(report["handoff_receipt_sha256"], self.handoff_pin)
        self.assertEqual(report["guarded_ui_receipt_sha256"], options[3])
        self.assertEqual(report["generation_receipt_sha256"], "7" * 64)
        self.assertIs(report["assembly_ready"], False)
        self.assertEqual(report["payload"][self.ui_path.name], sha256(guarded).hexdigest())
        self.assertEqual((self.output / self.ui_path.name).read_bytes(), guarded)
        self.assertEqual(self.ui_path.read_bytes(), self.ui)
        self.assertEqual(set(report["payload"]), {
            *self.artwork, *role_images, self.ui_path.name, "AppxManifest.xml", "resources.pri",
        })

    def test_guarded_ui_requires_all_three_arguments_and_combined_roles(self) -> None:
        options: tuple[str, ...]
        options, _ = self.guarded_fixture()
        inert: tuple[str, ...] = ("--inert", str(self.inert), "--inert-sha256", self.inert_pin)
        for arguments in (self.role_args + options[:4], self.role_args + options[2:],
                          self.role_args + options[:2] + options[4:], inert + options):
            with self.subTest(arguments=arguments):
                status, stdout, stderr = self.invoke(arguments, parsed=False)
                self.assertEqual((status, stdout), (2, ""))
                self.assertIn("error:", stderr)
                self.assertFalse(self.output.exists())

    def test_guarded_sdk_handoff_binds_pack_inputs_to_the_atomic_publication(self) -> None:
        options: tuple[str, ...]
        guarded: bytes
        options, guarded = self.guarded_fixture()
        sdk: Path = self.root / "sdk"
        sdk_inputs: dict[str, bytes] = {
            name: f"inert fixture leaf {name}".encode("ascii") for name in (
                "Microsoft.Windows.Build.Appx.AppxPackaging.dll.manifest",
                "Microsoft.Windows.Build.Appx.OpcServices.dll.manifest",
                "appxpackaging.dll",
                "en-US/AppxPackaging.dll.mui",
                "makeappx.exe",
                "opcservices.dll",
            )
        }
        name: str
        data: bytes
        for name, data in sdk_inputs.items():
            leaf: Path = sdk / name
            leaf.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
            leaf.write_bytes(data)
        sdk_pins: dict[str, str] = {name: sha256(data).hexdigest() for name, data in sdk_inputs.items()}
        role_images: dict[str, bytes] = {
            "nevrotick-lab-s02-engine.exe": b"linked engine fixture",
            "nevrotick-lab-s02-broker.exe": b"linked broker fixture",
        }
        args: tuple[str, ...] = self.role_args + options + ("--sdk-source", str(sdk))
        tampered: Path = sdk / "en-US/AppxPackaging.dll.mui"
        original: bytes = tampered.read_bytes()
        with (patch.object(producer.stage, "SDK_SHA256", sdk_pins),
              patch.object(producer.stage, "role_images", return_value=role_images) as read_roles):
            tampered.write_bytes(b"!" + original[1:])
            status, stdout, stderr = self.invoke(args)
            self.assertEqual((status, stdout), (1, ""))
            self.assertIn("Fixture input hash or size differs", stderr)
            self.assertFalse(self.output.exists())
            self.assertEqual(list(self.root.glob(".nevrotick-metadata-*")), [])

            tampered.write_bytes(original)
            status, stdout, stderr = self.invoke(args)
            self.assertEqual((status, stderr), (0, ""))
            report: dict[str, object] = json.loads(stdout)
            inventory: bytes = (self.output / "input-set.json").read_bytes()
            expected_payload: dict[str, bytes] = {
                **self.artwork, **role_images, self.ui_path.name: guarded, "resources.pri": self.pri,
                "AppxManifest.xml": producer.combined_manifest(self.properties),
            }
            expected: dict[str, bytes] = {
                **{f"sdk/{name}": data for name, data in sdk_inputs.items()},
                **{f"payload/{name}": data for name, data in expected_payload.items()},
                "input-set.json": inventory,
            }
            payload_pins: dict[str, str] = {
                name: sha256(data).hexdigest() for name, data in expected_payload.items()
            }
            self.assertEqual(json.loads(inventory), {"sdk": sdk_pins, "payload": payload_pins})
            self.assertEqual(report["sdk"], sdk_pins)
            self.assertEqual(report["payload"], payload_pins)
            self.assertEqual(report["input_set_sha256"], sha256(inventory).hexdigest())
            self.assertIs(report["assembly_ready"], False)
            self.assertEqual(len(expected), 63)

            status, stdout, stderr = self.invoke(args)
            self.assertEqual((status, stdout), (1, ""))
            self.assertIn("File exists", stderr)
            self.assertEqual(read_roles.call_args_list, [
                call(self.role_receipt, self.role_pin, require_baseline_entry=True, matching_sources=GUARDED_ROLE_SOURCES),
            ] * 3)
        actual: dict[str, bytes] = {
            path.relative_to(self.output).as_posix(): path.read_bytes()
            for path in self.output.rglob("*") if path.is_file()
        }
        self.assertEqual(actual, expected)

    def test_sdk_handoff_requires_the_guarded_profile(self) -> None:
        status, stdout, stderr = self.invoke(
            self.role_args + ("--sdk-source", str(self.root / "unused-sdk")), parsed=False,
        )
        self.assertEqual((status, stdout), (2, ""))
        self.assertIn("SDK handoff requires guarded UI and service roles", stderr)
        self.assertFalse(self.output.exists())

    def test_guarded_ui_refuses_crossed_evidence_without_publishing(self) -> None:
        options: tuple[str, ...]
        options, _ = self.guarded_fixture()
        crossed: tuple[str, ...] = (*options[:3], self.handoff_pin, *options[4:])
        status, stdout, stderr = self.invoke(self.role_args + crossed)
        self.assertEqual((status, stdout), (1, ""))
        self.assertIn("differs from the checked generation", stderr)
        self.assertFalse(self.output.exists())

        Path(options[5]).write_bytes(b"crossed link map")
        status, stdout, stderr = self.invoke(self.role_args + options)
        self.assertEqual((status, stdout), (1, ""))
        self.assertIn("Fixture input hash or size differs", stderr)
        self.assertFalse(self.output.exists())

    def test_guarded_ui_refuses_stale_tray_source_with_matching_resources(self) -> None:
        options, _ = self.guarded_fixture(stale_tray=True)
        role_images: dict[str, bytes] = {
            "nevrotick-lab-s02-engine.exe": b"engine", "nevrotick-lab-s02-broker.exe": b"broker",
        }
        with patch.object(producer.stage, "role_images", return_value=role_images):
            status, stdout, stderr = self.invoke(self.role_args + options)
        self.assertEqual((status, stdout), (1, ""), stderr)
        self.assertIn("Guarded UI build inputs differ", stderr)
        self.assertFalse(self.output.exists())

    def test_guarded_cli_rejects_a_repinned_stale_role_export_before_publication(self) -> None:
        options: tuple[str, ...]
        guarded: bytes
        options, guarded = self.guarded_fixture()
        linked: Path = self.role_directory / (self.role_receipt.stem + "-artifacts")
        images: dict[str, bytes]
        report: dict[str, object]
        images, _, report = write_guarded_role_export(linked)
        sources: dict[str, str] = {**GUARDED_ROLE_SOURCES, "tools/windows-service-fixture/msix/service_entry.c": "b" * 64}
        report["source_sha256"] = {**sources, "crates/nevrotick-ui/startup/cpu_entry_control.c": "a" * 64}
        stale: bytes = json.dumps(report).encode("utf-8")
        self.role_receipt.write_bytes(stale)
        status, stdout, stderr = self.invoke((*self.role_args[:-1], sha256(stale).hexdigest()) + options)
        self.assertEqual((status, stdout), (1, ""), stderr)
        self.assertIn("Service role source pins differ from the selected UI build", stderr)
        self.assertFalse(self.output.exists())

        report["source_sha256"] = sources
        healthy: bytes = json.dumps(report).encode("utf-8")
        self.role_receipt.write_bytes(healthy)
        status, stdout, stderr = self.invoke((*self.role_args[:-1], sha256(healthy).hexdigest()) + options)
        self.assertEqual((status, stderr), (0, ""))
        self.assertEqual(json.loads(stdout)["role_link_receipt_sha256"], sha256(healthy).hexdigest())
        self.assertEqual((self.output / self.ui_path.name).read_bytes(), guarded)
        name: str
        expected: bytes
        for name, expected in images.items():
            self.assertEqual((self.output / name).read_bytes(), expected)

    def test_guarded_ui_requires_both_pinned_build_identities(self) -> None:
        for kind in ("missing-source", "shared-entry", "extra-source", "different-profile"):
            with self.subTest(kind=kind):
                options, _ = self.guarded_fixture()
                receipt_path: Path = Path(options[1]) / "verification.json"
                guarded: dict = json.loads(receipt_path.read_bytes())
                if kind == "missing-source":
                    del guarded["source_sha256"]
                elif kind == "shared-entry":
                    guarded["source_sha256"]["crates/nevrotick-ui/startup/cpu_entry.c"] = "1" * 64
                elif kind == "extra-source":
                    guarded["source_sha256"]["crates/nevrotick-ui/src/extra.rs"] = "2" * 64
                else:
                    guarded["dependency_profile"] = "engine-ke-association"
                raw: bytes = json.dumps(guarded).encode("utf-8")
                receipt_path.write_bytes(raw)
                selected: tuple[str, ...] = (*options[:3], sha256(raw).hexdigest(), *options[4:])
                with patch.object(producer.stage, "role_images") as read_roles:
                    status, stdout, stderr = self.invoke(self.role_args + selected)
                    read_roles.assert_not_called()
                self.assertEqual((status, stdout), (1, ""), stderr)
                self.assertIn("frozen source map" if kind == "missing-source" else "build inputs differ", stderr)
                self.assertFalse(self.output.exists())

    def test_guarded_ui_refuses_unbound_entry_and_changed_generation_or_resources(self) -> None:
        for kwargs, reason in (
            ({"missing_guard": True}, "guarded"),
            ({"generation": "6" * 64}, "generation"),
            ({"version": b"different version resource"}, "resource"),
        ):
            with self.subTest(reason=reason):
                options, _ = self.guarded_fixture(**kwargs)
                status, stdout, stderr = self.invoke(self.role_args + options)
                self.assertEqual((status, stdout), (1, ""))
                self.assertIn(reason, stderr.lower())
                self.assertFalse(self.output.exists())

    def test_rejects_unlisted_missing_and_substituted_ui_without_publishing(self) -> None:
        files: dict[str, dict[str, str]] = self.handoff_report["files"]
        ui_row: dict[str, str] = files.pop(self.ui_path.name)
        role_images: dict[str, bytes] = {
            "nevrotick-lab-s02-engine.exe": b"engine", "nevrotick-lab-s02-broker.exe": b"broker",
        }
        with patch.object(producer.stage, "role_images", return_value=role_images):
            status, stdout, stderr = self.invoke(self.role_args)
            self.assertEqual((status, stdout), (1, ""), stderr)
            self.assertFalse(self.output.exists())
            files[self.ui_path.name] = {**ui_row, "disposition": "excluded-observe-image"}
            status, stdout, stderr = self.invoke(self.role_args)
            self.assertEqual((status, stdout), (1, ""), stderr)
            self.assertFalse(self.output.exists())
            files[self.ui_path.name] = ui_row
            self.ui_path.unlink()
            status, stdout, stderr = self.invoke(self.role_args)
            self.assertEqual((status, stdout), (1, ""), stderr)
            self.assertFalse(self.output.exists())
            self.ui_path.write_bytes(b"substituted UI fixture")
            self.output.mkdir(mode=0o700)
            marker: Path = self.output / "prior"
            marker.write_bytes(b"unchanged")
            status, stdout, stderr = self.invoke(self.role_args)
            self.assertEqual((status, stdout), (1, ""), stderr)
            self.assertIn("differs from the checked generation", stderr)
            self.assertEqual(list(self.output.iterdir()), [marker])
            self.assertEqual(marker.read_bytes(), b"unchanged")

    def test_rejects_the_role_reader_failure_without_partial_output(self) -> None:
        with patch.object(producer.stage, "role_images", side_effect=ValueError("fixture role read refused")) as read:
            status, stdout, stderr = self.invoke(self.role_args)
            read.assert_called_once_with(self.role_receipt, self.role_pin)
        self.assertEqual((status, stdout), (1, ""))
        self.assertIn("fixture role read refused", stderr)
        self.assertEqual(set(self.root.iterdir()), {
            self.handoff, self.pri_dir, self.inert, self.role_directory,
        })

    def test_stages_a_relocated_role_export_and_refuses_crossed_inputs(self) -> None:
        linked: Path = self.role_directory / (self.role_receipt.stem + "-artifacts")
        linked.mkdir(mode=0o700)
        logical: Path = producer.stage.RECEIPTS.relative_to(producer.stage.ROOT) / linked.name
        engine: str
        broker: str
        engine, broker = producer.stage.BINS
        images: dict[str, bytes] = {}
        symbols: dict[str, bytes] = {}
        rows: dict[str, dict[str, str]] = {}
        for role, guid in ((engine, _UI_GUID), (broker, _OBSERVE_GUID)):
            image: str = role + ".exe"
            pdb: str = role.replace("-", "_") + ".pdb"
            images[image] = bytes(_image(3, pdb_path=pdb, guid=guid))
            symbols[pdb] = bytes(_pdb(guid=guid))
            (linked / image).write_bytes(images[image])
            (linked / pdb).write_bytes(symbols[pdb])
            rows[role] = {"image": str(logical / image), "pdb": str(logical / pdb),
                          "image_sha256": sha256(images[image]).hexdigest(),
                          "pdb_sha256": sha256(symbols[pdb]).hexdigest()}
        report: dict[str, object] = {"status": "passed", "lane": "link", "bookends_passed": True,
                                     "index_bytes_unchanged": True, "artifacts": rows}
        healthy: bytes = json.dumps(report).encode("utf-8")
        self.role_receipt.write_bytes(healthy)
        arguments: tuple[str, ...] = (*self.role_args[:-1], sha256(healthy).hexdigest())

        status, stdout, stderr = self.invoke(arguments)
        self.assertEqual(status, 0, stderr)
        self.assertEqual(stderr, "")
        produced: dict[str, object] = json.loads(stdout)
        self.assertFalse(produced["assembly_ready"])
        for name, expected in images.items():
            self.assertEqual((self.output / name).read_bytes(), expected)
        self.assertFalse(any((self.output / name).exists() for name in symbols))
        prior: dict[Path, bytes] = {path: path.read_bytes() for path in self.output.rglob("*") if path.is_file()}

        standalone_output: Path = linked / "payload"
        standalone: list[str] = [
            "prepare_payload", "--receipt", str(self.role_receipt), "--receipt-sha256", arguments[-1],
            "--inert", str(self.inert), "--inert-sha256", self.inert_pin, "--output", str(standalone_output),
        ]
        out: io.StringIO = io.StringIO()
        err: io.StringIO = io.StringIO()
        with (patch.object(sys, "argv", standalone), redirect_stdout(out), redirect_stderr(err),
              self.assertRaises(SystemExit) as failure):
            producer.stage.main()
        self.assertEqual((failure.exception.code, out.getvalue()), (1, ""))
        self.assertIn("output-overlaps-source", err.getvalue())
        self.assertFalse(standalone_output.exists())

        image_path: str = rows[engine]["image"]
        rows[engine]["image"] = rows[broker]["image"]
        crossed: bytes = json.dumps(report).encode("utf-8")
        self.role_receipt.write_bytes(crossed)
        status, stdout, stderr = self.invoke((*arguments[:-1], sha256(crossed).hexdigest()))
        self.assertEqual((status, stdout), (1, ""))
        self.assertIn("maps a role to a different image or symbol file", stderr)
        rows[engine]["image"] = image_path
        self.role_receipt.write_bytes(healthy)

        engine_pdb: str = Path(rows[engine]["pdb"]).name
        broker_pdb: str = Path(rows[broker]["pdb"]).name
        (linked / engine_pdb).write_bytes(symbols[broker_pdb])
        status, stdout, stderr = self.invoke(arguments)
        self.assertEqual((status, stdout), (1, ""))
        self.assertIn("PDB GUID or age differs", stderr)
        self.assertEqual({path: path.read_bytes() for path in self.output.rglob("*") if path.is_file()}, prior)
        (linked / engine_pdb).write_bytes(symbols[engine_pdb])
        self.output = self.root / "restored-payload"
        status, stdout, stderr = self.invoke(arguments)
        self.assertEqual((status, stderr), (0, ""))
        self.assertEqual(produced, json.loads(stdout))


if __name__ == "__main__":
    unittest.main()
