# SPDX-License-Identifier: GPL-2.0-only
"""Exercise the combined ArtworkLab reader through its portable CLI."""

from collections.abc import Callable
from copy import deepcopy
from hashlib import sha256
import json
from pathlib import Path
import shutil
import struct
import subprocess
import sys
import tempfile
import unittest

from test_check_package import block_map, make_zip, with_zip64_end_records
from test_prepare_artwork_payload import GUARDED_ROLE_SOURCES
from test_prepare_payload import write_guarded_role_export
from test_windows_pe import _ui_resource_image
from test_windows_versioninfo import _DESCRIPTION, _FIXED, _MANIFEST, version_resource


SCRIPT: Path = Path(__file__).with_name("check_artwork_package.py")
MANIFEST: str = "AppxManifest.xml"
BLOCK_MAP: str = "AppxBlockMap.xml"
CONTENT_TYPES: str = "[Content_Types].xml"
PRI: str = "resources.pri"
UI: str = "nevrotick-ui.exe"
ENGINE: str = "nevrotick-lab-s02-engine.exe"
BROKER: str = "nevrotick-lab-s02-broker.exe"
PNG_SIGNATURE: bytes = b"\x89PNG\r\n\x1a\n"
PRI_SIGNATURE: bytes = b"mrm_pri2"
XML: bytes = rb"""<?xml version="1.0" encoding="utf-8"?>
<Package xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10"
 xmlns:uap="http://schemas.microsoft.com/appx/manifest/uap/windows10"
 xmlns:uap10="http://schemas.microsoft.com/appx/manifest/uap/windows10/10"
 xmlns:rescap="http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities"
 xmlns:trustedlaunch="http://schemas.microsoft.com/appx/manifest/trustedlaunch/windows10"
 xmlns:desktop6="http://schemas.microsoft.com/appx/manifest/desktop/windows10/6"
 IgnorableNamespaces="uap uap10 rescap trustedlaunch desktop6">
  <Identity Name="NevroTick.S06.ArtworkLab" Publisher="CN=NevroTick S06 Laboratory"
    Version="1.0.1.0" ProcessorArchitecture="x64"/>
  <Properties>
    <DisplayName>NevroTick</DisplayName><PublisherDisplayName>David Osipov</PublisherDisplayName>
    <Description>A Windows time synchronization client under development.</Description>
    <Logo>Assets\PackageLogo.png</Logo>
    <uap10:PackageIntegrity><uap10:Content Enforcement="on"/></uap10:PackageIntegrity>
    <trustedlaunch:TrustedLaunch>true</trustedlaunch:TrustedLaunch>
  </Properties>
  <Dependencies><TargetDeviceFamily Name="Windows.Desktop" MinVersion="10.0.26100.0"
    MaxVersionTested="10.0.26100.0"/></Dependencies>
  <Resources><Resource Language="en-US"/></Resources>
  <Applications>
    <Application Id="Observe" Executable="nevrotick-ui.exe"
      uap10:RuntimeBehavior="packagedClassicApp" uap10:TrustLevel="mediumIL">
      <uap:VisualElements DisplayName="NevroTick artwork laboratory" Description="Offline resource assembly fixture"
        BackgroundColor="transparent" Square44x44Logo="Assets\Square44x44Logo.png"
        Square150x150Logo="Assets\Square150x150Logo.png" AppListEntry="none"/>
      <Extensions>
        <desktop6:Extension Category="windows.service" Executable="nevrotick-lab-s02-engine.exe"
          EntryPoint="Windows.FullTrustApplication">
          <desktop6:Service Name="NevroTick-Lab-S02-Engine" StartupType="disabled" StartAccount="localService"/>
        </desktop6:Extension>
        <desktop6:Extension Category="windows.service" Executable="nevrotick-lab-s02-broker.exe"
          EntryPoint="Windows.FullTrustApplication">
          <desktop6:Service Name="NevroTick-Lab-S02-Broker" StartupType="disabled" StartAccount="localService"/>
        </desktop6:Extension>
      </Extensions>
    </Application>
  </Applications>
  <Capabilities><rescap:Capability Name="runFullTrust"/><rescap:Capability Name="packagedServices"/></Capabilities>
</Package>
"""
TYPES: bytes = b"""<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="exe" ContentType="application/x-msdownload"/>
  <Default Extension="png" ContentType="image/png"/>
  <Default Extension="pri" ContentType="application/octet-stream"/>
  <Default Extension="xml" ContentType="application/vnd.ms-appx.manifest+xml"/>
  <Override PartName="/AppxBlockMap.xml" ContentType="application/vnd.ms-appx.blockmap+xml"/>
</Types>
"""


def header(marker: bytes) -> bytes:
    """Stand in for a PE's checked declaration without an executable body."""
    raw: bytearray = bytearray(90)
    raw[:2] = b"MZ"
    struct.pack_into("<I", raw, 60, 64)
    raw[64:68] = b"PE\0\0"
    struct.pack_into("<H", raw, 68, 0x8664)
    struct.pack_into("<H", raw, 88, 0x20B)
    return bytes(raw) + marker


def authored_payload() -> dict[str, bytes]:
    """Independently build the expected carrier's closed lab roster."""
    payload: dict[str, bytes] = {
        MANIFEST: XML, PRI: PRI_SIGNATURE + b"synthetic-resource-index", UI: header(b"ui"),
        ENGINE: header(b"engine"), BROKER: header(b"broker"),
    }
    for stem in ("PackageLogo", "Square150x150Logo", "Square44x44Logo"):
        for qualifier in ("", ".scale-200", ".scale-400"):
            name: str = f"Assets/{stem}{qualifier}.png"
            payload[name] = PNG_SIGNATURE + name.encode("ascii")
    for size in (16, 20, 24, 30, 32, 36, 40, 48, 60, 64, 72, 80, 96, 256):
        target_stem: str = f"Assets/Square44x44Logo.targetsize-{size}"
        for qualifier in ("", "_altform-lightunplated", "_altform-unplated"):
            name: str = f"{target_stem}{qualifier}.png"
            payload[name] = PNG_SIGNATURE + name.encode("ascii")
    return payload


class CombinedPackageTests(unittest.TestCase):
    def setUp(self) -> None:
        self.root: Path = Path(self.enterContext(tempfile.TemporaryDirectory()))
        self.data: dict[str, bytes] = authored_payload()
        self.payload: Path = self.root / "payload"
        self.package: Path = self.root / "ArtworkLab.msix"
        self.pins: Path = self.root / "pins.json"

    def run_reader(
        self,
        *,
        archive_payload: dict[str, bytes] | None = None,
        archive_transform: Callable[[bytes], bytes] | None = None,
        comments: dict[str, bytes] | None = None,
        pinned_payload: dict[str, bytes] | None = None,
        manifest_sha: str | None = None,
        script: Path = SCRIPT,
        guarded: tuple[str, ...] = (),
    ) -> subprocess.CompletedProcess[bytes]:
        for name, value in self.data.items():
            path: Path = self.payload / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(value)
        archived: dict[str, bytes] = self.data if archive_payload is None else archive_payload
        pinned: dict[str, bytes] = self.data if pinned_payload is None else pinned_payload
        archive: bytes = with_zip64_end_records(make_zip({
            **archived, BLOCK_MAP: block_map(archived, {}), CONTENT_TYPES: TYPES,
        }, {}, comments=comments))
        if archive_transform is not None:
            archive = archive_transform(archive)
        self.package.write_bytes(archive)
        raw_pins: bytes = json.dumps({
            name: sha256(value).hexdigest() for name, value in pinned.items() if name != MANIFEST
        }).encode("utf-8")
        self.pins.write_bytes(raw_pins)
        return subprocess.run([
            sys.executable, "-I", str(script), "--payload", str(self.payload),
            "--package", str(self.package), "--package-sha256", sha256(archive).hexdigest(),
            "--pins", str(self.pins), "--pins-sha256", sha256(raw_pins).hexdigest(),
            "--manifest-sha256", manifest_sha or sha256(pinned[MANIFEST]).hexdigest(),
            *guarded,
        ], capture_output=True, timeout=8, check=False)

    def assert_refusal(self, outcome: subprocess.CompletedProcess[bytes], cause: bytes) -> None:
        self.assertEqual(outcome.returncode, 1, outcome.stderr)
        self.assertEqual(outcome.stdout, b"")
        self.assertIn(cause, outcome.stderr)
        self.assertNotIn(b"Traceback", outcome.stderr)

    def test_exact_unsigned_profile(self) -> None:
        scripts: Path = self.root / "scripts"
        scripts.mkdir()
        for name in (SCRIPT.name, "check_package.py"):
            shutil.copyfile(SCRIPT.with_name(name), scripts / name)
        outcome: subprocess.CompletedProcess[bytes] = self.run_reader(script=scripts / SCRIPT.name)
        self.assertEqual(outcome.returncode, 0, outcome.stderr)
        self.assertEqual({path.name for path in scripts.iterdir()}, {SCRIPT.name, "check_package.py"})
        self.assertEqual(outcome.stderr, b"")
        receipt: dict[str, object] = json.loads(outcome.stdout)
        self.assertEqual(receipt["status"], "unsigned-combined-artworklab-checked")
        self.assertNotIn("startup_inventory", receipt)
        self.assertEqual((receipt["payload_count"], receipt["artwork_count"]), (56, 51))
        members: dict[str, dict[str, int | str]] = receipt["members"]
        self.assertEqual(set(members), self.data.keys() | {BLOCK_MAP, CONTENT_TYPES})
        catalog: bytes = b""
        for name, value in sorted(self.data.items()):
            observed: str = sha256(value).hexdigest()
            self.assertEqual(members[name], {"bytes": len(value), "sha256": observed})
            catalog += f"{name}\t{len(value)}\t{observed}\n".encode("ascii")
        self.assertEqual(receipt["payload_sha256"], sha256(catalog).hexdigest())

        image_names: tuple[str, str, str] = (UI, ENGINE, BROKER)
        artwork_names: set[str] = set(self.data) - {MANIFEST, PRI, *image_names}
        self.assertEqual(len(artwork_names), 51)
        expected_formats: dict[str, str] = {
            MANIFEST: "xml",
            PRI: "pri",
            **{name: "pe32+" for name in image_names},
            **{name: "png" for name in artwork_names},
        }
        self.assertEqual(receipt["signing_inventory"], {
            "verification": "not-run",
            "package": {"format": "msix", "signature_requirement": "package-signature"},
            "payload": {
                name: {
                    "format": kind,
                    "signature_requirement": (
                        "embedded-authenticode" if name in image_names else "package-signature"
                    ),
                }
                for name, kind in expected_formats.items()
            },
        })

    def test_partial_guarded_startup_selection_refuses(self) -> None:
        self.assert_refusal(self.run_reader(guarded=("--guarded-ui-sha256", "0" * 64)),
                            b"guarded startup requires both pinned link receipts")

    def test_guarded_receipt_must_name_the_archived_ui(self) -> None:
        ui_receipt: Path = self.root / "verification.json"
        map_data: bytes = b"fixture linker map"
        link_map: Path = self.root / "entry.map"
        link_map.write_bytes(map_data)
        report: dict[str, object] = {
            "status": "passed",
            "native_windows_execution": "not performed",
            "artifact": {
                "sha256": sha256(header(b"previous-ui")).hexdigest(),
                "component_role": "ui", "source_state": "fixture",
                "profile": "release", "target": "x86_64-pc-windows-msvc",
            },
            "ui_preentry": {
                "image_sha256": sha256(header(b"previous-ui")).hexdigest(),
                "object_sha256": sha256(b"fixture baseline object").hexdigest(),
                "map_sha256": sha256(map_data).hexdigest(),
            },
        }
        body: bytes = json.dumps(report).encode("utf-8")
        ui_receipt.write_bytes(body)
        self.assert_refusal(self.run_reader(guarded=(
            "--guarded-ui-receipt", str(ui_receipt), "--guarded-ui-sha256", sha256(body).hexdigest(),
            "--guarded-ui-link-map", str(link_map),
            "--guarded-role-receipt", str(self.root / "unused-role.json"), "--guarded-role-sha256", "0" * 64,
            "--guarded-ui-source-sha256", "0" * 64,
        )), b"guarded UI link receipt differs from the archived UI")

    def test_guarded_archive_joins_both_original_service_roles(self) -> None:
        role_receipt: Path = self.root / "cross-artwork-roles.json"
        images, _, role_report = write_guarded_role_export(self.root / "cross-artwork-roles-artifacts")
        service_source: str = "tools/windows-service-fixture/msix/service_entry.c"
        service_sources: dict[str, str] = {**GUARDED_ROLE_SOURCES, service_source: "a" * 64}
        ui_sources: dict[str, str] = {**GUARDED_ROLE_SOURCES, "metadata/product.toml": "9" * 64}
        # Derive the independent pin once from the original fixture roster.
        source_pairs: str = ",".join(f'"{name}":"{pin}"' for name, pin in sorted(ui_sources.items()))
        source_pin: str = sha256(("{" + source_pairs + "}").encode("ascii")).hexdigest()
        role_report["source_sha256"] = service_sources
        role_bytes: bytes = json.dumps(role_report).encode("utf-8")
        role_receipt.write_bytes(role_bytes)
        role_pin: str = sha256(role_bytes).hexdigest()
        gui: bytearray = _ui_resource_image(version_resource(), _MANIFEST)
        struct.pack_into("<I", gui, 0x98 + 16, 0x1010)
        self.data.update(images)
        self.data[UI] = bytes(gui)

        gate_pin: str = sha256(b"fixture UI gate").hexdigest()
        mapping: bytes = (
            " Preferred load address is 0000000140000000\n"
            "  Address         Publics by Value              Rva+Base               Lib:Object\n"
            f" 0001:00000010 nevrotick_ui_entry 0000000140001010 nevrotick-ui-entry-{gate_pin}.obj\n"
            " 0001:00000030 mainCRTStartup 0000000140001030 msvcrt:exe_main.obj\n"
            " entry point at         0001:00000010\n"
        ).encode("ascii")
        link_map: Path = self.root / "ui.map"
        link_map.write_bytes(mapping)
        image_pin: str = sha256(gui).hexdigest()
        ui_report = {
            "status": "passed", "native_windows_execution": "not performed",
            "dependency_profile": "default",
            "source_sha256": ui_sources,
            "artifact": {
                "component_role": "ui", "source_state": "fixture", "profile": "release",
                "target": "x86_64-pc-windows-msvc", "sha256": image_pin,
            },
            "ui_preentry": {
                "image_sha256": image_pin, "object_sha256": gate_pin, "map_sha256": sha256(mapping).hexdigest(),
                "entry_rva": 0x1010, "crt_startup_rva": 0x1030, "image_base": 0x140000000,
                "tls_callback_table_rva": 0, "direct_import_modules": [],
            },
            "ui_versioninfo": {
                "image_sha256": image_pin,
                "fixed": list(_FIXED), "translation": [1033, 1200],
                "strings": {"FileDescription": _DESCRIPTION, "OriginalFilename": UI},
                "execution_manifest": {
                    "level": "asInvoker", "ui_access": False, "sha256": sha256(_MANIFEST).hexdigest(),
                },
            },
        }
        ui_receipt: Path = self.root / "ui.json"

        def write_receipt(report: dict[str, object], service_pin: str = role_pin) -> tuple[str, ...]:
            body: bytes = json.dumps(report).encode("utf-8")
            ui_receipt.write_bytes(body)
            return (
                "--guarded-ui-receipt", str(ui_receipt), "--guarded-ui-sha256", sha256(body).hexdigest(),
                "--guarded-ui-link-map", str(link_map),
                "--guarded-role-receipt", str(role_receipt), "--guarded-role-sha256", service_pin,
                "--guarded-ui-source-sha256", source_pin,
            )

        guarded: tuple[str, ...] = write_receipt(ui_report)
        healthy: subprocess.CompletedProcess[bytes] = self.run_reader(guarded=guarded)
        self.assertEqual(healthy.returncode, 0, healthy.stderr)
        observed: dict[str, dict[str, object]] = json.loads(healthy.stdout)["startup_inventory"]["members"]
        self.assertEqual(set(observed), {UI, ENGINE, BROKER})
        self.assertEqual(observed[UI]["crt_startup_rva"], 0x1030)
        self.assert_refusal(self.run_reader(guarded=guarded[:-2]),
                            b"the original UI map and the UI source digest")

        original_archive: bytes = self.package.read_bytes()
        crossed = {**role_report, "source_sha256": {**service_sources, "Cargo.lock": "0" * 64}}
        crossed_bytes: bytes = json.dumps(crossed).encode("utf-8")
        role_receipt.write_bytes(crossed_bytes)
        with self.subTest(service_source="Cargo.lock"):
            result = self.run_reader(guarded=write_receipt(ui_report, sha256(crossed_bytes).hexdigest()))
            self.assertEqual(self.package.read_bytes(), original_archive)
            self.assert_refusal(result, b"guarded service link evidence refused")

        shared: str = "crates/nevrotick-ui/src/native_tray.rs"
        incomplete_service: dict[str, str] = service_sources.copy()
        del incomplete_service[shared]
        incomplete_service_bytes: bytes = json.dumps({**role_report, "source_sha256": incomplete_service}).encode("utf-8")
        role_receipt.write_bytes(incomplete_service_bytes)
        incomplete_service_pin: str = sha256(incomplete_service_bytes).hexdigest()
        self.assert_refusal(self.run_reader(guarded=write_receipt(ui_report, incomplete_service_pin)),
                            b"guarded service link evidence refused")
        incomplete_ui: dict[str, str] = ui_sources.copy()
        del incomplete_ui[shared]
        for altered_sources in (incomplete_ui, {**incomplete_ui, service_source: service_sources[service_source]}):
            with self.subTest(ui_source_count=len(altered_sources)):
                altered_report: dict[str, object] = {**ui_report, "source_sha256": altered_sources}
                result = self.run_reader(guarded=write_receipt(altered_report, incomplete_service_pin))
                self.assertEqual(self.package.read_bytes(), original_archive)
                self.assert_refusal(result, b"guarded UI source map SHA-256 differs")
        role_receipt.write_bytes(role_bytes)

        for field, replacement in (
            ("strings", {"FileDescription": "Altered description", "OriginalFilename": UI}),
            ("fixed", [*_FIXED[:9], True, *_FIXED[10:]]),
            ("translation", [1033.0, 1200]),
            ("execution_manifest", {"level": "asInvoker", "ui_access": 0,
                                    "sha256": sha256(_MANIFEST).hexdigest()}),
            ("execution_manifest", {"level": "asInvoker", "ui_access": False, "sha256": "0" * 64}),
            ("image_sha256", "0" * 64),
        ):
            with self.subTest(resource_field=field, replacement=replacement):
                altered = deepcopy(ui_report)
                altered["ui_versioninfo"][field] = replacement
                self.assert_refusal(self.run_reader(guarded=write_receipt(altered)),
                                    b"guarded UI resource facts differ from the archived PE")

        guarded = write_receipt(ui_report)
        for service, original in images.items():
            with self.subTest(service=service):
                self.data[service] = original + b"\0"
                self.assert_refusal(self.run_reader(guarded=guarded),
                                    b"archived service differs from guarded link: " + service.encode("ascii"))
            self.data[service] = original

    def test_same_count_substituted_archive_member_refuses(self) -> None:
        altered: dict[str, bytes] = self.data.copy()
        altered["Assets/Unexpected.png"] = altered.pop("Assets/Square44x44Logo.targetsize-24.png")
        self.assert_refusal(self.run_reader(archive_payload=altered), b"package inventory differs")

    def test_refuses_central_member_comment_with_original_payload_and_new_archive_pin(self) -> None:
        healthy = self.run_reader()
        self.assertEqual(healthy.returncode, 0, healthy.stderr)
        self.assert_refusal(self.run_reader(comments={UI: b"note"}),
                            b"central ZIP member comment is not admitted")

    def test_unsupported_extract_version_refuses_cleanly(self) -> None:
        def version(package: bytes) -> bytes:
            offset: int = package.index(b"PK\x01\x02") + 6
            return package[:offset] + b"\xff\x00" + package[offset + 2:]

        self.assert_refusal(self.run_reader(archive_transform=version), b"unsupported ZIP extract version")

    def test_local_extract_version_mismatch_refuses(self) -> None:
        def version(package: bytes) -> bytes:
            return package[:4] + bytes([package[4] ^ 1]) + package[5:]

        self.assert_refusal(self.run_reader(archive_transform=version), b"local ZIP extract version differs")

    def test_same_count_substituted_authored_asset_refuses(self) -> None:
        original: dict[str, bytes] = self.data.copy()
        self.data["Assets/Unexpected.png"] = self.data.pop("Assets/Square44x44Logo.targetsize-24.png")
        self.assert_refusal(self.run_reader(archive_payload=original, pinned_payload=original),
                            b"authored asset inventory differs")

    def test_rebuilt_manual_service_refuses_after_all_pins_match(self) -> None:
        self.data[MANIFEST] = XML.replace(b'StartupType="disabled"', b'StartupType="manual"', 1)
        self.assert_refusal(self.run_reader(), b"nevrotick-lab-s02-engine.exe service profile attributes differ")

    def test_manifest_requires_original_independent_pin(self) -> None:
        self.data[MANIFEST] = XML + b"\n"
        self.assert_refusal(self.run_reader(manifest_sha=sha256(XML).hexdigest()),
                            b"member differs from independent source pin: AppxManifest.xml")

    def test_rebuilt_artwork_with_pe_header_refuses_after_all_pins_match(self) -> None:
        self.data["Assets/PackageLogo.png"] = header(b"artwork-header-control")
        self.assert_refusal(self.run_reader(), b"Assets/PackageLogo.png PNG signature differs")

    def test_rebuilt_pri_with_wrong_signature_refuses_after_all_pins_match(self) -> None:
        self.data[PRI] = b"wrongpri" + self.data[PRI][len(PRI_SIGNATURE):]
        self.assert_refusal(self.run_reader(), b"resources.pri PRI signature differs")

    def test_fresh_pins_and_archive_cannot_extend_pri_limit(self) -> None:
        self.data[PRI] = b"x" * (65_536 + 1)
        self.assert_refusal(self.run_reader(), b"resources.pri must be a bounded regular file")

    def test_rebuilt_pri_cannot_substitute_for_original_pin(self) -> None:
        pinned: dict[str, bytes] = self.data.copy()
        self.data[PRI] = PRI_SIGNATURE + b"changed-resource-index"
        self.assert_refusal(self.run_reader(pinned_payload=pinned),
                            b"member differs from independent source pin: resources.pri")


if __name__ == "__main__":
    unittest.main()
